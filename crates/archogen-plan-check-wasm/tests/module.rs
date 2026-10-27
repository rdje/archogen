//! The configuration checker's browser module on the host (leaf `M3.5.7.1`; `docs/decisions/decision_wasm-binding.md`
//! §11): its answer is the record `judgement::judge` writes, and the MCP tool's for the same texts and producer, byte
//! for byte; every request it cannot judge is a refusal naming what to change; the exports carry a request in and the
//! answer out as the loader does; and the module depends on no generator's package.

use std::collections::BTreeSet;
use std::process::Command;

use archogen_digest::Digest;
use archogen_json::{self as json, Value};
use archogen_outcome::record::{Producer, Record};
use archogen_outcome::Status;
use archogen_plan_check::assignment::{program_digest, Assignment, Requester};
use archogen_plan_check::check::plan_file;
use archogen_plan_check::judgement::{self, Inputs, BYTES, PLAN_NAME};
use archogen_plan_check::mcp::{Checker, LINE_LIMIT};
use archogen_plan_check_wasm::request::{frame, parse, Framed};
use archogen_plan_check_wasm::{
    answer, plan_check, plan_check_input, plan_check_output, INPUT_CAP, PRODUCER, REFUSAL_FORMAT,
    REQUEST_FORMAT,
};
use eadl_front::module::MemoryModules;
use eadl_front::SourceMap;
use eadl_model::profile::RT_STATIC_UP_V1;

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

/// The digest a loader would pass: any well-spelt one, the module recording what it is given.
fn digest() -> Digest {
    Digest::of(b"the module's bytes")
}

/// The modules `given` names, as the checker reads them.
fn memory(given: &[(String, String)]) -> MemoryModules {
    given
        .iter()
        .fold(MemoryModules::new(), |modules, (name, text)| {
            modules.with(name, text)
        })
}

/// The plan binding the console to `provider`, for the program `description` becomes with `modules`.
fn plan(description: &str, modules: &MemoryModules, provider: &str) -> String {
    let mut sources = SourceMap::new();
    let id = sources.add("description.eadl", description).unwrap();
    let program = plan_file(&mut sources, id, modules, &RT_STATIC_UP_V1, PLAN_NAME, "").0;
    let mut bindings = std::collections::BTreeMap::new();
    bindings.insert(
        (
            Requester::Named("console.write".to_owned()),
            "observable-output".to_owned(),
        ),
        provider.into(),
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

/// A request for `description` with `modules` and `plan`, under the default profile.
fn request(description: &str, modules: &[(&str, &str)], plan: &str) -> Framed {
    Framed {
        producer: digest(),
        name: "description.eadl".to_owned(),
        text: description.to_owned(),
        profile: None,
        modules: modules
            .iter()
            .map(|(name, text)| ((*name).to_owned(), (*text).to_owned()))
            .collect(),
        plan: plan.to_owned(),
    }
}

/// The record the MCP tool answers for `framed`'s texts, its producer the module's.
fn mcp_record(framed: &Framed) -> String {
    let checker = Checker::new(Producer {
        name: PRODUCER.to_owned(),
        sha256: framed.producer,
    });
    let arguments = Value::Object(vec![
        ("description".to_owned(), Value::String(framed.text.clone())),
        ("plan".to_owned(), Value::String(framed.plan.clone())),
        ("name".to_owned(), Value::String(framed.name.clone())),
        (
            "modules".to_owned(),
            Value::Object(
                framed
                    .modules
                    .iter()
                    .map(|(name, text)| (name.clone(), Value::String(text.clone())))
                    .collect(),
            ),
        ),
    ]);
    let line = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"tools/call","params":{{"_meta":{{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{{}}}},"name":"plan-check","arguments":{}}}}}"#,
        json::write(&arguments)
    );
    let answer = archogen_transport::Server::new(&checker, LINE_LIMIT)
        .answer(line.as_bytes())
        .expect("answered");
    let answer = json::read(answer.as_bytes(), usize::MAX).expect("JSON");
    match answer
        .get("result")
        .and_then(|result| result.get("content"))
    {
        Some(Value::Array(blocks)) => blocks[0]
            .get("text")
            .and_then(Value::as_str)
            .expect("a text block")
            .to_owned(),
        _ => panic!("a tool result: {}", json::write(&answer)),
    }
}

#[test]
fn a_plan_is_answered_with_the_record_judge_and_the_mcp_tool_write() {
    let none = MemoryModules::new();
    let board = MemoryModules::new().with("hw.board", BOARD);
    let broken = format!("{PLAIN}(defservice broken (requries (needs x)))\n");
    for (case, framed, status) in [
        (
            "accepted",
            request(PLAIN, &[], &plan(PLAIN, &none, "console.uart")),
            Status::Ok,
        ),
        (
            "corrupted",
            request(PLAIN, &[], &plan(PLAIN, &none, "soc")),
            Status::ToolFailure,
        ),
        (
            "stale",
            request(
                PLAIN,
                &[],
                &plan(
                    &format!("{PLAIN}(defservice idle (requires))\n"),
                    &none,
                    "console.uart",
                ),
            ),
            Status::StaleInput,
        ),
        (
            "a module's",
            request(
                APP,
                &[("hw.board", BOARD)],
                &plan(APP, &board, "hw.console.uart"),
            ),
            Status::Ok,
        ),
        (
            "a refused description's",
            request(&broken, &[], &plan(&broken, &none, "console.uart")),
            Status::InvalidDescription,
        ),
    ] {
        let answered = answer(&frame(&framed));
        let record = Record::read(answered.as_bytes(), 1 << 20)
            .unwrap_or_else(|refusal| panic!("{case}: a record: {refusal}\n{answered}"));
        assert_eq!(record.status(), status, "{case}: {:?}", record.findings());
        assert_eq!(
            record.producer(),
            &Producer {
                name: PRODUCER.to_owned(),
                sha256: digest()
            },
            "{case}"
        );
        let modules = memory(&framed.modules);
        let judged = judgement::judge(
            record.producer().clone(),
            &Inputs {
                description_name: &framed.name,
                description: &framed.text,
                modules: &modules,
                profile: &RT_STATIC_UP_V1,
                plan_name: PLAN_NAME,
                plan: framed.plan.as_bytes(),
            },
        )
        .expect("a record");
        assert_eq!(answered, judged.write(), "{case}: judge's record");
        assert_eq!(
            answered,
            mcp_record(&framed),
            "{case}: the MCP tool's record"
        );
    }
}

/// A refusal's status, exit code and note, its members in their order.
fn refusal(answered: &str) -> (String, i64, String) {
    assert!(answered.ends_with('\n'), "one line: {answered:?}");
    let value = json::read(answered.trim_end().as_bytes(), usize::MAX).expect("a refusal is JSON");
    let Value::Object(members) = &value else {
        panic!("an object: {answered}");
    };
    let keys: Vec<&str> = members.iter().map(|(key, _)| key.as_str()).collect();
    assert_eq!(keys, ["format", "status", "exit", "note"], "{answered}");
    assert_eq!(
        value.get("format").and_then(Value::as_str),
        Some(REFUSAL_FORMAT)
    );
    let exit = match value.get("exit") {
        Some(Value::Number(number)) => number.as_i64().expect("an integer"),
        _ => panic!("an exit: {answered}"),
    };
    (
        value
            .get("status")
            .and_then(Value::as_str)
            .unwrap()
            .to_owned(),
        exit,
        value
            .get("note")
            .and_then(Value::as_str)
            .unwrap()
            .to_owned(),
    )
}

/// A field as the framing writes one.
fn field(out: &mut Vec<u8>, bytes: &[u8]) {
    out.extend_from_slice(&u32::try_from(bytes.len()).unwrap().to_le_bytes());
    out.extend_from_slice(bytes);
}

#[test]
fn every_request_it_cannot_judge_is_refused_with_what_to_change() {
    let good = request(
        PLAIN,
        &[],
        &plan(PLAIN, &MemoryModules::new(), "console.uart"),
    );
    let framed = frame(&good);
    // The producer field starts after the format's: 4 bytes of length and the format itself.
    let producer_at = 4 + REQUEST_FORMAT.len();
    let with_producer = |producer: &str| {
        let mut out = Vec::new();
        field(&mut out, REQUEST_FORMAT.as_bytes());
        field(&mut out, producer.as_bytes());
        out.extend_from_slice(&framed[producer_at + 4 + 71..]);
        out
    };
    let mut other_format = Vec::new();
    field(&mut other_format, b"archogen-wasm-request/1");
    other_format.extend_from_slice(&framed[producer_at..]);
    let mut trailing = framed.clone();
    trailing.push(0);
    let mut twice = good.clone();
    twice.modules = vec![
        ("hw.board".to_owned(), BOARD.to_owned()),
        ("hw.board".to_owned(), BOARD.to_owned()),
    ];
    let mut not_utf8 = Vec::new();
    field(&mut not_utf8, REQUEST_FORMAT.as_bytes());
    field(&mut not_utf8, digest().to_string().as_bytes());
    field(&mut not_utf8, b"description.eadl");
    field(&mut not_utf8, &[0xff, 0xfe]);
    let mut profile = good.clone();
    profile.profile = Some("rt-other-v9".to_owned());
    let mut past = good.clone();
    past.modules = vec![("pad".to_owned(), " ".repeat(BYTES))];
    for (case, bytes, says) in [
        (
            "another format",
            other_format,
            "the request's format field, at byte 0, is `archogen-wasm-request/1`",
        ),
        (
            "a digest in capitals",
            with_producer(
                &digest()
                    .to_string()
                    .to_uppercase()
                    .replacen("SHA256", "sha256", 1),
            ),
            &*format!("the request's producer field, at byte {producer_at}, is not `sha256:`"),
        ),
        (
            "a digest without its prefix",
            with_producer(&format!("sha512:{}", digest().hex())),
            &*format!("the request's producer field, at byte {producer_at}, is not `sha256:`"),
        ),
        (
            "a request cut short",
            framed[..framed.len() - 3].to_vec(),
            "the request's plan field",
        ),
        (
            "a text that is no UTF-8",
            not_utf8,
            "the request's text field",
        ),
        (
            "a module named twice",
            frame(&twice),
            "the request's module 2 name field",
        ),
        ("a byte after the plan", trailing, "the request's end field"),
        (
            "another profile",
            frame(&profile),
            "the profile `rt-other-v9` is not one this checker has",
        ),
        (
            "texts past the budget",
            frame(&past),
            &*format!("past this checker's budget of {BYTES}"),
        ),
    ] {
        let (status, exit, note) = refusal(&answer(&bytes));
        assert_eq!((status.as_str(), exit), ("usage", 2), "{case}: {note}");
        assert!(note.contains(says), "{case}: {note}");
    }
}

#[test]
fn a_request_reads_back_as_it_was_framed() {
    let mut framed = request(APP, &[("hw.board", BOARD)], "a plan's text");
    assert_eq!(parse(&frame(&framed)), Ok(framed.clone()));
    framed.profile = Some(RT_STATIC_UP_V1.id.to_owned());
    assert_eq!(parse(&frame(&framed)), Ok(framed.clone()));
    // An empty profile field is the default, and the default judges as the profile named.
    let plan_text = plan(PLAIN, &MemoryModules::new(), "console.uart");
    let named = Framed {
        profile: Some(RT_STATIC_UP_V1.id.to_owned()),
        ..request(PLAIN, &[], &plan_text)
    };
    assert_eq!(
        answer(&frame(&named)),
        answer(&frame(&request(PLAIN, &[], &plan_text)))
    );
}

#[test]
fn the_exports_carry_a_request_in_and_the_answer_out_as_the_loader_does() {
    let request = frame(&request(
        PLAIN,
        &[],
        &plan(PLAIN, &MemoryModules::new(), "console.uart"),
    ));
    let at = plan_check_input(request.len());
    assert!(!at.is_null());
    // SAFETY: `plan_check_input` returned a buffer of exactly `request.len()` bytes that the module owns, and no Rust
    // borrow of it is live until the next export is called: the page's write, done the page's way.
    unsafe { core::ptr::copy_nonoverlapping(request.as_ptr(), at, request.len()) };
    let length = plan_check();
    // SAFETY: `plan_check_output` is the address of the answer `plan_check` just kept, `length` bytes long, and valid
    // until the next export is called.
    let bytes = unsafe { core::slice::from_raw_parts(plan_check_output(), length) }.to_vec();
    assert_eq!(bytes, answer(&request).into_bytes());
    // Above the cap, nothing is allocated, and the empty request left behind is refused, not judged.
    assert!(plan_check_input(INPUT_CAP + 1).is_null());
    let length = plan_check();
    // SAFETY: as above — the answer `plan_check` just kept, `length` bytes long.
    let bytes = unsafe { core::slice::from_raw_parts(plan_check_output(), length) }.to_vec();
    let (status, _, note) = refusal(&String::from_utf8(bytes).unwrap());
    assert_eq!(status, "usage", "{note}");
}

#[test]
fn the_module_depends_on_no_generator_package() {
    // Every workspace package the module compiles, through its normal dependencies, from the workspace's own
    // metadata: none of the generator's.
    let output = Command::new(env!("CARGO"))
        .args([
            "metadata",
            "--offline",
            "--no-deps",
            "--format-version",
            "1",
        ])
        .current_dir(env!("CARGO_MANIFEST_DIR"))
        .output()
        .expect("cargo metadata runs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    let metadata = json::read(&output.stdout, usize::MAX).expect("metadata is JSON");
    let Some(Value::Array(packages)) = metadata.get("packages") else {
        panic!("packages");
    };
    let normal = |package: &Value| -> Vec<String> {
        let Some(Value::Array(dependencies)) = package.get("dependencies") else {
            return Vec::new();
        };
        dependencies
            .iter()
            .filter(|dependency| matches!(dependency.get("kind"), None | Some(Value::Null)))
            .filter_map(|dependency| {
                dependency
                    .get("name")
                    .and_then(Value::as_str)
                    .map(str::to_owned)
            })
            .collect()
    };
    let by_name = |name: &str| {
        packages
            .iter()
            .find(|package| package.get("name").and_then(Value::as_str) == Some(name))
    };
    let mut reached = BTreeSet::new();
    let mut next = vec![env!("CARGO_PKG_NAME").to_owned()];
    while let Some(name) = next.pop() {
        if let Some(package) = by_name(&name) {
            for dependency in normal(package) {
                if reached.insert(dependency.clone()) {
                    next.push(dependency);
                }
            }
        }
    }
    assert!(reached.contains("archogen-plan-check"), "{reached:?}");
    for generator in [
        "archogen-api",
        "archogen-wasm",
        "eadl-resolve",
        "archogen-cli",
    ] {
        assert!(
            !reached.contains(generator),
            "the module compiles `{generator}`: {reached:?}"
        );
    }
}
