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

use rt_core::{Decision, Fatal, Fault, OverrunPolicy, Scheduler, TaskState, Transition};

/// The trace's format, as [`Stamped`]'s `Display` writes it.
pub const TRACE_FORMAT: &str = "archogen-playground-trace/4";

/// One step of a job's body.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Step {
    /// Execute for this many ticks.
    Run(u64),
    /// Mask interrupts: the hardware first, then the runtime's depth.
    Mask,
    /// Unmask interrupts: the runtime's depth first, then the hardware at depth zero.
    Unmask,
    /// Read a device, by its place in [`System::devices`]: no time passes, and the run shows it.
    Read(usize),
    /// Write a device, by its place in [`System::devices`]: no time passes, and the run shows it.
    Write(usize),
    /// Program an alarm, by its place in [`System::sources`], for this many ticks ahead of the write. A value already reached
    /// arrives at the write and is never lost, as a level compare's does; what stood is replaced.
    Program {
        /// Its place.
        alarm: usize,
        /// How far ahead.
        ahead: u64,
    },
}

/// What a plan's `node` line says a device is. It is read from the plan, and never from the description (`RTP-H9`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeviceKind {
    /// A block.
    Block,
    /// A platform.
    Platform,
}

/// One device the plan names.
#[derive(Debug, Clone, Copy)]
pub struct Device<'a> {
    /// Its name.
    pub name: &'a str,
    /// Its kind.
    pub kind: DeviceKind,
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
    /// The separation it arrives at, in ticks; `None` where the plan states none, so it does not arrive by itself.
    pub separation: Option<u64>,
    /// Whether it is an alarm, a timer route, which arrives where a step programs it and at no other time.
    pub alarm: bool,
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
    /// Its sources, each alarm among them, in the plan's route rank order.
    pub sources: &'a [Source<'a>],
    /// Each device the plan names.
    pub devices: &'a [Device<'a>],
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
    /// An alarm.
    Alarm(&'a str),
}

/// What a task's overrun policy did (`docs/profiles/rt-static-up-v1-faults-observation.md` §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Outcome {
    /// The late job is abandoned, and the release that found it is the task's next job.
    SkipLateJob,
    /// The task leaves the schedule.
    Fault,
}

/// A fatal fault's kind and what it carries, its tasks by name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FatalKind<'a> {
    /// An overrun a rule made fatal.
    Overrun {
        /// The task that overran.
        task: &'a str,
    },
    /// A stack guard touched.
    StackGuard {
        /// Whose guard: a task's, or none for the interrupt stack's.
        guard: Option<&'a str>,
    },
    /// A trap the runtime does not model.
    UnexpectedTrap {
        /// The architecture's cause, uninterpreted.
        cause: u64,
    },
    /// An assertion failure, the contract's `InvariantViolated`.
    InvariantViolated {
        /// Which case of the observation record's §4 item 5.
        invariant: &'static str,
    },
}

/// What a run observes: §6.3's events, those a system of this harness can raise, with the fields
/// `docs/profiles/rt-static-up-v1-faults-observation.md` §2 fixes on the fault paths.
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
    /// A job read a device.
    DeviceRead(&'a str),
    /// A job wrote a device.
    DeviceWrite(&'a str),
    /// A job programmed an alarm for an instant, in ticks.
    AlarmProgrammed {
        /// The alarm.
        alarm: &'a str,
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
    Release {
        /// The task.
        task: &'a str,
        /// For a timer-released task, the release's nominal instant, in ticks.
        nominal: Option<u64>,
    },
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
    /// Idle's first instruction after a transition into it; a trap that returns to idle stamps none.
    Idle,
    /// A release found its task owing a job: one event, and no `Release` beside it.
    Overrun {
        /// The task.
        task: &'a str,
        /// What its policy did.
        outcome: Outcome,
        /// For a timer-released task, the triggering release's nominal instant, in ticks.
        nominal: Option<u64>,
    },
    /// The runtime halted, with what its fatal handler keeps.
    Fatal {
        /// The fault, which carries its kind.
        kind: FatalKind<'a>,
        /// The task it is attributed to, if a task's code raised it.
        attributed: Option<&'a str>,
        /// The task whose job a fault raised in kernel context interrupted.
        interrupted: Option<&'a str>,
        /// Whether a rule escalated a containable fault.
        escalated: bool,
    },
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

/// `at` ticks as exact seconds.
struct Seconds {
    at: u64,
    ticks_per_second: u64,
}

impl core::fmt::Display for Seconds {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let divisor = gcd(self.at, self.ticks_per_second).max(1);
        let (numerator, denominator) = (self.at / divisor, self.ticks_per_second / divisor);
        if denominator == 1 {
            write!(f, "{numerator} s")
        } else {
            write!(f, "{numerator}/{denominator} s")
        }
    }
}

impl core::fmt::Display for Stamped<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let seconds = |at: u64| Seconds {
            at,
            ticks_per_second: self.ticks_per_second,
        };
        let nominal = |f: &mut core::fmt::Formatter<'_>, at: Option<u64>| match at {
            Some(at) => write!(f, " nominal {}", seconds(at)),
            None => Ok(()),
        };
        write!(f, "{} ", seconds(self.at))?;
        let interrupt = |interrupt: Interrupt<'_>| match interrupt {
            Interrupt::Timer => "timer".to_owned(),
            Interrupt::Source(name) => format!("source {name}"),
            Interrupt::Alarm(name) => format!("alarm {name}"),
        };
        match self.event {
            Event::Initialised { step, node } => write!(f, "initialised {step} {node}"),
            Event::Enabled => write!(f, "enabled"),
            Event::TimerProgrammed { at } => write!(f, "timer-programmed {}", seconds(at)),
            Event::DeviceRead(device) => write!(f, "device-read {device}"),
            Event::DeviceWrite(device) => write!(f, "device-write {device}"),
            Event::AlarmProgrammed { alarm, at } => {
                write!(f, "alarm-programmed {alarm} {}", seconds(at))
            }
            Event::InterruptPending(i) => write!(f, "pending {}", interrupt(i)),
            Event::InterruptEntry(i) => write!(f, "entry {}", interrupt(i)),
            Event::InterruptExit(i) => write!(f, "exit {}", interrupt(i)),
            Event::Release { task, nominal: at } => {
                write!(f, "release {task}")?;
                nominal(f, at)
            }
            Event::Start(task) => write!(f, "start {task}"),
            Event::Preempt(task) => write!(f, "preempt {task}"),
            Event::Resume(task) => write!(f, "resume {task}"),
            Event::Complete(task) => write!(f, "complete {task}"),
            Event::Mask(task) => write!(f, "mask {task}"),
            Event::Unmask(task) => write!(f, "unmask {task}"),
            Event::Idle => write!(f, "idle"),
            Event::Overrun {
                task,
                outcome,
                nominal: at,
            } => {
                let outcome = match outcome {
                    Outcome::SkipLateJob => "skip-late-job",
                    Outcome::Fault => "fault",
                };
                write!(f, "fault overrun {task} {outcome}")?;
                nominal(f, at)
            }
            Event::Fatal {
                kind,
                attributed,
                interrupted,
                escalated,
            } => {
                match kind {
                    FatalKind::Overrun { task } => write!(f, "fault overrun {task} halted")?,
                    FatalKind::StackGuard { guard: Some(task) } => {
                        write!(f, "fault stack-guard {task}")?;
                    }
                    FatalKind::StackGuard { guard: None } => {
                        write!(f, "fault stack-guard interrupt-stack")?;
                    }
                    FatalKind::UnexpectedTrap { cause } => {
                        write!(f, "fault unexpected-trap {cause}")?;
                    }
                    FatalKind::InvariantViolated { invariant } => {
                        write!(f, "fault invariant-violated {invariant}")?;
                    }
                }
                if let Some(task) = attributed {
                    write!(f, " attributed {task}")?;
                }
                if let Some(task) = interrupted {
                    write!(f, " interrupted {task}")?;
                }
                write!(f, " escalated {}", if escalated { "yes" } else { "no" })
            }
        }
    }
}

/// Where a run's events go.
pub trait Observer {
    /// One event, in the run's order.
    fn observe(&mut self, event: Stamped<'_>);
}

/// How many steps a run takes at one instant before it stops: what bounds a hyperperiod's releases (`decision_hosted-build.md` §3).
pub const MOST_STEPS_AT_ONE_INSTANT: u64 = 1 << 20;

/// How many alarm arrivals a run takes before it stops. A plan's count bounds a timer's releases and a source's arrivals, and none says what a job programs, so a job an alarm releases can program that alarm a tick ahead until a far horizon.
pub const MOST_ALARM_ARRIVALS: u64 = 1 << 20;

/// The stack a run of `tasks` tasks takes, on any profile: what the runtime holds and returns by value grows with its tasks, and a thread's default does not.
#[must_use]
pub const fn stack_for(tasks: usize) -> usize {
    (1 << 23) + tasks.saturating_mul(1 << 13)
}

/// How a run ended.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ended {
    /// It reached its horizon.
    Horizon,
    /// The runtime halted.
    Halted,
    /// It took [`MOST_STEPS_AT_ONE_INSTANT`] steps with time standing still: a body that long, or a job and what it programs keeping each other going.
    NoProgress,
    /// It took [`MOST_ALARM_ARRIVALS`] arrivals of its alarms.
    TooManyArrivals,
    /// The system could not be booted: its tasks' ranks were not the runtime's.
    NotBooted,
}

/// A job under way: where its body has reached, what is left of the step it is in, and how far its body is known to take no time.
#[derive(Debug, Clone, Copy, Default)]
struct Job {
    step: usize,
    left: u64,
    started: bool,
    /// No step from [`Job::step`] up to this one takes time.
    next_timed: usize,
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
    /// Each alarm's compare, where one stands, in step with [`System::sources`].
    compares: Vec<Option<u64>>,
    /// How many requests each source's interrupt has pending: at most one of a source, whose two before it is taken are one, and
    /// one per arrival of an alarm (`RA-H3`).
    pending: Vec<u32>,
    /// The timer's interrupt, pending.
    timer_pending: bool,
    /// Whether the hardware is masked.
    masked: bool,
    /// Whether idle has been entered and nothing has run since, so that a trap returning to it stamps no entry.
    idle: bool,
    now: u64,
    /// The instant steps were last counted at, and how many it has taken.
    counted_at: u64,
    steps_here: u64,
    /// How many arrivals its alarms have had.
    alarm_arrivals: u64,
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
        // A step names what the plan holds, or nothing runs.
        let named = |step: &Step| match *step {
            Step::Read(device) | Step::Write(device) => device < system.devices.len(),
            Step::Program { alarm, .. } => {
                system.sources.get(alarm).is_some_and(|source| source.alarm)
            }
            Step::Run(_) | Step::Mask | Step::Unmask => true,
        };
        if !system.tasks.iter().all(|task| task.body.iter().all(named)) {
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
            compares: vec![None; system.sources.len()],
            pending: vec![0; system.sources.len()],
            timer_pending: false,
            masked: false,
            idle: false,
            now: 0,
            counted_at: 0,
            steps_here: 0,
            alarm_arrivals: 0,
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
            if let Some(ended) = self.finish_what_is_due() {
                return ended;
            }
            self.arrive();
            if self.alarm_arrivals > MOST_ALARM_ARRIVALS {
                return Ended::TooManyArrivals;
            }
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
                Advance::NoProgress => return Ended::NoProgress,
            }
        }
    }

    /// The running job finished, if it has nothing left that takes time, so that its completion comes before what arrives at
    /// this instant (`ROADMAP.md` §13.4), and then the next that has nothing left, however it came to be held off. It stops at an
    /// unmask that lets a pending interrupt in, which is taken before the job's next instruction, its completion among them. How
    /// the run ended, where it did.
    fn finish_what_is_due(&mut self) -> Option<Ended> {
        loop {
            let interrupted = !self.masked
                && (self.timer_pending || self.pending.iter().any(|pending| *pending > 0));
            if interrupted {
                return None;
            }
            if !self.has_no_time_left() {
                if !self.next_has_no_time_left() {
                    return None;
                }
                if self.decide() {
                    return Some(Ended::Halted);
                }
                continue;
            }
            match self.advance() {
                Advance::Halted => return Some(Ended::Halted),
                Advance::NoProgress => return Some(Ended::NoProgress),
                Advance::Stepped | Advance::Horizon => {}
            }
        }
    }

    /// Whether a job is running with nothing left that takes time.
    fn has_no_time_left(&mut self) -> bool {
        let Some(task) = (0..N).find(|task| self.scheduler.state(*task) == TaskState::Running)
        else {
            return false;
        };
        self.holds_no_time(task)
    }

    /// Whether nothing is running, and what would run next has nothing left that takes time. The index is the rank, as the scheduler's decision reads it.
    fn next_has_no_time_left(&mut self) -> bool {
        if (0..N).any(|task| self.scheduler.state(task) == TaskState::Running) {
            return false;
        }
        (0..N)
            .find(|task| self.scheduler.state(*task) == TaskState::Ready)
            .is_some_and(|task| self.holds_no_time(task))
    }

    /// Whether `task`'s job has nothing left that takes time. It carries on from what it last found, so a job reads its body once
    /// however long that is, and what a run costs stays in step with the steps it takes.
    fn holds_no_time(&mut self, task: usize) -> bool {
        let job = self.jobs[task];
        if job.left != 0 {
            return false;
        }
        let body = self.system.tasks[task].body;
        let from = job.next_timed.max(job.step);
        let rest = body.get(from..).unwrap_or_default();
        let found = rest
            .iter()
            .position(|step| matches!(step, Step::Run(ticks) if *ticks > 0))
            .unwrap_or(rest.len());
        self.jobs[task].next_timed = from + found;
        from + found == body.len()
    }

    /// Whatever arrives at this instant becomes pending, whether or not anything can take it.
    fn arrive(&mut self) {
        let system = self.system;
        for (place, source) in system.sources.iter().enumerate() {
            let (Some(at), Some(separation)) = (self.arrivals[place], source.separation) else {
                continue;
            };
            if at == self.now && at < system.horizon {
                self.pending[place] = 1;
                self.arrivals[place] = at.checked_add(separation.max(1));
                self.emit(Event::InterruptPending(Interrupt::Source(source.name)));
            }
        }
        // An alarm arrives where its compare is reached, each arrival a request of its own (`RA-H3`).
        for (place, source) in system.sources.iter().enumerate() {
            if source.alarm && self.compares[place].is_some_and(|at| at <= self.now) {
                self.compares[place] = None;
                self.pending[place] = self.pending[place].saturating_add(1);
                self.alarm_arrivals += 1;
                self.emit(Event::InterruptPending(Interrupt::Alarm(source.name)));
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
            if let Some(place) = self.pending.iter().position(|pending| *pending > 0) {
                self.pending[place] -= 1;
                let source = system.sources[place];
                let interrupt = if source.alarm {
                    Interrupt::Alarm(source.name)
                } else {
                    Interrupt::Source(source.name)
                };
                self.emit(Event::InterruptEntry(interrupt));
                for (task, spec) in system.tasks.iter().enumerate() {
                    if spec.trigger == Trigger::Source(place) && self.release(task, None) {
                        return true;
                    }
                }
                self.emit(Event::InterruptExit(interrupt));
            } else if self.timer_pending {
                self.timer_pending = false;
                self.emit(Event::InterruptEntry(Interrupt::Timer));
                for (task, spec) in system.tasks.iter().enumerate() {
                    if let Trigger::Timer(period) = spec.trigger {
                        // A release masked past its instant is late, and its next stays nominal, each owed one taken.
                        while self.releases[task] <= self.now {
                            let nominal = self.releases[task];
                            self.releases[task] = nominal.saturating_add(period.max(1));
                            if self.release(task, Some(nominal)) {
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

    /// `task` released, as `rt-core` decides, the events the observation record's §5 maps each answer to. True where the
    /// runtime halted.
    fn release(&mut self, task: usize, nominal: Option<u64>) -> bool {
        let name = self.system.tasks[task].name;
        // Any release of a task already faulted is discarded, and produces no event.
        let discarded = self.scheduler.state(task) == TaskState::Faulted;
        match self.scheduler.release(task) {
            Transition::Released { .. } => {
                self.jobs[task] = Job::default();
                self.emit(Event::Release {
                    task: name,
                    nominal,
                });
                false
            }
            Transition::JobSkipped { .. } => {
                self.jobs[task] = Job::default();
                self.emit(Event::Overrun {
                    task: name,
                    outcome: Outcome::SkipLateJob,
                    nominal,
                });
                false
            }
            Transition::Faulted { .. } => {
                if !discarded {
                    self.emit(Event::Overrun {
                        task: name,
                        outcome: Outcome::Fault,
                        nominal,
                    });
                }
                false
            }
            Transition::Halted { fatal } => {
                self.halted(fatal);
                true
            }
            // No service runs with the depth raised, so nothing is ever latched; a completion is no release's.
            Transition::Latched { .. }
            | Transition::OverrunLatched { .. }
            | Transition::Completed { .. } => false,
        }
    }

    /// The halt's first report, as the fatal handler keeps it: the plan names the tasks, and `rt-core` holds the rest.
    fn halted(&mut self, fatal: Fatal) {
        let tasks = self.system.tasks;
        let named = |task: Option<usize>| task.map(|task| tasks[task].name);
        let kind = match fatal.fault {
            Fault::Overrun { task } => FatalKind::Overrun {
                task: tasks[task].name,
            },
            Fault::StackGuard { task } => FatalKind::StackGuard { guard: named(task) },
            Fault::UnexpectedTrap { cause } => FatalKind::UnexpectedTrap { cause },
            Fault::InvariantViolated { invariant } => FatalKind::InvariantViolated { invariant },
        };
        self.emit(Event::Fatal {
            kind,
            attributed: named(fatal.task),
            interrupted: named(fatal.interrupted),
            escalated: fatal.escalated,
        });
    }

    /// What a refused `mask` or `unmask` raised: the decision says why, a refusal being the same for every halt.
    fn refused(&mut self) -> Advance {
        if let Decision::Halt { fatal } = self.scheduler.decide() {
            self.halted(fatal);
        }
        Advance::Halted
    }

    /// The scheduling decision, and its switch. True where the runtime halted.
    fn decide(&mut self) -> bool {
        match self.scheduler.decide() {
            Decision::Continue { .. } => false,
            Decision::Idle => {
                if !self.idle {
                    self.idle = true;
                    self.emit(Event::Idle);
                }
                false
            }
            Decision::Dispatch { to } => {
                self.enter(to);
                false
            }
            Decision::Switch { from, to } => {
                self.emit(Event::Preempt(self.system.tasks[from].name));
                self.enter(to);
                false
            }
            Decision::Halt { fatal } => {
                self.halted(fatal);
                true
            }
        }
    }

    /// The transition into `task`: its job's start, or its preempted job's resumption.
    fn enter(&mut self, task: usize) {
        self.idle = false;
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
        if self.counted_at == self.now {
            self.steps_here += 1;
            if self.steps_here > MOST_STEPS_AT_ONE_INSTANT {
                return Advance::NoProgress;
            }
        } else {
            self.counted_at = self.now;
            self.steps_here = 1;
        }
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
                if let Transition::Halted { fatal } = completed {
                    self.halted(fatal);
                    return Advance::Halted;
                }
                self.masked = false;
                self.jobs[task] = Job::default();
                self.emit(Event::Complete(name));
                Advance::Stepped
            }
            Some(Step::Mask) => {
                self.masked = true;
                let refused = self.scheduler.mask().is_err();
                self.jobs[task].step += 1;
                // Each one that runs is shown, one that raises an assertion failure among them.
                self.emit(Event::Mask(name));
                if refused {
                    return self.refused();
                }
                Advance::Stepped
            }
            Some(Step::Unmask) => {
                let refused = self.scheduler.unmask().is_err();
                self.masked = self.scheduler.is_masked();
                self.jobs[task].step += 1;
                self.emit(Event::Unmask(name));
                if refused {
                    return self.refused();
                }
                Advance::Stepped
            }
            Some(Step::Read(device)) => {
                self.jobs[task].step += 1;
                self.emit(Event::DeviceRead(self.system.devices[device].name));
                Advance::Stepped
            }
            Some(Step::Write(device)) => {
                self.jobs[task].step += 1;
                self.emit(Event::DeviceWrite(self.system.devices[device].name));
                Advance::Stepped
            }
            Some(Step::Program { alarm, ahead }) => {
                let at = self.now.saturating_add(ahead);
                let name = self.system.sources[alarm].name;
                self.jobs[task].step += 1;
                self.emit(Event::AlarmProgrammed { alarm: name, at });
                // A value already reached arrives at this write, so what follows at this instant cannot replace it, and an
                // unmasked job's next step comes after its service. Any other stands, and arrives where it is reached.
                if at <= self.now {
                    self.compares[alarm] = None;
                    self.pending[alarm] = self.pending[alarm].saturating_add(1);
                    self.alarm_arrivals += 1;
                    self.emit(Event::InterruptPending(Interrupt::Alarm(name)));
                } else {
                    self.compares[alarm] = Some(at);
                }
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
        let compares = self.compares.iter().flatten().copied();
        self.releases
            .iter()
            .copied()
            .chain(arrivals)
            .chain(compares)
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
    /// Time stood still past [`MOST_STEPS_AT_ONE_INSTANT`] steps.
    NoProgress,
}
