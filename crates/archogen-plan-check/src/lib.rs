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
//! shared derivation (`M3.5.3.1`). [`assignment`] reads and writes the record it judges, `archogen-assignment/2`
//! (`M3.5.4.1`; `/2` by `M3.2.5`), an engine adapter's instance judged by the checker's own table, [`adapters`]; [`check`] runs the checks on it, check (1) and the stale plan first (`M3.5.4.2`), the description
//! re-checked (`M3.5.4.3`, `.6`), and the bindings over [`requesters`] (`M3.5.4.4`), then the joint rules (`M3.5.5`); the executable comes with
//! `M3.5.6`. Of the joint rules' module, [`joint`], one reading alone is public: [`joint::system`], a system's
//! hardware, the declarations it reaches and the requesters it serves as check (5) reads them, for a comparison test
//! another crate holds (`M3.5.5.7.1`). [`judgement`] makes one judgement the result record, which the executable writes
//! and [`mcp`]'s tool answers inline (`M3.5.1.6.4`). [`target`] is the checker's own reading of a target and of a system
//! put on it (`DC-H2`, `docs/decisions/decision_device-correspondence.md` rule 1; leaf `M4.1.7.5.4.2`), every refusal
//! the record's table names said by its code before any judgement. [`release`] is a system's release requester on a
//! target, the record's six lines as a side checks (3) to (5) read as any (`docs/decisions/decision_release-alarm.md`
//! §1; leaf `M4.1.7.5.4.3`), and [`check::plan_on_target`] judges a plan of a system on a target, `archogen-assignment/3`.

pub mod adapters;
pub mod assignment;
mod bindings;
pub mod check;
mod instances;
pub mod joint;
pub mod judgement;
pub mod mcp;
pub mod model;
pub mod providers;
pub mod release;
pub mod requesters;
pub mod target;

/// The executable's name: the result record's producer, and its MCP server's.
pub const NAME: &str = "archogen-plan-check";

/// The package's version, as its MCP server names itself; a test holds it to the manifest's, a value the trust gate
/// refuses to see compiled in through `env!`.
pub const VERSION: &str = "0.1.0";
