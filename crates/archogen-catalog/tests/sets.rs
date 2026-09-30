//! §3's source sets, reached sets and derived lines, beyond what the worked example exercises (`M2.7.3.2`).
//!
//! A small workspace of packages, a ledger, a target and a chain of records. Each kind of hash input is changed in
//! turn, and the test asserts exactly which hashes move, so an input left out of a hash turns a test red. Then every
//! refusal met while computing the hashes.

use archogen_catalog::hash::{Catalog, Hashes};
use archogen_catalog::record::{read_record, FacetKind};
use archogen_catalog::tree::Tree;
use archogen_catalog::{Code, Refusal};

const LEDGER: &str = "# Ledger\n\n## `alpha`\n\nthe alpha source\n\n## `beta`\n\nthe beta source\n";

fn tree() -> Tree {
    Tree::new(
        [
            ("Cargo.toml", "[workspace]\nmembers = [\"crates/*\"]\nexclude = [\"crates/skip\"]\n"),
            (".cargo/config.toml", "[alias]\nx = \"run -p xtask --\"\n"),
            ("rust-toolchain.toml", "[toolchain]\nchannel = \"1.95.0\"\n"),
            (
                "crates/a/Cargo.toml",
                "[package]\nname = \"a\"\n\n[dependencies]\nb = { path = \"../b\" }\n\n[dev-dependencies]\nt = { path = \"../t\" }\n",
            ),
            ("crates/a/src/lib.rs", "pub fn a() {}\n"),
            ("crates/b/Cargo.toml", "[package]\nname = \"b\"\n\n[dependencies]\ne = { path = \"../e\" }\n"),
            ("crates/b/src/lib.rs", "pub fn b() {}\n"),
            ("crates/e/Cargo.toml", "[package]\nname = \"e\"\n"),
            ("crates/e/src/lib.rs", "pub fn e() {}\n"),
            ("crates/t/Cargo.toml", "[package]\nname = \"t\"\n"),
            ("crates/t/src/lib.rs", "pub fn t() {}\n"),
            ("crates/c/Cargo.toml", "[package]\nname = \"c\"\n"),
            ("crates/c/src/lib.rs", "pub fn c() {}\n"),
            ("docs/models/m.txt", "a model\n"),
            ("docs/models/n.txt", "another\n"),
            ("docs/book/src/ledger.md", LEDGER),
            (
                "targets/board.env",
                "TARGET_KIND=board\nRUST_TARGET=riscv64gc-unknown-none-elf\nTARGET_FILE=docs/board/memory.x\n",
            ),
            ("targets/board.eadl", "(platform)\n"),
            ("docs/board/memory.x", "MEMORY\n"),
        ]
        .map(|(p, b)| (p.to_owned(), b.as_bytes().to_vec())),
    )
}

/// `lib.a` owns a package, depends on `lib.c`, describes it, and is measured with `lib.m`. `lib.c`
/// depends on `lib.d`, which depends on `lib.e`, and `lib.m` on `lib.n`: chains, so closures are transitive.
fn records() -> Vec<(String, String)> {
    let a = r#"(catalog-record lib.a
  (version "1.0.0") (catalog algorithms) (source (origin "a test") (license "MIT")) (maintainer M2)
  (depends (lib.c "1.0")) (supersedes) (profiles rt-static-up-v1) (targets board)
  (preconditions) (guarantees "it works")
  (implementation (version "1.0.0") (sources "crates/a"))
  (behavior-model (version "1.0.0") (sources "docs/models/m.txt") (describes lib.c)
    (facts (fact f yes (locator (code lib.a "crates/a/src/lib.rs")) (basis "see the code"))
           (fact g yes (locator (ledger alpha "r1, the alpha section")) (basis "the ledger"))
           (fact h yes (locator (file "docs/models/m.txt")) (basis "the model"))))
  (timing-model (version "1.0.0") (sources "docs/models/n.txt") (measured-with lib.m) (facts)
    (costs (cost k (target board) (value 3) (unit ns) (scope "x") (holds-for (tasks 1) (sources 0))
             (holds-under-preemption yes) (binary unbuilt) (evidence analytically-established)
             (locator (code lib.c "crates/c/src/lib.rs")) (basis "argued")))))
"#;
    let c = r#"(catalog-record lib.c
  (version "1.0.0") (catalog algorithms) (source (origin "a test") (license "MIT")) (maintainer M2)
  (depends (lib.d "1.0")) (supersedes) (profiles rt-static-up-v1) (targets any)
  (preconditions) (guarantees "it works too")
  (implementation (version "1.0.0") (sources "crates/c"))
  (behavior-model (version "1.0.0") (none "none"))
  (timing-model (version "1.0.0") (none "none")))
"#;
    let bare = |id: &str, depends: &str| {
        format!(
            r#"(catalog-record {id}
  (version "1.0.0") (catalog algorithms) (source (origin "a test") (license "MIT")) (maintainer M2)
  (depends {depends}) (supersedes) (profiles rt-static-up-v1) (targets board)
  (preconditions) (guarantees "a link in a chain")
  (implementation (version "1.0.0") (none "none"))
  (behavior-model (version "1.0.0") (none "none"))
  (timing-model (version "1.0.0") (none "none")))
"#
        )
    };
    vec![
        ("lib.a".into(), a.into()),
        ("lib.c".into(), c.into()),
        ("lib.d".into(), bare("lib.d", "(lib.e \"1.0\")")),
        ("lib.e".into(), bare("lib.e", "")),
        ("lib.m".into(), bare("lib.m", "(lib.n \"1.0\")")),
        ("lib.n".into(), bare("lib.n", "")),
    ]
}

fn catalog(tree: Tree, records: &[(String, String)]) -> Catalog {
    let read = |id: &str, text: &str| {
        read_record(
            &format!("catalog/experimental/{id}.catalog"),
            text.as_bytes(),
        )
        .unwrap_or_else(|e| panic!("{e}"))
    };
    Catalog::new(tree, records.iter().map(|(id, text)| read(id, text)))
}

fn hashes(tree: Tree, records: &[(String, String)]) -> Hashes {
    catalog(tree, records)
        .hashes()
        .unwrap_or_else(|e| panic!("{e}"))
}

/// Which `(record, facet, own|bound)` hashes differ between two computations, sorted.
fn moved(a: &Hashes, b: &Hashes) -> Vec<String> {
    let mut out = Vec::new();
    for (key, x) in &a.facets {
        let y = &b.facets[key];
        if x.own != y.own {
            out.push(format!("{} {} own", key.0, key.1.as_str()));
        }
        if x.bound != y.bound {
            out.push(format!("{} {} bound", key.0, key.1.as_str()));
        }
    }
    out.sort();
    out
}

fn with(path: &str, bytes: &str) -> Tree {
    let mut t = tree();
    t.insert(path, bytes.as_bytes().to_vec());
    t
}

fn sorted(items: &[&str]) -> Vec<String> {
    let mut v: Vec<String> = items.iter().map(|s| (*s).to_owned()).collect();
    v.sort();
    v
}

/// The second field of every derived line of one kind.
fn named<'a>(lines: &'a [String], kind: &str) -> Vec<&'a str> {
    lines
        .iter()
        .filter(|l| l.split(' ').next() == Some(kind))
        .map(|l| l.split(' ').nth(1).unwrap())
        .collect()
}

#[test]
fn the_sets_of_a_package_and_what_it_reaches() {
    let h = hashes(tree(), &records());
    let imp = h.facet("lib.a", FacetKind::Implementation).unwrap();
    assert_eq!(imp.own_set, ["crates/a/Cargo.toml", "crates/a/src/lib.rs"]);
    assert_eq!(
        imp.reached,
        [
            ".cargo/config.toml",
            "Cargo.toml",
            "crates/b/Cargo.toml",
            "crates/b/src/lib.rs",
            "crates/e/Cargo.toml",
            "crates/e/src/lib.rs",
            "rust-toolchain.toml",
        ],
        "`b`, `b`'s own dependency `e`, the workspace manifest, the configuration and toolchain files; not the dev dependency `t`"
    );
    let model = h.facet("lib.a", FacetKind::BehaviorModel).unwrap();
    assert_eq!(named(&model.derived, "contract"), ["lib.a"]);
    assert_eq!(
        named(&model.derived, "implementation"),
        ["lib.a", "lib.c"],
        "its own, and its `describes` record's"
    );
    assert_eq!(
        named(&model.derived, "behavior-model"),
        ["lib.c"],
        "each direct dependency's model"
    );
    assert_eq!(named(&model.derived, "ledger"), ["alpha"]);
    assert!(
        named(&model.derived, "target").contains(&"docs/board/memory.x"),
        "a path the `.env` gives is a target file"
    );
    let timing = h.facet("lib.a", FacetKind::TimingModel).unwrap();
    assert_eq!(
        named(&timing.derived, "implementation"),
        ["lib.a", "lib.c", "lib.d", "lib.e", "lib.m", "lib.n"],
        "its own, its dependency closure, each `measured-with` record, and that record's closure"
    );
    assert_eq!(
        named(&timing.derived, "timing-model"),
        ["lib.c", "lib.m"],
        "each direct dependency's and `measured-with` record's"
    );
    let contract = h.facet("lib.a", FacetKind::Contract).unwrap();
    assert_eq!(contract.derived.len(), 1);
    assert_eq!(named(&contract.derived, "contract"), ["lib.c"]);
}

#[test]
fn each_input_moves_exactly_the_hashes_it_is_in() {
    let base = hashes(tree(), &records());
    let ledger_alpha = LEDGER.replace("the alpha source", "the alpha source, revised");
    let ledger_beta = LEDGER.replace("the beta source", "the beta source, revised");
    let cases: Vec<(&str, Tree, Vec<String>)> = vec![
        (
            "a file of the own set",
            with("crates/a/src/lib.rs", "pub fn a() { 1; }\n"),
            sorted(&[
                "lib.a implementation own",
                "lib.a implementation bound",
                "lib.a behavior-model bound",
                "lib.a timing-model bound",
            ]),
        ),
        (
            "a file a dependency's dependency holds",
            with("crates/e/src/lib.rs", "pub fn e() { 2; }\n"),
            sorted(&[
                "lib.a implementation bound",
                "lib.a behavior-model bound",
                "lib.a timing-model bound",
            ]),
        ),
        (
            "the workspace manifest",
            with(
                "Cargo.toml",
                "[workspace]\nmembers = [\"crates/*\"]\nexclude = [\"crates/skip\", \"x\"]\n",
            ),
            sorted(&[
                "lib.a implementation bound",
                "lib.a behavior-model bound",
                "lib.a timing-model bound",
                "lib.c implementation bound",
                "lib.c behavior-model bound",
                "lib.c timing-model bound",
            ]),
        ),
        (
            "a model's own file",
            with("docs/models/m.txt", "a model, changed\n"),
            sorted(&["lib.a behavior-model own", "lib.a behavior-model bound"]),
        ),
        (
            "a target's file",
            with("docs/board/memory.x", "MEMORY 2\n"),
            sorted(&[
                "lib.a behavior-model bound",
                "lib.a timing-model bound",
                "lib.c behavior-model bound",
                "lib.d behavior-model bound",
                "lib.e behavior-model bound",
                "lib.m behavior-model bound",
                "lib.n behavior-model bound",
            ]),
        ),
        (
            "the ledger section a locator names",
            with("docs/book/src/ledger.md", &ledger_alpha),
            sorted(&["lib.a behavior-model bound"]),
        ),
        (
            "another ledger section",
            with("docs/book/src/ledger.md", &ledger_beta),
            Vec::new(),
        ),
    ];
    for (what, tree, want) in cases {
        assert_eq!(
            moved(&base, &hashes(tree, &records())),
            want,
            "changing {what}"
        );
    }
}

#[test]
fn a_dependency_and_a_measured_record_reach_their_dependents() {
    let base = hashes(tree(), &records());
    let mut changed = records();
    changed[1].1 = changed[1]
        .1
        .replace("\"it works too\"", "\"it works, and more\"");
    let got = moved(&base, &hashes(tree(), &changed));
    for want in [
        "lib.c contract own",
        "lib.c contract bound",
        "lib.a contract bound",
        "lib.a behavior-model bound",
        "lib.a timing-model bound",
    ] {
        assert!(got.contains(&want.to_owned()), "{want} in {got:?}");
    }
    let got = moved(
        &base,
        &hashes(
            with("crates/c/src/lib.rs", "pub fn c() { 3; }\n"),
            &records(),
        ),
    );
    for want in [
        "lib.c implementation bound",
        "lib.a behavior-model bound",
        "lib.a timing-model bound",
    ] {
        assert!(
            got.contains(&want.to_owned()),
            "a `describes` and `measured-with` record's code: {want} in {got:?}"
        );
    }
}

#[test]
fn the_two_models_are_independent_and_reviews_and_namespaces_move_nothing() {
    let base = hashes(tree(), &records());
    let mut timing = records();
    timing[0].1 = timing[0].1.replace("(value 3)", "(value 4)");
    assert_eq!(
        moved(&base, &hashes(tree(), &timing)),
        ["lib.a timing-model bound", "lib.a timing-model own"]
    );
    let mut behavior = records();
    behavior[0].1 = behavior[0]
        .1
        .replace("(basis \"the model\")", "(basis \"the model, again\")");
    assert_eq!(
        moved(&base, &hashes(tree(), &behavior)),
        ["lib.a behavior-model bound", "lib.a behavior-model own"]
    );
    let mut reviewed = records();
    let review = format!(
        " (review (facet contract) (hash \"sha256:{}\") (verdict rejected) (by director \"d\") (date \"2026-09-30\") (basis \"no\")))\n",
        "a".repeat(64)
    );
    reviewed[1].1 = reviewed[1]
        .1
        .trim_end()
        .strip_suffix(')')
        .unwrap()
        .to_owned()
        + &review;
    let r = hashes(tree(), &reviewed);
    assert!(moved(&base, &r).is_empty(), "a review is in no facet hash");
    assert_eq!(base.records, r.records);
    let experimental = catalog(tree(), &records());
    let production = read_record(
        "catalog/production/lib.c.catalog",
        records()[1].1.as_bytes(),
    )
    .unwrap();
    let others = experimental
        .records
        .values()
        .filter(|r| r.id != "lib.c")
        .cloned();
    let promoted = Catalog::new(tree(), others.chain([production]))
        .hashes()
        .unwrap();
    assert_eq!(base.records, promoted.records, "promotion changes no hash");
}

#[track_caller]
fn refused(tree: Tree, records: &[(String, String)], code: Code, says: &str) -> Refusal {
    match catalog(tree, records).hashes() {
        Ok(_) => panic!("expected {code} about `{says}`"),
        Err(r) => {
            assert_eq!(r.code, code, "{r}");
            assert!(r.message.contains(says), "not about `{says}`: {r}");
            r
        }
    }
}

fn edit(from: &str, to: &str) -> Vec<(String, String)> {
    let mut r = records();
    assert_eq!(
        r.iter().filter(|(_, t)| t.contains(from)).count(),
        1,
        "`{from}` in one record"
    );
    for (_, text) in &mut r {
        if text.contains(from) {
            *text = text.replacen(from, to, 1);
        }
    }
    r
}

#[test]
fn references_resolve_and_match() {
    let r = refused(
        tree(),
        &edit("(depends (lib.c \"1.0\"))", "(depends (lib.x \"1.0\"))"),
        Code::Dependency,
        "resolves to no record",
    );
    assert_eq!(
        (r.path.as_str(), r.field.as_str()),
        ("catalog/experimental/lib.a.catalog", "depends")
    );
    refused(
        tree(),
        &edit("(depends (lib.c \"1.0\"))", "(depends (lib.c \"2.0\"))"),
        Code::Dependency,
        "does not match",
    );
    let r = refused(
        tree(),
        &edit("(describes lib.c)", "(describes lib.x)"),
        Code::Dependency,
        "resolves to no record",
    );
    assert_eq!(r.field, "behavior-model describes");
    refused(
        tree(),
        &edit("(depends (lib.d \"1.0\"))", "(depends (lib.a \"1.0\"))"),
        Code::Dependency,
        "a cycle",
    );
    refused(
        tree(),
        &edit("(targets any)", "(targets moon)"),
        Code::Field,
        "not a target under `targets/`",
    );
    refused(
        tree(),
        &edit("(cost k (target board)", "(cost k (target moon)"),
        Code::Field,
        "does not admit",
    );
}

#[test]
fn source_sets_and_packages() {
    refused(
        tree(),
        &edit("(sources \"docs/models/m.txt\")", "(sources \"crates\")"),
        Code::Source,
        "has a package below it",
    );
    let mut nested = tree();
    nested.insert(
        "crates/a/inner/Cargo.toml",
        b"[package]\nname = \"inner\"\n".to_vec(),
    );
    refused(nested, &records(), Code::Source, "another package below it");
    refused(
        tree(),
        &edit("(sources \"crates/a\")", "(sources \"docs/models\")"),
        Code::Source,
        "an implementation names packages",
    );
    refused(
        tree(),
        &edit(
            "(sources \"docs/models/m.txt\")",
            "(sources \"docs/nothing\")",
        ),
        Code::Source,
        "is not tracked",
    );
    refused(
        tree(),
        &edit("(sources \"docs/models/m.txt\")", "(sources \"catalog/x\")"),
        Code::Source,
        "under `catalog/`",
    );
    let manifest = |deps: &str| {
        with(
            "crates/a/Cargo.toml",
            &format!("[package]\nname = \"a\"\n{deps}"),
        )
    };
    refused(
        manifest("[dependencies]\nb = \"1.0\"\n"),
        &records(),
        Code::Source,
        "not a path dependency",
    );
    refused(
        manifest("[dependencies]\nb = { path = \"../b\", workspace = true }\n"),
        &records(),
        Code::Source,
        "`workspace = true`",
    );
    refused(
        manifest("[dev-dependencies]\nt = \"0.1\"\n"),
        &records(),
        Code::Source,
        "not a path dependency",
    );
    refused(
        manifest("[target.riscv.dependencies]\nb = { path = \"../nope\" }\n"),
        &records(),
        Code::Source,
        "is not a tracked package",
    );
    refused(
        with("crates/a/Cargo.toml", "[package]\nname = 'a'\n"),
        &records(),
        Code::Source,
        "outside the manifest dialect",
    );
    let workspace = |text: &str| with("Cargo.toml", text);
    refused(
        workspace("[workspace]\nmembers = [\"crates/c\"]\n"),
        &records(),
        Code::Source,
        "not a member of the workspace",
    );
    refused(
        workspace("[workspace]\nmembers = [\"crates\"]\n"),
        &records(),
        Code::Source,
        "not a member of the workspace",
    );
    refused(
        workspace("[workspace]\nmembers = [\"crates/*\"]\nexclude = [\"crates/a\"]\n"),
        &records(),
        Code::Source,
        "not a member",
    );
    refused(
        with("rust-toolchain", "1.95.0\n"),
        &records(),
        Code::Source,
        "without the extension",
    );
    refused(
        with("crates/rust-toolchain.toml", "[toolchain]\n"),
        &records(),
        Code::Source,
        "anywhere but the repository root",
    );
}

#[test]
fn targets_and_their_env() {
    let mut unpaired = tree();
    unpaired.insert("targets/lonely.env", b"TARGET_KIND=board\n".to_vec());
    refused(unpaired, &records(), Code::Field, "without its pair");
    for (env, says) in [
        ("TARGET_KIND=board\nexport X\n", "`KEY=value`"),
        ("TARGET_KIND=board\nexport X=1\n", "not a key"),
        ("HOME=/x\n", "not a key"),
        ("TARGET_KIND=board\nTARGET_KIND=emulator\n", "appears twice"),
        ("TARGET_KIND=board $(x)\n", "outside §3's grammar"),
        ("TARGET_FILE=docs/none.x\n", "not a tracked path"),
        (
            "TARGET_KIND=board\nTARGET_WORD=Cargo.toml\n",
            "names a tracked file without its path",
        ),
    ] {
        refused(
            with("targets/board.env", env),
            &records(),
            Code::Field,
            says,
        );
    }
}

#[test]
fn locators_point_into_the_sets_they_are_allowed() {
    let file = edit(
        "(locator (file \"docs/models/m.txt\"))",
        "(locator (file \"docs/models/n.txt\"))",
    );
    refused(
        tree(),
        &file,
        Code::Locator,
        "not in the facet's own source set",
    );
    let code = edit(
        "(code lib.a \"crates/a/src/lib.rs\")",
        "(code lib.a \"crates/b/src/lib.rs\")",
    );
    refused(
        tree(),
        &code,
        Code::Locator,
        "not in `lib.a`'s implementation own set",
    );
    let other = edit(
        "(code lib.c \"crates/c/src/lib.rs\")",
        "(code lib.x \"crates/c/src/lib.rs\")",
    );
    refused(tree(), &other, Code::Locator, "which §2 does not allow");
    refused(
        tree(),
        &edit("(ledger alpha", "(ledger gamma"),
        Code::Locator,
        "0 times",
    );
    let doubled = with(
        "docs/book/src/ledger.md",
        &format!("{LEDGER}## `alpha`\n\nagain\n"),
    );
    refused(doubled, &records(), Code::Locator, "2 times");
}
