//! The transport held to an independent model of its rules (leaf `M3.5.1.6.3`, after review round 18's R18-D4): every
//! answer the package writes, over generated message sequences, compared line for line with what a model written apart
//! from the code says the answer is.
//!
//! ⭐ **Why a model.** The table in `tests/protocol.rs` holds each site; a fault lives in a condition guarding a site, in
//! an era, in an order of checks, and a table grows a case at a time behind the faults a review finds. So the rules of
//! `docs/decisions/decision_mcp-server.md` — its `## The fact / decision` and `## As built` — are written here once more,
//! as decisions taken in the order the record states them, with an integer test, a string writer and an account of the
//! fake's tools of their own; and a seeded generator writes messages from a grammar of every member a request can carry,
//! each present, absent or of the wrong kind, in sequences that cross `initialize`, so both eras and the change between
//! them are read. The messages' wording is the contract's, so it is the one thing written alike.
//!
//! ⚠️ What it cannot find: a reading of the protocol both the code and this model share — the model is the project's
//! reading too (the transport's §5 row names it, residual). It reads `Server::answer`, line by line; the line framing is
//! `tests/protocol.rs`'s.

mod common;

use archogen_json::{self as json, Value};
use archogen_transport::Server;
use common::{quoted, Fake, LIMIT, TOOLS};

const MODERN: &str = "2026-07-28";
const LEGACY: &str = "2025-11-25";
const VERSION_KEY: &str = "io.modelcontextprotocol/protocolVersion";
const CAPABILITIES_KEY: &str = "io.modelcontextprotocol/clientCapabilities";
const INFO: &str = r#""io.modelcontextprotocol/serverInfo":{"name":"fake","version":"9.9.9"}"#;
const REQUEST_SHAPE: &str = "a request has `jsonrpc: \"2.0\"`, an id and a method";
const ID_SHAPE: &str = "a request's id is a string or an integral number";

/// Whether a JSON number's text denotes an integer: its digits, the trailing zeros moved into the exponent, scaled by
/// a power of ten that is not negative — or zero. The exponent is summed in `i128` from a saturated `i64`, so no text
/// overflows it.
fn is_integer(text: &str) -> bool {
    let unsigned = text.strip_prefix('-').unwrap_or(text);
    let (mantissa, exponent) = unsigned.split_once(['e', 'E']).unwrap_or((unsigned, "0"));
    let (whole, fraction) = mantissa.split_once('.').unwrap_or((mantissa, ""));
    let digits = format!("{whole}{fraction}");
    let significant = digits.trim_start_matches('0');
    if significant.is_empty() {
        return true;
    }
    let trailing = significant.len() - significant.trim_end_matches('0').len();
    let (sign, magnitude) = match exponent.strip_prefix('-') {
        Some(rest) => (-1i128, rest),
        None => (1, exponent.strip_prefix('+').unwrap_or(exponent)),
    };
    let magnitude: i128 = if magnitude.len() > 30 {
        i128::from(i64::MAX)
    } else {
        magnitude
            .parse::<i128>()
            .unwrap_or(i128::from(i64::MAX))
            .min(i128::from(i64::MAX))
    };
    let power = sign * magnitude - fraction.len() as i128 + trailing as i128;
    power >= 0
}

/// An id as an answer writes it: a string quoted, a number as the request wrote it.
fn written_id(id: &Value) -> String {
    match id {
        Value::String(text) => quoted(text),
        Value::Number(number) => number.as_str().to_owned(),
        _ => unreachable!("only a string or an integral number is an id"),
    }
}

fn err(id: Option<&str>, code: i64, message: &str) -> String {
    let id = id.map(|id| format!(r#""id":{id},"#)).unwrap_or_default();
    format!(
        r#"{{"jsonrpc":"2.0",{id}"error":{{"code":{code},"message":{}}}}}"#,
        quoted(message)
    )
}

fn ok(id: &str, result: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#)
}

/// What the fake's tool answers: a result's members as written, the members its own `_meta` holds, or an error.
enum Called {
    Members(String, Option<&'static str>),
    Error(i64, String),
}

fn fake_call(name: &str, arguments: &Value) -> Called {
    let meta_owned = "a tool's `_meta` is no object, or already names `io.modelcontextprotocol/serverInfo`, which is the server's";
    match name {
        "echo" => match arguments.get("text").and_then(Value::as_str) {
            Some(text) => Called::Members(
                format!(r#""content":[{{"type":"text","text":{}}}]"#, quoted(text)),
                None,
            ),
            None => Called::Members(
                r#""content":[{"type":"text","text":"`text` is missing"}],"isError":true"#.into(),
                None,
            ),
        },
        "broken" => Called::Error(-32603, "the fake broke".into()),
        "params" => Called::Error(-32602, "the fake takes none".into()),
        "meta" => Called::Members(r#""content":[{"type":"text","text":"m"}]"#.into(), Some(r#""x.y/own":"kept""#)),
        "typed" => Called::Error(
            -32603,
            "the tool `typed` answered a result holding `resultType`, which is the server's to write".into(),
        ),
        "scalar" => Called::Error(-32603, "the tool `scalar` answered a result that is no JSON object".into()),
        "clash" | "badmeta" => Called::Error(-32603, meta_owned.into()),
        "dup" => Called::Error(
            -32603,
            format!(
                "the tool `dup` answered a result that does not read back: {}",
                json::read(br#"{"a":"1","a":"2"}"#, usize::MAX).unwrap_err()
            ),
        ),
        other => unreachable!("the generator names `{other}` only as an unknown tool"),
    }
}

/// The model: whether `initialize` has selected the earlier era.
#[derive(Default)]
struct Model {
    legacy: bool,
}

impl Model {
    /// The answer to one line — not blank, under the bound — or `None` where none is given.
    fn answer(&mut self, line: &[u8]) -> Option<String> {
        // 1. A message is one JSON object.
        let message = match json::read(line, LIMIT) {
            Ok(message) => message,
            Err(refusal) => return Some(err(None, -32700, &format!("parse error: {refusal}"))),
        };
        if !matches!(message, Value::Object(_)) {
            return Some(err(None, -32600, "a message is a JSON object"));
        }
        let field = |name: &str| message.get(name);
        let rpc = matches!(field("jsonrpc"), Some(Value::String(v)) if v == "2.0");
        let method = field("method").and_then(Value::as_str);
        // 2. A client's response is ignored; a notification is never answered.
        if method.is_none() && (field("result").is_some() || field("error").is_some()) {
            return None;
        }
        let id = match field("id") {
            None if rpc && method.is_some() => return None,
            None => return Some(err(None, -32600, REQUEST_SHAPE)),
            Some(id @ Value::String(_)) => written_id(id),
            Some(id @ Value::Number(number)) if is_integer(number.as_str()) => written_id(id),
            Some(_) => return Some(err(None, -32600, ID_SHAPE)),
        };
        let id = id.as_str();
        let (true, Some(method)) = (rpc, method) else {
            return Some(err(Some(id), -32600, REQUEST_SHAPE));
        };
        let empty = Value::Object(Vec::new());
        let params = match field("params") {
            None => &empty,
            Some(params @ Value::Object(_)) => params,
            Some(_) => return Some(err(Some(id), -32602, "a request's params are an object")),
        };
        // 3. A request naming either piece of the per-request `_meta` is a `2026-07-28` one.
        let meta = params.get("_meta");
        let names = |key: &str| meta.and_then(|m| m.get(key)).is_some();
        if names(VERSION_KEY) || names(CAPABILITIES_KEY) {
            return Some(self.modern(id, method, params));
        }
        // 4. Otherwise the earlier era's lifecycle.
        Some(match method {
            "initialize" => {
                let well_formed = params.get("protocolVersion").and_then(Value::as_str).is_some()
                    && matches!(params.get("capabilities"), Some(Value::Object(_)))
                    && matches!(params.get("clientInfo"), Some(Value::Object(_)));
                if well_formed {
                    self.legacy = true;
                    ok(
                        id,
                        &format!(
                            r#"{{"protocolVersion":"{LEGACY}","capabilities":{{"tools":{{"listChanged":false}}}},"serverInfo":{{"name":"fake","version":"9.9.9"}},"instructions":"call echo"}}"#
                        ),
                    )
                } else {
                    err(Some(id), -32602, "`initialize` takes `protocolVersion`, `capabilities` and `clientInfo`")
                }
            }
            "ping" => ok(id, "{}"),
            "tools/list" | "tools/call" if self.legacy => match self.tool_request(method, params) {
                Ok((members, own)) => {
                    let meta = own.map(|own| format!(r#","_meta":{{{own}}}"#)).unwrap_or_default();
                    ok(id, &format!("{{{members}{meta}}}"))
                }
                Err((code, message)) => err(Some(id), code, &message),
            },
            _ if self.legacy => err(
                Some(id),
                -32601,
                &format!(
                    "no method `{method}` in `{LEGACY}`; a `{MODERN}` request names its version in `_meta` (`{VERSION_KEY}`)"
                ),
            ),
            "tools/list" | "tools/call" | "server/discover" => err(
                Some(id),
                -32602,
                &format!(
                    "a request names its protocol version in `_meta` (`{VERSION_KEY}`, with `{CAPABILITIES_KEY}`), or follows `initialize`"
                ),
            ),
            _ => err(Some(id), -32601, &format!("no method `{method}`")),
        })
    }

    fn modern(&self, id: &str, method: &str, params: &Value) -> String {
        let meta = params.get("_meta");
        let Some(version) = meta.and_then(|m| m.get(VERSION_KEY)) else {
            return err(Some(id), -32602, &format!("`_meta` lacks `{VERSION_KEY}`"));
        };
        let Some(version) = version.as_str() else {
            return err(
                Some(id),
                -32602,
                &format!("`_meta`'s `{VERSION_KEY}` is not a string"),
            );
        };
        if !matches!(
            meta.and_then(|m| m.get(CAPABILITIES_KEY)),
            Some(Value::Object(_))
        ) {
            return err(
                Some(id),
                -32602,
                &format!("`_meta` lacks `{CAPABILITIES_KEY}` as an object"),
            );
        }
        if version != MODERN {
            return format!(
                r#"{{"jsonrpc":"2.0","id":{id},"error":{{"code":-32022,"message":{},"data":{{"supported":["{MODERN}","{LEGACY}"],"requested":{}}}}}}}"#,
                quoted(&format!(
                    "protocol version `{version}` is not served per request: `{MODERN}` is, and `{LEGACY}` after `initialize`"
                )),
                quoted(version)
            );
        }
        let complete = |members: &str, own: Option<&str>| match own {
            // A tool's own `_meta` keeps its place, the server's identity merged into it, `resultType` after it.
            Some(own) => ok(
                id,
                &format!(r#"{{{members},"_meta":{{{own},{INFO}}},"resultType":"complete"}}"#),
            ),
            None => ok(
                id,
                &format!(r#"{{{members},"resultType":"complete","_meta":{{{INFO}}}}}"#),
            ),
        };
        match method {
            "server/discover" => complete(
                &format!(
                    r#""supportedVersions":["{MODERN}","{LEGACY}"],"capabilities":{{"tools":{{"listChanged":false}}}},"instructions":"call echo","ttlMs":0,"cacheScope":"public""#
                ),
                None,
            ),
            "tools/list" | "tools/call" => match self.tool_request(method, params) {
                Ok((members, _)) if method == "tools/list" => complete(
                    &format!(r#"{members},"ttlMs":0,"cacheScope":"public""#),
                    None,
                ),
                Ok((members, own)) => complete(&members, own),
                Err((code, message)) => err(Some(id), code, &message),
            },
            _ => err(
                Some(id),
                -32601,
                &format!("no method `{method}` in `{MODERN}`"),
            ),
        }
    }

    /// `tools/list`'s or `tools/call`'s result members — with the tool's own `_meta` members, apart — or the error.
    fn tool_request(
        &self,
        method: &str,
        params: &Value,
    ) -> Result<(String, Option<&'static str>), (i64, String)> {
        if method == "tools/list" {
            if params.get("cursor").is_some() {
                return Err((
                    -32602,
                    "the tool list has one page, so no cursor names a place in it".into(),
                ));
            }
            let list: Vec<String> = TOOLS
                .iter()
                .map(|t| format!(r#"{{"name":"{t}"}}"#))
                .collect();
            return Ok((format!(r#""tools":[{}]"#, list.join(",")), None));
        }
        let Some(name) = params.get("name").and_then(Value::as_str) else {
            return Err((-32602, "`tools/call` names its tool in `name`".into()));
        };
        if !TOOLS.contains(&name) {
            return Err((-32602, format!("no tool `{name}`")));
        }
        let empty = Value::Object(Vec::new());
        let arguments = match params.get("arguments") {
            None => &empty,
            Some(arguments @ Value::Object(_)) => arguments,
            Some(_) => return Err((-32602, "a tool's arguments are an object".into())),
        };
        match fake_call(name, arguments) {
            Called::Members(members, own) => Ok((members, own)),
            Called::Error(code, message) => Err((code, message)),
        }
    }
}

/// A small deterministic sequence (xorshift64*).
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(n).unwrap()).unwrap()
    }

    fn pick<'a>(&mut self, of: &[&'a str]) -> &'a str {
        of[self.below(of.len())]
    }

    fn chance(&mut self, percent: usize) -> bool {
        self.below(100) < percent
    }
}

/// The full per-request `_meta` of `2026-07-28`.
const FULL_META: &str = r#"{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}"#;

/// Members as written: each a name and its value's JSON text.
type Members = Vec<(String, String)>;

fn set(members: &mut Members, key: &str, value: &str) {
    match members.iter_mut().find(|(k, _)| k == key) {
        Some((_, v)) => *v = value.to_owned(),
        None => members.push((key.to_owned(), value.to_owned())),
    }
}

fn written(members: &Members) -> String {
    let members: Vec<String> = members
        .iter()
        .map(|(k, v)| format!("{}:{v}", quoted(k)))
        .collect();
    format!("{{{}}}", members.join(","))
}

/// One generated line. Most are a well-formed request of a method the protocol names — in either era — with up to
/// three of its members changed: dropped, given another kind or value, or one the protocol does not name added; so
/// every check is reached with every other check passing. The rest: a notification, a client's response, or a text that
/// is no request at all.
fn generated(rng: &mut Seeded) -> String {
    if rng.chance(5) {
        return rng
            .pick(&[
                "{",
                "[1]",
                "1",
                "\"s\"",
                "null",
                "\u{0c}",
                r#"{"a":1,"a":2}"#,
                "{} x",
            ])
            .to_owned();
    }
    let method = rng.pick(&[
        "initialize",
        "initialize",
        "ping",
        "tools/list",
        "tools/list",
        "tools/call",
        "tools/call",
        "tools/call",
        "server/discover",
    ]);
    let mut params: Members = Vec::new();
    if rng.chance(50) {
        set(&mut params, "_meta", FULL_META);
    }
    match method {
        "initialize" => {
            set(&mut params, "protocolVersion", "\"2025-11-25\"");
            set(&mut params, "capabilities", "{}");
            set(&mut params, "clientInfo", r#"{"name":"c","version":"1"}"#);
        }
        "tools/call" => {
            let tool = format!("\"{}\"", rng.pick(TOOLS));
            set(&mut params, "name", &tool);
            if rng.chance(70) {
                let text = rng.pick(&[
                    r#""hi""#,
                    r#""a\"b\\c""#,
                    r#""line\nbreak\ttab\u0001""#,
                    r#"" \u0085é""#,
                ]);
                set(&mut params, "arguments", &format!(r#"{{"text":{text}}}"#));
            }
        }
        _ => {}
    }
    let id = rng.pick(&[
        "1",
        "\"a\"",
        "\"q\\\"\"",
        "-0",
        "1e2",
        "1.0",
        "150e-1",
        "12345678901234567890",
    ]);
    let mut top: Members = vec![
        ("jsonrpc".into(), "\"2.0\"".into()),
        ("id".into(), id.into()),
        ("method".into(), format!("\"{method}\"")),
    ];
    let mut params_text: Option<String> = None;
    for _ in 0..rng.below(4) {
        match rng.below(8) {
            0 => {
                let key = rng.pick(&["jsonrpc", "id", "method", "params"]);
                top.retain(|(k, _)| k != key);
                if key == "params" {
                    params_text = Some(String::new());
                }
            }
            1 => {
                let (key, values): (&str, &[&str]) = match rng.below(3) {
                    0 => ("jsonrpc", &["\"1.0\"", "2"]),
                    1 => (
                        "id",
                        &[
                            "1.5",
                            "1e-1",
                            "null",
                            "{}",
                            "[]",
                            "true",
                            "5e-99999999999999999999",
                            "5e-9223372036854775808",
                            "1e99999999999999999999",
                        ],
                    ),
                    _ => (
                        "method",
                        &[
                            "\"frobnicate\"",
                            "\"a\\\"b\"",
                            "7",
                            "\"ping\"",
                            "\"initialize\"",
                            "\"tools/list\"",
                            "\"tools/call\"",
                            "\"server/discover\"",
                        ],
                    ),
                };
                set(&mut top, key, rng.pick(values));
            }
            2 => params_text = Some(rng.pick(&["[]", "null", "\"p\""]).to_owned()),
            3 => {
                let key = rng.pick(&[
                    "_meta",
                    "protocolVersion",
                    "capabilities",
                    "clientInfo",
                    "name",
                    "arguments",
                ]);
                params.retain(|(k, _)| k != key);
            }
            4 | 5 => {
                let (key, values): (&str, &[&str]) = match rng.below(7) {
                    0 => (
                        "_meta",
                        &[
                            r#"{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}"#,
                            r#"{"io.modelcontextprotocol/clientCapabilities":{}}"#,
                            r#"{"io.modelcontextprotocol/protocolVersion":2026,"io.modelcontextprotocol/clientCapabilities":{}}"#,
                            r#"{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":[]}"#,
                            r#"{"io.modelcontextprotocol/protocolVersion":"2024-11-05","io.modelcontextprotocol/clientCapabilities":{}}"#,
                            r#"{"io.modelcontextprotocol/protocolVersion":"2025-11-25","io.modelcontextprotocol/clientCapabilities":{}}"#,
                            r#"{"x":1}"#,
                            r#""not an object""#,
                            FULL_META,
                        ],
                    ),
                    1 => (
                        "protocolVersion",
                        &["\"2024-01-01\"", "7", "\"2025-11-25\""],
                    ),
                    2 => ("capabilities", &["[]", "null", "{}"]),
                    3 => ("clientInfo", &["\"c\"", "{}"]),
                    4 => (
                        "name",
                        &["\"nothing\"", "7", "\"echo\"", "\"dup\"", "\"meta\""],
                    ),
                    5 => (
                        "arguments",
                        &[
                            "[1]",
                            "\"x\"",
                            "null",
                            "{}",
                            r#"{"text":7}"#,
                            r#"{"text":"t"}"#,
                        ],
                    ),
                    _ => ("cursor", &["\"x\""]),
                };
                set(&mut params, key, rng.pick(values));
            }
            6 => {
                // A member the protocol does not name, which a server ignores: in `params`, or beside it.
                let key = rng.pick(&["y", "x", "extra", "_meta2", "Method", "ID"]);
                let value = rng.pick(&["1", "{}", "\"v\""]);
                if rng.chance(50) {
                    set(&mut params, key, value);
                } else {
                    set(&mut top, key, value);
                }
            }
            _ => {
                let (key, value) = if rng.chance(50) {
                    ("result", "{}")
                } else {
                    ("error", r#"{"code":-1,"message":"no"}"#)
                };
                set(&mut top, key, value);
            }
        }
    }
    match params_text {
        Some(text) if text.is_empty() => {}
        Some(text) => set(&mut top, "params", &text),
        None => set(&mut top, "params", &written(&params)),
    }
    written(&top)
}

#[test]
fn every_answer_is_the_model_s_over_generated_sequences_that_cross_initialize() {
    let mut rng = Seeded(0x0005_eed0_fa11_5158);
    let (sequences, mut lines, mut answered, mut legacy, mut wrong) = (4000, 0, 0, 0, Vec::new());
    for sequence in 0..sequences {
        let mut server = Server::new(&Fake, LIMIT);
        let mut model = Model::default();
        for _ in 0..1 + rng.below(8) {
            let line = generated(&mut rng);
            lines += 1;
            let got = server.answer(line.as_bytes());
            let want = model.answer(line.as_bytes());
            answered += usize::from(want.is_some());
            legacy += usize::from(model.legacy);
            if got != want {
                wrong.push(format!(
                    "sequence {sequence}: {line}\n  model  {want:?}\n  server {got:?}"
                ));
            }
        }
    }
    assert!(
        lines > 9000 && answered > lines / 2 && legacy > lines / 20,
        "{lines} lines, {answered} answered, {legacy} in the earlier era"
    );
    assert!(
        wrong.is_empty(),
        "{} of {lines} answers differ from the model's:\n{}",
        wrong.len(),
        wrong
            .iter()
            .take(12)
            .cloned()
            .collect::<Vec<_>>()
            .join("\n")
    );
}

#[test]
fn the_model_s_integer_test_reads_every_spelling_as_json_schema_does() {
    for text in [
        "0",
        "-0",
        "0.0e5",
        "7",
        "-7",
        "1e2",
        "1E+2",
        "1.0",
        "150e-1",
        "12345678901234567890",
        "1e99999999999999999999",
    ] {
        assert!(is_integer(text), "{text}");
    }
    for text in [
        "1.5",
        "1e-1",
        "15e-2",
        "0.5",
        "-2.25",
        "5e-99999999999999999999",
        "5e-9223372036854775808",
    ] {
        assert!(!is_integer(text), "{text}");
    }
}
