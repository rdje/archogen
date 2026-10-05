//! The relation's executable model: `docs/decisions/decision_substitutability-relation.md`, transcribed rule by
//! rule, each rule citing its section (leaf `M3.1.1.1`).
//!
//! ⭐ **A transcription, not an implementation.** It is written to be read beside the record: one function per rule,
//! in the record's order, optimised for nothing. Where the record leaves a choice, the model makes one and says so
//! in a comment that names the sentence, so a reviewer sees the reading the checker exercised. Its scope is the
//! record's: the relation at **one provider** — rule 1's five outcomes, §4's derivation, rule 3's judgement, and
//! §8's refusals of what an offer or a requirement writes. What a description makes of several providers is `M3.4`'s
//! and is not modelled.
//!
//! - [`vocab`] — `/1`, the record's §1.1 table, held by hand;
//! - [`value`] — the domains of §2: reading a value, equality in a domain, the comparisons, the horizon;
//! - [`provider`] — a block or platform read into one state per fact, refusing what §8 refuses of an offer;
//! - [`requirement`] — what `requires`, `needs` and `uses` write, refusing what §8 refuses of a requirement;
//! - [`judge`] — rule 1's outcome, rule 3's judgement and rule 5's clause.

pub mod judge;
pub mod provider;
pub mod requirement;
pub mod value;
pub mod vocab;

pub use judge::{clause_satisfied, judge, outcome, Outcome, Verdict};
pub use provider::{read as read_provider, Provider, Refused, Stated};
pub use requirement::{
    check_clause, read_clause, read_constraint, read_declaration_name, read_needs, read_side,
    read_uses, NotJudged, Requirement,
};
pub use value::{Overflow, Value};
pub use vocab::{Direction, Domain, Entry, Role, Rule, VOCABULARY};
