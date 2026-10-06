//! One test per domain of `docs/decisions/decision_substitutability-relation.md` §2's table (`SR-H1`, leaf
//! `M3.1.2.3`): what an offer and a requirement write, the direction the domain admits, and when a value satisfies a
//! required one — the "Satisfied when" column, row by row — with §4's horizon from the record's worked values.

use eadl_front::{read, Form, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_model::Rational;
use eadl_resolve::value::{self, horizon, same, satisfies, NotOfDomainKind, Overflow, Value};
use eadl_resolve::vocabulary::{Direction, Fact, Vocabulary};

fn vocabulary() -> Vocabulary {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf();
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root.join(r)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the kinds load");
    Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
}

/// The value `text` writes for `fact`.
fn val(fact: &Fact, text: &str) -> Result<Value, NotOfDomainKind> {
    let mut sources = SourceMap::new();
    let id = sources
        .add("v.eadl", format!("({} {text})", fact.name))
        .expect("small");
    let (doc, diags) = read(&sources, id);
    assert!(!diags.has_errors(), "{text}");
    let form: &Form = &doc.forms[0];
    value::read(fact, &form.items()[1..], form.span()).map_err(|w| w.kind)
}

fn ok(fact: &Fact, text: &str) -> Value {
    val(fact, text).unwrap_or_else(|k| panic!("{} {text}: {k:?}", fact.name))
}

/// Whether `offered` satisfies `required` for `fact`, in `direction`.
fn sat(fact: &Fact, offered: &str, required: &str, direction: Direction) -> bool {
    satisfies(
        &ok(fact, offered),
        &ok(fact, required),
        direction,
        fact.order(),
        &fact.implies,
    )
    .expect("within the arithmetic")
}

#[test]
fn boolean_the_values_are_equal() {
    let v = vocabulary();
    let f = v.fact("observation-coherent").expect("in /1");
    assert!(f.domain.admits(Direction::Exact) && !f.domain.admits(Direction::AtLeast));
    assert!(sat(f, "true", "true", Direction::Exact));
    assert!(!sat(f, "false", "true", Direction::Exact));
    assert!(sat(f, "false", "false", Direction::Exact));
    for wrong in ["maybe", "1", "true false"] {
        assert_eq!(val(f, wrong), Err(NotOfDomainKind::Outside), "{wrong}");
    }
}

#[test]
fn count_the_integers_compare_in_the_direction() {
    let v = vocabulary();
    let queue = v.fact("queue-capacity").expect("in /1");
    assert!(
        sat(queue, "8", "8 tick", Direction::AtLeast),
        "a dimensionless unit is a count's own spelling"
    );
    assert!(sat(queue, "9", "8", Direction::AtLeast));
    assert!(!sat(queue, "7", "8", Direction::AtLeast));
    assert!(
        !sat(queue, "9", "8", Direction::Exact),
        "`exactly` admits no better value"
    );
    let modulus = v.fact("counter-modulus").expect("in /1");
    assert_eq!(ok(modulus, "(pow2 64)"), Value::Count(1i128 << 64));
    assert_eq!(ok(modulus, "(pow2 126)"), Value::Count(1i128 << 126));
    assert!(
        sat(modulus, "4294967296", "(pow2 32)", Direction::Exact),
        "one count, two spellings"
    );
    for wrong in ["-1", "1.5", "8 bit", "(pow2 -1)", "(pow2 127)"] {
        assert_eq!(
            val(modulus, wrong),
            Err(NotOfDomainKind::Outside),
            "{wrong}"
        );
    }
    assert_eq!(val(modulus, "0"), Err(NotOfDomainKind::ZeroModulus));
    assert_eq!(
        val(queue, "0"),
        Ok(Value::Count(0)),
        "0 is refused for the modulus alone"
    );
}

#[test]
fn quantity_compares_in_the_fact_direction_in_one_unit_as_written() {
    let v = vocabulary();
    let width = v.fact("counter-width").expect("in /1");
    assert!(
        sat(width, "64 bit", "32 bit", Direction::AtLeast),
        "§6: more bits satisfy `at-least`"
    );
    assert!(
        !sat(width, "64 bit", "32 bit", Direction::Exact),
        "§6: not where the width is the interface"
    );
    assert!(
        sat(width, "4 byte", "32 bit", Direction::Exact),
        "two units, compared in the base unit"
    );
    assert_eq!(val(width, "0.5 bit"), Err(NotOfDomainKind::NotWholeBits));
    assert_eq!(val(width, "0 bit"), Err(NotOfDomainKind::NotWholeBits));
    assert_eq!(
        val(width, "8 s"),
        Err(NotOfDomainKind::Outside),
        "another dimension"
    );
    let delivery = v.fact("delivery-bound").expect("in /1");
    assert!(
        sat(delivery, "20 us", "50 us", Direction::AtMost),
        "a shorter delivery satisfies"
    );
    assert!(!sat(delivery, "80 us", "50 us", Direction::AtMost));
    let rate = v.fact("tick-rate").expect("in /1");
    assert!(
        !sat(rate, "20 MHz", "10 MHz", Direction::Exact),
        "§3 rule 3: 20 MHz does not satisfy 10 MHz"
    );
    // Two tiny amounts in one unit compare as written; in two units the conversion is past the arithmetic (§2, §5).
    let time = v.fact("release-accuracy").expect("in /1");
    let tiny = ok(time, "0.0000000000000000000000000000001 ns");
    assert_eq!(same(&tiny, &tiny), Ok(true));
    let tiny_us = ok(time, "0.0000000000000000000000000000000001 us");
    assert_eq!(same(&tiny, &tiny_us), Err(Overflow));
}

#[test]
fn interval_the_required_one_lies_inside_the_offered_one() {
    let v = vocabulary();
    let f = v.fact("frequency").expect("in /1");
    assert!(f.domain.admits(Direction::Within) && f.domain.admits(Direction::Exact));
    assert!(
        sat(f, "(range 1 MHz 200 MHz)", "10 MHz", Direction::Within),
        "a point is [v, v]"
    );
    assert!(sat(
        f,
        "(range 1 MHz 200 MHz)",
        "(range 10 MHz 100 MHz)",
        Direction::Within
    ));
    assert!(!sat(
        f,
        "(range 1 MHz 200 MHz)",
        "(range 50 MHz 300 MHz)",
        Direction::Within
    ));
    assert!(!sat(
        f,
        "(range 1 MHz 200 MHz)",
        "250 MHz",
        Direction::Within
    ));
    assert!(
        sat(
            f,
            "(range 1000 kHz 0.2 GHz)",
            "(range 1 MHz 200 MHz)",
            Direction::Exact
        ),
        "equal in base units"
    );
    assert!(!sat(
        f,
        "(range 1 MHz 200 MHz)",
        "(range 10 MHz 100 MHz)",
        Direction::Exact
    ));
    assert_eq!(
        val(f, "(range 200 MHz 1 MHz)"),
        Err(NotOfDomainKind::Reversed)
    );
    assert_eq!(val(f, "(range 10 s 20 s)"), Err(NotOfDomainKind::Outside));
}

#[test]
fn enumeration_equal_or_in_a_declared_order() {
    let v = vocabulary();
    let unit = v.fact("tick-unit").expect("in /1");
    assert!(sat(unit, "ns", "ns", Direction::Exact));
    assert!(
        !sat(unit, "us", "ns", Direction::Exact),
        "§6: `us` does not satisfy `ns`"
    );
    assert!(unit.order().is_empty() && !unit.domain.admits(Direction::AtLeast));
    assert_eq!(val(unit, "zzz"), Err(NotOfDomainKind::Outside));
    // No `/1` entry is ordered, so an order's reading is held on a value-level order (R20 remark 7).
    let order = ["low", "mid", "high"].map(str::to_string);
    let e = |s: &str| Value::Enum(s.to_string());
    assert_eq!(
        satisfies(&e("high"), &e("mid"), Direction::AtLeast, &order, &[]),
        Ok(true)
    );
    assert_eq!(
        satisfies(&e("low"), &e("mid"), Direction::AtLeast, &order, &[]),
        Ok(false)
    );
    assert_eq!(
        satisfies(&e("low"), &e("mid"), Direction::AtMost, &order, &[]),
        Ok(true)
    );
    assert_eq!(
        satisfies(&e("other"), &e("mid"), Direction::AtLeast, &order, &[]),
        Ok(false)
    );
}

#[test]
fn set_the_required_set_with_its_implied_members_lies_inside_the_offered_one() {
    let v = vocabulary();
    let state = v.fact("available-in-state").expect("in /1");
    // §6, power state: `run idle` ⊇ required `idle` with `run`; `run` lacks `idle`; `idle` alone lacks `run`.
    assert!(sat(state, "run idle", "idle", Direction::Includes));
    assert!(!sat(state, "run", "idle", Direction::Includes));
    assert!(!sat(state, "idle", "idle", Direction::Includes));
    assert!(
        sat(state, "run idle", "idle", Direction::Exact),
        "`exactly idle` asks for exactly run and idle"
    );
    assert!(!sat(state, "run idle sleep", "idle", Direction::Exact));
    let privilege = v.fact("reachable-at-privilege").expect("in /1");
    // §6, access privilege: inclusion reads no order, so `machine` does not satisfy a `supervisor` requirer.
    assert!(sat(
        privilege,
        "supervisor machine",
        "supervisor",
        Direction::Includes
    ));
    assert!(!sat(
        privilege,
        "machine",
        "supervisor",
        Direction::Includes
    ));
    assert!(!sat(privilege, "user", "supervisor", Direction::Includes));
    assert_eq!(val(privilege, ""), Err(NotOfDomainKind::Empty));
    assert_eq!(val(privilege, "user user"), Err(NotOfDomainKind::Outside));
    assert_eq!(val(privilege, "user root"), Err(NotOfDomainKind::Outside));
}

#[test]
fn group_the_head_is_a_boolean_and_each_sub_fact_its_own() {
    let v = vocabulary();
    let head = v.fact("absolute-deadline").expect("in /1");
    assert!(head.domain.admits(Direction::Exact) && !head.domain.admits(Direction::Includes));
    assert!(sat(head, "true", "true", Direction::Exact));
    assert!(!sat(head, "false", "true", Direction::Exact));
    let horizon_of = v
        .fact("supported-horizon")
        .expect("a sub-fact is a fact in its own right");
    assert!(
        sat(horizon_of, "3600 s", "10 s", Direction::AtLeast),
        "§6, programming range"
    );
    assert!(!sat(horizon_of, "5 s", "10 s", Direction::AtLeast));
}

#[test]
fn the_horizon_is_the_modulus_less_one_over_the_rate() {
    // §4's worked values, exact.
    let v = vocabulary();
    let rate = |text: &str| match ok(v.fact("tick-rate").expect("in /1"), text) {
        Value::Quantity(q) => q,
        other => panic!("{other:?}"),
    };
    let seconds = |h: eadl_model::Quantity| h.in_base().expect("fits");
    let r = |n: i128, scale: u32| Rational::new(n, 10i128.pow(scale)).expect("fits");
    assert_eq!(
        seconds(horizon(4_294_967_296, rate("10 MHz")).expect("fits")),
        r(4_294_967_295, 7)
    );
    assert_eq!(
        seconds(horizon(65_536, rate("10 MHz")).expect("fits")),
        r(65_535, 7)
    );
    assert_eq!(
        seconds(horizon(1_000_000, rate("10 MHz")).expect("fits")),
        r(999_999, 7)
    );
    assert_eq!(
        seconds(horizon(600_000_000, rate("10 MHz")).expect("fits")),
        r(599_999_999, 7)
    );
    assert_eq!(
        seconds(horizon(4_294_967_296, rate("1 MHz")).expect("fits")),
        r(4_294_967_295, 6)
    );
    // (2^64 − 1) / 10 MHz fits; a rate so slow its quotient does not is the arithmetic's limit (§2).
    assert_eq!(
        seconds(horizon(1i128 << 64, rate("10 MHz")).expect("fits")),
        r((1i128 << 64) - 1, 7)
    );
    let slow = rate("0.0000000000000000000000000000001 Hz");
    assert_eq!(horizon(1i128 << 126, slow), Err(Overflow));
}
