//! §3's hashes against the worked example (`M2.7.3.2`).
//!
//! The example is read from `docs/specs/catalog/decision_catalog-records-example.md` itself: its two records,
//! its three files and every digest it states. So the test and the design cannot drift apart unseen.

use archogen_catalog::hash::{encode, forms_hash, review_ledger_hash, Catalog, IDENTIFIER};
use archogen_catalog::record::{read_record, FacetKind};
use archogen_catalog::tree::Tree;
use archogen_evidence::sha256::Digest;

const EXAMPLE: &str =
    include_str!("../../../docs/specs/catalog/decision_catalog-records-example.md");

/// The first ```` ```text ```` block that starts with `starts`.
fn block(starts: &str) -> &'static str {
    let at = EXAMPLE
        .find(&format!("```text\n{starts}"))
        .unwrap_or_else(|| panic!("no block starting `{starts}`"));
    let body = &EXAMPLE[at + "```text\n".len()..];
    &body[..body.find("```").unwrap()]
}

/// Every `sha256:…` on the line that begins with `label`.
fn digests(label: &str) -> Vec<String> {
    let line = EXAMPLE
        .lines()
        .find(|l| l.starts_with(label))
        .unwrap_or_else(|| panic!("no line `{label}`"));
    line.split('`')
        .filter(|s| s.starts_with("sha256:"))
        .map(str::to_owned)
        .collect()
}

/// The two records as the example gives them, over its three files.
fn example() -> Catalog {
    let records = block("(catalog-record example.base");
    let split = records.find("\n(catalog-record example.timed").unwrap();
    let (base, timed) = (&records[..split + 1], &records[split + 1..]);
    let tree = Tree::new([
        ("docs/example/model.txt".to_owned(), b"model\n".to_vec()),
        (
            "targets/example-target.env".to_owned(),
            b"TARGET_ID=example-target\n".to_vec(),
        ),
        (
            "targets/example-target.eadl".to_owned(),
            b"(platform)\n".to_vec(),
        ),
    ]);
    let read = |path: &str, text: &str| {
        read_record(path, text.as_bytes()).unwrap_or_else(|e| panic!("{e}"))
    };
    Catalog::new(
        tree,
        [
            read("catalog/experimental/example.base.catalog", base),
            read("catalog/experimental/example.timed.catalog", timed),
        ],
    )
}

#[test]
fn the_three_file_hashes() {
    for (file, bytes) in [
        ("`model.txt`", "model\n"),
        ("`.env`", "TARGET_ID=example-target\n"),
        ("`.eadl`", "(platform)\n"),
    ] {
        let expected = digests(&format!("- {file}:"));
        assert_eq!(
            Digest::of(bytes.as_bytes()).to_string(),
            expected[0],
            "{file}"
        );
    }
}

#[test]
fn every_facet_and_record_hash_of_the_worked_example() {
    let catalog = example();
    let hashes = catalog.hashes().unwrap_or_else(|e| panic!("{e}"));
    let mut compared = 0;
    for facet in FacetKind::ALL {
        for (kind, label) in [
            ("own", format!("| own {} |", facet.as_str())),
            ("bound", format!("| bound {} |", facet.as_str())),
        ] {
            let expected = digests(&label);
            assert_eq!(expected.len(), 2, "{label}");
            for (id, want) in ["example.base", "example.timed"].iter().zip(&expected) {
                let h = hashes.facet(id, facet).unwrap();
                let got = if kind == "own" { h.own } else { h.bound };
                assert_eq!(&got.to_string(), want, "{kind} {} of {id}", facet.as_str());
                compared += 1;
            }
        }
    }
    let expected = digests("| record |");
    for (id, want) in ["example.base", "example.timed"].iter().zip(&expected) {
        assert_eq!(
            &hashes.records[*id].to_string(),
            want,
            "record hash of {id}"
        );
        compared += 1;
    }
    assert_eq!(compared, 18);
}

#[test]
fn the_printed_bound_timing_input_of_example_timed() {
    let hashes = example().hashes().unwrap();
    let h = hashes
        .facet("example.timed", FacetKind::TimingModel)
        .unwrap();
    let mut lines = vec![
        IDENTIFIER.to_owned(),
        "bound timing-model example.timed".to_owned(),
        format!("own {}", h.own),
    ];
    lines.extend(h.derived.iter().cloned());
    let printed: Vec<&str> = block("archogen-catalog/1\nbound timing-model")
        .lines()
        .collect();
    assert_eq!(lines, printed);
}

#[test]
fn the_review_ledger_hash_and_its_encoding() {
    let catalog = example();
    let review = &catalog.records["example.base"].reviews[0];
    let printed = block("(review (facet behavior-model)").trim_end();
    assert_eq!(encode(&review.form), printed);
    let lock = block("# archogen-catalog/1");
    let line = lock
        .lines()
        .find(|l| l.starts_with("example.base review "))
        .unwrap();
    let ledger = line.split(' ').nth(2).unwrap();
    assert_eq!(
        review_ledger_hash("example.base", review).to_string(),
        ledger
    );
    // The line's last field is the bound hash the review names: the behavioral model's.
    let hashes = catalog.hashes().unwrap();
    assert!(line.ends_with(
        &hashes
            .facet("example.base", FacetKind::BehaviorModel)
            .unwrap()
            .bound
            .to_string()
    ));
}

#[test]
fn the_lock_lines_carry_the_own_hashes() {
    let hashes = example().hashes().unwrap();
    let lock = block("# archogen-catalog/1");
    for facet in FacetKind::ALL {
        let line = lock
            .lines()
            .find(|l| l.starts_with(&format!("example.base {} ", facet.as_str())))
            .unwrap();
        assert!(
            line.ends_with(&hashes.facet("example.base", facet).unwrap().own.to_string()),
            "{line}"
        );
    }
}

#[test]
fn the_forms_hash_of_example_base_timing_model() {
    let catalog = example();
    let at = EXAMPLE.find("has a forms hash (§5) of\n`").unwrap();
    let rest = &EXAMPLE[at + "has a forms hash (§5) of\n`".len()..];
    let want = &rest[..rest.find('`').unwrap()];
    assert_eq!(
        forms_hash(
            IDENTIFIER,
            &catalog.records["example.base"],
            FacetKind::TimingModel
        )
        .to_string(),
        want
    );
}

#[test]
fn the_cost_is_encoded_on_one_line_with_hexadecimal_in_decimal() {
    let catalog = example();
    let timing = &catalog.records["example.timed"].form.items()[14];
    assert_eq!(timing.head(), Some("timing-model"));
    let costs = timing
        .items()
        .iter()
        .find(|f| f.head() == Some("costs"))
        .unwrap();
    assert_eq!(encode(&costs.items()[1]), block("(cost switch").trim_end());
}
