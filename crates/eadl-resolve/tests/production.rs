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

use common::universe::{
    clause_texts, constraint_texts, horizon_rates, provider_bodies, samples, side_texts,
    widest_advance,
};
use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{read, Form, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_model::quantity::{unit, Quantity};
use eadl_model::Rational;
use eadl_resolve::model;
use eadl_resolve::offer::{self, State};
use eadl_resolve::refusal::Code;
use eadl_resolve::relation::{self, Verdict};
use eadl_resolve::requirement::{self, Requirement};
use eadl_resolve::value::Value;
use eadl_resolve::vocabulary::{Direction, Vocabulary};

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

fn direction_agrees(m: model::Direction, p: Direction) -> bool {
    matches!(
        (m, p),
        (model::Direction::AtLeast, Direction::AtLeast)
            | (model::Direction::AtMost, Direction::AtMost)
            | (model::Direction::Exact, Direction::Exact)
            | (model::Direction::Includes, Direction::Includes)
            | (model::Direction::Within, Direction::Within)
    )
}

/// Whether the model's requirement and the production's are the same requirement.
fn requirement_agrees(m: &model::Requirement, p: &Requirement) -> bool {
    match (m, p) {
        (model::Requirement::Presence(a), Requirement::Presence { fact, .. }) => a == fact,
        (
            model::Requirement::Constraint {
                fact: a,
                direction: d,
                value: v,
            },
            Requirement::Constraint {
                fact,
                direction,
                value,
                ..
            },
        ) => a == fact && direction_agrees(*d, *direction) && agree(v, value),
        (
            model::Requirement::Group { fact: a, parts: ms },
            Requirement::Group { fact, parts, .. },
        ) => {
            a == fact
                && ms.len() == parts.len()
                && ms.iter().zip(parts).all(|(x, y)| requirement_agrees(x, y))
        }
        (model::Requirement::Statement(a, v), Requirement::Statement { fact, value, .. }) => {
            a == fact && agree(v, value)
        }
        _ => false,
    }
}

/// Where the model's reading of a list of requirements and the production's part, if they do.
fn requirements_part(
    m: &Result<Vec<model::Requirement>, model::NotJudged>,
    p: &Result<Vec<Requirement>, requirement::Refusals>,
) -> Option<String> {
    match (m, p) {
        (Ok(ms), Ok(ps)) => {
            if ms.len() == ps.len() && ms.iter().zip(ps).all(|(x, y)| requirement_agrees(x, y)) {
                None
            } else {
                Some(format!(
                    "read otherwise: the model {ms:?}, production {ps:?}"
                ))
            }
        }
        (Err(model::NotJudged::Invalid(_)), Err(r))
            if requirement::code(r) == Code::InvalidDescription =>
        {
            None
        }
        (Err(model::NotJudged::Unsupported(_)), Err(r))
            if requirement::code(r) == Code::UnsupportedProfile =>
        {
            None
        }
        (m, p) => Some(format!("the model {m:?}, production {p:?}")),
    }
}

#[test]
fn every_constraint_in_the_universe_is_read_as_the_model_reads_it() {
    let v = vocabulary();
    let texts = constraint_texts();
    let mut wrong = Vec::new();
    for text in &texts {
        let clause = forms(&format!("(requires {text})"));
        let item = &clause[0].items()[1];
        let m = model::read_constraint(item).map(|r| vec![r]);
        let p = requirement::read_constraint(item, &v).map(|r| vec![r]);
        if let Some(why) = requirements_part(&m, &p) {
            wrong.push(format!("{text}\n    {why}"));
        }
    }
    println!("read {} constraints the same way as the model", texts.len());
    assert!(
        wrong.is_empty(),
        "{} of {}:\n{}",
        wrong.len(),
        texts.len(),
        wrong
            .iter()
            .take(15)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn every_clause_in_the_universe_is_read_as_the_model_reads_it() {
    let v = vocabulary();
    let texts = clause_texts();
    let mut wrong = Vec::new();
    for text in &texts {
        let clause = &forms(text)[0];
        let (m, p) = (
            model::read_clause(clause),
            requirement::read_clause(clause, &v),
        );
        if let Some(why) = requirements_part(&m, &p) {
            wrong.push(format!("{text}\n    {why}"));
        }
    }
    println!("read {} clauses the same way as the model", texts.len());
    assert!(
        wrong.is_empty(),
        "{} of {}:\n{}",
        wrong.len(),
        texts.len(),
        wrong
            .iter()
            .take(15)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn every_side_in_the_universe_is_read_as_the_model_reads_it() {
    let v = vocabulary();
    let texts = side_texts();
    let mut wrong = Vec::new();
    for text in &texts {
        let decl = &forms(text)[0];
        let (m, p) = (model::read_side(decl), requirement::read_side(decl, &v));
        if let Some(why) = requirements_part(&m, &p) {
            wrong.push(format!("{text}\n    {why}"));
        }
        // The `needs` and `uses` lists a side reads: the positions presence reads (record §1, R23 1).
        let names = |lists: Vec<&Form>| lists.iter().map(|l| format!("{l:?}")).collect::<Vec<_>>();
        if names(model::side_name_lists(decl)) != names(requirement::side_name_lists(decl)) {
            wrong.push(format!(
                "{text}\n    reads other `needs` and `uses` lists than the model"
            ));
        }
    }
    println!("read {} sides the same way as the model", texts.len());
    assert!(
        wrong.is_empty(),
        "{} of {}:\n{}",
        wrong.len(),
        texts.len(),
        wrong
            .iter()
            .take(15)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

fn verdict_agrees(m: model::Verdict, p: Verdict) -> bool {
    matches!(
        (m, p),
        (model::Verdict::Satisfied, Verdict::Satisfied)
            | (model::Verdict::Refused, Verdict::Refused)
            | (model::Verdict::Absent, Verdict::Absent)
            | (model::Verdict::Unknown, Verdict::Unknown)
            | (model::Verdict::Undescribed, Verdict::Undescribed)
            | (model::Verdict::Statement, Verdict::Statement)
            | (model::Verdict::Unsupported, Verdict::Unsupported)
    )
}

/// Every requirement of the universe both readers read, by the fact it is on: constraints, groups, statements and
/// presence.
fn requirements_by_fact(
    v: &Vocabulary,
) -> BTreeMap<String, Vec<(model::Requirement, Requirement)>> {
    let mut by_fact: BTreeMap<String, Vec<(model::Requirement, Requirement)>> = BTreeMap::new();
    for text in constraint_texts() {
        let clause = forms(&format!("(requires {text})"));
        let item = &clause[0].items()[1];
        if let (Ok(m), Ok(p)) = (
            model::read_constraint(item),
            requirement::read_constraint(item, v),
        ) {
            by_fact
                .entry(p.fact().to_string())
                .or_default()
                .push((m, p));
        }
    }
    for fact in v.facts() {
        let named = &forms(&fact.name)[0];
        if let (Ok(m), Ok(Some(p))) = (model::read_needs(named), requirement::read_needs(named, v))
        {
            by_fact.entry(fact.name.clone()).or_default().push((m, p));
        }
    }
    by_fact
}

/// The facts a provider bears on: those it states, a derived fact any input of which it states, and a group whose
/// head or sub-fact it states.
fn relevant(p: &offer::Provider, v: &Vocabulary) -> BTreeSet<String> {
    let mut out: BTreeSet<String> = p.facts.keys().cloned().collect();
    for fact in v.facts() {
        let reads: Vec<&String> = fact.derived_from.iter().chain(&fact.reads).collect();
        let subs: Vec<&String> = match &fact.domain {
            eadl_resolve::vocabulary::Domain::Group(subs) => subs.iter().collect(),
            _ => Vec::new(),
        };
        if reads.iter().chain(&subs).any(|i| out.contains(*i)) {
            out.insert(fact.name.clone());
        }
    }
    out
}

#[test]
fn every_pair_in_the_universe_is_judged_as_the_model_judges_it() {
    let v = vocabulary();
    let by_fact = requirements_by_fact(&v);
    let (mut pairs, mut wrong) = (0usize, Vec::new());
    for body in provider_bodies()
        .iter()
        .chain(std::iter::once(&String::new()))
    {
        let decl = forms(&format!("(defblock p {body})"));
        let (Ok(m), Ok(p)) = (model::read_provider(&decl[0]), offer::read(&decl[0], &v)) else {
            continue;
        };
        let facts = if body.is_empty() {
            by_fact.keys().cloned().collect()
        } else {
            relevant(&p, &v)
        };
        for fact in &facts {
            for (mr, pr) in by_fact.get(fact).into_iter().flatten() {
                let (mv, pv) = (model::judge(&m, mr), relation::judge(&p, pr, &v));
                if !verdict_agrees(mv, pv) {
                    wrong.push(format!(
                        "{body} against {mr:?}: the model {mv:?}, production {pv:?}"
                    ));
                }
                pairs += 1;
            }
        }
    }
    println!("judged {pairs} (provider, requirement) pairs as the model judges them");
    assert!(pairs > 100_000, "the universe shrank to {pairs}");
    assert!(
        wrong.is_empty(),
        "{} of {pairs}:\n{}",
        wrong.len(),
        wrong
            .iter()
            .take(15)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn every_clause_is_satisfied_by_the_providers_the_model_says_satisfy_it() {
    // Rule 5 over the universe's clauses, each against providers offering each sampled value of each fact it names.
    let v = vocabulary();
    let (mut judged, mut wrong) = (0usize, Vec::new());
    for text in clause_texts() {
        let clause = &forms(&text)[0];
        let (Ok(m), Ok(p)) = (
            model::read_clause(clause),
            requirement::read_clause(clause, &v),
        ) else {
            continue;
        };
        let facts: BTreeSet<&str> = p.iter().map(Requirement::fact).collect();
        for fact in facts {
            let Some(entry) = model::VOCABULARY.iter().find(|e| e.name == fact) else {
                continue;
            };
            for value in samples(entry.domain) {
                let decl = forms(&format!("(defblock p (offers ({fact} {value})))"));
                let (Ok(mp), Ok(pp)) = (model::read_provider(&decl[0]), offer::read(&decl[0], &v))
                else {
                    continue;
                };
                let (ms, ps) = (
                    model::clause_satisfied(&mp, &m),
                    relation::clause_satisfied(&pp, &p, &v),
                );
                if ms != ps {
                    wrong.push(format!(
                        "{text} at ({fact} {value}): the model {ms}, production {ps}"
                    ));
                }
                judged += 1;
            }
        }
    }
    println!("judged {judged} (clause, provider) pairs as the model judges them");
    assert!(
        wrong.is_empty(),
        "{} of {judged}:\n{}",
        wrong.len(),
        wrong
            .iter()
            .take(15)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn the_horizon_agrees_with_a_simulation_of_the_counter_s_reads() {
    // The checker's grid and simulation (`tests/common/universe.rs`), judged by production: a horizon of `T` is met
    // exactly when two reads `T` apart are unambiguous at every phase (record §4, R14 9).
    let v = vocabulary();
    let mut cases = 0usize;
    for modulus in 1i128..=40 {
        for (rate, hz) in horizon_rates() {
            let decl = forms(&format!(
                "(defblock p (offers (counter-modulus {modulus}) (tick-rate {rate})))"
            ));
            let p = offer::read(&decl[0], &v).expect("a counter reads");
            for k in 0i128..=(8 * modulus) {
                let t = Rational::new(k, 4)
                    .expect("k/4")
                    .checked_div(hz)
                    .expect("small");
                let required = Requirement::Constraint {
                    fact: "unambiguous-horizon".into(),
                    direction: Direction::AtLeast,
                    value: Value::Quantity(Quantity::new(t, unit("s").expect("s")).expect("valid")),
                    span: decl[0].span(),
                };
                let unambiguous = widest_advance(t, hz) < modulus;
                let got = relation::judge(&p, &required, &v);
                assert_eq!(
                    got == Verdict::Satisfied,
                    unambiguous,
                    "modulus {modulus} at {rate}, {k}/4 ticks: {got:?}"
                );
                cases += 1;
            }
        }
    }
    println!("checked {cases} horizon requirements against the simulation");
}

#[test]
fn no_stronger_precondition_passes_as_a_capability() {
    // §5.2, record §3 rule 4: a set judged by inclusion is satisfied only when every required member, and `run` for a
    // power state, is offered — no order is read among levels.
    let v = vocabulary();
    let mut judged = 0usize;
    for fact in v.facts().filter(|f| f.direction == Direction::Includes) {
        let entry = model::VOCABULARY
            .iter()
            .find(|e| e.name == fact.name)
            .expect("in /1");
        for offered in samples(entry.domain) {
            let decl = forms(&format!("(defblock p (offers ({} {offered})))", fact.name));
            let p = offer::read(&decl[0], &v).expect("a set offer reads");
            for required in samples(entry.domain) {
                let item = forms(&format!("({} {required})", fact.name));
                let r =
                    requirement::read_constraint(&item[0], &v).expect("a set requirement reads");
                if relation::judge(&p, &r, &v) == Verdict::Satisfied {
                    for member in required
                        .split(' ')
                        .chain(fact.implies.iter().map(String::as_str))
                    {
                        assert!(
                            offered.split(' ').any(|m| m == member),
                            "{}: `{offered}` met `{required}` without `{member}`",
                            fact.name
                        );
                    }
                }
                judged += 1;
            }
        }
    }
    assert!(judged > 50);
}
