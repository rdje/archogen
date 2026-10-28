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
//! providers that take part — those stating a fact it constrains, a group's sub-facts and a derivation's inputs among
//! them, or answering it otherwise than undescribed: `unsupported-profile` if one is past the exact arithmetic,
//! `missing-fact` if one leaves what it needs unknown or undescribed or none takes part, else
//! `infeasible-configuration`, every one refusing it or declaring it absent — its message saying that the search looked
//! at the description's providers alone, the engine realizing a requirement no other way yet, so that it reads as
//! missing engine support, never as impossibility (F10, `M3.4.5`). When every complete assignment is
//! refused, the first one free of conflicts gives its unknowns — describing what they lack might make it whole — and
//! otherwise the first one's conflicts are the conflict set. The description's verdict is the highest of its
//! diagnostics' codes by [`Verdict::precedence`].
//!
//! **One requirement explained** ([`explain`], leaf `M3.4.13.1`). The same run, its pairs read once: for each pair
//! selected, every provider of the hardware with its answer, the search's own diagnostic when none meets it, and —
//! when the search writes a plan — the provider bound and, for each other meeting it, the joint rules' first refusal
//! of the plan with it in the bound one's place, every other binding unchanged. The walk judges assignments in order,
//! the plan the first it accepts, so an other ordered before the bound one makes an assignment the walk judged and
//! refused — its refusal is the walk's own — and one ordered after it an assignment the walk never reached, judged
//! here by the same rules.
//!
//! **An adapter where nothing direct holds** (leaf `M3.2.3.2`, `docs/specs/adapters/decision_engine-adapters.md` §3).
//! The search runs over the hardware alone first; only when that concludes with no plan and some adapter's instance
//! meets some pair does it run again, each pair's candidates its hardware and then its instances, in the catalog's
//! order and the policy's, under what is left of the budget. An instance meets a pair where its kind's `beneath` holds
//! at a provider and the pair's fact has a value at the instance that meets every requirement of the pair; an
//! assignment choosing one binds what its kind requires beneath to that provider. A pair no provider and no instance
//! meets takes `SR-H18`'s code with the instances tried among those taking part: one whose `beneath` holds answers the
//! pair, one whose `beneath` fails only where its provider states some fact that side names.

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{Diagnostic, Form, Label, Span, Verdict};

use crate::adapter::{instance_name, Catalog, Instance};
use crate::joint::{
    is_level, is_qualifier, relies_on_a_function, requesters, Binding, Joint, Kind, Obligation,
    Report, Requester, System,
};
use crate::offer::{self, Provider};
use crate::relation::{judge, Verdict as Answer};
use crate::requirement::Requirement;
use crate::value::Value;
use crate::vocabulary::Vocabulary;

/// The engine's selection policy (§5.4: "explicit preferred-provider order with stable tie-breaking, stored in engine
/// configuration rather than eADL"): the providers it names first, in its order, then every other in the
/// description's declaration order; and the adapters it may bind where none of them will do. The default names none
/// and holds none.
#[derive(Debug, Clone, Default, PartialEq)]
pub struct Policy {
    /// The providers preferred, by name, most preferred first.
    pub prefer: Vec<String>,
    /// The engine's adapters the search may bind (record §3); the shipped catalog where the engine runs, none by default.
    pub adapters: Catalog,
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

impl std::fmt::Display for Who {
    /// As the assignment record writes a requester: a declaration by its name, a task `(task SYSTEM TASK)`.
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Named(name) => f.write_str(name),
            Self::Task { system, task } => write!(f, "(task {system} {task})"),
        }
    }
}

impl Who {
    /// `requester`, a requester `system` serves, as the record names it.
    fn of(requester: &Requester, system: &str) -> Self {
        match (requester.kind, crate::joint::task_of(&requester.name)) {
            (Kind::Task, Some((of, task))) if of == system => Self::Task {
                system: system.to_owned(),
                task: task.to_owned(),
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
    /// Each adapter instance the plan binds, by its name's order (record §3); none where the hardware alone holds.
    pub instances: Vec<Instance>,
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

/// One adapter kind tried over one provider for one pair (record §3.1, §3.4).
#[derive(Debug, Clone)]
struct Tried {
    /// The instance's name, as it would be bound.
    name: String,
    /// The kind.
    kind: String,
    /// The provider it programs, and where it is declared.
    over: (String, Span),
    /// Whether the kind's `beneath` holds there.
    beneath: bool,
    /// The instance's answer to the pair where its `beneath` holds; otherwise that side's answer at the provider.
    answer: Answer,
    /// The facts its `beneath` names that the provider leaves undescribed.
    undescribed: Vec<String>,
    /// The instance, where its `beneath` holds.
    instance: Option<Instance>,
}

/// One pair: a requester's requirements on one fact, and the providers meeting all of them.
struct Pair<'d> {
    requester: &'d Requester,
    fact: String,
    requirements: Vec<&'d Requirement>,
    candidates: Vec<String>,
}

/// Search `system`, a `defsystem` of `declarations` — one description's: a plain description, a module of one instance,
/// or a module tree read as the language checks it, its names qualified (`eadl_model::check::resolved_declarations`,
/// leaf `M3.4.10`) — for a plan whose callers are bound at `level`, the profile's.
#[must_use]
pub fn search(
    declarations: &[Form],
    system: &Form,
    level: &str,
    vocabulary: &Vocabulary,
    policy: &Policy,
    limits: Limits,
) -> Search {
    run(
        declarations,
        system,
        level,
        vocabulary,
        policy,
        limits,
        None,
    )
    .search
}

/// Which requirements of a fact [`explain`] gives an account of.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Select<'a> {
    /// Every requester of the system writing one.
    Every,
    /// A declaration's — a service, a policy, the system at its own level — by its name.
    Requester(&'a str),
    /// A task's of the system, by its name within it.
    Task(&'a str),
}

impl Select<'_> {
    /// Whether `who` is selected.
    fn admits(self, who: &Who) -> bool {
        match (self, who) {
            (Self::Every, _) => true,
            (Self::Requester(name), Who::Named(named)) => *named == name,
            (Self::Task(name), Who::Task { task, .. }) => *task == name,
            _ => false,
        }
    }
}

/// One requirement explained: a requester's requirements on one fact, as the search judged them (leaf `M3.4.13.1`).
#[derive(Debug, Clone)]
pub struct Account {
    /// Who requires it, as the plan names a requester.
    pub requester: Who,
    /// The fact.
    pub fact: String,
    /// Where each of its requirements is written.
    pub clauses: Vec<Span>,
    /// Each provider of the system's hardware, in the selection policy's order, with its answer: satisfied when it
    /// meets every requirement, otherwise the first of refused, absent, unsupported, unknown and undescribed among
    /// them, as a failed search ranks them.
    pub answers: Vec<(String, Answer)>,
    /// Why no provider meets it, when none does: the search's own diagnostic.
    pub unmet: Option<Diagnostic>,
    /// The provider the plan binds, when the search writes one.
    pub bound: Option<String>,
    /// With a plan, each other provider meeting it, in the policy's order, and the joint rules' first refusal of the
    /// plan with it in the bound one's place, every other binding unchanged — `None` where they accept it too, the
    /// policy's order having put the bound one first.
    pub instead: Vec<(String, Option<Diagnostic>)>,
}

/// What [`explain`] answers: the search, as [`search`] concludes it, and an account of each requirement selected.
#[derive(Debug, Clone)]
pub struct Explanation {
    /// The search's conclusion.
    pub search: Search,
    /// Whether the search reached its requirements; `false` when it stopped first — a refusal of what the description
    /// writes, or of the system's map — so that no requirement was judged.
    pub judged: bool,
    /// Every requirement the system's requesters write, as (requester, fact), in the search's order.
    pub required: Vec<(Who, String)>,
    /// An account of each requirement on the fact asked of the requesters selected, in the search's order.
    pub accounts: Vec<Account>,
}

/// What [`explain`] is asked: a fact, and whose requirements on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Asked<'a> {
    /// The fact.
    pub fact: &'a str,
    /// The requesters whose requirements on it are explained.
    pub select: Select<'a>,
}

/// Search `system` as [`search`] does, and give an account of each requirement `asked` names.
#[must_use]
pub fn explain(
    declarations: &[Form],
    system: &Form,
    level: &str,
    vocabulary: &Vocabulary,
    policy: &Policy,
    limits: Limits,
    asked: Asked<'_>,
) -> Explanation {
    run(
        declarations,
        system,
        level,
        vocabulary,
        policy,
        limits,
        Some(asked),
    )
}

/// The search, and with `asked` the account [`explain`] gives.
fn run(
    declarations: &[Form],
    system: &Form,
    level: &str,
    vocabulary: &Vocabulary,
    policy: &Policy,
    limits: Limits,
    asked: Option<Asked<'_>>,
) -> Explanation {
    let stopped = |search: Search| Explanation {
        search,
        judged: false,
        required: Vec::new(),
        accounts: Vec::new(),
    };
    let read = System::read(system, declarations);
    let read = match read.at(level, vocabulary) {
        Ok(read) => read,
        Err(why) => {
            return stopped(refused(vec![Diagnostic::error(
                "tool-failure",
                format!("the search was given a level it cannot bind callers at: {why}"),
                Label::new(system.span(), "the system searched"),
                "report it: the profile names a level the vocabulary does not have",
            )]))
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
        return stopped(refused(diagnostics));
    }
    let joint = match Joint::new(&providers, &all, vocabulary) {
        Ok(joint) => joint,
        Err(findings) => {
            return stopped(refused(findings.iter().map(|f| f.diagnostic()).collect()))
        }
    };
    let map = joint.map(&read);
    if !map.refused.is_empty() || !map.unknowns.is_empty() {
        return stopped(refused(
            map.refused
                .iter()
                .chain(&map.unknowns)
                .map(|f| f.diagnostic())
                .collect(),
        ));
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
    let required: Vec<(Who, String)> = pairs
        .iter()
        .map(|pair| (Who::of(pair.requester, &read.name), pair.fact.clone()))
        .collect();
    let asked: Vec<usize> = match asked {
        Some(asked) => (0..pairs.len())
            .filter(|&at| required[at].1 == asked.fact && asked.select.admits(&required[at].0))
            .collect(),
        None => Vec::new(),
    };
    // Every kind tried over every provider for every pair, once: the instances a second phase may bind, and the trials
    // a failed pair's code and an account read.
    let tried: Vec<Vec<Tried>> = pairs
        .iter()
        .map(|pair| trials(pair, &ordered, &policy.adapters, vocabulary))
        .collect();
    let meeting = |at: usize| -> Vec<&Tried> {
        tried[at]
            .iter()
            .filter(|t| t.beneath && t.answer == Answer::Satisfied)
            .collect()
    };
    let mut accounts: Vec<Account> = asked
        .iter()
        .map(|&at| {
            let pair = &pairs[at];
            Account {
                requester: required[at].0.clone(),
                fact: pair.fact.clone(),
                clauses: pair.requirements.iter().map(|r| r.span()).collect(),
                answers: ordered
                    .iter()
                    .map(|provider| {
                        (
                            provider.name.clone(),
                            answer(provider, &pair.requirements, vocabulary),
                        )
                    })
                    .chain(tried[at].iter().map(|t| (t.name.clone(), t.answer)))
                    .collect(),
                unmet: None,
                bound: None,
                instead: Vec::new(),
            }
        })
        .collect();
    let explained = |search: Search, accounts: Vec<Account>| Explanation {
        search,
        judged: true,
        required: required.clone(),
        accounts,
    };
    // Phase one: the hardware alone, as `M3.4.2` built it.
    let joint = joint.for_system(read.clone());
    let no_instances = BTreeMap::new();
    let (first, spent) = if pairs.iter().any(|pair| pair.candidates.is_empty()) {
        (None, 0)
    } else {
        let (search, spent) = walk(
            &joint,
            &read,
            &pairs,
            placement.clone(),
            limits,
            &no_instances,
        );
        (Some(search), spent)
    };
    let direct_holds = matches!(
        first,
        Some(Search::Resolved(_) | Search::Inconclusive { .. })
    );
    let any_instance = (0..pairs.len()).any(|at| !meeting(at).is_empty());
    // Phase two: each pair's hardware, then its instances, under what is left of the budget (record §3.2).
    let (search, final_pairs, instances) = if direct_holds || !any_instance {
        let unmet: Vec<Diagnostic> = (0..pairs.len())
            .filter(|&at| pairs[at].candidates.is_empty())
            .map(|at| unmet(&pairs[at], &tried[at], &read, &in_hardware, vocabulary))
            .collect();
        let search = match first {
            Some(search) => search,
            None => refused(unmet),
        };
        (search, pairs, BTreeMap::new())
    } else {
        let mut instances: BTreeMap<String, Instance> = BTreeMap::new();
        let widened: Vec<Pair<'_>> = pairs
            .iter()
            .enumerate()
            .map(|(at, pair)| {
                let mut candidates = pair.candidates.clone();
                for t in meeting(at) {
                    if let Some(instance) = &t.instance {
                        instances.insert(t.name.clone(), instance.clone());
                        candidates.push(t.name.clone());
                    }
                }
                Pair {
                    requester: pair.requester,
                    fact: pair.fact.clone(),
                    requirements: pair.requirements.clone(),
                    candidates,
                }
            })
            .collect();
        let unmet: Vec<Diagnostic> = (0..widened.len())
            .filter(|&at| widened[at].candidates.is_empty())
            .map(|at| unmet(&widened[at], &tried[at], &read, &in_hardware, vocabulary))
            .collect();
        let beneath = beneath_bindings(&instances, &policy.adapters);
        let search = if unmet.is_empty() {
            let all: Vec<Instance> = instances.values().cloned().collect();
            let joint = joint.clone().with_instances(&all);
            let left = Limits {
                steps: limits.steps.saturating_sub(spent),
            };
            match walk(&joint, &read, &widened, placement, left, &beneath).0 {
                Search::Resolved(mut resolution) => {
                    let bound: BTreeSet<&String> = resolution.bindings.values().collect();
                    resolution.instances = instances
                        .iter()
                        .filter(|(name, _)| bound.contains(name))
                        .map(|(_, instance)| instance.clone())
                        .collect();
                    Search::Resolved(resolution)
                }
                other => other,
            }
        } else {
            refused(unmet)
        };
        (search, widened, instances)
    };
    for (account, &at) in accounts.iter_mut().zip(&asked) {
        if final_pairs[at].candidates.is_empty() {
            account.unmet = Some(unmet(
                &final_pairs[at],
                &tried[at],
                &read,
                &in_hardware,
                vocabulary,
            ));
        }
    }
    if let Search::Resolved(resolution) = &search {
        let beneath = beneath_bindings(&instances, &policy.adapters);
        let all: Vec<Instance> = instances.values().cloned().collect();
        let joint = joint.clone().with_instances(&all);
        let chosen: Vec<&String> = final_pairs
            .iter()
            .zip(&required)
            .map(|(_, key)| &resolution.bindings[key])
            .collect();
        for (account, &at) in accounts.iter_mut().zip(&asked) {
            let bound = chosen[at];
            account.bound = Some(bound.clone());
            account.instead = final_pairs[at]
                .candidates
                .iter()
                .filter(|candidate| *candidate != bound)
                .map(|candidate| {
                    let mut providers = chosen.clone();
                    providers[at] = candidate;
                    let report = joint.check(&assignment(&final_pairs, &providers, &beneath));
                    let first = report.conflicts.iter().chain(&report.unknowns).next();
                    (candidate.clone(), first.map(|f| f.diagnostic()))
                })
                .collect();
        }
    }
    explained(search, accounts)
}

/// Each kind of `catalog`, in its order, tried over each provider of `ordered` for `pair` (record §3.1, §3.4): a kind
/// the mediation gate admits (`SR-H8`) that could offer the pair's fact — it offers it, or offers every fact it is
/// derived from — where its `beneath` holds, an instance and its answer to the pair; where it fails, only where the
/// provider states some fact that side names, with that side's answer there.
fn trials(
    pair: &Pair<'_>,
    ordered: &[&Provider],
    catalog: &Catalog,
    vocabulary: &Vocabulary,
) -> Vec<Tried> {
    let mediation = pair.requester.requirements.iter().any(|r| {
        matches!(r, Requirement::Statement { fact, value: Value::Enum(word), .. }
            if fact == "or-through-mediation" && word == "allowed")
    });
    let derivable = |kind: &crate::adapter::Kind| {
        kind.offer(&pair.fact).is_some()
            || vocabulary.fact(&pair.fact).is_some_and(|entry| {
                !entry.derived_from.is_empty()
                    && entry.derived_from.iter().all(|g| kind.offer(g).is_some())
            })
    };
    let mut out = Vec::new();
    for kind in catalog
        .kinds
        .iter()
        .filter(|kind| (!kind.mediates || mediation) && derivable(kind))
    {
        let beneath: Vec<&Requirement> = kind.beneath.iter().collect();
        let serves = kind.exclusive().then_some(pair.requester.name.as_str());
        for provider in ordered {
            let name = instance_name(&kind.name, &provider.name, serves);
            let side = answer(provider, &beneath, vocabulary);
            let undescribed: Vec<String> = kind
                .beneath_names
                .iter()
                .filter(|f| provider.state(f).is_none())
                .cloned()
                .collect();
            if side == Answer::Satisfied {
                let instance = kind.instance(
                    provider,
                    serves.map(|r| (r, pair.requester.requirements.as_slice())),
                    vocabulary,
                );
                out.push(Tried {
                    name,
                    kind: kind.name.clone(),
                    over: (provider.name.clone(), provider.span),
                    beneath: true,
                    answer: answer(&instance.provider, &pair.requirements, vocabulary),
                    undescribed,
                    instance: Some(instance),
                });
            } else if kind
                .beneath_names
                .iter()
                .any(|f| provider.state(f).is_some())
            {
                out.push(Tried {
                    name,
                    kind: kind.name.clone(),
                    over: (provider.name.clone(), provider.span),
                    beneath: false,
                    answer: side,
                    undescribed,
                    instance: None,
                });
            }
        }
    }
    out
}

/// Each instance's own bindings: what its kind requires beneath, bound to the provider it programs (record §3.3).
fn beneath_bindings(
    instances: &BTreeMap<String, Instance>,
    catalog: &Catalog,
) -> BTreeMap<String, Vec<Binding>> {
    instances
        .iter()
        .map(|(name, instance)| {
            let bindings = catalog
                .kind(&instance.kind)
                .map(|kind| {
                    kind.beneath
                        .iter()
                        .map(|requirement| Binding {
                            requester: name.clone(),
                            requirement: requirement.clone(),
                            provider: instance.over.clone(),
                        })
                        .collect()
                })
                .unwrap_or_default();
            (name.clone(), bindings)
        })
        .collect()
}

/// The bindings of `pairs`, each to the provider at its place in `providers`, and each instance chosen once, its own
/// bindings beneath.
fn assignment(
    pairs: &[Pair<'_>],
    providers: &[&String],
    beneath: &BTreeMap<String, Vec<Binding>>,
) -> Vec<Binding> {
    let mut out: Vec<Binding> = pairs
        .iter()
        .zip(providers)
        .flat_map(|(pair, provider)| {
            pair.requirements.iter().map(move |requirement| Binding {
                requester: pair.requester.name.clone(),
                requirement: (*requirement).clone(),
                provider: (*provider).clone(),
            })
        })
        .collect();
    let chosen: BTreeSet<&String> = providers.iter().copied().collect();
    for name in chosen {
        if let Some(own) = beneath.get(name) {
            out.extend(own.iter().cloned());
        }
    }
    out
}

/// `provider`'s answer to `requirements`, one requester's on one fact: satisfied when it meets every one, otherwise the
/// first of refused, absent, unsupported, unknown and undescribed among their answers, as record §3 rule 5 ranks a
/// group's — the one reading of a provider's answer, by which [`pairs`] finds candidates and [`unmet`] codes a
/// refusal, and which [`Account::answers`] reports.
fn answer(provider: &Provider, requirements: &[&Requirement], vocabulary: &Vocabulary) -> Answer {
    let rank = |answer: &Answer| match answer {
        Answer::Refused => 0,
        Answer::Absent => 1,
        Answer::Unsupported => 2,
        Answer::Unknown => 3,
        _ => 4,
    };
    requirements
        .iter()
        .map(|r| judge(provider, r, vocabulary))
        .filter(|a| !matches!(a, Answer::Satisfied | Answer::Statement))
        .min_by_key(rank)
        .unwrap_or(Answer::Satisfied)
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
        // A qualifier binds nothing where its requester relies on a function: the joint rules judge it at every
        // provider bound for one (model §8 rule 13, leaf `M4.1.8`). `reachable-at-privilege` binds nothing at all:
        // rule 9 judges it against the system's level (leaf `M4.1.9`).
        let relies = relies_on_a_function(&requester.name, all, vocabulary);
        for requirement in &requester.requirements {
            if matches!(requirement, Requirement::Statement { .. })
                || is_level(requirement)
                || (relies && is_qualifier(requirement, vocabulary))
            {
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
                .filter(|provider| answer(provider, &requirements, vocabulary) == Answer::Satisfied)
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
    tried: &[Tried],
    system: &System,
    hardware: &[&Provider],
    vocabulary: &Vocabulary,
) -> Diagnostic {
    // The providers that take part (`SR-H18`): those stating a fact the pair constrains — each requirement's, a group's
    // head and every sub-fact, and the inputs of their derivations — or answering it otherwise than undescribed; each
    // with its answer to the pair, the first of refused, absent, unsupported, unknown and undescribed, as rule 5 ranks
    // a group's (leaf `M3.4.9`, where a nested group's provider stating only a sub-fact was found left out).
    let mut facts = BTreeSet::new();
    for requirement in &pair.requirements {
        constrained(requirement, vocabulary, &mut facts);
    }
    let mut taking_part: Vec<(&Provider, Answer)> = Vec::new();
    for provider in hardware {
        // No provider of the hardware meets the pair, so each answer is one of the five a failed search ranks.
        let worst = answer(provider, &pair.requirements, vocabulary);
        let states = facts.iter().any(|fact| provider.state(fact).is_some());
        if worst != Answer::Undescribed || states {
            taking_part.push((provider, worst));
        }
    }
    // The instances tried take part on `SR-H18`'s own terms (record §3.4): `trials` keeps one whose `beneath` fails only
    // where its provider states part of it.
    let has = |answer: Answer| {
        taking_part.iter().any(|(_, a)| *a == answer) || tried.iter().any(|t| t.answer == answer)
    };
    let short: Vec<&Tried> = tried
        .iter()
        .filter(|t| {
            !t.beneath
                && matches!(t.answer, Answer::Undescribed | Answer::Unknown)
                && !t.undescribed.is_empty()
        })
        .collect();
    let (code, repair) = if has(Answer::Unsupported) {
        (
            "unsupported-profile",
            format!(
                "a provider's `{}` lies past the exact arithmetic the engine decides: write it within range",
                pair.fact
            ),
        )
    } else if let Some(t) = short.first() {
        (
            "missing-fact",
            format!(
                "the engine's adapter `{}` meets `{}` over `{}` once `{}` describes {}: describe {}, so the adapter \
                 can be judged",
                t.kind,
                pair.fact,
                t.over.0,
                t.over.0,
                t.undescribed
                    .iter()
                    .map(|f| format!("`{f}`"))
                    .collect::<Vec<_>>()
                    .join(", "),
                if t.undescribed.len() == 1 { "it" } else { "them" }
            ),
        )
    } else if has(Answer::Unknown) || has(Answer::Undescribed) {
        (
            "missing-fact",
            format!(
                "a provider states `{}`, or a fact it is read from, without what the requirement needs: describe it, \
                 so the requirement can be judged",
                pair.fact
            ),
        )
    } else if taking_part.is_empty() && tried.is_empty() {
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
    // F10 (`M3.4.5`): what was searched, so a refusal never reads as impossibility. The arithmetic's limit is the
    // engine's own and says so in its repair.
    let kinds: BTreeSet<&str> = tried.iter().map(|t| t.kind.as_str()).collect();
    let scope = if code == "unsupported-profile" {
        String::new()
    } else {
        format!(
            "; the search looked at the description's own blocks and platforms and at the engine's adapters — {} — \
             and the engine realizes a requirement no other way yet, catalog records being M3.7's, so this is missing \
             engine support, not proof that no realization exists",
            if kinds.is_empty() {
                "none of which offers it over this hardware".to_string()
            } else {
                format!(
                    "{} tried",
                    kinds
                        .iter()
                        .map(|k| format!("`{k}`"))
                        .collect::<Vec<_>>()
                        .join(", ")
                )
            }
        )
    };
    let mut diagnostic = Diagnostic::error(
        code,
        format!(
            "`{}` requires `{}`, and no provider of system `{}`'s hardware meets it{scope}",
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
            Answer::Undescribed => "states part of it and leaves the rest undescribed",
            Answer::Absent => "declares it absent",
            _ => "offers a value the requirement refuses",
        };
        diagnostic = diagnostic.with_secondary(Label::new(
            provider.span,
            format!("`{}` {said}", provider.name),
        ));
    }
    for t in tried {
        let said = if t.beneath {
            match t.answer {
                Answer::Unsupported => "offers it past the exact arithmetic",
                Answer::Unknown => "offers it without a value",
                Answer::Undescribed => "leaves part of it undescribed",
                Answer::Absent => "declares it absent",
                _ => "offers a value the requirement refuses",
            }
            .to_string()
        } else {
            let needs = match t.answer {
                Answer::Unsupported => "states what it needs past the exact arithmetic",
                Answer::Unknown => "states what it needs without a value",
                Answer::Undescribed => "leaves part of what it needs undescribed",
                Answer::Absent => "declares absent what it needs",
                _ => "offers a value it refuses",
            };
            format!("— `{}` {needs}", t.over.0)
        };
        diagnostic = diagnostic.with_secondary(Label::new(
            t.over.1,
            format!("the adapter `{}` over `{}` {said}", t.kind, t.over.0),
        ));
    }
    diagnostic
}

/// Every fact `requirement` constrains: its own, a group's sub-facts, and the facts each is derived from or reads.
fn constrained(requirement: &Requirement, vocabulary: &Vocabulary, facts: &mut BTreeSet<String>) {
    let fact = requirement.fact();
    if facts.insert(fact.to_owned()) {
        if let Some(entry) = vocabulary.fact(fact) {
            facts.extend(entry.derived_from.iter().chain(&entry.reads).cloned());
        }
    }
    if let Requirement::Group { parts, .. } = requirement {
        for part in parts {
            constrained(part, vocabulary, facts);
        }
    }
}

/// Judge the complete assignments in order — the first pair's candidate varying slowest — until one holds or the
/// budget is spent.
fn walk(
    joint: &Joint<'_>,
    system: &System,
    pairs: &[Pair<'_>],
    placement: Option<(String, String)>,
    limits: Limits,
    beneath: &BTreeMap<String, Vec<Binding>>,
) -> (Search, u64) {
    let mut choice = vec![0usize; pairs.len()];
    let mut steps = 0u64;
    let mut first: Option<Report> = None;
    let mut free: Option<Report> = None;
    loop {
        if steps == limits.steps {
            return (Search::Inconclusive {
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
            }, steps);
        }
        steps += 1;
        let providers: Vec<&String> = pairs
            .iter()
            .zip(&choice)
            .map(|(pair, &at)| &pair.candidates[at])
            .collect();
        let report = joint.check(&assignment(pairs, &providers, beneath));
        if report.holds() {
            return (
                Search::Resolved(Resolution {
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
                    // The instances bound are the caller's to name: it holds them, the walk their bindings beneath.
                    instances: Vec::new(),
                }),
                steps,
            );
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
                return (
                    refused(findings.iter().map(|f| f.diagnostic()).collect()),
                    steps,
                );
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
