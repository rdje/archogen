//! The typed eADL model: declarations, units, contract identities, and profile definitions
//! (`ROADMAP.md` §4.2).
//!
//! Nothing here reads source text or chooses an implementation. This crate owns *what a
//! description means* once it has been read; `eadl-front` owns turning bytes into forms, and
//! the engine crates own deciding what to build from it.

pub mod profile;

pub use profile::{Exclusion, Profile, ProfileDecision, RT_STATIC_UP_V1};
