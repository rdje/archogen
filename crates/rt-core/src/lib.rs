//! The shared runtime state machine (`ROADMAP.md` §4.2 `rt-core`, §8).
//!
//! ```text
//! #![no_std] outside its own tests — §3.1 fixes the runtime as a "Rust no_std core".
//! ```
//!
//! §8 states the design constraint before the feature list, and it is the one that shapes
//! everything here:
//!
//! > **Separate policy state transitions from the execution substrate.** A hosted harness can
//! > inject preemption at modeled boundaries; the architecture port must also validate real
//! > asynchronous interrupt entry, context preservation, and return.
//!
//! ⭐ **So this crate never switches a context.** It decides *that* a switch should happen and
//! hands back a [`Decision`]; performing it — saving registers, changing stacks, returning from
//! an interrupt — belongs to the architecture port, which is reviewed separately and cannot be
//! tested without a target. Splitting them that way is what makes the *policy* testable on a
//! host, which is `M2.1`'s acceptance, and it is also what stops a scheduler bug and a
//! context-save bug from being the same bug.
//!
//! # What is deliberately absent
//!
//! §8: *"Use the simplest bounded structures adequate for the profile. Do not add a generic
//! object manager, reference counting, or an asynchronous service framework without a workload
//! that needs them."*
//!
//! There is no heap, no `Vec`, no trait object, and no allocation of any kind: the task set is a
//! fixed-capacity array sized by a const parameter, because `rt-static-up-v1` admits a "finite
//! static task set" created at boot and excludes `dynamic-task-creation`. A task is a `usize`
//! index. If a later profile needs more, it needs a new structure and its own review — not a
//! generalization of this one.
//!
//! # Masking is modelled, not assumed away
//!
//! §8.1: *"Model synchronization and interrupt masking explicitly. Banning a mutex type does not
//! eliminate races or blocking."* So [`Scheduler::mask`] exists, a release arriving while masked
//! is **latched** rather than lost, and the latched set is delivered on unmask — which is the
//! same rule §13.4 states for its non-preemptible sections. A release that arrives while an
//! earlier one is still pending is an **overrun**, not a second pending bit: there is nowhere to
//! put it, and pretending otherwise is how a queue appears in a profile that excludes queues.

#![cfg_attr(not(test), no_std)]

pub mod fault;
pub mod scheduler;

pub use fault::{Fault, OverrunPolicy};
pub use scheduler::{BootError, Decision, Refused, Scheduler, TaskState, Transition};
