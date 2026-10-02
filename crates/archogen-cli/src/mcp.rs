//! `archogen mcp` — the MCP server (leaf `API.6.4`), as `docs/decisions/decision_mcp-server.md` decides it.
//!
//! One JSON-RPC 2.0 message per line on stdin, one answer per line on stdout, nothing else on stdout, and the
//! server returns when stdin ends. It speaks both protocol eras the record names:
//! - **`2026-07-28`**, per request: a request whose `_meta` carries `io.modelcontextprotocol/protocolVersion` and
//!   `io.modelcontextprotocol/clientCapabilities` is answered on its own, with `server/discover`, `tools/list` and
//!   `tools/call`; a request missing either is refused with `-32602`, another version with `-32022`.
//! - **`2025-11-25`**, after `initialize`, which selects it for the process: `tools/list`, `tools/call` and `ping`.
//!   A `ping` before `initialize` is answered too, since that revision's lifecycle lets a client send pings before
//!   the server has answered `initialize` (its `basic/lifecycle`, read at the ledger's commit).
//!
//! The tools are the command table's ([`crate::spec::tools`]). `check` takes the engine API's request — the
//! description's text, its profile and its modules' texts — and answers the engine API's response in the wasm
//! binding's encoding, as `structuredContent` and as the one text block; `isError` is `false` only when the
//! description is accepted. A tool that is not built answers with the leaf that owns it.
//!
//! ⛔ It reads with [`crate::json`], under [`LINE_LIMIT`]; it spawns nothing and touches no file.

use std::io::{self, BufRead, Write};

use archogen_api::{check_with, Limits, MemoryModules, Request, Status};

use crate::json::{self, Number, Value};
use crate::spec::{self, CommandSpec, Maturity};

/// The protocol revision a request names in its `_meta`.
pub const MODERN: &str = "2026-07-28";
/// The revision `initialize` selects.
pub const LEGACY: &str = "2025-11-25";

const META_VERSION: &str = "io.modelcontextprotocol/protocolVersion";
const META_CAPABILITIES: &str = "io.modelcontextprotocol/clientCapabilities";
const META_SERVER_INFO: &str = "io.modelcontextprotocol/serverInfo";
/// The prefix of the keys this server adds to a tool's `_meta` (the record's §2).
const PREFIX: &str = "io.github.rdje.archogen/";

/// The longest line read. A request carries a description under the engine API's byte budget, and JSON's escapes
/// can double a text's length (a line feed is written `\n`), so the bound is twice the budget and a fixed
/// allowance for the envelope; a longer line is refused with `-32700` and never parsed (the record's §5).
pub const LINE_LIMIT: usize = 2 * archogen_api::DEFAULT_BYTES + 64 * 1024;

const PARSE_ERROR: i64 = -32700;
const INVALID_REQUEST: i64 = -32600;
const METHOD_NOT_FOUND: i64 = -32601;
const INVALID_PARAMS: i64 = -32602;
const UNSUPPORTED_PROTOCOL_VERSION: i64 = -32022;

/// One sentence for a client to hand its model, in `server/discover` and `initialize`.
const INSTRUCTIONS: &str = "archogen checks eADL system descriptions: call `check` with a description's text, and \
read the verdict in the result's `status`; the other tools are listed with the task-tree leaf that will build them.";

/// The server's state: whether `initialize` has selected the earlier revision for this process.
#[derive(Debug, Default)]
pub struct Server {
    legacy: bool,
}

/// Serve `input` until it ends, answering each line on `output`.
pub fn serve(input: &mut dyn BufRead, output: &mut dyn Write) -> io::Result<()> {
    let mut server = Server::default();
    loop {
        let (line, too_long) = match read_line(input)? {
            None => return Ok(()),
            Some(read) => read,
        };
        let answer = if too_long {
            Some(error(
                &Value::Null,
                PARSE_ERROR,
                &format!("the message is longer than {LINE_LIMIT} bytes, and was not read"),
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

/// The next line without its terminator, and whether it ran past [`LINE_LIMIT`] — in which case only that much is
/// kept and the rest is consumed unread. `None` at the end of the input.
fn read_line(input: &mut dyn BufRead) -> io::Result<Option<(Vec<u8>, bool)>> {
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
            if line.len() + chunk.len() > LINE_LIMIT {
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

impl Server {
    /// The answer to one line, or `None` for a notification.
    pub fn answer(&mut self, line: &[u8]) -> Option<String> {
        let message = match json::read(line, LINE_LIMIT) {
            Ok(message) => message,
            Err(refusal) => {
                return Some(error(
                    &Value::Null,
                    PARSE_ERROR,
                    &format!("parse error: {refusal}"),
                    None,
                ));
            }
        };
        if !matches!(message, Value::Object(_)) {
            return Some(error(
                &Value::Null,
                INVALID_REQUEST,
                "a message is a JSON object",
                None,
            ));
        }
        let version_ok = message.get("jsonrpc").and_then(Value::as_str) == Some("2.0");
        let method = message.get("method").and_then(Value::as_str);
        let id = match message.get("id") {
            None => {
                // A notification is never answered; a message with neither an id nor a method is no request.
                return if version_ok && method.is_some() {
                    None
                } else {
                    Some(error(
                        &Value::Null,
                        INVALID_REQUEST,
                        "a request has `jsonrpc: \"2.0\"`, an id and a method",
                        None,
                    ))
                };
            }
            Some(id @ (Value::String(_) | Value::Number(_))) => id.clone(),
            Some(_) => {
                return Some(error(
                    &Value::Null,
                    INVALID_REQUEST,
                    "a request's id is a string or a number",
                    None,
                ));
            }
        };
        let (true, Some(method)) = (version_ok, method) else {
            return Some(error(
                &id,
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
                    &id,
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
            return modern_request(id, method, params);
        }
        match method {
            "initialize" => self.initialize(id, params),
            "ping" => result(id, Value::Object(Vec::new())),
            "tools/list" | "tools/call" if self.legacy => legacy_request(id, method, params),
            "tools/list" | "tools/call" | "server/discover" => error(
                id,
                INVALID_PARAMS,
                &format!(
                    "a request names its protocol version in `_meta` (`{META_VERSION}`, with \
                     `{META_CAPABILITIES}`), or follows `initialize`"
                ),
                None,
            ),
            _ => error(id, METHOD_NOT_FOUND, &format!("no method `{method}`"), None),
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
                id,
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
                ("serverInfo", server_info()),
                ("instructions", string(INSTRUCTIONS)),
            ]),
        )
    }
}

/// A request under `2026-07-28`: its `_meta` checked, then answered on its own.
fn modern_request(id: &Value, method: &str, params: &Value) -> String {
    let meta = params.get("_meta");
    let Some(version) = meta
        .and_then(|meta| meta.get(META_VERSION))
        .and_then(Value::as_str)
    else {
        return error(
            id,
            INVALID_PARAMS,
            &format!("`_meta` lacks `{META_VERSION}`"),
            None,
        );
    };
    if !matches!(
        meta.and_then(|meta| meta.get(META_CAPABILITIES)),
        Some(Value::Object(_))
    ) {
        return error(
            id,
            INVALID_PARAMS,
            &format!("`_meta` lacks `{META_CAPABILITIES}`"),
            None,
        );
    }
    if version != MODERN {
        return error(
            id,
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
            ("instructions", string(INSTRUCTIONS)),
            ("ttlMs", integer(0)),
            ("cacheScope", string("public")),
        ])),
        "tools/list" => list_tools(params).map(|tools| {
            object(vec![
                ("tools", tools),
                ("ttlMs", integer(0)),
                ("cacheScope", string("public")),
            ])
        }),
        "tools/call" => call_tool(params),
        _ => {
            return error(
                id,
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
                object(vec![(META_SERVER_INFO, server_info())]),
            ));
            result(id, Value::Object(members))
        }
        Ok(other) => result(id, other),
        Err(message) => error(id, INVALID_PARAMS, &message, None),
    }
}

/// `tools/list` or `tools/call` under `2025-11-25`, after `initialize`.
fn legacy_request(id: &Value, method: &str, params: &Value) -> String {
    let answered = if method == "tools/list" {
        list_tools(params).map(|tools| object(vec![("tools", tools)]))
    } else {
        call_tool(params)
    };
    match answered {
        Ok(value) => result(id, value),
        Err(message) => error(id, INVALID_PARAMS, &message, None),
    }
}

/// The tool list, from the command table. This server has one page, so no cursor names a place in it.
fn list_tools(params: &Value) -> Result<Value, String> {
    if params.get("cursor").is_some() {
        return Err("the tool list has one page, so no cursor names a place in it".to_owned());
    }
    Ok(Value::Array(spec::tools().map(tool).collect()))
}

/// One tool: its name and summary, its arguments' schema, the annotations true of every operation offered, and
/// its maturity and owner under this server's prefix (the record's §2).
fn tool(spec: &CommandSpec) -> Value {
    let (maturity, owner) = match spec.maturity {
        Maturity::Built => ("built", None),
        Maturity::Experimental { completed_by, .. } => ("experimental", Some(completed_by)),
        Maturity::Unimplemented { owner } => ("unimplemented", Some(owner)),
    };
    let mut meta = vec![(format!("{PREFIX}maturity"), string(maturity))];
    if let Some(owner) = owner {
        meta.push((format!("{PREFIX}owner"), string(owner)));
    }
    object(vec![
        ("name", string(spec.name)),
        ("description", string(spec.summary)),
        (
            "inputSchema",
            if spec.name == "check" {
                check_schema()
            } else {
                object(vec![("type", string("object"))])
            },
        ),
        (
            "annotations",
            object(vec![
                ("readOnlyHint", Value::Bool(true)),
                ("destructiveHint", Value::Bool(false)),
                ("idempotentHint", Value::Bool(true)),
                ("openWorldHint", Value::Bool(false)),
            ]),
        ),
        ("_meta", Value::Object(meta)),
    ])
}

/// What `check` takes: the engine API's request, as JSON.
fn check_schema() -> Value {
    let text = |what: &str| {
        object(vec![
            ("type", string("string")),
            ("description", string(what)),
        ])
    };
    object(vec![
        ("type", string("object")),
        (
            "properties",
            object(vec![
                ("description", text("the eADL description's text")),
                (
                    "name",
                    text("how diagnostics name the description; `description.eadl` when absent"),
                ),
                (
                    "profile",
                    text("the profile's identifier; `rt-static-up-v1` when absent"),
                ),
                (
                    "modules",
                    object(vec![
                        ("type", string("object")),
                        (
                            "description",
                            string("each module the description imports: its name, and its text"),
                        ),
                        (
                            "additionalProperties",
                            object(vec![("type", string("string"))]),
                        ),
                    ]),
                ),
            ]),
        ),
        ("required", Value::Array(vec![string("description")])),
        ("additionalProperties", Value::Bool(false)),
    ])
}

/// `tools/call`'s result, or why its params are not a call (`-32602`).
fn call_tool(params: &Value) -> Result<Value, String> {
    let Some(name) = params.get("name").and_then(Value::as_str) else {
        return Err("`tools/call` names its tool in `name`".to_owned());
    };
    let Some(spec) = spec::tools().find(|spec| spec.name == name) else {
        return Err(format!("no tool `{name}`"));
    };
    let empty = Value::Object(Vec::new());
    let arguments = match params.get("arguments") {
        None => &empty,
        Some(arguments @ Value::Object(_)) => arguments,
        Some(_) => return Err("a tool's arguments are an object".to_owned()),
    };
    if spec.name != "check" {
        let owner = spec.maturity.owner().unwrap_or("its leaf");
        let text = format!(
            "`{}` is not built yet: task-tree leaf {owner} owns it (docs/TASK_TREE.md)",
            spec.name
        );
        return Ok(object(vec![
            ("content", Value::Array(vec![text_block(&text)])),
            ("isError", Value::Bool(true)),
        ]));
    }
    check(arguments)
}

/// `check`: the arguments read as the engine API's request, judged, and answered in the wasm binding's encoding.
fn check(arguments: &Value) -> Result<Value, String> {
    let Value::Object(members) = arguments else {
        return Err("`check`'s arguments are an object".to_owned());
    };
    for (key, _) in members {
        if !matches!(key.as_str(), "description" | "name" | "profile" | "modules") {
            return Err(format!("`check` takes no argument `{key}`"));
        }
    }
    let Some(text) = arguments.get("description").and_then(Value::as_str) else {
        return Err("`check` takes the description's text in `description`".to_owned());
    };
    let optional = |key: &str| match arguments.get(key) {
        None => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.as_str())),
        Some(_) => Err(format!("`check`'s `{key}` is a string")),
    };
    let name = optional("name")?.unwrap_or("description.eadl");
    let profile = optional("profile")?;
    let mut modules = MemoryModules::new();
    match arguments.get("modules") {
        None => {}
        Some(Value::Object(entries)) => {
            for (module, value) in entries {
                let Some(module_text) = value.as_str() else {
                    return Err(format!("module `{module}`'s text is a string"));
                };
                modules = modules.with(module, module_text);
            }
        }
        Some(_) => return Err("`check`'s `modules` maps each module's name to its text".to_owned()),
    }
    let response = check_with(
        &Request {
            name,
            text,
            profile,
            modules: &modules,
        },
        Limits::DEFAULT,
    );
    let encoded = archogen_wasm::json::encode(&response);
    // The encoding is JSON this reader admits, and reading it back gives a value the writer writes as the same
    // bytes (`encoded_responses_read_back_to_the_same_bytes`), so the structured result and the text block agree.
    let structured = json::read(encoded.as_bytes(), usize::MAX)
        .map_err(|refusal| format!("the response's encoding could not be read back: {refusal}"))?;
    Ok(object(vec![
        ("content", Value::Array(vec![text_block(&encoded)])),
        ("structuredContent", structured),
        ("isError", Value::Bool(response.status != Status::Ok)),
    ]))
}

fn text_block(text: &str) -> Value {
    object(vec![("type", string("text")), ("text", string(text))])
}

fn tools_capability() -> Value {
    object(vec![(
        "tools",
        object(vec![("listChanged", Value::Bool(false))]),
    )])
}

fn server_info() -> Value {
    object(vec![
        ("name", string("archogen")),
        ("version", string(env!("CARGO_PKG_VERSION"))),
    ])
}

fn result(id: &Value, result: Value) -> String {
    json::write(&object(vec![
        ("jsonrpc", string("2.0")),
        ("id", id.clone()),
        ("result", result),
    ]))
}

fn error(id: &Value, code: i64, message: &str, data: Option<Value>) -> String {
    let mut fields = vec![("code", integer(code)), ("message", string(message))];
    if let Some(data) = data {
        fields.push(("data", data));
    }
    json::write(&object(vec![
        ("jsonrpc", string("2.0")),
        ("id", id.clone()),
        ("error", object(fields)),
    ]))
}

fn object<K: Into<String>>(members: Vec<(K, Value)>) -> Value {
    Value::Object(
        members
            .into_iter()
            .map(|(key, value)| (key.into(), value))
            .collect(),
    )
}

fn string(text: &str) -> Value {
    Value::String(text.to_owned())
}

fn integer(value: i64) -> Value {
    Value::Number(Number::from(value))
}

#[cfg(test)]
mod tests {
    use super::*;

    const MODERN_META: &str = r#""_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}"#;
    const ACCEPTED: &str = "(defblock console.uart (offers observable-output))\n";

    /// Every answer is JSON this server's reader admits, and is the writer's own text for what it says.
    fn answer(server: &mut Server, line: &str) -> Option<Value> {
        let answer = server.answer(line.as_bytes())?;
        let value = json::read(answer.as_bytes(), usize::MAX).expect("every answer is JSON");
        assert_eq!(json::write(&value), answer, "an answer is written one way");
        Some(value)
    }

    fn modern(method: &str, extra: &str) -> String {
        format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"{method}","params":{{{MODERN_META}{extra}}}}}"#
        )
    }

    fn code(value: &Value) -> Option<i64> {
        value.get("error")?.get("code").and_then(|code| match code {
            Value::Number(number) => number.as_i64(),
            _ => None,
        })
    }

    fn check_call(arguments: &str) -> String {
        modern(
            "tools/call",
            &format!(r#","name":"check","arguments":{arguments}"#),
        )
    }

    #[test]
    fn server_discover_names_both_versions_and_the_tools_capability() {
        let value =
            answer(&mut Server::default(), &modern("server/discover", "")).expect("answered");
        let result = value.get("result").expect("a result");
        assert_eq!(
            json::write(result.get("supportedVersions").unwrap()),
            r#"["2026-07-28","2025-11-25"]"#
        );
        assert_eq!(
            json::write(result.get("capabilities").unwrap()),
            r#"{"tools":{"listChanged":false}}"#
        );
        assert_eq!(
            result.get("resultType").and_then(Value::as_str),
            Some("complete")
        );
        assert_eq!(
            result.get("cacheScope").and_then(Value::as_str),
            Some("public")
        );
        assert!(result.get("ttlMs").is_some());
        assert_eq!(
            json::write(result.get("_meta").unwrap()),
            r#"{"io.modelcontextprotocol/serverInfo":{"name":"archogen","version":"0.1.0"}}"#
        );
    }

    #[test]
    fn the_tool_list_is_the_command_tables_less_its_exclusions() {
        let value = answer(&mut Server::default(), &modern("tools/list", "")).expect("answered");
        let Some(Value::Array(tools)) = value.get("result").and_then(|result| result.get("tools"))
        else {
            panic!("no tool list: {}", json::write(&value));
        };
        let names: Vec<&str> = tools
            .iter()
            .filter_map(|tool| tool.get("name").and_then(Value::as_str))
            .collect();
        assert_eq!(names, ["check", "resolve", "analyze", "explain", "replay"]);
        for tool in tools {
            assert_eq!(
                json::write(tool.get("annotations").unwrap()),
                r#"{"readOnlyHint":true,"destructiveHint":false,"idempotentHint":true,"openWorldHint":false}"#
            );
            assert_eq!(
                tool.get("inputSchema")
                    .and_then(|s| s.get("type"))
                    .and_then(Value::as_str),
                Some("object")
            );
            let meta = tool.get("_meta").expect("a tool's _meta");
            assert!(meta.get("io.github.rdje.archogen/maturity").is_some());
        }
        assert_eq!(
            tools[1]
                .get("_meta")
                .and_then(|meta| meta.get("io.github.rdje.archogen/owner"))
                .and_then(Value::as_str),
            Some("M3.4")
        );
    }

    #[test]
    fn a_modern_request_missing_either_meta_key_or_naming_another_version_is_refused() {
        let mut server = Server::default();
        let only_version = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}"#;
        assert_eq!(
            code(&answer(&mut server, only_version).unwrap()),
            Some(-32602)
        );
        let only_capabilities = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/clientCapabilities":{}}}}"#;
        assert_eq!(
            code(&answer(&mut server, only_capabilities).unwrap()),
            Some(-32602)
        );
        let other = modern("tools/list", "").replace("2026-07-28", "2024-11-05");
        let refused = answer(&mut server, &other).unwrap();
        assert_eq!(code(&refused), Some(-32022));
        assert_eq!(
            json::write(refused.get("error").unwrap().get("data").unwrap()),
            r#"{"supported":["2026-07-28","2025-11-25"],"requested":"2024-11-05"}"#
        );
        assert_eq!(
            code(&answer(&mut server, &modern("ping", "")).unwrap()),
            Some(-32601),
            "2026-07-28 has no ping"
        );
    }

    #[test]
    fn the_earlier_revision_opens_with_initialize_and_answers_without_a_result_type() {
        let mut server = Server::default();
        let list = r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#;
        assert_eq!(
            code(&answer(&mut server, list).unwrap()),
            Some(-32602),
            "nothing before initialize"
        );
        let ping = answer(&mut server, r#"{"jsonrpc":"2.0","id":3,"method":"ping"}"#).unwrap();
        assert_eq!(
            json::write(ping.get("result").unwrap()),
            "{}",
            "a ping may precede initialize"
        );
        let bad = r#"{"jsonrpc":"2.0","id":4,"method":"initialize","params":{"protocolVersion":"2025-11-25"}}"#;
        assert_eq!(code(&answer(&mut server, bad).unwrap()), Some(-32602));
        let init = r#"{"jsonrpc":"2.0","id":5,"method":"initialize","params":{"protocolVersion":"2025-03-26","capabilities":{},"clientInfo":{"name":"c","version":"1"}}}"#;
        let opened = answer(&mut server, init).unwrap();
        assert_eq!(
            opened
                .get("result")
                .and_then(|r| r.get("protocolVersion"))
                .and_then(Value::as_str),
            Some(LEGACY)
        );
        assert_eq!(
            answer(
                &mut server,
                r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#
            ),
            None
        );
        let listed = answer(&mut server, list).unwrap();
        let result = listed.get("result").expect("listed after initialize");
        assert!(result.get("tools").is_some() && result.get("resultType").is_none());
    }

    #[test]
    fn a_message_that_is_no_request_is_refused_and_a_notification_is_never_answered() {
        let mut server = Server::default();
        for (line, expected) in [
            ("[1]", -32600),
            (r#"{"jsonrpc":"2.0","id":null,"method":"ping"}"#, -32600),
            (r#"{"jsonrpc":"1.0","id":1,"method":"ping"}"#, -32600),
            (r#"{"jsonrpc":"2.0","id":1}"#, -32600),
            (
                r#"{"jsonrpc":"2.0","id":1,"method":"ping","params":[1]}"#,
                -32602,
            ),
            (
                r#"{"jsonrpc":"2.0","id":1,"method":"resources/list"}"#,
                -32601,
            ),
            (
                "{\"jsonrpc\":\"2.0\",\"id\":1,\"id\":2,\"method\":\"ping\"}",
                -32700,
            ),
            ("", -32700),
        ] {
            assert_eq!(
                code(&answer(&mut server, line).unwrap()),
                Some(expected),
                "{line}"
            );
        }
        assert_eq!(
            answer(
                &mut server,
                r#"{"jsonrpc":"2.0","method":"notifications/cancelled"}"#
            ),
            None
        );
    }

    #[test]
    fn an_answer_keeps_the_requests_id_as_it_came() {
        let mut server = Server::default();
        for id in ["7", "\"seven\"", "1e2", "12345678901234567890"] {
            let line = format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"ping"}}"#);
            let answer = server.answer(line.as_bytes()).unwrap();
            assert!(answer.contains(&format!(r#""id":{id},"#)), "{answer}");
        }
    }

    #[test]
    fn check_answers_the_wasm_bindings_bytes_and_flags_anything_but_acceptance() {
        let mut server = Server::default();
        let accepted = answer(
            &mut server,
            &check_call(&format!(
                r#"{{"description":{}}}"#,
                json::write(&string(ACCEPTED))
            )),
        )
        .unwrap();
        let result = accepted.get("result").unwrap();
        assert_eq!(result.get("isError"), Some(&Value::Bool(false)));
        let text = result.get("content").and_then(|c| match c {
            Value::Array(blocks) => blocks[0].get("text").and_then(Value::as_str),
            _ => None,
        });
        assert_eq!(
            text,
            Some(json::write(result.get("structuredContent").unwrap()).as_str())
        );
        let expected = archogen_wasm::json::encode(&check_with(
            &Request {
                name: "description.eadl",
                text: ACCEPTED,
                profile: None,
                modules: &MemoryModules::new(),
            },
            Limits::DEFAULT,
        ));
        assert_eq!(
            text,
            Some(expected.as_str()),
            "the same bytes as the wasm binding"
        );

        let refused = answer(&mut server, &check_call(r#"{"description":"(defblock"}"#)).unwrap();
        assert_eq!(
            refused.get("result").unwrap().get("isError"),
            Some(&Value::Bool(true))
        );
        let profile = answer(
            &mut server,
            &check_call(&format!(
                r#"{{"description":{},"profile":"no-such-profile"}}"#,
                json::write(&string(ACCEPTED))
            )),
        )
        .unwrap();
        assert_eq!(
            profile.get("result").unwrap().get("isError"),
            Some(&Value::Bool(true))
        );
    }

    #[test]
    fn check_reads_its_modules_from_the_arguments() {
        let mut server = Server::default();
        let description = "(defmodule app.system (version 1 0) (import hw.board (as board)) (export s) (defsystem s (requires (uses board.serial))))\n";
        let module = "(defmodule hw.board (version 1 0) (export serial) (defservice serial (requires (observable-output))))\n";
        let without = answer(
            &mut server,
            &check_call(&format!(
                r#"{{"description":{}}}"#,
                json::write(&string(description))
            )),
        )
        .unwrap();
        let with = answer(
            &mut server,
            &check_call(&format!(
                r#"{{"description":{},"modules":{{"hw.board":{}}}}}"#,
                json::write(&string(description)),
                json::write(&string(module))
            )),
        )
        .unwrap();
        let status = |value: &Value| {
            value
                .get("result")
                .and_then(|r| r.get("structuredContent"))
                .and_then(|s| s.get("status"))
                .and_then(Value::as_str)
                .map(str::to_owned)
        };
        assert_ne!(
            status(&without),
            status(&with),
            "a module given changes the answer"
        );
        assert!(
            json::write(&without).contains("module-not-found"),
            "{}",
            json::write(&without)
        );
    }

    #[test]
    fn a_call_that_is_not_a_tools_call_is_a_protocol_error() {
        let mut server = Server::default();
        for (extra, why) in [
            (r#","name":"build","arguments":{}"#, "build is no tool"),
            (r#","name":"verify","arguments":{}"#, "verify is no tool"),
            (r#","name":"mcp","arguments":{}"#, "the server is no tool"),
            (r#","arguments":{}"#, "no name"),
            (r#","name":"check","arguments":{}"#, "no description"),
            (
                r#","name":"check","arguments":{"description":"x","path":"/etc/passwd"}"#,
                "an argument check does not take",
            ),
            (
                r#","name":"check","arguments":{"description":1}"#,
                "a description that is no string",
            ),
            (
                r#","name":"check","arguments":{"description":"x","modules":{"m":1}}"#,
                "a module that is no text",
            ),
            (
                r#","name":"check","arguments":[]"#,
                "arguments that are no object",
            ),
        ] {
            assert_eq!(
                code(&answer(&mut server, &modern("tools/call", extra)).unwrap()),
                Some(-32602),
                "{why}"
            );
        }
        assert_eq!(
            code(&answer(&mut server, &modern("tools/list", r#","cursor":"next""#)).unwrap()),
            Some(-32602)
        );
    }

    #[test]
    fn a_tool_that_is_not_built_answers_with_the_leaf_that_owns_it() {
        let value = answer(
            &mut Server::default(),
            &modern("tools/call", r#","name":"replay","arguments":{}"#),
        )
        .unwrap();
        let result = value.get("result").unwrap();
        assert_eq!(result.get("isError"), Some(&Value::Bool(true)));
        assert!(
            json::write(result).contains("task-tree leaf M4.7 owns it"),
            "{}",
            json::write(result)
        );
    }

    #[test]
    fn serve_answers_each_line_and_refuses_one_past_the_bound_unread() {
        let mut input = Vec::new();
        input.extend(std::iter::repeat_n(b' ', LINE_LIMIT + 1));
        input.push(b'\n');
        input.extend_from_slice(b"{\"jsonrpc\":\"2.0\",\"id\":9,\"method\":\"ping\"}\n");
        input.extend_from_slice(b"{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}");
        let mut output = Vec::new();
        serve(&mut io::Cursor::new(input), &mut output).expect("served");
        let text = String::from_utf8(output).expect("utf-8");
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(lines.len(), 2, "{text}");
        assert!(
            lines[0].contains("-32700") && lines[0].contains("was not read"),
            "{}",
            lines[0]
        );
        assert_eq!(lines[1], r#"{"jsonrpc":"2.0","id":9,"result":{}}"#);
    }

    #[test]
    fn a_line_at_the_bound_is_read() {
        let message = br#"{"jsonrpc":"2.0","id":9,"method":"ping"}"#;
        let mut input = vec![b' '; LINE_LIMIT - message.len()];
        input.extend_from_slice(message);
        input.push(b'\n');
        let mut output = Vec::new();
        serve(&mut io::Cursor::new(input), &mut output).expect("served");
        assert_eq!(
            String::from_utf8(output).unwrap(),
            "{\"jsonrpc\":\"2.0\",\"id\":9,\"result\":{}}\n"
        );
    }
}
