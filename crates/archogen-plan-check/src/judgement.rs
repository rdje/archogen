//! One judgement as the result record says it (leaf `M3.5.1.6.4`): every check run over a description's text and a
//! plan's bytes ([`check::plan_file`]), and the verdict made `archogen-result/1` — its producer the program that ran,
//! its subject the program the description became and the sha256 of the plan's bytes. The executable's record and its
//! MCP tool's answer are both made here, so the two faces write one record for one judgement — and the browser
//! module's (`M3.5.7.1`), which with the tool takes texts rather than files, under one budget ([`BYTES`]) and one
//! name for the plan ([`PLAN_NAME`]).

use archogen_digest::Digest;
use archogen_outcome::record::{Judged, Producer, Record, Refusal, Subject};
use archogen_outcome::Status;
use eadl_front::module::ModuleSource;
use eadl_front::{SourceError, SourceMap};
use eadl_model::profile::{Profile, RT_STATIC_UP_V1};

use crate::assignment::program_text;
use crate::check;

/// The most text one judgement given as texts may bring: the description, every module given and the plan, counted
/// together, by the MCP tool and the browser module alike. The generator's budget for a description and its modules is
/// 1 MiB (`archogen_api::DEFAULT_BYTES`); this one counts the plan too, and is four times that. The checks' work is
/// linear in it, as the engine's is. The checker depends on no engine API, so the budget is its own.
pub const BYTES: usize = 4 * 1024 * 1024;

/// How findings name a plan given as text.
pub const PLAN_NAME: &str = "plan.json";

/// The texts' bytes together, or why they are past [`BYTES`].
///
/// # Errors
///
/// What to change: the texts' size and the budget.
pub fn budget<'a>(texts: impl IntoIterator<Item = &'a str>) -> Result<usize, String> {
    let bytes = texts
        .into_iter()
        .fold(0usize, |bytes, text| bytes.saturating_add(text.len()));
    if bytes > BYTES {
        return Err(format!(
            "the texts are {bytes} bytes together, past this checker's budget of {BYTES}"
        ));
    }
    Ok(bytes)
}

/// The profile `id` names, [`RT_STATIC_UP_V1`] when it names none.
///
/// # Errors
///
/// A profile this checker does not have, and the one it has.
pub fn profile(id: Option<&str>) -> Result<&'static Profile, String> {
    match id {
        None => Ok(&RT_STATIC_UP_V1),
        Some(id) if id == RT_STATIC_UP_V1.id => Ok(&RT_STATIC_UP_V1),
        Some(other) => Err(format!(
            "the profile `{other}` is not one this checker has; it has `{}`",
            RT_STATIC_UP_V1.id
        )),
    }
}

/// What one judgement reads.
#[derive(Clone, Copy)]
pub struct Inputs<'a> {
    /// How findings name the description.
    pub description_name: &'a str,
    /// The description's text.
    pub description: &'a str,
    /// Where the description's imports are found.
    pub modules: &'a dyn ModuleSource,
    /// The profile the plan is judged under.
    pub profile: &'a Profile,
    /// How findings name the plan.
    pub plan_name: &'a str,
    /// The plan's bytes as written: the subject names their sha256, and a plan that is no UTF-8 text does not read as
    /// a record, which check (1) says.
    pub plan: &'a [u8],
}

/// Why a judgement made no record.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Unjudged {
    /// The description, or a target's file given beside it, is larger than a span can address, the error naming which:
    /// nothing was judged.
    TooLarge(SourceError),
    /// The verdict does not make a record: the checker's own failure.
    Record(Refusal),
}

/// A target's file given beside a judgement (leaf `M4.1.7.5.4.4`; `DC-H2`): how findings name it, and its text.
#[derive(Clone, Copy)]
pub struct TargetFile<'a> {
    /// How findings name the target's file.
    pub name: &'a str,
    /// Its text.
    pub text: &'a str,
}

/// Judge `inputs`, the record naming `producer` as the program that ran.
///
/// # Errors
///
/// [`Unjudged`]: a description too large to read, or a verdict that does not make a record.
pub fn judge(producer: Producer, inputs: &Inputs<'_>) -> Result<Record, Unjudged> {
    judge_on(producer, inputs, None)
}

/// Judge `inputs` on `target` where one is given ([`check::plan_file_on_target`]), else as [`judge`] does
/// ([`check::plan_file`]), the record naming `producer` as the program that ran. The record's format is the same either
/// way: its subject is the program the description became and the plan's bytes, whose `(target NAME "SHA256")` check (1)
/// holds to the target's file.
///
/// # Errors
///
/// [`Unjudged`]: a description or a target's file too large to read, the error naming which, or a verdict that does not
/// make a record.
pub fn judge_on(
    producer: Producer,
    inputs: &Inputs<'_>,
    target: Option<TargetFile<'_>>,
) -> Result<Record, Unjudged> {
    let mut sources = SourceMap::new();
    let id = sources
        .add(inputs.description_name, inputs.description)
        .map_err(Unjudged::TooLarge)?;
    let plan_text = String::from_utf8_lossy(inputs.plan);
    let (program, verdict) = match target {
        None => check::plan_file(
            &mut sources,
            id,
            inputs.modules,
            inputs.profile,
            inputs.plan_name,
            &plan_text,
        ),
        Some(target) => {
            let target = sources
                .add(target.name, target.text)
                .map_err(Unjudged::TooLarge)?;
            check::plan_file_on_target(
                &mut sources,
                id,
                target,
                inputs.modules,
                inputs.profile,
                inputs.plan_name,
                &plan_text,
            )
        }
    };
    let (status, findings) = match verdict {
        Ok(()) => (Status::Ok, Vec::new()),
        Err(refused) => (refused.status, refused.findings),
    };
    Record::new(
        producer,
        Subject {
            program: Digest::of(program_text(&program).as_bytes()),
            judged: Judged::Plan(Digest::of(inputs.plan)),
        },
        inputs.profile.id.to_owned(),
        status,
        findings,
    )
    .map_err(Unjudged::Record)
}
