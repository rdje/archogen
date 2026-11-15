//! A plan of a system on a target, `archogen-assignment/3`, judged by the checker (leaf `M4.1.7.5.4.3`; `DC-H2`): check
//! (1) holds the record's `(target NAME "SHA256")` to the target given, then checks (2) to (5) judge it over what is read
//! on the target, the release requester among its requesters (`docs/decisions/decision_release-alarm.md` §1;
//! `docs/semantics/model.md` §8 rules 1, 7, 9 and 13). Each plan here is written by hand from the format and the rules,
//! never from a run.

use std::path::Path;

use archogen_outcome::Status;
use archogen_plan_check::assignment::program_digest;
use archogen_plan_check::check::{
    plan_on_target, record_on_target, Verdict, PLAN_BINDING, PLAN_JOINT, PLAN_RECORD, PLAN_STALE,
};
use archogen_plan_check::release;
use archogen_plan_check::target::Target;
use eadl_front::{Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;

/// A file of the repository, read from its root.
fn repository(file: &str) -> String {
    std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../..")
            .join(file),
    )
    .unwrap_or_else(|e| panic!("{file} reads: {e}"))
}

/// A description and a target, each read into one source map: what a plan is judged against.
struct Inputs {
    sources: SourceMap,
    program: Program,
    target: Target,
}

impl Inputs {
    /// The plain description `description` and the target `target`, which must both read.
    fn read(description: &str, target: &str) -> Self {
        let mut sources = SourceMap::new();
        let id = sources
            .add("system.eadl", description)
            .expect("the description fits");
        let (document, diagnostics) = eadl_front::read(&sources, id);
        assert!(
            !diagnostics.has_errors(),
            "{}",
            diagnostics.render(&sources)
        );
        let program = archogen_plan_check::check::plain(document.declarations().cloned().collect());
        let id = sources.add("target.eadl", target).expect("the target fits");
        let target = Target::read(&sources, id).expect("the target reads");
        Self {
            sources,
            program,
            target,
        }
    }

    /// The S0 heartbeat and the shipped target.
    fn shipped() -> Self {
        Self::read(
            &repository("examples/s0-heartbeat/system.eadl"),
            &repository("targets/riscv-virt-up.eadl"),
        )
    }

    /// A `/3` record for `system` on the target given, its program the description's, with `binds` as its bindings.
    fn plan(&self, system: &str, binds: &[&str]) -> String {
        record(
            &program_digest(&self.program),
            &self.target.platform,
            &self.target.digest(),
            system,
            binds,
        )
    }

    /// The checker's judgement of the record `text`.
    fn judge(&self, text: &str) -> Result<(), Verdict> {
        plan_on_target(
            &self.sources,
            &self.program,
            &self.target,
            &RT_STATIC_UP_V1,
            "plan.eadl",
            text,
        )
    }
}

/// A `/3` record's text, as the format writes one: the image placed in the shipped target's RAM.
fn record(program: &str, platform: &str, digest: &str, system: &str, binds: &[&str]) -> String {
    let mut text = format!(
        "(format \"archogen-assignment/3\")\n(profile rt-static-up-v1)\n(system {system})\n(program \"{program}\")\n\
         (target {platform} \"{digest}\")\n(adapters adapters/1)\n(placement target.riscv-virt-up target.ram)\n"
    );
    for bind in binds {
        text.push_str(bind);
        text.push('\n');
    }
    text
}

/// The S0 heartbeat's bindings on the shipped target, by the rules: the console's observable output, and the release
/// requester's eight facts that bind, each a guarantee of the `counter` function, at the one timer. `available-in-state`,
/// a qualifier, binds nothing where its requester binds a function (model §8 rule 13), and the tasks require nothing of
/// their own.
const HEARTBEAT: [&str; 9] = [
    "(bind console.write observable-output target.console)",
    "(bind (release heartbeat) alarm-kind target.timer)",
    "(bind (release heartbeat) alarm-wiring target.timer)",
    "(bind (release heartbeat) counter-modulus target.timer)",
    "(bind (release heartbeat) observation-coherent target.timer)",
    "(bind (release heartbeat) relative-delay target.timer)",
    "(bind (release heartbeat) supported-horizon target.timer)",
    "(bind (release heartbeat) tick-rate target.timer)",
    "(bind (release heartbeat) unambiguous-horizon target.timer)",
];

/// A refusal's status and the code of each of its findings.
fn refused(result: Result<(), Verdict>) -> (Status, Vec<String>) {
    let verdict = result.expect_err("refused");
    let codes = verdict.findings.iter().map(|f| f.code.clone()).collect();
    (verdict.status, codes)
}

/// The heartbeat's bindings with `drop` left out and `add` put in, in the record's order.
fn changed(drop: &str, add: Option<&str>) -> Vec<String> {
    let mut binds: Vec<String> = HEARTBEAT
        .iter()
        .filter(|bind| **bind != drop)
        .map(|bind| (*bind).to_owned())
        .collect();
    if let Some(add) = add {
        binds.push(add.to_owned());
        binds.sort_by_key(|bind| bind.replace("(bind (release", "(bind ~(release"));
    }
    binds
}

fn texts(binds: &[String]) -> Vec<&str> {
    binds.iter().map(String::as_str).collect()
}

#[test]
fn the_release_requester_asks_the_record_s_six_lines_at_the_shortest_period_as_its_task_writes_it()
{
    let inputs = Inputs::read(
        "(defblock console.uart (offers (observable-output true)))
(defplatform soc (offers (core-count 1 tick)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem s
  (task slow (period 30 ms) (deadline 30 ms) (priority 2) (uses console.write))
  (task fast (period 5000 us) (deadline 5 ms) (priority 1) (uses console.write))
  (task woken (min-separation 1 ms) (deadline 1 ms) (priority 3) (uses console.write))
  (platform (uses soc)))",
        &repository("targets/riscv-virt-up.eadl"),
    );
    let side = release::side(&inputs.program, "s").expect("a task the timer releases");
    assert_eq!(
        side.to_canonical(),
        "(release (needs relative-delay counter-modulus tick-rate observation-coherent) (requires (available-in-state idle) \
         (alarm-wiring core) (alarm-kind level-compare) (unambiguous-horizon (at-least 5000 us)) (supported-horizon \
         (at-least 5000 us))))"
    );
    // No release requester with no task writing a `period`.
    let sporadic = Inputs::read(
        "(defsystem s (task woken (min-separation 1 ms) (deadline 1 ms) (priority 1)))",
        &repository("targets/riscv-virt-up.eadl"),
    );
    assert!(release::side(&sporadic.program, "s").is_none());
}

#[test]
fn the_s0_heartbeat_on_the_shipped_target_is_accepted() {
    let inputs = Inputs::shipped();
    let text = inputs.plan("heartbeat", &HEARTBEAT);
    assert_eq!(inputs.judge(&text), Ok(()), "{text}");
}

#[test]
fn a_system_whose_services_name_another_system_is_accepted_on_the_shipped_target() {
    // `s`'s task uses the services `m` and `m.s`, which each name the system `y`: kept on the target, as rule 1 reads
    // "another system, but for one it names, itself or through a service or a policy it reaches", and standing on the
    // target's platform. `s` requires nothing but its release requester's eight facts, each at the one timer.
    let inputs = Inputs::read(
        &repository("crates/archogen-plan/tests/corpus/naming-namer-prefix.eadl"),
        &repository("targets/riscv-virt-up.eadl"),
    );
    let binds: Vec<String> = HEARTBEAT[1..]
        .iter()
        .map(|bind| bind.replace("(release heartbeat)", "(release s)"))
        .collect();
    let text = inputs.plan("s", &texts(&binds));
    assert_eq!(inputs.judge(&text), Ok(()), "{text}");
}

#[test]
fn a_release_requirement_left_unbound_is_refused() {
    let inputs = Inputs::shipped();
    let binds = changed("(bind (release heartbeat) tick-rate target.timer)", None);
    assert_eq!(
        refused(inputs.judge(&inputs.plan("heartbeat", &texts(&binds)))),
        (Status::ToolFailure, vec![PLAN_BINDING.to_owned()])
    );
}

#[test]
fn a_release_requirement_bound_to_an_instance_is_refused() {
    // Model §8 rule 1: the release requester binds a block or a platform itself, never an adapter's instance.
    let inputs = Inputs::shipped();
    let binds = changed(
        "(bind (release heartbeat) counter-modulus target.timer)",
        Some("(bind (release heartbeat) counter-modulus (adapter epoch-extender target.timer))"),
    );
    assert_eq!(
        refused(inputs.judge(&inputs.plan("heartbeat", &texts(&binds)))),
        (Status::ToolFailure, vec![PLAN_BINDING.to_owned()])
    );
}

#[test]
fn a_release_requester_of_a_system_with_no_task_the_timer_releases_is_refused() {
    let inputs = Inputs::read(
        "(defblock console.uart (offers (observable-output true)))
(defplatform host.playground (offers (core-count 1 tick)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
(defsystem heartbeat
  (task beat (min-separation 10 ms) (deadline 10 ms) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses host.playground)))",
        &repository("targets/riscv-virt-up.eadl"),
    );
    // Accepted without one: it has no release requester, `/3` writing `(timer none)` downstream.
    let without = inputs.plan("heartbeat", &HEARTBEAT[..1]);
    assert_eq!(inputs.judge(&without), Ok(()), "{without}");
    let with = inputs.plan(
        "heartbeat",
        &[
            HEARTBEAT[0],
            "(bind (release heartbeat) relative-delay target.timer)",
        ],
    );
    assert_eq!(
        refused(inputs.judge(&with)),
        (Status::ToolFailure, vec![PLAN_BINDING.to_owned()])
    );
}

#[test]
fn a_release_device_stating_no_compare_step_leaves_what_the_analysis_reads_unknown() {
    let shipped = repository("targets/riscv-virt-up.eadl");
    for (case, target) in [
        (
            "no `compare-step`",
            shipped.replace("    (compare-step 1 tick)\n", ""),
        ),
        (
            "neither a `compare-window` nor `(compare-magnitude true)`",
            shipped.replace("    (compare-magnitude true)\n", ""),
        ),
    ] {
        assert_ne!(target, shipped, "{case}: the variant changes the file");
        let inputs = Inputs::read(&repository("examples/s0-heartbeat/system.eadl"), &target);
        let verdict = inputs
            .judge(&inputs.plan("heartbeat", &HEARTBEAT))
            .expect_err(case);
        assert_eq!(verdict.status, Status::ToolFailure, "{case}: {verdict:?}");
        assert_eq!(verdict.findings.len(), 1, "{case}: {verdict:?}");
        let finding = &verdict.findings[0];
        assert_eq!(finding.code, PLAN_JOINT, "{case}");
        assert!(
            finding.message.contains("missing-fact") && finding.message.contains("`target.timer`"),
            "{case}: {}",
            finding.message
        );
    }
}

#[test]
fn a_plan_resolved_on_another_target_is_stale() {
    let inputs = Inputs::shipped();
    let digest = program_digest(&inputs.program);
    // Another file's digest: a target whose forms differ, a comment moving no digest.
    let other = Inputs::read(
        &repository("examples/s0-heartbeat/system.eadl"),
        &repository("targets/riscv-virt-up.eadl").replace(
            "(firmware-traps 0)",
            "(firmware-traps 0) (core-count 1 tick)",
        ),
    );
    assert_ne!(other.target.digest(), inputs.target.digest());
    for (case, text) in [
        (
            "another file's digest",
            record(
                &digest,
                &inputs.target.platform,
                &other.target.digest(),
                "heartbeat",
                &HEARTBEAT,
            ),
        ),
        (
            "another platform beside another file's digest: the digest decides first",
            record(
                &digest,
                "target.elsewhere",
                &other.target.digest(),
                "heartbeat",
                &HEARTBEAT,
            ),
        ),
        (
            "a `/2` record, of no target",
            format!(
                "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system heartbeat)\n(program \"{digest}\")\n\
                 (adapters adapters/1)\n(placement target.riscv-virt-up target.ram)\n{}\n",
                HEARTBEAT[0]
            ),
        ),
    ] {
        assert_eq!(
            refused(inputs.judge(&text)),
            (Status::StaleInput, vec![PLAN_STALE.to_owned()]),
            "{case}"
        );
    }
}

#[test]
fn a_record_naming_another_platform_beside_the_given_file_s_digest_is_one_no_run_writes() {
    // `docs/decisions/decision_runtime-plan-format-2.md` §1, amended `2026-11-15` (leaf `M4.1.7.5.6`): the digest says
    // the record is of this file's run, and this file's forms name one platform, so a record naming another is a record
    // no run writes, `tool-failure`, as one naming another profile or system is — never `stale-input`.
    let inputs = Inputs::shipped();
    let text = record(
        &program_digest(&inputs.program),
        "target.elsewhere",
        &inputs.target.digest(),
        "heartbeat",
        &HEARTBEAT,
    );
    let verdict = inputs.judge(&text).expect_err("refused");
    assert_eq!(
        (verdict.status, verdict.findings.len()),
        (Status::ToolFailure, 1),
        "{verdict:?}"
    );
    let finding = &verdict.findings[0];
    assert_eq!(finding.code, PLAN_RECORD);
    assert!(
        finding.message.contains("`target.elsewhere`")
            && finding.message.contains("`target.riscv-virt-up`"),
        "{}",
        finding.message
    );
    // The library's check (1) on a target says the same.
    let refusal = record_on_target(
        &inputs.program,
        &inputs.target,
        &RT_STATIC_UP_V1,
        "plan.eadl",
        &text,
    )
    .expect_err("refused");
    assert_eq!(
        (refusal.status, refusal.findings[0].code.as_str()),
        (Status::ToolFailure, PLAN_RECORD)
    );
}

#[test]
fn the_target_s_digest_is_its_forms_program_text_and_moves_with_no_comment() {
    let shipped = repository("targets/riscv-virt-up.eadl");
    let commented = format!("; one more comment\n{shipped}");
    let a = Inputs::read(&repository("examples/s0-heartbeat/system.eadl"), &shipped);
    let b = Inputs::read(&repository("examples/s0-heartbeat/system.eadl"), &commented);
    assert_eq!(a.target.digest(), b.target.digest());
    assert_eq!(
        a.target.digest(),
        program_digest(&archogen_plan_check::check::plain(
            a.target.declarations.clone()
        ))
    );
}
