//! A request's byte budget (leaf `API.4.2`): what an instance lets one request cost.
//!
//! Each edge is pinned — a request exactly at its budget is judged, one byte over is `tool-failure` — through the
//! description and through the modules, and a request within its budget is shown to get the very answer it gets
//! with no budget, so the budget never changes a judgement, only whether one is made.

use archogen_api::{
    check, check_with, Limits, MemoryModules, NoModules, Request, Response, Status, DEFAULT_BYTES,
};

const DESCRIPTION: &str = "(defblock console.uart (offers observable-output))\n";
const ROOT: &str = "(defmodule app (version 1 0) (import hw.timer (as timer)))\n";
const TIMER: &str = "(defmodule hw.timer (version 1 0))\n";

fn with_budget(text: &str, modules: &MemoryModules, bytes: usize) -> Response {
    check_with(
        &Request {
            name: "request.eadl",
            text,
            profile: None,
            modules,
        },
        Limits { bytes: Some(bytes) },
    )
}

/// A refusal for the budget: `tool-failure`, a note that names the budget, and nothing about the description.
fn assert_over_budget(response: &Response, budget: usize) {
    assert_eq!(response.status, Status::ToolFailure);
    assert!(
        response.judged.is_none(),
        "a judgement cut short is not one"
    );
    assert!(
        response.diagnostics.is_empty(),
        "nothing is said about the description: {}",
        response.render_diagnostics()
    );
    assert!(
        response.notes[0].contains(&format!("budget of {budget} bytes")),
        "{:?}",
        response.notes
    );
}

#[test]
fn a_description_exactly_at_its_budget_is_judged_and_one_byte_over_is_refused() {
    let none = MemoryModules::new();
    let at = with_budget(DESCRIPTION, &none, DESCRIPTION.len());
    assert_eq!(at.status, Status::Ok, "{}", at.render_diagnostics());
    let over = with_budget(DESCRIPTION, &none, DESCRIPTION.len() - 1);
    assert_over_budget(&over, DESCRIPTION.len() - 1);
}

#[test]
fn the_modules_a_request_loads_count_against_the_same_budget() {
    let modules = MemoryModules::new().with("hw.timer", TIMER);
    let budget = ROOT.len() + TIMER.len();
    let at = with_budget(ROOT, &modules, budget);
    assert_eq!(at.status, Status::Ok, "{}", at.render_diagnostics());
    let over = with_budget(ROOT, &modules, budget - 1);
    assert_over_budget(&over, budget - 1);
    assert!(
        over.notes[0].contains("`hw.timer`"),
        "the note names the module that went past: {:?}",
        over.notes
    );
}

#[test]
fn every_instance_s_load_counts_so_the_budget_bounds_the_parsing_too() {
    // Each import makes an instance, and each instance reads its module again. A root importing the same module
    // a hundred times brings its text a hundred times, and the budget counts every one of them.
    let imports: String = (0..100)
        .map(|i| format!(" (import hw.timer (as t{i}))"))
        .collect();
    let root = format!("(defmodule app (version 1 0){imports})\n");
    let modules = MemoryModules::new().with("hw.timer", TIMER);
    let at = with_budget(&root, &modules, root.len() + 100 * TIMER.len());
    assert_eq!(at.status, Status::Ok, "{}", at.render_diagnostics());
    let over = with_budget(&root, &modules, root.len() + 100 * TIMER.len() - 1);
    assert_over_budget(&over, root.len() + 100 * TIMER.len() - 1);
}

#[test]
fn a_request_within_its_budget_gets_the_answer_it_gets_with_no_budget() {
    let modules = MemoryModules::new().with("hw.timer", TIMER);
    for (text, modules) in [(DESCRIPTION, &MemoryModules::new()), (ROOT, &modules)] {
        let limited = with_budget(text, modules, 1_000);
        let unlimited = check_with(
            &Request {
                name: "request.eadl",
                text,
                profile: None,
                modules,
            },
            Limits::NONE,
        );
        assert_eq!(format!("{limited:?}"), format!("{unlimited:?}"));
    }
}

#[test]
fn check_applies_the_default_budget() {
    let mut text = String::from(DESCRIPTION);
    text.push_str(&format!("; {}\n", "x".repeat(DEFAULT_BYTES)));
    let response = check(&Request {
        name: "large.eadl",
        text: &text,
        profile: None,
        modules: &NoModules,
    });
    assert_over_budget(&response, DEFAULT_BYTES);
    assert_eq!(Limits::DEFAULT.bytes, Some(DEFAULT_BYTES));
}
