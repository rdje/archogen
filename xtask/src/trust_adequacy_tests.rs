//! The reviewed generated-sources census binds both exact mutation populations (`M3.6.9.4.2.1`).
//! These are catalogue contracts, outside either population's semantic tests.

use super::{parse, Expect, Mutation};
use std::collections::BTreeSet;
use std::path::Path;

const MARKER: &str = "<!-- machine-read: parent-mutation-census -->";
const ROUTE_TEST: &[&str] = &["-p", "xtask", "route_tests"];
const PARENT_TEST: &[&str] = &[
    "-p",
    "xtask",
    "--",
    "trust::",
    "trust_generated::",
    "trust_gate::",
    "--skip",
    "route_tests",
];

// The accepted census, derived from the frozen review findings. Keeping these identities independently of the
// live decision table catches a row removed together with its two entries, not just disagreement between them.
const CENSUS: &[(&str, &str, &str)] = &[
    ("the-first-generator-alone", "trust.rs", "R8-D1; R9-D1"),
    ("the-last-generator-alone", "trust.rs", "R9-D1"),
    ("the-first-reader-alone", "trust.rs", "R8-D1; R9-D1"),
    ("the-last-reader-alone", "trust.rs", "R9-D1"),
    (
        "a-program-target-judged-for-its-first-reader-alone",
        "trust.rs",
        "R9-D1",
    ),
    (
        "a-program-target-judged-for-its-last-reader-alone",
        "trust.rs",
        "R9-D1",
    ),
    (
        "chain-and-blob-rules-on-each-clause-s-first-entry-alone",
        "trust_generated.rs",
        "R7-D1; R10-D1; R10-R2",
    ),
    (
        "chain-and-blob-rules-on-each-clause-s-last-entry-alone",
        "trust_generated.rs",
        "R7-D1; R10-D1; R10-R2",
    ),
    (
        "both-clauses-the-first-input-alone",
        "trust_generated.rs",
        "R10-D1",
    ),
    (
        "both-clauses-the-last-input-alone",
        "trust_generated.rs",
        "R10-D1",
    ),
    (
        "both-clauses-the-last-generator-alone",
        "trust_generated.rs",
        "R10-D1",
    ),
    (
        "a-program-target-judged-when-first-alone",
        "trust.rs",
        "R11-D1",
    ),
    (
        "a-program-target-judged-when-last-alone",
        "trust.rs",
        "R11-D1",
    ),
    (
        "a-program-target-built-when-first-alone",
        "trust.rs",
        "R11-D2; R12-D2",
    ),
    (
        "a-program-target-built-when-last-alone",
        "trust.rs",
        "R11-D2; R12-D2",
    ),
    ("a-lone-rs-refused-when-first-alone", "trust.rs", "R11-D2"),
    ("a-lone-rs-refused-when-last-alone", "trust.rs", "R11-D2"),
    ("a-middle-script-never-judged", "trust.rs", "R12-D1"),
    ("a-middle-reader-never-judged", "trust.rs", "R12-D3; R13-D1"),
    (
        "a-middle-input-never-reached",
        "trust_generated.rs",
        "R12-D4",
    ),
    (
        "an-implementation-reader-never-judged",
        "trust.rs",
        "R13-D1",
    ),
    (
        "a-marked-build-read-dropped-beside-other-generators",
        "trust.rs",
        "R13-D3",
    ),
    (
        "only-a-form-s-first-input-reached",
        "trust_generated.rs",
        "R1-D2; R9-D2",
    ),
    (
        "an-input-reached-through-a-link",
        "trust_generated.rs",
        "R23-R2",
    ),
    ("held-judged-for-the-first-alone", "trust.rs", "R18-R1"),
    (
        "a-harness-never-judged-against-a-program-target",
        "trust.rs",
        "R13-D2; R14-D4",
    ),
    ("the-last-live-form-alone", "trust_generated.rs", "R14-D3"),
    (
        "a-harness-judged-by-its-first-role-alone",
        "trust.rs",
        "R18-R2",
    ),
    ("a-data-reader-never-judged", "trust.rs", "R16-R3; R18-R3"),
    (
        "a-build-read-naming-its-first-generator-alone",
        "trust.rs",
        "R18-R4",
    ),
    (
        "a-middle-generator-never-reached",
        "trust_generated.rs",
        "R13-D4",
    ),
    (
        "a-transitive-generator-build-read-dropped",
        "trust.rs",
        "R6-D1; R10-D2",
    ),
];

fn documents() -> (String, String, Vec<Mutation>) {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).parent().unwrap();
    (
        std::fs::read_to_string(root.join("docs/specs/trust/decision_trust-generated-adequacy.md"))
            .unwrap(),
        std::fs::read_to_string(
            root.join("docs/review-history/decision_trust-generated-refusals-reviews.md"),
        )
        .unwrap(),
        parse(&std::fs::read_to_string(root.join("xtask/mutations.txt")).unwrap()).unwrap(),
    )
}

fn check(decision: &str, history: &str, catalogue: &[Mutation]) -> Result<(), String> {
    if decision.matches(MARKER).count() != 1 {
        return Err("census: expected one machine-read marker".to_owned());
    }
    let table: Vec<_> = decision
        .split_once(MARKER)
        .unwrap()
        .1
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .take_while(|line| line.starts_with('|'))
        .collect();
    if table.len() != CENSUS.len() + 2
        || table[0] != "| Suffix | Source findings | Catalogue origin and file |"
        || table[1] != "| --- | --- | --- |"
    {
        return Err("census: table shape differs from the accepted census".to_owned());
    }
    for (index, (line, &(suffix, file, findings))) in table[2..].iter().zip(CENSUS).enumerate() {
        let cells: Vec<_> = line
            .strip_prefix('|')
            .and_then(|row| row.strip_suffix('|'))
            .ok_or_else(|| "census: row lacks delimiters".to_owned())?
            .split('|')
            .map(str::trim)
            .collect();
        let origin = if index < 23 { "existing" } else { "added" };
        if cells
            != [
                format!("`{suffix}`"),
                findings.to_owned(),
                format!("{origin}, `{file}`"),
            ]
        {
            return Err(format!(
                "census: row for `{suffix}` differs from the accepted source census"
            ));
        }
        for finding in findings.split("; ") {
            let start = format!("| {finding} |");
            if !history.lines().any(|row| row.starts_with(&start)) {
                return Err(format!(
                    "source finding `{finding}` has no frozen review row"
                ));
            }
        }
    }
    for prefix in ["route", "parent"] {
        let expected: BTreeSet<_> = CENSUS
            .iter()
            .map(|(suffix, _, _)| format!("{prefix}-{suffix}"))
            .collect();
        let actual: Vec<_> = catalogue
            .iter()
            .filter(|entry| entry.id.starts_with(&format!("{prefix}-")))
            .map(|entry| entry.id.clone())
            .collect();
        if actual.len() != expected.len() || actual.into_iter().collect::<BTreeSet<_>>() != expected
        {
            return Err(format!(
                "{prefix} population differs from the accepted census"
            ));
        }
    }
    for &(suffix, file, _) in CENSUS {
        let route = catalogue
            .iter()
            .find(|entry| entry.id == format!("route-{suffix}"))
            .unwrap();
        let parent = catalogue
            .iter()
            .find(|entry| entry.id == format!("parent-{suffix}"))
            .unwrap();
        for (name, entry, arguments) in [
            ("route", route, ROUTE_TEST),
            ("parent", parent, PARENT_TEST),
        ] {
            if !entry
                .test
                .iter()
                .map(String::as_str)
                .eq(arguments.iter().copied())
            {
                return Err(format!("{name} test arguments differ for `{suffix}`"));
            }
            if entry.expect != Expect::Killed {
                return Err(format!(
                    "{name} `{suffix}` must retain the expected killed criterion"
                ));
            }
        }
        if route.file != format!("xtask/src/{file}") {
            return Err(format!("route source file differs for `{suffix}`"));
        }
        if route.file != parent.file || route.from != parent.from || route.to != parent.to {
            return Err(format!("twin source differs for `{suffix}`"));
        }
    }
    Ok(())
}

fn refuses(result: Result<(), String>, reason: &str) {
    let message = result.expect_err("altered census or catalogue must be refused");
    assert!(
        message.contains(reason),
        "expected `{reason}`, got `{message}`"
    );
}

#[test]
fn generated_sources_census_has_exact_route_parent_pairs() {
    assert_eq!(
        CENSUS.len(),
        32,
        "the accepted record specifies 32 exact pairs"
    );
    let (decision, history, catalogue) = documents();
    check(&decision, &history, &catalogue).unwrap();
}

#[test]
fn census_and_frozen_source_reference_drift_are_refused() {
    let (decision, history, catalogue) = documents();
    check(&decision, &history, &catalogue).unwrap();
    let row = decision
        .lines()
        .find(|line| line.starts_with("| `an-input-reached-through-a-link` |"))
        .unwrap();
    refuses(
        check(&decision.replace(MARKER, ""), &history, &catalogue),
        "census",
    );
    refuses(
        check(
            &decision.replace(&format!("{row}\n"), ""),
            &history,
            &catalogue,
        ),
        "census",
    );
    refuses(
        check(
            &decision.replace(row, &format!("{row}\n{row}")),
            &history,
            &catalogue,
        ),
        "census",
    );
    refuses(
        check(&decision.replace("R23-R2", "R23-R99"), &history, &catalogue),
        "census",
    );
    refuses(
        check(
            &decision.replace(row, &row.replace("trust_generated.rs", "trust.rs")),
            &history,
            &catalogue,
        ),
        "census",
    );
    refuses(
        check(
            &decision,
            &history.replace("| R23-R2 |", "| moved-source |"),
            &catalogue,
        ),
        "source finding",
    );
}

#[test]
fn missing_extra_duplicate_and_drifting_twins_are_refused() {
    let (decision, history, catalogue) = documents();
    check(&decision, &history, &catalogue).unwrap();
    let route = catalogue
        .iter()
        .position(|entry| entry.id == format!("route-{}", CENSUS[0].0))
        .unwrap();
    let parent = catalogue
        .iter()
        .position(|entry| entry.id == format!("parent-{}", CENSUS[0].0))
        .unwrap();
    for (prefix, index) in [("route", route), ("parent", parent)] {
        let mut missing = catalogue.clone();
        missing.remove(index);
        refuses(
            check(&decision, &history, &missing),
            &format!("{prefix} population"),
        );
        let mut extra = catalogue.clone();
        let mut entry = extra[index].clone();
        entry.id = format!("{prefix}-outside-the-census");
        extra.push(entry);
        refuses(
            check(&decision, &history, &extra),
            &format!("{prefix} population"),
        );
        let mut duplicate = catalogue.clone();
        duplicate.push(duplicate[index].clone());
        refuses(
            check(&decision, &history, &duplicate),
            &format!("{prefix} population"),
        );
        let mut arguments = catalogue.clone();
        arguments[index].test.push("unrelated_filter".to_owned());
        refuses(
            check(&decision, &history, &arguments),
            &format!("{prefix} test arguments"),
        );
        let mut verdict = catalogue.clone();
        verdict[index].expect = Expect::Survives;
        refuses(check(&decision, &history, &verdict), "expected killed");
    }
    let mut file = catalogue.clone();
    file[parent].file = "xtask/src/trust_gate.rs".to_owned();
    refuses(check(&decision, &history, &file), "twin source");
    let mut from = catalogue.clone();
    from[parent].from.push_str(" drift");
    refuses(check(&decision, &history, &from), "twin source");
    let mut to = catalogue.clone();
    to[parent].to.push_str(" drift");
    refuses(check(&decision, &history, &to), "twin source");
    let mut route_file = catalogue.clone();
    route_file[route].file = "xtask/src/trust_gate.rs".to_owned();
    refuses(check(&decision, &history, &route_file), "route source file");
}
