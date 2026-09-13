//! `diagnose` — read an eADL file and show exactly what the reader saw.
//!
//! A diagnostic tool, kept in the repository rather than improvised (`TOOLBOX.md`). It answers
//! three questions without anyone having to reason about the reader's source:
//!
//! * does this file read, and if not, **where** does it stop?
//! * what did it parse to, in canonical form?
//! * what metadata does its comment block carry?
//!
//! ```console
//! $ cargo run -q -p eadl-front --example diagnose -- docs/semantics/boundary/accept/time-horizon.eadl
//! ```
//!
//! Exit codes: `0` read cleanly · `1` diagnostics were produced · `2` usage or I/O failure.

use std::process::ExitCode;

use eadl_front::{read, SourceMap};

fn main() -> ExitCode {
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: diagnose <file.eadl>");
        eprintln!("  reads the file and prints diagnostics, canonical form, and headers");
        return ExitCode::from(2);
    };

    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("diagnose: cannot read {path}: {error}");
            return ExitCode::from(2);
        }
    };

    let mut sources = SourceMap::new();
    let id = match sources.add(&path, text) {
        Ok(id) => id,
        Err(error) => {
            eprintln!("diagnose: {error}");
            return ExitCode::from(2);
        }
    };

    let (document, diagnostics) = read(&sources, id);

    if diagnostics.is_empty() {
        println!("read cleanly: {} top-level form(s)", document.forms.len());
    } else {
        print!("{}", diagnostics.render(&sources));
        println!(
            "{} diagnostic(s); {} top-level form(s) recovered",
            diagnostics.len(),
            document.forms.len()
        );
    }

    let headers = document.comment_headers();
    if !headers.is_empty() {
        println!("\nheaders:");
        for (key, value) in &headers {
            let shown = if value.chars().count() > 72 {
                format!("{}…", value.chars().take(72).collect::<String>())
            } else {
                value.clone()
            };
            println!("  {key}: {shown}");
        }
    }

    if !document.forms.is_empty() {
        println!("\ncanonical:");
        for line in document.to_canonical().lines() {
            println!("  {line}");
        }
    }

    if diagnostics.has_errors() {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    }
}
