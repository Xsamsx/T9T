mod spell;

use std::env;
use std::process;

fn main() {
    if let Err(err) = run() {
        eprintln!("promptfix: {err}");
        process::exit(1);
    }
}

fn run() -> Result<(), String> {
    let mut args = env::args().skip(1);
    match args.next().as_deref() {
        Some("check") => {
            let text = parse_check_args(args)?;
            let engine = spell::SpellEngine::new();
            let report = engine.check_text(&text)?;
            print!("{report}");
            Ok(())
        }
        Some("help") | Some("--help") | Some("-h") | None => {
            print_usage();
            Ok(())
        }
        Some(other) => Err(format!("unknown command: {other}")),
    }
}

fn parse_check_args(mut args: impl Iterator<Item = String>) -> Result<String, String> {
    let mut text = None;

    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--text" => {
                let value = args
                    .next()
                    .ok_or_else(|| String::from("missing value for --text"))?;
                text = Some(value);
            }
            other => return Err(format!("unknown argument: {other}")),
        }
    }

    text.ok_or_else(|| String::from("usage: promptfix check --text \"...\""))
}

fn print_usage() {
    println!("PromptFix");
    println!();
    println!("USAGE:");
    println!("  promptfix check --text \"Explian how neurel networks works\"");
    println!();
    println!("OUTPUT:");
    println!("  MESSAGE <original> <replacement> <start> <end>");
    println!("  APPLY <corrected text>");
}
