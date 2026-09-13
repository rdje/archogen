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

use eadl_front::{Diagnostic, Form, Label, Span};

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
    offered: BTreeMap<String, Declaration>,
    absent: BTreeMap<String, Declaration>,
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
    /// What the system requests.
    requested: Vec<(String, Span)>,
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

        for clause in form.items().iter().skip(1) {
            let Some(head) = clause.head() else { continue };
            match head {
                "offers" => self.record(&mut Slot::Offered, clause, &owner),
                "absent" => self.record(&mut Slot::Absent, clause, &owner),
                "needs" => {
                    for name in names_in(clause) {
                        self.needs.entry(owner.clone()).or_default().push(name);
                    }
                }
                "uses" => {
                    for name in names_in(clause) {
                        self.requested.push(name);
                    }
                }
                _ => {
                    // `requires` and other clauses can nest `uses`/`needs`, so recurse into
                    // anything that is not itself a presence clause.
                    self.collect_nested(clause, &owner);
                }
            }
        }
    }

    fn collect_nested(&mut self, clause: &Form, owner: &str) {
        match clause.head() {
            Some("needs") => {
                for name in names_in(clause) {
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
                        self.collect_nested(item, owner);
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
                Slot::Offered => {
                    self.offered.entry(name).or_insert(entry);
                }
                Slot::Absent => {
                    self.absent.entry(name).or_insert(entry);
                }
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
        let mut diagnostics = Vec::new();

        // ── F06: contradictions are rejected, never resolved by preference ───────────────────
        //
        // §5.3: "Reject contradictory declarations rather than choosing one." Choosing one
        // would be a silent decision about which half of the description the author meant, and
        // the author is the only one who knows.
        for (fact, offered) in &self.offered {
            if let Some(absent) = self.absent.get(fact) {
                diagnostics.push(
                    Diagnostic::error(
                        "invalid-description",
                        format!("`{fact}` is declared both offered and absent"),
                        Label::new(
                            absent.span,
                            format!("declared absent by `{}`", absent.declared_by),
                        ),
                        "remove one declaration — a contradiction is not resolved by preferring \
                         one side, because only the author knows which half was meant",
                    )
                    .with_secondary(Label::new(
                        offered.span,
                        format!("declared offered by `{}`", offered.declared_by),
                    )),
                );
            }
        }

        let closure = self.closure();

        // ── F04 / F05: relevance decides whether an unknown fact matters ─────────────────────
        let mut missing = Vec::new();
        for fact in &closure {
            // An explicit absence is checked FIRST, before anything can satisfy the name: a
            // block that declares a capability absent does not stop being absent because it is
            // also declared.
            if self.presence(fact) == Presence::Absent {
                let declaration = &self.absent[fact];
                diagnostics.push(
                    Diagnostic::error(
                        "infeasible-configuration",
                        format!("`{fact}` is required by this system but declared absent"),
                        Label::new(self.request_span(fact), "required through this request"),
                        "either the requirement or the platform is wrong; an explicitly \
                         absent fact is a definite answer, not a gap to be filled in",
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
                    missing.push(fact.clone());
                    diagnostics.push(Diagnostic::error(
                        "missing-fact",
                        format!("`{fact}` is required by this system and nothing describes it"),
                        Label::new(self.request_span(fact), "required through this request"),
                        format!(
                            "declare `{fact}` as offered or absent on the platform — it is inside \
                             the dependency closure of what this system requests, so no decision \
                             that depends on it can be made"
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
            diagnostics,
        }
    }

    /// A span to point at for a fact: its own `needs` edge if there is one, else the request.
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

/// The names a clause mentions: a bare symbol argument, or the head of a nested form.
///
/// `(offers counter-width)` and `(offers (counter-width 32 bit))` both name `counter-width`, so
/// an author can write the bare fact or the fact with its parameters and mean the same thing
/// about presence.
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
