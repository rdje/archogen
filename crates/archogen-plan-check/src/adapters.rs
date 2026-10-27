//! The engine's adapter kinds, `adapters/1`: one entry per kind, transcribed from the table of
//! `docs/specs/adapters/decision_engine-adapters.md` §2 (`M3.2`).
//!
//! ⛔ **This table is the checker's own copy, held by hand, on purpose** (record §1.2). The generator reads
//! `docs/semantics/adapters/adapters.eadl`; the checker does not, and this table was written from the record under
//! `M3.5`'s fence, never from the generator's code, so a fault in reading the file cannot hide behind it — a derivation
//! test holds the two equal kind by kind. A kind here changes only with the record's §2 row it transcribes.
//!
//! A kind declares what it requires of the one provider it programs, [`Beneath`]; what an instance offers over it,
//! [`Offer`]; the level its code runs at; the obligations a plan binding it carries and the interrupts its code takes;
//! and whether it mediates (record §1.3). Values are kept as the record writes them, each token separated by one space,
//! for the relation to read as it reads any provider's offers.
//!
//! **A test catalog** (record §6; leaf `M3.2.6.3`): `/1` holds no mediating kind, and "a test catalog holds one, which
//! owns no region, and holds the gate both ways". [`with_table`] judges against such a table in place of [`ADAPTERS`],
//! on one thread, for one call; every check reads a kind through [`kind`], so the record's reader and checks (3) to
//! (5) all read the table in force. A thread that never calls it reads [`ADAPTERS`] alone.

use std::cell::Cell;

/// The catalog's version, `adapters/1`.
pub const VERSION: &str = "adapters/1";

/// What a kind requires of the one provider it programs: its `beneath` side.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Beneath {
    /// A fact named in a `needs` list.
    Needs(&'static str),
    /// A constraint `(fact value)`, the value as written.
    Constraint {
        /// The fact it constrains, its head.
        fact: &'static str,
        /// The value it requires, as written.
        value: &'static str,
    },
}

impl Beneath {
    /// The fact it names: a `needs` name, or a constraint's head.
    const fn fact(&self) -> &'static str {
        match *self {
            Self::Needs(fact) | Self::Constraint { fact, .. } => fact,
        }
    }
}

/// One item of a kind's `offers`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Offer {
    /// `(fact value)`, the value as written.
    Value {
        /// The fact offered.
        fact: &'static str,
        /// Its value, as written.
        value: &'static str,
    },
    /// `(fact (from other))`: the provider's own value of `other`.
    From {
        /// The fact offered.
        fact: &'static str,
        /// The provider's fact whose value it takes, which may be one the vocabulary derives there.
        from: &'static str,
    },
    /// `(fact (obliged))`.
    Obliged {
        /// The fact offered, at the tightest bound its requester writes on it (record §1.5).
        fact: &'static str,
    },
}

impl Offer {
    /// The fact it offers.
    const fn fact(&self) -> &'static str {
        match *self {
            Self::Value { fact, .. } | Self::From { fact, .. } | Self::Obliged { fact } => fact,
        }
    }
}

/// One adapter kind.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Kind {
    /// Its name, the engine identity a plan writes in `(adapter KIND PROVIDER …)` (record §5).
    pub name: &'static str,
    /// What it requires of the provider it programs: its `needs` names in written order, then its constraints.
    pub beneath: &'static [Beneath],
    /// What an instance offers, in written order, each fact once.
    pub offers: &'static [Offer],
    /// The level of `reachable-at-privilege`'s domain its code runs at, model §8 rule 9's (record §4).
    pub level: &'static str,
    /// Its obligations' names, in the record's order.
    pub obligations: &'static [&'static str],
    /// Its cost interrupts' names, in the record's order.
    pub costs: &'static [&'static str],
    /// Whether it writes `(mediates true)`; absent, `false` (`SR-H8`, record §6).
    pub mediates: bool,
}

impl Kind {
    /// Exclusive when its offers include `channels` (record §1.4).
    #[must_use]
    pub fn exclusive(&self) -> bool {
        self.offers.iter().any(|o| o.fact() == "channels")
    }

    /// Every fact named in its `beneath`, `needs` and constraint heads alike, in written order.
    #[must_use]
    pub fn beneath_names(&self) -> Vec<&'static str> {
        self.beneath.iter().map(Beneath::fact).collect()
    }

    /// What it adds (record §7.1): each offered fact not passed through from itself and not named beneath, in offer
    /// order.
    #[must_use]
    pub fn adds(&self) -> Vec<&'static str> {
        let beneath = self.beneath_names();
        self.offers
            .iter()
            .filter(|o| !matches!(o, Offer::From { fact, from } if fact == from))
            .map(Offer::fact)
            .filter(|fact| !beneath.contains(fact))
            .collect()
    }
}

/// `/1`'s kinds, in the record's order.
pub const ADAPTERS: &[Kind] = &[
    // §5.4's own example: an absolute deadline programmed as the relative delay `T − now`, exclusive.
    Kind {
        name: "deadline-over-delay",
        beneath: &[
            Beneath::Needs("relative-delay"),
            Beneath::Needs("counter-modulus"),
            Beneath::Needs("tick-rate"),
            Beneath::Needs("observation-coherent"),
        ],
        offers: &[
            Offer::Value {
                fact: "absolute-deadline",
                value: "true",
            },
            Offer::From {
                fact: "supported-horizon",
                from: "unambiguous-horizon",
            },
            Offer::Obliged {
                fact: "delivery-bound",
            },
            Offer::Value {
                fact: "channels",
                value: "1 tick",
            },
            Offer::From {
                fact: "available-in-state",
                from: "available-in-state",
            },
        ],
        level: "machine",
        // `delivery`: an expiry is delivered within the bound the instance offers; `chain`: a deadline farther than one
        // delay reaches is reached by re-arming, each delay computed from a coherent read of the time base its
        // requester reads.
        obligations: &["delivery", "chain"],
        // `expiry`: one interrupt at its provider per deadline delivered and per re-arm.
        costs: &["expiry"],
        mediates: false,
    },
    // The relation record §4's epoch extender: the counter extended to `(pow2 64)` counts by an epoch, shared.
    Kind {
        name: "epoch-extender",
        beneath: &[
            Beneath::Needs("relative-delay"),
            Beneath::Needs("counter-modulus"),
            Beneath::Needs("tick-rate"),
            Beneath::Needs("observation-coherent"),
            Beneath::Constraint {
                fact: "wrap-behavior",
                value: "modular",
            },
        ],
        offers: &[
            Offer::Value {
                fact: "counter-modulus",
                value: "(pow2 64)",
            },
            Offer::Value {
                fact: "counter-width",
                value: "64 bit",
            },
            Offer::From {
                fact: "tick-rate",
                from: "tick-rate",
            },
            Offer::From {
                fact: "tick-unit",
                from: "tick-unit",
            },
            Offer::Value {
                fact: "wrap-behavior",
                value: "modular",
            },
            Offer::Value {
                fact: "observation-coherent",
                value: "true",
            },
            Offer::From {
                fact: "available-in-state",
                from: "available-in-state",
            },
        ],
        level: "machine",
        // `update`: the epoch is advanced by a wake programmed within the provider's unambiguous horizon; `coherent`: a
        // read joins epoch and counter so that no update is torn.
        obligations: &["update", "coherent"],
        // `wake`: one interrupt at its provider per update period, the period the realization states and no longer than
        // that horizon.
        costs: &["wake"],
        mediates: false,
    },
];

thread_local! {
    /// The table [`kind`] reads on this thread: [`ADAPTERS`], save while [`with_table`] runs.
    static TABLE: Cell<&'static [Kind]> = const { Cell::new(ADAPTERS) };
}

/// Runs `judge` with `table` in place of [`ADAPTERS`] on this thread, and puts back the table it replaced when `judge`
/// returns or panics: a test catalog's seam (record §6). The table changes, never [`VERSION`], which the record still
/// names: a test catalog is no version of its own.
pub fn with_table<R>(table: &'static [Kind], judge: impl FnOnce() -> R) -> R {
    /// The table to put back, when dropped.
    struct Restore(&'static [Kind]);
    impl Drop for Restore {
        fn drop(&mut self) {
            TABLE.with(|t| t.set(self.0));
        }
    }
    let _restore = Restore(TABLE.with(|t| t.replace(table)));
    judge()
}

/// The kind named `name`, if the table in force holds one: `/1`'s, save within [`with_table`].
#[must_use]
pub fn kind(name: &str) -> Option<&'static Kind> {
    TABLE.with(Cell::get).iter().find(|k| k.name == name)
}

#[cfg(test)]
mod tests {
    use super::{kind, with_table, Kind, Offer, ADAPTERS, VERSION};
    use crate::model::vocab::{entry, Direction, Domain, Function, Role};

    #[test]
    fn each_kind_adds_what_the_record_says() {
        // Record §7.1, which names `/1`'s adds kind by kind.
        let adds = |name| kind(name).expect(name).adds();
        assert_eq!(
            adds("deadline-over-delay"),
            [
                "absolute-deadline",
                "supported-horizon",
                "delivery-bound",
                "channels"
            ]
        );
        assert_eq!(adds("epoch-extender"), ["counter-width"]);
    }

    #[test]
    fn the_deadline_translator_alone_is_exclusive() {
        // Record §2's Instances column, which §1.4 reads off `channels`.
        assert!(kind("deadline-over-delay").unwrap().exclusive());
        assert!(!kind("epoch-extender").unwrap().exclusive());
        // §1.6: an exclusive kind offers `channels` at `1 tick` and takes a channel beneath, a consumed fact.
        for k in ADAPTERS.iter().filter(|k| k.exclusive()) {
            let channels = Offer::Value {
                fact: "channels",
                value: "1 tick",
            };
            assert!(k.offers.contains(&channels), "{}", k.name);
            assert!(
                k.beneath_names()
                    .iter()
                    .any(|f| entry(f).is_some_and(|e| e.consumed)),
                "{}",
                k.name
            );
        }
    }

    #[test]
    fn kind_finds_each_kind_and_refuses_another_name() {
        assert_eq!(VERSION, "adapters/1");
        assert_eq!(
            ADAPTERS.iter().map(|k| k.name).collect::<Vec<_>>(),
            ["deadline-over-delay", "epoch-extender"]
        );
        for k in ADAPTERS {
            assert_eq!(kind(k.name), Some(k));
        }
        // §2's kinds not in `/1`.
        assert_eq!(kind("multiplexer"), None);
        assert_eq!(kind("rescaler"), None);
    }

    #[test]
    fn a_test_table_is_in_force_for_one_call_and_then_1_s_again() {
        static MEDIATING: [Kind; 1] = [Kind {
            name: "test-mediator",
            mediates: true,
            ..ADAPTERS[1]
        }];
        let inside = with_table(&MEDIATING, || {
            (kind("test-mediator").copied(), kind("epoch-extender"))
        });
        assert_eq!(inside, (Some(MEDIATING[0]), None));
        assert_eq!(kind("test-mediator"), None);
        assert_eq!(kind("epoch-extender"), Some(&ADAPTERS[1]));
        // Put back when the judgement panics too.
        let panicked = std::panic::catch_unwind(|| with_table(&MEDIATING, || panic!("judged")));
        assert!(panicked.is_err());
        assert_eq!(kind("test-mediator"), None);
        assert_eq!(kind("deadline-over-delay"), Some(&ADAPTERS[0]));
    }

    #[test]
    fn only_an_exclusive_kind_obliges_and_only_a_bound() {
        // Record §1.5: an exclusive kind alone, a fact of a `quantity` domain and an `at-most` or `at-least` direction.
        let mut obliged = Vec::new();
        for k in ADAPTERS {
            for o in k.offers {
                if let Offer::Obliged { fact } = *o {
                    assert!(k.exclusive(), "{} obliges {fact}", k.name);
                    let e = entry(fact).expect(fact);
                    assert!(matches!(e.domain, Domain::Quantity(_)), "{fact}");
                    assert!(
                        matches!(e.direction, Direction::AtMost | Direction::AtLeast),
                        "{fact}"
                    );
                    obliged.push(fact);
                }
            }
        }
        assert_eq!(obliged, ["delivery-bound"]);
    }

    #[test]
    fn a_qualifier_is_offered_only_passed_through_from_itself() {
        // Record §1.6 — a kind's level is its `level` clause's — and §7.1: a qualifier is never added.
        let mut qualifiers = 0;
        for k in ADAPTERS {
            for o in k.offers {
                let fact = o.fact();
                if entry(fact).is_some_and(|e| e.function == Some(Function::Each)) {
                    assert_eq!(*o, Offer::From { fact, from: fact }, "{}", k.name);
                    assert!(!k.adds().contains(&fact), "{}: {fact}", k.name);
                    qualifiers += 1;
                }
            }
        }
        // `available-in-state`, at each kind.
        assert_eq!(qualifiers, 2);
    }

    #[test]
    fn every_fact_named_is_the_vocabulary_s() {
        let Domain::Set(levels) = entry("reachable-at-privilege").unwrap().domain else {
            panic!("reachable-at-privilege is a set");
        };
        for k in ADAPTERS {
            for f in k.beneath_names() {
                assert!(entry(f).is_some(), "{}: {f} beneath", k.name);
            }
            for o in k.offers {
                let fact = o.fact();
                let e = entry(fact).unwrap_or_else(|| panic!("{}: {fact} offered", k.name));
                // §1.6: each fact offered once, never a statement, and `(from g)` within one domain.
                assert_eq!(
                    k.offers.iter().filter(|p| p.fact() == fact).count(),
                    1,
                    "{}: {fact}",
                    k.name
                );
                assert_eq!(e.role, Role::Guarantee, "{}: {fact}", k.name);
                if let Offer::From { from, .. } = *o {
                    let g = entry(from).unwrap_or_else(|| panic!("{}: {from} read", k.name));
                    assert_eq!(e.domain, g.domain, "{}: {fact} from {from}", k.name);
                }
            }
            // §1.6: its level is of `reachable-at-privilege`'s domain.
            assert!(levels.contains(&k.level), "{}: {}", k.name, k.level);
            assert!(!k.mediates, "{}: `/1` holds no mediating kind (§6)", k.name);
        }
    }
}
