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

#[cfg(test)]
mod tests {
    use super::{items, propose, read_baseline, ItemId, UNSTATED};
    use crate::json;

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
