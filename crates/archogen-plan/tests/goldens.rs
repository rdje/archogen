//! `archogen-runtime-plan/1`'s goldens (leaf `M4.1.4.3`, `docs/decisions/decision_runtime-plan.md` §3): "Each golden is
//! lowered from frozen inputs, the crate's own fixtures and never another suite's file, its result record a fixed text, and
//! it is never rewritten."
//!
//! So nothing here writes a file, and no environment variable blesses one: a golden that moves is a new format number (§3).
//! Each is held four ways: the production entry lowers it from its frozen assignment and result record, byte for byte and
//! twice alike; its reader reads back the same bytes; the record's worked text is it, its digests aside; and the checker,
//! asked again today with the goldens' fixed producer, writes that result record, so the fixed text is what it says.

mod common;

use std::fs;
use std::path::PathBuf;

use archogen_digest::Digest;
use archogen_outcome::record::Producer;
use archogen_plan::{lower_checked, Acceptance, Generator, Plan};
use archogen_plan_check::judgement::{judge, Inputs};
use common::{checked, registry, root, without_digests};
use eadl_front::module::MemoryModules;
use eadl_model::profile::RT_STATIC_UP_V1;

/// The golden's frozen input, a fixture of the crate's corpus, and the record's worked text it is: the heartbeat, the
/// epoch extender and the sporadic system, in the record's order.
const GOLDENS: [(&str, &str, usize); 3] = [
    ("heartbeat", "worked-heartbeat", 0),
    ("epoch-extender", "worked-epoch-extender", 1),
    ("sporadic", "sporadic", 2),
];

/// Each golden plan's sha256, the plan's identity (§3): a golden is never rewritten, so that its file and the code moving
/// together still fail here. A change that moves one is a new format number, not a new digest here.
const IDENTITIES: [(&str, &str); 3] = [
    (
        "epoch-extender",
        "d7102b3428ac5193cac63c85785690fe3cf59cf883bfc7839422eaba45a36835",
    ),
    (
        "heartbeat",
        "390fb545f810c644ba38a4341dc4a158f2d586d3c728dea3879867e0fd529fa2",
    ),
    (
        "sporadic",
        "69af9ca0cf319a2df457af62f63ce7262220eea5e6095dfd2881d1474f31a0dd",
    ),
];

/// The fixed producer the goldens' result records name.
fn producer() -> Producer {
    Producer {
        name: "archogen-plan-check".to_owned(),
        sha256: Digest::of(b"the goldens' fixed checker"),
    }
}

fn generator() -> Generator {
    Generator {
        version: "0.1.0".to_owned(),
        sha256: Digest::of(b"the goldens' fixed generator"),
    }
}

fn golden(name: &str, extension: &str) -> String {
    let path: PathBuf = root()
        .join("crates/archogen-plan/tests/goldens")
        .join(format!("{name}.{extension}"));
    fs::read_to_string(&path).unwrap_or_else(|e| panic!("{}: {e}", path.display()))
}

/// The fixture's text as the language reads it: the sporadic system is the record's own block, with no version line.
fn description(fixture: &str) -> String {
    let path = root().join(format!("crates/archogen-plan/tests/corpus/{fixture}.eadl"));
    let text = fs::read_to_string(path).expect("the fixture");
    if text.contains("(eadl-version") {
        text
    } else {
        format!("(eadl-version eadl/1)\n{text}")
    }
}

/// The record's plan blocks with their digests elided, in its order.
fn worked_plans() -> Vec<String> {
    let record = fs::read_to_string(root().join("docs/decisions/decision_runtime-plan.md"))
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
    out.retain(|b| {
        b.starts_with("(format \"archogen-runtime-plan/1\")") && b.contains("(program \"…\")")
    });
    out
}

#[test]
fn each_golden_is_what_the_production_entry_lowers_from_its_frozen_inputs_twice_alike() {
    for (name, fixture, _) in GOLDENS {
        let program = checked(&description(fixture)).expect("the language accepts it");
        let (assignment, result, plan) = (
            golden(name, "assignment"),
            golden(name, "result"),
            golden(name, "plan"),
        );
        let lower = || {
            lower_checked(
                &program,
                &assignment,
                &result,
                &registry(),
                generator(),
                Acceptance::CheckedByFront,
            )
            .unwrap_or_else(|refusal| panic!("{name}: {refusal:?}"))
        };
        let lowered = lower();
        assert_eq!(lowered.plan.text(), plan, "{name}: the plan moved");
        assert_eq!(lower(), lowered, "{name}: lowered twice alike");
        assert_eq!(
            lowered.provenance.result,
            Digest::of(result.as_bytes()),
            "{name}"
        );
        assert_eq!(lowered.provenance.checker, producer().sha256, "{name}");
    }
}

#[test]
fn each_golden_reads_back_to_the_same_bytes_and_its_hash_is_its_text_s() {
    for (name, _, _) in GOLDENS {
        let text = golden(name, "plan");
        let plan = Plan::read(&text).unwrap_or_else(|why| panic!("{name}: {why}"));
        assert_eq!(plan.text(), text, "{name}");
        assert_eq!(plan.sha256(), Digest::of(text.as_bytes()), "{name}");
    }
}

#[test]
fn each_golden_keeps_its_identity() {
    for (name, identity) in IDENTITIES {
        assert_eq!(
            Digest::of(golden(name, "plan").as_bytes()).hex(),
            identity,
            "{name}"
        );
    }
}

#[test]
fn each_golden_is_the_record_s_worked_text_its_digests_aside() {
    let worked = worked_plans();
    assert_eq!(worked.len(), GOLDENS.len());
    for (name, _, place) in GOLDENS {
        assert_eq!(
            without_digests(&golden(name, "plan")),
            worked[place],
            "{name}"
        );
    }
}

#[test]
fn each_fixed_result_record_is_what_the_checker_writes_for_its_inputs() {
    for (name, fixture, _) in GOLDENS {
        let (text, assignment) = (description(fixture), golden(name, "assignment"));
        let record = judge(
            producer(),
            &Inputs {
                description_name: "description.eadl",
                description: &text,
                modules: &MemoryModules::new(),
                profile: &RT_STATIC_UP_V1,
                plan_name: "plan.eadl",
                plan: assignment.as_bytes(),
            },
        )
        .unwrap_or_else(|why| panic!("{name}: {why:?}"));
        assert!(record.accepts(), "{name}: {:?}", record.findings());
        assert_eq!(record.write(), golden(name, "result"), "{name}");
    }
}
