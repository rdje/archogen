//! §10's invalidation (`M2.7.3.4.3`).

mod common;

use archogen_catalog::hash::Catalog;
use archogen_catalog::history::History;
use archogen_catalog::invalidation::{
    affected_lines, affected_reads, Affected, Cause, ClosureLine, Read,
};
use archogen_catalog::lock::current_version;
use archogen_catalog::record::FacetKind;
use archogen_catalog::status::{statuses, Status};
use archogen_catalog::tree::Tree;
use archogen_catalog::{Code, Refusal};
use common::*;

/// `example.timed` as the example writes it: it depends on `example.base`.
fn timed() -> String {
    let records = block("(catalog-record example.base");
    let split = records.find("\n(catalog-record example.timed").unwrap();
    records[split + 1..].to_owned()
}

/// The lines a claim at `head` would record for these facets: each bound hash and status as they are there.
fn record(history: &History, head: &str, facets: &[(&str, FacetKind)]) -> Vec<ClosureLine> {
    let catalog = Catalog::read(history.get(head).unwrap().tree.clone()).unwrap();
    let hashes = catalog.hashes().unwrap();
    let now = statuses(history, head).unwrap_or_else(|e| panic!("{e}"));
    facets
        .iter()
        .map(|(id, facet)| ClosureLine {
            id: (*id).to_owned(),
            facet: *facet,
            version: current_version(&catalog.records[*id], *facet),
            bound: hashes.facet(id, *facet).unwrap().bound,
            status: now.facets[&((*id).to_owned(), *facet)],
        })
        .collect()
}

/// Each affected line's id, facet and cause.
fn causes(affected: &[Affected]) -> Vec<(String, FacetKind, Cause)> {
    affected
        .iter()
        .map(|a| (a.line.id.clone(), a.line.facet, a.cause.clone()))
        .collect()
}

const LINES: [(&str, FacetKind); 4] = [
    ("example.base", FacetKind::Contract),
    ("example.base", BEHAVIOR),
    ("example.timed", FacetKind::Contract),
    ("example.timed", TIMING),
];

/// A history whose first commit holds both example records, and the claim's lines recorded there.
fn claimed() -> (History, Vec<ClosureLine>) {
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&base(), &timed()], &[]), None);
    let lines = record(&h, &n('1'), &LINES);
    assert_eq!(lines[1].status, Status::Production, "the example's review");
    (h, lines)
}

#[test]
fn nothing_changed_affects_nothing() {
    let (mut h, lines) = claimed();
    assert_eq!(affected_lines(&h, &n('1'), &lines, false).unwrap(), []);
    // A facet that was `none` and still is relied on nothing that changed.
    let nones = record(
        &h,
        &n('1'),
        &[
            ("example.base", FacetKind::Implementation),
            ("example.base", TIMING),
        ],
    );
    assert_eq!(affected_lines(&h, &n('1'), &nones, false).unwrap(), []);
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&base(), &timed()], &[]),
        None,
    );
    assert_eq!(affected_lines(&h, &n('2'), &lines, false).unwrap(), []);
}

#[test]
fn a_record_gone_or_unreadable_is_affected_and_what_rests_on_it_cannot_be_hashed() {
    let (mut h, lines) = claimed();
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[], &[]),
        Some(String::new()),
    );
    let got = causes(&affected_lines(&h, &n('2'), &lines, false).unwrap());
    assert!(got.iter().all(|(_, _, c)| *c == Cause::Gone), "{got:?}");
    assert_eq!(got.len(), 4);
    let broken = base().replacen(
        "(catalog-record example.base",
        "(catalog-record example.base (",
        1,
    );
    add(
        &mut h,
        &n('3'),
        &[&n('1')],
        tree(&[&broken, &timed()], &[]),
        Some(String::new()),
    );
    let got = causes(&affected_lines(&h, &n('3'), &lines, false).unwrap());
    assert!(matches!(got[0].2, Cause::Unreadable(_)), "{got:?}");
    assert!(matches!(got[1].2, Cause::Unreadable(_)), "{got:?}");
    assert_eq!(
        got[2].2,
        Cause::Hash(None),
        "its dependency cannot be read: {got:?}"
    );
    // One id in both namespaces names two files.
    let mut twice = tree(&[&base(), &timed()], &[]);
    twice.insert("catalog/production/example.base.catalog", base().as_bytes());
    add(&mut h, &n('4'), &[&n('1')], twice, Some(String::new()));
    let got = causes(&affected_lines(&h, &n('4'), &lines, false).unwrap());
    assert!(
        matches!(&got[0].2, Cause::Unreadable(why) if why.contains("more than one file")),
        "{got:?}"
    );
}

#[test]
fn a_facet_now_none_is_affected() {
    let (mut h, lines) = claimed();
    let none = base()
        .replace(
            "(behavior-model (version \"0.1.0\") (sources \"docs/example/model.txt\") (describes)\n    (facts (fact one-processor yes (locator (file \"docs/example/model.txt\")) (basis \"the model says so\"))))",
            "(behavior-model (version \"0.2.0\") (none \"no longer modelled\"))",
        );
    assert_ne!(none, base());
    let timed_none = timed().replace(
        "(timing-model (version \"1.0.0\") (sources) (measured-with) (facts)",
        "(timing-model (version \"1.1.0\") (none \"no longer costed\")",
    );
    let timed_none = timed_none[..timed_none.find("(none \"no longer costed\")").unwrap()]
        .to_owned()
        + "(none \"no longer costed\")))\n";
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&none, &timed_none], &[]),
        None,
    );
    let got = causes(&affected_lines(&h, &n('2'), &lines, false).unwrap());
    assert_eq!(
        got,
        [
            ("example.base".to_owned(), BEHAVIOR, Cause::None),
            ("example.timed".to_owned(), TIMING, Cause::None),
        ]
    );
    // An implementation that becomes `none`.
    let files = package();
    let mut g = History::default();
    add(
        &mut g,
        &n('1'),
        &[],
        tree(&[&packaged("example.base", "p")], &files),
        None,
    );
    let implementation = record(&g, &n('1'), &[("example.base", FacetKind::Implementation)]);
    let dropped = packaged("example.base", "p").replace(
        "(implementation (version \"0.1.0\") (sources \"crates/p\"))",
        "(implementation (version \"0.2.0\") (none \"no code now\"))",
    );
    let dropped = dropped.replace(
        "(facts (fact f yes (locator (code example.base \"crates/p/src/lib.rs\")) (basis \"see the code\"))) (costs)",
        "(facts) (costs)",
    );
    let dropped = dropped.replacen(
        "(timing-model (version \"0.1.0\")",
        "(timing-model (version \"0.2.0\")",
        1,
    );
    add(&mut g, &n('2'), &[&n('1')], tree(&[&dropped], &files), None);
    assert_eq!(
        causes(&affected_lines(&g, &n('2'), &implementation, false).unwrap()),
        [(
            "example.base".to_owned(),
            FacetKind::Implementation,
            Cause::None
        )]
    );
}

#[test]
fn a_line_whose_own_references_break_cannot_be_hashed() {
    // `example.timed` needs `example.base` at `0.1`; at `0.2` its requirement no longer matches.
    let (mut h, lines) = claimed();
    let bumped = base().replacen("(version \"0.1.0\")", "(version \"0.2.0\")", 1);
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&bumped, &timed()], &[]),
        Some(String::new()),
    );
    let got = causes(&affected_lines(&h, &n('2'), &lines, false).unwrap());
    assert!(matches!(got[0].2, Cause::Hash(Some(_))), "{got:?}");
    assert_eq!(got[2].0, "example.timed");
    assert_eq!(got[2].2, Cause::Hash(None), "{got:?}");
}

#[test]
fn a_bound_hash_that_moved_affects_that_line_alone() {
    let (mut h, lines) = claimed();
    let moved = edit(
        &base(),
        "(behavior-model (version \"0.1.0\")",
        "(behavior-model (version \"0.1.1\")",
    );
    let mut t = tree(&[&moved, &timed()], &[]);
    t.insert("docs/example/model.txt", b"model, rewritten\n".to_vec());
    let now = bound(&t, "example.base", BEHAVIOR);
    add(&mut h, &n('2'), &[&n('1')], t, None);
    let got = causes(&affected_lines(&h, &n('2'), &lines, false).unwrap());
    assert_eq!(got.len(), 1, "{got:?}");
    assert_eq!(
        got[0].2,
        Cause::Hash(Some(
            archogen_evidence::sha256::Digest::parse(&now).unwrap()
        ))
    );
}

#[test]
fn a_later_rejection_or_answer_at_the_recorded_hash_is_affected() {
    let (mut h, lines) = claimed();
    let t = tree(&[&base(), &timed()], &[]);
    let rejected = with(
        &base(),
        &review(
            BEHAVIOR,
            &bound(&t, "example.base", BEHAVIOR),
            "rejected",
            &[],
        ),
    );
    let x = ledger(&rejected);
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&rejected, &timed()], &[]),
        None,
    );
    let got = causes(&affected_lines(&h, &n('2'), &lines, false).unwrap());
    assert_eq!(
        got,
        [(
            "example.base".to_owned(),
            BEHAVIOR,
            Cause::Status(Some(Status::Rejected))
        )]
    );
    // A claim made while it was rejected is affected once the rejection is answered.
    let rejected_lines = record(&h, &n('2'), &LINES);
    let answered = with(
        &rejected,
        &review(
            BEHAVIOR,
            &bound(&t, "example.base", BEHAVIOR),
            "production",
            &[&x],
        ),
    );
    add(
        &mut h,
        &n('3'),
        &[&n('2')],
        tree(&[&answered, &timed()], &[]),
        None,
    );
    let got = causes(&affected_lines(&h, &n('3'), &rejected_lines, false).unwrap());
    assert_eq!(
        got,
        [(
            "example.base".to_owned(),
            BEHAVIOR,
            Cause::Status(Some(Status::Production))
        )]
    );
}

#[test]
fn it_runs_when_the_catalog_does_not_load() {
    let (mut h, lines) = claimed();
    let mut t = tree(&[&base(), &timed()], &[]);
    t.insert("catalog/experimental/notes.txt", b"a stray file\n".to_vec());
    add(&mut h, &n('2'), &[&n('1')], t, Some(String::new()));
    let got = causes(&affected_lines(&h, &n('2'), &lines, false).unwrap());
    assert_eq!(got.len(), 4, "every status now uncomputable: {got:?}");
    assert!(
        got.iter().all(|(_, _, c)| *c == Cause::Status(None)),
        "{got:?}"
    );
    // With no such commit, there is no "now" to judge against.
    let r: Refusal = affected_lines(&h, &n('9'), &lines, false).unwrap_err();
    assert_eq!(r.code, Code::Layout);
}

#[test]
fn a_production_claims_record_left_production() {
    let mut h = History::default();
    let mut t: Tree = tree(&[&base(), &timed()], &[]);
    for id in ["example.base", "example.timed"] {
        let bytes = t
            .get(&format!("catalog/experimental/{id}.catalog"))
            .unwrap()
            .to_vec();
        t.remove(&format!("catalog/experimental/{id}.catalog"));
        t.insert(format!("catalog/production/{id}.catalog"), bytes);
    }
    add(&mut h, &n('1'), &[], t, None);
    let lines = record(&h, &n('1'), &LINES);
    assert_eq!(affected_lines(&h, &n('1'), &lines, true).unwrap(), []);
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&base(), &timed()], &[]),
        None,
    );
    let got = causes(&affected_lines(&h, &n('2'), &lines, true).unwrap());
    assert_eq!(got.len(), 4);
    assert!(
        got.iter().all(|(_, _, c)| *c == Cause::NotProduction),
        "{got:?}"
    );
    assert_eq!(
        affected_lines(&h, &n('2'), &lines, false).unwrap(),
        [],
        "an exploratory claim"
    );
}

#[test]
fn a_read_is_affected_when_the_same_lookup_now_answers_otherwise() {
    let read = |name: &str, found: Option<&str>| Read {
        facet: TIMING,
        name: name.to_owned(),
        profile: "rt-static-up-v1".to_owned(),
        target: Some("example-target".to_owned()),
        found: found.map(str::to_owned),
    };
    let reads = [
        read("switch", Some("example.timed")),
        read("dispatch", Some("example.timed")),
        read("entry", None),
        read("exit", None),
    ];
    let now = |r: &Read| -> Result<Option<String>, Refusal> {
        Ok(match r.name.as_str() {
            "switch" => Some("example.timed".to_owned()),
            "dispatch" => Some("example.other".to_owned()),
            "entry" => Some("example.timed".to_owned()),
            _ => None,
        })
    };
    let got: Vec<&str> = affected_reads(&reads, now)
        .iter()
        .map(|r| r.name.as_str())
        .collect();
    assert_eq!(
        got,
        ["dispatch", "entry"],
        "another record, or one where there was none"
    );
    let unloadable = |_: &Read| -> Result<Option<String>, Refusal> {
        Err(Refusal::new(
            Code::Layout,
            "catalog",
            "(file)",
            None,
            "does not load",
        ))
    };
    assert_eq!(
        affected_reads(&reads, unloadable).len(),
        4,
        "the catalog no longer loads"
    );
}
