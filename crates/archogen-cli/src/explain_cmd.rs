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
//! ⭐ **It reads a build too**, §10.2's form (leaf `M4.2.3.2`): given a directory `archogen build` wrote, it holds the
//! build's files to the digests it recorded, explains the description it records, and holds the search's plan equal to
//! the plan the build holds and to what its checker's record accepted. A build that fails any of that is `stale-input`,
//! and no account is printed: the search can only speak for the plan it writes. Reading a directory is the command
//! line's alone: the engine API and the MCP server take a description as text (`ROADMAP.md` §10.4).
//!
//! ⚠️ **Experimental.** It explains the search over a description's own blocks and platforms, as `archogen resolve` does.

use std::io::Write;
use std::path::Path;

use archogen_api::{Account, Explain, Limits, Plan, Response, Span};
use archogen_digest::Digest;
use archogen_emit::ASSIGNMENT_FILE;
use archogen_outcome::record::{Judged, Record};

use crate::built::{Built, NotAsWritten, Unbuilt};
use crate::check_cmd::{ask_with, report_not_judged};
use crate::cli::Parsed;
use crate::consume::{PLAN_CHECKER, RECORD};
use crate::spec::Maturity;
use crate::status::Status;

/// The most of a build's record the command reads.
use crate::consume::RECORD_LIMIT;

/// A build, as `archogen explain` speaks for it.
struct Asked {
    /// What it records of itself.
    built: Built,
    /// Its description, inside it.
    description: String,
    /// The assignment the search wrote.
    assignment: String,
    /// The checker's record for it.
    record: Record,
}

/// Why a directory cannot be spoken for.
enum Unasked {
    /// It is no build this engine reads: the repair is to give a description or a build.
    NoBuild(String),
    /// It is a build of another format, which only a new build answers.
    OtherFormat(String),
    /// It is one, and not as it was written.
    Altered(String),
    /// It is one, and what it holds could not be read, so nothing is said of whether it changed.
    Unread(String),
}

impl Asked {
    /// What `dir` records, each file held to its digest before anything reads it.
    fn read(dir: &str) -> Result<Self, Unasked> {
        let root = Path::new(dir);
        let built = Built::read(root).map_err(|unbuilt| match unbuilt {
            Unbuilt::OtherFormat(said) => Unasked::OtherFormat(said),
            Unbuilt::Closed(said) => Unasked::Unread(format!(
                "the build {dir} cannot be read, so nothing says whether it is as it was written: {said}"
            )),
            Unbuilt::Other(said) => Unasked::NoBuild(said),
        })?;
        match built.edited(root).or_else(|| built.foreign(root)) {
            Some(NotAsWritten::Changed(why)) => {
                return Err(Unasked::Altered(format!(
                    "the build {dir} is not as it was written: {why}"
                )))
            }
            Some(NotAsWritten::Unread(why)) => {
                return Err(Unasked::Unread(format!(
                    "the build {dir} cannot be read, so nothing says whether it is as it was written: {why}"
                )))
            }
            None => {}
        }
        let no_build = |message: String| Unasked::NoBuild(message);
        let assignment = std::fs::read_to_string(root.join(ASSIGNMENT_FILE))
            .map_err(|error| no_build(format!("{dir}/{ASSIGNMENT_FILE}: {error}")))?;
        let record = std::fs::read(root.join(RECORD))
            .map_err(|error| no_build(format!("{dir}/{RECORD}: {error}")))?;
        let record = Record::read(&record, RECORD_LIMIT)
            .map_err(|refusal| no_build(format!("{dir}/{RECORD} does not read: {refusal}")))?;
        Ok(Self {
            description: root
                .join("description")
                .join(&built.description)
                .display()
                .to_string(),
            built,
            assignment,
            record,
        })
    }

    /// Why the search's plan for this build's description is not the plan the build holds, or not what its checker
    /// accepted, if either is so. As far as digests go, which catch an edit and never a forger (`crate::built`).
    fn stale(&self, plan: &Plan, dir: &str) -> Option<String> {
        let held = Digest::of(self.assignment.as_bytes());
        let searched = Digest::of(plan.text.as_bytes());
        if held != searched {
            return Some(format!(
                "the build {dir} holds the assignment {held}, and its description resolves to {searched}: an engine \
                 that searches otherwise built it"
            ));
        }
        let producer = self.record.producer();
        let subject = self.record.subject();
        let program = Digest::parse(&format!("sha256:{}", plan.program));
        let accepted = self.record.accepts()
            && subject.judged == Judged::Plan(held)
            && Some(subject.program) == program
            && producer.name == PLAN_CHECKER
            && producer.sha256 == self.built.checker;
        (!accepted).then(|| {
            format!(
                "the build {dir} holds the assignment {held}, and its `{RECORD}` does not accept that assignment from \
                 `{PLAN_CHECKER}` {}",
                self.built.checker
            )
        })
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
        match Asked::read(given) {
            Ok(built) => Some(built),
            Err(Unasked::Altered(message)) => {
                let _ = writeln!(err, "archogen: {}: {message}", Status::StaleInput.slug());
                // A build is not put over one that has changed, which it did not write as it stands.
                let _ = writeln!(
                    err,
                    "  hint: run `archogen build` again, into a new directory"
                );
                return Status::StaleInput;
            }
            Err(Unasked::Unread(message)) => {
                let _ = writeln!(err, "archogen: {}: {message}", Status::Usage.slug());
                let _ = writeln!(err, "  hint: open what it could not read, or ask from nearer it if its path is long");
                return Status::Usage;
            }
            Err(Unasked::OtherFormat(message)) => {
                let _ = writeln!(err, "archogen: {}: {message}", Status::Usage.slug());
                let _ = writeln!(
                    err,
                    "  hint: run `archogen build` with this engine, into a new directory"
                );
                return Status::Usage;
            }
            Err(Unasked::NoBuild(message)) => {
                let _ = writeln!(err, "archogen: {}: {message}", Status::Usage.slug());
                let _ = writeln!(
                    err,
                    "  hint: give the path of an eADL description, or a directory `archogen build` wrote and this user can read"
                );
                return Status::Usage;
            }
        }
    } else {
        None
    };
    if let Some(built) = &built {
        if let Some(asked) = parsed
            .value("system")
            .filter(|asked| *asked != built.built.system)
        {
            let _ = writeln!(
                err,
                "archogen: {}: {given} is a build of system `{}`, not `{asked}`",
                Status::Usage.slug(),
                built.built.system
            );
            let _ = writeln!(
                err,
                "  hint: leave out `--system`: a build holds one system"
            );
            return Status::Usage;
        }
    }
    let path = built.as_ref().map_or(given, |built| &built.description);
    let explain = Explain {
        system: parsed
            .value("system")
            .or_else(|| built.as_ref().map(|built| built.built.system.as_str())),
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
    // The search can only speak for the plan it writes, so a build it does not write gets no account. A request that
    // misses, a requirement nobody writes, is `usage` of a healthy build, and is answered as of any description.
    if let Some(built) = &built {
        let stale = match &explanation.plan {
            Some(plan) => built.stale(plan, given),
            None if response.status == Status::Usage => None,
            None => Some(format!(
                "the build {given} holds an assignment, and its description resolves to none today"
            )),
        };
        if let Some(message) = stale {
            let _ = write!(err, "{}", response.render_diagnostics());
            let _ = writeln!(err, "archogen: {}: {message}", Status::StaleInput.slug());
            let _ = writeln!(err, "  hint: run `archogen build` again");
            return Status::StaleInput;
        }
    }
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
    if built.is_some() {
        let _ = writeln!(
            out,
            "{given}: the search's plan for system `{}` is the one this build holds, assignment {}, which \
             `{PLAN_CHECKER}` accepted",
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
