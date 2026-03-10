use std::env;
use std::io::{self, Read, Write};
use std::path::Path;
use std::sync::Arc;
use std::thread;

use crossterm::terminal::{disable_raw_mode, enable_raw_mode, size};
use portable_pty::{native_pty_system, CommandBuilder, PtySize};

use crate::spell::SpellEngine;

const ALLOWED_PROGRAMS: &[&str] = &["codex", "claude", "gemini"];

pub fn run(program: String, program_args: Vec<String>) -> Result<(), String> {
    validate_program(&program)?;

    let pty_system = native_pty_system();
    let (cols, rows) = size().unwrap_or((80, 24));
    let pair = pty_system
        .openpty(PtySize {
            rows,
            cols,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|err| format!("failed to open pty: {err}"))?;

    let mut command = CommandBuilder::new(program);
    for arg in program_args {
        command.arg(arg);
    }

    let mut child = pair
        .slave
        .spawn_command(command)
        .map_err(|err| format!("failed to launch target cli: {err}"))?;

    let mut reader = pair
        .master
        .try_clone_reader()
        .map_err(|err| format!("failed to clone pty reader: {err}"))?;
    let mut writer = pair
        .master
        .take_writer()
        .map_err(|err| format!("failed to open pty writer: {err}"))?;

    let output_thread = thread::spawn(move || -> io::Result<()> {
        let mut stdout = io::stdout().lock();
        let mut buf = [0u8; 4096];

        loop {
            let count = reader.read(&mut buf)?;
            if count == 0 {
                break;
            }

            stdout.write_all(&buf[..count])?;
            stdout.flush()?;
        }

        Ok(())
    });

    let _raw_mode = RawModeGuard::new().map_err(|err| format!("failed to enable raw mode: {err}"))?;
    let engine = Arc::new(SpellEngine::new());
    let mut tracker = LineTracker::default();
    let mut stdin = io::stdin().lock();
    let mut byte = [0u8; 1];

    loop {
        let count = stdin
            .read(&mut byte)
            .map_err(|err| format!("failed to read stdin: {err}"))?;
        if count == 0 {
            break;
        }

        let input = byte[0];
        match input {
            b' ' => handle_space(&mut writer, &engine, &mut tracker)?,
            b'\r' | b'\n' => {
                writer
                    .write_all(&[input])
                    .map_err(|err| format!("failed to write enter: {err}"))?;
                writer.flush().map_err(|err| format!("failed to flush enter: {err}"))?;
                tracker.reset();
            }
            0x7f | 0x08 => {
                writer
                    .write_all(&[input])
                    .map_err(|err| format!("failed to write backspace: {err}"))?;
                writer
                    .flush()
                    .map_err(|err| format!("failed to flush backspace: {err}"))?;
                tracker.backspace();
            }
            0x03 => {
                writer
                    .write_all(&[input])
                    .map_err(|err| format!("failed to write ctrl-c: {err}"))?;
                writer.flush().map_err(|err| format!("failed to flush ctrl-c: {err}"))?;
                tracker.reset();
            }
            0x1b => {
                handle_escape(&mut stdin, &mut writer, &mut tracker)?;
            }
            byte if is_printable_ascii(byte) => {
                writer
                    .write_all(&[byte])
                    .map_err(|err| format!("failed to write byte: {err}"))?;
                writer.flush().map_err(|err| format!("failed to flush byte: {err}"))?;
                tracker.insert(byte as char);
            }
            other => {
                writer
                    .write_all(&[other])
                    .map_err(|err| format!("failed to write raw byte: {err}"))?;
                writer
                    .flush()
                    .map_err(|err| format!("failed to flush raw byte: {err}"))?;
            }
        }

        if let Some(status) = child.try_wait().map_err(|err| format!("failed to poll child: {err}"))? {
            output_thread
                .join()
                .map_err(|_| String::from("failed to join output thread"))?
                .map_err(|err| format!("failed to stream child output: {err}"))?;

            if status.success() {
                return Ok(());
            }

            return Err(format!("target cli exited with status {status}"));
        }
    }

    let status = child.wait().map_err(|err| format!("failed to wait for child: {err}"))?;
    output_thread
        .join()
        .map_err(|_| String::from("failed to join output thread"))?
        .map_err(|err| format!("failed to stream child output: {err}"))?;

    if status.success() {
        Ok(())
    } else {
        Err(format!("target cli exited with status {status}"))
    }
}

fn validate_program(program: &str) -> Result<(), String> {
    if env::var_os("PROMPTFIX_ALLOW_ANY").is_some() {
        return Ok(());
    }

    let name = Path::new(program)
        .file_name()
        .and_then(|item| item.to_str())
        .unwrap_or(program);

    if ALLOWED_PROGRAMS.iter().any(|allowed| allowed == &name) {
        Ok(())
    } else {
        Err(format!(
            "interactive mode is restricted to AI CLIs: {}",
            ALLOWED_PROGRAMS.join(", ")
        ))
    }
}

fn handle_space(
    writer: &mut dyn Write,
    engine: &SpellEngine,
    tracker: &mut LineTracker,
) -> Result<(), String> {
    if let Some((start, end, word)) = tracker.current_word() {
        if let Some(replacement) = engine.suggest_prompt_word(&word)? {
            if replacement != word {
                let original_len = end.saturating_sub(start);
                let corrected = replacement.clone();

                for _ in 0..original_len {
                    writer
                        .write_all(&[0x08])
                        .map_err(|err| format!("failed to backspace old word: {err}"))?;
                }

                writer
                    .write_all(corrected.as_bytes())
                    .map_err(|err| format!("failed to write corrected word: {err}"))?;
                writer
                    .flush()
                    .map_err(|err| format!("failed to flush corrected word: {err}"))?;

                tracker.replace_range(start, end, &replacement);
            }
        }
    }

    writer
        .write_all(b" ")
        .map_err(|err| format!("failed to write space: {err}"))?;
    writer.flush().map_err(|err| format!("failed to flush space: {err}"))?;
    tracker.insert(' ');
    Ok(())
}

fn handle_escape(
    stdin: &mut dyn Read,
    writer: &mut dyn Write,
    tracker: &mut LineTracker,
) -> Result<(), String> {
    let mut seq = [0u8; 2];
    let read = stdin
        .read(&mut seq)
        .map_err(|err| format!("failed to read escape sequence: {err}"))?;

    let mut bytes = vec![0x1b];
    bytes.extend_from_slice(&seq[..read]);

    writer
        .write_all(&bytes)
        .map_err(|err| format!("failed to write escape sequence: {err}"))?;
    writer
        .flush()
        .map_err(|err| format!("failed to flush escape sequence: {err}"))?;

    if read == 2 && seq[0] == b'[' {
        match seq[1] {
            b'D' => tracker.move_left(),
            b'C' => tracker.move_right(),
            _ => {}
        }
    }

    Ok(())
}

fn is_printable_ascii(byte: u8) -> bool {
    (0x20..=0x7e).contains(&byte)
}

struct RawModeGuard;

impl RawModeGuard {
    fn new() -> io::Result<Self> {
        enable_raw_mode()?;
        Ok(Self)
    }
}

impl Drop for RawModeGuard {
    fn drop(&mut self) {
        let _ = disable_raw_mode();
    }
}

#[derive(Default)]
struct LineTracker {
    buffer: String,
    cursor: usize,
}

impl LineTracker {
    fn insert(&mut self, ch: char) {
        if self.cursor >= self.buffer.len() {
            self.buffer.push(ch);
            self.cursor = self.buffer.len();
            return;
        }

        self.buffer.insert(self.cursor, ch);
        self.cursor += ch.len_utf8();
    }

    fn backspace(&mut self) {
        if self.cursor == 0 || self.cursor > self.buffer.len() {
            return;
        }

        let remove_at = self.cursor - 1;
        self.buffer.remove(remove_at);
        self.cursor -= 1;
    }

    fn move_left(&mut self) {
        if self.cursor > 0 {
            self.cursor -= 1;
        }
    }

    fn move_right(&mut self) {
        if self.cursor < self.buffer.len() {
            self.cursor += 1;
        }
    }

    fn replace_range(&mut self, start: usize, end: usize, replacement: &str) {
        self.buffer.replace_range(start..end, replacement);
        self.cursor = start + replacement.len();
    }

    fn reset(&mut self) {
        self.buffer.clear();
        self.cursor = 0;
    }

    fn current_word(&self) -> Option<(usize, usize, String)> {
        if self.cursor == 0 || self.cursor > self.buffer.len() {
            return None;
        }

        let left = &self.buffer[..self.cursor];
        let mut start = left.len();
        for (idx, ch) in left.char_indices().rev() {
            if ch.is_ascii_alphabetic() || ch == '\'' {
                start = idx;
            } else {
                break;
            }
        }

        if start == self.cursor {
            return None;
        }

        let word = self.buffer[start..self.cursor].to_string();
        Some((start, self.cursor, word))
    }
}
