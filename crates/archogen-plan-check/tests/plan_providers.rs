//! Check (2)'s provider-level refusals (leaf `M3.5.4.6`), `docs/semantics/model.md` §8 rules 3, 5 and 10: each case a
//! description read as the root module of a program, judged by the checker's own reading.

use archogen_outcome::Status;
use archogen_plan_check::check::{description, Verdict, PLAN_DESCRIPTION};
use archogen_plan_check::providers::refusals;
use eadl_front::module::{elaborate, MemoryModules};
use eadl_front::{Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;

fn program(declarations: &str) -> (SourceMap, Program) {
    let modules = MemoryModules::new().with(
        "app.case",
        &format!("(defmodule app.case\n  (version 1 0)\n{declarations})"),
    );
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, &modules, "app.case");
    assert!(
        !diagnostics.has_errors(),
        "{}",
        diagnostics.render(&sources)
    );
    (sources, program)
}

fn judged(declarations: &str) -> Result<(), Verdict> {
    let (sources, program) = program(declarations);
    refusals(&sources, &program)
}

/// The status and every message of a refusal, each finding check (2)'s code.
fn refused(declarations: &str) -> (Status, Vec<String>) {
    let verdict = judged(declarations).unwrap_err();
    assert!(
        verdict.findings.iter().all(|f| f.code == PLAN_DESCRIPTION),
        "{verdict:?}"
    );
    (
        verdict.status,
        verdict.findings.into_iter().map(|f| f.message).collect(),
    )
}

/// A system on `soc`, which uses the blocks `a` and `b`; `soc`, `a` and `b` offer what each argument holds.
fn system(soc: &str, a: &str, b: &str) -> String {
    format!(
        "(defblock a (offers {a}))\n(defblock b (offers {b}))\n\
         (defplatform soc (offers (core-count 1 tick) {soc}) (requires (uses a b)))\n\
         (defsystem s (platform (uses soc)))\n"
    )
}

#[test]
fn rule_3_a_consumed_function_with_no_channel_is_invalid() {
    for (case, offers) in [
        (
            "channels absent",
            "(offers (relative-delay true))\n  (absent channels)",
        ),
        (
            "channels zero",
            "(offers (relative-delay true) (channels 0))",
        ),
        (
            "the fact bare",
            "(offers relative-delay)\n  (absent channels)",
        ),
    ] {
        let (status, messages) = refused(&format!("(defblock t {offers})\n"));
        assert_eq!(status, Status::InvalidDescription, "{case}");
        assert!(
            messages[0].contains("relative-delay") && messages[0].contains("rule 3"),
            "{case}: {messages:?}"
        );
    }
    for (case, offers) in [
        ("a channel", "(offers (relative-delay true) (channels 1))"),
        (
            "the function false",
            "(offers (relative-delay false))\n  (absent channels)",
        ),
        (
            "a shared fact",
            "(offers counter-width)\n  (absent channels)",
        ),
    ] {
        assert_eq!(
            judged(&format!("(defblock t {offers})\n")),
            Ok(()),
            "{case}"
        );
    }
}

#[test]
fn rule_5_a_region_not_read_whole_is_invalid() {
    for (case, region) in [
        ("no base", "(region r (size 4 KiB))"),
        ("a negative base", "(region r (base -1) (size 4 KiB))"),
        ("a size in bits", "(region r (base 0x1000) (size 4 bit))"),
        (
            "a size of no storage",
            "(region r (base 0x1000) (size 4 ms))",
        ),
        ("a size of zero", "(region r (base 0x1000) (size 0 byte))"),
        (
            "a flag twice",
            "(region r (base 0x1000) (size 4 KiB) (executable true) (executable true))",
        ),
        (
            "a flag of another value",
            "(region r (base 0x1000) (size 4 KiB) (reserved maybe))",
        ),
        (
            "another sub-form",
            "(region r (base 0x1000) (size 4 KiB) (cached true))",
        ),
        ("no name", "(region (base 0x1000) (size 4 KiB))"),
    ] {
        let (status, messages) = refused(&format!("(defblock t (offers {region}))\n"));
        assert_eq!(status, Status::InvalidDescription, "{case}: {messages:?}");
        assert!(messages[0].contains("rule 5"), "{case}: {messages:?}");
    }
    assert_eq!(
        judged("(defblock t (offers (region r (base 0x1000) (size 4 KiB) (executable false) (reserved true))))\n"),
        Ok(())
    );
}

#[test]
fn rule_10_placement_memory_is_judged_over_the_system_s_hardware() {
    let ram = "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))";
    // The book's example: firmware reserved inside the memory is a hole, a device's registers in it are refused.
    let fw = "(region fw.range (base 0x8000_0000) (size 2 MiB) (reserved true))";
    assert_eq!(
        judged(&system(
            ram,
            fw,
            "(region dev.regs (base 0x1000_0000) (size 4 KiB))"
        )),
        Ok(())
    );
    let (status, messages) = refused(&system(
        ram,
        fw,
        "(region dev.regs (base 0x8000_1000) (size 4 KiB))",
    ));
    assert_eq!(status, Status::InvalidDescription);
    assert!(
        messages
            .iter()
            .any(|m| m.contains("`b`'s region `dev.regs` lies in `soc`'s placement memory `ram`")),
        "{messages:?}"
    );
    // Two placement regions overlapping; placement memory reserved in every byte.
    let (status, messages) = refused(&system(
        ram,
        "(region more (base 0x8100_0000) (size 64 MiB) (executable true))",
        "(region other (base 0x0) (size 4 KiB))",
    ));
    assert_eq!(status, Status::InvalidDescription);
    assert!(
        messages.iter().any(|m| m.contains("overlap")),
        "{messages:?}"
    );
    let (status, messages) = refused(&system(
        "(region ram (base 0x8000_0000) (size 4 MiB) (executable true))",
        "(region lo (base 0x8000_0000) (size 2 MiB) (reserved true))",
        "(region hi (base 0x8020_0000) (size 2 MiB) (reserved true))",
    ));
    assert_eq!(status, Status::InvalidDescription);
    assert!(
        messages
            .iter()
            .any(|m| m.contains("reserved in every byte")),
        "{messages:?}"
    );
    // A device region with no size below the memory, its extent upward unknown, leaves the overlap undecided; one above
    // the memory's end cannot reach it.
    let (status, messages) = refused(&system(
        ram,
        "(region dev (base 0x7fff_0000))",
        "(region other (base 0x0) (size 4 KiB))",
    ));
    assert_eq!(status, Status::MissingFact, "{messages:?}");
    assert_eq!(
        judged(&system(
            ram,
            "(region dev (base 0x9000_0000))",
            "(region other (base 0x0) (size 4 KiB))"
        )),
        Ok(())
    );
    // A block no system's hardware reaches is judged by no system.
    assert_eq!(
        judged(&format!(
            "{}(defblock stray (offers (region dev.regs (base 0x8000_1000) (size 4 KiB))))\n",
            system(ram, fw, "(region other (base 0x0) (size 4 KiB))")
        )),
        Ok(())
    );
}

#[test]
fn check_two_runs_the_refusals_after_the_language_s_passes() {
    let text = format!(
        "{}(defservice console.write (requires (needs observable-output)))\n",
        "(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine) \
         (region uart.regs (base 0x8000_1000) (size 256 byte))))\n\
         (defplatform soc.p (offers (core-count 1 tick) (reachable-at-privilege machine) \
         (region ram (base 0x8000_0000) (size 128 MiB) (executable true))) (requires (uses console.uart)))\n\
         (defsystem s\n  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) \
         (uses console.write))\n  (requires (uses console.write))\n  (platform (uses soc.p)))\n"
    );
    let (sources, program) = program(&text);
    let verdict = description(&sources, &program, &RT_STATIC_UP_V1).unwrap_err();
    assert_eq!(verdict.status, Status::InvalidDescription, "{verdict:?}");
    assert!(
        verdict
            .findings
            .iter()
            .any(|f| f.code == PLAN_DESCRIPTION && f.message.contains("rule 10")),
        "{verdict:?}"
    );
}
