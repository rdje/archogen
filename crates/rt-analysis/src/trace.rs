//! A fixed-trace simulator for the §13.4 operational model, charging under `cost-accounting/1`.
//!
//! `ROADMAP.md` §13.4 specifies a repeated-preemption fixture completely — costs, transitions,
//! preemptibility, and the instant-by-instant trace it must produce — and then says why it is
//! shaped that way:
//!
//! > It deliberately creates two preemptions of one low-priority job, so **omitted interrupt or
//! > resume costs can turn a real miss in the fixture into a false pass**.
//!
//! ⛔ **The controls require re-simulation, not subtraction.** §13.4 is explicit:
//!
//! > Re-simulate mutations that change execution timing: they can change the number of
//! > interfering releases, so subtracting a fixed number from the original response is not
//! > generally valid.
//!
//! That is the whole reason this module exists rather than a hand-written table. Zeroing the
//! interrupt cost does not move `L`'s completion by two units; it changes *when* the second `H`
//! release is serviced, and the answer (21) has to be computed. A control implemented by
//! arithmetic on the original trace would agree with the roadmap by luck and diverge on the next
//! fixture.
//!
//! # Deliberately not a general scheduler
//!
//! Two tasks: one low-priority job, and one higher-priority periodic task with nominal releases.
//! That is the shape §13.4 specifies, and this module claims nothing beyond it. The general
//! runtime state machine is `rt-core` (leaf `M2.1`) and the general event harness is the hosted
//! simulator (leaf `M4.3`); a "general" simulator written here to serve one fixture would be a
//! second, unreviewed one of those.

use crate::cost::{Accounting, Category, Interval, Ledger, LedgerError};

/// The costs of `cost-accounting/1`, in abstract integer time units.
///
/// The two switch directions are separate fields because §13.4's third control omits exactly one
/// of them — "omit H-to-L switch costs" — and a single `switch` field would make that control
/// inexpressible without editing the simulator, which is not a control at all.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Costs {
    /// Idle to the first task. No outgoing context exists, so this is not a task switch.
    pub initial_dispatch: u64,
    /// One timer ISR per release of the higher-priority task.
    pub interrupt_service: u64,
    /// Switching *out of* the low-priority task and into the high-priority one.
    pub preempt_switch: u64,
    /// Switching back: the resume transition §13.4's third control deletes.
    pub resume_switch: u64,
}

impl Costs {
    /// The correct model of §13.4: dispatch 1, ISR 1, and 2 for each switch direction
    /// ("one unit for saving the outgoing context and one for restoring the incoming context").
    pub const SPECIFIED: Self = Self {
        initial_dispatch: 1,
        interrupt_service: 1,
        preempt_switch: 2,
        resume_switch: 2,
    };
}

/// The §13.4 workload: one low-priority job, one higher-priority periodic task.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Workload {
    /// The low-priority task's name.
    pub low: String,
    /// Its useful computation.
    pub low_computation: u64,
    /// The high-priority task's name.
    pub high: String,
    /// Its useful computation, per job.
    pub high_computation: u64,
    /// Its first nominal release.
    pub high_first_release: u64,
    /// Its period, so nominal releases are `first, first + period, …`.
    pub high_period: u64,
}

impl Workload {
    /// §13.4's workload: `L` with computation 8, `H` with computation 2 released at 4, 14, 24, …
    #[must_use]
    pub fn specified() -> Self {
        Self {
            low: "L".into(),
            low_computation: 8,
            high: "H".into(),
            high_computation: 2,
            high_first_release: 4,
            high_period: 10,
        }
    }
}

/// A completed job: what finished, when it was nominally released, and when it completed.
///
/// "Completed" means the end of useful computation — §13.4's observation boundary. The switch
/// away from the job is outside its response interval and remains interference for others.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Completion {
    /// The task.
    pub task: String,
    /// Its nominal release instant, which the deadline is relative to.
    pub released: u64,
    /// The instant its useful computation ended.
    pub completed: u64,
}

impl Completion {
    /// The response time: completion minus **nominal** release, so any readiness delay caused by
    /// the release interrupt is inside it (§13.4).
    #[must_use]
    pub const fn response(&self) -> u64 {
        self.completed - self.released
    }
}

/// What one simulation produced.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simulation {
    /// Every charged interval, sealed as an exact trace.
    pub ledger: Ledger,
    /// Every job that completed, in completion order.
    pub completions: Vec<Completion>,
}

impl Simulation {
    /// When the low-priority task's single job completed.
    ///
    /// # Panics
    ///
    /// If the simulation ended before it completed, which cannot happen: the run ends there.
    #[must_use]
    pub fn low_completion(&self, workload: &Workload) -> u64 {
        self.completions
            .iter()
            .find(|c| c.task == workload.low)
            .expect("the simulation ends when the low-priority job completes")
            .completed
    }

    /// Every completion of the high-priority task, in order.
    #[must_use]
    pub fn high_completions(&self, workload: &Workload) -> Vec<u64> {
        self.completions
            .iter()
            .filter(|c| c.task == workload.high)
            .map(|c| c.completed)
            .collect()
    }
}

/// One charged segment, added only if it has a duration.
///
/// A zero-cost segment is not a zero-length interval in the ledger: `Ledger` refuses an empty
/// interval, and rightly — an interval that charges nothing is not a charge. Skipping it leaves
/// the coverage gapless, because it consumed no time.
fn charge(
    intervals: &mut Vec<Interval>,
    now: &mut u64,
    duration: u64,
    category: Category,
    activity: &str,
) {
    if duration == 0 {
        return;
    }
    intervals.push(Interval {
        start: *now,
        end: *now + duration,
        category,
        activity: activity.to_string(),
    });
    *now += duration;
}

/// Simulate the §13.4 model under `costs`, ending when the low-priority job completes.
///
/// # Errors
///
/// A [`LedgerError`] if the produced trace does not seal as an exact trace — which would be a bug
/// in this simulator, and is surfaced rather than swallowed.
///
/// # Panics
///
/// If the high-priority task's period is zero, which would make its release set infinite at one
/// instant.
pub fn simulate(workload: &Workload, costs: Costs) -> Result<Simulation, LedgerError> {
    assert!(workload.high_period > 0, "a zero period has no release set");

    let mut intervals: Vec<Interval> = Vec::new();
    let mut completions: Vec<Completion> = Vec::new();
    let mut now: u64 = 0;
    let mut low_remaining = workload.low_computation;
    // The next nominal release of the high-priority task that has not yet been serviced.
    let mut next_release = workload.high_first_release;

    // The initial state already contains the low-priority job, and there is no time-0 release
    // ISR (§13.4). Getting from idle to it is a dispatch, not a switch: nothing is saved.
    charge(
        &mut intervals,
        &mut now,
        costs.initial_dispatch,
        Category::InitialDispatch,
        &format!("Initial dispatch to {}", workload.low),
    );

    loop {
        // ── the low-priority task computes until it finishes or a release interrupts it ───────
        let until_release = next_release.saturating_sub(now);
        let slice = low_remaining.min(until_release);
        charge(
            &mut intervals,
            &mut now,
            slice,
            Category::TaskExecution(workload.low.clone()),
            &format!("{} computation", workload.low),
        );
        low_remaining -= slice;

        // §13.4: "At a coincident completion/release instant, record completion before processing
        // the new release." So completion is tested first, and the run ends there.
        if low_remaining == 0 {
            completions.push(Completion {
                task: workload.low.clone(),
                released: 0,
                completed: now,
            });
            break;
        }

        // ── the release: an ISR, then a preemption ───────────────────────────────────────────
        let nominal = next_release;
        charge(
            &mut intervals,
            &mut now,
            costs.interrupt_service,
            Category::InterruptService,
            &format!("Timer ISR for {} release", workload.high),
        );
        next_release += workload.high_period;

        charge(
            &mut intervals,
            &mut now,
            costs.preempt_switch,
            Category::TaskSwitch,
            &format!("Switch {} to {}", workload.low, workload.high),
        );

        // ── the high-priority job runs to completion; nothing preempts it here ───────────────
        charge(
            &mut intervals,
            &mut now,
            workload.high_computation,
            Category::TaskExecution(workload.high.clone()),
            &format!("{} job", workload.high),
        );
        completions.push(Completion {
            task: workload.high.clone(),
            released: nominal,
            completed: now,
        });

        // ⛔ Switches are nonpreemptible and arrivals during them are latched (§13.4). In the
        // specified trace there are none, and a fixture that silently grew one would be a
        // different fixture — so it is asserted rather than assumed.
        let resume_end = now + costs.resume_switch;
        assert!(
            !(next_release > now && next_release < resume_end),
            "a {} release at {next_release} falls inside a non-preemptible resume switch \
             [{now}, {resume_end}); §13.4 states the correct trace has no such arrival, so this \
             model needs latching before it can be trusted",
            workload.high
        );
        charge(
            &mut intervals,
            &mut now,
            costs.resume_switch,
            Category::TaskSwitch,
            &format!("Switch {} to {}", workload.high, workload.low),
        );
    }

    Ok(Simulation {
        ledger: Ledger::seal(intervals, Accounting::ExactTrace)?,
        completions,
    })
}
