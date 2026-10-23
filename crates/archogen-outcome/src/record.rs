//! `archogen-result/1`: the record a checker writes, and the one reader that says what it holds (leaf `M3.5.1.1`).
//!
//! A checker — `archogen-plan-check`, `archogen-analyze` — is an executable of its own (`M3.5.1`). It reads files,
//! writes one record and exits with its status's code. The record says who wrote it, the program's name and sha256
//! (`TI-H14`); what it judged, the digests of the elaborated program and of the plan or the build; under which profile;
//! the status; and every finding, with its code, message, spans and repair. The front that ran the checker reads it
//! here, and so does the build that consumes it.
//!
//! ⭐ **One text per record.** [`Record::write`] writes a record one way: [`archogen_json::write`]'s, the members in a
//! fixed order, then one newline. [`Record::read`] reads with the strict reader and accepts a text only when writing
//! what it read gives back the same bytes, so whitespace, another member order, an escape that need not be one or a
//! number spelled another way is refused rather than interpreted. Two readers of one record cannot see two records.
//!
//! ⛔ **No refusal reads as acceptance.** This reader decides whether a plan is built, so a fault in it would turn a
//! refusal into an acceptance for every role that writes one: in the trust inventory's terms it is interpretation,
//! shared by every role. A record accepts only when its status is `ok`, its exit `0` and no finding an error; a record
//! whose status is anything else names its status's own exit and at least one error finding. So damage to one byte, a
//! truncation, or a status flipped alone, reads as a refusal or as nothing, never as acceptance (`tests/record.rs`),
//! and the front holds the process's own exit code to the record as a second channel (`M3.5.1.4`).
//!
//! It does no I/O: the caller hands it the bytes and the bound on how many.

use std::fmt;

use archogen_digest::Digest;
use archogen_json::{self as json, Number, Value};

use crate::status::{Status, VOCABULARY};
use crate::Version;

/// This format's identifier. A reader that meets another refuses the record rather than guess at it.
pub const FORMAT: &str = "archogen-result/1";

/// The program that wrote a record: its name, and the sha256 of its executable.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Producer {
    /// The executable's name, `archogen-plan-check` or `archogen-analyze`: lowercase letters, digits and `-`.
    pub name: String,
    /// The sha256 of the executable that ran.
    pub sha256: Digest,
}

/// What a record judged.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Subject {
    /// The sha256 of the elaborated program the description became.
    pub program: Digest,
    /// The plan or the build it was judged with.
    pub judged: Judged,
}

/// The second input of a judgement: a plan, for the configuration checker; a build, for the scheduling checker.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Judged {
    /// The sha256 of a build plan.
    Plan(Digest),
    /// The sha256 of a build.
    Build(Digest),
}

/// Whether a finding refuses.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// It refuses: a record holding one does not accept.
    Error,
    /// It informs, and refuses nothing.
    Warning,
}

impl Severity {
    const fn slug(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

/// A place a finding points at, as the wasm binding's response writes a label: the source's name, the 1-based line
/// and character column of the span's start, and what is said there. `0` for both is a place no source has.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    /// The name of the source, as the checker was given it.
    pub source: String,
    /// The 1-based line of the span's start.
    pub line: u32,
    /// The 1-based character column of the span's start.
    pub column: u32,
    /// What the label says about that place.
    pub message: String,
}

/// One finding: a diagnostic's code, severity, message, places and repair direction (`ROADMAP.md` §5.5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Finding {
    /// The diagnostic's code: lowercase letters, digits and `-`.
    pub code: String,
    /// Whether it refuses.
    pub severity: Severity,
    /// What was found.
    pub message: String,
    /// The place it is about.
    pub primary: Label,
    /// The other places it names.
    pub secondary: Vec<Label>,
    /// What to do about it.
    pub repair: String,
}

/// One checker's result: built only through [`Record::new`] or [`Record::read`], so a record that exists says one
/// consistent thing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    vocabulary: Version,
    producer: Producer,
    subject: Subject,
    profile: String,
    status: Status,
    findings: Vec<Finding>,
}

/// Why a text is not a record, or why a record cannot be made: each names what disagreed, for the `tool-failure` the
/// front reports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// Not a JSON text the strict reader admits, or longer than the bound.
    Json(json::Refusal),
    /// A JSON text, but not the one text [`Record::write`] writes for what it holds.
    NotCanonical,
    /// A format other than [`FORMAT`].
    Format(String),
    /// A member missing, unexpected or out of order, or a value of the wrong kind: the member's path.
    Shape(String),
    /// A vocabulary of another major, or one that cannot be read.
    Vocabulary(String),
    /// A status this vocabulary does not hold, one newer than the vocabulary the record names, or an exit that is
    /// not its status's code.
    Status(String),
    /// A value its member does not admit — a name, a digest, a severity, a line — and the member's path.
    Value(String),
    /// The status and the findings disagree: an acceptance holding an error, or a refusal holding none.
    Inconsistent(String),
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Json(refusal) => write!(f, "not a result record: {refusal}"),
            Self::NotCanonical => {
                write!(
                    f,
                    "not a result record: not the one text its writer writes for it"
                )
            }
            Self::Format(format) => write!(f, "a record in `{format}`, not `{FORMAT}`"),
            Self::Shape(what)
            | Self::Vocabulary(what)
            | Self::Status(what)
            | Self::Value(what)
            | Self::Inconsistent(what) => write!(f, "a result record that cannot be read: {what}"),
        }
    }
}

impl Record {
    /// A record in this vocabulary, [`VOCABULARY`]; refused when it would not read back as itself: a name or code
    /// outside its alphabet, an `ok` holding an error finding, or a refusal holding none.
    pub fn new(
        producer: Producer,
        subject: Subject,
        profile: String,
        status: Status,
        findings: Vec<Finding>,
    ) -> Result<Self, Refusal> {
        Self::checked(VOCABULARY, producer, subject, profile, status, findings)
    }

    fn checked(
        vocabulary: Version,
        producer: Producer,
        subject: Subject,
        profile: String,
        status: Status,
        findings: Vec<Finding>,
    ) -> Result<Self, Refusal> {
        name("producer.name", &producer.name)?;
        name("profile", &profile)?;
        for (index, finding) in findings.iter().enumerate() {
            name(&format!("findings[{index}].code"), &finding.code)?;
        }
        let errors = findings
            .iter()
            .filter(|f| f.severity == Severity::Error)
            .count();
        match (status, errors) {
            (Status::Ok, 0) => {}
            (Status::Ok, n) => {
                return Err(Refusal::Inconsistent(format!(
                    "status `ok` holds {n} error finding(s)"
                )))
            }
            (other, 0) => {
                return Err(Refusal::Inconsistent(format!(
                    "status `{}` holds no error finding",
                    other.slug()
                )))
            }
            _ => {}
        }
        Ok(Self {
            vocabulary,
            producer,
            subject,
            profile,
            status,
            findings,
        })
    }

    /// The vocabulary the record was written under.
    #[must_use]
    pub const fn vocabulary(&self) -> Version {
        self.vocabulary
    }

    /// The program that wrote it.
    #[must_use]
    pub const fn producer(&self) -> &Producer {
        &self.producer
    }

    /// What it judged.
    #[must_use]
    pub const fn subject(&self) -> &Subject {
        &self.subject
    }

    /// The profile it judged under.
    #[must_use]
    pub fn profile(&self) -> &str {
        &self.profile
    }

    /// Its status.
    #[must_use]
    pub const fn status(&self) -> Status {
        self.status
    }

    /// Its findings, in the order the checker wrote them.
    #[must_use]
    pub fn findings(&self) -> &[Finding] {
        &self.findings
    }

    /// Whether the record accepts what it judged: its status is `ok`, which [`Record::new`] admits only with no error
    /// finding.
    #[must_use]
    pub fn accepts(&self) -> bool {
        self.status == Status::Ok
    }

    /// The record's one text: compact JSON, the members in this order, and a newline.
    #[must_use]
    pub fn write(&self) -> String {
        let (judged, digest) = match self.subject.judged {
            Judged::Plan(digest) => ("plan", digest),
            Judged::Build(digest) => ("build", digest),
        };
        let value = object(vec![
            ("format", string(FORMAT)),
            ("vocabulary", string(&self.vocabulary.to_string())),
            (
                "producer",
                object(vec![
                    ("name", string(&self.producer.name)),
                    ("sha256", string(&self.producer.sha256.to_string())),
                ]),
            ),
            (
                "subject",
                object(vec![
                    ("program", string(&self.subject.program.to_string())),
                    (judged, string(&digest.to_string())),
                ]),
            ),
            ("profile", string(&self.profile)),
            ("status", string(self.status.slug())),
            ("exit", integer(i64::from(self.status.code()))),
            (
                "findings",
                Value::Array(self.findings.iter().map(finding).collect()),
            ),
        ]);
        let mut text = json::write(&value);
        text.push('\n');
        text
    }

    /// Read a record's text, at most `limit` bytes of it.
    pub fn read(bytes: &[u8], limit: usize) -> Result<Self, Refusal> {
        let text = bytes.strip_suffix(b"\n").ok_or(Refusal::NotCanonical)?;
        let value = json::read(text, limit).map_err(Refusal::Json)?;
        let [format, vocabulary, producer, subject, profile, status, exit, findings] = members(
            &value,
            "",
            &[
                "format",
                "vocabulary",
                "producer",
                "subject",
                "profile",
                "status",
                "exit",
                "findings",
            ],
        )?;
        let format = text_of(format, "format")?;
        if format != FORMAT {
            return Err(Refusal::Format(format.to_owned()));
        }
        let vocabulary = text_of(vocabulary, "vocabulary")?;
        let vocabulary = Version::parse(vocabulary)
            .ok_or_else(|| Refusal::Vocabulary(format!("`{vocabulary}` is not a version")))?;
        if vocabulary.major != VOCABULARY.major {
            return Err(Refusal::Vocabulary(format!(
                "written under vocabulary {vocabulary}, and this reader holds {VOCABULARY}"
            )));
        }
        let [name, sha256] = members(producer, "producer", &["name", "sha256"])?;
        let producer = Producer {
            name: text_of(name, "producer.name")?.to_owned(),
            sha256: digest(sha256, "producer.sha256")?,
        };
        let subject = match subject {
            Value::Object(fields) if fields.get(1).is_some_and(|(key, _)| key == "build") => {
                let [program, build] = members(subject, "subject", &["program", "build"])?;
                Subject {
                    program: digest(program, "subject.program")?,
                    judged: Judged::Build(digest(build, "subject.build")?),
                }
            }
            _ => {
                let [program, plan] = members(subject, "subject", &["program", "plan"])?;
                Subject {
                    program: digest(program, "subject.program")?,
                    judged: Judged::Plan(digest(plan, "subject.plan")?),
                }
            }
        };
        let profile = text_of(profile, "profile")?.to_owned();
        let slug = text_of(status, "status")?;
        let status = Status::from_slug(slug).ok_or_else(|| {
            Refusal::Status(format!(
                "`{slug}` is no status of vocabulary {VOCABULARY}, and the record names {vocabulary}"
            ))
        })?;
        if status.since() > vocabulary {
            return Err(Refusal::Status(format!(
                "`{slug}` entered vocabulary {}, and the record names {vocabulary}",
                status.since()
            )));
        }
        let exit = integer_of(exit, "exit")?;
        if exit != i64::from(status.code()) {
            return Err(Refusal::Status(format!(
                "exit {exit} is not `{slug}`'s, {}",
                status.code()
            )));
        }
        let Value::Array(items) = findings else {
            return Err(Refusal::Shape("`findings` is not an array".to_owned()));
        };
        let findings = items
            .iter()
            .enumerate()
            .map(|(index, item)| read_finding(item, &format!("findings[{index}]")))
            .collect::<Result<Vec<_>, _>>()?;
        let record = Self::checked(vocabulary, producer, subject, profile, status, findings)?;
        if record.write().as_bytes() != bytes {
            return Err(Refusal::NotCanonical);
        }
        Ok(record)
    }
}

/// Lowercase ASCII letters, digits and `-`, beginning with a letter: the alphabet of a program's, a profile's and a
/// diagnostic code's name.
fn name(path: &str, text: &str) -> Result<(), Refusal> {
    let mut chars = text.chars();
    let first = chars.next().is_some_and(|c| c.is_ascii_lowercase());
    if first && chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '-') {
        Ok(())
    } else {
        Err(Refusal::Value(format!(
            "`{path}` is `{text}`, not a name of lowercase letters, digits and `-`"
        )))
    }
}

fn object(members: Vec<(&str, Value)>) -> Value {
    Value::Object(
        members
            .into_iter()
            .map(|(key, value)| (key.to_owned(), value))
            .collect(),
    )
}

fn string(text: &str) -> Value {
    Value::String(text.to_owned())
}

fn integer(value: i64) -> Value {
    Value::Number(Number::from(value))
}

fn label(label: &Label) -> Value {
    object(vec![
        ("source", string(&label.source)),
        ("line", integer(i64::from(label.line))),
        ("column", integer(i64::from(label.column))),
        ("message", string(&label.message)),
    ])
}

fn finding(finding: &Finding) -> Value {
    object(vec![
        ("code", string(&finding.code)),
        ("severity", string(finding.severity.slug())),
        ("message", string(&finding.message)),
        ("primary", label(&finding.primary)),
        (
            "secondary",
            Value::Array(finding.secondary.iter().map(label).collect()),
        ),
        ("repair", string(&finding.repair)),
    ])
}

/// The values of an object's members, which must be exactly `names`, in that order.
fn members<'a, const N: usize>(
    value: &'a Value,
    path: &str,
    names: &[&str; N],
) -> Result<[&'a Value; N], Refusal> {
    let at = |name: &str| {
        if path.is_empty() {
            name.to_owned()
        } else {
            format!("{path}.{name}")
        }
    };
    let Value::Object(fields) = value else {
        return Err(Refusal::Shape(format!(
            "`{}` is not an object",
            if path.is_empty() { "the record" } else { path }
        )));
    };
    let found: Vec<&str> = fields.iter().map(|(key, _)| key.as_str()).collect();
    if found != names.as_slice() {
        let missing: Vec<String> = names
            .iter()
            .filter(|n| !found.contains(n))
            .map(|n| format!("`{}`", at(n)))
            .collect();
        let unexpected: Vec<String> = found
            .iter()
            .filter(|n| !names.contains(n))
            .map(|n| format!("`{}`", at(n)))
            .collect();
        return Err(Refusal::Shape(
            if missing.is_empty() && unexpected.is_empty() {
                format!(
                    "the members of `{}` are out of order",
                    if path.is_empty() { "the record" } else { path }
                )
            } else {
                format!(
                    "missing {}; unexpected {}",
                    if missing.is_empty() {
                        "none".to_owned()
                    } else {
                        missing.join(", ")
                    },
                    if unexpected.is_empty() {
                        "none".to_owned()
                    } else {
                        unexpected.join(", ")
                    }
                )
            },
        ));
    }
    Ok(std::array::from_fn(|index| &fields[index].1))
}

fn text_of<'a>(value: &'a Value, path: &str) -> Result<&'a str, Refusal> {
    value
        .as_str()
        .ok_or_else(|| Refusal::Shape(format!("`{path}` is not a string")))
}

fn integer_of(value: &Value, path: &str) -> Result<i64, Refusal> {
    match value {
        Value::Number(number) => number
            .as_i64()
            .ok_or_else(|| Refusal::Value(format!("`{path}` is not an integer"))),
        _ => Err(Refusal::Shape(format!("`{path}` is not a number"))),
    }
}

fn digest(value: &Value, path: &str) -> Result<Digest, Refusal> {
    let text = text_of(value, path)?;
    Digest::parse(text).ok_or_else(|| {
        Refusal::Value(format!(
            "`{path}` is `{text}`, not `sha256:` and 64 lowercase hexadecimal digits"
        ))
    })
}

fn position(value: &Value, path: &str) -> Result<u32, Refusal> {
    let number = integer_of(value, path)?;
    u32::try_from(number)
        .map_err(|_| Refusal::Value(format!("`{path}` is {number}, not a line or a column")))
}

fn read_label(value: &Value, path: &str) -> Result<Label, Refusal> {
    let [source, line, column, message] =
        members(value, path, &["source", "line", "column", "message"])?;
    Ok(Label {
        source: text_of(source, &format!("{path}.source"))?.to_owned(),
        line: position(line, &format!("{path}.line"))?,
        column: position(column, &format!("{path}.column"))?,
        message: text_of(message, &format!("{path}.message"))?.to_owned(),
    })
}

fn read_finding(value: &Value, path: &str) -> Result<Finding, Refusal> {
    let [code, severity, message, primary, secondary, repair] = members(
        value,
        path,
        &[
            "code",
            "severity",
            "message",
            "primary",
            "secondary",
            "repair",
        ],
    )?;
    let severity = match text_of(severity, &format!("{path}.severity"))? {
        "error" => Severity::Error,
        "warning" => Severity::Warning,
        other => {
            return Err(Refusal::Value(format!(
                "`{path}.severity` is `{other}`, not `error` or `warning`"
            )))
        }
    };
    let Value::Array(secondary) = secondary else {
        return Err(Refusal::Shape(format!(
            "`{path}.secondary` is not an array"
        )));
    };
    Ok(Finding {
        code: text_of(code, &format!("{path}.code"))?.to_owned(),
        severity,
        message: text_of(message, &format!("{path}.message"))?.to_owned(),
        primary: read_label(primary, &format!("{path}.primary"))?,
        secondary: secondary
            .iter()
            .enumerate()
            .map(|(index, item)| read_label(item, &format!("{path}.secondary[{index}]")))
            .collect::<Result<_, _>>()?,
        repair: text_of(repair, &format!("{path}.repair"))?.to_owned(),
    })
}
