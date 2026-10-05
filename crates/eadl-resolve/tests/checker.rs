//! The model's bounded exhaustive checker (leaf `M3.1.1.1`, `docs/decisions/decision_executable-design-reviews.md`).
//!
//! It enumerates a finite universe — every vocabulary fact, every written form of an offer, values on both sides of
//! every required value — and asserts the properties `M3.1.1`'s acceptance names, on every input:
//!
//! - **no value read the wrong way**: a satisfied constraint holds by an oracle written here, from §2's "Satisfied
//!   when" column alone, and a refused one does not;
//! - **`exactly` admits only what every direction admits**: satisfied under `exactly` ⇒ satisfied in the fact's
//!   direction;
//! - **no stronger precondition read as a capability**: a set judged by inclusion is satisfied only when every
//!   required member, and `run` for a power state, is offered;
//! - **one outcome per input**: every reading is total — refused, or judged to exactly one verdict;
//! - **the derivation agrees with what it derives**: the horizon satisfies `(at-least T)` exactly when two reads `T`
//!   apart are unambiguous at every phase, by an exact simulation that never uses the rule's formula;
//! - **a provider's reading is order-free**: three offers of one fact read alike in all six orders, one of them past
//!   the arithmetic where the domain has one, and two unequal ones are two values whatever the third (R18 1);
//! - **`absent` names a fact by name alone**: a value written inside it is refused (R18 2);
//! - **the modulus is bounded by its width at every width**, to 127 bits (R18 9);
//! - **writing an offer twice changes nothing** (R19 1);
//! - **a clause holds one value per equality**: two statements or equalities on one fact with two values refused, one
//!   value twice accepted (R19 2).
//!
//! Each property also has a catalogued mutation in `xtask/mutations.txt` that breaks the rule it guards, and this
//! file must kill it.

use std::collections::BTreeSet;

use eadl_front::{read, Form, SourceMap};
use eadl_model::quantity::{unit, Quantity};
use eadl_model::{Dimension, Rational};
use eadl_resolve::model::{
    judge, read_clause, read_constraint, read_needs, read_provider, Direction, Domain, Requirement,
    Role, Value, Verdict, VOCABULARY,
};

fn forms(text: &str) -> Vec<Form> {
    let mut sources = SourceMap::new();
    let id = sources.add("t.eadl", text).expect("small");
    let (doc, diags) = read(&sources, id);
    assert!(!diags.has_errors(), "{text}: {}", diags.render(&sources));
    doc.forms
}

fn provider(offers: &str) -> Result<eadl_resolve::model::Provider, eadl_resolve::model::Refused> {
    read_provider(&forms(&format!("(defblock p {offers})"))[0])
}

fn constraint(text: &str) -> Result<Requirement, eadl_resolve::model::NotJudged> {
    read_constraint(&forms(text)[0])
}

/// Values a fact's domain is sampled at, each written as an offer or a requirement would write it.
fn samples(domain: Domain) -> Vec<String> {
    let s = |v: &[&str]| v.iter().map(|x| (*x).to_string()).collect::<Vec<_>>();
    match domain {
        Domain::Boolean | Domain::Group(_) => s(&["true", "false"]),
        Domain::Count => s(&[
            "0",
            "1",
            "2",
            "7",
            "8",
            "9",
            "65535",
            "65536",
            "65537",
            "4294967296",
            "(pow2 64)",
            "(pow2 126)",
            // The corpus's own spelling, a count with its dimensionless unit (R18 9).
            "8 tick",
            "65536 tick",
        ]),
        Domain::Quantity(Dimension::Time) => s(&[
            "0 s",
            "1 us",
            "50 us",
            "80 us",
            "1 ms",
            "59.9999999 s",
            "60 s",
            "3600 s",
        ]),
        Domain::Quantity(Dimension::Frequency) => {
            s(&["1 Hz", "18 Hz", "1 MHz", "10 MHz", "10000 kHz", "20 MHz"])
        }
        Domain::Quantity(Dimension::Information) => {
            s(&["8 bit", "16 bit", "32 bit", "4 byte", "64 bit"])
        }
        Domain::Quantity(Dimension::Dimensionless) => s(&["1 tick", "8 tick"]),
        Domain::Interval(_) => s(&[
            "(range 1 MHz 200 MHz)",
            "(range 10 MHz 10 MHz)",
            "(range 50 MHz 300 MHz)",
            "10 MHz",
            "250 MHz",
        ]),
        Domain::Enumeration { alternatives, .. } => s(alternatives),
        Domain::Set(alternatives) => {
            let mut out = Vec::new();
            for mask in 1u32..(1 << alternatives.len()) {
                let members: Vec<&str> = alternatives
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| mask & (1 << i) != 0)
                    .map(|(_, a)| *a)
                    .collect();
                out.push(members.join(" "));
            }
            out
        }
    }
}

fn base(text: &str) -> Rational {
    let (n, u) = text.split_once(' ').expect("number and unit");
    let unit = unit(u).expect("a known unit");
    let value = if let Some((i, f)) = n.split_once('.') {
        let scale = u32::try_from(f.len()).expect("small");
        Rational::decimal(format!("{i}{f}").parse().expect("digits"), scale).expect("fits")
    } else {
        Rational::integer(n.parse().expect("integer"))
    };
    Quantity::new(value, unit)
        .expect("valid")
        .in_base()
        .expect("fits")
}

fn count_of(text: &str) -> i128 {
    let text = text.strip_suffix(" tick").unwrap_or(text);
    if let Some(n) = text
        .strip_prefix("(pow2 ")
        .and_then(|t| t.strip_suffix(')'))
    {
        1i128 << n.parse::<u32>().expect("N")
    } else {
        text.parse().expect("integer")
    }
}

/// §2's "Satisfied when" column, written here from the table alone: the oracle the model is held to.
fn oracle(domain: Domain, name: &str, offered: &str, required: &str, dir: Direction) -> bool {
    let set = |t: &str| t.split(' ').map(str::to_string).collect::<BTreeSet<_>>();
    let interval = |t: &str| -> (Rational, Rational) {
        if let Some(inner) = t.strip_prefix("(range ").and_then(|t| t.strip_suffix(')')) {
            let parts: Vec<&str> = inner.split(' ').collect();
            (
                base(&format!("{} {}", parts[0], parts[1])),
                base(&format!("{} {}", parts[2], parts[3])),
            )
        } else {
            (base(t), base(t))
        }
    };
    match domain {
        Domain::Boolean | Domain::Group(_) | Domain::Enumeration { .. } => offered == required,
        Domain::Count => {
            let (o, r) = (count_of(offered), count_of(required));
            match dir {
                Direction::AtLeast => o >= r,
                Direction::AtMost => o <= r,
                _ => o == r,
            }
        }
        Domain::Quantity(_) => {
            let (o, r) = (base(offered), base(required));
            match dir {
                Direction::AtLeast => o >= r,
                Direction::AtMost => o <= r,
                _ => o == r,
            }
        }
        Domain::Interval(_) => {
            let ((olo, ohi), (rlo, rhi)) = (interval(offered), interval(required));
            match dir {
                Direction::Within => olo <= rlo && rhi <= ohi,
                _ => olo == rlo && ohi == rhi,
            }
        }
        Domain::Set(_) => {
            let (o, mut r) = (set(offered), set(required));
            // §2's set row: the members the entry's `implies` names join the required set (R17 1, 2).
            if let Some(e) = eadl_resolve::model::vocab::entry(name) {
                r.extend(e.implies.iter().map(|m| (*m).to_string()));
            }
            match dir {
                Direction::Includes => r.is_subset(&o),
                _ => o == r,
            }
        }
    }
}

#[test]
fn every_constraint_is_judged_in_its_direction_by_the_oracle() {
    let mut judged = 0usize;
    for e in VOCABULARY.iter().filter(|e| e.role == Role::Guarantee) {
        for o in samples(e.domain) {
            let offer = format!("(offers ({} {o}))", e.name);
            let p = match provider(&offer) {
                Ok(p) => p,
                Err(_) => continue, // a provider §8 refuses is judged by the refusal arms below
            };
            for r in samples(e.domain) {
                let mut written = vec![(format!("({} {r})", e.name), e.direction)];
                written.push((format!("({} (exactly {r}))", e.name), Direction::Exact));
                if matches!(e.direction, Direction::AtLeast | Direction::AtMost) {
                    let w = if e.direction == Direction::AtLeast {
                        "at-least"
                    } else {
                        "at-most"
                    };
                    written.push((format!("({} ({w} {r}))", e.name), e.direction));
                }
                for (text, dir) in written {
                    let Ok(req) = constraint(&text) else {
                        // The reader refuses a requirement only where §8 refuses its value (R17 R6): a modulus of 0, or
                        // a value of information that is no positive whole number of bits.
                        let refused_by_record = (e.name == "counter-modulus" && r == "0")
                            || (matches!(e.domain, Domain::Quantity(Dimension::Information))
                                && !base(&r).is_positive());
                        assert!(
                            refused_by_record,
                            "`{text}` refused by the reader, which §8 does not refuse"
                        );
                        continue;
                    };
                    let v = judge(&p, &req);
                    let want = oracle(e.domain, e.name, &o, &r, dir);
                    assert_eq!(
                        v == Verdict::Satisfied,
                        want,
                        "{offer} against {text}: {v:?}, the oracle says {want}"
                    );
                    assert!(
                        matches!(v, Verdict::Satisfied | Verdict::Refused),
                        "{offer} against {text}: {v:?}"
                    );
                    judged += 1;
                }
            }
        }
    }
    println!("judged {judged} (offer, constraint) pairs against the oracle");
    assert!(judged > 2000, "the universe shrank to {judged}");
}

#[test]
fn exactly_admits_only_what_the_fact_direction_admits() {
    for e in VOCABULARY.iter().filter(|e| e.role == Role::Guarantee) {
        for o in samples(e.domain) {
            let Ok(p) = provider(&format!("(offers ({} {o}))", e.name)) else {
                continue;
            };
            for r in samples(e.domain) {
                let (Ok(exact), Ok(own)) = (
                    constraint(&format!("({} (exactly {r}))", e.name)),
                    constraint(&format!("({} {r})", e.name)),
                ) else {
                    continue;
                };
                if judge(&p, &exact) == Verdict::Satisfied {
                    assert_eq!(
                        judge(&p, &own),
                        Verdict::Satisfied,
                        "{} {o}: `exactly {r}` satisfied, its own direction not",
                        e.name
                    );
                }
            }
        }
    }
}

#[test]
fn no_stronger_precondition_passes_as_a_capability() {
    for e in VOCABULARY
        .iter()
        .filter(|e| e.direction == Direction::Includes)
    {
        let Domain::Set(alternatives) = e.domain else {
            panic!("{} includes over a set", e.name)
        };
        for o in samples(e.domain) {
            let p = provider(&format!("(offers ({} {o}))", e.name)).expect("a set offer reads");
            for r in samples(e.domain) {
                let req =
                    constraint(&format!("({} {r})", e.name)).expect("a set requirement reads");
                if judge(&p, &req) == Verdict::Satisfied {
                    for member in r.split(' ').chain(if e.name == "available-in-state" {
                        Some("run")
                    } else {
                        None
                    }) {
                        assert!(
                            o.split(' ').any(|m| m == member),
                            "{}: `{o}` satisfied `{r}` without `{member}`",
                            e.name
                        );
                    }
                }
            }
        }
        assert!(!alternatives.is_empty());
    }
}

#[test]
fn every_written_form_of_one_offer_has_exactly_one_reading() {
    // Bare, valued, `exactly`, bound, absent, twice, beside `false`: each provider is refused or read, and each
    // requirement judged to exactly one verdict — the model is total over the universe.
    let mut readings = 0usize;
    for e in VOCABULARY.iter() {
        let mut offers = vec![
            format!("(offers {})", e.name),
            format!("(offers ({}))", e.name),
            format!("(absent {})", e.name),
            String::new(),
        ];
        for v in samples(e.domain) {
            offers.push(format!("(offers ({} {v}))", e.name));
            offers.push(format!("(offers ({} (exactly {v})))", e.name));
            offers.push(format!("(offers ({} (at-least {v})))", e.name));
            offers.push(format!("(offers ({} (at-most {v})))", e.name));
            offers.push(format!("(offers {} ({} {v}))", e.name, e.name));
            offers.push(format!("(offers ({} {v}) ({} {v}))", e.name, e.name));
            offers.push(format!("(offers ({} {v})) (absent {})", e.name, e.name));
        }
        for offer in offers {
            let p = provider(&offer);
            readings += 1;
            if e.role == Role::Statement && offer.contains("offers") {
                assert!(
                    p.is_err(),
                    "an offer of a statement fact is refused: {offer}"
                );
                continue;
            }
            let Ok(p) = p else { continue };
            let presence = read_needs(&forms(e.name)[0]);
            if e.role == Role::Statement {
                assert!(
                    presence.is_err(),
                    "a `needs` of a statement fact is refused"
                );
                continue;
            }
            let v = judge(&p, &presence.expect("a guarantee fact is needed"));
            assert!(
                !matches!(v, Verdict::Statement),
                "{offer}: presence judged as a statement"
            );
        }
    }
    println!("read {readings} written offers");
}

/// The largest number of ticks two reads `delta` apart can be separated by, over every phase — computed at the
/// phase breakpoints, where a read's floor changes, never from the rule's formula.
fn widest_advance(delta: Rational, rate_hz: Rational) -> i128 {
    let x = delta.checked_mul(rate_hz).expect("small");
    // A phase φ ∈ [0, 1) of the counter's tick: the advance is ⌊φ + x⌋. It is piecewise constant, changing where
    // φ + x crosses an integer, so trying φ = 0 and the one breakpoint inside [0, 1) covers every phase.
    let floor = |r: Rational| r.floor().expect("small");
    let frac = x
        .checked_sub(Rational::new(floor(x), 1).expect("whole"))
        .expect("small");
    let mut widest = floor(x);
    if !frac.is_zero() {
        let phase = Rational::new(1, 1)
            .expect("one")
            .checked_sub(frac)
            .expect("small");
        widest = widest.max(floor(phase.checked_add(x).expect("small")));
    }
    widest
}

#[test]
fn the_horizon_agrees_with_a_simulation_of_the_counter_s_reads() {
    let mut cases = 0usize;
    for modulus in 1i128..=40 {
        for (rate, hz) in [
            ("1 Hz", Rational::integer(1)),
            ("2 Hz", Rational::integer(2)),
            ("3 Hz", Rational::integer(3)),
            ("18 Hz", Rational::integer(18)),
            ("1.5 Hz", Rational::new(3, 2).expect("3/2")),
        ] {
            let p = provider(&format!(
                "(offers (counter-modulus {modulus}) (tick-rate {rate}))"
            ))
            .expect("a counter reads");
            // Requirement values on a grid of quarter ticks up to two wraps, and one tick-fraction past each.
            for k in 0i128..=(8 * modulus) {
                let t = Rational::new(k, 4)
                    .expect("k/4")
                    .checked_div(hz)
                    .expect("small");
                let req = Requirement::Constraint {
                    fact: "unambiguous-horizon".into(),
                    direction: Direction::AtLeast,
                    value: Value::Quantity(Quantity::new(t, unit("s").expect("s")).expect("valid")),
                };
                let unambiguous = widest_advance(t, hz) < modulus; // the advance is recovered while it is at most modulus − 1
                let v = judge(&p, &req);
                assert_eq!(
                    v == Verdict::Satisfied,
                    unambiguous,
                    "modulus {modulus} at {rate}, horizon asked {k}/4 ticks: {v:?}"
                );
                cases += 1;
            }
        }
    }
    println!("checked {cases} horizon requirements against the simulation");
}

#[test]
fn a_derivation_past_the_arithmetic_is_unsupported_never_a_verdict() {
    let p = provider("(offers (counter-modulus (pow2 126)) (tick-rate 0.25 Hz))").expect("reads");
    let req = constraint("(unambiguous-horizon (at-least 60 s))").expect("reads");
    assert_eq!(judge(&p, &req), Verdict::Unsupported);
}

#[test]
fn a_fact_named_in_needs_is_present_by_any_offer_but_false() {
    for e in VOCABULARY.iter().filter(|e| e.role == Role::Guarantee) {
        let need = read_needs(&forms(e.name)[0]).expect("a guarantee is needed");
        let boolean = matches!(e.domain, Domain::Boolean | Domain::Group(_));
        let bare = provider(&format!("(offers {})", e.name));
        if let Ok(p) = bare {
            let v = judge(&p, &need);
            assert_eq!(
                v,
                Verdict::Satisfied,
                "{}: a bare offer meets presence",
                e.name
            );
        }
        if boolean {
            let p = provider(&format!("(offers ({} false))", e.name)).expect("reads");
            assert_eq!(
                judge(&p, &need),
                Verdict::Refused,
                "{}: `false` meets no presence",
                e.name
            );
        }
        let p = provider(&format!("(absent {})", e.name)).expect("reads");
        assert_eq!(judge(&p, &need), Verdict::Absent, "{}", e.name);
    }
}

#[test]
fn one_provider_offering_one_fact_twice_is_refused_unless_the_value_is_the_same() {
    let mut pairs = 0usize;
    for e in VOCABULARY.iter().filter(|e| e.role == Role::Guarantee) {
        for a in samples(e.domain) {
            if provider(&format!("(offers ({} {a}))", e.name)).is_err() {
                continue;
            }
            for b in samples(e.domain) {
                if provider(&format!("(offers ({} {b}))", e.name)).is_err() {
                    continue;
                }
                let both = provider(&format!("(offers ({} {a}) ({} {b}))", e.name, e.name));
                let same = oracle(e.domain, "", &a, &b, Direction::Exact)
                    && oracle(e.domain, "", &b, &a, Direction::Exact);
                assert_eq!(both.is_ok(), same, "{}: `{a}` beside `{b}`", e.name);
                pairs += 1;
            }
        }
    }
    println!("judged {pairs} pairs of values offered twice");
}

#[test]
fn an_offer_without_a_value_meets_no_valued_constraint() {
    for e in VOCABULARY.iter().filter(|e| {
        e.role == Role::Guarantee && !matches!(e.domain, Domain::Boolean | Domain::Group(_))
    }) {
        let bare = provider(&format!("(offers {})", e.name)).expect("a bare offer reads");
        for r in samples(e.domain) {
            let Ok(req) = constraint(&format!("({} {r})", e.name)) else {
                continue;
            };
            assert_eq!(
                judge(&bare, &req),
                Verdict::Unknown,
                "{}: bare against `{r}`",
                e.name
            );
            if matches!(e.direction, Direction::AtLeast | Direction::AtMost) {
                let w = if e.direction == Direction::AtLeast {
                    "at-least"
                } else {
                    "at-most"
                };
                if let Ok(bound) = provider(&format!("(offers ({} ({w} {r})))", e.name)) {
                    assert_eq!(
                        judge(&bound, &req),
                        Verdict::Unknown,
                        "{}: an abstract bound is no value",
                        e.name
                    );
                }
            }
        }
    }
}

#[test]
fn a_statement_constrains_no_provider_and_is_offered_by_none() {
    for e in VOCABULARY.iter().filter(|e| e.role == Role::Statement) {
        for v in samples(e.domain) {
            assert!(
                provider(&format!("(offers ({} {v}))", e.name)).is_err(),
                "{} offered",
                e.name
            );
            let req = constraint(&format!("({} {v})", e.name)).expect("a statement's word reads");
            let p = provider("").expect("an empty provider reads");
            assert_eq!(judge(&p, &req), Verdict::Statement, "{}", e.name);
        }
        assert!(
            read_needs(&forms(e.name)[0]).is_err(),
            "a `needs` of {}",
            e.name
        );
    }
}

#[test]
fn a_group_is_judged_the_same_whatever_order_its_parts_are_written_in() {
    // R17 4: rule 5 gives a clause's parts no order, so neither does a group's outcome.
    let states = [
        "(supported-horizon 3600 s)",
        "(supported-horizon 5 s)",
        "supported-horizon",
        "",
    ];
    let bounds = [
        "(delivery-bound 1 us)",
        "(delivery-bound 1 ms)",
        "delivery-bound",
        "",
    ];
    let mut judged = 0usize;
    for s in states {
        for b in bounds {
            for absent in ["", "(absent supported-horizon)", "(absent delivery-bound)"] {
                let offers = format!("(offers (absolute-deadline true) {s} {b}) {absent}");
                let Ok(p) = provider(&offers) else { continue };
                let one = constraint("(absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 50 us)))").expect("reads");
                let two = constraint("(absolute-deadline (delivery-bound (at-most 50 us)) (supported-horizon (at-least 10 s)))").expect("reads");
                assert_eq!(judge(&p, &one), judge(&p, &two), "{offers}");
                judged += 1;
            }
        }
    }
    println!("judged {judged} providers against a group in both orders");
    assert!(judged > 20);
}

#[test]
fn a_statement_reads_bare_or_under_exactly() {
    for e in VOCABULARY.iter().filter(|e| e.role == Role::Statement) {
        for v in samples(e.domain) {
            let p = provider("").expect("an empty provider reads");
            for text in [
                format!("({} {v})", e.name),
                format!("({} (exactly {v}))", e.name),
            ] {
                let req = constraint(&text).unwrap_or_else(|e| panic!("{text}: {e:?}"));
                assert_eq!(judge(&p, &req), Verdict::Statement, "{text}");
            }
        }
    }
}

/// Sameness by the oracle alone, no fact's `implies` joined: what "two values" compares.
fn same_by_oracle(domain: Domain, x: &str, y: &str) -> bool {
    oracle(domain, "", x, y, Direction::Exact) && oracle(domain, "", y, x, Direction::Exact)
}

#[test]
fn a_provider_reads_the_same_in_every_order_of_its_offers() {
    // R18 1: an overflowing comparison first made every later one overflow, so two plain values beside it went
    // uncompared.
    const TINY: &str = "0.0000000000000000000000000000001 ns";
    let mut providers = 0usize;
    for e in VOCABULARY.iter().filter(|e| e.role == Role::Guarantee) {
        let mut pool: Vec<String> = samples(e.domain).into_iter().take(4).collect();
        if e.domain == Domain::Quantity(Dimension::Time) {
            pool.push(TINY.to_owned());
        }
        let readable: Vec<String> = pool
            .into_iter()
            .filter(|v| provider(&format!("(offers ({} {v}))", e.name)).is_ok())
            .collect();
        let probes: Vec<Requirement> = readable
            .iter()
            .filter_map(|v| constraint(&format!("({} (exactly {v}))", e.name)).ok())
            .collect();
        let reading = |offers: &[&String; 3]| -> Option<Vec<Verdict>> {
            let text: String = offers
                .iter()
                .map(|v| format!(" ({} {v})", e.name))
                .collect();
            provider(&format!("(offers{text})"))
                .ok()
                .map(|p| probes.iter().map(|r| judge(&p, r)).collect())
        };
        for a in &readable {
            for b in &readable {
                for c in &readable {
                    let orders = [
                        [a, b, c],
                        [a, c, b],
                        [b, a, c],
                        [b, c, a],
                        [c, a, b],
                        [c, b, a],
                    ];
                    let first = reading(&orders[0]);
                    for o in &orders[1..] {
                        assert_eq!(
                            reading(o),
                            first,
                            "{}: {a}, {b}, {c} read otherwise as {o:?}",
                            e.name
                        );
                    }
                    let plain: Vec<&String> = [a, b, c]
                        .into_iter()
                        .filter(|v| v.as_str() != TINY)
                        .collect();
                    let unequal = plain.iter().enumerate().any(|(i, x)| {
                        plain[i + 1..]
                            .iter()
                            .any(|y| !same_by_oracle(e.domain, x, y))
                    });
                    if unequal {
                        assert!(
                            first.is_none(),
                            "{}: {a}, {b}, {c} hold two values and were read",
                            e.name
                        );
                    }
                    providers += 1;
                }
            }
        }
    }
    println!("read {providers} three-offer providers in all six orders");
}

#[test]
fn absent_names_a_fact_by_name_alone() {
    // R18 2: `(absent (available-in-state sleep))` read as the whole fact absent.
    let mut written = 0usize;
    for e in VOCABULARY.iter().filter(|e| e.role == Role::Guarantee) {
        assert!(
            provider(&format!("(absent {})", e.name)).is_ok(),
            "{}",
            e.name
        );
        assert!(
            provider(&format!("(absent ({}))", e.name)).is_err(),
            "{}",
            e.name
        );
        for v in samples(e.domain) {
            assert!(
                provider(&format!("(absent ({} {v}))", e.name)).is_err(),
                "{}: `{v}` inside `absent`",
                e.name
            );
            written += 1;
        }
    }
    println!("refused {written} values written inside `absent`");
}

#[test]
fn a_modulus_above_its_width_is_refused_at_every_width() {
    // R18 9: §4's `2^width` bound held beyond 32 bits, to the arithmetic's last width.
    let mut pairs = 0usize;
    for w in [1u32, 8, 16, 32, 63, 64, 125, 126, 127] {
        let bound = 1u128 << w;
        for m in [
            "1",
            "255",
            "256",
            "4294967296",
            "(pow2 64)",
            "(pow2 125)",
            "(pow2 126)",
        ] {
            let mv: u128 = m
                .strip_prefix("(pow2 ")
                .and_then(|t| t.strip_suffix(')'))
                .map_or_else(
                    || m.parse().expect("an integer"),
                    |n| 1u128 << n.parse::<u32>().expect("N"),
                );
            let p = provider(&format!(
                "(offers (counter-width {w} bit) (counter-modulus {m}))"
            ));
            assert_eq!(p.is_err(), mv > bound, "width {w} bit, modulus {m}");
            pairs += 1;
        }
    }
    println!("checked {pairs} width and modulus pairs");
}

#[test]
fn writing_an_offer_twice_changes_nothing() {
    // R19 1: an overflowing value written twice failed a presence requirement it met written once.
    const TINY: &str = "0.0000000000000000000000000000001 ns";
    let mut providers = 0usize;
    for e in VOCABULARY.iter().filter(|e| e.role == Role::Guarantee) {
        let mut pool = samples(e.domain);
        if e.domain == Domain::Quantity(Dimension::Time) {
            pool.push(TINY.to_owned());
        }
        let mut probes: Vec<Requirement> = pool
            .iter()
            .filter_map(|v| constraint(&format!("({} (exactly {v}))", e.name)).ok())
            .collect();
        probes.push(read_needs(&forms(e.name)[0]).expect("a vocabulary fact"));
        let reading = |offers: &str| {
            provider(&format!("(offers {offers})"))
                .ok()
                .map(|p| probes.iter().map(|r| judge(&p, r)).collect::<Vec<_>>())
        };
        for v in &pool {
            let once = format!("({} {v})", e.name);
            assert_eq!(
                reading(&once),
                reading(&format!("{once} {once}")),
                "{once} twice"
            );
            providers += 1;
        }
    }
    println!("read {providers} offers once and twice");
}

#[test]
fn a_clause_holds_one_value_per_equality() {
    // R19 2: a statement written both ways passed; two equalities on one fact are one value or a contradiction.
    let mut clauses = 0usize;
    for e in VOCABULARY
        .iter()
        .filter(|e| e.direction == Direction::Exact)
    {
        let pool = samples(e.domain);
        for a in &pool {
            for b in &pool {
                for (x, y) in [
                    (format!("({} {a})", e.name), format!("({} {b})", e.name)),
                    (
                        format!("({} {a})", e.name),
                        format!("({} (exactly {b}))", e.name),
                    ),
                ] {
                    let (Ok(_), Ok(_)) = (constraint(&x), constraint(&y)) else {
                        continue;
                    };
                    let read = read_clause(&forms(&format!("(requires {x} {y})"))[0]);
                    let same = same_by_oracle(e.domain, a, b);
                    assert_eq!(read.is_ok(), same, "{x} beside {y}");
                    clauses += 1;
                }
            }
        }
    }
    println!("read {clauses} clauses of two equalities on one fact");
}

#[test]
fn an_offer_under_exactly_reads_as_its_value() {
    // R19 remark 9: the oracle judged plain `(f v)` offers only; `(f (exactly v))` is that value (§1.1; R10 J8).
    let mut offers = 0usize;
    for e in VOCABULARY.iter().filter(|e| e.role == Role::Guarantee) {
        let pool = samples(e.domain);
        let mut probes: Vec<Requirement> = pool
            .iter()
            .filter_map(|v| constraint(&format!("({} {v})", e.name)).ok())
            .collect();
        probes.push(read_needs(&forms(e.name)[0]).expect("a vocabulary fact"));
        for v in &pool {
            let (Ok(plain), Ok(exact)) = (
                provider(&format!("(offers ({} {v}))", e.name)),
                provider(&format!("(offers ({} (exactly {v})))", e.name)),
            ) else {
                continue;
            };
            for r in &probes {
                assert_eq!(
                    judge(&plain, r),
                    judge(&exact, r),
                    "{}: `{v}` under `exactly`",
                    e.name
                );
            }
            offers += 1;
        }
    }
    println!("judged {offers} offers under `exactly` as their values");
}
