//! `archogen-provenance/1`'s shape cannot change without its identifier moving (leaf `PROGRAM.6.3`, `ROADMAP.md` §15).
//!
//! A consumer that meets an identifier it does not know must refuse rather than guess (the provenance's `FORMAT` constant says so), so
//! the identifier is the promise: every artifact labelled `archogen-provenance/1` has one shape. This builds the
//! S0 base fixture, reads the provenance it writes, and freezes its **shape** — every key path with the kind of
//! value at it — in a golden named after the identifier. The values change with the description and are not
//! frozen; a key added, removed, renamed or re-typed is a new shape. The golden for an identifier is **never
//! rewritten**: a shape change fails here until `FORMAT` moves, and the new identifier gets its golden only
//! deliberately (`ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-cli --test format_golden`).
//!
//! ⛔ Deliberately not a JSON parser, as in `s0_provenance.rs`: a lexer that walks strings, numbers and brackets and
//! keeps a stack of the keys it is inside. The set of key paths does not depend on how many records a build has.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use archogen_cli::{run, Status};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn goldens() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens")
}

/// Compare `actual` with the golden for `identifier`; create one only when blessing, and never overwrite one.
fn check_golden(identifier: &str, actual: &str) {
    let path = goldens().join(format!("{}.golden", identifier.replace('/', "-")));
    match fs::read_to_string(&path) {
        Ok(frozen) => assert!(
            frozen == actual,
            "the shape of `{identifier}` has changed. A golden is never rewritten: move `FORMAT` (and the register, \
             docs/book/src/versions.md), then bless the new identifier.\n--- frozen\n{frozen}\n--- now\n{actual}"
        ),
        Err(_) if std::env::var_os("ARCHOGEN_BLESS_FORMATS").is_some() => {
            fs::create_dir_all(goldens()).expect("the goldens directory is creatable");
            fs::write(&path, actual).expect("the golden is writable");
        }
        Err(_) => panic!(
            "no golden for `{identifier}` at {}. A new identifier gets one deliberately: \
             ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-cli --test format_golden",
            path.display()
        ),
    }
}

/// Every key path in a JSON text with the kind of value at it: `records[].source.line: number`.
fn shape(text: &str) -> BTreeSet<String> {
    let bytes = text.as_bytes();
    let mut paths = BTreeSet::new();
    // The path of the container being read; `[]` marks an array element.
    let mut stack: Vec<String> = Vec::new();
    let mut pending_key: Option<String> = None;
    let mut at = 0;
    // Empty frames — the root object, and each object that is an array element — add nothing to the path.
    let path_with = |stack: &[String], key: &str| {
        let mut parts: Vec<&str> = stack
            .iter()
            .map(String::as_str)
            .filter(|frame| !frame.is_empty())
            .collect();
        if !key.is_empty() {
            parts.push(key);
        }
        parts.join(".").replace(".[]", "[]")
    };
    while at < bytes.len() {
        let byte = bytes[at];
        match byte {
            b'"' => {
                let start = at + 1;
                at += 1;
                while at < bytes.len() && bytes[at] != b'"' {
                    at += if bytes[at] == b'\\' { 2 } else { 1 };
                }
                let token = &text[start..at.min(text.len())];
                at += 1;
                let mut next = at;
                while next < bytes.len() && bytes[next].is_ascii_whitespace() {
                    next += 1;
                }
                if next < bytes.len() && bytes[next] == b':' {
                    pending_key = Some(token.to_string());
                    at = next + 1;
                } else {
                    let key = pending_key.take().unwrap_or_default();
                    paths.insert(format!("{}: string", path_with(&stack, &key)));
                }
                continue;
            }
            b'{' | b'[' => {
                let key = pending_key.take().unwrap_or_default();
                let mut frame = key;
                if byte == b'[' {
                    frame.push_str("[]");
                }
                stack.push(frame);
            }
            b'}' | b']' => {
                stack.pop();
            }
            b'-' | b'0'..=b'9' | b't' | b'f' | b'n' => {
                let kind = match byte {
                    b't' | b'f' => "bool",
                    b'n' => "null",
                    _ => "number",
                };
                while at < bytes.len() && !matches!(bytes[at], b',' | b'}' | b']' | b'\n' | b' ') {
                    at += 1;
                }
                let key = pending_key.take().unwrap_or_default();
                paths.insert(format!("{}: {kind}", path_with(&stack, &key)));
                continue;
            }
            _ => {}
        }
        at += 1;
    }
    paths
}

#[test]
fn the_shape_of_the_provenance_format_is_frozen_under_its_identifier() {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("format-golden");
    let _ = fs::remove_dir_all(&dir);
    let source = repo_root().join("examples/s0-heartbeat/system.eadl");
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(
        [
            "build",
            &source.display().to_string(),
            "--out",
            &dir.display().to_string(),
        ]
        .map(String::from),
        &mut out,
        &mut err,
    );
    assert_eq!(status, Status::Ok, "{}", String::from_utf8_lossy(&err));
    let text =
        fs::read_to_string(dir.join("provenance.json")).expect("the build writes provenance.json");
    // The identifier is read from the artifact, which carries it — not imported from the S0 prototype, which
    // `S0-RETIREMENT` keeps free of new consumers. What is pinned is what the artifact says about itself.
    let identifier = text
        .lines()
        .find_map(|line| line.trim().strip_prefix("\"format\": \""))
        .and_then(|rest| rest.split('"').next())
        .expect("the artifact carries its format identifier");
    let paths = shape(&text);
    assert!(
        paths.len() >= 10,
        "the lexer found only {} key paths — it is broken, not the format",
        paths.len()
    );
    let mut actual = format!("{identifier}\n");
    for path in &paths {
        actual.push_str(path);
        actual.push('\n');
    }
    check_golden(identifier, &actual);
}

#[test]
fn the_shape_lexer_sees_keys_kinds_and_array_elements_but_not_counts() {
    let one = shape(r#"{"a": 1, "b": {"c": "x"}, "r": [ {"k": true} ]}"#);
    let two = shape(r#"{"a": 2, "b": {"c": "y \" z"}, "r": [ {"k": false}, {"k": true} ]}"#);
    assert_eq!(one, two, "values and element counts are not the shape");
    let expected: BTreeSet<String> = ["a: number", "b.c: string", "r[].k: bool"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    assert_eq!(one, expected);
    let renamed = shape(r#"{"a": 1, "b": {"d": "x"}, "r": [ {"k": true} ]}"#);
    assert_ne!(one, renamed, "a renamed key is a new shape");
    let retyped = shape(r#"{"a": "1", "b": {"c": "x"}, "r": [ {"k": true} ]}"#);
    assert_ne!(one, retyped, "a re-typed value is a new shape");
}
