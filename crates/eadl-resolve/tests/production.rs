//! The production relation against its executable model, over the model checker's universe (`SR-H7`;
//! `docs/decisions/decision_substitutability-relation.md` §11).
//!
//! ⭐ **One universe, two readings.** `tests/common/universe.rs` holds the inputs `tests/checker.rs` enumerates; here
//! each is read by the production code (`eadl_resolve::offer`, `value`) and by the model (`eadl_resolve::model`),
//! and the two must agree: both refuse, or both read the same state for every fact, every spelling of a value kept,
//! in written order. The model is the reference because a design review read it beside the record for fifteen rounds;
//! a disagreement is a defect in one of them, found by a machine rather than by the next reader.
//!
//! Slices add legs as they land: `M3.1.2.3` the offer reader; `M3.1.2.4` requirements and sides; `M3.1.2.5` the
//! relation itself.

mod common;

use common::universe::provider_bodies;
use eadl_front::{read, Form, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_resolve::model;
use eadl_resolve::offer::{self, State};
use eadl_resolve::value::Value;
use eadl_resolve::vocabulary::Vocabulary;

fn forms(text: &str) -> Vec<Form> {
    let mut sources = SourceMap::new();
    let id = sources.add("t.eadl", text).expect("small");
    let (doc, diags) = read(&sources, id);
    assert!(!diags.has_errors(), "{text}: {}", diags.render(&sources));
    doc.forms
}

fn vocabulary() -> Vocabulary {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf();
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root.join(r)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the kinds load");
    Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
}

/// Whether the model's value and the production's are the same value written the same way.
fn agree(m: &model::Value, p: &Value) -> bool {
    match (m, p) {
        (model::Value::Bool(a), Value::Bool(b)) => a == b,
        (model::Value::Count(a), Value::Count(b)) => a == b,
        (model::Value::Quantity(a), Value::Quantity(b)) => a == b,
        (model::Value::Interval(a0, a1), Value::Interval(b0, b1)) => a0 == b0 && a1 == b1,
        (model::Value::Enum(a), Value::Enum(b)) => a == b,
        (model::Value::Set(a), Value::Set(b)) => a == b,
        _ => false,
    }
}

/// Where the two readings of one declaration part, if they do.
fn disagreement(decl: &str, v: &Vocabulary) -> Option<String> {
    let form = &forms(decl)[0];
    let (m, p) = (model::read_provider(form), offer::read(form, v));
    match (m, p) {
        (Err(_), Err(_)) => None,
        (Ok(_), Err(refusals)) => Some(format!(
            "the model reads it; production refuses it: {refusals:?}"
        )),
        (Err(refused), Ok(_)) => Some(format!(
            "production reads it; the model refuses it: {refused:?}"
        )),
        (Ok(m), Ok(p)) => {
            let model_facts: Vec<&String> = m.stated.keys().collect();
            let production_facts: Vec<&String> = p.facts.keys().collect();
            if model_facts != production_facts {
                return Some(format!(
                    "facts {model_facts:?} against {production_facts:?}"
                ));
            }
            for (fact, stated) in &m.stated {
                let state = p.state(fact).expect("the same facts");
                let same = match (stated, state) {
                    (model::Stated::Valued(ms), State::Valued(ps)) => {
                        ms.len() == ps.len() && ms.iter().zip(ps).all(|(a, b)| agree(a, b))
                    }
                    (model::Stated::Unvalued, State::Unvalued)
                    | (model::Stated::Absent, State::Absent)
                    | (model::Stated::Overflow, State::Overflow) => true,
                    _ => false,
                };
                if !same {
                    return Some(format!(
                        "`{fact}`: the model reads {stated:?}, production {state:?}"
                    ));
                }
            }
            None
        }
    }
}

#[test]
fn every_provider_in_the_universe_is_read_as_the_model_reads_it() {
    let v = vocabulary();
    let bodies = provider_bodies();
    let mut wrong = Vec::new();
    let mut read = 0usize;
    for body in &bodies {
        for decl in [
            format!("(defblock p {body})"),
            format!("(defplatform soc.p {body})"),
        ] {
            if let Some(why) = disagreement(&decl, &v) {
                wrong.push(format!("{decl}\n    {why}"));
            }
            read += 1;
        }
        // A service's offers and absences are refused as a provider's are (record §1, R24 4).
        let service = format!("(defservice s {body})");
        let form = &forms(&service)[0];
        if model::read_service(form).is_err() != offer::read_service(form, &v).is_err() {
            wrong.push(format!("{service}\n    the two disagree on refusing it"));
        }
        read += 1;
    }
    println!(
        "read {read} declarations of {} provider bodies the same way as the model",
        bodies.len()
    );
    assert!(
        wrong.is_empty(),
        "{} of {read} declarations read otherwise than the model:\n{}",
        wrong.len(),
        wrong
            .iter()
            .take(20)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn a_declaration_named_like_a_fact_is_refused_by_both() {
    let v = vocabulary();
    for decl in [
        "(defblock counter-width (offers uart))",
        "(defplatform tick-unit)",
        "(defservice low-power-timer (offers uart))",
    ] {
        let form = &forms(decl)[0];
        assert!(model::read_provider(form).is_err(), "{decl}");
        let refused = offer::read(form, &v).expect_err(decl);
        assert_eq!(
            refused[0].cause,
            eadl_resolve::refusal::Cause::DeclarationNamedLikeFact,
            "{decl}"
        );
    }
}
