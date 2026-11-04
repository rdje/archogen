//! The production entry, `lower_checked` (leaf `M4.1.4.1`, `docs/decisions/decision_runtime-plan.md` §1): the lowering's
//! third input, the configuration checker's result record, each refusal with its code and taken in §1's order, and the
//! provenance it returns beside the plan.

mod common;

use archogen_digest::Digest;
use archogen_outcome::record::{Finding, Judged, Label, Producer, Record, Severity, Subject};
use archogen_outcome::Status;
use archogen_plan::{lower, lower_checked, Acceptance, Generator, Lowered, Refusal};
use archogen_plan_check::judgement::{judge, Inputs};
use common::{catalog, checked, registry, vocabulary};
use eadl_front::module::MemoryModules;
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::plan;

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

/// The assignment the search would write for it, with the profile and catalog given.
fn assignment(profile: &str, adapters: &str) -> String {
    let program = checked(DESCRIPTION).expect("accepted");
    format!(
        "(format \"archogen-assignment/2\")\n(profile {profile})\n(system dg)\n(program \"{}\")\n\
         (adapters {adapters})\n(bind console.write observable-output console.uart)\n",
        plan::program_digest(&program)
    )
}

/// The digest of the elaborated program.
fn program_digest() -> Digest {
    let program = checked(DESCRIPTION).expect("accepted");
    Digest::parse(&format!("sha256:{}", plan::program_digest(&program))).expect("a digest")
}

fn producer(name: &str) -> Producer {
    Producer {
        name: name.to_owned(),
        sha256: Digest::of(b"the checker's executable"),
    }
}

/// An accepting result record, as `archogen-plan-check` writes one for `text`.
fn accepting(text: &str) -> Record {
    Record::new(
        producer("archogen-plan-check"),
        Subject {
            program: program_digest(),
            judged: Judged::Plan(Digest::of(text.as_bytes())),
        },
        "rt-static-up-v1".to_owned(),
        Status::Ok,
        Vec::new(),
    )
    .expect("a record")
}

fn generator() -> Generator {
    Generator {
        version: "0.1.0".to_owned(),
        sha256: Digest::of(b"the generator's executable"),
    }
}

/// `result`'s text, lowered over the description by the production entry.
fn taken(text: &str, result: &str) -> Result<Lowered, Refusal> {
    let program = checked(DESCRIPTION).expect("accepted");
    lower_checked(
        &program,
        text,
        result,
        &registry(),
        generator(),
        Acceptance::CheckedByFront,
    )
}

/// The refusal's code and message.
fn refused(text: &str, result: &str) -> (&'static str, String) {
    let refusal = taken(text, result).expect_err("refused");
    (refusal.code, refusal.message)
}

#[test]
fn it_lowers_what_the_checker_accepted_and_says_where_it_came_from() {
    let text = assignment("rt-static-up-v1", "adapters/1");
    let result = accepting(&text).write();
    let lowered = taken(&text, &result).expect("lowered");

    // The plan is the one the rules write, the entry adding nothing to its text (§3: the provenance is never part of it).
    let program = checked(DESCRIPTION).expect("accepted");
    assert_eq!(
        lowered.plan,
        lower(&program, &text, &vocabulary(), &catalog()).expect("lowered")
    );
    assert_eq!(lowered.provenance.generator, generator());
    assert_eq!(lowered.provenance.result, Digest::of(result.as_bytes()));
    assert_eq!(
        lowered.provenance.checker,
        Digest::of(b"the checker's executable")
    );
    assert_eq!(lowered.provenance.acceptance, Acceptance::CheckedByFront);
}

#[test]
fn the_checker_s_own_record_for_this_assignment_is_taken() {
    // No hand-made record: what the configuration checker itself writes for these inputs, its subject computed by it.
    let text = assignment("rt-static-up-v1", "adapters/1");
    let record = judge(
        producer("archogen-plan-check"),
        &Inputs {
            description_name: "description.eadl",
            description: DESCRIPTION,
            modules: &MemoryModules::new(),
            profile: &RT_STATIC_UP_V1,
            plan_name: "plan.eadl",
            plan: text.as_bytes(),
        },
    )
    .expect("judged");
    assert!(record.accepts(), "{:?}", record.findings());
    assert!(taken(&text, &record.write()).is_ok());
}

#[test]
fn how_the_acceptance_was_obtained_is_the_caller_s_and_is_carried() {
    let text = assignment("rt-static-up-v1", "adapters/1");
    let program = checked(DESCRIPTION).expect("accepted");
    let lowered = lower_checked(
        &program,
        &text,
        &accepting(&text).write(),
        &registry(),
        generator(),
        Acceptance::TakenAsText,
    )
    .expect("lowered");
    assert_eq!(lowered.provenance.acceptance, Acceptance::TakenAsText);
}

#[test]
fn a_result_that_does_not_read_or_is_no_checker_s_judgement_of_a_plan_is_a_tool_failure() {
    let text = assignment("rt-static-up-v1", "adapters/1");
    let (code, message) = refused(&text, "this is not a record");
    assert_eq!(code, "tool-failure");
    assert!(
        message.starts_with("the result record does not read"),
        "{message}"
    );

    let scheduler = Record::new(
        producer("archogen-analyze"),
        Subject {
            program: program_digest(),
            judged: Judged::Plan(Digest::of(text.as_bytes())),
        },
        "rt-static-up-v1".to_owned(),
        Status::Ok,
        Vec::new(),
    )
    .expect("a record");
    let (code, message) = refused(&text, &scheduler.write());
    assert_eq!(code, "tool-failure");
    assert!(message.contains("`archogen-analyze`"), "{message}");

    let build = Record::new(
        producer("archogen-plan-check"),
        Subject {
            program: program_digest(),
            judged: Judged::Build(Digest::of(text.as_bytes())),
        },
        "rt-static-up-v1".to_owned(),
        Status::Ok,
        Vec::new(),
    )
    .expect("a record");
    let (code, message) = refused(&text, &build.write());
    assert_eq!(code, "tool-failure");
    assert!(message.contains("a build, not a plan"), "{message}");
}

#[test]
fn a_result_of_another_assignment_program_or_profile_is_stale() {
    let text = assignment("rt-static-up-v1", "adapters/1");
    let with = |judged: Digest, program: Digest, profile: &str| {
        Record::new(
            producer("archogen-plan-check"),
            Subject {
                program,
                judged: Judged::Plan(judged),
            },
            profile.to_owned(),
            Status::Ok,
            Vec::new(),
        )
        .expect("a record")
        .write()
    };
    let here = Digest::of(text.as_bytes());
    for (case, result, says) in [
        (
            "another assignment",
            with(Digest::of(b"another"), program_digest(), "rt-static-up-v1"),
            "another assignment",
        ),
        (
            "another program",
            with(here, Digest::of(b"another"), "rt-static-up-v1"),
            "another program",
        ),
        (
            "another profile",
            with(here, program_digest(), "some-other-v1"),
            "judged under `some-other-v1`",
        ),
    ] {
        let (code, message) = refused(&text, &result);
        assert_eq!(code, "stale-input", "{case}");
        assert!(message.contains(says), "{case}: {message}");
        assert!(message.ends_with("resolve again"), "{case}: {message}");
    }
}

#[test]
fn a_result_refusing_what_the_generator_accepted_names_both_verdicts() {
    let text = assignment("rt-static-up-v1", "adapters/1");
    let refusing = Record::new(
        producer("archogen-plan-check"),
        Subject {
            program: program_digest(),
            judged: Judged::Plan(Digest::of(text.as_bytes())),
        },
        "rt-static-up-v1".to_owned(),
        Status::InfeasibleConfiguration,
        vec![Finding {
            code: "plan-joint".to_owned(),
            severity: Severity::Error,
            message: "two requesters".to_owned(),
            primary: Label {
                source: "plan".to_owned(),
                line: 0,
                column: 0,
                message: "here".to_owned(),
            },
            secondary: Vec::new(),
            repair: "resolve again".to_owned(),
        }],
    )
    .expect("a record");
    let (code, message) = refused(&text, &refusing.write());
    assert_eq!(code, "tool-failure");
    assert!(message.contains("the generator accepted"), "{message}");
    assert!(message.contains("`infeasible-configuration`"), "{message}");
}

#[test]
fn the_result_is_taken_where_section_1_writes_it() {
    // After the assignment's own refusals: a stale catalog is said over a result that does not read.
    let text = assignment("rt-static-up-v1", "adapters/0");
    let (code, message) = refused(&text, "this is not a record");
    assert_eq!(code, "stale-input");
    assert!(message.contains("`adapters/0`"), "{message}");

    // Before the profile's: a result that does not read is said over a profile `/1` does not lower.
    let text = assignment("some-other-v1", "adapters/1");
    let (code, message) = refused(&text, "this is not a record");
    assert_eq!(code, "tool-failure");
    assert!(
        message.starts_with("the result record does not read"),
        "{message}"
    );

    // And so the profile's refusal is reached by a record agreeing with its assignment's profile.
    let accepted = Record::new(
        producer("archogen-plan-check"),
        Subject {
            program: program_digest(),
            judged: Judged::Plan(Digest::of(text.as_bytes())),
        },
        "some-other-v1".to_owned(),
        Status::Ok,
        Vec::new(),
    )
    .expect("a record");
    let (code, message) = refused(&text, &accepted.write());
    assert_eq!(code, "unsupported-profile");
    assert!(message.contains("`some-other-v1`"), "{message}");
}
