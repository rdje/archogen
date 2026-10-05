//! Rule 1's outcome at one provider, §4's derivation, and rule 3's judgement of one requirement.

use super::provider::{Provider, Stated};
use super::requirement::Requirement;
use super::value::{self, satisfies, Overflow, Value};
use super::vocab::{self, Domain, Rule};

/// Rule 1's outcome of a fact at one provider.
#[derive(Debug, Clone)]
pub enum Outcome {
    /// The value the provider offers.
    Valued(Value),
    /// Declared absent, or a required input of its derivation declared absent.
    Absent,
    /// Offered, or an input its rule reads offered, without a value.
    Unknown,
    /// Computed by the fact's rule from values at the provider.
    Derived(Value),
    /// None of these.
    Undescribed,
}

/// The outcome of `fact` at `p` (record §3 rule 1, §4).
///
/// # Errors
///
/// [`Overflow`]: the derivation's arithmetic overflows — §2's `unsupported-profile`, no outcome.
pub fn outcome(p: &Provider, fact: &str) -> Result<Outcome, Overflow> {
    match p.stated.get(fact) {
        Some(Stated::Valued(v)) => return Ok(Outcome::Valued(v.clone())),
        Some(Stated::Absent) => return Ok(Outcome::Absent),
        Some(Stated::Unvalued) => return Ok(Outcome::Unknown),
        Some(Stated::Overflow) => return Err(Overflow),
        None => {}
    }
    let Some(e) = vocab::entry(fact) else {
        return Ok(Outcome::Undescribed);
    };
    let Some(rule) = e.rule else {
        return Ok(Outcome::Undescribed);
    };
    // Where `P` does not offer `f`: a required input declared absent is absent (R12 L3) ...
    if e.derived_from
        .iter()
        .any(|i| matches!(p.stated.get(*i), Some(Stated::Absent)))
    {
        return Ok(Outcome::Absent);
    }
    // ... an input the rule reads, required or optional, offered without a value is unknown (R3 C5; R12 L15) ...
    if e.derived_from
        .iter()
        .chain(e.reads.iter())
        .any(|i| matches!(p.stated.get(*i), Some(Stated::Unvalued)))
    {
        return Ok(Outcome::Unknown);
    }
    // ... an optional input declared absent derives nothing (R7 G3).
    if e.reads
        .iter()
        .any(|i| matches!(p.stated.get(*i), Some(Stated::Absent)))
    {
        return Ok(Outcome::Undescribed);
    }
    match rule {
        Rule::HorizonFromModulusAndRate => {
            let modulus = match outcome(p, "counter-modulus")? {
                Outcome::Valued(Value::Count(m)) | Outcome::Derived(Value::Count(m)) => m,
                _ => return Ok(Outcome::Undescribed),
            };
            let rate = match outcome(p, "tick-rate")? {
                Outcome::Valued(Value::Quantity(r)) | Outcome::Derived(Value::Quantity(r)) => r,
                _ => return Ok(Outcome::Undescribed),
            };
            // `wrap-behavior` `modular` or undescribed; `saturating` beside a modulus was refused (§4).
            value::horizon(modulus, rate).map(|h| Outcome::Derived(Value::Quantity(h)))
        }
    }
}

/// What judging one requirement at one provider gives.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Verdict {
    /// The requirement holds at the provider.
    Satisfied,
    /// A value, offered or derived, lies outside the requirement.
    Refused,
    /// Rule 1's outcome `absent`.
    Absent,
    /// Rule 1's outcome `unknown`.
    Unknown,
    /// Rule 1's outcome `undescribed`.
    Undescribed,
    /// A statement: checked for its domain, judged by no provider (§3 rule 6).
    Statement,
    /// §2's `unsupported-profile`: the arithmetic's limit.
    Unsupported,
}

fn of(outcome: Outcome) -> Verdict {
    match outcome {
        Outcome::Absent => Verdict::Absent,
        Outcome::Unknown => Verdict::Unknown,
        _ => Verdict::Undescribed,
    }
}

/// Judge `r` at `p` (record §3 rules 3–6).
#[must_use]
pub fn judge(p: &Provider, r: &Requirement) -> Verdict {
    match r {
        Requirement::Statement(..) => Verdict::Statement,
        Requirement::Presence(fact) => {
            let e = vocab::entry(fact).expect("a presence requirement names a vocabulary fact");
            match outcome(p, fact) {
                Err(Overflow) => Verdict::Unsupported,
                // A boolean or a group's head named in `needs` is `(f true)`, so `false` satisfies no presence.
                Ok(Outcome::Valued(Value::Bool(b)) | Outcome::Derived(Value::Bool(b)))
                    if matches!(e.domain, Domain::Boolean | Domain::Group(_)) =>
                {
                    if b {
                        Verdict::Satisfied
                    } else {
                        Verdict::Refused
                    }
                }
                Ok(Outcome::Valued(_) | Outcome::Derived(_)) => Verdict::Satisfied,
                // Any offer satisfies presence, bare or an abstract platform's bound (§3 rule 3; R8 H5): an
                // offered fact without a value is unknown for a constraint, and present for presence.
                Ok(Outcome::Unknown) if p.stated.contains_key(fact.as_str()) => Verdict::Satisfied,
                Ok(o) => of(o),
            }
        }
        Requirement::Constraint {
            fact,
            direction,
            value,
        } => {
            let e = vocab::entry(fact).expect("a constraint names a vocabulary fact");
            // The members a set fact's entry implies join the required set under every written direction (§1.1's
            // `implies`; R16 2, R17 1, 2); an ordered enumeration's order is the entry's.
            let order: &[&str] = match e.domain {
                Domain::Enumeration {
                    alternatives,
                    ordered: true,
                } => alternatives,
                _ => &[],
            };
            let implied = e.implies;
            match outcome(p, fact) {
                Err(Overflow) => Verdict::Unsupported,
                Ok(Outcome::Valued(v) | Outcome::Derived(v)) => {
                    match satisfies(&v, value, *direction, order, implied) {
                        Ok(true) => Verdict::Satisfied,
                        Ok(false) => Verdict::Refused,
                        Err(Overflow) => Verdict::Unsupported,
                    }
                }
                Ok(o) => of(o),
            }
        }
        Requirement::Group { fact, parts } => {
            // The head first; then every sub-constraint on its own sub-fact, the group satisfied when each is, and
            // otherwise the first of refused, absent, unsupported, unknown and undescribed among them, whatever order
            // they are written in (§3 rule 5; R17 4).
            let head = judge(
                p,
                &Requirement::Constraint {
                    fact: fact.clone(),
                    direction: vocab::Direction::Exact,
                    value: Value::Bool(true),
                },
            );
            if head != Verdict::Satisfied {
                return head;
            }
            let verdicts: Vec<Verdict> = parts.iter().map(|part| judge(p, part)).collect();
            [
                Verdict::Refused,
                Verdict::Absent,
                Verdict::Unsupported,
                Verdict::Unknown,
                Verdict::Undescribed,
            ]
            .into_iter()
            .find(|v| verdicts.contains(v))
            .unwrap_or(Verdict::Satisfied)
        }
    }
}

/// Rule 5: a `requires` clause is satisfied by `p` when each of its constraints is, a statement apart; a group's
/// sub-constraints count among its constraints through the group's own verdict (record §3 rule 5; R18 6). What an
/// unsatisfied clause makes of a description is `M3.4`'s.
#[must_use]
pub fn clause_satisfied(p: &Provider, constraints: &[Requirement]) -> bool {
    constraints
        .iter()
        .filter(|r| !matches!(r, Requirement::Statement(..)))
        .all(|r| judge(p, r) == Verdict::Satisfied)
}
