//! `archogen build` — the experimental S0 generation path.
//!
//! ⚠️ **This is not the `ROADMAP.md` §10.2 `build` command.** That one generates "a complete
//! system and its simulator"; this one runs the S0 prototype (`crates/archogen-s0`): one fixed
//! engine-owned realization, periodic releases, the hosted playground, no analysis and no
//! assurance. §12 S0 requires that output to be marked experimental, so the command's own
//! summary line says so, every generated file says so, and
//! [`crate::spec::Maturity::Experimental`] makes `archogen --help` say so too.
//!
//! # The order of operations is the contract
//!
//! 1. Resolve the profile. A system built under a profile nobody supports has not been built.
//! 2. Refuse `--locked`, which the S0 path cannot honor (see below).
//! 3. Ask the **engine API** — the same `archogen_api::check` that `archogen check` asks, with the
//!    same module path. A description that does not check does not build, and the build reports the
//!    *check's* verdict rather than inventing one of its own.
//! 4. Interpret into an S0 plan, which is where a valid, in-profile description can still be
//!    refused for want of an engine realization.
//! 5. Emit.
//!
//! Step 3 is what keeps the two commands honest about each other: there is exactly one frontend,
//! so `build` can never accept something `check` rejects.

use std::io::Write;
use std::path::Path;

use archogen_s0::{emit, interpret};

use crate::check_cmd::{ask, report_not_judged};
use crate::cli::Parsed;
use crate::status::Status;

/// Run `archogen build`.
#[allow(clippy::too_many_lines)]
pub fn run(parsed: &Parsed, out: &mut dyn Write, err: &mut dyn Write) -> Status {
    let path = parsed
        .positionals
        .first()
        .expect("the parser guarantees the positional");
    let out_dir = parsed
        .value("out")
        .expect("the parser guarantees `--out` is required");

    let refuse_locked = |err: &mut dyn Write| -> Option<Status> {
        // ⛔ `--locked` is refused rather than accepted-and-ignored. Its contract is "fail on any
        // missing or mismatched locked input", and the S0 path has no lock data to compare against —
        // so honoring it is impossible and *appearing* to honor it is the worse outcome: a user who
        // asked for a reproducible build would get an ordinary one that claimed to be locked. §10.3
        // makes lock data a build output, and leaf `M4.1` owns producing it.
        // S0-ASSUMPTION: no-lock-data — the S0 path emits none, so `--locked` cannot be honored and
        // is refused rather than accepted and ignored. `M4.1` produces the lock data.
        if !parsed.flag("locked") {
            return None;
        }
        let _ = writeln!(
            err,
            "archogen: {}: `--locked` is not implemented on the experimental S0 build path",
            Status::Unimplemented.slug()
        );
        let _ = writeln!(
            err,
            "  hint: the S0 path emits no lock data, so there is nothing to check a locked \
             build against — and a build that accepted the flag silently would be an unlocked \
             build wearing a locked build's label. Lock data is ROADMAP.md §10.3 and task-tree \
             leaf M4.1. Re-run without `--locked` for an experimental build"
        );
        Some(Status::Unimplemented)
    };
    // The whole of `archogen check`, through the one engine API: a description that does not check does not
    // build, and the two commands cannot disagree about a description or a module tree.
    let response = match ask(path, parsed.value("profile"), refuse_locked, err) {
        Ok(response) => response,
        Err(status) => return status,
    };
    let Some(judged) = &response.judged else {
        return report_not_judged(&response, err);
    };
    if !response.is_ok() {
        let _ = write!(err, "{}", response.render_diagnostics());
        let _ = writeln!(
            err,
            "archogen: {}: {} diagnostic(s) in {path} — nothing was generated",
            response.status.slug(),
            response.diagnostics.len()
        );
        return response.status;
    }

    let plan = match interpret(&judged.declarations, judged.description) {
        Ok(plan) => plan,
        Err(diagnostic) => {
            // ⭐ The verdict comes from the diagnostic's own code through the one accessor every consumer
            // shares, and its default is `invalid-description`: a code that is not a §5.5 verdict slug is a
            // statement about the description, not about the toolchain. This line used to default to
            // `Status::ToolFailure` instead, so a build refusing `(period 10 parsec)` exited **70** and
            // told the author to file a bug about the toolchain for a symbol they mistyped — leaf
            // `M1.28.1`.
            let status = Status::from_verdict(eadl_front::Verdict::of_code(diagnostic.code));
            let _ = write!(err, "{}", diagnostic.render(&response.sources));
            let _ = writeln!(
                err,
                "archogen: {}: the S0 path cannot realize {path}, though the check pipeline accepted \
                 it — nothing was generated",
                status.slug()
            );
            return status;
        }
    };

    let dir = Path::new(out_dir);
    let generated = match emit(&plan, &response.sources, dir) {
        Ok(generated) => generated,
        Err(error) => {
            let _ = writeln!(
                err,
                "archogen: {}: cannot write the generated crate into {out_dir}: {error}",
                Status::ToolFailure.slug()
            );
            let _ = writeln!(
                err,
                "  hint: give `--out` a writable directory on the same filesystem as the project"
            );
            return Status::ToolFailure;
        }
    };

    let _ = writeln!(
        out,
        "{path}: generated `{}` into {}",
        plan.name,
        generated.dir.display()
    );
    let _ = writeln!(
        out,
        "  {} file(s): {}",
        generated.files.len(),
        generated.files.join(", ")
    );
    let _ = writeln!(
        out,
        "  {} task(s), hyperperiod {} ms",
        plan.tasks.len(),
        plan.horizon_ms
    );
    // §12 S0: "Mark the output experimental, with no claim of OS completeness or real-time
    // assurance." Said here as well as in every generated file, because the terminal is where a
    // user forms their impression of what they just got.
    let _ = writeln!(
        out,
        "  experimental: this is the S0 path — one fixed realization, periodic releases, the \
         hosted playground."
    );
    let _ = writeln!(
        out,
        "    It is not a complete system and carries no timing or assurance claim. See \
         `archogen help build`."
    );
    let _ = writeln!(
        out,
        "  next: cargo run --manifest-path {}/Cargo.toml",
        generated.dir.display()
    );
    Status::Ok
}
