//! The configuration checker (`ROADMAP.md` §4.4, §7.1; leaf `M3.5`): it reads a description and the assignment record
//! the search wrote, and judges the record against the description, apart from the generator that wrote it — the
//! configuration checker's one role package, its library the role's root (`docs/specs/trust/decision_trust-role-programs.md`
//! §2).
//!
//! ⭐ **Its relation is the reference model, moved.** [`model`] transcribes
//! `docs/decisions/decision_substitutability-relation.md` rule by rule, moved here from `eadl-resolve` as a pure rename
//! (leaf `M3.5.3`); its exhaustive checker, its falsification corpus and the comparison that holds the generator's
//! production relation equal to it stay in `crates/eadl-resolve/tests`, which takes this crate as a development
//! dependency — the checker's relation and the generator's one derivation held equal, which the trust gate holds as a
//! shared derivation (`M3.5.3.1`). [`assignment`] reads and writes the record it judges, `archogen-assignment/1`
//! (`M3.5.4.1`); [`check`] runs the checks on it, check (1) and the stale plan first (`M3.5.4.2`), the others with the
//! rest of `M3.5.4`; the executable comes with `M3.5.6`.

pub mod assignment;
pub mod check;
pub mod model;
pub mod providers;
