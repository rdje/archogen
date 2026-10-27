//! The engine's adapter catalog (leaf `M3.2.2`, `docs/specs/adapters/decision_engine-adapters.md` §1, §2).
//!
//! Two kinds of leg, as `tests/vocabulary.rs` has. The shipped catalog is held, kind by kind, to the configuration
//! checker's hand-held table (`archogen_plan_check::adapters`), written from the record and not from this crate: so the
//! file, the table and the record say one thing (§1.2). Then each refusal the reader makes is a test on a catalog
//! holding one fault, beside an arm the reader must accept, so the test fails when the rule is removed and does not
//! pass by refusing everything.

use std::path::{Path, PathBuf};

use archogen_plan_check::adapters::{self as table, Beneath, Offer as TableOffer};
use eadl_front::SourceMap;
use eadl_model::check::shipped_registry;
use eadl_resolve::adapter::{Catalog, Cause, Offered};
use eadl_resolve::vocabulary::Vocabulary;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
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
            std::fs::read_to_string(repo_root().join(r)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the kinds load");
    Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
}

fn shipped() -> Catalog {
    let mut sources = SourceMap::new();
    match Catalog::shipped(&mut sources, &vocabulary()) {
        Ok(catalog) => catalog,
        Err(refusals) => panic!(
            "the shipped catalog is refused:\n{}",
            refusals
                .iter()
                .map(|r| r.render(&sources))
                .collect::<Vec<_>>()
                .join("\n")
        ),
    }
}

/// `text` read as a catalog with the shipped kind: the causes it is refused for, none when it reads.
fn causes(text: &str) -> Vec<Cause> {
    let kind = std::fs::read_to_string(repo_root().join("docs/semantics/kinds/defadapter.eadl"))
        .expect("the kind");
    match Catalog::read(
        &mut SourceMap::new(),
        ("defadapter.eadl", &kind),
        ("catalog.eadl", text),
        &vocabulary(),
    ) {
        Ok(_) => Vec::new(),
        Err(refusals) => refusals.iter().map(|r| r.cause).collect(),
    }
}

/// An exclusive kind that reads, with `edit` applied to its text: `(from, to)` replaced once.
fn exclusive(edit: (&str, &str)) -> String {
    let base = r#"(eadl-version eadl/1)
(defadapter probe
  (doc "a probe")
  (beneath (needs relative-delay counter-modulus tick-rate observation-coherent))
  (offers
    (absolute-deadline true)
    (supported-horizon (from unambiguous-horizon))
    (delivery-bound (obliged))
    (channels 1 tick)
    (available-in-state (from available-in-state)))
  (level machine)
  (obligations (obligation delivery (doc "delivered within its bound")))
  (costs (interrupt expiry (doc "one interrupt per expiry"))))
"#;
    assert!(base.contains(edit.0), "the edit `{}` applies", edit.0);
    base.replacen(edit.0, edit.1, 1)
}

/// A shared kind that reads, with `edit` applied.
fn shared(edit: (&str, &str)) -> String {
    let base = r#"(eadl-version eadl/1)
(defadapter probe
  (doc "a probe")
  (beneath (needs relative-delay counter-modulus tick-rate observation-coherent) (wrap-behavior modular))
  (offers
    (counter-modulus (pow2 64))
    (counter-width 64 bit)
    (tick-rate (from tick-rate))
    (observation-coherent true))
  (level machine))
"#;
    assert!(base.contains(edit.0), "the edit `{}` applies", edit.0);
    base.replacen(edit.0, edit.1, 1)
}

#[test]
fn the_shipped_catalog_reads_as_the_record_writes_it() {
    let catalog = shipped();
    let names: Vec<&str> = catalog.kinds.iter().map(|k| k.name.as_str()).collect();
    assert_eq!(names, ["deadline-over-delay", "epoch-extender"]);
    let deadline = catalog.kind("deadline-over-delay").expect("the kind");
    assert!(deadline.exclusive());
    assert_eq!(
        deadline.adds(),
        [
            "absolute-deadline",
            "supported-horizon",
            "delivery-bound",
            "channels"
        ]
    );
    assert_eq!(
        deadline.beneath_names,
        [
            "relative-delay",
            "counter-modulus",
            "tick-rate",
            "observation-coherent"
        ]
    );
    assert_eq!(
        deadline.offer("delivery-bound").map(|o| &o.offered),
        Some(&Offered::Obliged)
    );
    assert_eq!(deadline.level, "machine");
    let extender = catalog.kind("epoch-extender").expect("the kind");
    assert!(!extender.exclusive());
    assert_eq!(extender.adds(), ["counter-width"]);
    assert_eq!(
        extender.beneath_constraints,
        [("wrap-behavior".to_string(), "modular".to_string())]
    );
    assert_eq!(
        extender
            .offer("counter-modulus")
            .and_then(|o| o.value_text()),
        Some("(pow2 64)".to_string())
    );
    assert!(catalog.kind("multiplexer").is_none());
}

#[test]
fn the_shipped_catalog_is_the_checker_s_table() {
    // Record §1.2: the generator's reading of the file and the checker's own table, written from the record apart, are
    // one catalog, kind by kind and item by item.
    let catalog = shipped();
    assert_eq!(table::VERSION, eadl_resolve::adapter::VERSION);
    let theirs: Vec<&str> = table::ADAPTERS.iter().map(|k| k.name).collect();
    let ours: Vec<&str> = catalog.kinds.iter().map(|k| k.name.as_str()).collect();
    assert_eq!(ours, theirs, "the same kinds, in the same order");
    for (kind, held) in catalog.kinds.iter().zip(table::ADAPTERS) {
        let name = &kind.name;
        let beneath: Vec<Beneath> = held.beneath.to_vec();
        let mut expected = Vec::new();
        for fact in &kind.beneath_names {
            match kind
                .beneath_constraints
                .iter()
                .find(|(constrained, _)| constrained == fact)
            {
                Some((fact, value)) => expected.push(format!("({fact} {value})")),
                None => expected.push(fact.clone()),
            }
        }
        let theirs_beneath: Vec<String> = beneath
            .iter()
            .map(|b| match b {
                Beneath::Needs(fact) => (*fact).to_string(),
                Beneath::Constraint { fact, value } => format!("({fact} {value})"),
            })
            .collect();
        assert_eq!(expected, theirs_beneath, "`{name}`'s beneath");
        let ours_offers: Vec<String> = kind
            .offers
            .iter()
            .map(|offer| match &offer.offered {
                Offered::Value(_) => format!(
                    "value {} {}",
                    offer.fact,
                    offer.value_text().unwrap_or_default()
                ),
                Offered::From(from) => format!("from {} {from}", offer.fact),
                Offered::Obliged => format!("obliged {}", offer.fact),
            })
            .collect();
        let theirs_offers: Vec<String> = held
            .offers
            .iter()
            .map(|offer| match offer {
                TableOffer::Value { fact, value } => format!("value {fact} {value}"),
                TableOffer::From { fact, from } => format!("from {fact} {from}"),
                TableOffer::Obliged { fact } => format!("obliged {fact}"),
            })
            .collect();
        assert_eq!(ours_offers, theirs_offers, "`{name}`'s offers");
        assert_eq!(kind.level, held.level, "`{name}`'s level");
        let obligations: Vec<&str> = kind.obligations.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(obligations, held.obligations, "`{name}`'s obligations");
        let costs: Vec<&str> = kind.costs.iter().map(|o| o.name.as_str()).collect();
        assert_eq!(costs, held.costs, "`{name}`'s costs");
        assert_eq!(kind.mediates, held.mediates, "`{name}`'s mediation");
        assert_eq!(kind.exclusive(), held.exclusive(), "`{name}`'s shape");
        assert_eq!(kind.adds(), held.adds(), "`{name}`'s adds");
        let names: Vec<&str> = kind.beneath_names.iter().map(String::as_str).collect();
        assert_eq!(names, held.beneath_names(), "`{name}`'s beneath names");
    }
}

#[test]
fn every_refusal_holds_a_fault_and_accepts_its_arm() {
    // The arms read.
    assert_eq!(causes(&exclusive(("a probe", "a probe"))), []);
    assert_eq!(causes(&shared(("a probe", "a probe"))), []);
    for (case, text, cause) in [
        (
            "a declaration that is no entry",
            exclusive((
                "(defadapter probe",
                "(defblock stray (offers x))\n(defadapter probe",
            )),
            Cause::NotAnEntry,
        ),
        (
            "a frame without its level",
            exclusive(("  (level machine)\n", "")),
            Cause::Frame,
        ),
        (
            "a kind named twice",
            format!(
                "{}{}",
                exclusive(("a probe", "a probe")),
                &shared(("a probe", "a probe"))[22..]
            ),
            Cause::Duplicate,
        ),
        (
            "an offer of no fact",
            exclusive((
                "(absolute-deadline true)",
                "(absolute-deadline true) (no-such-fact true)",
            )),
            Cause::UnknownFact,
        ),
        (
            "a fact offered twice",
            exclusive(("(channels 1 tick)", "(channels 1 tick) (channels 1 tick)")),
            Cause::OfferedTwice,
        ),
        (
            "a pass-through across domains",
            exclusive(("(from unambiguous-horizon)", "(from tick-rate)")),
            Cause::FromAcrossDomains,
        ),
        (
            "an obliged fact that is no bound",
            exclusive(("(absolute-deadline true)", "(absolute-deadline (obliged))")),
            Cause::ObligedNotABound,
        ),
        (
            "an obliged fact of a shared kind",
            shared((
                "(observation-coherent true))",
                "(observation-coherent true) (delivery-bound (obliged)))",
            )),
            Cause::ObligedByShared,
        ),
        (
            "a qualifier offered as a value",
            exclusive((
                "(available-in-state (from available-in-state))",
                "(reachable-at-privilege machine)",
            )),
            Cause::QualifierNotPassedThrough,
        ),
        (
            "an exclusive kind of two channels",
            exclusive(("(channels 1 tick)", "(channels 2 tick)")),
            Cause::ExclusiveChannels,
        ),
        (
            "an exclusive kind taking no channel",
            exclusive((
                "(needs relative-delay counter-modulus",
                "(needs counter-modulus",
            )),
            Cause::ExclusiveTakesNoChannel,
        ),
        (
            "a beneath the side reader refuses",
            exclusive(("(needs relative-delay", "(needs (relative-delay)")),
            Cause::Beneath,
        ),
        (
            "a literal offer the offer reader refuses",
            shared(("(observation-coherent true)", "(observation-coherent 3)")),
            Cause::Offer,
        ),
        (
            "a level outside its domain",
            exclusive(("(level machine)", "(level nowhere)")),
            Cause::Level,
        ),
        (
            "a mediation neither true nor false",
            exclusive(("(level machine)", "(level machine)\n  (mediates perhaps)")),
            Cause::Mediates,
        ),
        (
            "an obligation without its doc",
            exclusive((
                "(obligation delivery (doc \"delivered within its bound\"))",
                "(obligation delivery)",
            )),
            Cause::NotAnObligation,
        ),
    ] {
        let found = causes(&text);
        assert!(found.contains(&cause), "{case}: {found:?}\n{text}");
    }
}

#[test]
fn presence_reads_the_catalog_s_names_as_the_generator_does() {
    // Record §1.2: presence's reading of the file, names alone (`eadl_model::presence::adapter_paths`), is a third,
    // held to the generator's kind by kind — what each adds and what its `beneath` names.
    let catalog = shipped();
    let paths = eadl_model::presence::adapter_paths();
    let ours: Vec<(String, Vec<String>, Vec<String>)> = catalog
        .kinds
        .iter()
        .map(|kind| {
            (
                kind.name.clone(),
                kind.adds().into_iter().map(str::to_string).collect(),
                kind.beneath_names.clone(),
            )
        })
        .collect();
    let presence: Vec<(String, Vec<String>, Vec<String>)> = paths
        .into_iter()
        .map(|path| (path.kind, path.adds, path.beneath))
        .collect();
    assert_eq!(ours, presence);
}
