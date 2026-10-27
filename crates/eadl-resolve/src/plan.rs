//! The plan the search writes (leaf `M3.4.3`): `archogen-assignment/1`, the assignment record the configuration
//! checker judges against the description (`docs/book/src/versions.md`, `assignment-format`). One format with two
//! writers in two crates that share none — this one and the checker's, `archogen_plan_check::assignment` — each with
//! its own constant and its own derivation of the program's digest, both held to one golden and to each other by
//! `tests/plan.rs`. A divergence is then a stale plan on every run, which the checker refuses, rather than a fault the
//! two would share.
//!
//! ⭐ **Its canonical form**, one clause per line, in this order:
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
//! — a task written `(task SYSTEM TASK)`, the bindings sorted by requester, every named one before every task, each
//! kind by its text, then by fact.

use archogen_digest::Digest;
use eadl_front::module::{Instance, Program, Version};
use eadl_front::Form;

use crate::search::Resolution;

/// The format's identifier: the search's own constant, held with the checker's to one golden.
pub const FORMAT: &str = "archogen-assignment/1";

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

/// `resolution` as the record's canonical text, resolved under `profile` for the program whose digest is `program`.
#[must_use]
pub fn write(resolution: &Resolution, profile: &str, program: &str) -> String {
    let mut out = format!(
        "(format \"{FORMAT}\")\n(profile {profile})\n(system {})\n(program \"{program}\")\n",
        resolution.system
    );
    if let Some((provider, region)) = &resolution.placement {
        out.push_str(&format!("(placement {provider} {region})\n"));
    }
    // The map's order is the record's: `Who` orders every named requester before every task, each by its text.
    for ((who, fact), provider) in &resolution.bindings {
        out.push_str(&format!("(bind {who} {fact} {provider})\n"));
    }
    out
}
