//! The release alarm's design held against a timeline (`docs/decisions/decision_release-alarm.md` §2; `docs/decisions/decision_executable-design-reviews.md`). Nothing of the design is code yet,
//! and its review's counterexamples were worked by hand: so the claims the analysis is to be told are run here, over every way a job may program an alarm among a small universe.
//!
//! The timeline is the variant's model by unit steps, with `δ = ρ = γ = 0`: one core, services and transitions masked, no nesting, a compare level-triggered and exact. It is no realization. What it
//! shows is what the record states: a floor on the compare makes a `T_s` true whatever a job does, so the analysis's bound holds of the timeline; and with no floor it does not.

use std::collections::VecDeque;

use rt_analysis::compose::{
    compose, Application, Calls, Composition, Kernel, KernelFacts, Primitive, Run, Service,
    SourceParts, TaskParts,
};
use rt_analysis::runtime::{
    analyze, Acknowledge, Deferred, EnabledSet, Evidence, Outcome, Platform, PlatformFacts,
    ReleasedBy, TaskFacts, Value,
};

/// `api.arm` and `masked.arm`: a node's job path that programs the alarm, one masked run of this.
const ARM: u64 = 2;
/// One switch, the release timer's service and a completion, in the model's units.
const SWITCH: u64 = 1;
const C_REL: u64 = 1;
const COMPLETION: u64 = 1;
/// How far into its service an alarm's re-arm lands.
const REARM_AT: u64 = 1;

/// A task of the timeline and of the analysis.
#[derive(Clone)]
struct Spec {
    name: &'static str,
    rank: i64,
    period: u64,
    /// Its own code, before what it arms and its completion.
    own: u64,
    /// When it programs a deadline, a deadline per job, as an offset from its nominal release, by turns. Empty: it programs nothing.
    arms: Vec<i64>,
}

fn spec(name: &'static str, rank: i64, period: u64, own: u64, arms: &[i64]) -> Spec {
    Spec {
        name,
        rank,
        period,
        own,
        arms: arms.to_vec(),
    }
}

/// How an alarm is programmed.
#[derive(Clone, Copy)]
struct Shape {
    c_s: u64,
    /// The separation the source declares. With `floored`, what the compare is held to.
    t_s: u64,
    /// The most one delay reaches, so that a deadline further off is chained through arrivals on the way.
    reach: i64,
    /// Whether every compare is held at least `t_s` past the last arrival.
    floored: bool,
    /// The hardware's `external-before-timer`.
    external_first: bool,
}

/// What a timeline measured.
#[derive(Debug, Default)]
struct Measured {
    /// Each task's worst completion, from its nominal release, in the order given.
    worst: Vec<i64>,
    arrivals: Vec<i64>,
    /// Two arrivals that fell on one request, which is one lost.
    merged: u32,
    /// The latest a service ended, past when its deadline fell or was written if later.
    latest_delivery: i64,
    /// The longest between two reads of the counter by a service.
    longest_read_to_read: i64,
}

impl Measured {
    fn least_separation(&self) -> i64 {
        self.arrivals
            .windows(2)
            .map(|pair| pair[1] - pair[0])
            .min()
            .unwrap_or(i64::MAX)
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
enum Doing {
    Alarm { left: u64, done: u64 },
    Timer { left: u64 },
    Switch { to: Option<usize>, left: u64 },
}

/// The alarm, as the node programming it holds it.
struct Alarm {
    shape: Shape,
    pending: Vec<(i64, i64)>,
    compare: Option<i64>,
    request: bool,
    last_arrival: Option<i64>,
}

impl Alarm {
    /// The compare for the earliest deadline pending, as the write lands at `now`.
    fn arm(&mut self, now: i64) {
        self.compare = self
            .pending
            .iter()
            .map(|(deadline, _)| *deadline)
            .min()
            .map(|next| {
                let mut at = next.min(now + self.shape.reach).max(now);
                if self.shape.floored {
                    if let Some(last) = self.last_arrival {
                        at = at.max(last + i64::try_from(self.shape.t_s).expect("small"));
                    }
                }
                at
            });
    }
}

struct Live {
    spec: Spec,
    /// Each stretch of a job, whether it is masked, and how long.
    segments: Vec<(bool, u64)>,
    jobs: VecDeque<(i64, usize, u64, usize)>,
    next_release: i64,
    released: usize,
    worst: i64,
}

fn timeline(specs: &[Spec], shape: Shape, wake_every: Option<i64>, horizon: i64) -> Measured {
    let mut tasks: Vec<Live> = specs
        .iter()
        .map(|spec| Live {
            segments: if spec.arms.is_empty() {
                vec![(false, spec.own + COMPLETION)]
            } else {
                vec![(false, spec.own), (true, ARM), (false, COMPLETION)]
            },
            spec: spec.clone(),
            jobs: VecDeque::new(),
            next_release: 0,
            released: 0,
            worst: 0,
        })
        .collect();
    let mut alarm = Alarm {
        shape,
        pending: Vec::new(),
        compare: None,
        request: false,
        last_arrival: None,
    };
    let mut measured = Measured::default();
    if let Some(every) = wake_every {
        alarm.pending.push((every, 0));
        alarm.arm(0);
    }
    let (mut doing, mut running): (Option<Doing>, Option<usize>) = (None, None);
    let mut being_delivered: Vec<(i64, i64)> = Vec::new();
    let mut last_read: Option<i64> = None;
    for t in 0..horizon {
        if alarm.compare.is_some_and(|compare| t >= compare) {
            alarm.compare = None;
            measured.merged += u32::from(alarm.request);
            alarm.request = true;
            alarm.last_arrival = Some(t);
            measured.arrivals.push(t);
        }
        let due = tasks.iter().any(|task| task.next_release <= t);
        if doing.is_none() {
            let masked = running.is_some_and(|at| {
                tasks[at]
                    .jobs
                    .front()
                    .is_some_and(|(_, segment, _, _)| tasks[at].segments[*segment].0)
            });
            if !masked {
                let alarm_first = alarm.request && (shape.external_first || !due);
                if alarm_first {
                    alarm.request = false;
                    doing = Some(Doing::Alarm {
                        left: shape.c_s,
                        done: 0,
                    });
                } else if due {
                    doing = Some(Doing::Timer { left: C_REL });
                } else {
                    let top = tasks
                        .iter()
                        .enumerate()
                        .filter(|(_, task)| !task.jobs.is_empty())
                        .min_by_key(|(_, task)| task.spec.rank)
                        .map(|(at, _)| at);
                    if top != running {
                        doing = Some(Doing::Switch {
                            to: top,
                            left: SWITCH,
                        });
                    }
                }
            }
        }
        match doing {
            Some(Doing::Alarm { left, done }) => {
                let (left, done) = (left - 1, done + 1);
                if done == REARM_AT {
                    // The service reads the counter here, so a read-to-read is measured here.
                    if let Some(last) = last_read {
                        measured.longest_read_to_read =
                            measured.longest_read_to_read.max(t + 1 - last);
                    }
                    last_read = Some(t + 1);
                    let now = t + 1;
                    let (delivered, kept): (Vec<_>, Vec<_>) = alarm
                        .pending
                        .iter()
                        .copied()
                        .partition(|(deadline, _)| *deadline <= now);
                    alarm.pending = kept;
                    being_delivered.extend(delivered.iter().copied());
                    if let Some(every) = wake_every {
                        if !delivered.is_empty() {
                            // The next on the absolute grid, past now.
                            let next = (now / every + 1) * every;
                            alarm.pending.push((next, now));
                        }
                    }
                    alarm.arm(now);
                }
                doing = (left > 0).then_some(Doing::Alarm { left, done });
                if left == 0 {
                    for (deadline, written) in being_delivered.drain(..) {
                        measured.latest_delivery =
                            measured.latest_delivery.max(t + 1 - deadline.max(written));
                    }
                }
            }
            Some(Doing::Timer { left }) => {
                let left = left - 1;
                doing = (left > 0).then_some(Doing::Timer { left });
                if left == 0 {
                    for task in &mut tasks {
                        while task.next_release <= t {
                            let first = task.segments[0].1;
                            task.jobs
                                .push_back((task.next_release, 0, first, task.released));
                            task.released += 1;
                            task.next_release += i64::try_from(task.spec.period).expect("small");
                        }
                    }
                }
            }
            Some(Doing::Switch { to, left }) => {
                let left = left - 1;
                if left == 0 {
                    running = to;
                    doing = None;
                } else {
                    doing = Some(Doing::Switch { to, left });
                }
            }
            None => {
                if let Some(at) = running {
                    let task = &mut tasks[at];
                    if let Some((nominal, segment, left, job)) = task.jobs.front().copied() {
                        let left = left - 1;
                        if left > 0 {
                            task.jobs[0] = (nominal, segment, left, job);
                        } else {
                            if task.segments[segment].0 {
                                let arms = &task.spec.arms;
                                let deadline = nominal + arms[job % arms.len()];
                                alarm.pending.push((deadline, t + 1));
                                alarm.arm(t + 1);
                            }
                            if segment + 1 == task.segments.len() {
                                task.worst = task.worst.max(t + 1 - nominal);
                                task.jobs.pop_front();
                            } else {
                                let next = task.segments[segment + 1].1;
                                task.jobs[0] = (nominal, segment + 1, next, job);
                            }
                        }
                    }
                }
            }
        }
    }
    for task in &tasks {
        // A job still owed at the horizon has waited at least this long.
        let owed = task
            .jobs
            .front()
            .map_or(0, |(nominal, _, _, _)| horizon - nominal);
        measured.worst.push(task.worst.max(owed));
    }
    measured
}

fn v(value: u64) -> Option<Value> {
    Some(Value {
        value,
        evidence: Evidence::Analytical,
    })
}

fn platform() -> Platform {
    Platform {
        switch: v(SWITCH),
        wake: v(SWITCH),
        preemption_delay: v(0),
        timer_service: v(C_REL),
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
            interrupts: vec!["alarm".to_owned(), "timer".to_owned()],
            from_resolved_plan: true,
        }),
    }
}

fn kernel(external_first: bool) -> Kernel {
    let primitive = |name: &str, cost: u64| Primitive {
        name: name.to_owned(),
        call: v(cost),
        masked: v(cost),
    };
    Kernel {
        primitives: vec![
            primitive("mask", 0),
            primitive("unmask", 0),
            primitive("arm", ARM),
        ],
        completion: v(COMPLETION),
        masked_completion: v(COMPLETION),
        facts: KernelFacts {
            reprograms_only_in_service: Some(true),
            pending_taken_after_unmask: Some(true),
            releases_never_latched: Some(true),
            primitives_out_of_line: Some(true),
            external_before_timer: Some(external_first),
            one_claim_per_trap: Some(true),
            no_empty_claim: Some(true),
            starts_by_transition: Some(true),
            one_external_controller: Some(true),
        },
        discipline: vec![("rt-core".to_owned(), Some(true))],
    }
}

/// What the composition makes of this set, with one alarm at the top of the sources.
fn composed(specs: &[Spec], shape: Shape) -> Composition {
    let tasks = specs
        .iter()
        .map(|spec| TaskParts {
            id: spec.name.to_owned(),
            priority: spec.rank,
            separation: spec.period,
            deadline: spec.period,
            jitter_event: 0,
            released_by: ReleasedBy::Timer,
            facts: TaskFacts {
                suspends: Some(false),
                locks_scheduler: Some(false),
                shares_outside_sections: Some(false),
            },
            own_code: v(spec.own),
            calls: if spec.arms.is_empty() {
                Vec::new()
            } else {
                vec![Calls {
                    primitive: "arm".to_owned(),
                    count: 1,
                }]
            },
            calls_evidence: Some(Evidence::Analytical),
            runs: Vec::<Run>::new(),
            runs_evidence: Some(Evidence::Analytical),
        })
        .collect();
    let sources = [SourceParts {
        id: "alarm".to_owned(),
        separation: v(shape.t_s),
        acknowledge: Some(Acknowledge::AtEntry),
        deferred: Some(Deferred::Nothing),
        priority: Some(1),
        deliverable: Some(true),
        service: Service::Catalog(v(shape.c_s)),
        external: Some(true),
        one_request_per_arrival: Some(true),
    }];
    compose(
        &kernel(shape.external_first),
        &platform(),
        &Application {
            tasks,
            leaves_interrupt_hardware_alone: Some(true),
            releases_after_initialisation: Some(true),
        },
        &sources,
    )
}

/// The analysis's bound for each task, and `J_s`, where it admits the set and each bound holds.
fn bounds(specs: &[Spec], shape: Shape) -> Option<(Vec<i64>, i64)> {
    let composition = composed(specs, shape);
    let j_s = composition.sources[0].jitter?.value;
    let set = composition.admit(&platform()).ok()?;
    let mut found = Vec::new();
    for outcome in analyze(&set) {
        match outcome {
            Outcome::Holds { response, .. } => found.push(i64::try_from(response).ok()?),
            _ => return None,
        }
    }
    Some((found, i64::try_from(j_s).ok()?))
}

/// Every sequence of `length` offsets from `universe`, each a way a job may program its deadlines.
fn sequences(universe: &[i64], length: u32) -> Vec<Vec<i64>> {
    let count = universe.len().pow(length);
    (0..count)
        .map(|mut at| {
            (0..length)
                .map(|_| {
                    let offset = universe[at % universe.len()];
                    at /= universe.len();
                    offset
                })
                .collect()
        })
        .collect()
}

/// `T_s` is a floor on the compare, so it is true whatever a job programs, and the bound the analysis gives at it is one the timeline cannot pass.
#[test]
fn with_a_floor_the_declared_separation_is_true_and_the_bound_holds_whatever_a_job_programs() {
    let (mut judged, mut timelines) = (0_u32, 0_u32);
    let (mut tightest, mut widest_delivery) = (i64::MAX, 0);
    for (own, other, period) in [(1, 150, 400), (3, 60, 200), (1, 20, 100)] {
        for t_s in [12, 25, 40] {
            for reach in [15, 10_000] {
                for external_first in [true, false] {
                    let shape = Shape {
                        c_s: 5,
                        t_s,
                        reach,
                        floored: true,
                        external_first,
                    };
                    let set = |arms: &[i64]| {
                        vec![
                            spec("programs", 1, 40, own, arms),
                            spec("other", 2, period, other, &[]),
                        ]
                    };
                    let Some((bound, j_s)) = bounds(&set(&[1]), shape) else {
                        continue;
                    };
                    judged += 1;
                    let (c_s, floor) = (
                        i64::try_from(shape.c_s).expect("small"),
                        i64::try_from(t_s).expect("small"),
                    );
                    for arms in sequences(&[-5, 1, 3, 7, 13, 20, 33, 90], 3) {
                        timelines += 1;
                        let run = timeline(&set(&arms), shape, None, 2_000);
                        assert_eq!(run.merged, 0, "an arrival lost: {arms:?}");
                        assert!(
                            run.least_separation() >= floor,
                            "{arms:?}: arrivals {} apart against a floor of {floor}",
                            run.least_separation()
                        );
                        for (task, worst) in run.worst.iter().enumerate() {
                            assert!(
                                *worst <= bound[task],
                                "{arms:?}: {} took {worst} against a bound of {}",
                                set(&arms)[task].name,
                                bound[task]
                            );
                        }
                        // `delivery`: what lies between the programmed time and the match, then `J_s` and the service.
                        let lag = floor.max(j_s + c_s);
                        assert!(
                            run.latest_delivery <= lag + j_s + c_s,
                            "{arms:?}: delivered {} late, past {lag} + {j_s} + {c_s}",
                            run.latest_delivery
                        );
                        tightest = tightest.min(run.least_separation());
                        widest_delivery = widest_delivery.max(run.latest_delivery);
                    }
                }
            }
        }
    }
    assert_eq!(judged, 26, "the shapes that reach the analysis");
    assert_eq!(timelines, 13_312);
    assert_eq!(tightest, 12, "some timeline stands at the tightest floor");
    assert!(widest_delivery > 0);
}

/// With no floor a rate says nothing of a separation: round 2's chain, whose last segment arrives just after the re-arm, passes the analysis at the table's `T_s` and misses on the timeline.
#[test]
fn with_no_floor_a_chain_passes_the_analysis_and_misses() {
    let shape = Shape {
        c_s: 10,
        t_s: 100,
        reach: 100,
        floored: false,
        external_first: true,
    };
    let set = vec![
        spec("programs", 1, 125, 1, &[125]),
        spec("other", 2, 2_500, 1_999, &[]),
    ];
    let (bound, _) =
        bounds(&set, shape).expect("the analysis holds at the separation the table gave");
    assert_eq!(bound[1], 2_406);
    let run = timeline(&set, shape, None, 26_000);
    assert_eq!(run.least_separation(), 10);
    assert_eq!(
        run.worst[1], 2_988,
        "past its bound, and past its deadline of 2500"
    );
    // And the same set, the same deadlines, with the floor: the bound holds.
    let floored = Shape {
        floored: true,
        ..shape
    };
    let run = timeline(&set, floored, None, 26_000);
    assert!(run.least_separation() >= 100);
    assert_eq!(run.worst[1], 2_373, "inside its bound");
}

/// Two jobs of one period program deadlines closer than it: a rate of one per job is no separation, and a bound at the true least one holds.
#[test]
fn a_rate_is_no_separation() {
    let shape = |t_s, floored| Shape {
        c_s: 10,
        t_s,
        reach: 10_000,
        floored,
        external_first: true,
    };
    // One deadline per job, 100 apart on average, 50 at the closest.
    let set = vec![
        spec("programs", 1, 100, 1, &[50, 100]),
        spec("other", 2, 300, 60, &[]),
    ];
    let unfloored = timeline(&set, shape(100, false), None, 6_000);
    assert_eq!(unfloored.least_separation(), 50);
    let (bound, _) = bounds(&set, shape(50, true)).expect("it holds at the true least one");
    let run = timeline(&set, shape(50, true), None, 6_000);
    assert!(
        run.worst[1] <= bound[1],
        "{} against {}",
        run.worst[1],
        bound[1]
    );
}

/// A wake on an absolute grid with its service late: a floor keeps it to `T_s`, and from one read of the counter to the next is at most `P + J_s + C_s`, which `update` holds within the horizon.
#[test]
fn a_late_wake_keeps_its_separation_and_its_read_to_read() {
    for (every, c_s, own, external_first) in [
        (40, 5, 6, true),
        (40, 5, 6, false),
        (25, 3, 10, true),
        (25, 8, 2, false),
    ] {
        let shape = Shape {
            c_s,
            t_s: u64::try_from(every).expect("small"),
            reach: 10_000,
            floored: true,
            external_first,
        };
        let set = vec![
            spec("masks", 1, 50, own, &[1]),
            spec("other", 2, 200, 40, &[]),
        ];
        let Some((_, j_s)) = bounds(&set, shape) else {
            panic!("{every} {c_s} {own}: refused")
        };
        let run = timeline(&set, shape, Some(every), 4_000);
        assert!(
            run.least_separation() >= every,
            "{}",
            run.least_separation()
        );
        let c_s = i64::try_from(c_s).expect("small");
        assert!(
            run.longest_read_to_read <= every + j_s + c_s,
            "{} against {every} + {j_s} + {c_s}",
            run.longest_read_to_read
        );
    }
}

/// A deadline read off a modular counter, past at most `lag` or ahead at most `reach`: where `reach + lag` passes what one wrap tells apart, a deadline ahead and one past read the same. So an
/// offered horizon must leave the read-to-write lag inside the modulus: round 2's 16-bit counter at 65.535 ms.
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
}
