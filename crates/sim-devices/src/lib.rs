//! The hosted simulator's device models (`ROADMAP.md` §6.2; `docs/decisions/decision_device-models.md`): a timer and an interrupt
//! controller as exact state machines over ticks, each reading its contract as data, so that what a contract promises can be run
//! and every small case of it enumerated. F13, F14 and F15 are held in `tests/contract.rs`.
//!
//! It does no I/O and takes no clock: a caller says what instant it is. What wires these into a run is `M4.3.6`, once a plan names a
//! target's devices (`M4.1.7`): the playground's own release timer is an exact clock with no counter to wrap.

#![forbid(unsafe_code)]
#![no_std]

/// How a timer's compare raises its interrupt (§6.2, "interrupt level/edge behavior").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Compare {
    /// While the count has reached the compare value: a value already passed raises at once, by the hardware.
    Level,
    /// Only on the tick the count equals it: a value already passed raises nothing until the count comes round, so what programs
    /// it must read the count again after the write, and raise it itself.
    Edge,
}

/// A timer's contract, in its own ticks (§6.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Contract {
    /// `counter-modulus`: the count runs from zero to one below this, and wraps (`wrap-behavior` `modular`).
    pub modulus: u64,
    /// How its compare raises.
    pub compare: Compare,
}

/// Why a timer was not programmed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Refused {
    /// The deadline lies a whole wrap or more ahead, so its count is one the counter reaches sooner: the ambiguous horizon of §6.2,
    /// and of F13. It is refused, and never programmed as a nearer deadline.
    AmbiguousHorizon {
        /// How far ahead it lay.
        ahead: u64,
        /// The most that can be told apart: one below the modulus.
        horizon: u64,
    },
}

/// What programming did.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Programmed {
    /// The deadline is ahead: the interrupt is raised when it is reached.
    Armed,
    /// The deadline had passed when the write landed. The interrupt is raised now, late, and never lost: F14.
    AlreadyDue {
        /// By how many ticks it was late.
        late_by: u64,
        /// What raised it.
        raised_by: RaisedBy,
    },
}

/// What raised an interrupt whose deadline had already passed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RaisedBy {
    /// The compare's own level.
    Hardware,
    /// What programmed it, having read the count again after the write.
    Recheck,
}

/// A timer's model. A deadline is given as an absolute instant in ticks from time zero, the count being that modulo the modulus.
#[derive(Debug, Clone, Copy)]
pub struct Timer {
    contract: Contract,
    armed: Option<u64>,
    pending: bool,
}

impl Timer {
    /// A timer with nothing programmed.
    #[must_use]
    pub const fn new(contract: Contract) -> Self {
        Self {
            contract,
            armed: None,
            pending: false,
        }
    }

    /// What the counter reads at `now`.
    #[must_use]
    pub const fn read(&self, now: u64) -> u64 {
        now % self.contract.modulus
    }

    /// The longest a deadline can lie ahead and be told apart from a nearer one.
    #[must_use]
    pub const fn horizon(&self) -> u64 {
        self.contract.modulus - 1
    }

    /// `deadline` programmed, the write landing at `now`, however long after the deadline was worked out.
    ///
    /// # Errors
    ///
    /// [`Refused::AmbiguousHorizon`] for a deadline a whole wrap or more ahead, what stood before staying programmed.
    pub fn program(&mut self, now: u64, deadline: u64) -> Result<Programmed, Refused> {
        if deadline <= now {
            self.armed = None;
            self.pending = true;
            return Ok(Programmed::AlreadyDue {
                late_by: now - deadline,
                raised_by: match self.contract.compare {
                    Compare::Level => RaisedBy::Hardware,
                    Compare::Edge => RaisedBy::Recheck,
                },
            });
        }
        let ahead = deadline - now;
        if ahead > self.horizon() {
            return Err(Refused::AmbiguousHorizon {
                ahead,
                horizon: self.horizon(),
            });
        }
        self.armed = Some(deadline);
        Ok(Programmed::Armed)
    }

    /// Time moved on to `now`: what was armed is raised if it has been reached.
    pub fn advance(&mut self, now: u64) {
        if self.armed.is_some_and(|deadline| deadline <= now) {
            self.armed = None;
            self.pending = true;
        }
    }

    /// Whether its interrupt is raised.
    #[must_use]
    pub const fn pending(&self) -> bool {
        self.pending
    }

    /// Its interrupt acknowledged: lowered, and what is armed stays armed.
    pub fn acknowledge(&mut self) {
        self.pending = false;
    }
}

/// A modular count extended to a monotonic one, as the `epoch-extender` adapter does: correct while it is read at least once in every
/// wrap, which is the unambiguous horizon it owes (`docs/specs/adapters/decision_engine-adapters.md` §2).
#[derive(Debug, Clone, Copy)]
pub struct Epoch {
    modulus: u64,
    last: u64,
    wraps: u64,
}

impl Epoch {
    /// An epoch at time zero.
    #[must_use]
    pub const fn new(modulus: u64) -> Self {
        Self {
            modulus,
            last: 0,
            wraps: 0,
        }
    }

    /// The monotonic count for `count`, read now.
    pub const fn observe(&mut self, count: u64) -> u64 {
        if count < self.last {
            self.wraps += 1;
        }
        self.last = count;
        self.wraps * self.modulus + count
    }
}

/// How a source's request reaches the controller (§6.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Line {
    /// Asserted until the device lowers it: served again if still asserted when its service is acknowledged.
    Level,
    /// One request per edge: taken once, and a request while it is served is one more.
    Edge,
}

/// An interrupt controller's model over `N` sources, the lower place taken first when both are pending, as the plan's route ranks
/// configure it. One interrupt is served at a time, none nested, and none while masked: F15.
#[derive(Debug, Clone, Copy)]
pub struct Controller<const N: usize> {
    lines: [Line; N],
    enabled: [bool; N],
    asserted: [bool; N],
    pending: [bool; N],
    serving: Option<usize>,
    masked: bool,
}

impl<const N: usize> Controller<N> {
    /// A controller with every source enabled and nothing pending.
    #[must_use]
    pub const fn new(lines: [Line; N]) -> Self {
        Self {
            lines,
            enabled: [true; N],
            asserted: [false; N],
            pending: [false; N],
            serving: None,
            masked: false,
        }
    }

    /// Only these sources enabled. A disabled source's request stays pending, and is taken once it is enabled.
    pub const fn enable(&mut self, enabled: [bool; N]) {
        self.enabled = enabled;
    }

    /// A source's request raised, whether or not anything can take it now.
    pub const fn raise(&mut self, source: usize) {
        self.asserted[source] = true;
        self.pending[source] = true;
    }

    /// A level source's line lowered by its device. A request not yet taken goes with it: nothing is asserting it.
    pub const fn lower(&mut self, source: usize) {
        self.asserted[source] = false;
        if matches!(self.lines[source], Line::Level) {
            self.pending[source] = false;
        }
    }

    /// The hardware masked or unmasked.
    pub const fn set_masked(&mut self, masked: bool) {
        self.masked = masked;
    }

    /// The interrupt taken next, if one may be: none while masked, or while one is being served.
    pub fn take(&mut self) -> Option<usize> {
        if self.masked || self.serving.is_some() {
            return None;
        }
        let source = (0..N).find(|source| self.pending[*source] && self.enabled[*source])?;
        self.pending[source] = false;
        if matches!(self.lines[source], Line::Edge) {
            self.asserted[source] = false;
        }
        self.serving = Some(source);
        Some(source)
    }

    /// The service of what is being served ended. A level line still asserted is pending again.
    pub const fn acknowledge(&mut self) {
        if let Some(source) = self.serving {
            if matches!(self.lines[source], Line::Level) && self.asserted[source] {
                self.pending[source] = true;
            }
            self.serving = None;
        }
    }

    /// What is being served.
    #[must_use]
    pub const fn serving(&self) -> Option<usize> {
        self.serving
    }
}
