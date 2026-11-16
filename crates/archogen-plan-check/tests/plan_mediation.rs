//! `SR-H8`, mediation (`docs/specs/adapters/decision_engine-adapters.md` §6; leaf `M3.2.6.3`): a kind writing
//! `(mediates true)` is offered only to a requester that wrote `(or-through-mediation allowed)`, bare or under `exactly`
//! — the relation's one statement, read as `docs/decisions/decision_substitutability-relation.md` §3 rule 6 says. `/1`'s
//! catalog holds no mediating kind, so the checker is judged against a test catalog that holds one, through
//! [`with_table`], and holds the gate both ways: a plan binding an instance of it is refused for a requester that did not
//! allow mediation and accepted for one that did. Every plan is written by hand from the record's §5, and judged by
//! every check in order, [`plan`].

use std::sync::LazyLock;

use archogen_outcome::record::Severity;
use archogen_outcome::Status;
use archogen_plan_check::adapters::{with_table, Kind, ADAPTERS};
use archogen_plan_check::assignment::program_digest;
use archogen_plan_check::check::{plan, Verdict, PLAN_RECORD, PLAN_UNSATISFIED};
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;

/// The record's name in every finding's span.
const RECORD: &str = "plan.eadl";

/// The description's file, the root module `app`.
const SOURCE: &str = "app.eadl";

/// The mediating kind of the test catalog.
const MEDIATOR: &str = "test-mediator";

/// The test catalog (record §6): `/1`'s kinds and [`MEDIATOR`], the epoch extender's contract written `(mediates
/// true)`, so the one clause the gate reads is all that tells the two apart. Like any kind's instance, its own owns no
/// region (record §4, rule 5).
static CATALOG: LazyLock<Vec<Kind>> = LazyLock::new(|| {
    let extender = ADAPTERS
        .iter()
        .find(|k| k.name == "epoch-extender")
        .expect("`/1`'s epoch extender");
    let mut table = ADAPTERS.to_vec();
    table.push(Kind {
        name: MEDIATOR,
        mediates: true,
        ..*extender
    });
    table
});

/// A description, its declarations the root module `app`'s, so their names are unqualified.
struct Description {
    sources: SourceMap,
    program: Program,
}

/// A 16-bit modular counter, whose own horizon, `65535 / 1 MHz`, falls short of the minute a monotonic time service
/// needs, so an instance over it meets the service where the counter does not; the service writes `statement` beside, its
/// word on mediation or nothing. The service's requirement stands on line 6.
fn describe(statement: &str) -> Description {
    let modules = MemoryModules::new().with(
        "app",
        &format!(
            "(defmodule app
  (version 1 0)
  (defblock timer (offers (relative-delay true) (counter-modulus 65536) (tick-rate 1 MHz) (observation-coherent true)
    (wrap-behavior modular) (channels 1 tick) (reachable-at-privilege machine)))
  (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer)))
  (defservice time.monotonic (requires (unambiguous-horizon (at-least 60 s)) {statement}))
  (defsystem s (requires (uses time.monotonic)) (platform (uses soc))))"
        ),
    );
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, &modules, "app");
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    Description { sources, program }
}

/// The record binding the service's horizon through the shared instance of `kind` over `timer`, and the instance's five
/// facts beneath to `timer` — the extender's `needs` and its constraint's head, each once — in the canonical order of
/// record §5: every name, then every instance, each then by fact.
fn record(d: &Description, kind: &str) -> String {
    let digest = program_digest(&d.program);
    let instance = format!("(adapter {kind} timer)");
    format!(
        "(format \"archogen-assignment/2\")
(profile rt-static-up-v1)
(system s)
(program \"{digest}\")
(adapters adapters/1)
(bind time.monotonic unambiguous-horizon {instance})
(bind {instance} counter-modulus timer)
(bind {instance} observation-coherent timer)
(bind {instance} relative-delay timer)
(bind {instance} tick-rate timer)
(bind {instance} wrap-behavior timer)
"
    )
}

/// Every check over `record`, in order, against the test catalog.
fn judged(d: &Description, record: &str) -> Result<(), Verdict> {
    with_table(&CATALOG, || {
        plan(&d.sources, &d.program, &RT_STATIC_UP_V1, RECORD, record)
    })
}

#[test]
fn a_mediating_instance_is_refused_for_a_requester_that_did_not_allow_mediation() {
    for statement in [
        "",
        "(or-through-mediation forbidden)",
        "(or-through-mediation (exactly forbidden))",
    ] {
        let d = describe(statement);
        let verdict = judged(&d, &record(&d, MEDIATOR)).expect_err(statement);
        // A corrupted plan, refused by check (4) alone: one finding, the gate's.
        assert_eq!(
            verdict.status,
            Status::ToolFailure,
            "{statement}: {verdict:#?}"
        );
        assert_eq!(verdict.findings.len(), 1, "{statement}: {verdict:#?}");
        let finding = &verdict.findings[0];
        assert_eq!(finding.code, PLAN_UNSATISFIED, "{statement}: {finding:#?}");
        assert_eq!(finding.severity, Severity::Error);
        assert_eq!(
            finding.message,
            "`time.monotonic` is bound for `unambiguous-horizon` to `(adapter test-mediator timer)`, whose kind \
             `test-mediator` mediates: it is offered only to a requester that wrote `(or-through-mediation allowed)` \
             (the adapters record §6)",
            "{statement}"
        );
        assert_eq!(
            finding.repair,
            "resolve again: a mediating adapter serves a requester that allows mediation"
        );
        // At the requirement, then the binding and the device the instance programs.
        assert_eq!(
            (finding.primary.source.as_str(), finding.primary.line),
            (SOURCE, 6),
            "{finding:#?}"
        );
        let secondary: Vec<_> = finding
            .secondary
            .iter()
            .map(|l| (l.source.as_str(), l.line))
            .collect();
        assert_eq!(secondary, [(RECORD, 6), (SOURCE, 3)], "{finding:#?}");
    }
}

#[test]
fn a_mediating_instance_is_accepted_for_a_requester_that_allowed_mediation_bare_or_under_exactly() {
    for statement in [
        "(or-through-mediation allowed)",
        "(or-through-mediation (exactly allowed))",
    ] {
        let d = describe(statement);
        assert_eq!(judged(&d, &record(&d, MEDIATOR)), Ok(()), "{statement}");
    }
}

#[test]
fn its_twin_that_does_not_mediate_is_accepted_unallowed_so_the_refusal_is_the_gate_s() {
    // The same plan through `/1`'s epoch extender, which differs from the mediator only in not mediating, is accepted
    // with no word on mediation: the refusal above is the gate's, and the test catalog judges `/1`'s kinds as `/1` does.
    let d = describe("");
    assert_eq!(judged(&d, &record(&d, "epoch-extender")), Ok(()));
    assert_eq!(
        plan(
            &d.sources,
            &d.program,
            &RT_STATIC_UP_V1,
            RECORD,
            &record(&d, "epoch-extender")
        ),
        Ok(())
    );
}

#[test]
fn outside_the_test_catalog_the_checker_s_table_is_1_s_and_holds_no_mediator() {
    // The shipped table unchanged: the mediator's plan, allowed, is a kind `/1` does not hold, refused by check (1).
    let d = describe("(or-through-mediation allowed)");
    let verdict = plan(
        &d.sources,
        &d.program,
        &RT_STATIC_UP_V1,
        RECORD,
        &record(&d, MEDIATOR),
    )
    .expect_err("`/1` holds no mediating kind");
    assert_eq!(verdict.status, Status::ToolFailure, "{verdict:#?}");
    assert_eq!(verdict.findings.len(), 1, "{verdict:#?}");
    assert_eq!(verdict.findings[0].code, PLAN_RECORD);
    assert!(
        verdict.findings[0].message.contains(
            "names the adapter kind `test-mediator`, which this checker's table, `adapters/1`, does not hold"
        ),
        "{verdict:#?}"
    );
    assert!(ADAPTERS.iter().all(|k| !k.mediates));
}
