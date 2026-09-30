//! The lock over one tree (the record's §9, `M2.7.3.3.1`).
//!
//! `catalog/catalog.lock` holds a first line naming the rules version, then one line per facet version ever blessed,
//! one per review ever ledgered and one per waiver, sorted. This module reads it and writes it back byte for byte,
//! computes the lines blessing writes for a catalog, and makes the load checks that need nothing but the one tree.
//! What needs history, the comparison with each parent's lock and the replay of each commit's new lines, is
//! `M2.7.3.3.2`'s.

use std::cmp::Ordering;
use std::collections::{BTreeMap, BTreeSet};

use archogen_evidence::sha256::Digest;
use eadl_front::Form;

use crate::grammar::{self, Version};
use crate::hash::{review_ledger_hash, Catalog, Hashes};
use crate::record::{position, FacetKind, Record, Verdict};
use crate::refusal::{At, Code, Refusal};

/// Where the lock lives (§1).
pub const PATH: &str = "catalog/catalog.lock";

/// The rules versions this loader knows, by the number after `archogen-catalog/` (§5).
pub const KNOWN_VERSIONS: [u64; 1] = [1];

/// One line of the lock after its first.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Line {
    /// `<id> <facet> <version> sha256:<own hash>`: a facet version once blessed.
    Facet {
        /// The record's id.
        id: String,
        /// The facet.
        facet: FacetKind,
        /// Its version.
        version: Version,
        /// Its own hash at that version.
        own: Digest,
    },
    /// `<id> review sha256:<ledger hash> <facet> <verdict> sha256:<bound hash>`: a review once ledgered.
    Review {
        /// The record's id.
        id: String,
        /// The review's ledger hash (§3).
        ledger: Digest,
        /// The facet it reviews.
        facet: FacetKind,
        /// Its verdict.
        verdict: Verdict,
        /// The bound hash it names.
        bound: Digest,
    },
    /// `<id> waiver sha256:<ledger hash> <commit>`: a review line waived, with the commit that ledgered it.
    Waiver {
        /// The record's id.
        id: String,
        /// The ledger hash of the review line it names.
        ledger: Digest,
        /// The commit that ledgered that line, by its full object name in lowercase hex.
        commit: String,
    },
}

impl Line {
    /// The record's id the line is about.
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Facet { id, .. } | Self::Review { id, .. } | Self::Waiver { id, .. } => id,
        }
    }

    /// The kind's place in the order: the four facets as §2 lists them, then reviews, then waivers.
    fn rank(&self) -> usize {
        match self {
            Self::Facet { facet, .. } => FacetKind::ALL
                .iter()
                .position(|f| f == facet)
                .unwrap_or_default(),
            Self::Review { .. } => 4,
            Self::Waiver { .. } => 5,
        }
    }

    /// The line as the lock writes it, without its line feed.
    #[must_use]
    pub fn render(&self) -> String {
        match self {
            Self::Facet {
                id,
                facet,
                version,
                own,
            } => format!("{id} {} {version} {own}", facet.as_str()),
            Self::Review {
                id,
                ledger,
                facet,
                verdict,
                bound,
            } => format!(
                "{id} review {ledger} {} {} {bound}",
                facet.as_str(),
                verdict.as_str()
            ),
            Self::Waiver { id, ledger, commit } => format!("{id} waiver {ledger} {commit}"),
        }
    }

    /// A line in one of the three forms, or `None`.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let fields: Vec<&str> = text.split(' ').collect();
        let id = (*fields.first()?).to_owned();
        if !grammar::is_id(&id) {
            return None;
        }
        match fields.as_slice() {
            [_, "review", ledger, facet, verdict, bound] => Some(Self::Review {
                id,
                ledger: Digest::parse(ledger)?,
                facet: FacetKind::parse(facet)?,
                verdict: Verdict::parse(verdict)?,
                bound: Digest::parse(bound)?,
            }),
            [_, "waiver", ledger, commit] if is_object_name(commit) => Some(Self::Waiver {
                id,
                ledger: Digest::parse(ledger)?,
                commit: (*commit).to_owned(),
            }),
            [_, facet, version, own] => Some(Self::Facet {
                id,
                facet: FacetKind::parse(facet)?,
                version: grammar::version(version)?,
                own: Digest::parse(own)?,
            }),
            _ => None,
        }
    }
}

/// A commit's full object name in lowercase hex: 40 digits, or 64 in a SHA-256 repository.
fn is_object_name(text: &str) -> bool {
    (text.len() == 40 || text.len() == 64)
        && text
            .bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}

impl Ord for Line {
    /// §9's order: bytewise by id, then by kind, then versions in semantic-version order, and reviews and waivers
    /// in bytewise order of their line. A facet line's own hash breaks the last tie, so the order agrees with
    /// equality; two lines that differ only there are never both in order, since neither precedes the other in the
    /// lock's strict order of versions.
    fn cmp(&self, other: &Self) -> Ordering {
        self.id()
            .as_bytes()
            .cmp(other.id().as_bytes())
            .then_with(|| self.rank().cmp(&other.rank()))
            .then_with(|| match (self, other) {
                (
                    Self::Facet {
                        version: a, own: x, ..
                    },
                    Self::Facet {
                        version: b, own: y, ..
                    },
                ) => a.cmp(b).then_with(|| x.cmp(y)),
                _ => self.render().as_bytes().cmp(other.render().as_bytes()),
            })
    }
}

impl PartialOrd for Line {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// Whether two lines break §9's strict order: the second is not after the first, or both are one facet version.
fn out_of_order(before: &Line, after: &Line) -> bool {
    let same_version = matches!(
        (before, after),
        (Line::Facet { id: a, facet: f, version: v, .. }, Line::Facet { id: b, facet: g, version: w, .. })
            if a == b && f == g && v == w
    );
    same_version || before >= after
}

/// The lock as read.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lock {
    /// The rules version its first line names, the number after `archogen-catalog/`.
    pub version: u64,
    /// Every line after the first, in order.
    pub lines: Vec<Line>,
}

impl Lock {
    /// A lock under the rules version `version`, with these lines put in order.
    #[must_use]
    pub fn new(version: u64, lines: impl IntoIterator<Item = Line>) -> Self {
        let set: BTreeSet<Line> = lines.into_iter().collect();
        Self {
            version,
            lines: set.into_iter().collect(),
        }
    }

    /// Read a lock's bytes (§9).
    ///
    /// # Errors
    ///
    /// `catalog-lock-review` for a first line missing, malformed or naming a version this loader does not know; a
    /// line that matches none of the lock's forms, an empty one or a last one without its line feed included; a line
    /// out of order, a repeated one included; and a second line for one review, sorted beside the first.
    pub fn parse(bytes: &[u8]) -> Result<Self, Refusal> {
        Self::parse_under(bytes, &KNOWN_VERSIONS)
    }

    /// Read a lock's bytes as a loader that knows the rules versions `known` would: what a later loader, which knows
    /// more than one, sees, and what a test of §9's never-decreasing first line needs, since this loader knows one.
    ///
    /// # Errors
    ///
    /// As [`Lock::parse`].
    pub fn parse_under(bytes: &[u8], known: &[u64]) -> Result<Self, Refusal> {
        let refuse = |line: usize, message: String| {
            Refusal::new(
                Code::LockReview,
                PATH,
                "(lock)",
                Some(At {
                    line: u32::try_from(line).unwrap_or(u32::MAX),
                    column: 1,
                }),
                message,
            )
        };
        if let Some(offset) = bytes
            .iter()
            .position(|&b| !((0x20..=0x7e).contains(&b) || b == b'\n'))
        {
            let line = bytes[..offset].iter().filter(|&&b| b == b'\n').count() + 1;
            return Err(refuse(
                line,
                format!(
                    "byte 0x{:02x} is in none of the lock's forms",
                    bytes[offset]
                ),
            ));
        }
        let text = core::str::from_utf8(bytes).unwrap_or_default();
        if text.is_empty() {
            return Err(refuse(
                1,
                "the first line, naming the rules version, is missing: nothing is verified under no version"
                    .to_owned(),
            ));
        }
        let Some(body) = text.strip_suffix('\n') else {
            return Err(refuse(
                text.split('\n').count(),
                "the last line has no line feed".to_owned(),
            ));
        };
        let mut rows = body.split('\n');
        let first = rows.next().unwrap_or_default();
        let version = first_line(first).ok_or_else(|| {
            refuse(
                1,
                format!("`{first}` is not `# archogen-catalog/<version>`: nothing is verified under no version"),
            )
        })?;
        if !known.contains(&version) {
            return Err(refuse(
                1,
                format!("rules version `archogen-catalog/{version}` is not one this loader knows"),
            ));
        }
        let mut lines: Vec<Line> = Vec::new();
        for (index, row) in rows.enumerate() {
            let number = index + 2;
            let line = Line::parse(row).ok_or_else(|| {
                refuse(number, format!("`{row}` matches none of the lock's forms"))
            })?;
            if let Some(before) = lines.last() {
                if out_of_order(before, &line) {
                    return Err(refuse(
                        number,
                        format!("`{row}` is out of order after `{}`", before.render()),
                    ));
                }
                if let (
                    Line::Review {
                        id: a, ledger: x, ..
                    },
                    Line::Review {
                        id: b, ledger: y, ..
                    },
                ) = (before, &line)
                {
                    if a == b && x == y {
                        return Err(refuse(
                            number,
                            format!("`{row}` ledgers review {y} a second time: there is one line per review"),
                        ));
                    }
                }
            }
            lines.push(line);
        }
        Ok(Self { version, lines })
    }

    /// The lock's bytes, as blessing writes them.
    #[must_use]
    pub fn render(&self) -> String {
        let mut out = format!("# archogen-catalog/{}\n", self.version);
        for line in &self.lines {
            out.push_str(&line.render());
            out.push('\n');
        }
        out
    }

    /// The line number of `lines[index]` in the file: the first line is 1.
    fn number(index: usize) -> usize {
        index + 2
    }
}

/// The version a first line names: `# archogen-catalog/`, then a decimal number without a leading zero.
fn first_line(text: &str) -> Option<u64> {
    let digits = text.strip_prefix("# archogen-catalog/")?;
    // An empty number fails the parse below, and a sign passes it, so the digits are checked first.
    if !digits.bytes().all(|b| b.is_ascii_digit()) || (digits.len() > 1 && digits.starts_with('0'))
    {
        return None;
    }
    digits.parse().ok()
}

/// A facet's current version (§2, §9): the contract's is the record's own `version`.
#[must_use]
pub fn current_version(record: &Record, facet: FacetKind) -> Version {
    match facet {
        FacetKind::Contract => record.contract.version,
        FacetKind::Implementation => record.implementation.version,
        FacetKind::BehaviorModel => record.behavior_model.version,
        FacetKind::TimingModel => record.timing_model.version,
    }
}

/// The lines blessing writes for a catalog's present records (§9): each facet at its current version with its own
/// hash, and each review with its ledger hash and its form's facet, verdict and hash. Blessing keeps every line a
/// parent holds besides; which lines those are is history's (`M2.7.3.3.2`).
#[must_use]
pub fn blessed(catalog: &Catalog, hashes: &Hashes) -> BTreeSet<Line> {
    let mut out = BTreeSet::new();
    for (id, record) in &catalog.records {
        for facet in FacetKind::ALL {
            if let Some(h) = hashes.facet(id, facet) {
                out.insert(Line::Facet {
                    id: id.clone(),
                    facet,
                    version: current_version(record, facet),
                    own: h.own,
                });
            }
        }
        for review in &record.reviews {
            // The reader refuses a review whose hash is not written as §3 requires, so every one parses.
            if let Some(bound) = Digest::parse(&review.hash) {
                out.insert(Line::Review {
                    id: id.clone(),
                    ledger: review_ledger_hash(id, review),
                    facet: review.facet,
                    verdict: review.verdict,
                    bound,
                });
            }
        }
    }
    out
}

/// The form a facet's version is written in: the record's own `version` for the contract, the facet's otherwise.
fn version_form(record: &Record, facet: FacetKind) -> Option<&Form> {
    let items = record.form.items();
    let holder = match facet {
        FacetKind::Contract => &record.form,
        _ => items.iter().find(|f| f.head() == Some(facet.as_str()))?,
    };
    holder.items().iter().find(|f| f.head() == Some("version"))
}

/// A facet line as the checks index it: its version, its own hash, and its line number in the file.
type Locked = (Version, Digest, usize);

/// The load checks of §9 that need only the tree: every facet's current version and every review against the lock.
///
/// A line a waiver names is exempt from every check that ties a line to its form: its record need not hold its form,
/// and a form it still holds is not compared with it. Whether the waiver itself stands needs history
/// (`M2.7.3.3.2`); here, one that names no review line of the lock is refused.
///
/// # Errors
///
/// - `catalog-lock-unbumped`: a line has a current facet's id, facet and version and another own hash;
/// - `catalog-lock-downgrade`: a facet's current version is below one the lock holds for it;
/// - `catalog-lock-missing`: a facet's current version, or a review in a record, has no line;
/// - `catalog-lock-review`: a review's form disagrees with its line's facet, verdict or hash; a present record lacks
///   a review the ledger holds for its id; a waiver names no review line.
pub fn check(lock: &Lock, catalog: &Catalog, hashes: &Hashes) -> Result<(), Refusal> {
    let mut facets: BTreeMap<(&str, FacetKind), Vec<Locked>> = BTreeMap::new();
    let mut reviews: BTreeMap<(&str, Digest), (usize, &Line)> = BTreeMap::new();
    let mut waived: BTreeSet<(&str, Digest)> = BTreeSet::new();
    for (index, line) in lock.lines.iter().enumerate() {
        match line {
            Line::Facet {
                id,
                facet,
                version,
                own,
            } => facets.entry((id.as_str(), *facet)).or_default().push((
                *version,
                *own,
                Lock::number(index),
            )),
            Line::Review { id, ledger, .. } => {
                reviews.insert((id.as_str(), *ledger), (Lock::number(index), line));
            }
            Line::Waiver { id, ledger, .. } => {
                if !reviews.contains_key(&(id.as_str(), *ledger)) {
                    return Err(Refusal::new(
                        Code::LockReview,
                        PATH,
                        "(lock)",
                        Some(At {
                            line: u32::try_from(Lock::number(index)).unwrap_or(u32::MAX),
                            column: 1,
                        }),
                        format!("the waiver names `{id}`'s review {ledger}, which no line of the lock ledgers"),
                    ));
                }
                waived.insert((id.as_str(), *ledger));
            }
        }
    }
    for (id, record) in &catalog.records {
        let bytes = catalog.tree.get(&record.path).unwrap_or_default();
        for facet in FacetKind::ALL {
            let Some(own) = hashes.facet(id, facet).map(|h| h.own) else {
                continue;
            };
            let current = current_version(record, facet);
            let field = format!("{} version", facet.as_str());
            let at = version_form(record, facet).and_then(|f| position(&record.path, bytes, f));
            let refuse =
                |code: Code, message: String| Refusal::new(code, &record.path, &field, at, message);
            let locked = facets
                .get(&(id.as_str(), facet))
                .map_or(&[][..], Vec::as_slice);
            if let Some((_, was, number)) = locked.iter().find(|(v, ..)| *v == current) {
                if *was != own {
                    return Err(refuse(
                        Code::LockUnbumped,
                        format!(
                            "the lock's line {number} pins {} {current} at {was}, and its own hash is now {own}: \
                             it changed without a version bump",
                            facet.as_str()
                        ),
                    ));
                }
            }
            if let Some((highest, _, number)) = locked.iter().filter(|(v, ..)| *v > current).max() {
                return Err(refuse(
                    Code::LockDowngrade,
                    format!(
                        "version {current} is below {highest}, which the lock's line {number} holds for {}",
                        facet.as_str()
                    ),
                ));
            }
            if !locked.iter().any(|(v, ..)| *v == current) {
                return Err(refuse(
                    Code::LockMissing,
                    format!(
                        "the lock has no line for {} {current}: bless it",
                        facet.as_str()
                    ),
                ));
            }
        }
        let mut held = BTreeSet::new();
        for review in &record.reviews {
            let ledger = review_ledger_hash(id, review);
            held.insert(ledger);
            if waived.contains(&(id.as_str(), ledger)) {
                continue;
            }
            let at = position(&record.path, bytes, &review.form);
            let refuse = |code: Code, message: String| {
                Refusal::new(code, &record.path, "review", at, message)
            };
            let Some((number, line)) = reviews.get(&(id.as_str(), ledger)) else {
                return Err(refuse(
                    Code::LockMissing,
                    format!("the lock has no line for this review, {ledger}: bless it"),
                ));
            };
            let form_line = Line::Review {
                id: id.clone(),
                ledger,
                facet: review.facet,
                verdict: review.verdict,
                bound: Digest::parse(&review.hash).unwrap_or(Digest([0; 32])),
            };
            if **line != form_line {
                return Err(refuse(
                    Code::LockReview,
                    format!(
                        "the lock's line {number} disagrees with this review's form, which gives `{}`",
                        form_line.render()
                    ),
                ));
            }
        }
        for (key, (number, _)) in
            reviews.range((id.as_str(), Digest([0; 32]))..=(id.as_str(), Digest([0xff; 32])))
        {
            if !held.contains(&key.1) && !waived.contains(key) {
                return Err(Refusal::new(
                    Code::LockReview,
                    &record.path,
                    "review",
                    None,
                    format!(
                        "the lock's line {number} ledgers review {} for `{id}`, which the record no longer holds: a \
                         review cannot be taken back",
                        key.1
                    ),
                ));
            }
        }
    }
    Ok(())
}

/// The lock a catalog's tree holds, read and checked against its present records.
///
/// # Errors
///
/// `catalog-lock-missing` when records exist and there is no lock, since blessing writes it; every refusal of
/// [`Lock::parse`] and [`check`].
pub fn check_tree(catalog: &Catalog, hashes: &Hashes) -> Result<Option<Lock>, Refusal> {
    let Some(bytes) = catalog.tree.get(PATH) else {
        return match catalog.records.values().next() {
            None => Ok(None),
            Some(record) => Err(Refusal::new(
                Code::LockMissing,
                &record.path,
                "(file)",
                None,
                format!("there is no `{PATH}`, so no facet version is locked: bless it"),
            )),
        };
    };
    let lock = Lock::parse(bytes)?;
    check(&lock, catalog, hashes)?;
    Ok(Some(lock))
}
