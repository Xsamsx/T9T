#[derive(Clone, Debug)]
pub struct WordSpan {
    pub start: usize,
    pub end: usize,
}

const SKIP_COMMANDS: &[&str] = &[
    "git", "kubectl", "npm", "pnpm", "yarn", "cargo", "make", "docker", "docker-compose",
    "terraform", "aws", "gcloud", "claude", "codex", "gemini", "python", "python3", "node",
    "npx", "go", "uv", "pip", "pip3", "brew", "ssh", "scp", "rsync", "ls", "cd", "cat",
];

pub fn recent_word_spans(text: &str, max_words: usize) -> Vec<WordSpan> {
    let mut spans = Vec::new();
    let bytes = text.as_bytes();
    let mut index = 0;
    let mut token_index = 0;

    while index < bytes.len() {
        while index < bytes.len() && bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        if index >= bytes.len() {
            break;
        }

        let start = index;
        while index < bytes.len() && !bytes[index].is_ascii_whitespace() {
            index += 1;
        }
        let end = index;
        let token = &text[start..end];

        if should_check_token(token, token_index == 0) {
            spans.push(WordSpan { start, end });
        }
        token_index += 1;
    }

    let start = spans.len().saturating_sub(max_words);
    spans[start..].to_vec()
}

pub fn is_prompt_candidate(token: &str) -> bool {
    should_check_token(token, false)
}

fn should_check_token(token: &str, is_first_word: bool) -> bool {
    if token.len() < 3 {
        return false;
    }

    if is_first_word && SKIP_COMMANDS.iter().any(|cmd| token == *cmd) {
        return false;
    }

    if token.contains("://")
        || token.contains('/')
        || token.contains('-')
        || token.contains('.')
        || token.contains('_')
        || token.contains('$')
        || token.starts_with('~')
        || token.starts_with('-')
        || token.contains('=')
        || token.chars().any(|ch| ch.is_ascii_digit())
    {
        return false;
    }

    token.chars().all(|ch| ch.is_ascii_alphabetic() || ch == '\'' )
}
