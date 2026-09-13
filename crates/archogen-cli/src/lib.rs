//! `archogen` — the command-line entry point of the archogen toolchain.
//!
//! This crate owns the *shape* of the user contract: the command surface of `ROADMAP.md` §10.2,
//! the outcome vocabulary of §5.5 with stable exit codes, and the refusal wording every command
//! inherits. Each command that is not yet built to its §10.2 contract names the task-tree leaf
//! that will complete it, so a gap is a signpost rather than a dead end.
//!
//! No semantics live here. Each command arm is a thin translation between the command line and
//! the crate that does the work — [`check_cmd`] runs `eadl_model::check`, [`build_cmd`] runs the
//! experimental S0 realization in `archogen_s0` — and its job is to map a result onto the exit
//! code contract without adding a judgement of its own. `run` returns a [`Status`] and writes to
//! the caller's streams, so the whole surface is testable without a process.

pub mod build_cmd;
pub mod check_cmd;
pub mod cli;
pub mod spec;
pub mod status;

use std::io::Write;

pub use cli::{Invocation, Parsed, Refusal};
pub use status::Status;

/// The version reported by `archogen --version`.
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Run one invocation, writing normal output to `out` and diagnostics to `err`.
///
/// Returns the [`Status`] the process should exit with. Errors from the writers are reported
/// as [`Status::ToolFailure`]: a toolchain that cannot deliver its own output has not
/// produced a result, and §5.5 forbids presenting that as a valid system.
pub fn run<A, S>(args: A, out: &mut dyn Write, err: &mut dyn Write) -> Status
where
    A: IntoIterator<Item = S>,
    S: Into<String>,
{
    match cli::parse(args) {
        Ok(Invocation::Help(None)) => emit(out, &cli::help_overview()),
        Ok(Invocation::Help(Some(name))) => emit(out, &cli::help_command(name)),
        Ok(Invocation::Version) => emit(out, &format!("archogen {VERSION}\n")),
        Ok(Invocation::Run(parsed)) => dispatch(&parsed, out, err),
        Err(refusal) => report(err, &refusal),
    }
}

fn emit(out: &mut dyn Write, text: &str) -> Status {
    if write!(out, "{text}").is_err() {
        return Status::ToolFailure;
    }
    Status::Ok
}

/// Print a refusal in the shape every archogen diagnostic uses: what happened, then what to
/// do about it. §5.5 requires a concrete repair direction on every diagnostic.
fn report(err: &mut dyn Write, refusal: &Refusal) -> Status {
    let written = writeln!(
        err,
        "archogen: {}: {}",
        refusal.status.slug(),
        refusal.message
    )
    .and_then(|()| writeln!(err, "  hint: {}", refusal.repair));
    if written.is_err() {
        return Status::ToolFailure;
    }
    refusal.status
}

/// Route a well-formed invocation to its implementation.
///
/// As each command lands, it gains an arm here and its [`spec::Maturity`] changes in the same
/// edit — which is what keeps `archogen --help` from advertising a gap that is closed, or a
/// completeness that is not there.
fn dispatch(parsed: &Parsed, out: &mut dyn Write, err: &mut dyn Write) -> Status {
    let spec = parsed.spec();
    match spec.name {
        "check" => return check_cmd::run(parsed, out, err),
        "build" => return build_cmd::run(parsed, out, err),
        _ => {}
    }
    let Some(owner) = spec.maturity.owner() else {
        // Unreachable: a command marked `Built` without an arm above would land here. It must
        // not look like success — silently exiting 0 for a command that did nothing is the one
        // outcome worse than refusing.
        let _ = writeln!(
            err,
            "archogen: {}: `{}` is marked implemented but has no implementation",
            Status::ToolFailure.slug(),
            spec.name
        );
        return Status::ToolFailure;
    };

    let refusal = Refusal {
        status: Status::Unimplemented,
        message: format!("`archogen {}` is not implemented yet", spec.name),
        repair: format!(
            "it is part of the interface target in ROADMAP.md §10.2; the work is tracked by task-tree leaf {owner} (docs/TASK_TREE.md)"
        ),
    };
    report(err, &refusal)
}

#[cfg(test)]
mod tests {
    use super::{run, Status};

    /// Run an invocation and return `(status, stdout, stderr)`.
    fn invoke(args: &[&str]) -> (Status, String, String) {
        let mut out: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();
        let status = run(args.iter().map(|s| (*s).to_string()), &mut out, &mut err);
        (
            status,
            String::from_utf8(out).expect("utf-8 stdout"),
            String::from_utf8(err).expect("utf-8 stderr"),
        )
    }

    #[test]
    fn no_arguments_prints_the_overview_and_succeeds() {
        let (status, out, err) = invoke(&[]);
        assert_eq!(status, Status::Ok);
        assert!(out.contains("USAGE:"), "{out}");
        assert!(err.is_empty(), "{err}");
    }

    #[test]
    fn the_overview_lists_every_command_of_the_interface_target() {
        let (_, out, _) = invoke(&["--help"]);
        for name in [
            "check", "resolve", "build", "analyze", "verify", "explain", "replay",
        ] {
            assert!(out.contains(name), "`{name}` missing from help:\n{out}");
        }
    }

    #[test]
    fn the_overview_publishes_the_exit_code_contract() {
        let (_, out, _) = invoke(&["--help"]);
        for status in Status::ALL {
            assert!(
                out.contains(status.slug()),
                "`{}` missing from the exit-code table:\n{out}",
                status.slug()
            );
        }
    }

    #[test]
    fn version_prints_the_package_version() {
        let (status, out, _) = invoke(&["--version"]);
        assert_eq!(status, Status::Ok);
        assert_eq!(out.trim(), format!("archogen {}", super::VERSION));
    }

    #[test]
    fn an_unbuilt_command_exits_unimplemented_and_names_its_leaf() {
        // `resolve` is still unbuilt. This arm moved off `check` when leaf M1.8 built it — the
        // property under test is the routing of an unbuilt command, not any particular command
        // being unbuilt, so it follows the surface rather than pinning it.
        let (status, out, err) = invoke(&["resolve", "examples/x.eadl", "--out", "plan.json"]);
        assert_eq!(status, Status::Unimplemented);
        assert_eq!(status.code(), 20);
        assert!(
            out.is_empty(),
            "an unbuilt command must print nothing to stdout: {out}"
        );
        assert!(err.contains("unimplemented"), "{err}");
        assert!(
            err.contains("M3.4"),
            "the refusal must name the owning leaf: {err}"
        );
    }

    #[test]
    fn a_built_command_no_longer_appears_as_unimplemented_in_help() {
        // The help text is rendered from the same table the parser validates against, so
        // building a command and announcing it are one change rather than two.
        let (_, out, _) = invoke(&["--help"]);
        let check_line = out
            .lines()
            .find(|line| line.trim_start().starts_with("check "))
            .expect("check is listed");
        assert!(
            !check_line.contains("unimplemented"),
            "`check` is built but still advertised as unimplemented: {check_line}"
        );
        let resolve_line = out
            .lines()
            .find(|line| line.trim_start().starts_with("resolve "))
            .expect("resolve is listed");
        assert!(
            resolve_line.contains("unimplemented"),
            "`resolve` is unbuilt and should say so: {resolve_line}"
        );
    }

    #[test]
    fn check_on_a_missing_file_is_a_usage_error_with_a_hint() {
        let (status, _, err) = invoke(&["check", "no/such/file.eadl"]);
        assert_eq!(status, Status::Usage);
        assert!(err.contains("cannot read"), "{err}");
        assert!(err.contains("hint:"), "{err}");
    }

    #[test]
    fn check_against_an_unknown_profile_is_refused_before_anything_is_read() {
        // A description checked against a profile nobody supports has not been checked.
        let (status, _, err) =
            invoke(&["check", "no/such/file.eadl", "--profile", "rt-fantasy-v9"]);
        assert_eq!(status, Status::UnsupportedProfile);
        assert!(err.contains("not a supported profile"), "{err}");
        assert!(err.contains("rt-static-up-v1"), "{err}");
    }

    #[test]
    fn an_unknown_command_is_a_usage_error_listing_the_known_ones() {
        let (status, _, err) = invoke(&["generate"]);
        assert_eq!(status, Status::Usage);
        assert_eq!(status.code(), 2);
        assert!(err.contains("unknown command `generate`"), "{err}");
        assert!(err.contains("check"), "{err}");
    }

    #[test]
    fn an_unknown_option_names_what_the_command_does_accept() {
        let (status, _, err) = invoke(&["check", "system.eadl", "--strict"]);
        assert_eq!(status, Status::Usage);
        assert!(err.contains("unknown option `--strict`"), "{err}");
        assert!(err.contains("--profile"), "{err}");
    }

    #[test]
    fn a_missing_required_option_is_refused_with_the_usage_line() {
        let (status, _, err) = invoke(&["resolve", "system.eadl"]);
        assert_eq!(status, Status::Usage);
        assert!(err.contains("requires `--out`"), "{err}");
        assert!(err.contains("archogen resolve <DESCRIPTION>"), "{err}");
    }

    #[test]
    fn a_missing_positional_is_refused_with_its_help() {
        let (status, _, err) = invoke(&["replay"]);
        assert_eq!(status, Status::Usage);
        assert!(err.contains("missing <MANIFEST>"), "{err}");
    }

    #[test]
    fn an_option_that_swallowed_its_value_is_refused() {
        let (status, _, err) = invoke(&["analyze", "build/x", "--property"]);
        assert_eq!(status, Status::Usage);
        assert!(err.contains("`--property` requires a value"), "{err}");
    }

    #[test]
    fn a_flag_given_a_value_is_refused() {
        let (status, _, err) = invoke(&["build", "s.eadl", "--out", "d", "--locked=yes"]);
        assert_eq!(status, Status::Usage);
        assert!(err.contains("takes no value"), "{err}");
    }

    #[test]
    fn inline_and_separated_option_values_are_equivalent() {
        let separated = super::cli::parse(["analyze", "build/x", "--property", "deadlines"]);
        let inline = super::cli::parse(["analyze", "build/x", "--property=deadlines"]);
        assert_eq!(separated, inline);
    }

    #[test]
    fn a_repeated_option_is_refused_rather_than_silently_last_wins() {
        let (status, _, err) = invoke(&["analyze", "b", "--property", "a", "--property", "b"]);
        assert_eq!(status, Status::Usage);
        assert!(err.contains("more than once"), "{err}");
    }

    #[test]
    fn command_help_is_reachable_without_satisfying_the_command() {
        // A user asking what `resolve` needs must not be told to supply --out first.
        let (status, out, err) = invoke(&["resolve", "--help"]);
        assert_eq!(status, Status::Ok);
        assert!(out.contains("archogen resolve <DESCRIPTION>"), "{out}");
        assert!(
            out.contains("M3.4"),
            "command help must name the owning leaf:\n{out}"
        );
        assert!(err.is_empty(), "{err}");
    }

    #[test]
    fn help_subcommand_matches_the_help_flag() {
        let (_, via_word, _) = invoke(&["help", "build"]);
        let (_, via_flag, _) = invoke(&["build", "--help"]);
        assert_eq!(via_word, via_flag);
    }

    #[test]
    fn help_for_an_unknown_command_is_a_usage_error() {
        let (status, _, err) = invoke(&["help", "frobnicate"]);
        assert_eq!(status, Status::Usage);
        assert!(err.contains("unknown command"), "{err}");
    }

    #[test]
    fn every_diagnostic_carries_a_repair_direction() {
        // ROADMAP.md §5.5: every diagnostic carries "a concrete repair direction".
        for args in [
            vec!["generate"],
            vec!["check", "s.eadl", "--strict"],
            vec!["resolve", "s.eadl"],
            vec!["replay"],
            vec!["analyze", "b", "--property"],
        ] {
            let (status, _, err) = invoke(&args);
            assert_ne!(status, Status::Ok, "{args:?}");
            assert!(
                err.contains("hint:"),
                "no repair direction for {args:?}:\n{err}"
            );
        }
    }

    #[test]
    fn a_failing_writer_is_reported_as_a_tool_failure_not_as_success() {
        // §5.5: a tool failure is never reported as a valid system.
        struct Broken;
        impl std::io::Write for Broken {
            fn write(&mut self, _: &[u8]) -> std::io::Result<usize> {
                Err(std::io::Error::other("stream closed"))
            }
            fn flush(&mut self) -> std::io::Result<()> {
                Ok(())
            }
        }
        let mut out = Broken;
        let mut err: Vec<u8> = Vec::new();
        let status = run(["--help"].map(String::from), &mut out, &mut err);
        assert_eq!(status, Status::ToolFailure);
        assert_eq!(status.code(), 70);
    }
}
