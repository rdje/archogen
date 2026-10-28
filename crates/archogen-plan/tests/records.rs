//! Hand-written `archogen-assignment/2` records, as no search writes them (`docs/decisions/decision_runtime-plan.md`):
//! §1's refusals of a record, before anything is lowered, each with its code and taken in the order §1 writes them;
//! and a record holding every class of §2.5's order. `"PROGRAM"` in a record stands for the digest of the description's
//! program.

mod common;

use common::{lowered_record, root};

/// One task logging through a console: the system `dg`.
const DESCRIPTION: &str = "(eadl-version eadl/1)
(defblock console.uart
  (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform host.p
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart)))
(defservice console.write
  (requires (needs observable-output)))
(defsystem dg
  (task a (period 10 ms) (deadline 5 ms) (deadline-from release) (priority 1) (uses console.write))
  (platform (uses host.p)))
";

/// A record for `dg` with its header's values as given, and its one binding.
fn record(system: &str, program: &str, profile: &str, adapters: &str, extra: &str) -> String {
    format!(
        "(format \"archogen-assignment/2\")\n(profile {profile})\n(system {system})\n(program \"{program}\")\n\
         (adapters {adapters})\n(bind console.write observable-output console.uart)\n{extra}"
    )
}

/// The refusal's code and message, for a record the lowering must refuse.
fn refused(description: &str, record: &str) -> (&'static str, String) {
    let refusal = lowered_record(description, record).expect_err("refused");
    (refusal.code, refusal.message)
}

#[test]
fn the_record_as_written_lowers() {
    let plan = lowered_record(
        DESCRIPTION,
        &record("dg", "PROGRAM", "rt-static-up-v1", "adapters/1", ""),
    )
    .expect("lowered");
    assert!(plan.contains("(task 0 (task dg a) (rank 1) (period 1/100 s) (deadline 1/200 s)"));
}

#[test]
fn a_record_that_does_not_read_is_a_tool_failure() {
    let (code, message) = refused(DESCRIPTION, "(format \"archogen-assignment/9\")\n");
    assert_eq!(code, "tool-failure");
    assert!(message.contains("does not read"), "{message}");
}

#[test]
fn another_program_s_digest_is_stale() {
    let other = "0".repeat(64);
    let (code, message) = refused(
        DESCRIPTION,
        &record("dg", &other, "rt-static-up-v1", "adapters/1", ""),
    );
    assert_eq!(code, "stale-input");
    assert!(message.contains("another program's digest"), "{message}");
}

#[test]
fn a_system_the_program_does_not_declare_is_stale_and_comes_before_the_catalog() {
    let (code, message) = refused(
        DESCRIPTION,
        &record("ghost", "PROGRAM", "rt-static-up-v1", "adapters/2", ""),
    );
    assert_eq!(code, "stale-input");
    assert!(
        message.contains("`ghost`, which the program does not declare"),
        "{message}"
    );
}

#[test]
fn another_catalog_is_stale_and_comes_before_the_profile() {
    for profile in ["rt-static-up-v1", "rt-other-v9"] {
        let (code, message) = refused(
            DESCRIPTION,
            &record("dg", "PROGRAM", profile, "adapters/2", ""),
        );
        assert_eq!(code, "stale-input", "{profile}");
        assert!(message.contains("`adapters/2`"), "{message}");
    }
}

#[test]
fn another_profile_is_unsupported() {
    let (code, message) = refused(
        DESCRIPTION,
        &record("dg", "PROGRAM", "rt-other-v9", "adapters/1", ""),
    );
    assert_eq!(code, "unsupported-profile");
    assert!(message.contains("`rt-other-v9`"), "{message}");
}

#[test]
fn a_record_that_does_not_rebuild_is_a_tool_failure() {
    let (code, message) = refused(
        DESCRIPTION,
        &record(
            "dg",
            "PROGRAM",
            "rt-static-up-v1",
            "adapters/1",
            "(bind (task dg a) observable-output console.uart)\n",
        ),
    );
    assert_eq!(code, "tool-failure");
    assert!(message.contains("does not rebuild"), "{message}");
}

#[test]
fn the_joint_rules_refusing_the_accepted_record_name_their_findings() {
    let description =
        std::fs::read_to_string(root().join("crates/archogen-plan/tests/corpus/disagree.eadl"))
            .expect("the fixture");
    let (code, message) = refused(
        &description,
        "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system dis)\n(program \"PROGRAM\")\n\
         (adapters adapters/1)\n(bind alarm.a relative-delay timer.one)\n(bind alarm.b relative-delay timer.one)\n",
    );
    assert_eq!(code, "tool-failure");
    assert!(message.contains("infeasible-configuration: "), "{message}");
    assert!(message.contains("`timer.one`"), "{message}");
    assert!(message.ends_with("the two roles disagree"), "{message}");
}

#[test]
fn every_class_of_the_route_order_in_one_system() {
    // §2.5: the sources with a stated separation ascending, then those whose separation is `unknown` or `absent` by
    // their text, then the timer routes. The search binds every source's service to one device, so the record binds
    // each to its own.
    let description =
        std::fs::read_to_string(root().join("crates/archogen-plan/tests/corpus/rank-classes.eadl"))
            .expect("the fixture");
    let plan = lowered_record(
        &description,
        "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system rc)\n(program \"PROGRAM\")\n\
         (adapters adapters/1)\n(bind alarm relative-delay timer.a)\n(bind ea interrupt-source dev.a)\n\
         (bind eb interrupt-source dev.b)\n(bind ec interrupt-source dev.c)\n(bind ez interrupt-source dev.z)\n",
    )
    .expect("lowered");
    let routes: Vec<&str> = plan
        .lines()
        .filter(|l| l.starts_with("(route "))
        .map(|l| l.split(" (level ").next().expect("a route line"))
        .collect();
    assert_eq!(
        routes,
        [
            "(route 1 (source dev.c) (separation 1/200 s)",
            "(route 2 (source dev.z) (separation 1/20 s)",
            "(route 3 (source dev.a) (separation unknown)",
            "(route 4 (source dev.b) (separation absent)",
            "(route 5 (timer-route timer.a) (separation programmed)",
        ]
    );
    assert!(plan.contains(
        "(enabled timer (source dev.c) (source dev.z) (source dev.a) (source dev.b) (timer-route timer.a))\n"
    ));
}

#[test]
fn a_record_binding_an_instance_no_served_requester_is_bound_to_does_not_rebuild() {
    // Leaf `M4.1.10`: the checker refuses it, and the lowering once wrote the instance's obligations, owed by no one.
    let description = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defblock timer.short
  (offers (relative-delay true) (counter-modulus 65536) (tick-rate 10 MHz) (wrap-behavior modular)
    (observation-coherent true) (channels 1 tick) (reachable-at-privilege machine)))
(defplatform host.p
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart timer.short)))
(defservice console.write (requires (needs observable-output)))
(defsystem orph
  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (platform (uses host.p)))
";
    let mut record = "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system orph)\n(program \"PROGRAM\")\n\
                      (adapters adapters/1)\n(bind console.write observable-output console.uart)\n"
        .to_string();
    for fact in [
        "counter-modulus",
        "observation-coherent",
        "relative-delay",
        "tick-rate",
        "wrap-behavior",
    ] {
        record.push_str(&format!(
            "(bind (adapter epoch-extender timer.short) {fact} timer.short)\n"
        ));
    }
    let (code, message) = refused(description, &record);
    assert_eq!(code, "tool-failure");
    assert!(message.contains("does not rebuild"), "{message}");
    assert!(
        message.contains("which no requester `orph` serves is bound to"),
        "{message}"
    );
}

#[test]
fn a_record_binding_another_requester_s_exclusive_instance_does_not_rebuild() {
    // Leaf `M4.1.10`: the lowering once wrote `ghost`'s bound as `dl`'s.
    let description = "(eadl-version eadl/1)
(defblock timer.delay
  (offers (relative-delay true) (counter-modulus 4294967296) (tick-rate 1 MHz) (observation-coherent true)
    (channels 1 tick) (reachable-at-privilege machine)))
(defplatform soc.d (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.delay)))
(defservice dl (requires (needs absolute-deadline)))
(defservice ghost (requires (absolute-deadline (delivery-bound (at-most 7 us)))))
(defsystem mi
  (task x (period 25 ms) (deadline 25 ms) (deadline-from release) (priority 1) (uses dl))
  (platform (uses soc.d)))
";
    let ghost = "(adapter deadline-over-delay timer.delay ghost)";
    let mut record = format!(
        "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system mi)\n(program \"PROGRAM\")\n\
         (adapters adapters/1)\n(bind dl absolute-deadline {ghost})\n"
    );
    for fact in [
        "counter-modulus",
        "observation-coherent",
        "relative-delay",
        "tick-rate",
    ] {
        record.push_str(&format!("(bind {ghost} {fact} timer.delay)\n"));
    }
    let (code, message) = refused(description, &record);
    assert_eq!(code, "tool-failure");
    assert!(message.contains("which serves `ghost` alone"), "{message}");
    // Named for `dl`, the instance's bound is `dl`'s, which writes none: the fact offered bare, and owed.
    let plan = lowered_record(description, &record.replace(" ghost)", " dl)")).expect("lowered");
    assert!(plan.contains("(obliged delivery-bound) "), "{plan}");
}

#[test]
fn two_handlers_sharing_a_channel_multiplex_it() {
    // Round 8 (R8-D1): an alarm two sources' handlers call, which no task reaches, has two users, each an interrupt
    // routine (the record's §2.9). The search binds both handlers to one source, so the record binds each to its own.
    let description = "(eadl-version eadl/1)
(defblock dev.a (offers (interrupt-source true) (min-arrival-separation 20 ms) (reachable-at-privilege machine)))
(defblock dev.b (offers (interrupt-source true) (min-arrival-separation 30 ms) (reachable-at-privilege machine)))
(defblock timer.d (offers (relative-delay true) (channels 1 tick) (reachable-at-privilege machine)))
(defplatform soc
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses dev.a dev.b timer.d)))
(defservice alarm (requires (needs relative-delay)))
(defservice evt.a (requires (needs interrupt-source) (uses alarm)))
(defservice evt.b (requires (needs interrupt-source) (uses alarm)))
(defsystem ti
  (task p (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1))
  (requires (uses evt.a evt.b))
  (platform (uses soc)))
";
    let plan = lowered_record(
        description,
        "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system ti)\n(program \"PROGRAM\")\n\
         (adapters adapters/1)\n(bind alarm relative-delay timer.d)\n(bind evt.a interrupt-source dev.a)\n\
         (bind evt.b interrupt-source dev.b)\n",
    )
    .expect("lowered");
    assert!(
        plan.contains(
            "(context alarm (interrupt (source dev.a)) (interrupt (source dev.b)) (interrupt (timer-route timer.d)) initialisation)\n"
        ),
        "{plan}"
    );
    assert!(
        plan.contains("(obligation multiplex alarm (discharged-by M4.2 M2.21.2))\n"),
        "{plan}"
    );
    assert!(
        plan.contains("(owed alarm overhead (by realization))\n"),
        "{plan}"
    );
}

#[test]
fn a_record_binding_a_provider_that_does_not_meet_the_requirement_does_not_rebuild() {
    // Leaf `M4.1.12`: the lowering once wrote a source at the console for the swapped record.
    let description = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defblock dev.e (offers (interrupt-source true) (min-arrival-separation 20 ms) (reachable-at-privilege machine)))
(defplatform host.p
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart dev.e)))
(defservice console.write (requires (needs observable-output)))
(defservice evt (requires (needs interrupt-source)))
(defsystem rel
  (task s (min-separation 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses evt console.write))
  (platform (uses host.p)))
";
    let (code, message) = refused(
        description,
        "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system rel)\n(program \"PROGRAM\")\n\
         (adapters adapters/1)\n(bind console.write observable-output dev.e)\n(bind evt interrupt-source console.uart)\n",
    );
    assert_eq!(code, "tool-failure");
    assert!(message.contains("does not rebuild"), "{message}");
    assert!(message.contains("which does not meet it"), "{message}");
}

#[test]
fn of_two_refusals_the_one_section_1_writes_first_is_given() {
    // Round 9 (R9-R5): a record under another profile that would not rebuild either is refused for its profile, which
    // §1 writes before the rebuild, since the rules run at the profile's caller level.
    let (code, message) = refused(
        DESCRIPTION,
        &record(
            "dg",
            "PROGRAM",
            "rt-other-v9",
            "adapters/1",
            "(bind (task dg a) observable-output console.uart)\n",
        ),
    );
    assert_eq!(code, "unsupported-profile");
    assert!(message.contains("`rt-other-v9`"), "{message}");
}
