//! Presence reads a derived fact as the search derives it (model §2 rule 2; leaf `M3.4.17.1`).
//!
//! A fact the vocabulary derives is stated by no one: the search computes it at a provider that offers each fact its rule requires, and binds there. Presence read
//! only what is written, so a need on `unambiguous-horizon` written directly was `missing-fact` from the check while the search derived it and bound it, on one and the
//! same description. So a derived fact is described where one block or platform of the system's hardware offers each of its rule's facts, and the search judges it by
//! value. Where a block offers part of them, the refusal names the rest. It is a fact of the vocabulary, so the description is read whole for it, as for any such fact:
//! a block outside the system's hardware is the search's to refuse (`M3.4.16`).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use eadl_front::{SourceMap, Verdict};
use eadl_model::check::{
    check_with_vocabulary, default_profile, shipped_registry, VocabularyFacts,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// What the capability vocabulary says that the check reads, as a caller hands it down: each fact it declares, and each derived fact with the facts its rule requires.
fn told() -> VocabularyFacts {
    let text =
        std::fs::read_to_string(repo_root().join("docs/semantics/vocabulary/vocabulary.eadl"))
            .expect("the vocabulary");
    let (mut declared, mut derived, mut last) = (BTreeSet::new(), BTreeMap::new(), String::new());
    for line in text.lines() {
        if let Some(name) = line.strip_prefix("(deffact ") {
            last = name.trim().to_string();
            declared.insert(last.clone());
        }
        if let Some(rest) = line.trim().strip_prefix("(derived-from ") {
            let inputs: Vec<String> = rest
                .trim_end_matches(')')
                .split_whitespace()
                .map(str::to_string)
                .collect();
            derived.insert(last.clone(), inputs);
        }
    }
    VocabularyFacts { declared, derived }
}

/// The verdict of the check on `text`, told the vocabulary, and each error it gives as `code: message`.
fn checked(text: &str) -> (Verdict, Vec<String>) {
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
    let outcome = check_with_vocabulary(&sources, id, &registry, default_profile(), &told());
    let said = outcome
        .diagnostics
        .iter()
        .map(|d| format!("{}: {}", d.code, d.message))
        .collect();
    (outcome.verdict, said)
}

/// Its reader's case (`M4.4.1.3`, D3): the timer offers a modulus and a rate, and a service needs the horizon the vocabulary derives of the two.
const DERIVED: &str = "
(defblock own.timer (offers
    (relative-delay true) (channels 2 tick) (counter-modulus (pow2 32)) (wrap-behavior modular) (tick-rate 10 MHz) (observation-coherent true) (reachable-at-privilege machine)))
(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses own.timer)))
(defservice uh (requires (needs unambiguous-horizon)))
(defsystem s (requires (uses uh)) (platform (uses soc.here)))";

#[test]
fn the_vocabulary_derives_the_horizon_of_a_modulus_and_a_rate() {
    assert_eq!(
        told().derived,
        BTreeMap::from([(
            "unambiguous-horizon".to_string(),
            vec!["counter-modulus".to_string(), "tick-rate".to_string()]
        )])
    );
}

#[test]
fn a_derived_fact_is_described_where_one_block_offers_each_fact_its_rule_requires() {
    let (verdict, said) = checked(DERIVED);
    assert_eq!(verdict, Verdict::Ok, "{said:?}");
}

#[test]
fn a_block_offering_part_of_what_the_rule_requires_is_told_the_rest() {
    let partial = DERIVED.replace(" (tick-rate 10 MHz)", "");
    assert_ne!(partial, DERIVED);
    let (verdict, said) = checked(&partial);
    assert_eq!(verdict, Verdict::MissingFact, "{said:?}");
    assert!(
        said.iter()
            .any(|d| d.starts_with("missing-fact: `unambiguous-horizon`")
                && d.contains("`own.timer`")
                && d.contains("`tick-rate`")
                && d.contains("derive")),
        "the refusal names the block and the fact it leaves undescribed: {said:?}"
    );
}

#[test]
fn two_blocks_each_offering_one_of_them_describe_it_nowhere() {
    // The search derives at one provider: a modulus on one block and a rate on another are two devices.
    let split = DERIVED.replace(" (tick-rate 10 MHz)", "").replace(
        "(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses own.timer)))",
        "(defblock other.clock (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))\n(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses own.timer other.clock)))",
    );
    assert_ne!(split, DERIVED);
    let (verdict, said) = checked(&split);
    assert_eq!(verdict, Verdict::MissingFact, "{said:?}");
}

#[test]
fn a_derived_fact_no_one_needs_and_nothing_derives_fails_nothing() {
    // No service needs the horizon: a timer that offers no rate is as good as before.
    let unneeded = DERIVED
        .replace(" (tick-rate 10 MHz)", "")
        .replace("(needs unambiguous-horizon)", "(needs relative-delay)");
    let (verdict, said) = checked(&unneeded);
    assert_eq!(verdict, Verdict::Ok, "{said:?}");
}
