//! An engine adapter's instance as the checks judge it, from the checker's own table ([`crate::adapters`]) and
//! `docs/specs/adapters/decision_engine-adapters.md` §1, §3.1, §4 and §6 alone, under `M3.5`'s fence — never from the
//! generator's reading of the catalog. An instance is a requester for what its kind requires beneath, read by the
//! model's side reader as any requirer's (§1.3), and a provider for what its kind offers over the device it programs,
//! read into one state per fact as the model reads a block's, so the relation judges it like any provider (§1.5).
//!
//! # Where the record leaves a choice, the reading taken
//!
//! - §1.3, `(f (from g))`: the instance states of `f` what its device's outcome of `g` is — a value offered or derived
//!   there is the value; offered without one, `f` is offered without one; declared absent there, `f` is absent; past the
//!   arithmetic, so is `f`; undescribed there, the instance does not describe `f`.
//! - §1.5, an obliged offer: the requirements read are every one the requester it serves writes, its side read whole,
//!   on that fact — flat or a group's sub-constraint; two bounds whose order is past the exact arithmetic leave the fact
//!   past it, `unsupported-profile` where it is judged.
//! - §4 rule 9, its level: it states `reachable-at-privilege` as the set of its kind's one level.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{Form, SourceMap};

use crate::adapters::{self, Beneath, Kind, Offer};
use crate::assignment::{Instance, Provider as Bound};
use crate::model::judge::{outcome, Outcome};
use crate::model::provider::{Provider, Stated};
use crate::model::requirement::{read_side, Requirement};
use crate::model::value::{self, order_of, Overflow, Value};
use crate::model::vocab::{self, Direction};

/// The relation's one statement, which the mediation gate reads (record §6; `SR-H8`).
pub(crate) const MEDIATION: &str = "or-through-mediation";

/// The qualifier an instance states its kind's level as (record §4 rule 9).
const PRIVILEGE: &str = "reachable-at-privilege";

/// The kind `instance` names, if the checker's table holds it.
pub(crate) fn kind(instance: &Instance) -> Option<&'static Kind> {
    adapters::kind(&instance.kind)
}

/// The device a bound provider is (model §8 rule 14): a block or platform its own, an instance the one it programs.
pub(crate) fn device(provider: &Bound) -> &str {
    match provider {
        Bound::Named(name) => name,
        Bound::Instance(instance) => &instance.over,
    }
}

/// The one form `text` holds, read as a description's are.
fn form(text: &str) -> Result<Form, String> {
    let mut sources = SourceMap::new();
    let id = sources
        .add("adapters", text)
        .map_err(|e| format!("`{text}`: {e}"))?;
    let (document, diagnostics) = eadl_front::read(&sources, id);
    if diagnostics.has_errors() {
        return Err(diagnostics.render(&sources));
    }
    document
        .forms
        .first()
        .cloned()
        .ok_or_else(|| format!("`{text}` holds no form"))
}

/// How the table writes one item of a kind's `beneath`: `(needs f)`, or the constraint `(f v)`.
pub(crate) fn written(item: &Beneath) -> String {
    match *item {
        Beneath::Needs(fact) => format!("(needs {fact})"),
        Beneath::Constraint { fact, value } => format!("({fact} {value})"),
    }
}

/// What `kind` requires beneath of the provider it programs, read by the model's side reader as any requirer's side
/// (record §1.3): `(needs …)` for its `needs` names, each constraint as written.
///
/// # Errors
///
/// Why the table's `beneath` does not read: the checker's own fault, which a test of this module rules out.
pub(crate) fn beneath(kind: &Kind) -> Result<Vec<Requirement>, String> {
    let items: Vec<String> = kind.beneath.iter().map(written).collect();
    let side = form(&format!(
        "(defblock adapter (requires {}))",
        items.join(" ")
    ))?;
    read_side(&side).map_err(|refused| format!("`{}` beneath: {refused:?}", kind.name))
}

/// A literal offer's value, `(f v)` read in `f`'s domain.
fn literal(fact: &str, written: &str) -> Result<Value, String> {
    let entry =
        vocab::entry(fact).ok_or_else(|| format!("`{fact}` is no fact of the vocabulary"))?;
    let offer = form(&format!("({fact} {written})"))?;
    value::read(entry.domain, &offer.items()[1..], offer.span())
        .map_err(|malformed| format!("`({fact} {written})`: {}", malformed.0))
}

/// The order of two values of one bound, `None` past the exact arithmetic or between values no bound compares.
fn order(a: &Value, b: &Value) -> Option<Ordering> {
    match (a, b) {
        (Value::Quantity(x), Value::Quantity(y)) => order_of(*x, *y).ok(),
        (Value::Count(x), Value::Count(y)) => Some(x.cmp(y)),
        _ => None,
    }
}

/// Every constraint `requirements` write on `fact`, a group's sub-constraints among them: its direction and value.
fn bounds<'r>(fact: &str, requirements: &'r [Requirement], out: &mut Vec<(Direction, &'r Value)>) {
    for requirement in requirements {
        match requirement {
            Requirement::Constraint {
                fact: f,
                direction,
                value,
            } if f == fact => out.push((*direction, value)),
            Requirement::Group { parts, .. } => bounds(fact, parts, out),
            _ => {}
        }
    }
}

/// The tightest bound `requirements` write on the obliged `fact` (record §1.5): an `exactly` bound's value; else the
/// least of the `at-most` bounds, or the greatest of the `at-least` ones, in the fact's direction. `None` where they
/// write no bound on it.
///
/// # Errors
///
/// [`Overflow`] where two bounds' order is past the exact arithmetic.
pub(crate) fn tightest(
    fact: &str,
    requirements: &[Requirement],
) -> Option<Result<Value, Overflow>> {
    let mut written = Vec::new();
    bounds(fact, requirements, &mut written);
    if let Some((_, value)) = written.iter().find(|(d, _)| *d == Direction::Exact) {
        return Some(Ok((*value).clone()));
    }
    let direction = vocab::entry(fact)?.direction;
    let mut best: Option<&Value> = None;
    for (_, value) in written.into_iter().filter(|(d, _)| *d == direction) {
        best = Some(match best {
            None => value,
            Some(held) => match (order(held, value), direction) {
                (None, _) => return Some(Err(Overflow)),
                (Some(Ordering::Greater), Direction::AtMost)
                | (Some(Ordering::Less), Direction::AtLeast) => value,
                (Some(_), _) => held,
            },
        });
    }
    best.map(|value| Ok(value.clone()))
}

/// What an instance of `kind` offers over `over`, the device it programs as the model reads it, named `name` (record
/// §1.3, §3.1): each value as its kind writes it; `(f (from g))` as `over`'s own outcome of `g`, offered or derived
/// there; an obliged fact at the tightest bound `requester` — the requirements of the one requester it serves — writes
/// on it, bare where it writes none; and its kind's level as `reachable-at-privilege`. A fact it neither offers nor
/// passes through is undescribed there, and a derived one is derived from what it offers, as at any provider.
///
/// # Errors
///
/// Why a literal offer of the table does not read: the checker's own fault, which a test of this module rules out.
pub(crate) fn offered(
    kind: &Kind,
    name: &str,
    over: &Provider,
    requester: &[Requirement],
) -> Result<Provider, String> {
    let mut stated = BTreeMap::new();
    for offer in kind.offers {
        let (fact, state) = match *offer {
            Offer::Value { fact, value } => (fact, Stated::Valued(vec![literal(fact, value)?])),
            Offer::From { fact, from } => (
                fact,
                match outcome(over, from) {
                    Ok(Outcome::Valued(values)) => Stated::Valued(values),
                    Ok(Outcome::Derived(value)) => Stated::Valued(vec![value]),
                    Ok(Outcome::Unknown) => Stated::Unvalued,
                    Ok(Outcome::Absent) => Stated::Absent,
                    Ok(Outcome::Undescribed) => continue,
                    Err(Overflow) => Stated::Overflow,
                },
            ),
            Offer::Obliged { fact } => (
                fact,
                match tightest(fact, requester) {
                    None => Stated::Unvalued,
                    Some(Ok(value)) => Stated::Valued(vec![value]),
                    Some(Err(Overflow)) => Stated::Overflow,
                },
            ),
        };
        stated.insert(fact.to_owned(), state);
    }
    stated.insert(
        PRIVILEGE.to_owned(),
        Stated::Valued(vec![Value::Set(BTreeSet::from([kind.level.to_owned()]))]),
    );
    Ok(Provider {
        name: name.to_owned(),
        stated,
    })
}

/// The mediation gate (record §6; `SR-H8`): a kind writing `(mediates true)` is offered only to a requester that wrote
/// `(or-through-mediation allowed)`, bare or under `exactly` — both read as the statement's one value.
pub(crate) fn admits(kind: &Kind, requester: &[Requirement]) -> bool {
    !kind.mediates
        || requester.iter().any(|r| {
            matches!(r, Requirement::Statement(fact, Value::Enum(word)) if fact == MEDIATION && word == "allowed")
        })
}

#[cfg(test)]
mod tests {
    use super::{admits, beneath, form, literal, offered, tightest};
    use crate::adapters::{kind, Kind, Offer, ADAPTERS};
    use crate::model::judge::{judge, Verdict};
    use crate::model::provider::{self, Stated};
    use crate::model::requirement::{read_side, Requirement};
    use crate::model::value::same;

    fn side(text: &str) -> Vec<Requirement> {
        read_side(&form(text).expect("a form")).expect("a side")
    }

    fn device(text: &str) -> provider::Provider {
        provider::read(&form(text).expect("a form")).expect("a provider")
    }

    #[test]
    fn every_kind_s_beneath_and_literal_offers_read() {
        for k in ADAPTERS {
            let read = beneath(k).unwrap_or_else(|why| panic!("{why}"));
            assert_eq!(read.len(), k.beneath.len(), "{}", k.name);
            for o in k.offers {
                if let Offer::Value { fact, value } = *o {
                    literal(fact, value).unwrap_or_else(|why| panic!("{}: {why}", k.name));
                }
            }
        }
    }

    #[test]
    fn an_instance_offers_its_values_its_device_s_facts_and_its_level() {
        let timer = device(
            "(defblock timer (offers (relative-delay true) (counter-modulus 65536) (tick-rate 1 MHz) \
             (observation-coherent true) (wrap-behavior modular) (available-in-state run idle)))",
        );
        let extender = offered(kind("epoch-extender").unwrap(), "x", &timer, &[]).unwrap();
        // Its own modulus and its device's rate: a horizon derived at the instance, `(2^64 − 1) / 1 MHz`, where the
        // device's own, `65535 / 1 MHz`, falls short; its device's states passed through, and its kind's level stated.
        let held = side(
            "(defservice s (requires (unambiguous-horizon (at-least 60 s)) (counter-width (at-least 64 bit)) \
             (tick-rate 1 MHz) (available-in-state idle) (reachable-at-privilege machine)))",
        );
        for r in &held {
            assert_eq!(judge(&extender, r), Verdict::Satisfied, "{r:?}");
        }
        assert_eq!(judge(&timer, &held[0]), Verdict::Refused);
        // A counter fact the kind neither offers nor passes through, undescribed at its device, is undescribed here.
        let unit = side("(defservice s (requires (tick-unit us)))");
        assert_eq!(judge(&extender, &unit[0]), Verdict::Undescribed);
        assert!(!extender.stated.contains_key("tick-unit"));
    }

    #[test]
    fn an_obliged_offer_takes_its_requester_s_tightest_bound() {
        let timer = device(
            "(defblock timer (offers (relative-delay true) (counter-modulus 4294967296) (tick-rate 1 MHz) \
             (observation-coherent true) (channels 1 tick)))",
        );
        let k = kind("deadline-over-delay").unwrap();
        let bound = |text: &str| {
            let requester = side(text);
            offered(k, "d", &timer, &requester)
                .unwrap()
                .stated
                .get("delivery-bound")
                .cloned()
        };
        let is = |state: Option<Stated>, written: &str| match state {
            Some(Stated::Valued(values)) => {
                same(&values[0], &literal("delivery-bound", written).unwrap()) == Ok(true)
            }
            _ => false,
        };
        // The least `at-most`, a group's sub-constraint and a flat clause alike.
        assert!(is(
            bound(
                "(defservice a (requires (absolute-deadline (delivery-bound (at-most 200 us))) \
                 (delivery-bound 50 us)))"
            ),
            "50 us"
        ));
        assert!(is(
            bound("(defservice a (requires (absolute-deadline (delivery-bound (at-most 1 ms))) (delivery-bound 2 ms)))"),
            "1 ms"
        ));
        // An `exactly` bound's value.
        assert!(is(
            bound("(defservice a (requires (delivery-bound (exactly 70 us)) (delivery-bound (at-most 90 us))))"),
            "70 us"
        ));
        // No bound written: offered bare.
        assert!(matches!(
            bound("(defservice a (requires (needs absolute-deadline)))"),
            Some(Stated::Unvalued)
        ));
        assert!(tightest("delivery-bound", &[]).is_none());
    }

    #[test]
    fn the_mediation_gate_admits_a_mediating_kind_only_where_the_requester_allowed_it() {
        // `/1`'s catalog holds no mediating kind (record §6): a kind of this test's own holds the gate both ways.
        let base = kind("epoch-extender").unwrap();
        let mediating = Kind {
            name: "test-mediator",
            mediates: true,
            ..*base
        };
        for (text, admitted) in [
            (
                "(defservice a (requires (or-through-mediation allowed)))",
                true,
            ),
            (
                "(defservice a (requires (or-through-mediation (exactly allowed))))",
                true,
            ),
            (
                "(defservice a (requires (or-through-mediation forbidden)))",
                false,
            ),
            ("(defservice a (requires (needs relative-delay)))", false),
        ] {
            let requester = side(text);
            assert_eq!(admits(&mediating, &requester), admitted, "{text}");
            assert!(
                admits(base, &requester),
                "a kind that does not mediate: {text}"
            );
        }
    }
}
