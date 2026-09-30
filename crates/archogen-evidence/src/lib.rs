//! The evidence, claim and trust vocabulary.
//!
//! `ROADMAP.md` §7.1 opens with a prohibition rather than a feature:
//!
//! > The report contains separate statuses for configuration validity, runtime functional
//! > behavior, timing, memory bounds, startup behavior, and any future isolation property.
//! > **One global "verified" flag is prohibited.**
//!
//! A prohibition written in a document is a prohibition someone will violate under deadline.
//! This crate encodes it three ways, so the violation has to be deliberate and visible:
//!
//! 1. **There is no aggregate verdict type.** A [`Report`] is a set of per-property claims and
//!    offers no method that collapses them. You cannot ask it whether "it" passed, because the
//!    question has no referent.
//! 2. **A report is incomplete unless every property is addressed.** [`Report::render`] refuses
//!    to produce output while any [`Property`] lacks a claim — including an explicit
//!    `NotApplicable` with a reason. Silence about a property is not an option.
//! 3. **Every conclusion carries its qualifier by construction.** A [`Conclusion`] cannot be
//!    built that says a property holds without naming *what makes it hold*: the constraint set,
//!    the model and its assumptions, the tested coverage, the refinement, or the target and
//!    binary identity. There is no unqualified variant to reach for.
//!
//! The categories and their permitted conclusions come straight from §7.1's table, and the
//! three sentences under it are the reason each one is shaped the way it is:
//!
//! > Testing does not become proof through repetition. A published algorithm proof does not
//! > automatically verify its Rust implementation. A declared capability does not constitute
//! > hardware evidence.

pub mod bound;
pub mod claim;
pub mod sha256;
pub mod trust;

pub use bound::{Bound, BoundDefect, BoundOrigin, SafetyFactor};
pub use claim::{Claim, Conclusion, EvidenceCategory, Property, Report, ReportError};
pub use trust::{TrustDrift, TrustItem, TrustRole, TrustRoot};
