//! The frontend pipeline: a description in, a `ROADMAP.md` §5.5 verdict out.
//!
//! This is what `archogen check` runs, and what the semantic corpus is driven through. It composes
//! the passes built one at a time by `M1.1`–`M1.7`, in the order in which a failure in one makes
//! the next meaningless:
//!
//! | Pass | Owns | Verdict on failure |
//! |---|---|---|
//! | read | syntax and spans (`M1.1`) | `invalid-description` |
//! | resolve | a module tree's names, `docs/semantics/reference.md` §6 rules 9–10 (`M1.29.3`) — [`check_program`] only | `invalid-description` |
//! | boundary | implementation content (`M0.3`, F27) | `invalid-description` |
//! | schema | the declaration frame (`M1.2`) | `invalid-description` |
//! | profile | the capabilities the profile refuses (`M0.4`) | `unsupported-profile` |
//! | workload | the task model the profile admits (`M1.9`) | `unsupported-profile`, `missing-fact`, `invalid-description` |
//! | presence | offered / absent / undescribed (`M1.5`) | `missing-fact`, `infeasible-configuration`, `invalid-description` |
//! | refinement | the three obligations, and the target they are owed to (`M1.6`, `M1.26.3`) | `infeasible-configuration`, `missing-fact`, `invalid-description` |
//!
//! ⛔ **The refinement row carries three verdicts and that is not sloppiness.** A violated obligation is
//! `infeasible-configuration`. A target the description does not declare is `missing-fact`: the contract the
//! obligations are owed to is described nowhere, so none of them can be checked (`docs/semantics/model.md` §3
//! rule 4; until `M1.26.3` it was `infeasible-configuration`, a code that then named two rules). A quantity the
//! pass could not *read* — `(tick-rate 0 MHz)` — is ill-typed input and is `invalid-description`, which §5.5
//! puts above both. One list of diagnostics would force one verdict over all of them, so `refinements` returns
//! the three groups separately.
//!
//! # It does not stop at the first pass
//!
//! Every pass runs, and every diagnostic is collected, up to what [`MOST_ERRORS`] holds. The **verdict** is the highest-precedence
//! one, where precedence means *what to fix first* rather than severity of consequence: a
//! malformed description makes every later answer meaningless, and an unsupported request is
//! reported before a missing fact because describing that fact would be wasted work on a system
//! the profile will refuse anyway.
//!
//! An author who has three problems gets three messages and one instruction about which to
//! start with.

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::language_version::{declarations, is_identifier};
use eadl_front::{
    read, Diagnostic, Diagnostics, Form, Instance, Label, Program, SourceId, SourceMap, Span,
    Verdict, MOST_ERRORS,
};

use crate::boundary;
use crate::kind::{duplicate_names, duplicate_task_names, read_kind, validate, Registry};
use crate::presence::{FactMap, NAME_CLAUSES};
use crate::profile::{self, Profile};
use crate::refinement::{self, Facets};
use crate::workload;

/// The dependency closure a description was judged over (§5.3), for the report an author reads.
///
/// ⭐ **Metadata, not a diagnostic** (leaf `M1.30`). §5.3 requires that unknown facts outside the closure
/// "remain visible in metadata and do not fail generation", while §4 rule 1 of
/// `docs/semantics/reference.md` states there is no warning and no note anywhere in the toolchain. Both
/// hold: this is what `archogen check` prints *after* its verdict, never a diagnostic, and it can fail
/// nothing. `docs/semantics/model.md` §2 rule 4 states the channel.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Closure {
    /// Each fact inside, with the fact that needs it — `None` for one the description requests itself.
    pub inside: Vec<(String, Option<String>)>,
    /// Each fact declared or needed that lies outside: nothing this description requests depends on it.
    pub outside: Vec<String>,
}

/// What a check concluded.
#[derive(Debug, Clone)]
pub struct Outcome {
    /// The §5.5 result.
    pub verdict: Verdict,
    /// Everything found, in pass order.
    pub diagnostics: Vec<Diagnostic>,
    /// The declarations that were read, whether or not the check passed.
    ///
    /// ⛔ **Declarations, and not top-level forms.** §8 of `docs/semantics/reference.md` makes the
    /// language-version identifier a statement about the *document*, so it is filtered out before this
    /// field is populated — `archogen check` prints this count and `docs/book/src/checking.md` publishes
    /// it, and a field that counted forms would print one declaration too many for every description
    /// that states its version.
    pub declarations: Vec<Form>,
    /// The closure boundary, filled once the presence pass has run; empty when a description never
    /// reached it.
    pub closure: Closure,
}

impl Outcome {
    /// Whether the description was accepted.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.verdict.is_ok()
    }

    /// Render every diagnostic.
    #[must_use]
    pub fn render(&self, sources: &SourceMap) -> String {
        self.diagnostics
            .iter()
            .map(|d| d.render(sources))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

/// A verdict paired with the diagnostics that justify it.
struct Finding {
    verdict: Verdict,
    diagnostics: Vec<Diagnostic>,
}

/// Build the registry from the shipped kind modules.
///
/// # Errors
///
/// Returns the diagnostics from a malformed or duplicated kind. A registry that could not be
/// built is a [`Verdict::ToolFailure`], not a statement about the user's description.
pub fn shipped_registry(
    sources: &mut SourceMap,
    kind_files: &[(String, String)],
) -> Result<Registry, Vec<Diagnostic>> {
    let mut registry = Registry::new();
    let mut errors = Vec::new();
    for (name, text) in kind_files {
        let Ok(id) = sources.add(name.clone(), text.clone()) else {
            errors.push(tool_failure(format!(
                "kind module `{name}` is too large to address"
            )));
            continue;
        };
        let (document, diagnostics) = read(sources, id);
        if diagnostics.has_errors() {
            // The reader's own diagnostics are surfaced rather than swallowed, so a *malformed*
            // identifier in a kind module is still reported with its §4 code — the filter below drops
            // well-formed ones, not problems with one.
            errors.extend(diagnostics.items().iter().cloned());
            continue;
        }
        // §8: a kind module may state its language version like any other description. ⛔ Measured,
        // not anticipated — without this the loader hands the identifier to `read_kind` and refuses the
        // whole registry as `schema-not-a-kind`, so a kind file that states its version cannot be read
        // at all. This is the third consumer that needed telling (`is_identifier`'s doc comment names
        // all three), and the one that no description-level test could reach.
        for form in declarations(&document.forms) {
            match read_kind(form) {
                Ok(kind) => {
                    if let Err(error) = registry.register(kind) {
                        errors.push(*error);
                    }
                }
                Err(mut found) => errors.append(&mut found),
            }
        }
    }
    if errors.is_empty() {
        Ok(registry)
    } else {
        Err(errors)
    }
}

fn tool_failure(message: String) -> Diagnostic {
    Diagnostic::error(
        "tool-failure",
        message,
        Label::new(
            eadl_front::Span::new(SourceId(0), 0, 0),
            "the toolchain could not proceed",
        ),
        "this is a failure of the toolchain, not a verdict about the description — §5.5 is \
         explicit that a tool failure is never reported as a valid system",
    )
}

/// Check one description.
///
/// `source` must already be in `sources`.
#[must_use]
pub fn check(
    sources: &SourceMap,
    source: SourceId,
    registry: &Registry,
    active_profile: &Profile,
) -> Outcome {
    // ── read ─────────────────────────────────────────────────────────────────────────────────
    let (document, read_diagnostics) = read(sources, source);
    // §8 of `docs/semantics/reference.md`: the language-version identifier is a statement about the
    // *document*, not a declaration, so **no pass below may treat it as one** and the field that names
    // them may not hold it. Filtered once here, by move, rather than once per pass — the third consumer
    // that lacked the rule was this crate's kind registry, and a per-pass filter is how a pass comes to
    // lack it. The reader has already checked the identifier itself (`language_version::state` runs
    // inside `read`), so dropping the form here loses no verdict.
    let forms: Vec<Form> = document
        .forms
        .into_iter()
        .filter(|form| !is_identifier(form))
        .collect();
    if read_diagnostics.has_errors() {
        return Outcome {
            verdict: Verdict::InvalidDescription,
            diagnostics: read_diagnostics.items().to_vec(),
            declarations: forms,
            closure: Closure::default(),
        };
    }
    passes(forms, registry, active_profile, Vec::new(), None)
}

/// What the capability vocabulary says that the check reads, handed down by a caller that has read it: the vocabulary is the search's to read, and this crate lies
/// below the one that does.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct VocabularyFacts {
    /// Each fact it declares, by name: one it does not declare is read on each system's own hardware (leaf `M3.4.16`).
    pub declared: BTreeSet<String>,
    /// Each fact it derives, with the facts its rule requires: one is described where one block or platform offers each of those (leaf `M3.4.17.1`).
    pub derived: BTreeMap<String, Vec<String>>,
}

/// [`check`], told each fact the capability vocabulary declares: a fact it does not declare is then read on each
/// system's own hardware (model §2; leaf `M3.4.16`). The vocabulary is the search's to read and this crate lies below
/// the search's, so what calls both hands the names down, and [`check`] alone reads every description whole.
#[must_use]
pub fn check_with_vocabulary(
    sources: &SourceMap,
    source: SourceId,
    registry: &Registry,
    active_profile: &Profile,
    vocabulary: &VocabularyFacts,
) -> Outcome {
    let (document, read_diagnostics) = read(sources, source);
    let forms: Vec<Form> = document
        .forms
        .into_iter()
        .filter(|form| !is_identifier(form))
        .collect();
    if read_diagnostics.has_errors() {
        return Outcome {
            verdict: Verdict::InvalidDescription,
            diagnostics: read_diagnostics.items().to_vec(),
            declarations: forms,
            closure: Closure::default(),
        };
    }
    passes(
        forms,
        registry,
        active_profile,
        Vec::new(),
        Some(vocabulary),
    )
}

/// Check an elaborated module tree, by `docs/semantics/reference.md` §6 rules 9 and 10.
///
/// Every declaration is renamed to the path of its instance, and every operand of a `uses`, `needs` or
/// `refines` clause is resolved in the scope of the instance that wrote it, through [`Program::resolve`]. The
/// declarations then go through **exactly** the passes [`check`] runs — one sequence, so a module tree is
/// judged by the same rules as a single description and cannot drift from it.
#[must_use]
pub fn check_program(program: &Program, registry: &Registry, active_profile: &Profile) -> Outcome {
    let (forms, unresolved) = resolved_declarations(program);
    let mut findings = Vec::new();
    push(&mut findings, Verdict::InvalidDescription, unresolved);
    passes(forms, registry, active_profile, findings, None)
}

/// [`check_program`], told each fact the capability vocabulary declares, as [`check_with_vocabulary`] is.
#[must_use]
pub fn check_program_with_vocabulary(
    program: &Program,
    registry: &Registry,
    active_profile: &Profile,
    vocabulary: &VocabularyFacts,
) -> Outcome {
    let (forms, unresolved) = resolved_declarations(program);
    let mut findings = Vec::new();
    push(&mut findings, Verdict::InvalidDescription, unresolved);
    passes(forms, registry, active_profile, findings, Some(vocabulary))
}

/// A module tree's declarations as one description, by `docs/semantics/reference.md` §6 rules 9 and 10: each renamed
/// to the path of its instance, and every operand of a `uses`, `needs` or `refines` clause resolved in the scope of the
/// instance that wrote it — what [`check_program`] judges, and what the search reads a program of several instances
/// as (leaf `M3.4.10`), so the two read one tree one way. Returns the declarations, and a diagnostic for each operand
/// that does not resolve.
#[must_use]
pub fn resolved_declarations(program: &Program) -> (Vec<Form>, Vec<Diagnostic>) {
    let mut unresolved = Vec::new();
    let forms: Vec<Form> = program
        .instances
        .iter()
        .flat_map(|instance| {
            instance
                .declarations
                .iter()
                .map(|declaration| {
                    resolve_declaration(program, instance, declaration, &mut unresolved)
                })
                .collect::<Vec<_>>()
        })
        .collect();
    (forms, unresolved)
}

/// Rule 9 for the declaration's own name, and rule 10 for every name it writes.
fn resolve_declaration(
    program: &Program,
    instance: &Instance,
    form: &Form,
    errors: &mut Vec<Diagnostic>,
) -> Form {
    let Form::List { items, span } = form else {
        return form.clone();
    };
    let items = items
        .iter()
        .enumerate()
        .map(|(index, item)| match (index, item) {
            (1, Form::Symbol { name, span }) => Form::Symbol {
                name: instance.qualify(name),
                span: *span,
            },
            (0 | 1, _) => item.clone(),
            _ => resolve_clause(program, instance.id, item, errors),
        })
        .collect();
    Form::List { items, span: *span }
}

/// Walk a clause; the operands of a name clause are resolved, anything else is walked for one.
fn resolve_clause(program: &Program, id: usize, form: &Form, errors: &mut Vec<Diagnostic>) -> Form {
    let Form::List { items, span } = form else {
        return form.clone();
    };
    let names = form.head().is_some_and(|head| NAME_CLAUSES.contains(&head));
    let items = items
        .iter()
        .enumerate()
        .map(|(index, item)| match (index, names) {
            (0, _) => item.clone(),
            (_, true) => resolve_operand(program, id, item, errors),
            (_, false) => resolve_clause(program, id, item, errors),
        })
        .collect();
    Form::List { items, span: *span }
}

/// A name operand is a symbol, or a list naming its head — the two shapes `presence.rs` reads as names.
fn resolve_operand(
    program: &Program,
    id: usize,
    form: &Form,
    errors: &mut Vec<Diagnostic>,
) -> Form {
    match form {
        Form::Symbol { name, span } => Form::Symbol {
            name: resolve_name(program, id, name, *span, errors),
            span: *span,
        },
        Form::List { items, span } => {
            let mut items = items.clone();
            if let Some(Form::Symbol { name, span: head }) = items.first() {
                items[0] = Form::Symbol {
                    name: resolve_name(program, id, name, *head, errors),
                    span: *head,
                };
            }
            Form::List { items, span: *span }
        }
        other => other.clone(),
    }
}

fn resolve_name(
    program: &Program,
    id: usize,
    name: &str,
    span: Span,
    errors: &mut Vec<Diagnostic>,
) -> String {
    match program.resolve(id, name, span) {
        Ok(Some(qualified)) => qualified,
        // A capability-vocabulary name, left as written for the presence pass to judge.
        Ok(None) => name.to_string(),
        Err(diagnostic) => {
            errors.push(*diagnostic);
            name.to_string()
        }
    }
}

/// Every pass after reading, in the order in which a failure in one makes the next meaningless. `check` and
/// `check_program` both end here; `findings` carries what the caller found before handing over.
fn passes(
    forms: Vec<Form>,
    registry: &Registry,
    active_profile: &Profile,
    mut findings: Vec<Finding>,
    vocabulary: Option<&VocabularyFacts>,
) -> Outcome {
    // ── boundary (F27) ───────────────────────────────────────────────────────────────────────
    let mut boundary_errors = Vec::new();
    for form in &forms {
        // A pass stops gathering where nothing more would be said and nothing it holds back could move the verdict, every error of it carrying one: a file of junk that reads as forms costs a host what each error costs. A pass whose errors carry different verdicts gathers them all, the highest deciding.
        if boundary_errors.len() > MOST_ERRORS {
            break;
        }
        if let Err(diagnostic) = boundary::check(form) {
            boundary_errors.push(*diagnostic);
        }
    }
    push(&mut findings, Verdict::InvalidDescription, boundary_errors);

    // ── schema ───────────────────────────────────────────────────────────────────────────────
    //
    // Skipped for a declaration the boundary already refused: "`implementation` is not a known
    // kind" adds nothing to a message that already said what it is and where it belongs.
    //
    // ⛔ The language-version identifier is not skipped here any more; it was filtered once above,
    // which is the same rule in one place instead of two. Without it the form was refused as
    // `schema-unknown-kind` — which it is, and correctly: it is not a kind.
    let declared: Vec<&Form> = forms
        .iter()
        .filter(|form| boundary::classify(form).is_accepted())
        .collect();
    let mut schema_errors = Vec::new();
    for form in &declared {
        if schema_errors.len() > MOST_ERRORS {
            break;
        }
        schema_errors.extend(validate(registry, form));
    }
    // §7 rule 6: a name is declared once — across declarations, so after the per-declaration frame.
    schema_errors.extend(duplicate_names(registry, &declared));
    schema_errors.extend(duplicate_task_names(&declared));
    push(&mut findings, Verdict::InvalidDescription, schema_errors);

    // ── profile admission ────────────────────────────────────────────────────────────────────
    let (admission_errors, out_of_profile) = admission(&forms, active_profile);
    push(&mut findings, Verdict::UnsupportedProfile, admission_errors);

    // ── workload admission (§3.1) ────────────────────────────────────────────────────────────
    //
    // The profile's admitted *task model*, as opposed to the capabilities it refuses. Runs after
    // exclusion admission and before presence: a task set the profile does not admit should be
    // reported before the author is sent to describe facts for it.
    for diagnostic in workload::check(&forms) {
        push(
            &mut findings,
            Verdict::of_code(diagnostic.code),
            vec![diagnostic],
        );
    }

    // ── presence and relevance ───────────────────────────────────────────────────────────────
    let mut facts = FactMap::new();
    for form in &forms {
        facts.collect(form);
    }
    // A fact the vocabulary derives is stated by no declaration, and the search computes it: presence is told the rule's facts, and reads it as described where one
    // block or platform offers each (leaf `M3.4.17.1`).
    for (fact, of) in vocabulary.iter().flat_map(|told| &told.derived) {
        facts.derive(fact, of);
    }
    let presence = facts.check_excluding(&out_of_profile);
    let closure = Closure {
        inside: presence.pulled_in_by.clone(),
        outside: presence.outside_closure.clone(),
    };
    // A fact the vocabulary does not declare is judged by presence alone, the search binding nothing for it, so it is
    // read as the search reads a system: on that system's own hardware (model §8 rule 7; leaf `M3.4.16`). A block or a
    // platform outside it neither asks nor answers there, and another system asks in its own reading. What the
    // description read whole says of such a fact gives way to that; a fact of the vocabulary is the search's to bind
    // or refuse, and is read as before. A description of no system is read whole, as every one is where the caller
    // names no vocabulary.
    let systems: Vec<&Form> = forms
        .iter()
        .filter(|form| form.head() == Some("defsystem"))
        .collect();
    // One reading is held at a time: a map names its declaration beside each fact, so two of a deep module tree's would
    // hold twice what one request may (`crates/archogen-api/tests/cost.rs`).
    drop(facts);
    let mut said: Vec<Diagnostic> = presence.diagnostics;
    if let Some(vocabulary) = vocabulary.filter(|_| !systems.is_empty()) {
        said.retain(|diagnostic| {
            !presence
                .judged
                .iter()
                .any(|(fact, of_it)| of_it == diagnostic && !vocabulary.declared.contains(fact))
        });
        for system in systems {
            let has = crate::presence::hardware(system, &forms);
            let mut on_its_hardware = FactMap::new();
            for form in &forms {
                let name = form.items().get(1).and_then(Form::as_symbol);
                if form.head() == Some("defsystem") && !std::ptr::eq(system, form) {
                    on_its_hardware.declare(form);
                } else if matches!(form.head(), Some("defblock" | "defplatform"))
                    && !name.is_some_and(|name| has.contains(name))
                {
                    on_its_hardware.collect_silent(form);
                } else {
                    on_its_hardware.collect(form);
                }
            }
            for (fact, diagnostic) in on_its_hardware.check_excluding(&out_of_profile).judged {
                if !vocabulary.declared.contains(&fact) && !said.contains(&diagnostic) {
                    said.push(diagnostic);
                }
            }
        }
    }
    for diagnostic in said {
        push(
            &mut findings,
            Verdict::of_code(diagnostic.code),
            vec![diagnostic],
        );
    }

    // ── refinement ───────────────────────────────────────────────────────────────────────────
    let found = refinements(&forms);
    for diagnostic in found.unreadable {
        push(
            &mut findings,
            Verdict::of_code(diagnostic.code),
            vec![diagnostic],
        );
    }
    push(
        &mut findings,
        Verdict::MissingFact,
        found.undeclared_targets,
    );
    push(
        &mut findings,
        Verdict::InfeasibleConfiguration,
        found.violated,
    );

    let verdict = findings
        .iter()
        .map(|finding| finding.verdict)
        .max_by_key(|verdict| verdict.precedence())
        .unwrap_or(Verdict::Ok);
    let diagnostics =
        Diagnostics::held(findings.into_iter().flat_map(|finding| finding.diagnostics));

    Outcome {
        verdict: if diagnostics.is_empty() {
            Verdict::Ok
        } else {
            verdict
        },
        diagnostics,
        declarations: forms,
        closure,
    }
}

fn push(findings: &mut Vec<Finding>, verdict: Verdict, diagnostics: Vec<Diagnostic>) {
    if !diagnostics.is_empty() {
        findings.push(Finding {
            verdict,
            diagnostics,
        });
    }
}

/// Refuse anything the active profile excludes.
///
/// §3.1: "A request for these features returns an unsupported-profile diagnostic **rather than
/// silently reducing the requested guarantee**." The refusal names the capability and the
/// obligation admitting it would add, so it reads as a statement about work rather than a wall.
/// What `forms` ask that `active` excludes, which presence does not say again: another reading of a description's presence uses this to say what `check` says.
#[must_use]
pub fn out_of_profile(forms: &[Form], active: &Profile) -> BTreeSet<String> {
    admission(forms, active).1
}

fn admission(forms: &[Form], active: &Profile) -> (Vec<Diagnostic>, BTreeSet<String>) {
    let mut errors = Vec::new();
    let mut refused: BTreeSet<String> = BTreeSet::new();
    for form in forms {
        walk(form, &mut |node| {
            let Some(name) = node_name(node) else { return };
            // More than one active core says what the word says, by a count: a platform's `core-count` above one, or a requirement of one.
            let name = if name == "core-count" && counts_more_than_one(node) {
                "multicore"
            } else {
                name
            };
            if let Some(exclusion) = active.exclusion(name) {
                refused.insert(exclusion.slug.to_string());
                errors.push(Diagnostic::error(
                    "unsupported-profile",
                    format!(
                        "`{}` is not admitted by profile `{}`",
                        exclusion.slug, active.id
                    ),
                    Label::new(
                        node.span(),
                        format!("{} is out of profile", exclusion.summary),
                    ),
                    format!(
                        "admitting it would add: {}. Request a profile that supports it, or \
                         remove the requirement — it is never silently reduced to a weaker \
                         guarantee",
                        exclusion.reason
                    ),
                ));
            }
        });
    }
    (errors, refused)
}

/// Whether a `core-count` clause states or asks more than one: `(core-count 4 tick)`, `(core-count (at-least 2 tick))`.
fn counts_more_than_one(clause: &Form) -> bool {
    let above_one =
        |form: Option<&Form>| matches!(form, Some(Form::Integer { value, .. }) if *value > 1);
    match clause.items().get(1) {
        Some(Form::List { items, .. }) => above_one(items.get(1)),
        count => above_one(count),
    }
}

/// The name a node contributes to admission: a bare symbol, or a list's head.
fn node_name(node: &Form) -> Option<&str> {
    match node {
        Form::Symbol { name, .. } => Some(name.as_str()),
        Form::List { .. } => node.head(),
        _ => None,
    }
}

fn walk(form: &Form, visit: &mut impl FnMut(&Form)) {
    visit(form);
    for item in form.items() {
        walk(item, visit);
    }
}

/// What the refinement pass found, in groups that carry different verdicts. See the pass table above.
struct Refinements {
    /// Quantities the pass could not read: `invalid-description`.
    unreadable: Vec<Diagnostic>,
    /// Targets the description does not declare: `missing-fact`.
    undeclared_targets: Vec<Diagnostic>,
    /// Obligations a concrete platform breaks: `infeasible-configuration`.
    violated: Vec<Diagnostic>,
}

/// Every block and platform `name` composes, at any composition depth, by [`crate::presence::composition`]'s reading —
/// a block's or a platform's every `uses` (model §8 rules 1 and 5).
fn composed(forms: &[Form], name: &str) -> Vec<String> {
    let hardware = |form: &&Form| matches!(form.head(), Some("defblock" | "defplatform"));
    let declared = |wanted: &str| {
        forms
            .iter()
            .filter(hardware)
            .find(|form| form.items().get(1).and_then(Form::as_symbol) == Some(wanted))
    };
    let mut seen: Vec<String> = Vec::new();
    let mut next: Vec<String> = declared(name)
        .map(crate::presence::composition)
        .unwrap_or_default();
    while let Some(part) = next.pop() {
        if part == name || seen.contains(&part) {
            continue;
        }
        if let Some(form) = declared(&part) {
            next.extend(crate::presence::composition(form));
            seen.push(part);
        }
    }
    seen
}

/// Check every `(refines …)` claim in the description.
fn refinements(forms: &[Form]) -> Refinements {
    let mut unreadable = Vec::new();
    let facets: Vec<Facets> = forms
        .iter()
        .map(|form| {
            let (facets, found) = Facets::of(form);
            unreadable.extend(found);
            facets
        })
        .collect();
    let mut undeclared_targets = Vec::new();
    let mut violated = Vec::new();
    for (index, concrete) in facets.iter().enumerate() {
        let Some((target, span)) = &concrete.refines else {
            continue;
        };
        let Some(abstract_) = facets
            .iter()
            .enumerate()
            .find(|(other, f)| *other != index && &f.name == target)
            .map(|(_, f)| f)
        else {
            undeclared_targets.push(Diagnostic::error(
                "missing-fact",
                format!(
                    "`{}` refines `{target}`, which is not declared here",
                    concrete.name
                ),
                Label::new(*span, "unknown refinement target"),
                "a refinement obligation cannot be checked against a description that is not \
                 present — import the module that declares it, or correct the name",
            ));
            continue;
        };
        // Every one of these is `infeasible-configuration`, so what is held back moves no verdict, and one offer refined a thousand ways multiplies.
        if violated.len() > MOST_ERRORS {
            break;
        }
        violated.extend(refinement::check(abstract_, concrete).diagnostics());
        // Model §3 rule 3 through the chain the abstract refines and what the concrete composes (leaf `M3.4.8`).
        let named = |name: &str| facets.iter().find(|f| f.name == name);
        let mut ancestors: Vec<&Facets> = Vec::new();
        let mut link = abstract_;
        while let Some(next) = link
            .refines
            .as_ref()
            .and_then(|(target, _)| named(target))
            .filter(|next| ancestors.len() < facets.len() && !std::ptr::eq(*next, concrete))
        {
            ancestors.push(next);
            link = next;
        }
        let composes: Vec<&Facets> = composed(forms, &concrete.name)
            .iter()
            .filter_map(|name| named(name))
            .collect();
        violated.extend(refinement::exclusions_through(
            abstract_, &ancestors, concrete, &composes,
        ));
    }
    Refinements {
        unreadable,
        undeclared_targets,
        violated,
    }
}

/// The default profile: the only one this build supports.
///
/// # Panics
///
/// Never: `rt-static-up-v1` is a `const` in [`crate::profile`].
#[must_use]
pub fn default_profile() -> &'static Profile {
    profile::supported("rt-static-up-v1").expect("the first profile is always supported")
}
