//! §5.3's second half reaches the author: what an accepted description's closure holds, what pulled each fact
//! in, and what lies outside it (leaf `M1.30`).
//!
//! ⛔ **Why a leg and not only a feature.** Until `M1.30` the presence pass computed the closure boundary and the
//! pipeline dropped it: `outside_closure` had no production consumer, so "unknown facts outside the closure
//! remain visible in metadata" was computed, asserted by a unit test, and shown to nobody. A computed-and-dropped
//! field returns silently unless something reads the *output*. This leg reads it for every description the
//! repository records as accepted.
//!
//! The channel is the `archogen check` report, after the verdict line — metadata, never a diagnostic, so the
//! reference's §4 rule 1 (no warning, no note) holds (`docs/semantics/model.md` §2 rule 4).

use std::path::{Path, PathBuf};

use archogen_cli::run;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Every description `verdicts.txt` records as accepted — derived from the frozen verdicts, not listed.
fn accepted() -> Vec<String> {
    include_str!("verdicts.txt")
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            (parts.next()? == "0").then(|| parts.nth(1).map(str::to_string))?
        })
        .collect()
}

fn check(relative: &str) -> String {
    let root = repo_root();
    std::env::set_current_dir(&root).expect("the repository root");
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let _ = run(
        ["check".to_string(), relative.to_string()],
        &mut out,
        &mut err,
    );
    String::from_utf8_lossy(&out).to_string()
}

/// Every way an accepted description's report fails to carry its closure boundary.
fn report_violations(name: &str, report: &str) -> Vec<String> {
    let mut out = Vec::new();
    let inside: Vec<&str> = report
        .lines()
        .filter(|l| l.starts_with("  closure: "))
        .collect();
    let outside: Vec<&str> = report
        .lines()
        .filter(|l| l.starts_with("  outside the closure"))
        .collect();
    if inside.len() != 1 {
        out.push(format!(
            "{name}: {} `closure:` line(s), not 1",
            inside.len()
        ));
    }
    if outside.len() != 1 {
        out.push(format!(
            "{name}: {} `outside the closure` line(s), not 1",
            outside.len()
        ));
    }
    if let Some(line) = inside.first() {
        let facts: Vec<(&str, Option<&str>)> = line["  closure: ".len()..]
            .split(", ")
            .filter_map(|entry| {
                let (fact, why) = entry.split_once(" (")?;
                let why = why.trim_end_matches(')');
                Some((fact, why.strip_prefix("needed by ")))
            })
            .collect();
        // What pulled a fact in is itself inside: a closure is closed under the edges it followed.
        for (fact, by) in &facts {
            if let Some(owner) = by {
                if !facts.iter().any(|(other, _)| other == owner) {
                    out.push(format!(
                        "{name}: `{fact}` is said to be needed by `{owner}`, which the closure does not hold"
                    ));
                }
            }
        }
    }
    out
}

#[test]
fn every_accepted_description_reports_its_closure_boundary() {
    let population = accepted();
    assert!(
        population.len() >= 20,
        "found {} accepted descriptions — the reader is broken",
        population.len()
    );
    let mut wrong = Vec::new();
    for relative in &population {
        wrong.extend(report_violations(relative, &check(relative)));
    }
    assert!(
        wrong.is_empty(),
        "{} way(s) the report drops §5.3's metadata:\n\n{}",
        wrong.len(),
        wrong.join("\n")
    );
}

#[test]
fn an_irrelevant_unknown_fact_is_visible_outside_the_closure_and_fails_nothing() {
    // F05, end to end: `descriptor-format` is needed only by an engine nothing requests, and nothing describes
    // it. It fails nothing — the description is accepted — and the author can see it.
    let report = check("docs/semantics/cases/positive-unrelated-unknown.eadl");
    assert!(report.contains("accepted against profile"), "{report}");
    let outside = report
        .lines()
        .find(|l| l.starts_with("  outside the closure"))
        .expect("the outside line");
    assert!(outside.contains("descriptor-format"), "{outside}");
    assert!(!report
        .lines()
        .any(|l| l.starts_with("  closure: ") && l.contains("descriptor-format")));
}

#[test]
fn arm_a_report_that_drops_the_boundary_is_reported() {
    let report = "x.eadl: accepted against profile `rt-static-up-v1` (1 declaration(s))\n  this checks the description, not a system\n";
    let wrong = report_violations("x.eadl", report);
    assert_eq!(wrong.len(), 2, "{wrong:?}");
    assert!(
        wrong[0].contains("0 `closure:` line(s)")
            && wrong[1].contains("0 `outside the closure` line(s)")
    );
}

#[test]
fn arm_an_owner_outside_the_closure_is_reported() {
    let report = "  closure: a (requested), b (needed by z)\n  outside the closure: nothing\n";
    let wrong = report_violations("x.eadl", report);
    assert_eq!(wrong.len(), 1, "{wrong:?}");
    assert!(
        wrong[0].contains("`b` is said to be needed by `z`"),
        "{wrong:?}"
    );
}
