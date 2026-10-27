//! The search (leaf `M3.4.2`; `ROADMAP.md` §5.4, the design in `docs/tasks/M3.md`'s `M3.4`): one system's
//! providers chosen, one per requester and fact, so that the relation satisfies each binding and the joint rules
//! accept them together — or what stops it, in §5.5's terms.
//!
//! ⭐ **Finite candidates, deterministic order, a budget** (§5.4: "Start with finite candidates, deterministic
//! backtracking and constraint checking"). A system's requesters are those [`joint::requesters`] writes that it
//! serves; each requirement one writes, statements apart, is grouped with the others it writes on the same fact, a
//! *pair*, which one provider meets whole, as the assignment record binds one per requester and fact. A pair's
//! candidates are the system's hardware ([`System::read`]) — the description's blocks and platforms alone until
//! `M3.7` (`SR-H19`) — at which the relation ([`judge`]) satisfies every requirement of the pair, in the engine's
//! selection policy's order ([`Policy`]). Complete assignments are judged by [`Joint::check`] in that order, the first
//! it accepts being the plan, under [`Limits`]: a budget spent is `analysis-inconclusive` and no plan (F11), never
//! infeasibility.
//!
//! ⛔ **No pruning on a partial assignment.** Model §8 rule 13 classes one requester's function split between providers
//! as a conflict or an unknown by what its other requirements of the function are, so a partial assignment's conflict
//! can become an unknown as bindings are added: pruning on it would answer `infeasible-configuration` where
//! `missing-fact` is due. The rules judge complete assignments alone.
//!
//! **What a failed search says** (§5.5, `SR-H18`). A pair no provider of the hardware meets takes the code of the
//! providers that take part — those whose answer to it is not undescribed: `unsupported-profile` if one is past the
//! exact arithmetic, `missing-fact` if one states the fact without a value or none takes part, else
//! `infeasible-configuration`, every one refusing it or declaring it absent. When every complete assignment is
//! refused, the first one free of conflicts gives its unknowns — describing what they lack might make it whole — and
//! otherwise the first one's conflicts are the conflict set. The description's verdict is the highest of its
//! diagnostics' codes by [`Verdict::precedence`].

use std::collections::BTreeMap;

use eadl_front::{Diagnostic, Form, Label, Verdict};

use crate::joint::{requesters, Binding, Joint, Kind, Obligation, Report, Requester, System};
use crate::offer::{self, Provider};
use crate::relation::{judge, Verdict as Answer};
use crate::requirement::Requirement;
use crate::vocabulary::Vocabulary;

/// The engine's selection policy (§5.4: "explicit preferred-provider order with stable tie-breaking, stored in engine
/// configuration rather than eADL"): the providers it names first, in its order, then every other in the
/// description's declaration order. The default names none.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Policy {
    /// The providers preferred, by name, most preferred first.
    pub prefer: Vec<String>,
}

impl Policy {
    /// `providers`, in this policy's order.
    fn order<'p>(&self, providers: &[&'p Provider]) -> Vec<&'p Provider> {
        let rank = |provider: &Provider| {
            self.prefer
                .iter()
                .position(|name| *name == provider.name)
                .unwrap_or(self.prefer.len())
        };
        let mut ordered: Vec<(usize, usize, &Provider)> = providers
            .iter()
            .enumerate()
            .map(|(declared, provider)| (rank(provider), declared, *provider))
            .collect();
        ordered.sort_by_key(|(rank, declared, _)| (*rank, *declared));
        ordered
            .into_iter()
            .map(|(_, _, provider)| provider)
            .collect()
    }
}

/// What one search may cost: the complete assignments it judges.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// The most complete assignments judged before the search gives up, `analysis-inconclusive`.
    pub steps: u64,
}

impl Limits {
    /// The budget `archogen resolve` searches under.
    pub const DEFAULT: Self = Self { steps: 100_000 };
}

/// Who requires, as the assignment record names a requester: a declaration by its name, or a task of a system — never
/// `system/task`, which a service may be named.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Who {
    /// A declaration — a service, a policy, a system at its own level — by its name.
    Named(String),
    /// A task of a system.
    Task {
        /// The system's name.
        system: String,
        /// The task's name within it.
        task: String,
    },
}

impl Who {
    /// `requester`, a requester `system` serves, as the record names it.
    fn of(requester: &Requester, system: &str) -> Self {
        match (requester.kind, requester.name.strip_prefix(system)) {
            (Kind::Task, Some(rest)) if rest.starts_with('/') => Self::Task {
                system: system.to_owned(),
                task: rest[1..].to_owned(),
            },
            _ => Self::Named(requester.name.clone()),
        }
    }
}

/// A plan: the system, the region its image is placed in, and one provider per requester and fact.
#[derive(Debug, Clone, PartialEq)]
pub struct Resolution {
    /// The system's name.
    pub system: String,
    /// The placement memory chosen, where the system has some: the provider offering the region, and the region's
    /// name within it (model §8 rule 10).
    pub placement: Option<(String, String)>,
    /// Each binding, (requester, fact) to provider: in the record's order, every named requester before every task.
    pub bindings: BTreeMap<(Who, String), String>,
    /// What the assignment obliges a realization to do, as [`Joint::check`] reports it.
    pub obligations: Vec<Obligation>,
    /// The complete assignments judged to reach it.
    pub steps: u64,
}

/// What a search concludes.
#[derive(Debug, Clone)]
pub enum Search {
    /// The first complete assignment the joint rules accept.
    Resolved(Resolution),
    /// No assignment: the verdict, and each diagnostic that says why.
    Refused {
        /// The highest of the diagnostics' codes.
        verdict: Verdict,
        /// What stops it.
        diagnostics: Vec<Diagnostic>,
    },
    /// The budget was spent first: no plan, and no conclusion either way (F11).
    Inconclusive {
        /// The complete assignments judged.
        steps: u64,
        /// What happened, and what to do.
        diagnostic: Diagnostic,
    },
}

/// One pair: a requester's requirements on one fact, and the providers meeting all of them.
struct Pair<'d> {
    requester: &'d Requester,
    fact: String,
    requirements: Vec<&'d Requirement>,
    candidates: Vec<String>,
}

/// Search `system`, a `defsystem` of `declarations` — one instance's: a plain description, or a module of one
/// instance — for a plan whose callers are bound at `level`, the profile's.
#[must_use]
pub fn search(
    declarations: &[Form],
    system: &Form,
    level: &str,
    vocabulary: &Vocabulary,
    policy: &Policy,
    limits: Limits,
) -> Search {
    let read = System::read(system, declarations);
    let read = match read.at(level, vocabulary) {
        Ok(read) => read,
        Err(why) => {
            return refused(vec![Diagnostic::error(
                "tool-failure",
                format!("the search was given a level it cannot bind callers at: {why}"),
                Label::new(system.span(), "the system searched"),
                "report it: the profile names a level the vocabulary does not have",
            )])
        }
    };
    let hardware = |decl: &Form| matches!(decl.head(), Some("defblock" | "defplatform"));
    let mut providers: Vec<(Provider, &Form)> = Vec::new();
    let mut diagnostics = Vec::new();
    for decl in declarations.iter().filter(|decl| hardware(decl)) {
        match offer::read(decl, vocabulary) {
            Ok(provider) => providers.push((provider, decl)),
            Err(refusals) => diagnostics.extend(refusals.iter().map(|r| r.diagnostic())),
        }
    }
    let mut all: Vec<Requester> = Vec::new();
    for decl in declarations
        .iter()
        .filter(|decl| !hardware(decl) && decl.head().is_some_and(|head| head.starts_with("def")))
    {
        match requesters(decl, vocabulary) {
            Ok(written) => all.extend(written),
            Err(refusals) => diagnostics.extend(refusals.iter().map(|r| r.diagnostic())),
        }
    }
    if !diagnostics.is_empty() {
        return refused(diagnostics);
    }
    let joint = match Joint::new(&providers, &all, vocabulary) {
        Ok(joint) => joint,
        Err(findings) => return refused(findings.iter().map(|f| f.diagnostic()).collect()),
    };
    let map = joint.map(&read);
    if !map.refused.is_empty() || !map.unknowns.is_empty() {
        return refused(
            map.refused
                .iter()
                .chain(&map.unknowns)
                .map(|f| f.diagnostic())
                .collect(),
        );
    }
    let placement = map.obligations.iter().find_map(|o| match o {
        Obligation::Placement {
            provider, region, ..
        } => Some((provider.clone(), region.clone())),
        _ => None,
    });
    let read = match &placement {
        Some((_, region)) => read.placed_in(region),
        None => read,
    };
    let in_hardware: Vec<&Provider> = providers
        .iter()
        .map(|(provider, _)| provider)
        .filter(|provider| read.hardware.contains(&provider.name))
        .collect();
    let ordered = policy.order(&in_hardware);
    let pairs = pairs(&all, &read, &ordered, vocabulary);
    let unmet: Vec<Diagnostic> = pairs
        .iter()
        .filter(|pair| pair.candidates.is_empty())
        .map(|pair| unmet(pair, &read, &in_hardware, vocabulary))
        .collect();
    if !unmet.is_empty() {
        return refused(unmet);
    }
    let joint = joint.for_system(read.clone());
    walk(&joint, &read, &pairs, placement, limits)
}

/// Every pair the requesters `system` serves write, in declaration order, each requester's facts in the order it first
/// writes them; each with its candidates among `ordered`.
fn pairs<'d>(
    all: &'d [Requester],
    system: &System,
    ordered: &[&Provider],
    vocabulary: &Vocabulary,
) -> Vec<Pair<'d>> {
    let mut out = Vec::new();
    for requester in all.iter().filter(|r| system.serves(&r.name)) {
        let mut facts: Vec<(String, Vec<&Requirement>)> = Vec::new();
        for requirement in &requester.requirements {
            if matches!(requirement, Requirement::Statement { .. }) {
                continue;
            }
            match facts
                .iter_mut()
                .find(|(fact, _)| fact == requirement.fact())
            {
                Some((_, written)) => written.push(requirement),
                None => facts.push((requirement.fact().to_string(), vec![requirement])),
            }
        }
        for (fact, requirements) in facts {
            let candidates = ordered
                .iter()
                .filter(|provider| {
                    requirements
                        .iter()
                        .all(|r| judge(provider, r, vocabulary) == Answer::Satisfied)
                })
                .map(|provider| provider.name.clone())
                .collect();
            out.push(Pair {
                requester,
                fact,
                requirements,
                candidates,
            });
        }
    }
    out
}

/// Why no provider of the hardware meets `pair` (`SR-H18`): the code the providers taking part decide.
fn unmet(
    pair: &Pair<'_>,
    system: &System,
    hardware: &[&Provider],
    vocabulary: &Vocabulary,
) -> Diagnostic {
    let mut taking_part: Vec<(&Provider, Answer)> = Vec::new();
    for provider in hardware {
        let answers: Vec<Answer> = pair
            .requirements
            .iter()
            .map(|r| judge(provider, r, vocabulary))
            .collect();
        let rank = |answer: &Answer| match answer {
            Answer::Unsupported => 0,
            Answer::Unknown => 1,
            Answer::Refused => 2,
            Answer::Absent => 3,
            _ => 4,
        };
        if let Some(worst) = answers
            .iter()
            .filter(|a| {
                !matches!(
                    a,
                    Answer::Satisfied | Answer::Undescribed | Answer::Statement
                )
            })
            .min_by_key(|a| rank(a))
        {
            taking_part.push((provider, *worst));
        }
    }
    let has = |answer: Answer| taking_part.iter().any(|(_, a)| *a == answer);
    let (code, repair) = if has(Answer::Unsupported) {
        (
            "unsupported-profile",
            format!(
                "a provider's `{}` lies past the exact arithmetic the engine decides: write it within range",
                pair.fact
            ),
        )
    } else if has(Answer::Unknown) {
        (
            "missing-fact",
            format!(
                "a provider states `{}` without a value: give its value, so the requirement can be judged",
                pair.fact
            ),
        )
    } else if taking_part.is_empty() {
        (
            "missing-fact",
            format!(
                "no block or platform of the system states `{}`: describe it at the one that provides it; a constraint \
                 only a realization meets — a scheduling policy's, a release's, a queue's — waits on the catalog's \
                 providers (`M3.7`, `docs/decisions/decision_substitutability-relation.md` §9)",
                pair.fact
            ),
        )
    } else {
        (
            "infeasible-configuration",
            format!(
                "every provider of the system's hardware stating `{}` refuses the requirement or declares the fact \
                 absent: use a provider that meets it, or relax the requirement",
                pair.fact
            ),
        )
    };
    let first = pair.requirements[0];
    let mut diagnostic = Diagnostic::error(
        code,
        format!(
            "`{}` requires `{}`, and no provider of system `{}`'s hardware meets it",
            pair.requester.name, pair.fact, system.name
        ),
        Label::new(first.span(), "required here"),
        repair,
    );
    for requirement in &pair.requirements[1..] {
        diagnostic = diagnostic.with_secondary(Label::new(requirement.span(), "and here"));
    }
    for (provider, answer) in taking_part {
        let said = match answer {
            Answer::Unsupported => "states it past the exact arithmetic",
            Answer::Unknown => "states it without a value",
            Answer::Absent => "declares it absent",
            _ => "offers a value the requirement refuses",
        };
        diagnostic = diagnostic.with_secondary(Label::new(
            provider.span,
            format!("`{}` {said}", provider.name),
        ));
    }
    diagnostic
}

/// Judge the complete assignments in order — the first pair's candidate varying slowest — until one holds or the
/// budget is spent.
fn walk(
    joint: &Joint<'_>,
    system: &System,
    pairs: &[Pair<'_>],
    placement: Option<(String, String)>,
    limits: Limits,
) -> Search {
    let mut choice = vec![0usize; pairs.len()];
    let mut steps = 0u64;
    let mut first: Option<Report> = None;
    let mut free: Option<Report> = None;
    loop {
        if steps == limits.steps {
            return Search::Inconclusive {
                steps,
                diagnostic: Diagnostic::error(
                    "analysis-inconclusive",
                    format!(
                        "the search judged {steps} complete assignment(s) of system `{}`, its budget, and found none \
                         the joint rules accept, nor showed that none exists",
                        system.name
                    ),
                    Label::new(system.span, "the system searched"),
                    "raise the search's budget, or narrow the choices: fewer providers meeting each requirement",
                ),
            };
        }
        steps += 1;
        let bindings: Vec<Binding> = pairs
            .iter()
            .zip(&choice)
            .flat_map(|(pair, &at)| {
                pair.requirements.iter().map(move |requirement| Binding {
                    requester: pair.requester.name.clone(),
                    requirement: (*requirement).clone(),
                    provider: pair.candidates[at].clone(),
                })
            })
            .collect();
        let report = joint.check(&bindings);
        if report.holds() {
            return Search::Resolved(Resolution {
                system: system.name.clone(),
                placement,
                bindings: pairs
                    .iter()
                    .zip(&choice)
                    .map(|(pair, &at)| {
                        (
                            (Who::of(pair.requester, &system.name), pair.fact.clone()),
                            pair.candidates[at].clone(),
                        )
                    })
                    .collect(),
                obligations: report.obligations,
                steps,
            });
        }
        if free.is_none() && report.conflicts.is_empty() {
            free = Some(report.clone());
        }
        if first.is_none() {
            first = Some(report);
        }
        // The next assignment: the last pair's choice advances, carrying into the one before it.
        let mut index = pairs.len();
        loop {
            if index == 0 {
                let (report, conflicts) = match (free, first) {
                    (Some(free), _) => (free, false),
                    (None, Some(first)) => (first, true),
                    (None, None) => {
                        unreachable!("one complete assignment is judged before the walk ends")
                    }
                };
                let findings = if conflicts {
                    report.conflicts
                } else {
                    report.unknowns
                };
                return refused(findings.iter().map(|f| f.diagnostic()).collect());
            }
            index -= 1;
            choice[index] += 1;
            if choice[index] < pairs[index].candidates.len() {
                break;
            }
            choice[index] = 0;
        }
    }
}

/// A refusal: the diagnostics, and the highest of their codes.
fn refused(diagnostics: Vec<Diagnostic>) -> Search {
    let verdict = diagnostics
        .iter()
        .map(|d| Verdict::of_code(d.code))
        .max_by_key(|v| v.precedence())
        .unwrap_or(Verdict::ToolFailure);
    Search::Refused {
        verdict,
        diagnostics,
    }
}
