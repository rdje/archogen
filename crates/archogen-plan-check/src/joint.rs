//! Check (5) (leaf `M3.5.5`), the joint rules: the bindings of one system's assignment judged together, written from
//! `docs/semantics/model.md` §8 and the checker's own crate alone, under `M3.5`'s fence — never from the generator's
//! reading of them. A conflict or an unknown in a plan the search wrote is a corrupted plan, `tool-failure`, each
//! finding under [`PLAN_JOINT`] naming its rule.
//!
//! Rule 7 (`M3.5.5.1`): a requester the system serves bound to a block or platform outside the system's hardware is a
//! conflict naming the requester, the provider and the system — "a provider on a platform the system does not use
//! never satisfying".

use archogen_outcome::Status;
use eadl_front::{Program, SourceMap};

use crate::assignment::Assignment;
use crate::bindings::{fact_of, finding, place, places, who};
use crate::check::{at, Verdict, PLAN_JOINT};
use crate::requesters::{declarations, hardware, served};

/// Check (5) of `assignment`, which checks (1)–(4) accepted, over the elaborated `program`, `sources` its files; `name`
/// and `text` the record's, for the spans of its bindings.
pub(crate) fn check(
    sources: &SourceMap,
    program: &Program,
    name: &str,
    text: &str,
    assignment: &Assignment,
) -> Result<(), Verdict> {
    let system = &assignment.system;
    let declarations = declarations(program);
    let hardware = hardware(program, &declarations, system);
    // Check (3) read every served requester's side; one it refuses never reaches here.
    let served = served(program, system).unwrap_or_default();
    let places = places(name, text);
    let mut found = Vec::new();
    // Rule 7: each binding to the system's own hardware.
    for (key, provider) in &assignment.bindings {
        if hardware.contains(provider) {
            continue;
        }
        let (requester, fact) = key;
        let written = served.sides.get(requester).into_iter().flat_map(|side| {
            side.requirements
                .iter()
                .filter(|l| fact_of(&l.requirement) == fact)
                .map(|l| at(sources, l.span, "the requirement"))
        });
        found.push(finding(
            PLAN_JOINT,
            format!(
                "{} is bound for `{fact}` to `{provider}`, which is no part of the system `{system}`'s hardware: a \
                 provider on a platform the system does not use never satisfies (rule 7)",
                who(requester)
            ),
            place(&places, name, key),
            written.collect(),
            "resolve again: a plan binds a system's requesters to the blocks and platforms its `platform` clause, \
             its own `uses` and `needs` and its tasks' name, and those they compose",
        ));
    }
    if found.is_empty() {
        Ok(())
    } else {
        Err(Verdict {
            status: Status::ToolFailure,
            findings: found,
        })
    }
}
