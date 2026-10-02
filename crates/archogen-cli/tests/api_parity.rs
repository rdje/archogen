//! The CLI and the engine API cannot disagree (leaf `API.3.3`, `docs/decisions/decision_engine-api.md`).
//!
//! `ROADMAP.md` §10.4: "a capability that exists only behind the CLI does not exist programmatically, and the
//! reverse." Three legs hold that, each with a RED arm that feeds its comparison a disagreement:
//!
//! 1. **Structurally** — no production source of the CLI reaches a judging entry point of the engine except
//!    through `archogen_api`. A command that judged a description its own way would be a capability the API
//!    does not have.
//! 2. **By operation** — the API's operations are exactly the commands `spec.rs` says run (built or
//!    experimental), less the ones §10.4 keeps out of the programmatic interface, each named with its reason.
//! 3. **By behaviour** — over every description in the repository, `archogen check`'s exit code, its diagnostic
//!    codes and its notes are the API's status, diagnostic codes and notes.

use std::fs;
use std::path::{Path, PathBuf};

use archogen_api::{Request, OPERATIONS};
use archogen_cli::spec::{Maturity, COMMANDS};
use archogen_cli::{run, Status};
use eadl_front::DirectoryModules;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

// ── 1. structurally ──────────────────────────────────────────────────────────────────────────────

/// What only the engine API may call: the engine's judging entry points and the elaborator.
const JUDGING: &[&str] = &[
    "eadl_model::check",
    "shipped_registry",
    "check_program",
    "elaborate_source",
];

/// Every line of production code in `files` that names a judging entry point, as `file:line: name`. The
/// production half of a file is the part before its first `#[cfg(test)]`; a comment is not a call.
fn direct_judging(files: &[(String, String)]) -> Vec<String> {
    let mut found = Vec::new();
    for (name, text) in files {
        let production = text.split("#[cfg(test)]").next().unwrap_or("");
        for (index, line) in production.lines().enumerate() {
            if line.trim_start().starts_with("//") {
                continue;
            }
            for entry in JUDGING {
                if line.contains(entry) {
                    found.push(format!("{name}:{}: {entry}", index + 1));
                }
            }
        }
    }
    found
}

/// The CLI's production sources, derived from its `src/` directory.
fn cli_sources() -> Vec<(String, String)> {
    let dir = repo_root().join("crates/archogen-cli/src");
    let mut files: Vec<(String, String)> = fs::read_dir(&dir)
        .expect("the CLI's src/ is readable")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "rs"))
        .map(|path| {
            let name = path
                .strip_prefix(repo_root())
                .expect("under the root")
                .display()
                .to_string();
            (name, fs::read_to_string(&path).expect("readable"))
        })
        .collect();
    files.sort();
    files
}

#[test]
fn the_cli_judges_nothing_except_through_the_engine_api() {
    let files = cli_sources();
    assert!(
        files.iter().any(|(name, _)| name.ends_with("check_cmd.rs")),
        "the population this leg reads is not the CLI's sources: {:?}",
        files.iter().map(|(name, _)| name).collect::<Vec<_>>()
    );
    let found = direct_judging(&files);
    assert!(
        found.is_empty(),
        "the CLI calls the engine's judging path directly, so it can do what the API cannot — route it \
         through archogen_api::check:\n{}",
        found.join("\n")
    );
}

#[test]
fn arm_1_a_cli_source_that_builds_its_own_registry_is_reported() {
    let mutated = vec![(
        "crates/archogen-cli/src/check_cmd.rs".to_string(),
        "// shipped_registry is named in a comment, which is not a call\n\
         let registry = shipped_registry(&mut sources, &modules);\n\
         #[cfg(test)]\nmod tests { fn f() { check_program(); } }\n"
            .to_string(),
    )];
    assert_eq!(
        direct_judging(&mutated),
        ["crates/archogen-cli/src/check_cmd.rs:2: shipped_registry"]
    );
}

// ── 2. by operation ──────────────────────────────────────────────────────────────────────────────

/// The commands `ROADMAP.md` §10.4 keeps out of the programmatic interface, each with its reason.
const EXCLUDED: &[(&str, &str)] = &[
    (
        "build",
        "§10.4: generation writes a crate tree and stays a human or CI action",
    ),
    (
        "mcp",
        "§10.4: the server is a transport for the operations, not one of them",
    ),
];

/// Every way the API's operations and the commands that run disagree.
fn operation_mismatches(
    runnable: &[&str],
    excluded: &[(&str, &str)],
    operations: &[&str],
) -> Vec<String> {
    let mut wrong = Vec::new();
    for command in runnable {
        let is_excluded = excluded.iter().any(|(name, _)| name == command);
        if !is_excluded && !operations.contains(command) {
            wrong.push(format!(
                "`{command}` runs behind the CLI and is no API operation"
            ));
        }
        if is_excluded && operations.contains(command) {
            wrong.push(format!(
                "`{command}` is excluded by §10.4 and is an API operation"
            ));
        }
    }
    for operation in operations {
        if !runnable.contains(operation) {
            wrong.push(format!(
                "`{operation}` is an API operation and no command runs it"
            ));
        }
    }
    for (name, reason) in excluded {
        if !runnable.contains(name) {
            wrong.push(format!(
                "`{name}` is excluded, and no command of that name runs — a stale exclusion"
            ));
        }
        if reason.is_empty() {
            wrong.push(format!("`{name}` is excluded without a reason"));
        }
    }
    wrong
}

/// The commands `spec.rs` says run: built, or experimental.
fn runnable() -> Vec<&'static str> {
    COMMANDS
        .iter()
        .filter(|command| !matches!(command.maturity, Maturity::Unimplemented { .. }))
        .map(|command| command.name)
        .collect()
}

#[test]
fn the_api_offers_exactly_the_commands_that_run_less_the_ones_section_10_4_excludes() {
    let runnable = runnable();
    assert!(
        runnable.contains(&"check"),
        "spec.rs no longer says `check` runs: {runnable:?}"
    );
    let wrong = operation_mismatches(&runnable, EXCLUDED, OPERATIONS);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn arm_2_an_operation_missing_or_extra_or_a_stale_exclusion_is_reported() {
    let runnable = ["check", "build"];
    // `check` runs and is no operation; `resolve` is an operation nothing runs; `frobnicate` is excluded and
    // does not exist.
    let wrong = operation_mismatches(
        &runnable,
        &[("build", "§10.4"), ("frobnicate", "why")],
        &["resolve"],
    );
    assert_eq!(wrong.len(), 3, "{wrong:#?}");
    // And an excluded command offered anyway.
    let wrong = operation_mismatches(&runnable, &[("build", "§10.4")], &["check", "build"]);
    assert_eq!(
        wrong,
        ["`build` is excluded by §10.4 and is an API operation"]
    );
}

// ── 3. by behaviour ──────────────────────────────────────────────────────────────────────────────

/// What one surface said about a description: its status, its diagnostic codes in order, and its notes.
#[derive(Debug, PartialEq, Eq)]
struct Said {
    status: Status,
    codes: Vec<String>,
    notes: Vec<String>,
}

/// `archogen check` on `path`, in-process.
fn cli_says(path: &Path) -> Said {
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(
        ["check".to_string(), path.display().to_string()],
        &mut out,
        &mut err,
    );
    let err = String::from_utf8_lossy(&err);
    let prefix = format!("archogen: {}: ", status.slug());
    Said {
        status,
        codes: err
            .lines()
            .filter_map(|line| line.strip_prefix("error[")?.split_once(']'))
            .map(|(code, _)| code.to_string())
            .collect(),
        // A judged refusal ends with a count line, which is the CLI's own summary, not a note.
        notes: err
            .lines()
            .filter_map(|line| line.strip_prefix(prefix.as_str()))
            .filter(|rest| !rest.contains(" diagnostic(s) in "))
            .map(str::to_string)
            .collect(),
    }
}

/// The engine API on `path`, with the module path the CLI uses.
fn api_says(path: &Path) -> Said {
    let text = fs::read_to_string(path).expect("readable");
    let name = path.display().to_string();
    let modules = DirectoryModules::new(path.parent().expect("a file has a directory"));
    let response = archogen_api::check(&Request {
        name: &name,
        text: &text,
        profile: None,
        modules: &modules,
    });
    Said {
        status: response.status,
        codes: response
            .diagnostics
            .iter()
            .map(|diagnostic| diagnostic.code.to_string())
            .collect(),
        notes: response.notes,
    }
}

/// Every description in the repository, as the verdict ratchet walks them.
fn descriptions(root: &Path) -> Vec<PathBuf> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<PathBuf>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                let top = dir == root;
                if name.starts_with('.')
                    || (top && ["target", "build", "vendor"].contains(&name.as_str()))
                {
                    continue;
                }
                walk(root, &path, out);
            } else if name.ends_with(".eadl") {
                out.push(path);
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
}

/// Every description on which the two surfaces said different things.
fn behaviour_mismatches(pairs: &[(String, Said, Said)]) -> Vec<String> {
    pairs
        .iter()
        .filter(|(_, cli, api)| cli != api)
        .map(|(name, cli, api)| format!("{name}:\n  cli {cli:?}\n  api {api:?}"))
        .collect()
}

#[test]
fn over_every_description_the_cli_says_what_the_api_says() {
    let root = repo_root();
    let population = descriptions(&root);
    assert!(
        population.len() > 1,
        "no descriptions found under {}",
        root.display()
    );
    let pairs: Vec<(String, Said, Said)> = population
        .iter()
        .map(|path| (path.display().to_string(), cli_says(path), api_says(path)))
        .collect();
    // More than one status, or the leg compares one outcome and calls it parity.
    let statuses: std::collections::BTreeSet<i32> =
        pairs.iter().map(|(_, cli, _)| cli.status.code()).collect();
    assert!(statuses.len() > 2, "{statuses:?}");
    let wrong = behaviour_mismatches(&pairs);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

#[test]
fn arm_3_a_surface_that_drops_a_diagnostic_or_a_note_is_reported() {
    let said = |status, codes: &[&str], notes: &[&str]| Said {
        status,
        codes: codes.iter().map(|c| (*c).to_string()).collect(),
        notes: notes.iter().map(|n| (*n).to_string()).collect(),
    };
    let pairs = vec![
        (
            "agrees".to_string(),
            said(Status::Ok, &[], &[]),
            said(Status::Ok, &[], &[]),
        ),
        (
            "drops a code".to_string(),
            said(Status::InvalidDescription, &["read-unclosed-list"], &[]),
            said(Status::InvalidDescription, &[], &[]),
        ),
        (
            "drops a note".to_string(),
            said(Status::Unimplemented, &[], &[]),
            said(Status::Unimplemented, &[], &["declares a kind"]),
        ),
    ];
    let wrong = behaviour_mismatches(&pairs);
    assert_eq!(wrong.len(), 2, "{wrong:#?}");
}
