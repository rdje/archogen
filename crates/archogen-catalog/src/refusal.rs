//! Refusals: why a catalog does not load (the record's §11).
//!
//! > Every refusal names its record, its field and the field's source location, and has one code.
//!
//! A refusal to load is a defect in the engine's own knowledge, not a verdict on anyone's description. So a
//! [`Refusal`] is never softened into a warning: the catalog either loads or it does not, and the code says which
//! rule it broke.

use core::fmt;

/// The one code a refusal carries, as the record's §11 names it.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Code {
    /// A reader diagnostic; a byte outside §1's set; a decoded string outside printable ASCII.
    Read,
    /// A file under `catalog/` that is neither a record in a namespace directory nor the lock; a shallow
    /// repository or a grafts file.
    Layout,
    /// Not exactly one `catalog-record` form; a field or subform missing, unknown, duplicated or out of order, a
    /// code fact's locators excepted; two identical locators of one fact, or several on a fact not about code; a
    /// second `assembly` declaration, or one naming no package (§14.2); a repeated name; a decimal.
    Shape,
    /// The id breaks §1's grammar, differs from the file stem, or is used twice.
    Id,
    /// A version or a requirement breaks §2's form.
    Version,
    /// A field's value is outside what §2 admits: an empty text, an unknown name from a closed set, a malformed
    /// fact or cost, a negative integer, a safety factor that does not pad, and the rest of §11's list; an unknown
    /// architecture, a known port fact with no locator into declared assembly, and a record reaching declared
    /// assembly or stating such a fact off the dialect's targets (§14.2); the port's statement, by one record's text
    /// and under a selection (§14.4).
    Field,
    /// A locator outside what §2 admits for its facet, or missing where §2 requires one; a code fact's locator that
    /// is not a `code` locator (§14.2).
    Locator,
    /// §3's package rules, manifest dialect and workspace rule, and §4's path rules, an `assembly` declaration's
    /// packages among them (§14.2).
    Source,
    /// An unresolved, unmatched or repeated reference to another record, or a cycle; a dependency on a check-passing
    /// convention that does not admit the record's profiles and targets (§14.4).
    Dependency,
    /// §5's review rules.
    Review,
    /// §6's production namespace rules.
    Production,
    /// §9: a facet's current version, or a review, has no lock line.
    LockMissing,
    /// §9: a facet changed without its version moving.
    LockUnbumped,
    /// §9: a facet's version below one the lock holds.
    LockDowngrade,
    /// §9: the lock's first line, a line of no known form, a review's line against its form, a waiver.
    LockReview,
    /// §9: a retired id's unanswered rejection, a superseded id taken again, a dropped lineage.
    LockRetired,
    /// §12: two records supply one name under one selection; §14.4's check per selection, of the check-passing
    /// convention the port and the records beside it depend on.
    Conflict,
}

impl Code {
    /// The code as §11 writes it.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Read => "catalog-read",
            Self::Layout => "catalog-layout",
            Self::Shape => "catalog-shape",
            Self::Id => "catalog-id",
            Self::Version => "catalog-version",
            Self::Field => "catalog-field",
            Self::Locator => "catalog-locator",
            Self::Source => "catalog-source",
            Self::Dependency => "catalog-dependency",
            Self::Review => "catalog-review",
            Self::Production => "catalog-production",
            Self::LockMissing => "catalog-lock-missing",
            Self::LockUnbumped => "catalog-lock-unbumped",
            Self::LockDowngrade => "catalog-lock-downgrade",
            Self::LockReview => "catalog-lock-review",
            Self::LockRetired => "catalog-lock-retired",
            Self::Conflict => "catalog-conflict",
        }
    }
}

impl fmt::Display for Code {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(self.as_str())
    }
}

/// Where in a file a refusal points: a 1-based line and column.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct At {
    /// 1-based line.
    pub line: u32,
    /// 1-based column, in characters.
    pub column: u32,
}

/// One reason a catalog does not load.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    /// The rule broken.
    pub code: Code,
    /// The file, relative to the repository root.
    pub path: String,
    /// The field, as a path of form heads: `timing-model costs cost[dispatch] value`, or `(file)`.
    pub field: String,
    /// Where the field is, when the file could be read far enough to say.
    pub at: Option<At>,
    /// What is wrong, in one sentence.
    pub message: String,
}

impl Refusal {
    /// A refusal with every part named.
    #[must_use]
    pub fn new(
        code: Code,
        path: &str,
        field: &str,
        at: Option<At>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            code,
            path: path.to_owned(),
            field: field.to_owned(),
            at,
            message: message.into(),
        }
    }
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.at {
            Some(at) => write!(
                f,
                "{}:{}:{}: {} [{}] {}",
                self.path, at.line, at.column, self.code, self.field, self.message
            ),
            None => write!(
                f,
                "{}: {} [{}] {}",
                self.path, self.code, self.field, self.message
            ),
        }
    }
}
