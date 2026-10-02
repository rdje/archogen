//! The port's statement (§14.4, `M2.12.4.3`): its facts, costs and convention, the refusals one record's text
//! shows, and those a selection shows.
//!
//! §14.4's table is typed here from `docs/specs/catalog/decision_catalog-records-port.md`, not taken from the crate,
//! so a name, an obligatory mark or a read condition the crate drops is caught.

mod common;

use archogen_catalog::hash::Catalog;
use archogen_catalog::selection::Selection;
use archogen_catalog::{read_record, Code, Refusal};
use common::*;

/// §14.4's table: each fact, whether it is a code fact, whether it is obligatory, and the facts its read condition
/// needs `yes` (`&` all of them, `|` one of them).
const TABLE: [(&str, bool, bool, &str); 23] = [
    ("detects-non-job-calls", true, true, ""),
    ("services-preempt-completion-interval", true, false, ""),
    (
        "completion-decision-remade",
        true,
        false,
        "services-preempt-completion-interval",
    ),
    ("guard-check-contexts", true, true, ""),
    ("generated-guard-check-contexts", false, true, ""),
    ("guarded-stacks", true, true, ""),
    ("decision-placement", true, true, ""),
    ("api-entry-by-trap", true, false, ""),
    (
        "api-trap-preemptible-before-decode",
        true,
        false,
        "api-entry-by-trap",
    ),
    ("primitives-preemptible", true, false, ""),
    (
        "unmask-preemptible-before-check",
        true,
        false,
        "primitives-preemptible",
    ),
    ("fault-window", true, true, ""),
    ("window-trap-preempts-outside", true, false, ""),
    (
        "window-trap-preempts-inside",
        true,
        false,
        "primitives-preemptible",
    ),
    (
        "window-release-abandons",
        true,
        false,
        "|window-trap-preempts-outside window-trap-preempts-inside",
    ),
    (
        "window-primitive-consistent",
        true,
        false,
        "&window-release-abandons window-trap-preempts-inside",
    ),
    ("abandoned-primitive-completion", true, true, ""),
    ("fault-path-entries", true, true, ""),
    ("checks-trap", true, false, ""),
    ("traps-discriminated", true, true, ""),
    ("kept-record-readout", true, true, ""),
    ("panic-strategy-abort", false, true, ""),
    ("vector-direct", true, true, ""),
];

const ONE_PROCESSOR: &str =
    "(fact one-processor yes (locator (file \"docs/example/model.txt\")) (basis \"the model says so\"))";
const CODE: &str = "(locator (code example.base \"crates/p/src/lib.rs\"))";
const FILE: &str = "(locator (file \"docs/example/model.txt\"))";

fn unknown(name: &str) -> String {
    format!("(fact {name} (unknown \"not known\"))")
}

fn known(name: &str, value: &str, locator: &str) -> String {
    format!("(fact {name} {value} {locator} (basis \"see it\"))")
}

/// The locator a fact of the table takes.
fn locator(name: &str) -> &'static str {
    if TABLE.iter().any(|(n, code, ..)| *n == name && !code) {
        FILE
    } else {
        CODE
    }
}

/// The facts a fact's read condition needs `yes`, stated `yes`.
fn condition(name: &str) -> Vec<String> {
    let needs = TABLE.iter().find(|(n, ..)| *n == name).map_or("", |t| t.3);
    let names = needs.trim_start_matches(['&', '|']);
    let mut out = Vec::new();
    for need in names
        .split_whitespace()
        .take(if needs.starts_with('|') { 1 } else { 2 })
    {
        out.extend(condition(need));
        out.push(known(need, "yes", locator(need)));
    }
    out
}

/// `example.base`, its behavioral model stating `facts` beside `one-processor`.
fn stating(facts: &[String]) -> String {
    edit(
        &bare(),
        ONE_PROCESSOR,
        &format!("{ONE_PROCESSOR} {}", facts.join(" ")),
    )
}

fn read(text: &str) -> Result<(), Refusal> {
    let id = text["(catalog-record ".len()..]
        .split_whitespace()
        .next()
        .unwrap();
    read_record(
        &format!("catalog/experimental/{id}.catalog"),
        text.as_bytes(),
    )
    .map(|_| ())
}

#[track_caller]
fn refused(result: Result<(), Refusal>, code: Code, says: &str) -> Refusal {
    match result {
        Ok(()) => panic!("expected {code} about `{says}`, and it was admitted"),
        Err(r) => {
            assert_eq!(r.code, code, "wrong code for `{says}`: {r}");
            assert!(r.message.contains(says), "not about `{says}`: {r}");
            r
        }
    }
}

// --- one record ---------------------------------------------------------------------------------------------

#[test]
fn every_fact_of_the_table_is_behavioral_and_refused_in_the_timing_model() {
    for (name, ..) in TABLE {
        let mut facts = condition(name);
        facts.push(unknown(name));
        read(&stating(&facts)).unwrap_or_else(|e| panic!("`{name}`: {e}"));
        let timing = edit(
            &stating(&condition(name)),
            "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
            &format!(
                "(timing-model (version \"0.1.0\") (sources) (measured-with) (facts {}) (costs))",
                unknown(name)
            ),
        );
        refused(read(&timing), Code::Field, "facts table gives");
    }
}

#[test]
fn a_code_fact_takes_code_locators_and_the_others_a_file_locator() {
    let ledger = "(locator (ledger rust-toolchain \"1.95.0, the pin\"))";
    for (name, code, ..) in TABLE {
        let with = |value: &str, locator: &str| {
            let mut facts = condition(name);
            facts.push(known(name, value, locator));
            stating(&facts)
        };
        if code {
            read(&with("yes", CODE)).unwrap_or_else(|e| panic!("`{name}`: {e}"));
            read(&with(
                "yes",
                &format!("{CODE} (locator (code example.base \"crates/p/src/main.rs\"))"),
            ))
            .unwrap_or_else(|e| panic!("`{name}` with two: {e}"));
            refused(
                read(&with("yes", FILE)),
                Code::Locator,
                "`code` locators and no other",
            );
        } else {
            read(&with("yes", FILE)).unwrap_or_else(|e| panic!("`{name}`: {e}"));
            refused(
                read(&with("yes", CODE)),
                Code::Locator,
                "takes a `file` locator and no other",
            );
            refused(
                read(&with("yes", ledger)),
                Code::Locator,
                "takes a `file` locator and no other",
            );
            refused(
                read(&with("yes", &format!("{FILE} {CODE}"))),
                Code::Shape,
                "not a code fact",
            );
        }
    }
}

#[test]
fn an_obligatory_facts_known_no_is_refused() {
    for (name, _, obligatory, _) in TABLE {
        let mut facts = condition(name);
        facts.push(known(name, "no", locator(name)));
        let result = read(&stating(&facts));
        if obligatory {
            refused(result, Code::Field, "is obligatory");
        } else {
            result.unwrap_or_else(|e| panic!("`{name}` `no`: {e}"));
        }
    }
}

#[test]
fn a_fact_is_stated_only_where_its_read_condition_holds() {
    for (name, _, _, needs) in TABLE {
        if needs.is_empty() {
            continue;
        }
        for value in [
            unknown(name),
            known(name, "yes", CODE),
            known(name, "no", CODE),
        ] {
            refused(
                read(&stating(&[value])),
                Code::Field,
                "read condition does not hold",
            );
        }
        // A condition's fact stated `no` or `unknown` holds it no better than its absence.
        let first = needs
            .trim_start_matches(['&', '|'])
            .split_whitespace()
            .next()
            .unwrap();
        let mut facts: Vec<String> = condition(name)
            .into_iter()
            .filter(|f| !f.starts_with(&format!("(fact {first} ")))
            .collect();
        facts.push(known(first, "no", CODE));
        facts.push(unknown(name));
        if !needs.starts_with('|') {
            refused(
                read(&stating(&facts)),
                Code::Field,
                "read condition does not hold",
            );
        }
    }
    // `or`: either fact suffices, and neither does not.
    let outside = known("window-trap-preempts-outside", "yes", CODE);
    let inside = [
        known("primitives-preemptible", "yes", CODE),
        known("window-trap-preempts-inside", "yes", CODE),
    ];
    let abandons = unknown("window-release-abandons");
    read(&stating(&[outside.clone(), abandons.clone()])).unwrap();
    read(&stating(&[
        inside[0].clone(),
        inside[1].clone(),
        abandons.clone(),
    ]))
    .unwrap();
    refused(
        read(&stating(&[
            known("window-trap-preempts-outside", "no", CODE),
            abandons.clone(),
        ])),
        Code::Field,
        "read condition does not hold",
    );
    // `and`: both facts, not one.
    let consistent = unknown("window-primitive-consistent");
    refused(
        read(&stating(&[
            outside.clone(),
            known("window-release-abandons", "yes", CODE),
            consistent.clone(),
        ])),
        Code::Field,
        "read condition does not hold",
    );
    read(&stating(&[
        inside[0].clone(),
        inside[1].clone(),
        known("window-release-abandons", "yes", CODE),
        consistent,
    ]))
    .unwrap();
}

#[test]
fn an_api_trap_preemptible_before_decode_needs_preemptible_primitives() {
    let trap = [
        known("api-entry-by-trap", "yes", CODE),
        known("api-trap-preemptible-before-decode", "yes", CODE),
    ];
    let with = |primitives: String| stating(&[trap[0].clone(), trap[1].clone(), primitives]);
    refused(
        read(&with(known("primitives-preemptible", "no", CODE))),
        Code::Field,
        "rule 2 makes a primitive's trap the primitive",
    );
    read(&with(unknown("primitives-preemptible"))).unwrap();
    read(&with(known("primitives-preemptible", "yes", CODE))).unwrap();
}

#[test]
fn a_self_named_fact_is_its_own_records() {
    let guard = |id: &str| known(&format!("guard-check-contexts.{id}"), "yes", CODE);
    read(&stating(&[guard("example.base")])).unwrap();
    refused(
        read(&stating(&[guard("example.other")])),
        Code::Field,
        "only about itself",
    );
    let stated = |id: &str| known(&format!("convention-stated.{id}"), "yes", FILE);
    refused(
        read(&stating(&[stated("example.base")])),
        Code::Field,
        "only a check-passing convention record",
    );
    refused(
        read(&stating(&[stated("example.other")])),
        Code::Field,
        "only about itself",
    );
    // A record supplying `switch` states `guard-check-contexts`, not its `.<id>`.
    let switching = edit(
        &stating(&[guard("example.base")]),
        "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
        "(timing-model (version \"0.1.0\") (sources) (measured-with) (facts) (costs (cost switch (target example-target) (unknown \"later\"))))",
    );
    refused(read(&switching), Code::Field, "a record supplying `switch`");
}

#[test]
fn a_convention_record_has_its_form() {
    let c = convention();
    read(&c).unwrap_or_else(|e| panic!("{e}"));
    let fact = format!(
        "(fact convention-stated.{CONVENTION} yes {FILE} (basis \"the convention, copied\"))"
    );
    let r = refused(
        read(&edit(&c, "(catalog interfaces)", "(catalog algorithms)")),
        Code::Field,
        "`interfaces` catalog",
    );
    assert_eq!(r.field, "catalog");
    refused(
        read(&edit(
            &c,
            "(implementation (version \"0.1.0\") (none \"a machine has no code\"))",
            "(implementation (version \"0.1.0\") (sources \"crates/p\"))",
        )),
        Code::Field,
        "implementation `none`",
    );
    let says = "states exactly one fact";
    refused(
        read(&edit(&c, &fact, &fact.replace(" yes ", " no "))),
        Code::Field,
        says,
    );
    refused(
        read(&edit(
            &c,
            &fact,
            &format!("(fact convention-stated.{CONVENTION} (unknown \"later\"))"),
        )),
        Code::Field,
        says,
    );
    refused(read(&edit(&c, &fact, "")), Code::Field, says);
    refused(
        read(&edit(&c, &fact, &format!("{fact} {ONE_PROCESSOR}"))),
        Code::Field,
        says,
    );
    refused(
        read(&edit(
            &c,
            &format!("(behavior-model (version \"0.1.0\") (sources \"docs/example/model.txt\") (describes)\n    (facts {fact}))"),
            "(behavior-model (version \"0.1.0\") (none \"no model\"))",
        )),
        Code::Field,
        says,
    );
    // Its one fact's locator is one `file` locator.
    refused(
        read(&edit(&c, &fact, &fact.replace(FILE, CODE))),
        Code::Locator,
        "takes a `file` locator and no other",
    );
    refused(
        read(&edit(
            &c,
            &fact,
            &fact.replace(FILE, &format!("{FILE} {CODE}")),
        )),
        Code::Shape,
        "not a code fact",
    );
}

// --- the catalog under a selection --------------------------------------------------------------------------

/// `example.base` as `id`, supplying `switch` on the example's target.
fn switching(id: &str) -> String {
    let record = edit(
        &bare(),
        "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
        "(timing-model (version \"0.1.0\") (sources) (measured-with) (facts) (costs (cost switch (target example-target) (unknown \"not measured\"))))",
    );
    edit(
        &record,
        "(catalog-record example.base",
        &format!("(catalog-record {id}"),
    )
}

fn port() -> String {
    ported(&switching("example.port"), "example-target")
}

/// The catalog of `records` read, hashed and checked under every selection.
fn load(records: &[&str]) -> Result<(), Refusal> {
    let catalog = Catalog::read(tree(records, &[]))?;
    catalog.hashes()?;
    catalog.check_selections()
}

/// `id` beside the port: `example.base` with no fact the port's group holds, depending on `depends`.
fn beside(id: &str, depends: &str) -> String {
    let record = edit(&copy(id), "(depends)", &format!("(depends {depends})"));
    edit(&record, ONE_PROCESSOR, "")
}

#[test]
fn the_port_with_its_statement_loads() {
    load(&[&port(), &convention()]).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn the_port_depends_on_exactly_one_convention() {
    let none = edit(&port(), &format!("({CONVENTION} \"0.1\")"), "");
    let r = refused(
        load(&[&none, &convention()]),
        Code::Conflict,
        "depends on 0 check-passing conventions",
    );
    assert!(r.path.ends_with("example.port.catalog"), "{r}");
    assert_eq!(r.field, "depends");
    let two = edit(
        &port(),
        &format!("({CONVENTION} \"0.1\")"),
        &format!("({CONVENTION} \"0.1\") (convention.check-passing.second \"0.1\")"),
    );
    refused(
        load(&[&two, &convention(), &second()]),
        Code::Conflict,
        "depends on 2 check-passing conventions",
    );
}

/// A second convention record, `convention.check-passing.second`.
fn second() -> String {
    convention().replace(CONVENTION, "convention.check-passing.second")
}

#[test]
fn every_record_in_the_selection_depends_on_the_ports_convention() {
    let same = beside("example.user", &format!("({CONVENTION} \"0.1\")"));
    load(&[&port(), &convention(), &same]).unwrap_or_else(|e| panic!("{e}"));
    let other = beside("example.user", "(convention.check-passing.second \"0.1\")");
    let other_here = edit(&other, "(targets twin-target)", "(targets example-target)");
    let r = refused(
        load(&[&port(), &convention(), &second(), &other_here]),
        Code::Conflict,
        "one convention per selection",
    );
    assert!(r.path.ends_with("example.user.catalog"), "{r}");
    // On another target, no port is selected with it.
    load(&[&port(), &convention(), &second(), &other]).unwrap_or_else(|e| panic!("{e}"));
    // With no `switch` in a selection, nothing is checked.
    let two = beside(
        "example.user",
        &format!("({CONVENTION} \"0.1\") (convention.check-passing.second \"0.1\")"),
    );
    load(&[&convention(), &second(), &two]).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn a_convention_admits_its_dependers_profiles_and_targets() {
    let user = beside("example.user", &format!("({CONVENTION} \"0.1\")"));
    load(&[&convention(), &user]).unwrap_or_else(|e| panic!("{e}"));
    let named = edit(&convention(), "(targets any)", "(targets example-target)");
    let r = refused(
        load(&[&named, &user]),
        Code::Dependency,
        "does not admit this record's profiles and targets",
    );
    assert!(r.path.ends_with("example.user.catalog"), "{r}");
    load(&[
        &named,
        &edit(&user, "(targets twin-target)", "(targets example-target)"),
    ])
    .unwrap_or_else(|e| panic!("{e}"));
    refused(
        load(&[
            &named,
            &edit(&user, "(targets twin-target)", "(targets any)"),
        ]),
        Code::Dependency,
        "does not admit",
    );
    // The profiles' half cannot fail while the engine supports one profile: every record names it.
}

#[test]
fn the_port_supplies_every_fact_its_statement_owes_and_both_costs() {
    for name in STATEMENT {
        let without = edit(
            &port(),
            &format!("(fact {name} (unknown \"not stated yet\")) "),
            "",
        );
        let r = refused(
            load(&[&without, &convention()]),
            Code::Field,
            &format!("so it states `{name}`"),
        );
        assert!(r.path.ends_with("example.port.catalog"), "{r}");
    }
    for cost in ["fatal-path.entry", "fatal-path.trap"] {
        let without = edit(
            &port(),
            &format!(" (cost {cost} (target example-target) (unknown \"not measured\"))"),
            "",
        );
        refused(
            load(&[&without, &convention()]),
            Code::Field,
            &format!("so it states `{cost}`"),
        );
    }
    // A condition that holds owes the facts it reads. The port has code here, for the `yes` to locate.
    let coded = edit(
        &ported(&switching("example.base"), "example-target"),
        "(implementation (version \"0.1.0\") (none \"a machine has no code\"))",
        "(implementation (version \"0.1.0\") (sources \"crates/p\"))",
    );
    let preemptible = edit(
        &coded,
        "(fact primitives-preemptible (unknown \"not stated yet\"))",
        &format!("(fact primitives-preemptible yes {CODE} (basis \"see it\"))"),
    );
    let with_code = |record: &str| {
        let catalog = Catalog::read(tree(&[record, &convention()], &package()))?;
        catalog.hashes()?;
        catalog.check_selections()
    };
    with_code(&coded).unwrap_or_else(|e| panic!("{e}"));
    let r = refused(
        with_code(&preemptible),
        Code::Field,
        "so it states `unmask-preemptible-before-check`",
    );
    assert!(r.path.ends_with("example.base.catalog"), "{r}");
    let owed = edit(
        &preemptible,
        "(fact primitives-preemptible yes",
        "(fact unmask-preemptible-before-check (unknown \"later\")) (fact window-trap-preempts-inside (unknown \"later\")) (fact primitives-preemptible yes",
    );
    with_code(&owed).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn the_statement_is_in_the_switch_group() {
    // Another record stating a fact of the table, or a fatal-path cost, where the port supplies `switch`.
    let fact = edit(
        &beside("example.other", &format!("({CONVENTION} \"0.1\")")),
        "(targets twin-target)",
        "(targets example-target)",
    );
    let fact = edit(
        &fact,
        "(describes)\n    (facts",
        "(describes)\n    (facts (fact checks-trap (unknown \"later\"))",
    );
    let port_without = edit(
        &port(),
        "(fact checks-trap (unknown \"not stated yet\")) ",
        "",
    );
    let r = refused(
        load(&[&port_without, &convention(), &fact]),
        Code::Field,
        "`switch` group",
    );
    assert!(r.path.ends_with("example.other.catalog"), "{r}");
    // And it is read only from the port.
    let catalog = Catalog::read(tree(
        &[
            &ported(&switching("example.port"), "example-target"),
            &convention(),
        ],
        &[],
    ))
    .unwrap();
    let on = Selection {
        profile: "rt-static-up-v1",
        target: Some("example-target"),
    };
    let found = catalog
        .lookup(
            on,
            archogen_catalog::record::FacetKind::BehaviorModel,
            "checks-trap",
        )
        .unwrap()
        .unwrap();
    assert_eq!(found.id, "example.port");
    let found = catalog
        .lookup(
            on,
            archogen_catalog::record::FacetKind::TimingModel,
            "fatal-path.trap",
        )
        .unwrap()
        .unwrap();
    assert_eq!(found.id, "example.port");
}

// --- the arms the mutation matrix asked for ---------------------------------------------------------------

#[test]
fn a_records_guard_check_contexts_is_a_code_fact() {
    let fact = |locators: &str| {
        format!("(fact guard-check-contexts.example.base yes {locators} (basis \"see it\"))")
    };
    read(&stating(&[fact(CODE)])).unwrap();
    read(&stating(&[fact(&format!(
        "{CODE} (locator (code example.base \"crates/p/src/main.rs\"))"
    ))]))
    .unwrap();
    refused(
        read(&stating(&[fact(FILE)])),
        Code::Locator,
        "`code` locators and no other",
    );
}

#[test]
fn a_self_named_fact_names_a_record() {
    for name in ["guard-check-contexts.", "convention-stated."] {
        refused(
            read(&stating(&[unknown(name)])),
            Code::Field,
            "only about itself",
        );
    }
}

#[test]
fn the_fatal_path_costs_are_in_the_switch_group() {
    let cost = " (cost fatal-path.entry (target example-target) (unknown \"not measured\"))";
    let port_without = edit(&port(), cost, "");
    let other = edit(
        &beside("example.other", &format!("({CONVENTION} \"0.1\")")),
        "(targets twin-target)",
        "(targets example-target)",
    );
    let other = edit(
        &other,
        "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
        &format!(
            "(timing-model (version \"0.1.0\") (sources) (measured-with) (facts) (costs{cost}))"
        ),
    );
    let r = refused(
        load(&[&port_without, &convention(), &other]),
        Code::Field,
        "`switch` group",
    );
    assert!(r.path.ends_with("example.other.catalog"), "{r}");
}

#[test]
fn only_a_convention_counts_among_the_ports_dependencies() {
    let base = edit(
        &copy("example.lib"),
        "(targets twin-target)",
        "(targets any)",
    );
    let base = edit(&base, ONE_PROCESSOR, "");
    let port = edit(
        &port(),
        &format!("({CONVENTION} \"0.1\")"),
        &format!("({CONVENTION} \"0.1\") (example.lib \"0.1\")"),
    );
    load(&[&port, &convention(), &base]).unwrap_or_else(|e| panic!("{e}"));
}
