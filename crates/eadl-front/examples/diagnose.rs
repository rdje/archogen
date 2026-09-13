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
//!
//! ⛔ A closed pipe is not a failure. `diagnose … | head -3` closes stdout early, and Rust's
//! `println!` panics on the resulting `BrokenPipe` — which prints a Rust backtrace where the
//! user expected three lines of output. Found by piping the tool while using it. Every write
//! here goes through [`emit`], which exits quietly on a closed pipe and reports any other I/O
//! error honestly.

use std::io::{ErrorKind, Write};
use std::process::ExitCode;

use eadl_front::{read, SourceMap};

/// Write one line, treating a closed pipe as a normal end of output.
fn emit(out: &mut dyn Write, line: &str) -> Result<(), ExitCode> {
    match writeln!(out, "{line}") {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::BrokenPipe => Err(ExitCode::SUCCESS),
        Err(error) => {
            // stderr may still be open even when stdout is not.
            let _ = writeln!(std::io::stderr(), "diagnose: cannot write output: {error}");
            Err(ExitCode::from(2))
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(code) | Err(code) => code,
    }
}

#[allow(clippy::too_many_lines)]
fn run() -> Result<ExitCode, ExitCode> {
    let mut stdout = std::io::stdout().lock();
    let mut args = std::env::args().skip(1);
    let Some(path) = args.next() else {
        eprintln!("usage: diagnose <file.eadl>");
        eprintln!("  reads the file and prints diagnostics, canonical form, and headers");
        return Ok(ExitCode::from(2));
    };

    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("diagnose: cannot read {path}: {error}");
            return Ok(ExitCode::from(2));
        }
    };

    let mut sources = SourceMap::new();
    let id = match sources.add(&path, text) {
        Ok(id) => id,
        Err(error) => {
            eprintln!("diagnose: {error}");
            return Ok(ExitCode::from(2));
        }
    };

    let (document, diagnostics) = read(&sources, id);

    if diagnostics.is_empty() {
        emit(
            &mut stdout,
            &format!("read cleanly: {} top-level form(s)", document.forms.len()),
        )?;
    } else {
        for line in diagnostics.render(&sources).lines() {
            emit(&mut stdout, line)?;
        }
        emit(
            &mut stdout,
            &format!(
                "{} diagnostic(s); {} top-level form(s) recovered",
                diagnostics.len(),
                document.forms.len()
            ),
        )?;
    }

    let headers = document.comment_headers();
    if !headers.is_empty() {
        emit(&mut stdout, "")?;
        emit(&mut stdout, "headers:")?;
        for (key, value) in &headers {
            let shown = if value.chars().count() > 72 {
                format!("{}…", value.chars().take(72).collect::<String>())
            } else {
                value.clone()
            };
            emit(&mut stdout, &format!("  {key}: {shown}"))?;
        }
    }

    if !document.forms.is_empty() {
        emit(&mut stdout, "")?;
        emit(&mut stdout, "canonical:")?;
        for line in document.to_canonical().lines() {
            emit(&mut stdout, &format!("  {line}"))?;
        }
    }

    // Flush explicitly: a drop-time flush failure would be discarded, turning truncated output
    // into a silent success.
    if let Err(error) = stdout.flush() {
        if error.kind() != ErrorKind::BrokenPipe {
            eprintln!("diagnose: cannot flush output: {error}");
            return Ok(ExitCode::from(2));
        }
    }

    Ok(if diagnostics.has_errors() {
        ExitCode::from(1)
    } else {
        ExitCode::SUCCESS
    })
}
