//! Presence reads a system's own hardware, as the search does (model §8 rule 7; leaf `M3.4.16`).
//!
//! A need on a fact the vocabulary does not declare is judged by presence alone, the search binding nothing for it. So a
//! block that is none of the system's hardware must not answer one: the plan that would come is for a machine without the
//! device. What answers is a block or a platform the system's `platform` clause, its own `uses` and `needs` or its tasks'
//! name, and those they name in turn. A block outside it still says what it says: its name is declared, a contradiction of
//! its own is one, and what it offers stays visible outside the closure. A fact the vocabulary declares is read as before:
//! the search binds it, and refuses a provider outside the hardware itself.

use std::path::{Path, PathBuf};

use eadl_front::{SourceMap, Verdict};
use std::collections::BTreeMap;

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

/// Each fact the capability vocabulary declares, by name, read as a caller of the check hands them down. What it derives is `presence_derived.rs`'s to hold.
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
        let text = std::fs::read_to_string(repo_root().join(relative)).expect("a kind module");
        ((*relative).to_string(), text)
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

/// As the commands check a description: told the vocabulary.
fn checked(text: &str) -> (Verdict, Vec<String>) {
    judged(text, true)
}

/// The reader's case: `accel` offers what the service needs, and no part of `soc.here` is `accel`.
const OUTSIDE: &str = "
(defblock accel (offers sha-accel (reachable-at-privilege machine)))
(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)))
(defservice hash (requires (needs sha-accel)))
(defsystem s (requires (uses hash)) (platform (uses soc.here)))";

#[test]
fn a_block_outside_the_system_s_hardware_answers_no_need() {
    let (verdict, said) = checked(OUTSIDE);
    assert_eq!(verdict, Verdict::MissingFact, "{said:?}");
    assert!(
        said.iter()
            .any(|d| d.starts_with("missing-fact: `sha-accel`") && d.contains("`accel`")),
        "the refusal names the block that offers it and is none of the hardware: {said:?}"
    );
}

#[test]
fn a_block_the_hardware_names_answers_at_any_depth() {
    // The platform composes it.
    let composed = OUTSIDE.replace(
        "(reachable-at-privilege machine)))\n(defservice",
        "(reachable-at-privilege machine)) (requires (uses accel)))\n(defservice",
    );
    assert_ne!(composed, OUTSIDE);
    assert_eq!(
        checked(&composed).0,
        Verdict::Ok,
        "{:?}",
        checked(&composed).1
    );
    // A block the platform composes composes it.
    let deeper = format!(
        "(defblock bus (offers (reachable-at-privilege machine)) (requires (uses accel)))\n{}",
        OUTSIDE.replace(
            "(reachable-at-privilege machine)))\n(defservice",
            "(reachable-at-privilege machine)) (requires (uses bus)))\n(defservice",
        )
    );
    assert_eq!(checked(&deeper).0, Verdict::Ok, "{:?}", checked(&deeper).1);
    // The system names it itself, beside its platform.
    let named = OUTSIDE.replace("(requires (uses hash))", "(requires (uses hash accel))");
    assert_ne!(named, OUTSIDE);
    assert_eq!(checked(&named).0, Verdict::Ok, "{:?}", checked(&named).1);
}

#[test]
fn a_block_only_a_service_names_is_none_of_the_hardware() {
    // Rule 7: "A declaration of another kind ends the path: a block that only a service names is not hardware."
    let through_a_service = OUTSIDE.replace(
        "(defservice hash (requires (needs sha-accel)))",
        "(defservice hash (requires (needs sha-accel) (uses accel)))",
    );
    assert_ne!(through_a_service, OUTSIDE);
    let (verdict, said) = checked(&through_a_service);
    assert_eq!(verdict, Verdict::MissingFact, "{said:?}");
}

#[test]
fn an_absence_outside_the_hardware_says_nothing_of_the_system_s_machine() {
    // Declared absent by a block the system does not have, and offered by none it has: undescribed there, not infeasible.
    let absent_outside = OUTSIDE.replace(
        "(offers sha-accel (reachable",
        "(absent sha-accel) (offers (reachable",
    );
    assert_ne!(absent_outside, OUTSIDE);
    let (verdict, said) = checked(&absent_outside);
    assert_eq!(verdict, Verdict::MissingFact, "{said:?}");
    // Declared absent by the platform it has: a definite answer, whatever a block outside offers.
    let absent_inside = OUTSIDE.replace(
        "(defplatform soc.here (offers",
        "(defplatform soc.here (absent sha-accel) (offers",
    );
    assert_ne!(absent_inside, OUTSIDE);
    let (verdict, said) = checked(&absent_inside);
    assert_eq!(verdict, Verdict::InfeasibleConfiguration, "{said:?}");
}

#[test]
fn a_block_outside_the_hardware_still_says_what_it_says() {
    // Its contradiction is one (model §2 rule 1), whoever has it.
    let contradicts = OUTSIDE.replace(
        "(offers sha-accel (reachable",
        "(absent sha-accel) (offers sha-accel (reachable",
    );
    assert_ne!(contradicts, OUTSIDE);
    let (verdict, said) = checked(&contradicts);
    assert_eq!(verdict, Verdict::InvalidDescription, "{said:?}");
    assert!(
        said.iter().any(|d| d.contains("both offered and absent")),
        "{said:?}"
    );
    // With nothing needing what it offers, it is no one's concern.
    let unneeded = OUTSIDE.replace(
        "(defservice hash (requires (needs sha-accel)))",
        "(defservice hash (requires (needs core-count)))",
    );
    assert_ne!(unneeded, OUTSIDE);
    assert_eq!(
        checked(&unneeded).0,
        Verdict::Ok,
        "{:?}",
        checked(&unneeded).1
    );
}

#[test]
fn each_system_is_read_on_its_own_hardware() {
    // `first` has the block and `second` does not: one description, two machines, and the second lacks the device.
    let two = "
(defblock accel (offers sha-accel (reachable-at-privilege machine)))
(defplatform soc.first (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses accel)))
(defplatform soc.second (offers (core-count 1 tick) (reachable-at-privilege machine)))
(defservice hash (requires (needs sha-accel)))
(defsystem first (requires (uses hash)) (platform (uses soc.first)))
(defsystem second (requires (uses hash)) (platform (uses soc.second)))";
    let (verdict, said) = checked(two);
    assert_eq!(verdict, Verdict::MissingFact, "{said:?}");
    let missing: Vec<&String> = said
        .iter()
        .filter(|d| d.starts_with("missing-fact"))
        .collect();
    assert_eq!(
        missing.len(),
        1,
        "one refusal, the second system's: {said:?}"
    );
    // Each with the block is admissible.
    let both = two.replace(
        "(defplatform soc.second (offers (core-count 1 tick) (reachable-at-privilege machine)))",
        "(defplatform soc.second (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses accel)))",
    );
    assert_ne!(both, two);
    assert_eq!(checked(&both).0, Verdict::Ok, "{:?}", checked(&both).1);
}

#[test]
fn a_fact_of_the_vocabulary_stays_the_search_s_to_refuse() {
    // `observable-output` is the vocabulary's: the search binds it, and refuses a provider outside the system's hardware
    // with its own conflict (model §8 rule 7), which the check adopts with `M3.4.7`. Presence reads it as before.
    let of_the_vocabulary = "
(defblock uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)))
(defservice console (requires (needs observable-output)))
(defsystem s (requires (uses console)) (platform (uses soc.here)))";
    assert_eq!(
        checked(of_the_vocabulary).0,
        Verdict::Ok,
        "{:?}",
        checked(of_the_vocabulary).1
    );
}

#[test]
fn hardware_outside_a_system_s_asks_nothing_of_it() {
    // A second platform composes a block needing what only a block of that platform offers: none of `s`'s concern.
    let beside = format!(
        "{OUTSIDE}
(defblock far.need (offers (reachable-at-privilege machine)) (requires (needs far-fact)))
(defblock far.offer (offers far-fact (reachable-at-privilege machine)))
(defplatform soc.far (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses far.need far.offer)))"
    )
    .replace(
        "(reachable-at-privilege machine)))\n(defservice hash",
        "(reachable-at-privilege machine)) (requires (uses accel)))\n(defservice hash",
    );
    assert_eq!(checked(&beside).0, Verdict::Ok, "{:?}", checked(&beside).1);
}

#[test]
fn a_check_told_no_vocabulary_reads_every_description_whole() {
    // This crate lies below the vocabulary's: a caller that names none gets the reading of before, and the commands name it.
    assert_eq!(judged(OUTSIDE, false).0, Verdict::Ok);
    let facts = vocabulary().declared;
    assert!(
        facts.len() > 30 && facts.contains("observable-output") && !facts.contains("sha-accel")
    );
}

#[test]
fn a_description_of_no_system_is_read_whole() {
    let none = "
(defblock accel (offers sha-accel (reachable-at-privilege machine)))
(defservice hash (requires (needs sha-accel)))";
    assert_eq!(checked(none).0, Verdict::Ok, "{:?}", checked(none).1);
}
