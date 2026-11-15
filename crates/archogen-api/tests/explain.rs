//! The engine API's `explain` (leaf `M3.4.13.2`, API `1.5`): one requirement's account over a description read as
//! `resolve` reads it — each provider's answer, the provider bound or the search's own refusal — and every answer that
//! explains nothing — the check's, the request's — carrying its status and what to change.

use archogen_api::{
    explain_with, Answer, Explain, Explanation, Limits, NoModules, Request, Status,
};

const PLAIN: &str = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
";

fn ask(text: &str, explain: Explain<'_>) -> Explanation {
    explain_with(
        &Request {
            name: "description.eadl",
            text,
            profile: None,
            modules: &NoModules,
            target: None,
        },
        &explain,
        Limits::DEFAULT,
    )
}

fn fact(fact: &str) -> Explain<'_> {
    Explain {
        fact,
        ..Explain::default()
    }
}

#[test]
fn a_realized_requirement_is_explained_with_the_unchecked_plan() {
    let explanation = ask(PLAIN, fact("observable-output"));
    assert_eq!(explanation.response.status, Status::Ok);
    let plan = explanation.plan.as_ref().expect("the search's plan");
    assert!(plan
        .text
        .contains("(bind console.write observable-output console.uart)"));
    let [account] = explanation.accounts.as_slice() else {
        panic!("one account: {:?}", explanation.accounts);
    };
    assert_eq!(account.requester, "console.write");
    assert_eq!(account.fact, "observable-output");
    let written: Vec<&str> = account
        .clauses
        .iter()
        .map(|span| explanation.response.sources.snippet(*span))
        .collect();
    assert_eq!(written, ["observable-output"]);
    assert_eq!(
        account.answers,
        [
            ("console.uart".to_owned(), Answer::Satisfied),
            ("soc".to_owned(), Answer::Undescribed),
        ]
    );
    assert_eq!(account.bound.as_deref(), Some("console.uart"));
    assert!(account.instead.is_empty());
    assert!(account.unmet.is_none());
}

#[test]
fn an_unmet_requirement_carries_the_search_s_refusal() {
    let text = PLAIN.replace("(observable-output true)", "(observable-output false)");
    let explanation = ask(&text, fact("observable-output"));
    assert_eq!(explanation.response.status, Status::InfeasibleConfiguration);
    assert!(explanation.plan.is_none());
    let [account] = explanation.accounts.as_slice() else {
        panic!("one account: {:?}", explanation.accounts);
    };
    assert_eq!(
        account.answers[0],
        ("console.uart".to_owned(), Answer::Refused)
    );
    let unmet = account.unmet.as_ref().expect("the search's own diagnostic");
    assert_eq!(unmet.code, "infeasible-configuration");
    assert_eq!(
        explanation.response.diagnostics,
        std::slice::from_ref(unmet)
    );
    assert_eq!(account.bound, None);
}

#[test]
fn a_fact_no_requester_requires_is_usage_listing_those_that_are() {
    let explanation = ask(PLAIN, fact("absolute-deadline"));
    assert_eq!(explanation.response.status, Status::Usage);
    assert!(explanation.accounts.is_empty());
    assert_eq!(
        explanation.response.notes,
        ["system `s` of description.eadl writes no requirement on `absolute-deadline`"]
    );
    assert_eq!(
        explanation.response.hint.as_deref(),
        Some("explain one it writes: `observable-output` by `console.write`")
    );
    let task = ask(
        PLAIN,
        Explain {
            task: Some("beat"),
            ..fact("observable-output")
        },
    );
    assert_eq!(task.response.status, Status::Usage);
    assert_eq!(
        task.response.notes,
        ["system `s` of description.eadl writes no requirement on `observable-output` by its task `beat`"]
    );
}

#[test]
fn a_requester_and_a_task_together_are_usage() {
    let explanation = ask(
        PLAIN,
        Explain {
            requester: Some("console.write"),
            task: Some("beat"),
            ..fact("observable-output")
        },
    );
    assert_eq!(explanation.response.status, Status::Usage);
    assert_eq!(
        explanation.response.hint.as_deref(),
        Some("name the requester or the task, not both")
    );
}

#[test]
fn a_description_the_check_refuses_is_answered_by_the_check() {
    let text = PLAIN.replace(
        "(defblock console.uart (offers",
        "(defblock console.uart (absent observable-output) (offers",
    );
    let explanation = ask(&text, fact("observable-output"));
    assert_eq!(explanation.response.status, Status::InvalidDescription);
    assert!(
        explanation.response.judged.is_some(),
        "the check's own response"
    );
    assert!(explanation.accounts.is_empty());
    assert!(explanation.plan.is_none());
}
