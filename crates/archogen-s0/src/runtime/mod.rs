//! The engine-owned Rust that S0 emits, verbatim, into every generated crate.
//!
//! Both modules here are compiled and tested as part of the engine *and* copied into the
//! generated output. `crate::emit` reads them with `include_str!`, and a test asserts the
//! emitted bytes equal these sources — so the code that was reviewed and the code that ships
//! cannot diverge.
//!
//! ⛔ Nothing in here may reference `crate::`. In the generated crate these are sibling modules
//! at the crate root, so `service` reaches its sibling as `super::rt`, which resolves in both
//! layouts.

pub mod rt;
pub mod service;
