//! §3's token scan over a package an `assembly` declaration names (`M2.12.4.2`): `asm!` and `naked_asm!` as
//! §14.2 admits them, each template line by §14.3's `riscv64` dialect.
//!
//! One port source in §14.2's form — §14.1's probe's trap entry, reset and install, rewritten in it — and one
//! change of it per refused construction. Each change must be refused with a message naming its rule, so a check
//! that is removed, or that starts refusing for another reason, turns its test red.

mod common;

use archogen_catalog::hash::Catalog;
use archogen_catalog::package::{scan, scan_assembly};
use archogen_catalog::Code;
use common::*;

const CFG: &str = "#[cfg(all(target_arch = \"riscv64\", target_os = \"none\"))]";

/// A port in §14.2's form.
fn valid() -> String {
    format!(
        r#"//! A port, in §14.2's form.

pub extern "C" fn handler() -> usize {{
    7
}}

{CFG}
#[unsafe(naked)]
pub extern "C" fn trap_entry() {{
    ::core::arch::naked_asm!(
        "addi sp, sp, -16",
        "sd ra, 0(sp)",
        "call {{h}}",
        "ld ra, 0(sp)",
        "addi sp, sp, 16",
        "mret",
        h = sym handler,
    )
}}

{CFG}
#[unsafe(naked)]
pub extern "C" fn reset() {{
    ::core::arch::naked_asm!("la t0, {{e}}", "csrw mtvec, t0", "1:", "wfi", "j 1b", e = sym crate::trap_entry)
}}

{CFG}
#[inline(never)]
pub fn install() {{
    unsafe {{
        ::core::arch::asm!(
            "la {{t}}, {{e}}",
            "csrw mtvec, {{t}}",
            t = inout(reg) 0usize => _,
            e = sym trap_entry,
            options(nostack),
        )
    }};
}}

{CFG}
pub fn mask() -> usize {{
    let previous: usize;
    unsafe {{ ::core::arch::asm!("csrrci {{p}}, mstatus, 8", p = out(reg) previous) }};
    previous
}}

{CFG}
pub fn unmask(previous: usize) {{
    unsafe {{ ::core::arch::asm!("csrw mstatus, {{p}}", "csrr zero, mhartid", p = in(reg) previous) }};
}}
"#
    )
}

/// `valid()` with `from` replaced by `to`; `from` must occur exactly once.
fn changed(from: &str, to: &str) -> String {
    let text = valid();
    assert_eq!(
        text.matches(from).count(),
        1,
        "the fixture must hold `{from}` once"
    );
    text.replacen(from, to, 1)
}

/// `source` refused, with a reason holding `says`.
#[track_caller]
fn refused_source(source: &str, says: &str) {
    match scan_assembly(source.as_bytes(), "riscv64") {
        Ok(()) => {
            panic!("expected a refusal about `{says}`, and the source was admitted:\n{source}")
        }
        Err(f) => assert!(
            f.why.contains(says),
            "not about `{says}`: line {} column {}: {}",
            f.line,
            f.column,
            f.why
        ),
    }
}

/// `valid()` with `from` replaced by `to` is refused, with a reason holding `says`.
#[track_caller]
fn refused(from: &str, to: &str, says: &str) {
    refused_source(&changed(from, to), says);
}

/// `valid()` with `from` replaced by `to` is admitted.
#[track_caller]
fn admitted(from: &str, to: &str) {
    let source = changed(from, to);
    scan_assembly(source.as_bytes(), "riscv64")
        .unwrap_or_else(|f| panic!("line {} column {}: {}\n{source}", f.line, f.column, f.why));
}

const INSTALL: &str = "\"la {t}, {e}\",\n            \"csrw mtvec, {t}\",";

#[test]
fn the_port_in_section_14_2s_form_is_admitted() {
    scan_assembly(valid().as_bytes(), "riscv64")
        .unwrap_or_else(|f| panic!("line {}: {}", f.line, f.why));
}

#[test]
fn outside_a_declared_package_assembly_is_refused_as_before() {
    let f = scan(valid().as_bytes()).unwrap_err();
    assert!(f.why.contains("the identifier `naked_asm`"), "{}", f.why);
}

// --- the invocation ---------------------------------------------------------------------------------------

#[test]
fn the_macro_is_written_out_in_full() {
    let says = "other than in `::core::arch::asm!(`";
    refused("::core::arch::asm!(\"csrrci", "asm!(\"csrrci", says);
    refused(
        "::core::arch::asm!(\"csrrci",
        "core::arch::asm!(\"csrrci",
        says,
    );
    refused(
        "unsafe { ::core::arch::asm!(\"csrrci",
        "unsafe { x::core::arch::asm!(\"csrrci",
        says,
    );
    refused(
        "//! A port, in §14.2's form.",
        "use ::core::arch::asm;",
        "other than in `::core::arch::asm!(`",
    );
    refused(
        "::core::arch::naked_asm!(\"la t0",
        "::core::arch::naked_asm![\"la t0",
        "naked_asm!(",
    );
    refused(
        "//! A port, in §14.2's form.",
        "::core::arch::global_asm!(\"nop\");",
        "the identifier `global_asm`",
    );
}

#[test]
fn an_invocation_lies_inside_a_function() {
    refused(
        "//! A port, in §14.2's form.",
        "static S: fn() = || unsafe { ::core::arch::asm!(\"nop\") };",
        "inside no function",
    );
}

#[test]
fn the_innermost_function_carries_the_cfg() {
    let says = "does not carry `#[cfg(all(target_arch = \"riscv64\", target_os = \"none\"))]`";
    refused(&format!("{CFG}\npub fn mask()"), "pub fn mask()", says);
    refused(
        &format!("{CFG}\npub fn mask()"),
        "#[cfg_attr(all(), cfg(all(target_arch = \"riscv64\", target_os = \"none\")))]\npub fn mask()",
        says,
    );
    refused(
        &format!("{CFG}\npub fn mask()"),
        "#[cfg(all(target_os = \"none\", target_arch = \"riscv64\"))]\npub fn mask()",
        says,
    );
    refused(
        &format!("{CFG}\npub fn mask()"),
        "#[cfg(all(target_arch = \"riscv32\", target_os = \"none\"))]\npub fn mask()",
        says,
    );
    refused(
        &format!("{CFG}\npub fn mask()"),
        "#[cfg(all(target_arch = r\"riscv64\", target_os = \"none\"))]\npub fn mask()",
        says,
    );
    // The innermost function is the one that must carry it.
    refused(
        "let previous: usize;\n    unsafe { ::core::arch::asm!(\"csrrci {p}, mstatus, 8\", p = out(reg) previous) };\n    previous",
        "fn inner() -> usize { let previous: usize; unsafe { ::core::arch::asm!(\"csrrci {p}, mstatus, 8\", p = out(reg) previous) }; previous }\n    inner()",
        says,
    );
    // Other attributes and qualifiers around it are no matter.
    admitted(
        &format!("{CFG}\npub fn mask()"),
        &format!("#[inline]\n{CFG}\n#[must_use]\npub(crate) const unsafe fn mask()"),
    );
}

// --- the arguments ----------------------------------------------------------------------------------------

#[test]
fn a_template_string_is_plain_and_one_line() {
    refused(
        "\"csrrci {p}, mstatus, 8\"",
        "r\"csrrci {p}, mstatus, 8\"",
        "raw or prefixed",
    );
    refused(
        "\"csrrci {p}, mstatus, 8\"",
        "\"csrrci {p}, mstatus, 8\\n\"",
        "holds `\\`",
    );
    refused(
        "\"csrrci {p}, mstatus, 8\"",
        "concat!(\"csrrci {p}, \", \"mstatus, 8\")",
        "`concat`: an argument",
    );
    refused(
        "p = out(reg) previous) }",
        "out(reg) previous) }",
        "`out`: an argument",
    );
}

#[test]
fn an_operand_is_named_and_of_an_admitted_kind() {
    refused(
        "p = out(reg) previous) }",
        "type = out(reg) previous) }",
        "the keyword `type`",
    );
    refused(
        "p = out(reg) previous) }",
        "r#p = out(reg) previous) }",
        "`r`: an argument",
    );
    refused(
        "p = out(reg) previous) }",
        "p = out(\"a0\") previous) }",
        "an explicit register",
    );
    refused(
        "p = out(reg) previous) }",
        "p = lateout(reg) previous) }",
        "`lateout`, refused",
    );
    refused(
        "p = in(reg) previous) }",
        "p = inlateout(reg) previous => _) }",
        "`inlateout`, refused",
    );
    refused(
        "p = out(reg) previous) }",
        "p = out(reg) previous, clobber_abi(\"C\")) }",
        "`clobber_abi`",
    );
    refused(
        "p = out(reg) previous) }",
        "p = label { }) }",
        "a `label` operand",
    );
    refused(
        "p = out(reg) previous) }",
        "p = out(reg) previous, #[cfg(x)] q = out(reg) _) }",
        "an attribute on an argument",
    );
    refused(
        "p = out(reg) previous) }",
        "p = out(reg) previous, p = out(reg) _) }",
        "named twice",
    );
    refused(
        "e = sym crate::trap_entry",
        "e = sym crate::trap_entry, x = in(reg) 0",
        "`in` in `naked_asm!`",
    );
    refused(
        "p = out(reg) previous) }",
        "p = out(reg)) }",
        "no expression",
    );
}

#[test]
fn options_is_nostack_alone_and_last() {
    refused(
        "options(nostack),",
        "options(nomem),",
        "an option other than `nostack`",
    );
    refused(
        "options(nostack),",
        "options(nostack, pure),",
        "an option other than `nostack`",
    );
    refused(
        "options(nostack),",
        "options(nostack), options(nostack),",
        "a second `options`",
    );
    refused(
        "options(nostack),",
        "options(nostack), x = out(reg) _,",
        "after `options`",
    );
    refused(
        "e = sym crate::trap_entry",
        "e = sym crate::trap_entry, options(nostack)",
        "`options` in `naked_asm!`",
    );
    admitted("options(nostack),", "options(nostack,),");
    admitted("options(nostack),", "");
}

#[test]
fn the_arguments_come_in_order() {
    refused(
        "p = out(reg) previous) }",
        "p = out(reg) previous, \"nop\") }",
        "out of §14.2's order",
    );
    refused(
        "::core::arch::asm!(\"csrrci {p}, mstatus, 8\", p = out(reg) previous)",
        "::core::arch::asm!(p = out(reg) previous, \"csrrci {p}, mstatus, 8\")",
        "before any template string",
    );
    refused(
        "::core::arch::asm!(\"csrrci {p}, mstatus, 8\", p = out(reg) previous)",
        "::core::arch::asm!()",
        "no template string",
    );
}

#[test]
fn a_sym_path_is_concrete() {
    let says = "a `sym` path §14.2 does not admit";
    refused("h = sym handler,", "h = sym handler::<u8>,", says);
    refused("h = sym handler,", "h = sym Self::handler,", says);
    refused("h = sym handler,", "h = sym ::handler,", says);
    refused("h = sym handler,", "h = sym crate,", says);
    refused("h = sym handler,", "h = sym r#handler,", says);
    admitted("h = sym handler,", "h = sym self::handler,");
    admitted("h = sym handler,", "h = sym super::super::port::handler,");
}

#[test]
fn a_sym_operand_has_no_generic_context() {
    let says = "a `sym` operand where an enclosing function";
    refused(
        "pub extern \"C\" fn reset() {",
        "pub extern \"C\" fn reset<T>() {",
        says,
    );
    refused(
        "pub extern \"C\" fn reset() {",
        "pub extern \"C\" fn reset<const N: usize>() {",
        says,
    );
    refused(
        "pub extern \"C\" fn reset() {",
        "pub extern \"C\" fn reset(_x: impl Copy) {",
        says,
    );
    admitted(
        "pub extern \"C\" fn reset() {",
        "pub extern \"C\" fn reset<'a>(_x: &'a u8) {",
    );
    // An enclosing generic `impl`, or any `trait`.
    let install = format!("{CFG}\n#[inline(never)]\npub fn install() {{");
    let source = changed(
        &install,
        &format!("pub struct S<T>(T);\nimpl<T> S<T> {{\n{install}"),
    );
    let source = source.replacen("    };\n}\n", "    };\n}\n}\n", 1);
    refused_source(&source, says);
    let source = changed(&install, &format!("pub trait P {{\n{install}"));
    let source = source.replacen("    };\n}\n", "    };\n}\n}\n", 1);
    refused_source(&source, says);
    // A plain `impl` is no matter, and nor is a generic one for an invocation with no `sym`.
    let source = changed(&install, &format!("pub struct S;\nimpl S {{\n{install}"));
    let source = source.replacen("    };\n}\n", "    };\n}\n}\n", 1);
    scan_assembly(source.as_bytes(), "riscv64").unwrap_or_else(|f| panic!("{}", f.why));
    let mask = format!("{CFG}\npub fn mask()");
    let source = changed(
        &mask,
        &format!("pub struct G<T>(T);\nimpl<T> G<T> {{\n{mask}"),
    );
    let source = source.replacen("    previous\n}\n", "    previous\n}\n}\n", 1);
    scan_assembly(source.as_bytes(), "riscv64").unwrap_or_else(|f| panic!("{}", f.why));
}

#[test]
fn a_placeholder_names_an_operand_and_nothing_more() {
    refused("\"call {h}\"", "\"call {}\"", "`{}` and `{<digits>}`");
    refused("\"call {h}\"", "\"call {0}\"", "`{}` and `{<digits>}`");
    refused("\"call {h}\"", "\"call {h:e}\"", "a modifier or a space");
    refused("\"call {h}\"", "\"call {g}\"", "`{g}` names no operand");
}

// --- the template ----------------------------------------------------------------------------------------

#[test]
fn a_line_holds_no_directive_comment_or_separator() {
    refused(
        "\"mret\",\n        h",
        "\".word 0\",\n        \"mret\",\n        h",
        "a directive",
    );
    refused(
        "\"mret\",\n        h",
        "\"mret # done\",\n        h",
        "a comment",
    );
    refused(
        "\"mret\",\n        h",
        "\"mret // done\",\n        h",
        "a comment",
    );
    refused("\"mret\",\n        h", "\"nop; mret\",\n        h", "`;`");
    refused("\"mret\",\n        h", "\"mret\t\",\n        h", "a tab");
    refused("\"call {h}\"", "\"call {{h}}\"", "`{{` or `}}`");
    refused(
        "\"addi sp, sp, -16\"",
        "\"addi sp, sp, %lo(x)\"",
        "a relocation operator",
    );
}

#[test]
fn a_name_goes_only_where_its_kind_does() {
    refused(
        "\"call {h}\"",
        "\"call handler\"",
        "a code position takes a placeholder",
    );
    refused(
        "\"call {h}\"",
        "\"call mepc\"",
        "a code position takes a placeholder",
    );
    refused(
        "\"la t0, {e}\"",
        "\"la t0, t1\"",
        "a code position takes a placeholder",
    );
    refused("\"j 1b\"", "\"j mstatus\"", "a label is a label number");
    refused(
        "\"csrw mtvec, t0\"",
        "\"csrw 773, t0\"",
        "not a system register",
    );
    refused(
        "\"csrw mtvec, t0\"",
        "\"csrw mtvec, mepc\"",
        "not a register",
    );
    refused(
        "\"addi sp, sp, -16\"",
        "\"addi sp, sp, {h}\"",
        "not `const`, in an integer's position",
    );
    refused("\"call {h}\"", "\"call {h}, ra\"", "takes 1 operands");
    refused(
        "\"addi sp, sp, -16\"",
        "\"addi sp, sp,  -16\"",
        "is not an integer",
    );
    refused(
        "\"addi sp, sp, -16\"",
        "\"addi sp,sp, -16\"",
        "takes 3 operands",
    );
    refused(
        "\"mret\",\n        h",
        "\"c.j 1f\",\n        \"mret\",\n        h",
        "not a mnemonic §14.3 lists",
    );
    refused(
        "\"mret\",\n        h",
        "\"jal ra, 0\",\n        \"mret\",\n        h",
        "not a mnemonic §14.3 lists",
    );
    refused(
        "\"mret\",\n        h",
        "\"mret zero\",\n        h",
        "takes 0 operands",
    );
}

#[test]
fn an_integer_is_decimal_and_within_64_bits() {
    refused(
        "\"addi sp, sp, -16\"",
        "\"addi sp, sp, -016\"",
        "a leading `0`",
    );
    refused("\"addi sp, sp, -16\"", "\"addi sp, sp, -0\"", "`-0`");
    refused(
        "\"addi sp, sp, -16\"",
        "\"addi sp, sp, 18446744073709549568\"",
        "outside the signed 64-bit range",
    );
    refused(
        "\"addi sp, sp, -16\"",
        "\"addi sp, sp, 0x10\"",
        "is not an integer",
    );
    admitted(
        "\"addi sp, sp, -16\"",
        "\"addi sp, sp, -9223372036854775808\"",
    );
    admitted("\"addi sp, sp, -16\"", "\"addi sp, sp, 0\"");
    // A `const` placeholder, with no `-` before it.
    let with_const = changed(
        "h = sym handler,",
        "h = sym handler,\n        c = const 16,",
    );
    let source = with_const.replacen("\"addi sp, sp, -16\"", "\"addi sp, sp, {c}\"", 1);
    scan_assembly(source.as_bytes(), "riscv64").unwrap_or_else(|f| panic!("{}", f.why));
    let source = with_const.replacen("\"addi sp, sp, -16\"", "\"addi sp, sp, -{c}\"", 1);
    refused_source(&source, "alone in its position");
}

#[test]
fn a_memory_operand_is_an_integer_then_a_register() {
    refused(
        "\"sd ra, 0(sp)\"",
        "\"sd ra, sp\"",
        "a memory operand is an integer",
    );
    refused("\"sd ra, 0(sp)\"", "\"sd ra, 08(sp)\"", "a leading `0`");
    refused("\"sd ra, 0(sp)\"", "\"sd ra, 0(mepc)\"", "not a register");
    refused(
        "\"sd ra, 0(sp)\"",
        "\"sd ra, {h}\"",
        "a memory operand is an integer",
    );
}

#[test]
fn a_label_reference_resolves_in_its_invocation() {
    refused("\"j 1b\"", "\"j 1f\"", "`1f` resolves to no label line");
    refused("\"j 1b\"", "\"j 2b\"", "`2b` resolves to no label line");
    refused("\"1:\"", "\"01:\"", "a label number is");
    refused("\"1:\"", "\"10000:\"", "a label number is");
    admitted(
        "\"1:\", \"wfi\", \"j 1b\"",
        "\"9999:\", \"wfi\", \"j 9999b\"",
    );
    admitted(
        "\"1:\", \"wfi\", \"j 1b\"",
        "\"j 1f\", \"1:\", \"wfi\", \"j 1b\"",
    );
}

#[test]
fn a_naked_body_ends_in_a_line_that_never_falls_through() {
    let says = "a naked body's last line";
    refused(
        "\"mret\",\n        h",
        "\"mret\",\n        \"nop\",\n        h",
        says,
    );
    refused(
        "\"mret\",\n        h",
        "\"mret\",\n        \"2:\",\n        h",
        says,
    );
    for last in ["ret", "jr t0", "tail {h}", "j 1b"] {
        admitted(
            "\"mret\",\n        h",
            &format!("\"1:\",\n        \"{last}\",\n        h"),
        );
    }
}

#[test]
fn an_asm_declares_every_register_it_touches() {
    refused("\"csrrci {p}, mstatus, 8\"", "\"ret\"", "not marked inline");
    refused(
        "\"csrrci {p}, mstatus, 8\"",
        "\"1:\"",
        "a label line in `asm!`",
    );
    refused(
        "\"csrrci {p}, mstatus, 8\"",
        "\"csrrci a0, mstatus, 8\"",
        "is `zero` or a placeholder",
    );
    refused(
        "\"csrw mstatus, {p}\"",
        "\"li {p}, 1\"",
        "writes it, and it is an `in` operand",
    );
    refused(
        INSTALL,
        "\"la {t}, {e}\",\n            \"csrw mtvec, {t}\",\n            \"mv {o}, {t}\",",
        "names no operand",
    );
    let reads_out = changed("t = inout(reg) 0usize => _,", "t = out(reg) _,");
    refused_source(&reads_out, "reads it, and it is an `out` operand");
    admitted(
        "\"csrrci {p}, mstatus, 8\"",
        "\"csrrci zero, mstatus, 8\", \"li {p}, 0\"",
    );
}

#[test]
fn mhartid_is_only_read() {
    refused(
        "\"csrr zero, mhartid\"",
        "\"csrw mhartid, {p}\"",
        "`mhartid` is read-only",
    );
    refused(
        "\"csrr zero, mhartid\"",
        "\"csrrs zero, mhartid, zero\"",
        "`mhartid` is read-only",
    );
}

// --- through the catalog ---------------------------------------------------------------------------------

#[test]
fn the_catalog_scans_a_declared_package_by_its_dialect() {
    let record = |declaration: &str| {
        edit(
            &edit(
                &packaged("example.base", "p"),
                "(sources \"crates/p\"))",
                &format!("(sources \"crates/p\") {declaration})"),
            ),
            "(targets example-target)",
            "(targets rv)",
        )
    };
    let files = |lib: String| {
        let mut extra: Vec<(&str, String)> = package()
            .into_iter()
            .map(|(p, t)| (p, t.to_owned()))
            .collect();
        extra.retain(|(p, _)| *p != "crates/p/src/lib.rs");
        extra.push(("crates/p/src/lib.rs", lib));
        extra.push((
            "targets/rv.env",
            "TARGET_ID=rv\nRUST_TARGET=riscv64imac-unknown-none-elf\n".to_owned(),
        ));
        extra.push(("targets/rv.eadl", "(platform)\n".to_owned()));
        extra
    };
    let load = |record: &str, lib: String| {
        let extra = files(lib);
        let extra: Vec<(&str, &str)> = extra.iter().map(|(p, t)| (*p, t.as_str())).collect();
        Catalog::read(tree(&[record], &extra)).and_then(|c| c.hashes())
    };
    load(&record("(assembly riscv64 \"crates/p\")"), valid()).unwrap_or_else(|e| panic!("{e}"));
    let r = load(&record(""), valid()).unwrap_err();
    assert_eq!(r.code, Code::Source, "{r}");
    assert!(r.message.contains("the identifier `naked_asm`"), "{r}");
    let r = load(
        &record("(assembly riscv64 \"crates/p\")"),
        changed("\"call {h}\"", "\"call handler\""),
    )
    .unwrap_err();
    assert_eq!(r.code, Code::Source, "{r}");
    assert!(r.message.contains("crates/p/src/lib.rs"), "{r}");
    assert!(
        r.message.contains("a code position takes a placeholder"),
        "{r}"
    );
}

// --- the arms the mutation matrix asked for ---------------------------------------------------------------

#[test]
fn the_bang_is_part_of_the_sequence() {
    refused(
        "::core::arch::asm!(\"csrrci {p}, mstatus, 8\", p = out(reg) previous)",
        "::core::arch::asm((\"csrrci {p}, mstatus, 8\", p = out(reg) previous))",
        "other than in `::core::arch::asm!(`",
    );
}

#[test]
fn the_cfg_names_target_arch_itself() {
    refused(
        &format!("{CFG}\npub fn mask()"),
        "#[cfg(all(target_vendor = \"riscv64\", target_os = \"none\"))]\npub fn mask()",
        "does not carry",
    );
    refused(
        &format!("{CFG}\npub fn mask()"),
        "#[cfg(all(target_arch = \"riscv64\", target_family = \"none\"))]\npub fn mask()",
        "does not carry",
    );
}

#[test]
fn the_attributes_read_are_the_ones_just_before_the_function() {
    // A bracket group that is no attribute ends them, whatever stands before it.
    refused(
        &format!("{CFG}\npub fn mask()"),
        &format!("{CFG}\nx [y]\npub fn mask()"),
        "does not carry",
    );
}

#[test]
fn an_empty_argument_is_refused() {
    refused(
        "p = out(reg) previous) }",
        ", p = out(reg) previous) }",
        "an empty argument",
    );
}

#[test]
fn a_const_operand_has_an_expression() {
    refused(
        "h = sym handler,",
        "h = sym handler,\n        c = const,",
        "a `const` operand with no expression",
    );
}

#[test]
fn a_sym_path_has_a_separator_between_segments() {
    let says = "a `sym` path §14.2 does not admit";
    refused("h = sym handler,", "h = sym crate x y handler,", says);
    refused("h = sym handler,", "h = sym handler x y z,", says);
}

#[test]
fn an_impl_trait_in_return_position_opens_no_impl() {
    admitted("pub fn mask() -> usize {", "pub fn mask() -> impl Copy {");
}

#[test]
fn a_backward_label_is_an_earlier_one() {
    refused(
        "\"1:\", \"wfi\", \"j 1b\"",
        "\"j 2b\", \"2:\", \"wfi\", \"j 2b\"",
        "`2b` resolves to no label line",
    );
}

#[test]
fn a_sym_placeholder_is_no_register() {
    refused(
        INSTALL,
        "\"la {t}, {e}\",\n            \"mv {t}, {e}\",",
        "a placeholder for a `sym` or `const` operand, in a register's position",
    );
}

#[test]
fn a_register_is_one_the_dialect_lists() {
    for name in ["x32", "x01", "t7", "s12", "a8", "f0", "pc"] {
        refused(
            "\"addi sp, sp, -16\"",
            &format!("\"addi {name}, sp, -16\""),
            "not a register",
        );
    }
    for name in ["x0", "x31", "t6", "s11", "a7", "fp", "gp", "tp", "zero"] {
        admitted("\"addi sp, sp, -16\"", &format!("\"addi {name}, sp, -16\""));
    }
}
