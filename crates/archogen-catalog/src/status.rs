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
pub(crate) struct AtCommit {
    pub(crate) catalog: Catalog,
    pub(crate) hashes: Option<Hashes>,
}

/// A rejection as the ledger and its ledgering commits give it.
pub(crate) struct Rejection {
    pub(crate) id: String,
    pub(crate) ledger: Digest,
    /// The bound hash it names.
    pub(crate) bound: Digest,
    /// Each facet it names: its line's, and its form's where a waiver lets the two differ.
    pub(crate) facets: BTreeSet<FacetKind>,
    /// Its items, per facet kind, from every commit that ledgered it.
    pub(crate) items: BTreeMap<FacetKind, BTreeSet<Item>>,
}

/// A production review that counts: not waived.
pub(crate) struct Production {
    pub(crate) id: String,
    pub(crate) facet: FacetKind,
    pub(crate) bound: Digest,
    pub(crate) answers: BTreeSet<Digest>,
    /// The items the facet held at the hash the review names.
    pub(crate) items: BTreeSet<Item>,
}

/// A value of `targets/<target>.env`, read leniently: §3's grammar is the hashes' to enforce.
pub(crate) fn env_value(tree: &Tree, target: &str, key: &str) -> Option<String> {
    let text = core::str::from_utf8(tree.get(&format!("targets/{target}.env"))?).ok()?;
    text.split('\n')
        .find_map(|l| l.strip_prefix(key)?.strip_prefix('='))
        .map(str::to_owned)
}

/// The items `facet` of `record` holds in `at` (§5), its forms hash under each rules version in `versions`.
pub(crate) fn items(
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
            if let Content::Present(Packages {
                sources: entries, ..
            }) = &record.implementation.content
            {
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

/// What status reads: the history, and each commit's catalog read once.
pub(crate) struct Reader<'h> {
    pub(crate) history: &'h History,
    commits: BTreeMap<String, AtCommit>,
}

impl<'h> Reader<'h> {
    /// A reader of `history`.
    pub(crate) fn new(history: &'h History) -> Self {
        Self {
            history,
            commits: BTreeMap::new(),
        }
    }

    /// Commit `name`'s catalog, read once.
    pub(crate) fn at(&mut self, name: &str) -> Result<&AtCommit, Refusal> {
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
pub(crate) fn review_at<'a>(
    at: &'a AtCommit,
    id: &str,
    ledger: &Digest,
) -> Option<(&'a Record, &'a Review)> {
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
    let mut reader = Reader::new(history);
    let catalog = Catalog::read(history.get(head)?.tree.clone())?;
    let hashes = catalog.hashes()?;
    let ledger = Ledger::at(&mut reader, head, known)?;
    let head_at = AtCommit {
        catalog,
        hashes: Some(hashes),
    };
    let mut out = Statuses::default();
    for (id, record) in &head_at.catalog.records {
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
            let now = items(&head_at, record, facet, &ledger.versions);
            let inherited = ledger.inherited(id, facet, &now);
            let status = if inherited
                .iter()
                .any(|(x, _)| !ledger.answered(id, facet, &now, x, None))
            {
                Status::Rejected
            } else if ledger
                .productions
                .iter()
                .any(|p| p.id == *id && p.facet == facet && p.bound == h)
                && inherited
                    .iter()
                    .filter(|(x, _)| x.bound == h)
                    .all(|(x, _)| ledger.answered(id, facet, &now, x, Some(h)))
            {
                Status::Production
            } else if ledger.named.contains(&(id.clone(), facet)) {
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

/// How a facet inherits a rejection (§5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub(crate) enum Route {
    /// Its own record's rejection of that facet.
    Own,
    /// A rejection of that facet of an id its record supersedes, directly or through a chain.
    Lineage,
    /// Only through items the facet holds.
    Items,
}

/// What the ledger at a commit gives status: its rejections, the production reviews that count, the facets its
/// reviews name, the lineage and the rules versions in play.
pub(crate) struct Ledger {
    pub(crate) rejections: Vec<Rejection>,
    pub(crate) productions: Vec<Production>,
    pub(crate) named: BTreeSet<(String, FacetKind)>,
    pub(crate) edges: BTreeMap<String, BTreeSet<String>>,
    pub(crate) versions: BTreeSet<u64>,
}

impl Ledger {
    /// The ledger of the lock at `head`, each review read from the commits that ledgered it.
    pub(crate) fn at(reader: &mut Reader<'_>, head: &str, known: &[u64]) -> Result<Self, Refusal> {
        let lock = lock_at(reader.history, head, known)?;
        let lines: Vec<Line> = lock.map(|l| l.lines).unwrap_or_default();
        let history = reader.history;
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
        Ok(Self {
            rejections,
            productions,
            named,
            edges,
            versions,
        })
    }

    /// The rejections facet `facet` of record `id` inherits, holding the items `now`, with how.
    pub(crate) fn inherited(
        &self,
        id: &str,
        facet: FacetKind,
        now: &BTreeSet<Item>,
    ) -> Vec<(&Rejection, Route)> {
        let lineage = closure(&self.edges, id);
        self.rejections
            .iter()
            .filter(|x| x.facets.contains(&facet))
            .filter_map(|x| {
                if x.id == id {
                    Some((x, Route::Own))
                } else if lineage.contains(&x.id) {
                    Some((x, Route::Lineage))
                } else if !reached(x, facet, now).is_empty() {
                    Some((x, Route::Items))
                } else {
                    None
                }
            })
            .collect()
    }

    /// Whether a single production review of facet `facet` of record `id`, at hash `at` when one is given, answers
    /// rejection `x` while covering every item of it the facet, holding `now`, holds.
    pub(crate) fn answered(
        &self,
        id: &str,
        facet: FacetKind,
        now: &BTreeSet<Item>,
        x: &Rejection,
        at: Option<Digest>,
    ) -> bool {
        let reach = reached(x, facet, now);
        self.productions.iter().any(|p| {
            p.id == id
                && p.facet == facet
                && at.is_none_or(|h| p.bound == h)
                && p.answers.contains(&x.ledger)
                && reach.is_subset(&p.items)
        })
    }
}

/// The items of rejection `x` that facet `facet`, holding `now`, holds.
fn reached(x: &Rejection, facet: FacetKind, now: &BTreeSet<Item>) -> BTreeSet<Item> {
    x.items
        .get(&facet)
        .map(|i| i.intersection(now).cloned().collect())
        .unwrap_or_default()
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
