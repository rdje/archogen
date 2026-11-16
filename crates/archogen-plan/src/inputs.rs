//! An application's file as format `/2` writes it (`docs/decisions/decision_runtime-plan-format-2.md` §1, "The application
//! and the realization in the identity", and §3; leaf `M4.1.7.6.6`): `(application "HEX")`, `(code …)`, a `(task-inputs …)`
//! for each task, `(declared …)` and `(initialisation-inputs …)`.
//!
//! Each is the file's clause "written as the file states it, in `decision_application-inputs.md`'s forms and nothing
//! converted", in the record's order: so `1 KiB` and `1024 byte` stay two texts, and a string is the language's own
//! canonical text of it, written by `eadl_front`'s printer. Its reader, [`crate::application::read`], has already refused
//! a file that does not read, so each item here is one the format holds.

use std::collections::{BTreeMap, BTreeSet};

use archogen_digest::Digest;
use eadl_front::{read_record, Form, SourceMap, Span};

use crate::application::{
    Application, Binary, Bound, Calls, Category, Ending, Evidence, MaskedRuns, Task, CLAUSE_CUT,
    CLAUSE_PARTS, FORMAT,
};

/// What `/2` writes of one application's file: each of its lines, and what each task names of alarms in its
/// `deadlines-within-reach`, which decides what `RA-H7` still asks of it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) struct Lines {
    /// `(application "HEX")`.
    pub(crate) application: String,
    /// `(code NAME "PLACE" "DIGEST")`.
    pub(crate) code: String,
    /// Each task's `(task-inputs …)`, by its name in its system.
    pub(crate) tasks: BTreeMap<String, String>,
    /// `(declared leaves-interrupt-hardware-alone …)`.
    pub(crate) declared: String,
    /// `(initialisation-inputs …)`.
    pub(crate) initialisation: String,
    /// Each task's alarms its `deadlines-within-reach` names, by its name.
    pub(crate) named: BTreeMap<String, BTreeSet<String>>,
}

/// A string as the language's canonical text writes one, by `eadl_front`'s own printer (its §3 rules 3 and 5), so a
/// quote, a backslash or a control character in a basis is escaped as the file's canonical text escapes it.
fn quoted(value: &str) -> String {
    let mut sources = SourceMap::new();
    let source = sources
        .add("quoted", String::new())
        .expect("an empty source is added");
    Form::Str {
        value: value.to_owned(),
        span: Span {
            source,
            start: 0,
            end: 0,
        },
    }
    .to_canonical()
}

/// The digest of the file's canonical text, `eadl_front`'s `Document::to_canonical` of it as read (§1): so a comment
/// or spacing moves it not, and a figure, its evidence and the order of its clauses do.
fn digest(text: &str) -> String {
    let mut sources = SourceMap::new();
    let id = sources
        .add(FORMAT, text.to_owned())
        .expect("a file the reader took is added");
    let (document, _) = read_record(&sources, id);
    Digest::of(document.to_canonical().as_bytes()).hex()
}

/// A category's word, as the application's format writes it.
fn category(category: Category) -> &'static str {
    match category {
        Category::Assumed => "assumed",
        Category::ObservedMaximum => "observed-maximum",
        Category::ExternallySupplied => "externally-supplied",
        Category::AnalyticallyEstablished => "analytically-established",
    }
}

/// `(binary unbuilt)` or `(binary "sha256:…")`.
fn binary(binary: &Binary) -> String {
    match binary {
        Binary::Unbuilt => "(binary unbuilt)".to_owned(),
        Binary::Image(identity) => format!("(binary {})", quoted(identity)),
    }
}

/// An evidence's items after its `scope` and `binary`: `(evidence …)`, an observed maximum's `observed` before its
/// `safety-factor` (RP19-3), then `(locator "…")` where the file states one, then `(basis "…")`.
fn evidence(evidence: &Evidence) -> Vec<String> {
    let mut items = vec![match evidence.observation {
        None => format!("(evidence {})", category(evidence.category)),
        Some(seen) => format!(
            "(evidence {} (observed {}) (safety-factor {} {}))",
            category(evidence.category),
            seen.observed,
            seen.numerator,
            seen.denominator
        ),
    }];
    if let Some(locator) = &evidence.locator {
        items.push(format!("(locator {})", quoted(locator)));
    }
    items.push(format!("(basis {})", quoted(&evidence.basis)));
    items
}

/// A bound's items, `BOUND` (§3): its `value` and `unit`, then its evidence's `scope`, `binary`, `evidence`,
/// `locator` and `basis`, each where it has one.
fn bound(bound: &Bound) -> Vec<String> {
    let mut items = vec![
        format!("(value {})", bound.value),
        format!("(unit {})", bound.unit),
        format!("(scope {})", quoted(&bound.scope)),
        binary(&bound.binary),
    ];
    items.extend(evidence(&bound.evidence));
    items
}

/// Each call of a list, `(HEAD NAME COUNT)`, sorted by its callee's text (§3).
fn calls(head: &str, calls: &[(String, u64)]) -> Vec<String> {
    let mut sorted: Vec<&(String, u64)> = calls.iter().collect();
    sorted.sort();
    sorted
        .iter()
        .map(|(name, count)| format!("({head} {name} {count})"))
        .collect()
}

/// A list of calls, `(HEAD …)`: its evidence's items, then its calls.
fn list(head: &str, list: &Calls) -> String {
    let mut items = vec![
        format!("(scope {})", quoted(&list.scope)),
        binary(&list.binary),
    ];
    items.extend(evidence(&list.evidence));
    items.extend(calls("call", &list.calls));
    format!("({head} {})", items.join(" "))
}

/// `(masked-runs …)`: the list's evidence's items, then each run by name and then ending, a run's name, its `ending`,
/// its bound and its evidence's items, then its `call`s and then its `node-call`s (RP18-4).
fn masked_runs(runs: &MaskedRuns) -> String {
    let mut items = vec![
        format!("(scope {})", quoted(&runs.scope)),
        binary(&runs.binary),
    ];
    items.extend(evidence(&runs.evidence));
    let mut sorted: Vec<_> = runs.runs.iter().collect();
    sorted.sort_by(|a, b| (&a.name, a.ending).cmp(&(&b.name, b.ending)));
    for run in sorted {
        let ending = match run.ending {
            Ending::Unmask => "unmask",
            Ending::Completion => "completion",
        };
        let mut run_items = vec![run.name.clone(), format!("(ending {ending})")];
        run_items.extend(bound(&run.bound));
        run_items.extend(calls("call", &run.calls));
        run_items.extend(calls("node-call", &run.node_calls));
        items.push(format!("(run {})", run_items.join(" ")));
    }
    format!("(masked-runs {})", items.join(" "))
}

/// `yes` or `no`.
fn yes(said: bool) -> &'static str {
    if said {
        "yes"
    } else {
        "no"
    }
}

/// A task's `(task-inputs (task SYSTEM NAME) …)` (§3): `entry-point`, `execution-bound`, `os-calls`, `node-calls`,
/// `masked-runs`, `stack-bound`, then the yes-or-no facts, then `deadlines-within-reach`, its alarms by their text.
fn task_inputs(system: &str, task: &Task) -> String {
    let mut deadlines: Vec<&String> = task.deadlines_within_reach.iter().collect();
    deadlines.sort();
    let deadlines: Vec<&str> = deadlines.into_iter().map(String::as_str).collect();
    let items = [
        format!("(task {system} {})", task.name),
        format!("(entry-point {})", quoted(&task.entry_point)),
        format!(
            "(execution-bound {})",
            bound(&task.execution_bound).join(" ")
        ),
        list("os-calls", &task.os_calls),
        list("node-calls", &task.node_calls),
        masked_runs(&task.masked_runs),
        format!("(stack-bound {})", bound(&task.stack_bound).join(" ")),
        format!("(suspends {})", yes(task.suspends)),
        format!("(locks-scheduler {})", yes(task.locks_scheduler)),
        format!(
            "(shares-outside-sections {})",
            yes(task.shares_outside_sections)
        ),
        format!(
            "(masks-only-through-api {})",
            yes(task.masks_only_through_api)
        ),
        format!(
            "(figures-under-preemption {})",
            yes(task.figures_under_preemption)
        ),
        if deadlines.is_empty() {
            "(deadlines-within-reach)".to_owned()
        } else {
            format!("(deadlines-within-reach {})", deadlines.join(" "))
        },
    ];
    format!("(task-inputs {})", items.join(" "))
}

/// What `/2` writes of `application`, read from `text` (§3).
pub(crate) fn lines(text: &str, application: &Application) -> Lines {
    let initialisation = match &application.initialisation {
        None => "(initialisation-inputs none)".to_owned(),
        Some(initialisation) => format!(
            "(initialisation-inputs (entry-point {}) (stack-bound {}) {})",
            quoted(&initialisation.entry_point),
            bound(&initialisation.stack_bound).join(" "),
            list("node-calls", &initialisation.node_calls)
        ),
    };
    Lines {
        application: format!("(application \"{}\")", digest(text)),
        code: format!(
            "(code {} {} {})",
            application.code.name,
            quoted(&application.code.place),
            quoted(&application.code.digest)
        ),
        tasks: application
            .tasks
            .iter()
            .map(|task| (task.name.clone(), task_inputs(&application.system, task)))
            .collect(),
        declared: format!(
            "(declared leaves-interrupt-hardware-alone {} (clause {}) (basis {}))",
            yes(application.leaves_interrupt_hardware_alone),
            quoted(&CLAUSE_PARTS.join(CLAUSE_CUT)),
            quoted(&application.leaves_alone_basis)
        ),
        initialisation,
        named: application
            .tasks
            .iter()
            .map(|task| {
                (
                    task.name.clone(),
                    task.deadlines_within_reach.iter().cloned().collect(),
                )
            })
            .collect(),
    }
}
