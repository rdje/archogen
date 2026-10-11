//! Parent-rule controls for every clause position, blob/chain diagnostic and reached provenance entry.
//! M3.6.9.6.1; these tests belong to the parent population, independently of the route fixtures.

use std::collections::{BTreeMap, BTreeSet};

use archogen_catalog::tree::Tree;
use archogen_evidence::sha256::Digest;

use super::{blobs_and_chains, live, provenance, Generated, Provenance, Provenances};

fn words(entries: &[&str]) -> Vec<String> {
    entries.iter().map(|entry| (*entry).to_owned()).collect()
}

fn generated(path: &str, generators: &[&str], inputs: &[&str]) -> Generated {
    Generated {
        path: path.to_owned(),
        generators: words(generators),
        inputs: words(inputs),
    }
}

#[test]
fn every_generator_and_input_position_is_checked_for_clause_overlap() {
    let quoted = |entries: &[String]| {
        entries
            .iter()
            .map(|entry| format!("\"{entry}\""))
            .collect::<Vec<_>>()
            .join(" ")
    };
    for generator_position in 0..3 {
        for input_position in 0..3 {
            let generators = words(&["ga.sh", "gb.sh", "gc.sh"]);
            let mut inputs = words(&["ia.sh", "ib.sh", "ic.sh"]);
            let shared = &generators[generator_position];
            inputs[input_position] = shared.clone();
            let text = format!(
                "(defgenerated \"out.rs\" (generator {}) (inputs {}) (command \"c\") (reason \"r\"))",
                quoted(&generators),
                quoted(&inputs)
            );
            let reports = crate::trust::read_roots(&text)
                .expect("well-formed declaration")
                .generated_refused;
            assert_eq!(reports.len(), 1, "{text}: {reports:#?}");
            assert!(
                reports[0].starts_with("trust-undeclared-input: trust/roots.eadl: ")
                    && reports[0].ends_with(&format!(
                        "names `{shared}` as both a generator and an input"
                    )),
                "{text}: {reports:#?}"
            );
        }
    }
}

#[test]
fn every_clause_entry_has_its_own_blob_or_chain_refusal() {
    let definition = generated(
        "out.rs",
        &["ga.sh", "gb.sh", "gc.sh"],
        &["ia.sh", "ib.sh", "ic.sh"],
    );
    let plain = Tree::new(
        definition
            .generators
            .iter()
            .chain(&definition.inputs)
            .map(|entry| (entry.clone(), b"# ordinary file\n".to_vec())),
    );
    assert!(blobs_and_chains(
        std::slice::from_ref(&definition),
        &[&definition],
        &plain,
        &BTreeSet::new()
    )
    .is_empty());
    for (role, entries) in [
        ("generator", &definition.generators),
        ("input", &definition.inputs),
    ] {
        for entry in entries {
            for defect in ["missing", "link", "declared", "marked"] {
                let mut tree = plain.clone();
                let mut symlinks = BTreeSet::new();
                let mut declarations = vec![definition.clone()];
                let reason = match defect {
                    "missing" => {
                        tree.remove(entry);
                        "is no blob of the commit"
                    }
                    "link" => {
                        // The commit's regular-file tree excludes links. Keep the link diagnostic distinct
                        // from the no-blob refusal that would still reject it if the link branch were skipped.
                        tree.remove(entry);
                        symlinks.insert(entry.clone());
                        "is a symbolic link in the commit"
                    }
                    "declared" => {
                        declarations.push(generated(entry, &["upstream.sh"], &[]));
                        "is declared by a `defgenerated` form"
                    }
                    "marked" => {
                        tree.insert(entry, b"# @generated\n".to_vec());
                        "has a header that marks it generated"
                    }
                    _ => unreachable!("fixed cases above"),
                };
                let reports = blobs_and_chains(&declarations, &[&definition], &tree, &symlinks);
                assert_eq!(reports.len(), 1, "{role} {entry} {defect}: {reports:#?}");
                assert!(
                    reports[0].starts_with("trust-undeclared-input: trust/roots.eadl: ")
                        && reports[0].contains(&format!("'s {role} `{entry}`"))
                        && reports[0].contains(reason),
                    "{role} {entry} {defect}: {reports:#?}"
                );
            }
        }
    }
}

#[test]
fn provenance_keeps_every_entry_role_hash_and_reached_declaration() {
    let definitions = [
        generated(
            "first.rs",
            &["ga.sh", "shared.sh", "gc.sh"],
            &["ia.sh", "ib.sh", "ic.sh"],
        ),
        generated(
            "second.rs",
            &["ha.sh", "hb.sh", "hc.sh"],
            &["ja.sh", "shared.sh", "jc.sh"],
        ),
        generated("other.rs", &["other.sh"], &["other-input.sh"]),
        generated("dormant.rs", &["dormant.sh"], &[]),
    ];
    let contents = |path: &str| format!("contents of {path}\n").into_bytes();
    let tree = Tree::new(definitions.iter().flat_map(|definition| {
        std::iter::once(&definition.path)
            .chain(&definition.generators)
            .chain(&definition.inputs)
            .map(|path| (path.clone(), contents(path)))
    }));
    let reached: BTreeSet<String> = words(&["first.rs", "second.rs"]).into_iter().collect();
    let programs = BTreeMap::from([
        ("main".to_owned(), reached.clone()),
        (
            "other".to_owned(),
            words(&["other.rs"]).into_iter().collect(),
        ),
    ]);
    let live_forms = live(&definitions, &programs);
    assert_eq!(
        live_forms,
        [&definitions[0], &definitions[1], &definitions[2]]
    );
    // The expected relation is literal, independent of the implementation's nested form/role/file traversal.
    let expected: Provenances = [
        ("ga.sh", &["generator"][..], &["first.rs"][..]),
        (
            "shared.sh",
            &["generator", "input"][..],
            &["first.rs", "second.rs"][..],
        ),
        ("gc.sh", &["generator"][..], &["first.rs"][..]),
        ("ia.sh", &["input"][..], &["first.rs"][..]),
        ("ib.sh", &["input"][..], &["first.rs"][..]),
        ("ic.sh", &["input"][..], &["first.rs"][..]),
        ("ha.sh", &["generator"][..], &["second.rs"][..]),
        ("hb.sh", &["generator"][..], &["second.rs"][..]),
        ("hc.sh", &["generator"][..], &["second.rs"][..]),
        ("ja.sh", &["input"][..], &["second.rs"][..]),
        ("jc.sh", &["input"][..], &["second.rs"][..]),
    ]
    .into_iter()
    .map(|(path, roles, declared)| {
        (
            path.to_owned(),
            Provenance {
                sha256: Digest::of(&contents(path)).hex(),
                roles: roles.iter().copied().collect(),
                declared: declared
                    .iter()
                    .map(|path| (path.to_string(), Digest::of(&contents(path)).hex()))
                    .collect(),
            },
        )
    })
    .collect();
    assert_eq!(provenance(&live_forms, &reached, &tree), expected);
}
