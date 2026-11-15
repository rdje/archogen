//! The configuration checker's MCP server (leaf `M3.5.1.6.4`; `M3.5.1`'s design, *The programmatic surface, named for
//! what it does*): its one tool, `plan-check`, over `archogen-transport`'s framing, which the generator's server
//! shares — one JSON-RPC message per line, both protocol eras, the envelopes. The executable serves it as
//! `archogen-plan-check --mcp`.
//!
//! `plan-check` takes texts, never paths: the description's, each module's it imports and the plan's, and optionally the
//! target's file the plan was resolved on, `target`, `{ "text", "name" }` as the generator's server takes it (leaf
//! `M4.1.7.5.4.4`), judged on it as the executable's `--target` is — all under this checker's own byte budget,
//! [`judgement::BYTES`], which the browser module counts alike. It answers the
//! result record the executable writes for the same texts ([`judgement::judge`]): the record's bytes as the one text
//! block, the same JSON as `structuredContent`, and `isError` unless its status is `ok`, so a client that reads nothing
//! but `isError` never takes a refusal for a pass. An argument it cannot use, a profile it does not have and texts past
//! the budget are a tool error with what to change, nothing judged — as the executable's usage error writes no record.
//!
//! ⛔ It spawns nothing and touches no file: its modules are the texts it was given ([`MemoryModules`]).

use std::io::{self, BufRead, Write};

use archogen_json::{self as json, Value};
use archogen_outcome::record::Producer;
use archogen_outcome::Status;
use archogen_transport::{object, string, text_block, tool_error, Failure, Tools};
use eadl_front::module::MemoryModules;
use eadl_model::profile::Profile;

use crate::judgement::{self, Inputs, TargetFile, Unjudged, BYTES, PLAN_NAME};

/// The tool's name.
pub const TOOL: &str = "plan-check";

/// The longest line read: a JSON writer may spell any byte as six (`\u0001`), so six times the budget and a fixed
/// allowance for the envelope — no text within the budget is refused for how it was escaped. A longer line is refused
/// with `-32700` and never parsed.
pub const LINE_LIMIT: usize = 6 * BYTES + 64 * 1024;

/// How findings name the description when the call names it not.
const DESCRIPTION_NAME: &str = "description.eadl";

/// How findings name a target's file when the call names it not.
const TARGET_NAME: &str = "target.eadl";

/// One sentence for a client to hand its model, in `server/discover` and `initialize`.
const INSTRUCTIONS: &str = "archogen-plan-check checks a resolved plan against its eADL description, apart from the \
search that wrote it: call `plan-check` with the description's text and the plan's, and read the verdict in the \
result record's `status`.";

/// The configuration checker's tools: `plan-check`, its records naming `producer` as the program that ran.
#[derive(Debug, Clone)]
pub struct Checker {
    producer: Producer,
}

impl Checker {
    /// The tools of the program `producer` names: the executable's name and the sha256 of the file that runs.
    #[must_use]
    pub const fn new(producer: Producer) -> Self {
        Self { producer }
    }
}

impl Tools for Checker {
    fn name(&self) -> &str {
        crate::NAME
    }

    fn version(&self) -> &str {
        crate::VERSION
    }

    fn instructions(&self) -> &str {
        INSTRUCTIONS
    }

    fn list(&self) -> Vec<Value> {
        vec![tool()]
    }

    fn knows(&self, name: &str) -> bool {
        name == TOOL
    }

    fn call(&self, name: &str, arguments: &Value) -> Result<Value, Failure> {
        if name != TOOL {
            return Err(Failure::Params(format!("no tool `{name}`")));
        }
        match request(arguments) {
            Ok(request) => self.judge(&request),
            Err(problem) => Ok(tool_error(&format!(
                "{problem}; `{TOOL}` takes `description` and `plan` (the texts), and optionally `name`, `profile`, \
                 `modules`, each module's name mapped to its text, and `target`, the target's file the plan was \
                 resolved on, `{{\"text\": …, \"name\": …}}`, its name optional"
            ))),
        }
    }
}

impl Checker {
    /// The record `request` makes, answered inline.
    fn judge(&self, request: &Request<'_>) -> Result<Value, Failure> {
        let inputs = Inputs {
            description_name: request.name,
            description: request.description,
            modules: &request.modules,
            profile: request.profile,
            plan_name: PLAN_NAME,
            plan: request.plan.as_bytes(),
        };
        let record = match judgement::judge_on(self.producer.clone(), &inputs, request.target) {
            Ok(record) => record,
            Err(Unjudged::TooLarge(error)) => {
                return Ok(tool_error(&format!("a text is too large to read: {error}")))
            }
            Err(Unjudged::Record(refusal)) => {
                return Err(Failure::Internal(format!(
                    "its own result does not make a record: {refusal}"
                )))
            }
        };
        let written = record.write();
        // The record's writer writes JSON the strict reader admits, so the structured result and the text block are
        // one record: `Record::read` reads either back.
        let structured = json::read(written.as_bytes(), usize::MAX).map_err(|refusal| {
            Failure::Internal(format!("the record could not be read back: {refusal}"))
        })?;
        Ok(object(vec![
            ("content", Value::Array(vec![text_block(&written)])),
            ("structuredContent", structured),
            ("isError", Value::Bool(record.status() != Status::Ok)),
        ]))
    }
}

/// Serve `input` until it ends, answering each line on `output`.
///
/// # Errors
///
/// The streams' own failure.
pub fn serve(input: &mut dyn BufRead, output: &mut dyn Write, checker: &Checker) -> io::Result<()> {
    archogen_transport::serve(input, output, checker, LINE_LIMIT)
}

/// A call's arguments, read.
struct Request<'a> {
    name: &'a str,
    description: &'a str,
    plan: &'a str,
    profile: &'static Profile,
    modules: MemoryModules,
    target: Option<TargetFile<'a>>,
}

/// The `target` argument (leaf `M4.1.7.5.4.4`; `DC-H2`), the target's file the plan was resolved on — `{ "text", "name" }`,
/// as the generator's server takes it, its name optional — read, or what is wrong with it.
fn target(argument: Option<&Value>) -> Result<Option<TargetFile<'_>>, String> {
    let Some(argument) = argument else {
        return Ok(None);
    };
    let Value::Object(members) = argument else {
        return Err("`target` is not an object".to_owned());
    };
    if let Some((key, _)) = members
        .iter()
        .find(|(key, _)| !matches!(key.as_str(), "text" | "name"))
    {
        return Err(format!("`target` has no member `{key}`"));
    }
    let text = argument
        .get("text")
        .and_then(Value::as_str)
        .ok_or_else(|| {
            "`target`'s `text`, the target's file, is missing or not a string".to_owned()
        })?;
    let name = match argument.get("name") {
        None => TARGET_NAME,
        Some(Value::String(name)) => name.as_str(),
        Some(_) => return Err("`target`'s `name` is not a string".to_owned()),
    };
    Ok(Some(TargetFile { name, text }))
}

/// `plan-check`'s arguments, read, or what is wrong with them.
fn request(arguments: &Value) -> Result<Request<'_>, String> {
    let Value::Object(members) = arguments else {
        return Err("the arguments are not an object".to_owned());
    };
    for (key, _) in members {
        if !matches!(
            key.as_str(),
            "description" | "plan" | "name" | "profile" | "modules" | "target"
        ) {
            return Err(format!("there is no argument `{key}`"));
        }
    }
    let text = |key: &str, what: &str| {
        arguments
            .get(key)
            .and_then(Value::as_str)
            .ok_or_else(|| format!("`{key}`, {what}, is missing or not a string"))
    };
    let description = text("description", "the description's text")?;
    let plan = text("plan", "the plan's text")?;
    let optional = |key: &str| match arguments.get(key) {
        None => Ok(None),
        Some(Value::String(value)) => Ok(Some(value.as_str())),
        Some(_) => Err(format!("`{key}` is not a string")),
    };
    let name = optional("name")?.unwrap_or(DESCRIPTION_NAME);
    let profile = judgement::profile(optional("profile")?)?;
    let mut modules = MemoryModules::new();
    let mut texts = vec![description, plan];
    match arguments.get("modules") {
        None => {}
        Some(Value::Object(entries)) => {
            for (module, value) in entries {
                let Some(module_text) = value.as_str() else {
                    return Err(format!("module `{module}`'s text is not a string"));
                };
                texts.push(module_text);
                modules = modules.with(module, module_text);
            }
        }
        Some(_) => return Err("`modules` is not an object".to_owned()),
    }
    let target = target(arguments.get("target"))?;
    texts.extend(target.map(|target| target.text));
    judgement::budget(texts)?;
    Ok(Request {
        name,
        description,
        plan,
        profile,
        modules,
        target,
    })
}

/// The tool: its name and summary, its arguments' schema, and the annotations true of it.
fn tool() -> Value {
    object(vec![
        ("name", string(TOOL)),
        (
            "description",
            string(
                "Check a resolved plan against its eADL description, apart from the search that wrote it; the \
                 answer is the result record, archogen-result/1, and its status the verdict",
            ),
        ),
        ("inputSchema", schema()),
        (
            "annotations",
            object(vec![
                ("readOnlyHint", Value::Bool(true)),
                ("destructiveHint", Value::Bool(false)),
                ("idempotentHint", Value::Bool(true)),
                ("openWorldHint", Value::Bool(false)),
            ]),
        ),
    ])
}

/// What `plan-check` takes.
fn schema() -> Value {
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
                    "plan",
                    text(
                        "the assignment record the search wrote for it, archogen-assignment/2; findings name it \
                         `plan.json`",
                    ),
                ),
                (
                    "name",
                    text("how findings name the description; `description.eadl` when absent"),
                ),
                (
                    "profile",
                    text("the profile the plan was resolved under; `rt-static-up-v1` when absent"),
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
                (
                    "target",
                    object(vec![
                        ("type", string("object")),
                        (
                            "description",
                            string(
                                "the target's eADL file the plan was resolved on, archogen-assignment/3: the system is \
                                 put on it and the plan judged there; a plan of another target, or of none, is \
                                 stale-input",
                            ),
                        ),
                        (
                            "properties",
                            object(vec![
                                ("text", text("the target's file's text")),
                                (
                                    "name",
                                    text("how findings name the target's file; `target.eadl` when absent"),
                                ),
                            ]),
                        ),
                        ("required", Value::Array(vec![string("text")])),
                        ("additionalProperties", Value::Bool(false)),
                    ]),
                ),
            ]),
        ),
        (
            "required",
            Value::Array(vec![string("description"), string("plan")]),
        ),
        ("additionalProperties", Value::Bool(false)),
    ])
}
