//! Check descriptions held in memory through the engine API: the example `docs/book/src/engine-api.md` shows.
//! `tests/book_example.rs` holds the chapter's transcript to what [`run`] prints, so the two cannot drift.
//!
//! No file is read and none is written: every description is a string, and imports would resolve through the
//! `ModuleSource` passed in, here one that holds nothing.

use std::io::Write;

use archogen_api::{check, NoModules, Request, Response, TargetFile};

/// A system whose one task writes to a console.
const BEAT: &str = "(defservice console.write (requires (needs observable-output)))
(defsystem beat
  (task tick (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write) (on-overrun fault))
  (requires (uses console.write)))
";

/// A machine with a console, one core, memory to place an image in, and `NMI` sources no mask stops.
fn board(nmi: u32) -> String {
    format!(
        "(defblock board.uart (offers (observable-output true)))
(defplatform board.soc
  (offers (core-count 1 tick) (non-maskable-interrupts {nmi}) (firmware-traps 0)
    (region board.ram (base 0x8000_0000) (size 1 MiB) (executable true)))
  (requires (uses board.uart)))
"
    )
}

/// Five requests: one accepted, one judged and refused, one the API does not judge, and a system on a machine that admits
/// it, then on one that does not.
pub fn run(out: &mut dyn Write) {
    let accepted = check(&Request {
        name: "uart.eadl",
        text: "(defblock console.uart (offers observable-output))\n",
        profile: None,
        modules: &NoModules,
        target: None,
    });
    show(out, &accepted);

    let refused = check(&Request {
        name: "timer.eadl",
        text: "(defblock timer.counter (offers (tick-rate 10 parsec)))\n",
        profile: None,
        modules: &NoModules,
        target: None,
    });
    show(out, &refused);

    let not_judged = check(&Request {
        name: "uart.eadl",
        text: "(defblock console.uart (offers observable-output))\n",
        profile: Some("rt-dynamic-mp"),
        modules: &NoModules,
        target: None,
    });
    show(out, &not_judged);

    for nmi in [0, 1] {
        let target = board(nmi);
        let on_board = check(&Request {
            name: "beat.eadl",
            text: BEAT,
            profile: None,
            modules: &NoModules,
            target: Some(TargetFile {
                name: "board.eadl",
                text: &target,
            }),
        });
        show(out, &on_board);
    }
}

/// One line for the outcome, then what the response says about it.
fn show(out: &mut dyn Write, response: &Response) {
    let _ = writeln!(
        out,
        "api {} · engine {} · status {}",
        response.version,
        response.engine,
        response.status.slug()
    );
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
