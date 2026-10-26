//! Running a sibling program, as git runs its sibling commands (leaf `M3.5.1.4.1`; `M3.5.1`'s design, *The front's
//! place in the code*): what the front asks of one, how it ended, and why the front could not run it.
//!
//! ⛔ **The library runs nothing.** [`Launch`] is only the request. The one implementation that starts a process is
//! the `archogen` executable's own `sibling.rs`, declared by its `main.rs` and nowhere else, the one file
//! `NO-SUBPROCESS` admits; `main` hands it to [`crate::run_with_input`]. [`crate::run`], the MCP server and every other
//! surface that drives archogen in memory hold [`NoLaunch`], so none of them can make it start a program, and a test
//! hands a fake.

use std::io::Write;

use archogen_digest::Digest;

use crate::status::Status;

/// How a sibling's process ended.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Ended {
    /// It exited with this code.
    Code(i32),
    /// It ended with no exit code — a signal stopped it — as the platform describes the ending.
    Abnormally(String),
}

/// A sibling the front ran: how its process ended, and the sha256 of the program it verified before running it — the
/// producer a record it wrote must name.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Launched {
    /// How the process ended.
    pub ended: Ended,
    /// The sha256 of the file that ran, as the install's manifest gives it and the front checked it.
    pub sha256: Digest,
}

/// Why the front did not run a sibling: what it could not vouch for or could not do, and the repair.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unlaunched {
    /// What went wrong, one line.
    pub message: String,
    /// A concrete repair direction (`ROADMAP.md` §5.5 requires one on every diagnostic).
    pub repair: String,
}

/// What the front asks of a sibling program.
pub trait Launch {
    /// Run `program` — an executable an install placed beside the front, by its file name — with `args` handed over
    /// as given, its standard input empty, its standard output relayed to `out` and its standard error to `err`.
    ///
    /// # Errors
    ///
    /// [`Unlaunched`] when the front cannot vouch for the program or cannot start it.
    fn launch(
        &self,
        program: &str,
        args: &[String],
        out: &mut dyn Write,
        err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched>;
}

/// The launcher every surface but the `archogen` executable holds: it runs nothing.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoLaunch;

impl Launch for NoLaunch {
    fn launch(
        &self,
        program: &str,
        _args: &[String],
        _out: &mut dyn Write,
        _err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched> {
        Err(Unlaunched {
            message: format!("`{program}` is a program of its own, and this surface runs no program"),
            repair: format!(
                "run the `archogen` executable of an install, which runs `{program}` beside it, or run `{program}` itself"
            ),
        })
    }
}

/// The status a sibling's ending answers with: its exit code, when the status vocabulary holds it.
///
/// # Errors
///
/// Why the ending is no status — a code outside the vocabulary, a panic's `101` among them, or no code at all — for
/// the front to answer `tool-failure` with.
pub fn status_of(ended: &Ended) -> Result<Status, String> {
    match ended {
        Ended::Code(code) => Status::ALL
            .iter()
            .copied()
            .find(|status| status.code() == *code)
            .ok_or_else(|| format!("exited with {code}, which no status of the vocabulary has")),
        Ended::Abnormally(how) => Err(format!("ended without an exit code: {how}")),
    }
}
