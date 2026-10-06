//! The typed vocabulary table (leaf `M3.1.2.2`, `docs/decisions/decision_substitutability-relation.md` §1.1).
//!
//! Two kinds of leg. The shipped table is held, entry by entry, to the executable model's hand-held `/1`
//! (`eadl_resolve::model::VOCABULARY`), which `tests/checker.rs` holds to the record's §1.1 table row by row: so the
//! file, the model and the record say one thing, and a fault in reading the file cannot hide behind the model, which
//! does not read it. Then each refusal the reader makes is a test on a vocabulary holding one fault, with an arm the
//! reader must accept beside it, so the test fails when the rule is removed and does not pass by refusing everything.

use std::path::{Path, PathBuf};

use eadl_front::SourceMap;
use eadl_model::check::shipped_registry;
use eadl_model::kind::Registry;
use eadl_model::Dimension;
use eadl_resolve::model::{self, CLAUSE_WORDS, VOCABULARY};
use eadl_resolve::vocabulary::{clause_words, Cause, Direction, Domain, Role, Rule, Vocabulary};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// The description kinds a vocabulary's names are held apart from: the two modules a description is checked against.
fn description_kinds() -> Registry {
    let files: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|relative| {
        let text = std::fs::read_to_string(repo_root().join(relative)).expect("a kind module");
        ((*relative).to_string(), text)
    })
    .collect();
    shipped_registry(&mut SourceMap::new(), &files).expect("the description kinds load")
}

fn shipped() -> Vocabulary {
    let mut sources = SourceMap::new();
    Vocabulary::shipped(&mut sources, &description_kinds()).unwrap_or_else(|refusals| {
        panic!(
            "the shipped vocabulary is refused:\n{}",
            refusals
                .iter()
                .map(|r| r.render(&sources))
                .collect::<Vec<_>>()
                .join("\n")
        )
    })
}

/// The causes of every refusal of a vocabulary holding `entries`, read against the shipped `deffact`.
fn causes(entries: &str) -> Vec<Cause> {
    let kind = std::fs::read_to_string(repo_root().join("docs/semantics/kinds/deffact.eadl"))
        .expect("the kind deffact");
    let text = format!("(eadl-version eadl/1)\n{entries}\n");
    let mut sources = SourceMap::new();
    match Vocabulary::read(
        &mut sources,
        ("deffact.eadl", &kind),
        ("v.eadl", &text),
        &description_kinds(),
    ) {
        Ok(_) => Vec::new(),
        Err(refusals) => refusals.iter().map(|r| r.cause).collect(),
    }
}

/// One entry, from its domain, direction and any further clauses.
fn fact(name: &str, domain: &str, direction: &str, more: &str) -> String {
    format!("(deffact {name} (doc \"a fact\") (domain {domain}) (role guarantee) (direction {direction}) {more})")
}

fn model_domain(d: model::Domain) -> Domain {
    let owned = |v: &[&str]| v.iter().map(|s| (*s).to_string()).collect::<Vec<_>>();
    match d {
        model::Domain::Boolean => Domain::Boolean,
        model::Domain::Count => Domain::Count,
        model::Domain::Quantity(dim) => Domain::Quantity(dim),
        model::Domain::Interval(dim) => Domain::Interval(dim),
        model::Domain::Enumeration {
            alternatives,
            ordered,
        } => Domain::Enumeration {
            alternatives: owned(alternatives),
            ordered,
        },
        model::Domain::Set(v) => Domain::Set(owned(v)),
        model::Domain::Group(v) => Domain::Group(owned(v)),
    }
}

fn model_direction(d: model::Direction) -> Direction {
    match d {
        model::Direction::AtLeast => Direction::AtLeast,
        model::Direction::AtMost => Direction::AtMost,
        model::Direction::Exact => Direction::Exact,
        model::Direction::Includes => Direction::Includes,
        model::Direction::Within => Direction::Within,
    }
}

#[test]
fn the_shipped_table_is_the_model_s_slash_one_entry_by_entry() {
    let table = shipped();
    let names: Vec<&str> = table.facts().map(|f| f.name.as_str()).collect();
    let model_names: Vec<&str> = VOCABULARY.iter().map(|e| e.name).collect();
    assert_eq!(names, model_names, "the same facts, in the record's order");
    for e in VOCABULARY {
        let f = table.fact(e.name).expect("named above");
        assert_eq!(f.domain, model_domain(e.domain), "{}: domain", e.name);
        assert_eq!(
            f.role,
            match e.role {
                model::Role::Guarantee => Role::Guarantee,
                model::Role::Statement => Role::Statement,
            },
            "{}: role",
            e.name
        );
        assert_eq!(
            f.direction,
            model_direction(e.direction),
            "{}: direction",
            e.name
        );
        assert_eq!(f.derived_from, e.derived_from, "{}: derived-from", e.name);
        assert_eq!(f.reads, e.reads, "{}: reads", e.name);
        assert_eq!(
            f.rule,
            e.rule
                .map(|model::Rule::HorizonFromModulusAndRate| Rule::HorizonFromModulusAndRate),
            "{}: rule",
            e.name
        );
        assert_eq!(f.implies, e.implies, "{}: implies", e.name);
        assert!(
            !f.doc.is_empty(),
            "{}: every entry says what it means",
            e.name
        );
    }
    assert!(table.fact("region").is_none() && table.fact("ordering").is_none());
}

#[test]
fn the_clause_words_are_the_model_s() {
    // The model holds the clause words by hand, checked against the kind files by `tests/checker.rs`; the table
    // derives them from the registry it is given. One list, read two ways (record §1, R30 1).
    let derived: Vec<String> = clause_words(&description_kinds()).into_iter().collect();
    let mut held: Vec<&str> = CLAUSE_WORDS.to_vec();
    held.sort_unstable();
    assert_eq!(derived, held);
}

#[test]
fn a_well_formed_vocabulary_of_every_domain_is_accepted() {
    // The arm every refusal below stands beside: each domain, an ordered enumeration read in its order, a group, a
    // set with an implied member and a derivation with its rule are all accepted.
    let entries = [
        fact("on", "boolean", "exact", ""),
        fact("n", "count", "at-most", ""),
        fact("q", "quantity time", "at-least", ""),
        fact("i", "interval frequency", "within", ""),
        fact(
            "level",
            "enumeration (ordered low mid high)",
            "at-least",
            "",
        ),
        fact("pick", "enumeration a b", "exact", ""),
        fact("s", "set x y", "includes", "(implies x)"),
        fact("g", "group q n", "exact", ""),
        fact("m", "count", "exact", ""),
        fact("rate", "quantity frequency", "exact", ""),
        fact("wrap", "enumeration modular saturating", "exact", ""),
        fact(
            "h",
            "quantity time",
            "at-least",
            "(derived-from m rate) (reads wrap) (rule horizon-from-modulus-and-rate)",
        ),
    ]
    .join("\n");
    assert_eq!(causes(&entries), []);
    let mut sources = SourceMap::new();
    let kind = std::fs::read_to_string(repo_root().join("docs/semantics/kinds/deffact.eadl"))
        .expect("kind");
    let table = Vocabulary::read(
        &mut sources,
        ("deffact.eadl", &kind),
        ("v.eadl", &format!("(eadl-version eadl/1)\n{entries}\n")),
        &description_kinds(),
    )
    .expect("accepted above");
    assert_eq!(table.len(), 12);
    assert_eq!(
        table.fact("level").expect("declared").order(),
        ["low", "mid", "high"]
    );
    assert!(table.fact("pick").expect("declared").order().is_empty());
    assert_eq!(
        table.fact("i").expect("declared").domain,
        Domain::Interval(Dimension::Frequency)
    );
}

#[test]
fn an_entry_is_held_to_the_frame_deffact_declares() {
    assert_eq!(
        causes("(deffact f (doc \"a fact\") (domain boolean) (direction exact))"),
        [Cause::Frame]
    );
    assert_eq!(
        causes(&fact("f", "boolean", "exact", "(rule a b)")),
        [Cause::Frame],
        "`rule` holds one symbol"
    );
}

#[test]
fn the_vocabulary_holds_entries_and_nothing_else() {
    assert_eq!(
        causes(&format!(
            "{}\n(defblock b (offers f))",
            fact("f", "boolean", "exact", "")
        )),
        [Cause::NotAnEntry]
    );
}

#[test]
fn a_name_is_declared_once() {
    let one = fact("f", "boolean", "exact", "");
    assert_eq!(causes(&format!("{one}\n{one}")), [Cause::Duplicate]);
}

#[test]
fn no_fact_is_named_like_a_clause_word() {
    // `offers` from `core.eadl`, `period` from `os-rt.eadl`; `domain` is `deffact`'s own and stays apart (R31 2).
    assert_eq!(
        causes(&fact("offers", "boolean", "exact", "")),
        [Cause::ClauseWord]
    );
    assert_eq!(
        causes(&fact("period", "quantity time", "exact", "")),
        [Cause::ClauseWord]
    );
    assert_eq!(causes(&fact("domain", "boolean", "exact", "")), []);
}

#[test]
fn a_domain_is_one_the_record_lists_written_its_way() {
    for domain in [
        "colour",
        "quantity length",
        "quantity",
        "boolean extra",
        "count 8",
        "set",
        "set a a",
        "enumeration",
        "enumeration (ordered)",
        "group",
        "(boolean)",
    ] {
        assert_eq!(
            causes(&fact("f", domain, "exact", "")),
            [Cause::Domain],
            "{domain}"
        );
    }
}

#[test]
fn a_role_is_guarantee_or_statement() {
    assert_eq!(
        causes("(deffact f (doc \"a fact\") (domain boolean) (role demand) (direction exact))"),
        [Cause::Role]
    );
    assert_eq!(
        causes("(deffact f (doc \"a fact\") (domain enumeration a b) (role statement) (direction exact))"),
        []
    );
}

#[test]
fn a_direction_is_one_of_five_and_one_its_domain_admits() {
    assert_eq!(
        causes(&fact("f", "boolean", "more", "")),
        [Cause::Direction]
    );
    for (domain, direction) in [
        ("boolean", "at-least"),
        ("count", "includes"),
        ("quantity time", "within"),
        ("interval time", "at-most"),
        ("enumeration a b", "at-least"),
        ("set a b", "within"),
        ("group g", "includes"),
    ] {
        let entries = format!(
            "{}\n{}",
            fact("f", domain, direction, ""),
            fact("g", "boolean", "exact", "")
        );
        assert_eq!(causes(&entries), [Cause::Direction], "{domain} {direction}");
    }
    assert_eq!(
        causes(&fact("f", "enumeration (ordered a b)", "at-most", "")),
        []
    );
}

#[test]
fn every_fact_a_clause_names_is_declared() {
    assert_eq!(
        causes(&fact("g", "group missing", "exact", "")),
        [Cause::UndeclaredFact]
    );
    assert_eq!(
        causes(&fact(
            "h",
            "quantity time",
            "at-least",
            "(derived-from m rate) (rule horizon-from-modulus-and-rate)"
        )),
        [Cause::UndeclaredFact, Cause::UndeclaredFact]
    );
}

#[test]
fn a_derivation_names_its_rule_and_the_rule_s_inputs() {
    let inputs = [
        fact("m", "count", "exact", ""),
        fact("rate", "quantity frequency", "exact", ""),
        fact("wrap", "enumeration modular saturating", "exact", ""),
        fact("on", "boolean", "exact", ""),
    ]
    .join("\n");
    for (more, why) in [
        ("(rule spin)", "a rule the engine does not have"),
        ("(derived-from m rate)", "a derivation without its rule"),
        (
            "(rule horizon-from-modulus-and-rate)",
            "a rule without its inputs",
        ),
        ("(reads wrap)", "optional inputs without a rule"),
        (
            "(derived-from rate m) (rule horizon-from-modulus-and-rate)",
            "the inputs out of order",
        ),
        (
            "(derived-from m rate) (reads on) (rule horizon-from-modulus-and-rate)",
            "an optional input not the wrap",
        ),
        (
            "(derived-from m m rate) (rule horizon-from-modulus-and-rate)",
            "an input twice",
        ),
    ] {
        let entries = format!("{inputs}\n{}", fact("h", "quantity time", "at-least", more));
        assert_eq!(causes(&entries), [Cause::Rule], "{why}");
    }
    let wrong_domain = format!(
        "{inputs}\n{}",
        fact(
            "h",
            "quantity frequency",
            "at-least",
            "(derived-from m rate) (rule horizon-from-modulus-and-rate)"
        )
    );
    assert_eq!(
        causes(&wrong_domain),
        [Cause::Rule],
        "the rule computes a time"
    );
}

#[test]
fn derived_from_and_reads_close_no_cycle() {
    // Two derived facts reading each other: the walk meets the first again at the second.
    let entries = [
        fact("m", "count", "exact", ""),
        fact("rate", "quantity frequency", "exact", ""),
        fact(
            "a",
            "quantity time",
            "at-least",
            "(derived-from m rate) (reads b) (rule horizon-from-modulus-and-rate)",
        ),
        fact(
            "b",
            "enumeration modular",
            "exact",
            "(derived-from a) (rule horizon-from-modulus-and-rate)",
        ),
    ]
    .join("\n");
    let found = causes(&entries);
    assert!(found.contains(&Cause::Cycle), "{found:?}");
    assert_eq!(
        found.iter().filter(|c| **c == Cause::Cycle).count(),
        1,
        "{found:?}"
    );
}

#[test]
fn implies_names_members_of_a_set() {
    assert_eq!(
        causes(&fact("f", "boolean", "exact", "(implies on)")),
        [Cause::Implies]
    );
    assert_eq!(
        causes(&fact("f", "set a b", "includes", "(implies c)")),
        [Cause::Implies]
    );
    assert_eq!(
        causes(&fact("f", "set a b", "includes", "(implies)")),
        [Cause::Implies]
    );
    assert_eq!(causes(&fact("f", "set a b", "includes", "(implies a)")), []);
}

#[test]
fn every_refusal_is_reported_not_only_the_first() {
    let entries = [
        fact("offers", "boolean", "exact", ""),
        fact("f", "colour", "exact", ""),
        fact("g", "set a", "exact", "(implies b)"),
    ]
    .join("\n");
    assert_eq!(
        causes(&entries),
        [Cause::Domain, Cause::ClauseWord, Cause::Implies]
    );
}
