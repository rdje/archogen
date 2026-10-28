//! The record's worked texts are what this code writes (`docs/decisions/decision_runtime-plan.md` §3): the rules stated
//! twice — as the record's prose and as this transcription of it — say the same thing, line for line, and a change to
//! either that moves a worked text fails here.

mod common;

use common::{lowered, root, without_digests, Lowered};

/// The record's `text` blocks, in order.
fn blocks() -> Vec<String> {
    let record = std::fs::read_to_string(root().join("docs/decisions/decision_runtime-plan.md"))
        .expect("the record");
    let mut out = Vec::new();
    let mut open: Option<String> = None;
    for line in record.lines() {
        match (&mut open, line.trim_start()) {
            (None, "```text") => open = Some(String::new()),
            (Some(block), "```") => {
                out.push(std::mem::take(block));
                open = None;
            }
            (Some(block), _) => {
                block.push_str(line);
                block.push('\n');
            }
            (None, _) => {}
        }
    }
    out
}

/// The worked plans: the blocks that are a plan with its digests elided, in the record's order.
fn worked_plans() -> Vec<String> {
    blocks()
        .into_iter()
        .filter(|b| {
            b.starts_with("(format \"archogen-runtime-plan/1\")") && b.contains("(program \"…\")")
        })
        .collect()
}

fn plan_of(text: &str) -> String {
    match lowered(text, None) {
        Lowered::Plan(plan) => without_digests(&plan),
        other => panic!("not lowered: {other:?}"),
    }
}

#[test]
fn the_record_holds_three_worked_texts() {
    assert_eq!(worked_plans().len(), 3);
}

#[test]
fn the_heartbeat_lowers_to_its_worked_text() {
    // `examples/s0-heartbeat/system.eadl`, frozen as the corpus's fixture (§3).
    let text = std::fs::read_to_string(
        root().join("crates/archogen-plan/tests/corpus/worked-heartbeat.eadl"),
    )
    .expect("the heartbeat");
    assert_eq!(plan_of(&text), worked_plans()[0]);
}

#[test]
fn the_epoch_extender_lowers_to_its_worked_text() {
    // `crates/eadl-resolve/tests/search/adapter-epoch-extender.eadl`, frozen as the corpus's fixture (§3).
    let text = std::fs::read_to_string(
        root().join("crates/archogen-plan/tests/corpus/worked-epoch-extender.eadl"),
    )
    .expect("the epoch extender");
    assert_eq!(plan_of(&text), worked_plans()[1]);
}

#[test]
fn the_record_s_sporadic_system_lowers_to_its_worked_text() {
    // The description is the record's own block, so the fixture and its text cannot drift apart.
    let description = blocks()
        .into_iter()
        .find(|b| b.starts_with("(defblock dev.e"))
        .expect("the record writes the sporadic system");
    assert_eq!(
        plan_of(&format!("(eadl-version eadl/1)\n{description}")),
        worked_plans()[2]
    );
}
