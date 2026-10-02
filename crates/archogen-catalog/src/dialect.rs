//! The assembly dialects (the record's §14.3, `docs/specs/catalog/decision_catalog-records-port.md`).
//!
//! > Its one target triple is `riscv64imac-unknown-none-elf`, one of the two `rust-toolchain.toml` pins, for code
//! > running in machine mode on one hart.
//!
//! An `assembly` declaration names one of these architectures, and every record reaching a package it declares
//! names targets whose `RUST_TARGET` is among its triples (§14.2). §14.2 also refuses two declarations of one
//! package that name different architectures; with one dialect, every declaration the reader admits names
//! `riscv64`, so that refusal cannot arise until a second dialect is added, a change to §14.3 that is a new rules
//! version (`M2.12.4.1`).

/// Each architecture §14.3 lists, with its target triples.
const DIALECTS: [(&str, &[&str]); 1] = [("riscv64", &["riscv64imac-unknown-none-elf"])];

/// The target triples of `architecture`; `None` for one §14.3 does not list.
#[must_use]
pub fn triples(architecture: &str) -> Option<&'static [&'static str]> {
    DIALECTS
        .iter()
        .find(|(name, _)| *name == architecture)
        .map(|(_, triples)| *triples)
}
