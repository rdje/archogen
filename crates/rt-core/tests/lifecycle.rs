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

use rt_core::{Decision, Fault, OverrunPolicy, Scheduler, TaskState, Transition};

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

    assert_eq!(s.complete(0), Transition::Completed { task: 0 });
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

    s.mask();
    assert!(s.is_masked());
    assert_eq!(s.release(0), Transition::Latched { task: 0 });
    assert_eq!(
        s.decide(),
        Decision::Continue { task: 2 },
        "a latched release must not preempt inside a critical section"
    );

    let delivered: Vec<Transition> = s.unmask().as_slice().collect();
    assert_eq!(delivered, vec![Transition::Released { task: 0 }]);
    assert_eq!(s.decide(), Decision::Switch { from: 2, to: 0 });
}

#[test]
fn masking_nests_and_only_the_outermost_unmask_delivers() {
    // A critical section inside another must not unmask early — the classic way a "protected"
    // region stops being one.
    let mut s = scheduler();
    s.mask();
    s.mask();
    s.release(0);
    assert!(s.unmask().is_empty(), "the inner unmask delivers nothing");
    assert!(s.is_masked());
    assert_eq!(s.unmask().len(), 1, "the outer one delivers");
    assert!(!s.is_masked());
}

#[test]
fn latched_releases_are_delivered_highest_priority_first() {
    let mut s = scheduler();
    s.mask();
    s.release(2);
    s.release(0);
    s.release(1);
    let delivered: Vec<Transition> = s.unmask().as_slice().collect();
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
fn a_second_latched_release_is_an_overrun_too() {
    // There is nowhere to put it. Masking delays delivery; it does not create a buffer.
    let mut s = scheduler();
    s.mask();
    assert_eq!(s.release(1), Transition::Latched { task: 1 });
    assert!(matches!(
        s.release(1),
        Transition::Faulted {
            fault: Fault::Overrun { .. },
            ..
        }
    ));
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
        Transition::Released { task: 0 },
        "the overrun is reported as a release of the new job, not as nothing"
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
    s.fault(Fault::UnexpectedTrap { cause: 7 });
    assert_eq!(
        s.decide(),
        Decision::Halt {
            fault: Fault::UnexpectedTrap { cause: 7 }
        },
        "after an unmodelled trap the runtime's own state is not trustworthy"
    );
}

#[test]
fn a_stack_guard_halts_and_takes_the_task_off_the_processor() {
    // Terminal, because §8.1 does not offer resumption after an invariant is violated — and a
    // task that faulted and then ran is a system nobody analyzed.
    let mut s = scheduler();
    s.release(1);
    assert_eq!(s.decide(), Decision::Dispatch { to: 1 });
    s.fault(Fault::StackGuard { task: 1 });
    assert_eq!(s.state(1), TaskState::Faulted);
    assert_ne!(s.state(1), TaskState::Running);
    assert_eq!(
        s.decide(),
        Decision::Halt {
            fault: Fault::StackGuard { task: 1 }
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
