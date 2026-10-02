//! `fixed-priority-with-overheads/2` against the record that defines it (leaf `M2.6.2`,
//! `docs/decisions/decision_runtime-analysis-variant.md`).
//!
//! Every expected number here was derived by hand from the record's §2 before the test was run; each derivation is
//! written beside its assertion so a reader can redo it. The independent derivation of a wider fixture set, by a
//! context that never reads this crate, is `M2.6.3`.

use archogen_evidence::claim::Conclusion;
use rt_analysis::runtime::{
    admit, analyze, conclusion, Acknowledge, Deferred, EnabledSet, Evidence, Outcome, Platform,
    PlatformFacts, RefusalVerdict, ReleasedBy, RuntimeTask, Source, TaskFacts, Value, CONDITIONS,
    MODEL,
};

fn v(value: u64) -> Option<Value> {
    Some(Value {
        value,
        evidence: Evidence::Analytical,
    })
}

fn all_facts() -> PlatformFacts {
    PlatformFacts {
        one_processor: Some(true),
        preemptive_everywhere: Some(true),
        interrupts_do_not_nest: Some(true),
        sections_mask_every_interrupt: Some(true),
        services_preempt_every_task: Some(true),
        pending_taken_and_transitions_unmasked: Some(true),
        eager_switching: Some(true),
        services_paid_by_arrivals: Some(true),
        timer_event_driven: Some(true),
        compare_level: Some(true),
        compare_rounds_up: Some(true),
        due_check_matches_compare: Some(true),
        no_early_release: Some(true),
        raised_only_when_due: Some(true),
        only_timer_releases_timer_tasks: Some(true),
        costs_hold_under_any_preemption: Some(true),
    }
}

/// `S = 1`, `W_wake = 1`, `γ = 0`, `C_rel = 1`, `ρ = 0`, `δ = 0`; the timer is the only enabled interrupt. With every
/// `CS = 0`, the longest contiguous masked run is `L = max(0 + S, C_rel + S, W_wake) = 2`, so a timer-released task's
/// floor is `ρ + δ + L = 2`.
fn platform() -> Platform {
    Platform {
        switch: v(1),
        wake: v(1),
        preemption_delay: v(0),
        timer_service: v(1),
        rounding: v(0),
        delivery: v(0),
        facts: all_facts(),
        enabled: Some(EnabledSet {
            interrupts: vec!["timer".into()],
            from_resolved_plan: true,
        }),
    }
}

fn task(id: &str, priority: i64, c: u64, t: u64, d: u64, j_release: u64) -> RuntimeTask {
    RuntimeTask {
        id: id.into(),
        priority,
        computation: v(c),
        separation: t,
        deadline: d,
        jitter_event: 0,
        jitter_release: v(j_release),
        masked_section: v(0),
        released_by: ReleasedBy::Timer,
        facts: TaskFacts {
            suspends: Some(false),
            locks_scheduler: Some(false),
            shares_outside_sections: Some(false),
        },
    }
}

fn holds(outcome: &Outcome) -> (u128, Vec<u128>) {
    match outcome {
        Outcome::Holds {
            response, witness, ..
        } => (*response, witness.sequence.clone()),
        other => panic!("expected a bound within the deadline, got {other:?}"),
    }
}

fn refused(
    result: Result<rt_analysis::runtime::RuntimeSet, rt_analysis::runtime::Refusal>,
) -> rt_analysis::runtime::Refusal {
    match result {
        Err(refusal) => refusal,
        Ok(set) => panic!("expected a refusal, and {:?} was admitted", set.task_ids()),
    }
}

#[test]
fn the_hand_derived_example_holds_at_the_derived_bounds() {
    // H: C 2, T 10, J 2. w0 = C + S = 3. Timer term over every task, ⌈(w+J)/T⌉·(C_rel+γ):
    //   w = 3 → H ⌈5/10⌉ = 1, L ⌈5/20⌉ = 1 → 3 + 2 = 5; w = 5 → ⌈7/10⌉ = 1, ⌈7/20⌉ = 1 → 5. R = 2 + 5 = 7.
    // L: C 3, T 20, J 2. w0 = 4. hp H costs C + 2S + γ = 4.
    //   w = 4 → 4 + ⌈6/10⌉·4 + (⌈6/10⌉ + ⌈6/20⌉)·1 = 4 + 4 + 2 = 10
    //   w = 10 → 4 + ⌈12/10⌉·4 + (⌈12/10⌉ + ⌈12/20⌉) = 4 + 8 + 3 = 15
    //   w = 15 → 4 + ⌈17/10⌉·4 + (⌈17/10⌉ + ⌈17/20⌉) = 4 + 8 + 3 = 15.  R = 2 + 15 = 17.
    let set = admit(
        &[task("H", 1, 2, 10, 10, 2), task("L", 2, 3, 20, 20, 2)],
        &[],
        &platform(),
    )
    .expect("admitted");
    let outcomes = analyze(&set);
    assert_eq!(holds(&outcomes[0]), (7, vec![3, 5, 5]));
    assert_eq!(holds(&outcomes[1]), (17, vec![4, 10, 15, 15]));
    match conclusion(&set, &outcomes) {
        Conclusion::HoldsUnderAssumptions { model, assumptions } => {
            assert_eq!(model, MODEL);
            assert_eq!(
                assumptions.len(),
                CONDITIONS.len(),
                "every input analytical, so the conditions alone: {assumptions:?}"
            );
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn an_interrupt_source_and_a_task_it_releases_are_charged_as_the_record_says() {
    // A source `uart`: C_s 2, T_s 25, J_s 3, acknowledged at entry. L = max(CS + S, C_rel + S, C_s + S, W_wake)
    // = max(1, 2, 3, 1) = 3, so a source's floor is δ + L = 3 and a timer task's is ρ + δ + L = 3.
    // H (timer): C 2, T 10, J 3. w0 = 3. w = 3 → 3 + timer (⌈6/10⌉ + ⌈6/25⌉) + source ⌈6/25⌉·2 = 3 + 2 + 2 = 7;
    //   w = 7 → timer (⌈10/10⌉ + ⌈10/25⌉) + source ⌈10/25⌉·2 → 7. R = 3 + 7 = 10 = D.
    // E (released by uart on every arrival): C 1, T 25, J 3. w0 = 2. hp H costs 4.
    //   w = 2 → 2 + ⌈5/10⌉·4 + (⌈5/10⌉ + ⌈5/25⌉) + ⌈5/25⌉·2 = 2 + 4 + 2 + 2 = 10
    //   w = 10 → 2 + ⌈13/10⌉·4 + (⌈13/10⌉ + ⌈13/25⌉) + ⌈13/25⌉·2 = 2 + 8 + 3 + 2 = 15
    //   w = 15 → 2 + ⌈18/10⌉·4 + (2 + 1) + 2 = 15.  R = 3 + 15 = 18 ≤ 25.
    // E's own timer charge is a declared over-charge: it is released by the source (record §3).
    let mut platform = platform();
    platform.enabled = Some(EnabledSet {
        interrupts: vec!["timer".into(), "uart".into()],
        from_resolved_plan: true,
    });
    let uart = Source {
        id: "uart".into(),
        service: v(2),
        separation: v(25),
        jitter: v(3),
        acknowledge: Some(Acknowledge::AtEntry),
        priority: Some(1),
        deferred: Some(Deferred::Nothing),
    };
    let mut e = task("E", 2, 1, 25, 25, 3);
    e.released_by = ReleasedBy::Source {
        source: "uart".into(),
        every_arrival: true,
    };
    let set = admit(&[task("H", 1, 2, 10, 10, 3), e], &[uart], &platform).expect("admitted");
    let outcomes = analyze(&set);
    assert_eq!(holds(&outcomes[0]), (10, vec![3, 7, 7]));
    assert_eq!(holds(&outcomes[1]), (18, vec![2, 10, 15, 15]));
}

#[test]
fn a_bound_past_its_deadline_is_not_established_and_never_a_counterexample() {
    // The first example's L has bound 17; with a deadline of 16 that bound is past it.
    let set = admit(
        &[task("H", 1, 2, 10, 10, 2), task("L", 2, 3, 20, 16, 2)],
        &[],
        &platform(),
    )
    .expect("admitted");
    let outcomes = analyze(&set);
    assert!(
        matches!(outcomes[1], Outcome::NotEstablished { .. }),
        "{:?}",
        outcomes[1]
    );
    match conclusion(&set, &outcomes) {
        Conclusion::NotEstablished { why } => assert!(why.contains("envelope"), "{why}"),
        other => panic!("a pessimistic bound past a deadline is not a counterexample: {other:?}"),
    }
}

#[test]
fn an_interference_utilisation_of_one_is_not_established_without_iterating() {
    // C_rel = 5 and two tasks of separation 10: the timer term alone is 5/10 + 5/10 = 1. L = C_rel + S = 6, so each
    // J^release is its floor, 6.
    let mut platform = platform();
    platform.timer_service = v(5);
    let set = admit(
        &[task("A", 1, 1, 10, 10, 6), task("B", 2, 1, 10, 10, 6)],
        &[],
        &platform,
    )
    .expect("admitted");
    match &analyze(&set)[0] {
        Outcome::NotEstablished { why, witness } => {
            assert!(why.contains("at least one"), "{why}");
            assert!(witness.sequence.is_empty(), "no iteration: {witness:?}");
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn a_task_whose_jitter_and_first_iterate_pass_its_separation_stops_at_the_busy_period() {
    // J 9 + w0 (C 1 + S 1 = 2) = 11 > T 10.
    let set = admit(&[task("A", 1, 1, 10, 10, 9)], &[], &platform()).expect("admitted");
    match &analyze(&set)[0] {
        Outcome::NotEstablished { why, witness } => {
            assert!(why.contains("busy period"), "{why}");
            assert_eq!(witness.sequence, vec![2]);
        }
        other => panic!("{other:?}"),
    }
}

#[test]
fn the_third_review_s_set_holds_without_early_release_and_is_refused_with_it() {
    // The third review's counterexample. S 1, C_rel 1, ρ 1, δ 0, W_wake 1, every CS 0: L = 2 and the timer floor
    // ρ + δ + L = 3, so each J^release is 3. H: C 5, T 20. L: C 6, T 100, D 20. K: C 1, T 100.
    // L: w0 = 7; w = 7 → 7 + ⌈10/20⌉·7 + (⌈10/20⌉ + ⌈10/100⌉ + ⌈10/100⌉) = 17; w = 17 → 7 + ⌈20/20⌉·7 + 3 = 17.
    //   R = 3 + 17 = 20 = D: holds, as the reviewer computed. The real timeline misses at 23 only because its kernel
    //   released H's next job early, so the analysis must refuse such a kernel rather than answer for it.
    let mut platform = platform();
    platform.rounding = v(1);
    let tasks = [
        task("H", 1, 5, 20, 20, 3),
        task("L", 2, 6, 100, 20, 3),
        task("K", 3, 1, 100, 100, 3),
    ];
    let set = admit(&tasks, &[], &platform).expect("admitted without early release");
    assert_eq!(holds(&analyze(&set)[1]), (20, vec![7, 17, 17]));

    platform.facts.no_early_release = Some(false);
    let refusal = refused(admit(&tasks, &[], &platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(
        refusal.reasons[0].contains("only once its nominal release has passed"),
        "{refusal}"
    );
}

#[test]
fn a_missing_platform_input_is_an_unresolved_bound() {
    let mut platform = platform();
    platform.switch = None;
    let refusal = refused(admit(&[task("A", 1, 1, 10, 10, 2)], &[], &platform));
    assert_eq!(refusal.verdict, RefusalVerdict::AnalysisInconclusive);
    assert!(
        refusal
            .reasons
            .iter()
            .any(|r| r.contains("S (one context transition) is not declared")),
        "{refusal}"
    );
}

#[test]
fn a_missing_fact_is_unresolved_and_a_false_one_is_outside_the_model() {
    let mut platform = platform();
    platform.facts.interrupts_do_not_nest = None;
    let refusal = refused(admit(&[task("A", 1, 1, 10, 10, 2)], &[], &platform));
    assert_eq!(refusal.verdict, RefusalVerdict::AnalysisInconclusive);
    platform.facts.interrupts_do_not_nest = Some(false);
    let refusal = refused(admit(&[task("A", 1, 1, 10, 10, 2)], &[], &platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);

    let mut suspending = task("A", 1, 1, 10, 10, 2);
    suspending.facts.suspends = Some(true);
    let refusal = refused(admit(&[suspending], &[], &self::platform()));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(refusal.reasons[0].contains("suspends itself"), "{refusal}");
}

#[test]
fn an_enabled_interrupt_nobody_declared_is_unmodelled_load() {
    let mut platform = platform();
    platform.enabled = Some(EnabledSet {
        interrupts: vec!["timer".into(), "uart".into()],
        from_resolved_plan: true,
    });
    let refusal = refused(admit(&[task("A", 1, 1, 10, 10, 2)], &[], &platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(
        refusal.reasons[0].contains("unmodeled-interrupt-load"),
        "{refusal}"
    );
}

#[test]
fn a_delay_below_its_floor_cannot_be_true() {
    // The timer floor is 2 here, and J^release 1 is below it.
    let refusal = refused(admit(&[task("A", 1, 1, 10, 10, 1)], &[], &platform()));
    assert_eq!(refusal.verdict, RefusalVerdict::AnalysisInconclusive);
    assert!(refusal.reasons[0].contains("cannot be true"), "{refusal}");
}

fn with_uart(jitter: u64, acknowledge: Acknowledge) -> (Platform, Source) {
    let mut platform = platform();
    platform.enabled = Some(EnabledSet {
        interrupts: vec!["timer".into(), "uart".into()],
        from_resolved_plan: true,
    });
    let uart = Source {
        id: "uart".into(),
        service: v(2),
        separation: v(25),
        jitter: v(jitter),
        acknowledge: Some(acknowledge),
        priority: Some(1),
        deferred: Some(Deferred::Nothing),
    };
    (platform, uart)
}

#[test]
fn a_source_that_can_lose_an_arrival_is_refused() {
    // Acknowledged at entry, a delay of 25 against a separation of 25 can lose one.
    let (platform, uart) = with_uart(25, Acknowledge::AtEntry);
    let refusal = refused(admit(&[task("A", 1, 1, 10, 10, 3)], &[uart], &platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(refusal.reasons[0].contains("lose an arrival"), "{refusal}");
    // Acknowledged at exit, J_s + C_s + S = 22 + 2 + 1 = 25 is not below 25.
    let (platform, uart) = with_uart(22, Acknowledge::AtExit);
    let refusal = refused(admit(&[task("A", 1, 1, 10, 10, 3)], &[uart], &platform));
    assert!(refusal.reasons[0].contains("lose an arrival"), "{refusal}");
}

#[test]
fn an_interrupt_no_arrival_pays_for_is_outside_the_model() {
    // Condition 5, added by leaf `M2.11`. The source term charges `C_s` once per arrival: for A, `w = 2 + 1 + 2 = 5`
    // and `R = 3 + 5 = 8`, within a deadline of 9. Were each arrival served twice, the term would have to charge 4:
    // `w = 2 + 1 + 4 = 7` and `R = 10`, past it. A trap no request pays for is charged nowhere at all. The variant
    // refuses a platform that does not declare every interrupt paid for, rather than answer for it.
    let (mut platform, uart) = with_uart(3, Acknowledge::AtEntry);
    let tasks = [task("A", 1, 1, 10, 9, 3)];
    let set = admit(&tasks, std::slice::from_ref(&uart), &platform)
        .expect("admitted, every interrupt paid for");
    assert_eq!(holds(&analyze(&set)[0]), (8, vec![2, 5, 5]));

    platform.facts.services_paid_by_arrivals = Some(false);
    let refusal = refused(admit(&tasks, std::slice::from_ref(&uart), &platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(
        refusal.reasons[0].contains("paid for by a due release or an arrival does not hold"),
        "{refusal}"
    );

    platform.facts.services_paid_by_arrivals = None;
    let refusal = refused(admit(&tasks, &[uart], &platform));
    assert_eq!(refusal.verdict, RefusalVerdict::AnalysisInconclusive);
    assert!(
        refusal.reasons[0].contains("does not declare whether every interrupt taken is paid for"),
        "{refusal}"
    );
}

#[test]
fn a_task_released_on_every_arrival_cannot_claim_a_longer_separation() {
    let (platform, uart) = with_uart(3, Acknowledge::AtEntry);
    let mut e = task("E", 1, 1, 30, 30, 3);
    e.released_by = ReleasedBy::Source {
        source: "uart".into(),
        every_arrival: true,
    };
    let refusal = refused(admit(&[e], &[uart], &platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(refusal.reasons[0].contains("cannot exceed"), "{refusal}");
}

#[test]
fn outside_the_model_outranks_an_unresolved_input() {
    let mut platform = platform();
    platform.switch = None;
    platform.facts.one_processor = Some(false);
    let refusal = refused(admit(&[task("A", 1, 1, 10, 10, 2)], &[], &platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(
        refusal.reasons.iter().all(|r| !r.contains("not declared")),
        "{refusal}"
    );
}

#[test]
fn deferred_work_must_run_as_a_declared_task_and_an_empty_set_is_refused() {
    let (platform, mut uart) = with_uart(3, Acknowledge::AtEntry);
    uart.deferred = Some(Deferred::Task("ghost".into()));
    let refusal = refused(admit(&[task("A", 1, 1, 10, 10, 3)], &[uart], &platform));
    assert!(refusal.reasons[0].contains("`ghost`"), "{refusal}");
    let refusal = refused(admit(&[], &[], &self::platform()));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(refusal.reasons[0].contains("empty"), "{refusal}");
}

#[test]
fn an_input_that_is_not_analytically_established_is_named_as_an_assumption() {
    let mut h = task("H", 1, 2, 10, 10, 2);
    h.computation = Some(Value {
        value: 2,
        evidence: Evidence::ObservedMaximum,
    });
    let mut platform = platform();
    platform.enabled = Some(EnabledSet {
        interrupts: vec!["timer".into()],
        from_resolved_plan: false,
    });
    let set = admit(&[h], &[], &platform).expect("admitted");
    let assumptions = set.assumptions();
    assert!(
        assumptions
            .iter()
            .any(|a| a == "C of task `H` = 2 (an observed maximum)"),
        "{assumptions:?}"
    );
    assert!(
        assumptions
            .iter()
            .any(|a| a.contains("catalog's platform declaration")),
        "{assumptions:?}"
    );
}

#[test]
fn an_undeclared_task_fact_is_an_unresolved_bound() {
    // Named as unexercised by `M2.6.3`'s derivation: a task fact left undeclared, not declared false.
    let mut undeclared = task("A", 1, 1, 10, 10, 2);
    undeclared.facts.locks_scheduler = None;
    let refusal = refused(admit(&[undeclared], &[], &platform()));
    assert_eq!(refusal.verdict, RefusalVerdict::AnalysisInconclusive);
    assert!(
        refusal.reasons[0].contains("does not declare whether it locks the scheduler"),
        "{refusal}"
    );
}

#[test]
fn a_task_released_on_some_arrivals_only_has_its_rate_limit_named() {
    // Named as unexercised by `M2.6.3`'s derivation: a rate-limited source release is admitted with its separation
    // as an assumption (record condition 5), not checked against the source's.
    let (platform, uart) = with_uart(3, Acknowledge::AtEntry);
    let mut e = task("E", 1, 1, 30, 30, 3);
    e.released_by = ReleasedBy::Source {
        source: "uart".into(),
        every_arrival: false,
    };
    let set = admit(&[e], &[uart], &platform).expect("admitted");
    assert!(
        set.assumptions()
            .iter()
            .any(|a| a
                == "task `E` is released on some arrivals of `uart` only, at most once per 30"),
        "{:?}",
        set.assumptions()
    );
}
