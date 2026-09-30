//! §7's claims (`M2.7.3.6.1`).

mod common;

use std::collections::BTreeSet;

use archogen_catalog::claim::{Claim, Source, Strength, Verdict};
use archogen_catalog::history::History;
use archogen_catalog::load::{load, Loaded};
use archogen_catalog::record::FacetKind;
use archogen_catalog::status::Status;
use common::*;

const P: &str = "rt-static-up-v1";
const CONTRACT: FacetKind = FacetKind::Contract;
const IMPLEMENTATION: FacetKind = FacetKind::Implementation;

/// `example.timed` as the example writes it: it depends on `example.base`, and costs a switch on the example's
/// target.
fn timed() -> String {
    let records = block("(catalog-record example.base");
    let split = records.find("\n(catalog-record example.timed").unwrap();
    records[split + 1..].to_owned()
}

/// A one-commit history of these records, loaded.
fn loaded(records: &[&str], extra: &[(&str, &str)]) -> (History, Loaded) {
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(records, extra), None);
    let l = load(&h, &n('1')).unwrap_or_else(|e| panic!("{e}"));
    (h, l)
}

fn set(items: &[(&str, FacetKind)]) -> BTreeSet<(String, FacetKind)> {
    items.iter().map(|(id, f)| ((*id).to_owned(), *f)).collect()
}

#[test]
fn a_claims_citations_are_what_it_read_and_its_closure_what_they_rest_on() {
    let (h, l) = loaded(&[&base(), &timed()], &[]);
    let mut claim = Claim::new(
        &l,
        &h,
        None,
        P,
        Some("example-target"),
        Strength::Exploratory,
    );
    let found = claim.lookup(TIMING, "switch").unwrap().unwrap();
    assert_eq!(found.id, "example.timed");
    // The example's printed bound input names these facets, and each of theirs is followed in turn.
    assert_eq!(
        claim.closure(),
        set(&[
            ("example.timed", TIMING),
            ("example.timed", CONTRACT),
            ("example.timed", IMPLEMENTATION),
            ("example.base", IMPLEMENTATION),
            ("example.base", TIMING),
            ("example.base", CONTRACT),
        ])
    );
    assert!(claim.lookup(TIMING, "wake").unwrap().is_none());
    assert_eq!(
        claim.closure().len(),
        6,
        "a lookup that found nothing cites nothing"
    );
    claim.lookup(BEHAVIOR, "one-processor").unwrap().unwrap();
    assert!(claim
        .closure()
        .contains(&("example.base".to_owned(), BEHAVIOR)));
    let result = claim.result();
    let reads: Vec<(&str, Option<&str>)> = result
        .reads
        .iter()
        .map(|r| (r.name.as_str(), r.found.as_deref()))
        .collect();
    assert_eq!(
        reads,
        [
            ("switch", Some("example.timed")),
            ("wake", None),
            ("one-processor", Some("example.base"))
        ]
    );
    assert!(result
        .reads
        .iter()
        .all(|r| r.profile == P && r.target.as_deref() == Some("example-target")));
    let lines: Vec<(&str, FacetKind, Status)> = result
        .closure
        .iter()
        .map(|c| (c.id.as_str(), c.facet, c.status))
        .collect();
    assert!(
        lines.contains(&("example.base", BEHAVIOR, Status::Production)),
        "{lines:?}"
    );
    assert_eq!(result.closure.len(), 7);
    assert!(result
        .closure
        .iter()
        .all(|c| c.bound == l.hashes.facet(&c.id, c.facet).unwrap().bound));
    let version = |id: &str, f: FacetKind| {
        result
            .closure
            .iter()
            .find(|c| c.id == id && c.facet == f)
            .map(|c| c.version.to_string())
    };
    assert_eq!(version("example.timed", TIMING).as_deref(), Some("1.0.0"));
    assert_eq!(version("example.base", CONTRACT).as_deref(), Some("0.1.0"));
    assert_eq!(
        result.assumptions,
        ["`example.base`: a \"quoted\" precondition"]
    );
    assert_eq!(
        result.experimental,
        [
            ("example.base".to_owned(), Status::Unreviewed),
            ("example.timed".to_owned(), Status::Unreviewed)
        ]
    );
    assert_eq!(result.commit, n('1'));
    let admitted = claim.admit().unwrap();
    assert_eq!(
        (admitted.verdict, admitted.reasons.len()),
        (Verdict::Admitted, 0),
        "exploratory"
    );
}

#[test]
fn a_closure_outside_the_claims_target_is_unsupported() {
    let (h, l) = loaded(&[&base(), &timed()], &[]);
    let mut claim = Claim::new(&l, &h, None, P, Some("nowhere"), Strength::Exploratory);
    claim.lookup(TIMING, "switch").unwrap();
    let a = claim.admit().unwrap();
    assert_eq!(a.verdict, Verdict::UnsupportedProfile);
    assert!(
        a.reasons.iter().any(|r| r.contains("not a named target")),
        "{a:?}"
    );
    // A record on the claim's target that depends on one that is not.
    let twin = edit(
        &edit(
            &timed(),
            "(catalog-record example.timed",
            "(catalog-record example.twin",
        ),
        "(target example-target)",
        "(target twin-target)",
    );
    let (h, l) = loaded(&[&base(), &twin], &[]);
    let mut claim = Claim::new(&l, &h, None, P, Some("twin-target"), Strength::Production);
    claim.lookup(TIMING, "switch").unwrap().unwrap();
    let a = claim.admit().unwrap();
    assert_eq!(
        a.verdict,
        Verdict::UnsupportedProfile,
        "strongest first, even for a production claim"
    );
    assert_eq!(
        a.reasons,
        ["`example.base` does not admit the target `twin-target`"]
    );
    // A claim with no target needs every record in its closure to name `(targets any)`.
    let described = edit(
        &timed(),
        "(behavior-model (version \"1.0.0\") (sources) (describes) (facts))",
        "(behavior-model (version \"1.0.0\") (sources) (describes) (facts (fact compare-level (unknown \"not known\"))))",
    );
    let (h, l) = loaded(&[&base(), &described], &[]);
    let mut claim = Claim::new(&l, &h, None, P, None, Strength::Exploratory);
    assert_eq!(
        claim.lookup(BEHAVIOR, "compare-level").unwrap().unwrap().id,
        "example.timed"
    );
    let a = claim.admit().unwrap();
    assert_eq!(a.verdict, Verdict::UnsupportedProfile);
    assert!(
        a.reasons
            .iter()
            .any(|r| r.contains("`example.base` names targets")),
        "{a:?}"
    );
}

#[test]
fn a_production_claim_names_every_cause_it_is_not_established() {
    let (h, l) = loaded(&[&base(), &timed()], &[]);
    let mut claim = Claim::new(
        &l,
        &h,
        None,
        P,
        Some("example-target"),
        Strength::Production,
    );
    claim.lookup(TIMING, "switch").unwrap().unwrap();
    claim.input("C_i of task a", Source::Caller);
    claim.input("the plan's interrupt order", Source::Application);
    let a = claim.admit().unwrap();
    assert_eq!(a.verdict, Verdict::NotEstablished);
    for cause in [
        "`example.base` is `experimental`",
        "`example.timed` is `experimental`",
        "`C_i of task a` came from the Caller",
        "`the plan's interrupt order` came from the Application",
        "the cost `switch` is on `example-target`, which is not a board",
        "the cost `switch` is measured on an image, or none",
        "the clone has no `origin/main`",
        "premise 3 has no named commit yet",
    ] {
        assert!(
            a.reasons.iter().any(|r| r.contains(cause)),
            "`{cause}` missing from {:?}",
            a.reasons
        );
    }
    assert!(
        !a.reasons
            .iter()
            .any(|r| r.contains("implementation holds code")),
        "every implementation is `none`"
    );
    // An implementation that holds code needs an image, which no claim has before `M4`.
    let files = package();
    let coded = edit(&packaged("example.base", "p"), "(fact f yes", "(fact g yes");
    let (h, l) = loaded(&[&coded], &files);
    let mut claim = Claim::new(
        &l,
        &h,
        Some(&n('1')),
        P,
        Some("example-target"),
        Strength::Production,
    );
    claim.lookup(BEHAVIOR, "one-processor").unwrap().unwrap();
    let a = claim.admit().unwrap();
    let coded: Vec<&String> = a
        .reasons
        .iter()
        .filter(|r| r.contains("implementation holds code"))
        .collect();
    assert_eq!(coded.len(), 1, "once, for its implementation facet: {a:?}");
    assert!(coded[0].contains("`example.base`'s"), "{a:?}");
    assert!(
        !a.reasons.iter().any(|r| r.contains("origin/main")),
        "{a:?}"
    );
    assert!(
        !a.reasons.iter().any(|r| r.contains("cost")),
        "no cost read: {a:?}"
    );
}

#[test]
fn the_commit_read_holds_every_line_of_its_ancestors_and_of_origin_main() {
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&base(), &timed()], &[]), None);
    // `origin/main` has since ledgered a rejection this branch lacks.
    let t = tree(&[&base(), &timed()], &[]);
    let rejected = with(
        &base(),
        &review(TIMING, &bound(&t, "example.base", TIMING), "rejected", &[]),
    );
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&rejected, &timed()], &[]),
        None,
    );
    let l = load(&h, &n('1')).unwrap();
    let mut claim = Claim::new(
        &l,
        &h,
        Some(&n('2')),
        P,
        Some("example-target"),
        Strength::Production,
    );
    claim.lookup(TIMING, "switch").unwrap();
    let a = claim.admit().unwrap();
    let lacking: Vec<&String> = a
        .reasons
        .iter()
        .filter(|r| r.contains("the commit read lacks"))
        .collect();
    assert_eq!(lacking.len(), 1, "{a:?}");
    assert!(
        lacking[0].contains("example.base review") && lacking[0].contains(&n('2')),
        "{a:?}"
    );
    // Checked against itself, nothing is lacking.
    let mut same = Claim::new(
        &l,
        &h,
        Some(&n('1')),
        P,
        Some("example-target"),
        Strength::Production,
    );
    same.lookup(TIMING, "switch").unwrap();
    assert!(!same
        .admit()
        .unwrap()
        .reasons
        .iter()
        .any(|r| r.contains("lacks")));
}

#[test]
fn a_load_refuses_what_any_check_refuses() {
    let a = copy("example.other");
    let clash = edit(&a, "(targets twin-target)", "(targets example-target)");
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&bare(), &clash], &[]), None);
    assert_eq!(
        load(&h, &n('1')).unwrap_err().code,
        archogen_catalog::Code::Conflict
    );
}

/// `record`, id `id`, with a production review of each facet at its bound hash in `tree`.
fn reviewed_all(record: &str, id: &str, t: &archogen_catalog::tree::Tree) -> String {
    let mut out = record.to_owned();
    for facet in FacetKind::ALL {
        out = with(
            &out,
            &review(facet, &bound(t, id, facet), "production", &[]),
        );
    }
    out
}

#[test]
fn a_production_record_on_a_board_with_an_independent_cost_raises_none_of_those_causes() {
    let board = [(
        "targets/example-target.env",
        "TARGET_ID=example-target\nTARGET_KIND=board\nRUST_TARGET=thumbv7em-none-eabihf\n",
    )];
    let costed = edit(
        &bare(),
        "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
        "(timing-model (version \"0.1.0\") (sources) (measured-with) (facts)\n    (costs (cost delivery (target example-target) (value 0x28) (unit ns) (scope \"any code\") (holds-for (tasks 8) (sources 2)) (holds-under-preemption yes) (binary independent \"a fixed pipeline\") (evidence assumed) (basis \"the manual\"))))",
    );
    let t = tree(&[&costed], &board);
    let full = reviewed_all(&costed, "example.base", &t);
    let mut placed = tree(&[&full], &board);
    let bytes = placed
        .get("catalog/experimental/example.base.catalog")
        .unwrap()
        .to_vec();
    placed.remove("catalog/experimental/example.base.catalog");
    placed.insert("catalog/production/example.base.catalog", bytes);
    let mut h = History::default();
    add(&mut h, &n('1'), &[], placed, None);
    let l = load(&h, &n('1')).unwrap_or_else(|e| panic!("{e}"));
    let mut claim = Claim::new(
        &l,
        &h,
        Some(&n('1')),
        P,
        Some("example-target"),
        Strength::Production,
    );
    claim.lookup(TIMING, "delivery").unwrap().unwrap();
    claim.lookup(BEHAVIOR, "one-processor").unwrap().unwrap();
    assert_eq!(
        claim.result().experimental,
        [],
        "a production record is not listed"
    );
    let a = claim.admit().unwrap();
    assert_eq!(a.verdict, Verdict::NotEstablished, "{a:?}");
    assert_eq!(
        a.reasons,
        ["premise 3 has no named commit yet: the published main line's protection is not on"],
        "the only cause left for a reviewed record on a board, until premise 3 has a named commit"
    );
}

#[test]
fn behavioral_and_timing_model_versions_are_independent() {
    use archogen_catalog::invalidation::affected_lines;
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&base(), &timed()], &[]), None);
    let l = load(&h, &n('1')).unwrap();
    let mut on_timing = Claim::new(
        &l,
        &h,
        None,
        P,
        Some("example-target"),
        Strength::Exploratory,
    );
    on_timing.lookup(TIMING, "switch").unwrap().unwrap();
    let timing_lines = on_timing.result().closure;
    assert!(
        !timing_lines.iter().any(|c| c.facet == BEHAVIOR),
        "a claim on a cost rests on no behavioral model"
    );
    let mut on_behavior = Claim::new(
        &l,
        &h,
        None,
        P,
        Some("example-target"),
        Strength::Exploratory,
    );
    on_behavior
        .lookup(BEHAVIOR, "one-processor")
        .unwrap()
        .unwrap();
    let behavior_lines = on_behavior.result().closure;
    assert!(
        !behavior_lines.iter().any(|c| c.facet == TIMING),
        "a claim on a fact rests on no timing model"
    );
    // The behavioral model moves, with its version: the timing claim is untouched, the behavioral one affected.
    let moved = edit(
        &edit(
            &base(),
            "(basis \"the model says so\")",
            "(basis \"the model says it\")",
        ),
        "(behavior-model (version \"0.1.0\")",
        "(behavior-model (version \"0.1.1\")",
    );
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&moved, &timed()], &[]),
        None,
    );
    assert_eq!(
        affected_lines(&h, &n('2'), &timing_lines, false).unwrap(),
        []
    );
    assert_eq!(
        affected_lines(&h, &n('2'), &behavior_lines, false)
            .unwrap()
            .len(),
        1
    );
    // A line carries its own facet's version, which here is not its contract's.
    let l2 = load(&h, &n('2')).unwrap();
    let mut again = Claim::new(
        &l2,
        &h,
        None,
        P,
        Some("example-target"),
        Strength::Exploratory,
    );
    again.lookup(BEHAVIOR, "one-processor").unwrap().unwrap();
    let versions: Vec<(FacetKind, String)> = again
        .result()
        .closure
        .iter()
        .filter(|c| c.id == "example.base")
        .map(|c| (c.facet, c.version.to_string()))
        .collect();
    assert!(
        versions.contains(&(BEHAVIOR, "0.1.1".to_owned())),
        "{versions:?}"
    );
    assert!(
        versions.contains(&(CONTRACT, "0.1.0".to_owned())),
        "{versions:?}"
    );
}

#[test]
fn a_load_refuses_a_lock_that_disagrees_with_its_tree_and_a_review_that_does_not_verify() {
    // A facet changed without its version moving, over the example's own lock.
    let lock = block("# archogen-catalog/1").to_owned();
    let changed = edit(
        &base(),
        "(basis \"the model says so\")",
        "(basis \"the model says it\")",
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&changed], &[]), Some(lock));
    assert_eq!(
        load(&h, &n('1')).unwrap_err().code,
        archogen_catalog::Code::LockUnbumped
    );
    // A review that names a hash the facet never had.
    let zero = format!("sha256:{}", "0".repeat(64));
    let misnamed = with(&bare(), &review(BEHAVIOR, &zero, "production", &[]));
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&misnamed], &[]), None);
    assert_eq!(
        load(&h, &n('1')).unwrap_err().code,
        archogen_catalog::Code::LockReview
    );
}
