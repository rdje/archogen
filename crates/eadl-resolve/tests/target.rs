//! A system on a target (`docs/decisions/decision_device-correspondence.md`, leaf `M4.4.1`): the rule held over the real target's
//! file and the examples, each refusal by what it says.

use std::collections::BTreeMap;
use std::path::PathBuf;

use eadl_front::{read, Diagnostic, Form, SourceMap, Span};
use eadl_model::check::shipped_registry;
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::adapter::Catalog;
use eadl_resolve::joint::System;
use eadl_resolve::search::{search, Limits, Policy, Resolution, Search};
use eadl_resolve::target::{correspond, on_target, Correspondence, Target};
use eadl_resolve::vocabulary::Vocabulary;

fn root() -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../..")
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

fn file_start() -> Span {
    let mut sources = SourceMap::new();
    Span::at(sources.add("t.eadl", "").expect("small"), 0)
}

fn target_of(text: &str) -> Target {
    Target::read(&forms(text), file_start()).expect("a target")
}

/// The target this repository describes, as its file stands.
fn real_target() -> Target {
    target_of(&std::fs::read_to_string(root().join("targets/riscv-virt-up.eadl")).expect("read"))
}

/// A target with what a timer's requirements ask stated, which the real target's file does not yet state: its hand-off is `DC-H1`.
const A_WHOLE_TARGET: &str = "
(defblock target.timer
  (offers
    (counter-width 64 bit)
    (counter-modulus (pow2 64))
    (tick-rate 10 MHz)
    (observation-coherent true)
    (reachable-at-privilege machine)
    (region target.timer (base 0x0200_0000) (size 64 KiB))))
(defblock target.console
  (offers
    (observable-output true)
    (reachable-at-privilege machine)
    (region target.console (base 0x1000_0000) (size 256 byte))))
(defplatform target.machine
  (offers
    (core-count 1 tick)
    (reachable-at-privilege machine)
    (region target.ram (base 0x8000_0000) (size 128 MiB) (executable true)))
  (requires (uses target.timer target.console)))
";

/// The one system `text` declares.
fn system_of(declarations: &[Form]) -> &Form {
    declarations
        .iter()
        .find(|form| form.head() == Some("defsystem"))
        .expect("a system")
}

fn run(declarations: &[Form], system: &Form) -> Search {
    let v = vocabulary();
    let policy = Policy {
        adapters: Catalog::shipped(&mut SourceMap::new(), &v).expect("the shipped catalog reads"),
        ..Policy::default()
    };
    search(
        declarations,
        system,
        RT_STATIC_UP_V1.caller_level,
        &v,
        &policy,
        Limits::DEFAULT,
    )
}

fn resolved(search: Search) -> Resolution {
    match search {
        Search::Resolved(resolution) => resolution,
        other => panic!("no plan: {other:?}"),
    }
}

/// `text`'s own plan, its plan on `target`, and which device is which.
fn both(text: &str, target: &Target) -> (Resolution, Resolution, Correspondence) {
    let declarations = forms(text);
    let system = system_of(&declarations);
    let own = resolved(run(&declarations, system));
    let moved = on_target(system, &declarations, target).expect("it can be read there");
    let there = resolved(run(&moved.declarations, &moved.system));
    let found = correspond(&own, &System::read(system, &declarations).hardware, &there);
    (own, there, found)
}

/// Each device, with the target devices it is, requirements left out.
fn map(found: &Correspondence) -> BTreeMap<&str, Vec<&str>> {
    found
        .devices
        .iter()
        .map(|(device, there)| {
            (
                device.as_str(),
                there.keys().map(String::as_str).collect::<Vec<_>>(),
            )
        })
        .collect()
}

fn example(name: &str) -> String {
    std::fs::read_to_string(root().join("examples").join(name).join("system.eadl")).expect("read")
}

fn codes(errors: &[Diagnostic]) -> Vec<&'static str> {
    errors.iter().map(|error| error.code).collect()
}

const A_SYSTEM: &str = "
(defblock console.uart
  (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc.here
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (requires (uses console.write))
  (platform (uses soc.here)))
";

#[test]
fn the_real_target_reads_as_one_platform_composing_its_devices() {
    let target = real_target();
    assert_eq!(target.platform, "target.riscv-virt-up");
    assert_eq!(target.declarations.len(), 3);
}

#[test]
fn what_is_no_target_is_refused_by_what_it_is() {
    let refused = |text: &str, says: &str| {
        let errors = Target::read(&forms(text), file_start()).expect_err("no target");
        assert!(
            errors.iter().any(|error| error.message.contains(says)),
            "{says}: {errors:?}"
        );
    };
    refused("(defblock b (offers x))", "declares none");
    refused(
        "(defplatform a (offers x)) (defplatform b (offers x))",
        "`a`, `b`",
    );
    refused(
        "(defplatform a (offers x)) (defservice s (requires (needs x)))",
        "declares hardware alone",
    );
    refused(
        "(defplatform a (offers x)) (defblock stray (offers x))",
        "does not compose it",
    );
    // One name is one declaration: a map of names would have kept the last of these and judged the machine by it.
    let duplicate = |text: &str| {
        let errors =
            Target::read(&forms(text), file_start()).expect_err("one name, two declarations");
        assert!(
            codes(&errors).contains(&"schema-duplicate-name"),
            "{errors:?}"
        );
    };
    duplicate(
        "(defblock c (offers x)) (defblock c (offers y)) (defplatform t (requires (uses c)))",
    );
    duplicate("(defplatform t (offers x)) (defplatform t (offers y))");
    duplicate("(defblock t (offers x)) (defplatform t (offers y))");
    let nameless = Target::read(
        &forms("(defplatform t (offers x)) (defblock (offers y))"),
        file_start(),
    )
    .expect_err("a device with no name");
    assert!(
        codes(&nameless).contains(&"schema-missing-name"),
        "{nameless:?}"
    );
    // A cycle leaves no platform nothing names, which says so and does not spin.
    refused(
        "(defplatform a (requires (uses b))) (defplatform b (requires (uses a)))",
        "declares none",
    );
}

#[test]
fn a_device_is_the_target_s_device_bound_for_the_same_requirement() {
    let (_, there, found) = both(&example("s0-heartbeat"), &real_target());
    assert_eq!(
        map(&found),
        BTreeMap::from([("console.uart", vec!["target.console"])])
    );
    assert!(found.unreached.contains("host.playground"), "{found:?}");
    assert_eq!(
        there.placement,
        Some(("target.riscv-virt-up".to_owned(), "target.ram".to_owned()))
    );
    assert!(there
        .bindings
        .values()
        .all(|provider| provider.starts_with("target.")));
}

#[test]
fn what_the_real_target_does_not_state_is_said_and_no_plan_guessed() {
    // `periodic-three` asks of a counter what `targets/riscv-virt-up.eadl` states nothing of: `DC-H1`.
    let declarations = forms(&example("periodic-three"));
    let system = system_of(&declarations);
    let moved = on_target(system, &declarations, &real_target()).expect("it can be read there");
    let Search::Refused { diagnostics, .. } = run(&moved.declarations, &moved.system) else {
        panic!("a plan from facts nothing states");
    };
    let said: String = diagnostics.iter().map(|d| d.message.as_str()).collect();
    assert!(
        said.contains("counter-width") || said.contains("unambiguous-horizon"),
        "{said}"
    );
}

/// `examples/periodic-three`'s hardware, its region where it writes it, with the facts a plan of its own needs. That example has no
/// plan of its own yet: `release-accuracy` and `priorities` wait on a realization (`M3.7`), on any hardware, and `bounded-queue` is refused by the profile.
const A_REGION_AT_THE_TARGET_S_CONSOLE: &str = "
(defblock timer.counter
  (offers
    (counter-width 32 bit)
    (counter-modulus 4294967296)
    (tick-rate 10 MHz)
    (observation-coherent true)
    (reachable-at-privilege machine)))
(defblock console.uart
  (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc.playground
  (offers
    (core-count 1 tick)
    (region device.timer (base 0x1000_0000) (size 64 KiB))
    (reachable-at-privilege machine))
  (requires (uses timer.counter console.uart)))
(defservice time.monotonic
  (requires
    (unambiguous-horizon (at-least 60 s))
    (observation-coherent true)
    (needs counter-width tick-rate observation-coherent)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (requires (uses time.monotonic console.write))
  (platform (uses soc.playground)))
";

#[test]
fn a_description_s_regions_are_not_carried_and_cannot_meet_the_target_s() {
    // It says `device.timer` is at 0x1000_0000, which is this target's console. Its platform is replaced whole, so the two never meet:
    // the plan's regions are the target's, judged by the joint rules there (`M3.3.5`'s R8).
    let (own, there, found) = both(A_REGION_AT_THE_TARGET_S_CONSOLE, &target_of(A_WHOLE_TARGET));
    assert!(own
        .bindings
        .values()
        .all(|provider| !provider.starts_with("target.")));
    assert_eq!(
        map(&found),
        BTreeMap::from([
            ("console.uart", vec!["target.console"]),
            ("timer.counter", vec!["target.timer"]),
        ])
    );
    assert!(found.unreached.contains("soc.playground"), "{found:?}");
    assert_eq!(
        there.placement,
        Some(("target.machine".to_owned(), "target.ram".to_owned()))
    );
}

#[test]
fn an_adapter_a_better_device_does_not_need_is_gone_and_the_device_still_corresponds() {
    let text = std::fs::read_to_string(
        root().join("crates/eadl-resolve/tests/search/adapter-epoch-extender.eadl"),
    )
    .expect("read");
    let (own, there, found) = both(&text, &target_of(A_WHOLE_TARGET));
    assert!(
        !own.instances.is_empty(),
        "the description needs its adapter"
    );
    assert!(there.instances.is_empty(), "this target's counter does not");
    assert_eq!(
        map(&found).get("timer.short"),
        Some(&vec!["target.timer"]),
        "{found:?}"
    );
}

#[test]
fn one_device_may_be_two_and_two_one() {
    // One device of the description, two of the target's.
    let one = "
(defblock all.in-one
  (offers
    (observable-output true)
    (counter-width 32 bit)
    (tick-rate 10 MHz)
    (reachable-at-privilege machine)))
(defplatform soc.here
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses all.in-one)))
(defservice console.write (requires (needs observable-output)))
(defservice clock.read (requires (needs tick-rate)))
(defsystem s
  (requires (uses console.write clock.read))
  (platform (uses soc.here)))
";
    let (_, _, found) = both(one, &target_of(A_WHOLE_TARGET));
    assert_eq!(
        map(&found),
        BTreeMap::from([("all.in-one", vec!["target.console", "target.timer"])])
    );
    // Two of the description's, told apart by what is asked of them, and one of the target's offering both.
    let two = "
(defblock own.out (offers (observable-output true) (reachable-at-privilege machine)))
(defblock own.clock (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))
(defplatform soc.here
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses own.out own.clock)))
(defservice a.write (requires (needs observable-output)))
(defservice b.read (requires (needs tick-rate)))
(defsystem s
  (requires (uses a.write b.read))
  (platform (uses soc.here)))
";
    let one_machine = "
(defblock t.both (offers (observable-output true) (tick-rate 10 MHz) (reachable-at-privilege machine)))
(defplatform t
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses t.both)))
";
    let (_, _, found) = both(two, &target_of(one_machine));
    assert_eq!(
        map(&found),
        BTreeMap::from([("own.clock", vec!["t.both"]), ("own.out", vec!["t.both"])])
    );
}

#[test]
fn what_one_plan_alone_binds_is_said_as_that() {
    // A device of the description with a requirement of its own, which the target's hardware does not ask, and another bound only for it.
    let own_requirement = "
(defblock timer.block (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))
(defblock gate.block (offers (reachable-at-privilege machine)) (requires (needs tick-rate)))
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc.here
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses timer.block gate.block console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s (requires (uses console.write)) (platform (uses soc.here)))
";
    let (_, _, found) = both(own_requirement, &target_of(A_WHOLE_TARGET));
    assert_eq!(found.only_described.len(), 1, "{found:?}");
    assert_eq!(found.only_described[0].1, "tick-rate", "{found:?}");
    assert!(found.only_on_target.is_empty(), "{found:?}");
    // The device bound only for that is among the devices, with nothing beside it, and is not called unreached.
    assert_eq!(
        map(&found).get("timer.block"),
        Some(&Vec::new()),
        "{found:?}"
    );
    assert!(!found.unreached.contains("timer.block"), "{found:?}");
    assert!(found.unreached.contains("gate.block"), "{found:?}");
    // And a target's device with a requirement of its own.
    let their_requirement = "
(defblock t.timer (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))
(defblock t.gate (offers (observable-output true) (reachable-at-privilege machine)) (requires (needs tick-rate)))
(defplatform t
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses t.timer t.gate)))
";
    let (_, _, found) = both(A_SYSTEM, &target_of(their_requirement));
    assert_eq!(found.only_on_target.len(), 1, "{found:?}");
    assert_eq!(found.only_on_target[0].1, "tick-rate", "{found:?}");
    assert!(found.only_described.is_empty(), "{found:?}");
}

#[test]
fn what_a_platform_clause_holds_beside_its_hardware_stays_the_system_s() {
    // Over the hardware of its own it is met, and on a target it is judged again, as it would be anywhere else in the system.
    let with = |clause: &str| {
        format!(
            "
(defblock fast.counter (offers (counter-width 256 bit) (reachable-at-privilege machine)))
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc.here
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses fast.counter console.uart)))
(defservice console.write (requires (needs observable-output)))
(defservice time.read (requires (counter-width (at-least 128 bit))))
(defsystem s
  (requires (uses console.write))
  (platform {clause}))
"
        )
    };
    let refused_there = |text: &str| {
        let declarations = forms(text);
        let system = system_of(&declarations);
        assert!(
            matches!(run(&declarations, system), Search::Resolved(_)),
            "its own hardware meets it"
        );
        let moved = on_target(system, &declarations, &target_of(A_WHOLE_TARGET))
            .expect("it can be read there");
        let Search::Refused { diagnostics, .. } = run(&moved.declarations, &moved.system) else {
            panic!("the clause's requirement was dropped: {text}");
        };
        assert!(
            diagnostics.iter().any(|d| d.code == "missing-fact"),
            "{diagnostics:?}"
        );
    };
    refused_there(&with(
        "(uses soc.here) (requires (counter-width (at-least 128 bit)))",
    ));
    refused_there(&with("(uses soc.here time.read)"));
    // A name of hardware the clause holds anywhere but its own `uses` is a name no requirement stands for.
    let nested = with("(uses soc.here) (requires (uses console.uart))");
    let declarations = forms(&nested);
    let errors = on_target(
        system_of(&declarations),
        &declarations,
        &target_of(A_WHOLE_TARGET),
    )
    .expect_err("a device named");
    assert_eq!(
        codes(&errors),
        vec!["infeasible-configuration"],
        "{errors:?}"
    );
}

#[test]
fn each_file_stands_alone_though_they_are_read_together() {
    let run_on = |description: &str, target: &str| {
        let declarations = forms(description);
        on_target(system_of(&declarations), &declarations, &target_of(target))
    };
    // A target naming what only the description declares would have bound the description's device.
    let errors = run_on(
        A_SYSTEM,
        "(defplatform t (offers (core-count 1 tick)) (requires (uses console.uart)))",
    )
    .expect_err("a target's name found in the description");
    assert_eq!(codes(&errors), vec!["invalid-description"], "{errors:?}");
    assert!(errors[0].message.contains("`console.uart`"), "{errors:?}");
    let errors = run_on(
        A_SYSTEM,
        "(defplatform t (refines soc.here) (offers (core-count 1 tick)))",
    )
    .expect_err("a target refining the description's platform");
    assert_eq!(codes(&errors), vec!["invalid-description"], "{errors:?}");
    // A description naming the target's device would read only beside that target.
    for naming in [
        A_SYSTEM.replace(
            "(requires (uses console.write))",
            "(requires (uses console.write target.console))",
        ),
        A_SYSTEM.replace(
            "(defservice console.write (requires (needs observable-output)))",
            "(defservice console.write (requires (needs observable-output) (uses target.console)))",
        ),
        A_SYSTEM.replace(
            "(platform (uses soc.here))",
            "(platform (uses target.machine))",
        ),
    ] {
        let errors =
            run_on(&naming, A_WHOLE_TARGET).expect_err("a description naming the target's device");
        assert!(
            codes(&errors).contains(&"infeasible-configuration"),
            "{naming}: {errors:?}"
        );
    }
}

#[test]
fn what_no_target_device_offers_is_the_search_s_refusal() {
    // `high-interference` is released by a source, and this target has none.
    let declarations = forms(&example("high-interference"));
    let system = system_of(&declarations);
    let moved =
        on_target(system, &declarations, &target_of(A_WHOLE_TARGET)).expect("it can be read there");
    let Search::Refused { diagnostics, .. } = run(&moved.declarations, &moved.system) else {
        panic!("a plan with no source");
    };
    let said: String = diagnostics.iter().map(|d| d.message.as_str()).collect();
    assert!(said.contains("interrupt-source"), "{said}");
}

#[test]
fn a_system_or_a_task_naming_hardware_itself_is_refused_with_what_to_do() {
    for text in [
        A_SYSTEM.replace("(requires (uses console.write))", "(requires (uses console.write console.uart))"),
        A_SYSTEM.replace(
            "(requires (uses console.write))",
            "(task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.uart) (on-overrun fault))
             (requires (uses console.write))",
        ),
    ] {
        let declarations = forms(&text);
        let errors = on_target(system_of(&declarations), &declarations, &real_target())
            .expect_err("named itself");
        assert_eq!(codes(&errors), vec!["infeasible-configuration"], "{errors:?}");
        assert!(errors[0].message.contains("names `console.uart` itself"), "{errors:?}");
        assert!(errors[0].repair.contains("`needs`"), "{errors:?}");
    }
}

#[test]
fn a_service_it_reaches_naming_hardware_is_refused_like_the_system_with_a_repair_that_works() {
    let text = A_SYSTEM.replace(
        "(defservice console.write (requires (needs observable-output)))",
        "(defservice console.write (requires (needs observable-output) (uses console.uart)))",
    );
    let declarations = forms(&text);
    let errors = on_target(system_of(&declarations), &declarations, &real_target())
        .expect_err("a device named by what the system reaches");
    assert_eq!(
        codes(&errors),
        vec!["infeasible-configuration"],
        "{errors:?}"
    );
    assert!(
        errors[0]
            .message
            .contains("`console.write` names `console.uart`"),
        "{errors:?}"
    );
    assert!(
        errors[0].repair.contains("say what is asked of it"),
        "{errors:?}"
    );
    // The repair it gives: the service asked only by what it asks, and it reads there.
    assert!(on_target(
        system_of(&forms(A_SYSTEM)),
        &forms(A_SYSTEM),
        &real_target()
    )
    .is_ok());
    // Nor is one it reached only through the hardware the target replaces: it is gone with that hardware.
    let through = "
(defblock console.uart
  (offers (observable-output true) (reachable-at-privilege machine))
  (requires (uses helper.service)))
(defservice helper.service (requires (uses console.uart)))
(defplatform soc.here
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (requires (uses console.write))
  (platform (uses soc.here)))
";
    let declarations = forms(through);
    let there = on_target(system_of(&declarations), &declarations, &real_target())
        .expect("it can be read there");
    assert!(there.presence().is_empty(), "{:?}", there.presence());
    // A service the system does not reach says nothing of it, to the search or to presence.
    let unreached = format!("{A_SYSTEM} (defservice unused.one (requires (uses console.uart)))");
    let declarations = forms(&unreached);
    let there = on_target(system_of(&declarations), &declarations, &real_target())
        .expect("it can be read there");
    assert!(there.presence().is_empty(), "{:?}", there.presence());
}

/// A description whose hardware offers `sha-accel`, which no target here does, and no vocabulary declares.
const ACCEL: &str = "
(defblock accel (offers sha-accel (reachable-at-privilege machine)))
(defplatform soc.here
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses accel)))
";

#[test]
fn a_need_outside_the_vocabulary_is_judged_wherever_the_system_writes_it() {
    for system in [
        "(defsystem s (requires (needs sha-accel)) (platform (uses soc.here)))",
        "(defsystem s (needs sha-accel) (platform (uses soc.here)))",
        "(defsystem s (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (needs sha-accel)) (platform (uses soc.here)))",
        "(defsystem s (platform (uses soc.here) (needs sha-accel)))",
        "(defservice hash (requires (needs sha-accel))) (defsystem s (requires (needs hash)) (platform (uses soc.here)))",
        "(defservice hash (requires (needs sha-accel))) (defsystem s (requires (uses hash)) (platform (uses soc.here)))",
    ] {
        let declarations = forms(&format!("{ACCEL} {system}"));
        let there = on_target(system_of(&declarations), &declarations, &real_target())
            .expect("it can be read there");
        assert_eq!(codes(&there.presence()), ["missing-fact"], "{system}");
    }
}

#[test]
fn what_the_system_does_not_reach_neither_asks_nor_answers() {
    let console = "
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc.here
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s (requires (uses console.write)) (platform (uses soc.here)))
";
    for beside in [
        // A service naming the replaced block; one using another that needs what no machine here has; a policy naming the replaced platform.
        "(defservice unused.one (requires (uses console.uart)))",
        "(defservice u.one (requires (uses u.two))) (defservice u.two (requires (needs sha-accel)))",
        "(defpolicy p.unused (requires (uses soc.here)))",
        // A service only another system reaches, naming that system's block: gone with it.
        "(defblock o.dev (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))
         (defplatform o.soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses o.dev)))
         (defservice o.svc (requires (uses o.dev)))
         (defsystem other (requires (uses o.svc)) (platform (uses o.soc)))",
    ] {
        let declarations = forms(&format!("{console} {beside}"));
        let system = declarations
            .iter()
            .find(|form| form.items().get(1).and_then(Form::as_symbol) == Some("s"))
            .expect("the system");
        let there = on_target(system, &declarations, &real_target()).expect("read there");
        assert!(
            there.presence().is_empty(),
            "{beside}: {:?}",
            there.presence()
        );
    }
    // What the machine lacks, a service's offer does not answer, reached or not: the search binds it nothing, and the plan would be for a machine without the device.
    for beside in [
        "(defservice sha.driver (offers sha-accel) (requires (uses console.uart)))
         (defsystem t (requires (needs sha-accel)) (platform (uses soc.here)))",
        "(defservice hash (requires (needs sha-accel))) (defservice offers.it (offers sha-accel))
         (defsystem t (requires (uses hash)) (platform (uses soc.here)))",
        "(defservice offers.it (offers sha-accel))
         (defsystem t (requires (uses offers.it) (needs sha-accel)) (platform (uses soc.here)))",
    ] {
        let declarations = forms(&format!("{console} {beside}"));
        let system = declarations
            .iter()
            .find(|form| form.items().get(1).and_then(Form::as_symbol) == Some("t"))
            .expect("the system");
        let there = on_target(system, &declarations, &real_target()).expect("read there");
        assert_eq!(codes(&there.presence()), ["missing-fact"], "{beside}");
    }
}

#[test]
fn what_the_profile_excludes_is_not_said_again_as_missing() {
    let declarations = forms(&example("bounded-queue"));
    let there = on_target(
        system_of(&declarations),
        &declarations,
        &target_of(A_WHOLE_TARGET),
    )
    .expect("it can be read there");
    assert!(
        there
            .presence()
            .iter()
            .all(|d| !d.message.contains("general-ipc")),
        "{:?}",
        there.presence()
    );
}

#[test]
fn two_platform_clauses_are_refused_rather_than_read_as_one() {
    let declarations = forms(
        "(defplatform a (offers (core-count 1 tick) (reachable-at-privilege machine)))
         (defplatform b (offers (core-count 1 tick) (reachable-at-privilege machine)))
         (defsystem s (platform (uses a)) (platform (uses b)))",
    );
    let refused = on_target(system_of(&declarations), &declarations, &real_target())
        .expect_err("two clauses");
    assert_eq!(codes(&refused), ["schema-cardinality"]);
}

#[test]
fn what_a_clause_stands_on_is_read_by_composition_and_refinement_alone() {
    let target = "
(defblock target.timer (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))
(defblock target.console (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform target.machine
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses target.timer target.console)))
";
    let read = |text: &str| {
        let declarations = forms(&format!(
            "(defservice console.write (requires (needs observable-output))) {text}"
        ));
        on_target(system_of(&declarations), &declarations, &target_of(target)).map(|_| ())
    };
    let q = "(defblock q.block (offers (observable-output true) (reachable-at-privilege machine)))
             (defplatform p.top (absent tick-rate) (offers (core-count 1 tick)) (requires (needs q.block)))";
    // What it `needs` is no part of it, so its absence is not the machine's, one clause naming both or naming it alone: the system has that block whichever.
    assert!(read(&format!(
        "{q} (defsystem s (requires (uses console.write)) (platform (uses p.top q.block)))"
    ))
    .is_ok());
    assert!(read(&format!(
        "{q} (defsystem s (requires (uses console.write)) (platform (uses p.top)))"
    ))
    .is_ok());
    // What it composes is of it, so named alone it is the machine.
    let composed = "(defblock q.part (offers (observable-output true) (reachable-at-privilege machine)))
                    (defplatform p.whole (absent tick-rate) (offers (core-count 1 tick)) (requires (uses q.part)))";
    assert_eq!(
        codes(
            &read(&format!(
                "{composed} (defsystem s (requires (uses console.write)) (platform (uses p.whole)))"
            ))
            .expect_err("alone")
        ),
        ["refinement-violated"]
    );
    // What it refines says it again and brings no blocks: what that one composes is not of it, so the block beside it is a second.
    let refining = "(defblock b.timer (offers (tick-rate 1 MHz) (reachable-at-privilege machine)))
                    (defplatform x.up (offers (core-count 1 tick)) (requires (uses b.timer)))
                    (defplatform a.top (refines x.up) (absent tick-rate) (offers (core-count 1 tick) (observable-output true)))";
    assert!(read(&format!(
        "{refining} (defsystem s (requires (uses console.write)) (platform (uses a.top b.timer)))"
    ))
    .is_ok());
    // A cycle named whole is one machine, whichever of it holds the absence and whatever each is called.
    let cycle =
        "(defblock c.a (absent tick-rate) (offers (observable-output true)) (requires (uses c.b)))
                 (defblock c.b (requires (uses c.a)))";
    let later = "(defblock c.a (offers (observable-output true)) (requires (uses c.b)))
         (defblock c.b (absent tick-rate) (requires (uses c.a)))";
    for clause in ["c.a c.b", "c.b c.a", "c.a", "c.b"] {
        assert_eq!(
            codes(
                &read(&format!(
                    "{later} (defsystem s (requires (uses console.write)) (platform (uses {clause})))"
                ))
                .expect_err("the cycle's absence")
            ),
            ["refinement-violated"],
            "{clause}"
        );
    }
    for clause in ["c.a c.b", "c.b c.a", "c.a"] {
        assert_eq!(
            codes(
                &read(&format!(
                    "{cycle} (defsystem s (requires (uses console.write)) (platform (uses {clause})))"
                ))
                .expect_err("the cycle's absence")
            ),
            ["refinement-violated"],
            "{clause}"
        );
    }
    // A platform and what it refines, both named, read as the one that refines it.
    let chain = "(defplatform y.up (absent tick-rate) (offers (core-count 1 tick)))
                 (defplatform y.p (refines y.up) (offers (core-count 1 tick) (observable-output true)))";
    for clause in ["y.p y.up", "y.p"] {
        assert_eq!(
            codes(
                &read(&format!(
                    "{chain} (defsystem s (requires (uses console.write)) (platform (uses {clause})))"
                ))
                .expect_err("the chain's absence")
            ),
            ["refinement-violated"],
            "{clause}"
        );
    }
}

#[test]
fn a_target_s_file_composes_by_uses_alone() {
    let refused = Target::read(
        &forms(
            "(defblock only.needed (offers (tick-rate 10 MHz)))
             (defplatform t (requires (needs only.needed)))",
        ),
        file_start(),
    )
    .expect_err("a `needs` is no composition");
    assert_eq!(codes(&refused), ["invalid-description"]);
}

#[test]
fn a_platform_s_absence_is_kept_and_a_block_s_speaks_for_the_block() {
    let absent_on = |text: &str| {
        let declarations = forms(text);
        on_target(
            system_of(&declarations),
            &declarations,
            &target_of(A_WHOLE_TARGET),
        )
    };
    // A platform says this of all it composes, so the target owes it, its blocks included.
    let on_platform = A_SYSTEM.replace(
        "(defplatform soc.here\n  (offers",
        "(defplatform soc.here\n  (absent tick-rate)\n  (offers",
    );
    let errors = absent_on(&on_platform).expect_err("the target offers it");
    assert_eq!(codes(&errors), vec!["refinement-violated"], "{errors:?}");
    assert!(
        errors[0].message.contains("`target.timer` of the target"),
        "{errors:?}"
    );
    // Up the chain it refines too.
    let chained = format!(
        "(defplatform soc.up (absent tick-rate)) {}",
        A_SYSTEM.replace(
            "(defplatform soc.here",
            "(defplatform soc.here (refines soc.up)"
        )
    );
    assert_eq!(
        codes(&absent_on(&chained).expect_err("kept")),
        vec!["refinement-violated"]
    );
    // What a composed platform says it says of what it is made of, and another device beside it may offer the fact: the same hardware passes as its own.
    let composed = "
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc.sub
  (absent tick-rate)
  (offers (reachable-at-privilege machine))
  (requires (uses console.uart)))
(defblock timer.block (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))
(defplatform soc.top
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses soc.sub timer.block)))
(defservice console.write (requires (needs observable-output)))
(defsystem s (requires (uses console.write)) (platform (uses soc.top)))
";
    assert!(matches!(
        run(&forms(composed), system_of(&forms(composed))),
        Search::Resolved(_)
    ));
    assert!(absent_on(composed).is_ok());
    // Nor does one of two platforms the clause names speak for the machine.
    let two_named = "
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc.a
  (absent tick-rate)
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart)))
(defplatform soc.b (offers (tick-rate 10 MHz) (reachable-at-privilege machine)))
(defservice console.write (requires (needs observable-output)))
(defsystem s (requires (uses console.write)) (platform (uses soc.a soc.b)))
";
    assert!(absent_on(two_named).is_ok());
    // The one a clause stands on, however it is written: named twice, or beside a part of itself.
    for clause in ["(uses soc.here soc.here)", "(uses soc.here console.uart)"] {
        let twice = on_platform.replace(
            "(platform (uses soc.here))",
            &format!("(platform {clause})"),
        );
        assert_eq!(
            codes(&absent_on(&twice).expect_err("the same machine")),
            vec!["refinement-violated"],
            "{clause}"
        );
    }
    // A lone block is the machine, and what it refines goes with it, a block's absence up a chain among it.
    let lone = "
(defblock only.one
  (absent tick-rate)
  (offers (observable-output true) (reachable-at-privilege machine)))
(defservice console.write (requires (needs observable-output)))
(defsystem s (requires (uses console.write)) (platform (uses only.one)))
";
    assert_eq!(
        codes(&absent_on(lone).expect_err("all of it")),
        vec!["refinement-violated"]
    );
    let up_a_block = format!(
        "(defblock soc.up (absent tick-rate)) {}",
        A_SYSTEM.replace(
            "(defplatform soc.here",
            "(defplatform soc.here (refines soc.up)"
        )
    );
    assert_eq!(
        codes(&absent_on(&up_a_block).expect_err("kept")),
        vec!["refinement-violated"]
    );
    // A block says it of itself alone: another device may offer the fact.
    let on_block = A_SYSTEM.replace(
        "(defblock console.uart\n  (offers",
        "(defblock console.uart\n  (absent tick-rate)\n  (offers",
    );
    assert!(absent_on(&on_block).is_ok());
}

#[test]
fn what_the_target_replaces_is_gone_and_cannot_answer_for_it() {
    // A need on a fact the vocabulary does not declare: the search binds nothing for it, and presence is what judges it.
    let text = "
(defblock accel (offers sha-accel (reachable-at-privilege machine)))
(defplatform soc.here
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses accel)))
(defservice hash (requires (needs sha-accel)))
(defsystem s (requires (uses hash)) (platform (uses soc.here)))
";
    let declarations = forms(text);
    let moved = on_target(system_of(&declarations), &declarations, &real_target())
        .expect("it can be read there");
    assert!(
        moved.declarations.iter().all(|form| !matches!(
            form.items().get(1).and_then(Form::as_symbol),
            Some("accel" | "soc.here")
        )),
        "the description's hardware reaches the target"
    );
    let found = moved.presence();
    assert_eq!(codes(&found), vec!["missing-fact"], "{found:?}");
    assert!(found[0].message.contains("sha-accel"), "{found:?}");
    // A target that offers it answers.
    let offering = "
(defblock t.accel (offers sha-accel (reachable-at-privilege machine)))
(defplatform t
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses t.accel)))
";
    let there = on_target(
        system_of(&declarations),
        &declarations,
        &target_of(offering),
    )
    .expect("it can be read there");
    assert!(there.presence().is_empty(), "{:?}", there.presence());
    // Another system of the description does not come along, nor does what it says.
    let two = format!("{text} (defsystem other (requires (needs nothing-at-all)))");
    let declarations = forms(&two);
    let system = declarations
        .iter()
        .find(|form| form.items().get(1).and_then(Form::as_symbol) == Some("s"))
        .expect("the system");
    let there =
        on_target(system, &declarations, &target_of(offering)).expect("it can be read there");
    assert!(there.presence().is_empty(), "{:?}", there.presence());
    // Nor does a description need a clause, or a device of its own for what it asks: judged on the target alone.
    let no_clause = forms(&text.replace("(platform (uses soc.here))", ""));
    let there = on_target(system_of(&no_clause), &no_clause, &target_of(offering))
        .expect("it can be read there");
    assert!(there.presence().is_empty(), "{:?}", there.presence());
}

#[test]
fn a_name_both_declare_is_refused_rather_than_taken_for_one_device() {
    let text = A_SYSTEM.replace("console.uart", "target.console");
    let declarations = forms(&text);
    let errors = on_target(system_of(&declarations), &declarations, &real_target())
        .expect_err("two declarations of one name");
    assert_eq!(codes(&errors), vec!["schema-duplicate-name"], "{errors:?}");
}

#[test]
fn a_system_with_no_platform_clause_is_given_the_target_s() {
    let text = A_SYSTEM.replace("(platform (uses soc.here))", "");
    let declarations = forms(&text);
    let moved = on_target(system_of(&declarations), &declarations, &real_target())
        .expect("it can be read there");
    let there = resolved(run(&moved.declarations, &moved.system));
    assert_eq!(
        there
            .bindings
            .values()
            .map(String::as_str)
            .collect::<Vec<_>>(),
        vec!["target.console"]
    );
}

#[test]
fn every_example_gets_an_answer_on_the_target_and_none_is_silently_the_description_s() {
    let target = target_of(A_WHOLE_TARGET);
    for name in [
        "alternative-timer",
        "bounded-queue",
        "high-interference",
        "periodic-three",
        "s0-heartbeat",
    ] {
        let declarations = forms(&example(name));
        let system = system_of(&declarations);
        let moved = on_target(system, &declarations, &target).expect("it can be read there");
        match run(&moved.declarations, &moved.system) {
            Search::Resolved(there) => assert!(
                there.bindings.values().all(|provider| {
                    let device = there
                        .instances
                        .iter()
                        .find(|instance| instance.provider.name == *provider)
                        .map_or(provider.as_str(), |instance| instance.over.as_str());
                    device.starts_with("target.")
                }),
                "{name}: {:?}",
                there.bindings
            ),
            Search::Refused { diagnostics, .. } => {
                // Each waits on `M3.7`, or on a later profile, as it does on its own hardware.
                assert!(!diagnostics.is_empty(), "{name}");
                assert!(
                    diagnostics.iter().all(|d| d.code == "missing-fact"),
                    "{name}: {diagnostics:?}"
                );
            }
            Search::Inconclusive { .. } => panic!("{name}: the budget spent"),
        }
    }
}

/// A machine that offers what most of the corpus asks.
const A_RICH_TARGET: &str = "
(defblock target.timer (offers (relative-delay true) (channels 4 tick) (counter-width 64 bit) (counter-modulus (pow2 64)) (tick-rate 10 MHz) (tick-unit ns) (wrap-behavior modular) (observation-coherent true) (reachable-at-privilege machine) (region target.timer (base 0x0200_0000) (size 64 KiB))))
(defblock target.console (offers (observable-output true) (uart true) (reachable-at-privilege machine) (region target.console (base 0x1000_0000) (size 256 byte))))
(defblock target.irq (offers (interrupt-source true) (min-arrival-separation 1 ms) (reachable-at-privilege machine) (region target.irq (base 0x0c00_0000) (size 64 KiB))))
(defplatform target.machine (offers (core-count 1 tick) (preemptive true) (reachable-at-privilege machine) (region target.ram (base 0x8000_0000) (size 128 MiB) (executable true))) (requires (uses target.timer target.console target.irq)))
";

fn eadl_files_under(dir: &std::path::Path, into: &mut Vec<PathBuf>) {
    let Ok(entries) = std::fs::read_dir(dir) else {
        return;
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if path.is_dir() {
            eadl_files_under(&path, into);
        } else if path
            .extension()
            .is_some_and(|extension| extension == "eadl")
        {
            into.push(path);
        }
    }
}

/// The rule held over every system the repository's own trees describe, rather than over shapes written for it: what comes on a target binds only that target's devices, and presence refuses nothing the
/// search resolves, so `DC-H3`'s order refuses nothing falsely. A sweep and not a list, so a description added tomorrow is in it. Its roots keep it to the examples and the crates' own fixtures.
#[test]
fn every_system_in_the_repository_reads_on_a_target_as_the_rule_says() {
    let target = target_of(A_RICH_TARGET);
    let mut files = Vec::new();
    for dir in [
        "examples",
        "crates/eadl-resolve/tests",
        "crates/archogen-plan/tests/corpus",
        "targets",
    ] {
        eadl_files_under(&root().join(dir), &mut files);
    }
    files.sort();
    let (mut systems, mut resolved) = (0_u32, 0_u32);
    for file in &files {
        let Ok(text) = std::fs::read_to_string(file) else {
            continue;
        };
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", text.as_str()).expect("small");
        let (doc, diagnostics) = read(&sources, id);
        if diagnostics.has_errors() {
            continue;
        }
        let declarations: Vec<Form> = doc
            .forms
            .into_iter()
            .filter(|form| form.head() != Some("eadl-version"))
            .collect();
        for system in declarations
            .iter()
            .filter(|form| form.head() == Some("defsystem"))
        {
            systems += 1;
            let Ok(there) = on_target(system, &declarations, &target) else {
                continue;
            };
            if let Search::Resolved(plan) = run(&there.declarations, &there.system) {
                resolved += 1;
                let said = file.display();
                assert!(
                    there.presence().is_empty(),
                    "{said}: presence refuses what the search resolves: {:?}",
                    there.presence()
                );
                for provider in plan.bindings.values() {
                    let device = plan
                        .instances
                        .iter()
                        .find(|instance| instance.provider.name == *provider)
                        .map_or(provider.as_str(), |instance| instance.over.as_str());
                    assert!(device.starts_with("target."), "{said}: bound to {device}");
                }
            }
        }
    }
    assert!(systems >= 180, "only {systems} systems");
    assert!(resolved >= 100, "only {resolved} resolved");
}

#[test]
fn another_system_named_is_no_missing_fact_and_presence_keeps_its_metadata() {
    let declarations = forms(
        "(defplatform o.soc (offers (core-count 1 tick) (reachable-at-privilege machine)))
         (defsystem other (platform (uses o.soc)))
         (defsystem s (requires (uses other)) (platform (uses o.soc)))",
    );
    let system = declarations
        .iter()
        .find(|form| form.items().get(1).and_then(Form::as_symbol) == Some("s"))
        .expect("the system");
    let there = on_target(system, &declarations, &real_target()).expect("read there");
    assert!(there.presence().is_empty(), "{:?}", there.presence());
    assert!(there.report().closure.iter().any(|name| name == "other"));
}

#[test]
fn what_refines_what_it_composes_or_is_made_of_is_refused() {
    for text in [
        "(defplatform b (absent tick-rate) (offers (core-count 1 tick)) (requires (uses c)))
         (defblock c (requires (uses a)))
         (defblock a (refines b))
         (defsystem s (platform (uses b)))",
        "(defplatform p (offers (core-count 1 tick)) (requires (uses q)))
         (defblock q (refines p) (absent tick-rate))
         (defsystem s (platform (uses p)))",
    ] {
        let declarations = forms(text);
        let refused = on_target(system_of(&declarations), &declarations, &real_target())
            .expect_err("no machine stands on the two");
        assert!(
            codes(&refused).contains(&"invalid-description"),
            "{refused:?}"
        );
    }
}
