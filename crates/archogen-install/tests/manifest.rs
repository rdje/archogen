//! `archogen-install/1` (leaf `M3.5.1.3`): one text per manifest, each refusal named, the shape frozen under the
//! identifier.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use archogen_digest::Digest;
use archogen_install::{Build, Manifest, Program, Refusal, FORMAT};
use archogen_json::{self as json, Value};

const LIMIT: usize = 1 << 20;

fn build() -> Build {
    Build {
        commit: "263e1533ff447ef79fd73772f18e3ebd02d75d09".to_owned(),
        tree: "68c537aa79b4c2d151c3a266c9cf0d5c3e731e31".to_owned(),
        rustc: "rustc 1.95.0 (59807616e 2026-04-14)\nbinary: rustc\nhost: aarch64-apple-darwin"
            .to_owned(),
        host: "aarch64-apple-darwin".to_owned(),
    }
}

fn program(name: &str, role: &str) -> Program {
    Program {
        name: name.to_owned(),
        role: role.to_owned(),
        sha256: Digest::of(name.as_bytes()),
    }
}

fn front() -> Program {
    program("archogen", "generator")
}

fn siblings() -> Vec<Program> {
    vec![
        program("archogen-analyze", "scheduling-checker"),
        program("archogen-plan-check", "configuration-checker"),
    ]
}

fn manifest(siblings: Vec<Program>) -> Manifest {
    Manifest::new(
        "/opt/archogen/bin".to_owned(),
        build(),
        Digest::of(b"trust-dependencies.json"),
        front(),
        siblings,
    )
    .expect("a whole manifest")
}

#[test]
fn a_manifest_reads_back_as_itself_and_writes_one_text() {
    for siblings in [vec![], siblings()] {
        let manifest = manifest(siblings);
        let text = manifest.write();
        let read = Manifest::read(text.as_bytes(), LIMIT).unwrap_or_else(|r| panic!("{r}\n{text}"));
        assert_eq!(read, manifest);
        assert_eq!(read.write(), text);
    }
    let manifest = manifest(siblings());
    assert_eq!(
        manifest
            .sibling("archogen-plan-check")
            .map(|p| p.role.as_str()),
        Some("configuration-checker")
    );
    assert!(
        manifest.sibling("archogen").is_none(),
        "the front is no sibling"
    );
}

#[test]
fn a_manifest_that_describes_no_one_install_is_not_made() {
    let make = |directory: &str, front: Program, siblings: Vec<Program>| {
        Manifest::new(
            directory.to_owned(),
            build(),
            Digest::of(b"i"),
            front,
            siblings,
        )
    };
    let value = |r: Result<Manifest, Refusal>| matches!(r, Err(Refusal::Value(_)));
    assert!(value(make("bin", front(), vec![])), "a relative directory");
    assert!(
        value(make(
            "/b",
            program("archogen", "scheduling-checker"),
            vec![]
        )),
        "a front not the generator's"
    );
    assert!(
        value(make("/b", front(), vec![program("other", "generator")])),
        "a second generator"
    );
    assert!(
        value(make("/b", front(), vec![program("archogen", "x")])),
        "a sibling of the front's name"
    );
    let mut reversed = siblings();
    reversed.reverse();
    assert!(
        value(make("/b", front(), reversed)),
        "siblings out of order"
    );
    let twice = vec![program("a", "x"), program("a", "y")];
    assert!(
        value(make("/b", front(), twice)),
        "two programs of one name"
    );
    assert!(
        value(make("/b", program("Archogen", "generator"), vec![])),
        "a name outside its alphabet"
    );
    let mut odd = build();
    odd.commit = "263E1533".to_owned();
    assert!(matches!(
        Manifest::new("/b".to_owned(), odd, Digest::of(b"i"), front(), vec![]),
        Err(Refusal::Value(_))
    ));
}

/// Replace the one occurrence of `from` in a manifest's text.
fn edited(text: &str, from: &str, to: &str) -> String {
    assert_eq!(text.matches(from).count(), 1, "`{from}` once in {text}");
    text.replacen(from, to, 1)
}

#[test]
fn what_the_reader_refuses_it_names() {
    let text = manifest(siblings()).write();
    let refused = |text: &str| Manifest::read(text.as_bytes(), LIMIT).expect_err(text);
    assert_eq!(
        refused(&edited(&text, FORMAT, "archogen-install/2")),
        Refusal::Format("archogen-install/2".to_owned())
    );
    assert!(matches!(
        refused(&edited(&text, r#""directory":"/opt/archogen/bin","#, "")),
        Refusal::Shape(_)
    ));
    assert!(matches!(
        refused(&edited(
            &text,
            r#""directory":"/opt/archogen/bin","#,
            r#""directory":"/opt/archogen/bin","directory":"/tmp","#
        )),
        Refusal::Json(_)
    ));
    let inventory = format!(
        r#""inventory":"{}""#,
        Digest::of(b"trust-dependencies.json")
    );
    assert!(matches!(
        refused(&edited(&text, &inventory, r#""inventory":"sha256:00""#)),
        Refusal::Value(_)
    ));
    assert!(matches!(
        refused(&edited(
            &text,
            r#""directory":"/opt/archogen/bin""#,
            r#""directory":"opt""#
        )),
        Refusal::Value(_)
    ));
    assert_eq!(refused(&text.replacen(",", ", ", 1)), Refusal::NotCanonical);
    assert_eq!(refused(text.trim_end()), Refusal::NotCanonical);
    assert!(matches!(
        Manifest::read(text.as_bytes(), 16),
        Err(Refusal::Json(_))
    ));
}

fn goldens() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens")
}

/// Every key path of a JSON value with the kind of value at it: `siblings[].sha256: string`.
fn shape(value: &Value, path: &str, paths: &mut BTreeSet<String>) {
    let kind = match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Number(_) => "number",
        Value::String(_) => "string",
        Value::Array(items) => {
            for item in items {
                shape(item, &format!("{path}[]"), paths);
            }
            "array"
        }
        Value::Object(members) => {
            for (key, member) in members {
                let inner = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                shape(member, &inner, paths);
            }
            "object"
        }
    };
    if !path.is_empty() && !matches!(value, Value::Object(_)) {
        paths.insert(format!("{path}: {kind}"));
    }
}

#[test]
fn the_shape_of_the_format_is_frozen_under_its_identifier() {
    let text = manifest(siblings()).write();
    let mut paths = BTreeSet::new();
    shape(
        &json::read(text.trim_end().as_bytes(), LIMIT).expect("JSON"),
        "",
        &mut paths,
    );
    let actual = std::iter::once(FORMAT.to_owned())
        .chain(paths)
        .map(|line| line + "\n")
        .collect::<String>();
    let path = goldens().join(format!("{}.golden", FORMAT.replace('/', "-")));
    match fs::read_to_string(&path) {
        Ok(frozen) => assert!(
            frozen == actual,
            "the shape of `{FORMAT}` has changed. A golden is never rewritten: move `FORMAT` (and the register, \
             docs/book/src/versions.md), then bless the new identifier.\n--- frozen\n{frozen}\n--- now\n{actual}"
        ),
        Err(_) if std::env::var_os("ARCHOGEN_BLESS_FORMATS").is_some() => {
            fs::create_dir_all(goldens()).expect("the goldens directory is creatable");
            fs::write(&path, actual).expect("the golden is writable");
        }
        Err(_) => panic!(
            "no golden for `{FORMAT}` at {}. A new identifier gets one deliberately: \
             ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-install --test manifest",
            path.display()
        ),
    }
}
