//! The runtime plan (`ROADMAP.md` §4.2 `archogen-plan`, §10.1 step 5; leaf `M4.1.4`): a checked assignment lowered into
//! `archogen-runtime-plan/1`, the plan for a system run on the hosted playground, as
//! `docs/decisions/decision_runtime-plan.md` writes it.
//!
//! ⭐ **Built ahead of the record's acceptance, as its executable form** (`docs/decisions/decision_executable-design-reviews.md`):
//! a direct transcription, rule by rule, each rule citing its section of the record, optimised for nothing. A review
//! round reads the record beside this code and runs the corpus `tests/corpus/`, in which every probe system of the
//! record's review that the lowering decides is a fixture with the outcome it gives, judged against the record, and the
//! record's own worked texts
//! (§3) are held equal to what this code writes (`tests/worked.rs`). A question a machine can settle — whether a rule
//! applies, what text it writes — is then settled by one.
//!
//! It lowers the record the configuration checker accepted, read back by the generator's own reader
//! (`eadl_resolve::plan::read`, `rebuild`; leaf `M4.1.3`), never the search's memory. It does no I/O.
//!
//! [`lower_checked`] is the production entry: it takes the checker's result record, refuses what §1 refuses of it, reads
//! the vocabulary and the adapter catalog as this engine's itself (§1's fourth input), and returns the provenance beside
//! the plan. [`lower`] is the executable form's, for what holds the record's rules against a record no checker judged;
//! nothing but a test may call it, since a plan it lowers has no acceptance behind it.

use std::collections::{BTreeMap, BTreeSet};

use archogen_digest::Digest;
use archogen_outcome::record::{Judged, Record as ResultRecord};

use eadl_front::{Form, Program, SourceMap};
use eadl_model::check::resolved_declarations;
use eadl_model::kind::Registry;
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_model::quantity::{Dimension, Quantity};
use eadl_model::rational::Rational;
use eadl_resolve::adapter::{self, Catalog, Instance, Offered};
use eadl_resolve::joint::{
    self, Because, Binding, Context, Interrupt, Kind, Node, Obligation, Routine,
};
use eadl_resolve::offer::{self, State};
use eadl_resolve::plan as assignment;
use eadl_resolve::requirement::Requirement;
use eadl_resolve::target::{judge_alone, on_target, OnTarget, Target};
use eadl_resolve::value::Value;
use eadl_resolve::vocabulary::{Usage, Vocabulary};

pub mod alarm;
pub mod application;
pub mod contents;
pub mod controller;
pub mod identity;
mod inputs;
pub mod layout;

/// The format's identifier (§3).
pub const FORMAT: &str = "archogen-runtime-plan/1";

/// Format `/2`'s identifier (`docs/decisions/decision_runtime-plan-format-2.md` §3), a plan for a system on a target, or
/// on the hosted playground with its application's inputs: written by [`lower_hosted`], which no command reaches yet
/// (leaf `M4.1.7.6.4`).
pub const FORMAT_2: &str = "archogen-runtime-plan/2";

/// The executable a result record must name as its producer (§1).
const CHECKER: &str = "archogen-plan-check";

/// The most bytes a result record may hold, as the command line reads one (`crates/archogen-cli/src/consume.rs`).
const RESULT_LIMIT: usize = 1 << 20;

/// The interrupt fact (model §8 rule 8).
const INTERRUPT_SOURCE: &str = "interrupt-source";

/// Why a record is not lowered (§1): the status vocabulary's code, and a sentence naming what and the repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    /// The code: `tool-failure`, `stale-input` or `unsupported-profile`.
    pub code: &'static str,
    /// What is refused, and its repair.
    pub message: String,
}

impl std::fmt::Display for Refusal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}: {}", self.code, self.message)
    }
}

fn refuse(code: &'static str, message: impl Into<String>) -> Refusal {
    Refusal {
        code,
        message: message.into(),
    }
}

/// A runtime plan: its canonical text, the one thing it is, and what that text holds, read back from it (§3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    text: String,
    contents: contents::Contents,
}

impl Plan {
    /// `text`, a plan's canonical text, read.
    ///
    /// # Errors
    ///
    /// Why it is not one: the line that does not read, is not written as §3 writes it, or stands out of §3's order.
    pub fn read(text: &str) -> Result<Self, String> {
        Ok(Self {
            contents: contents::read(text)?,
            text: text.to_owned(),
        })
    }

    /// The plan's canonical text: each line one form, ending with a newline, the last included (§3).
    #[must_use]
    pub fn text(&self) -> &str {
        &self.text
    }

    /// What it holds.
    #[must_use]
    pub const fn contents(&self) -> &contents::Contents {
        &self.contents
    }

    /// The sha256 of its canonical text, the plan's identity in the build's metadata (§3, `RTP-H7`).
    #[must_use]
    pub fn sha256(&self) -> Digest {
        Digest::of(self.text.as_bytes())
    }
}

/// A list written with its head alone when it has no member (§3, "Values").
fn list(head: &str, items: &[String]) -> String {
    if items.is_empty() {
        format!("({head})")
    } else {
        format!("({head} {})", items.join(" "))
    }
}

/// A duration as §2.2 writes it: one reduced rational number of seconds, `N s` or `N/D s`.
fn duration(seconds: Rational) -> String {
    if seconds.denominator() == 1 {
        format!("{} s", seconds.numerator())
    } else {
        format!("{}/{} s", seconds.numerator(), seconds.denominator())
    }
}

/// A quantity in seconds (§2.2): its value, or why it has none.
enum Seconds {
    Of(Rational),
    /// A quantity of another dimension, or none.
    NotTime,
    /// A time whose seconds lie past the engine's exact arithmetic.
    PastArithmetic,
}

fn quantity_seconds(quantity: Quantity) -> Seconds {
    if quantity.unit.dimension != Dimension::Time {
        return Seconds::NotTime;
    }
    quantity
        .in_base()
        .map_or(Seconds::PastArithmetic, Seconds::Of)
}

/// A clause's quantity, `(head N UNIT)`, in seconds.
fn seconds(clause: &Form) -> Seconds {
    Quantity::read(clause.items().get(1), clause.items().get(2), clause.span())
        .map_or(Seconds::NotTime, quantity_seconds)
}

/// A value, in seconds.
fn value_seconds(value: &Value) -> Seconds {
    match value {
        Value::Quantity(q) => quantity_seconds(*q),
        _ => Seconds::NotTime,
    }
}

/// The joint rules' "takes a channel": a consumed fact the value `true` meets (`joint.rs`, `takes_a_channel`).
fn takes_a_channel(requirement: &Requirement, vocabulary: &Vocabulary) -> bool {
    vocabulary
        .fact(requirement.fact())
        .is_some_and(|f| f.usage == Usage::Consumed)
        && met_by_true(requirement)
}

/// What the value `true` meets (`joint.rs`, `met_by_true`).
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

/// The names the plan writes: a task `(task SYSTEM TASK)`, an instance its `(adapter …)` text, anything else its name
/// (§2, a node's **text**).
struct Names<'r> {
    system: String,
    instances: BTreeMap<String, &'r Instance>,
}

impl Names<'_> {
    fn text(&self, name: &str) -> String {
        if let Some(instance) = self.instances.get(name) {
            return assignment::instance_text(instance, &self.system);
        }
        // A task's name is already `(task SYSTEM TASK)` (`joint::task_name`), and any other is a declaration's.
        name.to_string()
    }
}

/// One route of §2.5: its interrupt, its separation, the requesters it names, and the costs written on it.
struct Route {
    provider: String,
    timer: bool,
    /// The provider's stated separation, in seconds, or the word `/1` writes in its place.
    separation: Result<Rational, &'static str>,
    requesters: Vec<String>,
}

impl Route {
    /// Its interrupt's text (§2).
    fn item(&self) -> String {
        interrupt_text(&self.provider, self.timer)
    }
}

/// An interrupt as the plan writes it (§2): `(timer-route NAME)` or `(source NAME)`.
fn interrupt_text(provider: &str, timer: bool) -> String {
    if timer {
        format!("(timer-route {provider})")
    } else {
        format!("(source {provider})")
    }
}

/// One task of the system, as its `defsystem` writes it (§2.2).
struct Task<'f> {
    name: String,
    form: &'f Form,
    rank: i64,
}

/// A task's release and deadline, in seconds, once §1 has refused what the description check should have (§2.2).
struct Timing {
    /// Whether its release is a period, not a minimum separation.
    periodic: bool,
    release: Rational,
    deadline: Rational,
    jitter: Rational,
}

/// How the checker's acceptance reached the lowering, which cannot see it itself: neither whether the result came from an
/// installed, verified checker, nor its exit code (§1, "Provenance"; `M3.5.1`'s contract).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Acceptance {
    /// The command line ran the checker, having verified it, and read its result.
    CheckedByFront,
    /// The result was handed over as text, which `TI-H14` makes experimental: a plan so taken is built only marked so.
    TakenAsText,
}

/// The program lowering the record, which this crate cannot read for itself, since it does no I/O (§1, "Provenance").
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generator {
    /// The engine's version.
    pub version: String,
    /// The sha256 of the executable that ran (`TI-H14`).
    pub sha256: Digest,
}

/// What a plan was lowered from, the build's metadata and never part of the plan's text (§1, §3; `RTP-H7`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Provenance {
    /// The program that lowered it.
    pub generator: Generator,
    /// The sha256 of the result record, as the text it was given.
    pub result: Digest,
    /// The sha256 of the checker program that record names.
    pub checker: Digest,
    /// How the acceptance was obtained.
    pub acceptance: Acceptance,
}

/// A plan, with what it was lowered from.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lowered {
    /// The plan.
    pub plan: Plan,
    /// Its provenance.
    pub provenance: Provenance,
}

/// §1's refusals of the result record, each in the order written there: it does not read, or is not a configuration
/// checker's judgement of a plan, `tool-failure`; it judged another assignment, program or profile, `stale-input`; or it
/// refused what the generator accepted, `tool-failure`, naming both verdicts.
fn take_result(
    text: &str,
    assignment_text: &str,
    assignment: &assignment::Record,
    digest: &str,
) -> Result<ResultRecord, Refusal> {
    let result = ResultRecord::read(text.as_bytes(), RESULT_LIMIT).map_err(|why| {
        refuse(
            "tool-failure",
            format!("the result record does not read: {why}"),
        )
    })?;
    if result.producer().name != CHECKER {
        return Err(refuse(
            "tool-failure",
            format!(
                "the result record's producer is `{}`, and a plan is judged by `{CHECKER}`",
                result.producer().name
            ),
        ));
    }
    let Judged::Plan(judged) = result.subject().judged else {
        return Err(refuse(
            "tool-failure",
            "the result record judged a build, not a plan",
        ));
    };
    if judged != Digest::of(assignment_text.as_bytes()) {
        return Err(refuse(
            "stale-input",
            "the result record judged another assignment: resolve again",
        ));
    }
    if result.subject().program.hex() != digest {
        return Err(refuse(
            "stale-input",
            "the result record judged another program: resolve again",
        ));
    }
    if result.profile() != assignment.profile {
        return Err(refuse(
            "stale-input",
            format!(
                "the result record judged under `{}`, and the assignment was resolved under `{}`: resolve again",
                result.profile(),
                assignment.profile
            ),
        ));
    }
    if !result.accepts() {
        return Err(refuse(
            "tool-failure",
            format!(
                "the generator accepted this assignment and the configuration checker refused it, `{}`: the two \
                 roles disagree",
                result.status().slug()
            ),
        ));
    }
    Ok(result)
}

/// `assignment`, an `archogen-assignment/2` record, lowered into `archogen-runtime-plan/1` over `program`, the elaborated
/// program the language's check accepted, on `result`, the configuration checker's `archogen-result/1` for them (§1's
/// four inputs). `kinds` is the language definition, which the shipped vocabulary is read against: the vocabulary and the
/// adapter catalog are this engine's, compiled in, so no caller can hand it another.
///
/// # Errors
///
/// §1's refusals, in its order, the result record's among them after the assignment's (see [`lower`] for the rest); the
/// shipped vocabulary or catalog not reading is `tool-failure`.
pub fn lower_checked(
    program: &Program,
    assignment: &str,
    result: &str,
    kinds: &Registry,
    generator: Generator,
    acceptance: Acceptance,
) -> Result<Lowered, Refusal> {
    let shipped = |what: &str| refuse("tool-failure", format!("the shipped {what} does not read"));
    let mut sources = SourceMap::new();
    let vocabulary = Vocabulary::shipped(&mut sources, kinds).map_err(|_| shipped("vocabulary"))?;
    let catalog =
        Catalog::shipped(&mut sources, &vocabulary).map_err(|_| shipped("adapter catalog"))?;
    let (plan, accepted) = lowered(program, assignment, Some(result), &vocabulary, &catalog)?;
    let accepted = accepted.ok_or_else(|| {
        refuse(
            "tool-failure",
            "the lowering lost the result record it was given",
        )
    })?;
    Ok(Lowered {
        plan,
        provenance: Provenance {
            generator,
            result: Digest::of(result.as_bytes()),
            checker: accepted.producer().sha256,
            acceptance,
        },
    })
}

/// `record`, an `archogen-assignment/2` record, lowered into `archogen-runtime-plan/1` over `program`, the elaborated
/// program the language's check accepted, whose declarations and digest the lowering takes from it (§1, §2, §3): the
/// executable form's entry, with no result record behind it, which a test holds the rules with. [`lower_checked`] is the
/// production entry.
///
/// # Errors
///
/// §1's refusals, in its order: a record that does not read; that names another program, system, catalog or profile;
/// a system declaring two tasks of one name; a record that does not rebuild, or that the generator's own joint rules
/// refuse; a task's release or deadline the language's
/// check should have refused — none stated, a periodic task bound to an interrupt source itself; a declaration or a task the system serves naming a system; a block or a platform the system serves
/// naming what is not hardware; and what `/1` cannot lower — a duration past the exact arithmetic, placement memory, a
/// system with no task, a deadline measured from an event other than the release, an alarm or an expiry a task or the
/// system's own level would receive. Then §2.4's: an obliged value that is not a duration. A block or a platform a
/// service or a policy names outside the system's hardware is the joint rules' to refuse (model §8 rule 7, `M1.49`).
pub fn lower(
    program: &Program,
    record: &str,
    vocabulary: &Vocabulary,
    catalog: &Catalog,
) -> Result<Plan, Refusal> {
    lowered(program, record, None, vocabulary, catalog).map(|(plan, _)| plan)
}

/// The lowering, with the result record taken where §1 writes its refusals, when there is one: the body computed, then
/// written as `/1` and read back by the format's own reader, so the writer and the reader of §3 cannot part.
fn lowered(
    program: &Program,
    record: &str,
    result: Option<&str>,
    vocabulary: &Vocabulary,
    catalog: &Catalog,
) -> Result<(Plan, Option<ResultRecord>), Refusal> {
    let (body, accepted) = body(
        program,
        record,
        result,
        vocabulary,
        catalog,
        Format::One,
        None,
    )?;
    let text = body.write_one();
    let plan = Plan::read(&text).map_err(|why| {
        refuse(
            "tool-failure",
            format!("the lowering wrote a plan its own reader refuses: {why}"),
        )
    })?;
    Ok((plan, accepted))
}

/// What a lowering computes of a record, each of §3's groups of lines as `/1` writes them, in the format's order within
/// each group (leaf `M4.1.7.6.3`): what `/1`'s writer joins, and what `/2`'s writes from, so the two formats share one
/// body and `/1`'s bytes stay its own.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Body {
    /// `PROFILE` of `(profile …)`.
    profile: String,
    /// `NAME` of `(system …)`.
    system: String,
    /// The program's digest, `(program …)`'s `HEX`.
    program: String,
    /// The assignment record's digest, `(assignment …)`'s `HEX`.
    assignment: String,
    /// `(adapters …)`'s version.
    adapters: String,
    /// The `(task …)` lines.
    tasks: Vec<String>,
    /// The `(bind …)` lines.
    binds: Vec<String>,
    /// The `(instance …)` lines.
    instances: Vec<String>,
    /// The release timer's `(releases …)` items.
    releases: Vec<String>,
    /// The `(route …)` lines.
    routes: Vec<String>,
    /// The `(enabled …)` items.
    enabled: Vec<String>,
    /// The `(node …)` lines, by their text.
    nodes: Vec<String>,
    /// The `(context …)` lines.
    contexts: Vec<String>,
    /// The `(depends …)` lines.
    depends: Vec<String>,
    /// The `(step …)` lines.
    steps: Vec<String>,
    /// The `(obligation …)` lines.
    obligations: Vec<String>,
    /// The `(owed …)` lines, sorted.
    owed: Vec<String>,
    /// What `/2` adds to the owed lines of every plan, hosted or on a target (`decision_runtime-plan-format-2.md` §3,
    /// "`(owed …)`, as `/2` writes it"): each task's `node-calls` and `node-calls-category` and the system's
    /// `initialisation`, by the application, where no application's file is given, and each node a job calls owes its
    /// `primitives-out-of-line` and `masking-balanced`, by the realization.
    owed_by_two: Vec<String>,
    /// `/2`'s `(region …)` lines (§5, "Which regions"), in §3's order: each region the system's hardware states.
    regions: Vec<String>,
    /// `/2`'s groups where an alarm makes them differ from `/1`'s (§3, "An alarm's identity is its node"; leaf
    /// `M4.1.7.6.5`); with no alarm, `/1`'s own.
    two: Two,
    /// Each task by its name in its system and its text, `(task SYSTEM NAME)`, in runtime index order: what an
    /// application's file is said of, and the order its `(task-inputs …)` lines stand in (§3, "Orders").
    task_order: Vec<(String, String)>,
    /// On a target, its time (leaf `M4.1.7.6.7.2`); none on the playground.
    time: Option<Time>,
    /// On a target, `(placement PROVIDER REGION)`, the assignment's placement memory, written beside the map (§3, §5; leaf
    /// `M4.1.7.6.7.3`).
    placement: Option<String>,
    /// On a target, what a target adds to what is owed (§3): each source's and each alarm's `external`, and the
    /// `holds-under-preemption` of each node a job calls, each alarm and each source, by the realization (leaf
    /// `M4.1.7.6.7.3`).
    target_owed: Vec<String>,
    /// On a target, its memory and its controller (leaf `M4.1.7.6.8`); none on the playground.
    memory: Option<Memory>,
}

/// A plan on a target's memory and its controller (`decision_runtime-plan-format-2.md` §5 and §6; leaf `M4.1.7.6.8`): what
/// the body gives of them, and what [`laid_out`] lays out once the application's file is known.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Memory {
    /// `(image (floor F) (limit owed) (truncated-by …))`.
    image: String,
    /// Each `(obligation placement (region PROVIDER NAME) (discharged-by M4.4))`, by its text.
    obligations: Vec<String>,
    /// How many initialisation steps there are, each owing its `initialisation-stack`.
    steps: usize,
    /// Each route's `ITEM`, by rank: each owes its `service-stack`, and each reaches the controller.
    routes: Vec<String>,
    /// Whether the plan has a release timer, whose service's stack the interrupt stack holds.
    timer: bool,
    /// Each `(allocation UNIT owed (needs …))`, in §5's order, once laid out.
    allocations: Vec<String>,
    /// What the memory and the controller owe, once laid out.
    owed: Vec<String>,
}

/// What `/2` writes of a body's alarms (`decision_runtime-plan-format-2.md` §3; `decision_release-alarm.md` §2 and §3):
/// each alarm's line, each clock's, and the routes, the enabled line, the contexts and the obligations, one route for each
/// alarm where `/1` writes one for its device, ranked by the release alarm's order.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
struct Two {
    /// The `(alarm …)` lines, by `NODE`'s text.
    alarms: Vec<String>,
    /// The `(clock …)` lines, by `PROVIDER`'s text.
    clocks: Vec<String>,
    /// The `(route …)` lines, by rank.
    routes: Vec<String>,
    /// The `(enabled …)` items.
    enabled: Vec<String>,
    /// The `(context …)` lines.
    contexts: Vec<String>,
    /// The `(obligation …)` lines.
    obligations: Vec<String>,
    /// Each task that writes an alarm `RA-H7` asks the application's word of, by its text, and that alarm's text: one on
    /// the counter itself that is no wake ([`alarm::asks_application`]), which a task writes where it reaches its node
    /// (§3, `RA-H7`).
    asked: Vec<(String, String)>,
    /// Each alarm's item, `(alarm NODE)`, by `NODE`'s text: a source of its own on a target, which owes what a source does
    /// there (§3; leaf `M4.1.7.6.7.3`).
    items: Vec<String>,
}

/// One alarm of `/2` (§3): what takes a channel of a device for a compare, its node.
struct AlarmOf {
    /// Its node, by the joint rules' name.
    node: String,
    /// Its node's text, `NODE`.
    text: String,
    /// Its device, by the joint rules' name.
    device: String,
    /// A wake, an instance whose kind's cost is `wake`; otherwise an expiry, or an alarm bound directly.
    wake: bool,
    /// What its interrupt's code may run (§3; [`alarm::reach`]).
    reach: BTreeSet<String>,
    /// Its rank's value (`decision_release-alarm.md` §3): a wake's horizon, any other's delivery bound.
    rank: Option<Rational>,
    /// Whether its device states `(tick-rate …)` and `(counter-modulus …)`, for what it owes.
    stated: (bool, bool),
    /// Whether it asks a delivery bound, for what an expiry owes.
    delivery: bool,
    /// Its time base.
    base: alarm::Base,
    /// Its `(alarm …)` line.
    line: String,
}

/// Which format a body is lowered for: what differs before the lines are written is a refusal's repair (§1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Format {
    /// `archogen-runtime-plan/1`.
    One,
    /// `archogen-runtime-plan/2` on the hosted playground.
    Two,
    /// `archogen-runtime-plan/2` on a target, whose placement memory it lays out (leaf `M4.1.7.6.7.1`).
    OnTarget,
}

/// What generates a `/2` plan's code, which its kept lines name (`decision_runtime-plan-format-2.md` §1): the emitter's
/// version, `(emitter VERSION)`, and each record the tool's catalog holds, by its id and its code hash, which `(catalog
/// "HEX" …)` digests. Both are the caller's: the emitter states its version (`RPF-H3`, `M4.2`) and `archogen_catalog`
/// each record's code hash (`RPF-H9`, `M4.10`), neither of which this crate links.
#[derive(Debug, Clone, Copy)]
pub struct Generation<'a> {
    /// `(emitter …)`'s version, `archogen-emit/N`.
    pub emitter: &'a str,
    /// Each record of the tool's catalog, by its id and its code hash.
    pub records: &'a [(String, Digest)],
}

/// Where a `/2` plan stands, and the lines that place writes (`decision_runtime-plan-format-2.md` §1 and §3): the hosted
/// playground's, or a target's.
struct Place {
    /// The `(target …)` line.
    target: String,
    /// The `(catalog …)` line.
    catalog: String,
    /// The `(epoch …)` line.
    epoch: String,
    /// The `(timer …)` line.
    timer: String,
    /// Whether its `(region …)` lines are written: a hosted plan's are, and a target's since leaf `M4.1.7.6.7.3`.
    regions: bool,
    /// `(placement PROVIDER REGION)`, a target's; a hosted plan has none (§1).
    placement: Option<String>,
    /// A target's `(allocation …)` lines, then its `(image …)` line (§5); a hosted plan lays out no memory.
    memory: Vec<String>,
    /// A target's `(obligation placement …)` lines, among the rest by their text (§3).
    obligations: Vec<String>,
    /// Whether each route ends in `(controller owed)`, on a target, where every route reaches the controller while its
    /// `external` is owed (§6); a hosted route writes no `(controller …)`.
    controller: bool,
    /// What the place owes beside the rest: a target's time, memory and controller.
    owed: Vec<String>,
}

impl Body {
    /// `archogen-runtime-plan/1`'s text (§3): the lines, in the format's order, each ending in a line feed.
    fn write_one(&self) -> String {
        let mut lines = vec![
            format!("(format \"{FORMAT}\")"),
            format!("(profile {})", self.profile),
            format!("(system {})", self.system),
            format!("(program \"{}\")", self.program),
            format!("(assignment \"{}\")", self.assignment),
            format!("(adapters {})", self.adapters),
        ];
        lines.extend(self.tasks.iter().cloned());
        lines.extend(self.binds.iter().cloned());
        lines.extend(self.instances.iter().cloned());
        lines.push(format!(
            "(timer simulator {})",
            list("releases", &self.releases)
        ));
        lines.extend(self.routes.iter().cloned());
        lines.push(list("enabled", &self.enabled));
        lines.push("(interrupt-model (nesting no) (masking all))".to_string());
        lines.extend(self.nodes.iter().cloned());
        lines.extend(self.contexts.iter().cloned());
        lines.extend(self.depends.iter().cloned());
        lines.extend(self.steps.iter().cloned());
        lines.extend(self.obligations.iter().cloned());
        lines.extend(self.owed.iter().cloned());
        lines.push("(complete no)".to_string());
        let mut text = lines.join("\n");
        text.push('\n');
        text
    }

    /// `archogen-runtime-plan/2`'s text on the hosted playground (`decision_runtime-plan-format-2.md` §1, "The hosted
    /// playground", and §3): `/1`'s lines in §3's order, with these and no others: `(target hosted-playground)`,
    /// `(emitter …)`, `(catalog "HEX" none)`, `(epoch simulator)`, each alarm's and clock's line, a route for each alarm
    /// and what an alarm changes of the enabled line, the contexts and the obligations (leaf `M4.1.7.6.5`), each
    /// `(region …)`, and §3's owed lines.
    fn write_two_hosted(
        &self,
        generation: &Generation<'_>,
        application: Option<&inputs::Lines>,
    ) -> String {
        self.write_two(
            &Place {
                target: "(target hosted-playground)".to_string(),
                catalog: format!(
                    "(catalog \"{}\" none)",
                    identity::catalog_digest(generation.records).hex()
                ),
                epoch: "(epoch simulator)".to_string(),
                timer: format!("(timer simulator {})", list("releases", &self.releases)),
                regions: true,
                placement: None,
                memory: Vec::new(),
                obligations: Vec::new(),
                controller: false,
                owed: Vec::new(),
            },
            generation,
            application,
        )
    }

    /// `archogen-runtime-plan/2`'s text on a target (`decision_runtime-plan-format-2.md` §1, §3, §5 and §6; leaves
    /// `M4.1.7.6.7.1` to `M4.1.7.6.8`): the target's and the catalog's lines, its time, every line it shares with a hosted
    /// plan, an alarm's `COUNTER` and its `lag` among them, its regions with its placement memory's, its `(placement …)`
    /// line, its allocations and its image, its placement obligations, each route's `(controller owed)`, and what a target
    /// adds to what is owed.
    fn write_two_on_target(
        &self,
        target: &TargetLines,
        generation: &Generation<'_>,
        application: Option<&inputs::Lines>,
    ) -> Result<String, Refusal> {
        let time = self.time.as_ref().ok_or_else(|| {
            refuse(
                "tool-failure",
                "a body lowered on a target holds no time (§4)",
            )
        })?;
        let memory = self.memory.as_ref().ok_or_else(|| {
            refuse(
                "tool-failure",
                "a body lowered on a target holds no memory (§5)",
            )
        })?;
        Ok(self.write_two(
            &Place {
                target: target.target.clone(),
                catalog: target.catalog.clone(),
                epoch: time.epoch.clone(),
                timer: time.timer.clone(),
                regions: true,
                placement: self.placement.clone(),
                memory: memory
                    .allocations
                    .iter()
                    .chain(std::iter::once(&memory.image))
                    .cloned()
                    .collect(),
                obligations: memory.obligations.clone(),
                controller: true,
                owed: time
                    .owed
                    .iter()
                    .chain(&self.target_owed)
                    .chain(&memory.owed)
                    .cloned()
                    .collect(),
            },
            generation,
            application,
        ))
    }

    /// `archogen-runtime-plan/2`'s text (§3), `place` saying where it stands: `/1`'s lines in §3's order, with the place's
    /// own, each alarm's and clock's line, a route for each alarm and what an alarm changes of the enabled line, the
    /// contexts and the obligations (leaf `M4.1.7.6.5`), the application's lines (`.6`), and §3's owed lines.
    fn write_two(
        &self,
        place: &Place,
        generation: &Generation<'_>,
        application: Option<&inputs::Lines>,
    ) -> String {
        let mut lines = vec![
            format!("(format \"{FORMAT_2}\")"),
            format!("(profile {})", self.profile),
            format!("(system {})", self.system),
            place.target.clone(),
            format!("(program \"{}\")", self.program),
            format!("(assignment \"{}\")", self.assignment),
            format!("(adapters {})", self.adapters),
            format!("(emitter {})", generation.emitter),
            place.catalog.clone(),
        ];
        if let Some(application) = application {
            lines.push(application.application.clone());
            lines.push(application.code.clone());
        }
        lines.extend(self.tasks.iter().cloned());
        if let Some(application) = application {
            // Each task's inputs by runtime index (§3, "Orders"); `Application::unmet` has held that the file says one
            // of each task and of no other.
            lines.extend(
                self.task_order
                    .iter()
                    .filter_map(|(name, _)| application.tasks.get(name).cloned()),
            );
            lines.push(application.declared.clone());
            lines.push(application.initialisation.clone());
        }
        lines.extend(self.binds.iter().cloned());
        lines.extend(self.instances.iter().cloned());
        lines.push(place.epoch.clone());
        lines.push(place.timer.clone());
        lines.extend(self.two.alarms.iter().cloned());
        lines.extend(self.two.clocks.iter().cloned());
        // §6: on a target each route that reaches the controller writes `(controller …)` last on its line, owed while no
        // record states its line and its levels; every route reaches it while its `external` is owed (RP22-1).
        lines.extend(self.two.routes.iter().map(|route| {
            match (place.controller, route.strip_suffix(')')) {
                (true, Some(open)) => format!("{open} (controller owed))"),
                _ => route.clone(),
            }
        }));
        lines.push(list("enabled", &self.two.enabled));
        lines.push("(interrupt-model (nesting no) (masking all))".to_string());
        if place.regions {
            lines.extend(self.regions.iter().cloned());
        }
        lines.extend(place.placement.iter().cloned());
        lines.extend(place.memory.iter().cloned());
        lines.extend(self.nodes.iter().cloned());
        lines.extend(self.two.contexts.iter().cloned());
        lines.extend(self.depends.iter().cloned());
        lines.extend(self.steps.iter().cloned());
        let mut obligations: Vec<String> = self
            .two
            .obligations
            .iter()
            .chain(&place.obligations)
            .cloned()
            .collect();
        obligations.sort();
        obligations.dedup();
        lines.extend(obligations);
        // An alarm's line stands in place of `/1`'s timer route, so what `/1` owes of that route is owed of none (§3).
        // What the application's file states is owed of none: each task's facts it holds and the system's (§3, "Only what
        // no input states is written").
        let stated: BTreeSet<String> = application.map_or_else(BTreeSet::new, |_| {
            self.task_order
                .iter()
                .flat_map(|(_, text)| {
                    application::HELD_OF_A_TASK
                        .iter()
                        .map(move |fact| format!("(owed {text} {fact} (by application))"))
                })
                .chain(
                    application::HELD_OF_THE_SYSTEM
                        .iter()
                        .map(|fact| format!("(owed {} {fact} (by application))", self.system)),
                )
                .collect()
        });
        let mut owed: Vec<String> = self
            .owed
            .iter()
            .filter(|line| !line.starts_with("(owed (timer-route "))
            .chain(&self.owed_by_two)
            .chain(&place.owed)
            .filter(|line| !stated.contains(*line))
            .cloned()
            .collect();
        // `RA-H7`: a task writing an alarm on the counter itself that is no wake owes `deadlines-within-reach` where its
        // file does not name that alarm, or where no file is given.
        for (text, alarm) in &self.two.asked {
            let named = application.is_some_and(|application| {
                self.task_order
                    .iter()
                    .find(|(_, said)| said == text)
                    .and_then(|(name, _)| application.named.get(name))
                    .is_some_and(|named| named.contains(alarm))
            });
            if !named {
                owed.push(format!(
                    "(owed {text} deadlines-within-reach (by application))"
                ));
            }
        }
        owed.sort();
        owed.dedup();
        lines.extend(owed);
        lines.push("(complete no)".to_string());
        let mut text = lines.join("\n");
        text.push('\n');
        text
    }
}

/// Whether `text` is written as `/2`'s §3 writes a plan, as far as a lowering can tell of its own text before `/2`'s
/// reader is built (`M4.1.7.7`): each line a form whose head §1's table classifies, the heads in §3's order, which
/// [`identity::LINES`] follows, and the code identity's preimage taken.
fn written_as_two(text: &str) -> Result<(), String> {
    let order = |head: &str| identity::LINES.iter().position(|(name, _)| *name == head);
    let mut last = 0;
    for line in text.lines() {
        if !(line.starts_with('(') && line.ends_with(')')) {
            return Err(format!("`{line}` is no form"));
        }
        let head = identity::head(line);
        let at = order(head).ok_or_else(|| format!("no line of `/2` begins `{head}`"))?;
        if at < last {
            return Err(format!("`{line}` stands out of §3's order"));
        }
        last = at;
    }
    identity::preimage(text)
        .map(|_| ())
        .map_err(|why| why.to_string())
}

/// An application's file given to a lowering (`decision_runtime-plan-format-2.md` §1, "Where inputs are given"; leaf
/// `M4.1.7.6.6`): how a refusal names it, and its text.
#[derive(Debug, Clone, Copy)]
pub struct ApplicationFile<'a> {
    /// How a refusal names the file: its path as given.
    pub name: &'a str,
    /// Its text.
    pub text: &'a str,
}

/// The executable form's hosted `/2` lowering (`decision_runtime-plan-format-2.md` §1, "The hosted playground"; leaf
/// `M4.1.7.6.4`): [`lower`]'s record and body, written as `archogen-runtime-plan/2` with no realization's file and the
/// application's where one is given (leaf `M4.1.7.6.6`), `generation` naming what generates its code. No command reaches
/// it: nothing but a test may call it, as for [`lower`]. Each alarm is written as `/2` writes one (leaf `M4.1.7.6.5`).
///
/// # Errors
///
/// In §7's order: the application's file not reading, or naming another format, `invalid-description`, naming the file;
/// then [`lower`]'s refusals, a system with placement memory's repair naming a target (RP37-3); then the file's `(target
/// …)` other than the plan's, `invalid-description`, naming both, and each other thing [`application::Application::unmet`]
/// reports, `invalid-description`. And `tool-failure` for an emitter's version that is no single atom, for a timer route
/// with no alarm, for a count or a rate the format cannot write, and for a text that is not written as `/2`'s §3 writes
/// a plan.
pub fn lower_hosted(
    program: &Program,
    record: &str,
    vocabulary: &Vocabulary,
    catalog: &Catalog,
    generation: &Generation<'_>,
    application: Option<&ApplicationFile<'_>>,
) -> Result<String, Refusal> {
    emitter_checked(generation)?;
    // §7: the application's file not reading, or naming another format, before anything of the record (RP34-3, RP35-4).
    let application = match application {
        None => None,
        Some(file) => Some((
            file,
            application::read(file.text).map_err(|why| {
                refuse(
                    "invalid-description",
                    format!(
                        "the application's file `{}` is not `{}`'s inputs: {why}",
                        file.name,
                        application::FORMAT
                    ),
                )
            })?,
        )),
    };
    // §1, in `/1`'s first bullet's place: a `/3`, resolved on a target, lowered with none was resolved for another run (RP38-1).
    if let Some(why) = assignment::read(record)
        .ok()
        .and_then(|read| read.stale_for(None))
    {
        return Err(refuse("stale-input", why));
    }
    let (body, _) = body(
        program,
        record,
        None,
        vocabulary,
        catalog,
        Format::Two,
        None,
    )?;
    // §7, in an application's unmet's place: its own `(target …)` against the plan's, naming both, then each other thing
    // `unmet` reports of the plan's lines it reads, held over `/1`'s contents of the same body (`AI-H2`). The catalog
    // costs no primitive yet, its first record waiting on `M2.7.4.5`.
    if let Some((file, inputs)) = &application {
        if inputs.target != application::HOSTED_PLAYGROUND {
            return Err(refuse(
                "invalid-description",
                format!(
                    "the application's file `{}` is of the target `{}`, and the plan is of `{}`",
                    file.name,
                    inputs.target,
                    application::HOSTED_PLAYGROUND
                ),
            ));
        }
        let one = Plan::read(&body.write_one()).map_err(|why| {
            refuse(
                "tool-failure",
                format!("the lowering wrote a plan its own reader refuses: {why}"),
            )
        })?;
        let unmet = inputs.unmet(one.contents(), &[]);
        if !unmet.is_empty() {
            return Err(refuse(
                "invalid-description",
                format!(
                    "the application's file `{}` does not meet the plan: {}",
                    file.name,
                    unmet.join("; ")
                ),
            ));
        }
    }
    let lines = application
        .as_ref()
        .map(|(file, inputs)| inputs::lines(file.text, inputs));
    let text = body.write_two_hosted(generation, lines.as_ref());
    written_as_two(&text).map_err(|why| {
        refuse(
            "tool-failure",
            format!("the lowering wrote a `/2` plan not written as §3 writes one: {why}"),
        )
    })?;
    Ok(text)
}

/// The emitter's version, which `(emitter …)` writes as one atom, or `tool-failure`.
fn emitter_checked(generation: &Generation<'_>) -> Result<(), Refusal> {
    if generation.emitter.is_empty()
        || !generation
            .emitter
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || matches!(c, '-' | '/' | '.' | '_'))
    {
        return Err(refuse(
            "tool-failure",
            format!(
                "the emitter's version `{}` is no single atom, as `(emitter …)` writes one",
                generation.emitter
            ),
        ));
    }
    Ok(())
}

/// A target's file as a lowering on it is handed one (`decision_runtime-plan-format-2.md` §1; leaf `M4.1.7.6.7.1`).
#[derive(Debug, Clone, Copy)]
pub struct TargetFile<'a> {
    /// How a refusal names the file: its path as given.
    pub name: &'a str,
    /// Its text.
    pub text: &'a str,
}

/// A target's lines in a `/2` plan's identity (`decision_runtime-plan-format-2.md` §1, "The target in the identity" and
/// "The catalog in the identity"; leaf `M4.1.7.6.7.1`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TargetLines {
    /// `(target NAME "HEX")`: `NAME` the file's one platform, `HEX` the digest of its forms' program text.
    pub target: String,
    /// `(catalog "HEX" NAME)`: the digest of the tool's catalog and the platform's `catalog-target`.
    pub catalog: String,
    /// The platform's name, which a `/3` record and an application's file name the target by.
    pub platform: String,
    /// The digest, as `(target …)` writes it.
    pub digest: String,
}

/// The target's file judged alone, §7's first stage, and the target it is.
fn a_target(
    file: &TargetFile<'_>,
    kinds: &Registry,
    vocabulary: &Vocabulary,
) -> Result<Target, Refusal> {
    let mut sources = SourceMap::new();
    let id = sources
        .add(file.name, file.text.to_owned())
        .map_err(|why| {
            refuse(
                "invalid-description",
                format!("the target's file `{}` does not read: {why}", file.name),
            )
        })?;
    // §7, first: a target's own refusals before anything is lowered, `check`'s judgement of the target's file alone as `DC-H3` runs it, with
    // the code `check` gives, so one input gives the code `check` gives it (RP34-4, RP35-4, RP36-1).
    let alone = judge_alone(&sources, id, kinds, &RT_STATIC_UP_V1, vocabulary);
    let (verdict, diagnostics) = alone.concluded();
    if verdict != eadl_front::Verdict::Ok {
        let said: Vec<String> = diagnostics
            .iter()
            .map(|d| format!("{}: {}", d.code, d.message))
            .collect();
        return Err(refuse(
            verdict.slug(),
            format!(
                "the target's file `{}` is no machine this profile admits: {}",
                file.name,
                said.join("; ")
            ),
        ));
    }
    alone.target.ok_or_else(|| {
        refuse(
            "tool-failure",
            format!(
                "the target's file `{}` was judged a target and read as none",
                file.name
            ),
        )
    })
}

/// [`TargetLines`] of `target`, `generation` naming what generates the code.
fn lines_of(target: &Target, generation: &Generation<'_>) -> Result<TargetLines, Refusal> {
    let platform = target
        .declarations
        .iter()
        .find(|d| d.items().get(1).and_then(Form::as_symbol) == Some(target.platform.as_str()))
        .ok_or_else(|| {
            refuse(
                "tool-failure",
                format!(
                    "the target's platform `{}` has no declaration",
                    target.platform
                ),
            )
        })?;
    // The platform's `catalog-target`, the name a catalog record's `(target …)` goes by for this machine (§2's table; §5, "What the catalog is
    // read by"), as its offers write it.
    let catalog_target = platform
        .items()
        .iter()
        .filter(|item| item.head() == Some("offers"))
        .flat_map(|offers| offers.items().iter().skip(1))
        .find(|offer| offer.head() == Some("catalog-target"))
        .and_then(|offer| offer.items().get(1).and_then(Form::as_symbol))
        .ok_or_else(|| {
            refuse(
                "missing-fact",
                format!(
                    "the target's platform `{}` states no `catalog-target`, the name its catalog records go by",
                    target.platform
                ),
            )
        })?;
    let digest = assignment::target_digest(target);
    Ok(TargetLines {
        target: format!("(target {} \"{digest}\")", target.platform),
        catalog: format!(
            "(catalog \"{}\" {catalog_target})",
            identity::catalog_digest(generation.records).hex()
        ),
        platform: target.platform.clone(),
        digest,
    })
}

/// A target's lines in a plan's identity, the file judged first as §7's first stage judges it (leaf `M4.1.7.6.7.1`): what a lowering on the
/// target writes of it, for a reader to hold before the rest of a plan on a target is written (`M4.1.7.6.7.2`).
///
/// # Errors
///
/// The target's own refusals, by the code `check` gives them; `missing-fact` for a platform stating no `catalog-target`.
pub fn target_lines(
    file: &TargetFile<'_>,
    kinds: &Registry,
    vocabulary: &Vocabulary,
    generation: &Generation<'_>,
) -> Result<TargetLines, Refusal> {
    lines_of(&a_target(file, kinds, vocabulary)?, generation)
}

/// The executable form's `/2` lowering on a target (`decision_runtime-plan-format-2.md` §1 to §7; leaves `M4.1.7.6.7.1` to
/// `M4.1.7.6.8`): an `archogen-assignment/3` record resolved on `target`, lowered over `program` with the target's platform in
/// the system's own's place, as `on_target` gives it, and written whole: the target's lines, its time, its regions and
/// placement, its allocations and image, each route's controller and what it owes. No command reaches it: nothing but a
/// test may call it, as for [`lower`]. [`target_lines`] gives the target's own lines.
///
/// # Errors
///
/// In §7's order: the target's own refusals, by `check`'s code; the application's file not reading, or naming another format,
/// `invalid-description`; then `/1`'s list, the record not reading `tool-failure`, a record of the other kind or of another target's digest
/// `stale-input` with the program's digest, another platform beside the file's own digest `tool-failure`, the rest of `/1`'s list with the
/// joint rules' findings over the target's hardware `tool-failure`; then the application's `(target …)` other than the plan's and each thing
/// `unmet` reports, `invalid-description`; and `tool-failure` for an emitter's version that is no single atom, a count the
/// format cannot write, and a text not written as `/2`'s §3 writes one.
#[allow(clippy::too_many_arguments)]
pub fn lower_on_target(
    program: &Program,
    record: &str,
    target: &TargetFile<'_>,
    kinds: &Registry,
    vocabulary: &Vocabulary,
    catalog: &Catalog,
    generation: &Generation<'_>,
    application: Option<&ApplicationFile<'_>>,
) -> Result<String, Refusal> {
    let (body, lines, application) = on_target_body(
        program,
        record,
        target,
        kinds,
        vocabulary,
        catalog,
        generation,
        application,
    )?;
    let text = body.write_two_on_target(&lines, generation, application.as_ref())?;
    written_as_two(&text).map_err(|why| {
        refuse(
            "tool-failure",
            format!("the lowering wrote a `/2` plan not written as §3 writes one: {why}"),
        )
    })?;
    Ok(text)
}

/// [`lower_on_target`]'s stages, in §7's order, and what they give: the body, the target's lines and the application's.
#[allow(clippy::too_many_arguments)]
fn on_target_body(
    program: &Program,
    record: &str,
    target: &TargetFile<'_>,
    kinds: &Registry,
    vocabulary: &Vocabulary,
    catalog: &Catalog,
    generation: &Generation<'_>,
    application: Option<&ApplicationFile<'_>>,
) -> Result<(Body, TargetLines, Option<inputs::Lines>), Refusal> {
    emitter_checked(generation)?;
    let read_target = a_target(target, kinds, vocabulary)?;
    // §7: then the application's file not reading, or naming another format, the realization's being none (RP34-3, RP35-4).
    let application = match application {
        None => None,
        Some(file) => Some((
            file,
            application::read(file.text).map_err(|why| {
                refuse(
                    "invalid-description",
                    format!(
                        "the application's file `{}` is not `{}`'s inputs: {why}",
                        file.name,
                        application::FORMAT
                    ),
                )
            })?,
        )),
    };
    let lines = lines_of(&read_target, generation)?;
    // §1, "The target in the identity", in `/1`'s first bullet's place: the record read, of the kind and the target given, the digest
    // deciding which run it is of, and a platform against that digest no record a run writes (leaf `M4.1.7.5.6`).
    let read = assignment::read(record).map_err(|e| {
        refuse(
            "tool-failure",
            format!("the assignment record does not read: {e}"),
        )
    })?;
    if let Some(why) = read.stale_for(Some((&lines.platform, &lines.digest))) {
        return Err(refuse("stale-input", why));
    }
    if let Some(why) = read.misnames(&lines.platform) {
        return Err(refuse("tool-failure", why));
    }
    // The system on the target, as the search and the checker read it there.
    let flattened;
    let declarations: &[Form] = if program.instances.len() == 1 {
        &program.root().declarations
    } else {
        flattened = resolved_declarations(program).0;
        &flattened
    };
    let system = declarations
        .iter()
        .find(|d| d.head() == Some("defsystem") && d.items().get(1).and_then(Form::as_symbol) == Some(read.system.as_str()))
        .ok_or_else(|| {
            refuse(
                "stale-input",
                format!(
                    "the assignment record names the system `{}`, which the program does not declare: resolve again",
                    read.system
                ),
            )
        })?;
    let there = on_target(system, declarations, &read_target).map_err(|refused| {
        let said: Vec<String> = refused.iter().map(|d| format!("{}: {}", d.code, d.message)).collect();
        refuse(
            "tool-failure",
            format!(
                "the configuration checker accepted the record, and the system `{}` does not stand on the target `{}`: {}",
                read.system,
                target.name,
                said.join("; ")
            ),
        )
    })?;
    let (mut body, _) = body(
        program,
        record,
        None,
        vocabulary,
        catalog,
        Format::OnTarget,
        Some(&there),
    )?;
    // §7, in an application's unmet's place: its own `(target …)` against the plan's, the platform's name, then each thing `unmet` reports.
    if let Some((file, inputs)) = &application {
        if inputs.target != lines.platform {
            return Err(refuse(
                "invalid-description",
                format!(
                    "the application's file `{}` is of the target `{}`, and the plan is of `{}`",
                    file.name, inputs.target, lines.platform
                ),
            ));
        }
        let one = Plan::read(&body.write_one()).map_err(|why| {
            refuse(
                "tool-failure",
                format!("the lowering wrote a plan its own reader refuses: {why}"),
            )
        })?;
        let unmet = inputs.unmet_on(one.contents(), &lines.platform, &[]);
        if !unmet.is_empty() {
            return Err(refuse(
                "invalid-description",
                format!(
                    "the application's file `{}` does not meet the plan: {}",
                    file.name,
                    unmet.join("; ")
                ),
            ));
        }
    }
    // §5: its allocations, laid out once the application's file is known, which says what each stack's own code needs.
    let memory = body.memory.as_ref().map(|memory| {
        laid_out(
            &body,
            memory,
            application.as_ref().map(|(_, inputs)| inputs),
        )
    });
    body.memory = memory;
    let written = application
        .as_ref()
        .map(|(file, inputs)| inputs::lines(file.text, inputs));
    Ok((body, lines, written))
}

/// A lowering's body (§1 to §2.10): the record refused as §1 refuses it, or what `/1` writes of it, beside the result
/// record taken where there is one.
#[allow(clippy::too_many_lines)]
fn body(
    program: &Program,
    record: &str,
    result: Option<&str>,
    vocabulary: &Vocabulary,
    catalog: &Catalog,
    format: Format,
    there: Option<&OnTarget>,
) -> Result<(Body, Option<ResultRecord>), Refusal> {
    // §1, input 1: the declarations and the digest are the program's own, never its caller's; a module tree is read as
    // the language checks it, its names qualified.
    let flattened;
    let declarations: &[Form] = if program.instances.len() == 1 {
        &program.root().declarations
    } else {
        flattened = resolved_declarations(program).0;
        &flattened
    };
    let digest = assignment::program_digest(program);

    // §1: the record read back by the generator's own reader, and refused before anything is lowered, in the order
    // §1 writes its refusals, the first that applies given.
    let record_text = record;
    let record = assignment::read(record_text).map_err(|e| {
        refuse(
            "tool-failure",
            format!("the assignment record does not read: {e}"),
        )
    })?;
    if record.program != digest {
        return Err(refuse(
            "stale-input",
            "the assignment record names another program's digest: resolve again",
        ));
    }
    let system = record.system.clone();
    let decl = declarations
        .iter()
        .find(|d| {
            d.head() == Some("defsystem")
                && d.items().get(1).and_then(Form::as_symbol) == Some(system.as_str())
        })
        .ok_or_else(|| {
            refuse(
                "stale-input",
                format!(
                    "the assignment record names the system `{system}`, which the program does not declare: resolve again"
                ),
            )
        })?;
    if record.adapters != adapter::VERSION {
        return Err(refuse(
            "stale-input",
            format!(
                "the assignment record was resolved with the catalog `{}`, and this engine's is `{}`: resolve again",
                record.adapters,
                adapter::VERSION
            ),
        ));
    }
    let accepted = result
        .map(|text| take_result(text, record_text, &record, &digest))
        .transpose()?;
    if record.profile != RT_STATIC_UP_V1.id {
        return Err(refuse(
            "unsupported-profile",
            format!(
                "`/1` lowers `{}`'s rules alone, and the record was resolved under `{}`",
                RT_STATIC_UP_V1.id, record.profile
            ),
        ));
    }
    // On a target the system reads what `on_target` gives it, the target's platform in its own's place (leaf `M4.1.7.6.7.1`).
    let (declarations, decl): (&[Form], &Form) = match there {
        Some(there) => (&there.declarations, &there.system),
        None => (declarations, decl),
    };
    let rebuilt = assignment::rebuild(
        &record,
        declarations,
        RT_STATIC_UP_V1.caller_level,
        vocabulary,
        catalog,
    )
    .map_err(|e| {
        refuse(
            "tool-failure",
            format!(
                "the configuration checker accepted the assignment record, and it does not rebuild over the program: {e}"
            ),
        )
    })?;
    let report = rebuilt.joint.check(&rebuilt.bindings);
    let findings: Vec<String> = report
        .conflicts
        .iter()
        .chain(&report.unknowns)
        .chain(&rebuilt.map.refused)
        .chain(&rebuilt.map.unknowns)
        .map(|f| format!("{}: {}", f.code(), f.diagnostic().message))
        .collect();
    if !findings.is_empty() {
        return Err(refuse(
            "tool-failure",
            format!(
                "the configuration checker accepted the record, and the generator's joint rules refuse it ({}): the two roles disagree",
                findings.join("; ")
            ),
        ));
    }

    // §2.2: the system's tasks, in runtime order.
    let clause = |task: &Form, head: &str| -> Option<Form> {
        task.items()
            .iter()
            .find(|i| i.head() == Some(head))
            .cloned()
    };
    let mut tasks: Vec<Task<'_>> = decl
        .items()
        .iter()
        .filter(|i| i.head() == Some("task"))
        .map(|form| Task {
            name: form
                .items()
                .get(1)
                .and_then(Form::as_symbol)
                .unwrap_or_default()
                .to_string(),
            form,
            rank: clause(form, "priority")
                .and_then(|c| match c.items().get(1) {
                    Some(Form::Integer { value, .. }) => Some(*value),
                    _ => None,
                })
                .unwrap_or(i64::MAX),
        })
        .collect();
    tasks.sort_by(|a, b| a.rank.cmp(&b.rank).then(a.name.cmp(&b.name)));
    // §1: a task's release or deadline the description check should have refused — none stated, or a periodic task bound
    // to an interrupt source itself, a second release model. One that is no time, or a release separation of zero, the
    // language refuses (`M1.42`, model §4 rule 8), so an accepted description holds neither.
    let ill_formed = |task: &Task<'_>, what: String, leaf: &str| {
        refuse(
            "tool-failure",
            format!(
                "`(task {system} {})` {what}: the description check accepted an ill-formed description, which `{leaf}` makes it refuse",
                task.name
            ),
        )
    };
    let mut timings: Vec<Timing> = Vec::new();
    // Each task's duration past the exact arithmetic, in runtime order, refused first among `/1`'s limits (§1, §2.2).
    let mut past: Vec<String> = Vec::new();
    for task in &tasks {
        let period = clause(task.form, "period");
        let (Some(release), Some(deadline)) = (
            period
                .clone()
                .or_else(|| clause(task.form, "min-separation")),
            clause(task.form, "deadline"),
        ) else {
            return Err(refuse(
                "tool-failure",
                format!(
                    "`(task {system} {})` states no release or no deadline, which the description check refuses: it accepted what it refuses",
                    task.name
                ),
            ));
        };
        let jitter = clause(task.form, "jitter");
        let mut time = |c: &Form| {
            match seconds(c) {
            Seconds::Of(s) => Ok(Some(s)),
            Seconds::NotTime => Err(refuse(
                "tool-failure",
                format!(
                    "`(task {system} {})` writes `{}`, which is not a time, and the language's check refuses that: \
                     report it, the two disagree",
                    task.name,
                    c.to_canonical()
                ),
            )),
            Seconds::PastArithmetic => {
                past.push(format!(
                    "`(task {system} {})` writes `{}`",
                    task.name,
                    c.to_canonical()
                ));
                Ok(None)
            }
        }
        };
        let release_seconds = time(&release)?;
        let deadline_seconds = time(&deadline)?;
        let jitter_seconds = match &jitter {
            Some(c) => time(c)?,
            None => Some(Rational::ZERO),
        };
        // A duration past the arithmetic is refused below; its place holds zero until then.
        let timing = Timing {
            periodic: period.is_some(),
            release: release_seconds.unwrap_or(Rational::ZERO),
            deadline: deadline_seconds.unwrap_or(Rational::ZERO),
            jitter: jitter_seconds.unwrap_or(Rational::ZERO),
        };
        let internal = joint::task_name(&system, &task.name);
        if timing.periodic
            && rebuilt.bindings.iter().any(|b| {
                b.requester == internal
                    && b.requirement.fact() == INTERRUPT_SOURCE
                    && met_by_true(&b.requirement)
            })
        {
            let period = period.as_ref().expect("a periodic task writes its period");
            return Err(ill_formed(
                task,
                format!(
                    "writes `{}` and is bound to an interrupt source itself, a second release model",
                    period.to_canonical()
                ),
                "M1.43",
            ));
        }
        timings.push(timing);
    }
    // §1: a `uses` or `needs` naming a system, each named (R11-D3); its own level would become a node §2 does not give.
    let is_system = |name: &str| {
        declarations.iter().any(|d| {
            d.head() == Some("defsystem")
                && d.items().get(1).and_then(Form::as_symbol) == Some(name)
        })
    };
    // Each pair by the namer's text, then the system's, compared byte by byte, never by the message's words (§1).
    // A block or a platform whose needs bind nothing among them: what it names is the language's to judge (leaf `M3.4.17.6`).
    let mut naming_systems: Vec<(&str, &str)> = rebuilt
        .requesters
        .iter()
        .chain(&rebuilt.uncounted)
        .filter(|r| rebuilt.system.serves(&r.name))
        .flat_map(|r| {
            r.uses
                .iter()
                .chain(&r.needs)
                .filter(|n| is_system(n))
                .map(move |n| (r.name.as_str(), n.as_str()))
        })
        .collect();
    naming_systems.sort();
    naming_systems.dedup();
    if !naming_systems.is_empty() {
        let named: Vec<String> = naming_systems
            .iter()
            .map(|(namer, n)| format!("`{namer}` names the system `{n}`"))
            .collect();
        return Err(refuse(
            "tool-failure",
            format!(
                "{}: a system is no thing to use, and the description check accepted it, which `M1.46` makes it refuse",
                named.join("; ")
            ),
        ));
    }
    // §1: a block or a platform the system serves naming what is not hardware, each named (R13-D1, R13-D3); only what
    // the system serves, as the refusal above (R15-R2).
    let is_hardware_decl = |d: &Form| matches!(d.head(), Some("defblock" | "defplatform"));
    let declared = |name: &str| {
        declarations
            .iter()
            .find(|d| d.items().get(1).and_then(Form::as_symbol) == Some(name))
    };
    // Each pair by the block's or platform's text, then the operand's, byte by byte (§1).
    let mut composing_other: Vec<(&str, &str)> = rebuilt
        .requesters
        .iter()
        .chain(&rebuilt.uncounted)
        .filter(|r| {
            rebuilt.system.serves(&r.name) && declared(&r.name).is_some_and(is_hardware_decl)
        })
        .flat_map(|r| {
            r.uses
                .iter()
                .chain(&r.needs)
                .filter(|n| declared(n).is_some_and(|d| !is_hardware_decl(d)))
                .map(move |n| (r.name.as_str(), n.as_str()))
        })
        .collect();
    composing_other.sort();
    composing_other.dedup();
    if !composing_other.is_empty() {
        let named: Vec<String> = composing_other
            .iter()
            .map(|(namer, n)| format!("`{namer}` names `{n}`"))
            .collect();
        return Err(refuse(
            "tool-failure",
            format!(
                "{}: a block's or a platform's `uses` and `needs` name only hardware, and the description check accepted these, which `M1.47` makes it refuse",
                named.join("; ")
            ),
        ));
    }

    // §1: what `/1` cannot lower — first, a duration the plan would write past the exact arithmetic: a task's, a
    // source's separation, an obliged bound (§2.2).
    let past_sources = report.obligations.iter().filter_map(|o| match o {
        Obligation::Route {
            provider,
            interrupt:
                Interrupt::Source {
                    separation: Some(State::Valued(values)),
                },
            ..
        } if matches!(
            values.first().map(value_seconds),
            Some(Seconds::PastArithmetic)
        ) =>
        {
            Some(format!("`(source {provider})` states a separation"))
        }
        // Spellings whose comparison overflows: the fact is past the arithmetic at its provider (relation record §5).
        Obligation::Route {
            provider,
            interrupt:
                Interrupt::Source {
                    separation: Some(State::Overflow),
                },
            ..
        } => Some(format!("`(source {provider})` states a separation")),
        _ => None,
    });
    let mut past_sources: Vec<String> = past_sources.collect();
    past_sources.sort();
    let system_text = system.as_str();
    let past_obliged = rebuilt.instances.iter().flat_map(|instance| {
        catalog
            .kind(&instance.kind)
            .into_iter()
            .flat_map(|k| k.offers.iter())
            .filter(|o| o.offered == Offered::Obliged)
            .filter_map(move |offer| match instance.provider.state(&offer.fact) {
                Some(State::Valued(values))
                    if matches!(
                        values.first().map(value_seconds),
                        Some(Seconds::PastArithmetic)
                    ) =>
                {
                    Some((
                        assignment::instance_text(instance, system_text),
                        offer.fact.to_string(),
                    ))
                }
                Some(State::Overflow) => Some((
                    assignment::instance_text(instance, system_text),
                    offer.fact.to_string(),
                )),
                _ => None,
            })
    });
    // By the instance's text, then the fact's, byte by byte (§1).
    let mut past_obliged: Vec<(String, String)> = past_obliged.collect();
    past_obliged.sort();
    let past_obliged = past_obliged
        .into_iter()
        .map(|(instance, fact)| format!("`{instance}` is obliged to a `{fact}`"));
    let past: Vec<String> = past
        .into_iter()
        .chain(past_sources)
        .chain(past_obliged)
        .collect();
    if !past.is_empty() {
        return Err(refuse(
            "unsupported-profile",
            format!(
                "each of these is a duration whose seconds lie past the engine's exact arithmetic, signed 128-bit integers (§2.2) — {}: state each within that range; `M1.45` has the language refuse them",
                past.join("; ")
            ),
        ));
    }
    let placed = record.placement.is_some()
        || rebuilt
            .map
            .obligations
            .iter()
            .any(|o| matches!(o, Obligation::Placement { .. }));
    // A plan on a target lays its placement memory out (§5); `/1` and a hosted `/2` refuse it.
    let refused_placement = match format {
        Format::One => Some("the system has placement memory, and laying an image out in memory is `/2`'s (`M4.1.7`): describe a hosted platform, or wait for `/2`"),
        // `/2` §1: a hosted plan lays out no memory, so placement memory stays refused, its repair naming a target (RP37-3).
        Format::Two => Some("the system has placement memory, and a hosted plan lays no image out in memory: describe a hosted platform, or name the target the system runs on, where `/2` lays the image out"),
        Format::OnTarget => None,
    };
    if let (true, Some(why)) = (placed, refused_placement) {
        return Err(refuse("unsupported-profile", why));
    }
    if tasks.is_empty() {
        return Err(refuse(
            "unsupported-profile",
            format!(
                "the system `{system}` declares no task, and the scheduler a build runs has no empty form (`rt_core::Scheduler::from_eadl_ranks`, `BootError::NoTasks`): describe a task"
            ),
        ));
    }
    for task in &tasks {
        let event = clause(task.form, "deadline-from").and_then(|c| {
            c.items()
                .get(1)
                .and_then(Form::as_symbol)
                .map(str::to_string)
        });
        if let Some(event) = event.filter(|e| e != "release") {
            return Err(refuse(
                "unsupported-profile",
                format!(
                    "`(task {system} {})`'s deadline is measured from `{event}`, and the analysis measures every relative deadline from the nominal release",
                    task.name
                ),
            ));
        }
    }
    let init = rebuilt
        .joint
        .initialization(&rebuilt.system, &rebuilt.bindings);
    let names = Names {
        system: system.clone(),
        instances: rebuilt
            .instances
            .iter()
            .map(|i| (i.provider.name.clone(), i))
            .collect(),
    };
    let nodes: BTreeSet<String> = init.nodes.iter().map(|(n, _)| n.clone()).collect();
    let requester = |name: &str| rebuilt.requesters.iter().find(|r| r.name == name);
    let is_task = |name: &str| requester(name).is_some_and(|r| r.kind == Kind::Task);

    // §2: reaching — what a requester names by `uses` or `needs`, and what it is bound to as its provider, at any depth,
    // as the joint rules read it (model §8 rule 4).
    let reach = |from: &str| rebuilt.joint.reaches(from, &rebuilt.bindings);

    // §2.5: the routes, as `Joint::check` reports them, one for each provider and kind.
    let routes: Vec<Route> = report
        .obligations
        .iter()
        .filter_map(|o| match o {
            Obligation::Route {
                provider,
                interrupt,
                requesters,
                ..
            } => Some(Route {
                provider: provider.clone(),
                timer: matches!(interrupt, Interrupt::Timer),
                separation: match interrupt {
                    Interrupt::Timer => Err("programmed"),
                    Interrupt::Source { separation } => match separation {
                        // Past the arithmetic is refused above.
                        Some(State::Valued(values)) => match values.first().map(value_seconds) {
                            Some(Seconds::Of(s)) => Ok(s),
                            _ => Err("unknown"),
                        },
                        Some(State::Absent) => Err("absent"),
                        _ => Err("unknown"),
                    },
                },
                requesters: requesters.clone(),
            }),
            _ => None,
        })
        .collect();
    let source_routes: Vec<&Route> = routes.iter().filter(|r| !r.timer).collect();

    // §1: a task, or the system's own level, that would receive an interrupt's code is refused.
    let own_level_or_task = |name: &str| name == system || is_task(name);
    let bound_directly = |r: &str, x: &str, fact_ok: &dyn Fn(&Requirement) -> bool| {
        rebuilt
            .bindings
            .iter()
            .any(|b: &Binding| b.requester == r && b.provider == x && fact_ok(&b.requirement))
    };
    // Each receiver's place: a task's runtime index, the system's own level after every task (§1).
    let place = |name: &str| {
        tasks
            .iter()
            .position(|t| joint::task_name(&system, &t.name) == name)
            .unwrap_or(tasks.len())
    };
    // Each receiver in its place; within it, its alarms (0) and then its expiries (1), each by the text the message
    // names — the provider's, the delivering instance's — byte by byte (§1).
    let mut receivers: Vec<(usize, u8, String, String)> = Vec::new();
    for route in routes.iter().filter(|r| r.timer) {
        for r in &route.requesters {
            if own_level_or_task(r)
                && bound_directly(r, &route.provider, &|q| takes_a_channel(q, vocabulary))
            {
                receivers.push((
                    place(r),
                    0,
                    route.provider.clone(),
                    format!(
                        "`{}` programs an alarm at `{}` itself",
                        names.text(r),
                        route.provider
                    ),
                ));
            }
        }
    }
    for instance in rebuilt.instances.iter().filter(|i| i.delivers) {
        // Its requesters: the one an exclusive instance serves, and each bound to it, a shared instance's included.
        let served = instance.serves.iter().chain(
            rebuilt
                .bindings
                .iter()
                .filter(|b| b.provider == instance.provider.name)
                .map(|b| &b.requester),
        );
        for served in served {
            if own_level_or_task(served) {
                receivers.push((
                    place(served),
                    1,
                    names.text(&instance.provider.name),
                    format!(
                        "`{}` delivers an expiry to `{}`",
                        names.text(&instance.provider.name),
                        names.text(served)
                    ),
                ));
            }
        }
    }
    receivers.sort();
    receivers.dedup();
    if !receivers.is_empty() {
        let named: Vec<String> = receivers.into_iter().map(|(.., what)| what).collect();
        return Err(refuse(
            "unsupported-profile",
            format!(
                "{}: no context runs a job's code in an interrupt, and how a job receives an alarm it programs itself or an expiry, and what the system's own level's requirement for either means, are `M4.1.6`'s",
                named.join("; ")
            ),
        ));
    }

    // §2.2's lines, and what each task owes.
    let mut owed: Vec<String> = Vec::new();
    let owe = |owed: &mut Vec<String>, subject: &str, fact: &str, owner: &str| {
        owed.push(format!("(owed {subject} {fact} (by {owner}))"));
    };
    let mut task_lines = Vec::new();
    let mut releases = Vec::new();
    for (task, timing) in tasks.iter().zip(&timings) {
        let internal = joint::task_name(&system, &task.name);
        let text = names.text(&internal);
        // Its runtime index, `|hp(i)|`, the number of tasks of smaller rank (§2.2).
        let index = tasks.iter().filter(|t| t.rank < task.rank).count();
        let release = if timing.periodic {
            format!("(period {})", duration(timing.release))
        } else {
            format!("(min-separation {})", duration(timing.release))
        };
        let deadline = duration(timing.deadline);
        // Any event but the release is refused above (§1).
        let reference = if clause(task.form, "deadline-from").is_some() {
            "(deadline-from release)"
        } else {
            owe(&mut owed, &text, "deadline-from", "description");
            "(deadline-from)"
        };
        let jitter = timing.jitter;
        let overrun = clause(task.form, "on-overrun")
            .and_then(|c| {
                c.items()
                    .get(1)
                    .and_then(Form::as_symbol)
                    .map(str::to_string)
            })
            .unwrap_or_else(|| "fault".to_string());
        // Every node it names by `uses` or `needs`, each once: a service, a policy, or a block or a platform of the
        // system's hardware, whose code the name calls (§2.2, R24-D1).
        let uses: BTreeSet<String> = requester(&internal)
            .map(|r| {
                r.uses
                    .iter()
                    .chain(&r.needs)
                    .filter(|n| nodes.contains(n.as_str()))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        // What releases it (§2.2).
        let trigger = if timing.periodic {
            releases.push(text.clone());
            "timer".to_string()
        } else {
            // The source it is bound to itself for `interrupt-source` at `true`, where it is bound to one; otherwise the
            // sources whose routes name it or a requester it reaches (§2.2, R16-B1).
            let own = rebuilt
                .bindings
                .iter()
                .find(|b| {
                    b.requester == internal
                        && b.requirement.fact() == INTERRUPT_SOURCE
                        && met_by_true(&b.requirement)
                })
                .map(|b| b.provider.as_str());
            let reached = reach(&internal);
            let releasing: Vec<&&Route> = source_routes
                .iter()
                .filter(|r| match own {
                    Some(x) => r.provider == x,
                    None => r
                        .requesters
                        .iter()
                        .any(|q| *q == internal || reached.contains(q)),
                })
                .collect();
            match releasing.as_slice() {
                [route] if route.separation.is_ok_and(|s| s >= timing.release) => {
                    format!("(source {} every)", route.provider)
                }
                releasing => {
                    owe(&mut owed, &text, "trigger", "description");
                    // Its cause, so that no consumer re-derives this rule to choose `RTP-H1`'s code (§2.2, R24-R1): the
                    // one source the task binds itself written apart from the one the routes leave, since the task's
                    // own `(needs interrupt-source)` repairs the second and not the first (R28-D2).
                    let own_ = if own.is_some() { "own-" } else { "" };
                    let cause = match releasing {
                        [] => "unreached".to_string(),
                        [route] => match route.separation {
                            Ok(_) => format!("({own_}faster {})", route.item()),
                            Err("absent") => format!("({own_}absent {})", route.item()),
                            Err(_) => format!("({own_}unstated {})", route.item()),
                        },
                        several => {
                            let mut items: Vec<String> = several.iter().map(|r| r.item()).collect();
                            items.sort();
                            format!("(several {})", items.join(" "))
                        }
                    };
                    format!("(unknown {cause})")
                }
            }
        };
        task_lines.push(format!(
            "(task {index} {text} (rank {}) {release} (deadline {deadline}) {reference} (jitter {}) (overrun {overrun}) (released-by {trigger}) {})",
            task.rank,
            duration(jitter),
            list("uses", &uses.into_iter().collect::<Vec<_>>())
        ));
        for fact in [
            "entry-point",
            "execution-bound",
            "execution-category",
            "figures-under-preemption",
            "masked-category",
            "masked-runs",
            "masks-only-through-api",
            "os-calls",
            "os-calls-category",
            "stack-bound",
            "stack-category",
            "task-facts",
        ] {
            owe(&mut owed, &text, fact, "application");
        }
    }
    owe(
        &mut owed,
        &system,
        "leaves-interrupt-hardware-alone",
        "application",
    );

    // §2.3: the bindings, copied as written.
    let bind_lines: Vec<String> = record
        .binds
        .iter()
        .map(|b| format!("(bind {} {} {})", b.requester, b.fact, b.provider))
        .collect();

    // §2.4: the instances.
    let mut instance_lines: Vec<String> = Vec::new();
    for instance in &rebuilt.instances {
        let text = names.text(&instance.provider.name);
        let Some(kind) = catalog.kind(&instance.kind) else {
            continue;
        };
        let mut items = Vec::new();
        for offer in kind.offers.iter().filter(|o| o.offered == Offered::Obliged) {
            match instance.provider.state(&offer.fact) {
                Some(State::Valued(values)) => {
                    let Some(Seconds::Of(s)) = values.first().map(value_seconds) else {
                        return Err(refuse(
                            "tool-failure",
                            format!(
                                "`{text}` is obliged to `{}` at a value that is not a duration (§2.4)",
                                offer.fact
                            ),
                        ));
                    };
                    items.push(format!("(obliged {} {})", offer.fact, duration(s)));
                    owe(&mut owed, &text, &offer.fact, "realization");
                }
                _ => {
                    items.push(format!("(obliged {})", offer.fact));
                    owe(&mut owed, &text, &offer.fact, "realization");
                }
            }
        }
        let obligations: Vec<String> = kind.obligations.iter().map(|o| o.name.clone()).collect();
        let costs: Vec<String> = kind.costs.iter().map(|c| c.name.clone()).collect();
        items.push(list("obligations", &obligations));
        items.push(list("costs", &costs));
        instance_lines.push(format!("(instance {text} {})", items.join(" ")));
        if !costs.is_empty() {
            owe(&mut owed, &text, "cost-terms", "realization");
        }
    }
    instance_lines.sort();

    // §2.5: the routes ranked — sources with a stated separation, ascending, ties on the interrupt's text; then the
    // sources whose separation is `unknown` or `absent`; then the timer routes, each by its interrupt's text.
    let mut ranked: Vec<&Route> = routes.iter().collect();
    ranked.sort_by(|a, b| {
        let class = |r: &Route| match (r.timer, &r.separation) {
            (false, Ok(_)) => 0,
            (false, Err(_)) => 1,
            (true, _) => 2,
        };
        class(a)
            .cmp(&class(b))
            .then_with(|| match (&a.separation, &b.separation) {
                (Ok(x), Ok(y)) if !a.timer && !b.timer => x.cmp(y),
                _ => std::cmp::Ordering::Equal,
            })
            .then_with(|| a.item().cmp(&b.item()))
    });
    let instance_over = |provider: &str| -> Vec<&Instance> {
        rebuilt
            .instances
            .iter()
            .filter(|i| {
                i.over == provider
                    && bound_directly(&i.provider.name, provider, &|q| {
                        takes_a_channel(q, vocabulary)
                    })
            })
            .collect()
    };
    let mut route_lines = Vec::new();
    for (at, route) in ranked.iter().enumerate() {
        let mut requesters: Vec<String> = route.requesters.iter().map(|r| names.text(r)).collect();
        requesters.sort();
        let mut costs: Vec<String> = Vec::new();
        if route.timer {
            for instance in instance_over(&route.provider) {
                if let Some(kind) = catalog.kind(&instance.kind) {
                    for cost in &kind.costs {
                        costs.push(format!(
                            "({} {})",
                            names.text(&instance.provider.name),
                            cost.name
                        ));
                    }
                }
            }
        }
        costs.sort();
        costs.dedup();
        let separation = match &route.separation {
            Ok(s) => duration(*s),
            Err(word) => (*word).to_string(),
        };
        route_lines.push(format!(
            "(route {} {} (separation {separation}) (level {}) (deliverable yes) {} {})",
            at + 1,
            route.item(),
            RT_STATIC_UP_V1.caller_level,
            list("requesters", &requesters),
            list("costs", &costs)
        ));
        let item = route.item();
        if route.timer {
            owe(&mut owed, &item, "arrival-bound", "realization");
            owe(&mut owed, &item, "service-cost", "realization");
        } else {
            for fact in [
                "acknowledge-point",
                "deferred-work",
                "no-application-code",
                "one-request-per-arrival",
                "service-cost",
            ] {
                owe(&mut owed, &item, fact, "realization");
            }
            if route.separation == Err("unknown") {
                owe(&mut owed, &item, "separation", "description");
            }
        }
    }
    let enabled: Vec<String> = std::iter::once("timer".to_string())
        .chain(ranked.iter().map(|r| r.item()))
        .collect();

    // §2.6: the execution contexts, as the joint rules derive them (model §8 rule 4).
    let derived = rebuilt.joint.contexts(&rebuilt.system, &rebuilt.bindings);
    let contexts: BTreeMap<String, BTreeSet<String>> = derived
        .of
        .iter()
        .map(|(subject, set)| {
            let words = set.iter().map(|context| match context {
                Context::Job => "job".to_string(),
                Context::Routine(Routine { provider, timer }) => {
                    format!("(interrupt {})", interrupt_text(provider, *timer))
                }
                Context::Initialisation => "initialisation".to_string(),
            });
            (subject.clone(), words.collect())
        })
        .collect();
    let mut context_lines: Vec<String> = contexts
        .iter()
        .map(|(subject, set)| {
            let mut ordered: Vec<&String> = Vec::new();
            if set.contains("job") {
                ordered.push(set.get("job").expect("held"));
            }
            ordered.extend(set.iter().filter(|c| c.starts_with("(interrupt ")));
            if set.contains("initialisation") {
                ordered.push(set.get("initialisation").expect("held"));
            }
            let words: Vec<String> = ordered.into_iter().cloned().collect();
            format!("(context {} {})", names.text(subject), words.join(" "))
        })
        .collect();
    context_lines.sort();

    // §2.7: the graph's edges with their reasons, and its steps.
    let mut edges: BTreeMap<(String, String), (BTreeSet<String>, bool)> = BTreeMap::new();
    for (from, to, because) in &init.edges {
        let entry = edges.entry((names.text(from), names.text(to))).or_default();
        match because {
            Because::Binding(fact) => {
                entry.0.insert(fact.clone());
            }
            Because::Named => entry.1 = true,
        }
    }
    let depends_lines: Vec<String> = edges
        .iter()
        .map(|((from, to), (facts, named))| {
            let mut reasons: Vec<String> = facts.iter().map(|f| format!("(binding {f})")).collect();
            if *named {
                reasons.push("named".to_string());
            }
            format!("(depends {from} {to} {})", list("because", &reasons))
        })
        .collect();
    let steps = steps(&init.nodes, &init.edges, &init.components, &names);
    let step_lines: Vec<String> = steps
        .iter()
        .enumerate()
        .map(|(at, members)| format!("(step {} {})", at + 1, members.join(" ")))
        .collect();

    // §2.9: the obligations.
    let mut obligation_lines: Vec<String> = Vec::new();
    for instance in &rebuilt.instances {
        if let Some(kind) = catalog.kind(&instance.kind) {
            for o in &kind.obligations {
                obligation_lines.push(format!(
                    "(obligation {} {} (discharged-by M4.2 M4.8))",
                    o.name,
                    names.text(&instance.provider.name)
                ));
            }
        }
    }
    // A node that takes a channel — a service, a policy, or hardware with requirements of its own; an instance aside —
    // is multiplexed where it has two or more users: each of the system's tasks that reaches it, and each interrupt
    // whose context line holds it, but for its own timer route — at the provider whose channel it takes, or that the
    // instance whose channel it takes programs — where nothing else that route runs reaches it (§2.9). The joint rules
    // report it so (model §8 rule 4), and the record holds, so they did.
    let multiplexed: BTreeSet<&str> = report
        .obligations
        .iter()
        .filter_map(|o| match o {
            Obligation::Multiplex { node, .. } => Some(node.as_str()),
            _ => None,
        })
        .collect();
    for service in multiplexed {
        let text = names.text(service);
        obligation_lines.push(format!(
            "(obligation multiplex {text} (discharged-by M4.2 M2.21.2))"
        ));
        owe(&mut owed, &text, "overhead", "realization");
    }
    for route in &routes {
        obligation_lines.push(format!(
            "(obligation route {} (discharged-by M4.2 M4.3))",
            route.item()
        ));
    }
    obligation_lines.sort();
    obligation_lines.dedup();

    // §2.10: what each node owes.
    for n in &nodes {
        let text = names.text(n);
        owe(&mut owed, &text, "contexts", "realization");
        owe(&mut owed, &text, "no-wait", "realization");
        // The discipline of the code the realization generates for it, which every composite reads (§2.10, R16-D3).
        owe(&mut owed, &text, "runtime-discipline", "realization");
        if contexts.get(n).is_some_and(|c| c.contains("job")) {
            owe(&mut owed, &text, "call-cost", "realization");
            // Its code's masked runs in a job, as a task's, their evidence category in their record (§2.10, R17-R2, R18-D3).
            owe(&mut owed, &text, "masked-runs", "realization");
        }
    }
    owed.sort();
    owed.dedup();

    // Each node with its kind, by its text (§3, R24-D2): the simulator and the emitter read which nodes are devices here.
    let kind_of = |n: &str| -> &'static str {
        if names.instances.contains_key(n) {
            return "instance";
        }
        match declarations
            .iter()
            .find(|d| d.items().get(1).and_then(Form::as_symbol) == Some(n))
            .and_then(Form::head)
        {
            Some("defblock") => "block",
            Some("defplatform") => "platform",
            Some("defservice") => "service",
            _ => "policy",
        }
    };
    let mut node_lines: Vec<String> = nodes
        .iter()
        .map(|n| format!("(node {} {})", names.text(n), kind_of(n)))
        .collect();
    node_lines.sort();
    // `/2`'s additions to what is owed (§3), whatever `/1` owes beside them.
    let mut owed_by_two = Vec::new();
    for task in &tasks {
        let text = names.text(&joint::task_name(&system, &task.name));
        owe(&mut owed_by_two, &text, "node-calls", "application");
        owe(
            &mut owed_by_two,
            &text,
            "node-calls-category",
            "application",
        );
    }
    owe(&mut owed_by_two, &system, "initialisation", "application");
    for n in nodes
        .iter()
        .filter(|n| contexts.get(*n).is_some_and(|c| c.contains("job")))
    {
        let text = names.text(n);
        owe(
            &mut owed_by_two,
            &text,
            "primitives-out-of-line",
            "realization",
        );
        owe(&mut owed_by_two, &text, "masking-balanced", "realization");
    }
    // `/2`'s regions (§5, "Which regions"): each region of the system's hardware, its use as §3 gives one and the
    // requesters bound to its provider, in the assignment's order, by base, then name, then provider.
    let mut regions: Vec<(u128, String, String, String)> = rebuilt
        .joint
        .regions(&rebuilt.system)
        .into_iter()
        .map(|(provider, region)| {
            let provider_text = names.text(provider);
            let mut requesters: Vec<String> = Vec::new();
            for bind in record.binds.iter().filter(|b| b.provider == provider_text) {
                if !requesters.contains(&bind.requester) {
                    requesters.push(bind.requester.clone());
                }
            }
            let chosen = record
                .placement
                .as_ref()
                .is_some_and(|(p, r)| *p == provider_text && *r == region.name);
            let usage = match (region.reserved, region.executable, chosen) {
                (true, _, _) => "reserved",
                (false, true, true) => "placement",
                (false, true, false) => "executable",
                (false, false, _) => "device",
            };
            let size = region
                .bytes
                .map_or_else(|| "unsized".to_string(), |bytes| bytes.to_string());
            let line = format!(
                "(region {} {provider_text} (base {}) (size {size}) (use {usage}) {})",
                region.name,
                region.base,
                list("requesters", &requesters)
            );
            (
                u128::try_from(region.base).unwrap_or(0),
                region.name.clone(),
                provider_text,
                line,
            )
        })
        .collect();
    regions.sort();
    let region_lines: Vec<String> = regions.into_iter().map(|(_, _, _, line)| line).collect();
    let task_names: Vec<String> = tasks
        .iter()
        .map(|task| joint::task_name(&system, &task.name))
        .collect();
    // On a target, the device the release requester is bound to, the release timer's (`decision_release-alarm.md` §1),
    // every binding of it at one device, rule 13 holding its facts of the function `counter` to one block.
    let release_name = joint::release_name(&system);
    let release_devices: BTreeSet<&str> = rebuilt
        .bindings
        .iter()
        .filter(|b| there.is_some() && b.requester == release_name)
        .map(|b| b.provider.as_str())
        .collect();
    if release_devices.len() > 1 {
        return Err(refuse(
            "tool-failure",
            format!(
                "the release requester of `{system}` is bound at more than one device (rule 13)"
            ),
        ));
    }
    let release_device = release_devices.into_iter().next();
    let release = there.map(|_| Release {
        device: release_device,
    });
    let (two, owed_by_alarms) = alarms_of(
        &routes,
        &contexts,
        &nodes,
        &obligation_lines,
        &names,
        &rebuilt,
        declarations,
        vocabulary,
        catalog,
        &task_names,
        release.as_ref(),
    )?;
    // On a target, its time (§3's `(timer …)` and `(epoch …)`, §4; leaf `M4.1.7.6.7.2`): the release timer's line, its
    // device's counter as `COUNTER` writes it, its `unambiguous-horizon` derived of that counter, its rounding owed until a
    // catalog record states it; or `(timer none)` where no task is released by the timer; then the epoch's line, and what
    // each owes.
    let time = match there {
        None => None,
        Some(_) => {
            let timer = match release_device {
                None if !releases.is_empty() => {
                    return Err(refuse(
                        "tool-failure",
                        format!("a task of `{system}` is released by the timer, and no release requester is bound (§3)"),
                    ))
                }
                None => "(timer none)".to_string(),
                Some(device) => {
                    let (counted, _) = counter_line(device, declarations, vocabulary)?;
                    let (rate, modulus) = counter_of(device, declarations, vocabulary);
                    let horizon = rate
                        .zip(modulus)
                        .and_then(|(rate, modulus)| eadl_resolve::value::horizon(modulus, rate).ok())
                        .and_then(|horizon| horizon.in_base().ok())
                        .ok_or_else(|| {
                            refuse(
                                "tool-failure",
                                format!(
                                    "the release timer's device `{device}` has no `unambiguous-horizon` its counter derives (§3)"
                                ),
                            )
                        })?;
                    format!(
                        "(timer (device {}) {counted} (horizon {}) (rounding owed) (first-compare transition) {})",
                        names.text(device),
                        duration(horizon),
                        list("releases", &releases)
                    )
                }
            };
            let alarmed = !two.alarms.is_empty();
            let epoch = alarm::epoch_line(false, release_device.is_some(), alarmed, "owed");
            let mut owed = Vec::new();
            if release_device.is_some() {
                owed.push("(owed (timer) compare-rounding (by catalog))".to_string());
            }
            if epoch.contains("(lead owed)") {
                owed.push("(owed (epoch) epoch-lead (by catalog))".to_string());
            }
            if alarmed {
                owed.push("(owed (epoch) last-act-lead (by realization))".to_string());
            }
            Some(Time { timer, epoch, owed })
        }
    };
    let task_order: Vec<(String, String)> = tasks
        .iter()
        .map(|task| {
            (
                task.name.clone(),
                names.text(&joint::task_name(&system, &task.name)),
            )
        })
        .collect();
    owed_by_two.extend(owed_by_alarms);
    // On a target, its placement line and what a target adds to what is owed (§3, "`(owed …)`, as `/2` writes it", and
    // §5, "Which is placement memory"; leaf `M4.1.7.6.7.3`): `external` of a source on a target, and of each alarm, a
    // source of its own, whether its interrupt reaches the controller; `holds-under-preemption` of each node whose context
    // line holds `job`, each alarm and each source on a target, all by the realization.
    let (placement, target_owed) = match there {
        None => (None, Vec::new()),
        Some(_) => {
            let placement = record
                .placement
                .as_ref()
                .map(|(provider, region)| format!("(placement {provider} {region})"));
            let sources: Vec<String> = routes
                .iter()
                .filter(|r| !r.timer)
                .map(Route::item)
                .collect();
            let mut target_owed = Vec::new();
            for item in sources.iter().chain(&two.items) {
                owe(&mut target_owed, item, "external", "realization");
                owe(
                    &mut target_owed,
                    item,
                    "holds-under-preemption",
                    "realization",
                );
            }
            for n in nodes
                .iter()
                .filter(|n| contexts.get(*n).is_some_and(|c| c.contains("job")))
            {
                owe(
                    &mut target_owed,
                    &names.text(n),
                    "holds-under-preemption",
                    "realization",
                );
            }
            (placement, target_owed)
        }
    };
    // On a target, its image and its placement obligations (§5, "The image lies at the bottom", "Which regions"; §3,
    // `(obligation …)`; leaf `M4.1.7.6.8`); what lays out its allocations waits for the application's file (`laid_out`).
    let memory = match there {
        None => None,
        Some(_) => Some(memory_of(
            &rebuilt,
            &names,
            record.placement.as_ref(),
            step_lines.len(),
            two.enabled
                .iter()
                .filter(|item| *item != "timer")
                .cloned()
                .collect(),
            time.as_ref()
                .is_some_and(|time| time.timer != "(timer none)"),
        )?),
    };
    Ok((
        Body {
            profile: record.profile.clone(),
            system: system.clone(),
            program: digest.clone(),
            assignment: sha256(record_text),
            adapters: record.adapters.clone(),
            tasks: task_lines,
            binds: bind_lines,
            instances: instance_lines,
            releases,
            routes: route_lines,
            enabled,
            nodes: node_lines,
            contexts: context_lines,
            depends: depends_lines,
            steps: step_lines,
            obligations: obligation_lines,
            owed,
            owed_by_two,
            regions: region_lines,
            two,
            task_order,
            time,
            placement,
            target_owed,
            memory,
        },
        accepted,
    ))
}

/// A target's image and placement obligations, and what its memory waits on but the application's file (§5; leaf
/// `M4.1.7.6.8`). `F` is the floor of the placement region the assignment chose, the holes being those `Joint::map`
/// reports in it; with every unit owed, as every one is while no catalog record states a frame, a guard or an alignment,
/// the limit is owed and `truncated-by` names each hole at the lowest base above `F`, by provider, then name.
fn memory_of(
    rebuilt: &assignment::Rebuilt<'_>,
    names: &Names<'_>,
    placement: Option<&(String, String)>,
    steps: usize,
    routes: Vec<String>,
    timer: bool,
) -> Result<Memory, Refusal> {
    let unwritable = |why: String| refuse("tool-failure", why);
    let regions = rebuilt.joint.regions(&rebuilt.system);
    let region_of = |provider: &str, name: &str| {
        regions
            .iter()
            .find(|(p, r)| names.text(p) == provider && r.name == name)
            .map(|(_, r)| *r)
    };
    let mut obligations = Vec::new();
    let mut image = None;
    for obligation in &rebuilt.map.obligations {
        let Obligation::Placement {
            provider,
            region,
            holes,
            ..
        } = obligation
        else {
            continue;
        };
        let provider_text = names.text(provider);
        obligations.push(layout::placement_obligation(&provider_text, region));
        if placement.is_some_and(|(p, r)| *p == provider_text && r == region) {
            let at = region_of(&provider_text, region).ok_or_else(|| {
                unwritable(format!("the placement region `{region}` of `{provider_text}` is no region of the target's hardware (§5)"))
            })?;
            let span_of = |base: i128, bytes: Option<i128>| -> Result<layout::Reserved, Refusal> {
                Ok(layout::Reserved {
                    base: u128::try_from(base).map_err(|_| {
                        unwritable(format!("a base below address nothing, {base} (§5)"))
                    })?,
                    bytes: bytes
                        .map(|bytes| {
                            u128::try_from(bytes).map_err(|_| {
                                unwritable(format!("a size below nothing, {bytes} (§5)"))
                            })
                        })
                        .transpose()?,
                })
            };
            let place = span_of(at.base, at.bytes)?;
            let bytes = place.bytes.ok_or_else(|| {
                unwritable(format!("the placement region `{region}` states no size, which `Joint::map` refuses before any plan (§5)"))
            })?;
            let span = layout::Span::of(place.base, bytes);
            let mut named = Vec::new();
            let mut reserved = Vec::new();
            for (hole_provider, hole_region) in holes {
                let hole_text = names.text(hole_provider);
                let hole = region_of(&hole_text, hole_region).ok_or_else(|| {
                    unwritable(format!("the hole `{hole_region}` of `{hole_text}` is no region of the target's hardware (§5)"))
                })?;
                reserved.push(span_of(hole.base, hole.bytes)?);
                named.push((hole_text, hole_region.clone()));
            }
            let laid = layout::holes(span, &reserved);
            let floor = layout::floor(span, &laid);
            let limit = layout::limit(span, &laid, floor, &[], true);
            let mut truncated: Vec<String> = limit
                .truncated_by
                .iter()
                .map(|hole| {
                    let (p, r) = &named[hole.of];
                    format!("({p} {r})")
                })
                .collect();
            truncated.sort();
            truncated.dedup();
            let written_floor = u64::try_from(floor)
                .map_err(|_| unwritable(format!("a floor past 2^64 − 1, {floor} (§3, `INT`)")))?;
            image = Some(format!(
                "(image (floor {written_floor}) (limit owed) {})",
                list("truncated-by", &truncated)
            ));
        }
    }
    obligations.sort();
    obligations.dedup();
    let image = image.ok_or_else(|| {
        unwritable("a plan on a target names no placement memory `Joint::map` reports, which `Target::admission` refuses before any plan (§5)".to_string())
    })?;
    Ok(Memory {
        image,
        obligations,
        steps,
        routes,
        timer,
        allocations: Vec::new(),
        owed: Vec::new(),
    })
}

/// The catalog's figures of each stack, which no record states yet, so each is owed of its unit (§3, "`(owed …)`, as
/// `/2` writes it"; §5's table of sums and its table of figures): `guard` and `alignment` of every stack, the fault path's
/// two figures and a trap frame of every stack, and of each its sum's own.
const OF_EVERY_STACK: [&str; 5] = [
    "alignment",
    "fault-check-stack",
    "fault-path-stack",
    "guard",
    "trap-frame",
];

/// A plan on a target's allocations and what its memory and its controller owe (§5, "The formula", "An owed unit makes
/// every later unit owed" and "With no application's file the boot's stack is owed"; §3, `UNIT`, `NEED`, "Orders" and
/// "`(owed …)`, as `/2` writes it"; §6; leaf `M4.1.7.6.8`). The units in §5's order: the boot's stack, the interrupt
/// stack, each task's by runtime index, then each table by name, which none is while the catalog states none. Each lacks
/// the figures of its sum not yet stated: the application's where no file is given, the catalog's, which no record states
/// yet, and the realization's, which no file states yet; a node's or a primitive's only once a file names its calls, none
/// being known until then. So every unit is owed, and each later one names the last before it.
fn laid_out(
    body: &Body,
    memory: &Memory,
    application: Option<&application::Application>,
) -> Memory {
    use layout::{Need, Owner};
    let record = |fact: &str| Need {
        owner: Owner::Record,
        fact: fact.to_owned(),
    };
    let realization = |subject: &str, fact: &str| Need {
        owner: Owner::Realization {
            subject: subject.to_owned(),
        },
        fact: fact.to_owned(),
    };
    let of_application = |of: &str, fact: &str| Need {
        owner: Owner::Application { of: of.to_owned() },
        fact: fact.to_owned(),
    };
    let fault = realization("(fault)", "fault-entry-stack");
    let mut units: Vec<(String, Vec<Need>)> = Vec::new();
    // The boot's stack: `call-frame + max(initialisation-stack, initialisation-path-stack, stack-bound + node-stack,
    // last-act-stack + max(transition-stack, idle-stack)) + fault-use + trap-frame`.
    let mut boot: Vec<Need> = OF_EVERY_STACK
        .iter()
        .chain(&[
            "idle-stack",
            "initialisation-path-stack",
            "transition-stack",
        ])
        .map(|fact| record(fact))
        .collect();
    boot.push(realization("(stack boot)", "call-frame"));
    boot.push(realization("(stack boot)", "last-act-stack"));
    boot.push(fault.clone());
    boot.extend(
        (1..=memory.steps)
            .map(|step| realization(&format!("(step {step})"), "initialisation-stack")),
    );
    match application {
        None => boot.push(of_application(&body.system, "initialisation")),
        Some(file) => boot.extend(
            file.initialisation
                .iter()
                .flat_map(|initialisation| initialisation.node_calls.calls.iter())
                .map(|(node, _)| realization(node, "node-stack")),
        ),
    }
    units.push(("(stack boot)".to_string(), boot));
    // The interrupt stack: `trap-path-stack + call-frame + max(service-stack, timer-service-stack) + fault-use +
    // trap-frame`, `timer-service-stack` nothing and owed by no one under `(timer none)` (RP12-4).
    let mut interrupt: Vec<Need> = OF_EVERY_STACK
        .iter()
        .chain(&["trap-path-stack"])
        .map(|fact| record(fact))
        .collect();
    if memory.timer {
        interrupt.push(record("timer-service-stack"));
    }
    interrupt.push(realization("(stack interrupt)", "call-frame"));
    interrupt.push(fault.clone());
    interrupt.extend(
        memory
            .routes
            .iter()
            .map(|item| realization(&format!("(route {item})"), "service-stack")),
    );
    units.push(("(stack interrupt)".to_string(), interrupt));
    // Each task's stack, by runtime index: `call-frame + stack-bound + max(node-stack, primitive-stack, completion-stack) +
    // fault-use + trap-frame`.
    for (name, text) in &body.task_order {
        let unit = format!("(stack {text})");
        let mut needs: Vec<Need> = OF_EVERY_STACK
            .iter()
            .chain(&["completion-stack"])
            .map(|fact| record(fact))
            .collect();
        needs.push(realization(&unit, "call-frame"));
        needs.push(fault.clone());
        match application.and_then(|file| file.tasks.iter().find(|task| task.name == *name)) {
            None => {
                for fact in ["node-calls", "os-calls", "stack-bound"] {
                    needs.push(of_application(text, fact));
                }
            }
            Some(task) => {
                if !task.os_calls.calls.is_empty() {
                    needs.push(record("primitive-stack"));
                }
                needs.extend(
                    task.node_calls
                        .calls
                        .iter()
                        .map(|(node, _)| realization(node, "node-stack")),
                );
            }
        }
        units.push((unit, needs));
    }
    let owed_needs = layout::owed_needs(&units);
    let allocations = owed_needs
        .iter()
        .map(|(unit, needs)| format!("(allocation {unit} owed (needs {}))", needs.join(" ")))
        .collect();
    // What is owed of them: each unit's catalog figures of that unit, and each figure of the generated code of its own
    // subject, by the realization (§3); the application's are owed already, `/1`'s and `/2`'s, where no file is given.
    let mut owed = Vec::new();
    for (unit, needs) in &units {
        for need in needs {
            match &need.owner {
                Owner::Record => owed.push(format!("(owed {unit} {} (by catalog))", need.fact)),
                Owner::Realization { subject } => {
                    owed.push(format!("(owed {subject} {} (by realization))", need.fact));
                }
                Owner::Application { .. } => {}
            }
        }
    }
    // §6: every route reaches the controller while its `external` is owed, so each owes its line and the controller its
    // levels, by the catalog, which states neither yet (RP29-2, RP32-4).
    for item in &memory.routes {
        owed.push(format!("(owed {item} controller-line (by catalog))"));
    }
    if !memory.routes.is_empty() {
        owed.push("(owed (controller) controller-levels (by catalog))".to_string());
    }
    owed.sort();
    owed.dedup();
    Memory {
        allocations,
        owed,
        ..memory.clone()
    }
}

/// The facts a source owes as a source, which `/2` owes of each alarm too, each being a source of its own
/// (`decision_runtime-plan-format-2.md` §3; `decision_release-alarm.md` §2's last row); its `service-cost` is
/// [`alarm::owes`]'s.
const AS_A_SOURCE: [&str; 4] = [
    "acknowledge-point",
    "deferred-work",
    "no-application-code",
    "one-request-per-arrival",
];

/// A device's counter as the description states it: its rate, in hertz, its quantity, and its modulus, each where it
/// has a value (`decision_runtime-plan-format-2.md` §3, `SIMULATED`).
fn counter_of(
    device: &str,
    declarations: &[Form],
    vocabulary: &Vocabulary,
) -> (Option<Quantity>, Option<i128>) {
    let provider = declarations
        .iter()
        .find(|d| d.items().get(1).and_then(Form::as_symbol) == Some(device))
        .and_then(|d| offer::read(d, vocabulary).ok());
    let valued = |fact: &str| match provider.as_ref().and_then(|p| p.state(fact)) {
        Some(State::Valued(values)) => values.first().cloned(),
        _ => None,
    };
    let rate = match valued("tick-rate") {
        Some(Value::Quantity(q)) => Some(q),
        _ => None,
    };
    let modulus = match valued("counter-modulus") {
        Some(Value::Count(m)) => Some(m),
        _ => None,
    };
    (rate, modulus)
}

/// What `device`, as `declarations` hold it, states of `fact`, where it states a value.
fn stated(
    device: &str,
    declarations: &[Form],
    vocabulary: &Vocabulary,
    fact: &str,
) -> Option<Value> {
    declarations
        .iter()
        .find(|d| d.items().get(1).and_then(Form::as_symbol) == Some(device))
        .and_then(|d| offer::read(d, vocabulary).ok())
        .and_then(|provider| match provider.state(fact) {
            Some(State::Valued(values)) => values.first().cloned(),
            _ => None,
        })
}

/// A rate in hertz as §3's `R` writes one, exact, a whole number or `N/D` in lowest terms; nothing where it is none the
/// format writes.
fn rate_text(rate: Quantity) -> Option<String> {
    let hertz = rate.in_base().ok()?;
    let numerator = u128::try_from(hertz.numerator()).ok()?;
    let denominator = u128::try_from(hertz.denominator())
        .ok()
        .and_then(std::num::NonZeroU128::new)?;
    Some(alarm::rate(numerator, denominator))
}

/// A target device's counter as §3's `COUNTER` writes it, `(tick-rate R) (modulus M) (reach DUR) (step S) (window W)`,
/// and its `KIND`, its `alarm-kind` (`decision_runtime-plan-format-2.md` §3, `COUNTER` and `KIND`; leaf `M4.1.7.6.7.2`):
/// each as the device states it, unconverted, `W` `magnitude` where it states `(compare-magnitude true)` and no window
/// written on a delay; `reach` its `supported-horizon`. The search asks each of a device it binds for a compare
/// (`decision_release-alarm.md` §1 and §2), so a device stating none of one, which no checked plan holds, is a toolchain
/// failure here.
fn counter_line(
    device: &str,
    declarations: &[Form],
    vocabulary: &Vocabulary,
) -> Result<(String, String), Refusal> {
    let unstated = |fact: &str| {
        refuse(
            "tool-failure",
            format!(
                "the target's device `{device}` states no `{fact}`, which the search asks of a device it binds (§3)"
            ),
        )
    };
    let unwritable = |fact: &str| {
        refuse(
            "tool-failure",
            format!("the target's device `{device}` states a `{fact}` that is no value `/2` writes (§3)"),
        )
    };
    let value = |fact: &str| stated(device, declarations, vocabulary, fact);
    let count = |fact: &str| -> Result<Option<String>, Refusal> {
        match value(fact) {
            None => Ok(None),
            Some(Value::Count(c)) => u128::try_from(c)
                .map(|c| Some(alarm::count(c)))
                .map_err(|_| unwritable(fact)),
            Some(_) => Err(unwritable(fact)),
        }
    };
    let rate = match value("tick-rate") {
        Some(Value::Quantity(q)) => rate_text(q).ok_or_else(|| unwritable("tick-rate"))?,
        Some(_) => return Err(unwritable("tick-rate")),
        None => return Err(unstated("tick-rate")),
    };
    let modulus = count("counter-modulus")?.ok_or_else(|| unstated("counter-modulus"))?;
    let reach = match value("supported-horizon") {
        Some(Value::Quantity(q)) => {
            duration(q.in_base().map_err(|_| unwritable("supported-horizon"))?)
        }
        Some(_) => return Err(unwritable("supported-horizon")),
        None => return Err(unstated("supported-horizon")),
    };
    let step = count("compare-step")?.ok_or_else(|| unstated("compare-step"))?;
    let kind = match value("alarm-kind") {
        Some(Value::Enum(kind)) => kind,
        Some(_) => return Err(unwritable("alarm-kind")),
        None => return Err(unstated("alarm-kind")),
    };
    // §3: an alarm on a delay writes no `(window …)`; a compare is read by its window, or as whole numbers (§2).
    let window = if kind == "delay" {
        String::new()
    } else {
        let window = match value("compare-window") {
            None => None,
            Some(Value::Count(c)) => {
                Some(u128::try_from(c).map_err(|_| unwritable("compare-window"))?)
            }
            Some(_) => return Err(unwritable("compare-window")),
        };
        let magnitude = match value("compare-magnitude") {
            None => None,
            Some(Value::Bool(said)) => Some(said),
            Some(_) => return Err(unwritable("compare-magnitude")),
        };
        match alarm::compare_reading(window, magnitude) {
            alarm::Reading::Window(window) => format!(" (window {})", alarm::count(window)),
            alarm::Reading::WholeNumbers => " (window magnitude)".to_string(),
            alarm::Reading::Unstated => return Err(unstated("compare-window")),
            alarm::Reading::Refused => {
                return Err(refuse(
                    "tool-failure",
                    format!(
                        "the target's device `{device}` states `(compare-magnitude true)` beside a window, which `check` refuses of the file (§2)"
                    ),
                ))
            }
        }
    };
    Ok((
        format!("(tick-rate {rate}) (modulus {modulus}) (reach {reach}) (step {step}){window}"),
        kind,
    ))
}

/// A plan on a target's release timer (`decision_release-alarm.md` §1; `decision_runtime-plan-format-2.md` §3 and §4;
/// leaf `M4.1.7.6.7.2`): the device the system's release requester is bound to, by the joint rules' name, or none where
/// no task is released by the timer, `(timer none)`.
struct Release<'a> {
    /// The release timer's device.
    device: Option<&'a str>,
}

/// A plan on a target's time (`decision_runtime-plan-format-2.md` §3's `(timer …)` and `(epoch …)`, §4; leaf
/// `M4.1.7.6.7.2`): its timer's line, its epoch's, and what they owe.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Time {
    /// `(timer (device NODE) COUNTER (horizon DUR) (rounding FIGURE) (first-compare transition) (releases …))` or
    /// `(timer none)`.
    timer: String,
    /// `(epoch after-read (lead FIGURE))`, `(epoch none (lead FIGURE))` or `(epoch none)`.
    epoch: String,
    /// `compare-rounding` of `(timer)` and `epoch-lead` of `(epoch)` by the catalog, where its line writes the figure
    /// owed, and `last-act-lead` of `(epoch)` by the realization with any alarm.
    owed: Vec<String>,
}

/// `(simulated (tick-rate R|unstated) (modulus M|unstated))` (§3, `SIMULATED`): a rate exact, in lowest terms, and a
/// count as the language holds one.
fn simulated(rate: Option<Quantity>, modulus: Option<i128>) -> Result<String, Refusal> {
    let unwritable = |what: &str| {
        refuse(
            "tool-failure",
            format!("the device's {what} is no value `/2` writes (§3)"),
        )
    };
    let rate = match rate {
        None => "unstated".to_string(),
        Some(q) => rate_text(q).ok_or_else(|| unwritable("tick-rate"))?,
    };
    let modulus = match modulus {
        None => "unstated".to_string(),
        Some(m) => alarm::count(u128::try_from(m).map_err(|_| unwritable("counter-modulus"))?),
    };
    Ok(format!(
        "(simulated (tick-rate {rate}) (modulus {modulus}))"
    ))
}

/// `/2`'s alarms and what they change (`decision_runtime-plan-format-2.md` §3, "An alarm's identity is its node", the
/// paragraph after it, `SIMULATED`, `KIND`, `BASE`, the horizon, the delivery bound and the asked reach, `(clock …)` and
/// what is owed; `decision_release-alarm.md` §2 and §3; leaf `M4.1.7.6.5`). Each requester bound directly to a timer
/// route's device for a compare, `relative-delay` or `absolute-deadline`, is an alarm, its node; a wake where it is an
/// instance whose kind's cost is `wake`, and otherwise an expiry or an alarm bound directly. With no alarm, every group
/// is `/1`'s. Also what each alarm and clock owes, which `/2` adds.
#[allow(clippy::too_many_arguments, clippy::too_many_lines)]
fn alarms_of(
    routes: &[Route],
    contexts: &BTreeMap<String, BTreeSet<String>>,
    nodes: &BTreeSet<String>,
    obligation_lines: &[String],
    names: &Names<'_>,
    rebuilt: &assignment::Rebuilt<'_>,
    declarations: &[Form],
    vocabulary: &Vocabulary,
    catalog: &Catalog,
    tasks: &[String],
    target: Option<&Release<'_>>,
) -> Result<(Two, Vec<String>), Refusal> {
    const COMPARES: [&str; 2] = ["relative-delay", "absolute-deadline"];
    let instance = |name: &str| rebuilt.instances.iter().find(|i| i.provider.name == name);
    let reach = |from: &str| rebuilt.joint.reaches(from, &rebuilt.bindings);
    let seconds = |q: &Requirement| match q {
        Requirement::Constraint { value, .. } => match value_seconds(value) {
            Seconds::Of(s) => Some(s),
            _ => None,
        },
        _ => None,
    };
    // What an alarm's requirement asks by `fact`: its node's own at its device, or, an instance's, what is bound to it.
    let asked = |node: &str, device: &str, fact: &str| -> Vec<Rational> {
        let mut values = Vec::new();
        for b in rebuilt
            .bindings
            .iter()
            .filter(|b| (b.requester == node && b.provider == device) || b.provider == node)
        {
            let parts: Vec<&Requirement> = match &b.requirement {
                Requirement::Group { parts, .. } => parts.iter().collect(),
                other => vec![other],
            };
            values.extend(
                parts
                    .into_iter()
                    .filter(|q| q.fact() == fact)
                    .filter_map(&seconds),
            );
        }
        values
    };
    let mut owed = Vec::new();
    let owe = |owed: &mut Vec<String>, subject: &str, fact: &str, owner: &str| {
        owed.push(format!("(owed {subject} {fact} (by {owner}))"));
    };
    let mut alarms: Vec<AlarmOf> = Vec::new();
    for route in routes.iter().filter(|r| r.timer) {
        let device = route.provider.as_str();
        let mut takers: Vec<&str> = rebuilt
            .bindings
            .iter()
            .filter(|b| {
                b.provider == device
                    && COMPARES.contains(&b.requirement.fact())
                    // The release requester is no node and takes no alarm: its channel is the release timer's (§3).
                    && joint::release_of(&b.requester).is_none()
            })
            .map(|b| b.requester.as_str())
            .collect();
        takers.sort_unstable();
        takers.dedup();
        if takers.is_empty() {
            return Err(refuse(
                "tool-failure",
                format!("the timer route of `{device}` has no alarm, no requester binding a compare there (§3)"),
            ));
        }
        let (rate, modulus) = counter_of(device, declarations, vocabulary);
        for node in takers {
            let of = instance(node);
            let wake = of.is_some_and(|i| {
                catalog
                    .kind(&i.kind)
                    .is_some_and(|k| k.costs.iter().any(|c| c.name == "wake"))
            });
            let bound_to = |facts: &[&str]| -> Vec<String> {
                let mut to: Vec<String> = rebuilt
                    .bindings
                    .iter()
                    .filter(|b| {
                        b.provider == node
                            && (facts.is_empty() || facts.contains(&b.requirement.fact()))
                    })
                    .map(|b| b.requester.clone())
                    .collect();
                to.sort();
                to.dedup();
                to
            };
            let delivers_to = if of.is_some_and(|i| i.delivers) {
                bound_to(&[])
            } else {
                Vec::new()
            };
            let reach_of = alarm::reach(device, node, &delivers_to, reach);
            // `BASE`: a wake's is the counter; any other's is read of the readers of time its deadline's requester reaches,
            // itself not among them (rule 14), each by where it is bound.
            let base = if wake {
                alarm::Base::Counter
            } else {
                let deadline_requesters = if of.is_some() {
                    bound_to(&COMPARES)
                } else {
                    vec![node.to_string()]
                };
                let readers: Vec<alarm::Reader> = deadline_requesters
                    .iter()
                    .flat_map(|from| rebuilt.joint.readers_of_time(from, &rebuilt.bindings))
                    .map(|(_, provider)| match instance(&provider) {
                        Some(i) if i.kind == "epoch-extender" && i.over == device => {
                            alarm::Reader::ExtendedOverIt
                        }
                        Some(i) if i.kind == "epoch-extender" => alarm::Reader::ExtendedOverAnother,
                        Some(i) if i.over == device => alarm::Reader::AtTheDevice,
                        Some(_) => alarm::Reader::OnAnother,
                        None if provider == device => alarm::Reader::AtTheDevice,
                        None => alarm::Reader::OnAnother,
                    })
                    .collect();
                alarm::base(false, &readers)
            };
            let delivery_bound = if wake {
                None
            } else {
                asked(node, device, "delivery-bound").into_iter().min()
            };
            let asked_reach = if wake {
                None
            } else {
                asked(node, device, "supported-horizon").into_iter().max()
            };
            // A wake's horizon is its device's `unambiguous-horizon`, derived from its modulus and rate (§3).
            let horizon = match (wake, modulus, rate) {
                (true, Some(m), Some(q)) => eadl_resolve::value::horizon(m, q)
                    .ok()
                    .and_then(|h| h.in_base().ok()),
                _ => None,
            };
            let text = names.text(node);
            let at_most =
                |value: Option<Rational>| value.map_or_else(|| "none".to_string(), duration);
            // On a target, its device's counter as `COUNTER` writes it and its kind as the device states it; on the playground,
            // `SIMULATED` and the simulator's level compare (§3, `COUNTER`, `SIMULATED` and `KIND`).
            let (counted, kind) = match target {
                Some(_) => counter_line(device, declarations, vocabulary)?,
                None => (simulated(rate, modulus)?, "level-compare".to_string()),
            };
            let line = format!(
                "(alarm {text} (device {}) {counted} (kind {kind}) (time-base {}) (horizon {}) (delivery-bound {}) (asked-reach {}))",
                names.text(device),
                base.as_str(),
                at_most(horizon),
                at_most(delivery_bound),
                at_most(asked_reach),
            );
            alarms.push(AlarmOf {
                node: node.to_string(),
                text,
                device: device.to_string(),
                wake,
                reach: reach_of,
                rank: if wake { horizon } else { delivery_bound },
                stated: (rate.is_some(), modulus.is_some()),
                delivery: delivery_bound.is_some(),
                base,
                line,
            });
        }
    }
    alarms.sort_by(|a, b| a.text.cmp(&b.text));

    // The routes, by the release alarm's order (§3): a source by its separation, an expiry or an alarm bound directly by
    // its delivery bound, a wake by its horizon, what states no value last, ties by the item's text. A source's line is
    // `/1`'s but for its rank; an alarm's carries the requesters of its device's route that reach its node, or are it, and
    // the costs `/1` writes of its node.
    struct Ranked {
        item: String,
        separation: String,
        requesters: Vec<String>,
        costs: Vec<String>,
    }
    let mut ranked: Vec<(Option<Rational>, (String, usize))> = Vec::new();
    let mut described: Vec<Ranked> = Vec::new();
    for route in routes.iter().filter(|r| !r.timer) {
        let mut requesters: Vec<String> = route.requesters.iter().map(|r| names.text(r)).collect();
        requesters.sort();
        ranked.push((route.separation.ok(), (route.item(), described.len())));
        described.push(Ranked {
            item: route.item(),
            separation: match &route.separation {
                Ok(s) => duration(*s),
                Err(word) => (*word).to_string(),
            },
            requesters,
            costs: Vec::new(),
        });
    }
    for alarm in &alarms {
        let route = routes
            .iter()
            .find(|r| r.timer && r.provider == alarm.device)
            .expect("an alarm is of a timer route");
        let mut requesters: Vec<String> = route
            .requesters
            .iter()
            .filter(|r| joint::release_of(r).is_none())
            .filter(|r| **r == alarm.node || reach(r).contains(&alarm.node))
            .map(|r| names.text(r))
            .collect();
        requesters.sort();
        requesters.dedup();
        let mut costs: Vec<String> = instance(&alarm.node)
            .and_then(|i| catalog.kind(&i.kind))
            .map(|kind| {
                kind.costs
                    .iter()
                    .map(|c| format!("({} {})", alarm.text, c.name))
                    .collect()
            })
            .unwrap_or_default();
        costs.sort();
        costs.dedup();
        let item = format!("(alarm {})", alarm.text);
        ranked.push((alarm.rank, (item.clone(), described.len())));
        described.push(Ranked {
            item,
            separation: "programmed".to_string(),
            requesters,
            costs,
        });
    }
    let order = controller::ranked(ranked);
    let mut two = Two::default();
    for (at, (_, index)) in order.iter().enumerate() {
        let route = &described[*index];
        two.routes.push(format!(
            "(route {} {} (separation {}) (level {}) (deliverable yes) {} {})",
            at + 1,
            route.item,
            route.separation,
            RT_STATIC_UP_V1.caller_level,
            list("requesters", &route.requesters),
            list("costs", &route.costs)
        ));
    }
    // `timer` first, but under `(timer none)` on a target, whose release timer's interrupt is never enabled (§4; RP11-1).
    let timer_enabled = target.is_none_or(|release| release.device.is_some());
    two.enabled = timer_enabled
        .then(|| "timer".to_string())
        .into_iter()
        .chain(
            order
                .iter()
                .map(|(_, index)| described[*index].item.clone()),
        )
        .collect();

    // The contexts: `/1`'s, a timer route's interrupt replaced by each alarm whose reach holds the subject (§3).
    let named = alarm::contexts(
        &alarms
            .iter()
            .map(|a| (a.text.clone(), a.reach.clone()))
            .collect::<Vec<_>>(),
    );
    let mut subjects: BTreeMap<String, BTreeSet<String>> = contexts
        .iter()
        .map(|(subject, words)| {
            (
                subject.clone(),
                words
                    .iter()
                    .filter(|w| !w.starts_with("(interrupt (timer-route "))
                    .cloned()
                    .collect(),
            )
        })
        .collect();
    for (subject, by) in &named {
        if contexts.contains_key(subject) || nodes.contains(subject) {
            let words = subjects.entry(subject.clone()).or_default();
            words.extend(by.iter().map(|text| format!("(interrupt (alarm {text}))")));
        }
    }
    // On a target the release timer's device is a node whose context holds `(interrupt timer)`, the catalog's timer service
    // running there (§3, the paragraph on `(release SYSTEM)`; model §8 rule 12).
    if let Some(device) = target.and_then(|release| release.device) {
        subjects
            .entry(device.to_string())
            .or_default()
            .insert("(interrupt timer)".to_string());
    }
    two.contexts = subjects
        .iter()
        .filter(|(_, words)| !words.is_empty())
        .map(|(subject, set)| {
            let mut ordered: Vec<&String> = Vec::new();
            ordered.extend(set.get("job"));
            ordered.extend(set.iter().filter(|c| c.starts_with("(interrupt ")));
            ordered.extend(set.get("initialisation"));
            let words: Vec<String> = ordered.into_iter().cloned().collect();
            format!("(context {} {})", names.text(subject), words.join(" "))
        })
        .collect();
    two.contexts.sort();

    // The obligations: one route obligation for each alarm, where `/1` writes one for its device (`RA-H1`).
    two.obligations = obligation_lines
        .iter()
        .filter(|line| !line.starts_with("(obligation route (timer-route "))
        .cloned()
        .chain(alarms.iter().map(|a| {
            format!(
                "(obligation route (alarm {}) (discharged-by M4.2 M4.3))",
                a.text
            )
        }))
        .collect();
    two.obligations.sort();
    two.obligations.dedup();

    // What each alarm owes: as the checker asks of it, as a source, and of the description what its line leaves
    // unstated or an expiry asks no bound of (§3).
    for alarm in &alarms {
        let subject = format!("(alarm {})", alarm.text);
        for fact in alarm::owes(
            alarm.wake,
            alarm.base == alarm::Base::Counter,
            target.is_none(),
        )
        .into_iter()
        .chain(AS_A_SOURCE)
        {
            owe(&mut owed, &subject, fact, "realization");
        }
        // A `SIMULATED` line's `unstated`; a target's device states each, the search having asked it (§3).
        if target.is_none() && !alarm.stated.0 {
            owe(&mut owed, &subject, "tick-rate", "description");
        }
        if target.is_none() && !alarm.stated.1 {
            owe(&mut owed, &subject, "counter-modulus", "description");
        }
        if !alarm.wake && !alarm.delivery {
            owe(&mut owed, &subject, "delivery-bound", "description");
        }
    }
    two.alarms = alarms.iter().map(|a| a.line.clone()).collect();
    two.items = alarms
        .iter()
        .map(|a| format!("(alarm {})", a.text))
        .collect();
    // `RA-H7`: each task that writes an alarm on the counter itself that is no wake, where it reaches the alarm's node.
    for alarm in alarms
        .iter()
        .filter(|a| alarm::asks_application(a.wake, a.base == alarm::Base::Counter))
    {
        for task in tasks {
            if reach(task).contains(&alarm.node) {
                two.asked.push((names.text(task), alarm.text.clone()));
            }
        }
    }
    two.asked.sort();
    two.asked.dedup();

    // Each counter a hosted time base reads that no alarm's line names: a `(clock …)` line of its own (§3), a hosted plan's
    // alone, a target's time bases reading the target's own devices.
    let named_devices: BTreeSet<&str> = alarms.iter().map(|a| a.device.as_str()).collect();
    let mut clocks: BTreeSet<&str> = BTreeSet::new();
    for b in rebuilt.bindings.iter().filter(|_| target.is_none()) {
        if rebuilt.joint.reads_a_counter(b.requirement.fact())
            && instance(&b.provider).is_none()
            && !named_devices.contains(b.provider.as_str())
        {
            clocks.insert(b.provider.as_str());
        }
    }
    let mut clock_lines: Vec<(String, String)> = Vec::new();
    for device in clocks {
        let (rate, modulus) = counter_of(device, declarations, vocabulary);
        let text = names.text(device);
        clock_lines.push((
            text.clone(),
            format!("(clock {text} {})", simulated(rate, modulus)?),
        ));
        let subject = format!("(clock {text})");
        if rate.is_none() {
            owe(&mut owed, &subject, "tick-rate", "description");
        }
        if modulus.is_none() {
            owe(&mut owed, &subject, "counter-modulus", "description");
        }
    }
    clock_lines.sort();
    two.clocks = clock_lines.into_iter().map(|(_, line)| line).collect();
    Ok((two, owed))
}

/// The assignment record's sha256, its identity in the plan (§2.1).
fn sha256(text: &str) -> String {
    Digest::of(text.as_bytes()).hex()
}

/// §2.7's steps: the strongly connected components in a topological order, repeatedly taking, among the components
/// whose predecessors have all been taken, the one whose least member's text is least; a step's members by their text.
fn steps(
    nodes: &[(String, Node)],
    edges: &[(String, String, Because)],
    components: &[Vec<String>],
    names: &Names<'_>,
) -> Vec<Vec<String>> {
    let mut component_of: BTreeMap<String, usize> = BTreeMap::new();
    let mut members: Vec<Vec<String>> = Vec::new();
    for component in components {
        for n in component {
            component_of.insert(n.clone(), members.len());
        }
        members.push(component.iter().map(|n| names.text(n)).collect());
    }
    for (n, _) in nodes {
        if !component_of.contains_key(n) {
            component_of.insert(n.clone(), members.len());
            members.push(vec![names.text(n)]);
        }
    }
    for m in &mut members {
        m.sort();
    }
    let mut before: Vec<BTreeSet<usize>> = vec![BTreeSet::new(); members.len()];
    for (from, to, _) in edges {
        if let (Some(&a), Some(&b)) = (component_of.get(from), component_of.get(to)) {
            if a != b {
                before[b].insert(a);
            }
        }
    }
    let mut taken: BTreeSet<usize> = BTreeSet::new();
    let mut out = Vec::new();
    while taken.len() < members.len() {
        let next = (0..members.len())
            .filter(|c| !taken.contains(c) && before[*c].iter().all(|p| taken.contains(p)))
            .min_by(|a, b| members[*a][0].cmp(&members[*b][0]));
        let Some(next) = next else {
            break;
        };
        taken.insert(next);
        out.push(members[next].clone());
    }
    out
}
