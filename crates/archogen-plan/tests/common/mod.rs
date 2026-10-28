//! What the tests share: the shipped vocabulary and catalog, and a description resolved and lowered as the engine runs
//! it — searched, its `archogen-assignment/2` record written, and that record lowered.

use std::path::{Path, PathBuf};

use archogen_plan::{lower, Refusal};
use eadl_front::{Form, SourceMap};
use eadl_model::check::shipped_registry;
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

/// The shipped vocabulary.
pub fn vocabulary() -> Vocabulary {
    let kinds: Vec<(String, String)> = [
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
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the kinds load");
    Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
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
}

/// `text`, a plain description, its `system` (the first when `None`) searched, its record written, and lowered.
pub fn lowered(text: &str, system: Option<&str>) -> Lowered {
    let vocabulary = vocabulary();
    let catalog =
        Catalog::shipped(&mut SourceMap::new(), &vocabulary).expect("the shipped catalog reads");
    let mut sources = SourceMap::new();
    let id = sources.add("description", text.to_owned()).expect("small");
    let (document, diagnostics) = eadl_front::read(&sources, id);
    assert!(!diagnostics.has_errors(), "the description reads");
    let declarations: Vec<Form> = document
        .forms
        .into_iter()
        .filter(|f| f.head() != Some("eadl-version"))
        .collect();
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
        &declarations,
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
    let program = plan::program_digest(&plan::plain(declarations.clone()));
    let record = plan::write(&resolution, RT_STATIC_UP_V1.id, &program);
    match lower(&declarations, &program, &record, &vocabulary, &catalog) {
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
