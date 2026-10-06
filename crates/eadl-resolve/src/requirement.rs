//! What a side requires: constraints inside `requires`, facts named in `needs`, and what `uses` may not name
//! (`docs/decisions/decision_substitutability-relation.md` §1, §3 rules 5 and 6, §8; leaf `M3.1.2.4`).
//!
//! ⭐ **A side is read whole, at the positions presence reads.** A requirement's side is the declaration that writes
//! it, and it holds every `requires` clause, `needs` and `uses` written at any depth of any clause headed by a name
//! but `offers`, `absent` and `refines` — so no fact enters the closure that the relation does not read, and no
//! constraint stands where it is not read (record §1). The positions hold a grammar: a system's `platform` clause
//! holds `needs`, `uses` and `requires` alone; `requires` holds constraints, `needs`, `uses` and `requires`, any other
//! clause inside it refused; `needs` and `uses` hold names. A `requires` inside another is a clause of its own.
//!
//! Then the side's constraints are checked together, since a contradiction split across two clauses is one: on one
//! fact, two equalities that differ, a statement with two values, or an equality another constraint refuses is
//! `invalid-description`; what the exact arithmetic cannot decide is `unsupported-profile` (record §3 rule 5).

use std::collections::BTreeSet;

use eadl_front::module::Program;
use eadl_front::{Form, Span};

use crate::refusal::{Cause, Code, Refusal};
use crate::value::{self, NotOfDomain, NotOfDomainKind, Value};
use crate::vocabulary::{Direction, Domain, Fact, Role, Vocabulary};

/// A requirement on one fact, as the relation judges it (record §1's definitions).
#[derive(Debug, Clone, PartialEq)]
pub enum Requirement {
    /// `(needs f)`: presence, which a boolean or a group's head reads as `(f true)` (§3 rule 3).
    Presence {
        /// The fact.
        fact: String,
        /// Where it is named.
        span: Span,
    },
    /// A value or a bound in a direction: the fact's own, or `exactly` (§1.1).
    Constraint {
        /// The fact.
        fact: String,
        /// The direction written, or the fact's for a bare value.
        direction: Direction,
        /// The required value.
        value: Value,
        /// Where it is written.
        span: Span,
    },
    /// A group: the head holds, and each sub-constraint (§2's group row).
    Group {
        /// The head.
        fact: String,
        /// The sub-constraints, each on its own sub-fact.
        parts: Vec<Requirement>,
        /// Where it is written.
        span: Span,
    },
    /// A statement fact's word: checked for its domain, constraining no provider (§3 rule 6).
    Statement {
        /// The fact.
        fact: String,
        /// The value stated.
        value: Value,
        /// Where it is written.
        span: Span,
    },
}

impl Requirement {
    /// The fact it is on.
    #[must_use]
    pub fn fact(&self) -> &str {
        match self {
            Self::Presence { fact, .. }
            | Self::Constraint { fact, .. }
            | Self::Group { fact, .. }
            | Self::Statement { fact, .. } => fact,
        }
    }

    /// Where it is written.
    #[must_use]
    pub fn span(&self) -> Span {
        match self {
            Self::Presence { span, .. }
            | Self::Constraint { span, .. }
            | Self::Group { span, .. }
            | Self::Statement { span, .. } => *span,
        }
    }
}

/// Every refusal of what a side writes; the class of the whole is [`code`]'s.
pub type Refusals = Vec<Refusal>;

/// The code a set of refusals is reported under: `invalid-description` when any gives it, else `unsupported-profile`
/// — whatever order the items are written in (record §3 rule 5, R21 2).
#[must_use]
pub fn code(refusals: &[Refusal]) -> Code {
    if refusals
        .iter()
        .any(|r| r.code() == Code::InvalidDescription)
    {
        Code::InvalidDescription
    } else {
        Code::UnsupportedProfile
    }
}

fn refusal(cause: Cause, fact: Option<&str>, span: Span, detail: impl Into<String>) -> Refusal {
    Refusal::new(cause, fact, span, detail)
}

fn not_of_domain(fact: &str, span: Span, wrong: NotOfDomain) -> Refusal {
    let cause = match wrong.kind {
        NotOfDomainKind::Outside => Cause::OutsideDomain,
        NotOfDomainKind::Empty => Cause::EmptyValue,
        NotOfDomainKind::Reversed => Cause::ReversedInterval,
        NotOfDomainKind::NotWholeBits => Cause::NotWholeBits,
        NotOfDomainKind::ZeroModulus => Cause::ZeroModulus,
    };
    refusal(cause, Some(fact), span, wrong.detail)
}

/// The name an item writes — itself, or a list's head — or `None` when it names nothing.
fn named(item: &Form) -> Option<&str> {
    match item {
        Form::Symbol { name, .. } => Some(name),
        _ => item.head(),
    }
}

/// Whether `form` is, or holds at any depth, a list headed by a clause word but those `requires` reads where they
/// stand: `needs`, `uses` and `requires` (record §1, R29 1, R30 1).
fn holds_other_clause(form: &Form, vocabulary: &Vocabulary) -> bool {
    matches!(form, Form::List { .. })
        && (form.head().is_some_and(|h| {
            vocabulary.is_clause_word(h) && !matches!(h, "needs" | "uses" | "requires")
        }) || form
            .items()
            .iter()
            .any(|item| holds_other_clause(item, vocabulary)))
}

/// A requirement's value, read for `fact`; a value whose interval cannot be ordered is past the arithmetic (§2).
fn required_value(fact: &Fact, forms: &[Form], item: &Form) -> Result<Value, Refusals> {
    let v = value::read(fact, forms, item.span())
        .map_err(|w| vec![not_of_domain(&fact.name, item.span(), w)])?;
    if value::orderable(&v).is_err() {
        return Err(vec![refusal(
            Cause::PastTheArithmetic,
            Some(&fact.name),
            item.span(),
            "an interval whose endpoints' order is past the exact arithmetic",
        )]);
    }
    Ok(v)
}

/// Read one item of a `requires` clause that is not a `needs`, a `uses` or a `requires`: a constraint.
///
/// # Errors
///
/// Every refusal the item gives (record §8).
pub fn read_constraint(item: &Form, vocabulary: &Vocabulary) -> Result<Requirement, Refusals> {
    if holds_other_clause(item, vocabulary) {
        return Err(vec![refusal(
            Cause::ClauseInRequires,
            named(item),
            item.span(),
            "a clause written inside `requires`, where a constraint stands, is read by nothing",
        )]);
    }
    let Some(head) = named(item) else {
        return Err(vec![refusal(
            Cause::NotAConstraint,
            None,
            item.span(),
            "an item of `requires` is a constraint: a list headed by a fact's name",
        )]);
    };
    let rest = match item {
        Form::List { items, .. } => &items[1..],
        _ => {
            return Err(vec![refusal(
                Cause::BareNameInRequires,
                Some(head),
                item.span(),
                "a bare name inside `requires` states no constraint; presence is written `(needs f)`",
            )]);
        }
    };
    let Some(fact) = vocabulary.fact(head) else {
        return Err(vec![refusal(
            Cause::UndeclaredFact,
            Some(head),
            item.span(),
            format!("a constraint on `{head}`, which the vocabulary does not declare: no rule decides it"),
        )]);
    };
    if rest.is_empty() {
        return Err(vec![refusal(
            Cause::NoConstraintWritten,
            Some(head),
            item.span(),
            "`(f)` inside `requires` states no constraint; presence is written `(needs f)`",
        )]);
    }
    if fact.role == Role::Statement {
        // Checked for its domain and nothing else, bare or under `exactly` (§3 rule 6, R17 3).
        let written = match rest {
            [w @ Form::List { .. }] if w.head() == Some("exactly") => &w.items()[1..],
            _ => rest,
        };
        return value::read(fact, written, item.span())
            .map(|value| Requirement::Statement {
                fact: fact.name.clone(),
                value,
                span: item.span(),
            })
            .map_err(|w| vec![not_of_domain(&fact.name, item.span(), w)]);
    }
    if let Domain::Group(subs) = &fact.domain {
        // `(f (g …) (h …))`: the head and its sub-constraints. A head value beside them is neither form and is read
        // below as the head's value, which refuses it (§2's group row, R19 7).
        if rest
            .iter()
            .all(|r| r.head().is_some_and(|h| subs.iter().any(|s| s == h)))
        {
            let mut parts = Vec::new();
            let mut refused = Vec::new();
            for r in rest {
                match read_constraint(r, vocabulary) {
                    Ok(p) => parts.push(p),
                    Err(mut why) => refused.append(&mut why),
                }
            }
            return if refused.is_empty() {
                Ok(Requirement::Group {
                    fact: fact.name.clone(),
                    parts,
                    span: item.span(),
                })
            } else {
                Err(refused)
            };
        }
    }
    let mut direction = fact.direction;
    let mut forms = rest;
    if let [wrapper @ Form::List { .. }] = rest {
        // `()` has no head and holds nothing, so there is nothing after one to slice.
        let inner = wrapper.items().get(1..).unwrap_or_default();
        let written = match wrapper.head() {
            Some("at-least") => Some(Direction::AtLeast),
            Some("at-most") => Some(Direction::AtMost),
            Some("exactly") => Some(Direction::Exact),
            Some(word @ ("exact" | "includes" | "within")) => {
                return Err(vec![refusal(
                    Cause::DirectionNameAsWrapper,
                    Some(head),
                    item.span(),
                    format!("`{word}` names a direction: a requirement writes its value, and the direction is the fact's"),
                )]);
            }
            _ => None,
        };
        if let Some(written) = written {
            if written != Direction::Exact && written != fact.direction {
                return Err(vec![refusal(
                    Cause::DirectionAgainstFact,
                    Some(head),
                    item.span(),
                    format!(
                        "written against the fact's direction, `{}`",
                        fact.direction.word()
                    ),
                )]);
            }
            if inner.is_empty() {
                return Err(vec![refusal(
                    Cause::EmptyValue,
                    Some(head),
                    item.span(),
                    "a bound with no value",
                )]);
            }
            direction = written;
            forms = inner;
        }
    }
    // A bare value is a bound in the fact's own direction (§1).
    let value = required_value(fact, forms, item)?;
    Ok(Requirement::Constraint {
        fact: fact.name.clone(),
        direction,
        value,
        span: item.span(),
    })
}

/// Read one operand of a `needs`: a presence requirement, or `None` for a name the vocabulary does not declare —
/// a declaration's, left to presence and the closure (record §1).
///
/// # Errors
///
/// A list, whatever its head; an operand naming nothing; a statement fact.
pub fn read_needs(operand: &Form, vocabulary: &Vocabulary) -> Result<Option<Requirement>, Refusal> {
    if matches!(operand, Form::List { .. }) {
        return Err(refusal(
            Cause::ListOperand,
            operand.head(),
            operand.span(),
            "an operand of `needs` is a name; a value or a constraint belongs in `requires`",
        ));
    }
    let Some(name) = named(operand) else {
        return Err(refusal(
            Cause::OperandNamesNothing,
            None,
            operand.span(),
            "an operand of `needs` is a name",
        ));
    };
    let Some(fact) = vocabulary.fact(name) else {
        return Ok(None);
    };
    if fact.role == Role::Statement {
        return Err(refusal(
            Cause::NeedsStatement,
            Some(name),
            operand.span(),
            "a statement fact is the requiring side's word about itself, and is needed by no one",
        ));
    }
    Ok(Some(Requirement::Presence {
        fact: fact.name.clone(),
        span: operand.span(),
    }))
}

/// Judge one operand of a `uses`.
///
/// # Errors
///
/// A list, whatever its head; an operand naming nothing; a vocabulary fact, which is needed and never used.
pub fn read_uses(operand: &Form, vocabulary: &Vocabulary) -> Result<(), Refusal> {
    if matches!(operand, Form::List { .. }) {
        return Err(refusal(
            Cause::ListOperand,
            operand.head(),
            operand.span(),
            "an operand of `uses` is a declaration's name",
        ));
    }
    match named(operand) {
        None => Err(refusal(
            Cause::OperandNamesNothing,
            None,
            operand.span(),
            "an operand of `uses` is a name",
        )),
        Some(name) if vocabulary.fact(name).is_some() => Err(refusal(
            Cause::UsesNamesFact,
            Some(name),
            operand.span(),
            "a fact is needed, never used: write `(needs f)`",
        )),
        Some(_) => Ok(()),
    }
}

/// The `needs` and `uses` lists and the `requires` clauses among `form`'s items, at any depth, never looking inside
/// one of them — where presence reads a `needs` (record §1; R23 1, R24 1).
fn collect<'a>(form: &'a Form, clauses: &mut Vec<&'a Form>, lists: &mut Vec<&'a Form>) {
    for item in form.items().iter().skip(1) {
        match item.head() {
            Some("needs" | "uses") => lists.push(item),
            Some("requires") => clauses.push(item),
            _ if matches!(item, Form::List { .. }) => collect(item, clauses, lists),
            _ => {}
        }
    }
}

/// One clause's own items — its constraints and every `needs` and `uses` inside it — with the `requires` clauses
/// nested in it pushed to `nested`, read on their own (record §3 rule 5, R26 2).
fn own_items<'a>(clause: &'a Form, nested: &mut Vec<&'a Form>) -> Vec<&'a Form> {
    let mut lists = Vec::new();
    collect(clause, nested, &mut lists);
    clause
        .items()
        .iter()
        .skip(1)
        .filter(|i| !matches!(i.head(), Some("needs" | "uses" | "requires")))
        .chain(lists)
        .collect()
}

/// Every item one side writes, at the positions of record §1.
fn side_items(decl: &Form) -> Result<Vec<&Form>, Refusals> {
    let (mut clauses, mut items, mut refused) = (Vec::new(), Vec::new(), Vec::new());
    for clause in decl.items().iter().skip(1) {
        match clause.head() {
            None | Some("offers" | "absent" | "refines") => {}
            Some("needs" | "uses") => items.push(clause),
            Some("requires") => clauses.push(clause),
            Some("platform") => {
                for item in clause.items().iter().skip(1) {
                    match item.head() {
                        Some("needs" | "uses") => items.push(item),
                        Some("requires") => clauses.push(item),
                        _ => refused.push(refusal(
                            Cause::NotPlatformItem,
                            named(item),
                            item.span(),
                            "a `platform` clause holds `needs`, `uses` and `requires` alone",
                        )),
                    }
                }
            }
            Some(_) => collect(clause, &mut clauses, &mut items),
        }
    }
    if !refused.is_empty() {
        return Err(refused);
    }
    while let Some(clause) = clauses.pop() {
        items.extend(own_items(clause, &mut clauses));
    }
    Ok(items)
}

/// Every item read, none abandoning the rest, then the side's constraints checked together.
fn read_items<'a>(
    items: impl IntoIterator<Item = &'a Form>,
    vocabulary: &Vocabulary,
) -> Result<Vec<Requirement>, Refusals> {
    let (mut out, mut refused) = (Vec::new(), Vec::new());
    for item in items {
        match item.head() {
            Some("needs") => {
                for operand in &item.items()[1..] {
                    match read_needs(operand, vocabulary) {
                        Ok(Some(r)) => out.push(r),
                        Ok(None) => {}
                        Err(r) => refused.push(r),
                    }
                }
            }
            Some("uses") => {
                for operand in &item.items()[1..] {
                    if let Err(r) = read_uses(operand, vocabulary) {
                        refused.push(r);
                    }
                }
            }
            _ => match read_constraint(item, vocabulary) {
                Ok(r) => out.push(r),
                Err(mut why) => refused.append(&mut why),
            },
        }
    }
    refused.extend(check_together(&out, vocabulary));
    if refused.is_empty() {
        Ok(out)
    } else {
        Err(refused)
    }
}

/// Read one `requires` clause: its own constraints and every `needs` and `uses` inside them, a `requires` nested in
/// it being a clause of its own (record §3 rule 5, R26 2).
///
/// # Errors
///
/// Every refusal its items or their contradictions give.
pub fn read_clause(clause: &Form, vocabulary: &Vocabulary) -> Result<Vec<Requirement>, Refusals> {
    let mut nested = Vec::new();
    read_items(own_items(clause, &mut nested), vocabulary)
}

/// Read one side: every item at record §1's positions, every clause together, so a contradiction split across two
/// clauses is one (record §3 rules 5 and 6; R21 1, R23 1, R24 1). A declaration whose local name is a vocabulary fact
/// is refused first (§1.1, R16 1).
///
/// # Errors
///
/// Every refusal its name, its positions, its items or their contradictions give.
pub fn read_side(decl: &Form, vocabulary: &Vocabulary) -> Result<Vec<Requirement>, Refusals> {
    let mut refused: Refusals = declarations_named_like_facts(std::iter::once(decl), vocabulary);
    match side_items(decl) {
        Ok(items) => match read_items(items, vocabulary) {
            Ok(requirements) if refused.is_empty() => return Ok(requirements),
            Ok(_) => {}
            Err(mut why) => refused.append(&mut why),
        },
        Err(mut why) => refused.append(&mut why),
    }
    Err(refused)
}

/// Every `needs` and `uses` list one side writes, at record §1's positions; none for a side refused there.
#[must_use]
pub fn side_name_lists(decl: &Form) -> Vec<&Form> {
    side_items(decl)
        .unwrap_or_default()
        .into_iter()
        .filter(|item| matches!(item.head(), Some("needs" | "uses")))
        .collect()
}

/// Every declaration among `declarations` whose local name is a vocabulary fact: module resolution binds a `uses`,
/// `needs` or `refines` operand to a declaration of its own instance first, so such a declaration would capture a
/// requirement on the fact, and an import would rename it out of the relation's sight (record §1.1, R16 1; `SR-H4`).
#[must_use]
pub fn declarations_named_like_facts<'a>(
    declarations: impl IntoIterator<Item = &'a Form>,
    vocabulary: &Vocabulary,
) -> Refusals {
    declarations
        .into_iter()
        .filter_map(|decl| {
            let name = decl.items().get(1).and_then(Form::as_symbol)?;
            vocabulary.fact(name).map(|_| {
                refusal(
                    Cause::DeclarationNamedLikeFact,
                    Some(name),
                    decl.span(),
                    "a declaration whose local name is a vocabulary fact would capture every requirement on that fact",
                )
            })
        })
        .collect()
}

/// [`declarations_named_like_facts`] over an elaborated module tree: every instance's own declarations, by the local
/// name each is written with — at the root and inside every imported module (`SR-H4`).
#[must_use]
pub fn program_declarations_named_like_facts(
    program: &Program,
    vocabulary: &Vocabulary,
) -> Refusals {
    declarations_named_like_facts(
        program
            .instances
            .iter()
            .flat_map(|instance| instance.declarations.iter()),
        vocabulary,
    )
}

/// One constraint flattened to the fact it is on, its direction and its value.
struct Flat<'r> {
    fact: &'r str,
    direction: Direction,
    value: Value,
    span: Span,
}

fn flatten<'r>(r: &'r Requirement, vocabulary: &Vocabulary, out: &mut Vec<Flat<'r>>) {
    match r {
        Requirement::Constraint {
            fact,
            direction,
            value,
            span,
        } => out.push(Flat {
            fact,
            direction: *direction,
            value: value.clone(),
            span: *span,
        }),
        Requirement::Statement { fact, value, span } => out.push(Flat {
            fact,
            direction: Direction::Exact,
            value: value.clone(),
            span: *span,
        }),
        // A boolean or a group's head named in `needs` is `(f true)` (§3 rule 3).
        Requirement::Presence { fact, span } => {
            if vocabulary
                .fact(fact)
                .is_some_and(|f| matches!(f.domain, Domain::Boolean | Domain::Group(_)))
            {
                out.push(Flat {
                    fact,
                    direction: Direction::Exact,
                    value: Value::Bool(true),
                    span: *span,
                });
            }
        }
        Requirement::Group { fact, parts, span } => {
            out.push(Flat {
                fact,
                direction: Direction::Exact,
                value: Value::Bool(true),
                span: *span,
            });
            for part in parts {
                flatten(part, vocabulary, out);
            }
        }
    }
}

/// Refuse one side's constraints on one fact that no value satisfies together, and say so when the exact arithmetic
/// cannot decide whether they do (record §3 rules 5 and 6). On each fact the equalities must be one value — every two
/// compare equal, a set's implied members joined to each — and that value must meet each bound by some spelling
/// whose comparison does not overflow. Any comparison that refuses proves a contradiction, whatever else overflows;
/// only what neither proves is past the arithmetic (R27 1, R28 1).
#[must_use]
pub fn check_together(requirements: &[Requirement], vocabulary: &Vocabulary) -> Refusals {
    let mut flat = Vec::new();
    for r in requirements {
        flatten(r, vocabulary, &mut flat);
    }
    let facts: BTreeSet<&str> = flat.iter().map(|c| c.fact).collect();
    let mut out = Vec::new();
    for name in facts {
        let Some(fact) = vocabulary.fact(name) else {
            continue;
        };
        let admitted = |v: &Value| match v {
            Value::Set(members) => {
                let mut all = members.clone();
                all.extend(fact.implies.iter().cloned());
                Value::Set(all)
            }
            other => other.clone(),
        };
        let on_it: Vec<&Flat> = flat.iter().filter(|c| c.fact == name).collect();
        let equalities: Vec<(Value, Span)> = on_it
            .iter()
            .filter(|c| c.direction == Direction::Exact)
            .map(|c| (admitted(&c.value), c.span))
            .collect();
        let mut contradiction = None;
        let mut decided = true;
        for (i, (a, _)) in equalities.iter().enumerate() {
            for (b, at) in &equalities[i + 1..] {
                match value::same(a, b) {
                    Ok(true) => {}
                    Ok(false) => contradiction = contradiction.or(Some(*at)),
                    Err(value::Overflow) => decided = false,
                }
            }
        }
        for bound in on_it.iter().filter(|c| c.direction != Direction::Exact) {
            let mut met = equalities.is_empty();
            for (v, _) in &equalities {
                match value::satisfies(
                    v,
                    &bound.value,
                    bound.direction,
                    fact.order(),
                    &fact.implies,
                ) {
                    Ok(true) => met = true,
                    Ok(false) => contradiction = contradiction.or(Some(bound.span)),
                    Err(value::Overflow) => {}
                }
            }
            decided &= met;
        }
        if let Some(at) = contradiction {
            out.push(refusal(
                Cause::Contradiction,
                Some(name),
                at,
                "this side's constraints on the fact are satisfied by no value together: contradictory requirements are \
                 refused, never one chosen",
            ));
        } else if !decided {
            out.push(refusal(
                Cause::PastTheArithmetic,
                Some(name),
                on_it.first().map_or_else(|| requirements[0].span(), |c| c.span),
                "whether this side's constraints on the fact hold together is past the exact arithmetic",
            ));
        }
    }
    out
}
