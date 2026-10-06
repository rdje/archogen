//! The substitutability relation (`ROADMAP.md` §4.2 `eadl-resolve`, §5.2): when what one provider offers satisfies
//! what a requirement asks.
//!
//! ⭐ **The production relation is being built here, slice by slice, beside its executable model** (leaf `M3.1.2`).
//! [`vocabulary`] is its typed table: `/1`, read from `docs/semantics/vocabulary/vocabulary.eadl` against the kind
//! `deffact` (`M3.1.2.2`). [`model`] transcribes
//! `docs/decisions/decision_substitutability-relation.md` rule by rule, each rule citing its section, optimised for
//! nothing (leaf `M3.1.1.1`, `docs/decisions/decision_executable-design-reviews.md`). A design review reads the
//! record beside the model and runs the model's exhaustive checker and its falsification corpus
//! (`tests/checker.rs`, `tests/corpus.rs`), so a question a machine can settle — whether every input has one
//! outcome, whether a value is ever read the wrong way — is settled by one. The production relation, leaf
//! `M3.1.2`, is tested against the model over the same bounded universe. `archogen check` does not call it.

pub mod model;
pub mod vocabulary;
