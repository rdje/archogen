//! A provider's offers and absences, read into one statement per fact
//! (`docs/decisions/decision_substitutability-relation.md` §1, §2, §4, §5, §8; leaf `M3.1.2.3`).
//!
//! ⭐ **The relation's own reader.** `refinement.rs`'s `Facets::of` reads an abstract platform's bounds for the
//! refinement check and is not the relation's to change (record §1.1, `SR-H6`); this module reads what the relation
//! judges. A block or a platform is a provider. A service is not, but its offers and absences are read, and refused,
//! as a provider's are, since a declaration that contradicts itself does so whoever writes it (record §1, R24 4).
//!
//! What it reads, item by item (record §1, §2):
//! - an item of `offers` names a fact, bare or as a list's head; one naming nothing, or holding a clause, is refused;
//! - a name the vocabulary does not declare has no domain to contradict and is left to presence (record §5, R1 A6);
//! - an offer is bare (`f`, `(f)`), a value (`(f v)`, `(f (exactly v))`), or an abstract platform's bound in the
//!   fact's own direction, `(f (at-least v))` — not a value, read by refinement (§2);
//! - an item of `absent` is a fact's name alone.
//!
//! Then per fact (record §4, §5): offered and absent at once, two values, a bound beside a value, a derived fact beside
//! an input of its rule, a modulus above its width or beside a saturating wrap — each refused. One value written in
//! several spellings is kept in every spelling, in the order written, since a comparison is decided by any spelling
//! whose arithmetic does not overflow (§5, R25 1).

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{Form, Span};

use crate::refusal::{Cause, Refusal};
use crate::value::{self, NotOfDomain, NotOfDomainKind, Value};
use crate::vocabulary::{Direction, Domain, Fact, Role, Vocabulary};

/// The counter's facts §4's cross-field rules name.
const COUNTER_WIDTH: &str = "counter-width";
const WRAP_BEHAVIOR: &str = "wrap-behavior";
const SATURATING: &str = "saturating";

/// The widest counter whose `2^width` the modulus check compares: `(pow2 126)`, the largest writable modulus, is held
/// by every width of 127 bits or more (record §4, R7 G8).
const WIDTH_THAT_HOLDS_EVERY_MODULUS: i128 = 127;

/// What one provider states of one fact.
#[derive(Debug, Clone, PartialEq)]
pub enum State {
    /// One value, in every spelling written, in written order, never empty (§5).
    Valued(Vec<Value>),
    /// Offered with no value: bare in a domain where bare has none, or only an abstract platform's bound (§2).
    Unvalued,
    /// Declared absent.
    Absent,
    /// Offered so that comparing its spellings, or ordering an interval's endpoints, overflows: `unsupported-profile`
    /// at this provider (§2, §5).
    Overflow,
}

/// A fact's state at one provider, with every item that wrote it.
#[derive(Debug, Clone, PartialEq)]
pub struct Statement {
    /// The state.
    pub state: State,
    /// Where each offer or absence of the fact is written, in written order.
    pub spans: Vec<Span>,
}

/// A provider read.
#[derive(Debug, Clone, PartialEq)]
pub struct Provider {
    /// Its declaration's name.
    pub name: String,
    /// Where its declaration is written.
    pub span: Span,
    /// What it states of each vocabulary fact it names; a fact it does not name is not here.
    pub facts: BTreeMap<String, Statement>,
}

impl Provider {
    /// What it states of `fact`, if it names it.
    #[must_use]
    pub fn state(&self, fact: &str) -> Option<&State> {
        self.facts.get(fact).map(|s| &s.state)
    }
}

/// One offer, as written.
#[derive(Debug, Clone)]
enum Written {
    /// `f` or `(f)`.
    Bare,
    /// `(f v)` or `(f (exactly v))`.
    Value(Value),
    /// An abstract platform's `(f (at-least v))` in the fact's direction.
    Bound,
    /// A value whose interval's endpoints cannot be ordered.
    Overflow,
}

/// A declared fact and every offer of it, with where each is written, in written order.
type Offers<'v> = (&'v Fact, Vec<(Written, Span)>);

fn refusal(cause: Cause, fact: &str, span: Span, detail: impl Into<String>) -> Refusal {
    Refusal::new(cause, Some(fact), span, detail)
}

fn not_of_domain(fact: &str, span: Span, wrong: NotOfDomain) -> Refusal {
    let cause = match wrong.kind {
        NotOfDomainKind::Outside => Cause::OutsideDomain,
        NotOfDomainKind::Empty => Cause::EmptyValue,
        NotOfDomainKind::Reversed => Cause::ReversedInterval,
        NotOfDomainKind::NotWholeBits => Cause::NotWholeBits,
        NotOfDomainKind::ZeroModulus => Cause::ZeroModulus,
    };
    refusal(cause, fact, span, wrong.detail)
}

/// The name an item writes — itself, or a list's head — or `None` when it names nothing.
fn named(item: &Form) -> Option<&str> {
    match item {
        Form::Symbol { name, .. } => Some(name),
        _ => item.head(),
    }
}

/// Whether `form` is, or holds at any depth, a list headed by a clause word (record §1, R27 2, R28 3).
fn holds_clause(form: &Form, vocabulary: &Vocabulary) -> bool {
    matches!(form, Form::List { .. })
        && (form.head().is_some_and(|h| vocabulary.is_clause_word(h))
            || form
                .items()
                .iter()
                .any(|item| holds_clause(item, vocabulary)))
}

/// A value read, or [`Written::Overflow`] when its interval's endpoints cannot be ordered (§2, R18 7).
fn valued(fact: &Fact, forms: &[Form], item: &Form) -> Result<Written, Refusal> {
    let v = value::read(fact, forms, item.span())
        .map_err(|w| not_of_domain(&fact.name, item.span(), w))?;
    Ok(match value::orderable(&v) {
        Ok(()) => Written::Value(v),
        Err(value::Overflow) => Written::Overflow,
    })
}

/// One offer of a declared guarantee fact, classified (record §1.1, §2).
fn classify(fact: &Fact, item: &Form) -> Result<Written, Refusal> {
    let rest = match item {
        Form::List { items, .. } => &items[1..],
        _ => return Ok(Written::Bare),
    };
    if rest.is_empty() {
        // `(f)` with nothing after it is a bare offer (§2's set row, R12 L12).
        return Ok(Written::Bare);
    }
    if let [wrapper @ Form::List { .. }] = rest {
        // `()` has no head and holds nothing, so there is nothing after one to slice.
        let inner = wrapper.items().get(1..).unwrap_or_default();
        match wrapper.head() {
            Some("exactly") if inner.is_empty() => {
                return Err(refusal(
                    Cause::EmptyValue,
                    &fact.name,
                    item.span(),
                    "`(f (exactly))` writes no value",
                ));
            }
            // In an offer `(f (exactly v))` is, for the relation, the value `v` (§1.1, R10 J8).
            Some("exactly") => return valued(fact, inner, item),
            Some(word @ ("exact" | "includes" | "within")) => {
                return Err(refusal(
                    Cause::DirectionNameAsWrapper,
                    &fact.name,
                    item.span(),
                    format!("`{word}` names a direction, and a bound is written `at-least`, `at-most` or `exactly`"),
                ));
            }
            Some(word @ ("at-least" | "at-most")) => {
                if matches!(fact.domain, Domain::Boolean | Domain::Group(_)) {
                    return Err(refusal(
                        Cause::BoundOnBoolean,
                        &fact.name,
                        item.span(),
                        "a boolean or a group's head takes no bound but `exactly`",
                    ));
                }
                let written = if word == "at-least" {
                    Direction::AtLeast
                } else {
                    Direction::AtMost
                };
                if written != fact.direction {
                    return Err(refusal(
                        Cause::DirectionAgainstFact,
                        &fact.name,
                        item.span(),
                        format!(
                            "`{word}` written against the fact's direction, `{}`",
                            fact.direction.word()
                        ),
                    ));
                }
                // An abstract platform's bound: refinement's, not a value; its value still of the domain (§2).
                value::read(fact, inner, item.span())
                    .map_err(|w| not_of_domain(&fact.name, item.span(), w))?;
                return Ok(Written::Bound);
            }
            _ => {}
        }
    }
    valued(fact, rest, item)
}

/// One item of `offers`: refused, a declared fact's offer, or `None` for a name the vocabulary does not declare.
fn offered<'v>(
    item: &Form,
    vocabulary: &'v Vocabulary,
) -> Result<Option<(&'v Fact, Written)>, Refusal> {
    let Some(name) = named(item) else {
        return Err(Refusal::new(
            Cause::ItemNamesNothing,
            None,
            item.span(),
            "an item of `offers` names a fact: a name, or a list headed by one",
        ));
    };
    if holds_clause(item, vocabulary) {
        return Err(refusal(
            Cause::ClauseInOffer,
            name,
            item.span(),
            "a clause written inside an item of `offers` is read by nothing",
        ));
    }
    let Some(fact) = vocabulary.fact(name) else {
        return Ok(None);
    };
    if fact.role == Role::Statement {
        return Err(refusal(
            Cause::StatementOffered,
            name,
            item.span(),
            "a statement fact is the requiring side's word about itself, and no provider offers it",
        ));
    }
    classify(fact, item).map(|w| Some((fact, w)))
}

/// One item of `absent`: refused, a declared fact, or `None` for a name the vocabulary does not declare.
fn declared_absent<'v>(
    item: &Form,
    vocabulary: &'v Vocabulary,
) -> Result<Option<&'v Fact>, Refusal> {
    let Some(name) = named(item) else {
        return Err(Refusal::new(
            Cause::ItemNamesNothing,
            None,
            item.span(),
            "an item of `absent` names a fact",
        ));
    };
    if matches!(item, Form::List { .. }) {
        return Err(refusal(
            Cause::ListInAbsent,
            name,
            item.span(),
            "`absent` names a fact by its name alone; a list would drop what follows its head",
        ));
    }
    let Some(fact) = vocabulary.fact(name) else {
        return Ok(None);
    };
    if fact.role == Role::Statement {
        return Err(refusal(
            Cause::StatementAbsent,
            name,
            item.span(),
            "a statement fact is the requiring side's word about itself, and no provider declares it absent",
        ));
    }
    Ok(Some(fact))
}

/// What one fact's offers make of it at the provider (record §5), or the refusal they give.
fn settle(fact: &Fact, writes: &[(Written, Span)]) -> Result<State, Refusal> {
    let boolean = matches!(fact.domain, Domain::Boolean | Domain::Group(_));
    let (mut spellings, mut bound, mut overflow) = (Vec::new(), None, false);
    for (written, span) in writes {
        match written {
            // For a boolean or a group's head, bare presence is the value `true` (§1).
            Written::Bare if boolean => spellings.push((Value::Bool(true), *span)),
            // Elsewhere a bare offer beside a value is that value, and alone it is presence without one (§5).
            Written::Bare => {}
            Written::Bound => bound = bound.or(Some(*span)),
            Written::Overflow => overflow = true,
            Written::Value(v) => spellings.push((v.clone(), *span)),
        }
    }
    // Every pair compared, so the order written decides nothing: two that compare unequal are two values whatever the
    // others; a pair whose comparison overflows leaves the fact past the arithmetic (§5, R17 5, R18 1).
    for (i, (a, _)) in spellings.iter().enumerate() {
        for (b, at) in &spellings[i + 1..] {
            match value::same(a, b) {
                Ok(true) => {}
                Ok(false) => {
                    return Err(refusal(
                        Cause::TwoValues,
                        &fact.name,
                        *at,
                        "one provider offers one fact with two values",
                    ));
                }
                Err(value::Overflow) => overflow = true,
            }
        }
    }
    if let (Some(at), false) = (bound, spellings.is_empty()) {
        return Err(refusal(
            Cause::BoundBesideValue,
            &fact.name,
            at,
            "a bound beside a value of the same fact",
        ));
    }
    Ok(if overflow {
        State::Overflow
    } else if spellings.is_empty() {
        State::Unvalued
    } else {
        State::Valued(spellings.into_iter().map(|(v, _)| v).collect())
    })
}

fn offers_it(state: Option<&State>) -> bool {
    matches!(
        state,
        Some(State::Valued(_) | State::Unvalued | State::Overflow)
    )
}

/// Read a block or platform declaration as a provider.
///
/// # Errors
///
/// Every [`Refusal`] its offers and absences give, never only the first (record §8).
pub fn read(decl: &Form, vocabulary: &Vocabulary) -> Result<Provider, Vec<Refusal>> {
    let name = decl
        .items()
        .get(1)
        .and_then(Form::as_symbol)
        .unwrap_or_default();
    let mut refusals = Vec::new();
    if vocabulary.fact(name).is_some() {
        refusals.push(refusal(
            Cause::DeclarationNamedLikeFact,
            name,
            decl.span(),
            "a declaration whose local name is a vocabulary fact would capture every requirement on that fact",
        ));
    }
    let mut writes: BTreeMap<&str, Offers> = BTreeMap::new();
    let mut absent: BTreeMap<&str, Span> = BTreeMap::new();
    for clause in decl.items().iter().skip(2) {
        match clause.head() {
            Some("offers") => {
                for item in &clause.items()[1..] {
                    match offered(item, vocabulary) {
                        Ok(Some((fact, w))) => writes
                            .entry(fact.name.as_str())
                            .or_insert_with(|| (fact, Vec::new()))
                            .1
                            .push((w, item.span())),
                        Ok(None) => {}
                        Err(r) => refusals.push(r),
                    }
                }
            }
            Some("absent") => {
                for item in &clause.items()[1..] {
                    match declared_absent(item, vocabulary) {
                        Ok(Some(fact)) => {
                            absent.entry(fact.name.as_str()).or_insert(item.span());
                        }
                        Ok(None) => {}
                        Err(r) => refusals.push(r),
                    }
                }
            }
            _ => {}
        }
    }
    let mut facts: BTreeMap<String, Statement> = BTreeMap::new();
    for (name, (fact, ws)) in &writes {
        if let Some(at) = absent.get(name) {
            refusals.push(refusal(
                Cause::OfferedAndAbsent,
                name,
                *at,
                "one provider offers this fact and declares it absent",
            ));
            continue;
        }
        match settle(fact, ws) {
            Ok(state) => {
                facts.insert(
                    (*name).to_string(),
                    Statement {
                        state,
                        spans: ws.iter().map(|(_, s)| *s).collect(),
                    },
                );
            }
            Err(r) => refusals.push(r),
        }
    }
    for (name, at) in &absent {
        if !writes.contains_key(name) {
            facts.insert(
                (*name).to_string(),
                Statement {
                    state: State::Absent,
                    spans: vec![*at],
                },
            );
        }
    }
    refusals.extend(derived_beside_inputs(&facts, vocabulary, decl.span()));
    refusals.extend(counter(&facts, decl.span()));
    if refusals.is_empty() {
        Ok(Provider {
            name: name.to_string(),
            span: decl.span(),
            facts,
        })
    } else {
        Err(refusals)
    }
}

/// A derived fact offered, or declared absent, where the provider offers a fact its rule reads: whoever offers the
/// inputs has offered the result's grounds, and the engine computes it (record §4, R16 8).
fn derived_beside_inputs(
    facts: &BTreeMap<String, Statement>,
    vocabulary: &Vocabulary,
    whole: Span,
) -> Vec<Refusal> {
    let mut out = Vec::new();
    for fact in vocabulary.facts().filter(|f| f.rule.is_some()) {
        let Some(stated) = facts.get(&fact.name) else {
            continue;
        };
        let inputs: BTreeSet<&String> = fact.derived_from.iter().chain(&fact.reads).collect();
        if !inputs
            .iter()
            .any(|i| offers_it(facts.get(*i).map(|s| &s.state)))
        {
            continue;
        }
        let at = stated.spans.first().copied().unwrap_or(whole);
        out.push(match stated.state {
            State::Absent => refusal(
                Cause::DerivedAbsentBesideInput,
                &fact.name,
                at,
                "a derived fact declared absent where its rule's inputs are offered: the engine derives it",
            ),
            _ => refusal(
                Cause::DerivedBesideInput,
                &fact.name,
                at,
                "a derived fact offered beside a fact its rule reads: the engine derives it from those",
            ),
        });
    }
    out
}

/// §4's cross-field rules of the counter: a modulus above `2^width`, and a modulus beside a saturating wrap. They only
/// refuse; no requirement is met through them (record §4, R28 5).
fn counter(facts: &BTreeMap<String, Statement>, whole: Span) -> Vec<Refusal> {
    let state = |name: &str| facts.get(name).map(|s| &s.state);
    let at = |name: &str| {
        facts
            .get(name)
            .and_then(|s| s.spans.first().copied())
            .unwrap_or(whole)
    };
    let mut out = Vec::new();
    // The first spelling of each: spellings in two units were compared in the base unit as the provider was read, so
    // each converts to the one value (§5, R25 1).
    if let (Some(State::Valued(widths)), Some(State::Valued(moduli))) =
        (state(COUNTER_WIDTH), state(value::COUNTER_MODULUS))
    {
        if let (Some(Value::Quantity(width)), Some(Value::Count(modulus))) =
            (widths.first(), moduli.first())
        {
            let bits = width
                .in_base()
                .ok()
                .filter(|b| b.is_integer())
                .map(|b| b.numerator());
            if let Some(bits) = bits.filter(|b| (0..WIDTH_THAT_HOLDS_EVERY_MODULUS).contains(b)) {
                if *modulus > 1i128 << bits {
                    out.push(refusal(
                        Cause::ModulusAboveWidth,
                        value::COUNTER_MODULUS,
                        at(value::COUNTER_MODULUS),
                        format!(
                            "a modulus above `2^{bits}`, which a {bits}-bit register cannot hold"
                        ),
                    ));
                }
            }
        }
    }
    let saturating = matches!(state(WRAP_BEHAVIOR), Some(State::Valued(w))
        if matches!(w.first(), Some(Value::Enum(e)) if e == SATURATING));
    if saturating && offers_it(state(value::COUNTER_MODULUS)) {
        out.push(refusal(
            Cause::ModulusBesideSaturating,
            value::COUNTER_MODULUS,
            at(value::COUNTER_MODULUS),
            "a modulus beside `(wrap-behavior saturating)`: a modulus is where a modular counter wraps",
        ));
    }
    out
}

/// Read a service's offers and absences as a provider's are, refusing what §8 refuses in one. A service is no
/// provider: its offers are judged against no requirement in `/1` (record §1, R3 C13, R24 4).
///
/// # Errors
///
/// As [`read`].
pub fn read_service(decl: &Form, vocabulary: &Vocabulary) -> Result<(), Vec<Refusal>> {
    read(decl, vocabulary).map(|_| ())
}
