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
                h: 0,
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
            h: 0,
        },
    )
}

/// The record's `window`, as `delivery` above is its `delivery`.
fn window_charge(lag: i64, rho: i64, j_s: i64, c_s: i64) -> i64 {
    charge(
        "window",
        Device::Compare,
        Inputs {
            t_s: 0,
            lag,
            rho,
            j_s,
            c_s,
            p: 0,
            reach: 0,
            w: 0,
            h: 0,
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
    h: i64,
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
            ("H", i.h),
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
        h: 0,
    };
    assert_eq!(charge("delivery", Device::Compare, i), 10 + 1 + 6 + 5 + 2);
    assert_eq!(charge("delivery", Device::Delay, i), 10 + 2 + 9 + 5 + 2);
    assert_eq!(charge("update", Device::Compare, i), 64 + 3 + 5 + 2);
    assert_eq!(charge("update", Device::Delay, i), 64 + 1 + 6 + 5 + 2);
    assert_eq!(charge("room", Device::Compare, i), 100 + 3 + 24 + 9);
    assert_eq!(charge("room", Device::Delay, i), 100 + 3 + 28 + 9);
    assert_eq!(charge("write", Device::Compare, i), 10 + 1 + 6);
    assert_eq!(charge("write", Device::Delay, i), 10 + 1 + 6);
    assert_eq!(charge("window", Device::Compare, i), 1 + 3 + 5 + 2);
}

/// A hand-off's row of the record.
fn row_of(id: &str) -> &'static str {
    RECORD
        .lines()
        .find(|line| line.starts_with(&format!("| `{id}` |")))
        .expect("the hand-off's row")
}

/// What `/2` carries of one alarm, and what its realization and the analysis give it, as the table of inputs has each. `None` is what its owner has not said, for an alarm the table asks it of.
#[derive(Clone, Copy)]
struct Line {
    /// `alarm-kind`. The joint rules refuse an edge compare before this (`RA-H1`).
    device: Option<Device>,
    /// On the simulator, where `ρ`, `λ` and a window are nothing, and what one write reaches is one below the modulus (`RA-H3`).
    simulated: bool,
    /// Ticks in a second.
    tick_rate: Option<u64>,
    /// In ticks.
    modulus: Option<u128>,
    /// `supported-horizon`, in nanoseconds.
    reach: Option<u64>,
    /// `compare-step`, in ticks.
    step: Option<u64>,
    /// `compare-window`.
    window: Option<Reads>,
    /// `unambiguous-horizon`, in nanoseconds, for a wake.
    horizon: Option<u64>,
    /// `delivery-bound`, in nanoseconds, for an expiry.
    bound: Option<u64>,
    /// What an expiry's requirement asks by `supported-horizon`, in nanoseconds.
    asked_reach: Option<u64>,
    /// Whether the time base is the counter itself.
    counter_itself: bool,
    /// A wake, with a step of its own, or an expiry.
    wake: bool,
    /// What `RA-H7` asks of a task that writes it.
    stated_by_the_application: bool,
    /// The realization's, in nanoseconds. A wake's `R` is its `P`.
    t_s: Option<u64>,
    lag: Option<u64>,
    c_s: Option<u64>,
    p: Option<u64>,
    r: Option<u64>,
    /// The analysis's, in its unit. A wake's `W` is its service's `J_s + C_s`.
    j_s: i64,
    w: i64,
}

/// What the scheduling checker comes to of a line (`RA-H2`), with its verdict: an owed input is `analysis-inconclusive`, a `delivery` past its bound `not-established`, and any other refusal `unsupported-profile`.
#[derive(Debug, PartialEq, Eq)]
enum Judged {
    Holds,
    Refused(&'static str),
    NotEstablished(&'static str),
    Owed(&'static str),
    /// A figure stated below what it must bound: the variant's "a declared delay below its floor", `analysis-inconclusive` too.
    BelowItsFloor(&'static str),
}

/// How one figure rounds, as §1's last point says: a cost up to the analysis's unit, what a sum lies within down.
#[derive(Clone, Copy)]
struct Rounding {
    unit: u64,
}

impl Rounding {
    fn down(self, ns: u128) -> i64 {
        i64::try_from(ns / u128::from(self.unit)).unwrap_or(i64::MAX)
    }

    fn up(self, ns: u128) -> i64 {
        i64::try_from(ns.div_ceil(u128::from(self.unit))).unwrap_or(i64::MAX)
    }

    /// A count of ticks that is a cost: rounding up at each step, as a fraction of a nanosecond is still a cost.
    fn ticks_up(self, ticks: u64, rate: u64) -> i64 {
        self.up((u128::from(ticks) * 1_000_000_000).div_ceil(u128::from(rate)))
    }

    /// A count of ticks that a sum lies within, reckoned as whole seconds of ticks and what is left, so that no count the language writes, to `(pow2 126)`,
    /// overflows: one past what 128 bits of nanoseconds hold is past any figure the checker holds. Multiplied first, a count from `(pow2 99)` overflowed
    /// (format `/2`'s review, round 10, RP10-3).
    fn ticks_down(self, ticks: u128, rate: u64) -> i64 {
        let rate = u128::from(rate);
        let part = ticks % rate * 1_000_000_000 / rate;
        (ticks / rate)
            .checked_mul(1_000_000_000)
            .and_then(|whole| whole.checked_add(part))
            .map_or(i64::MAX, |ns| self.down(ns))
    }
}

/// `T_s` as the variant's separation, where a sum reads it rounded up: the table's "up in a sum, down as the variant's separation".
fn separation(t_s: u64, unit: u64) -> i64 {
    Rounding { unit }.down(u128::from(t_s))
}

/// How a compare reads a value against its count, which its device states by `compare-window` or `compare-magnitude` (§2's `window`).
#[derive(Clone, Copy, PartialEq, Eq)]
enum Reads {
    /// One raw difference on the modulus: a value up to this many ticks past reads as due, and any other as ahead.
    Within(u64),
    /// The two as whole numbers, `count ≥ value`, which is right only short of a wrap and fits no window.
    Magnitude,
}

/// The longest a system is taken to run, in seconds: a hundred years, the record's figure for `ROADMAP.md` §6.2's operating horizon, and as far as a compare that reads whole numbers is told apart.
const OPERATING_HORIZON: u128 = 3_155_760_000;

/// What a compare's reading asks of a due value and leaves of a modulus, each where this line can say, or nothing where the device's statement cannot stand. A value past the count's top reads as due at once on a whole-number compare, so what the horizon leaves is its limit.
fn what_it_reads(
    reads: Reads,
    modulus: Option<u128>,
    rate: Option<u64>,
    round: Rounding,
) -> Option<(Option<i64>, Option<i64>)> {
    // A window past its modulus cannot stand whatever the rate.
    if matches!((reads, modulus), (Reads::Within(said), Some(modulus)) if u128::from(said) >= modulus)
    {
        return None;
    }
    let Some(rate) = rate else {
        return Some((None, None));
    };
    match (reads, modulus) {
        (Reads::Within(said), Some(modulus)) => Some((
            Some(round.ticks_down(u128::from(said), rate)),
            Some(round.ticks_down(modulus - u128::from(said) - 1, rate)),
        )),
        (Reads::Within(said), None) => Some((Some(round.ticks_down(u128::from(said), rate)), None)),
        // Its wrap must lie past the horizon, and at the horizon it does not.
        (Reads::Magnitude, Some(modulus)) if modulus <= OPERATING_HORIZON * u128::from(rate) => {
            None
        }
        (Reads::Magnitude, Some(modulus)) => Some((
            None,
            Some(round.ticks_down(modulus - OPERATING_HORIZON * u128::from(rate) - 1, rate)),
        )),
        (Reads::Magnitude, None) => Some((None, None)),
    }
}

/// What one write reaches, of what a target states and what a compare's reading leaves, each where it can be said.
fn what_one_write_reaches(reach: Option<i64>, leaves: Option<i64>) -> Option<i64> {
    match (reach, leaves) {
        (Some(reach), Some(leaves)) => Some(reach.min(leaves)),
        (one, other) => one.or(other),
    }
}

/// Every reason the scheduling checker gives of one alarm, the record's sums as its table writes them, each input arriving as the table of inputs says and rounding as it says: a run of `RA-H2`, so that what a line lacks shows as what it lacks. It checks every sum whose inputs it has, as the variant's §4 asks, and a sum missing an input says only that the input is owed. With no kind no cell can be read, and that is all that stops every sum. `unit` is the analysis's, in nanoseconds. `ρ` is one whole step: a step less one unit would rest on every event falling on the unit, as a timeline's do and a machine's do not.
fn reasons(line: Line, unit: u64) -> Vec<Judged> {
    let round = Rounding { unit };
    let simulated = line.simulated;
    let on_the_counter = !line.wake && line.counter_itself;
    let compare = !simulated && line.device == Some(Device::Compare);
    let mut found = said_owed(&[
        (line.tick_rate.is_none(), "tick-rate"),
        (line.modulus.is_none(), "counter-modulus"),
        (!simulated && line.device.is_none(), "alarm-kind"),
        (!simulated && line.reach.is_none(), "supported-horizon"),
        (!simulated && line.step.is_none(), "compare-step"),
        (line.t_s.is_none(), "T_s"),
        (!simulated && line.lag.is_none(), "λ"),
        (line.c_s.is_none(), "C_s"),
        (compare && line.window.is_none(), "compare-window"),
        (line.wake && line.p.is_none(), "P"),
        (
            on_the_counter && !line.stated_by_the_application,
            "the application's statement",
        ),
        (on_the_counter && line.r.is_none(), "R"),
        (line.wake && line.horizon.is_none(), "unambiguous-horizon"),
        (!line.wake && line.bound.is_none(), "delivery-bound"),
    ]);
    let (rate, modulus) = (line.tick_rate, line.modulus);
    if rate == Some(0) || modulus == Some(0) {
        found.push(Judged::Refused("a counter of no rate or modulus"));
        return found;
    }
    let Some(device) = (if simulated {
        Some(Device::Compare)
    } else {
        line.device
    }) else {
        return found;
    };
    // What this line lacks, by the name a cell writes it: a sum reading one of these says nothing but that it is owed.
    let mut gone = Vec::new();
    let rho = if simulated {
        0
    } else {
        match (line.step, rate) {
            (Some(0), _) => {
                found.push(Judged::Refused("compare-step"));
                gone.push("ρ");
                0
            }
            (Some(step), Some(rate)) => round.ticks_up(step, rate),
            _ => {
                gone.push("ρ");
                0
            }
        }
    };
    let mut cost = |ns: Option<u64>, name: &'static str| {
        ns.map_or_else(
            || {
                gone.push(name);
                0
            },
            |ns| round.up(u128::from(ns)),
        )
    };
    let t_s = cost(line.t_s, "T_s");
    let lag = if simulated { 0 } else { cost(line.lag, "λ") };
    let c = cost(line.c_s, "C_s");
    let p = if line.wake { cost(line.p, "P") } else { 0 };
    let r = if line.wake {
        cost(line.p, "R")
    } else if line.counter_itself {
        cost(line.r, "R")
    } else {
        0
    };
    let w = if line.wake {
        line.j_s + cost(line.c_s, "W")
    } else {
        line.w
    };
    if line.wake && line.t_s.zip(line.p).is_some_and(|(t_s, p)| t_s > p) {
        found.push(Judged::Refused("a wake's floor"));
    }
    if on_the_counter
        && line
            .r
            .zip(line.asked_reach)
            .is_some_and(|(r, asked)| r < asked)
    {
        found.push(Judged::Refused("R"));
    }
    // What a due value lies within, and what a compare's reading leaves one write.
    let (window, leaves) = if compare {
        match line
            .window
            .map(|reads| what_it_reads(reads, modulus, rate, round))
        {
            Some(Some(limits)) => limits,
            Some(None) => {
                found.push(Judged::Refused("compare-window"));
                (None, None)
            }
            None => (None, None),
        }
    } else {
        (None, None)
    };
    let h = if simulated {
        modulus
            .zip(rate)
            .map(|(modulus, rate)| round.ticks_down(modulus - 1, rate))
    } else {
        what_one_write_reaches(
            line.reach.map(|reach| round.down(u128::from(reach))),
            leaves,
        )
    };
    let i = Inputs {
        t_s,
        lag,
        rho,
        j_s: line.j_s,
        c_s: c,
        p,
        reach: r,
        w,
        h: h.unwrap_or(0),
    };
    let runs = |sum: &str| {
        let reads_none = |written: &str| !gone.iter().any(|name| written.contains(name));
        let written = cell(sum, device);
        reads_none(written)
            && (!written.contains("delivery") || reads_none(cell("delivery", device)))
    };
    if window.is_some_and(|window| runs("window") && charge("window", device, i) > window) {
        found.push(Judged::Refused("window"));
    }
    if h.is_some_and(|h| runs("write") && charge("write", device, i) > h) {
        found.push(Judged::Refused("write"));
    }
    if line.counter_itself
        && runs("room")
        && modulus.zip(rate).is_some_and(|(modulus, rate)| {
            charge("room", device, i) >= round.ticks_down(modulus, rate)
        })
    {
        found.push(Judged::Refused("room"));
    }
    if line.wake {
        if line.horizon.is_some_and(|horizon| {
            runs("update") && charge("update", device, i) > round.down(u128::from(horizon))
        }) {
            found.push(Judged::Refused("update"));
        }
        if h.is_some_and(|h| !gone.contains(&"P") && !gone.contains(&"ρ") && p + rho > h) {
            found.push(Judged::Refused("a wake's first arm"));
        }
    } else if line.bound.is_some_and(|bound| {
        runs("delivery") && charge("delivery", device, i) > round.down(u128::from(bound))
    }) {
        found.push(Judged::NotEstablished("delivery"));
    }
    found
}

/// Each input a line lacks, as an owed reason.
fn said_owed(lacking: &[(bool, &'static str)]) -> Vec<Judged> {
    lacking
        .iter()
        .filter(|(lacks, _)| *lacks)
        .map(|(_, input)| Judged::Owed(input))
        .collect()
}

/// The verdict among all the reasons, as the variant's §4 and §5 read: `unsupported-profile`, then `not-established`, then `analysis-inconclusive`, naming the first of its kind.
fn highest(mut found: Vec<Judged>) -> Judged {
    found.sort_by_key(|reason| match reason {
        Judged::Refused(_) => 0,
        Judged::NotEstablished(_) => 1,
        Judged::Owed(_) | Judged::BelowItsFloor(_) => 2,
        Judged::Holds => 3,
    });
    found.into_iter().next().unwrap_or(Judged::Holds)
}

/// What the scheduling checker comes to of one alarm.
fn judge(line: Line, unit: u64) -> Judged {
    highest(reasons(line, unit))
}

/// What `/2` and the record supplying `C_rel` carry of the release timer. `(timer none)` asks none of it (`RA-H2`).
#[derive(Clone, Copy)]
struct ReleaseLine {
    tick_rate: Option<u64>,
    modulus: Option<u128>,
    reach: Option<u64>,
    step: Option<u64>,
    window: Option<Reads>,
    horizon: Option<u64>,
    /// In nanoseconds.
    shortest: u64,
    /// `compare-rounding`, the variant's `ρ`, and `C_rel`, in nanoseconds.
    rounding: Option<u64>,
    c_rel: Option<u64>,
    /// The analysis's, in its unit.
    j_release: i64,
}

/// What `compare-rounding` must be at least, as the analysis reads it: rounding up to a step adds less than a whole step to a release, which falls anywhere, a period being an exact duration, so the worst case rounded up to the unit is one step rounded up.
fn rounding_at_worst(step: u64, rate: u64, unit: u64) -> i64 {
    Rounding { unit }.ticks_up(step, rate)
}

/// The rest of `RA-H2`, the release timer's own holds, each owed input and refusal read as `reasons` reads them, every one this line can run.
fn release_reasons(line: ReleaseLine, unit: u64) -> Vec<Judged> {
    let round = Rounding { unit };
    let mut found = said_owed(&[
        (line.tick_rate.is_none(), "tick-rate"),
        (line.modulus.is_none(), "counter-modulus"),
        (line.reach.is_none(), "supported-horizon"),
        (line.step.is_none(), "compare-step"),
        (line.window.is_none(), "compare-window"),
        (line.horizon.is_none(), "unambiguous-horizon"),
        (line.rounding.is_none(), "compare-rounding"),
        (line.c_rel.is_none(), "C_rel"),
    ]);
    let (rate, modulus) = (line.tick_rate, line.modulus);
    if rate == Some(0) || modulus == Some(0) {
        found.push(Judged::Refused("a counter of no rate or modulus"));
        return found;
    }
    let (window, leaves) = match line
        .window
        .map(|reads| what_it_reads(reads, modulus, rate, round))
    {
        Some(Some(limits)) => limits,
        Some(None) => {
            found.push(Judged::Refused("compare-window"));
            (None, None)
        }
        None => (None, None),
    };
    let rho = line.rounding.map(|rounding| round.up(u128::from(rounding)));
    if line.step == Some(0) {
        found.push(Judged::Refused("compare-step"));
    }
    // At least one step rounded up: `RA-H5`'s compare. An understatement is what breaks a bound, which the variant reads as a delay below its floor, and an overstatement a cost's upper bound.
    if rho
        .zip(line.step)
        .zip(rate)
        .is_some_and(|((rho, step), rate)| step != 0 && rho < rounding_at_worst(step, rate, unit))
    {
        found.push(Judged::BelowItsFloor("compare-rounding"));
    }
    // `SHORTEST` rounds up to a tick wherever it enters a sum of lags.
    let shortest = rate.map(|rate| {
        let ticks = (u128::from(line.shortest) * u128::from(rate)).div_ceil(1_000_000_000);
        round.up((ticks * 1_000_000_000).div_ceil(u128::from(rate)))
    });
    let lag = line
        .c_rel
        .map(|c_rel| line.j_release + round.up(u128::from(c_rel)));
    if shortest
        .zip(lag)
        .zip(line.horizon)
        .is_some_and(|((shortest, lag), horizon)| shortest + lag > round.down(u128::from(horizon)))
    {
        found.push(Judged::Refused("the release timer's horizon"));
    }
    if lag.zip(window).is_some_and(|(lag, window)| lag > window) {
        found.push(Judged::Refused("the release timer's window"));
    }
    let h = what_one_write_reaches(
        line.reach.map(|reach| round.down(u128::from(reach))),
        leaves,
    );
    if shortest
        .zip(rho)
        .zip(h)
        .is_some_and(|((shortest, rho), h)| shortest + 2 * rho > h)
    {
        found.push(Judged::Refused("the release timer's write"));
    }
    found
}

/// What the scheduling checker comes to of the release timer.
fn judge_release(line: ReleaseLine, unit: u64) -> Judged {
    highest(release_reasons(line, unit))
}

/// Each case a round of the review found by reading, run: what a line lacks is what the checker says, and each figure reaches it.
#[test]
fn what_the_checker_holds_of_a_line_runs() {
    let good = Line {
        device: Some(Device::Compare),
        simulated: false,
        tick_rate: Some(1_000_000),
        modulus: Some(65_536),
        reach: Some(60_000_000),
        step: Some(1),
        window: Some(Reads::Within(32_768)),
        horizon: Some(65_535_000),
        bound: Some(4_000_000),
        asked_reach: Some(2_000_000),
        counter_itself: true,
        wake: false,
        stated_by_the_application: true,
        t_s: Some(100_000),
        lag: Some(5_000),
        c_s: Some(10_000),
        p: None,
        r: Some(2_000_000),
        j_s: 50,
        w: 500,
    };
    let at_1_us = |line: Line| judge(line, 1_000);
    assert_eq!(at_1_us(good), Judged::Holds);
    // Round 12: the same count, at another rate, is another modulus and another window, and the rate reaches the checker.
    let faster = Line {
        tick_rate: Some(100_000_000),
        ..good
    };
    assert_eq!(at_1_us(faster), Judged::Refused("room"));
    assert_eq!(
        at_1_us(Line {
            counter_itself: false,
            window: Some(Reads::Within(100)),
            ..faster
        }),
        Judged::Refused("window")
    );
    // Round 10, round 13: silence gives no value, whoever owes it, and a failed sum says which verdict it is.
    for (lacking, said) in [
        (
            Line {
                device: None,
                ..good
            },
            "alarm-kind",
        ),
        (
            Line {
                tick_rate: None,
                ..good
            },
            "tick-rate",
        ),
        (
            Line {
                modulus: None,
                ..good
            },
            "counter-modulus",
        ),
        (
            Line {
                reach: None,
                ..good
            },
            "supported-horizon",
        ),
        (Line { step: None, ..good }, "compare-step"),
        (
            Line {
                window: None,
                ..good
            },
            "compare-window",
        ),
        (
            Line {
                bound: None,
                ..good
            },
            "delivery-bound",
        ),
        (Line { t_s: None, ..good }, "T_s"),
        (Line { lag: None, ..good }, "λ"),
        (Line { c_s: None, ..good }, "C_s"),
        (Line { r: None, ..good }, "R"),
        (
            Line {
                stated_by_the_application: false,
                ..good
            },
            "the application's statement",
        ),
    ] {
        assert_eq!(at_1_us(lacking), Judged::Owed(said));
    }
    assert_eq!(
        at_1_us(Line {
            bound: Some(100_000),
            ..good
        }),
        Judged::NotEstablished("delivery")
    );
    // On an epoch the code's reads ask neither `R` nor the application's statement.
    assert_eq!(
        at_1_us(Line {
            counter_itself: false,
            stated_by_the_application: false,
            r: None,
            ..good
        }),
        Judged::Holds
    );
    // An expiry's `R` covers what its requirement asks.
    assert_eq!(
        at_1_us(Line {
            r: Some(1_000_000),
            ..good
        }),
        Judged::Refused("R")
    );
    // Round 8 and 9's rounding: what one write reaches rounds down, so 3.5 µs is 3 and 2.999 µs is 2, and a cost rounds up, so 1.5 µs is 2.
    let tight = Line {
        t_s: Some(1_000),
        lag: Some(0),
        reach: Some(3_500),
        ..good
    };
    assert_eq!(at_1_us(tight), Judged::Holds);
    assert_eq!(
        at_1_us(Line {
            reach: Some(2_999),
            ..tight
        }),
        Judged::Refused("write")
    );
    assert_eq!(
        at_1_us(Line {
            t_s: Some(1_500),
            ..tight
        }),
        Judged::Refused("write")
    );
    assert_eq!(separation(1_500, 1_000), 1);
    // Round 13: `ρ` is the whole step. At a step of 2, `λ` of 5, `J_s` of 30 and `C_s` of 5 a window of 42 holds and 41 does not, where a step less one unit took 41.
    let epoch = Line {
        step: Some(2),
        modulus: Some(256),
        window: Some(Reads::Within(45)),
        counter_itself: false,
        wake: true,
        stated_by_the_application: false,
        t_s: Some(20_000),
        lag: Some(5_000),
        c_s: Some(5_000),
        p: Some(100_000),
        horizon: Some(1_000_000),
        r: None,
        j_s: 30,
        w: 0,
        ..good
    };
    assert_eq!(at_1_us(epoch), Judged::Holds);
    assert_eq!(
        at_1_us(Line {
            window: Some(Reads::Within(42)),
            ..epoch
        }),
        Judged::Holds
    );
    assert_eq!(
        at_1_us(Line {
            window: Some(Reads::Within(41)),
            ..epoch
        }),
        Judged::Refused("window")
    );
    // Round 13: a step that is no whole nanosecond is a cost all the same, and at 3 MHz one tick is 334 ns, not 332.
    let at_1_ns = |line: Line| judge(line, 1);
    let thirds = Line {
        tick_rate: Some(3_000_000),
        lag: Some(0),
        c_s: Some(0),
        ..good
    };
    assert_eq!(
        at_1_ns(Line {
            window: Some(Reads::Within(1)),
            ..thirds
        }),
        Judged::Refused("window")
    );
    assert_eq!(
        at_1_ns(Line {
            window: Some(Reads::Within(2)),
            ..thirds
        }),
        Judged::Holds
    );
    // And at 2 GHz half a nanosecond is one, not none and not less.
    let fine = Line {
        tick_rate: Some(2_000_000_000),
        modulus: Some(u128::from(u64::MAX)),
        lag: Some(0),
        c_s: Some(0),
        j_s: 0,
        ..good
    };
    assert_eq!(
        at_1_ns(Line {
            window: Some(Reads::Within(1)),
            ..fine
        }),
        Judged::Refused("window")
    );
    assert_eq!(
        at_1_ns(Line {
            window: Some(Reads::Within(2)),
            ..fine
        }),
        Judged::Holds
    );
    // Round 13: the device's window, and not some window: a device reading 16 of 256 as due reads a value 30 past as 226 ahead.
    assert_eq!(
        at_1_us(Line {
            modulus: Some(256),
            window: Some(Reads::Within(16)),
            step: Some(1),
            lag: Some(30_000),
            reach: Some(200_000),
            counter_itself: false,
            bound: Some(100_000_000),
            ..good
        }),
        Judged::Refused("window")
    );
    // Round 13: a reach to the whole modulus no longer refuses what holds, and needs no target to know a lag. A segment stops at what the window leaves.
    assert_eq!(
        at_1_us(Line {
            reach: Some(65_535_000),
            ..good
        }),
        Judged::Holds
    );
    assert_eq!(
        at_1_us(Line {
            p: Some(208_000),
            ..epoch
        }),
        Judged::Holds
    );
    assert_eq!(
        at_1_us(Line {
            p: Some(209_000),
            ..epoch
        }),
        Judged::Refused("a wake's first arm")
    );
    // Round 10: a wake's first arm must reach what the target says, too.
    assert_eq!(
        at_1_us(Line {
            reach: Some(90_000),
            ..epoch
        }),
        Judged::Refused("a wake's first arm")
    );
    // A delay counts from its write, and has no window.
    assert_eq!(
        at_1_us(Line {
            device: Some(Device::Delay),
            window: None,
            reach: Some(250_000),
            horizon: Some(255_000),
            ..epoch
        }),
        Judged::Holds
    );
    // Round 13: a wake's floor is at most its step, and it owes one.
    assert_eq!(
        at_1_us(Line {
            t_s: Some(300_000),
            ..epoch
        }),
        Judged::Refused("a wake's floor")
    );
    assert_eq!(at_1_us(Line { p: None, ..epoch }), Judged::Owed("P"));
    assert_eq!(
        at_1_us(Line {
            horizon: None,
            ..epoch
        }),
        Judged::Owed("unambiguous-horizon")
    );
    // Round 13: a wake's `R` and `W` come of its own figures, whatever a line says of them.
    let long_wake = Line {
        modulus: Some(65_536),
        step: Some(1),
        window: Some(Reads::Within(37)),
        counter_itself: true,
        lag: Some(1_000),
        reach: Some(65_535_000),
        horizon: Some(65_535_000),
        r: Some(0),
        w: 0,
        ..epoch
    };
    assert_eq!(
        at_1_us(Line {
            p: Some(65_400_000),
            ..long_wake
        }),
        Judged::Holds
    );
    assert_eq!(
        at_1_us(Line {
            p: Some(65_450_000),
            ..long_wake
        }),
        Judged::Refused("room")
    );
    // Round 14: a level line that falls past its window takes a request held off past it, so a wait of 43 is no window of 16, and the sum says so. Floor 60, `C_s` 10, a job's masked write of 40, as the reader composed it.
    let held_off = Line {
        modulus: Some(256),
        step: Some(1),
        window: Some(Reads::Within(16)),
        reach: Some(200_000),
        counter_itself: false,
        t_s: Some(60_000),
        lag: Some(5_000),
        c_s: Some(10_000),
        bound: Some(200_000),
        asked_reach: None,
        r: None,
        j_s: 43,
        w: 0,
        ..good
    };
    assert_eq!(at_1_us(held_off), Judged::Refused("window"));
    // Round 15: the sum runs to the claim, within `C_s`: 5 + 1 + 43 + 10.
    assert_eq!(
        at_1_us(Line {
            window: Some(Reads::Within(58)),
            ..held_off
        }),
        Judged::Refused("window")
    );
    assert_eq!(
        at_1_us(Line {
            window: Some(Reads::Within(59)),
            ..held_off
        }),
        Judged::Holds
    );
    // Round 15: every sum whose inputs it has runs, though a modulus or a reach is owed.
    let no_modulus = Line {
        modulus: None,
        bound: Some(100_000),
        window: Some(Reads::Within(59)),
        ..held_off
    };
    assert_eq!(at_1_us(no_modulus), Judged::NotEstablished("delivery"));
    assert_eq!(
        reasons(no_modulus, 1_000),
        vec![
            Judged::Owed("counter-modulus"),
            Judged::NotEstablished("delivery")
        ]
    );
    let no_reach = Line {
        reach: None,
        t_s: Some(190_000),
        window: Some(Reads::Within(59)),
        ..held_off
    };
    assert_eq!(at_1_us(no_reach), Judged::Refused("write"));
    // Round 14: a compare that reads whole numbers has no window to hold and no cap on a segment, past an operating horizon, and short of it is refused.
    let whole = Line {
        modulus: Some(1 << 64),
        tick_rate: Some(10_000_000),
        window: Some(Reads::Magnitude),
        reach: Some(u64::MAX),
        ..held_off
    };
    assert_eq!(at_1_us(whole), Judged::Holds);
    assert_eq!(
        at_1_us(Line {
            modulus: Some(1 << 32),
            ..whole
        }),
        Judged::Refused("compare-window")
    );
    // Round 15: a value past the count's top reads as due at once, so a segment stops at what the operating horizon leaves: 2^55 at 10 MHz is 114 years, which leaves 14.
    assert_eq!(
        at_1_us(Line {
            modulus: Some(1 << 55),
            t_s: Some(15 * 31_557_600_000_000_000),
            bound: Some(u64::MAX),
            ..whole
        }),
        Judged::Refused("write")
    );
    // Round 16: a wrap exactly at the horizon does not lie past it, and a window past its modulus cannot stand though the rate is owed.
    assert_eq!(
        at_1_us(Line {
            modulus: Some(OPERATING_HORIZON * 10_000_000),
            ..whole
        }),
        Judged::Refused("compare-window")
    );
    assert_eq!(
        at_1_us(Line {
            tick_rate: None,
            modulus: Some(256),
            window: Some(Reads::Within(300)),
            ..held_off
        }),
        Judged::Refused("compare-window")
    );
    // Round 14: every sum whose inputs it has is checked, as the variant's §4 asks, and the verdict is the highest among them: a refusal over what is owed, and each reason kept.
    let both = Line {
        t_s: None,
        window: Some(Reads::Within(1)),
        ..held_off
    };
    assert_eq!(at_1_us(both), Judged::Refused("window"));
    assert_eq!(
        reasons(both, 1_000),
        vec![Judged::Owed("T_s"), Judged::Refused("window")]
    );
    let late = Line {
        bound: Some(100_000),
        t_s: Some(60_000),
        window: Some(Reads::Within(1)),
        ..held_off
    };
    assert_eq!(
        reasons(late, 1_000),
        vec![
            Judged::Refused("window"),
            Judged::NotEstablished("delivery")
        ]
    );
    assert_eq!(at_1_us(late), Judged::Refused("window"));
    assert_eq!(
        at_1_us(Line {
            bound: Some(100_000),
            window: Some(Reads::Within(59)),
            ..held_off
        }),
        Judged::NotEstablished("delivery")
    );
    // Round 13: the simulator's "has none" is no owed figure: `ρ`, `λ` and a window nothing, and one write reaching one below the modulus, which fits. Its requests wait for their claim, so `window` holds nothing there.
    let simulated = Line {
        simulated: true,
        device: None,
        step: None,
        window: None,
        lag: None,
        reach: None,
        ..good
    };
    assert_eq!(at_1_us(simulated), Judged::Holds);
    assert_eq!(
        at_1_us(Line {
            tick_rate: None,
            ..simulated
        }),
        Judged::Owed("tick-rate")
    );
}

/// The release timer's own holds, as `RA-H2` lists them.
#[test]
fn what_the_checker_holds_of_the_release_timer_runs() {
    let good = ReleaseLine {
        tick_rate: Some(1_000_000),
        modulus: Some(65_536),
        reach: Some(60_000_000),
        step: Some(2),
        window: Some(Reads::Within(32_768)),
        horizon: Some(65_535_000),
        shortest: 1_000_000,
        rounding: Some(2_000),
        c_rel: Some(20_000),
        j_release: 30,
    };
    let at_1_us = |line: ReleaseLine| judge_release(line, 1_000);
    assert_eq!(at_1_us(good), Judged::Holds);
    for (lacking, said) in [
        (
            ReleaseLine {
                tick_rate: None,
                ..good
            },
            "tick-rate",
        ),
        (
            ReleaseLine {
                modulus: None,
                ..good
            },
            "counter-modulus",
        ),
        (
            ReleaseLine {
                reach: None,
                ..good
            },
            "supported-horizon",
        ),
        (ReleaseLine { step: None, ..good }, "compare-step"),
        (
            ReleaseLine {
                window: None,
                ..good
            },
            "compare-window",
        ),
        (
            ReleaseLine {
                horizon: None,
                ..good
            },
            "unambiguous-horizon",
        ),
        (
            ReleaseLine {
                rounding: None,
                ..good
            },
            "compare-rounding",
        ),
        (
            ReleaseLine {
                c_rel: None,
                ..good
            },
            "C_rel",
        ),
    ] {
        assert_eq!(at_1_us(lacking), Judged::Owed(said));
    }
    // `SHORTEST + J^release + C_rel`, 1,050, within the horizon.
    assert_eq!(
        at_1_us(ReleaseLine {
            horizon: Some(1_049_999),
            ..good
        }),
        Judged::Refused("the release timer's horizon")
    );
    assert_eq!(
        at_1_us(ReleaseLine {
            horizon: Some(1_050_000),
            ..good
        }),
        Judged::Holds
    );
    // `J^release + C_rel`, 50, within the window.
    assert_eq!(
        at_1_us(ReleaseLine {
            window: Some(Reads::Within(49)),
            ..good
        }),
        Judged::Refused("the release timer's window")
    );
    // `SHORTEST + 2ρ`, 1,004, within what one write reaches, by the target or by what the window leaves.
    assert_eq!(
        at_1_us(ReleaseLine {
            reach: Some(1_003_999),
            ..good
        }),
        Judged::Refused("the release timer's write")
    );
    assert_eq!(
        at_1_us(ReleaseLine {
            reach: Some(1_004_000),
            ..good
        }),
        Judged::Holds
    );
    // Round 15: and with no reach stated, what the window leaves is still a limit.
    assert_eq!(
        at_1_us(ReleaseLine {
            reach: None,
            modulus: Some(1_052),
            window: Some(Reads::Within(50)),
            horizon: Some(1_050_000),
            ..good
        }),
        Judged::Refused("the release timer's write")
    );
    assert_eq!(
        at_1_us(ReleaseLine {
            modulus: Some(1_052),
            window: Some(Reads::Within(50)),
            horizon: Some(1_050_000),
            ..good
        }),
        Judged::Refused("the release timer's write")
    );
    // `RA-H5`'s compare: at least one step rounded up, below it the variant's delay below its floor, and above it a cost's upper bound.
    assert_eq!(
        at_1_us(ReleaseLine {
            rounding: Some(1_999),
            ..good
        }),
        Judged::Holds
    );
    assert_eq!(
        at_1_us(ReleaseLine {
            rounding: Some(1_000),
            ..good
        }),
        Judged::BelowItsFloor("compare-rounding")
    );
    assert_eq!(
        at_1_us(ReleaseLine {
            rounding: Some(2_001),
            ..good
        }),
        Judged::Holds
    );
    // `SHORTEST` rounds up to a tick before it is a sum's: 1,000.5 µs at 1 MHz is 1,001.
    assert_eq!(
        at_1_us(ReleaseLine {
            shortest: 1_000_500,
            horizon: Some(1_050_000),
            ..good
        }),
        Judged::Refused("the release timer's horizon")
    );
}

/// The arithmetic of `window`, every raw difference and every window a device can state: a due value `behind` and a segment `reach` ahead are told apart exactly where they fit that window and what it leaves, and some window does exactly where the two fit one modulus.
#[test]
fn a_compare_tells_a_late_value_from_a_far_one_only_within_its_modulus() {
    for modulus in [16_i64, 64, 256] {
        for behind in [0_i64, 1, 6, 12] {
            for reach in 1..modulus {
                let told_by = |window: i64| {
                    (-behind..=reach).all(|distance| {
                        let raw = distance.rem_euclid(modulus);
                        // A value up to `window` past reads as due, and what is farther as ahead.
                        let judged_due = raw == 0 || raw >= modulus - window;
                        judged_due == (distance <= 0)
                    })
                };
                for window in 0..modulus {
                    assert_eq!(
                        told_by(window),
                        behind <= window && reach + window < modulus,
                        "modulus {modulus}, reach {reach}, behind {behind}, window {window}"
                    );
                }
                assert_eq!(
                    (0..modulus).any(told_by),
                    reach + behind < modulus,
                    "modulus {modulus}, reach {reach}, behind {behind}"
                );
            }
        }
    }
}

/// A level line stands while its compare reads the value as due, and a request not yet taken goes with it (`sim_devices`' `Controller::lower`). So a request held off for a `wait` past a value landed `behind` is taken then exactly where the two fit the window, and otherwise only when the count comes round: why `window` holds `J_s` as well as `λ + ρ`.
#[test]
fn a_request_waits_only_within_its_window() {
    for modulus in [16_i64, 64, 256] {
        for window in 0..modulus {
            for behind in 0..=window {
                for wait in 0..modulus - behind {
                    // The first instant at or past `wait` that the line stands, counted from the landing.
                    let taken = (wait..)
                        .find(|since| {
                            let raw = (-behind - since).rem_euclid(modulus);
                            raw == 0 || raw >= modulus - window
                        })
                        .expect("the count comes round");
                    if behind + wait <= window {
                        assert_eq!(taken, wait, "{modulus} {window} {behind} {wait}");
                    } else {
                        assert_eq!(
                            taken,
                            modulus - behind,
                            "{modulus} {window} {behind} {wait}"
                        );
                    }
                }
            }
        }
    }
}

/// `count ≥ value` as whole numbers against every window a device could state: even the best misreads a quarter of all pairs of a count and a value, so `compare-window` has a reading of its own for it.
#[test]
fn a_compare_that_reads_whole_numbers_fits_no_window() {
    for modulus in [16_i64, 64, 256] {
        for window in 0..modulus {
            let misread = (0..modulus)
                .flat_map(|count| (0..modulus).map(move |value| (count, value)))
                .filter(|(count, value)| {
                    let raw = (value - count).rem_euclid(modulus);
                    (raw == 0 || raw >= modulus - window) != (count >= value)
                })
                .count();
            assert!(
                misread >= usize::try_from(modulus * modulus / 4).expect("it fits"),
                "modulus {modulus}, window {window}: {misread}"
            );
        }
    }
}

/// What `compare-rounding` is held to is what rounding does: each release over many of a period that is an exact duration, on no grid of the unit's, adds less than a step, and rounded up to the unit it reaches what the relation asks.
#[test]
fn rounding_at_worst_is_what_rounding_adds() {
    for (numerator, denominator, step, rate, unit) in [
        // Period 1/3 ms, at 1 MHz on a unit of 1 µs: 667 ns, nothing on the unit.
        (1_u128, 3_000_u128, 1_u64, 1_000_000_u64, 1_000_u64),
        // 5/2 µs, and 1/3 s at 1 GHz on 1 ns.
        (5, 2_000_000, 1, 1_000_000, 1_000),
        (1, 3, 1, 1_000_000_000, 1),
        // A step of 16, on 17/2 µs, and a thousandth of a tick at 24 MHz, whose worst is all but the whole step.
        (17, 2_000_000, 16, 1_000_000, 1_000),
        (1, 24_000_000_000, 1, 24_000_000, 1),
    ] {
        let (rate_wide, step_wide) = (u128::from(rate), u128::from(step));
        let mut worst = 0_u128;
        for k in 0..2_000_u128 {
            // In ticks, over `denominator`: `k · numerator · rate`.
            let at = k * numerator * rate_wide;
            let next = at.div_ceil(denominator * step_wide) * step_wide * denominator;
            worst = worst.max(next - at);
        }
        assert!(
            worst < step_wide * denominator,
            "less than a step: {numerator}/{denominator} s"
        );
        // Back to a time, up to the unit, as a cost is.
        let rounded = (worst * 1_000_000_000).div_ceil(denominator * rate_wide * u128::from(unit));
        assert_eq!(
            i64::try_from(rounded).expect("it fits"),
            rounding_at_worst(step, rate, unit),
            "{numerator}/{denominator} s, step {step} at {rate} Hz on {unit} ns"
        );
    }
}

/// `sweep` holds `window` over its timelines, each reckoned at a claim at its service's last unit. A timeline takes a request at entry and its claim is arithmetic, so this shows no lost request: it shows a wait coming within `C_s` of a landing and its `J_s`, so that a window short of a real claim could fall under some run's. Past the service's own write the line is lowered, and `C_s` over-charges, as the record says.
#[test]
fn a_request_is_taken_at_its_claim_past_what_its_wait_alone_charges() {
    let mut r = Rng(0x7717_5EED_0246_8ACF);
    let (mut past_the_wait, mut judged) = (0_u32, 0_u32);
    let mut tries = 0;
    while judged < 40 && tries < 40_000 {
        tries += 1;
        let (tasks, alarms, p) = random_set(&mut r, false, false, true, 3);
        let Some((_, j)) = bounds(&tasks, &alarms, p) else {
            continue;
        };
        judged += 1;
        let run = Run {
            tasks: &tasks,
            alarms: &alarms,
            plat: p,
            know: Know::WhatItReadAndWrote,
            device: Device::Compare,
            lag: LagModel {
                max: 3,
                pattern: vec![3],
                past: 0,
            },
            horizon: 8_000,
        }
        .go();
        let rho = i64::try_from(p.res - 1).expect("small");
        for (a, alarm) in alarms.iter().enumerate() {
            let c_s = i64::try_from(alarm.c_s).expect("small");
            assert!(run.worst_standing[a] <= window_charge(3, rho, j[a], c_s));
            if run.worst_standing[a] > window_charge(3, rho, j[a], 0) {
                past_the_wait += 1;
            }
        }
    }
    assert!(judged >= 40, "few sets: {judged}");
    assert!(past_the_wait > 0, "none took its claim past its wait");
}

/// The real devices the records and the reviews name, each as its target would state it, so a relation is held of the figures it will meet and not only of a round number.
#[test]
fn the_devices_the_records_name_read_as_they_should() {
    let line = |rate: u64, step: u64, rounding: u64, unit: u64| {
        judge_release(
            ReleaseLine {
                tick_rate: Some(rate),
                modulus: Some(1 << 32),
                reach: Some(60_000_000_000),
                step: Some(step),
                window: Some(Reads::Within(1 << 31)),
                horizon: Some(60_000_000_000),
                shortest: 1_000_000_000,
                rounding: Some(rounding),
                c_rel: Some(0),
                j_release: 0,
            },
            unit,
        )
    };
    // `compare-rounding`'s honest figure at each, one unit below it, and one above: an overstatement is a cost's upper bound.
    for (rate, step, unit, honest) in [
        (24_000_000, 1, 1, 42),
        (10_000_000, 1, 1, 100),
        (19_200_000, 1, 1, 53),
        (3_000_000, 1, 1, 334),
        (32_768, 1, 1, 30_518),
        (32_768, 1, 1_000, 31_000),
        (1_500_000, 1, 1_000, 1_000),
        (1_000_000, 16, 1_000, 16_000),
        (1_000_000, 1, 1_000, 1_000),
    ] {
        assert_eq!(line(rate, step, honest, unit), Judged::Holds, "{rate}");
        assert_eq!(
            line(rate, step, honest - unit, unit),
            Judged::BelowItsFloor("compare-rounding"),
            "{rate}"
        );
        assert_eq!(
            line(rate, step, honest + unit, unit),
            Judged::Holds,
            "{rate}"
        );
    }
    // A step of 16 and a rounding of nothing is the understatement a bound breaks on, and a step of 1 at no figure is too, a release falling anywhere.
    assert_eq!(
        line(1_000_000, 16, 0, 1_000),
        Judged::BelowItsFloor("compare-rounding")
    );
    assert_eq!(
        line(1_000_000, 1, 0, 1_000),
        Judged::BelowItsFloor("compare-rounding")
    );
    // The machine timer: `mtime` of 64 bits at 10 MHz, read as whole numbers, and a value stays due until it is rewritten.
    let clint = ReleaseLine {
        tick_rate: Some(10_000_000),
        modulus: Some(1 << 64),
        reach: Some(u64::MAX),
        step: Some(1),
        window: Some(Reads::Magnitude),
        horizon: Some(u64::MAX),
        shortest: 1_000_000,
        rounding: Some(100),
        c_rel: Some(50_000),
        j_release: 1_000_000,
    };
    assert_eq!(judge_release(clint, 1), Judged::Holds);
    // At 32 bits it wraps in about seven minutes, past which it misreads what no window would tell it.
    assert_eq!(
        judge_release(
            ReleaseLine {
                modulus: Some(1 << 32),
                ..clint
            },
            1
        ),
        Judged::Refused("compare-window")
    );
    // At 2^55 its wrap is 114 years off, so a value farther than what the operating horizon leaves would read as due at once.
    assert_eq!(
        judge_release(
            ReleaseLine {
                modulus: Some(1 << 55),
                shortest: 15 * 31_557_600_000_000_000,
                ..clint
            },
            1
        ),
        Judged::Refused("the release timer's write")
    );
}

/// Each input a sum reads has a row of the record's other table: who owes it, which way it rounds and how it reaches the checker, so no hand-off can leave one with no path there. A fact a row reads is one the vocabulary has, or one the record says it adds.
#[test]
fn each_input_has_an_owner_a_direction_and_a_path_to_the_checker() {
    let (_, table) = RECORD
        .split_once("<!-- machine-read: alarm-inputs -->")
        .expect("the record's table of inputs");
    let mut named = Vec::new();
    for line in table
        .lines()
        .skip_while(|line| !line.starts_with('|'))
        .take_while(|line| line.starts_with('|'))
        .skip(2)
    {
        let cells: Vec<&str> = line.split('|').map(str::trim).collect();
        assert_eq!(cells.len(), 8, "{line}");
        let name = cells[1].split('`').nth(1).expect("a named input");
        for at in [2, 3, 4, 5, 6] {
            assert!(!cells[at].is_empty(), "{name}: {line}");
        }
        let direction = cells[5].split_whitespace().next().unwrap_or_default();
        assert!(
            matches!(direction, "up" | "down" | "none:"),
            "{name}: `{}`",
            cells[5]
        );
        // A path is a hand-off that says it carries this, or the checker's own working.
        let carried = match cells[6] {
            way if way.contains("computed there") => true,
            way if way.contains("(`RA-H5`)") => row_of("RA-H5").contains(&format!("`{name}`")),
            way if way.contains("(`RA-H1`)") => {
                row_of("RA-H1").contains("that record's §2 table says reaches the checker")
            }
            _ => false,
        };
        assert!(carried, "{name}: `{}`", cells[6]);
        named.push(name.to_owned());
    }
    let sums = ["delivery", "update", "room", "write", "window"];
    for sum in sums {
        for device in [Device::Compare, Device::Delay] {
            let written = cell(sum, device);
            for input in ["T_s", "λ", "ρ", "J_s", "C_s", "P", "R", "W", "H"] {
                if written.contains(input) {
                    assert!(named.iter().any(|name| name == input), "{sum}: {input}");
                }
            }
        }
    }
    // And what each lies within, as the sums' table has it.
    for within in [
        "H",
        "compare-window",
        "delivery-bound",
        "unambiguous-horizon",
        "counter-modulus",
    ] {
        assert!(named.iter().any(|name| name == within), "{within}");
    }
    const VOCABULARY: &str = include_str!("../../../docs/semantics/vocabulary/vocabulary.eadl");
    for each in ["tick-rate", "time-base"] {
        assert!(named.iter().any(|name| name == each), "{each}");
    }
    for fact in [
        "supported-horizon",
        "tick-rate",
        "counter-modulus",
        "unambiguous-horizon",
        "delivery-bound",
        "observation-coherent",
    ] {
        assert!(VOCABULARY.contains(&format!("(deffact {fact}")), "{fact}");
    }
    // `RA-H8`'s five, which this record handed to the vocabulary and leaf `M4.1.7.3.1` declared: each now an entry.
    for added in [
        "alarm-wiring",
        "alarm-kind",
        "compare-step",
        "compare-window",
        "compare-magnitude",
    ] {
        assert!(
            VOCABULARY.contains(&format!("(deffact {added}")),
            "{added}: declared by `M4.1.7.3.1`"
        );
        assert!(RECORD.contains(&format!("`{added}`")), "{added}");
    }
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
                if device == Device::Compare {
                    // With a wake not nothing, the timeline's wake charged to an interrupt already pending (RA4-10) can pass `J_s`, and this holds of these seeds.
                    let held = window_charge(lag_i, rho, j[a], c_s);
                    assert!(
                        run.worst_standing[a] <= held,
                        "taken {} past its match against a window of {held}: {said}",
                        run.worst_standing[a]
                    );
                }
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
            h: 0,
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
                            h: 0,
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
                        h: 0,
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

/// A count of ticks reckons as multiplying first would wherever that fits, and past it gives the largest figure rather than overflowing, to the largest count
/// the language writes (format `/2`'s review, round 10, RP10-3).
#[test]
fn a_count_of_ticks_reckons_exactly_to_the_language_s_largest() {
    for unit in [1_u64, 1_000] {
        let round = Rounding { unit };
        for ticks in [
            0_u128,
            1,
            7,
            999_999_999,
            1_000_000_001,
            u128::from(u64::MAX),
        ] {
            for rate in [1_u64, 3, 32_768, 10_000_000, 1_000_000_000] {
                let multiplied = ticks
                    .checked_mul(1_000_000_000)
                    .map(|ns| ns / u128::from(rate));
                if let Some(ns) = multiplied {
                    assert_eq!(
                        round.ticks_down(ticks, rate),
                        round.down(ns),
                        "{ticks} {rate} {unit}"
                    );
                }
            }
        }
        // `(pow2 99)` and the language's largest, `(pow2 126)`: past every figure, and no overflow.
        assert_eq!(round.ticks_down(1 << 99, 1_000_000_000), i64::MAX);
        assert_eq!(round.ticks_down(1 << 126, 1), i64::MAX);
    }
}
