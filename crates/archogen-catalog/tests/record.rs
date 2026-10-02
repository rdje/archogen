//! Reading one record: every rule of the record's §1 and §2 that needs nothing but the file (`M2.7.3.1`).
//!
//! One valid record, and one mutation of it per rule. Each mutation must be refused with the rule's code and its
//! message, so a check that is removed, or that starts refusing for another reason, turns its test red.

use archogen_catalog::record::{
    classify, read_record, Binary, CatalogPath, Content, Evidence, FacetKind, FactValue, Locator,
    Namespace, Targets, Unit, Verdict,
};
use archogen_catalog::{Code, Record, Refusal};

const PATH: &str = "catalog/experimental/rt.scheduler.catalog";

fn hash(c: char) -> String {
    format!("sha256:{}", c.to_string().repeat(64))
}

/// A record that meets every rule of §1 and §2.
fn valid() -> String {
    format!(
        r#"; a comment is allowed
(catalog-record rt.scheduler
  (version "0.1.0")
  (catalog algorithms)
  (source (origin "this repository, leaf M2.7.4") (license "MIT OR Apache-2.0"))
  (maintainer M2)
  (depends (rt.port "0.1"))
  (supersedes)
  (profiles rt-static-up-v1)
  (targets riscv-virt-up)
  (preconditions "one processor")
  (guarantees "tasks run in priority order")
  (implementation (version "0.1.0") (sources "crates/rt-core"))
  (behavior-model (version "0.2.0") (sources "docs/models/sched.txt") (describes)
    (facts
      (fact no-suspension-primitive yes (locator (code rt.scheduler "crates/rt-core/src/scheduler.rs")) (basis "no primitive suspends a task"))
      (fact interrupts-do-not-nest (unknown "the port's code is assembly"))))
  (timing-model (version "0.1.0") (sources "docs/models/dispatch.txt") (measured-with) (facts)
    (costs
      (cost switch (target riscv-virt-up) (unknown "not measured yet"))
      (cost dispatch (target riscv-virt-up) (value 120) (unit ns) (scope "one dispatch")
        (holds-for (tasks 8) (sources 2)) (holds-under-preemption yes) (binary unbuilt)
        (evidence observed-maximum (observed 80) (safety-factor 3 2))
        (locator (file "docs/models/dispatch.txt")) (basis "measured on the spike"))
      (cost wake (target riscv-virt-up) (value 5) (unit us) (scope "idle to first instruction")
        (holds-for (tasks 8) (sources 2)) (holds-under-preemption yes) (binary "{image}")
        (evidence assumed) (basis "a guess, named as one"))))
  (review (facet contract) (hash "{h1}") (verdict production) (by independent-context "round 1")
    (date "2026-09-30") (basis "checked the guarantees against the code"))
  (review (facet timing-model) (hash "{h2}") (verdict rejected) (by director "the director")
    (date "2026-02-28") (basis "the dispatch figure is not reproducible")))
"#,
        image = hash('b'),
        h1 = hash('c'),
        h2 = hash('d'),
    )
}

fn read(text: &str) -> Result<Record, Refusal> {
    read_record(PATH, text.as_bytes())
}

/// The valid record with `from` replaced by `to`, which must occur exactly once.
fn mutated(from: &str, to: &str) -> String {
    let text = valid();
    assert_eq!(
        text.matches(from).count(),
        1,
        "the fixture must hold `{from}` once"
    );
    text.replacen(from, to, 1)
}

/// The mutation must be refused with `code`, and a message holding `says`.
#[track_caller]
fn refused(text: &str, code: Code, says: &str) -> Refusal {
    match read(text) {
        Ok(_) => panic!("expected {code} about `{says}`, and the record was read"),
        Err(r) => {
            assert_eq!(r.code, code, "wrong code for `{says}`: {r}");
            assert!(r.message.contains(says), "not about `{says}`: {r}");
            r
        }
    }
}

#[test]
fn the_valid_record_reads_with_every_field() {
    let r = read(&valid()).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(
        (r.id.as_str(), r.namespace),
        ("rt.scheduler", Namespace::Experimental)
    );
    assert_eq!(r.contract.version.to_string(), "0.1.0");
    assert_eq!(r.contract.maintainer, "M2");
    assert_eq!(r.contract.depends[0].id, "rt.port");
    assert_eq!(
        r.contract.targets,
        Targets::Named(vec!["riscv-virt-up".into()])
    );
    assert_eq!(r.behavior_model.version.to_string(), "0.2.0");
    let Content::Present(model) = &r.behavior_model.content else {
        panic!("model")
    };
    assert!(matches!(
        &model.facts[0].value,
        FactValue::Known {
            holds: true,
            locators,
            ..
        } if matches!(locators.as_slice(), [Locator::Code { .. }])
    ));
    assert!(matches!(&model.facts[1].value, FactValue::Unknown(_)));
    let Content::Present(timing) = &r.timing_model.content else {
        panic!("timing")
    };
    assert!(timing.costs[0].value.is_err());
    let dispatch = timing.costs[1].value.as_ref().unwrap();
    assert_eq!(
        (dispatch.value, dispatch.unit, dispatch.tasks),
        (120, Unit::Ns, 8)
    );
    assert!(matches!(
        dispatch.evidence,
        Evidence::ObservedMaximum(Some((80, _)))
    ));
    assert_eq!(dispatch.binary, Binary::Unbuilt);
    let wake = timing.costs[2].value.as_ref().unwrap();
    assert_eq!(
        (wake.evidence.clone(), wake.locator.clone()),
        (Evidence::Assumed, None)
    );
    assert_eq!(r.reviews.len(), 2);
    assert_eq!(
        (r.reviews[0].facet, r.reviews[0].verdict),
        (FacetKind::Contract, Verdict::Production)
    );
    assert_eq!(r.reviews[1].date, (2026, 2, 28));
}

#[test]
fn a_path_under_catalog_is_the_lock_a_record_or_refused() {
    assert_eq!(classify("catalog/catalog.lock"), Ok(CatalogPath::Lock));
    assert_eq!(classify("crates/x.rs"), Ok(CatalogPath::Outside));
    assert_eq!(
        classify("catalog/production/a.b.catalog"),
        Ok(CatalogPath::Record(Namespace::Production, "a.b".into()))
    );
    for bad in [
        "catalog/README.md",
        "catalog/staging/a.catalog",
        "catalog/experimental/a.catalogue",
        "catalog/experimental/sub/a.catalog",
        "catalog/experimental/.catalog",
    ] {
        assert_eq!(
            classify(bad).map_err(|r| r.code),
            Err(Code::Layout),
            "{bad}"
        );
    }
}

#[test]
fn section_1_the_file() {
    let r = refused(
        &mutated("(supersedes)", "(supersedes)\t"),
        Code::Read,
        "neither printable ASCII",
    );
    assert_eq!(r.at.map(|a| a.line), Some(8));
    refused(
        &mutated("\"one processor\"", "\"one\\nprocessor\""),
        Code::Read,
        "decoded value is not printable ASCII",
    );
    refused(
        &mutated("(maintainer M2)", "(maintainer M2"),
        Code::Read,
        "",
    );
    refused(
        &format!("{}(catalog-record x)", valid()),
        Code::Shape,
        "exactly one top-level form",
    );
    refused(
        &mutated("(catalog-record rt.scheduler", "(record rt.scheduler"),
        Code::Shape,
        "`(catalog-record …)`",
    );
    refused(
        &mutated("(value 120)", "(value 1.5)"),
        Code::Shape,
        "a decimal",
    );
    refused(
        &mutated("(catalog-record rt.scheduler", "(catalog-record rt.other"),
        Code::Id,
        "differs from the file stem",
    );
    let bad_id = read_record(
        "catalog/experimental/Rt.catalog",
        valid()
            .replace("(catalog-record rt.scheduler", "(catalog-record Rt")
            .as_bytes(),
    );
    assert_eq!(bad_id.map_err(|r| r.code), Err(Code::Id));
}

#[test]
fn section_1_fields_in_order_each_once() {
    refused(
        &mutated("  (supersedes)\n", ""),
        Code::Shape,
        "`supersedes` is missing",
    );
    refused(
        &mutated("(supersedes)", "(supersedes) (colour red)"),
        Code::Shape,
        "`colour` is not a subform",
    );
    refused(
        &mutated("(supersedes)", "(supersedes) (supersedes)"),
        Code::Shape,
        "appears twice",
    );
    refused(
        &mutated(
            "(version \"0.1.0\")\n  (catalog algorithms)",
            "(catalog algorithms)\n  (version \"0.1.0\")",
        ),
        Code::Shape,
        "out of §2's order",
    );
    refused(
        &mutated("(value 120) (unit ns)", "(unit ns) (value 120)"),
        Code::Shape,
        "out of §2's order",
    );
    refused(
        &mutated("(maintainer M2)", "(maintainer M2 M3)"),
        Code::Shape,
        "exactly one value",
    );
}

#[test]
fn section_2_the_contract() {
    refused(
        &mutated(
            "(version \"0.1.0\")\n  (catalog",
            "(version \"0.01.0\")\n  (catalog",
        ),
        Code::Version,
        "MAJOR.MINOR.PATCH",
    );
    refused(
        &mutated("(rt.port \"0.1\")", "(rt.port \"0.1.0\")"),
        Code::Version,
        "MAJOR.MINOR requirement",
    );
    refused(
        &mutated("(catalog algorithms)", "(catalog recipes)"),
        Code::Field,
        "four catalogs",
    );
    refused(
        &mutated(
            "(origin \"this repository, leaf M2.7.4\")",
            "(origin \"  \")",
        ),
        Code::Field,
        "empty where §2 requires text",
    );
    refused(
        &mutated("(maintainer M2)", "(maintainer m2)"),
        Code::Field,
        "task tree's id",
    );
    refused(
        &mutated("(rt.port \"0.1\")", "(rt.port \"0.1\") (rt.port \"0.2\")"),
        Code::Dependency,
        "listed twice",
    );
    refused(
        &mutated("(supersedes)", "(supersedes a.b a.b)"),
        Code::Dependency,
        "listed twice",
    );
    refused(
        &mutated("(supersedes)", "(supersedes A)"),
        Code::Id,
        "id grammar",
    );
    refused(
        &mutated("(profiles rt-static-up-v1)", "(profiles rt-dynamic)"),
        Code::Field,
        "not a profile the engine supports",
    );
    refused(
        &mutated("(profiles rt-static-up-v1)", "(profiles)"),
        Code::Shape,
        "at least one profile",
    );
    refused(
        &mutated(
            "(profiles rt-static-up-v1)",
            "(profiles rt-static-up-v1 rt-static-up-v1)",
        ),
        Code::Shape,
        "is repeated",
    );
    refused(
        &mutated("(targets riscv-virt-up)", "(targets any riscv-virt-up)"),
        Code::Field,
        "`any` stands alone",
    );
    refused(
        &mutated(
            "(targets riscv-virt-up)",
            "(targets riscv-virt-up riscv-virt-up)",
        ),
        Code::Shape,
        "repeated",
    );
    assert_eq!(
        read(&mutated("(targets riscv-virt-up)", "(targets any)"))
            .unwrap()
            .contract
            .targets,
        Targets::Any
    );
    refused(
        &mutated(
            "(guarantees \"tasks run in priority order\")",
            "(guarantees)",
        ),
        Code::Field,
        "no guarantees",
    );
    refused(
        &mutated("(preconditions \"one processor\")", "(preconditions \"\")"),
        Code::Field,
        "empty",
    );
    assert!(read(&mutated(
        "(preconditions \"one processor\")",
        "(preconditions)"
    ))
    .is_ok());
}

#[test]
fn section_2_facets_and_none() {
    refused(
        &mutated(
            "(sources \"crates/rt-core\"))",
            "(sources \"crates/rt-core\") (none \"no code\"))",
        ),
        Code::Shape,
        "`none` facet holds its version and nothing else",
    );
    refused(
        &mutated(" (sources \"crates/rt-core\"))", ")"),
        Code::Shape,
        "`sources` is missing",
    );
    refused(
        &mutated("(sources \"crates/rt-core\")", "(sources \"crates/../x\")"),
        Code::Source,
        "§4's normal form",
    );
    refused(
        &mutated(
            "(sources \"crates/rt-core\")",
            "(sources \"crates/rt-core\" \"crates/rt-core\")",
        ),
        Code::Source,
        "appears twice",
    );
    refused(
        &mutated("(sources \"crates/rt-core\")", "(sources)"),
        Code::Shape,
        "at least one package",
    );
    let none = mutated(
        " (sources \"crates/rt-core\"))",
        " (none \"an interface has no code of its own\"))",
    );
    assert!(matches!(
        read(&none).unwrap().implementation.content,
        Content::None(_)
    ));
    let text = valid();
    let (a, b) = (
        text.find("(behavior-model").unwrap(),
        text.find("  (timing-model").unwrap(),
    );
    let model = |inner: &str| {
        format!(
            "{}(behavior-model (version \"0.2.0\") {inner})\n{}",
            &text[..a],
            &text[b..]
        )
    };
    refused(
        &model("(describes) (none \"nothing\")"),
        Code::Shape,
        "nothing else",
    );
    assert!(read(&model(
        "(none \"a device record states its behavior elsewhere\")"
    ))
    .is_ok());
    refused(
        &mutated("(measured-with) (facts)", "(measured-with)"),
        Code::Shape,
        "`facts` is missing",
    );
}

#[test]
fn section_2_facts_and_locators() {
    refused(
        &mutated(
            "(fact interrupts-do-not-nest (unknown",
            "(fact Interrupts (unknown",
        ),
        Code::Field,
        "a malformed fact",
    );
    refused(
        &mutated(
            "(fact interrupts-do-not-nest (unknown \"the port's code is assembly\"))",
            "(fact interrupts-do-not-nest maybe)",
        ),
        Code::Field,
        "a malformed fact",
    );
    refused(
        &mutated(
            " (locator (code rt.scheduler \"crates/rt-core/src/scheduler.rs\"))",
            "",
        ),
        Code::Shape,
        "`locator` is missing",
    );
    refused(
        &mutated("(code rt.scheduler \"crates", "(url rt.scheduler \"crates"),
        Code::Locator,
        "a locator is",
    );
    refused(
        &mutated("(code rt.scheduler \"crates", "(code Rt \"crates"),
        Code::Locator,
        "names a record by its id",
    );
    refused(
        &mutated("(file \"docs/models/dispatch.txt\")", "(file \"/docs/x\")"),
        Code::Source,
        "§4's normal form",
    );
    let ledger = mutated(
        "(file \"docs/models/dispatch.txt\")",
        "(ledger rust-toolchain \"1.95.0, the pin\")",
    );
    assert!(read(&ledger).is_ok());
    refused(
        &mutated(
            "(file \"docs/models/dispatch.txt\")",
            "(ledger rust-toolchain \" \")",
        ),
        Code::Locator,
        "revision",
    );
}

#[test]
fn section_2_costs() {
    refused(
        &mutated("(value 120)", "(value -1)"),
        Code::Field,
        "a negative integer",
    );
    refused(
        &mutated(
            "(tasks 8) (sources 2)) (holds-under-preemption yes) (binary unbuilt)",
            "(tasks -8) (sources 2)) (holds-under-preemption yes) (binary unbuilt)",
        ),
        Code::Field,
        "a negative integer",
    );
    refused(&mutated("(unit ns)", "(unit s)"), Code::Field, "not a unit");
    refused(
        &mutated(
            "(unknown \"not measured yet\")",
            "(unknown \"not measured yet\") (unit ns)",
        ),
        Code::Shape,
        "and nothing else",
    );
    refused(
        &mutated(" (basis \"measured on the spike\")", ""),
        Code::Shape,
        "`basis` is missing from a known cost",
    );
    refused(
        &mutated(" (locator (file \"docs/models/dispatch.txt\"))", ""),
        Code::Locator,
        "required unless the evidence is `assumed`",
    );
    refused(
        &mutated("(safety-factor 3 2)", "(safety-factor 1 2)"),
        Code::Field,
        "does not pad",
    );
    refused(
        &mutated("(safety-factor 3 2)", "(safety-factor 3 0)"),
        Code::Field,
        "does not pad",
    );
    refused(
        &mutated("(safety-factor 3 2)", "(safety-factor 4294967296 2)"),
        Code::Field,
        "outside `u32`",
    );
    refused(
        &mutated("(value 120)", "(value 119)"),
        Code::Field,
        "the padded observation is 120",
    );
    refused(
        &mutated("(evidence assumed)", "(evidence guessed)"),
        Code::Field,
        "unknown evidence category",
    );
    refused(
        &mutated("(binary unbuilt)", "(binary independent \"no image\")"),
        Code::Field,
        "`compare-rounding` and `delivery`",
    );
    let independent = mutated("(cost dispatch (target", "(cost delivery (target").replacen(
        "(binary unbuilt)",
        "(binary independent \"on a board\")",
        1,
    );
    assert!(
        read(&independent).is_ok(),
        "`independent` on `delivery` reads; the board check is §12's"
    );
    refused(
        &mutated("(binary unbuilt)", "(binary \"sha256:abc\")"),
        Code::Field,
        "`binary` is",
    );
    refused(
        &mutated(
            "(holds-under-preemption yes) (binary unbuilt)",
            "(holds-under-preemption maybe) (binary unbuilt)",
        ),
        Code::Field,
        "`yes` or `no`",
    );
    refused(
        &mutated(
            "(cost switch (target riscv-virt-up)",
            "(cost switch (target Riscv)",
        ),
        Code::Field,
        "not a target stem",
    );
}

#[test]
fn section_2_names() {
    refused(
        &mutated(
            "(fact interrupts-do-not-nest",
            "(fact no-suspension-primitive",
        ),
        Code::Shape,
        "a fact name appears once per record",
    );
    refused(
        &mutated(
            "(measured-with) (facts)",
            "(measured-with) (facts (fact wake (unknown \"x\")))",
        ),
        Code::Shape,
        "share one name space",
    );
    refused(
        &mutated("(cost switch (target", "(cost dispatch (target"),
        Code::Shape,
        "once per target",
    );
    let other_target = mutated(
        "(cost switch (target riscv-virt-up)",
        "(cost dispatch (target other-board)",
    );
    assert!(
        read(&other_target).is_ok(),
        "a cost name repeats on another target"
    );
}

#[test]
fn section_5_reviews() {
    refused(
        &mutated("(facet contract)", "(facet manifest)"),
        Code::Review,
        "not one of the four",
    );
    let h1 = hash('c');
    refused(
        &mutated(&h1, "sha256:C0FFEE"),
        Code::Review,
        "not written as §3 requires",
    );
    refused(
        &mutated("(verdict production)", "(verdict maybe)"),
        Code::Field,
        "not a verdict",
    );
    refused(
        &mutated(
            "(verdict rejected)",
            &format!("(verdict rejected) (answers \"{}\")", hash('e')),
        ),
        Code::Review,
        "only on a `production` verdict",
    );
    assert!(read(&mutated(
        "(verdict production)",
        &format!("(verdict production) (answers \"{}\")", hash('e'))
    ))
    .is_ok());
    refused(
        &mutated("(by independent-context", "(by friend"),
        Code::Field,
        "not a role",
    );
    refused(
        &mutated("\"round 1\"", "\"\""),
        Code::Review,
        "`who` is empty",
    );
    refused(
        &mutated("\"round 1\"", "\"M2\""),
        Code::Review,
        "maintainer's tree id",
    );
    refused(
        &mutated("(date \"2026-02-28\")", "(date \"2026-02-30\")"),
        Code::Review,
        "real calendar date",
    );
    refused(
        &mutated(
            "(basis \"checked the guarantees against the code\")",
            "(basis \"\")",
        ),
        Code::Review,
        "basis is empty",
    );
    refused(
        &mutated("(verdict production) (by", "(by"),
        Code::Shape,
        "`verdict` is missing",
    );
}

#[test]
fn every_refusal_names_its_record_field_and_position() {
    let r = refused(
        &mutated("(maintainer M2)", "(maintainer m2)"),
        Code::Field,
        "task tree's id",
    );
    assert_eq!(r.path, PATH);
    assert_eq!(r.field, "maintainer");
    assert_eq!(r.at.map(|a| (a.line, a.column)), Some((6, 15)));
    assert!(r
        .to_string()
        .starts_with("catalog/experimental/rt.scheduler.catalog:6:15: catalog-field [maintainer]"));
}
