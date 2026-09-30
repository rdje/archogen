//! The checks at each ledgering commit, and retired ids (§5, §9, §11; `M2.7.3.4.2`).

mod common;

use archogen_catalog::history::{CommitterDate, History};
use archogen_catalog::ledger::local_date;
use archogen_catalog::replay::{check_history, replay};
use archogen_catalog::Code;
use common::*;

#[test]
fn a_committer_date_is_read_in_its_offset() {
    let at = |seconds, offset_minutes| {
        local_date(CommitterDate {
            seconds,
            offset_minutes,
        })
    };
    assert_eq!(at(0, 0), (1970, 1, 1));
    assert_eq!(at(1_790_812_800, 0), (2026, 10, 1));
    assert_eq!(at(951_782_400, 0), (2000, 2, 29), "a leap day");
    assert_eq!(
        at(1_790_722_800, 120),
        (2026, 9, 30),
        "23:00 UTC is the next day two hours east"
    );
    assert_eq!(
        at(1_790_730_000, -120),
        (2026, 9, 29),
        "01:00 UTC is the day before two hours west"
    );
    assert_eq!(at(-1, 0), (1969, 12, 31));
}

#[test]
fn a_review_is_dated_no_later_than_the_commit_that_ledgers_it() {
    let t0 = tree(&[&bare()], &[]);
    let reviewed = with(
        &bare(),
        &review(
            BEHAVIOR,
            &bound(&t0, "example.base", BEHAVIOR),
            "production",
            &[],
        ),
    );
    for (seconds, offset, admitted, why) in [
        (
            1_790_722_800,
            120,
            true,
            "east of UTC, already the review's day",
        ),
        (1_790_722_800, 0, false, "the day before, in UTC"),
        (
            1_790_730_000,
            -120,
            false,
            "west of UTC, still the day before",
        ),
        (1_790_730_000, 0, true, "the review's day"),
    ] {
        let date = CommitterDate {
            seconds,
            offset_minutes: offset,
        };
        let mut h = History::default();
        add_on(&mut h, &n('1'), &[], tree(&[&reviewed], &[]), None, date);
        let replayed = replay(&h, &[], &n('1'));
        let loaded = check_history(&h, &n('1'));
        if admitted {
            replayed.unwrap_or_else(|e| panic!("{why}: {e}"));
            loaded.unwrap_or_else(|e| panic!("{why}: {e}"));
        } else {
            for r in [replayed.unwrap_err(), loaded.unwrap_err()] {
                assert_eq!(r.code, Code::Review, "{why}: {r}");
                assert!(
                    r.message.contains("later than the commit's committer date"),
                    "{why}: {r}"
                );
            }
        }
    }
}

#[test]
fn answers_name_only_rejections_the_facet_inherits() {
    let t0 = tree(&[&bare()], &[]);
    let h_timing = bound(&t0, "example.base", TIMING);
    let rejected = with(&bare(), &review(TIMING, &h_timing, "rejected", &[]));
    let x = ledger(&rejected);
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &[]), None);
    let zero = format!("sha256:{}", "0".repeat(64));
    for (digit, answers, why) in [
        ('2', vec![zero.as_str()], "no rejection at all"),
        ('3', vec![x.as_str(), zero.as_str()], "one of two"),
    ] {
        let answering = with(
            &rejected,
            &review(TIMING, &h_timing, "production", &answers),
        );
        add(
            &mut h,
            &n(digit),
            &[&n('1')],
            tree(&[&answering], &[]),
            None,
        );
        let r = replay(&h, &[&n('1')], &n(digit)).unwrap_err();
        assert_eq!(r.code, Code::Review, "{why}: {r}");
        assert!(
            r.message.contains("not a rejection its facet inherits"),
            "{why}: {r}"
        );
    }
    let answering = with(&rejected, &review(TIMING, &h_timing, "production", &[&x]));
    add(&mut h, &n('4'), &[&n('1')], tree(&[&answering], &[]), None);
    replay(&h, &[&n('1')], &n('4')).unwrap_or_else(|e| panic!("{e}"));
    check_history(&h, &n('4')).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn an_own_rejection_is_answered_only_once_a_parent_holds_it() {
    let t0 = tree(&[&bare()], &[]);
    let h_timing = bound(&t0, "example.base", TIMING);
    let rejected = with(&bare(), &review(TIMING, &h_timing, "rejected", &[]));
    let x = ledger(&rejected);
    let both = with(&rejected, &review(TIMING, &h_timing, "production", &[&x]));
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&both], &[]), None);
    for r in [
        replay(&h, &[], &n('1')).unwrap_err(),
        check_history(&h, &n('1')).unwrap_err(),
    ] {
        assert_eq!(r.code, Code::Review, "{r}");
        assert!(r.message.contains("its own record's"), "{r}");
    }
}

#[test]
fn an_items_only_rejection_may_be_answered_in_the_bless_that_ledgers_it() {
    // `example.other` shares every item of `example.base`'s behavioral model, and answers its rejection at once.
    let t0 = tree(&[&bare(), &copy("example.other")], &[]);
    let rejected = with(
        &bare(),
        &review(
            BEHAVIOR,
            &bound(&t0, "example.base", BEHAVIOR),
            "rejected",
            &[],
        ),
    );
    let x = ledger(&rejected);
    let other = with(
        &copy("example.other"),
        &review(
            BEHAVIOR,
            &bound(&t0, "example.other", BEHAVIOR),
            "production",
            &[&x],
        ),
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected, &other], &[]), None);
    replay(&h, &[], &n('1')).unwrap_or_else(|e| panic!("{e}"));
    check_history(&h, &n('1')).unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn a_lineage_rejection_is_answered_only_once_a_parent_holds_it() {
    // Both present at once, so `supersedes` is refused too; the answer is refused first, where it is ledgered.
    let t0 = tree(&[&bare()], &[]);
    let rejected = with(
        &bare(),
        &review(TIMING, &bound(&t0, "example.base", TIMING), "rejected", &[]),
    );
    let x = ledger(&rejected);
    let next = edit(
        &edit(
            &copy("example.next"),
            "(supersedes)",
            "(supersedes example.base)",
        ),
        "(none \"the costs of a machine are its devices'\")",
        "(none \"a machine's costs are its devices'\")",
    );
    let t1 = tree(&[&rejected, &next], &[]);
    let answering = with(
        &next,
        &review(
            TIMING,
            &bound(&t1, "example.next", TIMING),
            "production",
            &[&x],
        ),
    );
    let mut h = History::default();
    add(
        &mut h,
        &n('1'),
        &[],
        tree(&[&rejected, &answering], &[]),
        None,
    );
    let r = check_history(&h, &n('1')).unwrap_err();
    assert_eq!(r.code, Code::Review, "{r}");
    assert!(r.message.contains("its lineage's"), "{r}");
}

#[test]
fn supersedes_names_a_retired_id_with_ledger_lines() {
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&bare()], &[]), None);
    let next = edit(
        &copy("example.next"),
        "(supersedes)",
        "(supersedes example.base)",
    );
    add(
        &mut h,
        &n('2'),
        &[&n('1')],
        tree(&[&bare(), &next], &[]),
        None,
    );
    let r = replay(&h, &[&n('1')], &n('2')).unwrap_err();
    assert_eq!(r.code, Code::Dependency, "a present record: {r}");
    let never = edit(
        &copy("example.next"),
        "(supersedes)",
        "(supersedes example.never)",
    );
    add(
        &mut h,
        &n('3'),
        &[&n('1')],
        tree(&[&bare(), &never], &[]),
        None,
    );
    let r = replay(&h, &[&n('1')], &n('3')).unwrap_err();
    assert_eq!(r.code, Code::Dependency, "no ledger line: {r}");
    assert!(r.message.contains("no ledger line"), "{r}");
}

#[test]
fn a_retired_ids_rejection_is_not_shed() {
    let t0 = tree(&[&bare()], &[]);
    let rejected = with(
        &bare(),
        &review(TIMING, &bound(&t0, "example.base", TIMING), "rejected", &[]),
    );
    let x = ledger(&rejected);
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &[]), None);
    // Deleted with nothing superseding it: refused.
    add(&mut h, &n('2'), &[&n('1')], tree(&[], &[]), None);
    let r = replay(&h, &[&n('1')], &n('2')).unwrap_err();
    assert_eq!(r.code, Code::LockRetired, "{r}");
    assert_eq!(
        check_history(&h, &n('2')).unwrap_err().code,
        Code::LockRetired
    );
    // Superseded by a present record, which the rejection binds: admitted.
    let next = edit(
        &edit(
            &copy("example.next"),
            "(supersedes)",
            "(supersedes example.base)",
        ),
        "(none \"the costs of a machine are its devices'\")",
        "(none \"a machine's costs are its devices'\")",
    );
    add(&mut h, &n('3'), &[&n('1')], tree(&[&next], &[]), None);
    replay(&h, &[&n('1')], &n('3')).unwrap_or_else(|e| panic!("{e}"));
    check_history(&h, &n('3')).unwrap_or_else(|e| panic!("{e}"));
    // Its successor answers it, and then retires too: admitted, since a production review of a record superseding
    // it answered it.
    let t3 = tree(&[&next], &[]);
    let answered = with(
        &next,
        &review(
            TIMING,
            &bound(&t3, "example.next", TIMING),
            "production",
            &[&x],
        ),
    );
    add(&mut h, &n('4'), &[&n('3')], tree(&[&answered], &[]), None);
    replay(&h, &[&n('3')], &n('4')).unwrap_or_else(|e| panic!("{e}"));
    let last = edit(
        &edit(
            &copy("example.last"),
            "(supersedes)",
            "(supersedes example.next)",
        ),
        "(none \"the costs of a machine are its devices'\")",
        "(none \"the last machine's costs are its devices'\")",
    );
    add(&mut h, &n('5'), &[&n('4')], tree(&[&last], &[]), None);
    replay(&h, &[&n('4')], &n('5')).unwrap_or_else(|e| panic!("{e}"));
    add(&mut h, &n('6'), &[&n('5')], tree(&[], &[]), None);
    replay(&h, &[&n('5')], &n('6')).unwrap_or_else(|e| panic!("answered by a successor: {e}"));
    // Without the answer, the same retirement is refused.
    add(&mut h, &n('7'), &[&n('3')], tree(&[], &[]), None);
    assert_eq!(
        replay(&h, &[&n('3')], &n('7')).unwrap_err().code,
        Code::LockRetired
    );
}

#[test]
fn a_superseded_id_is_not_taken_again_and_a_lineage_is_not_dropped() {
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&bare()], &[]), None);
    let next = edit(
        &copy("example.next"),
        "(supersedes)",
        "(supersedes example.base)",
    );
    add(&mut h, &n('2'), &[&n('1')], tree(&[&next], &[]), None);
    replay(&h, &[&n('1')], &n('2')).unwrap_or_else(|e| panic!("{e}"));
    // The superseded id comes back.
    add(
        &mut h,
        &n('3'),
        &[&n('2')],
        tree(&[&next, &bare()], &[]),
        None,
    );
    let r = replay(&h, &[&n('2')], &n('3')).unwrap_err();
    assert_eq!(r.code, Code::Dependency, "a present record superseded: {r}");
    let unrelated = edit(&copy("example.next"), "(supersedes)", "(supersedes)");
    let rename = edit(
        &unrelated,
        "(catalog-record example.next",
        "(catalog-record example.fresh",
    );
    add(
        &mut h,
        &n('4'),
        &[&n('2')],
        tree(&[&rename, &bare()], &[]),
        None,
    );
    let r = replay(&h, &[&n('2')], &n('4')).unwrap_err();
    assert_eq!(r.code, Code::LockRetired, "{r}");
    assert!(r.message.contains("not taken again"), "{r}");
    // The successor drops the lineage.
    let dropped = edit(&next, "(supersedes example.base)", "(supersedes)").replacen(
        "(version \"0.1.0\")",
        "(version \"0.1.1\")",
        1,
    );
    add(&mut h, &n('5'), &[&n('2')], tree(&[&dropped], &[]), None);
    let r = replay(&h, &[&n('2')], &n('5')).unwrap_err();
    assert_eq!(r.code, Code::LockRetired, "{r}");
    assert!(r.message.contains("never dropped"), "{r}");
}

#[test]
fn a_retired_ids_rejection_binds_a_successor_at_any_remove() {
    let t0 = tree(&[&bare()], &[]);
    let rejected = with(
        &bare(),
        &review(TIMING, &bound(&t0, "example.base", TIMING), "rejected", &[]),
    );
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&rejected], &[]), None);
    let next = edit(
        &copy("example.next"),
        "(supersedes)",
        "(supersedes example.base)",
    );
    add(&mut h, &n('2'), &[&n('1')], tree(&[&next], &[]), None);
    let last = edit(
        &edit(
            &copy("example.last"),
            "(supersedes)",
            "(supersedes example.next)",
        ),
        "(none \"the costs of a machine are its devices'\")",
        "(none \"the last machine's costs are its devices'\")",
    );
    add(&mut h, &n('3'), &[&n('2')], tree(&[&last], &[]), None);
    replay(&h, &[&n('2')], &n('3')).unwrap_or_else(|e| panic!("superseded at two removes: {e}"));
    check_history(&h, &n('3')).unwrap_or_else(|e| panic!("{e}"));
}
