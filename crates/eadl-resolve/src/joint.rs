//! Joint constraints: whether the bindings of one complete assignment hold together (`docs/semantics/model.md` §8;
//! leaf `M3.3.2`, the design in `docs/tasks/M3.md`'s `M3.3`).
//!
//! ⭐ **The relation judges a binding alone; these rules judge the bindings together.** An assignment binds each
//! requester's requirement to one provider the relation satisfies it at (§7). Two rules are decided here, over one
//! complete assignment the resolver proposes:
//!
//! - **Channels** (F08). A requester bound to a consumed fact — `use consumed` in the vocabulary — at the value `true`
//!   takes one of the provider's `channels`, and a provider serves no more requesters than it has.
//! - **Ownership** (F09). A provider bound in the assignment owns every region it offers, and the regions of two such
//!   providers do not overlap, unless one composes the other: names it through `uses`, at any depth through blocks and
//!   platforms, whose `uses` alone is composition (model §8 rules 1 and 5).
//! - **Connectivity** (leaf `M3.3.3`). An assignment is one [`System`]'s, and a block or platform it binds a requester
//!   of that system to is part of the system's hardware — `SR-H15`'s "a provider on a platform the system does not use
//!   never satisfying", the joint check's.
//! - **Routes.** Each provider the assignment relies on for an interrupt — a source, or a timer's alarm — is reported
//!   as an [`Obligation::Route`]: the description states that the provider is part of the system, and the path from
//!   it to the core, with its latency, is the catalog's to state and the plan's to enable.
//! - **Placement** (leaf `M3.3.5.1`). A system's memory map, once: placement memory — executable, unreserved regions —
//!   holds no device's registers and no second placement region, and the reserved ranges in it are holes the plan avoids,
//!   reported with it as an [`Obligation::Placement`], one with no size reaching the region's end; a placement the plan
//!   is given is placement memory of the system.
//! - **Privilege** (leaf `M3.3.4.1`). Given the level a plan binds the system's callers to, a caller states no other
//!   level, and every provider bound for a caller is reachable at that level and says so (`SR-H11`, `SR-H12`).
//! - **Functions** (leaf `M3.3.6.2`, `SR-H14`). One requester's bindings for the facts of one function — the
//!   vocabulary's `function` — are to one provider; a qualifier, `(function each)`, holds at every provider bound for
//!   the requester's functions and for those of what it relies on through `uses` and `needs`.
//!
//! A [`Report`] lists what one assignment breaks. It is a judgment of that assignment only: the description is
//! `infeasible-configuration`, or `missing-fact`, when no assignment the resolver can propose avoids it, which is
//! `M3.4`'s search to conclude. What a provider writes that cannot mean anything — a consumed function offered with no
//! channel, a region with no base — is refused when the providers are read, whether or not anything binds them.
//!
//! A node several users reach is one requester of the channel it takes (model §8 rule 4): its users are the jobs of the
//! system's tasks that reach it and the interrupts whose routines run it, which [`Joint::contexts`] derives, so its
//! realization is the multiplexer, and an [`Obligation::Multiplex`] reports that it owes them the semantics and a bounded
//! overhead.

use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};
use std::rc::Rc;

use eadl_front::{Diagnostic, Form, Label, Span};
use eadl_model::{Dimension, Quantity};

use crate::adapter::Instance;
use crate::offer::{Provider, State};
use crate::relation::{judge, Verdict};
use crate::requirement::side_name_lists;
use crate::requirement::{self, Requirement};
use crate::value::{order, Value};
use crate::vocabulary::{Domain, Function, Usage, Vocabulary};

/// Who a requester is (the design's requester).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A task, for what it writes itself.
    Task,
    /// A service.
    Service,
    /// Any other declaration, for what it writes at its own level, outside its tasks.
    Other,
    /// An engine adapter's instance, for what its kind requires of the provider it programs: a caller at its kind's
    /// level (`docs/specs/adapters/decision_engine-adapters.md` §4, rules 1 and 9).
    Adapter,
}

/// One requester, its requirements and the declarations its `uses` names.
#[derive(Debug, Clone)]
pub struct Requester {
    /// Its name; a task's is `(task SYSTEM TASK)` ([`task_name`]).
    pub name: String,
    /// Who it is.
    pub kind: Kind,
    /// Where it is written.
    pub span: Span,
    /// What it requires, as the relation reads a side.
    pub requirements: Vec<Requirement>,
    /// The declarations its `uses` lists name.
    pub uses: Vec<String>,
    /// The declarations its `needs` lists name — a service another service needs, not a vocabulary fact.
    pub needs: Vec<String>,
    /// A task's `min-separation`, where it writes one as a duration; no other requester's (model §8 rule 16).
    pub separation: Option<Quantity>,
}

/// Every requester `decl` writes: a system's own level and each of its tasks apart, any other declaration whole.
/// A block or a platform writes requirements too; its `uses` is composition, which [`Provider`]s read instead.
///
/// # Errors
///
/// The refusals the relation's reading of a side gives (§7 rule 4).
pub fn requesters(
    decl: &Form,
    vocabulary: &Vocabulary,
) -> Result<Vec<Requester>, requirement::Refusals> {
    let name = name_of(decl).to_string();
    let kind = match decl.head() {
        Some("defservice") => Kind::Service,
        _ => Kind::Other,
    };
    if decl.head() != Some("defsystem") {
        return Ok(vec![requester(decl, name, kind, vocabulary)?]);
    }
    let (tasks, own): (Vec<&Form>, Vec<&Form>) = decl
        .items()
        .iter()
        .partition(|item| item.head() == Some("task"));
    let own = Form::List {
        items: own.into_iter().cloned().collect(),
        span: decl.span(),
    };
    let mut out = vec![requester(&own, name.clone(), Kind::Other, vocabulary)?];
    for task in tasks {
        out.push(requester(
            task,
            task_name(&name, name_of(task)),
            Kind::Task,
            vocabulary,
        )?);
    }
    Ok(out)
}

/// A task's name as the joint rules hold it: `(task SYSTEM TASK)`, the form the assignment record writes, which no
/// declaration's name can be — a symbol holds no space or parenthesis — as an instance's `(adapter …)` cannot (leaf
/// `M4.1.11`: `SYSTEM/TASK` was a symbol a declaration could also be named).
#[must_use]
pub fn task_name(system: &str, task: &str) -> String {
    format!("(task {system} {task})")
}

/// A task's system and name, where `name` is one ([`task_name`]).
#[must_use]
pub fn task_of(name: &str) -> Option<(&str, &str)> {
    let inner = name.strip_prefix("(task ")?.strip_suffix(')')?;
    match inner.split(' ').collect::<Vec<_>>()[..] {
        [system, task] => Some((system, task)),
        _ => None,
    }
}

fn requester(
    form: &Form,
    name: String,
    kind: Kind,
    vocabulary: &Vocabulary,
) -> Result<Requester, requirement::Refusals> {
    Ok(Requester {
        name,
        kind,
        span: form.span(),
        requirements: requirement::read_side(form, vocabulary)?,
        uses: used_names(form),
        needs: requirement::side_name_lists(form)
            .into_iter()
            .filter(|list| list.head() == Some("needs"))
            .flat_map(|list| list.items()[1..].iter().filter_map(Form::as_symbol))
            .map(str::to_string)
            .collect(),
        separation: if kind == Kind::Task {
            separation(form)
        } else {
            None
        },
    })
}

/// A task's `(min-separation N UNIT)`, as a duration: one the language's check refuses reads as none.
fn separation(task: &Form) -> Option<Quantity> {
    let clause = task
        .items()
        .iter()
        .find(|item| item.head() == Some("min-separation"))?;
    let items = clause.items();
    Quantity::read(items.get(1), items.get(2), clause.span())
        .ok()
        .filter(|q| q.unit.dimension == Dimension::Time)
}

/// The names `form`'s `uses` lists hold, at the positions the relation reads a side's.
fn used_names(form: &Form) -> Vec<String> {
    requirement::side_name_lists(form)
        .into_iter()
        .filter(|list| list.head() == Some("uses"))
        .flat_map(|list| list.items()[1..].iter().filter_map(Form::as_symbol))
        .map(str::to_string)
        .collect()
}

fn name_of(form: &Form) -> &str {
    form.items()
        .get(1)
        .and_then(Form::as_symbol)
        .unwrap_or_default()
}

/// The names the `uses` and `needs` lists of `form` hold, at the positions the relation reads a side's — a system's
/// `platform` clause and its tasks' included, `refines` never.
fn named_by(form: &Form) -> impl Iterator<Item = &str> {
    side_name_lists(form)
        .into_iter()
        .flat_map(|list| list.items()[1..].iter().filter_map(Form::as_symbol))
}

/// Whether `decl` declares hardware: a block or a platform.
fn is_hardware(decl: &Form) -> bool {
    matches!(decl.head(), Some("defblock" | "defplatform"))
}

/// One system as the joint rules read it: its hardware, and every declaration its closure reaches (model §8 rule 7).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct System {
    /// Its name.
    pub name: String,
    /// Where it is declared.
    pub span: Span,
    /// The blocks and platforms it has: every one its `platform` clause, its own `uses` and `needs` or its tasks' name,
    /// and every one those name in turn, at any depth. `refines` is no edge: a refinement keeps the abstract
    /// platform's obligations (model §3), not its blocks.
    pub hardware: BTreeSet<String>,
    /// Every declaration a `uses` or `needs` of the system, its tasks, or of what they reach names, at any depth.
    pub reached: BTreeSet<String>,
    /// The privilege level a plan binds the system's callers to, when one is given (model §8 rule 9).
    pub level: Option<String>,
    /// The region a plan places the image in, when one is given (model §8 rule 10): M3 chooses it, `M4.1` the offsets.
    pub placement: Option<String>,
}

impl System {
    /// Read `decl`, a `defsystem`, among the description's `declarations`; a name no declaration bears is a
    /// vocabulary fact or another rule's refusal, and leads nowhere.
    #[must_use]
    pub fn read(decl: &Form, declarations: &[Form]) -> Self {
        let declared: BTreeMap<&str, &Form> =
            declarations.iter().map(|d| (name_of(d), d)).collect();
        // Another system named is reached for its own level alone, never its tasks, which are that system's requesters
        // (model §8 rule 7's "a declaration its closure reaches"; `M1.46` refuses the name).
        let own_level = |system: &Form| Form::List {
            items: system
                .items()
                .iter()
                .filter(|item| item.head() != Some("task"))
                .cloned()
                .collect(),
            span: system.span(),
        };
        let mut reached = BTreeSet::new();
        let mut todo: Vec<Form> = vec![decl.clone()];
        while let Some(at) = todo.pop() {
            for name in named_by(&at) {
                if let Some(next) = declared.get(name) {
                    if reached.insert(name.to_string()) {
                        todo.push(if next.head() == Some("defsystem") {
                            own_level(next)
                        } else {
                            (*next).clone()
                        });
                    }
                }
            }
        }
        let mut hardware = BTreeSet::new();
        let mut todo: Vec<&Form> = named_by(decl)
            .filter_map(|name| declared.get(name).copied())
            .filter(|d| is_hardware(d))
            .collect();
        while let Some(at) = todo.pop() {
            if hardware.insert(name_of(at).to_string()) {
                todo.extend(
                    named_by(at)
                        .filter_map(|name| declared.get(name).copied())
                        .filter(|d| is_hardware(d)),
                );
            }
        }
        Self {
            name: name_of(decl).to_string(),
            span: decl.span(),
            hardware,
            reached,
            level: None,
            placement: None,
        }
    }

    /// The system with the region a plan places its image in.
    #[must_use]
    pub fn placed_in(mut self, region: &str) -> Self {
        self.placement = Some(region.to_string());
        self
    }

    /// The system with the level a plan binds its callers to — the profile's, once `M3.3.4.2` states it.
    ///
    /// # Errors
    ///
    /// A level outside `reachable-at-privilege`'s domain: the engine's input error, never a description's code.
    pub fn at(mut self, level: &str, vocabulary: &Vocabulary) -> Result<Self, String> {
        match vocabulary.fact(REACHABLE_AT_PRIVILEGE).map(|f| &f.domain) {
            Some(Domain::Set(members)) if members.iter().any(|m| m == level) => {
                self.level = Some(level.to_string());
                Ok(self)
            }
            Some(Domain::Set(members)) => Err(format!(
                "`{level}` is no level of `{REACHABLE_AT_PRIVILEGE}`, whose levels are {}",
                members.join(", ")
            )),
            _ => Err(format!(
                "the vocabulary declares no set `{REACHABLE_AT_PRIVILEGE}`"
            )),
        }
    }

    /// Whether the system's closure reaches `requester`: its own level, one of its tasks, or a declaration it reaches.
    #[must_use]
    pub fn serves(&self, requester: &str) -> bool {
        requester == self.name
            || task_of(requester).is_some_and(|(system, _)| system == self.name)
            || self.reached.contains(requester)
    }
}

/// One binding of an assignment: a requester's requirement, met by a provider.
#[derive(Debug, Clone)]
pub struct Binding {
    /// The requester's name.
    pub requester: String,
    /// The requirement the provider meets.
    pub requirement: Requirement,
    /// The provider's name.
    pub provider: String,
}

/// The fact a requester released by an interrupt requires of its source.
const INTERRUPT_SOURCE: &str = "interrupt-source";
/// A deadline programmed at an absolute time, whose time base rule 14 holds to one device.
const ABSOLUTE_DEADLINE: &str = "absolute-deadline";
/// The source's own bound on how often it fires, which the analysis reads with the route.
const MIN_ARRIVAL_SEPARATION: &str = "min-arrival-separation";
/// The levels a function is reachable from, and the level a caller states it calls from.
const REACHABLE_AT_PRIVILEGE: &str = "reachable-at-privilege";

/// A byte range a provider offers: `(region NAME (base B) (size S) (executable E) (reserved R))`, model §8 rule 5's
/// grammar — `base` once, `size` at most once, each flag at most once and `true` or `false`, nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    /// The region's name.
    pub name: String,
    /// Where it is offered.
    pub span: Span,
    /// Its first byte.
    pub base: i128,
    /// How many bytes it holds, when its size is written.
    pub bytes: Option<i128>,
    /// `(executable true)`: memory code may be placed in.
    pub executable: bool,
    /// `(reserved true)`: a range nothing may be placed in.
    pub reserved: bool,
}

impl Region {
    /// Placement memory: executable and not reserved (model §8 rule 10).
    #[must_use]
    pub const fn is_memory(&self) -> bool {
        self.executable && !self.reserved
    }

    /// The bytes it is known to own: `[base, base + size)`, or its base alone when its size is not written.
    const fn known(&self) -> (i128, i128) {
        match self.bytes {
            Some(bytes) => (self.base, self.base + bytes),
            None => (self.base, self.base + 1),
        }
    }
}

/// What a provider says of its channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Channels {
    /// A count.
    Count(i128),
    /// Not written.
    Undescribed,
    /// Offered without a value, or past the arithmetic.
    Unknown,
}

/// What the joint rules read of one provider.
#[derive(Debug, Clone)]
struct Held {
    span: Span,
    /// A block or a platform, which connectivity judges; an adapter or a record would take its users' system.
    hardware: bool,
    /// What it states of its own `min-arrival-separation`, which a route obligation carries.
    separation: Option<State>,
    /// What it states of `reachable-at-privilege`.
    level: Option<State>,
    channels: Channels,
    regions: Vec<Region>,
    composes: BTreeSet<String>,
}

/// What a provider leaves of a function it would meet whole (rule 13): each fact it does not yet meet, where the
/// requester requires it, and the relation's verdict there.
pub type Gaps = Vec<(String, Span, Verdict)>;

/// One rule broken, or one fact a rule cannot do without.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Finding {
    /// More requesters take a provider's channels than it has (F08).
    OverChannels {
        /// The provider.
        provider: String,
        /// Its `channels`.
        channels: i128,
        /// The requesters taking one, each with where it requires the fact.
        requesters: Vec<(String, Span)>,
    },
    /// Two requesters or more take a provider's channels, and it does not say how many it has.
    ChannelsUnknown {
        /// The provider.
        provider: String,
        /// The requesters taking one.
        requesters: Vec<(String, Span)>,
    },
    /// Two bound providers own overlapping bytes, and neither composes the other (F09).
    Overlap {
        /// The first provider and its region.
        first: (String, Region),
        /// The second.
        second: (String, Region),
    },
    /// A bound provider's region has no size, and another bound provider owns bytes at or above its base.
    RegionUnsized {
        /// The provider.
        provider: String,
        /// The region.
        region: Region,
        /// The provider owning bytes above its base.
        against: String,
    },
    /// A provider offers a consumed function `true` and declares its `channels` absent, or zero.
    ChannelsContradicted {
        /// The provider.
        provider: String,
        /// Where it is declared.
        span: Span,
    },
    /// A region written so that it names no byte range.
    RegionMalformed {
        /// The provider.
        provider: String,
        /// Where the region is written.
        span: Span,
        /// What is wrong.
        detail: String,
    },
    /// A region whose range lies past the exact arithmetic.
    RegionPastArithmetic {
        /// The provider.
        provider: String,
        /// Where the region is written.
        span: Span,
    },
    /// A caller stating a level its system does not run its callers at (leaf `M3.3.4.1`).
    LevelUnsupported {
        /// The caller.
        requester: String,
        /// Where it states its level.
        span: Span,
        /// The levels it states.
        stated: Vec<String>,
        /// The system's level.
        level: String,
        /// The system.
        system: String,
    },
    /// A caller bound to a provider whose `reachable-at-privilege` does not hold the system's level (`SR-H11`).
    OutOfLevel {
        /// The caller.
        requester: String,
        /// Where it requires what the provider meets.
        span: Span,
        /// The provider.
        provider: String,
        /// Where the provider is declared.
        declared: Span,
        /// The system's level.
        level: String,
        /// The levels the provider is reachable from.
        offered: Vec<String>,
    },
    /// A caller bound to a provider that states no level: undescribed or bare, or declared absent (`SR-H12`).
    LevelUnstated {
        /// The caller.
        requester: String,
        /// Where it requires what the provider meets.
        span: Span,
        /// The provider.
        provider: String,
        /// Where the provider is declared.
        declared: Span,
        /// Declared absent — reachable from no level — rather than undescribed or offered bare.
        absent: bool,
    },
    /// A sporadic task bound itself for `interrupt-source` to a source the analysis does not admit for it: every arrival
    /// releases it, so the source's stated `min-arrival-separation` must be at least the task's own `min-separation`
    /// (model §8 rule 16; `RTP-H24`).
    OwnSource {
        /// The task.
        requester: String,
        /// Where it requires `interrupt-source`.
        span: Span,
        /// The source.
        provider: String,
        /// Where the source is declared.
        declared: Span,
        /// Why the analysis does not admit it.
        why: Unadmitted,
    },
    /// Two claims on the same bytes in a system's memory map: a region that is neither placement memory nor reserved —
    /// a device's registers — overlapping placement memory, or two placement regions overlapping (leaf `M3.3.5.1`).
    MapContradiction {
        /// The system.
        system: String,
        /// The placement region and its provider.
        memory: (String, Region),
        /// The region overlapping it and its provider.
        other: (String, Region),
    },
    /// Placement memory whose every byte is a reserved hole: nothing is left to place in.
    MemoryAllHoles {
        /// The system.
        system: String,
        /// The placement region and its provider.
        memory: (String, Region),
    },
    /// A region of a system's memory map with no size, where whether it overlaps placement memory is unknown.
    PlacementUnsized {
        /// The system.
        system: String,
        /// The sizeless region and its provider.
        sizeless: (String, Region),
        /// The provider whose region lies at or above its base.
        against: String,
    },
    /// Placement memory with no size: nothing can be laid out in it.
    MemoryUnsized {
        /// The system.
        system: String,
        /// The placement region and its provider.
        memory: (String, Region),
    },
    /// A hole of no size in placement memory the holes, each taken to reach the region's end, leave no free byte of.
    HoleUnsized {
        /// The system.
        system: String,
        /// The placement region and its provider.
        memory: (String, Region),
        /// The reserved range with no size and its provider.
        hole: (String, Region),
    },
    /// A placement given to the plan that is not placement memory of the system's hardware.
    PlacementIllegal {
        /// The system.
        system: String,
        /// Where the system is declared.
        span: Span,
        /// The region given.
        region: String,
        /// Why it cannot hold the image.
        why: String,
    },
    /// One requester's facts of one function bound to two providers or more, none of which could meet them whole by a
    /// fact described (model §8 rule 13, `SR-H14`).
    FunctionSplit {
        /// The requester.
        requester: String,
        /// The function.
        function: String,
        /// Each provider, by name, with the facts bound to it and where they are required.
        bound: Vec<(String, Vec<(String, Span)>)>,
    },
    /// One requester's facts of one function bound to two providers or more, where a provider bound for them refutes
    /// none of them and would meet them whole once what it leaves undescribed or unknown is described (rule 13).
    FunctionIncomplete {
        /// The requester.
        requester: String,
        /// The function.
        function: String,
        /// Each such provider, by name, with each fact it does not yet meet, where it is required, and its verdict there.
        completions: Vec<(String, Gaps)>,
        /// Whether a fact some such provider does not meet lies past the exact arithmetic, judged first (`SR-H18`).
        past_arithmetic: bool,
    },
    /// A qualifier a requester requires, unmet at a provider bound for a function it relies on (rule 13).
    QualifierUnmet {
        /// The requester.
        requester: String,
        /// The qualifier.
        fact: String,
        /// Where it is required.
        span: Span,
        /// The provider.
        provider: String,
        /// Where the provider is declared.
        declared: Span,
        /// The relation's verdict there: refused or absent, unknown or undescribed, or past the arithmetic.
        verdict: Verdict,
    },
    /// A requester of a system bound to a block or platform outside the system's hardware (leaf `M3.3.3`).
    Unconnected {
        /// The requester.
        requester: String,
        /// Where it requires the fact.
        span: Span,
        /// The provider.
        provider: String,
        /// Where the provider is declared.
        declared: Span,
        /// The system.
        system: String,
    },
    /// A deadline programmed at one device and a time read from another, by requesters the deadline's reaches (model
    /// §8 rule 14, `docs/specs/adapters/decision_engine-adapters.md` §4).
    TwoTimeBases {
        /// The requester bound for the deadline.
        requester: String,
        /// Where it requires it.
        span: Span,
        /// The device the deadline is programmed at.
        device: String,
        /// The requester reading a time-base fact.
        reader: String,
        /// Where it requires that fact.
        read: Span,
        /// The device it reads it at.
        read_device: String,
    },
}

/// Why the analysis does not admit a task's own source (model §8 rule 16).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unadmitted {
    /// Its separation shorter than the task's, against the analysis's condition 5: `unsupported-profile`.
    Faster {
        /// The task's `min-separation`.
        task: Quantity,
        /// The source's `min-arrival-separation`.
        source: Quantity,
    },
    /// Its separation declared absent, an unbounded source the analysis refuses (its condition 3): `unsupported-profile`.
    Absent,
    /// The comparison past the exact arithmetic (§7 rule 2): `unsupported-profile`.
    PastArithmetic,
    /// No value stated — none, bare, or a bound, which the relation reads as no value: `missing-fact`.
    Unstated,
}

impl Finding {
    /// Its code (`ROADMAP.md` §5.5).
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::OverChannels { .. }
            | Self::Overlap { .. }
            | Self::Unconnected { .. }
            | Self::TwoTimeBases { .. }
            | Self::OutOfLevel { .. }
            | Self::PlacementIllegal { .. }
            | Self::LevelUnstated { absent: true, .. } => "infeasible-configuration",
            Self::ChannelsUnknown { .. }
            | Self::RegionUnsized { .. }
            | Self::PlacementUnsized { .. }
            | Self::MemoryUnsized { .. }
            | Self::HoleUnsized { .. }
            | Self::LevelUnstated { absent: false, .. } => "missing-fact",
            Self::MapContradiction { .. } | Self::MemoryAllHoles { .. } => "invalid-description",
            Self::LevelUnsupported { .. } => "unsupported-profile",
            Self::OwnSource {
                why: Unadmitted::Unstated,
                ..
            } => "missing-fact",
            Self::OwnSource { .. } => "unsupported-profile",
            Self::ChannelsContradicted { .. } | Self::RegionMalformed { .. } => {
                "invalid-description"
            }
            Self::RegionPastArithmetic { .. } => "unsupported-profile",
            Self::FunctionSplit { .. } => "infeasible-configuration",
            Self::FunctionIncomplete {
                past_arithmetic: true,
                ..
            } => "unsupported-profile",
            Self::FunctionIncomplete { .. } => "missing-fact",
            Self::QualifierUnmet { verdict, .. } => match verdict {
                Verdict::Refused | Verdict::Absent => "infeasible-configuration",
                Verdict::Unsupported => "unsupported-profile",
                _ => "missing-fact",
            },
        }
    }

    /// The finding as a diagnostic: what is wrong, where, and what to do.
    #[must_use]
    pub fn diagnostic(&self) -> Diagnostic {
        match self {
            Self::OverChannels {
                provider,
                channels,
                requesters,
            } => {
                let names = listed(requesters);
                let primary = Label::new(requesters[0].1, "takes a channel here");
                requesters[1..]
                    .iter()
                    .fold(
                        Diagnostic::error(
                            "infeasible-configuration",
                            format!(
                                "`{provider}` serves {channels} requester(s) at once through its consumed facts, \
                                 and {} take one: {names}",
                                requesters.len()
                            ),
                            primary,
                            "bind a requester to another provider, select a realization that shares the function, \
                             or use a provider with more channels",
                        ),
                        |d, (_, span)| d.with_secondary(Label::new(*span, "and here")),
                    )
            }
            Self::ChannelsUnknown {
                provider,
                requesters,
            } => {
                let names = listed(requesters);
                requesters[1..].iter().fold(
                    Diagnostic::error(
                        "missing-fact",
                        format!(
                            "`{provider}` does not say how many requesters it serves at once, and {} take a \
                             channel: {names}",
                            requesters.len()
                        ),
                        Label::new(requesters[0].1, "takes a channel here"),
                        format!("describe `{provider}`'s `channels`, or bind a requester to another provider"),
                    ),
                    |d, (_, span)| d.with_secondary(Label::new(*span, "and here")),
                )
            }
            Self::Overlap { first, second } => Diagnostic::error(
                "infeasible-configuration",
                format!(
                    "`{}` and `{}` both own bytes of `{}` and `{}`, and neither composes the other",
                    first.0, second.0, first.1.name, second.1.name
                ),
                Label::new(first.1.span, "owned here"),
                "bind one of the two requesters to a provider that does not own these bytes, or describe one \
                 provider as composing the other",
            )
            .with_secondary(Label::new(second.1.span, "and here")),
            Self::RegionUnsized {
                provider,
                region,
                against,
            } => Diagnostic::error(
                "missing-fact",
                format!(
                    "`{provider}`'s region `{}` has no size, and `{against}` owns bytes above its base, so whether \
                     they overlap is unknown",
                    region.name
                ),
                Label::new(region.span, "no size"),
                "write the region's `(size …)`",
            ),
            Self::ChannelsContradicted { provider, span } => Diagnostic::error(
                "invalid-description",
                format!(
                    "`{provider}` offers a consumed function and declares no channel to serve it with"
                ),
                Label::new(*span, "declared here"),
                "describe `channels` as one or more, or withdraw the consumed offer",
            ),
            Self::RegionMalformed {
                provider,
                span,
                detail,
            } => Diagnostic::error(
                "invalid-description",
                format!("`{provider}`: {detail}"),
                Label::new(*span, "written here"),
                "write a region as `(region NAME (base ADDRESS) (size AMOUNT UNIT))`",
            ),
            Self::RegionPastArithmetic { provider, span } => Diagnostic::error(
                "unsupported-profile",
                format!("`{provider}`: a region whose bytes the exact arithmetic cannot count"),
                Label::new(*span, "written here"),
                "write a smaller region",
            ),
            Self::Unconnected {
                requester,
                span,
                provider,
                declared,
                system,
            } => Diagnostic::error(
                "infeasible-configuration",
                format!(
                    "`{requester}` is bound to `{provider}`, which is not part of `{system}`'s hardware"
                ),
                Label::new(*span, "required here"),
                format!(
                    "compose `{provider}` into `{system}`'s platform, or bind `{requester}` to a provider its \
                     platform composes"
                ),
            )
            .with_secondary(Label::new(*declared, "declared here, outside it")),
            Self::TwoTimeBases {
                requester,
                span,
                device,
                reader,
                read,
                read_device,
            } => Diagnostic::error(
                "infeasible-configuration",
                format!(
                    "`{requester}` programs a deadline in `{device}`'s time, and `{reader}`, which it reaches, reads \
                     time from `{read_device}`: one deadline, two time bases"
                ),
                Label::new(*span, "the deadline required here"),
                format!(
                    "bind the deadline and the time it is read in at one device; a counter read for another purpose \
                     belongs to a requester `{requester}` does not reach"
                ),
            )
            .with_secondary(Label::new(*read, "the time read here")),
            Self::MapContradiction {
                system,
                memory,
                other,
            } => Diagnostic::error(
                "invalid-description",
                if other.1.is_memory() {
                    format!(
                        "`{system}`'s placement regions `{}` and `{}` claim the same bytes",
                        memory.1.name, other.1.name
                    )
                } else {
                    format!(
                        "`{}`'s region `{}` lies in `{}`'s placement memory `{}`, in `{system}`'s map",
                        other.0, other.1.name, memory.0, memory.1.name
                    )
                },
                Label::new(other.1.span, "overlapping here"),
                "correct the region map: mark the range `(reserved true)`, or move it out of the memory",
            )
            .with_secondary(Label::new(memory.1.span, "placement memory")),
            Self::MemoryAllHoles { system, memory } => Diagnostic::error(
                "invalid-description",
                format!(
                    "`{}`'s placement memory `{}` is reserved in every byte, so `{system}` has nothing to place in it",
                    memory.0, memory.1.name
                ),
                Label::new(memory.1.span, "every byte reserved"),
                "correct the region map: shrink the reserved ranges, or drop `(executable true)`",
            ),
            Self::PlacementUnsized {
                system,
                sizeless,
                against,
            } => Diagnostic::error(
                "missing-fact",
                format!(
                    "`{}`'s region `{}` has no size, and `{against}` has a region above its base, so whether `{system}`'s \
                     placement memory overlaps it is unknown",
                    sizeless.0, sizeless.1.name
                ),
                Label::new(sizeless.1.span, "no size"),
                "write the region's `(size …)`",
            ),
            Self::MemoryUnsized { system, memory } => Diagnostic::error(
                "missing-fact",
                format!(
                    "`{}`'s placement memory `{}` has no size, so nothing can be laid out in it in `{system}`",
                    memory.0, memory.1.name
                ),
                Label::new(memory.1.span, "no size"),
                "write the region's `(size …)`",
            ),
            Self::HoleUnsized {
                system,
                memory,
                hole,
            } => Diagnostic::error(
                "missing-fact",
                format!(
                    "`{}`'s reserved range `{}` has no size, and with each such range taken to reach the end of `{}`'s \
                     placement memory `{}`, `{system}` has no free byte to place in",
                    hole.0, hole.1.name, memory.0, memory.1.name
                ),
                Label::new(hole.1.span, "no size"),
                "write the reserved range's `(size …)`",
            )
            .with_secondary(Label::new(memory.1.span, "placement memory")),
            Self::PlacementIllegal {
                system,
                span,
                region,
                why,
            } => Diagnostic::error(
                "infeasible-configuration",
                format!("`{system}`'s image cannot be placed in `{region}`: {why}"),
                Label::new(*span, "the system placed"),
                "place the image in an executable, unreserved region of the system's hardware",
            ),
            Self::LevelUnsupported {
                requester,
                span,
                stated,
                level,
                system,
            } => Diagnostic::error(
                "unsupported-profile",
                format!(
                    "`{requester}` states it calls from {}, and `{system}` runs its callers at `{level}` alone",
                    quoted(stated)
                ),
                Label::new(*span, "stated here"),
                format!("drop the stated level, or state `{level}`; another level is a later profile's"),
            ),
            Self::OutOfLevel {
                requester,
                span,
                provider,
                declared,
                level,
                offered,
            } => Diagnostic::error(
                "infeasible-configuration",
                format!(
                    "`{requester}` runs at `{level}`, and `{provider}` is reachable from {} alone",
                    quoted(offered)
                ),
                Label::new(*span, "required here"),
                format!(
                    "bind a provider reachable at `{level}`, or a mediating adapter where the requirer allows one"
                ),
            )
            .with_secondary(Label::new(*declared, "declared here")),
            Self::LevelUnstated {
                requester,
                span,
                provider,
                declared,
                absent,
            } => Diagnostic::error(
                self.code(),
                if *absent {
                    format!("`{provider}` is declared reachable from no level, and `{requester}` is bound to it")
                } else {
                    format!(
                        "`{provider}` does not say which levels it is reachable from, and `{requester}` is bound to it"
                    )
                },
                Label::new(*span, "required here"),
                format!("describe `{provider}`'s `{REACHABLE_AT_PRIVILEGE}` with the levels it is reachable from"),
            )
            .with_secondary(Label::new(*declared, "declared here")),
            Self::OwnSource {
                requester,
                span,
                provider,
                declared,
                why,
            } => {
                let released = format!(
                    "`{requester}` is bound itself to `{provider}` for `{INTERRUPT_SOURCE}`, so every arrival of \
                     `{provider}` releases it"
                );
                Diagnostic::error(
                    self.code(),
                    match why {
                        Unadmitted::Faster { task, source } => format!(
                            "{released}, and `{provider}`'s `{MIN_ARRIVAL_SEPARATION}` {source} is shorter than its \
                             `min-separation` {task}: the analysis admits a task released on every arrival of a \
                             source only where T ≤ T_s (its condition 5)"
                        ),
                        Unadmitted::Absent => format!(
                            "{released}, and `{provider}` declares its `{MIN_ARRIVAL_SEPARATION}` absent, a source \
                             with no bound, which the analysis refuses (its condition 3)"
                        ),
                        Unadmitted::PastArithmetic => format!(
                            "{released}, and comparing `{provider}`'s `{MIN_ARRIVAL_SEPARATION}` with its \
                             `min-separation` lies past the exact arithmetic"
                        ),
                        Unadmitted::Unstated => format!(
                            "{released}, and `{provider}` states no value for its `{MIN_ARRIVAL_SEPARATION}`, which \
                             the analysis needs to admit the task"
                        ),
                    },
                    Label::new(*span, "required here"),
                    match why {
                        Unadmitted::Unstated => format!(
                            "state `{provider}`'s `{MIN_ARRIVAL_SEPARATION}` as a value, at least the task's \
                             `min-separation`"
                        ),
                        _ => format!(
                            "bind the task's `{INTERRUPT_SOURCE}` to a source whose `{MIN_ARRIVAL_SEPARATION}` is at \
                             least its `min-separation`"
                        ),
                    },
                )
                .with_secondary(Label::new(*declared, "declared here"))
            }
            Self::FunctionSplit {
                requester,
                function,
                bound,
            } => {
                let at = |(provider, facts): &(String, Vec<(String, Span)>)| {
                    format!(
                        "{} at `{provider}`",
                        facts
                            .iter()
                            .map(|(f, _)| format!("`{f}`"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                };
                bound.iter().flat_map(|(_, facts)| facts).skip(1).fold(
                    Diagnostic::error(
                        "infeasible-configuration",
                        format!(
                            "`{requester}`'s `{function}` facts are bound to {} providers: {}",
                            bound.len(),
                            bound.iter().map(at).collect::<Vec<_>>().join("; ")
                        ),
                        Label::new(bound[0].1[0].1, "required here"),
                        format!(
                            "bind `{requester}`'s `{function}` to one provider that offers it whole, or to an adapter that \
                             does"
                        ),
                    ),
                    |d, (_, span)| d.with_secondary(Label::new(*span, "and here")),
                )
            }
            Self::FunctionIncomplete {
                requester,
                function,
                completions,
                ..
            } => {
                let (provider, parts) = &completions[0];
                let facts = parts
                    .iter()
                    .map(|(f, _, _)| format!("`{f}`"))
                    .collect::<Vec<_>>()
                    .join(", ");
                Diagnostic::error(
                    self.code(),
                    format!(
                        "`{requester}`'s `{function}` facts are bound to several providers, and `{provider}` meets every \
                         one of them but {facts}, which it does not describe"
                    ),
                    Label::new(parts[0].1, "required here"),
                    format!("describe {facts} at `{provider}`, and bind `{requester}`'s `{function}` to it whole"),
                )
            }
            Self::QualifierUnmet {
                requester,
                fact,
                span,
                provider,
                declared,
                verdict,
            } => Diagnostic::error(
                self.code(),
                match verdict {
                    Verdict::Refused | Verdict::Absent => format!(
                        "`{requester}` requires `{fact}` of every function it relies on, and `{provider}` does not \
                         meet it"
                    ),
                    _ => format!(
                        "`{requester}` requires `{fact}` of every function it relies on, and `{provider}` does not say"
                    ),
                },
                Label::new(*span, "required here"),
                format!("bind the function to a provider that meets `{fact}`, or describe `{provider}`'s `{fact}`"),
            )
            .with_secondary(Label::new(*declared, "declared here")),
        }
    }
}

/// The function the vocabulary gives `fact`, if it declares the fact and names one.
fn function_of<'v>(vocabulary: &'v Vocabulary, fact: &str) -> Option<&'v Function> {
    vocabulary.fact(fact).and_then(|f| f.function.as_ref())
}

/// Whether `requirement` binds no provider at all: `reachable-at-privilege`, which rule 9 judges against the level the
/// system is given, never at a provider of its own (model §8 rules 9 and 13, leaf `M4.1.9`).
#[must_use]
pub fn is_level(requirement: &Requirement) -> bool {
    requirement.fact() == REACHABLE_AT_PRIVILEGE
}

/// Whether `requirement` is one of rule 13's qualifiers: a requirement on an `each` fact, `reachable-at-privilege` apart,
/// which is rule 9's.
#[must_use]
pub fn is_qualifier(requirement: &Requirement, vocabulary: &Vocabulary) -> bool {
    requirement.fact() != REACHABLE_AT_PRIVILEGE
        && function_of(vocabulary, requirement.fact()) == Some(&Function::Each)
}

/// Whether `requester` relies on a function: it, or a requester it reaches through `uses` and `needs` at any depth,
/// requires a fact of a function. Every requirement of a complete assignment is bound, so such a fact is bound, and a
/// qualifier of `requester` then "makes no binding of its own" (model §8 rule 13); a requester relying on no function
/// binds its qualifier as any requirement. The search's pairs and a record read back both read it (leaf `M4.1.8`).
#[must_use]
pub fn relies_on_a_function(
    requester: &str,
    requesters: &[Requester],
    vocabulary: &Vocabulary,
) -> bool {
    let by_name: BTreeMap<&str, &Requester> =
        requesters.iter().map(|r| (r.name.as_str(), r)).collect();
    let mut seen: BTreeSet<&str> = BTreeSet::from([requester]);
    let mut todo = vec![requester];
    while let Some(at) = todo.pop() {
        let Some(r) = by_name.get(at) else {
            continue;
        };
        if r.requirements.iter().any(|q| {
            !matches!(q, Requirement::Statement { .. })
                && matches!(function_of(vocabulary, q.fact()), Some(Function::Of(_)))
        }) {
            return true;
        }
        for named in r.uses.iter().chain(&r.needs) {
            if seen.insert(named.as_str()) {
                todo.push(named.as_str());
            }
        }
    }
    false
}

fn quoted(levels: &[String]) -> String {
    levels
        .iter()
        .map(|l| format!("`{l}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

fn listed(requesters: &[(String, Span)]) -> String {
    requesters
        .iter()
        .map(|(name, _)| format!("`{name}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// What a realization owes because of an assignment.
#[derive(Debug, Clone, PartialEq)]
pub enum Obligation {
    /// A node with two or more users takes a channel (model §8 rule 4): its realization serves them all through it,
    /// so it owes them the shared semantics and a bounded overhead (`ROADMAP.md` §5.4).
    Multiplex {
        /// The node: a service, a policy, or a block or a platform bound for requirements of its own.
        node: String,
        /// Its users, two or more: the system's tasks that reach it, then the routines whose interrupts run it.
        users: Vec<Caller>,
        /// Each provider whose channel it takes, by name.
        providers: Vec<String>,
    },
    /// A provider the assignment relies on for an interrupt: the path from it to the system's core, with its latency,
    /// is the catalog's to state and the plan's to enable (leaf `M3.3.3`; `decision_runtime-analysis-variant.md`'s
    /// *enabled sources*).
    Route {
        /// The system.
        system: String,
        /// The provider.
        provider: String,
        /// What the interrupt is.
        interrupt: Interrupt,
        /// The requesters relying on it.
        requesters: Vec<String>,
        /// The level its interrupt is delivered at, when the system is given one.
        level: Option<String>,
    },
    /// Placement memory of a system and the reserved ranges in it, which the plan lays the image out around (`M4.1`)
    /// and the build verifies (`M4.4`, F20).
    Placement {
        /// The system.
        system: String,
        /// The provider offering the memory.
        provider: String,
        /// The placement region.
        region: String,
        /// Each hole in it: its provider and its region, one with no size reaching the region's end.
        holes: Vec<(String, String)>,
    },
}

/// What one system's memory map says of placement, judged once for the system (model §8 rule 10).
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Map {
    /// Two claims on the same bytes, or memory all holes: `invalid-description`.
    pub refused: Vec<Finding>,
    /// A size the judgment cannot do without: `missing-fact`.
    pub unknowns: Vec<Finding>,
    /// Each placement region with its holes.
    pub obligations: Vec<Obligation>,
}

/// What a node of an initialization graph is (model §8 rule 12).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Node {
    /// A provider: a block, a platform, an adapter or a record.
    Provider,
    /// A service.
    Service,
    /// Any other requester the system serves but its own level: a policy.
    Other,
}

/// Why one node is initialised before another.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Because {
    /// The later is bound to the earlier for a requirement on this fact.
    Binding(String),
    /// The later names the earlier by `uses` or `needs`.
    Named,
}

/// The initialization graph of one system's assignment (leaf `M3.3.7`): derived for `M4.1`'s plan, refusing nothing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Initialization {
    /// Each node and what it is, by name.
    pub nodes: Vec<(String, Node)>,
    /// Each edge, earlier to later, with its reason, sorted.
    pub edges: Vec<(String, String, Because)>,
    /// Each strongly connected component of more than one node, its nodes by name: a unit the engine initialises
    /// together, in one phase, its members in their texts' order (model §8 rule 12; the runtime plan's §2.7).
    pub components: Vec<Vec<String>>,
}

/// What an [`Obligation::Route`]'s interrupt is.
#[derive(Debug, Clone, PartialEq)]
pub enum Interrupt {
    /// A source, bound through `interrupt-source` at `true`, with what it states of its own `min-arrival-separation`
    /// — `None` when it does not name it; model §8 rule 13 keeps a requester's `interrupt` facts at one provider, so a
    /// separation the requester requires is this source's.
    Source {
        /// Its own separation.
        separation: Option<State>,
    },
    /// A timer, bound through a consumed fact at `true`: an alarm the requester programs.
    Timer,
}

/// One route's interrupt routine, by its provider and kind (model §8 rule 8): what an [`Obligation::Route`] names.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Routine {
    /// The route's provider.
    pub provider: String,
    /// Whether the route is a timer route; a source otherwise.
    pub timer: bool,
}

/// Who can call a node at run time (model §8 rule 4).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Caller {
    /// The jobs of a task of the system that reaches the node, by the task's name.
    Task(String),
    /// The routine of a route of the system whose interrupt runs the node.
    Routine(Routine),
}

/// A context a task's or a node's code runs in (model §8 rule 4; `docs/decisions/decision_runtime-plan.md` §2.6).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Context {
    /// A job: a task's own, and every node a task reaches.
    Job,
    /// The routine of a route's interrupt, for everything that interrupt runs.
    Routine(Routine),
    /// Initialisation, every node's.
    Initialisation,
}

/// The contexts one system's assignment runs its code in (model §8 rule 4; `docs/decisions/decision_runtime-plan.md`
/// §2.6), derived for the multiplexer and for `M4.1`'s plan, refusing nothing.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Contexts {
    /// Each task and each node, by name, with every context its code runs in.
    pub of: BTreeMap<String, BTreeSet<Context>>,
    /// Each route's routine, with what its interrupt runs: the code rule 4 names and every node that code reaches.
    pub runs: BTreeMap<Routine, BTreeSet<String>>,
}

/// What a requester reaches over one assignment (model §8 rule 4): its direct names gathered once, and each closure
/// computed once, since the search judges every assignment it tries.
struct Reach<'a> {
    direct: BTreeMap<&'a str, BTreeSet<&'a str>>,
    closures: RefCell<BTreeMap<&'a str, Rc<BTreeSet<&'a str>>>>,
}

impl<'a> Reach<'a> {
    /// Everything `from` reaches, at any depth; itself only where a path returns to it.
    fn of(&self, from: &'a str) -> Rc<BTreeSet<&'a str>> {
        if let Some(known) = self.closures.borrow().get(from) {
            return Rc::clone(known);
        }
        let mut seen = BTreeSet::new();
        let mut todo = vec![from];
        while let Some(at) = todo.pop() {
            for next in self.direct.get(at).into_iter().flatten() {
                if seen.insert(*next) {
                    todo.push(next);
                }
            }
        }
        let seen = Rc::new(seen);
        self.closures.borrow_mut().insert(from, Rc::clone(&seen));
        seen
    }
}

/// What one assignment breaks, what it leaves unknown, and what it obliges.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Report {
    /// Rules the assignment breaks: `infeasible-configuration` once every assignment breaks one.
    pub conflicts: Vec<Finding>,
    /// Facts a rule needs and the description does not give: `missing-fact` once no assignment is free of conflicts.
    pub unknowns: Vec<Finding>,
    /// What the assignment obliges a realization to do.
    pub obligations: Vec<Obligation>,
}

impl Report {
    /// Whether the assignment holds together.
    #[must_use]
    pub fn holds(&self) -> bool {
        self.conflicts.is_empty() && self.unknowns.is_empty()
    }
}

/// The joint rules over one description's providers and requesters.
#[derive(Debug, Clone)]
pub struct Joint<'v> {
    vocabulary: &'v Vocabulary,
    system: Option<System>,
    held: BTreeMap<String, Held>,
    /// The levels each requester states it calls from, and where.
    stated: BTreeMap<String, (BTreeSet<String>, Span)>,
    requesters: BTreeMap<String, Kind>,
    /// The declarations each requester names by `uses` or `needs`.
    names: BTreeMap<String, BTreeSet<String>>,
    /// Each provider as read, which rule 13 judges requirements at.
    providers: BTreeMap<String, Provider>,
    /// Each requester's requirements on a qualifier but `reachable-at-privilege`, rule 9's (rule 13).
    qualified: BTreeMap<String, Vec<Requirement>>,
    /// Each engine adapter's instance the rules may meet, by name, with the provider it programs (`M3.2.3.1`).
    instances: BTreeMap<String, String>,
    /// The instances whose kind's cost runs their requester's code (rule 4, the adapters record §9).
    delivering: BTreeSet<String>,
    /// Each sporadic task's `min-separation`, by its name (rule 16).
    separations: BTreeMap<String, Quantity>,
}

impl<'v> Joint<'v> {
    /// Read what the rules need of each provider — its channels, its regions, the providers it composes — and what each
    /// requester names.
    ///
    /// # Errors
    ///
    /// Every provider-level finding: a consumed function offered `true` with `channels` absent or zero; a region
    /// that names no byte range, or one past the arithmetic.
    pub fn new(
        providers: &[(Provider, &Form)],
        requesters: &[Requester],
        vocabulary: &'v Vocabulary,
    ) -> Result<Self, Vec<Finding>> {
        let mut findings = Vec::new();
        let mut held = BTreeMap::new();
        for (provider, decl) in providers {
            let channels = match provider.state("channels") {
                None => Channels::Undescribed,
                Some(State::Valued(values)) => match values.first() {
                    Some(Value::Count(n)) => Channels::Count(*n),
                    _ => Channels::Unknown,
                },
                Some(State::Absent) => Channels::Count(0),
                Some(State::Unvalued | State::Overflow) => Channels::Unknown,
            };
            if channels == Channels::Count(0) && offers_a_consumed_function(provider, vocabulary) {
                findings.push(Finding::ChannelsContradicted {
                    provider: provider.name.clone(),
                    span: provider
                        .facts
                        .get("channels")
                        .and_then(|s| s.spans.first().copied())
                        .unwrap_or(provider.span),
                });
            }
            let regions = regions(provider, decl, &mut findings);
            held.insert(
                provider.name.clone(),
                Held {
                    span: provider.span,
                    hardware: is_hardware(decl),
                    separation: provider.state(MIN_ARRIVAL_SEPARATION).cloned(),
                    level: provider.state(REACHABLE_AT_PRIVILEGE).cloned(),
                    channels,
                    regions,
                    composes: used_names(decl).into_iter().collect(),
                },
            );
        }
        if !findings.is_empty() {
            return Err(findings);
        }
        let stated = requesters
            .iter()
            .filter_map(|r| {
                r.requirements.iter().find_map(|q| match q {
                    Requirement::Constraint {
                        fact,
                        value: Value::Set(levels),
                        span,
                        ..
                    } if fact == REACHABLE_AT_PRIVILEGE => {
                        Some((r.name.clone(), (levels.clone(), *span)))
                    }
                    _ => None,
                })
            })
            .collect();
        let qualified = requesters
            .iter()
            .filter_map(|r| {
                let qualifiers: Vec<Requirement> = r
                    .requirements
                    .iter()
                    .filter(|q| {
                        q.fact() != REACHABLE_AT_PRIVILEGE
                            && function_of(vocabulary, q.fact()) == Some(&Function::Each)
                    })
                    .cloned()
                    .collect();
                (!qualifiers.is_empty()).then(|| (r.name.clone(), qualifiers))
            })
            .collect();
        Ok(Self {
            vocabulary,
            system: None,
            held,
            stated,
            qualified,
            providers: providers
                .iter()
                .map(|(provider, _)| (provider.name.clone(), provider.clone()))
                .collect(),
            requesters: requesters
                .iter()
                .map(|r| (r.name.clone(), r.kind))
                .collect(),
            names: requesters
                .iter()
                .map(|r| {
                    (
                        r.name.clone(),
                        r.uses.iter().chain(&r.needs).cloned().collect(),
                    )
                })
                .collect(),
            instances: BTreeMap::new(),
            delivering: BTreeSet::new(),
            separations: requesters
                .iter()
                .filter_map(|r| r.separation.map(|q| (r.name.clone(), q)))
                .collect(),
        })
    }

    /// The rules with the instances an assignment may bind (`docs/specs/adapters/decision_engine-adapters.md` §4): each
    /// a provider for its offers — no hardware, owning no region and composing nothing, so ownership never compares it
    /// and the initialization graph keeps its edges — and a requester for what its kind requires beneath, a caller at the
    /// level it states.
    #[must_use]
    pub fn with_instances(mut self, instances: &[Instance]) -> Self {
        for instance in instances {
            let name = instance.provider.name.clone();
            let channels = match instance.provider.state("channels") {
                Some(State::Valued(values)) => match values.first() {
                    Some(Value::Count(n)) => Channels::Count(*n),
                    _ => Channels::Unknown,
                },
                None => Channels::Undescribed,
                Some(_) => Channels::Unknown,
            };
            let level = instance.provider.state(REACHABLE_AT_PRIVILEGE).cloned();
            if let Some(State::Valued(values)) = &level {
                if let Some(Value::Set(levels)) = values.first() {
                    self.stated
                        .insert(name.clone(), (levels.clone(), instance.provider.span));
                }
            }
            self.held.insert(
                name.clone(),
                Held {
                    span: instance.provider.span,
                    hardware: false,
                    separation: None,
                    level,
                    channels,
                    regions: Vec::new(),
                    composes: BTreeSet::new(),
                },
            );
            self.providers
                .insert(name.clone(), instance.provider.clone());
            self.requesters.insert(name.clone(), Kind::Adapter);
            self.names.insert(name.clone(), BTreeSet::new());
            if instance.delivers {
                self.delivering.insert(name.clone());
            }
            self.instances.insert(name, instance.over.clone());
        }
        self
    }

    /// `system` as one assignment's rules read it: an instance is served when a requester it serves is, at any depth
    /// (record §4, rule 7: it takes the system of the requesters bound to it).
    fn serving(&self, system: &System, assignment: &[Binding]) -> System {
        let mut system = system.clone();
        loop {
            let reached: Vec<String> = assignment
                .iter()
                .filter(|b| self.instances.contains_key(&b.provider) && system.serves(&b.requester))
                .map(|b| b.provider.clone())
                .filter(|p| !system.reached.contains(p))
                .collect();
            if reached.is_empty() {
                return system;
            }
            system.reached.extend(reached);
        }
    }

    /// The device `provider` stands for: itself, or the provider an instance programs.
    fn device<'a>(&'a self, provider: &'a str) -> &'a str {
        self.instances
            .get(provider)
            .map_or(provider, String::as_str)
    }

    /// The rules for one system's assignment: connectivity and routes judge the bindings of the requesters it serves.
    /// Without a system, an assignment is judged for channels and ownership alone.
    #[must_use]
    pub fn for_system(mut self, system: System) -> Self {
        self.system = Some(system);
        self
    }

    /// Judge one complete assignment. Its routes are reported whatever else the rules find (rule 8); its multiplexers
    /// (rule 4) only where it holds, since only an assignment that holds is realized.
    #[must_use]
    pub fn check(&self, assignment: &[Binding]) -> Report {
        let mut report = Report::default();
        self.channels(assignment, &mut report);
        self.ownership(assignment, &mut report);
        self.functions(assignment, &mut report);
        self.qualifiers(assignment, &mut report);
        if let Some(system) = &self.system {
            let system = &self.serving(system, assignment);
            self.connectivity(system, assignment, &mut report);
            self.routes(system, assignment, &mut report);
            self.time_base(system, assignment, &mut report);
            if let Some(level) = &system.level {
                self.privilege(system, level, assignment, &mut report);
            } else {
                // An instance is a caller at its kind's level, whether or not the plan gives its system one (record §4,
                // rule 9): its own bindings are judged at that level.
                for (instance, (levels, _)) in self
                    .stated
                    .iter()
                    .filter(|(name, _)| self.instances.contains_key(*name) && system.serves(name))
                {
                    let own: Vec<Binding> = assignment
                        .iter()
                        .filter(|b| &b.requester == instance)
                        .cloned()
                        .collect();
                    for level in levels {
                        self.privilege(system, level, &own, &mut report);
                    }
                }
            }
            self.own_sources(system, assignment, &mut report);
            if let Some(region) = &system.placement {
                self.placed(system, region, &mut report);
            }
            // Only a realization discharges a multiplexer, and an assignment the rules refuse is realized by none; the
            // search judges every assignment it tries and keeps the obligations of the one that holds.
            if report.holds() {
                self.multiplexing(system, assignment, &mut report);
            }
        }
        report
    }

    /// Model §8 rule 13 (`SR-H14`): one requester's bindings for the facts of one function are to one provider. Where
    /// they are to several, the split is the assignment's conflict when some provider would meet every one of them, or
    /// when none of them could; an unknown when none meets them all and one refutes none, so that describing what it
    /// leaves undescribed would make it whole (`SR-H18`'s `missing-fact`).
    fn functions(&self, assignment: &[Binding], report: &mut Report) {
        let mut split: BTreeMap<(&str, &str), BTreeMap<&str, Vec<&Requirement>>> = BTreeMap::new();
        for b in assignment {
            if let Some(Function::Of(function)) = function_of(self.vocabulary, b.requirement.fact())
            {
                split
                    .entry((b.requester.as_str(), function.as_str()))
                    .or_default()
                    .entry(b.provider.as_str())
                    .or_default()
                    .push(&b.requirement);
            }
        }
        for ((requester, function), by_provider) in split {
            if by_provider.len() < 2 {
                continue;
            }
            let all: Vec<&Requirement> = by_provider.values().flatten().copied().collect();
            let verdicts = |name: &str| -> Option<Vec<(&Requirement, Verdict)>> {
                let p = self.providers.get(name)?;
                Some(
                    all.iter()
                        .map(|q| (*q, judge(p, q, self.vocabulary)))
                        .collect(),
                )
            };
            let judged: Vec<(&str, Vec<(&Requirement, Verdict)>)> = by_provider
                .keys()
                .filter_map(|name| verdicts(name).map(|v| (*name, v)))
                .collect();
            let whole = judged
                .iter()
                .any(|(_, v)| v.iter().all(|(_, verdict)| *verdict == Verdict::Satisfied));
            let completions: Vec<(String, Gaps)> = judged
                .iter()
                .filter(|(_, v)| {
                    !v.iter()
                        .any(|(_, verdict)| matches!(verdict, Verdict::Refused | Verdict::Absent))
                })
                .map(|(name, v)| {
                    let parts = v
                        .iter()
                        .filter(|(_, verdict)| *verdict != Verdict::Satisfied)
                        .map(|(q, verdict)| (q.fact().to_string(), q.span(), *verdict))
                        .collect();
                    ((*name).to_string(), parts)
                })
                .collect();
            if whole || completions.is_empty() {
                report.conflicts.push(Finding::FunctionSplit {
                    requester: requester.to_string(),
                    function: function.to_string(),
                    bound: by_provider
                        .iter()
                        .map(|(provider, qs)| {
                            (
                                (*provider).to_string(),
                                qs.iter()
                                    .map(|q| (q.fact().to_string(), q.span()))
                                    .collect(),
                            )
                        })
                        .collect(),
                });
            } else {
                let past_arithmetic = completions
                    .iter()
                    .any(|(_, parts)| parts.iter().any(|(_, _, v)| *v == Verdict::Unsupported));
                report.unknowns.push(Finding::FunctionIncomplete {
                    requester: requester.to_string(),
                    function: function.to_string(),
                    completions,
                    past_arithmetic,
                });
            }
        }
    }

    /// Rule 13's qualifiers: each requirement a requester makes of an `each` fact — `reachable-at-privilege` is rule 9's
    /// — is judged at every provider bound for its functions and for those of each requester it relies on through
    /// `uses` and `needs`, at any depth. A requester relying on no function bound binds its qualifier as any requirement.
    fn qualifiers(&self, assignment: &[Binding], report: &mut Report) {
        let mut bound: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for b in assignment {
            if matches!(
                function_of(self.vocabulary, b.requirement.fact()),
                Some(Function::Of(_))
            ) {
                bound
                    .entry(b.requester.as_str())
                    .or_default()
                    .insert(b.provider.as_str());
            }
        }
        for (requester, qualifiers) in &self.qualified {
            let mut reached: BTreeSet<&str> = BTreeSet::from([requester.as_str()]);
            let mut todo = vec![requester.as_str()];
            while let Some(at) = todo.pop() {
                for named in self.names.get(at).into_iter().flatten() {
                    if self.requesters.contains_key(named) && reached.insert(named.as_str()) {
                        todo.push(named.as_str());
                    }
                }
            }
            let relied: BTreeSet<&str> = reached
                .iter()
                .filter_map(|r| bound.get(r))
                .flatten()
                .copied()
                .collect();
            for q in qualifiers {
                for name in &relied {
                    let Some(provider) = self.providers.get(*name) else {
                        continue;
                    };
                    let verdict = judge(provider, q, self.vocabulary);
                    if verdict == Verdict::Satisfied {
                        continue;
                    }
                    let finding = Finding::QualifierUnmet {
                        requester: requester.clone(),
                        fact: q.fact().to_string(),
                        span: q.span(),
                        provider: (*name).to_string(),
                        declared: provider.span,
                        verdict,
                    };
                    if finding.code() == "infeasible-configuration" {
                        report.conflicts.push(finding);
                    } else {
                        report.unknowns.push(finding);
                    }
                }
            }
        }
    }

    /// The regions of `system`'s hardware, each with its provider, in the order the providers are named.
    fn hardware_regions<'s>(&'s self, system: &System) -> Vec<(&'s str, &'s Region)> {
        system
            .hardware
            .iter()
            .filter_map(|name| self.held.get_key_value(name))
            .flat_map(|(name, h)| h.regions.iter().map(move |r| (name.as_str(), r)))
            .collect()
    }

    /// A placement given to the plan is placement memory of the system's hardware.
    fn placed(&self, system: &System, region: &str, report: &mut Report) {
        let why = match self
            .hardware_regions(system)
            .iter()
            .find(|(_, r)| r.name == region)
        {
            None => Some("it is no region of the system's hardware"),
            Some((_, r)) if !r.is_memory() => Some("it is not executable, unreserved memory"),
            Some(_) => None,
        };
        if let Some(why) = why {
            report.conflicts.push(Finding::PlacementIllegal {
                system: system.name.clone(),
                span: system.span,
                region: region.to_string(),
                why: why.to_string(),
            });
        }
    }

    /// Model §8 rule 12: the initialization graph of `system`'s assignment. Its nodes are the providers bound for the
    /// requesters it serves — closed over bindings, so an adapter's or a block's own bindings count — the services and
    /// policies it serves, each block or platform it serves that is bound for requirements of its own (leaf `M4.1.15`:
    /// a requester like any other, so its code may run in an interrupt, `docs/decisions/decision_runtime-plan.md` §2), and
    /// each block or platform a served task, service or policy names by `uses` or `needs`, a call (leaf `M4.1.18`); its
    /// edges run from
    /// a provider to each requester bound to it, whatever composes what, but to itself, and from each node a service or
    /// a policy names by `uses` or `needs`, a block or a platform included, to that namer — hardware's own names giving
    /// none. Nothing is refused (composition alone gives no edge, leaf `M4.1.16`; a named block an edge, `M4.1.17`).
    #[must_use]
    pub fn initialization(&self, system: &System, assignment: &[Binding]) -> Initialization {
        let system = &self.serving(system, assignment);
        let active = active(system, assignment);
        let nodes = self.nodes(system, assignment, &active);
        let mut edges: BTreeSet<(String, String, Because)> = BTreeSet::new();
        for b in assignment
            .iter()
            .filter(|b| active.contains(b.requester.as_str()))
        {
            let (from, to) = (b.provider.as_str(), b.requester.as_str());
            if nodes.contains_key(to) && from != to {
                edges.insert((
                    from.to_string(),
                    to.to_string(),
                    Because::Binding(b.requirement.fact().to_string()),
                ));
            }
        }
        for (user, named) in &self.names {
            if !nodes.contains_key(user) || self.is_hardware(user) {
                continue;
            }
            for used in named
                .iter()
                .filter(|n| *n != user && nodes.contains_key(*n))
            {
                edges.insert((used.clone(), user.clone(), Because::Named));
            }
        }
        let components = components(&nodes, &edges);
        Initialization {
            nodes: nodes.into_iter().collect(),
            edges: edges.into_iter().collect(),
            components,
        }
    }

    /// Rule 12's nodes of `system`, as [`Joint::serving`] reads it, each with what it is: the providers `active`'s
    /// bindings name, the requesters the system serves but its own level, its tasks and hardware bound for nothing, and
    /// the hardware a served task, service or policy names.
    fn nodes(
        &self,
        system: &System,
        assignment: &[Binding],
        active: &BTreeSet<&str>,
    ) -> BTreeMap<String, Node> {
        let kind = |name: &str| self.requesters.get(name).copied();
        // Hardware is a node as a requester only where it is bound for requirements of its own (leaf `M4.1.15`).
        let bound_for_its_own = |name: &str| assignment.iter().any(|b| b.requester == name);
        let is_node_requester = |name: &str| {
            name != system.name
                && !matches!(kind(name), Some(Kind::Task))
                && system.serves(name)
                && (!self.is_hardware(name) || bound_for_its_own(name))
        };
        let mut nodes: BTreeMap<String, Node> = BTreeMap::new();
        for b in assignment
            .iter()
            .filter(|b| active.contains(b.requester.as_str()))
        {
            nodes.insert(b.provider.clone(), Node::Provider);
        }
        for (name, k) in &self.requesters {
            if is_node_requester(name) && !nodes.contains_key(name) {
                nodes.insert(
                    name.clone(),
                    if self.is_hardware(name) {
                        Node::Provider
                    } else if *k == Kind::Service {
                        Node::Service
                    } else {
                        Node::Other
                    },
                );
            }
        }
        // A block or a platform a served task, service or policy names by `uses` or `needs` is a node: the name is a call
        // (`docs/decisions/decision_runtime-plan.md` §2, leaf `M4.1.18`).
        for (user, named) in &self.names {
            if *user == system.name || self.is_hardware(user) || !system.serves(user) {
                continue;
            }
            for n in named {
                if self.is_hardware(n) && system.serves(n) && !nodes.contains_key(n) {
                    nodes.insert(n.clone(), Node::Provider);
                }
            }
        }
        nodes
    }

    /// What `from` reaches over `assignment` (model §8 rule 4): what it names by `uses` or `needs` and the provider each
    /// of its bindings names, at any depth — but a block's or a platform's own names, which name only hardware and are
    /// never a call (rules 1 and 7). It holds `from` itself only where a path returns to it.
    #[must_use]
    pub fn reaches(&self, from: &str, assignment: &[Binding]) -> BTreeSet<String> {
        self.reach(assignment)
            .of(from)
            .iter()
            .map(|n| (*n).to_string())
            .collect()
    }

    /// Reaching over `assignment`, each requester's direct names gathered once.
    fn reach<'a>(&'a self, assignment: &'a [Binding]) -> Reach<'a> {
        let mut direct: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
        for (requester, named) in self.names.iter().filter(|(r, _)| !self.is_hardware(r)) {
            direct
                .entry(requester.as_str())
                .or_default()
                .extend(named.iter().map(String::as_str));
        }
        for b in assignment {
            direct
                .entry(b.requester.as_str())
                .or_default()
                .insert(b.provider.as_str());
        }
        Reach {
            direct,
            closures: RefCell::default(),
        }
    }

    /// The tasks of `system`, by name.
    fn tasks<'a>(&'a self, system: &'a System) -> impl Iterator<Item = &'a str> + 'a {
        self.requesters
            .iter()
            .filter(move |(name, kind)| {
                **kind == Kind::Task && task_of(name).is_some_and(|(s, _)| s == system.name)
            })
            .map(|(name, _)| name.as_str())
    }

    /// Model §8 rule 4's contexts of one system's assignment (`docs/decisions/decision_runtime-plan.md` §2.6): a job for
    /// each task and every node a task reaches; each route's routine for what its interrupt runs; initialisation for
    /// every node.
    #[must_use]
    pub fn contexts(&self, system: &System, assignment: &[Binding]) -> Contexts {
        let system = &self.serving(system, assignment);
        let nodes = self.nodes(system, assignment, &active(system, assignment));
        self.contexts_of(system, assignment, &nodes, &self.reach(assignment))
    }

    /// [`Joint::contexts`] over `system` as [`Joint::serving`] reads it, given its nodes and its reaching.
    fn contexts_of<'a>(
        &'a self,
        system: &'a System,
        assignment: &'a [Binding],
        nodes: &BTreeMap<String, Node>,
        reach: &Reach<'a>,
    ) -> Contexts {
        let mut of: BTreeMap<String, BTreeSet<Context>> = BTreeMap::new();
        let mut put = |name: &str, context: Context| {
            of.entry(name.to_string()).or_default().insert(context);
        };
        for task in self.tasks(system) {
            put(task, Context::Job);
            for n in reach.of(task).iter().filter(|n| nodes.contains_key(**n)) {
                put(n, Context::Job);
            }
        }
        let mut by_requester: BTreeMap<&str, Vec<&Binding>> = BTreeMap::new();
        for b in assignment {
            by_requester
                .entry(b.requester.as_str())
                .or_default()
                .push(b);
        }
        let bound_to = |requester: &str, provider: &str, fact_ok: &dyn Fn(&Requirement) -> bool| {
            by_requester.get(requester).is_some_and(|bound| {
                bound
                    .iter()
                    .any(|b| b.provider == provider && fact_ok(&b.requirement))
            })
        };
        let channel = |q: &Requirement| takes_a_channel(q, self.vocabulary).is_some();
        let source = |q: &Requirement| q.fact() == INTERRUPT_SOURCE && met_by_true(q);
        let mut runs: BTreeMap<Routine, BTreeSet<String>> = BTreeMap::new();
        for ((x, is_source), requesters) in self.route_map(system, assignment) {
            let timer = !is_source;
            let fact_ok: &dyn Fn(&Requirement) -> bool = if timer { &channel } else { &source };
            // The code the interrupt runs of its own: its provider's, and each node the route names bound to it itself.
            let mut own: BTreeSet<&str> = BTreeSet::new();
            if nodes.contains_key(x) {
                own.insert(x);
            }
            for r in &requesters {
                if nodes.contains_key(*r) && bound_to(r, x, fact_ok) {
                    own.insert(r);
                }
            }
            // A timer route's: each instance the system serves programming an alarm at `x`, which takes its cost there,
            // and the requesters such an instance delivers to, where its kind's cost runs their code.
            if timer {
                for (instance, _) in self.instances.iter().filter(|(name, over)| {
                    *over == x && system.serves(name) && bound_to(name, x, &channel)
                }) {
                    own.insert(instance);
                    if self.delivering.contains(instance) {
                        for r in &requesters {
                            if nodes.contains_key(*r) && bound_to(r, instance, &|_| true) {
                                own.insert(r);
                            }
                        }
                    }
                }
            }
            let mut all = own.clone();
            for n in &own {
                all.extend(reach.of(n).iter().filter(|m| nodes.contains_key(**m)));
            }
            let routine = Routine {
                provider: x.to_string(),
                timer,
            };
            for n in &all {
                put(n, Context::Routine(routine.clone()));
            }
            runs.insert(routine, all.into_iter().map(str::to_string).collect());
        }
        for n in nodes.keys() {
            put(n, Context::Initialisation);
        }
        Contexts { of, runs }
    }

    /// Model §8 rule 4: a node bound for a requirement that takes a channel, an instance aside, with two or more users —
    /// the system's tasks that reach it, and the routines whose interrupts run it, its own timer route's only where
    /// another node that route runs reaches it — is the multiplexer of the channels it takes, an obligation.
    fn multiplexing(&self, system: &System, assignment: &[Binding], report: &mut Report) {
        let taking: Vec<&Binding> = assignment
            .iter()
            .filter(|b| {
                !self.instances.contains_key(&b.requester)
                    && takes_a_channel(&b.requirement, self.vocabulary).is_some()
            })
            .collect();
        if taking.is_empty() {
            return; // nothing takes a channel, so nothing multiplexes one
        }
        let nodes = self.nodes(system, assignment, &active(system, assignment));
        // Each node taking a channel: the providers whose channels it takes, and the providers of its own timer routes,
        // where those are, or where the instance it binds programs.
        let mut takers: BTreeMap<&str, (BTreeSet<&str>, BTreeSet<&str>)> = BTreeMap::new();
        for b in taking
            .into_iter()
            .filter(|b| nodes.contains_key(&b.requester))
        {
            let (providers, own) = takers.entry(b.requester.as_str()).or_default();
            providers.insert(b.provider.as_str());
            own.insert(self.device(&b.provider));
        }
        if takers.is_empty() {
            return;
        }
        let reach = self.reach(assignment);
        let contexts = self.contexts_of(system, assignment, &nodes, &reach);
        let tasks: Vec<(&str, Rc<BTreeSet<&str>>)> = self
            .tasks(system)
            .map(|task| (task, reach.of(task)))
            .collect();
        for (node, (providers, own)) in takers {
            let mut users: Vec<Caller> = tasks
                .iter()
                .filter(|(_, reached)| reached.contains(node))
                .map(|(task, _)| Caller::Task((*task).to_string()))
                .collect();
            for context in contexts.of.get(node).into_iter().flatten() {
                let Context::Routine(routine) = context else {
                    continue;
                };
                // Its own timer route's code delivering to it is no caller; another node's code there calling it is.
                let own_route = routine.timer && own.contains(routine.provider.as_str());
                let called_there = || {
                    contexts.runs.get(routine).is_some_and(|runs| {
                        runs.iter().any(|m| m != node && reach.of(m).contains(node))
                    })
                };
                if !own_route || called_there() {
                    users.push(Caller::Routine(routine.clone()));
                }
            }
            if users.len() >= 2 {
                report.obligations.push(Obligation::Multiplex {
                    node: node.to_string(),
                    users,
                    providers: providers.into_iter().map(str::to_string).collect(),
                });
            }
        }
    }

    /// Model §8 rule 10, once per system: its placement memory, the holes in it, and what contradicts or blocks it.
    #[must_use]
    pub fn map(&self, system: &System) -> Map {
        let regions = self.hardware_regions(system);
        let mut map = Map::default();
        for (i, (owner, memory)) in regions.iter().enumerate() {
            if !memory.is_memory() {
                continue;
            }
            if memory.bytes.is_none() {
                map.unknowns.push(Finding::MemoryUnsized {
                    system: system.name.clone(),
                    memory: ((*owner).to_string(), (*memory).clone()),
                });
            }
            let mut holes = Vec::new();
            for (j, (other_owner, other)) in regions.iter().enumerate() {
                if i == j {
                    continue;
                }
                let (m0, m1) = memory.known();
                let (o0, o1) = other.known();
                if m0 < o1 && o0 < m1 {
                    if other.reserved {
                        holes.push(((*other_owner).to_string(), (*other).clone()));
                    } else if !other.is_memory() || i < j {
                        map.refused.push(Finding::MapContradiction {
                            system: system.name.clone(),
                            memory: ((*owner).to_string(), (*memory).clone()),
                            other: ((*other_owner).to_string(), (*other).clone()),
                        });
                    }
                    continue;
                }
                // Memory with no size is said once, above, for itself: what lies below its base is what stays unknown.
                if other.bytes.is_none() && !other.is_memory() && m0 >= o0 {
                    let finding = Finding::PlacementUnsized {
                        system: system.name.clone(),
                        sizeless: ((*other_owner).to_string(), (*other).clone()),
                        against: (*owner).to_string(),
                    };
                    if !map.unknowns.contains(&finding) {
                        map.unknowns.push(finding);
                    }
                }
            }
            if let Some(size) = memory.bytes {
                // First the bytes surely reserved, which no size repairs; only then each hole to the region's end.
                let end = memory.base + size;
                let surely = holes.iter().map(|(_, r)| r.known());
                let taken = holes
                    .iter()
                    .map(|(_, r)| (r.base, r.bytes.map_or(end, |b| r.base + b)));
                if covered_bytes(memory.base, size, surely) == size {
                    map.refused.push(Finding::MemoryAllHoles {
                        system: system.name.clone(),
                        memory: ((*owner).to_string(), (*memory).clone()),
                    });
                } else if covered_bytes(memory.base, size, taken) == size {
                    for (hole_owner, hole) in holes.iter().filter(|(_, r)| r.bytes.is_none()) {
                        map.unknowns.push(Finding::HoleUnsized {
                            system: system.name.clone(),
                            memory: ((*owner).to_string(), (*memory).clone()),
                            hole: (hole_owner.clone(), hole.clone()),
                        });
                    }
                }
            }
            map.obligations.push(Obligation::Placement {
                system: system.name.clone(),
                provider: (*owner).to_string(),
                region: memory.name.clone(),
                holes: holes.into_iter().map(|(o, r)| (o, r.name)).collect(),
            });
        }
        map
    }

    /// Whether `requester` is a block or platform: hardware needing hardware has no privilege level.
    fn is_hardware(&self, requester: &str) -> bool {
        self.held.get(requester).is_some_and(|h| h.hardware)
    }

    /// `SR-H11` and `SR-H12` over one system's callers, given its level: a caller stating another level first, then each
    /// provider bound for a caller — any provider — reachable at the level and saying so; one finding a caller and provider.
    fn privilege(&self, system: &System, level: &str, assignment: &[Binding], report: &mut Report) {
        let callers: Vec<&Binding> = assignment
            .iter()
            .filter(|b| system.serves(&b.requester) && !self.is_hardware(&b.requester))
            .collect();
        let mut unsupported = BTreeSet::new();
        for b in &callers {
            if let Some((levels, span)) = self.stated.get(&b.requester) {
                if !levels.contains(level) && unsupported.insert(b.requester.as_str()) {
                    report.conflicts.push(Finding::LevelUnsupported {
                        requester: b.requester.clone(),
                        span: *span,
                        stated: levels.iter().cloned().collect(),
                        level: level.to_string(),
                        system: system.name.clone(),
                    });
                }
            }
        }
        let mut judged = BTreeSet::new();
        for b in callers {
            if unsupported.contains(b.requester.as_str())
                || !judged.insert((b.requester.as_str(), b.provider.as_str()))
            {
                continue;
            }
            let Some(held) = self.held.get(&b.provider) else {
                continue;
            };
            let unstated = |absent: bool| Finding::LevelUnstated {
                requester: b.requester.clone(),
                span: b.requirement.span(),
                provider: b.provider.clone(),
                declared: held.span,
                absent,
            };
            match &held.level {
                Some(State::Valued(values)) => {
                    let offered = values.iter().find_map(|v| match v {
                        Value::Set(levels) => Some(levels),
                        _ => None,
                    });
                    match offered {
                        Some(levels) if levels.contains(level) => {}
                        Some(levels) => report.conflicts.push(Finding::OutOfLevel {
                            requester: b.requester.clone(),
                            span: b.requirement.span(),
                            provider: b.provider.clone(),
                            declared: held.span,
                            level: level.to_string(),
                            offered: levels.iter().cloned().collect(),
                        }),
                        None => report.unknowns.push(unstated(false)),
                    }
                }
                Some(State::Absent) => report.conflicts.push(unstated(true)),
                None | Some(State::Unvalued | State::Overflow) => {
                    report.unknowns.push(unstated(false));
                }
            }
        }
    }

    /// `SR-H15`'s last clause, the joint check's: a block or platform bound for a requester the system serves is
    /// part of the system's hardware.
    fn connectivity(&self, system: &System, assignment: &[Binding], report: &mut Report) {
        for b in assignment.iter().filter(|b| system.serves(&b.requester)) {
            let Some(held) = self.held.get(&b.provider) else {
                continue;
            };
            if held.hardware && !system.hardware.contains(&b.provider) {
                report.conflicts.push(Finding::Unconnected {
                    requester: b.requester.clone(),
                    span: b.requirement.span(),
                    provider: b.provider.clone(),
                    declared: held.span,
                    system: system.name.clone(),
                });
            }
        }
    }

    /// Model §8 rule 16 (`RTP-H24`): a sporadic task the system serves, bound itself for `interrupt-source` at `true`,
    /// is released on every arrival of that source, which the analysis admits only where the source's stated
    /// `min-arrival-separation` is at least the task's `min-separation` — its condition 5, "a task released on every
    /// arrival of a source has T ≤ T_s". A shorter separation, one declared absent, or a comparison past the exact
    /// arithmetic is a conflict; a source stating no value, an unknown. The route stays rule 8's obligation.
    fn own_sources(&self, system: &System, assignment: &[Binding], report: &mut Report) {
        for b in assignment.iter().filter(|b| {
            b.requirement.fact() == INTERRUPT_SOURCE
                && met_by_true(&b.requirement)
                && system.serves(&b.requester)
        }) {
            let Some(&task) = self.separations.get(&b.requester) else {
                continue;
            };
            let Some(held) = self.held.get(&b.provider) else {
                continue;
            };
            let why = match &held.separation {
                Some(State::Valued(values)) => match values.first() {
                    Some(Value::Quantity(source)) => match order(*source, task) {
                        Ok(Ordering::Less) => Some(Unadmitted::Faster {
                            task,
                            source: *source,
                        }),
                        Ok(_) => None,
                        Err(_) => Some(Unadmitted::PastArithmetic),
                    },
                    _ => Some(Unadmitted::Unstated),
                },
                Some(State::Absent) => Some(Unadmitted::Absent),
                Some(State::Overflow) => Some(Unadmitted::PastArithmetic),
                Some(State::Unvalued) | None => Some(Unadmitted::Unstated),
            };
            if let Some(why) = why {
                let finding = Finding::OwnSource {
                    requester: b.requester.clone(),
                    span: b.requirement.span(),
                    provider: b.provider.clone(),
                    declared: held.span,
                    why,
                };
                if finding.code() == "missing-fact" {
                    report.unknowns.push(finding);
                } else {
                    report.conflicts.push(finding);
                }
            }
        }
    }

    /// One route obligation per provider the system's requesters rely on for an interrupt, and per kind.
    fn routes(&self, system: &System, assignment: &[Binding], report: &mut Report) {
        for ((provider, is_source), requesters) in self.route_map(system, assignment) {
            let interrupt = if is_source {
                Interrupt::Source {
                    separation: self.held.get(provider).and_then(|h| h.separation.clone()),
                }
            } else {
                Interrupt::Timer
            };
            report.obligations.push(Obligation::Route {
                system: system.name.clone(),
                provider: provider.to_string(),
                interrupt,
                requesters: requesters.into_iter().map(str::to_string).collect(),
                level: system.level.clone(),
            });
        }
    }

    /// Rule 8's routes of `system`, one for each provider and kind, keyed `(provider, is_source)`, each with the
    /// requesters relying on it.
    fn route_map<'a>(
        &self,
        system: &System,
        assignment: &'a [Binding],
    ) -> BTreeMap<(&'a str, bool), BTreeSet<&'a str>> {
        let mut routes: BTreeMap<(&str, bool), BTreeSet<&str>> = BTreeMap::new();
        for b in assignment
            .iter()
            .filter(|b| system.serves(&b.requester) && !self.instances.contains_key(&b.provider))
        {
            let source = b.requirement.fact() == INTERRUPT_SOURCE && met_by_true(&b.requirement);
            let timer = takes_a_channel(&b.requirement, self.vocabulary).is_some();
            for (is_source, relied) in [(true, source), (false, timer)] {
                if relied {
                    routes
                        .entry((b.provider.as_str(), is_source))
                        .or_default()
                        .extend(self.relying(system, assignment, &b.requester));
                }
            }
        }
        routes
    }

    /// Who relies on a binding of `requester`'s for an interrupt: itself, or, for an instance, the requesters the system
    /// serves that are bound to it (record §4, rule 8).
    fn relying<'a>(
        &self,
        system: &System,
        assignment: &'a [Binding],
        requester: &'a str,
    ) -> Vec<&'a str> {
        if self.instances.contains_key(requester) {
            assignment
                .iter()
                .filter(|b| b.provider == requester && system.serves(&b.requester))
                .map(|b| b.requester.as_str())
                .collect()
        } else {
            vec![requester]
        }
    }

    /// Model §8 rule 14, one time base (`docs/specs/adapters/decision_engine-adapters.md` §4): where a requester is bound
    /// for `absolute-deadline`, every other requester it reaches through `needs` and `uses`, bound for a fact of the
    /// function `counter` that is not consumed, is bound at the same device — an instance's being its provider. A
    /// deadline programmed in one counter's time and read in another's is a conflict. Its own facts of the function are
    /// rule 13's, whose codes tell a split from one describing a fact would make whole (`SR-H17`).
    fn time_base(&self, system: &System, assignment: &[Binding], report: &mut Report) {
        let counter = |fact: &str| {
            self.vocabulary.fact(fact).is_some_and(|f| {
                f.usage == Usage::Shared && f.function == Some(Function::Of("counter".to_string()))
            })
        };
        let mut found = BTreeSet::new();
        for deadline in assignment
            .iter()
            .filter(|b| system.serves(&b.requester) && b.requirement.fact() == ABSOLUTE_DEADLINE)
        {
            let device = self.device(&deadline.provider);
            let mut reached = BTreeSet::new();
            let mut todo = vec![deadline.requester.as_str()];
            while let Some(at) = todo.pop() {
                for next in self.names.get(at).into_iter().flatten() {
                    if self.requesters.contains_key(next)
                        && next != &deadline.requester
                        && reached.insert(next.as_str())
                    {
                        todo.push(next.as_str());
                    }
                }
            }
            for read in assignment
                .iter()
                .filter(|b| reached.contains(b.requester.as_str()) && counter(b.requirement.fact()))
            {
                let read_device = self.device(&read.provider);
                if read_device != device
                    && found.insert((
                        deadline.requester.clone(),
                        read.requester.clone(),
                        read_device.to_string(),
                    ))
                {
                    report.conflicts.push(Finding::TwoTimeBases {
                        requester: deadline.requester.clone(),
                        span: deadline.requirement.span(),
                        device: device.to_string(),
                        reader: read.requester.clone(),
                        read: read.requirement.span(),
                        read_device: read_device.to_string(),
                    });
                }
            }
        }
    }

    /// F08: the requesters taking a provider's channels number at most its `channels`.
    fn channels(&self, assignment: &[Binding], report: &mut Report) {
        let mut taking: BTreeMap<&str, BTreeMap<&str, Span>> = BTreeMap::new();
        for b in assignment {
            if let Some(span) = takes_a_channel(&b.requirement, self.vocabulary) {
                taking
                    .entry(b.provider.as_str())
                    .or_default()
                    .entry(b.requester.as_str())
                    .or_insert(span);
            }
        }
        for (provider, by) in taking {
            let requesters: Vec<(String, Span)> = by
                .iter()
                .map(|(name, span)| ((*name).to_string(), *span))
                .collect();
            if requesters.len() < 2 {
                continue; // one requester needs no count: offering the function at all serves one
            }
            let channels = self
                .held
                .get(provider)
                .map_or(Channels::Undescribed, |h| h.channels);
            match channels {
                Channels::Count(n)
                    if i128::try_from(requesters.len()).is_ok_and(|taken| taken <= n) => {}
                Channels::Count(n) => report.conflicts.push(Finding::OverChannels {
                    provider: provider.to_string(),
                    channels: n,
                    requesters,
                }),
                Channels::Undescribed | Channels::Unknown => {
                    report.unknowns.push(Finding::ChannelsUnknown {
                        provider: provider.to_string(),
                        requesters,
                    })
                }
            }
        }
    }

    /// F09: the regions of two bound providers do not overlap, unless one composes the other.
    fn ownership(&self, assignment: &[Binding], report: &mut Report) {
        let bound: BTreeSet<&str> = assignment.iter().map(|b| b.provider.as_str()).collect();
        let bound: Vec<(&str, &Held)> = bound
            .into_iter()
            .filter_map(|name| self.held.get(name).map(|h| (name, h)))
            .collect();
        for (i, (p, ph)) in bound.iter().enumerate() {
            for (q, qh) in &bound[i + 1..] {
                if self.composes(p, q) || self.composes(q, p) {
                    continue;
                }
                // Placement memory is the plan's, not a device's: rule 10 judges it (leaf `M3.3.5.1`).
                for a in ph.regions.iter().filter(|r| !r.is_memory()) {
                    for b in qh.regions.iter().filter(|r| !r.is_memory()) {
                        judge_pair((p, a), (q, b), report);
                    }
                }
            }
        }
    }

    /// Whether `outer` composes `inner` through `uses`, at any depth.
    fn composes(&self, outer: &str, inner: &str) -> bool {
        let mut seen = BTreeSet::new();
        let mut todo = vec![outer];
        while let Some(at) = todo.pop() {
            if !seen.insert(at) {
                continue;
            }
            if let Some(h) = self.held.get(at) {
                for next in &h.composes {
                    if next == inner {
                        return true;
                    }
                    todo.push(next);
                }
            }
        }
        false
    }

    /// Where `provider` is declared, when the rules read it.
    #[must_use]
    pub fn span_of(&self, provider: &str) -> Option<Span> {
        self.held.get(provider).map(|h| h.span)
    }
}

/// The strongly connected components of more than one node, each by name, in the order of their least names (Tarjan's
/// algorithm, visiting nodes and successors by name, so the result is one for one graph).
/// Rule 12's requesters whose bindings count: those `system` serves, then each provider bound for one, at any depth.
fn active<'a>(system: &System, assignment: &'a [Binding]) -> BTreeSet<&'a str> {
    let mut active: BTreeSet<&str> = assignment
        .iter()
        .map(|b| b.requester.as_str())
        .filter(|r| system.serves(r))
        .collect();
    loop {
        let reached: Vec<&str> = assignment
            .iter()
            .filter(|b| active.contains(b.requester.as_str()))
            .map(|b| b.provider.as_str())
            .filter(|p| !active.contains(p))
            .collect();
        if reached.is_empty() {
            return active;
        }
        active.extend(reached);
    }
}

fn components(
    nodes: &BTreeMap<String, Node>,
    edges: &BTreeSet<(String, String, Because)>,
) -> Vec<Vec<String>> {
    struct Walk<'g> {
        next: BTreeMap<&'g str, BTreeSet<&'g str>>,
        index: BTreeMap<&'g str, usize>,
        low: BTreeMap<&'g str, usize>,
        stack: Vec<&'g str>,
        on_stack: BTreeSet<&'g str>,
        found: Vec<Vec<String>>,
    }
    fn visit<'g>(w: &mut Walk<'g>, at: &'g str) {
        let i = w.index.len();
        w.index.insert(at, i);
        w.low.insert(at, i);
        w.stack.push(at);
        w.on_stack.insert(at);
        let next: Vec<&'g str> = w
            .next
            .get(at)
            .map(|n| n.iter().copied().collect())
            .unwrap_or_default();
        for to in next {
            if !w.index.contains_key(to) {
                visit(w, to);
                let low = w.low[at].min(w.low[to]);
                w.low.insert(at, low);
            } else if w.on_stack.contains(to) {
                let low = w.low[at].min(w.index[to]);
                w.low.insert(at, low);
            }
        }
        if w.low[at] == w.index[at] {
            let mut component = Vec::new();
            while let Some(top) = w.stack.pop() {
                w.on_stack.remove(top);
                component.push(top.to_string());
                if top == at {
                    break;
                }
            }
            if component.len() > 1 {
                component.sort();
                w.found.push(component);
            }
        }
    }
    let mut walk = Walk {
        next: BTreeMap::new(),
        index: BTreeMap::new(),
        low: BTreeMap::new(),
        stack: Vec::new(),
        on_stack: BTreeSet::new(),
        found: Vec::new(),
    };
    for (from, to, _) in edges {
        walk.next
            .entry(from.as_str())
            .or_default()
            .insert(to.as_str());
    }
    for name in nodes.keys() {
        if !walk.index.contains_key(name.as_str()) {
            visit(&mut walk, name.as_str());
        }
    }
    walk.found.sort();
    walk.found
}

/// How many of the `size` bytes from `base` the holes' spans cover, each counted once.
fn covered_bytes(base: i128, size: i128, holes: impl Iterator<Item = (i128, i128)>) -> i128 {
    let mut spans: Vec<(i128, i128)> = holes
        .map(|(lo, hi)| (lo.max(base), hi.min(base + size)))
        .filter(|(lo, hi)| lo < hi)
        .collect();
    spans.sort_unstable();
    let (mut covered, mut reach) = (0, base);
    for (lo, hi) in spans {
        let lo = lo.max(reach);
        if hi > lo {
            covered += hi - lo;
            reach = hi;
        }
    }
    covered
}

/// Two regions of two bound providers: overlapping known bytes are a conflict; a region with no size, with the other's
/// bytes at or above its base, is unknown.
fn judge_pair(a: (&str, &Region), b: (&str, &Region), report: &mut Report) {
    let ((p, ra), (q, rb)) = (a, b);
    let (a0, a1) = ra.known();
    let (b0, b1) = rb.known();
    if a0 < b1 && b0 < a1 {
        report.conflicts.push(Finding::Overlap {
            first: (p.to_string(), ra.clone()),
            second: (q.to_string(), rb.clone()),
        });
        return;
    }
    if ra.bytes.is_none() && b0 >= a0 {
        report.unknowns.push(Finding::RegionUnsized {
            provider: p.to_string(),
            region: ra.clone(),
            against: q.to_string(),
        });
    }
    if rb.bytes.is_none() && a0 >= b0 {
        report.unknowns.push(Finding::RegionUnsized {
            provider: q.to_string(),
            region: rb.clone(),
            against: p.to_string(),
        });
    }
}

/// Whether the value `true` of the requirement's fact meets it: `(needs f)`, `(f true)`, or the group whose head it is.
const fn met_by_true(requirement: &Requirement) -> bool {
    matches!(
        requirement,
        Requirement::Presence { .. }
            | Requirement::Group { .. }
            | Requirement::Constraint {
                value: Value::Bool(true),
                ..
            }
    )
}

/// Where a requirement takes a channel: one on a consumed fact that the value `true` meets.
fn takes_a_channel(requirement: &Requirement, vocabulary: &Vocabulary) -> Option<Span> {
    let consumed = vocabulary
        .fact(requirement.fact())
        .is_some_and(|f| f.usage == Usage::Consumed);
    (consumed && met_by_true(requirement)).then(|| requirement.span())
}

/// Whether `provider` offers a consumed fact at the value `true`.
fn offers_a_consumed_function(provider: &Provider, vocabulary: &Vocabulary) -> bool {
    vocabulary
        .facts()
        .filter(|f| f.usage == Usage::Consumed)
        .any(|f| matches!(provider.state(&f.name), Some(State::Valued(v)) if v.contains(&Value::Bool(true))))
}

/// Every region `decl` offers, each read or refused.
fn regions(provider: &Provider, decl: &Form, findings: &mut Vec<Finding>) -> Vec<Region> {
    let mut out = Vec::new();
    let offers = decl.items().iter().filter(|c| c.head() == Some("offers"));
    for item in offers.flat_map(|c| c.items()[1..].iter()) {
        // A bare `region` is an offer of one that names no byte range, which `region` refuses (model §8 rule 5).
        if item.head() != Some("region") && item.as_symbol() != Some("region") {
            continue;
        }
        match region(item) {
            Ok(r) => out.push(r),
            Err(None) => findings.push(Finding::RegionPastArithmetic {
                provider: provider.name.clone(),
                span: item.span(),
            }),
            Err(Some(detail)) => findings.push(Finding::RegionMalformed {
                provider: provider.name.clone(),
                span: item.span(),
                detail,
            }),
        }
    }
    out
}

/// One `(region NAME (base B) (size S) …)`: `Err(Some(why))` when it is written outside model §8 rule 5's grammar,
/// `Err(None)` when its range lies past the exact arithmetic.
fn region(item: &Form) -> Result<Region, Option<String>> {
    let items = item.items();
    let name = items
        .get(1)
        .and_then(Form::as_symbol)
        .ok_or_else(|| Some("a region is named first".to_string()))?
        .to_string();
    if let Some(other) = items[2..]
        .iter()
        .find(|c| !matches!(c.head(), Some("base" | "size" | "executable" | "reserved")))
    {
        return Err(Some(format!(
            "region `{name}` has a sub-form `{}` no rule reads",
            other.head().or_else(|| other.as_symbol()).unwrap_or("?")
        )));
    }
    let executable = flag(&items[2..], "executable", &name)?;
    let reserved = flag(&items[2..], "reserved", &name)?;
    let base = match clauses(&items[2..], "base").as_slice() {
        [one] => match one.items() {
            [_, Form::Integer { value, .. }] if *value >= 0 => i128::from(*value),
            _ => {
                return Err(Some(format!(
                    "region `{name}`'s base is a non-negative address"
                )))
            }
        },
        [] => return Err(Some(format!("region `{name}` has no base"))),
        _ => return Err(Some(format!("region `{name}` has two bases"))),
    };
    let bytes = match clauses(&items[2..], "size").as_slice() {
        [] => None,
        [one] => Some(size_in_bytes(one, &name)?),
        _ => return Err(Some(format!("region `{name}` has two sizes"))),
    };
    if let Some(bytes) = bytes {
        base.checked_add(bytes).ok_or(None)?;
    }
    Ok(Region {
        name,
        span: item.span(),
        base,
        bytes,
        executable,
        reserved,
    })
}

/// A region's flag, `(executable true)` or `(reserved false)`: at most once, `true` or `false`, `false` when unwritten.
fn flag(items: &[Form], head: &str, name: &str) -> Result<bool, Option<String>> {
    match clauses(items, head).as_slice() {
        [] => Ok(false),
        [one] => match one.items() {
            [_, value] if value.as_symbol() == Some("true") => Ok(true),
            [_, value] if value.as_symbol() == Some("false") => Ok(false),
            _ => Err(Some(format!(
                "region `{name}`'s `{head}` is `true` or `false`"
            ))),
        },
        _ => Err(Some(format!("region `{name}` has two `{head}`s"))),
    }
}

/// The sub-clauses among `items` headed `head`.
fn clauses<'a>(items: &'a [Form], head: &str) -> Vec<&'a Form> {
    items.iter().filter(|c| c.head() == Some(head)).collect()
}

/// A size clause, `(size AMOUNT UNIT)`: a positive whole number of bytes.
fn size_in_bytes(clause: &Form, name: &str) -> Result<i128, Option<String>> {
    let wrong = || {
        Some(format!(
            "region `{name}`'s size is a positive whole number of bytes"
        ))
    };
    let items = clause.items();
    let quantity = Quantity::read(items.get(1), items.get(2), clause.span()).map_err(|d| {
        (d.code != "quantity-overflow")
            .then(|| format!("region `{name}`'s size does not read: {}", d.message))
    })?;
    if quantity.unit.dimension != Dimension::Information || items.len() != 3 {
        return Err(wrong());
    }
    let bits = quantity.in_base().map_err(|_| None)?;
    if !bits.is_integer() || !bits.is_positive() || bits.numerator() % 8 != 0 {
        return Err(wrong());
    }
    Ok(bits.numerator() / 8)
}
