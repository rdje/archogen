//! **The S0 prototype realization** — one fixed engine-owned path from an eADL description to a
//! running hosted artifact (`ROADMAP.md` §12 S0, fixture F28).
//!
//! ⛔ **EVERYTHING IN THIS CRATE IS TEMPORARY, AND THAT IS THE DESIGN.** §12 S0:
//!
//! > The prototype implementation can be discarded or replaced as semantics settle. Keep its
//! > functional fixtures and documented pipeline lessons. By M4, the supported path replaces any
//! > temporary hard-coded assumptions with checked engine plans; **no hidden special-case
//! > generator is grandfathered into the release.**
//!
//! The crate is deliberately *not* named `archogen-plan` or `archogen-emit`. Those are the §4.2
//! responsibility names M4 builds under, and a prototype squatting on them is precisely how a
//! special-case generator gets grandfathered — it stops looking like a prototype. Retiring S0 is
//! then one visible operation: delete this crate and watch what fails.
//!
//! # The path
//!
//! ```text
//! description ─► archogen check ─► interpret ─► emit ─► cargo build ─► run ─► observation
//!               (the real frontend)  (a Plan)   (a crate)                     (frozen in
//!                                                                              expected/)
//! ```
//!
//! Three modules, and the split between them is the §12 S0 boundary *"all templates, service
//! code, and dispatch choices stay inside the engine"*:
//!
//! * [`interpret`] — what the system must be observed to do, in the vocabulary of the published
//!   observation contract (`examples/s0-heartbeat/README.md`). It refuses in two families:
//!   inputs that describe no buildable system, and requests the S0 engine has no realization
//!   for — the second naming the missing capability and its owning leaf, as §5.4 requires.
//! * [`runtime`] — the engine-owned Rust that every generated crate contains, compiled and
//!   tested *here* and copied there byte for byte.
//! * [`emit`] — the manifest, the specialized table, and that copy. It generates no behavior.
//!
//! # What it is not
//!
//! Not a scheduler, not an analysis, not a claim. The events it produces are **releases**, which
//! follow from the description; when a task actually runs is a scheduling result and S0
//! establishes none. §12 S0: "It does not establish catalog reuse, sound scheduling, or
//! suitability for real hardware."
//!
//! The temporary assumptions are listed with their retiring leaves in
//! `examples/s0-heartbeat/README.md`, and leaf `S0.6` owns the retirement note.

pub mod emit;
pub mod interpret;
pub mod runtime;

pub use emit::{emit, Generated};
pub use interpret::{interpret, Plan, Task};
