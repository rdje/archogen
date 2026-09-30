//! The wasm binding on the host (leaf `API.5.2`): what `docs/decisions/decision_wasm-binding.md` §4 and §6 say a
//! request and a response are, checked without a wasm toolchain.
//!
//! The response is read back by [`reader`], a strict RFC 8259 parser written here for the purpose. It shares no
//! code with the encoder: one writes JSON and the other reads it, so an escaping mistake shows up as a value that
//! does not survive the trip. Each response is then compared **field by field** with the engine API's own
//! `Response` for the same request, and its shape is frozen in `tests/goldens/archogen-wasm-response-1.golden`.

use std::collections::BTreeSet;
use std::fs;
use std::path::{Path, PathBuf};

use archogen_api::{check_with, Limits, MemoryModules, Request, Response, SourceMap, Status};
use archogen_wasm::request::{frame, parse, Framed};
use archogen_wasm::{
    answer, archogen_check, archogen_input, archogen_output, INPUT_CAP, REQUEST_FORMAT,
    RESPONSE_FORMAT,
};

/// A strict JSON reader: RFC 8259's grammar, integers only (the format writes no other number), object keys in
/// their order, a duplicated key refused.
mod reader {
    #[derive(Debug, Clone, PartialEq, Eq)]
    pub enum Value {
        Null,
        Bool(bool),
        Integer(i64),
        String(String),
        Array(Vec<Value>),
        Object(Vec<(String, Value)>),
    }

    impl Value {
        pub fn get(&self, key: &str) -> &Value {
            match self {
                Value::Object(entries) => entries
                    .iter()
                    .find(|(name, _)| name == key)
                    .map(|(_, value)| value)
                    .unwrap_or_else(|| panic!("no key `{key}` in {self:?}")),
                other => panic!("`{key}` looked up in a non-object {other:?}"),
            }
        }

        pub fn keys(&self) -> Vec<&str> {
            match self {
                Value::Object(entries) => entries.iter().map(|(name, _)| name.as_str()).collect(),
                other => panic!("keys of a non-object {other:?}"),
            }
        }

        pub fn str(&self) -> &str {
            match self {
                Value::String(text) => text,
                other => panic!("expected a string, got {other:?}"),
            }
        }

        pub fn int(&self) -> i64 {
            match self {
                Value::Integer(value) => *value,
                other => panic!("expected an integer, got {other:?}"),
            }
        }

        pub fn items(&self) -> &[Value] {
            match self {
                Value::Array(items) => items,
                other => panic!("expected an array, got {other:?}"),
            }
        }

        pub fn optional_str(&self) -> Option<&str> {
            match self {
                Value::Null => None,
                other => Some(other.str()),
            }
        }
    }

    pub struct Parsed {
        pub value: Value,
        /// Whether any whitespace stood between two tokens.
        pub had_whitespace: bool,
    }

    struct Cursor<'a> {
        text: &'a [u8],
        at: usize,
        had_whitespace: bool,
    }

    pub fn parse(text: &str) -> Result<Parsed, String> {
        let mut cursor = Cursor {
            text: text.as_bytes(),
            at: 0,
            had_whitespace: false,
        };
        let value = cursor.value()?;
        cursor.whitespace();
        if cursor.at != cursor.text.len() {
            return Err(format!("text after the value at byte {}", cursor.at));
        }
        Ok(Parsed {
            value,
            had_whitespace: cursor.had_whitespace,
        })
    }

    impl Cursor<'_> {
        fn whitespace(&mut self) {
            while let Some(b' ' | b'\t' | b'\n' | b'\r') = self.text.get(self.at) {
                self.at += 1;
                self.had_whitespace = true;
            }
        }

        fn expect(&mut self, byte: u8) -> Result<(), String> {
            self.whitespace();
            if self.text.get(self.at) == Some(&byte) {
                self.at += 1;
                Ok(())
            } else {
                Err(format!("expected `{}` at byte {}", byte as char, self.at))
            }
        }

        fn literal(&mut self, word: &str, value: Value) -> Result<Value, String> {
            if self.text[self.at..].starts_with(word.as_bytes()) {
                self.at += word.len();
                Ok(value)
            } else {
                Err(format!("unknown literal at byte {}", self.at))
            }
        }

        fn value(&mut self) -> Result<Value, String> {
            self.whitespace();
            match self.text.get(self.at) {
                Some(b'n') => self.literal("null", Value::Null),
                Some(b't') => self.literal("true", Value::Bool(true)),
                Some(b'f') => self.literal("false", Value::Bool(false)),
                Some(b'"') => self.string().map(Value::String),
                Some(b'[') => self.array(),
                Some(b'{') => self.object(),
                Some(b'-' | b'0'..=b'9') => self.integer(),
                other => Err(format!("unexpected {other:?} at byte {}", self.at)),
            }
        }

        fn integer(&mut self) -> Result<Value, String> {
            let start = self.at;
            if self.text[self.at] == b'-' {
                self.at += 1;
            }
            match self.text.get(self.at) {
                Some(b'0') => self.at += 1,
                Some(b'1'..=b'9') => {
                    while let Some(b'0'..=b'9') = self.text.get(self.at) {
                        self.at += 1;
                    }
                }
                _ => return Err(format!("a malformed number at byte {start}")),
            }
            if let Some(b'.' | b'e' | b'E') = self.text.get(self.at) {
                return Err(format!(
                    "a fraction or exponent at byte {start}; the format writes integers"
                ));
            }
            let digits = std::str::from_utf8(&self.text[start..self.at]).expect("ASCII digits");
            digits
                .parse()
                .map(Value::Integer)
                .map_err(|error| format!("{digits}: {error}"))
        }

        fn hex4(&mut self) -> Result<u32, String> {
            let digits = self
                .text
                .get(self.at..self.at + 4)
                .ok_or("a short \\u escape")?;
            let digits = std::str::from_utf8(digits).map_err(|_| "a non-ASCII \\u escape")?;
            self.at += 4;
            u32::from_str_radix(digits, 16).map_err(|_| format!("a bad \\u escape `{digits}`"))
        }

        fn string(&mut self) -> Result<String, String> {
            self.at += 1; // the opening quote
            let mut out = String::new();
            loop {
                let Some(&byte) = self.text.get(self.at) else {
                    return Err("an unterminated string".into());
                };
                match byte {
                    b'"' => {
                        self.at += 1;
                        return Ok(out);
                    }
                    b'\\' => {
                        self.at += 1;
                        let escape = *self.text.get(self.at).ok_or("an unterminated escape")?;
                        self.at += 1;
                        match escape {
                            b'"' => out.push('"'),
                            b'\\' => out.push('\\'),
                            b'/' => out.push('/'),
                            b'b' => out.push('\u{08}'),
                            b'f' => out.push('\u{0c}'),
                            b'n' => out.push('\n'),
                            b'r' => out.push('\r'),
                            b't' => out.push('\t'),
                            b'u' => {
                                let high = self.hex4()?;
                                let code = if (0xd800..0xdc00).contains(&high) {
                                    if !self.text[self.at..].starts_with(b"\\u") {
                                        return Err("a lone high surrogate".into());
                                    }
                                    self.at += 2;
                                    let low = self.hex4()?;
                                    0x10000 + ((high - 0xd800) << 10) + (low - 0xdc00)
                                } else {
                                    high
                                };
                                out.push(char::from_u32(code).ok_or("an invalid code point")?);
                            }
                            other => {
                                return Err(format!("an unknown escape `\\{}`", other as char))
                            }
                        }
                    }
                    0x00..=0x1f => {
                        return Err(format!(
                            "an unescaped control character {byte:#04x} at byte {}",
                            self.at
                        ))
                    }
                    _ => {
                        // Copy one UTF-8 character.
                        let rest = std::str::from_utf8(&self.text[self.at..])
                            .map_err(|_| "invalid UTF-8".to_string())?;
                        let c = rest.chars().next().expect("not at the end");
                        out.push(c);
                        self.at += c.len_utf8();
                    }
                }
            }
        }

        fn array(&mut self) -> Result<Value, String> {
            self.at += 1;
            let mut items = Vec::new();
            self.whitespace();
            if self.text.get(self.at) == Some(&b']') {
                self.at += 1;
                return Ok(Value::Array(items));
            }
            loop {
                items.push(self.value()?);
                self.whitespace();
                match self.text.get(self.at) {
                    Some(b',') => self.at += 1,
                    Some(b']') => {
                        self.at += 1;
                        return Ok(Value::Array(items));
                    }
                    _ => return Err(format!("expected `,` or `]` at byte {}", self.at)),
                }
            }
        }

        fn object(&mut self) -> Result<Value, String> {
            self.at += 1;
            let mut entries: Vec<(String, Value)> = Vec::new();
            self.whitespace();
            if self.text.get(self.at) == Some(&b'}') {
                self.at += 1;
                return Ok(Value::Object(entries));
            }
            loop {
                self.whitespace();
                if self.text.get(self.at) != Some(&b'"') {
                    return Err(format!("expected a key at byte {}", self.at));
                }
                let key = self.string()?;
                if entries.iter().any(|(seen, _)| *seen == key) {
                    return Err(format!("the key `{key}` twice"));
                }
                self.expect(b':')?;
                let value = self.value()?;
                entries.push((key, value));
                self.whitespace();
                match self.text.get(self.at) {
                    Some(b',') => self.at += 1,
                    Some(b'}') => {
                        self.at += 1;
                        return Ok(Value::Object(entries));
                    }
                    _ => return Err(format!("expected `,` or `}}` at byte {}", self.at)),
                }
            }
        }
    }
}

use reader::Value;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Read `json` strictly, and require what the record's §6 fixes about its bytes: no whitespace between tokens.
fn read(json: &str) -> Value {
    let parsed = reader::parse(json).unwrap_or_else(|error| panic!("{error}\n{json}"));
    assert!(!parsed.had_whitespace, "whitespace between tokens:\n{json}");
    parsed.value
}

/// A request to the binding and the same request to the API, side by side.
struct Case {
    name: &'static str,
    framed: Framed,
}

impl Case {
    fn new(
        name: &'static str,
        text: &str,
        profile: Option<&str>,
        modules: &[(&str, &str)],
    ) -> Self {
        Self {
            name,
            framed: Framed {
                name: format!("{name}.eadl"),
                text: text.to_string(),
                profile: profile.map(str::to_string),
                modules: modules
                    .iter()
                    .map(|(module, text)| ((*module).to_string(), (*text).to_string()))
                    .collect(),
            },
        }
    }

    /// The engine API's own answer, asked as the binding asks it.
    fn api(&self) -> Response {
        let modules = self
            .framed
            .modules
            .iter()
            .fold(MemoryModules::new(), |modules, (name, text)| {
                modules.with(name, text)
            });
        check_with(
            &Request {
                name: &self.framed.name,
                text: &self.framed.text,
                profile: self.framed.profile.as_deref(),
                modules: &modules,
            },
            Limits::DEFAULT,
        )
    }
}

fn module_case() -> Case {
    let dir = repo_root().join("docs/semantics/modules");
    let text = |module: &str| {
        fs::read_to_string(dir.join(format!("{module}.eadl")))
            .unwrap_or_else(|error| panic!("{module}: {error}"))
    };
    let root = text("app.sibling");
    let bus = text("hw.bus");
    let mut case = Case::new("app.sibling", &root, None, &[]);
    case.framed.modules = vec![("hw.bus".to_string(), bus)];
    case
}

/// One request of each kind the response distinguishes: accepted, judged and refused, not judged, a diagnostic
/// with a secondary label, and a module tree.
fn cases() -> Vec<Case> {
    vec![
        Case::new(
            "uart",
            "(defblock console.uart (offers observable-output))\n",
            None,
            &[],
        ),
        Case::new(
            "timer",
            "(defblock timer.counter (offers (tick-rate 10 parsec)))\n",
            None,
            &[],
        ),
        Case::new(
            "elsewhere",
            "(defblock console.uart (offers observable-output))\n",
            Some("rt-static-up-v9"),
            &[],
        ),
        Case::new(
            "twice",
            "(defblock timer.counter (offers observable-output))\n(defblock timer.counter (offers observable-output))\n",
            None,
            &[],
        ),
        module_case(),
    ]
}

fn assert_label(value: &Value, label: &eadl_front::Label, sources: &SourceMap) {
    let source = sources
        .get(label.span.source)
        .expect("a response holds its sources");
    let position = source.position(label.span.start);
    assert_eq!(value.keys(), ["source", "line", "column", "message"]);
    assert_eq!(value.get("source").str(), source.name);
    assert_eq!(value.get("line").int(), i64::from(position.line));
    assert_eq!(value.get("column").int(), i64::from(position.column));
    assert_eq!(value.get("message").str(), label.message);
}

/// Every field of the encoded response against the API's response, in the record's order.
fn assert_matches(value: &Value, response: &Response) {
    assert_eq!(
        value.keys(),
        [
            "format",
            "api",
            "engine",
            "status",
            "exit",
            "notes",
            "hint",
            "diagnostics",
            "rendered",
            "judged"
        ]
    );
    assert_eq!(value.get("format").str(), RESPONSE_FORMAT);
    assert_eq!(value.get("api").str(), response.version.to_string());
    assert_eq!(value.get("engine").str(), response.engine);
    assert_eq!(value.get("status").str(), response.status.slug());
    assert_eq!(value.get("exit").int(), i64::from(response.status.code()));
    let notes: Vec<&str> = value.get("notes").items().iter().map(Value::str).collect();
    assert_eq!(notes, response.notes);
    assert_eq!(value.get("hint").optional_str(), response.hint.as_deref());
    let diagnostics = value.get("diagnostics").items();
    assert_eq!(diagnostics.len(), response.diagnostics.len());
    for (encoded, diagnostic) in diagnostics.iter().zip(&response.diagnostics) {
        assert_eq!(
            encoded.keys(),
            [
                "code",
                "severity",
                "message",
                "primary",
                "secondary",
                "repair"
            ]
        );
        assert_eq!(encoded.get("code").str(), diagnostic.code);
        let severity = match diagnostic.severity {
            eadl_front::Severity::Error => "error",
            eadl_front::Severity::Warning => "warning",
        };
        assert_eq!(encoded.get("severity").str(), severity);
        assert_eq!(encoded.get("message").str(), diagnostic.message);
        assert_label(
            encoded.get("primary"),
            &diagnostic.primary,
            &response.sources,
        );
        let secondary = encoded.get("secondary").items();
        assert_eq!(secondary.len(), diagnostic.secondary.len());
        for (encoded, label) in secondary.iter().zip(&diagnostic.secondary) {
            assert_label(encoded, label, &response.sources);
        }
        assert_eq!(encoded.get("repair").str(), diagnostic.repair);
    }
    assert_eq!(value.get("rendered").str(), response.render_diagnostics());
    match (&response.judged, value.get("judged")) {
        (None, encoded) => assert_eq!(*encoded, Value::Null),
        (Some(judged), encoded) => {
            assert_eq!(
                encoded.keys(),
                [
                    "verdict",
                    "language",
                    "profile",
                    "declarations",
                    "instances",
                    "closure"
                ]
            );
            assert_eq!(
                encoded.get("verdict").str(),
                Status::from_verdict(judged.verdict).slug()
            );
            assert_eq!(encoded.get("language").str(), judged.language);
            assert_eq!(encoded.get("profile").str(), judged.profile);
            assert_eq!(
                encoded.get("declarations").int(),
                i64::try_from(judged.declarations.len()).expect("small")
            );
            match &judged.instances {
                None => assert_eq!(*encoded.get("instances"), Value::Null),
                Some(instances) => {
                    let encoded: Vec<&str> = encoded
                        .get("instances")
                        .items()
                        .iter()
                        .map(Value::str)
                        .collect();
                    assert_eq!(encoded, *instances);
                }
            }
            let closure = encoded.get("closure");
            assert_eq!(closure.keys(), ["inside", "outside"]);
            let inside: Vec<(&str, Option<&str>)> = closure
                .get("inside")
                .items()
                .iter()
                .map(|pair| match pair.items() {
                    [fact, needed_by] => (fact.str(), needed_by.optional_str()),
                    other => panic!("an inside entry is a pair, not {other:?}"),
                })
                .collect();
            let expected: Vec<(&str, Option<&str>)> = judged
                .closure
                .inside
                .iter()
                .map(|(fact, needed_by)| (fact.as_str(), needed_by.as_deref()))
                .collect();
            assert_eq!(inside, expected);
            let outside: Vec<&str> = closure
                .get("outside")
                .items()
                .iter()
                .map(Value::str)
                .collect();
            assert_eq!(outside, judged.closure.outside);
        }
    }
}

#[test]
fn each_response_equals_the_apis_field_by_field() {
    let mut statuses = BTreeSet::new();
    for case in cases() {
        let response = case.api();
        statuses.insert(response.status.slug());
        let encoded = answer(&frame(&case.framed));
        assert_matches(&read(&encoded), &response);
        assert_eq!(
            encoded,
            answer(&frame(&case.framed)),
            "{}: the same request encodes to the same bytes",
            case.name
        );
    }
    // The cases reach every shape the response has: accepted, refused with diagnostics, and not judged.
    assert!(statuses.contains("ok"), "{statuses:?}");
    assert!(statuses.len() >= 3, "{statuses:?}");
}

#[test]
fn a_module_tree_is_judged_with_its_modules_and_names_its_instances() {
    let case = module_case();
    let value = read(&answer(&frame(&case.framed)));
    assert_eq!(value.get("status").str(), "ok", "{value:?}");
    let instances = value.get("judged").get("instances").items();
    assert!(!instances.is_empty(), "{value:?}");
}

#[test]
fn every_control_character_quote_and_backslash_survives_the_encoding() {
    let mut text: String = (0u8..0x20).map(char::from).collect();
    text.push_str("\"\\/\u{7f}é😀 plain");
    let response = Response {
        version: archogen_api::VERSION,
        engine: archogen_api::ENGINE,
        status: Status::Usage,
        notes: vec![text.clone()],
        hint: Some(text.clone()),
        diagnostics: Vec::new(),
        sources: SourceMap::new(),
        judged: None,
    };
    let json = archogen_wasm::json::encode(&response);
    let value = read(&json);
    assert_eq!(value.get("notes").items()[0].str(), text);
    assert_eq!(value.get("hint").str(), text);
    // Exactly the record's escapes: the five short ones, `\u00XX` in lowercase for the rest below 0x20, and
    // nothing else — not `/`, not DEL, not a character outside ASCII.
    let written = json
        .split("\"notes\":[\"")
        .nth(1)
        .and_then(|rest| rest.split("\"]").next())
        .expect("the note is written");
    let expected = "\\u0000\\u0001\\u0002\\u0003\\u0004\\u0005\\u0006\\u0007\\b\\t\\n\\u000b\\f\\r\\u000e\\u000f\
                    \\u0010\\u0011\\u0012\\u0013\\u0014\\u0015\\u0016\\u0017\\u0018\\u0019\\u001a\\u001b\\u001c\\u001d\
                    \\u001e\\u001f\\\"\\\\/\u{7f}é😀 plain";
    assert_eq!(written, expected);
}

/// The framing a test writes by hand, field by field.
fn field(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&u32::try_from(bytes.len()).expect("short").to_le_bytes());
    out.extend_from_slice(bytes);
}

fn head(format: &str) -> Vec<u8> {
    let mut out = Vec::new();
    field(&mut out, format.as_bytes());
    field(&mut out, b"d.eadl");
    field(
        &mut out,
        b"(defblock console.uart (offers observable-output))\n",
    );
    field(&mut out, b"");
    out
}

/// What a framing refusal says: the status, the note, and that nothing was judged.
fn refusal(bytes: &[u8]) -> (String, String) {
    let value = read(&answer(bytes));
    assert_eq!(value.get("status").str(), "usage", "{value:?}");
    assert_eq!(value.get("exit").int(), i64::from(Status::Usage.code()));
    assert_eq!(*value.get("judged"), Value::Null);
    assert!(value.get("diagnostics").items().is_empty());
    let note = value.get("notes").items()[0].str().to_string();
    let error = parse(bytes).expect_err("a refusal");
    (note, error.field)
}

#[test]
fn every_framing_refusal_is_answered_with_usage_and_names_its_field_and_offset() {
    // Nothing at all.
    let (note, field_name) = refusal(&[]);
    assert_eq!(field_name, "format");
    assert!(
        note.contains("format field, at byte 0, needs a 4-byte length, and 0 byte(s) remain"),
        "{note}"
    );

    // A length that runs past the end.
    let mut bytes = Vec::new();
    bytes.extend_from_slice(&100u32.to_le_bytes());
    bytes.extend_from_slice(b"short");
    let (note, field_name) = refusal(&bytes);
    assert_eq!(field_name, "format");
    assert!(
        note.contains("gives a length of 100 byte(s), and 5 remain"),
        "{note}"
    );

    // Another format.
    let mut bytes = head("archogen-wasm-request/0");
    bytes.extend_from_slice(&0u32.to_le_bytes());
    let (note, field_name) = refusal(&bytes);
    assert_eq!(field_name, "format");
    assert!(note.contains("is `archogen-wasm-request/0`"), "{note}");

    // Bytes that are not UTF-8, in the text.
    let mut bytes = Vec::new();
    field(&mut bytes, REQUEST_FORMAT.as_bytes());
    field(&mut bytes, b"d.eadl");
    let text_at = bytes.len();
    field(&mut bytes, &[0x28, 0xff, 0x29]);
    field(&mut bytes, b"");
    bytes.extend_from_slice(&0u32.to_le_bytes());
    let (note, field_name) = refusal(&bytes);
    assert_eq!(field_name, "text");
    assert!(
        note.contains(&format!("text field, at byte {text_at}, is not UTF-8")),
        "{note}"
    );

    // No module count.
    let bytes = head(REQUEST_FORMAT);
    let (_, field_name) = refusal(&bytes);
    assert_eq!(field_name, "module count");

    // A module count larger than the request holds: the first missing field fails, and nothing is allocated for
    // the count.
    let mut bytes = head(REQUEST_FORMAT);
    bytes.extend_from_slice(&u32::MAX.to_le_bytes());
    let (_, field_name) = refusal(&bytes);
    assert_eq!(field_name, "module 1 name");

    // A module named twice.
    let mut bytes = head(REQUEST_FORMAT);
    bytes.extend_from_slice(&2u32.to_le_bytes());
    field(&mut bytes, b"hw.bus");
    field(&mut bytes, b"(defmodule hw.bus (version 1 0))");
    let second_at = bytes.len();
    field(&mut bytes, b"hw.bus");
    field(&mut bytes, b"(defmodule hw.bus (version 1 0))");
    let (note, field_name) = refusal(&bytes);
    assert_eq!(field_name, "module 2 name");
    assert!(
        note.contains(&format!(
            "at byte {second_at}, names `hw.bus`, which an earlier module already names"
        )),
        "{note}"
    );

    // Bytes after the last module.
    let mut bytes = head(REQUEST_FORMAT);
    bytes.extend_from_slice(&0u32.to_le_bytes());
    let end_at = bytes.len();
    bytes.push(0);
    let (note, field_name) = refusal(&bytes);
    assert_eq!(field_name, "end");
    assert!(
        note.contains(&format!(
            "end field, at byte {end_at}, is followed by 1 byte(s)"
        )),
        "{note}"
    );
}

#[test]
fn a_request_reads_back_as_it_was_framed_and_an_empty_profile_is_the_default() {
    let mut case = module_case();
    assert_eq!(parse(&frame(&case.framed)), Ok(case.framed.clone()));
    case.framed.profile = Some("rt-static-up-v1".into());
    assert_eq!(parse(&frame(&case.framed)), Ok(case.framed.clone()));
    case.framed.profile = None;
    let bytes = frame(&case.framed);
    assert_eq!(parse(&bytes).expect("framed").profile, None);
}

#[test]
fn the_exports_carry_a_request_in_and_the_response_out_as_the_loader_does() {
    let request = frame(&cases()[1].framed);
    let at = archogen_input(request.len());
    assert!(!at.is_null());
    // SAFETY: `archogen_input` returned a buffer of exactly `request.len()` bytes that the module owns, and no Rust
    // borrow of it is live until the next export is called: the page's write, done the page's way.
    unsafe { core::ptr::copy_nonoverlapping(request.as_ptr(), at, request.len()) };
    let length = archogen_check();
    // SAFETY: `archogen_output` is the address of the response `archogen_check` just kept, `length` bytes long,
    // and valid until the next export is called.
    let bytes = unsafe { core::slice::from_raw_parts(archogen_output(), length) }.to_vec();
    assert_eq!(String::from_utf8(bytes).expect("UTF-8"), answer(&request));
}

#[test]
fn a_request_above_the_cap_is_given_no_buffer_and_the_next_check_answers_usage() {
    assert!(archogen_input(INPUT_CAP + 1).is_null());
    let length = archogen_check();
    // SAFETY: as above — the response `archogen_check` just kept, `length` bytes long.
    let bytes = unsafe { core::slice::from_raw_parts(archogen_output(), length) }.to_vec();
    let value = read(&String::from_utf8(bytes).expect("UTF-8"));
    assert_eq!(value.get("status").str(), "usage");
    assert!(
        !archogen_input(INPUT_CAP).is_null(),
        "the cap itself is allowed"
    );
    let _ = archogen_check();
}

/// Every key path with the kinds of value found at it, over the cases: the response's shape.
fn shape(value: &Value, path: &str, out: &mut BTreeSet<String>) {
    let kind = match value {
        Value::Null => "null",
        Value::Bool(_) => "bool",
        Value::Integer(_) => "integer",
        Value::String(_) => "string",
        Value::Array(_) => "array",
        Value::Object(_) => "object",
    };
    if !path.is_empty() {
        out.insert(format!("{path}: {kind}"));
    }
    match value {
        Value::Array(items) => {
            for item in items {
                shape(item, &format!("{path}[]"), out);
            }
        }
        Value::Object(entries) => {
            for (key, item) in entries {
                let path = if path.is_empty() {
                    key.clone()
                } else {
                    format!("{path}.{key}")
                };
                shape(item, &path, out);
            }
        }
        _ => {}
    }
}

#[test]
fn the_response_shape_is_frozen_under_its_format_identifier() {
    let mut paths = BTreeSet::new();
    for case in cases() {
        shape(&read(&answer(&frame(&case.framed))), "", &mut paths);
    }
    shape(&read(&answer(&[])), "", &mut paths);
    let actual = format!(
        "{RESPONSE_FORMAT}\n{}\n",
        paths.into_iter().collect::<Vec<_>>().join("\n")
    );
    let path = Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("tests/goldens")
        .join(format!("{}.golden", RESPONSE_FORMAT.replace('/', "-")));
    match fs::read_to_string(&path) {
        Ok(frozen) => assert!(
            frozen == actual,
            "the shape of `{RESPONSE_FORMAT}` has changed. A golden is never rewritten: move RESPONSE_FORMAT (and \
             the register, docs/book/src/versions.md), then bless the new identifier.\n--- frozen\n{frozen}\n\
             --- now\n{actual}"
        ),
        Err(_) if std::env::var_os("ARCHOGEN_BLESS_FORMATS").is_some() => {
            fs::create_dir_all(path.parent().expect("a directory")).expect("create the goldens");
            fs::write(&path, &actual).expect("write the golden");
        }
        Err(error) => panic!(
            "no golden for `{RESPONSE_FORMAT}` at {} ({error}); bless it deliberately with \
             ARCHOGEN_BLESS_FORMATS=1 cargo test -p archogen-wasm --test binding",
            path.display()
        ),
    }
}
