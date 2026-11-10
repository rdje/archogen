//! What is no block or platform, and an offer of `false`, say nothing to presence of a fact the vocabulary does not declare (model §2 rule 3; leaf `M3.4.17.2`).
//!
//! The search binds nothing for a fact outside the vocabulary, so presence alone judges a need on one, and a plan comes of what it accepts. It read an offer by its
//! name: a service's `(offers sha-accel)` answered a need though the search binds nothing to a service, and a block's `(sha-accel false)` answered one though it says
//! the block has none. Either way a plan came for a machine without the device. A fact of the vocabulary is read as before, the search judging it by value and by
//! who offers it. A census of the 334 tracked descriptions found no valid one resting on either (`target/m3417/work/census.py`).

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use eadl_front::{SourceMap, Verdict};
use eadl_model::check::{
    check, check_with_vocabulary, default_profile, shipped_registry, VocabularyFacts,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Each fact the capability vocabulary declares, by name, as a caller hands them down.
fn vocabulary() -> VocabularyFacts {
    let declared =
        std::fs::read_to_string(repo_root().join("docs/semantics/vocabulary/vocabulary.eadl"))
            .expect("the vocabulary")
            .lines()
            .filter_map(|line| line.strip_prefix("(deffact "))
            .map(|rest| rest.trim().to_string())
            .collect();
    VocabularyFacts {
        declared,
        derived: BTreeMap::new(),
    }
}

/// The verdict of the check on `text`, told the vocabulary or not, and each error it gives as `code: message`.
fn judged(text: &str, told_the_vocabulary: bool) -> (Verdict, Vec<String>) {
    let mut sources = SourceMap::new();
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|relative| {
        (
            (*relative).to_string(),
            std::fs::read_to_string(repo_root().join(relative)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut sources, &kinds).expect("the shipped kinds read");
    let id = sources
        .add("t.eadl", format!("(eadl-version eadl/1)\n{text}"))
        .expect("small");
    let outcome = if told_the_vocabulary {
        check_with_vocabulary(&sources, id, &registry, default_profile(), &vocabulary())
    } else {
        check(&sources, id, &registry, default_profile())
    };
    let said = outcome
        .diagnostics
        .iter()
        .map(|d| format!("{}: {}", d.code, d.message))
        .collect();
    (outcome.verdict, said)
}

fn checked(text: &str) -> (Verdict, Vec<String>) {
    judged(text, true)
}

/// A service offers what another needs, and no block or platform of the machine does.
const BY_A_SERVICE: &str = "
(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)))
(defservice provider.x (requires (needs observable-output)) (offers sha-accel))
(defservice hash (requires (needs sha-accel)))
(defsystem s (requires (uses hash)) (platform (uses soc.here)))";

#[test]
fn a_service_s_offer_answers_no_need() {
    let (verdict, said) = checked(BY_A_SERVICE);
    assert_eq!(verdict, Verdict::MissingFact, "{said:?}");
    assert!(
        said.iter()
            .any(|d| d.starts_with("missing-fact: `sha-accel`") && d.contains("`provider.x`")),
        "the refusal names what offers it and is no block or platform: {said:?}"
    );
}

#[test]
fn a_block_of_the_hardware_answers_whatever_a_service_offers_beside_it() {
    let with_a_block = BY_A_SERVICE.replace(
        "(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)))",
        "(defblock accel (offers sha-accel (reachable-at-privilege machine)))\n(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses accel)))",
    );
    assert_ne!(with_a_block, BY_A_SERVICE);
    assert_eq!(
        checked(&with_a_block).0,
        Verdict::Ok,
        "{:?}",
        checked(&with_a_block).1
    );
}

#[test]
fn an_absence_a_service_declares_says_nothing_of_the_machine() {
    // Declared absent by a service and offered by the platform: the machine has it, and a service is no part of the machine.
    let absent_by_a_service = BY_A_SERVICE
        .replace(
            "(defservice provider.x (requires (needs observable-output)) (offers sha-accel))",
            "(defservice provider.x (requires (needs observable-output)) (absent sha-accel))",
        )
        .replace(
            "(offers (core-count 1 tick) (reachable-at-privilege machine))",
            "(offers sha-accel (core-count 1 tick) (reachable-at-privilege machine))",
        );
    assert_eq!(
        checked(&absent_by_a_service).0,
        Verdict::Ok,
        "{:?}",
        checked(&absent_by_a_service).1
    );
}

#[test]
fn an_offer_of_false_answers_no_need() {
    let offered = |value: &str| {
        format!(
            "
(defblock accel (offers {value} (reachable-at-privilege machine)))
(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses accel)))
(defservice hash (requires (needs sha-accel)))
(defsystem s (requires (uses hash)) (platform (uses soc.here)))"
        )
    };
    for stated in ["sha-accel", "(sha-accel true)", "(sha-accel 2 tick)"] {
        assert_eq!(
            checked(&offered(stated)).0,
            Verdict::Ok,
            "{stated}: {:?}",
            checked(&offered(stated)).1
        );
    }
    let (verdict, said) = checked(&offered("(sha-accel false)"));
    assert_eq!(verdict, Verdict::MissingFact, "{said:?}");
    assert!(
        said.iter()
            .any(|d| d.starts_with("missing-fact: `sha-accel`")
                && d.contains("`accel`")
                && d.contains("`false`")),
        "the refusal names the block that offers it `false`: {said:?}"
    );
}

#[test]
fn a_fact_of_the_vocabulary_is_read_as_before() {
    // The search judges it, by value and by who offers it (model §2 rule 2): a service's offer of one, and a block's offer of `false`, are accepted here.
    let by_a_service = BY_A_SERVICE.replace("sha-accel", "relative-delay");
    assert_eq!(
        checked(&by_a_service).0,
        Verdict::Ok,
        "{:?}",
        checked(&by_a_service).1
    );
}

#[test]
fn a_check_told_no_vocabulary_reads_every_description_whole() {
    assert_eq!(judged(BY_A_SERVICE, false).0, Verdict::Ok);
}
