//! Exact rational arithmetic.
//!
//! `ROADMAP.md` §7.4 is not ambiguous about this:
//!
//! > Use **exact integer time units or checked rational arithmetic**, upward rounding where
//! > needed, **overflow detection**, and explicit convergence/deadline limits.
//!
//! Two properties follow, and both are enforced by the type rather than by discipline:
//!
//! * **Exact.** `1/3` is `1/3`, not `0.333…`. A response-time recurrence that accumulates
//!   rounding error produces a bound that is neither an upper bound nor a measurement, and the
//!   error is invisible because the answer still looks like a number.
//! * **Checked.** Every operation returns an [`Option`], and overflow is `None`. A silently
//!   wrapped numerator turns a schedulable system into an unschedulable one, or worse, the
//!   reverse. §7.4 asks for detection, so there is no unchecked arithmetic to reach for.
//!
//! The representation is `i128 / i128`, always normalized: `gcd` reduced, denominator strictly
//! positive, and zero canonically `0/1`. Normalization on construction is what makes equality
//! and ordering structural rather than a computation that can be got wrong at a call site.

/// An exact rational number.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    /// The numerator, carrying the sign.
    numerator: i128,
    /// The denominator, always strictly positive.
    denominator: i128,
}

impl Rational {
    /// Zero.
    pub const ZERO: Self = Self {
        numerator: 0,
        denominator: 1,
    };
    /// One.
    pub const ONE: Self = Self {
        numerator: 1,
        denominator: 1,
    };

    /// A whole number.
    #[must_use]
    pub const fn integer(value: i64) -> Self {
        Self {
            numerator: value as i128,
            denominator: 1,
        }
    }

    /// `numerator / denominator`, normalized.
    ///
    /// Returns `None` for a zero denominator. A rational with no value is not a value, and
    /// returning one would push the failure to whoever eventually divides by it.
    #[must_use]
    pub fn new(numerator: i128, denominator: i128) -> Option<Self> {
        if denominator == 0 {
            return None;
        }
        let sign = if denominator < 0 { -1 } else { 1 };
        let numerator = numerator.checked_mul(sign)?;
        let denominator = denominator.checked_mul(sign)?;
        let divisor = gcd(numerator.unsigned_abs(), denominator.unsigned_abs());
        let divisor = i128::try_from(divisor).ok()?.max(1);
        Some(Self {
            numerator: numerator / divisor,
            denominator: denominator / divisor,
        })
    }

    /// `value / 10^scale`, the shape the reader produces for a decimal literal.
    #[must_use]
    pub fn decimal(value: i64, scale: u32) -> Option<Self> {
        let mut denominator: i128 = 1;
        for _ in 0..scale {
            denominator = denominator.checked_mul(10)?;
        }
        Self::new(i128::from(value), denominator)
    }

    /// The numerator.
    #[must_use]
    pub const fn numerator(self) -> i128 {
        self.numerator
    }

    /// The denominator, always strictly positive.
    #[must_use]
    pub const fn denominator(self) -> i128 {
        self.denominator
    }

    /// Whether this is exactly zero.
    #[must_use]
    pub const fn is_zero(self) -> bool {
        self.numerator == 0
    }

    /// Whether this is strictly greater than zero.
    #[must_use]
    pub const fn is_positive(self) -> bool {
        self.numerator > 0
    }

    /// Whether this is strictly less than zero.
    #[must_use]
    pub const fn is_negative(self) -> bool {
        self.numerator < 0
    }

    /// Whether this is a whole number.
    #[must_use]
    pub const fn is_integer(self) -> bool {
        self.denominator == 1
    }

    /// Checked addition.
    #[must_use]
    pub fn checked_add(self, other: Self) -> Option<Self> {
        let left = self.numerator.checked_mul(other.denominator)?;
        let right = other.numerator.checked_mul(self.denominator)?;
        Self::new(
            left.checked_add(right)?,
            self.denominator.checked_mul(other.denominator)?,
        )
    }

    /// Checked subtraction.
    #[must_use]
    pub fn checked_sub(self, other: Self) -> Option<Self> {
        self.checked_add(other.negate()?)
    }

    /// Checked multiplication.
    #[must_use]
    pub fn checked_mul(self, other: Self) -> Option<Self> {
        Self::new(
            self.numerator.checked_mul(other.numerator)?,
            self.denominator.checked_mul(other.denominator)?,
        )
    }

    /// Checked division. `None` when dividing by zero.
    #[must_use]
    pub fn checked_div(self, other: Self) -> Option<Self> {
        if other.is_zero() {
            return None;
        }
        Self::new(
            self.numerator.checked_mul(other.denominator)?,
            self.denominator.checked_mul(other.numerator)?,
        )
    }

    /// Negation, checked because `i128::MIN` has no positive counterpart.
    #[must_use]
    pub fn negate(self) -> Option<Self> {
        Some(Self {
            numerator: self.numerator.checked_neg()?,
            denominator: self.denominator,
        })
    }

    /// The smallest integer greater than or equal to this value.
    ///
    /// The rounding direction §7.4 calls for: response-time analysis needs `⌈R/T⌉`, and
    /// rounding the other way understates interference, which turns a missed deadline into a
    /// reported pass.
    #[must_use]
    pub fn ceil(self) -> Option<i128> {
        let quotient = self.numerator.checked_div(self.denominator)?;
        let remainder = self.numerator % self.denominator;
        if remainder > 0 {
            quotient.checked_add(1)
        } else {
            Some(quotient)
        }
    }

    /// The largest integer less than or equal to this value.
    #[must_use]
    pub fn floor(self) -> Option<i128> {
        let quotient = self.numerator.checked_div(self.denominator)?;
        let remainder = self.numerator % self.denominator;
        if remainder < 0 {
            quotient.checked_sub(1)
        } else {
            Some(quotient)
        }
    }

    /// Exact decimal text when the denominator divides a power of ten, otherwise `n/d`.
    ///
    /// Never an approximation: a value that cannot be written exactly in decimal is written as
    /// a fraction rather than rounded into something that reads like a measurement.
    #[must_use]
    pub fn to_exact_string(self) -> String {
        if self.is_integer() {
            return self.numerator.to_string();
        }
        let mut denominator = self.denominator;
        let mut scale = 0_u32;
        while denominator % 2 == 0 {
            denominator /= 2;
            scale += 1;
        }
        let mut fives = 0_u32;
        while denominator % 5 == 0 {
            denominator /= 5;
            fives += 1;
        }
        if denominator != 1 {
            return format!("{}/{}", self.numerator, self.denominator);
        }
        let scale = scale.max(fives);
        let mut multiplier: i128 = 1;
        for _ in 0..scale {
            multiplier *= 10;
        }
        let Some(scaled) = self.numerator.checked_mul(multiplier / self.denominator) else {
            return format!("{}/{}", self.numerator, self.denominator);
        };
        let negative = scaled < 0;
        let digits = scaled.unsigned_abs().to_string();
        let scale = scale as usize;
        let padded = if digits.len() <= scale {
            format!("{}{}", "0".repeat(scale - digits.len() + 1), digits)
        } else {
            digits
        };
        let split = padded.len() - scale;
        format!(
            "{}{}.{}",
            if negative { "-" } else { "" },
            &padded[..split],
            &padded[split..]
        )
    }
}

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Rational {
    /// Compares by cross-multiplication, on `i128` widened from normalized values.
    ///
    /// Both denominators are positive after normalization, so the cross products keep their
    /// sign and the comparison needs no case analysis. A saturating multiply is used rather
    /// than a checked one: ordering must be total to satisfy `Ord`, and saturation preserves
    /// the sign of the comparison at the extremes where an exact product would overflow.
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        let left = self.numerator.saturating_mul(other.denominator);
        let right = other.numerator.saturating_mul(self.denominator);
        left.cmp(&right)
    }
}

impl core::fmt::Display for Rational {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}", self.to_exact_string())
    }
}

/// Greatest common divisor, Euclid.
const fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

#[cfg(test)]
mod tests {
    use super::Rational;

    #[test]
    fn construction_normalizes_sign_and_divisor() {
        let a = Rational::new(6, 8).expect("valid");
        assert_eq!((a.numerator(), a.denominator()), (3, 4));
        let b = Rational::new(3, -4).expect("valid");
        assert_eq!((b.numerator(), b.denominator()), (-3, 4));
        let zero = Rational::new(0, 7).expect("valid");
        assert_eq!((zero.numerator(), zero.denominator()), (0, 1));
    }

    #[test]
    fn equality_is_structural_because_construction_normalizes() {
        assert_eq!(
            Rational::new(1, 2).unwrap(),
            Rational::new(50, 100).unwrap()
        );
        assert_eq!(Rational::new(-2, 4).unwrap(), Rational::new(2, -4).unwrap());
    }

    #[test]
    fn a_zero_denominator_is_not_a_number() {
        assert!(Rational::new(1, 0).is_none());
        assert!(Rational::ONE.checked_div(Rational::ZERO).is_none());
    }

    #[test]
    fn thirds_stay_exact_through_arithmetic() {
        // ⭐ The reason for the whole module. In binary floating point this sum is not 1.
        let third = Rational::new(1, 3).unwrap();
        let sum = third
            .checked_add(third)
            .and_then(|s| s.checked_add(third))
            .unwrap();
        assert_eq!(sum, Rational::ONE);
    }

    #[test]
    fn a_decimal_literal_is_exact() {
        // 0.1 is the number binary floating point cannot represent.
        let tenth = Rational::decimal(1, 1).unwrap();
        assert_eq!(tenth, Rational::new(1, 10).unwrap());
        let ten_tenths = (0..10).fold(Rational::ZERO, |acc, _| acc.checked_add(tenth).unwrap());
        assert_eq!(ten_tenths, Rational::ONE);
    }

    #[test]
    fn overflow_is_detected_rather_than_wrapped() {
        // §7.4 asks for overflow detection. A wrapped numerator turns an unschedulable system
        // into a schedulable-looking one, and nothing in the output says so.
        let huge = Rational::new(i128::MAX, 1).unwrap();
        assert!(huge.checked_add(Rational::ONE).is_none());
        assert!(huge.checked_mul(huge).is_none());
        let min = Rational::new(i128::MIN, 1).unwrap();
        assert!(min.negate().is_none());
        assert!(min.checked_sub(Rational::ONE).is_none());
    }

    #[test]
    fn ceil_rounds_upward_which_is_the_direction_analysis_needs() {
        // Rounding the other way understates interference, which turns a missed deadline into
        // a reported pass.
        assert_eq!(Rational::new(7, 2).unwrap().ceil(), Some(4));
        assert_eq!(Rational::new(8, 2).unwrap().ceil(), Some(4));
        assert_eq!(Rational::new(-7, 2).unwrap().ceil(), Some(-3));
        assert_eq!(Rational::new(1, 1000).unwrap().ceil(), Some(1));
    }

    #[test]
    fn floor_rounds_downward() {
        assert_eq!(Rational::new(7, 2).unwrap().floor(), Some(3));
        assert_eq!(Rational::new(-7, 2).unwrap().floor(), Some(-4));
        assert_eq!(Rational::new(8, 2).unwrap().floor(), Some(4));
    }

    #[test]
    fn ordering_is_exact_and_total() {
        let a = Rational::new(1, 3).unwrap();
        let b = Rational::new(1, 2).unwrap();
        assert!(a < b);
        assert!(b > a);
        assert!(Rational::new(-1, 3).unwrap() < Rational::ZERO);
        let mut values = [b, a, Rational::ZERO];
        values.sort();
        assert_eq!(values, [Rational::ZERO, a, b]);
    }

    #[test]
    fn exact_text_never_approximates() {
        assert_eq!(Rational::new(1, 2).unwrap().to_exact_string(), "0.5");
        assert_eq!(Rational::new(1, 8).unwrap().to_exact_string(), "0.125");
        assert_eq!(Rational::new(3, 1).unwrap().to_exact_string(), "3");
        assert_eq!(Rational::new(-1, 4).unwrap().to_exact_string(), "-0.25");
        // A third has no exact decimal, so it is written as a fraction rather than rounded
        // into something that reads like a measurement.
        assert_eq!(Rational::new(1, 3).unwrap().to_exact_string(), "1/3");
        assert_eq!(Rational::new(22, 7).unwrap().to_exact_string(), "22/7");
    }

    #[test]
    fn predicates_agree_with_the_value() {
        assert!(Rational::ZERO.is_zero());
        assert!(!Rational::ZERO.is_positive());
        assert!(!Rational::ZERO.is_negative());
        assert!(Rational::ONE.is_positive());
        assert!(Rational::new(-1, 5).unwrap().is_negative());
        assert!(Rational::integer(4).is_integer());
        assert!(!Rational::new(1, 4).unwrap().is_integer());
    }
}
