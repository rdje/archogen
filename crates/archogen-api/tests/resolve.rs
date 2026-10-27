//! The engine API's `resolve` (leaf `M3.4.4`, API `1.4`): the search alone, asked with `unchecked` set and refused
//! without it (`ROADMAP.md` §10.4); a plain description and a module of one instance resolved to the plan
//! `archogen-assignment/1` writes; and every answer that is no plan — the check's, the request's, the search's —
//! carrying its status and what to change.

use archogen_api::{resolve_with, Limits, MemoryModules, Request, Resolve, Status};

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
    assert_eq!(
        plan.text,
        format!(
            "(format \"archogen-assignment/1\")\n(profile rt-static-up-v1)\n(system s)\n(program \"{}\")\n\
             (bind console.write observable-output console.uart)\n",
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
fn every_answer_that_is_no_plan_says_why() {
    let two =
        format!("{PLAIN}(defsystem t (requires (uses console.write)) (platform (uses soc)))\n");
    let several = "(defmodule app (version 1 0) (import hw (as hw))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s (requires (uses console.write)) (platform (uses hw.soc))))";
    let hw = MemoryModules::new().with(
        "hw",
        "(defmodule hw (version 1 0) (export soc console.uart)
  (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart))))",
    );
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
            "several instances",
            several.to_owned(),
            hw,
            UNCHECKED,
            Status::UnsupportedProfile,
            "elaborates to 2 instances",
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
