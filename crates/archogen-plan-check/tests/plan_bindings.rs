//! Checks (3) and (4) (leaf `M3.5.4.4`): each requirement that binds is bound once, each binding is one requirement's,
//! of a requester the system serves, to one of the description's blocks or platforms; each binding satisfied by the
//! model's judgement; each mismatch a corrupted plan at the requirement's span, its requirement located by the checker
//! and held here to the model's own reading.

use std::collections::{BTreeMap, BTreeSet};
use std::path::Path;

use archogen_outcome::record::{Finding, Severity};
use archogen_outcome::Status;
use archogen_plan_check::assignment::{program_digest, Assignment, Requester};
use archogen_plan_check::check::{
    bindings, Verdict, PLAN_BINDING, PLAN_DESCRIPTION, PLAN_UNSATISFIED,
};
use archogen_plan_check::model::requirement::read_side;
use archogen_plan_check::requesters::sides;
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Form, Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;

/// The description every case varies: a system whose task needs a timer's channel and uses a monotonic time service
/// and a console; an idle hook relying on no function, so its qualifier binds; a statement and a privilege requirement
/// that bind nothing; a service nothing reaches; a slow counter beside the timer.
const DESCRIPTION: &str = "\
(defblock timer
  (offers (relative-delay true) (channels 2) (counter-width 32 bit) (tick-rate 1 MHz) (counter-modulus (pow2 32))
          observation-coherent (available-in-state run idle) (reachable-at-privilege machine)))
(defblock slow
  (offers (counter-width 16 bit) tick-rate (reachable-at-privilege machine))
  (absent observation-coherent))
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer slow console.uart)))
(defservice time.monotonic
  (requires (unambiguous-horizon (at-least 60 s)) (counter-width (at-least 32 bit)) (available-in-state run)
            (needs tick-rate observation-coherent)))
(defservice console.write
  (requires (needs observable-output) (or-through-mediation allowed) (reachable-at-privilege machine)))
(defservice idle.hook (requires (available-in-state idle)))
(defservice stray (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1)
        (uses time.monotonic console.write) (needs relative-delay))
  (requires (uses idle.hook))
  (platform (uses soc)))
";

fn elaborated(declarations: &str) -> (SourceMap, Program) {
    let modules = MemoryModules::new().with(
        "app.case",
        &format!("(defmodule app.case\n  (version 1 0)\n{declarations})"),
    );
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, &modules, "app.case");
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    (sources, program)
}

fn named(name: &str) -> Requester {
    Requester::Named(name.to_owned())
}

fn beat() -> Requester {
    Requester::Task {
        system: "s".to_owned(),
        task: "beat".to_owned(),
    }
}

/// The plan a search would write for [`DESCRIPTION`].
fn plan(program: &Program) -> Assignment {
    let mut bindings = BTreeMap::new();
    for (requester, fact, provider) in [
        (beat(), "relative-delay", "timer"),
        (named("time.monotonic"), "unambiguous-horizon", "timer"),
        (named("time.monotonic"), "counter-width", "timer"),
        (named("time.monotonic"), "tick-rate", "timer"),
        (named("time.monotonic"), "observation-coherent", "timer"),
        (named("console.write"), "observable-output", "console.uart"),
        (named("idle.hook"), "available-in-state", "timer"),
    ] {
        bindings.insert((requester, fact.to_owned()), provider.to_owned());
    }
    Assignment {
        profile: RT_STATIC_UP_V1.id.to_owned(),
        system: "s".to_owned(),
        program: program_digest(program),
        placement: None,
        bindings,
    }
}

fn checked(sources: &SourceMap, program: &Program, plan: &Assignment) -> Result<(), Verdict> {
    bindings(sources, program, "plan.eadl", &plan.to_canonical(), plan)
}

/// Every finding of a corrupted plan, each an error with a repair.
fn corrupted(verdict: Verdict) -> Vec<Finding> {
    assert_eq!(verdict.status, Status::ToolFailure, "{verdict:?}");
    for f in &verdict.findings {
        assert_eq!(f.severity, Severity::Error);
        assert!(!f.repair.is_empty(), "{f:?}");
    }
    verdict.findings
}

/// The record's line binding `fact` for `requester`.
fn line_of(plan: &Assignment, requester: &Requester, fact: &str) -> u32 {
    let clause = format!("(bind {} {fact} ", requester.canonical());
    let index = plan
        .to_canonical()
        .lines()
        .position(|l| l.starts_with(&clause))
        .expect("the record binds it");
    u32::try_from(index).unwrap() + 1
}

#[test]
fn a_plan_binding_every_requirement_to_a_provider_that_satisfies_it_passes() {
    let (sources, program) = elaborated(DESCRIPTION);
    assert_eq!(checked(&sources, &program, &plan(&program)), Ok(()));
}

#[test]
fn check_three_refuses_a_requirement_left_unbound() {
    let (sources, program) = elaborated(DESCRIPTION);
    let mut unbound = plan(&program);
    unbound
        .bindings
        .remove(&(named("time.monotonic"), "tick-rate".to_owned()));
    unbound
        .bindings
        .remove(&(beat(), "relative-delay".to_owned()));
    let findings = corrupted(checked(&sources, &program, &unbound).unwrap_err());
    assert_eq!(findings.len(), 2, "{findings:?}");
    for (finding, (who, fact, line)) in findings.iter().zip([
        ("`time.monotonic`", "tick-rate", 13),
        ("the task `beat` of `s`", "relative-delay", 20),
    ]) {
        assert_eq!(finding.code, PLAN_BINDING);
        assert!(
            finding.message.contains(who) && finding.message.contains(fact),
            "{finding:?}"
        );
        // The requirement, where the description writes it.
        assert_eq!(
            (finding.primary.source.as_str(), finding.primary.line),
            ("app.case.eadl", line),
            "{finding:?}"
        );
    }
}

#[test]
fn check_three_refuses_a_binding_of_no_requirement_or_of_a_requester_the_system_does_not_serve() {
    let (sources, program) = elaborated(DESCRIPTION);
    let other_task = Requester::Task {
        system: "s".to_owned(),
        task: "nobody".to_owned(),
    };
    for (requester, fact, provider, said) in [
        (
            named("stray"),
            "observable-output",
            "console.uart",
            "does not serve",
        ),
        (other_task, "relative-delay", "timer", "does not serve"),
        (
            named("console.write"),
            "or-through-mediation",
            "timer",
            "a statement",
        ),
        (
            named("console.write"),
            "reachable-at-privilege",
            "timer",
            "rule 9",
        ),
        (
            named("time.monotonic"),
            "available-in-state",
            "timer",
            "rule 13",
        ),
        (
            named("time.monotonic"),
            "clock-rate",
            "timer",
            "requires nothing of it",
        ),
    ] {
        let mut extra = plan(&program);
        extra
            .bindings
            .insert((requester.clone(), fact.to_owned()), provider.to_owned());
        let findings = corrupted(checked(&sources, &program, &extra).unwrap_err());
        assert_eq!(findings.len(), 1, "{findings:?}");
        let finding = &findings[0];
        assert_eq!(finding.code, PLAN_BINDING);
        assert!(finding.message.contains(said), "{said}: {finding:?}");
        // The binding, where the record writes it.
        assert_eq!(
            (finding.primary.source.as_str(), finding.primary.line),
            ("plan.eadl", line_of(&extra, &requester, fact)),
            "{finding:?}"
        );
    }
}

#[test]
fn check_three_refuses_a_provider_that_is_none_of_the_description_s_blocks_or_platforms() {
    let (sources, program) = elaborated(DESCRIPTION);
    for provider in ["nowhere", "stray"] {
        let mut moved = plan(&program);
        let key = (named("console.write"), "observable-output".to_owned());
        moved.bindings.insert(key.clone(), provider.to_owned());
        let findings = corrupted(checked(&sources, &program, &moved).unwrap_err());
        assert_eq!(findings.len(), 1, "{findings:?}");
        assert_eq!(findings[0].code, PLAN_BINDING);
        assert!(
            findings[0]
                .message
                .contains(&format!("`{provider}`, which is none")),
            "{findings:?}"
        );
        assert_eq!(findings[0].primary.line, line_of(&moved, &key.0, &key.1));
    }
}

#[test]
fn check_four_judges_each_binding_at_its_provider() {
    let (sources, program) = elaborated(DESCRIPTION);
    // Moved to the slow counter: a width below the bound refused, an absence, a horizon its rule cannot compute; the
    // presence of an unvalued tick rate holds.
    let mut moved = plan(&program);
    for fact in [
        "unambiguous-horizon",
        "counter-width",
        "tick-rate",
        "observation-coherent",
    ] {
        moved.bindings.insert(
            (named("time.monotonic"), fact.to_owned()),
            "slow".to_owned(),
        );
    }
    // And a console bound to the timer, which does not describe output.
    moved.bindings.insert(
        (named("console.write"), "observable-output".to_owned()),
        "timer".to_owned(),
    );
    let findings = corrupted(checked(&sources, &program, &moved).unwrap_err());
    let said: BTreeSet<(&str, u32)> = findings
        .iter()
        .map(|f| {
            assert_eq!(f.code, PLAN_UNSATISFIED, "{f:?}");
            assert_eq!(f.primary.source, "app.case.eadl", "{f:?}");
            assert_eq!(f.secondary[0].source, "plan.eadl", "{f:?}");
            let found = [
                "lies outside it",
                "declares it absent",
                "is unknown",
                "does not describe it",
            ]
            .into_iter()
            .find(|word| f.message.contains(word))
            .unwrap_or_else(|| panic!("{f:?}"));
            (found, f.primary.line)
        })
        .collect();
    assert_eq!(findings.len(), 4, "{findings:?}");
    assert_eq!(
        said,
        BTreeSet::from([
            ("is unknown", 12),
            ("lies outside it", 12),
            ("declares it absent", 13),
            ("does not describe it", 15),
        ]),
        "{findings:?}"
    );
    // What the provider states is named beside the binding.
    let width = findings
        .iter()
        .find(|f| f.message.contains("(counter-width (at-least 32 bit))"))
        .expect("the width's finding quotes the requirement");
    assert!(
        width
            .secondary
            .iter()
            .any(|l| l.source == "app.case.eadl" && l.line == 7 && l.message.contains("`slow`")),
        "{width:?}"
    );
}

#[test]
fn what_the_model_refuses_to_read_is_the_description_s_status() {
    // A bound provider stating two values of one fact.
    let twice = DESCRIPTION.replace(
        "(counter-width 32 bit) (tick-rate 1 MHz)",
        "(counter-width 32 bit) (counter-width 64 bit) (tick-rate 1 MHz)",
    );
    let (sources, program) = elaborated(&twice);
    let verdict = checked(&sources, &program, &plan(&program)).unwrap_err();
    assert_eq!(verdict.status, Status::InvalidDescription, "{verdict:?}");
    assert!(
        verdict
            .findings
            .iter()
            .all(|f| f.code == PLAN_DESCRIPTION && f.message.contains("`timer`")),
        "{verdict:?}"
    );
    // A served requester whose requirements contradict.
    let contradiction = DESCRIPTION.replace(
        "(defservice idle.hook (requires (available-in-state idle)))",
        "(defservice idle.hook (requires (counter-width (at-least 16 bit)) (counter-width (exactly 8 bit))))",
    );
    let (sources, program) = elaborated(&contradiction);
    let verdict = checked(&sources, &program, &plan(&program)).unwrap_err();
    assert_eq!(verdict.status, Status::InvalidDescription, "{verdict:?}");
    assert_eq!(verdict.findings[0].code, PLAN_DESCRIPTION);
    assert!(
        verdict.findings[0].message.contains("`idle.hook`"),
        "{verdict:?}"
    );
}

/// Each declaration of a file, those of a module's body among them.
fn declarations(forms: &[Form], out: &mut Vec<Form>) {
    for form in forms {
        match form.head() {
            Some("defmodule") => declarations(&form.items()[1..], out),
            Some(head) if head.starts_with("def") => out.push(form.clone()),
            _ => {}
        }
    }
}

#[test]
fn every_requirement_the_model_reads_is_located_once() {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut files = Vec::new();
    for dir in [
        "docs/semantics/cases",
        "docs/semantics/boundary/accept",
        "docs/semantics/boundary/reject",
    ] {
        files.extend(
            std::fs::read_dir(root.join(dir))
                .unwrap()
                .map(|e| e.unwrap().path()),
        );
    }
    for example in std::fs::read_dir(root.join("examples")).unwrap() {
        let example = example.unwrap().path();
        if example.is_dir() {
            files.extend(
                std::fs::read_dir(example)
                    .unwrap()
                    .map(|e| e.unwrap().path()),
            );
        }
    }
    files.retain(|p| p.extension().is_some_and(|e| e == "eadl"));
    files.sort();
    let mut sources = SourceMap::new();
    let mut forms = Vec::new();
    for path in &files {
        let id = sources
            .add(
                path.display().to_string(),
                std::fs::read_to_string(path).unwrap(),
            )
            .unwrap();
        declarations(&eadl_front::read(&sources, id).0.forms, &mut forms);
    }
    let fixture = sources.add("fixture", DESCRIPTION).unwrap();
    declarations(&eadl_front::read(&sources, fixture).0.forms, &mut forms);
    let (mut sides_read, mut located) = (0, 0);
    for decl in &forms {
        let Ok(model) = read_side(decl) else {
            assert!(sides(decl).is_err(), "{decl:?}");
            continue;
        };
        let requesters = sides(decl).unwrap_or_else(|e| panic!("{e:?}: {decl:?}"));
        let mut spans = BTreeSet::new();
        let mut ours: Vec<String> = Vec::new();
        for side in &requesters {
            for l in &side.requirements {
                assert!(
                    spans.insert((l.span.source, l.span.start, l.span.end)),
                    "located twice: {l:?}"
                );
                ours.push(format!("{:?}", l.requirement));
            }
        }
        let mut theirs: Vec<String> = model.iter().map(|r| format!("{r:?}")).collect();
        ours.sort();
        theirs.sort();
        assert_eq!(ours, theirs, "{decl:?}");
        sides_read += 1;
        located += ours.len();
    }
    // The corpus is not empty, and a task is a requester of its own.
    assert!(
        sides_read > 100 && located > 100,
        "{sides_read} sides, {located} requirements"
    );
    let s = forms
        .iter()
        .rfind(|f| f.items().get(1).and_then(Form::as_symbol) == Some("s"))
        .unwrap();
    let tasks: Vec<_> = sides(s).unwrap().iter().map(|side| side.task).collect();
    assert_eq!(tasks, [None, Some("beat")]);
}
