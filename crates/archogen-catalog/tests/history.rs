//! The lock over history (§9 and §5, `M2.7.3.3.2`).
//!
//! Every history is built in memory from the worked example's records and files, read from
//! `docs/specs/catalog/decision_catalog-records-example.md` itself. Blessing is the crate's own
//! [`blessed`] lines joined with the parents' locks, as §9 says bless writes them; a hand-edited lock is written
//! out as text.

use std::collections::BTreeSet;

use archogen_catalog::hash::Catalog;
use archogen_catalog::history::{Commit, CommitterDate, History};
use archogen_catalog::lock::{self, blessed, Line, Lock};
use archogen_catalog::replay::{check_history, compare, replay, replay_under};
use archogen_catalog::tree::Tree;
use archogen_catalog::{Code, Refusal};

const EXAMPLE: &str =
    include_str!("../../../docs/specs/catalog/decision_catalog-records-example.md");

/// The first ```` ```text ```` block that starts with `starts`.
fn block(starts: &str) -> &'static str {
    let at = EXAMPLE
        .find(&format!("```text\n{starts}"))
        .unwrap_or_else(|| panic!("no block starting `{starts}`"));
    let body = &EXAMPLE[at + "```text\n".len()..];
    &body[..body.find("```").unwrap()]
}

/// `example.base` as the example writes it, with its review.
fn base() -> String {
    let records = block("(catalog-record example.base");
    let split = records.find("\n(catalog-record example.timed").unwrap();
    records[..split + 1].to_owned()
}

/// `example.timed` as the example writes it.
fn timed() -> String {
    let records = block("(catalog-record example.base");
    let split = records.find("\n(catalog-record example.timed").unwrap();
    records[split + 1..].to_owned()
}

/// `example.base` without its review.
fn unreviewed() -> String {
    let record = base();
    let start = record.find("  (review").unwrap();
    format!("{})\n", record[..start].trim_end())
}

/// `example.base` whose review names a hash that is not the facet's bound hash.
fn misnamed() -> String {
    edit(
        &base(),
        "sha256:2c55cee38e3334519e295600e799644bea64552d89930c11662eafa7bca43813",
        &format!("sha256:{}", "0".repeat(64)),
    )
}

/// `text` with the one occurrence of `old` replaced.
fn edit(text: &str, old: &str, new: &str) -> String {
    assert_eq!(text.matches(old).count(), 1, "`{old}` is not unique");
    text.replace(old, new)
}

/// A commit name: one hex digit forty times.
fn n(digit: char) -> String {
    digit.to_string().repeat(40)
}

/// The example's three files and these records.
fn tree(records: &[String]) -> Tree {
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
    ]);
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

/// What a commit's lock is.
enum Locked {
    /// What blessing writes: the parents' lines and the lines of the commit's records.
    Blessed,
    /// Written by hand.
    Text(String),
    /// No lock at all.
    Absent,
}

/// The lock blessing writes for `tree` over `parents`' locks.
fn bless(history: &History, parents: &[&str], tree: &Tree) -> String {
    let catalog = Catalog::read(tree.clone()).unwrap_or_else(|e| panic!("{e}"));
    let hashes = catalog.hashes().unwrap_or_else(|e| panic!("{e}"));
    let mut lines: BTreeSet<Line> = blessed(&catalog, &hashes);
    for parent in parents {
        if let Some(bytes) = history.get(parent).unwrap().tree.get(lock::PATH) {
            lines.extend(Lock::parse(bytes).unwrap().lines);
        }
    }
    Lock::new(1, lines).render()
}

/// Add commit `name` with these parents, records and lock; return its lock's text.
fn add(
    history: &mut History,
    name: &str,
    parents: &[&str],
    records: &[String],
    locked: Locked,
) -> String {
    let mut tree = tree(records);
    let text = match locked {
        Locked::Blessed => Some(bless(history, parents, &tree)),
        Locked::Text(text) => Some(text),
        Locked::Absent => None,
    };
    if let Some(text) = &text {
        tree.insert(lock::PATH, text.as_bytes());
    }
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
            hosting: false,
        },
    );
    text.unwrap_or_default()
}

/// The refusal, failing the test when the check passed.
fn refused(result: Result<(), Refusal>) -> Refusal {
    match result {
        Ok(()) => panic!("passed, and a refusal was expected"),
        Err(r) => r,
    }
}

/// The lock's line that starts with `prefix`.
fn line(lock: &str, prefix: &str) -> String {
    lock.lines()
        .find(|l| l.starts_with(prefix))
        .unwrap_or_else(|| panic!("no line `{prefix}`"))
        .to_owned()
}

#[test]
fn the_worked_example_replays_and_loads() {
    let mut h = History::default();
    let lock = add(&mut h, &n('1'), &[], &[base()], Locked::Blessed);
    assert_eq!(
        lock,
        block("# archogen-catalog/1"),
        "blessing writes the example's lock"
    );
    replay(&h, &[], &n('1')).unwrap_or_else(|e| panic!("{e}"));
    check_history(&h, &n('1')).unwrap_or_else(|e| panic!("{e}"));
    add(&mut h, &n('2'), &[&n('1')], &[base()], Locked::Blessed);
    replay(&h, &[&n('1')], &n('2')).unwrap_or_else(|e| panic!("{e}"));
    replay(&h, &[], &n('2')).unwrap_or_else(|e| panic!("{e}"));
    check_history(&h, &n('2')).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn the_lock_is_append_only() {
    let mut h = History::default();
    add(&mut h, &n('0'), &[], &[base()], Locked::Absent);
    let r = refused(replay(&h, &[], &n('0')));
    assert_eq!(
        r.code,
        Code::LockMissing,
        "records and no lock, at a replayed commit: {r}"
    );
    add(&mut h, &n('1'), &[], &[base()], Locked::Blessed);
    // The record retired, and its lines dropped with it: they stay behind as history.
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        &[],
        Locked::Text("# archogen-catalog/1\n".to_owned()),
    );
    let r = refused(replay(&h, &[&n('1')], &n('2')));
    assert_eq!(r.code, Code::LockReview, "{r}");
    assert!(
        r.message.contains("dropped or altered") && r.message.contains(&n('2')),
        "{r}"
    );
    add(&mut h, &n('3'), &[&n('1')], &[], Locked::Absent);
    let r = refused(replay(&h, &[&n('1')], &n('3')));
    assert!(r.message.contains("there is no lock"), "{r}");
    // A lower first line than a parent's, compared directly, since this loader knows one version.
    let r = compare(
        Some(&Lock::new(1, [])),
        Some(&Lock::new(2, [])),
        "child",
        "parent",
    )
    .unwrap_err();
    assert_eq!(
        (r.code, r.at.map(|a| a.line)),
        (Code::LockReview, Some(1)),
        "{r}"
    );
    assert!(compare(
        Some(&Lock::new(2, [])),
        Some(&Lock::new(1, [])),
        "child",
        "parent"
    )
    .is_ok());
}

#[test]
fn no_line_is_new_under_a_version_lower_than_the_bases() {
    // A loader that knows a second rules version. The lines are still computed under the first, which is a
    // fiction of this test: it judges the first line's order, not the hashes a second version would define.
    let known = [1, 2];
    let mut h = History::default();
    let a = add(&mut h, &n('a'), &[], &[base()], Locked::Blessed);
    add(
        &mut h,
        &n('b'),
        &[&n('a')],
        &[base()],
        Locked::Text(a.replace("catalog/1", "catalog/2")),
    );
    replay_under(&h, &[&n('a')], &n('b'), &known).unwrap_or_else(|e| panic!("{e}"));
    // A branch forked from `a` before the bump goes on ledgering under the first version.
    let bumped = edit(
        &base(),
        "(timing-model (version \"0.1.0\")",
        "(timing-model (version \"0.1.1\")",
    );
    add(&mut h, &n('c'), &[&n('a')], &[bumped], Locked::Blessed);
    replay_under(&h, &[&n('a')], &n('c'), &known).unwrap_or_else(|e| panic!("{e}"));
    let r = refused(replay_under(&h, &[&n('b')], &n('c'), &known));
    assert_eq!(r.code, Code::LockReview, "{r}");
    assert!(r.message.contains("lower than the base's"), "{r}");
    // With no new line, a lower version is not this rule's.
    add(&mut h, &n('d'), &[&n('a')], &[base()], Locked::Blessed);
    replay_under(&h, &[&n('b')], &n('d'), &known).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn every_new_line_is_one_blessing_writes() {
    let mut h = History::default();
    let lock = add(&mut h, &n('1'), &[], &[base()], Locked::Blessed);
    let contract = line(&lock, "example.base contract");
    let zero = format!("sha256:{}", "0".repeat(64));
    for (digit, text) in [
        (
            '2',
            edit(
                &lock,
                &contract,
                &format!("example.base contract 0.0.9 {zero}\n{contract}"),
            ),
        ),
        ('3', format!("{lock}other.gone contract 0.1.0 {zero}\n")),
        (
            '4',
            format!("{lock}zz.gone review {zero} behavior-model rejected {zero}\n"),
        ),
    ] {
        add(&mut h, &n(digit), &[&n('1')], &[base()], Locked::Text(text));
        let r = refused(replay(&h, &[&n('1')], &n(digit)));
        assert_eq!(r.code, Code::LockReview, "{r}");
        assert!(r.message.contains("blessing would not write"), "{r}");
    }
}

#[test]
fn the_checks_over_one_tree_run_at_every_replayed_commit() {
    let mut h = History::default();
    let lock = add(&mut h, &n('1'), &[], &[base()], Locked::Blessed);
    let changed = edit(
        &base(),
        "(basis \"the model says so\")",
        "(basis \"the model says it\")",
    );
    add(&mut h, &n('2'), &[&n('1')], &[changed], Locked::Text(lock));
    add(&mut h, &n('3'), &[&n('2')], &[base()], Locked::Blessed);
    let r = refused(replay(&h, &[&n('1')], &n('3')));
    assert_eq!(
        r.code,
        Code::LockUnbumped,
        "judged at the commit that made it: {r}"
    );
    assert!(r.message.contains(&n('2')), "{r}");
}

#[test]
fn a_review_is_verified_where_it_was_ledgered() {
    let mut h = History::default();
    add(&mut h, &n('1'), &[], &[misnamed()], Locked::Blessed);
    let r = refused(replay(&h, &[], &n('1')));
    assert_eq!(r.code, Code::LockReview, "{r}");
    assert!(
        r.message.contains("does not verify") && r.message.contains("bound hash"),
        "{r}"
    );
    let r = refused(check_history(&h, &n('1')));
    assert_eq!(r.code, Code::LockReview, "{r}");
    assert!(r.message.contains("which ledgered it"), "{r}");
}

#[test]
fn a_bypassed_gate_is_caught_at_load() {
    // Each commit `a` was made with the gate bypassed; `b` retires the record, so the one-tree checks at `b` see
    // nothing, and only the verification at `a`, the ledgering commit, refuses.
    let good = block("# archogen-catalog/1").to_owned();
    let review = line(&good, "example.base review");
    let zero = format!("sha256:{}", "0".repeat(64));
    let ledger = review.split(' ').nth(2).unwrap().to_owned();
    for (lock, why) in [
        (
            edit(
                &good,
                &review,
                &review.replace(" production ", " rejected "),
            ),
            "form gives",
        ),
        (
            edit(
                &good,
                &review,
                &review.replace(" behavior-model ", " timing-model "),
            ),
            "form gives",
        ),
        (edit(&good, &ledger, &zero), "hashes to"),
        (
            format!("{good}zz.gone review {zero} behavior-model rejected {zero}\n"),
            "no record",
        ),
    ] {
        let mut h = History::default();
        add(&mut h, &n('a'), &[], &[base()], Locked::Text(lock.clone()));
        add(&mut h, &n('b'), &[&n('a')], &[], Locked::Text(lock));
        let r = refused(check_history(&h, &n('b')));
        assert_eq!(r.code, Code::LockReview, "{why}: {r}");
        assert!(
            r.message.contains(why) && r.message.contains(&n('a')),
            "{why}: {r}"
        );
    }
    // A ledgering commit whose catalog does not read, or whose hashes do not compute, verifies nothing.
    for (change, why) in [
        (Some("catalog/experimental/notes.txt"), "does not read"),
        (None, "do not compute"),
    ] {
        let mut h = History::default();
        add(&mut h, &n('a'), &[], &[base()], Locked::Text(good.clone()));
        let mut commit = h.get(&n('a')).unwrap().clone();
        match change {
            Some(stray) => commit.tree.insert(stray, b"x".to_vec()),
            None => commit.tree.remove("docs/example/model.txt"),
        }
        h.insert(n('a'), commit);
        add(&mut h, &n('b'), &[&n('a')], &[], Locked::Text(good.clone()));
        let r = refused(check_history(&h, &n('b')));
        assert!(
            r.message.contains(why) && r.message.contains(&n('a')),
            "{why}: {r}"
        );
    }
}

#[test]
fn a_waiver_stands_only_as_section_9_says() {
    let mut h = History::default();
    let bad = add(&mut h, &n('1'), &[], &[misnamed()], Locked::Blessed);
    let ledger = line(&bad, "example.base review")
        .split(' ')
        .nth(2)
        .unwrap()
        .to_owned();
    let waiver = |commit: &str| format!("example.base waiver {ledger} {commit}\n");
    // Named at the commit that ledgered the line, where it fails: it stands, in the replay and at load.
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        &[misnamed()],
        Locked::Text(format!("{bad}{}", waiver(&n('1')))),
    );
    replay(&h, &[&n('1')], &n('2')).unwrap_or_else(|e| panic!("{e}"));
    check_history(&h, &n('2')).unwrap_or_else(|e| panic!("{e}"));
    // Naming a commit that did not ledger it.
    for (digit, named) in [('3', n('2')), ('4', n('e'))] {
        let parent = if digit == '3' { n('2') } else { n('1') };
        let parent_lock = String::from_utf8(
            h.get(&parent)
                .unwrap()
                .tree
                .get(lock::PATH)
                .unwrap()
                .to_vec(),
        )
        .unwrap();
        let text = Lock::parse(format!("{parent_lock}{}", waiver(&named)).as_bytes())
            .map(|l| l.render())
            .unwrap_or_else(|e| panic!("{e}"));
        add(
            &mut h,
            &n(digit),
            &[&parent],
            &[misnamed()],
            Locked::Text(text),
        );
        let r = refused(replay(&h, &[&parent], &n(digit)));
        assert!(r.message.contains("did not ledger"), "{r}");
        let r = refused(check_history(&h, &n(digit)));
        assert!(r.message.contains("did not ledger"), "{r}");
    }
    // Naming a line that verifies: there is nothing to repair.
    let mut g = History::default();
    let good = add(&mut g, &n('1'), &[], &[base()], Locked::Blessed);
    let good_ledger = line(&good, "example.base review")
        .split(' ')
        .nth(2)
        .unwrap()
        .to_owned();
    let text = format!("{good}example.base waiver {good_ledger} {}\n", n('1'));
    add(&mut g, &n('2'), &[&n('1')], &[base()], Locked::Text(text));
    assert!(refused(replay(&g, &[&n('1')], &n('2')))
        .message
        .contains("verifies"));
    assert!(refused(check_history(&g, &n('2')))
        .message
        .contains("verifies"));
    // Added in the commit that ledgers the line: the line is in no parent's lock.
    let mut s = History::default();
    let same = format!("{good}example.base waiver {good_ledger} {}\n", n('1'));
    add(&mut s, &n('1'), &[], &[base()], Locked::Text(same));
    let r = refused(replay(&s, &[], &n('1')));
    assert!(r.message.contains("no parent's lock"), "{r}");
    // A waiver whose review line the lock does not hold.
    let mut w = History::default();
    let text = format!("# archogen-catalog/1\n{}", waiver(&n('1')));
    add(&mut w, &n('1'), &[], &[], Locked::Text(text));
    let r = refused(check_history(&w, &n('1')));
    assert!(r.message.contains("no line of the lock"), "{r}");
}

#[test]
fn a_merge_replays_both_branches_oldest_first() {
    let mut h = History::default();
    let root = n('1');
    let (a, b, m) = (n('a'), n('b'), n('0'));
    add(&mut h, &root, &[], &[unreviewed()], Locked::Blessed);
    add(&mut h, &a, &[&root], &[base()], Locked::Blessed);
    add(
        &mut h,
        &b,
        &[&root],
        &[unreviewed(), timed()],
        Locked::Blessed,
    );
    add(&mut h, &m, &[&a, &b], &[base(), timed()], Locked::Blessed);
    assert_eq!(
        h.between(&[&root], &m).unwrap(),
        [a.clone(), b.clone(), m.clone()],
        "parents first"
    );
    replay(&h, &[&root], &m).unwrap_or_else(|e| panic!("{e}"));
    check_history(&h, &m).unwrap_or_else(|e| panic!("{e}"));
    // A merge that drops one side's record and its lines.
    let a_lock =
        String::from_utf8(h.get(&a).unwrap().tree.get(lock::PATH).unwrap().to_vec()).unwrap();
    add(&mut h, &n('f'), &[&a, &b], &[base()], Locked::Text(a_lock));
    let r = refused(replay(&h, &[&root], &n('f')));
    assert!(
        r.message.contains("dropped or altered") && r.message.contains(&b),
        "{r}"
    );
}

#[test]
fn each_branch_that_ledgered_a_review_is_verified() {
    let mut h = History::default();
    let root = n('1');
    add(&mut h, &root, &[], &[unreviewed()], Locked::Blessed);
    add(&mut h, &n('a'), &[&root], &[base()], Locked::Blessed);
    // The same review ledgered on another branch, where its facet's bound hash has moved.
    let moved = edit(
        &base(),
        "(behavior-model (version \"0.1.0\")",
        "(behavior-model (version \"0.1.1\")",
    );
    add(
        &mut h,
        &n('b'),
        &[&root],
        std::slice::from_ref(&moved),
        Locked::Blessed,
    );
    let r = refused(replay(&h, &[&root], &n('b')));
    assert!(r.message.contains("does not verify"), "{r}");
    add(
        &mut h,
        &n('0'),
        &[&n('a'), &n('b')],
        &[moved],
        Locked::Blessed,
    );
    let r = refused(check_history(&h, &n('0')));
    assert!(
        r.message.contains(&n('b')) && r.message.contains("which ledgered it"),
        "{r}"
    );
    check_history(&h, &n('a')).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn a_history_cut_short_is_refused() {
    let mut h = History::default();
    let lock = bless(&h, &[], &tree(&[base()]));
    add(&mut h, &n('2'), &[&n('9')], &[base()], Locked::Text(lock));
    assert_eq!(refused(replay(&h, &[], &n('2'))).code, Code::Layout);
    assert_eq!(refused(check_history(&h, &n('2'))).code, Code::Layout);
}
