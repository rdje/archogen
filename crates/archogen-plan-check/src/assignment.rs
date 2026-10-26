//! The assignment record, `archogen-assignment/1` (leaf `M3.5.4.1`; `M3.5`'s design, *The assignment record*): what
//! the search chose for one system of a description, which the configuration checker judges against that
//! description. One format with two writers in two crates that share none — the search's (`M3.4`) and this one —
//! each with its own constant, both held to one golden in the version register (`assignment-format`).
//!
//! ⭐ **An eADL document with a canonical form of its own**: one clause per line, in this order —
//!
//! ```text
//! (format "archogen-assignment/1")
//! (profile PROFILE)
//! (system SYSTEM)
//! (program "SHA256")
//! (placement PROVIDER REGION)            ; only where the system has placement memory
//! (bind REQUESTER FACT PROVIDER)         ; one per requester and fact, sorted
//! ```
//!
//! — the profile it was resolved under, the system, the digest of the elaborated program ([`program_digest`]), the
//! placement memory chosen — a region by the provider that offers it and the name it offers it under, as `(region
//! NAME …)` names it within that provider (`docs/semantics/model.md` §8 rules 5 and 10) — and one binding per
//! requester and fact.
//! Names are **qualified** as module resolution qualifies them (`eadl_front::Instance::qualify`); a task is written
//! `(task SYSTEM TASK)`, never `system/task`, which a legal `(defservice s/t …)` collides with. Bindings are sorted
//! by requester — every named one before every task, each kind by its text — then by fact. **What binds** is
//! `M3.5.4.4`'s to check, and the level is never in the record: the checker reads it from the profile.
//!
//! ⛔ **`/1`'s providers are blocks and platforms alone.** A provider is a qualified name; a provider written as a
//! list — an adapter by its engine identity, a catalog record by its hash, the kinds `M3.2` and `M3.7` add — is a
//! kind `/1` does not name, refused here rather than read as a name.

use std::collections::BTreeMap;
use std::fmt;

use archogen_digest::Digest;
use eadl_front::{Form, Program, SourceMap};

/// The format's identifier: the checker's own constant, held with the search's to one golden.
pub const FORMAT: &str = "archogen-assignment/1";

/// Who requires: a declaration by its qualified name, or a task of a system, `(task SYSTEM TASK)`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
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
}

impl Requester {
    /// The requester as the record writes it.
    #[must_use]
    pub fn canonical(&self) -> String {
        match self {
            Self::Named(name) => name.clone(),
            Self::Task { system, task } => format!("(task {system} {task})"),
        }
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
    /// Each binding, (requester, fact) to provider: one per requester and fact, by construction.
    pub bindings: BTreeMap<(Requester, String), String>,
}

impl Assignment {
    /// The record's canonical text: one clause per line, in the format's order, the bindings sorted.
    #[must_use]
    pub fn to_canonical(&self) -> String {
        let mut out = format!(
            "(format \"{FORMAT}\")\n(profile {})\n(system {})\n(program \"{}\")\n",
            self.profile, self.system, self.program
        );
        if let Some((provider, region)) = &self.placement {
            out.push_str(&format!("(placement {provider} {region})\n"));
        }
        for ((requester, fact), provider) in &self.bindings {
            out.push_str(&format!(
                "(bind {} {fact} {provider})\n",
                requester.canonical()
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
            let requester = match requester {
                Form::Symbol { name, .. } => Requester::Named(name.clone()),
                Form::List { items, .. } => match items.as_slice() {
                    [Form::Symbol { name: head, .. }, Form::Symbol { name: system, .. }, Form::Symbol { name: task, .. }]
                        if head == "task" =>
                    {
                        Requester::Task {
                            system: system.clone(),
                            task: task.clone(),
                        }
                    }
                    _ => return Err(Refusal::RequesterKind(requester.to_canonical())),
                },
                _ => return Err(Refusal::RequesterKind(requester.to_canonical())),
            };
            let Form::Symbol { name: fact, .. } = fact else {
                return Err(Refusal::Value {
                    clause: "bind",
                    found: form.to_canonical(),
                });
            };
            let provider = match provider {
                Form::Symbol { name, .. } => name.clone(),
                Form::List { .. } => return Err(Refusal::ProviderKind(provider.to_canonical())),
                _ => {
                    return Err(Refusal::Value {
                        clause: "bind",
                        found: form.to_canonical(),
                    })
                }
            };
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

/// Why a record does not read as `archogen-assignment/1`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Refusal {
    /// It is no eADL document: the reader's diagnostics.
    Unreadable(String),
    /// It names another format — or none.
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
    /// A requester neither a qualified name nor `(task SYSTEM TASK)`.
    RequesterKind(String),
    /// A provider of a kind `/1` does not name: an adapter, a catalog record, anything but a qualified name.
    ProviderKind(String),
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
            Self::RequesterKind(found) => write!(
                f,
                "`{found}` is no requester `{FORMAT}` names: a qualified name, or `(task SYSTEM TASK)`"
            ),
            Self::ProviderKind(found) => write!(
                f,
                "`{found}` is a provider kind `{FORMAT}` does not name: its providers are blocks and platforms, by \
                 their qualified names"
            ),
            Self::NotCanonical(found) => write!(
                f,
                "`{found}` is out of the record's canonical order, or a second binding for one requester and fact"
            ),
        }
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
