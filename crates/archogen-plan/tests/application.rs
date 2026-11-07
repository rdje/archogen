//! The application inputs' reader (leaf `M4.12`; `docs/decisions/decision_application-inputs.md`): a whole file reads, every malformed
//! input is refused naming what is wrong, and every fact a plan owes `(by application)` is held against the three goldens.

use archogen_plan::application::{read, Binary, Category, CLAUSE_PARTS, FORMAT, HELD_OF_A_TASK};
use archogen_plan::contents;

const BOUND: &str = r#"(value 200) (unit us) (scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by the loop's bound")"#;

fn a_task(name: &str) -> String {
    format!(
        r#"(task {name}
  (entry-point "app::{name}")
  (execution-bound {BOUND})
  (os-calls (evidence analytically-established) (locator "docs/argument.md") (basis "by reading") (call mask 2) (call unmask 2))
  (node-calls (evidence analytically-established) (locator "docs/argument.md") (basis "by reading"))
  (masked-runs (evidence analytically-established)
    (run tail (value 5) (unit us) (scope "the tail's update") (binary unbuilt) (locator "docs/argument.md") (basis "four loads") (call unmask 1)))
  (stack-bound (value 512) (unit bytes) (scope "its deepest path") (binary unbuilt) (evidence assumed) (basis "chosen, to be measured"))
  (suspends no) (locks-scheduler no) (shares-outside-sections no)
  (masks-only-through-api yes)
  (figures-under-preemption yes))
"#
    )
}

/// The whole line a file declares the fact by.
fn leaves_alone() -> String {
    format!(
        "(leaves-interrupt-hardware-alone yes (clause \"{}\") (basis \"it touches no device\"))\n",
        CLAUSE_PARTS.join(" ")
    )
}

fn a_file(system: &str, tasks: &[&str]) -> String {
    let each: String = tasks.iter().map(|name| a_task(name)).collect();
    format!(
        r#"(format "{FORMAT}")
(system {system})
(target hosted-playground)
{}{each}"#,
        leaves_alone()
    )
}

#[test]
fn a_whole_file_reads_as_it_says() {
    let application = read(&a_file("heartbeat", &["beat", "chime"])).expect("it reads");
    assert_eq!(application.system, "heartbeat");
    assert_eq!(application.target, "hosted-playground");
    assert!(application.leaves_interrupt_hardware_alone);
    assert_eq!(application.tasks.len(), 2);
    let beat = &application.tasks[0];
    assert_eq!(beat.entry_point, "app::beat");
    assert_eq!(beat.execution_bound.value, 200);
    assert_eq!(beat.execution_bound.unit, "us");
    assert_eq!(beat.execution_bound.binary, Binary::Unbuilt);
    assert_eq!(
        beat.execution_bound.evidence.category,
        Category::AnalyticallyEstablished
    );
    assert_eq!(
        beat.os_calls.calls,
        [("mask".to_owned(), 2), ("unmask".to_owned(), 2)]
    );
    assert!(beat.node_calls.calls.is_empty());
    assert_eq!(beat.masked_runs.runs[0].calls, [("unmask".to_owned(), 1)]);
    assert_eq!(beat.stack_bound.evidence.category, Category::Assumed);
    assert!(beat.stack_bound.evidence.locator.is_none());
    assert!(beat.masks_only_through_api);
    assert!(!beat.suspends);
}

/// Each malformed input, and a word of what it is told.
#[test]
fn each_malformed_input_is_refused_saying_what_is_wrong() {
    let good = a_file("s", &["t"]);
    let replaced = |from: &str, to: &str| {
        assert!(good.contains(from), "{from}");
        good.replacen(from, to, 1)
    };
    for (text, said) in [
        ("(junk".to_owned(), "does not read"),
        (replaced(FORMAT, "archogen-application/2"), "this reads"),
        (replaced(&format!("(format \"{FORMAT}\")\n"), ""), "`format`"),
        (replaced("(system s)\n", ""), "`system`"),
        (replaced("(target hosted-playground)\n", ""), "`target`"),
        (replaced("(system s)", "(system s) (system s)"), "written twice"),
        (replaced("(system s)", "(system s) (other x)"), "no clause is called"),
        (replaced(&leaves_alone(), ""), "leaves-interrupt-hardware-alone"),
        (replaced("hardware-alone yes (clause", "hardware-alone maybe (clause"), "`yes` or `no`"),
        (replaced(&format!(" (clause \"{}\")", CLAUSE_PARTS.join(" ")), ""), "`clause`"),
        (replaced(CLAUSE_PARTS[1], "it behaves"), "and these are others"),
        (replaced(" (basis \"it touches no device\")", ""), "`basis`"),
        (good.clone() + &a_task("t"), "written twice"),
        (replaced("(entry-point \"app::t\")", ""), "`entry-point`"),
        (replaced("(entry-point \"app::t\")", "(entry-point \"\")"), "says something"),
        (replaced("(entry-point \"app::t\")", "(entry-point app)"), "says something"),
        (replaced("(value 200)", "(value -1)"), "below nothing"),
        (replaced("(value 200)", "(value 1.5)"), "whole number"),
        (replaced("(value 200) ", ""), "`value`"),
        (replaced("(unit us)", "(unit parsec)"), "a unit here"),
        (replaced("(unit us) ", ""), "`unit`"),
        (replaced("(scope \"one job\") ", ""), "`scope`"),
        (replaced("(binary unbuilt) (evidence analytically-established) (locator \"docs/argument.md\") (basis \"by the loop's bound\")", "(evidence analytically-established) (locator \"docs/argument.md\") (basis \"by the loop's bound\")"), "`binary`"),
        (replaced("(binary unbuilt)", "(binary \"sha256:1\")"), "sha256"),
        (replaced("(evidence analytically-established) (locator \"docs/argument.md\") (basis \"by the loop's bound\")", "(evidence analytically-established) (basis \"by the loop's bound\")"), "`locator`"),
        (replaced("(evidence analytically-established) (locator \"docs/argument.md\") (basis \"by the loop's bound\")", "(evidence proven) (locator \"docs/argument.md\") (basis \"by the loop's bound\")"), "four categories"),
        (replaced("(evidence analytically-established) (locator \"docs/argument.md\") (basis \"by the loop's bound\")", "(evidence analytically-established) (locator \"docs/argument.md\")"), "`basis`"),
        (replaced("(evidence analytically-established) (locator \"docs/argument.md\") (basis \"by the loop's bound\")", "(evidence observed-maximum (observed 100) (safety-factor 1 2)) (locator \"docs/argument.md\") (basis \"b\")"), "pads"),
        (replaced("(evidence analytically-established) (locator \"docs/argument.md\") (basis \"by the loop's bound\")", "(evidence observed-maximum (observed 100) (safety-factor 3 2)) (locator \"docs/argument.md\") (basis \"b\")"), "by that factor is 150"),
        (replaced("(call mask 2)", "(call mask 2) (call mask 1)"), "counted twice"),
        (replaced("(call mask 2)", "(call mask)"), "`(call NAME COUNT)`"),
        (replaced("(call mask 2)", "(call mask two)"), "whole number"),
        (replaced("(node-calls (evidence analytically-established) (locator \"docs/argument.md\") (basis \"by reading\"))", ""), "`node-calls`"),
        (replaced("(os-calls (evidence analytically-established) (locator \"docs/argument.md\") (basis \"by reading\") ", "(os-calls "), "`evidence`"),
        (replaced("(run tail (value 5)", "(run tail (value 5) (scope \"again\")"), "written twice"),
        (replaced("(unit bytes)", "(unit us)"), "a unit here"),
        (replaced("(suspends no) ", ""), "`suspends`"),
        (replaced("(suspends no)", "(suspends sometimes)"), "`yes` or `no`"),
        (replaced("(masks-only-through-api yes)\n", ""), "`masks-only-through-api`"),
        (replaced("(figures-under-preemption yes)", ""), "`figures-under-preemption`"),
    ] {
        let refused = read(&text).expect_err(said);
        assert!(refused.to_string().contains(said), "`{said}`: {refused}");
    }
}

/// A safety factor is held to its arithmetic, and it changes no category.
#[test]
fn a_safety_factor_is_arithmetic_and_changes_no_category() {
    let text = a_file("s", &["t"]).replace(
        "(evidence analytically-established) (locator \"docs/argument.md\") (basis \"by the loop's bound\")",
        "(evidence observed-maximum (observed 100) (safety-factor 2 1)) (locator \"docs/argument.md\") (basis \"seen\")",
    );
    let application = read(&text).expect("it reads");
    assert_eq!(
        application.tasks[0].execution_bound.evidence.category,
        Category::ObservedMaximum
    );
}

/// What `RA-H7` asks of a task writing an alarm on the counter's own time base.
#[test]
fn a_task_says_its_deadlines_stand_within_an_alarm_s_reach() {
    let text = a_file("s", &["t"]).replace(
        "(suspends no)",
        "(deadlines-within-reach timer.short) (deadlines-within-reach timer.long) (suspends no)",
    );
    let application = read(&text).expect("it reads");
    assert_eq!(
        application.tasks[0].deadlines_within_reach,
        ["timer.short", "timer.long"]
    );
    let twice = a_file("s", &["t"]).replace(
        "(suspends no)",
        "(deadlines-within-reach a) (deadlines-within-reach a) (suspends no)",
    );
    assert!(read(&twice).is_err());
}

fn golden(name: &str) -> String {
    let path = format!("{}/tests/goldens/{name}.plan", env!("CARGO_MANIFEST_DIR"));
    std::fs::read_to_string(path).expect("a golden")
}

/// Every fact each golden plan owes `(by application)` is one this format holds, and inputs for its tasks meet the plan.
#[test]
fn every_fact_a_plan_owes_the_application_is_held() {
    for name in ["heartbeat", "sporadic", "epoch-extender"] {
        let plan = contents::read(&golden(name)).expect("the golden reads");
        let owed: Vec<_> = plan
            .owed
            .iter()
            .filter(|owed| owed.by == "application")
            .collect();
        assert!(!owed.is_empty(), "{name}");
        for each in &owed {
            assert!(
                each.fact == "leaves-interrupt-hardware-alone"
                    || HELD_OF_A_TASK.contains(&each.fact.as_str()),
                "{name}: {}",
                each.fact
            );
        }
        let tasks: Vec<&str> = plan.tasks.iter().map(|task| task.name.as_str()).collect();
        let application = read(&a_file(&plan.system, &tasks)).expect("it reads");
        assert_eq!(application.unmet(&plan), Vec::<String>::new(), "{name}");
    }
}

/// What the plan holds them against.
#[test]
fn what_is_unmet_of_a_plan_is_said() {
    let plan = contents::read(&golden("heartbeat")).expect("the golden reads");
    let tasks: Vec<&str> = plan.tasks.iter().map(|task| task.name.as_str()).collect();
    let other = read(&a_file("other", &tasks)).expect("it reads");
    assert!(other
        .unmet(&plan)
        .iter()
        .any(|said| said.contains("are for `other`")));
    let fewer = read(&a_file(&plan.system, &tasks[..tasks.len() - 1])).expect("it reads");
    assert!(fewer
        .unmet(&plan)
        .iter()
        .any(|said| said.contains("nothing is said of the task")));
    let mut more = tasks.clone();
    more.push("extra");
    let more = read(&a_file(&plan.system, &more)).expect("it reads");
    assert!(more
        .unmet(&plan)
        .iter()
        .any(|said| said.contains("no task `extra`")));
    // A call into a node the task's lines do not name: only through a node they do.
    let stray = a_file(&plan.system, &tasks).replace(
        "(basis \"by reading\"))",
        "(basis \"by reading\") (call nowhere.node 1))",
    );
    let stray = read(&stray).expect("it reads");
    assert!(stray
        .unmet(&plan)
        .iter()
        .any(|said| said.contains("calls `nowhere.node`")));
    // A fact a plan owes that this format does not hold is said, so a new one cannot pass unread.
    let mut newer = plan.clone();
    newer.owed.push(contents::Owed {
        subject: format!("(task {} {})", plan.system, tasks[0]),
        fact: "something-new".to_owned(),
        by: "application".to_owned(),
    });
    let application = read(&a_file(&plan.system, &tasks)).expect("it reads");
    assert!(application
        .unmet(&newer)
        .iter()
        .any(|said| said.contains("something-new")));
}

/// What a file quotes is the composite-inputs record's own words, read from there: a change to them moves this, and a file declaring the old ones stops reading.
#[test]
fn the_clause_a_file_quotes_is_the_record_s_own_words() {
    let record = include_str!("../../../docs/specs/catalog/decision_runtime-composite-inputs.md");
    let plain: String = record
        .replace("**", "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ");
    for part in CLAUSE_PARTS {
        assert!(plain.contains(part), "{part}");
    }
}

/// What a yes or a no cannot carry: the fact rests on each task's stack bound, by its category, since an overrun of it is a write the clause forbids.
#[test]
fn the_fact_says_what_it_rests_on() {
    let application = read(&a_file("s", &["one", "two"])).expect("it reads");
    assert_eq!(
        application.leaves_alone_rests_on(),
        [("one", Category::Assumed), ("two", Category::Assumed)]
    );
}
