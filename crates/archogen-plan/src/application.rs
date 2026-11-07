//! The application inputs, `archogen-application/1` (leaf `M4.12`; `docs/decisions/decision_application-inputs.md`): what a
//! description may not state of a task's own code, `ROADMAP.md` §10.3's "separately supplied application inputs", in one
//! file a person or a tool writes. It reads what `archogen-runtime-plan/1` owes `(by application)`, and each bound carries
//! `ROADMAP.md` §7.3's record: units, origin, scope, target and binary identity, and evidence category.
//!
//! This reads the format and holds it against a plan. What carries it into a run is `archogen-runtime-plan/2`'s (`M4.1.7`), and what
//! judges its figures is the scheduling checker's (`M2.21`).

use std::collections::BTreeSet;

use eadl_front::{read_record, Form, SourceMap};

use crate::contents::Contents;

/// The format this reads and no other.
pub const FORMAT: &str = "archogen-application/1";

/// The most of a file this reads, which a person writes and so no engine bounds by what it wrote: about 12,000 tasks at a kilobyte
/// and a half each, and far past any set the analysis can hold.
pub const MOST_BYTES: usize = 16 << 20;

/// The only target a plan of format `/1` describes.
pub const HOSTED_PLAYGROUND: &str = "hosted-playground";

/// What joins the parts of [`CLAUSE_PARTS`] in the words a file quotes, where the record continues with what is no part of them.
pub const CLAUSE_CUT: &str = " … ";

/// What `leaves-interrupt-hardware-alone` declares, in the composite-inputs record's own words (its §2), each part read back from there by a
/// test. A plan's owed line shows the fact's name alone, and the name no longer says what it holds, so a file declaring it quotes this
/// whole: what a person signs is the words, and a change to them makes an earlier file refuse to read.
pub const CLAUSE_PARTS: [&str; 3] = [
    "after the first enabling of interrupts, application code runs only as a declared task's job or as a service's application code; and no application code, a task's, a service's or application initialisation's, writes the timer's counter or compare, the interrupt controller's configuration, or the hart's interrupt or trap state other than through the runtime API's masking primitives (a job's only: the fault contract, Terms) or reads or writes the controller's claim and complete registers, a read of which claims, or a declared source's device registers; no service calls a runtime primitive; and no application code releases a task or calls a runtime function other than the runtime API record's primitives and its completion path. A runtime function is any function of a catalog record's implementation, a driver's included, called after the first enabling of interrupts; before it, application initialisation reaches that hardware only through catalog functions, which `runtime-discipline.<id>` covers.",
    "Application code calls no entry of the fault path — in initialisation, before the first enabling of interrupts, as in a job or a service's application code after it; a panic reaching the image's handler is no such call",
    "The runtime's own data is what this section's facts read in memory or in a register a port reserves: what the scheduler holds, its mask depth among it, what the timer service holds of the next release, the plan's tables, each saved context, and what a service, a record's or generated, holds of its source. Application code writes none of it, nor any other data or code of a catalog record's implementation or of the code the build generates, but through the calls `leaves-interrupt-hardware-alone` lets it make, or into what a call between it and that code hands it to fill, and it hands no call a place in any of it to write, but what that code handed it; in initialisation as in a job or a service's application code, and by no other means, a stray pointer or an overrun of its own stack bound among them (its own stack, within its bound, is its own, and a write a guard stops writes nothing, and is the fault contract's). For this clause a runtime function is also a function of the code the build generates, but for a node's code its task line's `uses` lists or its own `bind` lines name (the runtime plan's §2.2, `RTP-H5`, `RTP-H13`), which §7 says what charges; and a job's code calls the completion path only as its last act, the path not returning to it.",
];

/// Why a text is no application inputs: where, and what is wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refused {
    /// The clause at fault.
    pub clause: String,
    /// What is wrong with it.
    pub why: String,
}

impl core::fmt::Display for Refused {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "`{}`: {}", self.clause, self.why)
    }
}

fn refused(clause: &str, why: impl Into<String>) -> Refused {
    Refused {
        clause: clause.to_owned(),
        why: why.into(),
    }
}

/// `ROADMAP.md` §7.3's four categories, as the catalog writes them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Category {
    /// Chosen, not derived.
    Assumed,
    /// The largest value seen. A safety factor changes nothing about what this is.
    ObservedMaximum,
    /// Supplied from outside, with its own scope.
    ExternallySupplied,
    /// Derived by an argument.
    AnalyticallyEstablished,
}

/// What an observed maximum saw and how far it was padded: what `ROADMAP.md` §7.3 asks be kept, since a padded observation is still one.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Observation {
    /// What was seen.
    pub observed: u64,
    /// The safety factor's numerator.
    pub numerator: u64,
    /// Its denominator.
    pub denominator: u64,
}

/// What a figure rests on: its category, and where to find that out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    /// Its category, a fact of its own (`RTP-H5`).
    pub category: Category,
    /// What was seen and how far it was padded, where it says so.
    pub observation: Option<Observation>,
    /// Where it can be checked, which all but an assumption must say.
    pub locator: Option<String>,
    /// Why it holds.
    pub basis: String,
}

/// The image a bound was obtained on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binary {
    /// `sha256:…`.
    Image(String),
    /// There was no image yet.
    Unbuilt,
}

/// One numerical bound, with §7.3's record. Its target is the file's.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bound {
    /// Its value, in `unit`.
    pub value: u64,
    /// Its unit.
    pub unit: String,
    /// What it applies to.
    pub scope: String,
    /// The binary it belongs to.
    pub binary: Binary,
    /// What it rests on.
    pub evidence: Evidence,
}

/// How a masked run ends, which the composition charges: `api.unmask` or `completion` (the composite-inputs record's §3).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Ending {
    /// An `unmask` call ends it.
    Unmask,
    /// The job's completion does, no `unmask` having.
    Completion,
}

/// One masked run of a task's own code with one of its endings: `CS_{i,r,e}^app`, and what it calls, `n_{r,e,p}`. A run that ends both ways
/// is written twice, once for each. Its bound keeps its own category and any padding (the composite-inputs record's §6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaskedRun {
    /// Its name, which nothing else reads.
    pub name: String,
    /// How this one ends.
    pub ending: Ending,
    /// Its bound, of the task's own code inside it.
    pub bound: Bound,
    /// Each primitive it calls, and the most it does, other than the `mask` and the `unmask` that delimit it.
    pub calls: Vec<(String, u64)>,
    /// Each node it calls, and the most it does.
    pub node_calls: Vec<(String, u64)>,
}

/// Counts of calls, with `ROADMAP.md` §7.3's record for them all, a list with no call in it saying the job makes none (the plan's §2.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Calls {
    /// What they apply to.
    pub scope: String,
    /// The binary they belong to.
    pub binary: Binary,
    /// What rests on it.
    pub evidence: Evidence,
    /// Each callee, and the most one job makes.
    pub calls: Vec<(String, u64)>,
}

/// Masked runs, with a record for the list, whose category is that of the counts inside them; each run's bound keeps its own.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaskedRuns {
    /// What the list applies to.
    pub scope: String,
    /// The binary it belongs to.
    pub binary: Binary,
    /// What the list and its counts rest on.
    pub evidence: Evidence,
    /// Each run.
    pub runs: Vec<MaskedRun>,
}

/// What one task's own code is.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// Its name in its system.
    pub name: String,
    /// The function a job runs.
    pub entry_point: String,
    /// `C_i^app`.
    pub execution_bound: Bound,
    /// `n_{i,p}`.
    pub os_calls: Calls,
    /// `n_{i,N}`, each into a node its plan's lines name (`RTP-H5`).
    pub node_calls: Calls,
    /// Its masked runs.
    pub masked_runs: MaskedRuns,
    /// Its code's stack bound.
    pub stack_bound: Bound,
    /// Whether it suspends itself.
    pub suspends: bool,
    /// Whether it locks the scheduler or defers preemption.
    pub locks_scheduler: bool,
    /// Whether it shares data outside its masked sections.
    pub shares_outside_sections: bool,
    /// Whether it masks only through the runtime API.
    pub masks_only_through_api: bool,
    /// Whether its own figures hold under any preemption and from any entry state.
    pub figures_under_preemption: bool,
    /// Each alarm it writes on a time base that is the counter itself, whose deadlines it says stand at or after its own release and
    /// within that alarm's `R` (`RA-H7`), which may be none.
    pub deadlines_within_reach: Vec<String>,
}

/// One file of application inputs.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Application {
    /// The system they are for.
    pub system: String,
    /// The target its bounds are of.
    pub target: String,
    /// That no application code touches what the runtime owns, with its grounds.
    pub leaves_interrupt_hardware_alone: bool,
    /// Why that holds.
    pub leaves_alone_basis: String,
    /// The stack bound of the application's initialisation, which the boot stack holds and that declaration rests on too.
    pub initialisation_stack_bound: Bound,
    /// Each task.
    pub tasks: Vec<Task>,
}

/// Each clause `form` holds, by its head, each at most once and none but `known`.
fn clauses<'f>(
    of: &str,
    form: &'f [Form],
    known: &[&str],
) -> Result<Vec<(&'f str, &'f Form)>, Refused> {
    let mut seen: Vec<(&str, &Form)> = Vec::new();
    for item in form {
        let Some(head) = item.head() else {
            return Err(refused(
                of,
                format!("`{}` is no clause", item.to_canonical()),
            ));
        };
        if !known.contains(&head) {
            return Err(refused(of, format!("no clause is called `{head}`")));
        }
        if seen.iter().any(|(name, _)| *name == head) {
            return Err(refused(of, format!("`{head}` is written twice")));
        }
        seen.push((head, item));
    }
    Ok(seen)
}

fn clause<'f>(of: &str, found: &[(&'f str, &'f Form)], name: &str) -> Result<&'f Form, Refused> {
    found
        .iter()
        .find(|(head, _)| *head == name)
        .map(|(_, form)| *form)
        .ok_or_else(|| refused(of, format!("it states no `{name}`")))
}

fn string(of: &str, form: &Form) -> Result<String, Refused> {
    match form.items() {
        [_, Form::Str { value, .. }]
            if value.chars().any(|c| c.is_control() || c == '"' || c == '\\') =>
        {
            Err(refused(
                of,
                format!(
                    "`{}` holds a line break, a quote or what is no text, which a plan's one line cannot carry",
                    form.head().unwrap_or_default()
                ),
            ))
        }
        [_, Form::Str { value, .. }] if !value.trim().is_empty() => Ok(value.clone()),
        _ => Err(refused(
            of,
            format!(
                "`{}` wants one text that says something",
                form.to_canonical()
            ),
        )),
    }
}

fn symbol(of: &str, form: &Form) -> Result<String, Refused> {
    match form.items() {
        [_, name] => name
            .as_symbol()
            .map(str::to_owned)
            .ok_or_else(|| refused(of, format!("`{}` wants one name", form.to_canonical()))),
        _ => Err(refused(
            of,
            format!("`{}` wants one name", form.to_canonical()),
        )),
    }
}

fn yes_or_no(of: &str, form: &Form) -> Result<bool, Refused> {
    match symbol(of, form)?.as_str() {
        "yes" => Ok(true),
        "no" => Ok(false),
        _ => Err(refused(
            of,
            format!("`{}` is `yes` or `no`", form.to_canonical()),
        )),
    }
}

fn natural(of: &str, form: &Form) -> Result<u64, Refused> {
    match form {
        Form::Integer { value, .. } => {
            u64::try_from(*value).map_err(|_| refused(of, format!("{value} is below nothing")))
        }
        other => Err(refused(
            of,
            format!("`{}` is no whole number", other.to_canonical()),
        )),
    }
}

fn the_natural(of: &str, form: &Form) -> Result<u64, Refused> {
    match form.items() {
        [_, value] => natural(of, value),
        _ => Err(refused(
            of,
            format!("`{}` wants one whole number", form.to_canonical()),
        )),
    }
}

fn category(
    of: &str,
    form: &Form,
    value: Option<u64>,
) -> Result<(Category, Option<Observation>), Refused> {
    let items = form.items();
    let word = items.get(1).and_then(Form::as_symbol).unwrap_or_default();
    match (word, items.get(2..).unwrap_or_default()) {
        ("assumed", []) => Ok((Category::Assumed, None)),
        ("observed-maximum", []) => Ok((Category::ObservedMaximum, None)),
        ("externally-supplied", []) => Ok((Category::ExternallySupplied, None)),
        ("analytically-established", []) => Ok((Category::AnalyticallyEstablished, None)),
        ("observed-maximum", rest) => {
            // A list's counts are each what was seen, and no one observation is any of them.
            let Some(value) = value else {
                return Err(refused(
                    of,
                    "a list's counts are each their own observation, so a list says `observed-maximum` alone",
                ));
            };
            let found = clauses(of, rest, &["observed", "safety-factor"])?;
            let observed = the_natural(of, clause(of, &found, "observed")?)?;
            let factor = clause(of, &found, "safety-factor")?;
            let [_, numerator, denominator] = factor.items() else {
                return Err(refused(
                    of,
                    "`safety-factor` is a numerator and a denominator",
                ));
            };
            let (numerator, denominator) = (natural(of, numerator)?, natural(of, denominator)?);
            // It must pad, and the value is what it gives: §7.3 lets it change no category, and a bare figure beside it could say anything.
            if denominator == 0 || numerator < denominator {
                return Err(refused(of, "a safety factor pads: its numerator is at least its denominator, and that is not zero"));
            }
            let padded =
                (u128::from(observed) * u128::from(numerator)).div_ceil(u128::from(denominator));
            if u128::from(value) != padded {
                return Err(refused(
                    of,
                    format!("{observed} by that factor is {padded}, and the value says {value}"),
                ));
            }
            Ok((
                Category::ObservedMaximum,
                Some(Observation {
                    observed,
                    numerator,
                    denominator,
                }),
            ))
        }
        _ => Err(refused(
            of,
            format!(
                "`{}` is none of §7.3's four categories",
                form.to_canonical()
            ),
        )),
    }
}

fn evidence(of: &str, found: &[(&str, &Form)], value: Option<u64>) -> Result<Evidence, Refused> {
    let (category, observation) = category(of, clause(of, found, "evidence")?, value)?;
    let basis = string(of, clause(of, found, "basis")?)?;
    let locator = match clause(of, found, "locator") {
        Ok(form) => Some(string(of, form)?),
        Err(_) if category == Category::Assumed => None,
        Err(_) => {
            return Err(refused(
                of,
                "what is no assumption says where it can be checked: `locator`",
            ))
        }
    };
    Ok(Evidence {
        category,
        observation,
        locator,
        basis,
    })
}

/// `(binary "sha256:…")` or `(binary unbuilt)`.
fn binary(of: &str, form: &Form) -> Result<Binary, Refused> {
    match form.items() {
        [_, Form::Symbol { name, .. }] if name == "unbuilt" => Ok(Binary::Unbuilt),
        [_, Form::Str { value, .. }]
            if value.strip_prefix("sha256:").is_some_and(|hex| {
                hex.len() == 64
                    && hex
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            }) =>
        {
            Ok(Binary::Image(value.clone()))
        }
        _ => Err(refused(
            of,
            format!("`{}` is `\"sha256:…\"` or `unbuilt`", form.to_canonical()),
        )),
    }
}

fn bound(of: &str, items: &[Form], units: &[&str], extra: &[&str]) -> Result<Bound, Refused> {
    let mut known = vec![
        "value", "unit", "scope", "binary", "evidence", "locator", "basis",
    ];
    known.extend_from_slice(extra);
    let found = clauses(of, items, &known)?;
    let value = the_natural(of, clause(of, &found, "value")?)?;
    let unit = symbol(of, clause(of, &found, "unit")?)?;
    if !units.contains(&unit.as_str()) {
        return Err(refused(
            of,
            format!("a unit here is one of {units:?}, and this says `{unit}`"),
        ));
    }
    Ok(Bound {
        value,
        unit,
        scope: string(of, clause(of, &found, "scope")?)?,
        binary: binary(of, clause(of, &found, "binary")?)?,
        evidence: evidence(of, &found, Some(value))?,
    })
}

/// What a node is written as here: its name, or an instance's `(adapter …)` text, as the plan writes it.
fn node_text(form: &Form) -> Option<String> {
    match form.as_symbol() {
        Some(name) => Some(name.to_owned()),
        None if form.head() == Some("adapter") => Some(form.to_canonical()),
        None => None,
    }
}

/// `(HEAD NAME COUNT)` each, none twice, `NAME` a node's text.
fn counted(of: &str, items: &[Form], head: &str) -> Result<Vec<(String, u64)>, Refused> {
    let mut calls: Vec<(String, u64)> = Vec::new();
    let mut seen = BTreeSet::new();
    for item in items {
        let [_, name, count] = item.items() else {
            return Err(refused(
                of,
                format!("`{}` is no `({head} NAME COUNT)`", item.to_canonical()),
            ));
        };
        let Some(name) = node_text(name) else {
            return Err(refused(
                of,
                format!("`{}` calls no name", item.to_canonical()),
            ));
        };
        if !seen.insert(name.clone()) {
            return Err(refused(of, format!("`{name}` is counted twice")));
        }
        calls.push((name, natural(of, count)?));
    }
    Ok(calls)
}

const LIST_RECORD: &[&str] = &["scope", "binary", "evidence", "locator", "basis"];

fn calls(of: &str, form: &Form, head: &str) -> Result<Calls, Refused> {
    let items = form.items().get(1..).unwrap_or_default();
    let (counts, rest): (Vec<Form>, Vec<Form>) = items
        .iter()
        .cloned()
        .partition(|item| item.head() == Some(head));
    let found = clauses(of, &rest, LIST_RECORD)?;
    Ok(Calls {
        scope: string(of, clause(of, &found, "scope")?)?,
        binary: binary(of, clause(of, &found, "binary")?)?,
        evidence: evidence(of, &found, None)?,
        calls: counted(of, &counts, head)?,
    })
}

const TIMES: &[&str] = &["s", "ms", "us", "ns"];
const SIZES: &[&str] = &["byte", "KiB", "MiB"];

fn masked_runs(of: &str, form: &Form) -> Result<MaskedRuns, Refused> {
    let items = form.items().get(1..).unwrap_or_default();
    let (runs, rest): (Vec<Form>, Vec<Form>) = items
        .iter()
        .cloned()
        .partition(|item| item.head() == Some("run"));
    let found = clauses(of, &rest, LIST_RECORD)?;
    let mut out: Vec<MaskedRun> = Vec::new();
    let mut seen = BTreeSet::new();
    for run in &runs {
        let [_, name, body @ ..] = run.items() else {
            return Err(refused(of, "a `run` has a name"));
        };
        let Some(name) = name.as_symbol() else {
            return Err(refused(of, format!("`{}` has no name", run.to_canonical())));
        };
        let of = format!("{of} run {name}");
        let of = of.as_str();
        let plain: Vec<Form> = body
            .iter()
            .filter(|item| !matches!(item.head(), Some("call" | "node-call" | "ending")))
            .cloned()
            .collect();
        let those = |head: &str| -> Vec<Form> {
            body.iter()
                .filter(|item| item.head() == Some(head))
                .cloned()
                .collect()
        };
        let endings = those("ending");
        let [ending] = endings.as_slice() else {
            return Err(refused(of, "it states one `ending`"));
        };
        let ending = match symbol(of, ending)?.as_str() {
            "unmask" => Ending::Unmask,
            "completion" => Ending::Completion,
            _ => {
                return Err(refused(
                    of,
                    format!("`{}` is `unmask` or `completion`", ending.to_canonical()),
                ))
            }
        };
        if !seen.insert((name.to_owned(), ending)) {
            return Err(refused(
                of,
                "this run with this ending is written twice: one that ends both ways is written once for each",
            ));
        }
        out.push(MaskedRun {
            name: name.to_owned(),
            ending,
            bound: bound(of, &plain, TIMES, &[])?,
            calls: counted(of, &those("call"), "call")?,
            node_calls: counted(of, &those("node-call"), "node-call")?,
        });
    }
    Ok(MaskedRuns {
        scope: string(of, clause(of, &found, "scope")?)?,
        binary: binary(of, clause(of, &found, "binary")?)?,
        evidence: evidence(of, &found, None)?,
        runs: out,
    })
}

/// Whether `text` is a path a generated job can call: names of a letter or `_`, then letters, digits or `_`, joined by `::`.
fn is_a_path(text: &str) -> bool {
    text.split("::").all(|name| {
        let mut chars = name.chars();
        chars
            .next()
            .is_some_and(|c| c.is_ascii_alphabetic() || c == '_')
            && chars.all(|c| c.is_ascii_alphanumeric() || c == '_')
            && name != "_"
    })
}

fn task(form: &Form) -> Result<Task, Refused> {
    let [_, name, body @ ..] = form.items() else {
        return Err(refused("task", "a task has a name"));
    };
    let Some(name) = name.as_symbol() else {
        return Err(refused(
            "task",
            format!("`{}` is no name", name.to_canonical()),
        ));
    };
    let of = format!("task {name}");
    let of = of.as_str();
    let found = clauses(
        of,
        body,
        &[
            "entry-point",
            "execution-bound",
            "os-calls",
            "node-calls",
            "masked-runs",
            "stack-bound",
            "suspends",
            "locks-scheduler",
            "shares-outside-sections",
            "masks-only-through-api",
            "figures-under-preemption",
            "deadlines-within-reach",
        ],
    )?;
    let fact = |name: &str| yes_or_no(of, clause(of, &found, name)?);
    let body_of = |name: &str| -> Result<&[Form], Refused> {
        Ok(clause(of, &found, name)?
            .items()
            .get(1..)
            .unwrap_or_default())
    };
    let mut deadlines_within_reach = Vec::new();
    for item in body_of("deadlines-within-reach")? {
        let Some(alarm) = node_text(item) else {
            return Err(refused(
                of,
                format!("`{}` names no alarm", item.to_canonical()),
            ));
        };
        if deadlines_within_reach.contains(&alarm) {
            return Err(refused(of, format!("`{alarm}` is written twice")));
        }
        deadlines_within_reach.push(alarm);
    }
    let entry_point = string(of, clause(of, &found, "entry-point")?)?;
    if !is_a_path(&entry_point) {
        return Err(refused(
            of,
            format!("`entry-point` is a path a job can call, such as `app::beat`, and this says `{entry_point}`"),
        ));
    }
    Ok(Task {
        name: name.to_owned(),
        entry_point,
        execution_bound: bound(
            &format!("{of} execution-bound"),
            body_of("execution-bound")?,
            TIMES,
            &[],
        )?,
        os_calls: calls(
            &format!("{of} os-calls"),
            clause(of, &found, "os-calls")?,
            "call",
        )?,
        node_calls: calls(
            &format!("{of} node-calls"),
            clause(of, &found, "node-calls")?,
            "call",
        )?,
        masked_runs: masked_runs(
            &format!("{of} masked-runs"),
            clause(of, &found, "masked-runs")?,
        )?,
        stack_bound: bound(
            &format!("{of} stack-bound"),
            body_of("stack-bound")?,
            SIZES,
            &[],
        )?,
        suspends: fact("suspends")?,
        locks_scheduler: fact("locks-scheduler")?,
        shares_outside_sections: fact("shares-outside-sections")?,
        masks_only_through_api: fact("masks-only-through-api")?,
        figures_under_preemption: fact("figures-under-preemption")?,
        deadlines_within_reach,
    })
}

/// Read one file of application inputs.
///
/// # Errors
///
/// [`Refused`], naming the clause at fault, for what does not read, is another format, or leaves out or repeats what the format holds.
pub fn read(text: &str) -> Result<Application, Refused> {
    if text.len() > MOST_BYTES {
        return Err(refused(
            "the file",
            format!(
                "it holds {} bytes, past the {MOST_BYTES} this reads",
                text.len()
            ),
        ));
    }
    let mut sources = SourceMap::new();
    let id = sources
        .add(FORMAT, text)
        .map_err(|error| refused("the file", error.to_string()))?;
    let (document, diagnostics) = read_record(&sources, id);
    if diagnostics.has_errors() {
        let said = diagnostics
            .items()
            .first()
            .map_or("", |first| first.message.as_str());
        return Err(refused(
            "the file",
            format!("it does not read as forms: {said}"),
        ));
    }
    // Every form, so that one a description may state and this format does not know is refused too.
    let (tasks, rest): (Vec<Form>, Vec<Form>) = document
        .forms
        .into_iter()
        .partition(|form| form.head() == Some("task"));
    let found = clauses(
        "the file",
        &rest,
        &[
            "format",
            "system",
            "target",
            "leaves-interrupt-hardware-alone",
            "initialisation",
        ],
    )?;
    let format = string("format", clause("the file", &found, "format")?)?;
    if format != FORMAT {
        return Err(refused(
            "format",
            format!("this reads `{FORMAT}`, and it says `{format}`"),
        ));
    }
    let leaves = clause("the file", &found, "leaves-interrupt-hardware-alone")?;
    let [_, answer, grounds @ ..] = leaves.items() else {
        return Err(refused(
            "leaves-interrupt-hardware-alone",
            "it says `yes` or `no`, and why",
        ));
    };
    let leaves_interrupt_hardware_alone = match answer.as_symbol() {
        Some("yes") => true,
        Some("no") => false,
        _ => {
            return Err(refused(
                "leaves-interrupt-hardware-alone",
                "it says `yes` or `no`",
            ))
        }
    };
    let grounds = clauses(
        "leaves-interrupt-hardware-alone",
        grounds,
        &["clause", "basis"],
    )?;
    let quoted = string(
        "leaves-interrupt-hardware-alone",
        clause("leaves-interrupt-hardware-alone", &grounds, "clause")?,
    )?;
    if quoted != CLAUSE_PARTS.join(CLAUSE_CUT) {
        return Err(refused(
            "leaves-interrupt-hardware-alone",
            "its `clause` quotes what it declares, in the words the book's runtime plan chapter gives whole, and these are others",
        ));
    }
    let initialisation = clause("the file", &found, "initialisation")?;
    let initialisation = clauses(
        "initialisation",
        initialisation.items().get(1..).unwrap_or_default(),
        &["stack-bound"],
    )?;
    let initialisation_stack_bound = bound(
        "initialisation stack-bound",
        clause("initialisation", &initialisation, "stack-bound")?
            .items()
            .get(1..)
            .unwrap_or_default(),
        SIZES,
        &[],
    )?;
    let mut read_tasks: Vec<Task> = Vec::new();
    let mut names = BTreeSet::new();
    for form in &tasks {
        let one = task(form)?;
        if !names.insert(one.name.clone()) {
            return Err(refused("task", format!("`{}` is written twice", one.name)));
        }
        read_tasks.push(one);
    }
    Ok(Application {
        system: symbol("system", clause("the file", &found, "system")?)?,
        target: symbol("target", clause("the file", &found, "target")?)?,
        leaves_interrupt_hardware_alone,
        leaves_alone_basis: string(
            "leaves-interrupt-hardware-alone",
            clause("leaves-interrupt-hardware-alone", &grounds, "basis")?,
        )?,
        initialisation_stack_bound,
        tasks: read_tasks,
    })
}

impl Application {
    /// What the declaration `leaves-interrupt-hardware-alone` rests on, which a yes or a no cannot carry: the clause counts an overrun of a code's own
    /// stack bound among what application code must not do, in initialisation as in a job, so the fact holds only as far as each of those bounds does, each with its category. The first is the application's initialisation, and the rest each task by its name.
    #[must_use]
    pub fn leaves_alone_rests_on(&self) -> Vec<(&str, Category)> {
        core::iter::once((
            "the application's initialisation",
            self.initialisation_stack_bound.evidence.category,
        ))
        .chain(
            self.tasks
                .iter()
                .map(|task| (task.name.as_str(), task.stack_bound.evidence.category)),
        )
        .collect()
    }

    /// What these inputs leave unmet of `plan`, each in words: the system and the target, each task as the plan has it and no other, each call into a
    /// node its lines name, in its `node-calls` and in each masked run (`RTP-H5`), and each fact the plan owes `(by application)` this format does not hold of that subject.
    /// A primitive's name is the catalog's, and the composition refuses one no record costs.
    #[must_use]
    pub fn unmet(&self, plan: &Contents) -> Vec<String> {
        let mut unmet = Vec::new();
        if self.system != plan.system {
            unmet.push(format!(
                "these are for `{}`, and the plan is of `{}`",
                self.system, plan.system
            ));
        }
        if self.target != HOSTED_PLAYGROUND {
            unmet.push(format!(
                "these are of the target `{}`, and this plan is of `{HOSTED_PLAYGROUND}`",
                self.target
            ));
        }
        for task in &self.tasks {
            if !plan.tasks.iter().any(|known| known.name == task.name) {
                unmet.push(format!("the plan has no task `{}`", task.name));
            }
        }
        for task in &plan.tasks {
            let Some(mine) = self.tasks.iter().find(|known| known.name == task.name) else {
                unmet.push(format!("nothing is said of the task `{}`", task.name));
                continue;
            };
            let in_runs = mine
                .masked_runs
                .runs
                .iter()
                .flat_map(|run| run.node_calls.iter());
            for (node, _) in mine.node_calls.calls.iter().chain(in_runs) {
                let named = task.uses.iter().any(|used| used == node)
                    || plan
                        .binds
                        .iter()
                        .any(|bind| bind.requester == task.identity && bind.provider == *node);
                if !named {
                    unmet.push(format!(
                        "the task `{}` calls `{node}`, which neither its task line nor its own `bind` lines name",
                        task.name
                    ));
                }
            }
        }
        for owed in &plan.owed {
            if owed.by != "application" {
                continue;
            }
            let held = if owed.subject == plan.system {
                owed.fact == "leaves-interrupt-hardware-alone"
            } else if plan.tasks.iter().any(|task| task.identity == owed.subject) {
                HELD_OF_A_TASK.contains(&owed.fact.as_str())
            } else {
                false
            };
            if !held {
                unmet.push(format!(
                    "the plan owes `{}` of {}, and this format holds no such thing",
                    owed.fact, owed.subject
                ));
            }
        }
        unmet
    }
}

/// What this format holds of a task, by the plan's name for each (the plan's §2.10).
pub const HELD_OF_A_TASK: &[&str] = &[
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
];
