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
    Statement(String),
}

/// What reading a requirement gives besides a requirement.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum NotJudged {
    /// `invalid-description`: §8's first row.
    Invalid(String),
    /// `unsupported-profile`: a constraint on a fact the vocabulary does not declare (§8's second row).
    Unsupported(String),
    /// Not the relation's: a name that is not a vocabulary fact, where presence or the closure judges it.
    NotAFact,
}

fn invalid<T>(why: impl Into<String>) -> Result<T, NotJudged> {
    Err(NotJudged::Invalid(why.into()))
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
        return invalid("a constraint names its fact first");
    };
    let Some(e) = vocab::entry(head) else {
        return match item {
            Form::List { .. } => Err(NotJudged::Unsupported(format!(
                "a constraint on `{head}`, which the vocabulary does not declare"
            ))),
            _ => Err(NotJudged::NotAFact),
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
            .map(|_| Requirement::Statement(e.name.to_string()))
            .or_else(|m| invalid(m.0));
    }
    if let Domain::Group(subs) = e.domain {
        // `(f true)` / `(f (exactly true))`, or `(f (g …) (h …))`: the head and its sub-constraints.
        let nested = rest
            .iter()
            .all(|r| r.head().is_some_and(|h| subs.contains(&h)));
        if nested {
            let mut parts = Vec::new();
            for r in rest {
                parts.push(read_constraint(r)?);
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
        return Err(NotJudged::NotAFact);
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
        _ => Ok(()),
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
