//! Format `/2` on a target, its identity and refusals, as `lower_on_target` and `target_lines` give them
//! (`docs/decisions/decision_runtime-plan-format-2.md` §1, "The target in the identity", "The catalog in the identity" and
//! "The application and the realization in the identity", and §7's order; leaf `M4.1.7.6.7.1`), over `targets/riscv-virt-up.eadl`
//! and the S0 heartbeat: the target's own refusals first, by the code `check` gives them; then the application's file; then
//! the record, of the kind and the target given, the digest deciding the run; then the application's target, the platform's
//! name. The rest of a plan on a target is `M4.1.7.6.7.2`'s, and the lowering says so rather than write it short.

mod common;

use std::fs;

use archogen_digest::Digest;
use archogen_plan::application::{CLAUSE_CUT, CLAUSE_PARTS, FORMAT};
use archogen_plan::{
    lower_hosted, lower_on_target, target_lines, ApplicationFile, Generation, Refusal, TargetFile,
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

/// Past every stage this leaf builds: the rest of the plan is `M4.1.7.6.7.2`'s.
const UNBUILT: &str = "M4.1.7.6.7.2";

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
