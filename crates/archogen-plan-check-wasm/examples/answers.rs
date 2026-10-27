//! The host build's side of `scripts/plan_check_wasm.sh`'s comparisons (`docs/decisions/decision_wasm-binding.md`
//! §11): the population, derived on every run, and the host build's answer to each case, one per line.
//!
//! ```text
//! answers --module ARTIFACT --out DIR DESCRIPTION...
//! ```
//!
//! No plan is tracked, so the plans are derived here without a search: for each description, the empty plan, which
//! check (1) refuses, and the bare plan — the profile, the description's first system, the program's digest and no
//! binding — which the checks read further; and the accepted plans the checker's own tests write, a plain description's
//! and a module's. Each case's request names the description by its file's name, and as modules every `*.eadl` file of
//! its own directory under its stem, which is where the executable's directory source finds a module. The producer is
//! the digest of `ARTIFACT`, the module the harness loads, as the loader computes it.
//!
//! For the executable's side, each case is written under `DIR/<n>/` — the description, its directory's modules and
//! `plan.json` — and described in `DIR/cases.jsonl`, one JSON object per line, in the order the answers are printed.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::ExitCode;

use archogen_digest::Digest;
use archogen_json::{self as json, Value};
use archogen_plan_check::assignment::{program_digest, Assignment, Requester};
use archogen_plan_check::check::plan_file;
use archogen_plan_check::judgement::PLAN_NAME;
use archogen_plan_check_wasm::answer;
use archogen_plan_check_wasm::request::{frame, Framed};
use eadl_front::module::MemoryModules;
use eadl_front::SourceMap;
use eadl_model::profile::RT_STATIC_UP_V1;

/// A plain description and the provider its accepted plan binds, as `crates/archogen-plan-check/tests/mcp.rs` writes
/// them.
const PLAIN: &str = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
";

/// A module the module description below imports.
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

/// One case: what it is, the description's file name and text, its modules, and the plan.
struct Case {
    label: String,
    name: String,
    description: String,
    modules: Vec<(String, String)>,
    plan: String,
}

/// The program `description` becomes with `modules`, as the checker reads it, and its digest.
fn digest_of(name: &str, description: &str, modules: &[(String, String)]) -> String {
    let memory = modules
        .iter()
        .fold(MemoryModules::new(), |memory, (module, text)| {
            memory.with(module, text)
        });
    let mut sources = SourceMap::new();
    let Ok(id) = sources.add(name, description) else {
        return "0".repeat(64);
    };
    program_digest(&plan_file(&mut sources, id, &memory, &RT_STATIC_UP_V1, PLAN_NAME, "").0)
}

/// The plan for the program `description` becomes, naming `system` and `bindings`.
fn plan(
    name: &str,
    description: &str,
    modules: &[(String, String)],
    system: &str,
    bindings: &[(&str, &str, &str)],
) -> String {
    Assignment {
        profile: RT_STATIC_UP_V1.id.to_owned(),
        system: system.to_owned(),
        program: digest_of(name, description, modules),
        placement: None,
        bindings: bindings
            .iter()
            .map(|(requester, fact, provider)| {
                (
                    (
                        Requester::Named((*requester).to_owned()),
                        (*fact).to_owned(),
                    ),
                    (*provider).into(),
                )
            })
            .collect(),
    }
    .to_canonical()
}

/// The first system `text` declares, or `s` when it declares none.
fn first_system(text: &str) -> String {
    text.split("(defsystem")
        .nth(1)
        .and_then(|rest| {
            rest.split(|c: char| c.is_whitespace() || c == '(' || c == ')')
                .find(|word| !word.is_empty())
        })
        .unwrap_or("s")
        .to_owned()
}

/// The two cases of the tracked description at `path`: its empty plan and its bare one.
fn tracked(path: &str) -> Result<Vec<Case>, String> {
    let description = fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    let file = Path::new(path);
    let name = file
        .file_name()
        .and_then(|name| name.to_str())
        .ok_or_else(|| format!("{path}: no file name"))?
        .to_owned();
    let dir = match file.parent() {
        Some(dir) if !dir.as_os_str().is_empty() => dir,
        _ => Path::new("."),
    };
    let mut paths: Vec<PathBuf> = fs::read_dir(dir)
        .map_err(|error| format!("{}: {error}", dir.display()))?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    paths.sort();
    let mut modules = Vec::new();
    for module in paths {
        if module.extension().and_then(|e| e.to_str()) != Some("eadl") || !module.is_file() {
            continue;
        }
        let Some(stem) = module.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let text = fs::read_to_string(&module)
            .map_err(|error| format!("{}: {error}", module.display()))?;
        modules.push((stem.to_owned(), text));
    }
    let bare = plan(
        &name,
        &description,
        &modules,
        &first_system(&description),
        &[],
    );
    Ok(vec![
        Case {
            label: format!("{path}, the empty plan"),
            name: name.clone(),
            description: description.clone(),
            modules: modules.clone(),
            plan: String::new(),
        },
        Case {
            label: format!("{path}, the bare plan"),
            name,
            description,
            modules,
            plan: bare,
        },
    ])
}

/// The accepted plans the checker's own tests write.
fn accepted() -> Vec<Case> {
    let name = "description.eadl";
    let board = vec![("hw.board".to_owned(), BOARD.to_owned())];
    let binding = |provider| [("console.write", "observable-output", provider)];
    vec![
        Case {
            label: "a plain description's accepted plan".to_owned(),
            name: name.to_owned(),
            description: PLAIN.to_owned(),
            modules: Vec::new(),
            plan: plan(name, PLAIN, &[], "s", &binding("console.uart")),
        },
        Case {
            label: "a module's accepted plan".to_owned(),
            name: name.to_owned(),
            description: APP.to_owned(),
            plan: plan(name, APP, &board, "s", &binding("hw.console.uart")),
            modules: board,
        },
    ]
}

/// Write `case` under `dir` for the executable, and describe it.
fn write(case: &Case, dir: &Path) -> Result<Value, String> {
    let failed = |error: std::io::Error| format!("{}: {error}", dir.display());
    fs::create_dir_all(dir).map_err(failed)?;
    for (module, text) in &case.modules {
        fs::write(dir.join(format!("{module}.eadl")), text).map_err(failed)?;
    }
    fs::write(dir.join(&case.name), &case.description).map_err(failed)?;
    fs::write(dir.join(PLAN_NAME), &case.plan).map_err(failed)?;
    let text = |value: &str| Value::String(value.to_owned());
    Ok(Value::Object(vec![
        ("case".to_owned(), text(&case.label)),
        ("dir".to_owned(), text(&dir.display().to_string())),
        ("name".to_owned(), text(&case.name)),
        ("description".to_owned(), text(&case.description)),
        (
            "modules".to_owned(),
            Value::Object(
                case.modules
                    .iter()
                    .map(|(module, module_text)| (module.clone(), text(module_text)))
                    .collect(),
            ),
        ),
        ("plan".to_owned(), text(&case.plan)),
    ]))
}

fn run(args: &[String]) -> Result<(), String> {
    let usage = "usage: answers --module ARTIFACT --out DIR DESCRIPTION...";
    let (Some("--module"), Some(artifact), Some("--out"), Some(out)) = (
        args.first().map(String::as_str),
        args.get(1),
        args.get(2).map(String::as_str),
        args.get(3),
    ) else {
        return Err(usage.to_owned());
    };
    let producer = Digest::of(&fs::read(artifact).map_err(|error| format!("{artifact}: {error}"))?);
    let mut cases = Vec::new();
    for path in &args[4..] {
        cases.extend(tracked(path)?);
    }
    cases.extend(accepted());
    let out = Path::new(out);
    let mut described = String::new();
    for (index, case) in cases.iter().enumerate() {
        described.push_str(&json::write(&write(case, &out.join(index.to_string()))?));
        described.push('\n');
        let answered = answer(&frame(&Framed {
            producer,
            name: case.name.clone(),
            text: case.description.clone(),
            profile: None,
            modules: case.modules.clone(),
            plan: case.plan.clone(),
        }));
        print!("{answered}");
    }
    fs::write(out.join("cases.jsonl"), described)
        .map_err(|error| format!("{}: {error}", out.display()))
}

fn main() -> ExitCode {
    let args: Vec<String> = std::env::args().skip(1).collect();
    match run(&args) {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("answers: {error}");
            ExitCode::FAILURE
        }
    }
}
