//! Checks (3) and (4) (leaf `M3.5.4.4`; `M3.5`'s design, *The checks*), from `docs/semantics/model.md` §8 alone:
//! each requirement that binds is bound once, each binding is one requirement's, and each provider named is one of the
//! description's blocks or platforms (3); and each binding is satisfied, by the model's judgement (4). Each mismatch is
//! a corrupted plan, `tool-failure`, naming the binding, the requirement where the description writes it
//! ([`crate::requesters`]), and what was expected and found.
//!
//! **What binds**: a requirement on a guarantee of a named function; a qualifier — `(function each)` — only where its
//! requester and every requester it reaches through `uses` and `needs` bind no function (rule 13); never a statement,
//! never `reachable-at-privilege`, which rule 9 judges at the profile's level.
//!
//! ⭐ **An engine adapter's instance** (`docs/specs/adapters/decision_engine-adapters.md` §3.1, §4, §6; model §8 rule 15)
//! is a provider of the description when it programs one of its blocks or platforms, its kind the checker's table's
//! ([`crate::adapters`]) and an exclusive one its one requester's own. The system serves it when a requester it serves
//! is bound to it; then each fact its kind requires beneath is bound, to the provider it programs, and met there (3,
//! 4). A requirement bound to it is judged at what it offers ([`crate::instances::offered`]), and only where the
//! mediation gate admits its requester (§6) — a refusal of check (4)'s, since a mediating kind offers nothing to a
//! requester that did not allow mediation.

use std::collections::{BTreeMap, BTreeSet};

use archogen_outcome::record::{Finding, Label, Severity};
use archogen_outcome::Status;
use eadl_front::{Form, Program, SourceMap};

use crate::adapters::{Beneath, Kind};
use crate::assignment::{read_requester, Assignment, Instance, Provider, Requester};
use crate::check::{at, Verdict, PLAN_BINDING, PLAN_DESCRIPTION, PLAN_UNSATISFIED};
use crate::instances;
use crate::model::judge::{judge, Verdict as Judged};
use crate::model::provider as model;
use crate::model::requirement::{NotJudged, Requirement};
use crate::model::vocab::{self, Function, Role};
use crate::requesters::{served, Located};

/// Whether a requirement on a fact binds a provider.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Binds {
    /// A guarantee of a named function.
    Function,
    /// A qualifier: where its requester and what it reaches bind no function (rule 13).
    Qualifier,
    /// Never, and why.
    Never(&'static str),
}

fn binds(fact: &str) -> Binds {
    match vocab::entry(fact) {
        None => Binds::Never("the vocabulary does not declare it"),
        Some(e) if e.role == Role::Statement => {
            Binds::Never("a statement constrains no provider (§7 rule 6)")
        }
        Some(e) => match e.function {
            Some(Function::Of(_)) => Binds::Function,
            Some(Function::Each) if fact == "reachable-at-privilege" => Binds::Never(
                "rule 9 judges `reachable-at-privilege` against the level the profile gives the system",
            ),
            Some(Function::Each) => Binds::Qualifier,
            None => Binds::Never("it describes no function"),
        },
    }
}

/// The fact a requirement is on: a group's head for a group.
fn fact_of(requirement: &Requirement) -> &str {
    match requirement {
        Requirement::Presence(fact)
        | Requirement::Constraint { fact, .. }
        | Requirement::Group { fact, .. }
        | Requirement::Statement(fact, _) => fact,
    }
}

/// A requester as a finding names it.
fn who(requester: &Requester) -> String {
    match requester {
        Requester::Named(name) => format!("`{name}`"),
        Requester::Task { system, task } => format!("the task `{task}` of `{system}`"),
        Requester::Instance(instance) => format!("the instance `{}`", instance.canonical()),
    }
}

fn finding(
    code: &str,
    message: String,
    primary: Label,
    secondary: Vec<Label>,
    repair: &str,
) -> Finding {
    Finding {
        code: code.to_owned(),
        severity: Severity::Error,
        message,
        primary,
        secondary,
        repair: repair.to_owned(),
    }
}

/// Checks (3) and (4) of `assignment`, which check (1) read from `text`, the record `name` names, over the elaborated
/// `program`, `sources` its files.
pub(crate) fn check(
    sources: &SourceMap,
    program: &Program,
    name: &str,
    text: &str,
    assignment: &Assignment,
) -> Result<(), Verdict> {
    let served = served(program, &assignment.system).map_err(|(declaration, span, refused)| {
        let (status, why) = match refused {
            NotJudged::Unsupported(why) => (Status::UnsupportedProfile, why),
            NotJudged::Invalid(why) => (Status::InvalidDescription, why),
            NotJudged::NotAFact => (Status::InvalidDescription, String::new()),
        };
        Verdict {
            status,
            findings: vec![finding(
                PLAN_DESCRIPTION,
                format!("what `{declaration}` requires does not read: {why}"),
                at(sources, span, "here"),
                Vec::new(),
                "repair the description, then resolve again",
            )],
        }
    })?;
    // Which requesters bind a function, so which may bind a qualifier (rule 13).
    let functional: BTreeSet<&Requester> = served
        .sides
        .iter()
        .filter(|(_, side)| {
            side.requirements
                .iter()
                .any(|l| binds(fact_of(&l.requirement)) == Binds::Function)
        })
        .map(|(requester, _)| requester)
        .collect();
    let mut expected: BTreeMap<(Requester, String), Vec<&Located>> = BTreeMap::new();
    let mut unbinding: BTreeMap<(Requester, String), (&str, &Located)> = BTreeMap::new();
    for (requester, side) in &served.sides {
        let qualifies = !functional.contains(requester)
            && served
                .reach(requester)
                .iter()
                .all(|r| !functional.contains(r));
        for located in &side.requirements {
            let fact = fact_of(&located.requirement);
            let key = (requester.clone(), fact.to_owned());
            match binds(fact) {
                Binds::Function => expected.entry(key).or_default().push(located),
                Binds::Qualifier if qualifies => expected.entry(key).or_default().push(located),
                Binds::Qualifier => {
                    unbinding.entry(key).or_insert((
                        "a qualifier binds nothing where its requester, or one it reaches through `uses` and `needs`, \
                         binds a function: it is judged at each provider bound for them (rule 13)",
                        located,
                    ));
                }
                Binds::Never(why) => {
                    unbinding.entry(key).or_insert((why, located));
                }
            }
        }
    }
    // The instances the system serves (rule 7, as rule 15 amends it): each one a requester it serves is bound to.
    let mut serving: BTreeMap<&Instance, &(Requester, String)> = BTreeMap::new();
    for (key, provider) in &assignment.bindings {
        if let Provider::Instance(instance) = provider {
            if served.sides.contains_key(&key.0) {
                serving.entry(instance).or_insert(key);
            }
        }
    }
    let declared: BTreeMap<String, &Form> = program.qualified_declarations().into_iter().collect();
    let device = |name: &str| {
        declared
            .get(name)
            .copied()
            .filter(|form| matches!(form.head(), Some("defblock" | "defplatform")))
    };
    let places = places(name, text);
    let place = |key: &(Requester, String)| place(&places, name, key);
    let mut found = Vec::new();
    // Check (3): each requirement that binds is bound ...
    for (key, located) in &expected {
        let (requester, fact) = key;
        if !assignment.bindings.contains_key(key) {
            found.push(finding(
                PLAN_BINDING,
                format!("{}'s requirement on `{fact}` is bound to no provider", who(requester)),
                at(sources, located[0].span, "the requirement"),
                Vec::new(),
                "resolve again: a plan binds each requirement on a function's guarantee to one provider",
            ));
        }
    }
    // ... and each instance's, beneath (record §3.3).
    for (instance, first) in &serving {
        let Some(kind) = instances::kind(instance) else {
            continue;
        };
        let requester = Requester::Instance((*instance).clone());
        for fact in kind.beneath_names() {
            if !assignment
                .bindings
                .contains_key(&(requester.clone(), fact.to_owned()))
            {
                found.push(finding(
                    PLAN_BINDING,
                    format!(
                        "{}'s requirement beneath on `{fact}` is bound to no provider",
                        who(&requester)
                    ),
                    place(first),
                    Vec::new(),
                    "resolve again: an instance binds each fact its kind requires beneath to the provider it programs",
                ));
            }
        }
    }
    // Each binding a requirement's, of a requester the system serves, to one of the description's providers — a block
    // or a platform, or an instance programming one.
    let mut judged = Vec::new();
    let mut through = Vec::new();
    let mut beneath = Vec::new();
    for (key, provider) in &assignment.bindings {
        let (requester, fact) = key;
        if let Requester::Instance(instance) = requester {
            match own(instance, fact, provider, serving.contains_key(instance), &assignment.system) {
                Err((message, repair)) => found.push(finding(PLAN_BINDING, message, place(key), Vec::new(), repair)),
                Ok(requirements) => match device(&instance.over) {
                    Some(form) => beneath.push((key, instance.over.as_str(), form, requirements)),
                    None => found.push(finding(
                        PLAN_BINDING,
                        format!(
                            "the plan binds `{fact}` for {} to `{}`, which is none of the description's blocks or \
                             platforms",
                            who(requester),
                            instance.over
                        ),
                        place(key),
                        Vec::new(),
                        "resolve again: an instance programs a block or a platform the description declares",
                    )),
                },
            }
            continue;
        }
        if !served.sides.contains_key(requester) {
            found.push(finding(
                PLAN_BINDING,
                format!(
                    "the plan binds `{fact}` for {}, which the system `{}` does not serve",
                    who(requester),
                    assignment.system
                ),
                place(key),
                Vec::new(),
                "resolve again: a plan binds the requesters its system serves — its own level, its tasks, and what \
                 they reach through `uses` and `needs`",
            ));
            continue;
        }
        let Some(located) = expected.get(key) else {
            let (message, secondary) = match unbinding.get(key) {
                Some((why, located)) => (
                    format!("the plan binds `{fact}` for {}, whose requirement on it binds no provider: {why}", who(requester)),
                    vec![at(sources, located.span, "the requirement")],
                ),
                None => (
                    format!("the plan binds `{fact}` for {}, which requires nothing of it", who(requester)),
                    Vec::new(),
                ),
            };
            found.push(finding(
                PLAN_BINDING,
                message,
                place(key),
                secondary,
                "resolve again: a binding is one requirement's, on a function's guarantee",
            ));
            continue;
        };
        match provider {
            Provider::Named(provider) => match device(provider) {
                Some(form) => judged.push((key, provider.as_str(), form, located)),
                None => found.push(finding(
                    PLAN_BINDING,
                    format!(
                        "the plan binds `{fact}` for {} to `{provider}`, which is none of the description's blocks or \
                         platforms",
                        who(requester)
                    ),
                    place(key),
                    Vec::new(),
                    "resolve again: `archogen-assignment/2` binds a block or a platform the description declares, or \
                     an adapter's instance programming one",
                )),
            },
            Provider::Instance(instance) => match through_instance(instance, requester) {
                Err(why) => found.push(finding(
                    PLAN_BINDING,
                    format!(
                        "the plan binds `{fact}` for {} to `{}`, {why}",
                        who(requester),
                        instance.canonical()
                    ),
                    place(key),
                    Vec::new(),
                    "resolve again: an exclusive instance is its one requester's own, and a shared one serves every \
                     requester bound to it",
                )),
                Ok(kind) => match device(&instance.over) {
                    Some(form) => through.push((key, instance, kind, form, located)),
                    None => found.push(finding(
                        PLAN_BINDING,
                        format!(
                            "the plan binds `{fact}` for {} to `{}`, whose provider `{}` is none of the description's \
                             blocks or platforms",
                            who(requester),
                            instance.canonical(),
                            instance.over
                        ),
                        place(key),
                        Vec::new(),
                        "resolve again: an instance programs a block or a platform the description declares",
                    )),
                },
            },
        }
    }
    // Check (4): each binding satisfied, each of its requirements judged at its provider.
    let mut read: BTreeMap<String, Option<model::Provider>> = BTreeMap::new();
    let mut refused = Vec::new();
    let mut model_of = |provider: &str, form: &Form| -> Option<model::Provider> {
        read.entry(provider.to_owned())
            .or_insert_with(|| match model::read(form) {
                Ok(p) => Some(p),
                Err(why) => {
                    refused.push(finding(
                        PLAN_DESCRIPTION,
                        format!(
                            "what `{provider}` offers does not read: `{}`: {}",
                            why.fact, why.why
                        ),
                        at(sources, form.span(), "here"),
                        Vec::new(),
                        "repair the description, then resolve again",
                    ));
                    None
                }
            })
            .clone()
    };
    for (key, provider, form, located) in judged {
        let Some(p) = model_of(provider, form) else {
            continue;
        };
        for l in located {
            let verdict = judge(&p, &l.requirement);
            if verdict == Judged::Satisfied {
                continue;
            }
            let mut secondary = vec![place(key)];
            secondary.extend(
                offers(form, &l.requirement)
                    .into_iter()
                    .map(|offer| at(sources, offer.span(), &format!("what `{provider}` states"))),
            );
            found.push(unmet(sources, &key.0, l, provider, verdict, secondary));
        }
    }
    // An instance offers what its kind does over its device (record §3.1), to a requester the gate admits (§6).
    for (key, instance, kind, form, located) in through {
        let Some(over) = model_of(&instance.over, form) else {
            continue;
        };
        let requester = &key.0;
        let written: Vec<Requirement> = served
            .sides
            .get(requester)
            .map(|side| {
                side.requirements
                    .iter()
                    .map(|l| l.requirement.clone())
                    .collect()
            })
            .unwrap_or_default();
        let canonical = instance.canonical();
        let secondary = || {
            vec![
                place(key),
                at(
                    sources,
                    form.span(),
                    &format!("`{}`, the device `{canonical}` programs", instance.over),
                ),
            ]
        };
        if !instances::admits(kind, &written) {
            found.push(finding(
                PLAN_UNSATISFIED,
                format!(
                    "{} is bound for `{}` to `{canonical}`, whose kind `{}` mediates: it is offered only to a requester \
                     that wrote `({} allowed)` (the adapters record §6)",
                    who(requester),
                    key.1,
                    kind.name,
                    instances::MEDIATION
                ),
                at(sources, located[0].span, "the requirement"),
                secondary(),
                "resolve again: a mediating adapter serves a requester that allows mediation",
            ));
            continue;
        }
        let at_instance = match instances::offered(kind, &canonical, &over, &written) {
            Ok(p) => p,
            Err(why) => {
                found.push(table(&why, place(key)));
                continue;
            }
        };
        for l in located {
            let verdict = judge(&at_instance, &l.requirement);
            if verdict != Judged::Satisfied {
                found.push(unmet(
                    sources,
                    requester,
                    l,
                    &canonical,
                    verdict,
                    secondary(),
                ));
            }
        }
    }
    // An instance's requirements beneath, each met at the provider it programs.
    for (key, over, form, requirements) in beneath {
        let Some(p) = model_of(over, form) else {
            continue;
        };
        for (r, written) in requirements {
            let verdict = judge(&p, &r);
            if verdict == Judged::Satisfied {
                continue;
            }
            found.push(finding(
                PLAN_UNSATISFIED,
                format!(
                    "{}'s requirement beneath `{written}` is not met at `{over}`, the provider it programs: {}",
                    who(&key.0),
                    outcome(verdict, over)
                ),
                place(key),
                offers(form, &r)
                    .into_iter()
                    .map(|offer| at(sources, offer.span(), &format!("what `{over}` states")))
                    .collect(),
                "resolve again: an instance programs a provider that satisfies what its kind requires beneath",
            ));
        }
    }
    if !refused.is_empty() {
        return Err(Verdict {
            status: Status::InvalidDescription,
            findings: refused,
        });
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

/// An instance's own binding, of `fact` to `provider` (record §3.3, §4 rule 1): of an instance the system serves, of a
/// fact its kind requires beneath, to the provider it programs — each requirement its kind writes on the fact, with
/// how the table writes it; else the finding's message and repair.
fn own(
    instance: &Instance,
    fact: &str,
    provider: &Provider,
    served: bool,
    system: &str,
) -> Result<Vec<(Requirement, String)>, (String, &'static str)> {
    let who = who(&Requester::Instance(instance.clone()));
    if !served {
        return Err((
            format!(
                "the plan binds `{fact}` for {who}, which the system `{system}` does not serve: no requester it serves \
                 is bound to it"
            ),
            "resolve again: an instance is a requester of the system whose requesters are bound to it",
        ));
    }
    let Some(kind) = instances::kind(instance) else {
        return Err((
            format!(
                "the plan binds `{fact}` for {who}, whose kind `{}` this checker's table does not hold",
                instance.kind
            ),
            "resolve again with the adapter catalog this checker holds",
        ));
    };
    let Ok(requirements) = instances::beneath(kind) else {
        return Err((
            format!(
                "the checker's table does not read `{}`'s beneath",
                kind.name
            ),
            "report it: the checker's adapter table is its own fault",
        ));
    };
    let of: Vec<(Requirement, String)> = requirements
        .into_iter()
        .filter(|r| fact_of(r) == fact)
        .map(|r| {
            let written = kind
                .beneath
                .iter()
                .find(|item| matches!(item, Beneath::Needs(f) | Beneath::Constraint { fact: f, .. } if *f == fact))
                .map_or_else(|| format!("(needs {fact})"), instances::written);
            (r, written)
        })
        .collect();
    if of.is_empty() {
        return Err((
            format!(
                "the plan binds `{fact}` for {who}, whose kind `{}` requires nothing of it beneath",
                kind.name
            ),
            "resolve again: an instance binds what its kind requires beneath, and nothing else",
        ));
    }
    if *provider != Provider::Named(instance.over.clone()) {
        return Err((
            format!(
                "the plan binds `{fact}` for {who} to `{provider}`, and an instance binds what its kind requires \
                 beneath to the provider it programs, `{}`",
                instance.over
            ),
            "resolve again: an instance's requirements beneath are bound to the provider it programs",
        ));
    }
    Ok(of)
}

/// A binding of `requester` to `instance`: its kind the table's, and an exclusive one the requester's own (record §1.4,
/// §3.1); else why not.
fn through_instance(instance: &Instance, requester: &Requester) -> Result<&'static Kind, String> {
    let Some(kind) = instances::kind(instance) else {
        return Err(format!(
            "whose kind `{}` this checker's table does not hold",
            instance.kind
        ));
    };
    match (kind.exclusive(), instance.serves.as_deref()) {
        (true, Some(serves)) if serves == requester => Ok(kind),
        (true, Some(serves)) => Err(format!(
            "an exclusive instance, which serves `{}` alone",
            serves.canonical()
        )),
        (true, None) => Err("an exclusive instance that names no requester it serves".to_owned()),
        (false, Some(_)) => Err("a shared instance, which names no requester".to_owned()),
        (false, None) => Ok(kind),
    }
}

/// A requirement not met at the provider a plan binds it to: check (4)'s finding.
fn unmet(
    sources: &SourceMap,
    requester: &Requester,
    located: &Located,
    provider: &str,
    verdict: Judged,
    secondary: Vec<Label>,
) -> Finding {
    finding(
        PLAN_UNSATISFIED,
        format!(
            "{}'s requirement `{}` is not met at `{provider}`, the provider the plan binds it to: {}",
            who(requester),
            written(sources, located),
            outcome(verdict, provider)
        ),
        at(sources, located.span, "the requirement"),
        secondary,
        "resolve again: a plan binds each requirement to a provider that satisfies it",
    )
}

/// The checker's own table does not read: its fault, never the plan's.
fn table(why: &str, place: Label) -> Finding {
    finding(
        PLAN_BINDING,
        format!("the checker's adapter table does not read: {why}"),
        place,
        Vec::new(),
        "report it: the checker's adapter table is its own fault",
    )
}

/// What the description writes of a requirement, its whitespace collapsed: `(needs f)` for a fact `needs` names.
fn written(sources: &SourceMap, located: &Located) -> String {
    if let Requirement::Presence(fact) = &located.requirement {
        return format!("(needs {fact})");
    }
    let span = located.span;
    sources
        .get(span.source)
        .and_then(|s| s.text.get(span.start as usize..span.end as usize))
        .map_or_else(String::new, |text| {
            text.split_whitespace().collect::<Vec<_>>().join(" ")
        })
}

/// What the model found at the provider, said.
fn outcome(verdict: Judged, provider: &str) -> String {
    match verdict {
        Judged::Refused => format!("what `{provider}` offers lies outside it"),
        Judged::Absent => format!("`{provider}` declares it absent"),
        Judged::Unknown => {
            format!("`{provider}` offers it without a value, so whether it holds is unknown")
        }
        Judged::Undescribed => format!("`{provider}` does not describe it"),
        Judged::Unsupported => "comparing the two is past the exact arithmetic (§2)".to_owned(),
        Judged::Satisfied | Judged::Statement => "it holds".to_owned(),
    }
}

/// The offers and absences `provider` writes of the facts a requirement is judged by: its own, a group's parts', and
/// what a derived one is derived from.
fn offers<'f>(provider: &'f Form, requirement: &Requirement) -> Vec<&'f Form> {
    fn facts<'r>(requirement: &'r Requirement, out: &mut BTreeSet<&'r str>) {
        let fact = fact_of(requirement);
        out.insert(fact);
        if let Some(e) = vocab::entry(fact) {
            out.extend(e.derived_from.iter().chain(e.reads).copied());
        }
        if let Requirement::Group { parts, .. } = requirement {
            for part in parts {
                facts(part, out);
            }
        }
    }
    let mut judged = BTreeSet::new();
    facts(requirement, &mut judged);
    provider
        .items()
        .iter()
        .skip(1)
        .filter(|clause| matches!(clause.head(), Some("offers" | "absent")))
        .flat_map(|clause| clause.items().iter().skip(1))
        .filter(|item| {
            item.as_symbol()
                .or_else(|| item.head())
                .is_some_and(|fact| judged.contains(fact))
        })
        .collect()
}

/// A binding's place in the record, else the record's first line.
fn place(
    places: &BTreeMap<(Requester, String), Label>,
    name: &str,
    key: &(Requester, String),
) -> Label {
    places.get(key).cloned().unwrap_or_else(|| Label {
        source: name.to_owned(),
        line: 1,
        column: 1,
        message: "the record".to_owned(),
    })
}

/// Each binding's place in the record, by its requester and fact.
fn places(name: &str, text: &str) -> BTreeMap<(Requester, String), Label> {
    let mut sources = SourceMap::new();
    let Ok(id) = sources.add(name, text) else {
        return BTreeMap::new();
    };
    let (document, _) = eadl_front::read_record(&sources, id);
    let Some(source) = sources.get(id) else {
        return BTreeMap::new();
    };
    let mut out = BTreeMap::new();
    for form in document.forms.iter().filter(|f| f.head() == Some("bind")) {
        let items = form.items();
        let Some(Ok(requester)) = items.get(1).map(read_requester) else {
            continue;
        };
        let Some(fact) = items.get(2).and_then(Form::as_symbol) else {
            continue;
        };
        let position = source.position(form.span().start);
        out.insert(
            (requester, fact.to_owned()),
            Label {
                source: name.to_owned(),
                line: position.line,
                column: position.column,
                message: "the binding".to_owned(),
            },
        );
    }
    out
}
