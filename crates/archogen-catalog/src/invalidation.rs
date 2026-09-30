//! Invalidation (the record's §10, `M2.7.3.4.3`).
//!
//! > Dependency-based invalidation must identify affected builds and analyses. (`ROADMAP.md` §9)
//!
//! A claim records its closure, each line with the bound hash and status it relied on, and its reads. Invalidation
//! judges each against the catalog now, over the recorded closure and not today's graph. It runs even when the
//! catalog does not load: each record is read on its own, each facet hashed on its own and without the lock, and
//! anything that cannot be read or computed counts as affected. The answer errs toward too many and never too few.

use archogen_evidence::sha256::Digest;

use crate::hash::Catalog;
use crate::history::History;
use crate::record::{classify, read_record, CatalogPath, Content, FacetKind, Namespace, Record};
use crate::refusal::Refusal;
use crate::status::{statuses, Status};

/// One line of a recorded closure: a facet with the bound hash and status the claim relied on.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClosureLine {
    /// The record's id.
    pub id: String,
    /// The facet.
    pub facet: FacetKind,
    /// Its bound hash when the claim was made.
    pub bound: Digest,
    /// Its status when the claim was made.
    pub status: Status,
}

/// Why a recorded line is affected, the first of §10's causes that holds, in §10's order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Cause {
    /// No record has the id now.
    Gone,
    /// The record cannot be read, or the id names more than one file.
    Unreadable(String),
    /// The facet is now `none`.
    None,
    /// The bound hash now differs (`Some`), or cannot be computed (`None`).
    Hash(Option<Digest>),
    /// The status at the recorded hash now differs (`Some`), or cannot be computed (`None`).
    Status(Option<Status>),
    /// For a production claim: the record is no longer in the `production` namespace.
    NotProduction,
}

/// A recorded line judged affected, with why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Affected {
    /// The line as the claim recorded it.
    pub line: ClosureLine,
    /// The cause.
    pub cause: Cause,
}

/// A lookup a claim recorded: a `(facet, name)` under a profile and a target, and the record it found, if any.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Read {
    /// The facet looked up.
    pub facet: FacetKind,
    /// The name looked up.
    pub name: String,
    /// The profile it was looked up under.
    pub profile: String,
    /// The target, when the claim had one.
    pub target: Option<String>,
    /// The record's id it found, or `None` when it found nothing.
    pub found: Option<String>,
}

/// Each record file the tracked set holds for `id`, and whether it is in `production`.
fn files_for<'a>(catalog_tree: &'a crate::tree::Tree, id: &str) -> Vec<(&'a str, Namespace)> {
    catalog_tree
        .under("catalog")
        .filter_map(|path| match classify(path) {
            Ok(CatalogPath::Record(namespace, stem)) if stem == id => Some((path, namespace)),
            _ => None,
        })
        .collect()
}

/// Whether facet `facet` of `record` is a `none` statement.
fn is_none(record: &Record, facet: FacetKind) -> bool {
    match facet {
        FacetKind::Contract => false,
        FacetKind::Implementation => matches!(record.implementation.content, Content::None(_)),
        FacetKind::BehaviorModel => matches!(record.behavior_model.content, Content::None(_)),
        FacetKind::TimingModel => matches!(record.timing_model.content, Content::None(_)),
    }
}

/// The recorded closure `lines` judged against commit `head` of `history` (§10); `production` for a production
/// claim. Each affected line comes with its first cause, in §10's order: the record gone or unreadable, the facet
/// now `none` where its bound hash moved, the bound hash differing or not computable, the status at the recorded
/// hash differing or not computable, and for a production claim the record out of `production`.
///
/// What a production claim also checks, that the `main` commit it recorded is still an ancestor of `origin/main`,
/// is the claim's (`M2.7.3.6`).
///
/// # Errors
///
/// `catalog-layout` when the history lacks `head`: there is then no "now" to judge against.
pub fn affected_lines(
    history: &History,
    head: &str,
    lines: &[ClosureLine],
    production: bool,
) -> Result<Vec<Affected>, Refusal> {
    let tree = history.get(head)?.tree.clone();
    // Every record that reads on its own; the catalog they make need not load.
    let mut readable = Vec::new();
    for path in tree.under("catalog") {
        if let Ok(CatalogPath::Record(..)) = classify(path) {
            if let Ok(record) = read_record(path, tree.get(path).unwrap_or_default()) {
                readable.push(record);
            }
        }
    }
    let files: Vec<(String, Vec<(String, Namespace)>)> = lines
        .iter()
        .map(|l| {
            (
                l.id.clone(),
                files_for(&tree, &l.id)
                    .into_iter()
                    .map(|(p, ns)| (p.to_owned(), ns))
                    .collect(),
            )
        })
        .collect();
    let catalog = Catalog::new(tree, readable);
    let now = statuses(history, head).ok();
    let mut out = Vec::new();
    for (line, (_, found)) in lines.iter().zip(&files) {
        let cause = judge(&catalog, now.as_ref(), line, found, production);
        if let Some(cause) = cause {
            out.push(Affected {
                line: line.clone(),
                cause,
            });
        }
    }
    Ok(out)
}

/// The first cause that affects `line`, if any.
fn judge(
    catalog: &Catalog,
    now: Option<&crate::status::Statuses>,
    line: &ClosureLine,
    found: &[(String, Namespace)],
    production: bool,
) -> Option<Cause> {
    let [(path, namespace)] = found else {
        return Some(if found.is_empty() {
            Cause::Gone
        } else {
            Cause::Unreadable(format!("`{}` is in more than one file", line.id))
        });
    };
    // The one file for the id holds a record with that id, or the reader refused it: ids are file stems.
    let Some(record) = catalog.records.get(&line.id) else {
        let why = match read_record(path, catalog.tree.get(path).unwrap_or_default()) {
            Err(r) => r.to_string(),
            Ok(_) => format!("`{path}` does not hold `{}`", line.id),
        };
        return Some(Cause::Unreadable(why));
    };
    // A facet that became `none` moved its bound hash, so `none` labels a moved hash; one that was `none` and has
    // not moved relied on nothing that changed.
    match catalog.facet_hash(&line.id, line.facet) {
        Ok(h) if h.bound == line.bound => {}
        _ if is_none(record, line.facet) => return Some(Cause::None),
        Ok(h) => return Some(Cause::Hash(Some(h.bound))),
        Err(_) => return Some(Cause::Hash(None)),
    }
    // The bound hash is the recorded one, so the status now is the status at the recorded hash.
    let status = now.and_then(|s| s.facets.get(&(line.id.clone(), line.facet)).copied());
    if status != Some(line.status) {
        return Some(Cause::Status(status));
    }
    if production && *namespace != Namespace::Production {
        return Some(Cause::NotProduction);
    }
    None
}

/// The recorded reads that are affected (§10): the same lookup, under the same selection, now returns another record,
/// or finds one where it found nothing, or cannot be made because the catalog no longer loads. `lookup` makes a
/// lookup against the catalog now; §12's selection is `M2.7.3.5`'s.
pub fn affected_reads<F>(reads: &[Read], mut lookup: F) -> Vec<&Read>
where
    F: FnMut(&Read) -> Result<Option<String>, Refusal>,
{
    reads
        .iter()
        .filter(|read| match lookup(read) {
            Ok(found) => found != read.found,
            Err(_) => true,
        })
        .collect()
}
