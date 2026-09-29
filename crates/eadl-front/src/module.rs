//! Modules, imports, and elaboration into a program.
//!
//! `ROADMAP.md` §5.1.1 sets the contract:
//!
//! > An eADL system imports reusable sub-HW and sub-OS descriptions with namespaces, explicit
//! > exports, typed parameters, and version constraints. **Instantiate a module more than once
//! > without sharing mutable elaboration state.** Diagnose circular imports, conflicting
//! > exports, contradictory requirements, and incompatible feature versions.
//!
//! # Instances, not modules
//!
//! The sentence above is the design. Elaboration does not produce a graph of modules; it
//! produces a tree of **instances**. Importing `platform.timer` twice with different parameters
//! yields two instances with their own bindings and their own qualified names, and neither can
//! observe the other. A design that cached one elaborated module per name would be smaller and
//! would silently make the second import a no-op — the failure being that a system with two
//! timers has one.
//!
//! # Errors are collected, not thrown
//!
//! A description with three composition problems should cost one edit cycle. The only failure
//! that stops elaboration is a cycle, because continuing into it does not terminate.

use std::collections::BTreeMap;

use crate::diagnostic::{Diagnostic, Diagnostics, Label};
use crate::form::{Document, Form};
use crate::reader::read;
use crate::source::{SourceMap, Span};

/// A module version: `major.minor`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Version {
    /// Incompatible changes.
    pub major: u32,
    /// Compatible additions.
    pub minor: u32,
}

impl Version {
    /// A version.
    #[must_use]
    pub const fn new(major: u32, minor: u32) -> Self {
        Self { major, minor }
    }

    /// Whether this version satisfies a requirement.
    ///
    /// Same major, at least the required minor. §15: "A source description retains its meaning
    /// under its locked semantic version" — so a major bump is never silently accepted, however
    /// much newer it is.
    #[must_use]
    pub const fn satisfies(self, required: Self) -> bool {
        self.major == required.major && self.minor >= required.minor
    }
}

impl core::fmt::Display for Version {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{}.{}", self.major, self.minor)
    }
}

/// A name a module makes visible to its importers.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Export {
    /// The exported name.
    pub name: String,
    /// Where it was declared.
    pub span: Span,
}

/// A parameter a module accepts.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParamDecl {
    /// The parameter's name.
    pub name: String,
    /// Its default value, if it has one. Without a default it must be supplied.
    pub default: Option<Form>,
    /// Where it was declared.
    pub span: Span,
}

/// One `(import …)` clause.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ImportDecl {
    /// The module being imported.
    pub module: String,
    /// The local namespace it is bound to.
    pub alias: String,
    /// The minimum version required, if stated.
    pub required_version: Option<Version>,
    /// Parameter bindings, in source order.
    pub arguments: Vec<(String, Form)>,
    /// Where the clause is.
    pub span: Span,
}

/// One parsed module.
#[derive(Debug, Clone)]
pub struct ModuleDecl {
    /// Its dotted name.
    pub name: String,
    /// Its version.
    pub version: Version,
    /// What it makes visible.
    pub exports: Vec<Export>,
    /// What it accepts.
    pub params: Vec<ParamDecl>,
    /// What it imports.
    pub imports: Vec<ImportDecl>,
    /// Everything else — the declarations themselves.
    pub declarations: Vec<Form>,
    /// The whole `(defmodule …)` form's span.
    pub span: Span,
}

impl ModuleDecl {
    /// The names this module declares, from each declaration's name position.
    #[must_use]
    pub fn declared_names(&self) -> Vec<&str> {
        self.declarations
            .iter()
            .filter_map(|form| form.items().get(1).and_then(Form::as_symbol))
            .collect()
    }
}

/// Where module text comes from.
///
/// A trait rather than a directory, so composition can be tested without a filesystem — and so
/// a future locked build can serve modules from a content-addressed store without changing
/// elaboration.
pub trait ModuleSource {
    /// Return `(display name, text)` for a module, or `None` if it is not available.
    fn load(&self, module: &str) -> Option<(String, String)>;
}

/// Modules held in memory, keyed by name.
#[derive(Debug, Clone, Default)]
pub struct MemoryModules {
    entries: BTreeMap<String, String>,
}

impl MemoryModules {
    /// An empty set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a module's text.
    #[must_use]
    pub fn with(mut self, module: &str, text: &str) -> Self {
        self.entries.insert(module.to_string(), text.to_string());
        self
    }
}

impl ModuleSource for MemoryModules {
    fn load(&self, module: &str) -> Option<(String, String)> {
        self.entries
            .get(module)
            .map(|text| (format!("{module}.eadl"), text.clone()))
    }
}

/// One elaborated instance of a module.
#[derive(Debug, Clone)]
pub struct Instance {
    /// Its position in [`Program::instances`].
    pub id: usize,
    /// The module it instantiates.
    pub module: String,
    /// Its version.
    pub version: Version,
    /// The dotted path of aliases from the root, e.g. `soc.timer`. Empty for the root.
    pub path: String,
    /// Resolved parameter bindings, sorted by name.
    pub bindings: Vec<(String, Form)>,
    /// The instance ids this one imports, in source order.
    pub imports: Vec<usize>,
    /// The declarations this instance contributes.
    pub declarations: Vec<Form>,
}

impl Instance {
    /// A declaration's fully qualified name: the instance path and the local name.
    #[must_use]
    pub fn qualify(&self, name: &str) -> String {
        if self.path.is_empty() {
            name.to_string()
        } else {
            format!("{}.{name}", self.path)
        }
    }
}

/// An elaborated program.
#[derive(Debug, Clone)]
pub struct Program {
    /// Every instance, children before parents, root last.
    pub instances: Vec<Instance>,
}

impl Program {
    /// The root instance.
    ///
    /// # Panics
    ///
    /// Only if the program has no instances, which elaboration never produces.
    #[must_use]
    pub fn root(&self) -> &Instance {
        self.instances.last().expect("a program has a root")
    }

    /// Every declaration, fully qualified, in elaboration order.
    #[must_use]
    pub fn qualified_declarations(&self) -> Vec<(String, &Form)> {
        let mut out = Vec::new();
        for instance in &self.instances {
            for form in &instance.declarations {
                let name = form
                    .items()
                    .get(1)
                    .and_then(Form::as_symbol)
                    .unwrap_or("<unnamed>");
                out.push((instance.qualify(name), form));
            }
        }
        out
    }

    /// How many instances were created.
    #[must_use]
    pub fn len(&self) -> usize {
        self.instances.len()
    }

    /// Whether the program is empty. Elaboration never produces one.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.instances.is_empty()
    }
}

/// Parse one `(defmodule …)` form.
///
/// Collects every problem rather than the first: a malformed module header usually has more
/// than one thing wrong with it.
fn read_module(form: &Form, diagnostics: &mut Diagnostics) -> Option<ModuleDecl> {
    if form.head() != Some("defmodule") {
        diagnostics.push(Diagnostic::error(
            "module-not-a-module",
            format!(
                "expected a `defmodule` declaration, found `{}`",
                form.head().unwrap_or(form.kind())
            ),
            Label::new(form.span(), "not a module"),
            "a module file begins `(defmodule <name> (version <major> <minor>) …)`",
        ));
        return None;
    }

    let items = form.items();
    let Some(name) = items.get(1).and_then(Form::as_symbol) else {
        diagnostics.push(Diagnostic::error(
            "module-missing-name",
            "a module must be named",
            Label::new(form.span(), "no module name"),
            "write `(defmodule platform.timer (version 1 0) …)`",
        ));
        return None;
    };

    let mut version = None;
    let mut exports = Vec::new();
    let mut params: Vec<ParamDecl> = Vec::new();
    let mut imports: Vec<ImportDecl> = Vec::new();
    let mut declarations = Vec::new();

    for item in items.iter().skip(2) {
        match item.head() {
            Some("version") => {
                let parts = item.items();
                match (parts.get(1), parts.get(2)) {
                    (
                        Some(Form::Integer { value: major, .. }),
                        Some(Form::Integer { value: minor, .. }),
                    ) => {
                        version = Some(Version::new(
                            u32::try_from(*major).unwrap_or(0),
                            u32::try_from(*minor).unwrap_or(0),
                        ));
                    }
                    _ => diagnostics.push(Diagnostic::error(
                        "module-bad-version",
                        "`version` takes a major and a minor integer",
                        Label::new(item.span(), "expected `(version <major> <minor>)`"),
                        "write `(version 1 0)`",
                    )),
                }
            }
            Some("export") => {
                for exported in item.items().iter().skip(1) {
                    match exported.as_symbol() {
                        Some(name) => exports.push(Export {
                            name: name.to_string(),
                            span: exported.span(),
                        }),
                        None => diagnostics.push(Diagnostic::error(
                            "module-bad-export",
                            format!("an export must be a name, found a {}", exported.kind()),
                            Label::new(exported.span(), "not a name"),
                            "write `(export timer.counter timer.compare)`",
                        )),
                    }
                }
            }
            Some("param") => {
                let parts = item.items();
                let Some(param_name) = parts.get(1).and_then(Form::as_symbol) else {
                    diagnostics.push(Diagnostic::error(
                        "module-bad-param",
                        "a parameter must be named",
                        Label::new(item.span(), "no parameter name"),
                        "write `(param tick-rate (default 10 MHz))`",
                    ));
                    continue;
                };
                let default = parts
                    .iter()
                    .skip(2)
                    .find(|part| part.head() == Some("default"))
                    .cloned();
                params.push(ParamDecl {
                    name: param_name.to_string(),
                    default,
                    span: item.span(),
                });
            }
            Some("import") => {
                if let Some(import) = read_import(item, diagnostics) {
                    imports.push(import);
                }
            }
            Some(_) => declarations.push(item.clone()),
            None => diagnostics.push(Diagnostic::error(
                "module-bare-form",
                format!("a module holds declarations, not a bare {}", item.kind()),
                Label::new(item.span(), "expected `(<kind> …)`"),
                "every item in a module is a declaration or a module clause",
            )),
        }
    }

    let Some(version) = version else {
        diagnostics.push(Diagnostic::error(
            "module-missing-version",
            format!("module `{name}` declares no version"),
            Label::new(form.span(), "no `version` clause"),
            "write `(version 1 0)` — an unversioned module cannot be required by an importer, \
             and §15 needs a locked description to keep its meaning",
        ));
        return None;
    };

    // ⛔ A duplicate export is a clash, not a harmless repetition: an importer resolving the
    // name has two answers and no rule for choosing (F02).
    let mut seen: BTreeMap<&str, Span> = BTreeMap::new();
    for export in &exports {
        if let Some(first) = seen.get(export.name.as_str()) {
            diagnostics.push(
                Diagnostic::error(
                    "module-conflicting-export",
                    format!("`{}` is exported twice by module `{name}`", export.name),
                    Label::new(export.span, "exported again here"),
                    "export each name once; two exports of one name give an importer two \
                     answers and no rule for choosing between them",
                )
                .with_secondary(Label::new(*first, "first exported here")),
            );
        } else {
            seen.insert(export.name.as_str(), export.span);
        }
    }

    // An export naming nothing is a promise the module cannot keep.
    let declared: Vec<&str> = declarations
        .iter()
        .filter_map(|f| f.items().get(1).and_then(Form::as_symbol))
        .collect();
    for export in &exports {
        if !declared.contains(&export.name.as_str()) {
            diagnostics.push(Diagnostic::error(
                "module-dangling-export",
                format!(
                    "module `{name}` exports `{}`, which it does not declare",
                    export.name
                ),
                Label::new(export.span, "nothing declares this name"),
                if declared.is_empty() {
                    "this module declares nothing; remove the export or add the declaration"
                        .to_string()
                } else {
                    format!("this module declares {}", join(&declared))
                },
            ));
        }
    }

    Some(ModuleDecl {
        name: name.to_string(),
        version,
        exports,
        params,
        imports,
        declarations,
        span: form.span(),
    })
}

fn read_import(form: &Form, diagnostics: &mut Diagnostics) -> Option<ImportDecl> {
    let items = form.items();
    let Some(module) = items.get(1).and_then(Form::as_symbol) else {
        diagnostics.push(Diagnostic::error(
            "module-bad-import",
            "an import must name a module",
            Label::new(form.span(), "no module named"),
            "write `(import platform.timer (as timer))`",
        ));
        return None;
    };

    let mut alias = None;
    let mut required_version = None;
    let mut arguments = Vec::new();

    for item in items.iter().skip(2) {
        match item.head() {
            Some("as") => match item.items().get(1).and_then(Form::as_symbol) {
                Some(name) => alias = Some(name.to_string()),
                None => diagnostics.push(Diagnostic::error(
                    "module-bad-alias",
                    "`as` takes a namespace name",
                    Label::new(item.span(), "expected a name"),
                    "write `(as timer)`",
                )),
            },
            Some("version") => {
                // The requirement is NESTED: `(version (at-least 1 0))`, not
                // `(version at-least 1 0)`. Keeping the relation as its own form leaves room
                // for `(at-most …)` and `(exactly …)` without changing the surrounding shape.
                let constraint = item.items().get(1).cloned();
                let parts = constraint.as_ref().map(Form::items).unwrap_or_default();
                // items of `(at-least 1 0)` are [at-least, 1, 0]; head() already read index 0.
                match (
                    constraint.as_ref().and_then(Form::head),
                    parts.get(1),
                    parts.get(2),
                ) {
                    (
                        Some("at-least"),
                        Some(Form::Integer { value: major, .. }),
                        Some(Form::Integer { value: minor, .. }),
                    ) => {
                        required_version = Some(Version::new(
                            u32::try_from(*major).unwrap_or(0),
                            u32::try_from(*minor).unwrap_or(0),
                        ));
                    }
                    _ => diagnostics.push(Diagnostic::error(
                        "module-bad-version-requirement",
                        "a version requirement is written `(version (at-least <major> <minor>))`",
                        Label::new(item.span(), "not a version requirement"),
                        "write `(version (at-least 1 0))`",
                    )),
                }
            }
            Some("with") => {
                for binding in item.items().iter().skip(1) {
                    match (binding.head(), binding.items().len()) {
                        (Some(name), 2) => {
                            arguments.push((name.to_string(), binding.items()[1].clone()));
                        }
                        (Some(name), _) => {
                            // A multi-form value is kept whole, so `(tick-rate 10 MHz)` binds
                            // the quantity rather than only its magnitude.
                            arguments.push((name.to_string(), binding.clone()));
                        }
                        (None, _) => diagnostics.push(Diagnostic::error(
                            "module-bad-argument",
                            "a parameter binding is written `(<param> <value>)`",
                            Label::new(binding.span(), "not a binding"),
                            "write `(with (tick-rate 20 MHz))`",
                        )),
                    }
                }
            }
            _ => diagnostics.push(Diagnostic::error(
                "module-unknown-import-clause",
                format!(
                    "`{}` is not part of an import",
                    item.head().unwrap_or(item.kind())
                ),
                Label::new(item.span(), "unknown here"),
                "an import holds `as`, `version` and `with`",
            )),
        }
    }

    // The alias defaults to the module's last dotted segment, which is what an author means
    // nine times out of ten and is still explicit in the qualified names that result.
    let alias = alias.unwrap_or_else(|| module.rsplit('.').next().unwrap_or(module).to_string());

    Some(ImportDecl {
        module: module.to_string(),
        alias,
        required_version,
        arguments,
        span: form.span(),
    })
}

/// Elaborate a root module and everything it imports.
///
/// Returns the program built so far together with every diagnostic. A program is returned even
/// on failure: the instances that did elaborate are useful to an editor, and the caller decides
/// what an error means by asking [`Diagnostics::has_errors`].
#[must_use]
pub fn elaborate(
    sources: &mut SourceMap,
    modules: &dyn ModuleSource,
    root: &str,
) -> (Program, Diagnostics) {
    let mut elaborator = Elaborator {
        sources,
        modules,
        diagnostics: Diagnostics::new(),
        instances: Vec::new(),
        stack: Vec::new(),
    };
    elaborator.instantiate(root, String::new(), &[], None, None);
    let Elaborator {
        diagnostics,
        instances,
        ..
    } = elaborator;
    (Program { instances }, diagnostics)
}

struct Elaborator<'a> {
    sources: &'a mut SourceMap,
    modules: &'a dyn ModuleSource,
    diagnostics: Diagnostics,
    instances: Vec<Instance>,
    /// The chain of modules currently being elaborated, for cycle detection.
    stack: Vec<String>,
}

impl Elaborator<'_> {
    /// Elaborate one instance and return its id.
    fn instantiate(
        &mut self,
        module_name: &str,
        path: String,
        arguments: &[(String, Form)],
        required_version: Option<Version>,
        site: Option<Span>,
    ) -> Option<usize> {
        // ⛔ The one failure that must stop rather than collect: continuing into a cycle does
        // not terminate. The whole chain is reported, because "there is a cycle" without the
        // path is a puzzle rather than a diagnostic.
        if let Some(position) = self.stack.iter().position(|item| item == module_name) {
            let mut chain: Vec<&str> = self.stack[position..].iter().map(String::as_str).collect();
            chain.push(module_name);
            let span = site.unwrap_or_else(|| Span::new(crate::source::SourceId(0), 0, 0));
            self.diagnostics.push(Diagnostic::error(
                "module-circular-import",
                format!("circular import: {}", chain.join(" → ")),
                Label::new(span, "this import closes the cycle"),
                format!(
                    "break the cycle by moving the shared declarations into a module that both \
                     import, or by removing one edge of {}",
                    chain.join(" → ")
                ),
            ));
            return None;
        }

        let Some((display, text)) = self.modules.load(module_name) else {
            let span = site.unwrap_or_else(|| Span::new(crate::source::SourceId(0), 0, 0));
            self.diagnostics.push(Diagnostic::error(
                "module-not-found",
                format!("module `{module_name}` was not found"),
                Label::new(span, "imported here"),
                "check the module name, or add it to the module path",
            ));
            return None;
        };

        let Ok(source_id) = self.sources.add(display, text) else {
            let span = site.unwrap_or_else(|| Span::new(crate::source::SourceId(0), 0, 0));
            self.diagnostics.push(Diagnostic::error(
                "module-too-large",
                format!("module `{module_name}` is too large to address"),
                Label::new(span, "imported here"),
                "split the module",
            ));
            return None;
        };

        let (document, read_diagnostics) = read(self.sources, source_id);
        let had_read_errors = read_diagnostics.has_errors();
        for item in read_diagnostics.items() {
            self.diagnostics.push(item.clone());
        }
        if had_read_errors {
            return None;
        }

        let declaration = single_module(&document, &mut self.diagnostics)?;
        let module = read_module(&declaration, &mut self.diagnostics)?;

        if module.name != module_name {
            self.diagnostics.push(Diagnostic::error(
                "module-name-mismatch",
                format!(
                    "module `{module_name}` declares itself as `{}`",
                    module.name
                ),
                Label::new(module.span, "declared name"),
                "the declared name must match the name it is imported by, or a locked build \
                 cannot tell which module it resolved",
            ));
        }

        if let Some(required) = required_version {
            if !module.version.satisfies(required) {
                let span = site.unwrap_or(module.span);
                self.diagnostics.push(
                    Diagnostic::error(
                        "module-incompatible-version",
                        format!(
                            "module `{module_name}` is version {} but {required} or compatible is required",
                            module.version
                        ),
                        Label::new(span, "required here"),
                        if module.version.major == required.major {
                            format!(
                                "the import needs at least minor {}; this module offers {}",
                                required.minor, module.version.minor
                            )
                        } else {
                            format!(
                                "major versions differ ({} vs {}), so no minor version can satisfy \
                                 this import — §15 keeps a locked description's meaning, which a \
                                 major bump is defined not to preserve",
                                module.version.major, required.major
                            )
                        },
                    )
                    .with_secondary(Label::new(module.span, "declared here")),
                );
            }
        }

        let bindings = self.resolve_bindings(&module, arguments, site);

        self.stack.push(module_name.to_string());

        // ⭐ Children are elaborated before the parent is pushed, so `instances` is in
        // dependency order: an instance's imports always have smaller ids than it does.
        let mut import_ids = Vec::new();
        let mut aliases: BTreeMap<&str, Span> = BTreeMap::new();
        for import in &module.imports {
            if let Some(first) = aliases.get(import.alias.as_str()) {
                self.diagnostics.push(
                    Diagnostic::error(
                        "module-conflicting-alias",
                        format!(
                            "`{}` is already bound in module `{module_name}`",
                            import.alias
                        ),
                        Label::new(import.span, "bound again here"),
                        format!(
                            "give one of them a different namespace, e.g. `(as {}_2)` — two \
                             imports under one alias make every qualified name ambiguous",
                            import.alias
                        ),
                    )
                    .with_secondary(Label::new(*first, "first bound here")),
                );
                continue;
            }
            aliases.insert(import.alias.as_str(), import.span);

            let child_path = if path.is_empty() {
                import.alias.clone()
            } else {
                format!("{path}.{}", import.alias)
            };
            if let Some(id) = self.instantiate(
                &import.module,
                child_path,
                &import.arguments,
                import.required_version,
                Some(import.span),
            ) {
                import_ids.push(id);
            }
        }

        self.stack.pop();

        let id = self.instances.len();
        self.instances.push(Instance {
            id,
            module: module.name.clone(),
            version: module.version,
            path,
            bindings,
            imports: import_ids,
            declarations: module.declarations,
        });
        Some(id)
    }

    /// Bind parameters: supplied arguments, then declared defaults, refusing unknown names and
    /// missing values.
    fn resolve_bindings(
        &mut self,
        module: &ModuleDecl,
        arguments: &[(String, Form)],
        site: Option<Span>,
    ) -> Vec<(String, Form)> {
        let mut bindings: BTreeMap<String, Form> = BTreeMap::new();

        for (name, value) in arguments {
            if module.params.iter().all(|param| &param.name != name) {
                let known: Vec<&str> = module.params.iter().map(|p| p.name.as_str()).collect();
                self.diagnostics.push(Diagnostic::error(
                    "module-unknown-parameter",
                    format!("module `{}` has no parameter `{name}`", module.name),
                    Label::new(value.span(), "unknown parameter"),
                    if known.is_empty() {
                        format!("module `{}` takes no parameters", module.name)
                    } else {
                        format!("its parameters are {}", join(&known))
                    },
                ));
                continue;
            }
            bindings.insert(name.clone(), value.clone());
        }

        for param in &module.params {
            if bindings.contains_key(&param.name) {
                continue;
            }
            match &param.default {
                Some(default) => {
                    bindings.insert(param.name.clone(), default.clone());
                }
                None => {
                    let span = site.unwrap_or(param.span);
                    self.diagnostics.push(
                        Diagnostic::error(
                            "module-missing-argument",
                            format!(
                                "module `{}` requires parameter `{}`",
                                module.name, param.name
                            ),
                            Label::new(span, "not supplied by this import"),
                            format!("add `(with ({} <value>))` to the import", param.name),
                        )
                        .with_secondary(Label::new(param.span, "declared here, with no default")),
                    );
                }
            }
        }

        bindings.into_iter().collect()
    }
}

fn single_module(document: &Document, diagnostics: &mut Diagnostics) -> Option<Form> {
    // §8: a language-version identifier may precede the declaration and is not a stray form — §6's
    // "exactly one top-level form" is about the *declaration*. Without this filter, stating the
    // language version in a module file would be refused as `module-multiple-forms`, which would leave
    // the one file kind that most needs a locked version unable to carry one.
    let declarations: Vec<&Form> = document
        .forms
        .iter()
        .filter(|form| !crate::language_version::is_identifier(form))
        .collect();
    match declarations.len() {
        1 => Some(declarations[0].clone()),
        0 => {
            diagnostics.push(Diagnostic::error(
                "module-empty",
                "this module file holds no declaration",
                Label::new(Span::new(crate::source::SourceId(0), 0, 0), "empty"),
                "a module file holds exactly one `(defmodule …)` form",
            ));
            None
        }
        n => {
            // ⛔ The label points at the second *declaration*, not at `document.forms[1]`: with an
            // identifier form present those are different forms, and labelling `forms[1]` would point
            // at the `(defmodule …)` and tell the author to delete the declaration instead of the
            // stray form.
            let offender = declarations
                .get(1)
                .map_or_else(|| document.forms[1].span(), |form| form.span());
            diagnostics.push(Diagnostic::error(
                "module-multiple-forms",
                format!("this module file holds {n} top-level declarations"),
                Label::new(offender, "only one module per file"),
                "a module file holds exactly one `(defmodule …)` form, optionally preceded by \
                 `(eadl-version eadl/1)`; move the rest into their own modules",
            ));
            None
        }
    }
}

fn join(items: &[&str]) -> String {
    items
        .iter()
        .map(|item| format!("`{item}`"))
        .collect::<Vec<_>>()
        .join(", ")
}
