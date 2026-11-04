//! The hosted playground simulator (leaf `M4.3.1`, `docs/decisions/decision_hosted-build.md` §3): each order §3 writes, as the
//! exact trace it makes. A job's body is data, so what `/1` cannot yet reach, preemption, masking and overrun, is held here.

use rt_core::OverrunPolicy;
use sim_playground::{run, Ended, Observer, Source, Stamped, Step, System, Task, Trigger};

/// Every event as the trace writes it.
#[derive(Default)]
struct Lines(Vec<String>);

impl Observer for Lines {
    fn observe(&mut self, event: Stamped<'_>) {
        self.0.push(event.to_string());
    }
}

fn trace<const N: usize>(system: &System<'_>) -> (Ended, Vec<String>) {
    let mut lines = Lines::default();
    let ended = run::<N>(system, &mut lines);
    (ended, lines.0)
}

fn system<'a>(tasks: &'a [Task<'a>], sources: &'a [Source<'a>], horizon: u64) -> System<'a> {
    System {
        name: "s",
        ticks_per_second: 100,
        horizon,
        tasks,
        sources,
        initialisation: &[],
    }
}

fn timer<'a>(name: &'a str, rank: i64, period: u64, body: &'a [Step]) -> Task<'a> {
    Task {
        name,
        rank,
        policy: OverrunPolicy::Fault,
        trigger: Trigger::Timer(period),
        body,
    }
}

#[test]
fn two_empty_jobs_on_the_timer_are_the_heartbeat_s_trace() {
    // S0's heartbeat: 10 ms and 30 ms, one hyperperiod. A job's body is empty, so it starts and completes at one instant.
    let tasks = [timer("beat", 1, 1, &[]), timer("chime", 2, 3, &[])];
    let (ended, lines) = trace::<2>(&system(&tasks, &[], 3));
    assert_eq!(ended, Ended::Horizon);
    assert_eq!(
        lines,
        [
            "0 s enabled",
            "0 s timer-programmed 0",
            "0 s pending timer",
            "0 s entry timer",
            "0 s release beat",
            "0 s release chime",
            "0 s timer-programmed 1",
            "0 s exit timer",
            "0 s start beat",
            "0 s complete beat",
            "0 s start chime",
            "0 s complete chime",
            "1/100 s pending timer",
            "1/100 s entry timer",
            "1/100 s release beat",
            "1/100 s timer-programmed 2",
            "1/100 s exit timer",
            "1/100 s start beat",
            "1/100 s complete beat",
            "1/50 s pending timer",
            "1/50 s entry timer",
            "1/50 s release beat",
            "1/50 s exit timer",
            "1/50 s start beat",
            "1/50 s complete beat",
        ]
    );
}

#[test]
fn initialisation_runs_first_in_the_plan_s_steps_and_interrupts_are_enabled_after_it() {
    let tasks = [timer("t", 1, 1, &[])];
    let steps: [&[&str]; 2] = [&["uart"], &["console", "log"]];
    let (_, lines) = trace::<1>(&System {
        initialisation: &steps,
        ..system(&tasks, &[], 1)
    });
    assert_eq!(
        lines[..4],
        [
            "0 s initialised 1 uart",
            "0 s initialised 2 console",
            "0 s initialised 2 log",
            "0 s enabled"
        ]
    );
}

#[test]
fn a_higher_priority_release_preempts_and_the_lower_job_resumes_where_it_was() {
    let low_body = [Step::Run(5)];
    let high_body = [Step::Run(1)];
    let tasks = [
        timer("high", 1, 3, &high_body),
        timer("low", 2, 100, &low_body),
    ];
    let (_, lines) = trace::<2>(&system(&tasks, &[], 8));
    let around: Vec<&str> = lines
        .iter()
        .map(String::as_str)
        .filter(|l| {
            ["start", "preempt", "resume", "complete"]
                .iter()
                .any(|w| l.contains(w))
        })
        .collect();
    assert_eq!(
        around,
        [
            "0 s start high",
            "1/100 s complete high",
            "1/100 s start low",
            "3/100 s preempt low",
            "3/100 s start high",
            "1/25 s complete high",
            "1/25 s resume low",
            "3/50 s preempt low",
            "3/50 s start high",
            "7/100 s complete high",
            "7/100 s resume low",
        ]
    );
}

#[test]
fn a_masked_run_holds_the_interrupt_pending_to_its_unmask_and_the_release_is_late() {
    // `releases-never-latched`: no service runs with the depth raised, so the timer's trap waits for the unmask.
    let low_body = [Step::Mask, Step::Run(4), Step::Unmask, Step::Run(1)];
    let tasks = [timer("high", 1, 2, &[]), timer("low", 2, 100, &low_body)];
    let (_, lines) = trace::<2>(&system(&tasks, &[], 6));
    let at = |wanted: &str| {
        lines
            .iter()
            .position(|l| l == wanted)
            .unwrap_or_else(|| panic!("no `{wanted}` in {lines:#?}"))
    };
    assert!(at("0 s mask low") < at("1/50 s pending timer"));
    assert!(at("1/50 s pending timer") < at("1/25 s unmask low"));
    assert!(at("1/25 s unmask low") < at("1/25 s entry timer"));
    // Two nominal instants passed under the mask, each release taken: the first job has not run when the second is due, so
    // the second is the first's overrun, and `high` runs under `Fault`.
    assert_eq!(
        lines.iter().filter(|l| *l == "1/25 s release high").count(),
        1,
        "{lines:#?}"
    );
    assert!(
        lines.contains(&"1/25 s faulted high".to_owned()),
        "{lines:#?}"
    );
}

#[test]
fn a_masked_job_completing_closes_its_section_and_the_interrupt_is_taken() {
    let body = [Step::Mask, Step::Run(2)];
    let tasks = [timer("high", 1, 1, &[]), timer("low", 2, 100, &body)];
    let (ended, lines) = trace::<2>(&system(&tasks, &[], 4));
    assert_eq!(ended, Ended::Horizon, "{lines:#?}");
    assert!(
        lines.contains(&"1/50 s complete low".to_owned()),
        "{lines:#?}"
    );
    assert!(
        lines.contains(&"1/50 s entry timer".to_owned()),
        "{lines:#?}"
    );
}

#[test]
fn the_routes_are_taken_in_rank_order_and_all_before_the_timer() {
    let sources = [
        Source {
            name: "a",
            separation: Some(10),
        },
        Source {
            name: "b",
            separation: Some(10),
        },
    ];
    let tasks = [
        Task {
            name: "on-b",
            rank: 1,
            policy: OverrunPolicy::Fault,
            trigger: Trigger::Source(1),
            body: &[],
        },
        timer("tick", 2, 10, &[]),
    ];
    let (_, lines) = trace::<2>(&system(&tasks, &sources, 1));
    let entries: Vec<&str> = lines
        .iter()
        .map(String::as_str)
        .filter(|l| l.contains("entry"))
        .collect();
    assert_eq!(
        entries,
        [
            "0 s entry source a",
            "0 s entry source b",
            "0 s entry timer"
        ]
    );
    // The source's own task, released by the source's service.
    let at = |wanted: &str| lines.iter().position(|l| l == wanted).expect(wanted);
    assert!(at("0 s entry source b") < at("0 s release on-b"));
    assert!(at("0 s release on-b") < at("0 s exit source b"));
}

#[test]
fn a_source_arrives_at_its_stated_separation_and_one_stating_none_never() {
    let sources = [
        Source {
            name: "stated",
            separation: Some(2),
        },
        Source {
            name: "unstated",
            separation: None,
        },
    ];
    let tasks = [Task {
        name: "t",
        rank: 1,
        policy: OverrunPolicy::Fault,
        trigger: Trigger::Source(0),
        body: &[],
    }];
    let (_, lines) = trace::<1>(&system(&tasks, &sources, 5));
    let pending: Vec<&str> = lines
        .iter()
        .map(String::as_str)
        .filter(|l| l.contains("pending"))
        .collect();
    assert_eq!(
        pending,
        [
            "0 s pending source stated",
            "1/50 s pending source stated",
            "1/25 s pending source stated"
        ]
    );
    assert!(!lines.iter().any(|l| l.contains("unstated")));
}

#[test]
fn an_overrun_under_fault_leaves_the_schedule_and_the_others_go_on() {
    let tasks = [
        timer("late", 1, 1, &[Step::Run(3)]),
        timer("other", 2, 2, &[]),
    ];
    let (ended, lines) = trace::<2>(&system(&tasks, &[], 4));
    assert_eq!(ended, Ended::Horizon, "{lines:#?}");
    assert!(
        lines.contains(&"1/100 s faulted late".to_owned()),
        "{lines:#?}"
    );
    assert!(
        lines.iter().any(|l| l.contains("start other")),
        "{lines:#?}"
    );
}

#[test]
fn an_overrun_under_skip_late_job_abandons_it_and_the_release_is_the_next_job() {
    let tasks = [Task {
        policy: OverrunPolicy::SkipLateJob,
        ..timer("late", 1, 2, &[Step::Run(3)])
    }];
    let (_, lines) = trace::<1>(&system(&tasks, &[], 5));
    assert!(
        lines.contains(&"1/50 s job-skipped late".to_owned()),
        "{lines:#?}"
    );
    assert!(
        lines.contains(&"1/50 s release late".to_owned()),
        "{lines:#?}"
    );
}

#[test]
fn tasks_not_in_rank_order_or_of_one_rank_do_not_boot() {
    let out_of_order = [timer("b", 2, 1, &[]), timer("a", 1, 1, &[])];
    assert_eq!(
        trace::<2>(&system(&out_of_order, &[], 1)),
        (Ended::NotBooted, vec![])
    );
    let one_rank = [timer("a", 1, 1, &[]), timer("b", 1, 1, &[])];
    assert_eq!(
        trace::<2>(&system(&one_rank, &[], 1)),
        (Ended::NotBooted, vec![])
    );
    // And a table of another length than the run was built for.
    assert_eq!(
        trace::<3>(&system(&one_rank, &[], 1)),
        (Ended::NotBooted, vec![])
    );
}

#[test]
fn one_input_has_one_trace() {
    let low_body = [Step::Run(5)];
    let tasks = [
        timer("high", 1, 3, &[Step::Run(1)]),
        timer("low", 2, 7, &low_body),
    ];
    let sources = [Source {
        name: "s",
        separation: Some(4),
    }];
    let first = trace::<2>(&system(&tasks, &sources, 21));
    assert_eq!(first, trace::<2>(&system(&tasks, &sources, 21)));
}
