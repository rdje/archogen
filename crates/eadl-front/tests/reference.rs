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
//! 1. **EXECUTED** — every row of both literal tables, against the reader: verdict, exact value,
//!    canonical text, and the property that canonical text carries no control character.
//! 2. **COVERING** — every numeric literal the repository actually ships has a row, and so does every
//!    string that carries a backslash. The population is the corpus, which this file does not define,
//!    so the leg cannot be satisfied by writing a table that agrees with itself.
//! 3. **NON-VACUOUS** — each verdict class the notation defines is pinned by at least one row, and a
//!    document that stops carrying a table is reported rather than silently gating nothing.
//! 4. **CENSUSED, producer → reference** — every diagnostic code the sources this document *declares
//!    itself normative over* can emit is stated in §4, with the repair direction §5.5 requires. The
//!    scan is multi-line safe and stops at `#[cfg(test)]`; measured, a same-line pattern finds **zero**
//!    codes in every source here, so multi-line tolerance is the census and not a refinement of it.
//! 5. **CENSUSED, reference → producer** — every code §4 states is one a declared source emits, so a
//!    renamed code leaves a rotted row instead of a stale truth. ⭐ Legs 4 and 5 together pin the set
//!    *exactly*: a code the scan missed would be stated-but-unemitted, and a code it invented would be
//!    emitted-but-unstated — so a green run is evidence about the scanner, not only about the table.
//! 6. **ANCHORED** — every repository path the reference cites resolves, which is the leg
//!    `scripts/check_book_anchors.sh` runs on the book, applied to a normative document.
//! 7. **HEADERS** — every case in §5's comment-header table, run against `Document::comment_headers`,
//!    including the cases that must yield *no* header: a table of successful parses alone would pass
//!    on an implementation that treated every comment as one.
//!
//! ⚠️ **Honest limit.** A deleted row is caught only when the corpus used that literal, or when it was
//! the last row of its verdict class; the literal *forms* the language admits are enumerated by
//! `conformance.rs`, which runs these same tables against the recognizer derived from
//! `docs/semantics/grammar.md`. This file also says nothing about nesting or meaning — `corpus.rs`
//! owns those — and §6's and §7's rules are prose that cites the tests enforcing them, because a
//! module elaboration or a schema check is not a row in a table of literals.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use eadl_front::{read, Form, Severity, SourceMap};

mod common;

use common::reference_table::{
    control_character, decode_escapes, encode_canonical, machine_table, StringValue, Value,
};

/// The normative document under test.
///
/// `include_str!` rather than `fs::read_to_string`, for the reason `corpus.rs` gives for its live
/// surfaces: if the reference moves or is deleted, this crate **stops compiling** instead of silently
/// gating nothing.
const REFERENCE: &str = include_str!("../../../docs/semantics/reference.md");

// ── the value notation, and reading the tables out of the document ─────────────────────────────
//
// Both live in `tests/common/reference_table.rs`, because `conformance.rs` reads the same tables to
// compare them with the recognizer derived from `docs/semantics/grammar.md`. One normative document,
// one reader for it.

/// One violation, phrased so the failure names the row and both sides.
fn violation(line: usize, message: String) -> String {
    format!("docs/semantics/reference.md:{line}: {message}")
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
    /// The `; key: value` headers the comments yielded, in source order.
    headers: Vec<(String, String)>,
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
    let headers = document.comment_headers();
    Probe {
        headers,
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

/// §3's round-trip guarantee, checked on the canonical text the frontend just printed.
///
/// ⭐ Not a duplicate of the canonical-text comparison, and finding F-D is the reason. A printer that
/// emitted a raw carriage return still produced a *form* equal to the row's value: the defect was in
/// the text, and only re-reading that text catches a printer whose output the reader cannot read back.
fn round_trip_violation(line: usize, literal: &str, printed: &str) -> Option<String> {
    let before = probe(literal);
    let again = probe(printed);
    if !again.errors.is_empty() {
        return Some(violation(
            line,
            format!(
                "`{literal}`: the frontend printed canonical text it cannot read back:\n{}",
                again.rendered
            ),
        ));
    }
    let (Some(before), Some(after)) = (before.forms.first(), again.forms.first()) else {
        return Some(violation(
            line,
            format!("`{literal}`: canonical text `{printed}` did not read back to a form"),
        ));
    };
    if !before.structurally_eq(after) {
        return Some(violation(
            line,
            format!(
                "`{literal}`: canonical form is not a fixed point — {} reads back as {}",
                describe(before),
                describe(after)
            ),
        ));
    }
    None
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
                     `rational N/10^S`, `symbol T`, `error C`, `refused C`), so nothing checks \
                     `{literal}`"
                ),
            ));
            continue;
        };
        let found = probe(literal);
        if let Some(code) = expected.refusal() {
            if !found.errors.contains(&code) {
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
        if let Some(message) = round_trip_violation(line, literal, &printed) {
            out.push(message);
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
        let expected = match StringValue::parse(stated) {
            None => {
                out.push(violation(
                    line,
                    format!(
                        "`{stated}` is not writable in the reference's own notation (`error C`, \
                         `refused C`, or the decoded text in the language's escapes), so nothing \
                         checks {source}"
                    ),
                ));
                continue;
            }
            Some(StringValue::Error(code)) | Some(StringValue::Refused(code)) => {
                if !found.errors.contains(&code.as_str()) {
                    out.push(violation(
                        line,
                        format!(
                            "the reference refuses {source} as `{code}`, and the frontend did \
                             not:\n{}",
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
            Some(StringValue::Text(text)) => text,
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
        if let Some(message) = round_trip_violation(line, source, &printed) {
            out.push(message);
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
        Value::Refused(c) => format!("`refused {c}`"),
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
        for class in ["integer", "rational", "symbol", "error", "refused"] {
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
        let values: Vec<StringValue> = strings
            .iter()
            .filter_map(|(_, cells)| cells.get(1))
            .filter_map(|cell| StringValue::parse(cell))
            .collect();
        if !values
            .iter()
            .any(|value| matches!(value, StringValue::Error(_)))
        {
            out.push(
                "docs/semantics/reference.md: the string table refuses nothing as malformed, so no \
                 row pins that an unsupported escape is an error rather than a value"
                    .to_string(),
            );
        }
        if !values.iter().any(|value| {
            matches!(value, StringValue::Text(text) if text.contains(['\n', '\t', '\r', '"', '\\']))
        }) {
            out.push(
                "docs/semantics/reference.md: the string table decodes no escape, so no row pins \
                 what a backslash denotes"
                    .to_string(),
            );
        }
    }
    out
}

// ── leg 4: the diagnostic-code census, both directions ─────────────────────────────────────────

/// Every diagnostic a source's **production half** can emit, as `(severity, code)`.
///
/// ⛔ **Multi-line safe, and both halves of that sentence are measured rather than assumed.**
/// A pattern that required the code on the same line as its constructor — the grep someone reaches
/// for, `Diagnostic::error("…"` — finds **zero** codes in every source this census can declare:
/// `reader.rs` 0 of 7, `module.rs` 0 of 24, `kind.rs` 0 of 22. Every call in this codebase puts the
/// code on the next line, so a same-line census does not under-count, it counts nothing, and the leg
/// built on it would pass by comparing two empty sets. That is what the `emitted.is_empty()` guard
/// below is for.
///
/// ⛔ **Stopping at `#[cfg(test)]` is a rule about the population, not a fix for today's files.** For
/// the two sources declared right now the cut changes nothing (measured). It is load-bearing for
/// `crates/eadl-front/src/diagnostic.rs`, whose test half would otherwise contribute `read-example`
/// and `read-unclosed-list` alongside `e` and `w` — two test locals that are not codes at all.
/// Documenting a test-only code would add a row no description can ever falsify with an input.
fn emitted_diagnostics(text: &str) -> BTreeSet<(String, String)> {
    let production = text
        .split_once("#[cfg(test)]")
        .map_or(text, |(head, _)| head);
    let mut out = BTreeSet::new();
    for (at, _) in production.match_indices("Diagnostic::") {
        let rest = &production[at + "Diagnostic::".len()..];
        let Some(severity) = ["error", "warning", "note"]
            .into_iter()
            .find(|word| rest.starts_with(*word))
        else {
            continue;
        };
        let after = &rest[severity.len()..];
        // The code is the first string literal after the opening parenthesis, which is usually on a
        // later line. Everything between is whitespace and newlines, so scanning rather than matching
        // a fixed shape is what makes this leg see the whole population.
        let (Some(open), Some(quote)) = (after.find('('), after.find('"')) else {
            continue;
        };
        if quote < open {
            continue;
        }
        let literal = &after[quote + 1..];
        let Some(end) = literal.find('"') else {
            continue;
        };
        let code = &literal[..end];
        if !code.is_empty()
            && code
                .chars()
                .all(|ch| ch.is_ascii_lowercase() || ch.is_ascii_digit() || ch == '-')
        {
            out.insert((severity.to_string(), code.to_string()));
        }
    }
    out
}

/// The sources the reference declares itself normative over, with their line numbers.
fn declared_sources(document: &str) -> Vec<(usize, String)> {
    machine_table(document, "normative-sources")
        .iter()
        .filter_map(|(line, cells)| cells.first().map(|path| (*line, path.clone())))
        .collect()
}

/// Legs 4 and 5 — every code the declared sources emit is stated, and every code stated is emitted.
fn census_violations(document: &str) -> Vec<String> {
    let mut out = Vec::new();
    let sources = declared_sources(document);
    if sources.is_empty() {
        out.push(
            "docs/semantics/reference.md declares no normative source, so the diagnostic census has \
             no population and proves nothing"
                .to_string(),
        );
    }
    let rows = machine_table(document, "diagnostics");
    if rows.is_empty() {
        out.push(
            "docs/semantics/reference.md carries no `<!-- machine-read: diagnostics -->` table, so no \
             diagnostic code is stated anywhere in it"
                .to_string(),
        );
    }

    let mut stated = BTreeSet::new();
    for (line, cells) in &rows {
        let Some(code) = cells.first() else { continue };
        stated.insert(code.clone());
        if cells.len() != 3 {
            out.push(violation(
                *line,
                format!(
                    "the row for `{code}` has {} cell(s) and the table's header has 3",
                    cells.len()
                ),
            ));
        } else if cells[2].trim().is_empty() || cells[2].trim() == "—" {
            // §5.5 requires a repair direction, and a table is where that requirement can be checked
            // instead of merely stated.
            out.push(violation(
                *line,
                format!(
                    "`{code}` states no repair direction, which §5.5 requires of every diagnostic — \
                     a refusal that does not say what to do costs an author an edit cycle"
                ),
            ));
        }
    }

    let mut emitted: BTreeSet<String> = BTreeSet::new();
    for (line, path) in &sources {
        let full = repo_root().join(path);
        let text = match std::fs::read_to_string(&full) {
            Ok(text) => text,
            Err(error) => {
                out.push(violation(
                    *line,
                    format!(
                        "`{path}` is declared as a normative source and cannot be read: {error}"
                    ),
                ));
                continue;
            }
        };
        for (severity, code) in emitted_diagnostics(&text) {
            if severity != "error" {
                out.push(violation(
                    *line,
                    format!(
                        "`{path}` emits `{code}` at severity `{severity}`, and §4 rule 1 states that \
                         every diagnostic the frontend emits is an error"
                    ),
                ));
            }
            if emitted.insert(code.clone()) && !stated.contains(&code) {
                out.push(violation(
                    *line,
                    format!(
                        "`{path}` can emit `{code}` and §4 does not state it — a rule the frontend \
                         enforces and this reference does not"
                    ),
                ));
            }
        }
    }
    for (line, cells) in &rows {
        let Some(code) = cells.first() else { continue };
        if !emitted.contains(code) {
            out.push(violation(
                *line,
                format!(
                    "§4 states `{code}` and no declared source emits it — a renamed or removed code \
                     leaves a rotted row that reads exactly like a live rule"
                ),
            ));
        }
    }
    if !sources.is_empty() && emitted.is_empty() {
        out.push(
            "the declared sources emit no diagnostic code at all, so the census compared nothing"
                .to_string(),
        );
    }
    out
}

// ── leg 6: the reference's own citations resolve ───────────────────────────────────────────────

/// The top-level directories whose contents are unambiguously claims about this repository.
///
/// Mirrors `scripts/check_book_anchors.sh`'s `PATH_RE` rather than inventing a second notion of what
/// counts as a citation. Matching a repository path and not any filename is deliberate: a chapter may
/// legitimately name `src/main.rs` of a *generated* crate, and requiring that to resolve would teach
/// authors to route around the gate.
const REPO_DIRS: [&str; 7] = [
    "crates/",
    "scripts/",
    "examples/",
    "docs/",
    "xtask/",
    "targets/",
    "knowledge-map/",
];

/// The root documents that are cited by name rather than by path.
const ROOT_DOCS: [&str; 14] = [
    "ROADMAP.md",
    "COMMIT.md",
    "TOOLBOX.md",
    "MEMORY.md",
    "LIVE_STATUS.md",
    "CHANGELOG.md",
    "DEV_NOTES.md",
    "README.md",
    "README_POLICY.md",
    "DOCTRINE_ENFORCEMENT.md",
    "MEMORY_ARCHITECTURE.md",
    "KNOWLEDGE_MAP.md",
    "AGENTS.md",
    "CLAUDE.md",
];

/// Every repository path this document cites inside a code span, with line numbers.
fn cited_paths(document: &str) -> Vec<(usize, String)> {
    let mut out = Vec::new();
    for (index, line) in document.lines().enumerate() {
        // Odd-numbered pieces of a backtick split are inside a code span.
        for (position, span) in line.split('`').enumerate() {
            if position % 2 == 0 {
                continue;
            }
            if REPO_DIRS.iter().any(|dir| span.starts_with(dir)) || ROOT_DOCS.contains(&span) {
                out.push((index + 1, span.to_string()));
            }
        }
    }
    out
}

/// Leg 6 — a normative document's rotted citation reads exactly like a live one.
fn citation_violations(document: &str) -> Vec<String> {
    let root = repo_root();
    let mut out = Vec::new();
    let mut seen = BTreeSet::new();
    for (line, path) in cited_paths(document) {
        if !seen.insert(path.clone()) {
            continue;
        }
        // A trailing slash names a directory and a trailing `/*` a glob; both resolve to their parent.
        let probe = path.trim_end_matches("/*").trim_end_matches('/');
        if !root.join(probe).exists() {
            out.push(violation(
                line,
                format!(
                    "this file cites `{path}`, which does not exist — the reference is normative, so a \
                     rotted citation sends its reader to nothing"
                ),
            ));
        }
    }
    if seen.is_empty() {
        out.push(
            "docs/semantics/reference.md cites no repository path at all, so nothing in it can be \
             checked against the code it describes"
                .to_string(),
        );
    }
    out
}

// ── leg 7: the header convention, executed ─────────────────────────────────────────────────────

/// Leg 7 — every case in the comment-header table, run against `Document::comment_headers`.
///
/// Rows that share a source cell are one case, and their order is the order the headers must come out
/// in. A case whose rows are all `—` states that the block yields **no header at all**, which is the
/// half of §5's rule a table of successful parses could never pin.
fn header_violations(document: &str) -> Vec<String> {
    let mut out = Vec::new();
    let rows = machine_table(document, "comment-headers");
    if rows.is_empty() {
        out.push(
            "docs/semantics/reference.md carries no `<!-- machine-read: comment-headers -->` table, \
             so §5's header convention is prose and nothing executes it"
                .to_string(),
        );
        return out;
    }
    let mut order: Vec<String> = Vec::new();
    let mut cases: BTreeMap<String, Vec<(usize, String, String)>> = BTreeMap::new();
    for (line, cells) in &rows {
        let Some([cell_source, key, value]) = array3(cells) else {
            out.push(violation(
                *line,
                format!(
                    "this row has {} cell(s) and the table's header has 3",
                    cells.len()
                ),
            ));
            continue;
        };
        let Some(source) = decode_escapes(cell_source) else {
            out.push(violation(
                *line,
                format!("`{cell_source}` is not writable in the reference's own escape notation"),
            ));
            continue;
        };
        if !cases.contains_key(&source) {
            order.push(source.clone());
        }
        cases
            .entry(source)
            .or_default()
            .push((*line, key.clone(), value.clone()));
    }

    let mut yielded = 0;
    let mut refused = 0;
    for source in &order {
        let expected = &cases[source];
        let line = expected[0].0;
        let found = probe(source);
        let shown = source.replace('\n', "\\n");
        if expected
            .iter()
            .all(|(_, key, value)| key == "—" && value == "—")
        {
            refused += 1;
            if !found.headers.is_empty() {
                out.push(violation(
                    line,
                    format!(
                        "§5 says `{shown}` yields no header, and `comment_headers` yielded {:?}",
                        found.headers
                    ),
                ));
            }
            continue;
        }
        yielded += 1;
        let want: Vec<(String, String)> = expected
            .iter()
            .map(|(_, key, value)| (key.clone(), value.clone()))
            .collect();
        if found.headers != want {
            out.push(violation(
                line,
                format!(
                    "§5 says `{shown}` yields {want:?}, and `comment_headers` yielded {:?}",
                    found.headers
                ),
            ));
        }
    }
    // ⛔ Not vacuous in either direction. A table that only produced headers would pass on a
    // `comment_headers` treating every comment as one; a table that only refused would pass on one
    // that returned nothing. §5's rule is two discriminators *and* a continuation, so both outcomes
    // have to be pinned or the leg proves one of them.
    if yielded == 0 || refused == 0 {
        out.push(format!(
            "docs/semantics/reference.md: the header table pins {yielded} case(s) that yield a header \
             and {refused} that do not, so one of the two outcomes is unchecked"
        ));
    }
    out
}

/// Every leg at once, so an arm can be fed a mutated document and read one list of complaints.
fn all_violations(document: &str) -> Vec<String> {
    let mut out = vacuity_violations(document);
    out.extend(number_violations(document));
    out.extend(string_violations(document));
    out.extend(coverage_violations(document));
    out.extend(census_violations(document));
    out.extend(citation_violations(document));
    out.extend(header_violations(document));
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

#[test]
fn the_reference_states_every_diagnostic_its_declared_sources_can_emit() {
    let wrong = census_violations(REFERENCE);
    assert!(
        wrong.is_empty(),
        "the diagnostic census disagrees with the reference, in one direction or both:\n\n{}\n\n\
         The population is the code, taken from the sources the reference itself declares — not a \
         list inside this file, which is the drift the declaration exists to prevent.",
        wrong.join("\n\n")
    );
}

#[test]
fn every_repository_path_the_reference_cites_resolves() {
    let wrong = citation_violations(REFERENCE);
    assert!(
        wrong.is_empty(),
        "the reference cites something that is not there:\n\n{}",
        wrong.join("\n\n")
    );
}

#[test]
fn the_reference_header_table_is_the_frontend_s_verdict() {
    let wrong = header_violations(REFERENCE);
    assert!(
        wrong.is_empty(),
        "§5's header convention and `comment_headers` disagree:\n\n{}\n\n\
         The convention is how a description states facts about itself, and F27 reads those facts — \
         a header that silently moves between fields is a verdict attached to the wrong case.",
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

#[test]
fn arm_9_a_code_the_frontend_emits_but_the_reference_does_not_state_is_reported() {
    // Census leg 1, producer → reference. Deleting the row is the defect: the frontend still refuses
    // out-of-range literals, and nothing would say so.
    let mutated = REFERENCE
        .lines()
        .filter(|line| !line.starts_with("| `read-number-overflow` |"))
        .collect::<Vec<_>>()
        .join("\n");
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    assert_reported(
        &census_violations(&mutated),
        1,
        &[
            "can emit `read-number-overflow`",
            "§4 does not state it",
            "crates/eadl-front/src/reader.rs",
        ],
    );
}

#[test]
fn arm_10_a_code_the_reference_states_but_nothing_emits_is_reported() {
    // Census leg 2, reference → producer: a renamed code leaves a rotted row that reads exactly like
    // a live rule. Two complaints, and both are honest — the typo is stated-but-unemitted, and the
    // real code the typo replaced is now emitted-but-unstated.
    let mutated = replacing_line(
        REFERENCE,
        "| `read-number-overflow` |",
        "| `read-number-overflowed` | a well-formed literal whose value lies outside the range | reduce the digits |",
    );
    assert_reported(
        &census_violations(&mutated),
        2,
        &[
            "§4 states `read-number-overflowed` and no declared source emits it",
            "can emit `read-number-overflow`",
        ],
    );
}

#[test]
fn arm_11_the_census_sees_a_code_written_on_the_line_after_its_constructor() {
    // ⛔ The census must not be a population bounded by one spelling. Measured over this repository:
    // a pattern requiring the code on the constructor's own line finds **zero** codes in `reader.rs`
    // (0 of 7), `module.rs` (0 of 24) and `kind.rs` (0 of 22), because every call puts the code on
    // the next line. The test-only half is excluded for the opposite reason: `diagnostic.rs`'s tests
    // construct `read-example`, and documenting a code no description can trigger would add a row
    // nothing could ever falsify with an input.
    let multi_line = "fn f(&mut self) {\n    self.diagnostics.push(Diagnostic::error(\n        \"read-late-code\",\n        \"message\",\n        Label::new(span, \"label\"),\n        \"hint\",\n    ));\n}\n";
    let found = emitted_diagnostics(multi_line);
    assert!(
        found.contains(&("error".to_string(), "read-late-code".to_string())),
        "a code on the line after its constructor was not seen: {found:?}"
    );

    let same_line = r#"Diagnostic::error("read-same-line", "message", label, "hint")"#;
    assert!(
        emitted_diagnostics(same_line)
            .contains(&("error".to_string(), "read-same-line".to_string())),
        "a same-line code was not seen"
    );

    let with_tests = "Diagnostic::error(\"read-production\", \"m\", l, \"h\")\n\n#[cfg(test)]\nmod tests {\n    Diagnostic::error(\"read-example\", \"m\", l, \"h\")\n}\n";
    let found = emitted_diagnostics(with_tests);
    assert!(
        found.contains(&("error".to_string(), "read-production".to_string())),
        "the production half was not scanned: {found:?}"
    );
    assert!(
        !found.iter().any(|(_, code)| code == "read-example"),
        "a test-only code was counted as one the language can emit: {found:?}"
    );

    // Severity is part of the census, because §4 rule 1 claims every diagnostic is an error.
    assert!(
        emitted_diagnostics(r#"Diagnostic::warning("module-slow", "m", l, "h")"#)
            .contains(&("warning".to_string(), "module-slow".to_string())),
        "a non-error severity was not recorded, so rule 1 would be checked by nothing"
    );
}

#[test]
fn arm_12_a_rotted_citation_is_reported() {
    let mutated = REFERENCE.replace(
        "`crates/archogen-s0/src/provenance.rs`",
        "`crates/archogen-s0/src/provenence.rs`",
    );
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    assert_reported(
        &citation_violations(&mutated),
        1,
        &["provenence.rs", "does not exist"],
    );
}

#[test]
fn arm_13_a_diagnostic_with_no_repair_direction_is_reported() {
    // §5.5 requires every diagnostic to say what to do about it. That requirement is checkable in a
    // table and is checked here, rather than being a sentence a new row can quietly ignore.
    let mutated = replacing_line(
        REFERENCE,
        "| `read-unexpected-close` |",
        "| `read-unexpected-close` | a `)` appears with no list open | — |",
    );
    assert_reported(
        &census_violations(&mutated),
        1,
        &["`read-unexpected-close` states no repair direction", "§5.5"],
    );
}

#[test]
fn arm_14_a_document_that_stops_declaring_its_sources_is_reported() {
    let mutated = REFERENCE.replace(
        "<!-- machine-read: normative-sources -->",
        "<!-- sources -->",
    );
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    // One complaint for the missing declaration, and one per stated code that no longer has a
    // population to be compared against — so a renamed marker reads as a hole, not as a clean sweep.
    let stated = machine_table(REFERENCE, "diagnostics").len();
    let wrong = census_violations(&mutated);
    assert_eq!(
        wrong.len(),
        stated + 1,
        "expected one complaint for the missing declaration plus one per stated code; got:\n{}",
        wrong.join("\n\n")
    );
    assert!(
        wrong
            .iter()
            .any(|item| item.contains("declares no normative source")),
        "the legs did not name the missing declaration:\n{}",
        wrong.join("\n\n")
    );
}

#[test]
fn arm_15_a_wrong_header_value_is_reported() {
    let mutated = replacing_line(
        REFERENCE,
        "| `; case: counter-width-and-rate` |",
        "| `; case: counter-width-and-rate` | `case` | `counter-width` |",
    );
    assert_reported(
        &header_violations(&mutated),
        1,
        &["counter-width", "`comment_headers` yielded"],
    );
}

#[test]
fn arm_16_the_indentation_discriminator_is_load_bearing() {
    // ⛔ §5 rule 5's first case, from `counter-width-and-rate.eadl`: a wrapped rationale line whose
    // text is *exactly* a well-shaped key followed by a colon. Take its indentation away and the line
    // opens a spurious header and truncates the value — the defect `corpus.rs` holds a test for,
    // reproduced here from the table alone.
    let mutated = REFERENCE.replace(
        ";   implementation-independence: a different timer is",
        "; implementation-independence: a different timer is",
    );
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    let wrong = header_violations(&mutated);
    assert_eq!(
        wrong.len(),
        1,
        "expected one violation; got:\n{}",
        wrong.join("\n\n")
    );
    assert!(
        wrong[0].contains("implementation-independence"),
        "the violation does not name the spurious header:\n{}",
        wrong[0]
    );
}

#[test]
fn arm_17_the_key_shape_discriminator_is_load_bearing() {
    // ⛔ §5 rule 5's second case, from `examples/bounded-queue/system.eadl`: prose at one space of
    // indentation, where only the capital in `Expected` stops it becoming a field.
    let mutated = REFERENCE.replace(
        "; Expected: unsupported-profile",
        "; expected: unsupported-profile",
    );
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    assert_reported(
        &header_violations(&mutated),
        1,
        &["yields no header", "expected"],
    );
}

#[test]
fn arm_18_a_document_that_stops_carrying_the_header_table_is_reported() {
    let mutated = REFERENCE.replace("<!-- machine-read: comment-headers -->", "<!-- headers -->");
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    assert_reported(
        &header_violations(&mutated),
        1,
        &["carries no `<!-- machine-read: comment-headers -->` table"],
    );
}
