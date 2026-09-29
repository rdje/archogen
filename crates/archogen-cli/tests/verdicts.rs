//! Every description this repository holds keeps its verdict — `ROADMAP.md` §15's "a source description retains its
//! meaning under its locked semantic version", made a test (leaf `PROGRAM.6.2`).
//!
//! ⭐ **Why a frozen table.** The engine changes under the language: `M1.34`–`M1.36` each changed engine code, and
//! each proved it moved no verdict by building the CLI from a stash and from the change and diffing `archogen check`
//! over every description — by hand, three times in one day. `tests/verdicts.txt` is that census, frozen: for every
//! description, the exit code and the set of diagnostic codes. An engine change that moves any of them fails here,
//! and passes only when the table is regenerated in the same change — which puts the moved verdicts in its diff,
//! for review, instead of nowhere.
//!
//! Regenerate deliberately: `ARCHOGEN_BLESS_VERDICTS=1 cargo test -p archogen-cli --test verdicts`.
//!
//! The population is every `*.eadl` under the repository root except the directories given to generated output
//! and to other repositories — `target/`, `build/`, `vendor/` and hidden ones — found by walking, so a description
//! added tomorrow is in scope the day it is added.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};

use archogen_cli::run;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

const TABLE: &str = "crates/archogen-cli/tests/verdicts.txt";
const SKIPPED: &[&str] = &["target", "build", "vendor"];

/// Every description, as a repository-relative path, sorted.
fn descriptions(root: &Path) -> Vec<String> {
    fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
        let Ok(entries) = fs::read_dir(dir) else {
            return;
        };
        for entry in entries.flatten() {
            let path = entry.path();
            let name = entry.file_name().to_string_lossy().to_string();
            if path.is_dir() {
                let top = dir == root;
                if name.starts_with('.') || (top && SKIPPED.contains(&name.as_str())) {
                    continue;
                }
                walk(root, &path, out);
            } else if name.ends_with(".eadl") {
                let relative = path.strip_prefix(root).expect("under the root");
                out.push(relative.to_string_lossy().replace('\\', "/"));
            }
        }
    }
    let mut out = Vec::new();
    walk(root, root, &mut out);
    out.sort();
    out
}

/// `archogen check` on one description, in-process: its exit code and its diagnostic codes, sorted and deduplicated.
fn verdict(root: &Path, relative: &str) -> String {
    let path = root.join(relative);
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(
        ["check".to_string(), path.display().to_string()],
        &mut out,
        &mut err,
    );
    let err = String::from_utf8_lossy(&err);
    let mut codes: Vec<&str> = err
        .lines()
        .filter_map(|line| {
            let rest = line
                .strip_prefix("error[")
                .or_else(|| line.strip_prefix("warning["))?;
            rest.split_once(']').map(|(code, _)| code)
        })
        .collect();
    codes.sort_unstable();
    codes.dedup();
    let codes = if codes.is_empty() {
        "-".to_string()
    } else {
        codes.join(",")
    };
    format!("{} {codes}", status.code())
}

fn parse(table: &str) -> BTreeMap<String, String> {
    table
        .lines()
        .filter(|line| !line.starts_with('#') && !line.trim().is_empty())
        .filter_map(|line| {
            let (code, rest) = line.split_once(' ')?;
            let (codes, path) = rest.split_once(' ')?;
            Some((path.to_string(), format!("{code} {codes}")))
        })
        .collect()
}

#[test]
#[cfg_attr(
    miri,
    ignore = "walks every tracked description; the corpus walks are left out of Miri on measured cost"
)]
fn every_description_keeps_its_frozen_verdict() {
    let root = repo_root();
    let found: BTreeMap<String, String> = descriptions(&root)
        .into_iter()
        .map(|relative| {
            let v = verdict(&root, &relative);
            (relative, v)
        })
        .collect();
    assert!(
        found.len() >= 100,
        "the walk found only {} descriptions — the population is broken, not the engine",
        found.len()
    );
    let table_path = root.join(TABLE);
    if std::env::var_os("ARCHOGEN_BLESS_VERDICTS").is_some() {
        let mut text = String::from(
            "# The frozen verdict of every description in this repository (leaf `PROGRAM.6.2`).\n\
             # <exit code of `archogen check`> <its diagnostic codes, sorted, or -> <path>\n\
             # Regenerated deliberately: ARCHOGEN_BLESS_VERDICTS=1 cargo test -p archogen-cli --test verdicts\n",
        );
        for (path, v) in &found {
            text.push_str(&format!("{v} {path}\n"));
        }
        fs::write(&table_path, text).expect("the table is writable");
        return;
    }
    let frozen = parse(&fs::read_to_string(&table_path).unwrap_or_default());
    let mut problems = Vec::new();
    for (path, now) in &found {
        match frozen.get(path) {
            None => problems.push(format!("new, with no frozen verdict: {path} ({now})")),
            Some(then) if then != now => {
                problems.push(format!("MOVED: {path}: was `{then}`, is `{now}`"));
            }
            Some(_) => {}
        }
    }
    for path in frozen.keys().filter(|p| !found.contains_key(*p)) {
        problems.push(format!("frozen but gone: {path}"));
    }
    assert!(
        problems.is_empty(),
        "{} of {} description(s) differ from {TABLE}:\n  {}\n\nIf the engine meant to change these verdicts, \
         regenerate the table in the same change — ARCHOGEN_BLESS_VERDICTS=1 cargo test -p archogen-cli --test \
         verdicts — so the moved verdicts are in its diff.",
        problems.len(),
        found.len(),
        problems.join("\n  ")
    );
}
