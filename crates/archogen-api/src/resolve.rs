//! `resolve` (leaf `M3.4.4`; `ROADMAP.md` §10.4, `docs/decisions/decision_engine-api.md` §6): one system of an
//! accepted description searched for a plan — the search alone, which a consumer asks with `unchecked` set.
//!
//! ⭐ **The programmatic surface runs nothing** (§10.4, the ruling of `2026-10-22`). The configuration checker is a
//! program of its own, and this API is the generator's, so it can run no checker: asked without `unchecked`, it
//! refuses with the reason and the repair — compose the plan with `archogen-plan-check`'s own tool — and asked with
//! it, it answers the plan the search wrote, which nothing has checked. `archogen resolve` asks it so, then runs the
//! checker over the plan as a process, which the command line alone may (`M3.5.1`'s design).
//!
//! The order: the request is checked as [`check_with`] checks it, and a description the check does not accept is
//! answered with the check's own response; its program is the one the check elaborated, or a plain description's
//! declarations as one root instance, read as one description as the language checks it — a module tree's names
//! qualified by §6 rules 9 and 10 (`eadl_model::check::resolved_declarations`); one system, the only one or the one
//! named; then the search
//! ([`eadl_resolve::search::search`]) under the profile's caller level, and the plan written as
//! `archogen-assignment/1` ([`eadl_resolve::plan`]).

use eadl_front::{Form, Program, SourceMap};
use eadl_model::check::{resolved_declarations, shipped_registry};
use eadl_model::profile::{self, Profile};
use eadl_resolve::plan;
use eadl_resolve::search::{search, Limits as Budget, Policy, Search};
use eadl_resolve::vocabulary::Vocabulary;

use crate::{checked, kind_modules, verdict_status, Limits, Request, Response, Status};

/// What `resolve` asks beyond `check`'s request.
#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Resolve<'a> {
    /// The system to resolve, when the description declares several.
    pub system: Option<&'a str>,
    /// The search alone, nothing checking the plan: the one way this API answers `resolve` (§10.4).
    pub unchecked: bool,
}

/// A plan the search wrote.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Plan {
    /// The system it is for.
    pub system: String,
    /// The plan, `archogen-assignment/1`'s canonical text.
    pub text: String,
    /// The digest of the elaborated program it names, 64 lowercase hexadecimal digits.
    pub program: String,
    /// How many bindings it holds.
    pub bindings: usize,
}

/// What `resolve` answers: the response — the check's, or why no plan was written — and the plan, when there is one.
#[derive(Debug)]
pub struct Resolution {
    /// `ok` with a plan; otherwise the check's refusal, the search's, or the request's.
    pub response: Response,
    /// The plan, present exactly when `response` is `ok`.
    pub plan: Option<Plan>,
}

/// Resolve `request`'s description within `limits`, as `resolve` asks.
#[must_use]
pub fn resolve_with(request: &Request<'_>, resolve: &Resolve<'_>, limits: Limits) -> Resolution {
    let refused = |response: Response| Resolution {
        response,
        plan: None,
    };
    if !resolve.unchecked {
        return refused(Response::not_judged(
            Status::Usage,
            SourceMap::new(),
            vec![
                "this API runs no checker, so it answers `resolve` as the search alone, asked with `unchecked` set"
                    .to_owned(),
            ],
            Some(
                "ask again with `unchecked`, and compose the plan with `archogen-plan-check`'s own tool to have it \
                 checked (ROADMAP.md §10.4)"
                    .to_owned(),
            ),
            Vec::new(),
        ));
    }
    let searched = prepare(request, resolve.system, limits, |ready| {
        let found = search(
            ready.declarations,
            ready.system,
            ready.active.caller_level,
            ready.vocabulary,
            &Policy::default(),
            Budget::DEFAULT,
        );
        concluded(request, ready, found)
    });
    match searched {
        Ok(resolution) => resolution,
        Err(response) => refused(*response),
    }
}

/// What [`resolve_with`] and `explain` search: the check's response, which accepted the description, and what it read.
pub(crate) struct Ready<'a> {
    /// The check's response, `ok`.
    pub(crate) response: Response,
    /// The program a plan's digest is taken of.
    pub(crate) program: &'a Program,
    /// One description's declarations, as the language checks it.
    pub(crate) declarations: &'a [Form],
    /// The system searched.
    pub(crate) system: &'a Form,
    /// The shipped vocabulary.
    pub(crate) vocabulary: &'a Vocabulary,
    /// The profile the description was judged under.
    pub(crate) active: &'static Profile,
}

/// Check `request` within `limits` and choose its system — the only one, or the one `system` names — then answer
/// `then` with what was read; a description the check does not accept, or a request naming no one system, is answered
/// by its response alone, boxed as a `Result`'s error is.
pub(crate) fn prepare<T>(
    request: &Request<'_>,
    system: Option<&str>,
    limits: Limits,
    then: impl FnOnce(Ready<'_>) -> T,
) -> Result<T, Box<Response>> {
    let mut elaborated = None;
    let response = checked(request, limits, &mut elaborated);
    let Some(judged) = response.judged.as_ref().filter(|_| response.is_ok()) else {
        return Err(Box::new(response));
    };
    let active = profile::supported(judged.profile)
        .expect("the description was judged under a profile this build supports");
    let program: Program = match elaborated {
        Some(program) => program,
        None => plan::plain(judged.declarations.clone()),
    };
    // One description, as the language checks it: a program of several instances flattened by §6 rules 9 and 10, its
    // names qualified as the plan names them (leaf `M3.4.10`); the check accepted it, so every operand resolved.
    let flattened;
    let declarations: &[Form] = if program.instances.len() == 1 {
        &program.root().declarations
    } else {
        flattened = resolved_declarations(&program).0;
        &flattened
    };
    let systems: Vec<&Form> = declarations
        .iter()
        .filter(|f| f.head() == Some("defsystem"))
        .collect();
    let listed = systems
        .iter()
        .map(|f| format!("`{}`", name_of(f)))
        .collect::<Vec<_>>()
        .join(", ");
    let usage = |sources: SourceMap, note: String, hint: String| {
        Err(Box::new(Response::not_judged(
            Status::Usage,
            sources,
            vec![note],
            Some(hint),
            Vec::new(),
        )))
    };
    let system = match (system, systems.as_slice()) {
        (Some(wanted), _) => match systems.iter().find(|f| name_of(f) == wanted) {
            Some(system) => *system,
            None => {
                return usage(
                    response.sources,
                    format!("{} declares no system `{wanted}`", request.name),
                    format!("name one it declares: {listed}"),
                )
            }
        },
        (None, [only]) => *only,
        (None, []) => {
            return usage(
                response.sources,
                format!(
                    "{} declares no system, so there is nothing to resolve",
                    request.name
                ),
                "a plan is one system's: declare one with `defsystem`".to_owned(),
            )
        }
        (None, _) => {
            return usage(
                response.sources,
                format!(
                    "{} declares several systems, and a plan is one system's",
                    request.name
                ),
                format!("choose one by name — `system` in the request, `--system` on the command line: {listed}"),
            )
        }
    };
    let mut vocabulary_sources = SourceMap::new();
    let vocabulary = shipped_registry(&mut vocabulary_sources, &kind_modules())
        .ok()
        .and_then(|registry| Vocabulary::shipped(&mut vocabulary_sources, &registry).ok());
    let Some(vocabulary) = vocabulary else {
        return Err(Box::new(Response::not_judged(
            Status::ToolFailure,
            response.sources,
            vec!["the shipped vocabulary could not be read".to_owned()],
            None,
            Vec::new(),
        )));
    };
    Ok(then(Ready {
        response,
        program: &program,
        declarations,
        system,
        vocabulary: &vocabulary,
        active,
    }))
}

/// A declaration's name.
pub(crate) fn name_of(form: &Form) -> String {
    form.items()
        .get(1)
        .and_then(Form::as_symbol)
        .unwrap_or_default()
        .to_owned()
}

/// What `resolve` answers for `found`, the search of `ready`'s system: the plan, or why none was written.
pub(crate) fn concluded(request: &Request<'_>, ready: Ready<'_>, found: Search) -> Resolution {
    let unresolved = |sources: SourceMap, status: Status, diagnostics| Resolution {
        response: Response::not_judged(
            status,
            sources,
            vec![format!(
                "system `{}` of {} cannot be resolved — no plan was written",
                name_of(ready.system),
                request.name
            )],
            None,
            diagnostics,
        ),
        plan: None,
    };
    match found {
        Search::Resolved(resolution) => {
            let digest = plan::program_digest(ready.program);
            let text = plan::write(&resolution, ready.active.id, &digest);
            Resolution {
                plan: Some(Plan {
                    system: resolution.system,
                    text,
                    program: digest,
                    bindings: resolution.bindings.len(),
                }),
                response: ready.response,
            }
        }
        Search::Refused {
            verdict,
            diagnostics,
        } => unresolved(ready.response.sources, verdict_status(verdict), diagnostics),
        Search::Inconclusive { diagnostic, .. } => unresolved(
            ready.response.sources,
            Status::AnalysisInconclusive,
            vec![diagnostic],
        ),
    }
}
