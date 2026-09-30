//! Check descriptions held in memory through the engine API: the example `docs/book/src/engine-api.md` shows.
//! `tests/book_example.rs` holds the chapter's transcript to what [`run`] prints, so the two cannot drift.
//!
//! No file is read and none is written: every description is a string, and imports would resolve through the
//! `ModuleSource` passed in, here one that holds nothing.

use std::io::Write;

use archogen_api::{check, NoModules, Request, Response, VERSION};

/// Three requests: one accepted, one judged and refused, one the API does not judge.
pub fn run(out: &mut dyn Write) {
    let accepted = check(&Request {
        name: "uart.eadl",
        text: "(defblock console.uart (offers observable-output))\n",
        profile: None,
        modules: &NoModules,
    });
    show(out, &accepted);

    let refused = check(&Request {
        name: "timer.eadl",
        text: "(defblock timer.counter (offers (tick-rate 10 parsec)))\n",
        profile: None,
        modules: &NoModules,
    });
    show(out, &refused);

    let not_judged = check(&Request {
        name: "uart.eadl",
        text: "(defblock console.uart (offers observable-output))\n",
        profile: Some("rt-dynamic-mp"),
        modules: &NoModules,
    });
    show(out, &not_judged);
}

/// One line for the outcome, then what the response says about it.
fn show(out: &mut dyn Write, response: &Response) {
    let _ = writeln!(out, "api {VERSION} · status {}", response.status.slug());
    if let Some(judged) = &response.judged {
        let _ = writeln!(
            out,
            "  judged under {}, profile {}: {} declaration(s)",
            judged.language,
            judged.profile,
            judged.declarations.len()
        );
    }
    for diagnostic in &response.diagnostics {
        let _ = writeln!(out, "  {}: {}", diagnostic.code, diagnostic.message);
    }
    for note in &response.notes {
        let _ = writeln!(out, "  not judged: {note}");
    }
}

#[allow(dead_code)]
fn main() {
    run(&mut std::io::stdout());
}
