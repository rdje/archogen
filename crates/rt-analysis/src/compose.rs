//! The composition of the runtime variant's four composite inputs, `C_i`, `CS_i`, `J_i^release` and `J_s`, from
//! their parts (leaf `M2.10.2`; `docs/specs/catalog/decision_runtime-composite-inputs.md`, "the record" below).
//!
//! ⛔ **This module implements that record and nothing else.** Each part has one owner — the catalog, the
//! application, the plan or the description (record §1) — and the engine adds the parts up as §3 says, under the
//! facts §2 states, with the verdict §6 gives every way the composition can stop. The variant ([`crate::runtime`])
//! receives the composed values and never the parts (§6): [`compose`] builds the [`RuntimeTask`]s and [`Source`]s
//! that [`runtime::admit`] takes, and [`Composition::admit`] carries each composite's statement — its parts, their
//! owners and their evidence categories — into the admitted set, so the conclusion names them (§6).
//!
//! The parts arrive as values. The catalog's own lookups, `(facet, name)` against the records a selection names,
//! are the catalog path's (`M2.7.5`), which maps what it reads onto [`Kernel`] and [`SourceParts`]; this crate does
//! not depend on the catalog crate, so the arithmetic is tested against hand-derived figures alone.
//!
//! Exact integers in the variant's unit, every sum and product checked (§3). A composite that cannot be formed has
//! no value: the task or source reaches the variant with that input undeclared, and [`Composition::admit`] gives the
//! set the highest-precedence verdict among the composition's stops and the variant's own conditions (§6), so no
//! task of such a set is ever reported as holding.

use core::fmt;

use crate::runtime::{
    self, ceil_div, gcd, Acknowledge, Deferred, Evidence, Platform, Refusal, RefusalVerdict,
    ReleasedBy, RuntimeSet, RuntimeTask, Source, TaskFacts, Value, ITERATION_BUDGET, NOT_DECLARED,
};

/// The primitive a masked run begins with (record §1: "a stretch from a `mask` call").
pub const MASK: &str = "mask";
/// The primitive that ends a masked run when completion does not (record §1).
pub const UNMASK: &str = "unmask";
/// The completion path's name among the catalog's costs. A primitive so named cannot be composed (record §6).
pub const COMPLETION: &str = "completion";

/// Who supplies a part (record, "The fact / decision"; §6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Owner {
    /// Kernel code and the platform, as reviewed timing costs (record §1).
    Catalog,
    /// The task's own code and its calls (record §1).
    Application,
    /// The order among sources (record §1).
    Plan,
    /// A figure the caller supplies: `C_s` of a service that runs application code (record §6, §7).
    Caller,
    /// A separation from the description: a requirement or an arrival assumption, not evidence (record §6).
    Description,
    /// Another composite of the same composition, such as `L` or a `J` queued ahead.
    Composed,
}

impl Owner {
    /// The words a statement uses for the owner.
    #[must_use]
    pub const fn words(self) -> &'static str {
        match self {
            Self::Catalog => "the catalog",
            Self::Application => "the application",
            Self::Plan => "the plan",
            Self::Caller => "the caller's figure",
            Self::Description => "the description",
            Self::Composed => "composed here",
        }
    }
}

/// One part of a composite, as the conclusion names it (record §6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    /// The symbol, as the record writes it: `api.mask`, `C^app`, `L`, or a service queued ahead.
    pub symbol: String,
    /// Who supplies it.
    pub owner: Owner,
    /// How many times it is charged: `n_{i,p}` for a primitive, `2` for `δ` in a `B_x`, the services counted at
    /// the fixed point for a queued interrupt, else `1`.
    pub times: u128,
    /// Its value, once.
    pub value: u128,
    /// Its §7.3 category; `None` for a requirement, which carries none and lowers none (record §6).
    pub evidence: Option<Evidence>,
}

impl Part {
    fn render(&self) -> String {
        let times = if self.times == 1 {
            String::new()
        } else {
            format!("{} × ", self.times)
        };
        match self.evidence {
            Some(evidence) => format!(
                "{times}{} = {} ({}: {})",
                self.symbol,
                self.value,
                self.owner.words(),
                evidence.words()
            ),
            None => format!(
                "{times}{} = {} ({})",
                self.symbol,
                self.value,
                self.owner.words()
            ),
        }
    }
}

/// One alternative of a maximum, or the one term of a sum or a fixed point.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Term {
    /// What the alternative is: `run 0, ending at unmask`, `masked.spin`; empty for a sum's single term.
    pub label: String,
    /// The term's value.
    pub value: u128,
    /// What it adds up.
    pub parts: Vec<Part>,
}

/// The shape of a composite's rule (record §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    /// A sum of its parts: `C_i`.
    Sum,
    /// The largest of its alternatives: `CS_i`, `L`.
    Maximum,
    /// The least fixed point of `B_x` plus what queues ahead: `J^release`, `J_s`.
    FixedPoint,
}

/// One composed input with what it is made of (record §6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Composite {
    /// Which input: `` C of task `a` ``, `` CS of task `a` ``, `` J^release of task `a` ``, `` J_s of source `uart` ``,
    /// `L`, or `Δ_timer`.
    pub what: String,
    /// The value, its category the weakest of its parts' (record §6).
    pub value: Value,
    /// How the parts combine.
    pub rule: Rule,
    /// The parts, by alternative.
    pub terms: Vec<Term>,
    /// For a fixed point: `Δ^(0), Δ^(1), …`, the last two equal (record §3). Empty otherwise.
    pub iterates: Vec<u128>,
}

impl Composite {
    /// The statement the conclusion carries: the value with its category, then every part with its owner and
    /// category, and for a fixed point its iterates (record §6).
    #[must_use]
    pub fn render(&self) -> String {
        let head = format!(
            "{} = {} ({}, the weakest of its parts)",
            self.what,
            self.value.value,
            self.value.evidence.words()
        );
        let parts = |term: &Term| {
            term.parts
                .iter()
                .map(Part::render)
                .collect::<Vec<_>>()
                .join(" + ")
        };
        match self.rule {
            Rule::Sum => format!(
                "{head}: {}",
                self.terms.first().map_or_else(String::new, parts)
            ),
            Rule::Maximum => format!(
                "{head}: the largest of {}",
                self.terms
                    .iter()
                    .map(|term| format!("{} = {} [{}]", term.label, term.value, parts(term)))
                    .collect::<Vec<_>>()
                    .join("; ")
            ),
            Rule::FixedPoint => format!(
                "{head}: the least fixed point of {}; Δ {}",
                self.terms.first().map_or_else(String::new, parts),
                self.iterates
                    .iter()
                    .map(u128::to_string)
                    .collect::<Vec<_>>()
                    .join(" → ")
            ),
        }
    }
}

/// A primitive's two costs from the runtime API record (record §1, §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Primitive {
    /// Its name, as the application's call counts name it.
    pub name: String,
    /// `api.<p>`: one call, first instruction to last.
    pub call: Option<Value>,
    /// `masked.<p>`: the longest contiguous masked stretch inside one call made with interrupts unmasked.
    pub masked: Option<Value>,
}

/// The behavioral facts the composition reads from the catalog (record §2), each a declared yes or no; `None` is
/// a fact that cannot be read.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct KernelFacts {
    /// `reprograms-only-in-service`, in the `timer-service` group.
    pub reprograms_only_in_service: Option<bool>,
    /// `pending-taken-after-unmask`, in the `switch` group.
    pub pending_taken_after_unmask: Option<bool>,
    /// `releases-never-latched`, in the `completion` group.
    pub releases_never_latched: Option<bool>,
    /// `primitives-out-of-line`, in the `completion` group.
    pub primitives_out_of_line: Option<bool>,
    /// `external-before-timer`, a hardware fact: `yes` puts every source ahead of the timer, `no` the timer ahead
    /// of every source, admitted only with `one-claim-per-trap`.
    pub external_before_timer: Option<bool>,
    /// `one-claim-per-trap`, in the `switch` group; read only when `external-before-timer` is `no`.
    pub one_claim_per_trap: Option<bool>,
    /// `no-empty-claim`, in the `switch` group.
    pub no_empty_claim: Option<bool>,
    /// `starts-by-transition`, in the `switch` group.
    pub starts_by_transition: Option<bool>,
    /// `one-external-controller`, a hardware fact about the target.
    pub one_external_controller: Option<bool>,
}

/// What the catalog supplies to the composition beyond the variant's [`Platform`] (record §1 and §5).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Kernel {
    /// Each primitive a task can call, with its two costs.
    pub primitives: Vec<Primitive>,
    /// `completion`: the completion path, from the job's last instruction of its own code to the decided switch.
    pub completion: Option<Value>,
    /// `masked.completion`: the completion path's longest contiguous masked stretch.
    pub masked_completion: Option<Value>,
    /// The facts of §2 the catalog states.
    pub facts: KernelFacts,
    /// `runtime-discipline.<id>`, one per record of the selection, by record id (record §2).
    pub discipline: Vec<(String, Option<bool>)>,
}

/// `n_{i,p}` or `n_{r,e,p}`: the most calls one job, or one run, makes to a primitive (record §1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Calls {
    /// The primitive's name.
    pub primitive: String,
    /// The most calls.
    pub count: u64,
}

/// How a masked run ends (record §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Ending {
    /// At an `unmask` call.
    Unmask,
    /// At the job's completion, with no `unmask`.
    Completion,
}

/// One masked run of a task with one of its endings (record §1). A run that can end both ways is two of these.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Run {
    /// `CS_{i,r,e}^app`: the most of the task's own code inside the run, for this ending.
    pub own_code: Option<Value>,
    /// How it ends.
    pub ending: Ending,
    /// `n_{r,e,p}`: the most calls to each primitive inside the run, for this ending.
    pub calls: Vec<Calls>,
}

/// A task's parts: the description's figures, and the application's (record §1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TaskParts {
    /// Stable identity.
    pub id: String,
    /// The rank, ascending highest-first.
    pub priority: i64,
    /// `T_i`.
    pub separation: u64,
    /// `D_i`.
    pub deadline: u64,
    /// `J_i^event`.
    pub jitter_event: u64,
    /// What releases it.
    pub released_by: ReleasedBy,
    /// Its declared behaviour.
    pub facts: TaskFacts,
    /// `C_i^app`: the most one job executes of the task's own code, every primitive and service excluded.
    pub own_code: Option<Value>,
    /// `n_{i,p}` for each primitive the task calls. Two entries for one primitive are added, which over-counts.
    pub calls: Vec<Calls>,
    /// Every masked run, with each way it can end.
    pub runs: Vec<Run>,
}

/// The application's parts and declarations (record §1 and §2).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Application {
    /// Each task's parts.
    pub tasks: Vec<TaskParts>,
    /// `leaves-interrupt-hardware-alone`, the application's fact covering its tasks and its initialisation.
    pub leaves_interrupt_hardware_alone: Option<bool>,
    /// `releases-after-initialisation`, the caller's declaration until `M4.10` orders initialisation in the plan.
    pub releases_after_initialisation: Option<bool>,
}

/// Who supplies `C_s` (record §6, §7), decided by the record that anchors `service.<source>`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Service {
    /// The catalog's `service.<source>`, the record stating `no-application-code.<source>` `yes`.
    Catalog(Option<Value>),
    /// The caller's figure, the record stating `no`: it begins at the trap, and the caller declares
    /// `leaves-interrupt-hardware-alone` for the service's application code.
    Caller {
        /// The figure.
        cost: Option<Value>,
        /// The caller's declaration for the service's application code.
        leaves_interrupt_hardware_alone: Option<bool>,
    },
    /// No record anchors `service.<source>`, so none of the source's facts can be read.
    Unanchored,
}

/// A source's parts: the description's, the plan's and the catalog's (record §1, §2, §6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceParts {
    /// Its identity, as the build's enabled set names it.
    pub id: String,
    /// `T_s`: an arrival assumption about the environment.
    pub separation: Option<Value>,
    /// The acknowledge point.
    pub acknowledge: Option<Acknowledge>,
    /// Where its deferred work runs.
    pub deferred: Option<Deferred>,
    /// The plan's rank among the sources, ascending highest-first like a task's: the interrupt controller's
    /// priority for the hart's machine-mode context, with the plan's own rule for ties already applied.
    pub priority: Option<i64>,
    /// The plan's statement that the source is deliverable, its priority above the controller's threshold.
    pub deliverable: Option<bool>,
    /// `C_s`, and who supplies it.
    pub service: Service,
    /// `external.<source>`.
    pub external: Option<bool>,
    /// `one-request-per-arrival.<source>`.
    pub one_request_per_arrival: Option<bool>,
}

/// The stops of record §3 and the conditions of §2 and §6 that were not met, each under the verdict §6 gives it.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Stops {
    /// `unsupported-profile`: a fact declared `no`, a source stopped or past its no-loss limit, a plan whose order
    /// is not strict or that leaves a source undeliverable.
    pub unsupported: Vec<String>,
    /// `not-established`: the timer stopped, or an overflow in `C_i`, `CS_i`, `L` or a `B_x`.
    pub not_established: Vec<String>,
    /// `analysis-inconclusive`: a part or a fact that cannot be read, or a named resource limit.
    pub unresolved: Vec<String>,
}

/// The verdict a stop falls under (record §6), in the variant's §5 order of precedence.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Which {
    Unsupported,
    NotEstablished,
    Unresolved,
}

impl Stops {
    /// Record a stop under its verdict; its index in that verdict's list, for what is named under it.
    fn push(&mut self, which: Which, message: String) -> usize {
        let list = match which {
            Which::Unsupported => &mut self.unsupported,
            Which::NotEstablished => &mut self.not_established,
            Which::Unresolved => &mut self.unresolved,
        };
        list.push(message);
        list.len() - 1
    }
}

/// Why no task is reported as holding (record §6): a refusal, or the variant's own `not-established`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NoBound {
    /// `unsupported-profile` or `analysis-inconclusive`, with every reason of that verdict.
    Refused(Refusal),
    /// The variant's `not-established`: no single-job bound applies.
    NotEstablished {
        /// Every stop with that verdict.
        why: String,
    },
}

impl fmt::Display for NoBound {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Refused(refusal) => refusal.fmt(f),
            Self::NotEstablished { why } => write!(f, "not-established: {why}"),
        }
    }
}

/// What a composition establishes: the variant's inputs, every composite's statement, the assumptions the
/// conclusion names beside them, and every stop (record §6). Built by [`compose`]; [`Self::admit`] takes it to the
/// variant.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Composition {
    /// The tasks, their `C_i`, `CS_i` and `J_i^release` composed where they could be, highest priority first.
    pub tasks: Vec<RuntimeTask>,
    /// The sources, their `J_s` composed where it could be.
    pub sources: Vec<Source>,
    /// Every composite that has a value, with its parts.
    pub composites: Vec<Composite>,
    /// The arrival assumptions, the run's assumption, and every figure or declaration of the caller's (record §2,
    /// §6).
    pub assumptions: Vec<String>,
    /// Every stop, by verdict.
    pub stops: Stops,
    /// The composites left without a value, each as the variant names the input.
    without_value: Vec<String>,
}

impl Composition {
    /// Admit the composed set, or say why no task of it is reported as holding (record §6): the verdict of highest
    /// precedence among the composition's stops and the variant's own conditions, `unsupported-profile`, then
    /// `not-established`, then `analysis-inconclusive` (the variant's §5), with every reason of that verdict. An
    /// admitted set carries each composite's statement and the composition's assumptions.
    ///
    /// # Errors
    ///
    /// A [`NoBound`].
    pub fn admit(&self, platform: &Platform) -> Result<RuntimeSet, NoBound> {
        let admitted = runtime::admit(&self.tasks, &self.sources, platform);
        let mut unsupported = self.stops.unsupported.clone();
        let mut unresolved = self.stops.unresolved.clone();
        match &admitted {
            Err(refusal) if refusal.verdict == RefusalVerdict::UnsupportedProfile => {
                unsupported.extend(refusal.reasons.iter().cloned());
            }
            Err(refusal) => {
                // An input the composition left without a value is reported by the stop that left it so.
                unresolved.extend(
                    refusal
                        .reasons
                        .iter()
                        .filter(|reason| {
                            !self
                                .without_value
                                .iter()
                                .any(|what| **reason == format!("{what}{NOT_DECLARED}"))
                        })
                        .cloned(),
                );
            }
            Ok(_) => {}
        }
        if !unsupported.is_empty() {
            return Err(NoBound::Refused(Refusal {
                verdict: RefusalVerdict::UnsupportedProfile,
                reasons: unsupported,
            }));
        }
        if !self.stops.not_established.is_empty() {
            return Err(NoBound::NotEstablished {
                why: self.stops.not_established.join("; "),
            });
        }
        if !unresolved.is_empty() {
            return Err(NoBound::Refused(Refusal {
                verdict: RefusalVerdict::AnalysisInconclusive,
                reasons: unresolved,
            }));
        }
        let mut set = admitted.map_err(NoBound::Refused)?;
        set.carry(
            self.composites
                .iter()
                .map(Composite::render)
                .chain(self.assumptions.iter().cloned()),
        );
        Ok(set)
    }
}

/// The strength of a category, strongest first (record §6: analytically established, externally supplied,
/// observed maximum, assumed).
const fn strength(evidence: Evidence) -> u8 {
    match evidence {
        Evidence::Analytical => 3,
        Evidence::ExternallySupplied => 2,
        Evidence::ObservedMaximum => 1,
        Evidence::Assumed => 0,
    }
}

const fn weaker(a: Evidence, b: Evidence) -> Evidence {
    if strength(b) < strength(a) {
        b
    } else {
        a
    }
}

/// A sum under construction: its parts, its checked total, and the weakest category so far.
struct Sum {
    parts: Vec<Part>,
    total: Option<u128>,
    complete: bool,
    weakest: Evidence,
}

impl Sum {
    fn new() -> Self {
        Self {
            parts: Vec::new(),
            total: Some(0),
            complete: true,
            weakest: Evidence::Analytical,
        }
    }

    /// Add `times` × a part; a part without a value leaves the sum incomplete.
    fn add(&mut self, value: Option<Value>, symbol: &str, owner: Owner, times: u128) {
        let Some(value) = value else {
            self.complete = false;
            return;
        };
        self.weakest = weaker(self.weakest, value.evidence);
        self.parts.push(Part {
            symbol: symbol.to_owned(),
            owner,
            times,
            value: u128::from(value.value),
            evidence: Some(value.evidence),
        });
        self.total = self.total.and_then(|total| {
            u128::from(value.value)
                .checked_mul(times)
                .and_then(|product| total.checked_add(product))
        });
    }
}

/// What a sum or a maximum came to.
enum Formed {
    /// A part is missing, so the composite has no value and the part's own finding says why.
    Incomplete,
    /// Checked arithmetic failed, or the value does not fit the variant's `u64`.
    Overflow,
    /// The value, its category, and its terms.
    Value(u128, Evidence, Vec<Term>),
}

fn finish_sum(sum: Sum) -> Formed {
    if !sum.complete {
        return Formed::Incomplete;
    }
    match sum.total {
        Some(total) if u64::try_from(total).is_ok() => Formed::Value(
            total,
            sum.weakest,
            vec![Term {
                label: String::new(),
                value: total,
                parts: sum.parts,
            }],
        ),
        _ => Formed::Overflow,
    }
}

/// A maximum under construction: each alternative a sum.
struct Maximum {
    terms: Vec<Term>,
    best: Option<u128>,
    complete: bool,
    overflow: bool,
    weakest: Evidence,
}

impl Maximum {
    fn new() -> Self {
        Self {
            terms: Vec::new(),
            best: None,
            complete: true,
            overflow: false,
            weakest: Evidence::Analytical,
        }
    }

    fn alternative(&mut self, label: &str, sum: Sum) {
        match finish_sum(sum) {
            Formed::Incomplete => self.complete = false,
            Formed::Overflow => self.overflow = true,
            Formed::Value(value, evidence, mut terms) => {
                self.weakest = weaker(self.weakest, evidence);
                self.best = Some(self.best.map_or(value, |best| best.max(value)));
                let mut term = terms.remove(0);
                term.label = label.to_owned();
                self.terms.push(term);
            }
        }
    }

    fn finish(self) -> Formed {
        if !self.complete {
            Formed::Incomplete
        } else if self.overflow {
            Formed::Overflow
        } else {
            match self.best {
                Some(best) => Formed::Value(best, self.weakest, self.terms),
                None => Formed::Value(0, self.weakest, self.terms),
            }
        }
    }
}

/// One interrupt whose `J` is composed, as what queues behind it sees it (record §3).
#[derive(Clone)]
struct Ahead {
    /// `δ + C_q + S`.
    charge: u128,
    /// The ceilings that count its services: `(J, T)`, one for a source, one per timer-released task for the
    /// timer.
    ceilings: Vec<(u128, u128)>,
    /// Its part in a statement, `times` filled at the fixed point.
    part: Part,
}

/// Why an iteration stopped short of a fixed point (record §3).
enum Stop {
    /// The pre-check: what queues ahead can arrive at least as fast as it is served.
    NoFixedPoint,
    /// An iterate passed the refusal bound.
    PastBound,
    /// Checked arithmetic failed, or the value does not fit `u64`.
    Overflow,
    /// The step budget.
    Budget,
    /// The pre-check's exact sum does not fit 128 bits.
    Precheck,
}

/// `Σ_q charge_q · Σ_{(J, T)} 1/T ≥ 1`, compared exactly in reduced rationals (record §3); `None` when the sum
/// does not fit 128-bit numerator and denominator.
fn no_fixed_point(ahead: &[Ahead]) -> Option<bool> {
    let (mut num, mut den) = (0u128, 1u128);
    for q in ahead {
        for (_, t) in &q.ceilings {
            let g = gcd(den, *t);
            let lcm = den.checked_div(g)?.checked_mul(*t)?;
            num = num
                .checked_mul(lcm / den)?
                .checked_add(q.charge.checked_mul(lcm / t)?)?;
            den = lcm;
            let r = gcd(num, den).max(1);
            num /= r;
            den /= r;
        }
    }
    Some(num >= den)
}

/// A fixed point reached: the value, the iterates, and each queued interrupt's count of services at it.
type Reached = (u128, Vec<u128>, Vec<u128>);

/// The least fixed point of `Δ = base + Σ_q n_q(Δ) · charge_q` from `base`, with `n_q(Δ) = Σ ⌈(Δ + J)/T⌉` over
/// the interrupt's ceilings (record §3). At each iterate the fixed-point test comes first, then the refusal bound
/// `passes`. A stop carries the iterates so far.
fn least_fixed_point(
    base: u128,
    ahead: &[Ahead],
    passes: &dyn Fn(u128) -> bool,
    budget: usize,
) -> Result<Reached, (Stop, Vec<u128>)> {
    let mut iterates = vec![base];
    match no_fixed_point(ahead) {
        None => return Err((Stop::Precheck, iterates)),
        Some(true) => return Err((Stop::NoFixedPoint, iterates)),
        Some(false) => {}
    }
    let step = |delta: u128| -> Option<(u128, Vec<u128>)> {
        let mut next = base;
        let mut counts = Vec::with_capacity(ahead.len());
        for q in ahead {
            let mut n: u128 = 0;
            for (j, t) in &q.ceilings {
                n = n.checked_add(ceil_div(delta.checked_add(*j)?, *t))?;
            }
            next = next.checked_add(n.checked_mul(q.charge)?)?;
            counts.push(n);
        }
        Some((next, counts))
    };
    let mut delta = base;
    for _ in 0..budget {
        let Some((next, counts)) = step(delta) else {
            return Err((Stop::Overflow, iterates));
        };
        iterates.push(next);
        if next == delta {
            if u64::try_from(delta).is_err() {
                return Err((Stop::Overflow, iterates));
            }
            return Ok((delta, iterates, counts));
        }
        if passes(next) {
            return Err((Stop::PastBound, iterates));
        }
        delta = next;
    }
    Err((Stop::Budget, iterates))
}

/// What the composition knows of one source once its parts are read.
struct SourceRead<'a> {
    parts: &'a SourceParts,
    /// `C_s`, with who supplies it.
    service: Option<(Value, Owner)>,
    /// `T_s`.
    separation: Option<u128>,
}

/// An entry in the order the interrupts are taken, first taken first.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Queued {
    Source(usize),
    Timer,
}

/// The composition in progress.
struct Composer<'a> {
    kernel: &'a Kernel,
    out: Composition,
}

impl Composer<'_> {
    /// A fact of §2: unreadable leaves what it gates undeclared (§6), `no` is outside the composition (§6).
    /// Whether it holds.
    fn fact(&mut self, fact: Option<bool>, name: &str, gates: &str) -> bool {
        match fact {
            None => {
                self.out.stops.unresolved.push(format!(
                    "`{name}` cannot be read, which leaves {gates} undeclared (record §2)"
                ));
                false
            }
            Some(false) => {
                self.out.stops.unsupported.push(format!(
                    "`{name}` is declared `no`, outside what the composition covers (record §6)"
                ));
                false
            }
            Some(true) => true,
        }
    }

    /// A part: unreadable leaves what needs it undeclared (§6).
    fn part(&mut self, value: Option<Value>, what: &str, needs: &str) -> Option<Value> {
        if value.is_none() {
            self.out.stops.unresolved.push(format!(
                "{what} cannot be read, which leaves {needs} undeclared (record §6)"
            ));
        }
        value
    }

    /// `api.<p>` or `masked.<p>` (record §5); a primitive the catalog does not cost, or one named `completion`,
    /// leaves `needs` undeclared (record §6).
    fn primitive(&mut self, name: &str, masked: bool, needs: &str) -> Option<Value> {
        if name == COMPLETION {
            self.out.stops.unresolved.push(format!(
                "a primitive named `{COMPLETION}` cannot be composed, since `masked.{COMPLETION}` names the \
                 completion path, which leaves {needs} undeclared (record §6)"
            ));
            return None;
        }
        let kind = if masked { "masked" } else { "api" };
        let cost = self
            .kernel
            .primitives
            .iter()
            .find(|primitive| primitive.name == name)
            .and_then(|primitive| {
                if masked {
                    primitive.masked
                } else {
                    primitive.call
                }
            });
        if cost.is_none() {
            self.out.stops.unresolved.push(format!(
                "the catalog does not cost primitive `{name}` (`{kind}.{name}`), which leaves {needs} undeclared \
                 (record §6)"
            ));
        }
        cost
    }

    fn composite(
        &mut self,
        what: &str,
        value: u128,
        evidence: Evidence,
        rule: Rule,
        terms: Vec<Term>,
    ) -> Value {
        let value = Value {
            value: u64::try_from(value).unwrap_or(u64::MAX),
            evidence,
        };
        self.out.composites.push(Composite {
            what: what.to_owned(),
            value,
            rule,
            terms,
            iterates: Vec::new(),
        });
        value
    }

    fn without_value(&mut self, what: &str) {
        self.out.without_value.push(what.to_owned());
    }

    /// `C_i = C_i^app + Σ_p n_{i,p} · api.p + completion` (record §3). An overflow is the variant's busy-period
    /// stop, `not-established` (record §6).
    fn computation(&mut self, task: &TaskParts) -> Option<Value> {
        let what = format!("C of task `{}`", task.id);
        let mut sum = Sum::new();
        let own = self.part(
            task.own_code,
            &format!("C^app of task `{}`", task.id),
            &what,
        );
        sum.add(own, "C^app", Owner::Application, 1);
        for call in &task.calls {
            if call.count == 0 {
                continue;
            }
            let cost = self.primitive(&call.primitive, false, &what);
            sum.add(
                cost,
                &format!("api.{}", call.primitive),
                Owner::Catalog,
                u128::from(call.count),
            );
        }
        let completion = self.part(self.kernel.completion, COMPLETION, &what);
        sum.add(completion, COMPLETION, Owner::Catalog, 1);
        match finish_sum(sum) {
            Formed::Value(value, evidence, terms) => {
                Some(self.composite(&what, value, evidence, Rule::Sum, terms))
            }
            Formed::Incomplete => {
                self.without_value(&what);
                None
            }
            Formed::Overflow => {
                self.out.stops.not_established.push(format!(
                    "{what} overflows: the job's own cost is past what any period bounds, the variant's \
                     busy-period stop (record §6)"
                ));
                self.without_value(&what);
                None
            }
        }
    }

    /// `CS_i`: the largest of each run's sum, each called primitive's `masked.p`, and `masked.completion`
    /// (record §3). An overflow stops every interrupt (record §6), which the caller records.
    fn masked_section(&mut self, task: &TaskParts) -> (Option<Value>, bool) {
        let what = format!("CS of task `{}`", task.id);
        let mut maximum = Maximum::new();
        for (index, run) in task.runs.iter().enumerate() {
            let mut sum = Sum::new();
            let own = self.part(
                run.own_code,
                &format!("CS^app of run {index} of task `{}`", task.id),
                &what,
            );
            sum.add(own, "CS^app", Owner::Application, 1);
            let mask = self.primitive(MASK, false, &what);
            sum.add(mask, &format!("api.{MASK}"), Owner::Catalog, 1);
            for call in &run.calls {
                if call.count == 0 {
                    continue;
                }
                let cost = self.primitive(&call.primitive, false, &what);
                sum.add(
                    cost,
                    &format!("api.{}", call.primitive),
                    Owner::Catalog,
                    u128::from(call.count),
                );
            }
            let ending = match run.ending {
                Ending::Unmask => {
                    let unmask = self.primitive(UNMASK, false, &what);
                    sum.add(unmask, &format!("api.{UNMASK}"), Owner::Catalog, 1);
                    UNMASK
                }
                Ending::Completion => {
                    let completion = self.part(self.kernel.completion, COMPLETION, &what);
                    sum.add(completion, COMPLETION, Owner::Catalog, 1);
                    COMPLETION
                }
            };
            maximum.alternative(&format!("run {index}, ending at {ending}"), sum);
        }
        for call in &task.calls {
            if call.count == 0 {
                continue;
            }
            let mut sum = Sum::new();
            let cost = self.primitive(&call.primitive, true, &what);
            sum.add(
                cost,
                &format!("masked.{}", call.primitive),
                Owner::Catalog,
                1,
            );
            maximum.alternative(&format!("masked.{}", call.primitive), sum);
        }
        let mut sum = Sum::new();
        let masked_completion = self.part(
            self.kernel.masked_completion,
            &format!("masked.{COMPLETION}"),
            &what,
        );
        sum.add(
            masked_completion,
            &format!("masked.{COMPLETION}"),
            Owner::Catalog,
            1,
        );
        maximum.alternative(&format!("masked.{COMPLETION}"), sum);
        match maximum.finish() {
            Formed::Value(value, evidence, terms) => (
                Some(self.composite(&what, value, evidence, Rule::Maximum, terms)),
                false,
            ),
            Formed::Incomplete => {
                self.without_value(&what);
                (None, false)
            }
            Formed::Overflow => {
                self.without_value(&what);
                (None, true)
            }
        }
    }
}

/// Compose the four inputs from their parts (record §3), within the variant's iteration budget.
#[must_use]
pub fn compose(
    kernel: &Kernel,
    platform: &Platform,
    application: &Application,
    sources: &[SourceParts],
) -> Composition {
    compose_within(kernel, platform, application, sources, ITERATION_BUDGET)
}

/// [`compose`] with the step budget of each fixed point given: `budget` steps, each of which either reaches the
/// fixed point or grows by at least one unit, before the iteration stops as a named resource limit (record §3).
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn compose_within(
    kernel: &Kernel,
    platform: &Platform,
    application: &Application,
    sources: &[SourceParts],
    budget: usize,
) -> Composition {
    let mut c = Composer {
        kernel,
        out: Composition::default(),
    };

    // §2: the facts that gate every composite.
    let mut every_composite = true;
    for (fact, name) in [
        (
            kernel.facts.pending_taken_after_unmask,
            "pending-taken-after-unmask",
        ),
        (
            kernel.facts.releases_never_latched,
            "releases-never-latched",
        ),
        (
            kernel.facts.primitives_out_of_line,
            "primitives-out-of-line",
        ),
        (kernel.facts.starts_by_transition, "starts-by-transition"),
    ] {
        every_composite &= c.fact(fact, name, "every composite");
    }
    for (id, fact) in &kernel.discipline {
        every_composite &= c.fact(
            *fact,
            &format!("runtime-discipline.{id}"),
            "every composite",
        );
    }
    every_composite &= c.fact(
        application.leaves_interrupt_hardware_alone,
        "leaves-interrupt-hardware-alone",
        "every composite",
    );

    // The description's tasks, highest priority first, with `C_i` and `CS_i` (§3).
    let mut ordered: Vec<&TaskParts> = application.tasks.iter().collect();
    ordered.sort_by_key(|task| task.priority);
    let mut overflow_stops_every_interrupt: Option<String> = None;
    let mut masked_sections: Vec<Option<Value>> = Vec::with_capacity(ordered.len());
    for task in &ordered {
        let (computation, masked) = if every_composite {
            let computation = c.computation(task);
            let (masked, overflowed) = c.masked_section(task);
            if overflowed && overflow_stops_every_interrupt.is_none() {
                overflow_stops_every_interrupt =
                    Some(format!("CS of task `{}` overflows", task.id));
            }
            (computation, masked)
        } else {
            c.without_value(&format!("C of task `{}`", task.id));
            c.without_value(&format!("CS of task `{}`", task.id));
            (None, None)
        };
        masked_sections.push(masked);
        c.out.tasks.push(RuntimeTask {
            id: task.id.clone(),
            priority: task.priority,
            computation,
            separation: task.separation,
            deadline: task.deadline,
            jitter_event: task.jitter_event,
            jitter_release: None,
            masked_section: masked,
            released_by: task.released_by.clone(),
            facts: task.facts,
        });
    }

    // The sources' parts (§1, §6).
    let every_j = "every J";
    let mut read: Vec<SourceRead<'_>> = Vec::with_capacity(sources.len());
    let mut every_j_ok = every_composite;
    for source in sources {
        let name = format!("source `{}`", source.id);
        let separation = c.part(source.separation, &format!("T_s of {name}"), every_j);
        if let Some(separation) = separation {
            c.out.assumptions.push(format!(
                "T_s of {name} = {}: an arrival assumption about the environment, {} (record §6)",
                separation.value,
                separation.evidence.words()
            ));
        }
        let service = match &source.service {
            Service::Catalog(cost) => c
                .part(*cost, &format!("C_s of {name}"), every_j)
                .map(|cost| (cost, Owner::Catalog)),
            Service::Caller {
                cost,
                leaves_interrupt_hardware_alone,
            } => {
                let cost = c.part(
                    *cost,
                    &format!(
                        "C_s of {name}, the caller's figure for a service whose record states \
                         `no-application-code.{}` `no`",
                        source.id
                    ),
                    every_j,
                );
                every_j_ok &= c.fact(
                    *leaves_interrupt_hardware_alone,
                    &format!(
                        "leaves-interrupt-hardware-alone, the caller's declaration for the application code of \
                         {name}'s service"
                    ),
                    every_j,
                );
                if let Some(cost) = cost {
                    c.out.assumptions.push(format!(
                        "C_s of {name} = {} is the caller's figure, beginning at the trap, for a service that runs \
                         application code, {} (record §6)",
                        cost.value,
                        cost.evidence.words()
                    ));
                }
                cost.map(|cost| (cost, Owner::Caller))
            }
            Service::Unanchored => {
                c.out.stops.unresolved.push(format!(
                    "no record anchors `service.{}`, so none of {name}'s facts can be read, which leaves every J \
                     undeclared (record §6)",
                    source.id
                ));
                every_j_ok = false;
                None
            }
        };
        if !matches!(source.service, Service::Unanchored) {
            every_j_ok &= c.fact(source.external, &format!("external.{}", source.id), every_j);
            every_j_ok &= c.fact(
                source.one_request_per_arrival,
                &format!("one-request-per-arrival.{}", source.id),
                every_j,
            );
        }
        match source.deliverable {
            None => {
                c.out.stops.unresolved.push(format!(
                    "the plan does not state {name} deliverable, which leaves every J undeclared (record §6)"
                ));
                every_j_ok = false;
            }
            Some(false) => {
                c.out.stops.unsupported.push(format!(
                    "the plan leaves {name} enabled and undeliverable, below the controller's threshold for the \
                     hart's machine-mode context (record §6, the variant's condition 5)"
                ));
                every_j_ok = false;
            }
            Some(true) => {}
        }
        if source.priority.is_none() {
            c.out.stops.unresolved.push(format!(
                "the plan states no order among the sources: {name}'s rank cannot be read, which leaves every J \
                 undeclared (record §6)"
            ));
            every_j_ok = false;
        }
        read.push(SourceRead {
            parts: source,
            service,
            separation: separation.map(|value| u128::from(value.value)),
        });
        c.out.sources.push(Source {
            id: source.id.clone(),
            service: service.map(|(cost, _)| cost),
            separation: source.separation,
            jitter: None,
            acknowledge: source.acknowledge,
            priority: source.priority,
            deferred: source.deferred.clone(),
        });
    }
    // The plan's order is strict (§6).
    let mut ranked: Vec<(i64, &str)> = read
        .iter()
        .filter_map(|source| {
            source
                .parts
                .priority
                .map(|rank| (rank, source.parts.id.as_str()))
        })
        .collect();
    ranked.sort_unstable();
    for pair in ranked.windows(2) {
        if pair[0].0 == pair[1].0 {
            c.out.stops.unsupported.push(format!(
                "the plan's order among sources is not strict: `{}` and `{}` share rank {} (record §6)",
                pair[0].1, pair[1].1, pair[0].0
            ));
            every_j_ok = false;
        }
    }

    // §2: the interrupt and hardware facts gate every J.
    every_j_ok &= c.fact(
        kernel.facts.one_external_controller,
        "one-external-controller",
        every_j,
    );
    every_j_ok &= c.fact(kernel.facts.no_empty_claim, "no-empty-claim", every_j);
    let external_before_timer = match kernel.facts.external_before_timer {
        None => {
            c.out.stops.unresolved.push(
                "`external-before-timer` cannot be read, which leaves every J undeclared (record §2)".to_owned(),
            );
            every_j_ok = false;
            true
        }
        Some(true) => true,
        Some(false) => {
            every_j_ok &= c.fact(
                kernel.facts.one_claim_per_trap,
                "one-claim-per-trap, which admits `external-before-timer` `no`",
                every_j,
            );
            false
        }
    };

    // §2: the timer's facts gate J^release of every timer-released task, and every J behind the timer.
    let timer_tasks: Vec<usize> = ordered
        .iter()
        .enumerate()
        .filter(|(_, task)| task.released_by == ReleasedBy::Timer)
        .map(|(index, _)| index)
        .collect();
    let timer_needed = !timer_tasks.is_empty() || !external_before_timer;
    let mut timer_ok = true;
    if timer_needed {
        let gates = "J^release of every timer-released task, and every J behind the timer";
        timer_ok &= c.fact(
            kernel.facts.reprograms_only_in_service,
            "reprograms-only-in-service",
            gates,
        );
        timer_ok &= c.fact(
            application.releases_after_initialisation,
            "releases-after-initialisation, the caller's declaration",
            gates,
        );
        if timer_ok {
            c.out.assumptions.push(
                "every nominal release is at or after the first enabling of interrupts: the caller's declaration \
                 `releases-after-initialisation`, until `M4.10` orders initialisation in the plan (record §2)"
                    .to_owned(),
            );
        }
    }

    // The platform's parts every J needs (§3).
    let s = c.part(platform.switch, "S (one context transition)", every_j);
    let wake = c.part(platform.wake, "W_wake (the idle wake)", every_j);
    let c_rel = c.part(platform.timer_service, "C_rel (one timer service)", every_j);
    let rho = c.part(
        platform.rounding,
        "ρ (the compare's rounding delay)",
        every_j,
    );
    let delta = c.part(
        platform.delivery,
        "δ (the hardware delivery latency)",
        every_j,
    );

    let name_every_j_without_value = |c: &mut Composer<'_>| {
        for task in &ordered {
            c.without_value(&format!("J^release of task `{}`", task.id));
        }
        for source in sources {
            c.without_value(&format!("J_s of source `{}`", source.id));
        }
    };

    if let Some(why) = &overflow_stops_every_interrupt {
        stop_every_interrupt(&mut c, why, sources);
        name_every_j_without_value(&mut c);
        return c.out;
    }
    let (Some(s), Some(wake), Some(c_rel), Some(rho), Some(delta)) = (s, wake, c_rel, rho, delta)
    else {
        name_every_j_without_value(&mut c);
        return c.out;
    };
    let every_service: Option<Vec<(Value, Owner)>> =
        read.iter().map(|source| source.service).collect();
    let every_masked: Option<Vec<Value>> = masked_sections.iter().copied().collect();
    let separations_positive = ordered.iter().all(|task| task.separation > 0)
        && read
            .iter()
            .all(|source| source.separation.is_some_and(|t| t > 0));
    let (Some(every_service), Some(every_masked), true, true) = (
        every_service,
        every_masked,
        every_j_ok,
        separations_positive,
    ) else {
        name_every_j_without_value(&mut c);
        return c.out;
    };

    // `L = max(max_k (CS_k + S), C_rel + S, max_s (C_s + S), W_wake)` (§3, the variant's condition 7).
    let mut maximum = Maximum::new();
    for (task, masked) in ordered.iter().zip(&every_masked) {
        let mut sum = Sum::new();
        sum.add(
            Some(*masked),
            &format!("CS of task `{}`", task.id),
            Owner::Composed,
            1,
        );
        sum.add(Some(s), "S", Owner::Catalog, 1);
        maximum.alternative(&format!("CS of task `{}` + S", task.id), sum);
    }
    let mut sum = Sum::new();
    sum.add(Some(c_rel), "C_rel", Owner::Catalog, 1);
    sum.add(Some(s), "S", Owner::Catalog, 1);
    maximum.alternative("C_rel + S", sum);
    for (source, (service, owner)) in read.iter().zip(&every_service) {
        let mut sum = Sum::new();
        sum.add(
            Some(*service),
            &format!("C_s of source `{}`", source.parts.id),
            *owner,
            1,
        );
        sum.add(Some(s), "S", Owner::Catalog, 1);
        maximum.alternative(&format!("C_s of source `{}` + S", source.parts.id), sum);
    }
    let mut sum = Sum::new();
    sum.add(Some(wake), "W_wake", Owner::Catalog, 1);
    maximum.alternative("W_wake", sum);
    let l = match maximum.finish() {
        Formed::Value(value, evidence, terms) => Some(c.composite(
            "L (the longest masked run)",
            value,
            evidence,
            Rule::Maximum,
            terms,
        )),
        Formed::Overflow => {
            overflow_stops_every_interrupt.get_or_insert_with(|| "L overflows".to_owned());
            None
        }
        Formed::Incomplete => None,
    };

    // §2's assumptions of the environment and the run, named in the conclusion (§6).
    c.out.assumptions.push(
        "no source arrives before the first enabling of interrupts, and firmware or a boot loader leaves no \
         request pending that initialisation does not claim and complete: an arrival assumption (record §2)"
            .to_owned(),
    );
    c.out.assumptions.push(
        "no fatal fault is raised during the run, by a trap or through either of the fault path's entries \
         (record §2)"
            .to_owned(),
    );

    // The order, first taken first (§1, §3).
    let mut order: Vec<Queued> = Vec::with_capacity(sources.len() + 1);
    if !external_before_timer {
        order.push(Queued::Timer);
    }
    let mut by_rank: Vec<(i64, usize)> = read
        .iter()
        .enumerate()
        .filter_map(|(index, source)| source.parts.priority.map(|rank| (rank, index)))
        .collect();
    by_rank.sort_unstable();
    order.extend(by_rank.iter().map(|(_, index)| Queued::Source(*index)));
    if external_before_timer {
        order.push(Queued::Timer);
    }

    // `B_s = L + 2δ`, `B_timer = ρ + C_rel + S + L + 2δ` (§3), each a value in the variant's unit.
    let fits = |b: u128| u64::try_from(b).ok().map(|_| b);
    let two_delta = u128::from(delta.value).checked_mul(2);
    let b_source = l
        .and_then(|l| two_delta.and_then(|two| u128::from(l.value).checked_add(two)))
        .and_then(fits);
    let b_timer = b_source
        .and_then(|b| {
            b.checked_add(u128::from(rho.value))?
                .checked_add(u128::from(c_rel.value))?
                .checked_add(u128::from(s.value))
        })
        .and_then(fits);
    if l.is_some() && (b_source.is_none() || b_timer.is_none()) {
        overflow_stops_every_interrupt.get_or_insert_with(|| "a B_x overflows".to_owned());
    }
    if let Some(why) = &overflow_stops_every_interrupt {
        stop_every_interrupt(&mut c, why, sources);
        name_every_j_without_value(&mut c);
        return c.out;
    }
    let (Some(l), Some(b_source), Some(b_timer)) = (l, b_source, b_timer) else {
        name_every_j_without_value(&mut c);
        return c.out;
    };

    let base_parts = |timer: bool| -> Vec<Part> {
        let mut parts = Vec::new();
        if timer {
            for (value, symbol) in [(rho, "ρ"), (c_rel, "C_rel"), (s, "S")] {
                parts.push(Part {
                    symbol: symbol.to_owned(),
                    owner: Owner::Catalog,
                    times: 1,
                    value: u128::from(value.value),
                    evidence: Some(value.evidence),
                });
            }
        }
        parts.push(Part {
            symbol: "L".to_owned(),
            owner: Owner::Composed,
            times: 1,
            value: u128::from(l.value),
            evidence: Some(l.evidence),
        });
        parts.push(Part {
            symbol: "δ".to_owned(),
            owner: Owner::Catalog,
            times: 2,
            value: u128::from(delta.value),
            evidence: Some(delta.evidence),
        });
        parts
    };
    let base_evidence = |timer: bool| {
        let mut weakest = weaker(l.evidence, delta.evidence);
        if timer {
            weakest = weaker(
                weaker(weaker(weakest, rho.evidence), c_rel.evidence),
                s.evidence,
            );
        }
        weakest
    };

    let mut ahead: Vec<Ahead> = Vec::new();
    let mut stopped: Option<(Which, usize)> = None; // the stop's verdict, and its index in that verdict's list
    let mut gated = false;
    let mut timer_jitter: Option<Value> = None;
    let mut source_jitter: Vec<Option<Value>> = vec![None; sources.len()];
    for entry in order {
        let (what, name) = match entry {
            Queued::Timer => (
                "Δ_timer (J^release of every timer-released task)".to_owned(),
                "the timer".to_owned(),
            ),
            Queued::Source(index) => (
                format!("J_s of source `{}`", read[index].parts.id),
                format!("source `{}`", read[index].parts.id),
            ),
        };
        if gated {
            continue;
        }
        if let Some((which, stop)) = stopped {
            // Named under the stop, with no value, and checked against its no-loss limit through `B_y` (§3, §6).
            let message = match which {
                Which::Unsupported => &mut c.out.stops.unsupported[stop],
                Which::NotEstablished => &mut c.out.stops.not_established[stop],
                Which::Unresolved => &mut c.out.stops.unresolved[stop],
            };
            message.push_str(&format!("; behind it, with no value: {name}"));
            if let Queued::Source(index) = entry {
                let source = &read[index];
                if let (Some(t), Some(ack), Some((service, _))) =
                    (source.separation, source.parts.acknowledge, source.service)
                {
                    let lost = match ack {
                        Acknowledge::AtEntry => b_source >= t,
                        Acknowledge::AtExit => b_source
                            .checked_add(u128::from(service.value))
                            .and_then(|sum| sum.checked_add(u128::from(s.value)))
                            .is_none_or(|sum| sum >= t),
                    };
                    if lost {
                        c.out.stops.unsupported.push(format!(
                            "{name} can lose an arrival: its lower bound B_s = {b_source} already passes its \
                             no-loss limit inside its separation {t} (record §3, the variant's condition 7)"
                        ));
                    }
                }
            }
            continue;
        }
        match entry {
            Queued::Timer => {
                if !timer_ok {
                    gated = true;
                    continue;
                }
                let charge = u128::from(delta.value)
                    .checked_add(u128::from(c_rel.value))
                    .and_then(|sum| sum.checked_add(u128::from(s.value)));
                if timer_tasks.is_empty() {
                    // `Δ_timer` is not computed, and the timer counts nothing ahead of any source (§3).
                    if let Some(charge) = charge {
                        ahead.push(Ahead {
                            charge,
                            ceilings: Vec::new(),
                            part: Part {
                                symbol: "δ + C_rel + S, the timer queued ahead of no release"
                                    .to_owned(),
                                owner: Owner::Plan,
                                times: 0,
                                value: charge,
                                evidence: None,
                            },
                        });
                    }
                    continue;
                }
                let largest_t = timer_tasks
                    .iter()
                    .map(|index| u128::from(ordered[*index].separation))
                    .max()
                    .unwrap_or(0);
                let passes = |next: u128| next > largest_t;
                match least_fixed_point(b_timer, &ahead, &passes, budget) {
                    Ok((value, iterates, counts)) => {
                        let mut parts = base_parts(true);
                        let mut evidence = base_evidence(true);
                        for (q, count) in ahead.iter().zip(counts) {
                            let mut part = q.part.clone();
                            part.times = count;
                            if let Some(e) = part.evidence {
                                evidence = weaker(evidence, e);
                            }
                            parts.push(part);
                        }
                        let composed = c.composite(
                            &what,
                            value,
                            evidence,
                            Rule::FixedPoint,
                            vec![Term {
                                label: String::new(),
                                value,
                                parts,
                            }],
                        );
                        if let Some(last) = c.out.composites.last_mut() {
                            last.iterates = iterates;
                        }
                        timer_jitter = Some(composed);
                        let Some(charge) = charge else {
                            // Unreachable while `b_timer` fits: `δ + C_rel + S ≤ B_timer`.
                            continue;
                        };
                        let ceilings = timer_tasks
                            .iter()
                            .map(|index| {
                                (
                                    u128::from(ordered[*index].jitter_event) + value,
                                    u128::from(ordered[*index].separation),
                                )
                            })
                            .collect();
                        ahead.push(Ahead {
                            charge,
                            ceilings,
                            part: Part {
                                symbol: format!(
                                    "δ + C_rel + S, the timer queued ahead (one service per release, J_k = \
                                     J_k^event + {value})"
                                ),
                                owner: Owner::Plan,
                                times: 0,
                                value: charge,
                                evidence: Some(weaker(
                                    weaker(delta.evidence, c_rel.evidence),
                                    weaker(s.evidence, composed.evidence),
                                )),
                            },
                        });
                    }
                    Err((stop, iterates)) => {
                        let (message, which) =
                            stop_message(&stop, &name, &iterates, largest_t, false);
                        stopped = Some((which, c.out.stops.push(which, message)));
                    }
                }
            }
            Queued::Source(index) => {
                let source = &read[index];
                let (Some(t), Some((service, owner))) = (source.separation, source.service) else {
                    continue; // unreachable: every service and separation was read above
                };
                let Some(ack) = source.parts.acknowledge else {
                    // The variant's condition 9 names the missing acknowledge point.
                    gated = true;
                    continue;
                };
                let c_plus_s = u128::from(service.value).checked_add(u128::from(s.value));
                let passes = |next: u128| match ack {
                    Acknowledge::AtEntry => next >= t,
                    Acknowledge::AtExit => c_plus_s
                        .and_then(|sum| next.checked_add(sum))
                        .is_none_or(|sum| sum >= t),
                };
                match least_fixed_point(b_source, &ahead, &passes, budget) {
                    Ok((value, iterates, counts)) => {
                        let mut parts = base_parts(false);
                        let mut evidence = base_evidence(false);
                        for (q, count) in ahead.iter().zip(counts) {
                            let mut part = q.part.clone();
                            part.times = count;
                            if let Some(e) = part.evidence {
                                evidence = weaker(evidence, e);
                            }
                            parts.push(part);
                        }
                        let composed = c.composite(
                            &what,
                            value,
                            evidence,
                            Rule::FixedPoint,
                            vec![Term {
                                label: String::new(),
                                value,
                                parts,
                            }],
                        );
                        if let Some(last) = c.out.composites.last_mut() {
                            last.iterates = iterates;
                        }
                        source_jitter[index] = Some(composed);
                        let Some(charge) = u128::from(delta.value)
                            .checked_add(u128::from(service.value))
                            .and_then(|sum| sum.checked_add(u128::from(s.value)))
                        else {
                            continue; // unreachable while `b_source` fits: `δ + C_s + S ≤ L + 2δ`
                        };
                        ahead.push(Ahead {
                            charge,
                            ceilings: vec![(value, t)],
                            part: Part {
                                symbol: format!(
                                    "δ + C_s of source `{}` + S, queued ahead (T_s {t}, J_s {value})",
                                    source.parts.id
                                ),
                                owner: if owner == Owner::Caller {
                                    Owner::Caller
                                } else {
                                    Owner::Plan
                                },
                                times: 0,
                                value: charge,
                                evidence: Some(weaker(
                                    weaker(delta.evidence, service.evidence),
                                    weaker(s.evidence, composed.evidence),
                                )),
                            },
                        });
                    }
                    Err((stop, iterates)) => {
                        let (message, which) = stop_message(&stop, &name, &iterates, t, true);
                        stopped = Some((which, c.out.stops.push(which, message)));
                    }
                }
            }
        }
    }

    // The composed values reach the variant (§6).
    for (position, task) in ordered.iter().enumerate() {
        let what = format!("J^release of task `{}`", task.id);
        let (jitter, from) = match &task.released_by {
            ReleasedBy::Timer => (timer_jitter, "Δ_timer".to_owned()),
            ReleasedBy::Source { source, .. } => (
                read.iter()
                    .position(|read| &read.parts.id == source)
                    .and_then(|index| source_jitter[index]),
                format!("J_s of source `{source}`"),
            ),
        };
        match jitter {
            Some(jitter) => {
                c.out.tasks[position].jitter_release = Some(c.composite(
                    &what,
                    u128::from(jitter.value),
                    jitter.evidence,
                    Rule::Sum,
                    vec![Term {
                        label: String::new(),
                        value: u128::from(jitter.value),
                        parts: vec![Part {
                            symbol: from,
                            owner: Owner::Composed,
                            times: 1,
                            value: u128::from(jitter.value),
                            evidence: Some(jitter.evidence),
                        }],
                    }],
                ));
            }
            None => c.without_value(&what),
        }
    }
    for (index, source) in c.out.sources.iter_mut().enumerate() {
        source.jitter = source_jitter[index];
        if source_jitter[index].is_none() {
            c.out
                .without_value
                .push(format!("J_s of source `{}`", source.id));
        }
    }
    c.out
}

/// An overflow in `CS_i`, `L` or a `B_x` stops every interrupt as its own overflow would: sources get
/// `unsupported-profile` and the timer `not-established` (record §6).
fn stop_every_interrupt(c: &mut Composer<'_>, why: &str, sources: &[SourceParts]) {
    for source in sources {
        c.out.stops.unsupported.push(format!(
            "{why}, which stops source `{}` as its own overflow would: J_s cannot be bounded below its no-loss \
             limit (record §6, the variant's condition 7)",
            source.id
        ));
    }
    c.out.stops.not_established.push(format!(
        "{why}, which stops the timer as its own overflow would: no single-job bound applies (record §6)"
    ));
}

/// The stop's statement and its verdict: a source's stop is `unsupported-profile`, the timer's `not-established`,
/// and a limit `analysis-inconclusive` for either (record §6).
fn stop_message(
    stop: &Stop,
    name: &str,
    iterates: &[u128],
    limit: u128,
    source: bool,
) -> (String, Which) {
    let trail = iterates
        .iter()
        .map(u128::to_string)
        .collect::<Vec<_>>()
        .join(" → ");
    let (consequence, verdict) = if source {
        (
            "so J_s cannot be bounded below its no-loss limit (record §6, the variant's condition 7)",
            Which::Unsupported,
        )
    } else {
        (
            "so no single-job bound applies (record §6)",
            Which::NotEstablished,
        )
    };
    match stop {
        Stop::NoFixedPoint => (
            format!(
                "J of {name} has no fixed point: what queues ahead of it can arrive at least as fast as it is \
                 served (record §3), {consequence}"
            ),
            verdict,
        ),
        Stop::PastBound => (
            format!(
                "J of {name} passed its refusal bound {limit} before reaching a fixed point, Δ {trail} (record §3), \
                 {consequence}"
            ),
            verdict,
        ),
        Stop::Overflow => (
            format!("J of {name} overflows, Δ {trail} (record §3), {consequence}"),
            verdict,
        ),
        Stop::Budget => (
            format!(
                "J of {name} did not reach a fixed point within the step budget, Δ {trail}: a named resource limit \
                 (record §3)"
            ),
            Which::Unresolved,
        ),
        Stop::Precheck => (
            format!(
                "the pre-check's exact sum for J of {name} does not fit 128-bit numerator and denominator: a named \
                 resource limit (record §3)"
            ),
            Which::Unresolved,
        ),
    }
}
