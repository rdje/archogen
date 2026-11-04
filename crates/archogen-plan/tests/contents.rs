//! `archogen-runtime-plan/1` read back (leaf `M4.1.4.2`, `docs/decisions/decision_runtime-plan.md` §3): what a consumer of a
//! plan reads, typed, from the one text; every plan of the corpus reads back to itself; and a text the writer would not
//! write is refused, naming its line, rather than normalised.

mod common;

use std::fs;

use archogen_plan::contents::{Context, Item, NodeKind, Release, Separation, Trigger};
use archogen_plan::Plan;
use common::{lowered, root, Lowered};
use eadl_model::rational::Rational;

/// `name`'s plan for its only system, a corpus fixture the lowering lowers.
fn plan_of(name: &str) -> String {
    let path = root().join(format!("crates/archogen-plan/tests/corpus/{name}.eadl"));
    let text = fs::read_to_string(path).expect("the fixture");
    match lowered(&text, None) {
        Lowered::Plan(plan) => plan,
        _ => panic!("{name} lowers"),
    }
}

fn seconds(numerator: i128, denominator: i128) -> Rational {
    Rational::new(numerator, denominator).expect("a value")
}

#[test]
fn a_plan_s_contents_are_what_its_text_says() {
    // Two tasks, a source, a timer route, an adapter's instance and a cost on the route.
    let text = plan_of("both-kinds-adapter");
    let plan = Plan::read(&text).expect("reads");
    let contents = plan.contents();
    assert_eq!(contents.profile, "rt-static-up-v1");
    assert_eq!(contents.system, "bx");
    assert_eq!(contents.adapters, "adapters/1");
    assert!(!contents.complete);

    let [s, p] = contents.tasks.as_slice() else {
        panic!("two tasks: {:?}", contents.tasks);
    };
    assert_eq!((s.index, s.name.as_str(), s.rank), (0, "s", 1));
    assert_eq!(s.identity, "(task bx s)");
    assert_eq!(s.release, Release::MinSeparation(seconds(1, 50)));
    assert_eq!(s.deadline, seconds(1, 50));
    assert!(s.deadline_from_release);
    assert_eq!(s.jitter, seconds(0, 1));
    assert_eq!(s.overrun, "fault");
    assert_eq!(s.released_by, Trigger::Source("dev.x".to_owned()));
    assert_eq!(s.uses, ["evt"]);
    assert_eq!(p.release, Release::Period(seconds(1, 100)));
    assert_eq!(p.released_by, Trigger::Timer);

    assert_eq!(contents.timer_releases, ["(task bx p)"]);
    assert_eq!(
        contents.enabled,
        [
            Item::Timer,
            Item::Source("dev.x".to_owned()),
            Item::TimerRoute("dev.x".to_owned())
        ]
    );
    let [source, timer] = contents.routes.as_slice() else {
        panic!("two routes: {:?}", contents.routes);
    };
    assert_eq!(source.item, Item::Source("dev.x".to_owned()));
    assert_eq!(source.separation, Separation::Of(seconds(1, 50)));
    assert_eq!(source.level, "machine");
    assert_eq!(source.requesters, ["evt"]);
    assert_eq!(timer.separation, Separation::Programmed);
    assert_eq!(
        timer.costs,
        [(
            "(adapter epoch-extender dev.x)".to_owned(),
            "wake".to_owned()
        )]
    );
    assert!(contents.nodes.contains(&(
        "(adapter epoch-extender dev.x)".to_owned(),
        NodeKind::Instance
    )));
    assert!(contents
        .nodes
        .contains(&("dev.x".to_owned(), NodeKind::Block)));
    assert!(contents.contexts.contains(&(
        "evt".to_owned(),
        vec![
            Context::Job,
            Context::Interrupt(Item::Source("dev.x".to_owned())),
            Context::Initialisation
        ]
    )));
    assert_eq!(contents.instances[0].costs, ["wake"]);
    assert_eq!(contents.instances[0].obligations, ["update", "coherent"]);
    assert_eq!(contents.steps[0], ["dev.x"]);
    assert!(contents
        .owed
        .iter()
        .any(|o| o.subject == "(task bx p)" && o.fact == "entry-point" && o.by == "application"));
}

#[test]
fn every_plan_of_the_corpus_reads_back_to_its_own_text() {
    let dir = root().join("crates/archogen-plan/tests/corpus");
    let mut read = 0;
    for entry in fs::read_dir(dir).expect("the corpus").flatten() {
        let path = entry.path();
        if path.extension().is_none_or(|e| e != "eadl") {
            continue;
        }
        let text = fs::read_to_string(&path).expect("readable");
        let systems = text.matches("(defsystem").count();
        if systems != 1 {
            continue;
        }
        if let Lowered::Plan(plan) = lowered(&text, None) {
            let again = Plan::read(&plan).expect("reads");
            assert_eq!(again.text(), plan, "{}", path.display());
            read += 1;
        }
    }
    assert!(read > 50, "the corpus reached {read} plans");
}

#[test]
fn the_plan_s_hash_is_its_text_s() {
    let text = plan_of("both-kinds-adapter");
    let plan = Plan::read(&text).expect("reads");
    assert_eq!(plan.sha256(), archogen_digest::Digest::of(text.as_bytes()));
}

#[test]
fn a_text_the_writer_would_not_write_is_refused_naming_its_line() {
    let text = plan_of("both-kinds-adapter");
    let swap = |old: &str, new: &str| {
        assert!(text.contains(old), "the fixture holds `{old}`");
        text.replacen(old, new, 1)
    };
    let without_line = |head: &str| -> String {
        text.lines()
            .filter(|line| !line.starts_with(head))
            .map(|line| format!("{line}\n"))
            .collect()
    };
    for (case, wrong, says) in [
        ("no last line end", text.trim_end().to_owned(), "newline"),
        (
            "a blank line",
            swap("(system bx)\n", "(system bx)\n\n"),
            "a form was expected",
        ),
        (
            "two spaces",
            swap("(system bx)", "(system  bx)"),
            "not written as",
        ),
        (
            "a space after the paren",
            swap("(system bx)", "( system bx)"),
            "not written as",
        ),
        (
            "text after the form",
            swap("(system bx)", "(system bx) x"),
            "text after the form",
        ),
        (
            "an unclosed list",
            swap("(system bx)", "(system bx"),
            "not closed",
        ),
        (
            "another format",
            swap("archogen-runtime-plan/1", "archogen-runtime-plan/2"),
            "not `archogen-runtime-plan/1`",
        ),
        (
            "a digest not of its form",
            swap("(program \"", "(program \"x"),
            "holds no digest",
        ),
        (
            "a form it does not know",
            swap("(system bx)", "(host bx)"),
            "no form of the plan",
        ),
        (
            "out of order",
            swap(
                "(profile rt-static-up-v1)\n(system bx)",
                "(system bx)\n(profile rt-static-up-v1)",
            ),
            "stands after",
        ),
        (
            "a form written once, twice",
            swap("(system bx)\n", "(system bx)\n(system bx)\n"),
            "is written once",
        ),
        (
            "a form missing",
            without_line("(adapters"),
            "a plan writes `adapters`",
        ),
        ("no task", without_line("(task "), "at least one task"),
        (
            "a leading zero",
            swap("(rank 1)", "(rank 01)"),
            "as §3 writes an integer",
        ),
        (
            "a duration not reduced",
            swap("(period 1/100 s)", "(period 2/200 s)"),
            "not a reduced duration",
        ),
        (
            "a duration of another unit",
            swap("(period 1/100 s)", "(period 1/100 ms)"),
            "no duration's unit",
        ),
        (
            "a release of zero",
            swap("(period 1/100 s)", "(period 0 s)"),
            "never zero",
        ),
        (
            "a source programmed",
            swap(
                "(route 1 (source dev.x) (separation 1/50 s)",
                "(route 1 (source dev.x) (separation programmed)",
            ),
            "never programmed",
        ),
        (
            "the timer a route",
            swap("(route 1 (source dev.x)", "(route 1 timer"),
            "no route",
        ),
        (
            "a trigger it does not know",
            swap("(released-by timer)", "(released-by clock)"),
            "no trigger",
        ),
        (
            "a node's kind it does not know",
            swap("(node dev.x block)", "(node dev.x device)"),
            "no node's kind",
        ),
        (
            "a context it does not know",
            swap("(context (task bx p) job)", "(context (task bx p) idle)"),
            "expected `(interrupt",
        ),
        (
            "a step out of order",
            swap("(step 1 dev.x)", "(step 2 dev.x)"),
            "out of order",
        ),
        (
            "complete neither",
            swap("(complete no)", "(complete maybe)"),
            "not `yes` or `no`",
        ),
        (
            "another interrupt model",
            swap("(nesting no)", "(nesting yes)"),
            "no interrupt model",
        ),
    ] {
        let why = Plan::read(&wrong).expect_err(case);
        assert!(why.contains(says), "{case}: {why}");
    }
    // A line's number is what the reader names.
    let why = Plan::read(&swap("(system bx)", "(host bx)")).expect_err("refused");
    assert!(why.starts_with("line 3: "), "{why}");
}
