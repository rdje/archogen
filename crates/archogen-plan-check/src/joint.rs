//! Check (5) (leaf `M3.5.5`), the joint rules: the bindings of one system's assignment judged together, written from
//! `docs/semantics/model.md` §8 and the checker's own crate alone, under `M3.5`'s fence — never from the generator's
//! reading of them. A conflict or an unknown in a plan the search wrote is a corrupted plan, `tool-failure`, each
//! finding under [`PLAN_JOINT`] naming its rule.
//!
//! Rule 7 (`M3.5.5.1`): a requester the system serves bound to a block or platform outside the system's hardware is a
//! conflict naming the requester, the provider and the system — "a provider on a platform the system does not use
//! never satisfying".
//!
//! Rules 2–4 (`M3.5.5.2`): a binding takes a channel when its requirement is on a consumed fact and is met by the value
//! `true` — `(needs f)`, `(f true)` or the group whose head it is. The requesters taking a channel at one provider,
//! each counted once — a service several users reach is one requester — number at most its `channels`: more is a
//! conflict, and two or more where `channels` is undescribed or offered without a value an unknown. One needs no count.

use std::collections::BTreeMap;

use archogen_outcome::record::Label;
use archogen_outcome::Status;
use eadl_front::{Form, Program, SourceMap};

use crate::assignment::{Assignment, Requester};
use crate::bindings::{fact_of, finding, place, places, who};
use crate::check::{at, Verdict, PLAN_JOINT};
use crate::model::provider::{self, Stated};
use crate::model::requirement::Requirement;
use crate::model::value::Value;
use crate::model::vocab;
use crate::requesters::{declarations, hardware, served, Served};

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
    // Rules 2–4: the requesters taking a channel at each provider.
    let mut taking: BTreeMap<&str, BTreeMap<&Requester, &(Requester, String)>> = BTreeMap::new();
    for (key, provider) in &assignment.bindings {
        if takes_channel(&served, key) {
            taking
                .entry(provider.as_str())
                .or_default()
                .entry(&key.0)
                .or_insert(key);
        }
    }
    for (provider, requesters) in taking {
        if requesters.len() < 2 {
            continue;
        }
        let Some(&(_, form)) = declarations.get(provider) else {
            continue;
        };
        let names: Vec<String> = requesters.keys().map(|r| who(r)).collect();
        let taken = i128::try_from(requesters.len()).unwrap_or(i128::MAX);
        let mut secondary: Vec<Label> = requesters
            .values()
            .map(|key| place(&places, name, key))
            .collect();
        secondary.extend(
            channels_written(form)
                .map(|offer| at(sources, offer.span(), &format!("what `{provider}` states"))),
        );
        let (message, repair) = match channels(form) {
            Channels::Count(offered) if taken <= offered => continue,
            Channels::Count(offered) => (
                format!(
                    "the requesters taking a channel at `{provider}` — {} — number {taken}, and `{provider}` offers                      `channels` {offered}: more than it has is a conflict (rule 3)",
                    names.join(", ")
                ),
                "resolve again: a plan binds no more requesters taking a channel at one provider than its `channels`                  — another provider, or one service several users share",
            ),
            Channels::Unknown(why) => (
                format!(
                    "the requesters taking a channel at `{provider}` — {} — number {taken}, and `{provider}` {why}:                      whether they fit is unknown (rule 3)",
                    names.join(", ")
                ),
                "resolve again: a plan binds two requesters taking a channel at one provider only where its                  `channels` is described",
            ),
        };
        let primary = secondary.remove(0);
        found.push(finding(PLAN_JOINT, message, primary, secondary, repair));
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

/// Whether a binding takes a channel (rule 2): its fact consumed, and a requirement on it its requester writes met by
/// `true` — presence, `(f true)` or `(f (exactly true))`, or the group whose head it is; `(f false)` takes none.
fn takes_channel(served: &Served<'_>, (requester, fact): &(Requester, String)) -> bool {
    if !vocab::entry(fact).is_some_and(|e| e.consumed) {
        return false;
    }
    served.sides.get(requester).is_some_and(|side| {
        side.requirements.iter().any(|l| {
            fact_of(&l.requirement) == fact
                && match &l.requirement {
                    Requirement::Presence(_) | Requirement::Group { .. } => true,
                    Requirement::Constraint { value, .. } => matches!(value, Value::Bool(true)),
                    Requirement::Statement(..) => false,
                }
        })
    })
}

/// What a provider offers of `channels`.
enum Channels {
    /// A count: absent reads as none.
    Count(i128),
    /// Undescribed, offered without a value, or past the exact arithmetic: why.
    Unknown(&'static str),
}

fn channels(form: &Form) -> Channels {
    let Ok(read) = provider::read(form) else {
        // Check (4) read every bound provider; one the model refuses never reaches here.
        return Channels::Unknown("does not read");
    };
    match read.stated.get("channels") {
        None => Channels::Unknown("does not describe `channels`"),
        Some(Stated::Unvalued) => Channels::Unknown("offers `channels` without a value"),
        Some(Stated::Absent) => Channels::Count(0),
        Some(Stated::Valued(values)) => values
            .iter()
            .find_map(|v| match v {
                Value::Count(n) => Some(Channels::Count(*n)),
                _ => None,
            })
            .unwrap_or(Channels::Unknown("offers `channels` as no count")),
        Some(Stated::Overflow) => Channels::Unknown("offers `channels` past the exact arithmetic"),
    }
}

/// Where a provider writes `channels`, offered or absent.
fn channels_written(form: &Form) -> impl Iterator<Item = &Form> {
    form.items()
        .iter()
        .skip(1)
        .filter(|clause| matches!(clause.head(), Some("offers" | "absent")))
        .flat_map(|clause| clause.items().iter().skip(1))
        .filter(|item| item.as_symbol().or_else(|| item.head()) == Some("channels"))
}
