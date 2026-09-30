//! The request's framing (`docs/decisions/decision_wasm-binding.md` §4): length-prefixed UTF-8 fields, so the
//! module needs no parser for a format only its own loader writes.

use core::fmt;

/// A request, read from its framing.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Framed {
    /// How spans and notes name the description.
    pub name: String,
    /// The description's text.
    pub text: String,
    /// The profile's identifier; `None` for the API's default, which the framing writes as an empty field.
    pub profile: Option<String>,
    /// Each module's name and text, in the order the request gives them.
    pub modules: Vec<(String, String)>,
}

/// Why a request could not be read: the field, the byte offset it starts at, and what is wrong there.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FramingError {
    /// The field, as the record's §4 names it: `format`, `name`, `text`, `profile`, `module count`,
    /// `module 2 name`, `module 2 text`, or `end`.
    pub field: String,
    /// The offset in the request at which that field starts.
    pub offset: usize,
    /// What is wrong.
    pub problem: String,
}

impl fmt::Display for FramingError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "the request's {} field, at byte {}, {}",
            self.field, self.offset, self.problem
        )
    }
}

/// A cursor over the request's bytes.
struct Reader<'a> {
    bytes: &'a [u8],
    at: usize,
}

impl<'a> Reader<'a> {
    fn fail(&self, field: &str, offset: usize, problem: impl Into<String>) -> FramingError {
        FramingError {
            field: field.to_string(),
            offset,
            problem: problem.into(),
        }
    }

    /// A 4-byte little-endian integer.
    fn integer(&mut self, field: &str) -> Result<usize, FramingError> {
        let start = self.at;
        let Some(bytes) = self.bytes.get(start..start + 4) else {
            return Err(self.fail(
                field,
                start,
                format!(
                    "needs a 4-byte length, and {} byte(s) remain",
                    self.bytes.len() - start
                ),
            ));
        };
        self.at += 4;
        let value = u32::from_le_bytes([bytes[0], bytes[1], bytes[2], bytes[3]]);
        // A `u32` fits in `usize` on every target this workspace builds for; one that did not would not fit in
        // the request either.
        Ok(usize::try_from(value).unwrap_or(usize::MAX))
    }

    /// A length, then that many bytes of UTF-8.
    fn field(&mut self, field: &str) -> Result<&'a str, FramingError> {
        let start = self.at;
        let length = self.integer(field)?;
        let body = self.at;
        let Some(bytes) = body
            .checked_add(length)
            .and_then(|end| self.bytes.get(body..end))
        else {
            return Err(self.fail(
                field,
                start,
                format!(
                    "gives a length of {length} byte(s), and {} remain",
                    self.bytes.len() - body
                ),
            ));
        };
        self.at = body + length;
        core::str::from_utf8(bytes)
            .map_err(|error| self.fail(field, start, format!("is not UTF-8 ({error})")))
    }
}

/// Read a request. Everything the record's §4 requires is checked, and nothing may follow the last module.
///
/// # Errors
///
/// A [`FramingError`] naming the first field that breaks the framing.
pub fn parse(bytes: &[u8]) -> Result<Framed, FramingError> {
    let mut reader = Reader { bytes, at: 0 };
    let format = reader.field("format")?;
    if format != crate::REQUEST_FORMAT {
        return Err(reader.fail(
            "format",
            0,
            format!(
                "is `{format}`, and this module reads `{}`",
                crate::REQUEST_FORMAT
            ),
        ));
    }
    let name = reader.field("name")?.to_string();
    let text = reader.field("text")?.to_string();
    let profile = reader.field("profile")?;
    let profile = (!profile.is_empty()).then(|| profile.to_string());
    let count = reader.integer("module count")?;
    // Nothing is allocated for the count before the fields are read, so a count larger than the request can hold
    // fails on the first field that is not there.
    let mut modules: Vec<(String, String)> = Vec::new();
    for index in 1..=count {
        let name_field = format!("module {index} name");
        let name_at = reader.at;
        let module = reader.field(&name_field)?.to_string();
        if modules.iter().any(|(seen, _)| *seen == module) {
            return Err(reader.fail(
                &name_field,
                name_at,
                format!("names `{module}`, which an earlier module already names"),
            ));
        }
        let module_text = reader.field(&format!("module {index} text"))?.to_string();
        modules.push((module, module_text));
    }
    if reader.at != bytes.len() {
        return Err(reader.fail(
            "end",
            reader.at,
            format!(
                "is followed by {} byte(s), and nothing may follow the last module",
                bytes.len() - reader.at
            ),
        ));
    }
    Ok(Framed {
        name,
        text,
        profile,
        modules,
    })
}

/// Write a request in the framing [`parse`] reads: what the loader does, for tests and for other Rust hosts.
#[must_use]
pub fn frame(request: &Framed) -> Vec<u8> {
    fn field(out: &mut Vec<u8>, text: &str) {
        let length = u32::try_from(text.len()).expect("a field is shorter than 4 GiB");
        out.extend_from_slice(&length.to_le_bytes());
        out.extend_from_slice(text.as_bytes());
    }
    let mut out = Vec::new();
    field(&mut out, crate::REQUEST_FORMAT);
    field(&mut out, &request.name);
    field(&mut out, &request.text);
    field(&mut out, request.profile.as_deref().unwrap_or(""));
    let count = u32::try_from(request.modules.len()).expect("fewer than 2^32 modules");
    out.extend_from_slice(&count.to_le_bytes());
    for (name, text) in &request.modules {
        field(&mut out, name);
        field(&mut out, text);
    }
    out
}
