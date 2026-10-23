//! The census's oracle (`docs/specs/trust/decision_trust-role-programs.md` §3; round 10's change of method, leaf
//! `M3.5.2.7.3`): a corpus of the probes rounds 5 to 10 ran, built in a matrix of configurations, every file of the
//! tree any build's unit read counted by the census for that unit. Its reading of the builds is its own — a compiler
//! wrapper logging each invocation, and rustc's dependency information — so it shares no code with the instrument's
//! reader of a build, and the census is held to what rustc did, not to what the census's author expected.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_catalog::tree::Tree;

use super::{normalized, reach};
use crate::json::{self, Json};

/// The corpus: one workspace, a package for each family of probe.
fn corpus() -> Vec<(&'static str, String)> {
    let manifest = |name: &str, extra: &str| {
        format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n{extra}")
    };
    let module = "pub fn f() -> u32 { 1 }\n".to_owned();
    vec![
        (
            "Cargo.toml",
            "[workspace]\nresolver = \"2\"\nmembers = [\"p\", \"q\", \"n\", \"r\"]\n".to_owned(),
        ),
        // Every `cfg` a build can turn, every target flag, every spelling of an inclusion.
        (
            "p/Cargo.toml",
            manifest(
                "p",
                "[features]\nx = []\n\n[[test]]\nname = \"t_off\"\ntest = false\n\n[[test]]\nname = \"t_req\"\n\
                 required-features = [\"x\"]\n\n[[test]]\nname = \"t_bare\"\nharness = false\n\n[[bench]]\nname = \
                 \"b_off\"\nbench = false\nharness = false\n",
            ),
        ),
        (
            "p/src/lib.rs",
            "#[cfg(debug_assertions)]\n#[path = \"dbg.rs\"]\nmod dbg;\n\
             #[cfg(not(debug_assertions))]\n#[path = \"rel.rs\"]\nmod rel;\n\
             #[cfg(feature = \"x\")]\n#[path = \"feat.rs\"]\nmod feat;\n\
             #[cfg(test)]\nmod tests;\n\
             #[cfg_attr(not(test), path = \"ca.rs\")]\n#[cfg_attr(test, path = \"cb.rs\")]\nmod ca;\n\
             mod inline {\n    #[path = \"other.rs\"]\n    pub mod inner;\n}\n\
             #[path = \"pl/loaded.rs\"]\nmod loaded;\n\
             pub const A: &str = include_str![\"a.txt\"];\n\
             pub const B: &[u8] = include_bytes!{\"b.txt\"};\n\
             pub const C: &str = include_str!(\"\\x63.txt\");\n\
             pub const D: &str = include_str!(r#\"d.txt\"#);\n\
             pub mod shared;\n\
             pub fn f() -> u32 { inline::inner::f() }\n"
                .to_owned(),
        ),
        ("p/src/dbg.rs", module.clone()),
        ("p/src/rel.rs", module.clone()),
        ("p/src/feat.rs", module.clone()),
        ("p/src/tests.rs", "#[test]\nfn t() {}\n".to_owned()),
        ("p/src/ca.rs", module.clone()),
        ("p/src/cb.rs", module.clone()),
        ("p/src/inline/other.rs", module.clone()),
        ("p/src/pl/loaded.rs", "mod child;\n".to_owned()),
        ("p/src/pl/child.rs", module.clone()),
        ("p/src/pl/loaded/child.rs", module.clone()),
        ("p/src/a.txt", "a\n".to_owned()),
        ("p/src/b.txt", "b\n".to_owned()),
        ("p/src/c.txt", "c\n".to_owned()),
        ("p/src/d.txt", "d\n".to_owned()),
        ("p/src/shared.rs", "pub fn shared() {}\n".to_owned()),
        (
            "p/src/main.rs",
            "#[cfg(not(test))]\n#[path = \"bin_only.rs\"]\nmod bin_only;\n#[cfg(test)]\n#[path = \"bin_test.rs\"]\nmod bin_test;\n\
             fn main() {}\n"
                .to_owned(),
        ),
        ("p/src/bin_only.rs", module.clone()),
        ("p/src/bin_test.rs", module.clone()),
        ("p/tests/t_off.rs", "#[path = \"../src/off_only.rs\"]\nmod off_only;\n#[test]\nfn t() {}\n".to_owned()),
        ("p/src/off_only.rs", module.clone()),
        ("p/tests/t_req.rs", "#[path = \"../src/req_only.rs\"]\nmod req_only;\n#[test]\nfn t() {}\n".to_owned()),
        ("p/src/req_only.rs", module.clone()),
        ("p/tests/t_bare.rs", "#[path = \"../src/bare_only.rs\"]\nmod bare_only;\nfn main() {}\n".to_owned()),
        ("p/src/bare_only.rs", module.clone()),
        ("p/benches/b_off.rs", "#[path = \"../src/bench_only.rs\"]\nmod bench_only;\nfn main() {}\n".to_owned()),
        ("p/src/bench_only.rs", module.clone()),
        (
            "p/examples/ex.rs",
            "#[cfg(not(test))]\n#[path = \"mods/ex_only.rs\"]\nmod ex_only;\n#[cfg(test)]\n#[path = \"mods/ex_test.rs\"]\nmod ex_test;\n\
             fn main() {}\n"
                .to_owned(),
        ),
        ("p/examples/mods/ex_only.rs", module.clone()),
        ("p/examples/mods/ex_test.rs", module.clone()),
        ("p/src/inc.rs", "pub fn inc() {}\n".to_owned()),
        // A `cdylib` alone, which no unit links.
        ("q/Cargo.toml", manifest("q", "[lib]\ncrate-type = [\"cdylib\"]\n")),
        ("q/src/lib.rs", "#[path = \"q_only.rs\"]\nmod q_only;\n".to_owned()),
        ("q/src/q_only.rs", module.clone()),
        // A package nested in another's directory, reached by a path.
        ("n/Cargo.toml", manifest("n", "")),
        // An included file's own module, resolved under its directory, not its includer's.
        (
            "n/src/lib.rs",
            "#[path = \"../sub/src/x.rs\"]\npub mod x;\ninclude!(\"deep/included.rs\");\n".to_owned(),
        ),
        ("n/src/deep/included.rs", "pub fn included() {}\nmod inc_child;\n".to_owned()),
        ("n/src/deep/inc_child.rs", "pub fn f() {}\n".to_owned()),
        ("n/sub/Cargo.toml", manifest("sub", "")),
        ("n/sub/src/lib.rs", "pub fn sub() {}\n".to_owned()),
        ("n/sub/src/x.rs", module.clone()),
        // A target whose crate root is another package's file, and a dependency's own paths.
        (
            "r/Cargo.toml",
            manifest(
                "r",
                "[dev-dependencies]\nn = { path = \"../n\" }\n\n[[test]]\nname = \"foreign\"\npath = \"../p/src/shared.rs\"\n",
            ),
        ),
        ("r/src/lib.rs", "pub fn r() {}\n".to_owned()),
    ]
}

/// The configurations each built in: `cargo test`'s own selection, every target, release, every feature, a plain
/// build in both profiles, and each target a selection skips, by name, with its required features.
const MATRIX: &[&[&str]] = &[
    &["test", "--no-run"],
    &["test", "--no-run", "--all-targets"],
    &["test", "--no-run", "--all-targets", "--release"],
    &["test", "--no-run", "--all-targets", "--all-features"],
    &["test", "--no-run", "--all-targets", "--no-default-features"],
    &["build", "--all-targets"],
    &["build", "--all-targets", "--release"],
    &["build", "--all-targets", "--all-features"],
    &["test", "--no-run", "--test", "t_off"],
    &["test", "--no-run", "--test", "t_req", "--features", "p/x"],
    &["test", "--no-run", "--bench", "b_off"],
    &["test", "--no-run", "--test", "t_bare"],
];

/// A compiler wrapper of the oracle's own: each invocation's arguments, one JSON array a line, then rustc itself.
const WRAPPER: &str = "#!/usr/bin/env python3\nimport json, os, sys\n\
with open(os.environ['ORACLE_LOG'], 'a') as f:\n    f.write(json.dumps(sys.argv[2:]) + '\\n')\n\
os.execv(sys.argv[1], sys.argv[1:])\n";

/// The files a dependency-information file names as prerequisites, as written.
fn prerequisites(text: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    for line in text.lines() {
        let Some((_, rest)) = line.split_once(": ") else {
            continue;
        };
        let mut current = String::new();
        let mut chars = rest.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                '\\' if chars.peek() == Some(&' ') => {
                    current.push(' ');
                    chars.next();
                }
                ' ' => {
                    if !current.is_empty() {
                        out.insert(std::mem::take(&mut current));
                    }
                }
                c => current.push(c),
            }
        }
        if !current.is_empty() {
            out.insert(current);
        }
    }
    out
}

#[test]
fn the_census_counts_every_file_any_build_of_the_corpus_reads() {
    crate::trust_test_process::bounded!(the_census_counts_every_file_any_build_of_the_corpus_reads);
    let files = corpus();
    let named: Vec<(&str, String)> = files.iter().map(|(p, t)| (*p, t.clone())).collect();
    let f = crate::trust::tests::fixture("census-oracle", "1.95.0", &named);
    let repo = std::fs::canonicalize(&f.repo).expect("the corpus");
    let prefix = format!("{}/", repo.display());

    // The census of the corpus, as the instrument computes it.
    let meta = Command::new("cargo")
        .args([
            "metadata",
            "--format-version",
            "1",
            "--offline",
            "--locked",
            "--all-features",
        ])
        .env("RUSTUP_TOOLCHAIN", "1.95.0")
        .current_dir(&repo)
        .output()
        .expect("cargo metadata");
    assert!(
        meta.status.success(),
        "{}",
        String::from_utf8_lossy(&meta.stderr)
    );
    let meta = json::parse(&String::from_utf8_lossy(&meta.stdout)).expect("its JSON");
    let tree = Tree::new(
        files
            .iter()
            .map(|(p, t)| ((*p).to_owned(), t.as_bytes().to_vec())),
    );
    let mut sources: BTreeMap<(String, String, String), BTreeSet<String>> = BTreeMap::new();
    // Each target's crate root, by its canonical path: its package, its kinds, its name.
    let mut roots: BTreeMap<PathBuf, (String, Vec<String>, String)> = BTreeMap::new();
    for p in meta.get("packages").map(Json::elements).unwrap_or_default() {
        let manifest = p
            .get("manifest_path")
            .and_then(Json::as_str)
            .unwrap_or_default();
        let Some(dir) = manifest
            .strip_prefix(&prefix)
            .and_then(|m| m.strip_suffix("Cargo.toml"))
            .map(|d| d.trim_end_matches('/').to_owned())
        else {
            continue;
        };
        for t in p.get("targets").map(Json::elements).unwrap_or_default() {
            let src = t.get("src_path").and_then(Json::as_str).unwrap_or_default();
            let rel = src.strip_prefix(&prefix).unwrap_or(src).to_owned();
            let kinds: Vec<String> = t
                .get("kind")
                .map(Json::elements)
                .unwrap_or_default()
                .iter()
                .filter_map(Json::as_str)
                .map(str::to_owned)
                .collect();
            let name = t
                .get("name")
                .and_then(Json::as_str)
                .unwrap_or_default()
                .to_owned();
            for k in &kinds {
                sources
                    .entry((dir.clone(), k.clone(), name.clone()))
                    .or_default()
                    .insert(rel.clone());
            }
            if let Ok(canonical) = std::fs::canonicalize(src) {
                roots.insert(canonical, (dir.clone(), kinds, name));
            }
        }
    }
    let outer = |from: &str, rel: &str| -> Option<String> {
        let path = if rel.starts_with('/') {
            PathBuf::from(rel)
        } else {
            repo.join(from).join(rel)
        };
        let path = std::fs::canonicalize(path).ok()?;
        Some(
            path.strip_prefix(&repo)
                .ok()?
                .to_string_lossy()
                .into_owned(),
        )
    };
    let mut reaches = BTreeMap::new();
    for src in sources.values().flatten() {
        if let Some(root) = normalized(src).filter(|r| tree.is_file(r)) {
            reaches
                .entry(src.clone())
                .or_insert_with(|| reach(&tree, &BTreeSet::new(), &root, &outer));
        }
    }
    let census = crate::trust::Census::lexical(&reaches, &sources);

    // Every build of the matrix, through the oracle's own wrapper.
    let wrapper = f.base.join("oracle-rustc");
    std::fs::write(&wrapper, WRAPPER).expect("the wrapper");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755))
            .expect("executable");
    }
    let mut checked = 0usize;
    let mut missed: Vec<String> = Vec::new();
    for (n, config) in MATRIX.iter().enumerate() {
        let log = f.base.join(format!("oracle-{n}.log"));
        let target_dir = f.base.join(format!("oracle-target-{n}"));
        let out = Command::new("cargo")
            .args(*config)
            .args(["--offline", "--locked"])
            .env("RUSTUP_TOOLCHAIN", "1.95.0")
            .env("RUSTC_WRAPPER", &wrapper)
            .env("ORACLE_LOG", &log)
            .env("CARGO_TARGET_DIR", &target_dir)
            .current_dir(&repo)
            .output()
            .expect("cargo");
        assert!(
            out.status.success(),
            "{config:?}: {}",
            String::from_utf8_lossy(&out.stderr)
        );
        let text = std::fs::read_to_string(&log).unwrap_or_default();
        for line in text.lines() {
            let Ok(Json::Array(args)) = json::parse(line) else {
                continue;
            };
            let args: Vec<String> = args
                .iter()
                .filter_map(Json::as_str)
                .map(str::to_owned)
                .collect();
            let value = |flag: &str| {
                args.iter()
                    .position(|a| a == flag)
                    .and_then(|i| args.get(i + 1))
                    .cloned()
            };
            let (Some(crate_name), Some(out_dir)) = (value("--crate-name"), value("--out-dir"))
            else {
                continue;
            };
            let extra = args
                .iter()
                .find_map(|a| a.strip_prefix("extra-filename="))
                .unwrap_or_default();
            let Some(root) = args
                .iter()
                .filter(|a| !a.starts_with('-') && a.ends_with(".rs"))
                .find_map(|a| std::fs::canonicalize(repo.join(a)).ok())
            else {
                continue;
            };
            let Some((dir, kinds, name)) = roots.get(&root) else {
                continue;
            };
            let test = args.iter().any(|a| a == "--test");
            let kind = kinds.first().map(String::as_str).unwrap_or_default();
            let library = matches!(
                kind,
                "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro"
            );
            // The census's count for this unit: a test unit's every file, a library's and a binary's as no test, any
            // other target's every file.
            let counted: Option<&BTreeSet<String>> = match (test, kind) {
                (true, _) if library => {
                    census
                        .own
                        .get(&(dir.clone(), "lib-test".into(), name.clone()))
                }
                (true, "bin") => census
                    .own
                    .get(&(dir.clone(), "bin-test".into(), name.clone())),
                (false, _) if library => census.libraries.get(dir),
                (_, k) => census.own.get(&(dir.clone(), k.to_owned(), name.clone())),
            };
            let dep_info = Path::new(&out_dir).join(format!("{crate_name}{extra}.d"));
            let Ok(d) = std::fs::read_to_string(&dep_info) else {
                continue;
            };
            checked += 1;
            for file in prerequisites(&d) {
                let path = if file.starts_with('/') {
                    PathBuf::from(&file)
                } else {
                    repo.join(&file)
                };
                let Some(rel) = std::fs::canonicalize(&path).ok().and_then(|c| {
                    c.strip_prefix(&repo)
                        .ok()
                        .map(|r| r.to_string_lossy().into_owned())
                }) else {
                    continue;
                };
                if !counted.is_some_and(|c| c.contains(&rel)) {
                    missed.push(format!(
                        "{config:?}: the {}{kind} `{name}` of `{dir}` read `{rel}`, which the census does not count",
                        if test { "test unit of the " } else { "" }
                    ));
                }
            }
        }
    }
    missed.sort();
    missed.dedup();
    assert!(missed.is_empty(), "{missed:#?}");
    // Not vacuous: every package's units, in every configuration, were read.
    assert!(checked >= 60, "{checked} units checked");
}
