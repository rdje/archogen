//! The typed eADL model: declarations, units, contract identities, and profile definitions
//! (`ROADMAP.md` §4.2).
//!
//! Nothing here reads source text or chooses an implementation. This crate owns *what a
//! description means* once it has been read; `eadl-front` owns turning bytes into forms, and
//! the engine crates own deciding what to build from it.

pub mod boundary;
pub mod check;
pub mod kind;
pub mod presence;
pub mod profile;
pub mod quantity;
pub mod rational;
pub mod refinement;

pub use boundary::{classify, BoundaryTest, Classification, ForbiddenConstruct};
pub use check::{default_profile, shipped_registry, Outcome};
pub use kind::{
    read_kind, validate, Cardinality, ClauseDef, Holds, KindDef, NameRule, Registry, ValueType,
};
pub use presence::{FactMap, Presence, PresenceReport};
pub use profile::{Exclusion, Profile, ProfileDecision, RT_STATIC_UP_V1};
pub use quantity::{Dimension, Quantity, QuantityError, Unit};
pub use rational::Rational;
pub use refinement::{Facets, Obligation, RefinementReport};
