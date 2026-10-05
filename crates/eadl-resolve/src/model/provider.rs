//! A provider — a block or a platform — read into one state per fact, with §8's refusals of what it offers.

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::Form;

use super::value::{self, same, Value};
use super::vocab::{self, Direction, Domain, Role};

/// What a provider states of one fact (record §1's definitions, §2).
#[derive(Debug, Clone)]
pub enum Stated {
    /// One value — written, `(f (exactly v))`, or a bare boolean read as `true` — with every spelling of it written, in
    /// written order, never empty: `10 MHz` beside `10000 kHz` is one value in two spellings (§5; R25 1).
    Valued(Vec<Value>),
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
    /// A value whose endpoints' order is past the exact arithmetic (§2, R18 7).
    Overflow,
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
                        .map(written)
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
        .map(written)
        .or_else(|m| refuse(e.name, m.0))
}

/// A value read, or §2's overflow when its interval's endpoints cannot be ordered (R18 7).
fn written(v: Value) -> Written {
    match value::interval_order(&v) {
        Ok(()) => Written::Value(v),
        Err(_) => Written::Overflow,
    }
}

/// The name an item of `offers` or `absent` writes — itself, or a list's head — or `None` when it names nothing.
fn item_name(item: &Form) -> Option<&str> {
    match item {
        Form::Symbol { name, .. } => Some(name.as_str()),
        _ => item.head(),
    }
}

/// One item of `offers`, read and judged as §8 judges a value wherever written: refused when it names nothing (R23 3)
/// or offers a statement fact (§3 rule 6); `None` for a name the vocabulary does not declare, which has no domain to
/// contradict (§5; R1 A6).
fn offered(item: &Form) -> Result<Option<(&'static vocab::Entry, Written)>, Refused> {
    let Some(head) = item_name(item) else {
        return refuse("offers", "an item of `offers` that names nothing — a number, a string, `()`, a list headed by none");
    };
    let Some(e) = vocab::entry(head) else {
        return Ok(None);
    };
    if e.role == Role::Statement {
        return refuse(e.name, "an offer of a statement fact (§3 rule 6)");
    }
    classify(e, item).map(|w| Some((e, w)))
}

/// One item of `absent`: refused when it names nothing (R23 3), declares a statement fact absent — the requiring
/// side's word alone (§3 rule 6; R17 R4) — or writes a value, which would read as absence of the whole fact, as a list
/// inside `needs` would (§8; R18 2).
fn declared_absent(item: &Form) -> Result<Option<&'static vocab::Entry>, Refused> {
    let Some(head) = item_name(item) else {
        return refuse("absent", "an item of `absent` that names nothing");
    };
    let Some(e) = vocab::entry(head) else {
        return Ok(None);
    };
    if e.role == Role::Statement {
        return refuse(e.name, "a statement fact declared absent");
    }
    if matches!(item, Form::List { .. }) {
        return refuse(e.name, "a list inside `absent` naming a vocabulary fact");
    }
    Ok(Some(e))
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
                    if let Some((e, w)) = offered(item)? {
                        written.entry(e.name.to_string()).or_default().push(w);
                    }
                }
            }
            Some("absent") => {
                for item in &clause.items()[1..] {
                    if let Some(e) = declared_absent(item)? {
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
        let mut values: Vec<Value> = Vec::new();
        let mut overflow = false;
        let mut bound = false;
        let mut bare = false;
        for w in ws {
            match w {
                Written::Bare if boolean => values.push(Value::Bool(true)),
                Written::Bare => bare = true,
                Written::Bound => bound = true,
                Written::Overflow => overflow = true,
                Written::Value(v) => values.push(v.clone()),
            }
        }
        // Every pair compared: two offers unequal without overflowing are two values, whatever the others and their
        // order; otherwise a comparison that overflows makes the fact §2's `unsupported-profile` (§5; R17 5, R18 1).
        for (i, a) in values.iter().enumerate() {
            for b in &values[i + 1..] {
                match same(a, b) {
                    Ok(true) => {}
                    Ok(false) => {
                        return refuse(
                            fact,
                            "one provider offering one declared fact with two values",
                        )
                    }
                    Err(_) => overflow = true,
                }
            }
        }
        // Every pair being the same value, every spelling is kept: a comparison is decided by any of them whose
        // arithmetic does not overflow, so no spelling's place in the text decides it (§5; R25 1).
        let value = (!values.is_empty()).then_some(values);
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
    // refused as values, wherever written (`value::fact_value`). The first spelling of each: spellings in two units
    // were compared in the base unit as the provider was read, so each converts to the one value (§5; R25 1).
    if let (Some(Stated::Valued(ws)), Some(Stated::Valued(ms))) =
        (stated.get("counter-width"), stated.get("counter-modulus"))
    {
        if let (Some(Value::Quantity(w)), Some(Value::Count(m))) = (ws.first(), ms.first()) {
            if let Ok(b) = w.in_base() {
                let width = b.numerator();
                if width < 127 && *m > (1i128 << width) {
                    return refuse(
                        "counter-modulus",
                        "a `counter-modulus` above a valued `2^width`",
                    );
                }
            }
        }
    }
    if matches!(
        stated.get("counter-modulus"),
        Some(Stated::Valued(_) | Stated::Unvalued | Stated::Overflow)
    ) && matches!(stated.get("wrap-behavior"), Some(Stated::Valued(ws)) if matches!(ws.first(), Some(Value::Enum(w)) if w == "saturating"))
    {
        return refuse(
            "counter-modulus",
            "a `counter-modulus` beside `(wrap-behavior saturating)`",
        );
    }
    Ok(Provider { name, stated })
}

/// Read a service's `offers` and `absent` as a provider's are read. A service is no provider: its offers are judged
/// against no requirement in `/1` (record §1; R3 C13). But a declaration offering one fact with two values contradicts
/// itself whoever writes it, and `ROADMAP.md` §5.3 rejects contradictory declarations rather than choosing one, so §8
/// refuses in a service all it refuses in a provider's offers and absences (R24 4; R22 2 and R23 2 withdrawn).
///
/// # Errors
///
/// [`Refused`] — `invalid-description` — as [`read`].
pub fn read_service(decl: &Form) -> Result<(), Refused> {
    read(decl).map(|_| ())
}
