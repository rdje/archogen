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
//! | **guarantee** | the abstract offers a fact and the concrete does not, or offers it only as `false` |
//! | **constraint** | a value the concrete gives does not satisfy a bound the abstract states, or the concrete does not offer a value the abstract offers |
//! | **exclusion** | the abstract declares a fact absent and the concrete offers it |
//!
//! Anything the concrete adds that the abstract never mentions is an **addition**: reported,
//! never refused. That is the unused device.
//!
//! # What the abstract description writes, and what keeps it (leaf `M1.40`)
//!
//! ⛔ **Every offer is kept, not the last of each name, and every one is an obligation.** Until `M1.40` the
//! facets held one offer per fact, so an abstract platform writing `(f (at-least 1 s))` and `(f (at-most 5 s))`
//! was checked against the second alone, silently; and a value the abstract platform offered — `(tick-unit ns)`,
//! `(counter-width 32 bit)` — was read as nothing but the fact's presence, so `us` and `16 bit` kept them. A
//! system written against the abstract platform was then judged on values its refinement did not have. Both were
//! measured beside the substitutability relation (`docs/decisions/decision_substitutability-relation.md` §10,
//! reviews R12 and R13).
//!
//! | The abstract writes | It is kept when the concrete |
//! |---|---|
//! | `f` | offers `f` other than as `(f false)`: a bare boolean is `true`, and `false` says the fact does not hold |
//! | `(f (at-least v))`, `at-most`, `exactly` | gives `f` a quantity, and every quantity it gives `f` satisfies the bound |
//! | `(f 32 bit)` | offers `f` with the same amount, whatever the unit: `10000 kHz` keeps `10 MHz` |
//! | `(f ns)`, `(f true)`, `(f 4294967296)`, `(f device.timer (base …))` | offers `f` with the same value as written, a bare `f` reading as `true` |
//!
//! ⚠️ **"As written" is the conservative side, and it is a stated limit.** This check reads no vocabulary, so
//! `(pow2 32)` does not keep `4294967296`, a set's members in another order do not keep it, and a set with one
//! more member does not keep it either: each is refused, never wrongly accepted. A fact offered more than once,
//! as a `region` is per named region, is kept one offer at a time. One declaration writing two values of a fact
//! whose domain allows one is the substitutability relation's `invalid-description` (its §5), decided by the
//! vocabulary, not here.
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
                "a refinement's values satisfy every bound the abstract description states, and keep \
                 every value it offers"
            }
            Self::Exclusion => {
                "a refinement does not offer what the abstract description declares absent"
            }
        }
    }
}

/// How one offer writes its fact.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Reading {
    /// `f`, or `(f)`: the fact, with no value.
    Bare,
    /// `(f (at-least 32 bit))`: a bound in a direction, which an abstract description states.
    Bound(ComparisonDirection, Quantity),
    /// `(f 32 bit)`: a quantity.
    Quantity(Quantity),
    /// Any other value — `(f ns)`, `(f true)`, `(f 4294967296)`, `(f device.timer (base …))` — held as written,
    /// and compared with [`Form::structurally_eq`], which reads no span.
    Written(Vec<Form>),
}

impl Reading {
    /// Whether this is `(f false)`: the fact stated not to hold.
    fn is_false(&self) -> bool {
        match self {
            Self::Written(forms) => {
                matches!(forms.as_slice(), [only] if only.as_symbol() == Some("false"))
            }
            Self::Bare | Self::Bound(..) | Self::Quantity(_) => false,
        }
    }

    /// Whether this offer writes `written`, a bare offer reading as `true`.
    fn writes(&self, written: &[Form]) -> bool {
        match self {
            Self::Written(forms) => {
                forms.len() == written.len()
                    && forms.iter().zip(written).all(|(a, b)| a.structurally_eq(b))
            }
            Self::Bare => matches!(written, [only] if only.as_symbol() == Some("true")),
            Self::Bound(..) | Self::Quantity(_) => false,
        }
    }
}

/// One offered fact in a description.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Offer {
    /// Its name.
    name: String,
    /// What it writes of the fact.
    reading: Reading,
    /// Where it is written.
    span: Span,
}

/// A description reduced to what refinement cares about.
#[derive(Debug, Clone)]
pub struct Facets {
    /// The declaration's name, for messages.
    pub name: String,
    /// Every offer, in source order, by fact: a fact offered twice is two obligations, never the last one.
    offers: BTreeMap<String, Vec<Offer>>,
    absent: BTreeMap<String, Span>,
    /// The name this description claims to refine, if any.
    pub refines: Option<(String, Span)>,
    /// Whether every quantity this declaration writes could be read.
    ///
    /// ⛔ **An unreadable facet is neither refined nor refining, and [`check`] says nothing about it.**
    /// An obligation whose value could not be read is not *violated*, it is *unchecked*, and reporting
    /// "this refinement drops a guarantee" beside `quantity-non-positive-frequency` sends the author at
    /// the wrong half of their own description. The quantity diagnostic is the whole of the report, and
    /// the verdict it carries — `invalid-description`, because the input is ill-typed — outranks the
    /// `infeasible-configuration` a violated obligation would have produced, so the headline was already
    /// going to be the right one. Leaf `M1.28.2`.
    pub readable: bool,
}

impl Default for Facets {
    fn default() -> Self {
        Self {
            name: String::new(),
            offers: BTreeMap::new(),
            absent: BTreeMap::new(),
            refines: None,
            // A declaration that writes no quantity at all is readable; `false` here would make the
            // derived default refuse every refinement in the language.
            readable: true,
        }
    }
}

impl Facets {
    /// Reduce one declaration.
    ///
    /// Recognizes `(offers …)`, `(absent …)` and `(refines <name>)`. An offer is either a bare
    /// name, a name with a value (`(counter-width 32 bit)`), or a name with a bound
    /// (`(counter-width (at-least 32 bit))`).
    ///
    /// ⭐ **Returns the diagnostics it found beside the facets, and every caller has to destructure
    /// both.** This signature is the fix, not a detail of it: it used to return `Self` and swallow what
    /// [`Quantity::read`] refused, so `archogen check` accepted a description whose tick rate was zero and
    /// the only consumer left that surfaced the refusal was the S0 prototype `S0-RETIREMENT` exists to
    /// delete. A caller that wants only the facets has to write `.0`, and that `.` is where the next
    /// reader asks what the other half was.
    #[must_use]
    pub fn of(form: &Form) -> (Self, Vec<Diagnostic>) {
        let mut found = Vec::new();
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
                        let (offer, diagnostics) = read_offer(item);
                        if !diagnostics.is_empty() {
                            facets.readable = false;
                            found.extend(diagnostics);
                        }
                        if let Some(offer) = offer {
                            facets
                                .offers
                                .entry(offer.name.clone())
                                .or_default()
                                .push(offer);
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
        (facets, found)
    }
}

/// Read one offered fact, returning it with every diagnostic its quantities produced.
///
/// ⛔ **A form that is not shaped like a quantity is not a malformed one, and the shape is
/// `<number> <symbol>`.** Two measured cases this guard exists for, both from the shipped corpus:
/// `(offers (region device.timer (base 0x1000_0000)))` holds a name, a symbol and a nested clause, and
/// `(offers (counter-modulus 4294967296))` — in
/// `docs/semantics/boundary/accept/counter-width-and-rate.eadl`, verdict `accept` — holds a **count**.
/// `Quantity::read` refuses the first as `quantity-not-a-number` and the second as
/// `quantity-missing-unit`, so a reader that asked about every value-path offer turned an accepted
/// boundary case into `invalid-description`. That is measured, not anticipated: it is the one difference
/// a census over all 76 tracked descriptions found between this change and its parent.
///
/// ⭐ So the rule is: **a number immediately followed by a symbol denotes a quantity**, and the symbol
/// must be a known unit; a lone number is a count and is nobody's quantity. The bound path always asks,
/// because `(at-least …)` is a direction wrapper and a quantity is the only thing that can follow it.
/// `M1.26.1` carries this rule into the model layer's normative document; until then it is stated here and
/// in `docs/book/src/quantities.md`.
fn read_offer(item: &Form) -> (Option<Offer>, Vec<Diagnostic>) {
    match item {
        Form::Symbol { name, span } => (
            Some(Offer {
                name: name.clone(),
                reading: Reading::Bare,
                span: *span,
            }),
            Vec::new(),
        ),
        Form::List { items, span } => {
            let (Some(head), Some(rest)) = (items.first(), items.get(1..)) else {
                return (None, Vec::new());
            };
            let Some(name) = head.as_symbol() else {
                return (None, Vec::new());
            };
            let name = name.to_string();

            // `(counter-width (at-least 32 bit))` — a bound.
            if let Some(first) = rest.first() {
                if let Some(direction) = first.head().and_then(direction_of) {
                    let parts = first.items();
                    return match Quantity::read(parts.get(1), parts.get(2)) {
                        Ok(quantity) => (
                            Some(Offer {
                                name,
                                reading: Reading::Bound(direction, quantity),
                                span: *span,
                            }),
                            Vec::new(),
                        ),
                        Err(diagnostic) => (None, vec![*diagnostic]),
                    };
                }
            }

            // `(counter-width 32 bit)` — a value. `<number> <symbol>` and nothing else is a quantity
            // attempt; see this function's doc for the two corpus cases that decide the shape.
            let shaped_like_a_quantity = matches!(
                (rest.first(), rest.get(1)),
                (
                    Some(Form::Integer { .. } | Form::Decimal { .. }),
                    Some(Form::Symbol { .. })
                )
            );
            if !shaped_like_a_quantity {
                // `(f)` is the fact with no value; anything else is a value held as written (`M1.40`).
                let reading = if rest.is_empty() {
                    Reading::Bare
                } else {
                    Reading::Written(rest.to_vec())
                };
                return (
                    Some(Offer {
                        name,
                        reading,
                        span: *span,
                    }),
                    Vec::new(),
                );
            }
            match Quantity::read(rest.first(), rest.get(1)) {
                Ok(value) => (
                    Some(Offer {
                        name,
                        reading: Reading::Quantity(value),
                        span: *span,
                    }),
                    Vec::new(),
                ),
                Err(diagnostic) => (None, vec![*diagnostic]),
            }
        }
        _ => (None, Vec::new()),
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
///
/// ⛔ **Returns an empty report when either side could not be read.** See [`Facets::readable`]: an
/// obligation whose value is unreadable is unchecked, not violated, and the quantity diagnostic that
/// made it unreadable is already on its way to the author with a verdict that outranks this pass's.
#[must_use]
pub fn check(abstract_: &Facets, concrete: &Facets) -> RefinementReport {
    if !abstract_.readable || !concrete.readable {
        return RefinementReport {
            additions: Vec::new(),
            violations: Vec::new(),
        };
    }
    let mut violations = Vec::new();

    // ── Obligations 1 and 2: every offer of the abstract description is kept ──────────────────
    for (name, written) in &abstract_.offers {
        let Some(refined) = concrete.offers.get(name) else {
            let offer = &written[0];
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
        for offer in written {
            violations.extend(keep(abstract_, concrete, name, offer, refined));
        }
    }

    // ── Obligation 3: exclusions are respected ───────────────────────────────────────────────
    //
    // ⭐ The obligation a naive "the concrete may say more" rule silently drops. An abstract
    // description declares a fact absent *because something depends on its absence*; a
    // refinement that quietly adds it has changed what the abstract description meant.
    for (name, absent_span) in &abstract_.absent {
        // Every offer of an absent fact breaks the exclusion; the first is where the author looks.
        if let Some(offer) = concrete.offers.get(name).and_then(|offers| offers.first()) {
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

/// What one offer of the abstract description costs the concrete one, which offers the same fact as `refined`.
fn keep(
    abstract_: &Facets,
    concrete: &Facets,
    name: &str,
    offer: &Offer,
    refined: &[Offer],
) -> Vec<(Obligation, Diagnostic)> {
    let first = refined.first().map_or(offer.span, |r| r.span);
    match &offer.reading {
        // A bare guarantee is kept by any offer of the fact but `(f false)`.
        Reading::Bare => {
            if refined.iter().all(|r| r.reading.is_false()) {
                vec![(
                    Obligation::Guarantee,
                    Diagnostic::error(
                        "refinement-violated",
                        format!(
                            "`{}` offers `{name}` only as `false`, which `{}` guarantees",
                            concrete.name, abstract_.name
                        ),
                        Label::new(first, "`false` is not the guarantee"),
                        format!(
                            "violated obligation `{}`: {} — `false` says the fact does not hold, so it \
                             keeps no guarantee of it",
                            Obligation::Guarantee.slug(),
                            Obligation::Guarantee.statement()
                        ),
                    )
                    .with_secondary(Label::new(offer.span, "guaranteed here")),
                )]
            } else {
                Vec::new()
            }
        }
        Reading::Bound(direction, required) => bound(
            name, offer, *direction, *required, abstract_, concrete, refined,
        ),
        Reading::Quantity(value) => {
            let kept = refined.iter().any(|r| {
                matches!(&r.reading, Reading::Quantity(given) if given.equals(*value) == Ok(true))
            });
            if kept {
                Vec::new()
            } else {
                vec![not_kept(
                    abstract_,
                    concrete,
                    name,
                    offer,
                    first,
                    &value.to_string(),
                )]
            }
        }
        Reading::Written(forms) => {
            if refined.iter().any(|r| r.reading.writes(forms)) {
                Vec::new()
            } else {
                let text = forms
                    .iter()
                    .map(Form::to_canonical)
                    .collect::<Vec<_>>()
                    .join(" ");
                vec![not_kept(abstract_, concrete, name, offer, first, &text)]
            }
        }
    }
}

/// A value the abstract description offers, which the concrete one does not.
fn not_kept(
    abstract_: &Facets,
    concrete: &Facets,
    name: &str,
    offer: &Offer,
    first: Span,
    value: &str,
) -> (Obligation, Diagnostic) {
    (
        Obligation::Constraint,
        Diagnostic::error(
            "refinement-violated",
            format!(
                "`{}` offers `{name}` as `{value}`, and `{}` does not offer that value",
                abstract_.name, concrete.name
            ),
            Label::new(first, "a refinement keeps the value the abstract description offers"),
            format!(
                "offer `({name} {value})` — violated obligation `{}`: {} — a value is kept only by the same \
                 value: a quantity equal as a quantity, any other value as written",
                Obligation::Constraint.slug(),
                Obligation::Constraint.statement()
            ),
        )
        .with_secondary(Label::new(offer.span, "offered here")),
    )
}

/// A bound the abstract description states: met by every quantity the concrete one gives the fact, and it must
/// give one.
fn bound(
    name: &str,
    offer: &Offer,
    direction: ComparisonDirection,
    required: Quantity,
    abstract_: &Facets,
    concrete: &Facets,
    refined: &[Offer],
) -> Vec<(Obligation, Diagnostic)> {
    let values: Vec<(Quantity, Span)> = refined
        .iter()
        .filter_map(|r| match r.reading {
            Reading::Quantity(value) => Some((value, r.span)),
            _ => None,
        })
        .collect();
    if values.is_empty() {
        let first = refined.first().map_or(offer.span, |r| r.span);
        return vec![(
            Obligation::Constraint,
            Diagnostic::error(
                "refinement-violated",
                format!(
                    "`{name}` is bounded by `{}` but `{}` gives it no value",
                    abstract_.name, concrete.name
                ),
                Label::new(first, "no value to check the bound against"),
                format!(
                    "give `{name}` a concrete value, e.g. `({name} {required})` — \
                     violated obligation `{}`: {}",
                    Obligation::Constraint.slug(),
                    Obligation::Constraint.statement()
                ),
            )
            .with_secondary(Label::new(offer.span, "bound stated here")),
        )];
    }
    let mut violations = Vec::new();
    for (value, span) in values {
        match direction.satisfied_by(value, required) {
            Ok(true) => {}
            Ok(false) => violations.push((
                Obligation::Constraint,
                Diagnostic::error(
                    "refinement-violated",
                    format!(
                        "`{name}` is {value}, which does not satisfy `{} {required}`",
                        direction.slug()
                    ),
                    Label::new(span, "this value violates the bound"),
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
                    Label::new(span, "incompatible with the stated bound"),
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
    violations
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
        document
            .forms
            .iter()
            .map(|form| {
                let (facets, found) = Facets::of(form);
                assert!(
                    found.is_empty(),
                    "these fixtures are all readable: {}",
                    found.iter().map(|d| d.code).collect::<Vec<_>>().join(", ")
                );
                facets
            })
            .collect()
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
