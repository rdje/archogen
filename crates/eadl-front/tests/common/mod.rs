//! Shared code for `eadl-front`'s integration tests.
//!
//! ⚠️ `#![allow(dead_code)]` is deliberate, not laziness. Cargo compiles every integration test as
//! its own crate, and each `mod common;` gets a **separate copy** of this module — so an item that
//! `reference.rs` uses and `conformance.rs` does not is dead code in the second crate, and this
//! workspace runs clippy at `-D warnings`. The alternative is two copies of the reader for
//! `docs/semantics/reference.md`, which is the drift this module exists to prevent.

#![allow(dead_code)]

pub mod baseline;
pub mod reference_table;
pub mod suite;
