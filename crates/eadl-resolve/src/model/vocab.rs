//! The vocabulary `/1`: one entry per fact, transcribed from the record's §1.1 table.
//!
//! ⛔ **This table is the model's own copy, held by hand, on purpose.** The production relation (`M3.1.2`) reads
//! `docs/semantics/vocabulary/vocabulary.eadl`; the model does not, so a fault in that reading cannot also be the
//! model's. A row here changes only with the record's §1.1 row it transcribes.

use eadl_model::Dimension;

/// A fact's value kind (record §2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Domain {
    /// `true` or `false`; a bare offer is `true`.
    Boolean,
    /// A non-negative integer, `(pow2 N)` for `N` in 0..=126 included.
    Count,
    /// A number and a unit of one dimension.
    Quantity(Dimension),
    /// Two quantities of one dimension, `lo ≤ hi`, or a point.
    Interval(Dimension),
    /// One of the alternatives; `ordered` when the entry declares a total order, lowest first.
    Enumeration {
        /// The alternatives.
        alternatives: &'static [&'static str],
        /// Whether `at-least` and `at-most` read the alternatives' order.
        ordered: bool,
    },
    /// A non-empty set of the alternatives, without repetition.
    Set(&'static [&'static str]),
    /// A boolean head whose sub-facts are facts in their own right.
    Group(&'static [&'static str]),
}

/// Who a fact speaks for (record §1.1, §3 rules 4 and 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// A promise the offering provider makes.
    Guarantee,
    /// The requiring side's word about itself; it constrains no provider.
    Statement,
}

/// How an offered value satisfies a required one (record §2's table).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// A larger value satisfies.
    AtLeast,
    /// A smaller value satisfies.
    AtMost,
    /// Only an equal value satisfies; written `exactly`.
    Exact,
    /// The offered set contains every required member.
    Includes,
    /// The required point or interval lies inside the offered interval.
    Within,
}

/// The engine rule a derived fact names (record §4).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    /// `(modulus − 1) / rate`, when `wrap-behavior` is `modular` or undescribed.
    HorizonFromModulusAndRate,
}

/// One fact of `/1`.
#[derive(Debug, Clone, Copy)]
pub struct Entry {
    /// Its name.
    pub name: &'static str,
    /// Its value kind.
    pub domain: Domain,
    /// Whom it speaks for.
    pub role: Role,
    /// Its direction.
    pub direction: Direction,
    /// The facts its rule requires.
    pub derived_from: &'static [&'static str],
    /// The facts its rule reads beside those, optionally.
    pub reads: &'static [&'static str],
    /// The rule that computes it.
    pub rule: Option<Rule>,
    /// For a set fact: members every requirement on it holds beside what it writes — the entry's `implies` clause
    /// (record §1.1, R17 2): `available-in-state` implies `run`, the state every use runs in.
    pub implies: &'static [&'static str],
}

const fn plain(name: &'static str, domain: Domain, direction: Direction) -> Entry {
    Entry {
        name,
        domain,
        role: Role::Guarantee,
        direction,
        derived_from: &[],
        reads: &[],
        rule: None,
        implies: &[],
    }
}

const fn boolean(name: &'static str) -> Entry {
    plain(name, Domain::Boolean, Direction::Exact)
}

/// The entries of `/1`, in the order of the record's §1.1 table.
pub const VOCABULARY: &[Entry] = &[
    plain(
        "absolute-deadline",
        Domain::Group(&["supported-horizon", "delivery-bound"]),
        Direction::Exact,
    ),
    boolean("application-mutexes"),
    boolean("general-ipc"),
    boolean("descriptor-format"),
    Entry {
        name: "available-in-state",
        domain: Domain::Set(&["run", "idle", "sleep"]),
        role: Role::Guarantee,
        direction: Direction::Includes,
        derived_from: &[],
        reads: &[],
        rule: None,
        implies: &["run"],
    },
    boolean("bounded-arrival"),
    plain(
        "bus-width",
        Domain::Quantity(Dimension::Information),
        Direction::Exact,
    ),
    plain(
        "clock-rate",
        Domain::Quantity(Dimension::Frequency),
        Direction::Exact,
    ),
    plain("core-count", Domain::Count, Direction::Exact),
    plain("counter-modulus", Domain::Count, Direction::Exact),
    plain(
        "counter-width",
        Domain::Quantity(Dimension::Information),
        Direction::AtLeast,
    ),
    plain(
        "deadlines",
        Domain::Enumeration {
            alternatives: &["constrained", "arbitrary"],
            ordered: false,
        },
        Direction::Exact,
    ),
    boolean("debug-port"),
    boolean("interrupt-source"),
    boolean("low-power-timer"),
    boolean("multicore"),
    boolean("observable-output"),
    boolean("observation-coherent"),
    boolean("relative-delay"),
    boolean("transfer-engine"),
    boolean("uart"),
    plain(
        "delivery-bound",
        Domain::Quantity(Dimension::Time),
        Direction::AtMost,
    ),
    plain(
        "frequency",
        Domain::Interval(Dimension::Frequency),
        Direction::Within,
    ),
    plain(
        "min-arrival-separation",
        Domain::Quantity(Dimension::Time),
        Direction::AtLeast,
    ),
    Entry {
        name: "or-through-mediation",
        domain: Domain::Enumeration {
            alternatives: &["allowed", "forbidden"],
            ordered: false,
        },
        role: Role::Statement,
        direction: Direction::Exact,
        derived_from: &[],
        reads: &[],
        rule: None,
        implies: &[],
    },
    boolean("preemptive"),
    plain(
        "priorities",
        Domain::Set(&["static", "unique", "dynamic"]),
        Direction::Exact,
    ),
    plain("queue-capacity", Domain::Count, Direction::AtLeast),
    plain(
        "reachable-at-privilege",
        Domain::Set(&["user", "supervisor", "machine"]),
        Direction::Includes,
    ),
    plain(
        "release-accuracy",
        Domain::Quantity(Dimension::Time),
        Direction::AtMost,
    ),
    plain(
        "supported-horizon",
        Domain::Quantity(Dimension::Time),
        Direction::AtLeast,
    ),
    plain(
        "tick-rate",
        Domain::Quantity(Dimension::Frequency),
        Direction::Exact,
    ),
    plain(
        "tick-unit",
        Domain::Enumeration {
            alternatives: &["ns", "us", "ms"],
            ordered: false,
        },
        Direction::Exact,
    ),
    Entry {
        name: "unambiguous-horizon",
        domain: Domain::Quantity(Dimension::Time),
        role: Role::Guarantee,
        direction: Direction::AtLeast,
        derived_from: &["counter-modulus", "tick-rate"],
        reads: &["wrap-behavior"],
        rule: Some(Rule::HorizonFromModulusAndRate),
        implies: &[],
    },
    plain(
        "wrap-behavior",
        Domain::Enumeration {
            alternatives: &["modular", "saturating"],
            ordered: false,
        },
        Direction::Exact,
    ),
];

/// The entry named `name`, if `/1` declares one.
#[must_use]
pub fn entry(name: &str) -> Option<&'static Entry> {
    VOCABULARY.iter().find(|e| e.name == name)
}

#[cfg(test)]
mod tests {
    use super::{entry, VOCABULARY};

    #[test]
    fn the_table_holds_thirty_five_entries_named_once() {
        assert_eq!(VOCABULARY.len(), 35);
        for e in VOCABULARY {
            assert_eq!(
                VOCABULARY.iter().filter(|o| o.name == e.name).count(),
                1,
                "{}",
                e.name
            );
            assert!(entry(e.name).is_some());
        }
    }
}
