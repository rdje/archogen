//! Leaf `S0.2`: the S0 corpus reads, and a corrupted S0 corpus is reported *where it broke*.
//!
//! `S0.2` was written expecting to build "a minimal S-expression reader with source spans,
//! sufficient for the S0 fixture only". It never needed to: `M1` built the whole frontend
//! first, and §12 S0's own retirement clause anticipates exactly this — "the prototype
//! implementation can be discarded or replaced as semantics settle". What the leaf still owes is
//! its **acceptance**, and a leaf closed because the work was done elsewhere is only honest if
//! the acceptance is re-verified against what actually exists rather than assumed.
//!
//! So this file asserts the leaf's two clauses against `eadl-front`:
//!
//! | Clause | Asserted by |
//! |---|---|
//! | the three fixtures parse | [`all_three_fixtures_read_with_no_diagnostics`] |
//! | a malformed fixture reports a span-localized error | the three tests after it |
//!
//! # "Span-localized" is a claim about *where*, and it needs the negative case
//!
//! A reader that reported every syntax error at end-of-input would pass a test that only checked
//! "an error was produced", and would be useless on a 2 000-line description. So the sharp
//! assertion here is [`a_stray_close_is_reported_where_it_appears_not_at_the_end`]: a corruption
//! is injected at a *computed* line near the top of the file, and the diagnostic must name that
//! line. Nothing is hardcoded — moving a declaration in the fixture moves the expected line with
//! it.
//!
//! Every corruption is made **in memory**, from the committed fixture. A broken description
//! checked into the corpus would be one more file for a future reader to mistake for an
//! intended case.

use std::path::{Path, PathBuf};

use archogen_cli::{run, Status};
use eadl_front::{read, Diagnostic, SourceMap};

/// The three committed descriptions of the S0 corpus.
const FIXTURES: &[&str] = &[
    "examples/s0-heartbeat/system.eadl",
    "examples/s0-heartbeat/system-changed.eadl",
    "examples/s0-heartbeat/system-unsupported.eadl",
];

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn fixture_text(relative: &str) -> String {
    let path = repo_root().join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

/// Read `text` and return its diagnostics together with the map that resolves their spans.
fn read_text(name: &str, text: String) -> (Vec<Diagnostic>, SourceMap) {
    let mut sources = SourceMap::new();
    let id = sources.add(name, text).expect("small");
    let (_, diagnostics) = read(&sources, id);
    (diagnostics.items().to_vec(), sources)
}

/// The 1-based line a span starts on.
fn line_of(sources: &SourceMap, span: eadl_front::Span) -> u32 {
    sources
        .get(span.source)
        .expect("the span's source is in the map")
        .position(span.start)
        .line
}

/// The 1-based line of the first line that starts with `prefix`.
fn line_starting_with(text: &str, prefix: &str) -> u32 {
    let index = text
        .lines()
        .position(|line| line.starts_with(prefix))
        .unwrap_or_else(|| panic!("no line starts with `{prefix}`"));
    u32::try_from(index + 1).expect("fixtures are short")
}

#[test]
fn all_three_fixtures_read_with_no_diagnostics() {
    for relative in FIXTURES {
        let (diagnostics, sources) = read_text(relative, fixture_text(relative));
        assert!(
            diagnostics.is_empty(),
            "{relative} did not read cleanly:\n{}",
            diagnostics
                .iter()
                .map(|d| d.render(&sources))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

#[test]
fn an_unclosed_list_names_the_declaration_that_opened_it() {
    // Drop the final `)`. The *primary* label is necessarily at end-of-input — that is where the
    // reader ran out — so the useful half is the secondary one, which must point back at the
    // `(defsystem` that is still open. An error that only said "unexpected end of file" would
    // leave the author searching the whole description by hand.
    let text = fixture_text(FIXTURES[0]);
    let opened_at = line_starting_with(&text, "(defsystem");
    let truncated = text
        .trim_end()
        .strip_suffix(')')
        .expect("ends with `)`")
        .to_string();

    let (diagnostics, sources) = read_text("truncated", truncated);
    let diagnostic = diagnostics.first().expect("a truncated list is an error");
    assert_eq!(diagnostic.code, "read-unclosed-list");
    let secondary = diagnostic
        .secondary
        .first()
        .expect("the opening paren is reported too");
    assert_eq!(
        line_of(&sources, secondary.span),
        opened_at,
        "the secondary label must sit on the `(defsystem` line:\n{}",
        diagnostic.render(&sources)
    );
    assert!(
        !diagnostic.repair.is_empty(),
        "§5.5 requires a concrete repair direction on every diagnostic"
    );
}

#[test]
fn a_stray_close_is_reported_where_it_appears_not_at_the_end() {
    // ⭐ The assertion that makes "span-localized" mean something. The corruption goes in near
    // the TOP of the file and the diagnostic must name that line — not the last line, and not
    // the line the reader happened to stop on. The expected line is computed from the fixture,
    // so rearranging the declarations moves it automatically.
    let text = fixture_text(FIXTURES[0]);
    let mut lines: Vec<String> = text.lines().map(String::from).collect();
    let at = line_starting_with(&text, "(defplatform");
    lines.insert((at as usize) - 1, ")".to_string());
    let corrupted = lines.join("\n");

    let (diagnostics, sources) = read_text("stray-close", corrupted);
    let diagnostic = diagnostics.first().expect("a stray `)` is an error");
    assert_eq!(diagnostic.code, "read-unexpected-close");
    assert_eq!(
        line_of(&sources, diagnostic.primary.span),
        at,
        "the caret must sit on the stray `)`, not at end of input:\n{}",
        diagnostic.render(&sources)
    );
    // And the reader keeps going: the declarations after the corruption are still read, so one
    // typo does not hide every other problem in the file.
    assert_eq!(
        diagnostics.len(),
        1,
        "recovery should report the one real problem:\n{}",
        diagnostics
            .iter()
            .map(|d| d.render(&sources))
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn a_malformed_fixture_reaches_the_user_as_invalid_description() {
    // The whole path, not just the reader: a corrupted S0 description must come back to a user
    // as §5.5's `invalid-description` with the stable exit code scripts branch on.
    //
    // The corrupted copy is written under `CARGO_TARGET_TMPDIR` — inside `target/`, on the
    // repository's own volume and derived from it, never `/tmp`.
    let text = fixture_text(FIXTURES[0]);
    let truncated = text
        .trim_end()
        .strip_suffix(')')
        .expect("ends with `)`")
        .to_string();
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("s0-reader");
    std::fs::create_dir_all(&dir).expect("the target directory is writable");
    let path = dir.join("malformed.eadl");
    std::fs::write(&path, &truncated).expect("write the corrupted copy");

    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(
        ["check", &path.display().to_string()].map(String::from),
        &mut out,
        &mut err,
    );
    let rendered = String::from_utf8(err).expect("utf-8 stderr");

    assert_eq!(status, Status::InvalidDescription);
    assert_eq!(status.code(), 10);
    assert!(rendered.contains("read-unclosed-list"), "{rendered}");
    assert!(rendered.contains("opened here"), "{rendered}");
    assert!(rendered.contains("hint:"), "{rendered}");
    assert!(
        String::from_utf8(out).expect("utf-8 stdout").is_empty(),
        "a refused description must not also report acceptance"
    );
}
