//! The MCP server's framing (leaf `M3.5.1.6.1`; `M3.5.1`'s design, *The programmatic surface, named for what it does*),
//! as `docs/decisions/decision_mcp-server.md` decides it for `archogen mcp`, moved here so that each role serves its own
//! tools over one framing: the generator's server today, each checker's own (`M3.5.1.6.4`, `M2.21.4`).
//!
//! One JSON-RPC 2.0 message per line on the input, one answer per line on the output, nothing else on the output, and
//! the server returns when the input ends. It speaks both protocol eras the record names:
//! - **`2026-07-28`**, per request: a request whose `_meta` carries `io.modelcontextprotocol/protocolVersion` and
//!   `io.modelcontextprotocol/clientCapabilities` is answered on its own, with `server/discover`, `tools/list` and
//!   `tools/call`; a request missing either is refused with `-32602`, another version with `-32022`.
//! - **`2025-11-25`**, after `initialize`, which selects it for the process: `tools/list`, `tools/call` and `ping`.
//!   A `ping` before `initialize` is answered too, a leniency: that revision makes `initialize` the first exchange and
//!   lets a client ping while it waits for the answer, and an early ping's answer costs nothing.
//!
//! What the tools are, what each takes and what it answers, is the server's own, behind [`Tools`]. A call naming no
//! tool, an unknown tool or arguments that are no object is `-32602`; a tool's own refusal of its arguments is the
//! tool's result, `isError: true` ([`tool_error`]), as both revisions' tools pages ask of "input validation errors".
//!
//! An answer to a request whose id could not be read carries no id, which both revisions' schemas allow and neither
//! lets be `null`; a request's id is a string or an integral number.
//!
//! ⛔ It reads with `archogen-json` under the bound its server gives, and spawns nothing and touches no file.

use std::io::{self, BufRead, Write};

use archogen_json::{self as json, Number, Value};

/// The protocol revision a request names in its `_meta`.
pub const MODERN: &str = "2026-07-28";
/// The revision `initialize` selects.
pub const LEGACY: &str = "2025-11-25";

const META_VERSION: &str = "io.modelcontextprotocol/protocolVersion";
const META_CAPABILITIES: &str = "io.modelcontextprotocol/clientCapabilities";
const META_SERVER_INFO: &str = "io.modelcontextprotocol/serverInfo";

const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;
const INTERNAL_ERROR: i64 = -32603;
const UNSUPPORTED_PROTOCOL_VERSION: i64 = -32022;

/// A server's own part: who it is, and its tools.
pub trait Tools {
    /// Its name, `serverInfo.name`.
    fn name(&self) -> &str;
    /// Its version, `serverInfo.version`.
    fn version(&self) -> &str;
    /// One sentence for a client to hand its model, in `server/discover` and `initialize`.
    fn instructions(&self) -> &str;
    /// Every tool, as `tools/list` lists it: one page, so no cursor names a place in it.
    fn list(&self) -> Vec<Value>;
    /// Whether it has a tool of this name.
    fn knows(&self, name: &str) -> bool;
    /// A call of the tool `name`, which it knows, with `arguments`, an object.
    ///
    /// # Errors
    ///
    /// The [`Failure`] its answer is: params the tool cannot be called with, or the server failing.
    fn call(&self, name: &str, arguments: &Value) -> Result<Value, Failure>;
}

/// Why a request was answered with an error rather than a result.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Failure {
    /// The request's params are not what its method takes: `-32602`.
    Params(String),
    /// The server failed, not the request: `-32603`.
    Internal(String),
}

impl Failure {
    fn answer(self, id: &Value) -> String {
        match self {
            Self::Params(message) => error(Some(id), INVALID_PARAMS, &message, None),
            Self::Internal(message) => error(Some(id), INTERNAL_ERROR, &message, None),
        }
    }
}

/// One server's state: its tools, the longest line it reads, and whether `initialize` has selected the earlier
/// revision for this process.
pub struct Server<'t> {
    tools: &'t dyn Tools,
    limit: usize,
    legacy: bool,
}

/// Serve `input` until it ends, answering each line on `output`; a line longer than `limit` bytes is refused with
/// `-32700` and never parsed.
///
/// # Errors
///
/// The streams' own failure.
pub fn serve(
    input: &mut dyn BufRead,
    output: &mut dyn Write,
    tools: &dyn Tools,
    limit: usize,
) -> io::Result<()> {
    let mut server = Server::new(tools, limit);
    loop {
        let (line, too_long) = match read_line(input, limit)? {
            None => return Ok(()),
            Some(read) => read,
        };
        if !too_long && line.iter().all(|byte| matches!(byte, b' ' | b'\t' | b'\r')) {
            continue; // a blank line is no message
        }
        let answer = if too_long {
            Some(error(
                None,
                PARSE_ERROR,
                &format!("the message is longer than {limit} bytes, and was not read"),
                None,
            ))
        } else {
            server.answer(&line)
        };
        if let Some(answer) = answer {
            output.write_all(answer.as_bytes())?;
            output.write_all(b"\n")?;
            output.flush()?;
        }
    }
}

/// The next line without its terminator, and whether it ran past `limit` — in which case only that much is kept and
/// the rest is consumed unread. `None` at the end of the input.
fn read_line(input: &mut dyn BufRead, limit: usize) -> io::Result<Option<(Vec<u8>, bool)>> {
    let mut line = Vec::new();
    let mut too_long = false;
    let mut any = false;
    loop {
        let buffer = input.fill_buf()?;
        if buffer.is_empty() {
            return Ok(any.then_some((line, too_long)));
        }
        any = true;
        let (chunk, found) = match buffer.iter().position(|&byte| byte == b'\n') {
            Some(end) => (&buffer[..end], true),
            None => (buffer, false),
        };
        if !too_long {
            if line.len() + chunk.len() > limit {
                too_long = true;
                line.clear();
            } else {
                line.extend_from_slice(chunk);
            }
        }
        let used = chunk.len() + usize::from(found);
        input.consume(used);
        if found {
            return Ok(Some((line, too_long)));
        }
    }
}

impl<'t> Server<'t> {
    /// A server of `tools`, reading lines of at most `limit` bytes, before any `initialize`.
    #[must_use]
    pub fn new(tools: &'t dyn Tools, limit: usize) -> Self {
        Self {
            tools,
            limit,
            legacy: false,
        }
    }

    /// The answer to one line, or `None` for a notification.
    pub fn answer(&mut self, line: &[u8]) -> Option<String> {
        let message = match json::read(line, self.limit) {
            Ok(message) => message,
            Err(refusal) => {
                return Some(error(
                    None,
                    PARSE_ERROR,
                    &format!("parse error: {refusal}"),
                    None,
                ));
            }
        };
        if !matches!(message, Value::Object(_)) {
            return Some(error(
                None,
                INVALID_REQUEST,
                "a message is a JSON object",
                None,
            ));
        }
        let version_ok = message.get("jsonrpc").and_then(Value::as_str) == Some("2.0");
        let method = message.get("method").and_then(Value::as_str);
        // A response is a client's answer to a request of the server's, and this server sends none: it is ignored.
        if method.is_none() && (message.get("result").is_some() || message.get("error").is_some()) {
            return None;
        }
        let id = match message.get("id") {
            None => {
                // A notification is never answered; a message with neither an id nor a method is no request.
                return if version_ok && method.is_some() {
                    None
                } else {
                    Some(error(
                        None,
                        INVALID_REQUEST,
                        "a request has `jsonrpc: \"2.0\"`, an id and a method",
                        None,
                    ))
                };
            }
            Some(Value::Number(number)) if !integral(number.as_str()) => {
                return Some(error(
                    None,
                    INVALID_REQUEST,
                    "a request's id is a string or an integral number",
                    None,
                ));
            }
            Some(id @ (Value::String(_) | Value::Number(_))) => id.clone(),
            Some(_) => {
                return Some(error(
                    None,
                    INVALID_REQUEST,
                    "a request's id is a string or an integral number",
                    None,
                ));
            }
        };
        let (true, Some(method)) = (version_ok, method) else {
            return Some(error(
                Some(&id),
                INVALID_REQUEST,
                "a request has `jsonrpc: \"2.0\"`, an id and a method",
                None,
            ));
        };
        let empty = Value::Object(Vec::new());
        let params = match message.get("params") {
            None => &empty,
            Some(params @ Value::Object(_)) => params,
            Some(_) => {
                return Some(error(
                    Some(&id),
                    INVALID_PARAMS,
                    "a request's params are an object",
                    None,
                ))
            }
        };
        Some(self.request(&id, method, params))
    }

    fn request(&mut self, id: &Value, method: &str, params: &Value) -> String {
        let meta = params.get("_meta");
        let modern = meta.is_some_and(|meta| {
            meta.get(META_VERSION).is_some() || meta.get(META_CAPABILITIES).is_some()
        });
        if modern {
            return self.modern_request(id, method, params);
        }
        match method {
            "initialize" => self.initialize(id, params),
            "ping" => result(id, Value::Object(Vec::new())),
            "tools/list" | "tools/call" if self.legacy => self.legacy_request(id, method, params),
            _ if self.legacy => error(
                Some(id),
                METHOD_NOT_FOUND,
                &format!(
                    "no method `{method}` in `{LEGACY}`; a `{MODERN}` request names its version in `_meta` \
                     (`{META_VERSION}`)"
                ),
                None,
            ),
            "tools/list" | "tools/call" | "server/discover" => error(
                Some(id),
                INVALID_PARAMS,
                &format!(
                    "a request names its protocol version in `_meta` (`{META_VERSION}`, with \
                     `{META_CAPABILITIES}`), or follows `initialize`"
                ),
                None,
            ),
            _ => error(Some(id), METHOD_NOT_FOUND, &format!("no method `{method}`"), None),
        }
    }

    /// `initialize`: the earlier revision, for the rest of the process. The client's version is answered with
    /// `2025-11-25`, the one earlier revision this server supports, whatever it named (the lifecycle's version
    /// negotiation: the same version if supported, otherwise another the server supports).
    fn initialize(&mut self, id: &Value, params: &Value) -> String {
        let well_formed = params
            .get("protocolVersion")
            .and_then(Value::as_str)
            .is_some()
            && matches!(params.get("capabilities"), Some(Value::Object(_)))
            && matches!(params.get("clientInfo"), Some(Value::Object(_)));
        if !well_formed {
            return error(
                Some(id),
                INVALID_PARAMS,
                "`initialize` takes `protocolVersion`, `capabilities` and `clientInfo`",
                None,
            );
        }
        self.legacy = true;
        result(
            id,
            object(vec![
                ("protocolVersion", string(LEGACY)),
                ("capabilities", tools_capability()),
                ("serverInfo", self.server_info()),
                ("instructions", string(self.tools.instructions())),
            ]),
        )
    }

    /// A request under `2026-07-28`: its `_meta` checked, then answered on its own.
    fn modern_request(&self, id: &Value, method: &str, params: &Value) -> String {
        let meta = params.get("_meta");
        let Some(version) = meta.and_then(|meta| meta.get(META_VERSION)) else {
            return error(
                Some(id),
                INVALID_PARAMS,
                &format!("`_meta` lacks `{META_VERSION}`"),
                None,
            );
        };
        let Some(version) = version.as_str() else {
            return error(
                Some(id),
                INVALID_PARAMS,
                &format!("`_meta`'s `{META_VERSION}` is not a string"),
                None,
            );
        };
        if !matches!(
            meta.and_then(|meta| meta.get(META_CAPABILITIES)),
            Some(Value::Object(_))
        ) {
            return error(
                Some(id),
                INVALID_PARAMS,
                &format!("`_meta` lacks `{META_CAPABILITIES}` as an object"),
                None,
            );
        }
        if version != MODERN {
            return error(
                Some(id),
                UNSUPPORTED_PROTOCOL_VERSION,
                &format!(
                    "protocol version `{version}` is not served per request: `{MODERN}` is, and `{LEGACY}` after `initialize`"
                ),
                Some(object(vec![
                    ("supported", Value::Array(vec![string(MODERN), string(LEGACY)])),
                    ("requested", string(version)),
                ])),
            );
        }
        let answered = match method {
            "server/discover" => Ok(object(vec![
                (
                    "supportedVersions",
                    Value::Array(vec![string(MODERN), string(LEGACY)]),
                ),
                ("capabilities", tools_capability()),
                ("instructions", string(self.tools.instructions())),
                ("ttlMs", integer(0)),
                ("cacheScope", string("public")),
            ])),
            "tools/list" => self.list_tools(params).map(|tools| {
                object(vec![
                    ("tools", tools),
                    ("ttlMs", integer(0)),
                    ("cacheScope", string("public")),
                ])
            }),
            "tools/call" => self.call_tool(params),
            _ => {
                return error(
                    Some(id),
                    METHOD_NOT_FOUND,
                    &format!("no method `{method}` in `{MODERN}`"),
                    None,
                )
            }
        };
        match answered {
            Ok(Value::Object(mut members)) => {
                members.push(("resultType".to_owned(), string("complete")));
                members.push((
                    "_meta".to_owned(),
                    object(vec![(META_SERVER_INFO, self.server_info())]),
                ));
                result(id, Value::Object(members))
            }
            Ok(other) => result(id, other),
            Err(failure) => failure.answer(id),
        }
    }

    /// `tools/list` or `tools/call` under `2025-11-25`, after `initialize`.
    fn legacy_request(&self, id: &Value, method: &str, params: &Value) -> String {
        let answered = if method == "tools/list" {
            self.list_tools(params)
                .map(|tools| object(vec![("tools", tools)]))
        } else {
            self.call_tool(params)
        };
        match answered {
            Ok(value) => result(id, value),
            Err(failure) => failure.answer(id),
        }
    }

    /// The tool list. A server has one page, so no cursor names a place in it.
    fn list_tools(&self, params: &Value) -> Result<Value, Failure> {
        if params.get("cursor").is_some() {
            return Err(Failure::Params(
                "the tool list has one page, so no cursor names a place in it".to_owned(),
            ));
        }
        Ok(Value::Array(self.tools.list()))
    }

    /// `tools/call`'s result, or why the request is no call (`-32602`) or the server failed (`-32603`). A call that
    /// names a tool and gives it an object is the tool's to answer, an argument it cannot use included.
    fn call_tool(&self, params: &Value) -> Result<Value, Failure> {
        let Some(name) = params.get("name").and_then(Value::as_str) else {
            return Err(Failure::Params(
                "`tools/call` names its tool in `name`".to_owned(),
            ));
        };
        if !self.tools.knows(name) {
            return Err(Failure::Params(format!("no tool `{name}`")));
        }
        let empty = Value::Object(Vec::new());
        let arguments = match params.get("arguments") {
            None => &empty,
            Some(arguments @ Value::Object(_)) => arguments,
            Some(_) => {
                return Err(Failure::Params(
                    "a tool's arguments are an object".to_owned(),
                ))
            }
        };
        self.tools.call(name, arguments)
    }

    fn server_info(&self) -> Value {
        object(vec![
            ("name", string(self.tools.name())),
            ("version", string(self.tools.version())),
        ])
    }
}

/// A tool's result for a call it could not carry out: its text, and `isError: true`.
#[must_use]
pub fn tool_error(text: &str) -> Value {
    object(vec![
        ("content", Value::Array(vec![text_block(text)])),
        ("isError", Value::Bool(true)),
    ])
}

/// A text content block.
#[must_use]
pub fn text_block(text: &str) -> Value {
    object(vec![("type", string("text")), ("text", string(text))])
}

fn tools_capability() -> Value {
    object(vec![(
        "tools",
        object(vec![("listChanged", Value::Bool(false))]),
    )])
}

fn result(id: &Value, result: Value) -> String {
    json::write(&object(vec![
        ("jsonrpc", string("2.0")),
        ("id", id.clone()),
        ("result", result),
    ]))
}

/// An error answer. `id` is `None` when the request's id could not be read: the answer then carries none, which
/// both revisions' schemas allow (`id?: RequestId`) and neither lets be `null`.
fn error(id: Option<&Value>, code: i64, message: &str, data: Option<Value>) -> String {
    let mut fields = vec![("code", integer(code)), ("message", string(message))];
    if let Some(data) = data {
        fields.push(("data", data));
    }
    let mut members = vec![("jsonrpc", string("2.0"))];
    if let Some(id) = id {
        members.push(("id", id.clone()));
    }
    members.push(("error", object(fields)));
    json::write(&object(members))
}

/// Whether a number's text is an integer's — `7`, `1e2`, `1.0`, `150e-1` — as JSON Schema's `integer`, which both
/// revisions' `RequestId` uses, reads one; `1.5` and `1e-1` are not.
fn integral(text: &str) -> bool {
    let text = text.trim_start_matches('-');
    let (mantissa, exponent) = match text.find(['e', 'E']) {
        Some(at) => (&text[..at], &text[at + 1..]),
        None => (text, "0"),
    };
    let exponent: i64 = exponent.parse().unwrap_or(i64::MAX);
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let fraction = fraction.trim_end_matches('0');
    if whole.trim_start_matches('0').is_empty() && fraction.is_empty() {
        return true; // zero
    }
    if exponent >= 0 {
        i64::try_from(fraction.len()).is_ok_and(|digits| digits <= exponent)
    } else {
        let zeros = whole.len() - whole.trim_end_matches('0').len();
        fraction.is_empty() && i64::try_from(zeros).is_ok_and(|zeros| zeros >= -exponent)
    }
}

/// A JSON object of `members`, in their order.
#[must_use]
pub fn object<K: Into<String>>(members: Vec<(K, Value)>) -> Value {
    Value::Object(
        members
            .into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect(),
    )
}

/// A JSON string.
#[must_use]
pub fn string(text: &str) -> Value {
    Value::String(text.to_owned())
}

/// A JSON integer.
#[must_use]
pub fn integer(value: i64) -> Value {
    Value::Number(Number::from(value))
}

#[cfg(test)]
mod tests {
    use super::integral;

    #[test]
    fn a_number_is_integral_as_json_schema_reads_one() {
        for text in [
            "0",
            "-0",
            "7",
            "-7",
            "1e2",
            "1E+2",
            "1.0",
            "150e-1",
            "0.0e5",
            "12345678901234567890",
        ] {
            assert!(integral(text), "{text}");
        }
        for text in ["1.5", "1e-1", "15e-2", "0.5", "-2.25"] {
            assert!(!integral(text), "{text}");
        }
    }
}
