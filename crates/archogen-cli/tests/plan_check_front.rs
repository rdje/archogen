//! `archogen plan-check` at the front (leaf `M3.5.1.4.1`): the command a sibling program answers, run through the
//! launcher the caller hands over — here a fake that records what it was asked and ends as it is told — so the front's
//! half is held without a process: the arguments handed over unparsed, the sibling's exit code relayed, its help its
//! own, and every ending the vocabulary does not hold, or a sibling not run, `tool-failure`.

use std::cell::RefCell;
use std::io::Write;

use archogen_cli::launch::{Ended, Launch, Launched, Unlaunched};
use archogen_cli::spec::COMMANDS;
use archogen_cli::{run, run_with_input, Status};
use archogen_digest::Digest;

/// A launcher that records each request and answers with `answer`, writing `said` to the sibling's two streams.
struct Fake {
    asked: RefCell<Vec<(String, Vec<String>)>>,
    answer: Result<Ended, Unlaunched>,
}

impl Fake {
    fn ending(answer: Result<Ended, Unlaunched>) -> Self {
        Self {
            asked: RefCell::new(Vec::new()),
            answer,
        }
    }
}

impl Launch for Fake {
    fn launch(
        &self,
        program: &str,
        args: &[String],
        out: &mut dyn Write,
        err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched> {
        self.asked
            .borrow_mut()
            .push((program.to_owned(), args.to_vec()));
        let _ = writeln!(out, "the sibling's output");
        let _ = writeln!(err, "the sibling's findings");
        self.answer.clone().map(|ended| Launched {
            ended,
            sha256: Digest::of(b"the sibling"),
        })
    }
}

/// Run `args` through `launcher`: the status, standard output and standard error.
fn front(args: &[&str], launcher: &dyn Launch) -> (Status, String, String) {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let status = run_with_input(
        args.iter().copied(),
        &mut std::io::empty(),
        launcher,
        &mut out,
        &mut err,
    );
    (
        status,
        String::from_utf8(out).unwrap(),
        String::from_utf8(err).unwrap(),
    )
}

#[test]
fn the_arguments_are_handed_over_unparsed_and_the_exit_code_relayed() {
    // Whatever follows the command's name is the sibling's, an option the front does not know, `--help` and
    // `--version` among them.
    for args in [
        vec!["system.eadl", "plan.eadl"],
        vec![
            "--profile",
            "rt-static-up-v1",
            "--result",
            "r.json",
            "--",
            "system.eadl",
            "plan.eadl",
        ],
        vec!["--help"],
        vec!["--version"],
        vec!["-x", "--unknown=1"],
        vec![],
    ] {
        let fake = Fake::ending(Ok(Ended::Code(0)));
        let mut line = vec!["plan-check"];
        line.extend(&args);
        let (status, out, err) = front(&line, &fake);
        assert_eq!(status, Status::Ok, "{args:?}: {err}");
        assert_eq!(
            fake.asked.borrow().as_slice(),
            [(
                "archogen-plan-check".to_owned(),
                args.iter().map(|a| (*a).to_owned()).collect()
            )],
            "{args:?}"
        );
        assert_eq!(out, "the sibling's output\n");
        assert_eq!(err, "the sibling's findings\n");
    }
    for (code, status) in [
        (2, Status::Usage),
        (3, Status::StaleInput),
        (10, Status::InvalidDescription),
        (13, Status::InfeasibleConfiguration),
        (70, Status::ToolFailure),
    ] {
        let fake = Fake::ending(Ok(Ended::Code(code)));
        assert_eq!(
            front(&["plan-check", "d", "p"], &fake).0,
            status,
            "exit {code}"
        );
    }
}

#[test]
fn the_sibling_s_help_is_its_own() {
    let fake = Fake::ending(Ok(Ended::Code(0)));
    let (status, out, _) = front(&["help", "plan-check"], &fake);
    assert_eq!(status, Status::Ok);
    assert_eq!(
        fake.asked.borrow().as_slice(),
        [("archogen-plan-check".to_owned(), vec!["--help".to_owned()])]
    );
    assert_eq!(out, "the sibling's output\n");
    // And a command the front answers renders its help as before, asking no sibling.
    let fake = Fake::ending(Ok(Ended::Code(0)));
    let (status, out, _) = front(&["help", "check"], &fake);
    assert_eq!(status, Status::Ok);
    assert!(out.starts_with("archogen check — "), "{out}");
    assert!(fake.asked.borrow().is_empty());
}

#[test]
fn an_ending_the_vocabulary_does_not_hold_and_a_sibling_not_run_are_tool_failures() {
    for (ended, said) in [
        (
            Ended::Code(101),
            "exited with 101, which no status of the vocabulary has",
        ),
        (
            Ended::Code(1),
            "exited with 1, which no status of the vocabulary has",
        ),
        (
            Ended::Abnormally("signal: 9 (SIGKILL)".to_owned()),
            "ended without an exit code: signal: 9 (SIGKILL)",
        ),
    ] {
        let (status, _, err) = front(&["plan-check", "d", "p"], &Fake::ending(Ok(ended)));
        assert_eq!(status, Status::ToolFailure, "{said}");
        assert!(
            err.contains(&format!(
                "archogen: tool-failure: `archogen-plan-check` {said}"
            )),
            "{err}"
        );
    }
    let unlaunched = Unlaunched {
        message: "the install has no `archogen-plan-check`".to_owned(),
        repair: "install the toolchain again".to_owned(),
    };
    let (status, _, err) = front(&["plan-check", "d", "p"], &Fake::ending(Err(unlaunched)));
    assert_eq!(status, Status::ToolFailure);
    assert!(
        err.contains("archogen: tool-failure: the install has no `archogen-plan-check`\n  hint: install the toolchain again"),
        "{err}"
    );
}

#[test]
fn the_library_runs_no_program() {
    // `run`, the library's own entry, holds the launcher that runs nothing: only the executable's `main` hands over
    // one that does.
    for args in [vec!["plan-check", "d", "p"], vec!["help", "plan-check"]] {
        let (mut out, mut err) = (Vec::new(), Vec::new());
        let status = run(args.iter().copied(), &mut out, &mut err);
        let err = String::from_utf8(err).unwrap();
        assert_eq!(status, Status::ToolFailure, "{args:?}");
        assert!(
            err.contains(
                "`archogen-plan-check` is a program of its own, and this surface runs no program"
            ),
            "{err}"
        );
        assert!(out.is_empty(), "{args:?}");
    }
}

#[test]
fn the_front_s_table_holds_no_option_of_a_sibling_s() {
    // `M3.5.1`, *One table per command*: the sibling's options are its own, so the front can judge none of them.
    let siblings: Vec<_> = COMMANDS.iter().filter(|c| c.sibling.is_some()).collect();
    assert_eq!(
        siblings
            .iter()
            .map(|c| (c.name, c.sibling.unwrap()))
            .collect::<Vec<_>>(),
        [("plan-check", "archogen-plan-check")]
    );
    for command in siblings {
        assert!(
            command.options.is_empty(),
            "`{}` holds an option",
            command.name
        );
    }
}
