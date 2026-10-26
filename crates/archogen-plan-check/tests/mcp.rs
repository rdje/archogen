//! The configuration checker's MCP server (leaf `M3.5.1.6.4`): its one tool listed as `M3.5.1`'s design states it —
//! texts, never paths, read-only — the arguments it refuses and its budget, and its answer: the result record, read
//! back by the record's strict reader, and the same record, byte for byte, as the executable writes from files that
//! hold the same texts.

use std::io::Write as _;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_digest::Digest;
use archogen_json::{self as json, Value};
use archogen_outcome::record::{Judged, Producer, Record};
use archogen_outcome::Status;
use archogen_plan_check::assignment::{program_digest, program_text, Assignment, Requester};
use archogen_plan_check::check::{plan_file, PLAN_STALE, PLAN_UNSATISFIED};
use archogen_plan_check::mcp::{Checker, BYTES, LINE_LIMIT, TOOL};
use archogen_plan_check::{NAME, VERSION};
use archogen_transport::Server;
use eadl_front::module::MemoryModules;
use eadl_front::SourceMap;
use eadl_model::profile::RT_STATIC_UP_V1;

const EXE: &str = env!("CARGO_BIN_EXE_archogen-plan-check");

const MODERN_META: &str = r#""_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}"#;

/// A plain description: a console service on a platform composing a serial port.
const PLAIN: &str = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
";

/// A module the description below imports.
const BOARD: &str = "(defmodule hw.board (version 1 0) (export soc console.uart)
  (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart))))";

/// A module description importing [`BOARD`].
const APP: &str = "(defmodule app (version 1 0) (import hw.board (as hw))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s
    (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
    (requires (uses console.write))
    (platform (uses hw.soc))))";

/// The producer the library's tests name: no executable ran.
fn producer() -> Producer {
    Producer {
        name: NAME.to_owned(),
        sha256: Digest::of(b"the checker under test"),
    }
}

/// The plan binding the console to `provider`, for the program `description` becomes with `modules`.
fn plan(description: &str, modules: &MemoryModules, provider: &str) -> String {
    let mut sources = SourceMap::new();
    let id = sources.add("description.eadl", description).unwrap();
    let program = plan_file(&mut sources, id, modules, &RT_STATIC_UP_V1, "plan.json", "").0;
    let mut bindings = std::collections::BTreeMap::new();
    bindings.insert(
        (
            Requester::Named("console.write".to_owned()),
            "observable-output".to_owned(),
        ),
        provider.to_owned(),
    );
    Assignment {
        profile: RT_STATIC_UP_V1.id.to_owned(),
        system: "s".to_owned(),
        program: program_digest(&program),
        placement: None,
        bindings,
    }
    .to_canonical()
}

/// A request of the current era, `params` holding `extra` after its `_meta`.
fn modern(method: &str, extra: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":1,"method":"{method}","params":{{{MODERN_META}{extra}}}}}"#)
}

/// A call of the tool with `arguments`, a JSON object.
fn call_line(arguments: &Value) -> String {
    modern(
        "tools/call",
        &format!(r#","name":"{TOOL}","arguments":{}"#, json::write(arguments)),
    )
}

/// The answer to `line`, read; every answer is JSON the strict reader admits.
fn answer(line: &str) -> Value {
    let checker = Checker::new(producer());
    let answer = Server::new(&checker, LINE_LIMIT)
        .answer(line.as_bytes())
        .expect("a request is answered");
    json::read(answer.as_bytes(), usize::MAX).expect("every answer is JSON")
}

/// The tool's result for `arguments`.
fn call(arguments: &Value) -> Value {
    let answer = answer(&call_line(arguments));
    answer
        .get("result")
        .unwrap_or_else(|| panic!("a result: {}", json::write(&answer)))
        .clone()
}

/// The arguments: `description` and `plan`, and `extra` members after them.
fn arguments(description: &str, plan: &str, extra: Vec<(&str, Value)>) -> Value {
    let mut members = vec![
        (
            "description".to_owned(),
            Value::String(description.to_owned()),
        ),
        ("plan".to_owned(), Value::String(plan.to_owned())),
    ];
    members.extend(
        extra
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value)),
    );
    Value::Object(members)
}

/// The record a result carries, read back by the record's strict reader: its text block and its structured content
/// one record, and `isError` true exactly when it does not accept.
fn record_of(result: &Value) -> Record {
    let text = result
        .get("content")
        .and_then(|content| match content {
            Value::Array(blocks) if blocks.len() == 1 => blocks[0].get("text")?.as_str(),
            _ => None,
        })
        .expect("one text block");
    let record = Record::read(text.as_bytes(), 1 << 20).expect("the text block is a record");
    let structured = result.get("structuredContent").expect("structured content");
    // The text block is the record's bytes, its closing newline with them; the structured content, the same JSON.
    assert_eq!(
        format!("{}\n", json::write(structured)),
        text,
        "the structured content is the record's text"
    );
    assert_eq!(
        result.get("isError"),
        Some(&Value::Bool(record.status() != Status::Ok)),
        "isError says whether the record accepts"
    );
    record
}

/// A tool error's text, no record judged.
fn tool_error(result: &Value) -> String {
    assert_eq!(result.get("isError"), Some(&Value::Bool(true)));
    assert!(
        result.get("structuredContent").is_none(),
        "nothing was judged"
    );
    match result.get("content") {
        Some(Value::Array(blocks)) if blocks.len() == 1 => blocks[0]
            .get("text")
            .and_then(Value::as_str)
            .unwrap()
            .to_owned(),
        _ => panic!("one text block: {}", json::write(result)),
    }
}

#[test]
fn the_one_tool_takes_texts_and_is_read_only() {
    let answer = answer(&modern("tools/list", ""));
    let Some(Value::Array(tools)) = answer.get("result").and_then(|result| result.get("tools"))
    else {
        panic!("a tool list: {}", json::write(&answer));
    };
    assert_eq!(tools.len(), 1);
    let tool = &tools[0];
    assert_eq!(tool.get("name").and_then(Value::as_str), Some(TOOL));
    let schema = tool.get("inputSchema").unwrap();
    assert_eq!(
        json::write(schema.get("required").unwrap()),
        r#"["description","plan"]"#
    );
    let Some(Value::Object(properties)) = schema.get("properties") else {
        panic!("properties");
    };
    let names: Vec<&str> = properties.iter().map(|(name, _)| name.as_str()).collect();
    assert_eq!(names, ["description", "plan", "name", "profile", "modules"]);
    assert_eq!(
        schema.get("additionalProperties"),
        Some(&Value::Bool(false))
    );
    assert_eq!(
        json::write(tool.get("annotations").unwrap()),
        r#"{"readOnlyHint":true,"destructiveHint":false,"idempotentHint":true,"openWorldHint":false}"#
    );
}

#[test]
fn the_server_names_the_executable_and_the_package_s_version() {
    let answer = answer(&modern("server/discover", ""));
    // Under `2026-07-28` the server's identity is in the result's `_meta`.
    let info = answer
        .get("result")
        .and_then(|result| result.get("_meta"))
        .and_then(|meta| meta.get("io.modelcontextprotocol/serverInfo"))
        .unwrap_or_else(|| panic!("serverInfo: {}", json::write(&answer)));
    assert_eq!(info.get("name").and_then(Value::as_str), Some(NAME));
    assert_eq!(info.get("version").and_then(Value::as_str), Some(VERSION));
    assert_eq!(
        VERSION,
        env!("CARGO_PKG_VERSION"),
        "the version is the manifest's"
    );
}

#[test]
fn an_accepted_plan_is_answered_with_its_record() {
    let modules = MemoryModules::new();
    let plan_text = plan(PLAIN, &modules, "console.uart");
    let record = record_of(&call(&arguments(PLAIN, &plan_text, vec![])));
    assert!(record.accepts() && record.findings().is_empty());
    assert_eq!(record.producer(), &producer());
    assert_eq!(
        record.subject().judged,
        Judged::Plan(Digest::of(plan_text.as_bytes()))
    );
    assert_eq!(record.profile(), RT_STATIC_UP_V1.id);
    // The profile named, the same record.
    let named = record_of(&call(&arguments(
        PLAIN,
        &plan_text,
        vec![("profile", Value::String(RT_STATIC_UP_V1.id.to_owned()))],
    )));
    assert_eq!(named.write(), record.write());
}

#[test]
fn a_module_s_imports_are_read_from_the_texts_given() {
    let modules = MemoryModules::new().with("hw.board", BOARD);
    let plan_text = plan(APP, &modules, "hw.console.uart");
    let given = || {
        vec![(
            "modules",
            Value::Object(vec![(
                "hw.board".to_owned(),
                Value::String(BOARD.to_owned()),
            )]),
        )]
    };
    let record = record_of(&call(&arguments(APP, &plan_text, given())));
    assert_eq!(record.status(), Status::Ok, "{:?}", record.findings());
    let mut sources = SourceMap::new();
    let id = sources.add("description.eadl", APP).unwrap();
    let program = plan_file(
        &mut sources,
        id,
        &modules,
        &RT_STATIC_UP_V1,
        "plan.json",
        "",
    )
    .0;
    assert!(program_text(&program).contains("instance hw hw.board 1.0\n"));
    assert_eq!(
        record.subject().program,
        Digest::of(program_text(&program).as_bytes())
    );
    // Without the module's text, the description does not elaborate: its own status, judged and recorded.
    let record = record_of(&call(&arguments(APP, &plan_text, vec![])));
    assert!(
        !record.accepts() && record.status() != Status::ToolFailure,
        "{:?}",
        record.status()
    );
}

#[test]
fn a_stale_and_a_corrupted_plan_are_errors_carrying_their_records() {
    let modules = MemoryModules::new();
    let good = plan(PLAIN, &modules, "console.uart");
    let mut sources = SourceMap::new();
    let id = sources.add("description.eadl", PLAIN).unwrap();
    let digest = program_digest(
        &plan_file(
            &mut sources,
            id,
            &modules,
            &RT_STATIC_UP_V1,
            "plan.json",
            "",
        )
        .0,
    );
    let stale = good.replace(&digest, &"0".repeat(64));
    let record = record_of(&call(&arguments(PLAIN, &stale, vec![])));
    assert_eq!(
        (record.status(), record.findings()[0].code.as_str()),
        (Status::StaleInput, PLAN_STALE)
    );
    assert_eq!(record.findings()[0].primary.source, "plan.json");
    let moved = plan(PLAIN, &modules, "soc");
    let record = record_of(&call(&arguments(
        PLAIN,
        &moved,
        vec![("name", Value::String("system.eadl".to_owned()))],
    )));
    assert_eq!(
        (record.status(), record.findings()[0].code.as_str()),
        (Status::ToolFailure, PLAN_UNSATISFIED)
    );
}

#[test]
fn arguments_it_cannot_use_are_tool_errors_and_judge_nothing() {
    let text = |value: &str| Value::String(value.to_owned());
    let plan_text = plan(PLAIN, &MemoryModules::new(), "console.uart");
    for (case, arguments, says) in [
        (
            "no plan",
            Value::Object(vec![("description".to_owned(), text(PLAIN))]),
            "`plan`, the plan's text, is missing",
        ),
        (
            "no description",
            Value::Object(vec![("plan".to_owned(), text(&plan_text))]),
            "`description`, the description's text, is missing",
        ),
        (
            "a path for an argument",
            arguments(PLAIN, &plan_text, vec![("result", text("out.json"))]),
            "there is no argument `result`",
        ),
        (
            "a name that is no string",
            arguments(PLAIN, &plan_text, vec![("name", Value::Bool(true))]),
            "`name` is not a string",
        ),
        (
            "another profile",
            arguments(PLAIN, &plan_text, vec![("profile", text("rt-other-v9"))]),
            "the profile `rt-other-v9` is not one this checker has",
        ),
        (
            "modules that are no object",
            arguments(PLAIN, &plan_text, vec![("modules", text("hw.board"))]),
            "`modules` is not an object",
        ),
        (
            "a module's text that is no string",
            arguments(
                PLAIN,
                &plan_text,
                vec![(
                    "modules",
                    Value::Object(vec![("hw.board".to_owned(), Value::Bool(false))]),
                )],
            ),
            "module `hw.board`'s text is not a string",
        ),
    ] {
        let message = tool_error(&call(&arguments));
        assert!(message.contains(says), "{case}: {message}");
        assert!(
            message.contains("takes `description` and `plan`"),
            "{case}: {message}"
        );
    }
}

#[test]
fn texts_past_the_budget_are_refused_and_the_budget_itself_is_judged() {
    let plan_text = plan(PLAIN, &MemoryModules::new(), "console.uart");
    // Padded with blanks to the budget exactly, the description is the same program, and the plan still its plan.
    let at = BYTES - plan_text.len();
    let padded = format!("{PLAIN}{}", " ".repeat(at - PLAIN.len()));
    let record = record_of(&call(&arguments(&padded, &plan_text, vec![])));
    assert_eq!(record.status(), Status::Ok, "{:?}", record.findings());
    // One byte more, counted in a module's text, and nothing is judged.
    let past = arguments(
        &padded,
        &plan_text,
        vec![(
            "modules",
            Value::Object(vec![("hw.board".to_owned(), Value::String(" ".to_owned()))]),
        )],
    );
    let message = tool_error(&call(&past));
    assert!(
        message.contains(&format!(
            "{} bytes together, past this checker's budget of {BYTES}",
            BYTES + 1
        )),
        "{message}"
    );
}

/// A fresh scratch directory under the target directory, one per case.
fn scratch() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "plan-check-mcp-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

/// The executable served `input` as `--mcp`: its exit code and standard output.
fn serve(input: &str) -> (i32, String) {
    let mut child = Command::new(EXE)
        .arg("--mcp")
        .env_clear()
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the executable runs");
    child
        .stdin
        .take()
        .unwrap()
        .write_all(input.as_bytes())
        .unwrap();
    let output = child.wait_with_output().unwrap();
    (
        output.status.code().expect("an exit code"),
        String::from_utf8(output.stdout).unwrap(),
    )
}

#[test]
fn the_executable_serves_the_record_its_files_get() {
    let modules = MemoryModules::new().with("hw.board", BOARD);
    for (case, description, given, plan_text) in [
        (
            "accepted",
            PLAIN,
            vec![],
            plan(PLAIN, &MemoryModules::new(), "console.uart"),
        ),
        (
            "corrupted",
            PLAIN,
            vec![],
            plan(PLAIN, &MemoryModules::new(), "soc"),
        ),
        (
            "stale",
            PLAIN,
            vec![],
            plan(
                &format!("{PLAIN}(defservice idle (requires))\n"),
                &MemoryModules::new(),
                "console.uart",
            ),
        ),
        (
            "a module",
            APP,
            vec![("hw.board", BOARD)],
            plan(APP, &modules, "hw.console.uart"),
        ),
    ] {
        // The files, named as the tool names its texts when no name is given, judged from the scratch directory.
        let dir = scratch();
        std::fs::write(dir.join("description.eadl"), description).unwrap();
        std::fs::write(dir.join("plan.json"), &plan_text).unwrap();
        for (module, text) in &given {
            std::fs::write(dir.join(format!("{module}.eadl")), text).unwrap();
        }
        let files = Command::new(EXE)
            .args(["description.eadl", "plan.json"])
            .current_dir(&dir)
            .env_clear()
            .output()
            .expect("the executable runs");
        // The same texts, through the tool.
        let modules = Value::Object(
            given
                .iter()
                .map(|(module, text)| ((*module).to_owned(), Value::String((*text).to_owned())))
                .collect(),
        );
        let line = call_line(&arguments(
            description,
            &plan_text,
            vec![("modules", modules)],
        ));
        let (code, stdout) = serve(&format!("{line}\n"));
        assert_eq!(code, 0, "{case}: the server ends with its input");
        let answer = json::read(stdout.trim_end().as_bytes(), usize::MAX).expect("one answer");
        let record = record_of(answer.get("result").expect("a result"));
        assert_eq!(
            record.write().as_bytes(),
            files.stdout.as_slice(),
            "{case}: the tool's record is the files' record"
        );
        assert_eq!(files.status.code(), Some(record.status().code()), "{case}");
        // Both name the file that ran.
        assert_eq!(
            record.producer().sha256,
            Digest::of(&std::fs::read(EXE).unwrap()),
            "{case}"
        );
    }
}

#[test]
fn the_server_answers_each_line_and_takes_no_other_argument() {
    let (code, stdout) = serve(&format!(
        "{}\n{}\n",
        r#"{"jsonrpc":"2.0","id":7,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"t","version":"1"}}}"#,
        r#"{"jsonrpc":"2.0","id":8,"method":"tools/list"}"#
    ));
    assert_eq!(code, 0);
    let answers: Vec<Value> = stdout
        .lines()
        .map(|line| json::read(line.as_bytes(), usize::MAX).expect("each answer is JSON"))
        .collect();
    assert_eq!(answers.len(), 2, "{stdout}");
    let info = answers[0]
        .get("result")
        .and_then(|result| result.get("serverInfo"))
        .unwrap();
    assert_eq!(info.get("name").and_then(Value::as_str), Some(NAME));
    let listed = answers[1]
        .get("result")
        .and_then(|result| result.get("tools"))
        .unwrap();
    assert!(json::write(listed).contains(r#""name":"plan-check""#));
    for args in [
        vec!["--mcp", "a.eadl"],
        vec!["--profile", "rt-static-up-v1", "--mcp"],
        vec!["--mcp", "--result", "r"],
    ] {
        let output = Command::new(EXE)
            .args(&args)
            .env_clear()
            .stdin(Stdio::null())
            .output()
            .unwrap();
        assert_eq!(output.status.code(), Some(2), "{args:?}");
        assert!(
            String::from_utf8_lossy(&output.stderr).contains("`--mcp` takes no other argument"),
            "{args:?}"
        );
        assert!(output.stdout.is_empty(), "{args:?}");
    }
}
