//! The scheduler's half of what the composite-inputs record's clause on the runtime's own data rests on (leaf `M2.30`,
//! `docs/specs/catalog/decision_runtime-composite-inputs.md` §2). The fields are private, so no test here can make the
//! stray write itself: each makes the call that leaves the state such a write would, or a call in no role of that
//! record's, and reads what the state machine then does. A port, which no code holds yet, supplies the other half: that it
//! unmasks only at depth zero, and that a transition checks the depth. The record's second consequence, a next release
//! rewritten, is the timer service's alone, and no code holds that either.
//!
//! These pin why the clause is needed, not a behaviour anything relies on: each is what the composition must not meet.

use rt_core::{Decision, Fault, OverrunPolicy, Scheduler, TaskState, Transition};

/// Three tasks, index 0 the highest priority, all faulting on overrun.
fn scheduler() -> Scheduler<3> {
    Scheduler::new([OverrunPolicy::Fault; 3])
}

#[test]
fn a_depth_raised_inside_a_masked_run_outlasts_the_unmask_the_run_declares() {
    let mut s = scheduler();
    s.release(1);
    assert_eq!(s.decide(), Decision::Dispatch { to: 1 });

    s.mask().expect("the run's own");
    s.mask().expect("the depth a stray write would have raised");
    assert!(
        s.unmask().expect("masked").is_empty(),
        "the unmask the run declares as its end delivers nothing"
    );
    assert!(s.is_masked(), "and it does not end the run");

    // No check sees it: the completion closes the depth, as the fault contract's rule 4 lets a job's own.
    assert_eq!(s.complete(1).0, Transition::Completed { task: 1 });
    assert!(!s.is_masked());
}

#[test]
fn a_depth_raised_with_the_hardware_unmasked_has_the_next_service_latch_its_release() {
    let mut s = scheduler();
    s.release(1);
    s.decide();

    s.mask().expect("the depth a stray write would have raised");
    assert_eq!(
        s.release(0),
        Transition::Latched { task: 0 },
        "a release the variant's §2 takes as ready at the service's start"
    );
    // The state machine resumes the job here; on a target the fault contract's Terms have the transition halt.
    assert_eq!(s.decide(), Decision::Continue { task: 1 });
    assert_eq!(s.state(0), TaskState::Suspended);
}

#[test]
fn a_service_that_decides_and_ignores_the_decision_rewrites_which_task_runs() {
    let mut s = scheduler();
    s.release(1);
    s.decide();

    s.release(0);
    assert_eq!(
        s.decide(),
        Decision::Switch { from: 1, to: 0 },
        "the decision a driver's service would make and ignore, with no switch performed"
    );
    // The port's own decision finds none to make: task 1 still physically runs, and task 0 is marked as running.
    assert_eq!(s.decide(), Decision::Continue { task: 0 });
    assert_eq!(s.state(0), TaskState::Running);
    assert_eq!(s.state(1), TaskState::Ready);
    // Nothing says so until task 1's completion, which `complete` asserts, or the next release of a task still owing a
    // job, here task 0's, which the state machine reads as an overrun.
    assert_eq!(
        s.release(0),
        Transition::Faulted {
            task: 0,
            fault: Fault::Overrun { task: 0 }
        }
    );
}

#[test]
fn code_in_a_job_that_runs_on_after_the_completion_path_leaves_no_task_running() {
    let mut s = scheduler();
    s.release(1);
    s.decide();

    assert_eq!(s.complete(1).0, Transition::Completed { task: 1 });
    // The job's code runs on, and the state machine sees an idle processor and a fresh next release.
    assert_eq!(s.decide(), Decision::Idle);
    assert_eq!(s.state(1), TaskState::Suspended);
    assert_eq!(
        s.release(1),
        Transition::Released { task: 1 },
        "no overrun, though the job still runs"
    );
}
