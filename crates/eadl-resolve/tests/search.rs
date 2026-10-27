//! The search (leaf `M3.4.2`): one system's providers chosen, one per requester and fact — in the policy's order,
//! backtracking past an assignment the joint rules refuse, under a budget — and what a failed search says, by
//! `SR-H18` for a requirement nothing meets and by the joint rules' reports for a set of bindings nothing makes hold.
//! Expectations are written out literally — providers, codes, counts — never computed by the code under test.

use eadl_front::{read, Form, SourceMap, Verdict};
use eadl_model::check::shipped_registry;
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::search::{search, Limits, Policy, Resolution, Search, Who};
use eadl_resolve::vocabulary::Vocabulary;

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn vocabulary() -> Vocabulary {
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root().join(r)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the kinds load");
    Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
}

fn forms(text: &str) -> Vec<Form> {
    let mut sources = SourceMap::new();
    let id = sources.add("t.eadl", text).expect("small");
    let (doc, diags) = read(&sources, id);
    assert!(!diags.has_errors(), "{text}");
    doc.forms
        .into_iter()
        .filter(|f| f.head() != Some("eadl-version"))
        .collect()
}

/// The search of `text`'s system `name`, under the profile's level.
fn run(text: &str, name: &str, policy: &Policy, limits: Limits) -> Search {
    let v = vocabulary();
    let decls = forms(text);
    let system = decls
        .iter()
        .find(|f| {
            f.head() == Some("defsystem")
                && f.items().get(1).and_then(Form::as_symbol) == Some(name)
        })
        .expect("the system")
        .clone();
    search(
        &decls,
        &system,
        RT_STATIC_UP_V1.caller_level,
        &v,
        policy,
        limits,
    )
}

fn resolved(search: Search) -> Resolution {
    match search {
        Search::Resolved(resolution) => resolution,
        other => panic!("not resolved: {other:?}"),
    }
}

/// The bindings, as `requester fact provider` lines.
fn bound(resolution: &Resolution) -> Vec<String> {
    resolution
        .bindings
        .iter()
        .map(|((requester, fact), provider)| {
            let who = match requester {
                Who::Named(name) => name.clone(),
                Who::Task { system, task } => format!("(task {system} {task})"),
            };
            format!("{who} {fact} {provider}")
        })
        .collect()
}

/// A refusal's verdict and its diagnostics' codes, in order.
fn refusal(search: Search) -> (Verdict, Vec<&'static str>, Vec<String>) {
    match search {
        Search::Refused {
            verdict,
            diagnostics,
        } => (
            verdict,
            diagnostics.iter().map(|d| d.code).collect(),
            diagnostics.iter().map(|d| d.message.clone()).collect(),
        ),
        other => panic!("not refused: {other:?}"),
    }
}

/// A console service on a platform composing one serial port.
const CONSOLE: &str = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
";

/// Two services each programming an absolute deadline, and two comparators of one channel each.
const TWO_COMPARATORS: &str = "(eadl-version eadl/1)
(defblock timer.a (offers (absolute-deadline true) (channels 1 tick) (reachable-at-privilege machine)))
(defblock timer.b (offers (absolute-deadline true) (channels 1 tick) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a timer.b)))
(defservice alarm.engine (requires (needs absolute-deadline)))
(defservice alarm.watchdog (requires (needs absolute-deadline)))
(defsystem two-alarms
  (task engine (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses alarm.engine))
  (task watchdog (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 2) (uses alarm.watchdog))
  (requires (uses alarm.engine alarm.watchdog))
  (platform (uses soc)))
";

#[test]
fn a_system_resolves_to_the_provider_its_hardware_has() {
    let resolution = resolved(run(CONSOLE, "s", &Policy::default(), Limits::DEFAULT));
    assert_eq!(resolution.system, "s");
    assert_eq!(
        bound(&resolution),
        ["console.write observable-output console.uart"]
    );
    assert_eq!(
        resolution.placement, None,
        "no region, so no placement memory"
    );
    assert_eq!(resolution.steps, 1);
}

#[test]
fn the_search_backtracks_past_an_assignment_the_joint_rules_refuse() {
    // The first assignment binds both services to `timer.a`, which has one channel; the second is the first that holds.
    let resolution = resolved(run(
        TWO_COMPARATORS,
        "two-alarms",
        &Policy::default(),
        Limits::DEFAULT,
    ));
    assert_eq!(
        bound(&resolution),
        [
            "alarm.engine absolute-deadline timer.a",
            "alarm.watchdog absolute-deadline timer.b"
        ]
    );
    assert_eq!(resolution.steps, 2);
}

#[test]
fn the_policy_orders_the_candidates_and_the_order_is_stable() {
    let prefer_b = Policy {
        prefer: vec!["timer.b".to_owned()],
    };
    let resolution = resolved(run(
        TWO_COMPARATORS,
        "two-alarms",
        &prefer_b,
        Limits::DEFAULT,
    ));
    assert_eq!(
        bound(&resolution),
        [
            "alarm.engine absolute-deadline timer.b",
            "alarm.watchdog absolute-deadline timer.a"
        ]
    );
    // The same input, the same plan.
    let again = resolved(run(
        TWO_COMPARATORS,
        "two-alarms",
        &prefer_b,
        Limits::DEFAULT,
    ));
    assert_eq!(again, resolution);
}

#[test]
fn a_spent_budget_is_inconclusive_and_no_plan() {
    match run(
        TWO_COMPARATORS,
        "two-alarms",
        &Policy::default(),
        Limits { steps: 1 },
    ) {
        Search::Inconclusive { steps, diagnostic } => {
            assert_eq!(steps, 1);
            assert_eq!(diagnostic.code, "analysis-inconclusive");
        }
        other => panic!("not inconclusive: {other:?}"),
    }
    // A budget of two reaches the plan.
    assert_eq!(
        resolved(run(
            TWO_COMPARATORS,
            "two-alarms",
            &Policy::default(),
            Limits { steps: 2 }
        ))
        .steps,
        2
    );
}

#[test]
fn bindings_nothing_makes_hold_are_infeasible_with_the_conflict_set() {
    let text = std::fs::read_to_string(
        root().join("crates/eadl-resolve/tests/joint/f08-two-exclusive-requests.eadl"),
    )
    .expect("the F08 fixture");
    // Every assignment binds both services to the one comparator; each also leaves its level unstated, an unknown, and
    // no assignment is free of the conflict, so the conflict set is the first assignment's conflicts.
    let (verdict, codes, messages) = refusal(run(
        &text,
        "two-alarms",
        &Policy::default(),
        Limits::DEFAULT,
    ));
    assert_eq!(verdict, Verdict::InfeasibleConfiguration);
    assert_eq!(codes, ["infeasible-configuration"]);
    assert!(messages[0].contains("timer.compare"), "{messages:?}");
}

#[test]
fn a_requirement_nothing_meets_takes_the_code_of_the_providers_that_take_part() {
    let system = "(defservice console.write (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
";
    let with = |blocks: &str| format!("(eadl-version eadl/1)\n{blocks}\n{system}");
    for (case, blocks, verdict, says) in [
        (
            "no provider states it",
            "(defblock console.uart (offers (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))",
            Verdict::MissingFact,
            "no provider of system `s`'s hardware meets it",
        ),
        (
            "a stray block outside the hardware takes no part",
            "(defblock console.uart (offers (reachable-at-privilege machine)))
(defblock stray (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))",
            Verdict::MissingFact,
            "no provider of system `s`'s hardware meets it",
        ),
        (
            "the one provider declares it absent",
            "(defblock console.uart (offers (reachable-at-privilege machine)) (absent observable-output))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))",
            Verdict::InfeasibleConfiguration,
            "no provider of system `s`'s hardware meets it",
        ),
        (
            "the one provider offers it false",
            "(defblock console.uart (offers (observable-output false) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))",
            Verdict::InfeasibleConfiguration,
            "no provider of system `s`'s hardware meets it",
        ),
    ] {
        let (got, codes, messages) = refusal(run(&with(blocks), "s", &Policy::default(), Limits::DEFAULT));
        assert_eq!(got, verdict, "{case}: {messages:?}");
        assert_eq!(codes, [verdict.slug()], "{case}");
        assert!(messages[0].contains("`console.write` requires `observable-output`"), "{case}: {messages:?}");
        assert!(messages[0].contains(says), "{case}: {messages:?}");
    }
}

#[test]
fn the_tracked_examples_each_search_to_a_conclusion() {
    // Every system of every tracked example ends in one of the three conclusions, deterministically: the same input
    // twice, the same answer.
    let v = vocabulary();
    let mut systems = 0;
    for entry in std::fs::read_dir(root().join("examples")).expect("examples") {
        let dir = entry.expect("an entry").path();
        let Ok(files) = std::fs::read_dir(&dir) else {
            continue;
        };
        for file in files {
            let path = file.expect("a file").path();
            if path.extension().and_then(|e| e.to_str()) != Some("eadl") {
                continue;
            }
            let text = std::fs::read_to_string(&path).expect("readable");
            let decls = forms(&text);
            for system in decls.iter().filter(|f| f.head() == Some("defsystem")) {
                systems += 1;
                let once = format!(
                    "{:?}",
                    search(
                        &decls,
                        system,
                        RT_STATIC_UP_V1.caller_level,
                        &v,
                        &Policy::default(),
                        Limits::DEFAULT
                    )
                );
                let twice = format!(
                    "{:?}",
                    search(
                        &decls,
                        system,
                        RT_STATIC_UP_V1.caller_level,
                        &v,
                        &Policy::default(),
                        Limits::DEFAULT
                    )
                );
                assert_eq!(once, twice, "{}", path.display());
            }
        }
    }
    assert!(systems >= 8, "{systems} system(s)");
}

#[test]
fn a_realization_constraint_waits_on_the_catalog_as_the_record_says() {
    // `docs/decisions/decision_substitutability-relation.md` §9: a requirement no block or platform offers — a
    // policy's `priorities`, a release's accuracy — constrains the realization, which a catalog record offers once
    // `M3.7` gives it offers. Until then the search over the description's providers finds none to take part.
    let text = format!(
        "{CONSOLE}(defpolicy scheduling.fixed-priority (requires (priorities static unique)))
(defsystem ruled
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write scheduling.fixed-priority))
  (platform (uses soc)))
"
    );
    match run(&text, "ruled", &Policy::default(), Limits::DEFAULT) {
        Search::Refused {
            verdict,
            diagnostics,
        } => {
            assert_eq!(verdict, Verdict::MissingFact);
            assert_eq!(diagnostics.len(), 1, "{diagnostics:?}");
            assert!(diagnostics[0]
                .message
                .contains("`scheduling.fixed-priority` requires `priorities`"));
            assert!(diagnostics[0]
                .repair
                .contains("waits on the catalog's providers (`M3.7`"));
        }
        other => panic!("not refused: {other:?}"),
    }
}

fn fixture(name: &str) -> String {
    std::fs::read_to_string(root().join("crates/eadl-resolve/tests/search").join(name))
        .expect("a fixture")
}

#[test]
fn f10_a_missing_realization_path_is_missing_engine_support_not_impossibility() {
    // ROADMAP.md §13.1 F10: the comparator offers every fact the service needs, its horizon short of what it asks; an
    // epoch-extending adapter (§5.2) is engine support not built yet, so the answer names it, never impossibility.
    let text = fixture("f10-unsupported-implementation-path.eadl");
    let (verdict, codes, messages) = refusal(run(
        &text,
        "deadline-far-ahead",
        &Policy::default(),
        Limits::DEFAULT,
    ));
    assert_eq!(verdict, Verdict::InfeasibleConfiguration, "{messages:?}");
    assert_eq!(codes, ["infeasible-configuration"]);
    assert!(
        messages[0].contains("`time.deadline` requires `supported-horizon`"),
        "{messages:?}"
    );
    assert!(
        messages[0].contains("missing engine support, not proof that no realization exists"),
        "{messages:?}"
    );
    assert!(messages[0].contains("leaf M3.2's"), "{messages:?}");
}

#[test]
fn f11_a_budget_spent_on_the_fixture_is_inconclusive_at_any_size_short_of_its_space() {
    // ROADMAP.md §13.1 F11 at the library: the fixture's 2^18 assignments each take a channel twice; a budget short of
    // them ends `analysis-inconclusive` with the count judged, never infeasibility. The default budget's run is the
    // engine API's test (`crates/archogen-api/tests/resolve.rs`).
    let text = fixture("f11-solver-resource-limit.eadl");
    for budget in [1, 1000] {
        match run(
            &text,
            "many-alarms",
            &Policy::default(),
            Limits { steps: budget },
        ) {
            Search::Inconclusive { steps, diagnostic } => {
                assert_eq!(steps, budget);
                assert_eq!(diagnostic.code, "analysis-inconclusive");
                assert!(
                    diagnostic.message.contains("nor showed that none exists"),
                    "{}",
                    diagnostic.message
                );
            }
            other => panic!("not inconclusive under {budget}: {other:?}"),
        }
    }
}

/// One service on two timers and a serial port: `service` what the service requires, `system` what the system's own
/// level requires beside it, `b` what `timer.b` offers of the deadline's head.
fn groups(service: &str, system: &str, b: &str) -> String {
    format!(
        "(eadl-version eadl/1)
(defblock timer.a (offers (absolute-deadline true) (supported-horizon 1 s) (channels 1 tick) (reachable-at-privilege machine)))
(defblock timer.b (offers {b} (supported-horizon 100 s) (channels 1 tick) (reachable-at-privilege machine)))
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a timer.b console.uart)))
(defservice svc (requires {service}))
(defsystem s
  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses svc))
  (requires (uses svc) {system})
  (platform (uses soc)))
"
    )
}

#[test]
fn sr_h17_a_group_is_never_met_by_different_providers_nested_or_flat() {
    // `timer.a` offers the head with a horizon short of the requirement, `timer.b` the horizon alone: the only way to
    // meet both constraints splits the group, which the search never does. Flat, rule 13 refuses the split — a
    // missing fact while `timer.b` leaves the head undescribed, a conflict once it refuses it; nested, the group is
    // one requirement, and `timer.b`, stating a sub-fact and leaving the head undescribed, takes part (`SR-H18`).
    let flat = "(absolute-deadline true) (supported-horizon (at-least 10 s))";
    let nested = "(absolute-deadline (supported-horizon (at-least 10 s)))";
    for (case, service, b, verdict) in [
        (
            "flat, the head undescribed at timer.b",
            flat,
            "",
            Verdict::MissingFact,
        ),
        (
            "flat, the head refused at timer.b",
            flat,
            "(absolute-deadline false)",
            Verdict::InfeasibleConfiguration,
        ),
        (
            "nested, the head undescribed at timer.b",
            nested,
            "",
            Verdict::MissingFact,
        ),
        (
            "nested, the head refused at timer.b",
            nested,
            "(absolute-deadline false)",
            Verdict::InfeasibleConfiguration,
        ),
    ] {
        let (got, _, messages) = refusal(run(
            &groups(service, "", b),
            "s",
            &Policy::default(),
            Limits::DEFAULT,
        ));
        assert_eq!(got, verdict, "{case}: {messages:?}");
    }
}

#[test]
fn sr_h17_one_side_s_functions_may_be_met_apart_and_two_requesters_are_two() {
    // Decided by `M3.4.9`: on one side, the facts of one function are met at one provider (rule 13), a group's among
    // them; the facts of two functions may be met at two. And a system's own level and the service its task uses
    // are two requesters (model §8 rule 1), each bound on its own: a sub-fact written at the system's level joins no
    // head written by the service.
    let two_functions = resolved(run(
        &groups("(absolute-deadline true) (observable-output true)", "", ""),
        "s",
        &Policy::default(),
        Limits::DEFAULT,
    ));
    assert_eq!(
        bound(&two_functions),
        [
            "svc absolute-deadline timer.a",
            "svc observable-output console.uart"
        ]
    );
    let two_requesters = resolved(run(
        &groups(
            "(absolute-deadline true)",
            "(supported-horizon (at-least 10 s))",
            "",
        ),
        "s",
        &Policy::default(),
        Limits::DEFAULT,
    ));
    assert_eq!(
        bound(&two_requesters),
        [
            "s supported-horizon timer.b",
            "svc absolute-deadline timer.a"
        ]
    );
}
