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

/// An operand's kind in a mnemonic's signature (§14.2, §14.3).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Operand {
    /// `R`: a register §14.3 lists, or a placeholder for a `reg` operand.
    Register,
    /// `C`: a system register §14.3 lists, and nothing else.
    System,
    /// `I`: an integer, or a placeholder for a `const` operand.
    Integer,
    /// `M`: an integer, then a register in parentheses.
    Memory,
    /// `S`: a placeholder for a `sym` operand.
    Code,
    /// `L`: a label number, then `b` or `f`.
    Label,
}

use Operand::{Code as S, Integer as I, Label as L, Memory as M, Register as R, System as C};

/// A mnemonic of `riscv64`: its signature, and whether an `asm!` admits it (§14.3's table).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Mnemonic {
    /// The mnemonic.
    pub name: &'static str,
    /// Each operand's kind, in order.
    pub signature: &'static [Operand],
    /// Whether §14.3 marks it *inline*.
    pub inline: bool,
}

const fn m(name: &'static str, signature: &'static [Operand], inline: bool) -> Mnemonic {
    Mnemonic {
        name,
        signature,
        inline,
    }
}

/// §14.3's mnemonics, row by row.
const MNEMONICS: [Mnemonic; 39] = [
    m("addi", &[R, R, I], false),
    m("andi", &[R, R, I], false),
    m("ori", &[R, R, I], false),
    m("xori", &[R, R, I], false),
    m("add", &[R, R, R], false),
    m("sub", &[R, R, R], false),
    m("and", &[R, R, R], false),
    m("or", &[R, R, R], false),
    m("xor", &[R, R, R], false),
    m("li", &[R, I], true),
    m("mv", &[R, R], true),
    m("ld", &[R, M], false),
    m("lw", &[R, M], false),
    m("sd", &[R, M], false),
    m("sw", &[R, M], false),
    m("csrr", &[R, C], true),
    m("csrw", &[C, R], true),
    m("csrs", &[C, R], true),
    m("csrc", &[C, R], true),
    m("csrwi", &[C, I], true),
    m("csrsi", &[C, I], true),
    m("csrci", &[C, I], true),
    m("csrrw", &[R, C, R], true),
    m("csrrs", &[R, C, R], true),
    m("csrrc", &[R, C, R], true),
    m("csrrwi", &[R, C, I], true),
    m("csrrsi", &[R, C, I], true),
    m("csrrci", &[R, C, I], true),
    m("beq", &[R, R, L], false),
    m("bne", &[R, R, L], false),
    m("beqz", &[R, L], false),
    m("bnez", &[R, L], false),
    m("la", &[R, S], true),
    m("call", &[S], false),
    m("tail", &[S], false),
    m("j", &[L], false),
    m("jr", &[R], false),
    m("ret", &[], false),
    m("mret", &[], false),
];

/// The two inline mnemonics with no operand, kept apart so [`MNEMONICS`] reads as §14.3's rows.
const NO_OPERAND_INLINE: [Mnemonic; 2] = [m("wfi", &[], true), m("nop", &[], true)];

/// The mnemonic `name` of `architecture`'s dialect; `None` for one its list lacks.
#[must_use]
pub fn mnemonic(architecture: &str, name: &str) -> Option<Mnemonic> {
    if architecture != "riscv64" {
        return None;
    }
    MNEMONICS
        .iter()
        .chain(&NO_OPERAND_INLINE)
        .find(|m| m.name == name)
        .copied()
}

/// Whether a line of `name` never falls through, so it may end a naked body: `mret`, `ret`, `jr`, `tail`, and `j`,
/// whose only operand is a label (§14.2).
#[must_use]
pub fn ends_a_body(name: &str) -> bool {
    matches!(name, "mret" | "ret" | "jr" | "tail" | "j")
}

/// Whether `name` is a register §14.3 lists: `x0`–`x31`, and the ABI names.
#[must_use]
pub fn is_register(name: &str) -> bool {
    let numbered = |prefix: &str, last: u32| {
        name.strip_prefix(prefix).is_some_and(|n| {
            !n.is_empty()
                && n.bytes().all(|b| b.is_ascii_digit())
                && (n == "0" || !n.starts_with('0'))
                && n.parse::<u32>().is_ok_and(|v| v <= last)
        })
    };
    matches!(name, "zero" | "ra" | "sp" | "gp" | "tp" | "fp")
        || numbered("x", 31)
        || numbered("t", 6)
        || numbered("s", 11)
        || numbered("a", 7)
}

/// Whether `name` is a system register §14.3 lists.
#[must_use]
pub fn is_system_register(name: &str) -> bool {
    matches!(
        name,
        "mstatus" | "mie" | "mip" | "mtvec" | "mscratch" | "mepc" | "mcause" | "mtval" | "mhartid"
    )
}
