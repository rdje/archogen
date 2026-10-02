//! The catalog of engine knowledge (`ROADMAP.md` §9), as `docs/specs/catalog/decision_catalog-records.md`
//! decides it.
//!
//! A catalog record is a tracked ASCII file under `catalog/`, read by the eADL reader's datum layer alone. It has
//! four facets, a contract, an implementation, a behavioral model and a timing model, each with its own version.
//! This crate reads records and refuses what breaks the record's rules, each refusal with its one code (§11).
//!
//! It is **I/O-free**. Its caller hands it every byte: the tracked set, each file's contents and, for what needs
//! history, the commits. That is what lets every rule be a test built in memory, and what keeps the product free of
//! git and of processes (`NO-SUBPROCESS`); the repository's own tooling, under `xtask/`, is the caller that runs git.
//!
//! Built so far: [`record::read_record`], which reads one record and checks every rule of §1 and §2 that needs
//! nothing but the file (`M2.7.3.1`), and those of §12 and §13 its text shows (`M2.7.3.5.1`); [`hash::Catalog`], which reads a tree's records and computes §3's hashes
//! (`M2.7.3.2`); [`lock`], which reads §9's lock and checks it against one tree's records (`M2.7.3.3.1`); and
//! [`history`] and [`replay`], which check the lock over history: append-only, every new line recomputed commit by
//! commit, each review verified where it was ledgered, and each waiver judged (`M2.7.3.3.2`); and [`status`], which
//! derives §5's evidence status of every facet and record (`M2.7.3.4.1`); and [`ledger`], the checks §5 re-applies
//! at each ledgering commit and §9's retired ids, which the replay and the load run (`M2.7.3.4.2`); and
//! [`invalidation`], which judges a claim's recorded closure and reads against the catalog now (`M2.7.3.4.3`); and
//! [`selection`], §12's lookups, conflicts and groups (`M2.7.3.5.2`); [`production`], §6's namespace rules
//! (`M2.7.3.5.3`); [`load`] and [`claim`], every check composed, and §7's claims, premise 3 among them (`M2.7.3.6`); and [`package`],
//! §3's package rules and token scan (`M2.7.3.7`). §14.2 amends the record's form (`M2.12.4.1`): a code fact's
//! several `code` locators, the [`record::Assembly`] declaration and its [`dialect`], and the rules over declared
//! assembly that [`hash::Catalog::hashes`] checks once every set is known.

pub mod claim;
pub mod dialect;
pub mod grammar;
pub mod hash;
pub mod history;
pub mod invalidation;
pub mod ledger;
pub mod load;
pub mod lock;
pub mod manifest;
pub mod package;
pub mod production;
pub mod record;
pub mod refusal;
pub mod replay;
pub mod selection;
pub mod status;
pub mod tree;

pub use record::{classify, read_record, CatalogPath, Namespace, Record};
pub use refusal::{At, Code, Refusal};
