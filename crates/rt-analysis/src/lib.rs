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
//! ⚠️ What is still owed, and is **not** here: §7.4 requires a runtime-applicable variant that
//! accounts for bounded critical sections, release jitter, timer and other interrupt
//! interference, and context-switch costs, before any `rt-static-up-v1` timing result may be
//! accepted. That is leaf `M2.6`, and F29 (§13.4) is its control. Nothing in this crate may be
//! cited for a runtime claim until it lands.

pub mod cost;
pub mod model;
pub mod response;
pub mod runtime;
pub mod trace;

pub use cost::{Accounting, Category, Contract, Interval, Ledger, LedgerError, COST_ACCOUNTING_V1};
pub use model::{Inadmissible, Task, TaskSet};
pub use response::{analyze, conclusion, response_time, Bound, Limit, Outcome, Witness};
