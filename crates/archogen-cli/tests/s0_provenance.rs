//! Leaf `S0.5`: the generated output maps back to the description that produced it.
//!
//! F28's last unmet clause is *"failure points have useful diagnostics and **basic source
//! provenance**"*. The diagnostics half has been true since `M1` and is asserted in
//! `s0_reader.rs`; this file asserts the other half.
//!
//! # A provenance record is worth having only if it resolves
//!
//! The failure mode is not an absent record. It is a record that **names a line which does not
//! contain what it claims** — that sends a reader somewhere confidently wrong, which is strictly
//! worse than sending them nowhere. So every record is resolved from both ends:
//!
//! | End | Asserted |
//! |---|---|
//! | generated | the named line of `src/main.rs` exists and carries that declaration |
//! | source | the named byte span of the description exists and covers that declaration |
//!
//! and the artifact itself is checked for the structural property a consumer depends on: that it
//! is well-formed JSON.

use std::path::{Path, PathBuf};

use archogen_cli::{run, Status};

const BASE: &str = "examples/s0-heartbeat/system.eadl";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Generate the base fixture into a clean directory and return it.
fn generate(case: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("s0-provenance")
        .join(case);
    let _ = std::fs::remove_dir_all(&dir);
    let source = repo_root().join(BASE).display().to_string();
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(
        ["build", &source, "--out", &dir.display().to_string()].map(String::from),
        &mut out,
        &mut err,
    );
    assert_eq!(status, Status::Ok, "{}", String::from_utf8_lossy(&err));
    dir
}

/// Every `"key": value` pair of one flat JSON object, by brute-force scanning.
///
/// Deliberately not a JSON parser: the engine carries no dependencies, and a parser written to
/// support a test is a second implementation of a format nobody asked for. This reads the
/// handful of fields the assertions below need, and [`json_is_well_formed`] carries the
/// structural check separately.
fn records(text: &str) -> Vec<Vec<(String, String)>> {
    let mut all = Vec::new();
    let mut current: Vec<(String, String)> = Vec::new();
    let mut inside = false;
    for line in text.lines() {
        let trimmed = line.trim();
        if trimmed == "{" && inside {
            current = Vec::new();
        }
        if trimmed.starts_with("\"records\"") {
            inside = true;
            continue;
        }
        if !inside {
            continue;
        }
        if trimmed.starts_with('}') && !current.is_empty() {
            all.push(std::mem::take(&mut current));
            continue;
        }
        if let Some((key, value)) = trimmed.split_once(": ") {
            current.push((
                key.trim().trim_matches('"').to_string(),
                value.trim_end_matches(',').trim().to_string(),
            ));
        }
    }
    all
}

fn field(record: &[(String, String)], key: &str) -> String {
    record
        .iter()
        .find(|(name, _)| name == key)
        .map(|(_, value)| value.trim_matches('"').to_string())
        .unwrap_or_else(|| panic!("record has no `{key}`: {record:?}"))
}

/// Brace/bracket balance outside strings, plus no raw control character inside one.
///
/// The second half is the clause naive escapers miss and the one that makes a file unreadable to
/// a real parser, so it is checked rather than assumed.
fn json_is_well_formed(text: &str) -> Result<(), String> {
    let mut depth: i32 = 0;
    let mut in_string = false;
    let mut escaped = false;
    for (index, ch) in text.char_indices() {
        if in_string {
            if escaped {
                escaped = false;
            } else if ch == '\\' {
                escaped = true;
            } else if ch == '"' {
                in_string = false;
            } else if (ch as u32) < 0x20 {
                return Err(format!(
                    "raw control character U+{:04X} at {index}",
                    ch as u32
                ));
            }
            continue;
        }
        match ch {
            '"' => in_string = true,
            '{' | '[' => depth += 1,
            '}' | ']' => {
                depth -= 1;
                if depth < 0 {
                    return Err(format!("unbalanced close at {index}"));
                }
            }
            _ => {}
        }
    }
    if in_string {
        return Err("unterminated string".into());
    }
    if depth != 0 {
        return Err(format!("{depth} unclosed brace(s)"));
    }
    Ok(())
}

#[test]
fn the_generated_crate_carries_a_provenance_artifact() {
    let dir = generate("artifact");
    let text = std::fs::read_to_string(dir.join("provenance.json")).expect("provenance is written");
    json_is_well_formed(&text)
        .unwrap_or_else(|why| panic!("provenance.json is not JSON: {why}\n{text}"));
    assert!(
        text.contains("archogen-provenance/1"),
        "a consumer must be able to recognize the format:\n{text}"
    );
    // §12 S0 requires the output marked experimental — including the artifact a later tool reads.
    assert!(text.contains("\"experimental\": true"), "{text}");
    assert!(
        text.contains("stub of a §9 catalog entry"),
        "the realization record must say what it is not; a record that looks like a catalog \
         entry is one somebody will cite as if it were one:\n{text}"
    );
}

#[test]
fn every_record_resolves_to_the_generated_line_it_names() {
    let dir = generate("generated-end");
    let text = std::fs::read_to_string(dir.join("provenance.json")).expect("written");
    let main = std::fs::read_to_string(dir.join("src/main.rs")).expect("written");
    let lines: Vec<&str> = main.lines().collect();

    let found = records(&text);
    assert_eq!(
        found.len(),
        4,
        "system, hyperperiod, and one per task: {found:?}"
    );
    for record in &found {
        let declaration = field(record, "declaration");
        let name = field(record, "name");
        let generated = field(record, "generated");
        let line_number: usize = generated
            .rsplit_once("\"line\": ")
            .map(|(_, rest)| rest.trim_end_matches(" }").trim())
            .and_then(|n| n.parse().ok())
            .unwrap_or_else(|| panic!("no line number in {generated}"));
        let line = lines.get(line_number - 1).unwrap_or_else(|| {
            panic!("{declaration}: line {line_number} is past the end of main.rs")
        });
        let needle = if declaration == "hyperperiod" {
            "HORIZON_MS".to_string()
        } else {
            name
        };
        assert!(
            line.contains(&needle),
            "{declaration}: provenance points at main.rs:{line_number} — `{line}` — which does \
             not contain `{needle}`"
        );
    }
}

#[test]
fn every_record_resolves_to_the_source_span_it_names() {
    // The half that makes it *source* provenance. A span that does not cover the declaration is
    // a number that looks like evidence.
    let dir = generate("source-end");
    let text = std::fs::read_to_string(dir.join("provenance.json")).expect("written");
    let description = std::fs::read(repo_root().join(BASE)).expect("readable");

    for record in &records(&text) {
        let declaration = field(record, "declaration");
        let name = field(record, "name");
        let source = field(record, "source");
        let number = |key: &str| -> usize {
            source
                .split_once(&format!("\"{key}\": "))
                .map(|(_, rest)| rest.split(&[',', ' ', '}'][..]).next().unwrap_or(""))
                .and_then(|n| n.parse().ok())
                .unwrap_or_else(|| panic!("no `{key}` in {source}"))
        };
        let (start, end) = (number("start"), number("end"));
        assert!(
            end <= description.len() && start < end,
            "{declaration}: span {start}..{end} is not inside a {}-byte description",
            description.len()
        );
        let covered = std::str::from_utf8(&description[start..end]).expect("utf-8 span");
        // The system name and the hyperperiod are both properties of the `defsystem` form, so
        // both records point at it; a task points at its own clause.
        let needle = match declaration.as_str() {
            "hyperperiod" | "system" => "defsystem".to_string(),
            _ => format!("task {name}"),
        };
        assert!(
            covered.contains(&needle),
            "{declaration}: span {start}..{end} covers `{covered}`, which does not contain \
             `{needle}`"
        );

        // And the line number agrees with the offset, so a reader who uses either arrives at the
        // same place.
        let line = number("line");
        let computed = description[..start].iter().filter(|b| **b == b'\n').count() + 1;
        assert_eq!(
            line, computed,
            "{declaration}: the record says line {line}, but byte {start} is on line {computed}"
        );
    }
}

#[test]
fn the_provenance_of_a_task_points_at_that_task_and_not_at_its_neighbour() {
    // ⭐ An off-by-one that mapped every task to the first one would satisfy "each record resolves
    // to a task", so the records are checked to be *distinct* and in declaration order.
    let dir = generate("distinct");
    let text = std::fs::read_to_string(dir.join("provenance.json")).expect("written");
    let description = std::fs::read_to_string(repo_root().join(BASE)).expect("readable");

    let tasks: Vec<Vec<(String, String)>> = records(&text)
        .into_iter()
        .filter(|r| field(r, "declaration") == "task")
        .collect();
    assert_eq!(tasks.len(), 2);

    let start_of = |record: &[(String, String)]| -> usize {
        let source = field(record, "source");
        source
            .split_once("\"start\": ")
            .map(|(_, rest)| rest.split(&[',', ' ', '}'][..]).next().unwrap_or(""))
            .and_then(|n| n.parse().ok())
            .expect("a start offset")
    };
    assert!(
        start_of(&tasks[0]) < start_of(&tasks[1]),
        "the two tasks share, or invert, their source positions"
    );
    assert_eq!(field(&tasks[0], "name"), "beat");
    assert_eq!(field(&tasks[1], "name"), "chime");
    assert!(
        description[start_of(&tasks[0])..].starts_with("(task beat"),
        "the first record does not start at `beat`"
    );
    assert!(
        description[start_of(&tasks[1])..].starts_with("(task chime"),
        "the second record does not start at `chime`"
    );
}
