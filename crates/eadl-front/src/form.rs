//! The parsed form tree and its canonical printer.
//!
//! A [`Form`] is syntax, not meaning. `(counter-width 32 bit)` is a list of a symbol, an
//! integer and a symbol; that `bit` is a unit and `32` is its magnitude is the model layer's
//! business (`eadl-model`). Keeping the reader purely syntactic is what lets the same reader
//! serve the boundary corpus, the S0 fixture and the M1 semantic corpus without any of them
//! leaking assumptions into it.
//!
//! **Numbers are exact.** There is no float variant. §7.4 requires "exact integer time units or
//! checked rational arithmetic", and a reader that produces `f64` has already lost the property
//! before any analysis runs — a `0.1 ms` in a description would silently become a value that is
//! not one tenth of a millisecond. A decimal literal is kept as an integer and a scale, so
//! `1.5` is exactly 15/10.

use crate::source::Span;

/// A parsed form.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Form {
    /// `(a b c)`.
    List {
        /// The elements, in source order.
        items: Vec<Form>,
        /// The span from the opening to the closing parenthesis, inclusive.
        span: Span,
    },
    /// A bare atom: `defservice`, `time.monotonic`, `at-least`, `MHz`.
    Symbol {
        /// The text as written.
        name: String,
        /// Its span.
        span: Span,
    },
    /// An exact integer literal, written decimal or hexadecimal.
    Integer {
        /// The value.
        value: i64,
        /// Its span.
        span: Span,
    },
    /// An exact decimal literal, held as `value / 10^scale`.
    Decimal {
        /// The digits with the point removed.
        value: i64,
        /// How many digits were after the point.
        scale: u32,
        /// Its span.
        span: Span,
    },
    /// A double-quoted string.
    Str {
        /// The decoded value, with escapes resolved.
        value: String,
        /// The span including both quotes.
        span: Span,
    },
}

impl Form {
    /// The span this form covers.
    #[must_use]
    pub const fn span(&self) -> Span {
        match self {
            Self::List { span, .. }
            | Self::Symbol { span, .. }
            | Self::Integer { span, .. }
            | Self::Decimal { span, .. }
            | Self::Str { span, .. } => *span,
        }
    }

    /// The head symbol of a list, e.g. `defservice` in `(defservice …)`.
    #[must_use]
    pub fn head(&self) -> Option<&str> {
        match self {
            Self::List { items, .. } => match items.first() {
                Some(Self::Symbol { name, .. }) => Some(name.as_str()),
                _ => None,
            },
            _ => None,
        }
    }

    /// The elements of a list, or an empty slice for an atom.
    #[must_use]
    pub fn items(&self) -> &[Self] {
        match self {
            Self::List { items, .. } => items,
            _ => &[],
        }
    }

    /// This form's symbol text, if it is a symbol.
    #[must_use]
    pub fn as_symbol(&self) -> Option<&str> {
        match self {
            Self::Symbol { name, .. } => Some(name.as_str()),
            _ => None,
        }
    }

    /// A short name for the kind of form, for diagnostics.
    #[must_use]
    pub const fn kind(&self) -> &'static str {
        match self {
            Self::List { .. } => "list",
            Self::Symbol { .. } => "symbol",
            Self::Integer { .. } => "integer",
            Self::Decimal { .. } => "decimal",
            Self::Str { .. } => "string",
        }
    }

    /// Structural equality ignoring spans.
    ///
    /// This is what "round-trips semantically" means (§12 M1): printing a form and reading it
    /// back yields the same *structure*, not the same byte offsets — offsets necessarily change
    /// when whitespace is normalized.
    #[must_use]
    pub fn structurally_eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::List { items: a, .. }, Self::List { items: b, .. }) => {
                a.len() == b.len() && a.iter().zip(b.iter()).all(|(x, y)| x.structurally_eq(y))
            }
            (Self::Symbol { name: a, .. }, Self::Symbol { name: b, .. }) => a == b,
            (Self::Integer { value: a, .. }, Self::Integer { value: b, .. }) => a == b,
            (
                Self::Decimal {
                    value: a,
                    scale: sa,
                    ..
                },
                Self::Decimal {
                    value: b,
                    scale: sb,
                    ..
                },
            ) => a == b && sa == sb,
            (Self::Str { value: a, .. }, Self::Str { value: b, .. }) => a == b,
            _ => false,
        }
    }

    /// Print in canonical form: one space between siblings, no comments, no line breaks.
    ///
    /// Canonical text is what gets hashed and compared. §12 M4 requires "identical canonical
    /// plans and generated sources" from repeated generation, and that starts here: two
    /// descriptions that differ only in whitespace must print identically.
    #[must_use]
    pub fn to_canonical(&self) -> String {
        let mut out = String::new();
        self.write_canonical(&mut out);
        out
    }

    fn write_canonical(&self, out: &mut String) {
        match self {
            Self::List { items, .. } => {
                out.push('(');
                for (index, item) in items.iter().enumerate() {
                    if index > 0 {
                        out.push(' ');
                    }
                    item.write_canonical(out);
                }
                out.push(')');
            }
            Self::Symbol { name, .. } => out.push_str(name),
            Self::Integer { value, .. } => out.push_str(&value.to_string()),
            Self::Decimal { value, scale, .. } => {
                out.push_str(&format_decimal(*value, *scale));
            }
            Self::Str { value, .. } => {
                out.push('"');
                for ch in value.chars() {
                    match ch {
                        '"' => out.push_str("\\\""),
                        '\\' => out.push_str("\\\\"),
                        '\n' => out.push_str("\\n"),
                        '\t' => out.push_str("\\t"),
                        _ => out.push(ch),
                    }
                }
                out.push('"');
            }
        }
    }
}

/// Split `key: value` when the part before the first colon is a bare kebab-case key.
///
/// The key test is what separates a header from prose that happens to contain a colon — and
/// corpus rationales contain plenty of those ("§7.3 is precise about the split:"). A key has
/// no spaces, so such a line is never mistaken for one.
fn split_header(line: &str) -> Option<(String, String)> {
    let (key, value) = line.split_once(':')?;
    let key = key.trim();
    if key.is_empty()
        || !key
            .chars()
            .all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-')
    {
        return None;
    }
    Some((key.to_string(), value.trim().to_string()))
}

/// Render `value / 10^scale` exactly, with `scale` digits after the point.
fn format_decimal(value: i64, scale: u32) -> String {
    if scale == 0 {
        return value.to_string();
    }
    let negative = value < 0;
    let digits = value.unsigned_abs().to_string();
    let scale = scale as usize;
    let padded = if digits.len() <= scale {
        format!("{}{}", "0".repeat(scale - digits.len() + 1), digits)
    } else {
        digits
    };
    let split = padded.len() - scale;
    format!(
        "{}{}.{}",
        if negative { "-" } else { "" },
        &padded[..split],
        &padded[split..]
    )
}

/// A `;` comment, kept with its span.
///
/// Comments are semantically inert to the language, and they are *not* discarded, because the
/// boundary corpus carries its case metadata in them (`; verdict: accept`). A reader that threw
/// them away would force a second, divergent parser to exist just to read those headers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Comment {
    /// The text after `;`, with the leading `;` removed and trailing whitespace trimmed.
    pub text: String,
    /// The span of the whole comment including its `;`.
    pub span: Span,
}

/// Everything one source parsed to.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Document {
    /// The top-level forms, in source order.
    pub forms: Vec<Form>,
    /// Every comment, in source order.
    pub comments: Vec<Comment>,
}

impl Document {
    /// Canonical text of every top-level form, one per line. Comments are not included: they
    /// are inert, and canonical text is what gets compared and hashed.
    #[must_use]
    pub fn to_canonical(&self) -> String {
        let mut out = String::new();
        for form in &self.forms {
            out.push_str(&form.to_canonical());
            out.push('\n');
        }
        out
    }

    /// Whether two documents have the same forms, ignoring spans and comments.
    #[must_use]
    pub fn structurally_eq(&self, other: &Self) -> bool {
        self.forms.len() == other.forms.len()
            && self
                .forms
                .iter()
                .zip(other.forms.iter())
                .all(|(a, b)| a.structurally_eq(b))
    }

    /// Parse `; key: value` comment headers into pairs, in source order.
    ///
    /// This is what the boundary corpus's metadata block is written in, and what F27 reads.
    /// A wrapped value is one value: a comment line that is **not** a header continues the
    /// previous one, and an empty comment line closes the block.
    ///
    /// A line opens a header only when **both** hold:
    ///
    /// 1. it is not indented — at most one space after the `;`, the ordinary `; key: value`
    ///    spacing — while a continuation is indented further; and
    /// 2. the text before its first colon is a bare kebab-case key, with no spaces.
    ///
    /// ⛔ **Both conditions are load-bearing, and each was added after the other alone failed
    /// on a real corpus file.** Indentation alone fails because every comment begins with a
    /// space after the `;`, so every line looks indented. The key shape alone fails because a
    /// wrapped rationale can *begin* with something that is exactly a key — the
    /// `counter-width-and-rate` case wraps onto a line starting
    /// `implementation-independence: a different timer …`, which silently opened a spurious
    /// header and truncated the rationale at that point. Neither test can be dropped.
    #[must_use]
    pub fn comment_headers(&self) -> Vec<(String, String)> {
        let mut pairs: Vec<(String, String)> = Vec::new();
        let mut open = false;
        for comment in &self.comments {
            let trimmed = comment.text.trim();
            if trimmed.is_empty() {
                // A bare `;` ends the block, so a later unrelated comment cannot glue itself
                // onto the last value.
                open = false;
                continue;
            }
            let indented = comment
                .text
                .chars()
                .take_while(|c| c.is_whitespace())
                .count()
                > 1;
            if let Some((key, value)) = (!indented).then(|| split_header(trimmed)).flatten() {
                pairs.push((key, value));
                open = true;
            } else if open {
                if let Some((_, value)) = pairs.last_mut() {
                    value.push(' ');
                    value.push_str(trimmed);
                }
            }
        }
        pairs
    }
}

#[cfg(test)]
mod tests {
    use super::{format_decimal, Form};
    use crate::source::{SourceId, Span};

    fn span() -> Span {
        Span::new(SourceId(0), 0, 0)
    }

    fn sym(name: &str) -> Form {
        Form::Symbol {
            name: name.into(),
            span: span(),
        }
    }

    #[test]
    fn canonical_text_normalizes_spacing() {
        let form = Form::List {
            items: vec![
                sym("a"),
                Form::Integer {
                    value: 3,
                    span: span(),
                },
                sym("b"),
            ],
            span: span(),
        };
        assert_eq!(form.to_canonical(), "(a 3 b)");
    }

    #[test]
    fn decimals_print_exactly_without_a_float_anywhere() {
        // 0.1 is the number binary floating point cannot represent. Holding it as 1/10 means
        // it survives a round trip; holding it as f64 would not.
        assert_eq!(format_decimal(1, 1), "0.1");
        assert_eq!(format_decimal(15, 1), "1.5");
        assert_eq!(format_decimal(1050, 2), "10.50");
        assert_eq!(format_decimal(-25, 2), "-0.25");
        assert_eq!(format_decimal(7, 0), "7");
        assert_eq!(format_decimal(0, 3), "0.000");
    }

    #[test]
    fn strings_round_trip_their_escapes() {
        let form = Form::Str {
            value: "a\"b\\c\nd".into(),
            span: span(),
        };
        assert_eq!(form.to_canonical(), "\"a\\\"b\\\\c\\nd\"");
    }

    #[test]
    fn structural_equality_ignores_spans() {
        let a = Form::Symbol {
            name: "x".into(),
            span: Span::new(SourceId(0), 0, 1),
        };
        let b = Form::Symbol {
            name: "x".into(),
            span: Span::new(SourceId(3), 90, 91),
        };
        assert_ne!(a, b, "derived equality does compare spans");
        assert!(a.structurally_eq(&b), "structural equality must not");
    }

    #[test]
    fn structural_equality_distinguishes_kinds_and_scales() {
        let integer = Form::Integer {
            value: 15,
            span: span(),
        };
        let decimal = Form::Decimal {
            value: 15,
            scale: 1,
            span: span(),
        };
        assert!(!integer.structurally_eq(&decimal));
        let other_scale = Form::Decimal {
            value: 15,
            scale: 2,
            span: span(),
        };
        assert!(!decimal.structurally_eq(&other_scale), "1.5 is not 0.15");
    }

    #[test]
    fn head_and_items_read_a_list() {
        let form = Form::List {
            items: vec![sym("defservice"), sym("time.monotonic")],
            span: span(),
        };
        assert_eq!(form.head(), Some("defservice"));
        assert_eq!(form.items().len(), 2);
        assert_eq!(sym("x").items().len(), 0);
        assert_eq!(sym("x").head(), None);
    }

    #[test]
    fn a_list_whose_head_is_not_a_symbol_has_no_head() {
        let form = Form::List {
            items: vec![Form::Integer {
                value: 1,
                span: span(),
            }],
            span: span(),
        };
        assert_eq!(form.head(), None);
    }
}
