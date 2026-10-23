//! The census's oracle (`docs/specs/trust/decision_trust-role-programs.md` §3; round 10's change of method, leaf
//! `M3.5.2.7.3`): a corpus of the probes rounds 5 to 11 ran, built in a matrix of configurations, every file of the
//! tree any build's unit read — its own, and those of every crate it links and of their build scripts — counted by the
//! census for that unit and its closure (round 11's D8, leaf `M3.5.2.8.4`); and rustdoc run on a doc-tested library,
//! every block it compiled one the census refuses. Its reading of the builds is its own — a compiler wrapper logging
//! each invocation, rustc's dependency information, and rustdoc's own failure to read a file each doc test names — so
//! it shares no code with the instrument's reader of a build, and the census is held to what rustc and rustdoc did,
//! not to what the census's author expected. What it does not check: what the census refuses, which the fixtures
//! hold; what it over-counts; a non-member's tests, which no build of the workspace makes; and what a build script or
//! a procedural macro generates.

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_catalog::tree::Tree;

use super::Sources;
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
            "[workspace]\nresolver = \"2\"\nmembers = [\"p\", \"q\", \"n\", \"r\", \"k\", \"e\", \"o\", \"u\", \"dv\", \"bsd\", \"d\"]\n".to_owned(),
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
        // Round 11's code probes: `cfg(test)` on a field, a variant, a parameter and an arm, and inside an inline module, a
        // raw-identifier module, an inclusion in a macro's arguments, a debugger visualizer, a `doc` attribute's document
        // by a qualified path, and a precise capture before an inclusion.
        ("k/Cargo.toml", manifest("k", "")),
        (
            "k/src/lib.rs",
            "#![debugger_visualizer(gdb_script_file = \"pretty.py\")]\n\
             mod r#type;\n\
             pub struct S {\n    #[cfg(test)]\n    pub a: u8,\n}\n\
             pub const FIELD: &str = include_str!(\"field.txt\");\n\
             pub enum E {\n    #[cfg(test)]\n    A,\n    B,\n}\n\
             pub const VARIANT: &str = include_str!(\"variant.txt\");\n\
             pub fn g(#[cfg(test)] _a: u8) {}\n\
             pub const PARAM: &str = include_str!(\"param.txt\");\n\
             pub fn h(x: u8) -> u8 {\n    match x {\n        #[cfg(test)]\n        0 => 1,\n        _ => 2,\n    }\n}\n\
             pub const ARM: &str = include_str!(\"arm.txt\");\n\
             mod im {\n    #![cfg(test)]\n}\n\
             pub const INNER: &str = include_str!(\"inner.txt\");\n\
             macro_rules! keep {\n    ($e:expr) => {\n        pub const KEPT: &str = $e;\n    };\n}\n\
             keep!(include_str!(\"kept.txt\"));\n\
             #[doc = ::core::include_str!(\"doc.md\")]\npub struct Documented;\n\
             pub fn captured() -> impl Sized + use<> {}\n\
             pub const AFTER_USE: &str = include_str!(\"after_use.txt\");\n"
                .to_owned(),
        ),
        ("k/src/type.rs", module.clone()),
        ("k/src/pretty.py", "# a debugger script\n".to_owned()),
        ("k/src/field.txt", "f\n".to_owned()),
        ("k/src/variant.txt", "v\n".to_owned()),
        ("k/src/param.txt", "p\n".to_owned()),
        ("k/src/arm.txt", "a\n".to_owned()),
        ("k/src/inner.txt", "i\n".to_owned()),
        ("k/src/kept.txt", "k\n".to_owned()),
        ("k/src/doc.md", "Documented.\n".to_owned()),
        ("k/src/after_use.txt", "u\n".to_owned()),
        // Round 11's D8: a unit's closure — a renamed dependency, an optional one, a platform's, a development one, and
        // one whose build script compiles a file of its own.
        (
            "e/Cargo.toml",
            manifest(
                "e",
                "[dependencies]\nalias = { package = \"n\", path = \"../n\" }\nbsd = { path = \"../bsd\" }\n\
                 o = { path = \"../o\", optional = true }\n\n[target.'cfg(unix)'.dependencies]\nu = { path = \"../u\" }\n\n\
                 [target.'cfg(windows)'.dependencies]\nu = { path = \"../u\" }\n\n\
                 [dev-dependencies]\ndv = { path = \"../dv\" }\n\n[features]\nwith_o = [\"dep:o\"]\n",
            ),
        ),
        ("e/src/lib.rs", "pub fn e() {}\n".to_owned()),
        ("e/tests/t.rs", "#[test]\nfn t() {}\n".to_owned()),
        ("o/Cargo.toml", manifest("o", "")),
        ("o/src/lib.rs", "#[path = \"o_only.rs\"]\nmod o_only;\n".to_owned()),
        ("o/src/o_only.rs", module.clone()),
        ("u/Cargo.toml", manifest("u", "")),
        ("u/src/lib.rs", "#[path = \"u_only.rs\"]\nmod u_only;\n".to_owned()),
        ("u/src/u_only.rs", module.clone()),
        ("dv/Cargo.toml", manifest("dv", "")),
        ("dv/src/lib.rs", "#[path = \"dv_only.rs\"]\nmod dv_only;\n".to_owned()),
        ("dv/src/dv_only.rs", module.clone()),
        ("bsd/Cargo.toml", manifest("bsd", "")),
        ("bsd/src/lib.rs", "pub fn b() {}\n".to_owned()),
        (
            "bsd/build.rs",
            "#[path = \"helper/script.rs\"]\nmod script;\nfn main() {\n    script::run();\n}\n".to_owned(),
        ),
        ("bsd/helper/script.rs", "pub fn run() {}\n".to_owned()),
        // Round 11's D1: a doc-tested library, a doc test in each spelling rustdoc compiles, each including a file the
        // corpus lacks, so rustdoc's failure to read it names every block it compiled.
        ("d/Cargo.toml", manifest("d", "")),
        (
            "d/src/lib.rs",
            "#![doc = include_str!(\"../README.md\")]\n\
             pub mod fence_tildes;\npub mod fence_four;\npub mod indented;\npub mod rust_foo;\npub mod ignored;\n\
             pub mod ignore_windows;\npub mod split_attributes;\npub mod mixed;\npub mod crlf;\npub mod nested_block;\n\
             pub mod block_star;\n"
                .to_owned(),
        ),
        ("d/README.md", "Text.\n\n    include!(\"oracle_doc_missing/readme.rs\");\n".to_owned()),
        (
            "d/src/fence_tildes.rs",
            "/// ~~~\n/// include!(\"oracle_doc_missing/fence_tildes.rs\");\n/// ~~~\npub fn f() {}\n".to_owned(),
        ),
        (
            "d/src/fence_four.rs",
            "/// ````\n/// include!(\"oracle_doc_missing/fence_four.rs\");\n/// ````\npub fn f() {}\n".to_owned(),
        ),
        (
            "d/src/indented.rs",
            "/// Text.\n///\n///     include!(\"oracle_doc_missing/indented.rs\");\npub fn f() {}\n".to_owned(),
        ),
        (
            "d/src/rust_foo.rs",
            "/// ```rust,foo\n/// include!(\"oracle_doc_missing/rust_foo.rs\");\n/// ```\npub fn f() {}\n".to_owned(),
        ),
        (
            "d/src/ignored.rs",
            "/// ```ignore\n/// include!(\"oracle_doc_missing/ignored.rs\");\n/// ```\npub fn f() {}\n".to_owned(),
        ),
        (
            "d/src/ignore_windows.rs",
            "/// ```ignore-windows\n/// include!(\"oracle_doc_missing/ignore_windows.rs\");\n/// ```\npub fn f() {}\n"
                .to_owned(),
        ),
        (
            "d/src/split_attributes.rs",
            "#[doc = \"```\"]\n#[doc = \"include!(\\\"oracle_doc_missing/split_attributes.rs\\\");\"]\n#[doc = \"```\"]\n\
             pub fn f() {}\n"
                .to_owned(),
        ),
        (
            "d/src/mixed.rs",
            "/// ```\n#[doc = \"include!(\\\"oracle_doc_missing/mixed.rs\\\");\"]\n/// ```\npub fn f() {}\n".to_owned(),
        ),
        (
            "d/src/crlf.rs",
            "/// ```\r\n/// include!(\"oracle_doc_missing/crlf.rs\");\r\n/// ```\r\npub fn f() {}\r\n".to_owned(),
        ),
        (
            "d/src/nested_block.rs",
            "/** a /* b */ c\n\n```\ninclude!(\"oracle_doc_missing/nested_block.rs\");\n```\n*/\npub fn f() {}\n"
                .to_owned(),
        ),
        (
            "d/src/block_star.rs",
            "/**\n * ```\n * include!(\"oracle_doc_missing/block_star.rs\");\n * ```\n */\npub fn f() {}\n".to_owned(),
        ),
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
    let links = BTreeSet::new();
    let reader = Sources::new(&tree, &links);
    let mut doc_tested: BTreeMap<&String, bool> = BTreeMap::new();
    for ((_, kind, _), srcs) in &sources {
        for src in srcs {
            *doc_tested.entry(src).or_default() |=
                matches!(kind.as_str(), "lib" | "rlib" | "proc-macro");
        }
    }
    let mut reaches = BTreeMap::new();
    for (src, doc_tested) in doc_tested {
        if let Ok(root) = reader.root(src) {
            reaches.insert(src.clone(), reader.reach(&root, doc_tested));
        }
    }
    let census = crate::trust::Census::lexical(&reaches, &sources);

    // Each package's dependency edges, by directory, for the census's closure of a unit.
    let mut id_dir: BTreeMap<String, String> = BTreeMap::new();
    for p in meta.get("packages").map(Json::elements).unwrap_or_default() {
        let manifest = p
            .get("manifest_path")
            .and_then(Json::as_str)
            .unwrap_or_default();
        if let Some(dir) = manifest
            .strip_prefix(&prefix)
            .and_then(|m| m.strip_suffix("Cargo.toml"))
        {
            let id = p.get("id").and_then(Json::as_str).unwrap_or_default();
            id_dir.insert(id.to_owned(), dir.trim_end_matches('/').to_owned());
        }
    }
    let mut graph: Vec<(String, String, String)> = Vec::new();
    for node in meta
        .get("resolve")
        .and_then(|r| r.get("nodes"))
        .map(Json::elements)
        .unwrap_or_default()
    {
        let Some(from) = id_dir.get(node.get("id").and_then(Json::as_str).unwrap_or_default())
        else {
            continue;
        };
        for d in node.get("deps").map(Json::elements).unwrap_or_default() {
            let Some(to) = id_dir.get(d.get("pkg").and_then(Json::as_str).unwrap_or_default())
            else {
                continue;
            };
            for k in d.get("dep_kinds").map(Json::elements).unwrap_or_default() {
                let kind = k.get("kind").and_then(Json::as_str).unwrap_or("normal");
                graph.push((from.clone(), to.clone(), kind.to_owned()));
            }
        }
    }

    // Every build of the matrix, through the oracle's own wrapper.
    let wrapper = f.base.join("oracle-rustc");
    std::fs::write(&wrapper, WRAPPER).expect("the wrapper");
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        std::fs::set_permissions(&wrapper, std::fs::Permissions::from_mode(0o755))
            .expect("executable");
    }
    // A file of the corpus by its repository path, none for one outside it.
    let in_repo = |file: &str| -> Option<String> {
        let path = if file.starts_with('/') {
            PathBuf::from(file)
        } else {
            repo.join(file)
        };
        let c = std::fs::canonicalize(path).ok()?;
        Some(c.strip_prefix(&repo).ok()?.to_string_lossy().into_owned())
    };
    let mut checked = 0usize;
    let mut linked = 0usize;
    let mut read: BTreeSet<String> = BTreeSet::new();
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
        // Each invocation: its target, whether it is a test, the files it read, and the crates it links.
        let mut units: Vec<Invocation> = Vec::new();
        let mut produced: BTreeMap<PathBuf, usize> = BTreeMap::new();
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
                .unwrap_or_default()
                .to_owned();
            let Some(target) = args
                .iter()
                .filter(|a| !a.starts_with('-') && a.ends_with(".rs"))
                .find_map(|a| std::fs::canonicalize(repo.join(a)).ok())
                .and_then(|root| roots.get(&root).cloned())
            else {
                continue;
            };
            let dep_info = Path::new(&out_dir).join(format!("{crate_name}{extra}.d"));
            let Ok(d) = std::fs::read_to_string(&dep_info) else {
                continue;
            };
            let id = units.len();
            for suffix in ["rlib", "rmeta", "dylib", "so"] {
                let artifact = Path::new(&out_dir).join(format!("lib{crate_name}{extra}.{suffix}"));
                if let Ok(c) = std::fs::canonicalize(artifact) {
                    produced.insert(c, id);
                }
            }
            let externs = args
                .windows(2)
                .filter(|w| w[0] == "--extern")
                .filter_map(|w| w[1].split_once('=').map(|(_, p)| p.to_owned()))
                .filter_map(|p| std::fs::canonicalize(p).ok())
                .collect();
            units.push(Invocation {
                target,
                test: args.iter().any(|a| a == "--test"),
                files: prerequisites(&d)
                    .iter()
                    .filter_map(|f| in_repo(f))
                    .collect(),
                externs,
            });
        }
        for u in &units {
            read.extend(u.files.iter().cloned());
        }
        for u in &units {
            let (dir, kinds, name) = &u.target;
            let kind = kinds.first().map(String::as_str).unwrap_or_default();
            let library = matches!(
                kind,
                "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro"
            );
            // The census's count for this unit: a test unit's every file, a library's and a binary's as no test, any
            // other target's every file — and every library its package's closure reaches, a build script's with it.
            let (own, unit) = match (u.test, kind) {
                (true, _) if library => (
                    census
                        .own
                        .get(&(dir.clone(), "lib-test".into(), name.clone())),
                    "lib-test",
                ),
                (true, "bin") => (
                    census
                        .own
                        .get(&(dir.clone(), "bin-test".into(), name.clone())),
                    "bin-test",
                ),
                (false, _) if library => (census.libraries.get(dir), kind),
                (_, k) => (
                    census.own.get(&(dir.clone(), k.to_owned(), name.clone())),
                    k,
                ),
            };
            let mut counted: BTreeSet<&String> = own.into_iter().flatten().collect();
            for p in crate::trust::target_closure(dir, unit, &graph) {
                counted.extend(census.libraries.get(&p).into_iter().flatten());
            }
            // What rustc read for it: its own files, every crate it links, transitively, and the build scripts of their
            // packages, which run as each builds.
            let mut seen: BTreeSet<usize> = BTreeSet::new();
            let mut todo: Vec<&PathBuf> = u.externs.iter().collect();
            while let Some(artifact) = todo.pop() {
                let Some(&k) = produced.get(artifact) else {
                    continue;
                };
                if seen.insert(k) {
                    todo.extend(units[k].externs.iter());
                }
            }
            let packages: BTreeSet<&String> = seen.iter().map(|&k| &units[k].target.0).collect();
            let scripts = units.iter().filter(|s| {
                s.target.1.iter().any(|k| k == "custom-build") && packages.contains(&s.target.0)
            });
            checked += 1;
            linked += usize::from(!seen.is_empty());
            let closure = seen.iter().map(|&k| &units[k]).chain(scripts);
            for file in u.files.iter().chain(closure.flat_map(|c| c.files.iter())) {
                if !counted.contains(file) {
                    missed.push(format!(
                        "{config:?}: the {}{kind} `{name}` of `{dir}` read `{file}`, which the census does not count",
                        if u.test { "test unit of the " } else { "" }
                    ));
                }
            }
        }
    }

    // The rustdoc leg (round 11's D1): every block rustdoc compiles fails to read the file its case names, ignored ones
    // under `--ignored`, so rustdoc's output names each — and the census must refuse a doc test in that case's file.
    let doc_root = reaches
        .get("d/src/lib.rs")
        .expect("the doc-tested library's census");
    let mut rustdoc = String::new();
    for ignored in [&[][..], &["--", "--ignored"][..]] {
        let out = Command::new("cargo")
            .args(["test", "--doc", "-p", "d", "--offline", "--locked"])
            .args(ignored)
            .env("RUSTUP_TOOLCHAIN", "1.95.0")
            .env("CARGO_TARGET_DIR", f.base.join("oracle-target-doc"))
            .current_dir(&repo)
            .output()
            .expect("cargo test --doc");
        rustdoc.push_str(&String::from_utf8_lossy(&out.stdout));
        rustdoc.push_str(&String::from_utf8_lossy(&out.stderr));
    }
    for (case, file) in DOC_CASES {
        assert!(
            rustdoc.contains(&format!("oracle_doc_missing/{case}.rs")),
            "{case}: rustdoc compiled no block of it:\n{rustdoc}"
        );
        if !doc_root
            .unfollowed
            .iter()
            .any(|u| u.file == *file && u.why.contains("doc test"))
        {
            missed.push(format!("rustdoc compiled a doc test of `{file}` ({case}), which the census does not refuse"));
        }
    }

    missed.sort();
    missed.dedup();
    assert!(missed.is_empty(), "{missed:#?}");
    // Not vacuous: every package's units, in every configuration, were read; units linking others were followed; and
    // each probe's file was read by some build.
    assert!(checked >= 60, "{checked} units checked");
    assert!(linked >= 12, "{linked} units' closures followed");
    for probe in PROBED {
        assert!(read.contains(*probe), "no build read `{probe}`");
    }
}

/// One compiler invocation the wrapper logged: its target (package, kinds, name), whether it is a test, the files of
/// the corpus it read, and the artifacts of the crates it links.
struct Invocation {
    target: (String, Vec<String>, String),
    test: bool,
    files: BTreeSet<String>,
    externs: Vec<PathBuf>,
}

/// Each doc test case of `d`, and the file whose doc test the census must refuse for it.
const DOC_CASES: &[(&str, &str)] = &[
    ("fence_tildes", "d/src/fence_tildes.rs"),
    ("fence_four", "d/src/fence_four.rs"),
    ("indented", "d/src/indented.rs"),
    ("rust_foo", "d/src/rust_foo.rs"),
    ("ignored", "d/src/ignored.rs"),
    ("ignore_windows", "d/src/ignore_windows.rs"),
    ("split_attributes", "d/src/split_attributes.rs"),
    ("mixed", "d/src/mixed.rs"),
    ("crlf", "d/src/crlf.rs"),
    ("nested_block", "d/src/nested_block.rs"),
    ("block_star", "d/src/block_star.rs"),
    ("readme", "d/README.md"),
];

/// A file each of round 11's probes makes some build read, so none is vacuous: the code probes' and the closure's.
const PROBED: &[&str] = &[
    "k/src/type.rs",
    "k/src/pretty.py",
    "k/src/field.txt",
    "k/src/variant.txt",
    "k/src/param.txt",
    "k/src/arm.txt",
    "k/src/inner.txt",
    "k/src/kept.txt",
    "k/src/doc.md",
    "k/src/after_use.txt",
    "o/src/o_only.rs",
    "u/src/u_only.rs",
    "dv/src/dv_only.rs",
    "bsd/helper/script.rs",
    "n/src/deep/included.rs",
];
