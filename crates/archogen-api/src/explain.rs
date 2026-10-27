//! `explain` (leaf `M3.4.13.2`; `ROADMAP.md` §10.2, §10.4): how one requirement of one system was realized, or why it
//! could not be — the search's own account (`eadl_resolve::search::explain`, leaf `M3.4.13.1`) of a description read
//! as [`resolve_with`](crate::resolve_with) reads it: the check, its program, one system, the search.
//!
//! A requirement is a requester's requirements on one fact, as the assignment record binds one provider per requester
//! and fact: [`Explain::fact`], narrowed to a declaration ([`Explain::requester`]) or a task ([`Explain::task`]). For
//! each selected, an [`Account`]: every provider of the system's hardware with its [`Answer`], the search's own
//! diagnostic when none meets it, and with a plan the provider bound and the joint rules' first refusal of each other
//! one in its place. A fact no requester selected requires is `usage`, its note listing those that are.
//!
//! ⭐ **It claims no check.** The plan it explains is the search's: this API runs no checker (§10.4), and an account
//! says what the search chose, not that the choice holds — `archogen resolve` runs the configuration checker over the
//! plan it writes. The response's status is the resolution's, as [`resolve_with`](crate::resolve_with) answers it with
//! `unchecked` set: `ok` with a plan, otherwise the check's verdict, the search's or the request's.

use eadl_resolve::relation::Verdict;
use eadl_resolve::search::{self, Asked, Limits as Budget, Select};

use crate::resolve::{concluded, name_of, prepare, Plan};
use crate::{Diagnostic, Limits, Request, Response, SourceMap, Span, Status};

/// What `explain` asks beyond `check`'s request.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Explain<'a> {
    /// The system, when the description declares several.
    pub system: Option<&'a str>,
    /// The fact whose requirements are explained.
    pub fact: &'a str,
    /// Only this declaration's requirements on it — a service's, a policy's, the system's own — by its name.
    pub requester: Option<&'a str>,
    /// Only this task's, by its name within the system.
    pub task: Option<&'a str>,
}

/// A provider's answer to a requirement: the relation's verdict (`docs/decisions/decision_substitutability-relation.md`
/// §3), the first of refused, absent, unsupported, unknown and undescribed among its requirements' when it does not meet
/// every one, as a failed search ranks them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Answer {
    /// It meets every requirement.
    Satisfied,
    /// It offers a value a requirement refuses.
    Refused,
    /// It declares the fact absent.
    Absent,
    /// Its value lies past the exact arithmetic the engine decides.
    Unsupported,
    /// It states the fact, or one it is read from, without a value.
    Unknown,
    /// It leaves the fact undescribed, or part of what the requirement needs.
    Undescribed,
}

impl Answer {
    /// Its name, as a machine reads it.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Satisfied => "satisfied",
            Self::Refused => "refused",
            Self::Absent => "absent",
            Self::Unsupported => "unsupported",
            Self::Unknown => "unknown",
            Self::Undescribed => "undescribed",
        }
    }

    /// What it says of the provider, as a person reads it.
    #[must_use]
    pub const fn says(self) -> &'static str {
        match self {
            Self::Satisfied => "meets it",
            Self::Refused => "offers a value it refuses",
            Self::Absent => "declares it absent",
            Self::Unsupported => "states it past the exact arithmetic",
            Self::Unknown => "states it without a value",
            Self::Undescribed => "leaves it undescribed",
        }
    }

    fn of(verdict: Verdict) -> Self {
        match verdict {
            Verdict::Satisfied => Self::Satisfied,
            Verdict::Refused => Self::Refused,
            Verdict::Absent => Self::Absent,
            Verdict::Unsupported => Self::Unsupported,
            Verdict::Unknown => Self::Unknown,
            // An account's pair holds no statement, which the search sets apart; undescribed is what is left.
            _ => Self::Undescribed,
        }
    }
}

/// One requirement explained: a requester's requirements on one fact, as the search judged them.
#[derive(Debug, Clone)]
pub struct Account {
    /// Who requires it, as the assignment record writes a requester: a declaration by its name, a task
    /// `(task SYSTEM TASK)`.
    pub requester: String,
    /// The fact.
    pub fact: String,
    /// Where each of its requirements is written, in [`Response::sources`].
    pub clauses: Vec<Span>,
    /// Each provider of the system's hardware, in the order the search tries them, with its answer.
    pub answers: Vec<(String, Answer)>,
    /// Why no provider meets it, when none does: the search's own diagnostic, which the response carries too.
    pub unmet: Option<Diagnostic>,
    /// The provider the plan binds, when the search writes one.
    pub bound: Option<String>,
    /// With a plan, each other provider meeting it and the joint rules' first conflict or unknown with it in the bound
    /// one's place, every other binding unchanged — `None` where they accept that assignment too, the order the search
    /// tries them in having put the bound one first.
    pub instead: Vec<(String, Option<Diagnostic>)>,
}

/// What `explain` answers: the response — the resolution's, or why nothing was explained — the plan the search wrote,
/// unchecked, and an account of each requirement selected.
#[derive(Debug)]
pub struct Explanation {
    /// `ok` with a plan; otherwise the check's refusal, the search's, or the request's.
    pub response: Response,
    /// The plan the search wrote, nothing having checked it; present exactly when `response` is `ok`.
    pub plan: Option<Plan>,
    /// An account of each requirement selected, in the search's order; empty when no requirement was judged — the
    /// check refused the description, the search stopped before its requirements, or the request selects none.
    pub accounts: Vec<Account>,
}

/// Explain `explain`'s requirements of `request`'s description within `limits`.
#[must_use]
pub fn explain_with(request: &Request<'_>, explain: &Explain<'_>, limits: Limits) -> Explanation {
    let unexplained = |response: Response| Explanation {
        response,
        plan: None,
        accounts: Vec::new(),
    };
    let usage = |sources: SourceMap, note: String, hint: String| {
        unexplained(Response::not_judged(
            Status::Usage,
            sources,
            vec![note],
            Some(hint),
            Vec::new(),
        ))
    };
    let select = match (explain.requester, explain.task) {
        (Some(requester), Some(task)) => {
            return usage(
                SourceMap::new(),
                format!("a requirement is one requester's, and the request names `{requester}` and the task `{task}`"),
                "name the requester or the task, not both".to_owned(),
            )
        }
        (Some(requester), None) => Select::Requester(requester),
        (None, Some(task)) => Select::Task(task),
        (None, None) => Select::Every,
    };
    let explained = prepare(request, explain.system, limits, |ready| {
        let found = search::explain(
            ready.declarations,
            ready.system,
            ready.active.caller_level,
            ready.vocabulary,
            ready.policy,
            Budget::DEFAULT,
            Asked {
                fact: explain.fact,
                select,
            },
        );
        if found.judged && found.accounts.is_empty() {
            let whose = match select {
                Select::Every => String::new(),
                Select::Requester(name) => format!(" by `{name}`"),
                Select::Task(name) => format!(" by its task `{name}`"),
            };
            let written = found
                .required
                .iter()
                .map(|(who, fact)| format!("`{fact}` by `{who}`"))
                .collect::<Vec<_>>()
                .join(", ");
            return usage(
                ready.response.sources,
                format!(
                    "system `{}` of {} writes no requirement on `{}`{whose}",
                    name_of(ready.system),
                    request.name,
                    explain.fact
                ),
                if written.is_empty() {
                    "its requesters write no requirement a provider meets, so there is nothing to explain".to_owned()
                } else {
                    format!("explain one it writes: {written}")
                },
            );
        }
        let accounts = found
            .accounts
            .into_iter()
            .map(|account| Account {
                requester: account.requester.to_string(),
                fact: account.fact,
                clauses: account.clauses,
                answers: account
                    .answers
                    .into_iter()
                    .map(|(provider, verdict)| (provider, Answer::of(verdict)))
                    .collect(),
                unmet: account.unmet,
                bound: account.bound,
                instead: account.instead,
            })
            .collect();
        let resolution = concluded(request, ready, found.search);
        Explanation {
            response: resolution.response,
            plan: resolution.plan,
            accounts,
        }
    });
    explained.unwrap_or_else(|response| unexplained(*response))
}
