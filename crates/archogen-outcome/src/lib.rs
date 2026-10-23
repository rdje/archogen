//! The outcome vocabulary and the result record: no role's, and doing no I/O (leaf `M3.5.1.1`).
//!
//! Every role of the toolchain says how an invocation ended in one vocabulary, [`Status`]: the engine API's response,
//! the command line's exit code, and the result record a checker writes, [`record::Record`]. The generator and both
//! checkers compile this package, so the trust inventory sees it as a shared item of each pair it joins — the
//! vocabulary, the record's writer, and the record's reader, which decides whether a plan is built (`M3.5.1`). It
//! holds no role's judgement: what a language verdict is in this vocabulary stays with the engine API, so nothing of
//! the language's reader is compiled here.
//!
//! The contract is the book's: [*What is versioned, and what changing it costs*](../../versions.html).
//! This reference is generated from the code and documents it, item by item (`docs/book/api.crates`).

#![deny(missing_docs)]

pub mod record;
mod status;

use std::fmt;

pub use status::{Status, VOCABULARY};

/// A major and a minor version: the engine API's (`archogen_api::VERSION`) and the status vocabulary's
/// ([`VOCABULARY`]). It moved here from the engine API with the vocabulary (leaf `M3.5.1.1`), which re-exports it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Version {
    /// Changes when a promise of the previous major is broken.
    pub major: u32,
    /// Changes when something is added.
    pub minor: u32,
}

impl Version {
    /// Read `<major>.<minor>`, each in decimal without a leading zero: the one spelling [`Version`]'s `Display`
    /// writes, so a version read is the version written.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        let (major, minor) = text.split_once('.')?;
        Some(Self {
            major: decimal(major)?,
            minor: decimal(minor)?,
        })
    }
}

/// A decimal number without a sign, a leading zero or anything else around it.
fn decimal(text: &str) -> Option<u32> {
    let digits = !text.is_empty() && text.bytes().all(|b| b.is_ascii_digit());
    if digits && (text == "0" || !text.starts_with('0')) {
        text.parse().ok()
    } else {
        None
    }
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

#[cfg(test)]
mod tests {
    use super::Version;

    #[test]
    fn a_version_reads_as_it_is_written_and_no_other_way() {
        for version in [
            Version { major: 1, minor: 0 },
            Version {
                major: 1,
                minor: 12,
            },
            Version { major: 0, minor: 3 },
        ] {
            assert_eq!(Version::parse(&version.to_string()), Some(version));
        }
        for text in [
            "1", "1.", ".1", "01.0", "1.01", "+1.0", "1.0.0", "1 .0", "1.x", "",
        ] {
            assert_eq!(Version::parse(text), None, "{text:?}");
        }
    }

    #[test]
    fn versions_order_by_major_then_minor() {
        assert!(Version { major: 1, minor: 9 } < Version { major: 2, minor: 0 });
        assert!(Version { major: 1, minor: 0 } < Version { major: 1, minor: 1 });
    }
}
