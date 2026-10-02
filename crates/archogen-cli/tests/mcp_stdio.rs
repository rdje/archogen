//! The MCP server, driven as an agent drives it (leaf `API.6.4`, `docs/decisions/decision_mcp-server.md`): the
//! built `archogen` spawned, both protocol eras spoken over its standard input and output, and every verdict it
//! answers held to the command line's for the same description.
//!
//! The test spawns the binary; the server spawns nothing (`NO-SUBPROCESS` reads production code only).

use std::fs;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};

use archogen_cli::json::{self, Value};
use archogen_cli::{run, Status};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Send `lines` to a fresh `archogen mcp` and return its answers, one per line, with its exit status and stderr.
fn session(lines: &[String]) -> (Vec<Value>, i32, String) {
    let mut child = Command::new(env!("CARGO_BIN_EXE_archogen"))
        .arg("mcp")
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the built binary starts");
    // Written on a thread of its own, so a session whose input and output both pass a pipe's buffer cannot deadlock
    // (review of `API.6.5`, N7). Dropping stdin ends the input, and the server returns.
    let mut stdin = child.stdin.take().expect("a stdin");
    let input: Vec<String> = lines.to_vec();
    let writer = std::thread::spawn(move || {
        for line in input {
            writeln!(stdin, "{line}").expect("the server reads");
        }
    });
    let output = child.wait_with_output().expect("the server ends");
    writer.join().expect("the input was written");
    let stdout = String::from_utf8(output.stdout).expect("utf-8 stdout");
    let answers = stdout
        .lines()
        .map(|line| {
            let value = json::read(line.as_bytes(), usize::MAX).expect("every stdout line is JSON");
            assert_eq!(json::write(&value), line, "an answer is written one way");
            value
        })
        .collect();
    (
        answers,
        output.status.code().unwrap_or(-1),
        String::from_utf8(output.stderr).expect("utf-8 stderr"),
    )
}

const META: &str = r#""_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}"#;

fn check_request(id: usize, name: &str, text: &str) -> String {
    format!(
        r#"{{"jsonrpc":"2.0","id":{id},"method":"tools/call","params":{{{META},"name":"check","arguments":{{"description":{},"name":{}}}}}}}"#,
        json::write(&Value::String(text.to_owned())),
        json::write(&Value::String(name.to_owned()))
    )
}

fn error_code(answer: &Value) -> Option<String> {
    match answer.get("error")?.get("code")? {
        Value::Number(number) => Some(number.as_str().to_owned()),
        _ => None,
    }
}

#[test]
fn both_eras_are_spoken_over_stdio_a_refusal_and_an_unbuilt_tool_among_them() {
    let lines = [
        format!(r#"{{"jsonrpc":"2.0","id":1,"method":"server/discover","params":{{{META}}}}}"#),
        format!(r#"{{"jsonrpc":"2.0","id":2,"method":"tools/list","params":{{{META}}}}}"#),
        check_request(3, "broken.eadl", "(defblock"),
        format!(r#"{{"jsonrpc":"2.0","id":4,"method":"tools/call","params":{{{META},"name":"analyze","arguments":{{}}}}}}"#),
        r#"{"jsonrpc":"2.0","id":5,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"test","version":"1"}}}"#.to_owned(),
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#.to_owned(),
        r#"{"jsonrpc":"2.0","id":6,"method":"tools/list"}"#.to_owned(),
        r#"{"jsonrpc":"2.0","id":7,"method":"ping"}"#.to_owned(),
        r#"{"jsonrpc":"2.0","id":8,"method":"tools/call","params":{"name":"check","arguments":{"description":"(defblock console.uart (offers observable-output))\n"}}}"#.to_owned(),
        "{".to_owned(),
    ];
    let (answers, code, stderr) = session(&lines);
    assert_eq!(
        code, 0,
        "the server exits 0 at the end of its input: {stderr}"
    );
    assert!(
        stderr.is_empty(),
        "nothing but answers is written: {stderr}"
    );
    assert_eq!(
        answers.len(),
        9,
        "every request answered, the notification not"
    );

    let ids: Vec<String> = answers
        .iter()
        .map(|answer| answer.get("id").map(json::write).unwrap_or_default())
        .collect();
    assert_eq!(
        ids,
        ["1", "2", "3", "4", "5", "6", "7", "8", ""],
        "an unreadable request's answer carries no id"
    );

    let result = |index: usize| {
        answers[index]
            .get("result")
            .unwrap_or_else(|| panic!("answer {index}: {}", json::write(&answers[index])))
    };
    assert_eq!(
        result(0).get("resultType").and_then(Value::as_str),
        Some("complete")
    );
    assert!(result(1).get("tools").is_some());
    assert_eq!(
        result(2).get("isError"),
        Some(&Value::Bool(true)),
        "a refusal is an error result"
    );
    assert_eq!(
        result(2)
            .get("structuredContent")
            .and_then(|s| s.get("status"))
            .and_then(Value::as_str),
        Some("invalid-description")
    );
    assert_eq!(
        result(3).get("isError"),
        Some(&Value::Bool(true)),
        "an unbuilt tool names its owner"
    );
    assert!(json::write(result(3)).contains("M2.6"));
    assert_eq!(
        result(4).get("protocolVersion").and_then(Value::as_str),
        Some("2025-11-25")
    );
    assert!(result(5).get("tools").is_some() && result(5).get("resultType").is_none());
    assert_eq!(json::write(result(6)), "{}");
    assert_eq!(
        result(7).get("isError"),
        Some(&Value::Bool(false)),
        "an accepted description is no error"
    );
    assert_eq!(error_code(&answers[8]).as_deref(), Some("-32700"));
}

/// Every single-file case of the conformance suite, checked by the command line and by the server in both protocol
/// revisions: the server's exit code and `isError` are the command line's.
#[test]
fn every_verdict_is_the_command_lines_for_the_same_description() {
    let cases_dir = repo_root().join("docs/semantics/cases");
    let mut cases: Vec<PathBuf> = fs::read_dir(&cases_dir)
        .expect("the cases directory")
        .map(|entry| entry.expect("an entry").path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "eadl"))
        .collect();
    cases.sort();
    let texts: Vec<(String, String)> = cases
        .iter()
        .map(|path| {
            let text = fs::read_to_string(path).expect("a case reads");
            assert!(
                !text.contains("(import"),
                "{}: a single-file case",
                path.display()
            );
            (path.display().to_string(), text)
        })
        .collect();
    assert!(
        texts.len() >= 30,
        "the suite is where it was: {} cases",
        texts.len()
    );

    let modern: Vec<String> = texts
        .iter()
        .enumerate()
        .map(|(index, (name, text))| check_request(index, name, text))
        .collect();
    let (modern_answers, code, stderr) = session(&modern);
    assert_eq!((code, modern_answers.len()), (0, texts.len()), "{stderr}");
    let mut legacy = vec![r#"{"jsonrpc":"2.0","id":"init","method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"test","version":"1"}}}"#.to_owned()];
    legacy.extend(texts.iter().enumerate().map(|(index, (name, text))| {
        format!(
            r#"{{"jsonrpc":"2.0","id":{index},"method":"tools/call","params":{{"name":"check","arguments":{{"description":{},"name":{}}}}}}}"#,
            json::write(&Value::String(text.clone())),
            json::write(&Value::String(name.clone()))
        )
    }));
    let (mut legacy_answers, code, stderr) = session(&legacy);
    assert_eq!(
        (code, legacy_answers.len()),
        (0, texts.len() + 1),
        "{stderr}"
    );
    legacy_answers.remove(0);
    for (index, answer) in legacy_answers.iter().enumerate() {
        let result = answer.get("result").expect("a legacy result");
        assert!(
            result.get("resultType").is_none(),
            "{}",
            json::write(answer)
        );
        assert_eq!(
            json::write(result.get("structuredContent").expect("structured content")),
            json::write(
                modern_answers[index]
                    .get("result")
                    .unwrap()
                    .get("structuredContent")
                    .unwrap()
            ),
            "both revisions answer the same judgement"
        );
    }
    let answers = modern_answers;

    let mut disagreements = Vec::new();
    for ((name, _), answer) in texts.iter().zip(&answers) {
        let structured = answer
            .get("result")
            .and_then(|result| result.get("structuredContent"))
            .unwrap_or_else(|| panic!("{name}: {}", json::write(answer)));
        let exit = match structured.get("exit") {
            Some(Value::Number(number)) => number.as_i64(),
            _ => None,
        };
        let mut out = Vec::new();
        let mut err = Vec::new();
        let status: Status = run(["check".to_owned(), name.clone()], &mut out, &mut err);
        if exit != Some(i64::from(status.code())) {
            disagreements.push(format!(
                "{name}: the server says {exit:?}, the command line {}",
                status.code()
            ));
        }
        let is_error = answer
            .get("result")
            .and_then(|result| result.get("isError"));
        if is_error != Some(&Value::Bool(status != Status::Ok)) {
            disagreements.push(format!(
                "{name}: isError {is_error:?} against status {status:?}"
            ));
        }
    }
    assert!(disagreements.is_empty(), "{}", disagreements.join("\n"));
}

/// The transcript the book shows (`docs/book/src/engine-api.md`, under its `mcp-transcript` marker) is a real run:
/// each `→` line is sent to the built server, and its answers are the `←` lines, byte for byte.
#[test]
fn the_books_transcript_is_what_the_server_answers() {
    let chapter =
        fs::read_to_string(repo_root().join("docs/book/src/engine-api.md")).expect("the chapter");
    let after = chapter
        .split("<!-- mcp-transcript")
        .nth(1)
        .expect("the chapter marks its transcript");
    let block = after
        .split("```text\n")
        .nth(1)
        .and_then(|rest| rest.split("```").next())
        .expect("a text block follows the marker");
    let mut sent = Vec::new();
    let mut expected = Vec::new();
    for line in block.lines() {
        if let Some(request) = line.strip_prefix("→ ") {
            sent.push(request.to_owned());
        } else if let Some(answer) = line.strip_prefix("← ") {
            expected.push(answer.to_owned());
        } else {
            panic!("a transcript line is a `→` request or a `←` answer: {line}");
        }
    }
    assert!(!sent.is_empty(), "the transcript sends something");
    let (answers, code, stderr) = session(&sent);
    assert_eq!(code, 0, "{stderr}");
    let answered: Vec<String> = answers.iter().map(json::write).collect();
    assert_eq!(answered, expected);
}
