//! Reading one record (the record's §1 and §2).
//!
//! A record is a tracked ASCII file, `catalog/<namespace>/<id>.catalog`, holding exactly one `catalog-record` form
//! in the eADL reader's datum syntax. [`read_record`] reads it and checks every rule that needs nothing but the
//! file itself: its bytes, its one form, every field in §2's order with its form, and every name, version and
//! locator's grammar. What needs other files — whether a dependency resolves, whether a locator's file is in a set,
//! whether a target exists — is checked where the catalog is read whole.

use archogen_evidence::bound::{Bound, BoundOrigin, SafetyFactor};
use eadl_front::{Form, SourceId, SourceMap, Span};

use crate::dialect;
use crate::grammar::{self, Requirement, Version};
use crate::hash::encode;
use crate::refusal::{At, Code, Refusal};
use crate::statement;

/// Which namespace a record lives in (§6).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Namespace {
    /// Held to every production rule.
    Production,
    /// Anything that passes the structure.
    Experimental,
}

/// What a path under `catalog/` is (§1).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum CatalogPath {
    /// Not under `catalog/` at all.
    Outside,
    /// `catalog/catalog.lock`.
    Lock,
    /// A record, with its namespace and file stem.
    Record(Namespace, String),
}

/// Classify a tracked path (§1): the lock, a record in a namespace directory, or a refusal.
///
/// # Errors
///
/// `catalog-layout` for any other file under `catalog/`, so a misspelled extension cannot hide a record.
pub fn classify(path: &str) -> Result<CatalogPath, Refusal> {
    let Some(rest) = path.strip_prefix("catalog/") else {
        return Ok(CatalogPath::Outside);
    };
    if rest == "catalog.lock" {
        return Ok(CatalogPath::Lock);
    }
    let layout = || {
        Refusal::new(
            Code::Layout,
            path,
            "(file)",
            None,
            "only `catalog/catalog.lock` and `catalog/<production|experimental>/<id>.catalog` may be under `catalog/`",
        )
    };
    let (dir, file) = rest.split_once('/').ok_or_else(layout)?;
    let namespace = match dir {
        "production" => Namespace::Production,
        "experimental" => Namespace::Experimental,
        _ => return Err(layout()),
    };
    let stem = file
        .strip_suffix(".catalog")
        .filter(|s| !s.contains('/') && !s.is_empty())
        .ok_or_else(layout)?;
    Ok(CatalogPath::Record(namespace, stem.to_owned()))
}

/// The four catalogs of `ROADMAP.md` §9.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Catalog {
    /// `algorithms`.
    Algorithms,
    /// `machine`.
    Machine,
    /// `devices`.
    Devices,
    /// `interfaces`.
    Interfaces,
}

/// Where a record applies (§2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Targets {
    /// Every target under `targets/`.
    Any,
    /// The named targets, each a stem under `targets/`.
    Named(Vec<String>),
}

/// A dependency on another record's contract (§2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Dependency {
    /// The record depended on.
    pub id: String,
    /// The contract version it must have.
    pub requirement: Requirement,
}

/// The contract facet (§2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Contract {
    /// The record's semantic version, the contract's own.
    pub version: Version,
    /// Which of the four catalogs.
    pub catalog: Catalog,
    /// Where the content came from.
    pub origin: String,
    /// Its license.
    pub license: String,
    /// The task tree that owns the record.
    pub maintainer: String,
    /// Its dependencies.
    pub depends: Vec<Dependency>,
    /// The ids it replaces.
    pub supersedes: Vec<String>,
    /// The profiles it supports.
    pub profiles: Vec<String>,
    /// Where it applies.
    pub targets: Targets,
    /// Its preconditions, each a sentence.
    pub preconditions: Vec<String>,
    /// Its guarantees, each a sentence; at least one.
    pub guarantees: Vec<String>,
}

/// A locator: where a fact or cost comes from (§2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Locator {
    /// A file in the facet's own source set.
    File(String),
    /// A file in a record's implementation own set.
    Code {
        /// The record.
        id: String,
        /// The file.
        path: String,
    },
    /// A section of the source ledger.
    Ledger {
        /// The section's anchor.
        anchor: String,
        /// `"<revision>, <section>"`.
        detail: String,
    },
}

impl Locator {
    /// The locator as `Bound`'s origin renders it (§2): `file:<path>`, `code:<id>:<path>`,
    /// `ledger:<anchor>: <detail>`.
    #[must_use]
    pub fn render(&self) -> String {
        match self {
            Self::File(path) => format!("file:{path}"),
            Self::Code { id, path } => format!("code:{id}:{path}"),
            Self::Ledger { anchor, detail } => format!("ledger:{anchor}: {detail}"),
        }
    }
}

/// A fact (§2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Fact {
    /// Its name.
    pub name: String,
    /// What is known of it.
    pub value: FactValue,
}

/// A fact's value.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum FactValue {
    /// Known: `yes` or `no`, with where it comes from and why.
    Known {
        /// `yes`.
        holds: bool,
        /// Where it comes from: one locator, or for a code fact one or more `code` locators (§2, §14.2).
        locators: Vec<Locator>,
        /// Why.
        basis: String,
    },
    /// Nobody knows, and why.
    Unknown(String),
}

/// A cost's unit (§2).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Unit {
    /// Nanoseconds.
    Ns,
    /// Microseconds.
    Us,
    /// Milliseconds.
    Ms,
}

impl Unit {
    /// The unit as written.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Ns => "ns",
            Self::Us => "us",
            Self::Ms => "ms",
        }
    }
}

/// Which image a value was obtained on (§2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Binary {
    /// `sha256:<hex>`.
    Image(String),
    /// There was no image.
    Unbuilt,
    /// The value depends on no image, and why.
    Independent(String),
}

/// A cost's evidence category (`ROADMAP.md` §7.3; §2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Evidence {
    /// `assumed`.
    Assumed,
    /// `observed-maximum`, with the raw observation and its safety factor when it was padded.
    ObservedMaximum(Option<(u64, SafetyFactor)>),
    /// `externally-supplied`.
    ExternallySupplied,
    /// `analytically-established`.
    AnalyticallyEstablished,
}

/// A known cost (§2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KnownCost {
    /// The value, in `unit`.
    pub value: u64,
    /// The unit.
    pub unit: Unit,
    /// What the value applies to.
    pub scope: String,
    /// The most tasks it holds for.
    pub tasks: u64,
    /// The most declared interrupt sources, the timer apart, it holds for.
    pub sources: u64,
    /// Whether it holds under any preemption pattern.
    pub holds_under_preemption: bool,
    /// The image it was obtained on.
    pub binary: Binary,
    /// Its evidence category.
    pub evidence: Evidence,
    /// Where it comes from, optional for `assumed` alone.
    pub locator: Option<Locator>,
    /// Why.
    pub basis: String,
}

/// A cost (§2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Cost {
    /// Its name.
    pub name: String,
    /// The target it is on.
    pub target: String,
    /// Known, with its evidence, or unknown with the reason.
    pub value: Result<KnownCost, String>,
}

/// The three facets that may be `none` (§2): present with their content, or a statement that they do not exist.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Content<T> {
    /// The facet's content.
    Present(T),
    /// The facet does not exist, and why.
    None(String),
}

/// An implementation's content: its packages, and those of them that hold assembly.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Packages {
    /// Its `sources`, each a package (§3).
    pub sources: Vec<String>,
    /// Its `assembly` declaration, when it ends with one (§14.2).
    pub assembly: Option<Assembly>,
}

/// An `(assembly <architecture> "<package>" …)` declaration (§14.2): which of the facet's packages hold assembly,
/// and in which dialect.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assembly {
    /// The dialect's architecture, one §14.3 lists.
    pub architecture: String,
    /// The packages, each byte for byte an entry of the facet's `sources`.
    pub packages: Vec<String>,
}

/// A behavioral model's content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BehaviorModel {
    /// The files and packages it rests on.
    pub sources: Vec<String>,
    /// The records whose code its facts are about, beyond its own implementation.
    pub describes: Vec<String>,
    /// Its facts.
    pub facts: Vec<Fact>,
}

/// A timing model's content.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TimingModel {
    /// The files and packages it rests on.
    pub sources: Vec<String>,
    /// The records whose code or figures its costs rest on, beyond its dependency closure.
    pub measured_with: Vec<String>,
    /// Its facts.
    pub facts: Vec<Fact>,
    /// Its costs.
    pub costs: Vec<Cost>,
}

/// A facet other than the contract: its own version, and its content (§2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Facet<T> {
    /// The facet's own version.
    pub version: Version,
    /// Its content.
    pub content: Content<T>,
}

/// Which facet a review names (§5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum FacetKind {
    /// The contract.
    Contract,
    /// The implementation.
    Implementation,
    /// The behavioral model.
    BehaviorModel,
    /// The timing model.
    TimingModel,
}

impl FacetKind {
    /// Every facet, in §9's order.
    pub const ALL: [Self; 4] = [
        Self::Contract,
        Self::Implementation,
        Self::BehaviorModel,
        Self::TimingModel,
    ];

    /// The facet as written.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Contract => "contract",
            Self::Implementation => "implementation",
            Self::BehaviorModel => "behavior-model",
            Self::TimingModel => "timing-model",
        }
    }

    /// The facet named by `text`.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|f| f.as_str() == text)
    }
}

/// A review's verdict (§5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Verdict {
    /// Fit for production at the hash named.
    Production,
    /// Rejected, until a later review answers it.
    Rejected,
}

impl Verdict {
    /// The verdict as written.
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::Production => "production",
            Self::Rejected => "rejected",
        }
    }

    /// The verdict named by `text`.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        [Self::Production, Self::Rejected]
            .into_iter()
            .find(|v| v.as_str() == text)
    }
}

/// A reviewer's role (§5).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Role {
    /// The director.
    Director,
    /// A context that did not write the facet.
    IndependentContext,
    /// A named outside party.
    External,
}

/// A review (§2, §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Review {
    /// The facet it reviews.
    pub facet: FacetKind,
    /// The facet's bound hash it names.
    pub hash: String,
    /// Its verdict.
    pub verdict: Verdict,
    /// The ledger hashes of the rejections it answers.
    pub answers: Vec<String>,
    /// The reviewer's role.
    pub role: Role,
    /// Who the reviewer is.
    pub who: String,
    /// When, as a calendar date.
    pub date: (u32, u32, u32),
    /// What was checked against what.
    pub basis: String,
    /// The review's form, which its ledger hash is over (§3).
    pub form: Form,
}

/// A record as read (§1, §2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// The file it was read from.
    pub path: String,
    /// Its id, which is its file stem.
    pub id: String,
    /// Its namespace, which is its directory.
    pub namespace: Namespace,
    /// Its contract.
    pub contract: Contract,
    /// Its implementation.
    pub implementation: Facet<Packages>,
    /// Its behavioral model.
    pub behavior_model: Facet<BehaviorModel>,
    /// Its timing model.
    pub timing_model: Facet<TimingModel>,
    /// Its reviews, in the order written, which decides nothing (§5).
    pub reviews: Vec<Review>,
    /// The whole `catalog-record` form, which §3's hashes are computed over.
    pub form: Form,
}

/// The fields of a `catalog-record`, in §2's order, and how often each may appear.
const RECORD_FIELDS: &[(&str, Occurs)] = &[
    ("version", Occurs::One),
    ("catalog", Occurs::One),
    ("source", Occurs::One),
    ("maintainer", Occurs::One),
    ("depends", Occurs::One),
    ("supersedes", Occurs::One),
    ("profiles", Occurs::One),
    ("targets", Occurs::One),
    ("preconditions", Occurs::One),
    ("guarantees", Occurs::One),
    ("implementation", Occurs::One),
    ("behavior-model", Occurs::One),
    ("timing-model", Occurs::One),
    ("review", Occurs::Many),
];

/// How often a subform may appear.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Occurs {
    One,
    Optional,
    Many,
}

/// The one name `independent` is admitted for (§2, §12).
const INDEPENDENT_NAMES: [&str; 2] = ["compare-rounding", "delivery"];

/// Read one record from its path and bytes.
///
/// # Errors
///
/// The first rule the file breaks, as a [`Refusal`] naming its code, field and position.
pub fn read_record(path: &str, bytes: &[u8]) -> Result<Record, Refusal> {
    let (namespace, stem) = match classify(path)? {
        CatalogPath::Record(namespace, stem) => (namespace, stem),
        _ => {
            return Err(Refusal::new(
                Code::Layout,
                path,
                "(file)",
                None,
                "not a record's path",
            ));
        }
    };
    if let Some(offset) = bytes
        .iter()
        .position(|&b| !((0x20..=0x7e).contains(&b) || b == b'\n'))
    {
        let line = bytes[..offset].iter().filter(|&&b| b == b'\n').count() + 1;
        let column = offset
            - bytes[..offset]
                .iter()
                .rposition(|&b| b == b'\n')
                .map_or(0, |p| p + 1)
            + 1;
        return Err(Refusal::new(
            Code::Read,
            path,
            "(file)",
            Some(At {
                line: to_u32(line),
                column: to_u32(column),
            }),
            format!(
                "byte 0x{:02x} is neither printable ASCII nor a line feed",
                bytes[offset]
            ),
        ));
    }
    let text = core::str::from_utf8(bytes).unwrap_or_default();
    let mut sources = SourceMap::new();
    let source = sources
        .add(path, text)
        .map_err(|e| Refusal::new(Code::Read, path, "(file)", None, e.to_string()))?;
    let ctx = Ctx {
        path,
        sources: &sources,
        source,
    };
    let (document, diagnostics) = eadl_front::read(&sources, source);
    if let Some(d) = diagnostics.items().first() {
        return Err(Refusal::new(
            Code::Read,
            path,
            "(file)",
            ctx.at(d.primary.span),
            d.message.clone(),
        ));
    }
    for form in &document.forms {
        ctx.scan(form)?;
    }
    let [form] = document.forms.as_slice() else {
        return Err(ctx.refuse(
            Code::Shape,
            "(file)",
            None,
            "a record holds exactly one top-level form",
        ));
    };
    if form.head() != Some("catalog-record") {
        return Err(ctx.refuse(
            Code::Shape,
            "(file)",
            Some(form),
            "the top-level form must be `(catalog-record …)`",
        ));
    }
    let items = form.items();
    let id = match items.get(1) {
        Some(Form::Symbol { name, .. }) => name.clone(),
        other => {
            return Err(ctx.refuse(
                Code::Id,
                "id",
                other.or(Some(form)),
                "the id, a symbol, follows `catalog-record`",
            ))
        }
    };
    if !grammar::is_id(&id) {
        return Err(ctx.refuse(
            Code::Id,
            "id",
            items.get(1),
            format!("`{id}` breaks §1's id grammar"),
        ));
    }
    if id != stem {
        return Err(ctx.refuse(
            Code::Id,
            "id",
            items.get(1),
            format!("the id `{id}` differs from the file stem `{stem}`"),
        ));
    }
    let slots = ctx.slots("catalog-record", &items[2..], RECORD_FIELDS)?;
    let one = |i: usize| slots[i][0];
    let contract = Contract {
        version: ctx.version("version", one(0))?,
        catalog: ctx.catalog(one(1))?,
        origin: ctx.pair_text("source", one(2), "origin")?,
        license: ctx.pair_text("source", one(2), "license")?,
        maintainer: ctx.maintainer(one(3))?,
        depends: ctx.depends(one(4))?,
        supersedes: ctx.ids("supersedes", one(5), Code::Dependency)?,
        profiles: ctx.profiles(one(6))?,
        targets: ctx.targets(one(7))?,
        preconditions: ctx.texts("preconditions", one(8), false)?,
        guarantees: ctx.texts("guarantees", one(9), true)?,
    };
    let implementation = ctx.implementation(one(10))?;
    let behavior_model = ctx.behavior_model(one(11))?;
    let timing_model = ctx.timing_model(one(12))?;
    ctx.names(&behavior_model, &timing_model)?;
    ctx.section_12(&id, [one(11), one(12)], &behavior_model, &timing_model)?;
    ctx.section_14_4(
        &id,
        contract.catalog,
        &implementation,
        one(11),
        &behavior_model,
        &timing_model,
    )?;
    let reviews = slots[13]
        .iter()
        .map(|review| ctx.review(review, &contract.maintainer))
        .collect::<Result<Vec<_>, _>>()?;
    Ok(Record {
        path: path.to_owned(),
        id,
        namespace,
        contract,
        implementation,
        behavior_model,
        timing_model,
        reviews,
        form: form.clone(),
    })
}

fn to_u32(n: usize) -> u32 {
    u32::try_from(n).unwrap_or(u32::MAX)
}

/// Where `form`, one of the forms of the record read from `bytes`, begins: what a check made after reading, such as
/// the lock's (§9), points at.
#[must_use]
pub fn position(path: &str, bytes: &[u8], form: &Form) -> Option<At> {
    let mut sources = SourceMap::new();
    let source = sources.add(path, core::str::from_utf8(bytes).ok()?).ok()?;
    let p = sources.get(source)?.position(form.span().start);
    Some(At {
        line: p.line,
        column: p.column,
    })
}

/// The facts §12's facts table names that take any `.<suffix>`: a source's, or a record's id for
/// `runtime-discipline` and §14.4's two self-named facts.
const TABLED_FAMILIES: [(&str, FacetKind, bool); 8] = [
    (
        statement::GUARD_CHECK_CONTEXTS,
        FacetKind::BehaviorModel,
        true,
    ),
    (
        statement::CONVENTION_STATED,
        FacetKind::BehaviorModel,
        false,
    ),
    ("runtime-discipline.", FacetKind::BehaviorModel, true),
    ("no-application-code.", FacetKind::BehaviorModel, true),
    ("acknowledge-at-entry.", FacetKind::BehaviorModel, true),
    ("defers-nothing.", FacetKind::BehaviorModel, true),
    ("one-request-per-arrival.", FacetKind::BehaviorModel, true),
    ("external.", FacetKind::BehaviorModel, false),
];

/// Every other fact §12's facts table names, with its facet and whether it is about code.
const TABLED: [(&str, FacetKind, bool); 25] = [
    ("timer-event-driven", FacetKind::BehaviorModel, true),
    ("compare-rounds-up", FacetKind::BehaviorModel, true),
    ("due-check-matches-compare", FacetKind::BehaviorModel, true),
    ("no-early-release", FacetKind::BehaviorModel, true),
    ("raised-only-when-due", FacetKind::BehaviorModel, true),
    (
        "only-timer-releases-timer-tasks",
        FacetKind::BehaviorModel,
        true,
    ),
    ("preemptive-everywhere", FacetKind::BehaviorModel, true),
    (
        "sections-mask-every-interrupt",
        FacetKind::BehaviorModel,
        true,
    ),
    ("interrupts-do-not-nest", FacetKind::BehaviorModel, true),
    (
        "services-preempt-every-task",
        FacetKind::BehaviorModel,
        true,
    ),
    (
        "pending-taken-and-transitions-unmasked",
        FacetKind::BehaviorModel,
        true,
    ),
    ("one-claim-per-trap", FacetKind::BehaviorModel, true),
    ("starts-by-transition", FacetKind::BehaviorModel, true),
    ("pending-taken-after-unmask", FacetKind::BehaviorModel, true),
    ("no-empty-claim", FacetKind::BehaviorModel, true),
    ("one-processor", FacetKind::BehaviorModel, false),
    ("compare-level", FacetKind::BehaviorModel, false),
    ("external-before-timer", FacetKind::BehaviorModel, false),
    ("one-external-controller", FacetKind::BehaviorModel, false),
    ("reprograms-only-in-service", FacetKind::BehaviorModel, true),
    ("no-suspension-primitive", FacetKind::BehaviorModel, true),
    (
        "no-scheduler-lock-primitive",
        FacetKind::BehaviorModel,
        true,
    ),
    ("releases-never-latched", FacetKind::BehaviorModel, true),
    ("primitives-out-of-line", FacetKind::BehaviorModel, true),
    ("eager-switching", FacetKind::TimingModel, true),
];

/// The twelve facts about the port's code (§14.2): a known value of one needs a locator into assembly a record
/// declares, which the catalog checks whole ([`crate::hash::Catalog::hashes`]).
pub const PORT_FACTS: [&str; 12] = [
    "eager-switching",
    "interrupts-do-not-nest",
    "services-preempt-every-task",
    "pending-taken-and-transitions-unmasked",
    "pending-taken-after-unmask",
    "no-empty-claim",
    "one-claim-per-trap",
    "starts-by-transition",
    "preemptive-everywhere",
    "sections-mask-every-interrupt",
    "releases-never-latched",
    "primitives-out-of-line",
];

/// Whether `name` is a code fact (§2): one §12 or §14.4 lists as about code.
#[must_use]
pub fn is_code_fact(name: &str) -> bool {
    tabled(name).is_some_and(|(_, code)| code)
}

/// The facet §12's facts table gives fact `name`, and whether it is about code; `None` for a fact it does not name.
#[must_use]
pub fn tabled(name: &str) -> Option<(FacetKind, bool)> {
    if statement::FACTS.contains(&name) {
        return Some((
            FacetKind::BehaviorModel,
            !statement::NOT_CODE.contains(&name),
        ));
    }
    TABLED
        .iter()
        .find(|(n, ..)| *n == name)
        .or_else(|| {
            TABLED_FAMILIES.iter().find(|(prefix, ..)| {
                name.strip_prefix(prefix)
                    .is_some_and(|rest| !rest.is_empty())
            })
        })
        .map(|(_, facet, code)| (*facet, *code))
}

/// What every check needs: the file and its source map, for positions.
struct Ctx<'a> {
    path: &'a str,
    sources: &'a SourceMap,
    source: SourceId,
}

impl Ctx<'_> {
    fn at(&self, span: Span) -> Option<At> {
        let p = self.sources.get(self.source)?.position(span.start);
        Some(At {
            line: p.line,
            column: p.column,
        })
    }

    fn refuse(
        &self,
        code: Code,
        field: &str,
        form: Option<&Form>,
        message: impl Into<String>,
    ) -> Refusal {
        Refusal::new(
            code,
            self.path,
            field,
            form.and_then(|f| self.at(f.span())),
            message,
        )
    }

    /// No decimal anywhere, and every decoded string printable ASCII (§1).
    fn scan(&self, form: &Form) -> Result<(), Refusal> {
        match form {
            Form::Decimal { .. } => Err(self.refuse(Code::Shape, "(file)", Some(form), "a decimal; `/1` admits integers only")),
            Form::Str { value, .. } if !grammar::is_printable(value.as_bytes(), false) => Err(self.refuse(
                Code::Read,
                "(file)",
                Some(form),
                "a string whose decoded value is not printable ASCII; an escape cannot hide a control character",
            )),
            Form::List { items, .. } => items.iter().try_for_each(|item| self.scan(item)),
            _ => Ok(()),
        }
    }

    /// Match `items` against `spec`, in order: each a `(name …)` list, unknown, missing, duplicated or out of order
    /// refused as `catalog-shape` (§1).
    fn slots<'f>(
        &self,
        field: &str,
        items: &'f [Form],
        spec: &[(&str, Occurs)],
    ) -> Result<Vec<Vec<&'f Form>>, Refusal> {
        let mut slots: Vec<Vec<&Form>> = vec![Vec::new(); spec.len()];
        let mut last: Option<usize> = None;
        for item in items {
            let Some(head) = item.head() else {
                return Err(self.refuse(
                    Code::Shape,
                    field,
                    Some(item),
                    "expected a `(name …)` subform",
                ));
            };
            let Some(p) = spec.iter().position(|(name, _)| *name == head) else {
                return Err(self.refuse(
                    Code::Shape,
                    field,
                    Some(item),
                    format!("`{head}` is not a subform of `{field}`"),
                ));
            };
            let repeats = spec[p].1 == Occurs::Many;
            if !slots[p].is_empty() && !repeats {
                return Err(self.refuse(
                    Code::Shape,
                    field,
                    Some(item),
                    format!("`{head}` appears twice"),
                ));
            }
            if last.is_some_and(|l| p < l) {
                return Err(self.refuse(
                    Code::Shape,
                    field,
                    Some(item),
                    format!("`{head}` is out of §2's order"),
                ));
            }
            last = Some(p);
            slots[p].push(item);
        }
        if let Some(((name, _), _)) = spec
            .iter()
            .zip(&slots)
            .find(|((_, occurs), slot)| *occurs == Occurs::One && slot.is_empty())
        {
            return Err(Refusal::new(
                Code::Shape,
                self.path,
                field,
                None,
                format!("`{name}` is missing"),
            ));
        }
        Ok(slots)
    }

    /// `(head x)`: exactly one argument, returned.
    fn only<'f>(&self, field: &str, form: &'f Form) -> Result<&'f Form, Refusal> {
        match form.items() {
            [_, value] => Ok(value),
            _ => Err(self.refuse(
                Code::Shape,
                field,
                Some(form),
                format!("`{field}` takes exactly one value"),
            )),
        }
    }

    fn string<'f>(&self, field: &str, form: &'f Form) -> Result<&'f str, Refusal> {
        match form {
            Form::Str { value, .. } => Ok(value),
            _ => Err(self.refuse(
                Code::Shape,
                field,
                Some(form),
                format!("`{field}` is a string"),
            )),
        }
    }

    fn symbol<'f>(&self, field: &str, form: &'f Form) -> Result<&'f str, Refusal> {
        form.as_symbol().ok_or_else(|| {
            self.refuse(
                Code::Shape,
                field,
                Some(form),
                format!("`{field}` is a symbol"),
            )
        })
    }

    /// A string that §2 requires to hold text.
    fn text(&self, field: &str, form: &Form) -> Result<String, Refusal> {
        let value = self.string(field, form)?;
        if value.trim().is_empty() {
            return Err(self.refuse(
                Code::Field,
                field,
                Some(form),
                format!("`{field}` is empty where §2 requires text"),
            ));
        }
        Ok(value.to_owned())
    }

    fn version(&self, field: &str, form: &Form) -> Result<Version, Refusal> {
        let value = self.only(field, form)?;
        let text = self.string(field, value)?;
        grammar::version(text).ok_or_else(|| {
            self.refuse(
                Code::Version,
                field,
                Some(value),
                format!("`{text}` is not MAJOR.MINOR.PATCH without leading zeros"),
            )
        })
    }

    fn catalog(&self, form: &Form) -> Result<Catalog, Refusal> {
        let value = self.only("catalog", form)?;
        match self.symbol("catalog", value)? {
            "algorithms" => Ok(Catalog::Algorithms),
            "machine" => Ok(Catalog::Machine),
            "devices" => Ok(Catalog::Devices),
            "interfaces" => Ok(Catalog::Interfaces),
            other => Err(self.refuse(
                Code::Field,
                "catalog",
                Some(value),
                format!("`{other}` is not one of `ROADMAP.md` §9's four catalogs"),
            )),
        }
    }

    fn pair_text(&self, field: &str, form: &Form, part: &str) -> Result<String, Refusal> {
        let slots = self.slots(
            field,
            &form.items()[1..],
            &[("origin", Occurs::One), ("license", Occurs::One)],
        )?;
        let sub = slots[usize::from(part == "license")][0];
        let value = self.only(part, sub)?;
        self.text(&format!("{field} {part}"), value)
    }

    fn maintainer(&self, form: &Form) -> Result<String, Refusal> {
        let value = self.only("maintainer", form)?;
        let text = self.symbol("maintainer", value)?;
        if !grammar::is_maintainer(text) {
            return Err(self.refuse(
                Code::Field,
                "maintainer",
                Some(value),
                format!("`{text}` is not a task tree's id"),
            ));
        }
        Ok(text.to_owned())
    }

    fn depends(&self, form: &Form) -> Result<Vec<Dependency>, Refusal> {
        let mut out: Vec<Dependency> = Vec::new();
        for item in &form.items()[1..] {
            let [Form::Symbol { name: id, .. }, requirement] = item.items() else {
                return Err(self.refuse(
                    Code::Shape,
                    "depends",
                    Some(item),
                    "a dependency is `(<id> \"<MAJOR.MINOR>\")`",
                ));
            };
            if !grammar::is_id(id) {
                return Err(self.refuse(
                    Code::Id,
                    "depends",
                    Some(item),
                    format!("`{id}` breaks §1's id grammar"),
                ));
            }
            let text = self.string("depends", requirement)?;
            let requirement = grammar::requirement(text).ok_or_else(|| {
                self.refuse(
                    Code::Version,
                    "depends",
                    Some(item),
                    format!("`{text}` is not a MAJOR.MINOR requirement"),
                )
            })?;
            if out.iter().any(|d| d.id == *id) {
                return Err(self.refuse(
                    Code::Dependency,
                    "depends",
                    Some(item),
                    format!("`{id}` is listed twice"),
                ));
            }
            out.push(Dependency {
                id: id.clone(),
                requirement,
            });
        }
        Ok(out)
    }

    /// A list of record ids, each in §1's grammar and listed once.
    fn ids(&self, field: &str, form: &Form, twice: Code) -> Result<Vec<String>, Refusal> {
        let mut out: Vec<String> = Vec::new();
        for item in &form.items()[1..] {
            let id = self.symbol(field, item)?;
            if !grammar::is_id(id) {
                return Err(self.refuse(
                    Code::Id,
                    field,
                    Some(item),
                    format!("`{id}` breaks §1's id grammar"),
                ));
            }
            if out.iter().any(|x| x == id) {
                return Err(self.refuse(
                    twice,
                    field,
                    Some(item),
                    format!("`{id}` is listed twice"),
                ));
            }
            out.push(id.to_owned());
        }
        Ok(out)
    }

    fn profiles(&self, form: &Form) -> Result<Vec<String>, Refusal> {
        let items = &form.items()[1..];
        if items.is_empty() {
            return Err(self.refuse(
                Code::Shape,
                "profiles",
                Some(form),
                "a record supports at least one profile",
            ));
        }
        let mut out: Vec<String> = Vec::new();
        for item in items {
            let id = self.symbol("profiles", item)?;
            if eadl_model::profile::supported(id).is_none() {
                return Err(self.refuse(
                    Code::Field,
                    "profiles",
                    Some(item),
                    format!("`{id}` is not a profile the engine supports"),
                ));
            }
            if out.iter().any(|p| p == id) {
                return Err(self.refuse(
                    Code::Shape,
                    "profiles",
                    Some(item),
                    format!("`{id}` is repeated"),
                ));
            }
            out.push(id.to_owned());
        }
        Ok(out)
    }

    fn targets(&self, form: &Form) -> Result<Targets, Refusal> {
        let items = &form.items()[1..];
        if let [Form::Symbol { name, .. }] = items {
            if name == "any" {
                return Ok(Targets::Any);
            }
        }
        if items.is_empty() {
            return Err(self.refuse(
                Code::Shape,
                "targets",
                Some(form),
                "`targets` is `any` or at least one target",
            ));
        }
        let mut out: Vec<String> = Vec::new();
        for item in items {
            let stem = self.symbol("targets", item)?;
            if stem == "any" || !grammar::is_target_stem(stem) {
                return Err(self.refuse(
                    Code::Field,
                    "targets",
                    Some(item),
                    format!("`{stem}` is not a target stem, and `any` stands alone"),
                ));
            }
            if out.iter().any(|t| t == stem) {
                return Err(self.refuse(
                    Code::Shape,
                    "targets",
                    Some(item),
                    format!("`{stem}` is repeated"),
                ));
            }
            out.push(stem.to_owned());
        }
        Ok(Targets::Named(out))
    }

    fn texts(&self, field: &str, form: &Form, at_least_one: bool) -> Result<Vec<String>, Refusal> {
        let items = &form.items()[1..];
        if at_least_one && items.is_empty() {
            return Err(self.refuse(
                Code::Field,
                field,
                Some(form),
                "no guarantees: a record that guarantees nothing is not an entry",
            ));
        }
        items.iter().map(|item| self.text(field, item)).collect()
    }

    /// A list of paths in §4's normal form, each at most once (§4: `catalog-source`).
    fn paths(&self, field: &str, form: &Form, at_least_one: bool) -> Result<Vec<String>, Refusal> {
        let items = &form.items()[1..];
        if at_least_one && items.is_empty() {
            return Err(self.refuse(
                Code::Shape,
                field,
                Some(form),
                format!("`{field}` names at least one package"),
            ));
        }
        let mut out: Vec<String> = Vec::new();
        for item in items {
            let path = self.string(field, item)?;
            if !grammar::is_path(path) {
                return Err(self.refuse(
                    Code::Source,
                    field,
                    Some(item),
                    format!("`{path}` is not a path in §4's normal form"),
                ));
            }
            if out.iter().any(|p| p == path) {
                return Err(self.refuse(
                    Code::Source,
                    field,
                    Some(item),
                    format!("`{path}` appears twice in the facet"),
                ));
            }
            out.push(path.to_owned());
        }
        Ok(out)
    }

    /// A facet's `none`, when it is one: nothing else but the version beside it.
    fn none_of(
        &self,
        field: &str,
        slots: &[Vec<&Form>],
        none_slot: usize,
    ) -> Result<Option<String>, Refusal> {
        let Some(none) = slots[none_slot].first() else {
            return Ok(None);
        };
        if let Some(extra) = slots[1..none_slot].iter().flatten().next() {
            return Err(self.refuse(
                Code::Shape,
                field,
                Some(extra),
                "a `none` facet holds its version and nothing else",
            ));
        }
        let value = self.only("none", none)?;
        Ok(Some(self.text(&format!("{field} none"), value)?))
    }

    /// Every present-form subform of a facet other than `none` must be there.
    fn present(&self, field: &str, slots: &[Vec<&Form>], names: &[&str]) -> Result<(), Refusal> {
        match names
            .iter()
            .zip(&slots[1..])
            .find(|(_, slot)| slot.is_empty())
        {
            Some((name, _)) => Err(Refusal::new(
                Code::Shape,
                self.path,
                field,
                None,
                format!("`{name}` is missing, and the facet is not `none`"),
            )),
            None => Ok(()),
        }
    }

    fn implementation(&self, form: &Form) -> Result<Facet<Packages>, Refusal> {
        let spec = [
            ("version", Occurs::One),
            ("sources", Occurs::Optional),
            ("assembly", Occurs::Optional),
            ("none", Occurs::Optional),
        ];
        let slots = self.slots("implementation", &form.items()[1..], &spec)?;
        let version = self.version("implementation version", slots[0][0])?;
        let content = match self.none_of("implementation", &slots, 3)? {
            Some(why) => Content::None(why),
            None => {
                self.present("implementation", &slots, &["sources"])?;
                let sources = self.paths("implementation sources", slots[1][0], true)?;
                let assembly = match slots[2].first() {
                    Some(declaration) => Some(self.assembly(declaration, &sources)?),
                    None => None,
                };
                Content::Present(Packages { sources, assembly })
            }
        };
        Ok(Facet { version, content })
    }

    /// `(assembly <architecture> "<package>" …)` (§14.2), its refusals in §14.2's order: its shape, then its
    /// packages against the facet's `sources` (`catalog-source`), then its architecture (`catalog-field`).
    fn assembly(&self, form: &Form, sources: &[String]) -> Result<Assembly, Refusal> {
        let field = "implementation assembly";
        let items = form.items();
        let architecture = match items.get(1) {
            Some(Form::Symbol { name, .. }) => name.clone(),
            _ => {
                return Err(self.refuse(
                    Code::Shape,
                    field,
                    Some(form),
                    "an `assembly` declaration is `(assembly <architecture> \"<package>\" …)`",
                ))
            }
        };
        if items.len() < 3 {
            return Err(self.refuse(
                Code::Shape,
                field,
                Some(form),
                "an `assembly` declaration names at least one package",
            ));
        }
        for item in &items[2..] {
            self.string(field, item)?;
        }
        let mut packages: Vec<String> = Vec::new();
        for item in &items[2..] {
            let package = self.string(field, item)?;
            if packages.iter().any(|p| p == package) {
                return Err(self.refuse(
                    Code::Source,
                    field,
                    Some(item),
                    format!("`{package}` appears twice in the declaration"),
                ));
            }
            if !sources.iter().any(|s| s == package) {
                return Err(self.refuse(
                    Code::Source,
                    field,
                    Some(item),
                    format!("`{package}` is not an entry of the facet's `sources`"),
                ));
            }
            packages.push(package.to_owned());
        }
        if dialect::triples(&architecture).is_none() {
            return Err(self.refuse(
                Code::Field,
                field,
                items.get(1),
                format!("`{architecture}` is not an architecture §14.3 lists"),
            ));
        }
        Ok(Assembly {
            architecture,
            packages,
        })
    }

    fn behavior_model(&self, form: &Form) -> Result<Facet<BehaviorModel>, Refusal> {
        let spec = [
            ("version", Occurs::One),
            ("sources", Occurs::Optional),
            ("describes", Occurs::Optional),
            ("facts", Occurs::Optional),
            ("none", Occurs::Optional),
        ];
        let slots = self.slots("behavior-model", &form.items()[1..], &spec)?;
        let version = self.version("behavior-model version", slots[0][0])?;
        let content = match self.none_of("behavior-model", &slots, 4)? {
            Some(why) => Content::None(why),
            None => {
                self.present("behavior-model", &slots, &["sources", "describes", "facts"])?;
                Content::Present(BehaviorModel {
                    sources: self.paths("behavior-model sources", slots[1][0], false)?,
                    describes: self.ids(
                        "behavior-model describes",
                        slots[2][0],
                        Code::Dependency,
                    )?,
                    facts: self.facts("behavior-model", slots[3][0])?,
                })
            }
        };
        Ok(Facet { version, content })
    }

    fn timing_model(&self, form: &Form) -> Result<Facet<TimingModel>, Refusal> {
        let spec = [
            ("version", Occurs::One),
            ("sources", Occurs::Optional),
            ("measured-with", Occurs::Optional),
            ("facts", Occurs::Optional),
            ("costs", Occurs::Optional),
            ("none", Occurs::Optional),
        ];
        let slots = self.slots("timing-model", &form.items()[1..], &spec)?;
        let version = self.version("timing-model version", slots[0][0])?;
        let content = match self.none_of("timing-model", &slots, 5)? {
            Some(why) => Content::None(why),
            None => {
                self.present(
                    "timing-model",
                    &slots,
                    &["sources", "measured-with", "facts", "costs"],
                )?;
                Content::Present(TimingModel {
                    sources: self.paths("timing-model sources", slots[1][0], false)?,
                    measured_with: self.ids(
                        "timing-model measured-with",
                        slots[2][0],
                        Code::Dependency,
                    )?,
                    facts: self.facts("timing-model", slots[3][0])?,
                    costs: slots[4][0].items()[1..]
                        .iter()
                        .map(|c| self.cost(c))
                        .collect::<Result<_, _>>()?,
                })
            }
        };
        Ok(Facet { version, content })
    }

    fn facts(&self, facet: &str, form: &Form) -> Result<Vec<Fact>, Refusal> {
        form.items()[1..]
            .iter()
            .map(|f| self.fact(facet, f))
            .collect()
    }

    /// `(fact <name> yes|no (locator …) (basis "…"))` or `(fact <name> (unknown "why"))` (§2).
    fn fact(&self, facet: &str, form: &Form) -> Result<Fact, Refusal> {
        let field = format!("{facet} fact");
        let malformed = |why: &str| {
            self.refuse(
                Code::Field,
                &field,
                Some(form),
                format!("a malformed fact: {why}"),
            )
        };
        if form.head() != Some("fact") {
            return Err(self.refuse(
                Code::Shape,
                &field,
                Some(form),
                "`facts` holds `(fact …)` forms",
            ));
        }
        let items = form.items();
        let name = match items.get(1) {
            Some(Form::Symbol { name, .. }) if grammar::is_name(name) => name.clone(),
            _ => {
                return Err(malformed(
                    "its name is lowercase letters, digits, `-` and `.`",
                ))
            }
        };
        let field = format!("{facet} fact[{name}]");
        match items.get(2) {
            Some(Form::Symbol { name: word, .. }) if word == "yes" || word == "no" => {
                let slots = self.slots(
                    &field,
                    &items[3..],
                    &[("locator", Occurs::Many), ("basis", Occurs::One)],
                )?;
                self.several(&field, &name, form, &slots[0])?;
                let locators = slots[0]
                    .iter()
                    .map(|l| self.locator(&field, l))
                    .collect::<Result<_, _>>()?;
                let basis =
                    self.text(&format!("{field} basis"), self.only("basis", slots[1][0])?)?;
                Ok(Fact {
                    name,
                    value: FactValue::Known {
                        holds: word == "yes",
                        locators,
                        basis,
                    },
                })
            }
            Some(unknown) if unknown.head() == Some("unknown") && items.len() == 3 => {
                let why = self.text(&format!("{field} unknown"), self.only("unknown", unknown)?)?;
                Ok(Fact {
                    name,
                    value: FactValue::Unknown(why),
                })
            }
            _ => Err(malformed("it is `yes`, `no`, or `(unknown \"why\")`")),
        }
    }

    /// A known fact's locators, as structure (§1, §14.2): at least one; several only on a code fact; no two the
    /// same. Their order among the fact's subforms is [`Ctx::slots`]'s.
    fn several(
        &self,
        field: &str,
        name: &str,
        form: &Form,
        locators: &[&Form],
    ) -> Result<(), Refusal> {
        let Some(first) = locators.first() else {
            return Err(self.refuse(Code::Shape, field, Some(form), "`locator` is missing"));
        };
        if locators.len() > 1 && !is_code_fact(name) {
            return Err(self.refuse(
                Code::Shape,
                field,
                Some(locators[1]),
                format!("several locators on `{name}`, which is not a code fact"),
            ));
        }
        let mut seen = vec![encode(first)];
        for locator in &locators[1..] {
            let this = encode(locator);
            if seen.contains(&this) {
                return Err(self.refuse(
                    Code::Shape,
                    field,
                    Some(locator),
                    "two identical locators of one fact",
                ));
            }
            seen.push(this);
        }
        Ok(())
    }

    /// `(locator (file "…"))`, `(locator (code <id> "…"))` or `(locator (ledger <anchor> "…"))` (§2).
    fn locator(&self, field: &str, form: &Form) -> Result<Locator, Refusal> {
        let field = format!("{field} locator");
        let malformed =
            |form: &Form, why: &str| self.refuse(Code::Locator, &field, Some(form), why.to_owned());
        let inner = self.only("locator", form)?;
        match (inner.head(), inner.items()) {
            (Some("file"), [_, Form::Str { value, .. }]) => {
                if !grammar::is_path(value) {
                    return Err(self.refuse(Code::Source, &field, Some(inner), format!("`{value}` is not a path in §4's normal form")));
                }
                Ok(Locator::File(value.clone()))
            }
            (Some("code"), [_, Form::Symbol { name: id, .. }, Form::Str { value, .. }]) => {
                if !grammar::is_id(id) {
                    return Err(malformed(inner, "a code locator names a record by its id"));
                }
                if !grammar::is_path(value) {
                    return Err(self.refuse(Code::Source, &field, Some(inner), format!("`{value}` is not a path in §4's normal form")));
                }
                Ok(Locator::Code { id: id.clone(), path: value.clone() })
            }
            (Some("ledger"), [_, Form::Symbol { name: anchor, .. }, Form::Str { value, .. }]) => {
                if value.trim().is_empty() {
                    return Err(malformed(inner, "a ledger locator names `\"<revision>, <section>\"`"));
                }
                Ok(Locator::Ledger { anchor: anchor.clone(), detail: value.clone() })
            }
            _ => Err(malformed(inner, "a locator is `(file \"<path>\")`, `(code <id> \"<path>\")` or `(ledger <anchor> \"<detail>\")`")),
        }
    }

    /// A non-negative integer (§2: `0 … 2^63 − 1`).
    fn natural(&self, field: &str, form: &Form) -> Result<u64, Refusal> {
        match form {
            Form::Integer { value, .. } if *value >= 0 => Ok(value.unsigned_abs()),
            Form::Integer { .. } => {
                Err(self.refuse(Code::Field, field, Some(form), "a negative integer"))
            }
            _ => Err(self.refuse(
                Code::Field,
                field,
                Some(form),
                format!("a malformed cost: `{field}` is an integer"),
            )),
        }
    }

    /// A cost, unknown or known (§2).
    fn cost(&self, form: &Form) -> Result<Cost, Refusal> {
        if form.head() != Some("cost") {
            return Err(self.refuse(
                Code::Shape,
                "timing-model costs",
                Some(form),
                "`costs` holds `(cost …)` forms",
            ));
        }
        let items = form.items();
        let name = match items.get(1) {
            Some(Form::Symbol { name, .. }) if grammar::is_name(name) => name.clone(),
            _ => {
                return Err(self.refuse(
                    Code::Field,
                    "timing-model cost",
                    Some(form),
                    "a malformed cost: its name",
                ))
            }
        };
        let field = format!("timing-model cost[{name}]");
        let spec = [
            ("target", Occurs::One),
            ("unknown", Occurs::Optional),
            ("value", Occurs::Optional),
            ("unit", Occurs::Optional),
            ("scope", Occurs::Optional),
            ("holds-for", Occurs::Optional),
            ("holds-under-preemption", Occurs::Optional),
            ("binary", Occurs::Optional),
            ("evidence", Occurs::Optional),
            ("locator", Occurs::Optional),
            ("basis", Occurs::Optional),
        ];
        let slots = self.slots(&field, &items[2..], &spec)?;
        let target_form = self.only("target", slots[0][0])?;
        let target = self.symbol("target", target_form)?;
        if !grammar::is_target_stem(target) {
            return Err(self.refuse(
                Code::Field,
                &field,
                Some(target_form),
                format!("`{target}` is not a target stem"),
            ));
        }
        let target = target.to_owned();
        if let Some(unknown) = slots[1].first() {
            if let Some(extra) = slots[2..].iter().flatten().next() {
                return Err(self.refuse(
                    Code::Shape,
                    &field,
                    Some(extra),
                    "an unknown cost holds its target and its reason, and nothing else",
                ));
            }
            let why = self.text(&format!("{field} unknown"), self.only("unknown", unknown)?)?;
            return Ok(Cost {
                name,
                target,
                value: Err(why),
            });
        }
        for (i, (sub, _)) in spec.iter().enumerate().skip(2) {
            if *sub != "locator" && slots[i].is_empty() {
                return Err(Refusal::new(
                    Code::Shape,
                    self.path,
                    &field,
                    None,
                    format!("`{sub}` is missing from a known cost"),
                ));
            }
        }
        let value = self.natural(&format!("{field} value"), self.only("value", slots[2][0])?)?;
        let unit_form = self.only("unit", slots[3][0])?;
        let unit = match self.symbol("unit", unit_form)? {
            "ns" => Unit::Ns,
            "us" => Unit::Us,
            "ms" => Unit::Ms,
            other => {
                return Err(self.refuse(
                    Code::Field,
                    &field,
                    Some(unit_form),
                    format!("`{other}` is not a unit: `ns`, `us` or `ms`"),
                ))
            }
        };
        let scope = self.text(&format!("{field} scope"), self.only("scope", slots[4][0])?)?;
        let holds_for = self.slots(
            &format!("{field} holds-for"),
            &slots[5][0].items()[1..],
            &[("tasks", Occurs::One), ("sources", Occurs::One)],
        )?;
        let tasks = self.natural(
            &format!("{field} holds-for tasks"),
            self.only("tasks", holds_for[0][0])?,
        )?;
        let sources = self.natural(
            &format!("{field} holds-for sources"),
            self.only("sources", holds_for[1][0])?,
        )?;
        let preemption_form = self.only("holds-under-preemption", slots[6][0])?;
        let holds_under_preemption = match self.symbol("holds-under-preemption", preemption_form)? {
            "yes" => true,
            "no" => false,
            _ => {
                return Err(self.refuse(
                    Code::Field,
                    &field,
                    Some(preemption_form),
                    "a malformed cost: `holds-under-preemption` is `yes` or `no`",
                ))
            }
        };
        let binary = self.binary(&field, &name, slots[7][0])?;
        let evidence = self.evidence(&field, value, slots[8][0])?;
        let locator = slots[9]
            .first()
            .map(|l| self.locator(&field, l))
            .transpose()?;
        if locator.is_none() && evidence != Evidence::Assumed {
            return Err(Refusal::new(
                Code::Locator,
                self.path,
                &field,
                self.at(form.span()),
                "a locator is required unless the evidence is `assumed`",
            ));
        }
        let basis = self.text(&format!("{field} basis"), self.only("basis", slots[10][0])?)?;
        let known = KnownCost {
            value,
            unit,
            scope,
            tasks,
            sources,
            holds_under_preemption,
            binary,
            evidence,
            locator,
            basis,
        };
        if let Err(defect) = bound(&name, &target, &known).validate() {
            return Err(self.refuse(
                Code::Field,
                &field,
                Some(form),
                format!("`Bound::validate` refuses it: {defect}"),
            ));
        }
        Ok(Cost {
            name,
            target,
            value: Ok(known),
        })
    }

    fn binary(&self, field: &str, name: &str, form: &Form) -> Result<Binary, Refusal> {
        match form.items() {
            [_, Form::Str { value, .. }] if grammar::is_hash(value) => Ok(Binary::Image(value.clone())),
            [_, Form::Symbol { name: word, .. }] if word == "unbuilt" => Ok(Binary::Unbuilt),
            [_, Form::Symbol { name: word, .. }, why] if word == "independent" => {
                if !INDEPENDENT_NAMES.contains(&name) {
                    return Err(self.refuse(Code::Field, field, Some(form), "`independent` is admitted only for `compare-rounding` and `delivery`"));
                }
                Ok(Binary::Independent(self.text(&format!("{field} binary"), why)?))
            }
            _ => Err(self.refuse(Code::Field, field, Some(form), "a malformed cost: `binary` is `\"sha256:<hex>\"`, `unbuilt` or `independent \"why\"`")),
        }
    }

    fn evidence(&self, field: &str, value: u64, form: &Form) -> Result<Evidence, Refusal> {
        let malformed = || {
            self.refuse(
                Code::Field,
                field,
                Some(form),
                "a malformed cost: an unknown evidence category",
            )
        };
        let items = form.items();
        let category = items
            .get(1)
            .and_then(Form::as_symbol)
            .ok_or_else(malformed)?;
        match (category, &items[2..]) {
            ("assumed", []) => Ok(Evidence::Assumed),
            ("observed-maximum", []) => Ok(Evidence::ObservedMaximum(None)),
            ("externally-supplied", []) => Ok(Evidence::ExternallySupplied),
            ("analytically-established", []) => Ok(Evidence::AnalyticallyEstablished),
            ("observed-maximum", rest) => {
                let slots = self.slots(
                    &format!("{field} evidence"),
                    rest,
                    &[("observed", Occurs::One), ("safety-factor", Occurs::One)],
                )?;
                let observed = self.natural(
                    &format!("{field} observed"),
                    self.only("observed", slots[0][0])?,
                )?;
                let factor_form = slots[1][0];
                let [_, numerator, denominator] = factor_form.items() else {
                    return Err(self.refuse(
                        Code::Field,
                        field,
                        Some(factor_form),
                        "a safety factor is `(safety-factor <numerator> <denominator>)`",
                    ));
                };
                let term = |f: &Form| -> Result<u32, Refusal> {
                    let n = self.natural(&format!("{field} safety-factor"), f)?;
                    u32::try_from(n).map_err(|_| {
                        self.refuse(
                            Code::Field,
                            field,
                            Some(f),
                            "a safety factor's term is outside `u32`",
                        )
                    })
                };
                let factor = SafetyFactor::new(term(numerator)?, term(denominator)?);
                if !factor.pads() {
                    return Err(self.refuse(
                        Code::Field,
                        field,
                        Some(factor_form),
                        "a safety factor that does not pad",
                    ));
                }
                let padded = (u128::from(observed) * u128::from(factor.numerator))
                    .div_ceil(u128::from(factor.denominator));
                if padded != u128::from(value) {
                    return Err(self.refuse(
                        Code::Field,
                        field,
                        Some(factor_form),
                        format!("`value` is {value}, and the padded observation is {padded}"),
                    ));
                }
                Ok(Evidence::ObservedMaximum(Some((observed, factor))))
            }
            _ => Err(malformed()),
        }
    }

    /// The rules of §12 a record's own text shows (`M2.7.3.5.1`): a fact the facts table names, in a facet the
    /// table does not give it; a code fact with a locator that is not `code`; a `runtime-discipline.<id>` statement named with another record's id; a cost named
    /// `api.completion`, which would make `masked.completion` ambiguous.
    fn section_12(
        &self,
        id: &str,
        forms: [&Form; 2],
        behavior: &Facet<BehaviorModel>,
        timing: &Facet<TimingModel>,
    ) -> Result<(), Refusal> {
        let behavior_facts = match &behavior.content {
            Content::Present(m) => m.facts.as_slice(),
            Content::None(_) => &[],
        };
        let (timing_facts, costs) = match &timing.content {
            Content::Present(m) => (m.facts.as_slice(), m.costs.as_slice()),
            Content::None(_) => (&[][..], &[][..]),
        };
        for (facet, holder, facts) in [
            (FacetKind::BehaviorModel, forms[0], behavior_facts),
            (FacetKind::TimingModel, forms[1], timing_facts),
        ] {
            for fact in facts {
                let form = holder
                    .items()
                    .iter()
                    .filter(|f| f.head() == Some("facts"))
                    .flat_map(|f| f.items().iter().skip(1))
                    .find(|f| matches!(f.items().get(1), Some(Form::Symbol { name, .. }) if *name == fact.name));
                let field = format!("{} fact[{}]", facet.as_str(), fact.name);
                let refuse = |code: Code, message: String| self.refuse(code, &field, form, message);
                if let Some((given, code)) = tabled(&fact.name) {
                    if given != facet {
                        return Err(refuse(
                            Code::Field,
                            format!(
                                "§12's facts table gives `{}` the {} facet",
                                fact.name,
                                given.as_str()
                            ),
                        ));
                    }
                    if let FactValue::Known { locators, .. } = &fact.value {
                        if code && locators.iter().any(|l| !matches!(l, Locator::Code { .. })) {
                            return Err(refuse(
                                Code::Locator,
                                format!(
                                    "`{}` is a fact about code, which takes `code` locators and no other",
                                    fact.name
                                ),
                            ));
                        }
                    }
                }
                if let Some(named) = fact.name.strip_prefix("runtime-discipline.") {
                    if named != id {
                        return Err(refuse(
                            Code::Field,
                            format!(
                                "a record states `runtime-discipline.{id}` about itself, not `{}`",
                                fact.name
                            ),
                        ));
                    }
                }
            }
        }
        if let Some(cost) = costs.iter().find(|c| c.name == "api.completion") {
            let form = forms[1]
                .items()
                .iter()
                .filter(|f| f.head() == Some("costs"))
                .flat_map(|f| f.items().iter().skip(1))
                .find(|f| matches!(f.items().get(1), Some(Form::Symbol { name, .. }) if *name == cost.name));
            return Err(self.refuse(
                Code::Field,
                "timing-model cost[api.completion]",
                form,
                "no primitive is named `completion`, so `masked.completion` is always the completion path's (§12)",
            ));
        }
        Ok(())
    }

    /// The rules of §14.4 one record's text shows (`M2.12.4.3`), its locators before its fields: a fact that takes
    /// a `file` locator with another; a self-named fact named with another record's id; `convention-stated.<id>`
    /// outside a convention record, and `guard-check-contexts.<id>` in a record supplying `switch`; an obligatory
    /// fact's known `no`; a fact stated where its read condition does not hold; `api-trap-preemptible-before-decode`
    /// `yes` beside `primitives-preemptible` `no`; and a convention record's form.
    fn section_14_4(
        &self,
        id: &str,
        catalog: Catalog,
        implementation: &Facet<Packages>,
        holder: &Form,
        behavior: &Facet<BehaviorModel>,
        timing: &Facet<TimingModel>,
    ) -> Result<(), Refusal> {
        let facts = match &behavior.content {
            Content::Present(m) => m.facts.as_slice(),
            Content::None(_) => &[],
        };
        let form_of = |name: &str| {
            holder
                .items()
                .iter()
                .filter(|f| f.head() == Some("facts"))
                .flat_map(|f| f.items().iter().skip(1))
                .find(
                    |f| matches!(f.items().get(1), Some(Form::Symbol { name: n, .. }) if n == name),
                )
        };
        let refuse = |code: Code, name: &str, message: String| {
            self.refuse(
                code,
                &format!("behavior-model fact[{name}]"),
                form_of(name),
                message,
            )
        };
        let known = |name: &str, holds: bool| {
            facts.iter().any(|f| {
                f.name == name && matches!(f.value, FactValue::Known { holds: h, .. } if h == holds)
            })
        };
        for fact in facts {
            if let FactValue::Known { locators, .. } = &fact.value {
                if statement::file_located(&fact.name)
                    && locators.iter().any(|l| !matches!(l, Locator::File(_)))
                {
                    return Err(refuse(
                        Code::Locator,
                        &fact.name,
                        format!(
                            "`{}` takes a `file` locator and no other (§14.4)",
                            fact.name
                        ),
                    ));
                }
            }
        }
        let supplies_switch = matches!(&timing.content, Content::Present(m) if m.costs.iter().any(|c| c.name == "switch"));
        for fact in facts {
            let name = fact.name.as_str();
            if let Some(named) = statement::self_named(name) {
                if named != id {
                    return Err(refuse(
                        Code::Field,
                        name,
                        format!("a record states `{name}` only about itself, `{id}` (§14.4)"),
                    ));
                }
                if name.starts_with(statement::CONVENTION_STATED) && !statement::is_convention(id) {
                    return Err(refuse(
                        Code::Field,
                        name,
                        format!(
                            "only a check-passing convention record, whose id begins `{}`, states `{name}` (§14.4)",
                            statement::CONVENTION_PREFIX
                        ),
                    ));
                }
                if name.starts_with(statement::GUARD_CHECK_CONTEXTS) && supplies_switch {
                    return Err(refuse(
                        Code::Field,
                        name,
                        format!("a record supplying `switch` states `guard-check-contexts`, not `{name}` (§14.4)"),
                    ));
                }
            }
            if statement::OBLIGATORY.contains(&name) && known(name, false) {
                return Err(refuse(
                    Code::Field,
                    name,
                    format!("`{name}` is obligatory: the contract requires it, so a known `no` is refused (§14.4)"),
                ));
            }
            if statement::FACTS.contains(&name) && !statement::condition_holds(name, facts) {
                return Err(refuse(
                    Code::Field,
                    name,
                    format!("`{name}` is stated where its read condition does not hold (§14.4)"),
                ));
            }
        }
        if known("api-trap-preemptible-before-decode", true)
            && known("primitives-preemptible", false)
        {
            return Err(refuse(
                Code::Field,
                "api-trap-preemptible-before-decode",
                "`api-trap-preemptible-before-decode` `yes` beside `primitives-preemptible` `no`: the contract's rule 2 makes a primitive's trap the primitive (§14.4)".to_owned(),
            ));
        }
        if statement::is_convention(id) {
            let own = format!("{}{id}", statement::CONVENTION_STATED);
            let refuse = |field: &str, message: &str| {
                Refusal::new(
                    Code::Field,
                    self.path,
                    field,
                    None,
                    format!("a check-passing convention record {message} (§14.4)"),
                )
            };
            if catalog != Catalog::Interfaces {
                return Err(refuse("catalog", "is in the `interfaces` catalog"));
            }
            if !matches!(implementation.content, Content::None(_)) {
                return Err(refuse("implementation", "has implementation `none`"));
            }
            let stated = matches!(facts, [f] if f.name == own && matches!(f.value, FactValue::Known { holds: true, .. }));
            if !stated {
                return Err(refuse(
                    "behavior-model",
                    &format!("states exactly one fact, `{own}`, `yes`"),
                ));
            }
        }
        Ok(())
    }

    /// Facts and costs share one name space per record: a fact name once, a cost name once per target (§2).
    fn names(
        &self,
        behavior: &Facet<BehaviorModel>,
        timing: &Facet<TimingModel>,
    ) -> Result<(), Refusal> {
        let mut facts: Vec<&str> = Vec::new();
        let behavior_facts = match &behavior.content {
            Content::Present(m) => m.facts.as_slice(),
            Content::None(_) => &[],
        };
        let (timing_facts, costs) = match &timing.content {
            Content::Present(m) => (m.facts.as_slice(), m.costs.as_slice()),
            Content::None(_) => (&[][..], &[][..]),
        };
        for fact in behavior_facts.iter().chain(timing_facts) {
            if facts.contains(&fact.name.as_str()) {
                return Err(Refusal::new(
                    Code::Shape,
                    self.path,
                    &format!("fact[{}]", fact.name),
                    None,
                    "a repeated name: a fact name appears once per record",
                ));
            }
            facts.push(&fact.name);
        }
        let mut seen: Vec<(&str, &str)> = Vec::new();
        for cost in costs {
            if facts.contains(&cost.name.as_str()) {
                return Err(Refusal::new(
                    Code::Shape,
                    self.path,
                    &format!("cost[{}]", cost.name),
                    None,
                    "a repeated name: facts and costs share one name space",
                ));
            }
            if seen.contains(&(cost.name.as_str(), cost.target.as_str())) {
                return Err(Refusal::new(
                    Code::Shape,
                    self.path,
                    &format!("cost[{}]", cost.name),
                    None,
                    "a repeated name: a cost name appears once per target",
                ));
            }
            seen.push((&cost.name, &cost.target));
        }
        Ok(())
    }

    /// `(review (facet …) (hash "…") (verdict …) (answers "…" …) (by <role> "<who>") (date "…") (basis "…"))`.
    fn review(&self, form: &Form, maintainer: &str) -> Result<Review, Refusal> {
        let field = "review";
        let spec = [
            ("facet", Occurs::One),
            ("hash", Occurs::One),
            ("verdict", Occurs::One),
            ("answers", Occurs::Optional),
            ("by", Occurs::One),
            ("date", Occurs::One),
            ("basis", Occurs::One),
        ];
        let slots = self.slots(field, &form.items()[1..], &spec)?;
        let facet_form = self.only("facet", slots[0][0])?;
        let facet = FacetKind::parse(self.symbol("facet", facet_form)?).ok_or_else(|| {
            self.refuse(
                Code::Review,
                field,
                Some(facet_form),
                "its facet is not one of the four",
            )
        })?;
        let hash_form = self.only("hash", slots[1][0])?;
        let hash = self.string("hash", hash_form)?;
        if !grammar::is_hash(hash) {
            return Err(self.refuse(
                Code::Review,
                field,
                Some(hash_form),
                "its hash is not written as §3 requires",
            ));
        }
        let verdict_form = self.only("verdict", slots[2][0])?;
        let written = self.symbol("verdict", verdict_form)?;
        let verdict = Verdict::parse(written).ok_or_else(|| {
            self.refuse(
                Code::Field,
                field,
                Some(verdict_form),
                format!("`{written}` is not a verdict"),
            )
        })?;
        let mut answers = Vec::new();
        if let Some(a) = slots[3].first() {
            if verdict == Verdict::Rejected {
                return Err(self.refuse(
                    Code::Review,
                    field,
                    Some(a),
                    "`answers` is allowed only on a `production` verdict",
                ));
            }
            for item in &a.items()[1..] {
                let h = self.string("answers", item)?;
                if !grammar::is_hash(h) {
                    return Err(self.refuse(
                        Code::Review,
                        field,
                        Some(item),
                        "an answered hash is not written as §3 requires",
                    ));
                }
                answers.push(h.to_owned());
            }
        }
        let by = slots[4][0];
        let [_, role_form, who_form] = by.items() else {
            return Err(self.refuse(
                Code::Shape,
                field,
                Some(by),
                "`by` is `(by <role> \"<who>\")`",
            ));
        };
        let role = match self.symbol("by", role_form)? {
            "director" => Role::Director,
            "independent-context" => Role::IndependentContext,
            "external" => Role::External,
            other => {
                return Err(self.refuse(
                    Code::Field,
                    field,
                    Some(role_form),
                    format!("`{other}` is not a role"),
                ))
            }
        };
        let who = self.string("by", who_form)?;
        if who.trim().is_empty() {
            return Err(self.refuse(Code::Review, field, Some(who_form), "its `who` is empty"));
        }
        if who == maintainer {
            return Err(self.refuse(
                Code::Review,
                field,
                Some(who_form),
                "its `who` is the maintainer's tree id",
            ));
        }
        let date_form = self.only("date", slots[5][0])?;
        let date = grammar::date(self.string("date", date_form)?).ok_or_else(|| {
            self.refuse(
                Code::Review,
                field,
                Some(date_form),
                "its date is not a real calendar date",
            )
        })?;
        let basis_form = self.only("basis", slots[6][0])?;
        let basis = self.string("basis", basis_form)?;
        if basis.trim().is_empty() {
            return Err(self.refuse(Code::Review, field, Some(basis_form), "its basis is empty"));
        }
        Ok(Review {
            facet,
            hash: hash.to_owned(),
            verdict,
            answers,
            role,
            who: who.to_owned(),
            date,
            basis: basis.to_owned(),
            form: form.clone(),
        })
    }
}

/// The `archogen_evidence::Bound` a known cost becomes (§2).
#[must_use]
pub fn bound(name: &str, target: &str, cost: &KnownCost) -> Bound {
    let located = |basis: &str| match &cost.locator {
        Some(l) => format!("{}: {basis}", l.render()),
        None => basis.to_owned(),
    };
    let origin = match &cost.evidence {
        Evidence::Assumed => BoundOrigin::Assumed {
            rationale: located(&cost.basis),
        },
        Evidence::ObservedMaximum(factor) => BoundOrigin::ObservedMaximum {
            conditions: located(&cost.basis),
            safety_factor: factor.map(|(_, f)| f),
        },
        Evidence::ExternallySupplied => BoundOrigin::ExternallySupplied {
            source: cost
                .locator
                .as_ref()
                .map(Locator::render)
                .unwrap_or_default(),
        },
        Evidence::AnalyticallyEstablished => BoundOrigin::AnalyticallyEstablished {
            argument: located(&cost.basis),
        },
    };
    Bound {
        quantity: name.to_owned(),
        value: cost.value,
        unit: cost.unit.as_str().to_owned(),
        scope: cost.scope.clone(),
        target: target.to_owned(),
        binary: match &cost.binary {
            Binary::Image(hash) => hash.clone(),
            Binary::Unbuilt => "unbuilt".to_owned(),
            Binary::Independent(why) => format!("independent: {why}"),
        },
        origin,
    }
}
