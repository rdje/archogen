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
//! run unable to judge, its refusals reported beside the failure. Step 1 is here; steps 2 to 5 run in
//! [`crate::trust::inventory_with`].
//!
//! ⛔ **Coded, never uncoded** (§2). Every departure of a `defgenerated` form from its shape, and every construction the
//! forms' text alone shows wrong, is a `trust-undeclared-input` refusal of the commit. Only what the eADL reader cannot
//! read, or another form of `trust/roots.eadl` departing from the parent's shape, leaves the gate unable to judge — and
//! then the `defgenerated` refusals the parent's reader met before it are reported beside it.

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::Form;

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
