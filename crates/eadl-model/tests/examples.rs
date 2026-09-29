//! The three M0 use cases, expressed in eADL and checked.
//!
//! `M1.7`'s acceptance: *"the three M0 use cases express completely; no execution bound, code
//! reference, or allocation appears in eADL."*
//!
//! Both halves are checked, and the second is the one that is easy to assert and hard to mean.
//! "No execution bound appears" is not a claim about what anyone remembered to leave out — it is
//! the boundary classifier returning `Accepted` on every example, with `wcet`, `entry-point` and
//! `stack-allocation` all in its registry and all exercised by worked corpus cases.

use std::path::{Path, PathBuf};

use eadl_front::{read, Form, SourceMap};
use eadl_model::boundary::{classify, Classification};
use eadl_model::check::shipped_registry;
use eadl_model::kind::{validate, Registry};
use eadl_model::presence::FactMap;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn parse_file(relative: &str) -> (Vec<Form>, SourceMap) {
    let path = repo_root().join(relative);
    let text = std::fs::read_to_string(&path)
        .unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()));
    let mut sources = SourceMap::new();
    let id = sources.add(relative, text).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "{relative} did not read:\n{}",
        diagnostics.render(&sources)
    );
    (document.forms, sources)
}

/// The shipped registry, built by the **production loader** rather than by a second one here.
///
/// ⭐ This helper used to loop over `read_kind` itself, which made it a second implementation of
/// "what a kind module may contain" — and a second implementation is a second place for §8's rule to
/// be missing. `M1.13.4.1` measured the cost of exactly that: the production loader refused a kind
/// module stating its language version as `schema-not-a-kind`, and this helper would have refused it
/// too, so a test suite could have stayed green on a loader that could not read the shipped files.
/// The declarations of a parsed file: every top-level form except the language-version identifier.
///
/// §8 of `docs/semantics/reference.md` — the identifier is a statement about the document, not a
/// declaration. ⛔ A call through to the frontend's own accessor and not a second filter here: a rule
/// each consumer re-implements is a rule the next consumer lacks, and `M1.13.4.2` measured what that
/// costs, because every example states its version and a leg that validated *forms* was handed the
/// identifier as though it were a declaration (`error[schema-unknown-kind]`).
fn declarations(forms: &[Form]) -> Vec<&Form> {
    eadl_front::language_version::declarations(forms).collect()
}

fn registry() -> Registry {
    let mut sources = SourceMap::new();
    let kind_files: Vec<(String, String)> = [
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
    shipped_registry(&mut sources, &kind_files).unwrap_or_else(|errors| {
        panic!(
            "the shipped kind modules are malformed:\n{}",
            errors
                .iter()
                .map(|d| d.render(&sources))
                .collect::<Vec<_>>()
                .join("\n")
        )
    })
}

/// The examples, as `(relative path, use case)`.
const EXAMPLES: &[(&str, &str)] = &[
    ("examples/periodic-three/system.eadl", "uc1"),
    ("examples/high-interference/system.eadl", "uc2"),
    ("examples/alternative-timer/system.eadl", "uc3"),
];

#[test]
fn every_use_case_example_validates_against_the_shipped_kinds() {
    let registry = registry();
    for (relative, case) in EXAMPLES {
        let (forms, sources) = parse_file(relative);
        assert!(!forms.is_empty(), "{case}: {relative} has no declarations");
        for form in declarations(&forms) {
            let errors = validate(&registry, form);
            assert!(
                errors.is_empty(),
                "{case} ({relative}) does not validate:\n{}",
                errors
                    .iter()
                    .map(|d| d.render(&sources))
                    .collect::<Vec<_>>()
                    .join("\n")
            );
        }
    }
}

#[test]
fn no_example_carries_an_execution_bound_a_code_reference_or_an_allocation() {
    // ⭐ The half that is easy to assert and hard to mean. This is not "nobody wrote one" — it
    // is the boundary classifier returning Accepted with `wcet`, `entry-point` and
    // `stack-allocation` all registered and all exercised by worked corpus cases. If someone
    // adds a WCET to an example to make an analysis pass, this is what stops them.
    for (relative, case) in EXAMPLES {
        let (forms, _) = parse_file(relative);
        for form in declarations(&forms) {
            match classify(form) {
                Classification::Accepted => {}
                Classification::Rejected { head, .. } => panic!(
                    "{case} ({relative}) carries `{head}`, which is implementation and belongs \
                     outside the description"
                ),
            }
        }
    }
}

#[test]
fn every_example_declares_a_system_with_tasks_and_a_platform() {
    for (relative, case) in EXAMPLES {
        let (forms, _) = parse_file(relative);
        let system = forms
            .iter()
            .find(|form| form.head() == Some("defsystem"))
            .unwrap_or_else(|| panic!("{case}: no `defsystem` in {relative}"));

        let tasks = system
            .items()
            .iter()
            .filter(|clause| clause.head() == Some("task"))
            .count();
        assert!(
            tasks >= 2,
            "{case}: {tasks} task(s) — too few to exercise preemption"
        );

        assert!(
            system.items().iter().any(|c| c.head() == Some("platform")),
            "{case}: the system does not bind a platform"
        );
    }
}

#[test]
fn every_task_carries_a_release_model_a_deadline_and_a_priority() {
    // §3.1 admits "periodic or sporadic releases with declared minimum separation; constrained
    // deadlines". A task with neither a period nor a minimum separation has no arrival model,
    // and nothing downstream can analyze it — so the frontend is the right place to notice.
    for (relative, case) in EXAMPLES {
        let (forms, _) = parse_file(relative);
        for system in forms.iter().filter(|f| f.head() == Some("defsystem")) {
            for task in system.items().iter().filter(|c| c.head() == Some("task")) {
                let name = task
                    .items()
                    .get(1)
                    .and_then(Form::as_symbol)
                    .unwrap_or("<unnamed>");
                let has = |clause: &str| task.items().iter().any(|c| c.head() == Some(clause));
                assert!(
                    has("period") || has("min-separation"),
                    "{case} ({relative}): task `{name}` has no arrival model"
                );
                assert!(has("deadline"), "{case}: task `{name}` has no deadline");
                assert!(has("priority"), "{case}: task `{name}` has no priority");
                assert!(
                    has("deadline-from"),
                    "{case}: task `{name}` does not say what its deadline is relative to — \
                     §7.3 requires the reference event"
                );
            }
        }
    }
}

#[test]
fn task_priorities_are_unique_within_a_system() {
    // §3.1: "static unique task priorities". Two tasks at one priority is outside the profile,
    // and an analysis run on such a set answers a question about a different system.
    for (relative, case) in EXAMPLES {
        let (forms, _) = parse_file(relative);
        for system in forms.iter().filter(|f| f.head() == Some("defsystem")) {
            let mut priorities: Vec<i64> = Vec::new();
            for task in system.items().iter().filter(|c| c.head() == Some("task")) {
                for clause in task.items().iter().filter(|c| c.head() == Some("priority")) {
                    if let Some(Form::Integer { value, .. }) = clause.items().get(1) {
                        priorities.push(*value);
                    }
                }
            }
            let total = priorities.len();
            priorities.sort_unstable();
            priorities.dedup();
            assert_eq!(
                priorities.len(),
                total,
                "{case} ({relative}): duplicate task priorities"
            );
        }
    }
}

#[test]
fn uc1_and_uc2_have_no_missing_facts() {
    // Every fact these two systems depend on is described. `uc3` is deliberately different and
    // is checked on its own below.
    for relative in [
        "examples/periodic-three/system.eadl",
        "examples/high-interference/system.eadl",
    ] {
        let (forms, sources) = parse_file(relative);
        let mut facts = FactMap::new();
        for form in declarations(&forms) {
            facts.collect(form);
        }
        let report = facts.check();
        assert!(
            report.is_admissible(),
            "{relative} has unresolved facts:\n{}",
            report
                .diagnostics
                .iter()
                .map(|d| d.render(&sources))
                .collect::<Vec<_>>()
                .join("\n")
        );
    }
}

#[test]
fn uc3_states_the_gap_it_exists_to_expose_rather_than_hiding_it() {
    // ⭐ uc3's platform offers only a relative delay timer and declares `absolute-deadline`
    // ABSENT, while its service requires it. That contradiction is the case — §5.4's indirect
    // realization is what must close it, through an engine-owned adapter, without the
    // description changing.
    //
    // If someone "fixed" uc3 by deleting the absence or weakening the requirement, the case
    // would go quiet and stop testing anything. This asserts both halves are still there.
    let (forms, _) = parse_file("examples/alternative-timer/system.eadl");

    let timer = forms
        .iter()
        .find(|f| f.items().get(1).and_then(Form::as_symbol) == Some("timer.delay"))
        .expect("the delay-only timer");
    assert!(
        timer.items().iter().any(|c| c.head() == Some("absent")
            && c.items()
                .iter()
                .any(|i| i.as_symbol() == Some("absolute-deadline"))),
        "uc3's platform no longer declares `absolute-deadline` absent — the case has gone quiet"
    );

    let service = forms
        .iter()
        .find(|f| f.items().get(1).and_then(Form::as_symbol) == Some("time.deadline"))
        .expect("the deadline service");
    assert!(
        service.to_canonical().contains("absolute-deadline"),
        "uc3's service no longer requires `absolute-deadline` — the requirement was weakened \
         instead of the engine being extended, which §17's first stop/rework trigger forbids"
    );
}
