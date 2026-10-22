//! No suite system requires one function's facts that its hardware offers only across providers (leaf `M3.3.6.2`, model
//! §8 rule 13, `SR-H14`): so the rule refuses no frozen verdict when `M3.4` wires the joint rules into `archogen check`.
//!
//! ⛔ **A census over the hardware and the facts it leaves unoffered, not over assignments** (`M3.3.6`'s review, D7): a
//! census of complete assignments passes on a requirement no block meets, which a later adapter or record meets, and
//! proves nothing there — `privilege_census.rs`'s lesson. So each requester's function is read against every block and
//! platform of its system's hardware: whole at one of them; offered in part by hardware and in part by nothing, listed
//! below with its only route; or offered only across providers, the breach this census exists to catch.

#[path = "../../eadl-front/tests/common/reference_table.rs"]
#[allow(dead_code)]
mod reference_table;
#[path = "../../eadl-front/tests/common/suite.rs"]
#[allow(dead_code)]
mod suite;

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{read, Form, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_resolve::joint::{requesters, System};
use eadl_resolve::offer::{self, Provider};
use eadl_resolve::relation::{outcome, Outcome};
use eadl_resolve::requirement::Requirement;
use eadl_resolve::vocabulary::{Function, Vocabulary};

/// Each requester whose function its hardware offers only in part, `(description, requester, function, facts no block
/// or platform of its system states)`, with the only routes rule 13 leaves it (`M3.3.6`, measured `2026-10-22`): the fact
/// stated on the block that offers the rest — a description gaining a fact, `SR-H20`'s migration, `M3.4`'s — or a
/// provider re-offering the whole counter, `SR-H10`'s epoch extender, `M3.2`'s. A counter offering its rate bare states
/// its horizon as unknown (record §3 rule 1), so it is whole at its own block, and not listed: four suite cases are
/// that, which the design's approximate census, `target/m336/functions_census.py`, counted among nine.
const PARTIAL: &[(&str, &str, &str, &[&str])] = &[
    (
        "docs/semantics/cases/missing-relevant-fact.eadl",
        "time.monotonic",
        "counter",
        &["wrap-behavior"],
    ),
    (
        "docs/semantics/cases/unsupported-precedence-over-missing-fact.eadl",
        "time.monotonic",
        "counter",
        &["wrap-behavior"],
    ),
    (
        "examples/alternative-timer/system.eadl",
        "time.monotonic",
        "counter",
        &["unambiguous-horizon"],
    ),
    (
        "examples/high-interference/system.eadl",
        "time.monotonic",
        "counter",
        &["unambiguous-horizon"],
    ),
    (
        "examples/periodic-three/system.eadl",
        "time.monotonic",
        "counter",
        &["unambiguous-horizon"],
    ),
];

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn vocabulary() -> Vocabulary {
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root().join(r)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the kinds load");
    Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
}

/// Every fact a requirement constrains, a group's sub-facts with its head.
fn facts(requirement: &Requirement) -> Vec<String> {
    match requirement {
        Requirement::Group { fact, parts, .. } => std::iter::once(fact.clone())
            .chain(parts.iter().flat_map(facts))
            .collect(),
        other => vec![other.fact().to_string()],
    }
}

/// Whether `provider` states `fact`: offered, with a value or without, or derived from what it offers.
fn states(provider: &Provider, fact: &str, v: &Vocabulary) -> bool {
    matches!(
        outcome(provider, fact, v),
        Ok(Outcome::Valued(_) | Outcome::Unknown | Outcome::Derived { .. })
    )
}

/// What a census finds in one description: requesters offered only across providers, and those offered in part.
#[derive(Default)]
struct Found {
    split: Vec<String>,
    partial: Vec<(String, String, Vec<String>)>,
    held: usize,
}

fn census(text: &str, v: &Vocabulary) -> Found {
    let mut found = Found::default();
    let mut sources = SourceMap::new();
    let Ok(id) = sources.add("census.eadl", text) else {
        return found;
    };
    let (doc, diags) = read(&sources, id);
    if diags.has_errors() {
        return found; // unreadable: refused before any plan, so nothing is bound
    }
    let forms: Vec<Form> = doc.forms;
    let is_provider = |f: &Form| matches!(f.head(), Some("defblock" | "defplatform"));
    let all_requesters: Vec<_> = forms
        .iter()
        .filter(|f| !is_provider(f) && f.items().get(1).and_then(Form::as_symbol).is_some())
        .filter_map(|f| requesters(f, v).ok())
        .flatten()
        .collect();
    for decl in forms.iter().filter(|f| f.head() == Some("defsystem")) {
        let system = System::read(decl, &forms);
        let hardware: Vec<Provider> = system
            .hardware
            .iter()
            .filter_map(|name| {
                forms.iter().find(|f| {
                    is_provider(f) && f.items().get(1).and_then(Form::as_symbol) == Some(name)
                })
            })
            .filter_map(|f| offer::read(f, v).ok())
            .collect();
        if hardware.is_empty() {
            continue;
        }
        found.held += 1;
        for r in all_requesters.iter().filter(|r| system.serves(&r.name)) {
            let mut by_function: BTreeMap<&str, BTreeSet<String>> = BTreeMap::new();
            for fact in r.requirements.iter().flat_map(facts) {
                if let Some(Function::Of(name)) = v.fact(&fact).and_then(|f| f.function.as_ref()) {
                    by_function.entry(name.as_str()).or_default().insert(fact);
                }
            }
            for (function, wanted) in by_function {
                if hardware
                    .iter()
                    .any(|p| wanted.iter().all(|f| states(p, f, v)))
                {
                    continue; // whole at one provider
                }
                let unoffered: Vec<String> = wanted
                    .iter()
                    .filter(|f| !hardware.iter().any(|p| states(p, f, v)))
                    .cloned()
                    .collect();
                if unoffered.is_empty() {
                    found
                        .split
                        .push(format!("{}'s `{function}`: {wanted:?}", r.name));
                } else if unoffered.len() < wanted.len() {
                    found
                        .partial
                        .push((r.name.clone(), function.to_string(), unoffered));
                }
            }
        }
    }
    found
}

#[test]
fn no_suite_system_s_function_is_offered_only_across_providers() {
    let v = vocabulary();
    let population = suite::population(&root()).unwrap_or_else(|p| panic!("{p:#?}"));
    let (mut split, mut partial, mut held) = (Vec::new(), Vec::new(), 0);
    for (path, text) in &population {
        let found = census(text, &v);
        held += found.held;
        split.extend(found.split.into_iter().map(|s| format!("{path}: {s}")));
        partial.extend(
            found
                .partial
                .into_iter()
                .map(|(r, f, facts)| (path.clone(), r, f, facts)),
        );
    }
    assert!(
        held > 0,
        "the census held no system with hardware: it proves nothing"
    );
    assert!(
        split.is_empty(),
        "{} function(s) offered only across providers:\n{}",
        split.len(),
        split.join("\n")
    );
    let expected: Vec<(String, String, String, Vec<String>)> = PARTIAL
        .iter()
        .map(|(p, r, f, facts)| {
            (
                (*p).to_string(),
                (*r).to_string(),
                (*f).to_string(),
                facts.iter().map(|s| (*s).to_string()).collect(),
            )
        })
        .collect();
    partial.sort();
    assert_eq!(
        partial, expected,
        "the requesters whose function hardware offers only in part"
    );
}

#[test]
fn the_census_finds_a_function_split_across_two_blocks() {
    // The arm: one counter's width on one block and its rate on another, no block offering both.
    let v = vocabulary();
    let text = "(eadl-version eadl/1)
(defblock timer.a (offers (counter-width 32 bit) (reachable-at-privilege machine)))
(defblock timer.b (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a timer.b)))
(defservice time.monotonic (requires (needs counter-width tick-rate)))
(defsystem s (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.monotonic))
  (platform (uses soc)))";
    let found = census(text, &v);
    assert_eq!(found.held, 1);
    assert_eq!(
        found.split,
        ["time.monotonic's `counter`: {\"counter-width\", \"tick-rate\"}"]
    );
    assert!(found.partial.is_empty());
}
