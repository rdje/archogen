//! The `archogen` binary: a thin shell around [`archogen_cli::run_with_input`].
//!
//! Everything testable lives in the library. This file owns only the three things a library
//! cannot: reading the real argument vector and standard input, running a sibling program, and terminating the process
//! with the status code. The launcher, [`sibling`], is declared here and nowhere else, so no other surface can start a
//! program (leaf `M3.5.1.4.1`).

use std::io::Write;
use std::process::ExitCode;

mod sibling;

fn main() -> ExitCode {
    let mut stdout = std::io::stdout().lock();
    let mut stderr = std::io::stderr().lock();

    // `std::env::args` panics on an argument that is no text, and exit 101 is a code no status has.
    let args: Result<Vec<String>, _> = std::env::args_os()
        .skip(1)
        .map(std::ffi::OsString::into_string)
        .collect();
    let Ok(args) = args else {
        let usage = archogen_cli::Status::Usage;
        let _ = writeln!(
            stderr,
            "archogen: {}: an argument is not UTF-8",
            usage.slug()
        );
        let _ = writeln!(stderr, "  hint: give paths and names this tool can print");
        return ExitCode::from(u8::try_from(usage.code()).unwrap_or(2));
    };
    let mut stdin = std::io::stdin().lock();
    let status = archogen_cli::run_with_input(
        args,
        &mut stdin,
        &sibling::Installed,
        &mut stdout,
        &mut stderr,
    );

    // Flush explicitly: a locked stdout is flushed on drop, and a drop-time failure would be
    // discarded, turning a truncated report into a silent success.
    if stdout.flush().is_err() || stderr.flush().is_err() {
        let failure = archogen_cli::unwritten(&mut stderr);
        return ExitCode::from(u8::try_from(failure.code()).unwrap_or(70));
    }

    ExitCode::from(u8::try_from(status.code()).unwrap_or(70))
}
