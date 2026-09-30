//! The response's encoding (`docs/decisions/decision_wasm-binding.md` §6): one JSON object, keys in a fixed
//! order, no whitespace between tokens, and one way to write each string, so two builds of the same answer write
//! the same bytes.

use archogen_api::{Response, SourceMap, Status};
use eadl_front::{Diagnostic, Label, Severity};

/// A JSON text under construction.
struct Json(String);

impl Json {
    fn raw(&mut self, text: &str) {
        self.0.push_str(text);
    }

    /// A string: `"` and `\` escaped, every character below `0x20` escaped, and nothing else.
    fn string(&mut self, text: &str) {
        self.0.push('"');
        for c in text.chars() {
            match c {
                '"' => self.0.push_str("\\\""),
                '\\' => self.0.push_str("\\\\"),
                '\u{08}' => self.0.push_str("\\b"),
                '\t' => self.0.push_str("\\t"),
                '\n' => self.0.push_str("\\n"),
                '\u{0c}' => self.0.push_str("\\f"),
                '\r' => self.0.push_str("\\r"),
                c if u32::from(c) < 0x20 => {
                    const DIGITS: &[u8; 16] = b"0123456789abcdef";
                    let byte = u32::from(c);
                    self.0.push_str("\\u00");
                    self.0.push(char::from(DIGITS[(byte >> 4) as usize]));
                    self.0.push(char::from(DIGITS[(byte & 0x0f) as usize]));
                }
                c => self.0.push(c),
            }
        }
        self.0.push('"');
    }

    fn optional_string(&mut self, text: Option<&str>) {
        match text {
            Some(text) => self.string(text),
            None => self.raw("null"),
        }
    }

    fn integer(&mut self, value: impl Into<i64>) {
        self.0.push_str(&value.into().to_string());
    }

    /// `"key":` — the comma before it is the caller's, since only the caller knows whether a key came first.
    fn key(&mut self, key: &str) {
        self.string(key);
        self.0.push(':');
    }

    fn strings<'a>(&mut self, items: impl IntoIterator<Item = &'a str>) {
        self.0.push('[');
        for (index, item) in items.into_iter().enumerate() {
            if index > 0 {
                self.0.push(',');
            }
            self.string(item);
        }
        self.0.push(']');
    }
}

/// A label: its source's name and the 1-based line and character column of its span's start.
///
/// A response holds every source its spans point into, so the lookup always succeeds. Were it ever to fail, the
/// label is written with an empty source and line and column `0`, which no real position has, rather than
/// dropped: an encoding must be total.
fn label(json: &mut Json, label: &Label, sources: &SourceMap) {
    let (name, line, column) = sources.get(label.span.source).map_or(("", 0, 0), |source| {
        let position = source.position(label.span.start);
        (source.name.as_str(), position.line, position.column)
    });
    json.raw("{");
    json.key("source");
    json.string(name);
    json.raw(",");
    json.key("line");
    json.integer(line);
    json.raw(",");
    json.key("column");
    json.integer(column);
    json.raw(",");
    json.key("message");
    json.string(&label.message);
    json.raw("}");
}

fn diagnostic(json: &mut Json, diagnostic: &Diagnostic, sources: &SourceMap) {
    json.raw("{");
    json.key("code");
    json.string(diagnostic.code);
    json.raw(",");
    json.key("severity");
    json.string(match diagnostic.severity {
        Severity::Error => "error",
        Severity::Warning => "warning",
    });
    json.raw(",");
    json.key("message");
    json.string(&diagnostic.message);
    json.raw(",");
    json.key("primary");
    label(json, &diagnostic.primary, sources);
    json.raw(",");
    json.key("secondary");
    json.raw("[");
    for (index, secondary) in diagnostic.secondary.iter().enumerate() {
        if index > 0 {
            json.raw(",");
        }
        label(json, secondary, sources);
    }
    json.raw("]");
    json.raw(",");
    json.key("repair");
    json.string(&diagnostic.repair);
    json.raw("}");
}

/// The response, as the record's §6 fixes it.
#[must_use]
pub fn encode(response: &Response) -> String {
    let mut json = Json(String::new());
    json.raw("{");
    json.key("format");
    json.string(crate::RESPONSE_FORMAT);
    json.raw(",");
    json.key("api");
    json.string(&response.version.to_string());
    json.raw(",");
    json.key("engine");
    json.string(response.engine);
    json.raw(",");
    json.key("status");
    json.string(response.status.slug());
    json.raw(",");
    json.key("exit");
    json.integer(response.status.code());
    json.raw(",");
    json.key("notes");
    json.strings(response.notes.iter().map(String::as_str));
    json.raw(",");
    json.key("hint");
    json.optional_string(response.hint.as_deref());
    json.raw(",");
    json.key("diagnostics");
    json.raw("[");
    for (index, item) in response.diagnostics.iter().enumerate() {
        if index > 0 {
            json.raw(",");
        }
        diagnostic(&mut json, item, &response.sources);
    }
    json.raw("]");
    json.raw(",");
    json.key("rendered");
    json.string(&response.render_diagnostics());
    json.raw(",");
    json.key("judged");
    match &response.judged {
        None => json.raw("null"),
        Some(judged) => {
            json.raw("{");
            json.key("verdict");
            json.string(Status::from_verdict(judged.verdict).slug());
            json.raw(",");
            json.key("language");
            json.string(judged.language);
            json.raw(",");
            json.key("profile");
            json.string(judged.profile);
            json.raw(",");
            json.key("declarations");
            json.integer(i64::try_from(judged.declarations.len()).unwrap_or(i64::MAX));
            json.raw(",");
            json.key("instances");
            match &judged.instances {
                Some(instances) => json.strings(instances.iter().map(String::as_str)),
                None => json.raw("null"),
            }
            json.raw(",");
            json.key("closure");
            json.raw("{");
            json.key("inside");
            json.raw("[");
            for (index, (fact, needed_by)) in judged.closure.inside.iter().enumerate() {
                if index > 0 {
                    json.raw(",");
                }
                json.raw("[");
                json.string(fact);
                json.raw(",");
                json.optional_string(needed_by.as_deref());
                json.raw("]");
            }
            json.raw("]");
            json.raw(",");
            json.key("outside");
            json.strings(judged.closure.outside.iter().map(String::as_str));
            json.raw("}");
            json.raw("}");
        }
    }
    json.raw("}");
    json.0
}
