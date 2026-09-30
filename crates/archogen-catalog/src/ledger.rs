//! The checks §5 re-applies at each ledgering commit, and §9's retired ids (`M2.7.3.4.2`).
//!
//! > The loader also re-applies the ledger-time checks below at each ledgering commit, with that commit's record,
//! > parents' ledgers and committer date.
//!
//! The rules that need only the review's own file are the reader's. What is left needs the commit that ledgered the
//! review: its committer date, and the rejections the reviewed facet inherits there. [`crate::replay`] runs these at
//! each commit it replays, and [`crate::replay::check_history`] at every ledgering commit when a catalog loads.

use std::collections::{BTreeMap, BTreeSet};

use archogen_evidence::sha256::Digest;

use crate::history::CommitterDate;
use crate::lock::Line;
use crate::record::position;
use crate::refusal::{Code, Refusal};
use crate::replay::lock_at;
use crate::status::{items, review_at, Ledger, Reader, Route};

/// The calendar date of a committer date, read in the time-zone offset the commit records.
#[must_use]
pub fn local_date(date: CommitterDate) -> (i64, u32, u32) {
    let seconds = date.seconds + i64::from(date.offset_minutes) * 60;
    civil(seconds.div_euclid(86_400))
}

/// The proleptic Gregorian date `days` after 1970-01-01.
fn civil(days: i64) -> (i64, u32, u32) {
    let z = days + 719_468;
    let era = z.div_euclid(146_097);
    let doe = z.rem_euclid(146_097);
    let yoe = (doe - doe / 1_460 + doe / 36_524 - doe / 146_096) / 365;
    let doy = doe - (365 * yoe + yoe / 4 - yoe / 100);
    let mp = (5 * doy + 2) / 153;
    let day = doy - (153 * mp + 2) / 5 + 1;
    let month = if mp < 10 { mp + 3 } else { mp - 9 };
    let year = yoe + era * 400 + i64::from(month <= 2);
    (
        year,
        u32::try_from(month).unwrap_or_default(),
        u32::try_from(day).unwrap_or_default(),
    )
}

/// The checks §5 makes of review line `line` at commit `name`, which ledgered it:
///
/// - its date is no later than the commit's committer date, read in the offset the commit records;
/// - each hash its `answers` names is a rejection the reviewed facet inherits there, among the parents' ledgers and
///   the rejections the same bless ledgers;
/// - a rejection the facet inherits as its own record's or through its lineage is answered only once a parent's
///   ledger holds it; one it inherits only through items may be answered in the commit that ledgers it.
///
/// A commit where the review's form cannot be found is verification's to refuse, not these checks'.
///
/// # Errors
///
/// `catalog-review` for any of those, naming the commit; `catalog-layout` for a commit the history lacks; a refusal
/// met reading a commit's catalog or lock.
pub(crate) fn at_ledgering(
    reader: &mut Reader<'_>,
    name: &str,
    line: &Line,
    known: &[u64],
) -> Result<(), Refusal> {
    let Line::Review { id, ledger, .. } = line else {
        return Ok(());
    };
    let history = reader.history;
    let commit = history.get(name)?;
    let parents = commit.parents.clone();
    let today = local_date(commit.date);
    let answers_any = match review_at(reader.at(name)?, id, ledger) {
        Some((_, review)) => !review.answers.is_empty(),
        None => return Ok(()),
    };
    let ledger_there = if answers_any {
        Some(Ledger::at(reader, name, known)?)
    } else {
        None
    };
    let mut held: BTreeSet<Line> = BTreeSet::new();
    for parent in &parents {
        held.extend(
            lock_at(history, parent, known)?
                .into_iter()
                .flat_map(|l| l.lines),
        );
    }
    let at = reader.at(name)?;
    let Some((record, review)) = review_at(at, id, ledger) else {
        return Ok(());
    };
    let bytes = at.catalog.tree.get(&record.path).unwrap_or_default();
    let refuse = |message: String| {
        Refusal::new(
            Code::Review,
            &record.path,
            "review",
            position(&record.path, bytes, &review.form),
            format!("at commit {name}, which ledgers it: {message}"),
        )
    };
    let (y, m, d) = review.date;
    if (i64::from(y), m, d) > today {
        return Err(refuse(format!(
            "its date {y:04}-{m:02}-{d:02} is later than the commit's committer date, {}-{:02}-{:02} in its offset",
            today.0, today.1, today.2
        )));
    }
    let Some(ledger_there) = ledger_there else {
        return Ok(());
    };
    let now = items(at, record, review.facet, &ledger_there.versions);
    let inherited = ledger_there.inherited(id, review.facet, &now);
    for answer in &review.answers {
        let Some(digest) = Digest::parse(answer) else {
            continue;
        };
        let Some((x, route)) = inherited.iter().find(|(x, _)| x.ledger == digest) else {
            return Err(refuse(format!(
                "it answers {answer}, which is not a rejection its facet inherits there"
            )));
        };
        let in_a_parent = held.iter().any(
            |l| matches!(l, Line::Review { id: i, ledger: h, .. } if *i == x.id && *h == x.ledger),
        );
        if *route != Route::Items && !in_a_parent {
            return Err(refuse(format!(
                "it answers {answer}, a rejection its facet inherits as {}, in the bless that ledgers it: such a \
                 rejection is answered only once a parent's ledger holds it",
                if *route == Route::Own { "its own record's" } else { "its lineage's" }
            )));
        }
    }
    Ok(())
}

/// §9's checks of retired ids, and §11's rule on `supersedes`, at commit `head`:
///
/// - a `supersedes` id with a present record, or with no ledger line, is `catalog-dependency`;
/// - a retired id, one with ledger lines and no present record, whose rejection no production review of a record
///   superseding it answers, while no present record supersedes it;
/// - a present record whose id some record once superseded;
/// - a present record lacking a `supersedes` the lineage holds for it.
///
/// # Errors
///
/// `catalog-dependency` or `catalog-lock-retired` for any of those; `catalog-layout` for a commit the history lacks;
/// a refusal met reading a commit's catalog or lock.
pub(crate) fn retired(reader: &mut Reader<'_>, head: &str, known: &[u64]) -> Result<(), Refusal> {
    let ledger = Ledger::at(reader, head, known)?;
    let lines = lock_at(reader.history, head, known)?
        .map(|l| l.lines)
        .unwrap_or_default();
    let locked: BTreeSet<&str> = lines.iter().map(Line::id).collect();
    let at = reader.at(head)?;
    let present = &at.catalog.records;
    let refuse = |code: Code, path: &str, message: String| {
        Refusal::new(
            code,
            path,
            "contract supersedes",
            None,
            format!("at commit {head}: {message}"),
        )
    };
    for record in present.values() {
        for superseded in &record.contract.supersedes {
            if present.contains_key(superseded) {
                return Err(refuse(
                    Code::Dependency,
                    &record.path,
                    format!("it supersedes `{superseded}`, which has a present record"),
                ));
            }
            if !locked.contains(superseded.as_str()) {
                return Err(refuse(
                    Code::Dependency,
                    &record.path,
                    format!("it supersedes `{superseded}`, which has no ledger line"),
                ));
            }
        }
    }
    // Who supersedes whom, over every ancestor commit's records.
    let mut successors: BTreeMap<&str, BTreeSet<&str>> = BTreeMap::new();
    for (by, ids) in &ledger.edges {
        for id in ids {
            successors
                .entry(id.as_str())
                .or_default()
                .insert(by.as_str());
        }
    }
    let superseding = |id: &str| -> BTreeSet<&str> {
        let mut out = BTreeSet::new();
        let mut stack = vec![id];
        while let Some(current) = stack.pop() {
            for next in successors.get(current).into_iter().flatten() {
                if out.insert(*next) {
                    stack.push(next);
                }
            }
        }
        out
    };
    for record in present.values() {
        if let Some(by) = successors.get(record.id.as_str()) {
            return Err(refuse(
                Code::LockRetired,
                &record.path,
                format!(
                    "`{}` was superseded by `{}`: a superseded id is not taken again",
                    record.id,
                    by.iter().next().copied().unwrap_or_default()
                ),
            ));
        }
        let held: BTreeSet<&String> = record.contract.supersedes.iter().collect();
        if let Some(missing) = ledger
            .edges
            .get(&record.id)
            .into_iter()
            .flatten()
            .find(|s| !held.contains(s))
        {
            return Err(refuse(
                Code::LockRetired,
                &record.path,
                format!("the lineage holds `supersedes {missing}` for `{}`, and a lineage is never dropped", record.id),
            ));
        }
    }
    for rejection in &ledger.rejections {
        if present.contains_key(&rejection.id) {
            continue;
        }
        let by = superseding(&rejection.id);
        let superseded_now = by.iter().any(|s| present.contains_key(*s));
        let answered = ledger
            .productions
            .iter()
            .any(|p| by.contains(p.id.as_str()) && p.answers.contains(&rejection.ledger));
        if !superseded_now && !answered {
            return Err(refuse(
                Code::LockRetired,
                crate::lock::PATH,
                format!(
                    "retired `{}` has a rejection, {}, that no production review of a record superseding it answers, \
                     and no present record supersedes it",
                    rejection.id, rejection.ledger
                ),
            ));
        }
    }
    Ok(())
}
