//! The release alarm's design held against a timeline (`docs/decisions/decision_release-alarm.md` §2; `docs/decisions/decision_executable-design-reviews.md`). Nothing of the design is code yet, and its
//! reviews' counterexamples were worked by hand, so the claims the analysis is to be told are run here, over random sets and every way a job may program an alarm among them.
//!
//! `timeline` is the variant's model by unit steps, with what a realization has that a paper timeline leaves out: each masked run contiguous with its decided switch, as `L` reads it; `δ`, `γ` and a
//! compare's resolution; `W_wake`; several alarms; tasks released by an alarm; and a read of the time apart from a write's landing, with the floor computed from what code can know at that read. It came from
//! the design's third reader, whose finding was that the first model held the floor by a last arrival no code has. It is no realization. Each sweep is seeded, so a failure names a set and repeats.

#[path = "alarm_model/timeline.rs"]
mod timeline;

use rt_analysis::compose::{
    compose, Application, Calls, Kernel, KernelFacts, Primitive, Run as JobRun, Service,
    SourceParts, TaskParts,
};
use rt_analysis::runtime::{
    analyze, Acknowledge, Deferred, EnabledSet, Evidence, Outcome, Platform, PlatformFacts,
    ReleasedBy, TaskFacts, Value,
};
use timeline::{AlarmSpec, Device, Know, LagModel, Plat, Run, TaskSpec};

fn v(value: u64) -> Option<Value> {
    Some(Value {
        value,
        evidence: Evidence::Analytical,
    })
}

fn platform(p: Plat, alarms: &[AlarmSpec]) -> Platform {
    let mut interrupts: Vec<String> = (0..alarms.len()).map(|a| format!("alarm{a}")).collect();
    interrupts.push("timer".to_owned());
    Platform {
        switch: v(p.s),
        wake: v(p.wake),
        preemption_delay: v(p.gamma),
        timer_service: v(p.c_rel),
        rounding: v(p.res - 1),
        delivery: v(p.delta),
        facts: PlatformFacts {
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
        },
        enabled: Some(EnabledSet {
            interrupts,
            from_resolved_plan: true,
        }),
    }
}

fn kernel(p: Plat) -> Kernel {
    let primitive = |name: &str, cost: u64| Primitive {
        name: name.to_owned(),
        call: v(cost),
        masked: v(cost),
    };
    Kernel {
        primitives: vec![
            primitive("mask", 0),
            primitive("unmask", 0),
            primitive("arm", p.arm),
        ],
        completion: v(p.completion),
        masked_completion: v(p.completion),
        facts: KernelFacts {
            reprograms_only_in_service: Some(true),
            pending_taken_after_unmask: Some(true),
            releases_never_latched: Some(true),
            primitives_out_of_line: Some(true),
            external_before_timer: Some(p.external_first),
            one_claim_per_trap: Some(true),
            no_empty_claim: Some(true),
            starts_by_transition: Some(true),
            one_external_controller: Some(true),
        },
        discipline: vec![("rt-core".to_owned(), Some(true))],
    }
}

/// The bound of each task in the order given, and each alarm's `J_s`; `None` where the analysis does not hold the set.
fn bounds(tasks: &[TaskSpec], alarms: &[AlarmSpec], p: Plat) -> Option<(Vec<i64>, Vec<i64>)> {
    let mut parts: Vec<TaskParts> = Vec::new();
    for t in tasks {
        let n = if t.arms.is_empty() {
            0
        } else {
            t.writes_per_job
        };
        parts.push(TaskParts {
            id: t.name.clone(),
            priority: t.rank,
            separation: t.period,
            deadline: t.period,
            jitter_event: 0,
            released_by: match t.released_by {
                None => ReleasedBy::Timer,
                Some(a) => ReleasedBy::Source {
                    source: format!("alarm{a}"),
                    every_arrival: true,
                },
            },
            facts: TaskFacts {
                suspends: Some(false),
                locks_scheduler: Some(false),
                shares_outside_sections: Some(false),
            },
            own_code: v(t.own),
            calls: if n == 0 {
                Vec::new()
            } else {
                vec![Calls {
                    primitive: "arm".to_owned(),
                    count: u32::try_from(n).ok()?.into(),
                }]
            },
            calls_evidence: Some(Evidence::Analytical),
            runs: Vec::<JobRun>::new(),
            runs_evidence: Some(Evidence::Analytical),
        });
    }
    let sources: Vec<SourceParts> = alarms
        .iter()
        .enumerate()
        .map(|(a, s)| SourceParts {
            id: format!("alarm{a}"),
            separation: v(s.t_s),
            acknowledge: Some(Acknowledge::AtEntry),
            deferred: Some(Deferred::Nothing),
            priority: Some(s.rank),
            deliverable: Some(true),
            service: Service::Catalog(v(s.c_s)),
            external: Some(true),
            one_request_per_arrival: Some(true),
        })
        .collect();
    let plat = platform(p, alarms);
    let composition = compose(
        &kernel(p),
        &plat,
        &Application {
            tasks: parts,
            leaves_interrupt_hardware_alone: Some(true),
            releases_after_initialisation: Some(true),
        },
        &sources,
    );
    let mut j = Vec::new();
    for a in 0..alarms.len() {
        let id = format!("alarm{a}");
        let found = composition.sources.iter().find(|s| s.id == id)?;
        j.push(i64::try_from(found.jitter?.value).ok()?);
    }
    let set = composition.admit(&plat).ok()?;
    let mut order: Vec<usize> = (0..tasks.len()).collect();
    order.sort_by_key(|i| tasks[*i].rank);
    let mut found = vec![0_i64; tasks.len()];
    for (at, outcome) in analyze(&set).into_iter().enumerate() {
        match outcome {
            Outcome::Holds { response, .. } => found[order[at]] = i64::try_from(response).ok()?,
            _ => return None,
        }
    }
    Some((found, j))
}

struct Rng(u64);
impl Rng {
    fn next(&mut self) -> u64 {
        let mut x = self.0;
        x ^= x << 13;
        x ^= x >> 7;
        x ^= x << 17;
        self.0 = x;
        x
    }
    fn pick<T: Copy>(&mut self, of: &[T]) -> T {
        of[usize::try_from(self.next() % of.len() as u64).expect("small")]
    }
    fn below(&mut self, n: u64) -> u64 {
        self.next() % n
    }
}

fn task(
    name: &str,
    rank: i64,
    period: u64,
    own: u64,
    arms: Vec<(usize, i64)>,
    writes: usize,
    released_by: Option<usize>,
) -> TaskSpec {
    TaskSpec {
        name: name.to_owned(),
        rank,
        period,
        own,
        arms,
        writes_per_job: writes,
        released_by,
    }
}

fn random_set(
    r: &mut Rng,
    two_alarms: bool,
    by_alarm: bool,
    overheads: bool,
    lag_max: u64,
) -> (Vec<TaskSpec>, Vec<AlarmSpec>, Plat) {
    let p = Plat {
        s: r.pick(&[1, 1, 2, 3]),
        c_rel: r.pick(&[1, 2, 4]),
        completion: r.pick(&[1, 2, 3]),
        arm: (2 * lag_max + 1).max(r.pick(&[1, 2, 3, 5])),
        delta: if overheads { r.pick(&[0, 1, 2]) } else { 0 },
        gamma: if overheads { r.pick(&[0, 1, 2, 3]) } else { 0 },
        res: if overheads {
            r.pick(&[1, 2, 4, 5, 8, 16])
        } else {
            1
        },
        wake: r.pick(&[0, 1, 2]),
        external_first: r.below(2) == 0,
    };
    let n_alarms = if two_alarms { 2 } else { 1 };
    let mut alarms = Vec::new();
    for a in 0..n_alarms {
        let c_s = r.pick(&[2, 3, 5, 8, 12]).max(1 + 2 * lag_max);
        let t_s = r.pick(&[8, 12, 17, 25, 40, 60, 100]);
        // The record's `write`, whose two cells write one sum: the floor fits one write, so what one write reaches is at least that.
        let write = charge(
            "write",
            Device::Compare,
            Inputs {
                t_s: i64::try_from(t_s).expect("small"),
                lag: i64::try_from(lag_max).expect("small"),
                rho: i64::try_from(p.res - 1).expect("small"),
                j_s: 0,
                c_s: 0,
                p: 0,
                reach: 0,
                w: 0,
            },
        );
        alarms.push(AlarmSpec {
            c_s,
            t_s,
            reach: r.pick(&[7, 15, 30, 10_000]).max(write),
            rank: i64::try_from(a).expect("small") + 1,
            floored: true,
            read_at: 1,
            wake: None,
            calls: None,
        });
    }
    let n_tasks = 2 + usize::try_from(r.below(3)).expect("small");
    let mut tasks = Vec::new();
    for at in 0..n_tasks {
        let period = r.pick(&[30, 40, 50, 64, 75, 100, 150, 200, 300, 400]);
        let own = 1 + r.below((period / 4).max(1));
        let programs = at == 0 || r.below(2) == 0;
        let mut arms = Vec::new();
        let i_period = i64::try_from(period).expect("small");
        if programs {
            let len = 1 + usize::try_from(r.below(7)).expect("small");
            for _ in 0..len {
                let offset = match r.below(5) {
                    0 => -i64::try_from(r.below(20)).expect("small"),
                    1 => i64::try_from(r.below(5)).expect("small"),
                    2 => i64::try_from(r.below(u64::try_from(i_period).expect("small")))
                        .expect("small"),
                    3 => i_period + i64::try_from(r.below(5)).expect("small"),
                    _ => 2 * i_period + i64::try_from(r.below(40)).expect("small"),
                };
                let a = usize::try_from(r.below(n_alarms as u64)).expect("small");
                arms.push((a, offset));
            }
        }
        let writes = if programs {
            1 + usize::try_from(r.below(3)).expect("small")
        } else {
            0
        };
        tasks.push(task(
            &format!("t{at}"),
            i64::try_from(at).expect("small") + 1,
            period,
            own,
            arms,
            writes,
            None,
        ));
    }
    // Ranks in a random order.
    for at in (1..tasks.len()).rev() {
        let other = usize::try_from(r.below(at as u64 + 1)).expect("small");
        let (ra, rb) = (tasks[at].rank, tasks[other].rank);
        tasks[at].rank = rb;
        tasks[other].rank = ra;
    }
    if by_alarm {
        // A task released on every arrival of the last alarm, something else programming it.
        let a = n_alarms - 1;
        let period = alarms[a].t_s;
        let rank = i64::try_from(tasks.len()).expect("small") + 1;
        tasks.push(task(
            "released",
            if r.below(2) == 0 { 0 } else { rank },
            period,
            1 + r.below((period / 4).max(1)),
            Vec::new(),
            0,
            Some(a),
        ));
        tasks[0].arms = vec![(a, 3), (a, 50)];
        tasks[0].writes_per_job = 1;
        if tasks[0].released_by.is_some() {
            tasks[0].released_by = None;
        }
    }
    (tasks, alarms, p)
}

/// What `delivery` charges, by the record's §2: what lies between the programmed time and the match, then `J_s` and the service. A write in a service lands inside it, and the floor is read off an arrival
/// the code bounds from above, by the lag on a compare and by twice that on a delay, which also lands late itself.
fn delivery(device: Device, lag: i64, rho: i64, floor: i64, j_s: i64, c_s: i64) -> i64 {
    charge(
        "delivery",
        device,
        Inputs {
            t_s: floor,
            lag,
            rho,
            j_s,
            c_s,
            p: 0,
            reach: 0,
            w: 0,
        },
    )
}

/// The record, whose §2 table of sums this reads and runs: a charge here is the cell there, so the two cannot differ.
const RECORD: &str = include_str!("../../../docs/decisions/decision_release-alarm.md");

#[derive(Clone, Copy)]
struct Inputs {
    t_s: i64,
    lag: i64,
    rho: i64,
    j_s: i64,
    c_s: i64,
    p: i64,
    reach: i64,
    w: i64,
}

/// What the record's table writes for `sum` on `device`.
fn cell(sum: &str, device: Device) -> &'static str {
    let (_, table) = RECORD
        .split_once("<!-- machine-read: alarm-sums -->")
        .expect("the record's table of sums");
    let row = table
        .lines()
        .find(|line| line.starts_with(&format!("| `{sum}` |")))
        .expect("the sum's row");
    let cells: Vec<&str> = row.split('|').map(str::trim).collect();
    cells[match device {
        Device::Compare => 3,
        Device::Delay => 4,
    }]
    .trim_matches('`')
}

/// A cell, read as it is written: sums of names, `2λ` and `3ρ`, and `max(…, …)`.
struct Cell<'c> {
    rest: &'c str,
    device: Device,
    inputs: Inputs,
}

impl Cell<'_> {
    fn eat(&mut self, what: &str) -> bool {
        match self.rest.trim_start().strip_prefix(what) {
            Some(rest) => {
                self.rest = rest;
                true
            }
            None => false,
        }
    }

    fn sum(&mut self) -> i64 {
        let mut total = self.term();
        while self.eat("+") {
            total += self.term();
        }
        total
    }

    fn term(&mut self) -> i64 {
        if self.eat("max(") {
            let mut most = self.sum();
            while self.eat(",") {
                most = most.max(self.sum());
            }
            assert!(self.eat(")"), "a `max` unclosed at `{}`", self.rest);
            return most;
        }
        let times = if self.eat("2") {
            2
        } else if self.eat("3") {
            3
        } else {
            1
        };
        let i = self.inputs;
        for (name, value) in [
            ("J_s", i.j_s),
            ("C_s", i.c_s),
            ("T_s", i.t_s),
            ("λ", i.lag),
            ("ρ", i.rho),
            ("P", i.p),
            ("R", i.reach),
            ("W", i.w),
        ] {
            if self.eat(name) {
                return times * value;
            }
        }
        assert!(self.eat("delivery"), "no such input at `{}`", self.rest);
        charge("delivery", self.device, i)
    }
}

/// `sum` on `device`, as the record's table writes it.
fn charge(sum: &str, device: Device, inputs: Inputs) -> i64 {
    let mut reader = Cell {
        rest: cell(sum, device),
        device,
        inputs,
    };
    let value = reader.sum();
    assert!(
        reader.rest.trim().is_empty(),
        "`{sum}`'s cell ends in `{}`",
        reader.rest
    );
    value
}

/// That the table reads as it is written, by figures worked by hand, so a sweep over it is no sweep over a misreading.
#[test]
fn the_record_s_sums_read_as_they_are_written() {
    let i = Inputs {
        t_s: 10,
        lag: 1,
        rho: 3,
        j_s: 5,
        c_s: 2,
        p: 64,
        reach: 100,
        w: 9,
    };
    assert_eq!(charge("delivery", Device::Compare, i), 10 + 1 + 6 + 5 + 2);
    assert_eq!(charge("delivery", Device::Delay, i), 10 + 2 + 9 + 5 + 2);
    assert_eq!(charge("update", Device::Compare, i), 64 + 3 + 5 + 2);
    assert_eq!(charge("update", Device::Delay, i), 64 + 1 + 6 + 5 + 2);
    assert_eq!(charge("room", Device::Compare, i), 100 + 3 + 24 + 9);
    assert_eq!(charge("room", Device::Delay, i), 100 + 3 + 28 + 9);
    assert_eq!(charge("write", Device::Compare, i), 10 + 1 + 6);
    assert_eq!(charge("write", Device::Delay, i), 10 + 1 + 6);
}

/// Random sets the analysis holds, each run under each pattern of lags: no arrivals under the floor, none lost, no response past its bound, no deadline delivered past `delivery`. How many sets and
/// timelines were judged.
#[allow(clippy::too_many_arguments)]
fn sweep(
    seed: u64,
    sets: u32,
    two: bool,
    by_alarm: bool,
    overheads: bool,
    know: Know,
    device: Device,
    lag: u64,
) -> (u32, u32) {
    let mut r = Rng(seed);
    let (mut judged, mut timelines, mut tries) = (0_u32, 0_u32, 0_u64);
    let lag_i = i64::try_from(lag).expect("small");
    while judged < sets && tries < 400_000 {
        tries += 1;
        let (tasks, alarms, p) = random_set(&mut r, two, by_alarm, overheads, lag);
        let Some((bound, j)) = bounds(&tasks, &alarms, p) else {
            continue;
        };
        judged += 1;
        for pattern in [vec![lag], vec![0], vec![0, lag], vec![lag, 0, 0]] {
            let run = Run {
                tasks: &tasks,
                alarms: &alarms,
                plat: p,
                know,
                device,
                lag: LagModel {
                    max: lag,
                    pattern,
                    past: 0,
                },
                horizon: 8_000,
            }
            .go();
            timelines += 1;
            let said = format!("{tasks:?} {alarms:?} {p:?} lag {lag}");
            assert_eq!(run.merged, 0, "an arrival lost: {said}");
            if know == Know::WhatItReadAndWrote {
                assert_eq!(run.programmed_past_reach, 0, "past its reach: {said}");
            }
            for (task, worst) in run.worst.iter().enumerate() {
                assert!(
                    *worst <= bound[task],
                    "task {task} took {worst} against a bound of {}: {said}",
                    bound[task]
                );
            }
            let rho = i64::try_from(p.res - 1).expect("small");
            for (a, alarm) in alarms.iter().enumerate() {
                let floor = i64::try_from(alarm.t_s).expect("small");
                let c_s = i64::try_from(alarm.c_s).expect("small");
                assert!(
                    run.least_separation(a) >= floor,
                    "arrivals {} apart against a floor of {floor}: {said}",
                    run.least_separation(a)
                );
                let charge = delivery(device, lag_i, rho, floor, j[a], c_s);
                assert!(
                    run.latest_delivery[a] <= charge,
                    "delivered {} late against {charge}: {said}",
                    run.latest_delivery[a]
                );
                assert!(
                    run.oldest_pending[a] <= charge,
                    "pending {} past its time against {charge}: {said}",
                    run.oldest_pending[a]
                );
            }
            if lag == 0 {
                break;
            }
        }
    }
    (judged, timelines)
}

#[test]
fn with_the_read_and_the_landing_at_one_instant_nothing_breaks_whatever_a_job_programs() {
    assert_eq!(
        sweep(
            0x1111_9BDF_0246_8ACF,
            60,
            false,
            false,
            false,
            Know::Omniscient,
            Device::Compare,
            0
        ),
        (60, 60)
    );
    // With `δ`, `ρ` and `γ` not zero, two alarms, and a task an alarm releases.
    assert_eq!(
        sweep(
            0x2222_9BDF_0246_8ACF,
            60,
            false,
            false,
            true,
            Know::Omniscient,
            Device::Compare,
            0
        ),
        (60, 60)
    );
    assert_eq!(
        sweep(
            0x3333_9BDF_0246_8ACF,
            40,
            true,
            false,
            true,
            Know::Omniscient,
            Device::Compare,
            0
        ),
        (40, 40)
    );
    assert_eq!(
        sweep(
            0x4444_9BDF_0246_8ACF,
            40,
            false,
            true,
            true,
            Know::Omniscient,
            Device::Compare,
            0
        ),
        (40, 40)
    );
}

/// What code can know at a write is what it read, and what it wrote. So it bounds the last arrival from above and leaves a compare that may raise before the landing, whose service picks up what is pending.
#[test]
fn with_what_code_can_know_the_floor_holds_at_each_lag_on_a_level_compare() {
    for lag in [1_u64, 2, 4] {
        assert_eq!(
            sweep(
                0x1357_9BDF_0246_8ACF + lag,
                40,
                false,
                false,
                false,
                Know::WhatItReadAndWrote,
                Device::Compare,
                lag
            ),
            (40, 160)
        );
        assert_eq!(
            sweep(
                0x2357_9BDF_0246_8ACF + lag,
                20,
                true,
                false,
                true,
                Know::WhatItReadAndWrote,
                Device::Compare,
                lag
            ),
            (20, 80)
        );
    }
}

/// Tasks an alarm releases, under the protocol: their releases follow its arrivals, and the floor is what separates them.
#[test]
fn tasks_an_alarm_releases_hold_under_the_protocol() {
    for device in [Device::Compare, Device::Delay] {
        assert_eq!(
            sweep(
                0x5357_9BDF_0246_8ACF,
                20,
                true,
                true,
                true,
                Know::WhatItReadAndWrote,
                device,
                2
            ),
            (20, 80)
        );
    }
}

/// A delay lands late by its lag, which a compare does not, and the code bounds its arrival from above by that much more: `delivery` holds twice the lag there, and the separation holds as it does.
#[test]
fn on_a_delay_only_device_the_floor_holds_and_delivery_costs_twice_the_lag() {
    for lag in [1_u64, 2, 4] {
        assert_eq!(
            sweep(
                0x3357_9BDF_0246_8ACF + lag,
                40,
                false,
                false,
                false,
                Know::WhatItReadAndWrote,
                Device::Delay,
                lag
            ),
            (40, 160)
        );
        assert_eq!(
            sweep(
                0x4357_9BDF_0246_8ACF + lag,
                20,
                false,
                false,
                true,
                Know::WhatItReadAndWrote,
                Device::Delay,
                lag
            ),
            (20, 80)
        );
    }
}

fn plain(s: u64, c_rel: u64, completion: u64, arm: u64, wake: u64, external_first: bool) -> Plat {
    Plat {
        s,
        c_rel,
        completion,
        arm,
        delta: 0,
        gamma: 0,
        res: 1,
        wake,
        external_first,
    }
}

fn alarm(c_s: u64, t_s: u64, reach: i64, floored: bool) -> AlarmSpec {
    AlarmSpec {
        c_s,
        t_s,
        reach,
        rank: 1,
        floored,
        read_at: 1,
        wake: None,
        calls: None,
    }
}

fn once(
    tasks: &[TaskSpec],
    alarms: &[AlarmSpec],
    plat: Plat,
    know: Know,
    device: Device,
    lag: LagModel,
    horizon: i64,
) -> timeline::Measured {
    Run {
        tasks,
        alarms,
        plat,
        know,
        device,
        lag,
        horizon,
    }
    .go()
}

/// With no floor a rate says nothing of a separation: the review's chain, whose last segment arrives just after the re-arm, holds on paper at the table's `T_s` and misses on the timeline. With the floor it holds.
#[test]
fn with_no_floor_a_chain_passes_the_analysis_and_misses() {
    let tasks = vec![
        task("programs", 1, 125, 1, vec![(0, 125)], 1, None),
        task("other", 2, 2_500, 1_999, Vec::new(), 0, None),
    ];
    let p = plain(1, 1, 1, 2, 1, true);
    let at_once = || LagModel {
        max: 0,
        pattern: vec![0],
        past: 0,
    };
    let unfloored = [alarm(10, 100, 100, false)];
    let (bound, _) =
        bounds(&tasks, &unfloored, p).expect("the analysis holds at the separation the table gave");
    assert_eq!(bound[1], 2_406);
    let run = once(
        &tasks,
        &unfloored,
        p,
        Know::Omniscient,
        Device::Compare,
        at_once(),
        26_000,
    );
    assert_eq!(run.least_separation(0), 10);
    assert_eq!(
        run.worst[1], 2_989,
        "past its bound, and its deadline of 2500"
    );
    let floored = [alarm(10, 100, 100, true)];
    let run = once(
        &tasks,
        &floored,
        p,
        Know::Omniscient,
        Device::Compare,
        at_once(),
        26_000,
    );
    assert_eq!(run.least_separation(0), 100);
    assert_eq!(run.worst[1], 2_374, "inside its bound");
}

/// A floor read off the value last written is the first thing one would write, and no code knows more than that without the protocol. At a lag it does not hold, and the bound falls with it. The second set stands within the protocol's own conditions, a masked run holding both lags and a reach at `write`, and there the protocol holds what this breaks.
#[test]
fn a_floor_off_the_value_written_breaks_at_a_lag_and_the_protocol_holds() {
    let tasks = vec![
        task(
            "t0",
            1,
            200,
            20,
            vec![(0, 95), (0, 9), (0, 409), (0, 425)],
            1,
            None,
        ),
        task("t1", 2, 75, 1, Vec::new(), 0, None),
    ];
    let alarms = [alarm(5, 17, 7, true)];
    let p = plain(2, 2, 1, 5, 0, false);
    let (bound, _) = bounds(&tasks, &alarms, p).expect("the analysis holds it at the floor");
    assert_eq!(bound, [63, 74]);
    let late = || LagModel {
        max: 4,
        pattern: vec![4],
        past: 0,
    };
    let run = once(
        &tasks,
        &alarms,
        p,
        Know::ValueWritten,
        Device::Compare,
        late(),
        12_000,
    );
    assert_eq!(run.least_separation(0), 4);
    assert_eq!(run.worst, [104, 83]);

    let tasks = vec![
        task("t0", 2, 150, 26, vec![(0, -17)], 2, None),
        task(
            "t1",
            1,
            200,
            10,
            vec![(0, 412), (0, 202), (0, 148), (0, -6)],
            1,
            None,
        ),
    ];
    let alarms = [alarm(9, 40, 44, true)];
    let p = plain(3, 4, 1, 9, 0, false);
    let (bound, _) = bounds(&tasks, &alarms, p).expect("the analysis holds it at the floor");
    let run = once(
        &tasks,
        &alarms,
        p,
        Know::ValueWritten,
        Device::Compare,
        late(),
        12_000,
    );
    assert_eq!(run.least_separation(0), 36);
    let run = once(
        &tasks,
        &alarms,
        p,
        Know::WhatItReadAndWrote,
        Device::Compare,
        late(),
        12_000,
    );
    assert_eq!(run.least_separation(0), 40);
    assert_eq!(run.programmed_past_reach, 0);
    assert!(run.worst[0] <= bound[0] && run.worst[1] <= bound[1]);
}

/// A chain's segment is an arrival and delivers nothing, so it re-arms and releases nothing: a job hears of the deadline it asked for, and not of the way there.
#[test]
fn a_chain_s_segment_re_arms_and_releases_nothing() {
    let tasks = vec![
        task("programs", 1, 400, 2, vec![(0, 200)], 1, None),
        task("released", 2, 20, 1, Vec::new(), 0, Some(0)),
    ];
    let alarms = [AlarmSpec {
        c_s: 3,
        t_s: 20,
        reach: 30,
        rank: 1,
        floored: true,
        read_at: 1,
        wake: None,
        calls: None,
    }];
    let p = plain(1, 1, 1, 1, 0, true);
    assert!(bounds(&tasks, &alarms, p).is_some());
    let run = once(
        &tasks,
        &alarms,
        p,
        Know::WhatItReadAndWrote,
        Device::Compare,
        LagModel {
            max: 0,
            pattern: vec![0],
            past: 0,
        },
        380,
    );
    assert_eq!(run.arrivals[0], [35, 66, 97, 128, 159, 190, 210]);
    assert_eq!(run.released[0].len(), 1, "{:?}", run.released[0]);
    assert!(run.released[0][0] >= 200, "{:?}", run.released[0]);
}

/// Where the time base is the counter itself, an arrival a whole wrap gone reads as recent, and its floor stands within its own window of a read, on a compare as on a delay. So `write` holds that window, and the narrower sum of round 7 does not.
#[test]
fn an_arrival_a_wrap_gone_stands_a_write_no_farther_than_its_cell() {
    for (modulus, t_s, lag, res) in [
        (256_i64, 10_i64, 5_i64, 1_i64),
        (256, 10, 5, 4),
        (1024, 64, 8, 16),
        (4096, 100, 40, 32),
    ] {
        let rho = res - 1;
        let i = Inputs {
            t_s,
            lag,
            rho,
            j_s: 0,
            c_s: 0,
            p: 0,
            reach: 0,
            w: 0,
        };
        let window = t_s + lag + rho;
        let mut most = 0;
        for last in 0..modulus {
            for read in (0..modulus).step_by(usize::try_from(res).expect("small")) {
                let ahead = (last + t_s - read).rem_euclid(modulus);
                if ahead <= window {
                    let value = (read + ahead + res - 1).div_euclid(res) * res;
                    most = most.max(value - read);
                }
            }
        }
        let said = format!("modulus {modulus}, T_s {t_s}, lag {lag}, resolution {res}");
        for device in [Device::Compare, Device::Delay] {
            assert!(most <= charge("write", device, i), "{most}: {said}");
        }
        assert!(
            most + rho >= charge("write", Device::Compare, i),
            "wide: {most}: {said}"
        );
        assert!(most > t_s + 2 * rho, "no lag in it: {most}: {said}");
    }
}

/// One deadline per job, 100 apart on average and 50 at the closest: a rate of one per job is no separation.
#[test]
fn a_rate_is_no_separation() {
    let tasks = vec![
        task("programs", 1, 100, 1, vec![(0, 50), (0, 100)], 1, None),
        task("other", 2, 300, 60, Vec::new(), 0, None),
    ];
    let lag = LagModel {
        max: 0,
        pattern: vec![0],
        past: 0,
    };
    let run = once(
        &tasks,
        &[alarm(10, 100, 10_000, false)],
        plain(1, 1, 1, 2, 1, true),
        Know::Omniscient,
        Device::Compare,
        lag,
        6_000,
    );
    assert_eq!(run.least_separation(0), 50);
}

/// A wake, late or not, keeps to its floor, and from one read of the counter to the next is at most what the record's `update` writes, on each kind of device at each lag. Close to tight, its reader measured.
#[test]
fn a_wake_keeps_its_floor_and_its_read_to_read() {
    let mut r = Rng(0xDEAD_BEEF_1357_9BDF);
    for device in [Device::Compare, Device::Delay] {
        for lag in [0_u64, 1, 2, 4, 8] {
            let (mut judged, mut tries) = (0_u32, 0_u64);
            while judged < 12 && tries < 400_000 {
                tries += 1;
                let (mut tasks, mut alarms, p) = random_set(&mut r, true, false, true, lag);
                let every = r.pick(&[17_i64, 25, 40, 64, 100]);
                let c_s = alarms[1].c_s;
                alarms[1].wake = Some(every);
                // The realization's floor, at its step or under it.
                let floor = if r.below(2) == 0 { every } else { every / 2 };
                alarms[1].t_s = u64::try_from(floor).expect("small");
                alarms[1].read_at = 1 + r.below(c_s - 2 * lag);
                alarms[1].reach = 10_000;
                for task in &mut tasks {
                    for arm in &mut task.arms {
                        arm.0 = 0;
                    }
                }
                let Some((_, j)) = bounds(&tasks, &alarms, p) else {
                    continue;
                };
                judged += 1;
                for pattern in [vec![lag], vec![0], vec![0, lag], vec![lag, 0, 0]] {
                    let run = once(
                        &tasks,
                        &alarms,
                        p,
                        Know::WhatItReadAndWrote,
                        device,
                        LagModel {
                            max: lag,
                            pattern,
                            past: 0,
                        },
                        8_000,
                    );
                    let held = charge(
                        "update",
                        device,
                        Inputs {
                            t_s: floor,
                            lag: i64::try_from(lag).expect("small"),
                            rho: i64::try_from(p.res - 1).expect("small"),
                            j_s: j[1],
                            c_s: i64::try_from(c_s).expect("small"),
                            p: every,
                            reach: 0,
                            w: 0,
                        },
                    );
                    assert!(
                        run.longest_read_to_read[1] <= held,
                        "{} against {held}, {device:?}, lag {lag}: {tasks:?} {alarms:?} {p:?}",
                        run.longest_read_to_read[1]
                    );
                    assert!(
                        run.least_separation(1) >= floor,
                        "{device:?}, lag {lag}: {tasks:?} {alarms:?} {p:?}"
                    );
                }
            }
            assert_eq!(judged, 12, "{device:?}, lag {lag}");
        }
    }
}

/// A service that calls another alarm's node writes by a job's rule: the compare standing there is not the one that raised this service.
#[test]
fn a_service_writing_another_alarm_keeps_to_a_job_s_rule() {
    let mut r = Rng(0x0C05_5A1A_12A0_0006);
    for device in [Device::Compare, Device::Delay] {
        for lag in [0_u64, 2] {
            let (mut judged, mut tries) = (0_u32, 0_u64);
            while judged < 20 && tries < 400_000 {
                tries += 1;
                let (tasks, mut alarms, p) = random_set(&mut r, true, false, true, lag);
                alarms[1].calls = Some((0, r.pick(&[-5_i64, 0, 7, 30])));
                let Some((bound, j)) = bounds(&tasks, &alarms, p) else {
                    continue;
                };
                judged += 1;
                let run = once(
                    &tasks,
                    &alarms,
                    p,
                    Know::WhatItReadAndWrote,
                    device,
                    LagModel {
                        max: lag,
                        pattern: vec![lag, 0],
                        past: 0,
                    },
                    8_000,
                );
                assert_eq!(run.merged, 0, "{device:?}, lag {lag}");
                assert!(
                    run.worst
                        .iter()
                        .zip(&bound)
                        .all(|(worst, bound)| worst <= bound),
                    "{device:?}, lag {lag}: {tasks:?} {alarms:?} {p:?}"
                );
                let floor = i64::try_from(alarms[0].t_s).expect("small");
                assert!(run.least_separation(0) >= floor, "{device:?}, lag {lag}");
                let held = delivery(
                    device,
                    i64::try_from(lag).expect("small"),
                    i64::try_from(p.res - 1).expect("small"),
                    floor,
                    j[0],
                    i64::try_from(alarms[0].c_s).expect("small"),
                );
                assert!(
                    run.latest_delivery[0] <= held,
                    "{} against {held}, {device:?}, lag {lag}: {tasks:?} {alarms:?} {p:?}",
                    run.latest_delivery[0]
                );
            }
            assert_eq!(judged, 20, "{device:?}, lag {lag}");
        }
    }
}

/// What the code's bound rests on: a read after the landing farther than `λ` from it stands that much past the arrival, and the sum no longer holds. The record's `λ` covers it, and nothing else would owe it.
#[test]
fn a_read_after_the_landing_past_the_lag_breaks_the_sum() {
    let mut r = Rng(0x4444_5555_9BDF_0246);
    let (mut judged, mut over, mut tries) = (0_u32, 0_u32, 0_u64);
    let lag = 4_u64;
    while judged < 300 && over == 0 && tries < 400_000 {
        tries += 1;
        let two = r.below(3) == 0;
        let (tasks, alarms, p) = random_set(&mut r, two, false, true, lag);
        let Some((_, j)) = bounds(&tasks, &alarms, p) else {
            continue;
        };
        judged += 1;
        for pattern in [
            vec![lag],
            vec![0],
            vec![0, lag],
            vec![lag, 0, 0],
            vec![0, 0, lag],
        ] {
            let run = once(
                &tasks,
                &alarms,
                p,
                Know::WhatItReadAndWrote,
                Device::Compare,
                LagModel {
                    max: lag,
                    pattern,
                    past: 40,
                },
                8_000,
            );
            for (a, alarm) in alarms.iter().enumerate() {
                let held = delivery(
                    Device::Compare,
                    i64::try_from(lag).expect("small"),
                    i64::try_from(p.res - 1).expect("small"),
                    i64::try_from(alarm.t_s).expect("small"),
                    j[a],
                    i64::try_from(alarm.c_s).expect("small"),
                );
                if run.latest_delivery[a] > held {
                    over += 1;
                }
            }
        }
    }
    assert!(
        over > 0,
        "a read this far past the landing broke nothing, of {judged} sets"
    );
}

/// A floored compare is then rounded up, so the rounding adds to the floor and no `max` holds it: the design's fourth reader's case, a coarse compare with a deadline just past the floor.
#[test]
fn a_floored_compare_is_rounded_up_so_the_rounding_adds() {
    let tasks = vec![task("t0", 1, 128, 1, vec![(0, 1), (0, 18)], 2, None)];
    let alarms = [alarm(2, 17, 10_000, true)];
    let mut p = plain(1, 1, 1, 1, 0, true);
    p.res = 16;
    let Some((_, j)) = bounds(&tasks, &alarms, p) else {
        panic!("the analysis holds it");
    };
    let run = once(
        &tasks,
        &alarms,
        p,
        Know::WhatItReadAndWrote,
        Device::Compare,
        LagModel {
            max: 0,
            pattern: vec![0],
            past: 0,
        },
        4_000,
    );
    let (rho, floor, c_s) = (15, 17, 2);
    let charge = delivery(Device::Compare, 0, rho, floor, j[0], c_s);
    assert!(
        run.latest_delivery[0] <= charge,
        "{} against {charge}",
        run.latest_delivery[0]
    );
    // The sum a `max` over them would give does not hold it.
    assert!(
        run.latest_delivery[0] > rho.max(floor) + j[0] + c_s,
        "{} and J_s {}",
        run.latest_delivery[0],
        j[0]
    );
}

/// A deadline read off a modular counter, past at most `lag` or ahead at most `reach`: where `reach + lag` passes what one wrap tells apart, a deadline ahead and one past read the same. A floored compare stands `T_s`
/// past an arrival, so that is a reach too: round 2's 16-bit counter, and round 3's `T_s` 62 at modulus 64.
#[test]
fn a_modular_counter_tells_a_deadline_past_from_one_ahead_only_within_its_modulus() {
    let reads_right = |modulus: i64, reach: i64, lag: i64| {
        (0..modulus).all(|now| {
            (-lag..=reach).all(|distance| {
                let read = (now + distance).rem_euclid(modulus);
                let rest = (read - now).rem_euclid(modulus);
                // What is further ahead than `reach` can only be what has passed.
                let judged_due = rest > reach || rest == 0;
                judged_due == (distance <= 0)
            })
        })
    };
    for modulus in [16, 64, 256] {
        for lag in 0..modulus / 2 {
            for reach in 1..modulus {
                assert_eq!(
                    reads_right(modulus, reach, lag),
                    reach + lag < modulus,
                    "modulus {modulus}, reach {reach}, lag {lag}"
                );
            }
        }
    }
    // What stands ahead and what is read past: the floor stands `T_s` and a lag past a read, and a deadline is read past by as much as `delivery` lets it wait.
    let room = |reach: i64, t_s: i64, lag: i64, past: i64, modulus: i64| {
        reads_right(modulus, reach.max(t_s + lag), past)
    };
    assert!(room(15, 8, 5, 30, 64));
    assert!(
        !room(15, 62, 5, 5, 64),
        "the floor's reach, and not the delay's"
    );
    assert!(
        !room(100, 64, 1, 69, 128),
        "a deadline floored past its time is read as far ahead"
    );
    assert!(room(100, 64, 1, 69, 256));
    // The record's `room`, run: within one modulus is exactly what reads right, on each kind of device. What stands ahead is rounded up, and what is read past holds how long a writer may take.
    for device in [Device::Compare, Device::Delay] {
        for reach in [15_i64, 100, 238] {
            for t_s in [8_i64, 64] {
                for lag in [0_i64, 1, 5] {
                    let i = Inputs {
                        t_s,
                        lag,
                        rho: 3,
                        j_s: 2,
                        c_s: 1,
                        p: 0,
                        reach,
                        w: 9,
                    };
                    let past = charge("delivery", device, i) + 9;
                    for modulus in [64_i64, 256, 1024] {
                        assert_eq!(
                            reads_right(modulus, reach.max(t_s + lag + 3) + 3, past),
                            charge("room", device, i) < modulus,
                            "{device:?}, modulus {modulus}, reach {reach}, floor {t_s}, lag {lag}"
                        );
                    }
                }
            }
        }
    }
    // `R` is a deadline's and not a chain's segment: a segment of 30 has room in 256, and a deadline 238 ahead reads as one 18 past.
    assert!(reads_right(256, 30, 18));
    assert!(!reads_right(256, 238, 18));
    // A deadline past when it is written is read past by as long as its writer took: 100 ahead has room for `delivery` alone, and a job preempted 181 reads as 75 ahead.
    assert!(reads_right(256, 100, 20));
    assert!(!reads_right(256, 100, 181));
    // The floor on a counter that may idle past a wrap: judged within its own window it holds a deadline back no longer than that window, whatever the arrival's age, and holds it for as long as the floor is real.
    let held_back = |modulus: i64, t_s: i64, lag: i64, age: i64| {
        let ahead = (t_s - age).rem_euclid(modulus);
        // `lag` here stands for `λ + ρ`, what the record's window adds to `T_s`.
        if ahead <= t_s + lag {
            ahead
        } else {
            0
        }
    };
    for modulus in [64_i64, 256] {
        for t_s in [1_i64, 10, 17] {
            for lag in [0_i64, 1, 4] {
                for age in 0..4 * modulus {
                    let held = held_back(modulus, t_s, lag, age);
                    assert!(held <= t_s + lag, "{modulus} {t_s} {lag} {age}");
                    if age <= t_s {
                        assert_eq!(held, t_s - age, "{modulus} {t_s} {lag} {age}");
                    }
                }
            }
        }
    }
    // Judged within the reach, the only window that was named, `R` of 100, an arrival 200 ago holds a deadline back 66, against a window of 11.
    assert_eq!((10_i64 - 200).rem_euclid(256), 66);
    assert_eq!(held_back(256, 10, 1, 200), 0);
}
