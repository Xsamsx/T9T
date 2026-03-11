<p align="center">
  <img src="https://raw.githubusercontent.com/Xsamsx/T9T/7adbf119255f95b0427f2845d109a4d9256901e9/assets/readme-hero.svg" alt="T9T hero" width="980" />
</p>

<p align="center">
  <a href="#build-and-run"><img alt="Rust" src="https://img.shields.io/badge/built%20with-Rust-111111?style=for-the-badge&logo=rust&logoColor=white"></a>
  <a href="#current-scope"><img alt="Platform" src="https://img.shields.io/badge/platform-macOS-1f6feb?style=for-the-badge&logo=apple&logoColor=white"></a>
  <a href="./LICENSE"><img alt="License" src="https://img.shields.io/badge/license-MIT-0f172a?style=for-the-badge"></a>
</p>

<h1 align="center">T9T</h1>

<p align="center">
  T9 for terminal AI.
</p>

<p align="center">
  A local macOS utility that sits in front of <code>codex</code>, <code>claude</code>, or <code>gemini</code> and fixes obvious prompt typos before the final input is sent.
</p>

<p align="center">
  <code>Explian how neurel networks works</code> becomes cleaner without leaving the keyboard.
</p>

## Why This Exists

Terminal AI sessions are fast. Your fingers are faster. The result is a steady stream of small prompt typos that break flow and make your inputs noisier than they need to be.

T9T is a narrow tool built for that exact problem. It brings T9-style forgiveness to terminal AI workflows while staying out of the way of commands, paths, flags, variables, and other code-like input.

## What It Does

- launches an AI CLI inside a PTY wrapper
- watches the prompt text you type locally
- checks the recent word when you press `Space`
- rewrites obvious spelling mistakes in place
- skips paths, flags, URLs, numbers, variables, and code-like tokens

The current binary name is `promptfix`. The product idea and branding are `T9T`.

## Why It Feels Different

T9T is not a new terminal emulator.

It is not a cloud writing assistant.

It is not trying to rewrite your whole sentence.

It is a thin local correction layer designed to preserve terminal flow.

## Current Scope

Today, T9T supports:

- `promptfix exec codex`
- `promptfix exec claude`
- `promptfix exec gemini`

It is currently macOS-only because suggestions come from the native macOS spellchecker via `NSSpellChecker`.

## Before / After

```text
Before: Explian how neurel networks works
After:  Explian how neural networks works
```

The behavior is intentionally conservative. The tool would rather miss a correction than mutate something risky.

## Build And Run

```bash
cargo build --release
./target/release/promptfix exec codex
```

You can swap `codex` for `claude` or `gemini`.

If you want it to feel native in your shell:

```bash
alias codex='/Users/trysudo/Documents/project/t9t/target/release/promptfix exec codex'
alias claude='/Users/trysudo/Documents/project/t9t/target/release/promptfix exec claude'
alias gemini='/Users/trysudo/Documents/project/t9t/target/release/promptfix exec gemini'
```

## How It Works

1. Launch your AI CLI through `promptfix exec ...`
2. Type naturally in the terminal
3. Press `Space` after a word
4. T9T checks the recent natural-language token
5. If the correction is obvious, it rewrites the word before the CLI consumes the final prompt

## Trust Model

This project only works if users trust it. The current prototype stays deliberately narrow:

- correction currently happens only on `Space`
- only recent natural-language-like words are checked
- command and code-like tokens are skipped
- supported targets are restricted by default to `codex`, `claude`, and `gemini`
- uncertain corrections are ignored

For local experiments, you can bypass the target restriction:

```bash
PROMPTFIX_ALLOW_ANY=1 ./target/release/promptfix exec cat
```

## Check Command

The lower-level checker is also available:

```bash
./target/release/promptfix check --text "Explian how neurel networks works"
```

Example output:

```text
MESSAGE neurel neural 12 18
APPLY Explian how neural networks works
```

## zsh Prototype

The earlier shell-buffer prototype still exists:

```bash
source /Users/trysudo/Documents/project/t9t/share/promptfix.zsh
```

That version only works before a command launches. It does not affect an already-running interactive CLI.

## Roadmap

Near-term:

- improve suggestion quality for tricky typos
- add undo for the last applied correction
- add a suspend or toggle control inside sessions
- improve cursor and edit tracking
- package for Homebrew

Longer-term:

- better per-app heuristics for `codex`, `claude`, and `gemini`
- optional personal dictionary support
- cross-platform spell engines

## Positioning

The strongest short description is:

`T9T brings T9-style typo correction to terminal AI tools on macOS.`

The strongest headline is:

`T9 for terminal AI.`

Useful one-paragraph product copy:

`T9T adds a local spell layer to terminal AI workflows on macOS. Run Codex, Claude, or Gemini through it and obvious prompt typos get fixed as you type, without touching paths, flags, URLs, or code-like tokens.`

## License

MIT
