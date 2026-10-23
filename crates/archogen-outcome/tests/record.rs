//! `archogen-result/1` (leaf `M3.5.1.1`): one text per record, its shape frozen under its identifier, and no
//! corruption of a refusal that reads as an acceptance.
//!
//! ⛔ **The leg this format exists for.** The record's reader decides whether a plan is built. So for every status
//! that is not `ok`, single-byte substitutions, every deleted byte and every truncation of its record are read, and
//! each reads as a refusal of the reader's own or as a record that does not accept — never as one that does.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use archogen_digest::Digest;
use archogen_json::{self as json, Value};
use archogen_outcome::record::{
    Finding, Judged, Label, Producer, Record, Refusal, Severity, Subject, FORMAT,
};
use archogen_outcome::{Status, VOCABULARY};

const LIMIT: usize = 1 << 20;

fn producer() -> Producer {
    Producer {
        name: "archogen-plan-check".to_owned(),
        sha256: Digest::of(b"the checker's executable"),
    }
}

fn subject(judged: fn(Digest) -> Judged) -> Subject {
    Subject {
        program: Digest::of(b"the elaborated program"),
        judged: judged(Digest::of(b"the plan or the build")),
    }
}

fn label(message: &str) -> Label {
    Label {
        source: "examples/periodic-three/system.eadl".to_owned(),
        line: 12,
        column: 5,
        message: message.to_owned(),
    }
}

fn finding(severity: Severity) -> Finding {
    Finding {
        code: "requirement-unmet".to_owned(),
        severity,
        message: "`time.monotonic` needs `tick-rate`, and the plan binds a provider that \"offers\" none\n".to_owned(),
        primary: label("needed here"),
        secondary: vec![label("bound here")],
        repair: "bind a provider that offers `tick-rate`, or resolve again".to_owned(),
    }
}

/// A record of `status`: an `ok` with a warning, and any other with an error and a warning.
fn record(status: Status, judged: fn(Digest) -> Judged) -> Record {
    let findings = if status == Status::Ok {
        vec![finding(Severity::Warning)]
    } else {
        vec![finding(Severity::Error), finding(Severity::Warning)]
    };
    Record::new(
        producer(),
        subject(judged),
        "rt-static-up-v1".to_owned(),
        status,
        findings,
    )
    .expect("a consistent record")
}

/// The smallest refusal of `status`: one short error finding, nothing else to read, so the sweep below reads every
/// byte of a record whose every member is present.
fn minimal(status: Status) -> Record {
    let short = Label {
        source: "s".to_owned(),
        line: 1,
        column: 1,
        message: "m".to_owned(),
    };
    Record::new(
        producer(),
        subject(Judged::Plan),
        "p".to_owned(),
        status,
        vec![Finding {
            code: "c".to_owned(),
            severity: Severity::Error,
            message: "m".to_owned(),
            primary: short.clone(),
            secondary: vec![short],
            repair: "r".to_owned(),
        }],
    )
    .expect("a consistent refusal")
}

fn refusals() -> impl Iterator<Item = Status> {
    Status::ALL.iter().copied().filter(|s| *s != Status::Ok)
}

#[test]
fn every_record_reads_back_as_itself_and_writes_one_text() {
    for status in Status::ALL.iter().copied() {
        for judged in [Judged::Plan as fn(Digest) -> Judged, Judged::Build] {
            let record = record(status, judged);
            let text = record.write();
            let read = Record::read(text.as_bytes(), LIMIT)
                .unwrap_or_else(|r| panic!("`{}`: {r}\n{text}", status.slug()));
            assert_eq!(read, record, "`{}`", status.slug());
            assert_eq!(read.write(), text, "`{}`", status.slug());
            assert_eq!(read.accepts(), status == Status::Ok, "`{}`", status.slug());
            assert_eq!(read.vocabulary(), VOCABULARY);
        }
    }
}

/// The byte range of the one occurrence of `from` in `text`.
fn span(text: &[u8], from: &str) -> std::ops::Range<usize> {
    let from = from.as_bytes();
    let starts: Vec<usize> = (0..=text.len() - from.len())
        .filter(|&at| &text[at..at + from.len()] == from)
        .collect();
    assert_eq!(starts.len(), 1, "{:?} once", String::from_utf8_lossy(from));
    starts[0]..starts[0] + from.len()
}

/// ⛔ The refusal records of the vocabulary differ from each other only in their `status` and `exit` bytes, so one,
/// `not-established`'s, is damaged at every byte to every other value, and each of the others at every byte of the span
/// where they differ and of the error finding's severity, which an acceptance would also need changed; every record is
/// cut short and has a byte deleted at every place. More than 200 000 texts are read, each refused or not accepting.
#[test]
fn no_damage_to_a_refusal_reads_as_an_acceptance() {
    let mut reads = 0usize;
    for status in refusals() {
        let text = minimal(status).write().into_bytes();
        let mut check = |bytes: &[u8], what: &str| {
            reads += 1;
            if let Ok(read) = Record::read(bytes, LIMIT) {
                assert!(
                    !read.accepts(),
                    "`{}`'s record, {what}, reads as an acceptance:\n{}",
                    status.slug(),
                    String::from_utf8_lossy(bytes)
                );
            }
        };
        let swept: Vec<usize> = if status == Status::NotEstablished {
            (0..text.len()).collect()
        } else {
            let differing = span(
                &text,
                &format!(r#""status":"{}","exit":{},"#, status.slug(), status.code()),
            );
            differing
                .chain(span(&text, r#""severity":"error""#))
                .collect()
        };
        for at in swept {
            for byte in 0..=u8::MAX {
                if byte != text[at] {
                    let mut damaged = text.clone();
                    damaged[at] = byte;
                    check(&damaged, &format!("byte {at} made {byte:#04x}"));
                }
            }
        }
        for at in 0..text.len() {
            let mut shorter = text.clone();
            shorter.remove(at);
            check(&shorter, &format!("byte {at} deleted"));
            check(&text[..at], &format!("cut at {at}"));
        }
    }
    assert!(reads > 200_000, "the sweep read {reads} texts");
}

/// Replace the one occurrence of `from` in a record's text.
fn edited(text: &str, from: &str, to: &str) -> String {
    assert_eq!(text.matches(from).count(), 1, "`{from}` once in {text}");
    text.replacen(from, to, 1)
}

#[test]
fn a_status_flipped_alone_is_refused() {
    let text = record(Status::NotEstablished, Judged::Plan).write();
    // The slug made `ok`, the exit left `15`: the exit disagrees.
    let slug = edited(&text, r#""status":"not-established""#, r#""status":"ok""#);
    assert!(matches!(
        Record::read(slug.as_bytes(), LIMIT),
        Err(Refusal::Status(_))
    ));
    // Both made `ok`'s: the error finding disagrees.
    let both = edited(&slug, r#""exit":15"#, r#""exit":0"#);
    assert!(matches!(
        Record::read(both.as_bytes(), LIMIT),
        Err(Refusal::Inconsistent(_))
    ));
    // And a refusal holding no error finding is not written in the first place.
    assert!(matches!(
        Record::new(
            producer(),
            subject(Judged::Plan),
            "rt-static-up-v1".to_owned(),
            Status::NotEstablished,
            vec![]
        ),
        Err(Refusal::Inconsistent(_))
    ));
    assert!(matches!(
        Record::new(
            producer(),
            subject(Judged::Plan),
            "rt-static-up-v1".to_owned(),
            Status::Ok,
            vec![finding(Severity::Error)]
        ),
        Err(Refusal::Inconsistent(_))
    ));
}

#[test]
fn what_the_reader_refuses_it_names() {
    let text = record(Status::StaleInput, Judged::Build).write();
    let refused = |text: &str| Record::read(text.as_bytes(), LIMIT).expect_err(text);
    // A member named twice: the strict reader's own refusal.
    assert!(matches!(
        refused(&edited(
            &text,
            r#""status":"stale-input","#,
            r#""status":"stale-input","status":"ok","#
        )),
        Refusal::Json(_)
    ));
    // Another identifier, never guessed at.
    assert_eq!(
        refused(&edited(&text, FORMAT, "archogen-result/2")),
        Refusal::Format("archogen-result/2".to_owned())
    );
    // Another major of the vocabulary.
    let vocabulary = format!(r#""vocabulary":"{VOCABULARY}""#);
    assert!(matches!(
        refused(&edited(&text, &vocabulary, r#""vocabulary":"2.0""#)),
        Refusal::Vocabulary(_)
    ));
    // A status the vocabulary the record names did not yet hold: `stale-input` entered at `1.1`.
    assert!(matches!(
        refused(&edited(&text, &vocabulary, r#""vocabulary":"1.0""#)),
        Refusal::Status(_)
    ));
    // A slug no vocabulary holds.
    assert!(matches!(
        refused(&edited(
            &text,
            r#""status":"stale-input""#,
            r#""status":"stale""#
        )),
        Refusal::Status(_)
    ));
    // A member missing, one unexpected, the order changed.
    assert!(matches!(
        refused(&edited(&text, r#""profile":"rt-static-up-v1","#, "")),
        Refusal::Shape(_)
    ));
    assert!(matches!(
        refused(&edited(&text, r#""profile":"#, r#""note":"x","profile":"#)),
        Refusal::Shape(_)
    ));
    let order = edited(
        &text,
        r#""status":"stale-input","exit":3"#,
        r#""exit":3,"status":"stale-input""#,
    );
    assert!(matches!(refused(&order), Refusal::Shape(_)));
    // A digest spelled another way, a name outside its alphabet, a severity no finding has.
    let digest = Digest::of(b"the elaborated program");
    let program = format!(r#""program":"{digest}""#);
    let upper = format!(r#""program":"sha256:{}""#, digest.hex().to_uppercase());
    assert!(matches!(
        refused(&edited(&text, &program, &upper)),
        Refusal::Value(_)
    ));
    assert!(matches!(
        refused(&edited(
            &text,
            r#""name":"archogen-plan-check""#,
            r#""name":"Archogen""#
        )),
        Refusal::Value(_)
    ));
    assert!(matches!(
        refused(&text.replacen(r#""severity":"warning""#, r#""severity":"note""#, 1)),
        Refusal::Value(_)
    ));
    // Whitespace, an escape that need not be one, a missing newline: not the one text.
    assert_eq!(refused(&text.replacen(",", ", ", 1)), Refusal::NotCanonical);
    assert_eq!(
        refused(&text.replacen("rt-static", "\\u0072t-static", 1)),
        Refusal::NotCanonical
    );
    assert_eq!(refused(text.trim_end()), Refusal::NotCanonical);
    // Over the bound, before any of it is read.
    assert!(matches!(
        Record::read(text.as_bytes(), 16),
        Err(Refusal::Json(_))
    ));
}

fn goldens() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("tests/goldens")
}

/// Every key path of a JSON value with the kind of value at it: `findings[].primary.line: number`.
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
    // Both subjects, so both members a subject may hold are in the shape; findings with a secondary label.
    let mut paths = BTreeSet::new();
    for judged in [Judged::Plan as fn(Digest) -> Judged, Judged::Build] {
        let text = record(Status::NotEstablished, judged).write();
        shape(
            &json::read(text.trim_end().as_bytes(), LIMIT).expect("JSON"),
            "",
            &mut paths,
        );
    }
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
             ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-outcome --test record",
            path.display()
        ),
    }
}
