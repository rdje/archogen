//! The hosted playground simulator (leaf `M4.3.1`, `docs/decisions/decision_hosted-build.md` §3): each order §3 writes, as the
//! exact trace it makes. A job's body is data, so what `/1` cannot yet reach, preemption, masking and overrun, is held here.

use rt_core::OverrunPolicy;
use sim_playground::{
    run, Device, DeviceKind, Ended, Observer, Source, Stamped, Step, System, Task, Trigger,
};

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
        devices: &[],
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
            "0 s timer-programmed 0 s",
            "0 s pending timer",
            "0 s entry timer",
            "0 s release beat nominal 0 s",
            "0 s release chime nominal 0 s",
            "0 s timer-programmed 1/100 s",
            "0 s exit timer",
            "0 s start beat",
            "0 s complete beat",
            "0 s start chime",
            "0 s complete chime",
            "0 s idle",
            "1/100 s pending timer",
            "1/100 s entry timer",
            "1/100 s release beat nominal 1/100 s",
            "1/100 s timer-programmed 1/50 s",
            "1/100 s exit timer",
            "1/100 s start beat",
            "1/100 s complete beat",
            "1/100 s idle",
            "1/50 s pending timer",
            "1/50 s entry timer",
            "1/50 s release beat nominal 1/50 s",
            "1/50 s exit timer",
            "1/50 s start beat",
            "1/50 s complete beat",
            "1/50 s idle",
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
    // Each is stamped when it is observed, and says the instant it was due.
    assert!(
        lines.contains(&"1/25 s release high nominal 1/50 s".to_owned()),
        "{lines:#?}"
    );
    assert!(
        lines.contains(&"1/25 s fault overrun high fault nominal 1/25 s".to_owned()),
        "{lines:#?}"
    );
}

/// The lines of `lines` at `instant`, the instant's own text left off.
fn at<'l>(lines: &'l [String], instant: &str) -> Vec<&'l str> {
    lines
        .iter()
        .filter_map(|line| line.strip_prefix(instant))
        .collect()
}

#[test]
fn a_job_finishing_at_a_release_s_instant_completes_before_that_release() {
    // `ROADMAP.md` §13.4: "At a coincident completion/release instant, record completion before processing the new release".
    for body in [
        &[Step::Run(10)][..],
        &[Step::Run(10), Step::Mask, Step::Unmask][..],
        &[Step::Mask, Step::Run(10)][..],
    ] {
        let tasks = [timer("a", 1, 10, body)];
        let (ended, lines) = trace::<1>(&system(&tasks, &[], 20));
        assert_eq!(ended, Ended::Horizon, "{lines:#?}");
        let completes = lines.iter().position(|l| l == "1/10 s complete a");
        let releases = lines
            .iter()
            .position(|l| l == "1/10 s release a nominal 1/10 s");
        assert!(
            completes.is_some() && completes < releases,
            "{body:?}: {lines:#?}"
        );
        assert!(
            !lines.iter().any(|l| l.contains(" fault ")),
            "{body:?}: a job that filled its period read as an overrun: {lines:#?}"
        );
    }
}

#[test]
fn a_job_with_time_left_at_a_release_s_instant_still_overruns() {
    let body = [Step::Run(11)];
    let tasks = [timer("a", 1, 10, &body)];
    let (_, lines) = trace::<1>(&system(&tasks, &[], 20));
    assert!(
        lines.contains(&"1/10 s fault overrun a fault nominal 1/10 s".to_owned()),
        "{lines:#?}"
    );
    assert!(
        !lines.contains(&"1/10 s complete a".to_owned()),
        "{lines:#?}"
    );
}

#[test]
fn an_unmask_that_lets_an_interrupt_in_takes_it_before_the_job_s_next_step() {
    // The release arrives under the mask, so it is taken at the unmask, before the job's next instruction: here its completion,
    // at that same instant. So the task still owes that job, and the release finds it so.
    let body = [Step::Mask, Step::Run(15), Step::Unmask];
    let tasks = [timer("a", 1, 10, &body)];
    let (_, lines) = trace::<1>(&system(&tasks, &[], 20));
    let at_unmask = at(&lines, "3/20 s ");
    assert_eq!(at_unmask.first().copied(), Some("unmask a"), "{lines:#?}");
    assert_eq!(at_unmask.get(1).copied(), Some("entry timer"), "{lines:#?}");
    assert!(!at_unmask.contains(&"complete a"), "{lines:#?}");
}

#[test]
fn idle_is_stamped_on_a_transition_into_it_and_not_on_a_trap_that_returns_to_it() {
    // Nothing is ever released, so a run begins and stays in idle: one entry. A source's task runs once, and then idle again.
    let sources = [Source {
        name: "s",
        separation: Some(2),
        alarm: false,
    }];
    let tasks = [Task {
        name: "on-s",
        rank: 1,
        policy: OverrunPolicy::Fault,
        trigger: Trigger::Source(0),
        body: &[],
    }];
    let (_, lines) = trace::<1>(&system(&tasks, &sources, 3));
    assert_eq!(
        lines.iter().filter(|l| l.ends_with(" idle")).count(),
        2,
        "{lines:#?}"
    );
    // A trap that releases nothing returns to idle and stamps no entry: a timer task already faulted.
    let tasks = [
        timer("late", 1, 1, &[Step::Run(100)]),
        Task {
            name: "on-s",
            rank: 2,
            policy: OverrunPolicy::Fault,
            trigger: Trigger::Source(0),
            body: &[],
        },
    ];
    let (_, lines) = trace::<2>(&system(&tasks, &[], 4));
    assert_eq!(
        lines.iter().filter(|l| l.ends_with(" idle")).count(),
        1,
        "{lines:#?}"
    );
}

#[test]
fn a_fatal_fault_says_its_kind_its_case_and_nothing_follows_it() {
    let tasks = [timer("a", 1, 2, &[Step::Unmask])];
    let (ended, lines) = trace::<1>(&system(&tasks, &[], 4));
    assert_eq!(ended, Ended::Halted, "{lines:#?}");
    let last = lines.last().expect("a line");
    assert!(
        last.starts_with("0 s fault invariant-violated "),
        "{lines:#?}"
    );
    assert!(last.ends_with(" escalated no"), "{lines:#?}");
    assert_eq!(lines[lines.len() - 2], "0 s unmask a", "{lines:#?}");
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
            alarm: false,
        },
        Source {
            name: "b",
            separation: Some(10),
            alarm: false,
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
            alarm: false,
        },
        Source {
            name: "unstated",
            separation: None,
            alarm: false,
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
        lines.contains(&"1/100 s fault overrun late fault nominal 1/100 s".to_owned()),
        "{lines:#?}"
    );
    // The first is the task's last: every later release is discarded, and produces no event.
    assert_eq!(
        lines
            .iter()
            .filter(|l| l.contains(" fault overrun late"))
            .count(),
        1,
        "{lines:#?}"
    );
    assert!(
        !lines
            .iter()
            .any(|l| l.contains("release late nominal 1/50 s")),
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
    // One `fault` event, and no `release` event: the observation record's §2.
    assert!(
        lines.contains(&"1/50 s fault overrun late skip-late-job nominal 1/50 s".to_owned()),
        "{lines:#?}"
    );
    assert!(
        !lines.iter().any(|l| l.starts_with("1/50 s release late")),
        "{lines:#?}"
    );
    // Its later `start` is an ordinary one, the new job's.
    assert_eq!(
        lines.iter().filter(|l| l.ends_with(" start late")).count(),
        3,
        "{lines:#?}"
    );
    assert!(
        !lines.iter().any(|l| l.contains("resume late")),
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
        alarm: false,
    }];
    let first = trace::<2>(&system(&tasks, &sources, 21));
    assert_eq!(first, trace::<2>(&system(&tasks, &sources, 21)));
}

fn alarm(name: &str) -> Source<'_> {
    Source {
        name,
        separation: None,
        alarm: true,
    }
}

/// A job's read and write of a device the plan names are shown where they run, and take no time.
#[test]
fn a_job_s_access_to_a_device_is_shown_where_it_runs() {
    let devices = [Device {
        name: "dev",
        kind: DeviceKind::Block,
    }];
    let tasks = [timer(
        "job",
        1,
        10,
        &[Step::Read(0), Step::Run(2), Step::Write(0)],
    )];
    let mut system = system(&tasks, &[], 10);
    system.devices = &devices;
    let (ended, lines) = trace::<1>(&system);
    assert_eq!(ended, Ended::Horizon);
    let at = |line: &str| lines.iter().position(|said| said == line).expect(line);
    assert!(at("0 s start job") < at("0 s device-read dev"), "{lines:?}");
    assert!(
        at("0 s device-read dev") < at("1/50 s device-write dev"),
        "{lines:?}"
    );
    assert!(
        at("1/50 s device-write dev") < at("1/50 s complete job"),
        "{lines:?}"
    );
}

/// An alarm arrives where a step programs it and at no other time, as a request its service claims, and releases what it names.
#[test]
fn an_alarm_arrives_where_it_was_programmed_and_releases_its_task() {
    let sources = [alarm("wake")];
    let tasks = [
        Task {
            name: "woken",
            rank: 1,
            policy: OverrunPolicy::Fault,
            trigger: Trigger::Source(0),
            body: &[],
        },
        timer("arms", 2, 10, &[Step::Program { alarm: 0, ahead: 3 }]),
    ];
    let (ended, lines) = trace::<2>(&system(&tasks, &sources, 10));
    assert_eq!(ended, Ended::Horizon);
    for line in [
        "0 s alarm-programmed wake 3/100 s",
        "3/100 s pending alarm wake",
        "3/100 s entry alarm wake",
        "3/100 s release woken",
        "3/100 s exit alarm wake",
    ] {
        assert!(lines.iter().any(|said| said == line), "{line}: {lines:?}");
    }
    assert_eq!(
        lines
            .iter()
            .filter(|said| said.contains("pending alarm"))
            .count(),
        1,
        "{lines:?}"
    );
}

/// A value already reached arrives at the write and is never lost, and an unprogrammed alarm never arrives.
#[test]
fn a_value_already_reached_arrives_at_the_write_and_none_programmed_never() {
    let sources = [alarm("now"), alarm("never")];
    let tasks = [timer(
        "arms",
        1,
        10,
        &[Step::Program { alarm: 0, ahead: 0 }],
    )];
    let (_, lines) = trace::<1>(&system(&tasks, &sources, 10));
    assert!(
        lines.iter().any(|said| said == "0 s pending alarm now"),
        "{lines:?}"
    );
    assert!(
        lines.iter().any(|said| said == "0 s entry alarm now"),
        "{lines:?}"
    );
    assert!(
        !lines.iter().any(|said| said.contains("never")),
        "{lines:?}"
    );
}

/// Programmed inside a masked run, it waits for the unmask; each arrival is a request of its own, and a later write replaces what stood.
#[test]
fn an_alarm_waits_out_a_masked_run_each_arrival_a_request() {
    let sources = [alarm("a")];
    let body = [
        Step::Mask,
        Step::Program { alarm: 0, ahead: 1 },
        Step::Run(2),
        Step::Program { alarm: 0, ahead: 0 },
        Step::Run(1),
        Step::Unmask,
    ];
    let tasks = [timer("job", 1, 10, &body)];
    let (_, lines) = trace::<1>(&system(&tasks, &sources, 10));
    let count = |line: &str| lines.iter().filter(|said| *said == line).count();
    assert_eq!(count("1/100 s pending alarm a"), 1, "{lines:?}");
    assert_eq!(count("1/50 s pending alarm a"), 1, "{lines:?}");
    assert_eq!(count("3/100 s entry alarm a"), 2, "{lines:?}");
    assert!(
        !lines.iter().any(|said| said.starts_with("0 s entry alarm")),
        "{lines:?}"
    );

    let replaced = [
        Step::Program { alarm: 0, ahead: 5 },
        Step::Program { alarm: 0, ahead: 2 },
    ];
    let tasks = [timer("job", 1, 10, &replaced)];
    let (_, lines) = trace::<1>(&system(&tasks, &sources, 10));
    assert_eq!(
        lines
            .iter()
            .filter(|said| said.contains("pending alarm"))
            .count(),
        1,
        "{lines:?}"
    );
    assert!(
        lines.iter().any(|said| said == "1/50 s pending alarm a"),
        "{lines:?}"
    );
}

/// An alarm stands where a source stands: in the plan's rank among them, before the release timer.
#[test]
fn an_alarm_stands_in_rank_among_sources_before_the_timer() {
    let sources = [
        alarm("first"),
        Source {
            name: "second",
            separation: Some(10),
            alarm: false,
        },
    ];
    let tasks = [timer(
        "arms",
        1,
        10,
        &[Step::Program {
            alarm: 0,
            ahead: 10,
        }],
    )];
    let (_, lines) = trace::<1>(&system(&tasks, &sources, 20));
    let at = |line: &str| lines.iter().position(|said| said == line).expect(line);
    assert!(
        at("1/10 s entry alarm first") < at("1/10 s entry source second"),
        "{lines:?}"
    );
    assert!(
        at("1/10 s entry source second") < at("1/10 s entry timer"),
        "{lines:?}"
    );
}

/// A step naming what the plan does not hold, or a source that is no alarm, runs nothing.
#[test]
fn a_step_naming_what_the_plan_does_not_hold_boots_nothing() {
    let sources = [Source {
        name: "plain",
        separation: None,
        alarm: false,
    }];
    for body in [
        [Step::Read(0)],
        [Step::Write(3)],
        [Step::Program { alarm: 0, ahead: 1 }],
        [Step::Program { alarm: 9, ahead: 1 }],
    ] {
        let tasks = [timer("job", 1, 10, &body)];
        let (ended, _) = trace::<1>(&system(&tasks, &sources, 10));
        assert_eq!(ended, Ended::NotBooted);
    }
}
