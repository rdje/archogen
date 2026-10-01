//! §3's package rules beyond the sets (`M2.7.3.7`).
//!
//! The refused words are typed here from `docs/specs/catalog/decision_catalog-records-hashes.md`, not taken from
//! the crate, so a word the crate drops is caught.

mod common;

use archogen_catalog::hash::Catalog;
use archogen_catalog::manifest::parse;
use archogen_catalog::package::{check_config, check_manifest, check_workspace, scan};
use archogen_catalog::tree::Tree;
use archogen_catalog::Code;
use common::*;

/// §3: refused anywhere as tokens.
const ANYWHERE: [&str; 15] = [
    "asm",
    "global_asm",
    "naked_asm",
    "include",
    "include_str",
    "include_bytes",
    "debugger_visualizer",
    "no_mangle",
    "export_name",
    "link_section",
    "panic_handler",
    "global_allocator",
    "alloc_error_handler",
    "macro_rules",
    "macro",
];

fn refused(source: &str) -> bool {
    scan(source.as_bytes()).is_err()
}

#[test]
fn a_refused_word_is_refused_as_a_token_and_nowhere_else() {
    for word in ANYWHERE {
        assert!(
            refused(&format!("fn f() {{ {word}; }}\n")),
            "`{word}` as a token"
        );
        assert!(
            refused(&format!("use core::arch::{word} as x;\n")),
            "`{word}` in a path"
        );
        if word != "macro" {
            assert!(
                refused(&format!("fn f() {{ r#{word}; }}\n")),
                "`r#{word}`, compared by its name"
            );
        }
        for admitted in [
            format!("// {word}\nfn f() {{}}\n"),
            format!("/* {word} /* nested {word} */ */ fn f() {{}}\n"),
            format!("/// {word}\nfn f() {{}}\n"),
            format!("const S: &str = \"{word}\";\n"),
            format!("const S: &str = r#\"{word} \"quoted\" \"#;\n"),
            format!("const S: &[u8] = b\"{word}\";\n"),
            format!("const S: &[u8] = br\"{word}\";\n"),
            format!("const S: &str = \"escaped \\\" {word}\";\n"),
            format!("/* a /* b */ {word} */ fn f() {{}}\n"),
            format!("const S: &str = r#\"a \" {word} \"#;\n"),
        ] {
            assert!(!refused(&admitted), "`{word}` in {admitted:?}");
        }
    }
    let found = scan(b"fn f() {}\n\n   asm!(\"nop\");\n").unwrap_err();
    assert_eq!((found.line, found.column), (3, 4), "{found:?}");
}

#[test]
fn path_link_and_used_are_refused_only_where_they_could_become_an_attribute() {
    for word in ["path", "link", "used"] {
        for place in [
            format!("#[{word} = \"x.rs\"]\nmod m;\n"),
            format!("#![{word}]\n"),
            format!("#[cfg_attr(unix, {word})]\nstatic S: u8 = 0;\n"),
            format!("#[unsafe({word}(name = \"c\"))]\nfn f() {{}}\n"),
            format!("fn f() {{ m!({word}); }}\n"),
            format!("fn f() {{ m![a, ({word})]; }}\n"),
            format!("fn f() {{ m! {{ {{ {word} }} }}; }}\n"),
        ] {
            assert!(refused(&place), "`{word}` in {place:?}");
        }
        for place in [
            format!("fn f() {{ let {word} = 1; }}\n"),
            format!("fn {word}() {{}}\n"),
            format!("fn f(x: bool) {{ if !({word}(x)) {{}} }}\n"),
            format!("fn f(a: u8) -> bool {{ a != ({word}) }}\n"),
            format!("#[test]\nfn f() {{ let {word} = [0]; }}\n"),
            format!("fn f(r#type: &[u8], {word}: usize) -> u8 {{ r#type[{word}] }}\n"),
        ] {
            assert!(!refused(&place), "`{word}` in {place:?}");
        }
    }
}

#[test]
fn extern_is_admitted_only_as_extern_crate_or_an_extern_fn_outside_a_macro() {
    for admitted in [
        "extern crate alloc;\n",
        "extern \"C\" fn f() {}\n",
        "pub unsafe extern \"C\" fn f() {}\n",
        "extern fn f() {}\n",
        "type F = extern \"C\" fn(u8) -> u8;\n",
    ] {
        assert!(!refused(admitted), "{admitted:?}");
    }
    for refused_source in [
        "extern \"C\" { fn x(); }\n",
        "extern { fn x(); }\n",
        "unsafe extern \"C\" { fn x(); }\n",
        "m!(extern \"C\" fn f() {});\n",
        "m!(extern { fn x(); });\n",
    ] {
        assert!(refused(refused_source), "{refused_source:?}");
    }
}

#[test]
fn an_identifier_outside_ascii_is_refused_and_other_text_outside_ascii_is_not() {
    assert!(refused("fn f() { let é = 1; }\n"));
    assert!(
        refused("fn ﬁx() {}\n"),
        "a ligature that normalizes to `fix`"
    );
    assert!(refused("fn f<'é>() {}\n"), "a lifetime outside ASCII");
    for admitted in [
        "// é\nfn f() {}\n",
        "const S: &str = \"é\";\n",
        "const C: char = 'é';\n",
        "fn f<'a>(x: &'a u8) -> char { let _ = x; '\\'' }\n",
        "fn f() { 'outer: loop { break 'outer; } }\n",
        "const B: u8 = b'x';\n",
        "fn f() -> u64 { 0x28 + 1_000u64 + 2 }\n",
    ] {
        assert!(!refused(admitted), "{admitted:?}");
    }
    assert!(
        refused("const S: &str = \"unclosed;\n"),
        "a string never closed"
    );
    assert!(refused("/* unclosed\n"), "a comment never closed");
}

#[test]
fn rt_core_passes_as_the_record_measured() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../rt-core");
    let mut scanned = 0;
    for entry in std::fs::read_dir(root.join("src")).unwrap() {
        let path = entry.unwrap().path();
        if path.extension().is_some_and(|e| e == "rs") {
            let bytes = std::fs::read(&path).unwrap();
            scan(&bytes).unwrap_or_else(|f| panic!("{}: {f:?}", path.display()));
            scanned += 1;
        }
    }
    assert!(scanned >= 3, "rt-core's sources");
    let manifest = std::fs::read_to_string(root.join("Cargo.toml")).unwrap();
    let m = parse(&manifest).unwrap();
    check_manifest(
        &Tree::default(),
        "crates/rt-core",
        "crates/rt-core/Cargo.toml",
        &m,
    )
    .unwrap();
    let ws = std::fs::read_to_string(root.join("../../Cargo.toml")).unwrap();
    check_workspace("Cargo.toml", &parse(&ws).unwrap()).unwrap();
    let config = std::fs::read_to_string(root.join("../../.cargo/config.toml")).unwrap();
    check_config(".cargo/config.toml", &parse(&config).unwrap()).unwrap();
}

fn manifest(text: &str) -> Result<(), String> {
    let m = parse(text).unwrap_or_else(|e| panic!("{e:?}"));
    check_manifest(&Tree::default(), "crates/p", "crates/p/Cargo.toml", &m)
}

#[test]
fn a_manifest_refuses_what_would_build_another_configuration_or_crate() {
    let base = "[package]\nname = \"p\"\n";
    manifest(base).unwrap();
    manifest(&format!("{base}build = false\n")).unwrap();
    for (extra, why) in [
        ("build = \"gen.rs\"\n", "a build script"),
        ("workspace = \"..\"\n", "package.workspace"),
        ("[lib]\nproc-macro = true\n", "proc-macro"),
        (
            "[lib]\nproc_macro = false\n",
            "proc_macro, whatever its value",
        ),
        ("[lib]\ncrate-type = [\"cdylib\"]\n", "crate-type"),
        (
            "[[bin]]\nname = \"b\"\ncrate_type = [\"bin\"]\n",
            "crate_type in an array of tables",
        ),
        ("[features]\ndefault = []\n", "[features]"),
        (
            "[dependencies]\nq = { path = \"../q\", optional = true }\n",
            "an optional dependency",
        ),
        (
            "[dependencies.q]\npath = \"../q\"\noptional = true\n",
            "an optional dependency, as a table",
        ),
        (
            "[lib]\npath = \"../other/lib.rs\"\n",
            "a target path leaving the package",
        ),
        (
            "[[bin]]\nname = \"b\"\npath = \"../../b.rs\"\n",
            "a binary's path leaving the package",
        ),
        (
            "[[test]]\nname = \"t\"\npath = \"/abs.rs\"\n",
            "an absolute path",
        ),
        (
            "[[example]]\nname = \"e\"\npath = \"../e.rs\"\n",
            "an example's path leaving the package",
        ),
        (
            "[[bench]]\nname = \"b\"\npath = \"../b.rs\"\n",
            "a bench's path leaving the package",
        ),
        ("[lib.proc-macro]\n", "proc-macro as a table header alone"),
        ("[lib]\npath = 3\n", "a target path that is not a string"),
        ("[profile.dev]\nrustflags = [\"-C\"]\n", "rustflags"),
    ] {
        let text = format!("{base}{extra}");
        assert!(manifest(&text).is_err(), "{why}: {text:?}");
    }
    assert!(
        manifest(&format!("cargo-features = [\"x\"]\n{base}")).is_err(),
        "cargo-features"
    );
    manifest(&format!(
        "{base}[dependencies]\nq = {{ path = \"../q\", optional = false }}\n"
    ))
    .unwrap();
    manifest(&format!("{base}[lib]\npath = \"src/other.rs\"\n")).unwrap();
    manifest(&format!(
        "{base}[[bin]]\nname = \"b\"\npath = \"src/bin/b.rs\"\n"
    ))
    .unwrap();
    let mut with_build_rs = Tree::default();
    with_build_rs.insert("crates/p/build.rs", b"fn main() {}\n".to_vec());
    let m = parse(base).unwrap();
    assert!(
        check_manifest(&with_build_rs, "crates/p", "crates/p/Cargo.toml", &m).is_err(),
        "build.rs"
    );
}

#[test]
fn a_workspace_and_a_config_refuse_what_reaches_every_package() {
    let ws = "[workspace]\nmembers = [\"crates/*\"]\n";
    check_workspace("Cargo.toml", &parse(ws).unwrap()).unwrap();
    for extra in [
        "[patch.crates-io]\nq = { path = \"q\" }\n",
        "[replace]\nq = { path = \"q\" }\n",
        "[profile.release]\nrustflags = [\"-C\"]\n",
    ] {
        let text = format!("{ws}{extra}");
        assert!(
            check_workspace("Cargo.toml", &parse(&text).unwrap()).is_err(),
            "{text:?}"
        );
    }
    let text = format!("cargo-features = [\"x\"]\n{ws}");
    assert!(
        check_workspace("Cargo.toml", &parse(&text).unwrap()).is_err(),
        "cargo-features"
    );
    for admitted in [
        "[alias]\nb = \"build\"\n",
        "alias.b = \"build\"\n",
        "alias = { b = \"build\" }\n",
    ] {
        check_config(".cargo/config.toml", &parse(admitted).unwrap())
            .unwrap_or_else(|e| panic!("{admitted:?}: {e}"));
    }
    for config in [
        "[build]\nrustflags = [\"-C\"]\n",
        "build.target = \"x\"\n",
        "[env]\n",
        "[alias]\nb = \"build\"\n[target.x]\nlinker = \"y\"\n",
    ] {
        assert!(
            check_config(".cargo/config.toml", &parse(config).unwrap()).is_err(),
            "{config:?}"
        );
    }
}

#[test]
fn the_hashes_refuse_a_package_a_record_reaches_that_breaks_a_rule() {
    let files = package();
    let record = packaged("example.base", "p");
    let catalog = |lib: &str, config: Option<&str>| {
        let mut t = tree(&[&record], &files);
        t.insert("crates/p/src/lib.rs", lib.as_bytes().to_vec());
        if let Some(c) = config {
            t.insert(".cargo/config.toml", c.as_bytes().to_vec());
        }
        Catalog::read(t).unwrap()
    };
    catalog("//! p\n", None)
        .hashes()
        .unwrap_or_else(|e| panic!("{e}"));
    let r = catalog("//! p\nfn f() { core::arch::asm!(\"nop\"); }\n", None)
        .hashes()
        .unwrap_err();
    assert_eq!(r.code, Code::Source, "{r}");
    assert!(
        r.message.contains("crates/p/src/lib.rs") && r.message.contains("`asm`"),
        "{r}"
    );
    let r = catalog("//! p\n", Some("[build]\nrustflags = [\"-C\"]\n"))
        .hashes()
        .unwrap_err();
    assert_eq!(r.code, Code::Source, "{r}");
    let mut t = tree(&[&record], &files);
    t.insert(
        "Cargo.toml",
        b"[workspace]\nmembers = [\"crates/*\"]\n[patch.crates-io]\nq = { path = \"q\" }\n"
            .to_vec(),
    );
    assert_eq!(
        Catalog::read(t).unwrap().hashes().unwrap_err().code,
        Code::Source
    );
    let mut t = tree(&[&record], &files);
    t.insert(
        "crates/p/Cargo.toml",
        b"[package]\nname = \"p\"\n[features]\n".to_vec(),
    );
    assert_eq!(
        Catalog::read(t).unwrap().hashes().unwrap_err().code,
        Code::Source
    );
}
