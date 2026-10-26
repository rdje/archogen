//! Check (5), the joint rules, as the fenced context of `M3.5.5.8` reads them from `docs/semantics/model.md` §8: per
//! rule, a plan accepted and its corruptions refused. Each plan goes through checks (1) to (4) first, which must
//! accept it — a plan they refuse is no case of check (5) — then through [`joint_fenced`]; the few corruptions check
//! (3) refuses first are run apart, to hold check (5)'s own reading of them.

use std::collections::BTreeMap;

use archogen_outcome::record::{Finding, Severity};
use archogen_outcome::Status;
use archogen_plan_check::assignment::{program_digest, Assignment, Requester};
use archogen_plan_check::check::{
    bindings, description, joint, record, Verdict, PLAN_DESCRIPTION, PLAN_JOINT,
};
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Program, SourceMap};
use eadl_model::profile::{Profile, RT_STATIC_UP_V1};

/// The record's name in every finding's span.
const RECORD: &str = "plan.eadl";

/// The description's file, the root module `app`.
const SOURCE: &str = "app.eadl";

/// A description, its declarations the root module `app`'s, so their names are unqualified.
struct Description {
    sources: SourceMap,
    program: Program,
}

fn describe(declarations: &str) -> Description {
    let modules = MemoryModules::new().with(
        "app",
        &format!("(defmodule app\n  (version 1 0)\n{declarations})"),
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

fn named(name: &str) -> Requester {
    Requester::Named(name.to_owned())
}

fn task(system: &str, task: &str) -> Requester {
    Requester::Task {
        system: system.to_owned(),
        task: task.to_owned(),
    }
}

/// One binding: requester, fact, provider.
type Bind = (Requester, &'static str, &'static str);

impl Description {
    /// The record a search would write for the system `s`: `binds` its bindings, `placement` its placement.
    fn plan(&self, placement: Option<(&str, &str)>, binds: &[Bind]) -> Assignment {
        Assignment {
            profile: RT_STATIC_UP_V1.id.to_owned(),
            system: "s".to_owned(),
            program: program_digest(&self.program),
            placement: placement.map(|(p, r)| (p.to_owned(), r.to_owned())),
            bindings: binds
                .iter()
                .map(|(r, f, p)| ((r.clone(), (*f).to_owned()), (*p).to_owned()))
                .collect::<BTreeMap<_, _>>(),
        }
    }

    /// Checks (1) to (4) under `profile`, each of which must accept, then check (5)'s verdict.
    fn joint_under(&self, profile: &Profile, assignment: &Assignment) -> Result<(), Verdict> {
        let text = assignment.to_canonical();
        if let Err(verdict) = record(&self.program, profile, RECORD, &text) {
            panic!("check (1) refuses the plan:\n{text}{verdict:#?}");
        }
        if let Err(verdict) = description(&self.sources, &self.program, profile) {
            panic!("check (2) refuses the description: {verdict:#?}");
        }
        if let Err(verdict) = bindings(&self.sources, &self.program, RECORD, &text, assignment) {
            panic!("checks (3) and (4) refuse the plan:\n{text}{verdict:#?}");
        }
        joint(
            &self.sources,
            &self.program,
            profile,
            RECORD,
            &text,
            assignment,
        )
    }

    fn joint(&self, assignment: &Assignment) -> Result<(), Verdict> {
        self.joint_under(&RT_STATIC_UP_V1, assignment)
    }

    /// `binds` as a plan with no placement, judged.
    fn bound(&self, binds: &[Bind]) -> Result<(), Verdict> {
        self.joint(&self.plan(None, binds))
    }
}

/// The findings of a plan check (5) refuses: `tool-failure`, each `plan-joint`, an error with a place and a repair, and
/// each naming `(rule N)` — every one of them when `every`, else at least one.
fn refusal(verdict: Result<(), Verdict>, rule: u32, every: bool) -> Vec<Finding> {
    let verdict = verdict.expect_err("check (5) refuses the plan");
    assert_eq!(verdict.status, Status::ToolFailure, "{verdict:#?}");
    assert!(!verdict.findings.is_empty());
    let naming = |f: &Finding| f.message.contains(&format!("(rule {rule})"));
    for finding in &verdict.findings {
        assert_eq!(finding.code, PLAN_JOINT, "{finding:#?}");
        assert_eq!(finding.severity, Severity::Error);
        assert!(!finding.repair.is_empty(), "{finding:#?}");
        assert!(finding.primary.line >= 1 && !finding.primary.message.is_empty());
        assert!(!every || naming(finding), "{finding:#?}");
    }
    assert!(
        verdict.findings.iter().any(naming),
        "{:#?}",
        verdict.findings
    );
    verdict.findings
}

/// The messages of a plan check (5) refuses, every finding under `(rule N)`.
fn refused(verdict: Result<(), Verdict>, rule: u32) -> String {
    messages(&refusal(verdict, rule, true))
}

fn messages(findings: &[Finding]) -> String {
    findings
        .iter()
        .map(|f| f.message.as_str())
        .collect::<Vec<_>>()
        .join("\n")
}

/// The messages of a description check (5) refuses whatever the assignment: `status`, every finding `plan-description`
/// naming `(rule N)` and pointing into the description.
fn refused_description(verdict: Result<(), Verdict>, status: Status, rule: u32) -> String {
    let verdict = verdict.expect_err("check (5) refuses the description");
    assert_eq!(verdict.status, status, "{verdict:#?}");
    for finding in &verdict.findings {
        assert_eq!(finding.code, PLAN_DESCRIPTION, "{finding:#?}");
        assert_eq!(finding.severity, Severity::Error);
        assert!(
            finding.message.contains(&format!("(rule {rule})")),
            "{finding:#?}"
        );
        assert!(!finding.repair.is_empty(), "{finding:#?}");
        assert_eq!(finding.primary.source, SOURCE, "{finding:#?}");
        assert!(finding.primary.line > 2, "{finding:#?}");
    }
    messages(&verdict.findings)
}

/// The record line of the binding of `requester`'s `fact`, 1-based.
fn line_of(plan: &Assignment, requester: &Requester, fact: &str) -> u32 {
    let clause = format!("(bind {} {fact} ", requester.canonical());
    let index = plan
        .to_canonical()
        .lines()
        .position(|line| line.starts_with(&clause))
        .expect("the binding is in the record");
    u32::try_from(index + 1).expect("a short record")
}

// ------------------------------------------------------------------------------------------------------------------
// Rules 1 and 7: requesters, and the system's hardware.

/// `reference.md`'s `plan-joint` row, a task needing the console itself beside it.
const CONSOLE: &str = "
  (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defblock spare.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s
    (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
    (task log (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (needs observable-output))
    (requires (uses console.write))
    (platform (uses soc)))";

#[test]
fn rule_7_each_requester_the_system_serves_bound_to_its_hardware_holds() {
    let d = describe(CONSOLE);
    // A service and a task, two requesters of one shared fact at one provider: a shared fact takes no channel (rule 2).
    assert_eq!(
        d.bound(&[
            (named("console.write"), "observable-output", "console.uart"),
            (task("s", "log"), "observable-output", "console.uart"),
        ]),
        Ok(())
    );
}

#[test]
fn rule_7_a_requester_bound_outside_the_system_s_hardware_is_refused_at_its_binding() {
    let d = describe(CONSOLE);
    for (stray, other) in [
        (named("console.write"), task("s", "log")),
        (task("s", "log"), named("console.write")),
    ] {
        let plan = d.plan(
            None,
            &[
                (stray.clone(), "observable-output", "spare.uart"),
                (other, "observable-output", "console.uart"),
            ],
        );
        let findings = refusal(d.joint(&plan), 7, true);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        let finding = &findings[0];
        assert!(
            finding
                .message
                .contains(&format!("`{}`", stray.canonical()))
                && finding.message.contains("`spare.uart`")
                && finding.message.contains("hardware of `s`"),
            "{finding:#?}"
        );
        assert_eq!(finding.primary.source, RECORD);
        assert_eq!(
            finding.primary.line,
            line_of(&plan, &stray, "observable-output")
        );
        // The provider, where the requester requires the fact, and the system, in the description.
        assert_eq!(finding.secondary.len(), 3, "{finding:#?}");
        assert!(finding.secondary.iter().all(|l| l.source == SOURCE));
    }
}

#[test]
fn rule_7_hardware_is_composed_at_any_depth_and_never_through_refines() {
    // `soc` composes `bus.bridge` inside `requires`, which composes `console.uart` in a `uses` clause of its own;
    // `soc` refines `soc.base`, whose `spare.uart` is no hardware of `s`.
    let d = describe(
        "
      (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
      (defblock spare.uart (offers (observable-output true) (reachable-at-privilege machine)))
      (defblock bus.bridge (offers (bus-width 32 bit) (reachable-at-privilege machine)) (uses console.uart))
      (defplatform soc.base (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses spare.uart)))
      (defplatform soc (refines soc.base) (offers (core-count 1 tick) (reachable-at-privilege machine))
        (requires (uses bus.bridge)))
      (defservice console.write (requires (needs observable-output)))
      (defsystem s (requires (uses console.write)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[(named("console.write"), "observable-output", "console.uart")]),
        Ok(())
    );
    let message = refused(
        d.bound(&[(named("console.write"), "observable-output", "spare.uart")]),
        7,
    );
    assert!(message.contains("`spare.uart`"), "{message}");
}

#[test]
fn rule_7_a_block_a_service_names_is_reached_but_is_no_hardware() {
    // `those` in rule 7 are the blocks and platforms found: a service's `needs` of a block makes it a requester the
    // system serves, never hardware.
    let d = describe(
        "
      (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
      (defblock spare.uart (offers (observable-output true) (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
      (defservice console.write (requires (needs observable-output spare.uart)))
      (defsystem s (requires (uses console.write)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[(named("console.write"), "observable-output", "console.uart")]),
        Ok(())
    );
    refused(
        d.bound(&[(named("console.write"), "observable-output", "spare.uart")]),
        7,
    );
}

#[test]
fn rules_1_and_7_a_binding_check_three_refuses_first_is_refused_by_check_five_s_own_reading_too() {
    let d = describe(CONSOLE);
    let held = (task("s", "log"), "observable-output", "console.uart");
    let cases: [(Bind, u32, &str); 6] = [
        (
            (
                named("console.nowhere"),
                "observable-output",
                "console.uart",
            ),
            1,
            "names no declaration",
        ),
        (
            (task("s", "ghost"), "observable-output", "console.uart"),
            1,
            "no task `ghost`",
        ),
        (
            (task("t", "beat"), "observable-output", "console.uart"),
            7,
            "an assignment is one system's",
        ),
        (
            (named("spare.uart"), "observable-output", "console.uart"),
            7,
            "no requester `s` serves",
        ),
        (
            (named("console.write"), "console.uart", "console.uart"),
            1,
            "no fact of the vocabulary",
        ),
        (
            (named("console.write"), "observable-output", "console.write"),
            7,
            "no block or platform",
        ),
    ];
    for (corrupt, rule, words) in cases {
        let plan = d.plan(None, &[held.clone(), corrupt]);
        let text = plan.to_canonical();
        assert!(record(&d.program, &RT_STATIC_UP_V1, RECORD, &text).is_ok());
        assert_eq!(
            description(&d.sources, &d.program, &RT_STATIC_UP_V1),
            Ok(())
        );
        assert!(
            bindings(&d.sources, &d.program, RECORD, &text, &plan).is_err(),
            "check (3) refuses it first:\n{text}"
        );
        let findings = refusal(
            joint(
                &d.sources,
                &d.program,
                &RT_STATIC_UP_V1,
                RECORD,
                &text,
                &plan,
            ),
            rule,
            false,
        );
        assert!(messages(&findings).contains(words), "{findings:#?}");
    }
}

// ------------------------------------------------------------------------------------------------------------------
// Rules 2 to 4: consumed facts, channels, and a service several users reach.

/// A counter offering both consumed functions with `channels` as `pool` writes it, a lazy one whose `relative-delay`
/// is `false`, the services that use them, and the system `system`.
fn timers(pool: &str, system: &str) -> Description {
    describe(&format!(
        "
      (defblock timer.delay (offers (relative-delay true) (absolute-deadline true) (supported-horizon 3600 s) {pool}
        (reachable-at-privilege machine)))
      (defblock timer.lazy (offers (relative-delay false) (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.delay timer.lazy)))
      (defservice time.sleep (requires (needs relative-delay)))
      (defservice time.alarm (requires (relative-delay true)))
      (defservice time.deadline (requires (absolute-deadline (supported-horizon (at-least 10 s)))))
      (defservice time.both (requires (needs relative-delay) (absolute-deadline true)))
      (defservice time.never (requires (relative-delay false)))
      (defservice time.none (requires (relative-delay (exactly false))))
      {system}"
    ))
}

const THREE: &str =
    "(defsystem s (requires (uses time.sleep time.alarm time.deadline)) (platform (uses soc)))";

fn three() -> [Bind; 3] {
    [
        (named("time.sleep"), "relative-delay", "timer.delay"),
        (named("time.alarm"), "relative-delay", "timer.delay"),
        (named("time.deadline"), "absolute-deadline", "timer.delay"),
    ]
}

#[test]
fn rule_3_requesters_taking_channels_number_at_most_the_provider_s_channels() {
    // `(needs f)`, `(f true)` and the group whose head `f` is each take a channel (rule 2).
    assert_eq!(timers("(channels 3)", THREE).bound(&three()), Ok(()));
    assert_eq!(timers("(channels 4)", THREE).bound(&three()), Ok(()));
    let d = timers("(channels 2)", THREE);
    let plan = d.plan(None, &three());
    let findings = refusal(d.joint(&plan), 3, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let message = &findings[0].message;
    for named in [
        "`timer.delay`",
        "3 requesters",
        "`time.sleep` through `relative-delay`",
        "`time.alarm` through `relative-delay`",
        "`time.deadline` through `absolute-deadline`",
        "offers 2 `channels`",
    ] {
        assert!(message.contains(named), "{named}: {message}");
    }
    // At the record's first binding of them; each requester's requirement and the provider in the description.
    assert_eq!(findings[0].primary.source, RECORD);
    assert_eq!(
        findings[0].primary.line,
        line_of(&plan, &named("time.alarm"), "relative-delay")
    );
    assert_eq!(findings[0].secondary.len(), 4, "{findings:#?}");
}

#[test]
fn rule_3_two_requesters_where_channels_is_undescribed_or_offered_bare_are_an_unknown() {
    let two = "(defsystem s (requires (uses time.sleep time.alarm)) (platform (uses soc)))";
    let binds = &three()[..2];
    for (pool, why) in [
        ("", "does not describe `channels`"),
        ("channels", "offers `channels` without a value"),
    ] {
        let message = refused(timers(pool, two).bound(binds), 3);
        assert!(
            message.contains(why) && message.contains("missing-fact"),
            "{message}"
        );
    }
    // One requester needs no count.
    let one = "(defsystem s (requires (uses time.sleep)) (platform (uses soc)))";
    assert_eq!(timers("", one).bound(&three()[..1]), Ok(()));
}

#[test]
fn rule_2_a_requirement_met_by_false_takes_no_channel() {
    // Two requesters of `(relative-delay false)` at a provider that describes no `channels`: neither takes one.
    let d = timers(
        "",
        "(defsystem s (requires (uses time.sleep time.never time.none)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[
            (named("time.sleep"), "relative-delay", "timer.delay"),
            (named("time.never"), "relative-delay", "timer.lazy"),
            (named("time.none"), "relative-delay", "timer.lazy"),
        ]),
        Ok(())
    );
}

#[test]
fn rule_3_counts_requesters_so_one_binding_both_consumed_facts_takes_one_channel() {
    let d = timers(
        "(channels 1)",
        "(defsystem s (requires (uses time.both)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[
            (named("time.both"), "absolute-deadline", "timer.delay"),
            (named("time.both"), "relative-delay", "timer.delay"),
        ]),
        Ok(())
    );
    // Beside another requester, the two consumed functions share one pool.
    let d = timers(
        "(channels 1)",
        "(defsystem s (requires (uses time.both time.sleep)) (platform (uses soc)))",
    );
    let message = refused(
        d.bound(&[
            (named("time.both"), "absolute-deadline", "timer.delay"),
            (named("time.both"), "relative-delay", "timer.delay"),
            (named("time.sleep"), "relative-delay", "timer.delay"),
        ]),
        3,
    );
    assert!(
        message.contains("2 requesters")
            && message.contains("`time.both` through `absolute-deadline`, `relative-delay`"),
        "{message}"
    );
}

#[test]
fn rule_4_a_service_several_users_reach_is_one_requester() {
    // Two tasks and the system reach `time.sleep`: one requester of one channel.
    let shared = "(defsystem s
        (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.sleep))
        (task b (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (uses time.sleep))
        (requires (uses time.sleep)) (platform (uses soc)))";
    assert_eq!(timers("(channels 1)", shared).bound(&three()[..1]), Ok(()));
    // A task programming the counter itself is a second requester.
    let direct = "(defsystem s
        (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses time.sleep))
        (task c (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (needs relative-delay))
        (requires (uses time.sleep)) (platform (uses soc)))";
    let message = refused(
        timers("(channels 1)", direct).bound(&[
            (named("time.sleep"), "relative-delay", "timer.delay"),
            (task("s", "c"), "relative-delay", "timer.delay"),
        ]),
        3,
    );
    assert!(
        message.contains("`(task s c)` through `relative-delay`")
            && message.contains("offers 1 `channels`"),
        "{message}"
    );
}

// ------------------------------------------------------------------------------------------------------------------
// Rule 5: ownership.

/// A counter and a console, each with the region `timer` and `uart` write, on a platform with a bus window over both
/// and a spare device; the system uses `uses`.
fn devices(timer: &str, uart: &str, uses: &str) -> Description {
    describe(&format!(
        "
      (defblock timer.a (offers (counter-width 32 bit) (reachable-at-privilege machine) {timer}))
      (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine) {uart}))
      (defblock spare.dev (offers (debug-port true) (region regs (base 0x1000_0000) (size 64 KiB))))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)
          (region bus (base 0x1000_0000) (size 1 MiB)))
        (requires (uses timer.a console.uart spare.dev)))
      (defservice time.monotonic (requires (counter-width 32 bit)))
      (defservice console.write (requires (needs observable-output)))
      (defservice sched.core (requires (core-count 1 tick)))
      (defsystem s (requires (uses {uses})) (platform (uses soc)))"
    ))
}

fn two_devices() -> [Bind; 2] {
    [
        (named("time.monotonic"), "counter-width", "timer.a"),
        (named("console.write"), "observable-output", "console.uart"),
    ]
}

#[test]
fn rule_5_two_bound_providers_whose_bytes_do_not_overlap_hold() {
    // Adjacent ranges; and `spare.dev`, bound to nothing, owns nothing though its bytes cover both.
    let d = devices(
        "(region regs (base 0x1000_0000) (size 4 KiB))",
        "(region regs (base 0x1000_1000) (size 4 KiB))",
        "time.monotonic console.write",
    );
    assert_eq!(d.bound(&two_devices()), Ok(()));
    // A region without size owns its base byte: one lying wholly below it overlaps nothing.
    let d = devices(
        "(region regs (base 0x1000_2000))",
        "(region regs (base 0x1000_1000) (size 4 KiB))",
        "time.monotonic console.write",
    );
    assert_eq!(d.bound(&two_devices()), Ok(()));
}

#[test]
fn rule_5_two_bound_providers_owning_one_byte_are_a_conflict_naming_both_ranges() {
    for (timer, uart, range) in [
        (
            "(region regs (base 0x1000_0000) (size 4 KiB))",
            "(region regs (base 0x1000_0800) (size 4 KiB))",
            "[0x10000800, 0x10001800)",
        ),
        // A reserved region is owned too; and a region without size owns its base byte, here the other's first —
        // an overlap, not the unknown of the bytes above it.
        (
            "(region fw (base 0x1000_0000) (size 4 KiB) (reserved true))",
            "(region regs (base 0x1000_0000))",
            "[0x10000000, …) with no size",
        ),
    ] {
        let d = devices(timer, uart, "time.monotonic console.write");
        let plan = d.plan(None, &two_devices());
        let findings = refusal(d.joint(&plan), 5, true);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        let message = &findings[0].message;
        assert!(
            message.contains("`console.uart`")
                && message.contains("`timer.a`")
                && message.contains("[0x10000000, 0x10001000)")
                && message.contains(range)
                && message.contains("own the same bytes"),
            "{message}"
        );
        assert_eq!(findings[0].primary.source, RECORD);
        assert!(findings[0].secondary.iter().any(|l| l.source == SOURCE));
    }
}

#[test]
fn rule_5_a_region_without_size_below_another_bound_provider_s_bytes_is_an_unknown() {
    let d = devices(
        "(region regs (base 0x1000_0000))",
        "(region regs (base 0x1000_1000) (size 4 KiB))",
        "time.monotonic console.write",
    );
    let message = refused(d.bound(&two_devices()), 5);
    assert!(
        message.contains("is unknown")
            && message.contains("`regs` has no size")
            && message.contains("missing-fact"),
        "{message}"
    );
}

#[test]
fn rule_5_a_provider_composing_another_through_uses_owns_its_bytes_with_it() {
    // `soc`, bound for `sched.core`, composes both blocks: its bus window over theirs is no conflict.
    let d = devices(
        "(region regs (base 0x1000_0000) (size 4 KiB))",
        "(region regs (base 0x1000_1000) (size 4 KiB))",
        "time.monotonic console.write sched.core",
    );
    let mut binds = two_devices().to_vec();
    binds.push((named("sched.core"), "core-count", "soc"));
    assert_eq!(d.bound(&binds), Ok(()));
}

// ------------------------------------------------------------------------------------------------------------------
// Rule 9: privilege at the profile's level.

/// A console its `console` clauses describe, on a platform that states no level, written to by a service whose
/// requirements beside `(needs observable-output)` are `caller`, and by the task `log`.
fn privileged(console: &str, caller: &str) -> Description {
    describe(&format!(
        "
      (defblock console.uart {console})
      (defplatform soc (offers (core-count 1 tick)) (requires (uses console.uart)))
      (defservice console.write (requires (needs observable-output) {caller}))
      (defsystem s
        (task log (period 20 ms) (deadline 20 ms) (deadline-from release) (priority 2) (needs observable-output))
        (requires (uses console.write)) (platform (uses soc)))"
    ))
}

fn both_callers() -> [Bind; 2] {
    [
        (named("console.write"), "observable-output", "console.uart"),
        (task("s", "log"), "observable-output", "console.uart"),
    ]
}

/// `rt-static-up-v1`, its callers run at `level`.
fn at_level(level: &'static str) -> Profile {
    Profile {
        caller_level: level,
        ..RT_STATIC_UP_V1
    }
}

#[test]
fn rule_9_each_provider_bound_for_a_caller_is_reachable_at_the_profile_s_level() {
    for console in [
        "(offers (observable-output true) (reachable-at-privilege machine))",
        "(offers (observable-output true) (reachable-at-privilege supervisor machine))",
    ] {
        assert_eq!(
            privileged(console, "").bound(&both_callers()),
            Ok(()),
            "{console}"
        );
    }
    // A caller stating the level, or a set holding it, passes.
    for caller in [
        "(reachable-at-privilege machine)",
        "(reachable-at-privilege supervisor machine)",
    ] {
        let d = privileged(
            "(offers (observable-output true) (reachable-at-privilege machine))",
            caller,
        );
        assert_eq!(d.bound(&both_callers()), Ok(()), "{caller}");
    }
}

#[test]
fn rule_9_a_provider_bound_for_a_caller_and_not_reachable_at_the_level_is_refused() {
    for (console, why, conflict) in [
        (
            "(offers (observable-output true) (reachable-at-privilege supervisor))",
            "is reachable from `supervisor` alone",
            true,
        ),
        (
            "(offers (observable-output true)) (absent reachable-at-privilege)",
            "reachable from no level",
            true,
        ),
        (
            "(offers (observable-output true) reachable-at-privilege)",
            "bare, naming no level",
            false,
        ),
        (
            "(offers (observable-output true))",
            "does not describe `reachable-at-privilege`",
            false,
        ),
    ] {
        let findings = refusal(privileged(console, "").bound(&both_callers()), 9, true);
        assert_eq!(findings.len(), 1, "one finding per provider: {findings:#?}");
        let message = &findings[0].message;
        assert!(
            message.contains(why)
                && message.contains("`console.write` for `observable-output`")
                && message.contains("`(task s log)` for `observable-output`")
                && message.contains("`machine`")
                && (conflict != message.contains("missing-fact")),
            "{message}"
        );
    }
}

#[test]
fn rule_9_a_caller_stating_levels_that_do_not_hold_the_profile_s_is_the_description_s_unsupported_profile(
) {
    // Judged first: the provider, reachable from `supervisor` alone, would be the plan's conflict.
    let d = privileged(
        "(offers (observable-output true) (reachable-at-privilege supervisor))",
        "(reachable-at-privilege supervisor)",
    );
    let message = refused_description(d.bound(&both_callers()), Status::UnsupportedProfile, 9);
    assert!(
        message.contains("`console.write` states it calls from `supervisor`")
            && message.contains("`machine`"),
        "{message}"
    );
}

#[test]
fn rule_9_the_level_is_the_profile_s_never_the_record_s() {
    let supervisor = at_level("supervisor");
    // Under a profile running callers at `supervisor`, a console reachable from `machine` alone is refused ...
    let d = privileged(
        "(offers (observable-output true) (reachable-at-privilege machine))",
        "",
    );
    let message = refused(
        d.joint_under(&supervisor, &d.plan(None, &both_callers())),
        9,
    );
    assert!(
        message.contains("runs every caller at `supervisor`"),
        "{message}"
    );
    // ... one reachable from both holds, and a caller stating `machine` alone states a level no context runs at.
    let d = privileged(
        "(offers (observable-output true) (reachable-at-privilege supervisor machine))",
        "",
    );
    assert_eq!(
        d.joint_under(&supervisor, &d.plan(None, &both_callers())),
        Ok(())
    );
    let d = privileged(
        "(offers (observable-output true) (reachable-at-privilege supervisor machine))",
        "(reachable-at-privilege machine)",
    );
    refused_description(
        d.joint_under(&supervisor, &d.plan(None, &both_callers())),
        Status::UnsupportedProfile,
        9,
    );
}

#[test]
fn rule_9_with_no_level_given_says_nothing_and_a_level_outside_the_domain_is_the_engine_s_input_error(
) {
    let d = privileged(
        "(offers (observable-output true) (reachable-at-privilege supervisor))",
        "",
    );
    let plan = d.plan(None, &both_callers());
    assert_eq!(d.joint_under(&at_level(""), &plan), Ok(()));
    let verdict = d
        .joint_under(&at_level("hypervisor"), &plan)
        .expect_err("a level the vocabulary does not name");
    assert_eq!(verdict.status, Status::ToolFailure);
    assert_eq!(verdict.findings.len(), 1);
    assert!(
        verdict.findings[0].message.contains("`hypervisor`")
            && verdict.findings[0]
                .message
                .contains("the engine's input error")
            && verdict.findings[0].message.contains("(rule 9)"),
        "{verdict:#?}"
    );
    assert_eq!(
        verdict.findings[0].primary.line, 2,
        "the record's `profile` clause"
    );
}

#[test]
fn rule_9_judges_callers_never_a_block_needing_another_s_fact() {
    // `timer.a`'s own requirement binds `clock.main`, which states no level: a block has none to be reached at.
    let d = describe(
        "
      (defblock clock.main (offers (clock-rate 10 MHz)))
      (defblock timer.a (offers (counter-width 32 bit) (reachable-at-privilege machine)) (requires (clock-rate 10 MHz)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a clock.main)))
      (defservice time.monotonic (requires (counter-width 32 bit)))
      (defsystem s (requires (uses time.monotonic)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[
            (named("time.monotonic"), "counter-width", "timer.a"),
            (named("timer.a"), "clock-rate", "clock.main"),
        ]),
        Ok(())
    );
    // The same provider bound for a caller is refused.
    let d = describe(
        "
      (defblock clock.main (offers (clock-rate 10 MHz)))
      (defblock timer.a (offers (counter-width 32 bit) (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a clock.main)))
      (defservice time.monotonic (requires (counter-width 32 bit) (clock-rate 10 MHz)))
      (defsystem s (requires (uses time.monotonic)) (platform (uses soc)))",
    );
    let message = refused(
        d.bound(&[
            (named("time.monotonic"), "counter-width", "timer.a"),
            (named("time.monotonic"), "clock-rate", "clock.main"),
        ]),
        9,
    );
    assert!(message.contains("`clock.main`"), "{message}");
}

// ------------------------------------------------------------------------------------------------------------------
// Rule 10: placement.

/// A console on a platform whose regions `regions` writes, beside a memory block no platform composes.
fn memory(regions: &str) -> Description {
    describe(&format!(
        "
      (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
      (defblock spare.mem (offers (debug-port true) (region ram (base 0x4000_0000) (size 1 MiB) (executable true))))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine) {regions})
        (requires (uses console.uart)))
      (defservice console.write (requires (needs observable-output)))
      (defsystem s (requires (uses console.write)) (platform (uses soc)))"
    ))
}

const MAP: &str = "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
  (region fw (base 0x8000_0000) (size 1 MiB) (reserved true))
  (region rom (base 0x2000_0000) (size 1 MiB) (executable true) (reserved true))
  (region regs (base 0x1000_0000) (size 4 KiB))
  (region boot (base 0x0) (reserved true))";

fn write() -> [Bind; 1] {
    [(named("console.write"), "observable-output", "console.uart")]
}

#[test]
fn rule_10_a_plan_places_the_image_in_placement_memory_of_the_system_s_hardware() {
    // A reserved region over placement memory is a hole, and one without size a hole too, claiming nothing.
    let d = memory(MAP);
    assert_eq!(d.joint(&d.plan(Some(("soc", "ram")), &write())), Ok(()));
    // A system with no placement memory is said nothing of.
    let d = memory("(region regs (base 0x1000_0000) (size 4 KiB))");
    assert_eq!(d.joint(&d.plan(None, &write())), Ok(()));
}

#[test]
fn rule_10_a_placement_that_is_not_placement_memory_of_the_hardware_is_refused() {
    let d = memory(MAP);
    for (placement, why) in [
        (("soc", "rom"), "is reserved"),
        (("soc", "regs"), "is not `(executable true)`"),
        (("soc", "nowhere"), "offers no region `nowhere`"),
        (
            ("spare.mem", "ram"),
            "`spare.mem` is outside the hardware of `s`",
        ),
        (("console.write", "ram"), "is no block or platform"),
    ] {
        let plan = d.plan(Some(placement), &write());
        let findings = refusal(d.joint(&plan), 10, true);
        assert_eq!(findings.len(), 1, "{findings:#?}");
        assert!(findings[0].message.contains(why), "{findings:#?}");
        // At the record's `placement` clause.
        assert_eq!(
            (
                findings[0].primary.source.as_str(),
                findings[0].primary.line
            ),
            (RECORD, 5)
        );
    }
    // A plan placing the image nowhere, where the hardware has placement memory.
    let message = refused(d.joint(&d.plan(None, &write())), 10);
    assert!(
        message.contains("`soc`'s region `ram` [0x80000000, 0x88000000)")
            && message.contains("places the image in none"),
        "{message}"
    );
}

#[test]
fn rule_10_a_claim_without_size_below_placement_memory_is_the_description_s_missing_fact() {
    // Check (2) refuses it first, description-wide; check (5) judges it again over the system's hardware.
    let d = memory(
        "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))
         (region regs (base 0x1000_0000))",
    );
    let plan = d.plan(Some(("soc", "ram")), &write());
    let text = plan.to_canonical();
    assert!(record(&d.program, &RT_STATIC_UP_V1, RECORD, &text).is_ok());
    let first = description(&d.sources, &d.program, &RT_STATIC_UP_V1)
        .expect_err("check (2) refuses it first");
    assert_eq!(first.status, Status::MissingFact);
    let message = refused_description(
        joint(
            &d.sources,
            &d.program,
            &RT_STATIC_UP_V1,
            RECORD,
            &text,
            &plan,
        ),
        Status::MissingFact,
        10,
    );
    assert!(
        message.contains("`regs` has no size") && message.contains("`ram`"),
        "{message}"
    );
}

// ------------------------------------------------------------------------------------------------------------------
// Rule 13: one function, one provider, and the qualifiers.

/// Two counters offering `a` and `b`, and a service reading time from them, its requirements beside the counter's
/// width and rate `more`; the system uses it, its own requirements `own`.
fn counters(a: &str, b: &str, more: &str, own: &str) -> Description {
    describe(&format!(
        "
      (defblock timer.a (offers {a} (reachable-at-privilege machine)))
      (defblock timer.b (offers {b} (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a timer.b)))
      (defservice time.monotonic (requires (counter-width 32 bit) (tick-rate 10 MHz) {more}))
      (defsystem s (requires (uses time.monotonic) {own}) (platform (uses soc)))"
    ))
}

const COUNTER: &str = "(counter-width 32 bit) (tick-rate 10 MHz)";

fn split(width: &'static str, rate: &'static str) -> [Bind; 2] {
    [
        (named("time.monotonic"), "counter-width", width),
        (named("time.monotonic"), "tick-rate", rate),
    ]
}

#[test]
fn rule_13_one_requester_s_facts_of_one_function_bound_to_one_provider_hold() {
    let d = counters(COUNTER, COUNTER, "", "");
    assert_eq!(d.bound(&split("timer.a", "timer.a")), Ok(()));
    assert_eq!(d.bound(&split("timer.b", "timer.b")), Ok(()));
}

#[test]
fn rule_13_a_split_one_provider_meets_whole_is_the_assignment_s_conflict() {
    let d = counters(
        "(counter-width 32 bit) (tick-rate 10 MHz)",
        "(counter-width 64 bit) (tick-rate 10 MHz)",
        "",
        "",
    );
    let plan = d.plan(None, &split("timer.a", "timer.b"));
    let findings = refusal(d.joint(&plan), 13, true);
    assert_eq!(findings.len(), 1, "{findings:#?}");
    let message = &findings[0].message;
    assert!(
        message.contains("`time.monotonic`")
            && message.contains("`counter` function to 2 providers")
            && message.contains("`timer.a` for `counter-width`")
            && message.contains("`timer.b` for `tick-rate`")
            && message.contains("meets every one"),
        "{message}"
    );
    assert_eq!(
        findings[0].primary.line,
        line_of(&plan, &named("time.monotonic"), "counter-width")
    );
}

#[test]
fn rule_13_a_split_none_meets_whole_and_one_refutes_nothing_of_is_an_unknown_naming_it() {
    let d = counters("(counter-width 32 bit)", "(tick-rate 10 MHz)", "", "");
    let message = refused(d.bound(&split("timer.a", "timer.b")), 13);
    assert!(
        message.contains("refutes none")
            && message.contains("`timer.a`, where `tick-rate` undescribed")
            && message.contains("`timer.b`, where `counter-width` undescribed")
            && message.contains("missing-fact"),
        "{message}"
    );
}

#[test]
fn rule_13_a_split_each_provider_refutes_part_of_is_a_conflict_naming_each() {
    let d = counters(
        "(counter-width 32 bit) (tick-rate 1 MHz)",
        "(counter-width 16 bit) (tick-rate 10 MHz)",
        "",
        "",
    );
    let message = refused(d.bound(&split("timer.a", "timer.b")), 13);
    assert!(
        message.contains("each refutes one")
            && message.contains("`timer.a` for `counter-width`")
            && message.contains("`timer.b` for `tick-rate`"),
        "{message}"
    );
}

#[test]
fn rule_13_is_per_requester_and_never_spans_two() {
    let d = describe(
        "
      (defblock timer.a (offers (counter-width 32 bit) (reachable-at-privilege machine)))
      (defblock timer.b (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))
      (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.a timer.b)))
      (defservice time.width (requires (counter-width 32 bit)))
      (defservice time.rate (requires (tick-rate 10 MHz)))
      (defsystem s (requires (uses time.width time.rate)) (platform (uses soc)))",
    );
    assert_eq!(
        d.bound(&[
            (named("time.width"), "counter-width", "timer.a"),
            (named("time.rate"), "tick-rate", "timer.b"),
        ]),
        Ok(())
    );
}

#[test]
fn rule_13_a_qualifier_is_judged_at_every_provider_its_requester_relies_on() {
    let whole = "(counter-width 32 bit) (tick-rate 10 MHz)";
    // The service's own qualifier, and the system's, which reaches the service through `uses`: no binding of its own.
    for (more, own) in [
        ("(available-in-state idle)", ""),
        ("", "(available-in-state idle)"),
    ] {
        let d = counters(
            &format!("{whole} (available-in-state run idle)"),
            whole,
            more,
            own,
        );
        assert_eq!(d.bound(&split("timer.a", "timer.a")), Ok(()), "{more}{own}");
        for (a, why, conflict) in [
            (
                format!("{whole} (available-in-state run)"),
                "is refused there",
                true,
            ),
            (whole.to_owned(), "is undescribed there", false),
        ] {
            let d = counters(&a, whole, more, own);
            let message = refused(d.bound(&split("timer.a", "timer.a")), 13);
            assert!(
                message.contains(why)
                    && message.contains("`available-in-state`")
                    && message.contains("relies on `timer.a`")
                    && (conflict != message.contains("an unknown")),
                "{message}"
            );
        }
    }
}

#[test]
fn rule_13_never_repeats_rule_9_s_qualifier() {
    // The service states two levels; its provider is reachable from the profile's alone. Rule 9 judges the provider at
    // `machine`, and rule 13 judges `reachable-at-privilege` nowhere.
    let d = counters(
        COUNTER,
        COUNTER,
        "(reachable-at-privilege supervisor machine)",
        "",
    );
    assert_eq!(d.bound(&split("timer.a", "timer.a")), Ok(()));
}
