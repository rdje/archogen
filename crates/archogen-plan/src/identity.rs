//! Format `/2`'s code identity and the catalog's digest (`docs/decisions/decision_runtime-plan-format-2.md` §1; leaf
//! `M4.1.7.6.2`): what two plans of one code share, and so what lets a second plan, from a realization's file, say it is
//! of the first plan's code.
//!
//! "**The code identity** is the sha256 of the plan's canonical text, each line ending in a line feed, with each line
//! kept, left out, or written with its entry point alone, by its head, as this table says and nothing else does." The
//! table is [`LINES`]. `tests/format2_model.rs` holds it to the record's own table and to the model's, and holds
//! [`preimage`] to the model's over every plan the model writes.

use std::fmt;

use archogen_digest::Digest;

/// What the code identity does with a line of a plan, by the line's head (§1's table).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Treatment {
    /// The line stands in the preimage as written.
    Kept,
    /// The line is left out: a figure, where one came from, or what is said of the code and is no code.
    LeftOut,
    /// The line is written with its entry point alone, or as `none`.
    EntryPoint,
}

/// §1's table: each head a line of `/2` can begin with, and what the identity does with it.
pub const LINES: &[(&str, Treatment)] = &[
    ("format", Treatment::Kept),
    ("profile", Treatment::Kept),
    ("system", Treatment::Kept),
    ("target", Treatment::Kept),
    ("program", Treatment::Kept),
    ("assignment", Treatment::Kept),
    ("adapters", Treatment::Kept),
    ("emitter", Treatment::Kept),
    ("catalog", Treatment::Kept),
    ("application", Treatment::LeftOut),
    ("realization", Treatment::LeftOut),
    ("code", Treatment::Kept),
    ("record", Treatment::LeftOut),
    ("task", Treatment::Kept),
    ("task-inputs", Treatment::EntryPoint),
    ("declared", Treatment::LeftOut),
    ("initialisation-inputs", Treatment::EntryPoint),
    ("realization-figure", Treatment::LeftOut),
    ("realization-fact", Treatment::LeftOut),
    ("bind", Treatment::Kept),
    ("instance", Treatment::Kept),
    ("epoch", Treatment::Kept),
    ("timer", Treatment::Kept),
    ("alarm", Treatment::Kept),
    ("clock", Treatment::Kept),
    ("route", Treatment::Kept),
    ("enabled", Treatment::Kept),
    ("interrupt-model", Treatment::Kept),
    ("region", Treatment::Kept),
    ("placement", Treatment::Kept),
    ("allocation", Treatment::LeftOut),
    ("image", Treatment::LeftOut),
    ("node", Treatment::Kept),
    ("context", Treatment::Kept),
    ("depends", Treatment::Kept),
    ("step", Treatment::Kept),
    ("obligation", Treatment::Kept),
    ("owed", Treatment::LeftOut),
    ("complete", Treatment::LeftOut),
];

/// What the identity does with a line of this head, or nothing where no line of `/2` begins so.
#[must_use]
pub fn treatment(head: &str) -> Option<Treatment> {
    LINES
        .iter()
        .find(|(name, _)| *name == head)
        .map(|(_, treatment)| *treatment)
}

/// A line's head: the name after its opening parenthesis.
#[must_use]
pub fn head(line: &str) -> &str {
    let rest = line.strip_prefix('(').unwrap_or(line);
    rest.split([' ', ')']).next().unwrap_or(rest)
}

/// A line no row of §1's table classifies, by its head: no plan of `/2` writes one, so the text is not a plan of `/2`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct UnknownLine(pub String);

impl fmt::Display for UnknownLine {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no line of `/2` begins `{}`", self.0)
    }
}

impl std::error::Error for UnknownLine {}

/// The string literal opening at `from` in `line`, its quotes included, read as the plan's canonical text writes one: a
/// backslash takes the character after it.
fn literal(line: &str, from: usize) -> Option<&str> {
    let mut escaped = false;
    for (at, character) in line[from + 1..].char_indices() {
        match character {
            _ if escaped => escaped = false,
            '\\' => escaped = true,
            '"' => return Some(&line[from..from + 1 + at + 1]),
            _ => {}
        }
    }
    None
}

/// A line written with its entry point alone (§1): its head, a task's `(task SYSTEM NAME)` where it is a task's, and
/// `(entry-point "…")`, or `none` where the line states no entry point.
fn entry_point_alone(line: &str, head: &str) -> String {
    let after_head = 1 + head.len();
    let subject = if head == "task-inputs" {
        line[after_head..]
            .find("(task ")
            .map(|at| after_head + at)
            .and_then(|from| line[from..].find(')').map(|end| &line[from..=from + end]))
            .map_or_else(String::new, |task| format!("{task} "))
    } else {
        String::new()
    };
    let said = line
        .find("(entry-point \"")
        .and_then(|from| {
            literal(line, from + "(entry-point ".len()).map(|text| format!("(entry-point {text})"))
        })
        .unwrap_or_else(|| "none".to_owned());
    format!("({head} {subject}{said})")
}

/// §1, "The code identity": its preimage, "the plan's canonical text, each line ending in a line feed", each line kept,
/// left out or written with its entry point alone, by its head.
///
/// # Errors
///
/// [`UnknownLine`], naming the first line no row classifies.
pub fn preimage(plan: &str) -> Result<String, UnknownLine> {
    let mut out = String::new();
    for line in plan.lines() {
        let head = head(line);
        match treatment(head).ok_or_else(|| UnknownLine(head.to_owned()))? {
            Treatment::Kept => out.push_str(line),
            Treatment::LeftOut => continue,
            Treatment::EntryPoint => out.push_str(&entry_point_alone(line, head)),
        }
        out.push('\n');
    }
    Ok(out)
}

/// §1's code identity of `plan`: the sha256 of its [`preimage`].
///
/// # Errors
///
/// [`UnknownLine`], as [`preimage`] refuses.
pub fn code_identity(plan: &str) -> Result<Digest, UnknownLine> {
    preimage(plan).map(|preimage| Digest::of(preimage.as_bytes()))
}

/// §1, the catalog's digest's preimage: one line for each record, its id, a space and its hash as [`Digest`] displays
/// one, `sha256:` and its hex, by id, each ending in a line feed, with nothing before the first (RP13-3). An empty
/// catalog's is the empty text.
#[must_use]
pub fn catalog_preimage(records: &[(String, Digest)]) -> String {
    let mut sorted: Vec<&(String, Digest)> = records.iter().collect();
    sorted.sort_by(|a, b| (&a.0, a.1 .0).cmp(&(&b.0, b.1 .0)));
    sorted
        .iter()
        .map(|(id, hash)| format!("{id} {hash}\n"))
        .collect()
}

/// The catalog's digest, `(catalog "HEX" …)`'s `HEX`: the sha256 of [`catalog_preimage`].
#[must_use]
pub fn catalog_digest(records: &[(String, Digest)]) -> Digest {
    Digest::of(catalog_preimage(records).as_bytes())
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A string literal's escaped quote does not end the entry point a line is written with.
    #[test]
    fn an_entry_point_is_read_as_a_literal() {
        assert_eq!(
            preimage("(initialisation-inputs (entry-point \"a\\\")b\") (stack-bound (value 1) (unit byte)))\n"),
            Ok("(initialisation-inputs (entry-point \"a\\\")b\"))\n".to_owned())
        );
        assert_eq!(
            preimage("(task-inputs (task s t) (stack-bound (value 1) (unit byte)))\n"),
            Ok("(task-inputs (task s t) none)\n".to_owned())
        );
        assert_eq!(
            preimage("(nothing)\n"),
            Err(UnknownLine("nothing".to_owned()))
        );
    }

    /// An empty catalog's preimage is nothing, and its digest the sha256 of nothing.
    #[test]
    fn an_empty_catalog_digests_nothing() {
        assert_eq!(catalog_preimage(&[]), "");
        assert_eq!(catalog_digest(&[]), Digest::of(b""));
    }
}
