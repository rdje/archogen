//! F13, F14 and F15 (`ROADMAP.md` §13.1) over the device models, each by enumerating every case at small moduli, as
//! `docs/decisions/decision_executable-design-reviews.md` asks of a contract that is a decision procedure.

use sim_devices::{
    Compare, Contract, Controller, Epoch, Line, Programmed, RaisedBy, Refused, Timer,
};

const COMPARES: [Compare; 2] = [Compare::Level, Compare::Edge];

/// Every modulus small enough to enumerate, and large enough to wrap.
fn moduli() -> impl Iterator<Item = u64> {
    2..=16
}

#[test]
fn f13_every_deadline_within_the_horizon_is_raised_at_its_instant_across_a_wrap() {
    let mut wrapped = 0;
    for modulus in moduli() {
        for compare in COMPARES {
            for now in 0..3 * modulus {
                for ahead in 1..modulus {
                    let mut timer = Timer::new(Contract { modulus, compare });
                    let deadline = now + ahead;
                    wrapped += u64::from(timer.read(deadline) < timer.read(now));
                    assert_eq!(timer.program(now, deadline), Ok(Programmed::Armed));
                    for at in now..deadline {
                        timer.advance(at);
                        assert!(!timer.pending(), "early: {modulus} {now} {ahead} at {at}");
                    }
                    timer.advance(deadline);
                    assert!(timer.pending(), "lost: {modulus} {now} {ahead}");
                }
            }
        }
    }
    assert!(wrapped > 0, "no case crossed a wrap");
}

#[test]
fn f13_a_deadline_a_whole_wrap_or_more_ahead_is_refused_and_what_stood_stays() {
    for modulus in moduli() {
        for ahead in modulus..3 * modulus {
            let mut timer = Timer::new(Contract {
                modulus,
                compare: Compare::Level,
            });
            assert_eq!(timer.program(0, 1), Ok(Programmed::Armed));
            assert_eq!(
                timer.program(0, ahead),
                Err(Refused::AmbiguousHorizon {
                    ahead,
                    horizon: modulus - 1
                })
            );
            // The refused deadline's count is one the counter reaches sooner, and that is not what stays armed.
            timer.advance(1);
            assert!(timer.pending(), "what stood was lost: {modulus} {ahead}");
        }
    }
}

#[test]
fn f13_an_epoch_extends_a_modular_count_while_it_is_read_within_every_wrap() {
    for modulus in 2..=6 {
        let gaps = 0..modulus;
        for first in gaps.clone() {
            for second in gaps.clone() {
                for third in gaps.clone() {
                    let mut epoch = Epoch::new(modulus);
                    let mut now = 0;
                    for gap in [first, second, third] {
                        now += gap;
                        assert_eq!(epoch.observe(now % modulus), now, "{modulus}: {now}");
                    }
                }
            }
        }
    }
}

#[test]
fn f13_a_gap_of_a_whole_wrap_is_what_the_horizon_excludes() {
    // Held because it is the reason the horizon is an obligation: a count alone cannot show a missed wrap.
    let mut epoch = Epoch::new(8);
    assert_eq!(epoch.observe(3), 3);
    assert_ne!(epoch.observe((3 + 8) % 8), 3 + 8);
}

#[test]
fn f14_a_deadline_passed_while_programming_is_raised_late_and_never_lost() {
    for modulus in moduli() {
        for compare in COMPARES {
            for worked_out in 0..2 * modulus {
                for ahead in 0..modulus {
                    for latency in 0..modulus {
                        let mut timer = Timer::new(Contract { modulus, compare });
                        let deadline = worked_out + ahead;
                        let landed = worked_out + latency;
                        let programmed =
                            timer.program(landed, deadline).expect("within the horizon");
                        if deadline <= landed {
                            assert_eq!(
                                programmed,
                                Programmed::AlreadyDue {
                                    late_by: landed - deadline,
                                    raised_by: match compare {
                                        Compare::Level => RaisedBy::Hardware,
                                        Compare::Edge => RaisedBy::Recheck,
                                    },
                                }
                            );
                            assert!(timer.pending(), "lost: {modulus} {ahead} {latency}");
                        } else {
                            assert_eq!(programmed, Programmed::Armed);
                            timer.advance(deadline);
                            assert!(timer.pending(), "lost: {modulus} {ahead} {latency}");
                        }
                        // Once raised it stays raised until it is acknowledged, and then it is down.
                        timer.advance(deadline + 1);
                        assert!(timer.pending());
                        timer.acknowledge();
                        assert!(!timer.pending());
                    }
                }
            }
        }
    }
}

/// One thing that can happen to a controller.
#[derive(Debug, Clone, Copy)]
enum Op {
    Raise(usize),
    Lower(usize),
    Mask,
    Unmask,
    Take,
    Acknowledge,
}

const OPS: [Op; 8] = [
    Op::Raise(0),
    Op::Raise(1),
    Op::Lower(0),
    Op::Lower(1),
    Op::Mask,
    Op::Unmask,
    Op::Take,
    Op::Acknowledge,
];

/// `length` operations, written as a number in base `OPS.len()`.
fn sequence(mut number: usize, length: usize) -> impl Iterator<Item = Op> {
    (0..length).map(move |_| {
        let op = OPS[number % OPS.len()];
        number /= OPS.len();
        op
    })
}

#[test]
fn f15_an_interrupt_pending_while_masked_is_delivered_once_it_is_unmasked_and_never_before() {
    for lines in [
        [Line::Edge, Line::Edge],
        [Line::Level, Line::Level],
        [Line::Edge, Line::Level],
    ] {
        for length in 0..=5 {
            for number in 0..OPS.len().pow(u32::try_from(length).expect("small")) {
                let mut controller = Controller::new(lines);
                let mut masked = false;
                // What the contract owes: a request not yet served, per source.
                let mut owed = [false; 2];
                let mut asserted = [false; 2];
                let mut serving = None;
                for op in sequence(number, length) {
                    match op {
                        Op::Raise(source) => {
                            controller.raise(source);
                            owed[source] = true;
                            asserted[source] = true;
                        }
                        Op::Lower(source) => {
                            controller.lower(source);
                            asserted[source] = false;
                            if lines[source] == Line::Level {
                                owed[source] = false;
                            }
                        }
                        Op::Mask => {
                            controller.set_masked(true);
                            masked = true;
                        }
                        Op::Unmask => {
                            controller.set_masked(false);
                            masked = false;
                        }
                        Op::Take => {
                            let taken = controller.take();
                            if masked || serving.is_some() {
                                assert_eq!(taken, None, "taken while masked or nested: {number}");
                            } else {
                                let lowest = owed.iter().position(|owed| *owed);
                                assert_eq!(taken, lowest, "not the lowest place first: {number}");
                                if let Some(source) = taken {
                                    owed[source] = false;
                                    serving = Some(source);
                                    if lines[source] == Line::Edge {
                                        asserted[source] = false;
                                    }
                                }
                            }
                        }
                        Op::Acknowledge => {
                            controller.acknowledge();
                            if let Some(source) = serving.take() {
                                if lines[source] == Line::Level && asserted[source] {
                                    owed[source] = true;
                                }
                            }
                        }
                    }
                }
                // Whatever is still owed is delivered when it can be: nothing pending while masked is lost.
                controller.set_masked(false);
                if serving.is_some() {
                    controller.acknowledge();
                    if let Some(source) = serving.take() {
                        if lines[source] == Line::Level && asserted[source] {
                            owed[source] = true;
                        }
                    }
                }
                loop {
                    let lowest = owed.iter().position(|owed| *owed);
                    assert_eq!(controller.take(), lowest, "lost or out of order: {number}");
                    let Some(source) = lowest else {
                        break;
                    };
                    owed[source] = false;
                    controller.lower(source);
                    controller.acknowledge();
                }
            }
        }
    }
}

#[test]
fn f15_a_level_line_is_served_again_until_its_device_lowers_it() {
    let mut controller = Controller::new([Line::Level]);
    controller.raise(0);
    assert_eq!(controller.take(), Some(0));
    assert_eq!(controller.take(), None, "nested");
    controller.acknowledge();
    assert_eq!(controller.take(), Some(0), "still asserted");
    controller.lower(0);
    controller.acknowledge();
    assert_eq!(controller.take(), None);
}

#[test]
fn f15_an_edge_request_while_served_is_one_more_and_two_before_it_is_taken_are_one() {
    let mut controller = Controller::new([Line::Edge]);
    controller.raise(0);
    controller.raise(0);
    assert_eq!(controller.take(), Some(0));
    controller.raise(0);
    assert_eq!(controller.take(), None, "nested");
    controller.acknowledge();
    assert_eq!(controller.take(), Some(0));
    controller.acknowledge();
    assert_eq!(controller.take(), None);
}

#[test]
fn f15_a_disabled_source_s_request_stays_pending_and_is_taken_once_it_is_enabled() {
    let mut controller = Controller::new([Line::Edge, Line::Edge]);
    controller.enable([false, true]);
    controller.raise(0);
    assert_eq!(controller.take(), None);
    controller.enable([true, true]);
    assert_eq!(controller.take(), Some(0));
    assert_eq!(controller.serving(), Some(0));
}
