//! Committed generated sources in the trust inventory (leaf `M3.6.6.2`,
//! `docs/specs/trust/decision_trust-generated-sources.md`): the `defgenerated` form, and the steps the record adds to
//! `cargo xtask trust-inventory`.
//!
//! > A generated source declares its generator and its inputs in a reviewed form; one that does not is recognised by
//! > its header and refused.
//!
//! ⭐ **The order** (§2). The instrument decides in five steps — (1) the forms' text, nothing built; (2) the programs'
//! builds and handed data, with every refusal the parent record makes before or in them; (3) the blob rule and the
//! chain's first clause over live forms; (4) the generator judgments and builds; (5) the recogniser's refusal and the
//! chain's second clause — each running only when those before it refused nothing. A step that refuses ends the run,
//! no inventory written, every refusal it found reported; a step in which a build fails runs to its end and ends the
//! run unable to judge, its refusals reported beside the failure. Steps 1 and 3 are here, and the record of what a
//! program reaches through a live form (§4, §5); [`crate::trust::inventory_with`] runs them in order.
//!
//! ⭐ **Provenance is matched by path and by content** (§4, §5). A program's provenance is the generator files and
//! inputs of every live form whose declared file it reads; two paired programs share a generated-provenance item for
//! each file one's provenance holds and the other's holds or reads, and for each content both sides hold under paths
//! that differ, one of them provenance — so a shared input through two generators, a generator shared over two
//! inputs, and a data file one side reads that the other's generated source was made from are all seen.
//!
//! ⛔ **Coded, never uncoded** (§2). Every departure of a `defgenerated` form from its shape, and every construction the
//! forms' text alone shows wrong, is a `trust-undeclared-input` refusal of the commit. Only what the eADL reader cannot
//! read, or another form of `trust/roots.eadl` departing from the parent's shape, leaves the gate unable to judge — and
//! then the `defgenerated` refusals the parent's reader met before it are reported beside it.

use std::collections::{BTreeMap, BTreeSet};

use archogen_catalog::tree::Tree;
use eadl_front::Form;

use crate::json::Json;
use crate::trust::ROOTS;

/// The form that declares a committed generated source (§2).
pub const GENERATED: &str = "defgenerated";

/// The clauses a `defgenerated` form takes (§2).
const CLAUSES: &[&str] = &["generator", "inputs", "command", "reason"];

/// A committed generated source, declared: a `defgenerated` form whose shape holds (§2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generated {
    /// The declared file: the form's name, a repository path as the commit's tree spells it.
    pub path: String,
    /// The generator files that wrote it, one or more.
    pub generators: Vec<String>,
    /// The tracked files the generator read, none or more.
    pub inputs: Vec<String>,
}

/// One `defgenerated` form as its text reads: what it names, and every departure from the shape.
#[derive(Debug, Clone)]
pub struct Read {
    /// How a refusal names the form: its declared path, if it has one, and its line.
    label: String,
    /// The declared file, when the name is a string.
    path: Option<String>,
    /// The string entries of `(generator …)`.
    generators: Vec<String>,
    /// The string entries of `(inputs …)`.
    inputs: Vec<String>,
    /// Every departure from the shape, each a whole refusal.
    shape: Vec<String>,
}

impl Read {
    fn refusal(&self, why: &str) -> String {
        format!("trust-undeclared-input: {ROOTS}: {} {why}", self.label)
    }
}

/// Read one `defgenerated` form strictly (§2), `line` being where it stands in the roots file. Nothing here fails: a
/// departure is a refusal the form carries.
#[must_use]
pub fn read_form(form: &Form, line: u32) -> Read {
    let items = form.items();
    // The name comes first; a clause in its place is a name missing, read as one, so the clause is still read.
    let (path, first_clause) = match items.get(1) {
        Some(Form::Str { value, .. }) => (Some(value.clone()), 2),
        Some(clause) if clause.head().is_some() => (None, 1),
        Some(_) => (None, 2),
        None => (None, 1),
    };
    let label = match &path {
        Some(p) => format!("`({GENERATED} \"{p}\" …)` (line {line})"),
        None => format!("the `{GENERATED}` form of line {line}"),
    };
    let mut read = Read {
        label,
        path,
        generators: Vec::new(),
        inputs: Vec::new(),
        shape: Vec::new(),
    };
    let mut why: Vec<String> = Vec::new();
    if read.path.is_none() {
        why.push(
            "has a name that is not a string — it names the declared file by its repository path"
                .to_owned(),
        );
    }
    let mut seen: BTreeSet<&str> = BTreeSet::new();
    let mut generator_entries = 0usize;
    for c in items.iter().skip(first_clause) {
        let Some(head) = c.head() else {
            why.push("holds something that is not a clause".to_owned());
            continue;
        };
        if !CLAUSES.contains(&head) {
            why.push(format!("holds a clause `{head}` its form does not take"));
            continue;
        }
        if !seen.insert(head) {
            why.push(format!("holds `({head} …)` twice"));
            continue;
        }
        let entries = &c.items()[1..];
        match head {
            "generator" | "inputs" => {
                if head == "generator" {
                    generator_entries = entries.len();
                }
                for e in entries {
                    match e {
                        Form::Str { value, .. } if head == "generator" => {
                            read.generators.push(value.clone());
                        }
                        Form::Str { value, .. } => read.inputs.push(value.clone()),
                        _ => why.push(format!("`({head} …)` holds an entry that is not a string")),
                    }
                }
            }
            _ => {
                let one_string = matches!(entries, [Form::Str { value, .. }] if !value.is_empty());
                if !one_string {
                    why.push(format!(
                        "`({head} …)` holds other than exactly one non-empty string"
                    ));
                }
            }
        }
    }
    if generator_entries == 0 {
        why.push(
            "names no generator — `(generator …)` holds one generator file or more".to_owned(),
        );
    }
    for clause in ["command", "reason"] {
        if !seen.contains(clause) {
            why.push(format!(
                "has no `({clause} …)` — it holds exactly one non-empty string"
            ));
        }
    }
    read.shape = why.iter().map(|w| read.refusal(w)).collect();
    read
}

/// Every refusal of the forms' text (§2, step 1): each form's departures from the shape, and the constructions read
/// from the forms alone, no build consulted — a path two forms declare, each of them refused; an entry twice in one
/// clause; a file in both `generator` and `inputs`; a declared file that is its own form's generator or input.
#[must_use]
pub fn refusals(forms: &[Read]) -> Vec<String> {
    let mut declared: BTreeMap<&str, usize> = BTreeMap::new();
    for f in forms {
        if let Some(p) = &f.path {
            *declared.entry(p.as_str()).or_default() += 1;
        }
    }
    let mut out = Vec::new();
    for f in forms {
        out.extend(f.shape.iter().cloned());
        if let Some(p) = &f.path {
            let n = declared[p.as_str()];
            if n > 1 {
                out.push(f.refusal(&format!(
                    "declares `{p}`, which {n} forms declare — one form per declared path"
                )));
            }
        }
        for (clause, entries) in [("generator", &f.generators), ("inputs", &f.inputs)] {
            let mut seen = BTreeSet::new();
            let twice: BTreeSet<&String> = entries.iter().filter(|e| !seen.insert(*e)).collect();
            for e in twice {
                out.push(f.refusal(&format!("names `{e}` twice in `({clause} …)`")));
            }
        }
        let both: BTreeSet<&String> = f
            .generators
            .iter()
            .filter(|g| f.inputs.contains(g))
            .collect();
        for e in both {
            out.push(f.refusal(&format!("names `{e}` as both a generator and an input")));
        }
        if let Some(p) = &f.path {
            if f.generators.contains(p) || f.inputs.contains(p) {
                out.push(f.refusal(&format!(
                    "names its own declared file `{p}` as its generator or an input"
                )));
            }
        }
    }
    out
}

/// The declarations whose shape holds, in the order the file holds them.
#[must_use]
pub fn declarations(forms: Vec<Read>) -> Vec<Generated> {
    forms
        .into_iter()
        .filter(|f| f.shape.is_empty())
        .filter_map(|f| {
            Some(Generated {
                path: f.path?,
                generators: f.generators,
                inputs: f.inputs,
            })
        })
        .collect()
}

/// A failure that leaves the gate unable to judge, with the refusals found beside it (§2): the `defgenerated` forms'
/// when another form departs, or a step's when a build in it fails.
#[must_use]
pub fn beside(why: String, refused: &[String]) -> String {
    if refused.is_empty() {
        return why;
    }
    let mut sorted = refused.to_vec();
    sorted.sort();
    sorted.dedup();
    let mut out = format!("{why}\nand beside it, {} refusal(s):", sorted.len());
    for r in sorted {
        out.push_str("\n  ");
        out.push_str(&r);
    }
    out
}

// ── Step 3, and what a program reaches (§2, §4, §5) ────────────────────────────────────────────────────────────

/// The live forms (§2): those whose declared file a program reads, by its build or as data its form hands it — the
/// harness through the units it compiles beside its pair's two builds (§1). `reads` maps each program to what it reads.
#[must_use]
pub fn live<'a>(
    generated: &'a [Generated],
    reads: &BTreeMap<String, BTreeSet<String>>,
) -> Vec<&'a Generated> {
    generated
        .iter()
        .filter(|g| reads.values().any(|r| r.contains(&g.path)))
        .collect()
}

/// Step 3 (§2), over live forms: the blob rule — a generator or input that is a symbolic link, or is no blob of the
/// commit (a gitlink, a directory, a path the commit does not hold), refused by that rule alone — then the chain's
/// first clause — a generator or input a `defgenerated` form declares, or whose header marks it generated (§3), a
/// chain, refused for good (decision_trust-generated-refusals.md §2). A form no program reads is held to its text alone
/// (step 1).
#[must_use]
pub fn blobs_and_chains(
    generated: &[Generated],
    live: &[&Generated],
    tree: &Tree,
    symlinks: &BTreeSet<String>,
) -> Vec<String> {
    let declared: BTreeSet<&str> = generated.iter().map(|g| g.path.as_str()).collect();
    let mut out = Vec::new();
    for g in live {
        for (role, files) in [("generator", &g.generators), ("input", &g.inputs)] {
            for f in files {
                let name = format!(
                    "trust-undeclared-input: {ROOTS}: `({GENERATED} \"{}\" …)`'s {role} `{f}`",
                    g.path
                );
                if symlinks.contains(f) {
                    out.push(format!(
                        "{name} is a symbolic link in the commit — a generator or an input is a blob"
                    ));
                    continue;
                }
                let Some(bytes) = tree.get(f) else {
                    out.push(format!(
                        "{name} is no blob of the commit — a gitlink, a directory, or a path it does not hold"
                    ));
                    continue;
                };
                if declared.contains(f.as_str()) {
                    out.push(format!("{name} is declared by a `{GENERATED}` form — a chain of generators, refused (decision_trust-generated-refusals.md §2)"));
                } else if crate::generated_header::marked(f, bytes) {
                    out.push(format!("{name} has a header that marks it generated — a chain of generators, refused (decision_trust-generated-refusals.md §2)"));
                }
            }
        }
    }
    out
}

/// The innermost workspace member whose directory holds `file`, by whole path components (§5): the longest member
/// directory that is a prefix of its path at a `/` — the workspace root's own package, `""`, holding every file — or
/// none.
pub fn innermost_member<'a>(
    file: &str,
    members: impl IntoIterator<Item = &'a String>,
) -> Option<&'a String> {
    members
        .into_iter()
        .filter(|m| m.is_empty() || file.starts_with(&format!("{m}/")))
        .max_by_key(|m| m.len())
}

/// A provenance file of a program (§4).
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct Provenance {
    /// Its sha256.
    pub sha256: String,
    /// The roles it plays across the forms the program reaches: `generator`, `input`.
    pub roles: BTreeSet<&'static str>,
    /// The declared files through which the program reaches it, each with its sha256.
    pub declared: BTreeMap<String, String>,
}

/// A program's provenance, by path.
pub type Provenances = BTreeMap<String, Provenance>;

/// A file's sha256 in the commit, empty for one it does not hold.
fn sha_of(tree: &Tree, f: &str) -> String {
    tree.get(f).map_or_else(String::new, |b| {
        archogen_evidence::sha256::Digest::of(b).hex()
    })
}

/// A program's provenance (§4): for each live form whose declared file it reads, the form's generator files and inputs.
#[must_use]
pub fn provenance(live: &[&Generated], reads: &BTreeSet<String>, tree: &Tree) -> Provenances {
    let mut out = Provenances::new();
    for g in live.iter().filter(|g| reads.contains(&g.path)) {
        for (role, files) in [("generator", &g.generators), ("input", &g.inputs)] {
            for f in files {
                let p = out.entry(f.clone()).or_default();
                p.sha256 = sha_of(tree, f);
                p.roles.insert(role);
                p.declared.insert(g.path.clone(), sha_of(tree, &g.path));
            }
        }
    }
    out
}

/// A program's provenance as its record holds it: each file's sha256, roles and declared files.
#[must_use]
pub fn provenance_json(p: &Provenances) -> Json {
    Json::Object(
        p.iter()
            .map(|(path, f)| {
                let fields = [
                    ("sha256".to_owned(), Json::Str(f.sha256.clone())),
                    (
                        "roles".to_owned(),
                        Json::Array(f.roles.iter().map(|r| Json::Str((*r).to_owned())).collect()),
                    ),
                    (
                        "declared".to_owned(),
                        Json::Object(
                            f.declared
                                .iter()
                                .map(|(d, s)| (d.clone(), Json::Str(s.clone())))
                                .collect(),
                        ),
                    ),
                ];
                (path.clone(), Json::Object(fields.into_iter().collect()))
            })
            .collect(),
    )
}

/// One side of a pair, as its generated-provenance items see it: what its provenance holds, and what it reads.
pub struct Side<'a> {
    /// Its provenance.
    pub provenance: &'a Provenances,
    /// What it reads (§1).
    pub reads: &'a BTreeSet<String>,
}

impl Side<'_> {
    /// Whether the side holds `f` in its provenance or reads it.
    fn has(&self, f: &str) -> bool {
        self.provenance.contains_key(f) || self.reads.contains(f)
    }

    /// The roles the side plays over `paths` — `read` for a path it reads, beside those its provenance gives each.
    fn roles(&self, paths: &BTreeSet<&str>) -> Json {
        let mut roles: BTreeSet<&str> = BTreeSet::new();
        for p in paths {
            if self.reads.contains(*p) {
                roles.insert("read");
            }
            if let Some(f) = self.provenance.get(*p) {
                roles.extend(f.roles.iter().copied());
            }
        }
        Json::Array(roles.into_iter().map(|r| Json::Str(r.to_owned())).collect())
    }

    /// The declared files through which the side reaches `paths`, each with its sha256.
    fn declared(&self, paths: &BTreeSet<&str>) -> Json {
        let decl: BTreeSet<String> = paths
            .iter()
            .filter_map(|p| self.provenance.get(*p))
            .flat_map(|f| f.declared.iter().map(|(d, s)| format!("{d}={s}")))
            .collect();
        Json::Array(decl.into_iter().map(Json::Str).collect())
    }
}

/// The generated-provenance items a pair shares (§5): one per file one side's provenance holds and the other's holds or
/// reads, its identity the path; and, as the parent matches a copy, one per non-empty content each side holds or reads
/// under paths that differ, one of them a provenance file of its side, its identity those paths sorted. Each item's
/// aspects: the content's sha256, each side's roles over its paths, and each side's declared files with theirs.
#[must_use]
pub fn shared_items(a: &Side, b: &Side, tree: &Tree) -> Vec<Json> {
    let item = |identity: Json, sha256: String, pa: &BTreeSet<&str>, pb: &BTreeSet<&str>| {
        Json::Object(
            [
                (
                    "kind".to_owned(),
                    Json::Str("generated-provenance".to_owned()),
                ),
                ("item".to_owned(), identity),
                ("sha256".to_owned(), Json::Str(sha256)),
                (
                    "roles".to_owned(),
                    Json::Array(vec![a.roles(pa), b.roles(pb)]),
                ),
                (
                    "declared".to_owned(),
                    Json::Array(vec![a.declared(pa), b.declared(pb)]),
                ),
            ]
            .into_iter()
            .collect(),
        )
    };
    let mut out = Vec::new();
    // By path: a file one side's provenance holds that the other's holds or reads.
    let candidates: BTreeSet<&str> = a
        .provenance
        .keys()
        .chain(b.provenance.keys())
        .map(String::as_str)
        .collect();
    for f in candidates {
        let shared = (a.provenance.contains_key(f) && b.has(f))
            || (b.provenance.contains_key(f) && a.has(f));
        if shared {
            let one: BTreeSet<&str> = [f].into();
            out.push(item(Json::Str(f.to_owned()), sha_of(tree, f), &one, &one));
        }
    }
    // By content: each side's paths of one non-empty content, the two sets differing, one path a provenance file.
    let mut by_hash: BTreeMap<String, (BTreeSet<&str>, BTreeSet<&str>)> = BTreeMap::new();
    for (side, slot) in [(a, 0), (b, 1)] {
        let paths = side
            .provenance
            .keys()
            .map(String::as_str)
            .chain(side.reads.iter().map(String::as_str));
        for p in paths {
            if tree.get(p).is_some_and(|x| !x.is_empty()) {
                let e = by_hash.entry(sha_of(tree, p)).or_default();
                if slot == 0 { &mut e.0 } else { &mut e.1 }.insert(p);
            }
        }
    }
    for (hash, (pa, pb)) in by_hash {
        let provenance_among = pa.iter().any(|p| a.provenance.contains_key(*p))
            || pb.iter().any(|p| b.provenance.contains_key(*p));
        if !pa.is_empty() && !pb.is_empty() && pa != pb && provenance_among {
            let union: BTreeSet<&str> = pa.union(&pb).copied().collect();
            let identity = Json::Array(union.iter().map(|p| Json::Str((*p).to_owned())).collect());
            out.push(item(identity, hash, &pa, &pb));
        }
    }
    out
}

#[cfg(test)]
mod tests {
    //! Step 1 of the record's order (§2), and the frame every step runs in: each departure from a `defgenerated` form's
    //! shape and each text-only construction a coded refusal; the parent's reader stopping with them beside it; a run
    //! ending at a step that refuses, or unable to judge with that step's refusals beside a failed build.

    use super::Generated;
    use crate::trust::tests::{real_root, refused, two_roots, written, ROOTS as TWO_ROOTS};
    use crate::trust::{inventory_with, read_roots};

    const GOOD: &str = "(defgenerated \"crates/a/src/table.rs\" (generator \"scripts/gen.sh\") (inputs \"data/t.csv\") \
                        (command \"bash scripts/gen.sh data/t.csv\") (reason \"the table\"))";

    fn refusals_of(text: &str) -> Vec<String> {
        read_roots(text).expect(text).generated_refused
    }

    fn refuses(text: &str, why: &str) {
        let r = refusals_of(text);
        assert!(
            r.iter().any(
                |x| x.starts_with("trust-undeclared-input: trust/roots.eadl: ") && x.contains(why)
            ),
            "{text}\nno refusal says `{why}`: {r:#?}"
        );
    }

    #[test]
    fn a_well_formed_declaration_is_read_and_refuses_nothing() {
        let roots = read_roots(GOOD).expect("reads");
        assert!(
            roots.generated_refused.is_empty(),
            "{:#?}",
            roots.generated_refused
        );
        assert_eq!(
            roots.generated,
            [Generated {
                path: "crates/a/src/table.rs".to_owned(),
                generators: vec!["scripts/gen.sh".to_owned()],
                inputs: vec!["data/t.csv".to_owned()],
            }]
        );
        // `(inputs)` may be omitted when there are none, or written empty.
        for text in [
            "(defgenerated \"t.rs\" (generator \"g.sh\") (command \"g\") (reason \"r\"))",
            "(defgenerated \"t.rs\" (generator \"g.sh\") (inputs) (command \"g\") (reason \"r\"))",
        ] {
            let roots = read_roots(text).expect(text);
            assert!(
                roots.generated_refused.is_empty(),
                "{text}: {:#?}",
                roots.generated_refused
            );
            assert!(roots.generated[0].inputs.is_empty());
        }
    }

    #[test]
    fn every_departure_from_the_shape_is_a_coded_refusal() {
        let rest = "(generator \"g.sh\") (command \"c\") (reason \"r\")";
        for (text, why) in [
            (
                format!("(defgenerated t.rs {rest})"),
                "has a name that is not a string",
            ),
            (
                format!("(defgenerated {rest})"),
                "has a name that is not a string",
            ),
            (
                format!("(defgenerated \"t.rs\" stray {rest})"),
                "holds something that is not a clause",
            ),
            (
                format!("(defgenerated \"t.rs\" (output \"o\") {rest})"),
                "holds a clause `output` its form does not take",
            ),
            (
                format!("(defgenerated \"t.rs\" (inputs \"i\") (inputs \"j\") {rest})"),
                "holds `(inputs …)` twice",
            ),
            (
                "(defgenerated \"t.rs\" (command \"c\") (reason \"r\"))".to_owned(),
                "names no generator",
            ),
            (
                "(defgenerated \"t.rs\" (generator) (command \"c\") (reason \"r\"))".to_owned(),
                "names no generator",
            ),
            (
                "(defgenerated \"t.rs\" (generator \"g\") (reason \"r\"))".to_owned(),
                "has no `(command …)`",
            ),
            (
                "(defgenerated \"t.rs\" (generator \"g\") (command \"c\"))".to_owned(),
                "has no `(reason …)`",
            ),
            (
                "(defgenerated \"t.rs\" (generator \"g\") (command \"a\" \"b\") (reason \"r\"))"
                    .to_owned(),
                "`(command …)` holds other than exactly one non-empty string",
            ),
            (
                "(defgenerated \"t.rs\" (generator \"g\") (command c) (reason \"r\"))".to_owned(),
                "`(command …)` holds other than exactly one non-empty string",
            ),
            (
                "(defgenerated \"t.rs\" (generator \"g\") (command \"c\") (reason \"\"))"
                    .to_owned(),
                "`(reason …)` holds other than exactly one non-empty string",
            ),
            (
                "(defgenerated \"t.rs\" (generator \"g\") (command \"c\") (reason))".to_owned(),
                "`(reason …)` holds other than exactly one non-empty string",
            ),
            (
                "(defgenerated \"t.rs\" (generator g.sh) (command \"c\") (reason \"r\"))"
                    .to_owned(),
                "`(generator …)` holds an entry that is not a string",
            ),
            (
                format!("(defgenerated \"t.rs\" (inputs 3) {rest})"),
                "`(inputs …)` holds an entry that is not a string",
            ),
        ] {
            refuses(&text, why);
            // A departure is a refusal of the commit, never a form read as a declaration.
            assert!(
                read_roots(&text).expect(&text).generated.is_empty(),
                "{text}"
            );
        }
    }

    #[test]
    fn every_text_only_construction_is_a_coded_refusal() {
        // A path two forms declare: each of them refused.
        let r = refusals_of(&format!("{GOOD}\n{GOOD}"));
        let twice: Vec<&String> = r
            .iter()
            .filter(|x| x.contains("which 2 forms declare"))
            .collect();
        assert_eq!(twice.len(), 2, "{r:#?}");
        assert!(
            twice[0].contains("(line 1)") && twice[1].contains("(line 2)"),
            "{twice:#?}"
        );
        let c = "(command \"c\") (reason \"r\")";
        refuses(
            &format!("(defgenerated \"t.rs\" (generator \"g\" \"g\") {c})"),
            "names `g` twice in `(generator …)`",
        );
        refuses(
            &format!("(defgenerated \"t.rs\" (generator \"g\") (inputs \"i\" \"i\") {c})"),
            "names `i` twice in `(inputs …)`",
        );
        refuses(
            &format!("(defgenerated \"t.rs\" (generator \"g\") (inputs \"g\") {c})"),
            "names `g` as both a generator and an input",
        );
        // A declared file that is its own form's generator or input: by this rule alone (§2).
        for clauses in [
            "(generator \"t.rs\")",
            "(generator \"g\") (inputs \"t.rs\")",
        ] {
            refuses(
                &format!("(defgenerated \"t.rs\" {clauses} {c})"),
                "names its own declared file `t.rs` as its generator or an input",
            );
        }
        // Two spellings of one file are two entries: compared literally (§2).
        assert!(refusals_of(&format!(
            "(defgenerated \"t.rs\" (generator \"g\" \"./g\") {c})"
        ))
        .is_empty());
    }

    #[test]
    fn the_parent_s_reader_stops_at_another_form_with_the_refusals_before_it_beside_it() {
        let text = "(defgenerated \"before.rs\" (generator \"g\") (command \"c\"))\n\
                    (defroot x (role generator) (package \"p\") (target lib) (colour \"red\"))\n\
                    (defgenerated \"after.rs\" (generator \"g\") (command \"c\"))\n";
        let err = read_roots(text).expect_err("the parent's form departs");
        assert!(
            err.contains("`x` holds a clause `colour` its form does not take"),
            "{err}"
        );
        assert!(
            err.contains("and beside it, 1 refusal(s):\n  trust-undeclared-input: trust/roots.eadl: `(defgenerated \"before.rs\" …)` (line 1) has no `(reason …)`"),
            "{err}"
        );
        assert!(
            !err.contains("after.rs"),
            "a form after the departure was read: {err}"
        );
    }

    #[test]
    fn a_departure_found_once_the_file_is_read_has_every_refusal_beside_it() {
        let text = "(defgenerated \"before.rs\" (generator \"g\") (command \"c\"))\n\
                    (defharness h (pair a b) (package \"p\") (test t))\n\
                    (defgenerated \"after.rs\" (generator \"g\") (command \"c\"))\n";
        let err = read_roots(text).expect_err("the pair names no root");
        assert!(
            err.contains("`h`'s pair names `a`, which names no root"),
            "{err}"
        );
        assert!(err.contains("and beside it, 2 refusal(s):"), "{err}");
        assert!(
            err.contains("before.rs") && err.contains("after.rs"),
            "{err}"
        );
    }

    #[test]
    fn a_refusal_of_the_forms_text_ends_the_run_before_anything_is_written_or_built() {
        let f = two_roots("gs-step-1", "fn main() {}\n", "pub fn f() {}\n", &[]);
        written(f.run());
        let out = f.base.join("out");
        std::fs::remove_dir_all(out.join("tree")).expect("the first run wrote its tree");
        f.commit(&[(
            "trust/roots.eadl",
            format!("{TWO_ROOTS}(defgenerated \"crates/a/src/t.rs\" (generator \"g.sh\") (command \"c\"))\n"),
        )]);
        assert_eq!(
            refused(f.run()),
            ["trust-undeclared-input: trust/roots.eadl: `(defgenerated \"crates/a/src/t.rs\" …)` (line 3) has no \
              `(reason …)` — it holds exactly one non-empty string"]
        );
        assert!(
            !out.join("tree").exists(),
            "the tree was written before step 1 ended the run"
        );
        assert!(
            !out.join("trust-dependencies.json").exists(),
            "an earlier run's inventory was left"
        );
    }

    #[test]
    fn a_refused_run_leaves_no_inventory_an_earlier_run_wrote() {
        // The parent's own refusal before any build: a root naming a package the commit no longer has (its §6).
        let f = two_roots("gs-left-behind", "fn main() {}\n", "pub fn f() {}\n", &[]);
        written(f.run());
        f.commit(&[(
            "trust/roots.eadl",
            format!("{TWO_ROOTS}(defroot old (role reference-model) (package \"crates/gone\") (target lib))\n"),
        )]);
        let r = refused(f.run());
        assert!(r[0].starts_with("trust-baseline-stale:"), "{r:#?}");
        assert!(
            !f.base.join("out/trust-dependencies.json").exists(),
            "a refused run left the inventory of the commit before, to be read as this one's"
        );
    }

    #[test]
    fn a_build_failing_in_a_step_leaves_it_unable_to_judge_with_its_refusals_beside_it() {
        // Step 2 runs to its end: the other root is built and its sites judged, and the failure is reported with the
        // refusal that step found (§2).
        let f = two_roots(
            "gs-step-2-fails",
            "fn main() { let _: u32 = \"not a number\"; }\n",
            "#[no_mangle]\npub extern \"C\" fn f() {}\n",
            &[],
        );
        let Err(err) = inventory_with(&f.repo, "HEAD", &f.base.join("out"), &real_root()) else {
            panic!("a build failed and the run judged");
        };
        assert!(
            err.contains("`cargo build --bin a") && err.contains("failed"),
            "{err}"
        );
        assert!(err.contains("and beside it, 1 refusal(s):"), "{err}");
        assert!(
            err.contains("`crates/b/src/lib.rs:1`: `no_mangle`"),
            "{err}"
        );
        assert!(!f.base.join("out/trust-dependencies.json").exists());
    }
}

#[cfg(test)]
mod provenance_tests {
    //! Steps 2 and 3 of the record's order and what a program reaches (§2, §4, §5; `GS-H3`, `GS-H5`): each program's
    //! provenance, each pair's generated-provenance items by path and by content, the harness through the units it
    //! compiles beside its pair, and the blob rule and the chain's first clause over live forms.

    use crate::json::Json;
    use crate::trust::tests::{
        git, harness, items, kind, manifest, refused, says, two_roots, written, Fixture,
    };

    const GEN: &str = "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-packages \"crates/a\")";
    const CHK: &str =
        "(defroot chk (role scheduling-checker) (package \"crates/b\") (target lib) (role-packages \"crates/b\")";

    /// A `defgenerated` form.
    fn form(path: &str, generators: &[&str], inputs: &[&str]) -> String {
        let list = |xs: &[&str]| {
            xs.iter()
                .map(|x| format!("\"{x}\""))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let inputs = if inputs.is_empty() {
            String::new()
        } else {
            format!(" (inputs {})", list(inputs))
        };
        format!(
            "(defgenerated \"{path}\" (generator {}){inputs} (command \"bash gen\") (reason \"a table\"))\n",
            list(generators)
        )
    }

    /// The roots file: `gen` and `chk`, each with its handed data, and `forms`.
    fn roots(gen_data: &str, chk_data: &str, forms: &str) -> String {
        format!("{GEN}{gen_data})\n{CHK}{chk_data})\n{forms}")
    }

    /// Two roots, `gen` reading `crates/a/src/gen_a.rs` and `chk` reading `crates/b/src/gen_b.rs`, two generated
    /// modules differing in bytes; `files` beside them, the roots file among them.
    fn generated_pair(name: &str, files: &[(&str, String)]) -> Fixture {
        let mut all: Vec<(&str, String)> = vec![
            (
                "crates/a/src/gen_a.rs",
                "pub const T: u32 = 1;\n".to_owned(),
            ),
            (
                "crates/b/src/gen_b.rs",
                "pub const T: u32 = 2;\n".to_owned(),
            ),
            ("scripts/gen.sh", "echo a table\n".to_owned()),
            ("scripts/gen_a.sh", "echo a's table\n".to_owned()),
            ("scripts/gen_b.sh", "echo b's table\n".to_owned()),
        ];
        all.extend(files.iter().cloned());
        two_roots(
            name,
            "mod gen_a;\nfn main() { let _ = gen_a::T; }\n",
            "mod gen_b;\npub fn f() -> u32 { gen_b::T }\n",
            &all,
        )
    }

    fn text(j: &Json) -> String {
        match j {
            Json::Str(s) => s.clone(),
            Json::Array(xs) => xs.iter().map(text).collect::<Vec<_>>().join(" "),
            _ => String::new(),
        }
    }

    /// The generated-provenance items of one pair, each as `identity roles-of-one/roles-of-other`.
    fn provenance_items(inv: &Json, pair: &str) -> Vec<String> {
        items(inv)
            .into_iter()
            .filter(|(p, i)| p == pair && kind(i) == "generated-provenance")
            .map(|(_, i)| {
                let roles = i.get("roles").map(Json::elements).unwrap_or_default();
                let side = |n: usize| {
                    roles
                        .get(n)
                        .map(|r| text(r).replace(' ', ","))
                        .unwrap_or_default()
                };
                format!(
                    "{} {}/{}",
                    text(i.get("item").unwrap_or(&Json::Null)),
                    side(0),
                    side(1)
                )
            })
            .collect()
    }

    /// One program's provenance, as `path roles`.
    fn provenance_of(inv: &Json, program: &str) -> Vec<String> {
        let p = inv
            .get("programs")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .find(|p| p.get("name").and_then(Json::as_str) == Some(program))
            .expect("the program's record");
        match p.get("provenance") {
            Some(Json::Object(m)) => m
                .iter()
                .map(|(k, v)| {
                    format!(
                        "{k} {}",
                        text(v.get("roles").unwrap_or(&Json::Null)).replace(' ', ",")
                    )
                })
                .collect(),
            other => panic!("no provenance in {program}'s record: {other:?}"),
        }
    }

    #[test]
    fn case_3_two_generated_sources_sharing_an_input_through_two_generators_share_it() {
        // §14.4's case 3 (`TI-H22`, `GS-H5`): differently named generated sources, differing in bytes, made by two
        // generators from one input. The parent saw nothing shared but the build configuration; the input is shared.
        let forms = form(
            "crates/a/src/gen_a.rs",
            &["scripts/gen_a.sh"],
            &["data/shared.csv"],
        ) + &form(
            "crates/b/src/gen_b.rs",
            &["scripts/gen_b.sh"],
            &["data/shared.csv"],
        );
        let f = generated_pair(
            "gs-case-3",
            &[
                ("trust/roots.eadl", roots("", "", &forms)),
                ("data/shared.csv", "1,2\n".to_owned()),
            ],
        );
        let inv = written(f.run());
        assert_eq!(
            provenance_items(&inv, "gen+chk"),
            ["data/shared.csv input/input"]
        );
        assert_eq!(
            provenance_of(&inv, "gen"),
            ["data/shared.csv input", "scripts/gen_a.sh generator"]
        );
        let item = items(&inv)
            .into_iter()
            .find(|(_, i)| kind(i) == "generated-provenance")
            .expect("the item")
            .1;
        let declared = text(item.get("declared").unwrap_or(&Json::Null));
        assert!(
            declared.contains("crates/a/src/gen_a.rs=")
                && declared.contains("crates/b/src/gen_b.rs="),
            "{declared}"
        );
        let kinds: Vec<String> = items(&inv)
            .iter()
            .map(|(_, i)| kind(i).to_owned())
            .collect();
        assert_eq!(
            kinds,
            ["build-configuration", "generated-provenance"],
            "{kinds:?}"
        );
    }

    #[test]
    fn a_generator_shared_over_two_inputs_or_with_none_is_shared() {
        let forms = form(
            "crates/a/src/gen_a.rs",
            &["scripts/gen.sh"],
            &["data/x.csv"],
        ) + &form(
            "crates/b/src/gen_b.rs",
            &["scripts/gen.sh"],
            &["data/y.csv"],
        );
        let f = generated_pair(
            "gs-one-generator",
            &[
                ("trust/roots.eadl", roots("", "", &forms)),
                ("data/x.csv", "x\n".to_owned()),
                ("data/y.csv", "y\n".to_owned()),
            ],
        );
        assert_eq!(
            provenance_items(&written(f.run()), "gen+chk"),
            ["scripts/gen.sh generator/generator"]
        );
        // A table computed from a formula in the generator has no input, and still shares its generator.
        let forms = form("crates/a/src/gen_a.rs", &["scripts/gen.sh"], &[])
            + &form("crates/b/src/gen_b.rs", &["scripts/gen.sh"], &[]);
        f.commit(&[("trust/roots.eadl", roots("", "", &forms))]);
        assert_eq!(
            provenance_items(&written(f.run()), "gen+chk"),
            ["scripts/gen.sh generator/generator"]
        );
    }

    #[test]
    fn a_data_file_one_side_reads_and_a_copy_of_an_input_are_shared_and_a_plain_copy_is_not() {
        // `chk` is handed `data/t.csv`, from which `gen`'s generated source was made; and `data/copy.csv`, a byte copy of
        // `gen`'s other input. Both handed `data/p1.txt` and `data/p2.txt`, copies of each other and no provenance.
        let forms = form(
            "crates/a/src/gen_a.rs",
            &["scripts/gen_a.sh"],
            &["data/t.csv", "data/x.csv"],
        );
        let f = generated_pair(
            "gs-read-and-copy",
            &[
                (
                    "trust/roots.eadl",
                    roots(
                        " (data \"data/p1.txt\")",
                        " (data \"data/t.csv\" \"data/copy.csv\" \"data/p2.txt\")",
                        &forms,
                    ),
                ),
                ("data/t.csv", "t\n".to_owned()),
                ("data/x.csv", "same\n".to_owned()),
                ("data/copy.csv", "same\n".to_owned()),
                ("data/p1.txt", "plain\n".to_owned()),
                ("data/p2.txt", "plain\n".to_owned()),
            ],
        );
        let inv = written(f.run());
        assert_eq!(
            provenance_items(&inv, "gen+chk"),
            [
                "data/t.csv input/read",
                "data/copy.csv data/x.csv input/read"
            ]
        );
        // The plain copy is the parent's copy item, and no generated provenance.
        assert!(items(&inv).iter().any(|(_, i)| kind(i) == "copy"
            && text(i.get("paths").unwrap_or(&Json::Null)) == "data/p1.txt data/p2.txt"));
    }

    #[test]
    fn the_harness_reaches_provenance_through_the_units_beside_its_pair_alone() {
        // `diff`'s test target compiles `tests/table.rs`, generated by `scripts/gen.sh`; the reference model compiles
        // `src/gen_r.rs`, generated by `scripts/other.sh`, in its own build: the harness's item holds the first alone.
        let f = harness("gs-harness", "", "pub fn d() {}\n", "");
        let roots = "(defroot refm (role reference-model) (package \"crates/r\") (target lib) (role-packages \"crates/r\"))\n\
                     (defroot imp (role implementation) (package \"crates/i\") (target lib) (role-packages \"crates/i\"))\n\
                     (defharness diff (pair refm imp) (package \"crates/i\") (test diff))\n"
            .to_owned()
            + &form("crates/i/tests/table.rs", &["scripts/gen.sh"], &[])
            + &form("crates/r/src/gen_r.rs", &["scripts/other.sh"], &[]);
        f.commit(&[
            ("trust/roots.eadl", roots),
            ("scripts/gen.sh", "echo table\n".to_owned()),
            ("scripts/other.sh", "echo other\n".to_owned()),
            ("crates/i/tests/table.rs", "pub const T: u32 = 3;\n".to_owned()),
            (
                "crates/i/tests/diff.rs",
                "mod table;\n#[test]\nfn same() { assert_eq!(i::imp(table::T), r::refm(table::T)); }\n".to_owned(),
            ),
            ("crates/r/src/gen_r.rs", "pub const R: u32 = 0;\n".to_owned()),
            ("crates/r/src/lib.rs", "mod gen_r;\npub fn refm(x: u32) -> u32 { x + gen_r::R }\n".to_owned()),
        ]);
        let inv = written(f.run());
        assert_eq!(provenance_of(&inv, "diff"), ["scripts/gen.sh generator"]);
        assert_eq!(provenance_of(&inv, "refm"), ["scripts/other.sh generator"]);
        let harness_item = items(&inv)
            .into_iter()
            .find(|(_, i)| kind(i) == "comparison-harness")
            .expect("the pair's comparison harness")
            .1;
        let prov = match harness_item.get("provenance") {
            Some(Json::Object(m)) => m.keys().cloned().collect::<Vec<_>>(),
            other => panic!("no provenance aspect: {other:?}"),
        };
        assert_eq!(prov, ["scripts/gen.sh"]);
    }

    #[test]
    fn a_third_program_reaching_the_same_provenance_makes_items_of_its_own_and_moves_none() {
        let forms = form("crates/a/src/gen_a.rs", &["scripts/gen.sh"], &[])
            + &form("crates/b/src/gen_b.rs", &["scripts/gen.sh"], &[]);
        let f = generated_pair("gs-third", &[("trust/roots.eadl", roots("", "", &forms))]);
        let before = written(f.run());
        let item = |inv: &Json| {
            items(inv)
                .into_iter()
                .find(|(p, i)| p == "gen+chk" && kind(i) == "generated-provenance")
                .map(|(_, i)| crate::json::write(&i))
        };
        let third = forms
            + &form("crates/c/src/gen_c.rs", &["scripts/gen.sh"], &[])
            + "(defroot third (role reference-model) (package \"crates/c\") (target lib) (role-packages \"crates/c\"))\n";
        f.commit(&[
            ("trust/roots.eadl", roots("", "", &third)),
            ("crates/c/Cargo.toml", manifest("c", "")),
            (
                "crates/c/src/lib.rs",
                "mod gen_c;\npub fn c() -> u32 { gen_c::T }\n".to_owned(),
            ),
            (
                "crates/c/src/gen_c.rs",
                "pub const T: u32 = 9;\n".to_owned(),
            ),
        ]);
        let after = written(f.run());
        assert_eq!(
            item(&before),
            item(&after),
            "the third program moved the first pair's item"
        );
        for pair in ["gen+third", "chk+third"] {
            assert_eq!(
                provenance_items(&after, pair),
                ["scripts/gen.sh generator/generator"],
                "{pair}"
            );
        }
    }

    #[test]
    fn step_3_refuses_a_live_form_s_non_blob_and_its_chain_and_holds_a_form_no_program_reads_to_its_text(
    ) {
        let base = form("crates/b/src/gen_b.rs", &["scripts/gen_b.sh"], &[]);
        // A form no program reads, naming a generator the commit does not hold: its text alone is judged (step 1).
        let idle = form("crates/a/src/unused.rs", &["scripts/missing.sh"], &[]);
        let f = generated_pair(
            "gs-step-3",
            &[("trust/roots.eadl", roots("", "", &(base.clone() + &idle)))],
        );
        written(f.run());
        let with = |forms: String| [("trust/roots.eadl", roots("", "", &(base.clone() + &forms)))];
        // A chain: an input a form declares.
        f.commit(&[
            (
                "trust/roots.eadl",
                roots(
                    "",
                    "",
                    &(base.clone()
                        + &form(
                            "crates/a/src/gen_a.rs",
                            &["scripts/gen_a.sh"],
                            &["data/mid.csv"],
                        )
                        + &form("data/mid.csv", &["scripts/gen.sh"], &[])),
                ),
            ),
            ("data/mid.csv", "mid\n".to_owned()),
        ]);
        let r = refused(f.run());
        says(&r, "`(defgenerated \"crates/a/src/gen_a.rs\" …)`'s input `data/mid.csv` is declared by a `defgenerated` form");
        assert_eq!(r.len(), 1, "{r:#?}");
        // A chain: an input whose header marks it generated.
        f.commit(&[
            (
                "trust/roots.eadl",
                roots(
                    "",
                    "",
                    &(base.clone()
                        + &form(
                            "crates/a/src/gen_a.rs",
                            &["scripts/gen_a.sh"],
                            &["data/table.toml"],
                        )),
                ),
            ),
            (
                "data/table.toml",
                "# @generated by a tool\nx = 1\n".to_owned(),
            ),
        ]);
        says(
            &refused(f.run()),
            "input `data/table.toml` has a header that marks it generated",
        );
        // The blob rule alone: an input the commit does not hold, though a form declares it.
        f.commit(&with(
            form(
                "crates/a/src/gen_a.rs",
                &["scripts/gen_a.sh"],
                &["data/gone.csv"],
            ) + &form("data/gone.csv", &["scripts/gen.sh"], &[]),
        ));
        let r = refused(f.run());
        says(&r, "input `data/gone.csv` is no blob of the commit");
        assert!(
            !r.iter().any(|x| x.contains("declared by")),
            "a non-blob is the blob rule's alone: {r:#?}"
        );
        // A generator that is a symbolic link.
        std::os::unix::fs::symlink("gen_a.sh", f.repo.join("scripts/link.sh")).unwrap();
        f.commit(&with(form(
            "crates/a/src/gen_a.rs",
            &["scripts/link.sh"],
            &[],
        )));
        says(
            &refused(f.run()),
            "generator `scripts/link.sh` is a symbolic link in the commit",
        );
        // An input at a gitlink: no blob of the commit.
        f.commit(&with(form(
            "crates/a/src/gen_a.rs",
            &["scripts/gen_a.sh"],
            &["vendor/sub"],
        )));
        let head = String::from_utf8(
            std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(&f.repo)
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        git(
            &f.repo,
            &[
                "update-index",
                "--add",
                "--cacheinfo",
                &format!("160000,{},vendor/sub", head.trim()),
            ],
        );
        git(&f.repo, &["commit", "-q", "-m", "a gitlink"]);
        says(
            &refused(f.run()),
            "input `vendor/sub` is no blob of the commit",
        );
    }
}

#[cfg(test)]
mod generator_tests {
    //! Step 4 of the record's order (§2, §5; `GS-H11`, `GS-H5`): each live form's generator files judged against each
    //! program that reads its declared file — another role's program target or script refused, a `.rs` file that is no
    //! program target's crate root refused, a crate root of two targets judged by their union, a non-`.rs` crate root
    //! refused before any build — and an allowed generator target built as a root is, its sites judged with the
    //! programs', a site both compile admitted once; a generator that is a root its own build.

    use crate::trust::tests::{
        admit, manifest, real_root, refused, says, two_roots, written, Fixture, ROOTS as TWO_ROOTS,
    };
    use crate::trust::{inventory_with, Outcome};

    /// A `defgenerated` form for `chk`'s generated module, made by `generator`.
    fn chk_table(generator: &str) -> String {
        format!(
            "(defgenerated \"crates/b/src/gen_b.rs\" (generator \"{generator}\") (command \"run it\") (reason \"b's table\"))\n"
        )
    }

    /// Two roots — `gen`, an executable of `crates/a`, and `chk`, a library of `crates/b` reading `src/gen_b.rs` — and
    /// `files` beside them.
    fn pair(name: &str, files: &[(&str, String)]) -> Fixture {
        let mut all: Vec<(&str, String)> = vec![
            (
                "crates/b/src/gen_b.rs",
                "pub const T: u32 = 2;\n".to_owned(),
            ),
            ("crates/a/gen.sh", "echo a's script\n".to_owned()),
            ("scripts/gen.sh", "echo a script in no member\n".to_owned()),
        ];
        all.extend(files.iter().cloned());
        two_roots(
            name,
            "fn main() {}\n",
            "mod gen_b;\npub fn f() -> u32 { gen_b::T }\n",
            &all,
        )
    }

    fn roots(forms: &str) -> (&'static str, String) {
        ("trust/roots.eadl", format!("{TWO_ROOTS}{forms}"))
    }

    #[test]
    fn another_role_s_program_target_script_or_library_is_refused_as_a_generator() {
        // `chk`'s table written by the generator's own executable: the crate root of a root of another role, not built.
        let f = pair(
            "gs-gen-other-role",
            &[roots(&chk_table("crates/a/src/main.rs"))],
        );
        says(
            &refused(f.run()),
            "trust-shared-program: `chk` reads `crates/b/src/gen_b.rs`, generated by `crates/a/src/main.rs`, the crate root \
             of the bin `a` of `crates/a`, whose build compiles `crates/a`, a role package of the generator — not built",
        );
        // By a script lying in the generator's package.
        f.commit(&[roots(&chk_table("crates/a/gen.sh"))]);
        says(
            &refused(f.run()),
            "trust-shared-program: `chk` reads `crates/b/src/gen_b.rs`, generated by `crates/a/gen.sh`, which lies in \
             `crates/a`, a role package of the generator",
        );
        // By a library's crate root, which is no program target's: the instrument computes no build of it.
        f.commit(&[roots(&chk_table("crates/b/src/lib.rs"))]);
        says(
            &refused(f.run()),
            "`(defgenerated \"crates/b/src/gen_b.rs\" …)`'s generator `crates/b/src/lib.rs` is a `.rs` file that is no \
             program target's crate root",
        );
        // By a script in no workspace member: believed, and the tool it runs with it (§8).
        f.commit(&[roots(&chk_table("scripts/gen.sh"))]);
        written(f.run());
    }

    #[test]
    fn a_crate_root_of_two_targets_is_judged_by_the_union_of_their_role_packages() {
        // `crates/u`'s executable alone compiles no role package, and is built; its example, rooted in the same file,
        // compiles the generator's package by a development edge: the union holds it, and `chk` may not read the table.
        let u = |extra: &str| {
            manifest(
                "u",
                &format!("[[bin]]\nname = \"u\"\npath = \"src/tool.rs\"\n{extra}[dev-dependencies]\na = {{ path = \"../a\" }}\n"),
            )
        };
        let f = pair(
            "gs-gen-two-targets",
            &[
                roots(&chk_table("crates/u/src/tool.rs")),
                ("crates/a/src/lib.rs", "pub fn g() {}\n".to_owned()),
                ("crates/u/Cargo.toml", u("")),
                ("crates/u/src/tool.rs", "fn main() {}\n".to_owned()),
            ],
        );
        written(f.run());
        assert!(
            f.base.join("out/target-generators/crates-u-bin-u").exists(),
            "the executable was not built"
        );
        f.commit(&[(
            "crates/u/Cargo.toml",
            u("[[example]]\nname = \"ue\"\npath = \"src/tool.rs\"\n"),
        )]);
        says(
            &refused(f.run()),
            "generated by `crates/u/src/tool.rs`, the crate root of the bin `u` of `crates/u` and the example `ue` of \
             `crates/u`, whose build compiles `crates/a`, a role package of the generator — not built",
        );
        assert!(
            !f.base
                .join("out/target-generators/crates-u-example-ue")
                .exists(),
            "a refused generator target was built"
        );
    }

    #[test]
    fn the_harness_may_use_its_pair_s_roles_and_no_other() {
        // `diff` reads `tests/table.rs`; a script in the reference model's package wrote it — its pair's role, allowed.
        let f = crate::trust::tests::harness("gs-gen-harness", "", "pub fn d() {}\n", "");
        let roots = |generator: &str| {
            format!(
                "(defroot refm (role reference-model) (package \"crates/r\") (target lib) (role-packages \"crates/r\"))\n\
                 (defroot imp (role implementation) (package \"crates/i\") (target lib) (role-packages \"crates/i\"))\n\
                 (defroot gx (role generator) (package \"crates/x\") (target lib) (role-packages \"crates/x\"))\n\
                 (defharness diff (pair refm imp) (package \"crates/i\") (test diff))\n\
                 (defgenerated \"crates/i/tests/table.rs\" (generator \"{generator}\") (command \"run it\") (reason \"a table\"))\n"
            )
        };
        f.commit(&[
            ("trust/roots.eadl", roots("crates/r/gen.sh")),
            ("crates/r/gen.sh", "echo r\n".to_owned()),
            ("crates/x/gen.sh", "echo x\n".to_owned()),
            ("crates/x/Cargo.toml", manifest("x", "")),
            ("crates/x/src/lib.rs", "pub fn x() {}\n".to_owned()),
            ("crates/i/tests/table.rs", "pub const T: u32 = 3;\n".to_owned()),
            (
                "crates/i/tests/diff.rs",
                "mod table;\n#[test]\nfn same() { assert_eq!(i::imp(table::T), r::refm(table::T)); }\n".to_owned(),
            ),
        ]);
        written(f.run());
        // A script in a third root's package: another role's, for the harness as for any program.
        f.commit(&[("trust/roots.eadl", roots("crates/x/gen.sh"))]);
        says(
            &refused(f.run()),
            "trust-shared-program: `diff` reads `crates/i/tests/table.rs`, generated by `crates/x/gen.sh`, which lies in \
             `crates/x`, a role package of the generator",
        );
    }

    #[test]
    fn a_crate_root_that_is_not_rs_is_refused_before_its_build() {
        let f = pair(
            "gs-gen-not-rs",
            &[
                roots(&chk_table("crates/t/gen/main.txt")),
                (
                    "crates/t/Cargo.toml",
                    manifest("t", "[[bin]]\nname = \"t\"\npath = \"gen/main.txt\"\n"),
                ),
                ("crates/t/gen/main.txt", "fn main() {}\n".to_owned()),
            ],
        );
        says(
            &refused(f.run()),
            "trust-undeclared-input: `generator crates/t/gen/main.txt` reaches `crates/t/gen/main.txt`, a crate root that \
             is not a `.rs` file",
        );
        assert!(
            !f.base.join("out/target-generators").exists(),
            "a refused generator was built"
        );
    }

    #[test]
    fn a_generator_that_is_a_root_is_its_own_build_and_one_a_live_form_s_refusal_names_is_never_built(
    ) {
        // `gen2`, a second root of the generator's role, reads a table `gen`'s executable wrote: the same role, and the
        // root's build is the generator's, not built again.
        let table = "(defgenerated \"crates/c/src/table.rs\" (generator \"crates/a/src/main.rs\") (command \"run it\") \
                     (reason \"c's table\"))\n";
        let gen2 = "(defroot gen2 (role generator) (package \"crates/c\") (target bin c) (role-packages \"crates/c\"))\n";
        let f = pair(
            "gs-gen-root",
            &[
                roots(&(gen2.to_owned() + table)),
                ("crates/c/Cargo.toml", manifest("c", "")),
                (
                    "crates/c/src/table.rs",
                    "pub const T: u32 = 1;\n".to_owned(),
                ),
                (
                    "crates/c/src/main.rs",
                    "mod table;\nfn main() { let _ = table::T; }\n".to_owned(),
                ),
            ],
        );
        written(f.run());
        assert!(
            !f.base.join("out/target-generators").exists(),
            "a root was built again as a generator"
        );
        // `gen` reading a table its own executable wrote, its build compiling the table: the generator's build reads a
        // declared file, the chain's second clause (step 5).
        f.commit(&[
            roots(
                "(defgenerated \"crates/a/src/gen_a.rs\" (generator \"crates/a/src/main.rs\") (command \"run it\") \
                 (reason \"a's table\"))\n",
            ),
            ("crates/a/src/gen_a.rs", "pub const T: u32 = 1;\n".to_owned()),
            ("crates/a/src/main.rs", "mod gen_a;\nfn main() { let _ = gen_a::T; }\n".to_owned()),
        ]);
        says(
            &refused(f.run()),
            "`crates/a/src/gen_a.rs`, which the build of the generator `crates/a/src/main.rs` reads, is a generated source \
             — a `defgenerated` form declares it — a chain of generators",
        );
        // `chk`'s table from a tool of no role, with an input step 3 refuses: the run ends there, the tool not built.
        f.commit(&[
            roots(
                "(defgenerated \"crates/b/src/gen_b.rs\" (generator \"crates/g/src/main.rs\") (inputs \"data/gone.csv\") \
                 (command \"run it\") (reason \"b's table\"))\n",
            ),
            ("crates/g/Cargo.toml", manifest("g", "")),
            ("crates/g/src/main.rs", "fn main() {}\n".to_owned()),
        ]);
        says(
            &refused(f.run()),
            "input `data/gone.csv` is no blob of the commit",
        );
        assert!(
            !f.base.join("out/target-generators").exists(),
            "a generator was built after step 3 refused"
        );
    }

    #[test]
    fn an_allowed_generator_is_built_and_its_sites_judged_with_the_programs_a_shared_site_admitted_once(
    ) {
        // `crates/g`, a tool of no role, writes `chk`'s table; it and `chk` both compile `crates/s`, which holds one
        // `#[no_mangle]` site; `g` holds one of its own.
        let site = "#[no_mangle]\npub extern \"C\" fn s() {}";
        let own = "#[no_mangle]\npub extern \"C\" fn h() {}";
        let files = |admissions: &str| -> Vec<(&'static str, String)> {
            vec![
                roots(&(chk_table("crates/g/src/main.rs") + admissions)),
                ("crates/s/Cargo.toml", manifest("s", "")),
                ("crates/s/src/lib.rs", format!("{site}\n")),
                (
                    "crates/g/Cargo.toml",
                    manifest("g", "[dependencies]\ns = { path = \"../s\" }\n"),
                ),
                (
                    "crates/g/src/main.rs",
                    format!("{own}\nfn main() {{ s::s(); }}\n"),
                ),
                (
                    "crates/b/Cargo.toml",
                    manifest("b", "[dependencies]\ns = { path = \"../s\" }\n"),
                ),
            ]
        };
        let shared = admit("crates/s/src/lib.rs", "no_mangle", site);
        let f = pair("gs-gen-built", &files(&shared));
        let r = refused(f.run());
        says(
            &r,
            "trust-undeclared-input: `crates/g/src/main.rs:1`: `no_mangle`, refused by the catalog's rules, and no \
             admission names it",
        );
        assert!(
            !r.iter().any(|x| x.contains("crates/s/src/lib.rs")),
            "a site two builds compile is one site: {r:#?}"
        );
        f.commit(&files(
            &(shared + &admit("crates/g/src/main.rs", "no_mangle", own)),
        ));
        let inv = written(f.run());
        let unused = inv
            .get("admissions-unused")
            .map(crate::json::Json::elements)
            .unwrap_or_default();
        assert!(
            unused.is_empty(),
            "an admission the generator's site uses is used: {unused:?}"
        );
        assert_eq!(
            inv.get("refused-sites").and_then(crate::json::Json::as_str),
            Some("2")
        );
    }

    #[test]
    fn a_generator_build_failing_leaves_step_4_unable_to_judge_with_its_refusals_beside_it() {
        let f = pair(
            "gs-gen-fails",
            &[
                roots(
                    &(chk_table("crates/g/src/main.rs")
                        + "(defgenerated \"crates/a/src/unread.rs\" (generator \"crates/b/src/lib.rs\") (command \"run it\") \
                           (reason \"never read\"))\n"
                        + "(defgenerated \"crates/b/src/other.rs\" (generator \"crates/b/src/lib.rs\") (command \"run it\") \
                           (reason \"b's other table\"))\n"),
                ),
                ("crates/b/src/other.rs", "pub const U: u32 = 3;\n".to_owned()),
                (
                    "crates/b/src/lib.rs",
                    "mod gen_b;\nmod other;\npub fn f() -> u32 { gen_b::T + other::U }\n".to_owned(),
                ),
                ("crates/g/Cargo.toml", manifest("g", "")),
                ("crates/g/src/main.rs", "fn main() { let _: u32 = \"no\"; }\n".to_owned()),
            ],
        );
        let out = f.base.join("out");
        let err = match inventory_with(&f.repo, "HEAD", &out, &real_root()) {
            Err(e) => e,
            Ok(Outcome::Written(_)) => panic!("written"),
            Ok(Outcome::Refused(r)) => panic!("refused, where the gate could not judge: {r:#?}"),
        };
        assert!(
            err.contains("`cargo build --bin g") && err.contains("failed"),
            "{err}"
        );
        assert!(
            err.contains("and beside it, 1 refusal(s):")
                && err.contains("generator `crates/b/src/lib.rs` is a `.rs` file"),
            "{err}"
        );
    }
}

#[cfg(test)]
mod order_tests {
    //! Step 5 and staleness (§2, §3, §6; `GS-H2`, `GS-H4`, `GS-H14`, `GS-H15`, `GS-H5`): the chain's second clause over
    //! what a generator build reads, the recogniser's refusal over what a program reads, one message a file; a refusal
    //! at each of the five steps ending the run with no form stale; and a form no program reads stale on the baseline's
    //! host alone.

    use crate::trust::tests::{
        manifest, real_root, refused, says, two_roots, written, Fixture, ROOTS as TWO_ROOTS,
    };
    use crate::trust::{inventory_with, Outcome};

    const NEVER: &str = "(defgenerated \"crates/a/src/never.rs\" (generator \"scripts/gen.sh\") (command \"run it\") \
                         (reason \"a table no program reads\"))\n";

    fn roots(forms: &str) -> (&'static str, String) {
        ("trust/roots.eadl", format!("{TWO_ROOTS}{forms}"))
    }

    /// Two roots, `gen` (`crates/a`, an executable) and `chk` (`crates/b`, a library reading `src/gen_b.rs`), a script
    /// in no member, and `files` beside them.
    fn pair(name: &str, a_src: &str, b_src: &str, files: &[(&str, String)]) -> Fixture {
        let mut all: Vec<(&str, String)> = vec![
            (
                "crates/b/src/gen_b.rs",
                "pub const T: u32 = 2;\n".to_owned(),
            ),
            ("scripts/gen.sh", "echo a table\n".to_owned()),
        ];
        all.extend(files.iter().cloned());
        two_roots(name, a_src, b_src, &all)
    }

    fn outcome(f: &Fixture) -> Result<Outcome, String> {
        inventory_with(&f.repo, "HEAD", &f.base.join("out"), &real_root())
    }

    #[test]
    fn step_5_refuses_a_chain_through_a_generator_build_and_an_undeclared_marked_file_one_message_a_file(
    ) {
        // `crates/g`, a tool of no role, writes `chk`'s table; its build compiles `helper.rs`, marked, and `crates/s`,
        // marked, which `chk` compiles too; `gen` compiles `table.rs`, marked and declared by no form.
        let f = pair(
            "gs-step-5",
            "mod table;\nfn main() { let _ = table::T; }\n",
            "mod gen_b;\npub fn f() -> u32 { s::s(); gen_b::T }\n",
            &[
                roots("(defgenerated \"crates/b/src/gen_b.rs\" (generator \"crates/g/src/main.rs\") (command \"run it\") (reason \"b's table\"))\n"),
                ("crates/a/src/table.rs", "// @generated by a tool\npub const T: u32 = 1;\n".to_owned()),
                ("crates/s/Cargo.toml", manifest("s", "")),
                ("crates/s/src/lib.rs", "// @generated by another tool\npub fn s() {}\n".to_owned()),
                ("crates/b/Cargo.toml", manifest("b", "[dependencies]\ns = { path = \"../s\" }\n")),
                ("crates/g/Cargo.toml", manifest("g", "[dependencies]\ns = { path = \"../s\" }\n")),
                ("crates/g/src/main.rs", "mod helper;\nfn main() { helper::h(); s::s(); }\n".to_owned()),
                ("crates/g/src/helper.rs", "// @generated by a tool\npub fn h() {}\n".to_owned()),
            ],
        );
        let r = refused(f.run());
        says(&r, "`crates/g/src/helper.rs`, which the build of the generator `crates/g/src/main.rs` reads, is a generated source — its header marks it generated");
        says(&r, "`crates/s/src/lib.rs`, which the build of the generator `crates/g/src/main.rs` reads, is a generated source");
        says(&r, "`gen` reads `crates/a/src/table.rs`, whose header marks it generated and which no `defgenerated` form declares");
        assert_eq!(
            r.iter()
                .filter(|x| x.contains("crates/s/src/lib.rs"))
                .count(),
            1,
            "a chain is refused by that rule alone, one message for one file: {r:#?}"
        );
        // A declared file a generator build reads is a chain too.
        f.commit(&[roots(
            "(defgenerated \"crates/b/src/gen_b.rs\" (generator \"crates/g/src/main.rs\") (command \"run it\") (reason \"b's table\"))\n\
             (defgenerated \"crates/g/src/helper.rs\" (generator \"scripts/gen.sh\") (command \"run it\") (reason \"a helper\"))\n\
             (defgenerated \"crates/a/src/table.rs\" (generator \"scripts/gen.sh\") (command \"run it\") (reason \"a's table\"))\n\
             (defgenerated \"crates/s/src/lib.rs\" (generator \"scripts/gen.sh\") (command \"run it\") (reason \"s\"))\n",
        )]);
        says(&refused(f.run()), "`crates/g/src/helper.rs`, which the build of the generator `crates/g/src/main.rs` reads, is a generated source — a `defgenerated` form declares it");
    }

    #[test]
    fn a_marked_file_no_form_can_declare_is_refused_when_a_program_reads_it() {
        // A file a tool writes outside any committed script — as cargo writes its lock — handed to `chk`, marked.
        let f = pair(
            "gs-no-form-can-declare",
            "fn main() {}\n",
            "mod gen_b;\npub fn f() -> u32 { gen_b::T }\n",
            &[
                (
                    "trust/roots.eadl",
                    TWO_ROOTS.replace(
                        "(role-packages \"crates/b\"))",
                        "(role-packages \"crates/b\") (data \"data/deps.toml\"))",
                    ),
                ),
                (
                    "data/deps.toml",
                    "# @generated by a dependency tool\n[x]\ny = 1\n".to_owned(),
                ),
            ],
        );
        says(&refused(f.run()), "`chk` reads `data/deps.toml`, whose header marks it generated and which no `defgenerated` form declares");
    }

    #[test]
    fn a_refusal_at_each_of_the_five_steps_ends_the_run_and_no_form_is_stale() {
        // Each run holds a form no program reads, stale only over an inventory; each refuses, so none is written.
        let f = pair(
            "gs-order",
            "fn main() {}\n",
            "mod gen_b;\npub fn f() -> u32 { gen_b::T }\n",
            &[roots(NEVER)],
        );
        let mut checked = Vec::new();
        let mut step = |name: &str, files: Vec<(&'static str, String)>, why: &str| {
            f.commit(&files);
            let r = match outcome(&f) {
                Ok(Outcome::Refused(r)) => r,
                Ok(Outcome::Written(_)) => panic!("{name}: written"),
                Err(e) => panic!("{name}: not judged: {e}"),
            };
            says(&r, why);
            assert!(
                !r.iter().any(|x| x.contains("trust-baseline-stale")),
                "{name}: a form stale in a refused run: {r:#?}"
            );
            assert!(
                !f.base.join("out/trust-dependencies.json").exists(),
                "{name}: an inventory written"
            );
            checked.push(name.to_owned());
        };
        step(
            "step 1, a malformed form",
            vec![roots(
                &(NEVER.to_owned()
                    + "(defgenerated \"x.rs\" (generator \"scripts/gen.sh\") (command \"c\"))\n"),
            )],
            "has no `(reason …)`",
        );
        step(
            "step 2, the parent's refusal in a build",
            vec![
                roots(NEVER),
                (
                    "crates/b/src/lib.rs",
                    "mod gen_b;\n#[no_mangle]\npub extern \"C\" fn f() -> u32 { gen_b::T }\n"
                        .to_owned(),
                ),
            ],
            "`no_mangle`, refused by the catalog's rules",
        );
        step(
            "step 3, a live form's input no blob, its upstream form unread",
            vec![
                roots(&(NEVER.to_owned()
                    + "(defgenerated \"crates/b/src/gen_b.rs\" (generator \"scripts/gen.sh\") (inputs \"data/mid.csv\") (command \"c\") (reason \"r\"))\n\
                       (defgenerated \"data/mid.csv\" (generator \"scripts/gen.sh\") (command \"c\") (reason \"r\"))\n")),
                ("crates/b/src/lib.rs", "mod gen_b;\npub fn f() -> u32 { gen_b::T }\n".to_owned()),
            ],
            "input `data/mid.csv` is no blob of the commit",
        );
        step(
            "step 4, a generator target refused by its role packages",
            vec![roots(&(NEVER.to_owned()
                + "(defgenerated \"crates/b/src/gen_b.rs\" (generator \"crates/a/src/main.rs\") (command \"c\") (reason \"r\"))\n"))],
            "a role package of the generator — not built",
        );
        step(
            "step 5, a marked file no form declares",
            vec![
                roots(NEVER),
                (
                    "crates/b/src/gen_b.rs",
                    "// @generated by a tool\npub const T: u32 = 2;\n".to_owned(),
                ),
            ],
            "whose header marks it generated and which no `defgenerated` form declares",
        );
        assert_eq!(checked.len(), 5);
    }

    #[test]
    fn a_form_no_program_reads_is_stale_on_the_baseline_s_host_alone() {
        // `chk`'s table declared and read; `never.rs` declared and read by none.
        let live = "(defgenerated \"crates/b/src/gen_b.rs\" (generator \"scripts/gen.sh\") (command \"c\") (reason \"r\"))\n";
        let f = pair(
            "gs-stale",
            "fn main() {}\n",
            "mod gen_b;\npub fn f() -> u32 { gen_b::T }\n",
            &[
                roots(&(live.to_owned() + NEVER)),
                (
                    "crates/a/src/never.rs",
                    "pub const N: u32 = 0;\n".to_owned(),
                ),
            ],
        );
        let inv = written(f.run());
        let unread: Vec<&str> = inv
            .get("generated-unread")
            .map(crate::json::Json::elements)
            .unwrap_or_default()
            .iter()
            .filter_map(crate::json::Json::as_str)
            .collect();
        assert_eq!(unread, ["crates/a/src/never.rs"]);
        // Off the baseline's host — no baseline yet — nothing is stale.
        let out = f.base.join("gate");
        let (j, _) =
            crate::trust_gate::gate(&f.repo, "HEAD", None, &out, &real_root()).expect("judged");
        assert!(j.refused.is_empty(), "{:#?}", j.refused);
        // The proposed baseline committed, the gate is on its host: the unread form is stale, and it alone.
        let proposal =
            std::fs::read_to_string(out.join(crate::trust_gate::PROPOSAL)).expect("a proposal");
        f.commit(&[("trust/baseline.eadl", proposal)]);
        let (j, _) =
            crate::trust_gate::gate(&f.repo, "HEAD", None, &out, &real_root()).expect("judged");
        assert_eq!(
            j.refused,
            ["trust-baseline-stale: trust/roots.eadl: `(defgenerated \"crates/a/src/never.rs\" …)` declares a file no program reads — remove it, so a generated source read again is reviewed again"]
        );
    }
}

#[cfg(test)]
mod gate_tests {
    //! The gate's report and change part for committed generated sources (§2, §8; `GS-H7`, `GS-H5`): a `defgenerated`
    //! form added, changed and removed in the change part and named in the standing list; an edit to a declared
    //! generator reported and to an undeclared script "unchanged"; the report stating what the inventory does not see.

    use crate::trust::tests::{real_root, two_roots, Fixture, ROOTS as TWO_ROOTS};
    use crate::trust_gate::{gate, Judgement, PROPOSAL};

    fn judge(f: &Fixture, out: &str) -> (Judgement, String) {
        gate(&f.repo, "HEAD", None, &f.base.join(out), &real_root()).expect("judged")
    }

    fn pair(name: &str, forms: &str) -> Fixture {
        two_roots(
            name,
            "mod gen_a;\nfn main() { let _ = gen_a::T; }\n",
            "mod gen_b;\npub fn f() -> u32 { gen_b::T }\n",
            &[
                ("trust/roots.eadl", format!("{TWO_ROOTS}{forms}")),
                (
                    "crates/a/src/gen_a.rs",
                    "pub const T: u32 = 1;\n".to_owned(),
                ),
                (
                    "crates/b/src/gen_b.rs",
                    "pub const T: u32 = 2;\n".to_owned(),
                ),
                ("scripts/gen.sh", "echo a table\n".to_owned()),
                ("scripts/other.sh", "echo something else\n".to_owned()),
            ],
        )
    }

    fn form(path: &str, reason: &str) -> String {
        format!("(defgenerated \"{path}\" (generator \"scripts/gen.sh\") (command \"bash scripts/gen.sh\") (reason \"{reason}\"))\n")
    }

    #[test]
    fn a_defgenerated_form_added_changed_or_removed_is_in_the_change_part_and_every_one_is_standing(
    ) {
        let f = pair("gs-gate-forms", "");
        judge(&f, "first");
        f.commit(&[(
            "trust/roots.eadl",
            format!("{TWO_ROOTS}{}", form("crates/b/src/gen_b.rs", "b's table")),
        )]);
        let (j, text) = judge(&f, "added");
        assert!(
            j.change
                .contains(&"root form added: `defgenerated crates/b/src/gen_b.rs`".to_owned()),
            "{:#?}",
            j.change
        );
        assert!(
            j.standing.contains(
                &"generated source `crates/b/src/gen_b.rs`: generator scripts/gen.sh; inputs none"
                    .to_owned()
            ),
            "{:#?}",
            j.standing
        );
        for line in [
            "== what the inventory does not see ==",
            "a declaration is believed, not verified",
            "code a `cfg` gates",
        ] {
            assert!(text.contains(line), "{line}: {text}");
        }
        f.commit(&[(
            "trust/roots.eadl",
            format!(
                "{TWO_ROOTS}{}",
                form("crates/b/src/gen_b.rs", "b's table, regenerated")
            ),
        )]);
        let (j, _) = judge(&f, "changed");
        assert!(
            j.change
                .contains(&"root form changed: `defgenerated crates/b/src/gen_b.rs`".to_owned()),
            "{:#?}",
            j.change
        );
        f.commit(&[("trust/roots.eadl", TWO_ROOTS.to_owned())]);
        let (j, _) = judge(&f, "removed");
        assert!(
            j.change
                .contains(&"root form removed: `defgenerated crates/b/src/gen_b.rs`".to_owned()),
            "{:#?}",
            j.change
        );
    }

    #[test]
    fn an_edit_to_a_declared_generator_is_reported_and_to_an_undeclared_script_is_unchanged() {
        // Both roots' tables written by `scripts/gen.sh`: a generated-provenance item, its form proposed and committed,
        // so every later commit is judged on the baseline's host.
        let forms = form("crates/a/src/gen_a.rs", "a's table")
            + &form("crates/b/src/gen_b.rs", "b's table");
        let f = pair("gs-gate-edits", &forms);
        judge(&f, "first");
        let proposal =
            std::fs::read_to_string(f.base.join("first").join(PROPOSAL)).expect("a proposal");
        f.commit(&[("trust/baseline.eadl", proposal)]);
        let (j, _) = judge(&f, "proposed");
        assert!(j.refused.is_empty(), "{:#?}", j.refused);
        // A script no live form names: nothing shared moves.
        f.commit(&[(
            "scripts/other.sh",
            "echo something else, edited\n".to_owned(),
        )]);
        let (j, _) = judge(&f, "other");
        assert_eq!(j.change, ["unchanged"], "{:#?}", j.refused);
        assert!(j.refused.is_empty(), "{:#?}", j.refused);
        // The declared generator: its item's digest moves, reported, and on the baseline's host its form is missing.
        f.commit(&[("scripts/gen.sh", "echo a table, edited\n".to_owned())]);
        let (j, _) = judge(&f, "generator");
        assert!(
            j.change.contains(
                &"trust-shared-changed: `gen+chk generated-provenance scripts/gen.sh` — sha256"
                    .to_owned()
            ),
            "{:#?}",
            j.change
        );
        assert!(
            j.refused.iter().any(|r| r.starts_with(
                "trust-form-missing: `gen+chk generated-provenance scripts/gen.sh`'s form"
            )),
            "{:#?}",
            j.refused
        );
    }
}

#[cfg(test)]
mod route_tests {
    //! The refusals record's routes and no-route cases, each held here (`decision_trust-generated-refusals.md`, the
    //! section each test names; leaf `M3.6.6.4`, review rounds 5 to 12): the record claims a route only where a test
    //! here builds it and the gate writes the inventory, and a case with no route only where a test here, or one its §6
    //! names in another module, sees the gate refuse it — or, for what the record says is believed, sees the gate write
    //! it unseen. Review rounds probed routes the record claimed in untracked tests; these make the claims the code's.

    use crate::json::Json;
    use crate::trust::tests::{
        git, manifest, refused, says, two_roots, written, Fixture, ROOTS as TWO_ROOTS,
    };

    fn roots(forms: &str) -> (&'static str, String) {
        ("trust/roots.eadl", format!("{TWO_ROOTS}{forms}"))
    }

    /// A form declaring `path`, made by `generators` from `inputs`.
    fn form(path: &str, generators: &[&str], inputs: &[&str]) -> String {
        let list = |xs: &[&str]| {
            xs.iter()
                .map(|x| format!("\"{x}\""))
                .collect::<Vec<_>>()
                .join(" ")
        };
        let inputs = if inputs.is_empty() {
            String::new()
        } else {
            format!(" (inputs {})", list(inputs))
        };
        format!(
            "(defgenerated \"{path}\" (generator {}){inputs} (command \"run it\") (reason \"a route\"))\n",
            list(generators)
        )
    }

    /// The paths of `program`'s provenance.
    fn reached(inv: &Json, program: &str) -> Vec<String> {
        let p = inv
            .get("programs")
            .map(Json::elements)
            .unwrap_or_default()
            .iter()
            .find(|p| p.get("name").and_then(Json::as_str) == Some(program))
            .expect("the program's record");
        match p.get("provenance") {
            Some(Json::Object(m)) => m.keys().cloned().collect(),
            other => panic!("no provenance in {program}'s record: {other:?}"),
        }
    }

    fn holds(reached: &[String], paths: &[&str]) {
        for p in paths {
            assert!(
                reached.iter().any(|r| r == p),
                "`{p}` not reached: {reached:?}"
            );
        }
    }

    /// `chk`, a library of `crates/b` reading its generated table `src/gen_b.rs`, and `files` beside it.
    fn chk(name: &str, files: &[(&str, String)]) -> Fixture {
        let mut all = vec![(
            "crates/b/src/gen_b.rs",
            "pub const T: u32 = 2;\n".to_owned(),
        )];
        all.extend(files.iter().cloned());
        two_roots(
            name,
            "fn main() {}\n",
            "mod gen_b;\npub fn f() -> u32 { gen_b::T }\n",
            &all,
        )
    }

    #[test]
    fn route_a_chain_made_in_one_generation() {
        // §2. Control: the intermediate committed, declared, and read by the second step — a chain, refused.
        let f = chk(
            "route-chain",
            &[
                ("scripts/step1.sh", "echo step one\n".to_owned()),
                ("scripts/step2.sh", "echo step two\n".to_owned()),
                ("data/in.csv", "1,2\n".to_owned()),
                ("data/mid.csv", "3\n".to_owned()),
                roots(
                    &(form("data/mid.csv", &["scripts/step1.sh"], &["data/in.csv"])
                        + &form(
                            "crates/b/src/gen_b.rs",
                            &["scripts/step2.sh"],
                            &["data/mid.csv"],
                        )),
                ),
            ],
        );
        says(
            &refused(f.run()),
            "is declared by a `defgenerated` form — a chain of generators",
        );
        // Control, the generator half: a script a form declares, named as the later step's generator — a chain too.
        f.commit(&[
            ("scripts/made.sh", "echo made\n".to_owned()),
            roots(
                &(form("scripts/made.sh", &["scripts/step1.sh"], &["data/in.csv"])
                    + &form("crates/b/src/gen_b.rs", &["scripts/made.sh"], &[])),
            ),
        ]);
        says(
            &refused(f.run()),
            "generator `scripts/made.sh` is declared by a `defgenerated` form — a chain of generators",
        );
        std::fs::remove_file(f.repo.join("scripts/made.sh")).unwrap();
        // The route: one form names every step's generator files and every committed file any step reads; the
        // intermediate is the generation's own, never committed.
        std::fs::remove_file(f.repo.join("data/mid.csv")).unwrap();
        f.commit(&[
            ("data/step2.csv", "4\n".to_owned()),
            ("data/third.csv", "5\n".to_owned()),
            roots(&form(
                "crates/b/src/gen_b.rs",
                &["scripts/step1.sh", "scripts/step2.sh"],
                &["data/in.csv", "data/step2.csv", "data/third.csv"],
            )),
        ]);
        holds(
            &reached(&written(f.run()), "chk"),
            &[
                "scripts/step1.sh",
                "scripts/step2.sh",
                "data/in.csv",
                "data/step2.csv",
                "data/third.csv",
            ],
        );
    }

    #[test]
    fn route_a_marked_input_or_script_remade_in_the_generation() {
        // §2, §3. Control: a vendored copy whose header marks it generated, named as an input — a chain, refused.
        let f = chk(
            "route-marked-input",
            &[
                (
                    "scripts/gen.sh",
                    "python3 third_party/up/gen.py third_party/up/table.csv\n".to_owned(),
                ),
                (
                    "third_party/up/table.h",
                    "// @generated by gen.py from table.csv - do not edit\n#define T 2\n"
                        .to_owned(),
                ),
                roots(&form(
                    "crates/b/src/gen_b.rs",
                    &["scripts/gen.sh"],
                    &["third_party/up/table.h"],
                )),
            ],
        );
        says(
            &refused(f.run()),
            "has a header that marks it generated — a chain of generators",
        );
        // The route: the marked file never committed, remade within the generation from committed copies of its own
        // generator and input, each named.
        std::fs::remove_file(f.repo.join("third_party/up/table.h")).unwrap();
        f.commit(&[
            ("third_party/up/gen.py", "print('#define T 2')\n".to_owned()),
            ("third_party/up/table.csv", "T,2\n".to_owned()),
            roots(&form(
                "crates/b/src/gen_b.rs",
                &["scripts/gen.sh", "third_party/up/gen.py"],
                &["third_party/up/table.csv"],
            )),
        ]);
        holds(
            &reached(&written(f.run()), "chk"),
            &[
                "scripts/gen.sh",
                "third_party/up/gen.py",
                "third_party/up/table.csv",
            ],
        );
        // A marked script named as a generator: a chain, refused; remade the same way, its own generator named.
        f.commit(&[
            (
                "scripts/made.sh",
                "# @generated by scripts/make.py - do not edit\necho made\n".to_owned(),
            ),
            roots(&form("crates/b/src/gen_b.rs", &["scripts/made.sh"], &[])),
        ]);
        says(
            &refused(f.run()),
            "generator `scripts/made.sh` has a header that marks it generated",
        );
        std::fs::remove_file(f.repo.join("scripts/made.sh")).unwrap();
        f.commit(&[
            ("scripts/make.py", "print('echo made')\n".to_owned()),
            roots(&form(
                "crates/b/src/gen_b.rs",
                &["scripts/gen.sh", "scripts/make.py"],
                &[],
            )),
        ]);
        holds(&reached(&written(f.run()), "chk"), &["scripts/make.py"]);
    }

    #[test]
    fn no_route_for_a_marked_crate_root_but_the_parent_s_belief_in_a_script() {
        // §2. A crate root whose header marks it generated, named as a generator: a chain, refused — and no route, since
        // a program built from it compiles a generated file, a build the gate does not compute.
        let f = chk(
            "no-route-marked-root",
            &[
                ("crates/g/Cargo.toml", manifest("g", "")),
                (
                    "crates/g/src/main.rs",
                    "// @generated by scripts/remake.sh - do not edit\nfn main() {}\n".to_owned(),
                ),
                roots(&form(
                    "crates/b/src/gen_b.rs",
                    &["crates/g/src/main.rs"],
                    &[],
                )),
            ],
        );
        says(
            &refused(f.run()),
            "has a header that marks it generated — a chain of generators",
        );
        // Remade and built by a committed script instead, the script is the generator: written, and the program the
        // script builds is the script's own dependency, believed with whatever its build compiles — the parent's §8
        // limit, which the record states, not a route it offers.
        std::fs::remove_dir_all(f.repo.join("crates/g")).unwrap();
        f.commit(&[
            (
                "scripts/remake.sh",
                "sed s/x/x/ third_party/g_main.rs.in > $T/main.rs; cargo run\n".to_owned(),
            ),
            ("third_party/g_main.rs.in", "fn main() {}\n".to_owned()),
            roots(&form(
                "crates/b/src/gen_b.rs",
                &["scripts/remake.sh"],
                &["third_party/g_main.rs.in"],
            )),
        ]);
        holds(
            &reached(&written(f.run()), "chk"),
            &["scripts/remake.sh", "third_party/g_main.rs.in"],
        );
    }

    #[test]
    fn route_a_program_target_step_reads_the_intermediate_at_run_time() {
        // §2. A program target of a package in no role, the second step, reads the first step's output at run time:
        // its build compiles no generated file.
        let f = chk(
            "route-run-time",
            &[
                ("scripts/step1.sh", "echo 2 > $T/mid.txt\n".to_owned()),
                ("crates/w/Cargo.toml", manifest("w", "")),
                (
                    "crates/w/src/main.rs",
                    "fn main() {\n    let path = std::env::args().nth(1).unwrap_or_default();\n    \
                     let t = std::fs::read_to_string(path).unwrap_or_default();\n    \
                     println!(\"pub const T: u32 = {};\", t.trim());\n}\n"
                        .to_owned(),
                ),
                ("data/in.csv", "2\n".to_owned()),
                roots(&form(
                    "crates/b/src/gen_b.rs",
                    &["scripts/step1.sh", "crates/w/src/main.rs"],
                    &["data/in.csv"],
                )),
            ],
        );
        holds(
            &reached(&written(f.run()), "chk"),
            &["scripts/step1.sh", "crates/w/src/main.rs", "data/in.csv"],
        );
    }

    #[test]
    fn route_a_program_that_generates_its_own_source() {
        // §2. Control: `gen`'s executable writes the table its own build compiles — the chain's second clause.
        let own = |generator: &str| roots(&form("crates/a/src/gen_a.rs", &[generator], &[]));
        let f = two_roots(
            "route-own-source",
            "mod gen_a;\nfn main() { let _ = gen_a::T; }\n",
            "pub fn f() {}\n",
            &[
                (
                    "crates/a/src/gen_a.rs",
                    "pub const T: u32 = 1;\n".to_owned(),
                ),
                own("crates/a/src/main.rs"),
            ],
        );
        says(
            &refused(f.run()),
            "`crates/a/src/gen_a.rs`, which the build of the generator `crates/a/src/main.rs` reads, is a generated source",
        );
        // The route: a second executable of its package, which no library of the package makes compile the table.
        f.commit(&[
            (
                "crates/a/src/bin/gen_table.rs",
                "fn main() { println!(\"pub const T: u32 = 1;\"); }\n".to_owned(),
            ),
            own("crates/a/src/bin/gen_table.rs"),
        ]);
        holds(
            &reached(&written(f.run()), "gen"),
            &["crates/a/src/bin/gen_table.rs"],
        );
    }

    #[test]
    fn route_a_table_its_package_s_library_holds_is_made_by_another_package() {
        // §2. `crates/u`'s library holds the declared table, which `chk` reads through it. Its executable `u` writes it:
        // refused. A second executable of `crates/u` compiles that library, so the table: refused too.
        let u = manifest(
            "u",
            "[[bin]]\nname = \"u\"\npath = \"src/main.rs\"\n[[bin]]\nname = \"gen2\"\npath = \"src/bin/gen2.rs\"\n",
        );
        let table = |generator: &str| roots(&form("crates/u/src/table.rs", &[generator], &[]));
        let f = two_roots(
            "route-library-table",
            "fn main() {}\n",
            "pub fn f() -> u32 { u::table::T }\n",
            &[
                table("crates/u/src/main.rs"),
                (
                    "crates/b/Cargo.toml",
                    manifest("b", "[dependencies]\nu = { path = \"../u\" }\n"),
                ),
                ("crates/u/Cargo.toml", u),
                ("crates/u/src/lib.rs", "pub mod table;\n".to_owned()),
                (
                    "crates/u/src/table.rs",
                    "pub const T: u32 = 3;\n".to_owned(),
                ),
                (
                    "crates/u/src/main.rs",
                    "fn main() { println!(\"{}\", u::table::T); }\n".to_owned(),
                ),
                (
                    "crates/u/src/bin/gen2.rs",
                    "fn main() { println!(\"pub const T: u32 = 3;\"); }\n".to_owned(),
                ),
            ],
        );
        says(
            &refused(f.run()),
            "`crates/u/src/table.rs`, which the build of the generator `crates/u/src/main.rs` reads, is a generated source",
        );
        f.commit(&[table("crates/u/src/bin/gen2.rs")]);
        says(
            &refused(f.run()),
            "`crates/u/src/table.rs`, which the build of the generator `crates/u/src/bin/gen2.rs` reads, is a generated source",
        );
        // The route: an executable of a package that does not depend on `crates/u`.
        f.commit(&[
            ("crates/w/Cargo.toml", manifest("w", "")),
            (
                "crates/w/src/main.rs",
                "fn main() { println!(\"pub const T: u32 = 3;\"); }\n".to_owned(),
            ),
            table("crates/w/src/main.rs"),
        ]);
        holds(
            &reached(&written(f.run()), "chk"),
            &["crates/w/src/main.rs"],
        );
    }

    #[test]
    fn route_a_vendored_file_copied() {
        // §3. A gitlink's files are no blobs (the blob rule's test above); their copies are files of the commit. A
        // copied input, script and executable, the executable named beside the script that runs it: written, reached.
        let f = chk(
            "route-vendored-copies",
            &[
                (
                    "scripts/gen.sh",
                    "third_party/x/protoc --run third_party/x/gen.py third_party/x/x.proto\n"
                        .to_owned(),
                ),
                ("third_party/x/x.proto", "message T {}\n".to_owned()),
                (
                    "third_party/x/protoc",
                    "\u{7f}ELF\u{2}\u{1}\u{1} copied bytes \u{0}\u{1}\u{2}".to_owned(),
                ),
                (
                    "third_party/x/gen.py",
                    "print('pub const T: u32 = 2;')\n".to_owned(),
                ),
                roots(&form(
                    "crates/b/src/gen_b.rs",
                    &[
                        "scripts/gen.sh",
                        "third_party/x/protoc",
                        "third_party/x/gen.py",
                    ],
                    &["third_party/x/x.proto"],
                )),
            ],
        );
        {
            use std::os::unix::fs::PermissionsExt;
            let exe = f.repo.join("third_party/x/protoc");
            std::fs::set_permissions(&exe, std::fs::Permissions::from_mode(0o755)).unwrap();
            git(&f.repo, &["add", "-A"]);
            git(&f.repo, &["commit", "-q", "-m", "an executable"]);
        }
        holds(
            &reached(&written(f.run()), "chk"),
            &[
                "scripts/gen.sh",
                "third_party/x/protoc",
                "third_party/x/gen.py",
                "third_party/x/x.proto",
            ],
        );
        // A Rust generator copied as a lone `.rs` file is no program target's crate root: refused.
        f.commit(&[
            ("third_party/x/gen.rs", "fn main() {}\n".to_owned()),
            roots(&form(
                "crates/b/src/gen_b.rs",
                &["third_party/x/gen.rs"],
                &[],
            )),
        ]);
        says(
            &refused(f.run()),
            "generator `third_party/x/gen.rs` is a `.rs` file that is no",
        );
        // Copied as a program target of the workspace, its build computed: the route.
        f.commit(&[
            ("crates/v/Cargo.toml", manifest("v", "")),
            (
                "crates/v/src/main.rs",
                "fn main() { println!(\"pub const T: u32 = 2;\"); }\n".to_owned(),
            ),
            roots(&form(
                "crates/b/src/gen_b.rs",
                &["crates/v/src/main.rs"],
                &[],
            )),
        ]);
        holds(
            &reached(&written(f.run()), "chk"),
            &["crates/v/src/main.rs"],
        );
    }

    #[test]
    fn a_vendored_executable_a_script_runs_uncopied_is_believed() {
        // §3, §4. `vendor/x`, a gitlink, holds a tool a committed script runs; no form names it. Written: the script is
        // reached, the tool is in no provenance — believed, its bytes unseen.
        let f = chk(
            "believed-vendored",
            &[
                ("scripts/gen.sh", "vendor/x/tool > gen_b.rs\n".to_owned()),
                roots(&form("crates/b/src/gen_b.rs", &["scripts/gen.sh"], &[])),
            ],
        );
        let head = String::from_utf8(
            std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(&f.repo)
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        git(
            &f.repo,
            &[
                "update-index",
                "--add",
                "--cacheinfo",
                &format!("160000,{},vendor/x", head.trim()),
            ],
        );
        git(&f.repo, &["commit", "-q", "-m", "a gitlink"]);
        let r = reached(&written(f.run()), "chk");
        holds(&r, &["scripts/gen.sh"]);
        assert!(r.iter().all(|p| !p.starts_with("vendor/")), "{r:?}");
        // The control: the same tool named on the form is no blob of the commit, refused — the belief above is the
        // script's, not the gate's reading of the tool.
        f.commit(&[roots(&form(
            "crates/b/src/gen_b.rs",
            &["scripts/gen.sh", "vendor/x/tool"],
            &[],
        ))]);
        says(
            &refused(f.run()),
            "generator `vendor/x/tool` is no blob of the commit",
        );
    }

    #[test]
    fn route_a_tool_pin() {
        // §4. An unmarked version file the script checks the tool against, named as an input: reached.
        let f = chk(
            "route-tool-pin",
            &[
                (
                    "scripts/gen.sh",
                    "protoc --version | grep -qF \"$(cat tools/protoc.version)\" && protoc --rust_out=. x.proto\n"
                        .to_owned(),
                ),
                ("tools/protoc.version", "libprotoc 28.2\n".to_owned()),
                roots(&form("crates/b/src/gen_b.rs", &["scripts/gen.sh"], &["tools/protoc.version"])),
            ],
        );
        holds(
            &reached(&written(f.run()), "chk"),
            &["tools/protoc.version"],
        );
        // A pin whose header marks it generated, a lock a tool wrote: a chain, refused — the version file above is
        // its route.
        f.commit(&[
            (
                "tools/protoc.lock.toml",
                "# @generated by the lock tool - do not edit\nversion = \"28.2\"\n".to_owned(),
            ),
            roots(&form(
                "crates/b/src/gen_b.rs",
                &["scripts/gen.sh"],
                &["tools/protoc.lock.toml"],
            )),
        ]);
        says(
            &refused(f.run()),
            "input `tools/protoc.lock.toml` has a header that marks it generated",
        );
    }

    #[test]
    fn route_a_table_a_dependency_s_library_holds_is_made_by_a_package_not_depending_on_it() {
        // §2. `gen`'s package depends on `crates/t`, whose library holds the declared table: every executable of the
        // package compiles it, a second one too — refused; an executable of a package that does not depend on
        // `crates/t` is the route.
        let table = |generator: &str| roots(&form("crates/t/src/table.rs", &[generator], &[]));
        let f = two_roots(
            "route-dependency-table",
            "fn main() { let _ = t::table::T; }\n",
            "pub fn f() {}\n",
            &[
                (
                    "crates/a/Cargo.toml",
                    manifest("a", "[dependencies]\nt = { path = \"../t\" }\n"),
                ),
                ("crates/t/Cargo.toml", manifest("t", "")),
                ("crates/t/src/lib.rs", "pub mod table;\n".to_owned()),
                (
                    "crates/t/src/table.rs",
                    "pub const T: u32 = 1;\n".to_owned(),
                ),
                table("crates/a/src/main.rs"),
            ],
        );
        says(
            &refused(f.run()),
            "`crates/t/src/table.rs`, which the build of the generator `crates/a/src/main.rs` reads, is a generated source",
        );
        f.commit(&[
            (
                "crates/a/src/bin/gen2.rs",
                "fn main() { println!(\"pub const T: u32 = 1;\"); }\n".to_owned(),
            ),
            table("crates/a/src/bin/gen2.rs"),
        ]);
        says(
            &refused(f.run()),
            "`crates/t/src/table.rs`, which the build of the generator `crates/a/src/bin/gen2.rs` reads, is a generated source",
        );
        // Through another package: `crates/m` depends on `crates/t`, `crates/v` on `crates/m` — refused too.
        f.commit(&[
            (
                "crates/m/Cargo.toml",
                manifest("m", "[dependencies]\nt = { path = \"../t\" }\n"),
            ),
            (
                "crates/m/src/lib.rs",
                "pub fn m() -> u32 { t::table::T }\n".to_owned(),
            ),
            (
                "crates/v/Cargo.toml",
                manifest("v", "[dependencies]\nm = { path = \"../m\" }\n"),
            ),
            (
                "crates/v/src/main.rs",
                "fn main() { println!(\"pub const T: u32 = {};\", m::m()); }\n".to_owned(),
            ),
            table("crates/v/src/main.rs"),
        ]);
        says(
            &refused(f.run()),
            "`crates/t/src/table.rs`, which the build of the generator `crates/v/src/main.rs` reads, is a generated source",
        );
        f.commit(&[
            ("crates/w/Cargo.toml", manifest("w", "")),
            (
                "crates/w/src/main.rs",
                "fn main() { println!(\"pub const T: u32 = 1;\"); }\n".to_owned(),
            ),
            table("crates/w/src/main.rs"),
        ]);
        holds(
            &reached(&written(f.run()), "gen"),
            &["crates/w/src/main.rs"],
        );
    }

    #[test]
    fn route_an_intermediate_a_program_reads_is_remade_by_the_later_step() {
        // §2. `chk` reads the intermediate itself, a `data` file of its root, so it is committed and declared. Control:
        // the later step reading it as an input — a chain, refused. The route: the later step remakes it from the first
        // step's inputs.
        let roots = |forms: String| {
            (
                "trust/roots.eadl",
                format!(
                    "(defroot gen (role generator) (package \"crates/a\") (target bin a) (role-packages \"crates/a\"))\n\
                     (defroot chk (role scheduling-checker) (package \"crates/b\") (target lib) (role-packages \"crates/b\") \
                     (data \"data/mid.csv\"))\n{forms}"
                ),
            )
        };
        let mid = form("data/mid.csv", &["scripts/step1.sh"], &["data/in.csv"]);
        let f = chk(
            "route-read-intermediate",
            &[
                ("scripts/step1.sh", "echo step one\n".to_owned()),
                ("scripts/step2.sh", "echo step two\n".to_owned()),
                ("data/in.csv", "1,2\n".to_owned()),
                ("data/mid.csv", "3\n".to_owned()),
                roots(
                    mid.clone()
                        + &form(
                            "crates/b/src/gen_b.rs",
                            &["scripts/step2.sh"],
                            &["data/mid.csv"],
                        ),
                ),
            ],
        );
        says(
            &refused(f.run()),
            "is declared by a `defgenerated` form — a chain of generators",
        );
        f.commit(&[roots(
            mid + &form(
                "crates/b/src/gen_b.rs",
                &["scripts/step1.sh", "scripts/step2.sh"],
                &["data/in.csv"],
            ),
        )]);
        holds(
            &reached(&written(f.run()), "chk"),
            &["scripts/step1.sh", "scripts/step2.sh", "data/in.csv"],
        );
    }

    #[test]
    fn route_a_file_one_step_runs_and_another_reads_is_named_once() {
        // §2. A file one step runs and another reads, named in both clauses: refused; named once, as a generator: written.
        let f = chk(
            "route-named-once",
            &[
                (
                    "scripts/gen.sh",
                    "bash scripts/lib.sh; cat scripts/lib.sh\n".to_owned(),
                ),
                ("scripts/lib.sh", "echo shared\n".to_owned()),
                roots(&form(
                    "crates/b/src/gen_b.rs",
                    &["scripts/gen.sh", "scripts/lib.sh"],
                    &["scripts/lib.sh"],
                )),
            ],
        );
        says(
            &refused(f.run()),
            "names `scripts/lib.sh` as both a generator and an input",
        );
        f.commit(&[roots(&form(
            "crates/b/src/gen_b.rs",
            &["scripts/gen.sh", "scripts/lib.sh"],
            &[],
        ))]);
        holds(
            &reached(&written(f.run()), "chk"),
            &["scripts/gen.sh", "scripts/lib.sh"],
        );
    }

    #[test]
    fn route_a_chain_judges_every_step_s_generator() {
        // §2. One generation's form names every step's generator, and each is judged by the parent's §5 against the
        // program that reads the result: another role's step's generator, named last or first, is refused.
        let f = chk(
            "route-every-step-judged",
            &[
                ("scripts/step2.sh", "echo step two\n".to_owned()),
                ("crates/a/step1.sh", "echo step one\n".to_owned()),
                roots(&form(
                    "crates/b/src/gen_b.rs",
                    &["scripts/step2.sh", "crates/a/step1.sh"],
                    &[],
                )),
            ],
        );
        let foreign = "trust-shared-program: `chk` reads `crates/b/src/gen_b.rs`, generated by `crates/a/step1.sh`, which \
                       lies in `crates/a`, a role package of the generator";
        says(&refused(f.run()), foreign);
        // Named first, the same step's generator is judged as well: neither the first nor the last alone.
        f.commit(&[roots(&form(
            "crates/b/src/gen_b.rs",
            &["crates/a/step1.sh", "scripts/step2.sh"],
            &[],
        ))]);
        says(&refused(f.run()), foreign);
    }

    /// Three roots reading `crates/s/src/table.rs` — `gen` (role packages `crates/a`, `crates/x`), `chk` (`crates/b`,
    /// `crates/y`) and `imp` (`crates/i`) — their `defroot` lines in `order`, then `forms`.
    fn three_readers_roots(order: &[&str], forms: &str) -> (&'static str, String) {
        let line = |r: &str| {
            match r {
            "gen" => "(defroot gen (role generator) (package \"crates/a\") (target bin a) \
                      (role-packages \"crates/a\" \"crates/x\"))\n",
            "chk" => "(defroot chk (role scheduling-checker) (package \"crates/b\") (target lib) \
                      (role-packages \"crates/b\" \"crates/y\"))\n",
            _ => "(defroot imp (role implementation) (package \"crates/i\") (target lib) (role-packages \"crates/i\"))\n",
        }
        };
        (
            "trust/roots.eadl",
            order.iter().map(|r| line(r)).collect::<String>() + forms,
        )
    }

    /// The three readers' fixture: each reads `crates/s`'s table; a script and a program target in a role package of
    /// `gen` and of `chk` beside them.
    fn three_readers(name: &str) -> Fixture {
        let dep = |n: &str| manifest(n, "[dependencies]\ns = { path = \"../s\" }\n");
        two_roots(
            name,
            "fn main() { let _ = s::table::T; }\n",
            "pub fn f() -> u32 { s::table::T }\n",
            &[
                ("crates/a/Cargo.toml", dep("a")),
                ("crates/b/Cargo.toml", dep("b")),
                ("crates/i/Cargo.toml", dep("i")),
                (
                    "crates/i/src/lib.rs",
                    "pub fn f() -> u32 { s::table::T }\n".to_owned(),
                ),
                ("crates/s/Cargo.toml", manifest("s", "")),
                ("crates/s/src/lib.rs", "pub mod table;\n".to_owned()),
                (
                    "crates/s/src/table.rs",
                    "pub const T: u32 = 1;\n".to_owned(),
                ),
                ("crates/a/gen.sh", "echo a table\n".to_owned()),
                ("crates/b/gen.sh", "echo b table\n".to_owned()),
                ("crates/x/Cargo.toml", manifest("x", "")),
                (
                    "crates/x/src/main.rs",
                    "fn main() { println!(\"pub const T: u32 = 1;\"); }\n".to_owned(),
                ),
                ("crates/y/Cargo.toml", manifest("y", "")),
                (
                    "crates/y/src/main.rs",
                    "fn main() { println!(\"pub const T: u32 = 1;\"); }\n".to_owned(),
                ),
                three_readers_roots(&["gen", "chk", "imp"], ""),
            ],
        )
    }

    /// Every order of the three readers — each first, in the middle and last, before and after each other one.
    const ORDERS: [[&str; 3]; 6] = [
        ["gen", "chk", "imp"],
        ["gen", "imp", "chk"],
        ["chk", "gen", "imp"],
        ["chk", "imp", "gen"],
        ["imp", "gen", "chk"],
        ["imp", "chk", "gen"],
    ];

    #[test]
    fn route_a_generator_is_judged_against_every_reader() {
        // §2, §3. A script generating a declared file three programs read is judged against each, in every order of the
        // three: `gen`'s script refused for `chk` and for `imp`, `chk`'s for `gen` and for `imp` (review rounds 9, 12, 13).
        let f = three_readers("route-every-reader-judged");
        for (script, owner, pkg, role) in [
            ("crates/a/gen.sh", "gen", "crates/a", "generator"),
            ("crates/b/gen.sh", "chk", "crates/b", "scheduling-checker"),
        ] {
            for order in ORDERS {
                f.commit(&[three_readers_roots(
                    &order,
                    &form("crates/s/src/table.rs", &[script], &[]),
                )]);
                let r = refused(f.run());
                for reader in order.iter().filter(|x| **x != owner) {
                    says(
                        &r,
                        &format!(
                            "trust-shared-program: `{reader}` reads `crates/s/src/table.rs`, generated by `{script}`, \
                             which lies in `{pkg}`, a role package of the {role}"
                        ),
                    );
                }
            }
        }
    }

    #[test]
    fn route_a_program_target_generator_is_judged_against_every_reader() {
        // §2, §3. A program-target generator of a declared file three programs read, judged against each, in every
        // order of the three: one in a second role package of `gen`, refused for `chk` and `imp`; one of `chk`'s, for
        // `gen` and `imp` (review rounds 9, 12, 13).
        let f = three_readers("route-every-reader-target");
        for (target, name, owner, pkg, role) in [
            ("crates/x/src/main.rs", "x", "gen", "crates/x", "generator"),
            (
                "crates/y/src/main.rs",
                "y",
                "chk",
                "crates/y",
                "scheduling-checker",
            ),
        ] {
            for order in ORDERS {
                f.commit(&[three_readers_roots(
                    &order,
                    &form("crates/s/src/table.rs", &[target], &[]),
                )]);
                let r = refused(f.run());
                for reader in order.iter().filter(|x| **x != owner) {
                    says(
                        &r,
                        &format!(
                            "trust-shared-program: `{reader}` reads `crates/s/src/table.rs`, generated by `{target}`, \
                             the crate root of the bin `{name}` of `{pkg}`, whose build compiles `{pkg}`, a role package \
                             of the {role} — not built"
                        ),
                    );
                }
            }
        }
    }

    #[test]
    fn route_tests_refusals_hold_at_every_position_of_a_form_s_entries() {
        // §2, §3. Each refusal §6 names for an entry holds wherever the entry stands among plain ones — alone, first,
        // in the middle, last — in its clause: a declared generator or input, a marked one, a generator or input at or
        // under a gitlink, a file named in both clauses, another role's script and program target, a program target
        // whose build reads a declared file, a root's own among them, and a lone `.rs` file (review rounds 10 to 12).
        let declaring = form("scripts/made.sh", &["scripts/p1.sh"], &[])
            + &form("data/mid.csv", &["scripts/p1.sh"], &["data/p1.csv"])
            + &form("crates/c/src/gen_c.rs", &["scripts/p1.sh"], &[]);
        // `gen`'s own table: declared by a plain form, or, for the root case, by the form the sweep writes.
        let gen_a = form("crates/a/src/gen_a.rs", &["scripts/p1.sh"], &[]);
        let f = chk(
            "route-every-position",
            &[
                ("scripts/p1.sh", "echo p1\n".to_owned()),
                ("scripts/p2.sh", "echo p2\n".to_owned()),
                ("data/p1.csv", "1\n".to_owned()),
                ("data/p2.csv", "2\n".to_owned()),
                ("scripts/made.sh", "echo made\n".to_owned()),
                ("data/mid.csv", "3\n".to_owned()),
                (
                    "scripts/marked.sh",
                    "# @generated by a tool - do not edit\necho marked\n".to_owned(),
                ),
                (
                    "data/marked.h",
                    "// @generated by a tool - do not edit\n#define T 2\n".to_owned(),
                ),
                ("scripts/lib.sh", "echo shared\n".to_owned()),
                ("crates/c/Cargo.toml", manifest("c", "")),
                (
                    "crates/c/src/main.rs",
                    "mod gen_c;\nfn main() { let _ = gen_c::T; }\n".to_owned(),
                ),
                (
                    "crates/c/src/gen_c.rs",
                    "pub const T: u32 = 1;\n".to_owned(),
                ),
                ("third_party/x/gen.rs", "fn main() {}\n".to_owned()),
                ("crates/a/step.sh", "echo a step\n".to_owned()),
                ("crates/m/Cargo.toml", manifest("m", "")),
                (
                    "crates/m/src/main.rs",
                    "mod gen_m;\nfn main() { let _ = gen_m::T; }\n".to_owned(),
                ),
                (
                    "crates/m/src/gen_m.rs",
                    "// @generated by a tool - do not edit\npub const T: u32 = 1;\n".to_owned(),
                ),
                (
                    "crates/a/src/main.rs",
                    "mod gen_a;\nfn main() { let _ = gen_a::T; }\n".to_owned(),
                ),
                (
                    "crates/a/src/gen_a.rs",
                    "pub const T: u32 = 1;\n".to_owned(),
                ),
                roots(&(declaring.clone() + &gen_a)),
            ],
        );
        let head = String::from_utf8(
            std::process::Command::new("git")
                .args(["rev-parse", "HEAD"])
                .current_dir(&f.repo)
                .output()
                .unwrap()
                .stdout,
        )
        .unwrap();
        git(
            &f.repo,
            &[
                "update-index",
                "--add",
                "--cacheinfo",
                &format!("160000,{},vendor/sub", head.trim()),
            ],
        );
        git(&f.repo, &["commit", "-q", "-m", "a gitlink"]);
        let plain_g = ["scripts/p1.sh", "scripts/p2.sh"];
        let plain_i = ["data/p1.csv", "data/p2.csv"];
        // (the refused entry, its clause, the refusal it must meet)
        let cases: [(&str, &str, &str); 15] = [
            (
                "scripts/made.sh",
                "generator",
                "generator `scripts/made.sh` is declared by a `defgenerated` form — a chain",
            ),
            (
                "data/mid.csv",
                "input",
                "input `data/mid.csv` is declared by a `defgenerated` form — a chain",
            ),
            (
                "scripts/marked.sh",
                "generator",
                "generator `scripts/marked.sh` has a header that marks it generated",
            ),
            (
                "data/marked.h",
                "input",
                "input `data/marked.h` has a header that marks it generated",
            ),
            (
                "vendor/sub",
                "input",
                "input `vendor/sub` is no blob of the commit",
            ),
            (
                "vendor/sub/table.csv",
                "input",
                "input `vendor/sub/table.csv` is no blob of the commit",
            ),
            (
                "vendor/sub/tool",
                "generator",
                "generator `vendor/sub/tool` is no blob of the commit",
            ),
            (
                "scripts/lib.sh",
                "both",
                "names `scripts/lib.sh` as both a generator and an input",
            ),
            (
                "vendor/sub",
                "generator",
                "generator `vendor/sub` is no blob of the commit",
            ),
            (
                "crates/a/src/main.rs",
                "generator",
                "generated by `crates/a/src/main.rs`, the crate root of the bin `a` of `crates/a`",
            ),
            (
                "crates/c/src/main.rs",
                "generator",
                "which the build of the generator `crates/c/src/main.rs` reads, is a generated source",
            ),
            (
                "third_party/x/gen.rs",
                "generator",
                "generator `third_party/x/gen.rs` is a `.rs` file that is no",
            ),
            (
                "crates/m/src/main.rs",
                "generator",
                "`crates/m/src/gen_m.rs`, which the build of the generator `crates/m/src/main.rs` reads, is a generated source",
            ),
            (
                "crates/a/step.sh",
                "generator",
                "generated by `crates/a/step.sh`, which lies in `crates/a`, a role package of the generator",
            ),
            (
                "crates/a/src/main.rs",
                "generator",
                "`crates/a/src/gen_a.rs`, which the build of the generator `crates/a/src/main.rs` reads, is a generated source",
            ),
        ];
        for (entry, clause, refusal) in cases {
            for at in 0..=2 {
                for alone in [true, false] {
                    if alone && at > 0 {
                        continue;
                    }
                    let place = |plain: [&'static str; 2], here: bool| -> Vec<&str> {
                        if !here {
                            return plain.to_vec();
                        }
                        if alone {
                            return vec![entry];
                        }
                        let mut v = plain.to_vec();
                        v.insert(at, entry);
                        v
                    };
                    let gens = place(plain_g, clause == "generator" || clause == "both");
                    let ins = place(plain_i, clause == "input" || clause == "both");
                    // The root case is `gen`'s own: its crate root generating `gen`'s table, its build reading it.
                    let own = entry == "crates/a/src/main.rs" && refusal.contains("gen_a.rs");
                    let live = if own {
                        declaring.clone() + &form("crates/a/src/gen_a.rs", &gens, &ins)
                    } else {
                        declaring.clone() + &gen_a + &form("crates/b/src/gen_b.rs", &gens, &ins)
                    };
                    f.commit(&[roots(&live)]);
                    let r = refused(f.run());
                    assert!(
                        r.iter().any(|x| x.contains(refusal)),
                        "`{entry}` ({clause}) at {} — no refusal says `{refusal}`: {r:#?}",
                        if alone {
                            "alone".to_owned()
                        } else {
                            at.to_string()
                        }
                    );
                }
            }
        }
    }
}
