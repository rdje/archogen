//! The reader against the real boundary corpus.
//!
//! Unit tests use inputs chosen to exercise a branch. This suite uses the 21 files that were
//! written for a different purpose entirely — before the reader existed — which is what makes
//! it evidence rather than confirmation. Every one of them must read cleanly, round-trip
//! semantically, and yield its metadata block.

use std::path::{Path, PathBuf};

use eadl_front::{read, Document, SourceMap};

/// The repository root, derived from this crate's manifest directory.
///
/// Repo-root-relative by construction: the repository can be moved or renamed and this still
/// resolves, which is the project's path policy.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Every corpus case, as `(repo-relative path, text)`.
fn corpus() -> Vec<(String, String)> {
    let root = repo_root();
    let mut cases = Vec::new();
    for verdict in ["accept", "reject"] {
        let dir = root.join("docs/semantics/boundary").join(verdict);
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()));
        for entry in entries {
            let path = entry.expect("readable entry").path();
            if path.extension().is_some_and(|e| e == "eadl") {
                let text = std::fs::read_to_string(&path)
                    .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
                let name = format!(
                    "docs/semantics/boundary/{verdict}/{}",
                    path.file_name().expect("a file name").to_string_lossy()
                );
                cases.push((name, text));
            }
        }
    }
    cases.sort();
    cases
}

fn parse(name: &str, text: &str) -> Document {
    let mut sources = SourceMap::new();
    let id = sources.add(name, text).expect("corpus files are small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "{name} did not read cleanly:\n{}",
        diagnostics.render(&sources)
    );
    document
}

#[test]
fn the_corpus_is_the_size_the_index_claims() {
    // A suite that silently iterates over zero files passes forever. This is the census that
    // makes every assertion below mean something.
    let cases = corpus();
    assert_eq!(
        cases.len(),
        21,
        "expected 21 corpus cases, found {}",
        cases.len()
    );
    let accepts = cases
        .iter()
        .filter(|(name, _)| name.contains("/accept/"))
        .count();
    assert_eq!(accepts, 10);
    assert_eq!(cases.len() - accepts, 11);
}

#[test]
fn every_corpus_case_reads_without_a_single_diagnostic() {
    for (name, text) in corpus() {
        let document = parse(&name, &text);
        assert!(!document.forms.is_empty(), "{name} parsed to no forms");
        assert_eq!(
            document.forms.len(),
            1,
            "{name} should hold exactly one declaration"
        );
    }
}

#[test]
fn every_corpus_case_round_trips_semantically() {
    // ROADMAP.md §12 M1's exit gate, measured on the real corpus rather than on a toy.
    for (name, text) in corpus() {
        let first = parse(&name, &text);
        let canonical = first.to_canonical();
        let second = parse(&format!("{name} (canonical)"), &canonical);
        assert!(
            first.structurally_eq(&second),
            "{name} changed structure through canonical form:\n--- first ---\n{canonical}\n--- second ---\n{}",
            second.to_canonical()
        );
        assert_eq!(
            canonical,
            second.to_canonical(),
            "{name} is not a fixed point of canonicalization"
        );
    }
}

#[test]
fn every_corpus_case_yields_its_metadata_block() {
    // The corpus carries its verdict in `;` comments, which is why the reader keeps them. F27
    // reads exactly this.
    for (name, text) in corpus() {
        let document = parse(&name, &text);
        let headers = document.comment_headers();
        let get = |key: &str| {
            headers
                .iter()
                .find(|(k, _)| k == key)
                .map(|(_, v)| v.as_str())
        };

        assert_eq!(
            get("case"),
            Some(
                name.rsplit('/')
                    .next()
                    .and_then(|f| f.strip_suffix(".eadl"))
                    .expect("a case file name")
            ),
            "{name}: the `case` header must match the file name"
        );

        let verdict = get("verdict").unwrap_or_else(|| panic!("{name} has no `verdict` header"));
        let expected = if name.contains("/accept/") {
            "accept"
        } else {
            "reject"
        };
        assert_eq!(
            verdict, expected,
            "{name}: verdict disagrees with its directory"
        );

        assert!(
            matches!(get("ambiguous"), Some("yes" | "no")),
            "{name}: `ambiguous` must be yes or no, got {:?}",
            get("ambiguous")
        );

        let tests = get("tests").unwrap_or_else(|| panic!("{name} has no `tests` header"));
        for test in [
            "externality",
            "implementation-independence",
            "non-prescription",
        ] {
            assert!(
                tests.contains(test),
                "{name}: the `tests` header omits `{test}`: {tests}"
            );
        }

        let rationale = get("rationale").unwrap_or_else(|| panic!("{name} has no `rationale`"));
        assert!(
            rationale.len() > 40,
            "{name}: the rationale is too short to be an argument: {rationale}"
        );

        if expected == "reject" {
            let failing =
                get("failing-test").unwrap_or_else(|| panic!("{name} has no `failing-test`"));
            assert!(
                tests.contains(&format!("{failing}=fail")),
                "{name}: `failing-test: {failing}` disagrees with `tests: {tests}`"
            );
        }

        // ⭐ The exact key set, for EVERY case — not just the presence of the keys we wanted.
        // The weaker version of this test passed while the `counter-width-and-rate` rationale
        // was being truncated: it wrapped onto a line beginning `implementation-independence:`,
        // which opened a spurious seventh header. Presence checks cannot see an extra key; an
        // exact set can.
        let keys: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
        let expected_keys: Vec<&str> = if expected == "reject" {
            vec![
                "case",
                "verdict",
                "ambiguous",
                "tests",
                "failing-test",
                "rationale",
                "other-side",
            ]
        } else {
            vec![
                "case",
                "verdict",
                "ambiguous",
                "tests",
                "rationale",
                "other-side",
            ]
        };
        assert_eq!(
            keys, expected_keys,
            "{name}: unexpected header set — a wrapped line probably opened a header"
        );
    }
}

#[test]
fn a_rationale_that_wraps_onto_a_key_shaped_line_stays_one_value() {
    // The regression. `counter-width-and-rate` wraps onto a line that begins
    // `implementation-independence: a different timer …` — text that is exactly a bare
    // kebab-case key followed by a colon. Only the indentation distinguishes it.
    let (name, text) = corpus()
        .into_iter()
        .find(|(name, _)| name.ends_with("counter-width-and-rate.eadl"))
        .expect("the counter-width-and-rate case exists");
    let document = parse(&name, &text);
    let headers = document.comment_headers();
    let keys: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
    assert!(
        !keys.contains(&"implementation-independence"),
        "a wrapped line opened a spurious header: {keys:?}"
    );
    let rationale = headers
        .iter()
        .find(|(k, _)| k == "rationale")
        .map(|(_, v)| v.as_str())
        .expect("a rationale");
    assert!(
        rationale.contains("The test that settles it is implementation-independence:"),
        "the rationale was truncated at the wrap: {rationale}"
    );
    assert!(
        rationale.contains("describes the interface, not the part"),
        "the rationale lost its final sentence: {rationale}"
    );
}

#[test]
fn a_wrapped_rationale_is_one_value_not_several() {
    // The corpus wraps long rationales across many comment lines, several of which contain a
    // colon. Every one of them must land in the value, not open a new header.
    let (name, text) = corpus()
        .into_iter()
        .find(|(name, _)| name.ends_with("execution-bound.eadl"))
        .expect("the execution-bound case exists");
    let document = parse(&name, &text);
    let headers = document.comment_headers();
    let rationale = headers
        .iter()
        .find(|(k, _)| k == "rationale")
        .map(|(_, v)| v.as_str())
        .expect("a rationale");
    assert!(
        rationale.contains("AMBIGUOUS") && rationale.contains("observation about one"),
        "the rationale was truncated at a wrap: {rationale}"
    );
    let keys: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
    assert_eq!(
        keys,
        vec![
            "case",
            "verdict",
            "ambiguous",
            "tests",
            "failing-test",
            "rationale",
            "other-side"
        ],
        "unexpected header set — a wrapped line opened a header"
    );
}
