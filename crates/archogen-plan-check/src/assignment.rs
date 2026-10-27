//! The assignment record, `archogen-assignment/2` (leaf `M3.5.4.1`; `/2` by `M3.2.5`, from
//! `docs/specs/adapters/decision_engine-adapters.md` §5 alone): what the search chose for one system of a description,
//! which the configuration checker judges against that description. One format with two writers in two crates that
//! share none — the search's (`M3.4`) and this one — each with its own constant, both held to one golden in the version
//! register (`assignment-format`).
//!
//! ⭐ **An eADL document with a canonical form of its own**: one clause per line, in this order —
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
//! — the profile it was resolved under, the system, the digest of the elaborated program ([`program_digest`]), the
//! engine's adapter catalog it was resolved with ([`crate::adapters::VERSION`]), the placement memory chosen — a region
//! by the provider that offers it and the name it offers it under, as `(region NAME …)` names it within that provider
//! (`docs/semantics/model.md` §8 rules 5 and 10) — and one binding per requester and fact.
//! Names are **qualified** as module resolution qualifies them (`eadl_front::Instance::qualify`); a task is written
//! `(task SYSTEM TASK)`, never `system/task`, which a legal `(defservice s/t …)` collides with. **What binds** is
//! `M3.5.4.4`'s to check, and the level is never in the record: the checker reads it from the profile.
//!
//! ⭐ **An engine adapter's instance is a form too** (record §5): `(adapter KIND OVER)` for a shared kind and
//! `(adapter KIND OVER REQUESTER)` for an exclusive one, its `REQUESTER` a name or a task form — the one requester it
//! serves — wherever a requester or a provider stands. It is a provider for what its kind offers and a requester for
//! what its kind requires beneath, each fact bound to `OVER`, the block or platform it programs. A kind is the
//! checker's own table's ([`crate::adapters`]): a kind it does not hold, or an instance whose shape is not its kind's,
//! is refused here, never read as a name.
//!
//! ⭐ **The order is the canonical text's** (record §5, its canonical text fixed `2026-10-27`): bindings sort by
//! requester — every name, then every task, then every instance, each by its canonical text — and then by fact, by its
//! text. So an instance serving a task, `(adapter K O (task …))`, sorts before one serving a name, `(adapter K O n)`,
//! since `(` precedes every character a name begins with.

use std::cmp::Ordering;
use std::collections::BTreeMap;
use std::fmt;

use archogen_digest::Digest;
use eadl_front::{Form, Program, SourceMap};

use crate::adapters;

/// The format's identifier: the checker's own constant, held with the search's to one golden.
pub const FORMAT: &str = "archogen-assignment/2";

/// Who requires: a declaration by its qualified name, a task of a system, `(task SYSTEM TASK)`, or an engine adapter's
/// instance, for what its kind requires beneath.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Requester {
    /// A declaration — a service, a block — by its qualified name.
    Named(String),
    /// A task of a system.
    Task {
        /// The system's qualified name.
        system: String,
        /// The task's name within it.
        task: String,
    },
    /// An engine adapter's instance, a requester for what its kind requires beneath (record §4 rule 1).
    Instance(Instance),
}

impl Requester {
    /// The requester as the record writes it.
    #[must_use]
    pub fn canonical(&self) -> String {
        match self {
            Self::Named(name) => name.clone(),
            Self::Task { system, task } => format!("(task {system} {task})"),
            Self::Instance(instance) => instance.canonical(),
        }
    }

    /// Its place among the record's kinds of requester: every name, then every task, then every instance.
    const fn rank(&self) -> u8 {
        match self {
            Self::Named(_) => 0,
            Self::Task { .. } => 1,
            Self::Instance(_) => 2,
        }
    }

    /// One requester read from its canonical text, as a `bind` clause writes it.
    ///
    /// # Errors
    ///
    /// The [`Refusal`] the record's reader makes of it.
    pub fn parse(text: &str) -> Result<Self, Refusal> {
        read_requester(&one_form(text)?)
    }
}

impl Ord for Requester {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank()
            .cmp(&other.rank())
            .then_with(|| self.canonical().cmp(&other.canonical()))
    }
}

impl PartialOrd for Requester {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// An engine adapter's instance (record §5): its kind, the block or platform it programs, and — for an exclusive kind
/// — the one requester it serves.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Instance {
    /// The kind's name, one of the checker's table ([`crate::adapters::kind`]).
    pub kind: String,
    /// The block or platform it programs, by its qualified name: its device.
    pub over: String,
    /// The one requester it serves, a name or a task, for an exclusive kind; none for a shared one.
    pub serves: Option<Box<Requester>>,
}

impl Instance {
    /// The instance as the record writes it: `(adapter KIND OVER)` or `(adapter KIND OVER REQUESTER)`.
    #[must_use]
    pub fn canonical(&self) -> String {
        match &self.serves {
            None => format!("(adapter {} {})", self.kind, self.over),
            Some(requester) => format!(
                "(adapter {} {} {})",
                self.kind,
                self.over,
                requester.canonical()
            ),
        }
    }
}

impl Ord for Instance {
    fn cmp(&self, other: &Self) -> Ordering {
        self.canonical().cmp(&other.canonical())
    }
}

impl PartialOrd for Instance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// What a binding binds to: a block or platform by its qualified name, or an engine adapter's instance.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Provider {
    /// A block or platform — or, in a corrupted record, any other name — by its qualified name.
    Named(String),
    /// An engine adapter's instance, a provider for what its kind offers.
    Instance(Instance),
}

impl Provider {
    /// The provider as the record writes it.
    #[must_use]
    pub fn canonical(&self) -> String {
        match self {
            Self::Named(name) => name.clone(),
            Self::Instance(instance) => instance.canonical(),
        }
    }

    /// One provider read from its canonical text, as a `bind` clause writes it.
    ///
    /// # Errors
    ///
    /// The [`Refusal`] the record's reader makes of it.
    pub fn parse(text: &str) -> Result<Self, Refusal> {
        let form = one_form(text)?;
        read_provider(&form).map_err(|refusal| match refusal {
            None => Refusal::ProviderKind(form.to_canonical()),
            Some(refusal) => refusal,
        })
    }

    const fn rank(&self) -> u8 {
        match self {
            Self::Named(_) => 0,
            Self::Instance(_) => 1,
        }
    }
}

impl Ord for Provider {
    fn cmp(&self, other: &Self) -> Ordering {
        self.rank()
            .cmp(&other.rank())
            .then_with(|| self.canonical().cmp(&other.canonical()))
    }
}

impl PartialOrd for Provider {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl From<&str> for Provider {
    fn from(name: &str) -> Self {
        Self::Named(name.to_owned())
    }
}

impl From<String> for Provider {
    fn from(name: String) -> Self {
        Self::Named(name)
    }
}

impl fmt::Display for Provider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(&self.canonical())
    }
}

/// One system's assignment, as the record states it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Assignment {
    /// The profile it was resolved under.
    pub profile: String,
    /// The system's qualified name.
    pub system: String,
    /// The elaborated program's digest, [`program_digest`]: 64 lowercase hexadecimal digits.
    pub program: String,
    /// The placement memory chosen, where the system has placement memory: the provider offering the region, by its
    /// qualified name, and the region's name within it.
    pub placement: Option<(String, String)>,
    /// Each binding, (requester, fact) to provider: one per requester and fact, by construction. The adapter catalog
    /// the record names is no field: a record of `/2` this checker reads names its table's, [`adapters::VERSION`].
    pub bindings: BTreeMap<(Requester, String), Provider>,
}

impl Assignment {
    /// The record's canonical text: one clause per line, in the format's order, the bindings sorted.
    #[must_use]
    pub fn to_canonical(&self) -> String {
        let mut out = format!(
            "(format \"{FORMAT}\")\n(profile {})\n(system {})\n(program \"{}\")\n(adapters {})\n",
            self.profile,
            self.system,
            self.program,
            adapters::VERSION
        );
        if let Some((provider, region)) = &self.placement {
            out.push_str(&format!("(placement {provider} {region})\n"));
        }
        for ((requester, fact), provider) in &self.bindings {
            out.push_str(&format!(
                "(bind {} {fact} {})\n",
                requester.canonical(),
                provider.canonical()
            ));
        }
        out
    }

    /// Read a record. Whitespace and comments are the reader's; the clauses are the format's, in its order, and the
    /// bindings sorted, so a record reads only as the one text it has.
    ///
    /// # Errors
    ///
    /// A [`Refusal`] naming what does not read.
    pub fn read(text: &str) -> Result<Self, Refusal> {
        let mut sources = SourceMap::new();
        let id = sources
            .add("assignment", text)
            .map_err(|e| Refusal::Unreadable(e.to_string()))?;
        let (document, diagnostics) = eadl_front::read(&sources, id);
        if diagnostics.has_errors() {
            return Err(Refusal::Unreadable(diagnostics.render(&sources)));
        }
        let mut clauses = document.declarations().peekable();
        let format = clause(clauses.next(), "format")?;
        match format {
            [Form::Str { value, .. }] if value == FORMAT => {}
            [Form::Str { value, .. }] => return Err(Refusal::Format(Some(value.clone()))),
            _ => return Err(Refusal::Format(None)),
        }
        let profile = symbol("profile", clause(clauses.next(), "profile")?)?;
        let system = symbol("system", clause(clauses.next(), "system")?)?;
        let program = match clause(clauses.next(), "program")? {
            [Form::Str { value, .. }]
                if value.len() == 64
                    && value
                        .bytes()
                        .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b)) =>
            {
                value.clone()
            }
            other => {
                return Err(Refusal::Value {
                    clause: "program",
                    found: canonical(other),
                })
            }
        };
        let catalog = symbol("adapters", clause(clauses.next(), "adapters")?)?;
        if catalog != adapters::VERSION {
            return Err(Refusal::Catalog(catalog));
        }
        let placement = if clauses.peek().and_then(|f| f.head()) == Some("placement") {
            match clause(clauses.next(), "placement")? {
                [Form::Symbol { name: provider, .. }, Form::Symbol { name: region, .. }] => {
                    Some((provider.clone(), region.clone()))
                }
                other => {
                    return Err(Refusal::Value {
                        clause: "placement",
                        found: canonical(other),
                    })
                }
            }
        } else {
            None
        };
        let mut bindings = BTreeMap::new();
        let mut last: Option<(Requester, String)> = None;
        for form in clauses {
            let [requester, fact, provider] = clause(Some(form), "bind")? else {
                return Err(Refusal::Value {
                    clause: "bind",
                    found: form.to_canonical(),
                });
            };
            let requester = read_requester(requester)?;
            let Form::Symbol { name: fact, .. } = fact else {
                return Err(Refusal::Value {
                    clause: "bind",
                    found: form.to_canonical(),
                });
            };
            let provider = read_provider(provider).map_err(|refusal| {
                refusal.unwrap_or_else(|| Refusal::Value {
                    clause: "bind",
                    found: form.to_canonical(),
                })
            })?;
            let key = (requester, fact.clone());
            if last.as_ref().is_some_and(|previous| *previous >= key) {
                return Err(Refusal::NotCanonical(form.to_canonical()));
            }
            last = Some(key.clone());
            bindings.insert(key, provider);
        }
        Ok(Self {
            profile,
            system,
            program,
            placement,
            bindings,
        })
    }
}

/// Why a record does not read as `archogen-assignment/2`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// It is no eADL document: the reader's diagnostics.
    Unreadable(String),
    /// It names another format — `archogen-assignment/1` among them — or none.
    Format(Option<String>),
    /// A clause the format's order wants is missing, or another stands in its place.
    Clause {
        /// The clause expected.
        expected: &'static str,
        /// What stands there, in canonical form, or nothing.
        found: Option<String>,
    },
    /// A clause holds a value of the wrong kind.
    Value {
        /// The clause.
        clause: &'static str,
        /// What it holds, in canonical form.
        found: String,
    },
    /// It was resolved with an adapter catalog other than the checker's table's.
    Catalog(String),
    /// A requester neither a qualified name, `(task SYSTEM TASK)` nor an adapter's instance.
    RequesterKind(String),
    /// A provider of a kind `/2` does not name: a catalog record, anything but a qualified name or an adapter's
    /// instance.
    ProviderKind(String),
    /// An adapter's instance naming a kind the checker's table does not hold.
    AdapterKind {
        /// The kind it names.
        kind: String,
        /// The instance, in canonical form.
        found: String,
    },
    /// An adapter's instance whose shape is not its kind's: an exclusive kind written without the requester it serves,
    /// a shared one with one, an instance over another or serving one, or a form of another length.
    InstanceShape {
        /// The instance, in canonical form.
        found: String,
        /// What its shape should be.
        why: &'static str,
    },
    /// A binding out of the sorted order, or a second for one requester and fact.
    NotCanonical(String),
}

impl fmt::Display for Refusal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Unreadable(why) => write!(f, "the record is no eADL document: {why}"),
            Self::Format(None) => write!(f, "the record names no format; `{FORMAT}` reads one that does"),
            Self::Format(Some(other)) => write!(
                f,
                "the record is `{other}`, which this checker does not read; it reads `{FORMAT}`"
            ),
            Self::Clause {
                expected,
                found: None,
            } => write!(f, "the record ends where its `{expected}` clause should be"),
            Self::Clause {
                expected,
                found: Some(found),
            } => write!(f, "the record's `{expected}` clause should stand where `{found}` does"),
            Self::Value { clause, found } => {
                write!(f, "the record's `{clause}` clause holds `{found}`, a value it cannot hold")
            }
            Self::Catalog(other) => write!(
                f,
                "the record was resolved with the adapter catalog `{other}`, and this checker's table is `{}`",
                adapters::VERSION
            ),
            Self::RequesterKind(found) => write!(
                f,
                "`{found}` is no requester `{FORMAT}` names: a qualified name, `(task SYSTEM TASK)`, or an adapter's \
                 instance `(adapter KIND OVER …)`"
            ),
            Self::ProviderKind(found) => write!(
                f,
                "`{found}` is a provider kind `{FORMAT}` does not name: its providers are blocks and platforms, by \
                 their qualified names, and adapters' instances, `(adapter KIND OVER …)`"
            ),
            Self::AdapterKind { kind, found } => write!(
                f,
                "`{found}` names the adapter kind `{kind}`, which this checker's table, `{}`, does not hold",
                adapters::VERSION
            ),
            Self::InstanceShape { found, why } => {
                write!(f, "`{found}` is no instance `{FORMAT}` writes: {why}")
            }
            Self::NotCanonical(found) => write!(
                f,
                "`{found}` is out of the record's canonical order, or a second binding for one requester and fact"
            ),
        }
    }
}

/// The one form `text` holds.
fn one_form(text: &str) -> Result<Form, Refusal> {
    let mut sources = SourceMap::new();
    let id = sources
        .add("assignment", text)
        .map_err(|e| Refusal::Unreadable(e.to_string()))?;
    let (document, diagnostics) = eadl_front::read(&sources, id);
    if diagnostics.has_errors() {
        return Err(Refusal::Unreadable(diagnostics.render(&sources)));
    }
    match document.forms.as_slice() {
        [form] => Ok(form.clone()),
        _ => Err(Refusal::Unreadable(format!("`{text}` is not one form"))),
    }
}

/// A requester as a `bind` clause writes it: a name, `(task SYSTEM TASK)`, or an instance.
pub(crate) fn read_requester(form: &Form) -> Result<Requester, Refusal> {
    match form {
        Form::Symbol { name, .. } => Ok(Requester::Named(name.clone())),
        Form::List { items, .. } => match items.as_slice() {
            [Form::Symbol { name: head, .. }, Form::Symbol { name: system, .. }, Form::Symbol { name: task, .. }]
                if head == "task" =>
            {
                Ok(Requester::Task {
                    system: system.clone(),
                    task: task.clone(),
                })
            }
            _ if form.head() == Some("adapter") => read_instance(form).map(Requester::Instance),
            _ => Err(Refusal::RequesterKind(form.to_canonical())),
        },
        _ => Err(Refusal::RequesterKind(form.to_canonical())),
    }
}

/// A provider as a `bind` clause writes it: a name, or an instance; `None` for what is neither a name nor a list.
fn read_provider(form: &Form) -> Result<Provider, Option<Refusal>> {
    match form {
        Form::Symbol { name, .. } => Ok(Provider::Named(name.clone())),
        Form::List { .. } if form.head() == Some("adapter") => {
            read_instance(form).map(Provider::Instance).map_err(Some)
        }
        Form::List { .. } => Err(Some(Refusal::ProviderKind(form.to_canonical()))),
        _ => Err(None),
    }
}

/// An instance, `(adapter KIND OVER)` or `(adapter KIND OVER REQUESTER)`, its kind the table's and its shape its
/// kind's (record §5).
fn read_instance(form: &Form) -> Result<Instance, Refusal> {
    let found = form.to_canonical();
    let shape = |why| Refusal::InstanceShape {
        found: found.clone(),
        why,
    };
    let items = form.items();
    let Some(Form::Symbol { name: kind, .. }) = items.get(1) else {
        return Err(shape(
            "an instance is `(adapter KIND OVER)` or `(adapter KIND OVER REQUESTER)`, its kind a name",
        ));
    };
    let Some(table) = adapters::kind(kind) else {
        return Err(Refusal::AdapterKind {
            kind: kind.clone(),
            found,
        });
    };
    let over =
        match items.get(2) {
            Some(Form::Symbol { name, .. }) => name.clone(),
            _ => return Err(shape(
                "an instance programs a block or a platform, by its name — never another instance",
            )),
        };
    let serves =
        match &items[3.min(items.len())..] {
            [] => None,
            [requester] => match read_requester(requester)? {
                Requester::Instance(_) => return Err(shape(
                    "the requester an instance serves is a name or a task, never another instance",
                )),
                requester => Some(Box::new(requester)),
            },
            _ => {
                return Err(shape(
                    "an instance is `(adapter KIND OVER)` or `(adapter KIND OVER REQUESTER)`",
                ))
            }
        };
    match (table.exclusive(), &serves) {
        (true, None) => Err(shape(
            "its kind is exclusive, so an instance names the one requester it serves after the provider it programs",
        )),
        (false, Some(_)) => Err(shape(
            "its kind is shared, so an instance serves every requester bound to it and names none",
        )),
        _ => Ok(Instance {
            kind: kind.clone(),
            over,
            serves,
        }),
    }
}

/// The items after `head` of `form`, if `form` is that clause.
fn clause<'f>(form: Option<&'f Form>, head: &'static str) -> Result<&'f [Form], Refusal> {
    match form {
        Some(form) if form.head() == Some(head) => Ok(&form.items()[1..]),
        other => Err(Refusal::Clause {
            expected: head,
            found: other.map(Form::to_canonical),
        }),
    }
}

/// The one symbol a clause holds.
fn symbol(head: &'static str, items: &[Form]) -> Result<String, Refusal> {
    match items {
        [Form::Symbol { name, .. }] => Ok(name.clone()),
        other => Err(Refusal::Value {
            clause: head,
            found: canonical(other),
        }),
    }
}

fn canonical(items: &[Form]) -> String {
    items
        .iter()
        .map(Form::to_canonical)
        .collect::<Vec<_>>()
        .join(" ")
}

/// The elaborated program's canonical text, which [`program_digest`] hashes: for each instance, ordered by its alias
/// path — the root's, written `.`, first — a line `instance PATH MODULE MAJOR.MINOR`, a line `with NAME VALUE` for
/// each parameter binding by name, its value in canonical form, then each declaration it contributes in canonical
/// form, one per line, in source order. So a change to an imported module, to its `with` or to its version is a
/// change of the digest, and whitespace and comments are none. A plain description — no `defmodule` — is one root
/// instance of no module, version `0.0` ([`crate::check::plain`]): `instance . - 0.0`.
#[must_use]
pub fn program_text(program: &Program) -> String {
    let mut instances: Vec<_> = program.instances.iter().collect();
    instances.sort_by(|a, b| a.path.cmp(&b.path));
    let mut out = String::new();
    for instance in instances {
        let path = if instance.path.is_empty() {
            "."
        } else {
            instance.path.as_str()
        };
        out.push_str(&format!(
            "instance {path} {} {}\n",
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
