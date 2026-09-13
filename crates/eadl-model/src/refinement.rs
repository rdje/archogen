//! Refinement: does a concrete description still satisfy the abstract one it claims to refine?
//!
//! `ROADMAP.md` §5.1.1:
//!
//! > Refinement permits a more concrete architectural description to satisfy an abstract
//! > functional description. The engine checks the relevant guarantees, operating conditions,
//! > capacities, topology constraints, and negative requirements. **A refinement declaration is
//! > an obligation to check, not permission to trust a claim blindly.**
//!
//! And §5.3, which supplies the rule that keeps the check from being useless:
//!
//! > Platform refinement must preserve the obligations actually relied upon by the profile,
//! > including relevant negative constraints; **adding an unused device is not automatically an
//! > invalid refinement**.
//!
//! Those two sentences pull in opposite directions on purpose. A check that demanded the
//! concrete description match the abstract one exactly would reject every real refinement — the
//! whole point is that the concrete one says *more*. A check that only looked at what both
//! mention would miss the case that matters most: an abstract description that declares
//! something **absent** precisely because a guarantee depends on its absence.
//!
//! So there are exactly three obligations, and a violation names which one broke:
//!
//! | Obligation | Violated when |
//! |---|---|
//! | **guarantee** | the abstract offers a fact and the concrete does not |
//! | **constraint** | the concrete's value does not satisfy the abstract's stated bound |
//! | **exclusion** | the abstract declares a fact absent and the concrete offers it |
//!
//! Anything the concrete adds that the abstract never mentions is an **addition**: reported,
//! never refused. That is the unused device.
//!
//! # Why a bound needs a direction
//!
//! An abstract description states `(counter-width (at-least 32 bit))`, not `(counter-width 32
//! bit)`. §5.2 warns that "more bits or a faster clock is not universally better", so a bare
//! value in an abstract description would leave the checker guessing whether 64 satisfies 32 —
//! and guessing right for width while guessing wrong for a delivery bound. The direction is
//! written down because there is no correct default.

use std::collections::BTreeMap;

use eadl_front::{Diagnostic, Form, Label, Span};

use crate::quantity::{ComparisonDirection, Quantity, QuantityError};

/// Which obligation a refinement broke.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Obligation {
    /// A fact the abstract description offers.
    Guarantee,
    /// A bound the abstract description states on a fact's value.
    Constraint,
    /// A fact the abstract description declares absent.
    Exclusion,
}

impl Obligation {
    /// The stable machine-readable name.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Guarantee => "guarantee",
            Self::Constraint => "constraint",
            Self::Exclusion => "exclusion",
        }
    }

    /// What the obligation says, for a diagnostic.
    #[must_use]
    pub const fn statement(self) -> &'static str {
        match self {
            Self::Guarantee => "a refinement keeps every guarantee the abstract description offers",
            Self::Constraint => {
                "a refinement's values satisfy every bound the abstract description states"
            }
            Self::Exclusion => {
                "a refinement does not offer what the abstract description declares absent"
            }
        }
    }
}

/// One offered fact in a description.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Offer {
    /// Its name.
    name: String,
    /// The bound, when the abstract description states one.
    bound: Option<(ComparisonDirection, Quantity)>,
    /// The value, when the concrete description gives one.
    value: Option<Quantity>,
    /// Where it is written.
    span: Span,
}

/// A description reduced to what refinement cares about.
#[derive(Debug, Clone, Default)]
pub struct Facets {
    /// The declaration's name, for messages.
    pub name: String,
    offers: BTreeMap<String, Offer>,
    absent: BTreeMap<String, Span>,
    /// The name this description claims to refine, if any.
    pub refines: Option<(String, Span)>,
}

impl Facets {
    /// Reduce one declaration.
    ///
    /// Recognizes `(offers …)`, `(absent …)` and `(refines <name>)`. An offer is either a bare
    /// name, a name with a value (`(counter-width 32 bit)`), or a name with a bound
    /// (`(counter-width (at-least 32 bit))`).
    #[must_use]
    pub fn of(form: &Form) -> Self {
        let mut facets = Self {
            name: form
                .items()
                .get(1)
                .and_then(Form::as_symbol)
                .unwrap_or("<unnamed>")
                .to_string(),
            ..Self::default()
        };

        for clause in form.items().iter().skip(1) {
            match clause.head() {
                Some("refines") => {
                    if let Some(target) = clause.items().get(1) {
                        if let Some(name) = target.as_symbol() {
                            facets.refines = Some((name.to_string(), target.span()));
                        }
                    }
                }
                Some("offers") => {
                    for item in clause.items().iter().skip(1) {
                        if let Some(offer) = read_offer(item) {
                            facets.offers.insert(offer.name.clone(), offer);
                        }
                    }
                }
                Some("absent") => {
                    for item in clause.items().iter().skip(1) {
                        let name = match item {
                            Form::Symbol { name, .. } => Some(name.clone()),
                            Form::List { .. } => item.head().map(ToString::to_string),
                            _ => None,
                        };
                        if let Some(name) = name {
                            facets.absent.insert(name, item.span());
                        }
                    }
                }
                _ => {}
            }
        }
        facets
    }
}

fn read_offer(item: &Form) -> Option<Offer> {
    match item {
        Form::Symbol { name, span } => Some(Offer {
            name: name.clone(),
            bound: None,
            value: None,
            span: *span,
        }),
        Form::List { items, span } => {
            let name = items.first()?.as_symbol()?.to_string();
            let rest = &items[1..];

            // `(counter-width (at-least 32 bit))` — a bound.
            if let Some(first) = rest.first() {
                if let Some(direction) = first.head().and_then(direction_of) {
                    let parts = first.items();
                    let quantity = Quantity::read(parts.get(1), parts.get(2)).ok()?;
                    return Some(Offer {
                        name,
                        bound: Some((direction, quantity)),
                        value: None,
                        span: *span,
                    });
                }
            }

            // `(counter-width 32 bit)` — a value.
            let value = Quantity::read(rest.first(), rest.get(1)).ok();
            Some(Offer {
                name,
                bound: None,
                value,
                span: *span,
            })
        }
        _ => None,
    }
}

fn direction_of(head: &str) -> Option<ComparisonDirection> {
    Some(match head {
        "at-least" => ComparisonDirection::AtLeast,
        "at-most" => ComparisonDirection::AtMost,
        "exactly" => ComparisonDirection::Exact,
        _ => return None,
    })
}

/// What a refinement check concluded.
#[derive(Debug, Clone)]
pub struct RefinementReport {
    /// Facts the concrete description adds that the abstract never mentions.
    ///
    /// §5.3: "adding an unused device is not automatically an invalid refinement". Reported,
    /// never refused — the author should be able to see what grew.
    pub additions: Vec<String>,
    /// Every violated obligation, with its diagnostic.
    pub violations: Vec<(Obligation, Diagnostic)>,
}

impl RefinementReport {
    /// Whether the refinement holds.
    #[must_use]
    pub fn is_valid(&self) -> bool {
        self.violations.is_empty()
    }

    /// The diagnostics alone.
    #[must_use]
    pub fn diagnostics(&self) -> Vec<Diagnostic> {
        self.violations
            .iter()
            .map(|(_, diagnostic)| diagnostic.clone())
            .collect()
    }
}

/// Check that `concrete` refines `abstract_`.
#[must_use]
pub fn check(abstract_: &Facets, concrete: &Facets) -> RefinementReport {
    let mut violations = Vec::new();

    // ── Obligation 1: guarantees are kept ────────────────────────────────────────────────────
    for (name, offer) in &abstract_.offers {
        let Some(refined) = concrete.offers.get(name) else {
            violations.push((
                Obligation::Guarantee,
                Diagnostic::error(
                    "refinement-violated",
                    format!(
                        "`{}` does not offer `{name}`, which `{}` guarantees",
                        concrete.name, abstract_.name
                    ),
                    Label::new(
                        concrete
                            .refines
                            .as_ref()
                            .map_or(offer.span, |(_, span)| *span),
                        "this refinement drops a guarantee",
                    ),
                    format!(
                        "violated obligation `{}`: {}",
                        Obligation::Guarantee.slug(),
                        Obligation::Guarantee.statement()
                    ),
                )
                .with_secondary(Label::new(offer.span, "guaranteed here")),
            ));
            continue;
        };

        // ── Obligation 2: stated bounds are satisfied ────────────────────────────────────────
        if let Some((direction, required)) = &offer.bound {
            let Some(value) = refined.value else {
                violations.push((
                    Obligation::Constraint,
                    Diagnostic::error(
                        "refinement-violated",
                        format!(
                            "`{name}` is bounded by `{}` but `{}` gives it no value",
                            abstract_.name, concrete.name
                        ),
                        Label::new(refined.span, "no value to check the bound against"),
                        format!(
                            "give `{name}` a concrete value, e.g. `({name} {required})` — \
                             violated obligation `{}`: {}",
                            Obligation::Constraint.slug(),
                            Obligation::Constraint.statement()
                        ),
                    )
                    .with_secondary(Label::new(offer.span, "bound stated here")),
                ));
                continue;
            };

            match direction.satisfied_by(value, *required) {
                Ok(true) => {}
                Ok(false) => violations.push((
                    Obligation::Constraint,
                    Diagnostic::error(
                        "refinement-violated",
                        format!(
                            "`{name}` is {value}, which does not satisfy `{} {required}`",
                            direction.slug()
                        ),
                        Label::new(refined.span, "this value violates the bound"),
                        format!(
                            "violated obligation `{}`: {} — the direction is `{}`, so a value \
                             that is merely different is not a substitute",
                            Obligation::Constraint.slug(),
                            Obligation::Constraint.statement(),
                            direction.slug()
                        ),
                    )
                    .with_secondary(Label::new(offer.span, "bound stated here")),
                )),
                Err(error) => violations.push((
                    Obligation::Constraint,
                    Diagnostic::error(
                        "refinement-violated",
                        format!("`{name}` cannot be checked against its bound: {error}"),
                        Label::new(refined.span, "incompatible with the stated bound"),
                        match error {
                            QuantityError::IncompatibleDimensions { .. } => format!(
                                "the refinement measures something else entirely — violated \
                                 obligation `{}`",
                                Obligation::Constraint.slug()
                            ),
                            other => other.to_string(),
                        },
                    )
                    .with_secondary(Label::new(offer.span, "bound stated here")),
                )),
            }
        }
    }

    // ── Obligation 3: exclusions are respected ───────────────────────────────────────────────
    //
    // ⭐ The obligation a naive "the concrete may say more" rule silently drops. An abstract
    // description declares a fact absent *because something depends on its absence*; a
    // refinement that quietly adds it has changed what the abstract description meant.
    for (name, absent_span) in &abstract_.absent {
        if let Some(offer) = concrete.offers.get(name) {
            violations.push((
                Obligation::Exclusion,
                Diagnostic::error(
                    "refinement-violated",
                    format!(
                        "`{}` offers `{name}`, which `{}` declares absent",
                        concrete.name, abstract_.name
                    ),
                    Label::new(offer.span, "added by this refinement"),
                    format!(
                        "violated obligation `{}`: {} — an explicit absence is a constraint \
                         something relies on, not an omission to be filled in",
                        Obligation::Exclusion.slug(),
                        Obligation::Exclusion.statement()
                    ),
                )
                .with_secondary(Label::new(*absent_span, "declared absent here")),
            ));
        }
    }

    // Everything the concrete adds that the abstract never mentions: the unused device.
    let additions: Vec<String> = concrete
        .offers
        .keys()
        .filter(|name| {
            !abstract_.offers.contains_key(*name) && !abstract_.absent.contains_key(*name)
        })
        .cloned()
        .collect();

    RefinementReport {
        additions,
        violations,
    }
}

#[cfg(test)]
mod tests {
    use super::{check, Facets};
    use eadl_front::{read, SourceMap};

    fn facets(text: &str) -> Vec<Facets> {
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", text).expect("small");
        let (document, diagnostics) = read(&sources, id);
        assert!(
            !diagnostics.has_errors(),
            "{}",
            diagnostics.render(&sources)
        );
        document.forms.iter().map(Facets::of).collect()
    }

    #[test]
    fn a_refinement_that_keeps_everything_is_valid() {
        let parts = facets(
            "(defplatform a (offers (counter-width (at-least 32 bit))))\n\
             (defplatform b (refines a) (offers (counter-width 64 bit)))",
        );
        let report = check(&parts[0], &parts[1]);
        assert!(report.is_valid(), "{:?}", report.diagnostics());
    }

    #[test]
    fn the_refines_target_is_recorded() {
        let parts = facets("(defplatform b (refines a) (offers x))");
        assert_eq!(
            parts[0].refines.as_ref().map(|(n, _)| n.as_str()),
            Some("a")
        );
    }
}
