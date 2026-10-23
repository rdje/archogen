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

#[path = "trust_compiler.rs"]
mod compiler;

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
    configurations_on_paths, environment, toolchain_file, write_tree, ENV_ALLOWED,
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
    /// An integration test target: the comparison harness, or a derivation.
    Test(String),
}

/// A program whose build is inventoried: a root (§2) or the comparison harness (§3, §4).
#[derive(Debug, Clone)]
pub struct Program {
    /// Its name in the roots file.
    pub name: String,
    /// Its role; for the harness, `harness`; for a derivation, `derivation` (`decision_trust-role-programs.md` §3).
    pub role: String,
    /// Its package's directory, relative to the repository.
    pub package: String,
    /// What is built.
    pub target: Target,
    /// The packages holding its role's own logic (§2).
    pub role_packages: BTreeSet<String>,
    /// For the harness or a derivation: the two roots it compares.
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

/// The test-like units `cargo test` and `cargo bench` build, each compiling its package's development edges: an
/// integration test, a bench, a library's unit tests, an executable's unit tests and a library's doc tests. One is a
/// program target when its build joins two roles of a pair in [`JOINED`] (`decision_trust-role-programs.md` §3).
pub const TEST_KINDS: &[&str] = &["test", "bench", "lib-test", "bin-test", "doctest"];

/// The pairs of roles a property rests on (`decision_trust-role-programs.md` §5): a test-like unit whose build compiles
/// a role package of a root of each is a program target, named by a pair form or classified, whatever its sources
/// name, so no spelling hides a comparison (round 5's D1).
const JOINED: &[(&str, &str)] = &[
    ("generator", "configuration-checker"),
    ("generator", "scheduling-checker"),
    ("reference-model", "implementation"),
];

/// A program target that is not a root, classified by review (§2): the reason, and the role packages its build
/// compiles, so a set that grows is unreviewed again (R2 B6; R4 5; R5 remark 13).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Classification {
    /// Its name in the roots file.
    pub name: String,
    /// Its package's directory, relative to the repository.
    pub package: String,
    /// Which of [`PROGRAM_KINDS`] or [`TEST_KINDS`] it is.
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
    /// Every committed generated source declared, its form's shape holding (the generated-sources record §2).
    pub generated: Vec<crate::trust_generated::Generated>,
    /// Every refusal of the `defgenerated` forms' text, step 1 of that record's order: none, or the run ends.
    pub generated_refused: Vec<String>,
    /// Each root form's `(invokes …)` and `(installed …)`, as written, judged once the whole file is read
    /// (`decision_trust-role-programs.md` §4).
    pub front_clauses: Vec<FrontClauses>,
}

/// The launcher the front runs its invoked programs through, the one file `NO-SUBPROCESS` is to admit (`M3.5.1.4`): its
/// sha256 is part of every invocation, whose risk its acceptance contract carries (`decision_trust-role-programs.md` §4).
pub const LAUNCHER: &str = "crates/archogen-cli/src/sibling.rs";

/// A root form's `(invokes …)` and `(installed …)`: the programs the front runs and the file the install writes beside
/// it (`decision_trust-role-programs.md` §4). `None` is a clause the form does not hold; an empty list one it holds
/// empty.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct FrontClauses {
    /// The root whose form holds them.
    pub root: String,
    /// The roots its program invokes, by name.
    pub invokes: Option<Vec<String>>,
    /// The file it reads that the install writes.
    pub installed: Option<Vec<String>>,
}

/// Each form the instrument reads, and the clauses it takes; any other form or clause is refused (R8 11). A
/// `defgenerated` form is read apart, each departure a coded refusal (`trust_generated`).
const FORMS: &[(&str, &[&str])] = &[
    (
        "defroot",
        &[
            "role",
            "package",
            "target",
            "role-packages",
            "data",
            "invokes",
            "installed",
        ],
    ),
    ("defharness", &["pair", "package", "test"]),
    // A comparison of a reading of the generator's with a checker's, read as the harness but for its pairing and its
    // results (`decision_trust-role-programs.md` §3, leaf `M3.5.2.1.1`).
    ("defderivation", &["pair", "package", "test", "reason"]),
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
/// a harness whose pair names no root are refused (R8 11). A `defgenerated` form is read apart, every departure from
/// its shape a coded refusal held in [`Roots::generated_refused`] (the generated-sources record §2, step 1).
///
/// # Errors
///
/// A form the instrument cannot read, the gate unable to judge: the `defgenerated` refusals the reader met before it
/// are reported beside it, and every one beside a departure found only once the whole file is read.
pub fn read_roots(text: &str) -> Result<Roots, String> {
    let mut sources = SourceMap::new();
    let id = sources
        .add(ROOTS, text)
        .map_err(|e| format!("{ROOTS}: {e:?}"))?;
    let (doc, diags) = read(&sources, id);
    if diags.has_errors() {
        return Err(diags.render(&sources));
    }
    let line = |form: &Form| {
        sources
            .get(id)
            .map_or(0, |s| s.position(form.span().start).line)
    };
    let mut roots = Roots::default();
    let mut names: BTreeSet<String> = BTreeSet::new();
    let mut generated = Vec::new();
    for form in &doc.forms {
        if form.head() == Some(crate::trust_generated::GENERATED) {
            generated.push(crate::trust_generated::read_form(form, line(form)));
            continue;
        }
        // The parent's reader stops at the first other form that departs: the `defgenerated` forms before it are
        // reported beside it, and those after it once it is repaired.
        read_form(form, &mut roots, &mut names).map_err(|why| {
            crate::trust_generated::beside(why, &crate::trust_generated::refusals(&generated))
        })?;
    }
    // A departure found only once the whole file is read has every `defgenerated` form's refusals beside it.
    check_pairs(&roots)
        .and_then(|()| check_front(&roots))
        .map_err(|why| {
            crate::trust_generated::beside(why, &crate::trust_generated::refusals(&generated))
        })?;
    roots.generated_refused = crate::trust_generated::refusals(&generated);
    roots.generated = crate::trust_generated::declarations(generated);
    Ok(roots)
}

/// One form of the roots file other than a `defgenerated` one, read into `roots`.
fn read_form(form: &Form, roots: &mut Roots, names: &mut BTreeSet<String>) -> Result<(), String> {
    let head = form.head().unwrap_or("?");
    let Some((_, allowed)) = FORMS.iter().find(|(h, _)| *h == head) else {
        return Err(format!(
            "{ROOTS}: a form `{head}` the instrument does not know"
        ));
    };
    // An admission has no name; every other form names itself first.
    let named = head != "defadmit";
    let name = if named {
        form.items()
            .get(1)
            .and_then(word)
            .ok_or_else(|| format!("{ROOTS}: `{head}` needs a textual name before its clauses"))?
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
        // A clause holds names alone: anything else is refused, never dropped (`decision_trust-role-programs.md` §4).
        if c.items().iter().skip(1).any(|item| word(item).is_none()) {
            return Err(format!(
                "{ROOTS}: `{name}`'s `({h} …)` holds something that is not a name"
            ));
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
            let invokes = clause(form, "invokes").map(|_| values(form, "invokes"));
            let installed = clause(form, "installed").map(|_| values(form, "installed"));
            if invokes.is_some() || installed.is_some() {
                check_front_form(
                    &name,
                    &role,
                    &target,
                    invokes.as_deref(),
                    installed.as_deref(),
                )?;
                roots.front_clauses.push(FrontClauses {
                    root: name.clone(),
                    invokes,
                    installed,
                });
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
        "defharness" | "defderivation" => {
            let pair = values(form, "pair");
            let [a, b] = pair.as_slice() else {
                return Err(format!("{ROOTS}: `{name}` names `(pair ROOT ROOT)`"));
            };
            if head == "defderivation" {
                one(form, "reason", &name)?;
            }
            roots.programs.push(Program {
                role: if head == "defharness" {
                    "harness"
                } else {
                    "derivation"
                }
                .to_owned(),
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
                [k, n]
                    if PROGRAM_KINDS.contains(&k.as_str()) || TEST_KINDS.contains(&k.as_str()) =>
                {
                    (k.clone(), n.clone())
                }
                _ => {
                    return Err(format!(
                        "{ROOTS}: `{name}`'s target is `(target KIND NAME)`, KIND one of {}, {}",
                        PROGRAM_KINDS.join(", "),
                        TEST_KINDS.join(", ")
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
        "defrole" => {
            one(form, "leaf", &name)?;
        }
        _ => {}
    }
    Ok(())
}

/// A harness compares two distinct roots the file declares.
fn check_pairs(roots: &Roots) -> Result<(), String> {
    let root = |x: &str| {
        roots
            .programs
            .iter()
            .find(|p| p.name == x && p.pair.is_none())
    };
    // Each test target one pair form's (`decision_trust-role-programs.md` §3): a comparison is written one way.
    let mut targets: BTreeMap<(&str, &str), &str> = BTreeMap::new();
    for h in &roots.programs {
        let Some((a, b)) = &h.pair else { continue };
        for x in [a, b] {
            if root(x).is_none() {
                return Err(format!(
                    "{ROOTS}: `{}`'s pair names `{x}`, which names no root",
                    h.name
                ));
            }
        }
        if a == b {
            return Err(format!("{ROOTS}: `{}`'s pair names `{a}` twice", h.name));
        }
        // A harness compares a reference model with the implementation it validates, a derivation the generator with
        // either checker, in either order (`decision_trust-role-programs.md` §3; round 5's D2).
        let wants: &[[&str; 2]] = if h.role == "harness" {
            &[["implementation", "reference-model"]]
        } else {
            &[
                ["configuration-checker", "generator"],
                ["generator", "scheduling-checker"],
            ]
        };
        let mut roles = [root(a), root(b)].map(|p| p.map_or("", |p| p.role.as_str()));
        roles.sort_unstable();
        if !wants.contains(&roles) {
            let want = if h.role == "harness" {
                "`implementation` and `reference-model`"
            } else {
                "`generator` and `configuration-checker` or `scheduling-checker`"
            };
            return Err(format!(
                "{ROOTS}: `{}`'s pair has roles `{}` and `{}`; a {}'s has {want}",
                h.name, roles[0], roles[1], h.role
            ));
        }
        if let Target::Test(t) = &h.target {
            if let Some(other) = targets.insert((h.package.as_str(), t.as_str()), &h.name) {
                return Err(format!(
                    "{ROOTS}: `{other}` and `{}` name one test target, `{t}` of `{}`",
                    h.name, h.package
                ));
            }
        }
    }
    // A test a pair form names is a comparison; a classification says its unit compares nothing, so the two contradict
    // (`decision_trust-role-programs.md` §3; round 6's R9).
    for c in roots.classified.iter().filter(|c| c.kind == "test") {
        if let Some(pair) = targets.get(&(c.package.as_str(), c.target.as_str())) {
            return Err(format!(
                "{ROOTS}: `{}` classifies the test `{}` of `{}`, which `{pair}` names as a comparison",
                c.name, c.target, c.package
            ));
        }
    }
    Ok(())
}

/// The front's clauses, judged at its form (`decision_trust-role-programs.md` §4; the generated-sources record's
/// clarification, a read failure within a form stopping the reader there): on a root of role `generator` whose target
/// is an executable, both, `invokes` naming roots once and never its own, `installed` the install manifest's one name.
/// Which roots it names, and how many roots have its role, are known once the whole file is read ([`check_front`]).
fn check_front_form(
    name: &str,
    role: &str,
    target: &Target,
    invokes: Option<&[String]>,
    installed: Option<&[String]>,
) -> Result<(), String> {
    if role != "generator" {
        return Err(format!(
            "{ROOTS}: `{name}` holds `invokes` or `installed`, which only the front's form takes: the root of role \
             `generator`"
        ));
    }
    if !matches!(target, Target::Bin(_)) {
        return Err(format!(
            "{ROOTS}: the front `{name}` invokes nothing: its target is no executable"
        ));
    }
    let (Some(invokes), Some(installed)) = (invokes, installed) else {
        return Err(format!(
            "{ROOTS}: `{name}` holds one of `invokes` and `installed` without the other"
        ));
    };
    if invokes.is_empty() {
        return Err(format!("{ROOTS}: `{name}`'s `invokes` names no root"));
    }
    let mut seen = BTreeSet::new();
    for invoked in invokes {
        if !seen.insert(invoked) {
            return Err(format!("{ROOTS}: `{name}` invokes `{invoked}` twice"));
        }
        if invoked == name {
            return Err(format!("{ROOTS}: `{name}` invokes itself"));
        }
    }
    if installed != [archogen_install::FILE] {
        return Err(format!(
            "{ROOTS}: `{name}`'s `installed` holds {installed:?}, not the one name `{}`",
            archogen_install::FILE
        ));
    }
    Ok(())
}

/// The front's clauses, judged once the whole file is read (`decision_trust-role-programs.md` §4): the one root of role
/// `generator` holds them, and each root `invokes` names is a root whose target is an executable.
fn check_front(roots: &Roots) -> Result<(), String> {
    let root = |x: &str| {
        roots
            .programs
            .iter()
            .find(|p| p.name == x && p.pair.is_none())
    };
    let generators = roots
        .programs
        .iter()
        .filter(|p| p.pair.is_none() && p.role == "generator")
        .count();
    for c in &roots.front_clauses {
        if generators != 1 {
            return Err(format!(
                "{ROOTS}: {generators} roots have role `generator`, so none is the front"
            ));
        }
        for name in c.invokes.iter().flatten() {
            let Some(invoked) = root(name) else {
                return Err(format!(
                    "{ROOTS}: `{}` invokes `{name}`, which names no root",
                    c.root
                ));
            };
            if !matches!(invoked.target, Target::Bin(_)) {
                return Err(format!(
                    "{ROOTS}: `{}` invokes `{name}`, whose target is no executable",
                    c.root
                ));
            }
        }
    }
    Ok(())
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
    /// Its crate root, as a repository path.
    pub root: String,
    /// Whether it is built as a test harness (`--test`).
    pub harness: bool,
}

/// A unit's configuration as an item compares it: its normalised command line and every `# env-dep` line with its
/// value, since a value compiled in through `env!` is configuration as much as a flag is (§3, R8 5).
/// Separate ordered string arrays preserve token/record boundaries even when a value contains separator text.
fn unit_configuration(u: &Unit) -> String {
    crate::json::write(&Json::Array(vec![
        strings(u.configuration.clone()),
        strings(u.env.clone()),
    ]))
}

/// What one unit's dependency record names: its prerequisites, or the raw first rule and the way that text names
/// two prerequisite lists at once, which the inventory refuses rather than choosing one.
enum Prerequisites {
    Read(Vec<String>),
    Ambiguous { raw: String, why: &'static str },
}

/// Read rustc's first dependency rule, matching its known raw output target before interpreting separators.
/// A source path inside the written tree has the catalog's ASCII grammar; only its scratch prefix can contain
/// spaces or line breaks. Rustc escapes a prerequisite's spaces as backslash-space and writes every other byte
/// literally, repeats the prerequisites in one rule per output, each followed by a blank line, then lists one
/// phony target per prerequisite on a line of its own, then, after a blank line, its `# env-dep:` records. A
/// name ending in a backslash followed by the next prerequisite spells the same text as one escaped path, so one
/// rule can name two lists: mark each known escaped prefix at a prerequisite's start before splitting at spaces,
/// refuse a backslash anywhere else, and require the phony targets to re-spell exactly the list read, naming the
/// raw rule otherwise. This is not a general Makefile parser.
fn dependency_files(
    text: &str,
    dependency_file: &Path,
    scratch: &str,
    canonical_scratch: &str,
) -> Result<Prerequisites, String> {
    let target = format!("{}: ", dependency_file.display());
    let rules = text.strip_prefix(&target).ok_or_else(|| {
        format!(
            "{}: missing expected first dependency target",
            dependency_file.display()
        )
    })?;
    // A path never holds a NUL, so a marked prefix is one token however many spaces or line breaks it holds.
    let raw_prefix = scratch.replace(' ', "\\ ");
    let canonical_prefix = canonical_scratch.replace(' ', "\\ ");
    let marked = format!(" {rules}")
        .replace(&format!(" {raw_prefix}"), " \0r")
        .replace(&format!(" {canonical_prefix}"), " \0c");
    let rule = marked
        .split_once('\n')
        .map_or(marked.as_str(), |(rule, _)| rule);
    let raw = rule
        .replace("\0r", &raw_prefix)
        .replace("\0c", &canonical_prefix)[1..]
        .to_owned();
    let rest = &rules[raw.len()..];
    let mut files = Vec::new();
    for token in rule.split(' ').filter(|t| !t.is_empty()) {
        let (prefix, suffix) = [("\0r", scratch), ("\0c", canonical_scratch)]
            .into_iter()
            .find_map(|(mark, prefix)| token.strip_prefix(mark).map(|s| (prefix, s)))
            .unwrap_or(("", token));
        if suffix.contains('\\') {
            return Ok(Prerequisites::Ambiguous {
                raw,
                why: "a backslash outside the written tree's prefixes",
            });
        }
        files.push(format!("{prefix}{suffix}"));
    }
    if files.is_empty() {
        return Err(format!(
            "{}: empty first dependency rule",
            dependency_file.display()
        ));
    }
    let phony: String = files
        .iter()
        .map(|f| format!("{}:\n", f.replace(' ', "\\ ")))
        .collect();
    let repeated = format!(": {raw}\n\n");
    let agreed = rest
        .strip_prefix("\n\n")
        .and_then(|t| t.rsplit(&repeated).next())
        .and_then(|t| t.strip_prefix(&phony))
        .is_some_and(|env| {
            env.is_empty()
                || env
                    .strip_prefix('\n')
                    .is_some_and(|records| records.lines().all(|l| l.starts_with("# env-dep:")))
        });
    if !agreed {
        return Ok(Prerequisites::Ambiguous {
            raw,
            why: "its per-file targets spell the prerequisites otherwise",
        });
    }
    Ok(Prerequisites::Read(files))
}

/// Keep rustc's encoded environment records intact; encode only the scratch prefixes before replacing them.
/// The pinned writer escapes backslashes and line breaks, unlike prerequisite paths, which escape only spaces.
fn normalise_dependency_environment(dep: &str, scratch: &str, canonical_scratch: &str) -> String {
    let encode = |path: &str| {
        path.replace('\\', "\\\\")
            .replace('\n', "\\n")
            .replace('\r', "\\r")
    };
    dep.replace(&encode(scratch), "")
        .replace(&encode(canonical_scratch), "")
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

/// The units the harness `h` compiles beside its pair's two builds `x` and `y` — its test target, the adapters its
/// development dependencies add, and any unit compiled otherwise than in either build — recognised by package, crate and
/// configuration (§4, R8 7).
fn beside<'a>(h: &'a Build, x: &Build, y: &Build) -> Vec<&'a Unit> {
    let key = |u: &Unit| {
        (
            u.package.clone(),
            u.crate_name.clone(),
            unit_configuration(u),
        )
    };
    let in_pair: BTreeSet<_> = x.units.iter().chain(y.units.iter()).map(key).collect();
    h.units
        .iter()
        .filter(|u| !in_pair.contains(&key(u)))
        .collect()
}

/// Select a failure line outside multiline workspace and artifact paths. Admitted source/configuration names
/// contain no line breaks; their directory prefixes can. Include ancestor and canonical spellings so raw status
/// paths and configuration warnings are covered, as well as Cargo's Unix single-quoted command arguments.
fn cargo_error_line(stderr: &str, cwd: &Path, env: &[(String, String)]) -> String {
    let mut paths = vec![cwd.to_path_buf()];
    if let Some((_, target)) = env.iter().rfind(|(key, _)| key == "CARGO_TARGET_DIR") {
        let target = Path::new(target);
        paths.push(if target.is_absolute() {
            target.to_path_buf()
        } else {
            cwd.join(target)
        });
    }
    let mut prefixes = BTreeSet::new();
    for path in paths {
        let canonical = fs::canonicalize(&path).ok();
        for spelling in std::iter::once(path).chain(canonical) {
            for prefix in spelling.ancestors() {
                let prefix = prefix.to_string_lossy();
                if prefix.contains('\n') {
                    prefixes.insert(prefix.replace('\'', "'\\''"));
                    prefixes.insert(prefix.into_owned());
                }
            }
        }
    }
    let path_spans: Vec<_> = prefixes
        .iter()
        .flat_map(|prefix| {
            stderr
                .match_indices(prefix.as_str())
                .map(|(start, matched)| (start, start + matched.len()))
        })
        .collect();
    let mut offset = 0;
    for physical in stderr.split_inclusive('\n') {
        let start = offset;
        offset += physical.len();
        let line = if let Some(line) = physical.strip_suffix('\n') {
            line.strip_suffix('\r').unwrap_or(line)
        } else {
            physical
        };
        if line.starts_with("error")
            && !path_spans
                .iter()
                .any(|&(begin, end)| begin <= start && start < end)
        {
            return line.to_owned();
        }
    }
    String::new()
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
    // Keep the pinned compiler identity independent of its installation path. The private observer
    // requires this literal program; legacy verbose-output probes also retain an unambiguous field.
    c.env("RUSTC", "rustc");
    let out = c.output().map_err(|e| format!("cargo did not run: {e}"))?;
    let stdout = String::from_utf8_lossy(&out.stdout).into_owned();
    let stderr = String::from_utf8_lossy(&out.stderr).into_owned();
    if !out.status.success() {
        let first = cargo_error_line(&stderr, cwd, env);
        return Err(format!("`cargo {}` failed: {first}", args.join(" ")));
    }
    Ok((stdout, stderr))
}

/// The packages' targets from `cargo metadata`: each crate root, as a repository path, with its crate name (a
/// target's name, `-` written `_`, as rustc's `--crate-name` takes it), to its package's directory (R8 2).
pub type Targets = BTreeMap<(String, String), String>;

#[path = "trust_artifact.rs"]
mod artifact;

/// Metadata source roots by package, target kind and original target name.
type TargetSources = BTreeMap<(String, String, String), BTreeSet<String>>;

/// A metadata-owned compilation, retaining the observer's actual arguments.
struct Compilation {
    package: String,
    arguments: Vec<String>,
}

/// A unit's normalised `configuration`, returned as it is once each entry that still names a path outside the written
/// tree is refused, in order: one that begins with `/`, or one that holds `=/`. Such a path is something the commit
/// does not hold and another checkout would spell otherwise (R9 1). No producer route measured reaches it — the cargo
/// configuration, workspace `rustflags`, an inherited `RUSTFLAGS` and build scripts are refused before any build
/// (`M3.6.9.6.3.2.2.15.2.1.7.1`) — so it is defence in depth, held by its own control; a unit's configuration is
/// only ever had through it, so the check cannot be dropped without the configuration.
fn checked_configuration(
    crate_name: &str,
    who: &str,
    configuration: Vec<String>,
    refused: &mut Vec<String>,
) -> Vec<String> {
    for t in &configuration {
        if t.starts_with('/') || t.contains("=/") {
            refused.push(format!(
                "trust-undeclared-input: `{crate_name}` in `{who}` is configured with `{t}`, a path outside the \
                 written tree"
            ));
        }
    }
    configuration
}

/// Compile one target of the package at `package` clean in `tree_dir` — `target_args` such as `["build", "--bin",
/// NAME]` — and read every unit's dependency information, each refusal naming `who` (§3). Returns the units, in a fixed
/// order, and the metadata-owned compiler invocations.
#[allow(clippy::too_many_arguments)]
fn compile(
    who: &str,
    package: &str,
    target_args: &[String],
    package_names: &BTreeMap<String, String>,
    targets: &Targets,
    env: &[(String, String)],
    tree_dir: &Path,
    compiler: &compiler::Compiler,
    refused: &mut Vec<String>,
) -> Result<(Vec<Unit>, Vec<Compilation>), String> {
    let name = package_names
        .get(package)
        .ok_or_else(|| format!("`{who}`: no package at `{package}`"))?;
    let mut args: Vec<String> = target_args.to_vec();
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
    let session = compiler.session(who, env, tree_dir)?;
    let compile_env = session.environment(env)?;
    let _ = run_cargo(&argv, &compile_env, tree_dir)?;
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
    let mut compilations = Vec::new();
    for tokens in session
        .invocations()
        .map_err(|why| format!("`{who}` compiler provenance: {why}"))?
    {
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
                    who
                ));
                continue;
            }
        };
        let root = tokens
            .iter()
            .skip(1)
            .map(|t| strip(t))
            .find(|t| targets.contains_key(&(t.clone(), crate_name.clone())))
            .unwrap_or_default();
        let harness = tokens.iter().any(|t| t == "--test");
        let d = PathBuf::from(&out_dir).join(format!("{crate_name}{extra}.d"));
        let text = fs::read_to_string(&d).map_err(|e| format!("{}: {e}", d.display()))?;
        let mut files = BTreeSet::new();
        let mut env_deps = Vec::new();
        for l in text.lines() {
            if let Some(dep) = l.strip_prefix("# env-dep:") {
                // Only cargo's own variables, which the commit's manifests decide: the catalog's list (R8 remark 22).
                let var = dep.split('=').next().unwrap_or_default();
                if !(var.starts_with("CARGO_PKG_") || ENV_ALLOWED.contains(&var)) {
                    refused.push(format!(
                        "trust-undeclared-input: `{crate_name}` in `{}` depends on the environment variable `{var}`, \
                         which the commit does not hold",
                        who
                    ));
                }
                env_deps.push(normalise_dependency_environment(
                    dep,
                    &scratch,
                    &canonical_scratch,
                ));
            }
        }
        let raws = match dependency_files(&text, &d, &scratch, &canonical_scratch)? {
            Prerequisites::Read(raws) => raws,
            Prerequisites::Ambiguous { raw, why } => {
                refused.push(format!(
                    "trust-undeclared-input: `{}` reads `{raw}` ambiguously: {why}",
                    who
                ));
                Vec::new()
            }
        };
        for raw in raws {
            let rel = if let Some(r) = raw
                .strip_prefix(&scratch)
                .or_else(|| raw.strip_prefix(&canonical_scratch))
            {
                r.to_owned()
            } else if raw.starts_with('/') {
                refused.push(format!(
                    "trust-undeclared-input: `{}` reads `{raw}`, outside the written tree",
                    who
                ));
                continue;
            } else {
                raw
            };
            match lexical(&rel) {
                Some(path) => {
                    files.insert(path);
                }
                None => refused.push(format!("trust-undeclared-input: `{}` reads `{rel}`, which climbs out of the written tree", who)),
            }
        }
        let configuration =
            checked_configuration(&crate_name, who, normalise(&tokens, &scratch), refused);
        compilations.push(Compilation {
            package: package.clone(),
            arguments: tokens,
        });
        units.push(Unit {
            crate_name,
            package,
            configuration,
            files,
            env: env_deps,
            root,
            harness,
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
    if !units.iter().any(|u| u.package == package) {
        return Err(format!(
            "`{who}`'s build compiled nothing of its own package `{package}`"
        ));
    }
    Ok((units, compilations))
}

/// Build `p` clean in `tree_dir`, into `target_dir`, and read every unit's dependency information and its artifact.
#[allow(clippy::too_many_arguments)]
fn build(
    p: &Program,
    package_names: &BTreeMap<String, String>,
    targets: &Targets,
    sources: &TargetSources,
    env: &[(String, String)],
    tree_dir: &Path,
    target_dir: &Path,
    compiler: &compiler::Compiler,
    refused: &mut Vec<String>,
) -> Result<Build, String> {
    let target_args: Vec<String> = match &p.target {
        Target::Test(t) => ["test", "--no-run", "--test", t]
            .map(str::to_owned)
            .to_vec(),
        Target::Bin(b) => ["build", "--bin", b].map(str::to_owned).to_vec(),
        Target::Lib => ["build", "--lib"].map(str::to_owned).to_vec(),
    };
    let (units, compilations) = compile(
        &p.name,
        &p.package,
        &target_args,
        package_names,
        targets,
        env,
        tree_dir,
        compiler,
        refused,
    )?;
    let artifact = artifact::select(p, sources, &compilations, env, tree_dir, target_dir)?;
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
/// A program target's closure, as some build of it compiles it: its package's by normal and build edges, an example's
/// and a test-like unit's by its package's development edges too, since they compile them (§2, R5 remark 13;
/// `decision_trust-role-programs.md` §3). An edge for any platform counts, so a closure is the union over platforms.
fn target_closure(
    package: &str,
    kind: &str,
    graph: &[(String, String, String)],
) -> BTreeSet<String> {
    let mut closure: BTreeSet<String> = BTreeSet::new();
    let mut todo = vec![package.to_owned()];
    while let Some(pkg) = todo.pop() {
        if !closure.insert(pkg.clone()) {
            continue;
        }
        for (from, to, edge) in graph {
            let followed = edge == "normal"
                || edge == "build"
                || ((kind == "example" || TEST_KINDS.contains(&kind))
                    && edge == "dev"
                    && from == package);
            if *from == pkg && followed && !closure.contains(to) {
                todo.push(to.clone());
            }
        }
    }
    closure
}

/// Each test-like unit whose build — its package's closure, its development edges followed — compiles a role package of
/// a root of each role of a pair in [`JOINED`]: a program target, whatever its sources name
/// (`decision_trust-role-programs.md` §3; round 5's D1). A unit compiling one role's alone is none.
fn joined_units(
    roots: &Roots,
    units: &[(String, String, String)],
    graph: &[(String, String, String)],
    census: &Census,
) -> Vec<(String, String, String)> {
    units
        .iter()
        .filter(|(package, kind, name)| {
            !joined_pairs(roots, package, kind, name, graph, census).is_empty()
        })
        .cloned()
        .collect()
}

/// What the census build compiled (`decision_trust-role-programs.md` §3; round 8's change of method): the files each
/// target's own unit read, by package, kind and name — a test-like unit's by its kind of [`TEST_KINDS`] — and every file
/// a package's library units read. Read from the compiler's dependency information, so a file a unit compiles is
/// counted however its source names it, or whatever names it in a dependency.
#[derive(Default)]
struct Census {
    own: BTreeMap<(String, String, String), BTreeSet<String>>,
    libraries: BTreeMap<String, BTreeSet<String>>,
}

impl Census {
    /// The census of `units`, each placed by its crate root among `sources`' targets.
    fn of(units: &[Unit], sources: &TargetSources) -> Self {
        let mut census = Self::default();
        for u in units {
            let Some((dir, kind, name)) = sources
                .iter()
                .filter(|((dir, _, name), srcs)| {
                    *dir == u.package
                        && srcs.contains(&u.root)
                        && name.replace('-', "_") == u.crate_name
                })
                .map(|(key, _)| key)
                .min_by_key(|(_, kind, _)| kind != "lib")
            else {
                continue;
            };
            let library = matches!(
                kind.as_str(),
                "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro"
            );
            let unit_kind = match (u.harness, kind.as_str()) {
                (true, "test" | "bench") => kind.clone(),
                (true, _) if library => "lib-test".to_owned(),
                (true, "bin") => "bin-test".to_owned(),
                _ => kind.clone(),
            };
            if library && !u.harness {
                census
                    .libraries
                    .entry(dir.clone())
                    .or_default()
                    .extend(u.files.iter().cloned());
            }
            // An executable's unit tests compile its sources and more: a binary `cargo test` builds as its harness alone
            // counts them for the binary too, a superset of what its own build reads.
            if u.harness && kind == "bin" {
                census
                    .own
                    .entry((dir.clone(), "bin".to_owned(), name.clone()))
                    .or_default()
                    .extend(u.files.iter().cloned());
            }
            census
                .own
                .entry((dir.clone(), unit_kind, name.clone()))
                .or_default()
                .extend(u.files.iter().cloned());
        }
        census
    }
}

/// The role packages a program target's build compiles (`decision_trust-role-programs.md` §3): those its closure
/// reaches by edges, over every platform and feature, and those holding a file its own unit or a library of its closure
/// read — a doc test's own unit its library's. One computation for the census, a target's classification and the
/// generator judgement (round 8's R6).
fn compiled_role_packages(
    roots: &Roots,
    census: &Census,
    graph: &[(String, String, String)],
    package: &str,
    kind: &str,
    name: &str,
) -> BTreeSet<String> {
    let role_packages: BTreeSet<&String> = roots
        .programs
        .iter()
        .filter(|p| p.pair.is_none())
        .flat_map(|p| p.role_packages.iter())
        .collect();
    let closure = target_closure(package, kind, graph);
    let own = census
        .own
        .get(&(package.to_owned(), kind.to_owned(), name.to_owned()))
        .into_iter()
        .flatten();
    let libraries = closure
        .iter()
        .filter_map(|p| census.libraries.get(p))
        .flatten();
    let mut compiled: BTreeSet<String> = closure
        .iter()
        .filter(|p| role_packages.contains(p))
        .cloned()
        .collect();
    for file in own.chain(libraries) {
        for rp in &role_packages {
            if file.starts_with(&format!("{rp}/")) {
                compiled.insert((*rp).clone());
            }
        }
    }
    compiled
}

/// The pairs of [`JOINED`] a unit's build joins, each `<role>+<role>`: the generator's and a checker's are a derivation's
/// to name, the reference model's and the implementation's a harness's (`decision_trust-role-programs.md` §3).
fn joined_pairs(
    roots: &Roots,
    package: &str,
    kind: &str,
    name: &str,
    graph: &[(String, String, String)],
    census: &Census,
) -> Vec<String> {
    let compiled = compiled_role_packages(roots, census, graph, package, kind, name);
    let compiles = |role: &str| {
        roots
            .programs
            .iter()
            .filter(|p| p.pair.is_none() && p.role == role)
            .flat_map(|p| p.role_packages.iter())
            .any(|p| compiled.contains(p))
    };
    JOINED
        .iter()
        .filter(|(a, b)| compiles(a) && compiles(b))
        .map(|(a, b)| format!("{a}+{b}"))
        .collect()
}

fn classify_programs(
    roots: &Roots,
    program_targets: &[(String, String, String)],
    graph: &[(String, String, String)],
    census: &Census,
) -> (Vec<Json>, Vec<String>) {
    let role_packages: BTreeSet<&String> = roots
        .programs
        .iter()
        .filter(|p| p.pair.is_none())
        .flat_map(|p| p.role_packages.iter())
        .collect();
    let mut out = Vec::new();
    let mut matched: BTreeSet<&String> = BTreeSet::new();
    for (package, kind, name) in program_targets {
        // A root names its executable, and a harness or a derivation its test (`decision_trust-role-programs.md` §3).
        let root = roots.programs.iter().find(|p| {
            p.package == *package
                && match &p.target {
                    Target::Bin(b) => p.pair.is_none() && kind == "bin" && b == name,
                    Target::Test(t) => p.pair.is_some() && kind == "test" && t == name,
                    Target::Lib => false,
                }
        });
        let compiled: BTreeSet<String> =
            compiled_role_packages(roots, census, graph, package, kind, name)
                .into_iter()
                .filter(|p| role_packages.contains(p))
                .collect();
        let classification = roots
            .classified
            .iter()
            .find(|c| c.package == *package && c.kind == *kind && c.target == *name);
        let status = match (root, classification) {
            (Some(r), _) if r.pair.is_some() => format!("{} {}", r.role, r.name),
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
            (None, None) if TEST_KINDS.contains(&kind.as_str()) => format!(
                "trust-unclassified-program: the {kind} `{name}` of `{package}` compiles {}, role packages of two roles a \
                 property rests on, and no form in {ROOTS} names or classifies it",
                compiled.iter().map(|c| format!("`{c}`")).collect::<Vec<_>>().join(", ")
            ),
            (None, None) => format!(
                "trust-unclassified-program: the {kind} `{name}` of `{package}` is neither a root nor classified in {ROOTS}"
            ),
        };
        let mut entry = vec![
            ("package", s(package)),
            ("kind", s(kind)),
            ("name", s(name)),
            ("status", s(status)),
            ("role-packages", strings(compiled)),
        ];
        // The pairs a test-like unit joins, so its remedy names each pair's form (round 6's D3).
        if TEST_KINDS.contains(&kind.as_str()) {
            entry.push((
                "joins",
                strings(joined_pairs(roots, package, kind, name, graph, census)),
            ));
        }
        out.push(obj(entry));
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

/// A step that refuses ends the run: its refusals, sorted, and no inventory (§3, R5 9; the generated-sources record
/// §2).
fn refuse(mut refused: Vec<String>) -> Outcome {
    refused.sort();
    refused.dedup();
    Outcome::Refused(refused)
}

/// A run the gate cannot judge past, ended with the refusals its step found before it beside it (the generated-sources
/// record §2, clarified by `M3.5.2.5.2`).
fn unable(refused: &[String]) -> impl Fn(String) -> String + '_ {
    move |why| crate::trust_generated::beside(why, refused)
}

/// The catalog's rules across one run (§3): every site they refuse, counted once however many programs compile it; the
/// admissions the sites use; the manifests and the files already judged. A generator's build (the generated-sources
/// record §5) is judged through the same `Rules` after the programs', so a site a root and a generator both compile is
/// one site, admitted once, and an admission a generator's site uses is a use.
struct Rules<'a> {
    tree: &'a Tree,
    tree_dir: &'a Path,
    symlinks: &'a BTreeSet<String>,
    admissions: &'a BTreeMap<Admission, usize>,
    package_targets: &'a BTreeMap<String, Vec<String>>,
    /// Actual metadata kinds are a hard prebuild boundary, independent of manifest admissions.
    metadata_targets: &'a BTreeMap<String, Vec<(Vec<String>, String)>>,
    /// Every site the rules refused, admitted or not.
    sites: usize,
    /// How many sites each admission admits.
    used: BTreeMap<Admission, usize>,
    manifests_checked: BTreeSet<String>,
    judged: BTreeSet<String>,
}

impl Rules<'_> {
    /// The catalog's rules on a workspace manifest — no `[patch]` or `[replace]` swapping a dependency's source, no
    /// `cargo-features`, no `rustflags` — over the root manifest every build reads (R8 remark 16), admitted, like a
    /// package's manifest rule, only by the manifest's sha256.
    fn workspace(&mut self, refused: &mut Vec<String>) {
        if let Some(bytes) = self.tree.get("Cargo.toml") {
            match manifest::parse(&String::from_utf8_lossy(bytes)) {
                Ok(m) => {
                    if let Err(why) = package::check_workspace("Cargo.toml", &m) {
                        self.sites += 1;
                        let admission = Admission {
                            file: "Cargo.toml".to_owned(),
                            rule: why.clone(),
                            sha256: Digest::of(bytes).hex(),
                        };
                        if self.admissions.contains_key(&admission) {
                            *self.used.entry(admission).or_default() += 1;
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
    }

    /// The manifest rules over every package of `closure`, from the metadata graph, before any build of `who`: a build
    /// script or a procedural macro is refused here, so none runs (§3, R2 B12), and a crate root that is not a `.rs`
    /// file (R8 1); each successful package judgment once across the run.
    fn manifests(&mut self, who: &str, closure: &BTreeSet<String>, refused: &mut Vec<String>) {
        for pkg in closure {
            let before = refused.len();
            // rustc compiles a crate root whatever its name, and no token rule reads a file that is not `.rs`; `mod`
            // finds only `.rs` files, and `#[path]` and `include!` are refused, so with this every Rust source a root
            // compiles is a `.rs` file (R8 1).
            for src in self.package_targets.get(pkg).into_iter().flatten() {
                if std::path::Path::new(src)
                    .extension()
                    .is_none_or(|e| e != "rs")
                {
                    refused.push(format!("trust-undeclared-input: `{}` reaches `{src}`, a crate root that is not a `.rs` file, which rustc compiles and no rule reads", who));
                }
            }
            if self.manifests_checked.contains(pkg) {
                continue;
            }
            for (kinds, name) in self.metadata_targets.get(pkg).into_iter().flatten() {
                for kind in kinds {
                    if kind == "custom-build" || kind == "proc-macro" {
                        refused.push(format!(
                            "trust-undeclared-input: `{who}` reaches `{pkg}` target `{name}` of kind `{kind}`, \
                             which runs package code during compilation (§3)"
                        ));
                    }
                }
            }
            let path = if pkg.is_empty() {
                "Cargo.toml".to_owned()
            } else {
                format!("{pkg}/Cargo.toml")
            };
            match self
                .tree
                .get(&path)
                .map(|m| manifest::parse(&String::from_utf8_lossy(m)))
            {
                Some(Ok(m)) => {
                    if let Err(why) = package::check_manifest(self.tree, pkg, &path, &m) {
                        // A manifest rule has no line: its admission names the manifest, the catalog's reason, and
                        // the manifest's own sha256, so any edit to the manifest is reviewed again.
                        self.sites += 1;
                        let bytes = self.tree.get(&path).unwrap_or_default();
                        let admission = Admission {
                            file: path.clone(),
                            rule: why.clone(),
                            sha256: Digest::of(bytes).hex(),
                        };
                        if self.admissions.contains_key(&admission) {
                            *self.used.entry(admission).or_default() += 1;
                        } else {
                            refused.push(format!("trust-undeclared-input: `{}` reaches `{pkg}`: {why}, and no admission names it (manifest sha256 {})", who, admission.sha256));
                        }
                    }
                }
                Some(Err(o)) => refused.push(format!(
                    "trust-undeclared-input: `{path}` line {}: {}",
                    o.line, o.why
                )),
                None => refused.push(format!(
                    "trust-undeclared-input: `{}` reaches a package with no tracked `{path}`",
                    who
                )),
            }
            // A refusal must still block every later target that reaches this package.
            if refused.len() == before {
                self.manifests_checked.insert(pkg.clone());
            }
        }
    }

    /// The token rules over every `.rs` file `compiled` names — each file with the programs that read it — and no earlier
    /// call judged, however it was read (§3; R8 10), and one admission per site.
    fn files(&mut self, compiled: &BTreeMap<String, BTreeSet<String>>, refused: &mut Vec<String>) {
        let mut found_sites: BTreeMap<Admission, (Vec<String>, Vec<u8>)> = BTreeMap::new();
        for (f, readers) in compiled {
            if !self.judged.insert(f.clone()) {
                continue; // judged already: a site two programs compile is one site
            }
            let name = readers.iter().next().map_or("", |n| n.as_str());
            if self.symlinks.contains(f) {
                refused.push(format!(
                    "trust-undeclared-input: `{name}` reads `{f}`, a symbolic link in the commit"
                ));
                continue;
            }
            let Some(blob) = self.tree.get(f) else {
                refused.push(format!(
                    "trust-undeclared-input: `{name}` reads `{f}`, which is not a blob of the commit"
                ));
                continue;
            };
            if fs::read(self.tree_dir.join(f)).ok().as_deref() != Some(blob) {
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
                        self.sites += 1;
                        match site_extent(blob, line, column) {
                            Ok(extent) => found_sites
                                .entry(Admission {
                                    file: f.clone(),
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
            let admitted = self.admissions.get(key).copied().unwrap_or(0);
            if at.len() > admitted {
                for site in at {
                    if admitted == 0 {
                        refused.push(format!("trust-undeclared-input: `{site}`: `{}`, refused by the catalog's rules, and no admission names it (site sha256 {})", key.rule, key.sha256));
                    } else {
                        refused.push(format!("trust-undeclared-input: `{site}`: `{}`, refused by the catalog's rules: {} sites share this text and {admitted} admission(s) name it (site sha256 {})", key.rule, at.len(), key.sha256));
                    }
                }
            }
            *self.used.entry(key.clone()).or_default() += at.len().min(admitted);
        }
    }
}

/// Build the inventory of `commit` in the repository at `repo`, scratch under `out`.
///
/// # Errors
///
/// A failure that is not a refusal: git, cargo or the file system did not answer.
pub fn inventory(repo: &Path, commit: &str, out: &Path) -> Result<Outcome, String> {
    inventory_with(repo, commit, out, &[repo.to_path_buf()])
}

/// [`inventory`], with cargo configurations on the build's path judged against explicitly supplied outer roots.
/// The innermost containing root owns each configuration; its bytes/content still match the judged tree. Production
/// supplies the repository alone; tests may supply their authorized containing repositories.
///
/// # Errors
///
/// As [`inventory`].
pub fn inventory_with(
    repo: &Path,
    commit: &str,
    out: &Path,
    config_roots: &[PathBuf],
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
    // Clear the earlier inventory before forms are judged: refusal must leave none (§3, R5 9).
    let target_json = out.join("trust-dependencies.json");
    clear_previous_inventory(&target_json)?;
    let entries = tree_entries(&git, &commit)?;

    // Step 1 of the generated-sources record's order (§2): the forms' text, read from the commit's own blob before
    // anything is written or built. A refusal here ends the run.
    let roots_entry = entries
        .iter()
        .find(|e| e.path == ROOTS && (e.mode == "100644" || e.mode == "100755"))
        .ok_or_else(|| format!("the commit holds no `{ROOTS}`"))?;
    let roots_blob = git.bytes(&["cat-file", "blob", &roots_entry.sha])?;
    let roots = read_roots(&String::from_utf8_lossy(&roots_blob))?;
    if !roots.generated_refused.is_empty() {
        return Ok(refuse(roots.generated_refused.clone()));
    }

    // Step 2: the programs' builds and the data their forms hand, with every refusal this record makes before or in
    // them.
    let symlinks: BTreeSet<String> = entries
        .iter()
        .filter(|e| e.mode == "120000")
        .map(|e| e.path.clone())
        .collect();
    // Every refusal this step makes is reported together (round 8's D3): a root's data and the launcher are judged from
    // the commit's entries first; a refusal that leaves nothing after it safe to judge — a link or a submodule under
    // `catalog/`, no tree written, and a cargo configuration §3 refuses, cargo never run — ends the step with those
    // beside it; every other is judged on to the step's last judgment before a build, and no build follows (the
    // generated-sources record §2, clarified by `M3.5.2.5.2`).
    let blobs: BTreeSet<&str> = entries
        .iter()
        .filter(|e| e.mode == "100644" || e.mode == "100755")
        .map(|e| e.path.as_str())
        .collect();
    let mut refused: Vec<String> = Vec::new();
    // The fixed data a root is handed, by path: a blob of the commit, never a symbolic link (§2, §3; R9 remark 6).
    for p in &roots.programs {
        for d in &p.data {
            if !blobs.contains(d.as_str()) {
                refused.push(format!(
                    "trust-undeclared-input: `{}` declares `{d}`, which is not a blob of the commit",
                    p.name
                ));
            }
        }
    }
    // The launcher an invocation's content names, read with the tree as a root's data is
    // (`decision_trust-role-programs.md` §4).
    for c in roots.front_clauses.iter().filter(|c| c.invokes.is_some()) {
        if !blobs.contains(LAUNCHER) {
            refused.push(format!(
                "trust-undeclared-input: `{}` invokes programs, and its launcher `{LAUNCHER}` is not a blob of the commit",
                c.root
            ));
        }
    }

    let tree: Tree = match tree_of(&git, &entries) {
        Ok(t) => t,
        Err(crate::catalog_check::Failure::Refused(r)) => {
            refused.push(format!("trust-undeclared-input: {r}"));
            return Ok(refuse(refused));
        }
        Err(crate::catalog_check::Failure::Unjudged(e)) => return Err(unable(&refused)(e)),
    };
    let tree_dir = out.join("tree");
    write_tree(&tree, &tree_dir)
        .map_err(|f| format!("{f:?}"))
        .map_err(unable(&refused))?;
    let cargo_home = out.join("cargo-home");
    reset_cargo_home(&cargo_home).map_err(unable(&refused))?; // made anew on every run (§3, R2 B12)
    let pin = toolchain_file(&tree).map_err(unable(&refused))?;
    // A cargo configuration on the build's path that §3 refuses is a refusal of the commit, never a run that could
    // not judge (`M3.6.3.4`, found running the gate built from a base).
    match configurations_on_paths(&tree_dir, &tree_dir, config_roots, &tree) {
        Ok(()) => {}
        Err(crate::catalog_check::Failure::Refused(r)) => {
            refused.push(format!("trust-undeclared-input: {r}"));
            return Ok(refuse(refused));
        }
        Err(crate::catalog_check::Failure::Unjudged(e)) => return Err(unable(&refused)(e)),
    }

    // The workspace's packages, from `cargo metadata`, which runs no package's code.
    let meta_env =
        environment(&pin, &cargo_home, &out.join("target-metadata")).map_err(unable(&refused))?;
    let meta_out = Command::new("cargo")
        // Every feature, so a dependency behind one is an edge as any other (`decision_trust-role-programs.md` §3); the
        // roots' closures hold no feature, which the manifest rules refuse.
        .args([
            "metadata",
            "--offline",
            "--locked",
            "--all-features",
            "--format-version",
            "1",
        ])
        .current_dir(&tree_dir)
        .env_clear()
        .envs(meta_env.iter().map(|(k, v)| (k.as_str(), v.as_str())))
        .output()
        .map_err(|e| format!("`cargo metadata` did not run: {e}"))
        .map_err(unable(&refused))?;
    if !meta_out.status.success() {
        return Err(unable(&refused)(format!(
            "`cargo metadata` failed: {}",
            String::from_utf8_lossy(&meta_out.stderr).trim()
        )));
    }
    let meta = json::parse(&String::from_utf8_lossy(&meta_out.stdout)).map_err(unable(&refused))?;
    let root_prefix = format!(
        "{}/",
        fs::canonicalize(&tree_dir)
            .map_err(|e| e.to_string())
            .map_err(unable(&refused))?
            .display()
    );
    // A package whose manifest lies outside the written tree is one the commit does not hold, reached by the commit's
    // own manifest: it is refused, naming it, before anything is compiled — never read as the root's, whose directory
    // is the empty one (`M3.6.9.6.3.2.2.15.2.1.7.2`) — and judged nowhere else, the step's other judgments made beside
    // it (`M3.5.2.5.2`).
    let outside: Vec<String> = meta
        .get("packages")
        .map(Json::elements)
        .unwrap_or_default()
        .iter()
        .filter_map(|p| {
            let manifest = p
                .get("manifest_path")
                .and_then(Json::as_str)
                .unwrap_or_default();
            (!manifest.starts_with(&root_prefix)).then(|| {
                let name = p.get("name").and_then(Json::as_str).unwrap_or_default();
                format!(
                    "trust-undeclared-input: the commit reaches the package `{name}` at `{manifest}`, outside the \
                     written tree"
                )
            })
        })
        .collect();
    refused.extend(outside);
    let mut package_names: BTreeMap<String, String> = BTreeMap::new();
    let mut id_dir: BTreeMap<String, String> = BTreeMap::new();
    let mut targets: Targets = BTreeMap::new();
    let mut target_sources: TargetSources = BTreeMap::new();
    let mut package_targets: BTreeMap<String, Vec<String>> = BTreeMap::new();
    // Every program target (§2): (package, kind, name).
    let mut program_targets: Vec<(String, String, String)> = Vec::new();
    // Each program target by its crate root, as a generator file names it (the generated-sources record §5).
    let mut program_roots: BTreeMap<String, Vec<(String, String, String)>> = BTreeMap::new();
    // Every target, by package: its kinds and its name, so a root's target is known to exist.
    let mut every_target: BTreeMap<String, Vec<(Vec<String>, String)>> = BTreeMap::new();
    // Every test-like unit (`TEST_KINDS`): (package, kind, name), a program target if it joins two roles.
    let mut test_units: Vec<(String, String, String)> = Vec::new();
    for p in meta.get("packages").map(Json::elements).unwrap_or_default() {
        let manifest_path = p
            .get("manifest_path")
            .and_then(Json::as_str)
            .unwrap_or_default();
        let Some(dir) = manifest_path
            .strip_prefix(&root_prefix)
            .and_then(|r| r.strip_suffix("Cargo.toml"))
            .map(|d| d.trim_end_matches('/').to_owned())
        else {
            continue; // outside the written tree: refused above
        };
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
            for kind in &kinds {
                target_sources
                    .entry((dir.clone(), (*kind).to_owned(), target_name.to_owned()))
                    .or_default()
                    .insert(src.clone());
            }
            // The units some build of this target makes (`decision_trust-role-programs.md` §3): its own, if it is a test
            // or a bench; a library's or an executable's unit tests; a library's doc tests, which rustdoc runs on a
            // `lib`, `rlib` or `proc-macro` alone — cargo refuses a `dylib`'s and a `cdylib`'s (round 7's R4). Flags are
            // not read: `test`, `doctest`, `harness` and `required-features` change what the default build makes, and
            // `cargo test --lib`, `--test` or `--doc` builds the unit whatever they say (measured `2026-10-23`, round 6).
            let doctested = kinds
                .iter()
                .any(|k| matches!(*k, "lib" | "rlib" | "proc-macro"));
            let library = kinds.iter().any(|k| {
                matches!(
                    *k,
                    "lib" | "rlib" | "dylib" | "cdylib" | "staticlib" | "proc-macro"
                )
            });
            for (unit, built) in [
                ("test", kinds.contains(&"test")),
                ("bench", kinds.contains(&"bench")),
                ("lib-test", library),
                ("bin-test", kinds.contains(&"bin")),
                ("doctest", doctested),
            ] {
                if built {
                    test_units.push((dir.clone(), unit.to_owned(), target_name.to_owned()));
                }
            }
            if let Some(kind) = PROGRAM_KINDS.iter().find(|k| kinds.contains(k)) {
                program_targets.push((dir.clone(), (*kind).to_owned(), target_name.to_owned()));
                program_roots.entry(src.clone()).or_default().push((
                    dir.clone(),
                    (*kind).to_owned(),
                    target_name.to_owned(),
                ));
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
    refused.extend(stale);

    // The resolved graph: (dependent, dependency, kind).
    let mut graph: Vec<(String, String, String)> = Vec::new();
    for node in meta
        .get("resolve")
        .and_then(|r| r.get("nodes"))
        .map(Json::elements)
        .unwrap_or_default()
    {
        // A package outside the written tree has no edge here: it is refused above.
        let Some(from) = id_dir
            .get(node.get("id").and_then(Json::as_str).unwrap_or_default())
            .cloned()
        else {
            continue;
        };
        for d in node.get("deps").map(Json::elements).unwrap_or_default() {
            let Some(to) = id_dir
                .get(d.get("pkg").and_then(Json::as_str).unwrap_or_default())
                .cloned()
            else {
                continue;
            };
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

    let mut rules = Rules {
        tree: &tree,
        tree_dir: &tree_dir,
        symlinks: &symlinks,
        admissions: &roots.admissions,
        package_targets: &package_targets,
        metadata_targets: &every_target,
        sites: 0,
        used: BTreeMap::new(),
        manifests_checked: BTreeSet::new(),
        judged: BTreeSet::new(),
    };
    rules.workspace(&mut refused);

    // The manifest rules over every package in a program's closure, from the metadata graph, before any build:
    // a build script or a procedural macro is refused here, so none runs (§3, R2 B12); the harness's development
    // edges are followed, since its build compiles them (R7 3). A program whose package is gone is stale, refused
    // above, with no closure of the commit's to judge.
    for p in roots
        .programs
        .iter()
        .filter(|p| package_names.contains_key(&p.package))
    {
        let mut closure: BTreeSet<String> = BTreeSet::new();
        let mut todo = vec![p.package.clone()];
        while let Some(pkg) = todo.pop() {
            if !closure.insert(pkg.clone()) {
                continue;
            }
            for (from, to, kind) in &graph {
                // The harness's and a derivation's own development edges (`decision_trust-role-programs.md` §3).
                let followed = kind != "dev" || (p.pair.is_some() && *from == p.package);
                if *from == pkg && followed && !closure.contains(to) {
                    todo.push(to.clone());
                }
            }
        }
        rules.manifests(&p.name, &closure, &mut refused);
    }
    if !refused.is_empty() {
        return Ok(refuse(refused));
    }

    // Build each program clean, its own target directory, a fresh `CARGO_HOME` each. A build that fails leaves the
    // gate unable to judge, but the step runs to its end, every build and judgment in it made, so the refusals it
    // finds are reported beside the failure (the generated-sources record §2).
    let compiler = compiler::Compiler::prepare(out, &pin, &cargo_home)?;
    let mut builds: BTreeMap<String, Build> = BTreeMap::new();
    let mut failed: Vec<String> = Vec::new();
    for p in &roots.programs {
        reset_cargo_home(&cargo_home).map_err(unable(&refused))?;
        let target_dir = out.join("target").join(&p.name);
        let env = environment(&pin, &cargo_home, &target_dir).map_err(unable(&refused))?;
        match build(
            p,
            &package_names,
            &targets,
            &target_sources,
            &env,
            &tree_dir,
            &target_dir,
            &compiler,
            &mut refused,
        ) {
            Ok(b) => {
                builds.insert(p.name.clone(), b);
            }
            Err(why) => failed.push(why),
        }
    }

    // The census (`decision_trust-role-programs.md` §3; round 8's change of method): each package's targets built, a
    // test-like target as `cargo test` builds it, without default features as a root is, so what each unit compiles is
    // read from the compiler's dependency information. It runs no package's code: a package holding a build script or a
    // procedural macro, and every package whose closure reaches one, is left out, its units judged by their edges alone.
    // A package whose build fails compiles nothing, so none of its units can compare anything: judged by its edges, it
    // is named in the inventory's `census-failed`. What a census build would refuse of a program is not a program's, so
    // its refusals are dropped; and none is built once a program's build has failed.
    let running: BTreeSet<&String> = every_target
        .iter()
        .filter(|(_, ts)| {
            ts.iter().any(|(kinds, _)| {
                kinds
                    .iter()
                    .any(|k| k == "custom-build" || k == "proc-macro")
            })
        })
        .map(|(pkg, _)| pkg)
        .collect();
    let reaches_code = |pkg: &str| {
        let mut seen: BTreeSet<&str> = BTreeSet::new();
        let mut todo = vec![pkg];
        while let Some(at) = todo.pop() {
            if !seen.insert(at) {
                continue;
            }
            for (from, to, _) in &graph {
                if from == at {
                    todo.push(to);
                }
            }
        }
        seen.iter()
            .any(|s| running.iter().any(|r| r.as_str() == *s))
    };
    let mut census_units: Vec<Unit> = Vec::new();
    let mut census_failed: Vec<String> = Vec::new();
    if failed.is_empty() {
        reset_cargo_home(&cargo_home).map_err(unable(&refused))?;
        let census_dir = out.join("target-census");
        let census_env = environment(&pin, &cargo_home, &census_dir).map_err(unable(&refused))?;
        let args = ["test", "--no-run", "--all-targets"].map(str::to_owned);
        for pkg in package_names.keys().filter(|pkg| !reaches_code(pkg)) {
            match compile(
                &format!("the census of `{pkg}`"),
                pkg,
                &args,
                &package_names,
                &targets,
                &census_env,
                &tree_dir,
                &compiler,
                &mut Vec::new(),
            ) {
                Ok((units, _)) => census_units.extend(units),
                Err(why) => census_failed.push(format!("{pkg}: {why}")),
            }
        }
    }
    let census = Census::of(&census_units, &target_sources);
    // A test-like unit joining two roles is a program target (`decision_trust-role-programs.md` §3).
    program_targets.extend(joined_units(&roots, &test_units, &graph, &census));
    let (program_targets, classifications_unused) =
        classify_programs(&roots, &program_targets, &graph, &census);

    // The token rules over every `.rs` file a compilation read, however it read it (§3; R8 10), each file once.
    let mut compiled: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for (name, b) in &builds {
        for u in &b.units {
            for f in &u.files {
                compiled.entry(f.clone()).or_default().insert(name.clone());
            }
        }
    }
    rules.files(&compiled, &mut refused);

    // A program that runs two roles (§2): any root whose build compiles a role package of another role — a library
    // root too, which runs its role until an executable does (R8 remark 13) — and the harness, whose exemption is its
    // pair's two (R8 remark 12).
    for p in &roots.programs {
        let Some(b) = builds.get(&p.name) else {
            continue; // its build failed: the step ends unable to judge
        };
        let compiled: BTreeSet<&String> = b.units.iter().map(|u| &u.package).collect();
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

    if !failed.is_empty() {
        return Err(crate::trust_generated::beside(failed.join("\n"), &refused));
    }
    if !refused.is_empty() {
        return Ok(refuse(refused));
    }

    // What each program reads (the generated-sources record §1): its compilation's files and the data its form hands
    // it — the harness's through the units it compiles beside its pair's two builds alone.
    let reads: BTreeMap<String, BTreeSet<String>> = roots
        .programs
        .iter()
        .map(|p| {
            let b = &builds[&p.name];
            let units: Vec<&Unit> = match &p.pair {
                Some((x, y)) => beside(b, &builds[x], &builds[y]),
                None => b.units.iter().collect(),
            };
            let files = units
                .iter()
                .flat_map(|u| u.files.iter().cloned())
                .chain(p.data.iter().cloned())
                .collect();
            (p.name.clone(), files)
        })
        .collect();
    let live = crate::trust_generated::live(&roots.generated, &reads);
    // Step 3 (§2): over live forms, the blob rule and the chain's first clause. A refusal here ends the run.
    let step3 = crate::trust_generated::blobs_and_chains(&roots.generated, &live, &tree, &symlinks);
    if !step3.is_empty() {
        return Ok(refuse(step3));
    }
    // Each program's provenance (§4): the generator files and inputs of every live form whose declared file it reads.
    let provenance: BTreeMap<String, crate::trust_generated::Provenances> = roots
        .programs
        .iter()
        .map(|p| {
            (
                p.name.clone(),
                crate::trust_generated::provenance(&live, &reads[&p.name], &tree),
            )
        })
        .collect();

    // Step 4 (the generated-sources record §2, §5): each live form's generator files, judged against each program that
    // reads its declared file. A program target's crate root is refused when its role packages hold another role's,
    // and is otherwise built as a root is — manifest rules first, its sites judged with the programs' — unless it is a
    // root, whose build it is; a `.rs` file that is no program target's crate root is refused; a script is judged by
    // the innermost workspace member holding it. The step runs to its end; a build that fails leaves it unable to judge.
    let mut step4: Vec<String> = Vec::new();
    let mut generator_failed: Vec<String> = Vec::new();
    // What each generator build compiled, by file, with the generator files whose builds read it: step 5's.
    let mut generator_reads: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut generator_built: BTreeSet<(String, String, String)> = BTreeSet::new();
    let role_of_package: BTreeMap<&String, BTreeSet<&str>> = {
        let mut m: BTreeMap<&String, BTreeSet<&str>> = BTreeMap::new();
        for r in roots.programs.iter().filter(|r| r.pair.is_none()) {
            for rp in &r.role_packages {
                m.entry(rp).or_default().insert(r.role.as_str());
            }
        }
        m
    };
    let own_roles = |p: &Program| -> BTreeSet<String> {
        match &p.pair {
            Some((x, y)) => roots
                .programs
                .iter()
                .filter(|r| r.name == *x || r.name == *y)
                .map(|r| r.role.clone())
                .collect(),
            None => [p.role.clone()].into(),
        }
    };
    let all_role_packages: BTreeSet<&String> = role_of_package.keys().copied().collect();
    for g in &live {
        let readers: Vec<&Program> = roots
            .programs
            .iter()
            .filter(|p| reads[&p.name].contains(&g.path))
            .collect();
        for file in &g.generators {
            // What another role holds of `held`, for each reader: (reader, package, role).
            let foreign = |held: &BTreeSet<String>| -> Vec<(String, String, String)> {
                let mut out = Vec::new();
                for p in &readers {
                    let mine = own_roles(p);
                    for pkg in held {
                        for role in role_of_package.get(pkg).into_iter().flatten() {
                            if !mine.contains(*role) {
                                out.push((p.name.clone(), pkg.clone(), (*role).to_owned()));
                            }
                        }
                    }
                }
                out
            };
            if let Some(owned) = program_roots.get(file) {
                let what = owned
                    .iter()
                    .map(|(pkg, kind, name)| format!("the {kind} `{name}` of `{pkg}`"))
                    .collect::<Vec<_>>()
                    .join(" and ");
                // The union of its targets' role packages, as the parent computes them for every program target.
                let held: BTreeSet<String> = owned
                    .iter()
                    .flat_map(|(pkg, kind, name)| {
                        compiled_role_packages(&roots, &census, &graph, pkg, kind, name)
                    })
                    .filter(|pkg| all_role_packages.contains(pkg))
                    .collect();
                let refused_here = foreign(&held);
                for (reader, pkg, role) in &refused_here {
                    step4.push(format!(
                        "trust-shared-program: `{reader}` reads `{}`, generated by `{file}`, the crate root of {what}, whose \
                         build compiles `{pkg}`, a role package of the {role} — not built",
                        g.path
                    ));
                }
                if !refused_here.is_empty() {
                    continue;
                }
                for (pkg, kind, name) in owned {
                    let target = (pkg.clone(), kind.clone(), name.clone());
                    // A generator target that is itself a root is that root's own build.
                    let root = roots.programs.iter().find(|r| {
                        r.pair.is_none()
                            && r.package == *pkg
                            && (r.target == Target::Bin(name.clone())
                                || (r.target == Target::Lib && kind != "bin" && kind != "example"))
                    });
                    if let Some(r) = root {
                        for u in &builds[&r.name].units {
                            for f in &u.files {
                                generator_reads
                                    .entry(f.clone())
                                    .or_default()
                                    .insert(file.clone());
                            }
                        }
                        continue;
                    }
                    if !generator_built.insert(target.clone()) {
                        continue;
                    }
                    let who = format!("generator {file}");
                    let before = step4.len();
                    rules.manifests(&who, &target_closure(pkg, kind, &graph), &mut step4);
                    if step4.len() > before {
                        continue; // refused before its build, so nothing of it runs
                    }
                    let target_args: Vec<String> = match kind.as_str() {
                        "bin" => vec!["build".into(), "--bin".into(), name.clone()],
                        "example" => vec!["build".into(), "--example".into(), name.clone()],
                        _ => vec!["build".into(), "--lib".into()],
                    };
                    reset_cargo_home(&cargo_home)?;
                    let target_dir = out
                        .join("target-generators")
                        .join(format!("{}-{kind}-{name}", pkg.replace('/', "-")));
                    let env = environment(&pin, &cargo_home, &target_dir)?;
                    match compile(
                        &who,
                        pkg,
                        &target_args,
                        &package_names,
                        &targets,
                        &env,
                        &tree_dir,
                        &compiler,
                        &mut step4,
                    ) {
                        Ok((units, _)) => {
                            for u in &units {
                                for f in &u.files {
                                    generator_reads
                                        .entry(f.clone())
                                        .or_default()
                                        .insert(file.clone());
                                }
                            }
                        }
                        Err(why) => generator_failed.push(why),
                    }
                }
            } else if std::path::Path::new(file)
                .extension()
                .is_some_and(|e| e == "rs")
            {
                step4.push(format!(
                    "trust-undeclared-input: {ROOTS}: `(defgenerated \"{}\" …)`'s generator `{file}` is a `.rs` file \
                     that is no program target's crate root — a generator file is a script or a program target's crate \
                     root, whose build the instrument computes",
                    g.path
                ));
            } else if let Some(member) =
                crate::trust_generated::innermost_member(file, package_names.keys())
            {
                for (reader, pkg, role) in foreign(&[member.clone()].into()) {
                    step4.push(format!(
                        "trust-shared-program: `{reader}` reads `{}`, generated by `{file}`, which lies in `{pkg}`, a \
                         role package of the {role}",
                        g.path
                    ));
                }
            }
        }
    }
    // The catalog's token rules and admissions over every file a generator build read that no program's did.
    let generator_compiled: BTreeMap<String, BTreeSet<String>> = generator_reads
        .iter()
        .map(|(f, gens)| {
            (
                f.clone(),
                gens.iter().map(|g| format!("generator {g}")).collect(),
            )
        })
        .collect();
    rules.files(&generator_compiled, &mut step4);
    if !generator_failed.is_empty() {
        return Err(crate::trust_generated::beside(
            generator_failed.join("\n"),
            &step4,
        ));
    }
    if !step4.is_empty() {
        return Ok(refuse(step4));
    }

    // Step 5 (the generated-sources record §2, §3): over what a generator build reads, the chain's second clause — a
    // file a form declares or whose header marks it generated; then over what each program reads, the recogniser's
    // refusal — a file whose header marks it generated that no form declares and no chain holds, one message for one
    // file. A refusal here ends the run.
    let declared: BTreeSet<&str> = roots.generated.iter().map(|g| g.path.as_str()).collect();
    let marked = |f: &str| {
        tree.get(f)
            .is_some_and(|b| crate::generated_header::marked(f, b))
    };
    let mut step5: Vec<String> = Vec::new();
    for (f, generators) in &generator_reads {
        let why = if declared.contains(f.as_str()) {
            "a `defgenerated` form declares it"
        } else if marked(f) {
            "its header marks it generated"
        } else {
            continue;
        };
        for g in generators {
            step5.push(format!(
                "trust-undeclared-input: `{f}`, which the build of the generator `{g}` reads, is a generated source — \
                 {why} — a chain of generators, refused (decision_trust-generated-refusals.md §2)"
            ));
        }
    }
    for p in &roots.programs {
        for f in &reads[&p.name] {
            if marked(f) && !declared.contains(f.as_str()) && !generator_reads.contains_key(f) {
                step5.push(format!(
                    "trust-undeclared-input: `{}` reads `{f}`, whose header marks it generated and which no \
                     `defgenerated` form declares — a generated source with no declaration",
                    p.name
                ));
            }
        }
    }
    if !step5.is_empty() {
        return Ok(refuse(step5));
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
                            && (kind != "dev" || p.pair.is_some() && *from == p.package)
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
            // The harness's two roots, so a package's reference-model result can name the pair's harness as its
            // producer (`trust-verify`, `M3.6.3.5`); empty for a root.
            (
                "pair",
                strings(p.pair.iter().flat_map(|(x, y)| [x.clone(), y.clone()])),
            ),
            (
                "artifact",
                obj(vec![
                    ("path", s(&b.artifact.0)),
                    ("sha256", s(&b.artifact.1)),
                ]),
            ),
            ("packages", Json::Array(package_records)),
            (
                "provenance",
                crate::trust_generated::provenance_json(&provenance[&p.name]),
            ),
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
            // A derivation is paired with no root: its pair takes its item (`decision_trust-role-programs.md` §3).
            if a.role == "derivation" || b.role == "derivation" {
                continue;
            }
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
            // The comparison harness a pair's form names is that pair's shared item (§4, R2 B7), and so is each
            // derivation its forms name, one item a form (`decision_trust-role-programs.md` §3).
            let comparisons: Vec<&Program> = roots
                .programs
                .iter()
                .filter(|h| {
                    h.pair.as_ref().is_some_and(|(x, y)| {
                        (x == &a.name && y == &b.name) || (x == &b.name && y == &a.name)
                    })
                })
                .collect();
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
                                && (kind != "dev" || prog.pair.is_some() && *from == prog.package)
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
            // What the two share through committed generated sources (the generated-sources record §5).
            items.extend(crate::trust_generated::shared_items(
                &crate::trust_generated::Side {
                    provenance: &provenance[&a.name],
                    reads: &reads[&a.name],
                },
                &crate::trust_generated::Side {
                    provenance: &provenance[&b.name],
                    reads: &reads[&b.name],
                },
                &tree,
            ));
            for h in comparisons {
                // Every unit it compiles beside the pair's two roots' builds (§4, R8 7), and the provenance it reaches
                // through them, an aspect of the item as its files are (the generated-sources record §5).
                let beside = beside(&builds[&h.name], ba, bb);
                let content: BTreeSet<String> = beside
                    .iter()
                    .flat_map(|u| u.files.iter().map(|f| format!("{f}={}", file_hash(f))))
                    .collect();
                let (kind, key) = if h.role == "derivation" {
                    ("shared-derivation", "derivation")
                } else {
                    ("comparison-harness", "harness")
                };
                items.push(obj(vec![
                    ("kind", s(kind)),
                    (key, s(&h.name)),
                    ("content", strings(content)),
                    (
                        "configuration",
                        strings(beside.iter().map(|u| unit_configuration(u))),
                    ),
                    (
                        "provenance",
                        crate::trust_generated::provenance_json(&provenance[&h.name]),
                    ),
                ]));
            }
            // The front and each root its form invokes share an invocation: the invoked root and the launcher
            // (`decision_trust-role-programs.md` §4).
            for (front, invoked) in [(a, b), (b, a)] {
                let invokes = roots
                    .front_clauses
                    .iter()
                    .find(|c| c.root == front.name)
                    .and_then(|c| c.invokes.as_ref());
                if invokes.is_some_and(|names| names.contains(&invoked.name)) {
                    let target = match &invoked.target {
                        Target::Bin(name) => format!("bin {name}"),
                        Target::Lib => "lib".to_owned(),
                        Target::Test(name) => format!("test {name}"),
                    };
                    items.push(obj(vec![
                        ("kind", s("invocation")),
                        (
                            "content",
                            strings([
                                format!("invoked={} {} {target}", invoked.name, invoked.role),
                                format!("launcher={LAUNCHER}={}", file_hash(LAUNCHER)),
                            ]),
                        ),
                    ]));
                }
            }
            shared.push(obj(vec![
                ("pair", strings([a.name.clone(), b.name.clone()])),
                ("items", Json::Array(items)),
            ]));
        }
    }

    // Every file a program reads whose header marks it as generated (`generated_header`; the generated-sources record
    // §3): listed, so that record's census is this instrument's own output — each declared, or step 5 refused it.
    let mut generated_marked: BTreeSet<String> = BTreeSet::new();
    let read_by_programs = builds
        .values()
        .flat_map(|b| b.units.iter().flat_map(|u| u.files.iter()))
        .chain(roots.programs.iter().flat_map(|p| p.data.iter()));
    for f in read_by_programs {
        if tree
            .get(f)
            .is_some_and(|bytes| crate::generated_header::marked(f, bytes))
        {
            generated_marked.insert(f.clone());
        }
    }

    // An admission no current site uses: listed here, refused by the gate on the baseline's host as
    // `trust-baseline-stale` (§6, R8 remark 15).
    let unused: Vec<String> = roots
        .admissions
        .iter()
        .flat_map(|(a, n)| {
            let left = n.saturating_sub(rules.used.get(a).copied().unwrap_or(0));
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
        ("refused-sites", s(rules.sites.to_string())),
        ("admissions-unused", strings(unused)),
        ("generated-marked", strings(generated_marked)),
        // Every declared file no program reads: stale on the baseline's host, decided by the gate (the record §6).
        (
            "generated-unread",
            strings(
                roots
                    .generated
                    .iter()
                    .filter(|g| !live.iter().any(|l| l.path == g.path))
                    .map(|g| g.path.clone())
                    .collect::<BTreeSet<_>>(),
            ),
        ),
        ("program-targets", Json::Array(program_targets)),
        ("census-failed", strings(census_failed)),
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

/// An old Cargo home can contain configuration that changes what executes. Only an absent home is harmless;
/// any other removal failure must stop the run before an environment can reuse it (§3, R2 B12).
fn reset_cargo_home(home: &Path) -> Result<(), String> {
    match fs::remove_dir_all(home) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "cannot reset inventory Cargo home `{}`: {error}",
            home.display()
        )),
    }
}

/// Clear a previous result before forms can refuse. A removal failure can leave that file readable, so report
/// the failed invocation before classifying its input; only a missing file is harmless (§3, R5 9).
fn clear_previous_inventory(path: &Path) -> Result<(), String> {
    match fs::remove_file(path) {
        Ok(()) => Ok(()),
        Err(error) if error.kind() == std::io::ErrorKind::NotFound => Ok(()),
        Err(error) => Err(format!(
            "cannot remove previous inventory `{}`: {error}",
            path.display()
        )),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    //! Every channel a review measured by hand, a fixture (ledger `TI-H24`): a scratch repository under
    //! `target/trust-tests/`, committed, inventoried by the real instrument from its commit.

    use super::{
        inventory_with, read_roots, site_extent, Digest, FrontClauses, Outcome, Target, FORMS,
        TEST_KINDS,
    };
    use crate::json::Json;
    use std::ffi::OsStr;
    use std::path::{Path, PathBuf};
    use std::process::Command;

    // Historical verbose-format probes only; production uses private compiler records.
    /// Split one `rustc` command line as cargo's `-v` prints it, decoding single-quoted spans and outside escapes.
    fn split_command(line: &str) -> Vec<String> {
        let mut out = Vec::new();
        let mut cur = String::new();
        let mut quoted = false;
        let mut any = false;
        let mut chars = line.chars();
        while let Some(c) = chars.next() {
            match c {
                '\\' if !quoted => cur.push(chars.next().unwrap_or('\\')),
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

    /// Frame Cargo's complete verbose records before decoding their arguments. The pinned Unix formatter can
    /// put literal line breaks and backticks inside single-quoted arguments. Only an unquoted, unescaped backtick
    /// ends a record; embedded Running lines stay inside it. Malformed records refuse rather than slicing blindly.
    fn cargo_commands<'a>(
        mut stderr: &'a str,
        scratch: &str,
        canonical_scratch: &str,
    ) -> Result<Vec<&'a str>, String> {
        let mut commands = Vec::new();
        while !stderr.is_empty() {
            let line_end = stderr.find('\n').unwrap_or(stderr.len());
            let line = &stderr[..line_end];
            let Some(command) = line
                .trim_start_matches([' ', '\t'])
                .strip_prefix("Running `")
            else {
                // Cargo's Compiling/Checking status prints package paths without quoting. Consume any known raw
                // scratch prefix on this non-command line intact, so its embedded Running line is not a record.
                let prefix_end = [scratch, canonical_scratch]
                    .into_iter()
                    .filter(|prefix| !prefix.is_empty())
                    .filter_map(|prefix| {
                        stderr
                            .find(prefix)
                            .filter(|start| *start < line_end)
                            .map(|start| start + prefix.len())
                    })
                    .max()
                    .unwrap_or(line_end)
                    .max(line_end);
                let skip = prefix_end
                    + stderr[prefix_end..]
                        .find('\n')
                        .unwrap_or(stderr.len() - prefix_end);
                stderr = &stderr[(skip + 1).min(stderr.len())..];
                continue;
            };
            let rest = &stderr[line.len() - command.len()..];
            let mut quoted = false;
            let mut escaped = false;
            let mut end = None;
            for (index, c) in rest.char_indices() {
                if matches!(c, '\n' | '\r') && !quoted {
                    return Err("unquoted line break in Cargo Running command".to_owned());
                }
                if escaped {
                    escaped = false;
                    continue;
                }
                match c {
                    '\\' if !quoted => escaped = true,
                    '\'' => quoted = !quoted,
                    '`' if !quoted => {
                        end = Some(index);
                        break;
                    }
                    _ => {}
                }
            }
            let end = end.ok_or_else(|| "unterminated Cargo Running command".to_owned())?;
            if end == 0 {
                return Err("empty Cargo Running command".to_owned());
            }
            let after = &rest[end + 1..];
            let next = after.find('\n').unwrap_or(after.len());
            if !after[..next].trim_matches([' ', '\t', '\r']).is_empty() {
                return Err("trailing text after Cargo Running command".to_owned());
            }
            commands.push(&rest[..end]);
            stderr = &after[(next + 1).min(after.len())..];
        }
        Ok(commands)
    }

    pub(crate) fn real_root() -> PathBuf {
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .to_path_buf()
    }

    fn bound_configuration_roots(
        real: &Path,
        source: Option<&OsStr>,
    ) -> Result<Vec<PathBuf>, String> {
        let real = real
            .canonicalize()
            .map_err(|e| format!("cannot canonicalize the test root: {e}"))?;
        let Some(source) = source else {
            return Ok(vec![real]);
        };
        let source = Path::new(source);
        if !source.is_absolute() {
            return Err("the sweep source must name an absolute repository root".to_owned());
        }
        let source = source
            .canonicalize()
            .map_err(|e| format!("cannot canonicalize the sweep source: {e}"))?;
        // Discover the directory's own repository, not a work tree selected by inherited GIT_* overrides.
        let mut command = Command::new("git");
        command
            .args([
                "-c",
                "core.commitGraph=false",
                "rev-parse",
                "--show-toplevel",
            ])
            .current_dir(&source)
            .env_clear()
            .env("GIT_NO_REPLACE_OBJECTS", "1");
        for key in ["PATH", "HOME"] {
            if let Some(value) = std::env::var_os(key) {
                command.env(key, value);
            }
        }
        let output = command
            .output()
            .map_err(|e| format!("cannot discover the sweep source's Git root: {e}"))?;
        if !output.status.success() {
            return Err("the sweep source is not a Git working tree".to_owned());
        }
        let top = String::from_utf8(output.stdout)
            .map_err(|_| "the sweep source's Git root is not UTF-8".to_owned())?;
        let top = Path::new(top.strip_suffix('\n').unwrap_or(&top))
            .canonicalize()
            .map_err(|e| format!("cannot canonicalize the sweep source's Git root: {e}"))?;
        if top != source {
            return Err("the sweep source must be its Git top-level".to_owned());
        }
        if real != source.join("target/mutation-sweep/tree") {
            return Err(
                "the test root must be exactly the source's target/mutation-sweep/tree child"
                    .to_owned(),
            );
        }
        Ok(vec![real, source])
    }

    pub(crate) fn configuration_roots() -> Vec<PathBuf> {
        bound_configuration_roots(
            &real_root(),
            std::env::var_os("ARCHOGEN_SWEEP_SOURCE").as_deref(),
        )
        .unwrap_or_else(|why| panic!("ARCHOGEN_SWEEP_SOURCE refused: {why}"))
    }

    fn binding_layout(name: &str) -> (PathBuf, PathBuf, PathBuf) {
        let base = real_root().join("target/trust-tests").join(name);
        let _ = std::fs::remove_dir_all(&base);
        let source = base.join("source");
        let copy = source.join("target/mutation-sweep/tree");
        std::fs::create_dir_all(&copy).unwrap();
        git(&source, &["init", "-q"]);
        (
            base,
            source.canonicalize().unwrap(),
            copy.canonicalize().unwrap(),
        )
    }

    #[test]
    fn sweep_source_binding_accepts_unset_exact_and_canonical_aliases() {
        crate::trust_test_process::bounded!(
            sweep_source_binding_accepts_unset_exact_and_canonical_aliases
        );
        let (base, source, copy) = binding_layout("source-binding-positive");
        for root in [real_root(), copy.clone()] {
            assert_eq!(
                bound_configuration_roots(&root, None).unwrap(),
                [root.canonicalize().unwrap()]
            );
        }
        let expected = vec![copy.clone(), source.clone()];
        assert_eq!(
            bound_configuration_roots(&copy, Some(source.as_os_str())).unwrap(),
            expected
        );
        #[cfg(unix)]
        {
            let source_alias = base.join("source-alias");
            let copy_alias = base.join("copy-alias");
            std::os::unix::fs::symlink(&source, &source_alias).unwrap();
            std::os::unix::fs::symlink(&copy, &copy_alias).unwrap();
            assert_eq!(
                bound_configuration_roots(&copy, Some(source_alias.as_os_str())).unwrap(),
                expected
            );
            assert_eq!(
                bound_configuration_roots(&copy_alias, Some(source.as_os_str())).unwrap(),
                expected
            );
        }
    }

    #[test]
    fn sweep_source_binding_refuses_malformed_unrelated_and_merely_containing_roots() {
        crate::trust_test_process::bounded!(
            sweep_source_binding_refuses_malformed_unrelated_and_merely_containing_roots
        );
        let (base, source, copy) = binding_layout("source-binding-negative");
        let other = base.join("other");
        std::fs::create_dir_all(&other).unwrap();
        git(&other, &["init", "-q"]);
        let nested = source.join("nested");
        let nongit = base.join("not-a-git-root");
        std::fs::create_dir_all(&nested).unwrap();
        std::fs::create_dir_all(&nongit).unwrap();
        for candidate in [
            PathBuf::new(),
            PathBuf::from("."),
            base.join("nul\0"),
            base.join("missing"),
            nongit,
            source.join("target"),
            nested,
            other,
            real_root(),
        ] {
            assert!(
                bound_configuration_roots(&copy, Some(candidate.as_os_str())).is_err(),
                "unexpectedly admitted source {candidate:?}"
            );
        }
        for wrong_copy in [copy.join("deeper"), base.join("beside")] {
            std::fs::create_dir_all(&wrong_copy).unwrap();
            assert!(bound_configuration_roots(&wrong_copy, Some(source.as_os_str())).is_err());
        }
        // Exact layout alone cannot authorize a directory nested inside a different Git root.
        let intermediate = source.join("intermediate");
        let intermediate_copy = intermediate.join("target/mutation-sweep/tree");
        std::fs::create_dir_all(&intermediate_copy).unwrap();
        assert!(
            bound_configuration_roots(&intermediate_copy, Some(intermediate.as_os_str()))
                .unwrap_err()
                .contains("Git top-level")
        );
        #[cfg(unix)]
        {
            // Canonicalizing the expected child too would incorrectly admit this redirected physical location.
            let escaped = base.join("redirected-copy");
            std::fs::rename(&copy, &escaped).unwrap();
            std::os::unix::fs::symlink(&escaped, &copy).unwrap();
            assert!(bound_configuration_roots(&copy, Some(source.as_os_str())).is_err());
        }
    }

    #[test]
    fn sweep_source_environment_is_validated_before_fixture_inventory() {
        crate::trust_test_process::bounded!(
            sweep_source_environment_is_validated_before_fixture_inventory
        );
        const CHILD: &str = "ARCHOGEN_SOURCE_BINDING_CHILD";
        if std::env::var_os(CHILD).is_some() {
            configuration_roots();
            return;
        }
        let output = Command::new(std::env::current_exe().unwrap())
            .args([
                "--exact",
                "trust::tests::sweep_source_environment_is_validated_before_fixture_inventory",
                "--nocapture",
            ])
            .env(CHILD, "1")
            .env("ARCHOGEN_SWEEP_SOURCE", real_root())
            .output()
            .unwrap();
        assert_eq!(
            output.status.code(),
            Some(101),
            "invalid source unexpectedly admitted"
        );
        let diagnostic = String::from_utf8_lossy(&output.stderr);
        assert!(
            diagnostic.contains("ARCHOGEN_SWEEP_SOURCE refused:"),
            "{diagnostic}"
        );
    }

    pub(crate) fn git(dir: &Path, args: &[&str]) {
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

    pub(crate) const WS: &str = "[workspace]\nresolver = \"2\"\nmembers = [\"crates/*\"]\n";

    pub(crate) fn manifest(name: &str, extra: &str) -> String {
        format!("[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n{extra}")
    }

    /// Two roots, a binary `a` and a library `b`, as every fixture starts.
    pub(crate) const ROOTS: &str = "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-packages \"crates/a\"))\n\
                         (defroot chk (role scheduling-checker) (package \"crates/b\") (target lib) (role-packages \"crates/b\"))\n";

    /// A reference model and the implementation it validates, the pair a harness compares
    /// (`decision_trust-role-programs.md` §3).
    pub(crate) const PAIRED: &str =
        "(defroot refm (role reference-model) (package \"crates/r\") (target lib))\n\
                         (defroot imp (role implementation) (package \"crates/i\") (target lib))\n";

    fn write(dir: &Path, files: &[(&str, String)]) {
        for (path, text) in files {
            let full = dir.join(path);
            std::fs::create_dir_all(full.parent().unwrap()).unwrap();
            std::fs::write(full, text).unwrap();
        }
    }

    /// A committed scratch repository holding `files`, the real pin and configuration, and a lock cargo writes.
    pub(crate) fn fixture(name: &str, toolchain: &str, files: &[(&str, String)]) -> Fixture {
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
        /// The run as the inventory returns it: an error where the build itself fails, before any judgment.
        pub(crate) fn try_run_into(&self, out: &str) -> Result<Outcome, String> {
            inventory_with(
                &self.repo,
                "HEAD",
                &self.base.join(out),
                &configuration_roots(),
            )
        }
        pub(crate) fn run_into(&self, out: &str) -> Outcome {
            self.try_run_into(out).unwrap()
        }
        pub(crate) fn run(&self) -> Outcome {
            self.run_into("out")
        }
    }

    pub(crate) fn written(o: Outcome) -> Json {
        match o {
            Outcome::Written(j) => j,
            Outcome::Refused(r) => panic!("refused: {r:#?}"),
        }
    }

    pub(crate) fn refused(o: Outcome) -> Vec<String> {
        match o {
            Outcome::Written(_) => panic!("written where a refusal was due"),
            Outcome::Refused(r) => r,
        }
    }

    pub(crate) fn says(refusals: &[String], text: &str) {
        assert!(
            refusals.iter().any(|r| r.contains(text)),
            "no refusal says `{text}`: {refusals:#?}"
        );
    }

    pub(crate) fn items(inv: &Json) -> Vec<(String, Json)> {
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

    pub(crate) fn kind(item: &Json) -> &str {
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
        crate::trust_test_process::bounded!(
            every_program_target_is_a_root_or_classified_and_the_rest_is_reported
        );
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
    fn every_harness_pair_names_two_distinct_roots() {
        crate::trust_test_process::bounded!(every_harness_pair_names_two_distinct_roots);
        let forms = |pairs: [(&str, &str); 3]| {
            let harnesses: String = pairs
                .iter()
                .enumerate()
                .map(|(i, (a, b))| {
                    format!(
                        "(defharness h{i} (pair {a} {b}) (package \"crates/tests\") (test d{i}))\n"
                    )
                })
                .collect();
            format!("{ROOTS}{PAIRED}{harnesses}")
        };
        let valid = read_roots(&forms([("refm", "imp"); 3]))
            .expect("every harness compares two declared roots");
        assert_eq!(valid.programs.len(), 7);
        for harness in &valid.programs[4..] {
            assert_eq!(harness.pair, Some(("refm".to_owned(), "imp".to_owned())));
        }

        for position in 0..3 {
            for root in ["refm", "imp"] {
                let mut pairs = [("refm", "imp"); 3];
                pairs[position] = (root, root);
                let error = read_roots(&forms(pairs))
                    .expect_err("every harness must compare distinct roots");
                assert!(
                    error.contains(&format!("`h{position}`'s pair names `{root}` twice")),
                    "{position}: {error}"
                );
            }
        }
    }

    #[test]
    fn every_harness_pair_member_names_a_root() {
        crate::trust_test_process::bounded!(every_harness_pair_member_names_a_root);
        let forms = |pairs: [(&str, &str); 3]| {
            let harnesses: String = pairs
                .iter()
                .enumerate()
                .map(|(i, (a, b))| {
                    format!(
                        "(defharness h{i} (pair {a} {b}) (package \"crates/tests\") (test d{i}))\n"
                    )
                })
                .collect();
            format!("{ROOTS}{PAIRED}{harnesses}")
        };
        let valid = read_roots(&forms([("refm", "imp"); 3]))
            .expect("both members of every pair name declared roots");
        assert_eq!(valid.programs.len(), 7);

        for position in 0..3 {
            for missing_pair in [("missing", "imp"), ("refm", "missing")] {
                let mut pairs = [("refm", "imp"); 3];
                pairs[position] = missing_pair;
                let error = read_roots(&forms(pairs))
                    .expect_err("every member of every harness pair must name a root");
                assert!(
                    error.contains(&format!(
                        "`h{position}`'s pair names `missing`, which names no root"
                    )),
                    "{position}: {error}"
                );
            }
        }
    }

    #[test]
    fn a_harness_pair_cannot_name_another_harness() {
        crate::trust_test_process::bounded!(a_harness_pair_cannot_name_another_harness);
        let forms = |a: &str, b: &str| {
            format!(
                "{ROOTS}{PAIRED}(defharness other (pair refm imp) (package \"crates/tests\") (test other))\n\
                 (defharness candidate (pair {a} {b}) (package \"crates/tests\") (test candidate))\n"
            )
        };
        let valid = read_roots(&forms("refm", "imp"))
            .expect("several harnesses may compare the declared roots");
        assert_eq!(valid.programs.len(), 6);

        for (a, b) in [("other", "refm"), ("refm", "other")] {
            let error =
                read_roots(&forms(a, b)).expect_err("a declared harness is not a declared root");
            assert!(
                error.contains("`candidate`'s pair names `other`, which names no root"),
                "{a}, {b}: {error}"
            );
        }
    }

    #[test]
    fn a_harness_compares_a_reference_model_and_its_implementation_alone() {
        crate::trust_test_process::bounded!(
            a_harness_compares_a_reference_model_and_its_implementation_alone
        );
        // `decision_trust-role-programs.md` §3: a harness for any pair could take a comparison a derivation is for,
        // paired with every root and accepted as a producer; its pair's roles decide it, in either order.
        let harness = |a: &str, b: &str| {
            format!(
                "{ROOTS}{PAIRED}(defharness h (pair {a} {b}) (package \"crates/t\") (test h))\n"
            )
        };
        for (a, b) in [("refm", "imp"), ("imp", "refm")] {
            read_roots(&harness(a, b)).expect("a reference model and its implementation");
        }
        for (a, b) in [("gen", "chk"), ("gen", "imp"), ("refm", "chk")] {
            let e = read_roots(&harness(a, b)).expect_err("another pair of roles");
            assert!(
                e.contains("`h`'s pair has roles")
                    && e.contains("a harness's has `implementation` and `reference-model`"),
                "{a}, {b}: {e}"
            );
        }
    }

    /// The generator, both checkers, a reference model and an implementation, with a derivation over `pair`.
    fn derivation(pair: &str, extra: &str) -> String {
        format!(
            "{ROOTS}{PAIRED}(defroot cfg (role configuration-checker) (package \"crates/c\") (target lib))\n\
             (defderivation d (pair {pair}) (package \"crates/r\") (test production) {extra})\n"
        )
    }

    #[test]
    fn a_derivation_compares_the_generator_and_a_checker() {
        crate::trust_test_process::bounded!(a_derivation_compares_the_generator_and_a_checker);
        // `decision_trust-role-programs.md` §3: its pair's roles — the generator and either checker, round 5's D2 — in
        // either order, and a reason; every departure a read failure, as a harness's.
        for pair in ["gen cfg", "cfg gen", "gen chk", "chk gen"] {
            let roots = read_roots(&derivation(pair, "(reason \"the relation's comparison\")"))
                .expect("the generator and the configuration checker");
            let d = roots
                .programs
                .iter()
                .find(|p| p.name == "d")
                .expect("the derivation is read");
            assert_eq!(d.role, "derivation");
            assert_eq!(d.target, Target::Test("production".to_owned()));
        }
        let reason = "(reason \"r\")";
        for (pair, extra, why) in [
            (
                "cfg chk",
                reason,
                "`d`'s pair has roles `configuration-checker` and `scheduling-checker`; a derivation's has `generator` \
                 and `configuration-checker` or `scheduling-checker`",
            ),
            (
                "gen refm",
                reason,
                "`d`'s pair has roles `generator` and `reference-model`; a derivation's has",
            ),
            ("gen gen", reason, "`d`'s pair names `gen` twice"),
            (
                "gen nobody",
                reason,
                "`d`'s pair names `nobody`, which names no root",
            ),
            ("gen cfg", "", "`d` needs exactly one `(reason …)`"),
        ] {
            let e = read_roots(&derivation(pair, extra)).expect_err(why);
            assert!(e.contains(why), "{pair} {extra}: {e}");
        }
    }

    #[test]
    fn one_test_target_is_one_pair_form_s() {
        crate::trust_test_process::bounded!(one_test_target_is_one_pair_form_s);
        // `decision_trust-role-programs.md` §3: a comparison is written one way, so a harness and a derivation, or two
        // of either, never name one test target of one package.
        let forms = |harness_package: &str, harness_test: &str| {
            format!(
                "{ROOTS}{PAIRED}(defroot cfg (role configuration-checker) (package \"crates/c\") (target lib))\n\
                 (defderivation d (pair gen cfg) (package \"crates/r\") (test production) (reason \"r\"))\n\
                 (defharness h (pair refm imp) (package \"{harness_package}\") (test {harness_test}))\n"
            )
        };
        read_roots(&forms("crates/r", "other")).expect("another target of the package");
        read_roots(&forms("crates/i", "production"))
            .expect("a target of that name in another package");
        let e = read_roots(&forms("crates/r", "production")).expect_err("one target, two forms");
        assert!(
            e.contains("`d` and `h` name one test target, `production` of `crates/r`"),
            "{e}"
        );
        // A classification of a test a pair form names says its comparison compares nothing (round 6's R9).
        let classified = format!(
            "{}(defprogram p (package \"crates/r\") (target test production) (reason \"r\"))\n",
            forms("crates/r", "other")
        );
        let e = read_roots(&classified).expect_err("a comparison classified");
        assert!(
            e.contains("`p` classifies the test `production` of `crates/r`, which `d` names as a comparison"),
            "{e}"
        );
        read_roots(&format!(
            "{}(defprogram p (package \"crates/r\") (target test another) (reason \"r\"))\n",
            forms("crates/r", "other")
        ))
        .expect("a test no pair form names may be classified");
    }

    #[test]
    fn a_pair_form_failure_found_once_the_file_is_read_has_every_defgenerated_refusal_beside_it() {
        crate::trust_test_process::bounded!(
            a_pair_form_failure_found_once_the_file_is_read_has_every_defgenerated_refusal_beside_it
        );
        // Round 8's R12; the generated-sources record §2: a departure the reader finds only once the whole file is read
        // has every `defgenerated` form's refusals beside it, those after it as those before.
        let around = |forms: &str| {
            format!(
                "(defgenerated \"before.rs\" (generator \"g\") (command \"c\"))\n{ROOTS}{PAIRED}{forms}\
                 (defgenerated \"after.rs\" (generator \"g\") (command \"c\"))\n"
            )
        };
        let harness = |name: &str, pair: &str, test: &str| {
            format!("(defharness {name} (pair {pair}) (package \"crates/r\") (test {test}))\n")
        };
        for (forms, why) in [
            (
                harness("h", "missing imp", "t"),
                "`h`'s pair names `missing`, which names no root",
            ),
            (harness("h", "refm refm", "t"), "`h`'s pair names `refm` twice"),
            (
                harness("h", "gen chk", "t"),
                "`h`'s pair has roles `generator` and `scheduling-checker`",
            ),
            (
                "(defderivation d (pair refm imp) (package \"crates/r\") (test t) (reason \"r\"))\n".to_owned(),
                "`d`'s pair has roles `implementation` and `reference-model`",
            ),
            (
                format!("{}{}", harness("h", "refm imp", "t"), harness("k", "refm imp", "t")),
                "`h` and `k` name one test target, `t` of `crates/r`",
            ),
            (
                format!(
                    "{}(defprogram p (package \"crates/r\") (target test t) (reason \"r\"))\n",
                    harness("h", "refm imp", "t")
                ),
                "`p` classifies the test `t` of `crates/r`, which `h` names as a comparison",
            ),
        ] {
            let e = read_roots(&around(&forms)).expect_err(why);
            assert!(
                e.contains(why) && e.contains("before.rs") && e.contains("after.rs"),
                "{why}: {e}"
            );
        }
    }

    /// A front, `gen` as an executable, a configuration checker `cfg` as an executable, and the clauses on `on`'s form.
    fn front(on: &str, clauses: &str, gen_target: &str) -> String {
        let clauses_on = |name: &str| if name == on { clauses } else { "" };
        format!(
            "(defroot gen (role generator) (package \"crates/a\") (target {gen_target}) {})\n\
             (defroot cfg (role configuration-checker) (package \"crates/c\") (target bin cfg) {})\n\
             (defroot chk (role scheduling-checker) (package \"crates/b\") (target lib) {})\n\
             {PAIRED}(defharness h (pair refm imp) (package \"crates/i\") (test h))\n",
            clauses_on("gen"),
            clauses_on("cfg"),
            clauses_on("chk")
        )
    }

    #[test]
    fn the_front_alone_invokes_and_reads_its_install_manifest() {
        crate::trust_test_process::bounded!(the_front_alone_invokes_and_reads_its_install_manifest);
        // `decision_trust-role-programs.md` §4: both clauses, together, on the one root of role `generator` whose
        // target is an executable; every departure a read failure, decided from the text alone.
        let both = "(invokes cfg) (installed \"archogen-install.json\")";
        let roots =
            read_roots(&front("gen", both, "bin a")).expect("the front invokes the checker");
        assert_eq!(
            roots.front_clauses,
            [FrontClauses {
                root: "gen".to_owned(),
                invokes: Some(vec!["cfg".to_owned()]),
                installed: Some(vec!["archogen-install.json".to_owned()]),
            }]
        );
        let installed = "(installed \"archogen-install.json\")";
        for (on, clauses, gen_target, why) in [
            (
                "cfg",
                both,
                "bin a",
                "`cfg` holds `invokes` or `installed`, which only the front's form takes: the root of role `generator`",
            ),
            (
                "gen",
                both,
                "lib",
                "the front `gen` invokes nothing: its target is no executable",
            ),
            (
                "gen",
                "(invokes cfg)",
                "bin a",
                "`gen` holds one of `invokes` and `installed` without the other",
            ),
            (
                "gen",
                installed,
                "bin a",
                "`gen` holds one of `invokes` and `installed` without the other",
            ),
            (
                "gen",
                &format!("(invokes) {installed}"),
                "bin a",
                "`gen`'s `invokes` names no root",
            ),
            (
                "gen",
                &format!("(invokes cfg cfg) {installed}"),
                "bin a",
                "`gen` invokes `cfg` twice",
            ),
            (
                "gen",
                &format!("(invokes nobody) {installed}"),
                "bin a",
                "`gen` invokes `nobody`, which names no root",
            ),
            (
                "gen",
                &format!("(invokes h) {installed}"),
                "bin a",
                "`gen` invokes `h`, which names no root",
            ),
            (
                "gen",
                &format!("(invokes gen) {installed}"),
                "bin a",
                "`gen` invokes itself",
            ),
            (
                "gen",
                &format!("(invokes chk) {installed}"),
                "bin a",
                "`gen` invokes `chk`, whose target is no executable",
            ),
            (
                "gen",
                "(invokes cfg) (installed \"manifest.json\")",
                "bin a",
                "`gen`'s `installed` holds [\"manifest.json\"]",
            ),
            (
                "gen",
                "(invokes cfg) (installed \"archogen-install.json\" \"x\")",
                "bin a",
                "not the one name `archogen-install.json`",
            ),
            (
                "gen",
                "(invokes cfg) (installed)",
                "bin a",
                "`gen`'s `installed` holds [], not the one name `archogen-install.json`",
            ),
        ] {
            let e = read_roots(&front(on, clauses, gen_target)).expect_err(why);
            assert!(e.contains(why), "{on} {clauses}: {e}");
        }
        let two = format!(
            "{}(defroot gen2 (role generator) (package \"crates/z\") (target bin z))\n",
            front("gen", both, "bin a")
        );
        let e = read_roots(&two).expect_err("two generators");
        assert!(
            e.contains("2 roots have role `generator`, so none is the front"),
            "{e}"
        );
        // A derivation is no root, so no program the front runs (§4).
        let derivation = format!(
            "{}(defderivation d (pair gen cfg) (package \"crates/r\") (test t) (reason \"r\"))\n",
            front("gen", &format!("(invokes d) {installed}"), "bin a")
        );
        let e = read_roots(&derivation).expect_err("a derivation invoked");
        assert!(e.contains("`gen` invokes `d`, which names no root"), "{e}");
    }

    #[test]
    fn every_clause_holds_names_alone() {
        crate::trust_test_process::bounded!(every_clause_holds_names_alone);
        // `decision_trust-role-programs.md` §4, round 5's D3: an integer or a list in any clause of any form the
        // instrument reads is a read failure, never dropped; the same form with names alone passes that check.
        let mut dropped = Vec::new();
        for (head, clauses) in FORMS {
            let name = if *head == "defadmit" { "" } else { " n" };
            let who = if *head == "defadmit" {
                "an admission"
            } else {
                "n"
            };
            let text = |bad_in: &str, bad: &str| {
                let body: String = clauses
                    .iter()
                    .map(|c| {
                        let extra = if *c == bad_in { bad } else { "" };
                        format!(" ({c} \"v\"{extra})")
                    })
                    .collect();
                format!("({head}{name}{body})\n")
            };
            if let Err(e) = read_roots(&text("", "")) {
                assert!(!e.contains("is not a name"), "{head} with names alone: {e}");
            }
            for c in *clauses {
                for bad in [" 7", " (x)"] {
                    match read_roots(&text(c, bad)) {
                        Err(e) => assert!(
                            e.contains(&format!(
                                "`{who}`'s `({c} …)` holds something that is not a name"
                            )),
                            "{head} ({c}{bad}): {e}"
                        ),
                        Ok(_) => dropped.push(format!("{head} ({c}{bad})")),
                    }
                }
            }
        }
        assert!(dropped.is_empty(), "read with an item dropped: {dropped:?}");
    }

    #[test]
    fn a_fault_in_the_front_s_own_form_stops_the_reader_there() {
        crate::trust_test_process::bounded!(a_fault_in_the_front_s_own_form_stops_the_reader_there);
        // Round 5's D4: the generated-sources record's clarification — a read failure within a form stops the reader at
        // that form, so a `defgenerated` form after it is not read; one spanning forms is found once the file is read,
        // with every refusal beside it.
        let around = |on: &str, clauses: &str, gen_target: &str| {
            format!(
                "(defgenerated \"before.rs\" (generator \"g\") (command \"c\"))\n{}\
                 (defgenerated \"after.rs\" (generator \"g\") (command \"c\"))\n",
                front(on, clauses, gen_target)
            )
        };
        let installed = "(installed \"archogen-install.json\")";
        let both = format!("(invokes cfg) {installed}");
        for (on, clauses, gen_target) in [
            ("cfg", both.as_str(), "bin a"),
            ("gen", both.as_str(), "lib"),
            ("gen", "(invokes cfg)", "bin a"),
            ("gen", installed, "bin a"),
            ("gen", &format!("(invokes) {installed}"), "bin a"),
            ("gen", &format!("(invokes cfg cfg) {installed}"), "bin a"),
            ("gen", &format!("(invokes gen) {installed}"), "bin a"),
            (
                "gen",
                "(invokes cfg) (installed \"manifest.json\")",
                "bin a",
            ),
        ] {
            let e = read_roots(&around(on, clauses, gen_target)).expect_err(clauses);
            assert!(
                e.contains("before.rs") && !e.contains("after.rs"),
                "{on} {clauses}: within its form, the reader stops there: {e}"
            );
        }
        for clauses in [
            format!("(invokes nobody) {installed}"),
            format!("(invokes chk) {installed}"),
        ] {
            let e = read_roots(&around("gen", &clauses, "bin a")).expect_err(&clauses);
            assert!(
                e.contains("before.rs") && e.contains("after.rs"),
                "{clauses}: an invoked name is known once the file is read: {e}"
            );
        }
        let two = format!(
            "{}(defroot gen2 (role generator) (package \"crates/z\") (target bin z))\n",
            around("gen", &both, "bin a")
        );
        let e = read_roots(&two).expect_err("two generators");
        assert!(
            e.contains("2 roots have role `generator`")
                && e.contains("before.rs")
                && e.contains("after.rs"),
            "{e}"
        );
    }

    #[test]
    fn an_unrooted_role_names_exactly_one_future_leaf() {
        crate::trust_test_process::bounded!(an_unrooted_role_names_exactly_one_future_leaf);
        let placeholder = |leaf: &str| format!("(defrole configuration-checker {leaf})");
        let valid = read_roots(&placeholder("(leaf \"M3.5\")"))
            .expect("the unrooted role names its future leaf");
        assert!(valid.programs.is_empty(), "a placeholder is not a program");

        let mut accepted = Vec::new();
        for leaf in ["", "(leaf)", "(leaf \"M3.5\" \"M3.4\")"] {
            match read_roots(&placeholder(leaf)) {
                Ok(_) => accepted.push(leaf),
                Err(error) => assert!(
                    error.contains("`configuration-checker` needs exactly one `(leaf …)`"),
                    "{leaf:?}: {error}"
                ),
            }
        }
        assert!(
            accepted.is_empty(),
            "accepted without one future leaf: {accepted:?}"
        );
    }

    #[test]
    fn named_trust_forms_require_a_textual_name_before_their_clauses() {
        crate::trust_test_process::bounded!(
            named_trust_forms_require_a_textual_name_before_their_clauses
        );
        let forms = [
            (
                "defroot",
                "extra",
                "(role implementation) (package \"crates/extra\") (target lib)",
            ),
            (
                "defharness",
                "comparison",
                "(pair refm imp) (package \"crates/tests\") (test compare)",
            ),
            (
                "defprogram",
                "tool",
                "(package \"crates/tools\") (target bin tool) (reason \"a tool\")",
            ),
            ("defrole", "configuration-checker", "(leaf \"M3.5\")"),
        ];
        let mut accepted = Vec::new();
        for (head, name, clauses) in forms {
            for valid_name in [name.to_owned(), format!("\"{name}\"")] {
                read_roots(&format!("{ROOTS}{PAIRED}({head} {valid_name} {clauses})"))
                    .expect("a symbol or string name is valid");
            }
            for invalid_name in ["", "7", "(not-a-name)"] {
                let text = format!("{ROOTS}{PAIRED}({head} {invalid_name} {clauses})");
                match read_roots(&text) {
                    Ok(roots) => accepted.push(format!(
                        "{head} {invalid_name:?}: program names {:?}; classification names {:?}",
                        roots.programs.iter().map(|p| &p.name).collect::<Vec<_>>(),
                        roots.classified.iter().map(|p| &p.name).collect::<Vec<_>>()
                    )),
                    Err(error) => assert!(
                        error
                            .contains(&format!("`{head}` needs a textual name before its clauses")),
                        "{head} {invalid_name:?}: {error}"
                    ),
                }
            }
        }
        let unnamed = read_roots(&admit("crates/a/src/lib.rs", "include_str", "the template"))
            .expect("an admission deliberately has no name");
        assert_eq!(unnamed.admissions.len(), 1);
        assert!(
            accepted.is_empty(),
            "accepted without a textual name: {accepted:#?}"
        );
    }

    #[test]
    fn an_admission_requires_exactly_one_textual_reason_value() {
        crate::trust_test_process::bounded!(an_admission_requires_exactly_one_textual_reason_value);
        let hash = Digest::of(b"pub const T: &str = include_str!(\"table.txt\");").hex();
        let admission = |reason: &str| {
            format!(
                "(defadmit (file \"crates/a/src/lib.rs\") (rule \"include_str\") (sha256 \"{hash}\") {reason})"
            )
        };
        let valid = read_roots(&admission("(reason \"the reviewed template\")"))
            .expect("one textual reason is valid");
        assert_eq!(valid.admissions.len(), 1);
        assert_eq!(valid.admissions.values().copied().collect::<Vec<_>>(), [1]);

        for reason in ["", "(reason)", "(reason \"first\" \"second\")"] {
            let error = read_roots(&admission(reason))
                .expect_err("an admission needs exactly one textual reason value");
            assert!(
                error.contains("`an admission` needs exactly one `(reason …)`"),
                "{reason:?}: {error}"
            );
        }
    }

    #[test]
    fn a_root_retains_all_declared_role_packages_and_its_own_package() {
        crate::trust_test_process::bounded!(
            a_root_retains_all_declared_role_packages_and_its_own_package
        );
        let root = |packages: &str| {
            read_roots(&format!(
                "(defroot gen (role generator) (package \"crates/root\") (target bin gen) {packages})"
            ))
            .expect("valid root")
        };
        let implicit = root("");
        assert_eq!(implicit.programs.len(), 1);
        assert_eq!(
            implicit.programs[0]
                .role_packages
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["crates/root"]
        );

        let declared = root("(role-packages \"crates/first\" \"crates/middle\" \"crates/last\")");
        assert_eq!(declared.programs.len(), 1);
        assert_eq!(
            declared.programs[0]
                .role_packages
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            [
                "crates/first",
                "crates/last",
                "crates/middle",
                "crates/root"
            ]
        );
    }

    #[test]
    fn a_classification_retains_all_declared_role_packages() {
        crate::trust_test_process::bounded!(a_classification_retains_all_declared_role_packages);
        let classification = |packages: &str| {
            read_roots(&format!(
                "(defprogram tool (package \"crates/tool\") (target bin tool) (reason \"a tool\") {packages})"
            ))
            .expect("valid classification")
        };
        let empty = classification("");
        assert_eq!(empty.classified.len(), 1);
        assert!(empty.classified[0].role_packages.is_empty());

        let declared =
            classification("(role-packages \"crates/first\" \"crates/middle\" \"crates/last\")");
        assert_eq!(declared.classified.len(), 1);
        assert_eq!(
            declared.classified[0]
                .role_packages
                .iter()
                .map(String::as_str)
                .collect::<Vec<_>>(),
            ["crates/first", "crates/last", "crates/middle"]
        );
    }

    #[test]
    fn a_classification_is_read_strictly() {
        crate::trust_test_process::bounded!(a_classification_is_read_strictly);
        for (text, why) in [
            (
                "(defprogram x (package \"p\") (target lib t) (reason \"r\"))",
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
        // A test-like unit joining two roles is classified as a program target is (`decision_trust-role-programs.md` §3).
        for kind in TEST_KINDS {
            let roots = read_roots(&format!(
                "(defprogram x (package \"p\") (target {kind} t) (reason \"r\"))"
            ))
            .expect(kind);
            assert_eq!(roots.classified[0].kind, *kind);
        }
    }

    #[test]
    fn a_file_a_root_compiles_whose_header_marks_it_generated_is_refused_until_declared() {
        crate::trust_test_process::bounded!(
            a_file_a_root_compiles_whose_header_marks_it_generated_is_refused_until_declared
        );
        // The generated-sources record §3: a marked file a program reads, no form declaring it, is refused (`GS-H2`,
        // `M3.6.6.2.4`); declared, it passes and is listed, so the record's census is the instrument's own output.
        let f = two_roots(
            "generated-marked",
            "mod table;\nfn main() { let _ = table::T; }\n",
            "pub fn f() {}\n",
            &[(
                "crates/a/src/table.rs",
                "// @generated by scripts/gen_table.sh — do not edit\npub const T: u32 = 1;\n"
                    .to_owned(),
            )],
        );
        says(
            &refused(f.run()),
            "trust-undeclared-input: `gen` reads `crates/a/src/table.rs`, whose header marks it generated and which no \
             `defgenerated` form declares — a generated source with no declaration",
        );
        f.commit(&[
            (
                "trust/roots.eadl",
                format!(
                    "{ROOTS}(defgenerated \"crates/a/src/table.rs\" (generator \"scripts/gen_table.sh\") (command \"bash \
                     scripts/gen_table.sh\") (reason \"the table\"))\n"
                ),
            ),
            ("scripts/gen_table.sh", "echo table\n".to_owned()),
        ]);
        let inv = written(f.run());
        let marked: Vec<&str> = inv
            .get("generated-marked")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .filter_map(Json::as_str)
            .collect();
        assert_eq!(marked, ["crates/a/src/table.rs"]);
    }

    #[test]
    fn the_roles_the_roots_file_names_are_the_evidence_vocabulary_s_roots() {
        crate::trust_test_process::bounded!(
            the_roles_the_roots_file_names_are_the_evidence_vocabulary_s_roots
        );
        // `M3.6.3.6`: the vocabulary brought to the record's roles, and held there (§2).
        let slugs: Vec<&str> = archogen_evidence::TrustRoot::ALL
            .iter()
            .map(|r| r.slug())
            .collect();
        assert_eq!(slugs, super::ROLES);
    }

    #[test]
    fn a_path_dependency_outside_the_written_tree_is_refused_before_any_compilation() {
        crate::trust_test_process::bounded!(
            a_path_dependency_outside_the_written_tree_is_refused_before_any_compilation
        );
        // The commit's own manifest reaches a package it does not hold — by an absolute path, by one climbing out of the
        // tree, two at once, or under a directory whose name repeats the tree's own path — and each is refused, naming
        // it, before anything is compiled: never read as the root's, never a run that could not judge
        // (`M3.6.9.6.3.2.2.15.2.1.7.2`). Cargo lists the members first, so two outside packages are never both first
        // or both last.
        let external = real_root().join("target/trust-tests/outside-dependency-external");
        let _ = std::fs::remove_dir_all(&external);
        let crate_named = |dir: &str, name: &str| {
            vec![
                (format!("{dir}/Cargo.toml"), manifest(name, "")),
                (format!("{dir}/src/lib.rs"), "pub fn x() {}\n".to_owned()),
            ]
        };
        let put = |base: &Path, files: Vec<(String, String)>| {
            let files: Vec<(&str, String)> =
                files.iter().map(|(p, t)| (p.as_str(), t.clone())).collect();
            write(base, &files);
        };
        put(&external, crate_named("x", "x"));
        put(&external, crate_named("p", "p"));
        put(&external, crate_named("q", "q"));
        let tree_of = |name: &str| {
            std::fs::canonicalize(real_root())
                .unwrap()
                .join("target/trust-tests")
                .join(name)
                .join("out/tree")
        };
        let nested = external
            .join("nested")
            .join(
                tree_of("outside-dependency-substring")
                    .strip_prefix("/")
                    .unwrap(),
            )
            .join("s");
        put(&nested, crate_named(".", "s"));
        let path = |p: &Path| format!("{{ path = \"{}\" }}", p.display());
        let cases = [
            (
                "outside-dependency-absolute",
                format!("[dependencies]\nx = {}\n", path(&external.join("x"))),
                vec![external.join("x")],
            ),
            (
                "outside-dependency-climbing",
                "[dependencies]\nx = { path = \"../../../../outside-dependency-external/x\" }\n"
                    .to_owned(),
                vec![],
            ),
            (
                "outside-dependency-two",
                format!(
                    "[dependencies]\np = {}\nq = {}\n",
                    path(&external.join("p")),
                    path(&external.join("q"))
                ),
                vec![external.join("p"), external.join("q")],
            ),
            (
                "outside-dependency-substring",
                format!("[dependencies]\ns = {}\n", path(&nested)),
                vec![nested.clone()],
            ),
        ];
        for (name, dependencies, reached) in cases {
            let f = two_roots(
                name,
                "fn main() {}\n",
                "pub fn f() {}\n",
                &[("crates/a/Cargo.toml", manifest("a", &dependencies))],
            );
            // The tree is written one level deeper than the repository, so the climbing path resolves to the base from
            // it: a copy there is what the build would reach.
            let reached = if reached.is_empty() {
                put(
                    &f.base.join("outside-dependency-external"),
                    crate_named("x", "x"),
                );
                vec![f.base.join("outside-dependency-external/x")]
            } else {
                reached
            };
            let mut expected: Vec<String> = reached
                .iter()
                .map(|dir| {
                    let manifest = std::fs::canonicalize(dir.join("Cargo.toml")).unwrap();
                    let crate_name = dir.file_name().unwrap().to_string_lossy().into_owned();
                    format!(
                        "trust-undeclared-input: the commit reaches the package `{crate_name}` at `{}`, outside the \
                         written tree",
                        manifest.display()
                    )
                })
                .collect();
            expected.sort();
            assert_eq!(refused(f.run()), expected, "{name}");
            assert_eq!(
                tree_of(name),
                std::fs::canonicalize(f.base.join("out/tree")).unwrap(),
                "{name}"
            );
            assert!(
                !f.base.join("out/target/gen").exists(),
                "{name}: nothing compiled"
            );
            assert!(
                !f.base.join("out/target/chk").exists(),
                "{name}: nothing compiled"
            );
        }
        let _ = std::fs::remove_dir_all(external);
    }

    #[test]
    fn a_cargo_configuration_on_the_build_s_path_is_a_refusal_and_not_an_error() {
        crate::trust_test_process::bounded!(
            a_cargo_configuration_on_the_build_s_path_is_a_refusal_and_not_an_error
        );
        // The catalog's §3, adopted (R8 remark 16): the commit is refused, so the gate reports it, exit 1, rather than
        // saying it could not judge (`M3.6.3.4`).
        let f = two_roots("config-on-path", "fn main() {}\n", "pub fn f() {}\n", &[]);
        write(
            &f.base,
            &[(
                ".cargo/config.toml",
                "[build]\nrustflags = [\"--cfg\", \"x\"]\n".to_owned(),
            )],
        );
        let r = refused(f.run());
        says(&r, "trust-undeclared-input: ");
        says(&r, "a cargo configuration under `target/`");
    }

    #[test]
    fn a_form_naming_a_package_or_a_target_the_commit_no_longer_has_is_stale() {
        crate::trust_test_process::bounded!(
            a_form_naming_a_package_or_a_target_the_commit_no_longer_has_is_stale
        );
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
        crate::trust_test_process::bounded!(
            two_roots_sharing_nothing_share_only_the_build_configuration
        );
        let f = two_roots("clean", "fn main() {}\n", "pub fn f() -> u32 { 1 }\n", &[]);
        let inv = written(f.run());
        let shared = items(&inv);
        assert_eq!(shared.len(), 1, "{shared:?}");
        assert_eq!(kind(&shared[0].1), "build-configuration");
    }

    #[test]
    fn a_package_both_roots_compile_is_shared_case_1() {
        crate::trust_test_process::bounded!(a_package_both_roots_compile_is_shared_case_1);
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
        crate::trust_test_process::bounded!(an_incbin_under_each_assembler_macro_is_refused);
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
        crate::trust_test_process::bounded!(an_include_and_a_path_module_are_refused);
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
        crate::trust_test_process::bounded!(a_macro_assembling_a_path_attribute_is_refused);
        let f = two_roots("macro-path", "fn main() {}\n", "macro_rules! m { ($b:tt) => { # $b mod evil; } }\nm!([path = \"evil.txt\"]);\npub use evil::*;\n", &[("crates/b/src/evil.txt", "pub fn e() {}\n".to_owned())]);
        says(&refused(f.run()), "`macro_rules`, refused");
    }

    #[test]
    fn an_extern_block_calling_another_package_s_symbol_is_refused() {
        crate::trust_test_process::bounded!(
            an_extern_block_calling_another_package_s_symbol_is_refused
        );
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

    pub(crate) fn harness(
        name: &str,
        dev_dep: &str,
        dev_src: &str,
        dev_manifest_extra: &str,
    ) -> Fixture {
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
        crate::trust_test_process::bounded!(
            the_harness_is_inventoried_and_its_development_edges_followed
        );
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

    /// The generator `gen`, the configuration checker `cfg`, a scheduling checker `chk`, and `crates/r`, whose tests
    /// compare the generator's reading with the checker's, the derivation forms `forms` naming them
    /// (`decision_trust-role-programs.md` §3).
    fn derivations(
        name: &str,
        forms: &str,
        dev_dep: &str,
        dev_src: &str,
        dev_manifest_extra: &str,
    ) -> Fixture {
        let roots = format!(
            "{ROOTS}(defroot cfg (role configuration-checker) (package \"crates/c\") (target lib) (role-packages \"crates/c\"))\n{forms}"
        );
        let compare = |what: &str| {
            format!("#[test]\nfn {what}() {{ assert_eq!(r::reading(3), c::reading(3)); }}\n")
        };
        fixture(
            name,
            "1.95.0",
            &[
                ("Cargo.toml", WS.to_owned()),
                ("trust/roots.eadl", roots),
                ("crates/a/Cargo.toml", manifest("a", "")),
                ("crates/a/src/main.rs", "fn main() {}\n".to_owned()),
                ("crates/b/Cargo.toml", manifest("b", "")),
                ("crates/b/src/lib.rs", "pub fn analysis() {}\n".to_owned()),
                ("crates/c/Cargo.toml", manifest("c", "")),
                (
                    "crates/c/src/lib.rs",
                    "pub fn reading(x: u32) -> u32 { x }\n".to_owned(),
                ),
                (
                    "crates/r/Cargo.toml",
                    manifest(
                        "r",
                        &format!("[dev-dependencies]\nc = {{ path = \"../c\" }}\n{dev_dep}"),
                    ),
                ),
                (
                    "crates/r/src/lib.rs",
                    "pub fn reading(x: u32) -> u32 { x }\n".to_owned(),
                ),
                ("crates/r/tests/production.rs", compare("production")),
                ("crates/r/tests/vocabulary.rs", compare("vocabulary")),
                ("crates/d/Cargo.toml", manifest("d", dev_manifest_extra)),
                ("crates/d/src/lib.rs", dev_src.to_owned()),
            ],
        )
    }

    const PRODUCTION: &str =
        "(defderivation prod (pair gen cfg) (package \"crates/r\") (test production) (reason \"the relation\"))\n";

    #[test]
    fn a_derivation_is_its_pair_s_item_and_no_other_root_s() {
        crate::trust_test_process::bounded!(a_derivation_is_its_pair_s_item_and_no_other_root_s);
        // `decision_trust-role-programs.md` §3: built and recorded as the harness, its development edges followed, but
        // paired with no root, its pair taking its item, one item a form.
        let vocabulary =
            "(defderivation voc (pair cfg gen) (package \"crates/r\") (test vocabulary) (reason \"the table\"))\n";
        let f = derivations(
            "derivation",
            &format!("{PRODUCTION}{vocabulary}"),
            "",
            "pub fn d() {}\n",
            "",
        );
        let inv = written(f.run());
        let derived: Vec<(String, String)> = items(&inv)
            .iter()
            .filter(|(_, i)| kind(i) == "shared-derivation")
            .map(|(pair, i)| {
                (
                    pair.clone(),
                    i.get("derivation")
                        .and_then(Json::as_str)
                        .unwrap_or_default()
                        .to_owned(),
                )
            })
            .collect();
        assert_eq!(
            derived,
            [
                ("gen+cfg".to_owned(), "prod".to_owned()),
                ("gen+cfg".to_owned(), "voc".to_owned())
            ],
            "each derivation its pair's item, one a form"
        );
        for (pair, _) in items(&inv) {
            assert!(
                !pair.split('+').any(|n| n == "prod" || n == "voc"),
                "a derivation is paired with no root: {pair}"
            );
        }
        let recorded: Vec<&str> = inv
            .get("programs")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .filter(|p| p.get("role").and_then(Json::as_str) == Some("derivation"))
            .filter_map(|p| p.get("name").and_then(Json::as_str))
            .collect();
        assert_eq!(
            recorded,
            ["prod", "voc"],
            "recorded as the harness is, its role `derivation`"
        );
    }

    #[test]
    fn a_derivation_naming_a_test_target_its_package_lacks_is_stale() {
        crate::trust_test_process::bounded!(
            a_derivation_naming_a_test_target_its_package_lacks_is_stale
        );
        // `decision_trust-role-programs.md` §3: from metadata, wherever the gate runs, as any form's target.
        let gone = "(defderivation prod (pair gen cfg) (package \"crates/r\") (test gone) (reason \"the relation\"))\n";
        let f = derivations("derivation-gone", gone, "", "pub fn d() {}\n", "");
        says(
            &refused(f.run()),
            "trust-baseline-stale: trust/roots.eadl: `prod` names the test target `gone` of `crates/r`, which the commit no \
             longer has",
        );
        // Its package gone, as any form's (round 5's R8).
        let moved = "(defderivation prod (pair gen cfg) (package \"crates/gone\") (test production) (reason \"r\"))\n";
        let f = derivations("derivation-package-gone", moved, "", "pub fn d() {}\n", "");
        says(
            &refused(f.run()),
            "trust-baseline-stale: trust/roots.eadl: `prod` names `crates/gone`, which is no package of the commit",
        );
    }

    #[test]
    fn a_derivation_is_held_to_its_pair_s_role_packages_and_its_edges_scanned() {
        crate::trust_test_process::bounded!(
            a_derivation_is_held_to_its_pair_s_role_packages_and_its_edges_scanned
        );
        // Its exemption from `trust-shared-program` is its pair's role packages alone; its development edges are read
        // by the catalog's rules as the harness's.
        let f = derivations(
            "derivation-third-role",
            PRODUCTION,
            "b = { path = \"../b\" }\n",
            "pub fn d() {}\n",
            "",
        );
        says(
            &refused(f.run()),
            "trust-shared-program: `prod` compiles `crates/b`, a role package of `chk`",
        );
        let f = derivations(
            "derivation-no-mangle",
            PRODUCTION,
            "d = { path = \"../d\" }\n",
            "#[no_mangle]\npub fn helper() {}\n",
            "",
        );
        says(&refused(f.run()), "`no_mangle`, refused");
        let f = derivations(
            "derivation-proc-macro",
            PRODUCTION,
            "d = { path = \"../d\" }\n",
            "",
            "[lib]\nproc-macro = true\n",
        );
        says(&refused(f.run()), "reaches `crates/d`");
    }

    /// A front `gen` whose form holds `clauses`, the configuration checker `cfg` as an executable, a scheduling checker
    /// `chk`, and the launcher with `launcher` as its text, or none (`decision_trust-role-programs.md` §4).
    pub(crate) fn invoking(name: &str, clauses: &str, launcher: Option<&str>) -> Fixture {
        let roots = format!(
            "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-packages \"crates/a\") {clauses})\n\
             (defroot chk (role scheduling-checker) (package \"crates/b\") (target lib) (role-packages \"crates/b\"))\n\
             (defroot cfg (role configuration-checker) (package \"crates/c\") (target bin c) (role-packages \"crates/c\"))\n"
        );
        let mut files = vec![
            ("Cargo.toml", WS.to_owned()),
            ("trust/roots.eadl", roots),
            ("crates/a/Cargo.toml", manifest("a", "")),
            ("crates/a/src/main.rs", "fn main() {}\n".to_owned()),
            ("crates/b/Cargo.toml", manifest("b", "")),
            ("crates/b/src/lib.rs", "pub fn analysis() {}\n".to_owned()),
            ("crates/c/Cargo.toml", manifest("c", "")),
            ("crates/c/src/main.rs", "fn main() {}\n".to_owned()),
            (
                "crates/archogen-cli/Cargo.toml",
                manifest("archogen-cli", ""),
            ),
            (
                "crates/archogen-cli/src/lib.rs",
                "pub fn front() {}\n".to_owned(),
            ),
        ];
        if let Some(text) = launcher {
            files.push(("crates/archogen-cli/src/sibling.rs", text.to_owned()));
        }
        fixture(name, "1.95.0", &files)
    }

    pub(crate) const INVOKES: &str = "(invokes cfg) (installed \"archogen-install.json\")";

    /// The invocation items of an inventory, by pair, with their content.
    fn invocations(inv: &Json) -> Vec<(String, Vec<String>)> {
        items(inv)
            .into_iter()
            .filter(|(_, i)| kind(i) == "invocation")
            .map(|(pair, i)| {
                let content = i.get("content").map(Json::elements).unwrap_or_default();
                (
                    pair,
                    content
                        .iter()
                        .filter_map(Json::as_str)
                        .map(str::to_owned)
                        .collect(),
                )
            })
            .collect()
    }

    #[test]
    fn the_front_and_each_root_it_invokes_share_an_invocation() {
        crate::trust_test_process::bounded!(the_front_and_each_root_it_invokes_share_an_invocation);
        // `decision_trust-role-programs.md` §4: the invoked root's name, role and target and the launcher's sha256, so a
        // launcher edit moves the item; no clause, no item.
        let first = written(invoking("invocation", INVOKES, Some("pub fn run() {}\n")).run());
        let found = invocations(&first);
        let launcher = Digest::of(b"pub fn run() {}\n").hex();
        assert_eq!(
            found,
            [(
                "gen+cfg".to_owned(),
                vec![
                    "invoked=cfg configuration-checker bin c".to_owned(),
                    format!("launcher=crates/archogen-cli/src/sibling.rs={launcher}"),
                ]
            )],
            "one invocation, the front's and the invoked root's, none with the scheduling checker"
        );
        let edited = written(
            invoking(
                "invocation-edited",
                INVOKES,
                Some("pub fn run() { /* another contract */ }\n"),
            )
            .run(),
        );
        assert_ne!(
            invocations(&edited),
            found,
            "a launcher edit moves the invocation"
        );
        let none = written(invoking("invocation-none", "", Some("pub fn run() {}\n")).run());
        assert!(invocations(&none).is_empty(), "no clause, no invocation");
    }

    #[test]
    fn a_front_that_invokes_without_its_launcher_is_refused() {
        crate::trust_test_process::bounded!(a_front_that_invokes_without_its_launcher_is_refused);
        // `decision_trust-role-programs.md` §4: read with the tree, in step 2, as a root's `(data …)` is.
        let f = invoking("invocation-no-launcher", INVOKES, None);
        says(
            &refused(f.run()),
            "trust-undeclared-input: `gen` invokes programs, and its launcher `crates/archogen-cli/src/sibling.rs` is not a \
             blob of the commit",
        );
    }

    /// [`invoking`] with the front's launcher missing and its form handing it `missing.json`, which the commit lacks.
    fn step_two(name: &str) -> Fixture {
        invoking(name, &format!("{INVOKES} (data \"missing.json\")"), None)
    }

    /// Commit a symbolic link `link` to `to` in `f`'s repository.
    fn link(f: &Fixture, link: &str, to: &str) {
        let at = f.repo.join(link);
        std::fs::create_dir_all(at.parent().unwrap()).unwrap();
        std::os::unix::fs::symlink(to, &at).unwrap();
        git(&f.repo, &["add", "-A"]);
        git(&f.repo, &["commit", "-q", "-m", "link"]);
    }

    #[test]
    fn every_refusal_of_step_two_is_reported_together() {
        crate::trust_test_process::bounded!(every_refusal_of_step_two_is_reported_together);
        // Round 8's D3; the generated-sources record §2, clarified by `M3.5.2.5.2`: a root's data and the launcher are
        // judged first, from the commit's entries, so a refusal ending the step early has them beside it.
        let data = "trust-undeclared-input: `gen` declares `missing.json`, which is not a blob of the commit";
        let launcher = "trust-undeclared-input: `gen` invokes programs, and its launcher \
                        `crates/archogen-cli/src/sibling.rs` is not a blob of the commit";
        // A link under `catalog/`: no tree is written.
        let f = step_two("step-two-catalog-link");
        link(&f, "catalog/link", "../Cargo.toml");
        let r = refused(f.run());
        for want in [data, launcher, "under `catalog/`"] {
            says(&r, want);
        }
        // A cargo configuration §3 refuses: cargo is never run.
        let f = step_two("step-two-configuration");
        write(
            &f.base,
            &[(
                ".cargo/config.toml",
                "[build]\nrustflags = [\"--cfg\", \"x\"]\n".to_owned(),
            )],
        );
        let r = refused(f.run());
        for want in [data, launcher, "a cargo configuration under `target/`"] {
            says(&r, want);
        }
        // A package outside the written tree, a stale form and a build script reached: every judgment before a build
        // is made, and none is built.
        let external = real_root().join("target/trust-tests/step-two-external");
        let _ = std::fs::remove_dir_all(&external);
        write(
            &external,
            &[
                ("x/Cargo.toml", manifest("x", "")),
                ("x/src/lib.rs", "pub fn x() {}\n".to_owned()),
            ],
        );
        let f = step_two("step-two-judged-on");
        let roots = std::fs::read_to_string(f.repo.join("trust/roots.eadl"))
            .unwrap()
            .replace(
                "(package \"crates/b\") (target lib) (role-packages \"crates/b\")",
                "(package \"crates/gone\") (target lib) (role-packages \"crates/gone\")",
            );
        f.commit(&[
            ("trust/roots.eadl", roots),
            (
                "crates/a/Cargo.toml",
                manifest(
                    "a",
                    &format!(
                        "[dependencies]\nx = {{ path = \"{}\" }}\n",
                        external.join("x").display()
                    ),
                ),
            ),
            ("crates/c/build.rs", "fn main() {}\n".to_owned()),
            // A package at the workspace's root, with a build script no program reaches: an outside package read as it,
            // or an edge to one kept, would reach it.
            ("Cargo.toml", format!("{WS}{}", manifest("root", ""))),
            ("src/lib.rs", "pub fn root() {}\n".to_owned()),
            ("build.rs", "fn main() {}\n".to_owned()),
        ]);
        let r = refused(f.run());
        let manifest_at = std::fs::canonicalize(external.join("x/Cargo.toml")).unwrap();
        for want in [
            data.to_owned(),
            launcher.to_owned(),
            format!(
                "trust-undeclared-input: the commit reaches the package `x` at `{}`, outside the written tree",
                manifest_at.display()
            ),
            "trust-baseline-stale: trust/roots.eadl: `chk` names `crates/gone`, which is no package of the commit"
                .to_owned(),
            "trust-undeclared-input: `cfg` reaches `crates/c` target `build-script-build` of kind `custom-build`"
                .to_owned(),
            "trust-undeclared-input: `cfg` reaches `crates/c`: `crates/c/build.rs`: a package has no build script"
                .to_owned(),
        ] {
            says(&r, &want);
        }
        // Those six and no other: the stale form's program and the outside package are judged nowhere else, and the
        // root's package, which no program reaches, not at all.
        assert_eq!(r.len(), 6, "{r:#?}");
        for program in ["gen", "chk", "cfg"] {
            assert!(
                !f.base.join("out/target").join(program).exists(),
                "{program}: nothing built"
            );
        }
        let _ = std::fs::remove_dir_all(external);
        // A run the gate cannot judge past has the refusals already found beside it: here `cargo metadata`, refusing
        // a lock a manifest committed without it has made stale.
        let f = step_two("step-two-unable");
        write(
            &f.repo,
            &[(
                "crates/b/Cargo.toml",
                manifest("b", "[dependencies]\nc = { path = \"../c\" }\n"),
            )],
        );
        git(
            &f.repo,
            &[
                "commit",
                "-q",
                "-a",
                "-m",
                "a dependency, the lock unchanged",
            ],
        );
        let e = match f.try_run_into("out") {
            Err(e) => e,
            Ok(Outcome::Written(_)) => panic!("written"),
            Ok(Outcome::Refused(r)) => panic!("refused, not unable to judge: {r:#?}"),
        };
        assert!(
            e.starts_with("`cargo metadata` failed")
                && e.contains("and beside it, 2 refusal(s):")
                && e.contains(data)
                && e.contains(launcher),
            "{e}"
        );
    }

    #[test]
    fn a_launcher_that_is_a_symbolic_link_is_refused() {
        crate::trust_test_process::bounded!(a_launcher_that_is_a_symbolic_link_is_refused);
        // Round 8's R12: the launcher is read as a root's data is, a blob of the commit and never a symbolic link
        // (`decision_trust-role-programs.md` §4).
        let f = invoking("invocation-linked-launcher", INVOKES, None);
        link(&f, "crates/archogen-cli/src/sibling.rs", "lib.rs");
        says(
            &refused(f.run()),
            "trust-undeclared-input: `gen` invokes programs, and its launcher `crates/archogen-cli/src/sibling.rs` is not a \
             blob of the commit",
        );
    }

    /// Five roots — the generator `gen` (`crates/a`, its role packages `crates/a` and `crates/g`), the configuration
    /// checker `cfg` (`crates/c`), the scheduling checker `chk` (`crates/s`), the reference model `refm` (`crates/m`) and
    /// the implementation `imp` (`crates/i`) — a helper `crates/h` over `crates/g` and `crates/c` whose own tests are
    /// off, and `crates/r`, whose development dependencies are `dev`: a library with a doc comment, a test `compare`
    /// that names no crate, and a bench `speed`; `forms` appended to the roots.
    fn joining(name: &str, dev: &str, forms: &str) -> Fixture {
        joining_with(name, dev, forms, &[])
    }

    /// [`joining`], `crates/r`'s manifest holding `dev` beside its package table, and `extra` files besides.
    fn joining_with(name: &str, dev: &str, forms: &str, extra: &[(&str, &str)]) -> Fixture {
        let roots = format!(
            "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-packages \"crates/a\" \"crates/g\"))\n\
             (defroot cfg (role configuration-checker) (package \"crates/c\") (target lib))\n\
             (defroot chk (role scheduling-checker) (package \"crates/s\") (target lib))\n\
             (defroot refm (role reference-model) (package \"crates/m\") (target lib))\n\
             (defroot imp (role implementation) (package \"crates/i\") (target lib))\n{forms}"
        );
        let library = |n: &str| {
            (
                format!("crates/{n}/src/lib.rs"),
                "pub fn x() -> u32 { 1 }\n".to_owned(),
            )
        };
        let mut files: Vec<(String, String)> = vec![
            ("Cargo.toml".into(), WS.to_owned()),
            ("trust/roots.eadl".into(), roots),
            (
                "crates/a/Cargo.toml".into(),
                manifest("a", "[dependencies]\ng = { path = \"../g\" }\n"),
            ),
            ("crates/a/src/main.rs".into(), "fn main() { g::x(); }\n".to_owned()),
            (
                "crates/h/Cargo.toml".into(),
                manifest(
                    "h",
                    "[lib]\ntest = false\ndoctest = false\n\n[dependencies]\ng = { path = \"../g\" }\nc = { path = \"../c\" }\n",
                ),
            ),
            (
                "crates/h/src/lib.rs".into(),
                "pub use c::x as checked;\npub use g::x as generated;\n".to_owned(),
            ),
            ("crates/r/Cargo.toml".into(), manifest("r", dev)),
            ("crates/r/src/lib.rs".into(), "//! r\npub fn r() {}\n".to_owned()),
            ("crates/r/tests/compare.rs".into(), "#[test]\nfn t() {}\n".to_owned()),
            ("crates/r/benches/speed.rs".into(), "fn main() {}\n".to_owned()),
        ];
        for n in ["g", "c", "s", "m", "i"] {
            files.push((format!("crates/{n}/Cargo.toml"), manifest(n, "")));
            files.push(library(n));
        }
        files.extend(
            extra
                .iter()
                .map(|(p, t)| ((*p).to_owned(), (*t).to_owned())),
        );
        let files: Vec<(&str, String)> =
            files.iter().map(|(p, t)| (p.as_str(), t.clone())).collect();
        fixture(name, "1.95.0", &files)
    }

    /// Each program target of `crates/r` the inventory reports, by kind and name, with its status.
    fn units_of_r(inv: &Json) -> Vec<(String, String, String)> {
        units_of(inv, "crates/r")
    }

    /// Each program target of `package` the inventory reports, by kind and name, with its status.
    fn units_of(inv: &Json, package: &str) -> Vec<(String, String, String)> {
        let field = |t: &Json, k: &str| {
            t.get(k)
                .and_then(Json::as_str)
                .unwrap_or_default()
                .to_owned()
        };
        let mut out: Vec<_> = inv
            .get("program-targets")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .filter(|t| field(t, "package") == package)
            .map(|t| (field(t, "kind"), field(t, "name"), field(t, "status")))
            .collect();
        out.sort();
        out
    }

    /// The pairs the inventory records for `package`'s unit of `kind`.
    fn joins_of(inv: &Json, package: &str, kind: &str) -> Vec<String> {
        inv.get("program-targets")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .find(|t| {
                t.get("package").and_then(Json::as_str) == Some(package)
                    && t.get("kind").and_then(Json::as_str) == Some(kind)
            })
            .and_then(|t| t.get("joins"))
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .filter_map(Json::as_str)
            .map(str::to_owned)
            .collect()
    }

    /// `crates/r`'s four test-like units, each unclassified, compiling `compiled`.
    fn four_unclassified(compiled: &str) -> Vec<(String, String, String)> {
        unclassified_units(
            "crates/r",
            &[
                ("bench", "speed"),
                ("doctest", "r"),
                ("lib-test", "r"),
                ("test", "compare"),
            ],
            compiled,
        )
    }

    /// `crates/r`'s test `compare` alone, unclassified, compiling `compiled`: the one unit whose build compiled the file.
    fn compare_alone(compiled: &str) -> Vec<(String, String, String)> {
        unclassified_units("crates/r", &[("test", "compare")], compiled)
    }

    /// `units` of `package`, each unclassified, compiling `compiled`.
    fn unclassified_units(
        package: &str,
        units: &[(&str, &str)],
        compiled: &str,
    ) -> Vec<(String, String, String)> {
        units
            .iter()
            .map(|(kind, name)| {
                (
                    (*kind).to_owned(),
                    (*name).to_owned(),
                    format!(
                        "trust-unclassified-program: the {kind} `{name}` of `{package}` compiles {compiled}, role \
                         packages of two roles a property rests on, and no form in trust/roots.eadl names or \
                         classifies it"
                    ),
                )
            })
            .collect()
    }

    #[test]
    fn a_unit_joining_two_roles_is_a_program_target_whatever_it_names() {
        crate::trust_test_process::bounded!(
            a_unit_joining_two_roles_is_a_program_target_whatever_it_names
        );
        // `decision_trust-role-programs.md` §3, rounds 5 and 6: the build decides, not the sources' names — each
        // test-like unit some build of its package makes, whose build compiles role packages of the two roles of a
        // pair a property rests on, is a program target: through a renamed dependency, a helper's, a build-dependency,
        // a feature or another platform's dependency as directly, whatever its flags say; one role's alone, or two
        // roles no property pairs, none.
        let dep = |n: &str| format!("{n} = {{ path = \"../{n}\" }}\n");
        let dev = |deps: &str| format!("[dev-dependencies]\n{deps}");
        let gc = "`crates/c`, `crates/g`";
        let gs = "`crates/g`, `crates/s`";
        for (case, manifest_extra, extra, want) in [
            ("joined", dev(&(dep("g") + &dep("c"))), &[][..], four_unclassified(gc)),
            ("joined-helper", dev(&dep("h")), &[][..], four_unclassified(gc)),
            (
                "joined-renamed",
                dev(&(dep("g") + "chk = { package = \"c\", path = \"../c\" }\n")),
                &[][..],
                four_unclassified(gc),
            ),
            ("joined-scheduling", dev(&(dep("g") + &dep("s"))), &[][..], four_unclassified(gs)),
            (
                "joined-reference",
                dev(&(dep("m") + &dep("i"))),
                &[][..],
                four_unclassified("`crates/i`, `crates/m`"),
            ),
            // A build-dependency's edge counts, a build script being refused wherever the census reaches it.
            (
                "joined-by-a-build-dependency",
                format!("[build-dependencies]\n{}\n{}", dep("s"), dev(&dep("g"))),
                &[][..],
                four_unclassified(gs),
            ),
            (
                "joined-behind-a-feature",
                format!(
                    "[dependencies]\nc = {{ path = \"../c\", optional = true }}\n\n[features]\ncmp = [\"dep:c\"]\n\n{}",
                    dev(&dep("g"))
                ),
                &[][..],
                four_unclassified(gc),
            ),
            (
                "joined-on-another-platform",
                format!("{}\n[target.'cfg(windows)'.dev-dependencies]\n{}", dev(&dep("g")), dep("s")),
                &[][..],
                four_unclassified(gs),
            ),
            (
                "joined-flags-off",
                format!(
                    "[lib]\ntest = false\ndoctest = false\n\n[features]\ncmp = []\n\n[[test]]\nname = \"compare\"\n\
                     test = false\nrequired-features = [\"cmp\"]\n\n[[bench]]\nname = \"speed\"\ntest = false\n\n{}",
                    dev(&(dep("g") + &dep("c")))
                ),
                &[][..],
                four_unclassified(gc),
            ),
            // A file compiled into a unit by any spelling joins that unit, read from what its build compiled (rounds 7
            // and 8): a literal path, an `include!`, a `cfg_attr`'s path, a bracketed `include!`, an escaped literal, a
            // file the catalog's tokenizer refuses, an aliased `include!`, a target whose own path lies outside its
            // package. A path into no role's package, and a file no build compiles, join nothing.
            (
                "joined-by-a-path",
                dev(&dep("g")),
                &[(
                    "crates/r/tests/compare.rs",
                    "#[path = \"../../c/src/lib.rs\"]\nmod checker;\n#[test]\nfn t() { assert_eq!(checker::x(), g::x()); }\n",
                )][..],
                compare_alone(gc),
            ),
            (
                "joined-by-an-include",
                dev(&dep("g")),
                &[(
                    "crates/r/tests/compare.rs",
                    "mod checker { include!(\"../../c/src/lib.rs\"); }\n#[test]\nfn t() { assert_eq!(checker::x(), g::x()); }\n",
                )][..],
                compare_alone(gc),
            ),
            (
                "joined-by-a-cfg-attr-path",
                dev(&dep("g")),
                &[(
                    "crates/r/tests/compare.rs",
                    "#[cfg_attr(all(), path = \"../../c/src/lib.rs\")]\nmod checker;\n#[test]\nfn t() {}\n",
                )][..],
                compare_alone(gc),
            ),
            (
                "joined-by-a-bracketed-include",
                dev(&dep("g")),
                &[(
                    "crates/r/tests/compare.rs",
                    "mod checker { include![\"../../c/src/lib.rs\"]; }\n#[test]\nfn t() { assert_eq!(checker::x(), g::x()); }\n",
                )][..],
                compare_alone(gc),
            ),
            (
                "joined-by-an-escaped-path",
                dev(&dep("g")),
                &[(
                    "crates/r/tests/compare.rs",
                    "#[path = \"../../\\x63/src/lib.rs\"]\nmod checker;\n#[test]\nfn t() { assert_eq!(checker::x(), g::x()); }\n",
                )][..],
                compare_alone(gc),
            ),
            (
                "joined-by-an-untokenized-file",
                dev(&dep("g")),
                &[(
                    "crates/r/tests/compare.rs",
                    "#[path = \"../../c/src/lib.rs\"]\nmod checker;\n#[test]\nfn t() { 'é: loop { break 'é; } assert_eq!(checker::x(), g::x()); }\n",
                )][..],
                compare_alone(gc),
            ),
            (
                "joined-by-an-aliased-include",
                dev(&dep("g")),
                &[(
                    "crates/r/tests/compare.rs",
                    "use std::include as inc;\nmod checker { super::inc!(\"../../c/src/lib.rs\"); }\n#[test]\nfn t() { assert_eq!(checker::x(), g::x()); }\n",
                )][..],
                compare_alone(gc),
            ),
            (
                "joined-by-a-target-path",
                format!("[[test]]\nname = \"foreign\"\npath = \"../c/src/lib.rs\"\n\n{}", dev(&dep("g"))),
                &[][..],
                unclassified_units("crates/r", &[("test", "foreign")], gc),
            ),
            (
                "a-path-into-no-role",
                dev(&dep("g")),
                &[
                    ("crates/h/src/plain.rs", "pub fn p() {}\n"),
                    (
                        "crates/r/tests/compare.rs",
                        "// mod c: #[path = \"../../c/src/lib.rs\"]\n#[path = \"../../h/src/plain.rs\"]\nmod helper;\n#[test]\nfn t() {}\n",
                    ),
                ][..],
                Vec::new(),
            ),
            (
                "a-file-no-build-compiles",
                dev(&dep("g")),
                &[("crates/r/tests/data/sample.rs", "include!(\"../../../c/src/lib.rs\");\n")][..],
                Vec::new(),
            ),
            // A `dylib`'s doc tests no build makes (round 7's R4).
            (
                "joined-dylib",
                format!("[lib]\ncrate-type = [\"dylib\"]\n\n{}", dev(&(dep("g") + &dep("c")))),
                &[][..],
                {
                    // The library is a `dylib`, a program target of §2 of its own; it has no doc-test unit.
                    let mut units = unclassified_units(
                        "crates/r",
                        &[("bench", "speed"), ("lib-test", "r"), ("test", "compare")],
                        gc,
                    );
                    units.insert(
                        1,
                        (
                            "dylib".to_owned(),
                            "r".to_owned(),
                            "trust-unclassified-program: the dylib `r` of `crates/r` is neither a root nor classified \
                             in trust/roots.eadl"
                                .to_owned(),
                        ),
                    );
                    units
                },
            ),
            ("one-role", dev(&dep("g")), &[][..], Vec::new()),
            ("two-checkers", dev(&(dep("c") + &dep("s"))), &[][..], Vec::new()),
            ("generator-and-model", dev(&(dep("g") + &dep("m"))), &[][..], Vec::new()),
        ] {
            let inv = written(joining_with(case, &manifest_extra, "", extra).run());
            assert_eq!(units_of_r(&inv), want, "{case}");
            // The helper joins both roles; its unit tests and doc tests are off, and some build still makes them.
            assert_eq!(
                units_of(&inv, "crates/h"),
                unclassified_units("crates/h", &[("doctest", "h"), ("lib-test", "h")], gc),
                "{case}: a unit its flags turn off counts"
            );
        }
    }

    #[test]
    fn a_unit_joins_by_what_a_dependency_or_a_chain_of_paths_compiles() {
        crate::trust_test_process::bounded!(
            a_unit_joins_by_what_a_dependency_or_a_chain_of_paths_compiles
        );
        // Round 8's D2: a role package's file a dependency's library compiles joins every unit linking it; one a chain
        // of paths reaches joins the unit whose build read it, and no other.
        let gh = "[dev-dependencies]\ng = { path = \"../g\" }\nh = { path = \"../h\" }\n";
        let h = manifest("h", "");
        let gc = "`crates/c`, `crates/g`";
        let through_h: &[(&str, &str)] = &[
            ("crates/h/Cargo.toml", &h),
            (
                "crates/h/src/lib.rs",
                "#[path = \"../../c/src/lib.rs\"]\npub mod checker;\n",
            ),
            (
                "crates/r/tests/compare.rs",
                "#[test]\nfn t() { assert_eq!(h::checker::x(), g::x()); }\n",
            ),
        ];
        let chain: &[(&str, &str)] = &[
            ("crates/h/Cargo.toml", &h),
            ("crates/h/src/lib.rs", "pub fn h() {}\n"),
            ("crates/h/src/inner.rs", "#[path = \"../../c/src/lib.rs\"]\npub mod checker;\n"),
            (
                "crates/r/tests/compare.rs",
                "#[path = \"../../h/src/inner.rs\"]\nmod inner;\n#[test]\nfn t() { assert_eq!(inner::checker::x(), g::x()); }\n",
            ),
        ];
        for (case, extra, want) in [
            (
                "joined-through-a-dependency",
                through_h,
                four_unclassified(gc),
            ),
            ("joined-through-a-chain", chain, compare_alone(gc)),
        ] {
            let inv = written(joining_with(case, gh, "", extra).run());
            assert_eq!(units_of_r(&inv), want, "{case}");
        }
    }

    #[test]
    fn a_package_with_a_build_script_is_left_out_of_the_census_and_never_run() {
        crate::trust_test_process::bounded!(
            a_package_with_a_build_script_is_left_out_of_the_census_and_never_run
        );
        // `decision_trust-role-programs.md` §3: the census runs no package's code, so a package holding a build script
        // is left out of its build, its units judged by their edges — here, joining the generator and the checker.
        let f = joining_with(
            "census-build-script",
            "[dev-dependencies]\ng = { path = \"../g\" }\nc = { path = \"../c\" }\n",
            "",
            &[(
                "crates/r/build.rs",
                "fn main() { std::fs::write(\"RAN\", \"\").unwrap(); }\n",
            )],
        );
        let inv = written(f.run());
        assert_eq!(
            units_of_r(&inv),
            four_unclassified("`crates/c`, `crates/g`")
        );
        assert!(
            !f.base.join("out/tree/crates/r/RAN").exists(),
            "the build script ran"
        );
    }

    #[test]
    fn a_joined_unit_is_named_by_its_pair_form_or_classified() {
        crate::trust_test_process::bounded!(a_joined_unit_is_named_by_its_pair_form_or_classified);
        // `decision_trust-role-programs.md` §3: a joined integration test a derivation names is that derivation's; any
        // joined unit a `defprogram` classifies is classified, unreviewed again once it compiles more; a classification
        // left on a unit that no longer joins two roles names no program target.
        let joined = "[dev-dependencies]\ng = { path = \"../g\" }\nc = { path = \"../c\" }\n";
        let derivation =
            "(defderivation compare (pair gen cfg) (package \"crates/r\") (test compare) (reason \"the comparison\"))\n";
        let units = |reviewed: &str| {
            format!(
                "(defprogram r-units (package \"crates/r\") (target lib-test r) (reason \"the library's own tests\") \
                 (role-packages {reviewed}))\n"
            )
        };
        let named = units_of_r(&written(
            joining(
                "joined-named",
                joined,
                &format!("{derivation}{}", units("\"crates/c\" \"crates/g\"")),
            )
            .run(),
        ));
        let mut want = four_unclassified("`crates/c`, `crates/g`");
        want[2].2 = "classified r-units".to_owned();
        want[3].2 = "derivation compare".to_owned();
        assert_eq!(named, want, "named and classified");
        // Every joined unit records the pairs it joins, so its remedy names each pair's form (round 6's D3).
        let pairs = joins_of(
            &written(joining("joined-pairs", joined, "").run()),
            "crates/r",
            "test",
        );
        assert_eq!(pairs, ["generator+configuration-checker"]);

        let grown = units_of_r(&written(
            joining("joined-grown", joined, &units("\"crates/g\"")).run(),
        ));
        assert_eq!(
            grown[2].2,
            "trust-unclassified-program: `r-units` compiles `crates/c` beside the role packages its classification \
             reviewed"
        );

        // An executable's unit tests join as a library's do; the executable itself is a program target of §2.
        let binary = written(
            joining_with(
                "joined-binary",
                "",
                "",
                &[
                    (
                        "crates/x/Cargo.toml",
                        "[package]\nname = \"x\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[dependencies]\n\
                         g = { path = \"../g\" }\nc = { path = \"../c\" }\n",
                    ),
                    ("crates/x/src/main.rs", "fn main() {}\n"),
                ],
            )
            .run(),
        );
        let mut want =
            unclassified_units("crates/x", &[("bin-test", "x")], "`crates/c`, `crates/g`");
        want.insert(
            0,
            (
            "bin".to_owned(),
            "x".to_owned(),
            "trust-unclassified-program: the bin `x` of `crates/x` is neither a root nor classified in trust/roots.eadl"
                .to_owned(),
            ),
        );
        assert_eq!(
            units_of(&binary, "crates/x"),
            want,
            "an executable's unit tests"
        );

        // A joined integration test a harness names is the harness's.
        let harness = units_of_r(&written(
            joining(
                "joined-harness",
                "[dev-dependencies]\nm = { path = \"../m\" }\ni = { path = \"../i\" }\n",
                "(defharness diff (pair refm imp) (package \"crates/r\") (test compare))\n",
            )
            .run(),
        ));
        let mut want = four_unclassified("`crates/i`, `crates/m`");
        want[3].2 = "harness diff".to_owned();
        assert_eq!(harness, want, "named by a harness");
        let pairs = joins_of(
            &written(
                joining(
                    "joined-pairs-model",
                    "[dev-dependencies]\nm = { path = \"../m\" }\ni = { path = \"../i\" }\n",
                    "",
                )
                .run(),
            ),
            "crates/r",
            "test",
        );
        assert_eq!(pairs, ["reference-model+implementation"]);

        let gone = written(
            joining(
                "joined-gone",
                "[dev-dependencies]\ng = { path = \"../g\" }\n",
                &units("\"crates/c\" \"crates/g\""),
            )
            .run(),
        );
        assert_eq!(
            gone.get("classifications-unused")
                .map(Json::elements)
                .unwrap_or_default()
                .iter()
                .filter_map(Json::as_str)
                .collect::<Vec<_>>(),
            ["r-units: the lib-test `r` of `crates/r` is no program target of the commit"]
        );
    }

    #[test]
    fn a_build_script_is_refused_before_anything_runs() {
        crate::trust_test_process::bounded!(a_build_script_is_refused_before_anything_runs);
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

    #[cfg(unix)]
    #[test]
    fn a_failed_previous_inventory_cleanup_stops_before_a_forms_refusal() {
        crate::trust_test_process::bounded!(
            a_failed_previous_inventory_cleanup_stops_before_a_forms_refusal
        );
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        struct RestorePermissions(PathBuf, fs::Permissions);
        impl Drop for RestorePermissions {
            fn drop(&mut self) {
                fs::set_permissions(&self.0, self.1.clone())
                    .expect("restore owned output permissions");
            }
        }

        let f = two_roots(
            "old-inventory-cleanup",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[],
        );
        written(f.run());
        let out = f.base.join("out");
        let inventory = out.join("trust-dependencies.json");
        let previous = fs::read(&inventory).unwrap();
        let previous_commit = crate::catalog_check::Git::at(&f.repo)
            .text(&["rev-parse", "HEAD"])
            .unwrap();
        f.commit(&[(
            "trust/roots.eadl",
            format!("{ROOTS}(defgenerated \"data/out.csv\" (generator \"scripts/make.sh\"))\n"),
        )]);
        fs::remove_dir_all(out.join("tree")).unwrap();
        fs::remove_dir_all(out.join("target-metadata")).unwrap();
        fs::remove_dir_all(out.join("target")).unwrap();
        let restore = RestorePermissions(out.clone(), fs::metadata(&out).unwrap().permissions());
        fs::set_permissions(&out, fs::Permissions::from_mode(0o555)).unwrap();
        let deletion_probe = fs::remove_file(&inventory);
        let result = inventory_with(&f.repo, "HEAD", &out, &configuration_roots());
        drop(restore);

        assert!(
            deletion_probe.is_err(),
            "the host did not enforce deletion denial"
        );
        let remaining = fs::read(&inventory).unwrap();
        assert_eq!(remaining, previous);
        let retained = crate::json::parse(std::str::from_utf8(&remaining).unwrap()).unwrap();
        assert_eq!(
            retained
                .get("identity")
                .and_then(|v| v.get("commit"))
                .and_then(Json::as_str),
            Some(previous_commit.trim())
        );
        assert_ne!(
            crate::catalog_check::Git::at(&f.repo)
                .text(&["rev-parse", "HEAD"])
                .unwrap(),
            previous_commit
        );
        let error = match result {
            Err(error) => error,
            Ok(Outcome::Refused(why)) => {
                panic!("an ordinary forms refusal left the previous inventory readable: {why:?}")
            }
            Ok(Outcome::Written(_)) => panic!("failed cleanup produced a written inventory"),
        };
        assert!(
            error.starts_with("cannot remove previous inventory `"),
            "{error}"
        );
        assert!(error.contains(inventory.to_str().unwrap()), "{error}");
        assert!(!out.join("tree").exists());
        assert!(!out.join("target-metadata").exists());
        assert!(!out.join("target").exists());
        assert!(!refused(f.run()).is_empty());
        assert!(
            !inventory.exists(),
            "ordinary refusal must clear the earlier inventory"
        );
    }

    #[test]
    fn a_directory_at_the_inventory_path_stops_before_writing_the_tree() {
        crate::trust_test_process::bounded!(
            a_directory_at_the_inventory_path_stops_before_writing_the_tree
        );
        let f = two_roots(
            "inventory-output-directory",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[],
        );
        let out = f.base.join("out");
        let inventory = out.join("trust-dependencies.json");
        std::fs::create_dir_all(&inventory).unwrap();
        std::fs::write(inventory.join("keep"), b"owned contents").unwrap();
        let error = match inventory_with(&f.repo, "HEAD", &out, &configuration_roots()) {
            Err(error) => error,
            Ok(_) => panic!("a directory at the inventory path must stop the invocation"),
        };
        assert!(
            !out.join("tree").exists(),
            "the source tree was written before rejecting the inventory output directory"
        );
        assert!(
            error.starts_with("cannot remove previous inventory `"),
            "{error}"
        );
        assert!(error.contains(inventory.to_str().unwrap()), "{error}");
        assert_eq!(
            std::fs::read(inventory.join("keep")).unwrap(),
            b"owned contents"
        );
        assert!(!out.join("target-metadata").exists());
        assert!(!out.join("target").exists());
        std::fs::remove_dir_all(&inventory).unwrap();
        written(f.run());
    }

    #[cfg(unix)]
    #[test]
    fn a_failed_cargo_home_reset_stops_before_metadata_or_wrapper_execution() {
        crate::trust_test_process::bounded!(
            a_failed_cargo_home_reset_stops_before_metadata_or_wrapper_execution
        );
        use std::fs;
        use std::os::unix::fs::PermissionsExt;

        struct RestorePermissions(PathBuf, fs::Permissions);
        impl Drop for RestorePermissions {
            fn drop(&mut self) {
                fs::set_permissions(&self.0, self.1.clone())
                    .expect("restore owned home permissions");
            }
        }

        let f = two_roots("hard-home-reset", "fn main() {}\n", "pub fn f() {}\n", &[]);
        written(f.run()); // Absent home and ordinary static compilation must succeed first.
        let out = f.base.join("out");
        let home = out.join("cargo-home");
        let wrapper = f.base.join("wrapper.sh");
        let sentinel = f.base.join("WRAPPER_RAN");
        fs::write(
            &wrapper,
            "#!/bin/sh\nprintf 'wrapper ran' > \"$(dirname \"$0\")/WRAPPER_RAN\"\nexec \"$@\"\n",
        )
        .unwrap();
        fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o755)).unwrap();
        fs::write(
            home.join("config.toml"),
            format!(
                "[build]\nrustc-wrapper = {}\n",
                crate::json::write(&Json::Str(wrapper.to_str().unwrap().to_owned()))
            ),
        )
        .unwrap();
        // Keep exactly the populated Cargo home, but expose any new metadata or compilation work.
        fs::remove_dir_all(out.join("target-metadata")).unwrap();
        fs::remove_dir_all(out.join("target")).unwrap();
        let restore = RestorePermissions(home.clone(), fs::metadata(&home).unwrap().permissions());
        fs::set_permissions(&home, fs::Permissions::from_mode(0o555)).unwrap();
        let permission_probe = fs::write(home.join("permission-probe"), b"must fail");
        let result = inventory_with(&f.repo, "HEAD", &out, &configuration_roots());
        drop(restore); // Restore before assertions, also on unwinding through the scoped guard.

        assert!(
            permission_probe.is_err(),
            "this host did not enforce the fixture permissions"
        );
        assert!(
            !sentinel.exists(),
            "a surviving Cargo-home wrapper executed"
        );
        let error = match result {
            Err(error) => error,
            Ok(_) => panic!("a failed Cargo-home reset did not stop the inventory"),
        };
        assert!(
            error.starts_with("cannot reset inventory Cargo home `"),
            "{error}"
        );
        assert!(error.contains(home.to_str().unwrap()), "{error}");
        assert!(!out.join("target-metadata").exists());
        assert!(!out.join("target").exists());
        assert!(!out.join("trust-dependencies.json").exists());
        // A writable populated home must be cleared, including the untracked wrapper configuration.
        written(f.run());
        assert!(!home.join("config.toml").exists());
        assert!(!sentinel.exists());
    }

    #[test]
    fn cargo_home_reset_accepts_absence_and_clears_data_but_rejects_a_file() {
        crate::trust_test_process::bounded!(
            cargo_home_reset_accepts_absence_and_clears_data_but_rejects_a_file
        );
        let f = two_roots(
            "home-reset-states",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[],
        );
        let home = f.base.join("home");
        super::reset_cargo_home(&home).expect("an absent home is safe");
        std::fs::create_dir_all(home.join("registry/cache")).unwrap();
        std::fs::write(home.join("config.toml"), "[build]\n").unwrap();
        std::fs::write(home.join("registry/cache/package"), b"old cache").unwrap();
        super::reset_cargo_home(&home).expect("a writable home is removed completely");
        assert!(!home.exists());
        std::fs::write(&home, b"not a directory").unwrap();
        let error =
            super::reset_cargo_home(&home).expect_err("an actual removal failure must stop");
        assert!(
            error.starts_with("cannot reset inventory Cargo home `"),
            "{error}"
        );
        assert!(error.contains(home.to_str().unwrap()), "{error}");
        assert_eq!(std::fs::read(&home).unwrap(), b"not a directory");
        let out = f.base.join("out");
        std::fs::create_dir_all(&out).unwrap();
        std::fs::rename(&home, out.join("cargo-home")).unwrap();
        let error = match inventory_with(&f.repo, "HEAD", &out, &configuration_roots()) {
            Err(error) => error,
            Ok(_) => panic!("a file at Cargo home did not stop the inventory"),
        };
        assert!(
            error.starts_with("cannot reset inventory Cargo home `"),
            "{error}"
        );
        assert!(!out.join("target-metadata").exists());
        assert!(!out.join("target").exists());
        assert!(!out.join("trust-dependencies.json").exists());
        std::fs::remove_file(out.join("cargo-home")).unwrap();
        written(f.run());
    }

    fn repeated_generator_preflight(dynamic: bool) {
        for order in [["one", "two"], ["two", "one"]] {
            let accepted = manifest("g", if dynamic { "build = false\n" } else { "" });
            let rejected = manifest(
                "g",
                if dynamic {
                    ""
                } else {
                    "[features]\ndefault = [\"extra\"]\nextra = []\n"
                },
            );
            let roots = |text: &str| {
                let mut value = format!(
                    "{ROOTS}(defgenerated \"crates/a/src/main.rs\" \
                     (generator \"crates/g/src/bin/{}.rs\" \"crates/g/src/bin/{}.rs\") \
                     (command \"fixture\") (reason \"fixture\"))\n",
                    order[0], order[1],
                );
                for name in ["one", "two"] {
                    value.push_str(&format!(
                        "(defprogram generator-{name} (package \"crates/g\") \
                         (target bin {name}) (reason \"fixture\") (role-packages \"crates/g\"))\n"
                    ));
                }
                if dynamic {
                    value.push_str(&admit(
                        "crates/g/Cargo.toml",
                        "`crates/g/build.rs`: a package has no build script",
                        text,
                    ));
                }
                value
            };
            let f = two_roots(
                &format!("generator-preflight-cache-{dynamic}-{}", order[0]),
                "fn main() {}\n",
                "pub fn f() {}\n",
                &[
                    ("crates/g/Cargo.toml", accepted.clone()),
                    ("crates/g/src/bin/one.rs", "fn main() {}\n".to_owned()),
                    ("crates/g/src/bin/two.rs", "fn main() {}\n".to_owned()),
                    ("trust/roots.eadl", roots(&accepted)),
                ],
            );
            if dynamic {
                f.commit(&[(
                    "crates/g/build.rs",
                    "fn main() { std::fs::write(\"RAN\", \"build script ran\").unwrap(); }\n"
                        .to_owned(),
                )]);
            }
            let check = |inventory: &Json| {
                assert_eq!(
                    inventory.get("refused-sites").and_then(Json::as_str),
                    Some(if dynamic { "1" } else { "0" })
                );
                for name in ["one", "two"] {
                    let targets = inventory.get("program-targets").unwrap().elements();
                    let target = targets
                        .iter()
                        .find(|target| {
                            target.get("package").and_then(Json::as_str) == Some("crates/g")
                                && target.get("name").and_then(Json::as_str) == Some(name)
                        })
                        .unwrap();
                    assert_eq!(
                        target.get("status").and_then(Json::as_str),
                        Some(format!("classified generator-{name}").as_str())
                    );
                }
                assert!(inventory
                    .get("admissions-unused")
                    .unwrap()
                    .elements()
                    .is_empty());
                assert!(inventory
                    .get("classifications-unused")
                    .unwrap()
                    .elements()
                    .is_empty());
            };
            let ordinary = written(f.run());
            check(&ordinary);
            assert!(f
                .base
                .join("out/target-generators/crates-g-bin-one")
                .is_dir());
            assert!(f
                .base
                .join("out/target-generators/crates-g-bin-two")
                .is_dir());
            assert!(!f.base.join("out/tree/crates/g/RAN").exists());
            // The bad phase must not mistake the successful prior products for a new build.
            std::fs::remove_dir_all(f.base.join("out/target-generators")).unwrap();
            f.commit(&[
                ("crates/g/Cargo.toml", rejected.clone()),
                ("trust/roots.eadl", roots(&rejected)),
            ]);
            let outcome = f.run();
            let ran = f.base.join("out/tree/crates/g/RAN").exists();
            let compiled = f.base.join("out/target-generators").exists();
            let wrote_inventory = f.base.join("out/trust-dependencies.json").exists();
            // Hold the bad-case observations while a normal third invocation recovers.
            f.commit(&[
                ("crates/g/Cargo.toml", accepted.clone()),
                ("trust/roots.eadl", roots(&accepted)),
            ]);
            let recovered = written(f.run());
            check(&recovered);
            assert_eq!(items(&recovered), items(&ordinary));
            assert!(f
                .base
                .join("out/target-generators/crates-g-bin-one")
                .is_dir());
            assert!(f
                .base
                .join("out/target-generators/crates-g-bin-two")
                .is_dir());
            assert!(!f.base.join("out/tree/crates/g/RAN").exists());
            assert!(
                !ran,
                "an earlier refusal let the repeated package run its script"
            );
            assert!(
                !compiled,
                "an earlier refusal let the repeated package compile"
            );
            assert!(!wrote_inventory);
            let why = refused(outcome);
            for name in order {
                says(&why, &format!("generator crates/g/src/bin/{name}.rs"));
            }
            if dynamic {
                says(&why, "custom-build");
            } else {
                says(&why, "[features]");
                says(&why, "no admission names it");
            }
        }
    }

    #[test]
    fn repeated_generator_build_script_refusals_prevent_compilation() {
        crate::trust_test_process::bounded!(
            repeated_generator_build_script_refusals_prevent_compilation
        );
        repeated_generator_preflight(true);
    }

    #[test]
    fn repeated_generator_manifest_refusals_prevent_compilation() {
        crate::trust_test_process::bounded!(
            repeated_generator_manifest_refusals_prevent_compilation
        );
        repeated_generator_preflight(false);
    }

    #[test]
    fn manifest_admissions_are_counted_once_for_a_middle_shared_package() {
        use std::collections::{BTreeMap, BTreeSet};

        use archogen_catalog::{manifest, package, tree::Tree};

        use super::{Admission, Rules};

        crate::trust_test_process::bounded!(
            manifest_admissions_are_counted_once_for_a_middle_shared_package
        );
        // Cargo metadata qualifies these files and targets: c is in the middle of both closures.
        // One admitted manifest site remains one site across both callers and either caller order.
        let tree = Tree::new(
            [
            ("Cargo.toml", r#"[workspace]
resolver = "2"
members = ["crates/*"]
"#),
            ("rust-toolchain.toml", r#"[toolchain]
channel = "1.95.0"
"#),
            (".cargo/config.toml", r#"# `cargo xtask …` runs the repository's own verification runner (ROADMAP.md §4.2, §14.3).
# Committed deliberately: a tier that only works if each developer remembers an incantation is a
# tier nobody runs.
[alias]
xtask = "run --quiet --package xtask --"
"#),
            ("crates/a/Cargo.toml", r#"[package]
name = "a"
version = "0.1.0"
edition = "2021"
[dependencies]
c = { path = "../c" }
z = { path = "../z" }
"#),
            ("crates/a/src/main.rs", r#"fn main() { assert_eq!(c::f() + z::f(), 18); }
"#),
            ("crates/b/Cargo.toml", r#"[package]
name = "b"
version = "0.1.0"
edition = "2021"
[dependencies]
c = { path = "../c" }
z = { path = "../z" }
"#),
            ("crates/b/src/lib.rs", r#"pub fn check() -> u32 { c::f() + z::f() }
"#),
            ("crates/c/Cargo.toml", r#"[package]
name = "c"
version = "0.1.0"
edition = "2021"
[features]
extra = []
"#),
            ("crates/c/src/lib.rs", r#"pub fn f() -> u32 { 7 }
"#),
            ("crates/z/Cargo.toml", r#"[package]
name = "z"
version = "0.1.0"
edition = "2021"
"#),
            ("crates/z/src/lib.rs", r#"pub fn f() -> u32 { 11 }
"#),
            ("Cargo.lock", r#"# This file is automatically @generated by Cargo.
# It is not intended for manual editing.
version = 4

[[package]]
name = "a"
version = "0.1.0"
dependencies = [
 "c",
 "z",
]

[[package]]
name = "b"
version = "0.1.0"
dependencies = [
 "c",
 "z",
]

[[package]]
name = "c"
version = "0.1.0"

[[package]]
name = "z"
version = "0.1.0"
"#),
            ]
            .into_iter()
            .map(|(path, bytes)| (path.to_owned(), bytes.as_bytes().to_vec())),
        );
        let manifest_sha = Digest::of(tree.get("crates/c/Cargo.toml").unwrap()).hex();
        assert_eq!(
            manifest_sha,
            "6feed7ef8e7fe5dd89c4d9abd102bde741fedb396571aba27e85527f912fc220"
        );
        let admitted_package = "crates/c";
        let parsed_manifest = manifest::parse(&String::from_utf8_lossy(
            tree.get("crates/c/Cargo.toml").unwrap(),
        ))
        .unwrap();
        let rule = package::check_manifest(
            &tree,
            admitted_package,
            "crates/c/Cargo.toml",
            &parsed_manifest,
        )
        .expect_err("the middle shared manifest must require an admission");
        let admission = Admission {
            file: "crates/c/Cargo.toml".to_owned(),
            rule: rule.to_owned(),
            sha256: manifest_sha.clone(),
        };
        let symlinks = BTreeSet::new();
        let package_targets = BTreeMap::from([
            (
                "crates/a".to_owned(),
                vec!["crates/a/src/main.rs".to_owned()],
            ),
            (
                "crates/b".to_owned(),
                vec!["crates/b/src/lib.rs".to_owned()],
            ),
            (
                "crates/c".to_owned(),
                vec!["crates/c/src/lib.rs".to_owned()],
            ),
            (
                "crates/z".to_owned(),
                vec!["crates/z/src/lib.rs".to_owned()],
            ),
        ]);
        let metadata_targets = ["a", "b", "c", "z"]
            .into_iter()
            .map(|name| {
                (
                    format!("crates/{name}"),
                    vec![(
                        vec![if name == "a" { "bin" } else { "lib" }.to_owned()],
                        name.to_owned(),
                    )],
                )
            })
            .collect();
        let closures: [(&str, BTreeSet<String>); 2] = [
            (
                "gen",
                ["crates/a", "crates/c", "crates/z"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
            ),
            (
                "chk",
                ["crates/b", "crates/c", "crates/z"]
                    .into_iter()
                    .map(str::to_owned)
                    .collect(),
            ),
        ];
        for order in [[0, 1], [1, 0]] {
            for (admitted, phase) in [(true, "accepted"), (false, "refused"), (true, "recovered")] {
                let admissions = if admitted {
                    BTreeMap::from([(admission.clone(), 1)])
                } else {
                    BTreeMap::new()
                };
                let mut rules = Rules {
                    tree: &tree,
                    tree_dir: Path::new("."),
                    symlinks: &symlinks,
                    admissions: &admissions,
                    package_targets: &package_targets,
                    metadata_targets: &metadata_targets,
                    sites: 0,
                    used: BTreeMap::new(),
                    manifests_checked: BTreeSet::new(),
                    judged: BTreeSet::new(),
                };
                let mut refusals = Vec::new();
                for index in order {
                    let (who, closure) = &closures[index];
                    rules.manifests(who, closure, &mut refusals);
                }
                if admitted {
                    assert!(refusals.is_empty(), "{phase} {order:?}: {refusals:?}");
                    assert_eq!(
                        rules.sites, 1,
                        "{phase} {order:?}: one shared manifest site"
                    );
                    assert_eq!(
                        rules.used,
                        BTreeMap::from([(admission.clone(), 1)]),
                        "{phase} {order:?}: one admission use"
                    );
                    assert_eq!(
                        rules.manifests_checked,
                        ["crates/a", "crates/b", "crates/c", "crates/z"]
                            .into_iter()
                            .map(str::to_owned)
                            .collect()
                    );
                } else {
                    assert_eq!(rules.sites, 2, "{phase} {order:?}: each caller is refused");
                    assert!(rules.used.is_empty());
                    let expected: Vec<String> = order
                        .into_iter()
                        .map(|index| {
                            let who = closures[index].0;
                            format!(
                                "trust-undeclared-input: `{who}` reaches `{admitted_package}`: {rule}, \
                                 and no admission names it (manifest sha256 {manifest_sha})"
                            )
                        })
                        .collect();
                    assert_eq!(refusals, expected, "{phase} {order:?}");
                    assert_eq!(
                        rules.manifests_checked,
                        ["crates/a", "crates/b", "crates/z"]
                            .into_iter()
                            .map(str::to_owned)
                            .collect(),
                        "{phase} {order:?}: a refused package is not cached"
                    );
                }
            }
        }
    }

    #[test]
    fn an_admitted_build_script_is_refused_before_any_compilation() {
        crate::trust_test_process::bounded!(
            an_admitted_build_script_is_refused_before_any_compilation
        );
        for enabled in [false, true] {
            let manifest = manifest("b", if enabled { "" } else { "build = false\n" });
            let rule = "`crates/b/build.rs`: a package has no build script";
            let f = two_roots(
                &format!("admitted-build-script-{enabled}"),
                "fn main() {}\n",
                "pub fn f() {}\n",
                &[
                    ("crates/b/Cargo.toml", manifest.clone()),
                    (
                        "crates/b/build.rs",
                        "fn main() { std::fs::write(\"RAN\", \"build script ran\").unwrap(); }\n"
                            .to_owned(),
                    ),
                    (
                        "trust/roots.eadl",
                        format!("{ROOTS}{}", admit("crates/b/Cargo.toml", rule, &manifest)),
                    ),
                ],
            );
            let outcome = f.run();
            assert!(
                !f.base.join("out/tree/crates/b/RAN").exists(),
                "an admitted build script ran before refusal"
            );
            if enabled {
                let why = refused(outcome);
                says(&why, "reaches `crates/b`");
                says(&why, "custom-build");
                assert!(
                    !f.base.join("out/target/gen").exists(),
                    "a root compiled before metadata preflight ended"
                );
                assert!(!f.base.join("out/target/chk").exists());
            } else {
                // An ignored script is not a dynamic target. Its ordinary manifest admission still works.
                let inv = written(outcome);
                let units = program(&inv, "chk").get("units").unwrap().elements();
                assert_eq!(units.len(), 1);
                assert_eq!(units[0].get("crate").and_then(Json::as_str), Some("b"));
            }
        }
    }

    #[test]
    fn an_admitted_procedural_macro_is_refused_before_any_compilation() {
        crate::trust_test_process::bounded!(
            an_admitted_procedural_macro_is_refused_before_any_compilation
        );
        let manifest = manifest("p", "[lib]\nproc-macro = true\n");
        let rule = "`crates/p/Cargo.toml`: `lib.proc-macro` holds `proc-macro`, which can make the package another kind of crate";
        let f = two_roots(
            "admitted-procedural-macro",
            "p::mark!();\nfn main() {}\n",
            "pub fn f() {}\n",
            &[
                ("crates/a/Cargo.toml", self::manifest("a", "[dependencies]\np = { path = \"../p\" }\n")),
                ("crates/p/Cargo.toml", manifest.clone()),
                (
                    "crates/p/src/lib.rs",
                    "extern crate proc_macro;\n#[proc_macro]\npub fn mark(_: proc_macro::TokenStream) -> proc_macro::TokenStream { std::fs::write(\"PROC_RAN\", \"procedural macro ran\").unwrap(); proc_macro::TokenStream::new() }\n".to_owned(),
                ),
                ("trust/roots.eadl", format!("{ROOTS}{}", admit("crates/p/Cargo.toml", rule, &manifest))),
            ],
        );
        let outcome = f.run();
        assert!(
            !f.base.join("out/tree/PROC_RAN").exists(),
            "an admitted procedural macro ran before refusal"
        );
        let why = refused(outcome);
        says(&why, "reaches `crates/p`");
        says(&why, "proc-macro");
        assert!(
            !f.base.join("out/target/gen").exists(),
            "a root compiled before metadata preflight ended"
        );
        assert!(!f.base.join("out/target/chk").exists());
    }

    #[test]
    fn an_admitted_site_passes_and_an_edited_one_is_refused() {
        crate::trust_test_process::bounded!(an_admitted_site_passes_and_an_edited_one_is_refused);
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

    #[test]
    fn every_absolute_dependency_outside_the_written_tree_is_refused() {
        crate::trust_test_process::bounded!(
            every_absolute_dependency_outside_the_written_tree_is_refused
        );
        let name = "absolute-dependency-refusals";
        let external = real_root()
            .join("target/trust-tests")
            .join(name)
            .join("external");
        let inputs = [
            ("FIRST", "a.txt", "first external input\n"),
            ("MIDDLE", "m.txt", "middle external input\n"),
            ("LAST", "z.txt", "last external input\n"),
        ];
        let lines: Vec<_> = inputs
            .iter()
            .map(|(constant, file, _)| {
                format!(
                    "pub const {constant}: &str = include_str!({:?});",
                    external.join(file).to_str().unwrap()
                )
            })
            .collect();
        let admissions: String = lines
            .iter()
            .map(|line| admit("crates/b/src/lib.rs", "include_str", line))
            .collect();
        let f = two_roots(
            name,
            "fn main() {}\n",
            &(lines.join("\n") + "\n"),
            &[("trust/roots.eadl", format!("{ROOTS}{admissions}"))],
        );
        std::fs::create_dir_all(&external).unwrap();
        for (_, file, contents) in inputs {
            std::fs::write(external.join(file), contents).unwrap();
        }

        let expected: Vec<_> = inputs
            .iter()
            .map(|(_, file, _)| {
                format!(
                    "trust-undeclared-input: `chk` reads `{}`, outside the written tree",
                    external.join(file).display()
                )
            })
            .collect();
        assert_eq!(refused(f.run()), expected);
        assert!(!f.base.join("out/trust-dependencies.json").exists());
    }

    /// The written tree's path holds a space, an outside input is named by the text before that space with a
    /// trailing backslash, and an in-tree prerequisite follows it: rustc's first rule spells the two as one escaped
    /// in-tree path while its per-file targets spell them apart. The inventory refuses the raw rule instead of
    /// reading the decoy blob held at the tree root (`M3.6.9.6.3.2.2.15.2.1.6.1.1.1`, D1).
    #[test]
    fn a_trailing_backslash_before_an_in_tree_prerequisite_is_refused_as_ambiguous() {
        crate::trust_test_process::bounded!(
            a_trailing_backslash_before_an_in_tree_prerequisite_is_refused_as_ambiguous
        );
        let name = "dependency-prefix-collision";
        let outside = real_root()
            .join("target/trust-tests")
            .join(name)
            .join("prefix\\");
        let lines = [
            format!(
                "pub const OUTSIDE: &str = include_str!({:?});",
                outside.to_str().unwrap()
            ),
            "pub const INSIDE: &str = include_str!(\"tree/data.txt\");".to_owned(),
        ];
        let admissions: String = lines
            .iter()
            .map(|line| admit("crates/b/src/lib.rs", "include_str", line))
            .collect();
        let f = two_roots(
            name,
            "fn main() {}\n",
            &(lines.join("\n") + "\n"),
            &[
                ("trust/roots.eadl", format!("{ROOTS}{admissions}")),
                (
                    "crates/b/src/tree/data.txt",
                    "held decoy bytes\n".to_owned(),
                ),
                ("data.txt", "held decoy bytes\n".to_owned()),
            ],
        );
        std::fs::write(&outside, "outside payload\n").unwrap();
        let out = "prefix crates/b/src";
        let expected = format!(
            "trust-undeclared-input: `chk` reads `crates/b/src/lib.rs {} crates/b/src/tree/data.txt` ambiguously: \
             its per-file targets spell the prerequisites otherwise",
            outside.display()
        );
        assert_eq!(refused(f.run_into(out)), [expected]);
        assert!(!f.base.join(out).join("trust-dependencies.json").exists());
    }

    /// Three outside inputs whose names hold spaces: rustc escapes each space as backslash-space, so a backslash
    /// outside the written tree's known prefixes may be an escape or a name's own byte. The inventory refuses the
    /// raw rule rather than splitting it, under the raw output root and under a symbolic link to it (D2).
    #[cfg(unix)]
    #[test]
    fn outside_inputs_with_spaces_are_refused_by_their_raw_rule() {
        crate::trust_test_process::bounded!(
            outside_inputs_with_spaces_are_refused_by_their_raw_rule
        );
        for (name, out) in [
            ("dependency-spaced-refusals", "out"),
            ("dependency-spaced-alias-refusals", "output-alias"),
        ] {
            let base = real_root().join("target/trust-tests").join(name);
            let external = base.join("external");
            let inputs = [
                ("FIRST", "a with space.txt", "first external input\n"),
                ("MIDDLE", "m with space.txt", "middle external input\n"),
                ("LAST", "z with space.txt", "last external input\n"),
            ];
            let mut lines: Vec<_> = inputs
                .iter()
                .map(|(constant, file, _)| {
                    format!(
                        "pub const {constant}: &str = include_str!({:?});",
                        external.join(file).to_str().unwrap()
                    )
                })
                .collect();
            let mut extra = vec![];
            if out != "out" {
                lines.push("pub const INSIDE: &str = include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/inside.txt\"));".to_owned());
                extra.push((
                    "crates/b/src/inside.txt",
                    "inside tracked input\n".to_owned(),
                ));
            }
            let admissions: String = lines
                .iter()
                .map(|line| admit("crates/b/src/lib.rs", "include_str", line))
                .collect();
            extra.push(("trust/roots.eadl", format!("{ROOTS}{admissions}")));
            let f = two_roots(name, "fn main() {}\n", &(lines.join("\n") + "\n"), &extra);
            std::fs::create_dir_all(&external).unwrap();
            for (_, file, contents) in inputs {
                std::fs::write(external.join(file), contents).unwrap();
            }
            let mut rule = vec!["crates/b/src/lib.rs".to_owned()];
            rule.extend(inputs.iter().map(|(_, file, _)| {
                external
                    .join(file)
                    .display()
                    .to_string()
                    .replace(' ', "\\ ")
            }));
            if out != "out" {
                let physical = base.join("physical-output");
                std::fs::create_dir_all(&physical).unwrap();
                std::os::unix::fs::symlink(&physical, base.join(out)).unwrap();
                rule.push(
                    std::fs::canonicalize(&physical)
                        .unwrap()
                        .join("tree/crates/b/src/inside.txt")
                        .display()
                        .to_string(),
                );
            }
            let expected = format!(
                "trust-undeclared-input: `chk` reads `{}` ambiguously: a backslash outside the written tree's \
                 prefixes",
                rule.join(" ")
            );
            assert_eq!(refused(f.run_into(out)), [expected], "{name}");
            assert!(!f.base.join(out).join("trust-dependencies.json").exists());
        }
    }

    /// A symbolic link as the output root: rustc spells an in-tree input by the raw root when its site names it so
    /// and by the canonical root when the manifest directory reaches it. Both are the written tree, the input is
    /// accepted once, and the three outside inputs are refused exactly.
    #[cfg(unix)]
    #[test]
    fn a_symbolic_output_root_accepts_raw_and_canonical_in_tree_inputs() {
        crate::trust_test_process::bounded!(
            a_symbolic_output_root_accepts_raw_and_canonical_in_tree_inputs
        );
        let name = "dependency-alias-refusals";
        let base = real_root().join("target/trust-tests").join(name);
        let external = base.join("external");
        let inputs = [
            ("FIRST", "a.txt", "first external input\n"),
            ("MIDDLE", "m.txt", "middle external input\n"),
            ("LAST", "z.txt", "last external input\n"),
        ];
        let mut lines: Vec<_> = inputs
            .iter()
            .map(|(constant, file, _)| {
                format!(
                    "pub const {constant}: &str = include_str!({:?});",
                    external.join(file).to_str().unwrap()
                )
            })
            .collect();
        lines.push(format!(
            "pub const RAW: &str = include_str!({:?});",
            base.join("output-alias/tree/crates/b/src/inside.txt")
                .to_str()
                .unwrap()
        ));
        lines.push("pub const CANONICAL: &str = include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/inside.txt\"));".to_owned());
        let admissions: String = lines
            .iter()
            .map(|line| admit("crates/b/src/lib.rs", "include_str", line))
            .collect();
        let f = two_roots(
            name,
            "fn main() {}\n",
            &(lines.join("\n") + "\n"),
            &[
                ("trust/roots.eadl", format!("{ROOTS}{admissions}")),
                (
                    "crates/b/src/inside.txt",
                    "inside tracked input\n".to_owned(),
                ),
            ],
        );
        std::fs::create_dir_all(&external).unwrap();
        for (_, file, contents) in inputs {
            std::fs::write(external.join(file), contents).unwrap();
        }
        let physical = base.join("physical-output");
        std::fs::create_dir_all(&physical).unwrap();
        std::os::unix::fs::symlink(&physical, base.join("output-alias")).unwrap();
        let expected: Vec<_> = inputs
            .iter()
            .map(|(_, file, _)| {
                format!(
                    "trust-undeclared-input: `chk` reads `{}`, outside the written tree",
                    external.join(file).display()
                )
            })
            .collect();
        assert_eq!(refused(f.run_into("output-alias")), expected);
        assert!(!f.base.join("output-alias/trust-dependencies.json").exists());
    }

    /// Three outside inputs whose names end in a backslash: rustc writes each literally, so the first rule's final
    /// prerequisite ends in a backslash, and the pinned Cargo refuses its own dependency record before the inventory
    /// reads anything. That is the build's error, not a judgment: the run returns Cargo's message, writes nothing and
    /// lists no refusal. The record rustc left is one the reader refuses as ambiguous, so the inventory does not rest
    /// on Cargo's refusal (`M3.6.9.6.3.2.2.15.2.1.6.1.1.2`, D3). A backslash is a name's own byte only on Unix.
    #[cfg(unix)]
    #[test]
    fn a_final_prerequisite_ending_in_a_backslash_is_the_builds_own_error() {
        crate::trust_test_process::bounded!(
            a_final_prerequisite_ending_in_a_backslash_is_the_builds_own_error
        );
        let name = "dependency-terminal-backslashes";
        let base = real_root().join("target/trust-tests").join(name);
        let external = base.join("external");
        let inputs = [
            ("FIRST", "a\\", "first external input\n"),
            ("MIDDLE", "m\\", "middle external input\n"),
            ("LAST", "z\\", "last external input\n"),
        ];
        let lines: Vec<_> = inputs
            .iter()
            .map(|(constant, file, _)| {
                format!(
                    "pub const {constant}: &str = include_str!({:?});",
                    external.join(file).to_str().unwrap()
                )
            })
            .collect();
        let admissions: String = lines
            .iter()
            .map(|line| admit("crates/b/src/lib.rs", "include_str", line))
            .collect();
        let f = two_roots(
            name,
            "fn main() {}\n",
            &(lines.join("\n") + "\n"),
            &[("trust/roots.eadl", format!("{ROOTS}{admissions}"))],
        );
        std::fs::create_dir_all(&external).unwrap();
        for (_, file, contents) in inputs {
            std::fs::write(external.join(file), contents).unwrap();
        }
        let error = match f.try_run_into("out") {
            Ok(Outcome::Written(_)) => panic!("written where the build fails"),
            Ok(Outcome::Refused(r)) => panic!("refused where the build fails: {r:#?}"),
            Err(error) => error,
        };
        // The record rustc wrote, by its real name: Cargo's message names it, and its first rule ends in a backslash.
        let records: Vec<_> = std::fs::read_dir(base.join("out/target/chk/release/deps"))
            .unwrap()
            .map(|entry| entry.unwrap().path())
            .filter(|path| {
                path.extension().is_some_and(|e| e == "d")
                    && path
                        .file_name()
                        .is_some_and(|n| n.to_string_lossy().starts_with("b-"))
            })
            .collect();
        assert_eq!(records.len(), 1, "one record of `b`: {records:?}");
        let record = &records[0];
        assert_eq!(
            error,
            format!(
                "`cargo build --lib --release --locked --offline --no-default-features --message-format=json -v \
                 -p b` failed: error: could not parse/generate dep info at: {}",
                record.display()
            )
        );
        assert!(!base.join("out/trust-dependencies.json").exists());
        let rule = std::iter::once("crates/b/src/lib.rs".to_owned())
            .chain(
                inputs
                    .iter()
                    .map(|(_, file, _)| external.join(file).display().to_string()),
            )
            .collect::<Vec<_>>()
            .join(" ");
        let text = std::fs::read_to_string(record).unwrap();
        assert!(
            text.starts_with(&format!("{}: {rule}\n", record.display())),
            "{text}"
        );
        let scratch = format!("{}/", base.join("out/tree").display());
        let canonical = std::fs::canonicalize(base.join("out/tree"))
            .map(|c| format!("{}/", c.display()))
            .unwrap_or_else(|_| scratch.clone());
        assert_eq!(
            ambiguous(super::dependency_files(&text, record, &scratch, &canonical).unwrap()),
            (rule, "a backslash outside the written tree's prefixes")
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
        crate::trust_test_process::bounded!(a_new_consumer_of_a_shared_package_changes_its_edges);
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
        crate::trust_test_process::bounded!(a_profile_edit_changes_the_build_configuration);
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
        crate::trust_test_process::bounded!(
            a_feature_another_manifest_enables_does_not_reach_a_root_s_build
        );
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
        crate::trust_test_process::bounded!(naked_asm_is_refused);
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
        crate::trust_test_process::bounded!(
            a_new_reader_of_a_shared_package_s_file_changes_its_readers
        );
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
        crate::trust_test_process::bounded!(
            the_configuration_is_the_same_from_two_directories_and_two_toolchains
        );
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
    fn cargo_diagnostic_flags_do_not_enter_unit_configurations() {
        crate::trust_test_process::bounded!(
            cargo_diagnostic_flags_do_not_enter_unit_configurations
        );
        let cfg_operands = ["cfg(docsrs,test)", "cfg(feature, values())"];
        let mut ordinary = None;
        for level in ["ordinary", "allow", "warn", "deny", "forbid"] {
            let lints = if level == "ordinary" {
                String::new()
            } else {
                format!("[lints.rust]\nunused_variables = \"{level}\"\nunsafe_code = \"{level}\"\ndead_code = \"{level}\"\n")
            };
            let dependency = "[dependencies]\ncommon = { path = \"../common\" }\n";
            let f = two_roots(
                &format!("cargo-diagnostic-flags-{level}"),
                "fn main() { common::c(); }\n",
                "pub fn f() { common::c(); }\n",
                &[
                    ("crates/common/Cargo.toml", manifest("common", &lints)),
                    ("crates/common/src/lib.rs", "pub fn c() {}\n".to_owned()),
                    (
                        "crates/a/Cargo.toml",
                        manifest("a", &format!("{dependency}\n{lints}")),
                    ),
                    (
                        "crates/b/Cargo.toml",
                        manifest("b", &format!("{dependency}\n{lints}")),
                    ),
                ],
            );
            // Successful real producer evidence binds the operands; a synthetic argv is insufficient.
            let env = super::environment(
                "1.95.0",
                &f.base.join("producer/cargo-home"),
                &f.base.join("producer/target"),
            )
            .unwrap();
            let (_, stderr) = super::run_cargo(
                &[
                    "build",
                    "--bin",
                    "a",
                    "--release",
                    "--locked",
                    "--offline",
                    "--no-default-features",
                    "--message-format=json",
                    "-v",
                    "-p",
                    "a",
                ],
                &env,
                &f.repo,
            )
            .expect("actual Cargo diagnostic-flag producer must compile");
            let scratch = format!("{}/", f.repo.display());
            let canonical = format!("{}/", f.repo.canonicalize().unwrap().display());
            let commands = cargo_commands(&stderr, &scratch, &canonical).unwrap();
            assert_eq!(
                commands.len(),
                2,
                "{level}: root and dependency must compile"
            );
            let mut compiled = std::collections::BTreeSet::new();
            for command in commands {
                let argv = split_command(command);
                assert_eq!(argv[0], "rustc");
                let name = argv
                    .windows(2)
                    .find(|pair| pair[0] == "--crate-name")
                    .unwrap();
                assert!(compiled.insert(name[1].clone()), "duplicate compiler unit");
                let actual: Vec<_> = argv
                    .windows(2)
                    .filter(|pair| pair[0] == "--check-cfg")
                    .map(|pair| pair[1].as_str())
                    .collect();
                assert_eq!(actual, cfg_operands, "{level}: actual producer operands");
                let actual_lints: std::collections::BTreeSet<_> = argv
                    .iter()
                    .filter(|arg| {
                        ["--allow=", "--warn=", "--deny=", "--forbid="]
                            .iter()
                            .any(|prefix| arg.starts_with(prefix))
                    })
                    .cloned()
                    .collect();
                let expected_lints = if level == "ordinary" {
                    std::collections::BTreeSet::new()
                } else {
                    ["unused_variables", "unsafe_code", "dead_code"]
                        .into_iter()
                        .map(|lint| format!("--{level}={lint}"))
                        .collect()
                };
                assert_eq!(
                    actual_lints, expected_lints,
                    "{level}: actual manifest lint flags"
                );
            }
            assert_eq!(
                compiled,
                ["a".to_owned(), "common".to_owned()].into_iter().collect()
            );

            let inventory = written(f.run());
            assert_eq!(inventory.get("programs").unwrap().elements().len(), 2);
            for (root, own, role, dependency) in [
                ("gen", "a", "generator", "--extern common=libcommon.rlib"),
                (
                    "chk",
                    "b",
                    "scheduling-checker",
                    "--extern common=libcommon.rmeta",
                ),
            ] {
                let p = program(&inventory, root);
                assert_eq!(p.get("role").and_then(Json::as_str), Some(role));
                let units = p.get("units").unwrap().elements();
                assert_eq!(units.len(), 2);
                let names: std::collections::BTreeSet<_> = units
                    .iter()
                    .map(|u| u.get("crate").unwrap().as_str().unwrap())
                    .collect();
                assert_eq!(names, [own, "common"].into_iter().collect());
                for unit in units {
                    let name = unit.get("crate").unwrap().as_str().unwrap();
                    let package = format!("crates/{name}");
                    assert_eq!(
                        unit.get("package").and_then(Json::as_str),
                        Some(package.as_str())
                    );
                    // Digests independently re-derived from literal fixture bytes with the system SHA-256 tool.
                    let (path, digest) = match name {
                        "a" => (
                            "crates/a/src/main.rs",
                            "354a2859cc377f4dc135ad8e3994aa7df352620a83186c45ce3427523c244c94",
                        ),
                        "b" => (
                            "crates/b/src/lib.rs",
                            "c8860bae75ba49f55cb4c79208938f9bf864bf0c804f3da3c4d918d899a75cb1",
                        ),
                        "common" => (
                            "crates/common/src/lib.rs",
                            "61d84a22ca31884e02beda52e98bac0f3388ac1015d3cbd1ec7eb90e3321e433",
                        ),
                        _ => unreachable!(),
                    };
                    let files = Json::Object(
                        [(path.to_owned(), Json::Str(digest.to_owned()))]
                            .into_iter()
                            .collect(),
                    );
                    assert_eq!(unit.get("files"), Some(&files));
                    assert_eq!(unit.get("env"), Some(&Json::Array(vec![])));
                    let configuration: Vec<_> = unit
                        .get("configuration")
                        .unwrap()
                        .elements()
                        .iter()
                        .map(|arg| arg.as_str().unwrap())
                        .collect();
                    assert!(configuration
                        .windows(2)
                        .any(|pair| pair == ["--crate-name", name]));
                    assert!(configuration.contains(&path));
                    if name == own {
                        assert!(configuration.contains(&dependency));
                    }
                    for operand in cfg_operands {
                        assert!(!configuration.contains(&operand), "{root}/{name}/{level} retained diagnostic operand {operand}: {configuration:?}");
                    }
                    assert!(!configuration
                        .iter()
                        .any(|arg| arg.starts_with("--check-cfg")
                            || ["--allow=", "--warn=", "--deny=", "--forbid="]
                                .iter()
                                .any(|prefix| arg.starts_with(prefix))));
                }
            }
            let actual = configurations(&inventory);
            if let Some(expected) = &ordinary {
                assert_eq!(&actual, expected, "{level}: lint-only configuration drift");
            } else {
                ordinary = Some(actual);
            }
        }
    }

    #[test]
    fn cargo_quoted_output_paths_preserve_inventory_units_and_configuration() {
        crate::trust_test_process::bounded!(
            cargo_quoted_output_paths_preserve_inventory_units_and_configuration
        );
        let f = two_roots(
            "cargo-quoted-output-paths",
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
        );
        let plain = written(f.run_into("plain"));
        let expected = configurations(&plain);
        assert_eq!(
            expected.len(),
            4,
            "each root must retain both compiled units"
        );
        for out in [
            "with space",
            "with'quote",
            "with space'quote",
            "with\\slash",
            "with\\slash'quote",
        ] {
            // Real Cargo builds and prints each invocation; no hand-written shell input stands in for it.
            let actual = written(f.run_into(out));
            assert_eq!(configurations(&actual), expected, "output path {out:?}");
            assert_eq!(items(&actual), items(&plain), "shared items for {out:?}");
        }
    }

    #[test]
    fn cargo_command_framing_preserves_quotes_and_refuses_malformed_records() {
        let command = "rustc --out-dir 'with\n     Running `fake`\r\\path'\\''tail' --cfg ''";
        let stderr = format!("warning: contains Running `ignored`\n     Running `{command}`\r\n     Running `rustc other.rs`\n    Finished\n");
        assert_eq!(
            cargo_commands(&stderr, "/scratch/", "/canonical/").unwrap(),
            [command, "rustc other.rs"]
        );
        assert_eq!(
            split_command(command),
            [
                "rustc",
                "--out-dir",
                "with\n     Running `fake`\r\\path'tail",
                "--cfg",
                ""
            ]
        );
        assert!(
            cargo_commands("warning: Running `unclosed\n", "/scratch/", "/canonical/")
                .unwrap()
                .is_empty()
        );
        let raw_prefix = "/written/with\n     Running `rustc --crate-name fake`\ntrailer/tree/";
        let status = format!(
            "   Compiling a v0.1.0 ({raw_prefix}crates/a)\n     Running `rustc actual.rs`\n"
        );
        for (scratch, canonical) in [(raw_prefix, "/other/"), ("/other/", raw_prefix)] {
            assert_eq!(
                cargo_commands(&status, scratch, canonical).unwrap(),
                ["rustc actual.rs"]
            );
        }
        for malformed in [
            "     Running `rustc",
            "     Running `rustc 'unclosed`\n",
            "     Running `rustc\nother`\n",
            "     Running `rustc\rother`\n",
            "     Running `rustc` junk\n",
            "     Running ``\n",
            "     Running `rustc \\`\n",
        ] {
            assert!(
                cargo_commands(malformed, "/scratch/", "/canonical/").is_err(),
                "malformed record {malformed:?}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn cargo_multiline_output_paths_preserve_complete_inventory() {
        crate::trust_test_process::bounded!(
            cargo_multiline_output_paths_preserve_complete_inventory
        );
        let data = "independently expected compiler input\n";
        let include = "const DATA: &str = include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/data.txt\"));";
        let main = format!("{include}\nfn main() {{ common::c(); assert!(!DATA.is_empty()); }}\n");
        let library = "pub fn f() { common::c(); }\n";
        let common = "pub fn c() {}\n";
        let f = two_roots(
            "cargo-multiline-output-paths",
            &main,
            library,
            &[
                ("crates/a/src/data.txt", data.to_owned()),
                ("crates/common/Cargo.toml", manifest("common", "")),
                ("crates/common/src/lib.rs", common.to_owned()),
                (
                    "crates/a/Cargo.toml",
                    manifest("a", "[dependencies]\ncommon = { path = \"../common\" }\n"),
                ),
                (
                    "crates/b/Cargo.toml",
                    manifest("b", "[dependencies]\ncommon = { path = \"../common\" }\n"),
                ),
                (
                    "trust/roots.eadl",
                    format!(
                        "{ROOTS}{}",
                        admit("crates/a/src/main.rs", "include_str", include)
                    ),
                ),
            ],
        );
        let plain = written(f.run_into("ordinary"));
        for out in [
            "ordinary",
            "with\nnewline",
            "with\rreturn",
            "with`tick",
            "with\n     Running `rustc --crate-name fake`\ntrailer",
            "with'\n\\`quote\rend",
        ] {
            // These are actual public builds: every unit, file and retained dependency is independently required.
            let actual = if out == "ordinary" {
                plain.clone()
            } else {
                written(f.run_into(out))
            };
            assert_eq!(actual.get("programs").unwrap().elements().len(), 2);
            for (root, own, dependency) in [
                ("gen", "crates/a", "--extern common=libcommon.rlib"),
                ("chk", "crates/b", "--extern common=libcommon.rmeta"),
            ] {
                let units = program(&actual, root).get("units").unwrap().elements();
                assert_eq!(units.len(), 2, "{root}: {out:?}");
                let packages: std::collections::BTreeSet<_> = units
                    .iter()
                    .map(|u| u.get("package").unwrap().as_str().unwrap())
                    .collect();
                assert_eq!(
                    packages,
                    std::collections::BTreeSet::from([own, "crates/common"])
                );
                for unit in units {
                    let package = unit.get("package").unwrap().as_str().unwrap();
                    let (name, inputs, environment) = match package {
                        "crates/a" => (
                            "a",
                            vec![
                                ("crates/a/src/main.rs", main.as_str()),
                                ("crates/a/src/data.txt", data),
                            ],
                            vec!["CARGO_MANIFEST_DIR=crates/a"],
                        ),
                        "crates/b" => ("b", vec![("crates/b/src/lib.rs", library)], vec![]),
                        "crates/common" => {
                            ("common", vec![("crates/common/src/lib.rs", common)], vec![])
                        }
                        _ => panic!("unexpected package {package}"),
                    };
                    assert_eq!(unit.get("crate").unwrap().as_str(), Some(name));
                    let files = Json::Object(
                        inputs
                            .into_iter()
                            .map(|(path, content)| {
                                (
                                    path.to_owned(),
                                    Json::Str(Digest::of(content.as_bytes()).hex()),
                                )
                            })
                            .collect(),
                    );
                    assert_eq!(unit.get("files"), Some(&files), "{root}/{package}: {out:?}");
                    let env = Json::Array(
                        environment
                            .into_iter()
                            .map(|e| Json::Str(e.to_owned()))
                            .collect(),
                    );
                    assert_eq!(unit.get("env"), Some(&env), "{root}/{package}: {out:?}");
                    if package == own {
                        assert!(
                            unit.get("configuration")
                                .unwrap()
                                .elements()
                                .iter()
                                .any(|arg| arg.as_str() == Some(dependency)),
                            "{root} lost {dependency}: {out:?}"
                        );
                    }
                }
            }
            assert_eq!(configurations(&actual), configurations(&plain), "{out:?}");
            assert_eq!(items(&actual), items(&plain), "{out:?}");
        }
    }

    #[test]
    fn a_root_s_configuration_retains_its_dependency_argument() {
        crate::trust_test_process::bounded!(a_root_s_configuration_retains_its_dependency_argument);
        let f = two_roots(
            "retained-dependency-argument",
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
        );
        let inventory = written(f.run());
        for (root, package, expected) in [
            ("gen", "crates/a", "--extern common=libcommon.rlib"),
            ("chk", "crates/b", "--extern common=libcommon.rmeta"),
        ] {
            let unit = program(&inventory, root)
                .get("units")
                .unwrap()
                .elements()
                .iter()
                .find(|u| u.get("package").and_then(Json::as_str) == Some(package))
                .expect("the root's own unit is inventoried");
            let arguments: Vec<_> = unit
                .get("configuration")
                .unwrap()
                .elements()
                .iter()
                .map(|a| a.as_str().expect("a configuration argument is text"))
                .collect();
            assert!(
                arguments.contains(&expected),
                "{root} lost its dependency argument {expected:?}: {arguments:?}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn cargo_dependency_environment_normalizes_escaped_scratch_paths() {
        crate::trust_test_process::bounded!(
            cargo_dependency_environment_normalizes_escaped_scratch_paths
        );
        use std::fs;
        let description = "real description\nsecond line\rreturn\\literal\\ntext";
        let source = "fn main() { let _ = include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/data.txt\")); let _ = env!(\"CARGO_PKG_DESCRIPTION\"); let _ = option_env!(\"CARGO_PKG_NO_SUCH_VARIABLE\"); }\n";
        for (index, case) in [
            "ordinary",
            "with\nnewline",
            "with\rreturn",
            "with\\slash",
            "with\\ntext",
            "with\n# env-dep:CARGO_PKG_DESCRIPTION=forged\nsuffix",
        ]
        .into_iter()
        .enumerate()
        {
            let f = two_roots(
                &format!("cargo-dependency-environment/{case}"),
                source,
                "pub fn f() {}\n",
                &[
                    ("crates/a/src/data.txt", "known input\n".to_owned()),
                    (
                        "crates/a/Cargo.toml",
                        manifest(
                            "a",
                            &format!(
                                "description = {}\n",
                                crate::json::write(&Json::Str(description.to_owned()))
                            ),
                        ),
                    ),
                ],
            );
            // Keep the target separate: a marker's colon cannot enter Cargo's library-search path on Unix.
            let target = real_root()
                .join("target/trust-tests/cargo-dependency-environment-targets")
                .join(index.to_string());
            let _ = fs::remove_dir_all(&target);
            let env = super::environment("1.95.0", &f.base.join("cargo-home"), &target).unwrap();
            super::run_cargo(
                &[
                    "build",
                    "--release",
                    "--locked",
                    "--offline",
                    "--no-default-features",
                    "--message-format=json",
                    "-v",
                    "-p",
                    "a",
                ],
                &env,
                &f.repo,
            )
            .expect("the actual producer must compile successfully");
            let files: Vec<_> = fs::read_dir(target.join("release/deps"))
                .unwrap()
                .map(|e| e.unwrap().path())
                .filter(|p| p.extension().is_some_and(|e| e == "d"))
                .collect();
            assert_eq!(files.len(), 1, "only the fixture binary is compiled");
            let text = fs::read_to_string(&files[0]).unwrap();
            let scratch = format!("{}/", f.repo.display());
            let canonical = format!("{}/", fs::canonicalize(&f.repo).unwrap().display());
            let records: Vec<_> = text
                .lines()
                .filter_map(|line| line.strip_prefix("# env-dep:"))
                .map(|dep| super::normalise_dependency_environment(dep, &scratch, &canonical))
                .collect();
            assert_eq!(
                records,
                [
                    "CARGO_MANIFEST_DIR=crates/a",
                    r"CARGO_PKG_DESCRIPTION=real description\nsecond line\rreturn\\literal\\ntext",
                    "CARGO_PKG_NO_SUCH_VARIABLE",
                ],
                "{case:?}: normalize paths, preserve encoded values and unset records"
            );
        }
    }

    #[test]
    fn dependency_environment_uses_the_canonical_encoded_prefix() {
        assert_eq!(
            super::normalise_dependency_environment(
                r"CARGO_MANIFEST_DIR=/canonical\\tree/crates/a",
                "/lexical\\alias/",
                "/canonical\\tree/",
            ),
            "CARGO_MANIFEST_DIR=crates/a"
        );
    }

    #[cfg(unix)]
    #[test]
    fn cargo_dependency_information_retains_actual_source_paths() {
        crate::trust_test_process::bounded!(
            cargo_dependency_information_retains_actual_source_paths
        );
        use std::{collections::BTreeSet, fs};
        for case in [
            "ordinary",
            "with\nnewline",
            "with\rreturn",
            "with space",
            "with`tick",
            "with\tspecial",
            "with\\ space",
            "with#hash",
        ] {
            let f = two_roots(
                &format!("cargo-dependency-information/{case}"),
                "fn main() { let _ = include_str!(concat!(env!(\"CARGO_MANIFEST_DIR\"), \"/src/data.txt\")); }\n",
                "pub fn f() {}\n",
                &[("crates/a/src/data.txt", "independently expected input\n".to_owned())],
            );
            let target = f.base.join("cargo-target");
            let env = super::environment("1.95.0", &f.base.join("cargo-home"), &target).unwrap();
            super::run_cargo(
                &[
                    "build",
                    "--release",
                    "--locked",
                    "--offline",
                    "--no-default-features",
                    "--message-format=json",
                    "-v",
                    "-p",
                    "a",
                ],
                &env,
                &f.repo,
            )
            .expect("the real producer must compile the fixture");
            let dep_files: Vec<_> = fs::read_dir(target.join("release/deps"))
                .unwrap()
                .map(|e| e.unwrap().path())
                .filter(|p| p.extension().is_some_and(|e| e == "d"))
                .collect();
            assert_eq!(dep_files.len(), 1, "only the actual binary is compiled");
            let d = &dep_files[0];
            let text = fs::read_to_string(d).unwrap();
            let scratch = format!("{}/", f.repo.display());
            let canonical = format!("{}/", fs::canonicalize(&f.repo).unwrap().display());
            let files: BTreeSet<_> =
                read(super::dependency_files(&text, d, &scratch, &canonical).unwrap())
                    .into_iter()
                    .collect();
            let expected = BTreeSet::from([
                "crates/a/src/main.rs".to_owned(),
                format!("{}crates/a/src/data.txt", scratch),
            ]);
            assert_eq!(
                files, expected,
                "{case:?}: preserve every real relative/absolute prerequisite"
            );
        }
    }

    /// The prerequisites a record names, or a panic where its text names two lists.
    fn read(prerequisites: super::Prerequisites) -> Vec<String> {
        match prerequisites {
            super::Prerequisites::Read(files) => files,
            super::Prerequisites::Ambiguous { raw, why } => panic!("ambiguous `{raw}`: {why}"),
        }
    }

    /// The raw rule and the reason a record is refused, or a panic where it was read.
    fn ambiguous(prerequisites: super::Prerequisites) -> (String, &'static str) {
        match prerequisites {
            super::Prerequisites::Read(files) => panic!("read {files:?} where a refusal was due"),
            super::Prerequisites::Ambiguous { raw, why } => (raw, why),
        }
    }

    #[test]
    fn dependency_information_requires_its_known_nonempty_target() {
        let file = std::path::Path::new("target/with: space.d");
        let text = "target/with: space.d: crates/a/src/main.rs\n\ncrates/a/src/main.rs:\n";
        assert_eq!(
            read(super::dependency_files(text, file, "tree/", "canonical/").unwrap()),
            ["crates/a/src/main.rs"]
        );
        for malformed in [
            "unrelated.d: crates/a/src/main.rs\n",
            "target/with: space.d: \n",
            "",
            "target/with: space.d",
        ] {
            assert!(
                super::dependency_files(malformed, file, "tree/", "canonical/").is_err(),
                "malformed dependency target: {malformed:?}"
            );
        }
    }

    /// One first-rule text can spell two prerequisite lists: the per-file targets decide, and a rule they do not
    /// re-spell, a backslash outside the known prefixes, or a line after them that is no environment record, is
    /// refused naming the raw rule (`M3.6.9.6.3.2.2.15.2.1.6.1.1.1`).
    #[test]
    fn a_configuration_naming_an_outside_path_is_refused_at_every_position() {
        // No producer route reaches this refusal (`M3.6.9.6.3.2.2.15.2.1.7.1`), so it is held here directly: each form
        // first, in the middle and last of an ordinary configuration, two at once in order, and none; the
        // configuration itself comes back unchanged.
        let ordinary = [
            "--crate-name",
            "b",
            "--edition=2021",
            "crates/b/src/lib.rs",
            "-C opt-level=3",
        ];
        let refusal = |t: &str| {
            format!(
                "trust-undeclared-input: `b` in `chk` is configured with `{t}`, a path outside the written tree"
            )
        };
        let with = |inserted: &[(usize, &str)]| {
            let mut tokens: Vec<String> = ordinary.iter().map(|t| (*t).to_owned()).collect();
            for (at, t) in inserted {
                tokens.insert(*at, (*t).to_owned());
            }
            let mut refused = Vec::new();
            let configuration =
                super::checked_configuration("b", "chk", tokens.clone(), &mut refused);
            assert_eq!(
                configuration, tokens,
                "the configuration is returned as it is"
            );
            refused
        };
        assert_eq!(with(&[]), Vec::<String>::new(), "an ordinary configuration");
        for outside in ["/outside/lib", "-C link-arg=/outside/lib.o"] {
            for at in [0, 3, ordinary.len()] {
                assert_eq!(
                    with(&[(at, outside)]),
                    [refusal(outside)],
                    "`{outside}` at {at}"
                );
            }
        }
        assert_eq!(
            with(&[(1, "/outside/lib"), (4, "-C link-arg=/outside/lib.o")]),
            [
                refusal("/outside/lib"),
                refusal("-C link-arg=/outside/lib.o")
            ],
            "two at once, in order"
        );
    }

    #[test]
    fn an_ambiguous_dependency_record_is_refused_by_its_raw_rule() {
        let file = std::path::Path::new("out/b.d");
        let scratch = "/run/prefix crates/b/src/tree/";
        let record = |rule: &str, rest: &str| {
            super::dependency_files(
                &format!("out/b.d: {rule}{rest}"),
                file,
                scratch,
                "/canonical/",
            )
        };
        let collision = "crates/b/src/lib.rs /run/prefix\\ crates/b/src/tree/data.txt";
        let targets = "its per-file targets spell the prerequisites otherwise";
        let backslash = "a backslash outside the written tree's prefixes";
        // The targets spell the outside name and the in-tree prerequisite apart: refused, never the decoy.
        assert_eq!(
            ambiguous(
                record(
                    collision,
                    "\n\ncrates/b/src/lib.rs:\n/run/prefix\\:\ncrates/b/src/tree/data.txt:\n"
                )
                .unwrap()
            ),
            (collision.to_owned(), targets)
        );
        // The same rule whose targets spell one in-tree path is read as that path.
        assert_eq!(
            read(
                record(
                    collision,
                    "\n\ncrates/b/src/lib.rs:\n/run/prefix\\ crates/b/src/tree/data.txt:\n"
                )
                .unwrap()
            ),
            [
                "crates/b/src/lib.rs",
                "/run/prefix crates/b/src/tree/data.txt"
            ]
        );
        for (rule, why) in [
            ("a /ext/x\\ y.txt", backslash),
            ("a /ext/x\\y.txt", backslash),
            ("a /run/prefix\\ crates/b/src/tree/x\\y.txt", backslash),
        ] {
            let spelled: String = rule.split(' ').map(|t| format!("{t}:\n")).collect();
            assert_eq!(
                ambiguous(record(rule, &format!("\n\n{spelled}")).unwrap()),
                (rule.to_owned(), why),
                "{rule}"
            );
        }
        for rest in [
            "\n\nout/libb.rlib: a b\n\na:\nb:\n\n# env-dep:X=1\n# env-dep:Y\n",
            "\n\nout/libb.rlib: a b\n\nout/libb.rmeta: a b\n\na:\nb:\n",
        ] {
            assert_eq!(read(record("a b", rest).unwrap()), ["a", "b"], "{rest:?}");
        }
        for rest in [
            "\n\na:\n",
            "\n\na:\nb:\nc:\n",
            "\na:\nb:\n",
            "\n\nout/libb.rlib: a c\n\na:\nb:\n",
            "\n\nout/libb.rlib: a b\n\nb:\na:\n",
            "\n\na:\nb:\n\n# env-dep X # env-dep:Y\n",
            "\n\na:\nb:\n# env-dep:X=1\n",
            "\n\na:\nb:\nextra\n",
            "",
        ] {
            assert_eq!(
                ambiguous(record("a b", rest).unwrap()),
                ("a b".to_owned(), targets),
                "{rest:?}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn cargo_verbose_empty_arguments_keep_their_token_boundaries() {
        crate::trust_test_process::bounded!(
            cargo_verbose_empty_arguments_keep_their_token_boundaries
        );
        let f = two_roots(
            "cargo-empty-lint-arguments",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[],
        );
        for (case, lint) in [("nonempty", "dead_code"), ("empty", "")] {
            let env = super::environment(
                "1.95.0",
                &f.base.join(case).join("cargo-home"),
                &f.base.join(case).join("target"),
            )
            .unwrap();
            // These explicit diagnostic flags exercise the helper's Cargo-format promise. Inventory builds
            // do not admit caller-supplied rustc flags. Empty lint names warn, but this real compilation succeeds.
            let (_, stderr) = super::run_cargo(
                &[
                    "rustc",
                    "--release",
                    "--locked",
                    "--offline",
                    "--no-default-features",
                    "--message-format=json",
                    "-v",
                    "-p",
                    "a",
                    "--",
                    "-A",
                    lint,
                    "-W",
                    lint,
                ],
                &env,
                &f.repo,
            )
            .expect("the actual producer must compile successfully");
            let commands: Vec<_> = stderr
                .lines()
                .filter_map(|line| line.trim().strip_prefix("Running `")?.strip_suffix('`'))
                .collect();
            assert_eq!(
                commands.len(),
                1,
                "exactly the fixture's binary is compiled"
            );
            let tokens = split_command(commands[0]);
            assert_eq!(tokens.first().map(String::as_str), Some("rustc"));
            let expected = ["-A", lint, "-W", lint].map(str::to_owned);
            assert!(
                tokens.ends_with(&expected),
                "{case}: actual verbose arguments lost boundaries: {tokens:?}"
            );
        }
    }

    #[cfg(unix)]
    #[test]
    fn compiler_installation_paths_preserve_inventory_units_and_configuration() {
        use std::collections::BTreeSet;
        use std::fs;

        crate::trust_test_process::bounded!(
            compiler_installation_paths_preserve_inventory_units_and_configuration
        );
        const CASE: &str = "ARCHOGEN_COMPILER_PATH_CASE";
        const NAME: &str =
            "trust::tests::compiler_installation_paths_preserve_inventory_units_and_configuration";
        if std::env::var_os(CASE).is_some() {
            let f = two_roots(
                "compiler-installation-paths",
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
            );
            let inventory = written(f.run());
            for (root, package, dependency) in [
                ("gen", "crates/a", "--extern common=libcommon.rlib"),
                ("chk", "crates/b", "--extern common=libcommon.rmeta"),
            ] {
                let units = program(&inventory, root).get("units").unwrap().elements();
                let packages: BTreeSet<_> = units
                    .iter()
                    .map(|u| u.get("package").and_then(Json::as_str).unwrap())
                    .collect();
                assert_eq!(units.len(), 2, "each root must retain exactly both units");
                assert_eq!(packages, BTreeSet::from([package, "crates/common"]));
                let own = units
                    .iter()
                    .find(|u| u.get("package").and_then(Json::as_str) == Some(package))
                    .unwrap();
                assert!(
                    list(own, "configuration").contains(&dependency.to_owned()),
                    "{root} lost its independently expected dependency argument"
                );
            }
            return;
        }

        // Only the installed OS/toolchain dependency is read through these local links. No other repository
        // is written, and each environment belongs to a child, never to the process running parallel tests.
        let output = Command::new("rustup")
            .args(["which", "--toolchain", "1.95.0", "rustc"])
            .output()
            .unwrap();
        assert!(
            output.status.success(),
            "rustup could not locate the pinned compiler"
        );
        let compiler = PathBuf::from(String::from_utf8(output.stdout).unwrap().trim());
        let toolchain = compiler
            .parent()
            .unwrap()
            .parent()
            .unwrap()
            .canonicalize()
            .unwrap();
        let homes = real_root().join("target/trust-tests/compiler-installation-homes");
        let _ = fs::remove_dir_all(&homes);
        for case in [
            "ordinary",
            "with space",
            "with'quote",
            "with\\slash",
            "with space'\\quote",
        ] {
            let home = homes.join(case);
            let toolchains = home.join("toolchains");
            fs::create_dir_all(&toolchains).unwrap();
            std::os::unix::fs::symlink(&toolchain, toolchains.join(toolchain.file_name().unwrap()))
                .unwrap();
            let result = Command::new(std::env::current_exe().unwrap())
                .args(["--exact", NAME, "--format=pretty", "--color=never"])
                .env(CASE, case)
                .env("RUSTUP_HOME", home)
                .output()
                .unwrap();
            assert!(
                result.status.success(),
                "compiler home {case:?}: stdout {}; stderr {}",
                String::from_utf8_lossy(&result.stdout),
                String::from_utf8_lossy(&result.stderr)
            );
            let stdout = String::from_utf8(result.stdout).unwrap();
            let named = format!("test {NAME} ... ok");
            assert!(stdout.lines().any(|l| l == named));
            assert!(stdout.contains("test result: ok. 1 passed; 0 failed; 0 ignored;"));
        }
    }

    const CARGO_ROOT_TYPE_ERRORS: &str = concat!(
        "`cargo build --bin a --release --locked --offline --no-default-features ",
        "--message-format=json -v -p a` failed: error: could not compile `a` (bin \"a\") ",
        "due to 1 previous error\n",
        "`cargo build --lib --release --locked --offline --no-default-features ",
        "--message-format=json -v -p b` failed: error: could not compile `b` (lib) ",
        "due to 1 previous error"
    );

    fn assert_cargo_type_failure(args: &[&str], env: &[(String, String)], cwd: &Path) {
        let output = Command::new("cargo")
            .args(args)
            .current_dir(cwd)
            .env_clear()
            .envs(env.iter().map(|(key, value)| (key, value)))
            .env("RUSTC", "rustc")
            .output()
            .unwrap();
        assert!(!output.status.success());
        let stdout = String::from_utf8(output.stdout).unwrap();
        assert!(
            stdout.lines().any(|line| {
                let message = crate::json::parse(line).unwrap();
                message.get("reason").and_then(Json::as_str) == Some("compiler-message")
                    && message
                        .get("message")
                        .and_then(|message| message.get("code"))
                        .and_then(|code| code.get("code"))
                        .and_then(Json::as_str)
                        == Some("E0308")
            }),
            "{stdout}"
        );
        let diagnostic = "error: could not compile `a` (bin \"a\") due to 1 previous error";
        let stderr = String::from_utf8(output.stderr).unwrap();
        assert!(stderr.lines().any(|line| line == diagnostic), "{stderr}");
        assert_eq!(
            super::run_cargo(args, env, cwd).unwrap_err(),
            format!("`cargo {}` failed: {diagnostic}", args.join(" "))
        );
    }

    #[test]
    fn cargo_failure_diagnostics_keep_an_independent_target_alias() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_keep_an_independent_target_alias
        );
        let main = "fn main() {}\n";
        let f = two_roots(
            "cargo-independent-target-alias",
            main,
            "pub fn f() {}\n",
            &[],
        );
        let target = f.base.join("physical-target");
        let alias = f.base.join("artifact\nerror-target-alias");
        std::fs::create_dir_all(&target).unwrap();
        std::os::unix::fs::symlink(&target, &alias).unwrap();
        assert_eq!(
            std::fs::canonicalize(&alias).unwrap(),
            std::fs::canonicalize(&target).unwrap()
        );
        let env = super::environment("1.95.0", &f.base.join("cargo-home"), &alias).unwrap();
        let args = [
            "build",
            "--bin",
            "a",
            "--release",
            "--locked",
            "--offline",
            "--no-default-features",
            "--message-format=json",
            "-v",
            "-p",
            "a",
        ];
        super::run_cargo(&args, &env, &f.repo).unwrap();
        f.commit(&[(
            "crates/a/src/main.rs",
            "fn main() { let _: u8 = \"wrong type\"; }\n".to_owned(),
        )]);
        assert_cargo_type_failure(&args, &env, &f.repo);
        f.commit(&[("crates/a/src/main.rs", main.to_owned())]);
        super::run_cargo(&args, &env, &f.repo).unwrap();
    }

    #[test]
    fn cargo_failure_diagnostics_keep_cwd_beside_a_separate_target() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_keep_cwd_beside_a_separate_target
        );
        let main = "fn main() {}\n";
        let legacy = std::fs::read_to_string(real_root().join(".cargo/config.toml")).unwrap();
        let mut f = two_roots(
            "cargo-independent-cwd",
            main,
            "pub fn f() {}\n",
            &[(".cargo/config", legacy)],
        );
        let cwd = f.base.join("workspace\nerror-cwd");
        std::fs::rename(&f.repo, &cwd).unwrap();
        f.repo = cwd;
        let env = super::environment(
            "1.95.0",
            &f.base.join("cargo-home"),
            &f.base.join("separate-target"),
        )
        .unwrap();
        let args = [
            "build",
            "--bin",
            "a",
            "--release",
            "--locked",
            "--offline",
            "--no-default-features",
            "--message-format=json",
            "-v",
            "-p",
            "a",
        ];
        let (_, stderr) = super::run_cargo(&args, &env, &f.repo).unwrap();
        assert!(stderr.contains("warning: both `"), "{stderr}");
        assert!(stderr.contains("error-cwd/.cargo/config"), "{stderr}");
        f.commit(&[(
            "crates/a/src/main.rs",
            "fn main() { let _: u8 = \"wrong type\"; }\n".to_owned(),
        )]);
        assert_cargo_type_failure(&args, &env, &f.repo);
        f.commit(&[("crates/a/src/main.rs", main.to_owned())]);
        super::run_cargo(&args, &env, &f.repo).unwrap();
    }

    #[test]
    fn cargo_failure_diagnostics_keep_an_error_before_a_multiline_path() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_keep_an_error_before_a_multiline_path
        );
        let mut f = two_roots(
            "cargo-leading-manifest-error",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[],
        );
        let cwd = f.base.join("workspace\nerror-manifest-path");
        std::fs::rename(&f.repo, &cwd).unwrap();
        f.repo = cwd;
        let env = super::environment(
            "1.95.0",
            &f.base.join("cargo-home"),
            &f.base.join("separate-target"),
        )
        .unwrap();
        let args = [
            "build",
            "--bin",
            "a",
            "--release",
            "--locked",
            "--offline",
            "--no-default-features",
            "--message-format=json",
            "-v",
            "-p",
            "a",
        ];
        super::run_cargo(&args, &env, &f.repo).unwrap();
        let manifest = f.repo.join("Cargo.toml");
        let held = std::fs::read(&manifest).unwrap();
        std::fs::write(
            &manifest,
            "[package]\nname = \"a\"\nversion = \"0.1.0\"\n[workspace]\n",
        )
        .unwrap();
        let error = super::run_cargo(&args, &env, &f.repo).unwrap_err();
        let leading = format!("error: failed to parse manifest at `{}", f.repo.display());
        let expected = leading.split('\n').next().unwrap();
        assert_eq!(
            error,
            format!("`cargo {}` failed: {expected}", args.join(" "))
        );
        std::fs::write(&manifest, held).unwrap();
        super::run_cargo(&args, &env, &f.repo).unwrap();
    }

    #[test]
    fn cargo_failure_diagnostics_keep_the_error_after_a_quoted_key_warning() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_keep_the_error_after_a_quoted_key_warning
        );
        let main = "fn main() {}\n";
        let f = two_roots("cargo-quoted-key-warning", main, "pub fn f() {}\n", &[]);
        written(f.run_into("ordinary"));
        let quoted = manifest("a", "\"unused\\nerroneous-warning\" = true\n");
        let outside = archogen_catalog::manifest::parse(&quoted).unwrap_err();
        assert_eq!(outside.why, "a key is bare keys joined by `.`");
        f.commit(&[("crates/a/Cargo.toml", quoted)]);
        // Cargo accepts the key, while the inventory's dialect refuses it before builds.
        says(
            &refused(f.run_into("syntax-refused")),
            "a key is bare keys joined by `.`",
        );
        let refused_out = f.base.join("syntax-refused");
        assert!(!refused_out.join("trust-dependencies.json").exists());
        assert!(!refused_out.join("target").exists());
        let env = super::environment(
            "1.95.0",
            &f.base.join("producer/cargo-home"),
            &f.base.join("producer/target"),
        )
        .unwrap();
        let args = [
            "build",
            "--bin",
            "a",
            "--release",
            "--locked",
            "--offline",
            "--no-default-features",
            "--message-format=json",
            "-v",
            "-p",
            "a",
        ];
        let (_, stderr) = super::run_cargo(&args, &env, &f.repo).unwrap();
        assert!(
            stderr.contains("unused manifest key: package.unused\nerroneous-warning"),
            "{stderr}"
        );
        f.commit(&[(
            "crates/a/src/main.rs",
            "fn main() { let _: u8 = \"wrong type\"; }\n".to_owned(),
        )]);
        assert_cargo_type_failure(&args, &env, &f.repo);
        f.commit(&[
            ("crates/a/src/main.rs", main.to_owned()),
            ("crates/a/Cargo.toml", manifest("a", "")),
        ]);
        super::run_cargo(&args, &env, &f.repo).unwrap();
        written(f.run_into("recovered"));
    }

    #[test]
    fn cargo_failure_diagnostics_keep_the_error_at_a_config_warning_boundary() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_keep_the_error_at_a_config_warning_boundary
        );
        let legacy = std::fs::read_to_string(real_root().join(".cargo/config.toml")).unwrap();
        let f = two_roots(
            "cargo-config-warning-boundary",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[(".cargo/config", legacy)],
        );
        written(f.run_into("ordinary"));
        // This distinct directory's spelling matches the containing legacy
        // configuration path, Cargo's closing backtick and its record newline.
        let boundary = f.repo.join(".cargo/config`\n");
        let destination = boundary.join("with:colon");
        let mut roots = configuration_roots();
        roots.push(f.repo.clone());
        let error = match inventory_with(&f.repo, "HEAD", &destination, &roots) {
            Err(error) => error,
            Ok(Outcome::Written(_)) => panic!("colon artifact path unexpectedly built"),
            Ok(Outcome::Refused(why)) => panic!("ordinary input unexpectedly refused: {why:?}"),
        };
        let failures: Vec<_> = error.lines().collect();
        assert_eq!(failures.len(), 2, "{error}");
        for failure in failures {
            assert!(
                failure.contains("` failed: error: failed to join paths from `$"),
                "{failure}"
            );
            assert!(failure.ends_with("` together"), "{failure}");
        }
        assert!(!destination.join("trust-dependencies.json").exists());
        assert!(f.repo.join(".cargo/config").is_file());
        assert!(boundary.is_dir());
        assert_ne!(
            std::fs::canonicalize(f.repo.join(".cargo/config")).unwrap(),
            std::fs::canonicalize(&boundary).unwrap()
        );
        written(f.run_into("recovered"));
    }

    fn check_cargo_failure_diagnostic_output_paths(fixture: &str, outputs: &[&str]) {
        let main = "fn main() {}\n";
        let library = "pub fn f() {}\n";
        let legacy = std::fs::read_to_string(real_root().join(".cargo/config.toml")).unwrap();
        let f = two_roots(fixture, main, library, &[(".cargo/config", legacy)]);
        for out in outputs {
            assert_eq!(
                written(f.run_into(out))
                    .get("programs")
                    .unwrap()
                    .elements()
                    .len(),
                2,
                "{out:?}"
            );
        }
        f.commit(&[
            (
                "crates/a/src/main.rs",
                "fn main() { let _: u8 = \"wrong type\"; }\n".to_owned(),
            ),
            (
                "crates/b/src/lib.rs",
                "pub fn f() { let _: u8 = \"wrong type\"; }\n".to_owned(),
            ),
        ]);

        for out in outputs {
            let destination = f.base.join(out);
            let error = match inventory_with(&f.repo, "HEAD", &destination, &configuration_roots())
            {
                Err(error) => error,
                Ok(Outcome::Written(_)) => panic!("failed builds wrote an inventory: {out:?}"),
                Ok(Outcome::Refused(why)) => {
                    panic!("build failure became a source refusal: {why:?}")
                }
            };
            assert_eq!(error, CARGO_ROOT_TYPE_ERRORS, "{out:?}");
            assert!(!destination.join("trust-dependencies.json").exists());
        }
        f.commit(&[
            ("crates/a/src/main.rs", main.to_owned()),
            ("crates/b/src/lib.rs", library.to_owned()),
        ]);
        for out in outputs {
            assert_eq!(
                written(f.run_into(out))
                    .get("programs")
                    .unwrap()
                    .elements()
                    .len(),
                2,
                "recovery: {out:?}"
            );
        }
    }

    #[test]
    fn cargo_failure_diagnostics_ignore_multiline_output_paths() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_ignore_multiline_output_paths
        );
        check_cargo_failure_diagnostic_output_paths(
            "cargo-failure-diagnostic-paths",
            &["ordinary", "with\nerroneous-path", "with\nerror-path"],
        );
    }

    #[test]
    fn cargo_failure_diagnostics_ignore_multiline_output_paths_messages() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_ignore_multiline_output_paths_messages
        );
        check_cargo_failure_diagnostic_output_paths(
            "cargo-failure-diagnostic-message-paths",
            &[
                "with\nerror decoy",
                "with\nerror could not compile `decoy`",
                "with\r\nerror carriage-return decoy",
            ],
        );
    }

    #[test]
    fn cargo_failure_diagnostics_ignore_multiline_output_paths_quoted_and_unicode() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_ignore_multiline_output_paths_quoted_and_unicode
        );
        check_cargo_failure_diagnostic_output_paths(
            "cargo-failure-diagnostic-quoted-paths",
            &["with'\\\nerror quoted path", "with-λ\nerror unicode path"],
        );
    }

    #[test]
    fn cargo_failure_diagnostics_ignore_multiline_target_paths() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_ignore_multiline_target_paths
        );
        let main = "fn main() {}\n";
        let f = two_roots(
            "cargo-failure-diagnostic-targets",
            main,
            "pub fn f() {}\n",
            &[],
        );
        let targets = [
            "ordinary",
            "with\nerror-path",
            "with\nerror decoy",
            "with\nerror could not compile `decoy`",
            "with'\\\nerror quoted path",
            "with-λ\nerror unicode path",
        ];
        let args = [
            "build",
            "--bin",
            "a",
            "--release",
            "--locked",
            "--offline",
            "--no-default-features",
            "--message-format=json",
            "-v",
            "-p",
            "a",
        ];
        for target in targets {
            let env = super::environment(
                "1.95.0",
                &f.base.join("producer/cargo-home"),
                &f.base.join("producer").join(target),
            )
            .unwrap();
            super::run_cargo(&args, &env, &f.repo).unwrap();
        }
        f.commit(&[(
            "crates/a/src/main.rs",
            "fn main() { let _: u8 = \"wrong type\"; }\n".to_owned(),
        )]);
        let expected = concat!(
            "`cargo build --bin a --release --locked --offline --no-default-features ",
            "--message-format=json -v -p a` failed: error: could not compile `a` (bin \"a\") ",
            "due to 1 previous error"
        );
        for target in targets {
            let env = super::environment(
                "1.95.0",
                &f.base.join("producer/cargo-home"),
                &f.base.join("producer").join(target),
            )
            .unwrap();
            assert_eq!(
                super::run_cargo(&args, &env, &f.repo).unwrap_err(),
                expected,
                "{target:?}"
            );
        }
    }

    #[test]
    fn cargo_failure_diagnostics_preserve_operational_refusals() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_preserve_operational_refusals
        );
        let legacy = std::fs::read_to_string(real_root().join(".cargo/config.toml")).unwrap();
        let f = two_roots(
            "cargo-failure-diagnostic-operational",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[(".cargo/config", legacy)],
        );
        written(f.run_into("ordinary"));
        // Cargo cannot join a Unix library search path containing a colon. The source is valid;
        // this is an operational failure control, not successful compiler evidence for this layout.
        for out in [
            "with\nerror: decoy",
            "with\nerror: could not compile `decoy`",
        ] {
            let destination = f.base.join(out);
            let error = match inventory_with(&f.repo, "HEAD", &destination, &configuration_roots())
            {
                Err(error) => error,
                Ok(Outcome::Written(_)) => {
                    panic!("colon artifact path unexpectedly built: {out:?}")
                }
                Ok(Outcome::Refused(why)) => panic!("source unexpectedly refused: {why:?}"),
            };
            let failures: Vec<_> = error.lines().collect();
            assert_eq!(failures.len(), 2, "{error}");
            for failure in failures {
                assert!(
                    failure.contains("` failed: error: failed to join paths from `$"),
                    "{failure}"
                );
                assert!(failure.ends_with("` together"), "{failure}");
            }
            assert!(!destination.join("trust-dependencies.json").exists());
        }
    }

    #[test]
    fn cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings() {
        crate::trust_test_process::bounded!(
            cargo_failure_diagnostics_cover_canonical_paths_and_ancestor_warnings
        );
        let main = "fn main() {}\n";
        let library = "pub fn f() {}\n";
        let legacy = std::fs::read_to_string(real_root().join(".cargo/config.toml")).unwrap();
        let f = two_roots(
            "cargo-diagnostic-ancestor\nerror-repo",
            main,
            library,
            &[
                (".cargo/config", legacy),
                (".gitignore", "target/\n".to_owned()),
            ],
        );
        let real = f.repo.join("target/with\nerror-real");
        let alias = f.repo.join("target/with\nerror-alias");
        std::fs::create_dir_all(&real).unwrap();
        std::os::unix::fs::symlink(&real, &alias).unwrap();
        assert_eq!(
            std::fs::canonicalize(&alias).unwrap(),
            std::fs::canonicalize(&real).unwrap()
        );
        let mut roots = configuration_roots();
        roots.push(f.repo.clone()); // the fixture explicitly authorizes its own containing configuration
        assert_eq!(
            written(inventory_with(&f.repo, "HEAD", &alias, &roots).unwrap())
                .get("programs")
                .unwrap()
                .elements()
                .len(),
            2,
        );
        f.commit(&[
            (
                "crates/a/src/main.rs",
                "fn main() { let _: u8 = \"wrong type\"; }\n".to_owned(),
            ),
            (
                "crates/b/src/lib.rs",
                "pub fn f() { let _: u8 = \"wrong type\"; }\n".to_owned(),
            ),
        ]);
        let error = match inventory_with(&f.repo, "HEAD", &alias, &roots) {
            Err(error) => error,
            Ok(Outcome::Written(_)) => panic!("failed builds wrote an aliased inventory"),
            Ok(Outcome::Refused(why)) => panic!("source unexpectedly refused: {why:?}"),
        };
        assert_eq!(error, CARGO_ROOT_TYPE_ERRORS);
        assert!(!alias.join("trust-dependencies.json").exists());
        assert!(!real.join("trust-dependencies.json").exists());
        f.commit(&[
            ("crates/a/src/main.rs", main.to_owned()),
            ("crates/b/src/lib.rs", library.to_owned()),
        ]);
        assert_eq!(
            written(inventory_with(&f.repo, "HEAD", &alias, &roots).unwrap())
                .get("programs")
                .unwrap()
                .elements()
                .len(),
            2,
        );
    }

    #[test]
    fn an_executable_compiling_another_role_s_package_is_refused() {
        crate::trust_test_process::bounded!(
            an_executable_compiling_another_role_s_package_is_refused
        );
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
    pub(crate) fn admit(file: &str, rule: &str, extent: &str) -> String {
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

    fn observed_artifact(f: &Fixture, inventory: &Json, name: &str) -> PathBuf {
        use std::io::Write as _;
        use std::process::Stdio;

        let artifact = program(inventory, name).get("artifact").unwrap();
        let relative = artifact.get("path").and_then(Json::as_str).unwrap();
        assert!(relative.starts_with("release/deps/"), "{name}: {relative}");
        let path = f.base.join("out/target").join(name).join(relative);
        let metadata = std::fs::symlink_metadata(&path).unwrap();
        assert!(metadata.is_file() && !metadata.file_type().is_symlink());
        // Hash bytes on stdin so filename escaping cannot alter the digest token.
        let bytes = std::fs::read(&path).unwrap();
        let mut child = Command::new("shasum")
            .args(["-a", "256"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let sent = child.stdin.take().unwrap().write_all(&bytes);
        let digest = child.wait_with_output().unwrap();
        sent.unwrap();
        assert!(digest.status.success());
        let digest = String::from_utf8(digest.stdout).unwrap();
        assert_eq!(
            artifact.get("sha256").and_then(Json::as_str),
            digest.split_whitespace().next()
        );
        path
    }

    /// Independent failed-child evidence, without treating a Cargo diagnostic string as compiler status.
    pub(crate) fn failed_role_compilation(target: &Path, crate_name: &str) {
        let hex = |value: &str| -> Json {
            Json::Str(
                value
                    .as_bytes()
                    .iter()
                    .map(|byte| format!("{byte:02x}"))
                    .collect(),
            )
        };
        let mut failures = 0;
        for session in std::fs::read_dir(target.join(".archogen-provenance")).unwrap() {
            for record in std::fs::read_dir(session.unwrap().path().join("records")).unwrap() {
                let path = record.unwrap().path();
                assert_eq!(path.extension(), Some(OsStr::new("json")), "{path:?}");
                let value = crate::json::parse(&std::fs::read_to_string(path).unwrap()).unwrap();
                if value.get("exit") == Some(&Json::Number("1".to_owned())) {
                    assert_eq!(value.get("signal"), Some(&Json::Null));
                    assert!(value
                        .get("argv_hex")
                        .unwrap()
                        .elements()
                        .windows(2)
                        .any(|pair| { pair == [hex("--crate-name"), hex(crate_name)] }));
                    failures += 1;
                }
            }
        }
        assert_eq!(
            failures, 1,
            "one actual compiler child failed for {crate_name}"
        );
    }

    #[test]
    fn observed_artifact_hashing_preserves_newlines_and_backslashes_in_paths() {
        crate::trust_test_process::bounded!(
            observed_artifact_hashing_preserves_newlines_and_backslashes_in_paths
        );
        let base = real_root().join("target/trust-tests/artifact-hash-filenames");
        match std::fs::remove_dir_all(&base) {
            Ok(()) => {}
            Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
            Err(error) => panic!("cannot clear {base:?}: {error}"),
        }
        let payload = b"held artifact bytes\n";
        let expected = "ea40e457619b75b94b1cd15dfbaaf73b8cf1f6b5d039a77548178711af4e7629";
        assert_eq!(Digest::of(payload).hex(), expected);
        let relative = "release/deps/libfixture.rlib";
        let inventory = super::obj(vec![(
            "programs",
            Json::Array(vec![super::obj(vec![
                ("name", super::s("chk")),
                (
                    "artifact",
                    super::obj(vec![
                        ("path", super::s(relative)),
                        ("sha256", super::s(expected)),
                    ]),
                ),
            ])]),
        )]);
        for name in ["ordinary", "line\nbreak", "back\\slash", "both\nand\\slash"] {
            let f = Fixture {
                repo: base.join(name).join("repo"),
                base: base.join(name),
            };
            let path = f.base.join("out/target/chk").join(relative);
            std::fs::create_dir_all(path.parent().unwrap()).unwrap();
            std::fs::write(&path, payload).unwrap();
            assert_eq!(std::fs::read(&path).unwrap(), payload);
            assert_eq!(observed_artifact(&f, &inventory, "chk"), path);
        }
    }

    #[test]
    fn observed_artifacts_are_actual_binary_and_library_outputs() {
        crate::trust_test_process::bounded!(
            observed_artifacts_are_actual_binary_and_library_outputs
        );
        let f = two_roots(
            "observed-artifacts",
            "fn main() { println!(\"artifact-root\"); }\n",
            "pub fn f() {}\n",
            &[],
        );
        let inventory = written(f.run());
        let executable = observed_artifact(&f, &inventory, "gen");
        let result = Command::new(executable).output().unwrap();
        assert!(result.status.success());
        assert_eq!(result.stdout, b"artifact-root\n");
        let library = observed_artifact(&f, &inventory, "chk");
        assert!(library
            .extension()
            .is_some_and(|extension| extension == "rlib"));
        assert!(std::fs::read(library).unwrap().starts_with(b"!<arch>\n"));
    }

    #[test]
    fn dep_info_suffix_does_not_match_an_incremental_output_path() {
        use std::io::Write as _;
        use std::process::Stdio;

        crate::trust_test_process::bounded!(
            dep_info_suffix_does_not_match_an_incremental_output_path
        );
        // The reviewed mixed library has an unhashed dep-info name on the pinned producer.
        let library = manifest("b", "[lib]\ncrate-type = [\"rlib\", \"dylib\"]\n");
        let rule = concat!(
            "`crates/b/Cargo.toml`: `lib.crate-type` holds `crate-type`, ",
            "which can make the package another kind of crate"
        );
        let roots = format!(
            "{ROOTS}{}(defprogram dynamic (package \"crates/b\") (target dylib b) \
             (reason \"explicit alternate output\") (role-packages \"crates/b\"))\n",
            admit("crates/b/Cargo.toml", rule, &library),
        );
        let f = two_roots(
            "dep-info-extra-filename=inside",
            "fn main() {}\n",
            "pub fn check() -> u8 { 37 }\n",
            &[
                ("crates/b/Cargo.toml", library),
                ("trust/roots.eadl", roots),
            ],
        );
        let check_units = |inventory: &Json| {
            for (name, package, crate_name, source, expected) in [
                (
                    "gen",
                    "crates/a",
                    "a",
                    "crates/a/src/main.rs",
                    "536e506bb90914c243a12b397b9a998f85ae2cbd9ba02dfd03a9e155ca5ca0f4",
                ),
                (
                    "chk",
                    "crates/b",
                    "b",
                    "crates/b/src/lib.rs",
                    "a746195f0d21200b88bfa6608912329fec0d6b4b19c2f09a0e75628a1eada50b",
                ),
            ] {
                let units = program(inventory, name).get("units").unwrap().elements();
                assert_eq!(units.len(), 1, "{name}");
                assert_eq!(
                    units[0].get("package").and_then(Json::as_str),
                    Some(package)
                );
                assert_eq!(
                    units[0].get("crate").and_then(Json::as_str),
                    Some(crate_name)
                );
                assert_eq!(
                    units[0].get("files"),
                    Some(&super::obj(vec![(source, super::s(expected))]))
                );
            }
            assert_eq!(
                program(inventory, "chk")
                    .get("artifact")
                    .unwrap()
                    .get("path")
                    .and_then(Json::as_str),
                Some("release/deps/libb.rlib"),
            );
        };
        let ordinary = written(f.run());
        check_units(&ordinary);
        observed_artifact(&f, &ordinary, "chk");

        f.commit(&[(
            "Cargo.toml",
            format!("{WS}\n[profile.release]\nincremental = true\n"),
        )]);
        let incremental = std::panic::catch_unwind(|| {
            inventory_with(&f.repo, "HEAD", &f.base.join("out"), &configuration_roots())
        });
        // Retain the genuine products before the next clean inventory replaces them.
        let deps = f.base.join("out/target/chk/release/deps");
        let dep_info = std::fs::read(deps.join("b.d")).unwrap();
        assert!(String::from_utf8(dep_info.clone())
            .unwrap()
            .contains("crates/b/src/lib.rs"));
        let dep_names: Vec<_> = std::fs::read_dir(&deps)
            .unwrap()
            .map(|entry| entry.unwrap())
            .filter(|entry| entry.path().extension() == Some(OsStr::new("d")))
            .map(|entry| entry.file_name())
            .collect();
        assert_eq!(dep_names, vec![std::ffi::OsString::from("b.d")]);
        let bytes = std::fs::read(deps.join("libb.rlib")).unwrap();
        assert!(bytes.starts_with(b"!<arch>\n"));
        let saved_library = f.base.join("libb-incremental-before.rlib");
        std::fs::write(&saved_library, &bytes).unwrap();
        std::fs::write(f.base.join("incremental-before.d"), dep_info).unwrap();
        let mut hash = Command::new("shasum")
            .args(["-a", "256"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
            .unwrap();
        let sent = hash.stdin.take().unwrap().write_all(&bytes);
        let hashed = hash.wait_with_output().unwrap();
        sent.unwrap();
        assert!(hashed.status.success());
        let digest = String::from_utf8(hashed.stdout)
            .unwrap()
            .split_whitespace()
            .next()
            .unwrap()
            .to_owned();
        let result = match &incremental {
            Ok(Ok(Outcome::Written(_))) => "written".to_owned(),
            Ok(Ok(Outcome::Refused(why))) => format!("refused: {why:?}"),
            Ok(Err(why)) => format!("error: {why}"),
            Err(_) => "panicked".to_owned(),
        };
        std::fs::write(f.base.join("incremental-result.txt"), result).unwrap();

        // Recover before judging the captured incremental result, including in the edit.
        f.commit(&[("Cargo.toml", WS.to_owned())]);
        let recovered = written(f.run());
        check_units(&recovered);
        observed_artifact(&f, &recovered, "chk");
        let consumer = f.base.join("incremental-consumer.rs");
        std::fs::write(&consumer, "fn main() { println!(\"{}\", b::check()); }\n").unwrap();
        let executable = f.base.join("incremental-consumer");
        let built = Command::new("rustc")
            .env("RUSTUP_TOOLCHAIN", "1.95.0")
            .args(["--edition=2021", "--crate-name", "dep_info_suffix_consumer"])
            .arg(&consumer)
            .arg("--extern")
            .arg(format!("b={}", saved_library.display()))
            .arg("-o")
            .arg(&executable)
            .output()
            .unwrap();
        assert!(
            built.status.success(),
            "{}",
            String::from_utf8_lossy(&built.stderr)
        );
        let ran = Command::new(executable).output().unwrap();
        assert!(ran.status.success());
        assert_eq!(ran.stdout, b"37\n");
        std::fs::write(
            f.base.join("incremental-recovery.txt"),
            "recovered\nconsumer=37\n",
        )
        .unwrap();
        let inventory = written(
            incremental
                .expect("incremental collection must not panic")
                .expect("the literal no-suffix dep-info file must be read"),
        );
        check_units(&inventory);
        assert_eq!(
            program(&inventory, "chk")
                .get("artifact")
                .unwrap()
                .get("sha256")
                .and_then(Json::as_str),
            Some(digest.as_str()),
        );
    }

    #[test]
    fn observed_artifact_metadata_distinguishes_harness_and_same_named_binary() {
        crate::trust_test_process::bounded!(
            observed_artifact_metadata_distinguishes_harness_and_same_named_binary
        );
        let f = three_roots("observed-harness-artifact", "d", SAME, "", "");
        let inventory = written(f.run());
        let harness = observed_artifact(&f, &inventory, "diff");
        let result = Command::new(harness).arg("--list").output().unwrap();
        assert!(result.status.success());
        assert!(String::from_utf8(result.stdout)
            .unwrap()
            .contains("same: test"));
        let mut roots = std::fs::read_to_string(f.repo.join("trust/roots.eadl")).unwrap();
        roots.push_str(
            "(defprogram companion (package \"crates/i\") (target bin d) (reason \"fixture\"))\n",
        );
        f.commit(&[
            ("trust/roots.eadl", roots),
            ("crates/i/Cargo.toml", manifest("i", "[dev-dependencies]\nr = { path = \"../r\" }\n[[test]]\nname = \"d\"\nharness = false\n[[bin]]\nname = \"d\"\npath = \"src/main.rs\"\n")),
            ("crates/i/src/main.rs", "fn main() { println!(\"companion-bin\"); }\n".to_owned()),
            ("crates/i/tests/d.rs", "fn main() { assert_eq!(i::imp(3), r::refm(3)); println!(\"custom-harness\"); }\n".to_owned()),
        ]);
        let inventory = written(f.run());
        let harness = observed_artifact(&f, &inventory, "diff");
        let result = Command::new(harness).output().unwrap();
        assert!(result.status.success());
        assert_eq!(result.stdout, b"custom-harness\n");
    }

    const HARNESS_WARNING_SOURCE: &str = "mod support;\n#[test]\nfn same() { assert_eq!(support::identity(i::imp(3)), r::refm(3)); }\n";

    fn harness_warning_fixture(name: &str) -> Fixture {
        let ordinary = HARNESS_WARNING_SOURCE;
        let f = three_roots(name, "d", ordinary, "", "");
        f.commit(&[(
            "crates/i/tests/support/mod.rs",
            "pub fn identity(x: u32) -> u32 { x }\n".to_owned(),
        )]);
        f
    }

    fn harness_warning_configurations(
        f: &Fixture,
        inventory: &Json,
    ) -> std::collections::BTreeMap<(String, String), Json> {
        let units = program(inventory, "diff").get("units").unwrap().elements();
        assert_eq!(
            units.len(),
            3,
            "only i, r and the actual test compile: {units:?}"
        );
        for unit in units {
            let expected = match unit.get("crate").and_then(Json::as_str).unwrap() {
                "d" => vec!["crates/i/tests/d.rs", "crates/i/tests/support/mod.rs"],
                "i" => vec!["crates/i/src/lib.rs"],
                "r" => vec!["crates/r/src/lib.rs"],
                unexpected => panic!("unexpected compiled unit {unexpected}"),
            };
            let Json::Object(files) = unit.get("files").unwrap() else {
                panic!("file map")
            };
            assert_eq!(
                files.keys().map(String::as_str).collect::<Vec<_>>(),
                expected
            );
        }
        let actual: std::collections::BTreeMap<_, _> = units
            .iter()
            .map(|unit| {
                (
                    (
                        unit.get("package").unwrap().as_str().unwrap().to_owned(),
                        unit.get("crate").unwrap().as_str().unwrap().to_owned(),
                    ),
                    unit.get("configuration").unwrap().clone(),
                )
            })
            .collect();
        assert_eq!(
            actual.keys().cloned().collect::<Vec<_>>(),
            [
                ("crates/i".to_owned(), "d".to_owned()),
                ("crates/i".to_owned(), "i".to_owned()),
                ("crates/r".to_owned(), "r".to_owned()),
            ]
        );
        let artifact = observed_artifact(f, inventory, "diff");
        let result = Command::new(artifact).arg("--list").output().unwrap();
        assert!(result.status.success());
        assert!(String::from_utf8(result.stdout)
            .unwrap()
            .contains("same: test"));
        actual
    }

    #[test]
    fn compiler_provenance_preserves_admitted_harness_warning_records_and_recovery() {
        crate::trust_test_process::bounded!(
            compiler_provenance_preserves_admitted_harness_warning_records_and_recovery
        );
        let ordinary = HARNESS_WARNING_SOURCE;
        let f = harness_warning_fixture("compiler-harness-warning-records");
        let roots = std::fs::read_to_string(f.repo.join("trust/roots.eadl")).unwrap();
        let baseline = harness_warning_configurations(&f, &written(f.run()));
        for payload in [
            "not-a-compiler",
            "rustc",
            "rustc --crate-name phantom phantom.rs --out-dir bogus",
        ] {
            let assembly =
                format!("core::arch::global_asm!(r#\".warning \"note\nRunning `{payload}`\"\"#);");
            let warned = format!("{assembly}\n{ordinary}");
            f.commit(&[
                ("crates/i/tests/d.rs", warned.clone()),
                (
                    "trust/roots.eadl",
                    format!(
                        "{roots}{}",
                        admit("crates/i/tests/d.rs", "global_asm", &assembly)
                    ),
                ),
            ]);
            assert_eq!(
                harness_warning_configurations(&f, &written(f.run())),
                baseline
            );
        }
        f.commit(&[
            ("crates/i/tests/d.rs", ordinary.to_owned()),
            ("trust/roots.eadl", roots),
        ]);
        assert_eq!(
            harness_warning_configurations(&f, &written(f.run())),
            baseline
        );
    }

    #[test]
    fn compiler_provenance_harness_warning_admission_and_failure_recover() {
        crate::trust_test_process::bounded!(
            compiler_provenance_harness_warning_admission_and_failure_recover
        );
        let ordinary = HARNESS_WARNING_SOURCE;
        let f = harness_warning_fixture("compiler-harness-warning-failure-records");
        let roots = std::fs::read_to_string(f.repo.join("trust/roots.eadl")).unwrap();
        let payload = "rustc --crate-name phantom phantom.rs --out-dir bogus";
        let assembly =
            format!("core::arch::global_asm!(r#\".warning \"note\nRunning `{payload}`\"\"#);");
        let last = format!("{assembly}\n{ordinary}");
        f.commit(&[
            ("crates/i/tests/d.rs", last.clone()),
            (
                "trust/roots.eadl",
                format!(
                    "{roots}{}",
                    admit("crates/i/tests/d.rs", "global_asm", &assembly)
                ),
            ),
        ]);
        let baseline = harness_warning_configurations(&f, &written(f.run()));
        // The same warning needs its exact source admission, independently of framing.
        f.commit(&[("trust/roots.eadl", roots.clone())]);
        says(&refused(f.run()), "crates/i/tests/d.rs:1");
        assert!(!f.base.join("out/trust-dependencies.json").exists());
        f.commit(&[(
            "crates/i/tests/d.rs",
            format!("{last}\ncompile_error!(\"actual harness failure\");\n"),
        )]);
        let failure =
            match inventory_with(&f.repo, "HEAD", &f.base.join("out"), &configuration_roots()) {
                Err(error) => error,
                Ok(_) => {
                    panic!("a genuine compiler failure must leave the inventory unable to judge")
                }
            };
        assert!(
            failure.contains("cargo test --no-run --test d")
                && failure.contains("could not compile `i` (test \"d\")"),
            "{failure}"
        );
        assert!(!f.base.join("out/trust-dependencies.json").exists());
        failed_role_compilation(&f.base.join("out/target/diff"), "d");
        f.commit(&[("crates/i/tests/d.rs", ordinary.to_owned())]);
        assert_eq!(
            harness_warning_configurations(&f, &written(f.run())),
            baseline
        );
    }

    #[test]
    fn observed_role_artifacts_preserve_paths_and_disable_target_default_features() {
        crate::trust_test_process::bounded!(
            observed_role_artifacts_preserve_paths_and_disable_target_default_features
        );
        use std::io::Write;
        use std::process::Stdio;
        let test = "mod support;\n#[test]\nfn same() { assert_eq!(i::imp(support::VALUE), r::refm(support::VALUE)); }\n";
        let f = three_roots("observed-role-path-features", "d", test, "", "");
        let features = "[features]\ndefault = [\"tripwire\"]\ntripwire = []\n";
        let implementation_manifest = manifest(
            "i",
            &format!("{features}[dev-dependencies]\nr = {{ path = \"../r\" }}\n"),
        );
        let generator_manifest = manifest("g", features);
        let implementation = "#[cfg(feature = \"tripwire\")]\ncompile_error!(\"role default features were enabled\");\npub fn imp(x: u32) -> u32 { x }\n";
        let generator = "#[cfg(feature = \"tripwire\")]\ncompile_error!(\"generator default features were enabled\");\nmod helper;\nfn main() { let _ = helper::VALUE; }\n";
        let helper = "pub const VALUE: u32 = 3;\n";
        let mut roots = std::fs::read_to_string(f.repo.join("trust/roots.eadl")).unwrap();
        roots.push_str("(defgenerated \"crates/i/tests/support/mod.rs\" (generator \"crates/g/src/main.rs\") (inputs \"crates/g/src/helper.rs\") (command \"fixture\") (reason \"fixture\"))\n");
        for (path, content) in [
            ("crates/i/Cargo.toml", &implementation_manifest),
            ("crates/g/Cargo.toml", &generator_manifest),
        ] {
            roots.push_str(&admit(
                path,
                &format!("`{path}`: `[features]` makes a second configuration"),
                content,
            ));
        }
        f.commit(&[
            ("trust/roots.eadl", roots),
            ("crates/i/Cargo.toml", implementation_manifest),
            ("crates/g/Cargo.toml", generator_manifest),
            ("crates/i/src/lib.rs", implementation.to_owned()),
            ("crates/i/tests/support/mod.rs", helper.to_owned()),
            ("crates/g/src/main.rs", generator.to_owned()),
            ("crates/g/src/helper.rs", helper.to_owned()),
        ]);
        let baseline = written(f.run_into("plain"));
        let physical = f.base.join("physical'\nµ");
        std::fs::create_dir(&physical).unwrap();
        std::os::unix::fs::symlink(&physical, f.base.join("alias")).unwrap();
        for output in [
            "plain",
            "space'quote\\slash\nRunning `rustc`\rµ",
            "alias/output",
        ] {
            let inventory = if output == "plain" {
                baseline.clone()
            } else {
                written(f.run_into(output))
            };
            assert_eq!(items(&inventory), items(&baseline), "{output:?}");
            for name in ["refm", "imp", "chk", "diff"] {
                let actual = program(&inventory, name);
                for field in ["packages", "units", "provenance"] {
                    assert_eq!(
                        actual.get(field),
                        program(&baseline, name).get(field),
                        "{name} {field} {output:?}"
                    );
                }
                let units = actual.get("units").unwrap().elements();
                assert_eq!(units.len(), if name == "diff" { 3 } else { 1 });
                for unit in units {
                    let Json::Object(files) = unit.get("files").unwrap() else {
                        panic!("file map")
                    };
                    let inputs = match unit.get("crate").and_then(Json::as_str).unwrap() {
                        "i" => vec![("crates/i/src/lib.rs", implementation)],
                        "r" => vec![("crates/r/src/lib.rs", "pub fn refm(x: u32) -> u32 { x }\n")],
                        "c" => vec![("crates/c/src/lib.rs", "pub fn c() {}\n")],
                        "d" => vec![
                            ("crates/i/tests/d.rs", test),
                            ("crates/i/tests/support/mod.rs", helper),
                        ],
                        other => panic!("unexpected unit {other}"),
                    };
                    assert_eq!(
                        files,
                        &inputs
                            .into_iter()
                            .map(|(path, bytes)| (
                                path.to_owned(),
                                Json::Str(Digest::of(bytes.as_bytes()).hex())
                            ))
                            .collect()
                    );
                }
                let artifact = actual.get("artifact").unwrap();
                let relative = artifact.get("path").and_then(Json::as_str).unwrap();
                assert!(relative.starts_with("release/deps/"));
                let path = f.base.join(output).join("target").join(name).join(relative);
                let metadata = std::fs::symlink_metadata(&path).unwrap();
                assert!(metadata.is_file() && !metadata.file_type().is_symlink());
                // Hash stdin bytes: shasum's filename format escapes mixed-byte path spellings.
                let mut process = Command::new("shasum")
                    .args(["-a", "256"])
                    .stdin(Stdio::piped())
                    .stdout(Stdio::piped())
                    .spawn()
                    .unwrap();
                process
                    .stdin
                    .take()
                    .unwrap()
                    .write_all(&std::fs::read(&path).unwrap())
                    .unwrap();
                let hash = process.wait_with_output().unwrap();
                assert!(hash.status.success());
                let hash = String::from_utf8(hash.stdout).unwrap();
                assert_eq!(
                    artifact.get("sha256").and_then(Json::as_str),
                    hash.split_whitespace().next()
                );
                if name == "diff" {
                    let result = Command::new(path).arg("--list").output().unwrap();
                    assert!(result.status.success());
                    assert!(String::from_utf8(result.stdout)
                        .unwrap()
                        .contains("same: test"));
                }
            }
            let provenance = program(&inventory, "diff").get("provenance").unwrap();
            let Json::Object(files) = provenance else {
                panic!("provenance map")
            };
            assert_eq!(
                files.keys().map(String::as_str).collect::<Vec<_>>(),
                ["crates/g/src/helper.rs", "crates/g/src/main.rs"]
            );
            for (path, bytes) in [
                ("crates/g/src/helper.rs", helper),
                ("crates/g/src/main.rs", generator),
            ] {
                assert_eq!(
                    files[path].get("sha256").and_then(Json::as_str),
                    Some(Digest::of(bytes.as_bytes()).hex().as_str())
                );
            }
            assert!(f
                .base
                .join(output)
                .join("target-generators/crates-g-bin-g")
                .is_dir());
        }
        // Independently enable the generator's defaults through Cargo's public interface.
        // The inventory build must have compiled this same source with defaults disabled.
        let env = super::environment(
            "1.95.0",
            &f.base.join("default-probe-home"),
            &f.base.join("default-probe-target"),
        )
        .unwrap();
        let failure = super::run_cargo(
            &["build", "--release", "--locked", "--offline", "-p", "g"],
            &env,
            &f.repo,
        )
        .expect_err("the generator default-feature tripwire must fail when enabled");
        assert!(
            failure.contains("generator default features were enabled"),
            "{failure}"
        );
    }

    fn harness_item(inv: &Json) -> Json {
        items(inv)
            .into_iter()
            .find(|(p, i)| p == "refm+imp" && kind(i) == "comparison-harness")
            .expect("the pair's harness item")
            .1
    }

    #[test]
    fn a_crate_root_that_is_not_a_rs_file_is_refused_before_any_build() {
        crate::trust_test_process::bounded!(
            a_crate_root_that_is_not_a_rs_file_is_refused_before_any_build
        );
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
        crate::trust_test_process::bounded!(
            a_unit_belongs_to_the_package_whose_target_names_its_crate_root
        );
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
        crate::trust_test_process::bounded!(each_site_needs_an_admission_of_its_own);
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
        crate::trust_test_process::bounded!(an_admission_covers_its_site_s_whole_extent);
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
    fn a_shared_package_item_holds_every_compiled_environment_value() {
        crate::trust_test_process::bounded!(
            a_shared_package_item_holds_every_compiled_environment_value
        );
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let common_manifest = |values: [&str; 3]| {
            manifest(
                "common",
                &format!(
                    "authors = [\"{}\"]\ndescription = \"{}\"\nhomepage = \"{}\"\n",
                    values[0], values[1], values[2]
                ),
            )
        };
        let original = ["alpha", "beta", "gamma"];
        let f = two_roots(
            "every-compiled-env-value",
            "fn main() { let _ = common::VALUES; }\n",
            "pub fn f() { let _ = common::VALUES; }\n",
            &[
                ("crates/common/Cargo.toml", common_manifest(original)),
                (
                    "crates/common/src/lib.rs",
                    "pub const VALUES: [&str; 3] = [env!(\"CARGO_PKG_AUTHORS\"), env!(\"CARGO_PKG_DESCRIPTION\"), env!(\"CARGO_PKG_HOMEPAGE\")];\n".to_owned(),
                ),
                ("crates/a/Cargo.toml", manifest("a", dep)),
                ("crates/b/Cargo.toml", manifest("b", dep)),
            ],
        );
        let before = written(f.run());
        let before_config = package_item(&items(&before), "crates/common")
            .get("configuration")
            .unwrap()
            .clone();
        for changed in 0..3 {
            let mut values = original;
            values[changed] = "changed";
            f.commit(&[("crates/common/Cargo.toml", common_manifest(values))]);
            let after = written(f.run());
            for (inventory, expected) in [(&before, original), (&after, values)] {
                for root in ["gen", "chk"] {
                    let unit = program(inventory, root)
                        .get("units")
                        .unwrap()
                        .elements()
                        .iter()
                        .find(|u| u.get("package").and_then(Json::as_str) == Some("crates/common"))
                        .unwrap();
                    let mut actual = list(unit, "env");
                    actual.sort();
                    assert_eq!(
                        actual,
                        [
                            "CARGO_PKG_AUTHORS",
                            "CARGO_PKG_DESCRIPTION",
                            "CARGO_PKG_HOMEPAGE"
                        ]
                        .into_iter()
                        .zip(expected)
                        .map(|(name, value)| format!("{name}={value}"))
                        .collect::<Vec<_>>()
                    );
                }
            }
            assert_ne!(
                before_config,
                *package_item(&items(&after), "crates/common")
                    .get("configuration")
                    .unwrap(),
                "compiled environment value {changed} changed without changing the shared configuration"
            );
        }
    }

    #[test]
    fn shared_environment_records_keep_their_boundaries() {
        crate::trust_test_process::bounded!(shared_environment_records_keep_their_boundaries);
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let f = two_roots(
            "compiled-env-boundaries",
            "fn main() { let _ = common::VALUES; }\n",
            "pub fn f() { let _ = common::VALUES; }\n",
            &[
                (
                    "crates/common/Cargo.toml",
                    manifest(
                        "common",
                        "description = \"alphaCARGO_PKG_HOMEPAGE=omega\"\n",
                    ),
                ),
                (
                    "crates/common/src/lib.rs",
                    "pub const VALUES: [&str; 1] = [env!(\"CARGO_PKG_DESCRIPTION\")];\n".to_owned(),
                ),
                ("crates/a/Cargo.toml", manifest("a", dep)),
                ("crates/b/Cargo.toml", manifest("b", dep)),
            ],
        );
        let before = written(f.run());
        f.commit(&[
            (
                "crates/common/Cargo.toml",
                manifest("common", "description = \"alpha\"\nhomepage = \"omega\"\n"),
            ),
            (
                "crates/common/src/lib.rs",
                "pub const VALUES: [&str; 2] = [env!(\"CARGO_PKG_DESCRIPTION\"), env!(\"CARGO_PKG_HOMEPAGE\")];\n".to_owned(),
            ),
        ]);
        let after = written(f.run());
        for root in ["gen", "chk"] {
            let unit = |inventory| {
                program(inventory, root)
                    .get("units")
                    .unwrap()
                    .elements()
                    .iter()
                    .find(|u| u.get("package").and_then(Json::as_str) == Some("crates/common"))
                    .unwrap()
            };
            assert_eq!(
                list(unit(&before), "env"),
                ["CARGO_PKG_DESCRIPTION=alphaCARGO_PKG_HOMEPAGE=omega"]
            );
            assert_eq!(
                list(unit(&after), "env"),
                ["CARGO_PKG_DESCRIPTION=alpha", "CARGO_PKG_HOMEPAGE=omega"]
            );
            assert_eq!(
                unit(&before).get("configuration"),
                unit(&after).get("configuration"),
                "this control changes environment records alone"
            );
        }
        assert_ne!(
            package_item(&items(&before), "crates/common").get("configuration"),
            package_item(&items(&after), "crates/common").get("configuration"),
            "one value containing another name collapsed with two environment records"
        );
    }

    #[test]
    fn a_separator_inside_an_environment_value_cannot_hide_a_configuration_change() {
        crate::trust_test_process::bounded!(
            a_separator_inside_an_environment_value_cannot_hide_a_configuration_change
        );
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let f = two_roots(
            "compiled-env-separator-value",
            "fn main() { let _ = common::VALUES; }\n",
            "pub fn f() { let _ = common::VALUES; }\n",
            &[
                (
                    "crates/common/Cargo.toml",
                    manifest(
                        "common",
                        "description = \"alpha env-dep:CARGO_PKG_HOMEPAGE=omega\"\n",
                    ),
                ),
                (
                    "crates/common/src/lib.rs",
                    "pub const VALUES: [&str; 1] = [env!(\"CARGO_PKG_DESCRIPTION\")];\n".to_owned(),
                ),
                ("crates/a/Cargo.toml", manifest("a", dep)),
                ("crates/b/Cargo.toml", manifest("b", dep)),
            ],
        );
        let before = written(f.run());
        f.commit(&[
            (
                "crates/common/Cargo.toml",
                manifest("common", "description = \"alpha\"\nhomepage = \"omega\"\n"),
            ),
            (
                "crates/common/src/lib.rs",
                "pub const VALUES: [&str; 2] = [env!(\"CARGO_PKG_DESCRIPTION\"), env!(\"CARGO_PKG_HOMEPAGE\")];\n".to_owned(),
            ),
        ]);
        let after = written(f.run());
        for root in ["gen", "chk"] {
            let unit = |inventory| {
                program(inventory, root)
                    .get("units")
                    .unwrap()
                    .elements()
                    .iter()
                    .find(|u| u.get("package").and_then(Json::as_str) == Some("crates/common"))
                    .unwrap()
            };
            assert_eq!(
                list(unit(&before), "env"),
                ["CARGO_PKG_DESCRIPTION=alpha env-dep:CARGO_PKG_HOMEPAGE=omega"]
            );
            assert_eq!(
                list(unit(&after), "env"),
                ["CARGO_PKG_DESCRIPTION=alpha", "CARGO_PKG_HOMEPAGE=omega"]
            );
            assert_eq!(
                unit(&before).get("configuration"),
                unit(&after).get("configuration"),
                "this control changes environment records alone"
            );
        }
        assert_ne!(
            package_item(&items(&before), "crates/common").get("configuration"),
            package_item(&items(&after), "crates/common").get("configuration"),
            "a separator inside one value collapsed with separate environment records"
        );
    }

    #[test]
    fn configuration_keys_preserve_tokens_records_and_escaped_values() {
        use super::{unit_configuration, Unit};
        use std::collections::{BTreeMap, BTreeSet};

        crate::trust_test_process::bounded!(
            configuration_keys_preserve_tokens_records_and_escaped_values
        );
        let cases: &[&[&str]] = &[
            &[],
            &[""],
            &["alpha beta"],
            &["alpha", "beta"],
            &["beta", "alpha"],
            &["alpha", "alpha"],
            &[" env-dep:"],
            &["CARGO_PKG_DESCRIPTION=alpha env-dep:CARGO_PKG_HOMEPAGE=omega"],
            &["CARGO_PKG_DESCRIPTION=alpha", "CARGO_PKG_HOMEPAGE=omega"],
            &["quote\"slash\\newline\nreturn\rtab\tcontrol\u{1}é"],
        ];
        let mut keys = BTreeMap::new();
        for command in cases {
            for environment in cases {
                let u = Unit {
                    crate_name: "common".to_owned(),
                    package: "crates/common".to_owned(),
                    configuration: command.iter().map(|s| (*s).to_owned()).collect(),
                    files: BTreeSet::new(),
                    env: environment.iter().map(|s| (*s).to_owned()).collect(),
                    root: String::new(),
                    harness: false,
                };
                let key = unit_configuration(&u);
                assert_eq!(
                    key,
                    unit_configuration(&u),
                    "the same unit needs the same key"
                );
                let decoded =
                    crate::json::parse(&key).expect("a configuration key is structured JSON");
                assert_eq!(decoded.elements().len(), 2);
                for (part, expected) in decoded.elements().iter().zip([command, environment]) {
                    assert_eq!(
                        part.elements()
                            .iter()
                            .map(|v| v.as_str().expect("a configuration component is text"))
                            .collect::<Vec<_>>(),
                        *expected
                    );
                }
                assert!(
                    keys.insert(key, (command, environment)).is_none(),
                    "distinct token/record vectors collapsed"
                );
            }
        }
    }

    #[test]
    fn a_shared_package_s_environment_is_in_its_item_and_a_foreign_variable_is_refused() {
        crate::trust_test_process::bounded!(
            a_shared_package_s_environment_is_in_its_item_and_a_foreign_variable_is_refused
        );
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
    fn compiler_command_filter_ignores_admitted_nonterminal_warning_records() {
        crate::trust_test_process::bounded!(
            compiler_command_filter_ignores_admitted_nonterminal_warning_records
        );
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let ordinary = "pub fn f() {}\n";
        let f = two_roots(
            "compiler-command-warning-records",
            "fn main() { common::f(); }\n",
            "pub fn f() { common::f(); }\n",
            &[
                ("crates/common/Cargo.toml", manifest("common", "")),
                ("crates/common/src/lib.rs", ordinary.to_owned()),
                ("crates/a/Cargo.toml", manifest("a", dep)),
                ("crates/b/Cargo.toml", manifest("b", dep)),
            ],
        );
        written(f.run());
        let assembly = r##"core::arch::global_asm!(r#".warning "first
Running `not-a-compiler-first`"
.warning "last
Running `not-a-compiler-last`""#);"##;
        f.commit(&[
            (
                "crates/common/src/lib.rs",
                format!("{assembly}\n{ordinary}"),
            ),
            (
                "trust/roots.eadl",
                format!(
                    "{ROOTS}{}",
                    admit("crates/common/src/lib.rs", "global_asm", assembly)
                ),
            ),
        ]);
        written(f.run());

        // Observe the actual producer ordering without requiring a particular parser result.
        let env = super::environment(
            "1.95.0",
            &f.base.join("diagnostic-cargo-home"),
            &f.base.join("diagnostic-target"),
        )
        .unwrap();
        let args = [
            "build",
            "--lib",
            "--release",
            "--locked",
            "--offline",
            "--no-default-features",
            "--message-format=json",
            "-v",
            "-p",
            "b",
        ];
        let (_, stderr) = super::run_cargo(&args, &env, &f.repo).unwrap();
        let first_compilation = stderr.find("--crate-name common ").unwrap();
        let first_warning = stderr.find("Running `not-a-compiler-first`").unwrap();
        let later_warning = stderr.find("Running `not-a-compiler-last`").unwrap();
        assert!(first_compilation < first_warning && first_warning < later_warning);

        f.commit(&[
            ("crates/common/src/lib.rs", ordinary.to_owned()),
            ("trust/roots.eadl", ROOTS.to_owned()),
        ]);
        written(f.run());
    }

    #[test]
    fn compiler_provenance_ignores_admitted_compiler_shaped_warning_records() {
        crate::trust_test_process::bounded!(
            compiler_provenance_ignores_admitted_compiler_shaped_warning_records
        );
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let ordinary = "pub fn f() {}\n";
        let f = two_roots(
            "compiler-provenance-warning-records",
            "fn main() { common::f(); }\n",
            "pub fn f() { common::f(); }\n",
            &[
                ("crates/common/Cargo.toml", manifest("common", "")),
                ("crates/common/src/lib.rs", ordinary.to_owned()),
                ("crates/a/Cargo.toml", manifest("a", dep)),
                ("crates/b/Cargo.toml", manifest("b", dep)),
            ],
        );
        let configurations = |inventory: &Json, root| {
            let units = program(inventory, root).get("units").unwrap().elements();
            assert_eq!(
                units.len(),
                2,
                "only the root and common compile: {units:?}"
            );
            let expected = match root {
                "gen" => ["crates/a", "crates/common"],
                "chk" => ["crates/b", "crates/common"],
                _ => panic!("unexpected fixture role {root}"),
            };
            assert_eq!(
                units
                    .iter()
                    .map(|unit| unit.get("package").unwrap().as_str().unwrap())
                    .collect::<std::collections::BTreeSet<_>>(),
                expected.into_iter().collect(),
            );
            units
                .iter()
                .map(|unit| {
                    (
                        unit.get("package").unwrap().as_str().unwrap().to_owned(),
                        unit.get("configuration").unwrap().clone(),
                    )
                })
                .collect::<std::collections::BTreeMap<_, _>>()
        };
        let plain = written(f.run());
        for payload in [
            "not-a-compiler",
            "rustc",
            "rustc --crate-name phantom phantom.rs --out-dir bogus",
        ] {
            let assembly =
                format!("core::arch::global_asm!(r#\".warning \"note\nRunning `{payload}`\"\"#);");
            f.commit(&[
                (
                    "crates/common/src/lib.rs",
                    format!("{assembly}\n{ordinary}"),
                ),
                (
                    "trust/roots.eadl",
                    format!(
                        "{ROOTS}{}",
                        admit("crates/common/src/lib.rs", "global_asm", &assembly)
                    ),
                ),
            ]);
            let actual = written(f.run());
            for root in ["gen", "chk"] {
                assert_eq!(configurations(&actual, root), configurations(&plain, root));
            }
        }
        f.commit(&[
            ("crates/common/src/lib.rs", ordinary.to_owned()),
            ("trust/roots.eadl", ROOTS.to_owned()),
        ]);
        let recovered = written(f.run());
        for root in ["gen", "chk"] {
            assert_eq!(
                configurations(&recovered, root),
                configurations(&plain, root)
            );
        }
    }

    #[test]
    fn compiled_environment_refusals_cover_nonterminal_records() {
        crate::trust_test_process::bounded!(
            compiled_environment_refusals_cover_nonterminal_records
        );
        let allowed = "pub const PACKAGE: &str = env!(\"CARGO_PKG_NAME\");\n";
        let f = two_roots("compiled-env-nonterminal", "fn main() {}\n", allowed, &[]);
        written(f.run());
        f.commit(&[(
            "crates/b/src/lib.rs",
            format!(
                "{allowed}pub const FOREIGN: Option<&str> = option_env!(\"ARCHOGEN_UNDECLARED\");\n"
            ),
        )]);
        says(
            &refused(f.run()),
            "the environment variable `ARCHOGEN_UNDECLARED`",
        );
        assert!(!f.base.join("out/trust-dependencies.json").exists());

        // Hold the actual producer position, independently of compile's environment reader.
        let dependency_files: Vec<_> =
            std::fs::read_dir(f.base.join("out/target/chk/release/deps"))
                .unwrap()
                .map(|entry| entry.unwrap().path())
                .filter(|path| {
                    path.extension() == Some(OsStr::new("d"))
                        && path
                            .file_name()
                            .unwrap()
                            .to_str()
                            .unwrap()
                            .starts_with("b-")
                })
                .collect();
        assert_eq!(dependency_files.len(), 1);
        let dependency = std::fs::read_to_string(&dependency_files[0]).unwrap();
        let lines: Vec<_> = dependency.lines().collect();
        let foreign = lines
            .iter()
            .position(|line| *line == "# env-dep:ARCHOGEN_UNDECLARED")
            .unwrap();
        let package = lines
            .iter()
            .position(|line| *line == "# env-dep:CARGO_PKG_NAME=b")
            .unwrap();
        assert!(foreign > 0 && foreign < package && package < lines.len());

        f.commit(&[("crates/b/src/lib.rs", allowed.to_owned())]);
        written(f.run());
    }

    #[test]
    fn compiled_environment_refuses_package_namespace_lookalikes() {
        crate::trust_test_process::bounded!(
            compiled_environment_refuses_package_namespace_lookalikes
        );
        let allowed = "pub const PACKAGE: &str = env!(\"CARGO_PKG_NAME\");\n\
                       pub const ABSENT: Option<&str> = option_env!(\"CARGO_PKG_NOT_DEFINED\");\n";
        for (index, variable) in ["CARGO_PKGNAME", "PREFIX_CARGO_PKG_NAME"]
            .into_iter()
            .enumerate()
        {
            let f = two_roots(
                &format!("compiled-env-namespace-{index}"),
                "fn main() {}\n",
                allowed,
                &[],
            );
            let inventory = written(f.run());
            let root = program(&inventory, "chk");
            let unit = root
                .get("units")
                .unwrap()
                .elements()
                .iter()
                .find(|unit| unit.get("package").and_then(Json::as_str) == Some("crates/b"))
                .unwrap();
            let environment = list(unit, "env");
            assert!(environment.contains(&"CARGO_PKG_NAME=b".to_owned()));
            assert!(environment.contains(&"CARGO_PKG_NOT_DEFINED".to_owned()));

            f.commit(&[(
                "crates/b/src/lib.rs",
                format!(
                    "{allowed}pub const FOREIGN: Option<&str> = option_env!(\"{variable}\");\n"
                ),
            )]);
            says(
                &refused(f.run()),
                &format!("the environment variable `{variable}`"),
            );
            assert!(!f.base.join("out/trust-dependencies.json").exists());
            f.commit(&[("crates/b/src/lib.rs", allowed.to_owned())]);
            written(f.run());
        }
    }

    #[test]
    fn an_edit_no_root_s_compilation_reads_leaves_the_build_configuration() {
        crate::trust_test_process::bounded!(
            an_edit_no_root_s_compilation_reads_leaves_the_build_configuration
        );
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
        crate::trust_test_process::bounded!(
            the_harness_item_holds_what_it_compiles_beside_its_pair
        );
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
        crate::trust_test_process::bounded!(the_harness_is_paired_with_every_root_but_its_pair_s);
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
        crate::trust_test_process::bounded!(each_root_records_its_artifact_and_its_packages);
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
        crate::trust_test_process::bounded!(a_rs_file_read_as_data_is_tokenised_too);
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
    fn unknown_root_clauses_are_refused_at_every_position() {
        crate::trust_test_process::bounded!(unknown_root_clauses_are_refused_at_every_position);
        let clauses = [
            "(role generator)",
            "(package \"crates/a\")",
            "(target bin a)",
            "(role-packages \"crates/a\")",
            "(data \"data/one.txt\")",
        ];
        let valid = read_roots(&format!("(defroot gen {})", clauses.join(" ")))
            .expect("each declared clause is valid");
        assert_eq!(valid.programs.len(), 1);
        assert_eq!(valid.programs[0].data, ["data/one.txt"]);

        for position in [0, clauses.len() / 2, clauses.len()] {
            let mut invalid = clauses.to_vec();
            invalid.insert(position, "(role-pakages \"crates/other\")");
            let text = format!("(defroot gen {})", invalid.join(" "));
            let error =
                read_roots(&text).expect_err("an unknown clause must refuse in every position");
            assert!(
                error.contains("`gen` holds a clause `role-pakages` its form does not take"),
                "position {position}: {error}"
            );
        }
    }

    #[test]
    fn duplicate_root_clauses_are_refused_at_early_middle_and_final_positions() {
        crate::trust_test_process::bounded!(
            duplicate_root_clauses_are_refused_at_early_middle_and_final_positions
        );
        let clauses = [
            "(data \"data/one.txt\")",
            "(role generator)",
            "(package \"crates/a\")",
            "(target bin a)",
            "(role-packages \"crates/a\")",
        ];
        let valid = read_roots(&format!("(defroot gen {})", clauses.join(" ")))
            .expect("one data clause is valid");
        assert_eq!(valid.programs.len(), 1);
        assert_eq!(valid.programs[0].data, ["data/one.txt"]);

        for position in [1, clauses.len() / 2, clauses.len()] {
            let mut invalid = clauses.to_vec();
            invalid.insert(position, "(data \"data/two.txt\")");
            let text = format!("(defroot gen {})", invalid.join(" "));
            let error =
                read_roots(&text).expect_err("a second data clause must refuse in every position");
            assert!(
                error.contains("`gen` holds `data` twice"),
                "position {position}: {error}"
            );
        }
    }

    #[test]
    fn reader_errors_are_refused_even_when_a_root_form_recovers() {
        crate::trust_test_process::bounded!(
            reader_errors_are_refused_even_when_a_root_form_recovers
        );
        let root = "(defroot gen (role generator) (package \"crates/a\") (target bin a))";
        let valid = read_roots(root).expect("the complete root is valid");
        assert_eq!(valid.programs.len(), 1);
        assert_eq!(valid.programs[0].name, "gen");

        for (text, code) in [
            (format!(")\n{root}"), "read-unexpected-close"),
            (format!("{root}\n)"), "read-unexpected-close"),
            (root[..root.len() - 1].to_owned(), "read-unclosed-list"),
        ] {
            // Recovery retains a usable root; accepting that partial document would hide the syntax error.
            let mut sources = eadl_front::SourceMap::new();
            let id = sources.add(super::ROOTS, &text).expect("source fits");
            let (document, diagnostics) = eadl_front::read(&sources, id);
            assert!(diagnostics.has_errors(), "{text}");
            assert_eq!(document.forms.len(), 1, "{text}");
            assert_eq!(document.forms[0].head(), Some("defroot"), "{text}");

            let error =
                read_roots(&text).expect_err("syntax errors must refuse the recovered root");
            assert!(error.contains(code), "{text}: {error}");
            assert!(error.contains(super::ROOTS), "{text}: {error}");
        }
    }

    #[test]
    fn the_roots_file_is_read_strictly() {
        crate::trust_test_process::bounded!(the_roots_file_is_read_strictly);
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
        crate::trust_test_process::bounded!(
            a_harness_or_a_library_root_compiling_another_role_s_package_is_refused
        );
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
        crate::trust_test_process::bounded!(an_incbin_under_asm_and_under_naked_asm_is_refused);
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
        crate::trust_test_process::bounded!(a_no_mangle_memcmp_in_the_harness_is_refused);
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
        crate::trust_test_process::bounded!(a_second_run_writes_the_same_bytes);
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
        crate::trust_test_process::bounded!(a_patch_in_the_workspace_manifest_is_refused);
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
        crate::trust_test_process::bounded!(a_site_two_roots_compile_is_counted_once);
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
        crate::trust_test_process::bounded!(
            an_incremental_profile_leaves_the_configuration_the_same_from_two_directories
        );
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
        crate::trust_test_process::bounded!(a_root_s_role_is_one_the_record_names);
        // R9 2: a root declared `(role harness)` escaped the two-roles rule.
        let text = "(defroot gen (role generator) (package \"crates/a\") (target bin a))\n\
                    (defroot chk (role harness) (package \"crates/b\") (target lib))\n";
        let e = read_roots(text).expect_err("a root with the harness's role");
        assert!(e.contains("`chk`'s role `harness`"), "{e}");
    }

    #[test]
    fn an_extent_runs_past_a_brace_delimited_const_generic_argument() {
        crate::trust_test_process::bounded!(
            an_extent_runs_past_a_brace_delimited_const_generic_argument
        );
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
        crate::trust_test_process::bounded!(a_role_package_that_names_no_package_is_refused);
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
        crate::trust_test_process::bounded!(a_root_s_run_time_data_is_hashed_and_shared);
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
        crate::trust_test_process::bounded!(an_admitted_include_or_path_names_a_rs_file);
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
        crate::trust_test_process::bounded!(
            an_alias_or_a_wrapper_of_a_refused_construct_is_inadmissible
        );
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

    #[test]
    fn cargo_source_paths_with_diagnostic_substrings_remain_in_configuration() {
        crate::trust_test_process::bounded!(
            cargo_source_paths_with_diagnostic_substrings_remain_in_configuration
        );
        let f = two_roots(
            "retained-diagnostic-source-paths",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[
                (
                    "crates/a/Cargo.toml",
                    manifest("a", "[[bin]]\nname = \"a\"\npath = \"src/kept--json.rs\"\n"),
                ),
                ("crates/a/src/kept--json.rs", "fn main() {}\n".to_owned()),
                (
                    "crates/b/Cargo.toml",
                    manifest("b", "[lib]\npath = \"src/kept--error-format.rs\"\n"),
                ),
                (
                    "crates/b/src/kept--error-format.rs",
                    "pub fn f() {}\n".to_owned(),
                ),
            ],
        );
        let inventory = written(f.run());
        // These are committed target paths, not diagnostic options despite their interior substrings.
        for (root, source) in [
            ("gen", "crates/a/src/kept--json.rs"),
            ("chk", "crates/b/src/kept--error-format.rs"),
        ] {
            let units = program(&inventory, root).get("units").unwrap().elements();
            assert_eq!(units.len(), 1, "{root} has exactly its own compiled unit");
            let arguments: Vec<_> = units[0]
                .get("configuration")
                .unwrap()
                .elements()
                .iter()
                .map(|arg| arg.as_str().expect("a configuration argument is text"))
                .collect();
            assert!(
                arguments.contains(&source),
                "{root} lost its source argument {source:?}: {arguments:?}"
            );
            assert!(units[0].get("files").unwrap().get(source).is_some());
        }
    }

    #[test]
    fn cargo_dependency_paths_with_equals_preserve_extern_arguments() {
        crate::trust_test_process::bounded!(
            cargo_dependency_paths_with_equals_preserve_extern_arguments
        );
        let f = two_roots(
            "retained-equals-dependency-paths",
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
        );
        for out in ["plain", "scratch=one", "scratch=one=two"] {
            let inventory = written(f.run_into(out));
            for (root, package, expected) in [
                ("gen", "crates/a", "--extern common=libcommon.rlib"),
                ("chk", "crates/b", "--extern common=libcommon.rmeta"),
            ] {
                let units = program(&inventory, root).get("units").unwrap().elements();
                assert_eq!(units.len(), 2, "{root} retains its dependency at {out:?}");
                let own = units
                    .iter()
                    .find(|unit| unit.get("package").and_then(Json::as_str) == Some(package))
                    .expect("the root's own compiled unit is inventoried");
                let arguments: Vec<_> = own
                    .get("configuration")
                    .unwrap()
                    .elements()
                    .iter()
                    .map(|arg| arg.as_str().expect("a configuration argument is text"))
                    .collect();
                // Only the first '=' separates the dependency name; scratch-path separators are path data.
                assert!(
                    arguments.contains(&expected),
                    "{root} at {out:?} lost its dependency argument {expected:?}: {arguments:?}"
                );
            }
        }
    }

    #[test]
    fn cargo_codegen_arguments_remain_single_normalized_options() {
        crate::trust_test_process::bounded!(
            cargo_codegen_arguments_remain_single_normalized_options
        );
        let f = two_roots(
            "retained-codegen-arguments",
            "fn main() {}\n",
            "pub fn f() {}\n",
            &[(
                "Cargo.toml",
                format!("{WS}\n[profile.release]\nopt-level = 1\ncodegen-units = 3\n\n[profile.release.package.b]\nopt-level = 2\ncodegen-units = 5\n"),
            )],
        );
        let inventory = written(f.run());
        for (root, package, options) in [
            (
                "gen",
                "crates/a",
                [
                    ("-C opt-level=1", "opt-level=1"),
                    ("-C codegen-units=3", "codegen-units=3"),
                ],
            ),
            (
                "chk",
                "crates/b",
                [
                    ("-C opt-level=2", "opt-level=2"),
                    ("-C codegen-units=5", "codegen-units=5"),
                ],
            ),
        ] {
            let units = program(&inventory, root).get("units").unwrap().elements();
            assert_eq!(units.len(), 1);
            assert_eq!(
                units[0].get("package").and_then(Json::as_str),
                Some(package)
            );
            let arguments: Vec<_> = units[0]
                .get("configuration")
                .unwrap()
                .elements()
                .iter()
                .map(|arg| arg.as_str().expect("a configuration argument is text"))
                .collect();
            for (expected, operand) in options {
                // Expectations come from the committed profile, independently of the normalized output.
                assert_eq!(
                    arguments
                        .iter()
                        .copied()
                        .filter(|arg| *arg == expected)
                        .count(),
                    1,
                    "{root} must retain {expected:?} once: {arguments:?}"
                );
                assert!(
                    !arguments.contains(&operand),
                    "{root} retained a consumed operand {operand:?}: {arguments:?}"
                );
            }
        }
    }

    #[test]
    fn temporary_process_paths_are_not_admitted_as_compiled_environment_inputs() {
        crate::trust_test_process::bounded!(
            temporary_process_paths_are_not_admitted_as_compiled_environment_inputs
        );
        for variable in ["TMPDIR", "TMP", "TEMP"] {
            let f = two_roots(
                &format!("compiled-temporary-input-{variable}"),
                "fn main() {}\n",
                &format!("pub const H: Option<&str> = option_env!(\"{variable}\");\n"),
                &[],
            );
            says(
                &refused(f.run()),
                &format!("the environment variable `{variable}`"),
            );
        }
    }
}

#[cfg(test)]
#[path = "trust_adequacy_parent_tests.rs"]
mod adequacy_tests;
