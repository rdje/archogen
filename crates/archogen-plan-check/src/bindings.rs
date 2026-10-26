//! Checks (3) and (4) (leaf `M3.5.4.4`; `M3.5`'s design, *The checks*), from `docs/semantics/model.md` §8 alone:
//! each requirement that binds is bound once, each binding is one requirement's, and each provider named is one of the
//! description's blocks or platforms (3); and each binding is satisfied, by the model's judgement (4). Each mismatch is
//! a corrupted plan, `tool-failure`, naming the binding, the requirement where the description writes it
//! ([`crate::requesters`]), and what was expected and found.
//!
//! **What binds**: a requirement on a guarantee of a named function; a qualifier — `(function each)` — only where its
//! requester and every requester it reaches through `uses` and `needs` bind no function (rule 13); never a statement,
//! never `reachable-at-privilege`, which rule 9 judges at the profile's level.

use std::collections::{BTreeMap, BTreeSet};

use archogen_outcome::record::{Finding, Label, Severity};
use archogen_outcome::Status;
use eadl_front::{Form, Program, SourceMap};

use crate::assignment::{Assignment, Requester};
use crate::check::{at, Verdict, PLAN_BINDING, PLAN_DESCRIPTION, PLAN_UNSATISFIED};
use crate::model::judge::{judge, Verdict as Judged};
use crate::model::provider::{self, Provider};
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
pub(crate) fn fact_of(requirement: &Requirement) -> &str {
    match requirement {
        Requirement::Presence(fact)
        | Requirement::Constraint { fact, .. }
        | Requirement::Group { fact, .. }
        | Requirement::Statement(fact, _) => fact,
    }
}

/// A requester as a finding names it.
pub(crate) fn who(requester: &Requester) -> String {
    match requester {
        Requester::Named(name) => format!("`{name}`"),
        Requester::Task { system, task } => format!("the task `{task}` of `{system}`"),
    }
}

pub(crate) fn finding(
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
    let declared: BTreeMap<String, &Form> = program.qualified_declarations().into_iter().collect();
    let places = places(name, text);
    let place = |key: &(Requester, String)| place(&places, name, key);
    let mut found = Vec::new();
    // Check (3): each requirement that binds is bound.
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
    // Each binding a requirement's, of a requester the system serves, to one of the description's providers.
    let mut judged = Vec::new();
    for (key, provider) in &assignment.bindings {
        let (requester, fact) = key;
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
        match declared.get(provider) {
            Some(form) if matches!(form.head(), Some("defblock" | "defplatform")) => {
                judged.push((key, provider, *form, located));
            }
            _ => found.push(finding(
                PLAN_BINDING,
                format!(
                    "the plan binds `{fact}` for {} to `{provider}`, which is none of the description's blocks or \
                     platforms",
                    who(requester)
                ),
                place(key),
                Vec::new(),
                "resolve again: `archogen-assignment/1` binds a block or a platform the description declares",
            )),
        }
    }
    // Check (4): each binding satisfied, each of its requirements judged at its provider.
    let mut read: BTreeMap<&str, Option<Provider>> = BTreeMap::new();
    let mut refused = Vec::new();
    for (key, provider, form, located) in judged {
        let requester = &key.0;
        let p = read
            .entry(provider.as_str())
            .or_insert_with(|| match provider::read(form) {
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
            });
        let Some(p) = p else { continue };
        for l in located {
            let verdict = judge(p, &l.requirement);
            if verdict == Judged::Satisfied {
                continue;
            }
            let mut secondary = vec![place(key)];
            secondary.extend(
                offers(form, &l.requirement)
                    .into_iter()
                    .map(|offer| at(sources, offer.span(), &format!("what `{provider}` states"))),
            );
            found.push(finding(
                PLAN_UNSATISFIED,
                format!(
                    "{}'s requirement `{}` is not met at `{provider}`, the provider the plan binds it to: {}",
                    who(requester),
                    written(sources, l),
                    outcome(verdict, provider)
                ),
                at(sources, l.span, "the requirement"),
                secondary,
                "resolve again: a plan binds each requirement to a provider that satisfies it",
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
pub(crate) fn place(
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
pub(crate) fn places(name: &str, text: &str) -> BTreeMap<(Requester, String), Label> {
    let mut sources = SourceMap::new();
    let Ok(id) = sources.add(name, text) else {
        return BTreeMap::new();
    };
    let (document, _) = eadl_front::read(&sources, id);
    let Some(source) = sources.get(id) else {
        return BTreeMap::new();
    };
    let mut out = BTreeMap::new();
    for form in document.forms.iter().filter(|f| f.head() == Some("bind")) {
        let items = form.items();
        let requester = match items.get(1) {
            Some(Form::Symbol { name, .. }) => Requester::Named(name.clone()),
            Some(list) if list.head() == Some("task") => {
                let part = |i: usize| {
                    list.items()
                        .get(i)
                        .and_then(Form::as_symbol)
                        .map(str::to_owned)
                };
                let (Some(system), Some(task)) = (part(1), part(2)) else {
                    continue;
                };
                Requester::Task { system, task }
            }
            _ => continue,
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
