//! Check (5), the joint rules, as `M3.5.5.8`'s fenced context writes it from `docs/semantics/model.md` §8 alone: the
//! rules that refuse an assignment — rules 1 to 5, 7, 9, 10 and 13 — judged over one system's assignment after checks
//! (1) to (4) accepted each binding alone.
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
//! assignment — a caller stating levels that do not hold the profile's (rule 9, judged first), a region placement
//! cannot be told apart from (rule 10), a side or a provider the relation does not read — is [`PLAN_DESCRIPTION`] under
//! that status, and refuses before any finding of the plan's.
//!
//! ⛔ **The level is the profile's, never the record's**: `Profile::caller_level`, so a corrupted level cannot pass.
//!
//! # Where §8's text leaves a choice, the reading taken
//!
//! - Rule 1: a system's own level is every clause of its `defsystem` but its tasks, its `platform` clause among them; a
//!   task is its own clauses; any other declaration is its whole side, as §7 rule 4 reads one.
//! - Rule 1: a binding whose requester names no declaration or no task of its system, or whose fact the vocabulary does
//!   not declare — a `uses` is composition, never a binding — is refused here too, though check (3) refuses it first.
//! - Rule 7: "every block or platform those name in turn" reads `those` as the blocks and platforms already found, so
//!   the hardware closure follows `uses` and `needs` out of blocks and platforms alone; a service naming a block does
//!   not make it hardware.
//! - Rule 7: the system serves its own level, its tasks and every declaration reached from any of them — the platform
//!   clause included — through `uses` and `needs`, at any depth and through declarations of every kind; a task of
//!   another system is refused under rule 7, an assignment being one system's.
//! - Rule 7: the positions read for `uses` and `needs` are a side's (§7 rule 4): any depth of any clause but `offers`,
//!   `absent` and `refines`.
//! - Rules 2 and 3: the count is of requesters, not bindings, so one requester binding both consumed facts to one
//!   provider takes one channel; rule 4's service is one requester however many users reach it, as the record names
//!   it once.
//! - Rule 3: `channels` declared absent at a provider two requesters take channels at is a conflict, no channel being
//!   offered; check (2) refuses such a provider first when it offers a consumed function `true`.
//! - Rule 5: a provider is bound when a binding names it; the `placement` clause binds nothing. Every region but
//!   placement memory is owned, reserved ones included. Composition is `uses` alone, at any depth, through
//!   declarations of every kind.
//! - Rules 5 and 10: a region without `size` owns its base byte, so another's bytes there overlap it; bytes strictly
//!   above its base are the unknown.
//! - Rules 5 and 10: `size` is a quantity of information — a number and a unit — whole and positive in bytes; a bare
//!   number is no size.
//! - Rule 9: "levels that do not hold the system's" is a stated set without the level; a set holding it and others
//!   passes, the others being the relation's wherever the qualifier binds. It is judged for every caller the system
//!   serves, bound or not.
//! - Rule 9: an empty `caller_level` is no level given, and the rule says nothing; one outside
//!   `reachable-at-privilege`'s domain is the engine's input error, refused as `tool-failure` before anything is judged.
//! - Rule 10: a record with no `placement` for a system whose hardware has placement memory places the image nowhere,
//!   that assignment's conflict. The unknown of an unsized claim is judged here, over the system's hardware, as the
//!   description's `missing-fact` — check (2) refuses it first, description-wide: a claim is placement memory or a
//!   device's registers, a reserved region being a hole and no claim. A region that does not read is likewise check
//!   (2)'s first, and the description's verdict here.
//! - Several verdicts of the description's: `invalid-description` before `unsupported-profile` before `missing-fact`,
//!   every finding kept.
//! - Rule 13: a function's requirements are every requirement the requester writes on a fact of it, a group by its head.
//!   "Meets them all" is every one satisfied; "refutes none" is none refused or absent. The unknown names every
//!   provider refuting none.
//! - Rule 13: a qualifier is judged for every requester the system serves that writes one, at the providers bound for
//!   its own functions and for those of every requester it reaches through `uses` and `needs`, at any depth; a
//!   judgement past the exact arithmetic is an unknown.

use std::collections::{BTreeMap, BTreeSet, VecDeque};

use archogen_outcome::record::{Finding, Label, Severity};
use archogen_outcome::Status;
use eadl_front::{Form, Program, SourceMap, Span};
use eadl_model::profile::Profile;
use eadl_model::quantity::{Dimension, Quantity};

use crate::assignment::{Assignment, Requester};
use crate::check::{self, Verdict, PLAN_DESCRIPTION, PLAN_JOINT};
use crate::model::judge::{judge, outcome, Outcome, Verdict as Judged};
use crate::model::provider::{self, Provider};
use crate::model::requirement::{read_side, NotJudged, Requirement};
use crate::model::value::Value;
use crate::model::vocab::{self, Domain, Function};

/// The qualifier rule 9 judges against the level, and rule 13 never repeats.
const PRIVILEGE: &str = "reachable-at-privilege";

/// The pool of a provider's consumed functions (rule 3).
const CHANNELS: &str = "channels";

/// Check (5) of `assignment`, which checks (1)–(4) accepted, over the elaborated `program`, `sources` its files, under
/// `profile`; `name` and `text` the record's, for the spans of its clauses.
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
    let Some(system) = world
        .get(&assignment.system)
        .filter(|d| d.kind == Kind::System)
    else {
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
    judgement.placement();
    judgement.functions();
    judgement.qualifiers();
    judgement.verdict()
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

    fn provides(&self, name: &str) -> bool {
        self.get(name).is_some_and(|d| d.kind.provides())
    }

    /// The declarations `form`, written in instance `instance`, names through the lists `heads` selects — `needs`,
    /// `uses` or both — each by its qualified name. A name that resolves to no declaration is a fact's, or nothing's.
    fn names(&self, instance: usize, form: &Form, heads: &[&str]) -> Vec<String> {
        let mut operands = Vec::new();
        operands_of(form, heads, &mut operands);
        operands
            .into_iter()
            .filter_map(|operand| match operand {
                Form::Symbol { name, span } => {
                    self.program.resolve(instance, name, *span).ok().flatten()
                }
                _ => None,
            })
            .filter(|qualified| self.declarations.contains_key(qualified))
            .collect()
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

/// Whether a requirement on the consumed fact `fact` is met by the value `true` — `(needs f)`, `(f true)` or the group
/// whose head it is — and so takes a channel (rule 2). `(f false)` takes none.
fn takes_channel(requirements: &[Requirement], fact: &str) -> bool {
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

    /// A claim on its bytes, as rule 10 counts them: placement memory or a device's registers; a reserved region is a
    /// hole, and claims nothing.
    const fn claims(&self) -> bool {
        !self.reserved
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
    let quantity = Quantity::read(Some(number), Some(unit), within)
        .map_err(|d| Unread::Invalid(format!("its size: {}", d.message)))?;
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
                    binds
                        .entry((requester.to_canonical(), fact.clone()))
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
        let named = world.names(system.instance, system.form, &["needs", "uses"]);
        let hardware = world.closure(named.clone(), &["needs", "uses"], |d| d.kind.provides());
        let reached = world.closure(named, &["needs", "uses"], |_| true);
        Self {
            sources,
            world,
            system,
            assignment,
            record,
            hardware,
            reached,
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

    /// Where `requester` requires `fact`, else where it is declared.
    fn requires(&self, requester: &Requester, fact: &str) -> Option<Label> {
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
    fn first_bound_to(&self, provider: &str) -> Label {
        match self
            .assignment
            .bindings
            .iter()
            .find(|(_, p)| p.as_str() == provider)
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
    /// description's verdict.
    fn requirements(&mut self, requester: &Requester) -> Vec<Requirement> {
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

    /// The block or platform `name`, read into one state per fact; `None` when it is none, or the relation refuses
    /// it, which is the description's verdict.
    fn provider(&mut self, name: &str) -> Option<Provider> {
        if let Some(read) = self.providers.get(name) {
            return read.clone();
        }
        let read = match self.world.get(name).filter(|d| d.kind.provides()) {
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
        };
        self.providers.insert(name.to_owned(), read.clone());
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

    /// Whether `requester` is a caller of the system (rule 9): its own level, one of its tasks, or a service or policy
    /// its closure reaches.
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
        }
    }

    /// Every requester the system serves (rule 7): its own level, each of its tasks, each declaration it reaches.
    fn served(&self) -> Vec<Requester> {
        let system = &self.assignment.system;
        let tasks = self
            .system
            .form
            .items()
            .iter()
            .filter(|clause| clause.head() == Some("task"))
            .filter_map(|clause| clause.items().get(1).and_then(Form::as_symbol))
            .map(|task| Requester::Task {
                system: system.clone(),
                task: task.to_owned(),
            });
        std::iter::once(Requester::Named(system.clone()))
            .chain(tasks)
            .chain(self.reached.iter().cloned().map(Requester::Named))
            .collect()
    }

    /// Rules 1 and 7: each binding is a requester's the system serves, of a vocabulary fact, to a block or platform of
    /// the system's hardware.
    fn requesters(&mut self) {
        let assignment = self.assignment;
        let system = self.assignment.system.clone();
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
            if !self.world.provides(provider) {
                self.conflict(
                    format!(
                        "`{who}` is bound for `{fact}` to `{provider}`, which is no block or platform of the \
                         description (rule 7)"
                    ),
                    here,
                    Vec::new(),
                    "resolve again: a provider is a block or platform of the system's hardware",
                );
            } else if !self.hardware.contains(provider) {
                let secondary = [
                    self.declared(provider),
                    self.requires(requester, fact),
                    Some(self.at(self.system.form.span(), &format!("the system `{system}`"))),
                ]
                .into_iter()
                .flatten()
                .collect();
                self.conflict(
                    format!(
                        "`{who}` is bound for `{fact}` to `{provider}`, outside the hardware of `{system}`: a provider on \
                         a platform the system does not use never satisfies (rule 7)"
                    ),
                    here,
                    secondary,
                    "resolve again: bind it to a block or platform of the system's hardware — one its `platform` \
                     clause, its own `uses` and `needs` or its tasks name, or one those compose",
                );
            }
        }
    }

    /// Rules 2 to 4: the requesters taking channels at one provider number at most its `channels`.
    fn channels(&mut self) {
        let assignment = self.assignment;
        let mut taking: BTreeMap<String, BTreeMap<Requester, Vec<String>>> = BTreeMap::new();
        for ((requester, fact), provider) in &assignment.bindings {
            if !vocab::entry(fact).is_some_and(|e| e.consumed) {
                continue;
            }
            if takes_channel(&self.requirements(requester), fact) {
                taking
                    .entry(provider.clone())
                    .or_default()
                    .entry(requester.clone())
                    .or_default()
                    .push(fact.clone());
            }
        }
        for (provider, requesters) in taking {
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
            secondary.extend(self.declared(&provider));
            if conflict {
                self.conflict(
                    format!(
                        "`{provider}` gives {count} requesters a channel — {} — and {why}: more requesters than \
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
                        "`{provider}` gives {count} requesters a channel — {} — and {why}, so whether they fit is \
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

    /// Rule 5: two distinct bound providers, neither composing the other through `uses`, own no byte in common.
    fn ownership(&mut self) {
        let bound: BTreeSet<String> = self
            .assignment
            .bindings
            .values()
            .filter(|p| self.world.provides(p))
            .cloned()
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
                self.world.closure(named, &["uses"], |_| true)
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
                        let primary = self.first_bound_to(p);
                        let secondary = vec![
                            self.at(a.span, &format!("`{p}` owns {}", a.range())),
                            self.at(b.span, &format!("`{q}` owns {}", b.range())),
                            self.first_bound_to(q),
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
    /// provider bound for a caller is reachable at the profile's level.
    fn privilege(&mut self, profile: &str, level: &str) {
        let assignment = self.assignment;
        // With no level given, these rules say nothing.
        if level.is_empty() {
            return;
        }
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
        let mut bound: BTreeMap<String, Vec<(Requester, String)>> = BTreeMap::new();
        for ((requester, fact), provider) in &assignment.bindings {
            if self.calls(requester) && self.world.provides(provider) {
                bound
                    .entry(provider.clone())
                    .or_default()
                    .push((requester.clone(), fact.clone()));
            }
        }
        for (provider, callers) in bound {
            let Some(read) = self.provider(&provider) else {
                continue;
            };
            let (conflict, why) = match outcome(&read, PRIVILEGE) {
                Ok(Outcome::Valued(values)) => match values.first() {
                    Some(Value::Set(offered)) if offered.contains(level) => continue,
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
            let secondary = self.declared(&provider).into_iter().collect();
            if conflict {
                self.conflict(
                    format!(
                        "`{provider}`, bound for {}, {why}, and `{profile}` runs every caller at `{level}` (rule 9)",
                        named.join(", ")
                    ),
                    primary,
                    secondary,
                    "resolve again: bind each caller to a provider reachable at the profile's level",
                );
            } else {
                self.conflict(
                    format!(
                        "`{provider}`, bound for {}, {why}, so whether it is reachable at `{level}`, the level \
                         `{profile}` runs every caller at, is unknown: `missing-fact`, which blocks the assignment \
                         (rule 9)",
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

    /// Rule 10: the plan places the image in placement memory of the system's hardware, where it has any; and where a
    /// claim on placement memory has no size, whether it overlaps another is the description's unknown.
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
        let claims: Vec<&Region> = regions.iter().filter(|r| r.claims()).collect();
        for (i, a) in claims.iter().enumerate() {
            for b in &claims[i + 1..] {
                if !(a.placement() || b.placement()) || overlap(a, b) != Overlap::Unknown {
                    continue;
                }
                let (open, other) = if a.end.is_none() { (a, b) } else { (b, a) };
                let primary = self.at(open.span, "a claim with no size");
                let secondary = vec![self.at(other.span, "bytes above its base")];
                self.refuse_description(
                    Status::MissingFact,
                    format!(
                        "whether {} and {} overlap is unknown: `{}` has no size, and the other's bytes lie above its \
                         base, in the hardware of `{system}` (rule 10)",
                        a.describe(),
                        b.describe(),
                        open.name
                    ),
                    primary,
                    secondary,
                    "give the region its `(size S)`",
                );
            }
        }
    }

    /// Rule 13: one requester's bindings for the facts of one function are to one provider.
    fn functions(&mut self) {
        let assignment = self.assignment;
        let mut by: BTreeMap<Requester, BTreeMap<&'static str, BTreeMap<String, Vec<String>>>> =
            BTreeMap::new();
        for ((requester, fact), provider) in &assignment.bindings {
            if let Some(Function::Of(function)) = function_of(fact) {
                by.entry(requester.clone())
                    .or_default()
                    .entry(function)
                    .or_default()
                    .entry(provider.clone())
                    .or_default()
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
                for provider in providers.keys() {
                    let Some(read) = self.provider(provider) else {
                        continue;
                    };
                    judged.push((
                        provider,
                        of.iter().map(|r| (fact_of(r), judge(&read, r))).collect(),
                    ));
                }
                if judged.len() < providers.len() {
                    continue;
                }
                let split: Vec<String> = providers
                    .iter()
                    .map(|(p, facts)| format!("`{p}` for `{}`", facts.join("`, `")))
                    .collect();
                let who = requester.canonical();
                let fact = providers
                    .values()
                    .flatten()
                    .next()
                    .expect("a function bound");
                let primary = self.bound(&requester, fact);
                let secondary: Vec<Label> =
                    providers.keys().filter_map(|p| self.declared(p)).collect();
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
    /// is judged at every provider bound for the requester's functions and for those of each requester it reaches.
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
            let reaches =
                self.world
                    .side(&requester)
                    .map_or_else(BTreeSet::new, |(form, instance)| {
                        let named = self.world.names(instance, &form, &["needs", "uses"]);
                        self.world.closure(named, &["needs", "uses"], |_| true)
                    });
            let mut relied: BTreeMap<String, (Requester, String)> = BTreeMap::new();
            for ((r, fact), provider) in &assignment.bindings {
                let mine =
                    *r == requester || matches!(r, Requester::Named(n) if reaches.contains(n));
                if mine && matches!(function_of(fact), Some(Function::Of(_))) {
                    relied
                        .entry(provider.clone())
                        .or_insert_with(|| (r.clone(), fact.clone()));
                }
            }
            // A requester relying on no function bound binds the qualifier as any requirement: checks (3) and (4).
            for (provider, (r, fact)) in relied {
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
                    let secondary = [self.requires(&requester, q), self.declared(&provider)]
                        .into_iter()
                        .flatten()
                        .collect();
                    let message = format!(
                        "`{}` relies on `{provider}` — bound for `{}`'s `{fact}` — and its qualifier `{q}`, which every \
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

    /// The verdict: the description's own, first and whole; else the plan's conflicts and unknowns, `tool-failure`.
    fn verdict(self) -> Result<(), Verdict> {
        if !self.description.is_empty() {
            let status = [
                Status::InvalidDescription,
                Status::UnsupportedProfile,
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
