//! `check` of a description whose systems are put on a target (leaf `M4.1.7.4.1`, API `1.10`): `DC-H3`'s judgement, with `DC-H4`'s
//! admission of the machine (`docs/decisions/decision_device-correspondence.md`, `docs/decisions/decision_runtime-plan-format-2.md` §1 and §2).
//!
//! ⭐ **A description need not pass presence over hardware of its own**: on a target the machine answers what its systems ask, so the
//! description's own hardware is one platform they can run on, and no more. What is judged, in this order, every stage asked whatever an
//! earlier one said, so that one request gives every answer it can:
//!
//! 1. **the target's file alone**: `check`'s judgement of it, each verdict where `check` places it; what keeps it from being a target,
//!    `Target::read`'s; what the search's readings refuse of what it writes, a compare read two ways among it; and what the profile asks of
//!    a machine, `Target::admission`'s: a count of sources no mask stops above zero `unsupported-profile`, either count silent `missing-fact`,
//!    no placement memory `infeasible-configuration`, and one active core, which `check`'s own admission holds;
//! 2. **every pass on the description itself but presence**, `eadl_model::check::check_but_presence`, a module tree's composition and the
//!    names that do not resolve in it among them;
//! 3. **each reading of the search over what the description writes, asked alone** (`search::refusals_of_what_is_written`), and a
//!    declaration named like a fact, as the engine's `prepare` asks it of a tree and of a plain description; never what binding says, a
//!    map's `missing-fact` among it, which is a plan's absence;
//! 4. **presence on each system put on the target**, `OnTarget::report`, before any search, what `on_target` refuses under the verdict
//!    `check` gives each code, `refinement-violated` as `infeasible-configuration`.
//!
//! The verdict is the highest of them by `check`'s precedence, and the closure is what presence read there, each system's together.
//! ⚠️ A description declaring no system puts nothing on the target, so presence judges nothing there and the closure is empty.

use std::collections::BTreeSet;

use eadl_front::{
    read, Diagnostic, Diagnostics, Form, Program, SourceId, SourceMap, Span, Verdict,
};
use eadl_model::check::{
    check_but_presence, check_program_but_presence, check_with_vocabulary, Closure, Outcome,
};
use eadl_model::kind::Registry;
use eadl_model::profile::Profile;
use eadl_resolve::plan;
use eadl_resolve::requirement::program_declarations_named_like_facts;
use eadl_resolve::search::refusals_of_what_is_written;
use eadl_resolve::target::{on_target, Target};
use eadl_resolve::vocabulary::Vocabulary;

use crate::{composition_verdict, facts_of, instances_of, Response, Status};

/// The description a request names, as [`judge`] is handed it.
pub(crate) enum Described<'p> {
    /// A plain description, its text the request's.
    Plain,
    /// A module tree, elaborated.
    Tree(&'p Program),
    /// A module tree that did not compose, with what elaboration refused of it: the passes cannot read it.
    Uncomposed(&'p [Diagnostic]),
}

/// What was found, each group with the verdict its stage gives it, in the order found.
#[derive(Default)]
struct Findings {
    groups: Vec<(Verdict, Vec<Diagnostic>)>,
    /// Every diagnostic held so far: one said twice, by two stages or for two systems, is said once.
    said: Vec<Diagnostic>,
}

impl Findings {
    /// `diagnostics`, each under `verdict`, but for one already said.
    fn push(&mut self, verdict: Verdict, diagnostics: impl IntoIterator<Item = Diagnostic>) {
        let new: Vec<Diagnostic> = diagnostics
            .into_iter()
            .filter(|diagnostic| !self.said.contains(diagnostic))
            .collect();
        if new.is_empty() {
            return;
        }
        self.said.extend(new.iter().cloned());
        self.groups.push((verdict, new));
    }

    /// Each of `diagnostics` under the verdict its own code carries.
    fn push_each(&mut self, diagnostics: impl IntoIterator<Item = Diagnostic>) {
        for diagnostic in diagnostics {
            self.push(Verdict::of_code(diagnostic.code), [diagnostic]);
        }
    }

    /// What a check concluded, its diagnostics under its own verdict, as `check` places them.
    fn push_outcome(&mut self, outcome: &Outcome) {
        self.push(outcome.verdict, outcome.diagnostics.iter().cloned());
    }

    /// The highest verdict found by `check`'s precedence, `ok` when nothing was, and every diagnostic in the order found.
    fn concluded(self) -> (Verdict, Vec<Diagnostic>) {
        let verdict = self
            .groups
            .iter()
            .map(|(verdict, _)| *verdict)
            .max_by_key(|verdict| verdict.precedence())
            .unwrap_or(Verdict::Ok);
        let diagnostics = Diagnostics::held(self.groups.into_iter().flat_map(|(_, found)| found));
        (
            if diagnostics.is_empty() {
                Verdict::Ok
            } else {
                verdict
            },
            diagnostics,
        )
    }
}

/// The forms of the source at `id` that are declarations, and whether it read.
fn declarations_of(sources: &SourceMap, id: SourceId) -> (Vec<Form>, bool) {
    let (document, diagnostics) = read(sources, id);
    let forms = eadl_front::language_version::declarations(&document.forms)
        .cloned()
        .collect();
    (forms, !diagnostics.has_errors())
}

/// The description at `id` judged with its systems on the target at `file`, as the module documentation orders it.
pub(crate) fn judge(
    sources: SourceMap,
    registry: &Registry,
    active: &'static Profile,
    id: SourceId,
    file: SourceId,
    described: Described<'_>,
) -> Response {
    let mut vocabulary_sources = SourceMap::new();
    let Ok(vocabulary) = Vocabulary::shipped(&mut vocabulary_sources, registry) else {
        return Response::not_judged(
            Status::ToolFailure,
            sources,
            vec!["the shipped vocabulary could not be read".to_owned()],
            None,
            Vec::new(),
        );
    };
    let mut findings = Findings::default();

    // 1. The target's file alone.
    findings.push_outcome(&check_with_vocabulary(
        &sources,
        file,
        registry,
        active,
        &facts_of(&vocabulary),
    ));
    let (target_forms, target_read) = declarations_of(&sources, file);
    let target = if target_read {
        findings.push_each(refusals_of_what_is_written(&target_forms, &vocabulary));
        match Target::read(&target_forms, Span::at(file, 0)) {
            Ok(target) => {
                findings.push_each(target.admission(&vocabulary));
                Some(target)
            }
            Err(refused) => {
                findings.push_each(refused);
                None
            }
        }
    } else {
        None
    };

    // 2. Every pass on the description itself but presence, and 3. the search's readings of what it writes, each asked alone.
    let plain;
    let (outcome, instances, declarations, program): (_, _, _, Option<&Program>) = match described {
        Described::Plain => {
            let outcome = check_but_presence(&sources, id, registry, active);
            let (declarations, readable) = declarations_of(&sources, id);
            plain = readable.then(|| plan::plain(declarations.clone()));
            (outcome, None, declarations, plain.as_ref())
        }
        Described::Uncomposed(refused) => {
            let outcome = Outcome {
                verdict: composition_verdict(refused),
                diagnostics: refused.to_vec(),
                declarations: Vec::new(),
                closure: Closure::default(),
            };
            (outcome, None, Vec::new(), None)
        }
        Described::Tree(program) => {
            let outcome = check_program_but_presence(program, registry, active);
            // One description, as the language checks it and the search reads it (`prepare`'s reading).
            let declarations = if program.instances.len() == 1 {
                program.root().declarations.clone()
            } else {
                eadl_model::check::resolved_declarations(program).0
            };
            (
                outcome,
                Some(instances_of(program)),
                declarations,
                Some(program),
            )
        }
    };
    findings.push_outcome(&outcome);
    if let Some(program) = program {
        findings.push_each(refusals_of_what_is_written(&declarations, &vocabulary));
        findings.push_each(
            program_declarations_named_like_facts(program, &vocabulary)
                .iter()
                .map(|refusal| refusal.diagnostic()),
        );
    }

    // 4. Presence on each system put on the target.
    let mut closure = Closure::default();
    if let (Some(target), Some(_)) = (&target, program) {
        let mut outside: Vec<String> = Vec::new();
        for system in declarations
            .iter()
            .filter(|form| form.head() == Some("defsystem"))
        {
            match on_target(system, &declarations, target) {
                Ok(there) => {
                    let report = there.report();
                    findings.push_each(report.diagnostics);
                    for pulled in report.pulled_in_by {
                        if !closure.inside.contains(&pulled) {
                            closure.inside.push(pulled);
                        }
                    }
                    for fact in report.outside_closure {
                        if !outside.contains(&fact) {
                            outside.push(fact);
                        }
                    }
                }
                Err(refused) => {
                    for diagnostic in refused {
                        let verdict = if diagnostic.code == "refinement-violated" {
                            Verdict::InfeasibleConfiguration
                        } else {
                            Verdict::of_code(diagnostic.code)
                        };
                        findings.push(verdict, [diagnostic]);
                    }
                }
            }
        }
        // What one system pulls in is inside the description's closure, whatever another leaves outside.
        let inside: BTreeSet<&str> = closure
            .inside
            .iter()
            .map(|(fact, _)| fact.as_str())
            .collect();
        outside.retain(|fact| !inside.contains(fact.as_str()));
        closure.outside = outside;
    }

    let (verdict, diagnostics) = findings.concluded();
    Response::judged(
        Outcome {
            verdict,
            diagnostics,
            declarations: outcome.declarations,
            closure,
        },
        instances,
        active,
        id,
        sources,
    )
}
