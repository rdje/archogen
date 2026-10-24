//! The census's oracle (`docs/specs/trust/decision_trust-role-programs.md` §3; round 10's change of method, leaf
//! `M3.5.2.7.3`): a corpus of the probes rounds 5 to 12 ran, built in a matrix of configurations, every file of the
//! tree any build's unit read — its own, and those of every crate it links and of their build scripts — counted by the
//! census for that unit and its closure (round 11's D8, leaf `M3.5.2.8.4`); and rustdoc run on a doc-tested library,
//! every block it compiled one the census refuses. Its reading of the builds is its own — a compiler wrapper logging
//! each invocation, rustc's dependency information, and rustdoc's own failure to read a file each doc test names — so
//! it shares no code with the instrument's reader of a build, and the census is held to what rustc and rustdoc did,
//! not to what the census's author expected. And a generated leg holds the census's reading of where a literal or a
//! comment ends to rustc's: every pair of the reference's lexical constructs before a trigger, compiled by the pin in one
//! crate (round 12's change of method); and three more its reading of a file's head, of a trigger a macro spells and of
//! an inline module's children, each compiled so (round 13's D4). What it does not check: what the census refuses, which the fixtures hold; what
//! it over-counts; a non-member's tests, which no build of the workspace makes; what a build script or a procedural
//! macro generates; and a spelling beyond its families, or through a channel the census does not name (round 13's
//! D4).

use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_catalog::tree::Tree;

use super::{Reach, Sources};
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
            "[workspace]\nresolver = \"2\"\nmembers = [\"p\", \"q\", \"n\", \"r\", \"k\", \"e\", \"o\", \"u\", \"dv\", \"bsd\", \"d\", \"bc\"]\n".to_owned(),
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
        // raw-identifier module, an inclusion in a macro's arguments — std's, which keep them in place, as round 13's D1
        // refuses any other's — a debugger visualizer, a `doc` attribute's document by a qualified path, and a precise
        // capture before an inclusion.
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
             pub fn kept() -> String {\n    format!(\"{}\", include_str!(\"kept.txt\"))\n}\n\
             #[doc = ::core::include_str!(\"doc.md\")]\npub struct Documented;\n\
             pub fn captured() -> impl Sized + use<> {}\n\
             pub const AFTER_USE: &str = include_str!(\"after_use.txt\");\n\
             #[cfg_attr(test, path = \"mock.rs\")]\npub mod defaulted;\n"
                .to_owned(),
        ),
        // Round 12's D2: a `cfg_attr`'s `path` beside a module's own file, which a build with the `cfg` off reads.
        ("k/src/mock.rs", String::new()),
        ("k/src/defaulted.rs", "pub const D: &str = include_str!(\"defaulted.txt\");\n".to_owned()),
        ("k/src/defaulted.txt", "d\n".to_owned()),
        // Round 12's D7: a build script setting `cfg(test)` on its own library, as every build of it compiles it.
        ("bc/Cargo.toml", manifest("bc", "")),
        ("bc/build.rs", "fn main() {\n    println!(\"cargo::rustc-cfg=test\");\n}\n".to_owned()),
        ("bc/src/lib.rs", "#[cfg(test)]\n#[path = \"bc_only.rs\"]\npub mod r;\n".to_owned()),
        ("bc/src/bc_only.rs", module.clone()),
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
             pub mod block_star;\npub mod lrm;\npub mod mod_nonascii;\npub mod crlf_literal;\npub mod macro_named;\n"
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
        // Round 12's D5, D6: U+200E before `!`, a module named outside ASCII, `\\` before a CRLF in a `doc` literal.
        (
            "d/src/lrm.rs",
            "/// ```\n/// include_str\u{200e}!(\"oracle_doc_missing/lrm.rs\");\n/// ```\npub fn f() {}\n".to_owned(),
        ),
        (
            "d/src/mod_nonascii.rs",
            "/// ```\n/// #[path = \"oracle_doc_missing/mod_nonascii.rs\"]\n/// mod é;\n/// fn main() {}\n/// ```\npub fn f() {}\n"
                .to_owned(),
        ),
        (
            "d/src/crlf_literal.rs",
            "#[doc = \"```\\ninclude!(\\\"oracle_doc_missing/crlf_literal.rs\\\");\\n```\\\r\n\"]\npub fn f() {}\n".to_owned(),
        ),
        // Round 13's D1: a doc test's own macro, passed an inclusion's name apart from its `!`.
        (
            "d/src/macro_named.rs",
            "/// ```\n/// macro_rules! call {\n///     ($m:ident) => {\n///         $m!(\"oracle_doc_missing/macro_named.rs\");\n\
             ///     };\n/// }\n/// call!(include);\n/// ```\npub fn f() {}\n"
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
    // A corpus target's invocation whose dependency information is missing: none may be (round 12's R16).
    let mut unread: Vec<String> = Vec::new();
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
                unread.push(format!("{config:?}: `{}`", dep_info.display()));
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
    assert!(
        unread.is_empty(),
        "invocations with no dependency information: {unread:#?}"
    );
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
    ("lrm", "d/src/lrm.rs"),
    ("mod_nonascii", "d/src/mod_nonascii.rs"),
    ("crlf_literal", "d/src/crlf_literal.rs"),
    ("macro_named", "d/src/macro_named.rs"),
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
    "k/src/defaulted.txt",
    "bc/src/bc_only.rs",
];

/// The reference's lexical constructs, each a self-contained item or comment, which round 12's generated leg reads
/// before a trigger (round 12's D8): every kind of string — plain, raw with hashes, byte, C — holding quotes, comment
/// openers and escapes; a continuation; characters and byte characters holding quotes; lifetimes and a label; nested
/// block comments and line comments holding quotes; doc comments; a raw identifier; a macro's arguments; numbers.
const CONSTRUCTS: &[&str] = &[
    r###"const _: &str = "a\"b // /* ";"###,
    r###"const _: &str = r#"a"b // /* "#;"###,
    r###"const _: &str = r##"a"#b"##;"###,
    r###"const _: &[u8] = b"a\"b";"###,
    r###"const _: &[u8] = br#"a"b"#;"###,
    r###"const _: &core::ffi::CStr = c"a\"b";"###,
    r###"const _: &core::ffi::CStr = cr#"a"b"#;"###,
    r###"const _: char = '"';"###,
    r###"const _: char = '\'';"###,
    r###"const _: u8 = b'"';"###,
    r###"const _: u8 = b'\'';"###,
    r###"const _: char = '\u{22}';"###,
    r###"const _: &str = "\\";"###,
    "const _: &str = \"a\\\n    b\";",
    r###"const _: &str = "'";"###,
    r###"const _: fn(&'static str) = |_| {};"###,
    r###"const _: () = { 'a: loop { break 'a; } };"###,
    r###"/* a /* b " */ ' */"###,
    r###"/* a /* b */ " */"###,
    r###"const _: &str = "a\" /* \" */ ";"###,
    "// a \" ' /*",
    "/** a \" ' */\nconst _: () = ();",
    "/// a \" '\nconst _: () = ();",
    r###"const _: () = { let r#type = 1; let _ = r#type; };"###,
    r###"m!("a // b" /* c */ 'd' r#"e"#);"###,
    r###"const _: f64 = 1.0e10_f64;"###,
    r###"const _: u32 = 0x1f_u32;"###,
];

/// The trigger a generated case ends with, by its number and its module's prefix: an `include_str!`, an `include!`, a
/// `path` module and a module by its name, in turn — each naming a file of the case alone.
fn trigger(n: usize, prefix: char) -> (String, Vec<(String, String)>) {
    match n % 4 {
        0 => (
            format!("const _: &str = include_str!(\"g{n}.txt\");"),
            vec![(format!("gen/src/g{n}.txt"), String::new())],
        ),
        1 => (
            format!("include!(\"g{n}.rs\");"),
            vec![(format!("gen/src/g{n}.rs"), "const _: () = ();\n".to_owned())],
        ),
        2 => (
            format!("#[path = \"g{n}_p.rs\"]\nmod g{n}_p;"),
            vec![(format!("gen/src/g{n}_p.rs"), String::new())],
        ),
        _ => (
            format!("mod g{n}_m;"),
            vec![(format!("gen/src/{prefix}{n}/g{n}_m.rs"), String::new())],
        ),
    }
}

#[test]
fn the_census_reads_every_pairing_of_lexical_constructs_as_rustc_does() {
    crate::trust_test_process::bounded!(
        the_census_reads_every_pairing_of_lexical_constructs_as_rustc_does
    );
    // Round 12's change of method: the census's reading of where a literal or a comment ends is held to rustc's over
    // a generated corpus — every construct alone and every ordered pair of them, each case a module of one crate ending
    // in a trigger, which rustc reads unless a construct swallows it. Every file rustc reads, the census counts or
    // refuses a site in its case; a construct the census misreads past its end would hide the trigger.
    // Each construct alone, then every ordered pair with the trigger between them: a construct the census misreads past
    // its end swallows the trigger, and the next can close what it opened, so the misreading hides it without an error.
    let mut cases: Vec<(&str, &str)> = CONSTRUCTS.iter().map(|a| (*a, "")).collect();
    for a in CONSTRUCTS {
        for b in CONSTRUCTS {
            cases.push((a, b));
        }
    }
    let mut files: Vec<(String, String)> = vec![(
        "gen/src/lib.rs".to_owned(),
        format!(
            "#![allow(unused)]\nmacro_rules! m {{\n    ($($t:tt)*) => {{}};\n}}\n{}",
            (0..cases.len())
                .map(|n| format!("mod c{n};\n"))
                .collect::<String>()
        ),
    )];
    for (n, (a, b)) in cases.iter().enumerate() {
        let (live, made) = trigger(n, 'c');
        files.push((format!("gen/src/c{n}.rs"), format!("{a}\n{live}\n{b}\n")));
        files.extend(made);
    }
    let (read, census) = compiled("census-generated", &files);
    let mut missed = Vec::new();
    for (n, case) in cases.iter().enumerate() {
        let (_, made) = trigger(n, 'c');
        let case_file = format!("gen/src/c{n}.rs");
        for (file, _) in made {
            let refused = census.unfollowed.iter().any(|u| u.file == case_file);
            if read.contains(&file) && !census.files.contains(&file) && !refused {
                missed.push(format!("case {n}, {case:?}: rustc read `{file}`, which the census neither counts nor refuses"));
            }
        }
    }
    assert!(missed.is_empty(), "{missed:#?}");
    // Not vacuous: every case's trigger is live, so rustc read each case's file.
    let triggered = (0..cases.len())
        .filter(|&n| {
            trigger(n, 'c')
                .1
                .iter()
                .all(|(file, _)| read.contains(file))
        })
        .count();
    assert_eq!(triggered, cases.len(), "cases whose trigger rustc read");
}

/// What the pin's rustc reads compiling the crate `gen/src/lib.rs` of `files`, by its dependency information, and what
/// the census reads of it, as a crate rustdoc does not test.
fn compiled(name: &str, files: &[(String, String)]) -> (BTreeSet<String>, Reach) {
    compiled_in(name, "2021", files)
}

/// What the pin's rustc reads compiling the crate `gen/src/lib.rs` of `files` in `edition`, and what the census reads of
/// it — which reads no edition.
fn compiled_in(name: &str, edition: &str, files: &[(String, String)]) -> (BTreeSet<String>, Reach) {
    let named: Vec<(&str, String)> = files.iter().map(|(p, t)| (p.as_str(), t.clone())).collect();
    let f = crate::trust::tests::fixture(name, "1.95.0", &named);
    let repo = std::fs::canonicalize(&f.repo).expect("the corpus");
    let out = f.base.join("generated-out");
    std::fs::create_dir_all(&out).expect("the output");
    let rustc = Command::new("rustc")
        .args(["--edition", edition])
        .args([
            "--crate-type",
            "lib",
            "--crate-name",
            "gen",
            "--emit=dep-info,metadata",
        ])
        .arg("--out-dir")
        .arg(&out)
        .arg("gen/src/lib.rs")
        .env("RUSTUP_TOOLCHAIN", "1.95.0")
        .current_dir(&repo)
        .output()
        .expect("rustc");
    assert!(
        rustc.status.success(),
        "{}",
        String::from_utf8_lossy(&rustc.stderr)
    );
    let d = std::fs::read_to_string(out.join("gen.d")).expect("its dependency information");
    let read = prerequisites(&d)
        .iter()
        .filter_map(|p| {
            let c = std::fs::canonicalize(repo.join(p)).ok()?;
            Some(c.strip_prefix(&repo).ok()?.to_string_lossy().into_owned())
        })
        .collect();
    let tree = Tree::new(
        files
            .iter()
            .map(|(p, t)| (p.clone(), t.as_bytes().to_vec())),
    );
    let links = BTreeSet::new();
    let census = Sources::new(&tree, &links).reach("gen/src/lib.rs", false);
    (read, census)
}

/// The heads a file can open with (round 13's D2): a line rustc removes as a shebang — `#!` before anything but `[`,
/// past whitespace and plain comments, or a doc comment ending its look-ahead, whatever follows the comment — and an
/// inner attribute after whitespace and plain comments, which rustc keeps; a byte-order mark before either, which
/// rustc removes; a doc comment before an inner attribute.
const HEADS: &[&str] = &[
    "#!/x \"",
    "#!",
    "#!/** */[a = r#\"",
    "#!/*! */[a = r#\"",
    "#!/// d [a = r#\"",
    "#!//! d [a = r#\"",
    "#![allow(unused)]",
    "#! [allow(unused)]",
    "#!\t[allow(unused)]",
    "#!\n[allow(unused)]",
    "#!/* c */[allow(unused)]",
    "#!/**/[allow(unused)]",
    "#!/***/[allow(unused)]",
    "#!/* a /** b */ */[allow(unused)]",
    "#!// c\n[allow(unused)]",
    "#!//// c\n[allow(unused)]",
    "\u{feff}#![allow(unused)]",
    "\u{feff}#!/x",
    "//! d\n#![allow(unused)]",
    "/*! d */ #![allow(unused)]",
];

#[test]
fn the_census_reads_every_head_a_file_can_open_with_as_rustc_does() {
    crate::trust_test_process::bounded!(
        the_census_reads_every_head_a_file_can_open_with_as_rustc_does
    );
    // Round 13's D4: each head opens a module file, then each trigger in turn, then a raw string closing what a head
    // misread as an attribute opened — so rustc reads the trigger whatever the head, and a census misreading the head
    // hides it without an error.
    let count = HEADS.len() * 4;
    let mut files: Vec<(String, String)> = vec![(
        "gen/src/lib.rs".to_owned(),
        (0..count).map(|n| format!("mod h{n};\n")).collect(),
    )];
    for n in 0..count {
        let (live, made) = trigger(n, 'h');
        files.push((
            format!("gen/src/h{n}.rs"),
            format!("{}\n{live}\nconst _: &str = r#\"\n\"#;\n", HEADS[n / 4]),
        ));
        files.extend(made);
    }
    let (read, census) = compiled("census-heads", &files);
    let mut missed = Vec::new();
    for n in 0..count {
        let case_file = format!("gen/src/h{n}.rs");
        let refused = census.unfollowed.iter().any(|u| u.file == case_file);
        for (file, _) in trigger(n, 'h').1 {
            if read.contains(&file) && !census.files.contains(&file) && !refused {
                missed.push(format!(
                    "head {:?}, case {n}: rustc read `{file}`, which the census neither counts nor refuses",
                    HEADS[n / 4]
                ));
            }
        }
    }
    assert!(missed.is_empty(), "{missed:#?}");
    // Not vacuous: rustc read every case's trigger.
    let triggered = (0..count)
        .filter(|&n| {
            trigger(n, 'h')
                .1
                .iter()
                .all(|(file, _)| read.contains(file))
        })
        .count();
    assert_eq!(triggered, count, "cases whose trigger rustc read");
}

/// The spellings of a trigger through a macro (round 13's D1): forwarded whole, its name passed apart from its `!`, its
/// literal rewritten, its tokens apart by a separator, a body's own, a repetition's name, in std's macros that keep
/// their arguments in place, an attribute a metavariable fills, a macro a later edition's keyword names.
const SPELLINGS: usize = 9;

/// The kinds of trigger a macro may spell: an `include_str!`, an `include!`, a `path` module, a module by its name, a
/// debugger visualizer — each naming a file of the case alone.
const KINDS: usize = 5;

/// Case `n`'s module `m{n}`, spelling `n / KINDS` of kind `n % KINDS`, its macros named `f{n}` but spelling 8's, which
/// forwards a trigger whole or rewrites its literal through a macro named `gen`, a keyword from 2024 (round 14's D2).
fn spelt(n: usize) -> Option<(String, Vec<(String, String)>)> {
    match n / KINDS {
        8 => spelt_as(n, if n % KINDS < 2 { 2 } else { 0 }, "gen"),
        spelling => spelt_as(n, spelling, &format!("f{n}")),
    }
}

/// The names a later edition reserves that a 2015 macro may bear (round 14's D2): `try`, `async`, `await`, `dyn`.
const RESERVED_LATER: [&str; 4] = ["try", "async", "await", "dyn"];

/// Case `n` of the 2015 crate: through a macro named `RESERVED_LATER[n / KINDS]`, forwarding a trigger whole or
/// rewriting its literal; no module by its name, which 2015 declares in no non-mod-rs file.
fn spelt_2015(n: usize) -> Option<(String, Vec<(String, String)>)> {
    let name = RESERVED_LATER.get(n / KINDS)?;
    match n % KINDS {
        3 => None,
        kind => spelt_as(n, if kind < 2 { 2 } else { 0 }, name),
    }
}

/// Case `n`'s module `m{n}`, spelling `spelling` of kind `n % KINDS` through macros named `name`, and the files it makes
/// rustc read; none where the spelling cannot hold the kind.
fn spelt_as(n: usize, spelling: usize, name: &str) -> Option<(String, Vec<(String, String)>)> {
    let (live, made): (&str, (String, &str)) = match n % KINDS {
        0 => (
            "const _: &str = include_str!(\"g{n}.txt\");",
            (format!("gen/src/g{n}.txt"), ""),
        ),
        1 => (
            "include!(\"g{n}.rs\");",
            (format!("gen/src/g{n}.rs"), "const _: () = ();\n"),
        ),
        2 => (
            "#[path = \"g{n}_p.rs\"]\nmod g{n}_p;",
            (format!("gen/src/g{n}_p.rs"), ""),
        ),
        3 => ("mod g{n}_m;", (format!("gen/src/m{n}/g{n}_m.rs"), "")),
        _ => (
            "#[debugger_visualizer(gdb_script_file = \"g{n}.py\")]\nmod v{n} {}",
            (format!("gen/src/g{n}.py"), "# a debugger script\n"),
        ),
    };
    let live = live.replace("{n}", &n.to_string());
    let tokens: &[&str] = match n % KINDS {
        0 => &[
            "const",
            "_",
            ":",
            "&",
            "str",
            "=",
            "include_str",
            "!",
            "(\"g{n}.txt\")",
            ";",
        ],
        1 => &["include", "!", "(\"g{n}.rs\")", ";"],
        2 => &["#", "[path = \"g{n}_p.rs\"]", "mod", "g{n}_p", ";"],
        3 => &["mod", "g{n}_m", ";"],
        _ => &[
            "#",
            "[debugger_visualizer(gdb_script_file = \"g{n}.py\")]",
            "mod",
            "v{n}",
            "{}",
        ],
    };
    let apart = tokens.join(" sep ").replace("{n}", &n.to_string());
    let mut made = vec![(made.0, made.1.to_owned())];
    let code = match (spelling, n % KINDS) {
        (0, _) => format!("macro_rules! {name} {{\n    ($($t:tt)*) => {{ $($t)* }};\n}}\n{name}!({live});\n"),
        (1, 0) => format!(
            "macro_rules! {name} {{\n    ($m:ident) => {{ const _: &str = $m!(\"g{n}.txt\"); }};\n}}\n{name}!(include_str);\n"
        ),
        (1, 1) => format!("macro_rules! {name} {{\n    ($m:ident) => {{ $m!(\"g{n}.rs\"); }};\n}}\n{name}!(include);\n"),
        (1, 2) => format!(
            "macro_rules! {name} {{\n    ($m:tt) => {{ #[path = \"g{n}_p.rs\"] $m g{n}_p; }};\n}}\n{name}!(mod);\n"
        ),
        (1, 3) => format!("macro_rules! {name} {{\n    ($m:tt) => {{ $m g{n}_m; }};\n}}\n{name}!(mod);\n"),
        (1, _) => format!(
            "macro_rules! {name} {{\n    ($k:ident) => {{ #[debugger_visualizer($k = \"g{n}.py\")] mod v{n} {{}} }};\n}}\n\
             {name}!(gdb_script_file);\n"
        ),
        (2, 0) => format!(
            "macro_rules! {name} {{\n    ($a:tt $b:tt ($p:literal)) => {{ const _: &str = $a $b (concat!(\"g\", $p)); }};\n}}\n\
             {name}!(include_str!(\"{n}.txt\"));\n"
        ),
        (2, 1) => format!(
            "macro_rules! {name} {{\n    ($a:tt $b:tt ($p:literal)) => {{ $a $b (concat!(\"g\", $p)); }};\n}}\n\
             {name}!(include!(\"{n}.rs\"));\n"
        ),
        (3, _) => format!("macro_rules! {name} {{\n    ($($t:tt) sep *) => {{ $($t)* }};\n}}\n{name}!({apart});\n"),
        (4, _) => format!("macro_rules! {name} {{\n    () => {{ {live} }};\n}}\n{name}!();\n"),
        (5, 0) => format!(
            "macro_rules! {name} {{\n    ($($m:ident)?) => {{ const _: &str = $($m)?!(\"g{n}.txt\"); }};\n}}\n\
             {name}!(include_str);\n"
        ),
        (5, 1) => format!(
            "macro_rules! {name} {{\n    ($($m:ident)?) => {{ $($m)?!(\"g{n}.rs\"); }};\n}}\n{name}!(include);\n"
        ),
        (6, 0) => format!("pub fn f{n}() -> usize {{\n    format!(\"{{}}\", include_str!(\"g{n}.txt\")).len()\n}}\n"),
        (6, 1) => {
            made[0].1 = "true\n".to_owned();
            format!("pub fn f{n}() -> bool {{\n    assert!(include!(\"g{n}.rs\"));\n    true\n}}\n")
        }
        (6, 2) => format!(
            "pub fn f{n}() -> bool {{\n    assert!({{\n        #[path = \"g{n}_p.rs\"]\n        mod g{n}_p;\n        true\n    }});\n    \
             true\n}}\n"
        ),
        (7, 2) => format!(
            "macro_rules! {name} {{\n    ($a:meta) => {{ #[$a] mod g{n}_p; }};\n}}\n{name}!(path = \"g{n}_p.rs\");\n"
        ),
        (7, 4) => format!(
            "macro_rules! {name} {{\n    ($a:meta) => {{ #[$a] mod v{n} {{}} }};\n}}\n\
             {name}!(debugger_visualizer(gdb_script_file = \"g{n}.py\"));\n"
        ),
        _ => return None,
    };
    Some((format!("#![allow(unused)]\n{code}"), made))
}

#[test]
fn the_census_reads_every_macro_spelling_of_a_trigger_as_rustc_does() {
    crate::trust_test_process::bounded!(
        the_census_reads_every_macro_spelling_of_a_trigger_as_rustc_does
    );
    // Round 13's D4: every spelling of every kind of trigger through a macro, each a module of one crate. Every file
    // rustc reads, the census counts or refuses a site in its case.
    let cases: Vec<usize> = (0..SPELLINGS * KINDS)
        .filter(|&n| spelt(n).is_some())
        .collect();
    let mut files: Vec<(String, String)> = vec![(
        "gen/src/lib.rs".to_owned(),
        cases.iter().map(|n| format!("mod m{n};\n")).collect(),
    )];
    for &n in &cases {
        let (code, made) = spelt(n).expect("a case");
        files.push((format!("gen/src/m{n}.rs"), code));
        files.extend(made);
    }
    let (read, census) = compiled("census-macros", &files);
    let mut missed = Vec::new();
    for &n in &cases {
        let case_file = format!("gen/src/m{n}.rs");
        let refused = census.unfollowed.iter().any(|u| u.file == case_file);
        for (file, _) in spelt(n).expect("a case").1 {
            if read.contains(&file) && !census.files.contains(&file) && !refused {
                missed.push(format!(
                    "spelling {}, kind {}: rustc read `{file}`, which the census neither counts nor refuses",
                    n / KINDS,
                    n % KINDS
                ));
            }
        }
    }
    assert!(missed.is_empty(), "{missed:#?}");
    // Not vacuous: rustc read every case's files, and std's macros' inclusion is counted, not refused.
    let triggered = cases
        .iter()
        .filter(|&&n| {
            spelt(n)
                .expect("a case")
                .1
                .iter()
                .all(|(file, _)| read.contains(file))
        })
        .count();
    assert_eq!(triggered, cases.len(), "cases whose files rustc read");
    let kept = format!("gen/src/g{}.txt", 6 * KINDS);
    assert!(census.files.contains(&kept), "{kept}: {:?}", census.files);
}

#[test]
fn the_census_reads_every_macro_a_name_2018_reserves_bears_as_rustc_does() {
    crate::trust_test_process::bounded!(
        the_census_reads_every_macro_a_name_2018_reserves_bears_as_rustc_does
    );
    // Round 14's D2: the names 2018 reserves, each a 2015 macro's, in a crate of that edition, forwarding a trigger or
    // rewriting its literal. Every file rustc reads, the census counts or refuses a site in its case; and rustc reads
    // each case's files.
    let cases: Vec<usize> = (0..RESERVED_LATER.len() * KINDS)
        .filter(|&n| spelt_2015(n).is_some())
        .collect();
    let mut files: Vec<(String, String)> = vec![(
        "gen/src/lib.rs".to_owned(),
        cases.iter().map(|n| format!("mod m{n};\n")).collect(),
    )];
    for &n in &cases {
        let (code, made) = spelt_2015(n).expect("a case");
        files.push((format!("gen/src/m{n}.rs"), code));
        files.extend(made);
    }
    let (read, census) = compiled_in("census-macros-2015", "2015", &files);
    let mut missed = Vec::new();
    for &n in &cases {
        let case_file = format!("gen/src/m{n}.rs");
        let refused = census.unfollowed.iter().any(|u| u.file == case_file);
        for (file, _) in spelt_2015(n).expect("a case").1 {
            if !read.contains(&file) {
                missed.push(format!(
                    "`{}`, kind {}: rustc did not read `{file}`",
                    RESERVED_LATER[n / KINDS],
                    n % KINDS
                ));
            } else if !census.files.contains(&file) && !refused {
                missed.push(format!(
                    "`{}`, kind {}: rustc read `{file}`, which the census neither counts nor refuses",
                    RESERVED_LATER[n / KINDS],
                    n % KINDS
                ));
            }
        }
    }
    assert!(missed.is_empty(), "{missed:#?}");
}

/// Case `n`'s module declaration (round 13's D3): in a file of one kind — the crate root, a non-mod-rs file, a mod-rs
/// file, a file a `path` loads, a file an `include!` brings in — under one chain of inline modules, each with or
/// without a `path`, or a block (round 14's D1), a child with or without its own: the files written, the child's file
/// under every directory any reading could seek it in, and what the crate root holds of it; none for a child with no
/// `path` in a block, which rustc refuses.
fn declared(n: usize) -> Option<(Vec<(String, String)>, String)> {
    let kind = n / (CHAINS.len() * 2);
    let chain = CHAINS[(n / 2) % CHAINS.len()];
    let with_path = n % 2 == 1;
    if chain.contains('b') && !with_path {
        return None;
    }
    let child = if with_path {
        format!("#[path = \"q{n}.rs\"]\npub mod c{n};\n")
    } else {
        format!("pub mod c{n};\n")
    };
    let mut body = child;
    for (k, level) in chain
        .chars()
        .enumerate()
        .collect::<Vec<_>>()
        .into_iter()
        .rev()
    {
        body = match level {
            'b' => format!("pub fn b{n}_{k}() {{\n{body}}}\n"),
            'p' => format!("#[path = \"p{n}_{k}\"]\npub mod a{n}_{k} {{\n{body}}}\n"),
            _ => format!("pub mod a{n}_{k} {{\n{body}}}\n"),
        };
    }
    let (file, root) = match kind {
        0 => (None, body.clone()),
        1 => (Some(format!("gen/src/f{n}.rs")), format!("pub mod f{n};\n")),
        2 => (
            Some(format!("gen/src/f{n}/mod.rs")),
            format!("pub mod f{n};\n"),
        ),
        3 => (
            Some(format!("gen/src/lp/f{n}.rs")),
            format!("#[path = \"lp/f{n}.rs\"]\npub mod f{n};\n"),
        ),
        _ => (
            Some(format!("gen/src/inc/f{n}.rs")),
            format!("include!(\"inc/f{n}.rs\");\n"),
        ),
    };
    let at = file.clone().unwrap_or_else(|| "gen/src/lib.rs".to_owned());
    let dir = at.rsplit_once('/').map_or("", |(d, _)| d).to_owned();
    let stem = at
        .rsplit('/')
        .next()
        .unwrap_or_default()
        .trim_end_matches(".rs")
        .to_owned();
    let mut dirs = vec![dir.clone(), format!("{dir}/{stem}")];
    for (k, level) in chain.chars().enumerate().filter(|(_, l)| *l != 'b') {
        dirs = dirs
            .iter()
            .flat_map(|d| {
                let mut next = vec![format!("{d}/a{n}_{k}")];
                if level == 'p' {
                    next.push(format!("{d}/p{n}_{k}"));
                }
                next
            })
            .collect();
    }
    let leaf = if with_path {
        format!("q{n}.rs")
    } else {
        format!("c{n}.rs")
    };
    let mut files: Vec<(String, String)> = dirs
        .iter()
        .map(|d| (format!("{d}/{leaf}"), String::new()))
        .collect();
    if let Some(file) = file {
        files.push((file, body));
    }
    Some((files, root))
}

/// The chains of `declared`, outermost first: `n` an inline module, `p` one with a `path`, `b` a block, a function's
/// body (round 14's D1).
const CHAINS: [&str; 13] = [
    "", "n", "p", "nn", "np", "pn", "pp", "b", "bn", "bp", "nbn", "bnn", "pbn",
];

#[test]
fn the_census_reads_every_inline_module_s_children_where_rustc_does() {
    crate::trust_test_process::bounded!(
        the_census_reads_every_inline_module_s_children_where_rustc_does
    );
    // Round 13's D3 and D4: every kind of file, every chain of up to two inline modules, each with or without a `path`,
    // a child with or without its own — the child's file written under every directory any reading could seek it in,
    // so rustc reads one and the census must count that one; and none of these plain spellings refused. Round 14's D1:
    // and a block among them, which takes no non-mod-rs module's name.
    let cases: Vec<usize> = (0..5 * CHAINS.len() * 2)
        .filter(|&n| declared(n).is_some())
        .collect();
    let mut lib = String::new();
    let mut files: Vec<(String, String)> = Vec::new();
    for &n in &cases {
        let (written, root) = declared(n).expect("a case");
        lib.push_str(&root);
        files.extend(written);
    }
    files.push(("gen/src/lib.rs".to_owned(), lib));
    let (read, census) = compiled("census-paths", &files);
    assert!(census.unfollowed.is_empty(), "{:?}", census.unfollowed);
    let missed: Vec<&String> = read.iter().filter(|f| !census.files.contains(*f)).collect();
    assert!(
        missed.is_empty(),
        "rustc read what the census does not count: {missed:#?}"
    );
    // Not vacuous: rustc read every case's child.
    let children = cases
        .iter()
        .copied()
        .filter(|&n| {
            let leaf = if n % 2 == 1 {
                format!("/q{n}.rs")
            } else {
                format!("/c{n}.rs")
            };
            read.iter().any(|f| f.ends_with(&leaf))
        })
        .count();
    assert_eq!(children, cases.len(), "cases whose child rustc read");
}
