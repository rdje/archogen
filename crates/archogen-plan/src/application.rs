//! The application inputs, `archogen-application/1` (leaf `M4.12`; `docs/decisions/decision_application-inputs.md`): what a
//! description may not state of a task's own code, `ROADMAP.md` §10.3's "separately supplied application inputs", in one
//! file a person or a tool writes. It reads what `archogen-runtime-plan/1` owes `(by application)`, and each bound carries
//! `ROADMAP.md` §7.3's record: units, origin, scope, target and binary identity, and evidence category.
//!
//! This reads the format and holds it against a plan. What carries it into a run is `archogen-runtime-plan/2`'s (`M4.1.7`), and what
//! judges its figures is the scheduling checker's (`M2.21`).

use eadl_front::{read_record, Form, SourceMap};

use crate::contents::Contents;

/// The format this reads and no other.
pub const FORMAT: &str = "archogen-application/1";

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

/// What a figure rests on: its category, and where to find that out.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Evidence {
    /// Its category, a fact of its own (`RTP-H5`).
    pub category: Category,
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

/// One masked run of a task's own code: `CS_{i,r,e}^app`, and what it calls, `n_{r,e,p}`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaskedRun {
    /// Its name, which nothing else reads.
    pub name: String,
    /// Its bound.
    pub bound: Bound,
    /// Each primitive it calls, and the most it does.
    pub calls: Vec<(String, u64)>,
}

/// Counts of calls, whose evidence category is one for them all (the plan's §2.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Calls {
    /// What rests on it.
    pub evidence: Evidence,
    /// Each callee, and the most one job makes.
    pub calls: Vec<(String, u64)>,
}

/// Masked runs, whose evidence category is one for them all.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MaskedRuns {
    /// The category of their counts and bounds.
    pub category: Category,
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
    /// within that alarm's `R` (`RA-H7`).
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

fn category(of: &str, form: &Form, value: Option<u64>) -> Result<Category, Refused> {
    let items = form.items();
    let word = items.get(1).and_then(Form::as_symbol).unwrap_or_default();
    match (word, items.get(2..).unwrap_or_default()) {
        ("assumed", []) => Ok(Category::Assumed),
        ("observed-maximum", []) => Ok(Category::ObservedMaximum),
        ("externally-supplied", []) => Ok(Category::ExternallySupplied),
        ("analytically-established", []) => Ok(Category::AnalyticallyEstablished),
        ("observed-maximum", rest) => {
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
            match value {
                Some(value) if u128::from(value) != padded => Err(refused(
                    of,
                    format!("{observed} by that factor is {padded}, and the value says {value}"),
                )),
                _ => Ok(Category::ObservedMaximum),
            }
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
    let category = category(of, clause(of, found, "evidence")?, value)?;
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
        locator,
        basis,
    })
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
    let binary = clause(of, &found, "binary")?;
    let binary = match binary.items() {
        [_, Form::Symbol { name, .. }] if name == "unbuilt" => Binary::Unbuilt,
        [_, Form::Str { value, .. }]
            if value.strip_prefix("sha256:").is_some_and(|hex| {
                hex.len() == 64
                    && hex
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
            }) =>
        {
            Binary::Image(value.clone())
        }
        _ => {
            return Err(refused(
                of,
                format!("`{}` is `\"sha256:…\"` or `unbuilt`", binary.to_canonical()),
            ))
        }
    };
    Ok(Bound {
        value,
        unit,
        scope: string(of, clause(of, &found, "scope")?)?,
        binary,
        evidence: evidence(of, &found, Some(value))?,
    })
}

/// `(call NAME COUNT)` each, none twice.
fn counted(of: &str, items: &[Form]) -> Result<Vec<(String, u64)>, Refused> {
    let mut calls: Vec<(String, u64)> = Vec::new();
    for item in items {
        let ["call", ..] = [item.head().unwrap_or_default()] else {
            return Err(refused(
                of,
                format!("`{}` is no `(call NAME COUNT)`", item.to_canonical()),
            ));
        };
        let [_, name, count] = item.items() else {
            return Err(refused(
                of,
                format!("`{}` is no `(call NAME COUNT)`", item.to_canonical()),
            ));
        };
        let Some(name) = name.as_symbol() else {
            return Err(refused(
                of,
                format!("`{}` calls no name", item.to_canonical()),
            ));
        };
        if calls.iter().any(|(called, _)| called == name) {
            return Err(refused(of, format!("`{name}` is counted twice")));
        }
        calls.push((name.to_owned(), natural(of, count)?));
    }
    Ok(calls)
}

fn calls(of: &str, form: &Form) -> Result<Calls, Refused> {
    let items = form.items().get(1..).unwrap_or_default();
    let (counts, rest): (Vec<Form>, Vec<Form>) = items
        .iter()
        .cloned()
        .partition(|item| item.head() == Some("call"));
    let found = clauses(of, &rest, &["evidence", "locator", "basis"])?;
    Ok(Calls {
        evidence: evidence(of, &found, None)?,
        calls: counted(of, &counts)?,
    })
}

const TIMES: &[&str] = &["ns", "us", "ms"];
const SIZES: &[&str] = &["bytes", "KiB"];

fn masked_runs(of: &str, form: &Form) -> Result<MaskedRuns, Refused> {
    let items = form.items().get(1..).unwrap_or_default();
    let (runs, rest): (Vec<Form>, Vec<Form>) = items
        .iter()
        .cloned()
        .partition(|item| item.head() == Some("run"));
    let found = clauses(of, &rest, &["evidence"])?;
    let category = category(of, clause(of, &found, "evidence")?, None)?;
    let mut out: Vec<MaskedRun> = Vec::new();
    for run in &runs {
        let [_, name, body @ ..] = run.items() else {
            return Err(refused(of, "a `run` has a name"));
        };
        let Some(name) = name.as_symbol() else {
            return Err(refused(of, format!("`{}` has no name", run.to_canonical())));
        };
        if out.iter().any(|known| known.name == name) {
            return Err(refused(of, format!("the run `{name}` is written twice")));
        }
        let (calling, plain): (Vec<Form>, Vec<Form>) = body
            .iter()
            .cloned()
            .partition(|item| item.head() == Some("call"));
        let mut with_evidence = plain;
        // The category is the runs' own, once, as the plan has it (§2.2).
        with_evidence.push(
            rest.iter()
                .find(|item| item.head() == Some("evidence"))
                .cloned()
                .ok_or_else(|| refused(of, "it states no `evidence`"))?,
        );
        out.push(MaskedRun {
            name: name.to_owned(),
            bound: bound(&format!("{of} run {name}"), &with_evidence, TIMES, &[])?,
            calls: counted(of, &calling)?,
        });
    }
    Ok(MaskedRuns {
        category,
        runs: out,
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
    let (reach, rest): (Vec<Form>, Vec<Form>) = body
        .iter()
        .cloned()
        .partition(|item| item.head() == Some("deadlines-within-reach"));
    let found = clauses(
        of,
        &rest,
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
    for item in &reach {
        let alarm = symbol(of, item)?;
        if deadlines_within_reach.contains(&alarm) {
            return Err(refused(of, format!("`{alarm}` is written twice")));
        }
        deadlines_within_reach.push(alarm);
    }
    Ok(Task {
        name: name.to_owned(),
        entry_point: string(of, clause(of, &found, "entry-point")?)?,
        execution_bound: bound(
            &format!("{of} execution-bound"),
            body_of("execution-bound")?,
            TIMES,
            &[],
        )?,
        os_calls: calls(&format!("{of} os-calls"), clause(of, &found, "os-calls")?)?,
        node_calls: calls(
            &format!("{of} node-calls"),
            clause(of, &found, "node-calls")?,
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
    let mut sources = SourceMap::new();
    let id = sources
        .add(FORMAT, text)
        .map_err(|error| refused("the file", error.to_string()))?;
    let (document, diagnostics) = read_record(&sources, id);
    if diagnostics.has_errors() {
        return Err(refused("the file", "it does not read as forms"));
    }
    let forms: Vec<Form> = document.declarations().cloned().collect();
    let (tasks, rest): (Vec<Form>, Vec<Form>) = forms
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
    if quoted != CLAUSE_PARTS.join(" ") {
        return Err(refused(
            "leaves-interrupt-hardware-alone",
            "its `clause` quotes what it declares, in the words this format holds (`CLAUSE_PARTS`, and the book's runtime plan chapter), and these are others",
        ));
    }
    let mut read_tasks: Vec<Task> = Vec::new();
    for form in &tasks {
        let one = task(form)?;
        if read_tasks.iter().any(|known| known.name == one.name) {
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
        tasks: read_tasks,
    })
}

impl Application {
    /// What the declaration `leaves-interrupt-hardware-alone` rests on, which a yes or a no cannot carry: an overrun of a task's own stack bound
    /// is a write the clause forbids, so the fact holds only as far as each task's bound does, each with its category.
    #[must_use]
    pub fn leaves_alone_rests_on(&self) -> Vec<(&str, Category)> {
        self.tasks
            .iter()
            .map(|task| (task.name.as_str(), task.stack_bound.evidence.category))
            .collect()
    }

    /// What these inputs leave unmet of `plan`, each in words: the system, each task as the plan has it and no other, each call into a
    /// node its lines name (`RTP-H5`), and each fact the plan owes `(by application)`, a fact this format does not hold among them.
    #[must_use]
    pub fn unmet(&self, plan: &Contents) -> Vec<String> {
        let mut unmet = Vec::new();
        if self.system != plan.system {
            unmet.push(format!(
                "these are for `{}`, and the plan is of `{}`",
                self.system, plan.system
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
            for (node, _) in &mine.node_calls.calls {
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
            let held = if owed.subject == plan.system || owed.subject.is_empty() {
                owed.fact == "leaves-interrupt-hardware-alone"
            } else {
                HELD_OF_A_TASK.contains(&owed.fact.as_str())
                    || owed.fact == "leaves-interrupt-hardware-alone"
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
