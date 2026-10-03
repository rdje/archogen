//! The composition of `C_i`, `CS_i`, `J^release` and `J_s` from their parts, against the record that defines it
//! (leaf `M2.10.2`, `docs/specs/catalog/decision_runtime-composite-inputs.md`, "the record" below).
//!
//! Every expected number here was derived by hand from the record's §3 before the test was run, and each derivation
//! is written beside its assertion so a reader can redo it. The leaf's acceptance adds a mutation matrix: each rule
//! of §3 removed in turn — a maximum where the definition adds, queued services omitted, `L` omitted, `2δ` halved,
//! `completion` dropped, a call count ignored — and the test that catches it named in the leaf.

use archogen_evidence::claim::Conclusion;
use rt_analysis::compose::{
    compose, compose_within, Application, Calls, Composition, Ending, Kernel, KernelFacts, NoBound,
    Owner, Primitive, Run, Service, SourceParts, Stops,
};
use rt_analysis::runtime::{
    analyze, conclusion, Acknowledge, Deferred, EnabledSet, Evidence, Outcome, Platform,
    PlatformFacts, RefusalVerdict, ReleasedBy, TaskFacts, Value,
};

fn v(value: u64) -> Option<Value> {
    Some(Value {
        value,
        evidence: Evidence::Analytical,
    })
}

fn observed(value: u64) -> Option<Value> {
    Some(Value {
        value,
        evidence: Evidence::ObservedMaximum,
    })
}

fn val(value: Option<Value>) -> u64 {
    value.expect("a composed value").value
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

/// `S = 1`, `W_wake = 1`, `γ = 0`, `C_rel = 1`, `ρ = 0`, `δ` as given; the timer and `sources` enabled.
fn platform_for(delta: u64, sources: &[&str]) -> Platform {
    let mut interrupts: Vec<String> = sources.iter().map(|s| (*s).to_owned()).collect();
    interrupts.push("timer".into());
    Platform {
        switch: v(1),
        wake: v(1),
        preemption_delay: v(0),
        timer_service: v(1),
        rounding: v(0),
        delivery: v(delta),
        facts: all_facts(),
        enabled: Some(EnabledSet {
            interrupts,
            from_resolved_plan: true,
        }),
    }
}

fn kernel_facts() -> KernelFacts {
    KernelFacts {
        reprograms_only_in_service: Some(true),
        pending_taken_after_unmask: Some(true),
        releases_never_latched: Some(true),
        primitives_out_of_line: Some(true),
        external_before_timer: Some(true),
        one_claim_per_trap: Some(true),
        no_empty_claim: Some(true),
        starts_by_transition: Some(true),
        one_external_controller: Some(true),
    }
}

fn primitive(name: &str, call: u64, masked: u64) -> Primitive {
    Primitive {
        name: name.into(),
        call: v(call),
        masked: v(masked),
    }
}

/// `api.mask = 1`, `masked.mask = 1`; `api.unmask = 1`, `masked.unmask = 1`; `api.spin = 2`, `masked.spin = 3`;
/// `completion = 2`, `masked.completion = 1`; every fact `yes`, sources ahead of the timer.
fn kernel() -> Kernel {
    Kernel {
        primitives: vec![
            primitive("mask", 1, 1),
            primitive("unmask", 1, 1),
            primitive("spin", 2, 3),
        ],
        completion: v(2),
        masked_completion: v(1),
        facts: kernel_facts(),
        discipline: vec![("rt-core".into(), Some(true))],
    }
}

fn calls(pairs: &[(&str, u64)]) -> Vec<Calls> {
    pairs
        .iter()
        .map(|(primitive, count)| Calls {
            primitive: (*primitive).to_owned(),
            count: *count,
        })
        .collect()
}

fn task(
    id: &str,
    priority: i64,
    own: Option<Value>,
    t: u64,
    d: u64,
    released_by: ReleasedBy,
) -> TaskParts {
    TaskParts {
        id: id.into(),
        priority,
        separation: t,
        deadline: d,
        jitter_event: 0,
        released_by,
        facts: TaskFacts {
            suspends: Some(false),
            locks_scheduler: Some(false),
            shares_outside_sections: Some(false),
        },
        own_code: own,
        calls: Vec::new(),
        runs: Vec::new(),
    }
}

use rt_analysis::compose::TaskParts;

fn application(tasks: Vec<TaskParts>) -> Application {
    Application {
        tasks,
        leaves_interrupt_hardware_alone: Some(true),
        releases_after_initialisation: Some(true),
    }
}

fn source(id: &str, c_s: u64, t_s: u64, acknowledge: Acknowledge, rank: i64) -> SourceParts {
    SourceParts {
        id: id.into(),
        separation: v(t_s),
        acknowledge: Some(acknowledge),
        deferred: Some(Deferred::Nothing),
        priority: Some(rank),
        deliverable: Some(true),
        service: Service::Catalog(v(c_s)),
        external: Some(true),
        one_request_per_arrival: Some(true),
    }
}

fn by_source(source: &str) -> ReleasedBy {
    ReleasedBy::Source {
        source: source.into(),
        every_arrival: true,
    }
}

fn statement<'a>(composition: &'a Composition, what: &str) -> &'a str {
    composition
        .composites
        .iter()
        .find(|composite| composite.what == what)
        .map(|composite| composite.what.as_str())
        .unwrap_or_else(|| panic!("no composite `{what}` among {:?}", composition.composites))
}

fn rendered(composition: &Composition, what: &str) -> String {
    let what = statement(composition, what).to_owned();
    composition
        .composites
        .iter()
        .find(|composite| composite.what == what)
        .expect("found above")
        .render()
}

fn refused(
    result: Result<rt_analysis::runtime::RuntimeSet, NoBound>,
) -> rt_analysis::runtime::Refusal {
    match result {
        Err(NoBound::Refused(refusal)) => refusal,
        other => panic!("expected a refusal, got {other:?}"),
    }
}

fn not_established(result: Result<rt_analysis::runtime::RuntimeSet, NoBound>) -> String {
    match result {
        Err(NoBound::NotEstablished { why }) => why,
        other => panic!("expected not-established, got {other:?}"),
    }
}

/// Fixture A: one timer-released task `a` and no source.
///   C_a  = C^app 5 + api.mask 1 + api.unmask 1 + 2 · api.spin 2 + completion 2 = 13 (§3), an observed maximum
///          since C^app is one.
///   CS_a = max(run 0: 3 + mask 1 + spin 2 + unmask 1 = 7; run 1: 2 + mask 1 + completion 2 = 5;
///              masked.mask 1, masked.unmask 1, masked.spin 3; masked.completion 1) = 7, every part analytical.
///   L    = max(CS_a + S = 8, C_rel + S = 2, W_wake = 1) = 8.
///   B_timer = ρ 0 + C_rel 1 + S 1 + L 8 + 2δ 2 = 12; nothing is ahead of the timer, so Δ_timer = 12 at once.
fn fixture_a() -> (Kernel, Platform, Application) {
    let mut a = task("a", 1, observed(5), 100, 100, ReleasedBy::Timer);
    a.calls = calls(&[("mask", 1), ("unmask", 1), ("spin", 2)]);
    a.runs = vec![
        Run {
            own_code: v(3),
            ending: Ending::Unmask,
            calls: calls(&[("spin", 1)]),
        },
        Run {
            own_code: v(2),
            ending: Ending::Completion,
            calls: Vec::new(),
        },
    ];
    (kernel(), platform_for(1, &[]), application(vec![a]))
}

#[test]
fn a_task_s_cost_and_masked_section_are_the_sum_and_the_maximum_the_record_says() {
    let (kernel, platform, application) = fixture_a();
    let composition = compose(&kernel, &platform, &application, &[]);
    assert_eq!(
        composition.stops,
        Stops::default(),
        "{:?}",
        composition.stops
    );
    let a = &composition.tasks[0];
    assert_eq!(val(a.computation), 13);
    assert_eq!(a.computation.unwrap().evidence, Evidence::ObservedMaximum);
    assert_eq!(val(a.masked_section), 7);
    assert_eq!(a.masked_section.unwrap().evidence, Evidence::Analytical);
    assert_eq!(val(a.jitter_release), 12);
    assert_eq!(
        rendered(&composition, "C of task `a`"),
        "C of task `a` = 13 (an observed maximum, the weakest of its parts): C^app = 5 (the application: an \
         observed maximum) + api.mask = 1 (the catalog: analytically established) + api.unmask = 1 (the catalog: \
         analytically established) + 2 × api.spin = 2 (the catalog: analytically established) + completion = 2 \
         (the catalog: analytically established)"
    );
    let cs = rendered(&composition, "CS of task `a`");
    assert!(
        cs.starts_with(
            "CS of task `a` = 7 (analytically established, the weakest of its parts): the largest of run 0, \
             ending at unmask = 7 [CS^app = 3 (the application: analytically established) + api.mask = 1 (the \
             catalog: analytically established) + api.spin = 2 (the catalog: analytically established) + \
             api.unmask = 1 (the catalog: analytically established)]; run 1, ending at completion = 5 ["
        ),
        "{cs}"
    );
    assert!(
        cs.contains("; masked.spin = 3 [masked.spin = 3 (the catalog: analytically established)]"),
        "{cs}"
    );
    assert!(cs.ends_with("; masked.completion = 1 [masked.completion = 1 (the catalog: analytically established)]"), "{cs}");
    assert_eq!(
        rendered(&composition, "L (the longest masked run)"),
        "L (the longest masked run) = 8 (analytically established, the weakest of its parts): the largest of CS \
         of task `a` + S = 8 [CS of task `a` = 7 (composed here: analytically established) + S = 1 (the catalog: \
         analytically established)]; C_rel + S = 2 [C_rel = 1 (the catalog: analytically established) + S = 1 \
         (the catalog: analytically established)]; W_wake = 1 [W_wake = 1 (the catalog: analytically established)]"
    );
    assert_eq!(
        rendered(&composition, "Δ_timer (J^release of every timer-released task)"),
        "Δ_timer (J^release of every timer-released task) = 12 (analytically established, the weakest of its \
         parts): the least fixed point of ρ = 0 (the catalog: analytically established) + C_rel = 1 (the catalog: \
         analytically established) + S = 1 (the catalog: analytically established) + L = 8 (composed here: \
         analytically established) + 2 × δ = 1 (the catalog: analytically established); Δ 12 → 12"
    );
    assert_eq!(
        rendered(&composition, "J^release of task `a`"),
        "J^release of task `a` = 12 (analytically established, the weakest of its parts): Δ_timer = 12 (composed \
         here: analytically established)"
    );
}

#[test]
fn the_conclusion_names_each_composed_input_with_its_parts_owners_and_categories() {
    let (kernel, platform, application) = fixture_a();
    let composition = compose(&kernel, &platform, &application, &[]);
    let set = composition.admit(&platform).expect("admitted");
    // a: w0 = C + S = 14; the timer charges C_rel + δ + γ = 2 per release: w = 14 → 14 + ⌈(14 + 12)/100⌉·2 = 16
    //    → ⌈28/100⌉·2 = 2 → 16. R = 12 + 16 = 28 ≤ 100.
    let outcomes = analyze(&set);
    match &outcomes[0] {
        Outcome::Holds {
            response, witness, ..
        } => {
            assert_eq!(*response, 28);
            assert_eq!(witness.sequence, vec![14, 16, 16]);
        }
        other => panic!("{other:?}"),
    }
    let Conclusion::HoldsUnderAssumptions { assumptions, .. } = conclusion(&set, &outcomes) else {
        panic!("expected a conclusion under assumptions");
    };
    let has = |needle: &str| {
        assert!(
            assumptions.iter().any(|line| line.contains(needle)),
            "no assumption holds `{needle}`:\n{}",
            assumptions.join("\n")
        );
    };
    // The variant's own line for an input not analytically established, then the composition's statements.
    has("C of task `a` = 13 (an observed maximum)");
    has("C of task `a` = 13 (an observed maximum, the weakest of its parts): C^app = 5 (the application: an observed maximum) + api.mask");
    has("CS of task `a` = 7 (analytically established, the weakest of its parts): the largest of");
    has("L (the longest masked run) = 8");
    has("Δ_timer (J^release of every timer-released task) = 12");
    has("J^release of task `a` = 12");
    // §2's assumptions of the environment and the run, and the caller's declaration (record §6).
    has("no source arrives before the first enabling of interrupts");
    has("no fatal fault is raised during the run");
    has("`releases-after-initialisation`");
}

/// Fixture B: source `uart` (C_s 2, T_s 25, acknowledged at entry) ahead of the timer; `a` timer-released
/// (C^app 2, T 100), `e` released by `uart` (C^app 1, T 25).
///   C_a = 2 + completion 2 = 4; CS_a = masked.completion = 1 (no run, no call). C_e = 3; CS_e = 1.
///   L = max(1 + 1, 1 + 1, C_s + S = 3, W_wake 1) = 3.
///   B_s = L + 2δ = 5; nothing ahead of `uart`: J_uart = 5.
///   B_timer = 0 + 1 + 1 + 3 + 2 = 7; ahead: `uart`, charge δ + C_s + S = 4, n = ⌈(Δ + 5)/25⌉.
///     Δ = 7 → 7 + 1·4 = 11 → ⌈16/25⌉ = 1 → 11. Δ_timer = 11.
fn fixture_b() -> (Kernel, Platform, Application, Vec<SourceParts>) {
    (
        kernel(),
        platform_for(1, &["uart"]),
        application(vec![
            task("a", 1, v(2), 100, 100, ReleasedBy::Timer),
            task("e", 2, v(1), 25, 25, by_source("uart")),
        ]),
        vec![source("uart", 2, 25, Acknowledge::AtEntry, 1)],
    )
}

#[test]
fn a_service_queued_ahead_is_charged_at_the_fixed_point_and_a_sources_release_jitter_is_its_delay()
{
    let (kernel, platform, application, sources) = fixture_b();
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops,
        Stops::default(),
        "{:?}",
        composition.stops
    );
    assert_eq!(val(composition.sources[0].jitter), 5);
    assert_eq!(val(composition.tasks[0].jitter_release), 11);
    assert_eq!(val(composition.tasks[1].jitter_release), 5);
    assert_eq!(
        rendered(&composition, "J_s of source `uart`"),
        "J_s of source `uart` = 5 (analytically established, the weakest of its parts): the least fixed point of \
         L = 3 (composed here: analytically established) + 2 × δ = 1 (the catalog: analytically established); \
         Δ 5 → 5"
    );
    let timer = rendered(
        &composition,
        "Δ_timer (J^release of every timer-released task)",
    );
    assert!(
        timer.ends_with(
            "+ L = 3 (composed here: analytically established) + 2 × δ = 1 (the catalog: analytically \
             established) + δ + C_s of source `uart` + S, queued ahead (T_s 25, J_s 5) = 4 (the plan: analytically \
             established); Δ 7 → 11 → 11"
        ),
        "{timer}"
    );
    assert_eq!(
        rendered(&composition, "J^release of task `e`"),
        "J^release of task `e` = 5 (analytically established, the weakest of its parts): J_s of source `uart` = 5 \
         (composed here: analytically established)"
    );
    assert!(
        composition
            .assumptions
            .iter()
            .any(|line| line == "T_s of source `uart` = 25: an arrival assumption about the environment, analytically established (record §6)"),
        "{:?}",
        composition.assumptions
    );
    // The variant admits the composed set and bounds both tasks:
    // a: w0 = C + S = 5; the timer charges 2 per release of every task, the source C_s + δ + γ = 3 per arrival.
    //    w = 5 → 5 + (⌈(5 + 11)/100⌉ + ⌈(5 + 5)/25⌉)·2 + ⌈(5 + 5)/25⌉·3 = 5 + 4 + 3 = 12
    //    w = 12 → 5 + (⌈23/100⌉ + ⌈17/25⌉)·2 + ⌈17/25⌉·3 = 12.  R_a = 11 + 12 = 23.
    // e: w0 = 4; a costs C + 2S + γ = 6 per release.
    //    w = 4 → 4 + ⌈(4 + 11)/100⌉·6 + (⌈15/100⌉ + ⌈(4 + 5)/25⌉)·2 + ⌈9/25⌉·3 = 4 + 6 + 4 + 3 = 17
    //    w = 17 → 4 + ⌈28/100⌉·6 + (1 + ⌈22/25⌉)·2 + ⌈22/25⌉·3 = 17.  R_e = 5 + 17 = 22 ≤ 25.
    let set = composition.admit(&platform).expect("admitted");
    let outcomes = analyze(&set);
    let bounds: Vec<(u128, Vec<u128>)> = outcomes
        .iter()
        .map(|outcome| match outcome {
            Outcome::Holds {
                response, witness, ..
            } => (*response, witness.sequence.clone()),
            other => panic!("{other:?}"),
        })
        .collect();
    assert_eq!(bounds, vec![(23, vec![5, 12, 12]), (22, vec![4, 17, 17])]);
}

#[test]
fn with_the_timer_ahead_of_every_source_its_releases_queue_before_a_source() {
    // Fixture B with `external-before-timer` `no` and `one-claim-per-trap` `yes`: the timer is first.
    //   Δ_timer = B_timer = 7, nothing ahead. J_uart: B_s = 5; ahead: the timer, charge δ + C_rel + S = 3, one
    //   ceiling per timer-released task, ⌈(Δ + J_a^event 0 + 7)/T_a 100⌉: Δ = 5 → 5 + 3 = 8 → ⌈15/100⌉ = 1 → 8.
    let (mut kernel, platform, application, sources) = fixture_b();
    kernel.facts.external_before_timer = Some(false);
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops,
        Stops::default(),
        "{:?}",
        composition.stops
    );
    assert_eq!(val(composition.tasks[0].jitter_release), 7);
    assert_eq!(val(composition.sources[0].jitter), 8);
    assert_eq!(val(composition.tasks[1].jitter_release), 8);
    let uart = rendered(&composition, "J_s of source `uart`");
    assert!(
        uart.ends_with(
            "+ δ + C_rel + S, the timer queued ahead (one service per release, J_k = J_k^event + 7) = 3 (the plan: \
             analytically established); Δ 5 → 8 → 8"
        ),
        "{uart}"
    );
    // Without `one-claim-per-trap`, that order is not admitted; unread, it leaves every J undeclared.
    kernel.facts.one_claim_per_trap = None;
    let composition = compose(&kernel, &platform, &application, &sources);
    assert!(
        composition.stops.unresolved.iter().any(|r| r.starts_with(
            "`one-claim-per-trap, which admits `external-before-timer` `no`` cannot be read"
        )),
        "{:?}",
        composition.stops
    );
    assert!(composition.tasks[0].jitter_release.is_none());
    assert!(
        composition.tasks[0].computation.is_some(),
        "C and CS are composed all the same"
    );
    kernel.facts.one_claim_per_trap = Some(false);
    let composition = compose(&kernel, &platform, &application, &sources);
    assert!(
        composition.stops.unresolved.is_empty(),
        "{:?}",
        composition.stops
    );
    assert!(
        composition
            .stops
            .unsupported
            .iter()
            .any(|r| r.contains("one-claim-per-trap") && r.contains("declared `no`")),
        "{:?}",
        composition.stops
    );
    // `external-before-timer` `yes` does not read `one-claim-per-trap` at all.
    kernel.facts.external_before_timer = Some(true);
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops,
        Stops::default(),
        "{:?}",
        composition.stops
    );
}

#[test]
fn a_kernel_masked_run_inside_a_call_can_be_the_longest_section() {
    // No run, one call to `spin`: CS = max(masked.spin 3, masked.completion 1) = 3; C = 2 + api.spin 2 + 2 = 6.
    let mut a = task("a", 1, v(2), 100, 100, ReleasedBy::Timer);
    a.calls = calls(&[("spin", 1)]);
    let composition = compose(&kernel(), &platform_for(1, &[]), &application(vec![a]), &[]);
    assert_eq!(val(composition.tasks[0].masked_section), 3);
    assert_eq!(val(composition.tasks[0].computation), 6);
}

// ----- The stops of §3, each with the verdict §6 gives it -----

#[test]
fn a_source_with_no_fixed_point_is_unsupported_and_what_queues_behind_it_is_named_under_the_stop() {
    // `fast` (C_s 3, T_s 5) ahead of `slow` (C_s 1, T_s 50) ahead of `late` (C_s 1, T_s 4), then the timer.
    //   L = max(CS_a + S = 2, C_rel + S = 2, 3 + 1 = 4, 2, 2, W 1) = 4; B_s = 4 + 2 = 6.
    //   fast: nothing ahead, J = 6 at once — a value, past its no-loss limit, which the variant judges.
    //   slow: ahead `fast`, charge δ + C_s + S = 5 every T_s 5: 5/5 ≥ 1, no fixed point — unsupported-profile.
    //   late and the timer: behind the stop, no value; late's B_s = 6 ≥ T_s 4 already loses an arrival.
    let (kernel, platform, application, _) = fixture_b();
    let platform = Platform {
        enabled: Some(EnabledSet {
            interrupts: vec!["fast".into(), "slow".into(), "late".into(), "timer".into()],
            from_resolved_plan: true,
        }),
        ..platform
    };
    let sources = vec![
        source("fast", 3, 5, Acknowledge::AtEntry, 1),
        source("slow", 1, 50, Acknowledge::AtEntry, 2),
        source("late", 1, 4, Acknowledge::AtEntry, 3),
    ];
    let application = application_with_task_e_released_by(&application, "slow");
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(val(composition.sources[0].jitter), 6);
    assert!(composition.sources[1].jitter.is_none());
    assert!(composition.sources[2].jitter.is_none());
    assert!(
        composition.tasks[0].jitter_release.is_none(),
        "the timer is behind the stop"
    );
    assert_eq!(
        composition.stops.unsupported[0],
        "J of source `slow` has no fixed point: what queues ahead of it can arrive at least as fast as it is \
         served (record §3), so J_s cannot be bounded below its no-loss limit (record §6, the variant's \
         condition 7); behind it, with no value: source `late`; behind it, with no value: the timer"
    );
    assert_eq!(
        composition.stops.unsupported[1],
        "source `late` can lose an arrival: its lower bound B_s = 6 already passes its no-loss limit inside its \
         separation 4 (record §3, the variant's condition 7)"
    );
    assert!(
        composition.stops.not_established.is_empty() && composition.stops.unresolved.is_empty()
    );
    let refusal = refused(composition.admit(&platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(
        refusal
            .reasons
            .iter()
            .any(|r| r.starts_with("J of source `slow` has no fixed point")),
        "{refusal}"
    );
    assert!(
        refusal
            .reasons
            .iter()
            .any(|r| r.starts_with("source `late` can lose an arrival")),
        "{refusal}"
    );
    // `fast`'s own value, 6, is past its no-loss limit too; the variant's condition 7 judges that on a complete
    // set only, and this set has two sources without a value, so the stop's verdict is the set's alone.
    assert!(
        !refusal
            .reasons
            .iter()
            .any(|r| r.contains("is not declared")),
        "an input the composition left without a value is reported by its stop alone: {refusal}"
    );
}

fn application_with_task_e_released_by(application: &Application, source: &str) -> Application {
    let mut application = application.clone();
    application.tasks[1].released_by = by_source(source);
    application
}

#[test]
fn an_iterate_past_a_sources_refusal_bound_stops_it_and_a_fixed_point_reached_is_the_value() {
    // δ = 0. `q1` (C_s 2, T_s 10) ahead of `q2` (C_s 2, acknowledged at exit). CS_a = 1.
    //   L = max(1 + 1, 1 + 1, 2 + 1, 2 + 1, 1) = 3; B_s = 3. J_q1 = 3 (< 10).
    //   q2: ahead q1, charge 0 + 2 + 1 = 3, n = ⌈(Δ + 3)/10⌉: Δ = 3 → 3 + 3 = 6.
    //   The fixed-point test comes first (6 ≠ 3), then the bound: at exit, 6 + C_s 2 + S 1 = 9 ≥ T_s.
    //     T_s = 9: past the bound before a fixed point — unsupported-profile.
    //     T_s = 10: not past; Δ = 6 → 3 + ⌈9/10⌉·3 = 6, the value; the variant then finds 9 < 10, no loss.
    let (kernel, _, mut application, _) = fixture_b();
    application.tasks[1].released_by = by_source("q1");
    application.tasks[1].separation = 10; // released on every arrival of `q1`, so T_e ≤ T_q1 (condition 5)
    application.tasks[1].deadline = 10;
    let platform = platform_for(0, &["q1", "q2"]);
    let sources = |t2: u64, acknowledge: Acknowledge| {
        vec![
            source("q1", 2, 10, Acknowledge::AtEntry, 1),
            source("q2", 2, t2, acknowledge, 2),
        ]
    };
    let stopped = compose(
        &kernel,
        &platform,
        &application,
        &sources(9, Acknowledge::AtExit),
    );
    assert_eq!(val(stopped.sources[0].jitter), 3);
    assert!(stopped.sources[1].jitter.is_none());
    assert_eq!(
        stopped.stops.unsupported[0],
        "J of source `q2` passed its refusal bound 9 before reaching a fixed point, Δ 3 → 6 (record §3), so J_s \
         cannot be bounded below its no-loss limit (record §6, the variant's condition 7); behind it, with no \
         value: the timer"
    );
    let composed = compose(
        &kernel,
        &platform,
        &application,
        &sources(10, Acknowledge::AtExit),
    );
    assert_eq!(composed.stops, Stops::default(), "{:?}", composed.stops);
    assert_eq!(val(composed.sources[1].jitter), 6);
    assert!(composed.admit(&platform).is_ok());
    // Acknowledged at entry the bound is T_s itself: 6 ≥ 6 stops, 6 < 7 is the value.
    let stopped = compose(
        &kernel,
        &platform,
        &application,
        &sources(6, Acknowledge::AtEntry),
    );
    assert!(
        stopped.stops.unsupported[0].starts_with("J of source `q2` passed its refusal bound 6"),
        "{:?}",
        stopped.stops
    );
    let composed = compose(
        &kernel,
        &platform,
        &application,
        &sources(7, Acknowledge::AtEntry),
    );
    assert_eq!(val(composed.sources[1].jitter), 6);
    assert!(composed.admit(&platform).is_ok());
}

#[test]
fn a_fixed_point_at_the_no_loss_limit_is_a_value_the_variant_refuses_not_a_stop() {
    // `uart` alone, T_s = B_s = 5: the first step returns 5 = Δ, the fixed point, before the bound is looked at.
    let (kernel, platform, application, mut sources) = fixture_b();
    sources[0].separation = v(5);
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops,
        Stops::default(),
        "{:?}",
        composition.stops
    );
    assert_eq!(val(composition.sources[0].jitter), 5);
    let refusal = refused(composition.admit(&platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(
        refusal
            .reasons
            .iter()
            .any(|r| r.starts_with("source `uart` can lose an arrival")),
        "{refusal}"
    );
}

#[test]
fn the_timer_past_the_largest_timer_period_is_not_established() {
    // Fixture B with T_a = D_a = 10: Δ_timer = 7 → 11 ≠ 7, and 11 > 10, so the timer stops; `uart` is composed.
    let (kernel, platform, mut application, sources) = fixture_b();
    application.tasks[0].separation = 10;
    application.tasks[0].deadline = 10;
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(val(composition.sources[0].jitter), 5);
    assert!(composition.tasks[0].jitter_release.is_none());
    assert_eq!(val(composition.tasks[1].jitter_release), 5);
    assert_eq!(
        composition.stops.not_established,
        vec![
            "J of the timer passed its refusal bound 10 before reaching a fixed point, Δ 7 → 11 (record §3), so \
             no single-job bound applies (record §6)"
                .to_owned()
        ]
    );
    assert!(composition.stops.unsupported.is_empty() && composition.stops.unresolved.is_empty());
    let why = not_established(composition.admit(&platform));
    assert!(
        why.starts_with("J of the timer passed its refusal bound 10"),
        "{why}"
    );
    // At T_a = 11 the iterate 11 is not past the bound, and the next step finds it the fixed point: a value, which
    // the variant then judges (its busy-period stop).
    application.tasks[0].separation = 11;
    application.tasks[0].deadline = 11;
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops,
        Stops::default(),
        "{:?}",
        composition.stops
    );
    assert_eq!(val(composition.tasks[0].jitter_release), 11);
}

#[test]
fn a_queued_sources_own_delay_widens_the_window_its_arrivals_are_counted_in() {
    // Fixture B with T_s = 12 (and T_e = 12): n_uart(Δ) = ⌈(Δ + J_uart 5)/12⌉.
    //   Δ = 7 → ⌈12/12⌉ = 1 → 7 + 4 = 11 → ⌈16/12⌉ = 2 → 7 + 8 = 15 → ⌈20/12⌉ = 2 → 15.
    // Counting arrivals from the window's start alone, ⌈Δ/12⌉, would stop at 11.
    let (kernel, platform, mut application, mut sources) = fixture_b();
    sources[0].separation = v(12);
    application.tasks[1].separation = 12;
    application.tasks[1].deadline = 12;
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops,
        Stops::default(),
        "{:?}",
        composition.stops
    );
    assert_eq!(val(composition.tasks[0].jitter_release), 15);
    let timer = composition
        .composites
        .iter()
        .find(|c| c.what == "Δ_timer (J^release of every timer-released task)")
        .expect("Δ_timer");
    assert_eq!(timer.iterates, vec![7, 11, 15, 15]);
    assert!(
        timer
            .render()
            .contains("2 × δ + C_s of source `uart` + S, queued ahead (T_s 12, J_s 5) = 4"),
        "{}",
        timer.render()
    );
}

#[test]
fn the_timer_has_no_fixed_point_only_behind_a_source_past_its_own_limit_so_unsupported_wins() {
    // `fast` (C_s 3, T_s 5) alone: L = 4, B_s = 6, J_fast = 6, a value past T_s. The timer behind it: charge
    // δ + C_s + S = 5 every 5 — no fixed point, the timer's `not-established`. A source that converged below its
    // limit cannot do this: Δ_q ≥ B/(1 − Σ_ahead) and C_q + S + δ ≤ B give Σ < 1 for what follows, so the set's
    // verdict is the source's `unsupported-profile` whenever the timer's pre-check fails.
    let (kernel, _, application, _) = fixture_b();
    let platform = platform_for(1, &["fast"]);
    let application = application_with_task_e_released_by(&application, "fast");
    let sources = vec![source("fast", 3, 5, Acknowledge::AtEntry, 1)];
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(val(composition.sources[0].jitter), 6);
    assert_eq!(
        composition.stops.not_established,
        vec![
            "J of the timer has no fixed point: what queues ahead of it can arrive at least as fast as it is \
             served (record §3), so no single-job bound applies (record §6)"
                .to_owned()
        ]
    );
    let refusal = refused(composition.admit(&platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
}

#[test]
fn an_overflow_in_c_is_the_busy_period_stop() {
    let (kernel, platform, mut application) = fixture_a();
    application.tasks[0].own_code = v(u64::MAX);
    let composition = compose(&kernel, &platform, &application, &[]);
    assert!(composition.tasks[0].computation.is_none());
    assert_eq!(
        val(composition.tasks[0].masked_section),
        7,
        "CS is composed all the same"
    );
    assert_eq!(
        composition.stops.not_established,
        vec![
            "C of task `a` overflows: the job's own cost is past what any period bounds, the variant's busy-period \
             stop (record §6)"
                .to_owned()
        ]
    );
    let why = not_established(composition.admit(&platform));
    assert!(why.starts_with("C of task `a` overflows"), "{why}");
}

#[test]
fn an_overflow_in_cs_l_or_a_base_stops_every_interrupt() {
    // CS: a run of u64::MAX plus api.mask. With a source: the source unsupported, the timer not established.
    let (kernel, platform, mut application, sources) = fixture_b();
    application.tasks[0].runs = vec![Run {
        own_code: v(u64::MAX),
        ending: Ending::Unmask,
        calls: Vec::new(),
    }];
    let composition = compose(&kernel, &platform, &application, &sources);
    assert!(composition.tasks[0].masked_section.is_none());
    assert_eq!(val(composition.tasks[1].masked_section), 1);
    assert_eq!(composition.stops.unsupported, vec!["CS of task `a` overflows, which stops source `uart` as its own overflow would: J_s cannot be bounded below its no-loss limit (record §6, the variant's condition 7)".to_owned()]);
    assert_eq!(composition.stops.not_established, vec!["CS of task `a` overflows, which stops the timer as its own overflow would: no single-job bound applies (record §6)".to_owned()]);
    assert_eq!(
        refused(composition.admit(&platform)).verdict,
        RefusalVerdict::UnsupportedProfile
    );
    // Without a source the timer's verdict is the set's.
    let (kernel, platform, mut application) = fixture_a();
    application.tasks[0].runs[0].own_code = v(u64::MAX);
    let composition = compose(&kernel, &platform, &application, &[]);
    let why = not_established(composition.admit(&platform));
    assert!(
        why.starts_with("CS of task `a` overflows, which stops the timer"),
        "{why}"
    );
    // B_timer: ρ = u64::MAX overflows the base while L fits.
    let (kernel, mut platform, application) = fixture_a();
    platform.rounding = v(u64::MAX);
    let composition = compose(&kernel, &platform, &application, &[]);
    assert_eq!(val(composition.tasks[0].masked_section), 7);
    assert!(composition
        .composites
        .iter()
        .any(|c| c.what == "L (the longest masked run)"));
    let why = not_established(composition.admit(&platform));
    assert!(
        why.starts_with("a B_x overflows, which stops the timer"),
        "{why}"
    );
}

#[test]
fn the_step_budget_and_the_pre_check_s_width_are_named_resource_limits() {
    // Fixture B within one step: `uart` reaches its fixed point in that step (5 → 5); the timer does not (7 → 11).
    let (kernel, platform, application, sources) = fixture_b();
    let composition = compose_within(&kernel, &platform, &application, &sources, 1);
    assert_eq!(val(composition.sources[0].jitter), 5);
    assert!(composition.tasks[0].jitter_release.is_none());
    assert_eq!(
        composition.stops.unresolved,
        vec![
            "J of the timer did not reach a fixed point within the step budget, Δ 7 → 11: a named resource limit \
             (record §3)"
                .to_owned()
        ]
    );
    let refusal = refused(composition.admit(&platform));
    assert_eq!(refusal.verdict, RefusalVerdict::AnalysisInconclusive);
    assert_eq!(refusal.reasons, composition.stops.unresolved);
    // Three sources whose separations are the three largest primes below 2^64: the timer's pre-check needs their
    // product, past 128 bits. Each source's own pre-check fits: the first has nothing ahead, the second one
    // separation, the third a product of two that is below 2^128.
    let (kernel, _, application, _) = fixture_b();
    let platform = platform_for(1, &["p1", "p2", "p3"]);
    let application = application_with_task_e_released_by(&application, "p1");
    let sources = vec![
        source("p1", 1, u64::MAX - 58, Acknowledge::AtEntry, 1),
        source("p2", 1, u64::MAX - 82, Acknowledge::AtEntry, 2),
        source("p3", 1, u64::MAX - 94, Acknowledge::AtEntry, 3),
    ];
    let composition = compose(&kernel, &platform, &application, &sources);
    // L = max(1 + 1, 1 + 1, 1 + 1, 1) = 2; B_s = 4. p1: 4. p2: 4 + 3·⌈(4 + 4)/T⌉ = 7. p3: 4 + 3 + 3 = 10.
    assert_eq!(val(composition.sources[0].jitter), 4);
    assert_eq!(val(composition.sources[1].jitter), 7);
    assert_eq!(val(composition.sources[2].jitter), 10);
    assert!(composition.tasks[0].jitter_release.is_none());
    assert_eq!(
        composition.stops.unresolved,
        vec![
            "the pre-check's exact sum for J of the timer does not fit 128-bit numerator and denominator: a named \
             resource limit (record §3)"
                .to_owned()
        ]
    );
    assert_eq!(
        refused(composition.admit(&platform)).verdict,
        RefusalVerdict::AnalysisInconclusive
    );
}

// ----- §2's facts and §6's conditions -----

#[test]
fn a_fact_that_cannot_be_read_leaves_what_it_gates_undeclared_and_a_no_is_outside_the_composition()
{
    let (mut kernel, platform, application) = fixture_a();
    kernel.facts.pending_taken_after_unmask = None;
    let composition = compose(&kernel, &platform, &application, &[]);
    assert!(
        composition.tasks[0].computation.is_none() && composition.tasks[0].masked_section.is_none()
    );
    assert_eq!(
        composition.stops.unresolved,
        vec!["`pending-taken-after-unmask` cannot be read, which leaves every composite undeclared (record §2)".to_owned()]
    );
    let refusal = refused(composition.admit(&platform));
    assert_eq!(refusal.verdict, RefusalVerdict::AnalysisInconclusive);
    assert_eq!(
        refusal.reasons, composition.stops.unresolved,
        "the variant's own `not declared` lines for these inputs are not repeated"
    );
    kernel.facts.pending_taken_after_unmask = Some(false);
    let composition = compose(&kernel, &platform, &application, &[]);
    assert_eq!(
        composition.stops.unsupported,
        vec!["`pending-taken-after-unmask` is declared `no`, outside what the composition covers (record §6)".to_owned()]
    );
    assert_eq!(
        refused(composition.admit(&platform)).verdict,
        RefusalVerdict::UnsupportedProfile
    );
    // A fact gating every J leaves C and CS composed.
    let (mut kernel, platform, application) = fixture_a();
    kernel.facts.external_before_timer = None;
    let composition = compose(&kernel, &platform, &application, &[]);
    assert_eq!(val(composition.tasks[0].computation), 13);
    assert!(composition.tasks[0].jitter_release.is_none());
    assert_eq!(
        composition.stops.unresolved,
        vec![
            "`external-before-timer` cannot be read, which leaves every J undeclared (record §2)"
                .to_owned()
        ]
    );
    // Each record's discipline, and the application's own fact.
    let (mut kernel, platform, application) = fixture_a();
    kernel.discipline.push(("uart-driver".into(), None));
    let composition = compose(&kernel, &platform, &application, &[]);
    assert!(
        composition.stops.unresolved[0]
            .starts_with("`runtime-discipline.uart-driver` cannot be read"),
        "{:?}",
        composition.stops
    );
    let (kernel, platform, mut application) = fixture_a();
    application.leaves_interrupt_hardware_alone = Some(false);
    let composition = compose(&kernel, &platform, &application, &[]);
    assert!(
        composition.stops.unsupported[0]
            .starts_with("`leaves-interrupt-hardware-alone` is declared `no`"),
        "{:?}",
        composition.stops
    );
}

#[test]
fn the_timers_facts_are_read_only_when_a_release_or_a_source_waits_on_the_timer() {
    // No timer-released task and sources ahead of the timer: `releases-after-initialisation` is not read.
    let (kernel, platform, application, sources) = fixture_b();
    let mut application = application_with_task_e_released_by(&application, "uart");
    application.tasks[0].released_by = by_source("uart");
    application.releases_after_initialisation = None;
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops,
        Stops::default(),
        "{:?}",
        composition.stops
    );
    assert_eq!(val(composition.tasks[0].jitter_release), 5);
    // With the timer ahead of every source it gates every J.
    let mut kernel = kernel;
    kernel.facts.external_before_timer = Some(false);
    let composition = compose(&kernel, &platform, &application, &sources);
    assert!(composition.sources[0].jitter.is_none());
    assert!(composition.stops.unresolved.iter().any(|r| r.starts_with("`releases-after-initialisation, the caller's declaration` cannot be read, which leaves J^release of every timer-released task, and every J behind the timer undeclared")), "{:?}", composition.stops);
    // A timer-released task reads `reprograms-only-in-service`.
    let (mut kernel, platform, application, sources) = fixture_b();
    kernel.facts.reprograms_only_in_service = None;
    let composition = compose(&kernel, &platform, &application, &sources);
    assert!(
        composition.tasks[0].jitter_release.is_none(),
        "the timer-released task"
    );
    assert_eq!(
        val(composition.tasks[1].jitter_release),
        5,
        "the source-released task, its source ahead of the timer"
    );
    assert_eq!(val(composition.sources[0].jitter), 5);
}

#[test]
fn the_plan_must_order_its_sources_strictly_and_leave_each_deliverable() {
    let (kernel, _, application, _) = fixture_b();
    let platform = platform_for(1, &["q1", "q2"]);
    let application = application_with_task_e_released_by(&application, "q1");
    let mut sources = vec![
        source("q1", 2, 25, Acknowledge::AtEntry, 1),
        source("q2", 2, 25, Acknowledge::AtEntry, 1),
    ];
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops.unsupported,
        vec![
            "the plan's order among sources is not strict: `q1` and `q2` share rank 1 (record §6)"
                .to_owned()
        ]
    );
    sources[1].priority = None;
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops.unresolved,
        vec!["the plan states no order among the sources: source `q2`'s rank cannot be read, which leaves every J undeclared (record §6)".to_owned()]
    );
    sources[1].priority = Some(2);
    sources[1].deliverable = None;
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops.unresolved,
        vec!["the plan does not state source `q2` deliverable, which leaves every J undeclared (record §6)".to_owned()]
    );
    sources[1].deliverable = Some(false);
    let composition = compose(&kernel, &platform, &application, &sources);
    assert!(
        composition.stops.unsupported[0]
            .starts_with("the plan leaves source `q2` enabled and undeliverable"),
        "{:?}",
        composition.stops
    );
    assert!(
        composition.sources[0].jitter.is_none(),
        "every J is undeclared"
    );
}

#[test]
fn a_sources_service_comes_from_its_record_or_from_the_caller_and_needs_a_record_either_way() {
    let (kernel, platform, application, mut sources) = fixture_b();
    sources[0].service = Service::Unanchored;
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops.unresolved,
        vec!["no record anchors `service.uart`, so none of source `uart`'s facts can be read, which leaves every J undeclared (record §6)".to_owned()]
    );
    assert!(composition.sources[0].service.is_none());
    sources[0].service = Service::Catalog(None);
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops.unresolved,
        vec![
            "C_s of source `uart` cannot be read, which leaves every J undeclared (record §6)"
                .to_owned()
        ]
    );
    // The caller's figure composes like the catalog's, and the conclusion names it as the caller's.
    sources[0].service = Service::Caller {
        cost: observed(2),
        leaves_interrupt_hardware_alone: Some(true),
    };
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops,
        Stops::default(),
        "{:?}",
        composition.stops
    );
    assert_eq!(val(composition.sources[0].jitter), 5);
    assert_eq!(
        composition.sources[0].service.unwrap().evidence,
        Evidence::ObservedMaximum
    );
    let l = composition
        .composites
        .iter()
        .find(|c| c.what == "L (the longest masked run)")
        .expect("L");
    assert_eq!(
        l.value.evidence,
        Evidence::ObservedMaximum,
        "the weakest of its parts"
    );
    assert!(
        l.terms.iter().any(|term| term
            .parts
            .iter()
            .any(|part| part.owner == Owner::Caller && part.symbol == "C_s of source `uart`")),
        "{l:?}"
    );
    assert!(composition.assumptions.iter().any(|line| line == "C_s of source `uart` = 2 is the caller's figure, beginning at the trap, for a service that runs application code, an observed maximum (record §6)"), "{:?}", composition.assumptions);
    let timer = rendered(
        &composition,
        "Δ_timer (J^release of every timer-released task)",
    );
    assert!(
        timer.contains("(an observed maximum, the weakest of its parts)"),
        "{timer}"
    );
    assert!(
        timer.contains(
            "queued ahead (T_s 25, J_s 5) = 4 (the caller's figure: an observed maximum)"
        ),
        "{timer}"
    );
    // The caller's declaration for the service's application code.
    sources[0].service = Service::Caller {
        cost: observed(2),
        leaves_interrupt_hardware_alone: None,
    };
    let composition = compose(&kernel, &platform, &application, &sources);
    assert!(composition.stops.unresolved[0].starts_with("`leaves-interrupt-hardware-alone, the caller's declaration for the application code of source `uart`'s service` cannot be read"), "{:?}", composition.stops);
    sources[0].service = Service::Caller {
        cost: None,
        leaves_interrupt_hardware_alone: Some(true),
    };
    let composition = compose(&kernel, &platform, &application, &sources);
    assert!(composition.stops.unresolved[0].starts_with("C_s of source `uart`, the caller's figure for a service whose record states `no-application-code.uart` `no` cannot be read"), "{:?}", composition.stops);
    // The source's own facts.
    let (kernel, platform, application, mut sources) = fixture_b();
    sources[0].external = None;
    sources[0].one_request_per_arrival = Some(false);
    let composition = compose(&kernel, &platform, &application, &sources);
    assert_eq!(
        composition.stops.unresolved,
        vec![
            "`external.uart` cannot be read, which leaves every J undeclared (record §2)"
                .to_owned()
        ]
    );
    assert_eq!(composition.stops.unsupported, vec!["`one-request-per-arrival.uart` is declared `no`, outside what the composition covers (record §6)".to_owned()]);
}

#[test]
fn a_primitive_the_catalog_does_not_cost_or_one_named_completion_cannot_be_composed() {
    let (kernel, platform, mut application) = fixture_a();
    application.tasks[0].calls.push(Calls {
        primitive: "dma".into(),
        count: 1,
    });
    let composition = compose(&kernel, &platform, &application, &[]);
    assert!(
        composition.tasks[0].computation.is_none() && composition.tasks[0].masked_section.is_none()
    );
    assert_eq!(
        composition.stops.unresolved,
        vec![
            "the catalog does not cost primitive `dma` (`api.dma`), which leaves C of task `a` undeclared (record §6)".to_owned(),
            "the catalog does not cost primitive `dma` (`masked.dma`), which leaves CS of task `a` undeclared (record §6)".to_owned(),
        ]
    );
    application.tasks[0].calls.pop();
    application.tasks[0].calls.push(Calls {
        primitive: "completion".into(),
        count: 1,
    });
    let composition = compose(&kernel, &platform, &application, &[]);
    assert!(composition.stops.unresolved[0].starts_with("a primitive named `completion` cannot be composed, since `masked.completion` names the completion path, which leaves C of task `a` undeclared"), "{:?}", composition.stops);
    // A part of the application's that cannot be read.
    let (kernel, platform, mut application) = fixture_a();
    application.tasks[0].runs[0].own_code = None;
    let composition = compose(&kernel, &platform, &application, &[]);
    assert_eq!(val(composition.tasks[0].computation), 13);
    assert!(composition.tasks[0].masked_section.is_none());
    assert_eq!(composition.stops.unresolved, vec!["CS^app of run 0 of task `a` cannot be read, which leaves CS of task `a` undeclared (record §6)".to_owned()]);
}

#[test]
fn the_sets_verdict_is_the_highest_precedence_among_the_stops_and_the_variants_conditions() {
    // not-established outranks analysis-inconclusive: C overflows and a J fact cannot be read.
    let (mut kernel, platform, mut application) = fixture_a();
    application.tasks[0].own_code = v(u64::MAX);
    kernel.facts.external_before_timer = None;
    let composition = compose(&kernel, &platform, &application, &[]);
    assert!(
        !composition.stops.not_established.is_empty() && !composition.stops.unresolved.is_empty()
    );
    assert!(matches!(
        composition.admit(&platform),
        Err(NoBound::NotEstablished { .. })
    ));
    // unsupported-profile outranks not-established, whether the composition or the variant finds it.
    kernel.facts.external_before_timer = Some(false);
    kernel.facts.one_claim_per_trap = Some(false);
    let composition = compose(&kernel, &platform, &application, &[]);
    assert_eq!(
        refused(composition.admit(&platform)).verdict,
        RefusalVerdict::UnsupportedProfile
    );
    let (kernel, platform, mut application) = fixture_a();
    application.tasks[0].own_code = v(u64::MAX);
    application.tasks[0].facts.suspends = Some(true);
    let composition = compose(&kernel, &platform, &application, &[]);
    let refusal = refused(composition.admit(&platform));
    assert_eq!(refusal.verdict, RefusalVerdict::UnsupportedProfile);
    assert!(
        refusal
            .reasons
            .iter()
            .any(|r| r.contains("suspends itself")),
        "{refusal}"
    );
    // The variant's own `analysis-inconclusive` reasons join the composition's.
    let (mut kernel, platform, application) = fixture_a();
    kernel.facts.external_before_timer = None;
    let platform = Platform {
        wake: None,
        ..platform
    };
    let composition = compose(&kernel, &platform, &application, &[]);
    let refusal = refused(composition.admit(&platform));
    assert_eq!(refusal.verdict, RefusalVerdict::AnalysisInconclusive);
    assert!(
        refusal
            .reasons
            .iter()
            .any(|r| r.starts_with("`external-before-timer` cannot be read")),
        "{refusal}"
    );
    assert!(
        refusal
            .reasons
            .iter()
            .any(|r| r.starts_with("W_wake (the idle wake) is not declared")),
        "{refusal}"
    );
    assert!(
        refusal.reasons.iter().any(|r| r
            .starts_with("W_wake (the idle wake) cannot be read, which leaves every J undeclared")),
        "{refusal}"
    );
    assert!(
        !refusal
            .reasons
            .iter()
            .any(|r| r.starts_with("J^release of task `a` is not declared")),
        "{refusal}"
    );
}

#[test]
fn a_composite_s_category_is_the_weakest_of_its_parts() {
    let (mut kernel, platform, application, sources) = fixture_b();
    kernel.primitives[0].call = Some(Value {
        value: 1,
        evidence: Evidence::ExternallySupplied,
    });
    let mut application = application;
    application.tasks[0].calls = calls(&[("mask", 1)]);
    application.tasks[0].own_code = Some(Value {
        value: 2,
        evidence: Evidence::Assumed,
    });
    let composition = compose(&kernel, &platform, &application, &sources);
    let a = &composition.tasks[0];
    assert_eq!(a.computation.unwrap().evidence, Evidence::Assumed);
    assert_eq!(
        a.masked_section.unwrap().evidence,
        Evidence::Analytical,
        "masked.mask and masked.completion"
    );
    assert_eq!(a.jitter_release.unwrap().evidence, Evidence::Analytical);
    let e = &composition.tasks[1];
    assert_eq!(e.computation.unwrap().evidence, Evidence::Analytical);
    let set = composition.admit(&platform).expect("admitted");
    assert!(
        set.assumptions()
            .iter()
            .any(|line| line == "C of task `a` = 5 (assumed)"),
        "{:?}",
        set.assumptions()
    );
    assert!(set.assumptions().iter().any(|line| line.starts_with("C of task `a` = 5 (assumed, the weakest of its parts): C^app = 2 (the application: assumed) + api.mask = 1 (the catalog: an externally supplied bound) + completion = 2")), "{:?}", set.assumptions());
}
