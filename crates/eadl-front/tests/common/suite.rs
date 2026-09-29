//! Reading `docs/semantics/conformance.md`'s manifest, and enumerating the population it declares.
//!
//! ⛔ **One reader for the suite's scope, shared by every walk.** `conformance.rs` compares the
//! recognizer derived from `docs/semantics/grammar.md` with the reader over the suite;
//! `reference.rs` compares `docs/semantics/reference.md`'s stated values with the frontend over the
//! same suite; `conformance_suite.rs` checks the manifest's own four rules. Before this module existed
//! the first two each hardcoded `docs/semantics` and `examples`, so the suite's scope was stated twice
//! and enforced nowhere — and a third definition would have been a fourth thing to drift.
//!
//! ⭐ **The manifest carries no count, and this module never derives one from prose.** The population is
//! walked from the declared roots, so a manifest that grows a root does not leave a stale number behind
//! and a suite that silently walks nothing is a violation rather than a green run.
//!
//! The notation is the house one: a markdown table introduced by a `<!-- machine-read: … -->` comment,
//! read by the same `machine_table` that reads the reference's tables. Two readers for one table shape
//! would be two things that can disagree about what the manifest *says*.

use std::path::Path;

use super::reference_table::machine_table;

/// Where the manifest lives, repo-root-relative.
pub const MANIFEST_PATH: &str = "docs/semantics/conformance.md";

/// The manifest, compiled in.
///
/// `include_str!` rather than `fs::read_to_string`, for the reason `corpus.rs` gives for its live
/// surfaces: if the manifest moves or is deleted, this crate **stops compiling** instead of silently
/// gating nothing.
pub const MANIFEST: &str = include_str!("../../../../docs/semantics/conformance.md");

/// The directories a repository walk never descends into.
///
/// ⛔ Structural, and not a list of "places that happen to hold no description": `.git` and every other
/// hidden directory hold no tracked description by construction; `target` and `build` are the two
/// regenerated scratch roots `.gitignore` names, and both *do* hold `.eadl` files — test scratch and a
/// deliberately malformed fixture; `vendor` is a submodule, read-only per §20 of the standing
/// instructions, whose contents are not this repository's descriptions. A description added under a
/// **new** top-level directory is none of these, so rule 4 sees it — which is the case rule 4 exists
/// for. The honest residue: a description placed *inside* a scratch root is invisible to that rule, and
/// cannot be made visible, because the root is regenerated.
const NEVER_WALKED: &[&str] = &["target", "build", "vendor"];

/// One root the suite walks, and what it proves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Root {
    /// The directory, repo-root-relative.
    pub path: String,
    /// What every description under it is evidence of.
    pub proves: String,
}

/// One excluded path prefix, and why it is not a conformance case.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Exclusion {
    /// A path prefix, matched segment-wise. No glob grammar — see the manifest's own rule.
    pub prefix: String,
    /// Why everything under it is outside the suite.
    pub reason: String,
}

/// The declared conformance suite.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Manifest {
    /// The language version the population conforms to.
    pub version: String,
    /// What conforming to it means, as the manifest states it.
    pub means: String,
    /// The roots, in the order the manifest lists them.
    pub roots: Vec<Root>,
    /// The exclusions, in the order the manifest lists them.
    pub exclusions: Vec<Exclusion>,
}

impl Manifest {
    /// Read a manifest out of a document, returning **every** problem found rather than the first.
    ///
    /// ⛔ A problem is a violation and never a reason to skip a table: a manifest whose version table
    /// is missing does not declare a suite with no version, it declares nothing, and a leg that
    /// silently enumerated an empty population would be green on a suite that does not exist.
    pub fn parse(document: &str) -> Result<Self, Vec<String>> {
        let mut problems = Vec::new();

        let versions = machine_table(document, "suite-version");
        let mut version = None;
        let mut means = None;
        if versions.is_empty() {
            problems.push(format!(
                "{MANIFEST_PATH} carries no `<!-- machine-read: suite-version -->` table, so nothing \
                 states which language version this suite conforms to"
            ));
        } else if versions.len() != 1 {
            problems.push(format!(
                "{MANIFEST_PATH}'s version table has {} rows, and a suite conforms to one version — \
                 two rows are two suites in one manifest",
                versions.len()
            ));
        } else {
            let (line, cells) = &versions[0];
            match cells.as_slice() {
                [declared, statement] if !declared.is_empty() && !statement.is_empty() => {
                    version = Some(declared.clone());
                    means = Some(statement.clone());
                }
                _ => problems.push(format!(
                    "{MANIFEST_PATH}:{line}: the version row needs a version and what conforming to it \
                     means, and this one has {} cell(s)",
                    cells.len()
                )),
            }
        }

        let roots = rows(document, "suite-roots", 2, &mut problems)
            .into_iter()
            .filter_map(|(line, cells)| {
                let [path, proves] = cells.as_slice() else {
                    return None;
                };
                if path.is_empty() || proves.is_empty() {
                    problems.push(format!(
                        "{MANIFEST_PATH}:{line}: a root row names a root and what it proves, and one of \
                         the two is empty — a root that proves nothing is a directory nobody can say \
                         why it is in the suite"
                    ));
                    return None;
                }
                Some(Root {
                    path: path.clone(),
                    proves: proves.clone(),
                })
            })
            .collect::<Vec<_>>();

        let exclusions = rows(document, "suite-exclusions", 2, &mut problems)
            .into_iter()
            .filter_map(|(line, cells)| {
                let [prefix, reason] = cells.as_slice() else {
                    return None;
                };
                if prefix.is_empty() || reason.is_empty() {
                    problems.push(format!(
                        "{MANIFEST_PATH}:{line}: an exclusion row names a path and why it is outside the \
                         suite, and one of the two is empty — an exclusion with no reason is a suite \
                         whose scope nobody can check"
                    ));
                    return None;
                }
                Some(Exclusion {
                    prefix: prefix.clone(),
                    reason: reason.clone(),
                })
            })
            .collect::<Vec<_>>();

        if !problems.is_empty() {
            return Err(problems);
        }
        Ok(Self {
            version: version.expect("one version row, checked above"),
            means: means.expect("one version row, checked above"),
            roots,
            exclusions,
        })
    }

    /// The manifest this repository ships.
    ///
    /// # Panics
    ///
    /// When the manifest is malformed. Every caller is a leg, and a leg that enumerated a suite the
    /// manifest does not declare would be worse than one that stops.
    pub fn declared() -> Self {
        Self::parse(MANIFEST).unwrap_or_else(|problems| {
            panic!(
                "{} is not a readable manifest:\n\n{}",
                MANIFEST_PATH,
                problems.join("\n")
            )
        })
    }

    /// The exclusion that keeps `path` out of the suite, if any.
    pub fn excludes(&self, path: &str) -> Option<&Exclusion> {
        self.exclusions
            .iter()
            .find(|exclusion| excluded_by(&exclusion.prefix, path))
    }
}

/// Read one machine-read table, reporting a missing or mis-shaped one as a violation.
fn rows(
    document: &str,
    marker: &str,
    width: usize,
    problems: &mut Vec<String>,
) -> Vec<(usize, Vec<String>)> {
    let found = machine_table(document, marker);
    if found.is_empty() {
        problems.push(format!(
            "{MANIFEST_PATH} carries no `<!-- machine-read: {marker} -->` table, so the manifest \
             declares nothing and a leg reading it would be green on an empty suite"
        ));
        return Vec::new();
    }
    for (line, cells) in &found {
        if cells.len() != width {
            problems.push(format!(
                "{MANIFEST_PATH}:{line}: the `{marker}` table takes {width} cells per row and this row \
                 has {} — a dropped cell is silent in markdown and invisible in the rendered page",
                cells.len()
            ));
        }
    }
    found
}

/// Whether `prefix` excludes `path`.
///
/// ⭐ Segment-wise, and the whole of the grammar: the prefix's segments must equal the path's leading
/// segments, so a prefix names a directory and everything under it. There is no `*`, no `**` and no
/// character class — a matcher with the full glob grammar is a second thing to be wrong about beside
/// the manifest, and the manifest states this rule so a pattern that needs a glob is a pattern to write
/// differently rather than a matcher to widen.
pub fn excluded_by(prefix: &str, path: &str) -> bool {
    let pattern: Vec<&str> = prefix
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    let segments: Vec<&str> = path
        .split('/')
        .filter(|segment| !segment.is_empty())
        .collect();
    if pattern.is_empty() || pattern.len() > segments.len() {
        return false;
    }
    pattern
        .iter()
        .zip(&segments)
        .all(|(want, found)| want == found)
}

/// Every `.eadl` file under `dir`, as repo-relative paths, appended to `out`.
fn walk(root: &Path, dir: &Path, out: &mut Vec<String>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.filter_map(Result::ok) {
        let path = entry.path();
        if path.is_dir() {
            walk(root, &path, out);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "eadl")
        {
            if let Ok(relative) = path.strip_prefix(root) {
                out.push(relative.display().to_string());
            }
        }
    }
}

/// Every `.eadl` file under each declared root, without reading them.
///
/// Returned with the problems found, because a leg has to be able to feed it a synthetic set of
/// candidates: an arm that can only run against the real tree cannot show the rule firing.
pub fn under_roots(manifest: &Manifest, root: &Path) -> (Vec<String>, Vec<String>) {
    let mut problems = Vec::new();
    let mut found = Vec::new();
    for declared in &manifest.roots {
        let dir = root.join(&declared.path);
        if !dir.is_dir() {
            problems.push(format!(
                "the manifest declares `{}` as a suite root and there is no such directory, so the \
                 suite is narrower than the manifest says",
                declared.path
            ));
            continue;
        }
        let mut under = Vec::new();
        walk(root, &dir, &mut under);
        if under.is_empty() {
            problems.push(format!(
                "the manifest declares `{}` as a suite root and it holds no `.eadl` file, so the root \
                 proves nothing — a root that stopped being walked looks exactly like one that was \
                 never populated",
                declared.path
            ));
            continue;
        }
        for path in under {
            if let Some(exclusion) = manifest.excludes(&path) {
                problems.push(format!(
                    "the root `{}` reaches `{}`, which the manifest excludes — {} — and a root that \
                     reaches an excluded path is a violation rather than a skip, because skipping \
                     quietly is how a too-wide root narrows a suite by accident",
                    declared.path, path, exclusion.reason
                ));
                continue;
            }
            found.push(path);
        }
    }
    found.sort();
    found.dedup();
    (found, problems)
}

/// The declared population, as `(repo-relative path, text)`, sorted by path.
///
/// # Errors
///
/// Every problem found: a malformed manifest, a root that does not exist or holds nothing, a root that
/// reaches an excluded path, an unreadable file, or an empty population.
pub fn population(root: &Path) -> Result<Vec<(String, String)>, Vec<String>> {
    let manifest = Manifest::declared();
    enumerate(&manifest, root)
}

/// [`population`] against a manifest a leg supplies, so an arm can vary one table at a time.
///
/// # Errors
///
/// As [`population`].
pub fn enumerate(manifest: &Manifest, root: &Path) -> Result<Vec<(String, String)>, Vec<String>> {
    let (paths, mut problems) = under_roots(manifest, root);
    if paths.is_empty() && problems.is_empty() {
        problems.push(
            "the manifest declares no description at all, so every leg that walks the suite is green on \
             an empty population and proves nothing"
                .to_string(),
        );
    }
    if !problems.is_empty() {
        return Err(problems);
    }
    let mut out = Vec::with_capacity(paths.len());
    for path in paths {
        let text = std::fs::read_to_string(root.join(&path))
            .map_err(|error| vec![format!("cannot read {path}: {error}")])?;
        out.push((path, text));
    }
    Ok(out)
}

/// Every `.eadl` file in the repository, including the ones the manifest excludes.
///
/// The ground truth rule 4 and the non-vacuity leg are checked against: a walk of the repository that
/// skips only [`NEVER_WALKED`] and hidden directories, and so does not share a root list with the
/// manifest it is checking.
pub fn repository_descriptions(root: &Path) -> Vec<String> {
    let mut out = Vec::new();
    let mut stack = vec![root.to_path_buf()];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            let name = path
                .file_name()
                .map_or_else(String::new, |name| name.to_string_lossy().to_string());
            if path.is_dir() {
                let relative = path
                    .strip_prefix(root)
                    .map(|relative| relative.display().to_string())
                    .unwrap_or_default();
                let top = relative.split('/').next().unwrap_or_default();
                if name.starts_with('.') || NEVER_WALKED.contains(&top) {
                    continue;
                }
                stack.push(path);
            } else if path
                .extension()
                .is_some_and(|extension| extension == "eadl")
            {
                if let Ok(relative) = path.strip_prefix(root) {
                    out.push(relative.display().to_string());
                }
            }
        }
    }
    out.sort();
    out
}

/// Rule 4: a description in the repository that is neither in the suite nor excluded.
pub fn outside_violations(
    manifest: &Manifest,
    population: &[String],
    repository: &[String],
) -> Vec<String> {
    let mut out = Vec::new();
    for path in repository {
        if population.contains(path) || manifest.excludes(path).is_some() {
            continue;
        }
        out.push(format!(
            "`{path}` is a description in this repository that is in no suite root and matches no \
             exclusion, so nothing checks it — either the manifest gains a root for it, or an exclusion \
             with a reason, and silence is not one of the two"
        ));
    }
    out
}

/// The non-vacuity leg: an exclusion that matches nothing excludes nothing.
///
/// ⭐ The arm `docs/semantics/boundary/README.md` warns about, applied to the manifest: "a registry
/// entry nothing exercises is an assertion, not a rule". A mistyped prefix passes every other leg while
/// letting the excluded area into the suite, so each one is required to match at least one real
/// description.
pub fn vacuous_exclusion_violations(
    manifest: &Manifest,
    repository: &[String],
) -> Vec<(String, usize)> {
    manifest
        .exclusions
        .iter()
        .map(|exclusion| {
            let matched = repository
                .iter()
                .filter(|path| excluded_by(&exclusion.prefix, path))
                .count();
            (exclusion.prefix.clone(), matched)
        })
        .collect()
}

/// Rule 3: a suite file byte-identical to an excluded one.
///
/// ⭐ This is the leg that reaches what a path prefix cannot — a frozen reproducer *copied into* a
/// walked root, where its path is a suite path and rule 2 sees nothing wrong. It is usable because the
/// one legitimate identity the repository had is gone: `LS-002`'s evidence copy of
/// `examples/s0-heartbeat/system.eadl` was byte-identical to it until `M1.13.4.2` gave the example a
/// language-version identifier, and the copy stays a four-form reproduction because that is what the
/// issue it reproduces is about. A future legitimate duplicate has to be declared, not assumed.
pub fn identity_violations(
    suite: &[(String, String)],
    excluded: &[(String, String)],
) -> Vec<String> {
    let mut out = Vec::new();
    for (suite_path, suite_text) in suite {
        for (excluded_path, excluded_text) in excluded {
            if suite_text == excluded_text {
                out.push(format!(
                    "`{suite_path}` is byte-identical to the excluded `{excluded_path}`, so a frozen \
                     reproducer is a conformance case — the exclusion is a path prefix and cannot see a \
                     copy, which is what this leg is for. If the duplicate is deliberate, the manifest \
                     has to say so rather than leave two files to drift"
                ));
            }
        }
    }
    out
}
