//! Every block and platform in each suite system's hardware states the level it is reachable from (leaf `M3.3.4.3`,
//! `SR-H12`): a plan binds no caller to a provider that states none, so a description whose hardware states none would
//! be refused the day `M3.4` wires the joint rules into `archogen check`.
//!
//! ⛔ **A census over the hardware, not over assignments.** Nine of the nineteen suite descriptions whose systems bind a
//! block have no complete assignment — a requirement the realization meets, which no block offers — so a census of
//! assignments would pass on them while proving nothing (`M3.3.4`'s review, D6). The population is the suite's,
//! walked by `eadl-front`'s one reader of `docs/semantics/conformance.md`, never a list of its own.

#[path = "../../eadl-front/tests/common/reference_table.rs"]
#[allow(dead_code)]
mod reference_table;
#[path = "../../eadl-front/tests/common/suite.rs"]
#[allow(dead_code)]
mod suite;

use eadl_front::{read, Form, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_model::RT_STATIC_UP_V1;
use eadl_resolve::joint::System;
use eadl_resolve::offer::{self, State};
use eadl_resolve::value::Value;
use eadl_resolve::vocabulary::Vocabulary;

/// Descriptions the census does not hold, each with why: none may be added without a leaf that owns it.
const EXCLUDED: &[(&str, &str)] = &[(
    "examples/alternative-timer/",
    "uc3's description changes only with the director's yes (`SR-H10`, findings §15); `M3.3.4.5` owns it",
)];

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

/// What one description's systems leave unstated: `(system, provider, why)` for each block or platform in a system's
/// hardware whose `reachable-at-privilege` does not hold the profile's level.
fn unstated(text: &str, v: &Vocabulary) -> Vec<(String, String, String)> {
    let mut sources = SourceMap::new();
    let Ok(id) = sources.add("census.eadl", text) else {
        return Vec::new();
    };
    let (doc, diags) = read(&sources, id);
    if diags.has_errors() {
        return Vec::new(); // unreadable: refused before any plan, so nothing is bound
    }
    let forms: Vec<Form> = doc.forms;
    let level = RT_STATIC_UP_V1.caller_level;
    let mut out = Vec::new();
    for decl in forms.iter().filter(|f| f.head() == Some("defsystem")) {
        let system = System::read(decl, &forms);
        for name in &system.hardware {
            let Some(hardware) = forms
                .iter()
                .find(|f| f.items().get(1).and_then(Form::as_symbol) == Some(name.as_str()))
            else {
                continue;
            };
            let why = match offer::read(hardware, v) {
                Err(_) => continue, // refused as it is read, before any plan binds it
                Ok(provider) => match provider.state("reachable-at-privilege") {
                    Some(State::Valued(values)) => match values.first() {
                        Some(Value::Set(levels)) if levels.contains(level) => continue,
                        _ => format!("reachable only from levels without `{level}`"),
                    },
                    None => "states no level".to_string(),
                    Some(other) => format!("states its level as {other:?}"),
                },
            };
            out.push((system.name.clone(), name.clone(), why));
        }
    }
    out
}

#[test]
fn every_suite_system_s_hardware_states_the_profile_s_level() {
    let v = vocabulary();
    let population = suite::population(&root()).unwrap_or_else(|p| panic!("{p:#?}"));
    let mut breaches = Vec::new();
    let mut held = 0;
    for (path, text) in &population {
        if EXCLUDED.iter().any(|(prefix, _)| path.starts_with(prefix)) {
            continue;
        }
        let found = unstated(text, &v);
        if text.contains("(defsystem") && text.contains("(platform") {
            held += 1;
        }
        breaches.extend(
            found
                .into_iter()
                .map(|(s, p, why)| format!("{path}: {s}'s `{p}` {why}")),
        );
    }
    assert!(
        held > 0,
        "the census held no system with a platform: it proves nothing"
    );
    assert!(
        breaches.is_empty(),
        "{} provider(s) state no level:\n{}",
        breaches.len(),
        breaches.join("\n")
    );
}

#[test]
fn every_exclusion_still_excludes_something() {
    // An exclusion whose path no longer holds a description would be a silent exemption for whatever lands there next.
    let population = suite::population(&root()).unwrap_or_else(|p| panic!("{p:#?}"));
    for (prefix, why) in EXCLUDED {
        assert!(
            population.iter().any(|(path, _)| path.starts_with(prefix)),
            "`{prefix}` excludes nothing ({why})"
        );
    }
}
