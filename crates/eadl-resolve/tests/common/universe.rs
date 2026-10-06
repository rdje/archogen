//! The bounded universe: values on both sides of every bound, every written form of an offer, and the provider
//! families `tests/checker.rs` enumerates (leaf `M3.1.1.1`), defined once for the model's checker and for the
//! production relation's comparison with the model (`tests/production.rs`, `SR-H7`).

use eadl_model::Dimension;
use eadl_resolve::model::{Direction, Domain, Entry, Role, VOCABULARY};

/// An amount whose conversion to another unit overflows the exact arithmetic (R18 1, R19 1, R25 1).
pub const TINY: &str = "0.0000000000000000000000000000001 ns";

/// Values a fact's domain is sampled at, each written as an offer or a requirement would write it.
pub fn samples(domain: Domain) -> Vec<String> {
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
        Domain::Quantity(Dimension::Frequency) => s(&[
            "1 Hz",
            "18 Hz",
            "1 MHz",
            "10 MHz",
            "10000 kHz",
            "20 MHz",
            "1 GHz",
        ]),
        Domain::Quantity(Dimension::Information) => s(&[
            "8 bit", "16 bit", "32 bit", "4 byte", "64 bit", "0.5 byte", "1 KiB",
        ]),
        Domain::Quantity(Dimension::Dimensionless) => s(&["1 tick", "8 tick"]),
        Domain::Interval(_) => s(&[
            "(range 1 MHz 200 MHz)",
            "(range 10 MHz 10 MHz)",
            "(range 50 MHz 300 MHz)",
            "10 MHz",
            "250 MHz",
            // R26 remark 4: endpoints in two units, compared in the base unit (§2, §5).
            "(range 1000 kHz 0.2 GHz)",
            "(range 1 MHz 200000 kHz)",
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

/// Values just past a domain's edge — an alternative the entry does not list, a member twice, a negative or
/// fractional count, a count in a unit, a power past 126, another dimension, an interval turned round (R30 remark 2).
pub fn outside(domain: Domain) -> Vec<String> {
    match domain {
        Domain::Boolean | Domain::Group(_) => {
            vec!["maybe".into(), "1".into(), "true false".into()]
        }
        Domain::Count => vec![
            "-1".into(),
            "1.5".into(),
            "8 bit".into(),
            "(pow2 -1)".into(),
            "(pow2 127)".into(),
        ],
        Domain::Quantity(d) => {
            let other = if d == Dimension::Time { "8 bit" } else { "8 s" };
            let mut past = vec![other.into(), "5".into(), "zzz".into()];
            if d == Dimension::Information {
                // Not a positive whole number of bits (record §4, §8; R17 6): a half, and none.
                past.extend(["0.5 bit".into(), "0 bit".into()]);
            }
            past
        }
        Domain::Interval(_) => vec![
            "(range 10 s 20 s)".into(),
            "(range 200 MHz 1 MHz)".into(),
            "8 s".into(),
        ],
        Domain::Enumeration { alternatives, .. } => vec![
            "zzz".into(),
            format!("{} {}", alternatives[0], alternatives[1]),
        ],
        Domain::Set(members) => vec![
            "zzz".into(),
            format!("{} zzz", members[0]),
            format!("{} {}", members[0], members[0]),
        ],
    }
}

/// Every written form of one fact's offer: bare, valued, `exactly`, bound, absent, twice, beside its absence; a
/// direction's name as a wrapper; a bound against the fact's direction (R23 7, R30 3).
pub fn written_offers(e: &Entry) -> Vec<String> {
    let f = e.name;
    let mut offers = vec![
        format!("(offers {f})"),
        format!("(offers ({f}))"),
        format!("(absent {f})"),
        String::new(),
        // A list with no head where a value or a bound would stand (the shape `M3.1.2.4`'s universe found crashing).
        format!("(offers ({f} ()))"),
        format!("(offers ({f} (exactly ())))"),
    ];
    let own = match e.direction {
        Direction::AtLeast => Some("at-least"),
        Direction::AtMost => Some("at-most"),
        _ => None,
    };
    for v in samples(e.domain) {
        offers.push(format!("(offers ({f} {v}))"));
        offers.push(format!("(offers ({f} (exactly {v})))"));
        offers.push(format!("(offers ({f} (at-least {v})))"));
        offers.push(format!("(offers ({f} (at-most {v})))"));
        offers.push(format!("(offers {f} ({f} {v}))"));
        offers.push(format!("(offers ({f} {v}) ({f} {v}))"));
        offers.push(format!("(offers ({f} {v})) (absent {f})"));
        for wrapper in ["includes", "within", "exact"] {
            offers.push(format!("(offers ({f} ({wrapper} {v})))"));
        }
        for bound in ["at-least", "at-most"] {
            if Some(bound) != own {
                offers.push(format!("(offers ({f} ({bound} {v})))"));
            }
        }
    }
    offers
}

/// Operands and items that name nothing, and lists that would drop what follows their head (R23 3, R26 1).
pub const NAMES_NOTHING: &[&str] = &["5", "\"t\"", "()", "(5 6)", "((uart))"];

/// Lists written where a name stands (R26 1).
pub const LISTS_FOR_NAMES: &[&str] = &[
    "(time.monotonic (tick-unit us))",
    "(time.monotonic (requires (tick-unit us)))",
    "(time.monotonic)",
    "(uart)",
    "(foo 5)",
];

/// The body of every provider declaration the universe holds: what follows `(defblock p` in each.
pub fn provider_bodies() -> Vec<String> {
    let mut out = Vec::new();
    // Every written form of every fact's offer.
    for e in VOCABULARY {
        out.extend(written_offers(e));
    }
    let guarantees = || VOCABULARY.iter().filter(|e| e.role == Role::Guarantee);
    for e in guarantees() {
        let f = e.name;
        // Three offers in all six orders, an amount past the arithmetic among them where the domain is time (R18 1,
        // R25 1).
        let mut pool: Vec<String> = samples(e.domain).into_iter().take(4).collect();
        if e.domain == Domain::Quantity(Dimension::Time) {
            pool.extend(["1 ms", "1000000 ns", "0.001 s", TINY].map(str::to_owned));
        }
        for a in &pool {
            for b in &pool {
                for c in &pool {
                    for [x, y, z] in [
                        [a, b, c],
                        [a, c, b],
                        [b, a, c],
                        [b, c, a],
                        [c, a, b],
                        [c, b, a],
                    ] {
                        out.push(format!("(offers ({f} {x}) ({f} {y}) ({f} {z}))"));
                    }
                }
            }
        }
        // A value written twice, the amount past the arithmetic too (R19 1).
        for v in &pool {
            out.push(format!("(offers ({f} {v}) ({f} {v}))"));
        }
        // `absent` names a fact by its name alone (R18 2).
        out.push(format!("(absent ({f}))"));
        for v in samples(e.domain) {
            out.push(format!("(absent ({f} {v}))"));
        }
        // A value outside its domain, offered and under `exactly` (R30 2).
        for v in outside(e.domain) {
            out.push(format!("(offers ({f} {v}))"));
            out.push(format!("(offers ({f} (exactly {v})))"));
        }
        // An abstract platform's bound beside a value of the same fact, in either order, beside a bare offer and
        // beside a second bound (record §2, §5, R10 J8) — the family `M3.1.2.3`'s mutation of that rule found missing.
        let own = match e.direction {
            Direction::AtLeast => Some("at-least"),
            Direction::AtMost => Some("at-most"),
            _ => None,
        };
        if let Some(own) = own {
            for v in samples(e.domain).into_iter().take(3) {
                out.push(format!("(offers ({f} ({own} {v})) ({f} {v}))"));
                out.push(format!("(offers ({f} {v}) ({f} ({own} {v})))"));
                out.push(format!("(offers {f} ({f} ({own} {v})))"));
                out.push(format!("(offers ({f} ({own} {v})) ({f} ({own} {v})))"));
            }
        }
    }
    // The modulus bounded by its width at every width, to 127 bits (R18 9).
    for w in [1u32, 8, 16, 32, 63, 64, 125, 126, 127] {
        for m in [
            "1",
            "255",
            "256",
            "4294967296",
            "(pow2 64)",
            "(pow2 125)",
            "(pow2 126)",
        ] {
            out.push(format!(
                "(offers (counter-width {w} bit) (counter-modulus {m}))"
            ));
        }
    }
    // Every state of the horizon and its inputs: none, bare, valued, absent (R29 4).
    let horizon = [
        "",
        "unambiguous-horizon",
        "(unambiguous-horizon 60 s)",
        "!unambiguous-horizon",
    ];
    let modulus = [
        "",
        "counter-modulus",
        "(counter-modulus 4294967296)",
        "(counter-modulus 65536)",
        "!counter-modulus",
    ];
    let rate = ["", "tick-rate", "(tick-rate 10 MHz)", "!tick-rate"];
    let wrap = [
        "",
        "wrap-behavior",
        "(wrap-behavior modular)",
        "(wrap-behavior saturating)",
        "!wrap-behavior",
    ];
    for h in horizon {
        for m in modulus {
            for r in rate {
                for w in wrap {
                    let parts = [h, m, r, w];
                    let offered: Vec<&str> = parts
                        .iter()
                        .copied()
                        .filter(|p| !p.is_empty() && !p.starts_with('!'))
                        .collect();
                    let absent: Vec<&str> =
                        parts.iter().filter_map(|p| p.strip_prefix('!')).collect();
                    let mut body = String::new();
                    if !offered.is_empty() {
                        body.push_str(&format!("(offers {})", offered.join(" ")));
                    }
                    if !absent.is_empty() {
                        body.push_str(&format!(" (absent {})", absent.join(" ")));
                    }
                    out.push(body);
                }
            }
        }
    }
    // Items that name nothing; lists inside `absent` (R23 3, R26 1).
    for n in NAMES_NOTHING {
        out.push(format!("(offers {n})"));
        out.push(format!("(absent {n})"));
    }
    for l in LISTS_FOR_NAMES {
        out.push(format!("(absent {l})"));
    }
    // A clause written as an offer, or inside one at any depth (R27 2, R28 3).
    for clause in [
        "(offers (requires (tick-unit us)))",
        "(offers (counter-width (requires (tick-unit us))))",
        "(offers (counter-width 32 bit (needs uart)))",
        "(offers (uart (platform (uses x))))",
        "(offers (refines soc.abstract))",
        "(offers (region (offers uart)))",
        "(offers (deadline 10 ms))",
    ] {
        out.push(clause.to_string());
    }
    // Names the vocabulary does not declare, beside declared ones: presence's, never refused here (R1 A6).
    out.push("(offers (region a) (region b) (p 1 bit) uart)".to_string());
    out
}

/// Placements of a `needs` or `uses` list, `X`, in a service — the positions presence reads, and those §1's grammar
/// refuses (R23 1, R24 1, R24 3, R29 1).
pub const PLACES: &[&str] = &[
    "X",
    "(requires X)",
    "(requires (tick-unit ns) X)",
    "(requires (ordering (before a b) X))",
    "(platform X)",
    "(platform (uses soc.p) X)",
    "(task t (period 10 ms) X)",
    "(somewhere (deeper X))",
    "(offers X)",
    "(absent X)",
    "(refines X)",
    "((X))",
    "(5 X)",
    "(\"s\" X)",
    "(platform (offers X))",
    "(platform (uses soc.p) (absent X))",
    "(platform (tick-unit ns) X)",
    "(requires (offers X))",
    "(requires (refines X))",
];

/// The placements among [`PLACES`] that §1's grammar refuses: anything but `needs`, `uses` and `requires` in
/// `platform`, a provider's clause inside `requires` (R29 1).
pub const REFUSED_PLACES: &[&str] = &[
    "(platform (offers X))",
    "(platform (uses soc.p) (absent X))",
    "(platform (tick-unit ns) X)",
    "(requires (offers X))",
    "(requires (refines X))",
];

/// The items a clause's code is read in every order of (R21 2).
pub const ORDER_POOL: &[&str] = &[
    "(ordering (before a b))",
    "(tick-unit ns)",
    "(tick-unit us)",
    "(or-through-mediation allowed)",
    "(or-through-mediation forbidden)",
    "(uses time.monotonic)",
    "(uses observation-coherent)",
    "(needs time.monotonic)",
    "something",
    "(counter-width (at-least 16 bit))",
];

/// One value in several spellings per row, some past the arithmetic in another unit (R27 1, R28 1).
pub const RESPELT: &[&[&str]] = &[
    &["1 ms", "1000000 ns", "0.001 s"],
    &[
        "0.000000000001 ns",
        "0.000000000000000000001 s",
        "0.000000000000001 us",
    ],
    &["2 ms", "2000 us", "0.002 s"],
    &[
        "0.0000000000000000000000000000001 ns",
        "0.0000000000000000000000000000000001 us",
    ],
];

/// Bounds beside [`RESPELT`]'s equalities.
pub const RESPELT_BOUNDS: &[&str] = &[
    "1 ns",
    "1 ms",
    "5 ms",
    "0.5 ms",
    "0.000000000009000000000000000001 ns",
    "0.0000000000005 ns",
    "0.0000000000000000000000000000001 ns",
];

/// The atoms of the grammar of small forms every item of `requires` is read over (R22 1).
pub const ATOMS: &[&str] = &["uart", "something", "5", "\"t\"", "()", "true"];

/// Every small item of `requires`: an atom, a list of one or two atoms, a list headed by a list (R22 1).
pub fn small_items() -> Vec<String> {
    let mut items: Vec<String> = ATOMS.iter().map(|a| (*a).to_string()).collect();
    for a in ATOMS {
        items.push(format!("({a})"));
        for b in ATOMS {
            items.push(format!("({a} {b})"));
        }
    }
    items.push("((uart))".to_owned());
    items
}

/// Every constraint the universe writes: each fact's sampled and out-of-domain values in every written form,
/// a direction's name as a wrapper, a bound with no value, the group's spellings and the item grammar.
pub fn constraint_texts() -> Vec<String> {
    let mut out = Vec::new();
    for e in VOCABULARY {
        let f = e.name;
        out.push(f.to_string());
        out.push(format!("({f})"));
        out.push(format!("({f} (at-least))"));
        out.push(format!("({f} (exactly))"));
        for v in samples(e.domain).into_iter().chain(outside(e.domain)) {
            for written in [
                format!("({f} {v})"),
                format!("({f} (exactly {v}))"),
                format!("({f} (at-least {v}))"),
                format!("({f} (at-most {v}))"),
                format!("({f} (includes {v}))"),
                format!("({f} (within {v}))"),
                format!("({f} (exact {v}))"),
            ] {
                out.push(written);
            }
        }
    }
    for group in [
        "(absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 50 us)))",
        "(absolute-deadline (delivery-bound (at-most 50 us)) (supported-horizon (at-least 10 s)))",
        "(absolute-deadline (supported-horizon 3600 s))",
        "(absolute-deadline true (supported-horizon (at-least 10 s)))",
        "(absolute-deadline (supported-horizon (at-least 10 s)) (tick-unit ns))",
        "(absolute-deadline (supported-horizon 5))",
        "(absolute-deadline (supported-horizon (at-most 5 s)))",
        "(absolute-deadline (supported-horizon (offers x)))",
        "(absolute-deadline (delivery-bound (exactly)))",
    ] {
        out.push(group.to_string());
    }
    // An interval whose endpoints cannot be ordered within the arithmetic (R18 7).
    out.push("(frequency (range 0.0000000000000000000000000000001 Hz 1 MHz))".to_string());
    out.push(
        "(frequency (exactly (range 0.0000000000000000000000000000001 Hz 1 MHz)))".to_string(),
    );
    out.extend(small_items());
    out
}

/// Every `requires` clause the universe writes.
pub fn clause_texts() -> Vec<String> {
    let mut out = Vec::new();
    // Two equalities on one fact, a `uses` and a service's `needs` beside them (R19 2, R20 1).
    for e in VOCABULARY
        .iter()
        .filter(|e| e.direction == Direction::Exact)
    {
        let pool = samples(e.domain);
        for a in &pool {
            for b in &pool {
                let f = e.name;
                out.push(format!(
                    "(requires (uses timer.counter) (needs time.monotonic) ({f} {a}) ({f} {b}))"
                ));
                out.push(format!("(requires (uses timer.counter) (needs time.monotonic) ({f} {a}) ({f} (exactly {b})))"));
            }
            if matches!(e.domain, Domain::Boolean | Domain::Group(_)) {
                out.push(format!("(requires ({} {a}) (needs {}))", e.name, e.name));
            }
        }
    }
    // At the arithmetic's edge: one unit compared as written, two whose conversion overflows (R20 2).
    let tiny = [TINY, "0.0000000000000000000000000000002 ns", "1 s", "1 ms"];
    for e in VOCABULARY
        .iter()
        .filter(|e| e.domain == Domain::Quantity(Dimension::Time))
    {
        let bound = match e.direction {
            Direction::AtLeast => "at-least",
            Direction::AtMost => "at-most",
            _ => continue,
        };
        for a in tiny {
            for b in tiny {
                out.push(format!(
                    "(requires ({0} (exactly {a})) ({0} (exactly {b})))",
                    e.name
                ));
                out.push(format!(
                    "(requires ({0} (exactly {a})) ({0} ({bound} {b})))",
                    e.name
                ));
            }
        }
    }
    // Up to three constraints from equalities and bounds, then each with an equality respelt (R27 1, R28 1).
    let mut pool: Vec<(String, Option<usize>)> = Vec::new();
    for (k, spellings) in RESPELT.iter().enumerate() {
        pool.push((
            format!("(delivery-bound (exactly {}))", spellings[0]),
            Some(k),
        ));
    }
    for b in RESPELT_BOUNDS {
        pool.push((format!("(delivery-bound (at-most {b}))"), None));
    }
    for i in 0..pool.len() {
        for j in i..pool.len() {
            for k in j..pool.len() {
                let items = [pool[i].0.as_str(), pool[j].0.as_str(), pool[k].0.as_str()];
                out.push(format!("(requires {})", items.join(" ")));
                for (_, class) in [&pool[i], &pool[j], &pool[k]] {
                    let Some(class) = class else { continue };
                    for respelt in &RESPELT[*class][1..] {
                        out.push(format!(
                            "(requires {} (delivery-bound (exactly {respelt})))",
                            items.join(" ")
                        ));
                    }
                }
            }
        }
    }
    // Three items in all six orders (R21 2).
    for a in ORDER_POOL {
        for b in ORDER_POOL {
            for c in ORDER_POOL {
                for [x, y, z] in [
                    [a, b, c],
                    [a, c, b],
                    [b, a, c],
                    [b, c, a],
                    [c, a, b],
                    [c, b, a],
                ] {
                    out.push(format!("(requires {x} {y} {z})"));
                }
            }
        }
    }
    // Operands that name nothing, lists for names, and names (R23 3, R26 1).
    for n in NAMES_NOTHING.iter().chain(LISTS_FOR_NAMES) {
        out.push(format!("(requires (needs {n}))"));
        out.push(format!("(requires (uses {n}))"));
    }
    for clause in [
        "(requires (needs uart))",
        "(requires (needs time.monotonic))",
        "(requires (uses time.monotonic))",
        "(requires (uses uart))",
        "(requires (needs or-through-mediation))",
        // A `requires` nested in another is a clause of its own (R26 2).
        "(requires (tick-unit ns) (requires (tick-unit us)))",
        "(requires (ordering (before a b) (needs uart)))",
        // A group nested and flat, and its head required false beside it.
        "(requires (absolute-deadline true) (supported-horizon (at-least 10 s)))",
        "(requires (absolute-deadline false) (absolute-deadline (supported-horizon (at-least 10 s))))",
        "(requires (needs absolute-deadline) (absolute-deadline false))",
        // A set's implied member on both sides of an equality, and an equality a bound refuses (R16 2, R17 1).
        "(requires (available-in-state (exactly idle)) (available-in-state (exactly run idle)))",
        "(requires (available-in-state (exactly idle)) (available-in-state sleep))",
        "(requires (available-in-state (exactly idle)) (available-in-state idle))",
        "(requires (queue-capacity (exactly 8)) (queue-capacity (at-least 9)))",
        "(requires (queue-capacity (exactly 8)) (queue-capacity (at-least 8 tick)))",
        "(requires (or-through-mediation allowed) (or-through-mediation (exactly allowed)))",
        "(requires (or-through-mediation allowed) (or-through-mediation forbidden))",
    ] {
        out.push(clause.to_string());
    }
    for item in small_items() {
        out.push(format!("(requires {item})"));
    }
    out
}

/// Every side the universe writes: a whole declaration.
pub fn side_texts() -> Vec<String> {
    let mut out = Vec::new();
    // A contradiction split across two clauses, a `uses` between them, or beside the side's own `needs` (R21 1).
    for e in VOCABULARY
        .iter()
        .filter(|e| e.direction == Direction::Exact)
    {
        let pool = samples(e.domain);
        for a in &pool {
            for b in &pool {
                out.push(format!(
                    "(defservice s (requires ({0} {a})) (uses time.monotonic) (requires ({0} {b})))",
                    e.name
                ));
            }
            if matches!(e.domain, Domain::Boolean | Domain::Group(_)) {
                out.push(format!(
                    "(defservice s (requires ({0} {a})) (needs {0}))",
                    e.name
                ));
            }
        }
    }
    // Every placement of a `needs`, a `uses` and a constraint (R23 1, R24 1, R29 1).
    for place in PLACES {
        for list in [
            "(needs uart)",
            "(uses uart)",
            "(needs observation-coherent)",
            "(uses tick-unit)",
        ] {
            out.push(format!("(defservice s {})", place.replace('X', list)));
        }
        out.push(format!(
            "(defservice s (requires (tick-unit ns)) {})",
            place.replace('X', "(requires (tick-unit us))")
        ));
    }
    for side in [
        // A declaration whose local name is a vocabulary fact, whatever its kind (§1.1, R16 1, R25 4).
        "(defservice low-power-timer (requires (needs uart)))",
        "(defsystem tick-unit (requires (uses s)))",
        "(defpolicy observation-coherent (requires (preemptive true)))",
        // A system with its platform and its tasks, read as one side.
        "(defsystem sys (platform (uses soc.p) (needs uart)) (task t (period 10 ms) (needs observation-coherent)) (requires (uses s)))",
        "(defsystem sys (platform (uses soc.p)) (task t (period 10 ms) (requires (tick-unit us))) (requires (tick-unit ns)))",
    ] {
        out.push(side.to_string());
    }
    out
}
