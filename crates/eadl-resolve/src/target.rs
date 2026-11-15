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
use eadl_model::check::{default_profile, out_of_profile};
use eadl_model::presence::{composition, FactMap, PresenceReport};
use eadl_model::refinement::{loops_refused, Facets};

use crate::joint::{Joint, System};
use crate::offer::{self, State};
use crate::requirement::side_name_lists;
use crate::search::{Resolution, Who};
use crate::value::Value;
use crate::vocabulary::Vocabulary;

/// The counts of sources no mask stops that the profile asks of every target (`docs/decisions/decision_runtime-plan-format-2.md` §2, R4-D15): each
/// with what it is, for a refusal to say.
const SOURCES_NO_MASK_STOPS: [(&str, &str); 2] = [
    ("non-maskable-interrupts", "interrupt sources no mask stops"),
    ("firmware-traps", "traps firmware beneath the image takes"),
];

/// How many cores a machine has, which the profile's one active core reads (`docs/decisions/decision_runtime-plan-format-2.md` §1).
const CORE_COUNT: &str = "core-count";

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
    /// Each thing that makes the file no target: a declaration of anything but hardware; one with no name, or a name two declarations
    /// bear; no one platform that nothing else in the file composes; or a block or platform that platform does not compose, which would be a
    /// device the machine does not have; hardware that refines what leads back to it, on which no machine stands; or a `uses` holding a list.
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
        let mut declared: BTreeMap<&str, &Form> = BTreeMap::new();
        for form in forms.iter().filter(|form| is_hardware(form)) {
            let Some(name) = declared_name(form) else {
                errors.push(Diagnostic::error(
                    "schema-missing-name",
                    "a target's file declares devices, and this one has no name",
                    Label::new(form.span(), "here"),
                    "name it, and compose it under the one platform with `uses`",
                ));
                continue;
            };
            if let Some(first) = declared.insert(name, form) {
                errors.push(
                    Diagnostic::error(
                        "schema-duplicate-name",
                        format!("`{name}` is declared twice by a target's file"),
                        Label::new(form.span(), "and here"),
                        "a name means one declaration, and one device judged as two would be neither: rename one or remove it",
                    )
                    .with_secondary(Label::new(first.span(), "declared here")),
                );
            }
        }
        let named: BTreeSet<String> = declared
            .values()
            .flat_map(|form| composition(form))
            .collect();
        let roots: Vec<(&str, &Form)> = declared
            .iter()
            .filter(|(name, form)| form.head() == Some("defplatform") && !named.contains(**name))
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
                    "a target's file declares one platform nothing else in it composes, and this declares none".to_owned()
                } else {
                    format!(
                        "a target's file declares one platform nothing else in it composes, and this declares {found}"
                    )
                },
                Label::new(roots.first().map_or(file, |(_, form)| form.span()), "here"),
                "compose the machine's devices under one platform, with `uses`",
            ));
            return Err(errors);
        };
        let mut composed: BTreeSet<String> = BTreeSet::from([(*platform).to_owned()]);
        let mut todo: Vec<&Form> = vec![*root];
        while let Some(at) = todo.pop() {
            for name in composition(at) {
                if let Some(next) = declared.get(name.as_str()) {
                    if composed.insert(name) {
                        todo.push(next);
                    }
                }
            }
        }
        errors.extend(loops_refused(&composed, &declared));
        // What a system reads as hardware is a plain name, and what this composes is read as more: one set, or every system on it is refused by the search, far from where this is written.
        for form in declared.values() {
            for span in uses_a_list(form) {
                errors.push(Diagnostic::error(
                    "invalid-description",
                    "a target composes its devices by plain names, and this `uses` or `needs` holds a list".to_owned(),
                    Label::new(span, "here"),
                    "write the name alone: a system on this machine reads a plain name as its hardware, and nothing else",
                ));
            }
        }
        for (name, form) in &declared {
            if !composed.contains(*name) {
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

    /// What `rt-static-up-v1`, the one profile, asks of a machine beyond what `check` judges of the target's file
    /// (`docs/decisions/decision_device-correspondence.md`, `DC-H4`; `docs/decisions/decision_runtime-plan-format-2.md` §1 and §2): no source no mask
    /// stops, since the variant's condition 5 masks every interrupt in a masked section, so a count of non-maskable interrupts or of firmware traps above
    /// zero is `unsupported-profile`, naming it, and one no device states as a count is `missing-fact`, since silence gives no value and a machine with
    /// such a source would otherwise be admitted; one active core, which reads the machine's `core-count`, so a count no device states is `missing-fact`
    /// for the same reason, a count above one being `check`'s own refusal (leaf `M4.1.7.4.3`); and memory to place an image in, since no playground
    /// provides it there, so a target with no placement memory is `infeasible-configuration`.
    ///
    /// Read off every device of the file, since the platform composes each: a count stated above zero anywhere is a source on the machine. What a
    /// device's offers or regions refuse is the readings', which a check of the file asks (`search::refusals_of_what_is_written`): a device that does not
    /// read states nothing here, and with a region that does not read, whether any is placement memory is not judged.
    #[must_use]
    pub fn admission(&self, vocabulary: &Vocabulary) -> Vec<Diagnostic> {
        let read: Vec<(offer::Provider, &Form)> = self
            .declarations
            .iter()
            .filter_map(|form| {
                offer::read(form, vocabulary)
                    .ok()
                    .map(|provider| (provider, form))
            })
            .collect();
        // Each device stating `fact` as a count, with the count and where it is stated.
        let counts = |fact: &str| -> Vec<(&str, i128, Span)> {
            read.iter()
                .filter_map(|(provider, _)| {
                    let statement = provider.facts.get(fact)?;
                    let State::Valued(values) = &statement.state else {
                        return None;
                    };
                    let Some(Value::Count(count)) = values.first() else {
                        return None;
                    };
                    let at = statement.spans.first().copied().unwrap_or(provider.span);
                    Some((provider.name.as_str(), *count, at))
                })
                .collect()
        };
        let mut refused = Vec::new();
        for (fact, what) in SOURCES_NO_MASK_STOPS {
            let stated = counts(fact);
            for (device, count, at) in stated.iter().filter(|(_, count, _)| *count > 0) {
                refused.push(Diagnostic::error(
                    "unsupported-profile",
                    format!("`{device}` states `({fact} {count})`, {what}, and profile `rt-static-up-v1` admits none"),
                    Label::new(*at, "stated here"),
                    "the profile masks every interrupt in a masked section, and a source no mask stops breaks that: run on a machine with none, \
                     or wait for a profile whose analysis charges one",
                ));
            }
            if stated.is_empty() {
                refused.push(Diagnostic::error(
                    "missing-fact",
                    format!("no device of `{}` states a count of `{fact}`, {what}", self.platform),
                    Label::new(self.span, "the machine"),
                    format!(
                        "state `({fact} N)` on `{}` with what checks it: silence gives no value, and the profile admits a machine only with none",
                        self.platform
                    ),
                ));
            }
        }
        if counts(CORE_COUNT).is_empty() {
            refused.push(Diagnostic::error(
                "missing-fact",
                format!("no device of `{}` states `{CORE_COUNT}`, how many cores it has", self.platform),
                Label::new(self.span, "the machine"),
                format!(
                    "state `({CORE_COUNT} N)` on `{}` with what checks it: silence gives no value, and profile `rt-static-up-v1` admits a machine \
                     of one active core only",
                    self.platform
                ),
            ));
        }
        if let Ok(joint) = Joint::new(&read, &[], vocabulary) {
            if joint.placement_memory().is_empty() {
                refused.push(Diagnostic::error(
                    "infeasible-configuration",
                    format!("`{}` offers no placement memory, so no image can be laid out on it", self.platform),
                    Label::new(self.span, "the machine"),
                    "offer the RAM an image starts in as a region with `(executable true)` and not reserved: on a target no playground provides \
                     memory",
                ));
            }
        }
        refused
    }
}

/// One system put on a target, for the search to read as it reads any system.
#[derive(Debug, Clone)]
pub struct OnTarget {
    /// The system, its `platform` clause the target's.
    pub system: Form,
    /// What the system reads there: the description's declarations with that system in its place and its hardware gone, and the target's. Another
    /// system is gone too, but for one it names, which `M1.46` is to refuse. So nothing the target replaces can answer for it.
    pub declarations: Vec<Form>,
    /// Those of `declarations` that are the description's and ask anything of the machine: the system, and what it reaches, another system among them.
    pub asking: BTreeSet<String>,
    /// The hardware of the description, gone from `declarations`: a name a kept declaration may still say, which is no fact.
    pub replaced: BTreeSet<String>,
}

impl OnTarget {
    /// What presence says of the system there (model §2 and §7 rule 1), as `archogen check` says it of any description, of what asks and of what can answer: a need on a fact the vocabulary
    /// does not declare is judged by nothing else, the search binding nothing for it. What offers is the machine, since the search binds a block, a platform or an adapter over one and nothing
    /// else, so a service's offer answers nothing here, and the plan it would let come is for a machine without the device. What the system does not reach neither asks nor answers, and its name is
    /// still a declaration's. The command that names a target asks this before the search (`DC-H3`).
    #[must_use]
    pub fn report(&self) -> PresenceReport {
        let mut facts = FactMap::new();
        for name in &self.replaced {
            facts.declare_name(name);
        }
        for form in &self.declarations {
            if is_hardware(form) {
                facts.collect(form);
            } else if form.head() == Some("defsystem")
                && declared_name(form) != declared_name(&self.system)
            {
                // Another system asks at its own level, as the system reading it reaches it: its tasks are that system's (model §8 rule 4), and only visible here.
                facts.declare(form);
                if declared_name(form).is_some_and(|name| self.asking.contains(name)) {
                    facts.collect_asking(&own_level(form));
                }
            } else if declared_name(form).is_some_and(|name| self.asking.contains(name)) {
                facts.collect_asking(form);
            } else {
                facts.declare(form);
            }
        }
        facts.check_excluding(&out_of_profile(&self.declarations, default_profile()))
    }

    /// What [`report`](Self::report) refuses.
    #[must_use]
    pub fn presence(&self) -> Vec<Diagnostic> {
        self.report().diagnostics
    }
}

/// Where `form`, composing, holds a list among what it `uses` or `needs`: `composition` walks into either, and a system's reading does not.
fn uses_a_list(form: &Form) -> Vec<Span> {
    fn walk(form: &Form, out: &mut Vec<Span>) {
        match form.head() {
            Some("offers" | "absent" | "refines") => {}
            Some("uses" | "needs") => out.extend(
                form.items()
                    .iter()
                    .skip(1)
                    .filter(|item| matches!(item, Form::List { .. }))
                    .map(Form::span),
            ),
            _ => {
                for item in form.items().iter().skip(1) {
                    if matches!(item, Form::List { .. }) {
                        walk(item, out);
                    }
                }
            }
        }
    }
    let mut out = Vec::new();
    for item in form.items().iter().skip(2) {
        if matches!(item, Form::List { .. }) {
            walk(item, &mut out);
        }
    }
    out
}

/// A system with its tasks taken out.
fn own_level(system: &Form) -> Form {
    Form::List {
        items: system
            .items()
            .iter()
            .filter(|item| item.head() != Some("task"))
            .cloned()
            .collect(),
        span: system.span(),
    }
}

/// `system`'s `platform` clause as the target has it: `platform` in place of the hardware its own `uses` names, and all else it holds kept, a
/// `platform` clause being the system's own side (model §7 rule 4). Written at `span`.
fn clause_on_target(
    system: &Form,
    declared: &BTreeMap<&str, &Form>,
    platform: &str,
    span: Span,
) -> Form {
    let symbol = |name: &str| Form::Symbol {
        name: name.to_owned(),
        span,
    };
    let mut items = vec![
        symbol("platform"),
        Form::List {
            items: vec![symbol("uses"), symbol(platform)],
            span,
        },
    ];
    let clauses = system
        .items()
        .iter()
        .filter(|item| item.head() == Some("platform"));
    for item in clauses.flat_map(|clause| clause.items().iter().skip(1)) {
        if item.head() != Some("uses") {
            items.push(item.clone());
            continue;
        }
        let kept: Vec<Form> = item
            .items()
            .iter()
            .skip(1)
            .filter(|name| {
                !name
                    .as_symbol()
                    .and_then(|name| declared.get(name))
                    .is_some_and(|form| is_hardware(form))
            })
            .cloned()
            .collect();
        if !kept.is_empty() {
            items.push(Form::List {
                items: std::iter::once(symbol("uses")).chain(kept).collect(),
                span: item.span(),
            });
        }
    }
    Form::List { items, span }
}

/// The hardware `system`'s `platform` clause itself names by `uses`.
fn named_by_clause<'s>(system: &'s Form, declared: &BTreeMap<&str, &Form>) -> Vec<&'s str> {
    system
        .items()
        .iter()
        .filter(|item| item.head() == Some("platform"))
        .flat_map(|clause| clause.items().iter().skip(1))
        .filter(|item| item.head() == Some("uses"))
        .flat_map(|uses| uses.items().iter().skip(1))
        .filter_map(Form::as_symbol)
        .filter(|name| declared.get(name).is_some_and(|form| is_hardware(form)))
        .collect()
}

/// Whether `part` is of `whole`: what it composes, as model §2 rule 1 reads composition, at any depth, or the more abstract saying of one of those that it refines (§3 rule 3). That says the same
/// thing again and brings no blocks of its own (§8 rule 7), so what the refined one composes is not of it.
fn holds(whole: &str, part: &str, declared: &BTreeMap<&str, &Form>) -> bool {
    let of = |name: &str| declared.get(name).copied().filter(|form| is_hardware(form));
    let mut made_of: BTreeSet<String> = BTreeSet::from([whole.to_owned()]);
    let mut todo = vec![whole.to_owned()];
    while let Some(at) = todo.pop() {
        for name in of(&at).map(composition).unwrap_or_default() {
            if name == part {
                return true;
            }
            if made_of.insert(name.clone()) {
                todo.push(name);
            }
        }
    }
    let mut seen = made_of.clone();
    let mut todo: Vec<String> = made_of.into_iter().collect();
    while let Some(at) = todo.pop() {
        if let Some((up, _)) = of(&at).and_then(|form| Facets::of(form).0.refines) {
            if up == part {
                return true;
            }
            if seen.insert(up.clone()) {
                todo.push(up);
            }
        }
    }
    false
}

/// What the machine is made of, as the clause says it: each piece of hardware it names, once, but for one that is of another of them. In a cycle each is of the
/// other, and there is no whole: one stands for all of it, each of whose absences then speaks, so no name moves the verdict.
fn what_the_machine_is<'s>(named: &[&'s str], declared: &BTreeMap<&str, &Form>) -> Vec<&'s str> {
    let distinct: BTreeSet<&'s str> = named.iter().copied().collect();
    distinct
        .iter()
        .copied()
        .filter(|name| {
            !distinct.iter().any(|other| {
                other != name
                    && holds(other, name, declared)
                    && (other < name || !holds(name, other, declared))
            })
        })
        .collect()
}

/// The declarations whose absences the whole machine owes: the one all the system's hardware is made of, as the clause says it, with what forms a cycle with it, and each up the chain it
/// refines (model §3 rule 3). Another's absence covers what it is made of and no more (model §2 rule 1), so it says nothing of a device beside it, and neither does one's among two that make the
/// machine, nor one's beside hardware the system has that it does not compose, a block it only `needs`.
fn declarations_speaking_for_the_machine<'d>(
    named: &[&str],
    hardware: &BTreeSet<String>,
    declared: &BTreeMap<&str, &'d Form>,
) -> Vec<&'d Form> {
    let [only] = what_the_machine_is(named, declared)[..] else {
        return Vec::new();
    };
    if !hardware
        .iter()
        .all(|part| part == only || holds(only, part, declared))
    {
        return Vec::new();
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut platforms = Vec::new();
    let composes = |whole: &str, part: &str| {
        let mut seen: BTreeSet<String> = BTreeSet::from([whole.to_owned()]);
        let mut todo = vec![whole.to_owned()];
        while let Some(at) = todo.pop() {
            let made = declared
                .get(at.as_str())
                .filter(|form| is_hardware(form))
                .map(|form| composition(form))
                .unwrap_or_default();
            for name in made {
                if name == part {
                    return true;
                }
                if seen.insert(name.clone()) {
                    todo.push(name);
                }
            }
        }
        false
    };
    let cycle = declared
        .keys()
        .copied()
        .filter(|name| *name == only || (composes(only, name) && composes(name, only)));
    for name in cycle {
        let mut at = declared.get_key_value(name);
        while let Some((name, form)) = at {
            if !is_hardware(form) || !seen.insert(*name) {
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

/// What `form` names of a declaration: by `uses`, `needs` or `refines`.
fn declarations_named_by(form: &Form) -> Vec<(String, Span)> {
    let mut names: Vec<(String, Span)> = names_in(form)
        .into_iter()
        .map(|(name, span)| (name.to_owned(), span))
        .collect();
    if is_hardware(form) {
        names.extend(Facets::of(form).0.refines);
    }
    names
}

/// `system`, one of `declarations`, on `target`.
///
/// # Errors
///
/// What keeps it from being read there, the search not having run: a name the target declares too; a file naming what only the other
/// declares, since each says what it holds with no help from the other; a block or platform the system, a task, another system it names, or a service or a policy it
/// reaches names itself, which no requirement can stand for; an absence the platform it runs on declares that the target offers (model
/// §3 rule 3); hardware the system has that refines what leads back to it; and two `platform` clauses, which read as one would hide what `check` says of them.
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
    // Each file stands alone: put together, a name either lacks would be found in the other and read as a device of the machine.
    for form in &target.declarations {
        for (name, span) in declarations_named_by(form) {
            if !theirs.contains_key(name.as_str()) && declared.contains_key(name.as_str()) {
                errors.push(Diagnostic::error(
                    "invalid-description",
                    format!("the target's file names `{name}`, which only the description declares"),
                    Label::new(span, "names it here"),
                    "a target's file says what the machine is by itself: declare the device there, or remove the name; if the name is a fact it means, rename the description's declaration",
                ));
            }
        }
    }
    for form in declarations {
        for (name, span) in declarations_named_by(form) {
            if theirs.contains_key(name.as_str()) && !declared.contains_key(name.as_str()) {
                errors.push(Diagnostic::error(
                    "infeasible-configuration",
                    format!("the description names `{name}`, a device of the target, so it can run nowhere else and cannot be read alone"),
                    Label::new(span, "names it here"),
                    "say what is asked of it instead, with `needs` and the facts: any device offering them then serves it. If `needs` names a fact here, the target's device has its name: rename the device, since a name that is both reads as the device",
                ));
            }
        }
    }
    if let [_, second, ..] = system
        .items()
        .iter()
        .filter(|item| item.head() == Some("platform"))
        .collect::<Vec<_>>()[..]
    {
        errors.push(Diagnostic::error(
            "schema-cardinality",
            format!(
                "`{}` writes two `platform` clauses, and a system has at most one",
                declared_name(system).unwrap_or("the system")
            ),
            Label::new(second.span(), "the second here"),
            "keep one: it is replaced by the target's, and two read as one would hide this",
        ));
    }
    let named = named_by_clause(system, &declared);
    let mut items: Vec<Form> = system
        .items()
        .iter()
        .filter(|item| item.head() != Some("platform"))
        .cloned()
        .collect();
    items.push(clause_on_target(
        system,
        &declared,
        &target.platform,
        target.span,
    ));
    let on_it = Form::List {
        items,
        span: system.span(),
    };
    // What the system runs on is the target's now, so a name of hardware still standing, its own or in what it reaches from there, says nothing of the machine.
    // What is reached only through hardware the target replaces is not reached there.
    let kept: Vec<Form> = declarations
        .iter()
        .filter(|form| !is_hardware(form))
        .cloned()
        .collect();
    let read_there = System::read(&on_it, &kept);
    let reached = read_there.reached.clone();
    // Another system named speaks at its own level, its clause's hardware being replaced as this one's is: what else it names of hardware, a replaced name answers no more than any.
    let others: Vec<(&str, Form)> = reached
        .iter()
        .filter_map(|name| declared.get_key_value(name.as_str()))
        .filter(|(_, form)| form.head() == Some("defsystem"))
        .map(|(name, form)| {
            let mut level = own_level(form);
            if let Form::List { items, .. } = &mut level {
                items.retain(|item| item.head() != Some("platform"));
                items.push(clause_on_target(
                    form,
                    &declared,
                    &target.platform,
                    target.span,
                ));
            }
            (*name, level)
        })
        .collect();
    let reached = reached
        .iter()
        .filter_map(|name| declared.get_key_value(name.as_str()))
        .filter(|(_, form)| !is_hardware(form) && form.head() != Some("defsystem"))
        .map(|(name, form)| (*name, *form))
        .chain(others.iter().map(|(name, form)| (*name, form)));
    let system_name = declared_name(system).unwrap_or("the system");
    for (who, form) in std::iter::once((system_name, &on_it)).chain(reached) {
        for (name, span) in names_in(form) {
            if declared.get(name).is_some_and(|form| is_hardware(form)) {
                errors.push(Diagnostic::error(
                    "infeasible-configuration",
                    format!(
                        "`{who}` names `{name}` itself, so no requirement says which device of `{}` it is",
                        target.platform
                    ),
                    Label::new(span, "names it here"),
                    "say what is asked of it instead, with `needs` and the facts: any device offering them then serves it",
                ));
            }
        }
    }
    let offered: Vec<Facets> = target
        .declarations
        .iter()
        .map(|form| Facets::of(form).0)
        .collect();
    let hardware = System::read(system, declarations).hardware;
    errors.extend(loops_refused(&hardware, &declared));
    for platform in declarations_speaking_for_the_machine(&named, &hardware, &declared) {
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
    let other_systems = read_there.other_systems;
    let asking = read_there
        .reached
        .into_iter()
        .chain(declared_name(system).map(str::to_owned))
        .collect();
    let replaced = declared
        .iter()
        .filter(|(_, form)| is_hardware(form))
        .map(|(name, _)| (*name).to_owned())
        .collect();
    let declarations = declarations
        .iter()
        .filter(|form| !is_hardware(form))
        .filter_map(|form| {
            if declared_name(form) == declared_name(system) && form.head() == system.head() {
                Some(on_it.clone())
            } else if form.head() == Some("defsystem")
                && !declared_name(form).is_some_and(|name| other_systems.contains(name))
            {
                None
            } else if form.head() == Some("defsystem") {
                // Named, it stands on the target too, as it is read above: a replaced platform is no part of what it says there.
                let mut items: Vec<Form> = form
                    .items()
                    .iter()
                    .filter(|item| item.head() != Some("platform"))
                    .cloned()
                    .collect();
                items.push(clause_on_target(
                    form,
                    &declared,
                    &target.platform,
                    target.span,
                ));
                Some(Form::List {
                    items,
                    span: form.span(),
                })
            } else {
                Some(form.clone())
            }
        })
        .chain(target.declarations.iter().cloned())
        .collect();
    Ok(OnTarget {
        system: on_it,
        declarations,
        asking,
        replaced,
    })
}

/// A requirement, as a plan binds it.
pub type Requirement = (Who, String);

/// Which target device each device of a system is, read off two plans of it.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Correspondence {
    /// Each device of the description a requirement is bound to, each target device that requirement is bound to on the target, and
    /// the requirements; a device bound only for what the target's plan does not bind has none. One device may be several, and several
    /// one: the joint rules have judged what they share.
    pub devices: BTreeMap<String, BTreeMap<String, Vec<Requirement>>>,
    /// The system's hardware no requirement is bound to: what the target's plan has nothing to say of.
    pub unreached: BTreeSet<String>,
    /// Requirements bound in the description's plan alone: what its hardware asks, or a service reached only through it, which the target's hardware does not.
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
        let targets = found.devices.entry(device.to_owned()).or_default();
        match there.bindings.get(requirement) {
            Some(theirs) => targets
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
