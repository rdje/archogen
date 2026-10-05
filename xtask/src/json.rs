//! Just enough JSON to read `cargo metadata` (leaf `M2.7.4.3`): a value tree, parsed strictly, with the accessors
//! the catalog's builds use. The workspace admits no third-party crate, and the repository's other JSON code is an
//! encoder (`crates/archogen-wasm/src/json.rs`), so this reader lives with the tooling that needs it.

use std::collections::BTreeMap;

/// A JSON value. Numbers are kept as written: the metadata's are versions and sizes this reader never computes with.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Json {
    Null,
    Bool(bool),
    Number(String),
    Str(String),
    Array(Vec<Json>),
    Object(BTreeMap<String, Json>),
}

impl Json {
    /// The member `key` of an object, or `None` for a missing key or a non-object.
    #[must_use]
    pub fn get(&self, key: &str) -> Option<&Json> {
        match self {
            Self::Object(members) => members.get(key),
            _ => None,
        }
    }

    /// The string a value is, or `None`.
    #[must_use]
    pub fn as_str(&self) -> Option<&str> {
        match self {
            Self::Str(s) => Some(s),
            _ => None,
        }
    }

    /// The elements an array has, or none for a non-array.
    #[must_use]
    pub fn elements(&self) -> &[Json] {
        match self {
            Self::Array(items) => items,
            _ => &[],
        }
    }

    /// Whether the value is `null`.
    #[must_use]
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
}

struct Parser<'a> {
    bytes: &'a [u8],
    at: usize,
}

/// Parse one JSON document.
///
/// # Errors
///
/// The byte offset and what was expected there.
/// Write `value` deterministically: object keys in order (a `BTreeMap`'s), no spaces, strings escaped as JSON
/// requires — the form an inventory's hash is taken over (leaf `M3.6.2`).
#[must_use]
pub fn write(value: &Json) -> String {
    let mut out = String::new();
    write_into(value, &mut out);
    out
}

fn write_into(value: &Json, out: &mut String) {
    match value {
        Json::Null => out.push_str("null"),
        Json::Bool(b) => out.push_str(if *b { "true" } else { "false" }),
        Json::Number(n) => out.push_str(n),
        Json::Str(s) => {
            out.push('"');
            for c in s.chars() {
                match c {
                    '"' => out.push_str("\\\""),
                    '\\' => out.push_str("\\\\"),
                    '\n' => out.push_str("\\n"),
                    '\r' => out.push_str("\\r"),
                    '\t' => out.push_str("\\t"),
                    c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
                    c => out.push(c),
                }
            }
            out.push('"');
        }
        Json::Array(items) => {
            out.push('[');
            for (i, item) in items.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_into(item, out);
            }
            out.push(']');
        }
        Json::Object(map) => {
            out.push('{');
            for (i, (k, v)) in map.iter().enumerate() {
                if i > 0 {
                    out.push(',');
                }
                write_into(&Json::Str(k.clone()), out);
                out.push(':');
                write_into(v, out);
            }
            out.push('}');
        }
    }
}

pub fn parse(text: &str) -> Result<Json, String> {
    let mut parser = Parser {
        bytes: text.as_bytes(),
        at: 0,
    };
    let value = parser.value()?;
    parser.skip_space();
    if parser.at != parser.bytes.len() {
        return Err(format!("trailing bytes at {}", parser.at));
    }
    Ok(value)
}

impl Parser<'_> {
    fn skip_space(&mut self) {
        while matches!(self.bytes.get(self.at), Some(b' ' | b'\t' | b'\n' | b'\r')) {
            self.at += 1;
        }
    }

    fn expect(&mut self, byte: u8) -> Result<(), String> {
        self.skip_space();
        if self.bytes.get(self.at) == Some(&byte) {
            self.at += 1;
            Ok(())
        } else {
            Err(format!("expected `{}` at {}", char::from(byte), self.at))
        }
    }

    fn value(&mut self) -> Result<Json, String> {
        self.skip_space();
        match self.bytes.get(self.at) {
            Some(b'{') => self.object(),
            Some(b'[') => self.array(),
            Some(b'"') => self.string().map(Json::Str),
            Some(b't') => self.literal("true", Json::Bool(true)),
            Some(b'f') => self.literal("false", Json::Bool(false)),
            Some(b'n') => self.literal("null", Json::Null),
            Some(b) if *b == b'-' || b.is_ascii_digit() => self.number(),
            Some(b) => Err(format!(
                "unexpected byte `{}` at {}",
                char::from(*b),
                self.at
            )),
            None => Err("unexpected end".to_owned()),
        }
    }

    fn literal(&mut self, word: &str, value: Json) -> Result<Json, String> {
        if self.bytes[self.at..].starts_with(word.as_bytes()) {
            self.at += word.len();
            Ok(value)
        } else {
            Err(format!("expected `{word}` at {}", self.at))
        }
    }

    fn number(&mut self) -> Result<Json, String> {
        let start = self.at;
        while matches!(
            self.bytes.get(self.at),
            Some(b'-' | b'+' | b'.' | b'e' | b'E' | b'0'..=b'9')
        ) {
            self.at += 1;
        }
        let text = &self.bytes[start..self.at];
        if text.is_empty() || text == b"-" {
            return Err(format!("a number at {start}"));
        }
        Ok(Json::Number(String::from_utf8_lossy(text).into_owned()))
    }

    fn string(&mut self) -> Result<String, String> {
        self.expect(b'"')?;
        let mut out = String::new();
        loop {
            let Some(&b) = self.bytes.get(self.at) else {
                return Err("an unterminated string".to_owned());
            };
            self.at += 1;
            match b {
                b'"' => return Ok(out),
                b'\\' => {
                    let Some(&escape) = self.bytes.get(self.at) else {
                        return Err("an unterminated escape".to_owned());
                    };
                    self.at += 1;
                    match escape {
                        b'"' => out.push('"'),
                        b'\\' => out.push('\\'),
                        b'/' => out.push('/'),
                        b'b' => out.push('\u{8}'),
                        b'f' => out.push('\u{c}'),
                        b'n' => out.push('\n'),
                        b'r' => out.push('\r'),
                        b't' => out.push('\t'),
                        b'u' => {
                            let code = self.hex4()?;
                            let ch = if (0xD800..0xDC00).contains(&code) {
                                // A surrogate pair: `\uXXXX\uXXXX`.
                                if self.bytes.get(self.at) != Some(&b'\\')
                                    || self.bytes.get(self.at + 1) != Some(&b'u')
                                {
                                    return Err(format!("a lone surrogate at {}", self.at));
                                }
                                self.at += 2;
                                let low = self.hex4()?;
                                let joined = 0x10000
                                    + ((code - 0xD800) << 10)
                                    + (low.wrapping_sub(0xDC00) & 0x3FF);
                                char::from_u32(joined)
                            } else {
                                char::from_u32(code)
                            };
                            out.push(ch.ok_or_else(|| format!("a bad escape at {}", self.at))?);
                        }
                        other => {
                            return Err(format!(
                                "a bad escape `\\{}` at {}",
                                char::from(other),
                                self.at
                            ))
                        }
                    }
                }
                _ => {
                    // Take the whole UTF-8 sequence this byte starts.
                    let len = match b {
                        0x00..=0x7F => 1,
                        0xC0..=0xDF => 2,
                        0xE0..=0xEF => 3,
                        _ => 4,
                    };
                    let start = self.at - 1;
                    let end = (start + len).min(self.bytes.len());
                    out.push_str(
                        std::str::from_utf8(&self.bytes[start..end])
                            .map_err(|_| format!("bytes that are not UTF-8 at {start}"))?,
                    );
                    self.at = end;
                }
            }
        }
    }

    fn hex4(&mut self) -> Result<u32, String> {
        let digits = self
            .bytes
            .get(self.at..self.at + 4)
            .ok_or_else(|| format!("a short `\\u` escape at {}", self.at))?;
        let text = std::str::from_utf8(digits).map_err(|_| "a bad `\\u` escape".to_owned())?;
        let code = u32::from_str_radix(text, 16)
            .map_err(|_| format!("a bad `\\u` escape at {}", self.at))?;
        self.at += 4;
        Ok(code)
    }

    fn array(&mut self) -> Result<Json, String> {
        self.expect(b'[')?;
        let mut items = Vec::new();
        self.skip_space();
        if self.bytes.get(self.at) == Some(&b']') {
            self.at += 1;
            return Ok(Json::Array(items));
        }
        loop {
            items.push(self.value()?);
            self.skip_space();
            match self.bytes.get(self.at) {
                Some(b',') => self.at += 1,
                Some(b']') => {
                    self.at += 1;
                    return Ok(Json::Array(items));
                }
                _ => return Err(format!("expected `,` or `]` at {}", self.at)),
            }
        }
    }

    fn object(&mut self) -> Result<Json, String> {
        self.expect(b'{')?;
        let mut members = BTreeMap::new();
        self.skip_space();
        if self.bytes.get(self.at) == Some(&b'}') {
            self.at += 1;
            return Ok(Json::Object(members));
        }
        loop {
            self.skip_space();
            let key = self.string()?;
            self.expect(b':')?;
            let value = self.value()?;
            members.insert(key, value);
            self.skip_space();
            match self.bytes.get(self.at) {
                Some(b',') => self.at += 1,
                Some(b'}') => {
                    self.at += 1;
                    return Ok(Json::Object(members));
                }
                _ => return Err(format!("expected `,` or `}}` at {}", self.at)),
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{parse, Json};

    #[test]
    fn the_metadatas_shapes_parse() {
        let doc = parse(r#"{"packages":[{"name":"a","source":null,"targets":[{"kind":["lib"]}]}],"n":-1.5e3,"s":"q\"\\\u00e9\ud83d\ude00"}"#).unwrap();
        let packages = doc.get("packages").unwrap().elements();
        assert_eq!(packages[0].get("name").unwrap().as_str(), Some("a"));
        assert!(packages[0].get("source").unwrap().is_null());
        assert_eq!(
            packages[0].get("targets").unwrap().elements()[0]
                .get("kind")
                .unwrap()
                .elements()[0]
                .as_str(),
            Some("lib")
        );
        assert_eq!(doc.get("n"), Some(&Json::Number("-1.5e3".into())));
        assert_eq!(doc.get("s").unwrap().as_str(), Some("q\"\\é😀"));
        assert_eq!(parse(" [ ] ").unwrap(), Json::Array(vec![]));
    }

    #[test]
    fn what_is_not_json_is_refused() {
        for text in [
            "",
            "{",
            "[1,]",
            "{\"a\" 1}",
            "tru",
            "\"\\x\"",
            "1 2",
            "\"\\ud83d\"",
        ] {
            assert!(parse(text).is_err(), "{text:?}");
        }
    }
}
