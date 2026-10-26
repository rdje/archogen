//! One judgement as the result record says it (leaf `M3.5.1.6.4`): every check run over a description's text and a
//! plan's bytes ([`check::plan_file`]), and the verdict made `archogen-result/1` — its producer the program that ran,
//! its subject the program the description became and the sha256 of the plan's bytes. The executable's record and its
//! MCP tool's answer are both made here, so the two faces write one record for one judgement.

use archogen_digest::Digest;
use archogen_outcome::record::{Judged, Producer, Record, Refusal, Subject};
use archogen_outcome::Status;
use eadl_front::module::ModuleSource;
use eadl_front::{SourceError, SourceMap};
use eadl_model::profile::Profile;

use crate::assignment::program_text;
use crate::check;

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
    /// The description is larger than a span can address: nothing was judged.
    TooLarge(SourceError),
    /// The verdict does not make a record: the checker's own failure.
    Record(Refusal),
}

/// Judge `inputs`, the record naming `producer` as the program that ran.
///
/// # Errors
///
/// [`Unjudged`]: a description too large to read, or a verdict that does not make a record.
pub fn judge(producer: Producer, inputs: &Inputs<'_>) -> Result<Record, Unjudged> {
    let mut sources = SourceMap::new();
    let id = sources
        .add(inputs.description_name, inputs.description)
        .map_err(Unjudged::TooLarge)?;
    let plan_text = String::from_utf8_lossy(inputs.plan);
    let (program, verdict) = check::plan_file(
        &mut sources,
        id,
        inputs.modules,
        inputs.profile,
        inputs.plan_name,
        &plan_text,
    );
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
