//! Reading `docs/semantics/BASELINE.txt`, and classifying how two baselines differ.
//!
//! ⛔ **One reader for the baseline, and the comparison is a classification rather than a `diff`.**
//! `M1.13.5`'s gate has to tell a *moved* construct from an *added* or *removed* one, because the three
//! mean different things: a moved digest is a language change or a correction, an added construct is a
//! language that grew, and a removed one is a construct that stopped being frozen — and the migration
//! note the gate demands has to say which happened. `diff` prints lines; this names constructs.
//!
//! The digests themselves are computed in shell, by `scripts/language_baseline.sh`, because the
//! workspace carries zero dependencies and so has no hasher
//! (`docs/decisions/decision_zero-dependency-engine-core.md`). What Rust owns is the classification and
//! the shape of the file, which is what a gate has to reason about.

use std::collections::BTreeMap;

/// Where the baseline lives, repo-root-relative.
pub const BASELINE_PATH: &str = "docs/semantics/BASELINE.txt";

/// The tracked baseline, compiled in.
///
/// `include_str!` for the reason `corpus.rs` gives for its live surfaces: if the baseline moves or is
/// deleted, this crate **stops compiling** instead of a gate silently comparing nothing.
pub const BASELINE: &str = include_str!("../../../../docs/semantics/BASELINE.txt");

/// A baseline: one digest per frozen construct, keyed by construct id.
pub type Baseline = BTreeMap<String, String>;

/// Parse a baseline file, returning **every** problem found rather than the first.
///
/// ⛔ A malformed row is a violation and never a row to skip: a baseline that silently dropped a
/// construct would freeze less than it says it does, and the gate built on it would report green.
pub fn parse(text: &str) -> Result<Baseline, Vec<String>> {
    let mut problems = Vec::new();
    let mut out = Baseline::new();
    let mut previous: Option<&str> = None;
    for (index, line) in text.lines().enumerate() {
        let number = index + 1;
        if line.starts_with('#') || line.trim().is_empty() {
            continue;
        }
        let Some((digest, id)) = line.split_once("  ") else {
            problems.push(format!(
                "{BASELINE_PATH}:{number}: `{}` is not `<sha256>  <construct id>` — two spaces \
                 separate them, and a row nothing can parse is a construct nothing freezes",
                truncate(line)
            ));
            continue;
        };
        if digest.len() != 64 || !digest.chars().all(|c| c.is_ascii_hexdigit()) {
            problems.push(format!(
                "{BASELINE_PATH}:{number}: `{}` is not a 64-character hexadecimal sha256, so the row \
                 cannot be compared with a fresh digest",
                truncate(digest)
            ));
        }
        if digest.chars().any(|c| c.is_ascii_uppercase()) {
            problems.push(format!(
                "{BASELINE_PATH}:{number}: the digest carries an uppercase character, and a file two \
                 tools produce must be byte-comparable"
            ));
        }
        if id.is_empty() {
            problems.push(format!(
                "{BASELINE_PATH}:{number}: the row names no construct",
            ));
        } else if let Some(existing) = out.insert(id.to_string(), digest.to_string()) {
            problems.push(format!(
                "{BASELINE_PATH}:{number}: `{id}` appears twice, and one digest would stand for two \
                 constructs (the first was `{}`)",
                truncate(&existing)
            ));
        } else if let Some(before) = previous {
            if before > id {
                problems.push(format!(
                    "{BASELINE_PATH}:{number}: `{id}` follows `{before}`, and the file is generated \
                     sorted by id so that two runs are byte-comparable — an unsorted row means the file \
                     was edited by hand"
                ));
            }
        }
        if !id.is_empty() {
            previous = Some(id);
        }
    }
    if out.is_empty() && problems.is_empty() {
        problems.push(format!(
            "{BASELINE_PATH} freezes no construct at all, so a gate comparing against it would pass on \
             any language"
        ));
    }
    if problems.is_empty() {
        Ok(out)
    } else {
        Err(problems)
    }
}

/// How a fresh baseline differs from the tracked one.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Difference {
    /// The construct is in both and its digest moved: the language, or a description, changed.
    Moved {
        /// The construct.
        id: String,
        /// The digest the tracked baseline records.
        tracked: String,
        /// The digest a fresh run computed.
        fresh: String,
    },
    /// The construct is fresh and not tracked: the frozen population grew.
    Added {
        /// The construct.
        id: String,
    },
    /// The construct is tracked and not fresh: something stopped being frozen.
    Removed {
        /// The construct.
        id: String,
    },
}

impl Difference {
    /// The construct this difference is about.
    #[must_use]
    pub fn id(&self) -> &str {
        match self {
            Self::Moved { id, .. } | Self::Added { id } | Self::Removed { id } => id,
        }
    }

    /// What kind of movement this is, in the words a migration note has to answer to.
    #[must_use]
    pub fn kind(&self) -> &'static str {
        match self {
            Self::Moved { .. } => "moved",
            Self::Added { .. } => "added",
            Self::Removed { .. } => "removed",
        }
    }

    /// One line naming the construct, the class of movement, and both digests when there are two.
    #[must_use]
    pub fn render(&self) -> String {
        match self {
            Self::Moved {
                id,
                tracked,
                fresh,
            } => format!(
                "{id} moved: the baseline records {} and a fresh run computes {}",
                truncate(tracked),
                truncate(fresh)
            ),
            Self::Added { id } => format!(
                "{id} is new: a construct the baseline does not freeze, so nothing compares it with \
                 anything"
            ),
            Self::Removed { id } => format!(
                "{id} is gone: the baseline freezes a construct that no longer exists, which is either \
                 a removal from the language or a construct the instrument stopped reaching"
            ),
        }
    }
}

/// Classify every difference between a fresh baseline and the tracked one.
///
/// ⭐ Ordered by construct id rather than by class, so two runs over the same movement print the same
/// thing and a reader can diff the report itself.
pub fn compare(fresh: &Baseline, tracked: &Baseline) -> Vec<Difference> {
    let mut out = Vec::new();
    for (id, digest) in tracked {
        match fresh.get(id) {
            None => out.push(Difference::Removed { id: id.clone() }),
            Some(found) if found != digest => out.push(Difference::Moved {
                id: id.clone(),
                tracked: digest.clone(),
                fresh: found.clone(),
            }),
            Some(_) => {}
        }
    }
    for id in fresh.keys() {
        if !tracked.contains_key(id) {
            out.push(Difference::Added { id: id.clone() });
        }
    }
    out.sort_by(|left, right| left.id().cmp(right.id()));
    out
}

/// The three shapes a construct id may have, as the instrument emits them.
///
/// ⛔ Checked rather than assumed: an id of a fourth shape means the instrument grew a construct class
/// and this reader — and every gate above it — has not been told, which is how a freeze comes to cover
/// less than its own file says.
pub fn id_shape_violations(baseline: &Baseline) -> Vec<String> {
    let mut out = Vec::new();
    for id in baseline.keys() {
        let known = if let Some(document) = id.split_once('#') {
            document.1 == "ebnf" || !document.1.is_empty()
        } else {
            id.starts_with("suite/")
        };
        if !known {
            out.push(format!(
                "`{id}` is neither `<document>#<fence or table>` nor `suite/<path>`, so nothing can say \
                 which class of frozen construct it is"
            ));
        }
    }
    out
}

/// Which of the three construct classes a baseline covers, as `(class, count)`.
pub fn classes(baseline: &Baseline) -> [(&'static str, usize); 3] {
    [
        (
            "the grammar's EBNF fence",
            baseline.keys().filter(|id| id.ends_with("#ebnf")).count(),
        ),
        (
            "the reference's machine-read tables",
            baseline
                .keys()
                .filter(|id| id.contains('#') && !id.ends_with("#ebnf"))
                .count(),
        ),
        (
            "the canonical form of a suite description",
            baseline
                .keys()
                .filter(|id| id.starts_with("suite/"))
                .count(),
        ),
    ]
}

fn truncate(text: &str) -> String {
    if text.len() <= 16 {
        text.to_string()
    } else {
        format!("{}…", &text[..12])
    }
}
