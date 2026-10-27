//! `archogen mcp` — the MCP server (leaf `API.6.4`), as `docs/decisions/decision_mcp-server.md` decides it: the
//! generator's tools over `archogen-transport`'s framing, which holds the protocol and serves each role's own tools
//! alike (leaf `M3.5.1.6.1`).
//!
//! One JSON-RPC 2.0 message per line on stdin, one answer per line on stdout, nothing else on stdout, and the
//! server returns when stdin ends. It speaks both protocol eras the record names:
//! - **`2026-07-28`**, per request: a request whose `_meta` carries `io.modelcontextprotocol/protocolVersion` and
//!   `io.modelcontextprotocol/clientCapabilities` is answered on its own, with `server/discover`, `tools/list` and
//!   `tools/call`; a request missing either is refused with `-32602`, another version with `-32022`.
//! - **`2025-11-25`**, after `initialize`, which selects it for the process: `tools/list`, `tools/call` and `ping`.
//!   A `ping` before `initialize` is answered too, a leniency: that revision makes `initialize` the first exchange
//!   and lets a client ping while it waits for the answer, and an early ping's answer costs nothing.
//!
//! The tools are the command table's ([`crate::spec::tools`]). `check` takes the engine API's request — the
//! description's text, its profile and its modules' texts — and answers the engine API's response in the wasm
//! binding's encoding, as `structuredContent` and as the one text block; `isError` is `false` only when the
//! description is accepted. A tool that is not built answers with the leaf that owns it. A call naming no tool, an
//! unknown tool or arguments that are no object is `-32602`; arguments `check` cannot use are a tool error,
//! `isError: true`, as both revisions' tools pages ask of "input validation errors", so a model can correct itself.
//!
//! An answer to a request whose id could not be read carries no id, which both revisions' schemas allow and neither
//! lets be `null`; a request's id is a string or an integral number.
//!
//! ⛔ It reads with [`crate::json`], under [`LINE_LIMIT`]; it spawns nothing and touches no file — it holds no
//! launcher (`crate::launch`), so it can make archogen start no program.

use std::io::{self, BufRead, Write};

use archogen_api::{
    check_with, Diagnostic, Explain, Limits, MemoryModules, Plan, Request, Resolve, Status,
};
use archogen_transport::{integer, object, string, text_block, tool_error, Failure, Tools};

use crate::json::{self, Value};
use crate::spec::{self, CommandSpec, Maturity};

pub use archogen_transport::{LEGACY, MODERN};

/// The prefix of the keys this server adds to a tool's `_meta` (the record's §2).
const PREFIX: &str = "io.github.rdje.archogen/";

/// The longest line read. A request carries texts under the engine API's byte budget, and a JSON writer may spell
/// any byte as six (`\u0001`), so the bound is six times the budget and a fixed allowance for the envelope: no text
/// the engine accepts is refused for how it was escaped. A longer line is refused with `-32700` and never parsed
/// (the record's §5).
pub const LINE_LIMIT: usize = 6 * archogen_api::DEFAULT_BYTES + 64 * 1024;

/// One sentence for a client to hand its model, in `server/discover` and `initialize`.
const INSTRUCTIONS: &str = "archogen checks eADL system descriptions: call `check` with a description's text, and \
read the verdict in the result's `status`; `resolve`, with `unchecked` set, searches one system for a plan that \
nothing has checked — compose it with archogen-plan-check's own tool; `explain`, with a fact as `requirement`, says \
what that search did with each requirement on it; the other tools are listed with the task-tree leaf that will build \
them.";

/// The generator's tools: the command table's ([`crate::spec::tools`]).
#[derive(Debug, Clone, Copy, Default)]
struct Engine;

impl Tools for Engine {
    fn name(&self) -> &str {
        "archogen"
    }

    fn version(&self) -> &str {
        crate::VERSION
    }

    fn instructions(&self) -> &str {
        INSTRUCTIONS
    }

    fn list(&self) -> Vec<Value> {
        spec::tools().map(tool).collect()
    }

    fn knows(&self, name: &str) -> bool {
        spec::tools().any(|spec| spec.name == name)
    }

    fn call(&self, name: &str, arguments: &Value) -> Result<Value, Failure> {
        let Some(spec) = spec::tools().find(|spec| spec.name == name) else {
            return Err(Failure::Params(format!("no tool `{name}`")));
        };
        // A tool with a handler runs whatever its maturity — `resolve` is experimental, and runs (leaf `M3.4.6`).
        match (&spec.maturity, spec.name) {
            (_, "check") => check(arguments),
            (_, "resolve") => resolve(arguments),
            (_, "explain") => explain(arguments),
            (Maturity::Built, name) => Err(Failure::Internal(format!(
                "`{name}` is marked built and this server has no handler for it"
            ))),
            (maturity, name) => {
                let owner = maturity.owner().unwrap_or("its leaf");
                Ok(tool_error(&format!(
                    "`{name}` is not built yet: task-tree leaf {owner} owns it (docs/TASK_TREE.md)"
                )))
            }
        }
    }
}

/// The server's state: whether `initialize` has selected the earlier revision for this process.
pub struct Server(archogen_transport::Server<'static>);

impl Default for Server {
    fn default() -> Self {
        Self(archogen_transport::Server::new(&Engine, LINE_LIMIT))
    }
}

impl Server {
    /// The answer to one line, or `None` for a notification.
    pub fn answer(&mut self, line: &[u8]) -> Option<String> {
        self.0.answer(line)
    }
}

/// Serve `input` until it ends, answering each line on `output`.
///
/// # Errors
///
/// The streams' own failure.
pub fn serve(input: &mut dyn BufRead, output: &mut dyn Write) -> io::Result<()> {
    archogen_transport::serve(input, output, &Engine, LINE_LIMIT)
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
    // A model reads a tool's description; few clients show it `_meta`. So what a tool is says so where it is read: an
    // unimplemented one that it is not built yet, which a call of it answers too, and an experimental one what it does
    // today (`resolve`, leaf `M3.4.6`).
    let description = match spec.maturity {
        Maturity::Unimplemented { owner } => {
            format!(
                "{} — not built yet: task-tree leaf {owner} owns it",
                spec.summary
            )
        }
        Maturity::Experimental {
            scope,
            completed_by,
        } => format!(
            "{} — experimental: {scope}; task-tree leaf {completed_by} completes it",
            spec.summary
        ),
        Maturity::Built => spec.summary.to_owned(),
    };
    object(vec![
        ("name", string(spec.name)),
        ("description", Value::String(description)),
        (
            "inputSchema",
            match spec.name {
                "check" => check_schema(),
                "resolve" => resolve_schema(),
                "explain" => explain_schema(),
                _ => object(vec![("type", string("object"))]),
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

/// `check`: the arguments read as the engine API's request, judged, and answered in the wasm binding's encoding.
/// An argument it cannot use is a tool error, with what to change.
fn check(arguments: &Value) -> Result<Value, Failure> {
    match check_request(arguments, &[]) {
        Ok((name, text, profile, modules)) => judge(name, text, profile, &modules),
        Err(problem) => Ok(tool_error(&format!(
            "{problem}; `check` takes `description` (the text), and optionally `name`, `profile` and `modules`, \
             each module's name mapped to its text"
        ))),
    }
}

/// `check`'s arguments — and `extra`, which a tool taking them adds — read, or what is wrong with them.
fn check_request<'a>(
    arguments: &'a Value,
    extra: &[&str],
) -> Result<(&'a str, &'a str, Option<&'a str>, MemoryModules), String> {
    let Value::Object(members) = arguments else {
        return Err("the arguments are not an object".to_owned());
    };
    for (key, _) in members {
        if !matches!(key.as_str(), "description" | "name" | "profile" | "modules")
            && !extra.contains(&key.as_str())
        {
            return Err(format!("there is no argument `{key}`"));
        }
    }
    let Some(text) = arguments.get("description").and_then(Value::as_str) else {
        return Err("`description`, the description's text, is missing or not a string".to_owned());
    };
    let optional = |key: &str| match arguments.get(key) {
        None => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.as_str())),
        Some(_) => Err(format!("`{key}` is not a string")),
    };
    let name = optional("name")?.unwrap_or("description.eadl");
    let profile = optional("profile")?;
    let mut modules = MemoryModules::new();
    match arguments.get("modules") {
        None => {}
        Some(Value::Object(entries)) => {
            for (module, value) in entries {
                let Some(module_text) = value.as_str() else {
                    return Err(format!("module `{module}`'s text is not a string"));
                };
                modules = modules.with(module, module_text);
            }
        }
        Some(_) => return Err("`modules` is not an object".to_owned()),
    }
    Ok((name, text, profile, modules))
}

/// What `resolve` takes: `check`'s arguments, the system to resolve, and `unchecked`, which the engine API requires.
fn resolve_schema() -> Value {
    let mut schema = check_schema();
    if let Value::Object(members) = &mut schema {
        for (key, value) in members.iter_mut() {
            match (key.as_str(), value) {
                ("properties", Value::Object(properties)) => {
                    properties.push((
                        "system".to_owned(),
                        object(vec![
                            ("type", string("string")),
                            (
                                "description",
                                string(
                                    "the system to resolve, when the description declares several",
                                ),
                            ),
                        ]),
                    ));
                    properties.push((
                        "unchecked".to_owned(),
                        object(vec![
                            ("type", string("boolean")),
                            (
                                "description",
                                string(
                                    "must be true: this server runs no checker, so it answers the search alone, a \
                                     plan nothing has checked — compose it with archogen-plan-check's own tool",
                                ),
                            ),
                        ]),
                    ));
                }
                ("required", required) => {
                    *required = Value::Array(vec![string("description"), string("unchecked")]);
                }
                _ => {}
            }
        }
    }
    schema
}

/// `resolve` (leaf `M3.4.6`): the arguments read as the engine API's request and `Resolve`, asked of
/// [`archogen_api::resolve_with`] under the default budget, and answered as `{"response": …, "plan": …}` — the response
/// in the wasm binding's encoding, the plan its system, text, program digest and bindings, or `null` — that JSON the one
/// text block, and `isError` unless the response is `ok`. Without `unchecked` the API refuses, with its reason and repair.
fn resolve(arguments: &Value) -> Result<Value, Failure> {
    let read = check_request(arguments, &["system", "unchecked"]).and_then(
        |(name, text, profile, modules)| {
            let system = match arguments.get("system") {
                None => None,
                Some(Value::String(system)) => Some(system.as_str()),
                Some(_) => return Err("`system` is not a string".to_owned()),
            };
            let unchecked = match arguments.get("unchecked") {
                None => false,
                Some(Value::Bool(unchecked)) => *unchecked,
                Some(_) => return Err("`unchecked` is not a boolean".to_owned()),
            };
            Ok((name, text, profile, modules, system, unchecked))
        },
    );
    let (name, text, profile, modules, system, unchecked) = match read {
        Ok(read) => read,
        Err(problem) => {
            return Ok(tool_error(&format!(
                "{problem}; `resolve` takes `description` (the text) and `unchecked`, and optionally `name`, \
                 `profile`, `system` and `modules`, each module's name mapped to its text"
            )))
        }
    };
    let resolution = archogen_api::resolve_with(
        &Request {
            name,
            text,
            profile,
            modules: &modules,
        },
        &Resolve { system, unchecked },
        Limits::DEFAULT,
    );
    let encoded = archogen_wasm::json::encode(&resolution.response);
    let response = json::read(encoded.as_bytes(), usize::MAX).map_err(|refusal| {
        Failure::Internal(format!(
            "the response's encoding could not be read back: {refusal}"
        ))
    })?;
    let structured = object(vec![
        ("response", response),
        ("plan", plan_value(resolution.plan.as_ref())),
    ]);
    let written = json::write(&structured);
    Ok(object(vec![
        ("content", Value::Array(vec![text_block(&written)])),
        ("structuredContent", structured),
        (
            "isError",
            Value::Bool(resolution.response.status != Status::Ok),
        ),
    ]))
}

/// What `explain` takes: `check`'s arguments, the fact whose requirements to explain, whose, and the system.
fn explain_schema() -> Value {
    let text = |what: &str| {
        object(vec![
            ("type", string("string")),
            ("description", string(what)),
        ])
    };
    let mut schema = check_schema();
    if let Value::Object(members) = &mut schema {
        for (key, value) in members.iter_mut() {
            match (key.as_str(), value) {
                ("properties", Value::Object(properties)) => {
                    for (name, what) in [
                        ("requirement", "the fact whose requirements to explain, e.g. observable-output"),
                        (
                            "requester",
                            "only this declaration's requirement on it: a service, a policy, the system's own",
                        ),
                        ("task", "only this task's requirement on it"),
                        ("system", "the system to explain, when the description declares several"),
                    ] {
                        properties.push((name.to_owned(), text(what)));
                    }
                }
                ("required", required) => {
                    *required = Value::Array(vec![string("description"), string("requirement")]);
                }
                _ => {}
            }
        }
    }
    schema
}

/// `explain` (leaf `M3.4.13.2`): the arguments read as the engine API's request and `Explain`, asked of
/// [`archogen_api::explain_with`] under the default budget, and answered as `{"response": …, "plan": …, "accounts":
/// […]}` — the response in the wasm binding's encoding, the plan as `resolve` gives it, the search's and unchecked, and
/// each account its requester, fact, clauses, each provider's answer, why none meets it, the provider bound and each
/// other one's refusal in its place — that JSON the one text block, and `isError` unless the response is `ok`.
fn explain(arguments: &Value) -> Result<Value, Failure> {
    let read = check_request(arguments, &["requirement", "requester", "task", "system"]).and_then(
        |(name, text, profile, modules)| {
            let optional = |key: &str| match arguments.get(key) {
                None => Ok(None),
                Some(Value::String(value)) => Ok(Some(value.as_str())),
                Some(_) => Err(format!("`{key}` is not a string")),
            };
            let Some(fact) = optional("requirement")? else {
                return Err(
                    "`requirement`, the fact whose requirements to explain, is missing".to_owned(),
                );
            };
            Ok((
                name,
                text,
                profile,
                modules,
                Explain {
                    system: optional("system")?,
                    fact,
                    requester: optional("requester")?,
                    task: optional("task")?,
                },
            ))
        },
    );
    let (name, text, profile, modules, asked) = match read {
        Ok(read) => read,
        Err(problem) => {
            return Ok(tool_error(&format!(
                "{problem}; `explain` takes `description` (the text) and `requirement` (a fact), and optionally \
                 `requester` or `task`, `system`, `name`, `profile` and `modules`, each module's name mapped to its text"
            )))
        }
    };
    let explanation = archogen_api::explain_with(
        &Request {
            name,
            text,
            profile,
            modules: &modules,
        },
        &asked,
        Limits::DEFAULT,
    );
    let encoded = archogen_wasm::json::encode(&explanation.response);
    let response = json::read(encoded.as_bytes(), usize::MAX).map_err(|refusal| {
        Failure::Internal(format!(
            "the response's encoding could not be read back: {refusal}"
        ))
    })?;
    let sources = &explanation.response.sources;
    let finding = |diagnostic: Option<&Diagnostic>| {
        diagnostic.map_or(Value::Null, |diagnostic| {
            object(vec![
                ("code", string(diagnostic.code)),
                ("message", string(&diagnostic.message)),
            ])
        })
    };
    let accounts = explanation
        .accounts
        .iter()
        .map(|account| {
            object(vec![
                ("requester", string(&account.requester)),
                ("fact", string(&account.fact)),
                (
                    "clauses",
                    Value::Array(
                        account
                            .clauses
                            .iter()
                            .filter_map(|span| {
                                let source = sources.get(span.source)?;
                                let at = source.position(span.start);
                                Some(object(vec![
                                    ("name", string(&source.name)),
                                    ("line", integer(i64::from(at.line))),
                                    ("column", integer(i64::from(at.column))),
                                ]))
                            })
                            .collect(),
                    ),
                ),
                (
                    "answers",
                    Value::Array(
                        account
                            .answers
                            .iter()
                            .map(|(provider, answer)| {
                                object(vec![
                                    ("provider", string(provider)),
                                    ("answer", string(answer.slug())),
                                ])
                            })
                            .collect(),
                    ),
                ),
                ("unmet", finding(account.unmet.as_ref())),
                (
                    "bound",
                    account.bound.as_deref().map_or(Value::Null, string),
                ),
                (
                    "instead",
                    Value::Array(
                        account
                            .instead
                            .iter()
                            .map(|(provider, refusal)| {
                                object(vec![
                                    ("provider", string(provider)),
                                    ("refusal", finding(refusal.as_ref())),
                                ])
                            })
                            .collect(),
                    ),
                ),
            ])
        })
        .collect();
    let structured = object(vec![
        ("response", response),
        ("plan", plan_value(explanation.plan.as_ref())),
        ("accounts", Value::Array(accounts)),
    ]);
    let written = json::write(&structured);
    Ok(object(vec![
        ("content", Value::Array(vec![text_block(&written)])),
        ("structuredContent", structured),
        (
            "isError",
            Value::Bool(explanation.response.status != Status::Ok),
        ),
    ]))
}

/// A plan as `resolve` and `explain` answer it: its system, text, program digest and bindings, or `null`.
fn plan_value(plan: Option<&Plan>) -> Value {
    match plan {
        None => Value::Null,
        Some(plan) => object(vec![
            ("system", string(&plan.system)),
            ("text", string(&plan.text)),
            ("program", string(&plan.program)),
            (
                "bindings",
                integer(i64::try_from(plan.bindings).unwrap_or(i64::MAX)),
            ),
        ]),
    }
}

/// The engine API's judgement of one request, in the wasm binding's encoding.
fn judge(
    name: &str,
    text: &str,
    profile: Option<&str>,
    modules: &MemoryModules,
) -> Result<Value, Failure> {
    let response = check_with(
        &Request {
            name,
            text,
            profile,
            modules,
        },
        Limits::DEFAULT,
    );
    let encoded = archogen_wasm::json::encode(&response);
    // The encoding is JSON this reader admits, and reading it back gives a value the writer writes as the same
    // bytes (`encoded_responses_read_back_to_the_same_bytes`), so the structured result and the text block agree.
    let structured = json::read(encoded.as_bytes(), usize::MAX).map_err(|refusal| {
        Failure::Internal(format!(
            "the response's encoding could not be read back: {refusal}"
        ))
    })?;
    Ok(object(vec![
        ("content", Value::Array(vec![text_block(&encoded)])),
        ("structuredContent", structured),
        ("isError", Value::Bool(response.status != Status::Ok)),
    ]))
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

    /// A plain description of one system, which the search resolves to one binding.
    const ONE_SYSTEM: &str = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write)) (platform (uses soc)))
";

    fn resolve_call(arguments: &str) -> String {
        modern(
            "tools/call",
            &format!(r#","name":"resolve","arguments":{arguments}"#),
        )
    }

    fn explain_call(arguments: &str) -> String {
        modern(
            "tools/call",
            &format!(r#","name":"explain","arguments":{arguments}"#),
        )
    }

    #[test]
    fn explain_answers_each_account_and_the_search_s_plan() {
        let description = json::write(&Value::String(ONE_SYSTEM.to_owned()));
        let value = answer(
            &mut Server::default(),
            &explain_call(&format!(
                r#"{{"description":{description},"requirement":"observable-output"}}"#
            )),
        )
        .expect("answered");
        let result = value.get("result").expect("a result");
        assert_eq!(result.get("isError"), Some(&Value::Bool(false)));
        let structured = result.get("structuredContent").expect("structured content");
        let text = match result.get("content") {
            Some(Value::Array(blocks)) => blocks[0].get("text").and_then(Value::as_str).unwrap(),
            _ => panic!("one text block"),
        };
        assert_eq!(
            json::write(structured),
            text,
            "the text block is the structured content"
        );
        assert!(structured.get("plan").and_then(|p| p.get("text")).is_some());
        let Some(Value::Array(accounts)) = structured.get("accounts") else {
            panic!("accounts");
        };
        assert_eq!(
            json::write(&accounts[0]),
            r#"{"requester":"console.write","fact":"observable-output","clauses":[{"name":"description.eadl","line":4,"column":44}],"answers":[{"provider":"console.uart","answer":"satisfied"},{"provider":"soc","answer":"undescribed"}],"unmet":null,"bound":"console.uart","instead":[]}"#
        );
        // A fact nothing requires is the API's usage refusal, no account; a missing `requirement` a tool error.
        let value = answer(
            &mut Server::default(),
            &explain_call(&format!(
                r#"{{"description":{description},"requirement":"absolute-deadline"}}"#
            )),
        )
        .expect("answered");
        let result = value.get("result").expect("a result");
        assert_eq!(result.get("isError"), Some(&Value::Bool(true)));
        let structured = result.get("structuredContent").expect("structured content");
        assert_eq!(
            structured
                .get("response")
                .and_then(|r| r.get("status"))
                .and_then(Value::as_str),
            Some("usage")
        );
        assert_eq!(json::write(structured.get("accounts").unwrap()), "[]");
        let value = answer(
            &mut Server::default(),
            &explain_call(&format!(r#"{{"description":{description}}}"#)),
        )
        .expect("answered");
        let result = value.get("result").expect("a result");
        assert_eq!(result.get("isError"), Some(&Value::Bool(true)));
        assert!(json::write(result).contains("`requirement`"));
    }

    #[test]
    fn resolve_answers_the_plan_with_unchecked_set_and_the_api_s_refusal_without_it() {
        let description = json::write(&Value::String(ONE_SYSTEM.to_owned()));
        let value = answer(
            &mut Server::default(),
            &resolve_call(&format!(
                r#"{{"description":{description},"unchecked":true}}"#
            )),
        )
        .expect("answered");
        let result = value.get("result").expect("a result");
        assert_eq!(result.get("isError"), Some(&Value::Bool(false)));
        let structured = result.get("structuredContent").expect("structured content");
        let text = match result.get("content") {
            Some(Value::Array(blocks)) => blocks[0].get("text").and_then(Value::as_str).unwrap(),
            _ => panic!("one text block"),
        };
        assert_eq!(
            json::write(structured),
            text,
            "the text block is the structured content"
        );
        assert_eq!(
            structured
                .get("response")
                .and_then(|r| r.get("status"))
                .and_then(Value::as_str),
            Some("ok")
        );
        let plan = structured.get("plan").expect("a plan");
        assert_eq!(plan.get("system").and_then(Value::as_str), Some("s"));
        assert!(plan.get("text").and_then(Value::as_str).is_some_and(
            |text| text.ends_with("(bind console.write observable-output console.uart)\n")
        ));
        // Without `unchecked`, or with it false, the engine API refuses, and says how to have the plan checked.
        for arguments in [
            format!(r#"{{"description":{description}}}"#),
            format!(r#"{{"description":{description},"unchecked":false}}"#),
        ] {
            let value =
                answer(&mut Server::default(), &resolve_call(&arguments)).expect("answered");
            let result = value.get("result").expect("a result");
            assert_eq!(
                result.get("isError"),
                Some(&Value::Bool(true)),
                "{arguments}"
            );
            let structured = result.get("structuredContent").expect("structured content");
            assert_eq!(structured.get("plan"), Some(&Value::Null), "{arguments}");
            let response = structured.get("response").expect("the response");
            assert_eq!(
                response.get("status").and_then(Value::as_str),
                Some("usage")
            );
            assert!(
                json::write(response).contains("archogen-plan-check"),
                "{arguments}"
            );
        }
        // An argument it cannot use is a tool error saying what it takes.
        let value = answer(
            &mut Server::default(),
            &resolve_call(&format!(
                r#"{{"description":{description},"unchecked":"yes"}}"#
            )),
        )
        .expect("answered");
        assert!(json::write(&value).contains("`unchecked` is not a boolean"));
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
        assert_eq!(names, ["check", "resolve", "explain", "replay"]);
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
            Some("M3.7")
        );
        assert_eq!(
            tools[1]
                .get("_meta")
                .and_then(|meta| meta.get("io.github.rdje.archogen/maturity"))
                .and_then(Value::as_str),
            Some("experimental"),
            "`resolve` runs over the description's own providers, and the catalog's are `M3.7`'s"
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
        // Review of `API.6.5`, D4 and D5: an answer to a request whose id could not be read carries no id, never
        // `null`; and a request's id is a string or an integral number.
        for line in [
            "[1]",
            "{",
            r#"{"jsonrpc":"2.0","id":1.5,"method":"ping"}"#,
            r#"{"jsonrpc":"2.0","id":null,"method":"ping"}"#,
        ] {
            let value = answer(&mut server, line).unwrap();
            assert!(value.get("id").is_none(), "{line}: {}", json::write(&value));
            assert!(code(&value).is_some(), "{line}");
        }
        // A response the client sends is to no request of the server's, and is not answered.
        assert_eq!(
            answer(&mut server, r#"{"jsonrpc":"2.0","id":5,"result":{}}"#),
            None
        );
        assert_eq!(
            answer(
                &mut server,
                r#"{"jsonrpc":"2.0","id":5,"error":{"code":1,"message":"m"}}"#
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
    fn a_call_that_is_no_call_is_a_protocol_error() {
        // The tools pages' protocol errors: an unknown tool, and a request that fails `CallToolRequest`'s schema.
        let mut server = Server::default();
        for (extra, why) in [
            (r#","name":"build","arguments":{}"#, "build is no tool"),
            (r#","name":"verify","arguments":{}"#, "verify is no tool"),
            (r#","name":"mcp","arguments":{}"#, "the server is no tool"),
            (r#","arguments":{}"#, "no name"),
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

    /// Review of `API.6.5`, D6: an argument `check` cannot use is a tool error, `isError: true`, with what to change —
    /// the tools pages' "input validation errors", which a model can correct — not a protocol error.
    #[test]
    fn an_argument_check_cannot_use_is_a_tool_error_a_model_can_correct() {
        let mut server = Server::default();
        for (arguments, says) in [
            ("{}", "`description`"),
            (
                r#"{"description":"x","path":"/etc/passwd"}"#,
                "no argument `path`",
            ),
            (r#"{"description":1}"#, "`description`"),
            (
                r#"{"description":"x","profile":7}"#,
                "`profile` is not a string",
            ),
            (r#"{"description":"x","modules":{"m":1}}"#, "module `m`"),
            (
                r#"{"description":"x","modules":[]}"#,
                "`modules` is not an object",
            ),
        ] {
            let value = answer(&mut server, &check_call(arguments)).unwrap();
            assert_eq!(code(&value), None, "{arguments}: {}", json::write(&value));
            let result = value.get("result").expect("a result");
            assert_eq!(
                result.get("isError"),
                Some(&Value::Bool(true)),
                "{arguments}"
            );
            assert!(
                json::write(result).contains(says),
                "{arguments}: {}",
                json::write(result)
            );
        }
    }

    #[test]
    fn every_built_tool_has_a_handler() {
        let mut server = Server::default();
        for spec in spec::tools().filter(|spec| matches!(spec.maturity, Maturity::Built)) {
            let call = modern(
                "tools/call",
                &format!(r#","name":"{}","arguments":{{}}"#, spec.name),
            );
            let value = answer(&mut server, &call).unwrap();
            assert_eq!(
                code(&value),
                None,
                "`{}`: {}",
                spec.name,
                json::write(&value)
            );
            assert!(
                !json::write(&value).contains("not built yet"),
                "`{}` is built",
                spec.name
            );
        }
    }

    /// Review of `API.6.5`, D7: the structured result and the text block are the same bytes — the wasm binding's
    /// encoding read back and written again — over every single-file case of the conformance suite and texts whose
    /// escaping is easy to get wrong.
    #[test]
    fn encoded_responses_read_back_to_the_same_bytes() {
        let cases =
            std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/semantics/cases");
        let mut texts: Vec<String> = std::fs::read_dir(&cases)
            .expect("the cases directory")
            .map(|entry| entry.expect("an entry").path())
            .filter(|path| path.extension().is_some_and(|ext| ext == "eadl"))
            .map(|path| std::fs::read_to_string(path).expect("a case reads"))
            .collect();
        assert!(texts.len() >= 30, "the suite: {} cases", texts.len());
        texts.extend(
            [
                "(defblock a\u{1}b (offers x))\n",
                "(defblock é😀 (offers \"quoted\\\" \u{2028} \u{85}\"))\n",
                "(defblock\t(\r\n",
                "",
            ]
            .map(str::to_owned),
        );
        for text in &texts {
            let encoded = archogen_wasm::json::encode(&check_with(
                &Request {
                    name: "case.eadl",
                    text,
                    profile: None,
                    modules: &MemoryModules::new(),
                },
                Limits::DEFAULT,
            ));
            let read = json::read(encoded.as_bytes(), usize::MAX).expect("the encoding is JSON");
            assert_eq!(json::write(&read), encoded, "{text:?}");
        }
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

#[cfg(test)]
mod review_tests {
    //! The review of `API.6.5`: R5, R6 and R8, each a message that said less than it should.
    use super::*;

    fn answer(server: &mut Server, line: &str) -> Value {
        let text = server.answer(line.as_bytes()).expect("answered");
        json::read(text.as_bytes(), usize::MAX).expect("JSON")
    }

    fn message(value: &Value) -> String {
        value
            .get("error")
            .and_then(|error| error.get("message"))
            .and_then(Value::as_str)
            .unwrap_or_default()
            .to_owned()
    }

    #[test]
    fn a_version_that_is_present_but_no_string_is_named_so() {
        let line = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":5,"io.modelcontextprotocol/clientCapabilities":{}}}}"#;
        let said = message(&answer(&mut Server::default(), line));
        assert!(said.contains("is not a string"), "{said}");
    }

    #[test]
    fn after_initialize_a_method_of_the_other_revision_is_not_found_and_says_why() {
        let mut server = Server::default();
        answer(
            &mut server,
            r#"{"jsonrpc":"2.0","id":1,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"c","version":"1"}}}"#,
        );
        let value = answer(
            &mut server,
            r#"{"jsonrpc":"2.0","id":2,"method":"server/discover"}"#,
        );
        let code = value
            .get("error")
            .and_then(|e| e.get("code"))
            .map(json::write);
        assert_eq!(code.as_deref(), Some("-32601"));
        assert!(
            message(&value).contains("2025-11-25"),
            "{}",
            message(&value)
        );
    }

    #[test]
    fn an_unbuilt_tools_description_says_so_where_a_model_reads_it() {
        let line = r#"{"jsonrpc":"2.0","id":1,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}}}"#;
        let value = answer(&mut Server::default(), line);
        let Some(Value::Array(tools)) = value.get("result").and_then(|result| result.get("tools"))
        else {
            panic!("a tool list");
        };
        for tool in tools {
            let name = tool.get("name").and_then(Value::as_str).unwrap_or_default();
            let description = tool
                .get("description")
                .and_then(Value::as_str)
                .unwrap_or_default();
            // Not built yet exactly where a call answers so; an experimental tool says what it does today.
            assert_eq!(
                description.contains("not built yet"),
                !matches!(name, "check" | "resolve" | "explain"),
                "{name}: {description}"
            );
            assert_eq!(
                description.contains("experimental:"),
                matches!(name, "resolve" | "explain"),
                "{name}: {description}"
            );
        }
    }
}

#[cfg(test)]
mod bound_tests {
    use super::*;

    /// Review of `API.6.5`, D2: a description under the API's budget, written by a client that escapes every
    /// non-ASCII character (`é` as six bytes, as Python's `json.dumps` does), passed twice the budget and was refused
    /// at the line. Now the line bound allows any escaping of a text the engine accepts.
    #[test]
    fn a_description_the_engine_accepts_is_not_refused_for_how_it_was_escaped() {
        let body = "é".repeat(400_000);
        let escaped = "\\u00e9".repeat(400_000);
        assert!(
            "(defblock console.uart (offers observable-output))\n;".len() + body.len() + 1
                < archogen_api::DEFAULT_BYTES
        );
        let line = format!(
            r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"_meta":{{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{{}}}},"name":"check","arguments":{{"description":"(defblock console.uart (offers observable-output))\n;{escaped}\n"}}}}}}"#
        );
        assert!(
            line.len() > 2 * archogen_api::DEFAULT_BYTES + 64 * 1024,
            "the old bound refused it"
        );
        let mut output = Vec::new();
        serve(&mut io::Cursor::new(format!("{line}\n")), &mut output).expect("served");
        let text = String::from_utf8(output).expect("utf-8");
        assert!(
            text.contains(r#""isError":false"#),
            "{}",
            &text[..text.len().min(300)]
        );
    }
}
