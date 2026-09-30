//! §6's `production` namespace (`M2.7.3.5.3`).

mod common;

use archogen_catalog::history::History;
use archogen_catalog::record::FacetKind;
use archogen_catalog::replay::{check_history, replay};
use archogen_catalog::tree::Tree;
use archogen_catalog::{Code, Refusal};
use common::*;

/// `records` in a tree, those whose ids are in `production` moved to `catalog/production/`.
fn placed(records: &[&str], production: &[&str], extra: &[(&str, &str)]) -> Tree {
    let mut t = tree(records, extra);
    for id in production {
        let from = format!("catalog/experimental/{id}.catalog");
        let bytes = t.get(&from).unwrap_or_else(|| panic!("no {from}")).to_vec();
        t.remove(&from);
        t.insert(format!("catalog/production/{id}.catalog"), bytes);
    }
    t
}

/// `record`, id `id`, with a production review of each facet in `facets` at its bound hash beside `others`.
fn reviewed(
    record: &str,
    id: &str,
    facets: &[FacetKind],
    others: &[&str],
    extra: &[(&str, &str)],
) -> String {
    let mut all: Vec<&str> = others.to_vec();
    all.push(record);
    let t = tree(&all, extra);
    let mut out = record.to_owned();
    for facet in facets {
        out = with(
            &out,
            &review(*facet, &bound(&t, id, *facet), "production", &[]),
        );
    }
    out
}

/// The replay from nothing and the load, both.
fn load(records: &[&str], production: &[&str], extra: &[(&str, &str)]) -> Result<(), Refusal> {
    let mut h = History::default();
    add(
        &mut h,
        &n('1'),
        &[],
        placed(records, production, extra),
        None,
    );
    let replayed = replay(&h, &[], &n('1'));
    let loaded = check_history(&h, &n('1'));
    assert_eq!(
        replayed.as_ref().err().map(|r| r.code),
        loaded.as_ref().err().map(|r| r.code),
        "the gate and the load agree"
    );
    replayed
}

/// The refusal's code and field.
fn refused(result: Result<(), Refusal>) -> (Code, String) {
    let r = result.unwrap_err();
    (r.code, r.field)
}

const ALL: [FacetKind; 4] = FacetKind::ALL;

#[test]
fn a_reviewed_record_is_production_and_an_unreviewed_facet_refuses_it() {
    let full = reviewed(&bare(), "example.base", &ALL, &[], &[]);
    load(&[&full], &["example.base"], &[]).unwrap_or_else(|e| panic!("{e}"));
    for missing in ALL {
        let some: Vec<FacetKind> = ALL.into_iter().filter(|f| *f != missing).collect();
        let partial = reviewed(&bare(), "example.base", &some, &[], &[]);
        assert_eq!(
            refused(load(&[&partial], &["example.base"], &[])),
            (Code::Production, missing.as_str().to_owned()),
            "{} unreviewed",
            missing.as_str()
        );
        // In `experimental` nothing of this is asked.
        load(&[&partial], &[], &[]).unwrap_or_else(|e| panic!("{e}"));
    }
}

#[test]
fn no_fact_or_cost_is_unknown_in_production() {
    let unknown_fact = edit(
        &bare(),
        "(fact one-processor yes (locator (file \"docs/example/model.txt\")) (basis \"the model says so\"))",
        "(fact one-processor (unknown \"not known\"))",
    );
    let r = reviewed(&unknown_fact, "example.base", &ALL, &[], &[]);
    assert_eq!(
        refused(load(&[&r], &["example.base"], &[])),
        (Code::Production, "fact".to_owned())
    );
    let unknown_cost = edit(
        &bare(),
        "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
        "(timing-model (version \"0.1.0\") (sources) (measured-with) (facts) (costs (cost dispatch (target example-target) (unknown \"not measured\"))))",
    );
    let r = reviewed(&unknown_cost, "example.base", &ALL, &[], &[]);
    assert_eq!(
        refused(load(&[&r], &["example.base"], &[])),
        (Code::Production, "cost".to_owned())
    );
    let unknown_timing_fact = edit(
        &bare(),
        "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
        "(timing-model (version \"0.1.0\") (sources) (measured-with) (facts (fact a-timing-fact (unknown \"not known\"))) (costs))",
    );
    let r = reviewed(&unknown_timing_fact, "example.base", &ALL, &[], &[]);
    assert_eq!(
        refused(load(&[&r], &["example.base"], &[])),
        (Code::Production, "fact".to_owned()),
        "a timing fact"
    );
}

/// `example.base` as `id`, on the example's target, with `edit` applied.
fn beside(id: &str, from: &str, to: &str) -> String {
    edit(
        &edit(
            &bare(),
            "(catalog-record example.base",
            &format!("(catalog-record {id}"),
        ),
        from,
        to,
    )
}

#[test]
fn what_a_production_record_names_is_in_production_too() {
    let base = reviewed(&bare(), "example.base", &ALL, &[], &[]);
    for (how, from, to) in [
        ("depends", "(depends)", "(depends (example.base \"0.1\"))"),
        ("describes", "(describes)", "(describes example.base)"),
        (
            "measured-with",
            "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
            "(timing-model (version \"0.1.0\") (sources) (measured-with example.base) (facts) (costs))",
        ),
    ] {
        // Its own facts renamed, so the two never supply one name.
        let other = edit(&beside("example.user", from, to), "(fact one-processor yes", "(fact compare-level yes");
        let other = reviewed(&other, "example.user", &ALL, &[&base], &[]);
        assert_eq!(
            refused(load(&[&base, &other], &["example.user"], &[])),
            (Code::Production, how.to_owned()),
            "{how} an experimental record"
        );
        load(&[&base, &other], &["example.user", "example.base"], &[])
            .unwrap_or_else(|e| panic!("{how} a production record: {e}"));
    }
}

#[test]
fn each_catalog_needs_its_facets_present_and_not_empty() {
    let no_facts = |kind: &str| {
        edit(
            &edit(&bare(), "(catalog machine)", &format!("(catalog {kind})")),
            "(facts (fact one-processor yes (locator (file \"docs/example/model.txt\")) (basis \"the model says so\"))))",
            "(facts))",
        )
    };
    for kind in ["machine", "interfaces", "devices"] {
        let r = reviewed(&no_facts(kind), "example.base", &ALL, &[], &[]);
        assert_eq!(
            refused(load(&[&r], &["example.base"], &[])),
            (Code::Production, "behavior-model".to_owned()),
            "{kind}"
        );
    }
    // A device needs its driver.
    let device = edit(&bare(), "(catalog machine)", "(catalog devices)");
    let r = reviewed(&device, "example.base", &ALL, &[], &[]);
    assert_eq!(
        refused(load(&[&r], &["example.base"], &[])),
        (Code::Production, "implementation".to_owned())
    );
    // An algorithm needs code, a model and a cost.
    let files = package();
    let cost = "(cost dispatch (target example-target) (value 0x28) (unit ns) (scope \"one dispatch\") (holds-for (tasks 8) (sources 2)) (holds-under-preemption yes) (binary unbuilt) (evidence assumed) (basis \"a cost\"))";
    let algorithm = edit(
        &packaged("example.base", "p"),
        "(catalog machine)",
        "(catalog algorithms)",
    );
    let costed = edit(&algorithm, "(costs))", &format!("(costs {cost}))"));
    let r = reviewed(&costed, "example.base", &ALL, &[], &files);
    load(&[&r], &["example.base"], &files).unwrap_or_else(|e| panic!("a whole algorithm: {e}"));
    let r = reviewed(&algorithm, "example.base", &ALL, &[], &files);
    assert_eq!(
        refused(load(&[&r], &["example.base"], &files)),
        (Code::Production, "timing-model".to_owned())
    );
    let modelless = edit(
        &costed,
        "(facts (fact one-processor yes (locator (file \"docs/example/model.txt\")) (basis \"the model says so\"))))",
        "(facts))",
    );
    let r = reviewed(&modelless, "example.base", &ALL, &[], &files);
    assert_eq!(
        refused(load(&[&r], &["example.base"], &files)),
        (Code::Production, "behavior-model".to_owned())
    );
    let codeless = edit(&bare(), "(catalog machine)", "(catalog algorithms)");
    let r = reviewed(&codeless, "example.base", &ALL, &[], &[]);
    assert_eq!(
        refused(load(&[&r], &["example.base"], &[])),
        (Code::Production, "implementation".to_owned())
    );
}
