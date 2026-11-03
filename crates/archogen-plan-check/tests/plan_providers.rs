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

/// Rule 5's finding on the bare symbol `region` that `provider` offers.
fn bare_region(provider: &str) -> String {
    format!(
        "`{provider}` offers a region not read whole: the bare `region` names no byte range (rule 5)"
    )
}

#[test]
fn rule_5_a_bare_region_in_a_composed_block_is_invalid() {
    let composed = system(RAM, FAR, "(core-count 1 tick) region");
    let verdict = judged(&composed).unwrap_err();
    assert_eq!(verdict.status, Status::InvalidDescription, "{verdict:?}");
    // It owns no byte, so rule 10 has nothing of it to judge: this one finding, at its place, and no other.
    assert_eq!(verdict.findings.len(), 1, "{verdict:?}");
    assert_eq!(verdict.findings[0].message, bare_region("b"));
    assert_eq!(verdict.findings[0].primary.line, LINE_B);
    assert!(
        verdict.findings[0]
            .repair
            .starts_with("write `(region NAME (base B) (size S))`"),
        "{verdict:?}"
    );
    // Check (2) whole, past the language's passes that accept it: a plan never reaches check (5) with one.
    let (sources, program) = program(&composed);
    let verdict = description(&sources, &program, &RT_STATIC_UP_V1).unwrap_err();
    assert_eq!(verdict.status, Status::InvalidDescription, "{verdict:?}");
    assert!(
        verdict
            .findings
            .iter()
            .any(|f| f.code == PLAN_DESCRIPTION && f.message == bare_region("b")),
        "{verdict:?}"
    );
    // The regions beside it are still read and judged, and it outranks rule 10's `missing-fact`.
    let (status, messages) = refused(&system(
        RAM,
        "(region dev (base 0x8000_1000) (size 4 KiB))",
        "region",
    ));
    assert_eq!(status, Status::InvalidDescription, "{messages:?}");
    assert_eq!(
        messages,
        [
            bare_region("b"),
            "`a`'s region `dev` lies in `soc`'s placement memory `ram`, two claims on the same bytes (rule 10)"
                .to_owned()
        ]
    );
    let (status, messages) = refused(&system(
        "(region ram (base 0x8000_0000) (executable true))",
        FAR,
        "region",
    ));
    assert_eq!(status, Status::InvalidDescription, "{messages:?}");
    assert_eq!(
        messages,
        [
            bare_region("b"),
            "`soc`'s placement memory `ram` has no size: nothing can be laid out in it (rule 10)"
                .to_owned()
        ]
    );
}

#[test]
fn rule_5_a_bare_region_no_system_s_hardware_holds_is_invalid() {
    for (case, declarations, provider, line) in [
        // The symbol on a line of its own: the finding points at the offer, not at the block.
        (
            "a block, in a description with no system",
            "(defblock b\n  (offers (core-count 1 tick)\n    region))\n".to_owned(),
            "b",
            5,
        ),
        (
            "a block beside a system that does not compose it",
            format!(
                "{}(defblock stray (offers region))\n",
                system(RAM, FAR, FAR)
            ),
            "stray",
            7,
        ),
        (
            "a platform, in a description with no system",
            "(defplatform p (offers region))\n".to_owned(),
            "p",
            3,
        ),
    ] {
        let verdict = judged(&declarations).unwrap_err();
        assert_eq!(verdict.status, Status::InvalidDescription, "{case}");
        assert_eq!(verdict.findings.len(), 1, "{case}: {verdict:?}");
        assert_eq!(verdict.findings[0].message, bare_region(provider), "{case}");
        assert_eq!(verdict.findings[0].primary.line, line, "{case}");
    }
}

#[test]
fn rule_5_only_an_offer_of_the_symbol_itself_is_a_bare_region() {
    // The list with nothing in it is the same nothing, refused as every list that names nothing is.
    let (status, messages) = refused("(defblock t (offers (region)))\n");
    assert_eq!(status, Status::InvalidDescription, "{messages:?}");
    assert_eq!(
        messages,
        ["`t` offers a region not read whole: it names nothing (rule 5)"]
    );
    for (case, declaration) in [
        (
            "declared absent, it is no offer",
            "(defblock t (absent region))\n",
        ),
        ("another fact's name", "(defblock t (offers regions))\n"),
        (
            "a region with no size reads, owning its base byte",
            "(defblock t (offers (region r (base 0x1000))))\n",
        ),
    ] {
        assert_eq!(judged(declaration), Ok(()), "{case}");
    }
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
    // A reserved range is a hole the plan avoids, so its extent matters as a device's does: one with no size below the
    // memory leaves the description a fact short; a sized hole inside, or one with no size above the memory's end,
    // decides itself.
    let (status, messages) = refused(&system(
        ram,
        "(region rom (base 0x7fff_0000) (reserved true))",
        "(region other (base 0x0) (size 4 KiB))",
    ));
    assert_eq!(status, Status::MissingFact, "{messages:?}");
    assert!(
        messages.iter().any(|m| m.contains(
            "whether `a`'s reserved range `rom` is a hole in `soc`'s placement memory `ram`"
        )),
        "{messages:?}"
    );
    assert_eq!(
        judged(&system(
            ram,
            "(region rom (base 0x9000_0000) (reserved true))",
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

/// The lines of `system`'s text that declare `a`, `b` and `soc`, each a finding's place.
const LINE_A: u32 = 3;
const LINE_B: u32 = 4;
const LINE_SOC: u32 = 5;

/// A region out of placement memory's way.
const FAR: &str = "(region other (base 0x0) (size 4 KiB))";

/// Placement memory of 128 MiB, `[0x8000_0000, 0x8800_0000)`.
const RAM: &str = "(region ram (base 0x8000_0000) (size 128 MiB) (executable true))";

/// Rule 10's finding on `soc`'s `ram`, reserved surely in every byte, for the system `s`.
const EVERY_BYTE: &str = "`soc`'s placement memory `ram` is reserved in every byte, leaving nothing to place in for `s` \
                          (rule 10)";

#[test]
fn rule_10_placement_memory_with_no_size_is_missing_fact_naming_it() {
    let ram = "(region ram (base 0x8000_0000) (executable true))";
    let said =
        "`soc`'s placement memory `ram` has no size: nothing can be laid out in it (rule 10)";
    // Nothing need lie against it, and what lies above it adds no finding of whether they overlap.
    for (case, against) in [
        ("nothing against it", FAR),
        (
            "a device's registers above it",
            "(region dev (base 0x9000_0000) (size 4 KiB))",
        ),
        (
            "a device with no size above it",
            "(region dev (base 0x9000_0000))",
        ),
        (
            "a reserved range above it",
            "(region fw (base 0x9000_0000) (size 1 MiB) (reserved true))",
        ),
        (
            "a reserved range with no size above it",
            "(region fw (base 0x9000_0000) (reserved true))",
        ),
        (
            "a reserved range with no size at its base",
            "(region fw (base 0x8000_0000) (reserved true))",
        ),
        (
            "placement memory with a size above it",
            "(region more (base 0x9000_0000) (size 1 MiB) (executable true))",
        ),
    ] {
        let (status, messages) = refused(&system(ram, against, FAR));
        assert_eq!(status, Status::MissingFact, "{case}: {messages:?}");
        assert_eq!(messages, [said], "{case}");
    }
    let verdict = judged(&system(ram, FAR, FAR)).unwrap_err();
    assert_eq!(verdict.findings[0].repair, "state the memory's `(size …)`");
    assert_eq!(verdict.findings[0].primary.line, LINE_SOC);
    // Two of them, each named, in the hardware's order.
    let (status, messages) = refused(&system(
        ram,
        "(region more (base 0x9000_0000) (executable true))",
        FAR,
    ));
    assert_eq!(status, Status::MissingFact, "{messages:?}");
    assert_eq!(
        messages,
        [
            "`a`'s placement memory `more` has no size: nothing can be laid out in it (rule 10)",
            said
        ]
    );
    // Its base byte is its own whatever its size: a claim on that byte is still refused, and it is still named.
    for (case, claim, refusal) in [
        (
            "a device's registers",
            "(region dev (base 0x8000_0000) (size 4 KiB))",
            "`a`'s region `dev` lies in `soc`'s placement memory `ram`",
        ),
        (
            "a device's registers with no size",
            "(region dev (base 0x8000_0000))",
            "`a`'s region `dev` lies in `soc`'s placement memory `ram`",
        ),
        (
            "placement memory",
            "(region more (base 0x8000_0000) (size 1 MiB) (executable true))",
            "`a`'s placement memory `more` and `soc`'s `ram` overlap",
        ),
    ] {
        let (status, messages) = refused(&system(ram, claim, FAR));
        assert_eq!(status, Status::InvalidDescription, "{case}: {messages:?}");
        assert!(
            messages.iter().any(|m| m.contains(refusal)) && messages.iter().any(|m| m == said),
            "{case}: {messages:?}"
        );
    }
    // Hardware no system has is judged by no system; reserved, an executable region is no placement memory.
    assert_eq!(
        judged(&format!(
            "{}(defblock stray (offers {ram}))\n",
            system(RAM, FAR, FAR)
        )),
        Ok(())
    );
    assert_eq!(
        judged(&system(
            "(region rom (base 0x9000_0000) (executable true) (reserved true))",
            FAR,
            FAR
        )),
        Ok(())
    );
}

#[test]
fn rule_10_a_region_with_no_size_is_undecided_against_placement_memory_only_below_it() {
    let dev = |base: &str| format!("(region dev (base {base}))");
    let hole = |base: &str| format!("(region fw (base {base}) (reserved true))");
    // Below the memory, by one byte too: only its size says whether it reaches the memory.
    for (case, region, said) in [
        (
            "a device's registers",
            dev("0x7fff_ffff"),
            "whether `a`'s region `dev` lies in `soc`'s placement memory `ram` is undecided: it states no size and the \
             memory's bytes lie above its base (rule 10, as rule 5 reads it)",
        ),
        (
            "a reserved range",
            hole("0x7fff_ffff"),
            "whether `a`'s reserved range `fw` is a hole in `soc`'s placement memory `ram` is undecided: it states no \
             size and the memory's bytes lie above its base (rule 10, as rule 5 reads it)",
        ),
    ] {
        let (status, messages) = refused(&system(RAM, &region, FAR));
        assert_eq!(status, Status::MissingFact, "{case}: {messages:?}");
        assert_eq!(messages, [said], "{case}");
    }
    // Inside the memory a device's registers surely overlap it, by rule 5's base byte.
    for base in ["0x8000_0000", "0x8001_0000", "0x87ff_ffff"] {
        let (status, messages) = refused(&system(RAM, &dev(base), FAR));
        assert_eq!(status, Status::InvalidDescription, "{base}: {messages:?}");
        assert_eq!(
            messages,
            ["`a`'s region `dev` lies in `soc`'s placement memory `ram`, two claims on the same bytes (rule 10)"],
            "{base}"
        );
    }
    // At or above the memory's end, neither reaches it; a sized region below it decides itself.
    for region in [
        dev("0x8800_0000"),
        hole("0x8800_0000"),
        "(region dev (base 0x7fff_f000) (size 4 KiB))".to_owned(),
        "(region fw (base 0x7fff_f000) (size 4 KiB) (reserved true))".to_owned(),
    ] {
        assert_eq!(judged(&system(RAM, &region, FAR)), Ok(()), "{region}");
    }
    // Below memory with no size too: both are said, the undecided overlap and the memory's own missing size.
    let (status, messages) = refused(&system(
        "(region ram (base 0x8000_0000) (executable true))",
        &dev("0x7fff_ffff"),
        FAR,
    ));
    assert_eq!(status, Status::MissingFact, "{messages:?}");
    assert!(
        messages
            .iter()
            .any(|m| m.contains("whether `a`'s region `dev`"))
            && messages.iter().any(|m| m.contains("`ram` has no size")),
        "{messages:?}"
    );
    // A hole in one placement region, below another: taken to the first's end, it is still undecided against the second.
    let (status, messages) = refused(&system(
        &format!("{RAM} (region more (base 0x9000_0000) (size 1 MiB) (executable true))"),
        &hole("0x8001_0000"),
        FAR,
    ));
    assert_eq!(status, Status::MissingFact, "{messages:?}");
    assert_eq!(messages.len(), 1, "{messages:?}");
    assert!(
        messages[0].contains("`fw` is a hole in `soc`'s placement memory `more` is undecided"),
        "{messages:?}"
    );
}

#[test]
fn rule_10_a_hole_with_no_size_is_taken_to_the_memory_s_end_and_may_leave_free_bytes() {
    for (case, a, b) in [
        (
            "the bytes below it free",
            "(region fw (base 0x8001_0000) (reserved true))",
            FAR,
        ),
        (
            "at the memory's last byte",
            "(region fw (base 0x87ff_ffff) (reserved true))",
            FAR,
        ),
        (
            "the memory's first byte alone free",
            "(region fw (base 0x8000_0001) (reserved true))",
            FAR,
        ),
        (
            "a gap below it, above a sized hole",
            "(region lo (base 0x8000_0000) (size 2 MiB) (reserved true))",
            "(region hi (base 0x8030_0000) (reserved true))",
        ),
        (
            "two of them, the bytes below both free",
            "(region lo (base 0x8001_0000) (reserved true))",
            "(region hi (base 0x8002_0000) (reserved true))",
        ),
    ] {
        assert_eq!(judged(&system(RAM, a, b)), Ok(()), "{case}");
    }
}

#[test]
fn rule_10_holes_with_no_size_leaving_no_free_byte_are_missing_fact_naming_each() {
    let said = |named: &str| {
        format!(
            "`soc`'s placement memory `ram` has no free byte for `s` with its holes of no size, {named}, taken to reach \
             its end, and the bytes surely reserved do not fill it (rule 10)"
        )
    };
    for (case, a, b, named, line) in [
        (
            "at the memory's base",
            "(region fw (base 0x8000_0000) (reserved true))",
            FAR,
            "`a`'s `fw`",
            LINE_A,
        ),
        (
            "a sized hole up to its base",
            "(region lo (base 0x8000_0000) (size 2 MiB) (reserved true))",
            "(region hi (base 0x8020_0000) (reserved true))",
            "`b`'s `hi`",
            LINE_B,
        ),
        (
            "a sized hole past its base",
            "(region lo (base 0x8000_0000) (size 3 MiB) (reserved true))",
            "(region hi (base 0x8020_0000) (reserved true))",
            "`b`'s `hi`",
            LINE_B,
        ),
        (
            "a sized hole from below the memory up to its base",
            "(region lo (base 0x7000_0000) (size 258 MiB) (reserved true))",
            "(region hi (base 0x8020_0000) (reserved true))",
            "`b`'s `hi`",
            LINE_B,
        ),
        (
            "a sized hole above it",
            "(region fw (base 0x8000_0000) (reserved true))",
            "(region lo (base 0x8020_0000) (size 2 MiB) (reserved true))",
            "`a`'s `fw`",
            LINE_A,
        ),
        (
            "two of them, the higher adding no byte",
            "(region lo (base 0x8000_0000) (reserved true))",
            "(region hi (base 0x8020_0000) (reserved true))",
            "`a`'s `lo`, `b`'s `hi`",
            LINE_A,
        ),
        (
            "surely reserving all but the last byte",
            "(region lo (base 0x8000_0000) (size 134217726 byte) (reserved true))",
            "(region hi (base 0x87ff_fffe) (reserved true))",
            "`b`'s `hi`",
            LINE_B,
        ),
        (
            "one with no size above the memory's end, which is not named",
            "(region fw (base 0x8000_0000) (reserved true))",
            "(region top (base 0x9000_0000) (reserved true))",
            "`a`'s `fw`",
            LINE_A,
        ),
    ] {
        let verdict = judged(&system(RAM, a, b)).unwrap_err();
        assert_eq!(verdict.status, Status::MissingFact, "{case}");
        assert_eq!(verdict.findings.len(), 1, "{case}: {verdict:?}");
        assert_eq!(verdict.findings[0].message, said(named), "{case}");
        assert_eq!(
            verdict.findings[0].repair, "state the `(size …)` of each hole of no size",
            "{case}"
        );
        assert_eq!(verdict.findings[0].primary.line, line, "{case}");
    }
    // Judged of each placement region: another below it with free bytes does not answer it.
    let (status, messages) = refused(&system(
        &format!("(region more (base 0x4000_0000) (size 1 MiB) (executable true)) {RAM}"),
        "(region fw (base 0x8000_0000) (reserved true))",
        FAR,
    ));
    assert_eq!(status, Status::MissingFact, "{messages:?}");
    assert_eq!(messages, [said("`a`'s `fw`")]);
    // A range with no size below the memory is no hole so taken: all but the last byte reserved leaves that byte free.
    let (status, messages) = refused(&system(
        RAM,
        "(region low (base 0x7fff_0000) (reserved true))",
        "(region lo (base 0x8000_0000) (size 134217727 byte) (reserved true))",
    ));
    assert_eq!(status, Status::MissingFact, "{messages:?}");
    assert_eq!(messages.len(), 1, "{messages:?}");
    assert!(
        messages[0].contains("whether `a`'s reserved range `low`"),
        "{messages:?}"
    );
}

#[test]
fn rule_10_a_system_s_holes_are_those_of_its_hardware() {
    let (status, messages) = refused(
        "(defblock mem (offers (region ram (base 0x8000_0000) (size 128 MiB) (executable true))))\n\
         (defblock rom (offers (region fw (base 0x8000_0000) (reserved true))))\n\
         (defplatform soc.a (offers (core-count 1 tick)) (requires (uses mem rom)))\n\
         (defplatform soc.b (offers (core-count 1 tick)) (requires (uses mem)))\n\
         (defsystem s (platform (uses soc.a)))\n\
         (defsystem t (platform (uses soc.b)))\n",
    );
    assert_eq!(status, Status::MissingFact, "{messages:?}");
    assert_eq!(messages.len(), 1, "{messages:?}");
    assert!(
        messages[0].contains("`mem`'s placement memory `ram` has no free byte for `s`")
            && messages[0].contains("`rom`'s `fw`"),
        "{messages:?}"
    );
}

#[test]
fn rule_10_a_byte_surely_reserved_is_a_sized_hole_s_or_the_base_of_one_with_no_size() {
    let small =
        |size: &str| format!("(region ram (base 0x8000_0000) (size {size}) (executable true))");
    for (case, ram, a, b) in [
        (
            "one byte, one with no size at its base",
            small("1 byte"),
            "(region fw (base 0x8000_0000) (reserved true))",
            FAR,
        ),
        (
            "two bytes, two with no size",
            small("2 byte"),
            "(region lo (base 0x8000_0000) (reserved true))",
            "(region hi (base 0x8000_0001) (reserved true))",
        ),
        (
            "sized up to the last byte, one with no size at it",
            RAM.to_owned(),
            "(region lo (base 0x8000_0000) (size 134217727 byte) (reserved true))",
            "(region hi (base 0x87ff_ffff) (reserved true))",
        ),
        (
            "one with no size at the base, sized from the next byte",
            RAM.to_owned(),
            "(region lo (base 0x8000_0000) (reserved true))",
            "(region hi (base 0x8000_0001) (size 134217727 byte) (reserved true))",
        ),
    ] {
        let (status, messages) = refused(&system(&ram, a, b));
        assert_eq!(status, Status::InvalidDescription, "{case}: {messages:?}");
        assert_eq!(messages, [EVERY_BYTE], "{case}");
    }
    // Sized holes alone: still refused the same way.
    let (status, messages) = refused(&system(
        RAM,
        "(region lo (base 0x8000_0000) (size 64 MiB) (reserved true))",
        "(region hi (base 0x8400_0000) (size 64 MiB) (reserved true))",
    ));
    assert_eq!(status, Status::InvalidDescription, "{messages:?}");
    assert_eq!(messages, [EVERY_BYTE]);
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
