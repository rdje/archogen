//! The minimal typed interpretation: a checked description in, an S0 plan out.
//!
//! `ROADMAP.md` §12 S0 asks for "a minimal typed interpretation and one fixed engine-owned
//! implementation". This is the first half. It answers one question — *what must the generated
//! system be observed to do?* — and it answers it in the vocabulary of the published observation
//! contract (`examples/s0-heartbeat/README.md`), not in the vocabulary of Rust.
//!
//! # Two families of refusal, and the difference matters
//!
//! | Family | Verdict | Means |
//! |---|---|---|
//! | the input does not describe a buildable system | `invalid-description` | no system, two systems, no tasks |
//! | the S0 engine has no realization for what is described | `unsupported-profile` | a sporadic release, a period that is not a whole millisecond |
//!
//! The second family is the one §5.4 legislates: *"If no known realization exists, explain the
//! missing engine capability; do not declare the requested function logically impossible unless
//! that conclusion has actually been established."* So every refusal in it names the capability
//! that is missing and the task-tree leaf that will supply it. The difference between "this
//! toolchain cannot do that yet" and "that is not a sensible thing to ask for" is the difference
//! between a user who files a request and a user who stops asking.
//!
//! # What it does not do
//!
//! No profile admission, no presence analysis, no unit checking beyond the conversion it needs.
//! `archogen build` runs the whole frontend first, so a description that reaches here has
//! already been accepted — which is why, for example, nothing here re-checks that priorities are
//! unique. That rule belongs to the profile and is enforced in `eadl_model::workload`; a second
//! copy here would be a second copy to drift.

use eadl_front::{Diagnostic, Form, Label, SourceId, Span};
use eadl_model::quantity::{unit, Quantity};

/// One task of the S0 plan.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Task {
    /// The declared name.
    pub name: String,
    /// The release period in whole milliseconds of modeled time.
    pub period_ms: i64,
    /// The priority rank; ascending is highest-first.
    pub priority: i64,
    /// Where the task was declared, kept for provenance (`S0.5`).
    pub span: Span,
}

/// The S0 plan: everything the emitter needs, and nothing else.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    /// The system's declared name.
    pub name: String,
    /// Its tasks, in declaration order.
    pub tasks: Vec<Task>,
    /// The observation horizon: the least common multiple of the declared periods.
    pub horizon_ms: i64,
    /// Where the system was declared.
    pub span: Span,
}

/// Interpret a checked description into an S0 plan.
///
/// # Errors
///
/// A single diagnostic whose `code` is the `ROADMAP.md` §5.5 result the caller must exit with.
/// One refusal, not a list: the first thing that makes the build impossible makes every later
/// observation about it meaningless.
pub fn interpret(forms: &[Form], source: SourceId) -> Result<Plan, Box<Diagnostic>> {
    let systems: Vec<&Form> = forms
        .iter()
        .filter(|form| form.head() == Some("defsystem"))
        .collect();

    let system = match systems.as_slice() {
        [only] => *only,
        [] => {
            return Err(Box::new(Diagnostic::error(
                "invalid-description",
                "this description declares no system, so there is nothing to build",
                Label::new(
                    forms.first().map_or(Span::at(source, 0), Form::span),
                    "no `defsystem` here",
                ),
                "`archogen build` takes a system description. Add a `(defsystem …)`, or point \
                 the command at the description that declares one — a file of blocks and \
                 services is a library, not a system",
            )))
        }
        [first, second, ..] => {
            return Err(Box::new(
                Diagnostic::error(
                    "invalid-description",
                    "this description declares more than one system",
                    Label::new(second.span(), "a second system is declared here"),
                    "`archogen build` builds one system. Split them into separate \
                     descriptions — choosing one silently would make the output depend on \
                     declaration order",
                )
                .with_secondary(Label::new(first.span(), "the first is declared here")),
            ))
        }
    };

    let name = system
        .items()
        .get(1)
        .and_then(Form::as_symbol)
        .unwrap_or("system")
        .to_string();

    let task_forms: Vec<&Form> = system
        .items()
        .iter()
        .filter(|clause| clause.head() == Some("task"))
        .collect();
    if task_forms.is_empty() {
        return Err(Box::new(Diagnostic::error(
            "invalid-description",
            format!("system `{name}` declares no tasks"),
            Label::new(system.span(), "no `(task …)` clause"),
            "declare at least one task. A system with no workload has no releases and no \
             hyperperiod, so there is no observation for the generated artifact to produce",
        )));
    }

    let mut tasks = Vec::with_capacity(task_forms.len());
    for form in task_forms {
        tasks.push(task(form)?);
    }

    let horizon_ms = horizon(&tasks)?;

    Ok(Plan {
        name,
        tasks,
        horizon_ms,
        span: system.span(),
    })
}

fn task(form: &Form) -> Result<Task, Box<Diagnostic>> {
    let name = form
        .items()
        .get(1)
        .and_then(Form::as_symbol)
        .unwrap_or("task")
        .to_string();

    let Some(period) = clause(form, "period") else {
        // A sporadic task. The description is valid eADL and inside the profile — §3.1 admits
        // "periodic or sporadic releases with declared minimum separation" — so this is a gap in
        // the engine, and §5.4 requires it to be reported as one.
        let separation = clause(form, "min-separation").unwrap_or(form);
        return Err(Box::new(Diagnostic::error(
            "unsupported-profile",
            format!("the S0 realization cannot release task `{name}` sporadically"),
            Label::new(
                separation.span(),
                "`min-separation` declares no release schedule",
            ),
            "this is missing engine support, not an impossible request: the profile admits \
             sporadic releases, and the S0 path derives its observation from declared periods, \
             so a release triggered by an event needs a modeled event source in the hosted \
             playground harness. That is task-tree leaf `M4.3`. To build today, declare a \
             `(period …)` instead",
        )));
    };

    let period_ms = whole_milliseconds(period, &name)?;

    let priority =
        match clause(form, "priority").and_then(|c| c.items().get(1)) {
            Some(Form::Integer { value, .. }) => *value,
            _ => return Err(Box::new(Diagnostic::error(
                "invalid-description",
                format!("task `{name}` has no integer priority"),
                Label::new(form.span(), "no usable `(priority …)`"),
                "give the task a whole-number priority. The generated system orders coincident \
                 releases by priority rank, so without one there is no defined observation",
            ))),
        };

    Ok(Task {
        name,
        period_ms,
        priority,
        span: form.span(),
    })
}

/// Read a duration clause as a whole, positive number of milliseconds.
///
/// The conversion is the point: `(period 1 s)` is a thousand whole milliseconds and builds,
/// while `(period 500 us)` is half of one and does not. A check that demanded the literal unit
/// `ms` would refuse the first, which is a restriction on *spelling* rather than on anything the
/// realization actually cannot do.
fn whole_milliseconds(clause: &Form, task_name: &str) -> Result<i64, Box<Diagnostic>> {
    let quantity = Quantity::read(clause.items().get(1), clause.items().get(2))?;
    let millisecond = unit("ms").expect("`ms` is in the unit table");
    let Ok(converted) = quantity.convert_to(millisecond) else {
        return Err(Box::new(Diagnostic::error(
            "invalid-description",
            format!("task `{task_name}`'s period is not a duration"),
            Label::new(clause.span(), "not a time"),
            "write the period as a duration, e.g. `(period 10 ms)`",
        )));
    };
    let (Some(floor), Some(ceil)) = (converted.value.floor(), converted.value.ceil()) else {
        return Err(Box::new(Diagnostic::error(
            "analysis-inconclusive",
            format!("task `{task_name}`'s period overflowed exact arithmetic"),
            Label::new(clause.span(), "too large to convert to milliseconds"),
            "use a smaller period. §7.4 requires overflow to be detected rather than wrapped, \
             so this is reported instead of producing a wrong horizon",
        )));
    };
    if floor != ceil || floor <= 0 {
        return Err(Box::new(Diagnostic::error(
            "unsupported-profile",
            format!(
                "the S0 realization needs whole-millisecond periods, and task `{task_name}` is {converted}"
            ),
            Label::new(clause.span(), "not a whole number of milliseconds"),
            "this is missing engine support, not an impossible request: S0's modeled clock ticks \
             in whole milliseconds, and the exact rational time base that removes the \
             restriction is task-tree leaf `M4.1`. To build today, express the period as a whole \
             number of milliseconds",
        )));
    }
    i64::try_from(floor).map_err(|_| {
        Box::new(Diagnostic::error(
            "analysis-inconclusive",
            format!("task `{task_name}`'s period does not fit a 64-bit millisecond count"),
            Label::new(clause.span(), "too large"),
            "use a smaller period",
        ))
    })
}

/// The observation horizon: the least common multiple of every declared period.
fn horizon(tasks: &[Task]) -> Result<i64, Box<Diagnostic>> {
    let mut horizon: i64 = 1;
    for task in tasks {
        let Some(next) = lcm(horizon, task.period_ms) else {
            return Err(Box::new(Diagnostic::error(
                "analysis-inconclusive",
                "the hyperperiod of this task set overflows a 64-bit millisecond count",
                Label::new(task.span, "this period makes the hyperperiod too large"),
                "choose periods with a smaller least common multiple — mutually prime periods \
                 multiply. §7.4 requires overflow detection, so this is reported rather than \
                 silently wrapped into a horizon that is not the hyperperiod",
            )));
        };
        horizon = next;
    }
    debug_assert!(horizon > 0, "periods are positive, so their lcm is");
    Ok(horizon)
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> Option<i64> {
    (a / gcd(a, b)).checked_mul(b)
}

fn clause<'a>(form: &'a Form, name: &str) -> Option<&'a Form> {
    form.items().iter().find(|item| item.head() == Some(name))
}
