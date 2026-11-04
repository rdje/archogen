//! `archogen resolve` (leaf `M3.4.4`; `ROADMAP.md` §10.2): one system of a description searched for a plan by the
//! engine API's `resolve`, the plan written, and — unless the user writes `--unchecked`, a downgrade never made
//! quietly — the configuration checker run over it, the command's status its verdict (findings §16 (a), `M3.5.1`'s
//! design).
//!
//! # The order of operations is the contract
//!
//! 1. Refuse `--locked` before reading anything: there is no lock data to honor it with until `M4.7`, and a plan that
//!    accepted the flag silently would be an unlocked one wearing a locked one's label.
//! 2. Ask the **engine API**'s `resolve` ([`archogen_api::resolve_with`]), the search alone, with `unchecked` set — it
//!    checks the description as `archogen check` does, and answers the check's verdict for one it does not accept,
//!    the search's refusal, or the plan. The command judges nothing itself, so it can do nothing the API cannot.
//! 3. The plan written whole at `--out`, beside it and renamed into place.
//! 4. Unless `--unchecked`: [`consume::check_plan`] runs `archogen-plan-check` over it, its record in `<PLAN>.check/`,
//!    and the status is the checker's verdict — `ok`, or `tool-failure` for a plan the checker refuses. Running a
//!    program is the command line's alone (§10.4), which is why this step is here and not in the API.
//!
//! ⚠️ **Experimental.** It searches the description's own blocks and platforms: a requirement only a realization meets
//! waits on the catalog's providers (`M3.7`), and the MCP server's `resolve` is `M3.4.6`'s.

use std::io::Write;
use std::path::{Path, PathBuf};

use archogen_api::{Limits, Plan, Resolve};
use archogen_digest::Digest;

use crate::check_cmd::{ask_with, report_not_judged};
use crate::cli::Parsed;
use crate::consume::{self, Expected};
use crate::launch::Launch;
use crate::spec::Maturity;
use crate::status::Status;

/// Run `archogen resolve`.
pub fn run(
    parsed: &Parsed,
    launcher: &dyn Launch,
    out: &mut dyn Write,
    err: &mut dyn Write,
) -> Status {
    let path = parsed
        .positionals
        .first()
        .expect("the parser guarantees the positional");
    let plan_path = PathBuf::from(
        parsed
            .value("out")
            .expect("the parser guarantees `--out` is required"),
    );
    let refuse_locked = |err: &mut dyn Write| -> Option<Status> {
        if !parsed.flag("locked") {
            return None;
        }
        let _ = writeln!(
            err,
            "archogen: {}: `--locked` is not implemented for `archogen resolve`",
            Status::Unimplemented.slug()
        );
        let _ = writeln!(
            err,
            "  hint: there is no lock data yet to check a resolution against, and a plan that accepted the flag \
             silently would be an unlocked one wearing a locked one's label. Lock data is ROADMAP.md §10.3 and \
             task-tree leaf M4.7. Re-run without `--locked`"
        );
        Some(Status::Unimplemented)
    };
    let resolve = Resolve {
        system: parsed.value("system"),
        unchecked: true,
    };
    // No budget, as `check` asks: the command line trusts the files it was given.
    let resolution = match ask_with(
        path,
        parsed.value("profile"),
        refuse_locked,
        err,
        |request| archogen_api::resolve_with(request, &resolve, Limits::NONE),
    ) {
        Ok(resolution) => resolution,
        Err(status) => return status,
    };
    let response = &resolution.response;
    let Some(plan) = &resolution.plan else {
        if response.judged.is_none() {
            return report_not_judged(response, err);
        }
        let _ = write!(err, "{}", response.render_diagnostics());
        let _ = writeln!(
            err,
            "archogen: {}: {} diagnostic(s) in {path} — nothing was resolved",
            response.status.slug(),
            response.diagnostics.len()
        );
        return response.status;
    };
    let profile = response
        .judged
        .as_ref()
        .map(|judged| judged.profile)
        .expect("a plan is written for a judged description");
    if let Err(error) = write_whole(&plan_path, &plan.text) {
        let _ = writeln!(
            err,
            "archogen: {}: cannot write the plan {}: {error}",
            Status::ToolFailure.slug(),
            plan_path.display()
        );
        return Status::ToolFailure;
    }
    let _ = writeln!(
        out,
        "{path}: resolved system `{}` against profile `{profile}` into {} ({} binding(s))",
        plan.system,
        plan_path.display(),
        plan.bindings
    );
    print_owed(plan, out);
    // What it searches over today, as the command table states it, so `--help` and this line cannot differ.
    if let Maturity::Experimental {
        scope,
        completed_by,
    } = parsed.spec().maturity
    {
        let _ = writeln!(out, "  experimental: {scope}; completed by {completed_by}");
    }
    if parsed.flag("unchecked") {
        let _ = writeln!(
            err,
            "archogen: unchecked: `--unchecked` — the plan was not checked by `{}`; nothing independent judged it",
            consume::PLAN_CHECKER
        );
        return Status::Ok;
    }
    let Some(program) = Digest::parse(&format!("sha256:{}", plan.program)) else {
        let _ = writeln!(
            err,
            "archogen: {}: the engine named the program by `{}`, which is no digest",
            Status::ToolFailure.slug(),
            plan.program
        );
        return Status::ToolFailure;
    };
    let mut dir = plan_path.clone().into_os_string();
    dir.push(".check");
    let dir = PathBuf::from(dir);
    let expected = Expected {
        description: Path::new(path),
        plan: &plan_path,
        program,
        plan_sha256: Digest::of(plan.text.as_bytes()),
        profile,
    };
    match consume::check_plan(launcher, &expected, &dir, err).map(|checked| checked.record) {
        Ok(record) if record.status() == Status::Ok => {
            let _ = writeln!(
                out,
                "  checked by `{}`: accepted ({})",
                consume::PLAN_CHECKER,
                dir.join(consume::RECORD).display()
            );
            Status::Ok
        }
        Ok(record) => {
            let _ = writeln!(
                err,
                "archogen: {}: `{}` refuses the plan the search wrote ({} finding(s), {}): the search and the \
                 checker disagree, and the plan is not one to build on",
                record.status().slug(),
                consume::PLAN_CHECKER,
                record.findings().len(),
                dir.join(consume::RECORD).display()
            );
            let _ = writeln!(
                err,
                "  hint: report it, with the checker's findings above: one of the two programs misreads this \
                 description"
            );
            record.status()
        }
        Err(refusal) => {
            let _ = writeln!(
                err,
                "archogen: {}: {}",
                refusal.status.slug(),
                refusal.message
            );
            let _ = writeln!(err, "  hint: {}", refusal.repair);
            refusal.status
        }
    }
}

/// What `plan` owes for the engine adapters it binds, if any: the plan is conditional until each is discharged, each
/// line the instance, what it owes, its name and its meaning (`docs/specs/adapters/decision_engine-adapters.md` §9).
/// `archogen explain` prints the same lines for the plan it explains.
pub(crate) fn print_owed(plan: &Plan, out: &mut dyn Write) {
    if plan.obligations.is_empty() {
        return;
    }
    let _ = writeln!(
        out,
        "  conditional: it binds engine adapters, and holds once each obligation below is discharged and each cost \
         charged"
    );
    for owed in &plan.obligations {
        let _ = writeln!(
            out,
            "    {} {} `{}`: {}",
            owed.instance,
            owed.owes.as_str(),
            owed.name,
            owed.doc
        );
    }
}

/// Write `text` at `path` whole: its directory made, the text written beside it and renamed into place.
fn write_whole(path: &Path, text: &str) -> std::io::Result<()> {
    if let Some(parent) = path.parent().filter(|p| !p.as_os_str().is_empty()) {
        std::fs::create_dir_all(parent)?;
    }
    let mut part = path.as_os_str().to_owned();
    part.push(".part");
    let part = PathBuf::from(part);
    std::fs::write(&part, text)?;
    std::fs::rename(&part, path)
}
