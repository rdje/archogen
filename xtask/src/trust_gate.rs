//! The trust gate's baseline (leaf `M3.6.3.2`, `docs/specs/trust/decision_trust-inventory.md` §5): one form per item two
//! roots share, as proposed, and the host its digests were taken on.
//!
//! ⭐ **A proposal, never an acceptance** (§5, §14.4). The tool writes what an inventory measured — each shared item's
//! identity and the sha256 of every aspect the inventory records of it — and leaves the review's part, the item's
//! classification, the property it can affect, its residual risk and the controls that remain, `unstated` until a
//! reviewer writes them. The file has no place to write acceptance: which forms are accepted is read where reviews
//! are protected (`M3.6.5`), never from a form's presence.
//!
//! ⛔ **The digests are the host's.** The build configuration every pair shares holds the toolchain's identity, and
//! `rustc -vV` names its host, so a form's digests compare only on the host they were taken on. A proposal names the
//! host its inventory was built on, never another, and the record's baseline host is the CI runner's (§2): only a
//! proposal made there is the baseline the gate compares against (`TI-H3`).

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;

use archogen_evidence::sha256::Digest;
use eadl_front::{read, Form, SourceMap};

use crate::json::{self, Json};

/// Where the baseline lives (§5).
pub const BASELINE: &str = "trust/baseline.eadl";

/// What a review has not yet written.
pub const UNSTATED: &str = "unstated";

/// §4.4's classifications of a shared item.
pub const CLASSIFICATIONS: &[&str] = &[
    "infrastructure",
    "interpretation-normalization",
    "semantic-analysis",
    "authoritative-data",
    "reference-derivation",
];

/// A shared item's identity: the pair of roots, its kind, and what it is — a package's or a file's path, a copy's paths,
/// a harness's name, nothing for the build configuration (§4).
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ItemId {
    /// The two roots, in the inventory's order.
    pub pair: (String, String),
    /// `build-configuration`, `package`, `file`, `copy` or `comparison-harness`.
    pub kind: String,
    /// What it is, by kind.
    pub item: String,
}

impl ItemId {
    /// The form's name: the pair, the kind and the item, one line.
    #[must_use]
    pub fn name(&self) -> String {
        let base = format!("{}+{} {}", self.pair.0, self.pair.1, self.kind);
        if self.item.is_empty() {
            base
        } else {
            format!("{base} {}", self.item)
        }
    }
}

/// The key of an item's identity, by kind; every other key of the item is an aspect.
fn identity_key(kind: &str) -> Option<&'static str> {
    match kind {
        "package" => Some("package"),
        "file" => Some("file"),
        "copy" => Some("paths"),
        "comparison-harness" => Some("harness"),
        _ => None,
    }
}

/// Every shared item an inventory records, by identity, with the sha256 of each aspect it records — content,
/// configuration, edges, readers, a manifest, a file's bytes (§4).
///
/// # Errors
///
/// An item whose kind or identity the inventory does not state.
pub fn items(inventory: &Json) -> Result<BTreeMap<ItemId, BTreeMap<String, String>>, String> {
    let mut out = BTreeMap::new();
    for pair in inventory
        .get("shared")
        .map(Json::elements)
        .unwrap_or_default()
    {
        let names: Vec<&str> = pair
            .get("pair")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .filter_map(Json::as_str)
            .collect();
        let [a, b] = names.as_slice() else {
            return Err("an inventory pair names other than two roots".to_owned());
        };
        for item in pair.get("items").map(Json::elements).unwrap_or_default() {
            let Json::Object(members) = item else {
                return Err(format!("{a}+{b}: an item is not an object"));
            };
            let kind = item
                .get("kind")
                .and_then(Json::as_str)
                .ok_or_else(|| format!("{a}+{b}: an item states no kind"))?;
            let key = identity_key(kind);
            let identity = match key {
                None => String::new(),
                Some(k) => match item.get(k) {
                    Some(Json::Str(v)) => v.clone(),
                    Some(Json::Array(vs)) => vs
                        .iter()
                        .filter_map(Json::as_str)
                        .collect::<Vec<_>>()
                        .join(" "),
                    _ => return Err(format!("{a}+{b}: a `{kind}` item states no `{k}`")),
                },
            };
            let aspects = members
                .iter()
                .filter(|(k, _)| k.as_str() != "kind" && Some(k.as_str()) != key)
                .map(|(k, v)| (k.clone(), Digest::of(json::write(v).as_bytes()).hex()))
                .collect();
            let id = ItemId {
                pair: ((*a).to_owned(), (*b).to_owned()),
                kind: kind.to_owned(),
                item: identity,
            };
            if out.insert(id.clone(), aspects).is_some() {
                return Err(format!("`{}` is recorded twice", id.name()));
            }
        }
    }
    Ok(out)
}

/// One form of the baseline.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shared {
    /// What is shared.
    pub id: ItemId,
    /// Each aspect's sha256, as proposed.
    pub digests: BTreeMap<String, String>,
    /// §4.4's classification, or [`UNSTATED`].
    pub classification: String,
    /// The property the item can affect.
    pub property: String,
    /// Its residual common-error risk.
    pub risk: String,
    /// The independent controls that remain.
    pub controls: String,
}

/// The baseline, read.
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Baseline {
    /// The host its digests were taken on.
    pub host: String,
    /// Every form, by identity.
    pub forms: BTreeMap<ItemId, Shared>,
}

/// Each form the baseline holds, and the clauses it takes; any other form or clause is refused, so nothing can be
/// written in the file that the gate would read as acceptance.
const FORMS: &[(&str, &[&str])] = &[
    ("defbaseline", &["host"]),
    (
        "defshared",
        &[
            "pair",
            "kind",
            "item",
            "digest",
            "classification",
            "property",
            "risk",
            "controls",
        ],
    ),
];

fn text_of(form: &Form) -> Option<String> {
    match form {
        Form::Symbol { name, .. } => Some(name.clone()),
        Form::Str { value, .. } => Some(value.clone()),
        _ => None,
    }
}

fn clause_values(form: &Form, head: &str) -> Vec<String> {
    form.items()
        .iter()
        .skip(1)
        .find(|c| c.head() == Some(head))
        .map(|c| c.items().iter().skip(1).filter_map(text_of).collect())
        .unwrap_or_default()
}

fn single(form: &Form, head: &str, name: &str) -> Result<String, String> {
    match clause_values(form, head).as_slice() {
        [v] => Ok(v.clone()),
        _ => Err(format!(
            "{BASELINE}: `{name}` needs exactly one `({head} …)`"
        )),
    }
}

/// Read the baseline, strictly: one `defbaseline` naming the host; each `defshared` named by its identity, a name twice
/// refused, a classification other than §4.4's five or [`UNSTATED`] refused.
///
/// # Errors
///
/// A form the gate cannot read.
pub fn read_baseline(text: &str) -> Result<Baseline, String> {
    let mut sources = SourceMap::new();
    let id = sources
        .add(BASELINE, text)
        .map_err(|e| format!("{BASELINE}: {e:?}"))?;
    let (doc, diags) = read(&sources, id);
    if diags.has_errors() {
        return Err(diags.render(&sources));
    }
    let mut host = None;
    let mut forms = BTreeMap::new();
    for form in &doc.forms {
        let head = form.head().unwrap_or("?");
        let Some((_, allowed)) = FORMS.iter().find(|(h, _)| *h == head) else {
            return Err(format!(
                "{BASELINE}: a form `{head}` the gate does not know"
            ));
        };
        let named = head == "defshared";
        let name = if named {
            form.items().get(1).and_then(text_of).unwrap_or_default()
        } else {
            head.to_owned()
        };
        let mut seen = BTreeSet::new();
        for c in form.items().iter().skip(if named { 2 } else { 1 }) {
            let Some(h) = c.head() else {
                return Err(format!(
                    "{BASELINE}: `{name}` holds something that is not a clause"
                ));
            };
            if !allowed.contains(&h) {
                return Err(format!(
                    "{BASELINE}: `{name}` holds a clause `{h}` its form does not take"
                ));
            }
            if !seen.insert(h) {
                return Err(format!("{BASELINE}: `{name}` holds `{h}` twice"));
            }
        }
        if head == "defbaseline" {
            if host.is_some() {
                return Err(format!("{BASELINE}: the host is named twice"));
            }
            host = Some(single(form, "host", "the baseline")?);
            continue;
        }
        let pair = clause_values(form, "pair");
        let [a, b] = pair.as_slice() else {
            return Err(format!("{BASELINE}: `{name}` names `(pair ROOT ROOT)`"));
        };
        let item_values = clause_values(form, "item");
        let id = ItemId {
            pair: (a.clone(), b.clone()),
            kind: single(form, "kind", &name)?,
            item: match item_values.as_slice() {
                [] => String::new(),
                [v] => v.clone(),
                _ => return Err(format!("{BASELINE}: `{name}` names one item")),
            },
        };
        if id.name() != name {
            return Err(format!(
                "{BASELINE}: `{name}` is named otherwise than its item, `{}`",
                id.name()
            ));
        }
        let mut digests = BTreeMap::new();
        for d in clause_values(form, "digest") {
            let Some((aspect, sha)) = d.split_once('=') else {
                return Err(format!(
                    "{BASELINE}: `{name}`'s digest `{d}` is not `ASPECT=SHA256`"
                ));
            };
            digests.insert(aspect.to_owned(), sha.to_owned());
        }
        let classification = single(form, "classification", &name)?;
        if classification != UNSTATED && !CLASSIFICATIONS.contains(&classification.as_str()) {
            return Err(format!(
                "{BASELINE}: `{name}`'s classification `{classification}` is none of {} nor `{UNSTATED}`",
                CLASSIFICATIONS.join(", ")
            ));
        }
        let shared = Shared {
            digests,
            classification,
            property: single(form, "property", &name)?,
            risk: single(form, "risk", &name)?,
            controls: single(form, "controls", &name)?,
            id: id.clone(),
        };
        if forms.insert(id, shared).is_some() {
            return Err(format!("{BASELINE}: `{name}` is named twice"));
        }
    }
    let host =
        host.ok_or_else(|| format!("{BASELINE}: no `(defbaseline (host …))` names the host"))?;
    Ok(Baseline { host, forms })
}

fn quoted(s: &str) -> String {
    format!("\"{}\"", s.replace('\\', "\\\\").replace('"', "\\\""))
}

/// The baseline proposed for `inventory`: one form per shared item, its digests as measured, its review's part as
/// `existing` wrote it for the same item or [`UNSTATED`], and the host the inventory was built on (§5).
///
/// # Errors
///
/// An inventory whose shared items or host it does not state.
pub fn propose(inventory: &Json, existing: Option<&Baseline>) -> Result<String, String> {
    let host = inventory
        .get("identity")
        .and_then(|i| i.get("host"))
        .and_then(Json::as_str)
        .ok_or("the inventory names no host")?;
    let mut out = String::from(
        "; trust/baseline.eadl — one form per item two roots share, as `cargo xtask trust-baseline --propose` measured it\n\
         ; (docs/specs/trust/decision_trust-inventory.md §5, leaf M3.6.3.2). A proposal, never an acceptance: each\n\
         ; form's classification, property, risk and controls are the review's to write, and which forms are accepted\n\
         ; is read where reviews are protected (M3.6.5). The digests compare only on the host named below.\n\n",
    );
    out.push_str(&format!("(defbaseline (host {}))\n", quoted(host)));
    for (id, digests) in items(inventory)? {
        let prior = existing.and_then(|b| b.forms.get(&id));
        let review =
            |f: fn(&Shared) -> &String| prior.map_or(UNSTATED.to_owned(), |p| f(p).clone());
        out.push_str(&format!("\n(defshared {}\n", quoted(&id.name())));
        out.push_str(&format!(
            "  (pair {} {})\n",
            quoted(&id.pair.0),
            quoted(&id.pair.1)
        ));
        out.push_str(&format!("  (kind {})\n", id.kind));
        if !id.item.is_empty() {
            out.push_str(&format!("  (item {})\n", quoted(&id.item)));
        }
        let digest_list: Vec<String> = digests
            .iter()
            .map(|(k, v)| quoted(&format!("{k}={v}")))
            .collect();
        out.push_str(&format!("  (digest {})\n", digest_list.join(" ")));
        out.push_str(&format!(
            "  (classification {})\n",
            review(|p| &p.classification)
        ));
        out.push_str(&format!(
            "  (property {})\n",
            quoted(&review(|p| &p.property))
        ));
        out.push_str(&format!("  (risk {})\n", quoted(&review(|p| &p.risk))));
        out.push_str(&format!(
            "  (controls {}))\n",
            quoted(&review(|p| &p.controls))
        ));
    }
    Ok(out)
}

/// `cargo xtask trust-baseline --propose [--commit REV] [--out FILE]`: inventory the commit, then write the baseline
/// proposed for it, keeping the review's part of every form the commit's own baseline already holds.
pub fn run(repo: &Path, args: &[&str]) -> i32 {
    let (mut commit, mut out, mut propose_flag) = ("HEAD".to_owned(), repo.join(BASELINE), false);
    let mut i = 0;
    while i < args.len() {
        match (args[i], args.get(i + 1)) {
            ("--propose", _) => {
                propose_flag = true;
                i += 1;
            }
            ("--commit", Some(v)) => {
                commit = (*v).to_owned();
                i += 2;
            }
            ("--out", Some(v)) => {
                out = repo.join(v);
                i += 2;
            }
            (other, _) => {
                eprintln!("trust-baseline: unknown argument `{other}`; write `--propose [--commit REV] [--out FILE]`");
                return 2;
            }
        }
    }
    if !propose_flag {
        eprintln!("trust-baseline: the tool proposes and never accepts; write `--propose`");
        return 2;
    }
    let scratch = repo.join("target").join("trust");
    let inventory = match crate::trust::inventory(repo, &commit, &scratch) {
        Ok(crate::trust::Outcome::Written(inv)) => inv,
        Ok(crate::trust::Outcome::Refused(refusals)) => {
            for r in &refusals {
                eprintln!("{r}");
            }
            eprintln!(
                "trust-baseline: the inventory refused {} input(s); nothing proposed",
                refusals.len()
            );
            return 1;
        }
        Err(e) => {
            eprintln!("trust-baseline: {e}");
            return 2;
        }
    };
    let existing = match fs::read_to_string(repo.join(BASELINE)) {
        Ok(text) => match read_baseline(&text) {
            Ok(b) => Some(b),
            Err(e) => {
                eprintln!("trust-baseline: {e}");
                return 1;
            }
        },
        Err(_) => None,
    };
    match propose(&inventory, existing.as_ref()) {
        Ok(text) => {
            if let Err(e) = fs::write(&out, &text) {
                eprintln!("trust-baseline: cannot write {}: {e}", out.display());
                return 2;
            }
            let forms = read_baseline(&text).map(|b| b.forms.len()).unwrap_or(0);
            println!(
                "trust-baseline: proposed {} form(s) in {} — none accepted",
                forms,
                out.display()
            );
            0
        }
        Err(e) => {
            eprintln!("trust-baseline: {e}");
            2
        }
    }
}

// ── The gate (§6; leaf `M3.6.3.3`) ──────────────────────────────────────────────────────────────────────────────

/// The report's file, under the gate's output directory (§6).
pub const REPORT: &str = "report.txt";

/// The base commit, as the gate compares against it: the commit, or none for a root commit, and its baseline's forms
/// with the sha256 of the file they were read from, or none when it holds no baseline.
pub struct Base<'a> {
    /// The base commit.
    pub commit: Option<&'a str>,
    /// Its `trust/baseline.eadl`, read, and the file's sha256.
    pub baseline: Option<(&'a Baseline, String)>,
}

/// What the gate concluded: the refusals that fail it, and the two parts of its report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Judgement {
    /// Every refusal, its code first.
    pub refused: Vec<String>,
    /// The change against the base commit's forms: its lines, "unchanged", or "not compared" and why.
    pub change: Vec<String>,
    /// Every shared item, root form, classification and admission not accepted, and every program target the
    /// inventory reports unclassified.
    pub standing: Vec<String>,
}

/// The aspects whose digests differ between a form and the item measured now, by name.
fn differing(was: &BTreeMap<String, String>, now: &BTreeMap<String, String>) -> Vec<String> {
    was.keys()
        .chain(now.keys())
        .collect::<BTreeSet<_>>()
        .into_iter()
        .filter(|k| was.get(*k) != now.get(*k))
        .cloned()
        .collect()
}

/// Judge an inventory (§6): `own` is the commit's own baseline, `roots` its own roots file, `base` the base commit's
/// baseline. On the baseline's host — the inventory's, named by `own` — `trust-form-missing` and `trust-baseline-stale`
/// apply; the change part is compared only when the base commit's baseline names the inventory's host.
///
/// # Errors
///
/// An inventory that names no host, or whose items the gate cannot read.
pub fn judge(
    inventory: &Json,
    roots: &crate::trust::Roots,
    own: Option<&Baseline>,
    base: &Base,
) -> Result<Judgement, String> {
    let host = inventory
        .get("identity")
        .and_then(|i| i.get("host"))
        .and_then(Json::as_str)
        .filter(|h| !h.is_empty())
        .ok_or("the inventory names no host")?;
    let current = items(inventory)?;
    let strings = |key: &str| -> Vec<String> {
        inventory
            .get(key)
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .filter_map(Json::as_str)
            .map(str::to_owned)
            .collect()
    };
    // A program target the inventory reports unclassified: with no classification at all, a form missing; with one
    // whose role packages grew, reported for review (§2, §6).
    let mut unclassified = Vec::new();
    let mut formless = Vec::new();
    for t in inventory
        .get("program-targets")
        .map(Json::elements)
        .unwrap_or_default()
    {
        let field = |k: &str| t.get(k).and_then(Json::as_str).unwrap_or_default();
        let status = field("status");
        if !status.starts_with("trust-unclassified-program") {
            continue;
        }
        unclassified.push(status.to_owned());
        let classified = roots.classified.iter().any(|c| {
            c.package == field("package") && c.kind == field("kind") && c.target == field("name")
        });
        if !classified {
            formless.push(format!(
                "trust-form-missing: the {} `{}` of `{}` has no form in {} — classify it, or make it a root",
                field("kind"),
                field("name"),
                field("package"),
                crate::trust::ROOTS
            ));
        }
    }

    let mut refused = Vec::new();
    if let Some(own) = own.filter(|b| b.host == host) {
        for id in current.keys().filter(|id| !own.forms.contains_key(id)) {
            refused.push(format!(
                "trust-form-missing: `{}` is shared and {BASELINE} holds no form for it — commit the tool's proposal, `cargo xtask trust-baseline --propose`",
                id.name()
            ));
        }
        refused.extend(formless);
        for id in own.forms.keys().filter(|id| !current.contains_key(id)) {
            refused.push(format!(
                "trust-baseline-stale: {BASELINE}: `{}` is no longer shared — remove its form, so a sharing reintroduced is reviewed again",
                id.name()
            ));
        }
        for c in strings("classifications-unused") {
            refused.push(format!(
                "trust-baseline-stale: {}: {c}",
                crate::trust::ROOTS
            ));
        }
        for a in strings("admissions-unused") {
            refused.push(format!(
                "trust-baseline-stale: {}: the admission `{a}` admits no current site",
                crate::trust::ROOTS
            ));
        }
    }

    let change = match (base.commit, &base.baseline) {
        (None, _) => vec!["not compared — the commit has no parent to compare against".to_owned()],
        (Some(commit), None) => vec![format!(
            "not compared — the base commit `{commit}` holds no {BASELINE}"
        )],
        (Some(_), Some((b, _))) if b.host != host => vec![format!(
            "not compared — the base commit's baseline names the host `{}`, and this inventory was built on `{host}`",
            b.host
        )],
        (Some(_), Some((b, _))) => {
            let mut lines = Vec::new();
            for (id, now) in &current {
                match b.forms.get(id) {
                    None => lines.push(format!("trust-new-shared: `{}`", id.name())),
                    Some(form) => {
                        let moved = differing(&form.digests, now);
                        if !moved.is_empty() {
                            lines.push(format!(
                                "trust-shared-changed: `{}` — {}",
                                id.name(),
                                moved.join(", ")
                            ));
                        }
                    }
                }
            }
            for id in b.forms.keys().filter(|id| !current.contains_key(id)) {
                lines.push(format!("no longer shared: `{}`", id.name()));
            }
            if lines.is_empty() {
                lines.push("unchanged".to_owned());
            }
            lines
        }
    };

    // Nothing is accepted before `M3.6.5` reads acceptance, so every entry is unreviewed (§5).
    let mut standing = Vec::new();
    for id in current.keys() {
        let state = match own.and_then(|b| b.forms.get(id)) {
            None => "no form".to_owned(),
            Some(f) => format!("proposed, classification {}", f.classification),
        };
        standing.push(format!("shared item `{}`: {state}", id.name()));
    }
    for p in &roots.programs {
        standing.push(format!(
            "root form `{}`: {}, `{}`",
            p.name, p.role, p.package
        ));
    }
    for c in &roots.classified {
        standing.push(format!(
            "classification `{}`: the {} `{}` of `{}`",
            c.name, c.kind, c.target, c.package
        ));
    }
    for (a, n) in &roots.admissions {
        for _ in 0..*n {
            standing.push(format!("admission `{}`: {} {}", a.file, a.rule, a.sha256));
        }
    }
    standing.extend(unclassified);
    Ok(Judgement {
        refused,
        change,
        standing,
    })
}

/// The report (§6): the build identity, the inventory's and the baseline's sha256, the verdict, the refusals, then its
/// two parts.
#[must_use]
pub fn report(inventory: Option<(&Json, &str)>, base: &Base, j: &Judgement) -> String {
    let mut out =
        String::from("trust-gate report — docs/specs/trust/decision_trust-inventory.md §6\n");
    if let Some((inv, sha)) = inventory {
        let id = |k: &str| {
            inv.get("identity")
                .and_then(|i| i.get(k))
                .and_then(Json::as_str)
                .unwrap_or("")
                .lines()
                .next()
                .unwrap_or("")
                .to_owned()
        };
        out.push_str(&format!("commit: {}\ntree: {}\n", id("commit"), id("tree")));
        out.push_str(&format!(
            "host: {}\nrustc: {}\ncargo: {}\n",
            id("host"),
            id("rustc"),
            id("cargo")
        ));
        out.push_str(&format!("Cargo.lock: sha256 {}\n", id("Cargo.lock")));
        out.push_str(&format!("inventory: sha256 {sha}\n"));
    } else {
        out.push_str("inventory: none — refused before it was written\n");
    }
    match (base.commit, &base.baseline) {
        (Some(c), Some((b, sha))) => out.push_str(&format!(
            "baseline: sha256 {sha} — {BASELINE} of the base commit {c}, host {}\n",
            b.host
        )),
        (Some(c), None) => out.push_str(&format!(
            "baseline: none — the base commit {c} holds no {BASELINE}\n"
        )),
        (None, _) => out.push_str("baseline: none — the commit has no parent\n"),
    }
    if j.refused.is_empty() {
        out.push_str("verdict: passed\n");
    } else {
        out.push_str(&format!(
            "verdict: refused — {} refusal(s)\n",
            j.refused.len()
        ));
        for r in &j.refused {
            out.push_str(&format!("  {r}\n"));
        }
    }
    out.push_str("\n== the change, against the base commit's forms ==\n");
    for l in &j.change {
        out.push_str(&format!("{l}\n"));
    }
    out.push_str("\n== the standing list: nothing is accepted before M3.6.5, so every entry is unreviewed ==\n");
    for l in &j.standing {
        out.push_str(&format!("{l}\n"));
    }
    out
}

/// A file of a commit's tree, or none when the tree does not hold it.
fn blob_at(
    git: &crate::catalog_check::Git,
    rev: &str,
    path: &str,
) -> Result<Option<Vec<u8>>, String> {
    if git
        .text(&["ls-tree", "--name-only", rev, "--", path])?
        .trim()
        .is_empty()
    {
        return Ok(None);
    }
    git.bytes(&["cat-file", "blob", &format!("{rev}:{path}")])
        .map(Some)
}

/// Run the gate on `commit` against `base` in the repository at `repo`, scratch and report under `out`, the cargo
/// configurations on the build's path judged against `config_root`'s tracked copy (as [`crate::trust::inventory_with`]).
/// Returns the judgement and the report written.
///
/// # Errors
///
/// What kept the gate from judging: git, cargo or the file system did not answer, or a file of `trust/` did not read.
pub fn gate(
    repo: &Path,
    commit: &str,
    base: Option<&str>,
    out: &Path,
    config_root: &Path,
) -> Result<(Judgement, String), String> {
    let git = crate::catalog_check::Git::at(repo);
    let commit = git
        .text(&[
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{commit}^{{commit}}"),
        ])?
        .trim()
        .to_owned();
    let base = match base {
        Some(b) => Some(b.to_owned()),
        None => git
            .text(&[
                "rev-parse",
                "--verify",
                "-q",
                "--end-of-options",
                &format!("{commit}^1^{{commit}}"),
            ])
            .ok(),
    }
    .map(|b| {
        git.text(&[
            "rev-parse",
            "--verify",
            "--end-of-options",
            &format!("{}^{{commit}}", b.trim()),
        ])
        .map(|s| s.trim().to_owned())
    })
    .transpose()?;
    let read_at = |rev: &str| -> Result<Option<(Baseline, String)>, String> {
        blob_at(&git, rev, BASELINE)?
            .map(|bytes| {
                let text = String::from_utf8(bytes.clone())
                    .map_err(|_| format!("{BASELINE} of {rev} is not UTF-8"))?;
                Ok((read_baseline(&text)?, Digest::of(&bytes).hex()))
            })
            .transpose()
    };
    let own = read_at(&commit)?;
    let base_baseline = match &base {
        Some(b) => read_at(b)?,
        None => None,
    };
    let base_view = Base {
        commit: base.as_deref(),
        baseline: base_baseline.as_ref().map(|(b, sha)| (b, sha.clone())),
    };
    fs::create_dir_all(out).map_err(|e| format!("{}: {e}", out.display()))?;
    let (judgement, text) = match crate::trust::inventory_with(repo, &commit, out, config_root)? {
        crate::trust::Outcome::Refused(refusals) => {
            let j = Judgement {
                refused: refusals,
                change: vec!["not compared — the inventory was refused".to_owned()],
                standing: Vec::new(),
            };
            let mut text = format!("commit: {commit}\n");
            text.push_str(&report(None, &base_view, &j));
            (j, text)
        }
        crate::trust::Outcome::Written(inv) => {
            let bytes = fs::read(out.join("trust-dependencies.json"))
                .map_err(|e| format!("the inventory it wrote: {e}"))?;
            let roots_text = blob_at(&git, &commit, crate::trust::ROOTS)?
                .ok_or_else(|| format!("the commit holds no {}", crate::trust::ROOTS))?;
            let roots = crate::trust::read_roots(&String::from_utf8_lossy(&roots_text))?;
            let j = judge(&inv, &roots, own.as_ref().map(|(b, _)| b), &base_view)?;
            let text = report(Some((&inv, &Digest::of(&bytes).hex())), &base_view, &j);
            (j, text)
        }
    };
    fs::write(out.join(REPORT), &text)
        .map_err(|e| format!("{}: {e}", out.join(REPORT).display()))?;
    Ok((judgement, text))
}

/// `cargo xtask trust-gate [--commit REV] [--base REV] [--out DIR]`: exit 0 passed, 1 refused, 2 not judged.
pub fn run_gate(repo: &Path, args: &[&str]) -> i32 {
    let (mut commit, mut base, mut out) =
        ("HEAD".to_owned(), None, repo.join("target").join("trust"));
    let mut i = 0;
    while i < args.len() {
        match (args[i], args.get(i + 1)) {
            ("--commit", Some(v)) => commit = (*v).to_owned(),
            ("--base", Some(v)) => base = Some((*v).to_owned()),
            ("--out", Some(v)) => out = repo.join(v),
            (other, _) => {
                eprintln!("trust-gate: unknown argument `{other}`; write `[--commit REV] [--base REV] [--out DIR]`");
                return 2;
            }
        }
        i += 2;
    }
    match gate(repo, &commit, base.as_deref(), &out, repo) {
        Ok((j, _)) => {
            for r in &j.refused {
                eprintln!("{r}");
            }
            let first = j.change.first().map_or("", String::as_str);
            println!(
                "trust-gate: {} — the change: {}; {} standing entr{} unreviewed; report {}",
                if j.refused.is_empty() {
                    "passed"
                } else {
                    "refused"
                },
                if j.change.len() == 1 {
                    first.to_owned()
                } else {
                    format!("{} line(s)", j.change.len())
                },
                j.standing.len(),
                if j.standing.len() == 1 { "y" } else { "ies" },
                out.join(REPORT).display()
            );
            i32::from(!j.refused.is_empty())
        }
        Err(e) => {
            eprintln!("trust-gate: {e}");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{gate, items, judge, propose, read_baseline, Base, ItemId, UNSTATED};
    use crate::json::{self, Json};
    use crate::trust::read_roots;

    /// An inventory as `trust-inventory` writes one: every kind of shared item, on a host of its own.
    const INVENTORY: &str = r#"{"identity":{"host":"test-host-triple"},"shared":[
        {"pair":["gen","chk"],"items":[
            {"kind":"build-configuration","content":{"toolchain":"rustc x","profiles":[]}},
            {"kind":"package","package":"crates/lib","manifest":"aa","content":["crates/lib/src/lib.rs=bb"],
             "configuration":[["--edition=2021"],["--edition=2021"]],"edges":["gen crates/a normal"],"readers":[]},
            {"kind":"file","file":"docs/data.txt","sha256":"cc"},
            {"kind":"copy","sha256":"dd","paths":["crates/a/x.rs","crates/b/x.rs"]}]},
        {"pair":["model","impl"],"items":[
            {"kind":"comparison-harness","harness":"differential","content":["t.rs=ee"],"configuration":["--test"]}]}]}"#;

    #[test]
    fn every_kind_of_item_has_its_identity_and_its_aspects() {
        let inv = json::parse(INVENTORY).expect("an inventory");
        let all = items(&inv).expect("items");
        let names: Vec<String> = all.keys().map(ItemId::name).collect();
        assert_eq!(
            names,
            [
                "gen+chk build-configuration",
                "gen+chk copy crates/a/x.rs crates/b/x.rs",
                "gen+chk file docs/data.txt",
                "gen+chk package crates/lib",
                "model+impl comparison-harness differential",
            ]
        );
        let package = all.values().nth(3).expect("the package");
        assert_eq!(
            package.keys().collect::<Vec<_>>(),
            ["configuration", "content", "edges", "manifest", "readers"],
            "the aspects §4 compares, the identity apart"
        );
        assert!(package.values().all(|d| d.len() == 64));
    }

    #[test]
    fn a_proposal_reads_back_to_the_inventory_s_items_and_proposes_no_acceptance() {
        let inv = json::parse(INVENTORY).expect("an inventory");
        let text = propose(&inv, None).expect("a proposal");
        let baseline = read_baseline(&text).expect("the proposal reads");
        assert_eq!(
            baseline.host, "test-host-triple",
            "the host the digests were taken on"
        );
        let measured = items(&inv).expect("items");
        assert_eq!(baseline.forms.len(), measured.len());
        for (id, digests) in &measured {
            let form = &baseline.forms[id];
            assert_eq!(&form.digests, digests, "{}", id.name());
            for review in [
                &form.classification,
                &form.property,
                &form.risk,
                &form.controls,
            ] {
                assert_eq!(review, UNSTATED, "{}: the tool writes no review", id.name());
            }
        }
        // No clause can say a form is accepted: acceptance is read where reviews are protected (§5, M3.6.5).
        let accepted = text.replacen(
            "(controls \"unstated\"))",
            "(controls \"unstated\") (accepted yes))",
            1,
        );
        assert!(read_baseline(&accepted)
            .expect_err("refused")
            .contains("a clause `accepted`"));
    }

    #[test]
    fn a_regenerated_baseline_keeps_the_review_and_changes_nothing_on_an_unchanged_inventory() {
        let inv = json::parse(INVENTORY).expect("an inventory");
        let first = propose(&inv, None).expect("a proposal");
        let reviewed = first.replacen(
            "(classification unstated)\n  (property \"unstated\")",
            "(classification infrastructure)\n  (property \"none: a toolchain both roots trust\")",
            1,
        );
        let baseline = read_baseline(&reviewed).expect("the reviewed file reads");
        assert_eq!(propose(&inv, Some(&baseline)).expect("again"), reviewed);
    }

    // ── The gate (§6) ──────────────────────────────────────────────────────────────────────────────────────────

    const CONFIG: &str = r#"{"kind":"build-configuration","content":{"toolchain":"rustc x"}}"#;
    const LIB: &str = r#"{"kind":"package","package":"crates/lib","content":["crates/lib/src/lib.rs=bb"],"edges":["gen crates/a normal"]}"#;
    const FILE: &str = r#"{"kind":"file","file":"docs/data.txt","sha256":"cc"}"#;

    /// Two roots, a classified tool and one admission, as the gate reads a commit's roots file.
    const ROOTS_TEXT: &str = "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-packages \"crates/a\"))\n\
         (defroot chk (role scheduling-checker) (package \"crates/b\") (target lib) (role-packages \"crates/b\"))\n\
         (defprogram tool (package \"crates/t\") (target bin t) (reason \"a tool\") (role-packages \"crates/a\"))\n\
         (defadmit (file \"crates/a/src/main.rs\") (rule \"include_str\") (sha256 \"ab\") (reason \"r\"))\n";

    /// An inventory on `host`, its shared items, program targets and the two lists the gate refuses on the host.
    fn inv(host: &str, items: &[&str], targets: &[&str], unused: (&[&str], &[&str])) -> Json {
        let list = |xs: &[&str]| {
            xs.iter()
                .map(|x| format!("\"{x}\""))
                .collect::<Vec<_>>()
                .join(",")
        };
        json::parse(&format!(
            r#"{{"identity":{{"host":"{host}"}},"shared":[{{"pair":["gen","chk"],"items":[{}]}}],"program-targets":[{}],"classifications-unused":[{}],"admissions-unused":[{}]}}"#,
            items.join(","),
            targets.join(","),
            list(unused.0),
            list(unused.1)
        ))
        .expect("an inventory")
    }

    fn baseline_of(j: &Json) -> super::Baseline {
        read_baseline(&propose(j, None).expect("a proposal")).expect("it reads")
    }

    const UNCLASSIFIED: &str = r#"{"package":"crates/x","kind":"bin","name":"x","status":"trust-unclassified-program: the bin `x` of `crates/x` is neither a root nor classified in trust/roots.eadl"}"#;
    const GROWN: &str = r#"{"package":"crates/t","kind":"bin","name":"t","status":"trust-unclassified-program: `tool` compiles `crates/b` beside the role packages its classification reviewed"}"#;

    fn has(lines: &[String], text: &str) -> bool {
        lines.iter().any(|l| l.contains(text))
    }

    #[test]
    fn on_the_baseline_s_host_a_missing_form_and_a_stale_one_are_refused() {
        // TI-H4: the commit's own `trust/` covers its inventory, and holds nothing the inventory no longer has.
        let roots = read_roots(ROOTS_TEXT).expect("roots");
        let own = baseline_of(&inv("h", &[CONFIG, FILE], &[], (&[], &[])));
        let now = inv(
            "h",
            &[CONFIG, LIB],
            &[UNCLASSIFIED],
            (
                &["tool: the bin `t` of `crates/t` is no program target of the commit"],
                &["crates/a/src/main.rs:include_str:ab"],
            ),
        );
        let base = Base {
            commit: Some("b"),
            baseline: Some((&own, "s".to_owned())),
        };
        let j = judge(&now, &roots, Some(&own), &base).expect("judged");
        for want in [
            "trust-form-missing: `gen+chk package crates/lib` is shared",
            "trust-form-missing: the bin `x` of `crates/x` has no form",
            "trust-baseline-stale: trust/baseline.eadl: `gen+chk file docs/data.txt` is no longer shared",
            "trust-baseline-stale: trust/roots.eadl: tool: the bin `t`",
            "trust-baseline-stale: trust/roots.eadl: the admission `crates/a/src/main.rs:include_str:ab`",
        ] {
            assert!(has(&j.refused, want), "{want}: {:#?}", j.refused);
        }
        assert_eq!(j.refused.len(), 5, "{:#?}", j.refused);
        assert_eq!(
            j.change,
            [
                "trust-new-shared: `gen+chk package crates/lib`",
                "no longer shared: `gen+chk file docs/data.txt`"
            ]
        );
    }

    #[test]
    fn off_the_baseline_s_host_nothing_is_compared_and_only_the_host_free_refusals_apply() {
        // TI-H5: every item unreviewed, "not compared", neither `trust-form-missing` nor `trust-baseline-stale`.
        let roots = read_roots(ROOTS_TEXT).expect("roots");
        let elsewhere = baseline_of(&inv("other-host", &[CONFIG, FILE], &[], (&[], &[])));
        let now = inv(
            "h",
            &[CONFIG, LIB],
            &[UNCLASSIFIED],
            (&["tool: gone"], &["crates/a/src/main.rs:include_str:ab"]),
        );
        let base = Base {
            commit: Some("b"),
            baseline: Some((&elsewhere, "s".to_owned())),
        };
        for own in [Some(&elsewhere), None] {
            let j = judge(&now, &roots, own, &base).expect("judged");
            assert!(j.refused.is_empty(), "{:#?}", j.refused);
            assert_eq!(j.change.len(), 1);
            assert!(j.change[0].starts_with(
                "not compared — the base commit's baseline names the host `other-host`"
            ));
            assert!(has(
                &j.standing,
                "shared item `gen+chk package crates/lib`: no form"
            ));
            assert!(has(&j.standing, "trust-unclassified-program: the bin `x`"));
        }
        let none = Base {
            commit: Some("b"),
            baseline: None,
        };
        let j = judge(&now, &roots, None, &none).expect("judged");
        assert_eq!(
            j.change,
            ["not compared — the base commit `b` holds no trust/baseline.eadl"]
        );
    }

    #[test]
    fn the_change_part_names_what_moved_and_says_unchanged_when_nothing_did() {
        let roots = read_roots(ROOTS_TEXT).expect("roots");
        let before = baseline_of(&inv("h", &[CONFIG, LIB], &[], (&[], &[])));
        let base = Base {
            commit: Some("b"),
            baseline: Some((&before, "s".to_owned())),
        };
        let same = inv("h", &[CONFIG, LIB], &[], (&[], &[]));
        let j = judge(&same, &roots, Some(&before), &base).expect("judged");
        assert_eq!(j.change, ["unchanged"]);
        assert!(j.refused.is_empty(), "{:#?}", j.refused);
        let edited = LIB
            .replace("=bb", "=b2")
            .replace("normal\"]", "normal\",\"chk crates/b normal\"]");
        let moved = inv("h", &[CONFIG, &edited], &[], (&[], &[]));
        let j = judge(&moved, &roots, Some(&before), &base).expect("judged");
        assert_eq!(
            j.change,
            ["trust-shared-changed: `gen+chk package crates/lib` — content, edges"]
        );
        assert!(
            j.refused.is_empty(),
            "a change is reported, never refused: {:#?}",
            j.refused
        );
    }

    #[test]
    fn a_classification_that_grew_is_reported_in_the_standing_list_and_not_refused() {
        let roots = read_roots(ROOTS_TEXT).expect("roots");
        let own = baseline_of(&inv("h", &[CONFIG], &[], (&[], &[])));
        let base = Base {
            commit: Some("b"),
            baseline: Some((&own, "s".to_owned())),
        };
        let j = judge(
            &inv("h", &[CONFIG], &[GROWN], (&[], &[])),
            &roots,
            Some(&own),
            &base,
        )
        .expect("judged");
        assert!(j.refused.is_empty(), "{:#?}", j.refused);
        assert_eq!(j.change, ["unchanged"], "a standing entry is no change");
        assert!(has(
            &j.standing,
            "trust-unclassified-program: `tool` compiles `crates/b`"
        ));
        for want in [
            "shared item `gen+chk build-configuration`: proposed, classification unstated",
            "root form `gen`: generator, `crates/a`",
            "root form `chk`: scheduling-checker, `crates/b`",
            "classification `tool`: the bin `t` of `crates/t`",
            "admission `crates/a/src/main.rs`: include_str ab",
        ] {
            assert!(has(&j.standing, want), "{want}: {:#?}", j.standing);
        }
    }

    #[test]
    fn the_gate_over_a_scratch_repository_s_commits() {
        // From a commit with no baseline to one whose forms cover it, then an unrelated commit, a shared source
        // edited and a new sharing — each judged against its parent by the real instrument.
        use crate::trust::tests::{manifest, real_root, two_roots};
        let dep = "[dependencies]\ncommon = { path = \"../common\" }\n";
        let f = two_roots(
            "gate-commits",
            "fn main() { common::c(); }\n",
            "pub fn f() -> u32 { common::c() }\n",
            &[
                ("crates/common/Cargo.toml", manifest("common", "")),
                (
                    "crates/common/src/lib.rs",
                    "pub fn c() -> u32 { 1 }\n".to_owned(),
                ),
                ("crates/a/Cargo.toml", manifest("a", dep)),
                ("crates/b/Cargo.toml", manifest("b", dep)),
            ],
        );
        let run = |out: &str| {
            gate(&f.repo, "HEAD", None, &f.base.join(out), &real_root()).expect("judged")
        };
        let (j, text) = run("g1");
        assert!(
            j.refused.is_empty(),
            "no baseline, so off its host: {:#?}",
            j.refused
        );
        assert_eq!(
            j.change,
            ["not compared — the commit has no parent to compare against"]
        );
        assert!(has(
            &j.standing,
            "shared item `gen+chk package crates/common`: no form"
        ));
        for header in [
            "commit: ",
            "host: ",
            "inventory: sha256 ",
            "baseline: none",
            "verdict: passed",
        ] {
            assert!(text.contains(header), "{header}: {text}");
        }
        assert_eq!(
            std::fs::read_to_string(f.base.join("g1/report.txt")).expect("written"),
            text
        );

        let measured = json::parse(
            &std::fs::read_to_string(f.base.join("g1/trust-dependencies.json"))
                .expect("the inventory"),
        )
        .expect("parses");
        f.commit(&[(
            "trust/baseline.eadl",
            propose(&measured, None).expect("a proposal"),
        )]);
        let (j, _) = run("g2");
        assert!(
            j.refused.is_empty(),
            "on its host, every form present: {:#?}",
            j.refused
        );
        assert!(
            j.change[0].contains("holds no trust/baseline.eadl"),
            "{:?}",
            j.change
        );

        f.commit(&[("README.md", "a change outside every root\n".to_owned())]);
        let (j, text) = run("g3");
        assert!(j.refused.is_empty(), "{:#?}", j.refused);
        assert_eq!(j.change, ["unchanged"], "case 5");
        assert!(text.contains("baseline: sha256 "), "{text}");

        f.commit(&[(
            "crates/common/src/lib.rs",
            "pub fn c() -> u32 { 2 }\n".to_owned(),
        )]);
        let (j, _) = run("g4");
        assert!(
            j.refused.is_empty(),
            "a change is reported: {:#?}",
            j.refused
        );
        assert!(
            has(
                &j.change,
                "trust-shared-changed: `gen+chk package crates/common` — content"
            ),
            "{:?}",
            j.change
        );

        let dep2 =
            "[dependencies]\ncommon = { path = \"../common\" }\nmore = { path = \"../more\" }\n";
        f.commit(&[
            ("crates/more/Cargo.toml", manifest("more", "")),
            ("crates/more/src/lib.rs", "pub fn m() {}\n".to_owned()),
            ("crates/a/Cargo.toml", manifest("a", dep2)),
            ("crates/b/Cargo.toml", manifest("b", dep2)),
        ]);
        let (j, text) = run("g5");
        assert!(
            has(
                &j.refused,
                "trust-form-missing: `gen+chk package crates/more`"
            ),
            "{:#?}",
            j.refused
        );
        assert!(
            has(&j.change, "trust-new-shared: `gen+chk package crates/more`"),
            "{:?}",
            j.change
        );
        assert!(text.contains("verdict: refused"), "{text}");
    }

    #[test]
    fn the_baseline_is_read_strictly() {
        let inv = json::parse(INVENTORY).expect("an inventory");
        let text = propose(&inv, None).expect("a proposal");
        for (mutated, why) in [
            (
                text.replacen("(defbaseline (host \"test-host-triple\"))\n", "", 1),
                "no `(defbaseline",
            ),
            (
                format!("{text}(defbaseline (host \"x\"))\n"),
                "the host is named twice",
            ),
            (
                text.replacen("(classification unstated)", "(classification trusted)", 1),
                "none of infrastructure",
            ),
            (
                text.replacen("\"gen+chk build-configuration\"", "\"other\"", 1),
                "named otherwise than its item",
            ),
            (format!("{text}(defroot x)\n"), "a form `defroot`"),
            (
                text.replacen(
                    "(kind build-configuration)",
                    "(kind build-configuration) (kind file)",
                    1,
                ),
                "twice",
            ),
        ] {
            let err = read_baseline(&mutated).expect_err(why);
            assert!(err.contains(why), "{why}: {err}");
        }
    }
}
