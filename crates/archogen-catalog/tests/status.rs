//! §5's status (`M2.7.3.4.1`).
//!
//! Every history is built in memory from the worked example's records and files, read from
//! `docs/specs/catalog/decision_catalog-records-example.md` itself, and each passes the replay and the load
//! checks unless a case says otherwise, so every status is judged on a history a repository could hold.

use std::collections::BTreeSet;

use archogen_catalog::hash::{review_ledger_hash, Catalog};
use archogen_catalog::history::{Commit, CommitterDate, History};
use archogen_catalog::lock::{self, blessed, Line, Lock};
use archogen_catalog::read_record;
use archogen_catalog::record::FacetKind;
use archogen_catalog::replay::{check_history, replay};
use archogen_catalog::status::{statuses, Status, Statuses};
use archogen_catalog::tree::Tree;

const EXAMPLE: &str =
    include_str!("../../../docs/specs/catalog/decision_catalog-records-example.md");

const BEHAVIOR: FacetKind = FacetKind::BehaviorModel;
const TIMING: FacetKind = FacetKind::TimingModel;

/// The first ```` ```text ```` block that starts with `starts`.
fn block(starts: &str) -> &'static str {
    let at = EXAMPLE
        .find(&format!("```text\n{starts}"))
        .unwrap_or_else(|| panic!("no block starting `{starts}`"));
    let body = &EXAMPLE[at + "```text\n".len()..];
    &body[..body.find("```").unwrap()]
}

/// `example.base` as the example writes it, with its production review of the behavioral model.
fn base() -> String {
    let records = block("(catalog-record example.base");
    let split = records.find("\n(catalog-record example.timed").unwrap();
    records[..split + 1].to_owned()
}

/// `example.base` without its review.
fn bare() -> String {
    let record = base();
    let start = record.find("  (review").unwrap();
    format!("{})\n", record[..start].trim_end())
}

/// `text` with the one occurrence of `old` replaced.
fn edit(text: &str, old: &str, new: &str) -> String {
    assert_eq!(text.matches(old).count(), 1, "`{old}` is not unique");
    text.replace(old, new)
}

/// `record` with `review` added as its last form.
fn with(record: &str, review: &str) -> String {
    let body = record.trim_end();
    format!("{}\n  {review})\n", &body[..body.len() - 1])
}

/// A review form.
fn review(facet: FacetKind, hash: &str, verdict: &str, answers: &[&str]) -> String {
    let answers = if answers.is_empty() {
        String::new()
    } else {
        let quoted: Vec<String> = answers.iter().map(|a| format!("\"{a}\"")).collect();
        format!(" (answers {})", quoted.join(" "))
    };
    format!(
        "(review (facet {}) (hash \"{hash}\") (verdict {verdict}){answers} (by independent-context \"a reviewer\") \
         (date \"2026-09-30\") (basis \"the test says so\"))",
        facet.as_str()
    )
}

/// A commit name: one hex digit forty times.
fn n(digit: char) -> String {
    digit.to_string().repeat(40)
}

/// The example's three files, `extra` files, and these records.
fn tree(records: &[&str], extra: &[(&str, &str)]) -> Tree {
    let mut tree = Tree::new([
        ("docs/example/model.txt".to_owned(), b"model\n".to_vec()),
        (
            "targets/example-target.env".to_owned(),
            b"TARGET_ID=example-target\n".to_vec(),
        ),
        (
            "targets/example-target.eadl".to_owned(),
            b"(platform)\n".to_vec(),
        ),
        (
            "targets/twin-target.env".to_owned(),
            b"TARGET_ID=twin-target\n".to_vec(),
        ),
        (
            "targets/twin-target.eadl".to_owned(),
            b"(platform)\n".to_vec(),
        ),
    ]);
    for (path, bytes) in extra {
        tree.insert(*path, bytes.as_bytes());
    }
    for record in records {
        let id = record["(catalog-record ".len()..]
            .split_whitespace()
            .next()
            .unwrap();
        tree.insert(
            format!("catalog/experimental/{id}.catalog"),
            record.as_bytes(),
        );
    }
    tree
}

/// Commit `name` over `parents`, its lock what blessing writes, or `lock` when one is given.
fn add(history: &mut History, name: &str, parents: &[&str], mut tree: Tree, lock: Option<String>) {
    let text = lock.unwrap_or_else(|| {
        let catalog = Catalog::read(tree.clone()).unwrap_or_else(|e| panic!("{e}"));
        let hashes = catalog.hashes().unwrap_or_else(|e| panic!("{e}"));
        let mut lines: BTreeSet<Line> = blessed(&catalog, &hashes);
        for parent in parents {
            if let Some(bytes) = history.get(parent).unwrap().tree.get(lock::PATH) {
                lines.extend(Lock::parse(bytes).unwrap().lines);
            }
        }
        Lock::new(1, lines).render()
    });
    tree.insert(lock::PATH, text.as_bytes());
    history.insert(
        name,
        Commit {
            parents: parents.iter().map(|p| (*p).to_owned()).collect(),
            // 2026-10-01T00:00:00Z, the day after the example's reviews.
            date: CommitterDate {
                seconds: 1_790_812_800,
                offset_minutes: 0,
            },
            tree,
        },
    );
}

/// The statuses at `head`, once the replay from `bases` and the load checks pass.
fn load(history: &History, bases: &[&str], head: &str) -> Statuses {
    replay(history, bases, head).unwrap_or_else(|e| panic!("replay: {e}"));
    check_history(history, head).unwrap_or_else(|e| panic!("load: {e}"));
    statuses(history, head).unwrap_or_else(|e| panic!("status: {e}"))
}

/// A facet's bound hash over `tree`.
fn bound(tree: &Tree, id: &str, facet: FacetKind) -> String {
    let catalog = Catalog::read(tree.clone()).unwrap_or_else(|e| panic!("{e}"));
    catalog
        .hashes()
        .unwrap()
        .facet(id, facet)
        .unwrap()
        .bound
        .to_string()
}

/// The ledger hash of the last review in `record`.
fn ledger(record: &str) -> String {
    let id = record["(catalog-record ".len()..]
        .split_whitespace()
        .next()
        .unwrap();
    let read = read_record(
        &format!("catalog/experimental/{id}.catalog"),
        record.as_bytes(),
    )
    .unwrap_or_else(|e| panic!("{e}"));
    review_ledger_hash(&read.id, read.reviews.last().unwrap()).to_string()
}

/// `example.base` as another record, `id`, with its reviews gone, on `twin-target`: under its own selection, so
/// the two never supply one name there (§12's `catalog-conflict`).
fn copy(id: &str) -> String {
    edit(
        &edit(
            &bare(),
            "(catalog-record example.base",
            &format!("(catalog-record {id}"),
        ),
        "(targets example-target)",
        "(targets twin-target)",
    )
}

#[test]
fn the_worked_example_is_production_where_reviewed_and_the_record_its_weakest() {
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&base()], &[]), None);
    let s = load(&h, &[], &n('1'));
    let facet = |f: FacetKind| s.facets[&("example.base".to_owned(), f)];
    assert_eq!(facet(BEHAVIOR), Status::Production);
    assert_eq!(facet(FacetKind::Contract), Status::Unreviewed);
    assert_eq!(facet(TIMING), Status::Unreviewed);
    assert_eq!(
        s.records["example.base"],
        Status::Unreviewed,
        "its weakest facet's"
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&bare()], &[]), None);
    assert_eq!(
        load(&h, &[], &n('1')).facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Unreviewed
    );
}

#[test]
fn a_review_at_another_hash_leaves_the_facet_stale() {
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&base()], &[]), None);
    let moved = edit(
        &edit(
            &base(),
            "(basis \"the model says so\")",
            "(basis \"the model says it\")",
        ),
        "(behavior-model (version \"0.1.0\")",
        "(behavior-model (version \"0.1.1\")",
    );
    add(&mut h, &n('2'), &[&n('1')], tree(&[&moved], &[]), None);
    let s = load(&h, &[&n('1')], &n('2'));
    assert_eq!(
        s.facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Stale
    );
}

#[test]
fn a_rejection_binds_until_a_production_review_answers_it() {
    let t0 = tree(&[&bare()], &[]);
    let h_timing = bound(&t0, "example.base", TIMING);
    let rejected = with(&bare(), &review(TIMING, &h_timing, "rejected", &[]));
    let x = ledger(&rejected);
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &[]), None);
    let s = load(&h, &[], &n('1'));
    assert_eq!(
        s.facets[&("example.base".to_owned(), TIMING)],
        Status::Rejected
    );
    assert_eq!(s.records["example.base"], Status::Rejected);
    // A production review that does not answer it leaves it rejected, at any hash.
    let silent = with(&rejected, &review(TIMING, &h_timing, "production", &[]));
    add(&mut h, &n('2'), &[&n('1')], tree(&[&silent], &[]), None);
    assert_eq!(
        load(&h, &[&n('1')], &n('2')).facets[&("example.base".to_owned(), TIMING)],
        Status::Rejected
    );
    let answering = with(&rejected, &review(TIMING, &h_timing, "production", &[&x]));
    add(&mut h, &n('3'), &[&n('1')], tree(&[&answering], &[]), None);
    assert_eq!(
        load(&h, &[&n('1')], &n('3')).facets[&("example.base".to_owned(), TIMING)],
        Status::Production
    );
    // The order of reviews in the file decides nothing.
    let answer_form = review(TIMING, &h_timing, "production", &[&x]);
    let reversed = with(
        &bare(),
        &format!(
            "{answer_form}\n  {}",
            review(TIMING, &h_timing, "rejected", &[])
        ),
    );
    add(&mut h, &n('4'), &[&n('1')], tree(&[&reversed], &[]), None);
    assert_eq!(load(&h, &[&n('1')], &n('4')), load(&h, &[&n('1')], &n('3')));
}

#[test]
fn a_rejection_binds_through_the_lineage_for_good() {
    let t0 = tree(&[&bare()], &[]);
    let rejected = with(
        &bare(),
        &review(TIMING, &bound(&t0, "example.base", TIMING), "rejected", &[]),
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &[]), None);
    // Renamed, with its timing model rewritten, so no item carries the rejection: only the lineage does.
    let renamed = edit(
        &edit(
            &copy("example.next"),
            "(supersedes)",
            "(supersedes example.base)",
        ),
        "(none \"the costs of a machine are its devices'\")",
        "(none \"a machine's costs are its devices'\")",
    );
    add(&mut h, &n('2'), &[&n('1')], tree(&[&renamed], &[]), None);
    let s = load(&h, &[&n('1')], &n('2'));
    assert_eq!(
        s.facets[&("example.next".to_owned(), TIMING)],
        Status::Rejected,
        "through the lineage"
    );
    assert_eq!(
        s.facets[&("example.next".to_owned(), BEHAVIOR)],
        Status::Unreviewed,
        "that facet's alone"
    );
    // A lineage once committed cannot be dropped.
    let dropped = edit(&renamed, "(supersedes example.base)", "(supersedes)").replacen(
        "(version \"0.1.0\")",
        "(version \"0.1.1\")",
        1,
    );
    add(&mut h, &n('3'), &[&n('2')], tree(&[&dropped], &[]), None);
    let s = statuses(&h, &n('3')).unwrap();
    assert_eq!(
        s.facets[&("example.next".to_owned(), TIMING)],
        Status::Rejected
    );
    // Without the lineage, the same content is merely unreviewed.
    let unrelated = edit(&renamed, "(supersedes example.base)", "(supersedes)");
    let mut g = History::default();
    add(&mut g, &n('1'), &[], tree(&[&unrelated], &[]), None);
    assert_eq!(
        load(&g, &[], &n('1')).facets[&("example.next".to_owned(), TIMING)],
        Status::Unreviewed
    );
}

/// A rejection of `facet` of `example.base`, ledgered at `1`, then `other` beside it at `2`; `other`'s status of
/// `facet`.
fn reached(facet: FacetKind, base_record: String, other: String, extra: &[(&str, &str)]) -> Status {
    let t0 = tree(&[&base_record], extra);
    let rejected = with(
        &base_record,
        &review(facet, &bound(&t0, "example.base", facet), "rejected", &[]),
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], extra), None);
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&rejected, &other], extra),
        None,
    );
    let id = other_id();
    load(&h, &[&n('1')], &n('2')).facets[&(id, facet)]
}

fn other_id() -> String {
    "example.other".to_owned()
}

#[test]
fn a_rejection_reaches_a_facet_of_the_same_kind_through_each_item() {
    let other = copy("example.other");
    let own_model = |fact: &str, source: &str| {
        format!(
            "(behavior-model (version \"0.1.0\") (sources \"{source}\") (describes)\n    (facts (fact {fact} yes \
             (locator (file \"{source}\")) (basis \"another model\"))))"
        )
    };
    let base_model = "(behavior-model (version \"0.1.0\") (sources \"docs/example/model.txt\") (describes)\n    (facts (fact one-processor yes (locator (file \"docs/example/model.txt\")) (basis \"the model says so\"))))";
    let files = [
        ("docs/example/other.txt", "other\n"),
        ("docs/example/copy.txt", "model\n"),
    ];
    // A fact's name, alone.
    let by_fact = edit(
        &other,
        base_model,
        &own_model("one-processor", "docs/example/other.txt"),
    );
    assert_eq!(
        reached(BEHAVIOR, bare(), by_fact, &files),
        Status::Rejected,
        "a fact's name"
    );
    // A file's bytes, alone.
    let by_file = edit(
        &other,
        base_model,
        &own_model("compare-level", "docs/example/copy.txt"),
    );
    assert_eq!(
        reached(BEHAVIOR, bare(), by_file, &files),
        Status::Rejected,
        "a file's bytes"
    );
    // Neither: unreviewed.
    let by_none = edit(
        &other,
        base_model,
        &own_model("compare-level", "docs/example/other.txt"),
    );
    assert_eq!(
        reached(BEHAVIOR, bare(), by_none, &files),
        Status::Unreviewed,
        "no item shared"
    );
    // A guarantee, alone: the preconditions differ, so the forms do too.
    let guarantee = edit(
        &other,
        "(preconditions \"a \\\"quoted\\\" precondition\")",
        "(preconditions)",
    );
    assert_eq!(
        reached(FacetKind::Contract, bare(), guarantee, &files),
        Status::Rejected,
        "a guarantee"
    );
    // A precondition, alone.
    let precondition = edit(
        &other,
        "(guarantees \"one processor\")",
        "(guarantees \"another\")",
    );
    assert_eq!(
        reached(FacetKind::Contract, bare(), precondition, &files),
        Status::Rejected,
        "a precondition"
    );
    // Forms that hash the same: an identical `none` statement.
    let none = edit(
        &other,
        "(implementation (version \"0.1.0\")",
        "(implementation (version \"0.2.0\")",
    );
    assert_eq!(
        reached(FacetKind::Implementation, bare(), none, &files),
        Status::Rejected,
        "the same forms, versions apart"
    );
}

#[test]
fn a_rejected_cost_reaches_the_same_cost_on_its_target_and_on_a_target_of_the_same_kind() {
    let env = "TARGET_ID=example-target\nTARGET_KIND=emulator\nRUST_TARGET=riscv32imac-unknown-none-elf\n";
    let costed = |target: &str| {
        edit(
            &bare(),
            "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
            &format!(
                "(timing-model (version \"0.1.0\") (sources) (measured-with) (facts)\n    (costs (cost switch \
                 (target {target}) (value 0x28) (unit ns) (scope \"one switch\") (holds-for (tasks 8) (sources 2)) \
                 (holds-under-preemption yes) (binary unbuilt) (evidence assumed) (basis \"a cost\"))))"
            ),
        )
    };
    let files = [
        ("targets/example-target.env", env),
        (
            "targets/twin-target.env",
            &env.replace("example-target", "twin-target"),
        ),
        ("targets/twin-target.eadl", "(platform)\n"),
        (
            "targets/odd-target.env",
            "TARGET_ID=odd-target\nTARGET_KIND=board\nRUST_TARGET=thumbv7em-none-eabihf\n",
        ),
        ("targets/odd-target.eadl", "(platform)\n"),
    ];
    let other = |target: &str| {
        let record = edit(
            &costed(target),
            "(catalog-record example.base",
            "(catalog-record example.other",
        );
        edit(
            &record,
            "(targets example-target)",
            &format!("(targets {target})"),
        )
    };
    let files: Vec<(&str, &str)> = files.iter().map(|(p, b)| (*p, *b)).collect();
    let with_scope = |r: String| edit(&r, "(scope \"one switch\")", "(scope \"one other switch\")");
    // With the example's `.env`, which gives neither key, the name and target alone reach. Two records cannot
    // both supply `switch` on one target, so the rejected record's cost is renamed before the other takes it up.
    let base_cost = costed("example-target");
    let t0 = tree(&[&base_cost], &[]);
    let rejected = with(
        &base_cost,
        &review(TIMING, &bound(&t0, "example.base", TIMING), "rejected", &[]),
    );
    let renamed = edit(
        &edit(&rejected, "(cost switch", "(cost dispatch"),
        "(timing-model (version \"0.1.0\")",
        "(timing-model (version \"0.1.1\")",
    );
    let taker = edit(
        &edit(
            &with_scope(other("example-target")),
            "(fact one-processor yes",
            "(fact compare-level yes",
        ),
        "(basis \"the model says so\")",
        "(basis \"another model\")",
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &[]), None);
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&renamed, &taker], &[]),
        None,
    );
    assert_eq!(
        load(&h, &[&n('1')], &n('2')).facets[&(other_id(), TIMING)],
        Status::Rejected,
        "the same name and target"
    );
    assert_eq!(
        reached(
            TIMING,
            costed("example-target"),
            with_scope(other("twin-target")),
            &files
        ),
        Status::Rejected,
        "the same name on a target of the same kind and Rust target"
    );
    assert_eq!(
        reached(
            TIMING,
            costed("example-target"),
            with_scope(other("odd-target")),
            &files
        ),
        Status::Unreviewed,
        "another kind of target"
    );
}

#[test]
fn a_source_entry_reaches_as_written_when_its_bytes_have_moved() {
    // Rejected while the file said `model`; the other record names the same entry once it says something else.
    let t0 = tree(&[&bare()], &[]);
    let rejected = with(
        &bare(),
        &review(
            BEHAVIOR,
            &bound(&t0, "example.base", BEHAVIOR),
            "rejected",
            &[],
        ),
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &[]), None);
    let bumped = edit(
        &rejected,
        "(behavior-model (version \"0.1.0\")",
        "(behavior-model (version \"0.1.1\")",
    );
    let other = edit(
        &edit(
            &copy("example.other"),
            "(fact one-processor yes",
            "(fact compare-level yes",
        ),
        "(basis \"the model says so\")",
        "(basis \"another model\")",
    );
    let mut later = tree(&[&bumped, &other], &[]);
    later.insert("docs/example/model.txt", b"model, rewritten\n".to_vec());
    add(&mut h, &n('2'), &[&n('1')], later, None);
    let s = load(&h, &[&n('1')], &n('2'));
    assert_eq!(s.facets[&(other_id(), BEHAVIOR)], Status::Rejected);
}

#[test]
fn an_answer_covers_only_the_items_its_review_saw() {
    let files = [("docs/example/copy.txt", "model\n")];
    let t0 = tree(&[&bare()], &files);
    let rejected = with(
        &bare(),
        &review(
            BEHAVIOR,
            &bound(&t0, "example.base", BEHAVIOR),
            "rejected",
            &[],
        ),
    );
    let x = ledger(&rejected);
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &files), None);
    // The other record shares only the file's bytes, and a review at its hash answers the rejection.
    let other = edit(
        &edit(
            &edit(
                &copy("example.other"),
                "(fact one-processor yes",
                "(fact compare-level yes",
            ),
            "(basis \"the model says so\")",
            "(basis \"another model\")",
        ),
        "docs/example/model.txt\") (describes)",
        "docs/example/copy.txt\") (describes)",
    );
    let other = edit(
        &other,
        "(locator (file \"docs/example/model.txt\"))",
        "(locator (file \"docs/example/copy.txt\"))",
    );
    let t1 = tree(&[&rejected, &other], &files);
    let answered = with(
        &other,
        &review(
            BEHAVIOR,
            &bound(&t1, "example.other", BEHAVIOR),
            "production",
            &[&x],
        ),
    );
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&rejected, &answered], &files),
        None,
    );
    assert_eq!(
        load(&h, &[&n('1')], &n('2')).facets[&(other_id(), BEHAVIOR)],
        Status::Production
    );
    // It then takes the rejected fact itself: the answer did not see it, so the rejection binds again.
    let widened = edit(
        &edit(
            &answered,
            "(basis \"another model\"))))",
            "(basis \"another model\"))\n      (fact one-processor yes (locator (file \"docs/example/copy.txt\")) (basis \"so\"))))",
        ),
        "(behavior-model (version \"0.1.0\")",
        "(behavior-model (version \"0.1.1\")",
    );
    add(
        &mut h,
        &n('3'),
        &[&n('2')],
        tree(&[&rejected, &widened], &files),
        None,
    );
    assert_eq!(
        load(&h, &[&n('2')], &n('3')).facets[&(other_id(), BEHAVIOR)],
        Status::Rejected
    );
    // A review at a hash that holds every item answers it.
    let t3 = tree(&[&rejected, &widened], &files);
    let again = with(
        &widened,
        &review(
            BEHAVIOR,
            &bound(&t3, "example.other", BEHAVIOR),
            "production",
            &[&x],
        ),
    );
    add(
        &mut h,
        &n('4'),
        &[&n('3')],
        tree(&[&rejected, &again], &files),
        None,
    );
    assert_eq!(
        load(&h, &[&n('3')], &n('4')).facets[&(other_id(), BEHAVIOR)],
        Status::Production
    );
}

#[test]
fn a_production_verdict_does_not_outlive_a_rejection_at_its_own_hash() {
    let env2 = [(
        "targets/example-target.env",
        "TARGET_ID=example-target\nTARGET_EXTRA=moved\n",
    )];
    let first = tree(&[&base()], &[]);
    let h1 = bound(&first, "example.base", BEHAVIOR);
    assert_ne!(
        h1,
        bound(&tree(&[&base()], &env2), "example.base", BEHAVIOR),
        "the target moves the bound hash"
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], first, None);
    let rejected = with(&base(), &review(BEHAVIOR, &h1, "rejected", &[]));
    let x = ledger(&rejected);
    add(&mut h, &n('2'), &[&n('1')], tree(&[&rejected], &[]), None);
    assert_eq!(
        load(&h, &[&n('1')], &n('2')).facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Rejected
    );
    let h2 = bound(&tree(&[&rejected], &env2), "example.base", BEHAVIOR);
    let answered = with(&rejected, &review(BEHAVIOR, &h2, "production", &[&x]));
    add(&mut h, &n('3'), &[&n('2')], tree(&[&answered], &env2), None);
    assert_eq!(
        load(&h, &[&n('2')], &n('3')).facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Production
    );
    // The target moves back: the earlier production review at `h1` counts only once a review at `h1` answers.
    add(&mut h, &n('4'), &[&n('3')], tree(&[&answered], &[]), None);
    assert_eq!(
        load(&h, &[&n('3')], &n('4')).facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Stale
    );
}

#[test]
fn a_waived_line_lowers_a_status_and_never_raises_one() {
    // A production review naming the hash the facet will have: it fails where it is ledgered, and is waived.
    let later = edit(
        &base(),
        "(basis \"the model says so\")",
        "(basis \"the model says it\")",
    );
    let later = edit(
        &later,
        "(behavior-model (version \"0.1.0\")",
        "(behavior-model (version \"0.1.1\")",
    );
    let h_later = bound(&tree(&[&later], &[]), "example.base", BEHAVIOR);
    let early = with(&bare(), &review(BEHAVIOR, &h_later, "production", &[]));
    let w = ledger(&early);
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&early], &[]), None);
    let lock1 = String::from_utf8(
        h.get(&n('1'))
            .unwrap()
            .tree
            .get(lock::PATH)
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    let waived = format!("{lock1}example.base waiver {w} {}\n", n('1'));
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&early], &[]),
        Some(waived),
    );
    // The facet moves to the hash the waived review names; that review establishes nothing.
    let moved = edit(
        &edit(
            &bare(),
            "(basis \"the model says so\")",
            "(basis \"the model says it\")",
        ),
        "(behavior-model (version \"0.1.0\")",
        "(behavior-model (version \"0.1.1\")",
    );
    let at_later = with(&moved, &review(BEHAVIOR, &h_later, "production", &[]));
    assert_eq!(ledger(&at_later), w, "the same review, so the same line");
    add(&mut h, &n('3'), &[&n('2')], tree(&[&at_later], &[]), None);
    replay(&h, &[&n('2')], &n('3')).unwrap_or_else(|e| panic!("{e}"));
    check_history(&h, &n('3')).unwrap_or_else(|e| panic!("{e}"));
    let s = statuses(&h, &n('3')).unwrap();
    assert_eq!(
        s.facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Stale,
        "a waived production review"
    );
    // A waived rejection binds.
    let rejected = with(
        &bare(),
        &review(
            BEHAVIOR,
            &format!("sha256:{}", "0".repeat(64)),
            "rejected",
            &[],
        ),
    );
    let r = ledger(&rejected);
    let mut g = History::default();
    add(&mut g, &n('1'), &[], tree(&[&rejected], &[]), None);
    let lock1 = String::from_utf8(
        g.get(&n('1'))
            .unwrap()
            .tree
            .get(lock::PATH)
            .unwrap()
            .to_vec(),
    )
    .unwrap();
    add(
        &mut g,
        &n('2'),
        &[&n('1')],
        tree(&[&rejected], &[]),
        Some(format!("{lock1}example.base waiver {r} {}\n", n('1'))),
    );
    replay(&g, &[&n('1')], &n('2')).unwrap_or_else(|e| panic!("{e}"));
    check_history(&g, &n('2')).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(
        statuses(&g, &n('2')).unwrap().facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Rejected
    );
}

#[test]
fn an_answer_lifts_a_rejection_only_for_its_own_records_facet() {
    // Two records with the same behavioral model: the rejection of one reaches the other through every item, and
    // the other's answer lifts it there alone.
    let t0 = tree(&[&bare()], &[]);
    let rejected = with(
        &bare(),
        &review(
            BEHAVIOR,
            &bound(&t0, "example.base", BEHAVIOR),
            "rejected",
            &[],
        ),
    );
    let x = ledger(&rejected);
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &[]), None);
    let t1 = tree(&[&rejected, &copy("example.other")], &[]);
    let other = with(
        &copy("example.other"),
        &review(
            BEHAVIOR,
            &bound(&t1, "example.other", BEHAVIOR),
            "production",
            &[&x],
        ),
    );
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&rejected, &other], &[]),
        None,
    );
    let s = load(&h, &[&n('1')], &n('2'));
    assert_eq!(s.facets[&(other_id(), BEHAVIOR)], Status::Production);
    assert_eq!(
        s.facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Rejected,
        "another record's answer"
    );
    // A rejection stands through a version bump, and another facet's review does not answer it.
    let t0 = tree(&[&base()], &[]);
    let timing_rejected = with(
        &base(),
        &review(TIMING, &bound(&t0, "example.base", TIMING), "rejected", &[]),
    );
    let y = ledger(&timing_rejected);
    let mut g = History::default();
    add(&mut g, &n('1'), &[], tree(&[&timing_rejected], &[]), None);
    let rewritten = edit(
        &timing_rejected,
        "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
        "(timing-model (version \"0.1.1\") (none \"a machine's costs are its devices'\"))",
    );
    let t2 = tree(&[&rewritten], &[]);
    let wrong_facet = with(
        &rewritten,
        &review(
            BEHAVIOR,
            &bound(&t2, "example.base", BEHAVIOR),
            "production",
            &[&y],
        ),
    );
    add(
        &mut g,
        &n('2'),
        &[&n('1')],
        tree(&[&wrong_facet], &[]),
        None,
    );
    // The gate refuses such an answer (`M2.7.3.4.2`); a bypassed one still leaves the rejection standing.
    let refused = replay(&g, &[&n('1')], &n('2')).unwrap_err();
    assert_eq!(refused.code, archogen_catalog::Code::Review, "{refused}");
    let s = statuses(&g, &n('2')).unwrap();
    assert_eq!(
        s.facets[&("example.base".to_owned(), TIMING)],
        Status::Rejected,
        "at any hash, by another facet"
    );
    assert_eq!(
        s.facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Production
    );
}

#[test]
fn the_lineage_is_followed_through_a_chain() {
    let t0 = tree(&[&bare()], &[]);
    let rejected = with(
        &bare(),
        &review(TIMING, &bound(&t0, "example.base", TIMING), "rejected", &[]),
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &[]), None);
    let rewrite = |r: String| {
        edit(
            &r,
            "(none \"the costs of a machine are its devices'\")",
            "(none \"a machine's costs are its devices'\")",
        )
    };
    let middle = rewrite(edit(
        &copy("example.middle"),
        "(supersedes)",
        "(supersedes example.base)",
    ));
    add(&mut h, &n('2'), &[&n('1')], tree(&[&middle], &[]), None);
    let last = edit(
        &edit(
            &copy("example.last"),
            "(supersedes)",
            "(supersedes example.middle)",
        ),
        "(none \"the costs of a machine are its devices'\")",
        "(none \"the last machine's costs are its devices'\")",
    );
    add(&mut h, &n('3'), &[&n('2')], tree(&[&last], &[]), None);
    assert_eq!(
        load(&h, &[&n('2')], &n('3')).facets[&("example.last".to_owned(), TIMING)],
        Status::Rejected
    );
}

#[test]
fn a_waived_line_binds_through_its_line_and_its_form() {
    // The form rejects the behavioral model; the line, edited by hand, says a production review of the timing model.
    let zero = format!("sha256:{}", "0".repeat(64));
    let rejected = with(&bare(), &review(BEHAVIOR, &zero, "rejected", &[]));
    let r = ledger(&rejected);
    let blessed_text = {
        let catalog = Catalog::read(tree(&[&rejected], &[])).unwrap();
        let hashes = catalog.hashes().unwrap();
        Lock::new(1, blessed(&catalog, &hashes)).render()
    };
    let line = format!("example.base review {r} behavior-model rejected {zero}");
    let edited = edit(
        &blessed_text,
        &line,
        &format!("example.base review {r} timing-model production {zero}"),
    );
    let mut h = History::default();
    add(
        &mut h,
        &n('1'),
        &[],
        tree(&[&rejected], &[]),
        Some(edited.clone()),
    );
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&rejected], &[]),
        Some(format!("{edited}example.base waiver {r} {}\n", n('1'))),
    );
    replay(&h, &[&n('1')], &n('2')).unwrap_or_else(|e| panic!("{e}"));
    check_history(&h, &n('2')).unwrap_or_else(|e| panic!("{e}"));
    let s = statuses(&h, &n('2')).unwrap();
    assert_eq!(
        s.facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Rejected,
        "the form's facet and verdict"
    );
    assert_eq!(
        s.facets[&("example.base".to_owned(), TIMING)],
        Status::Rejected,
        "the line's facet"
    );
}

/// `id` implementing package `crates/<package>`, with a timing fact `f` located in it.
fn packaged(id: &str, package: &str) -> String {
    let record = if id == "example.base" {
        bare()
    } else {
        copy(id)
    };
    edit(
        &edit(
            &record,
            "(implementation (version \"0.1.0\") (none \"a machine has no code\"))",
            &format!("(implementation (version \"0.1.0\") (sources \"crates/{package}\"))"),
        ),
        "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
        &format!(
            "(timing-model (version \"0.1.0\") (sources) (measured-with)\n    (facts (fact f yes (locator (code {id} \
             \"crates/{package}/src/lib.rs\")) (basis \"see the code\"))) (costs))"
        ),
    )
}

/// A workspace of two packages.
fn packages(p_lib: &'static str) -> Vec<(&'static str, &'static str)> {
    vec![
        ("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n"),
        ("crates/p/Cargo.toml", "[package]\nname = \"p\"\n"),
        ("crates/p/src/lib.rs", p_lib),
        ("crates/q/Cargo.toml", "[package]\nname = \"q\"\n"),
        ("crates/q/src/lib.rs", "//! q\n"),
    ]
}

#[test]
fn a_timing_fact_and_an_implementations_package_reach_by_name() {
    let files = packages("//! p\n");
    // A timing fact's name, alone: the other record's code and forms differ.
    assert_eq!(
        reached(
            TIMING,
            packaged("example.base", "p"),
            packaged("example.other", "q"),
            &files
        ),
        Status::Rejected,
        "a timing fact's name"
    );
    // A package entry, as written, once the package's bytes have moved.
    let a = packaged("example.base", "p");
    let t0 = tree(&[&a], &files);
    let rejected = with(
        &a,
        &review(
            FacetKind::Implementation,
            &bound(&t0, "example.base", FacetKind::Implementation),
            "rejected",
            &[],
        ),
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &files), None);
    let moved = edit(
        &rejected,
        "(implementation (version \"0.1.0\")",
        "(implementation (version \"0.1.1\")",
    );
    let other = edit(
        &packaged("example.other", "q"),
        "(sources \"crates/q\"))",
        "(sources \"crates/p\" \"crates/q\"))",
    );
    let mut later = tree(&[&moved, &other], &files);
    later.insert("crates/p/src/lib.rs", b"//! p, rewritten\n".to_vec());
    later.insert(
        "crates/p/Cargo.toml",
        b"[package]\nname = \"p\"\nversion = \"0.2.0\"\n".to_vec(),
    );
    add(&mut h, &n('2'), &[&n('1')], later, None);
    assert_eq!(
        load(&h, &[&n('1')], &n('2')).facets[&(other_id(), FacetKind::Implementation)],
        Status::Rejected
    );
}

#[test]
fn a_waived_production_review_still_names_the_facet_its_form_names() {
    let zero = format!("sha256:{}", "0".repeat(64));
    let reviewed = with(&bare(), &review(BEHAVIOR, &zero, "production", &[]));
    let r = ledger(&reviewed);
    let blessed_text = {
        let catalog = Catalog::read(tree(&[&reviewed], &[])).unwrap();
        let hashes = catalog.hashes().unwrap();
        Lock::new(1, blessed(&catalog, &hashes)).render()
    };
    let line = format!("example.base review {r} behavior-model production {zero}");
    let edited = edit(
        &blessed_text,
        &line,
        &format!("example.base review {r} timing-model production {zero}"),
    );
    let mut h = History::default();
    add(
        &mut h,
        &n('1'),
        &[],
        tree(&[&reviewed], &[]),
        Some(edited.clone()),
    );
    let waived = format!("{edited}example.base waiver {r} {}\n", n('1'));
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&reviewed], &[]),
        Some(waived),
    );
    replay(&h, &[&n('1')], &n('2')).unwrap_or_else(|e| panic!("{e}"));
    check_history(&h, &n('2')).unwrap_or_else(|e| panic!("{e}"));
    let s = statuses(&h, &n('2')).unwrap();
    assert_eq!(
        s.facets[&("example.base".to_owned(), BEHAVIOR)],
        Status::Stale,
        "the form's facet"
    );
    assert_eq!(
        s.facets[&("example.base".to_owned(), TIMING)],
        Status::Stale,
        "the line's facet"
    );
    assert_eq!(
        s.facets[&("example.base".to_owned(), FacetKind::Contract)],
        Status::Unreviewed
    );
}
