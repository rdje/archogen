//! The MCP server's JSON (leaf `API.6.3`; `docs/decisions/decision_mcp-server.md` §5, §6): a reader that refuses
//! every text RFC 8259 does not admit and bounds what it will read, and a writer with one way to write each value.
//!
//! ⭐ **Why the server's own.** The server reads untrusted text from whoever drives it. A reader written here is
//! code this repository reviews, with a nesting bound and a size bound it chooses, and no dependency to trust
//! (§6: "no dependency, no process"). It reads one JSON text into a [`Value`]; what a message means is the server's.
//!
//! **What the reader refuses**, each as a [`Refusal`] naming the byte where it stopped:
//! - a text longer than the bound it is given, before reading any of it;
//! - bytes that are not UTF-8, which RFC 8259 §8.1 requires of JSON exchanged between systems, and a byte order
//!   mark, which it says implementations "MUST NOT add";
//! - anything outside the grammar of RFC 8259 §2–§7: a missing or extra comma, a bare word, a number with a
//!   leading zero or no digits, a string with a raw control character or an escape the grammar does not list,
//!   text after the value;
//! - an escaped lone surrogate, which names no character (RFC 8259 §8.2 leaves its behaviour "unpredictable");
//! - an object that names a member twice, whose meaning RFC 8259 §4 calls "unpredictable";
//! - arrays and objects nested deeper than [`MAX_DEPTH`].
//!
//! **What the writer writes**: no whitespace between tokens; an object's members in the order they were made;
//! a string with `"` and `\` escaped, the five control characters that have short escapes written with them,
//! every other character below `0x20` as `\u00xx`, and nothing else escaped — the wasm binding's rule
//! (`crates/archogen-wasm/src/json.rs`), so the two write a string the same way; a number as its text.
//!
//! **A number keeps its text.** It is checked against the grammar and never converted, so a request's `id` is
//! answered with exactly the text it came with, as JSON-RPC 2.0 asks, and no precision is lost to a float.

use std::fmt;

/// The deepest nesting of arrays and objects the reader accepts. A message the server reads or writes nests
/// fewer than ten deep; the bound keeps a hostile text from exhausting the reader's stack.
pub const MAX_DEPTH: usize = 64;

/// A JSON value.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Value {
    Null,
    Bool(bool),
    Number(Number),
    String(String),
    Array(Vec<Value>),
    /// Members in the order they were read or made. The reader refuses a name given twice.
    Object(Vec<(String, Value)>),
}

/// A number, as the text RFC 8259 §6's grammar admits.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Number(String);

impl Number {
    /// The number's text, exactly as it was read or made.
    pub fn as_str(&self) -> &str {
        &self.0
    }

    /// The number as an integer, if its text is one — no fraction, no exponent — and it fits.
    pub fn as_i64(&self) -> Option<i64> {
        if self.0.contains(['.', 'e', 'E']) {
            return None;
        }
        self.0.parse().ok()
    }
}

impl From<i64> for Number {
    fn from(value: i64) -> Self {
        Self(value.to_string())
    }
}

impl Value {
    /// The value of the member `name`, if this is an object that has one.
    pub fn get(&self, name: &str) -> Option<&Value> {
        match self {
            Self::Object(members) => members
                .iter()
                .find(|(key, _)| key == name)
                .map(|(_, value)| value),
            _ => None,
        }
    }

    /// The string, if this is one.
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::String(text) => Some(text),
            _ => None,
        }
    }
}

/// Why a text was not read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum RefusalKind {
    /// Longer than the bound the caller gave.
    TooLong { limit: usize },
    /// Not UTF-8.
    NotUtf8,
    /// A byte order mark before the value.
    ByteOrderMark,
    /// The text ended where the grammar needs more.
    UnexpectedEnd,
    /// A byte the grammar does not admit here.
    Unexpected,
    /// A number outside RFC 8259 §6's grammar.
    BadNumber,
    /// A raw character below `0x20` inside a string.
    ControlCharacter,
    /// An escape RFC 8259 §7 does not list, or `\u` without four hexadecimal digits.
    BadEscape,
    /// A `\u` escape of a surrogate that is not half of a pair.
    LoneSurrogate,
    /// Arrays and objects nested deeper than [`MAX_DEPTH`].
    TooDeep,
    /// An object that names a member twice.
    DuplicateName,
    /// Text after the value.
    TrailingText,
}

/// A text the reader refused: why, and the byte offset where it stopped.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Refusal {
    pub kind: RefusalKind,
    pub at: usize,
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let what = match self.kind {
            RefusalKind::TooLong { limit } => {
                return write!(f, "the text is longer than {limit} bytes")
            }
            RefusalKind::NotUtf8 => "bytes that are not UTF-8",
            RefusalKind::ByteOrderMark => "a byte order mark",
            RefusalKind::UnexpectedEnd => "the text ends inside a value",
            RefusalKind::Unexpected => "a character JSON does not admit here",
            RefusalKind::BadNumber => "a number outside JSON's grammar",
            RefusalKind::ControlCharacter => "a raw control character inside a string",
            RefusalKind::BadEscape => "an escape JSON does not admit",
            RefusalKind::LoneSurrogate => "an escaped surrogate that is not half of a pair",
            RefusalKind::TooDeep => {
                return write!(
                    f,
                    "arrays and objects nested deeper than {MAX_DEPTH}, at byte {}",
                    self.at
                )
            }
            RefusalKind::DuplicateName => "an object that names a member twice",
            RefusalKind::TrailingText => "text after the value",
        };
        write!(f, "{what}, at byte {}", self.at)
    }
}

/// Read one JSON text of at most `limit` bytes.
pub fn read(bytes: &[u8], limit: usize) -> Result<Value, Refusal> {
    if bytes.len() > limit {
        return Err(Refusal {
            kind: RefusalKind::TooLong { limit },
            at: limit,
        });
    }
    let text = std::str::from_utf8(bytes).map_err(|error| Refusal {
        kind: RefusalKind::NotUtf8,
        at: error.valid_up_to(),
    })?;
    if text.starts_with('\u{feff}') {
        return Err(Refusal {
            kind: RefusalKind::ByteOrderMark,
            at: 0,
        });
    }
    let mut reader = Reader {
        bytes: text.as_bytes(),
        at: 0,
    };
    reader.whitespace();
    let value = reader.value(0)?;
    reader.whitespace();
    if reader.at < reader.bytes.len() {
        return Err(reader.refuse(RefusalKind::TrailingText));
    }
    Ok(value)
}

struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl Reader<'_> {
    fn refuse(&self, kind: RefusalKind) -> Refusal {
        Refusal { kind, at: self.at }
    }

    fn peek(&self) -> Option<u8> {
        self.bytes.get(self.at).copied()
    }

    /// The next byte, which must exist.
    fn bump(&mut self) -> Result<u8, Refusal> {
        let byte = self
            .peek()
            .ok_or_else(|| self.refuse(RefusalKind::UnexpectedEnd))?;
        self.at += 1;
        Ok(byte)
    }

    /// RFC 8259 §2's whitespace: space, tab, line feed, carriage return, and nothing else.
    fn whitespace(&mut self) {
        while matches!(self.peek(), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.at += 1;
        }
    }

    fn expect(&mut self, byte: u8) -> Result<(), Refusal> {
        match self.peek() {
            Some(found) if found == byte => {
                self.at += 1;
                Ok(())
            }
            Some(_) => Err(self.refuse(RefusalKind::Unexpected)),
            None => Err(self.refuse(RefusalKind::UnexpectedEnd)),
        }
    }

    /// A value, `depth` arrays and objects deep already.
    fn value(&mut self, depth: usize) -> Result<Value, Refusal> {
        match self.peek() {
            None => Err(self.refuse(RefusalKind::UnexpectedEnd)),
            Some(b'{') => self.object(depth + 1),
            Some(b'[') => self.array(depth + 1),
            Some(b'"') => self.string().map(Value::String),
            Some(b'-' | b'0'..=b'9') => self.number().map(Value::Number),
            Some(b't') => self.word("true", Value::Bool(true)),
            Some(b'f') => self.word("false", Value::Bool(false)),
            Some(b'n') => self.word("null", Value::Null),
            Some(_) => Err(self.refuse(RefusalKind::Unexpected)),
        }
    }

    fn word(&mut self, word: &str, value: Value) -> Result<Value, Refusal> {
        for &byte in word.as_bytes() {
            self.expect(byte)?;
        }
        Ok(value)
    }

    fn array(&mut self, depth: usize) -> Result<Value, Refusal> {
        if depth > MAX_DEPTH {
            return Err(self.refuse(RefusalKind::TooDeep));
        }
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.whitespace();
        if self.peek() == Some(b']') {
            self.at += 1;
            return Ok(Value::Array(items));
        }
        loop {
            self.whitespace();
            items.push(self.value(depth)?);
            self.whitespace();
            match self.bump()? {
                b',' => continue,
                b']' => return Ok(Value::Array(items)),
                _ => {
                    self.at -= 1;
                    return Err(self.refuse(RefusalKind::Unexpected));
                }
            }
        }
    }

    fn object(&mut self, depth: usize) -> Result<Value, Refusal> {
        if depth > MAX_DEPTH {
            return Err(self.refuse(RefusalKind::TooDeep));
        }
        self.expect(b'{')?;
        let mut members: Vec<(String, Value)> = Vec::new();
        self.whitespace();
        if self.peek() == Some(b'}') {
            self.at += 1;
            return Ok(Value::Object(members));
        }
        loop {
            self.whitespace();
            let start = self.at;
            if self.peek() != Some(b'"') {
                return Err(if self.peek().is_none() {
                    self.refuse(RefusalKind::UnexpectedEnd)
                } else {
                    self.refuse(RefusalKind::Unexpected)
                });
            }
            let name = self.string()?;
            if members.iter().any(|(key, _)| *key == name) {
                return Err(Refusal {
                    kind: RefusalKind::DuplicateName,
                    at: start,
                });
            }
            self.whitespace();
            self.expect(b':')?;
            self.whitespace();
            let value = self.value(depth)?;
            members.push((name, value));
            self.whitespace();
            match self.bump()? {
                b',' => continue,
                b'}' => return Ok(Value::Object(members)),
                _ => {
                    self.at -= 1;
                    return Err(self.refuse(RefusalKind::Unexpected));
                }
            }
        }
    }

    /// RFC 8259 §6: `-? (0 | [1-9][0-9]*) (. [0-9]+)? ([eE] [+-]? [0-9]+)?`.
    fn number(&mut self) -> Result<Number, Refusal> {
        let start = self.at;
        let bad = |reader: &Self| Refusal {
            kind: RefusalKind::BadNumber,
            at: reader.at,
        };
        if self.peek() == Some(b'-') {
            self.at += 1;
        }
        match self.peek() {
            Some(b'0') => {
                self.at += 1;
                if matches!(self.peek(), Some(b'0'..=b'9')) {
                    return Err(bad(self));
                }
            }
            Some(b'1'..=b'9') => self.digits(),
            _ => return Err(bad(self)),
        }
        if self.peek() == Some(b'.') {
            self.at += 1;
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(bad(self));
            }
            self.digits();
        }
        if matches!(self.peek(), Some(b'e' | b'E')) {
            self.at += 1;
            if matches!(self.peek(), Some(b'+' | b'-')) {
                self.at += 1;
            }
            if !matches!(self.peek(), Some(b'0'..=b'9')) {
                return Err(bad(self));
            }
            self.digits();
        }
        // Every byte consumed is ASCII, so the slice is a whole UTF-8 text.
        let text = std::str::from_utf8(&self.bytes[start..self.at]).map_err(|_| bad(self))?;
        Ok(Number(text.to_owned()))
    }

    fn digits(&mut self) {
        while matches!(self.peek(), Some(b'0'..=b'9')) {
            self.at += 1;
        }
    }

    /// A string, its quotes included; RFC 8259 §7.
    fn string(&mut self) -> Result<String, Refusal> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            let start = self.at;
            // The run of bytes that need no work: everything but `"`, `\` and the controls.
            while matches!(self.peek(), Some(byte) if byte != b'"' && byte != b'\\' && byte >= 0x20)
            {
                self.at += 1;
            }
            if self.at > start {
                // The text was checked as UTF-8 whole, and the run ends before an ASCII byte or at the end, so
                // it is a whole UTF-8 text.
                let run =
                    std::str::from_utf8(&self.bytes[start..self.at]).map_err(|_| Refusal {
                        kind: RefusalKind::NotUtf8,
                        at: start,
                    })?;
                out.push_str(run);
            }
            match self.peek() {
                None => return Err(self.refuse(RefusalKind::UnexpectedEnd)),
                Some(b'"') => {
                    self.at += 1;
                    return Ok(out);
                }
                Some(b'\\') => {
                    let escape_at = self.at;
                    self.at += 1;
                    let escaped = match self.bump()? {
                        b'"' => '"',
                        b'\\' => '\\',
                        b'/' => '/',
                        b'b' => '\u{08}',
                        b'f' => '\u{0c}',
                        b'n' => '\n',
                        b'r' => '\r',
                        b't' => '\t',
                        b'u' => self.unicode(escape_at)?,
                        _ => {
                            return Err(Refusal {
                                kind: RefusalKind::BadEscape,
                                at: escape_at,
                            })
                        }
                    };
                    out.push(escaped);
                }
                Some(_) => return Err(self.refuse(RefusalKind::ControlCharacter)),
            }
        }
    }

    /// The character of a `\u` escape whose backslash is at `escape_at`, the `\u` read: four hexadecimal digits,
    /// and for a high surrogate the `\u` escape of its low half.
    fn unicode(&mut self, escape_at: usize) -> Result<char, Refusal> {
        let first = self.hex4(escape_at)?;
        let lone = Refusal {
            kind: RefusalKind::LoneSurrogate,
            at: escape_at,
        };
        match first {
            0xD800..=0xDBFF => {
                if self.bytes.get(self.at..self.at + 2) != Some(b"\\u") {
                    return Err(lone);
                }
                let low_at = self.at;
                self.at += 2;
                let second = self.hex4(low_at)?;
                if !(0xDC00..=0xDFFF).contains(&second) {
                    return Err(lone);
                }
                let code = 0x10000 + ((first - 0xD800) << 10) + (second - 0xDC00);
                char::from_u32(code).ok_or(lone)
            }
            0xDC00..=0xDFFF => Err(lone),
            code => char::from_u32(code).ok_or(lone),
        }
    }

    fn hex4(&mut self, escape_at: usize) -> Result<u32, Refusal> {
        let mut code = 0;
        for _ in 0..4 {
            let digit = match self.peek() {
                Some(byte @ b'0'..=b'9') => byte - b'0',
                Some(byte @ b'a'..=b'f') => byte - b'a' + 10,
                Some(byte @ b'A'..=b'F') => byte - b'A' + 10,
                _ => {
                    return Err(Refusal {
                        kind: RefusalKind::BadEscape,
                        at: escape_at,
                    })
                }
            };
            self.at += 1;
            code = code * 16 + u32::from(digit);
        }
        Ok(code)
    }
}

/// Write a value: one way for each.
pub fn write(value: &Value) -> String {
    let mut out = String::new();
    write_into(&mut out, value);
    out
}

fn write_into(out: &mut String, value: &Value) {
    match value {
        Value::Null => out.push_str("null"),
        Value::Bool(true) => out.push_str("true"),
        Value::Bool(false) => out.push_str("false"),
        Value::Number(number) => out.push_str(number.as_str()),
        Value::String(text) => write_string(out, text),
        Value::Array(items) => {
            out.push('[');
            for (index, item) in items.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_into(out, item);
            }
            out.push(']');
        }
        Value::Object(members) => {
            out.push('{');
            for (index, (name, item)) in members.iter().enumerate() {
                if index > 0 {
                    out.push(',');
                }
                write_string(out, name);
                out.push(':');
                write_into(out, item);
            }
            out.push('}');
        }
    }
}

/// A string as the wasm binding writes one: `"` and `\` escaped, `\b \t \n \f \r` by their short escapes, every
/// other character below `0x20` as `\u00xx`, and nothing else.
fn write_string(out: &mut String, text: &str) {
    const DIGITS: &[u8; 16] = b"0123456789abcdef";
    out.push('"');
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\u{0c}' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            c if u32::from(c) < 0x20 => {
                let byte = u32::from(c);
                out.push_str("\\u00");
                out.push(char::from(DIGITS[(byte >> 4) as usize]));
                out.push(char::from(DIGITS[(byte & 0x0f) as usize]));
            }
            c => out.push(c),
        }
    }
    out.push('"');
}

#[cfg(test)]
mod tests {
    use super::*;

    const LIMIT: usize = 1 << 20;

    fn refused(text: &str) -> RefusalKind {
        match read(text.as_bytes(), LIMIT) {
            Err(refusal) => refusal.kind,
            Ok(value) => panic!("{text:?} was read as {value:?}"),
        }
    }

    fn string(text: &str) -> Value {
        Value::String(text.to_owned())
    }

    fn number(text: &str) -> Value {
        Value::Number(Number(text.to_owned()))
    }

    #[test]
    fn every_kind_of_value_is_read() {
        let text = r#" {"a": [1, -2.5e+3, "x\ny", true, false, null, {}, []], "b": {"c": "é😀"}} "#;
        let expected = Value::Object(vec![
            (
                "a".to_owned(),
                Value::Array(vec![
                    number("1"),
                    number("-2.5e+3"),
                    string("x\ny"),
                    Value::Bool(true),
                    Value::Bool(false),
                    Value::Null,
                    Value::Object(vec![]),
                    Value::Array(vec![]),
                ]),
            ),
            (
                "b".to_owned(),
                Value::Object(vec![("c".to_owned(), string("é😀"))]),
            ),
        ]);
        assert_eq!(read(text.as_bytes(), LIMIT), Ok(expected));
    }

    #[test]
    fn every_escape_the_grammar_lists_is_read() {
        let text = r#""\" \\ \/ \b \f \n \r \t \u0041 \u00e9 \ud83d\ude00""#;
        assert_eq!(
            read(text.as_bytes(), LIMIT),
            Ok(string("\" \\ / \u{8} \u{c} \n \r \t A é 😀"))
        );
    }

    #[test]
    fn each_malformed_text_is_refused_for_its_own_reason() {
        for (text, kind) in [
            ("", RefusalKind::UnexpectedEnd),
            ("   ", RefusalKind::UnexpectedEnd),
            ("[1,", RefusalKind::UnexpectedEnd),
            ("\"open", RefusalKind::UnexpectedEnd),
            ("[1,]", RefusalKind::Unexpected),
            ("[1 2]", RefusalKind::Unexpected),
            ("{\"a\" 1}", RefusalKind::Unexpected),
            ("{\"a\":1,}", RefusalKind::Unexpected),
            ("{a:1}", RefusalKind::Unexpected),
            ("'a'", RefusalKind::Unexpected),
            ("tru", RefusalKind::UnexpectedEnd),
            ("nul1", RefusalKind::Unexpected),
            ("NaN", RefusalKind::Unexpected),
            ("+1", RefusalKind::Unexpected),
            ("01", RefusalKind::BadNumber),
            ("-", RefusalKind::BadNumber),
            ("1.", RefusalKind::BadNumber),
            (".5", RefusalKind::Unexpected),
            ("1e", RefusalKind::BadNumber),
            ("1e+", RefusalKind::BadNumber),
            ("\"a\tb\"", RefusalKind::ControlCharacter),
            ("\"a\u{1}b\"", RefusalKind::ControlCharacter),
            ("\"\\x41\"", RefusalKind::BadEscape),
            ("\"\\u00G1\"", RefusalKind::BadEscape),
            ("\"\\u12\"", RefusalKind::BadEscape),
            ("\"\\ud800\"", RefusalKind::LoneSurrogate),
            ("\"\\ud800\\u0041\"", RefusalKind::LoneSurrogate),
            ("\"\\udc00\"", RefusalKind::LoneSurrogate),
            ("{\"a\":1,\"a\":2}", RefusalKind::DuplicateName),
            ("{\"a\":1,\"\\u0061\":2}", RefusalKind::DuplicateName),
            ("1 2", RefusalKind::TrailingText),
            ("{} //", RefusalKind::TrailingText),
            ("\u{feff}{}", RefusalKind::ByteOrderMark),
            ("\u{a0}1", RefusalKind::Unexpected),
        ] {
            assert_eq!(refused(text), kind, "{text:?}");
        }
    }

    #[test]
    fn a_refusal_names_the_byte_where_the_reader_stopped() {
        assert_eq!(
            read(b"[1, 2,]", LIMIT),
            Err(Refusal {
                kind: RefusalKind::Unexpected,
                at: 6
            })
        );
        assert_eq!(
            read(b"{\"a\":1, \"a\":2}", LIMIT),
            Err(Refusal {
                kind: RefusalKind::DuplicateName,
                at: 8
            })
        );
        assert_eq!(
            read(b"\"ab\\q\"", LIMIT),
            Err(Refusal {
                kind: RefusalKind::BadEscape,
                at: 3
            })
        );
    }

    #[test]
    fn bytes_that_are_not_utf8_are_refused() {
        assert_eq!(
            read(b"\"a\xffb\"", LIMIT),
            Err(Refusal {
                kind: RefusalKind::NotUtf8,
                at: 2
            })
        );
        assert_eq!(
            read(b"\"\xed\xa0\x80\"", LIMIT).map_err(|r| r.kind),
            Err(RefusalKind::NotUtf8)
        );
    }

    #[test]
    fn the_size_bound_holds_at_both_edges() {
        let text = br#"{"a":"bcd"}"#;
        assert!(
            read(text, text.len()).is_ok(),
            "a text exactly at the bound is read"
        );
        assert_eq!(
            read(text, text.len() - 1),
            Err(Refusal {
                kind: RefusalKind::TooLong {
                    limit: text.len() - 1
                },
                at: text.len() - 1
            })
        );
    }

    #[test]
    fn the_size_bound_is_applied_before_any_reading() {
        // Malformed and too long: the length is what is reported, so nothing of it was read.
        assert_eq!(
            read(b"\xff\xff\xff", 2).map_err(|r| r.kind),
            Err(RefusalKind::TooLong { limit: 2 })
        );
    }

    fn nested(open: &str, close: &str, depth: usize) -> String {
        format!("{}{}", open.repeat(depth), close.repeat(depth))
    }

    #[test]
    fn the_nesting_bound_holds_at_both_edges_for_arrays_and_objects() {
        let arrays = nested("[", "]", MAX_DEPTH);
        assert!(
            read(arrays.as_bytes(), LIMIT).is_ok(),
            "{MAX_DEPTH} arrays deep is read"
        );
        let deeper = nested("[", "]", MAX_DEPTH + 1);
        assert_eq!(
            read(deeper.as_bytes(), LIMIT),
            Err(Refusal {
                kind: RefusalKind::TooDeep,
                at: MAX_DEPTH
            })
        );

        let objects = format!("{}1{}", "{\"a\":".repeat(MAX_DEPTH), "}".repeat(MAX_DEPTH));
        assert!(
            read(objects.as_bytes(), LIMIT).is_ok(),
            "{MAX_DEPTH} objects deep is read"
        );
        let deeper = format!(
            "{}1{}",
            "{\"a\":".repeat(MAX_DEPTH + 1),
            "}".repeat(MAX_DEPTH + 1)
        );
        assert_eq!(
            read(deeper.as_bytes(), LIMIT).map_err(|r| r.kind),
            Err(RefusalKind::TooDeep)
        );
    }

    #[test]
    fn a_hostile_depth_is_refused_without_exhausting_the_stack() {
        let text = "[".repeat(1_000_000);
        assert_eq!(
            read(text.as_bytes(), LIMIT).map_err(|r| r.kind),
            Err(RefusalKind::TooDeep)
        );
    }

    #[test]
    fn a_number_keeps_its_text() {
        for text in [
            "0",
            "-0",
            "1",
            "-1",
            "1.0",
            "1e5",
            "1E-5",
            "123456789012345678901234567890",
            "-0.0e+0",
        ] {
            assert_eq!(read(text.as_bytes(), LIMIT), Ok(number(text)), "{text}");
            assert_eq!(write(&number(text)), text);
        }
        assert_eq!(Number("42".to_owned()).as_i64(), Some(42));
        assert_eq!(Number("4.2".to_owned()).as_i64(), None);
        assert_eq!(Number("1e2".to_owned()).as_i64(), None);
        assert_eq!(Number("99999999999999999999".to_owned()).as_i64(), None);
        assert_eq!(write(&Value::Number(Number::from(-7))), "-7");
    }

    #[test]
    fn the_writer_has_one_way_to_write_each_string() {
        let all_controls: String = (0u8..0x20).map(char::from).collect();
        let written = write(&string(&format!("\"\\/{all_controls}\u{7f}é\u{2028}")));
        assert_eq!(
            written,
            "\"\\\"\\\\/\\u0000\\u0001\\u0002\\u0003\\u0004\\u0005\\u0006\\u0007\\b\\t\\n\\u000b\\f\\r\\u000e\\u000f\
             \\u0010\\u0011\\u0012\\u0013\\u0014\\u0015\\u0016\\u0017\\u0018\\u0019\\u001a\\u001b\\u001c\\u001d\
             \\u001e\\u001f\u{7f}é\u{2028}\""
        );
    }

    #[test]
    fn the_writer_writes_no_whitespace_and_keeps_member_order() {
        let value = Value::Object(vec![
            ("z".to_owned(), Value::Array(vec![number("1"), Value::Null])),
            ("a".to_owned(), Value::Object(vec![])),
        ]);
        assert_eq!(write(&value), r#"{"z":[1,null],"a":{}}"#);
    }

    /// Message shapes like the server's, both eras, written before the server: each reads back to the value, and is
    /// its own canonical text. The server's own answers are round-tripped in `mcp.rs`'s tests and `tests/mcp_stdio.rs`.
    #[test]
    fn the_messages_the_server_sends_round_trip() {
        for text in [
            r#"{"jsonrpc":"2.0","id":1,"result":{"supportedVersions":["2026-07-28","2025-11-25"],"serverInfo":{"name":"archogen","version":"0.1.0"},"capabilities":{"tools":{}},"resultType":"complete"}}"#,
            r#"{"jsonrpc":"2.0","id":"a","result":{"tools":[{"name":"check","description":"Check an eADL description","inputSchema":{"type":"object","properties":{"description":{"type":"string"}},"required":["description"]},"annotations":{"readOnlyHint":true,"destructiveHint":false,"idempotentHint":true,"openWorldHint":false},"_meta":{"io.github.rdje.archogen/maturity":"partial"}}]}}"#,
            r#"{"jsonrpc":"2.0","id":7,"result":{"content":[{"type":"text","text":"{\"status\":\"accepted\"}"}],"structuredContent":{"status":"accepted"},"isError":false}}"#,
            r#"{"jsonrpc":"2.0","id":null,"error":{"code":-32700,"message":"parse error: a character JSON does not admit here, at byte 0"}}"#,
            r#"{"jsonrpc":"2.0","id":3,"error":{"code":-32022,"message":"unsupported protocol version","data":{"supported":["2026-07-28","2025-11-25"],"requested":"2024-01-01"}}}"#,
            r#"{"jsonrpc":"2.0","id":2,"result":{"protocolVersion":"2025-11-25","capabilities":{"tools":{}},"serverInfo":{"name":"archogen","version":"0.1.0"}}}"#,
        ] {
            let value =
                read(text.as_bytes(), LIMIT).unwrap_or_else(|refusal| panic!("{refusal}: {text}"));
            assert_eq!(write(&value), text);
        }
    }

    /// A deterministic generator of values (no dependency): every one written reads back to itself.
    #[test]
    fn every_value_written_reads_back_to_itself() {
        struct Lcg(u64);
        impl Lcg {
            fn next(&mut self, n: u64) -> u64 {
                self.0 = self
                    .0
                    .wrapping_mul(6_364_136_223_846_793_005)
                    .wrapping_add(1_442_695_040_888_963_407);
                (self.0 >> 33) % n
            }
        }
        fn text(rng: &mut Lcg) -> String {
            const PIECES: [&str; 12] = [
                "a", "\"", "\\", "\n", "\u{1}", "\u{1f}", "é", "😀", "\u{7f}", "/", " ", "\u{2028}",
            ];
            (0..rng.next(6))
                .map(|_| PIECES[rng.next(PIECES.len() as u64) as usize])
                .collect()
        }
        fn value(rng: &mut Lcg, depth: usize) -> Value {
            match rng.next(if depth >= 6 { 4 } else { 6 }) {
                0 => Value::Null,
                1 => Value::Bool(rng.next(2) == 0),
                2 => Value::Number(Number::from(rng.next(2_000_000) as i64 - 1_000_000)),
                3 => Value::String(text(rng)),
                4 => Value::Array((0..rng.next(4)).map(|_| value(rng, depth + 1)).collect()),
                _ => {
                    let mut members: Vec<(String, Value)> = Vec::new();
                    for index in 0..rng.next(4) {
                        members.push((format!("{index}{}", text(rng)), value(rng, depth + 1)));
                    }
                    Value::Object(members)
                }
            }
        }
        let mut rng = Lcg(0x5eed);
        for _ in 0..2_000 {
            let original = value(&mut rng, 0);
            let written = write(&original);
            assert_eq!(read(written.as_bytes(), LIMIT), Ok(original), "{written}");
        }
    }
}
