//! Values of the vocabulary's domains, read from written forms and compared in exact arithmetic
//! (`docs/decisions/decision_substitutability-relation.md` §2, §4, §5; leaf `M3.1.2.3`).
//!
//! ⭐ **Exact throughout.** A quantity is `eadl-model`'s `Quantity`, two amounts in one unit compared by their written
//! numbers and in two units in the base unit, as `Rational`s over `i128`; a count is an `i128`, a power of two written
//! `(pow2 N)` included. Nothing is rounded. A comparison whose arithmetic overflows answers [`Overflow`], which the
//! relation reports as `unsupported-profile` — a named limit of the value domain, never a verdict (record §2).

use std::cmp::Ordering;
use std::collections::BTreeSet;

use eadl_front::{Form, Span};
use eadl_model::quantity::{unit, Quantity};
use eadl_model::{Dimension, Rational};

use crate::vocabulary::{Direction, Domain, Fact};

/// The largest `N` a `(pow2 N)` count may write (record §2).
pub const POW2_LIMIT: i64 = 126;

/// The fact whose value may not be 0 (record §4, §8).
pub const COUNTER_MODULUS: &str = "counter-modulus";

/// A value of one of the vocabulary's domains.
#[derive(Debug, Clone, PartialEq)]
pub enum Value {
    /// A boolean, or a group's head.
    Bool(bool),
    /// A count, `0 ..= 2^126`.
    Count(i128),
    /// A quantity of one dimension.
    Quantity(Quantity),
    /// An interval, `lo ≤ hi` where the endpoints can be ordered; a point is `[v, v]`.
    Interval(Quantity, Quantity),
    /// One alternative of an enumeration.
    Enum(String),
    /// A non-empty set of alternatives.
    Set(BTreeSet<String>),
}

/// A comparison the exact arithmetic cannot answer: `unsupported-profile` (record §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Overflow;

/// Why written forms are not a value of the fact's domain: always `invalid-description` (record §2, §8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NotOfDomain {
    /// Which of §8's causes it is.
    pub kind: NotOfDomainKind,
    /// What was written, said in a sentence.
    pub detail: String,
}

/// The causes of [`NotOfDomain`] that §8 names apart.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NotOfDomainKind {
    /// A value outside the domain.
    Outside,
    /// A set written with no member, or `(f (exactly))`.
    Empty,
    /// An interval with `lo > hi`.
    Reversed,
    /// A value of information that is not a positive whole number of bits.
    NotWholeBits,
    /// A `counter-modulus` of 0.
    ZeroModulus,
}

fn outside<T>(detail: impl Into<String>) -> Result<T, NotOfDomain> {
    Err(NotOfDomain {
        kind: NotOfDomainKind::Outside,
        detail: detail.into(),
    })
}

/// The order of two amounts of one dimension: by their written numbers in one unit, needing no conversion; in the
/// base unit otherwise, which may overflow (record §2, §5).
///
/// # Errors
///
/// [`Overflow`] when converting to the base unit overflows.
pub fn order(a: Quantity, b: Quantity) -> Result<Ordering, Overflow> {
    if a.unit == b.unit {
        Ok(a.value.cmp(&b.value))
    } else {
        a.compare(b).map_err(|_| Overflow)
    }
}

/// Whether two values are one value in the fact's domain — `10 MHz` beside `10000 kHz` — the comparison that decides
/// whether a provider offers one value or two (record §5).
///
/// # Errors
///
/// [`Overflow`] when an amount's comparison overflows: neither one value nor two (record §5).
pub fn same(a: &Value, b: &Value) -> Result<bool, Overflow> {
    let equal = |x: Quantity, y: Quantity| order(x, y).map(Ordering::is_eq);
    match (a, b) {
        (Value::Bool(x), Value::Bool(y)) => Ok(x == y),
        (Value::Count(x), Value::Count(y)) => Ok(x == y),
        (Value::Quantity(x), Value::Quantity(y)) => equal(*x, *y),
        // One comparison of two values: both endpoints compared before either decides, so a difference at one never
        // hides an overflow at the other (record §3 rule 5, R28 4).
        (Value::Interval(x0, x1), Value::Interval(y0, y1)) => {
            let (lo, hi) = (equal(*x0, *y0), equal(*x1, *y1));
            Ok(lo? && hi?)
        }
        (Value::Enum(x), Value::Enum(y)) => Ok(x == y),
        (Value::Set(x), Value::Set(y)) => Ok(x == y),
        _ => Ok(false),
    }
}

/// Whether an interval's endpoints can be ordered: one whose cannot has no reading to compare (record §2, §5).
///
/// # Errors
///
/// [`Overflow`] when they cannot.
pub fn orderable(v: &Value) -> Result<(), Overflow> {
    match v {
        Value::Interval(lo, hi) => order(*lo, *hi).map(|_| ()),
        _ => Ok(()),
    }
}

/// Whether `offered` lies in `direction` against `required`, by record §2's "Satisfied when" column. `order` is an
/// ordered enumeration's alternatives, lowest first; `implied` the members a set requirement holds beside what it
/// writes (record §1.1's `implies`).
///
/// # Errors
///
/// [`Overflow`] when a quantity comparison overflows.
pub fn satisfies(
    offered: &Value,
    required: &Value,
    direction: Direction,
    order_of: &[String],
    implied: &[String],
) -> Result<bool, Overflow> {
    let quantities = |o: Quantity, r: Quantity, d: Direction| -> Result<bool, Overflow> {
        let ord = order(o, r)?;
        Ok(match d {
            Direction::AtLeast => ord.is_ge(),
            Direction::AtMost => ord.is_le(),
            _ => ord.is_eq(),
        })
    };
    match (offered, required) {
        (Value::Bool(o), Value::Bool(r)) => Ok(o == r),
        (Value::Count(o), Value::Count(r)) => Ok(match direction {
            Direction::AtLeast => o >= r,
            Direction::AtMost => o <= r,
            _ => o == r,
        }),
        (Value::Quantity(o), Value::Quantity(r)) => quantities(*o, *r, direction),
        (Value::Interval(olo, ohi), Value::Interval(rlo, rhi)) => {
            // `within`: the required interval inside the offered one; under `exactly`, equal. Both endpoints compared
            // before either decides (record §2, R18 7).
            let (lo, hi) = if direction == Direction::Within {
                (
                    quantities(*olo, *rlo, Direction::AtMost),
                    quantities(*ohi, *rhi, Direction::AtLeast),
                )
            } else {
                (
                    quantities(*olo, *rlo, Direction::Exact),
                    quantities(*ohi, *rhi, Direction::Exact),
                )
            };
            Ok(lo? && hi?)
        }
        (Value::Enum(o), Value::Enum(r)) => match direction {
            Direction::AtLeast | Direction::AtMost => {
                let rank = |a: &String| order_of.iter().position(|x| x == a);
                Ok(match (rank(o), rank(r)) {
                    (Some(oi), Some(ri)) if direction == Direction::AtLeast => oi >= ri,
                    (Some(oi), Some(ri)) => oi <= ri,
                    _ => false,
                })
            }
            _ => Ok(o == r),
        },
        (Value::Set(o), Value::Set(r)) => {
            let mut wanted = r.clone();
            wanted.extend(implied.iter().cloned());
            Ok(if direction == Direction::Includes {
                wanted.is_subset(o)
            } else {
                *o == wanted
            })
        }
        _ => Ok(false),
    }
}

/// `(modulus − 1) / rate` seconds, exact: §4's `horizon-from-modulus-and-rate`.
///
/// # Errors
///
/// [`Overflow`] when the arithmetic overflows `i128` (record §2, §4).
pub fn horizon(modulus: i128, rate: Quantity) -> Result<Quantity, Overflow> {
    let hertz = rate.in_base().map_err(|_| Overflow)?;
    let ticks = Rational::new(modulus.checked_sub(1).ok_or(Overflow)?, 1).ok_or(Overflow)?;
    let seconds = ticks.checked_div(hertz).ok_or(Overflow)?;
    Quantity::new(seconds, unit("s").ok_or(Overflow)?).map_err(|_| Overflow)
}

fn quantity(
    number: &Form,
    unit_form: &Form,
    within: Span,
    dim: Dimension,
) -> Result<Quantity, NotOfDomain> {
    match Quantity::read(Some(number), Some(unit_form), within) {
        Ok(q) if q.unit.dimension == dim => Ok(q),
        Ok(q) => outside(format!(
            "a quantity of {} where the fact's domain is {}",
            q.unit.dimension.slug(),
            dim.slug()
        )),
        Err(d) => outside(format!("{}: {}", d.code, d.message)),
    }
}

fn count(forms: &[Form]) -> Result<i128, NotOfDomain> {
    match forms {
        [Form::Integer { value, .. }] if *value >= 0 => Ok(i128::from(*value)),
        [Form::Integer { value, .. }, Form::Symbol { name, .. }] if *value >= 0 => match unit(name)
        {
            Some(u) if u.dimension == Dimension::Dimensionless => Ok(i128::from(*value)),
            _ => outside(format!("`{name}` is not a dimensionless unit")),
        },
        [power @ Form::List { .. }] if power.head() == Some("pow2") => match power.items() {
            [_, Form::Integer { value, .. }] if (0..=POW2_LIMIT).contains(value) => {
                Ok(1i128 << *value)
            }
            _ => outside("`(pow2 N)` writes `N` as an integer from 0 to 126"),
        },
        _ => outside(
            "a count is a non-negative integer, with an optional dimensionless unit, or `(pow2 N)`",
        ),
    }
}

/// Read the value `forms` write for `fact`, `within` the form holding them: its domain's reading, then what §8 refuses
/// of a value wherever it is written — information that is not a positive whole number of bits, a `counter-modulus`
/// of 0 (record §2, §4, §8).
///
/// # Errors
///
/// [`NotOfDomain`], `invalid-description`.
pub fn read(fact: &Fact, forms: &[Form], within: Span) -> Result<Value, NotOfDomain> {
    let value = read_domain(&fact.domain, forms, within)?;
    if let (Domain::Quantity(Dimension::Information), Value::Quantity(q)) = (&fact.domain, &value) {
        if !q
            .in_base()
            .is_ok_and(|bits| bits.is_integer() && bits.is_positive())
        {
            return Err(NotOfDomain {
                kind: NotOfDomainKind::NotWholeBits,
                detail: "a value of information that is not a positive whole number of bits"
                    .to_string(),
            });
        }
    }
    if fact.name == COUNTER_MODULUS && value == Value::Count(0) {
        return Err(NotOfDomain {
            kind: NotOfDomainKind::ZeroModulus,
            detail: "a `counter-modulus` of 0: a counter wraps at a positive count".to_string(),
        });
    }
    Ok(value)
}

fn read_domain(domain: &Domain, forms: &[Form], within: Span) -> Result<Value, NotOfDomain> {
    match domain {
        Domain::Boolean | Domain::Group(_) => match forms {
            [Form::Symbol { name, .. }] if name == "true" => Ok(Value::Bool(true)),
            [Form::Symbol { name, .. }] if name == "false" => Ok(Value::Bool(false)),
            _ => outside("a boolean is `true` or `false`"),
        },
        Domain::Count => count(forms).map(Value::Count),
        Domain::Quantity(dim) => match forms {
            [number, unit_form] => quantity(number, unit_form, within, *dim).map(Value::Quantity),
            _ => outside("a quantity is a number and a unit"),
        },
        Domain::Interval(dim) => match forms {
            [range @ Form::List { .. }] if range.head() == Some("range") => match range.items() {
                [_, lo_n, lo_u, hi_n, hi_u] => {
                    let lo = quantity(lo_n, lo_u, within, *dim)?;
                    let hi = quantity(hi_n, hi_u, within, *dim)?;
                    // Endpoints whose order is past the arithmetic are read; `orderable` makes the value
                    // `unsupported-profile` where it is judged, never `invalid-description` (record §2, R18 7).
                    if order(lo, hi) == Ok(Ordering::Greater) {
                        return Err(NotOfDomain {
                            kind: NotOfDomainKind::Reversed,
                            detail: "an interval written with `lo > hi`".to_string(),
                        });
                    }
                    Ok(Value::Interval(lo, hi))
                }
                _ => outside("`(range lo hi)` writes two quantities"),
            },
            [number, unit_form] => {
                let point = quantity(number, unit_form, within, *dim)?;
                Ok(Value::Interval(point, point))
            }
            _ => outside("an interval is `(range lo hi)` or a point"),
        },
        Domain::Enumeration { alternatives, .. } => match forms {
            [Form::Symbol { name, .. }] if alternatives.contains(name) => {
                Ok(Value::Enum(name.clone()))
            }
            _ => outside(format!("one of {}", alternatives.join(", "))),
        },
        Domain::Set(alternatives) => {
            if forms.is_empty() {
                return Err(NotOfDomain {
                    kind: NotOfDomainKind::Empty,
                    detail: "a set written with no member".to_string(),
                });
            }
            let mut members = BTreeSet::new();
            for form in forms {
                match form {
                    Form::Symbol { name, .. } if alternatives.contains(name) => {
                        if !members.insert(name.clone()) {
                            return outside(format!("`{name}` written twice in one set"));
                        }
                    }
                    _ => return outside(format!("members of {}", alternatives.join(", "))),
                }
            }
            Ok(Value::Set(members))
        }
    }
}
