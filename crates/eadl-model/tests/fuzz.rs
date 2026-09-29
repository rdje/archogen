//! Seeded fuzzing over the reader and the exact arithmetic — §13.3's "property tests and fuzzing over parsing,
//! constraints, checked arithmetic", as the `extended` tier's `fuzz` step (leaf `PROGRAM.9.2`).
//!
//! ⭐ **No `cargo-fuzz`.** It needs `libfuzzer-sys` and a nightly toolchain, and the engine crates depend on `std`
//! alone (`docs/decisions/decision_zero-dependency-engine-core.md`). The generator here is a seeded xorshift, so a
//! run is reproducible from its seed, and every case builds its own generator from `(seed, property, case)` — one
//! failing case replays alone, by command, without re-running the cases before it.
//!
//! ⭐ **Arms before properties.** A property that passes proves something only if the generator reaches the
//! inputs where it could fail. Each arm is a claim known to be *false* — "no input contains a multi-byte
//! character", "no pair of values has both cross products beyond `i128`" — and the run fails unless the
//! generator refutes it within the budget. Those two regions are where the day's two hand-found defects lived:
//! the reader's escape panic (`M1.35`) and `Rational`'s saturating comparison (`M1.34`).
//!
//! Two entry points: `fuzz_smoke` runs a small budget with the suite, so the harness cannot rot unseen; the
//! `extended` tier's step runs `fuzz_extended` with `FUZZ_CASES` (default 100 000) and a fresh `FUZZ_SEED`.
//! Replay one case: `FUZZ_SEED=<s> FUZZ_PROPERTY=<name> FUZZ_CASE=<k> cargo test -p eadl-model --test fuzz --
//! --ignored --nocapture`.

use std::panic::{catch_unwind, AssertUnwindSafe};

use eadl_front::{read, Form, SourceMap, Span};
use eadl_model::rational::Rational;

// ── the generator ─────────────────────────────────────────────────────────────────────────────────────────────

struct Rng(u64);

impl Rng {
    /// A generator for one case of one property: splitmix64 over the seed, the property's name and the case.
    fn new(seed: u64, property: &str, case: u64) -> Self {
        let mut state = seed ^ case.wrapping_mul(0x9E37_79B9_7F4A_7C15);
        for byte in property.bytes() {
            state = (state ^ u64::from(byte)).wrapping_mul(0x0100_0000_01B3);
        }
        state = state.wrapping_add(0x9E37_79B9_7F4A_7C15);
        state = (state ^ (state >> 30)).wrapping_mul(0xBF58_476D_1CE4_E5B9);
        state = (state ^ (state >> 27)).wrapping_mul(0x94D0_49BB_1331_11EB);
        Self((state ^ (state >> 31)) | 1)
    }

    /// xorshift64*.
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_F491_4F6C_DD1D)
    }

    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % n as u64).expect("below n")
    }

    fn chance(&mut self, percent: u64) -> bool {
        self.next() % 100 < percent
    }

    fn pick<'a, T>(&mut self, items: &'a [T]) -> &'a T {
        &items[self.below(items.len())]
    }
}

/// Fragments of text, chosen to reach every lexical corner: the multi-byte and four-byte characters, escapes
/// valid and invalid, numbers that overflow, stray closers, control characters, comments, a byte-order mark.
const FRAGMENTS: &[&str] = &[
    "(",
    ")",
    "(",
    ")",
    " ",
    " ",
    "\n",
    "\t",
    "\r\n",
    ";",
    "; é comment\n",
    "\"",
    "\"",
    "\\",
    "\\n",
    "\\t",
    "\\\"",
    "\\\\",
    "\\q",
    "\\é",
    "\\🙂",
    "\\u{e9}",
    "\\u{1F600}",
    "\\u{110000}",
    "\\u{D800}",
    "\\u{",
    "\\u{zz}",
    "a",
    "z",
    "-",
    ".",
    "_",
    "0",
    "9",
    "1.5",
    "0.125",
    "-3",
    "+4",
    "0x1F",
    "0x",
    "1e9",
    "3ms",
    "10 ms",
    "9223372036854775807",
    "9223372036854775808",
    "1.00000000000000001",
    "defsystem",
    "task",
    "(period 10 ms)",
    "(deadline 1.5 ms)",
    "eadl-version",
    "(eadl-version eadl/1)",
    "eadl/1",
    "é",
    "→",
    "🙂",
    "ß",
    "\u{0}",
    "\u{7f}",
    "\u{feff}",
    "\u{200b}",
    "'",
    "`",
    "#",
    ",",
    "[",
    "]",
    "{",
    "}",
];

/// Arbitrary text: 0–40 fragments.
fn text(rng: &mut Rng) -> String {
    let n = rng.below(41);
    (0..n).map(|_| *rng.pick(FRAGMENTS)).collect()
}

const SYMBOL_PARTS: &[&str] = &[
    "time",
    "monotonic",
    "task",
    "a",
    "x1",
    "soc",
    "delay",
    "rate",
    "é",
];
const STRING_PIECES: &[&str] = &[
    "a",
    " ",
    "é",
    "🙂",
    "\\n",
    "\\t",
    "\\\"",
    "\\\\",
    "\\u{e9}",
    "\\u{1F600}",
    "ok",
];

/// One well-formed form, most of the time — a document of these reads cleanly often enough for the
/// round-trip property to mean something, which arm `a-structured-document-reads-cleanly` checks.
fn form(rng: &mut Rng, depth: u32) -> String {
    match rng.below(if depth == 0 { 5 } else { 6 }) {
        0 => {
            let parts = 1 + rng.below(3);
            let separator = if rng.chance(50) { "-" } else { "." };
            (0..parts)
                .map(|_| *rng.pick(SYMBOL_PARTS))
                .collect::<Vec<_>>()
                .join(separator)
        }
        1 => {
            let value: i64 = match rng.below(4) {
                0 => 0,
                1 => i64::MAX,
                2 => -(i64::try_from(rng.next() % 1_000_000).expect("small")),
                _ => i64::try_from(rng.next() % 1_000_000_000).expect("small"),
            };
            value.to_string()
        }
        2 => {
            let whole = rng.next() % 100_000;
            let digits = 1 + rng.below(6);
            let fraction: String = (0..digits)
                .map(|_| char::from(b'0' + (rng.next() % 10) as u8))
                .collect();
            format!("{whole}.{fraction}")
        }
        3 => format!("0x{:X}", rng.next() % 0xFFFF),
        4 => {
            let n = rng.below(5);
            format!(
                "\"{}\"",
                (0..n).map(|_| *rng.pick(STRING_PIECES)).collect::<String>()
            )
        }
        _ => {
            let n = rng.below(5);
            let head = *rng.pick(SYMBOL_PARTS);
            let items: Vec<String> = (0..n).map(|_| form(rng, depth - 1)).collect();
            format!(
                "({head}{}{})",
                if items.is_empty() { "" } else { " " },
                items.join(" ")
            )
        }
    }
}

/// A description: the version statement and 1–4 forms, separated by whitespace and comments.
fn document(rng: &mut Rng) -> String {
    let mut out = String::from("(eadl-version eadl/1)\n");
    for _ in 0..=rng.below(4) {
        out.push_str(&form(rng, 3));
        out.push_str(rng.pick::<&str>(&["\n", " ", "\n\n", "  ; a comment\n", "\t"]));
    }
    out
}

/// A rational anywhere in the representation: small, large, or at the limits.
fn rational(rng: &mut Rng) -> Rational {
    fn wide(rng: &mut Rng) -> i128 {
        (i128::from(rng.next() as i64) << 64) | i128::from(rng.next())
    }
    fn magnitude(rng: &mut Rng) -> i128 {
        match rng.below(6) {
            0 => i128::from(rng.next() % 1000),
            1 => i128::from(rng.next() % (1 << 30)),
            2 => (1_i128 << 100) + i128::from(rng.next() % 16),
            3 => i128::MAX - i128::from(rng.next() % 16),
            4 => wide(rng).unsigned_abs().min(i128::MAX as u128) as i128,
            _ => 10_i128.pow(u32::try_from(rng.below(38)).expect("small")),
        }
    }
    for _ in 0..8 {
        let mut numerator = magnitude(rng);
        if rng.chance(50) {
            numerator = numerator.checked_neg().unwrap_or(i128::MIN);
        }
        if rng.chance(3) {
            numerator = i128::MIN;
        }
        let denominator = magnitude(rng).max(1);
        if let Some(value) = Rational::new(numerator, denominator) {
            return value;
        }
    }
    Rational::ONE
}

/// A rational small enough that every cross product and sum fits `i128`, so an exact oracle is cheap.
fn small_rational(rng: &mut Rng) -> Rational {
    let bound = 1_i128 << 30;
    // ⛔ `u64` literals: `(2 << 30) as u64` is computed as an `i32`, overflows to a negative, and
    // sign-extends to a modulus near 2^64 — measured on this harness's first run.
    let numerator = i128::from(rng.next() % (2_u64 << 30)) - bound;
    let denominator = i128::from(rng.next() % (1_u64 << 30)) + 1;
    Rational::new(numerator, denominator).expect("a positive denominator")
}

// ── the oracles ──────────────────────────────────────────────────────────────────────────────────────────────

fn gcd(mut a: u128, mut b: u128) -> u128 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

fn normalized(r: Rational) -> Result<(), String> {
    if r.denominator() <= 0 {
        return Err(format!("{r:?} has a denominator that is not positive"));
    }
    if gcd(r.numerator().unsigned_abs(), r.denominator().unsigned_abs()) != 1 {
        return Err(format!("{r:?} is not in lowest terms"));
    }
    Ok(())
}

/// `n/d == r` for a small result, by cross-multiplication in `i128`.
fn equals_fraction(r: Rational, n: i128, d: i128) -> bool {
    r.numerator() * d == n * r.denominator()
}

/// A span is inside its source and on character boundaries.
fn span_ok(text: &str, span: Span, what: &str) -> Result<(), String> {
    let (start, end) = (span.start as usize, span.end as usize);
    if start > end || end > text.len() {
        return Err(format!(
            "{what} span {start}..{end} is outside a {}-byte text",
            text.len()
        ));
    }
    if !text.is_char_boundary(start) || !text.is_char_boundary(end) {
        return Err(format!(
            "{what} span {start}..{end} is not on character boundaries"
        ));
    }
    Ok(())
}

fn forms_ok(text: &str, forms: &[Form]) -> Result<(), String> {
    for form in forms {
        span_ok(text, form.span(), "form")?;
        forms_ok(text, form.items())?;
    }
    Ok(())
}

// ── properties and arms ──────────────────────────────────────────────────────────────────────────────────────

type Property = fn(&mut Rng) -> Result<(), String>;
type Claim = fn(&mut Rng) -> bool;

/// Properties: each must hold on every case.
const PROPERTIES: &[(&str, Property)] = &[
    (
        "reader-never-panics-and-every-span-is-on-a-boundary",
        |rng| {
            let text = text(rng);
            let mut sources = SourceMap::new();
            let id = sources
                .add("fuzz.eadl", text.clone())
                .map_err(|e| format!("{e:?}"))?;
            let (document, diagnostics) = read(&sources, id);
            for diagnostic in diagnostics.items() {
                span_ok(&text, diagnostic.primary.span, diagnostic.code)?;
                for label in &diagnostic.secondary {
                    span_ok(&text, label.span, diagnostic.code)?;
                }
            }
            for comment in &document.comments {
                span_ok(&text, comment.span, "comment")?;
            }
            forms_ok(&text, &document.forms)?;
            let _ = diagnostics.render(&sources);
            Ok(())
        },
    ),
    ("canonical-form-reads-back-to-itself", |rng| {
        let text = document(rng);
        let mut sources = SourceMap::new();
        let id = sources
            .add("fuzz.eadl", text)
            .map_err(|e| format!("{e:?}"))?;
        let (first, diagnostics) = read(&sources, id);
        if diagnostics.has_errors() {
            return Ok(());
        }
        let canonical = first.to_canonical();
        let again = sources
            .add("canonical.eadl", canonical.clone())
            .map_err(|e| format!("{e:?}"))?;
        let (second, diagnostics) = read(&sources, again);
        if diagnostics.has_errors() {
            return Err(format!(
                "canonical text does not read cleanly:\n{canonical}\n{}",
                diagnostics.render(&sources)
            ));
        }
        if !second.structurally_eq(&first) || second.to_canonical() != canonical {
            return Err(format!(
                "canonical text does not read back to the same document:\n{canonical}"
            ));
        }
        Ok(())
    }),
    (
        "checked-arithmetic-never-panics-and-stays-normalized",
        |rng| {
            let (a, b) = (rational(rng), rational(rng));
            for r in [
                a.checked_add(b),
                a.checked_sub(b),
                a.checked_mul(b),
                a.checked_div(b),
                a.negate(),
            ]
            .into_iter()
            .flatten()
            {
                normalized(r)?;
            }
            if let (Some(floor), Some(ceil)) = (a.floor(), a.ceil()) {
                if !(floor <= ceil && ceil - floor <= 1) {
                    return Err(format!(
                        "{a:?}: floor {floor} and ceil {ceil} are not adjacent"
                    ));
                }
            }
            let _ = a.to_exact_string();
            Ok(())
        },
    ),
    ("exact-text-reads-back-to-the-value", |rng| {
        // `to_exact_string` is how every quantity is printed. It must never approximate: a decimal must be
        // the value, and a fraction must be its numerator and denominator.
        let a = rational(rng);
        let text = a.to_exact_string();
        if let Some((n, d)) = text.split_once('/') {
            let (n, d) = (n.parse::<i128>(), d.parse::<i128>());
            return match (n, d) {
                (Ok(n), Ok(d)) if n == a.numerator() && d == a.denominator() => Ok(()),
                _ => Err(format!("{a:?} printed as the fraction `{text}`")),
            };
        }
        let negative = text.starts_with('-');
        let digits = text.trim_start_matches('-');
        let (whole, fraction) = digits.split_once('.').unwrap_or((digits, ""));
        let scale = u32::try_from(fraction.len()).map_err(|_| "an absurd scale".to_string())?;
        // In `u128`, because a correct decimal can print a magnitude of exactly 2^127 — `i128::MIN`'s — which
        // no `i128` holds (measured: this oracle's first cut refused a correct `-2^112 / 5^15`).
        let magnitude = format!("{whole}{fraction}").parse::<u128>();
        let power = 10_u128.checked_pow(scale);
        let denominator = a.denominator().unsigned_abs();
        match (magnitude, power) {
            (Ok(magnitude), Some(power)) if power % denominator == 0 => {
                let expected = a
                    .numerator()
                    .unsigned_abs()
                    .checked_mul(power / denominator);
                if expected == Some(magnitude) && negative == a.is_negative() {
                    Ok(())
                } else {
                    Err(format!("{a:?} printed as `{text}`, which is another value"))
                }
            }
            _ => Err(format!(
                "{a:?} printed as `{text}`, which is not its exact decimal"
            )),
        }
    }),
    ("small-arithmetic-is-exact", |rng| {
        let (a, b) = (small_rational(rng), small_rational(rng));
        let (an, ad, bn, bd) = (
            a.numerator(),
            a.denominator(),
            b.numerator(),
            b.denominator(),
        );
        let sum = a.checked_add(b).ok_or("a small sum overflowed")?;
        let difference = a.checked_sub(b).ok_or("a small difference overflowed")?;
        let product = a.checked_mul(b).ok_or("a small product overflowed")?;
        if !equals_fraction(sum, an * bd + bn * ad, ad * bd)
            || !equals_fraction(difference, an * bd - bn * ad, ad * bd)
            || !equals_fraction(product, an * bn, ad * bd)
        {
            return Err(format!("{a:?} and {b:?}: a result is not the exact value"));
        }
        if bn != 0 {
            let quotient = a.checked_div(b).ok_or("a small quotient overflowed")?;
            if !equals_fraction(quotient, an * bd, ad * bn) {
                return Err(format!("{a:?} / {b:?} is not the exact value"));
            }
        }
        Ok(())
    }),
    ("ordering-agrees-with-equality-and-is-transitive", |rng| {
        let (a, b, c) = (rational(rng), rational(rng), rational(rng));
        for (x, y) in [(a, b), (b, c), (a, c), (a, a)] {
            if (x.cmp(&y) == core::cmp::Ordering::Equal) != (x == y) {
                return Err(format!(
                    "{x:?} and {y:?}: cmp says {:?}, == says {}",
                    x.cmp(&y),
                    x == y
                ));
            }
            if x.cmp(&y) != y.cmp(&x).reverse() {
                return Err(format!("{x:?} and {y:?}: cmp is not antisymmetric"));
            }
        }
        if a <= b && b <= c && a > c {
            return Err(format!("{a:?} ≤ {b:?} ≤ {c:?} but {a:?} > {c:?}"));
        }
        Ok(())
    }),
    ("small-ordering-is-the-sign-of-the-difference", |rng| {
        let (a, b) = (small_rational(rng), small_rational(rng));
        let exact = (a.numerator() * b.denominator()).cmp(&(b.numerator() * a.denominator()));
        if a.cmp(&b) != exact {
            return Err(format!(
                "{a:?} vs {b:?}: cmp says {:?}, the exact order is {exact:?}",
                a.cmp(&b)
            ));
        }
        Ok(())
    }),
    ("an-inverse-undoes-its-operation", |rng| {
        let (a, b) = (rational(rng), rational(rng));
        if let Some(back) = a.checked_add(b).and_then(|sum| sum.checked_sub(b)) {
            if back != a {
                return Err(format!("({a:?} + {b:?}) − {b:?} = {back:?}"));
            }
        }
        if !b.is_zero() {
            if let Some(back) = a.checked_mul(b).and_then(|product| product.checked_div(b)) {
                if back != a {
                    return Err(format!("({a:?} × {b:?}) ÷ {b:?} = {back:?}"));
                }
            }
        }
        Ok(())
    }),
];

/// Arms: each claim is known to be false, and the run fails unless the generator finds a case refuting it —
/// otherwise the properties above could pass without ever reaching the inputs that matter.
const ARMS: &[(&str, Claim)] = &[
    ("every-text-reads-without-an-error", |rng| {
        let mut sources = SourceMap::new();
        let id = sources.add("arm.eadl", text(rng)).expect("small");
        !read(&sources, id).1.has_errors()
    }),
    ("no-text-contains-a-multi-byte-character", |rng| {
        text(rng).is_ascii()
    }),
    ("no-text-escapes-a-multi-byte-character", |rng| {
        let text = text(rng);
        !(text.contains("\\é") || text.contains("\\🙂"))
    }),
    ("no-structured-document-reads-cleanly", |rng| {
        let mut sources = SourceMap::new();
        let id = sources.add("arm.eadl", document(rng)).expect("small");
        read(&sources, id).1.has_errors()
    }),
    ("checked-multiplication-never-overflows", |rng| {
        rational(rng).checked_mul(rational(rng)).is_some()
    }),
    ("no-pair-has-both-cross-products-beyond-i128", |rng| {
        let (a, b) = (rational(rng), rational(rng));
        a == b
            || a.numerator().checked_mul(b.denominator()).is_some()
            || b.numerator().checked_mul(a.denominator()).is_some()
    }),
];

// ── the runner ───────────────────────────────────────────────────────────────────────────────────────────────

/// The fixed seed the smoke test uses, and the extended step's default when `FUZZ_SEED` is unset.
const DEFAULT_SEED: u64 = 0x5EED_2026_0929_0001;

struct Budget {
    seed: u64,
    cases: u64,
    only_property: Option<String>,
    only_case: Option<u64>,
}

impl Budget {
    fn from_env(default_cases: u64) -> Self {
        let number = |name: &str| {
            std::env::var(name)
                .ok()
                .and_then(|v| v.trim().parse::<u64>().ok())
        };
        Self {
            seed: number("FUZZ_SEED").unwrap_or(DEFAULT_SEED),
            cases: number("FUZZ_CASES").unwrap_or(default_cases),
            only_property: std::env::var("FUZZ_PROPERTY").ok(),
            only_case: number("FUZZ_CASE"),
        }
    }

    fn cases(&self) -> Box<dyn Iterator<Item = u64>> {
        match self.only_case {
            Some(case) => Box::new(core::iter::once(case)),
            None => Box::new(0..self.cases),
        }
    }

    fn wants(&self, name: &str) -> bool {
        self.only_property
            .as_deref()
            .is_none_or(|only| only == name)
    }

    fn replay(&self, name: &str, case: u64) -> String {
        format!(
            "replay: FUZZ_SEED={} FUZZ_PROPERTY={name} FUZZ_CASE={case} cargo test -p eadl-model --test fuzz -- --ignored --nocapture",
            self.seed
        )
    }
}

fn panic_text(payload: &(dyn std::any::Any + Send)) -> String {
    payload
        .downcast_ref::<&str>()
        .map(|s| (*s).to_string())
        .or_else(|| payload.downcast_ref::<String>().cloned())
        .unwrap_or_else(|| "a panic with no message".to_string())
}

/// Every arm, then every property; the failures, each with its replay line.
fn run(budget: &Budget) -> Vec<String> {
    let mut failures = Vec::new();
    println!(
        "fuzz: seed {}, {} case(s) per property",
        budget.seed, budget.cases
    );
    for (name, claim) in ARMS {
        if !budget.wants(name) || budget.only_case.is_some() {
            continue;
        }
        let refuted = budget
            .cases()
            .find(|&case| !claim(&mut Rng::new(budget.seed, name, case)));
        match refuted {
            Some(case) => println!("  ✓ arm `{name}` refuted at case {case}"),
            None => failures.push(format!(
                "arm `{name}` was never refuted in {} case(s) — the generator does not reach that region, so a \
                 property that depends on it would pass without testing it",
                budget.cases
            )),
        }
    }
    for (name, property) in PROPERTIES {
        if !budget.wants(name) {
            continue;
        }
        let mut ran = 0_u64;
        for case in budget.cases() {
            ran += 1;
            let mut rng = Rng::new(budget.seed, name, case);
            let outcome = catch_unwind(AssertUnwindSafe(|| property(&mut rng)));
            let detail = match outcome {
                Ok(Ok(())) => continue,
                Ok(Err(detail)) => detail,
                Err(payload) => format!("panicked: {}", panic_text(payload.as_ref())),
            };
            failures.push(format!(
                "property `{name}` failed at case {case}: {detail}\n  {}",
                budget.replay(name, case)
            ));
            break;
        }
        println!("  ✓ property `{name}`: {ran} case(s)");
    }
    failures
}

#[test]
#[cfg_attr(
    miri,
    ignore = "the arms need their budget, and Miri runs the same code through the other suites"
)]
fn fuzz_smoke() {
    let budget = Budget {
        seed: DEFAULT_SEED,
        cases: 400,
        only_property: None,
        only_case: None,
    };
    let failures = run(&budget);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}

#[test]
#[ignore = "the extended tier's fuzz step runs this, with a fresh seed: scripts/extended_fuzz.sh"]
fn fuzz_extended() {
    let budget = Budget::from_env(100_000);
    let failures = run(&budget);
    assert!(failures.is_empty(), "{}", failures.join("\n"));
}
