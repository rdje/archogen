//! What the tests share: the shipped vocabulary and catalog, and a description resolved and lowered as the engine runs
//! it — checked by the language's check, searched, its `archogen-assignment/2` record written, and that record lowered.
//!
//! ⚠️ `#![allow(dead_code)]` is deliberate, as in `crates/eadl-resolve/tests/common/mod.rs`: Cargo compiles every
//! integration test as its own crate with its own copy of this module, so an item `refusals.rs` uses and `corpus.rs`
//! does not is dead code in the second, and this workspace runs clippy at `-D warnings`.

#![allow(dead_code)]

use std::path::{Path, PathBuf};

use archogen_plan::{lower, Refusal};
use archogen_plan_check::check::plan_file;
use eadl_front::module::DirectoryModules;
use eadl_front::{Form, Program, SourceMap};
use eadl_model::check::{check, check_program, resolved_declarations, shipped_registry};
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::adapter::Catalog;
use eadl_resolve::plan;
use eadl_resolve::search::{search, Limits, Policy, Search};
use eadl_resolve::vocabulary::Vocabulary;

/// The repository's root.
pub fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// The shipped kind modules, by path.
fn kinds() -> Vec<(String, String)> {
    [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root().join(r)).expect("a kind module"),
        )
    })
    .collect()
}

/// The shipped vocabulary, read once per test binary: the bounded checker lowers tens of thousands of systems, and
/// reading the kinds again for each was most of its time.
pub fn vocabulary() -> Vocabulary {
    static READ: std::sync::OnceLock<Vocabulary> = std::sync::OnceLock::new();
    READ.get_or_init(|| {
        let registry = shipped_registry(&mut SourceMap::new(), &kinds()).expect("the kinds load");
        Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
    })
    .clone()
}

/// The shipped catalog, read once per test binary.
pub fn catalog() -> Catalog {
    static READ: std::sync::OnceLock<Catalog> = std::sync::OnceLock::new();
    READ.get_or_init(|| {
        Catalog::shipped(&mut SourceMap::new(), &vocabulary()).expect("the shipped catalog reads")
    })
    .clone()
}

/// `text`, a plain description, as the language's check judges it under the profile (`archogen check`): the program
/// it accepts, or its verdict. The lowering's input is a program that check accepted (the record's §1).
pub fn checked(text: &str) -> Result<Program, &'static str> {
    let mut sources = SourceMap::new();
    let registry = shipped_registry(&mut sources, &kinds()).expect("the kinds load");
    let id = sources.add("description", text.to_owned()).expect("small");
    let outcome = check(&sources, id, &registry, &RT_STATIC_UP_V1);
    if outcome.is_ok() {
        Ok(plan::plain(outcome.declarations))
    } else {
        Err(outcome.verdict.slug())
    }
}

/// `record`, a hand-written `archogen-assignment/2` record whose `"PROGRAM"` stands for the digest of `text`'s program,
/// lowered over that program.
pub fn lowered_record(text: &str, record: &str) -> Result<String, Refusal> {
    let program = checked(text).expect("the language's check accepts the description");
    let record = record.replace("PROGRAM", &plan::program_digest(&program));
    lower(&program, &record, &vocabulary(), &catalog()).map(|plan| plan.text())
}

/// A hand-written record judged by the configuration checker, as the lowering takes only a record it accepted (the
/// record's §1), then lowered: its refusal, or the plan's text. A record the checker refuses fails the test.
pub fn judged_record(text: &str, record: &str) -> Result<String, Refusal> {
    let program = checked(text).expect("the language's check accepts the description");
    let record = record.replace("PROGRAM", &plan::program_digest(&program));
    let mut sources = SourceMap::new();
    let id = sources
        .add("description.eadl", text.to_owned())
        .expect("small");
    let (_, verdict) = plan_file(
        &mut sources,
        id,
        &DirectoryModules::new(root()),
        &RT_STATIC_UP_V1,
        "plan.json",
        &record,
    );
    if let Err(refused) = verdict {
        let findings: Vec<String> = refused
            .findings
            .iter()
            .map(|f| format!("{}: {}", f.code, f.message))
            .collect();
        panic!(
            "the configuration checker refuses the record, {}: {}\n{record}",
            refused.status.slug(),
            findings.join("; ")
        );
    }
    lower(&program, &record, &vocabulary(), &catalog()).map(|plan| plan.text())
}

/// What lowering one system of a description gave: the plan's text, or the search's or the lowering's refusal.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Lowered {
    /// The plan's text.
    Plan(String),
    /// The lowering's refusal.
    Refused(Refusal),
    /// The search resolved no plan: its verdict.
    Unresolved(String),
    /// The language's check refused the description: its verdict. Such a description is no input (the record's §1).
    Rejected(String),
}

/// `text`, a plain description, its `system` (the first when `None`) searched, its record written, and lowered.
pub fn lowered(text: &str, system: Option<&str>) -> Lowered {
    match checked(text) {
        Ok(program) => lowered_program(&program, system),
        Err(verdict) => Lowered::Rejected(verdict.to_string()),
    }
}

/// The module tree whose root module is the file at `path`, elaborated over its directory and checked as the engine
/// checks one (`check_program`), its first system searched, its record written, and that record lowered.
pub fn lowered_tree(path: &Path) -> Lowered {
    let text = std::fs::read_to_string(path).expect("the root module");
    let mut sources = SourceMap::new();
    let registry = shipped_registry(&mut sources, &kinds()).expect("the kinds load");
    let id = sources
        .add(path.display().to_string(), text)
        .expect("small");
    let modules = DirectoryModules::new(path.parent().expect("a directory"));
    let (program, diagnostics) = eadl_front::elaborate_source(&mut sources, &modules, id);
    if diagnostics.has_errors() {
        return Lowered::Rejected("invalid-description".to_string());
    }
    let outcome = check_program(&program, &registry, &RT_STATIC_UP_V1);
    if !outcome.is_ok() {
        return Lowered::Rejected(outcome.verdict.slug().to_string());
    }
    lowered_program(&program, None)
}

/// `program`'s `system` (the first when `None`) searched, its record written, and lowered.
fn lowered_program(program: &Program, system: Option<&str>) -> Lowered {
    let vocabulary = vocabulary();
    let catalog = catalog();
    // A module tree is read as the language checks it, its names qualified.
    let flattened;
    let declarations: &[Form] = if program.instances.len() == 1 {
        &program.root().declarations
    } else {
        flattened = resolved_declarations(program).0;
        &flattened
    };
    let decl = declarations
        .iter()
        .find(|f| {
            f.head() == Some("defsystem")
                && system.is_none_or(|s| f.items().get(1).and_then(Form::as_symbol) == Some(s))
        })
        .expect("the system")
        .clone();
    let policy = Policy {
        adapters: catalog.clone(),
        ..Policy::default()
    };
    let resolution = match search(
        declarations,
        &decl,
        RT_STATIC_UP_V1.caller_level,
        &vocabulary,
        &policy,
        Limits::DEFAULT,
    ) {
        Search::Resolved(resolution) => resolution,
        Search::Refused { verdict, .. } => return Lowered::Unresolved(format!("{verdict:?}")),
        Search::Inconclusive { .. } => return Lowered::Unresolved("inconclusive".to_string()),
    };
    let record = plan::write(
        &resolution,
        RT_STATIC_UP_V1.id,
        &plan::program_digest(program),
    );
    match lower(program, &record, &vocabulary, &catalog) {
        Ok(plan) => Lowered::Plan(plan.text()),
        Err(refusal) => Lowered::Refused(refusal),
    }
}

/// A plan's text with its two digests written `…`, as the record's worked texts write them.
pub fn without_digests(text: &str) -> String {
    text.lines()
        .map(|line| {
            if line.starts_with("(program \"") {
                "(program \"…\")".to_string()
            } else if line.starts_with("(assignment \"") {
                "(assignment \"…\")".to_string()
            } else {
                line.to_string()
            }
        })
        .map(|line| line + "\n")
        .collect()
}
