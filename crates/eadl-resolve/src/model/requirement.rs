//! A requirement read from what a side writes: a constraint inside `requires`, a fact named in `needs`, and what
//! `uses` may not name (record §1's definitions, §8).

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
    /// `invalid-description`: §8's first row.
    Invalid(String),
    /// `unsupported-profile`: a constraint on a fact the vocabulary does not declare (§8's second row), or a value
    /// past the exact arithmetic (§2).
    Unsupported(String),
    /// Not the relation's: a name that is not a vocabulary fact, where presence or the closure judges it.
    NotAFact,
}

/// Why an operand of `needs` or `uses` that names nothing is refused (§8; R23 3).
const NO_NAME_OPERAND: &str = "an operand of `needs` or `uses` is a name or a list headed by one";

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
/// [`NotJudged::Invalid`] for a list naming a vocabulary fact, or a statement fact; [`NotJudged::NotAFact`] for a
/// name that is not one.
pub fn read_needs(item: &Form) -> Result<Requirement, NotJudged> {
    let Some(head) = head_of(item) else {
        return invalid(NO_NAME_OPERAND);
    };
    let Some(e) = vocab::entry(head) else {
        return Err(NotJudged::NotAFact);
    };
    if matches!(item, Form::List { .. }) {
        return invalid(
            "a list inside `needs` naming a vocabulary fact; a value belongs in `requires` (R14 7)",
        );
    }
    if e.role == Role::Statement {
        return invalid("a `needs` of a statement fact (§3 rule 6)");
    }
    Ok(Requirement::Presence(e.name.to_string()))
}

/// Judge one operand of a `uses`: a vocabulary fact, bare or as a list, is refused (§1; R15 1).
///
/// # Errors
///
/// [`NotJudged::Invalid`] when it names a vocabulary fact.
pub fn read_uses(item: &Form) -> Result<(), NotJudged> {
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

/// The `requires` clauses and the `needs` and `uses` lists inside `form`, itself included, at any depth, never looking
/// inside a `needs` or a `uses` — where presence reads `needs` and `uses` (`eadl_model::presence::FactMap::collect`;
/// record §1, §3 rules 3, 5; R23 1, R24 1).
fn walk<'a>(form: &'a Form, clauses: &mut Vec<&'a Form>, lists: &mut Vec<&'a Form>) {
    match form.head() {
        Some("needs" | "uses") => lists.push(form),
        head => {
            if head == Some("requires") {
                clauses.push(form);
            }
            for item in form.items().iter().skip(1) {
                if matches!(item, Form::List { .. }) {
                    walk(item, clauses, lists);
                }
            }
        }
    }
}

/// Every `requires` clause and every `needs` and `uses` one side writes: at any depth of every clause headed by a name
/// but `offers`, `absent` and `refines` — a system's `platform` and its tasks among them — the positions presence
/// reads, so no fact enters the closure that the relation does not read, and no constraint stands where it is not read
/// (record §1; R23 1, R24 1, R24 3).
fn side_parts(decl: &Form) -> (Vec<&Form>, Vec<&Form>) {
    let (mut clauses, mut lists) = (Vec::new(), Vec::new());
    for c in decl.items().iter().skip(1) {
        if !matches!(c.head(), None | Some("offers" | "absent" | "refines")) {
            walk(c, &mut clauses, &mut lists);
        }
    }
    (clauses, lists)
}

/// Every `needs` and `uses` one side writes, at [`side_parts`]' positions.
#[must_use]
pub fn side_name_lists(decl: &Form) -> Vec<&Form> {
    side_parts(decl).1
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

/// The constraints of `clauses` and the `needs` and `uses` of `lists`, read together.
fn read_parts<'a>(
    clauses: &[&'a Form],
    lists: Vec<&'a Form>,
) -> Result<Vec<Requirement>, NotJudged> {
    let items: Vec<&Form> = clauses.iter().flat_map(|c| constraint_items(c)).collect();
    read_items(items.into_iter().chain(lists))
}

/// Read a `requires` clause, its every item, and every `requires`, `needs` and `uses` inside it, then [`check_clause`]
/// (record §3 rule 5).
///
/// # Errors
///
/// As [`read_side`].
pub fn read_clause(clause: &Form) -> Result<Vec<Requirement>, NotJudged> {
    let (mut clauses, mut lists) = (Vec::new(), Vec::new());
    walk(clause, &mut clauses, &mut lists);
    read_parts(&clauses, lists)
}

/// Read one side of a declaration — every `requires` clause, `needs` and `uses` at [`side_parts`]' positions — whose
/// constraints are checked together, since a contradiction split across two clauses is one (record §3 rules 5, 6;
/// R21 1, R23 1, R24 1).
///
/// # Errors
///
/// `invalid-description` when any item, or a contradiction among them, gives it; else `unsupported-profile` when any
/// does — whatever order the items are written in (R21 2).
pub fn read_side(decl: &Form) -> Result<Vec<Requirement>, NotJudged> {
    let (clauses, lists) = side_parts(decl);
    read_parts(&clauses, lists)
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
/// the fact's own direction never contradict. Where no pair clashes and deciding one is past the exact arithmetic, the
/// clause is `unsupported-profile` (§2; R20 2), whatever order its constraints are written in.
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
    for (i, (f, d, v)) in flat.iter().enumerate() {
        for (g, d2, w) in &flat[i + 1..] {
            if f != g {
                continue;
            }
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
            let holds = match (*d == Direction::Exact, *d2 == Direction::Exact) {
                (true, true) => value::same(&admitted(v), &admitted(w)),
                (true, false) => value::satisfies(&admitted(v), w, *d2, order, e.implies),
                (false, true) => value::satisfies(&admitted(w), v, *d, order, e.implies),
                (false, false) => Ok(true),
            };
            let clash = match holds {
                Ok(h) => !h,
                Err(_) => {
                    past.get_or_insert_with(|| f.clone());
                    false
                }
            };
            if clash {
                return invalid(format!(
                    "two constraints on `{f}` that no value satisfies together: contradictory requirements are \
                     refused, never one chosen (§5.3)"
                ));
            }
        }
    }
    match past {
        Some(f) => Err(NotJudged::Unsupported(format!(
            "whether the constraints on `{f}` can hold together is past the exact arithmetic (§2)"
        ))),
        None => Ok(()),
    }
}
