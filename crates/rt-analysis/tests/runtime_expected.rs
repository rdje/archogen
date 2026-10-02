//! The runtime variant against expected results derived independently (leaf `M2.6.3`).
//!
//! `tests/fixtures/runtime_expected.txt` was written by a context that read
//! `docs/decisions/decision_runtime-analysis-variant.md`, the roadmap and the profile, and was told not to read this
//! crate. It derived every expectation by hand. The inputs were given to it; the expected lines are its own, copied
//! verbatim. This test runs every fixture through `rt_analysis::runtime` and requires the two to agree, as `M2.2`'s
//! reference and the runtime agree. A disagreement is classified and answered in the record, never "fixed" here by
//! editing an expectation.

use std::collections::BTreeMap;

use rt_analysis::runtime::{
    admit, analyze, Acknowledge, Deferred, EnabledSet, Evidence, Outcome, Platform, PlatformFacts,
    RefusalVerdict, ReleasedBy, RuntimeTask, Source, TaskFacts, Value,
};

const FIXTURES: &str = include_str!("fixtures/runtime_expected.txt");

/// One fixture: its inputs as given, and the expectations the deriving context wrote.
#[derive(Debug, Default)]
struct Fixture {
    name: String,
    platform: BTreeMap<String, String>,
    sources: Vec<BTreeMap<String, String>>,
    tasks: Vec<BTreeMap<String, String>>,
    expect: Vec<Vec<String>>,
}

/// `key=value` words after the first `skip` words of a line.
fn pairs(words: &[&str]) -> BTreeMap<String, String> {
    words
        .iter()
        .filter_map(|w| w.split_once('='))
        .map(|(k, v)| (k.to_string(), v.to_string()))
        .collect()
}

fn parse() -> Vec<Fixture> {
    let mut fixtures = Vec::new();
    let mut current: Option<Fixture> = None;
    for line in FIXTURES.lines() {
        let line = line.trim();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let words: Vec<&str> = line.split_whitespace().collect();
        match words[0] {
            "fixture" => {
                assert!(
                    current.is_none(),
                    "fixture `{}` opened inside another",
                    words[1]
                );
                current = Some(Fixture {
                    name: words[1].to_string(),
                    ..Fixture::default()
                });
            }
            "end" => fixtures.push(current.take().expect("an `end` closes an open fixture")),
            keyword => {
                let fixture = current.as_mut().expect("a line inside a fixture");
                match keyword {
                    "platform" => fixture.platform = pairs(&words[1..]),
                    "source" => {
                        let mut source = pairs(&words[2..]);
                        source.insert("id".into(), words[1].to_string());
                        fixture.sources.push(source);
                    }
                    "task" => {
                        let mut task = pairs(&words[2..]);
                        task.insert("id".into(), words[1].to_string());
                        fixture.tasks.push(task);
                    }
                    "expect" => fixture
                        .expect
                        .push(words[1..].iter().map(|w| (*w).to_string()).collect()),
                    other => panic!("fixture `{}`: unknown line `{other}`", fixture.name),
                }
            }
        }
    }
    assert!(current.is_none(), "the last fixture is not closed");
    fixtures
}

fn number(map: &BTreeMap<String, String>, key: &str) -> u64 {
    map.get(key)
        .unwrap_or_else(|| panic!("missing `{key}` in {map:?}"))
        .parse()
        .unwrap_or_else(|_| panic!("`{key}` is not a number in {map:?}"))
}

fn value(map: &BTreeMap<String, String>, key: &str) -> Option<Value> {
    match map.get(key).map(String::as_str) {
        None | Some("missing") => None,
        Some(text) => Some(Value {
            value: text.parse().unwrap_or_else(|_| panic!("`{key}={text}`")),
            evidence: Evidence::Analytical,
        }),
    }
}

fn platform(map: &BTreeMap<String, String>) -> Platform {
    assert_eq!(map.get("facts").map(String::as_str), Some("all"), "{map:?}");
    let fact = |name: &str| match map.get(name).map(String::as_str) {
        Some("false") => Some(false),
        Some(other) => panic!("fact `{name}={other}`"),
        None => Some(true),
    };
    Platform {
        switch: value(map, "S"),
        wake: value(map, "W_wake"),
        preemption_delay: value(map, "gamma"),
        timer_service: value(map, "C_rel"),
        rounding: value(map, "rho"),
        delivery: value(map, "delta"),
        facts: PlatformFacts {
            one_processor: fact("one_processor"),
            preemptive_everywhere: fact("preemptive_everywhere"),
            interrupts_do_not_nest: fact("interrupts_do_not_nest"),
            sections_mask_every_interrupt: fact("sections_mask_every_interrupt"),
            services_preempt_every_task: fact("services_preempt_every_task"),
            pending_taken_and_transitions_unmasked: fact("pending_taken_and_transitions_unmasked"),
            eager_switching: fact("eager_switching"),
            services_paid_by_arrivals: fact("services_paid_by_arrivals"),
            timer_event_driven: fact("timer_event_driven"),
            compare_level: fact("compare_level"),
            compare_rounds_up: fact("compare_rounds_up"),
            due_check_matches_compare: fact("due_check_matches_compare"),
            no_early_release: fact("no_early_release"),
            raised_only_when_due: fact("raised_only_when_due"),
            only_timer_releases_timer_tasks: fact("only_timer_releases_timer_tasks"),
            costs_hold_under_any_preemption: fact("costs_hold_under_any_preemption"),
        },
        enabled: map.get("enabled").map(|list| EnabledSet {
            interrupts: list.split(',').map(str::to_string).collect(),
            from_resolved_plan: true,
        }),
    }
}

fn source(map: &BTreeMap<String, String>) -> Source {
    Source {
        id: map["id"].clone(),
        service: value(map, "C"),
        separation: value(map, "T"),
        jitter: value(map, "J"),
        acknowledge: match map.get("ack").map(String::as_str) {
            Some("entry") => Some(Acknowledge::AtEntry),
            Some("exit") => Some(Acknowledge::AtExit),
            other => panic!("ack `{other:?}`"),
        },
        priority: Some(number(map, "priority").try_into().expect("small")),
        deferred: match map.get("deferred").map(String::as_str) {
            Some("nothing") => Some(Deferred::Nothing),
            Some(task) => Some(Deferred::Task(task.to_string())),
            None => None,
        },
    }
}

fn task(map: &BTreeMap<String, String>) -> RuntimeTask {
    let released = &map["released"];
    RuntimeTask {
        id: map["id"].clone(),
        priority: number(map, "prio").try_into().expect("small"),
        computation: value(map, "C"),
        separation: number(map, "T"),
        deadline: number(map, "D"),
        jitter_event: number(map, "Jevent"),
        jitter_release: value(map, "Jrelease"),
        masked_section: value(map, "CS"),
        released_by: if released == "timer" {
            ReleasedBy::Timer
        } else {
            let (source, every) = released
                .split_once(':')
                .unwrap_or_else(|| panic!("released `{released}`"));
            ReleasedBy::Source {
                source: source.to_string(),
                every_arrival: every == "every",
            }
        },
        facts: TaskFacts {
            suspends: Some(false),
            locks_scheduler: Some(false),
            shares_outside_sections: Some(false),
        },
    }
}

fn sequence(words: &[String]) -> Option<Vec<u128>> {
    words.iter().find_map(|w| w.strip_prefix("w=")).map(|list| {
        list.split(',')
            .map(|n| n.parse().expect("an iterate"))
            .collect()
    })
}

fn response(words: &[String]) -> Option<u128> {
    words
        .iter()
        .find_map(|w| w.strip_prefix("R="))
        .map(|n| n.parse().expect("a response"))
}

/// Every way the implementation and the derivation disagree on one fixture.
fn disagreements(fixture: &Fixture) -> Vec<String> {
    let mut wrong = Vec::new();
    let sources: Vec<Source> = fixture.sources.iter().map(source).collect();
    let tasks: Vec<RuntimeTask> = fixture.tasks.iter().map(task).collect();
    let admitted = admit(&tasks, &sources, &platform(&fixture.platform));
    let refused = fixture.expect.iter().find(|e| e[0] == "refused");
    match (admitted, refused) {
        (Err(refusal), Some(expect)) => {
            if refusal.verdict.slug() != expect[1] {
                wrong.push(format!(
                    "refused as {}, derived {}: {refusal}",
                    refusal.verdict.slug(),
                    expect[1]
                ));
            }
        }
        (Err(refusal), None) => wrong.push(format!("refused ({refusal}), derived an analysis")),
        (Ok(_), Some(expect)) => {
            wrong.push(format!("admitted, derived a refusal as {}", expect[1]))
        }
        (Ok(set), None) => {
            let outcomes = analyze(&set);
            let ids = set.task_ids();
            for expect in &fixture.expect {
                let Some(index) = ids.iter().position(|id| *id == expect[0]) else {
                    wrong.push(format!("an expectation names `{}`, not a task", expect[0]));
                    continue;
                };
                let outcome = &outcomes[index];
                let (kind, detail) = (expect[1].as_str(), expect.get(2).map(String::as_str));
                let agrees = match (outcome, kind, detail) {
                    (
                        Outcome::Holds {
                            response: r,
                            witness,
                            ..
                        },
                        "holds",
                        _,
                    ) => {
                        response(expect) == Some(*r)
                            && sequence(expect).as_ref() == Some(&witness.sequence)
                    }
                    (
                        Outcome::NotEstablished { why, witness },
                        "not-established",
                        Some("utilisation"),
                    ) => why.contains("at least one") && witness.sequence.is_empty(),
                    (
                        Outcome::NotEstablished { why, witness },
                        "not-established",
                        Some("busy-period"),
                    ) => {
                        why.contains("busy period")
                            && sequence(expect).as_ref() == Some(&witness.sequence)
                    }
                    (
                        Outcome::NotEstablished { why, witness },
                        "not-established",
                        Some("past-deadline"),
                    ) => {
                        let bound = response(expect).map(|r| format!("response bound {r} "));
                        bound.is_some_and(|b| why.contains(&b))
                            && sequence(expect).as_ref() == Some(&witness.sequence)
                    }
                    (Outcome::Inconclusive { .. }, "inconclusive", _) => true,
                    _ => false,
                };
                if !agrees {
                    wrong.push(format!(
                        "task `{}`: derived `{}`, the implementation gives {outcome:?}",
                        expect[0],
                        expect.join(" ")
                    ));
                }
            }
            if fixture.expect.len() != ids.len() {
                wrong.push(format!(
                    "{} expectations for {} tasks",
                    fixture.expect.len(),
                    ids.len()
                ));
            }
        }
    }
    wrong
}

#[test]
fn every_independently_derived_expectation_is_what_the_implementation_gives() {
    let fixtures = parse();
    assert!(
        fixtures.len() > 10,
        "only {} fixtures read — the file moved or was cut",
        fixtures.len()
    );
    let mut wrong = Vec::new();
    for fixture in &fixtures {
        assert!(
            !fixture.expect.is_empty(),
            "fixture `{}` has no expectation",
            fixture.name
        );
        for w in disagreements(fixture) {
            wrong.push(format!("{}: {w}", fixture.name));
        }
    }
    assert!(
        wrong.is_empty(),
        "{} disagreement(s):\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

#[test]
fn a_derived_expectation_the_implementation_does_not_meet_is_reported() {
    // The RED arm: the same comparison over a fixture whose expectation is wrong by one.
    let mut fixture = parse()
        .into_iter()
        .find(|f| f.name == "basic")
        .expect("the `basic` fixture");
    for expect in &mut fixture.expect {
        for word in expect.iter_mut() {
            if let Some(r) = word.strip_prefix("R=") {
                *word = format!("R={}", r.parse::<u128>().expect("a number") + 1);
            }
        }
    }
    assert!(!disagreements(&fixture).is_empty());
    let _ = RefusalVerdict::UnsupportedProfile;
}
