//! The engine API's `resolve` (leaf `M3.4.4`, API `1.4`): the search alone, asked with `unchecked` set and refused
//! without it (`ROADMAP.md` §10.4); a plain description, a module of one instance and a module tree of several resolved
//! to the plan `archogen-assignment/2` writes; and every answer that is no plan — the check's, the request's, the search's —
//! carrying its status and what to change.

use archogen_api::{resolve_with, Limits, MemoryModules, Owes, Request, Resolve, Status};

const PLAIN: &str = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
";

fn ask(text: &str, modules: &MemoryModules, resolve: Resolve<'_>) -> archogen_api::Resolution {
    resolve_with(
        &Request {
            name: "description.eadl",
            text,
            profile: None,
            modules,
        },
        &resolve,
        Limits::DEFAULT,
    )
}

const UNCHECKED: Resolve<'static> = Resolve {
    system: None,
    unchecked: true,
};

#[test]
fn resolve_without_unchecked_is_refused_with_the_repair() {
    let resolution = ask(PLAIN, &MemoryModules::new(), Resolve::default());
    assert_eq!(resolution.response.status, Status::Usage);
    assert!(resolution.plan.is_none());
    assert!(resolution.response.notes[0].contains("runs no checker"));
    assert!(resolution
        .response
        .hint
        .as_deref()
        .is_some_and(|hint| hint.contains("archogen-plan-check")));
}

#[test]
fn a_plain_description_and_a_module_of_one_instance_resolve_to_their_plans() {
    let plain = ask(PLAIN, &MemoryModules::new(), UNCHECKED);
    assert_eq!(plain.response.status, Status::Ok);
    let plan = plain.plan.expect("a plan");
    assert_eq!(plan.system, "s");
    assert_eq!(plan.bindings, 1);
    assert!(
        plan.obligations.is_empty(),
        "no adapter bound, nothing owed"
    );
    assert_eq!(
        plan.text,
        format!(
            "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system s)\n(program \"{}\")\n\
             (adapters adapters/1)\n(bind console.write observable-output console.uart)\n",
            plan.program
        )
    );
    let module = PLAIN.replace("(eadl-version eadl/1)\n", "(defmodule app (version 1 0)\n") + ")";
    let resolved = ask(&module, &MemoryModules::new(), UNCHECKED);
    assert_eq!(
        resolved.response.status,
        Status::Ok,
        "{:?}",
        resolved.response.notes
    );
    let module_plan = resolved.plan.expect("a plan");
    assert_ne!(
        module_plan.program, plan.program,
        "a module is another program"
    );
}

#[test]
fn a_module_tree_of_several_instances_resolves_its_names_qualified() {
    // Leaf `M3.4.10`: the tree read as the language checks it, so the plan names the imported provider by its path.
    let app = "(defmodule app (version 1 0) (import hw (as hw))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s (requires (uses console.write)) (platform (uses hw.soc))))";
    let hw = MemoryModules::new().with(
        "hw",
        "(defmodule hw (version 1 0) (export soc console.uart)
  (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart))))",
    );
    let resolution = ask(app, &hw, UNCHECKED);
    assert_eq!(
        resolution.response.status,
        Status::Ok,
        "{:?}",
        resolution.response.notes
    );
    let plan = resolution.plan.expect("a plan");
    assert!(
        plan.text
            .ends_with("(bind console.write observable-output hw.console.uart)\n"),
        "{}",
        plan.text
    );
}

#[test]
fn every_answer_that_is_no_plan_says_why() {
    let two =
        format!("{PLAIN}(defsystem t (requires (uses console.write)) (platform (uses soc)))\n");
    // Offered, so the check's presence pass accepts it, and at a value the requirement refuses: the search's.
    let unmet = PLAIN.replace("(observable-output true)", "(observable-output false)");
    for (case, text, modules, resolve, status, says) in [
        (
            "a description the check refuses",
            "(eadl-version eadl/1)\n(defservice broken (requries (needs x)))\n".to_owned(),
            MemoryModules::new(),
            UNCHECKED,
            Status::InvalidDescription,
            "",
        ),
        (
            "several systems",
            two.clone(),
            MemoryModules::new(),
            UNCHECKED,
            Status::Usage,
            "declares several systems",
        ),
        (
            "a system it does not declare",
            two,
            MemoryModules::new(),
            Resolve {
                system: Some("u"),
                unchecked: true,
            },
            Status::Usage,
            "declares no system `u`",
        ),
        (
            "a requirement nothing meets",
            unmet,
            MemoryModules::new(),
            UNCHECKED,
            Status::InfeasibleConfiguration,
            "system `s` of description.eadl cannot be resolved",
        ),
    ] {
        let resolution = ask(&text, &modules, resolve);
        assert_eq!(
            resolution.response.status, status,
            "{case}: {:?}",
            resolution.response.notes
        );
        assert!(resolution.plan.is_none(), "{case}");
        if says.is_empty() {
            // The check's own refusal: judged, its diagnostics say why.
            assert!(!resolution.response.diagnostics.is_empty(), "{case}");
        } else {
            assert!(
                resolution
                    .response
                    .notes
                    .iter()
                    .any(|note| note.contains(says)),
                "{case}: {:?}",
                resolution.response.notes
            );
        }
    }
}

fn fixture(name: &str) -> String {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("two levels below the root");
    std::fs::read_to_string(root.join("crates/eadl-resolve/tests/search").join(name))
        .expect("a fixture")
}

#[test]
fn f11_the_default_budget_spent_is_analysis_inconclusive_and_no_plan() {
    // ROADMAP.md §13.1 F11, as `archogen resolve` meets it: the fixture's every assignment takes a channel twice, and
    // the search's default budget is spent before it could show so — no plan, and no claim of infeasibility.
    let resolution = ask(
        &fixture("f11-solver-resource-limit.eadl"),
        &MemoryModules::new(),
        UNCHECKED,
    );
    assert_eq!(resolution.response.status, Status::AnalysisInconclusive);
    assert!(resolution.plan.is_none());
    assert_eq!(resolution.response.diagnostics.len(), 1);
    assert_eq!(
        resolution.response.diagnostics[0].code,
        "analysis-inconclusive"
    );
    assert!(resolution.response.diagnostics[0]
        .message
        .contains("judged 100000 complete assignment(s)"));
}

#[test]
fn f10_through_the_api_is_infeasible_and_says_the_engine_lacks_the_path() {
    let resolution = ask(
        &fixture("f10-unsupported-implementation-path.eadl"),
        &MemoryModules::new(),
        UNCHECKED,
    );
    assert_eq!(resolution.response.status, Status::InfeasibleConfiguration);
    assert!(resolution.plan.is_none());
    assert!(resolution.response.diagnostics[0]
        .message
        .contains("missing engine support"));
}

#[test]
fn a_plan_through_an_adapter_names_the_instance_and_what_it_binds_beneath() {
    // Leaf `M3.2.5`: the check accepts the description, the search binds the engine's epoch extender over the short
    // counter, and `archogen-assignment/2` names the instance as a provider and as the requester of what its kind needs
    // beneath (`docs/specs/adapters/decision_engine-adapters.md` §5).
    let text = std::fs::read_to_string(
        std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../eadl-resolve/tests/search/adapter-epoch-extender.eadl"),
    )
    .expect("the fixture");
    let resolution = ask(&text, &MemoryModules::new(), UNCHECKED);
    assert_eq!(
        resolution.response.status,
        Status::Ok,
        "{:?}",
        resolution.response.notes
    );
    let plan = resolution.plan.expect("a plan");
    let lines: Vec<&str> = plan.text.lines().collect();
    assert_eq!(lines[0], "(format \"archogen-assignment/2\")");
    assert_eq!(lines[4], "(adapters adapters/1)");
    let binds: Vec<&str> = lines[5..].to_vec();
    assert_eq!(
        binds,
        [
            "(bind time.monotonic counter-width (adapter epoch-extender timer.short))",
            "(bind time.monotonic observation-coherent (adapter epoch-extender timer.short))",
            "(bind time.monotonic tick-rate (adapter epoch-extender timer.short))",
            "(bind time.monotonic unambiguous-horizon (adapter epoch-extender timer.short))",
            "(bind (adapter epoch-extender timer.short) counter-modulus timer.short)",
            "(bind (adapter epoch-extender timer.short) observation-coherent timer.short)",
            "(bind (adapter epoch-extender timer.short) relative-delay timer.short)",
            "(bind (adapter epoch-extender timer.short) tick-rate timer.short)",
            "(bind (adapter epoch-extender timer.short) wrap-behavior timer.short)",
        ]
    );
    // Leaf `M3.2.6.1`, the record's §9: the plan is conditional on what the instance's kind owes — its obligations,
    // then its costs, in the order and the words the catalog writes them.
    let owed: Vec<(&str, Owes, &str)> = plan
        .obligations
        .iter()
        .map(|o| (o.instance.as_str(), o.owes, o.name.as_str()))
        .collect();
    let instance = "(adapter epoch-extender timer.short)";
    assert_eq!(
        owed,
        [
            (instance, Owes::Obligation, "update"),
            (instance, Owes::Obligation, "coherent"),
            (instance, Owes::Interrupt, "wake"),
        ]
    );
    assert_eq!(
        plan.obligations[1].doc,
        "a read joins epoch and counter so that no update is torn"
    );
    assert_eq!(
        (Owes::Obligation.as_str(), Owes::Interrupt.as_str()),
        ("obligation", "interrupt")
    );
}
