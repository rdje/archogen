//! A declaration whose local name is a vocabulary fact is refused, at the root and inside an imported module
//! (`SR-H4`; `docs/decisions/decision_substitutability-relation.md` §1.1, R16 1; leaf `M3.1.2.4`).
//!
//! Module resolution binds a `uses`, `needs` or `refines` operand to a declaration of its own instance first
//! (reference §6 rule 10), so a declaration named `low-power-timer` inside a module captures that module's
//! `(needs low-power-timer)`; and the import renames it `<alias>.low-power-timer`, which no fact is called. So the
//! refusal reads each instance's declarations by the local name each is written with, never the qualified one.

use eadl_front::module::{elaborate_source, MemoryModules};
use eadl_front::{read, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_resolve::refusal::Cause;
use eadl_resolve::requirement::{
    declarations_named_like_facts, program_declarations_named_like_facts,
};
use eadl_resolve::vocabulary::Vocabulary;

fn vocabulary() -> Vocabulary {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf();
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root.join(r)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the kinds load");
    Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
}

/// The facts named by refusals of `root`'s module tree, `library` importable as `hw.timer`.
fn refused_in_tree(root: &str, library: &str) -> Vec<String> {
    let modules = MemoryModules::new().with("hw.timer", library);
    let mut sources = SourceMap::new();
    let id = sources.add("app.eadl", root).expect("small");
    let (program, diagnostics) = elaborate_source(&mut sources, &modules, id);
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    program_declarations_named_like_facts(&program, &vocabulary())
        .into_iter()
        .map(|r| {
            assert_eq!(r.cause, Cause::DeclarationNamedLikeFact);
            r.fact.expect("names the fact")
        })
        .collect()
}

const ROOT: &str = "(eadl-version eadl/1)
(defmodule app.system
  (version 1 0)
  (import hw.timer (as board))
  (export app.rt)
  (defsystem app.rt
    (requires (uses board.timer.counter))))
";

#[test]
fn a_declaration_named_like_a_fact_inside_an_imported_module_is_refused_by_its_local_name() {
    let library = "(eadl-version eadl/1)
(defmodule hw.timer
  (version 1 0)
  (export timer.counter)
  (defblock low-power-timer (offers (tick-unit ns)))
  (defblock timer.counter
    (offers (counter-width 32 bit))
    (needs low-power-timer)))
";
    // Qualified, the block is `board.low-power-timer`, which no fact is called; its local name is one.
    assert_eq!(refused_in_tree(ROOT, library), ["low-power-timer"]);
}

#[test]
fn a_module_tree_with_no_such_name_is_not_refused() {
    let library = "(eadl-version eadl/1)
(defmodule hw.timer
  (version 1 0)
  (export timer.counter)
  (defblock timer.counter (offers (counter-width 32 bit) low-power-timer)))
";
    // A fact offered or needed is the vocabulary's; only a declaration's own name is refused.
    assert!(refused_in_tree(ROOT, library).is_empty());
}

#[test]
fn a_declaration_named_like_a_fact_at_the_root_of_one_file_is_refused() {
    let mut sources = SourceMap::new();
    let id = sources
        .add(
            "one.eadl",
            "(eadl-version eadl/1)\n(defblock tick-unit (offers uart))\n(defservice time.monotonic (requires (needs uart)))\n",
        )
        .expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(!diagnostics.has_errors());
    let refused = declarations_named_like_facts(document.declarations(), &vocabulary());
    assert_eq!(refused.len(), 1);
    assert_eq!(refused[0].fact.as_deref(), Some("tick-unit"));
}
