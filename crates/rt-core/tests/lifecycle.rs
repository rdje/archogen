//! `M2.1`'s acceptance: the task lifecycle is testable **without a target**.
//!
//! §12 M2 wants "state transitions … testable without a target; policy separated from the
//! execution substrate; no heap, no generic object manager". Every test here runs on the host and
//! never performs a context switch — it checks the *decision*, which is all this crate makes.
//!
//! ⛔ These tests are written against `ROADMAP.md` §3.1's and §8's stated behavior, not against
//! the implementation's shape. `M2.2` builds an independent reference model of the same
//! semantics, and §12 M2 warns that "a checker sharing the same erroneous recurrence with its
//! reference does not qualify as independent" — so what these assert is the **contract**, which
//! both implementations must satisfy.

use rt_core::{
    Context, Decision, Fatal, Fault, OverrunPolicy, Refused, Scheduler, TaskState, Transition,
};

/// The record a halt keeps (§3.1.1 rule 7).
fn fatal(fault: Fault, task: Option<usize>, interrupted: Option<usize>, escalated: bool) -> Fatal {
    Fatal {
        fault,
        task,
        interrupted,
        escalated,
    }
}

/// Three tasks, index 0 the highest priority, all faulting on overrun — the examples' shape.
fn scheduler() -> Scheduler<3> {
    Scheduler::new([OverrunPolicy::Fault; 3])
}

#[test]
fn every_task_starts_suspended_and_an_empty_system_idles() {
    let mut s = scheduler();
    for task in 0..3 {
        assert_eq!(s.state(task), TaskState::Suspended);
    }
    assert_eq!(s.decide(), Decision::Idle);
    // Idempotent: consulting the decision is not itself an event.
    assert_eq!(s.decide(), Decision::Idle);
}

#[test]
fn entering_from_idle_is_a_dispatch_and_not_a_switch() {
    // §13.4 charges these differently, and the difference is real: there is no outgoing context
    // to save. A scheduler reporting `Switch` here would leave the cost model unable to tell them
    // apart.
    let mut s = scheduler();
    assert_eq!(s.release(1), Transition::Released { task: 1 });
    assert_eq!(s.decide(), Decision::Dispatch { to: 1 });
    assert_eq!(s.state(1), TaskState::Running);
}

#[test]
fn a_higher_priority_release_preempts_and_the_preempted_task_returns_to_ready() {
    let mut s = scheduler();
    s.release(2);
    assert_eq!(s.decide(), Decision::Dispatch { to: 2 });

    s.release(0);
    assert_eq!(s.decide(), Decision::Switch { from: 2, to: 0 });
    assert_eq!(
        s.state(2),
        TaskState::Ready,
        "the preempted job is not lost"
    );
    assert_eq!(s.state(0), TaskState::Running);
}

#[test]
fn a_lower_priority_release_does_not_preempt() {
    // Fixed priority, and the index IS the rank. A release that cannot run must not cost a
    // switch, or every low-priority release would charge one for nothing.
    let mut s = scheduler();
    s.release(0);
    assert_eq!(s.decide(), Decision::Dispatch { to: 0 });
    s.release(2);
    assert_eq!(s.decide(), Decision::Continue { task: 0 });
    assert_eq!(s.state(2), TaskState::Ready);
}

#[test]
fn completion_resumes_the_highest_priority_task_still_ready() {
    let mut s = scheduler();
    s.release(2);
    s.decide();
    s.release(0);
    s.decide();

    assert_eq!(s.complete(0).0, Transition::Completed { task: 0 });
    assert_eq!(
        s.state(0),
        TaskState::Suspended,
        "it awaits its next release"
    );
    assert_eq!(
        s.decide(),
        Decision::Dispatch { to: 2 },
        "the preempted job resumes"
    );
}

#[test]
fn a_release_while_masked_is_latched_and_delivered_on_unmask() {
    // §8.1: "Model synchronization and interrupt masking explicitly." It is the §13.4 rule too:
    // arrivals during a non-preemptible section are serviced before the next task computation
    // interval, not dropped.
    let mut s = scheduler();
    s.release(2);
    s.decide();

    s.mask().expect("within the declared bound");
    assert!(s.is_masked());
    assert_eq!(s.release(0), Transition::Latched { task: 0 });
    assert_eq!(
        s.decide(),
        Decision::Continue { task: 2 },
        "a latched release must not preempt inside a critical section"
    );

    let delivered: Vec<Transition> = s.unmask().expect("masked").as_slice().collect();
    assert_eq!(delivered, vec![Transition::Released { task: 0 }]);
    assert_eq!(s.decide(), Decision::Switch { from: 2, to: 0 });
}

#[test]
fn masking_nests_and_only_the_outermost_unmask_delivers() {
    // A critical section inside another must not unmask early — the classic way a "protected"
    // region stops being one.
    let mut s = scheduler();
    s.mask().expect("within the declared bound");
    s.mask().expect("within the declared bound");
    s.release(0);
    assert!(
        s.unmask().expect("masked").is_empty(),
        "the inner unmask delivers nothing"
    );
    assert!(s.is_masked());
    assert_eq!(
        s.unmask().expect("masked").len(),
        1,
        "the outer one delivers"
    );
    assert!(!s.is_masked());
}

#[test]
fn latched_releases_are_delivered_highest_priority_first() {
    let mut s = scheduler();
    s.mask().expect("within the declared bound");
    s.release(2);
    s.release(0);
    s.release(1);
    let delivered: Vec<Transition> = s.unmask().expect("masked").as_slice().collect();
    assert_eq!(
        delivered,
        vec![
            Transition::Released { task: 0 },
            Transition::Released { task: 1 },
            Transition::Released { task: 2 },
        ]
    );
}

#[test]
fn releasing_a_task_whose_job_has_not_completed_is_an_overrun() {
    // §3.1 requires a defined overrun policy, so this must be an event with a name — not a second
    // pending job, which would be a queue in a profile that excludes queues.
    let mut s = scheduler();
    s.release(1);
    s.decide();
    assert_eq!(
        s.release(1),
        Transition::Faulted {
            task: 1,
            fault: Fault::Overrun { task: 1 }
        }
    );
    assert_eq!(s.state(1), TaskState::Faulted);
}

#[test]
fn a_second_latched_release_is_an_overrun_judged_at_delivery() {
    // §3.1.1 rule 1, amended by findings §6 (b): there is nowhere to put it, so it is an overrun,
    // and it is judged at delivery, outside the masked region, under the task's own policy. Not
    // lost, and not fatal for landing inside a critical section rather than one instruction after.
    let mut s = scheduler();
    s.mask().expect("within the declared bound");
    assert_eq!(s.release(1), Transition::Latched { task: 1 });
    assert_eq!(s.release(1), Transition::OverrunLatched { task: 1 });
    assert_eq!(
        s.state(1),
        TaskState::Suspended,
        "nothing is judged while masked"
    );
    assert!(
        !matches!(s.decide(), Decision::Halt { .. }),
        "and nothing halts"
    );
    let delivered: Vec<Transition> = s.unmask().expect("masked").as_slice().collect();
    assert_eq!(
        delivered,
        vec![
            Transition::Released { task: 1 },
            Transition::Faulted {
                task: 1,
                fault: Fault::Overrun { task: 1 }
            },
        ],
        "the release is delivered, then the overrun beside it applies the task's `Fault` policy"
    );
    assert!(
        !matches!(s.decide(), Decision::Halt { .. }),
        "contained, as it would be unmasked"
    );
}

#[test]
fn a_latched_overrun_under_skip_late_job_becomes_the_next_job() {
    let mut s: Scheduler<2> = Scheduler::new([OverrunPolicy::SkipLateJob, OverrunPolicy::Fault]);
    s.mask().expect("maskable");
    s.release(0);
    assert_eq!(s.release(0), Transition::OverrunLatched { task: 0 });
    assert_eq!(
        s.release(0),
        Transition::OverrunLatched { task: 0 },
        "a third is the same overrun"
    );
    let delivered: Vec<Transition> = s.unmask().expect("masked").as_slice().collect();
    assert_eq!(
        delivered,
        vec![
            Transition::Released { task: 0 },
            Transition::JobSkipped { task: 0 }
        ],
        "the late job is skipped and the release that overran becomes the next job"
    );
    assert_eq!(s.state(0), TaskState::Ready);
    assert_eq!(s.decide(), Decision::Dispatch { to: 0 });
}

#[test]
fn a_completion_inside_a_masked_region_closes_it_and_delivers() {
    // §3.1.1 rule 4, findings §6 (a): the depth is the job's and ends with it, and what was latched
    // is delivered as at the outermost unmask, after the completion.
    let mut s = scheduler();
    s.release(2);
    assert_eq!(s.decide(), Decision::Dispatch { to: 2 });
    s.mask().expect("maskable");
    s.mask().expect("nested");
    assert_eq!(s.release(0), Transition::Latched { task: 0 });
    let (completed, delivered) = s.complete(2);
    assert_eq!(completed, Transition::Completed { task: 2 });
    assert_eq!(
        delivered.as_slice().collect::<Vec<_>>(),
        vec![Transition::Released { task: 0 }]
    );
    assert!(
        !s.is_masked(),
        "the job's sections end with it, nested ones included"
    );
    assert_eq!(s.decide(), Decision::Dispatch { to: 0 });
}

#[test]
fn a_task_completing_inside_its_region_is_released_afresh_by_its_own_latched_release() {
    let mut s = scheduler();
    s.release(1);
    s.decide();
    s.mask().expect("maskable");
    assert_eq!(s.release(1), Transition::Latched { task: 1 });
    let (_, delivered) = s.complete(1);
    assert_eq!(
        delivered.as_slice().collect::<Vec<_>>(),
        vec![Transition::Released { task: 1 }],
        "judged at delivery, after the job completed: a new job, not an overrun"
    );
    assert_eq!(s.state(1), TaskState::Ready);
}

#[test]
fn a_completion_delivers_an_overrun_latched_beside_a_release() {
    let mut s = scheduler();
    s.release(2);
    s.decide();
    s.mask().expect("maskable");
    s.release(0);
    assert_eq!(s.release(0), Transition::OverrunLatched { task: 0 });
    let (_, delivered) = s.complete(2);
    assert_eq!(
        delivered.as_slice().collect::<Vec<_>>(),
        vec![
            Transition::Released { task: 0 },
            Transition::Faulted {
                task: 0,
                fault: Fault::Overrun { task: 0 }
            },
        ]
    );
    assert!(!matches!(s.decide(), Decision::Halt { .. }));
}

#[test]
fn an_unmasked_completion_delivers_nothing() {
    let mut s = scheduler();
    s.release(0);
    s.decide();
    let (_, delivered) = s.complete(0);
    assert!(delivered.is_empty());
}

#[test]
fn the_skip_late_job_policy_accepts_the_new_release_and_says_so() {
    // §3.1 admits a defined policy other than faulting. What it may never be is silent: a dropped
    // job is a missed deadline by another name.
    let mut s: Scheduler<2> = Scheduler::new([OverrunPolicy::SkipLateJob, OverrunPolicy::Fault]);
    s.release(0);
    s.decide();
    assert_eq!(
        s.release(0),
        Transition::JobSkipped { task: 0 },
        "the overrun is reported as a skipped job, neither as nothing nor as an ordinary release"
    );
    assert_eq!(s.state(0), TaskState::Ready);
    assert_eq!(s.decide(), Decision::Dispatch { to: 0 });
}

#[test]
fn an_overrun_leaves_the_runtime_schedulable_and_a_trap_does_not() {
    // §8.1 distinguishes an expected error from a violated invariant, and the difference is
    // whether continuing to schedule means running a system nobody analyzed.
    let mut s = scheduler();
    s.release(0);
    s.decide();
    s.release(0); // overrun → task 0 faults
    assert_eq!(s.state(0), TaskState::Faulted);
    s.release(2);
    assert_eq!(
        s.decide(),
        Decision::Dispatch { to: 2 },
        "one faulted task does not stop the others"
    );

    let mut s = scheduler();
    let trap = Fault::UnexpectedTrap { cause: 7 };
    s.fault(trap, Context::Kernel);
    assert_eq!(
        s.decide(),
        Decision::Halt {
            fatal: fatal(trap, None, None, false)
        },
        "after an unmodelled trap the runtime's own state is not trustworthy"
    );
}

#[test]
fn a_trap_or_an_assertion_is_attributed_to_the_running_task() {
    // §3.1.1's table and rule 2 (amended 2026-10-01, findings §6 (c), (d)): both are synchronous
    // to the executing context and attributed to it. Neither is containable.
    for fault in [
        Fault::UnexpectedTrap { cause: 7 },
        Fault::InvariantViolated {
            invariant: "ready-set",
        },
    ] {
        let mut s = scheduler();
        s.release(1);
        s.decide();
        let kept = fatal(fault, Some(1), None, false);
        assert_eq!(
            s.fault(fault, Context::Job),
            Transition::Halted { fatal: kept }
        );
        assert_eq!(s.state(1), TaskState::Faulted);
        assert_eq!(s.decide(), Decision::Halt { fatal: kept });
    }
}

#[test]
fn a_trap_in_kernel_code_is_no_tasks_and_names_the_task_it_interrupted() {
    // §3.1.1 rule 2 (the review's finding 8): a service, the trap path, a transition or idle is no
    // task, and a task whose job the fault interrupted is recorded as interrupted, never as
    // attributed.
    let mut s = scheduler();
    s.release(1);
    s.decide();
    let trap = Fault::UnexpectedTrap { cause: 7 };
    let kept = fatal(trap, None, Some(1), false);
    assert_eq!(
        s.fault(trap, Context::Kernel),
        Transition::Halted { fatal: kept }
    );
    assert_eq!(
        s.state(1),
        TaskState::Running,
        "the interrupted task is not marked"
    );
    assert_eq!(s.decide(), Decision::Halt { fatal: kept });
}

#[test]
fn a_stack_guard_names_whose_guard_apart_from_who_is_blamed() {
    // The interrupt stack's guard, hit by a service: no task is attributed, and the fault says
    // whose guard it was.
    let mut s = scheduler();
    s.release(2);
    s.decide();
    let guard = Fault::StackGuard { task: None };
    s.fault(guard, Context::Kernel);
    assert_eq!(
        s.decide(),
        Decision::Halt {
            fatal: fatal(guard, None, Some(2), false)
        }
    );
}

#[test]
fn the_first_fatal_fault_is_the_one_kept() {
    // A fault raised after the runtime halted is a consequence; the halt reports the cause.
    let mut s = scheduler();
    s.release(1);
    s.decide();
    let guard = Fault::StackGuard { task: Some(1) };
    s.fault(guard, Context::Job);
    let kept = fatal(guard, Some(1), None, false);
    // One raised with no task to attribute it to, and one attributed to a task.
    s.fault(Fault::UnexpectedTrap { cause: 7 }, Context::Kernel);
    s.fault(Fault::StackGuard { task: Some(2) }, Context::Job);
    assert_eq!(s.decide(), Decision::Halt { fatal: kept });
}

#[test]
fn a_halted_runtime_changes_nothing_afterwards() {
    // §3.1.1 rule 7 (the review's finding 9): "From then no job runs, no release is processed or
    // latched, and no transition occurs."
    let mut s = scheduler();
    s.release(1);
    s.decide();
    s.release(2);
    let guard = Fault::StackGuard { task: Some(1) };
    let Transition::Halted { fatal: kept } = s.fault(guard, Context::Job) else {
        panic!("a stack guard halts");
    };
    let halted = Transition::Halted { fatal: kept };
    assert_eq!(s.release(0), halted, "not processed");
    assert_eq!(s.state(0), TaskState::Suspended);
    assert_eq!(s.fault(Fault::Overrun { task: 2 }, Context::Kernel), halted);
    assert_eq!(s.state(2), TaskState::Ready, "not contained, not faulted");
    assert_eq!(s.mask(), Err(Refused::Halted));
    assert_eq!(s.unmask().unwrap_err(), Refused::Halted);
    assert!(!s.is_masked(), "and not latched either");
    assert_eq!(s.complete(2).0, halted);
    assert_eq!(s.decide(), Decision::Halt { fatal: kept });
}

#[test]
fn a_trap_while_no_task_runs_is_attributed_to_no_task() {
    // The executing context is the kernel's — the idle loop or a handler — so no task carries it.
    for context in [Context::Job, Context::Kernel] {
        let mut s = scheduler();
        let fault = Fault::UnexpectedTrap { cause: 7 };
        let kept = fatal(fault, None, None, false);
        assert_eq!(s.fault(fault, context), Transition::Halted { fatal: kept });
        assert_eq!(s.state(0), TaskState::Suspended);
        assert_eq!(s.decide(), Decision::Halt { fatal: kept });
    }
}

#[test]
fn an_overrun_raised_without_a_release_starts_no_job() {
    // §3.1.1 rule 1 makes the triggering release the next job under `SkipLateJob`. An overrun an
    // execution-budget monitor raises has none, so the late job is abandoned and the task waits.
    let mut s: Scheduler<3> = Scheduler::new([OverrunPolicy::SkipLateJob; 3]);
    s.release(0);
    assert_eq!(s.decide(), Decision::Dispatch { to: 0 });
    assert_eq!(
        s.fault(Fault::Overrun { task: 0 }, Context::Kernel),
        Transition::JobSkipped { task: 0 }
    );
    assert_eq!(s.state(0), TaskState::Suspended);
    assert_eq!(s.decide(), Decision::Idle);

    // A release that finds the job late does start one.
    s.release(0);
    s.decide();
    assert_eq!(s.release(0), Transition::JobSkipped { task: 0 });
    assert_eq!(s.state(0), TaskState::Ready);
    assert_eq!(s.decide(), Decision::Dispatch { to: 0 });
}

#[test]
fn a_stack_guard_halts_and_takes_the_task_off_the_processor() {
    // Terminal, because §8.1 does not offer resumption after an invariant is violated — and a
    // task that faulted and then ran is a system nobody analyzed.
    let mut s = scheduler();
    s.release(1);
    assert_eq!(s.decide(), Decision::Dispatch { to: 1 });
    let guard = Fault::StackGuard { task: Some(1) };
    s.fault(guard, Context::Job);
    assert_eq!(s.state(1), TaskState::Faulted);
    assert_ne!(s.state(1), TaskState::Running);
    assert_eq!(
        s.decide(),
        Decision::Halt {
            fatal: fatal(guard, Some(1), None, false)
        }
    );
}

#[test]
fn the_whole_state_machine_fits_in_a_fixed_size_with_no_allocation() {
    // §8: "no generic object manager", and §3.1 excludes a runtime heap. The scheduler's size is
    // a function of N alone — if a `Vec`, a `Box` or a trait object appeared, this stops holding.
    use core::mem::size_of;
    assert!(
        size_of::<Scheduler<3>>() < 128,
        "{}",
        size_of::<Scheduler<3>>()
    );
    assert!(
        size_of::<Scheduler<32>>() > size_of::<Scheduler<3>>(),
        "the task set is inline, not behind a pointer"
    );
}

// ═════════════════════════════════════════════════════════════════════════════════════════════
// The five questions `ROADMAP.md` §3.1.1 and the amended priority record now decide (leaf `M2.9`).
//
// Each was found by an independently derived model reading the same contract and answering
// differently. Four of the five went against this implementation.
// ═════════════════════════════════════════════════════════════════════════════════════════════

#[test]
fn an_empty_task_set_is_refused_at_boot() {
    // §3.1.1: "A task set must be non-empty." "Finite static task set" admits the empty set, and a
    // system with no workload makes §7.2's second timing obligation vacuous — a vacuously passing
    // schedulability result is what §7.1 exists to prevent.
    let empty: Result<Scheduler<0>, _> = Scheduler::from_eadl_ranks([], []);
    assert_eq!(empty.unwrap_err(), rt_core::BootError::NoTasks);
}

#[test]
fn an_eadl_rank_below_one_is_refused() {
    // The amended `decision_priority-comparison-direction.md`: a rank is an integer `N >= 1`.
    // Admitting 0 would move the top of the range by inference, and §15 makes a change to a
    // parameter's meaning a versioned language change rather than a tolerance.
    let refused = Scheduler::from_eadl_ranks([0, 1, 2], [OverrunPolicy::Fault; 3]);
    assert_eq!(
        refused.unwrap_err(),
        rt_core::BootError::RankBelowOne { position: 0 }
    );
}

#[test]
fn the_eadl_rank_to_runtime_index_mapping_is_performed_in_one_place() {
    // ⛔ The off-by-one that was written down nowhere: a task's array index here IS its priority
    // and indices start at 0, while the language's highest rank is 1. A description listing its
    // tasks in any order must still produce a scheduler ordered by rank.
    let s: Scheduler<3> = Scheduler::from_eadl_ranks(
        // described in the order c, a, b — with ranks 3, 1, 2
        [3, 1, 2],
        [
            OverrunPolicy::Fault,
            OverrunPolicy::SkipLateJob,
            OverrunPolicy::Fault,
        ],
    )
    .expect("distinct ranks, none below 1");
    let mut s = s;
    // Rank 1 is the highest, and it lands at index 0.
    s.release(0);
    s.release(2);
    assert_eq!(s.decide(), Decision::Dispatch { to: 0 });
    // The policy travelled with the task, not with its position in the description.
    s.release(0);
    assert_eq!(
        s.state(0),
        TaskState::Ready,
        "the task described second carried SkipLateJob and must still have it at index 0"
    );
}

#[test]
fn duplicate_ranks_are_refused() {
    // §3.1: "static **unique** task priorities".
    assert_eq!(
        Scheduler::from_eadl_ranks([1, 1, 2], [OverrunPolicy::Fault; 3]).unwrap_err(),
        rt_core::BootError::DuplicateRank { rank: 1 }
    );
    assert_eq!(
        Scheduler::from_eadl_ranks([4, 9, 4], [OverrunPolicy::Fault; 3]).unwrap_err(),
        rt_core::BootError::DuplicateRank { rank: 4 }
    );
}

#[test]
fn ranks_with_gaps_lower_by_their_order() {
    // The language admits any distinct ranks from 1, and fixed priority uses only their order
    // (`hp(i) = { j : N_j < N_i }`), so the index is the number of tasks that outrank it.
    let mut s: Scheduler<3> = Scheduler::from_eadl_ranks(
        // described in the order c, a, b — with ranks 9, 1, 5
        [9, 1, 5],
        [
            OverrunPolicy::SkipLateJob,
            OverrunPolicy::Fault,
            OverrunPolicy::Fault,
        ],
    )
    .expect("distinct ranks, none below 1");
    s.release(1);
    s.release(2);
    assert_eq!(
        s.decide(),
        Decision::Dispatch { to: 1 },
        "rank 5 outranks rank 9"
    );
    s.release(2);
    assert_eq!(
        s.state(2),
        TaskState::Ready,
        "rank 9, described first with SkipLateJob, is index 2 and kept its policy"
    );
}

#[test]
fn exceeding_the_mask_bound_is_an_assertion_failure() {
    // §3.1.1: a counter that WRAPS re-enables interrupts inside a critical section while
    // reporting success; one that SATURATES stops counting; one that REFUSES leaves its caller's
    // matching `unmask` to close the section early. Each fails silently, so exceeding the bound
    // halts (the review's finding 10; until 2026-10-01 it refused).
    let mut s = scheduler();
    for _ in 0..Scheduler::<3>::MASK_DEPTH_LIMIT {
        s.mask().expect("within the declared bound");
    }
    assert_eq!(s.mask(), Err(Refused::Halted));
    assert!(s.is_masked(), "and the depth did not wrap to zero");
    let bound = Fault::InvariantViolated {
        invariant: "mask-depth-bound",
    };
    assert_eq!(
        s.decide(),
        Decision::Halt {
            fatal: fatal(bound, None, None, false)
        }
    );
}

#[test]
fn an_unmask_with_nothing_to_close_is_an_assertion_failure() {
    // Taken through the fatal path rather than a panic: §8.1 asks for "a defined fatal handler and
    // diagnostic evidence", and a panic in the scheduler reaches neither.
    let mut s = scheduler();
    s.release(0);
    s.decide();
    assert_eq!(s.unmask().unwrap_err(), Refused::Halted);
    let unbalanced = Fault::InvariantViolated {
        invariant: "unmask-at-depth-zero",
    };
    assert_eq!(
        s.decide(),
        Decision::Halt {
            fatal: fatal(unbalanced, Some(0), None, false)
        },
        "raised by the running job, so attributed to it"
    );
}

#[test]
fn a_job_starting_inside_a_masked_region_is_an_assertion_failure() {
    // §3.1.1, Terms: every job starts at depth zero, which is what makes every section open at a
    // completion the completing job's (rule 4, the review's finding 16).
    let mut s = scheduler();
    s.release(0);
    s.mask().expect("maskable");
    let masked = Fault::InvariantViolated {
        invariant: "dispatch-while-masked",
    };
    assert_eq!(
        s.decide(),
        Decision::Halt {
            fatal: fatal(masked, None, None, false)
        }
    );
}

#[test]
fn a_containable_fault_raised_while_masked_escalates() {
    // §3.1.1 rule 3, on both its grounds: terminating a job that holds the mask orphans the
    // nesting depth, and containment means resuming the schedule from a state the critical
    // section had not finished making consistent (amended 2026-10-01, findings §6 (e)).
    let mut s: Scheduler<3> = Scheduler::new([OverrunPolicy::SkipLateJob; 3]);
    s.release(0);
    s.decide();
    s.mask().expect("maskable");
    s.fault(Fault::Overrun { task: 0 }, Context::Kernel);
    assert_eq!(
        s.decide(),
        Decision::Halt {
            fatal: fatal(Fault::Overrun { task: 0 }, Some(0), None, true)
        },
        "SkipLateJob would have contained this outside a critical section, and the record says rule 3 escalated it"
    );
}

#[test]
fn an_overrun_discovered_at_unmask_applies_its_ordinary_policy() {
    // ⚠️ The other half of rule 3, and the one an implementer is most likely to get wrong:
    // delivery at unmask is NOT inside a masked region. The latched release is delivered once the
    // outermost section closes, so the overrun it discovers is contained by the task's own policy
    // rather than escalating. Getting this wrong makes the profile unable to contain an overrun
    // at all.
    let mut s: Scheduler<3> = Scheduler::new([OverrunPolicy::SkipLateJob; 3]);
    s.release(0);
    s.decide(); // task 0 is running and owes a job
    s.mask().expect("maskable");
    assert_eq!(s.release(0), Transition::Latched { task: 0 });

    let delivered: Vec<Transition> = s.unmask().expect("masked").as_slice().collect();
    assert_eq!(
        delivered,
        vec![Transition::JobSkipped { task: 0 }],
        "SkipLateJob abandons the late job and accepts the new release"
    );
    assert!(
        !matches!(s.decide(), Decision::Halt { .. }),
        "delivery at unmask is outside the critical section, so containment applies"
    );
}

#[test]
fn an_overrun_applies_its_policy_on_detection_without_a_second_call() {
    // §3.1.1 rule 1: "Detection applies the policy." §3.1 requires the policy to be *defined* and
    // §7.3 makes it part of the task record, so it is the system's behaviour rather than a
    // caller's option — nothing else has to ask for it.
    let mut s: Scheduler<3> = Scheduler::new([OverrunPolicy::Fault; 3]);
    s.release(2);
    s.decide();
    assert_eq!(
        s.release(2),
        Transition::Faulted {
            task: 2,
            fault: Fault::Overrun { task: 2 }
        }
    );
    assert_eq!(s.state(2), TaskState::Faulted);
}

#[test]
fn an_overrun_is_attributed_to_the_overrunning_task_even_when_it_is_not_running() {
    // §3.1.1 rule 2, and the one place the implementation's original reading survived review: a
    // release is signalled by an interrupt while some other context holds the processor, so the
    // overrunning task is by construction not the running one. The single-core rule governs
    // EXECUTION, not attribution.
    let mut s = scheduler();
    s.release(1);
    s.release(0);
    s.decide(); // task 0 runs; task 1 is Ready and owes a job
    assert_eq!(s.state(0), TaskState::Running);
    assert_eq!(s.state(1), TaskState::Ready);

    s.release(1);
    assert_eq!(
        s.state(1),
        TaskState::Faulted,
        "the overrunning task faulted"
    );
    assert_eq!(
        s.state(0),
        TaskState::Running,
        "and the running task did not"
    );
}
