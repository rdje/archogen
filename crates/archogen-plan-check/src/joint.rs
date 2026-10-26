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
//!
//! Rule 5 (`M3.5.5.3`): two distinct providers both bound whose regions share bytes are a conflict, unless one composes
//! the other through `uses`, at any depth; a region without a size, another's bytes above its base, an unknown.
//! Placement memory is owned by none, and a provider nothing is bound to owns nothing. Rule 10's assignment half: the
//! record places the image exactly where the system's hardware has placement memory, and in a region of it.
//!
//! Rule 9 (`M3.5.5.4`): the level a plan binds the system's callers to is the profile's, never the record's. A caller —
//! the system's own level, a task, a service or a policy, never a block or platform, which has no level — stating it
//! calls from levels that do not hold it is the description's `unsupported-profile`, judged first. Then each provider
//! bound for a caller is reachable at the level: a set without it, or `reachable-at-privilege` absent, a conflict;
//! undescribed or offered without a value, an unknown. With no level given, the rule says nothing.
//!
//! Rule 13 (`M3.5.5.5`): one requester's bindings for the facts of one function are to one provider. Where they are to
//! several, each of those requirements is judged at each of them: one meeting them all makes the split a conflict; none
//! meeting them all and one refuting none, an unknown naming it; otherwise a conflict. A qualifier, `(function each)`,
//! that binds nothing is judged at every provider bound for its requester's functions and those of each requester it
//! reaches: refused or absent a conflict, unknown or undescribed an unknown. `reachable-at-privilege` is rule 9's.

use std::collections::{BTreeMap, BTreeSet};

use archogen_outcome::record::Label;
use archogen_outcome::Status;
use eadl_front::{Form, Program, SourceMap};
use eadl_model::profile::Profile;

use crate::assignment::{Assignment, Requester};
use crate::bindings::{fact_of, finding, outcome, place, places, who, written};
use crate::check::{at, label, Verdict, PLAN_DESCRIPTION, PLAN_JOINT};
use crate::model::judge::{judge, Verdict as Judged};
use crate::model::provider::{self, Provider, Stated};
use crate::model::requirement::Requirement;
use crate::model::value::Value;
use crate::model::vocab::{self, Function, Role};
use crate::providers::{overlap, regions, Overlap, Region};
use crate::requesters::{declarations, hardware, served, Located, Served};

/// Check (5) of `assignment`, which checks (1)–(4) accepted, over the elaborated `program`, `sources` its files; `name`
/// and `text` the record's, for the spans of its bindings.
pub(crate) fn check(
    sources: &SourceMap,
    program: &Program,
    profile: &Profile,
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
    let level = profile.caller_level;
    // Rule 9, judged first: no context exists at the levels a caller states that do not hold the profile's.
    if !level.is_empty() {
        let unsupported = stated_levels(sources, &served, &declarations, system, level);
        if !unsupported.is_empty() {
            return Err(Verdict {
                status: Status::UnsupportedProfile,
                findings: unsupported,
            });
        }
    }
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
    ownership(sources, program, &declarations, assignment, &mut found);
    placement(
        sources,
        name,
        text,
        &declarations,
        &hardware,
        assignment,
        &mut found,
    );
    if !level.is_empty() {
        privilege(&declarations, assignment, level, &places, name, &mut found);
    }
    functions(
        sources,
        &served,
        &declarations,
        assignment,
        &places,
        name,
        &mut found,
    );
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

/// A region's bytes, as a finding writes them.
fn bytes(region: &Region) -> String {
    match region.size {
        Some(_) => format!("[{:#x}, {:#x})", region.base, region.known_end()),
        None => format!("from {:#x}, its size unstated", region.base),
    }
}

/// Rule 5 over the providers bound in the assignment: the regions two of them own sharing bytes, unless one composes the
/// other.
fn ownership(
    sources: &SourceMap,
    program: &Program,
    declarations: &BTreeMap<String, (usize, &Form)>,
    assignment: &Assignment,
    found: &mut Vec<archogen_outcome::record::Finding>,
) {
    let bound: BTreeSet<&str> = assignment
        .bindings
        .values()
        .map(String::as_str)
        .filter(|p| declarations.contains_key(*p))
        .collect();
    // What each owns: placement memory is rule 10's, owned by none.
    let owned: Vec<(&str, Vec<Region>)> = bound
        .iter()
        .map(|p| {
            let regions = regions(p, declarations[*p].1)
                .into_iter()
                .filter(|r| !r.placement())
                .collect();
            (*p, regions)
        })
        .collect();
    for (i, (p, mine)) in owned.iter().enumerate() {
        for (q, theirs) in &owned[i + 1..] {
            if composes(program, declarations, p, q) || composes(program, declarations, q, p) {
                continue;
            }
            for a in mine {
                for b in theirs {
                    let (message, repair) = match overlap(a, b) {
                        Overlap::No => continue,
                        Overlap::Yes => (
                            format!(
                                "`{p}`'s region `{}`, {}, and `{q}`'s region `{}`, {}, share bytes, and both are bound \
                                 while neither composes the other: two providers owning the same bytes is a conflict \
                                 (rule 5)",
                                a.name,
                                bytes(a),
                                b.name,
                                bytes(b)
                            ),
                            "resolve again with another provider; where one is part of the other, the description \
                             says so through `uses`",
                        ),
                        Overlap::Undecided => {
                            let (sizeless, other) = if a.size.is_none() { (a, b) } else { (b, a) };
                            (
                                format!(
                                    "`{}`'s region `{}` states no size, and `{}`'s region `{}`, {}, owns bytes above \
                                     its base: whether two bound providers own the same bytes is unknown (rule 5)",
                                    sizeless.provider,
                                    sizeless.name,
                                    other.provider,
                                    other.name,
                                    bytes(other)
                                ),
                                "resolve again only once the region's size is described",
                            )
                        }
                    };
                    found.push(finding(
                        PLAN_JOINT,
                        message,
                        at(sources, a.span, "a region"),
                        vec![at(sources, b.span, "the other")],
                        repair,
                    ));
                }
            }
        }
    }
}

/// Whether `from` composes `to` through `uses` — its own, or inside `requires` — at any depth.
fn composes(
    program: &Program,
    declarations: &BTreeMap<String, (usize, &Form)>,
    from: &str,
    to: &str,
) -> bool {
    fn used<'f>(form: &'f Form, out: &mut Vec<&'f Form>) {
        for clause in form.items().iter().skip(1) {
            match clause.head() {
                Some("uses") => out.extend(clause.items().iter().skip(1)),
                Some("requires") => used(clause, out),
                _ => {}
            }
        }
    }
    let mut seen = BTreeSet::new();
    let mut todo = vec![from.to_owned()];
    while let Some(next) = todo.pop() {
        if !seen.insert(next.clone()) {
            continue;
        }
        let Some(&(id, form)) = declarations.get(&next) else {
            continue;
        };
        let mut operands = Vec::new();
        used(form, &mut operands);
        for operand in operands {
            let Some(name) = operand.as_symbol() else {
                continue;
            };
            if let Ok(Some(qualified)) = program.resolve(id, name, operand.span()) {
                if qualified == to {
                    return true;
                }
                todo.push(qualified);
            }
        }
    }
    false
}

/// Rule 10's assignment half: the record's placement present exactly where the system's hardware has placement memory,
/// and naming a region of it.
fn placement(
    sources: &SourceMap,
    name: &str,
    text: &str,
    declarations: &BTreeMap<String, (usize, &Form)>,
    hardware: &BTreeSet<String>,
    assignment: &Assignment,
    found: &mut Vec<archogen_outcome::record::Finding>,
) {
    let memory: Vec<Region> = hardware
        .iter()
        .filter_map(|p| declarations.get(p).map(|(_, form)| regions(p, form)))
        .flatten()
        .filter(Region::placement)
        .collect();
    let system = &assignment.system;
    let beside = |memory: &[Region]| {
        memory
            .iter()
            .map(|r| at(sources, r.span, "placement memory"))
            .collect::<Vec<_>>()
    };
    match &assignment.placement {
        None if !memory.is_empty() => found.push(finding(
            PLAN_JOINT,
            format!(
                "the system `{system}`'s hardware has placement memory, `{}`'s `{}`, and the plan places its image \
                 nowhere (rule 10)",
                memory[0].provider, memory[0].name
            ),
            label(name, text, "system"),
            beside(&memory),
            "resolve again: a plan for a system with placement memory chooses a region of it",
        )),
        Some((provider, region))
            if !memory
                .iter()
                .any(|m| m.provider == *provider && m.name == *region) =>
        {
            found.push(finding(
                PLAN_JOINT,
                format!(
                    "the plan places the image in `{provider}`'s region `{region}`, which is no placement memory of \
                     the system `{system}`'s hardware — executable, unreserved, of a block or platform it uses \
                     (rule 10)"
                ),
                label(name, text, "placement"),
                beside(&memory),
                "resolve again: a plan places the image in placement memory of its system's own hardware",
            ));
        }
        _ => {}
    }
}

/// Whether a requester calls (rule 9): the system's own level, a task, a service or a policy; a block or a platform has
/// no level.
fn caller(requester: &Requester, declarations: &BTreeMap<String, (usize, &Form)>) -> bool {
    match requester {
        Requester::Task { .. } => true,
        Requester::Named(name) => declarations.get(name).is_some_and(|(_, form)| {
            matches!(form.head(), Some("defsystem" | "defservice" | "defpolicy"))
        }),
    }
}

/// Levels as a finding writes them.
fn levels(set: &BTreeSet<String>) -> String {
    set.iter()
        .map(|l| format!("`{l}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// Rule 9's first half: each caller's requirement on `reachable-at-privilege` whose levels do not hold `level`.
fn stated_levels(
    sources: &SourceMap,
    served: &Served<'_>,
    declarations: &BTreeMap<String, (usize, &Form)>,
    system: &str,
    level: &str,
) -> Vec<archogen_outcome::record::Finding> {
    let mut out = Vec::new();
    for (requester, side) in &served.sides {
        if !caller(requester, declarations) {
            continue;
        }
        for l in &side.requirements {
            let Requirement::Constraint {
                fact,
                value: Value::Set(stated),
                ..
            } = &l.requirement
            else {
                continue;
            };
            if fact == "reachable-at-privilege" && !stated.contains(level) {
                out.push(finding(
                    PLAN_DESCRIPTION,
                    format!(
                        "{} states it calls from {}, which do not hold the level the profile gives the system \
                         `{system}`, `{level}`: no context at those levels exists under it (rule 9)",
                        who(requester),
                        levels(stated)
                    ),
                    at(sources, l.span, "the requirement"),
                    Vec::new(),
                    "state the level the profile gives its system, or check the description under a profile with \
                     the level it states",
                ));
            }
        }
    }
    out
}

/// Rule 9's second half: each provider bound for a caller reachable at `level`.
fn privilege(
    declarations: &BTreeMap<String, (usize, &Form)>,
    assignment: &Assignment,
    level: &str,
    places: &BTreeMap<(Requester, String), Label>,
    name: &str,
    found: &mut Vec<archogen_outcome::record::Finding>,
) {
    let mut judged = BTreeSet::new();
    for (key, provider) in &assignment.bindings {
        let requester = &key.0;
        if !caller(requester, declarations) || !judged.insert((requester, provider)) {
            continue;
        }
        let Some(&(_, form)) = declarations.get(provider) else {
            continue;
        };
        let stated = provider::read(form)
            .ok()
            .and_then(|p| p.stated.get("reachable-at-privilege").cloned());
        let offered = match &stated {
            Some(Stated::Valued(values)) => values.iter().find_map(|v| match v {
                Value::Set(set) => Some(set.clone()),
                _ => None,
            }),
            _ => None,
        };
        let who = who(requester);
        let (message, repair) = match (&stated, offered) {
            (_, Some(set)) if set.contains(level) => continue,
            (_, Some(set)) => (
                format!(
                    "{who} is bound to `{provider}`, reachable from {} and not at `{level}`, the level the profile \
                     gives the system: a conflict (rule 9)",
                    levels(&set)
                ),
                "resolve again: a caller's providers are reachable at the level the profile gives its system",
            ),
            (Some(Stated::Absent), None) => (
                format!(
                    "{who} is bound to `{provider}`, which declares `reachable-at-privilege` absent — reachable from \
                     no level, `{level}` neither: a conflict (rule 9)"
                ),
                "resolve again: a caller's providers are reachable at the level the profile gives its system",
            ),
            (Some(Stated::Unvalued), None) => (
                format!(
                    "{who} is bound to `{provider}`, which offers `reachable-at-privilege` without a value: whether \
                     it is reachable at `{level}` is unknown (rule 9)"
                ),
                "resolve again only once the provider's level is described",
            ),
            _ => (
                format!(
                    "{who} is bound to `{provider}`, which does not describe `reachable-at-privilege`: whether it is \
                     reachable at `{level}` is unknown (rule 9)"
                ),
                "resolve again only once the provider's level is described",
            ),
        };
        found.push(finding(
            PLAN_JOINT,
            message,
            place(places, name, key),
            Vec::new(),
            repair,
        ));
    }
}

/// The function a fact describes, if a named one.
fn function_of(fact: &str) -> Option<&'static str> {
    match vocab::entry(fact).and_then(|e| e.function) {
        Some(Function::Of(function)) => Some(function),
        _ => None,
    }
}

/// Whether a requirement is on a qualifier rule 13 judges: `(function each)`, a guarantee, but `reachable-at-privilege`.
fn qualifier(requirement: &Requirement) -> bool {
    let fact = fact_of(requirement);
    fact != "reachable-at-privilege"
        && vocab::entry(fact)
            .is_some_and(|e| e.role == Role::Guarantee && e.function == Some(Function::Each))
}

/// One requester's bindings for one function, by the provider each is to.
type ByProvider<'a> = BTreeMap<&'a str, Vec<&'a (Requester, String)>>;

/// Rule 13 over the assignment: each requester's bindings for one function at one provider, and each qualifier that
/// binds nothing judged at the providers bound for the functions its requester relies on.
fn functions(
    sources: &SourceMap,
    served: &Served<'_>,
    declarations: &BTreeMap<String, (usize, &Form)>,
    assignment: &Assignment,
    places: &BTreeMap<(Requester, String), Label>,
    name: &str,
    found: &mut Vec<archogen_outcome::record::Finding>,
) {
    // Every bound provider, read once: check (4) read each, so none refuses here.
    let readings: BTreeMap<&str, Provider> = assignment
        .bindings
        .values()
        .filter_map(|p| {
            let (_, form) = declarations.get(p)?;
            provider::read(form).ok().map(|read| (p.as_str(), read))
        })
        .collect();
    // One function at one provider, per requester.
    let mut split: BTreeMap<(&Requester, &str), ByProvider<'_>> = BTreeMap::new();
    for (key, provider) in &assignment.bindings {
        if let Some(function) = function_of(&key.1) {
            split
                .entry((&key.0, function))
                .or_default()
                .entry(provider.as_str())
                .or_default()
                .push(key);
        }
    }
    for ((requester, function), providers) in split {
        if providers.len() < 2 {
            continue;
        }
        let requirements: Vec<&Located> = served
            .sides
            .get(requester)
            .into_iter()
            .flat_map(|side| &side.requirements)
            .filter(|l| function_of(fact_of(&l.requirement)) == Some(function))
            .collect();
        let (mut whole, mut refutes_none) = (None, None);
        for provider in providers.keys() {
            let Some(read) = readings.get(provider) else {
                continue;
            };
            let verdicts: Vec<Judged> = requirements
                .iter()
                .map(|l| judge(read, &l.requirement))
                .collect();
            if verdicts.iter().all(|v| *v == Judged::Satisfied) {
                whole.get_or_insert(*provider);
            } else if !verdicts
                .iter()
                .any(|v| matches!(v, Judged::Refused | Judged::Absent))
            {
                refutes_none.get_or_insert(*provider);
            }
        }
        let at_each = providers
            .iter()
            .map(|(p, keys)| {
                let facts: Vec<String> = keys.iter().map(|(_, f)| format!("`{f}`")).collect();
                format!("`{p}` for {}", facts.join(", "))
            })
            .collect::<Vec<_>>()
            .join("; ");
        let (why, repair) = match (whole, refutes_none) {
            (Some(p), _) => (
                format!("`{p}` meets them all, so binding them apart is a conflict"),
                "resolve again: a plan binds one requester's requirements on one function to one provider",
            ),
            (None, Some(p)) => (
                format!(
                    "none meets them all, and `{p}` refutes none: whether it would is unknown until its facts are \
                     described"
                ),
                "resolve again only once the provider's facts are described",
            ),
            (None, None) => (
                "none meets them all: a conflict".to_owned(),
                "resolve again: a requester relying on two devices of one function writes two requesters",
            ),
        };
        let mut labels = providers
            .values()
            .flatten()
            .map(|key| place(places, name, key));
        let primary = labels.next().expect("a split binds two providers");
        found.push(finding(
            PLAN_JOINT,
            format!(
                "{}'s requirements on the function `{function}` are bound to several providers — {at_each}: {why} \
                 (rule 13)",
                who(requester)
            ),
            primary,
            labels.collect(),
            repair,
        ));
    }
    // The qualifiers, at what their requesters rely on.
    for (requester, side) in &served.sides {
        let qualifiers: Vec<&Located> = side
            .requirements
            .iter()
            .filter(|l| qualifier(&l.requirement))
            .collect();
        if qualifiers.is_empty() {
            continue;
        }
        let relied: BTreeSet<&Requester> = std::iter::once(requester)
            .chain(served.reach(requester))
            .collect();
        let mut bound: BTreeMap<&str, &(Requester, String)> = BTreeMap::new();
        for (key, provider) in &assignment.bindings {
            if relied.contains(&key.0) && function_of(&key.1).is_some() {
                bound.entry(provider.as_str()).or_insert(key);
            }
        }
        // Relying on no function bound, the qualifier binds as any requirement: checks (3) and (4) judged it.
        for l in qualifiers {
            for (provider, key) in &bound {
                let Some(read) = readings.get(provider) else {
                    continue;
                };
                let verdict = judge(read, &l.requirement);
                let kind = match verdict {
                    Judged::Satisfied | Judged::Statement => continue,
                    Judged::Refused | Judged::Absent => "a conflict",
                    Judged::Unknown | Judged::Undescribed | Judged::Unsupported => "an unknown",
                };
                found.push(finding(
                    PLAN_JOINT,
                    format!(
                        "{}'s qualifier `{}` does not hold at `{provider}`, bound for {}'s `{}`: {} — {kind}, since a \
                         qualifier describes every function its requirer relies on (rule 13)",
                        who(requester),
                        written(sources, l),
                        who(&key.0),
                        key.1,
                        outcome(verdict, provider)
                    ),
                    at(sources, l.span, "the qualifier"),
                    vec![place(places, name, key)],
                    "resolve again: a provider a requester relies on meets its qualifiers",
                ));
            }
        }
    }
}
