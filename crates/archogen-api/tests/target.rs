//! `check` with a target (leaf `M4.1.7.4.1`, API `1.10`): `DC-H3`'s judgement and `DC-H4`'s admission of the machine
//! (`docs/decisions/decision_device-correspondence.md`, `docs/decisions/decision_runtime-plan-format-2.md` §1, §2 and §7), each stage by what it
//! says, over the shipped target's own file and the variants of it each rule refuses.

use std::path::{Path, PathBuf};

use archogen_api::{
    check, check_with, explain_with, resolve_with, Explain, Limits, MemoryModules, ModuleSource,
    NoModules, Request, Resolve, Response, Status, TargetFile,
};

const TARGET: &str = "targets/riscv-virt-up.eadl";

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// The shipped target's file, as it stands.
fn shipped() -> String {
    std::fs::read_to_string(repo_root().join(TARGET)).expect("the shipped target is readable")
}

/// `text` with its one `from` replaced by `to`.
fn edited(text: &str, from: &str, to: &str) -> String {
    assert_eq!(text.matches(from).count(), 1, "`{from}` once in the target");
    text.replacen(from, to, 1)
}

/// A description whose two tasks write to a console, on a playground of its own (`examples/s0-heartbeat`'s forms).
const HEARTBEAT: &str = "
(defblock console.uart
  (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform host.playground
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart)))
(defservice console.write
  (requires (needs observable-output)))
(defsystem heartbeat
  (task beat
    (period 10 ms) (deadline 10 ms) (deadline-from release)
    (priority 1) (uses console.write) (on-overrun fault))
  (task chime
    (period 30 ms) (deadline 30 ms) (deadline-from release)
    (priority 2) (uses console.write) (on-overrun fault))
  (requires (uses console.write))
  (platform (uses host.playground)))
";

fn request<'a>(
    text: &'a str,
    modules: &'a dyn ModuleSource,
    target: Option<&'a str>,
) -> Request<'a> {
    Request {
        name: "system.eadl",
        text,
        profile: None,
        modules,
        target: target.map(|text| TargetFile { name: TARGET, text }),
    }
}

/// `description` checked on `target`.
fn on(description: &str, target: &str) -> Response {
    check(&request(description, &NoModules, Some(target)))
}

/// `description` checked on its own hardware.
fn alone(description: &str) -> Response {
    check(&request(description, &NoModules, None))
}

fn codes(response: &Response) -> Vec<&'static str> {
    response.diagnostics.iter().map(|d| d.code).collect()
}

/// Whether some diagnostic carries `code` and says `words`.
fn says(response: &Response, code: &str, words: &str) -> bool {
    response
        .diagnostics
        .iter()
        .any(|d| d.code == code && d.message.contains(words))
}

#[track_caller]
fn assert_status(what: &str, response: &Response, status: Status) {
    assert_eq!(
        response.status,
        status,
        "{what}: {:?}\n{}",
        codes(response),
        response.render_diagnostics()
    );
    assert!(
        response.judged.is_some(),
        "{what}: a description on a target is judged"
    );
}

#[test]
fn a_system_runs_on_the_shipped_target_and_carries_the_closure_presence_read_there() {
    let response = on(HEARTBEAT, &shipped());
    assert_status("heartbeat on the target", &response, Status::Ok);
    let judged = response.judged.as_ref().expect("judged");
    assert!(
        judged.closure.inside.contains(&(
            "target.console".to_owned(),
            Some("console.write".to_owned())
        )),
        "the machine's console answers the service: {:?}",
        judged.closure.inside
    );
    assert!(
        judged
            .closure
            .outside
            .iter()
            .any(|fact| fact == "console.uart"),
        "the description's own hardware is outside it there: {:?}",
        judged.closure.outside
    );
    assert_eq!(
        judged.declarations.len(),
        4,
        "the description's declarations"
    );
}

/// `DC-H3`: "a description need not pass presence over hardware of its own" (round 2's DC2-2).
#[test]
fn a_description_need_not_pass_presence_over_hardware_of_its_own() {
    let silent = edited(
        HEARTBEAT,
        "(offers (observable-output true) (reachable-at-privilege machine))",
        "(offers (reachable-at-privilege machine))",
    );
    assert_eq!(
        alone(&silent).status,
        Status::MissingFact,
        "its own hardware is silent"
    );
    assert_status("on the target", &on(&silent, &shipped()), Status::Ok);
    let no_clause = edited(HEARTBEAT, "  (platform (uses host.playground)))", ")");
    assert_status(
        "with no clause, on the target",
        &on(&no_clause, &shipped()),
        Status::Ok,
    );
}

/// Presence judges what the search binds nothing for, of what the system asks, at the machine (`OnTarget::report`).
#[test]
fn a_need_the_machine_does_not_answer_is_missing_there() {
    let crc = edited(
        HEARTBEAT,
        "(offers (observable-output true) (reachable-at-privilege machine))",
        "(offers (observable-output true) (crc-engine true) (reachable-at-privilege machine))",
    );
    let crc = edited(
        &crc,
        "(requires (needs observable-output)))",
        "(requires (needs observable-output crc-engine)))",
    );
    assert_eq!(
        alone(&crc).status,
        Status::Ok,
        "{}",
        alone(&crc).render_diagnostics()
    );
    let there = on(&crc, &shipped());
    assert_status("on the target", &there, Status::MissingFact);
    assert!(
        says(&there, "missing-fact", "crc-engine"),
        "{}",
        there.render_diagnostics()
    );
}

/// `DC-H4`, `/2` §2: what the profile asks of a machine, each refusal naming what it refuses.
#[test]
fn the_profile_admits_a_machine_with_no_source_no_mask_stops() {
    for (count, fact) in [
        ("(non-maskable-interrupts 0)", "non-maskable-interrupts"),
        ("(firmware-traps 0)", "firmware-traps"),
    ] {
        let above = on(
            HEARTBEAT,
            &edited(&shipped(), count, &count.replace(" 0)", " 2)")),
        );
        assert_status(fact, &above, Status::UnsupportedProfile);
        assert!(
            says(&above, "unsupported-profile", fact),
            "{}",
            above.render_diagnostics()
        );
        let silent = on(HEARTBEAT, &edited(&shipped(), count, ""));
        assert_status(fact, &silent, Status::MissingFact);
        assert!(
            says(&silent, "missing-fact", fact),
            "{}",
            silent.render_diagnostics()
        );
    }
}

#[test]
fn a_machine_with_no_placement_memory_is_infeasible() {
    let unplaced = on(
        HEARTBEAT,
        &edited(&shipped(), "(executable true)", "(executable false)"),
    );
    assert_status(
        "no placement memory",
        &unplaced,
        Status::InfeasibleConfiguration,
    );
    assert!(
        says(&unplaced, "infeasible-configuration", "no placement memory"),
        "{}",
        unplaced.render_diagnostics()
    );
}

/// One active core, which `check`'s own admission holds of the target's file.
#[test]
fn a_machine_of_two_cores_is_out_of_profile() {
    let two = on(
        HEARTBEAT,
        &edited(&shipped(), "(core-count 1 tick)", "(core-count 2 tick)"),
    );
    assert_status("two cores", &two, Status::UnsupportedProfile);
    assert!(
        says(&two, "unsupported-profile", "multicore"),
        "{}",
        two.render_diagnostics()
    );
}

/// `/2` §7: the three stand as `check`'s precedence ranks them, and each is said.
#[test]
fn the_machines_refusals_stand_by_checks_precedence() {
    let target = edited(&shipped(), "(non-maskable-interrupts 0)", "");
    let target = edited(&target, "(firmware-traps 0)", "(firmware-traps 1)");
    let target = edited(&target, "(executable true)", "(executable false)");
    let all = on(HEARTBEAT, &target);
    assert_status("three refusals", &all, Status::UnsupportedProfile);
    for code in [
        "unsupported-profile",
        "infeasible-configuration",
        "missing-fact",
    ] {
        assert!(codes(&all).contains(&code), "{code}: {:?}", codes(&all));
    }
}

/// `/2` §2: a compare read two ways is refused when the file is read, whether or not anything binds the device.
#[test]
fn a_target_whose_compare_reads_two_ways_is_refused_when_its_file_is_read() {
    let two_ways = on(
        HEARTBEAT,
        &edited(
            &shipped(),
            "(compare-magnitude true)",
            "(compare-magnitude true)\n    (compare-window 16 tick)",
        ),
    );
    assert_status(
        "a window beside magnitude",
        &two_ways,
        Status::InvalidDescription,
    );
    assert!(
        says(&two_ways, "invalid-description", "compare-magnitude"),
        "{}",
        two_ways.render_diagnostics()
    );
}

/// `Target::read`'s refusals, and a file that does not read: the description is still judged, every stage that can be.
#[test]
fn a_file_that_is_no_target_is_refused_and_the_description_still_judged() {
    let service = on(HEARTBEAT, &format!("{}\n(defservice x.y)\n", shipped()));
    assert_status(
        "a service in the target",
        &service,
        Status::InvalidDescription,
    );
    assert!(
        says(&service, "invalid-description", "declares hardware alone"),
        "{}",
        service.render_diagnostics()
    );
    let unread = on(
        "(defblock a.b (offers (tick-rate 0 MHz)))\n",
        "(defplatform t.m",
    );
    assert_status(
        "a target that does not read",
        &unread,
        Status::InvalidDescription,
    );
    assert!(
        codes(&unread).iter().any(|code| code.starts_with("read-")),
        "{:?}",
        codes(&unread)
    );
    assert!(
        codes(&unread)
            .iter()
            .any(|code| code.starts_with("quantity-")),
        "the description's own refusal is said too: {:?}",
        codes(&unread)
    );
}

/// `DC-H3`: the search's readings of what the description writes, each asked alone — a compare read two ways on hardware the target replaces,
/// which `check` alone does not read (`M4.1.7.3.2`, measured).
#[test]
fn what_the_description_writes_is_read_as_the_search_reads_it() {
    let block =
        "(defblock timer.compare (offers (compare-window 16 tick) (compare-magnitude true)))\n";
    let described = format!(
        "{block}{}",
        edited(
            HEARTBEAT,
            "(requires (uses console.uart)))",
            "(requires (uses console.uart timer.compare)))"
        )
    );
    assert_eq!(
        alone(&described).status,
        Status::Ok,
        "{}",
        alone(&described).render_diagnostics()
    );
    let there = on(&described, &shipped());
    assert_status("on the target", &there, Status::InvalidDescription);
    assert!(
        says(&there, "invalid-description", "compare-magnitude"),
        "{}",
        there.render_diagnostics()
    );
}

/// "A search that stops at a first stage hides what a later one would say": two values of one fact stop the search's first stage, and the
/// channels a consumed function is offered with, the joint rules' reading, is said beside it.
#[test]
fn each_reading_is_asked_alone() {
    let blocks = "(defblock a.twice (offers (tick-rate 10 MHz) (tick-rate 20 MHz)))\n\
                  (defblock a.none (offers (relative-delay true) (channels 0 tick)))\n";
    let described = format!(
        "{blocks}{}",
        edited(
            HEARTBEAT,
            "(requires (uses console.uart)))",
            "(requires (uses console.uart a.twice a.none)))"
        )
    );
    let resolved = resolve_with(
        &request(&described, &NoModules, None),
        &Resolve {
            system: None,
            unchecked: true,
        },
        Limits::DEFAULT,
    );
    assert_eq!(resolved.response.status, Status::InvalidDescription);
    assert!(
        resolved
            .response
            .diagnostics
            .iter()
            .any(|d| d.message.contains("two values"))
            && !resolved
                .response
                .diagnostics
                .iter()
                .any(|d| d.message.contains("no channel")),
        "the search stops at its first stage, before the joint rules read the channels: {}",
        resolved.response.render_diagnostics()
    );
    let there = on(&described, &shipped());
    assert_status("on the target", &there, Status::InvalidDescription);
    assert!(
        says(&there, "invalid-description", "tick-rate"),
        "{}",
        there.render_diagnostics()
    );
    assert!(
        says(&there, "invalid-description", "no channel"),
        "{}",
        there.render_diagnostics()
    );
}

/// What `on_target` refuses, under the verdict `check` gives each code, `refinement-violated` as `infeasible-configuration`.
#[test]
fn what_keeps_a_system_from_being_read_there_takes_checks_verdict() {
    let named = on(
        &edited(
            HEARTBEAT,
            "  (requires (uses console.write))\n",
            "  (requires (uses console.write target.console))\n",
        ),
        &shipped(),
    );
    assert_status(
        "naming the target's device",
        &named,
        Status::InfeasibleConfiguration,
    );
    assert!(
        says(&named, "infeasible-configuration", "target.console"),
        "{}",
        named.render_diagnostics()
    );

    let absent = on(
        &edited(
            HEARTBEAT,
            "(offers (core-count 1 tick) (reachable-at-privilege machine))",
            "(absent tick-rate) (offers (core-count 1 tick) (reachable-at-privilege machine))",
        ),
        &shipped(),
    );
    assert_status(
        "an absence the target offers",
        &absent,
        Status::InfeasibleConfiguration,
    );
    assert_eq!(
        codes(&absent),
        ["refinement-violated"],
        "{}",
        absent.render_diagnostics()
    );

    let twice = on(
        &format!("{HEARTBEAT}(defblock target.console (offers (observable-output true)))\n"),
        &shipped(),
    );
    assert_status("a name both declare", &twice, Status::InvalidDescription);
    assert!(
        codes(&twice).contains(&"schema-duplicate-name"),
        "{:?}",
        codes(&twice)
    );
}

/// A module tree, read as the search reads it, its instances named in the judgement.
#[test]
fn a_module_tree_is_put_on_the_target() {
    let modules = MemoryModules::new().with("hw.timer", "(defmodule hw.timer (version 1 0))\n");
    let tree = format!(
        "(defmodule app (version 1 0) (import hw.timer (as timer) (version (at-least 1 0)))\n{HEARTBEAT})\n"
    );
    let response = check(&request(&tree, &modules, Some(&shipped())));
    assert_status("a tree on the target", &response, Status::Ok);
    let instances = response
        .judged
        .as_ref()
        .and_then(|judged| judged.instances.clone());
    assert_eq!(instances.expect("a tree").len(), 2);
    let uncomposed = check(&request(&tree, &NoModules, Some(&shipped())));
    assert_status(
        "an import not found",
        &uncomposed,
        Status::InvalidDescription,
    );
    assert!(
        codes(&uncomposed).contains(&"module-not-found"),
        "{:?}",
        codes(&uncomposed)
    );
}

/// The search on a target is `M4.1.7.5`'s: until then a request naming one is not searched, and says so.
#[test]
fn resolve_and_explain_on_a_target_are_not_built_yet() {
    let target = shipped();
    let asked = request(HEARTBEAT, &NoModules, Some(&target));
    let resolved = resolve_with(
        &asked,
        &Resolve {
            system: None,
            unchecked: true,
        },
        Limits::DEFAULT,
    );
    let explained = explain_with(
        &asked,
        &Explain {
            fact: "observable-output",
            requester: None,
            task: None,
            system: None,
        },
        Limits::DEFAULT,
    );
    for (what, response) in [
        ("resolve", &resolved.response),
        ("explain", &explained.response),
    ] {
        assert_eq!(response.status, Status::Unimplemented, "{what}");
        assert!(response.judged.is_none(), "{what}");
        assert!(
            response.notes.iter().any(|note| note.contains(TARGET)),
            "{what}: {:?}",
            response.notes
        );
        assert!(
            response
                .hint
                .as_deref()
                .is_some_and(|hint| hint.contains("M4.1.7.5")),
            "{what}"
        );
    }
    assert!(resolved.plan.is_none());
}

/// The budget counts what a request brings: the target's file with the description.
#[test]
fn the_budget_counts_the_target() {
    let target = shipped();
    let limits = Limits {
        bytes: Some(HEARTBEAT.len() + target.len() - 1),
    };
    let over = check_with(&request(HEARTBEAT, &NoModules, Some(&target)), limits);
    assert_eq!(over.status, Status::ToolFailure);
    assert!(
        over.notes.iter().any(|note| note.contains(TARGET)),
        "{:?}",
        over.notes
    );
    let within = check_with(
        &request(HEARTBEAT, &NoModules, Some(&target)),
        Limits {
            bytes: Some(HEARTBEAT.len() + target.len()),
        },
    );
    assert_eq!(within.status, Status::Ok, "{}", within.render_diagnostics());
}
