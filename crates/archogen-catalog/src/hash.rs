//! Hashes (the record's §3): a normative byte grammar.
//!
//! Every hash is SHA-256 over ASCII lines, each ending in a line feed. Each facet has an **own** hash, over its
//! forms and its own source set, and a **bound** hash, over its own hash and every derived line: the bound hashes
//! of the facets it rests on, the files its packages reach, its targets' files and its ledger sections. A record's
//! hash is over its four bound hashes, and a review's ledger hash over its form.
//!
//! [`Catalog::hashes`] computes all of them over a [`Tree`] given in memory, and refuses what §3 and §2's locator
//! rules refuse along the way. The worked example (`decision_catalog-records-example.md`) is its acceptance.

use std::collections::{BTreeMap, BTreeSet};

use archogen_evidence::sha256::Digest;
use eadl_front::Form;

use crate::grammar;
use crate::manifest::{self, Value};
use crate::record::{Content, Cost, FacetKind, Fact, FactValue, Locator, Record, Review, Targets};
use crate::refusal::{Code, Refusal};
use crate::tree::{self, Tree};

/// The identifier of this grammar and of the rules (§3, §5). Every hash begins with it.
pub const IDENTIFIER: &str = "archogen-catalog/1";

/// The source ledger, whose sections `ledger` locators name (§2).
pub const LEDGER: &str = "docs/book/src/ledger.md";

/// `E`, the encoding of a form (§3): independent of any printer.
#[must_use]
pub fn encode(form: &Form) -> String {
    match form {
        Form::Symbol { name, .. } => name.clone(),
        Form::Integer { value, .. } => value.to_string(),
        Form::Decimal { value, scale, .. } => format!("{value}e-{scale}"),
        Form::Str { value, .. } => {
            let mut out = String::with_capacity(value.len() + 2);
            out.push('"');
            for c in value.chars() {
                match c {
                    '\\' => out.push_str("\\\\"),
                    '"' => out.push_str("\\\""),
                    other => out.push(other),
                }
            }
            out.push('"');
            out
        }
        Form::List { items, .. } => {
            let inner: Vec<String> = items.iter().map(encode).collect();
            format!("({})", inner.join(" "))
        }
    }
}

/// The SHA-256 of `lines`, each followed by one line feed.
#[must_use]
pub fn hash_lines(lines: &[String]) -> Digest {
    let mut text = String::new();
    for line in lines {
        text.push_str(line);
        text.push('\n');
    }
    Digest::of(text.as_bytes())
}

/// The contract's forms, in record order (§3).
const CONTRACT_FIELDS: [&str; 10] = [
    "version",
    "catalog",
    "source",
    "maintainer",
    "depends",
    "supersedes",
    "profiles",
    "targets",
    "preconditions",
    "guarantees",
];

/// A facet's forms (§3): the contract's ten fields, or the other facet's one form.
#[must_use]
pub fn facet_forms(record: &Record, facet: FacetKind) -> Vec<&Form> {
    let fields = &record.form.items()[2..];
    match facet {
        FacetKind::Contract => fields
            .iter()
            .filter(|f| f.head().is_some_and(|h| CONTRACT_FIELDS.contains(&h)))
            .collect(),
        other => fields
            .iter()
            .filter(|f| f.head() == Some(other.as_str()))
            .collect(),
    }
}

/// A facet's forms hash, the item §5 matches content by: its forms with every `version` form removed, and for a
/// contract its `source`, `maintainer` and `supersedes` forms too, under `identifier`.
#[must_use]
pub fn forms_hash(identifier: &str, record: &Record, facet: FacetKind) -> Digest {
    let mut lines = vec![identifier.to_owned(), format!("forms {}", facet.as_str())];
    for form in facet_forms(record, facet) {
        match form.head() {
            Some("version" | "source" | "maintainer" | "supersedes")
                if facet == FacetKind::Contract => {}
            _ if facet == FacetKind::Contract => lines.push(encode(form)),
            _ => {
                let kept: Vec<Form> = form
                    .items()
                    .iter()
                    .filter(|sub| sub.head() != Some("version"))
                    .cloned()
                    .collect();
                lines.push(encode(&Form::List {
                    items: kept,
                    span: form.span(),
                }));
            }
        }
    }
    hash_lines(&lines)
}

/// A review's ledger hash (§3): over `archogen-catalog/1`, `review R` and `E` of its form.
#[must_use]
pub fn review_ledger_hash(record_id: &str, review: &Review) -> Digest {
    hash_lines(&[
        IDENTIFIER.to_owned(),
        format!("review {record_id}"),
        encode(&review.form),
    ])
}

/// Every hash of one facet, with what went into it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FacetHash {
    /// The own hash.
    pub own: Digest,
    /// The bound hash.
    pub bound: Digest,
    /// The own source set, sorted.
    pub own_set: Vec<String>,
    /// The reached set, sorted, without the own set's files.
    pub reached: Vec<String>,
    /// The derived lines, sorted and without duplicates.
    pub derived: Vec<String>,
}

/// Every hash of a catalog.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Hashes {
    /// Each facet of each record.
    pub facets: BTreeMap<(String, FacetKind), FacetHash>,
    /// Each record's hash.
    pub records: BTreeMap<String, Digest>,
}

impl Hashes {
    /// One facet's hashes.
    #[must_use]
    pub fn facet(&self, id: &str, facet: FacetKind) -> Option<&FacetHash> {
        self.facets.get(&(id.to_owned(), facet))
    }
}

/// Records over a tracked set: what §3's hashes are computed over.
#[derive(Debug, Clone, Default)]
pub struct Catalog {
    /// The tracked set.
    pub tree: Tree,
    /// Every record, by id.
    pub records: BTreeMap<String, Record>,
}

impl Catalog {
    /// A catalog of these records over this tree.
    #[must_use]
    pub fn new(tree: Tree, records: impl IntoIterator<Item = Record>) -> Self {
        Self {
            tree,
            records: records.into_iter().map(|r| (r.id.clone(), r)).collect(),
        }
    }

    /// The named targets (§2): stems with both `targets/<t>.env` and `targets/<t>.eadl`, directly under
    /// `targets/`.
    ///
    /// # Errors
    ///
    /// `catalog-field` for a `.env` or `.eadl` directly under `targets/` without its pair.
    pub fn named_targets(&self) -> Result<Vec<String>, Refusal> {
        let mut stems: BTreeMap<String, (bool, bool)> = BTreeMap::new();
        for path in self.tree.under("targets") {
            let Some(name) = path.strip_prefix("targets/").filter(|n| !n.contains('/')) else {
                continue;
            };
            if let Some(stem) = name.strip_suffix(".env") {
                stems.entry(stem.to_owned()).or_default().0 = true;
            } else if let Some(stem) = name.strip_suffix(".eadl") {
                stems.entry(stem.to_owned()).or_default().1 = true;
            }
        }
        let mut out = Vec::new();
        for (stem, pair) in stems {
            if pair != (true, true) {
                return Err(Refusal::new(
                    Code::Field,
                    &format!("targets/{stem}"),
                    "(file)",
                    None,
                    "a `.env` or `.eadl` directly under `targets/` without its pair",
                ));
            }
            if grammar::is_target_stem(&stem) {
                out.push(stem);
            }
        }
        Ok(out)
    }

    /// Every hash of every record (§3).
    ///
    /// # Errors
    ///
    /// The first refusal met: an unresolved or unmatched reference or a cycle (`catalog-dependency`), a source set
    /// or package rule (`catalog-source`), a target or locator outside §2 (`catalog-field`, `catalog-locator`).
    pub fn hashes(&self) -> Result<Hashes, Refusal> {
        let targets = self.named_targets()?;
        let mut computer = Computer {
            catalog: self,
            targets,
            hashes: Hashes::default(),
            stack: Vec::new(),
        };
        for record in self.records.values() {
            computer.references(record)?;
        }
        for (id, record) in &self.records {
            for facet in FacetKind::ALL {
                computer.bound(id, facet)?;
            }
            let bound = |facet: FacetKind| {
                computer
                    .hashes
                    .facet(id, facet)
                    .map(|h| h.bound.to_string())
                    .unwrap_or_default()
            };
            let lines = vec![
                IDENTIFIER.to_owned(),
                format!("record {id}"),
                format!("contract {}", bound(FacetKind::Contract)),
                format!("implementation {}", bound(FacetKind::Implementation)),
                format!("behavior-model {}", bound(FacetKind::BehaviorModel)),
                format!("timing-model {}", bound(FacetKind::TimingModel)),
            ];
            computer
                .hashes
                .records
                .insert(record.id.clone(), hash_lines(&lines));
        }
        Ok(computer.hashes)
    }
}

struct Computer<'a> {
    catalog: &'a Catalog,
    targets: Vec<String>,
    hashes: Hashes,
    stack: Vec<(String, FacetKind)>,
}

/// A record's sources for a facet, and its facts and costs, whatever its content.
fn parts(record: &Record, facet: FacetKind) -> (Vec<String>, Vec<&Fact>, Vec<&Cost>) {
    match facet {
        FacetKind::Contract => (Vec::new(), Vec::new(), Vec::new()),
        FacetKind::Implementation => match &record.implementation.content {
            Content::Present(p) => (p.0.clone(), Vec::new(), Vec::new()),
            Content::None(_) => (Vec::new(), Vec::new(), Vec::new()),
        },
        FacetKind::BehaviorModel => match &record.behavior_model.content {
            Content::Present(m) => (m.sources.clone(), m.facts.iter().collect(), Vec::new()),
            Content::None(_) => (Vec::new(), Vec::new(), Vec::new()),
        },
        FacetKind::TimingModel => match &record.timing_model.content {
            Content::Present(m) => (
                m.sources.clone(),
                m.facts.iter().collect(),
                m.costs.iter().collect(),
            ),
            Content::None(_) => (Vec::new(), Vec::new(), Vec::new()),
        },
    }
}

fn locators<'r>(facts: &[&'r Fact], costs: &[&'r Cost]) -> Vec<(&'r Locator, String)> {
    let mut out = Vec::new();
    for fact in facts {
        if let FactValue::Known { locator, .. } = &fact.value {
            out.push((locator, format!("fact[{}]", fact.name)));
        }
    }
    for cost in costs {
        if let Ok(known) = &cost.value {
            if let Some(locator) = &known.locator {
                out.push((locator, format!("cost[{}]", cost.name)));
            }
        }
    }
    out
}

impl<'a> Computer<'a> {
    fn record(&self, id: &str) -> Option<&'a Record> {
        self.catalog.records.get(id)
    }

    /// Every reference resolves and matches (§2): dependencies, `describes`, `measured-with`; every target a
    /// contract names exists, and every cost's target is one its contract admits.
    fn references(&self, record: &Record) -> Result<(), Refusal> {
        let refuse = |field: &str, message: String| {
            Refusal::new(Code::Dependency, &record.path, field, None, message)
        };
        for dependency in &record.contract.depends {
            let Some(other) = self.record(&dependency.id) else {
                return Err(refuse(
                    "depends",
                    format!("`{}` resolves to no record", dependency.id),
                ));
            };
            if !dependency.requirement.matches(other.contract.version) {
                return Err(refuse(
                    "depends",
                    format!(
                        "`{}` is at {}, which `\"{}\"` does not match",
                        dependency.id, other.contract.version, dependency.requirement
                    ),
                ));
            }
        }
        let (_, _, costs) = parts(record, FacetKind::TimingModel);
        let describes = match &record.behavior_model.content {
            Content::Present(m) => m.describes.clone(),
            Content::None(_) => Vec::new(),
        };
        let measured = match &record.timing_model.content {
            Content::Present(m) => m.measured_with.clone(),
            Content::None(_) => Vec::new(),
        };
        for (field, id) in describes
            .iter()
            .map(|d| ("behavior-model describes", d))
            .chain(measured.iter().map(|m| ("timing-model measured-with", m)))
        {
            if self.record(id).is_none() {
                return Err(refuse(field, format!("`{id}` resolves to no record")));
            }
        }
        if let Targets::Named(named) = &record.contract.targets {
            for t in named {
                if !self.targets.contains(t) {
                    return Err(Refusal::new(
                        Code::Field,
                        &record.path,
                        "targets",
                        None,
                        format!("`{t}` is not a target under `targets/`"),
                    ));
                }
            }
        }
        for cost in costs {
            let admitted = match &record.contract.targets {
                Targets::Any => self.targets.contains(&cost.target),
                Targets::Named(named) => named.contains(&cost.target),
            };
            if !admitted {
                return Err(Refusal::new(
                    Code::Field,
                    &record.path,
                    &format!("timing-model cost[{}]", cost.name),
                    None,
                    format!(
                        "a cost on `{}`, a target the contract does not admit",
                        cost.target
                    ),
                ));
            }
        }
        Ok(())
    }

    /// The dependency closure of a record: its dependencies, transitively (§3).
    fn closure(&self, id: &str) -> BTreeSet<String> {
        let mut seen = BTreeSet::new();
        let mut stack = vec![id.to_owned()];
        while let Some(current) = stack.pop() {
            if let Some(record) = self.record(&current) {
                for d in &record.contract.depends {
                    if seen.insert(d.id.clone()) {
                        stack.push(d.id.clone());
                    }
                }
            }
        }
        seen.remove(id);
        seen
    }

    fn bound(&mut self, id: &str, facet: FacetKind) -> Result<Digest, Refusal> {
        let key = (id.to_owned(), facet);
        if let Some(done) = self.hashes.facets.get(&key) {
            return Ok(done.bound);
        }
        let record = self.record(id).ok_or_else(|| {
            Refusal::new(
                Code::Dependency,
                "catalog",
                facet.as_str(),
                None,
                format!("`{id}` resolves to no record"),
            )
        })?;
        if self.stack.contains(&key) {
            return Err(Refusal::new(
                Code::Dependency,
                &record.path,
                facet.as_str(),
                None,
                format!(
                    "a cycle through {} of `{id}` in §3's derived lines",
                    facet.as_str()
                ),
            ));
        }
        self.stack.push(key.clone());
        let result = self.compute(record, facet);
        self.stack.pop();
        let hash = result?;
        let bound = hash.bound;
        self.hashes.facets.insert(key, hash);
        Ok(bound)
    }

    fn compute(&mut self, record: &Record, facet: FacetKind) -> Result<FacetHash, Refusal> {
        let id = record.id.as_str();
        let (sources, facts, costs) = parts(record, facet);
        let sets = Sets::of(&self.catalog.tree, &record.path, facet, &sources)?;
        let mut own_lines = vec![
            IDENTIFIER.to_owned(),
            format!("own {} {id}", facet.as_str()),
        ];
        own_lines.extend(facet_forms(record, facet).into_iter().map(encode));
        for path in &sets.own {
            own_lines.push(format!("file {path} {}", self.file_hash(path)));
        }
        let own = hash_lines(&own_lines);

        let mut derived: BTreeSet<String> = BTreeSet::new();
        let mut line = |this: &mut Self, kind: FacetKind, other: &str| -> Result<(), Refusal> {
            let bound = this.bound(other, kind)?;
            derived.insert(format!("{} {other} {bound}", kind.as_str()));
            Ok(())
        };
        match facet {
            FacetKind::Contract => {
                for d in record.contract.depends.clone() {
                    line(self, FacetKind::Contract, &d.id)?;
                }
            }
            FacetKind::Implementation => {}
            FacetKind::BehaviorModel => {
                line(self, FacetKind::Contract, id)?;
                line(self, FacetKind::Implementation, id)?;
                if let Content::Present(m) = &record.behavior_model.content {
                    for x in m.describes.clone() {
                        line(self, FacetKind::Implementation, &x)?;
                    }
                }
                for d in record.contract.depends.clone() {
                    line(self, FacetKind::BehaviorModel, &d.id)?;
                }
            }
            FacetKind::TimingModel => {
                line(self, FacetKind::Contract, id)?;
                let mut implementations: BTreeSet<String> = self.closure(id);
                implementations.insert(id.to_owned());
                let measured = match &record.timing_model.content {
                    Content::Present(m) => m.measured_with.clone(),
                    Content::None(_) => Vec::new(),
                };
                for m in &measured {
                    implementations.insert(m.clone());
                    implementations.extend(self.closure(m));
                }
                for x in implementations {
                    line(self, FacetKind::Implementation, &x)?;
                }
                for d in record
                    .contract
                    .depends
                    .iter()
                    .map(|d| d.id.clone())
                    .chain(measured)
                {
                    line(self, FacetKind::TimingModel, &d)?;
                }
            }
        }
        let target_names: Vec<String> = match facet {
            FacetKind::BehaviorModel => match &record.contract.targets {
                Targets::Any => self.targets.clone(),
                Targets::Named(named) => named.clone(),
            },
            FacetKind::TimingModel => costs
                .iter()
                .map(|c| c.target.clone())
                .collect::<BTreeSet<_>>()
                .into_iter()
                .collect(),
            _ => Vec::new(),
        };
        for t in &target_names {
            for path in target_files(&self.catalog.tree, t)? {
                derived.insert(format!("target {path} {}", self.file_hash(&path)));
            }
        }
        for (locator, field) in locators(&facts, &costs) {
            self.check_locator(record, facet, &sets, locator, &field)?;
            if let Locator::Ledger { anchor, .. } = locator {
                let section = ledger_section(&self.catalog.tree, anchor)
                    .map_err(|why| Refusal::new(Code::Locator, &record.path, &field, None, why))?;
                derived.insert(format!(
                    "ledger {anchor} {}",
                    Digest::of(section.as_bytes())
                ));
            }
        }
        for path in &sets.reached {
            derived.insert(format!("file {path} {}", self.file_hash(path)));
        }
        let derived: Vec<String> = derived.into_iter().collect();
        let mut bound_lines = vec![
            IDENTIFIER.to_owned(),
            format!("bound {} {id}", facet.as_str()),
            format!("own {own}"),
        ];
        bound_lines.extend(derived.iter().cloned());
        Ok(FacetHash {
            own,
            bound: hash_lines(&bound_lines),
            own_set: sets.own,
            reached: sets.reached,
            derived,
        })
    }

    fn file_hash(&self, path: &str) -> Digest {
        Digest::of(self.catalog.tree.get(path).unwrap_or_default())
    }

    /// §2's locator rule: a `file` locator names a file of the facet's own set; a `code` locator a file of an
    /// allowed record's implementation own set.
    fn check_locator(
        &mut self,
        record: &Record,
        facet: FacetKind,
        sets: &Sets,
        locator: &Locator,
        field: &str,
    ) -> Result<(), Refusal> {
        let refuse =
            |message: String| Refusal::new(Code::Locator, &record.path, field, None, message);
        match locator {
            Locator::File(path) => {
                if !sets.own.contains(path) {
                    return Err(refuse(format!(
                        "`{path}` is not in the facet's own source set"
                    )));
                }
            }
            Locator::Code { id, path } => {
                let allowed = match facet {
                    FacetKind::BehaviorModel => {
                        id == &record.id
                            || matches!(&record.behavior_model.content, Content::Present(m) if m.describes.contains(id))
                    }
                    FacetKind::TimingModel => {
                        id == &record.id
                            || self.closure(&record.id).contains(id)
                            || matches!(&record.timing_model.content, Content::Present(m) if m.measured_with.contains(id))
                    }
                    _ => false,
                };
                if !allowed {
                    return Err(refuse(format!(
                        "a `code` locator into `{id}`, which §2 does not allow for this facet"
                    )));
                }
                self.bound(id, FacetKind::Implementation)?;
                let own = &self
                    .hashes
                    .facet(id, FacetKind::Implementation)
                    .map(|h| h.own_set.clone())
                    .unwrap_or_default();
                if !own.contains(path) {
                    return Err(refuse(format!(
                        "`{path}` is not in `{id}`'s implementation own set"
                    )));
                }
            }
            Locator::Ledger { .. } => {}
        }
        Ok(())
    }
}

/// A target's files (§3): its `.env`, its `.eadl`, and every path its `.env` gives as a value.
///
/// # Errors
///
/// `catalog-field` for a `.env` outside §3's grammar, a path value untracked or outside §4's form, or a value
/// without `/` that names a tracked file.
pub fn target_files(tree: &Tree, target: &str) -> Result<Vec<String>, Refusal> {
    let env = format!("targets/{target}.env");
    let bytes = tree.get(&env).unwrap_or_default();
    let refuse = |line: usize, message: String| {
        Refusal::new(Code::Field, &env, &format!("line {line}"), None, message)
    };
    let text = core::str::from_utf8(bytes).map_err(|_| refuse(0, "a `.env` is ASCII".into()))?;
    let mut files = vec![env.clone(), format!("targets/{target}.eadl")];
    let mut keys: Vec<&str> = Vec::new();
    for (n, line) in text.split('\n').enumerate() {
        let n = n + 1;
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some((key, value)) = line.split_once('=') else {
            return Err(refuse(
                n,
                "a `.env` line is blank, a comment or `KEY=value`".into(),
            ));
        };
        let prefixed = [
            "TARGET_",
            "QEMU_",
            "RUST_",
            "REQUIRES_",
            "DEVICE_",
            "PLATFORM_",
        ]
        .iter()
        .any(|p| key.strip_prefix(p).is_some_and(|rest| !rest.is_empty()));
        if !prefixed
            || !key
                .bytes()
                .all(|b| b.is_ascii_uppercase() || b.is_ascii_digit() || b == b'_')
        {
            return Err(refuse(n, format!("`{key}` is not a key of §3's grammar")));
        }
        if keys.contains(&key) {
            return Err(refuse(n, format!("`{key}` appears twice")));
        }
        keys.push(key);
        if !value.bytes().all(|b| {
            b.is_ascii_alphanumeric() || matches!(b, b'.' | b'_' | b'-' | b'/' | b'+' | b':' | b',')
        }) {
            return Err(refuse(
                n,
                format!("`{key}`'s value holds a character outside §3's grammar"),
            ));
        }
        if value.contains('/') {
            if !grammar::is_path(value) || !tree.is_file(value) {
                return Err(refuse(
                    n,
                    format!("`{value}` is not a tracked path in normal form"),
                ));
            }
            if value.starts_with("catalog/") {
                return Err(refuse(n, format!("`{value}` is under `catalog/`")));
            }
            files.push(value.to_owned());
        } else if !value.is_empty()
            && (tree.is_file(value) || tree.is_file(&format!("targets/{value}")))
        {
            return Err(refuse(
                n,
                format!("`{value}` names a tracked file without its path"),
            ));
        }
    }
    files.sort();
    files.dedup();
    Ok(files)
}

/// A ledger section (§2): from the heading ``## `<anchor>` `` up to the next line beginning `## `, or the end.
///
/// # Errors
///
/// When the ledger holds no such heading, or holds it twice.
pub fn ledger_section(tree: &Tree, anchor: &str) -> Result<String, String> {
    let text = core::str::from_utf8(tree.get(LEDGER).unwrap_or_default())
        .map_err(|_| format!("`{LEDGER}` is not UTF-8"))?;
    let heading = format!("## `{anchor}`");
    let mut starts = Vec::new();
    let mut offset = 0;
    for line in text.split_inclusive('\n') {
        if line.trim_end_matches('\n') == heading {
            starts.push(offset);
        }
        offset += line.len();
    }
    let [start] = starts.as_slice() else {
        return Err(format!(
            "the ledger holds the anchor `{anchor}` {} times, and it must hold it once",
            starts.len()
        ));
    };
    let rest = &text[*start..];
    let mut end = rest.len();
    let mut offset = 0;
    for (i, line) in rest.split_inclusive('\n').enumerate() {
        if i > 0 && line.starts_with("## ") {
            end = offset;
            break;
        }
        offset += line.len();
    }
    Ok(rest[..end].to_owned())
}

/// A facet's own and reached sets (§3).
#[derive(Debug, Default)]
struct Sets {
    own: Vec<String>,
    reached: Vec<String>,
}

impl Sets {
    fn of(
        tree: &Tree,
        record: &str,
        facet: FacetKind,
        entries: &[String],
    ) -> Result<Self, Refusal> {
        let field = format!("{} sources", facet.as_str());
        let refuse = |message: String| Refusal::new(Code::Source, record, &field, None, message);
        let mut own: BTreeSet<String> = BTreeSet::new();
        let mut reached: BTreeSet<String> = BTreeSet::new();
        for entry in entries {
            if entry == "catalog" || entry.starts_with("catalog/") {
                return Err(refuse(format!("`{entry}` is under `catalog/`")));
            }
            let is_file = tree.is_file(entry);
            let is_dir = !is_file && tree.is_dir(entry);
            if !is_file && !is_dir {
                return Err(refuse(format!(
                    "`{entry}` is not tracked: no file, and no directory with a tracked file"
                )));
            }
            let start = if is_file {
                tree::parent(entry)
            } else {
                entry.as_str()
            };
            let mut enclosing = None;
            for dir in tree::ancestors(start) {
                if is_package(tree, dir).map_err(|e| {
                    refuse(format!(
                        "`{}` line {} is outside the manifest dialect: {}",
                        tree::join(dir, "Cargo.toml"),
                        e.line,
                        e.why
                    ))
                })? {
                    enclosing = Some(dir);
                    break;
                }
            }
            match enclosing {
                Some(package) => {
                    let package = package.to_owned();
                    packages_below(tree, &package, &field, record)?;
                    own.extend(tree.under(&package).map(str::to_owned));
                    reach(
                        tree,
                        &package,
                        &mut reached,
                        &mut BTreeSet::new(),
                        record,
                        &field,
                    )?;
                }
                None if facet == FacetKind::Implementation => {
                    return Err(refuse(format!(
                        "`{entry}` is not a package, and an implementation names packages"
                    )));
                }
                None if is_file => {
                    own.insert(entry.clone());
                }
                None => {
                    if let Some(below) = tree
                        .under(entry)
                        .filter(|p| p.ends_with("/Cargo.toml"))
                        .find(|p| is_package(tree, tree::parent(p)).unwrap_or(false))
                    {
                        return Err(refuse(format!(
                            "`{entry}` has a package below it, `{}`: name the package",
                            tree::parent(below)
                        )));
                    }
                    own.extend(tree.under(entry).map(str::to_owned));
                }
            }
        }
        for path in own.iter().chain(&reached) {
            if path.starts_with("catalog/") {
                return Err(refuse(format!("`{path}` is under `catalog/`")));
            }
        }
        let reached = reached.difference(&own).cloned().collect();
        Ok(Self {
            own: own.into_iter().collect(),
            reached,
        })
    }
}

fn read_manifest(
    tree: &Tree,
    path: &str,
    record: &str,
    field: &str,
) -> Result<manifest::Manifest, Refusal> {
    let text = core::str::from_utf8(tree.get(path).unwrap_or_default()).map_err(|_| {
        Refusal::new(
            Code::Source,
            record,
            field,
            None,
            format!("`{path}` is not UTF-8"),
        )
    })?;
    manifest::parse(text).map_err(|e| {
        Refusal::new(
            Code::Source,
            record,
            field,
            None,
            format!(
                "`{path}` line {} is outside the manifest dialect: {}",
                e.line, e.why
            ),
        )
    })
}

/// Whether `dir` is a package: its tracked `Cargo.toml` has a `[package]` table (§3).
fn is_package(tree: &Tree, dir: &str) -> Result<bool, manifest::Outside> {
    let path = tree::join(dir, "Cargo.toml");
    match tree.get(&path) {
        None => Ok(false),
        Some(bytes) => Ok(
            manifest::parse(core::str::from_utf8(bytes).unwrap_or_default())?
                .has_table(&["package"]),
        ),
    }
}

/// A package with another package below it is refused (§3).
fn packages_below(tree: &Tree, package: &str, field: &str, record: &str) -> Result<(), Refusal> {
    for path in tree.under(package).filter(|p| p.ends_with("/Cargo.toml")) {
        let dir = tree::parent(path);
        if dir != package && is_package(tree, dir).unwrap_or(false) {
            return Err(Refusal::new(
                Code::Source,
                record,
                field,
                None,
                format!("the package `{package}` has another package below it, `{dir}`"),
            ));
        }
    }
    Ok(())
}

/// The dependency tables a package's manifest holds, by TOML's meaning (§3): each dependency's name, table kind,
/// and value.
fn dependencies(m: &manifest::Manifest) -> Vec<(String, bool)> {
    let tables = [
        "dependencies",
        "build-dependencies",
        "build_dependencies",
        "dev-dependencies",
        "dev_dependencies",
    ];
    let mut out: Vec<(String, bool)> = Vec::new();
    for (path, _) in &m.values {
        let rest: &[String] = match path.as_slice() {
            [table, rest @ ..] if tables.contains(&table.as_str()) => {
                let dev = table.starts_with("dev");
                if let [name, ..] = rest {
                    if !out.iter().any(|(n, d)| n == name && *d == dev) {
                        out.push((name.clone(), dev));
                    }
                }
                continue;
            }
            [target, _triple, rest @ ..] if target == "target" => rest,
            _ => continue,
        };
        if let [table, name, ..] = rest {
            if tables.contains(&table.as_str()) {
                let dev = table.starts_with("dev");
                if !out.iter().any(|(n, d)| n == name && *d == dev) {
                    out.push((name.clone(), dev));
                }
            }
        }
    }
    out
}

/// What a package reaches beyond its directory (§3): its dependency packages, transitively, its workspace manifest,
/// and the cargo configuration and toolchain files on its ancestor path.
fn reach(
    tree: &Tree,
    package: &str,
    out: &mut BTreeSet<String>,
    seen: &mut BTreeSet<String>,
    record: &str,
    field: &str,
) -> Result<(), Refusal> {
    if !seen.insert(package.to_owned()) {
        return Ok(());
    }
    let refuse = |message: String| Refusal::new(Code::Source, record, field, None, message);
    let manifest_path = tree::join(package, "Cargo.toml");
    let m = read_manifest(tree, &manifest_path, record, field)?;
    for (name, dev) in dependencies(&m) {
        let path = dependency_path(&m, &name).ok_or_else(|| {
            refuse(format!(
                "`{manifest_path}`: `{name}` is not a path dependency"
            ))
        })?;
        if dependency_is_workspace(&m, &name) {
            return Err(refuse(format!(
                "`{manifest_path}`: `{name}` is `workspace = true`"
            )));
        }
        if dev {
            continue;
        }
        let dir = tree::resolve(package, &path)
            .filter(|d| grammar::is_path(d))
            .ok_or_else(|| {
                refuse(format!(
                    "`{manifest_path}`: `{name}`'s path leaves the repository or its normal form"
                ))
            })?;
        if !is_package(tree, &dir).unwrap_or(false) {
            return Err(refuse(format!(
                "`{manifest_path}`: `{name}`'s path `{dir}` is not a tracked package"
            )));
        }
        packages_below(tree, &dir, field, record)?;
        out.extend(tree.under(&dir).map(str::to_owned));
        reach(tree, &dir, out, seen, record, field)?;
    }
    // The workspace manifest.
    if let Some(workspace_dir) = tree::ancestors(package).into_iter().find(|dir| {
        tree.get(&tree::join(dir, "Cargo.toml"))
            .and_then(|b| manifest::parse(core::str::from_utf8(b).ok()?).ok())
            .is_some_and(|m| m.has_table(&["workspace"]))
    }) {
        let ws_path = tree::join(workspace_dir, "Cargo.toml");
        let ws = read_manifest(tree, &ws_path, record, field)?;
        if workspace_dir != package {
            let relative = package
                .strip_prefix(&format!("{workspace_dir}/"))
                .unwrap_or(package);
            let listed = |key: &str| -> Result<bool, Refusal> {
                let Some(value) = ws.get(&["workspace", key]) else {
                    return Ok(false);
                };
                let Value::Array(items) = value else {
                    return Err(refuse(format!("`{ws_path}`: `{key}` is an array")));
                };
                for item in items {
                    let Value::Str(entry) = item else {
                        return Err(refuse(format!("`{ws_path}`: `{key}` holds strings")));
                    };
                    if !entry.split('/').all(|s| {
                        s == "*" || (!s.is_empty() && !s.contains('*') && s != "." && s != "..")
                    }) {
                        return Err(refuse(format!(
                            "`{ws_path}`: `{entry}` is outside the literal-or-`*` form"
                        )));
                    }
                    let (a, b): (Vec<&str>, Vec<&str>) =
                        (entry.split('/').collect(), relative.split('/').collect());
                    let covers = |n: usize| {
                        a.len() <= b.len()
                            && a.iter().zip(&b).take(n).all(|(x, y)| *x == "*" || x == y)
                    };
                    if (key == "members" && a.len() == b.len() && covers(a.len()))
                        || (key == "exclude" && covers(a.len()))
                    {
                        return Ok(true);
                    }
                }
                Ok(false)
            };
            if !listed("members")? || listed("exclude")? {
                return Err(refuse(format!("`{package}` is not a member of the workspace `{ws_path}`, which Cargo would walk past")));
            }
        }
        out.insert(ws_path);
    }
    // Configuration and toolchain files on the ancestor path.
    for dir in tree::ancestors(package) {
        for name in [".cargo/config.toml", ".cargo/config"] {
            let path = tree::join(dir, name);
            if tree.is_file(&path) {
                out.insert(path);
            }
        }
        let legacy = tree::join(dir, "rust-toolchain");
        if tree.is_file(&legacy) {
            return Err(refuse(format!(
                "`{legacy}`: a `rust-toolchain` file without the extension is refused"
            )));
        }
        let toolchain = tree::join(dir, "rust-toolchain.toml");
        if tree.is_file(&toolchain) {
            if !dir.is_empty() {
                return Err(refuse(format!(
                    "`{toolchain}`: a toolchain file anywhere but the repository root is refused"
                )));
            }
            out.insert(toolchain);
        }
    }
    Ok(())
}

/// Whether `path` is `<dependency table>.<name>.<key>`, at the root or under `target.<triple>`.
fn dependency_key(path: &[String], name: &str, key: &str) -> bool {
    const TABLES: [&str; 5] = [
        "dependencies",
        "build-dependencies",
        "build_dependencies",
        "dev-dependencies",
        "dev_dependencies",
    ];
    let rest = match path {
        [target, _triple, rest @ ..] if target == "target" => rest,
        rest => rest,
    };
    matches!(rest, [table, n, k] if TABLES.contains(&table.as_str()) && n == name && k == key)
}

/// The `path` of a dependency, in the dependency tables of the manifest.
fn dependency_path(m: &manifest::Manifest, name: &str) -> Option<String> {
    m.values.iter().find_map(|(path, value)| match value {
        Value::Str(s) if dependency_key(path, name, "path") => Some(s.clone()),
        _ => None,
    })
}

/// Whether a dependency is `workspace = true`.
fn dependency_is_workspace(m: &manifest::Manifest, name: &str) -> bool {
    m.values
        .iter()
        .any(|(path, value)| dependency_key(path, name, "workspace") && *value == Value::Bool(true))
}
