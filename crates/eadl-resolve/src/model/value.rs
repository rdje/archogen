//! Values in each domain, read from written forms, compared as record §2's table says.

use std::cmp::Ordering;
use std::collections::BTreeSet;

use eadl_front::{Form, Span};
use eadl_model::quantity::{ComparisonDirection, Quantity};
use eadl_model::{Dimension, Rational};

use super::vocab::{Direction, Domain};

/// The largest `N` a `(pow2 N)` count may write (record §2).
pub const POW2_LIMIT: u32 = 126;

/// A value of some domain.
#[derive(Debug, Clone)]
pub enum Value {
    /// A boolean.
    Bool(bool),
    /// A count, `0 ..= 2^126`.
    Count(i128),
    /// A quantity.
    Quantity(Quantity),
    /// An interval, `lo ≤ hi`.
    Interval(Quantity, Quantity),
    /// One alternative of an enumeration.
    Enum(String),
    /// A non-empty set of alternatives.
    Set(BTreeSet<String>),
}

/// Why a written value is refused: always `invalid-description` (record §2, §8).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Malformed(pub String);

fn malformed<T>(why: impl Into<String>) -> Result<T, Malformed> {
    Err(Malformed(why.into()))
}

/// Whether two values are the same in their domain: quantities by amount, `10 MHz` beside `10000 kHz` (record §5,
/// R15 13).
///
/// # Errors
///
/// [`Overflow`] when comparing two amounts overflows the exact arithmetic: §2's `unsupported-profile`, never "two
/// values" (record §5, R17 5).
pub fn same(a: &Value, b: &Value) -> Result<bool, Overflow> {
    let eq = |x: Quantity, y: Quantity| x.equals(y).map_err(|_| Overflow);
    Ok(match (a, b) {
        (Value::Bool(x), Value::Bool(y)) => x == y,
        (Value::Count(x), Value::Count(y)) => x == y,
        (Value::Quantity(x), Value::Quantity(y)) => eq(*x, *y)?,
        (Value::Interval(a0, a1), Value::Interval(b0, b1)) => eq(*a0, *b0)? && eq(*a1, *b1)?,
        (Value::Enum(x), Value::Enum(y)) => x == y,
        (Value::Set(x), Value::Set(y)) => x == y,
        _ => false,
    })
}

/// What §8 refuses of one fact's value wherever it is written — offer, bound or requirement (record §4, §8; R17 6): a
/// value of information that is not a positive whole number of bits, and a `counter-modulus` of 0.
///
/// # Errors
///
/// [`Malformed`], `invalid-description`.
pub fn fact_value(fact: &str, domain: Domain, v: &Value) -> Result<(), Malformed> {
    if let (Domain::Quantity(Dimension::Information), Value::Quantity(q)) = (domain, v) {
        let whole = q.in_base().is_ok_and(|b| b.is_integer() && b.is_positive());
        if !whole {
            return malformed("a value of information that is not a positive whole number of bits");
        }
    }
    if fact == "counter-modulus" && matches!(v, Value::Count(0)) {
        return malformed("a `counter-modulus` of 0");
    }
    Ok(())
}

fn quantity(
    number: Option<&Form>,
    unit: Option<&Form>,
    within: Span,
    dim: Dimension,
) -> Result<Quantity, Malformed> {
    match Quantity::read(number, unit, within) {
        Ok(q) if q.unit.dimension == dim => Ok(q),
        Ok(q) => malformed(format!(
            "a quantity of {} where the fact is {}",
            q.unit.dimension.slug(),
            dim.slug()
        )),
        Err(d) => malformed(format!("{}: {}", d.code, d.message)),
    }
}

fn count(forms: &[Form]) -> Result<i128, Malformed> {
    match forms {
        [Form::Integer { value, .. }] if *value >= 0 => Ok(i128::from(*value)),
        [Form::Integer { value, .. }, Form::Symbol { name, .. }] if *value >= 0 => {
            match eadl_model::quantity::unit(name) {
                Some(u) if u.dimension == Dimension::Dimensionless => Ok(i128::from(*value)),
                _ => malformed(format!("`{name}` is not a dimensionless unit")),
            }
        }
        [pow @ Form::List { .. }] if pow.head() == Some("pow2") => match pow.items() {
            [_, Form::Integer { value, .. }] if (0..=i64::from(POW2_LIMIT)).contains(value) => {
                Ok(1i128 << *value)
            }
            _ => malformed("`(pow2 N)` with `N` an integer from 0 to 126"),
        },
        _ => malformed(
            "a count is a non-negative integer, an optional dimensionless unit, or `(pow2 N)`",
        ),
    }
}

/// Read the value `forms` write for a fact of `domain`, `within` the form that holds them.
///
/// # Errors
///
/// A value outside the domain (record §2, §8).
pub fn read(domain: Domain, forms: &[Form], within: Span) -> Result<Value, Malformed> {
    match domain {
        Domain::Boolean | Domain::Group(_) => match forms {
            [Form::Symbol { name, .. }] if name == "true" => Ok(Value::Bool(true)),
            [Form::Symbol { name, .. }] if name == "false" => Ok(Value::Bool(false)),
            _ => malformed("a boolean is `true` or `false`"),
        },
        Domain::Count => count(forms).map(Value::Count),
        Domain::Quantity(dim) => match forms {
            [n, u] => quantity(Some(n), Some(u), within, dim).map(Value::Quantity),
            _ => malformed("a quantity is a number and a unit"),
        },
        Domain::Interval(dim) => match forms {
            [range @ Form::List { .. }] if range.head() == Some("range") => match range.items() {
                [_, ln, lu, hn, hu] => {
                    let lo = quantity(Some(ln), Some(lu), within, dim)?;
                    let hi = quantity(Some(hn), Some(hu), within, dim)?;
                    match lo.compare(hi) {
                        Ok(Ordering::Greater) => malformed("an interval with `lo > hi`"),
                        Ok(_) => Ok(Value::Interval(lo, hi)),
                        Err(e) => malformed(e.to_string()),
                    }
                }
                _ => malformed("`(range lo hi)`, two quantities"),
            },
            [n, u] => {
                let v = quantity(Some(n), Some(u), within, dim)?;
                Ok(Value::Interval(v, v))
            }
            _ => malformed("an interval is `(range lo hi)` or a point"),
        },
        Domain::Enumeration { alternatives, .. } => match forms {
            [Form::Symbol { name, .. }] if alternatives.contains(&name.as_str()) => {
                Ok(Value::Enum(name.clone()))
            }
            _ => malformed(format!("one of {}", alternatives.join(", "))),
        },
        Domain::Set(alternatives) => {
            if forms.is_empty() {
                return malformed("a set written with no member");
            }
            let mut set = BTreeSet::new();
            for f in forms {
                match f {
                    Form::Symbol { name, .. } if alternatives.contains(&name.as_str()) => {
                        if !set.insert(name.clone()) {
                            return malformed(format!("`{name}` written twice"));
                        }
                    }
                    _ => return malformed(format!("members of {}", alternatives.join(", "))),
                }
            }
            Ok(Value::Set(set))
        }
    }
}

/// Why a comparison gives no answer: §2's `unsupported-profile`, the arithmetic's limit.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Overflow;

/// Whether `offered` lies in `direction` against `required` (record §2's "Satisfied when" column). `implied` holds
/// members a set requirement asks for beside what it writes — `run`, for `available-in-state` (record §1.1, R16 2).
///
/// # Errors
///
/// [`Overflow`] when the exact arithmetic of a quantity comparison overflows.
pub fn satisfies(
    offered: &Value,
    required: &Value,
    direction: Direction,
    order: &[&str],
    implied: &[&str],
) -> Result<bool, Overflow> {
    let q = |o: Quantity, r: Quantity, d: ComparisonDirection| {
        o.compare(r).map_err(|_| Overflow).map(|ord| match d {
            ComparisonDirection::AtLeast => ord.is_ge(),
            ComparisonDirection::AtMost => ord.is_le(),
            ComparisonDirection::Exact => ord.is_eq(),
        })
    };
    Ok(match (offered, required, direction) {
        (Value::Bool(o), Value::Bool(r), _) => o == r,
        (Value::Count(o), Value::Count(r), Direction::AtLeast) => o >= r,
        (Value::Count(o), Value::Count(r), Direction::AtMost) => o <= r,
        (Value::Count(o), Value::Count(r), _) => o == r,
        (Value::Quantity(o), Value::Quantity(r), Direction::AtLeast) => {
            q(*o, *r, ComparisonDirection::AtLeast)?
        }
        (Value::Quantity(o), Value::Quantity(r), Direction::AtMost) => {
            q(*o, *r, ComparisonDirection::AtMost)?
        }
        (Value::Quantity(o), Value::Quantity(r), _) => q(*o, *r, ComparisonDirection::Exact)?,
        (Value::Interval(olo, ohi), Value::Interval(rlo, rhi), Direction::Within) => {
            q(*olo, *rlo, ComparisonDirection::AtMost)?
                && q(*ohi, *rhi, ComparisonDirection::AtLeast)?
        }
        (Value::Interval(olo, ohi), Value::Interval(rlo, rhi), _) => {
            q(*olo, *rlo, ComparisonDirection::Exact)? && q(*ohi, *rhi, ComparisonDirection::Exact)?
        }
        (Value::Enum(o), Value::Enum(r), Direction::AtLeast | Direction::AtMost) => {
            let (Some(oi), Some(ri)) = (
                order.iter().position(|a| a == o),
                order.iter().position(|a| a == r),
            ) else {
                return Ok(false);
            };
            if direction == Direction::AtLeast {
                oi >= ri
            } else {
                oi <= ri
            }
        }
        (Value::Enum(o), Value::Enum(r), _) => o == r,
        (Value::Set(o), Value::Set(r), Direction::Includes) => {
            r.iter().all(|m| o.contains(m)) && implied.iter().all(|m| o.contains(*m))
        }
        (Value::Set(o), Value::Set(r), _) => {
            let mut want = r.clone();
            want.extend(implied.iter().map(|m| (*m).to_string()));
            *o == want
        }
        _ => false,
    })
}

/// `(modulus − 1) / rate` in seconds, exact (record §4's one rule).
///
/// # Errors
///
/// [`Overflow`] when the exact arithmetic overflows `i128` (record §2: `unsupported-profile`).
pub fn horizon(modulus: i128, rate: Quantity) -> Result<Quantity, Overflow> {
    let hz = rate.in_base().map_err(|_| Overflow)?;
    let ticks = Rational::new(modulus - 1, 1).ok_or(Overflow)?;
    let seconds = ticks.checked_div(hz).ok_or(Overflow)?;
    let unit = eadl_model::quantity::unit("s").ok_or(Overflow)?;
    Quantity::new(seconds, unit).map_err(|_| Overflow)
}
