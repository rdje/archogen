//! The ceiling gives back what a null answer counted (leaf `PROGRAM.150`): a fallible allocation that fails holds no
//! part of it. The count is the process's and a ceiling passed ends it, so the allocations run in a child process,
//! this test run again, and the parent reads how it ended.

use std::alloc::{GlobalAlloc, Layout};
use std::process::Command;

use archogen_transport::ceiling::{self, Ceiling};

/// Set in the child alone.
const CHILD: &str = "ARCHOGEN_CEILING_NULL_CHILD";

#[test]
fn a_null_answer_holds_no_part_of_the_ceiling() {
    if std::env::var_os(CHILD).is_some() {
        ceiling::from_environment(false).expect("the ceiling");
        // A quarter of what a count can say, more than any machine gives: the system answers null each time. Four
        // such answers kept in the count pass the highest ceiling there is, which ends this process with
        // `analysis-inconclusive`; given back, eight leave it where it was.
        let layout = Layout::from_size_align(1 << 62, 1).expect("a layout");
        for _ in 0..8 {
            // SAFETY: the layout's size is not zero, and a null answer is never read through.
            let at = unsafe { Ceiling.alloc(layout) };
            assert!(at.is_null(), "the system gave 4 EiB");
        }
        return;
    }
    let output = Command::new(std::env::current_exe().expect("this test"))
        .args([
            "--exact",
            "a_null_answer_holds_no_part_of_the_ceiling",
            "--test-threads=1",
        ])
        .env(CHILD, "1")
        .env(ceiling::VARIABLE, (usize::MAX >> 20).to_string())
        .output()
        .expect("the child runs");
    assert_eq!(
        output.status.code(),
        Some(0),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
}
