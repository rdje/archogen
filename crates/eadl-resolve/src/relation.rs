//! The relation: when what one provider offers satisfies one requirement, and the enumeration of every provider's
//! answer (`docs/decisions/decision_substitutability-relation.md` §3, §4, §5; leaf `M3.1.2.5`).
//!
//! ⭐ **A predicate between one requirement and one provider** (record §3). Rule 1 gives each fact one outcome at
//! the provider — valued, absent, unknown, derived or undescribed, in that order — and rule 3 judges the requirement
//! against it: a value in the requirement's written direction satisfies, `exact` admitting no better value, and a
//! set judged by inclusion, so no stronger precondition passes as a stronger capability (§3 rule 4). Nothing here
//! chooses among providers, ranks them or joins two of them: no lattice is claimed (§7). What several providers make
//! of a description is `M3.4`'s; §5's enumeration hands it each provider's answer, with the value found, the
//! derivation used and the direction.

use crate::offer::{Provider, State};
use crate::requirement::Requirement;
use crate::value::{self, Overflow, Value};
use crate::vocabulary::{Direction, Domain, Rule, Vocabulary};

/// Rule 1's outcome of a fact at one provider (record §3 rule 1).
#[derive(Debug, Clone, PartialEq)]
pub enum Outcome {
    /// The value the provider offers, in every spelling written.
    Valued(Vec<Value>),
    /// Declared absent, or a required input of its derivation declared absent where the fact is not offered.
    Absent,
    /// Offered, or an input its rule reads offered, without a value.
    Unknown,
    /// Computed by the fact's rule from values the provider has.
    Derived {
        /// The value computed.
        value: Value,
        /// The rule that computed it.
        rule: Rule,
    },
    /// None of these.
    Undescribed,
}

/// The outcome of `fact` at `provider` (record §3 rule 1, §4).
///
/// # Errors
///
/// [`Overflow`]: comparing the provider's spellings, or the derivation's arithmetic, overflows — §2's
/// `unsupported-profile`, no outcome (record §3 rule 1, R13 M10).
pub fn outcome(
    provider: &Provider,
    fact: &str,
    vocabulary: &Vocabulary,
) -> Result<Outcome, Overflow> {
    match provider.state(fact) {
        Some(State::Valued(spellings)) => return Ok(Outcome::Valued(spellings.clone())),
        Some(State::Absent) => return Ok(Outcome::Absent),
        Some(State::Unvalued) => return Ok(Outcome::Unknown),
        Some(State::Overflow) => return Err(Overflow),
        None => {}
    }
    let Some(entry) = vocabulary.fact(fact) else {
        return Ok(Outcome::Undescribed);
    };
    let Some(rule) = entry.rule else {
        return Ok(Outcome::Undescribed);
    };
    let state = |name: &String| provider.state(name);
    // Where the provider does not offer the fact: a required input declared absent is absent (R12 L3); an input the
    // rule reads, required or optional, offered without a value is unknown (R3 C5, R12 L15); an optional input
    // declared absent derives nothing (R7 G3).
    if entry
        .derived_from
        .iter()
        .any(|i| state(i) == Some(&State::Absent))
    {
        return Ok(Outcome::Absent);
    }
    if entry
        .derived_from
        .iter()
        .chain(&entry.reads)
        .any(|i| state(i) == Some(&State::Unvalued))
    {
        return Ok(Outcome::Unknown);
    }
    if entry.reads.iter().any(|i| state(i) == Some(&State::Absent)) {
        return Ok(Outcome::Undescribed);
    }
    // Every required input must have a value, offered or itself derived. The first spelling of each: spellings in two
    // units were compared in the base unit as the provider was read, so each converts to the one value (§5, R25 1).
    let mut inputs = Vec::new();
    for name in &entry.derived_from {
        match outcome(provider, name, vocabulary)? {
            Outcome::Valued(spellings) => inputs.extend(spellings.into_iter().next()),
            Outcome::Derived { value, .. } => inputs.push(value),
            _ => return Ok(Outcome::Undescribed),
        }
    }
    match (rule, inputs.as_slice()) {
        // `wrap-behavior` is `modular` or undescribed here: `saturating` beside a modulus was refused (§4).
        (Rule::HorizonFromModulusAndRate, [Value::Count(modulus), Value::Quantity(rate)]) => {
            value::horizon(*modulus, *rate).map(|h| Outcome::Derived {
                value: Value::Quantity(h),
                rule,
            })
        }
        _ => Ok(Outcome::Undescribed),
    }
}

/// What judging one requirement at one provider gives (record §3).
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
    /// §2's `unsupported-profile`: the exact arithmetic's limit.
    Unsupported,
}

impl Verdict {
    /// A group's rank of the verdicts that are not satisfaction: the first of these among its head and its parts is
    /// its verdict, whatever order they are written in (record §3 rule 5, R17 4, R22 4).
    const GROUP_RANK: [Self; 5] = [
        Self::Refused,
        Self::Absent,
        Self::Unsupported,
        Self::Unknown,
        Self::Undescribed,
    ];
}

/// Rule 3 over one value's spellings: decided by the first comparison whose arithmetic does not overflow, the value
/// being one so every such comparison agrees; `unsupported-profile` only when each overflows (§5, R25 1).
fn against(
    spellings: &[Value],
    required: &Value,
    direction: Direction,
    order: &[String],
    implied: &[String],
) -> Verdict {
    spellings
        .iter()
        .find_map(|v| value::satisfies(v, required, direction, order, implied).ok())
        .map_or(Verdict::Unsupported, |holds| {
            if holds {
                Verdict::Satisfied
            } else {
                Verdict::Refused
            }
        })
}

/// The verdict an outcome with no value to compare gives.
fn without_value(outcome: &Outcome) -> Verdict {
    match outcome {
        Outcome::Absent => Verdict::Absent,
        Outcome::Unknown => Verdict::Unknown,
        _ => Verdict::Undescribed,
    }
}

/// Judge `requirement` at `provider` (record §3 rules 3–6).
#[must_use]
pub fn judge(provider: &Provider, requirement: &Requirement, vocabulary: &Vocabulary) -> Verdict {
    match requirement {
        Requirement::Statement { .. } => Verdict::Statement,
        Requirement::Presence { fact, .. } => {
            let boolean = vocabulary
                .fact(fact)
                .is_some_and(|f| matches!(f.domain, Domain::Boolean | Domain::Group(_)));
            match outcome(provider, fact, vocabulary) {
                Err(Overflow) => Verdict::Unsupported,
                // A boolean or a group's head named in `needs` is `(f true)`: `false` meets no presence (§3 rule 3).
                Ok(Outcome::Valued(spellings)) if boolean => {
                    if spellings.first() == Some(&Value::Bool(true)) {
                        Verdict::Satisfied
                    } else {
                        Verdict::Refused
                    }
                }
                Ok(Outcome::Derived { value, .. }) if boolean => {
                    if value == Value::Bool(true) {
                        Verdict::Satisfied
                    } else {
                        Verdict::Refused
                    }
                }
                Ok(Outcome::Valued(_) | Outcome::Derived { .. }) => Verdict::Satisfied,
                // Any offer meets presence, bare or an abstract platform's bound, as no valued constraint is met by
                // one (§3 rule 3, R8 H5); unknown only through an input is not an offer of the fact.
                Ok(Outcome::Unknown) if provider.state(fact).is_some() => Verdict::Satisfied,
                Ok(other) => without_value(&other),
            }
        }
        Requirement::Constraint {
            fact,
            direction,
            value,
            ..
        } => {
            let Some(entry) = vocabulary.fact(fact) else {
                return Verdict::Undescribed;
            };
            match outcome(provider, fact, vocabulary) {
                Err(Overflow) => Verdict::Unsupported,
                Ok(Outcome::Valued(spellings)) => {
                    against(&spellings, value, *direction, entry.order(), &entry.implies)
                }
                Ok(Outcome::Derived { value: derived, .. }) => {
                    against(&[derived], value, *direction, entry.order(), &entry.implies)
                }
                Ok(other) => without_value(&other),
            }
        }
        Requirement::Group { fact, parts, span } => {
            // The head, judged as `(f true)`, ranked among the sub-constraints, each on its own sub-fact (§3 rule 5).
            let head = Requirement::Constraint {
                fact: fact.clone(),
                direction: Direction::Exact,
                value: Value::Bool(true),
                span: *span,
            };
            let verdicts: Vec<Verdict> = std::iter::once(&head)
                .chain(parts)
                .map(|r| judge(provider, r, vocabulary))
                .collect();
            Verdict::GROUP_RANK
                .into_iter()
                .find(|v| verdicts.contains(v))
                .unwrap_or(Verdict::Satisfied)
        }
    }
}

/// Rule 5: a `requires` clause is satisfied by `provider` when each of its constraints is, a statement apart, a
/// group's sub-constraints counting through the group's own verdict (record §3 rule 5).
#[must_use]
pub fn clause_satisfied(
    provider: &Provider,
    clause: &[Requirement],
    vocabulary: &Vocabulary,
) -> bool {
    clause
        .iter()
        .filter(|r| !matches!(r, Requirement::Statement { .. }))
        .all(|r| judge(provider, r, vocabulary) == Verdict::Satisfied)
}

/// One provider's answer to one requirement, as §5's enumeration lists it.
#[derive(Debug, Clone, PartialEq)]
pub struct Candidate {
    /// The provider's declaration name.
    pub provider: String,
    /// Rule 3's verdict.
    pub verdict: Verdict,
    /// Rule 1's outcome of the requirement's fact, the value found and the derivation used among it; `None` for a
    /// statement, which no provider answers, and for an outcome past the arithmetic.
    pub outcome: Option<Outcome>,
    /// The direction the requirement is judged in: the fact's, or `exact` for `exactly` and a group's head.
    pub direction: Direction,
}

/// §5's enumeration of one requirement over `providers`: each provider, in the order given, with its outcome under
/// rule 1, the value, the derivation used and the direction, and rule 3's verdict. It does not choose, rank, resolve
/// `uses` or combine providers: what the list makes of a description is `M3.4`'s (record §5).
#[must_use]
pub fn enumerate(
    requirement: &Requirement,
    providers: &[Provider],
    vocabulary: &Vocabulary,
) -> Vec<Candidate> {
    let direction = match requirement {
        Requirement::Constraint { direction, .. } => *direction,
        _ => Direction::Exact,
    };
    providers
        .iter()
        .map(|provider| Candidate {
            provider: provider.name.clone(),
            verdict: judge(provider, requirement, vocabulary),
            outcome: match requirement {
                Requirement::Statement { .. } => None,
                _ => outcome(provider, requirement.fact(), vocabulary).ok(),
            },
            direction,
        })
        .collect()
}

/// §5's verdict per `requires` clause and provider: the clause satisfied by that provider or not, each provider in the
/// order given (record §5, R3 C8).
#[must_use]
pub fn enumerate_clause(
    clause: &[Requirement],
    providers: &[Provider],
    vocabulary: &Vocabulary,
) -> Vec<(String, bool)> {
    providers
        .iter()
        .map(|provider| {
            (
                provider.name.clone(),
                clause_satisfied(provider, clause, vocabulary),
            )
        })
        .collect()
}
