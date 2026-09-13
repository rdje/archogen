//! The semantic corpus — `ROADMAP.md` §12 M1's twenty worked cases, driven end to end.
//!
//! > Adopt at least **twenty semantic examples**, including positive matches, relevant missing
//! > facts, conflicts, and unsupported cases.
//!
//! Each case declares the §5.5 verdict it expects, and the driver runs the whole frontend
//! pipeline against it. A case is evidence only if the expectation is written *in the case* —
//! a driver that computed the expectation would agree with itself forever.
//!
//! The suite also enforces the §5.5 diagnostic contract on every diagnostic the corpus produces:
//! a span, and a concrete repair direction.

use std::path::{Path, PathBuf};

use eadl_front::{SourceMap, Verdict};
use eadl_model::check::{check, default_profile, shipped_registry};
use eadl_model::kind::Registry;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn kind_files() -> Vec<(String, String)> {
    [
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
    .collect()
}

fn registry(sources: &mut SourceMap) -> Registry {
    shipped_registry(sources, &kind_files()).unwrap_or_else(|errors| {
        panic!(
            "the shipped kind modules are malformed: {}",
            errors
                .iter()
                .map(|d| d.message.clone())
                .collect::<Vec<_>>()
                .join("; ")
        )
    })
}

struct Case {
    name: String,
    expect: Verdict,
    why: String,
    text: String,
}

fn corpus() -> Vec<Case> {
    let dir = repo_root().join("docs/semantics/cases");
    let mut cases = Vec::new();
    for entry in std::fs::read_dir(&dir).expect("the semantic corpus exists") {
        let path = entry.expect("entry").path();
        if path.extension().is_none_or(|e| e != "eadl") {
            continue;
        }
        let text = std::fs::read_to_string(&path).expect("readable case");
        let name = path
            .file_stem()
            .expect("a stem")
            .to_string_lossy()
            .to_string();

        let header = |key: &str| -> Option<String> {
            let mut value: Option<String> = None;
            for line in text.lines() {
                let Some(body) = line.strip_prefix("; ") else {
                    if line.starts_with(";   ") {
                        if let Some(acc) = value.as_mut() {
                            acc.push(' ');
                            acc.push_str(line.trim_start_matches(';').trim());
                        }
                    }
                    continue;
                };
                if let Some((k, v)) = body.split_once(": ") {
                    if k == key {
                        value = Some(v.trim().to_string());
                    } else if value.is_some() {
                        break;
                    }
                }
            }
            value
        };

        let expect = header("expect").unwrap_or_else(|| panic!("{name}: no `expect` header"));
        cases.push(Case {
            expect: Verdict::parse(&expect)
                .unwrap_or_else(|| panic!("{name}: `{expect}` is not a §5.5 verdict")),
            why: header("why").unwrap_or_else(|| panic!("{name}: no `why` header")),
            name,
            text,
        });
    }
    cases.sort_by(|a, b| a.name.cmp(&b.name));
    cases
}

#[test]
fn the_corpus_meets_the_roadmap_minimum_and_covers_every_category() {
    // §12 M1: "at least twenty semantic examples, including positive matches, relevant missing
    // facts, conflicts, and unsupported cases." A corpus of twenty rejections would meet the
    // count and cover nothing.
    let cases = corpus();
    assert!(cases.len() >= 20, "only {} cases", cases.len());

    let count = |verdict: Verdict| cases.iter().filter(|c| c.expect == verdict).count();
    assert!(count(Verdict::Ok) >= 3, "too few positive matches");
    assert!(count(Verdict::MissingFact) >= 1, "no missing-fact case");
    assert!(
        count(Verdict::InvalidDescription) >= 3,
        "too few conflict cases"
    );
    assert!(
        count(Verdict::UnsupportedProfile) >= 3,
        "too few unsupported cases"
    );
    assert!(
        count(Verdict::InfeasibleConfiguration) >= 1,
        "no infeasible case"
    );

    for case in &cases {
        assert!(
            case.why.len() > 30,
            "{}: the `why` header does not explain the case: {}",
            case.name,
            case.why
        );
    }
}

#[test]
fn every_case_produces_the_verdict_it_declares() {
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    for case in corpus() {
        let id = sources
            .add(format!("cases/{}.eadl", case.name), case.text.clone())
            .expect("small");
        let outcome = check(&sources, id, &registry, profile);
        assert_eq!(
            outcome.verdict,
            case.expect,
            "{}: expected `{}`, got `{}`\n{}\n--- diagnostics ---\n{}",
            case.name,
            case.expect.slug(),
            outcome.verdict.slug(),
            case.why,
            outcome.render(&sources)
        );
    }
}

#[test]
fn an_accepted_case_produces_no_diagnostics_at_all() {
    // "Accepted with three warnings" is a shape this pipeline does not have: a case that is ok
    // is silent, so an author never has to judge which messages mattered.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    for case in corpus().into_iter().filter(|c| c.expect == Verdict::Ok) {
        let id = sources
            .add(format!("cases/{}.eadl", case.name), case.text)
            .expect("small");
        let outcome = check(&sources, id, &registry, profile);
        assert!(
            outcome.diagnostics.is_empty(),
            "{} is accepted but still said something:\n{}",
            case.name,
            outcome.render(&sources)
        );
    }
}

#[test]
fn a_refused_case_always_says_why() {
    // A verdict with no diagnostic is a refusal with no explanation, which is worse than no
    // refusal: the author has nothing to act on.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    for case in corpus().into_iter().filter(|c| c.expect != Verdict::Ok) {
        let id = sources
            .add(format!("cases/{}.eadl", case.name), case.text)
            .expect("small");
        let outcome = check(&sources, id, &registry, profile);
        assert!(
            !outcome.diagnostics.is_empty(),
            "{} was refused with no diagnostic",
            case.name
        );
    }
}

#[test]
fn every_diagnostic_the_corpus_produces_meets_the_5_5_contract() {
    // ⭐ §5.5: "Each diagnostic carries source spans … and a concrete repair direction."
    // Checked over every diagnostic the whole corpus produces, rather than on the handful a
    // unit test happens to construct.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();
    let mut checked = 0;

    for case in corpus() {
        let id = sources
            .add(format!("cases/{}.eadl", case.name), case.text)
            .expect("small");
        let outcome = check(&sources, id, &registry, profile);
        for diagnostic in &outcome.diagnostics {
            assert!(
                !diagnostic.code.is_empty(),
                "{}: a diagnostic with no code",
                case.name
            );
            assert!(
                !diagnostic.message.is_empty(),
                "{}: `{}` has no message",
                case.name,
                diagnostic.code
            );
            assert!(
                diagnostic.repair.len() > 15,
                "{}: `{}` has no usable repair direction: {:?}",
                case.name,
                diagnostic.code,
                diagnostic.repair
            );
            // The span must point into a real source, and the rendered form must locate it.
            let rendered = diagnostic.render(&sources);
            assert!(
                rendered.contains("-->") && !rendered.contains("<unknown source>"),
                "{}: `{}` does not locate itself:\n{rendered}",
                case.name,
                diagnostic.code
            );
            assert!(
                rendered.contains("= hint:"),
                "{}: `{}` renders without its repair direction",
                case.name,
                diagnostic.code
            );
            checked += 1;
        }
    }
    assert!(checked >= 20, "only {checked} diagnostics exercised");
}

#[test]
fn an_unsupported_request_names_the_capability_and_the_obligation() {
    // §3.1: refused "rather than silently reducing the requested guarantee". A refusal that
    // does not say what admitting it would cost reads as a wall rather than as work.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    let case = corpus()
        .into_iter()
        .find(|c| c.name == "unsupported-general-ipc")
        .expect("the uc4 case");
    let id = sources.add("uc4", case.text).expect("small");
    let outcome = check(&sources, id, &registry, profile);
    assert_eq!(outcome.verdict, Verdict::UnsupportedProfile);
    let rendered = outcome.render(&sources);
    assert!(
        rendered.contains("`general-ipc` is not admitted"),
        "{rendered}"
    );
    assert!(rendered.contains("rt-static-up-v1"), "{rendered}");
    assert!(
        rendered.contains("admitting it would add"),
        "the refusal must name the obligation:\n{rendered}"
    );
    assert!(rendered.contains("never silently reduced"), "{rendered}");
}

#[test]
fn the_verdict_is_what_to_fix_first_not_what_was_found_first() {
    // ⭐ A description with an out-of-profile request AND an undescribed fact reports
    // `unsupported-profile`: describing the fact would be wasted work on a system the profile
    // refuses anyway. Both diagnostics are still present — precedence chooses the headline, not
    // what the author gets to see.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    let case = corpus()
        .into_iter()
        .find(|c| c.name == "unsupported-precedence-over-missing-fact")
        .expect("the precedence case");
    let id = sources.add("precedence", case.text).expect("small");
    let outcome = check(&sources, id, &registry, profile);
    assert_eq!(outcome.verdict, Verdict::UnsupportedProfile);

    let codes: Vec<&str> = outcome.diagnostics.iter().map(|d| d.code).collect();
    assert!(codes.contains(&"unsupported-profile"), "{codes:?}");
    assert!(
        codes.contains(&"missing-fact"),
        "precedence must not hide the other problem: {codes:?}"
    );
}

#[test]
fn the_examples_directory_agrees_with_the_pipeline() {
    // The three shipped use cases run through the same pipeline as the corpus. uc1 and uc2 are
    // accepted; uc3 is `infeasible-configuration` today, because its platform declares
    // `absolute-deadline` absent and its service requires it — the gap M3.2's indirect
    // realization must close, without the description changing.
    let mut sources = SourceMap::new();
    let registry = registry(&mut sources);
    let profile = default_profile();

    for (relative, expected) in [
        ("examples/periodic-three/system.eadl", Verdict::Ok),
        ("examples/high-interference/system.eadl", Verdict::Ok),
        (
            "examples/alternative-timer/system.eadl",
            Verdict::InfeasibleConfiguration,
        ),
        // ⭐ uc3 and uc4 are both refused, for reasons that must not be confused. uc3's refusal
        // is TEMPORARY — the same description must build once M3.2 supplies indirect
        // realization. uc4's is PERMANENT for this profile: admitting it requires a new named
        // profile with its own analysis obligations, not a wider `rt-static-up-v1`.
        (
            "examples/bounded-queue/system.eadl",
            Verdict::UnsupportedProfile,
        ),
    ] {
        let text = std::fs::read_to_string(repo_root().join(relative)).expect("readable");
        let id = sources.add(relative, text).expect("small");
        let outcome = check(&sources, id, &registry, profile);
        assert_eq!(
            outcome.verdict,
            expected,
            "{relative}: expected `{}`, got `{}`\n{}",
            expected.slug(),
            outcome.verdict.slug(),
            outcome.render(&sources)
        );
    }
}
