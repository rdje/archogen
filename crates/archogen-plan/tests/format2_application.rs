//! An application's file in hosted `/2`, as `lower_hosted` writes it (`docs/decisions/decision_runtime-plan-format-2.md`
//! §1, "The application and the realization in the identity", §3's forms and owed lines, and §7's order; leaf
//! `M4.1.7.6.6`): its lines in the record's order and as the file states them, the owed lines it states gone, `RA-H7`'s
//! `deadlines-within-reach`, and each refusal by its code and in its place.

mod common;

use std::fs;

use archogen_digest::Digest;
use archogen_plan::application::{CLAUSE_CUT, CLAUSE_PARTS, FORMAT, HELD_OF_A_TASK};
use archogen_plan::{ApplicationFile, Generation, Refusal};
use common::{lowered_hosted, root};

const BOUND: &str = r#"(value 200) (unit us) (scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by the loop's bound")"#;
const LIST: &str = r#"(scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by reading")"#;
const CODE: &str = r#"(code (crate app) (place "app") (digest "sha256:0000000000000000000000000000000000000000000000000000000000000000"))"#;
const INITIALISATION: &str = r#"(initialisation (entry-point "app::init") (stack-bound (value 256) (unit byte) (scope "its deepest path") (binary unbuilt) (evidence assumed) (basis "chosen, to be measured")) (node-calls (scope "all of it") (binary unbuilt) (evidence assumed) (basis "it calls none")))"#;

/// A task's clause, its items in another order than `/2` writes them, its calls unsorted.
fn a_task(name: &str, deadlines: &str) -> String {
    format!(
        r#"(task {name}
  (entry-point "app::{name}")
  (execution-bound {BOUND})
  (os-calls {LIST} (call mask 2) (call unmask 2) (call release 1))
  (node-calls {LIST})
  (masked-runs {LIST}
    (run tail (ending unmask) (value 5) (unit us) (scope "the tail's update") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "four loads") (call release 1)))
  (stack-bound (value 512) (unit byte) (scope "its deepest path") (binary unbuilt) (evidence assumed) (basis "chosen, to be measured"))
  (suspends no) (locks-scheduler no) (shares-outside-sections no)
  (masks-only-through-api yes)
  (figures-under-preemption yes)
  (deadlines-within-reach{deadlines}))
"#
    )
}

fn the_clause() -> String {
    CLAUSE_PARTS.join(CLAUSE_CUT)
}

/// A whole file for `system`, each task with what its `deadlines-within-reach` names.
fn a_file(system: &str, tasks: &[(&str, &str)]) -> String {
    let each: String = tasks
        .iter()
        .map(|(name, deadlines)| a_task(name, deadlines))
        .collect();
    format!(
        "; an application's inputs\n(format \"{FORMAT}\")\n(system {system})\n(target hosted-playground)\n{CODE}\n(leaves-interrupt-hardware-alone yes (clause \"{}\") (basis \"it touches no device\"))\n{INITIALISATION}\n{each}",
        the_clause()
    )
}

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

fn the_heartbeat_s_file() -> String {
    a_file("heartbeat", &[("beat", ""), ("chime", "")])
}

/// `description` lowered as hosted `/2` with `file`, its application's file named `app.eadl`, where one is given.
fn lowered(description: &str, file: Option<&str>) -> Result<String, Refusal> {
    let application = file.map(|text| ApplicationFile {
        name: "app.eadl",
        text,
    });
    lowered_hosted(description, &generation(), application.as_ref()).expect("it resolves")
}

/// The digest of a file's canonical text, as §1 has `(application "HEX")` write it.
fn canonical_digest(text: &str) -> String {
    let mut sources = eadl_front::SourceMap::new();
    let id = sources.add(FORMAT, text.to_owned()).expect("small");
    let (document, _) = eadl_front::read_record(&sources, id);
    Digest::of(document.to_canonical().as_bytes()).hex()
}

/// The heartbeat with a whole file: each of its lines in §3's place and order, written as the file states it and in the
/// record's order, and every owed line it states gone.
#[test]
fn the_heartbeat_s_application_is_written_in_its_places() {
    let file = the_heartbeat_s_file();
    let plan = lowered(&heartbeat(), Some(&file)).expect("it lowers");
    let lines: Vec<&str> = plan.lines().collect();
    let at = |head: &str| {
        lines
            .iter()
            .position(|line| line.starts_with(&format!("({head} ")))
            .unwrap_or_else(|| panic!("no `({head} …)` in {plan}"))
    };
    let catalog = at("catalog");
    assert_eq!(
        lines[catalog + 1],
        format!("(application \"{}\")", canonical_digest(&file))
    );
    assert_eq!(
        lines[catalog + 2],
        "(code app \"app\" \"sha256:0000000000000000000000000000000000000000000000000000000000000000\")"
    );
    let first_input = at("task-inputs");
    assert!(lines[first_input - 1].starts_with("(task 1 "), "{plan}");
    let task = |name: &str| {
        format!(
            "(task-inputs (task heartbeat {name}) (entry-point \"app::{name}\") (execution-bound {BOUND}) (os-calls {LIST} (call mask 2) (call release 1) (call unmask 2)) (node-calls {LIST}) (masked-runs {LIST} (run tail (ending unmask) (value 5) (unit us) (scope \"the tail's update\") (binary unbuilt) (evidence analytically-established) (locator \"docs/argument.md\") (basis \"four loads\") (call release 1))) (stack-bound (value 512) (unit byte) (scope \"its deepest path\") (binary unbuilt) (evidence assumed) (basis \"chosen, to be measured\")) (suspends no) (locks-scheduler no) (shares-outside-sections no) (masks-only-through-api yes) (figures-under-preemption yes) (deadlines-within-reach))"
        )
    };
    assert_eq!(lines[first_input], task("beat"));
    assert_eq!(lines[first_input + 1], task("chime"));
    assert_eq!(
        lines[first_input + 2],
        format!(
            "(declared leaves-interrupt-hardware-alone yes (clause \"{}\") (basis \"it touches no device\"))",
            the_clause()
        )
    );
    assert_eq!(
        lines[first_input + 3],
        "(initialisation-inputs (entry-point \"app::init\") (stack-bound (value 256) (unit byte) (scope \"its deepest path\") (binary unbuilt) (evidence assumed) (basis \"chosen, to be measured\")) (node-calls (scope \"all of it\") (binary unbuilt) (evidence assumed) (basis \"it calls none\")))"
    );
    assert!(lines[first_input + 4].starts_with("(bind "), "{plan}");
    // What the file states is owed of none: each task's facts and the system's.
    for name in ["beat", "chime"] {
        for fact in HELD_OF_A_TASK {
            let line = format!("(owed (task heartbeat {name}) {fact} (by application))");
            assert!(!lines.contains(&line.as_str()), "{line}");
        }
    }
    for fact in ["leaves-interrupt-hardware-alone", "initialisation"] {
        let line = format!("(owed heartbeat {fact} (by application))");
        assert!(!lines.contains(&line.as_str()), "{line}");
    }
    assert!(!plan.contains("(by application)"), "{plan}");
    // What no input states stays owed.
    assert!(lines.contains(&"(owed console.write primitives-out-of-line (by realization))"));
}

/// The code identity keeps an entry point and the code's digest, and leaves out a figure (§1's table): a figure's value
/// moves the application's digest and not the identity; an entry point and the code's digest move both.
#[test]
fn the_code_identity_moves_with_an_entry_point_and_the_code_and_no_figure() {
    let file = the_heartbeat_s_file();
    let identity = |text: &str| {
        let plan = lowered(&heartbeat(), Some(text)).expect("it lowers");
        let application = plan
            .lines()
            .find(|line| line.starts_with("(application "))
            .expect("its line")
            .to_owned();
        (
            archogen_plan::identity::code_identity(&plan).expect("an identity"),
            application,
        )
    };
    let (base, application) = identity(&file);
    let figure = file.replacen("(value 200) (unit us)", "(value 300) (unit us)", 1);
    let (same, other_application) = identity(&figure);
    assert_eq!(same, base, "a figure is no code");
    assert_ne!(other_application, application, "the file moved");
    // A comment or spacing moves nothing.
    let (spaced, spaced_application) = identity(&file.replace("; an application's inputs\n", ""));
    assert_eq!((spaced, spaced_application), (base, application.clone()));
    let entry = file.replace("\"app::beat\"", "\"app::tick\"");
    assert_ne!(identity(&entry).0, base, "an entry point is code");
    let code = file.replace(
        "sha256:0000000000000000000000000000000000000000000000000000000000000000",
        "sha256:1111111111111111111111111111111111111111111111111111111111111111",
    );
    assert_ne!(identity(&code).0, base, "the code's digest is code");
}

/// A file that does not read, or names another format, is `invalid-description`, naming the file (§7).
#[test]
fn a_file_that_does_not_read_or_is_another_format_is_refused() {
    for text in [
        "(".to_owned(),
        the_heartbeat_s_file().replace(FORMAT, "archogen-application/2"),
        the_heartbeat_s_file().replace("(task chime", "(task chime (cheese yes)"),
    ] {
        let refusal = lowered(&heartbeat(), Some(&text)).expect_err("refused");
        assert_eq!(refusal.code, "invalid-description", "{refusal}");
        assert!(refusal.message.contains("`app.eadl`"), "{refusal}");
    }
    let refusal = lowered(
        &heartbeat(),
        Some(&the_heartbeat_s_file().replace(FORMAT, "archogen-application/2")),
    )
    .expect_err("refused");
    assert!(
        refusal.message.contains("archogen-application/2"),
        "{refusal}"
    );
}

/// A file of another target is `invalid-description`, naming both (§1, §7).
#[test]
fn a_file_of_another_target_is_refused_naming_both() {
    let text =
        the_heartbeat_s_file().replace("(target hosted-playground)", "(target riscv-virt-up)");
    let refusal = lowered(&heartbeat(), Some(&text)).expect_err("refused");
    assert_eq!(refusal.code, "invalid-description");
    assert!(
        refusal.message.contains("`riscv-virt-up`")
            && refusal.message.contains("`hosted-playground`"),
        "{refusal}"
    );
}

/// Each other thing `Application::unmet` reports is `invalid-description` (§7): another system, a task the file leaves
/// out, a task the plan has not, a call into a node the task's lines do not name.
#[test]
fn what_the_file_leaves_unmet_is_refused() {
    for (text, words) in [
        (
            a_file("other", &[("beat", ""), ("chime", "")]),
            "are for `other`",
        ),
        (
            a_file("heartbeat", &[("beat", "")]),
            "nothing is said of the task `chime`",
        ),
        (
            a_file("heartbeat", &[("beat", ""), ("chime", ""), ("extra", "")]),
            "no task `extra`",
        ),
        (
            the_heartbeat_s_file().replacen(
                &format!("(node-calls {LIST})"),
                &format!("(node-calls {LIST} (call nowhere.node 1))"),
                1,
            ),
            "calls `nowhere.node`",
        ),
    ] {
        let refusal = lowered(&heartbeat(), Some(&text)).expect_err("refused");
        assert_eq!(refusal.code, "invalid-description", "{refusal}");
        assert!(
            refusal.message.contains("`app.eadl`") && refusal.message.contains(words),
            "{words}: {refusal}"
        );
    }
}

/// §7's order: the file not reading before anything of the record, and the record's refusals before the file's unmet.
#[test]
fn the_file_s_refusals_stand_in_section_7_s_order() {
    let placement =
        fs::read_to_string(root().join("crates/archogen-plan/tests/corpus/placement.eadl"))
            .expect("the fixture");
    assert_eq!(
        lowered(&placement, None).expect_err("refused").code,
        "unsupported-profile"
    );
    assert_eq!(
        lowered(&placement, Some("(")).expect_err("refused").code,
        "invalid-description",
        "the file not reading comes first"
    );
    let unmet = a_file("other", &[]);
    assert_eq!(
        lowered(&placement, Some(&unmet)).expect_err("refused").code,
        "unsupported-profile",
        "the record's refusals come before what the file leaves unmet"
    );
}

/// `RA-H7`: a task writing an alarm on the counter itself that is no wake owes `deadlines-within-reach` where no file is
/// given and where its file does not name that alarm, and not where it does (§3).
#[test]
fn a_task_writing_an_alarm_on_the_counter_owes_its_word_until_it_names_it() {
    let description =
        fs::read_to_string(root().join("crates/archogen-plan/tests/corpus/dod-bare.eadl"))
            .expect("the fixture");
    let node = "(adapter deadline-over-delay timer.delay time.deadline)";
    let owed = "(owed (task bare t) deadlines-within-reach (by application))";
    let plan = lowered(&description, None).expect("it lowers");
    assert!(plan.lines().any(|line| line == owed), "{plan}");
    let unnamed = a_file("bare", &[("t", "")]);
    let plan = lowered(&description, Some(&unnamed)).expect("it lowers");
    assert!(plan.lines().any(|line| line == owed), "{plan}");
    let named = a_file("bare", &[("t", &format!(" {node}"))]);
    let plan = lowered(&description, Some(&named)).expect("it lowers");
    assert!(
        !plan.contains("deadlines-within-reach (by application)"),
        "{plan}"
    );
    assert!(
        plan.contains(&format!("(deadlines-within-reach {node})")),
        "{plan}"
    );
    // A wake asks none: the record's worked epoch extender's task owes no word of its alarm.
    let wake = fs::read_to_string(
        root().join("crates/archogen-plan/tests/corpus/worked-epoch-extender.eadl"),
    )
    .expect("the fixture");
    let plan = lowered(&wake, None).expect("it lowers");
    assert!(!plan.contains("deadlines-within-reach"), "{plan}");
}
