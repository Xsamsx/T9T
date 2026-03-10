mod filters;
mod macos;

use std::fmt::Write;

pub struct SpellEngine;

impl SpellEngine {
    pub fn new() -> Self {
        Self
    }

    pub fn check_text(&self, text: &str) -> Result<String, String> {
        if text.trim().is_empty() {
            return Ok(String::new());
        }

        let recent = filters::recent_word_spans(text, 4);
        if recent.is_empty() {
            return Ok(String::new());
        }

        let mut corrections = Vec::new();
        for span in recent {
            let token = &text[span.start..span.end];
            if let Some(replacement) = macos::first_suggestion(token)? {
                if replacement != token {
                    corrections.push(Correction {
                        original: token.to_string(),
                        replacement,
                        start: span.start,
                        end: span.end,
                    });
                }
            }
        }

        if corrections.is_empty() {
            return Ok(String::new());
        }

        let mut output = String::new();
        for correction in &corrections {
            let _ = writeln!(
                output,
                "MESSAGE {} {} {} {}",
                correction.original, correction.replacement, correction.start, correction.end
            );
        }

        let corrected = apply_corrections(text, &corrections);
        let _ = writeln!(output, "APPLY {}", corrected);
        Ok(output)
    }
}

#[derive(Clone, Debug)]
struct Correction {
    original: String,
    replacement: String,
    start: usize,
    end: usize,
}

fn apply_corrections(text: &str, corrections: &[Correction]) -> String {
    let mut corrected = text.to_string();
    for correction in corrections.iter().rev() {
        corrected.replace_range(correction.start..correction.end, &correction.replacement);
    }
    corrected
}
