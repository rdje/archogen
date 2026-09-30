//! Histories built in memory from the worked example, shared by the tests that need a ledger.
//!
//! The example is read from `docs/decisions/catalog/decision_catalog-records-example.md` itself. Blessing is the
//! crate's own lines joined with the parents' locks, as §9 says bless writes them.

#![allow(dead_code)]

use std::collections::BTreeSet;

use archogen_catalog::hash::{review_ledger_hash, Catalog};
use archogen_catalog::history::{Commit, CommitterDate, History};
use archogen_catalog::lock::{self, blessed, Line, Lock};
use archogen_catalog::read_record;
use archogen_catalog::record::FacetKind;
use archogen_catalog::tree::Tree;

pub const EXAMPLE: &str =
    include_str!("../../../../docs/decisions/catalog/decision_catalog-records-example.md");

pub const BEHAVIOR: FacetKind = FacetKind::BehaviorModel;
pub const TIMING: FacetKind = FacetKind::TimingModel;

/// 2026-10-01T00:00:00Z, the day after the example's reviews.
pub const DAY_AFTER: CommitterDate = CommitterDate {
    seconds: 1_790_812_800,
    offset_minutes: 0,
};

/// The first ```` ```text ```` block that starts with `starts`.
pub fn block(starts: &str) -> &'static str {
    let at = EXAMPLE
        .find(&format!("```text\n{starts}"))
        .unwrap_or_else(|| panic!("no block starting `{starts}`"));
    let body = &EXAMPLE[at + "```text\n".len()..];
    &body[..body.find("```").unwrap()]
}

/// `example.base` as the example writes it, with its production review of the behavioral model.
pub fn base() -> String {
    let records = block("(catalog-record example.base");
    let split = records.find("\n(catalog-record example.timed").unwrap();
    records[..split + 1].to_owned()
}

/// `example.base` without its review.
pub fn bare() -> String {
    let record = base();
    let start = record.find("  (review").unwrap();
    format!("{})\n", record[..start].trim_end())
}

/// `example.base` as another record, `id`, with its reviews gone.
pub fn copy(id: &str) -> String {
    edit(
        &bare(),
        "(catalog-record example.base",
        &format!("(catalog-record {id}"),
    )
}

/// `text` with the one occurrence of `old` replaced.
pub fn edit(text: &str, old: &str, new: &str) -> String {
    assert_eq!(text.matches(old).count(), 1, "`{old}` is not unique");
    text.replace(old, new)
}

/// `record` with `review` added as its last form.
pub fn with(record: &str, review: &str) -> String {
    let body = record.trim_end();
    format!("{}\n  {review})\n", &body[..body.len() - 1])
}

/// A review form dated `2026-09-30`.
pub fn review(facet: FacetKind, hash: &str, verdict: &str, answers: &[&str]) -> String {
    dated(facet, hash, verdict, answers, "2026-09-30")
}

/// A review form with its date.
pub fn dated(facet: FacetKind, hash: &str, verdict: &str, answers: &[&str], date: &str) -> String {
    let answers = if answers.is_empty() {
        String::new()
    } else {
        let quoted: Vec<String> = answers.iter().map(|a| format!("\"{a}\"")).collect();
        format!(" (answers {})", quoted.join(" "))
    };
    format!(
        "(review (facet {}) (hash \"{hash}\") (verdict {verdict}){answers} (by independent-context \"a reviewer\") \
         (date \"{date}\") (basis \"the test says so\"))",
        facet.as_str()
    )
}

/// A commit name: one hex digit forty times.
pub fn n(digit: char) -> String {
    digit.to_string().repeat(40)
}

/// The example's three files, `extra` files, and these records.
pub fn tree(records: &[&str], extra: &[(&str, &str)]) -> Tree {
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

/// The lock blessing writes for `tree` over `parents`' locks.
pub fn bless(history: &History, parents: &[&str], tree: &Tree) -> String {
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

/// Commit `name` over `parents` on `date`, its lock what blessing writes, or `lock` when one is given.
pub fn add_on(
    history: &mut History,
    name: &str,
    parents: &[&str],
    mut tree: Tree,
    lock: Option<String>,
    date: CommitterDate,
) {
    let text = lock.unwrap_or_else(|| bless(history, parents, &tree));
    tree.insert(lock::PATH, text.as_bytes());
    history.insert(
        name,
        Commit {
            parents: parents.iter().map(|p| (*p).to_owned()).collect(),
            date,
            tree,
        },
    );
}

/// Commit `name` the day after the example's reviews.
pub fn add(history: &mut History, name: &str, parents: &[&str], tree: Tree, lock: Option<String>) {
    add_on(history, name, parents, tree, lock, DAY_AFTER);
}

/// A facet's bound hash over `tree`.
pub fn bound(tree: &Tree, id: &str, facet: FacetKind) -> String {
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
pub fn ledger(record: &str) -> String {
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

/// `id` implementing package `crates/<package>`, with a timing fact `f` located in it.
pub fn packaged(id: &str, package: &str) -> String {
    edit(
        &edit(
            &copy(id),
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

/// A workspace of one package, `crates/p`.
pub fn package() -> Vec<(&'static str, &'static str)> {
    vec![
        ("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\n"),
        ("crates/p/Cargo.toml", "[package]\nname = \"p\"\n"),
        ("crates/p/src/lib.rs", "//! p\n"),
    ]
}
