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

/// The facts §3 adds to what a hosted plan owes, beside its alarms' and clocks', where no application's file is given:
/// each task's `node-calls` and `node-calls-category` and the system's `initialisation`, the application's, a node a job
/// calls its `primitives-out-of-line` and `masking-balanced`, the realization's, and `RA-H7`'s `deadlines-within-reach`.
const OWED_BY_TWO: [(&str, &str); 6] = [
    ("node-calls", "application"),
    ("node-calls-category", "application"),
    ("initialisation", "application"),
    ("primitives-out-of-line", "realization"),
    ("masking-balanced", "realization"),
    // `RA-H7`: of a task writing an alarm on the counter itself that is no wake, no file naming it (leaf `M4.1.7.6.6`).
    ("deadlines-within-reach", "application"),
];

/// The items of the clause `(name …)` in `line`, each an atom or a list as written.
fn items(line: &str, name: &str) -> BTreeSet<String> {
    let from = line.find(&format!("({name}")).expect("the clause") + name.len() + 1;
    let mut out = BTreeSet::new();
    let (mut depth, mut start) = (0_usize, None::<usize>);
    for (i, c) in line[from..].char_indices() {
        let at = from + i;
        match c {
            '(' => {
                if depth == 0 {
                    start = Some(at);
                }
                depth += 1;
            }
            ')' if depth == 0 => {
                if let Some(s) = start.take() {
                    out.insert(line[s..at].to_owned());
                }
                return out;
            }
            ')' => {
                depth -= 1;
                if depth == 0 {
                    out.insert(line[start.take().expect("an open item")..=at].to_owned());
                }
            }
            ' ' if depth == 0 => {
                if let Some(s) = start.take() {
                    out.insert(line[s..at].to_owned());
                }
            }
            _ if depth == 0 && start.is_none() => start = Some(at),
            _ => {}
        }
    }
    out
}

/// Every system of every corpus description `/1` lowers is lowered as `/2` hosted too, and the two texts differ by the
/// hosted table's rows alone: `(format …)` itself, the lines whose heads the table lists beyond `/1`'s, §3's owed lines,
/// and what an alarm changes (leaf `M4.1.7.6.5`): each `/1` timer route stands as one route for each alarm on its device,
/// whose requesters and costs together are `/1`'s; a context line differs in its interrupts alone; one route obligation
/// stands for each alarm where `/1` writes one for the device; the enabled line follows the routes' ranks. A refusal of
/// `/1` is `/2`'s, by its code, placement memory's repair naming a target.
#[test]
fn every_hosted_plan_is_one_s_with_the_table_s_lines() {
    let hosted = the_hosted_lines();
    let added: BTreeSet<&str> = [
        "target", "emitter", "catalog", "epoch", "region", "alarm", "clock",
    ]
    .into();
    for head in &added {
        assert!(
            hosted.contains(*head),
            "the record's table lists `({head} …)`"
        );
    }
    // Heads an alarm changes, compared below by what each must keep of `/1`'s.
    let changed: BTreeSet<&str> = ["route", "enabled", "context", "obligation", "owed"].into();
    let dir = root().join("crates/archogen-plan/tests/corpus");
    let mut fixtures: Vec<_> = fs::read_dir(&dir)
        .expect("the corpus")
        .map(|entry| entry.expect("an entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "eadl"))
        .collect();
    fixtures.sort();
    let (mut plans, mut alarmed, mut alarms, mut refused, mut regions, mut clocks) =
        (0, 0, 0, 0, 0, 0);
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
                (Ok(one), Ok(two)) => {
                    let one_lines: Vec<&str> = one.lines().collect();
                    let two_lines: Vec<&str> = two.lines().collect();
                    let of = |lines: &[&str], wanted: &str| -> Vec<String> {
                        lines
                            .iter()
                            .filter(|line| head(line) == wanted)
                            .map(|line| (*line).to_owned())
                            .collect()
                    };
                    // Lines no alarm changes: `/1`'s, but the table's added ones and the format's own.
                    let mut kept = Vec::new();
                    for line in &two_lines {
                        let h = head(line);
                        if h == "format" {
                            assert_eq!(*line, "(format \"archogen-runtime-plan/2\")", "{at}");
                            kept.push("(format \"archogen-runtime-plan/1\")");
                        } else if added.contains(h) {
                            regions += usize::from(h == "region");
                            clocks += usize::from(h == "clock");
                        } else if !changed.contains(h) {
                            kept.push(line);
                        }
                    }
                    let one_kept: Vec<&str> = one_lines
                        .iter()
                        .copied()
                        .filter(|line| !changed.contains(head(line)))
                        .collect();
                    assert_eq!(
                        kept, one_kept,
                        "{at}: `/2`'s lines but the table's are `/1`'s"
                    );
                    let alarm_lines = of(&two_lines, "alarm");
                    alarms += alarm_lines.len();
                    alarmed += usize::from(!alarm_lines.is_empty());
                    // Each alarm's node and device, from its line.
                    let nodes: Vec<(String, String)> = alarm_lines
                        .iter()
                        .map(|line| {
                            let rest = line.strip_prefix("(alarm ").expect("an alarm line");
                            let device_at = rest.find(" (device ").expect("its device");
                            let device = rest[device_at + " (device ".len()..]
                                .split(')')
                                .next()
                                .expect("a device");
                            (rest[..device_at].to_owned(), device.to_owned())
                        })
                        .collect();
                    // Routes: `/1`'s sources as `/1` writes them but their ranks; each timer route one route per alarm on its
                    // device, the requesters and costs together `/1`'s.
                    let strip_rank = |line: &str| -> String {
                        let rest = line.strip_prefix("(route ").expect("a route");
                        rest.split_once(' ').expect("its rank").1.to_owned()
                    };
                    let one_routes = of(&one_lines, "route");
                    let two_routes = of(&two_lines, "route");
                    let one_sources: BTreeSet<String> = one_routes
                        .iter()
                        .filter(|l| !l.contains("(timer-route "))
                        .map(|l| strip_rank(l))
                        .collect();
                    let two_sources: BTreeSet<String> = two_routes
                        .iter()
                        .filter(|l| !l.contains(" (alarm "))
                        .map(|l| strip_rank(l))
                        .collect();
                    assert_eq!(
                        two_sources, one_sources,
                        "{at}: a source's route is `/1`'s but for its rank"
                    );
                    for one_route in one_routes.iter().filter(|l| l.contains("(timer-route ")) {
                        let device = one_route
                            .split("(timer-route ")
                            .nth(1)
                            .expect("its device")
                            .split(')')
                            .next()
                            .expect("named")
                            .to_owned();
                        let on_it: Vec<&String> = nodes
                            .iter()
                            .filter(|(_, d)| *d == device)
                            .map(|(node, _)| node)
                            .collect();
                        assert!(!on_it.is_empty(), "{at}: `{device}`'s route has an alarm");
                        let mut requesters = BTreeSet::new();
                        let mut costs = BTreeSet::new();
                        for node in &on_it {
                            let route = two_routes
                                .iter()
                                .find(|l| l.contains(&format!(" (alarm {node}) ")))
                                .unwrap_or_else(|| panic!("{at}: `{node}`'s route"));
                            assert!(route.contains("(separation programmed)"), "{at}: {route}");
                            requesters.extend(items(route, "requesters"));
                            costs.extend(items(route, "costs"));
                        }
                        assert_eq!(
                            requesters,
                            items(one_route, "requesters"),
                            "{at}: `{device}`'s requesters"
                        );
                        assert_eq!(costs, items(one_route, "costs"), "{at}: `{device}`'s costs");
                    }
                    assert_eq!(
                        two_routes.len(),
                        one_sources.len() + nodes.len(),
                        "{at}: one route for each source and each alarm"
                    );
                    // The enabled line: `timer`, then each route's item by rank.
                    let items: Vec<String> = two_routes
                        .iter()
                        .map(|l| {
                            let rest = strip_rank(l);
                            let end = rest.find(" (separation ").expect("its separation");
                            rest[..end].to_owned()
                        })
                        .collect();
                    assert_eq!(
                        of(&two_lines, "enabled"),
                        [format!(
                            "(enabled timer{})",
                            items.iter().map(|i| format!(" {i}")).collect::<String>()
                        )],
                        "{at}"
                    );
                    // Context lines: as `/1`'s, but their interrupts; a line of `/1` that held nothing but a timer route's
                    // interrupt may stand in `/2` with an alarm's, or not at all where no alarm's reach holds its subject.
                    let without_interrupts = |line: &str| -> String {
                        let mut out = String::new();
                        let mut rest = line;
                        while let Some(at) = rest.find(" (interrupt ") {
                            out.push_str(&rest[..at]);
                            let tail = &rest[at + 1..];
                            let mut depth = 0;
                            let mut end = tail.len();
                            for (i, c) in tail.char_indices() {
                                match c {
                                    '(' => depth += 1,
                                    ')' => {
                                        depth -= 1;
                                        if depth == 0 {
                                            end = i + 1;
                                            break;
                                        }
                                    }
                                    _ => {}
                                }
                            }
                            rest = &tail[end..];
                        }
                        out.push_str(rest);
                        out
                    };
                    let two_contexts: BTreeSet<String> = of(&two_lines, "context")
                        .iter()
                        .map(|l| without_interrupts(l))
                        .collect();
                    for line in of(&one_lines, "context") {
                        let stripped = without_interrupts(&line);
                        let only_a_timer = line.contains("(interrupt (timer-route ")
                            && !line.contains(" job")
                            && !line.contains(" initialisation")
                            && !line.contains("(interrupt (source ");
                        assert!(
                            two_contexts.contains(&stripped) || only_a_timer,
                            "{at}: {line}"
                        );
                    }
                    for line in of(&two_lines, "context") {
                        assert!(!line.contains("(timer-route "), "{at}: {line}");
                    }
                    // Obligations: `/1`'s but its route obligations, one for each source and each alarm.
                    let route_obligation = |l: &String| l.starts_with("(obligation route ");
                    let one_obligations: Vec<String> = of(&one_lines, "obligation")
                        .into_iter()
                        .filter(|l| !route_obligation(l))
                        .collect();
                    let two_obligations: Vec<String> = of(&two_lines, "obligation")
                        .into_iter()
                        .filter(|l| !route_obligation(l))
                        .collect();
                    assert_eq!(two_obligations, one_obligations, "{at}");
                    let alarm_obligations = of(&two_lines, "obligation")
                        .iter()
                        .filter(|l| l.starts_with("(obligation route (alarm "))
                        .count();
                    assert_eq!(
                        alarm_obligations,
                        nodes.len(),
                        "{at}: one route obligation for each alarm"
                    );
                    // Owed: `/1`'s but a timer route's, and what §3 adds of the system, its tasks, its nodes, its alarms and
                    // its clocks.
                    let one_owed: BTreeSet<&str> = one_lines
                        .iter()
                        .copied()
                        .filter(|l| head(l) == "owed" && !l.starts_with("(owed (timer-route "))
                        .collect();
                    for line in two_lines.iter().filter(|l| head(l) == "owed") {
                        assert!(!line.starts_with("(owed (timer-route "), "{at}: {line}");
                        if !one_owed.contains(line) {
                            assert!(
                                line.starts_with("(owed (alarm ")
                                    || line.starts_with("(owed (clock ")
                                    || OWED_BY_TWO.iter().any(|(fact, owner)| line
                                        .ends_with(&format!(" {fact} (by {owner}))"))),
                                "{at}: `/2` owes what §3 does not add: {line}"
                            );
                        }
                    }
                    for line in &one_owed {
                        assert!(
                            two_lines.contains(line),
                            "{at}: `/1` owes what `/2` does not: {line}"
                        );
                    }
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
                (Ok(_), Err(two)) => panic!("{at}: `/2` refuses what `/1` lowers: {two}"),
            }
        }
    }
    // Each kind of outcome is among them, so none is held by having no case; and the counts are pinned, so a fixture
    // that stops lowering, or starts, is seen.
    assert_eq!(
        (plans, alarmed, alarms, refused, regions, clocks),
        (142, 61, 68, 34, 3, 1)
    );
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

/// A fixture of the corpus lowered as hosted `/2`: its lines whose head is `head`.
fn lines_of(fixture: &str, head_of: &str) -> Vec<String> {
    let text = fs::read_to_string(
        root()
            .join("crates/archogen-plan/tests/corpus")
            .join(fixture),
    )
    .expect("the fixture");
    let (_, two) = lowered_both(&text, None, &generation()).expect("it resolves");
    two.expect("it lowers")
        .lines()
        .filter(|line| head(line) == head_of)
        .map(str::to_owned)
        .collect()
}

/// `dod-bare`, an expiry: the shipped deadline over a delay delivering to `time.deadline`, which asks no bound (§3; leaf
/// `M4.1.7.6.5`). Its line, its route, the contexts naming it — the requester it delivers to among them — its obligation,
/// and what it owes: as a source, `reach-ahead` on the counter, and its delivery bound of the description.
#[test]
fn an_expiry_is_written_as_its_own_alarm() {
    let node = "(adapter deadline-over-delay timer.delay time.deadline)";
    assert_eq!(
        lines_of("dod-bare.eadl", "alarm"),
        [format!("(alarm {node} (device timer.delay) (simulated (tick-rate 1000000) (modulus 4294967296)) (kind level-compare) (time-base counter) (horizon none) (delivery-bound none) (asked-reach none))")]
    );
    assert_eq!(
        lines_of("dod-bare.eadl", "route"),
        [format!("(route 1 (alarm {node}) (separation programmed) (level machine) (deliverable yes) (requesters time.deadline) (costs ({node} expiry)))")]
    );
    assert_eq!(
        lines_of("dod-bare.eadl", "enabled"),
        [format!("(enabled timer (alarm {node}))")]
    );
    let contexts = lines_of("dod-bare.eadl", "context");
    for subject in [node, "time.deadline", "timer.delay"] {
        assert!(
            contexts.contains(&format!(
                "(context {subject} job (interrupt (alarm {node})) initialisation)"
            )),
            "{subject}: {contexts:?}"
        );
    }
    assert!(lines_of("dod-bare.eadl", "obligation").contains(&format!(
        "(obligation route (alarm {node}) (discharged-by M4.2 M4.3))"
    )));
    let owed: Vec<String> = lines_of("dod-bare.eadl", "owed")
        .into_iter()
        .filter(|l| l.starts_with("(owed (alarm "))
        .collect();
    let expected: Vec<String> = [
        ("acknowledge-point", "realization"),
        ("deferred-work", "realization"),
        ("delivery-bound", "description"),
        ("floor", "realization"),
        ("no-application-code", "realization"),
        ("one-request-per-arrival", "realization"),
        ("reach-ahead", "realization"),
        ("service-cost", "realization"),
    ]
    .iter()
    .map(|(fact, owner)| format!("(owed (alarm {node}) {fact} (by {owner}))"))
    .collect();
    assert_eq!(owed, expected);
}

/// The record's worked epoch extender, a wake: its horizon its device's, `(65536 − 1) / 10 MHz`, its `step` owed and no
/// `reach-ahead` or delivery bound, its time base the counter (§3).
#[test]
fn a_wake_is_written_with_its_device_s_horizon() {
    let node = "(adapter epoch-extender timer.short)";
    assert_eq!(
        lines_of("worked-epoch-extender.eadl", "alarm"),
        [format!("(alarm {node} (device timer.short) (simulated (tick-rate 10000000) (modulus 65536)) (kind level-compare) (time-base counter) (horizon 13107/2000000 s) (delivery-bound none) (asked-reach none))")]
    );
    assert_eq!(
        lines_of("worked-epoch-extender.eadl", "route"),
        [format!("(route 1 (alarm {node}) (separation programmed) (level machine) (deliverable yes) (requesters time.monotonic) (costs ({node} wake)))")]
    );
    let owed: Vec<String> = lines_of("worked-epoch-extender.eadl", "owed")
        .into_iter()
        .filter(|l| l.starts_with("(owed (alarm "))
        .collect();
    assert!(
        owed.contains(&format!("(owed (alarm {node}) step (by realization))")),
        "{owed:?}"
    );
    assert!(
        !owed
            .iter()
            .any(|l| l.contains(" reach-ahead ") || l.contains(" delivery-bound ")),
        "{owed:?}"
    );
    // A wake runs its own code alone: `time.monotonic` is bound to it, and reached by none of its code.
    let contexts = lines_of("worked-epoch-extender.eadl", "context");
    assert!(
        contexts.contains(&"(context time.monotonic job initialisation)".to_owned()),
        "{contexts:?}"
    );
}

/// Two expiries on one device, `two-dod`: two lines, two routes ranked by the delivery bound each asks, the one asking
/// none last, and the device's context naming both (§3; `decision_release-alarm.md` §3).
#[test]
fn two_alarms_on_one_device_are_two_lines_ranked_by_their_bounds() {
    let a = "(adapter deadline-over-delay timer.delay dl.a)";
    let b = "(adapter deadline-over-delay timer.delay dl.b)";
    let alarms = lines_of("two-dod.eadl", "alarm");
    assert_eq!(alarms.len(), 2);
    assert!(
        alarms[0].starts_with(&format!("(alarm {a} "))
            && alarms[0].contains("(delivery-bound 1/20000 s)")
    );
    assert!(
        alarms[1].starts_with(&format!("(alarm {b} "))
            && alarms[1].contains("(delivery-bound none)")
    );
    let routes = lines_of("two-dod.eadl", "route");
    assert!(
        routes[0].starts_with(&format!("(route 1 (alarm {a})"))
            && routes[1].starts_with(&format!("(route 2 (alarm {b})")),
        "{routes:?}"
    );
    assert!(lines_of("two-dod.eadl", "context").contains(&format!(
        "(context timer.delay job (interrupt (alarm {a})) (interrupt (alarm {b})) initialisation)"
    )));
    let obligations = lines_of("two-dod.eadl", "obligation");
    assert_eq!(
        obligations
            .iter()
            .filter(|l| l.starts_with("(obligation route "))
            .count(),
        2
    );
}

/// An alarm bound directly, `block-absolute-deadline`: its node the requester itself, its delivery bound the group's
/// `(delivery-bound (at-most 5 ms))`, and its device stating no rate or modulus, each written `unstated` and owed of the
/// description (§3, `SIMULATED`).
#[test]
fn an_alarm_bound_directly_is_its_requester_s() {
    assert_eq!(
        lines_of("block-absolute-deadline.eadl", "alarm"),
        ["(alarm dl (device rtc) (simulated (tick-rate unstated) (modulus unstated)) (kind level-compare) (time-base counter) (horizon none) (delivery-bound 1/200 s) (asked-reach none))"]
    );
    let owed = lines_of("block-absolute-deadline.eadl", "owed");
    for fact in ["tick-rate", "counter-modulus"] {
        assert!(
            owed.contains(&format!("(owed (alarm dl) {fact} (by description))")),
            "{fact}"
        );
    }
    assert!(!owed.contains(&"(owed (alarm dl) delivery-bound (by description))".to_owned()));
}

/// A counter a time base reads that no alarm's line names has a line of its own, `cycle`'s `timer.counter`, its rate and
/// modulus unstated and owed (§3, `(clock …)`).
#[test]
fn a_counter_no_alarm_names_has_a_clock_line() {
    assert_eq!(
        lines_of("cycle.eadl", "clock"),
        ["(clock timer.counter (simulated (tick-rate unstated) (modulus unstated)))"]
    );
    let owed = lines_of("cycle.eadl", "owed");
    for fact in ["tick-rate", "counter-modulus"] {
        assert!(
            owed.contains(&format!(
                "(owed (clock timer.counter) {fact} (by description))"
            )),
            "{fact}"
        );
    }
}

/// A description lowered as hosted `/2`: its lines whose head is `head`.
fn lines_of_text(text: &str, head_of: &str) -> Vec<String> {
    let (_, two) = lowered_both(text, None, &generation()).expect("it resolves");
    two.expect("it lowers")
        .lines()
        .filter(|line| head(line) == head_of)
        .map(str::to_owned)
        .collect()
}

/// An expiry whose requester reads time through an epoch extender over the alarm's own device has `epoch` for its time
/// base, and owes no `reach-ahead`; the wake beside it ranks first, by its horizon, the expiry asking no bound last, and a
/// requester reaching both stands on both routes (§3, `BASE`; RP5-1, RP8-6; `decision_release-alarm.md` §3).
#[test]
fn an_expiry_read_through_its_own_device_s_extender_is_on_the_epoch() {
    let text = "(eadl-version eadl/1)
(defblock timer.x (offers (relative-delay true) (counter-width 16 bit) (counter-modulus 65536) (tick-rate 10 MHz) (wrap-behavior modular) (observation-coherent true) (channels 2 tick) (reachable-at-privilege machine)))
(defplatform soc.x (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.x)))
(defservice time.monotonic (requires (unambiguous-horizon (at-least 60 s)) (needs counter-width tick-rate observation-coherent)))
(defservice dl (requires (needs absolute-deadline) (uses time.monotonic)))
(defsystem ti (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses dl)) (platform (uses soc.x)))
";
    let expiry = "(adapter deadline-over-delay timer.x dl)";
    let wake = "(adapter epoch-extender timer.x)";
    let alarms = lines_of_text(text, "alarm");
    assert!(
        alarms[0].starts_with(&format!("(alarm {expiry} "))
            && alarms[0].contains("(time-base epoch)"),
        "{alarms:?}"
    );
    assert!(
        alarms[1].starts_with(&format!("(alarm {wake} "))
            && alarms[1].contains("(time-base counter)"),
        "{alarms:?}"
    );
    let routes = lines_of_text(text, "route");
    assert!(
        routes[0].starts_with(&format!("(route 1 (alarm {wake}) "))
            && routes[0].contains("(requesters dl time.monotonic)"),
        "{routes:?}"
    );
    assert!(
        routes[1].starts_with(&format!("(route 2 (alarm {expiry}) "))
            && routes[1].contains("(requesters dl)"),
        "{routes:?}"
    );
    let owed = lines_of_text(text, "owed");
    assert!(
        !owed.contains(&format!(
            "(owed (alarm {expiry}) reach-ahead (by realization))"
        )),
        "{owed:?}"
    );
}

/// An expiry's asked reach is what its requirement asks by `supported-horizon`, a direct alarm's at its device (§3).
#[test]
fn an_expiry_s_asked_reach_is_what_its_requirement_asks() {
    let text = "(eadl-version eadl/1)
(defblock rtc (offers (absolute-deadline true) (supported-horizon 10 s) (delivery-bound 1 us) (channels 1 tick) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses rtc)))
(defservice dl (requires (absolute-deadline (delivery-bound (at-most 5 ms)) (supported-horizon (at-least 2 s)))))
(defsystem bd (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses dl)) (platform (uses soc)))
";
    assert_eq!(
        lines_of_text(text, "alarm"),
        ["(alarm dl (device rtc) (simulated (tick-rate unstated) (modulus unstated)) (kind level-compare) (time-base counter) (horizon none) (delivery-bound 1/200 s) (asked-reach 2 s))"]
    );
}
