//! The lock over history (the record's §9 and §5, `M2.7.3.3.2`).
//!
//! The lock is append-only, and every new line is recomputed commit by commit: the gate against the commit's
//! parents, CI against each of its bases, both through [`replay`]. A review line is verified at each commit that
//! ledgered it before anything is read from it, and a waiver stands only as §9 says: [`check_history`] makes both
//! checks when a catalog loads.

use std::collections::{BTreeMap, BTreeSet};

use crate::hash::{review_ledger_hash, Catalog};
use crate::history::History;
use crate::ledger;
use crate::lock::{self, blessed, Line, Lock, KNOWN_VERSIONS};
use crate::refusal::{At, Code, Refusal};
use crate::status::Reader;

/// A refusal about the lock at commit `name`, at line `line` of it when there is one.
fn refuse(code: Code, name: &str, line: Option<usize>, message: impl Into<String>) -> Refusal {
    Refusal::new(
        code,
        lock::PATH,
        "(lock)",
        line.map(|n| At {
            line: u32::try_from(n).unwrap_or(u32::MAX),
            column: 1,
        }),
        format!("at commit {name}: {}", message.into()),
    )
}

/// A refusal met reading commit `name`, with the commit named.
fn at_commit(name: &str, mut refusal: Refusal) -> Refusal {
    refusal.message = format!("at commit {name}: {}", refusal.message);
    refusal
}

/// The line number of `line` in `lock`: the first line is 1.
fn number(lock: &Lock, line: &Line) -> Option<usize> {
    lock.lines.iter().position(|l| l == line).map(|i| i + 2)
}

/// The lock commit `name` holds, read under the rules versions `known`; `None` when it holds none.
///
/// # Errors
///
/// `catalog-layout` for a commit the history lacks; [`Lock::parse_under`]'s refusals.
pub fn lock_at(history: &History, name: &str, known: &[u64]) -> Result<Option<Lock>, Refusal> {
    history
        .get(name)?
        .tree
        .get(lock::PATH)
        .map(|bytes| Lock::parse_under(bytes, known).map_err(|r| at_commit(name, r)))
        .transpose()
}

/// §9's append-only comparison of commit `name`'s lock with its parent `parent`'s: every line the parent holds is
/// held unchanged, and the first line names a version no lower.
///
/// # Errors
///
/// `catalog-lock-review` for a lock gone, a parent's line dropped or altered, or a lower version.
pub fn compare(
    lock: Option<&Lock>,
    parent_lock: Option<&Lock>,
    name: &str,
    parent: &str,
) -> Result<(), Refusal> {
    let Some(parent_lock) = parent_lock else {
        return Ok(());
    };
    let Some(lock) = lock else {
        return Err(refuse(
            Code::LockReview,
            name,
            None,
            format!("there is no lock, and parent {parent} holds one: a line, once committed, is never removed"),
        ));
    };
    if lock.version < parent_lock.version {
        return Err(refuse(
            Code::LockReview,
            name,
            Some(1),
            format!(
                "the lock names rules version {}, lower than parent {parent}'s {}",
                lock.version, parent_lock.version
            ),
        ));
    }
    let held: BTreeSet<&Line> = lock.lines.iter().collect();
    if let Some(line) = parent_lock.lines.iter().find(|l| !held.contains(l)) {
        return Err(refuse(
            Code::LockReview,
            name,
            None,
            format!(
                "parent {parent} holds `{}`, which this lock dropped or altered: a line, once committed, is never \
                 changed or removed",
                line.render()
            ),
        ));
    }
    Ok(())
}

/// Why review line `line` fails §5's verification at commit `name`, or `None` when it passes there: the review's
/// form in its record hashes to the line's ledger hash, the line's facet and verdict are the form's, and the facet's
/// bound hash is the one the line names. A commit whose catalog does not read or hash verifies nothing.
///
/// # Errors
///
/// `catalog-layout` for a commit the history lacks.
pub fn verification(history: &History, name: &str, line: &Line) -> Result<Option<String>, Refusal> {
    let Line::Review {
        id,
        ledger,
        facet,
        verdict,
        bound,
    } = line
    else {
        return Ok(None);
    };
    let commit = history.get(name)?;
    let catalog = match Catalog::read(commit.tree.clone()) {
        Ok(catalog) => catalog,
        Err(r) => return Ok(Some(format!("its catalog does not read: {r}"))),
    };
    let Some(record) = catalog.records.get(id) else {
        return Ok(Some(format!("it holds no record `{id}`")));
    };
    let Some(review) = record
        .reviews
        .iter()
        .find(|r| review_ledger_hash(id, r) == *ledger)
    else {
        return Ok(Some(format!("no review of `{id}` hashes to {ledger}")));
    };
    if review.facet != *facet || review.verdict != *verdict {
        return Ok(Some(format!(
            "the review's form gives {} {}, and the line {} {}",
            review.facet.as_str(),
            review.verdict.as_str(),
            facet.as_str(),
            verdict.as_str()
        )));
    }
    let hashes = match catalog.hashes() {
        Ok(hashes) => hashes,
        Err(r) => return Ok(Some(format!("its hashes do not compute: {r}"))),
    };
    let actual = hashes.facet(id, *facet).map(|h| h.bound);
    if actual != Some(*bound) {
        return Ok(Some(format!(
            "the facet's bound hash there is {}, and the line names {bound}",
            actual.map_or_else(|| "none".to_owned(), |d| d.to_string())
        )));
    }
    Ok(None)
}

/// Every lock among `commits`, as sets of lines, an absent lock holding none.
pub(crate) fn locks(
    history: &History,
    commits: &BTreeSet<String>,
    known: &[u64],
) -> Result<BTreeMap<String, BTreeSet<Line>>, Refusal> {
    let mut out = BTreeMap::new();
    for name in commits {
        let lines = lock_at(history, name, known)?
            .map_or_else(BTreeSet::new, |l| l.lines.into_iter().collect());
        out.insert(name.clone(), lines);
    }
    Ok(out)
}

/// The commits among `locks` that ledgered `line` (§5): each holds it, and none of its parents does.
pub(crate) fn ledgering(
    history: &History,
    locks: &BTreeMap<String, BTreeSet<Line>>,
    line: &Line,
) -> Result<Vec<String>, Refusal> {
    let mut out = Vec::new();
    for (name, lines) in locks {
        if !lines.contains(line) {
            continue;
        }
        let parents = &history.get(name)?.parents;
        if parents
            .iter()
            .all(|p| locks.get(p).is_none_or(|l| !l.contains(line)))
        {
            out.push(name.clone());
        }
    }
    Ok(out)
}

/// Whether review line `(id, ledger)` is ledgered by some line of `lines`.
pub(crate) fn ledgers(
    lines: &BTreeSet<Line>,
    id: &str,
    ledger: &archogen_evidence::sha256::Digest,
) -> Option<Line> {
    lines
        .iter()
        .find(|l| matches!(l, Line::Review { id: i, ledger: h, .. } if i == id && h == ledger))
        .cloned()
}

/// A waiver `waiver` in commit `name`'s lock, judged as §9 accepts one: the review line it names is in `held`, the
/// lines of the commit's parents' locks; the commit it names ledgered that line; and the line fails verification
/// there.
fn judge_waiver(
    history: &History,
    name: &str,
    lock: &Lock,
    waiver: &Line,
    held: &BTreeSet<Line>,
    known: &[u64],
) -> Result<(), Refusal> {
    let Line::Waiver { id, ledger, commit } = waiver else {
        return Ok(());
    };
    let at = number(lock, waiver);
    let Some(review) = ledgers(held, id, ledger) else {
        return Err(refuse(
            Code::LockReview,
            name,
            at,
            format!("the waiver names `{id}`'s review {ledger}, which no parent's lock ledgers"),
        ));
    };
    let ancestry = history.ancestry(name)?;
    let named_ledgered = ancestry.contains(commit) && {
        let mut around: BTreeSet<String> = history.get(commit)?.parents.iter().cloned().collect();
        around.insert(commit.clone());
        ledgering(history, &locks(history, &around, known)?, &review)?.contains(commit)
    };
    if !named_ledgered {
        return Err(refuse(
            Code::LockReview,
            name,
            at,
            format!(
                "the waiver names commit {commit}, which did not ledger `{}`",
                review.render()
            ),
        ));
    }
    if verification(history, commit, &review)?.is_none() {
        return Err(refuse(
            Code::LockReview,
            name,
            at,
            format!(
                "the waiver names `{}`, which verifies at {commit}: there is nothing to repair",
                review.render()
            ),
        ));
    }
    Ok(())
}

/// Replay every commit from `bases` to `head`, oldest first (§9), under this loader's rules versions.
///
/// # Errors
///
/// As [`replay_under`].
pub fn replay(history: &History, bases: &[&str], head: &str) -> Result<(), Refusal> {
    replay_under(history, bases, head, &KNOWN_VERSIONS)
}

/// Replay every commit `head` descends from, itself included, that no base descends from, oldest first, reading
/// locks under the rules versions `known`. At each commit:
///
/// - its catalog reads, its hashes compute, and its lock passes the checks over one tree;
/// - its lock holds every line of every parent's lock, under a version no lower (§9's append-only comparison);
/// - its new lines, those no parent's lock holds, are the lines blessing writes from its own tree, apart from
///   waivers, and none is under a version lower than the bases';
/// - each new review line verifies there, the commit being the one that ledgered it (§5);
/// - each new waiver names a review line a parent's lock holds, a commit that ledgered it, where it fails.
///
/// The gate's replay has the commit being made as `head` and its parents as `bases`.
///
/// # Errors
///
/// `catalog-lock-review` for any of those; the one-tree checks' codes; `catalog-layout` for a commit the history
/// lacks. Each names the commit.
pub fn replay_under(
    history: &History,
    bases: &[&str],
    head: &str,
    known: &[u64],
) -> Result<(), Refusal> {
    let mut floor = None;
    for base in bases {
        if let Some(lock) = lock_at(history, base, known)? {
            floor = floor.max(Some(lock.version));
        }
    }
    let mut reader = Reader::new(history);
    for name in history.between(bases, head)? {
        let commit = history.get(&name)?;
        let catalog = Catalog::read(commit.tree.clone()).map_err(|r| at_commit(&name, r))?;
        let hashes = catalog.hashes().map_err(|r| at_commit(&name, r))?;
        let lock = lock_at(history, &name, known)?;
        match &lock {
            Some(lock) => lock::check(lock, &catalog, &hashes),
            None => lock::check_tree(&catalog, &hashes).map(|_| ()),
        }
        .map_err(|r| at_commit(&name, r))?;
        let mut held: BTreeSet<Line> = BTreeSet::new();
        for parent in &commit.parents {
            let parent_lock = lock_at(history, parent, known)?;
            compare(lock.as_ref(), parent_lock.as_ref(), &name, parent)?;
            held.extend(parent_lock.into_iter().flat_map(|l| l.lines));
        }
        let Some(lock) = lock else {
            continue;
        };
        let new: Vec<&Line> = lock.lines.iter().filter(|l| !held.contains(l)).collect();
        if let (Some(floor), Some(first)) = (floor, new.first()) {
            if lock.version < floor {
                return Err(refuse(
                    Code::LockReview,
                    &name,
                    number(&lock, first),
                    format!(
                        "`{}` is new under rules version {}, lower than the base's {floor}: a branch forked before a \
                         bump cannot go on ledgering under the older rules",
                        first.render(),
                        lock.version
                    ),
                ));
            }
        }
        let written = blessed(&catalog, &hashes);
        for line in new {
            if let Line::Waiver { .. } = line {
                judge_waiver(history, &name, &lock, line, &held, known)?;
                continue;
            }
            if !written.contains(line) {
                return Err(refuse(
                    Code::LockReview,
                    &name,
                    number(&lock, line),
                    format!(
                        "`{}` is new, and blessing would not write it from this commit's tree",
                        line.render()
                    ),
                ));
            }
            if let Some(why) = verification(history, &name, line)? {
                return Err(refuse(
                    Code::LockReview,
                    &name,
                    number(&lock, line),
                    format!(
                        "`{}` does not verify where it is ledgered: {why}",
                        line.render()
                    ),
                ));
            }
            ledger::at_ledgering(&mut reader, &name, line, known)?;
        }
        ledger::retired(&mut reader, &name, known)?;
    }
    Ok(())
}

/// The load checks that need history, under this loader's rules versions.
///
/// # Errors
///
/// As [`check_history_under`].
pub fn check_history(history: &History, head: &str) -> Result<(), Refusal> {
    check_history_under(history, head, &KNOWN_VERSIONS)
}

/// The load checks of §9 and §5 that need history, for the lock at `head`: each review line verified at every
/// commit in `head`'s ancestry that ledgered it, unless a waiver names it, and each waiver naming a review line of
/// the lock, a commit that ledgered it, and a failure there.
///
/// # Errors
///
/// `catalog-lock-review` for a review line that fails verification where it was ledgered, or a waiver that does not
/// stand; `catalog-layout` for a commit the history lacks.
pub fn check_history_under(history: &History, head: &str, known: &[u64]) -> Result<(), Refusal> {
    let Some(lock) = lock_at(history, head, known)? else {
        return Ok(());
    };
    let ancestry = history.ancestry(head)?;
    let all = locks(history, &ancestry, known)?;
    let lines: BTreeSet<Line> = lock.lines.iter().cloned().collect();
    let mut reader = Reader::new(history);
    let mut waived: BTreeMap<(&str, _), Vec<&Line>> = BTreeMap::new();
    for line in &lock.lines {
        if let Line::Waiver { id, ledger, .. } = line {
            waived.entry((id.as_str(), *ledger)).or_default().push(line);
        }
    }
    for line in &lock.lines {
        let Line::Review { id, ledger, .. } = line else {
            continue;
        };
        let at = ledgering(history, &all, line)?;
        if let Some(waivers) = waived.get(&(id.as_str(), *ledger)) {
            for waiver in waivers {
                let Line::Waiver { commit, .. } = waiver else {
                    continue;
                };
                if !at.contains(commit) {
                    return Err(refuse(
                        Code::LockReview,
                        head,
                        number(&lock, waiver),
                        format!(
                            "the waiver names commit {commit}, which did not ledger `{}`",
                            line.render()
                        ),
                    ));
                }
                if verification(history, commit, line)?.is_none() {
                    return Err(refuse(
                        Code::LockReview,
                        head,
                        number(&lock, waiver),
                        format!(
                            "the waiver names `{}`, which verifies at {commit}: there is nothing to repair",
                            line.render()
                        ),
                    ));
                }
            }
            continue;
        }
        for commit in &at {
            if let Some(why) = verification(history, commit, line)? {
                return Err(refuse(
                    Code::LockReview,
                    head,
                    number(&lock, line),
                    format!(
                        "`{}` does not verify at {commit}, which ledgered it: {why}",
                        line.render()
                    ),
                ));
            }
            ledger::at_ledgering(&mut reader, commit, line, known)?;
        }
    }
    for (key, waivers) in &waived {
        if ledgers(&lines, key.0, &key.1).is_none() {
            return Err(refuse(
                Code::LockReview,
                head,
                waivers.first().and_then(|w| number(&lock, w)),
                format!(
                    "the waiver names `{}`'s review {}, which no line of the lock ledgers",
                    key.0, key.1
                ),
            ));
        }
    }
    ledger::retired(&mut reader, head, known)
}
