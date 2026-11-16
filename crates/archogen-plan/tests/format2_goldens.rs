//! `archogen-runtime-plan/2`'s goldens (leaf `M4.1.7.6.9`, `docs/decisions/decision_runtime-plan-format-2.md`): as `/1`'s
//! (`tests/goldens.rs`), each is lowered from frozen inputs, the crate's own fixtures and never another suite's file, and
//! it is never rewritten. So nothing here writes a file, and no environment variable blesses one: a golden that moves is a
//! new format number.
//!
//! ⚠️ **Lowered by the executable form's entries**, `lower_hosted` and `lower_on_target`, which no command reaches yet:
//! `/2` has no production entry taking a checker's result record and no reader of its own, `M4.1.7.7`'s, which is to be
//! held to these goldens. Each is held five ways: lowered from its frozen assignment, description, target and application
//! byte for byte and twice alike; its sha256 and its code identity (§1) frozen here; its text written as §3 writes one,
//! each head §1's table classifies, in §3's order; and its frozen assignment one the configuration checker accepts today,
//! on its target where it has one, so the frozen record is still the checker's word.
//!
//! The inputs are in `tests/goldens2/`. A target or an application's file there is no `.eadl` file, since every `.eadl`
//! file of the tree is a description whose verdict `crates/archogen-cli/tests/verdicts.rs` freezes; a description the
//! corpus does not hold is frozen there as `.description`. `riscv-virt-up.target` is `targets/riscv-virt-up.eadl` as it
//! stood when these were frozen, and `fixture.target` the test target of `tests/format2_target.rs`, `TWO_CHANNELS`. The
//! sporadic system stands on the fixture, since the shipped target holds no device raising an interrupt. What generates
//! the code is fixed: the emitter `archogen-emit/1`, and the catalog empty, as it stands (`M2.7.4.5`).

mod common;

use std::fs;

use archogen_digest::Digest;
use archogen_outcome::record::Producer;
use archogen_plan::identity::{code_identity, head, preimage, LINES};
use archogen_plan::{lower_hosted, lower_on_target, ApplicationFile, Generation, TargetFile};
use archogen_plan_check::judgement::{judge_on, Inputs, TargetFile as CheckedTarget};
use common::{catalog, checked, registry, root, vocabulary};
use eadl_front::module::MemoryModules;
use eadl_model::profile::RT_STATIC_UP_V1;

/// The emitter the goldens name, an input of each.
const EMITTER: &str = "archogen-emit/1";

/// The catalog the goldens are lowered with: empty, as the tool's stands.
const RECORDS: [(String, Digest); 0] = [];

/// Each golden: its name, its description, the target it stands on, and its application's file, each a frozen input.
struct Golden {
    name: &'static str,
    description: &'static str,
    target: Option<&'static str>,
    application: Option<&'static str>,
    assignment: &'static str,
}

/// The heartbeat is the corpus's own `worked-heartbeat`, as `/1`'s golden; the others are frozen in `tests/goldens2/`.
const GOLDENS: [Golden; 5] = [
    Golden {
        name: "heartbeat-hosted",
        description: "corpus/worked-heartbeat.eadl",
        target: None,
        application: None,
        assignment: "heartbeat-hosted",
    },
    Golden {
        name: "heartbeat-hosted-application",
        description: "corpus/worked-heartbeat.eadl",
        target: None,
        application: Some("heartbeat.application"),
        assignment: "heartbeat-hosted",
    },
    Golden {
        name: "heartbeat-target",
        description: "corpus/worked-heartbeat.eadl",
        target: Some("riscv-virt-up.target"),
        application: None,
        assignment: "heartbeat-target",
    },
    Golden {
        name: "sporadic-fixture",
        description: "goldens2/sporadic.description",
        target: Some("fixture.target"),
        application: None,
        assignment: "sporadic-fixture",
    },
    Golden {
        name: "two-alarms-fixture",
        description: "goldens2/two-alarms.description",
        target: Some("fixture.target"),
        application: None,
        assignment: "two-alarms-fixture",
    },
];

/// Each golden plan's sha256 and its code identity (§1), frozen: a change that moves one is a new format number.
const IDENTITIES: [(&str, &str, &str); 5] = [
    (
        "heartbeat-hosted",
        "2e23ac7cf8c5654dc2c78ff76e39a495199e1c173310014ca77d3e5aac7c16ca",
        "53855f97d00d8614d16f14b6a2cee0eb2bd9ba91b170416d0914ead288f053d2",
    ),
    (
        "heartbeat-hosted-application",
        "594929a16860cf2cf38f1390d59c9ce80cf42bf21ae13be5ff86e161386d919f",
        "1e59abc8a144abf3bcd790cd51c72288b7db1d294f0cbc169cb3b57899423f50",
    ),
    (
        "heartbeat-target",
        "14e6b9b92375372ed44fb6f13e0d74e68d4debd1e4aa6e7b730a6d56cc787d34",
        "8dd7b950089ac6af9c17fc38809f211f39ded583085797da3170d6fa828523d5",
    ),
    (
        "sporadic-fixture",
        "2025b8a283787145381380e3b973332515b202a3ea09f6e51d61eb8321702796",
        "4561934a588b354abaee9b76dd75393720e6761583e2bb6f26682a9a1b4dd34a",
    ),
    (
        "two-alarms-fixture",
        "4df14a854823c5d61acb777106cfa1c0eda155ef8d87ade9948e49903990e7f7",
        "4bacabb909ea94e8338e6009152bbbd693c19bad59c844e447c12ca35d3672cd",
    ),
];

/// A file of the crate's tests, by its path under `tests/`.
fn test_file(path: &str) -> String {
    let full = root().join("crates/archogen-plan/tests").join(path);
    fs::read_to_string(&full).unwrap_or_else(|e| panic!("{}: {e}", full.display()))
}

/// A frozen input or golden of `tests/goldens2/`.
fn frozen(name: &str) -> String {
    test_file(&format!("goldens2/{name}"))
}

/// `golden` lowered from its frozen inputs by the executable form's entry.
fn lowered(golden: &Golden) -> String {
    let program = checked(&test_file(golden.description)).expect("the language accepts it");
    let assignment = frozen(&format!("{}.assignment", golden.assignment));
    let generation = Generation {
        emitter: EMITTER,
        records: &RECORDS,
    };
    let application = golden.application.map(frozen);
    let file = golden
        .application
        .zip(application.as_deref())
        .map(|(name, text)| ApplicationFile { name, text });
    let outcome = match golden.target {
        None => lower_hosted(
            &program,
            &assignment,
            &vocabulary(),
            &catalog(),
            &generation,
            file.as_ref(),
        ),
        Some(target) => lower_on_target(
            &program,
            &assignment,
            &TargetFile {
                name: target,
                text: &frozen(target),
            },
            &registry(),
            &vocabulary(),
            &catalog(),
            &generation,
            file.as_ref(),
        ),
    };
    outcome.unwrap_or_else(|refusal| panic!("{}: {refusal}", golden.name))
}

#[test]
fn each_golden_is_what_its_entry_lowers_from_its_frozen_inputs_twice_alike() {
    for golden in &GOLDENS {
        let plan = frozen(&format!("{}.plan", golden.name));
        let once = lowered(golden);
        assert_eq!(once, plan, "{}: the plan moved", golden.name);
        assert_eq!(
            lowered(golden),
            once,
            "{}: lowered twice alike",
            golden.name
        );
    }
}

#[test]
fn each_golden_keeps_its_identity_and_its_code_identity() {
    assert_eq!(IDENTITIES.len(), GOLDENS.len());
    for (golden, (name, identity, code)) in GOLDENS.iter().zip(IDENTITIES) {
        assert_eq!(golden.name, name);
        let plan = frozen(&format!("{name}.plan"));
        assert_eq!(Digest::of(plan.as_bytes()).hex(), identity, "{name}");
        assert_eq!(
            code_identity(&plan).expect("an identity").hex(),
            code,
            "{name}: its code identity"
        );
    }
    // Two plans of one code, the heartbeat with and without its application's file, have one code identity: the file's
    // figures and where they came from are left out, its entry points kept, and those the hosted plan with no file has
    // none of (§1's table; `two_plans_of_one_code_have_one_preimage_and_code_moves_it`).
    let code = |name: &str| {
        IDENTITIES
            .iter()
            .find(|(n, _, _)| *n == name)
            .expect("named")
            .2
    };
    assert_ne!(
        code("heartbeat-hosted"),
        code("heartbeat-hosted-application"),
        "an application's entry points are code"
    );
}

/// Each golden is written as §3 writes a plan of `/2`: every line one form, its head one §1's table classifies, the heads
/// in §3's order, and the code identity's preimage taken; and each names what generates its code, the emitter and the catalog.
#[test]
fn each_golden_is_written_as_section_3_writes_one() {
    let order = |h: &str| LINES.iter().position(|(name, _)| *name == h);
    for golden in &GOLDENS {
        let plan = frozen(&format!("{}.plan", golden.name));
        let mut last = 0;
        for line in plan.lines() {
            let at = order(head(line)).unwrap_or_else(|| {
                panic!("{}: no line of `/2` begins `{}`", golden.name, head(line))
            });
            assert!(at >= last, "{}: `{line}` out of §3's order", golden.name);
            last = at;
        }
        assert!(preimage(&plan).is_ok(), "{}", golden.name);
        assert!(
            plan.starts_with("(format \"archogen-runtime-plan/2\")\n"),
            "{}",
            golden.name
        );
        assert!(
            plan.contains(&format!("\n(emitter {EMITTER})\n")),
            "{}",
            golden.name
        );
        assert!(plan.ends_with("(complete no)\n"), "{}", golden.name);
        let catalog = format!("(catalog \"{}\" ", Digest::of(b"").hex());
        assert!(
            plan.contains(&catalog),
            "{}: the empty catalog",
            golden.name
        );
    }
}

/// Each frozen assignment is one the configuration checker accepts today, on its target where it has one: the frozen
/// record is still the checker's word, as `/1`'s fixed result records are.
#[test]
fn each_frozen_assignment_is_one_the_checker_accepts() {
    let producer = Producer {
        name: "archogen-plan-check".to_owned(),
        sha256: Digest::of(b"the goldens' fixed checker"),
    };
    for golden in &GOLDENS {
        let (description, assignment) = (
            test_file(golden.description),
            frozen(&format!("{}.assignment", golden.assignment)),
        );
        let target = golden.target.map(frozen);
        let record = judge_on(
            producer.clone(),
            &Inputs {
                description_name: "description.eadl",
                description: &description,
                modules: &MemoryModules::new(),
                profile: &RT_STATIC_UP_V1,
                plan_name: "plan.eadl",
                plan: assignment.as_bytes(),
            },
            golden
                .target
                .zip(target.as_deref())
                .map(|(name, text)| CheckedTarget { name, text }),
        )
        .unwrap_or_else(|why| panic!("{}: {why:?}", golden.name));
        assert!(record.accepts(), "{}: {:?}", golden.name, record.findings());
    }
}
