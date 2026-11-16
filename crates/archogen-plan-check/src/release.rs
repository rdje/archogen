//! The release requester (leaf `M4.1.7.5.4.3`), from `docs/decisions/decision_release-alarm.md` §1 and
//! `docs/semantics/model.md` §8 rule 1 alone: on a target, a system with a task the timer releases — one writing a
//! `period` — has one more requester, `(release SYSTEM)`, which the engine adds and the system names, with the record's
//! six lines as its requirements:
//!
//! ```text
//! (needs relative-delay counter-modulus tick-rate observation-coherent)
//! (available-in-state idle)
//! (alarm-wiring core)
//! (alarm-kind level-compare)
//! (unambiguous-horizon (at-least SHORTEST))
//! (supported-horizon (at-least SHORTEST))
//! ```
//!
//! `SHORTEST` the shortest such period, "as its task writes it" (model §8 rule 1). None is added on the playground, nor
//! with no such task, so whether a plan is on a target is its caller's to say: [`side`] answers for the system alone.
//!
//! ⭐ **The lines are a side the checker's model reads as any** ([`side`]): a form whose `needs` and `requires` hold
//! them, read by the model's own `read_side`, so a requirement of the release requester is judged, bound and counted as
//! one any requester writes, and nothing in the checker reads it a second way. Each of its forms is written at the place
//! of the `period` that sets `SHORTEST`, so a finding about the release requester points where the system asks for it.
//!
//! # Where the records leave a choice, the reading taken
//!
//! - **"A task the timer releases — one writing a `period`"**: a task of the system writing a `(period N UNIT)` that
//!   reads as a duration. One that does not read is check (2)'s, and asks nothing here.
//! - **The shortest**: by exact comparison, and among periods equal as quantities, the first a task writes, kept as that
//!   task writes it: `5000 us` stays `5000 us`, never `5 ms`.

use eadl_front::{Form, Program, Span};
use eadl_model::quantity::{Dimension, Quantity};

use crate::requesters::declarations;

/// What the release requester needs: the four guarantees of its timer's counter it reads by presence.
const NEEDS: [&str; 4] = [
    "relative-delay",
    "counter-modulus",
    "tick-rate",
    "observation-coherent",
];

/// The release requester's side: the record's six lines as a form the checker's model reads as any side, `(release
/// (needs …) (requires …))`, each written at the `period` that sets `SHORTEST`. `None` where `system` names no system of
/// `program`, or no task of it writes a `period` that reads as a duration.
#[must_use]
pub fn side(program: &Program, system: &str) -> Option<Form> {
    let declarations = declarations(program);
    let &(_, form) = declarations
        .get(system)
        .filter(|(_, form)| form.head() == Some("defsystem"))?;
    let (number, unit, at) = shortest(form)?;
    let symbol = |name: &str| Form::Symbol {
        name: name.to_owned(),
        span: at,
    };
    let list = |items: Vec<Form>| Form::List { items, span: at };
    let at_least = || list(vec![symbol("at-least"), number.clone(), unit.clone()]);
    let mut needs = vec![symbol("needs")];
    needs.extend(NEEDS.iter().map(|fact| symbol(fact)));
    Some(list(vec![
        symbol("release"),
        list(needs),
        list(vec![
            symbol("requires"),
            list(vec![symbol("available-in-state"), symbol("idle")]),
            list(vec![symbol("alarm-wiring"), symbol("core")]),
            list(vec![symbol("alarm-kind"), symbol("level-compare")]),
            list(vec![symbol("unambiguous-horizon"), at_least()]),
            list(vec![symbol("supported-horizon"), at_least()]),
        ]),
    ]))
}

/// The shortest `period` a task of `system` writes that reads as a duration, as its task writes it: its number, its
/// unit and the clause's place.
fn shortest(system: &Form) -> Option<(Form, Form, Span)> {
    let mut best: Option<(Quantity, &Form)> = None;
    for task in system
        .items()
        .iter()
        .filter(|clause| clause.head() == Some("task"))
    {
        for clause in task
            .items()
            .iter()
            .filter(|clause| clause.head() == Some("period"))
        {
            let [_, number, unit] = clause.items() else {
                continue;
            };
            let Ok(period) = Quantity::read(Some(number), Some(unit), clause.span()) else {
                continue;
            };
            if period.unit.dimension != Dimension::Time {
                continue;
            }
            let shorter = best.as_ref().is_none_or(|(least, _)| {
                period
                    .compare(*least)
                    .is_ok_and(|order| order == std::cmp::Ordering::Less)
            });
            if shorter {
                best = Some((period, clause));
            }
        }
    }
    let (_, clause) = best?;
    let [_, number, unit] = clause.items() else {
        return None;
    };
    Some((number.clone(), unit.clone(), clause.span()))
}
