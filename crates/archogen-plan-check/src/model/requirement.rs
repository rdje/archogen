//! A requirement read from what a side writes: a constraint inside `requires`, a fact named in `needs`, and what
//! `uses` may not name (record §1's definitions, §8).

use std::collections::BTreeSet;

use eadl_front::Form;

use super::value::{self, Value};
use super::vocab::{self, Direction, Domain, Role};

/// A requirement on one fact, as rule 3 judges it.
#[derive(Debug, Clone)]
pub enum Requirement {
    /// `(needs f)`: presence, which a boolean or a group's head reads as `(f true)` (§3 rule 3).
    Presence(String),
    /// A value or bound in a direction: the fact's own, or `exactly`.
    Constraint {
        /// The fact.
        fact: String,
        /// The written direction.
        direction: Direction,
        /// The required value.
        value: Value,
    },
    /// A group: the head holds, and each sub-constraint (§2's group row).
    Group {
        /// The head.
        fact: String,
        /// The sub-constraints, each on its own sub-fact.
        parts: Vec<Requirement>,
    },
    /// A statement fact's word: checked for its domain, constraining no provider (§3 rule 6).
    Statement(String, Value),
}

/// What reading a requirement gives besides a requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotJudged {
    /// `invalid-description`: §8's `invalid-description` rows.
    Invalid(String),
    /// `unsupported-profile`: a constraint on a fact the vocabulary does not declare (§8's second row), or a value
    /// past the exact arithmetic (§2).
    Unsupported(String),
    /// Not the relation's: a name that is not a vocabulary fact, where presence or the closure judges it.
    NotAFact,
}

/// Why an operand of `needs` or `uses` that names nothing is refused (§8; R23 3).
const NO_NAME_OPERAND: &str = "an operand of `needs` or `uses` is a name";

/// Why a list operand of `needs` or `uses` is refused, whatever its head: what follows the head would be dropped, a
/// value or a constraint never judged (§1, §8; R14 7, R26 1).
const LIST_OPERAND: &str =
    "a list inside `needs` or `uses`; an operand is a name, and a value or a constraint belongs in `requires`";

/// Why an item of `requires` with no name at its head is refused (§1, §8; R22 1).
const NO_HEAD: &str = "an item of `requires` is a name or a list headed by one";

/// Why a bare name inside `requires` is refused, fact or not (§1, §8; R21 3).
const BARE_NAME: &str =
    "a bare name inside `requires` states no constraint; presence is `(needs f)`";

fn invalid<T>(why: impl Into<String>) -> Result<T, NotJudged> {
    Err(NotJudged::Invalid(why.into()))
}

/// An interval whose endpoints cannot be ordered within the exact arithmetic is §2's `unsupported-profile` (R18 7).
fn ordered(v: &Value) -> Result<(), NotJudged> {
    value::interval_order(v).map_err(|_| {
        NotJudged::Unsupported(
            "an interval whose endpoints' order is past the exact arithmetic (§2)".to_owned(),
        )
    })
}

fn head_of(item: &Form) -> Option<&str> {
    match item {
        Form::Symbol { name, .. } => Some(name.as_str()),
        _ => item.head(),
    }
}

/// Read one item of a `requires` clause that constrains a fact.
///
/// # Errors
///
/// [`NotJudged`] for what the relation refuses or does not judge.
pub fn read_constraint(item: &Form) -> Result<Requirement, NotJudged> {
    // A provider's clause where a constraint stands, at any depth: an offer or an absence nothing reads (§1; R29 1).
    if holds_provider_clause(item) {
        return invalid("a clause written inside `requires`, where a constraint stands");
    }
    let Some(head) = head_of(item) else {
        // An item that is neither a name nor a list headed by one — a number, a string, `()`, a list headed by any of
        // them — is `invalid-description` (§1, §8; R22 1).
        return invalid(NO_HEAD);
    };
    let Some(e) = vocab::entry(head) else {
        // `(x)` or `(x v)` naming no vocabulary fact is a constraint on an undeclared fact, §8's second row; a bare
        // name is presence's, not the relation's (R19 remark 7).
        return match item {
            Form::List { .. } => Err(NotJudged::Unsupported(format!(
                "a constraint on `{head}`, which the vocabulary does not declare"
            ))),
            // A bare name inside `requires` states no constraint, fact or not (§1, §8; R21 3).
            _ => invalid(BARE_NAME),
        };
    };
    let rest: &[Form] =
        match item {
            Form::Symbol { .. } => return invalid(
                "a bare fact name inside `requires` states no constraint; presence is `(needs f)`",
            ),
            Form::List { items, .. } => &items[1..],
            _ => return invalid("a constraint is a list"),
        };
    if rest.is_empty() {
        return invalid("`(f)` inside `requires` states no constraint");
    }
    if e.role == Role::Statement {
        // Checked for its domain and nothing else (§3 rule 6), bare or under `exactly`, the one direction either side
        // may always write (§1.1; R17 3).
        let written = match rest {
            [w @ Form::List { .. }] if w.head() == Some("exactly") => &w.items()[1..],
            _ => rest,
        };
        return value::read(e.domain, written, item.span())
            .map(|v| Requirement::Statement(e.name.to_string(), v))
            .or_else(|m| invalid(m.0));
    }
    if let Domain::Group(subs) = e.domain {
        // `(f true)` / `(f (exactly true))`, or `(f (g …) (h …))`: the head and its sub-constraints.
        let nested = rest
            .iter()
            .all(|r| r.head().is_some_and(|h| subs.contains(&h)));
        // A head value beside sub-constraints is neither form, and is read below as the head's value, which refuses
        // it: `invalid-description` (§2's group row; R19 remark 7).
        if nested {
            // Every part read, ranked as a clause's items are: `invalid-description` over the rest (R22 remark 9).
            let mut parts = Vec::new();
            let mut other = None;
            for r in rest {
                match read_constraint(r) {
                    Ok(p) => parts.push(p),
                    Err(NotJudged::Invalid(w)) => return Err(NotJudged::Invalid(w)),
                    Err(e) => {
                        other.get_or_insert(e);
                    }
                }
            }
            if let Some(e) = other {
                return Err(e);
            }
            return Ok(Requirement::Group {
                fact: e.name.to_string(),
                parts,
            });
        }
    }
    if let [wrapper @ Form::List { .. }] = rest {
        if let Some(word) = wrapper.head() {
            let written = match word {
                "at-least" => Some(Direction::AtLeast),
                "at-most" => Some(Direction::AtMost),
                "exactly" => Some(Direction::Exact),
                "exact" | "includes" | "within" => {
                    return invalid(format!("`{word}` is a direction's name, not a written wrapper; a requirement writes its value (R15 16; R16 4)"));
                }
                _ => None,
            };
            if let Some(dir) = written {
                if dir != Direction::Exact && dir != e.direction {
                    return invalid(format!(
                        "`{word}` written against the fact's direction (§1.1)"
                    ));
                }
                let inner = &wrapper.items()[1..];
                if inner.is_empty() {
                    return invalid("a bound with no value; a set written with no member");
                }
                let v = value::read(e.domain, inner, item.span())
                    .and_then(|v| value::fact_value(e.name, e.domain, &v).map(|()| v))
                    .or_else(|m| invalid(m.0))?;
                ordered(&v)?;
                return Ok(Requirement::Constraint {
                    fact: e.name.to_string(),
                    direction: dir,
                    value: v,
                });
            }
        }
    }
    let v = value::read(e.domain, rest, item.span())
        .and_then(|v| value::fact_value(e.name, e.domain, &v).map(|()| v))
        .or_else(|m| invalid(m.0))?;
    ordered(&v)?;
    // A bare value is a bound in the fact's own direction (§1).
    Ok(Requirement::Constraint {
        fact: e.name.to_string(),
        direction: e.direction,
        value: v,
    })
}

/// Read one operand of a `needs`.
///
/// # Errors
///
/// [`NotJudged::Invalid`] for anything but a name — a list whatever its head (R26 1) — or a statement fact;
/// [`NotJudged::NotAFact`] for a name that is not one.
pub fn read_needs(item: &Form) -> Result<Requirement, NotJudged> {
    if matches!(item, Form::List { .. }) {
        return invalid(LIST_OPERAND);
    }
    let Some(head) = head_of(item) else {
        return invalid(NO_NAME_OPERAND);
    };
    let Some(e) = vocab::entry(head) else {
        return Err(NotJudged::NotAFact);
    };
    if e.role == Role::Statement {
        return invalid("a `needs` of a statement fact (§3 rule 6)");
    }
    Ok(Requirement::Presence(e.name.to_string()))
}

/// Judge one operand of a `uses`: a vocabulary fact is refused (§1; R15 1), and a list whatever its head (R26 1).
///
/// # Errors
///
/// [`NotJudged::Invalid`] when it is a list, names a vocabulary fact or names nothing.
pub fn read_uses(item: &Form) -> Result<(), NotJudged> {
    if matches!(item, Form::List { .. }) {
        return invalid(LIST_OPERAND);
    }
    match head_of(item) {
        Some(head) if vocab::entry(head).is_some() => {
            invalid("a `uses` naming a vocabulary fact; a fact is needed, never used")
        }
        Some(_) => Ok(()),
        None => invalid(NO_NAME_OPERAND),
    }
}

/// Judge a declaration's local name: a vocabulary fact's name is refused (§1.1; R16 1).
///
/// # Errors
///
/// [`NotJudged::Invalid`] when it is one.
pub fn read_declaration_name(name: &str) -> Result<(), NotJudged> {
    if vocab::entry(name).is_some() {
        invalid("a declaration whose local name is a vocabulary fact")
    } else {
        Ok(())
    }
}

/// The `needs` and `uses` lists and the `requires` clauses among the items of `form`, at any depth, never looking
/// inside one of them — where presence reads `needs` and `uses` (`eadl_model::presence::FactMap::collect`), a
/// `requires` nested anywhere being a clause of its own (record §1, §3 rules 3, 5; R23 1, R24 1, R26 2).
fn walk<'a>(form: &'a Form, clauses: &mut Vec<&'a Form>, lists: &mut Vec<&'a Form>) {
    for item in form.items().iter().skip(1) {
        match item.head() {
            Some("needs" | "uses") => lists.push(item),
            Some("requires") => clauses.push(item),
            _ => {
                if matches!(item, Form::List { .. }) {
                    walk(item, clauses, lists);
                }
            }
        }
    }
}

/// One clause's own items — its constraints and the `needs` and `uses` inside them — with the `requires` clauses nested
/// in it pushed to `nested`, never read as its own (record §3 rule 5; R26 2).
fn clause_own<'a>(clause: &'a Form, nested: &mut Vec<&'a Form>) -> Vec<&'a Form> {
    let mut lists = Vec::new();
    walk(clause, nested, &mut lists);
    constraint_items(clause).chain(lists).collect()
}

/// Every clause a kind of `/1` declares — `docs/semantics/kinds/`' `(clause …)` names, held to them by the checker — the
/// words §1's grammar of positions calls a clause (R30 1).
pub const CLAUSE_WORDS: &[&str] = &[
    "absent",
    "deadline",
    "deadline-from",
    "jitter",
    "min-separation",
    "needs",
    "offers",
    "on-overrun",
    "period",
    "platform",
    "priority",
    "refines",
    "requires",
    "task",
    "uses",
];

/// Whether `form` is, or holds at any depth, a list headed by a clause word but those `requires` reads where they
/// stand — `needs`, `uses` and `requires` (§1; R29 1, R30 1).
fn holds_provider_clause(form: &Form) -> bool {
    matches!(form, Form::List { .. })
        && (form.head().is_some_and(|h| {
            CLAUSE_WORDS.contains(&h) && !matches!(h, "needs" | "uses" | "requires")
        }) || form.items().iter().any(holds_provider_clause))
}

/// Every item one side writes: the own items of every `requires` clause and every `needs` and `uses` at any depth of
/// every clause headed by a name but `offers`, `absent` and `refines` — a system's `platform` and its tasks among them
/// — the positions presence reads, so no fact enters the closure that the relation does not read, and no constraint
/// stands where it is not read (record §1; R23 1, R24 1, R24 3, R26 2). A system's `platform` clause holds `needs`,
/// `uses` and `requires` alone: anything else there — an offer, an absence, a constraint — is read by nothing and
/// refused (§1; R29 1).
fn side_items(decl: &Form) -> Result<Vec<&Form>, NotJudged> {
    let (mut clauses, mut items) = (Vec::new(), Vec::new());
    for c in decl.items().iter().skip(1) {
        match c.head() {
            None | Some("offers" | "absent" | "refines") => {}
            Some("needs" | "uses") => items.push(c),
            Some("requires") => clauses.push(c),
            Some("platform") => {
                for item in c.items().iter().skip(1) {
                    match item.head() {
                        Some("needs" | "uses") => items.push(item),
                        Some("requires") => clauses.push(item),
                        _ => {
                            return invalid(
                                "a `platform` clause holds `needs`, `uses` and `requires` alone",
                            )
                        }
                    }
                }
            }
            Some(_) => walk(c, &mut clauses, &mut items),
        }
    }
    while let Some(c) = clauses.pop() {
        items.extend(clause_own(c, &mut clauses));
    }
    Ok(items)
}

/// Every `needs` and `uses` one side writes, at [`side_items`]' positions; none for a side refused there.
#[must_use]
pub fn side_name_lists(decl: &Form) -> Vec<&Form> {
    side_items(decl)
        .unwrap_or_default()
        .into_iter()
        .filter(|i| matches!(i.head(), Some("needs" | "uses")))
        .collect()
}

/// The constraint items of a `requires` clause: its items but the `needs` and `uses` lists and the `requires` clauses
/// inside it, which are read where they stand, at any depth (R24 1).
fn constraint_items(clause: &Form) -> impl Iterator<Item = &Form> {
    clause
        .items()
        .iter()
        .skip(1)
        .filter(|i| !matches!(i.head(), Some("needs" | "uses" | "requires")))
}

/// Read one `requires` clause — its own constraints and every `needs` and `uses` inside them, a `requires` nested in it
/// being a clause of its own with a verdict of its own — then [`check_clause`] (record §3 rule 5; R26 2).
///
/// # Errors
///
/// As [`read_side`].
pub fn read_clause(clause: &Form) -> Result<Vec<Requirement>, NotJudged> {
    let mut nested = Vec::new();
    read_items(clause_own(clause, &mut nested))
}

/// Read one side of a declaration — every item at [`side_items`]' positions, every clause's together — whose
/// constraints are checked together, since a contradiction split across two clauses is one (record §3 rules 5, 6;
/// R21 1, R23 1, R24 1).
///
/// # Errors
///
/// `invalid-description` when any item, or a contradiction among them, gives it; else `unsupported-profile` when any
/// does — whatever order the items are written in (R21 2).
pub fn read_side(decl: &Form) -> Result<Vec<Requirement>, NotJudged> {
    // A declaration whose local name is a vocabulary fact, whatever its kind (§1.1; R16 1, R25 4).
    if let Some(name) = decl.items().get(1).and_then(Form::as_symbol) {
        read_declaration_name(name)?;
    }
    read_items(side_items(decl)?)
}

/// Every item read, none abandoning the rest: a `uses` operand through [`read_uses`], a service's `needs` left to
/// presence (§1), every constraint read; then the contradiction check over those that read.
fn read_items<'a>(
    items: impl IntoIterator<Item = &'a Form>,
) -> Result<Vec<Requirement>, NotJudged> {
    let mut out = Vec::new();
    let (mut invalid_why, mut unsupported_why): (Option<String>, Option<String>) = (None, None);
    let mut fail = |e: NotJudged| match e {
        NotJudged::Invalid(w) => {
            invalid_why.get_or_insert(w);
        }
        NotJudged::Unsupported(w) => {
            unsupported_why.get_or_insert(w);
        }
        // A service, or a name the vocabulary does not declare, in `needs`: presence's and the closure's (§1).
        NotJudged::NotAFact => {}
    };
    for item in items {
        match item.head() {
            Some("needs") => {
                for n in &item.items()[1..] {
                    match read_needs(n) {
                        Ok(r) => out.push(r),
                        Err(e) => fail(e),
                    }
                }
            }
            // A declaration used, never a fact: an operand naming one is refused (§1; R15 1, R20 1).
            Some("uses") => {
                for n in &item.items()[1..] {
                    if let Err(e) = read_uses(n) {
                        fail(e);
                    }
                }
            }
            _ => match read_constraint(item) {
                Ok(r) => out.push(r),
                Err(e) => fail(e),
            },
        }
    }
    if let Err(e) = check_clause(&out) {
        fail(e);
    }
    match (invalid_why, unsupported_why) {
        (Some(w), _) => Err(NotJudged::Invalid(w)),
        (None, Some(w)) => Err(NotJudged::Unsupported(w)),
        (None, None) => Ok(out),
    }
}

/// Refuse constraints of one clause on one fact that no value satisfies together — contradictory requirements, which
/// `ROADMAP.md` §5.3 rejects rather than choosing one (record §3 rules 5, 6; §8; R19 2, remark 8): a statement written
/// with two values, two equalities that differ, or an equality another constraint on its fact refuses. Two bounds in
/// the fact's own direction never contradict. Any comparison that refuses proves a contradiction, whatever else
/// overflows; the constraints hold together when the equalities compare equal — one value in several spellings — and
/// that value meets each bound by some spelling whose comparison does not overflow, as §5 decides an offer. What
/// neither proves is `unsupported-profile` (§2; R20 2, R27 1), whatever order the constraints are written in.
///
/// # Errors
///
/// `invalid-description`, naming the fact; or `unsupported-profile`.
pub fn check_clause(constraints: &[Requirement]) -> Result<(), NotJudged> {
    fn walk(r: &Requirement, flat: &mut Vec<(String, Direction, Value)>) {
        match r {
            Requirement::Constraint {
                fact,
                direction,
                value,
            } => flat.push((fact.clone(), *direction, value.clone())),
            Requirement::Statement(fact, value) => {
                flat.push((fact.clone(), Direction::Exact, value.clone()));
            }
            Requirement::Presence(fact) => {
                // A boolean or a group's head named in `needs` is `(f true)` (§3 rule 3).
                if vocab::entry(fact)
                    .is_some_and(|e| matches!(e.domain, Domain::Boolean | Domain::Group(_)))
                {
                    flat.push((fact.clone(), Direction::Exact, Value::Bool(true)));
                }
            }
            Requirement::Group { fact, parts } => {
                flat.push((fact.clone(), Direction::Exact, Value::Bool(true)));
                for p in parts {
                    walk(p, flat);
                }
            }
        }
    }
    let mut flat = Vec::new();
    for r in constraints {
        walk(r, &mut flat);
    }
    let mut past: Option<String> = None;
    let facts: BTreeSet<&String> = flat.iter().map(|(f, _, _)| f).collect();
    for f in facts {
        let e = vocab::entry(f).expect("a constraint names a vocabulary fact");
        let order: &[&str] = match e.domain {
            Domain::Enumeration {
                alternatives,
                ordered: true,
            } => alternatives,
            _ => &[],
        };
        // What an equality admits: its value, a set's implied members joined (§1.1's `implies`).
        let admitted = |x: &Value| match x {
            Value::Set(s) => {
                let mut s = s.clone();
                s.extend(e.implies.iter().map(|m| (*m).to_string()));
                Value::Set(s)
            }
            _ => x.clone(),
        };
        let equalities: Vec<Value> = flat
            .iter()
            .filter(|(g, d, _)| g == f && *d == Direction::Exact)
            .map(|(_, _, v)| admitted(v))
            .collect();
        let clash = || {
            invalid(format!(
                "two constraints on `{f}` that no value satisfies together: contradictory requirements are \
                 refused, never one chosen (§5.3)"
            ))
        };
        // The equalities: one value in several spellings when every two compare equal within the arithmetic, as a
        // provider's offers are (§5; R27 1, R28 1); two that compare unequal are a contradiction.
        let mut decided = true;
        for (i, v) in equalities.iter().enumerate() {
            for w in &equalities[i + 1..] {
                match value::same(v, w) {
                    Ok(true) => {}
                    Ok(false) => return clash(),
                    Err(_) => decided = false,
                }
            }
        }
        // Each bound against the value, met by any spelling that compares, refused by any that refuses.
        for (_, d, w) in flat
            .iter()
            .filter(|(g, d, _)| g == f && *d != Direction::Exact)
        {
            let mut met = equalities.is_empty();
            for v in &equalities {
                match value::satisfies(v, w, *d, order, e.implies) {
                    Ok(true) => met = true,
                    Ok(false) => return clash(),
                    Err(_) => {}
                }
            }
            decided &= met;
        }
        if !decided {
            past.get_or_insert_with(|| f.to_string());
        }
    }
    match past {
        Some(f) => Err(NotJudged::Unsupported(format!(
            "whether the constraints on `{f}` can hold together is past the exact arithmetic (§2)"
        ))),
        None => Ok(()),
    }
}
