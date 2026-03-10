# PromptFix

PromptFix is a local spell-correction assistant for terminal AI workflows.

The current prototype has two modes:

1. Interactive AI-CLI mode: `promptfix exec codex|claude|gemini`
2. Pre-launch `zsh` plugin mode: source `share/promptfix.zsh`

The interactive mode is the main direction. It runs the target AI CLI inside a PTY and corrects obvious misspellings when you press `Space`.

## What PromptFix is

PromptFix is not a new terminal emulator and not a cloud AI service.

It is a thin local layer that sits in front of an interactive AI CLI, watches the text you type, and applies conservative spelling fixes before the target CLI consumes the final input.

On macOS, it uses the system spellchecker through `NSSpellChecker`.

## Trust model

PromptFix only becomes useful if developers trust it. The prototype is intentionally narrow:

- Interactive correction only works for AI CLIs launched through `promptfix exec ...`
- Correction currently happens on `Space`
- It only checks recent natural-language-like words
- It skips paths, flags, variables, numbers, URLs, and code-like tokens
- It is designed to miss uncertain corrections rather than aggressively mutate input

Current trust boundary:

- Supported interactive targets: `codex`, `claude`, `gemini`
- Everything else is excluded by default
- You can override that restriction for local experiments with `PROMPTFIX_ALLOW_ANY=1`

## Build

```bash
cargo build --release
```

## Interactive mode

Interactive mode only supports AI CLIs:

```bash
./target/release/promptfix exec codex
./target/release/promptfix exec claude
./target/release/promptfix exec gemini
```

Example workflow:

1. Run `./target/release/promptfix exec codex`
2. Type `Explian how neurel networks works`
3. Press `Space` after a misspelled word
4. PromptFix rewrites obvious mistakes in place before the CLI receives the final text

### Recommended aliases

If you want this to feel native in your shell:

```bash
alias codex='/Users/trysudo/Documents/project/t9t/target/release/promptfix exec codex'
alias claude='/Users/trysudo/Documents/project/t9t/target/release/promptfix exec claude'
alias gemini='/Users/trysudo/Documents/project/t9t/target/release/promptfix exec gemini'
```

Then you can keep launching the tools with `codex`, `claude`, and `gemini`, but PromptFix will sit in front of them.

### Local test flow

Build PromptFix:

```bash
cd /Users/trysudo/Documents/project/t9t
cargo build --release
```

Start an AI CLI through PromptFix:

```bash
./target/release/promptfix exec codex
```

Then type naturally inside the interactive session, for example:

```text
Explian how neurel networks works
```

Press `Space` after a misspelled word. PromptFix will try to rewrite obvious mistakes in place before the underlying CLI receives the final input.

If you want to test the PTY wrapper without a real AI CLI, you can use:

```bash
PROMPTFIX_ALLOW_ANY=1 ./target/release/promptfix exec cat
```

That is only for local debugging. The shipped interactive mode is intentionally limited to AI CLIs.

## Check command

The low-level spellcheck command is still available:

```bash
./target/release/promptfix check --text "Explian how neurel networks works"
```

Example output:

```text
MESSAGE neurel neural 12 18
APPLY Explian how neural networks works
```

## zsh plugin prototype

The original shell-buffer prototype still exists:

```bash
source /Users/trysudo/Documents/project/t9t/share/promptfix.zsh
```

That mode only works before a command launches. It does not affect already-running interactive CLIs.

## macOS dependency model

PromptFix currently uses the macOS system spellchecker via `NSSpellChecker`.

There is no separate spellchecker dependency to install with Homebrew.

## Current prototype limits

- Interactive correction is intentionally limited to AI CLIs launched through `promptfix exec ...`
- The first interactive version is single-line oriented
- Correction currently happens on `Space`
- The prototype is conservative and may miss some misspellings
- Undo and suspend toggles are not implemented yet
- Terminal redraw behavior is still basic and has only been smoke-tested, not hardened

## Roadmap

Near-term work:

- Improve spelling suggestion quality for words like `Explian`
- Add undo for the last applied correction
- Add a suspend/toggle control inside the session
- Improve cursor/edit tracking beyond the current single-line baseline
- Clean up the macOS bridge warnings by moving off deprecated Cocoa bindings

Longer-term work:

- Better per-app heuristics for `codex`, `claude`, and `gemini`
- Packaging for Homebrew
- Optional personal dictionary support
