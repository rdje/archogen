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
//! `archogen-result/1`), the provenance, the plan's own reader and its round trip, and the frozen goldens.

use std::collections::{BTreeMap, BTreeSet};

use archogen_digest::Digest;

use eadl_front::Form;
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_model::quantity::Quantity;
use eadl_model::rational::Rational;
use eadl_resolve::adapter::{self, Catalog, Instance, Offered};
use eadl_resolve::joint::{Because, Binding, Interrupt, Kind, Node, Obligation};
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

/// A clause's quantity, `(head N UNIT)`, in seconds.
fn seconds(clause: &Form) -> Option<Rational> {
    Quantity::read(clause.items().get(1), clause.items().get(2), clause.span())
        .ok()?
        .in_base()
        .ok()
}

/// A value of a time fact, in seconds.
fn value_seconds(value: &Value) -> Option<Rational> {
    match value {
        Value::Quantity(q) => q.in_base().ok(),
        _ => None,
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
        match name
            .strip_prefix(self.system.as_str())
            .and_then(|r| r.strip_prefix('/'))
        {
            Some(task) => format!("(task {} {task})", self.system),
            None => name.to_string(),
        }
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

/// `record`, an `archogen-assignment/2` record the configuration checker accepted, lowered into
/// `archogen-runtime-plan/1` over `declarations`, the description it was resolved from as the language's check read
/// it, whose elaborated program's digest is `program` (§1, §2, §3).
///
/// # Errors
///
/// §1's refusals: a record that does not read, names another program, catalog or profile, or that the generator's own
/// joint rules refuse; and what `/1` cannot lower — placement memory, a deadline measured from an event other than the
/// release, an alarm or an expiry a task or the system's own level would receive.
#[allow(clippy::too_many_lines)]
pub fn lower(
    declarations: &[Form],
    program: &str,
    record: &str,
    vocabulary: &Vocabulary,
    catalog: &Catalog,
) -> Result<Plan, Refusal> {
    // §1: the record read back by the generator's own reader, and refused before anything is lowered.
    let record_text = record;
    let record = assignment::read(record_text).map_err(|e| {
        refuse(
            "tool-failure",
            format!("the assignment record does not read: {e}"),
        )
    })?;
    if record.program != program {
        return Err(refuse(
            "stale-input",
            "the assignment record names another program's digest: resolve again",
        ));
    }
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
        if e.0.starts_with("the description declares no system") {
            refuse("stale-input", format!("{e}: resolve again"))
        } else {
            refuse(
                "tool-failure",
                format!("the assignment record does not rebuild over the description: {e}"),
            )
        }
    })?;
    let report = rebuilt.joint.check(&rebuilt.bindings);
    if !report.holds() || !rebuilt.map.refused.is_empty() || !rebuilt.map.unknowns.is_empty() {
        return Err(refuse(
            "tool-failure",
            "the generator's joint rules refuse the record the configuration checker accepted: the two roles disagree",
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
    let init = rebuilt
        .joint
        .initialization(&rebuilt.system, &rebuilt.bindings);
    let system = record.system.clone();
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

    // §2.2: the tasks, in runtime order.
    let decl = declarations
        .iter()
        .find(|d| {
            d.head() == Some("defsystem")
                && d.items().get(1).and_then(Form::as_symbol) == Some(system.as_str())
        })
        .ok_or_else(|| refuse("stale-input", "the record's system is gone: resolve again"))?;
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
                        Some(State::Valued(values)) => {
                            values.first().and_then(value_seconds).ok_or("unknown")
                        }
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
    for route in routes.iter().filter(|r| r.timer) {
        for r in &route.requesters {
            if own_level_or_task(r)
                && bound_directly(r, &route.provider, &|q| takes_a_channel(q, vocabulary))
            {
                return Err(refuse(
                    "unsupported-profile",
                    format!(
                        "`{}` programs an alarm at `{}` itself, and how a job receives an alarm is `M4.1.6`'s",
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
        if let Some(served) = &instance.serves {
            if runs && own_level_or_task(served) {
                return Err(refuse(
                    "unsupported-profile",
                    format!(
                        "`{}` delivers an expiry to `{}`, and how a job receives an expiry is `M4.1.6`'s",
                        names.text(&instance.provider.name),
                        names.text(served)
                    ),
                ));
            }
        }
    }

    // §2.2's lines, and what each task owes.
    let mut owed: Vec<String> = Vec::new();
    let owe = |owed: &mut Vec<String>, subject: &str, fact: &str, owner: &str| {
        owed.push(format!("(owed {subject} {fact} (by {owner}))"));
    };
    let mut task_lines = Vec::new();
    let mut releases = Vec::new();
    for task in &tasks {
        let internal = format!("{system}/{}", task.name);
        let text = names.text(&internal);
        // Its runtime index, `|hp(i)|`, the number of tasks of smaller rank (§2.2).
        let index = tasks.iter().filter(|t| t.rank < task.rank).count();
        let release = if let Some(c) = clause(task.form, "period") {
            format!(
                "(period {})",
                duration(seconds(&c).unwrap_or(Rational::ZERO))
            )
        } else if let Some(c) = clause(task.form, "min-separation") {
            format!(
                "(min-separation {})",
                duration(seconds(&c).unwrap_or(Rational::ZERO))
            )
        } else {
            return Err(refuse("tool-failure", format!("{text} states no release")));
        };
        let deadline = clause(task.form, "deadline")
            .and_then(|c| seconds(&c))
            .map(duration)
            .ok_or_else(|| refuse("tool-failure", format!("{text} states no deadline")))?;
        let reference = match clause(task.form, "deadline-from")
            .and_then(|c| c.items().get(1).and_then(Form::as_symbol).map(str::to_string))
        {
            Some(event) if event == "release" => "(deadline-from release)".to_string(),
            Some(event) => {
                return Err(refuse(
                    "unsupported-profile",
                    format!(
                        "{text}'s deadline is measured from `{event}`, and the analysis measures every relative deadline from the nominal release"
                    ),
                ))
            }
            None => {
                owe(&mut owed, &text, "deadline-from", "description");
                "(deadline-from)".to_string()
            }
        };
        let jitter = clause(task.form, "jitter")
            .and_then(|c| seconds(&c))
            .unwrap_or(Rational::ZERO);
        let overrun = clause(task.form, "on-overrun")
            .and_then(|c| {
                c.items()
                    .get(1)
                    .and_then(Form::as_symbol)
                    .map(str::to_string)
            })
            .unwrap_or_else(|| "fault".to_string());
        let uses: BTreeSet<String> = requester(&internal)
            .map(|r| r.uses.iter().chain(&r.needs).cloned().collect())
            .unwrap_or_default();
        // What releases it (§2.2).
        let trigger = if clause(task.form, "period").is_some() {
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
            let own = clause(task.form, "min-separation").and_then(|c| seconds(&c));
            match (releasing.as_slice(), own) {
                ([route], Some(own)) if route.separation.is_ok_and(|s| s >= own) => {
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
            "masked-runs",
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
                Some(State::Valued(values)) => match values.first().and_then(value_seconds) {
                    Some(s) => items.push(format!("(obliged {} {})", offer.fact, duration(s))),
                    None => items.push(format!("(obliged {})", offer.fact)),
                },
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
            for fact in ["acknowledge-point", "deferred-work", "service-cost"] {
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

    // §2.6: the execution contexts.
    let mut contexts: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for task in &tasks {
        let internal = format!("{system}/{}", task.name);
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
            let fact_ok = |q: &Requirement| {
                (q.fact() == INTERRUPT_SOURCE && met_by_true(q)) || takes_a_channel(q, vocabulary)
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
        for n in all {
            contexts.entry(n).or_default().insert(item.clone());
        }
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
    for o in &report.obligations {
        if let Obligation::Multiplex { service, .. } = o {
            obligation_lines.push(format!(
                "(obligation multiplex {service} (discharged-by M4.2 M2.21.2))"
            ));
            owe(&mut owed, service, "overhead", "realization");
        }
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
        format!("(program \"{program}\")"),
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
