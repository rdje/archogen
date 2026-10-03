//! Dimensioned quantities: units, conversion, and comparison.
//!
//! `ROADMAP.md` §5.2 sets the contract this module implements:
//!
//! > For parameter-only contracts, implement explicit matching rules in a decidable fragment:
//! > enumerations, booleans, bounded integers, **rational quantities with units**, intervals,
//! > finite sets, and restricted arithmetic. … **Each parameter has a documented comparison
//! > direction and any required normalization.**
//!
//! And §13.1's fixture **F03**: *"Zero clock frequency or incompatible units → type/constraint
//! error **before arithmetic**."*
//!
//! "Before arithmetic" is a structural property here, not a matter of ordering statements
//! carefully. A [`Quantity`] can only be compared or combined through methods that check
//! dimension first and return [`QuantityError::IncompatibleDimensions`] without touching the
//! values. There is no arithmetic on the type that skips the check, so no call site can forget.
//!
//! # Why more bits is not better
//!
//! §5.2 warns: *"More bits or a faster clock is not universally better."* This module supplies
//! the vocabulary for saying which direction is better **per parameter** — a wrap interval where
//! longer is better, a delivery bound where shorter is better, a tick unit that must match
//! exactly. A comparison with no declared direction is a bug waiting for a substitution to
//! expose it, so [`ComparisonDirection`] has no default.

use eadl_front::{Diagnostic, Form, Label, Span};

use crate::rational::Rational;

/// What a quantity measures. Conversion is only ever within one dimension.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Dimension {
    /// Durations and instants. Base unit: second.
    Time,
    /// Rates. Base unit: hertz.
    Frequency,
    /// Amounts of storage. Base unit: bit.
    Information,
    /// A pure number with a unit name, such as a count of ticks.
    Dimensionless,
}

impl Dimension {
    /// The stable machine-readable name.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Time => "time",
            Self::Frequency => "frequency",
            Self::Information => "information",
            Self::Dimensionless => "dimensionless",
        }
    }

    /// The base unit's spelling.
    #[must_use]
    pub const fn base_unit(self) -> &'static str {
        match self {
            Self::Time => "s",
            Self::Frequency => "Hz",
            Self::Information => "bit",
            Self::Dimensionless => "1",
        }
    }
}

/// A unit, with its dimension and its exact scale to the dimension's base unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unit {
    /// The spelling as written in a description.
    pub name: &'static str,
    /// What it measures.
    pub dimension: Dimension,
    /// Numerator of the scale factor to the base unit.
    scale_numerator: i128,
    /// Denominator of the scale factor to the base unit.
    scale_denominator: i128,
}

impl Unit {
    /// The exact factor converting this unit to its dimension's base unit.
    ///
    /// # Panics
    ///
    /// Never for a unit from [`UNITS`]: every entry has a non-zero denominator, and
    /// `all_scales_are_valid` asserts it.
    #[must_use]
    pub fn scale(self) -> Rational {
        Rational::new(self.scale_numerator, self.scale_denominator)
            .expect("a registered unit has a non-zero scale denominator")
    }
}

/// Every unit a description may write.
///
/// Deliberately small and explicit. A unit table that accepts arbitrary SI prefixes accepts
/// `Ps` and `mHz` too, and a typo that parses is worse than one that does not.
pub const UNITS: &[Unit] = &[
    // Time.
    Unit {
        name: "s",
        dimension: Dimension::Time,
        scale_numerator: 1,
        scale_denominator: 1,
    },
    Unit {
        name: "ms",
        dimension: Dimension::Time,
        scale_numerator: 1,
        scale_denominator: 1_000,
    },
    Unit {
        name: "us",
        dimension: Dimension::Time,
        scale_numerator: 1,
        scale_denominator: 1_000_000,
    },
    Unit {
        name: "ns",
        dimension: Dimension::Time,
        scale_numerator: 1,
        scale_denominator: 1_000_000_000,
    },
    // Frequency.
    Unit {
        name: "Hz",
        dimension: Dimension::Frequency,
        scale_numerator: 1,
        scale_denominator: 1,
    },
    Unit {
        name: "kHz",
        dimension: Dimension::Frequency,
        scale_numerator: 1_000,
        scale_denominator: 1,
    },
    Unit {
        name: "MHz",
        dimension: Dimension::Frequency,
        scale_numerator: 1_000_000,
        scale_denominator: 1,
    },
    Unit {
        name: "GHz",
        dimension: Dimension::Frequency,
        scale_numerator: 1_000_000_000,
        scale_denominator: 1,
    },
    // Information. Binary prefixes only: a KiB is 1024 bytes and a "KB" is an argument.
    Unit {
        name: "bit",
        dimension: Dimension::Information,
        scale_numerator: 1,
        scale_denominator: 1,
    },
    Unit {
        name: "byte",
        dimension: Dimension::Information,
        scale_numerator: 8,
        scale_denominator: 1,
    },
    Unit {
        name: "KiB",
        dimension: Dimension::Information,
        scale_numerator: 8 * 1024,
        scale_denominator: 1,
    },
    Unit {
        name: "MiB",
        dimension: Dimension::Information,
        scale_numerator: 8 * 1024 * 1024,
        scale_denominator: 1,
    },
    // Dimensionless.
    Unit {
        name: "tick",
        dimension: Dimension::Dimensionless,
        scale_numerator: 1,
        scale_denominator: 1,
    },
];

/// Look up a unit by its spelling.
#[must_use]
pub fn unit(name: &str) -> Option<Unit> {
    UNITS.iter().copied().find(|item| item.name == name)
}

/// Units of one dimension, for a diagnostic that lists the alternatives.
#[must_use]
pub fn units_of(dimension: Dimension) -> Vec<&'static str> {
    UNITS
        .iter()
        .filter(|item| item.dimension == dimension)
        .map(|item| item.name)
        .collect()
}

/// Which direction of a parameter is "at least as good" (§5.2).
///
/// There is no default. §5.2 warns that "more bits or a faster clock is not universally
/// better", and a comparison with no declared direction is a bug waiting for a substitution to
/// expose it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ComparisonDirection {
    /// A larger offered value satisfies a requirement — an unambiguous-time horizon.
    AtLeast,
    /// A smaller offered value satisfies a requirement — a delivery bound.
    AtMost,
    /// Only an equal value satisfies it — a tick unit, a counter modulus.
    Exact,
}

impl ComparisonDirection {
    /// Whether `offered` satisfies `required` in this direction.
    ///
    /// # Errors
    ///
    /// Returns [`QuantityError::IncompatibleDimensions`] when the two measure different things,
    /// **before** comparing any value.
    pub fn satisfied_by(
        self,
        offered: Quantity,
        required: Quantity,
    ) -> Result<bool, QuantityError> {
        let ordering = offered.compare(required)?;
        Ok(match self {
            Self::AtLeast => ordering.is_ge(),
            Self::AtMost => ordering.is_le(),
            Self::Exact => ordering.is_eq(),
        })
    }

    /// The stable machine-readable name.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::AtLeast => "at-least",
            Self::AtMost => "at-most",
            Self::Exact => "exact",
        }
    }
}

/// A value with a unit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Quantity {
    /// The magnitude, exact.
    pub value: Rational,
    /// Its unit.
    pub unit: Unit,
}

/// Why a quantity could not be built, converted or compared.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum QuantityError {
    /// Two quantities measure different things.
    IncompatibleDimensions {
        /// The left-hand dimension.
        left: Dimension,
        /// The right-hand dimension.
        right: Dimension,
        /// The left-hand unit's spelling.
        left_unit: &'static str,
        /// The right-hand unit's spelling.
        right_unit: &'static str,
    },
    /// The unit is not in the table.
    UnknownUnit {
        /// What was written.
        written: String,
    },
    /// A frequency must be strictly positive (§6.2: "specify positive clock frequency").
    NonPositiveFrequency,
    /// A duration may not be negative.
    NegativeDuration,
    /// Exact arithmetic overflowed. §7.4 asks for detection, so this is a value, not a panic.
    Overflow,
}

impl core::fmt::Display for QuantityError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::IncompatibleDimensions {
                left,
                right,
                left_unit,
                right_unit,
            } => write!(
                f,
                "`{left_unit}` measures {} and `{right_unit}` measures {} — they cannot be compared",
                left.slug(),
                right.slug()
            ),
            Self::UnknownUnit { written } => write!(f, "`{written}` is not a known unit"),
            Self::NonPositiveFrequency => {
                write!(f, "a frequency must be strictly positive")
            }
            Self::NegativeDuration => write!(f, "a duration cannot be negative"),
            Self::Overflow => write!(f, "exact arithmetic overflowed"),
        }
    }
}

impl Quantity {
    /// A quantity, with the domain constraints applied.
    ///
    /// # Errors
    ///
    /// Refuses a non-positive frequency (§6.2) and a negative duration. Both are **constraint**
    /// errors raised at construction, so a zero clock never reaches a division.
    pub fn new(value: Rational, unit: Unit) -> Result<Self, QuantityError> {
        match unit.dimension {
            Dimension::Frequency if !value.is_positive() => {
                Err(QuantityError::NonPositiveFrequency)
            }
            Dimension::Time if value.is_negative() => Err(QuantityError::NegativeDuration),
            _ => Ok(Self { value, unit }),
        }
    }

    /// The magnitude in the dimension's base unit.
    ///
    /// # Errors
    ///
    /// [`QuantityError::Overflow`] when the exact product does not fit.
    pub fn in_base(self) -> Result<Rational, QuantityError> {
        self.value
            .checked_mul(self.unit.scale())
            .ok_or(QuantityError::Overflow)
    }

    /// Convert to another unit of the same dimension.
    ///
    /// # Errors
    ///
    /// [`QuantityError::IncompatibleDimensions`] when the target measures something else —
    /// checked **before** any arithmetic — or [`QuantityError::Overflow`].
    pub fn convert_to(self, target: Unit) -> Result<Self, QuantityError> {
        if self.unit.dimension != target.dimension {
            return Err(self.mismatch(target));
        }
        if self.unit == target {
            return Ok(self);
        }
        let base = self.in_base()?;
        let value = base
            .checked_div(target.scale())
            .ok_or(QuantityError::Overflow)?;
        Ok(Self {
            value,
            unit: target,
        })
    }

    /// Compare with another quantity of the same dimension.
    ///
    /// # Errors
    ///
    /// [`QuantityError::IncompatibleDimensions`] — raised **before** either value is read, which
    /// is F03's "type/constraint error before arithmetic" — or [`QuantityError::Overflow`].
    pub fn compare(self, other: Self) -> Result<core::cmp::Ordering, QuantityError> {
        if self.unit.dimension != other.unit.dimension {
            return Err(self.mismatch(other.unit));
        }
        Ok(self.in_base()?.cmp(&other.in_base()?))
    }

    /// Whether two quantities denote the same amount, whatever their units.
    ///
    /// # Errors
    ///
    /// As [`Quantity::compare`].
    pub fn equals(self, other: Self) -> Result<bool, QuantityError> {
        Ok(self.compare(other)?.is_eq())
    }

    fn mismatch(self, other: Unit) -> QuantityError {
        QuantityError::IncompatibleDimensions {
            left: self.unit.dimension,
            right: other.dimension,
            left_unit: self.unit.name,
            right_unit: other.name,
        }
    }

    /// Read `<number> <unit>` from two adjacent forms, written inside the form whose span is `within`.
    ///
    /// ⛔ **`within` is required, and it is the label when nothing is written.** A missing magnitude has no form
    /// of its own to point at, and this function used to build its label from `SourceId(0)`, which is whichever
    /// file the source map took first — a shipped kind module — so `(tick-rate (exactly))` was refused with a
    /// label on `docs/semantics/kinds/core.eadl:1:1` (leaf `M1.41`, found by the substitutability record's review
    /// R14). Taking the enclosing form's span as an argument leaves no caller able to produce that label.
    ///
    /// # Errors
    ///
    /// A diagnostic with the offending span: a missing or non-numeric magnitude, a missing or
    /// unknown unit, or a violated domain constraint.
    pub fn read(
        number: Option<&Form>,
        unit_form: Option<&Form>,
        within: Span,
    ) -> Result<Self, Box<Diagnostic>> {
        let (value, number_span) = match number {
            Some(Form::Integer { value, span }) => (Rational::integer(*value), *span),
            Some(Form::Decimal { value, scale, span }) => (
                Rational::decimal(*value, *scale).ok_or_else(|| {
                    Box::new(Diagnostic::error(
                        "quantity-overflow",
                        "this decimal is too precise to represent exactly",
                        Label::new(*span, "too many fractional digits"),
                        "reduce the precision, or change the unit so fewer digits are needed",
                    ))
                })?,
                *span,
            ),
            Some(other) => {
                return Err(Box::new(Diagnostic::error(
                    "quantity-not-a-number",
                    format!("expected a number, found a {}", other.kind()),
                    Label::new(other.span(), "not a magnitude"),
                    "write a quantity as a number followed by a unit, e.g. `10 ms`",
                )));
            }
            None => {
                return Err(Box::new(Diagnostic::error(
                    "quantity-missing",
                    "expected a quantity",
                    Label::new(within, "no quantity is written here"),
                    "write a quantity as a number followed by a unit, e.g. `10 ms`",
                )));
            }
        };

        let Some(unit_form) = unit_form else {
            return Err(Box::new(Diagnostic::error(
                "quantity-missing-unit",
                "this number has no unit",
                Label::new(number_span, "a bare number is not a quantity"),
                "write the unit after the number, e.g. `10 ms` — a bare number cannot be \
                 compared with anything, because nothing says what it measures",
            )));
        };

        let Some(name) = unit_form.as_symbol() else {
            return Err(Box::new(Diagnostic::error(
                "quantity-missing-unit",
                format!("expected a unit, found a {}", unit_form.kind()),
                Label::new(unit_form.span(), "not a unit"),
                format!("the known units are {}", known_units()),
            )));
        };

        let Some(unit) = unit(name) else {
            return Err(Box::new(Diagnostic::error(
                "quantity-unknown-unit",
                format!("`{name}` is not a known unit"),
                Label::new(unit_form.span(), "unknown unit"),
                format!("the known units are {}", known_units()),
            )));
        };

        Self::new(value, unit).map_err(|error| {
            let span = number_span.merge(unit_form.span());
            Box::new(match error {
                QuantityError::NonPositiveFrequency => Diagnostic::error(
                    "quantity-non-positive-frequency",
                    "a frequency must be strictly positive",
                    Label::new(span, "this frequency is not positive"),
                    "§6.2 requires a positive clock frequency: every conversion from ticks to \
                     time divides by it, so zero makes the platform's time contract undefined \
                     rather than merely wrong",
                ),
                QuantityError::NegativeDuration => Diagnostic::error(
                    "quantity-negative-duration",
                    "a duration cannot be negative",
                    Label::new(span, "negative duration"),
                    "write a non-negative duration; an interval's direction belongs in the \
                     clause that uses it, not in the sign of its magnitude",
                ),
                other => Diagnostic::error(
                    "quantity-invalid",
                    other.to_string(),
                    Label::new(span, "invalid quantity"),
                    "write a quantity as a number followed by a known unit",
                ),
            })
        })
    }
}

fn known_units() -> String {
    UNITS
        .iter()
        .map(|item| format!("`{}`", item.name))
        .collect::<Vec<_>>()
        .join(", ")
}

impl core::fmt::Display for Quantity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{} {}", self.value, self.unit.name)
    }
}
