# PromptFix

PromptFix is a minimal V1 prototype for terminal spelling suggestions in `zsh`.

## Current layout

```text
promptfix/
├─ Cargo.toml
├─ src/
│  ├─ main.rs
│  └─ spell/
│     ├─ filters.rs
│     ├─ hunspell.rs
│     └─ mod.rs
└─ share/
   └─ promptfix.zsh
```

## Build

```bash
cargo build --release
```

## Usage

```bash
./target/release/promptfix check --text "Explian how neurel networks works"
```

Example output:

```text
MESSAGE Explian Explain 0 7
MESSAGE neurel neural 12 18
APPLY Explain how neural networks works
```

## zsh integration

Source the plugin:

```bash
source /path/to/repo/share/promptfix.zsh
```

For Homebrew-style installs, the intended integration is:

```bash
source "$(brew --prefix)/share/promptfix/promptfix.zsh"
```

## Dependencies

PromptFix V1 now uses the macOS system spellchecker via `NSSpellChecker`.

There is no separate spellchecker dependency to install with Homebrew.
