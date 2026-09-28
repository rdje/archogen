//! `M1.12.1` — the language reference is normative, and this is what makes that true rather than
//! claimed.
//!
//! `docs/semantics/reference.md` states what a literal is **worth**: the exact value behind a number,
//! the decoded value behind a string, and what canonical form guarantees about both. This file reads
//! those tables *out of the document* and executes every row against `crates/eadl-front/src/reader.rs`
//! and `crates/eadl-front/src/form.rs`. A row the frontend disagrees with fails the build naming the
//! literal, the value the reference states, and the value the frontend produced.
//!
//! ⛔ **Why value, and not only syntax.** `M1.11` made the grammar normative and proved the reader
//! agrees with it on the language *and* on where every token begins and ends. It recorded its own
//! limit verbatim: *"a reader that tokenized identically and mis-nested, or that read `1.5` as three
//! halves, would still pass."* This is the leg that closes it, by the same move that made the EBNF
//! normative rather than decorative — the document is the source, and the test reads it.
//!
//! ⭐ **The expected values were derived, not transcribed.** Each row states the literal's arithmetic
//! meaning (`0x1000_0000` is 16^7), §7.4's exactness requirement, or a decision recorded in
//! `docs/tasks/M1.md` — never a printout of what the reader happened to produce. A table filled in
//! from the implementation would be exactly as unfalsifiable as no table at all. On its first run the
//! derivation and the reader disagreed three times, and every disagreement was a reader defect rather
//! than a wrong row: a leading hexadecimal separator was accepted, canonical form emitted a raw
//! carriage return, and `-9223372036854775808` was refused as overflow because the magnitude was
//! parsed as `i64` *before* the sign was applied.
//!
//! The legs, because "the rows pass" is the weakest thing this file could claim:
//!
//! 1. **EXECUTED** — every row of both tables, against the reader: verdict, exact value, canonical
//!    text, and the property that canonical text carries no control character.
//! 2. **COVERING** — every numeric literal the repository actually ships has a row, and so does every
//!    string that carries a backslash. The population is the corpus, which this file does not define,
//!    so the leg cannot be satisfied by writing a table that agrees with itself.
//! 3. **NON-VACUOUS** — each verdict class the notation defines is pinned by at least one row, and a
//!    document that stops carrying a table is reported rather than silently gating nothing.
//!
//! ⚠️ **Honest limit.** A deleted row is caught only when the corpus used that literal, or when it was
//! the last row of its verdict class; nothing here enumerates the *forms* the language admits, so a
//! literal shape no description happens to contain can be absent from the table and this file stays
//! green. That enumeration is `M1.12.2`'s, which runs the same rows against the recognizer derived
//! from `docs/semantics/grammar.md` and derives its production probes from them. This file also says
//! nothing about nesting, meaning, kinds or diagnostics — `corpus.rs` owns the first two and
//! `M1.12.3`/`M1.12.4` the last.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use eadl_front::{read, Form, Severity, SourceMap};

/// The normative document under test.
///
/// `include_str!` rather than `fs::read_to_string`, for the reason `corpus.rs` gives for its live
/// surfaces: if the reference moves or is deleted, this crate **stops compiling** instead of silently
/// gating nothing.
const REFERENCE: &str = include_str!("../../../docs/semantics/reference.md");

// ── the value notation ─────────────────────────────────────────────────────────────────────────
//
// Exactly the forms `docs/semantics/reference.md` defines in its notation table, and no others: a
// notation that could express more than this file understands is a notation that would eventually be
// used to write a rule nothing checks.

/// What the reference says a literal is worth.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    /// `integer N` — an exact 64-bit signed integer.
    Integer(i64),
    /// `rational N/10^S` — the digits `N` with the last `S` after the point. Never a float.
    Rational(i64, u32),
    /// `symbol T` — not a number: the atom is the symbol `T`.
    Symbol(String),
    /// `error C` — refused, with diagnostic code `C`.
    Error(String),
}

impl Value {
    /// Read one value cell, or `None` if the notation cannot express it.
    ///
    /// `None` is a **violation**, never a reason to skip the row: a cell this file cannot parse is a
    /// rule nothing checks, which is the state this leaf exists to end.
    fn parse(cell: &str) -> Option<Self> {
        let (head, rest) = cell.split_once(' ')?;
        let rest = rest.trim();
        match head {
            "integer" => Some(Self::Integer(rest.parse().ok()?)),
            "rational" => {
                let (digits, scale) = rest.split_once("/10^")?;
                Some(Self::Rational(digits.parse().ok()?, scale.parse().ok()?))
            }
            "symbol" if !rest.is_empty() => Some(Self::Symbol(rest.to_string())),
            "error" if !rest.is_empty() => Some(Self::Error(rest.to_string())),
            _ => None,
        }
    }

    /// The verdict class name, for the non-vacuity leg.
    fn class(&self) -> &'static str {
        match self {
            Self::Integer(_) => "integer",
            Self::Rational(_, _) => "rational",
            Self::Symbol(_) => "symbol",
            Self::Error(_) => "error",
        }
    }
}

/// Decode the reference's value notation into characters.
///
/// ⭐ A second implementation of the escape rule, written from §2 of the reference rather than from
/// `reader.rs`. That is the point: one implementation cannot disagree with itself, so a reader that
/// decoded `\n` as a tab would be checked by nothing.
fn decode_escapes(cell: &str) -> Option<String> {
    let mut out = String::new();
    let mut chars = cell.chars();
    while let Some(ch) = chars.next() {
        if ch != '\\' {
            out.push(ch);
            continue;
        }
        match chars.next()? {
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            '"' => out.push('"'),
            '\\' => out.push('\\'),
            // The notation says a backslash in a value cell always starts one of the five escapes, so
            // anything else means the document is not writable in its own notation.
            _ => return None,
        }
    }
    Some(out)
}

/// Encode characters as canonical text, per §3 of the reference.
///
/// Also a second implementation, and the one that makes finding F-D a permanent impossibility rather
/// than a repaired incident: canonical text escapes every character that would break "one form per
/// line", so a printer that emits a raw control byte disagrees with this and fails.
fn encode_canonical(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// The first ASCII control character in `text`, if any.
///
/// §3 rule 3 of the reference: canonical text is what gets diffed, hashed and printed in a report, so
/// it carries no control character. Tab, line feed and carriage return all break "one form per line"
/// in a different way, and a bare `\t` in a report column is invisible rather than merely ugly.
fn control_character(text: &str) -> Option<char> {
    text.chars().find(|ch| ch.is_ascii_control())
}

/// One violation, phrased so the failure names the row and both sides.
fn violation(line: usize, message: String) -> String {
    format!("docs/semantics/reference.md:{line}: {message}")
}

// ── reading the tables out of the document ─────────────────────────────────────────────────────

/// The rows of the table introduced by `<!-- machine-read: <marker> -->`, with their line numbers.
///
/// Returns an empty vector when the marker is absent — which the non-vacuity leg turns into a
/// violation, so a renamed marker cannot make this file pass by checking nothing.
fn machine_table(document: &str, marker: &str) -> Vec<(usize, Vec<String>)> {
    let needle = format!("<!-- machine-read: {marker} -->");
    let lines: Vec<&str> = document.lines().collect();
    let Some(start) = lines.iter().position(|line| line.trim() == needle) else {
        return Vec::new();
    };
    let mut rows = Vec::new();
    let mut header_seen = false;
    let mut separator_seen = false;
    for (index, line) in lines.iter().enumerate().skip(start + 1) {
        let trimmed = line.trim();
        if !trimmed.starts_with('|') {
            // A blank line between the marker and the table is layout; anything else ends it.
            if trimmed.is_empty() && !header_seen {
                continue;
            }
            break;
        }
        let cells = split_cells(trimmed);
        if !header_seen {
            header_seen = true;
            continue;
        }
        if !separator_seen {
            separator_seen = true;
            continue;
        }
        rows.push((index + 1, cells));
    }
    rows
}

/// Split a markdown table row into cells, dropping one surrounding code span per cell.
fn split_cells(line: &str) -> Vec<String> {
    line.trim()
        .trim_start_matches('|')
        .trim_end_matches('|')
        .split('|')
        .map(|cell| {
            let cell = cell.trim();
            cell.strip_prefix('`')
                .and_then(|inner| inner.strip_suffix('`'))
                .map_or_else(|| cell.to_string(), str::to_string)
        })
        .collect()
}

// ── running a literal through the frontend ─────────────────────────────────────────────────────

/// What the frontend made of one literal.
struct Probe {
    /// The top-level forms, in source order.
    forms: Vec<Form>,
    /// Every error-severity diagnostic code, in order.
    errors: Vec<&'static str>,
    /// The rendered diagnostics, for a failure message an author can act on.
    rendered: String,
}

/// Read `text` as a whole description.
///
/// The literal is probed **alone**, not inside a list: `(probe "unterminated` would add an
/// `read-unclosed-list` of its own, and a leg that has to know which diagnostics are the probe's
/// scaffolding is a leg that can be fooled by it.
fn probe(text: &str) -> Probe {
    let mut sources = SourceMap::new();
    let id = sources
        .add("reference.eadl", text.to_string())
        .expect("small");
    let (document, diagnostics) = read(&sources, id);
    Probe {
        forms: document.forms,
        errors: diagnostics
            .items()
            .iter()
            .filter(|item| matches!(item.severity, Severity::Error))
            .map(|item| item.code)
            .collect(),
        rendered: diagnostics.render(&sources),
    }
}

/// Name a form in the reference's own notation, so a failure reads as a disagreement about values.
fn describe(form: &Form) -> String {
    match form {
        Form::List { items, .. } => format!("a list of {} item(s)", items.len()),
        Form::Symbol { name, .. } => format!("symbol {name}"),
        Form::Integer { value, .. } => format!("integer {value}"),
        Form::Decimal { value, scale, .. } => format!("rational {value}/10^{scale}"),
        Form::Str { value, .. } => format!("string {}", encode_canonical(value)),
    }
}

/// Leg 1a — every row of the number table, executed against the reader.
fn number_violations(document: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (line, cells) in machine_table(document, "number-values") {
        let Some([literal, stated, canonical]) = array3(&cells) else {
            out.push(violation(
                line,
                format!(
                    "this row has {} cell(s) and the table's header has 3 — GFM silently pads or \
                     drops the difference, so the row does not say what it looks like it says",
                    cells.len()
                ),
            ));
            continue;
        };
        let Some(expected) = Value::parse(stated) else {
            out.push(violation(
                line,
                format!(
                    "`{stated}` is not a value this language's notation defines (`integer N`, \
                     `rational N/10^S`, `symbol T`, `error C`), so nothing checks `{literal}`"
                ),
            ));
            continue;
        };
        let found = probe(literal);
        if let Value::Error(code) = &expected {
            if !found.errors.contains(&code.as_str()) {
                out.push(violation(
                    line,
                    format!(
                        "the reference refuses `{literal}` as `{code}`, and the frontend did not:\n{}",
                        if found.errors.is_empty() {
                            format!("  it read {} with no diagnostic", describe_all(&found.forms))
                        } else {
                            found.rendered
                        }
                    ),
                ));
            }
            continue;
        }
        if !found.errors.is_empty() {
            out.push(violation(
                line,
                format!(
                    "the reference says `{literal}` is {}, and the frontend refused it:\n{}",
                    describe_value(&expected),
                    found.rendered
                ),
            ));
            continue;
        }
        let Some(form) = found.forms.first() else {
            out.push(violation(
                line,
                format!(
                    "the reference says `{literal}` is {}, and the frontend produced no form",
                    describe_value(&expected)
                ),
            ));
            continue;
        };
        if let Some(message) = value_mismatch(&expected, form) {
            out.push(violation(
                line,
                format!(
                    "`{literal}`: the reference states {}, the frontend produced {}",
                    describe_value(&expected),
                    message
                ),
            ));
        }
        let printed = form.to_canonical();
        if canonical != "—" && printed != *canonical {
            out.push(violation(
                line,
                format!(
                    "`{literal}`: the reference states canonical text `{canonical}`, the frontend \
                     printed `{printed}`"
                ),
            ));
        }
        if let Some(ch) = control_character(&printed) {
            out.push(violation(
                line,
                format!(
                    "`{literal}`: canonical text carries the control character {:?}, which §3 rule 3 \
                     forbids — canonical text is hashed, diffed and put in reports, so it stays on \
                     one line:\n  {}",
                    ch,
                    printed.escape_debug()
                ),
            ));
        }
    }
    out
}

/// Leg 1b — every row of the string table, executed against the reader.
fn string_violations(document: &str) -> Vec<String> {
    let mut out = Vec::new();
    for (line, cells) in machine_table(document, "string-values") {
        let Some([source, stated]) = array2(&cells) else {
            out.push(violation(
                line,
                format!(
                    "this row has {} cell(s) and the table's header has 2",
                    cells.len()
                ),
            ));
            continue;
        };
        let found = probe(source);
        if let Some(code) = stated.strip_prefix("error ") {
            if !found.errors.contains(&code) {
                out.push(violation(
                    line,
                    format!(
                        "the reference refuses {source} as `{code}`, and the frontend did not:\n{}",
                        if found.errors.is_empty() {
                            format!(
                                "  it read {} with no diagnostic",
                                describe_all(&found.forms)
                            )
                        } else {
                            found.rendered
                        }
                    ),
                ));
            }
            continue;
        }
        let Some(expected) = decode_escapes(stated) else {
            out.push(violation(
                line,
                format!(
                    "`{stated}` is not writable in the reference's own escape notation, so nothing \
                     checks {source}"
                ),
            ));
            continue;
        };
        if !found.errors.is_empty() {
            out.push(violation(
                line,
                format!(
                    "the reference says {source} denotes {:?}, and the frontend refused it:\n{}",
                    expected, found.rendered
                ),
            ));
            continue;
        }
        let Some(form @ Form::Str { value, .. }) = found.forms.first() else {
            out.push(violation(
                line,
                format!(
                    "the reference says {source} denotes a string, and the frontend produced {}",
                    found
                        .forms
                        .first()
                        .map_or_else(|| "no form".to_string(), describe)
                ),
            ));
            continue;
        };
        if *value != expected {
            out.push(violation(
                line,
                format!(
                    "{source}: the reference states the decoded value {:?}, the frontend produced \
                     {:?}",
                    expected, value
                ),
            ));
        }
        // §3 rule 3, applied through the reference's own escaping rule rather than a second table:
        // a string's canonical text is its decoded value re-encoded, and nothing else.
        let printed = form.to_canonical();
        let stated_canonical = encode_canonical(&expected);
        if printed != stated_canonical {
            out.push(violation(
                line,
                format!(
                    "{source}: §3 requires canonical text {}, the frontend printed {}",
                    stated_canonical.escape_debug(),
                    printed.escape_debug()
                ),
            ));
        }
        if let Some(ch) = control_character(&printed) {
            out.push(violation(
                line,
                format!(
                    "{source}: canonical text carries the control character {:?}, which §3 rule 3 \
                     forbids:\n  {}",
                    ch,
                    printed.escape_debug()
                ),
            ));
        }
    }
    out
}

fn describe_value(value: &Value) -> String {
    match value {
        Value::Integer(v) => format!("`integer {v}`"),
        Value::Rational(v, s) => format!("`rational {v}/10^{s}`"),
        Value::Symbol(t) => format!("`symbol {t}`"),
        Value::Error(c) => format!("`error {c}`"),
    }
}

fn describe_all(forms: &[Form]) -> String {
    if forms.is_empty() {
        return "nothing".to_string();
    }
    forms.iter().map(describe).collect::<Vec<_>>().join(", ")
}

/// Compare a stated value with the form the frontend built, in the reference's own notation.
fn value_mismatch(expected: &Value, form: &Form) -> Option<String> {
    let agreed = match (expected, form) {
        (Value::Integer(want), Form::Integer { value, .. }) => *value == *want,
        (
            Value::Rational(want, scale),
            Form::Decimal {
                value, scale: got, ..
            },
        ) => *value == *want && *got == *scale,
        (Value::Symbol(want), Form::Symbol { name, .. }) => name == want,
        _ => false,
    };
    if agreed {
        None
    } else {
        Some(describe(form))
    }
}

fn array2(cells: &[String]) -> Option<[&String; 2]> {
    match cells {
        [a, b] => Some([a, b]),
        _ => None,
    }
}

fn array3(cells: &[String]) -> Option<[&String; 3]> {
    match cells {
        [a, b, c] => Some([a, b, c]),
        _ => None,
    }
}

// ── leg 2: the corpus is the population ────────────────────────────────────────────────────────

/// The repository root, derived from this crate's manifest directory.
fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Every `.eadl` description the repository ships, as `(repo-relative path, text)`.
fn corpus() -> Vec<(String, String)> {
    let root = repo_root();
    let mut found = Vec::new();
    let mut stack = vec![root.join("docs/semantics"), root.join("examples")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path
                .extension()
                .is_some_and(|extension| extension == "eadl")
            {
                let relative = path
                    .strip_prefix(&root)
                    .expect("inside the repository")
                    .display()
                    .to_string();
                found.push((
                    relative,
                    std::fs::read_to_string(&path).expect("a description is readable"),
                ));
            }
        }
    }
    found.sort();
    found
}

/// Every numeric and string atom in a form tree, as it is **spelled in the source**.
///
/// The spelling comes from the atom's own span, so this leg asks the corpus what it contains rather
/// than guessing with a regular expression — a second tokenizer here would be a second thing to be
/// wrong about.
fn literal_atoms(forms: &[Form], text: &str) -> Vec<(char, String)> {
    let mut out = Vec::new();
    let mut stack: Vec<&Form> = forms.iter().collect();
    while let Some(form) = stack.pop() {
        match form {
            Form::List { items, .. } => stack.extend(items.iter()),
            Form::Integer { span, .. } | Form::Decimal { span, .. } => {
                out.push(('n', spelled(text, span.start, span.end)));
            }
            Form::Str { span, .. } => out.push(('s', spelled(text, span.start, span.end))),
            Form::Symbol { .. } => {}
        }
    }
    out
}

fn spelled(text: &str, start: u32, end: u32) -> String {
    text.get(start as usize..end as usize)
        .unwrap_or("<out of range>")
        .to_string()
}

/// Leg 2 — a literal the repository ships and the reference does not state is a gap, not a spare.
fn coverage_violations(document: &str) -> Vec<String> {
    let stated_numbers: BTreeSet<String> = machine_table(document, "number-values")
        .iter()
        .filter_map(|(_, cells)| cells.first().cloned())
        .collect();
    let stated_strings: BTreeSet<String> = machine_table(document, "string-values")
        .iter()
        .filter_map(|(_, cells)| cells.first().cloned())
        .collect();

    let mut out = Vec::new();
    let mut seen: BTreeSet<(char, String)> = BTreeSet::new();
    for (name, text) in corpus() {
        let mut sources = SourceMap::new();
        let id = match sources.add(name.clone(), text.clone()) {
            Ok(id) => id,
            Err(_) => continue,
        };
        let (parsed, _) = read(&sources, id);
        for (class, spelling) in literal_atoms(&parsed.forms, &text) {
            if !seen.insert((class, spelling.clone())) {
                continue;
            }
            match class {
                'n' if !stated_numbers.contains(&spelling) => out.push(format!(
                    "{name} writes the numeric literal `{spelling}` and the reference's number \
                     table has no row for it, so nothing states what it is worth"
                )),
                // §2 rule 1 covers a string with no backslash in it: it denotes exactly its
                // characters. Only a string that escapes something needs a row of its own.
                's' if spelling.contains('\\') && !stated_strings.contains(&spelling) => {
                    out.push(format!(
                        "{name} writes the string {spelling} and the reference's string table has \
                         no row for it, so nothing states what its escapes denote"
                    ))
                }
                _ => {}
            }
        }
    }
    out
}

// ── leg 3: the tables are not vacuous ──────────────────────────────────────────────────────────

/// Leg 3 — a table that stopped being a table is reported, not skipped.
fn vacuity_violations(document: &str) -> Vec<String> {
    let mut out = Vec::new();
    let numbers = machine_table(document, "number-values");
    if numbers.is_empty() {
        out.push(
            "docs/semantics/reference.md carries no `<!-- machine-read: number-values -->` table, so \
             nothing in it states what a number is worth"
                .to_string(),
        );
    } else {
        let classes: BTreeSet<&str> = numbers
            .iter()
            .filter_map(|(_, cells)| cells.get(1))
            .filter_map(|cell| Value::parse(cell))
            .map(|value| value.class())
            .collect();
        for class in ["integer", "rational", "symbol", "error"] {
            if !classes.contains(class) {
                out.push(format!(
                    "docs/semantics/reference.md: the number table has no `{class}` row, so no row \
                     pins that verdict and a frontend that got every `{class}` wrong would pass"
                ));
            }
        }
    }

    let strings = machine_table(document, "string-values");
    if strings.is_empty() {
        out.push(
            "docs/semantics/reference.md carries no `<!-- machine-read: string-values -->` table, so \
             nothing in it states what an escape denotes"
                .to_string(),
        );
    } else {
        let values: Vec<&str> = strings
            .iter()
            .filter_map(|(_, cells)| cells.get(1).map(String::as_str))
            .collect();
        if !values.iter().any(|cell| cell.starts_with("error ")) {
            out.push(
                "docs/semantics/reference.md: the string table refuses nothing, so no row pins that \
                 an unsupported escape is an error rather than a value"
                    .to_string(),
            );
        }
        if !values
            .iter()
            .any(|cell| !cell.starts_with("error ") && cell.contains('\\'))
        {
            out.push(
                "docs/semantics/reference.md: the string table decodes no escape, so no row pins \
                 what a backslash denotes"
                    .to_string(),
            );
        }
    }
    out
}

/// Every leg at once, so an arm can be fed a mutated document and read one list of complaints.
fn all_violations(document: &str) -> Vec<String> {
    let mut out = vacuity_violations(document);
    out.extend(number_violations(document));
    out.extend(string_violations(document));
    out.extend(coverage_violations(document));
    out.sort();
    out
}

// ── the green legs ─────────────────────────────────────────────────────────────────────────────

#[test]
fn the_reference_value_tables_are_the_frontend_s_verdict() {
    let wrong = all_violations(REFERENCE);
    assert!(
        wrong.is_empty(),
        "the language reference and the frontend disagree:\n\n{}\n\n\
         A row is normative: either the frontend is wrong, or the reference states a rule the \
         language does not have and the row must change with a decision recorded behind it.",
        wrong.join("\n\n")
    );
}

#[test]
fn the_reference_states_a_value_for_every_literal_the_repository_ships() {
    let wrong = coverage_violations(REFERENCE);
    assert!(
        wrong.is_empty(),
        "the corpus uses literals the reference does not state:\n\n{}",
        wrong.join("\n\n")
    );
}

#[test]
fn the_reference_tables_are_not_vacuous() {
    let wrong = vacuity_violations(REFERENCE);
    assert!(
        wrong.is_empty(),
        "a table that pins nothing is worse than no table, because it looks like a check:\n\n{}",
        wrong.join("\n\n")
    );
}

// ── RED arms ───────────────────────────────────────────────────────────────────────────────────
//
// Each arm feeds the legs a mutation of the real document and asserts the specific complaint, with
// the count pinned: an arm that only checks "some complaint mentions X" also passes on a gate that
// started reporting everything, which is the vacuous-pass shape `conformance.rs` refuses for the
// grammar. Every mutation is confirmed applied before the verdict is read — `M1.23`'s arm C reported
// a pass because its substitution never matched.

/// Assert the legs reported exactly `expected` violations, with every `needle` among them.
fn assert_reported(wrong: &[String], expected: usize, needles: &[&str]) {
    assert_eq!(
        wrong.len(),
        expected,
        "expected {expected} violation(s); the legs reported:\n{}",
        wrong.join("\n\n")
    );
    for needle in needles {
        assert!(
            wrong.iter().any(|item| item.contains(needle)),
            "no violation mentions {needle:?}; the legs reported:\n{}",
            wrong.join("\n\n")
        );
    }
}

/// Replace one whole line of the document, asserting the line was there to replace.
fn replacing_line(document: &str, needle: &str, replacement: &str) -> String {
    let mut hits = 0;
    let out = document
        .lines()
        .map(|line| {
            if line.contains(needle) {
                hits += 1;
                replacement.to_string()
            } else {
                line.to_string()
            }
        })
        .collect::<Vec<_>>()
        .join("\n");
    assert_eq!(hits, 1, "{needle:?} is not one line of the document");
    out
}

#[test]
fn arm_1_a_wrong_value_in_the_number_table_is_reported() {
    let mutated = REFERENCE.replace("`integer 268435456`", "`integer 268435457`");
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    assert_reported(
        &all_violations(&mutated),
        1,
        &["`0x1000_0000`", "integer 268435457", "integer 268435456"],
    );
}

#[test]
fn arm_2_a_wrong_canonical_text_is_reported() {
    // The reference prints a value, never a spelling; a row that says otherwise is the claim §3
    // rule 1 makes, and this arm is what keeps it a claim about the frontend rather than prose.
    let mutated = replacing_line(
        REFERENCE,
        "| `0x1000_0000` |",
        "| `0x1000_0000` | `integer 268435456` | `0x1000_0000` |",
    );
    assert_reported(
        &all_violations(&mutated),
        1,
        &["canonical text `0x1000_0000`", "printed `268435456`"],
    );
}

#[test]
fn arm_3_deleting_a_row_the_corpus_needs_is_reported() {
    // ⭐ The "a row was deleted" arm, and the reason leg 2 exists: nothing can notice an absent row
    // by reading the rows that are there, so the population has to come from somewhere the document
    // does not define. `0x1000_0000` is in `examples/periodic-three/system.eadl`.
    let mutated = REFERENCE
        .lines()
        .filter(|line| !line.starts_with("| `0x1000_0000` |"))
        .collect::<Vec<_>>()
        .join("\n");
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    assert_reported(
        &all_violations(&mutated),
        1,
        &["numeric literal `0x1000_0000`", "no row for it"],
    );
}

#[test]
fn arm_4_a_document_that_stops_carrying_a_table_is_reported_rather_than_skipped() {
    let mutated = REFERENCE.replace("<!-- machine-read: number-values -->", "<!-- numbers -->");
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    // One complaint for the missing table, and one for each numeric literal the corpus ships that
    // the vanished table no longer states.
    let corpus_numbers = coverage_violations("<!-- numbers -->")
        .iter()
        .filter(|item| item.contains("numeric literal"))
        .count();
    let wrong = all_violations(&mutated);
    assert!(
        wrong
            .iter()
            .any(|item| item.contains("carries no `<!-- machine-read: number-values -->` table")),
        "the legs did not name the missing table:\n{}",
        wrong.join("\n\n")
    );
    assert!(
        wrong.len() > corpus_numbers,
        "the legs reported only the coverage rows, so a renamed marker would read as a smaller gap \
         rather than as no table at all:\n{}",
        wrong.join("\n\n")
    );
}

#[test]
fn arm_5_a_wrong_decoded_string_value_is_reported() {
    // F-D's shape, stated as a value rather than as canonical text: if `\r` denoted a line feed the
    // decoded value would be wrong before any printer was involved. Two complaints, and both honest —
    // the value disagrees, and so does the canonical text §3 derives from it.
    let mutated = replacing_line(REFERENCE, "| `\"a\\rb\"` |", "| `\"a\\rb\"` | `a\\nb` |");
    assert_reported(
        &all_violations(&mutated),
        2,
        &["\"a\\rb\"", "decoded value", "canonical text"],
    );
}

#[test]
fn arm_6_canonical_text_carrying_a_control_character_is_reported() {
    // The predicate leg 1 rests on, armed directly: the defect it exists for (finding F-D) was a raw
    // carriage return inside the text §12 M4 hashes, and after the fix no row can reproduce it.
    assert_eq!(
        control_character("(a \"x\\ry\")"),
        None,
        "an escape is not a control character"
    );
    assert_eq!(control_character("(a \"x\ry\")"), Some('\r'));
    assert_eq!(control_character("(a \"x\ty\")"), Some('\t'));
    assert_eq!(control_character("(a \"x\ny\")"), Some('\n'));
    assert_eq!(encode_canonical("a\rb"), "\"a\\rb\"");
    assert_ne!(
        encode_canonical("a\rb"),
        format!("\"a{}b\"", '\r'),
        "a printer that emits the character itself is what this forbids"
    );
}

#[test]
fn arm_7_a_cell_the_notation_cannot_express_is_reported_rather_than_skipped() {
    let mutated = REFERENCE.replace("`integer 268435456`", "`int 268435456`");
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    assert_reported(
        &all_violations(&mutated),
        1,
        &[
            "`int 268435456` is not a value",
            "nothing checks `0x1000_0000`",
        ],
    );
}

#[test]
fn arm_8_a_literal_the_reference_refuses_but_the_frontend_accepts_is_reported() {
    // The shape of finding F-B, which fired on the real tree before the reader was fixed: the
    // reference refused `0x_10` and the reader read it as 16.
    let mutated = replacing_line(REFERENCE, "| `0xg` |", "| `0xg` | `integer 0` | `0` |");
    assert_reported(
        &all_violations(&mutated),
        1,
        &["`0xg`", "read-malformed-number"],
    );
}
