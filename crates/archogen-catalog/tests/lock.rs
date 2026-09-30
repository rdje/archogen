//! §9's lock over one tree (`M2.7.3.3.1`).
//!
//! Every case starts from the worked example's lock and its `example.base` record, read from
//! `docs/decisions/catalog/decision_catalog-records-example.md` itself, and changes one thing.

use archogen_catalog::hash::Catalog;
use archogen_catalog::lock::{self, blessed, check_tree, Line, Lock};
use archogen_catalog::tree::Tree;
use archogen_catalog::{Code, Refusal};

const EXAMPLE: &str =
    include_str!("../../../docs/decisions/catalog/decision_catalog-records-example.md");

const RECORD: &str = "catalog/experimental/example.base.catalog";

/// The first ```` ```text ```` block that starts with `starts`.
fn block(starts: &str) -> &'static str {
    let at = EXAMPLE
        .find(&format!("```text\n{starts}"))
        .unwrap_or_else(|| panic!("no block starting `{starts}`"));
    let body = &EXAMPLE[at + "```text\n".len()..];
    &body[..body.find("```").unwrap()]
}

/// `example.base` as the example writes it.
fn base() -> String {
    let records = block("(catalog-record example.base");
    let split = records.find("\n(catalog-record example.timed").unwrap();
    records[..split + 1].to_owned()
}

/// The example's lock, holding `example.base` alone.
fn lock_text() -> String {
    block("# archogen-catalog/1").to_owned()
}

/// The example's three files, the record, and the lock when there is one.
fn tree(record: &str, lock: Option<&str>) -> Tree {
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
    tree.insert(RECORD, record.as_bytes());
    if let Some(lock) = lock {
        tree.insert(lock::PATH, lock.as_bytes());
    }
    tree
}

/// Read the catalog, compute its hashes, and check the lock against it.
fn load(record: &str, lock: Option<&str>) -> Result<Option<Lock>, Refusal> {
    let catalog = Catalog::read(tree(record, lock))?;
    let hashes = catalog.hashes()?;
    check_tree(&catalog, &hashes)
}

/// The refusal's code, failing the test when the load passed.
fn refused(result: Result<Option<Lock>, Refusal>) -> Refusal {
    match result {
        Ok(_) => panic!("loaded, and a refusal was expected"),
        Err(r) => r,
    }
}

/// `text` with the one occurrence of `old` replaced.
fn edit(text: &str, old: &str, new: &str) -> String {
    assert_eq!(text.matches(old).count(), 1, "`{old}` is not unique");
    text.replace(old, new)
}

/// The lock's line that starts with `prefix`.
fn line(lock: &str, prefix: &str) -> String {
    lock.lines()
        .find(|l| l.starts_with(prefix))
        .unwrap_or_else(|| panic!("no line `{prefix}`"))
        .to_owned()
}

#[test]
fn the_worked_example_lock_reads_checks_and_renders_back() {
    let text = lock_text();
    let lock = Lock::parse(text.as_bytes()).unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(lock.render(), text);
    assert_eq!(lock.version, 1);
    assert_eq!(lock.lines.len(), 5);
    assert_eq!(
        load(&base(), Some(&text)).unwrap_or_else(|e| panic!("{e}")),
        Some(lock.clone())
    );
    let catalog = Catalog::read(tree(&base(), Some(&text))).unwrap();
    let hashes = catalog.hashes().unwrap();
    let lines: Vec<Line> = blessed(&catalog, &hashes).into_iter().collect();
    assert_eq!(
        lines, lock.lines,
        "blessing writes exactly the example's lines"
    );
    assert_eq!(Lock::new(1, lines).render(), text);
}

#[test]
fn the_first_line_names_a_known_rules_version() {
    let text = lock_text();
    let rest = &text["# archogen-catalog/1\n".len()..];
    for (first, why) in [
        ("", "missing, with the lines after it"),
        (
            "# archogen-catalog/2\n",
            "a version the loader does not know",
        ),
        ("# archogen-catalog/0\n", "version 0"),
        ("# archogen-catalog/01\n", "a leading zero"),
        ("# archogen-catalog/\n", "no version"),
        ("#archogen-catalog/1\n", "no space"),
        ("# archogen-catalog/1 \n", "a trailing space"),
        ("# archogen-rules/1\n", "another name"),
        (
            "# archogen-catalog/+1\n",
            "a sign, which Rust's integer parse would take",
        ),
    ] {
        let lock = format!("{first}{rest}");
        let r = refused(load(&base(), Some(&lock)));
        assert_eq!(r.code, Code::LockReview, "{why}: {r}");
        assert_eq!(
            (r.path.as_str(), r.at.map(|a| a.line)),
            (lock::PATH, Some(1)),
            "{why}"
        );
    }
    let r = refused(load(&base(), Some("")));
    assert_eq!(r.code, Code::LockReview, "an empty lock: {r}");
    assert!(r.message.contains("missing"), "{r}");
}

#[test]
fn every_line_matches_one_of_the_three_forms() {
    let text = lock_text();
    let facet = line(&text, "example.base behavior-model");
    let review = line(&text, "example.base review");
    let hex = review.split(' ').nth(2).unwrap().to_owned();
    let cases = [
        (facet.clone(), facet.replace("sha256:", "SHA256:")),
        (
            facet.clone(),
            facet.replace("behavior-model", "behaviour-model"),
        ),
        (facet.clone(), facet.replace(" 0.1.0 ", " 0.1 ")),
        (facet.clone(), facet.replace(" 0.1.0 ", " 0.01.0 ")),
        (facet.clone(), format!("{facet} extra")),
        (facet.clone(), format!("{facet} ")),
        (facet.clone(), facet.replacen('e', "E", 1)),
        (facet.clone(), facet.replace('c', "C")),
        (review.clone(), review.replace(" production ", " accepted ")),
        (
            review.clone(),
            review.replace(" behavior-model ", " model "),
        ),
        (review.clone(), review.replacen(" review ", " reviewed ", 1)),
        (
            review.clone(),
            review.rsplit_once(' ').unwrap().0.to_owned(),
        ),
    ];
    let zero = format!("sha256:{}", "0".repeat(64));
    let appended = [
        format!("zz_z contract 0.1.0 {zero}"),
        format!("zz..z contract 0.1.0 {zero}"),
        format!("zz-.z contract 0.1.0 {zero}"),
    ];
    for extra in appended {
        let lock = format!("{text}{extra}\n");
        let r = refused(load(&base(), Some(&lock)));
        assert_eq!(
            (r.code, r.at.map(|a| a.line)),
            (Code::LockReview, Some(7)),
            "`{extra}`: {r}"
        );
        assert!(
            r.message.contains("none of the lock's forms"),
            "`{extra}`: {r}"
        );
    }
    for (old, new) in cases {
        let lock = edit(&text, &old, &new);
        let r = refused(load(&base(), Some(&lock)));
        assert_eq!(r.code, Code::LockReview, "`{new}`: {r}");
        assert_eq!(r.path, lock::PATH, "`{new}`");
    }
    for commit in [
        "0".repeat(39),
        "A".repeat(40),
        format!("{}g", "0".repeat(39)),
    ] {
        let waiver = format!("example.base waiver {hex} {commit}");
        let lock = format!("{text}{waiver}\n");
        let r = refused(load(&base(), Some(&lock)));
        assert_eq!(r.code, Code::LockReview, "`{waiver}`: {r}");
        assert_eq!(r.at.map(|a| a.line), Some(7), "`{waiver}`");
    }
    let mut bytes = text.clone().into_bytes();
    let at = text.find("contract 0.1.0").unwrap();
    bytes[at] = 0xff;
    let r = Lock::parse(&bytes).unwrap_err();
    assert_eq!(
        (r.code, r.at.map(|a| a.line)),
        (Code::LockReview, Some(2)),
        "a byte that is not UTF-8: {r}"
    );
    for (lock, why) in [
        (text.replacen('\n', "\n\n", 2), "an empty line"),
        (
            text.trim_end().to_owned(),
            "no line feed after the last line",
        ),
        (
            edit(&text, "contract 0.1.0", "contract 0.1.0\u{e9}"),
            "a byte outside ASCII",
        ),
        (edit(&text, "contract 0.1.0", "contract\t0.1.0"), "a tab"),
    ] {
        let r = refused(load(&base(), Some(&lock)));
        assert_eq!(r.code, Code::LockReview, "{why}: {r}");
    }
}

#[test]
fn lines_are_in_order_and_each_is_there_once() {
    let text = lock_text();
    let contract = line(&text, "example.base contract");
    let implementation = line(&text, "example.base implementation");
    let review = line(&text, "example.base review");
    let ledger = review.split(' ').nth(2).unwrap();
    let waiver = format!("example.base waiver {ledger} {}", "ab".repeat(20));
    for (lock, why) in [
        (format!("{text}{waiver}\n{waiver}\n"), "a waiver repeated"),
        (
            edit(
                &text,
                &format!("{contract}\n{implementation}\n"),
                &format!("{implementation}\n{contract}\n"),
            ),
            "two facets swapped",
        ),
        (
            edit(&text, &contract, &format!("{contract}\n{contract}")),
            "a line repeated",
        ),
        (
            edit(&text, &contract, &format!("{review}\n{contract}")),
            "a review before a facet",
        ),
        (
            edit(
                &text,
                &review,
                &format!("{review}\n{}", review.replace(" production ", " rejected ")),
            ),
            "a second line for one review",
        ),
        (
            edit(
                &text,
                &contract,
                &format!(
                    "{contract}\n{}",
                    contract.replace(" 0.1.0 sha256:7d", " 0.1.0 sha256:8d")
                ),
            ),
            "one facet version twice, with two hashes",
        ),
        (
            edit(
                &text,
                &contract,
                &format!("zzz contract 0.1.0 sha256:{}\n{contract}", "0".repeat(64)),
            ),
            "an id after another bytewise, put first",
        ),
    ] {
        let r = refused(load(&base(), Some(&lock)));
        assert_eq!(r.code, Code::LockReview, "{why}: {r}");
        assert!(
            r.message.contains("order") || r.message.contains("second"),
            "{why}: {r}"
        );
    }
    // Versions are in semantic-version order, which is not bytewise: 0.2.0 comes before 0.10.0.
    let hash = format!("sha256:{}", "0".repeat(64));
    let two = format!("example.base timing-model 0.2.0 {hash}");
    let ten = format!("example.base timing-model 0.10.0 {hash}");
    let timing = line(&text, "example.base timing-model");
    let ordered = edit(&text, &timing, &format!("{timing}\n{two}\n{ten}"));
    assert!(Lock::parse(ordered.as_bytes()).is_ok());
    let reversed = edit(&text, &timing, &format!("{timing}\n{ten}\n{two}"));
    assert_eq!(
        Lock::parse(reversed.as_bytes()).unwrap_err().code,
        Code::LockReview
    );
    // Reviews and waivers are in bytewise order of their line.
    let low = format!(
        "example.base review sha256:{} behavior-model rejected {hash}",
        "0".repeat(64)
    );
    let parsed = Lock::parse(edit(&text, &review, &format!("{low}\n{review}")).as_bytes());
    assert!(parsed.is_ok());
    let parsed = Lock::parse(edit(&text, &review, &format!("{review}\n{low}")).as_bytes());
    assert_eq!(parsed.unwrap_err().code, Code::LockReview);
}

#[test]
fn a_facet_changed_without_a_version_bump_is_unbumped() {
    let record = edit(
        &base(),
        "(basis \"the model says so\")",
        "(basis \"the model says it\")",
    );
    let r = refused(load(&record, Some(&lock_text())));
    assert_eq!(r.code, Code::LockUnbumped, "{r}");
    assert_eq!(r.path, RECORD);
    assert_eq!(r.field, "behavior-model version");
    assert_eq!(r.at.map(|a| a.line), Some(13), "{r}");
}

#[test]
fn a_version_below_a_locked_one_is_a_downgrade() {
    let text = lock_text();
    let behavior = line(&text, "example.base behavior-model");
    let later = behavior.replace(" 0.1.0 ", " 0.2.0 ");
    let lock = edit(&text, &behavior, &format!("{behavior}\n{later}"));
    let r = refused(load(&base(), Some(&lock)));
    assert_eq!(r.code, Code::LockDowngrade, "{r}");
    assert_eq!(r.field, "behavior-model version");
    // Below it with no line of its own is still a downgrade, not a missing line.
    let lock = edit(&text, &behavior, &later);
    assert_eq!(
        refused(load(&base(), Some(&lock))).code,
        Code::LockDowngrade
    );
}

#[test]
fn a_version_or_a_review_without_a_line_is_missing() {
    let text = lock_text();
    let record = base().replacen("(version \"0.1.0\")", "(version \"0.2.0\")", 1);
    let r = refused(load(&record, Some(&text)));
    assert_eq!(r.code, Code::LockMissing, "{r}");
    assert_eq!(
        (r.field.as_str(), r.at.map(|a| (a.line, a.column))),
        ("contract version", Some((2, 3)))
    );
    for facet in ["implementation", "behavior-model", "timing-model"] {
        let record = edit(
            &base(),
            &format!("({facet} (version \"0.1.0\")"),
            &format!("({facet} (version \"0.1.1\")"),
        );
        let r = refused(load(&record, Some(&text)));
        let field = format!("{facet} version");
        assert_eq!(
            (r.code, r.field.as_str()),
            (Code::LockMissing, field.as_str()),
            "{r}"
        );
    }
    let record = edit(
        &base(),
        "(basis \"the model file says one processor\")))",
        "(basis \"the model file says one processor\"))\n  (review (facet timing-model) \
         (hash \"sha256:0000000000000000000000000000000000000000000000000000000000000000\") (verdict rejected) \
         (by director \"the example\") (date \"2026-09-30\") (basis \"a second review\")))",
    );
    let r = refused(load(&record, Some(&text)));
    assert_eq!(
        (r.code, r.field.as_str()),
        (Code::LockMissing, "review"),
        "{r}"
    );
    assert_eq!(
        r.at.map(|a| a.line),
        Some(20),
        "the record's 19 lines, then this review: {r}"
    );
    let empty = "# archogen-catalog/1\n";
    assert_eq!(refused(load(&base(), Some(empty))).code, Code::LockMissing);
}

#[test]
fn a_review_line_agrees_with_its_form() {
    let text = lock_text();
    let review = line(&text, "example.base review");
    let bound = review.rsplit_once(' ').unwrap().1.to_owned();
    for new in [
        review.replace(" production ", " rejected "),
        review.replace(" behavior-model ", " timing-model "),
        review.replace(&bound, &format!("sha256:{}", "0".repeat(64))),
    ] {
        let lock = edit(&text, &review, &new);
        let r = refused(load(&base(), Some(&lock)));
        assert_eq!(
            (r.code, r.field.as_str()),
            (Code::LockReview, "review"),
            "`{new}`: {r}"
        );
        assert_eq!(r.path, RECORD);
    }
}

#[test]
fn a_ledgered_review_cannot_be_taken_back() {
    let record = base();
    let start = record.find("  (review").unwrap();
    let without = format!("{})\n", record[..start].trim_end());
    let r = refused(load(&without, Some(&lock_text())));
    assert_eq!(
        (r.code, r.field.as_str()),
        (Code::LockReview, "review"),
        "{r}"
    );
    assert!(r.message.contains("taken back"), "{r}");
}

#[test]
fn a_waived_line_is_exempt_from_its_form_and_a_waiver_names_a_line() {
    let text = lock_text();
    let review = line(&text, "example.base review");
    let ledger = review.split(' ').nth(2).unwrap();
    let waiver = format!("example.base waiver {ledger} {}", "ab".repeat(20));
    let waived = format!("{text}{waiver}\n");
    let record = base();
    let start = record.find("  (review").unwrap();
    let without = format!("{})\n", record[..start].trim_end());
    assert!(
        load(&without, Some(&waived)).is_ok(),
        "its record need not hold its form"
    );
    let disagreeing = edit(
        &waived,
        &review,
        &review.replace(" production ", " rejected "),
    );
    assert!(
        load(&base(), Some(&disagreeing)).is_ok(),
        "a form it still holds is not compared with it"
    );
    let long = format!("example.base waiver {ledger} {}", "ab".repeat(32));
    assert!(
        load(&without, Some(&format!("{text}{long}\n"))).is_ok(),
        "a SHA-256 object name"
    );
    let stray = format!(
        "example.base waiver sha256:{} {}",
        "0".repeat(64),
        "ab".repeat(20)
    );
    let r = refused(load(&base(), Some(&format!("{text}{stray}\n"))));
    assert_eq!(
        (r.code, r.at.map(|a| a.line)),
        (Code::LockReview, Some(7)),
        "{r}"
    );
}

#[test]
fn records_without_a_lock_are_missing_and_no_records_need_none() {
    let r = refused(load(&base(), None));
    assert_eq!(r.code, Code::LockMissing, "{r}");
    let mut empty = tree(&base(), None);
    empty.remove(RECORD);
    let catalog = Catalog::read(empty).unwrap();
    assert_eq!(check_tree(&catalog, &catalog.hashes().unwrap()), Ok(None));
}

#[test]
fn the_catalog_is_read_from_its_tree() {
    let mut twice = tree(&base(), Some(&lock_text()));
    twice.insert("catalog/production/example.base.catalog", base().as_bytes());
    let r = Catalog::read(twice).unwrap_err();
    assert_eq!(r.code, Code::Id, "{r}");
    assert_eq!(
        r.path, "catalog/production/example.base.catalog",
        "the second met, in bytewise order"
    );
    let mut stray = tree(&base(), Some(&lock_text()));
    stray.insert("catalog/experimental/notes.txt", b"x".to_vec());
    assert_eq!(Catalog::read(stray).unwrap_err().code, Code::Layout);
    let catalog = Catalog::read(tree(&base(), Some(&lock_text()))).unwrap();
    assert_eq!(catalog.records.keys().collect::<Vec<_>>(), ["example.base"]);
}
