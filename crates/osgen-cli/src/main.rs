//! The `osgen` binary: a thin shell around [`osgen_cli::run`].
//!
//! Everything testable lives in the library. This file owns only the two things a library
//! cannot: reading the real argument vector and terminating the process with the status code.

use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    let mut stdout = std::io::stdout().lock();
    let mut stderr = std::io::stderr().lock();

    let status = osgen_cli::run(std::env::args().skip(1), &mut stdout, &mut stderr);

    // Flush explicitly: a locked stdout is flushed on drop, and a drop-time failure would be
    // discarded, turning a truncated report into a silent success.
    if stdout.flush().is_err() || stderr.flush().is_err() {
        return ExitCode::from(u8::try_from(osgen_cli::Status::ToolFailure.code()).unwrap_or(70));
    }

    ExitCode::from(u8::try_from(status.code()).unwrap_or(70))
}
