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

/// The function a fact describes (record §1.1's Function column; `M3.3.6`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Function {
    /// One device's operations only valid together, named.
    Of(&'static str),
    /// `each`: a qualifier of every function its requirer relies on.
    Each,
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
    /// Whether a requester bound to it takes one of the provider's `channels` — the entry's `use consumed`, on a
    /// boolean fact or a group's head (`M3.3`) — rather than reading it, the default.
    pub consumed: bool,
    /// The function it describes — the entry's `function` clause; a statement names none (`M3.3.6`).
    pub function: Option<Function>,
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
        consumed: false,
        function: None,
    }
}

const fn boolean(name: &'static str) -> Entry {
    plain(name, Domain::Boolean, Direction::Exact)
}

/// `entry`, marked `use consumed`.
const fn consumed(entry: Entry) -> Entry {
    Entry {
        consumed: true,
        ..entry
    }
}

/// `entry`, describing the function `name`.
const fn of(name: &'static str, entry: Entry) -> Entry {
    Entry {
        function: Some(Function::Of(name)),
        ..entry
    }
}

/// `entry`, a qualifier: `each`.
const fn each(entry: Entry) -> Entry {
    Entry {
        function: Some(Function::Each),
        ..entry
    }
}

/// The entries of `/1`, in the order of the record's §1.1 table.
pub const VOCABULARY: &[Entry] = &[
    of(
        "counter",
        consumed(plain(
            "absolute-deadline",
            Domain::Group(&["supported-horizon", "delivery-bound"]),
            Direction::Exact,
        )),
    ),
    of("mutex", boolean("application-mutexes")),
    of("queue", boolean("general-ipc")),
    of("transfer", boolean("descriptor-format")),
    Entry {
        name: "available-in-state",
        domain: Domain::Set(&["run", "idle", "sleep"]),
        role: Role::Guarantee,
        direction: Direction::Includes,
        derived_from: &[],
        reads: &[],
        rule: None,
        implies: &["run"],
        consumed: false,
        function: Some(Function::Each),
    },
    of("arrival", boolean("bounded-arrival")),
    of(
        "bus",
        plain(
            "bus-width",
            Domain::Quantity(Dimension::Information),
            Direction::Exact,
        ),
    ),
    of(
        "counter",
        plain("channels", Domain::Count, Direction::AtLeast),
    ),
    of(
        "clock",
        plain(
            "clock-rate",
            Domain::Quantity(Dimension::Frequency),
            Direction::Exact,
        ),
    ),
    of(
        "processor",
        plain("core-count", Domain::Count, Direction::Exact),
    ),
    of(
        "counter",
        plain("counter-modulus", Domain::Count, Direction::Exact),
    ),
    of(
        "counter",
        plain(
            "counter-width",
            Domain::Quantity(Dimension::Information),
            Direction::AtLeast,
        ),
    ),
    of(
        "scheduler",
        plain(
            "deadlines",
            Domain::Enumeration {
                alternatives: &["constrained", "arbitrary"],
                ordered: false,
            },
            Direction::Exact,
        ),
    ),
    of("debug", boolean("debug-port")),
    of("interrupt", boolean("interrupt-source")),
    of("counter", boolean("low-power-timer")),
    of("processor", boolean("multicore")),
    of("output", boolean("observable-output")),
    of("counter", boolean("observation-coherent")),
    of("counter", consumed(boolean("relative-delay"))),
    of("transfer", boolean("transfer-engine")),
    of("output", boolean("uart")),
    of(
        "counter",
        plain(
            "delivery-bound",
            Domain::Quantity(Dimension::Time),
            Direction::AtMost,
        ),
    ),
    of(
        "clock",
        plain(
            "frequency",
            Domain::Interval(Dimension::Frequency),
            Direction::Within,
        ),
    ),
    of(
        "interrupt",
        plain(
            "min-arrival-separation",
            Domain::Quantity(Dimension::Time),
            Direction::AtLeast,
        ),
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
        consumed: false,
        function: None,
    },
    of("scheduler", boolean("preemptive")),
    of(
        "scheduler",
        plain(
            "priorities",
            Domain::Set(&["static", "unique", "dynamic"]),
            Direction::Exact,
        ),
    ),
    of(
        "queue",
        plain("queue-capacity", Domain::Count, Direction::AtLeast),
    ),
    each(plain(
        "reachable-at-privilege",
        Domain::Set(&["user", "supervisor", "machine"]),
        Direction::Includes,
    )),
    of(
        "release",
        plain(
            "release-accuracy",
            Domain::Quantity(Dimension::Time),
            Direction::AtMost,
        ),
    ),
    of(
        "counter",
        plain(
            "supported-horizon",
            Domain::Quantity(Dimension::Time),
            Direction::AtLeast,
        ),
    ),
    of(
        "counter",
        plain(
            "tick-rate",
            Domain::Quantity(Dimension::Frequency),
            Direction::Exact,
        ),
    ),
    of(
        "counter",
        plain(
            "tick-unit",
            Domain::Enumeration {
                alternatives: &["ns", "us", "ms"],
                ordered: false,
            },
            Direction::Exact,
        ),
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
        consumed: false,
        function: Some(Function::Of("counter")),
    },
    of(
        "counter",
        plain(
            "wrap-behavior",
            Domain::Enumeration {
                alternatives: &["modular", "saturating"],
                ordered: false,
            },
            Direction::Exact,
        ),
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
    fn the_table_holds_thirty_six_entries_named_once() {
        // Thirty-five from the census, and `channels`, which `M3.3`'s joint check reads.
        assert_eq!(VOCABULARY.len(), 36);
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
