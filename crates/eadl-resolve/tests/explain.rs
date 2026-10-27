//! One requirement explained (leaf `M3.4.13.1`): the search's own run, its pairs read once, giving for a requester's
//! requirements on one fact each hardware provider's answer, the search's diagnostic when none meets it, and with a
//! plan the provider bound and the joint rules' refusal of each other one in its place. Expectations are written out
//! literally — providers, answers, codes — never computed by the code under test.

use eadl_front::{read, Form, SourceMap, Verdict};
use eadl_model::check::shipped_registry;
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::relation::Verdict as Answer;
use eadl_resolve::search::{
    explain, Account, Asked, Explanation, Limits, Policy, Search, Select, Who,
};
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

/// The explanation of `fact` for `select` in `text`'s system `name`, under `level`.
fn asked_at(text: &str, name: &str, level: &str, fact: &str, select: Select<'_>) -> Explanation {
    let decls = forms(text);
    let system = decls
        .iter()
        .find(|f| {
            f.head() == Some("defsystem")
                && f.items().get(1).and_then(Form::as_symbol) == Some(name)
        })
        .expect("the system")
        .clone();
    explain(
        &decls,
        &system,
        level,
        &vocabulary(),
        &Policy::default(),
        Limits::DEFAULT,
        Asked { fact, select },
    )
}

/// The same, under the profile's caller level.
fn asked(text: &str, name: &str, fact: &str, select: Select<'_>) -> Explanation {
    asked_at(text, name, RT_STATIC_UP_V1.caller_level, fact, select)
}

fn named(name: &str) -> Who {
    Who::Named(name.to_owned())
}

/// Each provider's answer, as `provider answer` pairs.
fn answers(account: &Account) -> Vec<(&str, Answer)> {
    account
        .answers
        .iter()
        .map(|(provider, answer)| (provider.as_str(), *answer))
        .collect()
}

/// Each other provider meeting it, and the code of the joint rules' refusal with it in the bound one's place.
fn instead(account: &Account) -> Vec<(&str, Option<&'static str>)> {
    account
        .instead
        .iter()
        .map(|(provider, refusal)| (provider.as_str(), refusal.as_ref().map(|d| d.code)))
        .collect()
}

/// Two services each programming an absolute deadline, and two comparators of one channel each: the plan binds the
/// engine to `timer.a` and the watchdog to `timer.b`, the walk's second assignment.
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
fn a_realized_requirement_names_its_provider_and_why_each_other_is_not_it() {
    let explanation = asked(
        TWO_COMPARATORS,
        "two-alarms",
        "absolute-deadline",
        Select::Every,
    );
    assert!(matches!(explanation.search, Search::Resolved(_)));
    assert!(explanation.judged);
    assert_eq!(
        explanation.required,
        [
            (named("alarm.engine"), "absolute-deadline".to_owned()),
            (named("alarm.watchdog"), "absolute-deadline".to_owned()),
        ]
    );
    let [engine, watchdog] = explanation.accounts.as_slice() else {
        panic!("two accounts: {:?}", explanation.accounts);
    };
    assert_eq!(engine.requester, named("alarm.engine"));
    assert_eq!(engine.clauses.len(), 1);
    // `soc` composes the comparators and states no deadline of its own.
    assert_eq!(
        answers(engine),
        [
            ("timer.a", Answer::Satisfied),
            ("timer.b", Answer::Satisfied),
            ("soc", Answer::Undescribed),
        ]
    );
    assert!(engine.unmet.is_none());
    assert_eq!(engine.bound.as_deref(), Some("timer.a"));
    // `timer.b` in the engine's place puts both services on its one channel.
    assert_eq!(
        instead(engine),
        [("timer.b", Some("infeasible-configuration"))]
    );
    assert_eq!(watchdog.bound.as_deref(), Some("timer.b"));
    // `timer.a` in the watchdog's place is the walk's first assignment, refused for the same channel.
    assert_eq!(
        instead(watchdog),
        [("timer.a", Some("infeasible-configuration"))]
    );
    let refusal = watchdog.instead[0].1.as_ref().expect("a refusal");
    assert!(refusal.message.contains("timer.a"), "{}", refusal.message);
}

#[test]
fn an_other_provider_the_joint_rules_accept_too_is_second_by_the_policy_s_order() {
    let text = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defblock console.spare (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart console.spare)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
";
    let explanation = asked(text, "s", "observable-output", Select::Every);
    let [account] = explanation.accounts.as_slice() else {
        panic!("one account: {:?}", explanation.accounts);
    };
    assert_eq!(account.bound.as_deref(), Some("console.uart"));
    assert_eq!(instead(account), [("console.spare", None)]);
}

#[test]
fn the_requesters_are_selected_by_name_or_by_task() {
    let watchdog = asked(
        TWO_COMPARATORS,
        "two-alarms",
        "absolute-deadline",
        Select::Requester("alarm.watchdog"),
    );
    let [account] = watchdog.accounts.as_slice() else {
        panic!("one account: {:?}", watchdog.accounts);
    };
    assert_eq!(account.requester, named("alarm.watchdog"));
    // A task that requires the fact only through a service it uses writes no requirement on it: no account, and every
    // requirement the system's requesters write is listed, for the caller to say so.
    let task = asked(
        TWO_COMPARATORS,
        "two-alarms",
        "absolute-deadline",
        Select::Task("engine"),
    );
    assert!(task.judged);
    assert!(task.accounts.is_empty());
    assert_eq!(task.required.len(), 2);
    // A fact nothing requires: no account either.
    let none = asked(
        TWO_COMPARATORS,
        "two-alarms",
        "observable-output",
        Select::Every,
    );
    assert!(none.judged);
    assert!(none.accounts.is_empty());
}

#[test]
fn an_unmet_requirement_gives_each_provider_s_answer_and_the_search_s_diagnostic() {
    let system = "(defservice console.write (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
";
    let soc = "(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))";
    for (case, uart, answer, code) in [
        (
            "refused",
            "(defblock console.uart (offers (observable-output false) (reachable-at-privilege machine)))",
            Answer::Refused,
            "infeasible-configuration",
        ),
        (
            "absent",
            "(defblock console.uart (offers (reachable-at-privilege machine)) (absent observable-output))",
            Answer::Absent,
            "infeasible-configuration",
        ),
        (
            "undescribed",
            "(defblock console.uart (offers (reachable-at-privilege machine)))",
            Answer::Undescribed,
            "missing-fact",
        ),
    ] {
        let text = format!("(eadl-version eadl/1)\n{uart}\n{soc}\n{system}");
        let explanation = asked(&text, "s", "observable-output", Select::Requester("console.write"));
        let [account] = explanation.accounts.as_slice() else {
            panic!("{case}: one account: {:?}", explanation.accounts);
        };
        assert_eq!(
            answers(account),
            [("console.uart", answer), ("soc", Answer::Undescribed)],
            "{case}"
        );
        let unmet = account.unmet.as_ref().expect("no provider meets it");
        assert_eq!(unmet.code, code, "{case}");
        assert!(unmet.message.contains("`console.write` requires `observable-output`"), "{case}");
        assert_eq!(account.bound, None, "{case}");
        assert!(account.instead.is_empty(), "{case}");
        match &explanation.search {
            Search::Refused { diagnostics, .. } => assert_eq!(&diagnostics[0], unmet, "{case}"),
            other => panic!("{case}: not refused: {other:?}"),
        }
    }
}

#[test]
fn a_requirement_met_alone_waits_on_another_that_stops_the_search() {
    // `periodic-three`'s monotonic time asks a coherent observation, which its counter offers, and a horizon its counter
    // cannot be judged on: the horizon derives from `counter-modulus` and `tick-rate` and reads `wrap-behavior`, and the
    // example states `counter-width` — the migration `M3.4.7` owns. So no plan is written, and the observation is met
    // alone, waiting on the horizon and on the constraints only the catalog's providers meet (`M3.7`).
    let text = std::fs::read_to_string(root().join("examples/periodic-three/system.eadl"))
        .expect("the example");
    let monotonic = Select::Requester("time.monotonic");
    let coherent = asked(&text, "periodic-three", "observation-coherent", monotonic);
    let [account] = coherent.accounts.as_slice() else {
        panic!("one account: {:?}", coherent.accounts);
    };
    // Its constraint and its `needs`, one pair.
    assert_eq!(account.clauses.len(), 2);
    assert_eq!(
        answers(account),
        [
            ("timer.counter", Answer::Satisfied),
            ("console.uart", Answer::Undescribed),
            ("soc.playground", Answer::Undescribed),
        ]
    );
    assert!(account.unmet.is_none(), "it is met alone");
    assert_eq!(account.bound, None, "no plan");
    let horizon = asked(&text, "periodic-three", "unambiguous-horizon", monotonic);
    let [unmet] = horizon.accounts.as_slice() else {
        panic!("one account: {:?}", horizon.accounts);
    };
    assert_eq!(
        answers(unmet),
        [
            ("timer.counter", Answer::Undescribed),
            ("console.uart", Answer::Undescribed),
            ("soc.playground", Answer::Undescribed),
        ]
    );
    let diagnostic = unmet.unmet.as_ref().expect("no provider meets it");
    assert_eq!(diagnostic.code, "missing-fact");
    match &coherent.search {
        Search::Refused {
            verdict,
            diagnostics,
        } => {
            assert_eq!(*verdict, Verdict::MissingFact);
            assert!(diagnostics.contains(diagnostic), "{diagnostics:?}");
            assert!(
                diagnostics
                    .iter()
                    .all(|d| !d.message.contains("requires `observation-coherent`")),
                "{diagnostics:?}"
            );
        }
        other => panic!("not refused: {other:?}"),
    }
}

#[test]
fn a_search_stopped_before_its_requirements_judges_none() {
    let explanation = asked_at(
        TWO_COMPARATORS,
        "two-alarms",
        "no-such-level",
        "absolute-deadline",
        Select::Every,
    );
    assert!(!explanation.judged);
    assert!(explanation.required.is_empty());
    assert!(explanation.accounts.is_empty());
    match explanation.search {
        Search::Refused { verdict, .. } => assert_eq!(verdict, Verdict::ToolFailure),
        other => panic!("not refused: {other:?}"),
    }
}
