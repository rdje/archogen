//! `cargo xtask trust-inventory` — the trust gate's measuring instrument (leaf `M3.6.2`,
//! `docs/decisions/decision_trust-inventory.md` §2–§4, `docs/decisions/decision_executable-design-reviews.md`).
//!
//! > The compiler, not a scan, decides completeness — and where it cannot, the input is refused, not trusted.
//!
//! It writes the commit's own blobs into a scratch tree, builds each root and the comparison harness clean under the
//! pinned toolchain and the catalog's allowlisted environment, reads what the compiler read for each compilation
//! unit, and writes `target/trust/trust-dependencies.json`: the build's identity, each program's packages, files and
//! unit configurations, and every item two programs share. On any refusal it writes no inventory (§3, R5 9).
//!
//! ⛔ **Default-deny** (§3, R7). Every manifest and token rule the catalog applies to a recorded package —
//! `check_manifest` and `scan` in `crates/archogen-catalog/src/package.rs`, unchanged — applies to every `.rs` file a
//! program's compilation reads, the harness's development edges followed; a site the rules refuse passes only when an
//! admission in `trust/roots.eadl` names its file, its rule and the sha256 of its line. Data a compilation reads is
//! hashed and never tokenised (R6 7).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_catalog::manifest;
use archogen_catalog::package;
use archogen_catalog::tree::Tree;
use archogen_evidence::sha256::Digest;
use eadl_front::{read, Form, SourceMap};

use crate::catalog_build::{configurations_on_path, environment, toolchain_file, write_tree};
use crate::catalog_check::{tree_entries, tree_of, Git};
use crate::json::{self, Json};

/// Where the roots, the harness and the admissions are declared (§2).
pub const ROOTS: &str = "trust/roots.eadl";

// ── The roots file ──────────────────────────────────────────────────────────────────────────────────────────────

/// What a program is built as.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Target {
    /// The package's library.
    Lib,
    /// An executable target.
    Bin(String),
    /// An integration test target: the comparison harness.
    Test(String),
}

/// A program whose build is inventoried: a root (§2) or the comparison harness (§3, §4).
#[derive(Debug, Clone)]
pub struct Program {
    /// Its name in the roots file.
    pub name: String,
    /// Its role; for the harness, `harness`.
    pub role: String,
    /// Its package's directory, relative to the repository.
    pub package: String,
    /// What is built.
    pub target: Target,
    /// The packages holding its role's own logic (§2).
    pub role_packages: BTreeSet<String>,
    /// For the harness: the two roots it compares.
    pub pair: Option<(String, String)>,
}

/// A refused site admitted by review (§3).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Admission {
    /// The file.
    pub file: String,
    /// The rule: the refused word as the catalog's scanner names it.
    pub rule: String,
    /// The sha256 of the line the site stands on.
    pub line: String,
}

/// The roots file, read.
#[derive(Debug, Clone, Default)]
pub struct Roots {
    /// Every root and the harness.
    pub programs: Vec<Program>,
    /// Every admission.
    pub admissions: BTreeSet<Admission>,
}

fn clause<'a>(form: &'a Form, head: &str) -> Option<&'a Form> {
    // Every item after the form's head: a name is a bare word, never a clause, so it is never found.
    form.items().iter().skip(1).find(|c| c.head() == Some(head))
}

fn word(form: &Form) -> Option<String> {
    match form {
        Form::Symbol { name, .. } => Some(name.clone()),
        Form::Str { value, .. } => Some(value.clone()),
        _ => None,
    }
}

fn values(form: &Form, head: &str) -> Vec<String> {
    clause(form, head)
        .map(|c| c.items().iter().skip(1).filter_map(word).collect())
        .unwrap_or_default()
}

fn one(form: &Form, head: &str, what: &str) -> Result<String, String> {
    let v = values(form, head);
    match v.as_slice() {
        [x] => Ok(x.clone()),
        _ => Err(format!("{ROOTS}: `{what}` needs exactly one `({head} …)`")),
    }
}

/// Read the roots file.
///
/// # Errors
///
/// A form the instrument cannot read.
pub fn read_roots(text: &str) -> Result<Roots, String> {
    let mut sources = SourceMap::new();
    let id = sources
        .add(ROOTS, text)
        .map_err(|e| format!("{ROOTS}: {e:?}"))?;
    let (doc, diags) = read(&sources, id);
    if diags.has_errors() {
        return Err(diags.render(&sources));
    }
    let mut roots = Roots::default();
    for form in &doc.forms {
        let name = form.items().get(1).and_then(word).unwrap_or_default();
        match form.head() {
            Some("defroot") => {
                let target = match values(form, "target").as_slice() {
                    [k] if k == "lib" => Target::Lib,
                    [k, n] if k == "bin" => Target::Bin(n.clone()),
                    _ => {
                        return Err(format!(
                            "{ROOTS}: `{name}`'s target is `(target lib)` or `(target bin NAME)`"
                        ))
                    }
                };
                let package = one(form, "package", &name)?;
                let mut role_packages: BTreeSet<String> =
                    values(form, "role-packages").into_iter().collect();
                role_packages.insert(package.clone());
                roots.programs.push(Program {
                    role: one(form, "role", &name)?,
                    package,
                    target,
                    role_packages,
                    pair: None,
                    name,
                });
            }
            Some("defharness") => {
                let pair = values(form, "pair");
                let [a, b] = pair.as_slice() else {
                    return Err(format!("{ROOTS}: `{name}` names `(pair ROOT ROOT)`"));
                };
                roots.programs.push(Program {
                    role: "harness".to_owned(),
                    package: one(form, "package", &name)?,
                    target: Target::Test(one(form, "test", &name)?),
                    role_packages: BTreeSet::new(),
                    pair: Some((a.clone(), b.clone())),
                    name,
                });
            }
            Some("defadmit") => {
                roots.admissions.insert(Admission {
                    file: one(form, "file", "an admission")?,
                    rule: one(form, "rule", "an admission")?,
                    line: one(form, "line", "an admission")?,
                });
            }
            // A role no root fills yet, and a classification (§2), are the gate's: `M3.6.3` reads them.
            Some("defrole" | "defclassify") => {}
            other => {
                return Err(format!(
                    "{ROOTS}: a form `{}` the instrument does not know",
                    other.unwrap_or("?")
                ))
            }
        }
    }
    Ok(roots)
}

// ── Building, and what the compiler reports ─────────────────────────────────────────────────────────────────────

/// One compilation unit of a program's build.
#[derive(Debug, Clone)]
pub struct Unit {
    /// `--crate-name`.
    pub crate_name: String,
    /// The package it belongs to, by directory.
    pub package: String,
    /// The normalised configuration (§3).
    pub configuration: Vec<String>,
    /// Every file its dependency information names, as repository paths.
    pub files: BTreeSet<String>,
    /// Every `# env-dep` line, with its value.
    pub env: Vec<String>,
}

/// Split one `rustc` command line as cargo's `-v` prints it: on spaces outside single quotes.
fn split_command(line: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut cur = String::new();
    let mut quoted = false;
    let mut any = false;
    for c in line.chars() {
        match c {
            '\'' => {
                quoted = !quoted;
                any = true;
            }
            ' ' if !quoted => {
                if any || !cur.is_empty() {
                    out.push(std::mem::take(&mut cur));
                }
                any = false;
            }
            c => cur.push(c),
        }
    }
    if any || !cur.is_empty() {
        out.push(cur);
    }
    out
}

/// The flags dropped from a unit's configuration, each with whether it takes the next token (§3): the toolchain's
/// own hashes, which the build configuration holds by identity; paths of the scratch build; lint levels and
/// `--check-cfg`, which change only what is warned about; and the output-format flags, which change only how.
const DROPPED: &[(&str, bool)] = &[
    ("--out-dir", true),
    ("-L", true),
    ("--check-cfg", true),
    ("--cap-lints", true),
    ("-W", true),
    ("-A", true),
    ("-D", true),
    ("-F", true),
    ("--warn", true),
    ("--allow", true),
    ("--deny", true),
    ("--forbid", true),
    ("--color", true),
];

const DROPPED_PREFIX: &[&str] = &[
    "--error-format",
    "--json",
    "--diagnostic-width",
    "--color=",
    "--warn=",
    "--allow=",
    "--deny=",
    "--forbid=",
    "--cap-lints=",
    "--check-cfg=",
];

/// A unit's configuration, normalised (§3).
fn normalise(tokens: &[String], scratch: &str) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 1; // the program path is dropped
    while i < tokens.len() {
        let t = &tokens[i];
        if let Some((_, takes)) = DROPPED.iter().find(|(f, _)| f == t) {
            i += if *takes { 2 } else { 1 };
            continue;
        }
        if DROPPED_PREFIX.iter().any(|p| t.starts_with(p)) {
            i += 1;
            continue;
        }
        if t == "-C" {
            if let Some(v) = tokens.get(i + 1) {
                if v.starts_with("metadata=") || v.starts_with("extra-filename=") {
                    i += 2;
                    continue;
                }
                out.push(format!("-C {v}"));
                i += 2;
                continue;
            }
        }
        if t == "--extern" {
            if let Some(v) = tokens.get(i + 1) {
                let (name, path) = v.split_once('=').unwrap_or((v, ""));
                let file = path.rsplit('/').next().unwrap_or(path);
                let stem = match (file.rfind('-'), file.rfind('.')) {
                    (Some(dash), Some(dot)) if dash < dot => {
                        format!("{}{}", &file[..dash], &file[dot..])
                    }
                    _ => file.to_owned(),
                };
                out.push(format!("--extern {name}={stem}"));
                i += 2;
                continue;
            }
        }
        out.push(t.replace(scratch, ""));
        i += 1;
    }
    out
}

/// `path` resolved lexically, `..` and `.` removed; `None` when it climbs above its start.
fn lexical(path: &str) -> Option<String> {
    let mut parts: Vec<&str> = Vec::new();
    for p in path.split('/') {
        match p {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            p => parts.push(p),
        }
    }
    Some(parts.join("/"))
}

/// What one build of one program reports.
struct Build {
    units: Vec<Unit>,
}

fn run_cargo(args: &[&str], env: &[(String, String)], cwd: &Path) -> Result<String, String> {
    let mut c = Command::new("cargo");
    c.args(args).current_dir(cwd).env_clear();
    for (k, v) in env {
        c.env(k, v);
    }
    let out = c.output().map_err(|e| format!("cargo did not run: {e}"))?;
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if !out.status.success() {
        let first = stderr
            .lines()
            .find(|l| l.starts_with("error"))
            .unwrap_or("")
            .to_owned();
        return Err(format!("`cargo {}` failed: {first}", args.join(" ")));
    }
    Ok(stderr)
}

/// Build `p` clean in `tree_dir`, into its own target directory, and read every unit's dependency information.
fn build(
    p: &Program,
    package_names: &BTreeMap<String, String>,
    env: &[(String, String)],
    tree_dir: &Path,
    refused: &mut Vec<String>,
) -> Result<Build, String> {
    let name = package_names
        .get(&p.package)
        .ok_or_else(|| format!("`{}`: no package at `{}`", p.name, p.package))?;
    let mut args: Vec<String> = Vec::new();
    match &p.target {
        Target::Test(t) => args.extend(["test", "--no-run", "--test", t].map(str::to_owned)),
        Target::Bin(b) => args.extend(["build", "--bin", b].map(str::to_owned)),
        Target::Lib => args.extend(["build", "--lib"].map(str::to_owned)),
    }
    args.extend(
        [
            "--release",
            "--locked",
            "--offline",
            "--no-default-features",
            "-v",
            "-p",
        ]
        .map(str::to_owned),
    );
    args.push(name.clone());
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let stderr = run_cargo(&argv, env, tree_dir)?;
    let scratch = format!("{}/", tree_dir.display());
    let canonical_scratch = fs::canonicalize(tree_dir)
        .map(|c| format!("{}/", c.display()))
        .unwrap_or_else(|_| scratch.clone());
    let mut units = Vec::new();
    for line in stderr.lines() {
        let Some(start) = line.find("Running `") else {
            continue;
        };
        let cmd = &line[start + 9..line.rfind('`').unwrap_or(line.len())];
        let tokens = split_command(cmd);
        if !tokens.first().is_some_and(|p| p.ends_with("rustc")) {
            continue; // a build script's run, or a test binary: neither is a compilation
        }
        let get = |flag: &str| {
            tokens
                .iter()
                .position(|t| t == flag)
                .and_then(|i| tokens.get(i + 1))
                .cloned()
        };
        let crate_name = get("--crate-name").unwrap_or_default();
        let out_dir = get("--out-dir").unwrap_or_default();
        let extra = tokens
            .windows(2)
            .find_map(|w| {
                (w[0] == "-C" && w[1].starts_with("extra-filename="))
                    .then(|| w[1]["extra-filename=".len()..].to_owned())
            })
            .unwrap_or_default();
        let src = tokens
            .iter()
            .skip(1)
            .find(|t| t.ends_with(".rs") && !t.starts_with('-'))
            .cloned()
            .unwrap_or_default();
        let src_rel = src
            .strip_prefix(&scratch)
            .or_else(|| src.strip_prefix(&canonical_scratch))
            .unwrap_or(&src)
            .to_owned();
        let package = package_names
            .keys()
            .filter(|dir| src_rel.starts_with(&format!("{dir}/")) || dir.is_empty())
            .max_by_key(|d| d.len())
            .cloned()
            .unwrap_or_default();
        let d = PathBuf::from(&out_dir).join(format!("{crate_name}{extra}.d"));
        let text = fs::read_to_string(&d).map_err(|e| format!("{}: {e}", d.display()))?;
        let mut files = BTreeSet::new();
        let mut env_deps = Vec::new();
        let mut first = true;
        for l in text.lines() {
            if let Some(dep) = l.strip_prefix("# env-dep:") {
                env_deps.push(dep.replace(&scratch, "").replace(&canonical_scratch, ""));
                continue;
            }
            if !first || l.is_empty() {
                continue;
            }
            first = false;
            let Some((_, rest)) = l.split_once(": ") else {
                continue;
            };
            for raw in rest.split(' ').filter(|x| !x.is_empty()) {
                let raw = raw.replace("\\ ", " ");
                let rel = if let Some(r) = raw
                    .strip_prefix(&scratch)
                    .or_else(|| raw.strip_prefix(&canonical_scratch))
                {
                    r.to_owned()
                } else if raw.starts_with('/') {
                    refused.push(format!(
                        "trust-undeclared-input: `{}` reads `{raw}`, outside the written tree",
                        p.name
                    ));
                    continue;
                } else {
                    raw
                };
                match lexical(&rel) {
                    Some(path) => {
                        files.insert(path);
                    }
                    None => refused.push(format!("trust-undeclared-input: `{}` reads `{rel}`, which climbs out of the written tree", p.name)),
                }
            }
        }
        units.push(Unit {
            crate_name,
            package,
            configuration: normalise(&tokens, &scratch),
            files,
            env: env_deps,
        });
    }
    Ok(Build { units })
}

// ── Refusals by the catalog's rules (§3) ────────────────────────────────────────────────────────────────────────

/// Every site the catalog's token rules refuse in `source`, by blanking each and scanning again: `(line, rule)`.
fn refused_sites(source: &[u8]) -> Result<Vec<(usize, String)>, String> {
    let mut bytes = source.to_vec();
    let mut sites = Vec::new();
    for _ in 0..10_000 {
        match package::scan(&bytes) {
            Ok(()) => return Ok(sites),
            Err(found) => {
                let rule = found
                    .why
                    .split('`')
                    .nth(1)
                    .map_or_else(|| found.why.clone(), str::to_owned);
                sites.push((found.line, rule));
                let line_start = bytes
                    .split(|&b| b == b'\n')
                    .take(found.line - 1)
                    .map(|l| l.len() + 1)
                    .sum::<usize>();
                let start = line_start + found.column.saturating_sub(1);
                let mut end = start;
                while end < bytes.len()
                    && (bytes[end].is_ascii_alphanumeric() || bytes[end] == b'_')
                {
                    end += 1;
                }
                if end == start {
                    return Err(format!("line {}: {}", found.line, found.why));
                }
                for b in &mut bytes[start..end] {
                    *b = b' ';
                }
            }
        }
    }
    Err("more than ten thousand refused sites".to_owned())
}

fn line_hash(source: &[u8], line: usize) -> String {
    let text = source
        .split(|&b| b == b'\n')
        .nth(line - 1)
        .unwrap_or_default();
    Digest::of(text).hex()
}

// ── The inventory ───────────────────────────────────────────────────────────────────────────────────────────────

fn s(v: impl Into<String>) -> Json {
    Json::Str(v.into())
}

fn obj(entries: Vec<(&str, Json)>) -> Json {
    Json::Object(
        entries
            .into_iter()
            .map(|(k, v)| (k.to_owned(), v))
            .collect(),
    )
}

fn strings(items: impl IntoIterator<Item = String>) -> Json {
    Json::Array(items.into_iter().map(Json::Str).collect())
}

fn tool_line(program: &str, args: &[&str], env: &[(String, String)], cwd: &Path) -> String {
    let mut c = Command::new(program);
    c.args(args).current_dir(cwd).env_clear();
    for (k, v) in env {
        c.env(k, v);
    }
    c.output().map_or_else(
        |e| format!("did not run: {e}"),
        |o| String::from_utf8_lossy(&o.stdout).trim().to_owned(),
    )
}

/// What one run concluded.
pub enum Outcome {
    /// The inventory, written.
    Written(Json),
    /// What was refused; no inventory is written.
    Refused(Vec<String>),
}

/// Build the inventory of `commit` in the repository at `repo`, scratch under `out`.
///
/// # Errors
///
/// A failure that is not a refusal: git, cargo or the file system did not answer.
pub fn inventory(repo: &Path, commit: &str, out: &Path) -> Result<Outcome, String> {
    inventory_with(repo, commit, out, repo)
}

/// [`inventory`], the cargo configurations on the build's path judged against `config_root`'s tracked copy: the
/// repository itself, or, for a test's scratch repository nested under it, the outer one (as the catalog's tests do).
///
/// # Errors
///
/// As [`inventory`].
pub fn inventory_with(
    repo: &Path,
    commit: &str,
    out: &Path,
    config_root: &Path,
) -> Result<Outcome, String> {
    let git = Git::at(repo);
    let commit = git
        .text(&["rev-parse", "--verify", &format!("{commit}^{{commit}}")])?
        .trim()
        .to_owned();
    let tree_hash = git
        .text(&["rev-parse", &format!("{commit}^{{tree}}")])?
        .trim()
        .to_owned();
    let entries = tree_entries(&git, &commit)?;
    let symlinks: BTreeSet<String> = entries
        .iter()
        .filter(|e| e.mode == "120000")
        .map(|e| e.path.clone())
        .collect();
    let tree: Tree = tree_of(&git, &entries).map_err(|f| format!("{f:?}"))?;
    let roots_text = tree
        .get(ROOTS)
        .ok_or_else(|| format!("the commit holds no `{ROOTS}`"))?;
    let roots = read_roots(&String::from_utf8_lossy(roots_text))?;
    let mut refused: Vec<String> = Vec::new();

    let tree_dir = out.join("tree");
    write_tree(&tree, &tree_dir).map_err(|f| format!("{f:?}"))?;
    let cargo_home = out.join("cargo-home");
    let _ = fs::remove_dir_all(&cargo_home); // made anew on every run (§3, R2 B12)
    let pin = toolchain_file(&tree)?;
    configurations_on_path(&tree_dir, &tree_dir, config_root, &tree)
        .map_err(|f| format!("{f:?}"))?;

    // The workspace's packages, from `cargo metadata`, which runs no package's code.
    let meta_env = environment(&pin, &cargo_home, &out.join("target-metadata"))?;
    let meta_out = Command::new("cargo")
        .args(["metadata", "--offline", "--locked", "--format-version", "1"])
        .current_dir(&tree_dir)
        .env_clear()
        .envs(meta_env.iter().map(|(k, v)| (k.as_str(), v.as_str())))
        .output()
        .map_err(|e| format!("`cargo metadata` did not run: {e}"))?;
    if !meta_out.status.success() {
        return Err(format!(
            "`cargo metadata` failed: {}",
            String::from_utf8_lossy(&meta_out.stderr).trim()
        ));
    }
    let meta = json::parse(&String::from_utf8_lossy(&meta_out.stdout))?;
    let root_prefix = format!(
        "{}/",
        fs::canonicalize(&tree_dir)
            .map_err(|e| e.to_string())?
            .display()
    );
    let mut package_names: BTreeMap<String, String> = BTreeMap::new();
    let mut id_dir: BTreeMap<String, String> = BTreeMap::new();
    for p in meta.get("packages").map(Json::elements).unwrap_or_default() {
        let manifest_path = p
            .get("manifest_path")
            .and_then(Json::as_str)
            .unwrap_or_default();
        let dir = manifest_path
            .strip_prefix(&root_prefix)
            .and_then(|r| r.strip_suffix("Cargo.toml"))
            .map(|d| d.trim_end_matches('/').to_owned())
            .unwrap_or_default();
        let name = p
            .get("name")
            .and_then(Json::as_str)
            .unwrap_or_default()
            .to_owned();
        let id = p
            .get("id")
            .and_then(Json::as_str)
            .unwrap_or_default()
            .to_owned();
        package_names.insert(dir.clone(), name);
        id_dir.insert(id, dir);
    }
    // The resolved graph: (dependent, dependency, kind).
    let mut graph: Vec<(String, String, String)> = Vec::new();
    for node in meta
        .get("resolve")
        .and_then(|r| r.get("nodes"))
        .map(Json::elements)
        .unwrap_or_default()
    {
        let from = id_dir
            .get(node.get("id").and_then(Json::as_str).unwrap_or_default())
            .cloned()
            .unwrap_or_default();
        for d in node.get("deps").map(Json::elements).unwrap_or_default() {
            let to = id_dir
                .get(d.get("pkg").and_then(Json::as_str).unwrap_or_default())
                .cloned()
                .unwrap_or_default();
            for k in d.get("dep_kinds").map(Json::elements).unwrap_or_default() {
                let kind = k
                    .get("kind")
                    .and_then(Json::as_str)
                    .unwrap_or("normal")
                    .to_owned();
                graph.push((from.clone(), to.clone(), kind));
            }
        }
    }

    // The manifest rules over every package in a program's closure, from the metadata graph, before any build:
    // a build script or a procedural macro is refused here, so none runs (§3, R2 B12); the harness's development
    // edges are followed, since its build compiles them (R7 3).
    let mut sites = 0usize;
    let mut used_admissions = BTreeSet::new();
    let mut manifests_checked = BTreeSet::new();
    for p in &roots.programs {
        let mut closure: BTreeSet<String> = BTreeSet::new();
        let mut todo = vec![p.package.clone()];
        while let Some(pkg) = todo.pop() {
            if !closure.insert(pkg.clone()) {
                continue;
            }
            for (from, to, kind) in &graph {
                let followed = kind != "dev" || (p.role == "harness" && *from == p.package);
                if *from == pkg && followed && !closure.contains(to) {
                    todo.push(to.clone());
                }
            }
        }
        for pkg in closure {
            if !manifests_checked.insert(pkg.clone()) {
                continue;
            }
            let path = if pkg.is_empty() {
                "Cargo.toml".to_owned()
            } else {
                format!("{pkg}/Cargo.toml")
            };
            match tree
                .get(&path)
                .map(|m| manifest::parse(&String::from_utf8_lossy(m)))
            {
                Some(Ok(m)) => {
                    if let Err(why) = package::check_manifest(&tree, &pkg, &path, &m) {
                        // A manifest rule has no line: its admission names the manifest, the catalog's reason, and
                        // the manifest's own sha256, so any edit to the manifest is reviewed again.
                        sites += 1;
                        let bytes = tree.get(&path).unwrap_or_default();
                        let admission = Admission {
                            file: path.clone(),
                            rule: why.clone(),
                            line: Digest::of(bytes).hex(),
                        };
                        if roots.admissions.contains(&admission) {
                            used_admissions.insert(admission);
                        } else {
                            refused.push(format!("trust-undeclared-input: `{}` reaches `{pkg}`: {why}, and no admission names it (manifest sha256 {})", p.name, admission.line));
                        }
                    }
                }
                Some(Err(o)) => refused.push(format!(
                    "trust-undeclared-input: `{path}` line {}: {}",
                    o.line, o.why
                )),
                None => refused.push(format!(
                    "trust-undeclared-input: `{}` reaches a package with no tracked `{path}`",
                    p.name
                )),
            }
        }
    }
    if !refused.is_empty() {
        refused.sort();
        refused.dedup();
        let _ = fs::remove_file(out.join("trust-dependencies.json"));
        return Ok(Outcome::Refused(refused));
    }

    // Build each program clean, its own target directory, a fresh `CARGO_HOME` each.
    let mut builds: BTreeMap<String, Build> = BTreeMap::new();
    for p in &roots.programs {
        let _ = fs::remove_dir_all(&cargo_home);
        let env = environment(&pin, &cargo_home, &out.join("target").join(&p.name))?;
        builds.insert(
            p.name.clone(),
            build(p, &package_names, &env, &tree_dir, &mut refused)?,
        );
    }

    // The token rules over every `.rs` file a compilation read (§3).
    for (name, b) in &builds {
        for u in &b.units {
            for f in &u.files {
                if symlinks.contains(f) {
                    refused.push(format!("trust-undeclared-input: `{name}` reads `{f}`, a symbolic link in the commit"));
                    continue;
                }
                let Some(blob) = tree.get(f) else {
                    refused.push(format!("trust-undeclared-input: `{name}` reads `{f}`, which is not a blob of the commit"));
                    continue;
                };
                if fs::read(tree_dir.join(f)).ok().as_deref() != Some(blob) {
                    refused.push(format!(
                        "trust-undeclared-input: `{f}`'s bytes after the build are not its blob's"
                    ));
                }
                if !std::path::Path::new(f)
                    .extension()
                    .is_some_and(|e| e.eq_ignore_ascii_case("rs"))
                {
                    continue; // data is hashed and never tokenised (R6 7)
                }
                match refused_sites(blob) {
                    Ok(found) => {
                        for (line, rule) in found {
                            sites += 1;
                            let admission = Admission {
                                file: f.clone(),
                                rule: rule.clone(),
                                line: line_hash(blob, line),
                            };
                            if roots.admissions.contains(&admission) {
                                used_admissions.insert(admission);
                            } else {
                                refused.push(format!("trust-undeclared-input: `{f}:{line}`: `{rule}`, refused by the catalog's rules, and no admission names it (line sha256 {})", admission.line));
                            }
                        }
                    }
                    Err(why) => refused.push(format!(
                        "trust-undeclared-input: `{f}` does not tokenize: {why}"
                    )),
                }
            }
        }
    }

    // An executable that runs two roles (§2).
    for p in roots
        .programs
        .iter()
        .filter(|p| matches!(p.target, Target::Bin(_)))
    {
        let compiled: BTreeSet<&String> =
            builds[&p.name].units.iter().map(|u| &u.package).collect();
        for other in roots
            .programs
            .iter()
            .filter(|o| o.role != p.role && o.role != "harness")
        {
            for rp in &other.role_packages {
                if compiled.contains(rp) {
                    refused.push(format!(
                        "trust-shared-program: `{}` compiles `{rp}`, a role package of `{}`",
                        p.name, other.name
                    ));
                }
            }
        }
    }

    refused.sort();
    refused.dedup();
    let target_json = out.join("trust-dependencies.json");
    if !refused.is_empty() {
        let _ = fs::remove_file(&target_json);
        return Ok(Outcome::Refused(refused));
    }

    // The build as a whole (§3).
    let hash = |p: &str| {
        tree.get(p)
            .map_or_else(|| "absent".to_owned(), |b| Digest::of(b).hex())
    };
    let env0 = environment(&pin, &cargo_home, &out.join("target-metadata"))?;
    let rustc = tool_line("rustc", &["-vV"], &env0, &tree_dir);
    let host = rustc
        .lines()
        .find_map(|l| l.strip_prefix("host: "))
        .unwrap_or("")
        .to_owned();
    let mut profile = Vec::new();
    if let Some(Ok(m)) = tree
        .get("Cargo.toml")
        .map(|b| manifest::parse(&String::from_utf8_lossy(b)))
    {
        for (path, value) in &m.values {
            if path.first().is_some_and(|k| k == "profile")
                || path == &["workspace", "package", "edition"]
            {
                profile.push(format!("{}={value:?}", path.join(".")));
            }
        }
    }
    profile.sort();
    let build_configuration = obj(vec![
        ("toolchain", s(&rustc)),
        ("rust-toolchain.toml", s(hash("rust-toolchain.toml"))),
        ("profiles", strings(profile.clone())),
    ]);

    // Each program.
    let file_hash = |f: &str| {
        tree.get(f)
            .map_or_else(String::new, |b| Digest::of(b).hex())
    };
    let mut programs = Vec::new();
    for p in &roots.programs {
        let b = &builds[&p.name];
        let packages: BTreeSet<String> = b.units.iter().map(|u| u.package.clone()).collect();
        programs.push(obj(vec![
            ("name", s(&p.name)),
            ("role", s(&p.role)),
            ("packages", strings(packages)),
            (
                "units",
                Json::Array(
                    b.units
                        .iter()
                        .map(|u| {
                            obj(vec![
                                ("crate", s(&u.crate_name)),
                                ("package", s(&u.package)),
                                ("configuration", strings(u.configuration.clone())),
                                ("env", strings(u.env.clone())),
                                (
                                    "files",
                                    Json::Object(
                                        u.files
                                            .iter()
                                            .map(|f| (f.clone(), s(file_hash(f))))
                                            .collect(),
                                    ),
                                ),
                            ])
                        })
                        .collect(),
                ),
            ),
        ]));
    }

    // Shared items, per pair (§4).
    let mut shared = Vec::new();
    let names: Vec<&Program> = roots.programs.iter().collect();
    for (i, a) in names.iter().enumerate() {
        for b in names.iter().skip(i + 1) {
            let (ba, bb) = (&builds[&a.name], &builds[&b.name]);
            let pk = |x: &Build| {
                x.units
                    .iter()
                    .map(|u| u.package.clone())
                    .collect::<BTreeSet<_>>()
            };
            let (pa, pb) = (pk(ba), pk(bb));
            // The comparison harness a pair's form names is that pair's shared item (§4, R2 B7).
            let harness = roots.programs.iter().find(|h| {
                h.pair.as_ref().is_some_and(|(x, y)| {
                    (x == &a.name && y == &b.name) || (x == &b.name && y == &a.name)
                })
            });
            let mut items = vec![obj(vec![
                ("kind", s("build-configuration")),
                ("content", build_configuration.clone()),
            ])];
            let both: BTreeSet<&String> = pa.intersection(&pb).collect();
            for pkg in &both {
                let edges = |prog: &Program, closure: &BTreeSet<String>| -> Vec<String> {
                    graph
                        .iter()
                        .filter(|(from, to, kind)| {
                            to == *pkg
                                && closure.contains(from)
                                && (kind != "dev"
                                    || prog.role == "harness" && *from == prog.package)
                        })
                        .map(|(from, _, kind)| format!("{}:{from}:{kind}", prog.name))
                        .collect()
                };
                let readers = |x: &Build, prog: &Program| -> Vec<String> {
                    let own: BTreeSet<&String> = x
                        .units
                        .iter()
                        .filter(|u| &&u.package == pkg)
                        .flat_map(|u| u.files.iter())
                        .collect();
                    x.units
                        .iter()
                        .flat_map(|u| {
                            u.files
                                .iter()
                                .filter(|f| own.contains(f))
                                .map(move |f| format!("{f}<{}:{}", prog.name, u.package))
                        })
                        .collect()
                };
                let manifest = if pkg.is_empty() {
                    "Cargo.toml".to_owned()
                } else {
                    format!("{pkg}/Cargo.toml")
                };
                let content: BTreeSet<String> = ba
                    .units
                    .iter()
                    .chain(bb.units.iter())
                    .filter(|u| &&u.package == pkg)
                    .flat_map(|u| u.files.iter().map(|f| format!("{f}={}", file_hash(f))))
                    .collect();
                let config = |x: &Build| {
                    x.units
                        .iter()
                        .filter(|u| &&u.package == pkg)
                        .map(|u| u.configuration.join(" "))
                        .collect::<Vec<_>>()
                };
                let mut e = edges(a, &pa);
                e.extend(edges(b, &pb));
                let mut r = readers(ba, a);
                r.extend(readers(bb, b));
                items.push(obj(vec![
                    ("kind", s("package")),
                    ("package", s(*pkg)),
                    ("manifest", s(file_hash(&manifest))),
                    ("content", strings(content)),
                    (
                        "configuration",
                        Json::Array(vec![strings(config(ba)), strings(config(bb))]),
                    ),
                    ("edges", strings(e.into_iter().collect::<BTreeSet<_>>())),
                    ("readers", strings(r.into_iter().collect::<BTreeSet<_>>())),
                ]));
            }
            // A file both read that no shared package's own compilation reads (§4, R4 6; files handed at run time
            // are `M2.7.5`'s and `M4`'s, not built yet).
            let files = |x: &Build| {
                x.units
                    .iter()
                    .flat_map(|u| u.files.iter().cloned())
                    .collect::<BTreeSet<_>>()
            };
            let (fa, fb) = (files(ba), files(bb));
            let in_shared: BTreeSet<String> = ba
                .units
                .iter()
                .chain(bb.units.iter())
                .filter(|u| both.contains(&u.package))
                .flat_map(|u| u.files.iter().cloned())
                .collect();
            for f in fa.intersection(&fb) {
                if !in_shared.contains(f) {
                    items.push(obj(vec![
                        ("kind", s("file")),
                        ("file", s(f)),
                        ("sha256", s(file_hash(f))),
                    ]));
                }
            }
            // A copy: one non-empty file's bytes under two paths (§4, R1 A11).
            let mut by_hash: BTreeMap<String, (BTreeSet<String>, BTreeSet<String>)> =
                BTreeMap::new();
            for f in &fa {
                if tree.get(f).is_some_and(|x| !x.is_empty()) {
                    by_hash.entry(file_hash(f)).or_default().0.insert(f.clone());
                }
            }
            for f in &fb {
                if tree.get(f).is_some_and(|x| !x.is_empty()) {
                    by_hash.entry(file_hash(f)).or_default().1.insert(f.clone());
                }
            }
            for (h, (xa, xb)) in &by_hash {
                if !xa.is_empty() && !xb.is_empty() && xa != xb {
                    items.push(obj(vec![
                        ("kind", s("copy")),
                        ("sha256", s(h)),
                        ("paths", strings(xa.union(xb).cloned())),
                    ]));
                }
            }
            if let Some(h) = harness {
                // Every file it compiles beside the pair's two libraries: its own package's test unit.
                let Target::Test(test) = &h.target else {
                    return Err(format!("`{}` is a harness without a test target", h.name));
                };
                let own: BTreeSet<String> = builds[&h.name]
                    .units
                    .iter()
                    .filter(|u| &u.crate_name == test)
                    .flat_map(|u| u.files.iter().map(|f| format!("{f}={}", file_hash(f))))
                    .collect();
                items.push(obj(vec![
                    ("kind", s("comparison-harness")),
                    ("harness", s(&h.name)),
                    ("content", strings(own)),
                ]));
            }
            shared.push(obj(vec![
                ("pair", strings([a.name.clone(), b.name.clone()])),
                ("items", Json::Array(items)),
            ]));
        }
    }

    let unused: Vec<String> = roots
        .admissions
        .difference(&used_admissions)
        .map(|a| format!("{}:{}", a.file, a.rule))
        .collect();
    let inv = obj(vec![
        ("format", s("archogen-trust-inventory/0")),
        (
            "identity",
            obj(vec![
                ("commit", s(&commit)),
                ("tree", s(&tree_hash)),
                ("rustc", s(&rustc)),
                ("cargo", s(tool_line("cargo", &["-V"], &env0, &tree_dir))),
                (
                    "linker",
                    s(tool_line("cc", &["--version"], &env0, &tree_dir)
                        .lines()
                        .next()
                        .unwrap_or("")
                        .to_owned()),
                ),
                ("host", s(&host)),
                ("Cargo.lock", s(hash("Cargo.lock"))),
                ("Cargo.toml", s(hash("Cargo.toml"))),
                ("rust-toolchain.toml", s(hash("rust-toolchain.toml"))),
                (".cargo/config.toml", s(hash(".cargo/config.toml"))),
            ]),
        ),
        ("build-configuration", build_configuration),
        ("programs", Json::Array(programs)),
        ("shared", Json::Array(shared)),
        ("refused-sites", s(sites.to_string())),
        ("admissions-unused", strings(unused)),
    ]);
    fs::create_dir_all(out).map_err(|e| e.to_string())?;
    fs::write(&target_json, json::write(&inv)).map_err(|e| e.to_string())?;
    Ok(Outcome::Written(inv))
}

/// `cargo xtask trust-inventory [--commit REV] [--out DIR]`.
pub fn run(repo: &Path, args: &[&str]) -> i32 {
    let mut commit = "HEAD".to_owned();
    let mut out = repo.join("target").join("trust");
    let mut i = 0;
    while i < args.len() {
        match (args[i], args.get(i + 1)) {
            ("--commit", Some(v)) => commit = (*v).to_owned(),
            ("--out", Some(v)) => out = repo.join(v), // absolute, so the configurations on its path are placed (§3)
            (other, _) => {
                eprintln!("trust-inventory: unknown argument `{other}`");
                return 2;
            }
        }
        i += 2;
    }
    match inventory(repo, &commit, &out) {
        Ok(Outcome::Written(inv)) => {
            let shared = inv.get("shared").map(Json::elements).unwrap_or_default();
            let items: usize = shared
                .iter()
                .map(|p| p.get("items").map(Json::elements).unwrap_or_default().len())
                .sum();
            println!(
                "trust-inventory: wrote {} — {} program(s), {} pair(s), {} shared item(s), {} refused site(s) each admitted",
                out.join("trust-dependencies.json").display(),
                inv.get("programs").map(Json::elements).unwrap_or_default().len(),
                shared.len(),
                items,
                inv.get("refused-sites").and_then(Json::as_str).unwrap_or("0"),
            );
            0
        }
        Ok(Outcome::Refused(refusals)) => {
            for r in &refusals {
                eprintln!("{r}");
            }
            eprintln!(
                "trust-inventory: {} refusal(s) — no inventory written",
                refusals.len()
            );
            1
        }
        Err(e) => {
            eprintln!("trust-inventory: {e}");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    //! Every channel a review measured by hand, a fixture (ledger `TI-H24`): a scratch repository under
    //! `target/trust-tests/`, committed, inventoried by the real instrument from its commit.

    use super::{inventory_with, line_hash, Outcome};
    use crate::json::Json;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    fn real_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
    }

    fn git(dir: &Path, args: &[&str]) {
        let out = Command::new("git")
            .args([
                "-c",
                "user.name=trust",
                "-c",
                "user.email=trust@example.invalid",
                "-c",
                "commit.gpgsign=false",
            ])
            .args(args)
            .current_dir(dir)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "git {args:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
    }

    struct Fixture {
        repo: PathBuf,
        base: PathBuf,
    }

    const WS: &str = "[workspace]\nresolver = \"2\"\nmembers = [\"crates/*\"]\n";

    fn manifest(name: &str, extra: &str) -> String {
        format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n{extra}")
    }

    /// Two roots, a binary `a` and a library `b`, as every fixture starts.
    const ROOTS: &str = "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-packages \"crates/a\"))\n\
                         (defroot chk (role scheduling-checker) (package \"crates/b\") (target lib) (role-packages \"crates/b\"))\n";

    fn write(dir: &Path, files: &[(&str, String)]) {
        for (path, text) in files {
            let full = dir.join(path);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(full, text).unwrap();
        }
    }

    /// A committed scratch repository holding `files`, the real pin and configuration, and a lock cargo writes.
    fn fixture(name: &str, toolchain: &str, files: &[(&str, String)]) -> Fixture {
        let base = real_root().join("target/trust-tests").join(name);
        let _ = std::fs::remove_dir_all(&base);
        let repo = base.join("repo");
        std::fs::create_dir_all(&repo).unwrap();
        let real = real_root();
        let mut all: Vec<(&str, String)> = vec![
            (
                ".cargo/config.toml",
                std::fs::read_to_string(real.join(".cargo/config.toml")).unwrap(),
            ),
            (
                "rust-toolchain.toml",
                format!("[toolchain]\nchannel = \"{toolchain}\"\n"),
            ),
        ];
        all.extend(files.iter().cloned());
        write(&repo, &all);
        let lock = Command::new("cargo")
            .args(["generate-lockfile", "--offline"])
            .env("RUSTUP_TOOLCHAIN", toolchain)
            .current_dir(&repo)
            .output()
            .unwrap();
        assert!(
            lock.status.success(),
            "{}",
            String::from_utf8_lossy(&lock.stderr)
        );
        git(&repo, &["init", "-q"]);
        git(&repo, &["add", "-A"]);
        git(&repo, &["commit", "-q", "-m", "fixture"]);
        Fixture { repo, base }
    }

    impl Fixture {
        fn commit(&self, files: &[(&str, String)]) {
            write(&self.repo, files);
            // A dependency changed is a lock changed: `--locked` refuses a stale one, rightly.
            let toolchain = std::fs::read_to_string(self.repo.join("rust-toolchain.toml")).unwrap();
            let channel = toolchain.split('"').nth(1).unwrap().to_owned();
            let lock = Command::new("cargo")
                .args(["generate-lockfile", "--offline"])
                .env("RUSTUP_TOOLCHAIN", channel)
                .current_dir(&self.repo)
                .output()
                .unwrap();
            assert!(
                lock.status.success(),
                "{}",
                String::from_utf8_lossy(&lock.stderr)
            );
            git(&self.repo, &["add", "-A"]);
            git(&self.repo, &["commit", "-q", "-m", "change"]);
        }
        fn run_into(&self, out: &str) -> Outcome {
            inventory_with(&self.repo, "HEAD", &self.base.join(out), &real_root()).unwrap()
        }
        fn run(&self) -> Outcome {
            self.run_into("out")
        }
    }

    fn written(o: Outcome) -> Json {
        match o {
            Outcome::Written(j) => j,
            Outcome::Refused(r) => panic!("refused: {r:#?}"),
        }
    }

    fn refused(o: Outcome) -> Vec<String> {
        match o {
            Outcome::Written(_) => panic!("written where a refusal was due"),
            Outcome::Refused(r) => r,
        }
    }

    fn says(refusals: &[String], text: &str) {
        assert!(
            refusals.iter().any(|r| r.contains(text)),
            "no refusal says `{text}`: {refusals:#?}"
        );
    }

    fn items(inv: &Json) -> Vec<(String, Json)> {
        let mut out = Vec::new();
        for pair in inv.get("shared").map(Json::elements).unwrap_or_default() {
            let names: Vec<&str> = pair
                .get("pair")
                .map(Json::elements)
                .unwrap_or_default()
                .iter()
                .filter_map(Json::as_str)
                .collect();
            for item in pair.get("items").map(Json::elements).unwrap_or_default() {
                out.push((names.join("+"), item.clone()));
            }
        }
        out
    }

    fn kind(item: &Json) -> &str {
        item.get("kind").and_then(Json::as_str).unwrap_or_default()
    }

    fn two_roots(name: &str, a_src: &str, b_src: &str, extra: &[(&str, String)]) -> Fixture {
        let mut files = vec![
            ("Cargo.toml", WS.to_owned()),
            ("trust/roots.eadl", ROOTS.to_owned()),
            ("crates/a/Cargo.toml", manifest("a", "")),
            ("crates/a/src/main.rs", a_src.to_owned()),
            ("crates/b/Cargo.toml", manifest("b", "")),
            ("crates/b/src/lib.rs", b_src.to_owned()),
        ];
        for (p, t) in extra {
            files.retain(|(q, _)| q != p);
            files.push((p, t.clone()));
        }
        fixture(name, "1.95.0", &files)
    }

    #[test]
    fn two_roots_sharing_nothing_share_only_the_build_configuration() {
        let f = two_roots("clean", "fn main() {}\n", "pub fn f() -> u32 { 1 }\n", &[]);
        let inv = written(f.run());
        let shared = items(&inv);
        assert_eq!(shared.len(), 1, "{shared:?}");
        assert_eq!(kind(&shared[0].1), "build-configuration");
    }

    #[test]
    fn a_package_both_roots_compile_is_shared_case_1() {
        let f = two_roots(
            "case1",
            "fn main() { common::c(); }\n",
            "pub fn f() -> u32 { common::c() }\n",
            &[
                ("crates/common/Cargo.toml", manifest("common", "")),
                (
                    "crates/common/src/lib.rs",
                    "pub fn c() -> u32 { 1 }\n".to_owned(),
                ),
                (
                    "crates/a/Cargo.toml",
                    manifest("a", "[dependencies]\ncommon = { path = \"../common\" }\n"),
                ),
                (
                    "crates/b/Cargo.toml",
                    manifest("b", "[dependencies]\ncommon = { path = \"../common\" }\n"),
                ),
            ],
        );
        let inv = written(f.run());
        assert!(items(&inv).iter().any(|(_, i)| kind(i) == "package"
            && i.get("package").and_then(Json::as_str) == Some("crates/common")));
    }

    #[test]
    fn an_incbin_under_each_assembler_macro_is_refused() {
        for (name, mac) in [
            ("global-asm", "core::arch::global_asm!"),
            ("renamed-use", "g!"),
        ] {
            let prelude = if name == "renamed-use" {
                "use core::arch::global_asm as g;\n"
            } else {
                ""
            };
            let f = two_roots(name, "fn main() {}\n", &format!("{prelude}{mac}(\".data\\n.incbin \\\"crates/b/blob.bin\\\"\");\npub fn f() {{}}\n"), &[
                ("crates/b/blob.bin", "SECRET-BYTES".to_owned()),
            ]);
            says(
                &refused(f.run()),
                "`global_asm`, refused by the catalog's rules",
            );
        }
        let f = two_roots(
            "asm",
            "fn main() {}\n",
            "pub fn f() { unsafe { core::arch::asm!(\"nop\") } }\n",
            &[],
        );
        says(&refused(f.run()), "`asm`, refused");
    }

    #[test]
    fn an_include_and_a_path_module_are_refused() {
        let f = two_roots(
            "include",
            "fn main() {}\n",
            "include!(\"../../../shared.rs\");\n",
            &[("shared.rs", "pub fn s() {}\n".to_owned())],
        );
        says(&refused(f.run()), "`include`, refused");
        let f = two_roots(
            "path-module",
            "fn main() {}\n",
            "#[path = \"m.txt\"]\nmod m;\npub use m::*;\n",
            &[("crates/b/src/m.txt", "pub fn m() {}\n".to_owned())],
        );
        says(&refused(f.run()), "`path`, refused");
    }

    #[test]
    fn a_macro_assembling_a_path_attribute_is_refused() {
        let f = two_roots("macro-path", "fn main() {}\n", "macro_rules! m { ($b:tt) => { # $b mod evil; } }\nm!([path = \"evil.txt\"]);\npub use evil::*;\n", &[("crates/b/src/evil.txt", "pub fn e() {}\n".to_owned())]);
        says(&refused(f.run()), "`macro_rules`, refused");
    }

    #[test]
    fn an_extern_block_calling_another_package_s_symbol_is_refused() {
        let f = two_roots(
            "extern-binding",
            "#[no_mangle]\npub fn transition(x: u32) -> u32 { x + 1 }\nfn main() { let _ = b::r(1); }\n",
            "extern \"Rust\" { fn transition(x: u32) -> u32; }\npub fn r(x: u32) -> u32 { unsafe { transition(x) } }\n",
            &[("crates/a/Cargo.toml", manifest("a", "[dependencies]\nb = { path = \"../b\" }\n"))],
        );
        let r = refused(f.run());
        says(&r, "`extern`");
        says(&r, "`no_mangle`, refused");
    }

    fn harness(name: &str, dev_dep: &str, dev_src: &str, dev_manifest_extra: &str) -> Fixture {
        let roots = "(defroot refm (role reference-model) (package \"crates/r\") (target lib) (role-packages \"crates/r\"))\n\
                     (defroot imp (role implementation) (package \"crates/i\") (target lib) (role-packages \"crates/i\"))\n\
                     (defharness diff (pair refm imp) (package \"crates/i\") (test diff))\n";
        fixture(
            name,
            "1.95.0",
            &[
                ("Cargo.toml", WS.to_owned()),
                ("trust/roots.eadl", roots.to_owned()),
                ("crates/r/Cargo.toml", manifest("r", "")),
                (
                    "crates/r/src/lib.rs",
                    "pub fn refm(x: u32) -> u32 { x }\n".to_owned(),
                ),
                (
                    "crates/i/Cargo.toml",
                    manifest(
                        "i",
                        &format!("[dev-dependencies]\nr = {{ path = \"../r\" }}\n{dev_dep}"),
                    ),
                ),
                (
                    "crates/i/src/lib.rs",
                    "pub fn imp(x: u32) -> u32 { x }\n".to_owned(),
                ),
                (
                    "crates/i/tests/diff.rs",
                    "#[test]\nfn same() { assert_eq!(i::imp(3), r::refm(3)); }\n".to_owned(),
                ),
                ("crates/d/Cargo.toml", manifest("d", dev_manifest_extra)),
                ("crates/d/src/lib.rs", dev_src.to_owned()),
            ],
        )
    }

    #[test]
    fn the_harness_is_inventoried_and_its_development_edges_followed() {
        let f = harness("harness", "", "pub fn d() {}\n", "");
        let inv = written(f.run());
        assert!(items(&inv)
            .iter()
            .any(|(pair, i)| pair == "refm+imp" && kind(i) == "comparison-harness"));
        let f = harness(
            "harness-no-mangle",
            "d = { path = \"../d\" }\n",
            "#[no_mangle]\npub fn helper() {}\n",
            "",
        );
        says(&refused(f.run()), "`no_mangle`, refused");
        let f = harness(
            "harness-proc-macro",
            "d = { path = \"../d\" }\n",
            "",
            "[lib]\nproc-macro = true\n",
        );
        says(&refused(f.run()), "reaches `crates/d`");
    }

    #[test]
    fn a_build_script_is_refused_before_anything_runs() {
        let f = two_roots(
            "build-script",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[(
                "crates/b/build.rs",
                "fn main() { std::fs::write(\"RAN\", \"\").unwrap(); }\n".to_owned(),
            )],
        );
        says(&refused(f.run()), "reaches `crates/b`");
        assert!(
            !f.base.join("out/tree/crates/b/RAN").exists(),
            "the build script ran"
        );
    }

    #[test]
    fn an_admitted_site_passes_and_an_edited_one_is_refused() {
        let line = "pub const T: &str = include_str!(\"t.txt\");";
        let admit = |l: &str| {
            format!("{ROOTS}(defadmit (file \"crates/b/src/lib.rs\") (rule \"include_str\") (line \"{}\"))\n", line_hash(l.as_bytes(), 1))
        };
        let f = two_roots(
            "admitted",
            "fn main() {}\n",
            &format!("{line}\n"),
            &[
                ("crates/b/src/t.txt", "table".to_owned()),
                ("trust/roots.eadl", admit(line)),
            ],
        );
        let inv = written(f.run());
        assert_eq!(inv.get("refused-sites").and_then(Json::as_str), Some("1"));
        f.commit(&[(
            "crates/b/src/lib.rs",
            "pub const T: &str = include_str!(\"t.txt\"); // edited\n".to_owned(),
        )]);
        says(&refused(f.run()), "no admission names it");
        assert!(
            !f.base.join("out/trust-dependencies.json").exists(),
            "an inventory written on a refusal"
        );
    }

    fn package_item<'a>(all: &'a [(String, Json)], package: &str) -> &'a Json {
        &all.iter()
            .find(|(_, i)| {
                kind(i) == "package" && i.get("package").and_then(Json::as_str) == Some(package)
            })
            .expect("the shared package")
            .1
    }

    fn list(item: &Json, key: &str) -> Vec<String> {
        item.get(key)
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .filter_map(Json::as_str)
            .map(str::to_owned)
            .collect()
    }

    #[test]
    fn a_new_consumer_of_a_shared_package_changes_its_edges() {
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let f = two_roots(
            "new-consumer",
            "fn main() { common::c(); }\n",
            "pub fn f() -> u32 { mid::m() }\n",
            &[
                ("crates/common/Cargo.toml", manifest("common", "")),
                (
                    "crates/common/src/lib.rs",
                    "pub fn c() -> u32 { 1 }\n".to_owned(),
                ),
                ("crates/mid/Cargo.toml", manifest("mid", dep)),
                (
                    "crates/mid/src/lib.rs",
                    "pub fn m() -> u32 { common::c() }\n".to_owned(),
                ),
                ("crates/a/Cargo.toml", manifest("a", dep)),
                (
                    "crates/b/Cargo.toml",
                    manifest("b", "[dependencies]\nmid = { path = \"../mid\" }\n"),
                ),
            ],
        );
        let before = list(
            package_item(&items(&written(f.run())), "crates/common"),
            "edges",
        );
        f.commit(&[
            ("crates/b/Cargo.toml", manifest("b", "[dependencies]\nmid = { path = \"../mid\" }\ncommon = { path = \"../common\" }\n")),
            ("crates/b/src/lib.rs", "pub fn f() -> u32 { mid::m() + common::c() }\n".to_owned()),
        ]);
        let after = list(
            package_item(&items(&written(f.run())), "crates/common"),
            "edges",
        );
        assert_ne!(
            before, after,
            "a new direct consumer left the edges as they were"
        );
        assert!(
            after.iter().any(|e| e == "chk:crates/b:normal"),
            "{after:?}"
        );
    }

    #[test]
    fn a_profile_edit_changes_the_build_configuration() {
        let f = two_roots("profile", "fn main() {}\n", "pub fn f() {}\n", &[]);
        let before = written(f.run());
        f.commit(&[(
            "Cargo.toml",
            format!("{WS}\n[profile.release]\npanic = \"abort\"\n"),
        )]);
        let after = written(f.run());
        assert_ne!(
            before.get("build-configuration"),
            after.get("build-configuration")
        );
    }

    #[test]
    fn a_feature_another_manifest_enables_does_not_reach_a_root_s_build() {
        // Default-deny closes the channel: the catalog's rules refuse `[features]` outright (R7) ...
        let common = manifest("common", "[features]\nx = []\n");
        let files = |roots: String| {
            vec![
                ("crates/common/Cargo.toml", common.clone()),
                ("crates/common/src/lib.rs", "pub fn c() {}\n".to_owned()),
                (
                    "crates/z/Cargo.toml",
                    manifest(
                        "z",
                        "[dependencies]\ncommon = { path = \"../common\", features = [\"x\"] }\n",
                    ),
                ),
                ("crates/z/src/lib.rs", "pub fn z() {}\n".to_owned()),
                (
                    "crates/a/Cargo.toml",
                    manifest("a", "[dependencies]\ncommon = { path = \"../common\" }\n"),
                ),
                ("trust/roots.eadl", roots),
            ]
        };
        let f = two_roots(
            "features",
            "fn main() { common::c(); }\n",
            "pub fn f() {}\n",
            &files(ROOTS.to_owned()),
        );
        says(
            &refused(f.run()),
            "`[features]` makes a second configuration",
        );
        // ... and, admitted, a feature another manifest enables does not reach a root's unit (R1 A10).
        let why = "`crates/common/Cargo.toml`: `[features]` makes a second configuration";
        let admitted = format!(
            "{ROOTS}(defadmit (file \"crates/common/Cargo.toml\") (rule \"{why}\") (line \"{}\"))\n",
            archogen_evidence::sha256::Digest::of(common.as_bytes()).hex()
        );
        let g = two_roots(
            "features-admitted",
            "fn main() { common::c(); }\n",
            "pub fn f() {}\n",
            &files(admitted),
        );
        let text = crate::json::write(&written(g.run()));
        assert!(
            !text.contains("feature=\\\"x\\\""),
            "workspace-wide feature unification reached a root's unit"
        );
    }

    #[test]
    fn naked_asm_is_refused() {
        let f = two_roots(
            "naked-asm",
            "fn main() {}\n",
            "#[unsafe(naked)]\npub extern \"C\" fn f() { core::arch::naked_asm!(\"ret\") }\n",
            &[],
        );
        says(&refused(f.run()), "`naked_asm`, refused");
    }

    #[test]
    fn a_new_reader_of_a_shared_package_s_file_changes_its_readers() {
        let common_line = "pub const T: &str = include_str!(\"t.txt\");";
        let b_line = "pub const U: &str = include_str!(\"../../common/src/t.txt\");";
        let roots = format!(
            "{ROOTS}(defadmit (file \"crates/common/src/lib.rs\") (rule \"include_str\") (line \"{}\"))\n\
             (defadmit (file \"crates/b/src/lib.rs\") (rule \"include_str\") (line \"{}\"))\n",
            line_hash(common_line.as_bytes(), 1),
            line_hash(b_line.as_bytes(), 1)
        );
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let f = two_roots(
            "new-reader",
            "fn main() { let _ = common::T; }\n",
            "pub fn f() -> &'static str { common::T }\n",
            &[
                ("crates/common/Cargo.toml", manifest("common", "")),
                ("crates/common/src/lib.rs", format!("{common_line}\n")),
                ("crates/common/src/t.txt", "table".to_owned()),
                ("crates/a/Cargo.toml", manifest("a", dep)),
                ("crates/b/Cargo.toml", manifest("b", dep)),
                ("trust/roots.eadl", roots),
            ],
        );
        let before = list(
            package_item(&items(&written(f.run())), "crates/common"),
            "readers",
        );
        f.commit(&[(
            "crates/b/src/lib.rs",
            format!("{b_line}\npub fn f() -> &'static str {{ common::T }}\n"),
        )]);
        let after = list(
            package_item(&items(&written(f.run())), "crates/common"),
            "readers",
        );
        assert_ne!(
            before, after,
            "a new reader of a shared package's file left its readers as they were"
        );
        assert!(
            after
                .iter()
                .any(|r| r == "crates/common/src/t.txt<chk:crates/b"),
            "{after:?}"
        );
    }

    fn configurations(inv: &Json) -> Vec<String> {
        let mut out = Vec::new();
        for p in inv.get("programs").map(Json::elements).unwrap_or_default() {
            for u in p.get("units").map(Json::elements).unwrap_or_default() {
                out.push(crate::json::write(u.get("configuration").unwrap()));
            }
        }
        out
    }

    #[test]
    fn the_configuration_is_the_same_from_two_directories_and_two_toolchains() {
        // Both roots share `common`, so a unit is compiled for each, and neither compiles the other's role.
        let shared = |name: &str| {
            two_roots(
                name,
                "fn main() { common::c(); }\n",
                "pub fn f() { common::c(); }\n",
                &[
                    ("crates/common/Cargo.toml", manifest("common", "")),
                    ("crates/common/src/lib.rs", "pub fn c() {}\n".to_owned()),
                    (
                        "crates/a/Cargo.toml",
                        manifest("a", "[dependencies]\ncommon = { path = \"../common\" }\n"),
                    ),
                    (
                        "crates/b/Cargo.toml",
                        manifest("b", "[dependencies]\ncommon = { path = \"../common\" }\n"),
                    ),
                ],
            )
        };
        let f = shared("two-dirs");
        let one = configurations(&written(f.run_into("one")));
        let two = configurations(&written(f.run_into("elsewhere/two")));
        assert_eq!(one, two);
        let g = shared("toolchain");
        g.commit(&[(
            "rust-toolchain.toml",
            "[toolchain]\nchannel = \"1.98.0\"\n".to_owned(),
        )]);
        let newer = written(g.run());
        assert_eq!(
            configurations(&newer),
            one,
            "a toolchain bump changed a unit's configuration"
        );
        assert_ne!(
            newer.get("build-configuration"),
            written(f.run()).get("build-configuration")
        );
    }

    #[test]
    fn an_executable_compiling_another_role_s_package_is_refused() {
        let f = two_roots(
            "two-roles",
            "fn main() { b::f(); }\n",
            "pub fn f() {}\n",
            &[(
                "crates/a/Cargo.toml",
                manifest("a", "[dependencies]\nb = { path = \"../b\" }\n"),
            )],
        );
        // `a` compiles `b`, the scheduling checker's role package.
        says(
            &refused(f.run()),
            "trust-shared-program: `gen` compiles `crates/b`",
        );
    }
}
