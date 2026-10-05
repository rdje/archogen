//! §3's builds (leaf `M2.7.4.3`): the judged tree written from blobs, every cargo configuration on a build's path
//! listed and held before any cargo command runs, `cargo metadata` under the allowlist with its graph checked against
//! the crate's reading of the manifests, every package in every source set built across the matrix, and the
//! compiler's dependency information read (`docs/specs/catalog/decision_catalog-records-hashes.md`, §3).
//!
//! > The compiler, not a scan, decides completeness.
//!
//! Every refusal here is `catalog-source`, naming the record and the facet whose set the package is in. What the
//! crate already refuses at load — the manifest dialect, the token rules, a non-path dependency reached by its
//! reading — is not repeated; what needs cargo or the compiler is here.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;

use archogen_catalog::grammar;
use archogen_catalog::load::Loaded;
use archogen_catalog::manifest::{self, Value};
use archogen_catalog::package;
use archogen_catalog::record::{Content, FacetKind, Targets};
use archogen_catalog::tree::Tree;
use archogen_catalog::{Code, Refusal};

use crate::catalog_check::Failure;
use crate::json::{self, Json};

/// What the builds did.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Built {
    /// Packages in some source set.
    pub packages: usize,
    /// `cargo build` runs.
    pub builds: usize,
    /// Dependency-information files read.
    pub units: usize,
}

/// The names a file system that folds case could make cargo or rustup read under another spelling (§3).
const SPECIAL: [&str; 7] = [
    ".cargo/config",
    ".cargo/config.toml",
    "rust-toolchain",
    "rust-toolchain.toml",
    "Cargo.toml",
    "Cargo.lock",
    "build.rs",
];

/// For each facet with packages in its own set: those packages' directories, and every package's it reads (§3).
type FacetPackages = BTreeMap<(String, FacetKind), (BTreeSet<String>, BTreeSet<String>)>;

/// The environment variables a build may depend on (§3); the trust instrument adopts the same list
/// (`docs/specs/trust/decision_trust-inventory.md` §3, R8 remark 22).
pub(crate) const ENV_ALLOWED: [&str; 3] = [
    "CARGO_CRATE_NAME",
    "CARGO_MANIFEST_DIR",
    "CARGO_MANIFEST_PATH",
];

fn source(path: &str, field: &str, message: impl Into<String>) -> Failure {
    Refusal::new(Code::Source, path, field, None, message).into()
}

/// Write `tree` file by file into `dir`, which is emptied first (§3): every path in §4's grammar, no two equal but
/// for ASCII case, and none spelling a special name in another case.
///
/// # Errors
///
/// `catalog-source` for a path the rules refuse; a write error cannot judge.
pub fn write_tree(tree: &Tree, dir: &Path) -> Result<(), Failure> {
    let mut lowered: BTreeMap<String, &str> = BTreeMap::new();
    for path in tree.paths() {
        if !grammar::is_path(path) {
            return Err(source(
                path,
                "(file)",
                "a tracked path outside §4's grammar",
            ));
        }
        if let Some(other) = lowered.insert(path.to_ascii_lowercase(), path) {
            return Err(source(
                path,
                "(file)",
                format!("`{other}` and `{path}` differ only in ASCII case, which a case-insensitive file system cannot hold"),
            ));
        }
        for special in SPECIAL {
            let matches = path.eq_ignore_ascii_case(special)
                || path.rsplit_once('/').is_some_and(|(_, name)| {
                    name.eq_ignore_ascii_case(special.rsplit('/').next().unwrap_or(special))
                }) && path
                    .to_ascii_lowercase()
                    .ends_with(&special.to_ascii_lowercase());
            let exact = path == special || path.ends_with(&format!("/{special}"));
            if matches && !exact {
                return Err(source(
                    path,
                    "(file)",
                    format!("spells `{special}` in another case, which a case-insensitive file system would read as it"),
                ));
            }
        }
    }
    let _ = fs::remove_dir_all(dir);
    for path in tree.paths() {
        let full = dir.join(path);
        if let Some(parent) = full.parent() {
            fs::create_dir_all(parent).map_err(|e| format!("{}: {e}", parent.display()))?;
        }
        fs::write(&full, tree.get(path).unwrap_or_default())
            .map_err(|e| format!("{}: {e}", full.display()))?;
    }
    Ok(())
}

/// The toolchain file's form (§3): one `[toolchain]` table whose keys are `channel`, required, and any of
/// `components`, `targets` and `profile`; the channel a release number. Returns the channel.
///
/// # Errors
///
/// Why the file is outside the form.
pub fn toolchain_file(tree: &Tree) -> Result<String, String> {
    let bytes = tree
        .get("rust-toolchain.toml")
        .ok_or("the tree holds no `rust-toolchain.toml`, so there is no pin (§3)")?;
    let text = String::from_utf8_lossy(bytes);
    let m = manifest::parse(&text)
        .map_err(|o| format!("`rust-toolchain.toml` line {}: {}", o.line, o.why))?;
    if m.tables != vec![vec!["toolchain".to_owned()]] {
        return Err(
            "`rust-toolchain.toml` holds a table other than `[toolchain]`, or none (§3)".to_owned(),
        );
    }
    let mut channel = None;
    for (path, value) in &m.values {
        match path.as_slice() {
            [t, key] if t == "toolchain" => match key.as_str() {
                "channel" => match value {
                    Value::Str(c) => channel = Some(c.clone()),
                    _ => return Err("`channel` is not a string".to_owned()),
                },
                "components" | "targets" | "profile" => {}
                other => {
                    return Err(format!(
                        "`rust-toolchain.toml` holds `toolchain.{other}`, a key §3 does not admit"
                    ))
                }
            },
            _ => {
                return Err(format!(
                    "`rust-toolchain.toml` holds `{}`, outside `[toolchain]`",
                    path.join(".")
                ))
            }
        }
    }
    let channel = channel.ok_or("`rust-toolchain.toml` names no channel")?;
    let release = channel.split('.').count() == 3
        && channel.split('.').all(|n| {
            !n.is_empty()
                && n.bytes().all(|b| b.is_ascii_digit())
                && (n == "0" || !n.starts_with('0'))
        });
    if !release {
        return Err(format!(
            "the pin `{channel}` is not a release number `MAJOR.MINOR.PATCH` (§3)"
        ));
    }
    Ok(channel)
}

/// §3's allowlisted environment for cargo and rustc: `PATH`, `HOME`, `RUSTUP_HOME`, `RUSTUP_TOOLCHAIN` the pin,
/// `CARGO_HOME` an empty directory of the gate's own, `CARGO_TARGET_DIR` its own, and nothing else.
///
/// # Errors
///
/// `CARGO_HOME` could not be made.
pub fn environment(
    pin: &str,
    cargo_home: &Path,
    target_dir: &Path,
) -> Result<Vec<(String, String)>, String> {
    fs::create_dir_all(cargo_home).map_err(|e| format!("{}: {e}", cargo_home.display()))?;
    fs::create_dir_all(target_dir).map_err(|e| format!("{}: {e}", target_dir.display()))?;
    let mut env = Vec::new();
    for key in ["PATH", "HOME"] {
        let value = std::env::var(key).map_err(|_| format!("`{key}` is unset"))?;
        env.push((key.to_owned(), value));
    }
    let rustup_home = std::env::var("RUSTUP_HOME").unwrap_or_else(|_| {
        let home = std::env::var("HOME").unwrap_or_default();
        format!("{home}/.rustup")
    });
    env.push(("RUSTUP_HOME".to_owned(), rustup_home));
    env.push(("RUSTUP_TOOLCHAIN".to_owned(), pin.to_owned()));
    env.push(("CARGO_HOME".to_owned(), cargo_home.display().to_string()));
    env.push((
        "CARGO_TARGET_DIR".to_owned(),
        target_dir.display().to_string(),
    ));
    Ok(env)
}

fn command(program: &str, args: &[&str], env: &[(String, String)], cwd: &Path) -> Command {
    let mut c = Command::new(program);
    c.args(args).current_dir(cwd).env_clear();
    for (k, v) in env {
        c.env(k, v);
    }
    c
}

/// Every cargo configuration on the directory path of a command run in `cwd`, held before it runs (§3): one inside
/// the written tree at `root` is a tracked file held to the content rules; one outside is admitted only as the
/// tracked copy of the judged tree at `repo`, byte for byte, and held to the same rules; `target/.cargo` and
/// anything above the repository are refused.
///
/// # Errors
///
/// `catalog-source`, naming the configuration.
pub fn configurations_on_path(
    cwd: &Path,
    root: &Path,
    repo: &Path,
    tree: &Tree,
) -> Result<(), Failure> {
    for dir in cwd.ancestors() {
        for name in ["config", "config.toml"] {
            let config = dir.join(".cargo").join(name);
            let Ok(meta) = fs::symlink_metadata(&config) else {
                continue;
            };
            let shown = config.display().to_string();
            if meta.file_type().is_symlink() {
                return Err(source(
                    &shown,
                    "(config)",
                    "a cargo configuration that is a symbolic link",
                ));
            }
            let relative = |base: &Path| {
                config.strip_prefix(base).ok().map(|r| {
                    r.components()
                        .map(|c| c.as_os_str().to_string_lossy().into_owned())
                        .collect::<Vec<_>>()
                        .join("/")
                })
            };
            let on_disk = fs::read(&config).map_err(|e| format!("{shown}: {e}"))?;
            let tracked = if let Some(rel) = relative(root) {
                rel
            } else if let Some(rel) = relative(repo) {
                if rel.starts_with("target/") {
                    return Err(source(
                        &shown,
                        "(config)",
                        "a cargo configuration under `target/`, outside the written index (§3)",
                    ));
                }
                rel
            } else {
                return Err(source(
                    &shown,
                    "(config)",
                    "a cargo configuration above the repository, on a build's path (§3)",
                ));
            };
            let Some(blob) = tree.get(&tracked) else {
                return Err(source(&shown, "(config)", format!("a cargo configuration on a build's path that the judged tree does not track as `{tracked}` (§3)")));
            };
            if blob != on_disk.as_slice() {
                return Err(source(&shown, "(config)", format!("a cargo configuration whose bytes differ from the judged tree's `{tracked}` (§3)")));
            }
            let text = String::from_utf8_lossy(blob);
            let m = manifest::parse(&text)
                .map_err(|o| source(&shown, "(config)", format!("line {}: {}", o.line, o.why)))?;
            package::check_config(&tracked, &m).map_err(|why| source(&shown, "(config)", why))?;
        }
    }
    Ok(())
}

/// A package in some source set: where it is, its name, which facets' sets hold it, whether some implementation's
/// own set does, and the Rust targets it is built for besides the host.
#[derive(Debug, Clone)]
struct Package {
    dir: String,
    name: String,
    /// The facets whose own or reached set holds it, as `(record, facet)`.
    facets: Vec<(String, FacetKind)>,
    implementation: bool,
    rust_targets: BTreeSet<String>,
}

/// The package directories among `paths`: each directory whose `Cargo.toml` is listed and holds `[package]`.
fn package_dirs(tree: &Tree, paths: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    for path in paths {
        let dir = match path.strip_suffix("/Cargo.toml") {
            Some(d) => d.to_owned(),
            None if path == "Cargo.toml" => String::new(),
            None => continue,
        };
        let text = String::from_utf8_lossy(tree.get(path).unwrap_or_default());
        if manifest::parse(&text).is_ok_and(|m| m.has_table(&["package"])) {
            out.push(dir);
        }
    }
    out
}

fn package_name(tree: &Tree, dir: &str) -> Result<String, String> {
    let manifest_path = if dir.is_empty() {
        "Cargo.toml".to_owned()
    } else {
        format!("{dir}/Cargo.toml")
    };
    let text = String::from_utf8_lossy(tree.get(&manifest_path).unwrap_or_default());
    let m = manifest::parse(&text)
        .map_err(|o| format!("`{manifest_path}` line {}: {}", o.line, o.why))?;
    match m.get(&["package", "name"]) {
        Some(Value::Str(name)) => Ok(name.clone()),
        _ => Err(format!("`{manifest_path}` names no package")),
    }
}

/// `RUST_TARGET` of `targets/<target>.env`.
fn rust_target(tree: &Tree, target: &str) -> Option<String> {
    let text = String::from_utf8_lossy(tree.get(&format!("targets/{target}.env"))?).into_owned();
    text.lines()
        .find_map(|l| l.strip_prefix("RUST_TARGET="))
        .map(str::to_owned)
}

/// Every package in every source set of `loaded`'s records, with its matrix (§3).
fn packages(loaded: &Loaded, tree: &Tree) -> Result<Vec<Package>, Failure> {
    let named = loaded.catalog.named_targets()?;
    let mut by_dir: BTreeMap<String, Package> = BTreeMap::new();
    for (id, record) in &loaded.catalog.records {
        let record_targets: BTreeSet<String> = match &record.contract.targets {
            Targets::Any => named.iter().cloned().collect(),
            Targets::Named(list) => list.iter().cloned().collect(),
        };
        let mut cost_targets: BTreeSet<String> = BTreeSet::new();
        if let Content::Present(m) = &record.timing_model.content {
            cost_targets.extend(m.costs.iter().map(|c| c.target.clone()));
        }
        let rust_targets: BTreeSet<String> = record_targets
            .iter()
            .chain(&cost_targets)
            .filter_map(|t| rust_target(tree, t))
            .collect();
        for facet in FacetKind::ALL {
            let Some(h) = loaded.hashes.facet(id, facet) else {
                continue;
            };
            let own = package_dirs(tree, &h.own_set);
            let reached = package_dirs(tree, &h.reached);
            for (dir, is_own) in own
                .iter()
                .map(|d| (d, true))
                .chain(reached.iter().map(|d| (d, false)))
            {
                let entry = by_dir.entry(dir.clone()).or_insert_with(|| Package {
                    dir: dir.clone(),
                    name: String::new(),
                    facets: Vec::new(),
                    implementation: false,
                    rust_targets: BTreeSet::new(),
                });
                entry.facets.push((id.clone(), facet));
                if facet == FacetKind::Implementation && is_own {
                    entry.implementation = true;
                    entry.rust_targets.extend(rust_targets.iter().cloned());
                }
            }
        }
    }
    let mut out = Vec::new();
    for (dir, mut p) in by_dir {
        p.name = package_name(tree, &dir)?;
        out.push(p);
    }
    Ok(out)
}

/// `cargo metadata --offline --locked --format-version 1` in `root`, and §3's checks of it: the workspace root is
/// the written tree's, every member's dependency is a path one, and the graph of normal and build dependencies of
/// each package in a source set holds no procedural macro or build script, no package from a source, and exactly
/// the packages §3's reading reached.
fn metadata(
    root: &Path,
    env: &[(String, String)],
    facets: &FacetPackages,
) -> Result<Json, Failure> {
    let output = command(
        "cargo",
        &["metadata", "--offline", "--locked", "--format-version", "1"],
        env,
        root,
    )
    .output()
    .map_err(|e| format!("`cargo metadata` did not run: {e}"))?;
    if !output.status.success() {
        return Err(source(
            "Cargo.toml",
            "(workspace)",
            format!(
                "`cargo metadata --offline --locked` failed: {}",
                String::from_utf8_lossy(&output.stderr).trim()
            ),
        ));
    }
    let doc = json::parse(&String::from_utf8_lossy(&output.stdout))
        .map_err(|e| format!("`cargo metadata`: {e}"))?;
    let reported = doc
        .get("workspace_root")
        .and_then(Json::as_str)
        .unwrap_or_default();
    let canonical = |p: &Path| fs::canonicalize(p).unwrap_or_else(|_| p.to_path_buf());
    if canonical(Path::new(reported)) != canonical(root) {
        return Err(source(
            "Cargo.toml",
            "(workspace)",
            format!("`cargo metadata` reports the workspace root `{reported}`, not the written index's (§3)"),
        ));
    }
    let root_prefix = format!("{}/", canonical(root).display());
    let dir_of = |manifest_path: &str| -> Option<String> {
        let rel = manifest_path.strip_prefix(&root_prefix)?;
        Some(rel.strip_suffix("/Cargo.toml").unwrap_or("").to_owned())
    };
    let members: BTreeSet<&str> = doc
        .get("workspace_members")
        .map(Json::elements)
        .unwrap_or_default()
        .iter()
        .filter_map(Json::as_str)
        .collect();
    let mut by_id: BTreeMap<&str, &Json> = BTreeMap::new();
    for p in doc.get("packages").map(Json::elements).unwrap_or_default() {
        let id = p.get("id").and_then(Json::as_str).unwrap_or_default();
        by_id.insert(id, p);
        let manifest_path = p
            .get("manifest_path")
            .and_then(Json::as_str)
            .unwrap_or_default();
        let shown = dir_of(manifest_path)
            .map_or_else(|| manifest_path.to_owned(), |d| format!("{d}/Cargo.toml"));
        if members.contains(id) {
            for d in p
                .get("dependencies")
                .map(Json::elements)
                .unwrap_or_default()
            {
                let path_dep = d.get("path").is_some_and(|v| !v.is_null())
                    && d.get("source").is_none_or(Json::is_null);
                if !path_dep {
                    let name = d.get("name").and_then(Json::as_str).unwrap_or_default();
                    return Err(source(
                        &shown,
                        "(dependencies)",
                        format!("`{name}` is not a path dependency (§3)"),
                    ));
                }
            }
        }
    }
    let nodes: BTreeMap<&str, &Json> = doc
        .get("resolve")
        .and_then(|r| r.get("nodes"))
        .map(Json::elements)
        .unwrap_or_default()
        .iter()
        .map(|n| (n.get("id").and_then(Json::as_str).unwrap_or_default(), n))
        .collect();
    let id_of_dir: BTreeMap<String, &str> = by_id
        .iter()
        .filter_map(|(id, p)| {
            dir_of(p.get("manifest_path").and_then(Json::as_str)?).map(|d| (d, *id))
        })
        .collect();
    for ((record, facet), (own, all)) in facets {
        let field = format!("{} sources", facet.as_str());
        let mut closure: BTreeSet<&str> = BTreeSet::new();
        let mut stack: Vec<&str> = Vec::new();
        for dir in own {
            let Some(start) = id_of_dir.get(dir) else {
                return Err(source(
                    record,
                    &field,
                    format!("`cargo metadata` does not hold the package `{dir}` (§3)"),
                ));
            };
            stack.push(start);
        }
        while let Some(id) = stack.pop() {
            if !closure.insert(id) {
                continue;
            }
            let Some(node) = nodes.get(id) else { continue };
            for dep in node.get("deps").map(Json::elements).unwrap_or_default() {
                let normal_or_build = dep
                    .get("dep_kinds")
                    .map(Json::elements)
                    .unwrap_or_default()
                    .iter()
                    .any(|k| {
                        k.get("kind")
                            .is_none_or(|kind| kind.is_null() || kind.as_str() == Some("build"))
                    });
                if normal_or_build {
                    stack.push(dep.get("pkg").and_then(Json::as_str).unwrap_or_default());
                }
            }
        }
        let mut graph_dirs: BTreeSet<String> = BTreeSet::new();
        for id in &closure {
            let Some(p) = by_id.get(id) else { continue };
            let manifest_path = p
                .get("manifest_path")
                .and_then(Json::as_str)
                .unwrap_or_default();
            let shown = dir_of(manifest_path)
                .map_or_else(|| manifest_path.to_owned(), |d| format!("{d}/Cargo.toml"));
            if !p.get("source").is_none_or(Json::is_null) {
                return Err(source(
                    &shown,
                    "(package)",
                    "a dependency from a source, not a path (§3)",
                ));
            }
            for t in p.get("targets").map(Json::elements).unwrap_or_default() {
                for kind in t.get("kind").map(Json::elements).unwrap_or_default() {
                    if matches!(kind.as_str(), Some("proc-macro" | "custom-build")) {
                        return Err(source(
                            &shown,
                            "(package)",
                            format!(
                                "a `{}` target in the graph of `{record}`'s {} (§3)",
                                kind.as_str().unwrap_or_default(),
                                facet.as_str()
                            ),
                        ));
                    }
                }
            }
            if let Some(d) = dir_of(manifest_path) {
                graph_dirs.insert(d);
            }
        }
        if &graph_dirs != all {
            let only_graph: Vec<&String> = graph_dirs.difference(all).collect();
            let only_read: Vec<&String> = all.difference(&graph_dirs).collect();
            return Err(source(
                record,
                &field,
                format!("cargo's graph and §3's reading of the manifests disagree: in the graph only {only_graph:?}, in the reading only {only_read:?} (§3)"),
            ));
        }
    }
    Ok(doc)
}

/// One `.d` file: the prerequisites of its first target, and its environment dependencies.
fn dep_info(text: &str) -> (Vec<String>, Vec<String>) {
    let mut prerequisites = Vec::new();
    let mut env = Vec::new();
    for line in text.lines() {
        if let Some(dep) = line.strip_prefix("# env-dep:") {
            env.push(dep.split('=').next().unwrap_or_default().to_owned());
        } else if prerequisites.is_empty() {
            if let Some((_, rest)) = line.split_once(": ") {
                prerequisites.extend(
                    rest.split(' ')
                        .filter(|p| !p.is_empty())
                        .map(|p| p.replace("\\ ", " ")),
                );
            }
        }
    }
    (prerequisites, env)
}

/// `relative` resolved lexically against `base`, both relative to the written root: `None` when it leaves it.
fn resolve(base: &str, relative: &str) -> Option<String> {
    let mut parts: Vec<&str> = if base.is_empty() {
        Vec::new()
    } else {
        base.split('/').collect()
    };
    for segment in relative.split('/') {
        match segment {
            "" | "." => {}
            ".." => {
                parts.pop()?;
            }
            s => parts.push(s),
        }
    }
    Some(parts.join("/"))
}

/// One prerequisite the compiler read for the unit `crate_name` of the package at `unit_dir` (§3): resolved
/// lexically against the written root — cargo runs `rustc` from the workspace root and writes the paths relative to
/// it — inside it and in §4's grammar; in some facet's set, `sets` naming each
/// file's holders; and its bytes after the build its blob's.
fn judge_prerequisite(
    unit_dir: &str,
    crate_name: &str,
    prerequisite: &str,
    root: &Path,
    sets: &BTreeMap<String, BTreeSet<String>>,
    tree: &Tree,
) -> Result<(), Failure> {
    let resolved = if prerequisite.starts_with('/') {
        Path::new(prerequisite)
            .strip_prefix(root)
            .ok()
            .map(|r| r.display().to_string())
    } else {
        resolve("", prerequisite)
    };
    let manifest = format!("{unit_dir}/Cargo.toml");
    let Some(rel) = resolved.filter(|r| grammar::is_path(r)) else {
        return Err(source(
            &manifest,
            "(build)",
            format!("`{crate_name}` read `{prerequisite}`, outside the written index (§3)"),
        ));
    };
    if !sets.contains_key(&rel) {
        return Err(source(
            &manifest,
            "(build)",
            format!("`{crate_name}` read `{rel}`, a file in no source set (§3)"),
        ));
    }
    let after = fs::read(root.join(&rel)).map_err(|e| format!("{rel}: {e}"))?;
    if Some(after.as_slice()) != tree.get(&rel) {
        return Err(source(
            &rel,
            "(build)",
            "its bytes after the build differ from its blob's (§3)",
        ));
    }
    Ok(())
}

/// Run §3's builds for `loaded` over the tree written at `root`, the repository at `repo`, with the gate's scratch
/// directories under `scratch`.
///
/// # Errors
///
/// `catalog-source` for what the builds refuse; a run that could not judge otherwise.
pub fn check(
    loaded: &Loaded,
    tree: &Tree,
    root: &Path,
    repo: &Path,
    pin: &str,
    scratch: &Path,
) -> Result<Built, Failure> {
    let packages = packages(loaded, tree)?;
    let mut built = Built {
        packages: packages.len(),
        ..Built::default()
    };
    if packages.is_empty() {
        return Ok(built);
    }
    let cargo_home = scratch.join("cargo-home");
    let target_dir = scratch.join("records-target");
    let _ = fs::remove_dir_all(&target_dir);
    let env = environment(pin, &cargo_home, &target_dir)?;
    configurations_on_path(root, root, repo, tree)?;
    // For each facet, the packages its own set holds and all it reads, for cargo's graph to agree with (§3).
    let mut facets: BTreeMap<(String, FacetKind), (BTreeSet<String>, BTreeSet<String>)> =
        BTreeMap::new();
    for ((id, facet), h) in &loaded.hashes.facets {
        let own: BTreeSet<String> = package_dirs(tree, &h.own_set).into_iter().collect();
        if own.is_empty() {
            continue;
        }
        let mut all = own.clone();
        all.extend(package_dirs(tree, &h.reached));
        facets.insert((id.clone(), *facet), (own, all));
    }
    metadata(root, &env, &facets)?;
    // Which (record, facet) sets hold each file, for the dependency information's check.
    let mut sets: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for ((id, facet), h) in &loaded.hashes.facets {
        for path in h.own_set.iter().chain(&h.reached) {
            sets.entry(path.clone())
                .or_default()
                .insert(format!("{id} {}", facet.as_str()));
        }
    }
    let mut unit_dirs: BTreeMap<String, String> = BTreeMap::new();
    for package in &packages {
        let cwd = if package.dir.is_empty() {
            root.to_path_buf()
        } else {
            root.join(&package.dir)
        };
        configurations_on_path(&cwd, root, repo, tree)?;
        unit_dirs.insert(package.name.replace('-', "_"), package.dir.clone());
        let mut matrix: Vec<(bool, Option<String>)> = vec![(false, None), (true, None)];
        if package.implementation {
            for t in &package.rust_targets {
                matrix.push((false, Some(t.clone())));
                matrix.push((true, Some(t.clone())));
            }
        }
        for (release, target) in matrix {
            let mut args = vec![
                "build",
                "--offline",
                "--locked",
                "-p",
                package.name.as_str(),
            ];
            if release {
                args.push("--release");
            }
            if let Some(t) = &target {
                args.push("--target");
                args.push(t);
            }
            let output = command("cargo", &args, &env, &cwd)
                .output()
                .map_err(|e| format!("`cargo build` did not run: {e}"))?;
            built.builds += 1;
            if !output.status.success() {
                return Err(source(
                    &format!("{}/Cargo.toml", package.dir),
                    "(build)",
                    format!(
                        "`{}` does not build{}{}: {}",
                        package.name,
                        if release { " in release" } else { " in dev" },
                        target
                            .as_deref()
                            .map_or(String::new(), |t| format!(" for {t}")),
                        first_error(&String::from_utf8_lossy(&output.stderr))
                    ),
                ));
            }
        }
    }
    // Every unit the builds compiled: each `.d` under a `deps/` directory of the target directory.
    let mut stack = vec![target_dir.clone()];
    while let Some(dir) = stack.pop() {
        for entry in fs::read_dir(&dir)
            .map_err(|e| format!("{}: {e}", dir.display()))?
            .filter_map(Result::ok)
        {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
                continue;
            }
            let is_dep_info = path.extension().is_some_and(|e| e == "d")
                && path
                    .parent()
                    .and_then(Path::file_name)
                    .is_some_and(|p| p == "deps");
            if !is_dep_info {
                continue;
            }
            built.units += 1;
            let stem = path
                .file_stem()
                .and_then(|s| s.to_str())
                .unwrap_or_default();
            let crate_name = stem.rsplit_once('-').map_or(stem, |(c, _)| c);
            let Some(unit_dir) = unit_dirs.get(crate_name) else {
                return Err(source(
                    &path.display().to_string(),
                    "(unit)",
                    format!("the build compiled `{crate_name}`, a unit no package in a source set names (§3)"),
                ));
            };
            let text = fs::read_to_string(&path).map_err(|e| format!("{}: {e}", path.display()))?;
            let (prerequisites, env_deps) = dep_info(&text);
            for name in env_deps {
                if !(name.starts_with("CARGO_PKG_") || ENV_ALLOWED.contains(&name.as_str())) {
                    return Err(source(
                        &format!("{unit_dir}/Cargo.toml"),
                        "(build)",
                        format!("`{crate_name}` depends on the environment variable `{name}` (§3)"),
                    ));
                }
            }
            for prerequisite in prerequisites {
                judge_prerequisite(unit_dir, crate_name, &prerequisite, root, &sets, tree)?;
            }
        }
    }
    Ok(built)
}

/// The first line of a cargo or rustc error that says what went wrong, rather than the closing summary.
fn first_error(stderr: &str) -> String {
    stderr
        .lines()
        .find(|l| l.starts_with("error") && !l.starts_with("error: could not compile"))
        .or_else(|| stderr.lines().rev().find(|l| !l.trim().is_empty()))
        .unwrap_or_default()
        .to_owned()
}

/// Whether commit `tree` changes, against `parent`, anything §3 says is built for: a file under `catalog/`, a file
/// in some facet's set, a manifest, `Cargo.lock`, a cargo configuration, a toolchain file or a target file.
fn changes_what_is_built(tree: &Tree, parent: &Tree, sets: &BTreeSet<String>) -> bool {
    let relevant = |path: &str| {
        path.starts_with("catalog/")
            || path.starts_with("targets/")
            || sets.contains(path)
            || path == "Cargo.toml"
            || path.ends_with("/Cargo.toml")
            || path == "Cargo.lock"
            || path.ends_with("/Cargo.lock")
            || path == "rust-toolchain"
            || path == "rust-toolchain.toml"
            || path.ends_with("/rust-toolchain")
            || path.ends_with("/rust-toolchain.toml")
            || path.contains(".cargo/config")
    };
    tree.paths()
        .filter(|p| relevant(p))
        .any(|p| parent.get(p) != tree.get(p))
        || parent
            .paths()
            .filter(|p| relevant(p))
            .any(|p| tree.get(p).is_none())
}

/// §3's builds for every commit CI replays between `bases` and `head`, `head` itself excepted, that changes what is
/// built against each of its parents: each such commit's catalog loaded, its tree written under `scratch`, and
/// [`check`] run on it (§3, "Who runs these checks").
///
/// # Errors
///
/// As [`check`], the commit named in the refusal's path.
pub fn check_replayed(
    history: &archogen_catalog::history::History,
    bases: &[&str],
    head: &str,
    repo: &Path,
    pin: &str,
    scratch: &Path,
) -> Result<Vec<(String, Built)>, Failure> {
    let mut out = Vec::new();
    for name in history.between(bases, head)? {
        if name == head {
            continue;
        }
        let commit = history.get(&name)?;
        if !commit.tree.is_dir("catalog") {
            continue;
        }
        let loaded = archogen_catalog::load::load(history, &name)?;
        let mut sets: BTreeSet<String> = BTreeSet::new();
        for h in loaded.hashes.facets.values() {
            sets.extend(h.own_set.iter().cloned());
            sets.extend(h.reached.iter().cloned());
        }
        let changes = commit.parents.is_empty()
            || commit.parents.iter().any(|p| {
                history
                    .get(p)
                    .is_ok_and(|parent| changes_what_is_built(&commit.tree, &parent.tree, &sets))
            });
        if !changes {
            continue;
        }
        let root = scratch.join("replay").join(&name).join("tree");
        write_tree(&commit.tree, &root)?;
        let built = check(
            &loaded,
            &commit.tree,
            &root,
            repo,
            pin,
            &scratch.join("replay").join(&name),
        )
        .map_err(|f| match f {
            Failure::Refused(r) => Failure::Refused(Refusal::new(
                r.code,
                &format!("{name}:{}", r.path),
                &r.field,
                r.at,
                format!("at the replayed commit {name}: {}", r.message),
            )),
            other => other,
        })?;
        out.push((name, built));
    }
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::{dep_info, resolve, toolchain_file, write_tree};
    use archogen_catalog::tree::Tree;

    #[test]
    fn dependency_information_reads_prerequisites_and_environment() {
        let text =
            "a.d: src/lib.rs src/a.rs\n\nsrc/lib.rs:\n# env-dep:CARGO_PKG_NAME=x\n# env-dep:FOO\n";
        let (p, e) = dep_info(text);
        assert_eq!(p, ["src/lib.rs", "src/a.rs"]);
        assert_eq!(e, ["CARGO_PKG_NAME", "FOO"]);
    }

    #[test]
    fn a_path_resolves_lexically_and_cannot_leave_the_root() {
        assert_eq!(
            resolve("crates/p", "src/lib.rs").as_deref(),
            Some("crates/p/src/lib.rs")
        );
        assert_eq!(
            resolve("crates/p", "../q/src/lib.rs").as_deref(),
            Some("crates/q/src/lib.rs")
        );
        assert_eq!(resolve("crates/p", "../../../etc/passwd"), None);
        assert_eq!(resolve("", "./Cargo.toml").as_deref(), Some("Cargo.toml"));
    }

    #[test]
    fn the_toolchain_file_is_held_to_its_form() {
        let tree =
            |text: &str| Tree::new([("rust-toolchain.toml".to_owned(), text.as_bytes().to_vec())]);
        assert_eq!(
            toolchain_file(&tree(
                "[toolchain]\nchannel = \"1.95.0\"\ncomponents = [\"clippy\"]\n"
            ))
            .unwrap(),
            "1.95.0"
        );
        assert!(toolchain_file(&tree("[toolchain]\nchannel = \"stable\"\n"))
            .unwrap_err()
            .contains("release number"));
        assert!(
            toolchain_file(&tree("[toolchain]\nchannel = \"1.95.0\"\npath = \"/x\"\n"))
                .unwrap_err()
                .contains("`toolchain.path`")
        );
        assert!(
            toolchain_file(&tree("[toolchain]\nchannel = \"1.95.0\"\n[other]\nx = 1\n"))
                .unwrap_err()
                .contains("other than")
        );
        assert!(toolchain_file(&Tree::new([]))
            .unwrap_err()
            .contains("no pin"));
    }

    #[test]
    fn a_written_tree_refuses_case_twins_and_misspelled_names() {
        let dir = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("target/catalog-build-tests/write");
        let ok = Tree::new([
            ("a/Cargo.toml".to_owned(), b"x".to_vec()),
            ("a/src/lib.rs".to_owned(), b"y".to_vec()),
        ]);
        write_tree(&ok, &dir).unwrap();
        assert_eq!(std::fs::read(dir.join("a/src/lib.rs")).unwrap(), b"y");
        let twins = Tree::new([("a/x.rs".to_owned(), vec![]), ("a/X.rs".to_owned(), vec![])]);
        assert!(matches!(
            write_tree(&twins, &dir),
            Err(super::Failure::Refused(_))
        ));
        let spelled = Tree::new([("a/cargo.toml".to_owned(), vec![])]);
        assert!(matches!(
            write_tree(&spelled, &dir),
            Err(super::Failure::Refused(_))
        ));
        let _ = std::fs::remove_dir_all(dir);
    }

    use std::collections::BTreeMap;
    use std::path::PathBuf;
    use std::process::Command;

    use archogen_catalog::history::{Commit, CommitterDate, History};
    use archogen_catalog::load::{load, Loaded};
    use archogen_catalog::Code;

    use archogen_catalog::hash::Catalog;
    use archogen_catalog::lock::{blessed, Lock, PATH};

    use super::{check, configurations_on_path, judge_prerequisite};
    use crate::catalog_check::Failure;

    fn real_root() -> PathBuf {
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
    }

    fn scratch(name: &str) -> PathBuf {
        let dir = real_root().join("target/catalog-build-tests").join(name);
        let _ = std::fs::remove_dir_all(&dir);
        std::fs::create_dir_all(&dir).unwrap();
        dir
    }

    /// This repository's tracked files, read from the working tree.
    fn real_tree() -> Tree {
        let root = real_root();
        let out = Command::new("git")
            .args(["ls-files", "-z"])
            .current_dir(&root)
            .output()
            .unwrap();
        let mut files = Vec::new();
        for path in out.stdout.split(|&b| b == 0).filter(|p| !p.is_empty()) {
            let path = String::from_utf8_lossy(path).into_owned();
            if let Ok(bytes) = std::fs::read(root.join(&path)) {
                files.push((path, bytes));
            }
        }
        Tree::new(files)
    }

    /// A one-commit history over `tree`, loaded.
    fn loaded_of(tree: &Tree) -> (History, Loaded) {
        let mut history = History::default();
        let name = "1".repeat(40);
        history.insert(
            name.clone(),
            Commit {
                parents: Vec::new(),
                date: CommitterDate {
                    seconds: 1_790_812_800,
                    offset_minutes: 0,
                },
                tree: tree.clone(),
                hosting: false,
            },
        );
        let loaded = load(&history, &name).unwrap_or_else(|e| panic!("{e}"));
        (history, loaded)
    }

    fn pin() -> String {
        super::toolchain_file(&real_tree()).unwrap()
    }

    /// A record implementing `package`, on `target`, with a behavioral fact over the example's model file.
    fn record(package: &str, target: &str) -> String {
        format!(
            r#"(catalog-record example.base
  (version "0.1.0")
  (catalog algorithms)
  (source (origin "a test") (license "MIT OR Apache-2.0"))
  (maintainer M2)
  (depends)
  (supersedes)
  (profiles rt-static-up-v1)
  (targets {target})
  (preconditions)
  (guarantees "it builds")
  (implementation (version "0.1.0") (sources "{package}"))
  (behavior-model (version "0.1.0") (sources "docs/example/model.txt") (describes)
    (facts (fact one-processor yes (locator (file "docs/example/model.txt")) (basis "the model says so"))))
  (timing-model (version "0.1.0") (sources) (measured-with)
    (facts (fact f yes (locator (code example.base "{package}/src/lib.rs")) (basis "see the code"))) (costs)))
"#
        )
    }

    /// A small workspace of one package `crates/p` with `lib`, over this repository's pin and configuration, its
    /// lock generated by cargo; `extra` files added.
    fn small_workspace(name: &str, lib: &str, extra: &[(&str, &str)]) -> (Tree, PathBuf) {
        let real = real_tree();
        let dir = scratch(name);
        let mut files: Vec<(String, Vec<u8>)> = vec![
            (
                "Cargo.toml".into(),
                b"[workspace]\nresolver = \"2\"\nmembers = [\"crates/*\"]\n".to_vec(),
            ),
            (
                "crates/p/Cargo.toml".into(),
                b"[package]\nname = \"p\"\nversion = \"0.1.0\"\nedition = \"2021\"\n".to_vec(),
            ),
            ("crates/p/src/lib.rs".into(), lib.as_bytes().to_vec()),
            (
                "rust-toolchain.toml".into(),
                real.get("rust-toolchain.toml").unwrap().to_vec(),
            ),
            (
                ".cargo/config.toml".into(),
                real.get(".cargo/config.toml").unwrap().to_vec(),
            ),
            ("docs/example/model.txt".into(), b"model\n".to_vec()),
            (
                "targets/example-target.env".into(),
                b"TARGET_ID=example-target\nRUST_TARGET=riscv64imac-unknown-none-elf\n".to_vec(),
            ),
            (
                "targets/example-target.eadl".into(),
                b"(platform)\n".to_vec(),
            ),
            (
                "catalog/experimental/example.base.catalog".into(),
                record("crates/p", "example-target").into_bytes(),
            ),
        ];
        for (path, text) in extra {
            files.push(((*path).to_owned(), text.as_bytes().to_vec()));
        }
        // Cargo writes the lock in a preparation directory, which the tree then holds.
        let prep = dir.join("prep");
        for (path, bytes) in &files {
            let full = prep.join(path);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(full, bytes).unwrap();
        }
        let out = Command::new("cargo")
            .args(["generate-lockfile", "--offline"])
            .current_dir(&prep)
            .output()
            .unwrap();
        assert!(
            out.status.success(),
            "{}",
            String::from_utf8_lossy(&out.stderr)
        );
        files.push((
            "Cargo.lock".into(),
            std::fs::read(prep.join("Cargo.lock")).unwrap(),
        ));
        (Tree::new(files), dir)
    }

    /// `tree` with its lock blessed, as the gate would commit it.
    fn with_lock(mut tree: Tree) -> Tree {
        tree.remove(PATH);
        let catalog = Catalog::read(tree.clone()).unwrap_or_else(|e| panic!("{e}"));
        let hashes = catalog.hashes().unwrap_or_else(|e| panic!("{e}"));
        tree.insert(PATH, Lock::new(1, blessed(&catalog, &hashes)).render());
        tree
    }

    fn run(tree: &Tree, dir: &std::path::Path) -> Result<super::Built, Failure> {
        let tree = with_lock(tree.clone());
        let (_, loaded) = loaded_of(&tree);
        let root = dir.join("tree");
        super::write_tree(&tree, &root)?;
        check(
            &loaded,
            &tree,
            &root,
            &real_root(),
            &pin(),
            &dir.join("scratch"),
        )
    }

    #[track_caller]
    fn refused(result: Result<super::Built, Failure>, says: &str) {
        match result {
            Err(Failure::Refused(r)) => {
                assert_eq!(r.code, Code::Source, "{r}");
                assert!(r.message.contains(says), "not about `{says}`: {r}");
            }
            other => panic!("expected catalog-source about `{says}`, got {other:?}"),
        }
    }

    #[test]
    fn a_package_builds_across_the_matrix_and_its_sources_are_read() {
        let (tree, dir) = small_workspace(
            "matrix",
            "#![no_std]\nmod a;\npub fn f() -> u32 { a::A }\n",
            &[("crates/p/src/a.rs", "pub const A: u32 = 1;\n")],
        );
        let built = run(&tree, &dir).unwrap_or_else(|e| panic!("{e:?}"));
        // dev and release, for the host and for the target's `RUST_TARGET`.
        assert_eq!((built.packages, built.builds), (1, 4));
        assert!(built.units >= 4, "{built:?}");
    }

    #[test]
    fn a_prerequisite_is_judged_against_the_sets_the_root_and_its_blob() {
        // §3's token rules keep `include_str!` and `#[path]` out at load, and every file under a package is in its
        // own set, so this check is the backstop for what no scan sees; it is judged here on its own.
        let dir = scratch("prerequisite");
        let root = dir.join("tree");
        std::fs::create_dir_all(root.join("crates/p/src")).unwrap();
        std::fs::write(root.join("crates/p/src/lib.rs"), b"x").unwrap();
        std::fs::write(root.join("NOTES"), b"n").unwrap();
        let tree = Tree::new([
            ("crates/p/src/lib.rs".to_owned(), b"x".to_vec()),
            ("NOTES".to_owned(), b"n".to_vec()),
        ]);
        let mut sets: BTreeMap<String, std::collections::BTreeSet<String>> = BTreeMap::new();
        sets.entry("crates/p/src/lib.rs".into())
            .or_default()
            .insert("example.base implementation".into());
        let judge = |p: &str| judge_prerequisite("crates/p", "p", p, &root, &sets, &tree);
        judge("crates/p/src/lib.rs").unwrap_or_else(|e| panic!("{e:?}"));
        let says = |r: Result<(), Failure>, text: &str| match r {
            Err(Failure::Refused(r)) => assert!(r.message.contains(text), "{r}"),
            other => panic!("expected a refusal about `{text}`, got {other:?}"),
        };
        says(judge("NOTES"), "a file in no source set");
        says(judge("../etc/passwd"), "outside the written index");
        says(judge("/etc/passwd"), "outside the written index");
        std::fs::write(root.join("crates/p/src/lib.rs"), b"changed").unwrap();
        says(judge("crates/p/src/lib.rs"), "bytes after the build differ");
    }

    #[test]
    fn an_environment_dependency_is_refused() {
        let (tree, dir) = small_workspace(
            "env",
            "#![no_std]\npub const X: Option<&str> = option_env!(\"ARCHOGEN_X\");\n",
            &[],
        );
        refused(run(&tree, &dir), "environment variable `ARCHOGEN_X`");
        let (tree, dir) = small_workspace(
            "env-ok",
            "#![no_std]\npub const X: &str = env!(\"CARGO_PKG_NAME\");\n",
            &[],
        );
        run(&tree, &dir).unwrap_or_else(|e| panic!("{e:?}"));
    }

    #[test]
    fn a_package_that_does_not_build_is_refused() {
        let (tree, dir) = small_workspace("broken", "pub fn f( {\n", &[]);
        refused(run(&tree, &dir), "does not build");
    }

    #[test]
    fn a_configuration_on_the_path_is_held() {
        // The tracked copy above the written tree, this repository's, must equal the judged tree's bytes.
        let (mut tree, dir) = small_workspace("config", "pub fn f() {}\n", &[]);
        tree.insert(
            ".cargo/config.toml",
            b"[alias]\nzz = \"version\"\n".to_vec(),
        );
        refused(run(&tree, &dir), "bytes differ");
        // A configuration under `target/` of the repository, and one above the repository.
        let repo = scratch("config-layout");
        let root = repo.join("target/x/tree");
        std::fs::create_dir_all(&root).unwrap();
        let fresh = Tree::new([]);
        std::fs::create_dir_all(repo.join("target/.cargo")).unwrap();
        std::fs::write(repo.join("target/.cargo/config.toml"), "[alias]\n").unwrap();
        match configurations_on_path(&root, &root, &repo, &fresh) {
            Err(Failure::Refused(r)) => assert!(r.message.contains("under `target/`"), "{r}"),
            other => panic!("{other:?}"),
        }
        std::fs::remove_dir_all(repo.join("target/.cargo")).unwrap();
        match configurations_on_path(&root, &root, &repo, &fresh) {
            Err(Failure::Refused(r)) => assert!(r.message.contains("above the repository"), "{r}"),
            other => panic!("{other:?}"),
        }
    }

    #[test]
    fn the_workspaces_own_runtime_core_passes() {
        let mut tree = real_tree();
        tree.insert("docs/example/model.txt", b"model\n".to_vec());
        tree.insert(
            "catalog/experimental/example.base.catalog",
            record("crates/rt-core", "riscv-virt-up").into_bytes(),
        );
        let dir = scratch("rt-core");
        let built = run(&tree, &dir).unwrap_or_else(|e| panic!("{e:?}"));
        assert_eq!((built.packages, built.builds), (1, 4), "{built:?}");
        let _ = BTreeMap::<u8, u8>::new();
    }

    #[test]
    fn the_commits_a_push_replays_are_built_where_they_change_what_is_built() {
        use super::check_replayed;
        let (first, dir) = small_workspace("replay", "#![no_std]\npub fn f() -> u32 { 1 }\n", &[]);
        let first = with_lock(first);
        // The second commit changes the package and bumps the implementation; the third changes only a document.
        let mut second = first.clone();
        second.insert(
            "crates/p/src/lib.rs",
            b"#![no_std]\npub fn f() -> u32 { 2 }\n".to_vec(),
        );
        let bumped = record("crates/p", "example-target").replace(
            "(implementation (version \"0.1.0\")",
            "(implementation (version \"0.1.1\")",
        );
        second.insert(
            "catalog/experimental/example.base.catalog",
            bumped.into_bytes(),
        );
        let second = {
            let mut t = second;
            t.remove(PATH);
            let catalog = Catalog::read(t.clone()).unwrap();
            let hashes = catalog.hashes().unwrap();
            let mut lines = blessed(&catalog, &hashes);
            lines.extend(Lock::parse(first.get(PATH).unwrap()).unwrap().lines);
            t.insert(PATH, Lock::new(1, lines).render());
            t
        };
        let mut third = second.clone();
        third.insert("docs/example/model.txt", b"model\n".to_vec());
        third.insert("NOTES", b"a note\n".to_vec());
        let mut history = History::default();
        let names = [
            "1".repeat(40),
            "2".repeat(40),
            "3".repeat(40),
            "4".repeat(40),
        ];
        let date = CommitterDate {
            seconds: 1_790_812_800,
            offset_minutes: 0,
        };
        history.insert(
            names[0].clone(),
            Commit {
                parents: vec![],
                date,
                tree: Tree::new([]),
                hosting: false,
            },
        );
        history.insert(
            names[1].clone(),
            Commit {
                parents: vec![names[0].clone()],
                date,
                tree: first.clone(),
                hosting: false,
            },
        );
        history.insert(
            names[2].clone(),
            Commit {
                parents: vec![names[1].clone()],
                date,
                tree: second.clone(),
                hosting: false,
            },
        );
        history.insert(
            names[3].clone(),
            Commit {
                parents: vec![names[2].clone()],
                date,
                tree: third.clone(),
                hosting: false,
            },
        );
        let built = check_replayed(
            &history,
            &[&names[0]],
            &names[3],
            &real_root(),
            &pin(),
            &dir.join("scratch"),
        )
        .unwrap_or_else(|e| panic!("{e:?}"));
        // The first catalog commit and the one that changed the package are built; the head is the caller's, and the
        // document-only commit would not be.
        let names_built: Vec<&str> = built.iter().map(|(n, _)| n.as_str()).collect();
        assert_eq!(names_built, [names[1].as_str(), names[2].as_str()]);
        assert!(built.iter().all(|(_, b)| b.builds == 4), "{built:?}");
        let mut fourth = third.clone();
        fourth.insert("NOTES", b"another note\n".to_vec());
        history.insert(
            "5".repeat(40),
            Commit {
                parents: vec![names[3].clone()],
                date,
                tree: fourth,
                hosting: false,
            },
        );
        let built = check_replayed(
            &history,
            &[&names[2]],
            &"5".repeat(40),
            &real_root(),
            &pin(),
            &dir.join("scratch2"),
        )
        .unwrap_or_else(|e| panic!("{e:?}"));
        assert!(
            built.is_empty(),
            "a document-only commit is not built: {built:?}"
        );
        // A replayed commit whose package does not build is refused, naming it.
        let mut broken = first.clone();
        broken.insert("crates/p/src/lib.rs", b"pub fn f( {\n".to_vec());
        let broken = with_lock(broken);
        let mut h = History::default();
        h.insert(
            names[0].clone(),
            Commit {
                parents: vec![],
                date,
                tree: Tree::new([]),
                hosting: false,
            },
        );
        h.insert(
            names[1].clone(),
            Commit {
                parents: vec![names[0].clone()],
                date,
                tree: broken,
                hosting: false,
            },
        );
        h.insert(
            names[2].clone(),
            Commit {
                parents: vec![names[1].clone()],
                date,
                tree: first.clone(),
                hosting: false,
            },
        );
        match check_replayed(
            &h,
            &[&names[0]],
            &names[2],
            &real_root(),
            &pin(),
            &dir.join("scratch3"),
        ) {
            Err(Failure::Refused(r)) => {
                assert_eq!(r.code, Code::Source, "{r}");
                assert!(
                    r.message.contains("at the replayed commit 2222")
                        && r.message.contains("does not build"),
                    "{r}"
                );
            }
            other => panic!("{other:?}"),
        }
    }
}
