//! The falsification corpus (`docs/decisions/decision_executable-design-reviews.md` §3): every probe system of the
//! runtime plan's review (`docs/reviews/decision_runtime-plan-reviews.md`, rounds 4 to 7), and the inputs of the
//! record's worked texts (`worked-*.eadl`, frozen here), each with what the lowering gives each of its systems — the
//! plan, with its two digests written `…`, or the refusal — in `tests/corpus/NAME.expected`. A description the
//! language's check refuses is no input (the record's §1): its outcome is that check's verdict, `rejected …`.
//!
//! ⭐ **An outcome here is what the code writes, blessed — evidence, never a rule.** A review judges each against the
//! record's rules, so a fixture the code gets wrong stays green until a reader compares the two. A later finding is a
//! defect when it carries a reproducer: a fixture here the code gets wrong against the record, or two quoted sentences
//! of the record that contradict each other. A fixed defect stays fixed, its fixture keeping the text the record gives
//! it.
//!
//! Regenerate deliberately, so every moved outcome is in the change's diff for review:
//! `ARCHOGEN_BLESS_PLANS=1 cargo test -p archogen-plan --test corpus`.

mod common;

use std::fs;

use common::{lowered, root, without_digests, Lowered};

/// Every system of `text`, by its name, in written order.
fn systems(text: &str) -> Vec<String> {
    let mut sources = eadl_front::SourceMap::new();
    let id = sources.add("description", text.to_owned()).expect("small");
    eadl_front::read(&sources, id)
        .0
        .forms
        .iter()
        .filter(|f| f.head() == Some("defsystem"))
        .filter_map(|f| f.items().get(1).and_then(eadl_front::Form::as_symbol))
        .map(str::to_string)
        .collect()
}

/// What the corpus records of one description: each system's outcome under `== system NAME`.
fn outcome(text: &str) -> String {
    let mut out = String::new();
    for system in systems(text) {
        out.push_str(&format!("== system {system}\n"));
        match lowered(text, Some(&system)) {
            Lowered::Plan(plan) => out.push_str(&without_digests(&plan)),
            Lowered::Refused(refusal) => out.push_str(&format!("refused {refusal}\n")),
            Lowered::Unresolved(verdict) => out.push_str(&format!("unresolved {verdict}\n")),
            Lowered::Rejected(verdict) => {
                out.push_str(&format!("rejected {verdict}\n"));
                break;
            }
        }
    }
    out
}

#[test]
fn every_fixture_lowers_as_the_corpus_records() {
    let dir = root().join("crates/archogen-plan/tests/corpus");
    let bless = std::env::var_os("ARCHOGEN_BLESS_PLANS").is_some();
    let mut fixtures: Vec<_> = fs::read_dir(&dir)
        .expect("the corpus")
        .flatten()
        .map(|e| e.path())
        .filter(|p| p.extension().is_some_and(|e| e == "eadl"))
        .collect();
    fixtures.sort();
    assert!(!fixtures.is_empty());
    let mut moved = Vec::new();
    for fixture in &fixtures {
        let text = fs::read_to_string(fixture).expect("readable");
        let got = outcome(&text);
        let expected = fixture.with_extension("expected");
        if bless {
            fs::write(&expected, &got).expect("writable");
            continue;
        }
        let want = fs::read_to_string(&expected).unwrap_or_default();
        if got != want {
            moved.push(format!(
                "{}:\n--- recorded\n{want}--- lowered\n{got}",
                fixture.display()
            ));
        }
    }
    assert!(moved.is_empty(), "{}", moved.join("\n"));
}

#[test]
fn every_expected_outcome_has_its_fixture() {
    let dir = root().join("crates/archogen-plan/tests/corpus");
    for entry in fs::read_dir(&dir).expect("the corpus").flatten() {
        let path = entry.path();
        if path.extension().is_some_and(|e| e == "expected") {
            assert!(
                path.with_extension("eadl").exists(),
                "{} has no fixture",
                path.display()
            );
        }
    }
}
