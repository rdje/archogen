//! The history the loader is given (the record's §4, `M2.7.3.3.2`).
//!
//! > Its caller gives it the tracked set and each file's bytes, and a **history**: for each commit the loader asks
//! > about, its parents, its committer date, and its tracked set and bytes.
//!
//! The loader never runs git. A test builds a history in memory; the repository's tooling builds one from git, the
//! gate's pending commit included, whose parents are `HEAD` and `MERGE_HEAD`. A commit the history lacks is refused
//! wherever it is asked about, since a missing parent would make a false ledgering commit of every line (§4).

use std::collections::{BTreeMap, BTreeSet};

use crate::refusal::{Code, Refusal};
use crate::tree::Tree;

/// When a commit was made, as git records it: seconds since the Unix epoch, and the committer's time-zone offset.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct CommitterDate {
    /// Seconds since `1970-01-01T00:00:00Z`.
    pub seconds: i64,
    /// The offset from UTC the commit records, in minutes, east positive.
    pub offset_minutes: i32,
}

/// One commit.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Commit {
    /// Its parents' object names, in order: none for a root, two for a merge.
    pub parents: Vec<String>,
    /// Its committer date.
    pub date: CommitterDate,
    /// Its tracked set, with each file's bytes.
    pub tree: Tree,
    /// The tooling's verdict that it is a merge commit the hosting made: committed as the hosting and signed with one
    /// of the hosting's keys the tooling holds for this commit's first-parent range (premise 3). The tooling checks
    /// the signature with a keyring it builds, never the user's own configuration.
    pub hosting: bool,
}

/// Every commit the loader may ask about, by object name.
#[derive(Debug, Clone, Default)]
pub struct History {
    commits: BTreeMap<String, Commit>,
}

impl History {
    /// A history of these commits.
    #[must_use]
    pub fn new(commits: impl IntoIterator<Item = (String, Commit)>) -> Self {
        Self {
            commits: commits.into_iter().collect(),
        }
    }

    /// Add or replace one commit.
    pub fn insert(&mut self, name: impl Into<String>, commit: Commit) {
        self.commits.insert(name.into(), commit);
    }

    /// A commit.
    ///
    /// # Errors
    ///
    /// `catalog-layout` when the history lacks it: a history cut short, as a shallow clone's is, would make a false
    /// ledgering commit of every line (§4).
    pub fn get(&self, name: &str) -> Result<&Commit, Refusal> {
        self.commits.get(name).ok_or_else(|| {
            Refusal::new(
                Code::Layout,
                "(history)",
                "(commit)",
                None,
                format!(
                    "the history lacks commit `{name}`: a history cut short would make a false ledgering commit"
                ),
            )
        })
    }

    /// `head` and every commit it descends from.
    ///
    /// # Errors
    ///
    /// `catalog-layout` for a commit the history lacks.
    pub fn ancestry(&self, head: &str) -> Result<BTreeSet<String>, Refusal> {
        let mut seen = BTreeSet::new();
        let mut stack = vec![head.to_owned()];
        while let Some(name) = stack.pop() {
            if seen.contains(&name) {
                continue;
            }
            let commit = self.get(&name)?;
            stack.extend(commit.parents.iter().cloned());
            seen.insert(name);
        }
        Ok(seen)
    }

    /// The commits `head` descends from, itself included, that no base descends from, oldest first: every commit
    /// after all of its parents, and ties in bytewise order of name, so the order is the same on every machine.
    ///
    /// # Errors
    ///
    /// `catalog-layout` for a commit the history lacks.
    pub fn between(&self, bases: &[&str], head: &str) -> Result<Vec<String>, Refusal> {
        let mut excluded = BTreeSet::new();
        for base in bases {
            excluded.extend(self.ancestry(base)?);
        }
        let range: BTreeSet<String> = self
            .ancestry(head)?
            .into_iter()
            .filter(|c| !excluded.contains(c))
            .collect();
        let mut out: Vec<String> = Vec::new();
        let mut placed: BTreeSet<String> = BTreeSet::new();
        while out.len() < range.len() {
            let next = range.iter().find(|c| {
                !placed.contains(*c)
                    && self.commits[c.as_str()]
                        .parents
                        .iter()
                        .all(|p| !range.contains(p) || placed.contains(p))
            });
            // A commit is its own ancestor only in a cycle, which git's object names cannot make; stop rather
            // than loop.
            let Some(next) = next.cloned() else { break };
            placed.insert(next.clone());
            out.push(next);
        }
        Ok(out)
    }
}
