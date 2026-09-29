//! **F27** — worked functionality/implementation boundary cases.
//!
//! `ROADMAP.md` §13.1: *"Accepted functional guarantees and rejected implementation fields,
//! with reasons; ambiguous cases reviewed."* First gate M0/M1.
//!
//! The fixture has three parts, and the third is the one that makes the first two mean
//! something:
//!
//! 1. **Agreement** — the classifier's verdict matches every corpus case's recorded verdict,
//!    and for a rejection, the test it names matches the case's recorded `failing-test`.
//! 2. **Coverage** — every registered forbidden construct is exercised by at least one case,
//!    and every case's refusal reason is one the registry can state.
//! 3. **Mutation** — seeding an implementation field into an accepted case flips it to
//!    rejected with the right test, and removing the offending construct from a rejected case
//!    flips it to accepted. A classifier that accepted everything would pass part 1 for the ten
//!    accept cases and part 3 for none.
//!
//! ⚠️ What F27 does **not** establish, stated here rather than left to be assumed: that an
//! accepted case is *correct*. §4.3 requires human review for intent, because a field can hide
//! an algorithm behind an innocent name. Part 1 passing means the corpus and the registry agree,
//! not that either is complete.

use std::path::{Path, PathBuf};

use eadl_front::{read, Form, SourceMap};
use eadl_model::boundary::{classify, Classification, FORBIDDEN};

struct Case {
    /// Repo-relative path, for messages.
    name: String,
    /// The declaration.
    form: Form,
    /// `accept` or `reject`, from the case's own metadata.
    verdict: String,
    /// The recorded failing test, for a rejection.
    failing_test: Option<String>,
    /// Whether the case is recorded as ambiguous.
    ambiguous: bool,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn corpus() -> Vec<Case> {
    let root = repo_root();
    let mut cases = Vec::new();
    for verdict_dir in ["accept", "reject"] {
        let dir = root.join("docs/semantics/boundary").join(verdict_dir);
        let entries = std::fs::read_dir(&dir)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", dir.display()));
        for entry in entries {
            let path = entry.expect("readable entry").path();
            if path.extension().is_none_or(|e| e != "eadl") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable case");
            let name = format!(
                "docs/semantics/boundary/{verdict_dir}/{}",
                path.file_name().expect("a file name").to_string_lossy()
            );

            let mut sources = SourceMap::new();
            let id = sources.add(&name, text).expect("small");
            let (document, diagnostics) = read(&sources, id);
            assert!(
                !diagnostics.has_errors(),
                "{name} did not read:\n{}",
                diagnostics.render(&sources)
            );

            let headers = document.comment_headers();
            let header = |key: &str| {
                headers
                    .iter()
                    .find(|(k, _)| k == key)
                    .map(|(_, v)| v.clone())
            };

            cases.push(Case {
                verdict: header("verdict").unwrap_or_else(|| panic!("{name}: no verdict")),
                failing_test: header("failing-test"),
                ambiguous: header("ambiguous").as_deref() == Some("yes"),
                // ⭐ The first **declaration**, not the first form. §8 of
                // `docs/semantics/reference.md` makes `(eadl-version eadl/1)` a top-level form that is
                // not a declaration, and every case in this corpus states its version — so `forms`
                // `.next()` here handed all five legs below the identifier instead of the case, and
                // each of them failed in its own way: the classifier "accepted" every rejected case,
                // every registered construct looked unexercised, and the seeding control spliced
                // `(implementation …)` into the version form. One accessor, one place.
                form: document
                    .declarations()
                    .next()
                    .unwrap_or_else(|| panic!("{name}: no declaration"))
                    .clone(),
                name,
            });
        }
    }
    cases.sort_by(|a, b| a.name.cmp(&b.name));
    cases
}

#[test]
fn f27_the_classifier_agrees_with_every_recorded_verdict() {
    let cases = corpus();
    assert_eq!(cases.len(), 23, "the corpus changed size");

    for case in &cases {
        match (case.verdict.as_str(), classify(&case.form)) {
            ("accept", Classification::Accepted) => {}
            ("accept", Classification::Rejected { head, .. }) => panic!(
                "{}: recorded as accepted, but the classifier refused it for `{head}`",
                case.name
            ),
            ("reject", Classification::Accepted) => panic!(
                "{}: recorded as rejected, but the classifier accepted it — the registry is \
                 missing the construct this case is about",
                case.name
            ),
            ("reject", Classification::Rejected { failing_test, .. }) => {
                let recorded = case
                    .failing_test
                    .as_deref()
                    .unwrap_or_else(|| panic!("{}: rejected with no failing-test", case.name));
                assert_eq!(
                    failing_test.slug(),
                    recorded,
                    "{}: the classifier blames `{}`, the case records `{recorded}`",
                    case.name,
                    failing_test.slug()
                );
            }
            (other, _) => panic!("{}: unknown verdict `{other}`", case.name),
        }
    }
}

#[test]
fn f27_the_ambiguous_cases_are_present_in_both_directions() {
    // §4.3: "Ambiguous new fields require a worked classification case before adoption." A
    // corpus of only obvious cases confirms the boundary rather than testing it, and the
    // ambiguous ones must cut both ways — some look like implementation and are accepted, some
    // look like requirements and are rejected.
    let cases = corpus();
    let ambiguous: Vec<&Case> = cases.iter().filter(|c| c.ambiguous).collect();
    assert!(
        ambiguous.len() >= 3,
        "only {} ambiguous cases; §4.3 wants the hard ones covered",
        ambiguous.len()
    );
    assert!(
        ambiguous.iter().any(|c| c.verdict == "accept"),
        "no ambiguous case is accepted — the corpus only tests one direction"
    );
    assert!(
        ambiguous.iter().any(|c| c.verdict == "reject"),
        "no ambiguous case is rejected — the corpus only tests one direction"
    );
}

#[test]
fn f27_every_registered_construct_is_exercised_by_a_case() {
    // A registry entry nothing tests is an assertion, not a rule. If a construct is worth
    // refusing, the corpus owes it a worked case.
    let cases = corpus();
    let mut unexercised: Vec<&str> = Vec::new();
    for construct in FORBIDDEN {
        let exercised = cases.iter().any(|case| {
            matches!(
                classify(&case.form),
                Classification::Rejected { head, .. } if head == construct.head
            )
        });
        if !exercised {
            unexercised.push(construct.head);
        }
    }
    assert!(
        unexercised.is_empty(),
        "registered constructs with no worked case: {unexercised:?}"
    );
}

#[test]
fn f27_seeding_an_implementation_field_flips_an_accepted_case() {
    // ⭐ The control that makes the other tests mean something. A classifier that accepted
    // everything would pass the accept half of the agreement test; it cannot pass this.
    let cases = corpus();
    let accepted: Vec<&Case> = cases.iter().filter(|c| c.verdict == "accept").collect();
    assert_eq!(accepted.len(), 10);

    for case in accepted {
        assert!(classify(&case.form).is_accepted(), "{}", case.name);

        for construct in FORBIDDEN {
            // Splice the forbidden construct into the declaration's last position, which is
            // where an author adding "just one implementation detail" would put it.
            let seeded = format!(
                "{} ({} seeded-content))",
                case.form
                    .to_canonical()
                    .strip_suffix(')')
                    .expect("a declaration is a list"),
                construct.head
            );
            let mut sources = SourceMap::new();
            let id = sources.add("seeded", &seeded).expect("small");
            let (document, diagnostics) = read(&sources, id);
            assert!(
                !diagnostics.has_errors(),
                "the seeded fixture is malformed: {seeded}"
            );
            let form = document.forms.into_iter().next().expect("one form");

            match classify(&form) {
                Classification::Rejected {
                    head, failing_test, ..
                } => {
                    assert_eq!(
                        head, construct.head,
                        "{}: wrong construct blamed",
                        case.name
                    );
                    assert_eq!(
                        failing_test,
                        construct.failing_test,
                        "{}: `{}` should fail `{}`",
                        case.name,
                        construct.head,
                        construct.failing_test.slug()
                    );
                }
                Classification::Accepted => panic!(
                    "{}: seeding `({} …)` did not flip the verdict — the gate is not checking",
                    case.name, construct.head
                ),
            }
        }
    }
}

#[test]
fn f27_removing_the_offending_construct_flips_a_rejected_case() {
    // The other direction: the classifier must refuse *the construct*, not the case. A
    // classifier hard-wired to reject anything in the reject directory would pass the
    // agreement test and fail this one.
    let cases = corpus();
    for case in cases.iter().filter(|c| c.verdict == "reject") {
        let Classification::Rejected { head, .. } = classify(&case.form) else {
            panic!("{}: expected a rejection", case.name);
        };
        let stripped = strip_construct(&case.form, head);
        assert!(
            classify(&stripped).is_accepted(),
            "{}: removing `({head} …)` left it rejected:\n{}",
            case.name,
            stripped.to_canonical()
        );
    }
}

/// Rebuild a form with every list headed by `head` removed.
fn strip_construct(form: &Form, head: &str) -> Form {
    match form {
        Form::List { items, span } => Form::List {
            items: items
                .iter()
                .filter(|item| item.head() != Some(head))
                .map(|item| strip_construct(item, head))
                .collect(),
            span: *span,
        },
        other => other.clone(),
    }
}

#[test]
fn f27_a_refusal_is_usable_guidance_not_just_a_no() {
    // §5.5 requires a concrete repair direction on every diagnostic. For a boundary refusal
    // that means: which test failed, what the test asks, and where the content belongs.
    let cases = corpus();
    for case in cases.iter().filter(|c| c.verdict == "reject") {
        let diagnostic = eadl_model::boundary::check(&case.form).expect_err("must refuse");
        assert_eq!(diagnostic.code, "boundary-implementation-in-description");
        assert!(
            !diagnostic.repair.is_empty(),
            "{}: refused with no repair direction",
            case.name
        );
        assert!(
            diagnostic.repair.contains('?'),
            "{}: the refusal does not carry the test's question: {}",
            case.name,
            diagnostic.repair
        );
        assert!(
            diagnostic.repair.contains("belongs to"),
            "{}: the refusal does not say where the content belongs: {}",
            case.name,
            diagnostic.repair
        );
    }
}

// ── the whole pipeline, not only the classifier and the schema ─────────────────────────────────────

use eadl_front::Verdict;
use eadl_model::check::{check, default_profile, shipped_registry};
use eadl_model::kind::Registry;

/// The registry a real invocation builds, through the production loader.
fn registry() -> Registry {
    let mut sources = SourceMap::new();
    let files: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|relative| {
        let path = repo_root().join(relative);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        ((*relative).to_string(), text)
    })
    .collect();
    shipped_registry(&mut sources, &files).unwrap_or_else(|errors| {
        panic!(
            "the shipped kind modules are malformed:\n{}",
            errors
                .iter()
                .map(|d| d.message.clone())
                .collect::<Vec<_>>()
                .join("; ")
        )
    })
}

/// ⛔ **Leaf `M1.28`, and the leg this file did not have.** Every other leg here asks the *classifier*
/// or the *schema*, and neither reads a fact's value: `(offers …)` is `(holds forms)`, which
/// `docs/semantics/kinds/core.eadl` is explicit is not interpreted at that layer. The pipeline reads
/// further, so a new rule about quantities can refuse a case this corpus records as `accept` while every
/// leg below stays green. That is measured and not hypothetical — reading every offered value as a
/// quantity turned `counter-width-and-rate.eadl`'s `(counter-modulus 4294967296)` into
/// `quantity-missing-unit`, and the census that found it was a hand-run loop over all 76 tracked
/// descriptions rather than anything in this file.
#[test]
fn f27_every_accepted_case_is_accepted_by_the_whole_pipeline() {
    let registry = registry();
    let profile = default_profile();
    let mut accepted = 0;
    for case in corpus() {
        if case.verdict != "accept" {
            continue;
        }
        accepted += 1;
        let path = repo_root().join(&case.name);
        let text = std::fs::read_to_string(&path)
            .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
        let mut sources = SourceMap::new();
        let id = sources.add(case.name.clone(), text).expect("small");
        let outcome = check(&sources, id, &registry, profile);
        assert!(
            outcome.diagnostics.is_empty(),
            "{} is recorded `accept` but the pipeline refused it:\n{}",
            case.name,
            outcome.render(&sources)
        );
        assert_eq!(
            outcome.verdict,
            Verdict::Ok,
            "{} is recorded `accept`",
            case.name
        );
    }
    assert_eq!(accepted, 10, "the accept corpus changed size");
}
