//! The substitutability relation (`ROADMAP.md` §4.2 `eadl-resolve`, §5.2): when what one provider offers satisfies
//! what a requirement asks.
//!
//! ⭐ **The production relation is being built here, slice by slice, held to its executable model** (leaf `M3.1.2`).
//! [`vocabulary`] is its typed table: `/1`, read from `docs/semantics/vocabulary/vocabulary.eadl` against the kind
//! `deffact` (`M3.1.2.2`). [`value`] reads and compares the domains' values, and [`offer`] reads a provider's offers
//! and absences, each refusal one of [`refusal`]'s causes (`M3.1.2.3`); [`requirement`] reads what a side requires,
//! read whole at the positions presence reads, and refuses its contradictions (`M3.1.2.4`); [`relation`] judges one
//! requirement at one provider and enumerates every provider's answer (`M3.1.2.5`). The model,
//! `archogen_plan_check::model`, transcribes `docs/decisions/decision_substitutability-relation.md` rule by rule, each
//! rule citing its section, optimised for nothing (leaf `M3.1.1.1`, `docs/decisions/decision_executable-design-reviews.md`),
//! and is the configuration checker's since leaf `M3.5.3` moved it there; this crate takes it as a development
//! dependency. A design review reads the record beside the model and runs the model's exhaustive checker and its
//! falsification corpus (`tests/checker.rs`, `tests/corpus.rs`), so a question a machine can settle — whether every
//! input has one outcome, whether a value is ever read the wrong way — is settled by one. The production relation,
//! leaf `M3.1.2`, is tested against the model over the same bounded universe. [`joint`] judges the bindings of one complete
//! assignment together — a provider's channels and the regions bound providers own (`M3.3.2`). `archogen check` does
//! not call any of it.

pub mod joint;
pub mod offer;
pub mod refusal;
pub mod relation;
pub mod requirement;
pub mod value;
pub mod vocabulary;
