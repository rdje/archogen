//! The runtime analysis variant `fixed-priority-with-overheads/1` (leaf `M2.6.2`).
//!
//! ⛔ **This module implements `docs/decisions/decision_runtime-analysis-variant.md` and nothing else.** Every
//! input, condition, term and verdict below is a section of that record, cited where it is used. Anything the
//! implementation needs that the record does not say goes back to the record first. Three independent reviews
//! each found a way an earlier draft could under-estimate a response time, so a local "improvement" here is
//! how a fourth gets in.
//!
//! The shape follows the baseline (`crate::response`): admission is a constructor precondition ([`admit`]), the
//! recurrence records its witness, and the three outcomes that are easy to collapse stay apart. The variant
//! differs in the one way the record insists on. Its bound is an **envelope**, so a bound past a deadline is
//! `not-established` and never a counterexample (§5).

use core::fmt;

use archogen_evidence::claim::Conclusion;

/// The model this variant reasons in, apart from the baseline's (record, "The fact / decision").
pub const MODEL: &str =
    "fixed-priority-with-overheads/1 (docs/decisions/decision_runtime-analysis-variant.md)";

/// The iteration budget (record §2): the derived step bound is pseudo-polynomial in `T_i`, so reaching this is a
/// named resource limit, `analysis-inconclusive`.
pub const ITERATION_BUDGET: usize = 1_000_000;

/// The conditions every conclusion of this variant carries (record §4, in the order given there).
pub const CONDITIONS: &[&str] = &[
    "one processor",
    "distinct fixed priorities; preemption at every point outside a masked section, a service or a transition",
    "constrained deadlines (D ≤ T)",
    "no self-suspension, no scheduler lock or deferred preemption, and data shared only inside declared masked sections",
    "every enabled interrupt declared; no nesting; every masked section masks every interrupt; every service \
     preempts every task; a pending interrupt is taken before a resumed instruction; transitions end unmasked; \
     eager switching or deferred saves inside S",
    "an event-driven timer with level compare that rounds up, whose service releases no task before its nominal \
     release and every due one",
    "every J^release and J_s is the engine's bound of the full delivery delay; the floors this analysis checks \
     are necessary, not sufficient",
    "every cost bound holds under any preemption pattern, and γ bounds the state one preemption destroys",
];

/// The §7.3 evidence category of a numerical input (record §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Evidence {
    /// Assumed, with no measurement or argument behind it.
    Assumed,
    /// An observed maximum; with a safety multiplier it is still an empirical assumption (§7.3).
    ObservedMaximum,
    /// A bound supplied from outside, whose own evidence is elsewhere.
    ExternallySupplied,
    /// Analytically established.
    Analytical,
}

impl Evidence {
    /// The words used when a conclusion names an input by its evidence.
    #[must_use]
    pub const fn words(self) -> &'static str {
        match self {
            Self::Assumed => "assumed",
            Self::ObservedMaximum => "an observed maximum",
            Self::ExternallySupplied => "an externally supplied bound",
            Self::Analytical => "analytically established",
        }
    }
}

/// A numerical input with its evidence (record §1: "Every numerical input carries its §7.3 evidence category").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Value {
    /// In the analysis's integer time unit, already rounded as §1 says: costs and delays up, separations down.
    pub value: u64,
    /// Where it comes from.
    pub evidence: Evidence,
}

/// What releases a task (record §1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ReleasedBy {
    /// The event-driven timer.
    Timer,
    /// A named interrupt source; its arrival is the task's nominal release.
    Source {
        /// The source's identity.
        source: String,
        /// Whether every arrival releases the task, or only some.
        every_arrival: bool,
    },
}

/// A task's declared behaviour (record §1, "task facts"). `None` is undeclared, and no fact defaults.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct TaskFacts {
    /// Whether it suspends itself.
    pub suspends: Option<bool>,
    /// Whether it locks the scheduler or defers preemption.
    pub locks_scheduler: Option<bool>,
    /// Whether it shares data outside its masked sections.
    pub shares_outside_sections: Option<bool>,
}

/// One task of the runtime model (record §1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeTask {
    /// Stable identity, used in witnesses and reasons.
    pub id: String,
    /// The rank, ascending highest-first.
    pub priority: i64,
    /// `C_i`: from the transition in up to the decided switch after completion, services excluded.
    pub computation: Option<Value>,
    /// `T_i`, minimum separation between nominal releases (from the description).
    pub separation: u64,
    /// `D_i`, from the nominal release (from the description).
    pub deadline: u64,
    /// `J_i^event`, the description's `jitter` clause; the language makes an absent clause `0`.
    pub jitter_event: u64,
    /// `J_i^release`, engine knowledge: nominal release to the start of the releasing service.
    pub jitter_release: Option<Value>,
    /// `CS_i`, the longest masked section the task executes.
    pub masked_section: Option<Value>,
    /// What releases it.
    pub released_by: ReleasedBy,
    /// Its declared behaviour.
    pub facts: TaskFacts,
}

/// Where a source's acknowledgement happens (record §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Acknowledge {
    /// At service entry: no loss needs `J_s < T_s`.
    AtEntry,
    /// At service exit: no loss needs `J_s + C_s + S < T_s`.
    AtExit,
}

/// Where a source's deferred work runs (record §1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Deferred {
    /// The service defers nothing.
    Nothing,
    /// A declared task runs it.
    Task(String),
}

/// One interrupt source other than the timer (record §1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Source {
    /// Its identity, as the build's enabled set names it.
    pub id: String,
    /// `C_s`, one service, entry to exit.
    pub service: Option<Value>,
    /// `T_s`, minimum separation between arrivals.
    pub separation: Option<Value>,
    /// `J_s`, the most by which a service can start after an arrival.
    pub jitter: Option<Value>,
    /// The acknowledge point.
    pub acknowledge: Option<Acknowledge>,
    /// Its rank among interrupts.
    pub priority: Option<i64>,
    /// Where its deferred work runs.
    pub deferred: Option<Deferred>,
}

/// The platform facts of record §1, each a declared yes or no. `None` is undeclared.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct PlatformFacts {
    /// One processor (condition 1).
    pub one_processor: Option<bool>,
    /// Preemption at every point outside a masked section, a service or a transition (condition 2).
    pub preemptive_everywhere: Option<bool>,
    /// Interrupts do not nest (condition 5).
    pub interrupts_do_not_nest: Option<bool>,
    /// Every masked section masks every interrupt (condition 5).
    pub sections_mask_every_interrupt: Option<bool>,
    /// Every service preempts every task (condition 5).
    pub services_preempt_every_task: Option<bool>,
    /// A pending interrupt is taken before a resumed instruction, and every transition ends unmasked (condition 5).
    pub pending_taken_and_transitions_unmasked: Option<bool>,
    /// Switching is eager, or `S` includes every deferred save and restore (condition 5).
    pub eager_switching: Option<bool>,
    /// Every interrupt taken is the timer's, or a service of a declared source for one of its arrivals, and no
    /// arrival is served twice (condition 5, added by leaf `M2.11`): the source term counts services by arrivals.
    pub services_paid_by_arrivals: Option<bool>,
    /// The timer is event-driven (condition 6).
    pub timer_event_driven: Option<bool>,
    /// The compare has level semantics (condition 6).
    pub compare_level: Option<bool>,
    /// The compare value is rounded up (condition 6).
    pub compare_rounds_up: Option<bool>,
    /// The due-check and the compare use one counter in one unit and round alike (condition 6).
    pub due_check_matches_compare: Option<bool>,
    /// A service releases a task only once its nominal release has passed (condition 6, the third review).
    pub no_early_release: Option<bool>,
    /// A timer interrupt is raised only when a release is due, and its service releases every due task
    /// (condition 6).
    pub raised_only_when_due: Option<bool>,
    /// Only the timer service releases timer-released tasks (condition 6).
    pub only_timer_releases_timer_tasks: Option<bool>,
    /// Every cost bound holds under any preemption pattern (condition 8).
    pub costs_hold_under_any_preemption: Option<bool>,
}

/// The interrupts the build enables (record §1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnabledSet {
    /// Every enabled interrupt by name, `timer` included, non-maskable interrupts and firmware traps included.
    pub interrupts: Vec<String>,
    /// Whether it comes from §7.5's resolved plan. Before `M4` it is the catalog's platform declaration, and the
    /// conclusion names that as an assumption.
    pub from_resolved_plan: bool,
}

/// The platform inputs of record §1.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Platform {
    /// `S`, one context transition.
    pub switch: Option<Value>,
    /// `W_wake`, pending to unmasked while idle.
    pub wake: Option<Value>,
    /// `γ`, the preemption delay.
    pub preemption_delay: Option<Value>,
    /// `C_rel`, one timer service.
    pub timer_service: Option<Value>,
    /// `ρ`, the largest delay the compare's rounding adds.
    pub rounding: Option<Value>,
    /// `δ`, the hardware's delivery latency.
    pub delivery: Option<Value>,
    /// The platform facts.
    pub facts: PlatformFacts,
    /// The build's enabled interrupts.
    pub enabled: Option<EnabledSet>,
}

/// The two verdicts a refusal carries (record §5), spelled as §5.5 spells them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefusalVerdict {
    /// Outside the model: a condition fails, or an interrupt is unmodelled.
    UnsupportedProfile,
    /// An input or fact missing, or a declared delay below its floor: an unresolved bound.
    AnalysisInconclusive,
}

impl RefusalVerdict {
    /// The §5.5 slug.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::UnsupportedProfile => "unsupported-profile",
            Self::AnalysisInconclusive => "analysis-inconclusive",
        }
    }
}

/// Why a task set was not admitted. It carries every reason of the winning verdict. `unsupported-profile`
/// outranks `analysis-inconclusive` (record §5, `Verdict::precedence`), so a set with both is refused as
/// unsupported, and its missing inputs are not the first thing to fix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    /// The verdict.
    pub verdict: RefusalVerdict,
    /// Every reason of that verdict, each naming its condition.
    pub reasons: Vec<String>,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}: {}", self.verdict.slug(), self.reasons.join("; "))
    }
}

/// A task with every input resolved, in the analysis's arithmetic.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Resolved {
    id: String,
    computation: u128,
    separation: u128,
    deadline: u128,
    jitter: u128,
}

/// A source with every input resolved.
#[derive(Debug, Clone, PartialEq, Eq)]
struct ResolvedSource {
    service: u128,
    separation: u128,
    jitter: u128,
}

/// A task set this variant applies to. Constructible only through [`admit`]. Tasks are held
/// highest-priority-first, so `hp(i)` is a prefix.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RuntimeSet {
    tasks: Vec<Resolved>,
    sources: Vec<ResolvedSource>,
    switch: u128,
    preemption_delay: u128,
    timer_service: u128,
    assumptions: Vec<String>,
}

impl RuntimeSet {
    /// The admitted tasks' identities, highest priority first.
    #[must_use]
    pub fn task_ids(&self) -> Vec<&str> {
        self.tasks.iter().map(|task| task.id.as_str()).collect()
    }

    /// The assumptions a conclusion from this set carries: [`CONDITIONS`], then each input whose evidence is not
    /// analytically established, and anything else the record names (record §5).
    #[must_use]
    pub fn assumptions(&self) -> &[String] {
        &self.assumptions
    }
}

/// Collects what admission finds, each in the verdict it belongs to.
#[derive(Default)]
struct Findings {
    unsupported: Vec<String>,
    unresolved: Vec<String>,
    assumptions: Vec<String>,
}

impl Findings {
    /// A numerical input: present, recorded as an assumption unless analytical, or missing.
    fn value(&mut self, value: Option<Value>, what: &str) -> Option<u64> {
        let Some(value) = value else {
            self.unresolved.push(format!(
                "{what} is not declared (condition 9: no input defaults)"
            ));
            return None;
        };
        if value.evidence != Evidence::Analytical {
            self.assumptions.push(format!(
                "{what} = {} ({})",
                value.value,
                value.evidence.words()
            ));
        }
        Some(value.value)
    }

    /// A declared fact that must hold: missing is unresolved, false is outside the model.
    fn fact(&mut self, fact: Option<bool>, what: &str, condition: u8) {
        match fact {
            None => self.unresolved.push(format!(
                "the platform does not declare whether {what} (condition 9)"
            )),
            Some(false) => self.unsupported.push(format!(
                "{what} does not hold, which is outside {MODEL} (condition {condition})"
            )),
            Some(true) => {}
        }
    }

    /// A task fact that must be false: missing is unresolved, true is outside the model.
    fn task_fact(&mut self, fact: Option<bool>, task: &str, what: &str) {
        match fact {
            None => self.unresolved.push(format!(
                "task `{task}` does not declare whether it {what} (condition 9)"
            )),
            Some(true) => self.unsupported.push(format!(
                "task `{task}` {what}, which is outside {MODEL} (condition 4)"
            )),
            Some(false) => {}
        }
    }
}

/// Admit a task set, or say why this variant does not apply (record §4, with the verdicts of §5).
///
/// # Errors
///
/// A [`Refusal`]: `unsupported-profile` when a condition fails or an enabled interrupt is unmodelled, and
/// otherwise `analysis-inconclusive` when an input or a fact is missing or a declared delay is below its floor.
#[allow(clippy::too_many_lines)]
pub fn admit(
    tasks: &[RuntimeTask],
    sources: &[Source],
    platform: &Platform,
) -> Result<RuntimeSet, Refusal> {
    let mut found = Findings::default();
    let facts = platform.facts;

    // Conditions 1, 2, 5, 6 and 8: the platform facts.
    found.fact(facts.one_processor, "the system has one processor", 1);
    found.fact(
        facts.preemptive_everywhere,
        "preemption happens at every point outside a masked section, a service or a transition",
        2,
    );
    found.fact(facts.interrupts_do_not_nest, "interrupts do not nest", 5);
    found.fact(
        facts.sections_mask_every_interrupt,
        "every masked section masks every interrupt",
        5,
    );
    found.fact(
        facts.services_preempt_every_task,
        "every service preempts every task",
        5,
    );
    found.fact(
        facts.pending_taken_and_transitions_unmasked,
        "a pending interrupt is taken before a resumed instruction and every transition ends unmasked",
        5,
    );
    found.fact(
        facts.eager_switching,
        "switching is eager, or S includes every deferred save and restore",
        5,
    );
    found.fact(
        facts.services_paid_by_arrivals,
        "every interrupt taken is the timer's or a declared source's service for one of its arrivals, and no arrival is served twice",
        5,
    );
    found.fact(facts.timer_event_driven, "the timer is event-driven", 6);
    found.fact(
        facts.compare_level,
        "the timer's compare has level semantics",
        6,
    );
    found.fact(
        facts.compare_rounds_up,
        "the timer's compare value is rounded up",
        6,
    );
    found.fact(
        facts.due_check_matches_compare,
        "the due-check and the compare use one counter in one unit and round alike",
        6,
    );
    found.fact(
        facts.no_early_release,
        "a service releases a task only once its nominal release has passed",
        6,
    );
    found.fact(
        facts.raised_only_when_due,
        "a timer interrupt is raised only when a release is due, and releases every due task",
        6,
    );
    found.fact(
        facts.only_timer_releases_timer_tasks,
        "only the timer service releases timer-released tasks",
        6,
    );
    found.fact(
        facts.costs_hold_under_any_preemption,
        "every cost bound holds under any preemption pattern",
        8,
    );

    // The platform's numerical inputs.
    let switch = found.value(platform.switch, "S (one context transition)");
    let wake = found.value(platform.wake, "W_wake (the idle wake)");
    let gamma = found.value(platform.preemption_delay, "γ (the preemption delay)");
    let timer_service = found.value(platform.timer_service, "C_rel (one timer service)");
    let rounding = found.value(platform.rounding, "ρ (the compare's rounding delay)");
    let delivery = found.value(platform.delivery, "δ (the hardware delivery latency)");

    // Condition 5: the declared interrupts plus the timer equal the build's enabled set.
    let mut declared: Vec<&str> = sources.iter().map(|source| source.id.as_str()).collect();
    declared.push("timer");
    declared.sort_unstable();
    if declared.windows(2).any(|pair| pair[0] == pair[1]) {
        found.unsupported.push(
            "two interrupt sources share one name, so the declared set cannot be compared with the enabled \
             one (condition 5)"
                .to_string(),
        );
    }
    match &platform.enabled {
        None => found
            .unresolved
            .push("the build's enabled interrupts are not declared (condition 9)".to_string()),
        Some(enabled) => {
            let mut enabled_names: Vec<&str> =
                enabled.interrupts.iter().map(String::as_str).collect();
            enabled_names.sort_unstable();
            enabled_names.dedup();
            let undeclared: Vec<&str> = enabled_names
                .iter()
                .copied()
                .filter(|name| !declared.contains(name))
                .collect();
            let not_enabled: Vec<&str> = declared
                .iter()
                .copied()
                .filter(|name| !enabled_names.contains(name))
                .collect();
            if !undeclared.is_empty() {
                found.unsupported.push(format!(
                    "the build enables {} and no source declares it: unmodeled-interrupt-load (condition 5)",
                    undeclared.join(", ")
                ));
            }
            if !not_enabled.is_empty() {
                found.unsupported.push(format!(
                    "{} is declared and the build does not enable it (condition 5)",
                    not_enabled.join(", ")
                ));
            }
            if !enabled.from_resolved_plan {
                found.assumptions.push(
                    "the enabled interrupts are the catalog's platform declaration, not a resolved plan's (§7.5)"
                        .to_string(),
                );
            }
        }
    }

    // Condition 2: distinct priorities. Condition 3: constrained deadlines and positive costs and separations.
    let mut ordered: Vec<&RuntimeTask> = tasks.iter().collect();
    ordered.sort_by_key(|task| task.priority);
    if ordered.is_empty() {
        found.unsupported.push(
            "the task set is empty, so there is nothing to analyse (condition 3)".to_string(),
        );
    }
    for pair in ordered.windows(2) {
        if pair[0].priority == pair[1].priority {
            found.unsupported.push(format!(
                "tasks `{}` and `{}` share priority {} (condition 2)",
                pair[0].id, pair[1].id, pair[0].priority
            ));
        }
    }

    // Each source (condition 5), resolved when complete.
    let mut resolved_sources = Vec::new();
    let mut source_jitter: Vec<(&str, Option<u64>, Option<u64>)> = Vec::new();
    for source in sources {
        let name = format!("source `{}`", source.id);
        let service = found.value(source.service, &format!("C_s of {name}"));
        let separation = found.value(source.separation, &format!("T_s of {name}"));
        let jitter = found.value(source.jitter, &format!("J_s of {name}"));
        if source.acknowledge.is_none() {
            found.unresolved.push(format!(
                "{name} does not declare its acknowledge point (condition 9)"
            ));
        }
        if source.priority.is_none() {
            found.unresolved.push(format!(
                "{name} does not declare its interrupt priority (condition 9)"
            ));
        }
        match &source.deferred {
            None => found.unresolved.push(format!(
                "{name} does not declare where its deferred work runs (condition 9)"
            )),
            Some(Deferred::Task(task)) if !tasks.iter().any(|t| &t.id == task) => {
                found.unsupported.push(format!(
                    "{name} defers work to `{task}`, which is not a declared task (condition 5)"
                ));
            }
            Some(_) => {}
        }
        if separation == Some(0) {
            found
                .unsupported
                .push(format!("{name} has a zero separation (condition 3)"));
        }
        source_jitter.push((source.id.as_str(), jitter, separation));
        if let (Some(service), Some(separation), Some(jitter)) = (service, separation, jitter) {
            resolved_sources.push((source, service, separation, jitter));
        }
    }

    // Each task (conditions 3, 4, 5).
    let mut resolved_tasks = Vec::new();
    for task in &ordered {
        let name = format!("task `{}`", task.id);
        let computation = found.value(task.computation, &format!("C of {name}"));
        let jitter_release = found.value(task.jitter_release, &format!("J^release of {name}"));
        let masked = found.value(task.masked_section, &format!("CS of {name}"));
        found.task_fact(task.facts.suspends, &task.id, "suspends itself");
        found.task_fact(
            task.facts.locks_scheduler,
            &task.id,
            "locks the scheduler or defers preemption",
        );
        found.task_fact(
            task.facts.shares_outside_sections,
            &task.id,
            "shares data outside its masked sections",
        );
        if task.separation == 0 {
            found
                .unsupported
                .push(format!("{name} has a zero separation (condition 3)"));
        }
        if task.deadline > task.separation {
            found.unsupported.push(format!(
                "{name} has deadline {} past its separation {} (condition 3)",
                task.deadline, task.separation
            ));
        }
        if computation == Some(0) {
            found.unsupported.push(format!(
                "{name} declares zero computation, an omitted bound (condition 3)"
            ));
        }
        if let ReleasedBy::Source {
            source,
            every_arrival,
        } = &task.released_by
        {
            match source_jitter.iter().find(|(id, _, _)| id == source) {
                None => found.unsupported.push(format!(
                    "{name} is released by `{source}`, which is not a declared source (condition 5)"
                )),
                Some((_, _, separation)) => {
                    if *every_arrival {
                        if let Some(separation) = separation {
                            if task.separation > *separation {
                                found.unsupported.push(format!(
                                    "{name} is released on every arrival of `{source}`, so its separation {} \
                                     cannot exceed the source's {separation} (condition 5)",
                                    task.separation
                                ));
                            }
                        }
                    } else {
                        found.assumptions.push(format!(
                            "{name} is released on some arrivals of `{source}` only, at most once per {}",
                            task.separation
                        ));
                    }
                }
            }
        }
        resolved_tasks.push((task, computation, jitter_release, masked));
    }

    // Condition 7: the floors (necessary only) and no loss. Computed only when every input they need is present.
    let masked_all: Option<Vec<u64>> = resolved_tasks.iter().map(|(_, _, _, cs)| *cs).collect();
    let sources_complete = resolved_sources.len() == sources.len();
    if let (Some(s), Some(wake), Some(c_rel), Some(rho), Some(delta), Some(sections), true) = (
        switch,
        wake,
        timer_service,
        rounding,
        delivery,
        masked_all,
        sources_complete,
    ) {
        let s = u128::from(s);
        let longest_section = sections
            .iter()
            .map(|cs| u128::from(*cs) + s)
            .max()
            .unwrap_or(0);
        let longest_service = resolved_sources
            .iter()
            .map(|(_, c, _, _)| u128::from(*c) + s)
            .max()
            .unwrap_or(0);
        let run = longest_section
            .max(u128::from(c_rel) + s)
            .max(longest_service)
            .max(u128::from(wake));
        let timer_floor = u128::from(rho) + u128::from(delta) + run;
        let source_floor = u128::from(delta) + run;
        for (source, c_s, t_s, j_s) in &resolved_sources {
            if u128::from(*j_s) < source_floor {
                found.unresolved.push(format!(
                    "J_s of source `{}` is {j_s}, below its floor δ + L = {source_floor}: a delay that cannot be \
                     true (condition 7)",
                    source.id
                ));
            }
            let lost = match source.acknowledge {
                Some(Acknowledge::AtEntry) => j_s >= t_s,
                Some(Acknowledge::AtExit) => {
                    u128::from(*j_s) + u128::from(*c_s) + s >= u128::from(*t_s)
                }
                None => false,
            };
            if lost {
                found.unsupported.push(format!(
                    "source `{}` can lose an arrival: its delay and service do not fit inside its separation \
                     {t_s} (condition 7)",
                    source.id
                ));
            }
        }
        for (task, _, jitter_release, _) in &resolved_tasks {
            let Some(jitter_release) = jitter_release else {
                continue;
            };
            let floor = match &task.released_by {
                ReleasedBy::Timer => Some(timer_floor),
                ReleasedBy::Source { source, .. } => source_jitter
                    .iter()
                    .find(|(id, _, _)| id == source)
                    .and_then(|(_, jitter, _)| *jitter)
                    .map(u128::from),
            };
            if let Some(floor) = floor {
                if u128::from(*jitter_release) < floor {
                    found.unresolved.push(format!(
                        "J^release of task `{}` is {jitter_release}, below its floor {floor}: a delay that \
                         cannot be true (condition 7)",
                        task.id
                    ));
                }
            }
        }
    }

    if !found.unsupported.is_empty() {
        return Err(Refusal {
            verdict: RefusalVerdict::UnsupportedProfile,
            reasons: found.unsupported,
        });
    }
    if !found.unresolved.is_empty() {
        return Err(Refusal {
            verdict: RefusalVerdict::AnalysisInconclusive,
            reasons: found.unresolved,
        });
    }

    // Everything is present here: an unresolved input would have refused above.
    let resolve = |value: Option<u64>| u128::from(value.unwrap_or_default());
    let mut assumptions: Vec<String> = CONDITIONS.iter().map(|c| (*c).to_string()).collect();
    assumptions.extend(found.assumptions);
    Ok(RuntimeSet {
        tasks: resolved_tasks
            .iter()
            .map(|(task, computation, jitter_release, _)| Resolved {
                id: task.id.clone(),
                computation: resolve(*computation),
                separation: u128::from(task.separation),
                deadline: u128::from(task.deadline),
                jitter: u128::from(task.jitter_event) + resolve(*jitter_release),
            })
            .collect(),
        sources: resolved_sources
            .iter()
            .map(|(_, service, separation, jitter)| ResolvedSource {
                service: u128::from(*service),
                separation: u128::from(*separation),
                jitter: u128::from(*jitter),
            })
            .collect(),
        switch: resolve(switch),
        preemption_delay: resolve(gamma),
        timer_service: resolve(timer_service),
        assumptions,
    })
}

/// The recurrence sequence for one task: `w^(0), w^(1), …` (record §2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Witness {
    /// Whose response this is.
    pub task: String,
    /// The iterates, in order. The last two are equal exactly when it converged.
    pub sequence: Vec<u128>,
}

impl Witness {
    /// The sequence, for a reader to check by hand.
    #[must_use]
    pub fn render(&self) -> String {
        let steps: Vec<String> = self.sequence.iter().map(u128::to_string).collect();
        format!("{}: w {}", self.task, steps.join(" → "))
    }
}

/// What the variant established for one task (record §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Outcome {
    /// Converged with `R_i ≤ D_i`.
    Holds {
        /// `R_i = J_i + w_i`.
        response: u128,
        /// `D_i`.
        deadline: u128,
        /// The iteration.
        witness: Witness,
    },
    /// Not established: a bound past the deadline, no fixed point, or the busy-period stop. Never a
    /// counterexample, because the bound is an envelope.
    NotEstablished {
        /// Why, concretely.
        why: String,
        /// How far the iteration got.
        witness: Witness,
    },
    /// A named resource limit: the utilisation too large for exact arithmetic, or the iteration budget.
    Inconclusive {
        /// The limit.
        why: String,
        /// How far the iteration got.
        witness: Witness,
    },
}

/// `⌈ numerator / denominator ⌉`, for a positive denominator.
const fn ceil_div(numerator: u128, denominator: u128) -> u128 {
    numerator / denominator
        + if numerator.is_multiple_of(denominator) {
            0
        } else {
            1
        }
}

const fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let r = a % b;
        a = b;
        b = r;
    }
    a
}

/// The interference utilisation of task `index`, exactly, as a reduced fraction; `None` when it does not fit
/// 128-bit numerator and denominator (record §2's pre-check).
fn utilisation(set: &RuntimeSet, index: usize) -> Option<(u128, u128)> {
    let two_s = set.switch.checked_mul(2)?;
    let job = |c: u128| c.checked_add(two_s)?.checked_add(set.preemption_delay);
    let mut terms: Vec<(u128, u128)> = Vec::new();
    for task in &set.tasks[..index] {
        terms.push((job(task.computation)?, task.separation));
    }
    for task in &set.tasks {
        terms.push((
            set.timer_service.checked_add(set.preemption_delay)?,
            task.separation,
        ));
    }
    for source in &set.sources {
        terms.push((
            source.service.checked_add(set.preemption_delay)?,
            source.separation,
        ));
    }
    let (mut num, mut den) = (0u128, 1u128);
    for (n, d) in terms {
        let g = gcd(den, d);
        let lcm = den.checked_div(g)?.checked_mul(d)?;
        num = num
            .checked_mul(lcm / den)?
            .checked_add(n.checked_mul(lcm / d)?)?;
        den = lcm;
        let r = gcd(num, den).max(1);
        num /= r;
        den /= r;
    }
    Some((num, den))
}

/// One step of the recurrence from `w` for task `index`; `None` on overflow, which the record makes the
/// busy-period stop (record §2).
fn next(set: &RuntimeSet, index: usize, w: u128) -> Option<u128> {
    let task = &set.tasks[index];
    let two_s = set.switch.checked_mul(2)?;
    let mut total = task.computation.checked_add(set.switch)?;
    for j in &set.tasks[..index] {
        let arrivals = ceil_div(w.checked_add(j.jitter)?, j.separation);
        let cost = j
            .computation
            .checked_add(two_s)?
            .checked_add(set.preemption_delay)?;
        total = total.checked_add(arrivals.checked_mul(cost)?)?;
    }
    let release = set.timer_service.checked_add(set.preemption_delay)?;
    for k in &set.tasks {
        let arrivals = ceil_div(w.checked_add(k.jitter)?, k.separation);
        total = total.checked_add(arrivals.checked_mul(release)?)?;
    }
    for s in &set.sources {
        let arrivals = ceil_div(w.checked_add(s.jitter)?, s.separation);
        let cost = s.service.checked_add(set.preemption_delay)?;
        total = total.checked_add(arrivals.checked_mul(cost)?)?;
    }
    Some(total)
}

/// The bound for the task at `index` of `set` (record §2 and §5).
///
/// # Panics
///
/// If `index` is out of range.
#[must_use]
pub fn response(set: &RuntimeSet, index: usize) -> Outcome {
    let task = &set.tasks[index];
    let mut witness = Witness {
        task: task.id.clone(),
        sequence: Vec::new(),
    };
    match utilisation(set, index) {
        None => {
            return Outcome::Inconclusive {
                why: format!(
                    "the exact interference utilisation for task `{}` does not fit 128-bit arithmetic, a named \
                     resource limit",
                    task.id
                ),
                witness,
            }
        }
        Some((num, den)) if num >= den => {
            return Outcome::NotEstablished {
                why: format!(
                    "the interference utilisation for task `{}` is {num}/{den}, at least one, so the recurrence \
                     has no fixed point",
                    task.id
                ),
                witness,
            }
        }
        Some(_) => {}
    }
    let busy = |w: u128| {
        task.jitter
            .checked_add(w)
            .is_none_or(|r| r > task.separation)
    };
    let Some(mut w) = task.computation.checked_add(set.switch) else {
        return busy_period(task, witness);
    };
    witness.sequence.push(w);
    if busy(w) {
        return busy_period(task, witness);
    }
    for _ in 0..ITERATION_BUDGET {
        let Some(following) = next(set, index, w) else {
            return busy_period(task, witness);
        };
        witness.sequence.push(following);
        if following == w {
            let response = task.jitter + w;
            return if response <= task.deadline {
                Outcome::Holds {
                    response,
                    deadline: task.deadline,
                    witness,
                }
            } else {
                Outcome::NotEstablished {
                    why: format!(
                        "task `{}` has response bound {response} against a deadline of {}. The bound is an \
                         envelope, so this shows no miss",
                        task.id, task.deadline
                    ),
                    witness,
                }
            };
        }
        if busy(following) {
            return busy_period(task, witness);
        }
        w = following;
    }
    Outcome::Inconclusive {
        why: format!(
            "the recurrence for task `{}` did not converge within the budget of {ITERATION_BUDGET} iterations, \
             a named resource limit",
            task.id
        ),
        witness,
    }
}

fn busy_period(task: &Resolved, witness: Witness) -> Outcome {
    Outcome::NotEstablished {
        why: format!(
            "the recurrence for task `{}` passed its separation {} with its jitter {}, so more than one job can \
             be in the busy period and the single-job bound does not apply",
            task.id, task.separation, task.jitter
        ),
        witness,
    }
}

/// Every task's outcome, highest priority first.
#[must_use]
pub fn analyze(set: &RuntimeSet) -> Vec<Outcome> {
    (0..set.tasks.len())
        .map(|index| response(set, index))
        .collect()
}

/// The set's conclusion (record §5): `not-established` outranks `analysis-inconclusive`, and only when every task
/// holds do the deadlines hold, in this model and under its assumptions. Never a counterexample.
#[must_use]
pub fn conclusion(set: &RuntimeSet, outcomes: &[Outcome]) -> Conclusion {
    if let Some(Outcome::NotEstablished { why, witness }) = outcomes
        .iter()
        .find(|o| matches!(o, Outcome::NotEstablished { .. }))
    {
        return Conclusion::NotEstablished {
            why: format!("in {MODEL}, {why}; {}", witness.render()),
        };
    }
    if let Some(Outcome::Inconclusive { why, .. }) = outcomes
        .iter()
        .find(|o| matches!(o, Outcome::Inconclusive { .. }))
    {
        return Conclusion::Inconclusive {
            limit: format!("in {MODEL}, {why}"),
        };
    }
    Conclusion::HoldsUnderAssumptions {
        model: MODEL.to_string(),
        assumptions: set.assumptions.clone(),
    }
}
