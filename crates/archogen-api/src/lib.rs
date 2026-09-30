//! The engine API — `ROADMAP.md` §10.4, `docs/decisions/decision_engine-api.md`.
//!
//! One declared, versioned, transport-neutral contract: a description as text plus a profile in, a
//! [`Response`] out. The command line, the wasm binding and the MCP server are consumers of it, not
//! implementations of it, so a capability that exists behind one of them exists behind all of them.
//!
//! ⭐ **Every response carries a [`Status`]**, and there is no response without one: the field is not
//! optional and the type has no default. When the description was judged, the status is its `ROADMAP.md`
//! §5.5 verdict and [`Response::judged`] says what it was judged under. When it was not — a profile nobody
//! supports, a kind module, an import that exists and cannot be read, a language definition that failed to
//! load — the status says so and [`Response::notes`] say why. It is never absent and never `ok`, which is
//! how an unanswered question would otherwise become `established` in a consumer that never saw the
//! distinction.
//!
//! It does no I/O. A description arrives as text, and its imports resolve through the [`ModuleSource`] the
//! caller supplies — a directory for the CLI, [`MemoryModules`] for a consumer that has no filesystem. It
//! renders nothing either: a consumer prints or serializes the structure it gets back.

mod status;

use std::cell::{Cell, RefCell};
use std::fmt;

use eadl_front::{elaborate_source, read, Position, Verdict};
use eadl_model::check::{check_program, default_profile, shipped_registry, Outcome};
use eadl_model::profile::{self, Profile};

pub use eadl_front::{Diagnostic, Form, MemoryModules, ModuleSource, SourceId, SourceMap};
pub use eadl_model::check::Closure;
pub use status::Status;

/// This API's version: apart from the language's (`eadl/1`) and the profiles', as `ROADMAP.md` §15 requires.
///
/// **The promise** (`docs/decisions/decision_engine-api.md`): within a major, an operation is never removed,
/// a response field is never removed or given a new meaning, and the status vocabulary only grows. Adding an
/// operation or a field is a minor; anything else is a new major. A description whose verdict changes under
/// the same language version is a language change, recorded in `docs/semantics/migrations/`, not an API
/// change. `1.0` was fixed `2026-09-30`, when leaf `API.3` closed; `1.1` added [`Response::engine`] (leaf
/// `API.4.1`); `1.2` added [`check_with`] and [`Limits`] (leaf `API.4.2`). `docs/book/src/versions.md` registers it.
pub const VERSION: Version = Version { major: 1, minor: 2 };

/// The engine version: the one the workspace's members share, which `archogen --version` reports and
/// `docs/book/src/versions.md` registers as `engine`. With [`VERSION`] it names the build a response came from
/// (`docs/decisions/decision_api-instance.md`).
pub const ENGINE: &str = env!("CARGO_PKG_VERSION");

/// A major and a minor version.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Version {
    /// Changes when a promise of the previous major is broken.
    pub major: u32,
    /// Changes when an operation or a response field is added.
    pub minor: u32,
}

impl fmt::Display for Version {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// The operations this API exposes, each by the name `crates/archogen-cli/src/spec.rs` gives its command.
/// Parity with the CLI's built commands is gated on it (leaf `API.3.3`).
pub const OPERATIONS: &[&str] = &["check"];

/// The leaf that loads a kind module a user writes (`ROADMAP.md` §5.6 rule 1): extension experiment 1, a new
/// kind built from the existing declaration constructs.
pub const KIND_MODULE_OWNER: &str = "M6.3";

/// The kind modules shipped with the toolchain: the language definition every consumer checks against.
///
/// Embedded at compile time rather than read from anywhere: a language definition that a file in some
/// directory could shadow is one an accident can change, and a consumer with no filesystem needs it too.
const KIND_MODULES: &[(&str, &str)] = &[
    (
        "docs/semantics/kinds/core.eadl",
        include_str!("../../../docs/semantics/kinds/core.eadl"),
    ),
    (
        "docs/semantics/kinds/os-rt.eadl",
        include_str!("../../../docs/semantics/kinds/os-rt.eadl"),
    ),
];

/// The shipped kind modules, owned so a [`SourceMap`] can take them.
#[must_use]
pub fn kind_modules() -> Vec<(String, String)> {
    KIND_MODULES
        .iter()
        .map(|(name, text)| ((*name).to_string(), (*text).to_string()))
        .collect()
}

/// The default budget for one request: 1 MiB of text, the description and every module elaboration loads
/// counted together (leaf `API.4.2`).
///
/// ⭐ **Why one number is enough.** The language bounds the shapes that multiply work — list nesting (`M1.38`),
/// module instances and import chains (`M1.39`) — and what is left is linear in the input. Measured `2026-09-30`
/// on a debug build: 1 MB of description checked in 0.36 s holding 82 MB, and 5 MB in 1.95 s holding 383 MB. So a
/// byte budget bounds a request's time and memory. The largest description the repository holds is 5 KB.
pub const DEFAULT_BYTES: usize = 1024 * 1024;

/// What an instance lets one request cost. The **instance** sets it, never the consumer: its purpose is to protect
/// the host from what a consumer sends (`docs/decisions/decision_api-instance.md`).
///
/// A budget only ever changes an answer into `tool-failure`: a request within it gets exactly the answer it would
/// get with no budget, so a response that is not a refusal does not depend on it.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Limits {
    /// The most text one request may bring, the description and its modules together; `None` is no budget.
    pub bytes: Option<usize>,
}

impl Limits {
    /// The budget [`check`] applies: [`DEFAULT_BYTES`].
    pub const DEFAULT: Self = Self {
        bytes: Some(DEFAULT_BYTES),
    };
    /// No budget: for a consumer that trusts what it sends, as the CLI does with the files it was given.
    pub const NONE: Self = Self { bytes: None };
}

/// A module source that counts what it hands out against a budget, and stops handing out past it.
struct Budgeted<'a> {
    inner: &'a dyn ModuleSource,
    remaining: Cell<usize>,
    /// The first module that would have gone past the budget: its name, its size, and what was left.
    over: RefCell<Option<(String, usize, usize)>>,
}

impl ModuleSource for Budgeted<'_> {
    fn load(&self, module: &str) -> Option<(String, String)> {
        if self.over.borrow().is_some() {
            return None;
        }
        let (name, text) = self.inner.load(module)?;
        if text.len() > self.remaining.get() {
            *self.over.borrow_mut() = Some((module.to_string(), text.len(), self.remaining.get()));
            return None;
        }
        self.remaining.set(self.remaining.get() - text.len());
        Some((name, text))
    }

    fn describe_missing(&self, module: &str) -> String {
        self.inner.describe_missing(module)
    }

    fn unreadable(&self) -> Vec<(String, String)> {
        self.inner.unreadable()
    }
}

/// What a consumer asks: one description, the profile to judge it against, and where its imports resolve.
pub struct Request<'a> {
    /// How spans and notes name the description — a path for the CLI, any label for another consumer.
    pub name: &'a str,
    /// The description's text.
    pub text: &'a str,
    /// The profile's identifier; `None` is the default profile, `rt-static-up-v1`.
    pub profile: Option<&'a str>,
    /// Where an import resolves. [`NoModules`] for a description that imports nothing.
    pub modules: &'a dyn ModuleSource,
}

/// A source that holds no module: every import is `module-not-found`.
#[derive(Debug, Clone, Copy, Default)]
pub struct NoModules;

impl ModuleSource for NoModules {
    fn load(&self, _module: &str) -> Option<(String, String)> {
        None
    }
}

/// What the API answers.
#[derive(Debug)]
pub struct Response {
    /// The API version that produced this response.
    pub version: Version,
    /// The engine version that produced it: with `version`, the build a response is reproducible against.
    pub engine: &'static str,
    /// The outcome: a §5.5 verdict when the description was judged, and otherwise what happened to the
    /// request. Always present.
    pub status: Status,
    /// Statements about the request that have no span, each one line. Empty when the description was judged.
    pub notes: Vec<String>,
    /// The repair for what the notes state, when there is one.
    pub hint: Option<String>,
    /// Every diagnostic, in pass order, with its code, spans and repair direction.
    pub diagnostics: Vec<Diagnostic>,
    /// The sources the diagnostics' spans point into: the description, its modules, and the kind modules.
    pub sources: SourceMap,
    /// Present exactly when the description was judged: what it was judged under, and what was read.
    pub judged: Option<Judgement>,
}

/// What a judged description was judged under, and what the judgement read.
#[derive(Debug)]
pub struct Judgement {
    /// The §5.5 verdict; [`Response::status`] is its projection.
    pub verdict: Verdict,
    /// The language version (`docs/semantics/reference.md` §8).
    pub language: &'static str,
    /// The profile's identifier.
    pub profile: &'static str,
    /// The description, in [`Response::sources`].
    pub description: SourceId,
    /// The declarations that were read, whether or not the check passed; empty for a module tree that did
    /// not compose.
    pub declarations: Vec<Form>,
    /// For a module tree, its instances as `<alias path> = <module> <version>`, the root as `(root)`.
    pub instances: Option<Vec<String>>,
    /// The closure boundary: what is inside and what pulled each fact in, and what is outside.
    pub closure: Closure,
}

impl Response {
    /// Whether the description was judged and accepted.
    #[must_use]
    pub fn is_ok(&self) -> bool {
        self.status == Status::Ok
    }

    /// Every diagnostic, rendered with its source excerpt and joined as `eadl_model::check::Outcome::render`
    /// joins them, so a consumer that prints it prints what the CLI always has.
    #[must_use]
    pub fn render_diagnostics(&self) -> String {
        self.diagnostics
            .iter()
            .map(|diagnostic| diagnostic.render(&self.sources))
            .collect::<Vec<_>>()
            .join("\n")
    }

    fn not_judged(
        status: Status,
        sources: SourceMap,
        notes: Vec<String>,
        hint: Option<String>,
        diagnostics: Vec<Diagnostic>,
    ) -> Self {
        Self {
            version: VERSION,
            engine: ENGINE,
            status,
            notes,
            hint,
            diagnostics,
            sources,
            judged: None,
        }
    }

    fn judged(
        outcome: Outcome,
        instances: Option<Vec<String>>,
        active: &Profile,
        description: SourceId,
        sources: SourceMap,
    ) -> Self {
        Self {
            version: VERSION,
            engine: ENGINE,
            status: Status::from_verdict(outcome.verdict),
            notes: Vec::new(),
            hint: None,
            diagnostics: outcome.diagnostics,
            sources,
            judged: Some(Judgement {
                verdict: outcome.verdict,
                language: eadl_front::language_version::EADL_1,
                profile: active.id,
                description,
                declarations: outcome.declarations,
                instances,
                closure: outcome.closure,
            }),
        }
    }
}

/// Check one description: read it, and elaborate it when it is a module file; resolve, type-check and admit
/// it against the profile; and answer with every finding.
///
/// The order is the one that makes each answer true. The profile is resolved first, because a description
/// checked against a profile nobody supports has not been checked. A file that does not read gets the read
/// pass's verdict, whatever it was meant to be. A module file is elaborated from `request.modules`, and an
/// import that exists and cannot be read fails the request rather than being called missing
/// (`docs/semantics/reference.md` §6 rule 7). A kind module is not judged at all (leaf `M1.32`).
#[must_use]
pub fn check(request: &Request<'_>) -> Response {
    check_with(request, Limits::DEFAULT)
}

/// [`check`], within `limits`, which the instance chooses (leaf `API.4.2`). A request that would bring more text
/// than the budget is `tool-failure`: a note names the budget and what went past it, and nothing is said about the
/// description, because a judgement cut short is not one.
#[must_use]
pub fn check_with(request: &Request<'_>, limits: Limits) -> Response {
    if let Some(budget) = limits.bytes {
        if request.text.len() > budget {
            return over_budget(format!(
                "{} is {} bytes, over this instance's budget of {budget} bytes for one request",
                request.name,
                request.text.len()
            ));
        }
        let budgeted = Budgeted {
            inner: request.modules,
            remaining: Cell::new(budget - request.text.len()),
            over: RefCell::new(None),
        };
        let response = judge(&Request {
            modules: &budgeted,
            ..*request
        });
        if let Some((module, size, left)) = budgeted.over.into_inner() {
            return over_budget(format!(
                "the modules {} imports come to more than this instance's budget of {budget} bytes for one \
                 request: `{module}` is {size} bytes, and {left} were left",
                request.name
            ));
        }
        return response;
    }
    judge(request)
}

/// The response for a request over its budget: `tool-failure`, never a partial result.
fn over_budget(note: String) -> Response {
    Response::not_judged(
        Status::ToolFailure,
        SourceMap::new(),
        vec![note],
        Some(
            "this says nothing about the description: the instance declined to spend more on it. Split it, \
             or ask an instance with a larger budget"
                .to_string(),
        ),
        Vec::new(),
    )
}

/// The judgement itself, with no budget.
fn judge(request: &Request<'_>) -> Response {
    if let Some(refused) = refuse_profile(request.profile) {
        return refused;
    }
    let mut sources = SourceMap::new();
    let active = profile::supported(request.profile.unwrap_or(default_profile().id))
        .expect("refuse_profile answered None, so the profile is supported");
    let registry = match shipped_registry(&mut sources, &kind_modules()) {
        Ok(registry) => registry,
        // A broken language definition is a fault in the toolchain, not a verdict about the description.
        // §5.5: never reported as a valid system.
        Err(diagnostics) => {
            return Response::not_judged(
                Status::ToolFailure,
                sources,
                vec!["the shipped kind modules could not be loaded".to_string()],
                None,
                diagnostics,
            )
        }
    };
    let Ok(id) = sources.add(request.name.to_string(), request.text.to_string()) else {
        return Response::not_judged(
            Status::ToolFailure,
            sources,
            vec![format!("{} is too large to address", request.name)],
            None,
            Vec::new(),
        );
    };

    if !is_module_file(&sources, id) {
        if let Some(kind) = kind_module(&sources, id) {
            return refuse_kind_module(request.name, &kind, sources);
        }
        let outcome = eadl_model::check::check(&sources, id, &registry, active);
        return Response::judged(outcome, None, active, id, sources);
    }

    let (program, diagnostics) = elaborate_source(&mut sources, request.modules, id);
    let unreadable = request.modules.unreadable();
    if !unreadable.is_empty() {
        let notes = unreadable
            .iter()
            .map(|(file, error)| {
                format!(
                    "cannot read {file}, which {} imports: {error}",
                    request.name
                )
            })
            .collect();
        return Response::not_judged(
            Status::Usage,
            sources,
            notes,
            Some(
                "the module exists and could not be read — check its permissions and that it is \
                 UTF-8 text"
                    .to_string(),
            ),
            Vec::new(),
        );
    }
    if diagnostics.has_errors() {
        // A composition problem is a verdict about the description, carried by each code through the one
        // accessor every consumer shares — and nothing past elaboration, because the passes cannot read a
        // tree that did not compose.
        let verdict = diagnostics
            .items()
            .iter()
            .map(|diagnostic| Verdict::of_code(diagnostic.code))
            .max_by_key(|verdict| verdict.precedence())
            .unwrap_or(Verdict::InvalidDescription);
        let outcome = Outcome {
            verdict,
            diagnostics: diagnostics.items().to_vec(),
            declarations: Vec::new(),
            closure: Closure::default(),
        };
        return Response::judged(outcome, None, active, id, sources);
    }
    let instances = program
        .instances
        .iter()
        .map(|instance| {
            let name = if instance.path.is_empty() {
                "(root)"
            } else {
                instance.path.as_str()
            };
            format!("{name} = {} {}", instance.module, instance.version)
        })
        .collect();
    let outcome = check_program(&program, &registry, active);
    Response::judged(outcome, Some(instances), active, id, sources)
}

/// The response for a profile nobody supports, or `None` when `requested` names one this build supports —
/// `None` itself names the default profile.
///
/// [`check`] asks it first, because a description checked against a profile nobody supports has not been
/// checked. It is public for a consumer that must refuse before doing anything else: the CLI asks before it
/// reads a file, since an unreadable file reported after an unsupported profile would send the author to fix
/// the wrong thing. Either way the refusal has one wording.
#[must_use]
pub fn refuse_profile(requested: Option<&str>) -> Option<Response> {
    let requested = requested.unwrap_or(default_profile().id);
    if profile::supported(requested).is_some() {
        return None;
    }
    let known: Vec<&str> = profile::SUPPORTED.iter().map(|p| p.id).collect();
    Some(Response::not_judged(
        Status::UnsupportedProfile,
        SourceMap::new(),
        vec![format!("`{requested}` is not a supported profile")],
        Some(format!("this build supports {}", known.join(", "))),
        Vec::new(),
    ))
}

/// Whether the description at `id` is a module file, in the sense of `docs/semantics/reference.md` §6.
///
/// A file is a module when **any** of its declarations is a `(defmodule …)`, not only the first. §6 makes a
/// module file hold exactly one; a file holding one beside other declarations is still a module file — a
/// malformed one, `module-multiple-forms`, which the elaborator reports — and classifying by the first
/// declaration alone would hand that file to the schema pass to be told `defmodule` is not a kind.
/// Declarations and not forms, because the language-version identifier may precede the module (§8) and is
/// not one. A file that does not **read** is not classified at all, so the read pass reports it.
#[must_use]
pub fn is_module_file(sources: &SourceMap, id: SourceId) -> bool {
    let (document, diagnostics) = read(sources, id);
    !diagnostics.has_errors()
        && document
            .declarations()
            .any(|form| form.head() == Some("defmodule"))
}

/// The first `(defkind …)` of a file that declares one: the kind it names, and where.
#[derive(Debug)]
pub struct KindModule {
    /// The kind's head symbol, when the declaration names one.
    pub name: Option<String>,
    /// Where the `(defkind …)` begins.
    pub position: Position,
}

/// The first kind the description at `id` declares, if it declares any (leaf `M1.32`), by the rule
/// [`is_module_file`] uses and for its reason. A file that does not read is not classified.
#[must_use]
pub fn kind_module(sources: &SourceMap, id: SourceId) -> Option<KindModule> {
    let (document, diagnostics) = read(sources, id);
    if diagnostics.has_errors() {
        return None;
    }
    let form = document
        .declarations()
        .find(|form| form.head() == Some("defkind"))?;
    let source = sources.get(id)?;
    Some(KindModule {
        name: form
            .items()
            .get(1)
            .and_then(|name| name.as_symbol())
            .map(str::to_string),
        position: source.position(form.span().start),
    })
}

/// A kind module is not judged: the toolchain loads only the kind modules it ships (leaf `M1.32`). Nor is it
/// validated alone, because a kind that redefines a shipped one is well-formed by itself and clashes only when
/// both are loaded, so a standalone `ok` would be one the tool cannot stand behind.
fn refuse_kind_module(name: &str, kind: &KindModule, sources: SourceMap) -> Response {
    let declared = kind.name.as_deref().map_or_else(
        || "(defkind …)".to_string(),
        |name| format!("(defkind {name} …)"),
    );
    Response::not_judged(
        Status::Unimplemented,
        sources,
        vec![format!(
            "{name}:{}:{} declares a kind, `{declared}`, and no command loads a kind module a user writes yet",
            kind.position.line, kind.position.column
        )],
        Some(format!(
            "the kinds a description may use are the ones this toolchain ships, embedded in the binary and \
             checked on every run (docs/semantics/kinds/). Loading a kind you write is task-tree leaf \
             {KIND_MODULE_OWNER} (docs/TASK_TREE.md); until then, check a description that uses the shipped \
             kinds"
        )),
        Vec::new(),
    )
}
