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

use eadl_front::{read, Diagnostic, Form, Label, SourceMap, Span, MOST_ERRORS};

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
    /// enter the closure as it may be bound (`composed`). Treating every `uses` target as a leaf
    /// fact made the first real example fail with `missing-fact: console.uart` — found by the
    /// example suite, not by inspection.
    declared: BTreeSet<String>,
    /// `name` → the facts and services it needs.
    needs: BTreeMap<String, Vec<(String, Span)>>,
    /// What the system requests: every `uses` but a block's or a platform's, which composes, and a system's own `needs`, its tasks and its clauses among it.
    /// A `needs` of anything else is an edge, followed only once what wrote it is reached.
    requested: Vec<(String, Span)>,
    /// Each name a block's or a platform's `uses` composes. It requests nothing: a part enters the closure as it may be bound, offering a fact inside it, and
    /// one that offers none fails nothing (`ROADMAP.md` §5.3; leaf `M3.4.17.3`). A name no declaration bears is requested all the same, so it is refused.
    composed: Vec<(String, Span)>,
    /// Each block's and platform's direct composition, by [`composition`] (leaf `M3.4.8`).
    composes: BTreeMap<String, Vec<String>>,
    /// What a declaration read for its name alone, or its asking alone, says beside that: it answers nothing, and stays visible (model §2 rule 4).
    mentioned: BTreeSet<String>,
    /// The declarations that answer nothing in this map: the blocks and platforms that are none of the hardware of the system it is read for, by
    /// [`collect_silent`](Self::collect_silent), and what is no block or platform, by [`collect_unanswering`](Self::collect_unanswering). What one offers or declares
    /// absent answers no need there, and it is still declared, compared with itself, and visible (model §8 rule 7, leaves `M3.4.16` and `M3.4.17.2`).
    silent: BTreeSet<String>,
    /// Each fact the capability vocabulary derives, with the facts its rule requires, by [`derive`](Self::derive): no declaration states one, and the search computes it
    /// where one provider offers each of those (model §2 rule 2, leaf `M3.4.17.1`).
    derived: BTreeMap<String, Vec<String>>,
    /// Each fact an engine's adapter adds, with the facts its `beneath` names, by [`realize`](Self::realize): a part offering one may be bound for the
    /// fact, an instance over it binding it (leaf `M3.4.17.6`).
    adapted: BTreeMap<String, Vec<String>>,
    /// Whether an offer's value is read, by [`read_values`](Self::read_values): an offer of `false` then offers nothing, and is kept apart to be named.
    reads_values: bool,
    /// Each offer of `false`, where values are read: what says a fact is `false` there has none of it (leaf `M3.4.17.2`).
    offered_false: BTreeMap<String, Vec<Declaration>>,
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
    /// Each refusal of a required fact, rules 2 and 3's, with the fact it is of: those of `diagnostics` that are no contradiction.
    pub judged: Vec<(String, Diagnostic)>,
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
        let composing = matches!(form.head(), Some("defblock" | "defplatform"));
        if composing {
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
                        if composing {
                            self.composed.push(name);
                        } else {
                            self.requested.push(name);
                        }
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
                    self.collect_nested(clause, &owner, asks, composing);
                }
            }
        }
    }

    /// That `form` is a declaration: its name is no fact, and all else it says judges nothing here and stays visible.
    pub fn declare(&mut self, form: &Form) {
        if let Some(name) = form.items().get(1).and_then(Form::as_symbol) {
            self.declared.insert(name.to_string());
        }
        let mut alone = Self::new();
        alone.collect(form);
        self.mentioned.extend(alone.universe());
    }

    /// That `name` is a declaration's, the declaration no longer read: a block a target replaced.
    pub fn declare_name(&mut self, name: &str) {
        self.declared.insert(name.to_string());
    }

    /// That an offer's value is read from here on: `(name false)` offers nothing. For a map of facts the search binds nothing for, which presence alone judges
    /// (leaf `M3.4.17.2`); a fact the search binds it judges by value itself.
    pub fn read_values(&mut self) {
        self.reads_values = true;
    }

    /// [`collect`](Self::collect), for a declaration that is no block and no platform, where presence alone judges: a service, a policy, a system. It asks, and it is
    /// what it is; what it offers or declares absent answers no need, the search binding nothing to one, and stays visible (leaf `M3.4.17.2`).
    pub fn collect_unanswering(&mut self, form: &Form) {
        self.collect(form);
        if let Some(name) = form.items().get(1).and_then(Form::as_symbol) {
            self.silent.insert(name.to_string());
        }
    }

    /// That the capability vocabulary derives `fact` of these facts, as a caller that has read it says: such a fact is described where one block or platform offers
    /// each of them, the search deriving it there and judging it by value.
    pub fn derive(&mut self, fact: &str, of: &[String]) {
        self.derived.insert(fact.to_string(), of.to_vec());
    }

    /// That an engine's adapter adds `fact` over a block or a platform offering what `beneath` names, as a caller that has read the catalog says: such a
    /// part may be bound for `fact`, an instance over it binding it, so its own needs are followed as the search counts them (leaf `M3.4.17.6`).
    pub fn realize(&mut self, fact: &str, beneath: &[String]) {
        let known = self.adapted.entry(fact.to_string()).or_default();
        for name in beneath {
            if !known.contains(name) {
                known.push(name.clone());
            }
        }
    }

    /// [`collect`](Self::collect), for a declaration whose offers answer nothing where the machine is another's: a service or a policy, to which the search binds nothing. It asks, and is
    /// what it is (`docs/decisions/decision_device-correspondence.md`).
    pub fn collect_asking(&mut self, form: &Form) {
        let offered = std::mem::take(&mut self.offered);
        self.collect(form);
        self.mentioned.extend(self.offered.keys().cloned());
        self.offered = offered;
    }

    fn collect_nested(&mut self, clause: &Form, owner: &str, asks: bool, composing: bool) {
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
                    if composing {
                        self.composed.push(name);
                    } else {
                        self.requested.push(name);
                    }
                }
            }
            _ => {
                for item in clause.items().iter().skip(1) {
                    if matches!(item, Form::List { .. }) {
                        self.collect_nested(item, owner, asks, composing);
                    }
                }
            }
        }
    }

    fn record(&mut self, slot: &mut Slot, clause: &Form, owner: &str) {
        for (name, span, says_false) in stated_in(clause) {
            let entry = Declaration {
                declared_by: owner.to_string(),
                span,
            };
            match slot {
                Slot::Offered if says_false && self.reads_values => {
                    self.offered_false.entry(name).or_default().push(entry);
                }
                Slot::Offered => self.offered.entry(name).or_default().push(entry),
                Slot::Absent => self.absent.entry(name).or_default().push(entry),
            }
        }
    }

    /// [`collect`](Self::collect), for a block or a platform that is none of the hardware of the system this map is read for (model §8 rule 7; leaf `M3.4.16`): what it offers or
    /// declares absent answers no need there, and what it names or needs asks nothing. It is still a declaration: its own contradiction is one, and all it says stays visible.
    pub fn collect_silent(&mut self, form: &Form) {
        let (needs, requested, composed) = (
            std::mem::take(&mut self.needs),
            std::mem::take(&mut self.requested),
            std::mem::take(&mut self.composed),
        );
        self.collect(form);
        let asked: Vec<String> = self
            .needs
            .iter()
            .flat_map(|(owner, edges)| {
                std::iter::once(owner).chain(edges.iter().map(|(name, _)| name))
            })
            .chain(self.requested.iter().map(|(name, _)| name))
            .chain(self.composed.iter().map(|(name, _)| name))
            .cloned()
            .collect();
        self.mentioned.extend(asked);
        self.needs = needs;
        self.requested = requested;
        self.composed = composed;
        if let Some(name) = form.items().get(1).and_then(Form::as_symbol) {
            self.silent.insert(name.to_string());
        }
    }

    /// Each statement of `fact` among `stated`, the offers or the absences, that a declaration answering here makes.
    fn answering<'m>(
        &'m self,
        stated: &'m BTreeMap<String, Vec<Declaration>>,
        fact: &str,
    ) -> impl Iterator<Item = &'m Declaration> {
        stated
            .get(fact)
            .into_iter()
            .flatten()
            .filter(|declaration| !self.silent.contains(&declaration.declared_by))
    }

    /// What the description says about one fact, by the declarations that answer here.
    #[must_use]
    pub fn presence(&self, fact: &str) -> Presence {
        if self.answering(&self.offered, fact).next().is_some() {
            Presence::Offered
        } else if self.answering(&self.absent, fact).next().is_some() {
            Presence::Absent
        } else {
            Presence::Undescribed
        }
    }

    /// What the closure starts from: what the system requests, and each composed name no declaration bears.
    fn roots(&self) -> Vec<String> {
        self.requested
            .iter()
            .chain(
                self.composed
                    .iter()
                    .filter(|(name, _)| !self.declared.contains(name)),
            )
            .map(|(name, _)| name.clone())
            .collect()
    }

    /// What `name`, inside the closure, brings into it: each name it needs, then each block or platform that may be bound for it, offering it, or offering a
    /// fact the vocabulary's rule derives it of. Such a part's own needs are then followed, as its preconditions: a part that offers nothing inside fails
    /// nothing, and stays visible (`ROADMAP.md` §5.3; leaf `M3.4.17.3`). Which one the search binds is the search's, so each that may be is read, a precondition
    /// of one it does not bind refused all the same. Each comes with whether it offers `name`, a part, rather than being needed by it.
    fn brought_by(&self, name: &str) -> Vec<(String, bool)> {
        let mut next: Vec<(String, bool)> = self
            .needs
            .get(name)
            .into_iter()
            .flatten()
            .map(|(fact, _)| (fact.clone(), false))
            .collect();
        // The facts `name` rests on: each its vocabulary rule requires, and each an adapter adding it needs beneath, to a fixed point (leaf `M3.4.17.6`).
        let mut rule: Vec<&str> = Vec::new();
        let mut todo = vec![name];
        while let Some(fact) = todo.pop() {
            let under = self
                .derived
                .get(fact)
                .into_iter()
                .chain(self.adapted.get(fact))
                .flatten()
                .map(String::as_str);
            for input in under {
                if input != name && !rule.contains(&input) {
                    rule.push(input);
                    todo.push(input);
                }
            }
        }
        for fact in std::iter::once(name).chain(rule) {
            for offer in self.offered.get(fact).into_iter().flatten() {
                let part = &offer.declared_by;
                if self.composes.contains_key(part)
                    && !self.silent.contains(part)
                    && !next.iter().any(|(named, _)| named == part)
                {
                    next.push((part.clone(), true));
                }
            }
        }
        next
    }

    /// The transitive closure of everything the requested services depend on.
    ///
    /// Breadth-first from the requests, following `needs` edges and each part that may be bound ([`brought_by`](Self::brought_by)). A name
    /// that needs nothing is a leaf fact; a name that needs others is a service.
    #[must_use]
    pub fn closure(&self) -> BTreeSet<String> {
        // Each name is queued once: a part offering many facts inside, each taking it into the queue apart, held its name once for each (`crates/archogen-api/tests/cost.rs`).
        let mut seen: BTreeSet<String> = BTreeSet::new();
        let mut queue: Vec<String> = Vec::new();
        for name in self.roots() {
            if seen.insert(name.clone()) {
                queue.push(name);
            }
        }
        while let Some(name) = queue.pop() {
            for (next, _) in self.brought_by(&name) {
                if seen.insert(next.clone()) {
                    queue.push(next);
                }
            }
        }
        seen
    }

    /// Each fact of the closure, and what pulled it in: `None` for a fact the description requests itself,
    /// `Some(owner)` for one a fact already inside needs (leaf `M1.30`). A part that may be bound is pulled in by what needs the fact it offers, so
    /// `None` where the description requests that fact itself (leaf `M3.4.17.3`): a fact needs no provider, its requester does.
    ///
    /// The same traversal as [`Self::closure`], breadth-first so the recorded owner is on a shortest path
    /// from a request, and deterministic because `requested` and `needs` are ordered. Its facts are exactly
    /// the closure's — a unit test holds the two together.
    #[must_use]
    pub fn closure_origins(&self) -> Vec<(String, Option<String>)> {
        let mut origin: BTreeMap<String, Option<String>> = BTreeMap::new();
        // Each name queued once, as in `closure`: the first to queue it is the first taken off, so it decides what pulled the name in, as before.
        let mut queued: BTreeSet<String> = BTreeSet::new();
        let mut queue: std::collections::VecDeque<(String, Option<String>)> = self
            .roots()
            .into_iter()
            .filter(|name| queued.insert(name.clone()))
            .map(|name| (name, None))
            .collect();
        while let Some((name, by)) = queue.pop_front() {
            if origin.contains_key(&name) {
                continue;
            }
            for (next, offers) in self.brought_by(&name) {
                if !origin.contains_key(&next) && queued.insert(next.clone()) {
                    let pulled_by = if offers {
                        by.clone()
                    } else {
                        Some(name.clone())
                    };
                    queue.push_back((next, pulled_by));
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
        all.extend(self.offered_false.keys().cloned());
        for (owner, edges) in &self.needs {
            all.insert(owner.clone());
            all.extend(edges.iter().map(|(name, _)| name.clone()));
        }
        all.extend(self.requested.iter().map(|(name, _)| name.clone()));
        all.extend(self.composed.iter().map(|(name, _)| name.clone()));
        all.extend(self.mentioned.iter().cloned());
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
        // Each one is `invalid-description`, so what is held back moves no verdict, and a fact offered and declared absent n times each is n² of them.
        'contradictions: for (fact, absences) in &self.absent {
            let Some(offers) = self.offered.get(fact) else {
                continue;
            };
            for absent in absences {
                let parts = self.composed_by(&absent.declared_by);
                for offered in offers {
                    if diagnostics.len() > MOST_ERRORS {
                        break 'contradictions;
                    }
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
        let quoted = |facts: &[String]| {
            facts
                .iter()
                .map(|f| format!("`{f}`"))
                .collect::<Vec<_>>()
                .join(", ")
        };
        let derived_in_part = |fact: &str,
                               of: &[String],
                               declaration: &str,
                               undescribed: &[String]| {
            Diagnostic::error(
                "missing-fact",
                format!(
                    "`{fact}` is required by this system and no declaration offers it; the vocabulary derives it of {}, at one block \
                     or platform offering each, and `{declaration}` offers part of that and leaves {} undescribed",
                    quoted(of),
                    quoted(undescribed)
                ),
                Label::new(self.request_span(fact), "required through this request"),
                format!(
                    "describe {} at `{declaration}`: then the search derives `{fact}` there and judges it by value",
                    quoted(undescribed)
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
        let mut judged = Vec::new();
        for fact in &closure {
            let said = diagnostics.len();
            'judged: {
                if out_of_profile.contains(fact) {
                    break 'judged;
                }
                // An explicit absence is checked FIRST, before anything can satisfy the name: a
                // block that declares a capability absent does not stop being absent because it is
                // also declared.
                if self.presence(fact) == Presence::Absent {
                    let Some(declaration) = self.answering(&self.absent, fact).next() else {
                        break 'judged;
                    };
                    let adapters = match self.path(fact, &kinds) {
                        // An adapter's offer satisfies past a declared absence (`SR-H9`): the search's to judge.
                        Path::Whole => break 'judged,
                        Path::Partial {
                            kind,
                            declaration: at,
                            undescribed,
                        } => {
                            diagnostics.push(
                                partial(fact, &kind, &at, &undescribed).with_secondary(Label::new(
                                    declaration.span,
                                    format!("declared absent by `{}`", declaration.declared_by),
                                )),
                            );
                            break 'judged;
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
                    break 'judged;
                }

                // A name that has outgoing `needs` edges is a service, satisfied by its parts
                // rather than by being offered itself. A name this description DECLARES is
                // satisfied by its declaration — its own obligations are checked there.
                if self.needs.contains_key(fact) || self.declared.contains(fact) {
                    break 'judged;
                }
                match self.presence(fact) {
                    Presence::Offered => {}
                    Presence::Absent => unreachable!("absence was handled above"),
                    Presence::Undescribed => {
                        // An offer of `false` says its block has none, and answers no need (leaf `M3.4.17.2`).
                        if let Some(declaration) = self.answering(&self.offered_false, fact).next()
                        {
                            missing.push(fact.clone());
                            diagnostics.push(
                                Diagnostic::error(
                                    "missing-fact",
                                    format!(
                                        "`{fact}` is required by this system and `{}` offers it `false`, which says it has none",
                                        declaration.declared_by
                                    ),
                                    Label::new(self.request_span(fact), "required through this request"),
                                    format!(
                                        "an offer of `false` answers no need: offer `{fact}` where a block or platform of the system has it, or \
                                         declare it `absent` at `{}`, which is a definite answer",
                                        declaration.declared_by
                                    ),
                                )
                                .with_secondary(Label::new(declaration.span, "offered `false` here")),
                            );
                            break 'judged;
                        }
                        // A fact the vocabulary derives is stated by no one: it is described where one block or platform offers each fact its rule requires, the
                        // search deriving it there and judging it by value (leaf `M3.4.17.1`).
                        let mut derivation = String::new();
                        if let Some(of) = self.derived.get(fact) {
                            match self.offering(of) {
                                Offering::Whole => break 'judged,
                                Offering::Partial {
                                    declaration,
                                    undescribed,
                                } => {
                                    missing.push(fact.clone());
                                    diagnostics.push(derived_in_part(
                                        fact,
                                        of,
                                        &declaration,
                                        &undescribed,
                                    ));
                                    break 'judged;
                                }
                                Offering::Nowhere => {
                                    derivation = format!(
                                        "; the vocabulary derives it of {}, which no one block or platform of this description offers",
                                        quoted(of)
                                    );
                                }
                            }
                        }
                        let adapters = match self.path(fact, &kinds) {
                            Path::Whole => break 'judged,
                            Path::Partial {
                                kind,
                                declaration,
                                undescribed,
                            } => {
                                missing.push(fact.clone());
                                diagnostics.push(partial(fact, &kind, &declaration, &undescribed));
                                break 'judged;
                            }
                            Path::Nowhere(kinds) => nowhere(&kinds) + &derivation,
                            Path::NoKind => derivation,
                        };
                        missing.push(fact.clone());
                        // What offers it and is none of the system's hardware is named: the fact is described, on a machine the system does not have.
                        let elsewhere: Vec<String> = self
                            .offered
                            .get(fact)
                            .into_iter()
                            .flatten()
                            .map(|declaration| format!("`{}`", declaration.declared_by))
                            .collect();
                        if !elsewhere.is_empty() {
                            let names = elsewhere.join(", ");
                            diagnostics.push(Diagnostic::error(
                            "missing-fact",
                            format!(
                                "`{fact}` is required by this system and nothing of its hardware describes it: {names} \
                                 offers it, and is none of the blocks and platforms the system has"
                            ),
                            Label::new(self.request_span(fact), "required through this request"),
                            format!(
                                "a system's hardware is what its `platform` clause, its own `uses` and `needs` or its \
                                 tasks' name, and what those name in turn: name {names} there, a platform's \
                                 `(requires (uses …))` among it, or declare `{fact}` on a block or platform the system \
                                 has{adapters}"
                            ),
                        ));
                            break 'judged;
                        }
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
            judged.extend(
                diagnostics[said..]
                    .iter()
                    .map(|diagnostic| (fact.clone(), diagnostic.clone())),
            );
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
            judged,
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
            for declaration in self
                .composes
                .keys()
                .filter(|name| !self.silent.contains(*name))
            {
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

    /// Where one block or platform that answers here offers each of `facts`: wholly; in part, the first that offers some and declares none of the rest absent, with
    /// what it leaves undescribed; or nowhere.
    fn offering(&self, facts: &[String]) -> Offering {
        let stated = |map: &BTreeMap<String, Vec<Declaration>>, declaration: &str, name: &str| {
            map.get(name)
                .is_some_and(|ds| ds.iter().any(|d| d.declared_by == declaration))
        };
        let mut partial = None;
        for declaration in self
            .composes
            .keys()
            .filter(|name| !self.silent.contains(*name))
        {
            let undescribed: Vec<String> = facts
                .iter()
                .filter(|f| !stated(&self.offered, declaration, f))
                .cloned()
                .collect();
            if undescribed.is_empty() {
                return Offering::Whole;
            }
            let none_absent = undescribed
                .iter()
                .all(|f| !stated(&self.absent, declaration, f));
            if partial.is_none() && undescribed.len() < facts.len() && none_absent {
                partial = Some(Offering::Partial {
                    declaration: declaration.clone(),
                    undescribed,
                });
            }
        }
        partial.unwrap_or(Offering::Nowhere)
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

/// Where one block or platform offers each fact of a list, by [`FactMap::offering`].
enum Offering {
    /// One offers each.
    Whole,
    /// One offers some, and leaves these undescribed.
    Partial {
        declaration: String,
        undescribed: Vec<String>,
    },
    /// None offers any of it beside an absence of the rest.
    Nowhere,
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

/// A system's hardware (model §8 rule 7): the blocks and platforms its `platform` clause, its own `uses` and `needs` or its
/// tasks' name, and every block or platform those name in turn through `uses` or `needs`, a clause of their own or inside
/// `requires`, at any depth. A declaration of another kind ends the path, and `refines` is no edge. The search reads the
/// same of a system, and `crates/eadl-resolve/tests/system_reading.rs` holds the two equal over every tracked description and
/// every generated shape.
#[must_use]
pub fn hardware(system: &Form, declarations: &[Form]) -> BTreeSet<String> {
    fn named(form: &Form, out: &mut Vec<String>) {
        match form.head() {
            Some("offers" | "absent" | "refines") => {}
            Some("uses" | "needs") => out.extend(
                form.items()
                    .iter()
                    .skip(1)
                    .filter_map(Form::as_symbol)
                    .map(str::to_string),
            ),
            _ => {
                for item in form.items().iter().skip(1) {
                    if matches!(item, Form::List { .. }) {
                        named(item, out);
                    }
                }
            }
        }
    }
    let declared: BTreeMap<&str, &Form> = declarations
        .iter()
        .filter(|form| matches!(form.head(), Some("defblock" | "defplatform")))
        .filter_map(|form| Some((form.items().get(1)?.as_symbol()?, form)))
        .collect();
    let mut found = BTreeSet::new();
    let mut next = Vec::new();
    named(system, &mut next);
    while let Some(name) = next.pop() {
        if let Some(form) = declared.get(name.as_str()) {
            if found.insert(name) {
                named(form, &mut next);
            }
        }
    }
    found
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

/// As [`names_in`], with whether an item says `false` of its name, `(name false)`.
fn stated_in(clause: &Form) -> Vec<(String, Span, bool)> {
    clause
        .items()
        .iter()
        .skip(1)
        .filter_map(|item| match item {
            Form::Symbol { name, span } => Some((name.clone(), *span, false)),
            Form::List { .. } => item.head().map(|head| {
                let says_false =
                    matches!(item.items(), [_, Form::Symbol { name, .. }] if name == "false");
                (head.to_string(), item.span(), says_false)
            }),
            _ => None,
        })
        .collect()
}

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
    fn a_declaration_that_only_asks_answers_nothing_it_offers() {
        let mut sources = SourceMap::new();
        let id = sources
            .add("t.eadl", "(defservice driver (offers sha-accel) (requires (needs other-fact))) (defsystem s (requires (uses driver) (needs sha-accel)))")
            .expect("small");
        let (doc, _) = read(&sources, id);
        let mut map = FactMap::new();
        map.collect_asking(&doc.forms[0]);
        map.collect(&doc.forms[1]);
        assert_eq!(map.presence("sha-accel"), Presence::Undescribed);
        assert!(map.closure().contains("other-fact"), "it still asks");
        assert_eq!(
            map.check()
                .diagnostics
                .iter()
                .filter(|d| d.code == "missing-fact")
                .count(),
            2
        );
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
