//! A provider — a block or a platform — read into one state per fact, with §8's refusals of what it offers.

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::Form;

use super::value::{self, same, Value};
use super::vocab::{self, Direction, Domain, Role};

/// What a provider states of one fact (record §1's definitions, §2).
#[derive(Debug, Clone)]
pub enum Stated {
    /// A value — written, `(f (exactly v))`, or a bare boolean read as `true`.
    Valued(Value),
    /// Offered with no value: bare in a domain where bare has none, or only an abstract platform's bound.
    Unvalued,
    /// Declared absent.
    Absent,
    /// The same value offered twice, where comparing the two overflows: §2's `unsupported-profile` (R17 5).
    Overflow,
}

/// A provider read.
#[derive(Debug, Clone)]
pub struct Provider {
    /// The declaration's name.
    pub name: String,
    /// What it states of each vocabulary fact it names. A fact it does not name is not here.
    pub stated: BTreeMap<String, Stated>,
}

/// Why a provider is refused: record §8's first row, `invalid-description`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refused {
    /// The fact the refusal is about.
    pub fact: String,
    /// Which cause of §8's row.
    pub why: String,
}

fn refuse<T>(fact: &str, why: impl Into<String>) -> Result<T, Refused> {
    Err(Refused {
        fact: fact.to_string(),
        why: why.into(),
    })
}

/// One written offer of a fact, classified.
#[derive(Debug, Clone)]
enum Written {
    Bare,
    Value(Value),
    Bound,
}

fn direction_word(word: &str) -> Option<Direction> {
    Some(match word {
        "at-least" => Direction::AtLeast,
        "at-most" => Direction::AtMost,
        "exactly" | "exact" => Direction::Exact,
        "includes" => Direction::Includes,
        "within" => Direction::Within,
        _ => return None,
    })
}

/// Classify one item of an `offers` clause whose head is a vocabulary fact (record §1, §1.1, §2).
fn classify(e: &vocab::Entry, item: &Form) -> Result<Written, Refused> {
    let rest: &[Form] = match item {
        Form::Symbol { .. } => return Ok(Written::Bare),
        Form::List { items, .. } => &items[1..],
        _ => return refuse(e.name, "an offer is a name or a list"),
    };
    if rest.is_empty() {
        // `(f)` with nothing after it is a bare offer (§2's set row; R12 L12).
        return Ok(Written::Bare);
    }
    if let [wrapper @ Form::List { .. }] = rest {
        if let Some(word) = wrapper.head() {
            if let Some(dir) = direction_word(word) {
                let inner = &wrapper.items()[1..];
                if word == "exactly" {
                    // In an offer, `(f (exactly v))` is the value `v` (§1.1; R10 J8).
                    if inner.is_empty() {
                        return refuse(e.name, "`(f (exactly))` writes no value");
                    }
                    return value::read(e.domain, inner, item.span())
                        .and_then(|v| value::fact_value(e.name, e.domain, &v).map(|()| v))
                        .map(Written::Value)
                        .or_else(|m| refuse(e.name, m.0));
                }
                if word != "at-least" && word != "at-most" {
                    return refuse(e.name, format!("`{word}` is not a written direction; a bound writes `at-least`, `at-most` or `exactly`"));
                }
                if matches!(e.domain, Domain::Boolean | Domain::Group(_)) {
                    return refuse(
                        e.name,
                        "a boolean or group head offered with a bound other than `exactly`",
                    );
                }
                if dir != e.direction {
                    return refuse(
                        e.name,
                        format!("a bound written `{word}` against the fact's direction"),
                    );
                }
                // An abstract platform's bound: refinement's, not a value (§2; R10 J4, J8). Its value must still be
                // of the domain.
                return value::read(e.domain, inner, item.span())
                    .and_then(|v| value::fact_value(e.name, e.domain, &v))
                    .map(|()| Written::Bound)
                    .or_else(|m| refuse(e.name, m.0));
            }
        }
    }
    value::read(e.domain, rest, item.span())
        .and_then(|v| value::fact_value(e.name, e.domain, &v).map(|()| v))
        .map(Written::Value)
        .or_else(|m| refuse(e.name, m.0))
}

/// Read a block or platform declaration, refusing what record §8 refuses of an offer.
///
/// # Errors
///
/// [`Refused`] — `invalid-description` — for the first cause of §8's first row the provider meets, in the order
/// rule 1 judges them before anything else.
pub fn read(decl: &Form) -> Result<Provider, Refused> {
    let name = decl
        .items()
        .get(1)
        .and_then(Form::as_symbol)
        .unwrap_or("<unnamed>")
        .to_string();
    if vocab::entry(&name).is_some() {
        // A declaration whose local name is a vocabulary fact (§1.1; R16 1), a provider's own name included (R17 R5).
        return refuse(&name, "a declaration whose local name is a vocabulary fact");
    }
    let mut written: BTreeMap<String, Vec<Written>> = BTreeMap::new();
    let mut absent: BTreeSet<String> = BTreeSet::new();
    for clause in decl.items().iter().skip(2) {
        match clause.head() {
            Some("offers") => {
                for item in &clause.items()[1..] {
                    let head = match item {
                        Form::Symbol { name, .. } => name.as_str(),
                        _ => item.head().unwrap_or(""),
                    };
                    let Some(e) = vocab::entry(head) else {
                        continue; // an undeclared fact has no domain to contradict (§5; R1 A6)
                    };
                    if e.role == Role::Statement {
                        return refuse(e.name, "an offer of a statement fact (§3 rule 6)");
                    }
                    let w = classify(e, item)?;
                    written.entry(e.name.to_string()).or_default().push(w);
                }
            }
            Some("absent") => {
                for item in &clause.items()[1..] {
                    let head = match item {
                        Form::Symbol { name, .. } => name.as_str(),
                        _ => item.head().unwrap_or(""),
                    };
                    if let Some(e) = vocab::entry(head) {
                        if e.role == Role::Statement {
                            // A statement is the requiring side's word alone (§3 rule 6; R17 R4).
                            return refuse(e.name, "a statement fact declared absent");
                        }
                        absent.insert(e.name.to_string());
                    }
                }
            }
            _ => {}
        }
    }
    // A fact offered and declared absent (§8; model §2 rule 1).
    for fact in &absent {
        if written.contains_key(fact) {
            return refuse(fact, "a provider offering and declaring absent one fact");
        }
    }
    let mut stated: BTreeMap<String, Stated> = BTreeMap::new();
    for (fact, ws) in &written {
        let e = vocab::entry(fact).expect("only vocabulary facts are kept");
        let boolean = matches!(e.domain, Domain::Boolean | Domain::Group(_));
        let mut value: Option<Value> = None;
        let mut overflow = false;
        let mut bound = false;
        let mut bare = false;
        for w in ws {
            let v = match w {
                Written::Bare if boolean => Value::Bool(true),
                Written::Bare => {
                    bare = true;
                    continue;
                }
                Written::Bound => {
                    bound = true;
                    continue;
                }
                Written::Value(v) => v.clone(),
            };
            match &value {
                None => value = Some(v),
                Some(prev) => match same(prev, &v) {
                    Ok(true) => {}
                    Ok(false) => {
                        return refuse(
                            fact,
                            "one provider offering one declared fact with two values",
                        )
                    }
                    Err(_) => overflow = true,
                },
            }
        }
        let _ = bare; // a bare offer beside a value is that value (§5; R13 M7)
        if bound && value.is_some() {
            return refuse(fact, "a bound beside a value of the same fact");
        }
        let state = if overflow {
            Stated::Overflow
        } else {
            value.map_or(Stated::Unvalued, Stated::Valued)
        };
        stated.insert(fact.clone(), state);
    }
    for fact in &absent {
        stated.insert(fact.clone(), Stated::Absent);
    }
    // A derived fact beside a fact its rule reads (§4; R4 D1, R5 E1, R6 F1; absent too, R16 8).
    for e in vocab::VOCABULARY.iter().filter(|e| e.rule.is_some()) {
        let inputs_offered = e.derived_from.iter().chain(e.reads.iter()).any(|i| {
            matches!(
                stated.get(*i),
                Some(Stated::Valued(_) | Stated::Unvalued | Stated::Overflow)
            )
        });
        match stated.get(e.name) {
            Some(Stated::Valued(_) | Stated::Unvalued | Stated::Overflow) if inputs_offered => {
                return refuse(
                    e.name,
                    "a derived fact offered beside a fact its rule reads",
                );
            }
            Some(Stated::Absent) if inputs_offered => {
                return refuse(
                    e.name,
                    "a derived fact declared absent beside a fact its rule reads",
                );
            }
            _ => {}
        }
    }
    // The counter's own refusals (§4; R3 C12, C6); a modulus of 0 and a width that is no whole number of bits are
    // refused as values, wherever written (`value::fact_value`).
    if let Some(Stated::Valued(Value::Quantity(w))) = stated.get("counter-width") {
        if let (Ok(b), Some(Stated::Valued(Value::Count(m)))) =
            (w.in_base(), stated.get("counter-modulus"))
        {
            let width = b.numerator();
            if width < 127 && *m > (1i128 << width) {
                return refuse(
                    "counter-modulus",
                    "a `counter-modulus` above a valued `2^width`",
                );
            }
        }
    }
    if matches!(
        stated.get("counter-modulus"),
        Some(Stated::Valued(_) | Stated::Unvalued | Stated::Overflow)
    ) && matches!(stated.get("wrap-behavior"), Some(Stated::Valued(Value::Enum(w))) if w == "saturating")
    {
        return refuse(
            "counter-modulus",
            "a `counter-modulus` beside `(wrap-behavior saturating)`",
        );
    }
    Ok(Provider { name, stated })
}
