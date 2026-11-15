//! Format `/2` on a target, its identity and refusals, as `lower_on_target` and `target_lines` give them
//! (`docs/decisions/decision_runtime-plan-format-2.md` §1, "The target in the identity", "The catalog in the identity" and
//! "The application and the realization in the identity", and §7's order; leaf `M4.1.7.6.7.1`), over `targets/riscv-virt-up.eadl`
//! and the S0 heartbeat: the target's own refusals first, by the code `check` gives them; then the application's file; then
//! the record, of the kind and the target given, the digest deciding the run; then the application's target, the platform's
//! name. Its time (leaf `M4.1.7.6.7.2`) is held through `on_target_so_far`: the release timer's line, the epoch's, each
//! alarm's `COUNTER` line, the release timer's device's `(interrupt timer)`, and what each owes, over the shipped target
//! and `TWO_CHANNELS`, a test target with an alarm's room and a source, written in this file. The rest of a plan on a target
//! is `M4.1.7.6.7.3` and `M4.1.7.6.8`'s, and the lowering says so rather than write it short.

mod common;

use std::fs;

use archogen_digest::Digest;
use archogen_plan::application::{CLAUSE_CUT, CLAUSE_PARTS, FORMAT};
use archogen_plan::{
    lower_hosted, lower_on_target, on_target_so_far, target_lines, ApplicationFile, Generation,
    Refusal, TargetFile,
};
use common::{catalog, hosted_record, registry, root, searched_on_target, vocabulary};
use eadl_resolve::plan;

const EMPTY: [(String, Digest); 0] = [];

fn generation() -> Generation<'static> {
    Generation {
        emitter: "archogen-emit/1",
        records: &EMPTY,
    }
}

fn heartbeat() -> String {
    fs::read_to_string(root().join("examples/s0-heartbeat/system.eadl")).expect("the example")
}

fn riscv_virt_up() -> String {
    fs::read_to_string(root().join("targets/riscv-virt-up.eadl")).expect("the shipped target")
}

/// The heartbeat's `/3` on the shipped target, lowered on `target` with `application`, its file named `app.eadl`.
fn on(target: &str, record: Option<&str>, application: Option<&str>) -> Result<String, Refusal> {
    let (program, resolved) =
        searched_on_target(&heartbeat(), &riscv_virt_up()).expect("it resolves on the target");
    let record = record.map_or(resolved, str::to_owned);
    let application = application.map(|text| ApplicationFile {
        name: "app.eadl",
        text,
    });
    lower_on_target(
        &program,
        &record,
        &TargetFile {
            name: "riscv-virt-up.eadl",
            text: target,
        },
        &registry(),
        &vocabulary(),
        &catalog(),
        &generation(),
        application.as_ref(),
    )
}

/// The heartbeat's `/3` record on the shipped target.
fn the_record() -> String {
    searched_on_target(&heartbeat(), &riscv_virt_up())
        .expect("it resolves")
        .1
}

/// The refusal's code, and that its message says `said`.
fn refused(outcome: Result<String, Refusal>, said: &str) -> &'static str {
    let refusal = outcome.expect_err("refused");
    assert!(refusal.message.contains(said), "{said}: {refusal}");
    refusal.code
}

/// Past every stage built: the rest of the plan is `M4.1.7.6.7.3` and `M4.1.7.6.8`'s.
const UNBUILT: &str = "M4.1.7.6.7.3";

/// A whole application's file for the heartbeat, of `target`.
fn an_application(target: &str) -> String {
    const LIST: &str = r#"(scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by reading")"#;
    const BOUND: &str = r#"(value 200) (unit us) (scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by the loop's bound")"#;
    let task = |name: &str| {
        format!(
            r#"(task {name} (entry-point "app::{name}") (execution-bound {BOUND}) (os-calls {LIST}) (node-calls {LIST}) (masked-runs {LIST}) (stack-bound (value 512) (unit byte) (scope "its deepest path") (binary unbuilt) (evidence assumed) (basis "chosen")) (suspends no) (locks-scheduler no) (shares-outside-sections no) (masks-only-through-api yes) (figures-under-preemption yes) (deadlines-within-reach))
"#
        )
    };
    format!(
        "(format \"{FORMAT}\")\n(system heartbeat)\n(target {target})\n(code (crate app) (place \"app\") (digest \"sha256:{}\"))\n(leaves-interrupt-hardware-alone yes (clause \"{}\") (basis \"it touches no device\"))\n(initialisation none)\n{}{}",
        "0".repeat(64),
        CLAUSE_PARTS.join(CLAUSE_CUT),
        task("beat"),
        task("chime")
    )
}

/// The heartbeat on the shipped target passes every stage this leaf builds, and its target's lines are the file's: its one
/// platform with the digest of its forms, and the catalog's digest with the platform's `catalog-target`.
#[test]
fn the_heartbeat_on_riscv_virt_up_has_its_target_s_lines() {
    assert_eq!(
        refused(on(&riscv_virt_up(), None, None), UNBUILT),
        "tool-failure"
    );
    let lines = target_lines(
        &TargetFile {
            name: "riscv-virt-up.eadl",
            text: &riscv_virt_up(),
        },
        &registry(),
        &vocabulary(),
        &generation(),
    )
    .expect("a target");
    let record = plan::read(&the_record()).expect("the record reads");
    let (platform, digest) = record.target.clone().expect("a `/3`");
    assert_eq!(lines.target, format!("(target {platform} \"{digest}\")"));
    assert_eq!(
        lines.target,
        "(target target.riscv-virt-up \"".to_owned() + &digest + "\")"
    );
    assert_eq!(
        lines.catalog,
        format!("(catalog \"{}\" riscv-virt-up)", Digest::of(b"").hex())
    );
    // A comment moves no digest, so the record resolved on the file stands on it commented.
    let commented = format!("; a comment\n{}", riscv_virt_up());
    assert_eq!(refused(on(&commented, None, None), UNBUILT), "tool-failure");
}

/// §7's first stage: the target's own refusals before anything is lowered, `check`'s judgement of its file alone, by the code
/// `check` gives (`DC-H3`, `DC-H4`): the file not reading; the profile's one core and no source no mask stops,
/// `unsupported-profile`; no placement memory, `infeasible-configuration`; a count silent, `missing-fact`; and where several
/// stand, the one `check`'s precedence ranks first (RP35-4, RP36-1). Each comes before the application's file and the record.
#[test]
fn a_target_s_own_refusals_come_first_by_check_s_code() {
    let shipped = riscv_virt_up();
    let edit = |from: &str, to: &str| {
        assert!(shipped.contains(from), "{from}");
        shipped.replacen(from, to, 1)
    };
    for (target, code) in [
        (
            "(defplatform target.broken".to_owned(),
            "invalid-description",
        ),
        (
            edit("(core-count 1 tick)", "(core-count 2 tick)"),
            "unsupported-profile",
        ),
        (
            edit("(non-maskable-interrupts 0)", "(non-maskable-interrupts 1)"),
            "unsupported-profile",
        ),
        (
            edit(
                "\n    (region target.ram (base 0x8000_0000) (size 128 MiB) (executable true))",
                "",
            ),
            "infeasible-configuration",
        ),
        (
            edit("\n    (non-maskable-interrupts 0)", ""),
            "missing-fact",
        ),
        (
            edit("(non-maskable-interrupts 0)", "(non-maskable-interrupts 1)").replacen(
                "\n    (firmware-traps 0)",
                "",
                1,
            ),
            "unsupported-profile",
        ),
    ] {
        // A bad application's file and a stale record too: the target's refusal stands before either.
        let outcome = on(
            &target,
            Some("(format \"archogen-assignment/3\")\n"),
            Some("not a file"),
        );
        assert_eq!(
            refused(outcome, "the target's file `riscv-virt-up.eadl`"),
            code,
            "{target}"
        );
    }
}

/// §1, the record's acceptance: of the kind and the target given, the digest deciding which run it is of (leaf
/// `M4.1.7.5.6`). Another target's digest, a `/2` given a target and a `/3` given none are records of other runs,
/// `stale-input`; another platform beside the file's own digest is no record a run writes, `tool-failure`.
#[test]
fn a_record_is_accepted_by_its_kind_and_its_target_s_digest() {
    let record = the_record();
    let (_, digest) = plan::read(&record)
        .expect("it reads")
        .target
        .expect("a `/3`");
    let other = record.replacen(&digest, &"0".repeat(64), 1);
    assert_eq!(
        refused(on(&riscv_virt_up(), Some(&other), None), "resolve again"),
        "stale-input"
    );
    let (_, second) = hosted_record(&heartbeat()).expect("it resolves on its own hardware");
    assert_eq!(
        refused(
            on(&riscv_virt_up(), Some(&second), None),
            "resolved on no target"
        ),
        "stale-input"
    );
    let misnamed = record.replacen(
        "(target target.riscv-virt-up ",
        "(target target.elsewhere ",
        1,
    );
    assert_eq!(
        refused(
            on(&riscv_virt_up(), Some(&misnamed), None),
            "never writes this record"
        ),
        "tool-failure"
    );
    // A `/3` lowered hosted, with no target, was resolved for another run.
    let (program, _) = searched_on_target(&heartbeat(), &riscv_virt_up()).expect("it resolves");
    let hosted = lower_hosted(
        &program,
        &record,
        &vocabulary(),
        &catalog(),
        &generation(),
        None,
    );
    assert_eq!(refused(hosted, "no target is given"), "stale-input");
    // The record's refusals come after the application's file not reading (§7).
    assert_eq!(
        refused(
            on(&riscv_virt_up(), Some(&misnamed), Some("not a file")),
            "app.eadl"
        ),
        "invalid-description"
    );
}

/// The application's file names its target as the plan's `(target NAME "HEX")` does, by the platform's name, and is refused
/// naming both where it names another: the catalog's name for the machine, `catalog-target`, is a catalog record's
/// (`decision_runtime-plan-format-2.md` §2's table), and the hosted playground's is a hosted plan's. `Application::unmet`
/// holds the file against the plan's own target.
#[test]
fn an_application_names_its_target_by_the_platform() {
    let own = an_application("target.riscv-virt-up");
    assert_eq!(
        refused(on(&riscv_virt_up(), None, Some(&own)), UNBUILT),
        "tool-failure"
    );
    for other in ["riscv-virt-up", "hosted-playground"] {
        let file = an_application(other);
        let refusal = on(&riscv_virt_up(), None, Some(&file)).expect_err("refused");
        assert_eq!(refusal.code, "invalid-description", "{other}");
        assert!(
            refusal
                .message
                .contains(&format!("of the target `{other}`"))
                && refusal.message.contains("`target.riscv-virt-up`"),
            "{refusal}"
        );
    }
}

/// The joint rules' findings over the target's hardware, its map's among them, are `tool-failure` at lowering, in `/1`'s
/// joint-rules bullet's place (§5, "Which regions"; §7): the search and the checker give them their codes before any plan, so
/// a record that reaches the lowering with one is the two roles disagreeing. A record no search wrote, hand-written with the
/// digest of a target whose placement memory a reserved range fills, `MemoryAllHoles`, shows it.
#[test]
fn the_joint_rules_findings_over_the_target_are_a_tool_failure() {
    let shipped = riscv_virt_up();
    let claimed = shipped.replacen(
        "(region target.ram (base 0x8000_0000) (size 128 MiB) (executable true))",
        "(region target.ram (base 0x8000_0000) (size 128 MiB) (executable true)) (region target.hole (base 0x8000_0000) (size 128 MiB) (reserved true))",
        1,
    );
    assert_ne!(claimed, shipped);
    let lines = target_lines(
        &TargetFile {
            name: "riscv-virt-up.eadl",
            text: &claimed,
        },
        &registry(),
        &vocabulary(),
        &generation(),
    );
    let record = the_record();
    let (_, digest) = plan::read(&record)
        .expect("it reads")
        .target
        .expect("a `/3`");
    match lines {
        // The file alone stands: the map is the joint rules', at lowering.
        Ok(lines) => {
            let theirs = record.replacen(&digest, &lines.digest, 1);
            assert_eq!(
                refused(
                    on(&claimed, Some(&theirs), None),
                    "the generator's joint rules refuse it"
                ),
                "tool-failure"
            );
        }
        Err(refusal) => panic!("the file alone is judged a target: {refusal}"),
    }
}

/// §7's order, read from the record, over the places this leaf reaches: each adjacent pair, both met by one input, gives the
/// earlier refusal. The target's own before the application's file not reading; that before the record; a platform against
/// the file's digest, and a joint rules' finding, before the application's target, which stands in an application's unmet's
/// place.
#[test]
fn each_pair_of_section_7_s_places_gives_the_earlier() {
    const RECORD: &str = include_str!("../../../docs/decisions/decision_runtime-plan-format-2.md");
    const PLACES: [&str; 7] = [
        "a target's own refusals",
        "the application's file not reading",
        "an assignment of the other kind",
        "the assignment's `(target …)` digest other than the file's",
        "its platform other than the file's beside the file's digest",
        "a finding of the generator's joint rules over the target's hardware",
        "an application's unmet",
    ];
    let flat = RECORD.split_whitespace().collect::<Vec<_>>().join(" ");
    let sentence = &flat[flat
        .find("A plan on a target adds those above, in this record's order")
        .expect("§7's order")..];
    let at: Vec<usize> = PLACES
        .iter()
        .map(|place| {
            sentence
                .find(place)
                .unwrap_or_else(|| panic!("`{place}` has no place"))
        })
        .collect();
    assert!(at.windows(2).all(|pair| pair[0] < pair[1]), "{at:?}");
    let shipped = riscv_virt_up();
    let silent = shipped.replacen("\n    (non-maskable-interrupts 0)", "", 1);
    let record = the_record();
    let misnamed = record.replacen(
        "(target target.riscv-virt-up ",
        "(target target.elsewhere ",
        1,
    );
    let catalog_named = an_application("riscv-virt-up");
    assert_eq!(
        refused(on(&silent, None, Some("not a file")), "riscv-virt-up.eadl"),
        "missing-fact"
    );
    assert_eq!(
        refused(
            on(&shipped, Some(&misnamed), Some("not a file")),
            "app.eadl"
        ),
        "invalid-description"
    );
    assert_eq!(
        refused(
            on(&shipped, Some(&misnamed), Some(&catalog_named)),
            "never writes this record"
        ),
        "tool-failure"
    );
    let filled = shipped.replacen(
        "(region target.ram (base 0x8000_0000) (size 128 MiB) (executable true))",
        "(region target.ram (base 0x8000_0000) (size 128 MiB) (executable true)) (region target.hole (base 0x8000_0000) (size 128 MiB) (reserved true))",
        1,
    );
    let digest = target_lines(
        &TargetFile {
            name: "riscv-virt-up.eadl",
            text: &filled,
        },
        &registry(),
        &vocabulary(),
        &generation(),
    )
    .expect("a target")
    .digest;
    let (_, shipped_digest) = plan::read(&record)
        .expect("it reads")
        .target
        .expect("a `/3`");
    let theirs = record.replacen(&shipped_digest, &digest, 1);
    assert_eq!(
        refused(
            on(&filled, Some(&theirs), Some(&catalog_named)),
            "joint rules"
        ),
        "tool-failure"
    );
}

// ---- a target's time (leaf `M4.1.7.6.7.2`)

/// A test target: the shipped target's timer with two channels and a device raising interrupts. It is written here and
/// not as a file of the tree, since every `.eadl` file the tree holds is a description whose verdict
/// `crates/archogen-cli/tests/verdicts.rs` freezes and whose third opinion `docs/semantics/third-opinion.txt` records.
const TWO_CHANNELS: &str = r#"(eadl-version eadl/1)
; A test target, no machine (leaf `M4.1.7.6.7.2`): `targets/riscv-virt-up.eadl`'s timer with two channels, so an alarm can
; stand beside the release timer on its device, and a device raising interrupts, so a sporadic task can be released on it.
; Its statements are this test's, checked by no agreement.

(defblock target.timer
  (offers
    (tick-rate 10 MHz)
    (relative-delay true)
    (channels 2 tick)
    (alarm-wiring core)
    (alarm-kind level-compare)
    (counter-modulus (pow2 64))
    (counter-width 64 bit)
    (wrap-behavior modular)
    (compare-step 1 tick)
    (compare-magnitude true)
    (observation-coherent true)
    (available-in-state run idle)
    (supported-horizon 1844674407370 s)
    (reachable-at-privilege machine)
    (region target.timer (base 0x0200_0000) (size 64 KiB))))

(defblock target.console
  (offers
    (observable-output true)
    (reachable-at-privilege machine)
    (region target.console (base 0x1000_0000) (size 256 byte))))

(defblock target.button
  (offers
    (interrupt-source true)
    (min-arrival-separation 20 ms)
    (reachable-at-privilege machine)
    (region target.button (base 0x1000_1000) (size 256 byte))))

(defplatform target.fixture
  (offers
    (core-count 1 tick)
    (catalog-target fixture)
    (firmware-traps 0)
    (non-maskable-interrupts 0)
    (reachable-at-privilege machine)
    (region target.ram (base 0x8000_0000) (size 128 MiB) (executable true)))
  (requires (uses target.timer target.console target.button)))
"#;

/// [`TWO_CHANNELS`]'s text.
fn two_channels() -> String {
    TWO_CHANNELS.to_owned()
}

/// A system whose task waits on a deadline service bound directly to a timer: on a target, an alarm beside the release
/// timer on its device.
const WAITER: &str = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defblock timer.d (offers (relative-delay true) (counter-modulus 65536) (tick-rate 10 MHz) (observation-coherent true) (channels 1 tick) (reachable-at-privilege machine)))
(defplatform host.p (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart timer.d)))
(defservice console.write (requires (needs observable-output)))
(defservice wait (requires (needs relative-delay)))
(defsystem waiter
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write wait) (on-overrun fault))
  (requires (uses console.write wait))
  (platform (uses host.p)))
";

/// A system of one sporadic task released by a device's interrupt, and no periodic one: no release timer.
const SPORADIC: &str = "(eadl-version eadl/1)
(defblock dev.e (offers (interrupt-source true) (min-arrival-separation 20 ms) (reachable-at-privilege machine)))
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc.e (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses dev.e console.uart)))
(defservice log (requires (needs observable-output)))
(defservice evt (requires (needs interrupt-source) (uses log)))
(defsystem sp
  (task s (min-separation 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses evt))
  (platform (uses soc.e)))
";

/// `description`'s first system on `target`, searched there and lowered as far as built: its lines.
fn so_far(description: &str, target: &str) -> Vec<String> {
    let (program, record) =
        searched_on_target(description, target).expect("it resolves on the target");
    on_target_so_far(
        &program,
        &record,
        &TargetFile {
            name: "target.eadl",
            text: target,
        },
        &registry(),
        &vocabulary(),
        &catalog(),
        &generation(),
        None,
    )
    .expect("it lowers as far as built")
    .text()
    .lines()
    .map(str::to_owned)
    .collect()
}

/// The lines of `lines` whose head is `head`.
fn headed<'l>(lines: &'l [String], head: &str) -> Vec<&'l str> {
    lines
        .iter()
        .filter(|line| archogen_plan::identity::head(line) == head)
        .map(String::as_str)
        .collect()
}

/// The S0 heartbeat on the shipped target: the release timer's line, its device's counter as `COUNTER` writes it,
/// unconverted, its horizon derived of that counter and its rounding owed; the epoch after a read, its lead owed; the
/// timer enabled; the release timer's device a node with its step, its context `(interrupt timer)`; no alarm, so no
/// `last-act-lead`; no clock line and nothing simulated (§3, §4; `decision_release-alarm.md` §1).
#[test]
fn the_heartbeat_s_time_on_the_shipped_target() {
    let lines = so_far(&heartbeat(), &riscv_virt_up());
    assert_eq!(
        headed(&lines, "timer"),
        ["(timer (device target.timer) (tick-rate 10000000) (modulus (pow2 64)) (reach 1844674407370 s) (step 1) (window magnitude) (horizon 3689348814741910323/2000000 s) (rounding owed) (first-compare transition) (releases (task heartbeat beat) (task heartbeat chime)))"]
    );
    assert_eq!(headed(&lines, "epoch"), ["(epoch after-read (lead owed))"]);
    assert_eq!(headed(&lines, "enabled"), ["(enabled timer)"]);
    assert!(lines.contains(&"(node target.timer block)".to_owned()));
    assert!(lines
        .iter()
        .any(|line| line.starts_with("(step ") && line.ends_with(" target.timer)")));
    assert_eq!(
        headed(&lines, "context")
            .into_iter()
            .filter(|line| line.starts_with("(context target.timer "))
            .collect::<Vec<_>>(),
        ["(context target.timer (interrupt timer) initialisation)"]
    );
    for owed in [
        "(owed (timer) compare-rounding (by catalog))",
        "(owed (epoch) epoch-lead (by catalog))",
    ] {
        assert!(lines.contains(&owed.to_owned()), "{owed}");
    }
    assert!(!lines.iter().any(|line| line.contains("last-act-lead")));
    assert!(headed(&lines, "clock").is_empty() && headed(&lines, "alarm").is_empty());
    assert!(!lines.iter().any(|line| line.contains("simulat")));
    // The epoch stands before the timer, as §3's forms order them.
    let at = |head: &str| {
        lines
            .iter()
            .position(|line| archogen_plan::identity::head(line) == head)
    };
    assert!(at("epoch") < at("timer") && at("timer") < at("enabled"));
}

/// A system no task of which the timer releases: `(timer none)`, `(epoch none)` with no alarm, no `timer` enabled, and
/// neither the rounding nor the lead owed (§4; RP11-1, RP35-1).
#[test]
fn a_system_with_no_timed_task_has_no_timer_and_no_epoch() {
    let lines = so_far(SPORADIC, &two_channels());
    assert_eq!(headed(&lines, "timer"), ["(timer none)"]);
    assert_eq!(headed(&lines, "epoch"), ["(epoch none)"]);
    assert_eq!(
        headed(&lines, "enabled"),
        ["(enabled (source target.button))"]
    );
    assert!(!lines
        .iter()
        .any(|line| line.contains("compare-rounding") || line.contains("epoch-lead")));
}

/// An alarm on a target: its device's counter as `COUNTER` writes it in place of `SIMULATED`, its kind the device's; its
/// `lag` owed, as `alarm::owes` asks of an alarm on a target; the release timer's device naming both interrupts; and
/// `last-act-lead` owed of the epoch with any alarm (§3, §4).
#[test]
fn an_alarm_on_a_target_writes_its_counter_and_owes_its_lag() {
    let lines = so_far(WAITER, &two_channels());
    assert_eq!(
        headed(&lines, "alarm"),
        ["(alarm wait (device target.timer) (tick-rate 10000000) (modulus (pow2 64)) (reach 1844674407370 s) (step 1) (window magnitude) (kind level-compare) (time-base counter) (horizon none) (delivery-bound none) (asked-reach none))"]
    );
    assert_eq!(
        headed(&lines, "route"),
        ["(route 1 (alarm wait) (separation programmed) (level machine) (deliverable yes) (requesters wait) (costs))"]
    );
    assert_eq!(headed(&lines, "enabled"), ["(enabled timer (alarm wait))"]);
    assert!(lines.contains(
        &"(context target.timer job (interrupt (alarm wait)) (interrupt timer) initialisation)"
            .to_owned()
    ));
    for owed in [
        "(owed (alarm wait) lag (by realization))",
        "(owed (epoch) last-act-lead (by realization))",
        "(owed (epoch) epoch-lead (by catalog))",
        "(owed (timer) compare-rounding (by catalog))",
    ] {
        assert!(lines.contains(&owed.to_owned()), "{owed}");
    }
    // A target's device states its rate and modulus, so the description owes neither, and no clock line is written.
    assert!(!lines
        .iter()
        .any(|line| line.contains(" tick-rate (by description)")
            || line.contains(" counter-modulus (by description)")));
    assert!(headed(&lines, "clock").is_empty());
    // The release requester is no node and takes no alarm.
    assert!(!lines.iter().any(|line| line.contains("(alarm (release ")));
}

/// The plan on a target is still refused past every stage built, now naming the leaves left: no plan short of its regions,
/// memory and controller is returned as whole.
#[test]
fn a_plan_on_a_target_names_the_leaves_it_waits_on() {
    let refusal = on(&riscv_virt_up(), None, None).expect_err("refused");
    assert_eq!(refusal.code, "tool-failure");
    assert!(
        refusal.message.contains("M4.1.7.6.7.3") && refusal.message.contains("M4.1.7.6.8"),
        "{refusal}"
    );
}
