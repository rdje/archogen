//! The hosted playground simulator (`ROADMAP.md` §6.3; `docs/decisions/decision_hosted-build.md` §3): a deterministic event
//! harness that executes the shared runtime logic, `rt-core`, over a system given as data.
//!
//! §6.3: "The first generated simulator is a composition of device models and a deterministic event harness that executes
//! the shared runtime logic. It is not a newly synthesized full CPU emulator." So nothing here decides what runs: `rt-core`
//! does. This decides *when each thing happens*, by one fixed order, so that one input has one trace.
//!
//! **The order of one instant.** A job with nothing left that takes time finishes first, so its completion is recorded before
//! a release at that instant (`ROADMAP.md` §13.4). Then an interrupt becomes pending at its arrival. While the hardware is
//! unmasked, every pending interrupt is taken, one at a time and none nested: the routes in rank order, then the release timer.
//! Taking one runs entry, its releases and exit. The scheduling decision follows, and its switch; then the running job's body
//! advances until the next arrival or its own next step.
//!
//! It does no I/O: each event goes to an [`Observer`]. Its sources are copied byte for byte into every generated system.

#![forbid(unsafe_code)]

use rt_core::{Decision, OverrunPolicy, Scheduler, TaskState, Transition};

/// The trace's format, as [`Stamped`]'s `Display` writes it.
pub const TRACE_FORMAT: &str = "archogen-playground-trace/2";

/// One step of a job's body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Execute for this many ticks.
    Run(u64),
    /// Mask interrupts: the hardware first, then the runtime's depth.
    Mask,
    /// Unmask interrupts: the runtime's depth first, then the hardware at depth zero.
    Unmask,
}

/// What releases a task.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Trigger {
    /// The release timer, at each multiple of this period in ticks.
    Timer(u64),
    /// A source, by its place in [`System::sources`], on every arrival.
    Source(usize),
}

/// One task, in runtime order: its place is its runtime index, the plan's.
#[derive(Debug, Clone, Copy)]
pub struct Task<'a> {
    /// Its name.
    pub name: &'a str,
    /// Its rank, as the description writes it.
    pub rank: i64,
    /// Its overrun policy.
    pub policy: OverrunPolicy,
    /// What releases it.
    pub trigger: Trigger,
    /// Its job's body.
    pub body: &'a [Step],
}

/// One source of interrupts, in the plan's route rank order.
#[derive(Debug, Clone, Copy)]
pub struct Source<'a> {
    /// Its name.
    pub name: &'a str,
    /// The separation it arrives at, in ticks; `None` where the plan states none, so it does not arrive.
    pub separation: Option<u64>,
}

/// A system, as a run reads it.
#[derive(Debug, Clone, Copy)]
pub struct System<'a> {
    /// Its name.
    pub name: &'a str,
    /// How many ticks make a second.
    pub ticks_per_second: u64,
    /// One past the last instant it runs.
    pub horizon: u64,
    /// Its tasks, in runtime order.
    pub tasks: &'a [Task<'a>],
    /// Its sources, in the plan's route rank order.
    pub sources: &'a [Source<'a>],
    /// Its initialisation's steps, each the nodes of one component.
    pub initialisation: &'a [&'a [&'a str]],
}

/// An interrupt.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interrupt<'a> {
    /// The release timer.
    Timer,
    /// A source.
    Source(&'a str),
}

/// What a run observes: §6.3's events, those a system of this harness can raise.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Event<'a> {
    /// A node initialised, in a step of the plan.
    Initialised {
        /// The step's number, from one.
        step: usize,
        /// The node.
        node: &'a str,
    },
    /// Interrupts first enabled.
    Enabled,
    /// The timer programmed for an instant, in ticks.
    TimerProgrammed {
        /// The instant.
        at: u64,
    },
    /// An interrupt became pending.
    InterruptPending(Interrupt<'a>),
    /// An interrupt's service entered.
    InterruptEntry(Interrupt<'a>),
    /// An interrupt's service left.
    InterruptExit(Interrupt<'a>),
    /// A task released.
    Release(&'a str),
    /// A job started.
    Start(&'a str),
    /// A running job preempted.
    Preempt(&'a str),
    /// A preempted job resumed.
    Resume(&'a str),
    /// A job completed.
    Complete(&'a str),
    /// A job masked interrupts.
    Mask(&'a str),
    /// A job unmasked interrupts.
    Unmask(&'a str),
    /// A task overran, its job abandoned for the next.
    JobSkipped(&'a str),
    /// A task overran and left the schedule.
    Faulted(&'a str),
    /// The runtime halted.
    Halted,
}

/// An event with its instant, as the trace writes a line.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Stamped<'a> {
    /// The instant, in ticks.
    pub at: u64,
    /// How many ticks make a second.
    pub ticks_per_second: u64,
    /// What happened.
    pub event: Event<'a>,
}

/// `a`'s and `b`'s greatest common divisor.
const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

impl core::fmt::Display for Stamped<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let divisor = gcd(self.at, self.ticks_per_second).max(1);
        let (numerator, denominator) = (self.at / divisor, self.ticks_per_second / divisor);
        if denominator == 1 {
            write!(f, "{numerator} s ")?;
        } else {
            write!(f, "{numerator}/{denominator} s ")?;
        }
        let interrupt = |interrupt: Interrupt<'_>| match interrupt {
            Interrupt::Timer => "timer".to_owned(),
            Interrupt::Source(name) => format!("source {name}"),
        };
        match self.event {
            Event::Initialised { step, node } => write!(f, "initialised {step} {node}"),
            Event::Enabled => write!(f, "enabled"),
            Event::TimerProgrammed { at } => write!(f, "timer-programmed {at}"),
            Event::InterruptPending(i) => write!(f, "pending {}", interrupt(i)),
            Event::InterruptEntry(i) => write!(f, "entry {}", interrupt(i)),
            Event::InterruptExit(i) => write!(f, "exit {}", interrupt(i)),
            Event::Release(task) => write!(f, "release {task}"),
            Event::Start(task) => write!(f, "start {task}"),
            Event::Preempt(task) => write!(f, "preempt {task}"),
            Event::Resume(task) => write!(f, "resume {task}"),
            Event::Complete(task) => write!(f, "complete {task}"),
            Event::Mask(task) => write!(f, "mask {task}"),
            Event::Unmask(task) => write!(f, "unmask {task}"),
            Event::JobSkipped(task) => write!(f, "job-skipped {task}"),
            Event::Faulted(task) => write!(f, "faulted {task}"),
            Event::Halted => write!(f, "halted"),
        }
    }
}

/// Where a run's events go.
pub trait Observer {
    /// One event, in the run's order.
    fn observe(&mut self, event: Stamped<'_>);
}

/// How a run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ended {
    /// It reached its horizon.
    Horizon,
    /// The runtime halted.
    Halted,
    /// The system could not be booted: its tasks' ranks were not the runtime's.
    NotBooted,
}

/// A job under way: where its body has reached, and what is left of the step it is in.
#[derive(Debug, Clone, Copy, Default)]
struct Job {
    step: usize,
    left: u64,
    started: bool,
}

/// `system` run to its horizon, each event handed to `observer`.
pub fn run<const N: usize>(system: &System<'_>, observer: &mut dyn Observer) -> Ended {
    Run::<N>::new(system, observer).map_or(Ended::NotBooted, Run::go)
}

struct Run<'s, 'o, const N: usize> {
    system: &'s System<'s>,
    observer: &'o mut dyn Observer,
    scheduler: Scheduler<N>,
    /// Each task's job, while it owes one.
    jobs: [Job; N],
    /// Each timer-released task's next nominal release; `u64::MAX` for another's.
    releases: [u64; N],
    /// Each source's next arrival, in step with [`System::sources`].
    arrivals: Vec<Option<u64>>,
    /// Each source's interrupt, pending.
    pending: Vec<bool>,
    /// The timer's interrupt, pending.
    timer_pending: bool,
    /// Whether the hardware is masked.
    masked: bool,
    now: u64,
}

impl<'s, 'o, const N: usize> Run<'s, 'o, N> {
    /// The run at time zero, the scheduler built by rank: the plan's index is `rt-core`'s slot, or nothing runs.
    fn new(system: &'s System<'s>, observer: &'o mut dyn Observer) -> Option<Self> {
        if system.tasks.len() != N {
            return None;
        }
        let ranks: [i64; N] = core::array::from_fn(|i| system.tasks[i].rank);
        let policies: [OverrunPolicy; N] = core::array::from_fn(|i| system.tasks[i].policy);
        let scheduler = Scheduler::from_eadl_ranks(ranks, policies).ok()?;
        // `from_eadl_ranks` holds them in rank order, so runtime order must already be it.
        if ranks.windows(2).any(|pair| pair[0] >= pair[1]) {
            return None;
        }
        Some(Self {
            system,
            observer,
            scheduler,
            jobs: [Job::default(); N],
            releases: core::array::from_fn(|i| match system.tasks[i].trigger {
                Trigger::Timer(_) => 0,
                Trigger::Source(_) => u64::MAX,
            }),
            arrivals: system
                .sources
                .iter()
                .map(|source| source.separation.map(|_| 0))
                .collect(),
            pending: vec![false; system.sources.len()],
            timer_pending: false,
            masked: false,
            now: 0,
        })
    }

    fn emit(&mut self, event: Event<'s>) {
        self.observer.observe(Stamped {
            at: self.now,
            ticks_per_second: self.system.ticks_per_second,
            event,
        });
    }

    /// The instant the release timer is next due, within the horizon.
    fn next_release(&self) -> Option<u64> {
        self.releases
            .iter()
            .copied()
            .filter(|at| *at < self.system.horizon)
            .min()
    }

    fn go(mut self) -> Ended {
        // Initialisation runs first, in the plan's steps, interrupts masked (`RTP-H9`).
        let system = self.system;
        for (number, step) in system.initialisation.iter().enumerate() {
            for node in *step {
                self.emit(Event::Initialised {
                    step: number + 1,
                    node,
                });
            }
        }
        self.emit(Event::Enabled);
        if let Some(at) = self.next_release() {
            self.emit(Event::TimerProgrammed { at });
        }
        loop {
            if self.finish_what_is_due() {
                return Ended::Halted;
            }
            self.arrive();
            if !self.masked && self.take_interrupts() {
                return Ended::Halted;
            }
            if self.decide() {
                return Ended::Halted;
            }
            match self.advance() {
                Advance::Stepped => {}
                Advance::Halted => return Ended::Halted,
                Advance::Horizon => return Ended::Horizon,
            }
        }
    }

    /// The running job finished, if it has nothing left that takes time, so that its completion comes before what arrives at
    /// this instant (`ROADMAP.md` §13.4). It stops at an unmask that lets a pending interrupt in, which is taken before the job's
    /// next instruction, its completion among them. True where the runtime halted.
    fn finish_what_is_due(&mut self) -> bool {
        loop {
            let interrupted =
                !self.masked && (self.timer_pending || self.pending.iter().any(|pending| *pending));
            if interrupted || !self.has_no_time_left() {
                return false;
            }
            if matches!(self.advance(), Advance::Halted) {
                return true;
            }
        }
    }

    /// Whether a job is running with nothing left that takes time.
    fn has_no_time_left(&self) -> bool {
        (0..N)
            .find(|task| self.scheduler.state(*task) == TaskState::Running)
            .is_some_and(|task| {
                let job = self.jobs[task];
                job.left == 0
                    && self.system.tasks[task]
                        .body
                        .iter()
                        .skip(job.step)
                        .all(|step| !matches!(step, Step::Run(ticks) if *ticks > 0))
            })
    }

    /// Whatever arrives at this instant becomes pending, whether or not anything can take it.
    fn arrive(&mut self) {
        let system = self.system;
        for (place, source) in system.sources.iter().enumerate() {
            let (Some(at), Some(separation)) = (self.arrivals[place], source.separation) else {
                continue;
            };
            if at == self.now && at < system.horizon {
                self.pending[place] = true;
                self.arrivals[place] = at.checked_add(separation.max(1));
                self.emit(Event::InterruptPending(Interrupt::Source(source.name)));
            }
        }
        // Only when a release is due (`RTP-H9`).
        if self.next_release() == Some(self.now) && !self.timer_pending {
            self.timer_pending = true;
            self.emit(Event::InterruptPending(Interrupt::Timer));
        }
    }

    /// Take every pending interrupt, one at a time and none nested, the routes in rank order and then the timer. True where
    /// the runtime halted.
    fn take_interrupts(&mut self) -> bool {
        let system = self.system;
        loop {
            if let Some(place) = self.pending.iter().position(|pending| *pending) {
                self.pending[place] = false;
                let name = system.sources[place].name;
                self.emit(Event::InterruptEntry(Interrupt::Source(name)));
                for (task, spec) in system.tasks.iter().enumerate() {
                    if spec.trigger == Trigger::Source(place) && self.release(task) {
                        return true;
                    }
                }
                self.emit(Event::InterruptExit(Interrupt::Source(name)));
            } else if self.timer_pending {
                self.timer_pending = false;
                self.emit(Event::InterruptEntry(Interrupt::Timer));
                for (task, spec) in system.tasks.iter().enumerate() {
                    if let Trigger::Timer(period) = spec.trigger {
                        // A release masked past its instant is late, and its next stays nominal, each owed one taken.
                        while self.releases[task] <= self.now {
                            self.releases[task] = self.releases[task].saturating_add(period.max(1));
                            if self.release(task) {
                                return true;
                            }
                        }
                    }
                }
                if let Some(at) = self.next_release() {
                    self.emit(Event::TimerProgrammed { at });
                }
                self.emit(Event::InterruptExit(Interrupt::Timer));
            } else {
                return false;
            }
        }
    }

    /// `task` released, as `rt-core` decides. True where the runtime halted.
    fn release(&mut self, task: usize) -> bool {
        let name = self.system.tasks[task].name;
        match self.scheduler.release(task) {
            Transition::Released { .. } => {
                self.jobs[task] = Job::default();
                self.emit(Event::Release(name));
                false
            }
            Transition::JobSkipped { .. } => {
                self.jobs[task] = Job::default();
                self.emit(Event::JobSkipped(name));
                self.emit(Event::Release(name));
                false
            }
            Transition::Faulted { .. } => {
                self.emit(Event::Faulted(name));
                false
            }
            Transition::Halted { .. } => {
                self.emit(Event::Halted);
                true
            }
            // No service runs with the depth raised, so nothing is ever latched; a completion is no release's.
            Transition::Latched { .. }
            | Transition::OverrunLatched { .. }
            | Transition::Completed { .. } => false,
        }
    }

    /// The scheduling decision, and its switch. True where the runtime halted.
    fn decide(&mut self) -> bool {
        match self.scheduler.decide() {
            Decision::Idle | Decision::Continue { .. } => false,
            Decision::Dispatch { to } => {
                self.enter(to);
                false
            }
            Decision::Switch { from, to } => {
                self.emit(Event::Preempt(self.system.tasks[from].name));
                self.enter(to);
                false
            }
            Decision::Halt { .. } => {
                self.emit(Event::Halted);
                true
            }
        }
    }

    /// The transition into `task`: its job's start, or its preempted job's resumption.
    fn enter(&mut self, task: usize) {
        let name = self.system.tasks[task].name;
        if self.jobs[task].started {
            self.emit(Event::Resume(name));
        } else {
            self.jobs[task].started = true;
            self.emit(Event::Start(name));
        }
    }

    /// The running job's body advanced to its next event, or time to the next arrival.
    fn advance(&mut self) -> Advance {
        let next = self.next_event();
        let running = (0..N).find(|task| self.scheduler.state(*task) == TaskState::Running);
        let Some(task) = running else {
            return match next {
                Some(at) => {
                    self.now = at;
                    Advance::Stepped
                }
                None => {
                    self.now = self.system.horizon;
                    Advance::Horizon
                }
            };
        };
        let name = self.system.tasks[task].name;
        let body = self.system.tasks[task].body;
        let job = self.jobs[task];
        match body.get(job.step).copied() {
            None => {
                let (completed, _) = self.scheduler.complete(task);
                if let Transition::Halted { .. } = completed {
                    self.emit(Event::Halted);
                    return Advance::Halted;
                }
                self.masked = false;
                self.jobs[task] = Job::default();
                self.emit(Event::Complete(name));
                Advance::Stepped
            }
            Some(Step::Mask) => {
                self.masked = true;
                if self.scheduler.mask().is_err() {
                    self.emit(Event::Halted);
                    return Advance::Halted;
                }
                self.jobs[task].step += 1;
                self.emit(Event::Mask(name));
                Advance::Stepped
            }
            Some(Step::Unmask) => {
                if self.scheduler.unmask().is_err() {
                    self.emit(Event::Halted);
                    return Advance::Halted;
                }
                self.masked = self.scheduler.is_masked();
                self.jobs[task].step += 1;
                self.emit(Event::Unmask(name));
                Advance::Stepped
            }
            Some(Step::Run(ticks)) => {
                let left = if job.left == 0 { ticks } else { job.left };
                let finish = self.now.saturating_add(left);
                let until = next.map_or(finish, |at| at.min(finish));
                if until >= self.system.horizon && next.is_none() && finish >= self.system.horizon {
                    self.now = self.system.horizon;
                    return Advance::Horizon;
                }
                self.jobs[task].left = finish - until;
                if finish == until {
                    self.jobs[task].step += 1;
                }
                self.now = until;
                Advance::Stepped
            }
        }
    }

    /// The instant of the next arrival or due release, after this one and within the horizon.
    fn next_event(&self) -> Option<u64> {
        let arrivals = self.arrivals.iter().flatten().copied();
        self.releases
            .iter()
            .copied()
            .chain(arrivals)
            .filter(|at| *at > self.now && *at < self.system.horizon)
            .min()
    }
}

/// What advancing did.
enum Advance {
    /// Time or a body moved on.
    Stepped,
    /// The runtime halted.
    Halted,
    /// The horizon was reached.
    Horizon,
}
