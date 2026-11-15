//! Format `/2` on the hosted playground, as `lower_hosted` writes it (`docs/decisions/decision_runtime-plan-format-2.md`
//! §1, "The hosted playground", and §3; leaf `M4.1.7.6.4`), held to `/1` over every system of the corpus: what a hosted
//! plan holds "is `/1`'s, with these and no others", the record's own table of a hosted plan's lines, read here, and §3's
//! owed lines. A system with an alarm is `M4.1.7.6.5`'s, and is refused until then.

mod common;

use std::collections::BTreeSet;
use std::fs;

use archogen_digest::Digest;
use archogen_plan::Generation;
use common::{lowered_both, root};

const RECORD: &str = include_str!("../../../docs/decisions/decision_runtime-plan-format-2.md");

/// The emitter a test names: no `archogen-emit/N` is stated yet (`RPF-H3`, `M4.2`), so a test gives one.
const EMITTER: &str = "archogen-emit/1";

/// The tool's catalog as it stands, no record in it (`M2.7.4.5`): its digest is of the empty text.
fn generation() -> Generation<'static> {
    Generation {
        emitter: EMITTER,
        records: &[],
    }
}

/// The heads a hosted plan writes beyond `/1`'s, from the record's table after `<!-- machine-read: hosted-lines -->`.
fn the_hosted_lines() -> BTreeSet<String> {
    let after = RECORD
        .split("<!-- machine-read: hosted-lines -->")
        .nth(1)
        .expect("the record marks what a hosted plan writes");
    after
        .lines()
        .skip_while(|line| !line.starts_with('|'))
        .take_while(|line| line.starts_with('|'))
        .skip(2)
        .flat_map(|row| {
            row.trim_matches('|')
                .split('|')
                .next()
                .expect("a first cell")
                .split(',')
                .map(|head| head.trim().trim_matches('`').to_owned())
                .collect::<Vec<_>>()
        })
        .collect()
}

/// A line's head.
fn head(line: &str) -> &str {
    archogen_plan::identity::head(line)
}

/// The facts §3 adds to what a hosted plan owes of a system with no alarm and no application's file: each task's
/// `node-calls` and `node-calls-category` and the system's `initialisation`, the application's, and a node a job calls
/// its `primitives-out-of-line` and `masking-balanced`, the realization's.
const OWED_BY_TWO: [(&str, &str); 5] = [
    ("node-calls", "application"),
    ("node-calls-category", "application"),
    ("initialisation", "application"),
    ("primitives-out-of-line", "realization"),
    ("masking-balanced", "realization"),
];

/// Every system of every corpus description `/1` lowers is lowered as `/2` hosted too, and the two texts differ by the
/// hosted table's rows alone: `(format …)` itself, the lines whose heads the table lists beyond `/1`'s and §3's owed
/// lines; every line of `/1` stands in `/2` as `/1` writes it. A refusal of `/1` is `/2`'s, by its code, placement
/// memory's repair naming a target; and a system with an alarm is refused by `/2` until `M4.1.7.6.5`.
#[test]
fn every_hosted_plan_is_one_s_with_the_table_s_lines() {
    let hosted = the_hosted_lines();
    let added: BTreeSet<&str> = ["target", "emitter", "catalog", "epoch", "region"].into();
    for head in &added {
        assert!(
            hosted.contains(*head),
            "the record's table lists `({head} …)`"
        );
    }
    let dir = root().join("crates/archogen-plan/tests/corpus");
    let mut fixtures: Vec<_> = fs::read_dir(&dir)
        .expect("the corpus")
        .map(|entry| entry.expect("an entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "eadl"))
        .collect();
    fixtures.sort();
    let (mut plans, mut alarmed, mut refused, mut regions) = (0, 0, 0, 0);
    for path in &fixtures {
        let text = fs::read_to_string(path).expect("a fixture");
        let named: Vec<Option<String>> = text
            .lines()
            .filter_map(|line| line.trim_start().strip_prefix("(defsystem "))
            .map(|rest| rest.split([' ', ')']).next().map(str::to_owned))
            .collect();
        let systems = if named.is_empty() { vec![None] } else { named };
        for system in &systems {
            let system = system.as_deref();
            let Some((one, two)) = lowered_both(&text, system, &generation()) else {
                continue;
            };
            let at = format!("{} {system:?}", path.display());
            match (one, two) {
                (Ok(one), Err(refusal)) => {
                    assert!(one.contains("(timer-route "), "{at}: {refusal}");
                    assert_eq!(refusal.code, "tool-failure", "{at}");
                    assert!(refusal.message.contains("M4.1.7.6.5"), "{at}: {refusal}");
                    alarmed += 1;
                }
                (Ok(one), Ok(two)) => {
                    assert!(!one.contains("(timer-route "), "{at}");
                    let one_lines: Vec<&str> = one.lines().collect();
                    let mut kept = Vec::new();
                    for line in two.lines() {
                        let h = head(line);
                        if h == "format" {
                            assert_eq!(line, "(format \"archogen-runtime-plan/2\")", "{at}");
                            kept.push("(format \"archogen-runtime-plan/1\")");
                        } else if added.contains(h) {
                            regions += usize::from(h == "region");
                        } else if h == "owed" && !one_lines.contains(&line) {
                            assert!(
                                OWED_BY_TWO.iter().any(|(fact, owner)| {
                                    line.ends_with(&format!(" {fact} (by {owner}))"))
                                }),
                                "{at}: `/2` owes what §3 does not add: {line}"
                            );
                        } else {
                            kept.push(line);
                        }
                    }
                    assert_eq!(
                        kept, one_lines,
                        "{at}: `/2`'s lines but the table's are `/1`'s"
                    );
                    plans += 1;
                }
                (Err(one), Err(two)) => {
                    assert_eq!(one.code, two.code, "{at}");
                    if one.message.contains("wait for `/2`") {
                        assert!(two.message.contains("name the target"), "{at}: {two}");
                    } else {
                        assert_eq!(one.message, two.message, "{at}");
                    }
                    refused += 1;
                }
                (Err(one), Ok(_)) => panic!("{at}: `/1` refuses what `/2` lowers: {one}"),
            }
        }
    }
    // Each kind of outcome is among them, so none is held by having no case; and the counts are pinned, so a fixture
    // that stops lowering, or starts, is seen.
    assert_eq!((plans, alarmed, refused, regions), (81, 61, 34, 3));
}

/// The S0 heartbeat as `/2` hosted writes it: the table's lines in §3's places and §3's owed lines, each named.
#[test]
fn the_heartbeat_s_hosted_plan_has_its_lines_in_their_places() {
    let text =
        fs::read_to_string(root().join("examples/s0-heartbeat/system.eadl")).expect("the example");
    let (_, two) = lowered_both(&text, None, &generation()).expect("it resolves");
    let two = two.expect("it lowers");
    let lines: Vec<&str> = two.lines().collect();
    let empty = Digest::of(b"").hex();
    assert_eq!(
        lines[..9],
        [
            "(format \"archogen-runtime-plan/2\")",
            "(profile rt-static-up-v1)",
            "(system heartbeat)",
            "(target hosted-playground)",
            lines[4],
            lines[5],
            "(adapters adapters/1)",
            "(emitter archogen-emit/1)",
            &format!("(catalog \"{empty}\" none)"),
        ]
    );
    assert!(lines[4].starts_with("(program \"") && lines[5].starts_with("(assignment \""));
    let epoch = lines
        .iter()
        .position(|line| *line == "(epoch simulator)")
        .expect("the epoch");
    assert!(
        lines[epoch + 1].starts_with("(timer simulator "),
        "the epoch stands before the timer"
    );
    for owed in [
        "(owed (task heartbeat beat) node-calls (by application))",
        "(owed (task heartbeat beat) node-calls-category (by application))",
        "(owed (task heartbeat chime) node-calls (by application))",
        "(owed (task heartbeat chime) node-calls-category (by application))",
        "(owed heartbeat initialisation (by application))",
        "(owed console.uart masking-balanced (by realization))",
        "(owed console.uart primitives-out-of-line (by realization))",
        "(owed console.write masking-balanced (by realization))",
        "(owed console.write primitives-out-of-line (by realization))",
    ] {
        assert!(lines.contains(&owed), "{owed}");
    }
    assert_eq!(lines.last(), Some(&"(complete no)"));
    assert!(
        !two.contains("(region "),
        "the heartbeat's hardware states no region"
    );
    // Its code identity is taken, and moves with the emitter and the catalog, which generate its code (§1).
    let identity = archogen_plan::identity::code_identity(&two).expect("an identity");
    let other_emitter = lowered_both(
        &text,
        None,
        &Generation {
            emitter: "archogen-emit/2",
            records: &[],
        },
    )
    .and_then(|(_, two)| two.ok())
    .expect("it lowers");
    assert_ne!(
        archogen_plan::identity::code_identity(&other_emitter).expect("an identity"),
        identity
    );
    let records = [("port.riscv".to_owned(), Digest::of(b"port"))];
    let other_catalog = lowered_both(
        &text,
        None,
        &Generation {
            emitter: EMITTER,
            records: &records,
        },
    )
    .and_then(|(_, two)| two.ok())
    .expect("it lowers");
    assert_ne!(
        archogen_plan::identity::code_identity(&other_catalog).expect("an identity"),
        identity
    );
}

/// A region the description's hardware states is written with its use and the requesters bound to its provider, by base,
/// then name, then provider (§3, "Values" and "Orders"; §5, "Which regions").
#[test]
fn a_hosted_plan_writes_each_region_of_its_hardware() {
    let text =
        fs::read_to_string(root().join("crates/archogen-plan/tests/corpus/region-hosted.eadl"))
            .expect("the fixture");
    let (_, two) = lowered_both(&text, None, &generation()).expect("it resolves");
    let two = two.expect("it lowers");
    let regions: Vec<&str> = two
        .lines()
        .filter(|line| line.starts_with("(region "))
        .collect();
    assert_eq!(
        regions,
        [
            "(region rom host.p (base 0) (size 4096) (use reserved) (requesters))",
            "(region uart.regs console.uart (base 268435456) (size 1024) (use device) (requesters console.write))",
            "(region device.timer host.p (base 536870912) (size unsized) (use device) (requesters))",
        ]
    );
}

/// An emitter's version that is no single atom is refused, never written into a kept line.
#[test]
fn an_emitter_that_is_no_atom_is_refused() {
    let text =
        fs::read_to_string(root().join("examples/s0-heartbeat/system.eadl")).expect("the example");
    for emitter in ["", "archogen emit", "(x)"] {
        let (_, two) = lowered_both(
            &text,
            None,
            &Generation {
                emitter,
                records: &[],
            },
        )
        .expect("it resolves");
        let refusal = two.expect_err("refused");
        assert_eq!(refusal.code, "tool-failure", "{emitter:?}");
    }
}
