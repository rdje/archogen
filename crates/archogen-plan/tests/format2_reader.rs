//! `archogen-runtime-plan/2`'s reader held to its goldens (`docs/decisions/decision_runtime-plan-format-2.md` §3; leaf
//! `M4.1.7.7`): "consumers read `/2` as they read `/1`: by a reader held to its goldens, never by matching text".
//!
//! Each golden of `tests/goldens2/` reads back to its own bytes, and what a consumer reads of it is what it writes. Every
//! line of every golden is mutated three ways — a token dropped, a value respelled, and the line swapped with the next —
//! and each mutation is refused, naming the line. `/1`'s text is refused by `/2`'s reader by its format, and `/2`'s by
//! `/1`'s. The lowering reads back what it writes through this reader (`lower_hosted`, `lower_on_target`), so
//! `format2_hosted.rs`, `format2_target.rs` and `format2_application.rs` hold it to every plan they lower too.

mod common;

use std::fs;

use archogen_plan::contents::{NodeKind, Separation};
use archogen_plan::contents2::{
    read, Context, Controller, Count, Epoch, Figure, Item, Reading, Target, Timer, Window,
};
use archogen_plan::{identity, Plan, Plan2};
use common::root;
use eadl_model::rational::Rational;

/// `/2`'s five goldens.
const GOLDENS: [&str; 5] = [
    "heartbeat-hosted",
    "heartbeat-hosted-application",
    "heartbeat-target",
    "sporadic-fixture",
    "two-alarms-fixture",
];

fn golden(name: &str) -> String {
    fs::read_to_string(root().join(format!("crates/archogen-plan/tests/goldens2/{name}.plan")))
        .expect("a golden")
}

fn whole(n: i128) -> Rational {
    Rational::new(n, 1).expect("a value")
}

fn ratio(n: i128, d: i128) -> Rational {
    Rational::new(n, d).expect("a value")
}

/// Every golden reads back as itself: its text, its sha256 and its code identity are what it writes.
#[test]
fn every_golden_reads_back_to_its_own_bytes() {
    for name in GOLDENS {
        let text = golden(name);
        let plan = Plan2::read(&text).unwrap_or_else(|why| panic!("{name}: {why}"));
        assert_eq!(plan.text(), text, "{name}");
        assert_eq!(
            plan.code_identity(),
            identity::code_identity(&text).expect("an identity"),
            "{name}"
        );
        assert_eq!(
            plan.sha256(),
            archogen_digest::Digest::of(text.as_bytes()),
            "{name}"
        );
    }
}

/// What a consumer reads of each golden is what it writes: the target and the catalog, the epoch, the timer, each alarm's
/// counter, each route's interrupt and controller, the regions, the placement, the allocations and the image, the
/// application's lines, and the contexts.
#[test]
fn what_a_consumer_reads_is_what_each_golden_writes() {
    let hosted = read(&golden("heartbeat-hosted")).expect("it reads");
    assert_eq!(hosted.target, Target::HostedPlayground);
    assert_eq!(hosted.catalog_target, None);
    assert_eq!(hosted.emitter, "archogen-emit/1");
    assert_eq!(hosted.epoch, Epoch::Simulator);
    assert_eq!(
        hosted.timer,
        Timer::Simulator(vec![
            "(task heartbeat beat)".to_owned(),
            "(task heartbeat chime)".to_owned()
        ])
    );
    assert_eq!(hosted.enabled, [Item::Timer]);
    assert!(hosted.placement.is_none() && hosted.image.is_none() && hosted.allocations.is_empty());
    assert!(hosted.application.is_none() && hosted.task_inputs.is_empty());
    assert_eq!(hosted.tasks.len(), 2);
    assert!(!hosted.complete);

    let application = read(&golden("heartbeat-hosted-application")).expect("it reads");
    assert!(application
        .application
        .as_deref()
        .is_some_and(|hex| hex.len() == 64));
    assert!(application
        .code
        .as_deref()
        .is_some_and(|code| code.starts_with("(code app \"app\" \"sha256:")));
    assert_eq!(
        application
            .task_inputs
            .iter()
            .map(|(identity, _)| identity.as_str())
            .collect::<Vec<_>>(),
        ["(task heartbeat beat)", "(task heartbeat chime)"]
    );
    assert!(application.declared.as_deref().is_some_and(
        |line| line.starts_with("(declared leaves-interrupt-hardware-alone yes (clause \"")
    ));
    assert_eq!(
        application.initialisation_inputs.as_deref(),
        Some("(initialisation-inputs none)")
    );

    let target = read(&golden("heartbeat-target")).expect("it reads");
    let Target::Machine { platform, digest } = &target.target else {
        panic!("a machine: {:?}", target.target);
    };
    assert_eq!(platform, "target.riscv-virt-up");
    assert_eq!(digest.len(), 64);
    assert_eq!(target.catalog_target.as_deref(), Some("riscv-virt-up"));
    assert_eq!(target.epoch, Epoch::AfterRead(Figure::Owed));
    let Timer::Device {
        device,
        counter,
        horizon,
        rounding,
        releases,
    } = &target.timer
    else {
        panic!("a release timer: {:?}", target.timer);
    };
    assert_eq!(device, "target.timer");
    assert_eq!(counter.tick_rate, whole(10_000_000));
    assert_eq!(counter.modulus, Count::Pow2(64));
    assert_eq!(counter.reach, whole(1_844_674_407_370));
    assert_eq!(counter.step, Count::Int(1));
    assert_eq!(counter.window, Some(Window::Magnitude));
    assert_eq!(*horizon, ratio(3_689_348_814_741_910_323, 2_000_000));
    assert_eq!(*rounding, Figure::Owed);
    assert_eq!(releases.len(), 2);
    assert_eq!(
        target
            .regions
            .iter()
            .map(|r| (
                r.name.as_str(),
                r.provider.as_str(),
                r.base,
                r.size,
                r.usage.as_str()
            ))
            .collect::<Vec<_>>(),
        [
            (
                "target.timer",
                "target.timer",
                33_554_432,
                Some(65_536),
                "device"
            ),
            (
                "target.console",
                "target.console",
                268_435_456,
                Some(256),
                "device"
            ),
            (
                "target.ram",
                "target.riscv-virt-up",
                2_147_483_648,
                Some(134_217_728),
                "placement"
            ),
        ]
    );
    assert_eq!(target.regions[0].requesters, ["(release heartbeat)"]);
    assert_eq!(
        target.placement,
        Some(("target.riscv-virt-up".to_owned(), "target.ram".to_owned()))
    );
    assert_eq!(
        target
            .allocations
            .iter()
            .map(|a| (a.unit.as_str(), a.laid.is_none()))
            .collect::<Vec<_>>(),
        [
            ("(stack boot)", true),
            ("(stack interrupt)", true),
            ("(stack (task heartbeat beat))", true),
            ("(stack (task heartbeat chime))", true),
        ]
    );
    assert!(target.allocations[0]
        .needs
        .contains(&"(application heartbeat initialisation)".to_owned()));
    let image = target.image.as_ref().expect("an image");
    assert_eq!(
        (image.floor, image.limit, image.truncated_by.len()),
        (2_147_483_648, None, 0)
    );
    assert_eq!(target.enabled, [Item::Timer]);
    assert!(target.contexts.contains(&(
        "target.timer".to_owned(),
        vec![Context::Interrupt(Item::Timer), Context::Initialisation]
    )));
    assert!(target
        .nodes
        .contains(&("target.timer".to_owned(), NodeKind::Block)));
    assert!(target.obligations.iter().any(|o| o.kind == "placement"
        && o.subject == "(region target.riscv-virt-up target.ram)"
        && o.discharged_by == ["M4.4"]));

    let sporadic = read(&golden("sporadic-fixture")).expect("it reads");
    assert_eq!(sporadic.epoch, Epoch::None(None));
    assert_eq!(sporadic.timer, Timer::None);
    assert_eq!(sporadic.routes.len(), 1);
    assert_eq!(
        sporadic.routes[0].item,
        Item::Source("target.button".to_owned())
    );
    assert_eq!(sporadic.routes[0].separation, Separation::Of(ratio(1, 50)));
    assert_eq!(sporadic.routes[0].controller, Some(Controller::Owed));
    assert_eq!(sporadic.enabled, [Item::Source("target.button".to_owned())]);

    let alarms = read(&golden("two-alarms-fixture")).expect("it reads");
    assert_eq!(alarms.epoch, Epoch::None(Some(Figure::Owed)));
    assert_eq!(
        alarms
            .alarms
            .iter()
            .map(|a| a.node.as_str())
            .collect::<Vec<_>>(),
        ["wait.a", "wait.b"]
    );
    for alarm in &alarms.alarms {
        assert_eq!(alarm.device, "target.timer");
        let Reading::Counter(counter) = &alarm.reading else {
            panic!("a counter on a target: {alarm:?}");
        };
        assert_eq!(
            (counter.tick_rate, counter.modulus),
            (whole(10_000_000), Count::Pow2(64))
        );
        assert_eq!(
            (alarm.kind.as_str(), alarm.time_base.as_str()),
            ("level-compare", "counter")
        );
        assert_eq!(
            (alarm.horizon, alarm.delivery_bound, alarm.asked_reach),
            (None, None, None)
        );
    }
    assert_eq!(
        alarms
            .routes
            .iter()
            .map(|r| (r.rank, r.item.clone(), r.separation))
            .collect::<Vec<_>>(),
        [
            (
                1,
                Item::Source("target.button".to_owned()),
                Separation::Of(ratio(1, 50))
            ),
            (2, Item::Alarm("wait.a".to_owned()), Separation::Programmed),
            (3, Item::Alarm("wait.b".to_owned()), Separation::Programmed),
        ]
    );
    assert!(alarms
        .routes
        .iter()
        .all(|r| r.controller == Some(Controller::Owed)));
    assert!(alarms.contexts.contains(&(
        "target.timer".to_owned(),
        vec![
            Context::Job,
            Context::Interrupt(Item::Alarm("wait.a".to_owned())),
            Context::Interrupt(Item::Alarm("wait.b".to_owned())),
            Context::Initialisation,
        ]
    )));
}

/// The index of the last top-level form of a line: the start of its last item, a list or an atom.
fn last_item(line: &str) -> Option<usize> {
    let bytes = line.as_bytes();
    let (mut depth, mut in_text, mut escaped, mut start) = (0_i32, false, false, None);
    for (at, &b) in bytes.iter().enumerate() {
        if in_text {
            match b {
                _ if escaped => escaped = false,
                b'\\' => escaped = true,
                b'"' => in_text = false,
                _ => {}
            }
            continue;
        }
        match b {
            b'"' => {
                if depth == 1 {
                    start = Some(at);
                }
                in_text = true;
            }
            b'(' => {
                if depth == 1 {
                    start = Some(at);
                }
                depth += 1;
            }
            b')' => depth -= 1,
            b' ' => {}
            _ if depth == 1 && bytes[at - 1] == b' ' => start = Some(at),
            _ => {}
        }
    }
    start
}

/// `text` with line `n` replaced.
fn with_line(text: &str, n: usize, line: &str) -> String {
    text.lines()
        .enumerate()
        .map(|(at, old)| if at == n { line } else { old })
        .map(|line| format!("{line}\n"))
        .collect()
}

/// A token dropped from every line of every golden: its last item, so `(owed S F (by X))` loses `(by X)` and `(enabled
/// timer)` loses `timer`. Each is refused naming its line, whatever the line: a line that loses its last item no longer
/// reads, or no longer says what the lines around it say of it.
#[test]
fn every_line_with_a_token_dropped_is_refused() {
    let mut mutated = 0;
    for name in GOLDENS {
        let text = golden(name);
        for (n, line) in text.lines().enumerate() {
            let Some(at) = last_item(line) else { continue };
            let dropped = format!("{})", line[..at].trim_end());
            if dropped == line {
                continue;
            }
            let refused = read(&with_line(&text, n, &dropped));
            match refused {
                Err(why) => assert!(
                    why.starts_with(&format!("line {}: ", n + 1)) || !why.starts_with("line "),
                    "{name} line {}: `{dropped}` refused naming another line: {why}",
                    n + 1
                ),
                Ok(_) => panic!("{name} line {}: `{dropped}` read", n + 1),
            }
            mutated += 1;
        }
    }
    assert_eq!(mutated, 590, "every line of the five goldens");
}

/// A value respelled in every line of every golden that holds one a consumer reads, each a spelling no writer writes: a
/// leading zero on its first integer, a rate or a duration not in lowest terms, a power of two written as an integer. Each
/// is refused naming its line.
#[test]
fn every_value_respelled_is_refused() {
    let mut mutated = 0;
    for name in GOLDENS {
        let text = golden(name);
        for (n, line) in text.lines().enumerate() {
            let head = identity::head(line);
            let mut respelled = Vec::new();
            // A leading zero on the first integer a consumer reads: a task's index, a step's number, a route's rank, a
            // region's base, the image's floor.
            if matches!(head, "task" | "step" | "route" | "region" | "image") {
                let digits = line
                    .match_indices(|c: char| c.is_ascii_digit())
                    .find(|(at, _)| {
                        line.as_bytes()[at - 1] == b' ' || line.as_bytes()[at - 1] == b'('
                    });
                if let Some((at, _)) = digits {
                    respelled.push(format!("{}0{}", &line[..at], &line[at..]));
                }
            }
            // A rate not in lowest terms; a duration not reduced; `(pow2 64)` written below where it is written; a
            // compare's step with a leading zero: each in the lines that read it as a value, a need's or an owed line's
            // subject, `(step 1)` among them, being text a consumer compares whole.
            for (heads, from, to) in [
                (
                    &["timer", "alarm"][..],
                    "(tick-rate 10000000)",
                    "(tick-rate 20000000/2)",
                ),
                (&["task"][..], "(deadline 1/100 s)", "(deadline 2/200 s)"),
                (&["task"][..], "(period 1/100 s)", "(period 2/200 s)"),
                (
                    &["route"][..],
                    "(separation 1/50 s)",
                    "(separation 2/100 s)",
                ),
                (
                    &["timer", "alarm"][..],
                    "(modulus (pow2 64))",
                    "(modulus 18446744073709551616)",
                ),
                (&["timer", "alarm"][..], "(step 1)", "(step 01)"),
            ] {
                if heads.contains(&head) && line.contains(from) {
                    respelled.push(line.replacen(from, to, 1));
                }
            }
            for spelling in respelled {
                let refused = read(&with_line(&text, n, &spelling));
                let why = refused
                    .err()
                    .unwrap_or_else(|| panic!("{name} line {}: `{spelling}` read", n + 1));
                assert!(
                    why.starts_with(&format!("line {}: ", n + 1)),
                    "{name} line {}: {why}",
                    n + 1
                );
                mutated += 1;
            }
        }
    }
    assert_eq!(
        mutated, 63,
        "every value a consumer reads that a respelling reaches"
    );
}

/// Every line of every golden swapped with the next where the two differ: each is refused, naming the line that stands out
/// of §3's order — a section's order or its lines' own order within it, "Orders" — so a plan has one order of its lines as
/// it has one spelling of each.
#[test]
fn every_line_swapped_with_the_next_is_refused() {
    let mut mutated = 0;
    for name in GOLDENS {
        let text = golden(name);
        let lines: Vec<&str> = text.lines().collect();
        for n in 0..lines.len() - 1 {
            if lines[n] == lines[n + 1] {
                continue;
            }
            let mut swapped = lines.clone();
            swapped.swap(n, n + 1);
            let swapped: String = swapped.iter().map(|line| format!("{line}\n")).collect();
            let why = read(&swapped)
                .err()
                .unwrap_or_else(|| panic!("{name} lines {} and {} swapped read", n + 1, n + 2));
            assert!(
                why.starts_with(&format!("line {}: ", n + 1))
                    || why.starts_with(&format!("line {}: ", n + 2))
                    || !why.starts_with("line "),
                "{name} lines {} and {}: {why}",
                n + 1,
                n + 2
            );
            mutated += 1;
        }
    }
    assert_eq!(
        mutated, 585,
        "every adjacent pair of distinct lines of the five goldens"
    );
}

/// `/1`'s text is refused by `/2`'s reader by its format, and `/2`'s by `/1`'s: a reader never takes the other format's
/// plan for its own.
#[test]
fn each_format_is_refused_by_the_other_s_reader() {
    let one = fs::read_to_string(root().join("crates/archogen-plan/tests/goldens/heartbeat.plan"))
        .expect("a golden");
    let why = read(&one).expect_err("refused");
    assert!(
        why.starts_with("line 1: not `archogen-runtime-plan/2`"),
        "{why}"
    );
    for name in GOLDENS {
        let why = Plan::read(&golden(name)).expect_err("refused");
        assert!(
            why.starts_with("line 1: not `archogen-runtime-plan/1`"),
            "{name}: {why}"
        );
    }
}

/// A text with a backslash in a quoted string reads, the string kept as written, so writing it back gives its bytes; a
/// basis holding a quote is written by the language's printer with that quote escaped (`M4.1.7.6.6`).
#[test]
fn an_escaped_quote_in_a_text_reads_and_writes_back() {
    let text = golden("heartbeat-hosted-application");
    let n = text
        .lines()
        .position(|line| line.starts_with("(declared "))
        .expect("a declaration");
    let line = text.lines().nth(n).expect("the line");
    let escaped = line.replacen("(basis \"", "(basis \"a \\\"quoted\\\" word, ", 1);
    assert_ne!(escaped, line);
    let read_back = read(&with_line(&text, n, &escaped)).expect("it reads");
    assert_eq!(read_back.declared.as_deref(), Some(escaped.as_str()));
}

/// The application's file `/2` reads through `application::read`, `/1`'s reader of it: the golden's file reads, names the
/// system and the hosted playground, and the plan holds of it what it states — its digest, the sha256 of its canonical
/// text (§1), and each task's inputs by its name, its entry point among them. `/2` asks nothing more of that reader than a
/// target's name compared on a target (`Application::unmet_on`, `M4.1.7.6.7.1`).
#[test]
fn the_application_s_file_reads_as_the_plan_holds_it() {
    let file = fs::read_to_string(
        root().join("crates/archogen-plan/tests/goldens2/heartbeat.application"),
    )
    .expect("the golden's file");
    let application = archogen_plan::application::read(&file).expect("it reads");
    assert_eq!(application.system, "heartbeat");
    assert_eq!(
        application.target,
        archogen_plan::application::HOSTED_PLAYGROUND
    );
    let plan = read(&golden("heartbeat-hosted-application")).expect("it reads");
    let mut sources = eadl_front::SourceMap::new();
    let id = sources
        .add("heartbeat.application", file.clone())
        .expect("small");
    let (document, _) = eadl_front::read_record(&sources, id);
    assert_eq!(
        plan.application.as_deref(),
        Some(
            archogen_digest::Digest::of(document.to_canonical().as_bytes())
                .hex()
                .as_str()
        )
    );
    assert_eq!(application.tasks.len(), plan.task_inputs.len());
    for task in &application.tasks {
        let (_, line) = plan
            .task_inputs
            .iter()
            .find(|(identity, _)| *identity == format!("(task heartbeat {})", task.name))
            .unwrap_or_else(|| panic!("the inputs of `{}`", task.name));
        assert!(
            line.contains(&format!("(entry-point \"{}\")", task.entry_point)),
            "{line}"
        );
    }
}
