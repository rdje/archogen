//! The checker's reading of a target and of a system put on it (leaf `M4.1.7.5.4.2`; `DC-H2`), written from
//! `docs/decisions/decision_device-correspondence.md` alone and never from the generator's reading of it: a target's
//! file read as the record's *What a target's file is* says ([`Target::read`]), and one system of a description put on
//! it as the record's rule 1 says ([`on_target`]), every row of its table *What is refused before any search* refused
//! first, each by the code the table gives.
//!
//! ⭐ **Each file says what it holds with no help from the other** (the table's second row). A target's file names only
//! what it declares and a description only what it declares, so on the target the one name that crosses between them is
//! the one rule 1 writes: the target's platform, in the system's `platform` clause. The system's hardware there is what
//! that platform composes, which is every declaration of the target's file.
//!
//! ⭐ **The codes are the table's**, each refusal a finding under its code, and a refusal's status is the most severe of
//! its findings' verdicts by the language's precedence: `refinement-violated` under `infeasible-configuration`, the
//! verdict `check` gives that code (`DC-H3`), and every other code under the verdict its name is, or, the name of a rule,
//! `invalid-description` (`eadl_front::Verdict::of_code`).
//!
//! # Where the record leaves a choice, the reading taken
//!
//! - **Names** are read where the language reads one declaration naming another (`docs/semantics/reference.md` §6 rule
//!   10): the operands of `refines`, and of `uses` and `needs` at a side's positions (`docs/semantics/model.md` §7 rule
//!   4) — any depth of any clause but `offers` and `absent`, never inside a `uses` or `needs` list, a system's `platform`
//!   clause and tasks among them. An operand that is a symbol is a name; a list is none, and a target's is refused (the
//!   table's sixth row).
//! - **Composition** is `uses` alone, at those positions: the paragraph under the table reads it "as model §2 rule 1
//!   reads composition, by `uses` and never `needs`". So a target's platform that only `needs` a block of its file does
//!   not compose it, and the file is refused.
//! - **"One platform nothing else in the file composes"**: one platform that no other declaration of the file names by
//!   `uses`. A platform naming itself is not another's composing it.
//! - **"Its `platform` clause's own `uses`"** is a `uses` list written directly in a system's `platform` clause, `its`
//!   the namer's, as the table's third row states it: "the system's, or another system's it names, whose clause stands
//!   on the target's platform as the system's does (rule 1; leaf `M4.1.7.5.4.2`)". So the hardware another system the
//!   system names writes in its own clause's own `uses` is refused no more than the system's, and what that system
//!   names anywhere else at its own level is refused as before. A `uses` nested in a clause, inside a `requires`, is a
//!   requirement the clause holds, kept, and hardware it names is refused by the third row.
//! - **"Another system, but for one it names"**: rule 1 as amended reads it "itself or through a service or a policy it
//!   reaches (model §8 rule 7's closure, by `uses` and `needs`)", so a system is kept that the system names by `uses` or
//!   `needs`, wherever it writes them, or that a service, a policy or a kept system's own level names along that
//!   closure, which never passes through hardware. Each "stands on the target as `S` does, its own clause's hardware
//!   replaced by that platform": its clause is put on the target's platform as the system's is, a clause given where it
//!   wrote none. Every other system of the description is gone from what is read.
//! - **What asks** (the third row, "another system it names, itself or through what it reaches (rule 1), at that
//!   system's own level"): the system, each of its tasks, each system kept so, at its own level — every clause but its
//!   tasks, its clause's own `uses` judged no more than the system's — and each service and policy reached along the
//!   same closure. What is reached only through hardware the target replaces is not reached there (rule 1), and the
//!   target's hardware names nothing of the description's.
//! - **The second row is of the files, reached or not**: any declaration of the description naming what only the target
//!   declares is refused, its own hardware and a service the system does not reach among them, and a name is the
//!   description's when it resolves in its instance's scope.
//! - **What a clause stands on**: the hardware the clause's own `uses` names, which must be some; a declaration `X` the
//!   system has (model §8 rule 7) is stood on where all the hardware the system has is *of* `X` — `X` itself, what `X`
//!   composes at any depth, and the chain `X` refines, never what a refined declaration composes. Two declarations both
//!   stood on are then a cycle of composition, a loop through refinement being refused by the seventh row first, and
//!   each speaks, so no name moves the verdict. The chain up stops at what is no block or platform of the description.
//!   Where the seventh row or the fifth refuses, what the clause stands on is not read: "what a clause stands on could
//!   not be read".
//! - **What the target offers**: a fact named in an `offers` of any declaration of its file, with any value, by its name
//!   alone ("a fact stated with any value counts as offered", "a derived fact's absence meets only its own name").
//!
//! ⭐ **What is read on the target is a program** ([`OnTarget::program`]): the description's instances, its hardware and
//! every other system but those the system names removed, the `platform` clause of the system and of each system it
//! names on the target's platform, and the target's declarations added to the root instance, so their names are their
//! own and a reading of a system over a [`Program`] reads this one as any (model §8 rule 7). A system the root instance
//! declares names the target's platform by its own name. One an imported instance declares reaches the root's
//! declarations by no name of the language's (§6 rule 10), so that instance is given the root as an import of the empty
//! alias, the root exporting the target's names, and its clause names the platform `.NAME`, which resolves there to the
//! target's `NAME` and to nothing else.

use std::collections::{BTreeMap, BTreeSet};

use archogen_outcome::record::{Finding, Severity};
use archogen_outcome::Status;
use eadl_front::{Form, Instance, Program, SourceId, SourceMap, Span, Verdict as Concluded};

use crate::assignment::program_digest;
use crate::check::{at, finding, plain, status_of, Verdict};
use crate::requesters::{declarations, hardware};

/// The record's code for a file malformed as a target's, or for hardware no machine stands on.
const INVALID_DESCRIPTION: &str = "invalid-description";

/// The record's code for a description that cannot be put on a target as it is written.
const INFEASIBLE_CONFIGURATION: &str = "infeasible-configuration";

/// The record's code for an absence of what a system's clause stands on that the target breaks.
const REFINEMENT_VIOLATED: &str = "refinement-violated";

/// The record's code for one name declared twice.
const SCHEMA_DUPLICATE_NAME: &str = "schema-duplicate-name";

/// The record's code for a declaration with no name.
const SCHEMA_MISSING_NAME: &str = "schema-missing-name";

/// The record's code for two `platform` clauses on one system.
const SCHEMA_CARDINALITY: &str = "schema-cardinality";

/// The language's code for a toolchain that could not proceed (`docs/semantics/model.md` §6).
const TOOL_FAILURE: &str = "tool-failure";

/// A target's file, read (the record's *What a target's file is*): it declares hardware alone, each declaration with a
/// name of its own, one platform nothing else in the file composes, and that platform composes every other declaration,
/// so the target's hardware and every block and platform its file declares are one set.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Target {
    /// The file it was read from.
    pub source: SourceId,
    /// Its one platform: the name a plan on the target writes in `(target NAME …)`
    /// (`docs/decisions/decision_runtime-plan-format-2.md` §1), and the one a system's clause names there.
    pub platform: String,
    /// Every declaration of the file, the language-version identifier aside, in the file's order: each a block or a
    /// platform, each named once.
    pub declarations: Vec<Form>,
}

impl Target {
    /// Reads the file `id` of `sources` as a target's file. A file that does not read is refused by the reader's own
    /// diagnostics, under the most severe verdict they carry, before anything of the record's is asked; then the
    /// record's *What a target's file is*, with the rows of its table a target's own file answers: a `uses` or `needs`
    /// holding a list, and hardware refining what leads back to it.
    ///
    /// # Errors
    ///
    /// The [`Verdict`] refusing the file, every refusal a finding: `invalid-description` for a declaration that is no
    /// block or platform, for no platform or two that nothing else in the file composes, for a declaration that
    /// platform does not compose, for a `uses` or `needs` holding a list and for a refinement that leads back to its
    /// declaration; `schema-missing-name` for a declaration with no name; `schema-duplicate-name` for a name declared
    /// twice.
    pub fn read(sources: &SourceMap, id: SourceId) -> Result<Self, Verdict> {
        Self::of(sources, id, forms(sources, id)?)
    }

    /// The target the forms `declarations` of the file `id` of `sources` are — its forms as [`forms`] reads them, the
    /// language-version identifier aside — by the record's *What a target's file is* and the rows of its table a target's
    /// own file answers, as [`Target::read`] judges them once the file reads.
    ///
    /// # Errors
    ///
    /// As [`Target::read`]'s, a file that does not read apart.
    pub fn of(sources: &SourceMap, id: SourceId, declarations: Vec<Form>) -> Result<Self, Verdict> {
        let mut refusals = Vec::new();
        let platform = judge(id, &declarations, &mut refusals);
        match platform {
            Some(platform) if refusals.is_empty() => Ok(Self {
                source: id,
                platform,
                declarations,
            }),
            _ => Err(refused(sources, refusals)),
        }
    }

    /// The declaration named `name`, if the file declares one.
    #[must_use]
    pub fn declaration(&self, name: &str) -> Option<&Form> {
        self.declarations
            .iter()
            .find(|form| name_of(form).is_some_and(|(declared, _)| declared == name))
    }

    /// Whether the file declares `name`.
    #[must_use]
    pub fn declares(&self, name: &str) -> bool {
        self.declaration(name).is_some()
    }

    /// The target's hardware: every block and platform its file declares, by name — what its platform composes, the
    /// platform among it.
    #[must_use]
    pub fn hardware(&self) -> BTreeSet<String> {
        self.declarations
            .iter()
            .filter_map(name_of)
            .map(|(name, _)| name.to_owned())
            .collect()
    }

    /// The target's digest, as a plan on it writes it in `(target NAME "HEX")`: the sha256 of its forms' program text, as
    /// [`crate::assignment::program_text`] writes a plain program's (`docs/decisions/decision_runtime-plan-format-2.md`
    /// §1, "The target in the identity"), the language-version identifier aside, so a comment moves it not.
    #[must_use]
    pub fn digest(&self) -> String {
        digest(&self.declarations)
    }
}

/// The forms of a target's file `id` of `sources`, the language-version identifier aside: what its digest is of, read
/// before anything of the record's is asked of them.
///
/// # Errors
///
/// A file that does not read, refused by the reader's own diagnostics under the most severe verdict they carry.
pub fn forms(sources: &SourceMap, id: SourceId) -> Result<Vec<Form>, Verdict> {
    let (document, diagnostics) = eadl_front::read(sources, id);
    if diagnostics.has_errors() {
        let verdict = diagnostics
            .items()
            .iter()
            .map(|d| Concluded::of_code(d.code))
            .max_by_key(|v| v.precedence())
            .unwrap_or(Concluded::InvalidDescription);
        return Err(Verdict {
            status: status_of(verdict),
            findings: diagnostics
                .items()
                .iter()
                .map(|d| finding(sources, d))
                .collect(),
        });
    }
    Ok(document.declarations().cloned().collect())
}

/// The digest a plan on a target writes in `(target NAME "HEX")`, of a target's file's forms `declarations` — the
/// language-version identifier aside: the sha256 of their program text, as [`crate::assignment::program_text`] writes a
/// plain program's (`docs/decisions/decision_runtime-plan-format-2.md` §1, "The target in the identity"). So a comment
/// moves it not, and a file's forms alone decide it — its one platform among them, where it reads as a target.
#[must_use]
pub fn digest(declarations: &[Form]) -> String {
    program_digest(&plain(declarations.to_vec()))
}

/// One system of a description put on a target (the record's rule 1): the system with the target's platform in its
/// `platform` clause, read as any system, its hardware what that platform composes.
#[derive(Debug, Clone)]
pub struct OnTarget {
    /// The system, by its qualified name.
    pub system: String,
    /// The target's platform, which the system's `platform` clause names there.
    pub platform: String,
    /// The system's hardware there: every block and platform the target's file declares, the platform among them.
    pub hardware: BTreeSet<String>,
    /// What is read on the target: the description's instances with its hardware and every other system but those the
    /// system names gone, the `platform` clause of the system and of each system it names on the target's platform —
    /// the hardware its own `uses` named replaced, all else it holds kept, a clause given where none was written — and
    /// the target's declarations in the root instance, each named as the target's file names it.
    pub program: Program,
}

/// Puts the system `system` of the description elaborated into `program` on `target` (the record's rule 1), refusing
/// first every row of the record's table *What is refused before any search*, each by its code. `sources` holds the
/// description's files and the target's, for each finding's place.
///
/// # Errors
///
/// The [`Verdict`] refusing it, every refusal a finding under the most severe status: `schema-duplicate-name` for a name
/// both declare; `invalid-description` for a target naming what only the description declares, and for hardware the
/// system has refining what leads back to it; `infeasible-configuration` for the description naming what only the target
/// declares, and for what asks naming a block or platform outside the system's clause's own `uses`;
/// `refinement-violated`, under `infeasible-configuration`, for an absence of what the clause stands on, or of one up the
/// chain it refines, that the target offers; `schema-cardinality` for two `platform` clauses on the system. And
/// `tool-failure` where the description declares no system `system`, which the checker reads before it puts one on a
/// target.
pub fn on_target(
    sources: &SourceMap,
    program: &Program,
    system: &str,
    target: &Target,
) -> Result<OnTarget, Verdict> {
    let description = Description::new(program);
    let Some(&(instance, form)) = description
        .declarations
        .get(system)
        .filter(|(_, form)| form.head() == Some("defsystem"))
    else {
        return Err(Verdict {
            status: Status::ToolFailure,
            findings: vec![Finding {
                code: TOOL_FAILURE.to_owned(),
                severity: Severity::Error,
                message: format!(
                    "the description declares no system `{system}` to put on the target `{}`",
                    target.platform
                ),
                primary: at(
                    sources,
                    Span::at(target.source, 0),
                    "the target it was to be put on",
                ),
                secondary: Vec::new(),
                repair: "name a system the description declares: the checker reads the plan's system before it puts \
                         one on a target"
                    .to_owned(),
            }],
        });
    };
    let putting = Putting {
        description,
        system,
        instance,
        form,
        target,
    };
    let mut refusals = Vec::new();
    putting.both_declare(&mut refusals);
    putting.target_names_the_description(&mut refusals);
    putting.description_names_the_target(&mut refusals);
    let clauses = putting.two_clauses(&mut refusals);
    let has = hardware(program, &putting.description.declarations, system);
    let looped = putting.loops(&has, &mut refusals);
    let kept = putting.what_asks(&mut refusals);
    if clauses <= 1 && !looped {
        putting.absences(&has, &mut refusals);
    }
    if !refusals.is_empty() {
        return Err(refused(sources, refusals));
    }
    Ok(putting.read_on(&kept))
}

/// One refusal, before it is a finding.
struct Refusal {
    code: &'static str,
    message: String,
    /// The place it is about, and what it says there.
    primary: (Span, String),
    /// The other places it names, each with what it says there.
    secondary: Vec<(Span, String)>,
    repair: String,
}

impl Refusal {
    /// The verdict its code carries: `refinement-violated`'s is `infeasible-configuration`, as `check` gives it
    /// (`DC-H3`); a code that is a verdict's name, that verdict; any other code, a rule's, `invalid-description`.
    fn verdict(&self) -> Concluded {
        if self.code == REFINEMENT_VIOLATED {
            Concluded::InfeasibleConfiguration
        } else {
            Concluded::of_code(self.code)
        }
    }
}

/// The verdict of `refusals`, which are some: the most severe status they carry, every refusal a finding.
fn refused(sources: &SourceMap, refusals: Vec<Refusal>) -> Verdict {
    let verdict = refusals
        .iter()
        .map(Refusal::verdict)
        .max_by_key(|v| v.precedence())
        .unwrap_or(Concluded::InvalidDescription);
    Verdict {
        status: status_of(verdict),
        findings: refusals
            .into_iter()
            .map(|r| Finding {
                code: r.code.to_owned(),
                severity: Severity::Error,
                message: r.message,
                primary: at(sources, r.primary.0, &r.primary.1),
                secondary: r
                    .secondary
                    .iter()
                    .map(|(span, said)| at(sources, *span, said))
                    .collect(),
                repair: r.repair,
            })
            .collect(),
    }
}

/// How one declaration names another (`docs/semantics/reference.md` §6 rule 10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum By {
    Uses,
    Needs,
    Refines,
}

impl By {
    const fn word(self) -> &'static str {
        match self {
            Self::Uses => "uses",
            Self::Needs => "needs",
            Self::Refines => "refines",
        }
    }
}

/// Where in a declaration an operand is written.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Place {
    /// A system's `platform` clause's own `uses`: the hardware rule 1 replaces.
    Clause,
    /// One of a system's tasks.
    Task,
    /// Anywhere else: the declaration's own level.
    Own,
}

/// One operand of a `uses`, `needs` or `refines` a declaration writes.
#[derive(Debug, Clone, Copy)]
struct Operand<'f> {
    form: &'f Form,
    by: By,
    place: Place,
    /// The task it is written in, for [`Place::Task`].
    task: Option<&'f str>,
}

/// Every operand of a `uses`, `needs` or `refines` that `declaration` writes: a `refines` clause's values, and each
/// operand of a `uses` or `needs` list at any depth of any other clause but `offers` and `absent`, never inside such a
/// list — for a system, its `platform` clause and its tasks among them (model §7 rule 4's positions).
fn operands(declaration: &Form) -> Vec<Operand<'_>> {
    let system = declaration.head() == Some("defsystem");
    let mut out = Vec::new();
    for clause in declaration.items().iter().skip(1) {
        match clause.head() {
            None | Some("offers" | "absent") => {}
            Some("refines") => listed(clause, By::Refines, Place::Own, None, &mut out),
            Some("platform") if system => {
                for item in clause.items().iter().skip(1) {
                    if item.head() == Some("uses") {
                        listed(item, By::Uses, Place::Clause, None, &mut out);
                    } else {
                        within(item, Place::Own, None, &mut out);
                    }
                }
            }
            Some("task") if system => {
                let task = clause.items().get(1).and_then(Form::as_symbol);
                for item in clause.items().iter().skip(1) {
                    within(item, Place::Task, task, &mut out);
                }
            }
            Some(_) => within(clause, Place::Own, None, &mut out),
        }
    }
    out
}

/// `item`'s operands if it is a `uses` or `needs` list, else those of every such list among its items at any depth.
fn within<'f>(item: &'f Form, place: Place, task: Option<&'f str>, out: &mut Vec<Operand<'f>>) {
    match item.head() {
        Some("uses") => listed(item, By::Uses, place, task, out),
        Some("needs") => listed(item, By::Needs, place, task, out),
        _ => {
            for inner in item.items().iter().skip(1) {
                within(inner, place, task, out);
            }
        }
    }
}

/// Each value of the list `list`, headed by its clause word, as an operand.
fn listed<'f>(
    list: &'f Form,
    by: By,
    place: Place,
    task: Option<&'f str>,
    out: &mut Vec<Operand<'f>>,
) {
    out.extend(list.items().iter().skip(1).map(|form| Operand {
        form,
        by,
        place,
        task,
    }));
}

/// A declaration's name and where it is written: its second item, a symbol.
fn name_of(form: &Form) -> Option<(&str, Span)> {
    let name = form.items().get(1)?;
    Some((name.as_symbol()?, name.span()))
}

/// Whether `form` declares hardware: a block or a platform.
fn is_hardware(form: &Form) -> bool {
    matches!(form.head(), Some("defblock" | "defplatform"))
}

/// The fact an item of an `offers` or `absent` clause names: the symbol itself, or a list's head.
fn fact_of(item: &Form) -> Option<&str> {
    item.as_symbol().or_else(|| item.head())
}

/// The items of each `head` clause `form` writes at its own level.
fn clause_items<'f>(form: &'f Form, head: &'f str) -> impl Iterator<Item = &'f Form> + 'f {
    form.items()
        .iter()
        .skip(1)
        .filter(move |clause| clause.head() == Some(head))
        .flat_map(|clause| clause.items().iter().skip(1))
}

/// Everything reached from `from` by `next`, `from` among it.
fn closure<T: Ord + Clone>(
    from: impl IntoIterator<Item = T>,
    next: impl Fn(&T) -> Vec<T>,
) -> BTreeSet<T> {
    let mut reached = BTreeSet::new();
    let mut todo: Vec<T> = from.into_iter().collect();
    while let Some(item) = todo.pop() {
        if reached.insert(item.clone()) {
            todo.extend(next(&item));
        }
    }
    reached
}

/// What a form is, for a message: its head, or the kind of atom it is.
fn what(form: &Form) -> String {
    form.head().map_or_else(
        || format!("a bare {}", form.kind()),
        |head| format!("a `{head}`"),
    )
}

/// The record's *What a target's file is* over a target's `declarations` (the file `source`): hardware alone, each
/// declaration with a name of its own, one platform nothing else in the file composes, and that platform composing
/// every other declaration; and the table's rows a target's own file answers, a `uses` or `needs` holding a list and
/// hardware refining what leads back to it. Returns the platform, where there is one.
fn judge(source: SourceId, declarations: &[Form], refusals: &mut Vec<Refusal>) -> Option<String> {
    // Hardware alone, each with a name of its own, none twice.
    let mut named: BTreeMap<&str, &Form> = BTreeMap::new();
    let mut order: Vec<&str> = Vec::new();
    for form in declarations {
        if !is_hardware(form) {
            refusals.push(Refusal {
                code: INVALID_DESCRIPTION,
                message: format!(
                    "a target's file declares hardware alone, `defblock` and `defplatform`, and this is {}",
                    what(form)
                ),
                primary: (form.span(), "no block or platform".to_owned()),
                secondary: Vec::new(),
                repair: "move it into a description: a target's file says what the machine is, and what a system \
                         asks of it is the system's"
                    .to_owned(),
            });
            continue;
        }
        let Some((name, written)) = name_of(form) else {
            refusals.push(Refusal {
                code: SCHEMA_MISSING_NAME,
                message: format!(
                    "a `{}` of the target's file carries no name: each declaration of a target's file has a name of \
                     its own",
                    form.head().unwrap_or_default()
                ),
                primary: (form.span(), "no name after the kind".to_owned()),
                secondary: Vec::new(),
                repair: format!(
                    "write `({} <name> …)`",
                    form.head().unwrap_or_default()
                ),
            });
            continue;
        };
        if let Some(first) = named.get(name) {
            let first = name_of(first).map_or_else(|| first.span(), |(_, span)| span);
            refusals.push(Refusal {
                code: SCHEMA_DUPLICATE_NAME,
                message: format!(
                    "`{name}` is declared twice in the target's file: a name means one declaration"
                ),
                primary: (written, "declared again here".to_owned()),
                secondary: vec![(first, "first declared here".to_owned())],
                repair: "rename one, or remove it: a target's file holds one declaration per name"
                    .to_owned(),
            });
            continue;
        }
        named.insert(name, form);
        order.push(name);
    }
    // A `uses` or `needs` holding a list, which a system would not read as hardware.
    for form in declarations.iter().filter(|form| is_hardware(form)) {
        let owner = name_of(form).map_or("<unnamed>", |(name, _)| name);
        for operand in operands(form) {
            if operand.by != By::Refines && matches!(operand.form, Form::List { .. }) {
                refusals.push(Refusal {
                    code: INVALID_DESCRIPTION,
                    message: format!(
                        "the target's `{owner}` holds a list in its `{}`, `{}`, which a system would not read as \
                         hardware: one set, or every system on it is refused by the search, far from where this is \
                         written",
                        operand.by.word(),
                        operand.form.to_canonical()
                    ),
                    primary: (operand.form.span(), "a list".to_owned()),
                    secondary: Vec::new(),
                    repair: format!(
                        "write each operand of `{}` as the name of a block or platform of the target's file",
                        operand.by.word()
                    ),
                });
            }
        }
    }
    let composes = |name: &&str| {
        named
            .get(name)
            .map_or_else(Vec::new, |form| declared_by(form, By::Uses, &named))
    };
    // One platform nothing else in the file composes.
    let composed: BTreeSet<&str> = named
        .iter()
        .flat_map(|(name, form)| {
            declared_by(form, By::Uses, &named)
                .into_iter()
                .filter(move |part| part != name)
        })
        .collect();
    let roots: Vec<&str> = order
        .iter()
        .copied()
        .filter(|name| named[name].head() == Some("defplatform") && !composed.contains(name))
        .collect();
    let platform = match roots.as_slice() {
        [] => {
            let none = !named
                .values()
                .any(|form| form.head() == Some("defplatform"));
            refusals.push(Refusal {
                code: INVALID_DESCRIPTION,
                message: if none {
                    "the target's file declares no platform: a target is one platform and what it composes".to_owned()
                } else {
                    "every platform of the target's file is composed by another of its declarations: a target's file \
                     has one platform nothing else in it composes"
                        .to_owned()
                },
                primary: (
                    declarations
                        .first()
                        .map_or_else(|| Span::at(source, 0), Form::span),
                    "the target's file".to_owned(),
                ),
                secondary: Vec::new(),
                repair: "declare one platform that composes, by `uses`, every other block and platform of the file, \
                         and that nothing else composes"
                    .to_owned(),
            });
            None
        }
        [root] => Some(*root),
        [first, rest @ ..] => {
            let first = name_of(named[first]).map_or_else(|| named[first].span(), |(_, span)| span);
            for extra in rest {
                let written =
                    name_of(named[extra]).map_or_else(|| named[extra].span(), |(_, span)| span);
                refusals.push(Refusal {
                    code: INVALID_DESCRIPTION,
                    message: format!(
                        "`{extra}` is a second platform of the target's file that nothing else in it composes: a \
                         target's file has one"
                    ),
                    primary: (written, "nothing composes it".to_owned()),
                    secondary: vec![(first, "nor this one".to_owned())],
                    repair: "compose one into the other by `uses`, or put each in a target's file of its own".to_owned(),
                });
            }
            None
        }
    };
    // That platform composes every other declaration.
    if let Some(root) = platform {
        let reached = closure([root], composes);
        for name in order.iter().filter(|name| !reached.contains(*name)) {
            let written = name_of(named[name]).map_or_else(|| named[name].span(), |(_, span)| span);
            refusals.push(Refusal {
                code: INVALID_DESCRIPTION,
                message: format!(
                    "`{name}` is not composed by the target's platform `{root}`: that platform composes every other \
                     declaration of a target's file, so the target's hardware and every block and platform its file \
                     declares are one set"
                ),
                primary: (written, "not composed by the platform".to_owned()),
                secondary: Vec::new(),
                repair: format!(
                    "compose it into `{root}` by `uses`, at any depth, or remove it from the target's file"
                ),
            });
        }
    }
    // Hardware that refines what leads back to it, through what it composes and refines.
    for name in &order {
        for operand in operands(named[name])
            .into_iter()
            .filter(|operand| operand.by == By::Refines)
        {
            let Some(refined) = operand.form.as_symbol().filter(|r| named.contains_key(r)) else {
                continue;
            };
            let back = closure([refined], |at: &&str| {
                let form = named[at];
                let mut next = declared_by(form, By::Uses, &named);
                next.extend(declared_by(form, By::Refines, &named));
                next
            });
            if back.contains(name) {
                refusals.push(loop_refused(
                    name,
                    refined,
                    operand.form.span(),
                    "hardware of the target's file",
                ));
            }
        }
    }
    platform.map(str::to_owned)
}

/// The names `form` writes by `by` that `declared`, a target's declarations by name, holds.
fn declared_by<'f>(form: &'f Form, by: By, declared: &BTreeMap<&str, &Form>) -> Vec<&'f str> {
    operands(form)
        .into_iter()
        .filter(|operand| operand.by == by)
        .filter_map(|operand| operand.form.as_symbol())
        .filter(|name| declared.contains_key(name))
        .collect()
}

/// The seventh row's refusal: `name`, whose hardware `whose` says, refines `refined`, which leads back to it.
fn loop_refused(name: &str, refined: &str, span: Span, whose: &str) -> Refusal {
    Refusal {
        code: INVALID_DESCRIPTION,
        message: format!(
            "`{name}`, {whose}, refines `{refined}`, which leads back to it through what they compose and refine: a \
             refinement says the same thing again and brings no blocks, so no machine stands on a loop"
        ),
        primary: (span, "the refinement".to_owned()),
        secondary: Vec::new(),
        repair: "drop the refinement, or refine a declaration that is not made of this one".to_owned(),
    }
}

/// The description a system is put on a target from: its elaborated program, and every declaration by its qualified
/// name with the instance that wrote it.
struct Description<'p> {
    program: &'p Program,
    declarations: BTreeMap<String, (usize, &'p Form)>,
}

impl<'p> Description<'p> {
    fn new(program: &'p Program) -> Self {
        Self {
            program,
            declarations: declarations(program),
        }
    }

    /// The declaration `operand`, written in the instance `instance`, names in that instance's scope
    /// (`docs/semantics/reference.md` §6 rule 10); none for a name of the vocabulary or of nothing.
    fn resolve(&self, instance: usize, operand: &Form) -> Option<String> {
        let name = operand.as_symbol()?;
        self.program
            .resolve(instance, name, operand.span())
            .ok()
            .flatten()
    }

    /// The head of the declaration named `name`.
    fn head(&self, name: &str) -> Option<&'p str> {
        self.declarations
            .get(name)
            .and_then(|&(_, form)| form.head())
    }

    /// Whether `name` is a block or a platform of the description.
    fn is_hardware(&self, name: &str) -> bool {
        matches!(self.head(name), Some("defblock" | "defplatform"))
    }

    /// The description's blocks and platforms the declaration `name` names by `by`.
    fn hardware_named(&self, name: &str, by: By) -> Vec<String> {
        let Some(&(instance, form)) = self.declarations.get(name) else {
            return Vec::new();
        };
        operands(form)
            .into_iter()
            .filter(|operand| operand.by == by)
            .filter_map(|operand| self.resolve(instance, operand.form))
            .filter(|named| self.is_hardware(named))
            .collect()
    }
}

/// One system of a description being put on a target: what each row of the record's table reads.
struct Putting<'a, 'p> {
    description: Description<'p>,
    /// The system, by its qualified name.
    system: &'a str,
    /// The instance that declares it.
    instance: usize,
    /// Its declaration.
    form: &'p Form,
    target: &'a Target,
}

impl Putting<'_, '_> {
    /// The table's first row: a name both the description and the target declare.
    fn both_declare(&self, refusals: &mut Vec<Refusal>) {
        for declaration in &self.target.declarations {
            let Some((name, written)) = name_of(declaration) else {
                continue;
            };
            let Some(&(_, theirs)) = self.description.declarations.get(name) else {
                continue;
            };
            let theirs = name_of(theirs).map_or_else(|| theirs.span(), |(_, span)| span);
            refusals.push(Refusal {
                code: SCHEMA_DUPLICATE_NAME,
                message: format!(
                    "`{name}` is declared by both the description and the target: a name means one declaration, and \
                     a device taken for another by name is what putting a system on a target exists not to do"
                ),
                primary: (written, "declared by the target here".to_owned()),
                secondary: vec![(theirs, "and by the description here".to_owned())],
                repair: "rename one: a system is matched to the target's devices by what it asks of them, so the \
                         description's hardware needs no name of the target's"
                    .to_owned(),
            });
        }
    }

    /// The table's second row, of the target's file: a name it writes by `uses`, `needs` or `refines` that only the
    /// description declares.
    fn target_names_the_description(&self, refusals: &mut Vec<Refusal>) {
        for declaration in &self.target.declarations {
            let owner = name_of(declaration).map_or("<unnamed>", |(name, _)| name);
            for operand in operands(declaration) {
                let Some(name) = operand.form.as_symbol() else {
                    continue;
                };
                if self.target.declares(name) {
                    continue;
                }
                let Some(&(_, theirs)) = self.description.declarations.get(name) else {
                    continue;
                };
                let theirs = name_of(theirs).map_or_else(|| theirs.span(), |(_, span)| span);
                refusals.push(Refusal {
                    code: INVALID_DESCRIPTION,
                    message: format!(
                        "the target's `{owner}` names `{name}` by `{}`, which only the description declares: a target's \
                         file says what it holds with no help from a description, and read beside one its name would \
                         find the description's device and bind it",
                        operand.by.word()
                    ),
                    primary: (operand.form.span(), "a name the target does not declare".to_owned()),
                    secondary: vec![(theirs, "declared by the description here".to_owned())],
                    repair: "name only what the target's file declares: declare the device there, or remove the name"
                        .to_owned(),
                });
            }
        }
    }

    /// The table's second row, of the description: a name any of its declarations writes by `uses`, `needs` or
    /// `refines` that resolves to nothing of its own and that only the target declares.
    fn description_names_the_target(&self, refusals: &mut Vec<Refusal>) {
        let program = self.description.program;
        for instance in &program.instances {
            for declaration in &instance.declarations {
                let owner = name_of(declaration).map_or_else(
                    || "a declaration with no name".to_owned(),
                    |(name, _)| format!("`{}`", instance.qualify(name)),
                );
                for operand in operands(declaration) {
                    let Some(name) = operand.form.as_symbol() else {
                        continue;
                    };
                    if !self.target.declares(name)
                        || self.description.declarations.contains_key(name)
                    {
                        continue;
                    }
                    if !matches!(
                        program.resolve(instance.id, name, operand.form.span()),
                        Ok(None)
                    ) {
                        continue;
                    }
                    let theirs = self
                        .target
                        .declaration(name)
                        .and_then(name_of)
                        .map_or_else(|| Span::at(self.target.source, 0), |(_, span)| span);
                    refusals.push(Refusal {
                        code: INFEASIBLE_CONFIGURATION,
                        message: format!(
                            "{owner} names `{name}` by `{}`, which only the target declares: a description says what \
                             it holds with no help from a target, and could otherwise be read only beside that one \
                             target",
                            operand.by.word()
                        ),
                        primary: (operand.form.span(), "a name the description does not declare".to_owned()),
                        secondary: vec![(theirs, "declared by the target here".to_owned())],
                        repair: "say what is asked of the device, a requirement, in place of its name: a target's \
                                 devices are matched to a system by what it asks of them"
                            .to_owned(),
                    });
                }
            }
        }
    }

    /// The table's fifth row: two `platform` clauses on the system. Returns how many it writes.
    fn two_clauses(&self, refusals: &mut Vec<Refusal>) -> usize {
        let clauses: Vec<&Form> = self
            .form
            .items()
            .iter()
            .filter(|item| item.head() == Some("platform"))
            .collect();
        if let [first, rest @ ..] = clauses.as_slice() {
            for extra in rest {
                refusals.push(Refusal {
                    code: SCHEMA_CARDINALITY,
                    message: format!(
                        "the system `{}` writes {} `platform` clauses: a system has at most one, and read as one they \
                         would hide it",
                        self.system,
                        clauses.len()
                    ),
                    primary: (extra.span(), "another `platform` clause".to_owned()),
                    secondary: vec![(first.span(), "beside this one".to_owned())],
                    repair: "keep one `(platform …)` clause".to_owned(),
                });
            }
        }
        clauses.len()
    }

    /// The table's seventh row, of the description: a block or platform the system has (`has`, model §8 rule 7) that
    /// refines what leads back to it through what they compose and refine. Returns whether one does.
    fn loops(&self, has: &BTreeSet<String>, refusals: &mut Vec<Refusal>) -> bool {
        let description = &self.description;
        let mut looped = false;
        for name in has {
            let Some(&(instance, form)) = description.declarations.get(name) else {
                continue;
            };
            for operand in operands(form)
                .into_iter()
                .filter(|operand| operand.by == By::Refines)
            {
                let Some(refined) = description
                    .resolve(instance, operand.form)
                    .filter(|refined| description.is_hardware(refined))
                else {
                    continue;
                };
                let back = closure([refined.clone()], |at: &String| {
                    let mut next = description.hardware_named(at, By::Uses);
                    next.extend(description.hardware_named(at, By::Refines));
                    next
                });
                if back.contains(name) {
                    looped = true;
                    let whose = format!("hardware the system `{}` has", self.system);
                    let mut refusal = loop_refused(name, &refined, operand.form.span(), &whose);
                    refusal
                        .message
                        .push_str(", and what the system's clause stands on could not be read");
                    refusals.push(refusal);
                }
            }
        }
        looped
    }

    /// The table's third row: the system, a task, another system it names, itself or through what it reaches, at that
    /// system's own level, or a service or a policy the system reaches on the target naming a block or platform itself, by
    /// `uses` or `needs`, anywhere but its `platform` clause's own `uses`, `its` the namer's — the system's, or another
    /// system's it names, which stands on the target's platform as the system's does. Returns the systems the system
    /// names so, which stay in what is read (rule 1).
    fn what_asks(&self, refusals: &mut Vec<Refusal>) -> BTreeSet<String> {
        let description = &self.description;
        let system = self.system;
        let own = operands(self.form);
        let named_by = |operands: &[(usize, Operand<'_>)]| -> Vec<String> {
            operands
                .iter()
                .filter(|(_, operand)| operand.by != By::Refines)
                .filter_map(|(instance, operand)| description.resolve(*instance, operand.form))
                .collect()
        };
        let mine: Vec<(usize, Operand<'_>)> = own
            .iter()
            .map(|operand| (self.instance, *operand))
            .collect();
        // What asks beside the system and its tasks: each service, policy and other system the system reaches by `uses` and
        // `needs`, itself or through a service or a policy it reaches (rule 1; model §8 rule 7's closure) — through another
        // system at that system's own level too, its tasks being its own, and never through hardware, the description's
        // being gone and the target's naming nothing of the description's.
        let asks = |named: &String| {
            named != system
                && matches!(
                    description.head(named),
                    Some("defservice" | "defpolicy" | "defsystem")
                )
        };
        let reached = closure(named_by(&mine).into_iter().filter(asks), |name: &String| {
            let &(instance, form) = &description.declarations[name];
            let theirs: Vec<(usize, Operand<'_>)> = operands(form)
                .into_iter()
                .filter(|operand| operand.place != Place::Task)
                .map(|operand| (instance, operand))
                .collect();
            named_by(&theirs).into_iter().filter(asks).collect()
        });
        let others: BTreeSet<String> = reached
            .iter()
            .filter(|named| description.head(named) == Some("defsystem"))
            .cloned()
            .collect();
        // Who asks, with each operand of theirs judged.
        let mut judged: Vec<(String, usize, Operand<'_>)> = Vec::new();
        for operand in &own {
            match operand.place {
                Place::Clause => {}
                Place::Task => judged.push((
                    format!(
                        "the task `{}` of the system `{system}`",
                        operand.task.unwrap_or("<unnamed>")
                    ),
                    self.instance,
                    *operand,
                )),
                Place::Own => {
                    judged.push((format!("the system `{system}`"), self.instance, *operand))
                }
            }
        }
        for name in &reached {
            let &(instance, form) = &description.declarations[name];
            let (kind, own_level) = match form.head() {
                Some("defsystem") => ("system", true),
                Some("defpolicy") => ("policy", false),
                _ => ("service", false),
            };
            for operand in operands(form) {
                // Another system at its own level, its tasks being its own; its clause's own `uses` stands on the target's
                // platform as the system's does, so it is judged no more than the system's.
                if own_level && operand.place != Place::Own {
                    continue;
                }
                judged.push((
                    format!("the {kind} `{name}`, which `{system}` reaches,"),
                    instance,
                    operand,
                ));
            }
        }
        for (who, instance, operand) in judged {
            if operand.by == By::Refines {
                continue;
            }
            let Some(named) = description
                .resolve(instance, operand.form)
                .filter(|named| description.is_hardware(named))
            else {
                continue;
            };
            let kind = if description.head(&named) == Some("defplatform") {
                "platform"
            } else {
                "block"
            };
            refusals.push(Refusal {
                code: INFEASIBLE_CONFIGURATION,
                message: format!(
                    "{who} names the {kind} `{named}` by `{}` outside a system's `platform` clause's own `uses`: on the \
                     target `{}` no requirement stands for a name, so nothing says which of its devices it is",
                    operand.by.word(),
                    self.target.platform
                ),
                primary: (operand.form.span(), format!("names the {kind} itself")),
                secondary: Vec::new(),
                repair: "say what is asked of it, a requirement such as `(needs FACT)`, in place of its name: a \
                         system's hardware is named by its `platform` clause's own `uses` alone, which the target's \
                         platform replaces"
                    .to_owned(),
            });
        }
        others
    }

    /// The table's fourth row: the one declaration the system's clause stands on, or one up the chain it refines,
    /// declaring a fact absent that the target offers. `has` is the hardware the system has (model §8 rule 7). The
    /// paragraph under the table defines what a clause stands on: see this module's choices.
    fn absences(&self, has: &BTreeSet<String>, refusals: &mut Vec<Refusal>) {
        let description = &self.description;
        let clause_names = operands(self.form)
            .into_iter()
            .filter(|operand| operand.place == Place::Clause)
            .filter_map(|operand| description.resolve(self.instance, operand.form))
            .any(|named| description.is_hardware(&named));
        if !clause_names {
            return;
        }
        // The chain `x` refines, up to what is no hardware.
        let up = |x: &String| -> BTreeSet<String> {
            let mut chain = closure(description.hardware_named(x, By::Refines), |at: &String| {
                description.hardware_named(at, By::Refines)
            });
            chain.remove(x);
            chain
        };
        // What is of `x`: itself, what it composes at any depth, and the chain it refines.
        let of = |x: &String| -> BTreeSet<String> {
            let mut of = closure([x.clone()], |at: &String| {
                description.hardware_named(at, By::Uses)
            });
            of.extend(up(x));
            of
        };
        let stood_on: Vec<&String> = has.iter().filter(|x| has.is_subset(&of(x))).collect();
        // Each speaks, and each one up the chain it refines, with the declaration the clause stands on it is up from.
        let mut speaking: BTreeMap<String, Option<String>> =
            stood_on.iter().map(|x| ((*x).clone(), None)).collect();
        for x in stood_on.iter().copied() {
            for above in up(x) {
                speaking.entry(above).or_insert_with(|| Some(x.clone()));
            }
        }
        let mut offered: BTreeMap<&str, Vec<(&str, Span)>> = BTreeMap::new();
        for declaration in &self.target.declarations {
            let owner = name_of(declaration).map_or("<unnamed>", |(name, _)| name);
            for item in clause_items(declaration, "offers") {
                if let Some(fact) = fact_of(item) {
                    offered.entry(fact).or_default().push((owner, item.span()));
                }
            }
        }
        for (declaration, from) in &speaking {
            let &(_, form) = &description.declarations[declaration];
            for item in clause_items(form, "absent") {
                let Some(fact) = fact_of(item) else {
                    continue;
                };
                let Some(offers) = offered.get(fact) else {
                    continue;
                };
                let standing = from.as_ref().map_or_else(
                    || "which the system's `platform` clause stands on".to_owned(),
                    |x| format!("up the chain `{x}` refines, which the system's `platform` clause stands on"),
                );
                let by: Vec<String> = offers
                    .iter()
                    .map(|(owner, _)| format!("`{owner}`"))
                    .collect();
                refusals.push(Refusal {
                    code: REFINEMENT_VIOLATED,
                    message: format!(
                        "`{declaration}`, {standing}, declares `{fact}` absent, and the target offers it, by {}: an \
                         absence is a constraint something relies on, and no requirement carries it (model §3 rule 3)",
                        by.join(", ")
                    ),
                    primary: (item.span(), "declared absent here".to_owned()),
                    secondary: offers
                        .iter()
                        .map(|(owner, span)| (*span, format!("offered by the target's `{owner}`")))
                        .collect(),
                    repair: format!(
                        "put the system on a target that does not offer `{fact}`, or drop the absence from \
                         `{declaration}` where nothing relies on it"
                    ),
                });
            }
        }
    }

    /// What is read on the target (rule 1), once nothing is refused: see [`OnTarget::program`]. `kept` holds the systems
    /// the system names, each standing on the target as the system does.
    fn read_on(&self, kept: &BTreeSet<String>) -> OnTarget {
        let description = &self.description;
        let program = description.program;
        let root = program.root().id;
        let target = self.target;
        let written = target
            .declaration(&target.platform)
            .and_then(name_of)
            .map_or_else(|| Span::at(target.source, 0), |(_, span)| span);
        // A system of the instance `instance` on the target's platform, named there as that instance reaches it.
        let stand = |instance: usize, system: &Form| -> Form {
            let operand = Form::Symbol {
                name: if instance == root {
                    target.platform.clone()
                } else {
                    format!(".{}", target.platform)
                },
                span: written,
            };
            let replaced = |operand: &Form| {
                description
                    .resolve(instance, operand)
                    .is_some_and(|named| description.is_hardware(&named))
            };
            on_the_platform(system, &operand, &replaced)
        };
        let standing = |qualified: &String| qualified == self.system || kept.contains(qualified);
        let read_there = |owner: &Instance, declaration: &Form| -> Option<Form> {
            match declaration.head() {
                Some("defblock" | "defplatform") => None,
                Some("defsystem") => {
                    let qualified = owner.qualify(name_of(declaration)?.0);
                    standing(&qualified).then(|| stand(owner.id, declaration))
                }
                _ => Some(declaration.clone()),
            }
        };
        let mut instances: Vec<Instance> = program
            .instances
            .iter()
            .map(|instance| {
                let mut each = instance.clone();
                each.declarations = instance
                    .declarations
                    .iter()
                    .filter_map(|declaration| read_there(instance, declaration))
                    .collect();
                each
            })
            .collect();
        instances[root]
            .declarations
            .extend(target.declarations.iter().cloned());
        // Each imported instance a standing system is declared in reaches the target's names through the root.
        let reaching: BTreeSet<usize> = std::iter::once(self.instance)
            .chain(
                kept.iter()
                    .filter_map(|other| description.declarations.get(other))
                    .map(|&(instance, _)| instance),
            )
            .filter(|instance| *instance != root)
            .collect();
        if !reaching.is_empty() {
            instances[root].exports.extend(target.hardware());
        }
        for instance in reaching {
            instances[instance].imports.push(root);
        }
        OnTarget {
            system: self.system.to_owned(),
            platform: target.platform.clone(),
            hardware: target.hardware(),
            program: Program { instances },
        }
    }
}

/// The system `system` with `operand`, the target's platform, in its `platform` clause (rule 1): the hardware the
/// clause's own `uses` names, each operand `replaced` says is, replaced by it, and a clause given where the system wrote
/// none; all else the clause holds kept. The system put on the target writes one clause, a second being refused first;
/// another system it names is refused for no second, and each of its clauses stands on the platform alike.
fn on_the_platform(system: &Form, operand: &Form, replaced: &dyn Fn(&Form) -> bool) -> Form {
    let span = system.span();
    let mut items = Vec::with_capacity(system.items().len() + 1);
    let mut given = false;
    for item in system.items() {
        if item.head() == Some("platform") {
            given = true;
            items.push(clause_on(item, operand, replaced));
        } else {
            items.push(item.clone());
        }
    }
    if !given {
        items.push(list(
            span,
            vec![symbol("platform", span), uses(operand, span)],
        ));
    }
    list(span, items)
}

/// One `platform` clause on the target's platform `operand`: `(uses OPERAND)` first, then each item it held, the hardware
/// of its own `uses` dropped and a `uses` left with nothing dropped with it.
fn clause_on(clause: &Form, operand: &Form, replaced: &dyn Fn(&Form) -> bool) -> Form {
    let span = clause.span();
    let mut items = vec![symbol("platform", span), uses(operand, span)];
    for item in clause.items().iter().skip(1) {
        if item.head() != Some("uses") {
            items.push(item.clone());
            continue;
        }
        let left: Vec<Form> = item
            .items()
            .iter()
            .skip(1)
            .filter(|named| !replaced(named))
            .cloned()
            .collect();
        if !left.is_empty() {
            let mut kept = vec![symbol("uses", item.span())];
            kept.extend(left);
            items.push(list(item.span(), kept));
        }
    }
    list(span, items)
}

/// `(uses OPERAND)`, written at `span`.
fn uses(operand: &Form, span: Span) -> Form {
    list(span, vec![symbol("uses", span), operand.clone()])
}

fn symbol(name: &str, span: Span) -> Form {
    Form::Symbol {
        name: name.to_owned(),
        span,
    }
}

fn list(span: Span, items: Vec<Form>) -> Form {
    Form::List { items, span }
}
