//! The rules of §12 and §13 that one record's text shows (`M2.7.3.5.1`).
//!
//! The facts table is typed here from `docs/decisions/catalog/decision_catalog-records-variant-inputs.md` §12, not
//! taken from the crate, so a name the crate drops is caught.

mod common;

use archogen_catalog::{read_record, Code, Refusal};
use common::*;

/// §12's facts table: each fact with its facet, `true` for the behavioral model, and whether it is about code.
const TABLE: [(&str, bool, bool); 31] = [
    ("timer-event-driven", true, true),
    ("compare-rounds-up", true, true),
    ("due-check-matches-compare", true, true),
    ("no-early-release", true, true),
    ("raised-only-when-due", true, true),
    ("only-timer-releases-timer-tasks", true, true),
    ("preemptive-everywhere", true, true),
    ("sections-mask-every-interrupt", true, true),
    ("interrupts-do-not-nest", true, true),
    ("services-preempt-every-task", true, true),
    ("pending-taken-and-transitions-unmasked", true, true),
    ("one-claim-per-trap", true, true),
    ("starts-by-transition", true, true),
    ("pending-taken-after-unmask", true, true),
    ("no-empty-claim", true, true),
    ("one-processor", true, false),
    ("compare-level", true, false),
    ("external-before-timer", true, false),
    ("one-external-controller", true, false),
    ("runtime-discipline.example.base", true, true),
    ("no-application-code.uart", true, true),
    ("acknowledge-at-entry.uart", true, true),
    ("defers-nothing.uart", true, true),
    ("one-request-per-arrival.uart", true, true),
    ("external.uart", true, false),
    ("reprograms-only-in-service", true, true),
    ("no-suspension-primitive", true, true),
    ("no-scheduler-lock-primitive", true, true),
    ("releases-never-latched", true, true),
    ("primitives-out-of-line", true, true),
    ("eager-switching", false, true),
];

/// §13's facts about the port's code.
const PORT: [&str; 12] = [
    "eager-switching",
    "interrupts-do-not-nest",
    "services-preempt-every-task",
    "pending-taken-and-transitions-unmasked",
    "pending-taken-after-unmask",
    "no-empty-claim",
    "one-claim-per-trap",
    "starts-by-transition",
    "preemptive-everywhere",
    "sections-mask-every-interrupt",
    "releases-never-latched",
    "primitives-out-of-line",
];

const BEHAVIOR_FACTS: &str = "(facts (fact one-processor yes (locator (file \"docs/example/model.txt\")) (basis \"the model says so\"))))";
const TIMING_NONE: &str =
    "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))";

/// `example.base` with `fact` as its behavioral model's one fact.
fn in_behavior(fact: &str) -> String {
    edit(&bare(), BEHAVIOR_FACTS, &format!("(facts {fact}))"))
}

/// `example.base` with a timing model holding `facts` and `costs`, and no behavioral fact.
fn in_timing(facts: &str, costs: &str) -> String {
    edit(
        &edit(&bare(), BEHAVIOR_FACTS, "(facts))"),
        TIMING_NONE,
        &format!("(timing-model (version \"0.1.0\") (sources) (measured-with) (facts {facts}) (costs {costs}))"),
    )
}

fn read(text: &str) -> Result<(), Refusal> {
    read_record("catalog/experimental/example.base.catalog", text.as_bytes()).map(|_| ())
}

fn unknown(name: &str) -> String {
    format!("(fact {name} (unknown \"not known\"))")
}

fn known(name: &str, locator: &str) -> String {
    format!("(fact {name} yes (locator {locator}) (basis \"see it\"))")
}

const CODE: &str = "(code example.base \"docs/example/model.txt\")";
const FILE: &str = "(file \"docs/example/model.txt\")";

#[test]
fn every_tabled_fact_is_read_in_its_own_facet_and_refused_in_the_other() {
    for (name, behavior, _) in TABLE {
        let (right, wrong) = if behavior {
            (in_behavior(&unknown(name)), in_timing(&unknown(name), ""))
        } else {
            (in_timing(&unknown(name), ""), in_behavior(&unknown(name)))
        };
        read(&right).unwrap_or_else(|e| panic!("`{name}` in its facet: {e}"));
        let r = read(&wrong).unwrap_err();
        assert_eq!(r.code, Code::Field, "`{name}` in the other facet: {r}");
        assert!(r.message.contains("facts table"), "`{name}`: {r}");
    }
    // A fact the table does not name may sit in either.
    read(&in_timing(&unknown("a-fact-of-its-own"), "")).unwrap();
    read(&in_behavior(&unknown("a-fact-of-its-own"))).unwrap();
}

#[test]
fn a_known_code_fact_takes_a_code_locator() {
    for (name, behavior, code) in TABLE {
        if PORT.contains(&name) {
            continue;
        }
        let place = |fact: String| {
            if behavior {
                in_behavior(&fact)
            } else {
                in_timing(&fact, "")
            }
        };
        read(&place(known(name, CODE)))
            .unwrap_or_else(|e| panic!("`{name}` with a code locator: {e}"));
        let with_file = read(&place(known(name, FILE)));
        if code {
            let r = with_file.unwrap_err();
            assert_eq!(r.code, Code::Locator, "`{name}`: {r}");
        } else {
            with_file.unwrap_or_else(|e| panic!("`{name}` is about hardware: {e}"));
        }
    }
}

#[test]
fn a_fact_about_the_ports_code_is_unknown_in_slash_1() {
    for name in PORT {
        let behavior = name != "eager-switching";
        let place = |fact: String| {
            if behavior {
                in_behavior(&fact)
            } else {
                in_timing(&fact, "")
            }
        };
        read(&place(unknown(name))).unwrap_or_else(|e| panic!("`{name}` unknown: {e}"));
        for value in ["yes", "no"] {
            let fact = format!("(fact {name} {value} (locator {CODE}) (basis \"see it\"))");
            let r = read(&place(fact)).unwrap_err();
            assert_eq!(r.code, Code::Field, "`{name}` {value}: {r}");
            assert!(r.message.contains("port's code"), "`{name}`: {r}");
        }
    }
    // It points at the fact.
    let r = read(&in_behavior(&known("interrupts-do-not-nest", CODE))).unwrap_err();
    assert_eq!(r.field, "behavior-model fact[interrupts-do-not-nest]");
    assert_eq!(r.at.map(|a| a.line), Some(14), "{r}");
}

#[test]
fn a_record_states_runtime_discipline_about_itself() {
    read(&in_behavior(&known(
        "runtime-discipline.example.base",
        CODE,
    )))
    .unwrap();
    let r = read(&in_behavior(&known(
        "runtime-discipline.example.other",
        CODE,
    )))
    .unwrap_err();
    assert_eq!(r.code, Code::Field, "{r}");
    assert!(r.message.contains("about itself"), "{r}");
}

#[test]
fn no_primitive_is_named_completion() {
    let cost =
        |name: &str| format!("(cost {name} (target example-target) (unknown \"not measured\"))");
    read(&in_timing("", &cost("api.mask"))).unwrap_or_else(|e| panic!("{e}"));
    read(&in_timing("", &cost("masked.completion"))).unwrap_or_else(|e| panic!("{e}"));
    let r = read(&in_timing("", &cost("api.completion"))).unwrap_err();
    assert_eq!(
        (r.code, r.field.as_str()),
        (Code::Field, "timing-model cost[api.completion]"),
        "{r}"
    );
    assert!(r.at.is_some(), "{r}");
}
