//! `cargo xtask trust-inventory` — the trust gate's measuring instrument (leaf `M3.6.2`,
//! `docs/specs/trust/decision_trust-inventory.md` §2–§4, `docs/decisions/decision_executable-design-reviews.md`).
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
//! program's compilation reads, however it is read, the harness's development edges followed; a crate root that is
//! not a `.rs` file is refused before any build (R8 1). A site the rules refuse passes only when an admission in
//! `trust/roots.eadl` names its file, its rule and the sha256 of its extent — the statement or item it stands in
//! (R8 4) — one admission per site (R8 3). Data other than a `.rs` file is hashed and never tokenised (R6 7; R8 10).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_catalog::manifest;
use archogen_catalog::package;
use archogen_catalog::tree::Tree;
use archogen_evidence::sha256::Digest;
use eadl_front::{read, Form, SourceMap};

use crate::catalog_build::{
    configurations_on_path, environment, toolchain_file, write_tree, ENV_ALLOWED,
};
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
    /// The fixed data the pipeline hands it at run time, by repository path (§2, R9 remark 6).
    pub data: Vec<String>,
}

/// The roles a root runs (§2): `ROADMAP.md` §4.4's four and the implementation a reference model validates. The
/// comparison harness is a form of its own, never a root's role (R9 2).
pub const ROLES: &[&str] = &[
    "generator",
    "configuration-checker",
    "scheduling-checker",
    "reference-model",
    "implementation",
];

/// A refused site admitted by review (§3).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Admission {
    /// The file.
    pub file: String,
    /// The rule: the refused word as the catalog's scanner names it.
    pub rule: String,
    /// The sha256 of the site's extent ([`site_extent`]), or, for a manifest rule, of the whole manifest.
    pub sha256: String,
}

/// The kinds of program target a classification names (§2): an executable — a `[[bin]]`, a `src/main.rs`, a
/// `src/bin/*.rs` — an example, and a library built as a `cdylib`, `staticlib` or `dylib`.
pub const PROGRAM_KINDS: &[&str] = &["bin", "example", "cdylib", "staticlib", "dylib"];

/// A program target that is not a root, classified by review (§2): the reason, and the role packages its build
/// compiles, so a set that grows is unreviewed again (R2 B6; R4 5; R5 remark 13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classification {
    /// Its name in the roots file.
    pub name: String,
    /// Its package's directory, relative to the repository.
    pub package: String,
    /// Which of [`PROGRAM_KINDS`] it is.
    pub kind: String,
    /// The target's name.
    pub target: String,
    /// The role packages its build compiles, as reviewed.
    pub role_packages: BTreeSet<String>,
}

/// The roots file, read.
#[derive(Debug, Clone, Default)]
pub struct Roots {
    /// Every root and the harness.
    pub programs: Vec<Program>,
    /// Every admission, with how many forms name it: each admits one site (R8 3).
    pub admissions: BTreeMap<Admission, usize>,
    /// Every program target classified as not a root (§2).
    pub classified: Vec<Classification>,
}

/// Each form the instrument reads, and the clauses it takes; any other form or clause is refused (R8 11).
const FORMS: &[(&str, &[&str])] = &[
    (
        "defroot",
        &["role", "package", "target", "role-packages", "data"],
    ),
    ("defharness", &["pair", "package", "test"]),
    ("defadmit", &["file", "rule", "sha256", "reason"]),
    // A program target that is not a root (§2): `M3.6.3.1`.
    (
        "defprogram",
        &["package", "target", "reason", "role-packages"],
    ),
    // A role no root fills yet (§2): the gate's, `M3.6.3`.
    ("defrole", &["leaf"]),
];

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

/// Read the roots file, strictly: a form or a clause the instrument does not know, a clause twice, a name twice, and
/// a harness whose pair names no root are refused (R8 11).
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
    let mut names: BTreeSet<String> = BTreeSet::new();
    for form in &doc.forms {
        let head = form.head().unwrap_or("?");
        let Some((_, allowed)) = FORMS.iter().find(|(h, _)| *h == head) else {
            return Err(format!(
                "{ROOTS}: a form `{head}` the instrument does not know"
            ));
        };
        // An admission has no name; every other form names itself first.
        let named = head != "defadmit";
        let name = if named {
            form.items().get(1).and_then(word).unwrap_or_default()
        } else {
            "an admission".to_owned()
        };
        if named && !names.insert(name.clone()) {
            return Err(format!("{ROOTS}: `{name}` is named twice"));
        }
        let mut seen = BTreeSet::new();
        for c in form.items().iter().skip(if named { 2 } else { 1 }) {
            let Some(h) = c.head() else {
                return Err(format!(
                    "{ROOTS}: `{name}` holds something that is not a clause"
                ));
            };
            if !allowed.contains(&h) {
                return Err(format!(
                    "{ROOTS}: `{name}` holds a clause `{h}` its form does not take"
                ));
            }
            if !seen.insert(h) {
                return Err(format!("{ROOTS}: `{name}` holds `{h}` twice"));
            }
        }
        match head {
            "defroot" => {
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
                let role = one(form, "role", &name)?;
                if !ROLES.contains(&role.as_str()) {
                    return Err(format!(
                        "{ROOTS}: `{name}`'s role `{role}` is none of the record's: {}",
                        ROLES.join(", ")
                    ));
                }
                roots.programs.push(Program {
                    role,
                    package,
                    target,
                    role_packages,
                    pair: None,
                    data: values(form, "data"),
                    name,
                });
            }
            "defharness" => {
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
                    data: Vec::new(),
                    name,
                });
            }
            "defprogram" => {
                one(form, "reason", &name)?;
                let (kind, target) = match values(form, "target").as_slice() {
                    [k, n] if PROGRAM_KINDS.contains(&k.as_str()) => (k.clone(), n.clone()),
                    _ => {
                        return Err(format!(
                            "{ROOTS}: `{name}`'s target is `(target KIND NAME)`, KIND one of {}",
                            PROGRAM_KINDS.join(", ")
                        ))
                    }
                };
                roots.classified.push(Classification {
                    package: one(form, "package", &name)?,
                    kind,
                    target,
                    role_packages: values(form, "role-packages").into_iter().collect(),
                    name,
                });
            }
            "defadmit" => {
                one(form, "reason", "an admission")?;
                *roots
                    .admissions
                    .entry(Admission {
                        file: one(form, "file", "an admission")?,
                        rule: one(form, "rule", "an admission")?,
                        sha256: one(form, "sha256", "an admission")?,
                    })
                    .or_default() += 1;
            }
            _ => {}
        }
    }
    // A harness compares two distinct roots the file declares.
    for h in &roots.programs {
        let Some((a, b)) = &h.pair else { continue };
        for x in [a, b] {
            if !roots
                .programs
                .iter()
                .any(|p| &p.name == x && p.pair.is_none())
            {
                return Err(format!(
                    "{ROOTS}: `{}`'s pair names `{x}`, which names no root",
                    h.name
                ));
            }
        }
        if a == b {
            return Err(format!("{ROOTS}: `{}`'s pair names `{a}` twice", h.name));
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

/// A unit's configuration as an item compares it: its normalised command line and every `# env-dep` line with its
/// value, since a value compiled in through `env!` is configuration as much as a flag is (§3, R8 5).
fn unit_configuration(u: &Unit) -> String {
    let mut c = u.configuration.join(" ");
    for e in &u.env {
        c.push_str(" env-dep:");
        c.push_str(e);
    }
    c
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
                // `-C incremental=` names the scratch build's own cache directory, as `--out-dir` does (R9 1).
                if v.starts_with("metadata=")
                    || v.starts_with("extra-filename=")
                    || v.starts_with("incremental=")
                {
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
    /// The root's artifact — the executable, the library or the test binary — relative to its own target directory,
    /// with its sha256 (§3, R8 9).
    artifact: (String, String),
}

/// What cargo printed: its standard output (the JSON messages) and its standard error (`-v`'s command lines).
fn run_cargo(
    args: &[&str],
    env: &[(String, String)],
    cwd: &Path,
) -> Result<(String, String), String> {
    let mut c = Command::new("cargo");
    c.args(args).current_dir(cwd).env_clear();
    for (k, v) in env {
        c.env(k, v);
    }
    let out = c.output().map_err(|e| format!("cargo did not run: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if !out.status.success() {
        let first = stderr
            .lines()
            .find(|l| l.starts_with("error"))
            .unwrap_or("")
            .to_owned();
        return Err(format!("`cargo {}` failed: {first}", args.join(" ")));
    }
    Ok((stdout, stderr))
}

/// The packages' targets from `cargo metadata`: each crate root, as a repository path, with its crate name (a
/// target's name, `-` written `_`, as rustc's `--crate-name` takes it), to its package's directory (R8 2).
pub type Targets = BTreeMap<(String, String), String>;

/// Build `p` clean in `tree_dir`, into `target_dir`, and read every unit's dependency information.
#[allow(clippy::too_many_arguments)]
fn build(
    p: &Program,
    package_names: &BTreeMap<String, String>,
    targets: &Targets,
    id_dir: &BTreeMap<String, String>,
    env: &[(String, String)],
    tree_dir: &Path,
    target_dir: &Path,
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
            "--message-format=json",
            "-v",
            "-p",
        ]
        .map(str::to_owned),
    );
    args.push(name.clone());
    let argv: Vec<&str> = args.iter().map(String::as_str).collect();
    let (stdout, stderr) = run_cargo(&argv, env, tree_dir)?;
    let scratch = format!("{}/", tree_dir.display());
    let canonical_scratch = fs::canonicalize(tree_dir)
        .map(|c| format!("{}/", c.display()))
        .unwrap_or_else(|_| scratch.clone());
    let strip = |t: &str| -> String {
        t.strip_prefix(&scratch)
            .or_else(|| t.strip_prefix(&canonical_scratch))
            .unwrap_or(t)
            .to_owned()
    };
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
        // The unit's package is the one whose target names its crate root, as cargo's metadata states it — never
        // the longest directory prefixing the source, which a crate root in a nested package's directory defeats
        // (R8 2).
        let owners: BTreeSet<&String> = tokens
            .iter()
            .skip(1)
            .filter_map(|t| targets.get(&(strip(t), crate_name.clone())))
            .collect();
        let package = match owners.into_iter().collect::<Vec<_>>().as_slice() {
            [one] => (*one).clone(),
            _ => {
                refused.push(format!(
                    "trust-undeclared-input: `{}` compiles `{crate_name}`, a unit whose crate root no one target of \
                     the commit's packages names",
                    p.name
                ));
                continue;
            }
        };
        let d = PathBuf::from(&out_dir).join(format!("{crate_name}{extra}.d"));
        let text = fs::read_to_string(&d).map_err(|e| format!("{}: {e}", d.display()))?;
        let mut files = BTreeSet::new();
        let mut env_deps = Vec::new();
        let mut first = true;
        for l in text.lines() {
            if let Some(dep) = l.strip_prefix("# env-dep:") {
                // Only cargo's own variables, which the commit's manifests decide: the catalog's list (R8 remark 22).
                let var = dep.split('=').next().unwrap_or_default();
                if !(var.starts_with("CARGO_PKG_") || ENV_ALLOWED.contains(&var)) {
                    refused.push(format!(
                        "trust-undeclared-input: `{crate_name}` in `{}` depends on the environment variable `{var}`, \
                         which the commit does not hold",
                        p.name
                    ));
                }
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
        let configuration = normalise(&tokens, &scratch);
        // A path the normalisation leaves absolute names something outside the written tree, which the commit does
        // not hold and another checkout would spell otherwise (R9 1).
        for t in &configuration {
            if t.starts_with('/') || t.contains("=/") {
                refused.push(format!(
                    "trust-undeclared-input: `{crate_name}` in `{}` is configured with `{t}`, a path outside the \
                     written tree",
                    p.name
                ));
            }
        }
        units.push(Unit {
            crate_name,
            package,
            configuration,
            files,
            env: env_deps,
        });
    }
    // In a fixed order, not cargo's scheduling order, so one commit's inventory is one sequence of bytes (R8 remark
    // 18).
    units.sort_by(|a, b| {
        (&a.package, &a.crate_name, &a.configuration).cmp(&(
            &b.package,
            &b.crate_name,
            &b.configuration,
        ))
    });
    if !units.iter().any(|u| u.package == p.package) {
        return Err(format!(
            "`{}`'s build compiled nothing of its own package `{}`",
            p.name, p.package
        ));
    }
    // The root's artifact, from cargo's own report of what it wrote (R8 9).
    let mut artifact = None;
    for line in stdout.lines() {
        let Ok(m) = json::parse(line) else { continue };
        if m.get("reason").and_then(Json::as_str) != Some("compiler-artifact") {
            continue;
        }
        let pkg = m
            .get("package_id")
            .and_then(Json::as_str)
            .and_then(|id| id_dir.get(id));
        if pkg != Some(&p.package) {
            continue;
        }
        let target = m.get("target");
        let tname = target
            .and_then(|t| t.get("name"))
            .and_then(Json::as_str)
            .unwrap_or_default();
        let kinds: Vec<String> = target
            .and_then(|t| t.get("kind"))
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .filter_map(Json::as_str)
            .map(str::to_owned)
            .collect();
        let executable = m.get("executable").and_then(Json::as_str);
        let file = match &p.target {
            Target::Bin(b) if tname == b && kinds.iter().any(|k| k == "bin") => {
                executable.map(str::to_owned)
            }
            Target::Test(t) if tname == t && kinds.iter().any(|k| k == "test") => {
                executable.map(str::to_owned)
            }
            Target::Lib if kinds.iter().any(|k| k.ends_with("lib")) => m
                .get("filenames")
                .map(Json::elements)
                .unwrap_or_default()
                .iter()
                .filter_map(Json::as_str)
                .find(|f| f.ends_with(".rlib"))
                .map(str::to_owned),
            _ => None,
        };
        if let Some(f) = file {
            let bytes = fs::read(&f).map_err(|e| format!("{f}: {e}"))?;
            let canonical_target = fs::canonicalize(target_dir)
                .map(|c| format!("{}/", c.display()))
                .unwrap_or_default();
            let rel = f
                .strip_prefix(&format!("{}/", target_dir.display()))
                .or_else(|| f.strip_prefix(&canonical_target))
                .unwrap_or(&f)
                .to_owned();
            artifact = Some((rel, Digest::of(&bytes).hex()));
        }
    }
    let artifact = artifact.ok_or_else(|| format!("`{}`'s build reported no artifact", p.name))?;
    Ok(Build { units, artifact })
}

// ── Refusals by the catalog's rules (§3) ────────────────────────────────────────────────────────────────────────

/// Every site the catalog's token rules refuse in `source`, by blanking each and scanning again: `(line, column,
/// rule)`.
fn refused_sites(source: &[u8]) -> Result<Vec<(usize, usize, String)>, String> {
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
                sites.push((found.line, found.column, rule));
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

/// The extent of the site the catalog's scanner refused at `line` and `column`: the statement or item it stands in
/// (§3, R8 4). Over the catalog's own tokens, at the innermost brace level holding the site — the file, a module, a
/// block or a brace-delimited macro's body — it runs from the token after the previous `;` or brace group at that
/// level, or the level's start, through the next `;` or the end of the next brace group at that level, so parentheses
/// and brackets never end it. A macro invocation is whole within it, an attribute runs with the item it marks through
/// the item's body, and an edit anywhere in either changes the extent's sha256.
///
/// # Errors
///
/// A source that does not tokenize, a position on no token, or a delimiter without its partner.
pub fn site_extent(source: &[u8], line: usize, column: usize) -> Result<&[u8], String> {
    use archogen_catalog::package::Kind;
    let toks = package::tokens(source).map_err(|(at, why)| format!("byte {at}: {why}"))?;
    let offset = source
        .split(|&b| b == b'\n')
        .take(line.saturating_sub(1))
        .map(|l| l.len() + 1)
        .sum::<usize>()
        + column.saturating_sub(1);
    let k = toks
        .iter()
        .position(|t| t.at == offset)
        .ok_or_else(|| format!("line {line} column {column} starts no token"))?;
    let mut partner = vec![usize::MAX; toks.len()];
    let mut stack = Vec::new();
    let mut enclosing = Vec::new();
    for (i, t) in toks.iter().enumerate() {
        if i == k {
            enclosing.clone_from(&stack);
        }
        match t.kind {
            Kind::Open(_) => stack.push(i),
            Kind::Close(_) => {
                let o = stack
                    .pop()
                    .ok_or_else(|| format!("byte {}: a delimiter closes nothing", t.at))?;
                partner[o] = i;
                partner[i] = o;
            }
            _ => {}
        }
    }
    if !stack.is_empty() {
        return Err("a delimiter is never closed".to_owned());
    }
    // The innermost brace holding the site sets the level; the outermost group inside it holding the site, or the
    // site's own token, is where the walk starts.
    let level = enclosing
        .iter()
        .rposition(|&o| toks[o].kind == Kind::Open('{'));
    let (lo, hi) = level.map_or((0, toks.len()), |j| {
        let o = enclosing[j];
        (o + 1, partner[o])
    });
    let anchor = level
        .map_or(enclosing.first(), |j| enclosing.get(j + 1))
        .copied()
        .unwrap_or(k);
    // A brace group between `<` or `,` and `>` or `,` is a const generic argument, `A<{ 1 }>`, inside an item's
    // header, never the end of a statement (R9 3).
    let generic = |open: usize| {
        let close = partner[open];
        open > 0
            && matches!(toks[open - 1].kind, Kind::Punct('<' | ','))
            && toks
                .get(close + 1)
                .is_some_and(|t| matches!(t.kind, Kind::Punct('>' | ',')))
    };
    let mut start = anchor;
    let mut i = anchor;
    while i > lo {
        let t = &toks[i - 1];
        match t.kind {
            Kind::Close('}') if generic(partner[i - 1]) => i = partner[i - 1],
            Kind::Punct(';') | Kind::Close('}') => break,
            Kind::Close(_) => i = partner[i - 1],
            _ => i -= 1,
        }
        start = i;
    }
    let mut end = anchor;
    let mut i = anchor;
    while i < hi {
        match toks[i].kind {
            Kind::Open('{') if generic(i) => {
                end = partner[i];
                i = partner[i] + 1;
            }
            Kind::Open('{') => {
                end = partner[i];
                break;
            }
            Kind::Open(_) => {
                end = partner[i];
                i = partner[i] + 1;
            }
            Kind::Punct(';') => {
                end = i;
                break;
            }
            _ => {
                end = i;
                i += 1;
            }
        }
    }
    Ok(&source[toks[start].at..toks[end].end])
}

/// Why a site may not be admitted, if it may not: its extent renames its refused identifier with `use … as`, or defines
/// a macro whose body holds a refused construct. Either makes a name no rule refuses stand for the construct, so its
/// invocations would escape every extent (R10 1).
fn inadmissible(rule: &str, extent: &[u8]) -> Option<&'static str> {
    use archogen_catalog::package::Kind;
    let Ok(toks) = package::tokens(extent) else {
        return Some("in a text that does not tokenize");
    };
    let ident = |t: &package::Token, name: &str| matches!(&t.kind, Kind::Ident(n, _) if n == name);
    if toks
        .windows(2)
        .any(|w| ident(&w[0], rule) && ident(&w[1], "as"))
    {
        return Some("renamed by `use … as`");
    }
    let defines = toks
        .iter()
        .any(|t| ident(t, "macro_rules") || ident(t, "macro"));
    let wraps = refused_sites(extent).map_or(true, |found| {
        found
            .iter()
            .any(|(_, _, r)| r != "macro_rules" && r != "macro")
    });
    (defines && wraps).then_some("defined in a macro that wraps a refused construct")
}

/// What an `include!` or a `path = "…"` in `extent` compiles as Rust, where it is not one `.rs` file named by a single
/// string literal: each offending name, or a description of what stands in its place (R9 remark 7).
fn compiled_names(rule: &str, extent: &[u8]) -> Vec<String> {
    use archogen_catalog::package::Kind;
    let Ok(toks) = package::tokens(extent) else {
        return vec!["a text that does not tokenize".to_owned()];
    };
    let literal = |t: &package::Token| -> String {
        let text = String::from_utf8_lossy(&extent[t.at..t.end]).into_owned();
        match (text.find('"'), text.rfind('"')) {
            (Some(a), Some(b)) if a < b => text[a + 1..b].to_owned(),
            _ => text,
        }
    };
    let mut out = Vec::new();
    for (i, t) in toks.iter().enumerate() {
        if t.kind != Kind::Ident(rule.to_owned(), false) {
            continue;
        }
        let next = |k: usize| toks.get(i + k).map(|t| &t.kind);
        let named = match rule {
            // `include!( "x.rs" )`: one string literal between the delimiters.
            "include" if next(1) == Some(&Kind::Punct('!')) => match (next(2), next(3), next(4)) {
                (Some(Kind::Open(_)), Some(Kind::Str), Some(Kind::Close(_))) => {
                    Some(literal(&toks[i + 3]))
                }
                _ => Some("an argument other than one string literal".to_owned()),
            },
            // `path = "x.rs"`, in an attribute or a macro's arguments.
            "path" if next(1) == Some(&Kind::Punct('=')) => match next(2) {
                Some(Kind::Str) => Some(literal(&toks[i + 2])),
                _ => Some("a value other than one string literal".to_owned()),
            },
            _ => None,
        };
        if let Some(n) = named {
            if std::path::Path::new(&n)
                .extension()
                .is_none_or(|e| e != "rs")
            {
                out.push(n);
            }
        }
    }
    out
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

/// Every program target of the commit, each a root's, classified, unclassified, or classified with role packages that
/// have grown since its review (§2), and every classification whose program target is gone, which the gate refuses as
/// `trust-baseline-stale` on the baseline's host (§6, R7 remark e). A target's role packages are those its build
/// compiles — its package's closure by normal edges, an example's by its package's development edges too, since an
/// example compiles them (R5 remark 13) — among every root's role packages.
fn classify_programs(
    roots: &Roots,
    program_targets: &[(String, String, String)],
    graph: &[(String, String, String)],
) -> (Vec<Json>, Vec<String>) {
    let role_packages: BTreeSet<&String> = roots
        .programs
        .iter()
        .filter(|p| p.role != "harness")
        .flat_map(|p| p.role_packages.iter())
        .collect();
    let mut out = Vec::new();
    let mut matched: BTreeSet<&String> = BTreeSet::new();
    for (package, kind, name) in program_targets {
        let root = roots.programs.iter().find(|p| {
            p.role != "harness"
                && p.package == *package
                && kind == "bin"
                && p.target == Target::Bin(name.clone())
        });
        let mut closure: BTreeSet<String> = BTreeSet::new();
        let mut todo = vec![package.clone()];
        while let Some(pkg) = todo.pop() {
            if !closure.insert(pkg.clone()) {
                continue;
            }
            for (from, to, edge) in graph {
                let followed =
                    edge == "normal" || (kind == "example" && edge == "dev" && *from == *package);
                if *from == pkg && followed && !closure.contains(to) {
                    todo.push(to.clone());
                }
            }
        }
        let compiled: BTreeSet<String> = closure
            .into_iter()
            .filter(|p| role_packages.contains(p))
            .collect();
        let classification = roots
            .classified
            .iter()
            .find(|c| c.package == *package && c.kind == *kind && c.target == *name);
        let status = match (root, classification) {
            (Some(r), _) => format!("root {}", r.name),
            (None, Some(c)) => {
                matched.insert(&c.name);
                let grown: Vec<&String> = compiled.difference(&c.role_packages).collect();
                if grown.is_empty() {
                    format!("classified {}", c.name)
                } else {
                    format!(
                        "trust-unclassified-program: `{}` compiles {} beside the role packages its classification reviewed",
                        c.name,
                        grown.iter().map(|g| format!("`{g}`")).collect::<Vec<_>>().join(", ")
                    )
                }
            }
            (None, None) => format!(
                "trust-unclassified-program: the {kind} `{name}` of `{package}` is neither a root nor classified in {ROOTS}"
            ),
        };
        out.push(obj(vec![
            ("package", s(package)),
            ("kind", s(kind)),
            ("name", s(name)),
            ("status", s(status)),
            ("role-packages", strings(compiled)),
        ]));
    }
    let unused = roots
        .classified
        .iter()
        .filter(|c| !matched.contains(&c.name))
        .map(|c| {
            format!(
                "{}: the {} `{}` of `{}` is no program target of the commit",
                c.name, c.kind, c.target, c.package
            )
        })
        .collect();
    (out, unused)
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
    // The fixed data a root is handed, by path: a blob of the commit, never a symbolic link (§2, §3; R9 remark 6).
    for p in &roots.programs {
        for d in &p.data {
            if symlinks.contains(d) || tree.get(d).is_none() {
                refused.push(format!(
                    "trust-undeclared-input: `{}` declares `{d}`, which is not a blob of the commit",
                    p.name
                ));
            }
        }
    }

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
    let mut targets: Targets = BTreeMap::new();
    let mut package_targets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    // Every program target (§2): (package, kind, name).
    let mut program_targets: Vec<(String, String, String)> = Vec::new();
    // Every target, by package: its kinds and its name, so a root's target is known to exist.
    let mut every_target: BTreeMap<String, Vec<(Vec<String>, String)>> = BTreeMap::new();
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
        for t in p.get("targets").map(Json::elements).unwrap_or_default() {
            let src = t.get("src_path").and_then(Json::as_str).unwrap_or_default();
            let src = src.strip_prefix(&root_prefix).unwrap_or(src).to_owned();
            let crate_name = t
                .get("name")
                .and_then(Json::as_str)
                .unwrap_or_default()
                .replace('-', "_");
            package_targets
                .entry(dir.clone())
                .or_default()
                .push(src.clone());
            let kinds: Vec<&str> = t
                .get("kind")
                .map(Json::elements)
                .unwrap_or_default()
                .iter()
                .filter_map(Json::as_str)
                .collect();
            let target_name = t.get("name").and_then(Json::as_str).unwrap_or_default();
            if let Some(kind) = PROGRAM_KINDS.iter().find(|k| kinds.contains(k)) {
                program_targets.push((dir.clone(), (*kind).to_owned(), target_name.to_owned()));
            }
            every_target.entry(dir.clone()).or_default().push((
                kinds.iter().map(|k| (*k).to_owned()).collect(),
                target_name.to_owned(),
            ));
            targets.insert((src, crate_name), dir.clone());
        }
        package_names.insert(dir.clone(), name);
        id_dir.insert(id, dir);
    }
    // Every package a form names is a package of the commit, so a misspelt role package cannot turn the two-roles
    // rule off for the package it meant (R9 remark 5).
    let named_packages = roots
        .programs
        .iter()
        .map(|p| {
            (
                &p.name,
                std::iter::once(&p.package)
                    .chain(p.role_packages.iter())
                    .collect::<Vec<_>>(),
            )
        })
        .chain(roots.classified.iter().map(|c| {
            (
                &c.name,
                std::iter::once(&c.package)
                    .chain(c.role_packages.iter())
                    .collect(),
            )
        }));
    // A form naming a package, or a root naming a target, the commit no longer has is stale (§6, R7 remark e): refused
    // wherever the gate runs, since no inventory can be built without it (`M3.6.3.3`).
    let mut stale = Vec::new();
    for (name, packages) in named_packages {
        for pkg in packages {
            if !package_names.contains_key(pkg) {
                stale.push(format!(
                    "trust-baseline-stale: {ROOTS}: `{name}` names `{pkg}`, which is no package of the commit"
                ));
            }
        }
    }
    for p in &roots.programs {
        let Some(have) = every_target.get(&p.package) else {
            continue;
        };
        let (kind, wanted) = match &p.target {
            Target::Lib => ("lib", None),
            Target::Bin(n) => ("bin", Some(n)),
            Target::Test(n) => ("test", Some(n)),
        };
        let found = have.iter().any(|(kinds, n)| {
            kinds
                .iter()
                .any(|k| k == kind || (kind == "lib" && k.ends_with("lib")))
                && wanted.is_none_or(|w| w == n)
        });
        if !found {
            stale.push(format!(
                "trust-baseline-stale: {ROOTS}: `{}` names the {kind} target{} of `{}`, which the commit no longer has",
                p.name,
                wanted.map_or(String::new(), |w| format!(" `{w}`")),
                p.package
            ));
        }
    }
    if !stale.is_empty() {
        return Ok(Outcome::Refused(stale));
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

    let (program_targets, classifications_unused) =
        classify_programs(&roots, &program_targets, &graph);

    let mut sites = 0usize;
    let mut used: BTreeMap<Admission, usize> = BTreeMap::new();
    let mut manifests_checked = BTreeSet::new();
    // The catalog's rules on a workspace manifest — no `[patch]` or `[replace]` swapping a dependency's source, no
    // `cargo-features`, no `rustflags` — over the root manifest every build reads (R8 remark 16), admitted, like a
    // package's manifest rule, only by the manifest's sha256.
    if let Some(bytes) = tree.get("Cargo.toml") {
        match manifest::parse(&String::from_utf8_lossy(bytes)) {
            Ok(m) => {
                if let Err(why) = package::check_workspace("Cargo.toml", &m) {
                    sites += 1;
                    let admission = Admission {
                        file: "Cargo.toml".to_owned(),
                        rule: why.clone(),
                        sha256: Digest::of(bytes).hex(),
                    };
                    if roots.admissions.contains_key(&admission) {
                        *used.entry(admission).or_default() += 1;
                    } else {
                        refused.push(format!("trust-undeclared-input: {why}, and no admission names it (manifest sha256 {})", admission.sha256));
                    }
                }
            }
            Err(o) => refused.push(format!(
                "trust-undeclared-input: `Cargo.toml` line {}: {}",
                o.line, o.why
            )),
        }
    }

    // The manifest rules over every package in a program's closure, from the metadata graph, before any build:
    // a build script or a procedural macro is refused here, so none runs (§3, R2 B12); the harness's development
    // edges are followed, since its build compiles them (R7 3).
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
            // rustc compiles a crate root whatever its name, and no token rule reads a file that is not `.rs`; `mod`
            // finds only `.rs` files, and `#[path]` and `include!` are refused, so with this every Rust source a root
            // compiles is a `.rs` file (R8 1).
            for src in package_targets.get(&pkg).into_iter().flatten() {
                if std::path::Path::new(src)
                    .extension()
                    .is_none_or(|e| e != "rs")
                {
                    refused.push(format!("trust-undeclared-input: `{}` reaches `{src}`, a crate root that is not a `.rs` file, which rustc compiles and no rule reads", p.name));
                }
            }
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
                            sha256: Digest::of(bytes).hex(),
                        };
                        if roots.admissions.contains_key(&admission) {
                            *used.entry(admission).or_default() += 1;
                        } else {
                            refused.push(format!("trust-undeclared-input: `{}` reaches `{pkg}`: {why}, and no admission names it (manifest sha256 {})", p.name, admission.sha256));
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
        let target_dir = out.join("target").join(&p.name);
        let env = environment(&pin, &cargo_home, &target_dir)?;
        builds.insert(
            p.name.clone(),
            build(
                p,
                &package_names,
                &targets,
                &id_dir,
                &env,
                &tree_dir,
                &target_dir,
                &mut refused,
            )?,
        );
    }

    // The token rules over every `.rs` file a compilation read, however it read it (§3; R8 10), each file once, so a
    // site two programs compile is one site (R8 remark 20).
    let mut compiled: BTreeMap<&String, BTreeSet<&String>> = BTreeMap::new();
    for (name, b) in &builds {
        for u in &b.units {
            for f in &u.files {
                compiled.entry(f).or_default().insert(name);
            }
        }
    }
    let mut found_sites: BTreeMap<Admission, (Vec<String>, Vec<u8>)> = BTreeMap::new();
    for (f, readers) in &compiled {
        let name = readers.iter().next().map_or("", |n| n.as_str());
        if symlinks.contains(*f) {
            refused.push(format!(
                "trust-undeclared-input: `{name}` reads `{f}`, a symbolic link in the commit"
            ));
            continue;
        }
        let Some(blob) = tree.get(f) else {
            refused.push(format!(
                "trust-undeclared-input: `{name}` reads `{f}`, which is not a blob of the commit"
            ));
            continue;
        };
        if fs::read(tree_dir.join(f)).ok().as_deref() != Some(blob) {
            refused.push(format!(
                "trust-undeclared-input: `{f}`'s bytes after the build are not its blob's"
            ));
        }
        if !std::path::Path::new(f.as_str())
            .extension()
            .is_some_and(|e| e.eq_ignore_ascii_case("rs"))
        {
            continue; // data other than Rust is hashed and never tokenised (R6 7)
        }
        match refused_sites(blob) {
            Ok(found) => {
                for (line, column, rule) in found {
                    sites += 1;
                    match site_extent(blob, line, column) {
                        Ok(extent) => found_sites
                            .entry(Admission {
                                file: (*f).clone(),
                                rule,
                                sha256: Digest::of(extent).hex(),
                            })
                            .or_insert_with(|| (Vec::new(), extent.to_vec()))
                            .0
                            .push(format!("{f}:{line}")),
                        Err(why) => refused.push(format!(
                            "trust-undeclared-input: `{f}:{line}`: `{rule}`, a site whose extent cannot be read: {why}"
                        )),
                    }
                }
            }
            Err(why) => refused.push(format!(
                "trust-undeclared-input: `{f}` does not tokenize: {why}"
            )),
        }
    }
    // One admission per site: sites sharing a file, a rule and an extent's text pass only as many as the admissions
    // naming them (R8 3).
    for (key, (at, extent)) in &found_sites {
        // An alias or a wrapper of a refused construct is inadmissible: its invocations name nothing a rule refuses,
        // so no extent would pin them (R10 1).
        if let Some(how) = inadmissible(&key.rule, extent) {
            for site in at {
                refused.push(format!("trust-undeclared-input: `{site}`: `{}` {how}, so every invocation of it would go unpinned — inadmissible, whatever admits it", key.rule));
            }
        }
        // An `include!` or a `path = "…"` compiles the file it names as Rust, and the rules read only `.rs` files:
        // admitted, it must name one in a single string literal, or what it compiles would go unread (R9 remark 7).
        if key.rule == "include" || key.rule == "path" {
            for named in compiled_names(&key.rule, extent) {
                for site in at {
                    refused.push(format!("trust-undeclared-input: `{site}`: `{}` names `{named}`, which is not a `.rs` file, so no rule would read what it compiles", key.rule));
                }
            }
        }
        let admitted = roots.admissions.get(key).copied().unwrap_or(0);
        if at.len() > admitted {
            for site in at {
                if admitted == 0 {
                    refused.push(format!("trust-undeclared-input: `{site}`: `{}`, refused by the catalog's rules, and no admission names it (site sha256 {})", key.rule, key.sha256));
                } else {
                    refused.push(format!("trust-undeclared-input: `{site}`: `{}`, refused by the catalog's rules: {} sites share this text and {admitted} admission(s) name it (site sha256 {})", key.rule, at.len(), key.sha256));
                }
            }
        }
        *used.entry(key.clone()).or_default() += at.len().min(admitted);
    }

    // A program that runs two roles (§2): any root whose build compiles a role package of another role — a library
    // root too, which runs its role until an executable does (R8 remark 13) — and the harness, whose exemption is its
    // pair's two (R8 remark 12).
    for p in &roots.programs {
        let compiled: BTreeSet<&String> =
            builds[&p.name].units.iter().map(|u| &u.package).collect();
        for other in roots
            .programs
            .iter()
            .filter(|o| o.pair.is_none() && o.role != p.role)
        {
            let in_pair = p
                .pair
                .as_ref()
                .is_some_and(|(a, b)| *a == other.name || *b == other.name);
            if in_pair {
                continue;
            }
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
    // The profile tables that apply to a root's units: `[profile.release]` and its overrides for a package some
    // program compiles, by name or by `*`, never the build overrides, which apply to build scripts and procedural
    // macros, both refused. The edition reaches rustc only as a unit's `--edition`, which its configuration holds, and
    // the toolchain only as its identity, so neither the edition key nor the toolchain file's bytes are here (R8 6).
    let compiled_names: BTreeSet<&String> = builds
        .values()
        .flat_map(|b| b.units.iter())
        .filter_map(|u| package_names.get(&u.package))
        .collect();
    let mut profile = Vec::new();
    if let Some(Ok(m)) = tree
        .get("Cargo.toml")
        .map(|b| manifest::parse(&String::from_utf8_lossy(b)))
    {
        for (path, value) in &m.values {
            let applies = match path.as_slice() {
                [p, r, rest @ ..] if p == "profile" && r == "release" => match rest {
                    [o, ..] if o == "build-override" => false,
                    [o, spec, ..] if o == "package" => {
                        let name = spec.split('@').next().unwrap_or_default();
                        name == "*" || compiled_names.iter().any(|n| n.as_str() == name)
                    }
                    _ => true,
                },
                _ => false,
            };
            if applies {
                profile.push(format!("{}={value:?}", path.join(".")));
            }
        }
    }
    profile.sort();
    let build_configuration = obj(vec![
        ("toolchain", s(&rustc)),
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
        // Each package with its manifest's sha256 and the kind of every edge that reached it in this build — the
        // harness's development edges from its own package, no other (§3, R8 9).
        let package_records: Vec<Json> = packages
            .iter()
            .map(|pkg| {
                let manifest = if pkg.is_empty() {
                    "Cargo.toml".to_owned()
                } else {
                    format!("{pkg}/Cargo.toml")
                };
                let edges: BTreeSet<String> = graph
                    .iter()
                    .filter(|(from, to, kind)| {
                        to == pkg
                            && packages.contains(from)
                            && (kind != "dev" || p.role == "harness" && *from == p.package)
                    })
                    .map(|(from, _, kind)| format!("{from}:{kind}"))
                    .collect();
                obj(vec![
                    ("package", s(pkg)),
                    ("manifest", s(file_hash(&manifest))),
                    ("edges", strings(edges)),
                ])
            })
            .collect();
        programs.push(obj(vec![
            ("name", s(&p.name)),
            ("role", s(&p.role)),
            (
                "artifact",
                obj(vec![
                    ("path", s(&b.artifact.0)),
                    ("sha256", s(&b.artifact.1)),
                ]),
            ),
            ("packages", Json::Array(package_records)),
            (
                "data",
                Json::Object(
                    p.data
                        .iter()
                        .map(|d| (d.clone(), s(file_hash(d))))
                        .collect(),
                ),
            ),
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
            // The harness is paired with every root but its pair's two, whose sharing with it is the pair's own
            // harness item (§4, R8 8).
            let own_pair = |h: &Program, x: &Program| {
                h.pair
                    .as_ref()
                    .is_some_and(|(p, q)| *p == x.name || *q == x.name)
            };
            if own_pair(a, b) || own_pair(b, a) {
                continue;
            }
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
                        .map(unit_configuration)
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
            // What each reads: what its compilation read, and the data it is handed at run time (§4, R6 2).
            let files = |x: &Build, prog: &Program| {
                x.units
                    .iter()
                    .flat_map(|u| u.files.iter().cloned())
                    .chain(prog.data.iter().cloned())
                    .collect::<BTreeSet<_>>()
            };
            let (fa, fb) = (files(ba, a), files(bb, b));
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
                // Every unit it compiles beside the pair's two roots' builds: its test target, the adapters its
                // development dependencies add, and any unit compiled otherwise than in either root's build (§4,
                // R8 7).
                let key = |u: &Unit| {
                    (
                        u.package.clone(),
                        u.crate_name.clone(),
                        unit_configuration(u),
                    )
                };
                let in_pair: BTreeSet<_> =
                    ba.units.iter().chain(bb.units.iter()).map(key).collect();
                let beside: Vec<&Unit> = builds[&h.name]
                    .units
                    .iter()
                    .filter(|u| !in_pair.contains(&key(u)))
                    .collect();
                let content: BTreeSet<String> = beside
                    .iter()
                    .flat_map(|u| u.files.iter().map(|f| format!("{f}={}", file_hash(f))))
                    .collect();
                items.push(obj(vec![
                    ("kind", s("comparison-harness")),
                    ("harness", s(&h.name)),
                    ("content", strings(content)),
                    (
                        "configuration",
                        strings(beside.iter().map(|u| unit_configuration(u))),
                    ),
                ]));
            }
            shared.push(obj(vec![
                ("pair", strings([a.name.clone(), b.name.clone()])),
                ("items", Json::Array(items)),
            ]));
        }
    }

    // An admission no current site uses: listed here, refused by the gate on the baseline's host as
    // `trust-baseline-stale` (§6, R8 remark 15).
    let unused: Vec<String> = roots
        .admissions
        .iter()
        .flat_map(|(a, n)| {
            let left = n.saturating_sub(used.get(a).copied().unwrap_or(0));
            std::iter::repeat_n(format!("{}:{}:{}", a.file, a.rule, a.sha256), left)
        })
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
        ("program-targets", Json::Array(program_targets)),
        ("classifications-unused", strings(classifications_unused)),
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
pub(crate) mod tests {
    //! Every channel a review measured by hand, a fixture (ledger `TI-H24`): a scratch repository under
    //! `target/trust-tests/`, committed, inventoried by the real instrument from its commit.

    use super::{inventory_with, read_roots, site_extent, Digest, Outcome};
    use crate::json::Json;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    pub(crate) fn real_root() -> PathBuf {
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

    pub(crate) struct Fixture {
        pub(crate) repo: PathBuf,
        pub(crate) base: PathBuf,
    }

    const WS: &str = "[workspace]\nresolver = \"2\"\nmembers = [\"crates/*\"]\n";

    pub(crate) fn manifest(name: &str, extra: &str) -> String {
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
        pub(crate) fn commit(&self, files: &[(&str, String)]) {
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

    pub(crate) fn two_roots(
        name: &str,
        a_src: &str,
        b_src: &str,
        extra: &[(&str, String)],
    ) -> Fixture {
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

    /// Each program target's status, by `package kind name`.
    fn statuses(inv: &Json) -> Vec<(String, String)> {
        inv.get("program-targets")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .map(|t| {
                let f = |k: &str| {
                    t.get(k)
                        .and_then(Json::as_str)
                        .unwrap_or_default()
                        .to_owned()
                };
                (
                    format!("{} {} {}", f("package"), f("kind"), f("name")),
                    f("status"),
                )
            })
            .collect()
    }

    #[test]
    fn every_program_target_is_a_root_or_classified_and_the_rest_is_reported() {
        // Record §2, leaf `M3.6.3.1`: a binary `c` beside the two roots, then an example of `b` whose development edge
        // reaches the generator's package, which an example compiles (R5 remark 13).
        let f = two_roots(
            "programs",
            "fn main() {}\n",
            "pub fn f() -> u32 { 1 }\n",
            &[
                ("crates/c/Cargo.toml", manifest("c", "")),
                ("crates/c/src/main.rs", "fn main() {}\n".to_owned()),
            ],
        );
        let inv = written(f.run());
        let s = statuses(&inv);
        assert!(
            s.contains(&("crates/a bin a".to_owned(), "root gen".to_owned())),
            "{s:?}"
        );
        let c = &s
            .iter()
            .find(|(t, _)| t == "crates/c bin c")
            .expect("c is a program target")
            .1;
        assert!(
            c.starts_with("trust-unclassified-program: the bin `c`"),
            "{c}"
        );
        // Classified, it passes; the gate reads acceptance, never this file's presence (§5).
        f.commit(&[(
            "trust/roots.eadl",
            format!("{ROOTS}(defprogram tool (package \"crates/c\") (target bin c) (reason \"a tool\") (role-packages))\n"),
        )]);
        let inv = written(f.run());
        assert!(
            statuses(&inv).contains(&("crates/c bin c".to_owned(), "classified tool".to_owned()))
        );
        // An example of `c` whose development edge reaches `a`, a role package: the classification's set has grown.
        f.commit(&[
            ("crates/c/Cargo.toml", manifest("c", "[dev-dependencies]\na = { path = \"../a\" }\n")),
            ("crates/a/src/lib.rs", "pub fn g() {}\n".to_owned()),
            ("crates/c/examples/e.rs", "fn main() {}\n".to_owned()),
            (
                "trust/roots.eadl",
                format!(
                    "{ROOTS}(defprogram tool (package \"crates/c\") (target bin c) (reason \"a tool\") (role-packages))\n\
                     (defprogram demo (package \"crates/c\") (target example e) (reason \"a demo\") (role-packages))\n"
                ),
            ),
        ]);
        let inv = written(f.run());
        let s = statuses(&inv);
        let e = &s
            .iter()
            .find(|(t, _)| t == "crates/c example e")
            .expect("an example")
            .1;
        assert_eq!(e, "trust-unclassified-program: `demo` compiles `crates/a` beside the role packages its classification reviewed");
        let c = &s.iter().find(|(t, _)| t == "crates/c bin c").expect("c").1;
        assert_eq!(
            c, "classified tool",
            "a binary follows normal edges alone: {c}"
        );
        // A classification whose target is gone is listed for `trust-baseline-stale` (§6, R7 remark e).
        f.commit(&[(
            "trust/roots.eadl",
            format!(
                "{ROOTS}(defprogram tool (package \"crates/c\") (target bin c) (reason \"a tool\") (role-packages))\n\
                 (defprogram demo (package \"crates/c\") (target example e) (reason \"a demo\") (role-packages \"crates/a\"))\n\
                 (defprogram gone (package \"crates/c\") (target bin old) (reason \"renamed\") (role-packages))\n"
            ),
        )]);
        let inv = written(f.run());
        let unused: Vec<&str> = inv
            .get("classifications-unused")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .filter_map(Json::as_str)
            .collect();
        assert_eq!(
            unused,
            ["gone: the bin `old` of `crates/c` is no program target of the commit"]
        );
        assert!(statuses(&inv).contains(&(
            "crates/c example e".to_owned(),
            "classified demo".to_owned()
        )));
    }

    #[test]
    fn a_classification_is_read_strictly() {
        for (text, why) in [
            (
                "(defprogram x (package \"p\") (target test t) (reason \"r\"))",
                "KIND one of",
            ),
            (
                "(defprogram x (package \"p\") (target bin t))",
                "needs exactly one `(reason",
            ),
            (
                "(defprogram x (package \"p\") (target bin t) (reason \"r\") (role \"g\"))",
                "a clause `role`",
            ),
        ] {
            let err = read_roots(text).expect_err(text);
            assert!(err.contains(why), "{text}: {err}");
        }
        let roots = read_roots(
            "(defprogram x (package \"p\") (target cdylib m) (reason \"r\") (role-packages \"q\"))",
        )
        .expect("reads");
        assert_eq!(roots.classified[0].kind, "cdylib");
        assert!(roots.classified[0].role_packages.contains("q"));
    }

    #[test]
    fn a_form_naming_a_package_or_a_target_the_commit_no_longer_has_is_stale() {
        // §6, R7 remark e: `trust-baseline-stale`, refused wherever the gate runs, since no inventory can be built
        // without what the form names (`M3.6.3.3`).
        for (name, roots, why) in [
            (
                "stale-package",
                ROOTS.replace("\"crates/b\") (target lib) (role-packages \"crates/b\")", "\"crates/gone\") (target lib) (role-packages \"crates/gone\")"),
                "`chk` names `crates/gone`, which is no package of the commit",
            ),
            (
                "stale-target",
                ROOTS.replace("(target bin a)", "(target bin gone)"),
                "`gen` names the bin target `gone` of `crates/a`, which the commit no longer has",
            ),
            (
                "stale-classified",
                format!("{ROOTS}(defprogram t (package \"crates/gone\") (target bin t) (reason \"r\") (role-packages \"crates/a\"))\n"),
                "`t` names `crates/gone`, which is no package of the commit",
            ),
        ] {
            let f = two_roots(name, "fn main() {}\n", "pub fn f() {}\n", &[("trust/roots.eadl", roots)]);
            let r = refused(f.run());
            says(&r, &format!("trust-baseline-stale: trust/roots.eadl: {why}"));
        }
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
        let f = two_roots(
            "admitted",
            "fn main() {}\n",
            &format!("{line}\n"),
            &[
                ("crates/b/src/t.txt", "table".to_owned()),
                ("crates/b/src/u.txt", "other".to_owned()),
                (
                    "trust/roots.eadl",
                    format!(
                        "{ROOTS}{}",
                        admit("crates/b/src/lib.rs", "include_str", line)
                    ),
                ),
            ],
        );
        let inv = written(f.run());
        assert_eq!(inv.get("refused-sites").and_then(Json::as_str), Some("1"));
        f.commit(&[(
            "crates/b/src/lib.rs",
            "pub const T: &str = include_str!(\"u.txt\");\n".to_owned(),
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
            "{ROOTS}(defadmit (file \"crates/common/Cargo.toml\") (rule \"{why}\") (sha256 \"{}\") (reason \"a fixture\"))\n",
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
            "{ROOTS}{}{}",
            admit("crates/common/src/lib.rs", "include_str", common_line),
            admit("crates/b/src/lib.rs", "include_str", b_line)
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

    // ── Round 8 (`M3.6.1` step 9): each reproducer the reader ran, as a fixture ─────────────────────────────────────

    /// The clause an admission names its site's sha256 in.
    const ADMIT: &str = "sha256";

    /// An admission of the site whose extent — the text §3 hashes — is `extent`.
    fn admit(file: &str, rule: &str, extent: &str) -> String {
        format!(
            "(defadmit (file \"{file}\") (rule \"{rule}\") ({ADMIT} \"{}\") (reason \"a fixture\"))\n",
            Digest::of(extent.as_bytes()).hex()
        )
    }

    fn program<'a>(inv: &'a Json, name: &str) -> &'a Json {
        inv.get("programs")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .find(|p| p.get("name").and_then(Json::as_str) == Some(name))
            .expect("the program")
    }

    fn pairs(inv: &Json) -> Vec<String> {
        let mut out: Vec<String> = items(inv).into_iter().map(|(p, _)| p).collect();
        out.dedup();
        out
    }

    /// Three roots and a harness: `refm` and `imp` its pair, `chk` a third role, `d` a package only the harness may
    /// compile.
    fn three_roots(name: &str, test: &str, test_src: &str, dev: &str, c_deps: &str) -> Fixture {
        let roots = format!(
            "(defroot refm (role reference-model) (package \"crates/r\") (target lib) (role-packages \"crates/r\"))\n\
             (defroot imp (role implementation) (package \"crates/i\") (target lib) (role-packages \"crates/i\"))\n\
             (defroot chk (role scheduling-checker) (package \"crates/c\") (target lib) (role-packages \"crates/c\"))\n\
             (defharness diff (pair refm imp) (package \"crates/i\") (test {test}))\n"
        );
        let test_path = format!("crates/i/tests/{test}.rs");
        fixture(
            name,
            "1.95.0",
            &[
                ("Cargo.toml", WS.to_owned()),
                ("trust/roots.eadl", roots),
                ("crates/r/Cargo.toml", manifest("r", "")),
                (
                    "crates/r/src/lib.rs",
                    "pub fn refm(x: u32) -> u32 { x }\n".to_owned(),
                ),
                ("crates/c/Cargo.toml", manifest("c", c_deps)),
                ("crates/c/src/lib.rs", "pub fn c() {}\n".to_owned()),
                (
                    "crates/i/Cargo.toml",
                    manifest(
                        "i",
                        &format!("[dev-dependencies]\nr = {{ path = \"../r\" }}\n{dev}"),
                    ),
                ),
                (
                    "crates/i/src/lib.rs",
                    "pub fn imp(x: u32) -> u32 { x }\n".to_owned(),
                ),
                (&test_path, test_src.to_owned()),
                ("crates/d/Cargo.toml", manifest("d", "")),
                (
                    "crates/d/src/lib.rs",
                    "pub fn adapt(x: u32) -> u32 { x }\n".to_owned(),
                ),
            ],
        )
    }

    const SAME: &str = "#[test]\nfn same() { assert_eq!(i::imp(3), r::refm(3)); }\n";

    fn harness_item(inv: &Json) -> Json {
        items(inv)
            .into_iter()
            .find(|(p, i)| p == "refm+imp" && kind(i) == "comparison-harness")
            .expect("the pair's harness item")
            .1
    }

    #[test]
    fn a_crate_root_that_is_not_a_rs_file_is_refused_before_any_build() {
        // R8 1: rustc compiles `lib.txt` as Rust, and no token rule read it.
        let f = two_roots(
            "r8-1-crate-root",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[
                ("crates/b/Cargo.toml", manifest("b", "[lib]\npath = \"src/lib.txt\"\n")),
                (
                    "crates/b/src/lib.txt",
                    "core::arch::global_asm!(\".data\\n.incbin \\\"shared.bin\\\"\");\npub fn f() {}\n".to_owned(),
                ),
                ("shared.bin", "SECRET-SHARED-BYTES".to_owned()),
            ],
        );
        says(
            &refused(f.run()),
            "`crates/b/src/lib.txt`, a crate root that is not a `.rs` file",
        );
        assert!(
            !f.base.join("out/target").exists(),
            "a build ran before the refusal"
        );
    }

    #[test]
    fn a_unit_belongs_to_the_package_whose_target_names_its_crate_root() {
        // R8 2: `b`'s library lives in a nested package's directory; by path prefix it was that package's.
        let f = two_roots(
            "r8-2-nested",
            "fn main() { b::f(); }\n",
            "pub fn f() {}\n",
            &[
                (
                    "Cargo.toml",
                    "[workspace]\nresolver = \"2\"\nmembers = [\"crates/a\", \"crates/b\", \"crates/b/x\"]\n".to_owned(),
                ),
                ("crates/a/Cargo.toml", manifest("a", "[dependencies]\nb = { path = \"../b\" }\n")),
                ("crates/b/Cargo.toml", manifest("b", "[lib]\npath = \"x/src/lib.rs\"\n")),
                ("crates/b/x/Cargo.toml", manifest("x", "")),
                ("crates/b/x/src/lib.rs", "pub fn f() {}\n".to_owned()),
            ],
        );
        says(
            &refused(f.run()),
            "trust-shared-program: `gen` compiles `crates/b`, a role package of `chk`",
        );
    }

    #[test]
    fn each_site_needs_an_admission_of_its_own() {
        // R8 3: two sites with one text need two admissions; a new `#[no_mangle]` beside an admitted one is refused.
        let item = "pub static X: &str = include_str!(\"../d.txt\");";
        let src = format!("pub mod m1 {{ {item} }}\npub mod m2 {{ {item} }}\n");
        let one = admit("crates/b/src/lib.rs", "include_str", item);
        let f = two_roots(
            "r8-3-two-sites",
            "fn main() {}\n",
            &src,
            &[
                ("crates/b/d.txt", "data".to_owned()),
                ("trust/roots.eadl", format!("{ROOTS}{one}{one}")),
            ],
        );
        assert_eq!(
            written(f.run()).get("refused-sites").and_then(Json::as_str),
            Some("2")
        );
        f.commit(&[("trust/roots.eadl", format!("{ROOTS}{one}"))]);
        says(&refused(f.run()), "`crates/b/src/lib.rs:2`: `include_str`");

        let first = "#[no_mangle]\npub extern \"C\" fn one() -> u32 { 1 }";
        let f = two_roots(
            "r8-3-no-mangle",
            "fn main() {}\n",
            &format!("{first}\n"),
            &[(
                "trust/roots.eadl",
                format!(
                    "{ROOTS}{}",
                    admit("crates/b/src/lib.rs", "no_mangle", first)
                ),
            )],
        );
        assert_eq!(
            written(f.run()).get("refused-sites").and_then(Json::as_str),
            Some("1")
        );
        f.commit(&[(
            "crates/b/src/lib.rs",
            format!("{first}\n#[no_mangle]\npub extern \"C\" fn memcmp() -> u32 {{ 2 }}\n"),
        )]);
        says(&refused(f.run()), "`crates/b/src/lib.rs:3`: `no_mangle`");
    }

    #[test]
    fn an_admission_covers_its_site_s_whole_extent() {
        // R8 4: an `.incbin` added on the second line of an admitted `global_asm!`.
        let asm = "core::arch::global_asm!(\n    \".data\"\n);";
        let f = two_roots(
            "r8-4-extent",
            "fn main() {}\n",
            &format!("{asm}\npub fn f() {{}}\n"),
            &[
                ("crates/b/blob.bin", "SECRET-BYTES".to_owned()),
                (
                    "trust/roots.eadl",
                    format!("{ROOTS}{}", admit("crates/b/src/lib.rs", "global_asm", asm)),
                ),
            ],
        );
        assert_eq!(
            written(f.run()).get("refused-sites").and_then(Json::as_str),
            Some("1")
        );
        f.commit(&[(
            "crates/b/src/lib.rs",
            "core::arch::global_asm!(\n    \".data\\n.incbin \\\"crates/b/blob.bin\\\"\"\n);\npub fn f() {}\n".to_owned(),
        )]);
        says(&refused(f.run()), "`crates/b/src/lib.rs:1`: `global_asm`");
    }

    #[test]
    fn a_shared_package_s_environment_is_in_its_item_and_a_foreign_variable_is_refused() {
        // R8 5: a value compiled in through `env!` changes; R8 remark 22: a variable from outside the commit.
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let common = "[package]\nname = \"common\"\nversion = \"0.1.0\"\nedition = \"2021\"\nlicense.workspace = true\n";
        let f = two_roots(
            "r8-5-env",
            "fn main() { println!(\"{}\", common::L); }\n",
            "pub fn f() -> &'static str { common::L }\n",
            &[
                (
                    "Cargo.toml",
                    format!("{WS}\n[workspace.package]\nlicense = \"MIT\"\n"),
                ),
                ("crates/common/Cargo.toml", common.to_owned()),
                (
                    "crates/common/src/lib.rs",
                    "pub const L: &str = env!(\"CARGO_PKG_LICENSE\");\n".to_owned(),
                ),
                ("crates/a/Cargo.toml", manifest("a", dep)),
                ("crates/b/Cargo.toml", manifest("b", dep)),
            ],
        );
        let before = written(f.run());
        f.commit(&[(
            "Cargo.toml",
            format!("{WS}\n[workspace.package]\nlicense = \"GPL-3.0-only\"\n"),
        )]);
        let after = written(f.run());
        assert_ne!(
            package_item(&items(&before), "crates/common").get("configuration"),
            package_item(&items(&after), "crates/common").get("configuration"),
            "a value compiled into a shared package changed and its item did not"
        );
        let f = two_roots(
            "r8-5-foreign-env",
            "fn main() {}\n",
            "pub const H: Option<&str> = option_env!(\"HOME\");\n",
            &[],
        );
        says(&refused(f.run()), "the environment variable `HOME`");
    }

    #[test]
    fn an_edit_no_root_s_compilation_reads_leaves_the_build_configuration() {
        // R8 6: a development profile, an override for a package no root compiles, a comment in the pin.
        let f = two_roots("r8-6-unrelated", "fn main() {}\n", "pub fn f() {}\n", &[]);
        let v0 = written(f.run());
        f.commit(&[(
            "Cargo.toml",
            format!("{WS}\n[profile.dev]\nopt-level = 1\n\n[profile.release.package.nobody]\nopt-level = 1\n"),
        )]);
        let v1 = written(f.run());
        assert_eq!(v0.get("build-configuration"), v1.get("build-configuration"));
        let pin = std::fs::read_to_string(f.repo.join("rust-toolchain.toml")).unwrap();
        f.commit(&[("rust-toolchain.toml", format!("# the pin\n{pin}"))]);
        let v2 = written(f.run());
        assert_eq!(v1.get("build-configuration"), v2.get("build-configuration"));
        f.commit(&[(
            "Cargo.toml",
            format!("{WS}\n[profile.release.package.b]\nopt-level = 1\n"),
        )]);
        assert_ne!(
            v2.get("build-configuration"),
            written(f.run()).get("build-configuration"),
            "an override for a package a root compiles changed nothing"
        );
    }

    #[test]
    fn the_harness_item_holds_what_it_compiles_beside_its_pair() {
        // R8 7: the adapter in a development dependency; a test named with a `-`.
        let f = three_roots(
            "r8-7-adapter",
            "diff",
            "#[test]\nfn same() { assert_eq!(d::adapt(i::imp(3)), r::refm(3)); }\n",
            "d = { path = \"../d\" }\n",
            "",
        );
        let before = harness_item(&written(f.run()));
        assert!(
            list(&before, "content")
                .iter()
                .any(|c| c.starts_with("crates/d/src/lib.rs=")),
            "{before:?}"
        );
        f.commit(&[(
            "crates/d/src/lib.rs",
            "pub fn adapt(x: u32) -> u32 { x.min(2) }\n".to_owned(),
        )]);
        assert_ne!(before, harness_item(&written(f.run())));
        let f = three_roots("r8-7-hyphen", "diff-check", SAME, "", "");
        assert!(!list(&harness_item(&written(f.run())), "content").is_empty());
    }

    #[test]
    fn the_harness_is_paired_with_every_root_but_its_pair_s() {
        // R8 8.
        let f = three_roots("r8-8-pairs", "diff", SAME, "", "");
        let p = pairs(&written(f.run()));
        assert!(
            !p.iter().any(|x| x == "refm+diff" || x == "imp+diff"),
            "{p:?}"
        );
        assert!(p.iter().any(|x| x == "chk+diff"), "{p:?}");
    }

    #[test]
    fn each_root_records_its_artifact_and_its_packages() {
        // R8 9: the artifact with its sha256; each package with its manifest's sha256 and the edges that reached it.
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let f = two_roots(
            "r8-9-artifact",
            "fn main() { common::c(); }\n",
            "pub fn f() {}\n",
            &[
                ("crates/common/Cargo.toml", manifest("common", "")),
                ("crates/common/src/lib.rs", "pub fn c() {}\n".to_owned()),
                ("crates/a/Cargo.toml", manifest("a", dep)),
            ],
        );
        let inv = written(f.run());
        let gen = program(&inv, "gen");
        let artifact = gen.get("artifact").expect("the root's artifact");
        let path = artifact.get("path").and_then(Json::as_str).unwrap();
        let bytes = std::fs::read(f.base.join("out/target/gen").join(path)).unwrap();
        assert_eq!(
            artifact.get("sha256").and_then(Json::as_str),
            Some(Digest::of(&bytes).hex().as_str())
        );
        let common = gen
            .get("packages")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .find(|p| p.get("package").and_then(Json::as_str) == Some("crates/common"))
            .expect("common among gen's packages");
        let manifest_bytes = std::fs::read(f.repo.join("crates/common/Cargo.toml")).unwrap();
        assert_eq!(
            common.get("manifest").and_then(Json::as_str),
            Some(Digest::of(&manifest_bytes).hex().as_str())
        );
        assert_eq!(list(common, "edges"), ["crates/a:normal"]);
    }

    #[test]
    fn a_rs_file_read_as_data_is_tokenised_too() {
        // R8 10: a `.rs` template read through an admitted `include_str!` is still Rust to the rules.
        let line = "pub const T: &str = include_str!(\"tpl.rs\");";
        let f = two_roots(
            "r8-10-rs-data",
            "fn main() {}\n",
            &format!("{line}\n"),
            &[
                (
                    "crates/b/src/tpl.rs",
                    "#[no_mangle]\npub extern \"C\" fn entry() {}\n".to_owned(),
                ),
                (
                    "trust/roots.eadl",
                    format!(
                        "{ROOTS}{}",
                        admit("crates/b/src/lib.rs", "include_str", line)
                    ),
                ),
            ],
        );
        says(&refused(f.run()), "`crates/b/src/tpl.rs:1`: `no_mangle`");
    }

    #[test]
    fn the_roots_file_is_read_strictly() {
        // R8 11: a name twice, a pair naming no root, a misspelt clause, a form no reader knows yet.
        let gen = "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-packages \"crates/a\"))\n";
        for (text, why) in [
            (
                format!("{gen}(defroot gen (role scheduling-checker) (package \"crates/b\") (target lib))\n"),
                "`gen` is named twice",
            ),
            (
                format!("{gen}(defharness diff (pair gen implementation) (package \"crates/a\") (test d))\n"),
                "`implementation`, which names no root",
            ),
            (
                "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-pakages \"crates/x\"))\n".to_owned(),
                "a clause `role-pakages`",
            ),
            (
                format!("{gen}(defclassify w (target \"crates/w\"))\n"),
                "a form `defclassify`",
            ),
        ] {
            let e = read_roots(&text).expect_err(why);
            assert!(e.contains(why), "{why}: {e}");
        }
    }

    #[test]
    fn a_harness_or_a_library_root_compiling_another_role_s_package_is_refused() {
        // R8 remarks 12, 13: the harness's exemption is its pair's two; a library root runs its role as an executable
        // would.
        let f = three_roots(
            "r8-12-harness",
            "diff",
            SAME,
            "c = { path = \"../c\" }\n",
            "",
        );
        says(
            &refused(f.run()),
            "trust-shared-program: `diff` compiles `crates/c`, a role package of `chk`",
        );
        let f = three_roots(
            "r8-13-library",
            "diff",
            SAME,
            "",
            "[dependencies]\nr = { path = \"../r\" }\n",
        );
        says(
            &refused(f.run()),
            "trust-shared-program: `chk` compiles `crates/r`, a role package of `refm`",
        );
    }

    #[test]
    fn an_incbin_under_asm_and_under_naked_asm_is_refused() {
        // R8 remark 21: the channel itself, not `nop` or `ret`.
        // `.previous`, not `.text`: on ELF a function has a section of its own, and a naked one's `.size` must
        // be measured in it (`PROGRAM.10.5.2`).
        for (name, src) in [
            (
                "r8-21-asm",
                "pub fn f() { unsafe { core::arch::asm!(\".data\\n.incbin \\\"crates/b/blob.bin\\\"\\n.previous\") } }\n",
            ),
            (
                "r8-21-naked-asm",
                "#[unsafe(naked)]\npub extern \"C\" fn f() { core::arch::naked_asm!(\".data\\n.incbin \\\"crates/b/blob.bin\\\"\\n.previous\\nret\") }\n",
            ),
        ] {
            let f = two_roots(
                name,
                "fn main() {}\n",
                src,
                &[("crates/b/blob.bin", "SECRET-BYTES".to_owned())],
            );
            says(&refused(f.run()), "asm`, refused by the catalog's rules");
        }
    }

    #[test]
    fn a_no_mangle_memcmp_in_the_harness_is_refused() {
        // R8 remark 21: the interposition R7 measured, in the harness's own test file.
        let f = three_roots(
            "r8-21-memcmp",
            "diff",
            &format!("#[no_mangle]\npub extern \"C\" fn memcmp(a: *const u8, b: *const u8, n: usize) -> i32 {{ let _ = (a, b, n); 0 }}\n{SAME}"),
            "",
            "",
        );
        says(&refused(f.run()), "`crates/i/tests/diff.rs:1`: `no_mangle`");
    }

    #[test]
    fn a_second_run_writes_the_same_bytes() {
        // R8 remark 18: units in a fixed order, not cargo's scheduling order.
        let f = three_roots(
            "r8-18-order",
            "diff",
            "#[test]\nfn same() { assert_eq!(d::adapt(i::imp(3)), r::refm(3)); }\n",
            "d = { path = \"../d\" }\n",
            "",
        );
        let _ = written(f.run());
        let first = std::fs::read(f.base.join("out/trust-dependencies.json")).unwrap();
        let _ = written(f.run());
        let second = std::fs::read(f.base.join("out/trust-dependencies.json")).unwrap();
        assert!(
            first == second,
            "two runs of one commit wrote different inventories"
        );
    }

    #[test]
    fn a_patch_in_the_workspace_manifest_is_refused() {
        // R8 remark 16: the catalog's workspace rules, adopted over the root manifest.
        let f = two_roots(
            "r8-16-patch",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[(
                "Cargo.toml",
                format!("{WS}\n[patch.crates-io]\nb = {{ path = \"crates/b\" }}\n"),
            )],
        );
        says(&refused(f.run()), "`[patch]` swaps a dependency's source");
    }

    #[test]
    fn a_site_two_roots_compile_is_counted_once() {
        // R8 remark 20: `refused-sites` counts sites, not compilations.
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let line = "pub const T: &str = include_str!(\"t.txt\");";
        let f = two_roots(
            "r8-20-count",
            "fn main() { let _ = common::T; }\n",
            "pub fn f() -> &'static str { common::T }\n",
            &[
                ("crates/common/Cargo.toml", manifest("common", "")),
                ("crates/common/src/lib.rs", format!("{line}\n")),
                ("crates/common/src/t.txt", "table".to_owned()),
                ("crates/a/Cargo.toml", manifest("a", dep)),
                ("crates/b/Cargo.toml", manifest("b", dep)),
                (
                    "trust/roots.eadl",
                    format!(
                        "{ROOTS}{}",
                        admit("crates/common/src/lib.rs", "include_str", line)
                    ),
                ),
            ],
        );
        assert_eq!(
            written(f.run()).get("refused-sites").and_then(Json::as_str),
            Some("1")
        );
    }

    // ── Round 9 (`M3.6.1` step 10) ───────────────────────────────────────────────────────────────────────────────

    #[test]
    fn an_incremental_profile_leaves_the_configuration_the_same_from_two_directories() {
        // R9 1: `-C incremental=` names the scratch build's own directory.
        let f = two_roots(
            "r9-1-incremental",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[(
                "Cargo.toml",
                format!("{WS}\n[profile.release]\nincremental = true\n"),
            )],
        );
        let one = configurations(&written(f.run_into("one")));
        let two = configurations(&written(f.run_into("elsewhere/two")));
        assert_eq!(one, two, "a unit's configuration names where it was built");
    }

    #[test]
    fn a_root_s_role_is_one_the_record_names() {
        // R9 2: a root declared `(role harness)` escaped the two-roles rule.
        let text = "(defroot gen (role generator) (package \"crates/a\") (target bin a))\n\
                    (defroot chk (role harness) (package \"crates/b\") (target lib))\n";
        let e = read_roots(text).expect_err("a root with the harness's role");
        assert!(e.contains("`chk`'s role `harness`"), "{e}");
    }

    #[test]
    fn an_extent_runs_past_a_brace_delimited_const_generic_argument() {
        // R9 3: the first brace group after an attribute was `{ 1 }` in `A<{ 1 }>`, not the item's body.
        let src = b"#[no_mangle]\npub extern \"C\" fn f() -> A<{ 1 }> { A }\n";
        let extent = site_extent(src, 1, 3).expect("an extent");
        assert_eq!(
            String::from_utf8_lossy(extent),
            "#[no_mangle]\npub extern \"C\" fn f() -> A<{ 1 }> { A }"
        );
    }

    #[test]
    fn a_role_package_that_names_no_package_is_refused() {
        // R9 remark 5: a misspelt role package turned the two-roles rule off for the package it meant.
        let roots = "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-packages \"crates/a\"))\n\
                     (defroot chk (role scheduling-checker) (package \"crates/b\") (target lib) (role-packages \"crates/b\" \"crates/cc\"))\n";
        let f = two_roots(
            "r9-5-role-package",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[("trust/roots.eadl", roots.to_owned())],
        );
        // Refused, and since `M3.6.3.3` as `trust-baseline-stale` rather than an error: the form names what the commit
        // does not hold.
        says(
            &refused(f.run()),
            "trust-baseline-stale: trust/roots.eadl: `chk` names `crates/cc`, which is no package of the commit",
        );
    }

    #[test]
    fn a_root_s_run_time_data_is_hashed_and_shared() {
        // R9 remark 6: §2's run-time data, declared by path in a root's form, read and hashed, and shared as a file.
        let roots = "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-packages \"crates/a\") (data \"docs/table.txt\"))\n\
                     (defroot chk (role scheduling-checker) (package \"crates/b\") (target lib) (role-packages \"crates/b\") (data \"docs/table.txt\"))\n";
        let f = two_roots(
            "r9-6-data",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[
                ("trust/roots.eadl", roots.to_owned()),
                ("docs/table.txt", "rows".to_owned()),
            ],
        );
        let inv = written(f.run());
        let data = program(&inv, "gen").get("data").expect("the root's data");
        assert_eq!(
            data.get("docs/table.txt").and_then(Json::as_str),
            Some(Digest::of(b"rows").hex().as_str())
        );
        assert!(items(&inv).iter().any(|(_, i)| kind(i) == "file"
            && i.get("file").and_then(Json::as_str) == Some("docs/table.txt")));
        f.commit(&[(
            "trust/roots.eadl",
            roots.replace("docs/table.txt", "docs/missing.txt"),
        )]);
        says(&refused(f.run()), "`docs/missing.txt`");
    }

    #[test]
    fn an_admitted_include_or_path_names_a_rs_file() {
        // R9 remark 7: an admitted `#[path]` onto a file that is not `.rs` compiled it and left it untokenised.
        let txt = "#[path = \"m.txt\"]\nmod m;";
        let f = two_roots(
            "r9-7-path-txt",
            "fn main() {}\n",
            &format!("{txt}\npub use m::*;\n"),
            &[
                ("crates/b/src/m.txt", "pub fn m() {}\n".to_owned()),
                (
                    "trust/roots.eadl",
                    format!("{ROOTS}{}", admit("crates/b/src/lib.rs", "path", txt)),
                ),
            ],
        );
        says(
            &refused(f.run()),
            "names `m.txt`, which is not a `.rs` file",
        );
        let rs = "#[path = \"inner.rs\"]\nmod m;";
        let f = two_roots(
            "r9-7-path-rs",
            "fn main() {}\n",
            &format!("{rs}\npub use m::*;\n"),
            &[
                ("crates/b/src/inner.rs", "pub fn m() {}\n".to_owned()),
                (
                    "trust/roots.eadl",
                    format!("{ROOTS}{}", admit("crates/b/src/lib.rs", "path", rs)),
                ),
            ],
        );
        assert_eq!(
            written(f.run()).get("refused-sites").and_then(Json::as_str),
            Some("1")
        );
    }

    // ── Round 10 (`M3.6.1` step 11) ──────────────────────────────────────────────────────────────────────────────

    #[test]
    fn an_alias_or_a_wrapper_of_a_refused_construct_is_inadmissible() {
        // R10 1: admitting the `use … as` or the `macro_rules!` freed every invocation of it, which no rule names.
        let renamed_include = "use std::include as inc;";
        let renamed_asm = "use core::arch::global_asm as g;";
        let wrapper = "macro_rules! put {\n    ($s:expr) => {\n        core::arch::global_asm!($s);\n    };\n}";
        for (name, src, extra, admitted) in [
            (
                "r10-1-renamed-include",
                format!("{renamed_include}\ninc!(\"m.txt\");\n"),
                vec![("crates/b/src/m.txt", "pub fn f() {}\n".to_owned())],
                admit("crates/b/src/lib.rs", "include", renamed_include),
            ),
            (
                "r10-1-renamed-asm",
                format!("{renamed_asm}\ng!(\".data\");\npub fn f() {{}}\n"),
                vec![],
                admit("crates/b/src/lib.rs", "global_asm", renamed_asm),
            ),
            (
                "r10-1-wrapper",
                format!("{wrapper}\nput!(\".data\");\npub fn f() {{}}\n"),
                vec![],
                format!(
                    "{}{}",
                    admit("crates/b/src/lib.rs", "macro_rules", wrapper),
                    admit(
                        "crates/b/src/lib.rs",
                        "global_asm",
                        "core::arch::global_asm!($s);"
                    )
                ),
            ),
        ] {
            let mut files = extra;
            files.push(("trust/roots.eadl", format!("{ROOTS}{admitted}")));
            let f = two_roots(name, "fn main() {}\n", &src, &files);
            says(
                &refused(f.run()),
                "so every invocation of it would go unpinned",
            );
        }
    }
}
