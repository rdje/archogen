//! Reading `docs/semantics/reference.md`'s machine-read tables, and the value notation they are
//! written in.
//!
//! ⛔ **One reader for the normative document, shared by every leg that checks anything against it.**
//! `reference.rs` runs the tables against the frontend; `conformance.rs` runs them against the
//! recognizer derived from `docs/semantics/grammar.md`. Two parsers for one normative table would be
//! two things that can disagree about what the rule *says*, before either asks what it means — the
//! same failure `docs/semantics/grammar.md` exists to end, one level up.
//!
//! The notation is exactly what the reference's own notation table defines, and nothing more: a
//! notation that could express more than this module understands is a notation that will eventually be
//! used to write a rule nothing checks.

/// What the reference says a numeric literal is worth.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// `integer N` — an exact 64-bit signed integer.
    Integer(i64),
    /// `rational N/10^S` — the digits `N` with the last `S` after the point. Never a float.
    Rational(i64, u32),
    /// `symbol T` — not a number at all: the atom is the symbol whose text is `T`.
    Symbol(String),
    /// `error C` — **not well-formed**, and refused with diagnostic code `C`.
    Error(String),
    /// `refused C` — well-formed, but its value is outside the domain the language can hold, so the
    /// frontend refuses it with code `C`.
    ///
    /// ⭐ The distinction is load-bearing and it is not cosmetic. `1.2.3` is not a number and the
    /// grammar must refuse it; `9223372036854775808` *is* a well-formed integer whose value does not
    /// fit, and the grammar must accept it — a syntax that could express "too large" would have to
    /// know the width of the value domain, which is exactly the implementation detail
    /// `docs/semantics/grammar.md` keeps out of itself. Collapsing the two would make one of the two
    /// legs wrong whichever way it was written.
    Refused(String),
}

impl Value {
    /// Read one value cell, or `None` if the notation cannot express it.
    ///
    /// `None` is a **violation**, never a reason to skip the row: a cell nothing can parse is a rule
    /// nothing checks.
    pub fn parse(cell: &str) -> Option<Self> {
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
            "refused" if !rest.is_empty() => Some(Self::Refused(rest.to_string())),
            _ => None,
        }
    }

    /// The verdict class name, for the non-vacuity leg.
    pub fn class(&self) -> &'static str {
        match self {
            Self::Integer(_) => "integer",
            Self::Rational(_, _) => "rational",
            Self::Symbol(_) => "symbol",
            Self::Error(_) => "error",
            Self::Refused(_) => "refused",
        }
    }

    /// Whether the reference says the frontend refuses this literal, and with which code.
    pub fn refusal(&self) -> Option<&str> {
        match self {
            Self::Error(code) | Self::Refused(code) => Some(code),
            _ => None,
        }
    }

    /// Whether the reference says this literal is **well-formed** — the question the grammar answers.
    ///
    /// A value and a refusal-because-out-of-range are both well-formed; only `error` is not.
    pub fn well_formed(&self) -> bool {
        !matches!(self, Self::Error(_))
    }
}

/// What the reference says a string literal denotes.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StringValue {
    /// The characters it denotes, decoded from the reference's own escape notation.
    Text(String),
    /// `error C` — not well-formed.
    Error(String),
    /// `refused C` — well-formed, outside the domain.
    Refused(String),
}

impl StringValue {
    /// Read one decoded-value cell.
    pub fn parse(cell: &str) -> Option<Self> {
        if let Some(code) = cell.strip_prefix("error ") {
            return (!code.is_empty()).then(|| Self::Error(code.to_string()));
        }
        if let Some(code) = cell.strip_prefix("refused ") {
            return (!code.is_empty()).then(|| Self::Refused(code.to_string()));
        }
        decode_escapes(cell).map(Self::Text)
    }

    /// Whether the reference says the frontend refuses this literal, and with which code.
    pub fn refusal(&self) -> Option<&str> {
        match self {
            Self::Error(code) | Self::Refused(code) => Some(code),
            Self::Text(_) => None,
        }
    }

    /// Whether the reference says this literal is well-formed.
    pub fn well_formed(&self) -> bool {
        !matches!(self, Self::Error(_))
    }
}

/// Decode a **source** cell into the text the frontend is given.
///
/// ⭐ This exists because the reference had a notation hole it could not write its way out of. A row
/// about a raw control byte — the input `read-unexpected-character` and `read-control-character` fire
/// on — cannot be written in a markdown table: the byte is invisible in the cell, and a line feed
/// ends the row. `M1.12.3` deferred an executable input column on §4's diagnostic table for exactly
/// this reason, and `M1.26`'s gap (b) inherits the deferral. `<0x1b>` names the character instead, so
/// the document stays readable and the row stays executable.
///
/// `None` is a **violation**, never a reason to skip the row: a cell nothing can decode is a rule
/// nothing checks, which is the same contract `Value::parse` holds.
pub fn source_text(cell: &str) -> Option<String> {
    let mut out = String::new();
    let mut rest = cell;
    while let Some(start) = rest.find("<0x") {
        out.push_str(&rest[..start]);
        let (digits, tail) = rest[start + 3..].split_once('>')?;
        // `char::from_u32` refuses a surrogate and anything past the last scalar value, so a marker
        // that names no character is a document defect rather than a silently dropped row.
        out.push(char::from_u32(u32::from_str_radix(digits, 16).ok()?)?);
        rest = tail;
    }
    out.push_str(rest);
    Some(out)
}

/// Decode the reference's value notation into characters.
///
/// ⭐ A second implementation of the escape rule, written from §2 of the reference rather than from
/// `reader.rs`. That is the point: one implementation cannot disagree with itself, so a reader that
/// decoded `\n` as a tab would be checked by nothing.
pub fn decode_escapes(cell: &str) -> Option<String> {
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
            // `\u{…}`, the general escape §2 added so the set is closed **and sufficient**: every
            // Unicode scalar value is writable, which is what lets §3 escape a control character
            // without producing text the language cannot read back.
            'u' => {
                if chars.next()? != '{' {
                    return None;
                }
                let mut digits = String::new();
                loop {
                    match chars.next()? {
                        '}' => break,
                        digit if digit.is_ascii_hexdigit() => digits.push(digit),
                        _ => return None,
                    }
                }
                out.push(char::from_u32(u32::from_str_radix(&digits, 16).ok()?)?);
            }
            // The notation says a backslash in a value cell always starts one of the escapes the
            // reference defines, so anything else means the document is not writable in its own
            // notation — which is a defect in the document, not a row to skip.
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
///
/// ⛔ **Total, and it has to be.** This mirrored `form.rs`'s four-arm escape set exactly, which is why
/// both were green while finding F-G was live: a second implementation of the *same* hole is not a
/// check on it. §3 rule 3 is a claim about every value the printer can be given, so the fall-through
/// escapes by code point, and the named escapes stay named because `\r` is more readable than `\u{d}`.
pub fn encode_canonical(value: &str) -> String {
    let mut out = String::from("\"");
    for ch in value.chars() {
        match ch {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\n' => out.push_str("\\n"),
            '\t' => out.push_str("\\t"),
            '\r' => out.push_str("\\r"),
            _ if ch.is_control() => out.push_str(&format!("\\u{{{:x}}}", u32::from(ch))),
            other => out.push(other),
        }
    }
    out.push('"');
    out
}

/// The first control character in `text`, if any.
///
/// §3 of the reference: canonical text is what gets diffed, hashed and printed in a report, so it
/// carries no control character. Tab, line feed and carriage return each break "one form per line" in
/// a different way, and a bare tab in a report column is invisible rather than merely ugly.
///
/// ⭐ Unicode `Cc`, not ASCII only: the C1 range `0x80`–`0x9f` is control characters too, and a
/// narrower predicate would have let one through a rule stated about "no control character".
pub fn control_character(text: &str) -> Option<char> {
    text.chars().find(|ch| ch.is_control())
}

/// The rows of the table introduced by `<!-- machine-read: <marker> -->`, with their line numbers.
///
/// Returns an empty vector when the marker is absent — which a non-vacuity leg turns into a
/// violation, so a renamed marker cannot make a check pass by checking nothing.
pub fn machine_table(document: &str, marker: &str) -> Vec<(usize, Vec<String>)> {
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
pub fn split_cells(line: &str) -> Vec<String> {
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
