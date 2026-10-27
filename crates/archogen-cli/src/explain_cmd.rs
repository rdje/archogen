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
//! ⚠️ **Experimental.** It reads a description, as `archogen resolve` does; §10.2's form reads a build, which
//! `archogen build` writes — `M4.2`'s.

use std::io::Write;

use archogen_api::{Account, Explain, Limits, Response, Span};

use crate::check_cmd::{ask_with, report_not_judged};
use crate::cli::Parsed;
use crate::spec::Maturity;
use crate::status::Status;

/// Run `archogen explain`.
pub fn run(parsed: &Parsed, out: &mut dyn Write, err: &mut dyn Write) -> Status {
    let path = parsed
        .positionals
        .first()
        .expect("the parser guarantees the positional");
    let explain = Explain {
        system: parsed.value("system"),
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
    if explanation.accounts.is_empty() {
        if response.judged.is_none() {
            return report_not_judged(response, err);
        }
        let _ = write!(err, "{}", response.render_diagnostics());
        let _ = writeln!(
            err,
            "archogen: {}: {} diagnostic(s) in {path} — the check refused it, so no requirement was searched",
            response.status.slug(),
            response.diagnostics.len()
        );
        return response.status;
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
        return response.status;
    };
    let _ = writeln!(
        out,
        "{path}: the plan of system `{}` is the search's, unchecked: `archogen resolve` runs the configuration \
         checker over the plan it writes",
        plan.system
    );
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
