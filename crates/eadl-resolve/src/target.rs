//! A system on a target: which target device each of its devices is (`docs/decisions/decision_device-correspondence.md`; leaf
//! `M4.4.1`, `RTP-H8`).
//!
//! ⭐ **A system's devices are matched by what is asked of them, never by what they are called.** [`on_target`] gives a system the
//! target's platform in place of its own, and the search, the joint rules and the checker then read it as any system: every
//! requirement is judged again at the target's providers, so nothing here decides what satisfies what. [`correspond`] then reads off
//! the answer: a device of the description is the target device the same requester's same fact is bound to there.
//!
//! ⚠️ Two devices no requirement tells apart are not told apart here either: the policy's order chooses, as for any system.

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{Diagnostic, Form, Label, Span};
use eadl_model::refinement::Facets;

use crate::joint::System;
use crate::requirement::side_name_lists;
use crate::search::{Resolution, Who};

/// The name a declaration gives itself, if it is one.
fn declared_name(form: &Form) -> Option<&str> {
    form.items().get(1).and_then(Form::as_symbol)
}

/// Whether `form` declares hardware: a block or a platform.
fn is_hardware(form: &Form) -> bool {
    matches!(form.head(), Some("defblock" | "defplatform"))
}

/// The names `form`'s `uses` and `needs` lists hold, each with where it is written.
fn names_in(form: &Form) -> Vec<(&str, Span)> {
    side_name_lists(form)
        .into_iter()
        .flat_map(|list| list.items().iter().skip(1))
        .filter_map(|item| item.as_symbol().map(|name| (name, item.span())))
        .collect()
}

/// A target as the search reads it: its one platform, which composes every block and platform its file declares.
#[derive(Debug, Clone)]
pub struct Target {
    /// The platform a system on it uses.
    pub platform: String,
    /// Where that platform is declared.
    pub span: Span,
    /// What its file declares, hardware alone.
    pub declarations: Vec<Form>,
}

impl Target {
    /// Read a target from what its file holds. `file` is what a refusal names when no declaration can be: it is the file's start.
    ///
    /// # Errors
    ///
    /// Each thing that makes the file no target: a declaration of anything but hardware, no one platform that nothing else in the file
    /// names, or a block or platform that platform does not compose, which would be a device the machine does not have.
    pub fn read(forms: &[Form], file: Span) -> Result<Self, Vec<Diagnostic>> {
        let mut errors = Vec::new();
        let forms: Vec<&Form> = forms
            .iter()
            .filter(|form| form.head() != Some("eadl-version"))
            .collect();
        for form in forms.iter().filter(|form| !is_hardware(form)) {
            errors.push(Diagnostic::error(
                "invalid-description",
                format!(
                    "a target's file declares hardware alone, and this is a `{}`",
                    form.head().unwrap_or("form")
                ),
                Label::new(form.span(), "here"),
                "move it to the description: a target says what the machine is, never what runs on it",
            ));
        }
        let declared: BTreeMap<&str, &Form> = forms
            .iter()
            .filter(|form| is_hardware(form))
            .filter_map(|form| declared_name(form).map(|name| (name, *form)))
            .collect();
        let named: BTreeSet<&str> = declared
            .values()
            .flat_map(|form| names_in(form))
            .map(|(name, _)| name)
            .collect();
        let roots: Vec<(&str, &Form)> = declared
            .iter()
            .filter(|(name, form)| form.head() == Some("defplatform") && !named.contains(*name))
            .map(|(name, form)| (*name, *form))
            .collect();
        let [(platform, root)] = roots.as_slice() else {
            let found = roots
                .iter()
                .map(|(name, _)| format!("`{name}`"))
                .collect::<Vec<_>>()
                .join(", ");
            errors.push(Diagnostic::error(
                "invalid-description",
                if roots.is_empty() {
                    "a target's file declares one platform nothing else in it names, and this declares none".to_owned()
                } else {
                    format!(
                        "a target's file declares one platform nothing else in it names, and this declares {found}"
                    )
                },
                Label::new(roots.first().map_or(file, |(_, form)| form.span()), "here"),
                "compose the machine's devices under one platform, with `uses`",
            ));
            return Err(errors);
        };
        let mut composed: BTreeSet<&str> = BTreeSet::from([*platform]);
        let mut todo: Vec<&Form> = vec![*root];
        while let Some(at) = todo.pop() {
            for (name, _) in names_in(at) {
                if let Some(next) = declared.get(name) {
                    if composed.insert(name) {
                        todo.push(next);
                    }
                }
            }
        }
        for (name, form) in &declared {
            if !composed.contains(name) {
                errors.push(Diagnostic::error(
                    "invalid-description",
                    format!("`{name}` is declared by a target's file, and `{platform}` does not compose it"),
                    Label::new(form.span(), "declared here"),
                    format!("compose it into `{platform}`, or remove it: a target declares the devices the machine has"),
                ));
            }
        }
        if !errors.is_empty() {
            return Err(errors);
        }
        Ok(Self {
            platform: (*platform).to_owned(),
            span: root.span(),
            declarations: forms.into_iter().cloned().collect(),
        })
    }
}

/// One system put on a target, for the search to read as it reads any system.
#[derive(Debug, Clone)]
pub struct OnTarget {
    /// The system, its `platform` clause the target's.
    pub system: Form,
    /// The description's declarations with that system in its place, and the target's.
    pub declarations: Vec<Form>,
}

/// `system` with no `platform` clause.
fn without_platform(system: &Form) -> Vec<Form> {
    system
        .items()
        .iter()
        .filter(|item| item.head() != Some("platform"))
        .cloned()
        .collect()
}

/// A `(platform (uses PLATFORM))` clause, written at `span`.
fn platform_clause(platform: &str, span: Span) -> Form {
    let symbol = |name: &str| Form::Symbol {
        name: name.to_owned(),
        span,
    };
    Form::List {
        items: vec![
            symbol("platform"),
            Form::List {
                items: vec![symbol("uses"), symbol(platform)],
                span,
            },
        ],
        span,
    }
}

/// Every platform whose absences `hardware` owes: each it holds, and each up the chain each refines (model §3 rule 3).
fn platforms_with_their_chains<'d>(
    hardware: &BTreeSet<String>,
    declared: &BTreeMap<&str, &'d Form>,
) -> Vec<&'d Form> {
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut platforms = Vec::new();
    for name in hardware {
        let mut at = declared.get_key_value(name.as_str());
        while let Some((name, form)) = at {
            if form.head() != Some("defplatform") || !seen.insert(*name) {
                break;
            }
            platforms.push(*form);
            at = Facets::of(form)
                .0
                .refines
                .and_then(|(up, _)| declared.get_key_value(up.as_str()));
        }
    }
    platforms
}

/// `system`, one of `declarations`, on `target`.
///
/// # Errors
///
/// What keeps it from being read there, the search not having run: a name the target declares too; a block or platform the system or
/// a task names itself, which no requirement can stand for; and an absence a platform of the system declares that the target offers
/// (model §3 rule 3). A service or a policy naming hardware past the target's is `M1.49`'s, the search's own refusal.
pub fn on_target(
    system: &Form,
    declarations: &[Form],
    target: &Target,
) -> Result<OnTarget, Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let declared: BTreeMap<&str, &Form> = declarations
        .iter()
        .filter_map(|form| declared_name(form).map(|name| (name, form)))
        .collect();
    let theirs: BTreeMap<&str, &Form> = target
        .declarations
        .iter()
        .filter_map(|form| declared_name(form).map(|name| (name, form)))
        .collect();
    for (name, form) in &theirs {
        if let Some(own) = declared.get(name) {
            errors.push(
                Diagnostic::error(
                    "schema-duplicate-name",
                    format!("`{name}` is declared by the description and by the target"),
                    Label::new(own.span(), "declared here"),
                    "rename the description's: a name means one declaration",
                )
                .with_secondary(Label::new(form.span(), "and by the target here")),
            );
        }
    }
    let rest = Form::List {
        items: without_platform(system),
        span: system.span(),
    };
    let system_name = declared_name(system).unwrap_or("the system");
    for (name, span) in names_in(&rest) {
        if declared.get(name).is_some_and(|form| is_hardware(form)) {
            errors.push(Diagnostic::error(
                "infeasible-configuration",
                format!(
                    "`{system_name}` names `{name}` itself, so no requirement says which device of `{}` it is",
                    target.platform
                ),
                Label::new(span, "names it here"),
                "say what is asked of it instead, with `needs` and the facts: any device offering them then serves it",
            ));
        }
    }
    let own = System::read(system, declarations);
    let offered: Vec<Facets> = target
        .declarations
        .iter()
        .map(|form| Facets::of(form).0)
        .collect();
    for platform in platforms_with_their_chains(&own.hardware, &declared) {
        let facets = Facets::of(platform).0;
        for (fact, span) in facets.absences() {
            if let Some(offerer) = offered.iter().find(|target| target.offers_fact(fact)) {
                errors.push(Diagnostic::error(
                    "refinement-violated",
                    format!(
                        "`{}` declares `{fact}` absent, and `{}` of the target offers it",
                        facets.name, offerer.name
                    ),
                    Label::new(span, "declared absent here"),
                    "run it on a target without the fact, or drop the absence if nothing relies on it",
                ));
            }
        }
    }
    if !errors.is_empty() {
        return Err(errors);
    }
    let mut items = without_platform(system);
    items.push(platform_clause(&target.platform, target.span));
    let on_it = Form::List {
        items,
        span: system.span(),
    };
    let declarations = declarations
        .iter()
        .map(|form| {
            if declared_name(form) == declared_name(system) && form.head() == system.head() {
                on_it.clone()
            } else {
                form.clone()
            }
        })
        .chain(target.declarations.iter().cloned())
        .collect();
    Ok(OnTarget {
        system: on_it,
        declarations,
    })
}

/// A requirement, as a plan binds it.
pub type Requirement = (Who, String);

/// Which target device each device of a system is, read off two plans of it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Correspondence {
    /// Each device of the description a requirement is bound to, each target device that requirement is bound to on the target, and
    /// the requirements. One device may be several, and several one: the joint rules have judged what they share.
    pub devices: BTreeMap<String, BTreeMap<String, Vec<Requirement>>>,
    /// The system's hardware no requirement is bound to: what the target's plan has nothing to say of.
    pub unreached: BTreeSet<String>,
    /// Requirements bound in the description's plan alone: a device's own, or an adapter's, that the target's hardware does not ask.
    pub only_described: Vec<Requirement>,
    /// Requirements bound in the target's plan alone.
    pub only_on_target: Vec<Requirement>,
}

/// The device a plan's provider rests on: itself, or the one an adapter instance programs.
fn device_of<'r>(provider: &'r str, plan: &'r Resolution) -> &'r str {
    plan.instances
        .iter()
        .find(|instance| instance.provider.name == provider)
        .map_or(provider, |instance| instance.over.as_str())
}

/// Read which target device each device is: `own` a plan of the system over `hardware`, its own, and `there` a plan of it on a target.
#[must_use]
pub fn correspond(
    own: &Resolution,
    hardware: &BTreeSet<String>,
    there: &Resolution,
) -> Correspondence {
    let mut found = Correspondence {
        unreached: hardware.clone(),
        ..Correspondence::default()
    };
    for (requirement, provider) in &own.bindings {
        let device = device_of(provider, own);
        found.unreached.remove(device);
        match there.bindings.get(requirement) {
            Some(theirs) => found
                .devices
                .entry(device.to_owned())
                .or_default()
                .entry(device_of(theirs, there).to_owned())
                .or_default()
                .push(requirement.clone()),
            None => found.only_described.push(requirement.clone()),
        }
    }
    found.only_on_target = there
        .bindings
        .keys()
        .filter(|requirement| !own.bindings.contains_key(*requirement))
        .cloned()
        .collect();
    found
}
