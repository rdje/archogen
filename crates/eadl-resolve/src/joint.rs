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
//!   providers do not overlap, unless one composes the other through `uses`.
//! - **Connectivity** (leaf `M3.3.3`). An assignment is one [`System`]'s, and a block or platform it binds a requester
//!   of that system to is part of the system's hardware — `SR-H15`'s "a provider on a platform the system does not use
//!   never satisfying", in the checker's form.
//! - **Routes.** Each provider the assignment relies on for an interrupt — a source, or a timer's alarm — is reported
//!   as an [`Obligation::Route`]: the description states that the provider is part of the system, and the path from
//!   it to the core, with its latency, is the catalog's to state and the plan's to enable.
//! - **Privilege** (leaf `M3.3.4.1`). Given the level a plan binds the system's callers to, a caller states no other
//!   level, and every provider bound for a caller is reachable at that level and says so (`SR-H11`, `SR-H12`).
//!
//! A [`Report`] lists what one assignment breaks. It is a judgment of that assignment only: the description is
//! `infeasible-configuration`, or `missing-fact`, when no assignment the resolver can propose avoids it, which is
//! `M3.4`'s search to conclude. What a provider writes that cannot mean anything — a consumed function offered with no
//! channel, a region with no base — is refused when the providers are read, whether or not anything binds them.
//!
//! A service several users reach — tasks or services that use it — is one requester of what it binds: its realization
//! serves them, so it is the multiplexer, and an [`Obligation::Multiplex`] reports that it owes them the semantics and
//! a bounded overhead.

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{Diagnostic, Form, Label, Span};
use eadl_model::{Dimension, Quantity};

use crate::offer::{Provider, State};
use crate::requirement::side_name_lists;
use crate::requirement::{self, Requirement};
use crate::value::Value;
use crate::vocabulary::{Domain, Usage, Vocabulary};

/// Who a requester is (the design's requester).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Kind {
    /// A task, for what it writes itself.
    Task,
    /// A service.
    Service,
    /// Any other declaration, for what it writes at its own level, outside its tasks.
    Other,
}

/// One requester, its requirements and the declarations its `uses` names.
#[derive(Debug, Clone)]
pub struct Requester {
    /// Its name; a task's is `system/task`.
    pub name: String,
    /// Who it is.
    pub kind: Kind,
    /// Where it is written.
    pub span: Span,
    /// What it requires, as the relation reads a side.
    pub requirements: Vec<Requirement>,
    /// The declarations its `uses` lists name.
    pub uses: Vec<String>,
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
        let task_name = format!("{name}/{}", name_of(task));
        out.push(requester(task, task_name, Kind::Task, vocabulary)?);
    }
    Ok(out)
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
    })
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
    /// The blocks and platforms it has: every one its `platform` clause, its own `uses` and `needs` or its tasks' name,
    /// and every one those name in turn, at any depth. `refines` is no edge: a refinement keeps the abstract
    /// platform's obligations (model §3), not its blocks.
    pub hardware: BTreeSet<String>,
    /// Every declaration a `uses` or `needs` of the system, its tasks, or of what they reach names, at any depth.
    pub reached: BTreeSet<String>,
    /// The privilege level a plan binds the system's callers to, when one is given (model §8 rule 9).
    pub level: Option<String>,
}

impl System {
    /// Read `decl`, a `defsystem`, among the description's `declarations`; a name no declaration bears is a
    /// vocabulary fact or another rule's refusal, and leads nowhere.
    #[must_use]
    pub fn read(decl: &Form, declarations: &[Form]) -> Self {
        let declared: BTreeMap<&str, &Form> =
            declarations.iter().map(|d| (name_of(d), d)).collect();
        let mut reached = BTreeSet::new();
        let mut todo = vec![decl];
        while let Some(at) = todo.pop() {
            for name in named_by(at) {
                if let Some(next) = declared.get(name) {
                    if reached.insert(name.to_string()) {
                        todo.push(next);
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
            hardware,
            reached,
            level: None,
        }
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
            || requester
                .strip_prefix(self.name.as_str())
                .is_some_and(|task| task.starts_with('/'))
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
/// The source's own bound on how often it fires, which the analysis reads with the route.
const MIN_ARRIVAL_SEPARATION: &str = "min-arrival-separation";
/// The levels a function is reachable from, and the level a caller states it calls from.
const REACHABLE_AT_PRIVILEGE: &str = "reachable-at-privilege";

/// A byte range a provider owns: `(region NAME (base B) (size S))`.
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
}

impl Region {
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
}

impl Finding {
    /// Its code (`ROADMAP.md` §5.5).
    #[must_use]
    pub const fn code(&self) -> &'static str {
        match self {
            Self::OverChannels { .. }
            | Self::Overlap { .. }
            | Self::Unconnected { .. }
            | Self::OutOfLevel { .. }
            | Self::LevelUnstated { absent: true, .. } => "infeasible-configuration",
            Self::ChannelsUnknown { .. }
            | Self::RegionUnsized { .. }
            | Self::LevelUnstated { absent: false, .. } => "missing-fact",
            Self::LevelUnsupported { .. } => "unsupported-profile",
            Self::ChannelsContradicted { .. } | Self::RegionMalformed { .. } => {
                "invalid-description"
            }
            Self::RegionPastArithmetic { .. } => "unsupported-profile",
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
        }
    }
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
    /// A service several requesters reach takes a channel: its realization serves them all through it, so it owes
    /// them the shared semantics and a bounded overhead (`ROADMAP.md` §5.4).
    Multiplex {
        /// The service.
        service: String,
        /// The requesters that use it.
        users: Vec<String>,
        /// The provider whose channel it takes.
        provider: String,
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
}

/// What an [`Obligation::Route`]'s interrupt is.
#[derive(Debug, Clone, PartialEq)]
pub enum Interrupt {
    /// A source, bound through `interrupt-source` at `true`, with what it states of its own `min-arrival-separation`
    /// — `None` when it does not name it — since `SR-H14` (`M3.3.6`) may bind that fact to another provider.
    Source {
        /// Its own separation.
        separation: Option<State>,
    },
    /// A timer, bound through a consumed fact at `true`: an alarm the requester programs.
    Timer,
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
    users: BTreeMap<String, BTreeSet<String>>,
}

impl<'v> Joint<'v> {
    /// Read what the rules need of each provider — its channels, its regions, the providers it composes — and who
    /// uses whom among the requesters.
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
        // A service's users are what calls it at run time — tasks and other services. A system's own `uses` says what
        // the system includes, not who calls it, so it is no second user beside the task that does.
        let mut users: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
        for r in requesters.iter().filter(|r| r.kind != Kind::Other) {
            for used in &r.uses {
                users
                    .entry(used.clone())
                    .or_default()
                    .insert(r.name.clone());
            }
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
        Ok(Self {
            vocabulary,
            system: None,
            held,
            stated,
            requesters: requesters
                .iter()
                .map(|r| (r.name.clone(), r.kind))
                .collect(),
            users,
        })
    }

    /// The rules for one system's assignment: connectivity and routes judge the bindings of the requesters it serves.
    /// Without a system, an assignment is judged for channels and ownership alone.
    #[must_use]
    pub fn for_system(mut self, system: System) -> Self {
        self.system = Some(system);
        self
    }

    /// Judge one complete assignment.
    #[must_use]
    pub fn check(&self, assignment: &[Binding]) -> Report {
        let mut report = Report::default();
        self.channels(assignment, &mut report);
        self.ownership(assignment, &mut report);
        if let Some(system) = &self.system {
            self.connectivity(system, assignment, &mut report);
            self.routes(system, assignment, &mut report);
            if let Some(level) = &system.level {
                self.privilege(system, level, assignment, &mut report);
            }
        }
        report
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

    /// `SR-H15`'s last clause, in the checker's form: a block or platform bound for a requester the system serves is
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

    /// One route obligation per provider the system's requesters rely on for an interrupt, and per kind.
    fn routes(&self, system: &System, assignment: &[Binding], report: &mut Report) {
        let mut routes: BTreeMap<(&str, bool), BTreeSet<&str>> = BTreeMap::new();
        for b in assignment.iter().filter(|b| system.serves(&b.requester)) {
            let source = b.requirement.fact() == INTERRUPT_SOURCE && met_by_true(&b.requirement);
            let timer = takes_a_channel(&b.requirement, self.vocabulary).is_some();
            for (is_source, relied) in [(true, source), (false, timer)] {
                if relied {
                    routes
                        .entry((b.provider.as_str(), is_source))
                        .or_default()
                        .insert(b.requester.as_str());
                }
            }
        }
        for ((provider, is_source), requesters) in routes {
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
            for (name, _) in &requesters {
                let users = self.users.get(name).filter(|u| u.len() > 1);
                if let (Some(Kind::Service), Some(users)) = (self.requesters.get(name), users) {
                    report.obligations.push(Obligation::Multiplex {
                        service: name.clone(),
                        users: users.iter().cloned().collect(),
                        provider: provider.to_string(),
                    });
                }
            }
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
                for a in &ph.regions {
                    for b in &qh.regions {
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
        if item.head() != Some("region") {
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

/// One `(region NAME (base B) (size S) …)`: `Err(Some(why))` when it names no byte range, `Err(None)` when its range
/// lies past the exact arithmetic. Sub-clauses other than `base` and `size` are another rule's and ignored here.
fn region(item: &Form) -> Result<Region, Option<String>> {
    let items = item.items();
    let name = items
        .get(1)
        .and_then(Form::as_symbol)
        .ok_or_else(|| Some("a region is named first".to_string()))?
        .to_string();
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
    })
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
    let quantity =
        Quantity::read(items.get(1), items.get(2), clause.span()).map_err(|_| wrong())?;
    if quantity.unit.dimension != Dimension::Information || items.len() != 3 {
        return Err(wrong());
    }
    let bits = quantity.in_base().map_err(|_| None)?;
    if !bits.is_integer() || !bits.is_positive() || bits.numerator() % 8 != 0 {
        return Err(wrong());
    }
    Ok(bits.numerator() / 8)
}
