//! The eADL frontend: reader, source spans, and source-localized diagnostics
//! (`ROADMAP.md` §4.2 `eadl-front`).
//!
//! This crate turns bytes into a [`Document`] of [`Form`]s, and turns anything it cannot read
//! into a [`Diagnostic`] that says where and what to do. It does **not** know what a
//! declaration means: kinds, schemas, units and refinement all live above it. That separation
//! is what lets one reader serve the boundary corpus, the S0 fixture and the M1 semantic corpus
//! without any of them leaking assumptions into the others.
//!
//! Two properties are load-bearing and tested rather than assumed:
//!
//! * **Numbers are exact.** There is no float anywhere. §7.4 requires exact integer or checked
//!   rational arithmetic, and a reader that produces `f64` loses that before any analysis runs.
//! * **Comments survive.** They are semantically inert to the language and are still kept with
//!   their spans, because the boundary corpus carries its case metadata in them. Discarding
//!   them would force a second, divergent parser to exist just to read those headers.

pub mod diagnostic;
pub mod form;
pub mod module;
pub mod reader;
pub mod source;

pub use diagnostic::{Diagnostic, Diagnostics, Label, Severity, Verdict};
pub use form::{Comment, Document, Form};
pub use module::{elaborate, Instance, MemoryModules, ModuleDecl, ModuleSource, Program, Version};
pub use reader::read;
pub use source::{Position, Source, SourceError, SourceId, SourceMap, Span};
