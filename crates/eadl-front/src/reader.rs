//! The reader: bytes in, [`Document`] out, or diagnostics that say exactly where it stopped.
//!
//! One pass, no lookahead beyond a byte, no backtracking. The grammar is small enough that a
//! hand-written reader is shorter than a specification of one, and owning it means owning the
//! wording of every refusal — which §5.5 makes part of the user contract.
//!
//! **It does not stop at the first error.** A description with three malformed numbers should
//! report three, not one per edit-compile cycle. Recovery is deliberately crude: skip the
//! offending token and continue. Crude recovery that reports real errors beats clever recovery
//! that invents cascading ones.

use crate::diagnostic::{Diagnostic, Diagnostics, Label};
use crate::form::{Comment, Document, Form};
use crate::source::{SourceId, SourceMap, Span};

/// Read one source into a document.
///
/// Returns the document and every diagnostic produced. A document is returned even when there
/// are errors: the forms that *did* parse are still useful to later passes and to editors, and
/// the caller decides what an error means by asking [`Diagnostics::has_errors`].
///
/// # Panics
///
/// Panics if `source` is not present in `sources`. That is a programming error — the id can
/// only have come from `SourceMap::add`.
#[must_use]
pub fn read(sources: &SourceMap, source: SourceId) -> (Document, Diagnostics) {
    let text = sources
        .get(source)
        .expect("read called with an id not in this SourceMap")
        .text
        .clone();
    let (document, mut diagnostics) = Reader::new(&text, source).run();
    // §8 of `docs/semantics/reference.md`: the language version a description states. Checked here
    // rather than by a caller of `read`, so every consumer of a `Document` gets the same verdict
    // including the ones that do not know the rule exists — the reason `M1.13.1` put the escape rule
    // in the printer, since a rule that lives in one consumer is a rule the next consumer lacks.
    crate::language_version::state(&document, &mut diagnostics);
    (document, diagnostics)
}

struct Reader<'a> {
    text: &'a [u8],
    raw: &'a str,
    source: SourceId,
    at: usize,
    diagnostics: Diagnostics,
    comments: Vec<Comment>,
}

/// What closed a list-reading loop.
enum Closed {
    /// A `)` was found at this offset.
    Paren(u32),
    /// End of input arrived first.
    Eof,
}

impl<'a> Reader<'a> {
    fn new(raw: &'a str, source: SourceId) -> Self {
        Self {
            text: raw.as_bytes(),
            raw,
            source,
            at: 0,
            diagnostics: Diagnostics::new(),
            comments: Vec::new(),
        }
    }

    fn run(mut self) -> (Document, Diagnostics) {
        let mut forms = Vec::new();
        loop {
            self.skip_trivia();
            let Some(byte) = self.peek() else { break };
            if byte == b')' {
                let start = self.offset();
                self.at += 1;
                self.diagnostics.push(Diagnostic::error(
                    "read-unexpected-close",
                    "a closing parenthesis with nothing open",
                    Label::new(self.span(start, self.offset()), "no list is open here"),
                    "remove it, or add the matching `(` that was meant to open a list",
                ));
                continue;
            }
            match self.read_form() {
                Some(form) => forms.push(form),
                None => continue,
            }
        }
        (
            Document {
                forms,
                comments: self.comments,
            },
            self.diagnostics,
        )
    }

    // ── position helpers ─────────────────────────────────────────────────────────────────────

    fn offset(&self) -> u32 {
        u32::try_from(self.at).unwrap_or(u32::MAX)
    }

    fn span(&self, start: u32, end: u32) -> Span {
        Span::new(self.source, start, end)
    }

    fn peek(&self) -> Option<u8> {
        self.text.get(self.at).copied()
    }

    fn peek_at(&self, ahead: usize) -> Option<u8> {
        self.text.get(self.at + ahead).copied()
    }

    /// Whitespace and comments. Comments are collected, not discarded.
    fn skip_trivia(&mut self) {
        loop {
            match self.peek() {
                Some(b) if b.is_ascii_whitespace() => self.at += 1,
                Some(b';') => {
                    let start = self.offset();
                    self.at += 1;
                    let text_start = self.at;
                    while let Some(b) = self.peek() {
                        if b == b'\n' {
                            break;
                        }
                        self.at += 1;
                    }
                    let text = self.raw[text_start..self.at].trim_end().to_string();
                    let span = self.span(start, self.offset());
                    self.comments.push(Comment { text, span });
                }
                _ => return,
            }
        }
    }

    // ── forms ────────────────────────────────────────────────────────────────────────────────

    /// Read one form. `None` means an error was recorded and the offending input consumed.
    fn read_form(&mut self) -> Option<Form> {
        self.skip_trivia();
        let byte = self.peek()?;
        match byte {
            b'(' => self.read_list(),
            b'"' => self.read_string(),
            b')' => None,
            b if b.is_ascii_digit() => self.read_number(),
            b'-' | b'+' if self.peek_at(1).is_some_and(|b| b.is_ascii_digit()) => {
                self.read_number()
            }
            _ => self.read_symbol(),
        }
    }

    fn read_list(&mut self) -> Option<Form> {
        let open = self.offset();
        self.at += 1; // consume '('
        let mut items = Vec::new();

        let closed = loop {
            self.skip_trivia();
            match self.peek() {
                None => break Closed::Eof,
                Some(b')') => {
                    let close = self.offset();
                    self.at += 1;
                    break Closed::Paren(close);
                }
                Some(_) => {
                    let before = self.at;
                    match self.read_form() {
                        Some(form) => items.push(form),
                        None => {
                            // Recovery: if the failing read consumed nothing we would spin
                            // forever, so force progress by a byte.
                            if self.at == before {
                                self.at += 1;
                            }
                        }
                    }
                }
            }
        };

        match closed {
            Closed::Paren(close) => Some(Form::List {
                items,
                span: self.span(open, close + 1),
            }),
            Closed::Eof => {
                let end = self.offset();
                self.diagnostics.push(
                    Diagnostic::error(
                        "read-unclosed-list",
                        "this list is never closed",
                        Label::new(self.span(end, end), "input ends here, still inside a list"),
                        "add the matching `)`",
                    )
                    .with_secondary(Label::new(self.span(open, open + 1), "opened here")),
                );
                // Still return what was read: a truncated list is more useful to a caller
                // than nothing, and the error is already recorded.
                Some(Form::List {
                    items,
                    span: self.span(open, end),
                })
            }
        }
    }

    fn read_string(&mut self) -> Option<Form> {
        let open = self.offset();
        self.at += 1; // consume '"'
        let mut value = String::new();
        loop {
            let Some(byte) = self.peek() else {
                let end = self.offset();
                self.diagnostics.push(
                    Diagnostic::error(
                        "read-unterminated-string",
                        "this string is never closed",
                        Label::new(
                            self.span(end, end),
                            "input ends here, still inside a string",
                        ),
                        "add the closing `\"`",
                    )
                    .with_secondary(Label::new(self.span(open, open + 1), "opened here")),
                );
                return None;
            };
            match byte {
                b'"' => {
                    self.at += 1;
                    return Some(Form::Str {
                        value,
                        span: self.span(open, self.offset()),
                    });
                }
                b'\n' => {
                    let at = self.offset();
                    self.diagnostics.push(
                        Diagnostic::error(
                            "read-unterminated-string",
                            "this string is not closed before the end of the line",
                            Label::new(self.span(at, at), "the line ends here"),
                            "close the string on its own line, or escape the newline as `\\n`",
                        )
                        .with_secondary(Label::new(self.span(open, open + 1), "opened here")),
                    );
                    return None;
                }
                b'\\' => {
                    let escape_start = self.offset();
                    self.at += 1;
                    // ⛔ The escaped character is read WHOLE. It was read as one byte, and a multi-byte
                    // one after a backslash — `"a\\éb"` — left the reader inside the character: the
                    // diagnostic's span ended mid-character and the next step panicked slicing there
                    // (leaf `M1.35`). The backslash is ASCII, so `at` is on a boundary here.
                    let Some(escape) = self.raw[self.at..].chars().next() else {
                        continue;
                    };
                    self.at += escape.len_utf8();
                    match escape {
                        'n' => value.push('\n'),
                        't' => value.push('\t'),
                        'r' => value.push('\r'),
                        '"' => value.push('"'),
                        '\\' => value.push('\\'),
                        'u' => {
                            if let Some(ch) = self.read_unicode_escape(escape_start) {
                                value.push(ch);
                            }
                        }
                        other => {
                            let end = self.offset();
                            self.diagnostics.push(Diagnostic::error(
                                "read-bad-escape",
                                format!("`\\{other}` is not an escape sequence"),
                                Label::new(self.span(escape_start, end), "unknown escape"),
                                "the supported escapes are \\n, \\t, \\r, \\\", \\\\ and \\u{…}",
                            ));
                            // Keep the characters literally so one typo does not cascade.
                            value.push('\\');
                            value.push(other);
                        }
                    }
                }
                _ => {
                    // Copy one whole character, so multi-byte text survives intact.
                    let char_start = self.at;
                    let ch = self.raw[char_start..].chars().next().unwrap_or('\u{FFFD}');
                    self.at += ch.len_utf8();
                    // ⛔ A raw control character is refused inside a string exactly as it already is
                    // between forms, where `read-unexpected-character` calls it "a stray control
                    // character". The rule was the language's and it stopped at the string's opening
                    // quote, so a description could carry an invisible byte that canonical form then
                    // printed into the text §12 M4 hashes and compares — a NUL included, with no
                    // diagnostic at all (finding F-G, leaf `M1.13.1`). TAB is the one exception: it is
                    // whitespace the grammar already names, and canonical form prints it as `\t`.
                    // Nothing writable is lost, because every control character now has an escape.
                    if ch.is_control() && ch != '\t' {
                        let end = self.offset();
                        self.diagnostics.push(Diagnostic::error(
                            "read-control-character",
                            format!(
                                "a string cannot hold a raw control character; `\\u{{{:x}}}` writes this one",
                                u32::from(ch)
                            ),
                            Label::new(
                                self.span(char_start as u32, end),
                                "invisible here, and it would print into canonical text",
                            ),
                            "write the escape instead — `\\n`, `\\t`, `\\r`, or `\\u{…}` for any other",
                        ));
                        continue;
                    }
                    value.push(ch);
                }
            }
        }
    }

    /// Read the `{…}` of a `\u{…}` escape — the `u` is already consumed — and return the character it
    /// names, or push the diagnostic saying why it names none.
    ///
    /// ⭐ **Two different refusals, and collapsing them would break a leg.** An escape that is not
    /// *shaped* like one (`\u41`, `\u{}`, `\u{1b`) is not well-formed at all, so the recognizer derived
    /// from `docs/semantics/grammar.md` must reject it too: `read-bad-escape`. One that is shaped
    /// correctly but names no Unicode scalar value (`\u{d800}`, `\u{110000}`) *is* well-formed and is
    /// outside the domain the language can hold — the same distinction §1 draws between
    /// `read-malformed-number` and `read-number-overflow`, with its own code for the same reason:
    /// `crates/eadl-front/tests/conformance.rs` requires the recognizer to accept every `refused` row
    /// and reject every `error` one, so one code for both would make one of the two legs wrong.
    fn read_unicode_escape(&mut self, escape_start: u32) -> Option<char> {
        if self.peek() != Some(b'{') {
            let end = self.offset();
            self.bad_unicode_escape(escape_start, end);
            return None;
        }
        self.at += 1;
        let digits_start = self.at;
        while self.peek().is_some_and(|byte| byte.is_ascii_hexdigit()) {
            self.at += 1;
        }
        // Owned, because the diagnostic below borrows `self` mutably while naming these digits.
        let digits = self.raw[digits_start..self.at].to_string();
        if digits.is_empty() || self.peek() != Some(b'}') {
            if self.peek() == Some(b'}') {
                self.at += 1;
            }
            let end = self.offset();
            self.bad_unicode_escape(escape_start, end);
            return None;
        }
        self.at += 1;
        let end = self.offset();
        let Some(ch) = u32::from_str_radix(&digits, 16)
            .ok()
            .and_then(char::from_u32)
        else {
            self.diagnostics.push(Diagnostic::error(
                "read-escape-out-of-range",
                format!("`\\u{{{digits}}}` names no character"),
                Label::new(self.span(escape_start, end), "not a Unicode scalar value"),
                "a code point runs to `10ffff` and is not a surrogate (`d800`–`dfff`)",
            ));
            return None;
        };
        Some(ch)
    }

    /// Report a `\u` escape whose shape is wrong, over the span from the backslash to where reading
    /// stopped.
    fn bad_unicode_escape(&mut self, escape_start: u32, end: u32) {
        let written = self.raw[escape_start as usize..end as usize].to_string();
        self.diagnostics.push(Diagnostic::error(
            "read-bad-escape",
            format!("`{written}` is not an escape sequence"),
            Label::new(self.span(escape_start, end), "malformed `\\u{…}` escape"),
            "write one to six hexadecimal digits in braces, e.g. `\\u{1b}`",
        ));
    }

    fn read_number(&mut self) -> Option<Form> {
        let start = self.offset();
        let negative = matches!(self.peek(), Some(b'-'));
        if matches!(self.peek(), Some(b'-' | b'+')) {
            self.at += 1;
        }

        // Hexadecimal.
        if self.peek() == Some(b'0') && matches!(self.peek_at(1), Some(b'x' | b'X')) {
            self.at += 2;
            let digits_start = self.at;
            // ⛔ A hexadecimal literal must BEGIN with a digit. `_` separates digits, it does not
            // lead them — and the decimal path below cannot begin with one either, so accepting
            // `0x_10` made hexadecimal the one literal form whose leading character the reader did
            // not police. It also disagreed with `docs/semantics/grammar.md`, whose `hexadecimal`
            // production requires a `hex_digit` first; no corpus file contained the spelling, which
            // is how the divergence stayed hidden (finding F-B, leaf `M1.12.1`).
            let leads_with_digit = self.peek().is_some_and(|b| b.is_ascii_hexdigit());
            while self
                .peek()
                .is_some_and(|b| b.is_ascii_hexdigit() || b == b'_')
            {
                self.at += 1;
            }
            let raw: String = self.raw[digits_start..self.at].replace('_', "");
            let end = self.offset();
            if !leads_with_digit {
                let written = self.raw[start as usize..end as usize].to_string();
                let diagnostic = if raw.is_empty() {
                    Diagnostic::error(
                        "read-malformed-number",
                        "`0x` with no hexadecimal digits after it",
                        Label::new(self.span(start, end), "expected hexadecimal digits"),
                        "write the digits, e.g. `0x1000_0000`",
                    )
                } else {
                    Diagnostic::error(
                        "read-malformed-number",
                        format!("`{written}` is not a hexadecimal literal"),
                        Label::new(self.span(start, end), "a separator cannot lead the digits"),
                        "write the digits first and separate them after, e.g. `0x1000_0000`",
                    )
                };
                self.diagnostics.push(diagnostic);
                return None;
            }
            let value = i128::from_str_radix(&raw, 16)
                .ok()
                .and_then(|m| signed(m, negative));
            return match value {
                Some(value) => Some(Form::Integer {
                    value,
                    span: self.span(start, end),
                }),
                None => {
                    self.diagnostics.push(Diagnostic::error(
                        "read-number-overflow",
                        "this hexadecimal literal does not fit in a 64-bit signed integer",
                        Label::new(self.span(start, end), "too large"),
                        OVERFLOW_REPAIR,
                    ));
                    None
                }
            };
        }

        // Decimal, with an optional fractional part. No exponent form: `1e9` invites a float,
        // and §7.4 requires exact arithmetic.
        let int_start = self.at;
        while self.peek().is_some_and(|b| b.is_ascii_digit() || b == b'_') {
            self.at += 1;
        }
        let int_digits: String = self.raw[int_start..self.at].replace('_', "");

        let mut frac_digits = String::new();
        if self.peek() == Some(b'.') && self.peek_at(1).is_some_and(|b| b.is_ascii_digit()) {
            self.at += 1;
            let frac_start = self.at;
            while self.peek().is_some_and(|b| b.is_ascii_digit() || b == b'_') {
                self.at += 1;
            }
            frac_digits = self.raw[frac_start..self.at].replace('_', "");
        }

        // A second point, or a digit-leading atom like `3abc`, is a malformed number rather
        // than a symbol: reading it as a symbol would silently accept a typo.
        if self.peek().is_some_and(|b| b == b'.' || is_symbol_byte(b)) {
            while self.peek().is_some_and(|b| b == b'.' || is_symbol_byte(b)) {
                self.at += 1;
            }
            let end = self.offset();
            self.diagnostics.push(Diagnostic::error(
                "read-malformed-number",
                format!("`{}` is not a number", &self.raw[start as usize..end as usize]),
                Label::new(self.span(start, end), "a number cannot continue like this"),
                "write an integer or a decimal such as `10` or `1.5`; units go in a following atom, e.g. `10 ms`",
            ));
            return None;
        }

        let end = self.offset();
        let combined = format!("{int_digits}{frac_digits}");
        let scale = u32::try_from(frac_digits.len()).unwrap_or(u32::MAX);
        let value = combined
            .parse::<i128>()
            .ok()
            .and_then(|magnitude| signed(magnitude, negative));
        match value {
            Some(value) => Some(if scale == 0 {
                Form::Integer {
                    value,
                    span: self.span(start, end),
                }
            } else {
                Form::Decimal {
                    value,
                    scale,
                    span: self.span(start, end),
                }
            }),
            None => {
                self.diagnostics.push(Diagnostic::error(
                    "read-number-overflow",
                    "this literal does not fit in a 64-bit signed integer",
                    Label::new(self.span(start, end), "too large"),
                    OVERFLOW_REPAIR,
                ));
                None
            }
        }
    }

    fn read_symbol(&mut self) -> Option<Form> {
        let start = self.offset();
        while self.peek().is_some_and(is_symbol_byte) {
            self.at += 1;
        }
        let end = self.offset();
        if end == start {
            // A byte that can start nothing: a stray control character or a lone delimiter.
            let ch = self.raw[start as usize..]
                .chars()
                .next()
                .unwrap_or('\u{FFFD}');
            self.at += ch.len_utf8();
            self.diagnostics.push(Diagnostic::error(
                "read-unexpected-character",
                format!("`{}` cannot start a form", ch.escape_debug()),
                Label::new(self.span(start, self.offset()), "unexpected here"),
                "forms are lists `(…)`, symbols, numbers or strings",
            ));
            return None;
        }
        Some(Form::Symbol {
            name: self.raw[start as usize..end as usize].to_string(),
            span: self.span(start, end),
        })
    }
}

/// Bytes a symbol may contain.
///
/// Deliberately permissive: `time.monotonic`, `at-least`, `rt-static-up-v1`, `KiB`, `>=`. It
/// excludes whitespace, parentheses, quotes, and `;` — the characters that structure the text —
/// so a missing delimiter is a read error rather than a symbol that quietly swallows the rest
/// of the line.
fn is_symbol_byte(byte: u8) -> bool {
    !byte.is_ascii_whitespace()
        && !matches!(byte, b'(' | b')' | b'"' | b';')
        && !byte.is_ascii_control()
}

/// The repair direction for a literal outside the value domain, shared by both arms that refuse one.
///
/// ⭐ **One code, one repair.** §4 rule 3 of `docs/semantics/reference.md` makes
/// `read-number-overflow` a *rule* rather than a call site, so the decimal and hexadecimal arms
/// enforce the same rule and must not offer two different ways out of it. Measured, they did: the
/// hexadecimal arm said "split the quantity", the decimal one said "reduce the digits", and §4's own
/// column merged them into a third wording — three texts for one rule, none checked against another.
///
/// ⛔ **The third clause is the one the domain's honest limit needs.** A canonical high-half address —
/// an RV64 kernel address, whose upper bits a paging scheme requires to be all set — is above this
/// domain *as a magnitude* while being exactly representable as a signed value, so what fails is the
/// unsigned spelling and not the address. Telling that author to "reduce the digits" sends them away
/// with the wrong repair, which is what §4's preamble calls the most expensive diagnostic to receive.
/// §1 rule 10 states the limit; leaf `M1.13.2` measured it.
const OVERFLOW_REPAIR: &str = concat!(
    "eADL integers are exact 64-bit signed values; reduce the magnitude, change the units, ",
    "or write the negative value the literal is two's-complement equal to",
);

/// Apply a sign to a parsed magnitude and narrow it to an exact `i64`, or refuse.
///
/// ⛔ **The magnitude is parsed in a wider domain than the value it becomes, and the sign is applied
/// afterwards.** The other order makes `-9223372036854775808` unwritable: its magnitude is one more
/// than `i64::MAX`, so it overflowed before the sign could make it exactly `i64::MIN` — and the
/// language's own rule, stated in `docs/semantics/reference.md` §1, is that every value in the 64-bit
/// signed range is writable including both endpoints (finding F-E, leaf `M1.12.1`).
///
/// ⭐ **Both spellings of the minimum go through here, and the hexadecimal one was pinned by nothing
/// until `M1.13.2`.** `-9223372036854775808` and `-0x8000_0000_0000_0000` are the same value written
/// two ways, and both are §1 rows now; before that the table and this file's own tests carried only
/// the decimal, so a regression in the hexadecimal arm — the spelling the original defect lived in —
/// would have left every gate green.
///
/// Refusing is the only other option: wrapping would turn an out-of-range quantity into a plausible
/// wrong one, which §7.4's exactness requirement exists to prevent.
fn signed(magnitude: i128, negative: bool) -> Option<i64> {
    let signed = if negative { -magnitude } else { magnitude };
    i64::try_from(signed).ok()
}

#[cfg(test)]
mod tests {
    use super::read;
    use crate::form::Form;
    use crate::source::SourceMap;

    fn parse(
        text: &str,
    ) -> (
        crate::form::Document,
        crate::diagnostic::Diagnostics,
        SourceMap,
    ) {
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", text).expect("small");
        let (document, diagnostics) = read(&sources, id);
        (document, diagnostics, sources)
    }

    fn parse_ok(text: &str) -> crate::form::Document {
        let (document, diagnostics, sources) = parse(text);
        assert!(
            !diagnostics.has_errors(),
            "unexpected errors:\n{}",
            diagnostics.render(&sources)
        );
        document
    }

    #[test]
    fn a_nested_list_parses_to_the_right_shape() {
        let document = parse_ok(
            "(defservice time.monotonic\n  (requires (unambiguous-horizon (at-least 60 s))))\n",
        );
        assert_eq!(document.forms.len(), 1);
        assert_eq!(document.forms[0].head(), Some("defservice"));
        assert_eq!(
            document.to_canonical().trim(),
            "(defservice time.monotonic (requires (unambiguous-horizon (at-least 60 s))))"
        );
    }

    #[test]
    fn numbers_are_exact_and_never_floats() {
        let document = parse_ok("(x 10 1.5 0.1 -3 0x1000_0000 1_000)");
        let items = document.forms[0].items();
        assert!(matches!(items[1], Form::Integer { value: 10, .. }));
        assert!(matches!(
            items[2],
            Form::Decimal {
                value: 15,
                scale: 1,
                ..
            }
        ));
        assert!(matches!(
            items[3],
            Form::Decimal {
                value: 1,
                scale: 1,
                ..
            }
        ));
        assert!(matches!(items[4], Form::Integer { value: -3, .. }));
        assert!(matches!(
            items[5],
            Form::Integer {
                value: 0x1000_0000,
                ..
            }
        ));
        assert!(matches!(items[6], Form::Integer { value: 1000, .. }));
        // The canonical text is the input's meaning, not its spelling.
        assert_eq!(
            document.to_canonical().trim(),
            "(x 10 1.5 0.1 -3 268435456 1000)"
        );
    }

    #[test]
    fn a_unit_is_just_the_next_symbol() {
        // Units are the model layer's business. The reader stays syntactic, which is why the
        // same reader serves the boundary corpus and the S0 fixture unchanged.
        let document = parse_ok("(tick-rate 10 MHz)");
        let items = document.forms[0].items();
        assert!(matches!(items[1], Form::Integer { value: 10, .. }));
        assert_eq!(items[2].as_symbol(), Some("MHz"));
    }

    #[test]
    fn an_unclosed_list_points_at_both_ends() {
        let (_, diagnostics, sources) = parse("(defservice time\n  (requires x)\n");
        assert!(diagnostics.has_errors());
        let rendered = diagnostics.render(&sources);
        assert!(rendered.contains("read-unclosed-list"), "{rendered}");
        assert!(rendered.contains("opened here"), "{rendered}");
        assert!(
            rendered.contains("t.eadl:1:1"),
            "the opener's location:\n{rendered}"
        );
    }

    #[test]
    fn an_unexpected_close_paren_is_reported_where_it_is() {
        let (_, diagnostics, sources) = parse("(a))\n");
        let rendered = diagnostics.render(&sources);
        assert!(rendered.contains("read-unexpected-close"), "{rendered}");
        assert!(rendered.contains("t.eadl:1:4"), "{rendered}");
    }

    #[test]
    fn an_unterminated_string_is_reported_at_the_line_end() {
        let (_, diagnostics, sources) = parse("(provider \"device-clint)\n");
        let rendered = diagnostics.render(&sources);
        assert!(rendered.contains("read-unterminated-string"), "{rendered}");
    }

    #[test]
    fn a_bad_escape_names_the_supported_ones() {
        let (_, diagnostics, sources) = parse("(x \"a\\qb\")");
        let rendered = diagnostics.render(&sources);
        assert!(rendered.contains("read-bad-escape"), "{rendered}");
        assert!(rendered.contains("\\n, \\t, \\r"), "{rendered}");
    }

    #[test]
    fn an_unknown_escape_before_a_multibyte_character_is_reported_not_a_panic() {
        // Leaf `M1.35`: the escaped character was read as one byte, which stranded the reader inside
        // `é` — the next slice panicked, and `archogen check` exited 101 on this text.
        for (escaped, width) in [("é", 2), ("🙂", 4)] {
            let text = format!("(x \"a\\{escaped}b\")");
            let (document, diagnostics, sources) = parse(&text);
            let bad: Vec<_> = diagnostics
                .items()
                .iter()
                .filter(|d| d.code == "read-bad-escape")
                .collect();
            assert_eq!(bad.len(), 1, "{}", diagnostics.render(&sources));
            assert!(
                bad[0].message.contains(&format!("`\\{escaped}`")),
                "{}",
                bad[0].message
            );
            // The span covers the backslash and the whole character, on character boundaries.
            let span = bad[0].primary.span;
            assert_eq!((span.end - span.start) as usize, 1 + width, "{span:?}");
            assert_eq!(
                &text[span.start as usize..span.end as usize],
                format!("\\{escaped}")
            );
            // The characters are kept literally, and the rest of the string survives.
            let Form::List { items, .. } = &document.forms[0] else {
                panic!("not a list: {:?}", document.forms[0]);
            };
            let expected = format!("a\\{escaped}b");
            assert!(
                matches!(&items[1], Form::Str { value, .. } if *value == expected),
                "{:?}",
                items[1]
            );
        }
    }

    #[test]
    fn a_malformed_number_is_not_silently_read_as_a_symbol() {
        // ⭐ `3ms` is a typo for `3 ms`. Reading it as a symbol would accept it and lose the
        // magnitude, and the failure would surface much later as a missing field.
        let (_, diagnostics, sources) = parse("(period 3ms)");
        let rendered = diagnostics.render(&sources);
        assert!(rendered.contains("read-malformed-number"), "{rendered}");
        assert!(
            rendered.contains("units go in a following atom"),
            "{rendered}"
        );
    }

    #[test]
    fn a_second_decimal_point_is_refused() {
        let (_, diagnostics, _) = parse("(x 1.2.3)");
        assert!(diagnostics.has_errors());
    }

    #[test]
    fn an_overflowing_literal_is_refused_rather_than_wrapping() {
        let (_, diagnostics, sources) = parse("(x 99999999999999999999)");
        let rendered = diagnostics.render(&sources);
        assert!(rendered.contains("read-number-overflow"), "{rendered}");
    }

    #[test]
    fn a_hexadecimal_literal_may_not_lead_with_a_separator() {
        // ⛔ `0x_10` read as 16 while `docs/semantics/grammar.md` refused it: `_` separates digits,
        // it does not lead them, and the decimal path never could. No corpus file contained the
        // spelling, which is how the divergence stayed hidden (finding F-B, leaf `M1.12.1`).
        let (_, diagnostics, sources) = parse("(base 0x_10)");
        let rendered = diagnostics.render(&sources);
        assert!(rendered.contains("read-malformed-number"), "{rendered}");
        assert!(
            rendered.contains("a separator cannot lead the digits"),
            "{rendered}"
        );

        // A prefix with no digits at all keeps its own wording: it is a different mistake, and the
        // repair an author needs is different too.
        let (_, empty, sources) = parse("(base 0x)");
        let rendered = empty.render(&sources);
        assert!(
            rendered.contains("no hexadecimal digits after it"),
            "{rendered}"
        );

        // A separator *between* digits is still a separator.
        let document = parse_ok("(base 0x1000_0000)");
        assert!(matches!(
            document.forms[0].items()[1],
            Form::Integer {
                value: 268435456,
                ..
            }
        ));
    }

    #[test]
    fn the_whole_signed_64_bit_range_is_writable_including_its_negative_endpoint() {
        // ⛔ `-9223372036854775808` is `i64::MIN`, and it was refused as overflow because the
        // magnitude was parsed as an `i64` *before* the sign was applied — so the range this
        // language claims had exactly one unwritable value (finding F-E, leaf `M1.12.1`).
        let document = parse_ok("(x -9223372036854775808 9223372036854775807)");
        let items = document.forms[0].items();
        assert!(matches!(
            items[1],
            Form::Integer {
                value: i64::MIN,
                ..
            }
        ));
        assert!(matches!(
            items[2],
            Form::Integer {
                value: i64::MAX,
                ..
            }
        ));
        assert_eq!(
            document.to_canonical().trim(),
            "(x -9223372036854775808 9223372036854775807)"
        );

        // One past either endpoint is still refused rather than wrapped.
        for literal in ["9223372036854775808", "-9223372036854775809"] {
            let (_, diagnostics, sources) = parse(&format!("(x {literal})"));
            let rendered = diagnostics.render(&sources);
            assert!(
                rendered.contains("read-number-overflow"),
                "{literal} was not refused:\n{rendered}"
            );
        }
    }

    #[test]
    fn the_hexadecimal_spelling_of_the_domain_boundary_behaves_like_the_decimal_one() {
        // ⭐ §1 rule 9 pins the domain in **both** spellings, and until `M1.13.2` nothing pinned the
        // hexadecimal one: the reference's table, the test above and `docs/book/src/reading.md` all
        // used decimal. `-0x8000_0000_0000_0000` is the same value as `-9223372036854775808` but
        // reaches it through a different arm of the reader, so a regression there left every gate
        // green — in the spelling finding F-E originally lived in.
        let document = parse_ok("(x -0x8000_0000_0000_0000 0x7fff_ffff_ffff_ffff)");
        let items = document.forms[0].items();
        assert!(matches!(
            items[1],
            Form::Integer {
                value: i64::MIN,
                ..
            }
        ));
        assert!(matches!(
            items[2],
            Form::Integer {
                value: i64::MAX,
                ..
            }
        ));
        // Canonical form is a function of value, so both spellings print as decimal (§3 rule 1).
        assert_eq!(
            document.to_canonical().trim(),
            "(x -9223372036854775808 9223372036854775807)"
        );

        for literal in ["0x8000_0000_0000_0000", "-0x8000_0000_0000_0001"] {
            let (_, diagnostics, sources) = parse(&format!("(x {literal})"));
            let rendered = diagnostics.render(&sources);
            assert!(
                rendered.contains("read-number-overflow"),
                "{literal} was not refused:\n{rendered}"
            );
        }
    }

    #[test]
    fn a_high_half_address_is_refused_unsigned_and_readable_signed() {
        // ⚠️ §1 rule 10's honest limit, executed rather than described. A canonical RV64 high-half
        // virtual address has its upper bits all set, so as a magnitude it is above this domain —
        // while the same 64-bit pattern is exactly a negative `i64`, and that spelling reads and
        // round-trips. ⭐ This test is what a future widening has to *change*: it pins today's
        // behaviour so widening the domain is a decision someone makes deliberately rather than a
        // refusal that quietly stops firing.
        let (_, diagnostics, sources) = parse("(base 0xFFFF_FFFF_C000_0000)");
        let rendered = diagnostics.render(&sources);
        assert!(
            rendered.contains("read-number-overflow"),
            "the unsigned spelling was not refused:\n{rendered}"
        );
        assert!(
            rendered.contains("two's-complement"),
            "the repair direction does not name the spelling that does work, which is the whole \
             point of §5.5's repair requirement:\n{rendered}"
        );

        let document = parse_ok("(base -1073741824)");
        assert!(matches!(
            document.forms[0].items()[1],
            Form::Integer {
                value: -1_073_741_824,
                ..
            }
        ));
        assert_eq!(document.to_canonical().trim(), "(base -1073741824)");
    }

    #[test]
    fn reading_does_not_stop_at_the_first_error() {
        // Three malformed numbers should cost one edit cycle, not three.
        let (_, diagnostics, _) = parse("(a 1.2.3)\n(b 4.5.6)\n(c 7.8.9)\n");
        assert_eq!(diagnostics.len(), 3, "the reader stopped early");
    }

    #[test]
    fn comments_are_kept_with_their_spans() {
        let document = parse_ok("; case: time-horizon\n; verdict: accept\n(x)\n");
        assert_eq!(document.comments.len(), 2);
        assert_eq!(document.comments[0].text, " case: time-horizon");
        assert_eq!(document.forms.len(), 1);
    }

    #[test]
    fn comment_headers_parse_keys_and_continuations() {
        let document = parse_ok(
            "; case: x\n; rationale: the first line\n;   and its continuation\n; verdict: accept\n(y)\n",
        );
        let headers = document.comment_headers();
        assert_eq!(headers[0], ("case".into(), "x".into()));
        assert_eq!(
            headers[1],
            (
                "rationale".into(),
                "the first line and its continuation".into()
            )
        );
        assert_eq!(headers[2], ("verdict".into(), "accept".into()));
        assert_eq!(headers.len(), 3, "{headers:?}");
    }

    #[test]
    fn prose_containing_a_colon_continues_a_value_and_does_not_open_a_header() {
        // ⭐ The rule that actually works. Every corpus comment reads `; key: value`, so every
        // line begins with a space after the `;` — indentation cannot distinguish a
        // continuation. A key has no spaces, so prose with a colon in it never looks like one.
        let document = parse_ok(
            "; rationale: AMBIGUOUS, resolved REJECT.\n;   §7.3 is precise about the split: evidence is not a field.\n; verdict: reject\n(y)\n",
        );
        let headers = document.comment_headers();
        assert_eq!(headers.len(), 2, "{headers:?}");
        assert_eq!(headers[0].0, "rationale");
        assert!(
            headers[0].1.contains("precise about the split:"),
            "{headers:?}"
        );
        assert_eq!(headers[1], ("verdict".into(), "reject".into()));
    }

    #[test]
    fn a_wrapped_line_that_begins_with_a_key_shaped_word_does_not_open_a_header() {
        // ⭐ Found by running `examples/diagnose` over the real corpus, not by imagining it.
        // A rationale wrapped onto `;   implementation-independence: a different timer …`,
        // which is exactly a bare key followed by a colon. The key-shape test alone accepted
        // it and silently truncated the rationale; the indentation test alone had already
        // failed on the ordinary `; key: value` spacing. Both are required.
        let document = parse_ok(
            "; rationale: the test that settles it is\n;   implementation-independence: a different timer works\n; verdict: accept\n(y)\n",
        );
        let headers = document.comment_headers();
        let keys: Vec<&str> = headers.iter().map(|(k, _)| k.as_str()).collect();
        assert_eq!(keys, vec!["rationale", "verdict"], "{headers:?}");
        assert!(
            headers[0]
                .1
                .contains("implementation-independence: a different timer works"),
            "{headers:?}"
        );
    }

    #[test]
    fn an_unindented_key_shaped_line_still_opens_a_header() {
        // The other direction: the fix must not make headers unreachable.
        let document = parse_ok("; rationale: one\n; other-side: two\n(y)\n");
        let keys: Vec<String> = document
            .comment_headers()
            .into_iter()
            .map(|(k, _)| k)
            .collect();
        assert_eq!(keys, vec!["rationale", "other-side"]);
    }

    #[test]
    fn an_empty_comment_closes_the_header_block() {
        let document = parse_ok("; case: x\n;\n; a trailing remark about nothing\n(y)\n");
        let headers = document.comment_headers();
        assert_eq!(headers.len(), 1);
        assert_eq!(headers[0], ("case".into(), "x".into()));
    }

    #[test]
    fn a_continuation_before_any_header_is_dropped_rather_than_inventing_one() {
        let document = parse_ok("; a preamble line\n; case: x\n(y)\n");
        let headers = document.comment_headers();
        assert_eq!(headers, vec![("case".to_string(), "x".to_string())]);
    }

    #[test]
    fn a_semicolon_inside_a_string_is_not_a_comment() {
        let document = parse_ok("(x \"a ; not a comment\")");
        assert!(document.comments.is_empty());
        assert_eq!(
            document.forms[0].items()[1].to_canonical(),
            "\"a ; not a comment\""
        );
    }

    #[test]
    fn a_stray_control_character_is_reported_and_skipped() {
        let (_, diagnostics, sources) = parse("(a \u{7} b)");
        let rendered = diagnostics.render(&sources);
        assert!(rendered.contains("read-unexpected-character"), "{rendered}");
    }

    #[test]
    fn empty_input_is_an_empty_document_not_an_error() {
        let document = parse_ok("");
        assert!(document.forms.is_empty());
        let only_trivia = parse_ok("\n  ; just a comment\n\n");
        assert!(only_trivia.forms.is_empty());
        assert_eq!(only_trivia.comments.len(), 1);
    }

    #[test]
    fn an_empty_list_is_a_list() {
        let document = parse_ok("()");
        assert_eq!(document.forms[0].items().len(), 0);
        assert_eq!(document.to_canonical().trim(), "()");
    }

    #[test]
    fn canonical_text_round_trips_semantically() {
        // §12 M1's exit gate: "examples parse, type-check, and round-trip semantically".
        let source = "(defblock timer.counter\n  ; inert\n  (offers (counter-width 32 bit)\n          (tick-rate 10.5 MHz)\n          (name \"clint\")))\n";
        let first = parse_ok(source);
        let second = parse_ok(&first.to_canonical());
        assert!(
            first.structurally_eq(&second),
            "round trip changed the structure:\n{}\n{}",
            first.to_canonical(),
            second.to_canonical()
        );
        assert_eq!(first.to_canonical(), second.to_canonical());
    }

    #[test]
    fn whitespace_differences_canonicalize_away() {
        let a = parse_ok("(a   b\n\n   c)");
        let b = parse_ok("(a b c)");
        assert_eq!(a.to_canonical(), b.to_canonical());
    }
}
