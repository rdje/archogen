//! Scheduling analysis with explicit applicability conditions and arithmetic witnesses
//! (`ROADMAP.md` §4.2 `rt-analysis`, §7.4).
//!
//! One analysis lives here so far: the **idealized zero-overhead** response-time baseline of
//! §7.4. It is deliberately restricted, and the restriction is the point —
//!
//! > This baseline is for validating arithmetic and theorem implementation. **It must not be
//! > selected for a physical runtime whose nonzero overhead and jitter are omitted.**
//!
//! Two mechanisms keep that honest, and neither is a comment:
//!
//! 1. [`model::TaskSet::admit`] refuses a task set the model does not cover, so the analysis
//!    cannot return a number for a system it does not describe.
//! 2. [`response::conclusion`] can only express a positive answer as
//!    `Conclusion::HoldsUnderAssumptions`, which is unconstructible without a named model and a
//!    non-empty assumption list. The sentence "the deadlines are met", detached from the eight
//!    conditions that make it true, does not exist in this API.
//!
//! The runtime-applicable variant §7.4 requires, `fixed-priority-with-overheads/2`, is [`runtime`]
//! (leaf `M2.6`, F29 of §13.4 its control): bounded critical sections, release jitter, timer and
//! other interrupt interference, and context-switch costs, every input and fact declared or the set
//! refused. Four of its inputs are composite — `C_i`, `CS_i`, `J_i^release` and `J_s` — and [`compose`]
//! puts each together from catalog, application and plan parts, as
//! `docs/specs/catalog/decision_runtime-composite-inputs.md` says, so that the caller supplies only
//! the application's own figures and the conclusion names every part with its owner and category.
//!
//! ⚠️ Nothing here may yet be cited for a running system: the variant's inputs come from the
//! catalog, whose records do not exist yet (`M2.7.4.5`, `M2.7.5`), and every application part is a
//! separately supplied input that `M4` gives its evidence rules.

pub mod compose;
pub mod cost;
pub mod model;
pub mod response;
pub mod runtime;
pub mod trace;

pub use cost::{Accounting, Category, Contract, Interval, Ledger, LedgerError, COST_ACCOUNTING_V1};
pub use model::{Inadmissible, Task, TaskSet};
pub use response::{analyze, conclusion, response_time, Bound, Limit, Outcome, Witness};
