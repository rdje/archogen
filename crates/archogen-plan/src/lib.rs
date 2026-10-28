//! The runtime plan (`ROADMAP.md` §4.2 `archogen-plan`, §10.1 step 5; leaf `M4.1.4`): a checked assignment lowered into
//! `archogen-runtime-plan/1`, the plan for a system run on the hosted playground, as
//! `docs/decisions/decision_runtime-plan.md` writes it.
//!
//! ⭐ **Built ahead of the record's acceptance, as its executable form** (`docs/decisions/decision_executable-design-reviews.md`):
//! a direct transcription, rule by rule, each rule citing its section of the record, optimised for nothing. A review
//! round reads the record beside this code and runs the corpus `tests/corpus/`, in which every probe system of the
//! record's review is a fixture with the text or the refusal the record gives it, and the record's own worked texts
//! (§3) are held equal to what this code writes (`tests/worked.rs`). A question a machine can settle — whether a rule
//! applies, what text it writes — is then settled by one.
//!
//! It lowers the record the configuration checker accepted, read back by the generator's own reader
//! (`eadl_resolve::plan::read`, `rebuild`; leaf `M4.1.3`), never the search's memory. It does no I/O.
//!
//! Not built here yet, `M4.1.4`'s production half: the result record's intake and its refusals (§1's checks of
//! `archogen-result/1`), the provenance, the plan's own reader and its round trip, and the goldens with their fixed
//! result records.

use std::collections::{BTreeMap, BTreeSet};

use archogen_digest::Digest;

use eadl_front::{Form, Program};
use eadl_model::check::resolved_declarations;
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_model::quantity::{Dimension, Quantity};
use eadl_model::rational::Rational;
use eadl_resolve::adapter::{self, Catalog, Instance, Offered};
use eadl_resolve::joint::{self, Because, Binding, Interrupt, Kind, Node, Obligation};
use eadl_resolve::offer::State;
use eadl_resolve::plan as assignment;
use eadl_resolve::requirement::Requirement;
use eadl_resolve::value::Value;
use eadl_resolve::vocabulary::{Usage, Vocabulary};

/// The format's identifier (§3).
pub const FORMAT: &str = "archogen-runtime-plan/1";

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

/// A runtime plan, as its canonical text's lines (§3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    lines: Vec<String>,
}

impl Plan {
    /// The plan's canonical text: each line one form, ending with a newline, the last included (§3).
    #[must_use]
    pub fn text(&self) -> String {
        let mut out = String::new();
        for line in &self.lines {
            out.push_str(line);
            out.push('\n');
        }
        out
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
    /// Its interrupt's text (§2): `(timer-route NAME)` or `(source NAME)`.
    fn item(&self) -> String {
        if self.timer {
            format!("(timer-route {})", self.provider)
        } else {
            format!("(source {})", self.provider)
        }
    }
}

/// Whether a cost's interrupt runs its requester's code (§2.6): `expiry`, a delivery to its requester, does; `wake`, an
/// epoch advance, runs the instance's alone. The adapters record states neither, so `M4.1.2` has it state them.
fn runs_requester_code(cost: &str) -> bool {
    cost == "expiry"
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

/// `record`, an `archogen-assignment/2` record the configuration checker accepted, lowered into
/// `archogen-runtime-plan/1` over `program`, the elaborated program the language's check accepted, whose declarations
/// and digest the lowering takes from it (§1, §2, §3).
///
/// # Errors
///
/// §1's refusals, in its order: a record that does not read; that names another program, system, catalog or profile;
/// that does not rebuild, or that the generator's own joint rules refuse; a task's release or deadline the language's
/// check should have refused — none stated, not a time, a zero release separation, a periodic task bound to an interrupt
/// source itself; and what `/1` cannot lower — placement memory, a system with no task, a deadline measured from an
/// event other than the release, an alarm or an expiry a task or the system's own level would receive. Then §2.4's: an
/// obliged value that is not a duration.
#[allow(clippy::too_many_lines)]
pub fn lower(
    program: &Program,
    record: &str,
    vocabulary: &Vocabulary,
    catalog: &Catalog,
) -> Result<Plan, Refusal> {
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
    if record.profile != RT_STATIC_UP_V1.id {
        return Err(refuse(
            "unsupported-profile",
            format!(
                "`/1` lowers `{}`'s rules alone, and the record was resolved under `{}`",
                RT_STATIC_UP_V1.id, record.profile
            ),
        ));
    }
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
    // §1: a task's release or deadline the description check should have refused — none stated, not a time, a release
    // separation of zero, or a periodic task bound to an interrupt source itself, a second release model.
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
        let mut time = |c: &Form| match seconds(c) {
            Seconds::Of(s) => Ok(Some(s)),
            Seconds::NotTime => Err(ill_formed(
                task,
                format!("writes `{}`, which is not a time", c.to_canonical()),
                "M1.42",
            )),
            Seconds::PastArithmetic => {
                past.push(format!(
                    "`(task {system} {})` writes `{}`",
                    task.name,
                    c.to_canonical()
                ));
                Ok(None)
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
        if release_seconds == Some(Rational::ZERO) {
            return Err(ill_formed(
                task,
                format!(
                    "writes `{}`, a release separation of zero, which no run can release",
                    release.to_canonical()
                ),
                "M1.42",
            ));
        }
        let internal = joint::task_name(&system, &task.name);
        if timing.periodic
            && rebuilt.bindings.iter().any(|b| {
                b.requester == internal
                    && b.requirement.fact() == INTERRUPT_SOURCE
                    && met_by_true(&b.requirement)
            })
        {
            return Err(ill_formed(
                task,
                "is periodic and bound to an interrupt source itself, a second release model"
                    .to_string(),
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
    let mut naming_systems: Vec<String> = rebuilt
        .requesters
        .iter()
        .flat_map(|r| {
            r.uses
                .iter()
                .chain(&r.needs)
                .filter(|n| is_system(n))
                .map(move |n| format!("`{}` names the system `{n}`", r.name))
        })
        .collect();
    naming_systems.sort();
    naming_systems.dedup();
    if !naming_systems.is_empty() {
        return Err(refuse(
            "tool-failure",
            format!(
                "{}: a system is no thing to use, and the description check accepted it, which `M1.46` makes it refuse",
                naming_systems.join("; ")
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
                    Some(format!(
                        "`{}` is obliged to a `{}`",
                        assignment::instance_text(instance, system_text),
                        offer.fact
                    ))
                }
                _ => None,
            })
    });
    let mut past_obliged: Vec<String> = past_obliged.collect();
    past_obliged.sort();
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
    if record.placement.is_some()
        || rebuilt
            .map
            .obligations
            .iter()
            .any(|o| matches!(o, Obligation::Placement { .. }))
    {
        return Err(refuse(
            "unsupported-profile",
            "the system has placement memory, and laying an image out in memory is `/2`'s (`M4.1.7`): describe a hosted platform, or wait for `/2`",
        ));
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
    let service_or_policy = |name: &str| {
        declarations.iter().any(|d| {
            matches!(d.head(), Some("defservice" | "defpolicy"))
                && d.items().get(1).and_then(Form::as_symbol) == Some(name)
        })
    };

    // §2: reaching — what a requester names by `uses` or `needs`, and what it is bound to as its provider, at any depth.
    let mut direct: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for r in &rebuilt.requesters {
        direct
            .entry(r.name.clone())
            .or_default()
            .extend(r.uses.iter().chain(&r.needs).cloned());
    }
    for b in &rebuilt.bindings {
        direct
            .entry(b.requester.clone())
            .or_default()
            .insert(b.provider.clone());
    }
    let reach = |from: &str| -> BTreeSet<String> {
        let mut seen = BTreeSet::new();
        let mut todo = vec![from.to_string()];
        while let Some(at) = todo.pop() {
            for next in direct.get(&at).into_iter().flatten() {
                if seen.insert(next.clone()) {
                    todo.push(next.clone());
                }
            }
        }
        seen
    };

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
    let mut receivers: Vec<(usize, String)> = Vec::new();
    for route in routes.iter().filter(|r| r.timer) {
        for r in &route.requesters {
            if own_level_or_task(r)
                && bound_directly(r, &route.provider, &|q| takes_a_channel(q, vocabulary))
            {
                receivers.push((
                    place(r),
                    format!(
                        "`{}` programs an alarm at `{}` itself",
                        names.text(r),
                        route.provider
                    ),
                ));
            }
        }
    }
    for instance in &rebuilt.instances {
        let runs = catalog
            .kind(&instance.kind)
            .is_some_and(|k| k.costs.iter().any(|c| runs_requester_code(&c.name)));
        if !runs {
            continue;
        }
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
        let named: Vec<String> = receivers.into_iter().map(|(_, what)| what).collect();
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
        // What it names by `uses` or `needs` that is a service or a policy, each once (§2.2).
        let uses: BTreeSet<String> = requester(&internal)
            .map(|r| {
                r.uses
                    .iter()
                    .chain(&r.needs)
                    .filter(|n| service_or_policy(n))
                    .cloned()
                    .collect()
            })
            .unwrap_or_default();
        // What releases it (§2.2).
        let trigger = if timing.periodic {
            releases.push(text.clone());
            "timer".to_string()
        } else {
            let reached = reach(&internal);
            let releasing: Vec<&&Route> = source_routes
                .iter()
                .filter(|r| {
                    r.requesters
                        .iter()
                        .any(|q| *q == internal || reached.contains(q))
                })
                .collect();
            match releasing.as_slice() {
                [route] if route.separation.is_ok_and(|s| s >= timing.release) => {
                    format!("(source {} every)", route.provider)
                }
                _ => {
                    owe(&mut owed, &text, "trigger", "description");
                    "unknown".to_string()
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

    // §2.6: the execution contexts, and what each interrupt runs of its own (§2.9 reads it).
    let mut contexts: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut runs_of: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for task in &tasks {
        let internal = joint::task_name(&system, &task.name);
        contexts
            .entry(internal.clone())
            .or_default()
            .insert("job".to_string());
        for n in reach(&internal).into_iter().filter(|n| nodes.contains(n)) {
            contexts.entry(n).or_default().insert("job".to_string());
        }
    }
    for route in &routes {
        let item = format!("(interrupt {})", route.item());
        let x = route.provider.as_str();
        let mut runs: BTreeSet<String> = BTreeSet::new();
        if nodes.contains(x) {
            runs.insert(x.to_string());
        }
        for r in &route.requesters {
            // The interrupt fact where the route is a source, a consumed fact where it is a timer route (§2.6).
            let fact_ok = |q: &Requirement| {
                if route.timer {
                    takes_a_channel(q, vocabulary)
                } else {
                    q.fact() == INTERRUPT_SOURCE && met_by_true(q)
                }
            };
            if nodes.contains(r) && bound_directly(r, x, &fact_ok) {
                runs.insert(r.clone());
            }
        }
        if route.timer {
            for instance in instance_over(x) {
                runs.insert(instance.provider.name.clone());
                let delivers = catalog
                    .kind(&instance.kind)
                    .is_some_and(|k| k.costs.iter().any(|c| runs_requester_code(&c.name)));
                if delivers {
                    for r in &route.requesters {
                        let through = rebuilt
                            .bindings
                            .iter()
                            .any(|b| b.requester == *r && b.provider == instance.provider.name);
                        if nodes.contains(r) && through {
                            runs.insert(r.clone());
                        }
                    }
                }
            }
        }
        let mut all = runs.clone();
        for n in &runs {
            all.extend(reach(n).into_iter().filter(|m| nodes.contains(m)));
        }
        for n in &all {
            contexts.entry(n.clone()).or_default().insert(item.clone());
        }
        runs_of.insert(item, all);
    }
    for n in &nodes {
        contexts
            .entry(n.clone())
            .or_default()
            .insert("initialisation".to_string());
    }
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
    // A service or a policy that takes a channel is multiplexed where it has two or more users: each of the system's
    // tasks that reaches it, and each interrupt whose context line holds it, but the timer route at a provider whose
    // channel it takes (§2.9).
    let reached: Vec<BTreeSet<String>> = tasks
        .iter()
        .map(|task| reach(&joint::task_name(&system, &task.name)))
        .collect();
    let channels: Vec<&Binding> = rebuilt
        .bindings
        .iter()
        .filter(|b| service_or_policy(&b.requester) && takes_a_channel(&b.requirement, vocabulary))
        .collect();
    let users = |node: &str| {
        // Its own channels' timer routes: at the provider it binds, or the one its instance programs.
        let own: BTreeSet<String> = channels
            .iter()
            .filter(|b| b.requester == node)
            .map(|b| {
                let x = names
                    .instances
                    .get(b.provider.as_str())
                    .map_or(b.provider.as_str(), |i| i.over.as_str());
                format!("(interrupt (timer-route {x}))")
            })
            .collect();
        // Its own route counts only where something else that route runs, neither it nor what it is bound to,
        // reaches it: that route's own code delivering to it is no caller (§2.9).
        let called_there = |item: &str| {
            runs_of
                .get(item)
                .is_some_and(|runs| runs.iter().any(|m| m != node && reach(m).contains(node)))
        };
        let jobs = reached.iter().filter(|r| r.contains(node)).count();
        let interrupts = contexts.get(node).map_or(0, |c| {
            c.iter()
                .filter(|i| i.starts_with("(interrupt ") && (!own.contains(*i) || called_there(i)))
                .count()
        });
        jobs + interrupts
    };
    let multiplexed: BTreeSet<&str> = channels
        .iter()
        .map(|b| b.requester.as_str())
        .filter(|node| users(node) >= 2)
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
        if contexts.get(n).is_some_and(|c| c.contains("job")) {
            owe(&mut owed, &text, "call-cost", "realization");
        }
    }
    owed.sort();
    owed.dedup();

    // §3: the lines, in the format's order.
    let mut lines = vec![
        format!("(format \"{FORMAT}\")"),
        format!("(profile {})", record.profile),
        format!("(system {system})"),
        format!("(program \"{digest}\")"),
        format!("(assignment \"{}\")", sha256(record_text)),
        format!("(adapters {})", record.adapters),
    ];
    lines.extend(task_lines);
    lines.extend(bind_lines);
    lines.extend(instance_lines);
    lines.push(format!("(timer simulator {})", list("releases", &releases)));
    lines.extend(route_lines);
    lines.push(list("enabled", &enabled));
    lines.push("(interrupt-model (nesting no) (masking all))".to_string());
    lines.extend(context_lines);
    lines.extend(depends_lines);
    lines.extend(step_lines);
    lines.extend(obligation_lines);
    lines.extend(owed);
    lines.push("(complete no)".to_string());
    Ok(Plan { lines })
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
