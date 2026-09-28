//! `literals` — census the literals a population of descriptions actually contains.
//!
//! A measurement instrument, kept in the repository rather than improvised (`TOOLBOX.md`). It answers
//! the questions a value-domain decision needs, about a population the command line names:
//!
//! * what integer values does this corpus write, and how far below the domain limit do they stay?
//! * does **any** description in it hit the limit — i.e. is the domain a live constraint or a stated
//!   one?
//! * which diagnostics did the population reach, so a category that produced nothing is visibly
//!   empty rather than silently unenumerated?
//!
//! ```console
//! $ cargo run -q -p eadl-front --example literals -- docs/semantics examples docs/feedback
//! ```
//!
//! Exit codes: `0` censused, and no literal in the population was refused by the value domain ·
//! `1` censused, and at least one was (`read-number-overflow`) · `2` usage or I/O failure.
//!
//! ⭐ **It asks the frontend, not a regular expression.** A census of literals is a claim about what
//! the *language* contains, and `crates/eadl-front/tests/reference.rs` states why a second tokenizer
//! is the wrong instrument for one: "a second tokenizer here would be a second thing to be wrong
//! about". Every value and every spelling below comes from `eadl_front::read` and from the atom's own
//! span. Provenance: the census behind leaf `M1.13`'s finding M-G was a shell pipeline, published a
//! radix-scoped maximum as the overall one, and its scope ("non-negative") was never stated — seven
//! restatements inherited both errors, because a figure whose producer is a description of a command
//! cannot be re-run and checked.
//!
//! ⛔ **This instrument asserts nothing.** A figure it printed into a document would be a carried
//! figure, and a carried figure is a stale one (`M1.23`, `M1.24`). Run it; quote the run.
//!
//! ⛔ A closed pipe is not a failure, for the reason `diagnose.rs` gives. Every write goes through
//! [`emit`], which exits quietly on a closed pipe and reports any other I/O error honestly.

use std::collections::BTreeMap;
use std::io::{ErrorKind, Write};
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use eadl_front::{read, Diagnostics, Form, SourceMap};

/// The diagnostic code that means "well-formed, but outside the value domain".
const DOMAIN_REFUSAL: &str = "read-number-overflow";

/// Write one line, treating a closed pipe as a normal end of output.
fn emit(out: &mut dyn Write, line: &str) -> Result<(), ExitCode> {
    match writeln!(out, "{line}") {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == ErrorKind::BrokenPipe => Err(ExitCode::SUCCESS),
        Err(error) => {
            let _ = writeln!(std::io::stderr(), "literals: cannot write output: {error}");
            Err(ExitCode::from(2))
        }
    }
}

fn main() -> ExitCode {
    match run() {
        Ok(code) | Err(code) => code,
    }
}

/// One integer literal the census found: what it is worth, how it was spelled, and where.
struct Literal {
    value: i64,
    spelling: String,
    path: String,
}

/// What the walk over one population accumulated.
#[derive(Default)]
struct Census {
    /// Descriptions walked.
    files: usize,
    /// Descriptions the reader produced no diagnostic for.
    clean: usize,
    /// A path that could not be read, with the reason — reported, never skipped.
    unreadable: Vec<(String, String)>,
    /// Every integer literal, in path order.
    integers: Vec<Literal>,
    /// Atom counts by category, so a category with nothing in it is visibly zero.
    decimals: usize,
    strings: usize,
    symbols: usize,
    /// Diagnostics by code, over the whole population.
    codes: BTreeMap<String, usize>,
    /// Every domain refusal, with the spelling that caused it.
    refused: Vec<Literal>,
}

impl Census {
    /// The distinct integer values in the population, ascending.
    fn distinct_values(&self) -> Vec<i64> {
        let mut values: Vec<i64> = self.integers.iter().map(|found| found.value).collect();
        values.sort_unstable();
        values.dedup();
        values
    }
}

fn run() -> Result<ExitCode, ExitCode> {
    let roots: Vec<String> = std::env::args().skip(1).collect();
    if roots.is_empty() {
        eprintln!("usage: literals <root>…");
        eprintln!("  censuses every *.eadl under each root, recursively");
        eprintln!("  the population is the argument list, so a census states its own scope");
        return Ok(ExitCode::from(2));
    }

    let mut paths = Vec::new();
    for root in &roots {
        collect(Path::new(root), &mut paths);
    }
    paths.sort();
    paths.dedup();

    let mut census = Census::default();
    for path in &paths {
        census_file(path, &mut census)?;
    }

    let mut stdout = std::io::stdout().lock();
    report(&mut stdout, &roots, &census)?;
    if let Err(error) = stdout.flush() {
        if error.kind() != ErrorKind::BrokenPipe {
            eprintln!("literals: cannot flush output: {error}");
            return Ok(ExitCode::from(2));
        }
    }

    Ok(if census.refused.is_empty() {
        ExitCode::SUCCESS
    } else {
        ExitCode::from(1)
    })
}

/// Add every `.eadl` at or under `root`, in filesystem order; the caller sorts.
fn collect(root: &Path, out: &mut Vec<PathBuf>) {
    if root.is_dir() {
        let Ok(entries) = std::fs::read_dir(root) else {
            // A root that does not exist is reported by the population line, not swallowed here:
            // an argument that matched nothing is the difference between a census of 75 and one of
            // 62, and the reader of the output has to be able to see which happened.
            out.push(root.to_path_buf());
            return;
        };
        for entry in entries.filter_map(Result::ok) {
            collect(&entry.path(), out);
        }
    } else if root
        .extension()
        .is_some_and(|extension| extension == "eadl")
    {
        out.push(root.to_path_buf());
    }
}

/// Read one description and add everything it holds to `census`.
fn census_file(path: &Path, census: &mut Census) -> Result<(), ExitCode> {
    let display = path.display().to_string();
    let text = match std::fs::read_to_string(path) {
        Ok(text) => text,
        Err(error) => {
            census.unreadable.push((display, error.to_string()));
            return Ok(());
        }
    };

    let mut sources = SourceMap::new();
    let id = match sources.add(&display, &text) {
        Ok(id) => id,
        Err(error) => {
            census.unreadable.push((display, error.to_string()));
            return Ok(());
        }
    };

    census.files += 1;
    let (document, diagnostics) = read(&sources, id);
    if diagnostics.is_empty() {
        census.clean += 1;
    }
    tally(&diagnostics, &text, &display, census);
    walk(&document.forms, &text, &display, census);
    Ok(())
}

/// Tally every diagnostic by code, keeping the spelling of each domain refusal.
fn tally(diagnostics: &Diagnostics, text: &str, path: &str, census: &mut Census) {
    for diagnostic in diagnostics.items() {
        *census.codes.entry(diagnostic.code.to_string()).or_insert(0) += 1;
        if diagnostic.code == DOMAIN_REFUSAL {
            let span = diagnostic.primary.span;
            census.refused.push(Literal {
                // The value is exactly what the language cannot hold, so it is not recorded: the
                // spelling is the evidence, and the code says which rule refused it.
                value: 0,
                spelling: spelled(text, span.start, span.end),
                path: path.to_string(),
            });
        }
    }
}

/// Count every atom in the tree, recording each integer's value and its own spelling.
fn walk(forms: &[Form], text: &str, path: &str, census: &mut Census) {
    let mut stack: Vec<&Form> = forms.iter().collect();
    while let Some(form) = stack.pop() {
        match form {
            Form::List { items, .. } => stack.extend(items.iter()),
            Form::Integer { value, span } => {
                census.integers.push(Literal {
                    value: *value,
                    spelling: spelled(text, span.start, span.end),
                    path: path.to_string(),
                });
            }
            Form::Decimal { .. } => census.decimals += 1,
            Form::Str { .. } => census.strings += 1,
            Form::Symbol { .. } => census.symbols += 1,
        }
    }
}

/// The source text a span covers. Spans are byte offsets, so a span that is not on a character
/// boundary is reported rather than allowed to panic.
fn spelled(text: &str, start: u32, end: u32) -> String {
    text.get(start as usize..end as usize)
        .unwrap_or("<span is not a character boundary>")
        .to_string()
}

/// Print the census. Every figure is computed from this run; nothing is carried.
fn report(out: &mut dyn Write, roots: &[String], census: &Census) -> Result<(), ExitCode> {
    let matched = census.files + census.unreadable.len();
    emit(
        out,
        &format!(
            "population: {} description(s) under {} root(s): {}",
            census.files,
            roots.len(),
            roots.join(" ")
        ),
    )?;
    emit(
        out,
        &format!(
            "  read cleanly        : {}\n  produced diagnostics: {}",
            census.clean,
            census.files - census.clean
        ),
    )?;
    for (path, error) in &census.unreadable {
        emit(out, &format!("  ⛔ not a description  : {path} — {error}"))?;
    }
    if matched == 0 {
        emit(
            out,
            "⛔ the population is EMPTY: no root matched a *.eadl file, so every figure below \
             is a census of nothing",
        )?;
        return Ok(());
    }

    emit(out, "")?;
    emit(
        out,
        "atoms, by category (so a category holding nothing is visibly zero)",
    )?;
    let distinct = census.distinct_values();
    emit(
        out,
        &format!(
            "  integer literals    : {} occurrence(s), {} distinct value(s)",
            census.integers.len(),
            distinct.len()
        ),
    )?;
    // A claim about a set carries its enumeration: the count alone cannot be checked, and the list
    // is what lets a reader see that the population is what they think it is.
    emit(
        out,
        &format!(
            "  values              : {}",
            distinct
                .iter()
                .map(i64::to_string)
                .collect::<Vec<_>>()
                .join(" ")
        ),
    )?;
    emit(out, &format!("  decimal literals    : {}", census.decimals))?;
    emit(out, &format!("  strings             : {}", census.strings))?;
    emit(out, &format!("  symbols             : {}", census.symbols))?;

    emit(out, "")?;
    emit(out, "integer value domain")?;
    emit(
        out,
        &format!(
            "  domain              : {} … {} — a signed 64-bit value, so {} magnitude bits and a sign",
            i64::MIN,
            i64::MAX,
            63
        ),
    )?;
    if !census.integers.is_empty() {
        let smallest = census
            .integers
            .iter()
            .min_by_key(|found| found.value)
            .expect("the population was just shown to be non-empty");
        let largest = census
            .integers
            .iter()
            .max_by_key(|found| found.value)
            .expect("the population was just shown to be non-empty");
        emit(
            out,
            &format!(
                "  smallest            : {:>20}  as `{}` in {}",
                smallest.value, smallest.spelling, smallest.path
            ),
        )?;
        emit(
            out,
            &format!(
                "  largest             : {:>20}  as `{}` in {}",
                largest.value, largest.spelling, largest.path
            ),
        )?;
        if let Some(hexadecimal) = census
            .integers
            .iter()
            .filter(|found| {
                let spelling = found.spelling.to_ascii_lowercase();
                let digits = spelling.strip_prefix('-').unwrap_or(&spelling);
                digits.starts_with("0x")
            })
            .max_by_key(|found| found.value)
        {
            emit(
                out,
                &format!(
                    "  largest hexadecimal : {:>20}  as `{}` in {}",
                    hexadecimal.value, hexadecimal.spelling, hexadecimal.path
                ),
            )?;
        }
        // A magnitude's width in two's complement: 2^32 needs 33 bits, because the sign bit is one
        // of them. Computed, never carried — a width quoted from memory is the figure that goes stale.
        let bits = 64 - largest.value.unsigned_abs().leading_zeros();
        emit(
            out,
            &format!(
                "  headroom            : the largest needs {bits} of the domain's 63 magnitude \
                 bits, so {}× the largest still fits",
                i64::MAX / largest.value.max(1)
            ),
        )?;
    } else {
        emit(
            out,
            "  ⛔ no integer literal: nothing in this population constrains the domain, so this \
             run is evidence about the population and not about the limit",
        )?;
    }

    emit(out, "")?;
    emit(
        out,
        &format!("literals the domain refused (`{DOMAIN_REFUSAL}`)"),
    )?;
    if census.refused.is_empty() {
        emit(
            out,
            "  none — the domain is a stated limit over this population, not a live constraint",
        )?;
    } else {
        for refused in &census.refused {
            emit(
                out,
                &format!("  `{}` in {}", refused.spelling, refused.path),
            )?;
        }
    }

    emit(out, "")?;
    emit(out, "diagnostics the population reached, by code")?;
    if census.codes.is_empty() {
        emit(out, "  none")?;
    } else {
        for (code, count) in &census.codes {
            emit(out, &format!("  {code:<28} {count}"))?;
        }
    }
    Ok(())
}
