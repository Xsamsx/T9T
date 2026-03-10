use std::collections::HashMap;
use std::io::Write;
use std::process::{Command, Stdio};

pub struct HunspellResult {
    suggestions: HashMap<String, Vec<String>>,
}

impl HunspellResult {
    pub fn first_suggestion(&self, word: &str) -> Option<String> {
        self.suggestions.get(word).and_then(|items| items.first()).cloned()
    }
}

pub fn check_words(words: &[&str]) -> Result<HunspellResult, String> {
    if words.is_empty() {
        return Ok(HunspellResult {
            suggestions: HashMap::new(),
        });
    }

    let mut child = Command::new("hunspell")
        .args(["-a", "-d", "en_US"])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|err| format!("failed to launch hunspell: {err}"))?;

    {
        let stdin = child
            .stdin
            .as_mut()
            .ok_or_else(|| String::from("failed to open hunspell stdin"))?;

        for word in words {
            writeln!(stdin, "{word}")
                .map_err(|err| format!("failed to write to hunspell stdin: {err}"))?;
        }
    }

    let output = child
        .wait_with_output()
        .map_err(|err| format!("failed to read hunspell output: {err}"))?;

    if !output.status.success() {
        let stderr = String::from_utf8_lossy(&output.stderr);
        return Err(format!("hunspell failed: {}", stderr.trim()));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    Ok(parse_hunspell_a(words, &stdout))
}

fn parse_hunspell_a(words: &[&str], stdout: &str) -> HunspellResult {
    let mut suggestions = HashMap::new();
    let mut lines = stdout.lines();

    let _ = lines.next();

    for word in words {
        let line = lines.next().unwrap_or_default();
        if let Some(items) = parse_suggestion_line(line) {
            suggestions.insert((*word).to_string(), items);
        }
    }

    HunspellResult { suggestions }
}

fn parse_suggestion_line(line: &str) -> Option<Vec<String>> {
    if line.starts_with('&') || line.starts_with('?') {
        let (_, right) = line.split_once(':')?;
        let items = right
            .split(',')
            .map(|item| item.trim().to_string())
            .filter(|item| !item.is_empty())
            .collect::<Vec<_>>();

        if !items.is_empty() {
            return Some(items);
        }
    }

    None
}
