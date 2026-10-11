//! Bound parent test execution so a production hang yields a finishing, named parent failure.
//!
//! Each guard runs its unchanged test body in a child selected by its exact libtest name. A child executes
//! the body; the parent checks both the exit status and the named single-test result. Child diagnostics are
//! escaped in a parent failure so their failure lists cannot be mistaken for the outer libtest's evidence.

use std::process::Command;
use std::time::{Duration, Instant};

const CHILD: &str = "ARCHOGEN_PARENT_TEST_CHILD";
const LIMIT: Duration = Duration::from_secs(30);

fn command(name: &str) -> Command {
    let mut command = Command::new(std::env::current_exe().expect("the current test binary"));
    command
        .args(["--exact", name, "--format=pretty", "--color=never"])
        .env(CHILD, name);
    command
}

fn checked(name: &str, command: &mut Command, limit: Duration) -> Result<Duration, String> {
    let started = Instant::now();
    let output = crate::mutation::run_test_command(command, limit)
        .map_err(|error| format!("{name}: bounded test command: {error}"))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    if !output.status.success() {
        return Err(format!(
            "{name}: child failed ({}); stdout {stdout:?}; stderr {stderr:?}",
            output.status
        ));
    }
    let named = format!("test {name} ... ok");
    if stdout.lines().filter(|line| *line == named).count() != 1
        || !stdout
            .lines()
            .any(|line| line.starts_with("test result: ok. 1 passed; 0 failed; 0 ignored;"))
    {
        return Err(format!(
            "{name}: child did not report one passed exact test; stdout {stdout:?}; stderr {stderr:?}"
        ));
    }
    Ok(started.elapsed())
}

/// True in the outer test once its child passed; false in that child, which executes the body.
pub(crate) fn run(qualified: &str) -> bool {
    // module_path! starts with the binary crate's name; libtest's names start with its first module.
    let (_, name) = qualified
        .split_once("::")
        .expect("a crate-qualified test name");
    if std::env::var(CHILD).ok().as_deref() == Some(name) {
        return false;
    }
    let elapsed =
        checked(name, &mut command(name), LIMIT).unwrap_or_else(|error| panic!("{error}"));
    if std::env::var_os("ARCHOGEN_PARENT_TEST_TIMINGS").is_some() {
        eprintln!("parent-worker: {name} {:.3}s", elapsed.as_secs_f64());
    }
    true
}

macro_rules! bounded {
    ($name:ident) => {
        if $crate::trust_test_process::run(concat!(module_path!(), "::", stringify!($name))) {
            return;
        }
    };
}
pub(crate) use bounded;

#[cfg(test)]
mod tests {
    use super::{checked, command};
    use std::time::{Duration, Instant};

    const PROBE: &str = "trust_test_process::tests::worker_probe";
    const KIND: &str = "ARCHOGEN_PARENT_TEST_PROBE";

    #[test]
    #[ignore = "invoked only by the bounded worker protocol controls"]
    fn worker_probe() {
        match std::env::var(KIND).ok().as_deref() {
            None | Some("pass") => {}
            Some("assert") => panic!("deliberate parent worker assertion"),
            Some("exit") => std::process::exit(23),
            Some("hang") => loop {
                std::thread::sleep(Duration::from_millis(100));
            },
            other => panic!("unknown worker probe {other:?}"),
        }
    }

    fn probe(kind: &str) -> std::process::Command {
        let mut child = command(PROBE);
        child.arg("--ignored").env(KIND, kind);
        child
    }

    #[test]
    fn only_a_finished_named_selected_child_passes() {
        assert!(checked(PROBE, &mut probe("pass"), Duration::from_secs(10)).is_ok());
        let absent = "trust_test_process::tests::no_such_test";
        let error = checked(
            absent,
            command(absent).env(KIND, "pass"),
            Duration::from_secs(10),
        )
        .expect_err("zero selected tests must refuse");
        assert!(
            error.contains("did not report one passed exact test"),
            "{error}"
        );
    }

    #[test]
    fn child_assertions_and_abrupt_exits_are_parent_failures() {
        for kind in ["assert", "exit"] {
            let error = checked(PROBE, &mut probe(kind), Duration::from_secs(10))
                .expect_err("a failing child must refuse");
            assert!(error.contains("child failed"), "{error}");
            assert!(
                !error.contains('\n'),
                "child diagnostics must stay escaped: {error:?}"
            );
        }
    }

    #[test]
    fn a_hanging_child_is_stopped_and_refused() {
        let started = Instant::now();
        let error = checked(PROBE, &mut probe("hang"), Duration::from_millis(300))
            .expect_err("a hang must refuse");
        assert!(error.contains("exceeded its limit"), "{error}");
        assert!(
            started.elapsed() < Duration::from_secs(10),
            "the child was waited on"
        );
    }
}
