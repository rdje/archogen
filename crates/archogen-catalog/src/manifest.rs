//! The manifest dialect (the record's §3): the part of TOML the workspace's manifests and cargo configuration use.
//!
//! > A line is blank; a comment; a table header, which is `[` or `[[`, then one or more bare keys joined by `.`,
//! > then `]` or `]]`, where a bare key is ASCII letters, digits, `-` and `_`; or `key = value`, where the key is a
//! > bare key or bare keys joined by `.`. A value is, on the same line, a basic string with no escape, an integer,
//! > `true` or `false`, an array of those, or an inline table `{ key = value, … }` of those.
//!
//! Anything else is outside the dialect, and a package whose manifest has such a line is refused. Inside it, the
//! parse is read by TOML's meaning: every value lands at its full key path, whether a header, a dotted key or an
//! inline table spelt it, so a dependency table cannot hide behind another spelling.

/// A value in the dialect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Value {
    /// A basic string with no escape.
    Str(String),
    /// A decimal integer.
    Int(i64),
    /// `true` or `false`.
    Bool(bool),
    /// An array of values.
    Array(Vec<Value>),
    /// An inline table: its keys, each a dotted path, with their values.
    Table(Vec<(Vec<String>, Value)>),
}

/// A manifest, read by TOML's meaning.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Manifest {
    /// Every table header, as its key path. An array of tables' instance ends in `[n]`.
    pub tables: Vec<Vec<String>>,
    /// Every value at its full key path, inline tables expanded to their leaves.
    pub values: Vec<(Vec<String>, Value)>,
}

/// Why a line is outside the dialect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Outside {
    /// The 1-based line.
    pub line: usize,
    /// What is outside it.
    pub why: &'static str,
}

impl Manifest {
    /// Every value whose path starts with `prefix`, with the rest of its path.
    pub fn under<'a>(
        &'a self,
        prefix: &'a [&str],
    ) -> impl Iterator<Item = (&'a [String], &'a Value)> + 'a {
        self.values
            .iter()
            .filter(move |(path, _)| {
                path.len() >= prefix.len() && path.iter().zip(prefix).all(|(a, b)| a == b)
            })
            .map(move |(path, value)| (&path[prefix.len()..], value))
    }

    /// Whether a table exists at `path`, by a header or by any value beneath it.
    #[must_use]
    pub fn has_table(&self, path: &[&str]) -> bool {
        let is = |p: &[String]| p.len() >= path.len() && p.iter().zip(path).all(|(a, b)| a == b);
        self.tables.iter().any(|t| is(t))
            || self
                .values
                .iter()
                .any(|(p, _)| p.len() > path.len() && is(p))
    }

    /// The value at exactly `path`.
    #[must_use]
    pub fn get(&self, path: &[&str]) -> Option<&Value> {
        self.values
            .iter()
            .find(|(p, _)| p.len() == path.len() && p.iter().zip(path).all(|(a, b)| a == b))
            .map(|(_, v)| v)
    }
}

/// Read a manifest in the dialect.
///
/// # Errors
///
/// The first line outside the dialect.
pub fn parse(text: &str) -> Result<Manifest, Outside> {
    let mut manifest = Manifest::default();
    let mut table: Vec<String> = Vec::new();
    let mut instances: Vec<(Vec<String>, usize)> = Vec::new();
    for (index, raw) in text.split('\n').enumerate() {
        let line = index + 1;
        let outside = |why| Outside { line, why };
        let trimmed = raw.trim_matches(|c| c == ' ' || c == '\t');
        if trimmed.is_empty() || trimmed.starts_with('#') {
            continue;
        }
        if let Some(inner) = trimmed
            .strip_prefix("[[")
            .and_then(|s| s.strip_suffix("]]"))
        {
            let path =
                keys(inner).ok_or_else(|| outside("a table header is bare keys joined by `.`"))?;
            let n = match instances.iter_mut().find(|(p, _)| *p == path) {
                Some((_, count)) => {
                    *count += 1;
                    *count
                }
                None => {
                    instances.push((path.clone(), 0));
                    0
                }
            };
            table = path;
            table.push(format!("[{n}]"));
            manifest.tables.push(table.clone());
            continue;
        }
        if let Some(inner) = trimmed.strip_prefix('[').and_then(|s| s.strip_suffix(']')) {
            table =
                keys(inner).ok_or_else(|| outside("a table header is bare keys joined by `.`"))?;
            manifest.tables.push(table.clone());
            continue;
        }
        let (key, rest) = trimmed
            .split_once('=')
            .ok_or_else(|| outside("a line is blank, a comment, a header or `key = value`"))?;
        let key = keys(key.trim_matches(|c| c == ' ' || c == '\t'))
            .ok_or_else(|| outside("a key is bare keys joined by `.`"))?;
        let mut cursor = Cursor {
            text: rest.as_bytes(),
            at: 0,
        };
        cursor.space();
        let value = cursor.value().map_err(outside)?;
        cursor.space();
        if cursor.at != cursor.text.len() {
            return Err(outside(
                "a value ends the line: no trailing text or comment",
            ));
        }
        let mut path = table.clone();
        path.extend(key);
        flatten(&mut manifest.values, path, value);
    }
    Ok(manifest)
}

/// Bare keys joined by `.`, each ASCII letters, digits, `-` and `_`.
fn keys(text: &str) -> Option<Vec<String>> {
    let parts: Vec<String> = text.split('.').map(str::to_owned).collect();
    parts
        .iter()
        .all(|k| {
            !k.is_empty()
                && k.bytes()
                    .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
        })
        .then_some(parts)
}

/// Record `value` at `path`, and each leaf of an inline table at its own path too.
fn flatten(out: &mut Vec<(Vec<String>, Value)>, path: Vec<String>, value: Value) {
    if let Value::Table(entries) = &value {
        for (key, inner) in entries {
            let mut p = path.clone();
            p.extend(key.iter().cloned());
            flatten(out, p, inner.clone());
        }
    }
    out.push((path, value));
}

struct Cursor<'a> {
    text: &'a [u8],
    at: usize,
}

impl Cursor<'_> {
    fn space(&mut self) {
        while self
            .text
            .get(self.at)
            .is_some_and(|&b| b == b' ' || b == b'\t')
        {
            self.at += 1;
        }
    }

    fn eat(&mut self, byte: u8) -> bool {
        if self.text.get(self.at) == Some(&byte) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    fn value(&mut self) -> Result<Value, &'static str> {
        match self.text.get(self.at) {
            Some(b'"') => self.string(),
            Some(b'[') => self.array(),
            Some(b'{') => self.table(),
            Some(b't' | b'f') => self.boolean(),
            Some(b'-' | b'0'..=b'9') => self.integer(),
            _ => {
                Err("a value is a basic string, an integer, a boolean, an array or an inline table")
            }
        }
    }

    fn string(&mut self) -> Result<Value, &'static str> {
        self.at += 1;
        let start = self.at;
        while let Some(&b) = self.text.get(self.at) {
            match b {
                b'"' => {
                    let s = core::str::from_utf8(&self.text[start..self.at])
                        .map_err(|_| "a string is UTF-8")?;
                    self.at += 1;
                    return Ok(Value::Str(s.to_owned()));
                }
                b'\\' => return Err("a string has no escape"),
                0x00..=0x1f | 0x7f => return Err("a string holds no control character"),
                _ => self.at += 1,
            }
        }
        Err("a string closes on its line")
    }

    fn integer(&mut self) -> Result<Value, &'static str> {
        let start = self.at;
        self.eat(b'-');
        let digits = self.at;
        while self.text.get(self.at).is_some_and(u8::is_ascii_digit) {
            self.at += 1;
        }
        if self.at == digits
            || self
                .text
                .get(self.at)
                .is_some_and(|&b| matches!(b, b'.' | b'_' | b'-' | b':') || b.is_ascii_alphabetic())
        {
            return Err("an integer is decimal digits: no float, date or underscore");
        }
        let text = core::str::from_utf8(&self.text[start..self.at]).map_err(|_| "an integer")?;
        text.parse()
            .map(Value::Int)
            .map_err(|_| "an integer fits 64 bits")
    }

    fn boolean(&mut self) -> Result<Value, &'static str> {
        for (word, value) in [(&b"true"[..], true), (&b"false"[..], false)] {
            if self.text[self.at..].starts_with(word) {
                self.at += word.len();
                return Ok(Value::Bool(value));
            }
        }
        Err("a bare word is `true` or `false`")
    }

    fn array(&mut self) -> Result<Value, &'static str> {
        self.at += 1;
        let mut items = Vec::new();
        loop {
            self.space();
            if self.eat(b']') {
                return Ok(Value::Array(items));
            }
            if self.at == self.text.len() {
                return Err("an array closes on its line");
            }
            items.push(self.value()?);
            self.space();
            if !self.eat(b',') {
                self.space();
                return if self.eat(b']') {
                    Ok(Value::Array(items))
                } else {
                    Err("an array closes on its line")
                };
            }
        }
    }

    fn table(&mut self) -> Result<Value, &'static str> {
        self.at += 1;
        let mut entries = Vec::new();
        self.space();
        if self.eat(b'}') {
            return Ok(Value::Table(entries));
        }
        loop {
            self.space();
            let start = self.at;
            while self
                .text
                .get(self.at)
                .is_some_and(|&b| b.is_ascii_alphanumeric() || matches!(b, b'-' | b'_' | b'.'))
            {
                self.at += 1;
            }
            let key = core::str::from_utf8(&self.text[start..self.at])
                .ok()
                .and_then(keys)
                .ok_or("an inline table's key is bare keys")?;
            self.space();
            if !self.eat(b'=') {
                return Err("an inline table's entry is `key = value`");
            }
            self.space();
            entries.push((key, self.value()?));
            self.space();
            if self.eat(b'}') {
                return Ok(Value::Table(entries));
            }
            if !self.eat(b',') {
                return Err("an inline table closes on its line");
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_dependency_reads_the_same_however_it_is_spelt() {
        let spellings = [
            "[dependencies]\nfoo = { path = \"../foo\" }\n",
            "[dependencies]\nfoo.path = \"../foo\"\n",
            "[dependencies.foo]\npath = \"../foo\"\n",
            "dependencies = { foo = { path = \"../foo\" } }\n",
            "dependencies.foo.path = \"../foo\"\n",
        ];
        for text in spellings {
            let m = parse(text).unwrap_or_else(|e| panic!("{text}: {e:?}"));
            assert_eq!(
                m.get(&["dependencies", "foo", "path"]),
                Some(&Value::Str("../foo".into())),
                "{text}"
            );
        }
    }

    #[test]
    fn lines_outside_the_dialect_are_named() {
        for (text, why) in [
            ("a = \"x\\ny\"\n", "no escape"),
            ("a = 'x'\n", "a value is"),
            ("a = 1.5\n", "no float"),
            ("a = 1979-05-27\n", "no float, date"),
            ("\"quoted\" = 1\n", "a key is bare keys"),
            ("a = [\n", "an array closes"),
            ("a = 1 # note\n", "no trailing text"),
            ("[a b]\n", "a table header"),
            ("a = \"\"\"x\"\"\"\n", "no trailing text"),
        ] {
            let e = parse(text).expect_err(text);
            assert!(e.why.contains(why), "{text}: {e:?}");
            assert_eq!(e.line, 1);
        }
    }

    #[test]
    fn arrays_of_tables_are_indexed() {
        let m =
            parse("[[bin]]\nname = \"a\"\n[[bin]]\nname = \"b\"\npath = \"src/b.rs\"\n").unwrap();
        assert_eq!(
            m.get(&["bin", "[1]", "path"]),
            Some(&Value::Str("src/b.rs".into()))
        );
        assert!(m.has_table(&["bin"]));
    }

    #[test]
    fn the_workspace_manifests_are_inside_the_dialect() {
        for (name, text) in [
            ("Cargo.toml", include_str!("../../../Cargo.toml")),
            (
                ".cargo/config.toml",
                include_str!("../../../.cargo/config.toml"),
            ),
            (
                "crates/rt-core/Cargo.toml",
                include_str!("../../rt-core/Cargo.toml"),
            ),
            (
                "xtask/Cargo.toml",
                include_str!("../../../xtask/Cargo.toml"),
            ),
            (
                "crates/archogen-catalog/Cargo.toml",
                include_str!("../Cargo.toml"),
            ),
        ] {
            parse(text).unwrap_or_else(|e| panic!("{name}: {e:?}"));
        }
        let root = parse(include_str!("../../../Cargo.toml")).unwrap();
        assert!(root.has_table(&["workspace"]) && !root.has_table(&["package"]));
        assert!(matches!(
            root.get(&["workspace", "members"]),
            Some(Value::Array(_))
        ));
    }
}
