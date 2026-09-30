//! Evidence status, derived and never written (the record's §5, `M2.7.3.4.1`).
//!
//! A facet is `rejected` while it inherits a rejection no single production review of it answers over every item the
//! facet now holds; `production` when a production review names its bound hash and every rejection naming that hash
//! is answered there; `stale` when some review names it; `unreviewed` otherwise. A record's status is its weakest
//! facet's. Status reads each review from the commits that ledgered it, and the lineage from every ancestor commit,
//! so it needs the history; the order of reviews in a file decides nothing.

use std::collections::{BTreeMap, BTreeSet};

use archogen_evidence::sha256::Digest;

use crate::hash::{forms_hash, review_ledger_hash, Catalog, Hashes};
use crate::history::History;
use crate::lock::{Line, KNOWN_VERSIONS};
use crate::record::{Content, FacetKind, Packages, Record, Review, Verdict};
use crate::refusal::Refusal;
use crate::replay::{ledgering, lock_at, locks};
use crate::tree::Tree;

/// A facet's status, weakest first (§5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Status {
    /// It inherits a rejection that no single production review of it answers.
    Rejected,
    /// No review names it.
    Unreviewed,
    /// Some review names it, and none makes it `production`.
    Stale,
    /// A production review names its current bound hash, and every rejection naming that hash is answered there.
    Production,
}

/// What a rejection reaches a facet of the same kind through (§5): nothing about items is written, so nothing
/// about them can be edited away.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Item {
    /// A fact's name.
    Fact(String),
    /// A cost's name and target.
    Cost(String, String),
    /// A cost's name with its target's `TARGET_KIND` and `RUST_TARGET`.
    CostKind(String, String, String),
    /// A source entry, as written.
    Source(String),
    /// The hash of an own-set file's bytes.
    File(Digest),
    /// A contract's guarantee, by its decoded value.
    Guarantee(String),
    /// A contract's precondition, by its decoded value.
    Precondition(String),
    /// The facet's forms hash under a rules version's identifier.
    Forms(u64, Digest),
}

/// Every facet's status and every record's, by id.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Statuses {
    /// Each present record's facets.
    pub facets: BTreeMap<(String, FacetKind), Status>,
    /// Each present record: its weakest facet's status.
    pub records: BTreeMap<String, Status>,
}

/// A commit's catalog, read once.
struct AtCommit {
    catalog: Catalog,
    hashes: Option<Hashes>,
}

/// A rejection as the ledger and its ledgering commits give it.
struct Rejection {
    id: String,
    ledger: Digest,
    /// The bound hash it names.
    bound: Digest,
    /// Each facet it names: its line's, and its form's where a waiver lets the two differ.
    facets: BTreeSet<FacetKind>,
    /// Its items, per facet kind, from every commit that ledgered it.
    items: BTreeMap<FacetKind, BTreeSet<Item>>,
}

/// A production review that counts: not waived.
struct Production {
    id: String,
    facet: FacetKind,
    bound: Digest,
    answers: BTreeSet<Digest>,
    /// The items the facet held at the hash the review names.
    items: BTreeSet<Item>,
}

/// A value of `targets/<target>.env`, read leniently: §3's grammar is the hashes' to enforce.
fn env_value(tree: &Tree, target: &str, key: &str) -> Option<String> {
    let text = core::str::from_utf8(tree.get(&format!("targets/{target}.env"))?).ok()?;
    text.split('\n')
        .find_map(|l| l.strip_prefix(key)?.strip_prefix('='))
        .map(str::to_owned)
}

/// The items `facet` of `record` holds in `at` (§5), its forms hash under each rules version in `versions`.
fn items(
    at: &AtCommit,
    record: &Record,
    facet: FacetKind,
    versions: &BTreeSet<u64>,
) -> BTreeSet<Item> {
    let mut out = BTreeSet::new();
    for v in versions {
        out.insert(Item::Forms(
            *v,
            forms_hash(&format!("archogen-catalog/{v}"), record, facet),
        ));
    }
    if let Some(h) = at.hashes.as_ref().and_then(|h| h.facet(&record.id, facet)) {
        for path in &h.own_set {
            if let Some(bytes) = at.catalog.tree.get(path) {
                out.insert(Item::File(Digest::of(bytes)));
            }
        }
    }
    let sources = |out: &mut BTreeSet<Item>, entries: &[String]| {
        out.extend(entries.iter().map(|e| Item::Source(e.clone())));
    };
    match facet {
        FacetKind::Contract => {
            out.extend(
                record
                    .contract
                    .guarantees
                    .iter()
                    .map(|g| Item::Guarantee(g.clone())),
            );
            out.extend(
                record
                    .contract
                    .preconditions
                    .iter()
                    .map(|p| Item::Precondition(p.clone())),
            );
        }
        FacetKind::Implementation => {
            if let Content::Present(Packages(entries)) = &record.implementation.content {
                sources(&mut out, entries);
            }
        }
        FacetKind::BehaviorModel => {
            if let Content::Present(model) = &record.behavior_model.content {
                sources(&mut out, &model.sources);
                out.extend(model.facts.iter().map(|f| Item::Fact(f.name.clone())));
            }
        }
        FacetKind::TimingModel => {
            if let Content::Present(model) = &record.timing_model.content {
                sources(&mut out, &model.sources);
                out.extend(model.facts.iter().map(|f| Item::Fact(f.name.clone())));
                for cost in &model.costs {
                    out.insert(Item::Cost(cost.name.clone(), cost.target.clone()));
                    let tree = &at.catalog.tree;
                    if let (Some(kind), Some(rust)) = (
                        env_value(tree, &cost.target, "TARGET_KIND"),
                        env_value(tree, &cost.target, "RUST_TARGET"),
                    ) {
                        out.insert(Item::CostKind(cost.name.clone(), kind, rust));
                    }
                }
            }
        }
    }
    out
}

/// What status reads: the history, each commit's catalog read once, and the rules versions in play.
struct Reader<'h> {
    history: &'h History,
    commits: BTreeMap<String, AtCommit>,
}

impl Reader<'_> {
    /// Commit `name`'s catalog, read once.
    fn at(&mut self, name: &str) -> Result<&AtCommit, Refusal> {
        if !self.commits.contains_key(name) {
            let tree = self.history.get(name)?.tree.clone();
            let catalog = Catalog::read(tree).map_err(|mut r| {
                r.message = format!("at commit {name}: {}", r.message);
                r
            })?;
            let hashes = catalog.hashes().ok();
            self.commits
                .insert(name.to_owned(), AtCommit { catalog, hashes });
        }
        Ok(&self.commits[name])
    }
}

/// The review `(id, ledger)` and its record, as commit `at` holds them.
fn review_at<'a>(at: &'a AtCommit, id: &str, ledger: &Digest) -> Option<(&'a Record, &'a Review)> {
    let record = at.catalog.records.get(id)?;
    let review = record
        .reviews
        .iter()
        .find(|r| review_ledger_hash(id, r) == *ledger)?;
    Some((record, review))
}

/// Every facet's and record's status at `head`, under this loader's rules versions.
///
/// # Errors
///
/// As [`statuses_under`].
pub fn statuses(history: &History, head: &str) -> Result<Statuses, Refusal> {
    statuses_under(history, head, &KNOWN_VERSIONS)
}

/// Every facet's and record's status at `head` (§5), reading locks under the rules versions `known`. It assumes the
/// load checks passed: the lock's, over one tree and over history.
///
/// # Errors
///
/// `catalog-layout` for a commit the history lacks; the refusal met reading `head`'s catalog, its hashes or a lock;
/// the refusal met reading an ancestor's catalog, whose records the lineage needs.
pub fn statuses_under(history: &History, head: &str, known: &[u64]) -> Result<Statuses, Refusal> {
    let mut reader = Reader {
        history,
        commits: BTreeMap::new(),
    };
    let catalog = Catalog::read(history.get(head)?.tree.clone())?;
    let hashes = catalog.hashes()?;
    let lock = lock_at(history, head, known)?;
    let lines: Vec<Line> = lock.map(|l| l.lines).unwrap_or_default();
    let ancestry = history.ancestry(head)?;
    let all = locks(history, &ancestry, known)?;
    let mut versions = BTreeSet::new();
    for name in &ancestry {
        if let Some(lock) = lock_at(history, name, known)? {
            versions.insert(lock.version);
        }
    }
    // The lineage: every `supersedes` any ancestor commit's record held (§5).
    let mut edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for name in &ancestry {
        for record in reader.at(name)?.catalog.records.values() {
            edges
                .entry(record.id.clone())
                .or_default()
                .extend(record.contract.supersedes.iter().cloned());
        }
    }
    let waived: BTreeSet<(&str, Digest)> = lines
        .iter()
        .filter_map(|l| match l {
            Line::Waiver { id, ledger, .. } => Some((id.as_str(), *ledger)),
            _ => None,
        })
        .collect();
    let mut rejections: Vec<Rejection> = Vec::new();
    let mut productions: Vec<Production> = Vec::new();
    let mut named: BTreeSet<(String, FacetKind)> = BTreeSet::new();
    for line in &lines {
        let Line::Review {
            id,
            ledger,
            facet,
            verdict,
            bound,
        } = line
        else {
            continue;
        };
        named.insert((id.clone(), *facet));
        let at = ledgering(history, &all, line)?;
        if waived.contains(&(id.as_str(), *ledger)) {
            // A waived line lowers a status and never raises one: a production review establishes nothing, and a
            // rejection binds through each facet its line or form names, with the items of every ledgering commit.
            let mut rejection = Rejection {
                id: id.clone(),
                ledger: *ledger,
                bound: *bound,
                facets: BTreeSet::from([*facet]),
                items: BTreeMap::new(),
            };
            let mut rejected = *verdict == Verdict::Rejected;
            for name in &at {
                if let Some((_, review)) = review_at(reader.at(name)?, id, ledger) {
                    rejected |= review.verdict == Verdict::Rejected;
                    rejection.facets.insert(review.facet);
                    named.insert((id.clone(), review.facet));
                }
            }
            for name in &at {
                let commit = reader.at(name)?;
                if let Some(record) = commit.catalog.records.get(id) {
                    for f in &rejection.facets {
                        rejection
                            .items
                            .entry(*f)
                            .or_default()
                            .extend(items(commit, record, *f, &versions));
                    }
                }
            }
            if rejected {
                rejections.push(rejection);
            }
            continue;
        }
        // Verified at every ledgering commit (the load checks), so any one gives the form and the facet's content.
        let Some(first) = at.first() else {
            continue;
        };
        let commit = reader.at(first)?;
        let Some((record, review)) = review_at(commit, id, ledger) else {
            continue;
        };
        // Status reads a review's facet, verdict, hash and answers from its form, never from the line (§5).
        let (facet, named_hash) = (review.facet, Digest::parse(&review.hash).unwrap_or(*bound));
        let held = items(commit, record, facet, &versions);
        match review.verdict {
            Verdict::Rejected => rejections.push(Rejection {
                id: id.clone(),
                ledger: *ledger,
                bound: named_hash,
                facets: BTreeSet::from([facet]),
                items: BTreeMap::from([(facet, held)]),
            }),
            Verdict::Production => productions.push(Production {
                id: id.clone(),
                facet,
                bound: named_hash,
                answers: review
                    .answers
                    .iter()
                    .filter_map(|a| Digest::parse(a))
                    .collect(),
                items: held,
            }),
        }
    }
    let head_at = AtCommit {
        catalog,
        hashes: Some(hashes),
    };
    let mut out = Statuses::default();
    for (id, record) in &head_at.catalog.records {
        let lineage = closure(&edges, id);
        let mut weakest = Status::Production;
        for facet in FacetKind::ALL {
            let Some(h) = head_at
                .hashes
                .as_ref()
                .and_then(|x| x.facet(id, facet))
                .map(|x| x.bound)
            else {
                continue;
            };
            let now = items(&head_at, record, facet, &versions);
            let reached = |x: &Rejection| -> BTreeSet<Item> {
                x.items
                    .get(&facet)
                    .map(|i| i.intersection(&now).cloned().collect())
                    .unwrap_or_default()
            };
            let inherited: Vec<&Rejection> = rejections
                .iter()
                .filter(|x| {
                    x.facets.contains(&facet)
                        && (x.id == *id || lineage.contains(&x.id) || !reached(x).is_empty())
                })
                .collect();
            let answered = |x: &Rejection, at: Option<Digest>| {
                let reach = reached(x);
                productions.iter().any(|p| {
                    p.id == *id
                        && p.facet == facet
                        && at.is_none_or(|h| p.bound == h)
                        && p.answers.contains(&x.ledger)
                        && reach.is_subset(&p.items)
                })
            };
            let status = if inherited.iter().any(|x| !answered(x, None)) {
                Status::Rejected
            } else if productions
                .iter()
                .any(|p| p.id == *id && p.facet == facet && p.bound == h)
                && inherited
                    .iter()
                    .filter(|x| x.bound == h)
                    .all(|x| answered(x, Some(h)))
            {
                Status::Production
            } else if named.contains(&(id.clone(), facet)) {
                Status::Stale
            } else {
                Status::Unreviewed
            };
            weakest = weakest.min(status);
            out.facets.insert((id.clone(), facet), status);
        }
        out.records.insert(id.clone(), weakest);
    }
    Ok(out)
}

/// Every id `id` supersedes, directly or through a chain, by `edges`.
fn closure(edges: &BTreeMap<String, BTreeSet<String>>, id: &str) -> BTreeSet<String> {
    let mut out = BTreeSet::new();
    let mut stack: Vec<&str> = vec![id];
    while let Some(current) = stack.pop() {
        for next in edges.get(current).into_iter().flatten() {
            if out.insert(next.clone()) {
                stack.push(next);
            }
        }
    }
    out
}
