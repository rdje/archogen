//! The application inputs' reader (leaf `M4.12`; `docs/decisions/decision_application-inputs.md`): a whole file reads, every malformed
//! input is refused naming what is wrong, and every fact a plan owes `(by application)` is held against the three goldens.

use archogen_plan::application::{
    read, Binary, Category, Ending, Observation, CLAUSE_CUT, CLAUSE_PARTS, FORMAT, HELD_OF_A_TASK,
    MOST_BYTES,
};
use archogen_plan::contents;

const BOUND: &str = r#"(value 200) (unit us) (scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by the loop's bound")"#;
const LIST: &str = r#"(scope "one job") (binary unbuilt) (evidence analytically-established) (locator "docs/argument.md") (basis "by reading")"#;
const CODE: &str = r#"(code (crate app) (place "app") (digest "sha256:0000000000000000000000000000000000000000000000000000000000000000"))"#;
const INITIALISATION: &str = r#"(initialisation (entry-point "app::init") (stack-bound (value 256) (unit byte) (scope "its deepest path") (binary unbuilt) (evidence assumed) (basis "chosen, to be measured")) (node-calls (scope "all of it") (binary unbuilt) (evidence assumed) (basis "it calls none")))"#;

fn a_task(name: &str) -> String {
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
  (deadlines-within-reach))
"#
    )
}

fn the_clause() -> String {
    CLAUSE_PARTS.join(CLAUSE_CUT)
}

/// The whole line a file declares the fact by.
fn leaves_alone() -> String {
    format!(
        "(leaves-interrupt-hardware-alone yes (clause \"{}\") (basis \"it touches no device\"))\n",
        the_clause()
    )
}

fn a_file(system: &str, tasks: &[&str]) -> String {
    let each: String = tasks.iter().map(|name| a_task(name)).collect();
    format!(
        r#"(format "{FORMAT}")
(system {system})
(target hosted-playground)
{CODE}
{}{INITIALISATION}
{each}"#,
        leaves_alone()
    )
}

#[test]
fn a_whole_file_reads_as_it_says() {
    let application = read(&a_file("heartbeat", &["beat", "chime"])).expect("it reads");
    assert_eq!(application.system, "heartbeat");
    assert_eq!(application.target, "hosted-playground");
    assert!(application.leaves_interrupt_hardware_alone);
    let initialisation = application.initialisation.as_ref().expect("it has one");
    assert_eq!(initialisation.entry_point, "app::init");
    assert_eq!(initialisation.stack_bound.value, 256);
    let none =
        read(&a_file("heartbeat", &["beat"]).replace(INITIALISATION, "(initialisation none)"))
            .expect("it reads");
    assert_eq!(
        none.initialisation, None,
        "an application may have none, and says so"
    );
    assert!(initialisation.node_calls.calls.is_empty());
    // A name is the language's symbol, as a description's is, so what a plan can name a file can: `_`, dashes, a backslash, a line separator.
    for name in ["_", "--", "be\\at", "ta\u{2028}il", "\u{200b}"] {
        let named = read(
            &a_file("heartbeat", &["beat"]).replace("(task beat\n", &format!("(task {name}\n")),
        )
        .unwrap_or_else(|refused| panic!("{name:?}: {refused}"));
        assert_eq!(named.tasks[0].name, name);
    }
    // And a place is a path from the file, the file's own directory and the one above among them.
    for place in [".", "..", "../..", "../app", "./app", "a/b.c/d"] {
        let placed = read(
            &a_file("heartbeat", &["beat"])
                .replace("(place \"app\")", &format!("(place \"{place}\")")),
        )
        .unwrap_or_else(|refused| panic!("{place}: {refused}"));
        assert_eq!(placed.code.place, place);
    }
    assert_eq!(application.code.name, "app");
    assert_eq!(application.code.place, "app");
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
        [
            ("mask".to_owned(), 2),
            ("unmask".to_owned(), 2),
            ("release".to_owned(), 1)
        ]
    );
    assert_eq!(beat.os_calls.scope, "one job");
    assert_eq!(beat.os_calls.binary, Binary::Unbuilt);
    assert!(beat.node_calls.calls.is_empty());
    assert_eq!(
        beat.node_calls.evidence.locator.as_deref(),
        Some("docs/argument.md")
    );
    assert_eq!(beat.masked_runs.runs[0].ending, Ending::Unmask);
    assert_eq!(beat.masked_runs.runs[0].calls, [("release".to_owned(), 1)]);
    assert_eq!(beat.stack_bound.evidence.category, Category::Assumed);
    assert!(beat.stack_bound.evidence.locator.is_none());
    assert!(beat.masks_only_through_api);
    assert!(!beat.suspends);
    assert!(beat.deadlines_within_reach.is_empty());
}

/// Each malformed input, and a word of what it is told.
#[test]
fn each_malformed_input_is_refused_saying_what_is_wrong() {
    let good = a_file("s", &["t"]);
    let replaced = |from: &str, to: &str| {
        assert!(good.contains(from), "{from}");
        good.replacen(from, to, 1)
    };
    let bound_evidence = "(evidence analytically-established) (locator \"docs/argument.md\") (basis \"by the loop's bound\")";
    for (text, said) in [
        ("(junk".to_owned(), "the reader of forms refuses it: "),
        (" ".repeat(MOST_BYTES + 1), "past the"),
        (format!("(eadl-version eadl/1)\n{good}"), "no clause is called `eadl-version`"),
        (format!("\u{feff}{good}"), "byte-order mark"),
        (replaced(&format!("{CODE}\n"), ""), "`code`"),
        (replaced("(crate app)", "(crate 1)"), "wants one name"),
        (replaced("(crate app)", "(crate fn)"), "no name a Rust path can hold"),
        (replaced("(place \"app\") ", ""), "`place`"),
        // A place is a path from the file: never one from the root, an empty name in it, or a name of dots but `.` and `..`.
        (replaced("(place \"app\")", "(place \"/\")"), "`place` is a path from this file"),
        (replaced("(place \"app\")", "(place \"/abs/app\")"), "`place` is a path from this file"),
        (replaced("(place \"app\")", "(place \"...\")"), "`place` is a path from this file"),
        (replaced("(place \"app\")", "(place \"a//b\")"), "`place` is a path from this file"),
        (replaced("(place \"app\")", "(place \"app/\")"), "`place` is a path from this file"),
        (replaced("(place \"app\")", "(place \"\")"), "`place` is a path from this file"),
        (replaced("sha256:0000000000000000000000000000000000000000000000000000000000000000", "sha256:1"), "`digest`"),
        (replaced(FORMAT, "archogen-application/2").replace("(system s)", "(system s) (newly-added x)"), "this reads"),
        (replaced(FORMAT, "archogen-application/2"), "this reads"),
        (replaced(&format!("(format \"{FORMAT}\")\n"), ""), "`format`"),
        (replaced("(system s)\n", ""), "`system`"),
        (replaced("(target hosted-playground)\n", ""), "`target`"),
        (replaced("(system s)", "(system s) (system s)"), "written twice"),
        (replaced("(system s)", "(system s) (other x)"), "no clause is called"),
        (replaced(&leaves_alone(), ""), "leaves-interrupt-hardware-alone"),
        (replaced("hardware-alone yes (clause", "hardware-alone maybe (clause"), "`yes` or `no`"),
        (replaced(&format!(" (clause \"{}\")", the_clause()), ""), "`clause`"),
        (replaced(CLAUSE_PARTS[1], "it behaves"), "and these are others"),
        (replaced(CLAUSE_CUT, " "), "and these are others"),
        (replaced(" (basis \"it touches no device\")", ""), "`basis`"),
        (replaced(INITIALISATION, ""), "`initialisation`"),
        (replaced(INITIALISATION, "(initialisation)"), "`entry-point`"),
        (replaced("(initialisation (entry-point \"app::init\") ", "(initialisation "), "`entry-point`"),
        (replaced("(initialisation (entry-point \"app::init\") ", "(initialisation (entry-point \"other::init\") "), "a path the boot can call"),
        (replaced("(initialisation (entry-point \"app::init\") ", "(initialisation (entry-point \"app::fn\") "), "a path the boot can call"),
        // The boot's refusal says which names it reads, as a job's does (round 7, AI7-5).
        (replaced("(initialisation (entry-point \"app::init\") ", "(initialisation (entry-point \"app::caf\u{e9}\") "), "a path the boot can call in `app`, such as `app::init`, each name of ASCII letters, digits and `_`"),
        (replaced(INITIALISATION, "(initialisation (entry-point \"app::init\"))"), "`stack-bound`"),
        (replaced(" (node-calls (scope \"all of it\") (binary unbuilt) (evidence assumed) (basis \"it calls none\"))", ""), "`node-calls`"),
        (replaced(INITIALISATION, "(initialisation nothing)"), "`nothing` is no clause"),
        (replaced(INITIALISATION, "(initialisation none none)"), "is no clause"),
        (good.clone() + &a_task("t"), "written twice"),
        (replaced("(entry-point \"app::t\")", ""), "`entry-point`"),
        (replaced("(entry-point \"app::t\")", "(entry-point \"\")"), "says something"),
        (replaced("(entry-point \"app::t\")", "(entry-point app)"), "says something"),
        (replaced("(entry-point \"app::t\")", "(entry-point \"x(); } unsafe { evil(); \")"), "a path a job can call"),
        (replaced("(entry-point \"app::t\")", "(entry-point \"app::\")"), "a path a job can call"),
        (replaced("(entry-point \"app::t\")", "(entry-point \"return\")"), "a path a job can call"),
        (replaced("(entry-point \"app::t\")", "(entry-point \"app::loop\")"), "a path a job can call"),
        // Rust calls each of these, and the record writes neither: the refusal says which names it reads.
        (replaced("(entry-point \"app::t\")", "(entry-point \"app::b\u{e9}at\")"), "each name of ASCII letters, digits and `_`"),
        (replaced("(entry-point \"app::t\")", "(entry-point \"app::r#loop\")"), "each name of ASCII letters, digits and `_`"),
        (replaced("(entry-point \"app::t\")", "(entry-point \"other::t\")"), "a path a job can call"),
        (replaced("(scope \"one job\")", "(scope \"\u{200b}\")"), "says something"),
        (replaced("(scope \"one job\")", "(scope \"one\u{2028}job\")"), "line break"),
        (replaced("(scope \"one job\")", "(scope \"one\\\\job\")"), "a backslash"),
        (replaced("(entry-point \"app::t\")", "(entry-point \"app\\n::t\")"), "holds a line break"),
        (replaced("(scope \"one job\")", "(scope \"one \\\"job\\\"\")"), "a quote"),
        (replaced("(value 200)", "(value -1)"), "below nothing"),
        (replaced("(value 200)", "(value 1.5)"), "whole number"),
        (replaced("(value 200) ", ""), "`value`"),
        (replaced("(unit us)", "(unit parsec)"), "a unit here"),
        (replaced("(unit us) ", ""), "`unit`"),
        (replaced("(scope \"one job\") ", ""), "`scope`"),
        (replaced(&format!("(binary unbuilt) {bound_evidence}"), bound_evidence), "`binary`"),
        (replaced("(binary unbuilt)", "(binary \"sha256:1\")"), "sha256"),
        (replaced("(binary unbuilt)", "(binary \"sha256:AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA\")"), "in lower case"),
        (replaced("sha256:0000000000000000000000000000000000000000000000000000000000000000", "sha256:00000000000000000000000000000000000000000000000000000000000000AA"), "in lower case"),
        (replaced(bound_evidence, "(evidence analytically-established) (basis \"by the loop's bound\")"), "`locator`"),
        (replaced(bound_evidence, "(evidence proven) (locator \"docs/argument.md\") (basis \"by the loop's bound\")"), "four categories"),
        (replaced(bound_evidence, "(evidence assumed (observed 1) (safety-factor 1 1)) (basis \"b\")"), "says more than its category"),
        (replaced(bound_evidence, "(evidence analytically-established (observed 1)) (locator \"docs/argument.md\") (basis \"b\")"), "says more than its category"),
        (replaced(bound_evidence, "(evidence analytically-established) (locator \"docs/argument.md\")"), "`basis`"),
        (replaced(bound_evidence, "(evidence observed-maximum (observed 100) (safety-factor 1 2)) (locator \"docs/argument.md\") (basis \"b\")"), "pads"),
        (replaced(bound_evidence, "(evidence observed-maximum (observed 100) (safety-factor 3 2)) (locator \"docs/argument.md\") (basis \"b\")"), "by that factor is 150"),
        (replaced("(call mask 2)", "(call mask 2) (call mask 1)"), "counted twice"),
        (replaced("(call mask 2)", "(call mask)"), "`(call NAME COUNT)`"),
        (replaced("(call mask 2)", "(call mask two)"), "whole number"),
        (replaced("(call mask 2)", "(call (other mask) 2)"), "calls no primitive"),
        (replaced("(call mask 2)", "(call (adapter x y) 2)"), "calls no primitive"),
        (replaced(&format!("(node-calls {LIST})"), &format!("(node-calls {LIST} (call (adapter) 1))")), "calls no node"),
        (replaced(&format!("(node-calls {LIST})"), &format!("(node-calls {LIST} (call (adapter 1 \"x\" (y)) 1))")), "calls no node"),
        (replaced("(call release 1)))", "(call spin 5)))"), "says it makes 0"),
        (replaced("(call release 1)))", "(call release 2)))"), "says it makes 1"),
        (replaced("(call release 1)))", "(node-call a.node 1)))"), "says it makes 0"),
        // A run is held to the two calls that make it one: the `mask` that begins it and the `unmask` that ends one ending so are its job's calls too.
        (replaced("(call mask 2) (call unmask 2) (call release 1))", "(call release 1))"), "the `mask` that begins it among them, and its job's `os-calls` says it makes 0"),
        (replaced("(call mask 2) (call unmask 2) (call release 1))", "(call mask 1) (call release 1))"), "the `unmask` that ends it among them, and its job's `os-calls` says it makes 0"),
        (replaced("(call release 1)))", "(call release 1) (call mask 2)))"), "calls `mask` 3 time(s), the `mask` that begins it among them, and its job's `os-calls` says it makes 2"),
        (replaced("(call release 1)))", "(call release 1) (call unmask 2)))"), "calls `unmask` 3 time(s), the `unmask` that ends it among them"),
        (replaced("(run tail (ending unmask) (value 5) (unit us)", "(run tail (ending unmask) (value 5000) (unit us)"), "past its job's"),
        (replaced(&format!("(node-calls {LIST})\n"), ""), "`node-calls`"),
        (replaced(&format!("(os-calls {LIST} "), "(os-calls "), "`scope`"),
        (replaced(&format!("(os-calls {LIST} "), "(os-calls (scope \"one job\") (evidence assumed) (basis \"b\") "), "`binary`"),
        (replaced(&format!("(os-calls {LIST} "), "(os-calls (scope \"one job\") (binary unbuilt) (evidence observed-maximum (observed 3) (safety-factor 2 1)) (locator \"l\") (basis \"b\") "), "a list says"),
        // An observation is of an image: none stands beside `(binary unbuilt)`, of a bound, a list or a run.
        (replaced(bound_evidence, "(evidence observed-maximum (observed 100) (safety-factor 2 1)) (locator \"docs/argument.md\") (basis \"b\")"), "an observed maximum is of an image"),
        (replaced(&format!("(os-calls {LIST} "), "(os-calls (scope \"one job\") (binary unbuilt) (evidence observed-maximum) (locator \"l\") (basis \"b\") "), "an observed maximum is of an image"),
        (replaced(&format!("(masked-runs {LIST}"), "(masked-runs (scope \"one job\") (binary unbuilt) (evidence observed-maximum) (locator \"l\") (basis \"b\")"), "an observed maximum is of an image"),
        (replaced("(binary unbuilt) (evidence analytically-established) (locator \"docs/argument.md\") (basis \"four loads\")", "(binary unbuilt) (evidence observed-maximum (observed 5) (safety-factor 1 1)) (locator \"l\") (basis \"four loads\")"), "an observed maximum is of an image"),
        (replaced(&format!("(masked-runs {LIST}"), "(masked-runs"), "`scope`"),
        (replaced("(run tail (ending unmask) (value 5)", "(run tail (value 5)"), "one `ending`"),
        (replaced("(run tail (ending unmask)", "(run tail (ending late)"), "`unmask` or `completion`"),
        (replaced("(run tail (ending unmask) (value 5)", "(run tail (ending unmask) (ending completion) (value 5)"), "one `ending`"),
        (replaced("(run tail (ending unmask) (value 5)", "(run tail (ending unmask) (value 5) (scope \"again\")"), "written twice"),
        (replaced("(value 256) (unit byte)", "(value 256) (unit us)"), "a unit here"),
        (replaced("(unit byte)", "(unit bytes)"), "a unit here"),
        (replaced("(suspends no) ", ""), "`suspends`"),
        (replaced("(suspends no)", "(suspends sometimes)"), "`yes` or `no`"),
        (replaced("(masks-only-through-api yes)\n", ""), "`masks-only-through-api`"),
        (replaced("(figures-under-preemption yes)", ""), "`figures-under-preemption`"),
        (replaced("(deadlines-within-reach)", ""), "`deadlines-within-reach`"),
        (replaced("(deadlines-within-reach)", "(deadlines-within-reach a a)"), "written twice"),
        (replaced("(deadlines-within-reach)", "(deadlines-within-reach 1)"), "names no alarm"),
        (replaced("(deadlines-within-reach)", "(deadlines-within-reach (adapter))"), "names no alarm"),
        // A `mask` is held to a run, as a run is to its `mask`: a job that calls one states the run it begins.
        (replaced("(run tail (ending unmask) (value 5) (unit us) (scope \"the tail's update\") (binary unbuilt) (evidence analytically-established) (locator \"docs/argument.md\") (basis \"four loads\") (call release 1))", ""), "it states no masked run"),
        // Another format's file is told its format first, whatever its language's version says to the reader of forms.
        (format!("(eadl-version eadl/2)\n{}", replaced(FORMAT, "archogen-application/2")), "this reads"),
    ] {
        let refused = read(&text).expect_err(said);
        assert!(refused.to_string().contains(said), "`{said}`: {refused}");
    }
}

/// A mask inside a mask is closed by an `unmask` while its run ends at the job's completion, so an `unmask` is held to no run's ending: the truthful file of
/// `mask; mask; unmask; return` reads, two masks and one unmask in one run that holds the inner pair and ends at completion (round 5, AI5-1; round 6, AI6-5, which found
/// this test's job to be another than its words said).
#[test]
fn an_unmask_is_held_to_no_run_s_ending() {
    let nested = a_file("s", &["t"])
        .replace(
            "(call mask 2) (call unmask 2) (call release 1))",
            "(call mask 2) (call unmask 1) (call release 1))",
        )
        .replace("(run tail (ending unmask)", "(run tail (ending completion)")
        .replace(
            "(basis \"four loads\") (call release 1)))",
            "(basis \"four loads\") (call mask 1) (call unmask 1) (call release 1)))",
        );
    // The job's one `unmask` is the run's own, and of its two `mask`s one begins the run and the run writes the other.
    assert_eq!(nested.matches("(call unmask 1)").count(), 2, "{nested}");
    assert_eq!(nested.matches("(call mask 1)").count(), 1, "{nested}");
    assert!(nested.contains("(run tail (ending completion)"), "{nested}");
    let application =
        read(&nested).expect("a job that calls `unmask` and ends its run at completion reads");
    assert!(application.tasks[0]
        .os_calls
        .calls
        .iter()
        .any(|(callee, count)| callee == "unmask" && *count > 0));
    assert!(application.tasks[0]
        .masked_runs
        .runs
        .iter()
        .all(|run| run.ending == Ending::Completion));
    let run = &application.tasks[0].masked_runs.runs[0];
    for (callee, count) in [("mask", 1), ("unmask", 1), ("release", 1)] {
        assert!(
            run.calls
                .iter()
                .any(|(called, times)| called == callee && *times == count),
            "the run's `{callee}`"
        );
    }
    // One `unmask` more in the run than its job makes is refused: the count is still held, where the ending is not.
    let over = nested.replace(
        "(call mask 1) (call unmask 1) (call release 1)))",
        "(call mask 1) (call unmask 2) (call release 1)))",
    );
    let refused = read(&over).expect_err("a run's `unmask` its job lacks");
    assert!(
        refused.to_string().contains("calls `unmask` 2 time(s)"),
        "{refused}"
    );
    // A `mask` is still held to a run: a job that calls one states the run it begins.
    let no_run = a_file("s", &["t"]);
    let from = no_run.find("(run tail").expect("its run");
    let mut depth = 0_i32;
    let to = from
        + no_run[from..]
            .char_indices()
            .find_map(|(at, c)| {
                depth += i32::from(c == '(') - i32::from(c == ')');
                (depth == 0).then_some(at + 1)
            })
            .expect("it closes");
    let refused =
        read(&format!("{}{}", &no_run[..from], &no_run[to..])).expect_err("a `mask` with no run");
    assert!(
        refused.to_string().contains("it states no masked run"),
        "{refused}"
    );
}

/// A safety factor is held to its arithmetic, changes no category, and what it padded is kept for what reads it.
#[test]
fn a_safety_factor_is_arithmetic_and_changes_no_category() {
    let unbuilt = a_file("s", &["t"]).replace(
        "(evidence analytically-established) (locator \"docs/argument.md\") (basis \"by the loop's bound\")",
        "(evidence observed-maximum (observed 100) (safety-factor 2 1)) (locator \"docs/argument.md\") (basis \"seen\")",
    );
    // An observation is of an image, so one beside `(binary unbuilt)` is refused (round 6, AI6-3), and reads once it names the image.
    let refused = read(&unbuilt).expect_err("an observation of no image");
    assert!(
        refused
            .to_string()
            .contains("an observed maximum is of an image"),
        "{refused}"
    );
    let text = unbuilt.replace(
        "(binary unbuilt) (evidence observed-maximum",
        "(binary \"sha256:1111111111111111111111111111111111111111111111111111111111111111\") (evidence observed-maximum",
    );
    assert_ne!(text, unbuilt);
    let application = read(&text).expect("it reads");
    assert_eq!(
        application.tasks[0].execution_bound.binary,
        Binary::Image(
            "sha256:1111111111111111111111111111111111111111111111111111111111111111".to_owned()
        )
    );
    let evidence = &application.tasks[0].execution_bound.evidence;
    assert_eq!(evidence.category, Category::ObservedMaximum);
    assert_eq!(
        evidence.observation,
        Some(Observation {
            observed: 100,
            numerator: 2,
            denominator: 1
        })
    );
}

/// The composite-inputs record's §6: a list carries one category, a run's bound keeps its own, each with its own padding, and an empty list still says what backs it. A run that ends both ways is written once for each.
#[test]
fn a_run_keeps_its_own_record_and_an_empty_list_still_says_what_backs_it() {
    let run = |name: &str, ending: &str, value: u64, evidence: &str| {
        // What was observed says the image it was observed on.
        let binary = if evidence.contains("observed-maximum") {
            "\"sha256:1111111111111111111111111111111111111111111111111111111111111111\""
        } else {
            "unbuilt"
        };
        format!("(run {name} (ending {ending}) (value {value}) (unit us) (scope \"s\") (binary {binary}) {evidence} (locator \"l\") (basis \"b\"))")
    };
    let runs = [
        run(
            "a",
            "unmask",
            150,
            "(evidence observed-maximum (observed 100) (safety-factor 3 2))",
        ),
        run("a", "completion", 30, "(evidence analytically-established)"),
        run("b", "unmask", 40, "(evidence observed-maximum)"),
    ]
    .join(" ");
    let original = a_file("s", &["t"]);
    let start = original.find("(masked-runs").expect("a list");
    let end = original.find("(stack-bound (value 512)").expect("a bound");
    let text = format!(
        "{}(masked-runs {LIST} {runs})\n  {}",
        &original[..start],
        &original[end..]
    );
    let application = read(&text).expect("it reads");
    let list = &application.tasks[0].masked_runs;
    assert_eq!(list.evidence.category, Category::AnalyticallyEstablished);
    assert_eq!(list.runs.len(), 3);
    assert_eq!(
        list.runs[0].bound.evidence.category,
        Category::ObservedMaximum
    );
    assert_eq!(list.runs[0].ending, Ending::Unmask);
    assert_eq!(list.runs[1].ending, Ending::Completion);
    assert_eq!(
        list.runs[1].bound.evidence.category,
        Category::AnalyticallyEstablished
    );
    assert_eq!(list.runs[1].bound.value, 30);
    // A job with no masked run calls no `mask`: one that named it would state a call that begins nothing.
    let empty = format!(
        "{}(masked-runs {LIST})\n  {}",
        &original[..start],
        &original[end..]
    )
    .replace("(call mask 2) (call unmask 2) ", "");
    let application = read(&empty).expect("it reads");
    let list = &application.tasks[0].masked_runs;
    assert!(list.runs.is_empty());
    assert_eq!(list.evidence.locator.as_deref(), Some("docs/argument.md"));
    assert_eq!(list.scope, "one job");
}

/// A node named by a list, as a task's own `bind` line names an instance, can be called and named as an alarm.
#[test]
fn a_node_an_instance_names_can_be_called() {
    let adapter = "(adapter epoch-extender timer.short)";
    let text = a_file("s", &["t"])
        .replace(
            &format!("(node-calls {LIST})"),
            &format!("(node-calls {LIST} (call {adapter} 1))"),
        )
        .replace(
            "(deadlines-within-reach)",
            &format!("(deadlines-within-reach {adapter})"),
        );
    let application = read(&text).expect("it reads");
    assert_eq!(
        application.tasks[0].node_calls.calls,
        [(adapter.to_owned(), 1)]
    );
    assert_eq!(application.tasks[0].deadlines_within_reach, [adapter]);
}

/// What `RA-H7` asks of a task writing an alarm on the counter's own time base.
#[test]
fn a_task_says_its_deadlines_stand_within_an_alarm_s_reach() {
    let text = a_file("s", &["t"]).replace(
        "(deadlines-within-reach)",
        "(deadlines-within-reach timer.short timer.long)",
    );
    let application = read(&text).expect("it reads");
    assert_eq!(
        application.tasks[0].deadlines_within_reach,
        ["timer.short", "timer.long"]
    );
}

/// The units a person may write are the language's.
#[test]
fn a_time_and_a_size_are_in_the_language_s_units() {
    for unit in ["s", "ms", "us", "ns"] {
        let text = a_file("s", &["t"]).replace(
            "(value 200) (unit us)",
            &format!("(value 200000000) (unit {unit})"),
        );
        assert!(read(&text).is_ok(), "{unit}");
    }
    for unit in ["byte", "KiB", "MiB"] {
        let text = a_file("s", &["t"]).replace(
            "(value 512) (unit byte)",
            &format!("(value 512) (unit {unit})"),
        );
        assert!(read(&text).is_ok(), "{unit}");
    }
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
        assert_eq!(
            application.unmet(&plan, &[]),
            Vec::<String>::new(),
            "{name}"
        );
    }
}

/// What the plan holds them against.
#[test]
fn what_is_unmet_of_a_plan_is_said() {
    let plan = contents::read(&golden("heartbeat")).expect("the golden reads");
    let tasks: Vec<&str> = plan.tasks.iter().map(|task| task.name.as_str()).collect();
    let said = |text: &str, of: &contents::Contents, words: &str| {
        let application = read(text).expect("it reads");
        assert!(
            application
                .unmet(of, &[])
                .iter()
                .any(|said| said.contains(words)),
            "{words}: {:?}",
            application.unmet(of, &[])
        );
    };
    let good = a_file(&plan.system, &tasks);
    said(&a_file("other", &tasks), &plan, "are for `other`");
    said(
        &a_file(&plan.system, &tasks[..tasks.len() - 1]),
        &plan,
        "nothing is said of the task",
    );
    let mut more = tasks.clone();
    more.push("extra");
    said(&a_file(&plan.system, &more), &plan, "no task `extra`");
    said(
        &good.replace("(target hosted-playground)", "(target riscv-virt-up)"),
        &plan,
        "of the target `riscv-virt-up`",
    );
    // A call into a node the task's lines do not name: only through a node they do. In its `node-calls`, and inside a masked run.
    said(
        &good.replace(
            &format!("(node-calls {LIST})"),
            &format!("(node-calls {LIST} (call nowhere.node 1))"),
        ),
        &plan,
        "calls `nowhere.node`",
    );
    said(
        &good
            .replace(
                &format!("(node-calls {LIST})"),
                &format!("(node-calls {LIST} (call nowhere.node 1))"),
            )
            .replace(
                "(call release 1)))",
                "(call release 1) (node-call nowhere.node 1)))",
            ),
        &plan,
        "calls `nowhere.node`",
    );
    // A node called as a primitive is said, in a job's calls and a run's, and an alarm the plan holds no node of.
    let a_node = plan.nodes[0].0.clone();
    said(
        &good.replace("(call mask 2)", &format!("(call {a_node} 2) (call mask 2)")),
        &plan,
        "as a primitive",
    );
    said(
        &good
            .replace("(call mask 2)", &format!("(call {a_node} 2) (call mask 2)"))
            .replace("(call release 1)))", &format!("(call {a_node} 1)))")),
        &plan,
        "as a primitive",
    );
    said(
        &good.replace(
            "(deadlines-within-reach)",
            "(deadlines-within-reach nowhere.alarm)",
        ),
        &plan,
        "names the alarm `nowhere.alarm`",
    );
    // What the application's initialisation calls is a node of the plan's, or it is said.
    said(
        &good.replace(
            "(basis \"it calls none\"))",
            "(basis \"it calls none\") (call nowhere.node 1))",
        ),
        &plan,
        "the application's initialisation calls `nowhere.node`",
    );
    let calls_one = read(&good.replace(
        "(basis \"it calls none\"))",
        &format!("(basis \"it sets it up\") (call {} 1))", plan.nodes[0].0),
    ))
    .expect("it reads");
    assert_eq!(calls_one.unmet(&plan, &[]), Vec::<String>::new());
    // A primitive a node is named like is the primitive it is written as: `mask` and `unmask` whatever the catalog holds, and any other the plan's catalog costs. So a
    // system with a service named `mask` has a file that is met, its node called among `node-calls`.
    let mut named_alike = plan.clone();
    let kind = named_alike.nodes[0].1;
    named_alike.nodes.push(("mask".to_owned(), kind));
    named_alike.nodes.push(("release".to_owned(), kind));
    let application = read(&good).expect("it reads");
    assert!(
        application
            .unmet(&named_alike, &["release"])
            .iter()
            .all(|said| !said.contains("as a primitive")),
        "{:?}",
        application.unmet(&named_alike, &["release"])
    );
    assert!(
        application
            .unmet(&named_alike, &[])
            .iter()
            .any(|said| said.contains("calls `release` as a primitive"))
            && application
                .unmet(&named_alike, &[])
                .iter()
                .all(|said| !said.contains("calls `mask` as a primitive")),
        "a name no catalog costs is a node's where the plan has that node: {:?}",
        application.unmet(&named_alike, &[])
    );
    // A task's own `bind` line names an instance, whose text it calls by.
    let adapter = "(adapter epoch-extender timer.short)";
    let bound = good.replace(
        &format!("(node-calls {LIST})"),
        &format!("(node-calls {LIST} (call {adapter} 1))"),
    );
    let mut with = plan.clone();
    for task in with.tasks.clone() {
        with.binds.push(contents::Bind {
            requester: task.identity.clone(),
            fact: "tick-rate".to_owned(),
            provider: adapter.to_owned(),
        });
    }
    let application = read(&bound).expect("it reads");
    assert_eq!(application.unmet(&with, &[]), Vec::<String>::new());
    // A fact a plan owes that this format does not hold is said, so a new one cannot pass unread, whichever subject it is of.
    let mut newer = plan.clone();
    newer.owed.push(contents::Owed {
        subject: format!("(task {} {})", plan.system, tasks[0]),
        fact: "something-new".to_owned(),
        by: "application".to_owned(),
    });
    said(&good, &newer, "something-new");
    let mut of_a_node = plan.clone();
    of_a_node.owed.push(contents::Owed {
        subject: "console.uart".to_owned(),
        fact: "stack-bound".to_owned(),
        by: "application".to_owned(),
    });
    said(&good, &of_a_node, "`stack-bound` of console.uart");
    // What `/2` owes of a task writing an alarm it does not name is held, `RA-H7`'s.
    let mut within_reach = plan.clone();
    within_reach.owed.push(contents::Owed {
        subject: format!("(task {} {})", plan.system, tasks[0]),
        fact: "deadlines-within-reach".to_owned(),
        by: "application".to_owned(),
    });
    assert_eq!(
        read(&good).expect("it reads").unmet(&within_reach, &[]),
        Vec::<String>::new()
    );
    // What `/2` owes of the system where no file is given, its `initialisation`, is held: a file names one or says `none` (round 5, AI5-5).
    let mut of_the_system = plan.clone();
    of_the_system.owed.push(contents::Owed {
        subject: plan.system.clone(),
        fact: "initialisation".to_owned(),
        by: "application".to_owned(),
    });
    assert_eq!(
        read(&good).expect("it reads").unmet(&of_the_system, &[]),
        Vec::<String>::new()
    );
}

/// What a file quotes is the composite-inputs record's own words, read from there with what stands on each side: a change to them, or a sentence added beside them, moves this, and a file declaring the old ones stops reading. The book and this format's record hold them whole, where a person is sent to read them.
#[test]
fn the_clause_a_file_quotes_is_the_record_s_own_words() {
    let plain = |text: &str| {
        text.replace("**", "")
            .split_whitespace()
            .collect::<Vec<_>>()
            .join(" ")
    };
    let record = plain(include_str!(
        "../../../docs/specs/catalog/decision_runtime-composite-inputs.md"
    ));
    let book = plain(include_str!("../../../docs/book/src/runtime-plan.md"));
    let mine = plain(include_str!(
        "../../../docs/decisions/decision_application-inputs.md"
    ));
    for (before, part, after) in [
        (
            "for a service's application code | ",
            CLAUSE_PARTS[0],
            " The paragraphs below add to it |",
        ),
        (
            "as assumptions | ",
            CLAUSE_PARTS[1],
            " (the role list below): `leaves-interrupt-hardware-alone` holds this too",
        ),
        (
            "R14 231, R15 239). ",
            CLAUSE_PARTS[2],
            " That fact holds this too (added `2026-11-04`, `M2.30`).",
        ),
    ] {
        assert!(
            record.contains(&format!("{before}{part}{after}")),
            "{before}|{part}|{after}"
        );
        assert!(book.contains(part), "the book lacks {part}");
        assert!(mine.contains(part), "the record lacks {part}");
    }
}

/// What a yes or a no cannot carry: the fact rests on each stack bound the clause speaks of, by its category, since an overrun of one is a write it forbids: the application's initialisation's, and each task's.
#[test]
fn the_fact_says_what_it_rests_on() {
    let application = read(&a_file("s", &["one", "two"])).expect("it reads");
    assert_eq!(
        application.leaves_alone_rests_on(),
        [
            ("the application's initialisation", Category::Assumed),
            ("one", Category::Assumed),
            ("two", Category::Assumed)
        ]
    );
    let none = read(&a_file("s", &["one"]).replace(INITIALISATION, "(initialisation none)"))
        .expect("it reads");
    assert_eq!(none.leaves_alone_rests_on(), [("one", Category::Assumed)]);
}

/// A file this size reads in a time a person would not notice, so a list's cost is in step with its length.
#[test]
fn a_long_list_reads_in_step_with_its_length() {
    let many: String = (0..80_000)
        .map(|call| format!("(call p{call} 1) "))
        .collect();
    let text = a_file("s", &["t"]).replace("(call mask 2)", &format!("(call mask 2) {many}"));
    let application = read(&text).expect("it reads");
    assert_eq!(application.tasks[0].os_calls.calls.len(), 80_003);
    // A run holding as many callees as its job is held to it through a map of the job's: a search of the job's list for each would be their product.
    let text = text.replace("(call release 1)))", &format!("(call release 1) {many}))"));
    let application = read(&text).expect("it reads");
    assert_eq!(application.tasks[0].masked_runs.runs[0].calls.len(), 80_001);
    let names: String = (0..200_000).map(|name| format!("a{name} ")).collect();
    let text = a_file("s", &["t"]).replace(
        "(deadlines-within-reach)",
        &format!("(deadlines-within-reach {names})"),
    );
    let application = read(&text).expect("it reads");
    assert_eq!(application.tasks[0].deadlines_within_reach.len(), 200_000);
}
