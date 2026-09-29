//! The frontend pipeline: a description in, a `ROADMAP.md` §5.5 verdict out.
//!
//! This is what `archogen check` runs, and what the semantic corpus is driven through. It composes
//! the passes built one at a time by `M1.1`–`M1.7`, in the order in which a failure in one makes
//! the next meaningless:
//!
//! | Pass | Owns | Verdict on failure |
//! |---|---|---|
//! | read | syntax and spans (`M1.1`) | `invalid-description` |
//! | boundary | implementation content (`M0.3`, F27) | `invalid-description` |
//! | schema | the declaration frame (`M1.2`) | `invalid-description` |
//! | profile | the capabilities the profile refuses (`M0.4`) | `unsupported-profile` |
//! | workload | the task model the profile admits (`M1.9`) | `unsupported-profile`, `missing-fact`, `invalid-description` |
//! | presence | offered / absent / undescribed (`M1.5`) | `missing-fact`, `infeasible-configuration`, `invalid-description` |
//! | refinement | the three obligations (`M1.6`) | `infeasible-configuration` |
//!
//! # It does not stop at the first pass
//!
//! Every pass runs, and every diagnostic is collected. The **verdict** is the highest-precedence
//! one, where precedence means *what to fix first* rather than severity of consequence: a
//! malformed description makes every later answer meaningless, and an unsupported request is
//! reported before a missing fact because describing that fact would be wasted work on a system
//! the profile will refuse anyway.
//!
//! An author who has three problems gets three messages and one instruction about which to
//! start with.

use std::collections::BTreeSet;

use eadl_front::language_version::{declarations, is_identifier};
use eadl_front::{read, Diagnostic, Form, Label, SourceId, SourceMap, Verdict};

use crate::boundary;
use crate::kind::{read_kind, validate, Registry};
use crate::presence::FactMap;
use crate::profile::{self, Profile};
use crate::refinement::{self, Facets};
use crate::workload;

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
    let mut findings: Vec<Finding> = Vec::new();

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
        };
    }

    // ── boundary (F27) ───────────────────────────────────────────────────────────────────────
    let mut boundary_errors = Vec::new();
    for form in &forms {
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
    let mut schema_errors = Vec::new();
    for form in &forms {
        if boundary::classify(form).is_accepted() {
            schema_errors.extend(validate(registry, form));
        }
    }
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
        let verdict = Verdict::parse(diagnostic.code).unwrap_or(Verdict::InvalidDescription);
        push(&mut findings, verdict, vec![diagnostic]);
    }

    // ── presence and relevance ───────────────────────────────────────────────────────────────
    let mut facts = FactMap::new();
    for form in &forms {
        facts.collect(form);
    }
    let presence = facts.check_excluding(&out_of_profile);
    for diagnostic in presence.diagnostics {
        let verdict = Verdict::parse(diagnostic.code).unwrap_or(Verdict::InvalidDescription);
        push(&mut findings, verdict, vec![diagnostic]);
    }

    // ── refinement ───────────────────────────────────────────────────────────────────────────
    push(
        &mut findings,
        Verdict::InfeasibleConfiguration,
        refinements(&forms),
    );

    let verdict = findings
        .iter()
        .map(|finding| finding.verdict)
        .max_by_key(|verdict| verdict.precedence())
        .unwrap_or(Verdict::Ok);
    let diagnostics: Vec<Diagnostic> = findings
        .into_iter()
        .flat_map(|finding| finding.diagnostics)
        .collect();

    Outcome {
        verdict: if diagnostics.is_empty() {
            Verdict::Ok
        } else {
            verdict
        },
        diagnostics,
        declarations: forms,
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
fn admission(forms: &[Form], active: &Profile) -> (Vec<Diagnostic>, BTreeSet<String>) {
    let mut errors = Vec::new();
    let mut refused: BTreeSet<String> = BTreeSet::new();
    for form in forms {
        walk(form, &mut |node| {
            let Some(name) = node_name(node) else { return };
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

/// Check every `(refines …)` claim in the description.
fn refinements(forms: &[Form]) -> Vec<Diagnostic> {
    let facets: Vec<Facets> = forms.iter().map(Facets::of).collect();
    let mut errors = Vec::new();
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
            errors.push(Diagnostic::error(
                "infeasible-configuration",
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
        errors.extend(refinement::check(abstract_, concrete).diagnostics());
    }
    errors
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
