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
    (
        "plan-check",
        "findings §16 (a): the configuration checker's own program, which the generator's API never runs",
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

/// `archogen check` on `path`, on `target` when one is named, in-process.
fn cli_says(path: &Path, target: Option<&Path>) -> Said {
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let mut args = vec!["check".to_string(), path.display().to_string()];
    if let Some(target) = target {
        args.push("--target".to_string());
        args.push(target.display().to_string());
    }
    let status = run(args, &mut out, &mut err);
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

/// The engine API on `path`, on `target` when one is named, with the module path the CLI uses.
fn api_says(path: &Path, target: Option<&Path>) -> Said {
    let text = fs::read_to_string(path).expect("readable");
    let name = path.display().to_string();
    let modules = DirectoryModules::new(path.parent().expect("a file has a directory"));
    let machine = target.map(|target| {
        (
            target.display().to_string(),
            fs::read_to_string(target).expect("the target is readable"),
        )
    });
    let response = archogen_api::check(&Request {
        name: &name,
        text: &text,
        profile: None,
        modules: &modules,
        target: machine
            .as_ref()
            .map(|(name, text)| archogen_api::TargetFile { name, text }),
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
        .map(|path| {
            (
                path.display().to_string(),
                cli_says(path, None),
                api_says(path, None),
            )
        })
        .collect();
    // More than one status, or the leg compares one outcome and calls it parity.
    let statuses: std::collections::BTreeSet<i32> =
        pairs.iter().map(|(_, cli, _)| cli.status.code()).collect();
    assert!(statuses.len() > 2, "{statuses:?}");
    let wrong = behaviour_mismatches(&pairs);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// The same leg on the shipped target: `archogen check --target` says what the API's `check` says with the target in its request (leaf
/// `M4.1.7.4.2.1`).
#[test]
fn over_every_description_on_the_shipped_target_the_cli_says_what_the_api_says() {
    let root = repo_root();
    let target = root.join("targets/riscv-virt-up.eadl");
    let pairs: Vec<(String, Said, Said)> = descriptions(&root)
        .iter()
        .map(|path| {
            (
                path.display().to_string(),
                cli_says(path, Some(&target)),
                api_says(path, Some(&target)),
            )
        })
        .collect();
    let statuses: std::collections::BTreeSet<i32> =
        pairs.iter().map(|(_, cli, _)| cli.status.code()).collect();
    assert!(statuses.len() > 2, "{statuses:?}");
    let wrong = behaviour_mismatches(&pairs);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}

/// What `archogen resolve --unchecked` said about `path`, in-process, and the plan it wrote at `plan`.
/// What `archogen resolve` printed of a plan's obligations, one line each (leaf `M3.2.6.1`), and what the API's say.
fn owed_lines(out: &str) -> Vec<String> {
    out.lines()
        .filter(|line| line.starts_with("    (adapter "))
        .map(str::to_string)
        .collect()
}

fn api_owed_lines(plan: &archogen_api::Plan) -> Vec<String> {
    plan.obligations
        .iter()
        .map(|owed| {
            format!(
                "    {} {} `{}`: {}",
                owed.instance,
                owed.owes.as_str(),
                owed.name,
                owed.doc
            )
        })
        .collect()
}

fn cli_resolves(path: &Path, plan: &Path) -> (Said, Option<String>) {
    let _ = fs::remove_file(plan);
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(
        [
            "resolve".to_string(),
            path.display().to_string(),
            "--out".to_string(),
            plan.display().to_string(),
            "--unchecked".to_string(),
        ],
        &mut out,
        &mut err,
    );
    let err = String::from_utf8_lossy(&err);
    let prefix = format!("archogen: {}: ", status.slug());
    let said = Said {
        status,
        codes: err
            .lines()
            .filter_map(|line| line.strip_prefix("error[")?.split_once(']'))
            .map(|(code, _)| code.to_string())
            .collect(),
        notes: err
            .lines()
            .filter_map(|line| line.strip_prefix(prefix.as_str()))
            .filter(|rest| !rest.contains(" diagnostic(s) in "))
            .map(str::to_string)
            .collect(),
    };
    // The plan as written, and what the command printed it owes, one text to compare.
    let written = fs::read_to_string(plan)
        .ok()
        .map(|text| text + &owed_lines(&String::from_utf8_lossy(&out)).join("\n"));
    (said, written)
}

/// The engine API's `resolve` on `path`, unchecked, with the module path the CLI uses: what it said, and its plan.
fn api_resolves(path: &Path) -> (Said, Option<String>) {
    let text = fs::read_to_string(path).expect("readable");
    let name = path.display().to_string();
    let modules = DirectoryModules::new(path.parent().expect("a file has a directory"));
    let resolution = archogen_api::resolve_with(
        &Request {
            name: &name,
            text: &text,
            profile: None,
            modules: &modules,
            target: None,
        },
        &archogen_api::Resolve {
            system: None,
            unchecked: true,
        },
        archogen_api::Limits::NONE,
    );
    let response = resolution.response;
    (
        Said {
            status: response.status,
            codes: response
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code.to_string())
                .collect(),
            notes: response.notes,
        },
        resolution
            .plan
            .map(|plan| plan.text.clone() + &api_owed_lines(&plan).join("\n")),
    )
}

#[test]
fn over_every_description_resolve_says_what_the_api_says_and_writes_its_plan() {
    // The record's §6, amended `2026-10-23`: `resolve`'s behaviour leg compares the API's, unchecked, with
    // `archogen resolve --unchecked` (leaf `M3.4.4`).
    let root = repo_root();
    let plan = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("api-parity-plan-{}.json", std::process::id()));
    let mut statuses = std::collections::BTreeSet::new();
    let mut plans = 0;
    let mut owing = 0;
    let mut wrong = Vec::new();
    for path in descriptions(&root) {
        let (cli, cli_plan) = cli_resolves(&path, &plan);
        let (api, api_plan) = api_resolves(&path);
        statuses.insert(cli.status.code());
        plans += usize::from(cli_plan.is_some());
        owing += usize::from(
            api_plan
                .as_deref()
                .is_some_and(|p| p.contains("    (adapter ")),
        );
        if cli != api || cli_plan != api_plan {
            wrong.push(format!(
                "{}:\n  cli {cli:?} {}\n  api {api:?} {}",
                path.display(),
                cli_plan.is_some(),
                api_plan.is_some()
            ));
        }
    }
    let _ = fs::remove_file(&plan);
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    // Plans written and refusals of several kinds, or the leg compares one outcome and calls it parity.
    assert!(plans > 0, "no description resolved");
    assert!(statuses.len() > 2, "{statuses:?}");
    // And a plan that owes something for an engine adapter, or the obligations compared are empty lists alone.
    assert!(owing > 0, "no plan bound an engine adapter");
}

/// The facts the `explain` leg asks, each description one in turn: one a console requires, one a comparator's, one a
/// counter's, and one only a realization meets — so accounts realized, unmet and met alone, and usage refusals, all
/// occur, at the cost of one explanation per description.
const EXPLAINED: &[&str] = &[
    "observable-output",
    "absolute-deadline",
    "counter-width",
    "release-accuracy",
];

/// One account as both surfaces state it: the requirement, each provider's answer, and the provider bound.
fn account_lines(account: &archogen_api::Account) -> Vec<String> {
    let mut lines = vec![format!(
        "`{}` required by `{}`",
        account.fact, account.requester
    )];
    lines.extend(
        account
            .answers
            .iter()
            .map(|(provider, answer)| format!("  `{provider}` {}", answer.says())),
    );
    lines.extend(
        account
            .bound
            .iter()
            .map(|bound| format!("  bound to `{bound}` by the search's plan")),
    );
    lines
}

/// What `archogen explain` said about `fact` in `path`, in-process, and its accounts' lines.
fn cli_explains(path: &Path, fact: &str) -> (Said, Vec<String>) {
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(
        [
            "explain".to_string(),
            path.display().to_string(),
            "--requirement".to_string(),
            fact.to_string(),
        ],
        &mut out,
        &mut err,
    );
    let out = String::from_utf8_lossy(&out);
    let err = String::from_utf8_lossy(&err);
    let prefix = format!("archogen: {}: ", status.slug());
    let said = Said {
        status,
        codes: err
            .lines()
            .filter_map(|line| line.strip_prefix("error[")?.split_once(']'))
            .map(|(code, _)| code.to_string())
            .collect(),
        notes: err
            .lines()
            .filter_map(|line| line.strip_prefix(prefix.as_str()))
            .filter(|rest| !rest.contains(" diagnostic(s) in "))
            .map(str::to_string)
            .collect(),
    };
    let lines = out
        .lines()
        .filter(|line| {
            line.contains("` required by `")
                || line.starts_with("  bound to `")
                || (line.starts_with("  `") && !line.contains("` in its place: "))
        })
        .map(str::to_string)
        .collect();
    (said, lines)
}

/// The engine API's `explain` of `fact` in `path`, with the module path the CLI uses: what it said, and its accounts'.
fn api_explains(path: &Path, fact: &str) -> (Said, Vec<String>) {
    let text = fs::read_to_string(path).expect("readable");
    let name = path.display().to_string();
    let modules = DirectoryModules::new(path.parent().expect("a file has a directory"));
    let explanation = archogen_api::explain_with(
        &Request {
            name: &name,
            text: &text,
            profile: None,
            modules: &modules,
            target: None,
        },
        &archogen_api::Explain {
            fact,
            ..archogen_api::Explain::default()
        },
        archogen_api::Limits::NONE,
    );
    let lines = explanation
        .accounts
        .iter()
        .flat_map(account_lines)
        .collect();
    let response = explanation.response;
    (
        Said {
            status: response.status,
            codes: response
                .diagnostics
                .iter()
                .map(|diagnostic| diagnostic.code.to_string())
                .collect(),
            notes: response.notes,
        },
        lines,
    )
}

#[test]
fn over_every_description_explain_says_what_the_api_says() {
    // `explain`'s behaviour leg (leaf `M3.4.13.2`): over every description, asked a fact `EXPLAINED` names in turn, the
    // status, codes and notes `archogen explain` prints, and each account's requirement, answers and binding, are the
    // API's.
    let root = repo_root();
    let mut statuses = std::collections::BTreeSet::new();
    let mut accounts = 0;
    let mut wrong = Vec::new();
    for (at, path) in descriptions(&root).iter().enumerate() {
        let fact = EXPLAINED[at % EXPLAINED.len()];
        let (cli, cli_lines) = cli_explains(path, fact);
        let (api, api_lines) = api_explains(path, fact);
        statuses.insert(cli.status.code());
        accounts += cli_lines
            .iter()
            .filter(|l| l.contains("` required by `"))
            .count();
        if cli != api || cli_lines != api_lines {
            wrong.push(format!(
                "{} `{fact}`:\n  cli {cli:?} {cli_lines:?}\n  api {api:?} {api_lines:?}",
                path.display()
            ));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    // Accounts given and refusals of several kinds, or the leg compares one outcome and calls it parity.
    assert!(accounts > 0, "no account given");
    assert!(statuses.len() > 2, "{statuses:?}");
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
