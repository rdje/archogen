//! The record's form as §14.2 amends it (`M2.12.4.1`): a code fact's several locators, the `assembly`
//! declaration, and the rules over declared assembly that need the catalog whole.
//!
//! One valid record or catalog, and one change of it per rule. Each change must be refused with the rule's code and
//! its message, so a check that is removed, or that starts refusing for another reason, turns its test red.

mod common;

use archogen_catalog::hash::{Catalog, Hashes};
use archogen_catalog::record::{Content, FacetKind, FactValue, Locator};
use archogen_catalog::{read_record, Code, Record, Refusal};
use common::*;

const PATH: &str = "catalog/experimental/example.base.catalog";

const ONE_PROCESSOR: &str = "(fact one-processor yes (locator (file \"docs/example/model.txt\")) (basis \"the model says so\"))";

/// `example.base` implementing `crates/p`, its behavioral model holding `fact` beside `one-processor`.
fn with_fact(fact: &str) -> String {
    edit(
        &packaged("example.base", "p"),
        ONE_PROCESSOR,
        &format!("{ONE_PROCESSOR}\n      {fact}"),
    )
}

/// `record` with its implementation's `sources` followed by `declaration`.
fn declaring(record: &str, declaration: &str) -> String {
    edit(
        record,
        "(sources \"crates/p\"))",
        &format!("(sources \"crates/p\") {declaration})"),
    )
}

fn read(text: &str) -> Result<Record, Refusal> {
    read_record(PATH, text.as_bytes())
}

/// The change must be refused with `code`, and a message holding `says`.
#[track_caller]
fn refused(result: Result<impl core::fmt::Debug, Refusal>, code: Code, says: &str) -> Refusal {
    match result {
        Ok(v) => panic!("expected {code} about `{says}`, and got {v:?}"),
        Err(r) => {
            assert_eq!(r.code, code, "wrong code for `{says}`: {r}");
            assert!(r.message.contains(says), "not about `{says}`: {r}");
            r
        }
    }
}

const CODE_LIB: &str = "(locator (code example.base \"crates/p/src/lib.rs\"))";
const CODE_MAIN: &str = "(locator (code example.base \"crates/p/src/main.rs\"))";

fn code_fact(locators: &str) -> String {
    format!("(fact no-suspension-primitive yes {locators} (basis \"see the code\"))")
}

// --- one record ---------------------------------------------------------------------------------------------

#[test]
fn a_code_fact_reads_with_several_code_locators() {
    let r = read(&with_fact(&code_fact(&format!("{CODE_LIB} {CODE_MAIN}"))))
        .unwrap_or_else(|e| panic!("{e}"));
    let Content::Present(model) = &r.behavior_model.content else {
        panic!("model")
    };
    let FactValue::Known { locators, .. } = &model.facts[1].value else {
        panic!("known")
    };
    assert_eq!(
        locators,
        &vec![
            Locator::Code {
                id: "example.base".into(),
                path: "crates/p/src/lib.rs".into()
            },
            Locator::Code {
                id: "example.base".into(),
                path: "crates/p/src/main.rs".into()
            },
        ]
    );
}

#[test]
fn two_identical_locators_of_one_fact_are_refused() {
    refused(
        read(&with_fact(&code_fact(&format!("{CODE_LIB} {CODE_LIB}")))),
        Code::Shape,
        "two identical locators",
    );
}

#[test]
fn several_locators_on_a_fact_that_is_not_about_code_are_refused() {
    let file = "(locator (file \"docs/example/model.txt\"))";
    let fact = format!("(fact one-external-controller yes {file} {CODE_LIB} (basis \"see it\"))");
    refused(read(&with_fact(&fact)), Code::Shape, "not a code fact");
    // A fact the table does not name is no code fact either.
    let fact = format!("(fact a-fact-of-its-own yes {CODE_LIB} {CODE_MAIN} (basis \"see it\"))");
    refused(read(&with_fact(&fact)), Code::Shape, "not a code fact");
}

#[test]
fn a_code_facts_locators_come_one_after_another() {
    let fact = format!(
        "(fact no-suspension-primitive yes {CODE_LIB} (basis \"see the code\") {CODE_MAIN})"
    );
    refused(read(&with_fact(&fact)), Code::Shape, "out of §2's order");
}

#[test]
fn a_known_fact_has_a_locator() {
    let fact = "(fact no-suspension-primitive yes (basis \"see the code\"))";
    refused(read(&with_fact(fact)), Code::Shape, "`locator` is missing");
}

#[test]
fn every_locator_of_a_code_fact_is_a_code_locator() {
    let file = "(locator (file \"docs/example/model.txt\"))";
    let r = refused(
        read(&with_fact(&code_fact(&format!("{CODE_LIB} {file}")))),
        Code::Locator,
        "`code` locators and no other",
    );
    assert_eq!(r.field, "behavior-model fact[no-suspension-primitive]");
}

#[test]
fn structure_is_refused_before_a_locators_path() {
    // Two identical locators whose path is outside §4's form: the shape is the refusal, not the path's
    // `catalog-source` (§14.2's order: structure, then locators, then sources, then fields).
    let bad = "(locator (code example.base \"crates//p\"))";
    refused(
        read(&with_fact(&code_fact(&format!("{bad} {bad}")))),
        Code::Shape,
        "two identical locators",
    );
}

#[test]
fn an_assembly_declaration_is_read() {
    let r = read(&declaring(
        &with_fact(""),
        "(assembly riscv64 \"crates/p\")",
    ))
    .unwrap_or_else(|e| panic!("{e}"));
    let Content::Present(p) = &r.implementation.content else {
        panic!("implementation")
    };
    let assembly = p.assembly.as_ref().expect("declared");
    assert_eq!(
        (assembly.architecture.as_str(), assembly.packages.as_slice()),
        ("riscv64", &["crates/p".to_owned()][..])
    );
    let r = read(&with_fact("")).unwrap_or_else(|e| panic!("{e}"));
    let Content::Present(p) = &r.implementation.content else {
        panic!("implementation")
    };
    assert_eq!(p.assembly, None);
}

#[test]
fn an_assembly_declarations_shape_is_refused() {
    let record = with_fact("");
    let twice = declaring(
        &record,
        "(assembly riscv64 \"crates/p\") (assembly riscv64 \"crates/p\")",
    );
    refused(read(&twice), Code::Shape, "appears twice");
    refused(
        read(&declaring(&record, "(assembly riscv64)")),
        Code::Shape,
        "at least one package",
    );
    refused(
        read(&declaring(&record, "(assembly \"riscv64\" \"crates/p\")")),
        Code::Shape,
        "(assembly <architecture>",
    );
    refused(
        read(&declaring(&record, "(assembly riscv64 crates)")),
        Code::Shape,
        "is a string",
    );
    let before = edit(
        &record,
        "(sources \"crates/p\"))",
        "(assembly riscv64 \"crates/p\") (sources \"crates/p\"))",
    );
    refused(read(&before), Code::Shape, "out of §2's order");
    let none = edit(
        &bare(),
        "(none \"a machine has no code\")",
        "(assembly riscv64 \"crates/p\") (none \"a machine has no code\")",
    );
    refused(read(&none), Code::Shape, "nothing else");
}

#[test]
fn an_assembly_declarations_packages_are_the_facets_sources() {
    let record = with_fact("");
    refused(
        read(&declaring(
            &record,
            "(assembly riscv64 \"crates/p\" \"crates/p\")",
        )),
        Code::Source,
        "appears twice in the declaration",
    );
    refused(
        read(&declaring(&record, "(assembly riscv64 \"crates/q\")")),
        Code::Source,
        "not an entry of the facet's `sources`",
    );
}

#[test]
fn an_unknown_architecture_is_refused_after_the_packages() {
    let record = with_fact("");
    let r = refused(
        read(&declaring(&record, "(assembly mips \"crates/p\")")),
        Code::Field,
        "not an architecture §14.3 lists",
    );
    assert_eq!(r.field, "implementation assembly");
    // A package named twice under an unknown architecture: the source is the refusal (§14.2's order).
    refused(
        read(&declaring(
            &record,
            "(assembly mips \"crates/p\" \"crates/p\")",
        )),
        Code::Source,
        "appears twice",
    );
}

// --- the catalog whole ------------------------------------------------------------------------------------

/// A target `rv` whose `RUST_TARGET` is `rust`, or none.
fn rv(rust: Option<&str>) -> Vec<(&'static str, String)> {
    let env = match rust {
        Some(r) => format!("TARGET_ID=rv\nRUST_TARGET={r}\n"),
        None => "TARGET_ID=rv\n".to_owned(),
    };
    vec![
        ("targets/rv.env", env),
        ("targets/rv.eadl", "(platform)\n".to_owned()),
    ]
}

const RISCV: Option<&str> = Some("riscv64imac-unknown-none-elf");

/// The catalog of `records` over the package `crates/p` and the target `rv`.
fn hashes(records: &[&str], rust: Option<&str>) -> Result<Hashes, Refusal> {
    let mut extra: Vec<(&str, String)> = package()
        .into_iter()
        .map(|(p, t)| (p, t.to_owned()))
        .collect();
    extra.extend(rv(rust));
    let extra: Vec<(&str, &str)> = extra.iter().map(|(p, t)| (*p, t.as_str())).collect();
    Catalog::read(tree(records, &extra))?.hashes()
}

const PORT_FACT: &str = "(fact interrupts-do-not-nest yes (locator (code example.base \"crates/p/src/lib.rs\")) (basis \"the trap entry masks\"))";

/// `example.base` on `targets`, declaring `crates/p` as `riscv64` assembly, stating `fact`.
fn port(targets: &str, declaration: &str, fact: &str) -> String {
    edit(
        &declaring(&with_fact(fact), declaration),
        "(targets example-target)",
        &format!("(targets {targets})"),
    )
}

const DECLARED: &str = "(assembly riscv64 \"crates/p\")";

#[test]
fn a_known_port_fact_located_in_declared_assembly_loads() {
    hashes(&[&port("rv", DECLARED, PORT_FACT)], RISCV).unwrap_or_else(|e| panic!("{e}"));
    // `unknown` needs no locator at all.
    let unknown = "(fact interrupts-do-not-nest (unknown \"not yet\"))";
    hashes(&[&port("rv", DECLARED, unknown)], RISCV).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn a_known_port_fact_outside_declared_assembly_is_refused() {
    let r = refused(
        hashes(&[&port("rv", "", PORT_FACT)], RISCV),
        Code::Field,
        "needs a locator into a package its record declares",
    );
    assert_eq!(r.field, "behavior-model fact[interrupts-do-not-nest]");
    // `no` is a known value too.
    let no = PORT_FACT.replace(" yes ", " no ");
    refused(
        hashes(&[&port("rv", "", &no)], RISCV),
        Code::Field,
        "needs a locator",
    );
}

#[test]
fn one_locator_into_declared_assembly_suffices() {
    // A second package, `crates/q`, undeclared: the fact's first locator is into it, its second into `crates/p`.
    let fact = "(fact interrupts-do-not-nest yes (locator (code example.base \"crates/q/src/lib.rs\")) \
                (locator (code example.base \"crates/p/src/lib.rs\")) (basis \"the trap entry masks\"))";
    let record = edit(
        &port("rv", DECLARED, fact),
        "(sources \"crates/p\")",
        "(sources \"crates/p\" \"crates/q\")",
    );
    let mut extra: Vec<(&str, String)> = package()
        .into_iter()
        .map(|(p, t)| (p, t.to_owned()))
        .collect();
    extra.extend(rv(RISCV));
    extra.push((
        "crates/q/Cargo.toml",
        "[package]\nname = \"q\"\n".to_owned(),
    ));
    extra.push(("crates/q/src/lib.rs", "//! q\n".to_owned()));
    let extra: Vec<(&str, &str)> = extra.iter().map(|(p, t)| (*p, t.as_str())).collect();
    let catalog = Catalog::read(tree(&[&record], &extra)).unwrap_or_else(|e| panic!("{e}"));
    catalog.hashes().unwrap_or_else(|e| panic!("{e}"));
    // With `crates/q` declared instead, the first locator is the one into declared assembly.
    let swapped = edit(&record, DECLARED, "(assembly riscv64 \"crates/q\")");
    let catalog = Catalog::read(tree(&[&swapped], &extra)).unwrap_or_else(|e| panic!("{e}"));
    catalog.hashes().unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn a_record_stating_a_port_fact_names_targets_on_the_dialect() {
    let r = refused(
        hashes(&[&port("any", DECLARED, PORT_FACT)], RISCV),
        Code::Field,
        "`any` is refused",
    );
    assert_eq!(r.field, "targets");
    refused(
        hashes(
            &[&port("rv", DECLARED, PORT_FACT)],
            Some("x86_64-unknown-linux-gnu"),
        ),
        Code::Field,
        "is not (§14.3)",
    );
    refused(
        hashes(&[&port("rv", DECLARED, PORT_FACT)], None),
        Code::Field,
        "`rv`'s is not",
    );
}

#[test]
fn a_record_reaching_declared_assembly_names_targets_on_the_dialect() {
    // The declaring record itself, with no port fact.
    let r = refused(
        hashes(&[&port("any", DECLARED, "")], RISCV),
        Code::Field,
        "reaches `crates/p`, which holds assembly",
    );
    assert_eq!(r.field, "targets");
    refused(
        hashes(
            &[&port("rv", DECLARED, "")],
            Some("riscv32imac-unknown-none-elf"),
        ),
        Code::Field,
        "`rv`'s is not",
    );
    // Another record whose behavioral model's sources name the package.
    let reader = edit(
        &copy("example.reader"),
        "(sources \"docs/example/model.txt\")",
        "(sources \"docs/example/model.txt\" \"crates/p\")",
    );
    let reader = edit(&reader, "(targets twin-target)", "(targets any)");
    hashes(
        &[
            &port("rv", DECLARED, ""),
            &edit(&reader, "(targets any)", "(targets rv)"),
        ],
        RISCV,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let r = refused(
        hashes(&[&port("rv", DECLARED, ""), &reader], RISCV),
        Code::Field,
        "its behavior-model reaches `crates/p`",
    );
    assert!(r.path.ends_with("example.reader.catalog"), "{r}");
    // Undeclared, the same reach binds nothing.
    hashes(&[&port("rv", "", ""), &reader], RISCV).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn each_locator_of_a_fact_is_checked_against_its_set() {
    let fact = code_fact(&format!(
        "{CODE_LIB} (locator (code example.base \"docs/example/model.txt\"))"
    ));
    refused(
        hashes(&[&port("rv", "", &fact)], RISCV),
        Code::Locator,
        "not in `example.base`'s implementation own set",
    );
}

#[test]
fn the_declaration_is_in_the_implementations_own_hash() {
    let own = |record: &str| {
        hashes(&[record], RISCV)
            .unwrap_or_else(|e| panic!("{e}"))
            .facet("example.base", FacetKind::Implementation)
            .unwrap()
            .own
    };
    assert_ne!(own(&port("rv", DECLARED, "")), own(&port("rv", "", "")));
}

#[test]
fn an_assembly_declarations_shape_is_refused_before_its_packages() {
    // A package named twice, then one that is not a string: the shape is the refusal (§14.2's order).
    refused(
        read(&declaring(
            &with_fact(""),
            "(assembly riscv64 \"crates/p\" \"crates/p\" crates)",
        )),
        Code::Shape,
        "is a string",
    );
}

/// The package `crates/q` beside `crates/p`, and the target `rv`.
fn two_packages() -> Vec<(&'static str, String)> {
    let mut extra: Vec<(&str, String)> = package()
        .into_iter()
        .map(|(p, t)| (p, t.to_owned()))
        .collect();
    extra.extend(rv(RISCV));
    extra.push((
        "crates/q/Cargo.toml",
        "[package]\nname = \"q\"\n".to_owned(),
    ));
    extra.push(("crates/q/src/lib.rs", "//! q\n".to_owned()));
    extra
}

fn hashes_over(records: &[&str], extra: &[(&'static str, String)]) -> Result<Hashes, Refusal> {
    let extra: Vec<(&str, &str)> = extra.iter().map(|(p, t)| (*p, t.as_str())).collect();
    Catalog::read(tree(records, &extra))?.hashes()
}

#[test]
fn the_locator_reaches_a_declared_package_of_the_record_it_names() {
    // `example.base` implements `crates/p` and `crates/q` and declares only `crates/p`: a locator into `crates/q`
    // is into code its record implements, not into declared assembly.
    let fact = PORT_FACT.replace("crates/p/src/lib.rs", "crates/q/src/lib.rs");
    let record = edit(
        &port("rv", DECLARED, &fact),
        "(sources \"crates/p\")",
        "(sources \"crates/p\" \"crates/q\")",
    );
    refused(
        hashes_over(&[&record], &two_packages()),
        Code::Field,
        "needs a locator",
    );
    // A record that implements `crates/p` without declaring it, beside one that declares it: its own locator
    // names itself, which declares nothing.
    let declarer = port("rv", DECLARED, "");
    let other = edit(
        &packaged("example.other", "p"),
        "(targets twin-target)",
        "(targets rv)",
    );
    hashes_over(&[&declarer, &other], &two_packages()).unwrap_or_else(|e| panic!("{e}"));
    let stated = edit(
        &other,
        ONE_PROCESSOR,
        &format!(
            "{ONE_PROCESSOR}\n      (fact interrupts-do-not-nest yes (locator (code example.other \"crates/p/src/lib.rs\")) (basis \"masks\"))"
        ),
    );
    let r = refused(
        hashes_over(&[&declarer, &stated], &two_packages()),
        Code::Field,
        "needs a locator",
    );
    assert!(r.path.ends_with("example.other.catalog"), "{r}");
}

#[test]
fn a_record_stating_a_port_fact_about_anothers_assembly_names_its_targets() {
    // `example.reader` describes the port and reaches none of its code in its own sets.
    let declarer = port("rv", DECLARED, "");
    let fact = "(fact interrupts-do-not-nest yes (locator (code example.base \"crates/p/src/lib.rs\")) (basis \"masks\"))";
    let reader = edit(
        &edit(
            &copy("example.reader"),
            "(describes)",
            "(describes example.base)",
        ),
        ONE_PROCESSOR,
        &format!("{ONE_PROCESSOR}\n      {fact}"),
    );
    hashes(
        &[
            &declarer,
            &edit(&reader, "(targets twin-target)", "(targets rv)"),
        ],
        RISCV,
    )
    .unwrap_or_else(|e| panic!("{e}"));
    let r = refused(
        hashes(
            &[
                &declarer,
                &edit(&reader, "(targets twin-target)", "(targets any)"),
            ],
            RISCV,
        ),
        Code::Field,
        "it states a known value of `interrupts-do-not-nest`",
    );
    assert!(r.path.ends_with("example.reader.catalog"), "{r}");
    refused(
        hashes(&[&declarer, &reader], RISCV),
        Code::Field,
        "`twin-target`'s is not",
    );
}

#[test]
fn eager_switching_in_the_timing_model_is_held_to_the_same_rule() {
    let record = edit(
        &edit(&with_fact(""), "(targets example-target)", "(targets rv)"),
        "(facts (fact f yes",
        "(facts (fact eager-switching yes (locator (code example.base \"crates/p/src/lib.rs\")) (basis \"switches\")) (fact f yes",
    );
    let r = refused(hashes(&[&record], RISCV), Code::Field, "needs a locator");
    assert_eq!(r.field, "timing-model fact[eager-switching]");
    hashes(&[&declaring(&record, DECLARED)], RISCV).unwrap_or_else(|e| panic!("{e}"));
}
