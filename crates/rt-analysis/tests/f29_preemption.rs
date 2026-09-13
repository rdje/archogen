//! **F29** — the repeated-preemption and cost-accounting fixture of `ROADMAP.md` §13.4.
//!
//! §13.4 asks for the fixture to be checked two ways, and this file is both:
//!
//! > M2 implements the repeated-preemption fixture in §13.4 through both **an independently
//! > specified event timeline** and **the engine's accounting path**.
//!
//! | Source | What it is here |
//! |---|---|
//! | the independently specified timeline | §13.4's **Expected trace** table, parsed out of `ROADMAP.md` |
//! | the engine's accounting path | `rt_analysis::trace::simulate`, which computes the trace from the operational model |
//!
//! The two are compared interval by interval. Neither is derived from the other: the roadmap's
//! table was written before this crate existed, and the simulator is written from the
//! operational rules in the prose above the table, not from the table itself.
//!
//! ⛔ **The controls re-simulate.** §13.4: *"Re-simulate mutations that change execution timing:
//! they can change the number of interfering releases, so subtracting a fixed number from the
//! original response is not generally valid."* Zeroing the interrupt cost does not move `L`'s
//! completion by two units — it changes when the second `H` release is serviced, and 21 has to be
//! computed. Only the duplicate-cost control works on the original fixed trace, and §13.4 says so.

use std::path::{Path, PathBuf};

use rt_analysis::cost::{Accounting, Category, Interval, Ledger, LedgerError};
use rt_analysis::trace::{simulate, Costs, Workload};

/// One row of §13.4's expected trace.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Row {
    start: u64,
    end: u64,
    activity: String,
    duration: u64,
    category: String,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Parse §13.4's **Expected trace** table out of `ROADMAP.md`.
///
/// The same device F18 uses on §13.2, and for the same reason: §14.1 forbids implementation
/// changes that "silently … adjust expected oracle results", and a test holding its own copy of
/// the timeline makes exactly that a one-line edit that looks like a fix.
fn specified_timeline() -> Vec<Row> {
    let text = std::fs::read_to_string(repo_root().join("ROADMAP.md")).expect("the roadmap");
    let section = text
        .split_once("**Expected trace:**")
        .expect("§13.4 publishes an expected trace")
        .1;
    let section = section
        .split_once("\nThe ledger is")
        .map_or(section, |(head, _)| head);

    let rows: Vec<Row> = section
        .lines()
        .filter_map(|line| {
            let cells: Vec<&str> = line
                .trim()
                .strip_prefix('|')?
                .strip_suffix('|')?
                .split('|')
                .map(str::trim)
                .collect();
            if cells.len() != 4 {
                return None;
            }
            let (start, end) = cells[0]
                .strip_prefix('[')?
                .strip_suffix(')')?
                .split_once(", ")?;
            Some(Row {
                start: start.parse().ok()?,
                end: end.parse().ok()?,
                activity: cells[1].to_string(),
                duration: cells[2].parse().ok()?,
                category: cells[3].to_string(),
            })
        })
        .collect();

    assert_eq!(
        rows.len(),
        12,
        "§13.4 publishes twelve trace intervals; the parser found {} — the table changed shape, \
         and a timeline that quietly shrank would still be green",
        rows.len()
    );
    rows
}

/// The category names §13.4 uses, mapped onto the ledger's.
fn category_of(row: &Row, workload: &Workload) -> Category {
    match row.category.as_str() {
        "Initial dispatch" => Category::InitialDispatch,
        "Interrupt service" => Category::InterruptService,
        "Task switch" => Category::TaskSwitch,
        "L useful execution" => Category::TaskExecution(workload.low.clone()),
        "H useful execution" => Category::TaskExecution(workload.high.clone()),
        other => panic!("§13.4 uses a ledger category this test does not map: `{other}`"),
    }
}

/// §13.4's table, sealed as a ledger — the independently specified side.
fn specified_ledger(workload: &Workload) -> Ledger {
    let intervals: Vec<Interval> = specified_timeline()
        .iter()
        .map(|row| Interval {
            start: row.start,
            end: row.end,
            category: category_of(row, workload),
            activity: row.activity.clone(),
        })
        .collect();
    Ledger::seal(intervals, Accounting::ExactTrace)
        .expect("§13.4's published trace is disjoint and gapless")
}

#[test]
fn f29_the_published_timeline_is_internally_consistent() {
    // Before comparing anything to it, the oracle is checked against itself: each row's stated
    // duration must equal its interval, and the whole must cover [0, 23) with no overlap or hole.
    // §13.4 is a specification, and a specification can contain a typo.
    let rows = specified_timeline();
    for row in &rows {
        assert_eq!(
            row.end - row.start,
            row.duration,
            "§13.4 row `{}` states duration {} over [{}, {})",
            row.activity,
            row.duration,
            row.start,
            row.end
        );
    }
    let ledger = specified_ledger(&Workload::specified());
    assert_eq!(ledger.span(), (0, 23));
    assert_eq!(ledger.total(), 23);
}

#[test]
fn f29_the_ledger_totals_twenty_three_with_every_unit_covered_exactly_once() {
    // §13.4: "The ledger is 8 (L) + 4 (H) + 1 (initial dispatch) + 2 (ISRs) + 8 (four switches)
    // = 23." Each addend is checked, not only the sum: a total that is right for two offsetting
    // reasons is the failure this fixture exists to catch.
    let workload = Workload::specified();
    let simulation = simulate(&workload, Costs::SPECIFIED).expect("the specified model");
    let by_category = simulation.ledger.by_category();

    assert_eq!(
        by_category[&Category::TaskExecution(workload.low.clone())],
        8
    );
    assert_eq!(
        by_category[&Category::TaskExecution(workload.high.clone())],
        4
    );
    assert_eq!(by_category[&Category::InitialDispatch], 1);
    assert_eq!(by_category[&Category::InterruptService], 2);
    assert_eq!(by_category[&Category::TaskSwitch], 8);
    assert_eq!(simulation.ledger.total(), 23);
    assert_eq!(
        simulation.ledger.span(),
        (0, 23),
        "all 23 units, exactly once"
    );
}

#[test]
fn f29_the_engines_accounting_path_reproduces_the_specified_timeline_interval_by_interval() {
    // ⭐ The comparison §13.4 asks for. Neither side is derived from the other: the roadmap's
    // table predates this crate, and the simulator is written from the operational rules in the
    // prose, not from the table.
    let workload = Workload::specified();
    let simulated = simulate(&workload, Costs::SPECIFIED).expect("the specified model");
    let specified = specified_ledger(&workload);

    assert_eq!(
        simulated.ledger.intervals().len(),
        specified.intervals().len(),
        "simulated:\n{}\nspecified:\n{}",
        simulated.ledger.render(),
        specified.render()
    );
    for (got, want) in simulated
        .ledger
        .intervals()
        .iter()
        .zip(specified.intervals())
    {
        assert_eq!(
            (got.start, got.end, &got.category),
            (want.start, want.end, &want.category),
            "the accounting path and §13.4's timeline disagree.\nsimulated:\n{}\nspecified:\n{}",
            simulated.ledger.render(),
            specified.render()
        );
    }
}

#[test]
fn f29_the_completion_times_are_the_ones_the_roadmap_states() {
    // §13.4: "The H jobs finish at 9 and 19, each five units after nominal release. L finishes at
    // 23 and misses its deadline of 22 by one unit. With its deadline changed to 23, that
    // particular job meets the deadline exactly."
    let workload = Workload::specified();
    let simulation = simulate(&workload, Costs::SPECIFIED).expect("the specified model");

    assert_eq!(simulation.high_completions(&workload), vec![9, 19]);
    for completion in simulation
        .completions
        .iter()
        .filter(|c| c.task == workload.high)
    {
        assert_eq!(
            completion.response(),
            5,
            "H's response is measured from its NOMINAL release, so the ISR delay is inside it"
        );
    }

    let low = simulation.low_completion(&workload);
    assert_eq!(low, 23);
    assert!(low > 22, "the deadline-22 claim must never be established");
    assert!(low <= 23, "with a deadline of 23 the job meets it exactly");
}

#[test]
fn f29_control_omitting_the_timer_isr_cost_moves_the_completion_to_twenty_one() {
    // §13.4's second control: "Omit the timer ISR cost and re-simulate the altered model →
    // Incorrect L completion at 21, producing a false pass at deadline 22."
    //
    // ⛔ 23 − 2 = 21 is the right answer for the wrong reason. The two ISR units are not simply
    // removed: the whole trace shifts, the second H release is serviced earlier, and 21 is what
    // the altered model computes. A control implemented by subtraction would agree here and
    // diverge on the next fixture, which is exactly what §13.4 warns about.
    let workload = Workload::specified();
    let altered = Costs {
        interrupt_service: 0,
        ..Costs::SPECIFIED
    };
    let simulation = simulate(&workload, altered).expect("the altered model still closes");
    let low = simulation.low_completion(&workload);

    assert_eq!(low, 21);
    assert!(
        low <= 22,
        "this is the false pass the control exists to expose: an omitted interrupt category turns \
         a real miss at deadline 22 into a pass"
    );
    assert_eq!(
        simulation
            .ledger
            .by_category()
            .get(&Category::InterruptService),
        None,
        "the missing category is itself the detection: a ledger with no interrupt service in a \
         model that has interrupts is an omission, not a zero"
    );
    assert_eq!(
        simulation.ledger.total(),
        21,
        "two units are unaccounted for"
    );
}

#[test]
fn f29_control_omitting_the_resume_switch_moves_the_completion_to_fourteen() {
    // §13.4's third control: "Omit H-to-L switch costs and re-simulate → Incorrect L completion
    // at 14; L completes before another modeled H release is serviced."
    //
    // ⭐ This is the one subtraction could never reach. Losing four units of resume cost does not
    // give 19: it lets L finish at exactly 14, the instant of the second nominal release, and
    // §13.4's rule "record completion before processing the new release" then removes that
    // release's interference entirely. One interfering job disappears.
    let workload = Workload::specified();
    let altered = Costs {
        resume_switch: 0,
        ..Costs::SPECIFIED
    };
    let simulation = simulate(&workload, altered).expect("the altered model still closes");

    assert_eq!(simulation.low_completion(&workload), 14);
    assert_eq!(
        simulation.high_completions(&workload),
        vec![9],
        "the second H release is never serviced — one interfering job vanished, which is why \
         subtracting a fixed number from the original response is not valid"
    );
    assert_eq!(
        simulation.ledger.by_category()[&Category::TaskSwitch],
        2,
        "only the preempting switch is charged; the resume transitions are gone"
    );
}

#[test]
fn f29_control_charging_the_isr_intervals_twice_cannot_seal_the_original_trace() {
    // §13.4's fourth control: "Charge the two ISR intervals again inside task cost on the
    // original fixed trace → Incorrect exact ledger total 25 → Duplicate interval ownership."
    //
    // ⭐ Note what this control does NOT do: re-simulate. §13.4 is explicit that it "intentionally
    // checks the original fixed trace instead of asserting a new schedule" — the mistake being
    // modelled is an accounting one, not a timing one.
    let workload = Workload::specified();
    let mut intervals: Vec<Interval> = specified_ledger(&workload).intervals().to_vec();

    // The mistake: the two ISR intervals are charged a second time, inside L's task cost.
    let duplicated: Vec<Interval> = intervals
        .iter()
        .filter(|interval| interval.category == Category::InterruptService)
        .map(|interval| Interval {
            category: Category::TaskExecution(workload.low.clone()),
            activity: format!("{} cost including the ISR", workload.low),
            ..interval.clone()
        })
        .collect();
    assert_eq!(duplicated.len(), 2, "§13.4's trace has two ISR intervals");
    let would_be_total: u64 = intervals
        .iter()
        .chain(&duplicated)
        .map(rt_analysis::cost::Interval::duration)
        .sum();
    assert_eq!(
        would_be_total, 25,
        "§13.4 states the incorrect exact ledger total is 25"
    );

    intervals.extend(duplicated);
    let error = Ledger::seal(intervals, Accounting::ExactTrace)
        .expect_err("an interval charged twice cannot be an exact trace");
    let LedgerError::Overlap {
        held_by,
        claimed_by,
        ..
    } = error
    else {
        panic!("expected duplicate interval ownership, got {error:?}");
    };
    // Either order, depending on which sorts first at the shared instant — what matters is that
    // the two categories claiming it are named.
    let named = format!("{held_by}/{claimed_by}");
    assert!(named.contains("interrupt service"), "{named}");
    assert!(named.contains("useful execution"), "{named}");
}

#[test]
fn f29_a_sound_bound_for_this_scenario_can_never_be_below_twenty_three() {
    // §13.4: "For the admitted model, a sound response bound that covers this scenario cannot be
    // below 23. A general conservative analyzer may return a higher bound and `not-established`
    // … It must never establish the deadline-22 claim."
    //
    // The exact trace is the floor. Anything an analyzer returns below it is unsound, and the
    // deadline-22 claim is unreachable from any sound bound.
    let workload = Workload::specified();
    let exact = simulate(&workload, Costs::SPECIFIED)
        .expect("the specified model")
        .low_completion(&workload);
    assert_eq!(exact, 23);
    for candidate_bound in [23_u64, 24, 30] {
        assert!(
            candidate_bound >= exact,
            "a bound below the exact trace is unsound"
        );
        assert!(
            candidate_bound > 22,
            "no sound bound establishes the deadline-22 claim"
        );
    }
}
