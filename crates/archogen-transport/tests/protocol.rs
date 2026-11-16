//! The transport's protocol held on its own (leaf `M3.5.1.6.3`), over a fake server ([`common`]), so that a checker
//! serving its own tools over this package relies on no test of the generator's.
//!
//! ⭐ **A table of answers, each held to the site that writes it.** Every answer the package writes leaves through one of
//! its two writers, `error(` and `result(`, and each condition the framing refuses on its own passes through
//! `refused(`; each call names its site by a literal. [`cases`] holds requests and the whole answer line each must
//! produce — its id, its code and message, every member in its order — written out by hand, not by the package's
//! writer. Each case runs under `archogen_transport::traced`, which records the sites the request went through: the
//! site a case names must be among them, and every site of the source must be reached by some case, so a site no case
//! reaches, or a case that reaches another site than it names, fails. The census reads the source as text: every call
//! of the three names its site by a literal, each site once but the two `Failure` answers, and `json::write` is called
//! by the two writers and the tool result's read-back alone, so no answer is written past them.
//!
//! The table holds the sites; `tests/model.rs` holds the conditions, comparing every answer with an independent model
//! over generated message sequences. Beside them here, the framing: the bound exactly and across split reads, blank and
//! unterminated lines, one flush per answer.

mod common;

use std::collections::BTreeSet;
use std::io::{BufReader, Cursor, Write};

use archogen_json as json;
use archogen_transport::{serve, traced, Server};
use common::{quoted, Fake, LIMIT};

/// An error answer's whole line: no id where the request's could not be read.
fn err(id: Option<&str>, code: i64, message: &str) -> String {
    let id = id.map(|id| format!(r#""id":{id},"#)).unwrap_or_default();
    format!(
        r#"{{"jsonrpc":"2.0",{id}"error":{{"code":{code},"message":{}}}}}"#,
        quoted(message)
    )
}

/// A result answer's whole line.
fn ok(id: &str, result: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":{id},"result":{result}}}"#)
}

const META: &str = r#""_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":{}}"#;
const INFO: &str = r#""io.modelcontextprotocol/serverInfo":{"name":"fake","version":"9.9.9"}"#;
const INITIALIZE: &str = r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":{},"clientInfo":{"name":"c","version":"1"}}}"#;
const LIST: &str = r#"[{"name":"echo"},{"name":"broken"},{"name":"params"},{"name":"meta"},{"name":"typed"},{"name":"scalar"},{"name":"clash"},{"name":"badmeta"},{"name":"dup"}]"#;

fn modern(id: &str, method: &str, params: &str) -> String {
    let sep = if params.is_empty() { "" } else { "," };
    format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"{method}","params":{{{META}{sep}{params}}}}}"#)
}

fn legacy(id: &str, method: &str, params: &str) -> String {
    format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"{method}","params":{{{params}}}}}"#)
}

/// A modern result's whole line: `members`, then what the framing adds.
fn modern_ok(id: &str, members: &str) -> String {
    ok(
        id,
        &format!(r#"{{{members},"resultType":"complete","_meta":{{{INFO}}}}}"#),
    )
}

/// How a case's request is sent: to a server after its earlier lines, or alone through `serve`, for what only the
/// line framing decides.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Via {
    Server,
    Serve,
}

/// One case: the site it names, the lines answered before it, the request, and its whole answer line.
struct Case {
    site: &'static str,
    via: Via,
    before: Vec<&'static str>,
    request: String,
    answer: String,
}

fn case(site: &'static str, before: &[&'static str], request: String, answer: String) -> Case {
    Case {
        site,
        via: Via::Server,
        before: before.to_vec(),
        request,
        answer,
    }
}

fn served_alone(site: &'static str, request: String, answer: String) -> Case {
    Case {
        site,
        via: Via::Serve,
        before: Vec::new(),
        request,
        answer,
    }
}

/// Every site of the source, at least once, and each condition the review rounds found untested.
#[allow(clippy::too_many_lines)]
fn cases() -> Vec<Case> {
    let parse = |text: &[u8]| json::read(text, LIMIT).unwrap_err().to_string();
    let request_shape = "a request has `jsonrpc: \"2.0\"`, an id and a method";
    let id_shape = "a request's id is a string or an integral number";
    let resulttype = |tool: &str| {
        format!("the tool `{tool}` answered a result holding `resultType`, which is the server's to write")
    };
    let meta_owned = "a tool's `_meta` is no object, or already names `io.modelcontextprotocol/serverInfo`, which is the server's";
    let no_era = "a request names its protocol version in `_meta` (`io.modelcontextprotocol/protocolVersion`, with `io.modelcontextprotocol/clientCapabilities`), or follows `initialize`";
    let read_back = format!(
        "the tool `dup` answered a result that does not read back: {}",
        parse(br#"{"a":"1","a":"2"}"#)
    );
    let malformed_initialize =
        "`initialize` takes `protocolVersion`, `capabilities` and `clientInfo`";
    let long = format!(
        r#"{{"jsonrpc":"2.0","id":1,"method":"ping","params":{{"x":"{}"}}}}"#,
        "a".repeat(LIMIT)
    );
    vec![
        served_alone(
            "too-long",
            long,
            err(None, -32700, &format!("the message is longer than {LIMIT} bytes, and was not read")),
        ),
        // A form feed is no blank: JSON's whitespace has none, so the line is a message that does not read.
        served_alone("parse-error", "\u{0c}".into(), err(None, -32700, &format!("parse error: {}", parse(b"\x0c")))),
        case("parse-error", &[], "{".into(), err(None, -32700, &format!("parse error: {}", parse(b"{")))),
        case("not-an-object", &[], "[1]".into(), err(None, -32600, "a message is a JSON object")),
        case("no-id", &[], r#"{"jsonrpc":"2.0"}"#.into(), err(None, -32600, request_shape)),
        case("no-id", &[], r#"{"method":"ping"}"#.into(), err(None, -32600, request_shape)),
        case("fractional-id", &[], r#"{"jsonrpc":"2.0","id":1.5,"method":"ping"}"#.into(), err(None, -32600, id_shape)),
        case("fractional-id", &[], r#"{"jsonrpc":"2.0","id":1e-1,"method":"ping"}"#.into(), err(None, -32600, id_shape)),
        // An exponent past what `i64` holds, and the one whose negation overflows: fractional, read without a panic.
        case(
            "fractional-id",
            &[],
            r#"{"jsonrpc":"2.0","id":5e-99999999999999999999,"method":"ping"}"#.into(),
            err(None, -32600, id_shape),
        ),
        case(
            "fractional-id",
            &[],
            r#"{"jsonrpc":"2.0","id":5e-9223372036854775808,"method":"ping"}"#.into(),
            err(None, -32600, id_shape),
        ),
        case("other-id", &[], r#"{"jsonrpc":"2.0","id":null,"method":"ping"}"#.into(), err(None, -32600, id_shape)),
        case("other-id", &[], r#"{"jsonrpc":"2.0","id":{},"method":"ping"}"#.into(), err(None, -32600, id_shape)),
        case("no-version-or-method", &[], r#"{"id":"a","method":"ping"}"#.into(), err(Some("\"a\""), -32600, request_shape)),
        case("no-version-or-method", &[], r#"{"jsonrpc":"2.0","id":"b"}"#.into(), err(Some("\"b\""), -32600, request_shape)),
        case(
            "params-not-an-object",
            &[],
            r#"{"jsonrpc":"2.0","id":3,"method":"ping","params":[]}"#.into(),
            err(Some("3"), -32602, "a request's params are an object"),
        ),
        case(
            "params-not-an-object",
            &[],
            r#"{"jsonrpc":"2.0","id":3,"method":"ping","params":null}"#.into(),
            err(Some("3"), -32602, "a request's params are an object"),
        ),
        case("ping", &[], legacy("\"p\"", "ping", ""), ok("\"p\"", "{}")),
        case("ping", &[], r#"{"jsonrpc":"2.0","id":1e2,"method":"ping"}"#.into(), ok("1e2", "{}")),
        case(
            "ping",
            &[],
            r#"{"jsonrpc":"2.0","id":1e99999999999999999999,"method":"ping"}"#.into(),
            ok("1e99999999999999999999", "{}"),
        ),
        case(
            "legacy-no-method",
            &[INITIALIZE],
            legacy("4", "server/discover", ""),
            err(
                Some("4"),
                -32601,
                "no method `server/discover` in `2025-11-25`; a `2026-07-28` request names its version in `_meta` (`io.modelcontextprotocol/protocolVersion`)",
            ),
        ),
        case("no-era", &[], legacy("5", "tools/call", ""), err(Some("5"), -32602, no_era)),
        // A malformed `initialize` selects no era.
        case(
            "no-era",
            &[r#"{"jsonrpc":"2.0","id":0,"method":"initialize","params":{"protocolVersion":"2025-11-25","capabilities":[],"clientInfo":{}}}"#],
            legacy("5", "tools/list", ""),
            err(Some("5"), -32602, no_era),
        ),
        case("no-method", &[], legacy("6", "frobnicate", ""), err(Some("6"), -32601, "no method `frobnicate`")),
        case(
            "initialize-malformed",
            &[],
            legacy("7", "initialize", r#""protocolVersion":"2025-11-25","capabilities":{}"#),
            err(Some("7"), -32602, malformed_initialize),
        ),
        case(
            "initialize-malformed",
            &[],
            legacy("7", "initialize", r#""protocolVersion":"2025-11-25","capabilities":[],"clientInfo":{}"#),
            err(Some("7"), -32602, malformed_initialize),
        ),
        case(
            "initialize-malformed",
            &[],
            legacy("7", "initialize", r#""capabilities":{},"clientInfo":{}"#),
            err(Some("7"), -32602, malformed_initialize),
        ),
        case(
            "initialize",
            &[],
            INITIALIZE.replace(r#""id":0"#, r#""id":8"#),
            ok(
                "8",
                r#"{"protocolVersion":"2025-11-25","capabilities":{"tools":{"listChanged":false}},"serverInfo":{"name":"fake","version":"9.9.9"},"instructions":"call echo"}"#,
            ),
        ),
        case(
            "meta-no-version",
            &[],
            r#"{"jsonrpc":"2.0","id":9,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/clientCapabilities":{}}}}"#.into(),
            err(Some("9"), -32602, "`_meta` lacks `io.modelcontextprotocol/protocolVersion`"),
        ),
        case(
            "meta-version-not-a-string",
            &[],
            r#"{"jsonrpc":"2.0","id":10,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":2026,"io.modelcontextprotocol/clientCapabilities":{}}}}"#.into(),
            err(Some("10"), -32602, "`_meta`'s `io.modelcontextprotocol/protocolVersion` is not a string"),
        ),
        case(
            "meta-no-capabilities",
            &[INITIALIZE],
            r#"{"jsonrpc":"2.0","id":11,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28"}}}"#.into(),
            err(Some("11"), -32602, "`_meta` lacks `io.modelcontextprotocol/clientCapabilities` as an object"),
        ),
        case(
            "meta-no-capabilities",
            &[],
            r#"{"jsonrpc":"2.0","id":12,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2026-07-28","io.modelcontextprotocol/clientCapabilities":[]}}}"#.into(),
            err(Some("12"), -32602, "`_meta` lacks `io.modelcontextprotocol/clientCapabilities` as an object"),
        ),
        case(
            "unsupported-version",
            &[],
            r#"{"jsonrpc":"2.0","id":13,"method":"tools/list","params":{"_meta":{"io.modelcontextprotocol/protocolVersion":"2024-11-05","io.modelcontextprotocol/clientCapabilities":{}}}}"#.into(),
            r#"{"jsonrpc":"2.0","id":13,"error":{"code":-32022,"message":"protocol version `2024-11-05` is not served per request: `2026-07-28` is, and `2025-11-25` after `initialize`","data":{"supported":["2026-07-28","2025-11-25"],"requested":"2024-11-05"}}}"#.into(),
        ),
        case("modern-no-method", &[], modern("14", "frobnicate", ""), err(Some("14"), -32601, "no method `frobnicate` in `2026-07-28`")),
        case(
            "modern-result",
            &[],
            modern("15", "server/discover", ""),
            modern_ok(
                "15",
                r#""supportedVersions":["2026-07-28","2025-11-25"],"capabilities":{"tools":{"listChanged":false}},"instructions":"call echo","ttlMs":0,"cacheScope":"public""#,
            ),
        ),
        case("modern-result", &[], modern("16", "tools/list", ""), modern_ok("16", &format!(r#""tools":{LIST},"ttlMs":0,"cacheScope":"public""#))),
        case(
            "modern-result",
            &[],
            modern("17", "tools/call", r#""name":"echo","arguments":{"text":"hi"}"#),
            modern_ok("17", r#""content":[{"type":"text","text":"hi"}]"#),
        ),
        case(
            "modern-result",
            &[],
            modern("18", "tools/call", r#""name":"echo""#),
            modern_ok("18", r#""content":[{"type":"text","text":"`text` is missing"}],"isError":true"#),
        ),
        // A tool's own `_meta` keeps its place, the server's identity merged into it, `resultType` after it.
        case(
            "modern-result",
            &[],
            modern("19", "tools/call", r#""name":"meta""#),
            ok(
                "19",
                &format!(r#"{{"content":[{{"type":"text","text":"m"}}],"_meta":{{"x.y/own":"kept",{INFO}}},"resultType":"complete"}}"#),
            ),
        ),
        case("legacy-result", &[INITIALIZE], legacy("20", "tools/list", ""), ok("20", &format!(r#"{{"tools":{LIST}}}"#))),
        case(
            "legacy-result",
            &[INITIALIZE],
            legacy("21", "tools/call", r#""name":"echo","arguments":{"text":"x"}"#),
            ok("21", r#"{"content":[{"type":"text","text":"x"}]}"#),
        ),
        case(
            "legacy-result",
            &[INITIALIZE],
            legacy("21", "tools/call", r#""name":"meta""#),
            ok("21", r#"{"content":[{"type":"text","text":"m"}],"_meta":{"x.y/own":"kept"}}"#),
        ),
        case("call-no-name", &[], modern("22", "tools/call", r#""arguments":{}"#), err(Some("22"), -32602, "`tools/call` names its tool in `name`")),
        case("call-no-name", &[INITIALIZE], legacy("22", "tools/call", r#""name":7"#), err(Some("22"), -32602, "`tools/call` names its tool in `name`")),
        // `call` is never reached for a name the server does not know: the fake would panic.
        case("call-unknown-tool", &[], modern("23", "tools/call", r#""name":"nothing""#), err(Some("23"), -32602, "no tool `nothing`")),
        case("call-unknown-tool", &[INITIALIZE], legacy("23", "tools/call", r#""name":"nothing""#), err(Some("23"), -32602, "no tool `nothing`")),
        case(
            "call-arguments-not-an-object",
            &[],
            modern("24", "tools/call", r#""name":"echo","arguments":[1]"#),
            err(Some("24"), -32602, "a tool's arguments are an object"),
        ),
        case(
            "call-arguments-not-an-object",
            &[INITIALIZE],
            legacy("24", "tools/call", r#""name":"echo","arguments":"x""#),
            err(Some("24"), -32602, "a tool's arguments are an object"),
        ),
        case("failure-params", &[], modern("25", "tools/call", r#""name":"params""#), err(Some("25"), -32602, "the fake takes none")),
        case("failure-params", &[INITIALIZE], legacy("27", "tools/call", r#""name":"params""#), err(Some("27"), -32602, "the fake takes none")),
        case(
            "list-cursor",
            &[],
            modern("26", "tools/list", r#""cursor":"x""#),
            err(Some("26"), -32602, "the tool list has one page, so no cursor names a place in it"),
        ),
        case(
            "list-cursor",
            &[INITIALIZE],
            legacy("26", "tools/list", r#""cursor":"x""#),
            err(Some("26"), -32602, "the tool list has one page, so no cursor names a place in it"),
        ),
        case("failure-internal", &[], modern("28", "tools/call", r#""name":"broken""#), err(Some("28"), -32603, "the fake broke")),
        case("failure-internal", &[INITIALIZE], legacy("29", "tools/call", r#""name":"broken""#), err(Some("29"), -32603, "the fake broke")),
        case("result-holds-resulttype", &[], modern("30", "tools/call", r#""name":"typed""#), err(Some("30"), -32603, &resulttype("typed"))),
        case("result-holds-resulttype", &[INITIALIZE], legacy("31", "tools/call", r#""name":"typed""#), err(Some("31"), -32603, &resulttype("typed"))),
        case(
            "result-no-object",
            &[],
            modern("32", "tools/call", r#""name":"scalar""#),
            err(Some("32"), -32603, "the tool `scalar` answered a result that is no JSON object"),
        ),
        case(
            "result-no-object",
            &[INITIALIZE],
            legacy("32", "tools/call", r#""name":"scalar""#),
            err(Some("32"), -32603, "the tool `scalar` answered a result that is no JSON object"),
        ),
        case("result-meta-owned", &[], modern("33", "tools/call", r#""name":"clash""#), err(Some("33"), -32603, meta_owned)),
        case("result-meta-owned", &[], modern("34", "tools/call", r#""name":"badmeta""#), err(Some("34"), -32603, meta_owned)),
        // In the earlier era too, where the framing writes nothing into the result (review round 18's R18-D1).
        case("result-meta-owned", &[INITIALIZE], legacy("35", "tools/call", r#""name":"clash""#), err(Some("35"), -32603, meta_owned)),
        case("result-meta-owned", &[INITIALIZE], legacy("36", "tools/call", r#""name":"badmeta""#), err(Some("36"), -32603, meta_owned)),
        case("result-does-not-read-back", &[], modern("37", "tools/call", r#""name":"dup""#), err(Some("37"), -32603, &read_back)),
        case("result-does-not-read-back", &[INITIALIZE], legacy("38", "tools/call", r#""name":"dup""#), err(Some("38"), -32603, &read_back)),
    ]
}

/// Serve `input`, read through a buffer of `capacity` bytes: the answers, one per line.
fn served(input: &str, capacity: usize) -> Vec<String> {
    let mut output = Vec::new();
    let mut reader = BufReader::with_capacity(capacity, Cursor::new(input.as_bytes().to_vec()));
    serve(&mut reader, &mut output, &Fake, LIMIT).expect("served");
    String::from_utf8(output)
        .unwrap()
        .lines()
        .map(str::to_owned)
        .collect()
}

/// The source's sites, read as text: every call of the two writers and of `refused`, each naming its site by a
/// literal; and every way the source breaks the census's premises.
fn census() -> (Vec<String>, Vec<String>) {
    let source =
        std::fs::read_to_string(concat!(env!("CARGO_MANIFEST_DIR"), "/src/lib.rs")).unwrap();
    let (mut sites, mut wrong) = (Vec::new(), Vec::new());
    for call in ["error(", "result(", "refused("] {
        for (at, _) in source.match_indices(call) {
            let before = source[..at].chars().next_back();
            if before.is_some_and(|c| c.is_alphanumeric() || c == '_')
                || source[..at].ends_with("fn ")
            {
                continue; // another name ending so, or the definition itself
            }
            let after = source[at + call.len()..].trim_start();
            match after
                .strip_prefix('"')
                .and_then(|rest| rest.split_once('"'))
            {
                Some((site, _)) => sites.push(site.to_owned()),
                None => wrong.push(format!(
                    "a call `{call}` at byte {at} names its site by no literal: {}",
                    &after[..after.len().min(40)]
                )),
            }
        }
    }
    let writes = source.matches("json::write(").count();
    if writes != 3 {
        wrong.push(format!(
            "`json::write(` is called {writes} times; the two writers and the read-back of a tool's result are its only callers"
        ));
    }
    (sites, wrong)
}

#[test]
fn every_case_is_answered_with_its_whole_line_through_the_site_it_names() {
    let mut wrong = Vec::new();
    let mut reached = BTreeSet::new();
    for case in cases() {
        let (lines, sites) = match case.via {
            Via::Serve => traced(|| served(&format!("{}\n", case.request), 1 << 16)),
            Via::Server => {
                let mut server = Server::new(&Fake, LIMIT);
                for line in &case.before {
                    server.answer(line.as_bytes());
                }
                let (answer, sites) = traced(|| server.answer(case.request.as_bytes()));
                (answer.into_iter().collect(), sites)
            }
        };
        reached.extend(sites.iter().map(|s| (*s).to_owned()));
        if lines.as_slice() != [case.answer.clone()] {
            wrong.push(format!(
                "{}: {}\n  want {}\n  got  {lines:?}",
                case.site, case.request, case.answer
            ));
        } else if let Err(refusal) = json::read(case.answer.as_bytes(), usize::MAX) {
            wrong.push(format!(
                "{}: the answer does not read back: {refusal:?}",
                case.site
            ));
        }
        if !sites.contains(&case.site) {
            wrong.push(format!(
                "{}: the request went through {sites:?}, not the site it names",
                case.site
            ));
        }
    }
    let (source_sites, _) = census();
    for site in source_sites.iter().collect::<BTreeSet<_>>() {
        if !reached.contains(site) {
            wrong.push(format!("the site `{site}` is reached by no case"));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn the_census_finds_every_site_named_once_and_every_case_names_one() {
    let (sites, mut wrong) = census();
    let mut seen = BTreeSet::new();
    for site in &sites {
        // The two `Failure` answers are the one site of every refusal a tool or the framing makes.
        if !seen.insert(site.as_str()) && site != "failure-params" && site != "failure-internal" {
            wrong.push(format!("the site `{site}` is named twice"));
        }
    }
    let named: BTreeSet<&str> = cases().iter().map(|c| c.site).collect();
    for site in named.difference(&seen) {
        wrong.push(format!("the case `{site}` names no site of the source"));
    }
    assert!(
        seen.len() >= 30,
        "the census found {} sites: {seen:?}",
        seen.len()
    );
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// A ping whose line is exactly `bytes` long.
fn ping_of(bytes: usize) -> String {
    let shell = r#"{"jsonrpc":"2.0","id":1,"method":"ping","params":{"x":""}}"#;
    let pad = "a".repeat(bytes - shell.len());
    format!(r#"{{"jsonrpc":"2.0","id":1,"method":"ping","params":{{"x":"{pad}"}}}}"#)
}

#[test]
fn serve_holds_the_bound_exactly_and_frames_lines_however_the_input_is_read() {
    let pong = ok("1", "{}");
    let refused = err(
        None,
        -32700,
        &format!("the message is longer than {LIMIT} bytes, and was not read"),
    );
    // A line exactly at the bound is read, one byte more is refused unread; blank lines — spaces, tabs, a CR — are no
    // messages; notifications and a client's responses are unanswered; the last line needs no newline, a too-long one
    // included. Read whole, and through a buffer of 7 bytes, which splits every line.
    let unanswered = [
        r#"{"jsonrpc":"2.0","method":"notifications/initialized"}"#,
        r#"{"jsonrpc":"2.0","id":7,"result":{}}"#,
        r#"{"jsonrpc":"2.0","id":7,"error":{"code":-1,"message":"no"}}"#,
    ]
    .join("\n");
    for last in [
        r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#.to_owned(),
        ping_of(LIMIT + 1),
    ] {
        let input = format!(
            "{}\n\n \r\n\t\n{unanswered}\n{}\n{last}",
            ping_of(LIMIT),
            ping_of(LIMIT + 1)
        );
        let want = if last.len() > LIMIT { &refused } else { &pong };
        for capacity in [1 << 16, 7] {
            assert_eq!(
                served(&input, capacity),
                [pong.clone(), refused.clone(), want.clone()],
                "a buffer of {capacity}"
            );
        }
    }
}

/// A writer that records what it holds at each flush.
#[derive(Default)]
struct Flushes {
    bytes: Vec<u8>,
    flushed: Vec<usize>,
}

impl Write for Flushes {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.bytes.extend_from_slice(buf);
        Ok(buf.len())
    }
    fn flush(&mut self) -> std::io::Result<()> {
        self.flushed.push(self.bytes.len());
        Ok(())
    }
}

#[test]
fn each_answer_is_flushed_as_it_is_written() {
    // A client waits for each answer before it sends the next request: an answer held in a buffer stalls it.
    let mut output = Flushes::default();
    let input = format!("{}\n{}\n", legacy("1", "ping", ""), legacy("2", "ping", ""));
    serve(&mut Cursor::new(input), &mut output, &Fake, LIMIT).expect("served");
    let first = ok("1", "{}").len() + 1;
    assert_eq!(output.flushed, [first, first * 2]);
}

#[test]
fn an_answer_is_one_line_whatever_the_tool_s_text_holds() {
    // `\n` and `\r` in a tool's text are escaped, so an answer never spans two lines; U+2028, U+2029 and U+0085 are
    // written as they are, the MCP record's accepted choice, which a client splitting on Unicode line breaks misreads.
    let line = modern(
        "1",
        "tools/call",
        r#""name":"echo","arguments":{"text":"a\nb\rc d e\u0085f"}"#,
    );
    let lines = served(&format!("{line}\n"), 1 << 16);
    assert_eq!(lines.len(), 1, "{lines:?}");
    assert!(
        lines[0].contains("a\\nb\\rc\u{2028}d\u{2029}e\u{85}f"),
        "{}",
        lines[0]
    );
}
