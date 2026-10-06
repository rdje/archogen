//! Shared code for `eadl-resolve`'s integration tests: the bounded universe the model's checker enumerates, so the
//! production relation is tested against the model over the same one (`SR-H7`, leaf `M3.1.2.3`).
//!
//! ⚠️ `#![allow(dead_code)]` is deliberate, as in `crates/eadl-front/tests/common/mod.rs`: Cargo compiles every
//! integration test as its own crate with its own copy of this module, so an item `production.rs` uses and
//! `checker.rs` does not is dead code in the second, and this workspace runs clippy at `-D warnings`. The alternative
//! is two copies of the universe, which is the drift this module exists to prevent.

#![allow(dead_code)]

pub mod universe;
