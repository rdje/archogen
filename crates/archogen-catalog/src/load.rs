//! Loading a catalog: every check, composed (`M2.7.3.6.1`).
//!
//! > A refusal to load is a defect in the engine's own knowledge, not a verdict on anyone's description.
//!
//! A catalog loads at a commit of a history when it reads, its hashes compute, its lock passes the checks over one
//! tree and over history, each name comes from one record under every selection, and its `production` namespace
//! holds. What a load yields is what a claim reads.

use crate::hash::{Catalog, Hashes};
use crate::history::History;
use crate::lock::{self, Lock, KNOWN_VERSIONS};
use crate::refusal::Refusal;
use crate::replay::check_history_under;
use crate::status::{statuses_under, Statuses};

/// A catalog that loaded, with what its checks computed.
#[derive(Debug, Clone)]
pub struct Loaded {
    /// The commit it was loaded at.
    pub commit: String,
    /// The catalog.
    pub catalog: Catalog,
    /// Every hash (§3).
    pub hashes: Hashes,
    /// The lock, when records exist.
    pub lock: Option<Lock>,
    /// Every facet's and record's status (§5).
    pub statuses: Statuses,
}

/// Load the catalog at commit `commit` of `history`, under this loader's rules versions.
///
/// # Errors
///
/// The first refusal of any check: the reader's, the hashes', the lock's over one tree and over history, the
/// selection's, and the `production` namespace's.
pub fn load(history: &History, commit: &str) -> Result<Loaded, Refusal> {
    let catalog = Catalog::read(history.get(commit)?.tree.clone())?;
    let hashes = catalog.hashes()?;
    let lock = lock::check_tree(&catalog, &hashes)?;
    catalog.check_selections()?;
    check_history_under(history, commit, &KNOWN_VERSIONS)?;
    let statuses = statuses_under(history, commit, &KNOWN_VERSIONS)?;
    Ok(Loaded {
        commit: commit.to_owned(),
        catalog,
        hashes,
        lock,
        statuses,
    })
}
