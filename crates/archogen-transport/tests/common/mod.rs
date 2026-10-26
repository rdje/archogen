//! The fake server both protocol suites run the transport over: a few tools, each ending a call one way. `call` is
//! reached only for a name `knows` admits — any other name panics, so a framing that skipped `knows` fails.

use archogen_json::Value;
use archogen_transport::{object, string, text_block, tool_error, Failure, Tools};

/// The longest line the suites' servers read.
pub const LIMIT: usize = 4096;

/// The fake's tools, in the order it lists them.
pub const TOOLS: &[&str] = &[
    "echo", "broken", "params", "meta", "typed", "scalar", "clash", "badmeta", "dup",
];

/// The fake server.
pub struct Fake;

impl Tools for Fake {
    fn name(&self) -> &str {
        "fake"
    }
    fn version(&self) -> &str {
        "9.9.9"
    }
    fn instructions(&self) -> &str {
        "call echo"
    }
    fn list(&self) -> Vec<Value> {
        TOOLS
            .iter()
            .map(|name| object(vec![("name", string(name))]))
            .collect()
    }
    fn knows(&self, name: &str) -> bool {
        TOOLS.contains(&name)
    }
    fn call(&self, name: &str, arguments: &Value) -> Result<Value, Failure> {
        match name {
            "echo" => Ok(match arguments.get("text").and_then(Value::as_str) {
                Some(text) => object(vec![("content", Value::Array(vec![text_block(text)]))]),
                None => tool_error("`text` is missing"),
            }),
            "broken" => Err(Failure::Internal("the fake broke".to_owned())),
            "params" => Err(Failure::Params("the fake takes none".to_owned())),
            // A result carrying its own `_meta`, as the schema's `Result` allows.
            "meta" => Ok(object(vec![
                ("content", Value::Array(vec![text_block("m")])),
                ("_meta", object(vec![("x.y/own", string("kept"))])),
            ])),
            "typed" => Ok(object(vec![("resultType", string("complete"))])),
            "scalar" => Ok(string("not an object")),
            "clash" => Ok(object(vec![(
                "_meta",
                object(vec![(
                    "io.modelcontextprotocol/serverInfo",
                    string("forged"),
                )]),
            )])),
            "badmeta" => Ok(object(vec![("_meta", string("not an object"))])),
            // A result naming a member twice, which no strict reader reads back.
            "dup" => Ok(object(vec![("a", string("1")), ("a", string("2"))])),
            other => panic!("`call` reached for `{other}`, which the server does not know"),
        }
    }
}

/// `text` as a JSON string, written by this suite's own hand: `"` and `\` escaped, `\b \t \n \f \r` by their short
/// escapes, every other character below `0x20` as `\u00xx`, and nothing else — the MCP record's one spelling.
pub fn quoted(text: &str) -> String {
    let mut out = String::from("\"");
    for c in text.chars() {
        match c {
            '"' => out.push_str("\\\""),
            '\\' => out.push_str("\\\\"),
            '\u{08}' => out.push_str("\\b"),
            '\t' => out.push_str("\\t"),
            '\n' => out.push_str("\\n"),
            '\u{0c}' => out.push_str("\\f"),
            '\r' => out.push_str("\\r"),
            c if (c as u32) < 0x20 => out.push_str(&format!("\\u{:04x}", c as u32)),
            c => out.push(c),
        }
    }
    out.push('"');
    out
}
