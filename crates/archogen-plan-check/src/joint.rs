//! Check (5), the joint rules, as `M3.5.5.8`'s fenced context writes it from `docs/semantics/model.md` §8 alone: the
//! rules that refuse an assignment — rules 1 to 3, 5, 7, 9, 10, 13 and 14, with rule 4's node counted once by rule 3,
//! and rule 16 (`M4.1.2.3`, by a fenced context too) — judged over one system's assignment after checks (1) to (4)
//! accepted each binding alone; an engine adapter's instance judged by them as rule 15 and
//! `docs/specs/adapters/decision_engine-adapters.md` §4 say, from the checker's own table ([`crate::adapters`]) under
//! `M3.2.5`'s fence.
//!
//! ⭐ **Everything the rules need is read here, from the description**: who requires (rule 1), the declarations a
//! system's closure reaches and its hardware (rule 7), which providers compose which (rule 5), every region a provider
//! offers (rules 5 and 10) and where a requester writes a fact, for a finding's place. Only the relation at one provider
//! is the reference model's ([`crate::model`]): a side read whole (`read_side`), a provider read into one state per fact
//! (`provider::read`), a fact's outcome there (`outcome`) and one requirement's verdict (`judge`).
//!
//! ⭐ **Two kinds of finding.** A conflict or an unknown the rules find in the plan is the plan's — a plan the search
//! wrote that the rules refuse is a corrupted plan, never a valid system (`ROADMAP.md` §5.5) — so it is [`PLAN_JOINT`]
//! under `tool-failure`, its message naming its rule. What §8 makes the description's own status whatever the
//! assignment — a service or a policy the system serves naming hardware outside it (rule 7), a caller stating levels
//! that do not hold the profile's (rule 9, judged first), a region that does not read (rule 5), rule 10's unknowns
//! (placement memory with no size, a region against it whose overlap no size decides, holes of no size that leave it no
//! free byte), a side or a provider the relation does not read — is [`PLAN_DESCRIPTION`] under that status, and refuses
//! before any finding of the plan's. Rule 5's unread region and rule 10's unknowns are check (2)'s first,
//! description-wide ([`crate::providers`]), so [`check::plan`] never reaches check (5) with one; check (5) judges them
//! again, over the system's hardware, so its own verdict holds where it is called alone. Rule 10's
//! `invalid-description`s, overlaps and memory reserved in every byte, are check (2)'s alone.
//!
//! ⭐ **Rule 7's name is one system's, so it is no refusal of check (2)**, which judges the description for every system
//! and would refuse a plan for a system the name leaves inside its hardware. It is check (5)'s, for the plan's system; and
//! since it holds whatever the assignment, [`check::plan`] asks `names_outside_hardware` first and, where one holds, goes
//! to check (5) past checks (3) and (4), so no finding of the bindings stands before it.
//!
//! ⛔ **The level is the profile's, never the record's**: `Profile::caller_level`, so a corrupted level cannot pass.
//!
//! ⭐ **The system reading is public**, for a comparison test another crate holds: [`system`] gives one system's
//! hardware, the declarations its closure reaches and the requesters it serves as a [`SystemReading`] — computed by the
//! very code the rules judge an assignment with, never a second copy of it. Everything else here stays the crate's.
//!
//! # Where §8's text leaves a choice, the reading taken
//!
//! - Rule 1: a system's own level is every clause of its `defsystem` but its tasks, its `platform` clause among them; a
//!   task is its own clauses; any other declaration is its whole side, as §7 rule 4 reads one.
//! - Rule 1: a binding whose requester names no declaration or no task of its system, or whose fact the vocabulary does
//!   not declare — a `uses` is composition, never a binding — is refused here too, though check (3) refuses it first.
//! - Rule 7: the system serves its own level, its tasks and every declaration reached from any of them — the platform
//!   clause included — through `uses` and `needs`, at any depth and through declarations of every kind; a task of
//!   another system is refused under rule 7, an assignment being one system's.
//! - Rule 7: the positions read for `uses` and `needs` are a side's (§7 rule 4): any depth of any clause but `offers`,
//!   `absent` and `refines` — on a block or platform the kinds admit, the positions rule 7 names, a clause of its own
//!   or inside `requires`. Rule 5's `uses` is read at the same positions.
//! - Rule 7: a service or a policy naming a block or platform outside the hardware is "the same conflict" by what it
//!   names, and the description's own by "judged once for the system whatever the assignment" and the code the rule
//!   states: [`PLAN_DESCRIPTION`] under `infeasible-configuration`, where a binding's conflict is the plan's
//!   [`PLAN_JOINT`]. Once is one finding per namer and name, however often written or however many reach the namer.
//! - Rule 7: its namers are the declarations of kind service and policy the system's closure reaches, at any depth and
//!   through declarations of every kind, as the requesters it serves are read. What the rule's words do not name is not
//!   judged: a block's or a platform's own names are composition, its tasks' and its own level's are its hardware, and a
//!   declaration of another kind, a system another system names among them, is said nothing of.
//! - Rule 3: a provider offering a consumed function `true` with `channels` absent or `0` is the description's
//!   `invalid-description`, refused when it is read — check (2)'s, first. Check (5) meets one only when called past
//!   check (2), and there counts what it offers: `channels` absent or `0` holds no channel for two requesters taking
//!   one, a conflict.
//! - Rule 4: a node bound for a requirement that takes a channel — a service, a policy, or a block or platform bound
//!   for requirements of its own — is the requester of that binding, counted once at its provider by rule 3's count
//!   by requester, however many users reach it; a user bound itself there is a requester of its own (rule 3), never
//!   one counted through the node. Its users and its multiplexer are an obligation, never a finding of this
//!   checker's, as rule 8's route is: rule 4 adds no refusal to rule 3's, which the preamble's "rule 4's node counted
//!   once by rule 3" says. An instance shares as its kind says, rule 15's reading below.
//! - Rules 5 and 10: `size` is a quantity of information — a number and a unit — whole and positive in bytes; a bare
//!   number is no size.
//! - Rule 9: "levels that do not hold the system's" is a stated set without the level; a set holding it and others
//!   passes, the others being the relation's wherever the qualifier binds. It is judged for every caller the system
//!   serves, bound or not.
//! - Rule 9: an empty `caller_level` is no level given, and the rule says nothing; one outside
//!   `reachable-at-privilege`'s domain is the engine's input error, refused as `tool-failure` before anything is judged.
//! - Several verdicts of the description's: `invalid-description` before `unsupported-profile` before
//!   `infeasible-configuration` before `missing-fact`, the precedence rule 6 names as the language orders it
//!   (`eadl_front::Verdict::precedence`), every finding kept.
//! - Rule 13: a function's requirements are every requirement the requester writes on a fact of it, a group by its head.
//! - Rule 13: a qualifier is judged for every requester the system serves that writes one; the requesters it reaches
//!   through `uses` and `needs` are reached at any depth; a qualifier judged past the exact arithmetic is an unknown.
//! - Rule 14: the requesters a deadline's requester reaches are the declarations it names through `needs` and `uses`,
//!   at any depth, itself apart; a requester bound for counter facts at two devices besides the deadline's is a
//!   conflict per device; a provider that is no block or platform is its own device.
//! - Rule 15, rule 7: an instance is served when a requester the system serves is bound to it; its own bindings, of an
//!   instance served by none, are rule 7's conflict, and a binding to an exclusive instance by a requester it does not
//!   name, rule 3's — it offers one channel, its one requester's.
//! - Rule 15, rule 9: an instance is a caller at its kind's level whatever the profile gives, so its device is judged
//!   at that level even where no level is given the system's callers; a caller bound to an instance reads that level
//!   as the instance's `reachable-at-privilege`.
//! - Rule 16: a task "bound itself to a source for `interrupt-source` at `true`" is a task of the plan's system with a
//!   binding of `interrupt-source` whose requirement is met by `true`, as rule 2 reads "met by the value `true`" — for a
//!   task, whose kind admits `needs` and `uses` but no `requires`, `(needs interrupt-source)` alone. A sporadic task is
//!   one writing a `min-separation`, read as a time; one that does not read is check (2)'s.
//! - Rule 16: the comparison is the relation's judgement of `(min-arrival-separation (at-least T))` at the source, `T`
//!   the task's `min-separation`: a value in every spelling written, `(exactly v)` among them, decides it by the first
//!   spelling whose comparison the exact arithmetic decides (§7 rules 2, 3); a source whose spellings of it cannot be
//!   compared with each other is a comparison the arithmetic cannot decide.
//! - Rule 16: its conflicts and unknowns are the assignment's — another binding of the task may hold — as rule 9's
//!   providers are, so a plan holding one is [`PLAN_JOINT`] under `tool-failure`, its message naming the code the rule
//!   gives it, `unsupported-profile` or `missing-fact`. The route rule 8 reports is no finding of this checker's.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use archogen_outcome::record::{Finding, Label, Severity};
use archogen_outcome::Status;
use eadl_front::{Form, Program, SourceMap, Span};
use eadl_model::profile::Profile;
use eadl_model::quantity::{Dimension, Quantity};

use crate::assignment::{read_requester, Assignment, Instance, Provider as Bound, Requester};
use crate::check::{self, Verdict, PLAN_DESCRIPTION, PLAN_JOINT};
use crate::instances;
use crate::model::judge::{judge, outcome, Outcome, Verdict as Judged};
use crate::model::provider::{self, Provider};
use crate::model::requirement::{read_side, NotJudged, Requirement};
use crate::model::value::Value;
use crate::model::vocab::{self, Direction, Domain, Function};
use crate::providers::fills;

/// The qualifier rule 9 judges against the level, and rule 13 never repeats.
const PRIVILEGE: &str = "reachable-at-privilege";

/// The pool of a provider's consumed functions (rule 3).
const CHANNELS: &str = "channels";

/// The consumed fact whose requester rule 14 holds to one time base.
const DEADLINE: &str = "absolute-deadline";

/// The function rule 14 reads a time base from.
const COUNTER: &str = "counter";

/// The fact a task binds itself to the source releasing it through (rule 16).
const SOURCE: &str = "interrupt-source";

/// What a source states of how far apart its arrivals are (rule 16).
const ARRIVALS: &str = "min-arrival-separation";

/// The clause a sporadic task states its own minimum separation in (§4 rule 1).
const SEPARATION: &str = "min-separation";

/// Check (5) of `assignment`, which checks (1)–(4) accepted — or checks (1) and (2) alone, where rule 7's name holds
/// whatever the assignment — over the elaborated `program`, `sources` its files, under `profile`; `name` and `text` the
/// record's, for the spans of its clauses.
pub(crate) fn check(
    sources: &SourceMap,
    program: &Program,
    profile: &Profile,
    name: &str,
    text: &str,
    assignment: &Assignment,
) -> Result<(), Verdict> {
    let record = RecordText::read(name, text);
    let level = profile.caller_level;
    if !level.is_empty() && !levels().contains(&level) {
        return Err(Verdict {
            status: Status::ToolFailure,
            findings: vec![error(
                PLAN_JOINT,
                format!(
                    "the profile `{}` runs its callers at `{level}`, which is no level of `{PRIVILEGE}`'s domain — {}: \
                     the engine's input error, never the plan's or the description's (rule 9)",
                    profile.id,
                    levels().join(", ")
                ),
                record.clause("profile", "the profile the plan is checked under"),
                Vec::new(),
                "report it: the checker's profile table gives a level the vocabulary does not name",
            )],
        });
    }
    let world = World::read(program);
    let Some(system) = world.system(&assignment.system) else {
        return Err(Verdict {
            status: Status::ToolFailure,
            findings: vec![error(
                PLAN_JOINT,
                format!(
                    "the plan is for `{}`, which is no system of the description: an assignment is one system's \
                     (rule 7)",
                    assignment.system
                ),
                record.clause("system", "the system the plan is for"),
                Vec::new(),
                "resolve again: the plan is for a system of another description",
            )],
        });
    };
    let mut judgement = Judgement::new(sources, world, system, assignment, record);
    judgement.requesters();
    judgement.channels();
    judgement.ownership();
    judgement.privilege(profile.id, level);
    judgement.names();
    judgement.sources();
    judgement.placement();
    judgement.functions();
    judgement.qualifiers();
    judgement.time_base();
    judgement.verdict()
}

/// One system as check (5) reads it (model §8 rules 1 and 7): what the rules judge its assignment against, each
/// declaration by its qualified name, as module resolution qualifies it (`eadl_front::Instance::qualify`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SystemReading {
    /// The system's hardware (rule 7): the blocks and platforms its `platform` clause, its own `uses` and `needs` or
    /// its tasks name, and every block or platform those blocks and platforms name in turn through `uses` or `needs`,
    /// at any depth — a declaration of another kind ends the path, so a block only a service names is no hardware.
    pub hardware: BTreeSet<String>,
    /// Every declaration the system's closure reaches through `uses` and `needs` (rule 7), at any depth and through
    /// declarations of every kind.
    pub reached: BTreeSet<String>,
    /// Every requester the system serves (rules 1 and 7): its own level, each of its tasks, each declaration reached.
    pub served: BTreeSet<Requester>,
}

/// The system `name` of the elaborated `program`, read as check (5) reads it before judging an assignment for it:
/// `None` when `name` is no declaration of kind system there — where check (5) refuses a plan for it as for no system.
#[must_use]
pub fn system(program: &Program, name: &str) -> Option<SystemReading> {
    let world = World::read(program);
    let declaration = world.system(name)?;
    let Reading {
        hardware,
        reached,
        served,
    } = Reading::of(&world, name, declaration);
    Some(SystemReading {
        hardware,
        reached,
        served: served.into_iter().collect(),
    })
}

/// Whether a service or a policy the system `name` serves names, by `uses` or `needs`, a block or platform outside its
/// hardware (rule 7): a conflict of that system whatever the assignment, which check (5) reports; `false` for a name of
/// no system.
pub(crate) fn names_outside_hardware(program: &Program, name: &str) -> bool {
    let world = World::read(program);
    world.system(name).is_some_and(|system| {
        let reading = Reading::of(&world, name, system);
        !world
            .named_outside(&reading.hardware, &reading.reached)
            .is_empty()
    })
}

/// One system's reading, as the rules judge with it: [`SystemReading`]'s, the requesters served in the order they are
/// judged — its own level, its tasks as it writes them, then each declaration reached.
struct Reading {
    hardware: BTreeSet<String>,
    reached: BTreeSet<String>,
    served: Vec<Requester>,
}

impl Reading {
    /// The reading of `system`, the declaration of kind system `name` names in `world` (rules 1 and 7).
    fn of<'p>(world: &World<'p>, name: &str, system: Declaration<'p>) -> Self {
        let named = world.names(system.instance, system.form, &["needs", "uses"]);
        let hardware = world.closure(named.clone(), &["needs", "uses"], |d| d.kind.provides());
        let reached = world.closure(named, &["needs", "uses"], |_| true);
        let tasks = system
            .form
            .items()
            .iter()
            .filter(|clause| clause.head() == Some("task"))
            .filter_map(|clause| clause.items().get(1).and_then(Form::as_symbol))
            .map(|task| Requester::Task {
                system: name.to_owned(),
                task: task.to_owned(),
            });
        let served = std::iter::once(Requester::Named(name.to_owned()))
            .chain(tasks)
            .chain(reached.iter().cloned().map(Requester::Named))
            .collect();
        Self {
            hardware,
            reached,
            served,
        }
    }
}

/// The levels of `reachable-at-privilege`'s domain, as the vocabulary declares them.
fn levels() -> &'static [&'static str] {
    match vocab::entry(PRIVILEGE).map(|e| e.domain) {
        Some(Domain::Set(levels)) => levels,
        _ => &[],
    }
}

/// An error finding.
fn error(
    code: &str,
    message: String,
    primary: Label,
    secondary: Vec<Label>,
    repair: &str,
) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: Severity::Error,
        message,
        primary,
        secondary,
        repair: repair.to_owned(),
    }
}

/// What kind of declaration a name is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Kind {
    Block,
    Platform,
    Service,
    Policy,
    System,
    Other,
}

impl Kind {
    fn of(form: &Form) -> Self {
        match form.head() {
            Some("defblock") => Self::Block,
            Some("defplatform") => Self::Platform,
            Some("defservice") => Self::Service,
            Some("defpolicy") => Self::Policy,
            Some("defsystem") => Self::System,
            _ => Self::Other,
        }
    }

    /// A block or a platform: what `/1` binds a requirement to.
    const fn provides(self) -> bool {
        matches!(self, Self::Block | Self::Platform)
    }

    const fn word(self) -> &'static str {
        match self {
            Self::Block => "block",
            Self::Platform => "platform",
            Self::Service => "service",
            Self::Policy => "policy",
            Self::System => "system",
            Self::Other => "declaration",
        }
    }
}

/// One declaration of the program: the instance that writes it, its form and its kind.
#[derive(Debug, Clone, Copy)]
struct Declaration<'p> {
    instance: usize,
    form: &'p Form,
    kind: Kind,
}

/// A service or a policy naming a block or platform outside a system's hardware (rule 7).
#[derive(Debug, Clone)]
struct NamedOutside {
    namer: String,
    namer_kind: Kind,
    hardware: String,
    hardware_kind: Kind,
    /// Where the namer first writes the name.
    span: Span,
}

/// The program's declarations by qualified name, and the names each writes, resolved as module resolution resolves
/// them.
struct World<'p> {
    program: &'p Program,
    declarations: BTreeMap<String, Declaration<'p>>,
}

impl<'p> World<'p> {
    fn read(program: &'p Program) -> Self {
        let mut declarations = BTreeMap::new();
        for (index, instance) in program.instances.iter().enumerate() {
            for form in &instance.declarations {
                let Some(local) = form.items().get(1).and_then(Form::as_symbol) else {
                    continue;
                };
                declarations
                    .entry(instance.qualify(local))
                    .or_insert(Declaration {
                        instance: index,
                        form,
                        kind: Kind::of(form),
                    });
            }
        }
        Self {
            program,
            declarations,
        }
    }

    fn get(&self, name: &str) -> Option<Declaration<'p>> {
        self.declarations.get(name).copied()
    }

    /// The declaration `name` names, when it is of kind system: what an assignment is for (rule 7).
    fn system(&self, name: &str) -> Option<Declaration<'p>> {
        self.get(name).filter(|d| d.kind == Kind::System)
    }

    fn provides(&self, name: &str) -> bool {
        self.get(name).is_some_and(|d| d.kind.provides())
    }

    /// The declarations `form`, written in instance `instance`, names through the lists `heads` selects — `needs`,
    /// `uses` or both — each by its qualified name.
    fn names(&self, instance: usize, form: &Form, heads: &[&str]) -> Vec<String> {
        self.named_at(instance, form, heads)
            .into_iter()
            .map(|(qualified, _)| qualified)
            .collect()
    }

    /// What [`Self::names`] reads, each with where it is written. A name that resolves to no declaration is a fact's, or
    /// nothing's.
    fn named_at(&self, instance: usize, form: &Form, heads: &[&str]) -> Vec<(String, Span)> {
        let mut operands = Vec::new();
        operands_of(form, heads, &mut operands);
        operands
            .into_iter()
            .filter_map(|operand| match operand {
                Form::Symbol { name, span } => self
                    .program
                    .resolve(instance, name, *span)
                    .ok()
                    .flatten()
                    .map(|qualified| (qualified, *span)),
                _ => None,
            })
            .filter(|(qualified, _)| self.declarations.contains_key(qualified))
            .collect()
    }

    /// Rule 7: each service or policy of `reached` naming, by `uses` or `needs`, a block or platform outside `hardware` —
    /// once for a namer and a name, at the place it is first written, namers in order of name.
    fn named_outside(
        &self,
        hardware: &BTreeSet<String>,
        reached: &BTreeSet<String>,
    ) -> Vec<NamedOutside> {
        let mut found = Vec::new();
        for namer in reached {
            let Some(declaration) = self.get(namer) else {
                continue;
            };
            if !matches!(declaration.kind, Kind::Service | Kind::Policy) {
                continue;
            }
            let mut said = BTreeSet::new();
            for (name, span) in
                self.named_at(declaration.instance, declaration.form, &["needs", "uses"])
            {
                let Some(hardware_kind) = self.get(&name).map(|d| d.kind) else {
                    continue;
                };
                if hardware_kind.provides()
                    && !hardware.contains(&name)
                    && said.insert(name.clone())
                {
                    found.push(NamedOutside {
                        namer: namer.clone(),
                        namer_kind: declaration.kind,
                        hardware: name,
                        hardware_kind,
                        span,
                    });
                }
            }
        }
        found
    }

    /// The declarations reached from `seeds` that `keep` admits, each followed through what it names in `heads`, at
    /// any depth.
    fn closure(
        &self,
        seeds: Vec<String>,
        heads: &[&str],
        keep: impl Fn(Declaration<'p>) -> bool,
    ) -> BTreeSet<String> {
        let mut reached = BTreeSet::new();
        let mut queue: VecDeque<String> = seeds.into();
        while let Some(name) = queue.pop_front() {
            let Some(declaration) = self.get(&name) else {
                continue;
            };
            if !keep(declaration) || !reached.insert(name) {
                continue;
            }
            queue.extend(self.names(declaration.instance, declaration.form, heads));
        }
        reached
    }

    /// The form `requester` writes what it requires in, and the instance its names resolve in: a system's own level
    /// without its tasks, a task's own clauses, any other declaration whole (rule 1). `None` when it names no
    /// declaration, or no task of a system.
    fn side(&self, requester: &Requester) -> Option<(Form, usize)> {
        match requester {
            Requester::Named(name) => {
                let declaration = self.get(name)?;
                let form = if declaration.kind == Kind::System {
                    clauses(declaration.form, |clause| clause.head() == Some("task"))
                } else {
                    declaration.form.clone()
                };
                Some((form, declaration.instance))
            }
            Requester::Task { system, task } => {
                let declaration = self.get(system).filter(|d| d.kind == Kind::System)?;
                let form = declaration.form.items().iter().find(|clause| {
                    clause.head() == Some("task")
                        && clause.items().get(1).and_then(Form::as_symbol) == Some(task.as_str())
                })?;
                Some((clauses(form, |_| false), declaration.instance))
            }
            // An instance writes nothing in the description: what it requires is its kind's.
            Requester::Instance(_) => None,
        }
    }
}

/// `form` as a side of its own: its head and its clauses, its name and every clause `drop` selects left out — a name is
/// judged where its declaration is, never again here.
fn clauses(form: &Form, drop: impl Fn(&Form) -> bool) -> Form {
    let items = form.items();
    Form::List {
        items: items
            .iter()
            .take(1)
            .chain(items.iter().skip(2).filter(|clause| !drop(clause)))
            .cloned()
            .collect(),
        span: form.span(),
    }
}

/// Every operand of the `needs` and `uses` lists `heads` selects, written at a side's positions in `form`: at any depth
/// of any clause but `offers`, `absent` and `refines` (§7 rule 4), where a `requires` nests.
fn operands_of<'f>(form: &'f Form, heads: &[&str], out: &mut Vec<&'f Form>) {
    for item in form.items().iter().skip(1) {
        match item.head() {
            Some("offers" | "absent" | "refines") => {}
            Some(head @ ("needs" | "uses")) => {
                if heads.contains(&head) {
                    out.extend(item.items().iter().skip(1));
                }
            }
            _ => operands_of(item, heads, out),
        }
    }
}

/// Where `form` writes a requirement on `fact`: its name inside a `needs`, or a list it heads, at a side's positions.
fn site(form: &Form, fact: &str) -> Option<Span> {
    for item in form.items().iter().skip(1) {
        match item.head() {
            Some("offers" | "absent" | "refines") => {}
            Some("needs") => {
                if let Some(operand) = item
                    .items()
                    .iter()
                    .skip(1)
                    .find(|operand| operand.as_symbol() == Some(fact))
                {
                    return Some(operand.span());
                }
            }
            Some(head) if head == fact => return Some(item.span()),
            _ => {
                if let Some(span) = site(item, fact) {
                    return Some(span);
                }
            }
        }
    }
    None
}

/// The fact a requirement is on: a group's head for a group.
fn fact_of(requirement: &Requirement) -> &str {
    match requirement {
        Requirement::Presence(fact)
        | Requirement::Constraint { fact, .. }
        | Requirement::Group { fact, .. }
        | Requirement::Statement(fact, _) => fact,
    }
}

/// The function the vocabulary says `fact` describes.
fn function_of(fact: &str) -> Option<Function> {
    vocab::entry(fact).and_then(|e| e.function)
}

/// Whether a requirement on `fact` is met by the value `true` — `(needs f)`, `(f true)` or the group whose head it is:
/// on a consumed fact, a binding taking a channel (rule 2); on `interrupt-source`, a task bound itself to the source
/// releasing it (rule 16). `(f false)` is neither.
fn at_true(requirements: &[Requirement], fact: &str) -> bool {
    requirements.iter().any(|r| match r {
        Requirement::Presence(f) | Requirement::Group { fact: f, .. } => f == fact,
        Requirement::Constraint {
            fact: f,
            value: Value::Bool(true),
            ..
        } => f == fact,
        _ => false,
    })
}

/// A region a provider offers: `(region NAME (base B) (size S) (executable …) (reserved …))` (rule 5).
#[derive(Debug, Clone)]
struct Region {
    provider: String,
    name: String,
    base: i128,
    /// One past its last byte, where it has a size.
    end: Option<i128>,
    executable: bool,
    reserved: bool,
    span: Span,
}

impl Region {
    /// Placement memory: executable and not reserved (rule 10), owned by none.
    const fn placement(&self) -> bool {
        self.executable && !self.reserved
    }

    /// The bytes known to be its own, `[lo, hi)`: a region without size owns at least its base byte.
    fn known(&self) -> (i128, i128) {
        (self.base, self.end.unwrap_or(self.base + 1))
    }

    fn range(&self) -> String {
        match self.end {
            Some(end) => format!("[{:#x}, {end:#x})", self.base),
            None => format!("[{:#x}, …) with no size", self.base),
        }
    }

    fn describe(&self) -> String {
        format!(
            "`{}`'s region `{}` {}",
            self.provider,
            self.name,
            self.range()
        )
    }
}

/// How two regions' bytes meet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Overlap {
    None,
    Bytes,
    /// One has no size, and the other has bytes above its base.
    Unknown,
}

fn overlap(a: &Region, b: &Region) -> Overlap {
    let ((a_lo, a_hi), (b_lo, b_hi)) = (a.known(), b.known());
    if a_lo < b_hi && b_lo < a_hi {
        return Overlap::Bytes;
    }
    let above = |open: &Region, other_hi: i128| open.end.is_none() && other_hi - 1 > open.base;
    if above(a, b_hi) || above(b, a_hi) {
        Overlap::Unknown
    } else {
        Overlap::None
    }
}

/// Why a region does not read: rule 5's codes.
#[derive(Debug, Clone)]
enum Unread {
    Invalid(String),
    Unsupported(String),
}

/// Every region `form`'s `offers` write, read or refused.
fn regions_of(provider: &str, form: &Form) -> Vec<Result<Region, (Span, Unread)>> {
    form.items()
        .iter()
        .filter(|clause| clause.head() == Some("offers"))
        .flat_map(|clause| clause.items().iter().skip(1))
        .filter(|item| item.head() == Some("region") || item.as_symbol() == Some("region"))
        .map(|item| read_region(provider, item).map_err(|why| (item.span(), why)))
        .collect()
}

/// One region, read whole (rule 5): a name, one non-negative `base`, at most one `size` that is a positive whole number of
/// bytes, and at most one each of `executable` and `reserved`, `true` or `false`.
fn read_region(provider: &str, item: &Form) -> Result<Region, Unread> {
    let invalid = |why: &str| Unread::Invalid(why.to_owned());
    let Some(name) = item.items().get(1).and_then(Form::as_symbol) else {
        return Err(invalid(
            "a region names itself and its bytes: `(region NAME (base B) (size S))`",
        ));
    };
    let (mut base, mut size, mut executable, mut reserved) = (None, None, None, None);
    for sub in item.items().iter().skip(2) {
        let operands = sub.items().get(1..).unwrap_or(&[]);
        let fresh = match sub.head() {
            Some("base") => match operands {
                [Form::Integer { value, .. }] if *value >= 0 => {
                    base.replace(i128::from(*value)).is_none()
                }
                _ => return Err(invalid("a region's base is one non-negative address")),
            },
            Some("size") => match operands {
                [number, unit] => size.replace(bytes(number, unit, sub.span())?).is_none(),
                _ => {
                    return Err(invalid(
                        "a region's size is one amount of information, a number and a unit",
                    ))
                }
            },
            Some(flag @ ("executable" | "reserved")) => {
                let value = match operands {
                    [Form::Symbol { name, .. }] if name == "true" => true,
                    [Form::Symbol { name, .. }] if name == "false" => false,
                    _ => return Err(invalid("`executable` and `reserved` are `true` or `false`")),
                };
                if flag == "executable" {
                    executable.replace(value).is_none()
                } else {
                    reserved.replace(value).is_none()
                }
            }
            _ => {
                return Err(invalid(
                    "a region's sub-forms are `base`, `size`, `executable` and `reserved`",
                ))
            }
        };
        if !fresh {
            return Err(invalid("a region writes each sub-form at most once"));
        }
    }
    let Some(base) = base else {
        return Err(invalid("a region without a base names no byte"));
    };
    let end = match size {
        Some(size) => Some(base.checked_add(size).ok_or_else(|| {
            Unread::Unsupported("its last byte is past the exact arithmetic".to_owned())
        })?),
        None => None,
    };
    Ok(Region {
        provider: provider.to_owned(),
        name: name.to_owned(),
        base,
        end,
        executable: executable.unwrap_or(false),
        reserved: reserved.unwrap_or(false),
        span: item.span(),
    })
}

/// A region's size in bytes: a quantity of information, positive and whole in bytes.
fn bytes(number: &Form, unit: &Form, within: Span) -> Result<i128, Unread> {
    let quantity = Quantity::read(Some(number), Some(unit), within).map_err(|d| {
        let why = format!("its size: {}", d.message);
        if d.code == "quantity-overflow" {
            Unread::Unsupported(why)
        } else {
            Unread::Invalid(why)
        }
    })?;
    if quantity.unit.dimension != Dimension::Information {
        return Err(Unread::Invalid(
            "a region's size is an amount of information, in bytes".to_owned(),
        ));
    }
    let bits = quantity.in_base().map_err(|_| {
        Unread::Unsupported("its size in bits is past the exact arithmetic".to_owned())
    })?;
    if !bits.is_integer() || !bits.is_positive() || bits.numerator() % 8 != 0 {
        return Err(Unread::Invalid(
            "a region's size is a positive whole number of bytes".to_owned(),
        ));
    }
    Ok(bits.numerator() / 8)
}

/// The record's text, and where each `bind` clause stands in it.
struct RecordText<'a> {
    name: &'a str,
    text: &'a str,
    binds: BTreeMap<(String, String), (u32, u32)>,
}

impl<'a> RecordText<'a> {
    fn read(name: &'a str, text: &'a str) -> Self {
        let mut binds = BTreeMap::new();
        let mut sources = SourceMap::new();
        if let Ok(id) = sources.add(name, text) {
            let (document, _) = eadl_front::read(&sources, id);
            for form in document.forms.iter().filter(|f| f.head() == Some("bind")) {
                if let (Some(source), [_, requester, Form::Symbol { name: fact, .. }, _]) =
                    (sources.get(id), form.items())
                {
                    let at = source.position(form.span().start);
                    let requester = read_requester(requester)
                        .map_or_else(|_| requester.to_canonical(), |r| r.canonical());
                    binds
                        .entry((requester, fact.clone()))
                        .or_insert((at.line, at.column));
                }
            }
        }
        Self { name, text, binds }
    }

    /// The `bind` clause of `requester` and `fact`, else the record's first `bind`.
    fn bind(&self, requester: &Requester, fact: &str, message: String) -> Label {
        match self.binds.get(&(requester.canonical(), fact.to_owned())) {
            Some(&(line, column)) => Label {
                source: self.name.to_owned(),
                line,
                column,
                message,
            },
            None => Label {
                message,
                ..check::label(self.name, self.text, "bind")
            },
        }
    }

    /// The clause `head` opens.
    fn clause(&self, head: &str, message: &str) -> Label {
        Label {
            message: message.to_owned(),
            ..check::label(self.name, self.text, head)
        }
    }
}

/// One system's assignment under judgement: what the description gives it, and every finding so far.
struct Judgement<'a, 'p> {
    sources: &'a SourceMap,
    world: World<'p>,
    system: Declaration<'p>,
    assignment: &'a Assignment,
    record: RecordText<'a>,
    /// The blocks and platforms of the system's hardware (rule 7).
    hardware: BTreeSet<String>,
    /// Every declaration the system's closure reaches through `uses` and `needs` (rule 7).
    reached: BTreeSet<String>,
    /// Every requester the system serves (rule 7), in the order judged: its own level, its tasks, each declaration
    /// reached.
    served: Vec<Requester>,
    requirements: BTreeMap<Requester, Option<Vec<Requirement>>>,
    providers: BTreeMap<String, Option<Provider>>,
    regions: BTreeMap<String, Vec<Region>>,
    /// The description's own verdicts, each with its status.
    description: Vec<(Status, Finding)>,
    /// The plan's conflicts and unknowns.
    plan: Vec<Finding>,
}

impl<'a, 'p> Judgement<'a, 'p> {
    fn new(
        sources: &'a SourceMap,
        world: World<'p>,
        system: Declaration<'p>,
        assignment: &'a Assignment,
        record: RecordText<'a>,
    ) -> Self {
        let Reading {
            hardware,
            reached,
            served,
        } = Reading::of(&world, &assignment.system, system);
        Self {
            sources,
            world,
            system,
            assignment,
            record,
            hardware,
            reached,
            served,
            requirements: BTreeMap::new(),
            providers: BTreeMap::new(),
            regions: BTreeMap::new(),
            description: Vec::new(),
            plan: Vec::new(),
        }
    }

    fn at(&self, span: Span, message: &str) -> Label {
        check::at(self.sources, span, message)
    }

    /// Where `name` is declared.
    fn declared(&self, name: &str) -> Option<Label> {
        let d = self.world.get(name)?;
        Some(self.at(d.form.span(), &format!("`{name}`, a {}", d.kind.word())))
    }

    /// Where a bound provider is declared: a block or platform itself, an instance's device.
    fn declared_bound(&self, provider: &Bound) -> Option<Label> {
        match provider {
            Bound::Named(name) => self.declared(name),
            Bound::Instance(instance) => {
                let d = self.world.get(&instance.over)?;
                Some(self.at(
                    d.form.span(),
                    &format!(
                        "`{}`, a {}, the device `{}` programs",
                        instance.over,
                        d.kind.word(),
                        instance.canonical()
                    ),
                ))
            }
        }
    }

    /// Where `requester` requires `fact`, else where it is declared; an instance's requirement beneath, at its device.
    fn requires(&self, requester: &Requester, fact: &str) -> Option<Label> {
        if let Requester::Instance(instance) = requester {
            let d = self.world.get(&instance.over)?;
            return Some(self.at(
                d.form.span(),
                &format!(
                    "`{}` requires `{fact}` beneath of `{}`",
                    instance.canonical(),
                    instance.over
                ),
            ));
        }
        let (form, _) = self.world.side(requester)?;
        let span = site(&form, fact).unwrap_or_else(|| form.span());
        Some(self.at(
            span,
            &format!("`{}` requires `{fact}` here", requester.canonical()),
        ))
    }

    /// A binding's place in the record.
    fn bound(&self, requester: &Requester, fact: &str) -> Label {
        self.record.bind(
            requester,
            fact,
            format!("the binding of `{}`'s `{fact}`", requester.canonical()),
        )
    }

    /// The record's first binding to `provider`.
    fn first_bound_to(&self, provider: &Bound) -> Label {
        match self
            .assignment
            .bindings
            .iter()
            .find(|(_, p)| *p == provider)
        {
            Some(((requester, fact), _)) => self.bound(requester, fact),
            None => self.record.clause("bind", "the record's bindings"),
        }
    }

    fn conflict(&mut self, message: String, primary: Label, secondary: Vec<Label>, repair: &str) {
        self.plan
            .push(error(PLAN_JOINT, message, primary, secondary, repair));
    }

    fn refuse_description(
        &mut self,
        status: Status,
        message: String,
        primary: Label,
        secondary: Vec<Label>,
        repair: &str,
    ) {
        self.description.push((
            status,
            error(PLAN_DESCRIPTION, message, primary, secondary, repair),
        ));
    }

    /// What `requester` requires, read whole by the relation; nothing when the relation refuses it, which is the
    /// description's verdict. An instance requires what its kind does beneath, read as any side is (rule 1).
    fn requirements(&mut self, requester: &Requester) -> Vec<Requirement> {
        if let Requester::Instance(instance) = requester {
            return instances::kind(instance)
                .and_then(|kind| instances::beneath(kind).ok())
                .unwrap_or_default();
        }
        if let Some(read) = self.requirements.get(requester) {
            return read.clone().unwrap_or_default();
        }
        let read = match self.world.side(requester) {
            None => None,
            Some((form, _)) => match read_side(&form) {
                Ok(requirements) => Some(requirements),
                Err(refusal) => {
                    let (status, why) = match refusal {
                        NotJudged::Invalid(why) => (Status::InvalidDescription, why),
                        NotJudged::Unsupported(why) => (Status::UnsupportedProfile, why),
                        NotJudged::NotAFact => (Status::InvalidDescription, String::new()),
                    };
                    let primary = self.at(form.span(), "the requester");
                    self.refuse_description(
                        status,
                        format!(
                            "`{}` requires what the relation does not read: {why} (rule 1)",
                            requester.canonical()
                        ),
                        primary,
                        Vec::new(),
                        "repair the description, then resolve again",
                    );
                    None
                }
            },
        };
        self.requirements.insert(requester.clone(), read.clone());
        read.unwrap_or_default()
    }

    /// A bound provider read into one state per fact: a block or platform as the relation reads it, an instance as
    /// its kind offers over its device (rule 15); `None` when it is neither, or the relation refuses the block, which is
    /// the description's verdict.
    fn provider(&mut self, provider: &Bound) -> Option<Provider> {
        let key = provider.canonical();
        if let Some(read) = self.providers.get(&key) {
            return read.clone();
        }
        let read = match provider {
            Bound::Named(name) => match self.world.get(name).filter(|d| d.kind.provides()) {
                None => None,
                Some(declaration) => match provider::read(declaration.form) {
                    Ok(provider) => Some(provider),
                    Err(refused) => {
                        let primary = self.at(declaration.form.span(), "the provider");
                        self.refuse_description(
                            Status::InvalidDescription,
                            format!(
                                "`{name}` offers what the relation does not read: `{}`, {}",
                                refused.fact, refused.why
                            ),
                            primary,
                            Vec::new(),
                            "repair the description, then resolve again",
                        );
                        None
                    }
                },
            },
            Bound::Instance(instance) => {
                let over = self.provider(&Bound::Named(instance.over.clone()));
                match (over, instances::kind(instance)) {
                    (Some(over), Some(kind)) => {
                        let requester = instance
                            .serves
                            .as_deref()
                            .map(|r| self.requirements(r))
                            .unwrap_or_default();
                        instances::offered(kind, &key, &over, &requester).ok()
                    }
                    _ => None,
                }
            }
        };
        self.providers.insert(key, read.clone());
        read
    }

    /// The regions the block or platform `name` offers; one that does not read is the description's verdict (rule 5).
    fn regions(&mut self, name: &str) -> Vec<Region> {
        if let Some(read) = self.regions.get(name) {
            return read.clone();
        }
        let mut regions = Vec::new();
        if let Some(declaration) = self.world.get(name).filter(|d| d.kind.provides()) {
            for read in regions_of(name, declaration.form) {
                match read {
                    Ok(region) => regions.push(region),
                    Err((span, why)) => {
                        let (status, why) = match why {
                            Unread::Invalid(why) => (Status::InvalidDescription, why),
                            Unread::Unsupported(why) => (Status::UnsupportedProfile, why),
                        };
                        let primary = self.at(span, "the region");
                        self.refuse_description(
                            status,
                            format!("a region `{name}` offers does not read whole: {why} (rule 5)"),
                            primary,
                            Vec::new(),
                            "write the region as `(region NAME (base B) (size S))`, with `(executable …)` and \
                             `(reserved …)` at most once each",
                        );
                    }
                }
            }
        }
        self.regions.insert(name.to_owned(), regions.clone());
        regions
    }

    /// Whether `requester` is a caller of the system at its level (rule 9): its own level, one of its tasks, or a service
    /// or policy its closure reaches. An instance is a caller at its kind's level, judged on its own.
    fn calls(&self, requester: &Requester) -> bool {
        match requester {
            Requester::Task { system, .. } => *system == self.assignment.system,
            Requester::Named(name) => {
                *name == self.assignment.system
                    || (self.reached.contains(name)
                        && self
                            .world
                            .get(name)
                            .is_some_and(|d| matches!(d.kind, Kind::Service | Kind::Policy)))
            }
            Requester::Instance(_) => false,
        }
    }

    /// Every requester the system serves (rule 7): its own level, each of its tasks, each declaration it reaches — as
    /// [`Reading::of`] read them.
    fn served(&self) -> Vec<Requester> {
        self.served.clone()
    }

    /// The instances the system serves (rule 7, as rule 15 amends it): each one a requester it serves is bound to.
    fn instances(&self) -> BTreeSet<Instance> {
        self.assignment
            .bindings
            .iter()
            .filter_map(|((requester, _), provider)| match provider {
                Bound::Instance(instance) if self.served.contains(requester) => {
                    Some(instance.clone())
                }
                _ => None,
            })
            .collect()
    }

    /// Every declaration `requester` reaches through `needs` and `uses`, at any depth.
    fn reaches(&self, requester: &Requester) -> BTreeSet<String> {
        self.world
            .side(requester)
            .map_or_else(BTreeSet::new, |(form, instance)| {
                let named = self.world.names(instance, &form, &["needs", "uses"]);
                self.world.closure(named, &["needs", "uses"], |_| true)
            })
    }

    /// Rule 7: a service or a policy the system serves naming, by `uses` or `needs`, a block or platform outside its
    /// hardware is no path into it, and a conflict of the system whatever the assignment: the description's
    /// `infeasible-configuration`, at the name, naming the namer, the hardware and the system.
    fn names(&mut self) {
        let system = self.assignment.system.clone();
        for named in self.world.named_outside(&self.hardware, &self.reached) {
            let NamedOutside {
                namer,
                namer_kind,
                hardware,
                hardware_kind,
                span,
            } = named;
            let primary = self.at(span, &format!("`{namer}` names `{hardware}` here"));
            let secondary = [
                self.declared(&hardware),
                Some(self.at(self.system.form.span(), &format!("the system `{system}`"))),
            ]
            .into_iter()
            .flatten()
            .collect();
            self.refuse_description(
                Status::InfeasibleConfiguration,
                format!(
                    "`{namer}`, a {} `{system}` serves, names `{hardware}`, a {} outside the hardware of `{system}`: a \
                     service's or a policy's name is no path into a system's hardware, and the conflict is `{system}`'s \
                     whatever the assignment (rule 7)",
                    namer_kind.word(),
                    hardware_kind.word()
                ),
                primary,
                secondary,
                &format!(
                    "compose `{hardware}` into the platform of `{system}`, or name in `{namer}` hardware the platform \
                     composes"
                ),
            );
        }
    }

    /// Rules 1 and 7: each binding is a requester's the system serves, of a vocabulary fact, to a block or platform of
    /// the system's hardware, or to an instance programming one; an instance's own binding is of what its kind requires
    /// beneath, to the device it programs.
    fn requesters(&mut self) {
        let assignment = self.assignment;
        let system = self.assignment.system.clone();
        let instances = self.instances();
        for ((requester, fact), provider) in &assignment.bindings {
            let here = self.bound(requester, fact);
            let who = requester.canonical();
            let refusal = match requester {
                Requester::Task { system: of, .. } if *of != system => Some(format!(
                    "`{who}` is a task of `{of}`, and the plan is `{system}`'s: an assignment is one system's \
                     (rule 7)"
                )),
                Requester::Task { task, .. } if self.world.side(requester).is_none() => Some(format!(
                    "`{system}` has no task `{task}`: a requester is a task, a service or another declaration \
                     (rule 1)"
                )),
                Requester::Named(name) if self.world.get(name).is_none() => Some(format!(
                    "`{name}` names no declaration of the description: a requester is a task, a service or another \
                     declaration (rule 1)"
                )),
                Requester::Named(name) if *name != system && !self.reached.contains(name) => Some(format!(
                    "`{name}` is no requester `{system}` serves: neither its own level, one of its tasks, nor a \
                     declaration its closure reaches through `uses` and `needs` (rule 7)"
                )),
                Requester::Instance(instance) => self.instance_requester(instance, fact, provider, &instances),
                _ => None,
            };
            if let Some(message) = refusal {
                self.conflict(
                    message,
                    here.clone(),
                    Vec::new(),
                    "resolve again: an assignment binds the requesters of one system",
                );
            }
            if vocab::entry(fact).is_none() {
                self.conflict(
                    format!(
                        "`{who}` is bound for `{fact}`, which is no fact of the vocabulary: what a `uses` names is \
                         composed, never bound (rule 1)"
                    ),
                    here.clone(),
                    Vec::new(),
                    "resolve again: a binding is of a requirement on a vocabulary fact",
                );
            }
            let (device, via) = match provider {
                Bound::Named(name) => (name, String::new()),
                Bound::Instance(instance) => {
                    if let Some(message) = through(instance, requester) {
                        self.conflict(
                            format!("`{who}` is bound for `{fact}` to `{}`, {message}", instance.canonical()),
                            here.clone(),
                            self.declared_bound(provider).into_iter().collect(),
                            "resolve again: an exclusive instance is its one requester's own, and a shared one \
                             serves every requester bound to it",
                        );
                    }
                    (
                        &instance.over,
                        format!(", the device `{}` programs", instance.canonical()),
                    )
                }
            };
            if !self.world.provides(device) {
                self.conflict(
                    format!(
                        "`{who}` is bound for `{fact}` to `{device}`{via}, which is no block or platform of the \
                         description (rule 7)"
                    ),
                    here,
                    Vec::new(),
                    "resolve again: a provider is a block or platform of the system's hardware",
                );
            } else if !self.hardware.contains(device) {
                let secondary = [
                    self.declared(device),
                    self.requires(requester, fact),
                    Some(self.at(self.system.form.span(), &format!("the system `{system}`"))),
                ]
                .into_iter()
                .flatten()
                .collect();
                self.conflict(
                    format!(
                        "`{who}` is bound for `{fact}` to `{device}`{via}, outside the hardware of `{system}`: a \
                         provider on a platform the system does not use never satisfies (rule 7)"
                    ),
                    here,
                    secondary,
                    "resolve again: bind it to a block or platform of the system's hardware — one its `platform` \
                     clause, its own `uses` and `needs` or its tasks name, or one those compose",
                );
            }
        }
    }

    /// Rules 1 and 7 for an instance's own binding of `fact` to `provider`: an instance the system serves, of a fact
    /// its kind requires beneath, to the device it programs; else why not.
    fn instance_requester(
        &self,
        instance: &Instance,
        fact: &str,
        provider: &Bound,
        served: &BTreeSet<Instance>,
    ) -> Option<String> {
        let who = instance.canonical();
        let system = &self.assignment.system;
        let Some(kind) = instances::kind(instance) else {
            return Some(format!(
                "`{who}` names the adapter kind `{}`, which this checker's table does not hold (rule 1)",
                instance.kind
            ));
        };
        if !served.contains(instance) {
            return Some(format!(
                "`{who}` is no requester `{system}` serves: no requester it serves is bound to it, and an instance \
                 takes the system of the requesters bound to it (rule 7)"
            ));
        }
        if !kind.beneath_names().contains(&fact) {
            return Some(format!(
                "`{who}` is bound for `{fact}`, which its kind does not require beneath: an instance is a requester for \
                 what its kind requires beneath (rule 1)"
            ));
        }
        if *provider != Bound::Named(instance.over.clone()) {
            return Some(format!(
                "`{who}` is bound for `{fact}` to `{provider}`, and an instance binds what its kind requires beneath to \
                 the device it programs, `{}` (rule 1)",
                instance.over
            ));
        }
        None
    }

    /// Rules 2 to 4: the requesters taking channels at one provider number at most its `channels` — each counted by its
    /// own bindings, so a node once however many users reach it; an instance among them, through its binding beneath,
    /// and an exclusive instance offering its one.
    fn channels(&mut self) {
        let assignment = self.assignment;
        let mut taking: BTreeMap<String, (Bound, BTreeMap<Requester, Vec<String>>)> =
            BTreeMap::new();
        for ((requester, fact), provider) in &assignment.bindings {
            if !vocab::entry(fact).is_some_and(|e| e.consumed) {
                continue;
            }
            if at_true(&self.requirements(requester), fact) {
                taking
                    .entry(provider.canonical())
                    .or_insert_with(|| (provider.clone(), BTreeMap::new()))
                    .1
                    .entry(requester.clone())
                    .or_default()
                    .push(fact.clone());
            }
        }
        for (name, (provider, requesters)) in taking {
            // One requester needs no count.
            if requesters.len() < 2 {
                continue;
            }
            let Some(read) = self.provider(&provider) else {
                continue;
            };
            let count = requesters.len();
            let (conflict, why) = match outcome(&read, CHANNELS) {
                Ok(Outcome::Valued(values)) => match values.first() {
                    Some(Value::Count(channels))
                        if i128::try_from(count).is_ok_and(|n| n > *channels) =>
                    {
                        (true, format!("offers {channels} `channels`"))
                    }
                    _ => continue,
                },
                Ok(Outcome::Derived(_)) => continue,
                Ok(Outcome::Absent) => (true, "declares `channels` absent".to_owned()),
                Ok(Outcome::Unknown) => (false, "offers `channels` without a value".to_owned()),
                Ok(Outcome::Undescribed) => (false, "does not describe `channels`".to_owned()),
                Err(_) => (
                    false,
                    "states `channels` past the exact arithmetic".to_owned(),
                ),
            };
            let named: Vec<String> = requesters
                .iter()
                .map(|(r, facts)| format!("`{}` through `{}`", r.canonical(), facts.join("`, `")))
                .collect();
            let (first, facts) = requesters.iter().next().expect("two requesters");
            let primary = self.bound(first, &facts[0]);
            let mut secondary: Vec<Label> = requesters
                .iter()
                .filter_map(|(r, facts)| self.requires(r, &facts[0]))
                .collect();
            secondary.extend(self.declared_bound(&provider));
            if conflict {
                self.conflict(
                    format!(
                        "`{name}` gives {count} requesters a channel — {} — and {why}: more requesters than \
                         channels (rule 3)",
                        named.join(", ")
                    ),
                    primary,
                    secondary,
                    "resolve again: bind one of them to another provider, or reach it through a service several \
                     users share, one requester of one channel (rule 4)",
                );
            } else {
                self.conflict(
                    format!(
                        "`{name}` gives {count} requesters a channel — {} — and {why}, so whether they fit is \
                         unknown: `missing-fact`, which blocks the assignment (rule 3)",
                        named.join(", ")
                    ),
                    primary,
                    secondary,
                    "describe `channels` at the provider with a count, or resolve again binding one of them to \
                     another provider",
                );
            }
        }
    }

    /// Rule 5: two distinct bound providers, neither composing the other, own no byte in common — one composing another
    /// that it names through `uses`, at any depth through blocks and platforms, whose `uses` alone is composition; a
    /// service's `uses` is a binding and composes nothing. An instance owns no region and composes nothing: its device,
    /// bound by its binding beneath, owns its own.
    fn ownership(&mut self) {
        let bound: BTreeSet<String> = self
            .assignment
            .bindings
            .values()
            .filter_map(|p| match p {
                Bound::Named(name) if self.world.provides(name) => Some(name.clone()),
                _ => None,
            })
            .collect();
        let mut owned: Vec<(String, Vec<Region>, BTreeSet<String>)> = Vec::new();
        for provider in &bound {
            let regions = self
                .regions(provider)
                .into_iter()
                .filter(|r| !r.placement())
                .collect();
            let composes = self.world.get(provider).map_or_else(BTreeSet::new, |d| {
                let named = self.world.names(d.instance, d.form, &["uses"]);
                self.world.closure(named, &["uses"], |d| d.kind.provides())
            });
            owned.push((provider.clone(), regions, composes));
        }
        for (i, (p, p_regions, p_composes)) in owned.iter().enumerate() {
            for (q, q_regions, q_composes) in &owned[i + 1..] {
                if p_composes.contains(q) || q_composes.contains(p) {
                    continue;
                }
                for a in p_regions {
                    for b in q_regions {
                        let found = overlap(a, b);
                        if found == Overlap::None {
                            continue;
                        }
                        let primary = self.first_bound_to(&Bound::Named(p.clone()));
                        let secondary = vec![
                            self.at(a.span, &format!("`{p}` owns {}", a.range())),
                            self.at(b.span, &format!("`{q}` owns {}", b.range())),
                            self.first_bound_to(&Bound::Named(q.clone())),
                        ];
                        if found == Overlap::Bytes {
                            self.conflict(
                                format!(
                                    "`{p}` and `{q}`, both bound in this assignment and neither composing the other, \
                                     own the same bytes: {} and {} (rule 5)",
                                    a.describe(),
                                    b.describe()
                                ),
                                primary,
                                secondary,
                                "resolve again: bind one of them to another provider, so one owner holds the bytes",
                            );
                        } else {
                            let (open, other) = if a.end.is_none() { (a, b) } else { (b, a) };
                            self.conflict(
                                format!(
                                    "whether {} and {} overlap is unknown: `{}` has no size, and `{}` owns bytes \
                                     above its base — `missing-fact`, which blocks the assignment (rule 5)",
                                    a.describe(),
                                    b.describe(),
                                    open.name,
                                    other.provider
                                ),
                                primary,
                                secondary,
                                "give the region its `(size S)`, or resolve again binding one of them to another \
                                 provider",
                            );
                        }
                    }
                }
            }
        }
    }

    /// Rule 9: a caller states no level the profile does not run it at — the description's, judged first — and each
    /// provider bound for a caller is reachable at the level it calls at: the profile's for the system's callers, its
    /// kind's for an instance, which states that level for the callers bound to it.
    fn privilege(&mut self, profile: &str, level: &str) {
        let assignment = self.assignment;
        // With no level given, these rules say nothing of the system's callers.
        if !level.is_empty() {
            let callers: Vec<Requester> = self
                .served()
                .into_iter()
                .filter(|r| self.calls(r))
                .collect();
            for caller in callers {
                for requirement in self.requirements(&caller) {
                    let Requirement::Constraint {
                        fact,
                        value: Value::Set(stated),
                        ..
                    } = &requirement
                    else {
                        continue;
                    };
                    if fact != PRIVILEGE || stated.contains(level) {
                        continue;
                    }
                    let primary = self
                        .requires(&caller, PRIVILEGE)
                        .unwrap_or_else(|| self.at(self.system.form.span(), "the system"));
                    let stated: Vec<&str> = stated.iter().map(String::as_str).collect();
                    self.refuse_description(
                        Status::UnsupportedProfile,
                        format!(
                            "`{}` states it calls from `{}`, which does not hold `{level}`, the level `{profile}` runs \
                             every caller at: no context at those levels exists under this profile (rule 9)",
                            caller.canonical(),
                            stated.join(" ")
                        ),
                        primary,
                        Vec::new(),
                        "state the level the profile runs its callers at among the caller's levels, or check under a \
                         profile that runs callers at a level it states",
                    );
                }
            }
        }
        // Each provider by the level it is called at — `None` the profile's, else an instance's kind's — with its callers.
        type Callers = (Bound, Vec<(Requester, String)>);
        let mut bound: BTreeMap<(String, Option<&'static str>), Callers> = BTreeMap::new();
        for ((requester, fact), provider) in &assignment.bindings {
            let kind_level = match requester {
                Requester::Instance(instance) => match instances::kind(instance) {
                    Some(kind) => Some(kind.level),
                    None => continue,
                },
                _ if !level.is_empty() && self.calls(requester) => None,
                _ => continue,
            };
            let judged = match provider {
                Bound::Named(name) => self.world.provides(name),
                Bound::Instance(_) => true,
            };
            if judged {
                bound
                    .entry((provider.canonical(), kind_level))
                    .or_insert_with(|| (provider.clone(), Vec::new()))
                    .1
                    .push((requester.clone(), fact.clone()));
            }
        }
        for ((name, kind_level), (provider, callers)) in bound {
            let at = kind_level.unwrap_or(level);
            let Some(read) = self.provider(&provider) else {
                continue;
            };
            let (conflict, why) = match outcome(&read, PRIVILEGE) {
                Ok(Outcome::Valued(values)) => match values.first() {
                    Some(Value::Set(offered)) if offered.contains(at) => continue,
                    Some(Value::Set(offered)) => (
                        true,
                        format!(
                            "is reachable from `{}` alone",
                            offered
                                .iter()
                                .map(String::as_str)
                                .collect::<Vec<_>>()
                                .join(" ")
                        ),
                    ),
                    _ => (false, "states its level in no set".to_owned()),
                },
                Ok(Outcome::Absent) => (
                    true,
                    format!("declares `{PRIVILEGE}` absent, reachable from no level"),
                ),
                Ok(Outcome::Unknown) => {
                    (false, format!("offers `{PRIVILEGE}` bare, naming no level"))
                }
                Ok(Outcome::Undescribed | Outcome::Derived(_)) | Err(_) => {
                    (false, format!("does not describe `{PRIVILEGE}`"))
                }
            };
            let named: Vec<String> = callers
                .iter()
                .map(|(r, fact)| format!("`{}` for `{fact}`", r.canonical()))
                .collect();
            let (first, fact) = &callers[0];
            let primary = self.bound(first, fact);
            let secondary = self.declared_bound(&provider).into_iter().collect();
            let (runs, whose) = match kind_level {
                None => (
                    format!("`{profile}` runs every caller at `{at}`"),
                    format!("the level `{profile}` runs every caller at"),
                ),
                Some(_) => (
                    format!("an adapter's instance runs at its kind's level, `{at}`"),
                    "the level its instance's kind runs at".to_owned(),
                ),
            };
            if conflict {
                self.conflict(
                    format!(
                        "`{name}`, bound for {}, {why}, and {runs} (rule 9)",
                        named.join(", ")
                    ),
                    primary,
                    secondary,
                    "resolve again: bind each caller to a provider reachable at the level it calls at",
                );
            } else {
                self.conflict(
                    format!(
                        "`{name}`, bound for {}, {why}, so whether it is reachable at `{at}`, {whose}, is unknown: \
                         `missing-fact`, which blocks the assignment (rule 9)",
                        named.join(", ")
                    ),
                    primary,
                    secondary,
                    "describe the levels the provider is reachable from, `(reachable-at-privilege …)`, or resolve \
                     again with a provider that states its level",
                );
            }
        }
    }

    /// Rule 16, judged beside rule 9's callers: a sporadic task of the system bound itself to a source for
    /// `interrupt-source` at `true` is released on every arrival of it, so the source's stated `min-arrival-separation`
    /// is at least the task's own `min-separation` — the relation's judgement of `(min-arrival-separation (at-least
    /// T))` at the source, `T` the task's. A shorter one, one declared absent, or a comparison past the exact arithmetic
    /// is a conflict, `unsupported-profile`; none, bare or a bound — no value — an unknown, `missing-fact`. A task a
    /// source reaches through a handler is bound itself to none, and this rule says nothing of it.
    fn sources(&mut self) {
        let assignment = self.assignment;
        for ((requester, fact), provider) in &assignment.bindings {
            let Requester::Task { system, .. } = requester else {
                continue;
            };
            if fact != SOURCE
                || *system != assignment.system
                || !at_true(&self.requirements(requester), SOURCE)
            {
                continue;
            }
            let Some((separation, clause)) = self.min_separation(requester) else {
                continue;
            };
            let Some(read) = self.provider(provider) else {
                continue;
            };
            let verdict = judge(
                &read,
                &Requirement::Constraint {
                    fact: ARRIVALS.to_owned(),
                    direction: Direction::AtLeast,
                    value: Value::Quantity(separation),
                },
            );
            // The value the source states, in the spelling written first, for the message.
            let stated = match outcome(&read, ARRIVALS) {
                Ok(Outcome::Valued(values)) => match values.first() {
                    Some(Value::Quantity(q)) => format!(" of `{q}`"),
                    _ => String::new(),
                },
                _ => String::new(),
            };
            let (conflict, why) = match verdict {
                Judged::Satisfied | Judged::Statement => continue,
                Judged::Refused => (
                    true,
                    format!(
                        "states a `{ARRIVALS}`{stated}, shorter than the task's `{SEPARATION}` of `{separation}`, and \
                         the analysis admits a task released on every arrival of a source only where T ≤ T_s, its \
                         condition 5"
                    ),
                ),
                Judged::Absent => (
                    true,
                    format!(
                        "declares `{ARRIVALS}` absent — an unbounded source, which the analysis refuses under its \
                         condition 3"
                    ),
                ),
                Judged::Unsupported => (
                    true,
                    format!(
                        "states a `{ARRIVALS}`{stated} whose comparison with the task's `{SEPARATION}` of \
                         `{separation}` is past the exact arithmetic (§7 rule 2)"
                    ),
                ),
                Judged::Unknown => (
                    false,
                    format!(
                        "offers `{ARRIVALS}` without a value — bare, or only a bound, which the relation reads as no \
                         value"
                    ),
                ),
                Judged::Undescribed => (false, format!("states no `{ARRIVALS}`")),
            };
            let who = requester.canonical();
            let source = provider.canonical();
            let primary = self.bound(requester, SOURCE);
            let secondary = [
                self.requires(requester, SOURCE),
                Some(self.at(clause, &format!("`{who}` states its `{SEPARATION}` here"))),
                self.declared_bound(provider),
            ]
            .into_iter()
            .flatten()
            .collect();
            let released = format!(
                "`{who}` is bound itself to `{source}` for `{SOURCE}`, so every arrival of `{source}` releases it"
            );
            if conflict {
                self.conflict(
                    format!("{released}, and `{source}` {why}: a conflict, `unsupported-profile` (rule 16)"),
                    primary,
                    secondary,
                    "resolve again: bind the task's `interrupt-source` to a source whose `min-arrival-separation` is \
                     at least its `min-separation`, which the search does where the description offers one",
                );
            } else {
                self.conflict(
                    format!(
                        "{released}, and `{source}` {why}, so whether its arrivals are at least the task's \
                         `{SEPARATION}` of `{separation}` apart is unknown: `missing-fact`, which blocks the assignment \
                         (rule 16)"
                    ),
                    primary,
                    secondary,
                    "state the source's `min-arrival-separation`, at least the task's `min-separation`, or resolve \
                     again binding the task to a source that states one",
                );
            }
        }
    }

    /// The `min-separation` a task writes, read as a time, and where: `None` for a task writing none — a periodic one —
    /// or one that does not read, which check (2) refuses.
    fn min_separation(&self, task: &Requester) -> Option<(Quantity, Span)> {
        let (form, _) = self.world.side(task)?;
        let clause = form
            .items()
            .iter()
            .find(|clause| clause.head() == Some(SEPARATION))?;
        let items = clause.items();
        let separation = Quantity::read(items.get(1), items.get(2), clause.span()).ok()?;
        (items.len() == 3 && separation.unit.dimension == Dimension::Time)
            .then_some((separation, clause.span()))
    }

    /// Rule 10: the plan places the image in placement memory of the system's hardware, where it has any; and the
    /// description's unknowns over those regions.
    fn placement(&mut self) {
        let hardware: Vec<String> = self.hardware.iter().cloned().collect();
        let regions: Vec<Region> = hardware.iter().flat_map(|h| self.regions(h)).collect();
        let memory: Vec<String> = regions
            .iter()
            .filter(|r| r.placement())
            .map(Region::describe)
            .collect();
        let system = self.assignment.system.clone();
        match &self.assignment.placement {
            Some((provider, name)) => {
                let why = if !self.world.provides(provider) {
                    Some(format!(
                        "`{provider}` is no block or platform of the description"
                    ))
                } else if !self.hardware.contains(provider) {
                    Some(format!(
                        "`{provider}` is outside the hardware of `{system}`"
                    ))
                } else {
                    match regions
                        .iter()
                        .find(|r| r.provider == *provider && r.name == *name)
                    {
                        None => Some(format!("`{provider}` offers no region `{name}`")),
                        Some(r) if r.reserved => Some(format!("{} is reserved", r.describe())),
                        Some(r) if !r.executable => {
                            Some(format!("{} is not `(executable true)`", r.describe()))
                        }
                        Some(_) => None,
                    }
                };
                if let Some(why) = why {
                    let primary = self
                        .record
                        .clause("placement", "the placement the plan gives");
                    self.conflict(
                        format!(
                            "the plan places the image in `{provider}`'s `{name}`, and {why}: a placement is \
                             placement memory of the system's hardware (rule 10)"
                        ),
                        primary,
                        Vec::new(),
                        "resolve again: place the image in an executable, unreserved region of the system's hardware",
                    );
                }
            }
            None if !memory.is_empty() => {
                let primary = self
                    .record
                    .clause("system", "the plan places the image nowhere");
                self.conflict(
                    format!(
                        "the hardware of `{system}` has placement memory — {} — and the plan places the image in none \
                         (rule 10)",
                        memory.join(", ")
                    ),
                    primary,
                    Vec::new(),
                    "resolve again: a plan places the image in placement memory of the system's hardware",
                );
            }
            None => {}
        }
        self.placement_unknowns(&regions, &system);
    }

    /// Rule 10's unknowns, the description's whatever the assignment: placement memory with no size, nothing to lay out
    /// in; a device's registers or a reserved range, whose extent is the hole the plan avoids, with no size and its base
    /// below placement memory, where only that size says whether they overlap; and holes of no size, each taken to reach
    /// the memory's end, that leave it no free byte while the bytes surely reserved do not fill it.
    fn placement_unknowns(&mut self, regions: &[Region], system: &str) {
        let holes: Vec<&Region> = regions.iter().filter(|r| r.reserved).collect();
        for p in regions.iter().filter(|r| r.placement()) {
            for open in regions
                .iter()
                .filter(|r| !r.placement() && r.end.is_none() && r.base < p.base)
            {
                let primary = self.at(open.span, "a region with no size");
                let secondary = vec![self.at(p.span, "placement memory above its base")];
                self.refuse_description(
                    Status::MissingFact,
                    format!(
                        "whether {} and {} overlap is unknown: `{}` has no size, and the memory's bytes lie above its \
                         base, in the hardware of `{system}` (rule 10)",
                        p.describe(),
                        open.describe(),
                        open.name
                    ),
                    primary,
                    secondary,
                    "give the region its `(size S)`",
                );
            }
            let Some(end) = p.end else {
                let primary = self.at(p.span, "placement memory with no size");
                self.refuse_description(
                    Status::MissingFact,
                    format!(
                        "`{}`'s placement memory `{}` has no size: nothing can be laid out in it, in the hardware of \
                         `{system}` (rule 10)",
                        p.provider, p.name
                    ),
                    primary,
                    Vec::new(),
                    "give the memory its `(size S)`",
                );
                continue;
            };
            let open: Vec<&Region> = holes
                .iter()
                .copied()
                .filter(|r| r.end.is_none() && (p.base..end).contains(&r.base))
                .collect();
            let Some(first) = open.first() else { continue };
            let surely = || holes.iter().map(|r| r.known());
            let taken = surely().chain(open.iter().map(|r| (r.base, end)));
            if fills(surely(), p.base, end) || !fills(taken, p.base, end) {
                continue;
            }
            let primary = self.at(first.span, "a hole of no size");
            let mut secondary: Vec<Label> = open[1..]
                .iter()
                .map(|r| self.at(r.span, "a hole of no size"))
                .collect();
            secondary.push(self.at(p.span, "the placement memory"));
            let named: Vec<String> = open.iter().map(|r| r.describe()).collect();
            self.refuse_description(
                Status::MissingFact,
                format!(
                    "{} has no free byte with its holes of no size, {}, taken to reach its end, and the bytes surely \
                     reserved do not fill it, in the hardware of `{system}` (rule 10)",
                    p.describe(),
                    named.join(", ")
                ),
                primary,
                secondary,
                "give each hole of no size its `(size S)`",
            );
        }
    }

    /// Rule 13: one requester's bindings for the facts of one function are to one provider — an instance one provider
    /// of its function.
    fn functions(&mut self) {
        let assignment = self.assignment;
        type ByProvider = BTreeMap<String, (Bound, Vec<String>)>;
        let mut by: BTreeMap<Requester, BTreeMap<&'static str, ByProvider>> = BTreeMap::new();
        for ((requester, fact), provider) in &assignment.bindings {
            if let Some(Function::Of(function)) = function_of(fact) {
                by.entry(requester.clone())
                    .or_default()
                    .entry(function)
                    .or_default()
                    .entry(provider.canonical())
                    .or_insert_with(|| (provider.clone(), Vec::new()))
                    .1
                    .push(fact.clone());
            }
        }
        for (requester, functions) in by {
            let requirements = self.requirements(&requester);
            for (function, providers) in functions {
                if providers.len() < 2 {
                    continue;
                }
                let of: Vec<&Requirement> = requirements
                    .iter()
                    .filter(|r| function_of(fact_of(r)) == Some(Function::Of(function)))
                    .collect();
                // Bindings of nothing the requester requires are check (3)'s to refuse, and leave nothing to judge.
                if of.is_empty() {
                    continue;
                }
                let mut judged: Vec<(&String, Vec<(&str, Judged)>)> = Vec::new();
                for (name, (provider, _)) in &providers {
                    let Some(read) = self.provider(provider) else {
                        continue;
                    };
                    judged.push((
                        name,
                        of.iter().map(|r| (fact_of(r), judge(&read, r))).collect(),
                    ));
                }
                if judged.len() < providers.len() {
                    continue;
                }
                let split: Vec<String> = providers
                    .iter()
                    .map(|(p, (_, facts))| format!("`{p}` for `{}`", facts.join("`, `")))
                    .collect();
                let who = requester.canonical();
                let fact = providers
                    .values()
                    .flat_map(|(_, facts)| facts)
                    .next()
                    .expect("a function bound");
                let primary = self.bound(&requester, fact);
                let secondary: Vec<Label> = providers
                    .values()
                    .filter_map(|(p, _)| self.declared_bound(p))
                    .collect();
                let meets = judged
                    .iter()
                    .find(|(_, verdicts)| verdicts.iter().all(|(_, v)| *v == Judged::Satisfied));
                let whole: Vec<String> = judged
                    .iter()
                    .filter(|(_, verdicts)| {
                        !verdicts
                            .iter()
                            .any(|(_, v)| matches!(v, Judged::Refused | Judged::Absent))
                    })
                    .map(|(p, verdicts)| {
                        let open: Vec<String> = verdicts
                            .iter()
                            .filter(|(_, v)| *v != Judged::Satisfied)
                            .map(|(f, v)| format!("`{f}` {}", judged_word(*v)))
                            .collect();
                        format!("`{p}`, where {}", open.join(", "))
                    })
                    .collect();
                if let Some((provider, _)) = meets {
                    self.conflict(
                        format!(
                            "`{who}` binds the facts of its `{function}` function to {} providers — {} — and \
                             `{provider}` meets every one of those requirements: one requester's facts of one function \
                             are met at one provider (rule 13)",
                            providers.len(),
                            split.join(", ")
                        ),
                        primary,
                        secondary,
                        "resolve again: bind every fact of the function for this requester to the one provider that \
                         meets them all",
                    );
                } else if !whole.is_empty() {
                    self.conflict(
                        format!(
                            "`{who}` binds the facts of its `{function}` function to {} providers — {} — none meets \
                             them all, and {} refutes none: describing those facts makes it whole, so the split is \
                             unknown — `missing-fact`, or `unsupported-profile` past the exact arithmetic — and blocks \
                             the assignment (rule 13)",
                            providers.len(),
                            split.join(", "),
                            whole.join("; ")
                        ),
                        primary,
                        secondary,
                        "describe the facts at the provider that refutes none, or resolve again",
                    );
                } else {
                    self.conflict(
                        format!(
                            "`{who}` binds the facts of its `{function}` function to {} providers — {} — and each \
                             refutes one of those requirements: no provider meets them whole (rule 13)",
                            providers.len(),
                            split.join(", ")
                        ),
                        primary,
                        secondary,
                        "resolve again with one provider meeting every requirement of the function, or write two \
                         requesters where two devices are meant",
                    );
                }
            }
        }
    }

    /// Rule 13's qualifiers: each requirement on a fact of `(function each)` — `reachable-at-privilege` apart, rule 9's —
    /// is judged at every provider bound for the requester's functions and for those of each requester it reaches, an
    /// instance among them.
    fn qualifiers(&mut self) {
        let assignment = self.assignment;
        for requester in self.served() {
            let qualifiers: Vec<Requirement> = self
                .requirements(&requester)
                .into_iter()
                .filter(|r| {
                    !matches!(r, Requirement::Statement(..))
                        && fact_of(r) != PRIVILEGE
                        && function_of(fact_of(r)) == Some(Function::Each)
                })
                .collect();
            if qualifiers.is_empty() {
                continue;
            }
            let reaches = self.reaches(&requester);
            let mut relied: BTreeMap<String, (Bound, Requester, String)> = BTreeMap::new();
            for ((r, fact), provider) in &assignment.bindings {
                let mine =
                    *r == requester || matches!(r, Requester::Named(n) if reaches.contains(n));
                if mine && matches!(function_of(fact), Some(Function::Of(_))) {
                    relied
                        .entry(provider.canonical())
                        .or_insert_with(|| (provider.clone(), r.clone(), fact.clone()));
                }
            }
            // A requester relying on no function bound binds the qualifier as any requirement: checks (3) and (4).
            for (name, (provider, r, fact)) in relied {
                let Some(read) = self.provider(&provider) else {
                    continue;
                };
                for qualifier in &qualifiers {
                    let verdict = judge(&read, qualifier);
                    let conflict = match verdict {
                        Judged::Satisfied | Judged::Statement => continue,
                        Judged::Refused | Judged::Absent => true,
                        Judged::Unknown | Judged::Undescribed | Judged::Unsupported => false,
                    };
                    let q = fact_of(qualifier);
                    let primary = self.bound(&r, &fact);
                    let secondary = [self.requires(&requester, q), self.declared_bound(&provider)]
                        .into_iter()
                        .flatten()
                        .collect();
                    let message = format!(
                        "`{}` relies on `{name}` — bound for `{}`'s `{fact}` — and its qualifier `{q}`, which every \
                         function it relies on must hold, is {} there{} (rule 13)",
                        requester.canonical(),
                        r.canonical(),
                        judged_word(verdict),
                        if conflict {
                            ""
                        } else {
                            ": an unknown, which blocks the assignment"
                        }
                    );
                    self.conflict(
                        message,
                        primary,
                        secondary,
                        "resolve again with providers that hold the qualifier, or describe it at the provider",
                    );
                }
            }
        }
    }

    /// Rule 14, one time base: where a requester is bound for `absolute-deadline`, every other requester it reaches
    /// through `needs` and `uses` that is bound for a fact of the function `counter` that is not consumed is bound at
    /// the same device — a block or platform its own, an instance the one it programs.
    fn time_base(&mut self) {
        let assignment = self.assignment;
        for ((requester, fact), provider) in &assignment.bindings {
            if fact != DEADLINE {
                continue;
            }
            let device = instances::device(provider);
            let reaches = self.reaches(requester);
            // Each other requester it reaches, by the other device it is bound at: its facts bound there, and to what.
            let mut elsewhere: BTreeMap<(&Requester, &str), Vec<(&str, &Bound)>> = BTreeMap::new();
            for ((other, read), bound) in &assignment.bindings {
                let Requester::Named(name) = other else {
                    continue;
                };
                if other == requester || !reaches.contains(name) {
                    continue;
                }
                let counter = vocab::entry(read)
                    .is_some_and(|e| e.function == Some(Function::Of(COUNTER)) && !e.consumed);
                let theirs = instances::device(bound);
                if counter && theirs != device {
                    elsewhere
                        .entry((other, theirs))
                        .or_default()
                        .push((read, bound));
                }
            }
            for ((other, theirs), bound) in elsewhere {
                let read: Vec<&str> = bound.iter().map(|(read, _)| *read).collect();
                let via = |b: &Bound| match b {
                    Bound::Named(_) => String::new(),
                    Bound::Instance(instance) => format!(" through `{}`", instance.canonical()),
                };
                let primary = self.bound(requester, fact);
                let secondary = [
                    Some(self.bound(other, read[0])),
                    self.declared(device),
                    self.declared(theirs),
                ]
                .into_iter()
                .flatten()
                .collect();
                self.conflict(
                    format!(
                        "`{}` is bound for `{DEADLINE}` at the device `{device}`{}, and `{}`, which it reaches through \
                         `needs` and `uses`, is bound for `{}` at the device `{theirs}`{}: a deadline programmed in one \
                         counter's time and read in another's (rule 14)",
                        requester.canonical(),
                        via(provider),
                        other.canonical(),
                        read.join("`, `"),
                        via(bound[0].1)
                    ),
                    primary,
                    secondary,
                    "resolve again: bind the counter the deadline's requester reads at the device its deadline is \
                     programmed on, or write the second counter for a requester the deadline's does not reach",
                );
            }
        }
    }

    /// The verdict: the description's own, first and whole; else the plan's conflicts and unknowns, `tool-failure`.
    fn verdict(self) -> Result<(), Verdict> {
        if !self.description.is_empty() {
            let status = [
                Status::InvalidDescription,
                Status::UnsupportedProfile,
                Status::InfeasibleConfiguration,
                Status::MissingFact,
            ]
            .into_iter()
            .find(|s| self.description.iter().any(|(t, _)| t == s))
            .unwrap_or(self.description[0].0);
            return Err(Verdict {
                status,
                findings: self.description.into_iter().map(|(_, f)| f).collect(),
            });
        }
        if self.plan.is_empty() {
            Ok(())
        } else {
            Err(Verdict {
                status: Status::ToolFailure,
                findings: self.plan,
            })
        }
    }
}

/// Why a binding of `requester` to `instance` is not one its kind admits (record §1.4, §3.1): an exclusive instance is
/// its one requester's own (rules 2 and 3), a shared one names none, and a kind is the checker's table's (rule 1).
fn through(instance: &Instance, requester: &Requester) -> Option<String> {
    let Some(kind) = instances::kind(instance) else {
        return Some(format!(
            "whose kind `{}` this checker's table does not hold (rule 1)",
            instance.kind
        ));
    };
    match (kind.exclusive(), instance.serves.as_deref()) {
        (true, Some(serves)) if serves == requester => None,
        (true, Some(serves)) => Some(format!(
            "an exclusive instance, which serves `{}` alone: it offers one channel, its one requester's (rule 3)",
            serves.canonical()
        )),
        (true, None) => Some(
            "an exclusive instance naming no requester: it offers one channel, its one requester's (rule 3)".to_owned(),
        ),
        (false, Some(_)) => Some(
            "a shared instance, which serves every requester bound to it and names none (rule 1)".to_owned(),
        ),
        (false, None) => None,
    }
}

/// A verdict of the relation, in words.
const fn judged_word(verdict: Judged) -> &'static str {
    match verdict {
        Judged::Satisfied => "satisfied",
        Judged::Refused => "refused",
        Judged::Absent => "absent",
        Judged::Unknown => "unknown",
        Judged::Undescribed => "undescribed",
        Judged::Statement => "a statement",
        Judged::Unsupported => "past the exact arithmetic",
    }
}
