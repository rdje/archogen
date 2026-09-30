//! **F17** — `ROADMAP.md` §13.1: "Unknown interrupt interference or unsupported task behavior — Refuse timing
//! assurance" (leaf `M2.6.4`, the analysis half; the runtime half is `M4`'s).
//!
//! The fixture is one complete task set that `fixed-priority-with-overheads/1` admits, and a list of single
//! defects, each applied alone. Every defect must be **refused** with the verdict
//! `docs/decisions/decision_runtime-analysis-variant.md` §5 gives it, `unsupported-profile` or
//! `analysis-inconclusive`, and **never silently downgraded**:
//! - a refused set yields no admitted set, so no conclusion of the runtime variant exists for it;
//! - the idealized baseline, which a careless caller might fall back to, states a different model, and its
//!   conclusion always carries its "no overhead" assumption, so it cannot pass for a runtime result.

use archogen_evidence::claim::Conclusion;
use rt_analysis::runtime::{
    self, admit, Acknowledge, Deferred, EnabledSet, Evidence, Platform, PlatformFacts,
    RefusalVerdict, ReleasedBy, RuntimeTask, Source, TaskFacts, Value,
};
use rt_analysis::{analyze, conclusion, Task, TaskSet};

fn v(value: u64) -> Option<Value> {
    Some(Value {
        value,
        evidence: Evidence::Analytical,
    })
}

/// The complete set: a timer, one UART, a timer-released task and a UART-released one, everything declared.
/// Masked run `L = max(CS + S, C_rel + S, C_s + S, W_wake) = 3`, so every floor is 3.
fn complete() -> (Vec<RuntimeTask>, Vec<Source>, Platform) {
    let facts = TaskFacts {
        suspends: Some(false),
        locks_scheduler: Some(false),
        shares_outside_sections: Some(false),
    };
    let tasks = vec![
        RuntimeTask {
            id: "H".into(),
            priority: 1,
            computation: v(2),
            separation: 10,
            deadline: 10,
            jitter_event: 0,
            jitter_release: v(3),
            masked_section: v(0),
            released_by: ReleasedBy::Timer,
            facts,
        },
        RuntimeTask {
            id: "E".into(),
            priority: 2,
            computation: v(1),
            separation: 25,
            deadline: 25,
            jitter_event: 0,
            jitter_release: v(3),
            masked_section: v(0),
            released_by: ReleasedBy::Source {
                source: "uart".into(),
                every_arrival: true,
            },
            facts,
        },
    ];
    let sources = vec![Source {
        id: "uart".into(),
        service: v(2),
        separation: v(25),
        jitter: v(3),
        acknowledge: Some(Acknowledge::AtEntry),
        priority: Some(1),
        deferred: Some(Deferred::Nothing),
    }];
    let platform = Platform {
        switch: v(1),
        wake: v(1),
        preemption_delay: v(0),
        timer_service: v(1),
        rounding: v(0),
        delivery: v(0),
        facts: PlatformFacts {
            one_processor: Some(true),
            preemptive_everywhere: Some(true),
            interrupts_do_not_nest: Some(true),
            sections_mask_every_interrupt: Some(true),
            services_preempt_every_task: Some(true),
            pending_taken_and_transitions_unmasked: Some(true),
            eager_switching: Some(true),
            timer_event_driven: Some(true),
            compare_level: Some(true),
            compare_rounds_up: Some(true),
            due_check_matches_compare: Some(true),
            no_early_release: Some(true),
            raised_only_when_due: Some(true),
            only_timer_releases_timer_tasks: Some(true),
            costs_hold_under_any_preemption: Some(true),
        },
        enabled: Some(EnabledSet {
            interrupts: vec!["timer".into(), "uart".into()],
            from_resolved_plan: true,
        }),
    };
    (tasks, sources, platform)
}

type Defect = fn(&mut Vec<RuntimeTask>, &mut Vec<Source>, &mut Platform);

/// Every F17 defect: its name, what it does, the verdict record §5 gives it, and a word its reason must carry.
const DEFECTS: &[(&str, Defect, RefusalVerdict, &str)] = &[
    // Unknown interrupt interference.
    (
        "an enabled interrupt no source declares",
        |_, _, p| {
            if let Some(enabled) = p.enabled.as_mut() {
                enabled.interrupts.push("spi".into());
            }
        },
        RefusalVerdict::UnsupportedProfile,
        "unmodeled-interrupt-load",
    ),
    (
        "a declared source whose service cost is unknown",
        |_, s, _| s[0].service = None,
        RefusalVerdict::AnalysisInconclusive,
        "C_s of source `uart` is not declared",
    ),
    (
        "a declared source whose arrival separation is unknown",
        |_, s, _| s[0].separation = None,
        RefusalVerdict::AnalysisInconclusive,
        "T_s of source `uart` is not declared",
    ),
    (
        "the timer's service cost is unknown",
        |_, _, p| p.timer_service = None,
        RefusalVerdict::AnalysisInconclusive,
        "C_rel (one timer service) is not declared",
    ),
    (
        "the build's enabled interrupts are unknown",
        |_, _, p| p.enabled = None,
        RefusalVerdict::AnalysisInconclusive,
        "enabled interrupts are not declared",
    ),
    (
        "interrupts nest",
        |_, _, p| p.facts.interrupts_do_not_nest = Some(false),
        RefusalVerdict::UnsupportedProfile,
        "interrupts do not nest does not hold",
    ),
    // Unsupported task behavior.
    (
        "a task that suspends itself",
        |t, _, _| t[1].facts.suspends = Some(true),
        RefusalVerdict::UnsupportedProfile,
        "suspends itself",
    ),
    (
        "a task that locks the scheduler",
        |t, _, _| t[0].facts.locks_scheduler = Some(true),
        RefusalVerdict::UnsupportedProfile,
        "locks the scheduler",
    ),
    (
        "a task that shares data outside its masked sections",
        |t, _, _| t[0].facts.shares_outside_sections = Some(true),
        RefusalVerdict::UnsupportedProfile,
        "shares data outside",
    ),
    (
        "a task whose behavior is undeclared",
        |t, _, _| t[1].facts.suspends = None,
        RefusalVerdict::AnalysisInconclusive,
        "does not declare whether it suspends itself",
    ),
];

#[test]
fn the_complete_set_is_admitted_so_each_refusal_is_its_defect_s() {
    let (tasks, sources, platform) = complete();
    let set = admit(&tasks, &sources, &platform).expect("the complete set is admitted");
    assert_eq!(set.task_ids(), ["H", "E"]);
}

#[test]
fn every_f17_defect_refuses_timing_assurance_with_its_verdict() {
    assert!(DEFECTS.len() >= 10, "the defect list was cut");
    let mut wrong = Vec::new();
    for (name, defect, verdict, reason) in DEFECTS {
        let (mut tasks, mut sources, mut platform) = complete();
        defect(&mut tasks, &mut sources, &mut platform);
        match admit(&tasks, &sources, &platform) {
            Ok(_) => wrong.push(format!(
                "{name}: admitted — a silent downgrade to an answer"
            )),
            Err(refusal) => {
                if refusal.verdict != *verdict {
                    wrong.push(format!("{name}: {refusal}, expected {}", verdict.slug()));
                } else if !refusal.reasons.iter().any(|r| r.contains(reason)) {
                    wrong.push(format!("{name}: {refusal} does not say `{reason}`"));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn the_idealized_baseline_cannot_pass_for_a_runtime_result() {
    // The same computation times and separations through the baseline: it may say the deadlines hold, but in its
    // own model, and always with "no overhead" among its assumptions. A consumer that read it as the runtime
    // variant's answer would have to drop both.
    let set = TaskSet::admit(vec![
        Task {
            id: "H".into(),
            priority: 1,
            computation: 2,
            separation: 10,
            deadline: 10,
        },
        Task {
            id: "E".into(),
            priority: 2,
            computation: 1,
            separation: 25,
            deadline: 25,
        },
    ])
    .expect("admitted by the baseline");
    match conclusion(&analyze(&set)) {
        Conclusion::HoldsUnderAssumptions { model, assumptions } => {
            assert_ne!(model, runtime::MODEL);
            assert!(model.contains("idealized-zero-overhead"), "{model}");
            assert!(
                assumptions.iter().any(|a| a.starts_with("no overhead")),
                "{assumptions:?}"
            );
        }
        other => panic!("{other:?}"),
    }
}
