//! `archogen explain` (leaf `M3.4.13.2`; `ROADMAP.md` §10.2): how one requirement of one system was realized, or why it
//! could not be — the engine API's `explain` ([`archogen_api::explain_with`]) printed. The command judges nothing
//! itself, so it can say nothing the API does not.
//!
//! For each requirement selected — `--requirement FACT`, narrowed by `--requester NAME` or `--task NAME` — it prints,
//! on standard output, one line naming it, where it is written, one line per provider of the system's hardware with
//! its answer, and then either the provider the plan binds and each other one's fate in its place, or why no provider
//! meets it, or that it was met alone while another requirement stopped the search. What stops the search is printed
//! on standard error as `archogen resolve` prints it, and the status is the resolution's.
//!
//! ⭐ **It claims no check.** The plan it explains is the search's; `archogen resolve` runs the configuration checker
//! over the plan it writes, and this command says so rather than run one.
//!
//! ⭐ **It reads a build too**, §10.2's form (leaf `M4.2.3.2`): given a directory `archogen build` wrote, it explains the
//! description that build records, and holds the search's plan for it equal to the plan the build holds, by its digest,
//! and to what the build's checker record judged. A build that holds another plan is `stale-input`, and no account of it
//! is printed: the search can only speak for the plan it writes. Reading a directory is the command line's alone: the
//! engine API and the MCP server take a description as text (`ROADMAP.md` §10.4).
//!
//! ⚠️ **Experimental.** It explains the search over a description's own blocks and platforms, as `archogen resolve` does.

use std::io::Write;
use std::path::{Component, Path};

use archogen_api::{Account, Explain, Limits, Response, Span};
use archogen_digest::Digest;
use archogen_emit::PROVENANCE_FORMAT;
use archogen_json::Value;
use archogen_outcome::record::{Judged, Record};

use crate::check_cmd::{ask_with, report_not_judged};
use crate::cli::Parsed;
use crate::consume::{PLAN_CHECKER, RECORD};
use crate::spec::Maturity;
use crate::status::Status;

/// The most of a build's provenance or record the command reads.
const RECORD_LIMIT: usize = 1 << 24;

/// What a build records of what it was made from (`docs/decisions/decision_hosted-build.md` §1).
struct Built {
    /// Its description, inside the build.
    description: String,
    /// The system it was built for.
    system: String,
    /// The assignment the search wrote.
    assignment: String,
    /// The checker's record for it.
    record: Record,
}

impl Built {
    /// What `dir` records, or what it is instead: the repair is always to give a description or a build.
    fn read(dir: &str) -> Result<Self, String> {
        let root = Path::new(dir);
        let bytes = std::fs::read(root.join("provenance.json"))
            .map_err(|error| format!("{dir} is a directory, and its `provenance.json`: {error}"))?;
        let provenance = archogen_json::read(&bytes, RECORD_LIMIT)
            .map_err(|refusal| format!("{dir}/provenance.json does not read: {refusal}"))?;
        let text = |name: &str| {
            provenance
                .get(name)
                .and_then(Value::as_str)
                .map(str::to_owned)
                .ok_or_else(|| format!("{dir}/provenance.json names no `{name}`"))
        };
        let format = text("format")?;
        if format != PROVENANCE_FORMAT {
            return Err(format!(
                "{dir} is a build of `{format}`, and this engine reads `{PROVENANCE_FORMAT}`: build it again"
            ));
        }
        // A provenance entry is text anyone may have edited, so it stays inside the build it names.
        let name = text("description")?;
        if !Path::new(&name)
            .components()
            .all(|part| matches!(part, Component::Normal(_)))
        {
            return Err(format!(
                "{dir}/provenance.json names the description `{name}`, which lies outside the build"
            ));
        }
        let assignment = std::fs::read_to_string(root.join("assignment.eadl"))
            .map_err(|error| format!("{dir}/assignment.eadl: {error}"))?;
        let record =
            std::fs::read(root.join(RECORD)).map_err(|error| format!("{dir}/{RECORD}: {error}"))?;
        let record = Record::read(&record, RECORD_LIMIT)
            .map_err(|refusal| format!("{dir}/{RECORD} does not read: {refusal}"))?;
        Ok(Self {
            description: root.join("description").join(name).display().to_string(),
            system: text("system")?,
            assignment,
            record,
        })
    }

    /// Why the search's plan for this build's description is not the plan the build holds, if it is not.
    fn stale(&self, plan: &str, dir: &str) -> Option<String> {
        let held = Digest::of(self.assignment.as_bytes());
        let searched = Digest::of(plan.as_bytes());
        if held != searched {
            return Some(format!(
                "the build {dir} holds the plan {held}, and its description resolves to {searched}: \
                 it was edited, or an engine that searches otherwise built it"
            ));
        }
        if !self.record.accepts() || self.record.subject().judged != Judged::Plan(held) {
            return Some(format!(
                "the build {dir} holds the plan {held}, and its `{RECORD}` does not accept that plan"
            ));
        }
        None
    }
}

/// Run `archogen explain`.
pub fn run(parsed: &Parsed, out: &mut dyn Write, err: &mut dyn Write) -> Status {
    let given = parsed
        .positionals
        .first()
        .expect("the parser guarantees the positional");
    // A directory is a build: there is no other directory a description can be.
    let built = if Path::new(given).is_dir() {
        match Built::read(given) {
            Ok(built) => Some(built),
            Err(message) => {
                let _ = writeln!(err, "archogen: {}: {message}", Status::Usage.slug());
                let _ = writeln!(
                    err,
                    "  hint: give the path of an eADL description, or a directory `archogen build` wrote"
                );
                return Status::Usage;
            }
        }
    } else {
        None
    };
    let path = built.as_ref().map_or(given, |built| &built.description);
    let explain = Explain {
        system: parsed
            .value("system")
            .or_else(|| built.as_ref().map(|built| built.system.as_str())),
        fact: parsed
            .value("requirement")
            .expect("the parser guarantees `--requirement` is required"),
        requester: parsed.value("requester"),
        task: parsed.value("task"),
    };
    // No budget, as `check` asks: the command line trusts the files it was given.
    let explanation = match ask_with(
        path,
        parsed.value("profile"),
        |_| None,
        err,
        |request| archogen_api::explain_with(request, &explain, Limits::NONE),
    ) {
        Ok(explanation) => explanation,
        Err(status) => return status,
    };
    let response = &explanation.response;
    // A build's description that no longer gives the build's plan: the search cannot speak for the plan it holds.
    if let (Some(built), Some(plan)) = (&built, &explanation.plan) {
        if let Some(message) = built.stale(&plan.text, given) {
            let _ = writeln!(err, "archogen: {}: {message}", Status::StaleInput.slug());
            let _ = writeln!(err, "  hint: run `archogen build` again");
            return Status::StaleInput;
        }
    }
    // What it answers of a build whose description gives no plan today: stale, whatever else stands in the way.
    let answer = |status: Status, err: &mut dyn Write| {
        if built.is_none() {
            return status;
        }
        let _ = writeln!(
            err,
            "archogen: {}: the build {given} holds a plan, and its description resolves to none today",
            Status::StaleInput.slug()
        );
        Status::StaleInput
    };
    if explanation.accounts.is_empty() {
        if response.judged.is_none() {
            let status = report_not_judged(response, err);
            return answer(status, err);
        }
        let _ = write!(err, "{}", response.render_diagnostics());
        let _ = writeln!(
            err,
            "archogen: {}: {} diagnostic(s) in {path} — the check refused it, so no requirement was searched",
            response.status.slug(),
            response.diagnostics.len()
        );
        return answer(response.status, err);
    }
    let system = explanation.plan.as_ref().map(|plan| plan.system.as_str());
    for account in &explanation.accounts {
        print_account(account, response, system.is_some(), out);
    }
    let Some(plan) = &explanation.plan else {
        let _ = write!(err, "{}", response.render_diagnostics());
        for note in &response.notes {
            let _ = writeln!(err, "archogen: {}: {note}", response.status.slug());
        }
        return answer(response.status, err);
    };
    if built.is_some() {
        let _ = writeln!(
            out,
            "{given}: the plan of system `{}` is the one this build holds, {}, which `{PLAN_CHECKER}` accepted",
            plan.system,
            Digest::of(plan.text.as_bytes())
        );
    } else {
        let _ = writeln!(
            out,
            "{path}: the plan of system `{}` is the search's, unchecked: `archogen resolve` runs the configuration \
             checker over the plan it writes",
            plan.system
        );
    }
    crate::resolve_cmd::print_owed(plan, out);
    if let Maturity::Experimental {
        scope,
        completed_by,
    } = parsed.spec().maturity
    {
        let _ = writeln!(out, "  experimental: {scope}; completed by {completed_by}");
    }
    Status::Ok
}

/// One account: the requirement, where it is written, each provider's answer, and what became of it.
fn print_account(account: &Account, response: &Response, planned: bool, out: &mut dyn Write) {
    let _ = writeln!(
        out,
        "`{}` required by `{}`",
        account.fact, account.requester
    );
    for clause in &account.clauses {
        let _ = writeln!(out, "  written at {}", at(response, *clause));
    }
    for (provider, answer) in &account.answers {
        let _ = writeln!(out, "  `{provider}` {}", answer.says());
    }
    match (&account.bound, &account.unmet) {
        (Some(bound), _) => {
            let _ = writeln!(out, "  bound to `{bound}` by the search's plan");
            for (other, refusal) in &account.instead {
                match refusal {
                    Some(diagnostic) => {
                        let _ = writeln!(
                            out,
                            "  `{other}` in its place: {} — {}",
                            diagnostic.code, diagnostic.message
                        );
                    }
                    None => {
                        let _ = writeln!(
                            out,
                            "  `{other}` in its place: the joint rules accept it too; the search tries `{bound}` first"
                        );
                    }
                }
            }
        }
        (None, Some(unmet)) => {
            let _ = writeln!(out, "  no provider meets it: {} (below)", unmet.code);
        }
        (None, None) if !planned => {
            let met: Vec<String> = account
                .answers
                .iter()
                .filter(|(_, answer)| *answer == archogen_api::Answer::Satisfied)
                .map(|(provider, _)| format!("`{provider}`"))
                .collect();
            let _ = writeln!(
                out,
                "  met alone by {}; no plan was written: another requirement stops the search (below)",
                met.join(", ")
            );
        }
        (None, None) => {}
    }
}

/// `span` as `FILE:LINE:COLUMN`.
fn at(response: &Response, span: Span) -> String {
    response.sources.get(span.source).map_or_else(
        || "an unknown place".to_owned(),
        |source| {
            let position = source.position(span.start);
            format!("{}:{}:{}", source.name, position.line, position.column)
        },
    )
}
