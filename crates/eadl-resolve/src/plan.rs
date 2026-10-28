//! The plan the search writes (leaf `M3.4.3`; `/2` since leaf `M3.2.5`): `archogen-assignment/2`, the assignment record
//! the configuration
//! checker judges against the description (`docs/book/src/versions.md`, `assignment-format`). One format with two
//! writers in two crates that share none — this one and the checker's, `archogen_plan_check::assignment` — each with
//! its own constant and its own derivation of the program's digest, both held to one golden and to each other by
//! `tests/plan.rs`. A divergence is then a stale plan on every run, which the checker refuses, rather than a fault the
//! two would share.
//!
//! ⭐ **Its canonical form**, one clause per line, in this order:
//!
//! ```text
//! (format "archogen-assignment/2")
//! (profile PROFILE)
//! (system SYSTEM)
//! (program "SHA256")
//! (adapters adapters/1)
//! (placement PROVIDER REGION)            ; only where the system has placement memory
//! (bind REQUESTER FACT PROVIDER)         ; one per requester and fact, sorted
//! ```
//!
//! — a task written `(task SYSTEM TASK)`, an engine adapter's instance `(adapter KIND OVER)` or `(adapter KIND OVER
//! REQUESTER)` (`docs/specs/adapters/decision_engine-adapters.md` §5), each instance bound also a requester of what its
//! kind needs beneath; the bindings sorted by requester, every name, then every task, then every instance, each by its
//! text, then by fact.
//!
//! ⭐ **Read back** (leaf `M4.1.3`): [`read`] is the generator's own reader of the format, accepting only the text
//! [`write`] writes, and [`rebuild`] gives the joint rules the record's bindings over the description as the search held
//! them, so the runtime plan is lowered from the record the checker accepted (`docs/decisions/decision_runtime-plan.md`
//! §1).

use std::collections::BTreeMap;

use archogen_digest::Digest;
use eadl_front::module::{Instance, Program, Version};
use eadl_front::{Form, SourceMap};

use crate::adapter::{self, Catalog, Instance as AdapterInstance};
use crate::joint::{self, Binding, Joint, Map, Requester, System};
use crate::offer::{self, Provider};
use crate::requirement::Requirement;
use crate::search::{Resolution, Who};
use crate::vocabulary::Vocabulary;

/// The format's identifier: the search's own constant, held with the checker's to one golden.
pub const FORMAT: &str = "archogen-assignment/2";

/// The module a plain description's one root instance names: no module name is `-` (`docs/semantics/reference.md` §6
/// rule 8), so no module's instance writes the same line of the digest.
pub const PLAIN: &str = "-";

/// A plain description — a file with no `defmodule` among its forms — as one root instance: no module, version `0.0`,
/// `declarations` in source order, as the language's check read them. So its digest is defined, [`program_text`]
/// writing `instance . - 0.0`.
#[must_use]
pub fn plain(declarations: Vec<Form>) -> Program {
    // Built outside `vec!`: the trust inventory's catalog refuses the word `path` inside a macro's arguments.
    let root = Instance {
        id: 0,
        module: PLAIN.to_owned(),
        version: Version { major: 0, minor: 0 },
        path: String::new(),
        alias: String::new(),
        exports: Vec::new(),
        bindings: Vec::new(),
        imports: Vec::new(),
        declarations,
    };
    Program {
        instances: vec![root],
    }
}

/// The elaborated program's text, which its digest is taken of: for each instance in order of its path — the root's,
/// written `.`, first — a line `instance PATH MODULE MAJOR.MINOR`, a line `with NAME VALUE` for each parameter binding
/// by name, its value in canonical form, then each declaration it contributes in canonical form, one per line, in
/// source order. Whitespace and comments change nothing; an imported module, its `with` or its version changes it.
#[must_use]
pub fn program_text(program: &Program) -> String {
    let mut instances: Vec<&Instance> = program.instances.iter().collect();
    instances.sort_by(|a, b| a.path.cmp(&b.path));
    let mut out = String::new();
    for instance in instances {
        let at = if instance.path.is_empty() {
            "."
        } else {
            instance.path.as_str()
        };
        out.push_str(&format!(
            "instance {at} {} {}\n",
            instance.module, instance.version
        ));
        for (name, value) in &instance.bindings {
            out.push_str(&format!("with {name} {}\n", value.to_canonical()));
        }
        for declaration in &instance.declarations {
            out.push_str(&declaration.to_canonical());
            out.push('\n');
        }
    }
    out
}

/// The elaborated program's digest: the sha256 of [`program_text`], 64 lowercase hexadecimal digits.
#[must_use]
pub fn program_digest(program: &Program) -> String {
    Digest::of(program_text(program).as_bytes()).hex()
}

/// `resolution` as the record's canonical text, resolved under `profile` for the program whose digest is `program`
/// (`docs/specs/adapters/decision_engine-adapters.md` §5): the catalog the plan was resolved with named after the
/// program; every binding of a requester, and each instance bound as a requester of its own, binding each fact its
/// kind names beneath to the provider it programs; sorted by requester — names, then tasks, then instances, each by
/// its canonical text — then by fact.
#[must_use]
pub fn write(resolution: &Resolution, profile: &str, program: &str) -> String {
    let mut out = format!(
        "(format \"{FORMAT}\")\n(profile {profile})\n(system {})\n(program \"{program}\")\n(adapters {})\n",
        resolution.system,
        adapter::VERSION
    );
    if let Some((provider, region)) = &resolution.placement {
        out.push_str(&format!("(placement {provider} {region})\n"));
    }
    let instances: BTreeMap<&str, &AdapterInstance> = resolution
        .instances
        .iter()
        .map(|instance| (instance.provider.name.as_str(), instance))
        .collect();
    let written = |provider: &str| {
        instances.get(provider).map_or_else(
            || provider.to_string(),
            |instance| instance_text(instance, &resolution.system),
        )
    };
    // (class, requester, fact, provider): a name 0, a task 1, an instance 2.
    let mut lines: Vec<(u8, String, &str, String)> = resolution
        .bindings
        .iter()
        .map(|((who, fact), provider)| {
            let class = match who {
                Who::Named(_) => 0,
                Who::Task { .. } => 1,
            };
            (class, who.to_string(), fact.as_str(), written(provider))
        })
        .collect();
    for instance in &resolution.instances {
        let requester = instance_text(instance, &resolution.system);
        for fact in &instance.beneath {
            lines.push((2, requester.clone(), fact.as_str(), instance.over.clone()));
        }
    }
    lines.sort();
    for (_, requester, fact, provider) in lines {
        out.push_str(&format!("(bind {requester} {fact} {provider})\n"));
    }
    out
}

/// An instance as the record writes it: `(adapter KIND OVER)`, or `(adapter KIND OVER REQUESTER)` for an exclusive
/// kind's, its requester a name or a task of `system`, `(task SYSTEM TASK)` — the name every face gives it (record §9).
#[must_use]
pub fn instance_text(instance: &AdapterInstance, system: &str) -> String {
    match &instance.serves {
        None => format!("(adapter {} {})", instance.kind, instance.over),
        Some(requester) => {
            let who = match requester
                .strip_prefix(system)
                .and_then(|rest| rest.strip_prefix('/'))
            {
                Some(task) => format!("(task {system} {task})"),
                None => requester.clone(),
            };
            format!("(adapter {} {} {who})", instance.kind, instance.over)
        }
    }
}

// ⭐ The record read back (leaf `M4.1.3`, `docs/decisions/decision_runtime-plan.md` §1): the lowering lowers the record
// the configuration checker accepted, never the search's memory, so the generator reads `/2` with a reader of its own —
// sharing nothing with the checker's, `archogen_plan_check::assignment`, as the two writers share nothing — and rebuilds
// the bindings the joint rules judge from it, as the search held them.

/// One `archogen-assignment/2` record, read back by the generator's own reader.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Record {
    /// The profile the plan was resolved under.
    pub profile: String,
    /// The system.
    pub system: String,
    /// The elaborated program's digest, 64 lowercase hexadecimal digits.
    pub program: String,
    /// The adapter catalog's version.
    pub adapters: String,
    /// The placement memory chosen, where the system has some: its provider and its region.
    pub placement: Option<(String, String)>,
    /// Each binding, in the record's order.
    pub binds: Vec<Bind>,
}

/// One `(bind REQUESTER FACT PROVIDER)` line, its requester and its provider as the record writes them: a name, a task
/// `(task SYSTEM TASK)`, an instance `(adapter KIND OVER)` or `(adapter KIND OVER REQUESTER)`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Bind {
    /// Who requires.
    pub requester: String,
    /// The fact.
    pub fact: String,
    /// What meets it.
    pub provider: String,
}

/// Why a record does not read, or does not rebuild over a description: one sentence naming what is wrong.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Unread(pub String);

impl std::fmt::Display for Unread {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

/// The record's order for a requester's or a provider's text: a name 0, a task 1, an instance 2 ([`write`]'s classes).
fn class(text: &str) -> u8 {
    if text.starts_with("(task ") {
        1
    } else if text.starts_with("(adapter ") {
        2
    } else {
        0
    }
}

impl Record {
    /// The record's canonical text, as [`write`] writes it.
    #[must_use]
    pub fn text(&self) -> String {
        let mut out = format!(
            "(format \"{FORMAT}\")\n(profile {})\n(system {})\n(program \"{}\")\n(adapters {})\n",
            self.profile, self.system, self.program, self.adapters
        );
        if let Some((provider, region)) = &self.placement {
            out.push_str(&format!("(placement {provider} {region})\n"));
        }
        for bind in &self.binds {
            out.push_str(&format!(
                "(bind {} {} {})\n",
                bind.requester, bind.fact, bind.provider
            ));
        }
        out
    }
}

/// `text` read as an `archogen-assignment/2` record. It accepts only the canonical text [`write`] writes — one clause a
/// line, in its order, the bindings sorted and none twice — so a record read back writes back the same bytes.
///
/// # Errors
///
/// A text that does not read as the format's clauses, names another format, or is not in its canonical form.
pub fn read(text: &str) -> Result<Record, Unread> {
    let mut sources = SourceMap::new();
    let id = sources
        .add(FORMAT, text)
        .map_err(|e| Unread(e.to_string()))?;
    let (document, diagnostics) = eadl_front::read(&sources, id);
    if diagnostics.has_errors() || !document.comments.is_empty() {
        return Err(Unread(
            "the record does not read as clauses with no comment".to_string(),
        ));
    }
    let mut forms = document.forms.iter();
    let mut clause = |head: &str, arity: usize| -> Result<&[Form], Unread> {
        match forms.next() {
            Some(form) if form.head() == Some(head) && form.items().len() == arity + 1 => {
                Ok(&form.items()[1..])
            }
            _ => Err(Unread(format!(
                "the record has no `({head} …)` clause where its format places one"
            ))),
        }
    };
    match clause("format", 1)? {
        [Form::Str { value, .. }] if value == FORMAT => {}
        [Form::Str { value, .. }] => {
            return Err(Unread(format!(
                "the record names the format `{value}`, and this reader reads `{FORMAT}`"
            )))
        }
        _ => return Err(Unread("the record's format is no string".to_string())),
    }
    let symbol = |items: &[Form], what: &str| -> Result<String, Unread> {
        items
            .first()
            .and_then(Form::as_symbol)
            .map(str::to_string)
            .ok_or_else(|| Unread(format!("the record's {what} is no name")))
    };
    let profile = symbol(clause("profile", 1)?, "profile")?;
    let system = symbol(clause("system", 1)?, "system")?;
    let program = match clause("program", 1)? {
        [Form::Str { value, .. }]
            if value.len() == 64
                && value
                    .bytes()
                    .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) =>
        {
            value.clone()
        }
        _ => {
            return Err(Unread(
                "the record's program is no 64 lowercase hexadecimal digits".to_string(),
            ))
        }
    };
    let adapters = symbol(clause("adapters", 1)?, "catalog version")?;
    let rest: Vec<&Form> = forms.collect();
    let mut placement = None;
    let mut binds = Vec::new();
    for (at, form) in rest.iter().enumerate() {
        match (form.head(), form.items()) {
            (Some("placement"), [_, provider, region]) if at == 0 => {
                placement = Some((
                    symbol(std::slice::from_ref(provider), "placement provider")?,
                    symbol(std::slice::from_ref(region), "placement region")?,
                ));
            }
            (Some("bind"), [_, requester, fact, provider]) => {
                let text = |form: &Form, what: &str| match form {
                    Form::Symbol { name, .. } => Ok(name.clone()),
                    Form::List { .. } if matches!(form.head(), Some("task" | "adapter")) => {
                        Ok(form.to_canonical())
                    }
                    _ => Err(Unread(format!(
                        "a binding's {what} is no name, task or instance"
                    ))),
                };
                binds.push(Bind {
                    requester: text(requester, "requester")?,
                    fact: symbol(std::slice::from_ref(fact), "fact")?,
                    provider: text(provider, "provider")?,
                });
            }
            _ => {
                return Err(Unread(format!(
                    "the record holds `{}` where only `(placement …)`, first, and `(bind …)` clauses follow",
                    form.to_canonical()
                )))
            }
        }
    }
    let key = |b: &Bind| (class(&b.requester), b.requester.clone(), b.fact.clone());
    if binds.windows(2).any(|w| key(&w[0]) >= key(&w[1])) {
        return Err(Unread(
            "the record's bindings are not sorted by requester and fact, each pair once"
                .to_string(),
        ));
    }
    let record = Record {
        profile,
        system,
        program,
        adapters,
        placement,
        binds,
    };
    if record.text() != text {
        return Err(Unread(
            "the record is not in its canonical form: the reader accepts only what its writer writes".to_string(),
        ));
    }
    Ok(record)
}

/// A record's bindings over the description it was resolved from, rebuilt as the search held them.
#[derive(Debug, Clone)]
pub struct Rebuilt<'v> {
    /// The system, read at the profile's caller level, placed where the record places it.
    pub system: System,
    /// Every requester the description writes.
    pub requesters: Vec<Requester>,
    /// Each adapter instance the record names, rebuilt from the catalog, in the record's order.
    pub instances: Vec<AdapterInstance>,
    /// The joint rules over the description, with the record's instances, for the system.
    pub joint: Joint<'v>,
    /// The system's memory map as the joint rules read it.
    pub map: Map,
    /// Each binding, a requester's requirement met by a provider, by the providers' and requesters' own names.
    pub bindings: Vec<Binding>,
}

/// `text`, a task or an instance as the record writes it, read as the joint rules name it: a task `(task SYSTEM TASK)`
/// as `SYSTEM/TASK`, an instance by [`adapter::instance_name`], its requester read so too; a name as itself.
fn internal(text: &str, system: &str) -> Result<String, Unread> {
    if let Some(inner) = text
        .strip_prefix("(task ")
        .and_then(|t| t.strip_suffix(')'))
    {
        return match inner.split(' ').collect::<Vec<_>>()[..] {
            [of, task] if of == system => Ok(format!("{of}/{task}")),
            _ => Err(Unread(format!("`{text}` is no task of `{system}`"))),
        };
    }
    if let Some((kind, over, serves)) = instance_parts(text) {
        let serves = serves.map(|s| internal(s, system)).transpose()?;
        return Ok(adapter::instance_name(kind, over, serves.as_deref()));
    }
    Ok(text.to_string())
}

/// An instance's text `(adapter KIND OVER)` or `(adapter KIND OVER REQUESTER)` as its parts.
fn instance_parts(text: &str) -> Option<(&str, &str, Option<&str>)> {
    let inner = text.strip_prefix("(adapter ")?.strip_suffix(')')?;
    let (kind, rest) = inner.split_once(' ')?;
    match rest.split_once(' ') {
        Some((over, serves)) => Some((kind, over, Some(serves))),
        None => Some((kind, rest, None)),
    }
}

/// `record` over `declarations` — the description it was resolved from, as the language's check read it — rebuilt as
/// the search held it: the system read at `level`; every provider and requester read as the search reads them; each
/// instance the record names rebuilt from `catalog`, an exclusive kind's with every requirement its requester writes;
/// each binding matched to every requirement its requester writes on its fact but a statement, an instance's to its
/// kind's `beneath`. A record that binds what the description does not require, or leaves a requirement of the system's
/// unbound, does not rebuild.
///
/// # Errors
///
/// A system, requester, provider, kind or requirement the record names and the description or the catalog does not
/// hold; a requirement bound twice or left unbound; a declaration the joint rules do not read.
pub fn rebuild<'v>(
    record: &Record,
    declarations: &[Form],
    level: &str,
    vocabulary: &'v Vocabulary,
    catalog: &Catalog,
) -> Result<Rebuilt<'v>, Unread> {
    let name_of = |d: &Form| {
        d.items()
            .get(1)
            .and_then(Form::as_symbol)
            .unwrap_or_default()
            .to_string()
    };
    let decl = declarations
        .iter()
        .find(|d| d.head() == Some("defsystem") && name_of(d) == record.system)
        .ok_or_else(|| {
            Unread(format!(
                "the description declares no system `{}`",
                record.system
            ))
        })?;
    let mut system = System::read(decl, declarations)
        .at(level, vocabulary)
        .map_err(Unread)?;
    let hardware = |d: &Form| matches!(d.head(), Some("defblock" | "defplatform"));
    let mut providers: Vec<(Provider, &Form)> = Vec::new();
    for d in declarations.iter().filter(|d| hardware(d)) {
        let provider = offer::read(d, vocabulary)
            .map_err(|_| Unread(format!("`{}`'s offers do not read", name_of(d))))?;
        providers.push((provider, d));
    }
    let mut requesters: Vec<Requester> = Vec::new();
    for d in declarations
        .iter()
        .filter(|d| !hardware(d) && d.head().is_some_and(|h| h.starts_with("def")))
    {
        requesters.extend(
            joint::requesters(d, vocabulary)
                .map_err(|_| Unread(format!("`{}`'s requirements do not read", name_of(d))))?,
        );
    }
    let joint = Joint::new(&providers, &requesters, vocabulary)
        .map_err(|_| Unread("the joint rules do not read the description".to_string()))?;
    let map = joint.map(&system);
    if let Some((_, region)) = &record.placement {
        system = system.placed_in(region);
    }
    // Each instance the record names, as a requester or as a provider, rebuilt once.
    let mut instances: Vec<AdapterInstance> = Vec::new();
    for text in record
        .binds
        .iter()
        .flat_map(|b| [b.requester.as_str(), b.provider.as_str()])
    {
        let Some((kind_name, over, serves)) = instance_parts(text) else {
            continue;
        };
        let name = internal(text, &record.system)?;
        if instances.iter().any(|i| i.provider.name == name) {
            continue;
        }
        let kind = catalog.kind(kind_name).ok_or_else(|| {
            Unread(format!(
                "the catalog has no kind `{kind_name}`, which `{text}` names"
            ))
        })?;
        let over = providers
            .iter()
            .map(|(p, _)| p)
            .find(|p| p.name == over)
            .ok_or_else(|| {
                Unread(format!(
                    "the description has no provider `{over}`, which `{text}` programs"
                ))
            })?;
        let served = serves.map(|s| internal(s, &record.system)).transpose()?;
        if served.is_some() != kind.exclusive() {
            return Err(Unread(format!(
                "`{text}` names {} requester, and `{kind_name}` serves {}",
                if served.is_some() { "a" } else { "no" },
                if kind.exclusive() {
                    "one"
                } else {
                    "every requester bound to it"
                }
            )));
        }
        let requirements = match &served {
            Some(r) => Some(
                requesters
                    .iter()
                    .find(|q| &q.name == r)
                    .map(|q| q.requirements.as_slice())
                    .ok_or_else(|| {
                        Unread(format!(
                            "`{text}` serves `{r}`, which the description does not write"
                        ))
                    })?,
            ),
            None => None,
        };
        let instance = kind.instance(over, served.as_deref().zip(requirements), vocabulary);
        debug_assert_eq!(instance.provider.name, name);
        instances.push(instance);
    }
    let instance_of = |name: &str| instances.iter().find(|i| i.provider.name == name);
    // Each binding, matched to the requirements its requester writes on its fact.
    let mut bindings = Vec::new();
    let mut bound: BTreeMap<(String, String), String> = BTreeMap::new();
    for bind in &record.binds {
        let requester = internal(&bind.requester, &record.system)?;
        let provider = internal(&bind.provider, &record.system)?;
        if bound
            .insert((requester.clone(), bind.fact.clone()), provider.clone())
            .is_some()
        {
            return Err(Unread(format!(
                "the record binds `{}`'s `{}` twice",
                bind.requester, bind.fact
            )));
        }
        if instance_of(&provider).is_none() && !providers.iter().any(|(p, _)| p.name == provider) {
            return Err(Unread(format!(
                "the record binds to `{}`, which is no provider",
                bind.provider
            )));
        }
        let written: Vec<Requirement> = match instance_of(&requester) {
            Some(instance) => {
                if provider != instance.over {
                    return Err(Unread(format!(
                        "`{}` binds `{}` to `{}`, and programs `{}`",
                        bind.requester, bind.fact, bind.provider, instance.over
                    )));
                }
                catalog
                    .kind(&instance.kind)
                    .map(|k| {
                        k.beneath
                            .iter()
                            .filter(|q| q.fact() == bind.fact)
                            .cloned()
                            .collect()
                    })
                    .unwrap_or_default()
            }
            None => {
                if !system.serves(&requester) {
                    return Err(Unread(format!(
                        "the record binds `{}`, which `{}` does not serve",
                        bind.requester, record.system
                    )));
                }
                requesters
                    .iter()
                    .find(|q| q.name == requester)
                    .map(|q| {
                        q.requirements
                            .iter()
                            .filter(|r| {
                                r.fact() == bind.fact && !matches!(r, Requirement::Statement { .. })
                            })
                            .cloned()
                            .collect()
                    })
                    .unwrap_or_default()
            }
        };
        if written.is_empty() {
            return Err(Unread(format!(
                "the record binds `{}`'s `{}`, which it does not require",
                bind.requester, bind.fact
            )));
        }
        bindings.extend(written.into_iter().map(|requirement| Binding {
            requester: requester.clone(),
            requirement,
            provider: provider.clone(),
        }));
    }
    // Every requirement the system's requesters, and the instances' kinds, write is bound.
    let required = requesters
        .iter()
        .filter(|q| system.serves(&q.name))
        .flat_map(|q| {
            q.requirements
                .iter()
                .filter(|r| !matches!(r, Requirement::Statement { .. }))
                .map(move |r| (q.name.clone(), r.fact().to_string()))
        })
        .chain(instances.iter().flat_map(|i| {
            i.beneath
                .iter()
                .map(move |fact| (i.provider.name.clone(), fact.clone()))
        }));
    for (requester, fact) in required {
        if !bound.contains_key(&(requester.clone(), fact.clone())) {
            return Err(Unread(format!(
                "the record leaves `{requester}`'s `{fact}` unbound"
            )));
        }
    }
    let joint = joint.with_instances(&instances).for_system(system.clone());
    Ok(Rebuilt {
        system,
        requesters,
        instances,
        joint,
        map,
        bindings,
    })
}
