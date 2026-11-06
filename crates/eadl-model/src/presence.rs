//! Presence, relevance, and the dependency closure.
//!
//! `ROADMAP.md` §5.3 states three rules, and the middle one is the interesting one:
//!
//! > Maintain **offered**, **explicitly absent**, and **undescribed** states for facts in a
//! > versioned capability vocabulary. "Offered" records a claim whose evidential status is
//! > separate; declaring it does not make it proven.
//! >
//! > Compute the transitive dependency closure of the requested services, provider
//! > preconditions, target bindings, and requested analyses. **Unknown facts inside that
//! > closure block the relevant decision. Unknown facts outside it remain visible in metadata
//! > and do not fail generation.**
//! >
//! > **Reject contradictory declarations rather than choosing one.**
//!
//! The correction table in §2 says why the second rule exists. The original position — "unknown
//! capability anywhere blocks generation" — was revised to "required facts in the selected
//! dependency closure must be known", because a platform description carries facts about
//! hardware the system never touches, and failing on those makes every real platform
//! undescribable. F04 and F05 are the two halves of that single decision, and a checker that
//! satisfies one by giving up on the other has not implemented it.
//!
//! # What this is not
//!
//! This is **presence and relevance analysis**, not provider resolution. It answers "is every
//! fact this system depends on actually known?" — it does not choose implementations, allocate
//! resources, or check capacity. That is `M3`, and conflating the two here would produce a
//! resolver nobody reviewed.

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{read, Diagnostic, Form, Label, SourceMap, Span};

/// What a description says about one fact.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Presence {
    /// The description claims it. §5.3: a claim, not evidence — "declaring it does not make it
    /// proven".
    Offered,
    /// The description states it is **not** available. A negative fact is a fact.
    Absent,
    /// Nothing says. Not the same as absent, and the difference is the whole point: an
    /// undescribed fact inside the closure is a `missing-fact`, while an absent one is a
    /// definite answer that a requirement then contradicts.
    Undescribed,
}

impl Presence {
    /// The stable machine-readable name.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Offered => "offered",
            Self::Absent => "absent",
            Self::Undescribed => "undescribed",
        }
    }
}

/// The engine's adapter catalog, embedded so that presence knows where an engine path may exist
/// (`docs/specs/adapters/decision_engine-adapters.md` §7). Presence reads names alone; the engine reads the rest.
const ADAPTERS_TEXT: &str = include_str!("../../../docs/semantics/adapters/adapters.eadl");

/// One adapter kind as presence reads it: what it adds and what its `beneath` names (record §7.1, §7.2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdapterPath {
    /// The kind's name.
    pub kind: String,
    /// Each fact its offers name that is not passed through from itself and that its `beneath` does not name.
    pub adds: Vec<String>,
    /// Every fact its `beneath` names — `needs` operands and constraint heads — in written order.
    pub beneath: Vec<String>,
}

/// Every kind of the shipped catalog, read for names, in its order. A catalog that does not read gives none — the
/// engine's reader refuses it, and `tests/adapters.rs` holds the shipped file to no refusal.
#[must_use]
pub fn adapter_paths() -> Vec<AdapterPath> {
    let mut sources = SourceMap::new();
    let Ok(id) = sources.add("docs/semantics/adapters/adapters.eadl", ADAPTERS_TEXT) else {
        return Vec::new();
    };
    let (document, diagnostics) = read(&sources, id);
    if diagnostics.has_errors() {
        return Vec::new();
    }
    let rest = |form: &Form| form.items().get(1..).unwrap_or_default().to_vec();
    document
        .declarations()
        .filter(|form| form.head() == Some("defadapter"))
        .map(|form| {
            let clause = |head: &str| {
                form.items()
                    .iter()
                    .find(|item| item.head() == Some(head))
                    .cloned()
            };
            let mut beneath = Vec::new();
            for item in clause("beneath").map(|c| rest(&c)).unwrap_or_default() {
                match item.head() {
                    Some("needs") => beneath.extend(
                        rest(&item)
                            .iter()
                            .filter_map(Form::as_symbol)
                            .map(str::to_string),
                    ),
                    Some(fact) => beneath.push(fact.to_string()),
                    None => {}
                }
            }
            let adds = clause("offers")
                .map(|c| rest(&c))
                .unwrap_or_default()
                .iter()
                .filter_map(|item| {
                    let fact = item.head()?;
                    let passed_through = matches!(
                        rest(item).as_slice(),
                        [from] if from.head() == Some("from")
                            && rest(from).first().and_then(Form::as_symbol) == Some(fact)
                    );
                    (!passed_through && !beneath.iter().any(|b| b == fact))
                        .then(|| fact.to_string())
                })
                .collect();
            AdapterPath {
                kind: form
                    .items()
                    .get(1)
                    .and_then(Form::as_symbol)
                    .unwrap_or_default()
                    .to_string(),
                adds,
                beneath,
            }
        })
        .collect()
}

/// Where an engine path for one required fact stands, by name (record §7.2, §7.3).
enum Path {
    /// One block or platform offers everything a kind adding the fact needs beneath: the search's to judge.
    Whole,
    /// One offers part of it and leaves the rest undescribed there, declaring none of it absent.
    Partial {
        kind: String,
        declaration: String,
        undescribed: Vec<String>,
    },
    /// Kinds add it, and no block or platform offers even part of what one needs.
    Nowhere(Vec<AdapterPath>),
    /// No kind adds it.
    NoKind,
}

/// Where a fact's presence was declared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Declaration {
    /// The declaring form's name, for messages.
    pub declared_by: String,
    /// Where.
    pub span: Span,
}

/// Everything a set of declarations says about facts, plus what needs what.
#[derive(Debug, Clone, Default)]
pub struct FactMap {
    /// Every provider's offer of each fact, in the order written — not the first alone: an offer and an absence are
    /// compared provider by provider (model §2 rule 1, `docs/decisions/decision_absence-per-provider.md`).
    offered: BTreeMap<String, Vec<Declaration>>,
    /// Every provider's absence of each fact, in the order written.
    absent: BTreeMap<String, Vec<Declaration>>,
    /// Every name this description declares — a block, a service, a policy, a system.
    ///
    /// ⭐ A composition reference is not a fact request. `(requires (uses timer.counter))` on a
    /// platform names a *block that is declared right there*, and that block's own obligations
    /// are checked by its own declaration. Treating every `uses` target as a leaf fact made the
    /// first real example fail with `missing-fact: console.uart` — found by the example suite,
    /// not by inspection.
    declared: BTreeSet<String>,
    /// `name` → the facts and services it needs.
    needs: BTreeMap<String, Vec<(String, Span)>>,
    /// What the system requests: every `uses`, and a system's own `needs`, its tasks and its clauses among it. A
    /// `needs` of anything else is an edge, followed only once what wrote it is reached.
    requested: Vec<(String, Span)>,
    /// Each block's and platform's direct composition, by [`composition`] (leaf `M3.4.8`).
    composes: BTreeMap<String, Vec<String>>,
}

/// The outcome of a presence check.
#[derive(Debug, Clone)]
pub struct PresenceReport {
    /// The transitive closure of everything the request depends on, sorted.
    pub closure: Vec<String>,
    /// Facts that are declared or needed but lie outside the closure, sorted.
    ///
    /// §5.3: these "remain visible in metadata and do not fail generation". Visible is the
    /// operative word — they are reported, just not as failures.
    pub outside_closure: Vec<String>,
    /// Facts inside the closure that nothing describes.
    pub missing: Vec<String>,
    /// Each fact of the closure and what pulled it in, from [`FactMap::closure_origins`].
    pub pulled_in_by: Vec<(String, Option<String>)>,
    /// Everything wrong, in the order found.
    pub diagnostics: Vec<Diagnostic>,
}

impl PresenceReport {
    /// Whether the description can proceed.
    #[must_use]
    pub fn is_admissible(&self) -> bool {
        self.diagnostics.is_empty()
    }
}

impl FactMap {
    /// An empty map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Collect presence and dependency clauses from one declaration.
    ///
    /// Recognized clauses: `(offers <name> …)`, `(absent <name> …)`, `(needs <name> …)`,
    /// `(uses <name> …)`. An `offers` clause whose first element is a nested form declares the
    /// nested head, so `(offers (counter-width 32 bit))` offers `counter-width`.
    pub fn collect(&mut self, form: &Form) {
        let owner = form
            .items()
            .get(1)
            .and_then(Form::as_symbol)
            .unwrap_or("<unnamed>")
            .to_string();
        if owner != "<unnamed>" {
            self.declared.insert(owner.clone());
        }
        if matches!(form.head(), Some("defblock" | "defplatform")) {
            self.composes.insert(owner.clone(), composition(form));
        }
        let asks = form.head() == Some("defsystem");

        for clause in form.items().iter().skip(1) {
            let Some(head) = clause.head() else { continue };
            match head {
                "offers" => self.record(&mut Slot::Offered, clause, &owner),
                "absent" => self.record(&mut Slot::Absent, clause, &owner),
                "needs" => {
                    for name in names_in(clause) {
                        if asks {
                            self.requested.push(name.clone());
                        }
                        self.needs.entry(owner.clone()).or_default().push(name);
                    }
                }
                "uses" => {
                    for name in names_in(clause) {
                        self.requested.push(name);
                    }
                }
                // A refinement pair is two declarations, one owing the other model §3's obligations,
                // which `refinement.rs` checks — a fact the abstract declares absent and the concrete
                // offers is a violated exclusion obligation, never this module's contradiction, which
                // is one declaration's (leaf `M3.4.8`).
                "refines" => {}
                _ => {
                    // `requires` and other clauses can nest `uses`/`needs`, so recurse into
                    // anything that is not itself a presence clause.
                    self.collect_nested(clause, &owner, asks);
                }
            }
        }
    }

    /// [`collect`](Self::collect), for a declaration nothing requested reaches: it says what it offers and what it is, and asks nothing, so a `uses` of
    /// its is no request (`docs/decisions/decision_device-correspondence.md`).
    pub fn collect_unreached(&mut self, form: &Form) {
        let requested = self.requested.len();
        self.collect(form);
        self.requested.truncate(requested);
    }

    fn collect_nested(&mut self, clause: &Form, owner: &str, asks: bool) {
        match clause.head() {
            Some("needs") => {
                for name in names_in(clause) {
                    if asks {
                        self.requested.push(name.clone());
                    }
                    self.needs.entry(owner.to_string()).or_default().push(name);
                }
            }
            Some("uses") => {
                for name in names_in(clause) {
                    self.requested.push(name);
                }
            }
            _ => {
                for item in clause.items().iter().skip(1) {
                    if matches!(item, Form::List { .. }) {
                        self.collect_nested(item, owner, asks);
                    }
                }
            }
        }
    }

    fn record(&mut self, slot: &mut Slot, clause: &Form, owner: &str) {
        for (name, span) in names_in(clause) {
            let entry = Declaration {
                declared_by: owner.to_string(),
                span,
            };
            match slot {
                Slot::Offered => self.offered.entry(name).or_default().push(entry),
                Slot::Absent => self.absent.entry(name).or_default().push(entry),
            }
        }
    }

    /// What the description says about one fact.
    #[must_use]
    pub fn presence(&self, fact: &str) -> Presence {
        if self.offered.contains_key(fact) {
            Presence::Offered
        } else if self.absent.contains_key(fact) {
            Presence::Absent
        } else {
            Presence::Undescribed
        }
    }

    /// The transitive closure of everything the requested services depend on.
    ///
    /// Breadth-first from the requests, following `needs` edges. A name that needs nothing is a
    /// leaf fact; a name that needs others is a service.
    #[must_use]
    pub fn closure(&self) -> BTreeSet<String> {
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut queue: Vec<String> = self
            .requested
            .iter()
            .map(|(name, _)| name.clone())
            .collect();
        while let Some(name) = queue.pop() {
            if !seen.insert(name.clone()) {
                continue;
            }
            if let Some(edges) = self.needs.get(&name) {
                for (next, _) in edges {
                    if !seen.contains(next) {
                        queue.push(next.clone());
                    }
                }
            }
        }
        seen
    }

    /// Each fact of the closure, and what pulled it in: `None` for a fact the description requests itself,
    /// `Some(owner)` for one a fact already inside needs (leaf `M1.30`).
    ///
    /// The same traversal as [`Self::closure`], breadth-first so the recorded owner is on a shortest path
    /// from a request, and deterministic because `requested` and `needs` are ordered. Its facts are exactly
    /// the closure's — a unit test holds the two together.
    #[must_use]
    pub fn closure_origins(&self) -> Vec<(String, Option<String>)> {
        let mut origin: BTreeMap<String, Option<String>> = BTreeMap::new();
        let mut queue: std::collections::VecDeque<(String, Option<String>)> = self
            .requested
            .iter()
            .map(|(name, _)| (name.clone(), None))
            .collect();
        while let Some((name, by)) = queue.pop_front() {
            if origin.contains_key(&name) {
                continue;
            }
            if let Some(edges) = self.needs.get(&name) {
                for (next, _) in edges {
                    if !origin.contains_key(next) {
                        queue.push_back((next.clone(), Some(name.clone())));
                    }
                }
            }
            origin.insert(name, by);
        }
        origin.into_iter().collect()
    }

    /// Every name mentioned anywhere — declared, needed, or requested.
    fn universe(&self) -> BTreeSet<String> {
        let mut all: BTreeSet<String> = BTreeSet::new();
        all.extend(self.declared.iter().cloned());
        all.extend(self.offered.keys().cloned());
        all.extend(self.absent.keys().cloned());
        for (owner, edges) in &self.needs {
            all.insert(owner.clone());
            all.extend(edges.iter().map(|(name, _)| name.clone()));
        }
        all.extend(self.requested.iter().map(|(name, _)| name.clone()));
        all
    }

    /// Check presence and relevance.
    #[must_use]
    pub fn check(&self) -> PresenceReport {
        self.check_excluding(&BTreeSet::new())
    }

    /// Check presence and relevance, ignoring facts the active profile has already refused.
    ///
    /// ⭐ An out-of-profile capability is not a missing fact. Telling an author that
    /// `general-ipc` "is required and nothing describes it" invites them to go and describe it,
    /// which would be wasted work on a capability the profile refuses either way. The profile
    /// refusal is the message that helps; this one is noise beside it.
    #[must_use]
    pub fn check_excluding(&self, out_of_profile: &BTreeSet<String>) -> PresenceReport {
        let mut diagnostics = Vec::new();

        // ── F06: contradictions are rejected, never resolved by preference ───────────────────
        //
        // §5.3: "Reject contradictory declarations rather than choosing one." Choosing one
        // would be a silent decision about which half of the description the author meant, and
        // the author is the only one who knows. A contradiction is one declaration saying both of
        // itself, or of a part it composes: at two other declarations, an offer and an absence are
        // two statements of two devices, each judged on its own, as two values are (model §2 rule 1,
        // leaf `M3.4.8`,
        // `docs/decisions/decision_absence-per-provider.md`). Every declaration is compared, so a
        // contradiction written after another provider's statement is never hidden by it.
        for (fact, absences) in &self.absent {
            let Some(offers) = self.offered.get(fact) else {
                continue;
            };
            for absent in absences {
                let parts = self.composed_by(&absent.declared_by);
                for offered in offers {
                    let (message, offered_label) = if offered.declared_by == absent.declared_by {
                        (
                            format!(
                                "`{fact}` is declared both offered and absent by `{}`",
                                offered.declared_by
                            ),
                            format!("declared offered by `{}`", offered.declared_by),
                        )
                    } else if parts.contains(&offered.declared_by) {
                        // A declaration's absence covers what it is made of (model §2 rule 1): a platform
                        // composing a block that offers what it declares absent says both of itself.
                        (
                            format!(
                                "`{fact}` is declared absent by `{}` and offered by `{}`, which it composes",
                                absent.declared_by, offered.declared_by
                            ),
                            format!(
                                "offered by `{}`, part of `{}`",
                                offered.declared_by, absent.declared_by
                            ),
                        )
                    } else {
                        continue;
                    };
                    diagnostics.push(
                        Diagnostic::error(
                            "invalid-description",
                            message,
                            Label::new(
                                absent.span,
                                format!("declared absent by `{}`", absent.declared_by),
                            ),
                            "remove one declaration — a contradiction is not resolved by preferring \
                             one side, because only the author knows which half was meant",
                        )
                        .with_secondary(Label::new(offered.span, offered_label)),
                    );
                }
            }
        }

        let closure = self.closure();
        // The engine's adapters, by name: rules 2 and 3 yield where one may realize a fact (leaf `M3.2.4`).
        let kinds = adapter_paths();
        let partial = |fact: &str, kind: &str, declaration: &str, undescribed: &[String]| {
            let names = undescribed
                .iter()
                .map(|f| format!("`{f}`"))
                .collect::<Vec<_>>()
                .join(", ");
            Diagnostic::error(
                "missing-fact",
                format!(
                    "`{fact}` is required by this system and no declaration offers it; the engine's adapter `{kind}` \
                     offers it over a block or platform offering what it needs, and `{declaration}` offers part of \
                     that and leaves {names} undescribed"
                ),
                Label::new(self.request_span(fact), "required through this request"),
                format!(
                    "describe {names} at `{declaration}`: then `{kind}`'s path exists by name, and the search judges \
                     it by value — or declare `{fact}` offered where a provider has it"
                ),
            )
        };
        let nowhere = |kinds: &[AdapterPath]| {
            kinds
                .iter()
                .map(|k| {
                    format!(
                        "; the engine's adapter `{}` offers it over a block or platform offering {}, which none of \
                         this description's does",
                        k.kind,
                        k.beneath
                            .iter()
                            .map(|f| format!("`{f}`"))
                            .collect::<Vec<_>>()
                            .join(", ")
                    )
                })
                .collect::<String>()
        };

        // ── F04 / F05: relevance decides whether an unknown fact matters ─────────────────────
        let mut missing = Vec::new();
        for fact in &closure {
            if out_of_profile.contains(fact) {
                continue;
            }
            // An explicit absence is checked FIRST, before anything can satisfy the name: a
            // block that declares a capability absent does not stop being absent because it is
            // also declared.
            if self.presence(fact) == Presence::Absent {
                let declaration = &self.absent[fact][0];
                let adapters = match self.path(fact, &kinds) {
                    // An adapter's offer satisfies past a declared absence (`SR-H9`): the search's to judge.
                    Path::Whole => continue,
                    Path::Partial {
                        kind,
                        declaration: at,
                        undescribed,
                    } => {
                        diagnostics.push(partial(fact, &kind, &at, &undescribed).with_secondary(
                            Label::new(
                                declaration.span,
                                format!("declared absent by `{}`", declaration.declared_by),
                            ),
                        ));
                        continue;
                    }
                    Path::Nowhere(kinds) => nowhere(&kinds),
                    Path::NoKind => String::new(),
                };
                diagnostics.push(
                    Diagnostic::error(
                        "infeasible-configuration",
                        format!("`{fact}` is required by this system but declared absent"),
                        Label::new(self.request_span(fact), "required through this request"),
                        format!(
                            "either the requirement or the platform is wrong; an explicitly absent fact is a \
                             definite answer, not a gap to be filled in{adapters}"
                        ),
                    )
                    .with_secondary(Label::new(
                        declaration.span,
                        format!("declared absent by `{}`", declaration.declared_by),
                    )),
                );
                continue;
            }

            // A name that has outgoing `needs` edges is a service, satisfied by its parts
            // rather than by being offered itself. A name this description DECLARES is
            // satisfied by its declaration — its own obligations are checked there.
            if self.needs.contains_key(fact) || self.declared.contains(fact) {
                continue;
            }
            match self.presence(fact) {
                Presence::Offered => {}
                Presence::Absent => unreachable!("absence was handled above"),
                Presence::Undescribed => {
                    let adapters = match self.path(fact, &kinds) {
                        Path::Whole => continue,
                        Path::Partial {
                            kind,
                            declaration,
                            undescribed,
                        } => {
                            missing.push(fact.clone());
                            diagnostics.push(partial(fact, &kind, &declaration, &undescribed));
                            continue;
                        }
                        Path::Nowhere(kinds) => nowhere(&kinds),
                        Path::NoKind => String::new(),
                    };
                    missing.push(fact.clone());
                    diagnostics.push(Diagnostic::error(
                        "missing-fact",
                        format!("`{fact}` is required by this system and nothing describes it"),
                        Label::new(self.request_span(fact), "required through this request"),
                        format!(
                            "declare `{fact}` as offered or absent on the platform — it is inside \
                             the dependency closure of what this system requests, so no decision \
                             that depends on it can be made{adapters}"
                        ),
                    ));
                }
            }
        }

        let outside_closure: Vec<String> = self
            .universe()
            .into_iter()
            .filter(|name| !closure.contains(name))
            .collect();

        PresenceReport {
            closure: closure.into_iter().collect(),
            outside_closure,
            missing,
            pulled_in_by: self.closure_origins(),
            diagnostics,
        }
    }

    /// Where an engine path for `fact` stands among this description's blocks and platforms (record §7).
    fn path(&self, fact: &str, kinds: &[AdapterPath]) -> Path {
        let adding: Vec<&AdapterPath> = kinds
            .iter()
            .filter(|k| k.adds.iter().any(|a| a == fact))
            .collect();
        if adding.is_empty() {
            return Path::NoKind;
        }
        let stated = |map: &BTreeMap<String, Vec<Declaration>>, declaration: &str, name: &str| {
            map.get(name)
                .is_some_and(|ds| ds.iter().any(|d| d.declared_by == declaration))
        };
        let mut partial = None;
        for kind in &adding {
            for declaration in self.composes.keys() {
                let offered: Vec<&String> = kind
                    .beneath
                    .iter()
                    .filter(|b| stated(&self.offered, declaration, b))
                    .collect();
                if offered.len() == kind.beneath.len() {
                    return Path::Whole;
                }
                let undescribed: Vec<String> = kind
                    .beneath
                    .iter()
                    .filter(|b| !stated(&self.offered, declaration, b))
                    .cloned()
                    .collect();
                let none_absent = undescribed
                    .iter()
                    .all(|b| !stated(&self.absent, declaration, b));
                if partial.is_none() && !offered.is_empty() && none_absent {
                    partial = Some(Path::Partial {
                        kind: kind.kind.clone(),
                        declaration: declaration.clone(),
                        undescribed,
                    });
                }
            }
        }
        partial.unwrap_or_else(|| Path::Nowhere(adding.into_iter().cloned().collect()))
    }

    /// Every block and platform `name` composes, at any composition depth.
    fn composed_by(&self, name: &str) -> BTreeSet<String> {
        let mut seen = BTreeSet::new();
        let mut next: Vec<&String> = self.composes.get(name).into_iter().flatten().collect();
        while let Some(part) = next.pop() {
            if part != name && self.composes.contains_key(part) && seen.insert(part.clone()) {
                next.extend(self.composes.get(part).into_iter().flatten());
            }
        }
        seen
    }

    fn request_span(&self, fact: &str) -> Span {
        for edges in self.needs.values() {
            for (name, span) in edges {
                if name == fact {
                    return *span;
                }
            }
        }
        self.requested
            .iter()
            .find(|(name, _)| name == fact)
            .map_or_else(
                || {
                    self.requested
                        .first()
                        .map_or(Span::new(eadl_front::SourceId(0), 0, 0), |(_, span)| *span)
                },
                |(_, span)| *span,
            )
    }
}

enum Slot {
    Offered,
    Absent,
}

/// What a block or a platform composes directly: every name a `uses` list of its holds, wherever the declaration
/// writes one — at its top, inside `requires`, inside a nested `requires` — but inside `offers`, `absent` or
/// `refines`, which name facts and a refinement target. A block's or a platform's `uses` alone is composition (model §8
/// rules 1 and 5); the one reading model §2 rule 1 and §3 rule 3 share (leaf `M3.4.8`).
#[must_use]
pub fn composition(form: &Form) -> Vec<String> {
    fn walk(form: &Form, out: &mut Vec<String>) {
        match form.head() {
            Some("offers" | "absent" | "refines") => {}
            Some("uses") => out.extend(names_in(form).into_iter().map(|(name, _)| name)),
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

/// The names a clause mentions: a bare symbol argument, or the head of a nested form.
///
/// `(offers counter-width)` and `(offers (counter-width 32 bit))` both name `counter-width`, so
/// an author can write the bare fact or the fact with its parameters and mean the same thing
/// about presence.
/// The clauses whose operands name other declarations: `collect` reads their operands as names, and
/// `docs/semantics/reference.md` §6 rule 10 resolves exactly these positions in an elaborated module tree.
/// One list, so the pass that reads a name and the rule that resolves it cannot disagree about where names are.
pub const NAME_CLAUSES: &[&str] = &["uses", "needs", "refines"];

fn names_in(clause: &Form) -> Vec<(String, Span)> {
    clause
        .items()
        .iter()
        .skip(1)
        .filter_map(|item| match item {
            Form::Symbol { name, span } => Some((name.clone(), *span)),
            Form::List { .. } => item.head().map(|head| (head.to_string(), item.span())),
            _ => None,
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::{FactMap, Presence};
    use eadl_front::{read, SourceMap};
    use std::collections::BTreeSet;

    fn map_of(text: &str) -> (FactMap, SourceMap) {
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", text).expect("small");
        let (document, diagnostics) = read(&sources, id);
        assert!(
            !diagnostics.has_errors(),
            "{}",
            diagnostics.render(&sources)
        );
        let mut map = FactMap::new();
        for form in &document.forms {
            map.collect(form);
        }
        (map, sources)
    }

    #[test]
    fn closure_origins_hold_exactly_the_closure_and_say_what_pulled_each_fact_in() {
        // `M1.30`'s report reads the origins, and the presence verdicts read the closure: two traversals of one
        // graph, held together here so the report can never list a fact the verdicts did not judge.
        let (map, _) = map_of(
            "(defblock timer.counter (offers counter-width)) \
             (defblock other.engine (offers transfer-engine) (needs descriptor-format)) \
             (defservice time.monotonic (requires (needs counter-width))) \
             (defsystem s (requires (uses time.monotonic)))",
        );
        let origins = map.closure_origins();
        let keys: BTreeSet<String> = origins.iter().map(|(fact, _)| fact.clone()).collect();
        assert_eq!(keys, map.closure());
        assert!(origins.contains(&("time.monotonic".to_string(), None)));
        assert!(origins.contains(&(
            "counter-width".to_string(),
            Some("time.monotonic".to_string())
        )));
        assert!(
            !keys.contains("descriptor-format"),
            "an unrequested engine's need is outside"
        );
    }

    #[test]
    fn presence_distinguishes_three_states() {
        let (map, _) = map_of("(defblock b (offers alpha) (absent beta))");
        assert_eq!(map.presence("alpha"), Presence::Offered);
        assert_eq!(map.presence("beta"), Presence::Absent);
        assert_eq!(map.presence("gamma"), Presence::Undescribed);
    }

    #[test]
    fn a_parameterized_offer_names_the_same_fact_as_a_bare_one() {
        let (map, _) = map_of("(defblock b (offers (counter-width 32 bit)))");
        assert_eq!(map.presence("counter-width"), Presence::Offered);
    }

    #[test]
    fn the_closure_follows_needs_edges_transitively() {
        let (map, _) = map_of(
            "(defsystem s (uses time.deadline))\n\
             (defservice time.deadline (needs time.monotonic compare-unit))\n\
             (defservice time.monotonic (needs counter-width))",
        );
        let closure = map.closure();
        for name in [
            "time.deadline",
            "time.monotonic",
            "compare-unit",
            "counter-width",
        ] {
            assert!(closure.contains(name), "`{name}` missing from {closure:?}");
        }
    }

    #[test]
    fn what_a_system_needs_is_requested_wherever_it_writes_it_and_only_a_system() {
        for system in [
            "(defsystem s (requires (needs sha-accel)))",
            "(defsystem s (needs sha-accel))",
            "(defsystem s (task t (needs sha-accel)))",
            "(defsystem s (platform (needs sha-accel)))",
            "(defservice hash (requires (needs sha-accel))) (defsystem s (requires (needs hash)))",
            "(defservice hash (requires (needs sha-accel))) (defsystem s (requires (uses hash)))",
        ] {
            let (map, _) = map_of(system);
            assert!(map.closure().contains("sha-accel"), "{system}");
            assert!(
                map.check()
                    .diagnostics
                    .iter()
                    .any(|d| d.code == "missing-fact"),
                "{system}"
            );
        }
        // A service nothing requests asks nothing, as before.
        let (map, _) = map_of("(defservice hash (requires (needs sha-accel)))");
        assert!(map.closure().is_empty());
    }

    #[test]
    fn a_declaration_nothing_reaches_says_what_it_offers_and_asks_nothing() {
        let mut sources = SourceMap::new();
        let id = sources
            .add("t.eadl", "(defservice unused (offers sha-accel) (requires (uses a.block) (needs other-fact))) (defsystem s (requires (needs sha-accel)))")
            .expect("small");
        let (doc, _) = read(&sources, id);
        let mut map = FactMap::new();
        map.collect_unreached(&doc.forms[0]);
        map.collect(&doc.forms[1]);
        assert_eq!(map.presence("sha-accel"), Presence::Offered);
        assert_eq!(
            map.closure(),
            BTreeSet::from(["sha-accel".to_string()]),
            "neither its `uses` nor its `needs` is asked"
        );
        assert!(map.check().diagnostics.is_empty());
    }

    #[test]
    fn a_declared_block_satisfies_a_composition_reference() {
        // ⭐ Found by the first real example, not by inspection: a platform's
        // `(requires (uses timer.counter))` names a block that is declared right there. Treating
        // every `uses` target as a leaf fact reported `missing-fact: console.uart` on a
        // description that was complete.
        let (map, _) = map_of(
            "(defsystem s (uses soc.playground))\n\
             (defplatform soc.playground (requires (uses timer.counter console.uart)))\n\
             (defblock timer.counter (offers counter-width))\n\
             (defblock console.uart (offers observable-output))",
        );
        let report = map.check();
        assert!(report.is_admissible(), "{:?}", report.diagnostics);
    }

    #[test]
    fn a_declared_name_that_is_also_absent_is_still_infeasible() {
        // Being declared does not repeal an explicit absence. Order matters here: the absence
        // check runs first, so a block that declares a capability absent cannot satisfy a
        // requirement for it merely by existing.
        let (map, _) = map_of(
            "(defsystem s (uses low-power-timer))\n\
             (defblock low-power-timer (offers x))\n\
             (defblock other (absent low-power-timer))",
        );
        let report = map.check();
        assert!(!report.is_admissible());
        assert!(report
            .diagnostics
            .iter()
            .any(|d| d.code == "infeasible-configuration"));
    }

    #[test]
    fn a_service_is_not_itself_required_to_be_offered() {
        // A service is satisfied by its parts; requiring it to be `offered` as well would make
        // every composite requirement unsatisfiable.
        let (map, _) = map_of(
            "(defsystem s (uses time.monotonic))\n\
             (defservice time.monotonic (needs counter-width))\n\
             (defblock b (offers counter-width))",
        );
        let report = map.check();
        assert!(report.is_admissible(), "{:?}", report.diagnostics);
    }
}
