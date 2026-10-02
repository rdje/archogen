//! `cargo xtask pin-premises` — the port record's premises about the pinned toolchain, held as compile-only tests
//! (leaf `M2.12.4.4`; `docs/specs/catalog/decision_catalog-records-port.md`, How to apply).
//!
//! > A changed result is a change to §14.2 or §14.3, so a new rules version from the first lock on. `M2.12.4` holds
//! > these premises as compile-only tests built with the pinned toolchain, so the bump's own run fails first.
//!
//! The record prints five probes with their sha256: §14.1's naked-function probe and §14.4's four. Each is read from
//! the record, its hash checked, and built as the record says. §14.2's premises were measured in its reviews
//! without printed sources; their probes are written here, each named by the record's own words. Every premise is
//! built with the toolchain `rust-toolchain.toml` pins — the run refuses another — and judged on what the compiler,
//! the assembler or the linker answered: a message, an alignment, or the object code itself ([`crate::elf`]).
//!
//! It runs the compiler, so it is tooling, outside the product (`NO-SUBPROCESS`), and the integration tier runs it.
//! Exit 0: every premise holds. 1: one does not, named. 2: the record's probes or the pin could not be read.

use std::collections::BTreeMap;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::{Command, Output};

use archogen_evidence::sha256::Digest;

use crate::elf::{instructions, targets, writes, Elf};

/// The record that prints the probes.
const RECORD: &str = "docs/specs/catalog/decision_catalog-records-port.md";

/// §14.3's one target triple for `riscv64`.
const TARGET: &str = "riscv64imac-unknown-none-elf";

/// The probes the record prints, by the name it gives each.
const PROBES: [&str; 5] = [
    "nakedprobe",
    "lib.rs",
    "bin_main.rs",
    "two_handlers.rs",
    "cj.rs",
];

/// Every probe the record prints, read and hash-checked: each 64-digit hash in backticks, and the `rust` block after
/// it, its fence's indentation stripped, each line ending in a line feed. A probe in a list item is named by the file
/// name before its hash; §14.1's, which has none, is `nakedprobe`.
///
/// # Errors
///
/// A probe missing, a block not closed, or a source whose hash is not the one printed.
pub fn recorded(text: &str) -> Result<BTreeMap<String, String>, String> {
    let mut out = BTreeMap::new();
    let mut rest = text;
    let mut offset = 0;
    while let Some(start) = rest.find('`') {
        let after = &rest[start + 1..];
        let hex: String = after.chars().take_while(char::is_ascii_hexdigit).collect();
        let at = offset + start;
        offset += start + 1;
        rest = after;
        if hex.len() != 64 || !after[hex.len()..].starts_with('`') {
            continue;
        }
        let before = text[..at].trim_end();
        let name = before
            .strip_suffix('(')
            .map(str::trim_end)
            .and_then(|b| b.strip_suffix('`'))
            .and_then(|b| b.rsplit_once('`'))
            .map_or_else(|| "nakedprobe".to_owned(), |(_, name)| name.to_owned());
        let fence = text[at..]
            .find("```rust\n")
            .ok_or_else(|| format!("`{name}`: no `rust` block after its hash"))?;
        let fence_start = at + fence;
        let line_start = text[..fence_start].rfind('\n').map_or(0, |p| p + 1);
        let indent = &text[line_start..fence_start];
        let body_start = fence_start + "```rust\n".len();
        let mut lines = Vec::new();
        let mut closed = false;
        for line in text[body_start..].split('\n') {
            if line.trim() == "```" {
                closed = true;
                break;
            }
            lines.push(line.strip_prefix(indent).unwrap_or(line.trim_start()));
        }
        if !closed {
            return Err(format!("`{name}`: its block is not closed"));
        }
        let source = format!("{}\n", lines.join("\n"));
        let digest = Digest::of(source.as_bytes()).hex();
        if digest != hex {
            return Err(format!(
                "`{name}`: its printed source hashes to {digest}, not the {hex} printed with it"
            ));
        }
        out.insert(name, source);
    }
    for name in PROBES {
        if !out.contains_key(name) {
            return Err(format!("the record prints no probe `{name}`"));
        }
    }
    Ok(out)
}

/// The channel `rust-toolchain.toml` pins.
fn pinned_channel(root: &Path) -> Result<String, String> {
    let text = fs::read_to_string(root.join("rust-toolchain.toml")).map_err(|e| e.to_string())?;
    text.lines()
        .find_map(|l| l.strip_prefix("channel = \""))
        .and_then(|l| l.strip_suffix('"'))
        .map(str::to_owned)
        .ok_or_else(|| "rust-toolchain.toml names no channel".to_owned())
}

/// Where each probe is built: a directory under the repository, so `rust-toolchain.toml` picks the compiler.
struct Bench {
    dir: PathBuf,
}

/// What a build answered.
struct Answer {
    ok: bool,
    text: String,
}

impl From<Output> for Answer {
    fn from(o: Output) -> Self {
        Self {
            ok: o.status.success(),
            text: format!(
                "{}{}",
                String::from_utf8_lossy(&o.stdout),
                String::from_utf8_lossy(&o.stderr)
            ),
        }
    }
}

impl Bench {
    fn run(&self, program: &str, args: &[&str], env: &[(&str, &str)], at: &Path) -> Answer {
        let mut command = Command::new(program);
        command.args(args).current_dir(at);
        for (k, v) in env {
            command.env(k, v);
        }
        match command.output() {
            Ok(o) => o.into(),
            Err(e) => Answer {
                ok: false,
                text: format!("`{program}` did not run: {e}"),
            },
        }
    }

    /// `rustc` over `source`, written as `file` in its own directory, with `args`.
    fn rustc(
        &self,
        file: &str,
        source: &str,
        args: &[&str],
        env: &[(&str, &str)],
    ) -> Result<(PathBuf, Answer), String> {
        let dir = self.dir.join(file.replace('.', "_"));
        fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
        fs::write(dir.join(file), source).map_err(|e| e.to_string())?;
        let mut all = vec![file, "--edition", "2021", "--target", TARGET];
        all.extend_from_slice(args);
        let answer = self.run("rustc", &all, env, &dir);
        Ok((dir, answer))
    }

    /// A crate `name`, `crate-type = ["rlib"]` and `panic = "abort"` in its release profile, its library `source`,
    /// built by `cargo rustc` with `release` and `args` after `--`.
    fn cargo(
        &self,
        name: &str,
        source: &str,
        release: bool,
        args: &[&str],
    ) -> Result<(PathBuf, Answer), String> {
        let dir = self.dir.join(format!(
            "{name}-{}",
            if release { "release" } else { "debug" }
        ));
        fs::create_dir_all(dir.join("src")).map_err(|e| e.to_string())?;
        fs::write(
            dir.join("Cargo.toml"),
            format!(
                "[package]\nname = \"{name}\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[lib]\ncrate-type = [\"rlib\"]\n\n\
                 [profile.release]\npanic = \"abort\"\n\n[workspace]\n"
            ),
        )
        .map_err(|e| e.to_string())?;
        fs::write(dir.join("src/lib.rs"), source).map_err(|e| e.to_string())?;
        let mut all = vec!["rustc", "-q", "--target", TARGET];
        if release {
            all.push("--release");
        }
        all.push("--");
        all.extend_from_slice(args);
        let answer = self.run("cargo", &all, &[], &dir);
        Ok((dir, answer))
    }
}

/// The one file under `dir`'s build output named `<prefix>…<suffix>`.
fn built(dir: &Path, release: bool, prefix: &str, suffix: &str) -> Result<PathBuf, String> {
    let deps = dir
        .join("target")
        .join(TARGET)
        .join(if release { "release" } else { "debug" })
        .join("deps");
    fs::read_dir(&deps)
        .map_err(|e| format!("{}: {e}", deps.display()))?
        .filter_map(Result::ok)
        .map(|e| e.path())
        .find(|p| {
            p.file_name()
                .and_then(|n| n.to_str())
                .is_some_and(|n| n.starts_with(prefix) && n.ends_with(suffix))
        })
        .ok_or_else(|| format!("no `{prefix}*{suffix}` under {}", deps.display()))
}

/// The `.p2align` the symbol holding `part` is emitted under, from the compiler's assembly: the one in its section
/// before its label, wherever its `.globl` stands — before the alignment for a naked function, after it for the
/// others.
#[must_use]
pub fn alignment(asm: &str, part: &str) -> Option<u32> {
    let mut align = None;
    for line in asm.lines() {
        let line = line.trim();
        if let Some(n) = line.strip_prefix(".p2align") {
            align = n
                .trim()
                .split(',')
                .next()
                .and_then(|v| v.trim().parse().ok());
        } else if line.starts_with(".section") {
            align = None;
        } else if let Some(label) = line.strip_suffix(':') {
            if label.contains(part) && !label.starts_with('.') {
                return align;
            }
        }
    }
    None
}

/// The register each line of `asm` writing `instruction` names last, in order.
#[must_use]
pub fn operands(asm: &str, instruction: &str) -> Vec<String> {
    asm.lines()
        .filter_map(|l| l.trim().strip_prefix(instruction))
        .filter_map(|rest| rest.rsplit(',').next().map(|r| r.trim().to_owned()))
        .collect()
}

/// The registers a function's body writes, by `x` number.
fn body_writes(elf: &Elf<'_>, function: &str) -> Result<Vec<u8>, String> {
    let symbol = elf
        .function(function)
        .ok_or_else(|| format!("no function `{function}` in the object"))?;
    let (at, bytes) = elf.code(symbol)?;
    Ok(writes(&instructions(at, bytes)?))
}

/// One premise: the record's words, and its check.
struct Premise {
    says: &'static str,
    check: fn(&Bench, &BTreeMap<String, String>) -> Result<(), String>,
}

fn expect(answer: &Answer, ok: bool, holds: &str) -> Result<(), String> {
    if answer.ok != ok {
        return Err(format!(
            "the build {}, where the record says it {}: {}",
            if answer.ok { "succeeded" } else { "failed" },
            if ok { "succeeds" } else { "fails" },
            answer.text.trim()
        ));
    }
    if !holds.is_empty() && !answer.text.contains(holds) {
        return Err(format!(
            "the answer does not hold \"{holds}\": {}",
            answer.text.trim()
        ));
    }
    Ok(())
}

const PRELUDE: &str = "#![no_std]\n#[panic_handler]\nfn on_panic(_: &core::panic::PanicInfo) -> ! {\n    loop {}\n}\n";

/// A library of one naked function `f` with `template` and `operands`, built to an object.
fn naked(
    bench: &Bench,
    file: &str,
    template: &str,
    operands: &str,
) -> Result<(PathBuf, Answer), String> {
    let source = format!(
        "{PRELUDE}pub extern \"C\" fn target() {{}}\n#[unsafe(naked)]\npub extern \"C\" fn f() {{\n    ::core::arch::naked_asm!({template}{operands})\n}}\n"
    );
    bench.rustc(
        file,
        &source,
        &[
            "--crate-type",
            "rlib",
            "-C",
            "panic=abort",
            "-C",
            "opt-level=2",
            "--emit",
            "obj",
        ],
        &[],
    )
}

fn object(dir: &Path, file: &str) -> Result<Vec<u8>, String> {
    let stem = file.trim_end_matches(".rs");
    fs::read(dir.join(format!("{stem}.o"))).map_err(|e| format!("{stem}.o: {e}"))
}

const PREMISES: &[Premise] = &[
    Premise {
        says: "§14.1: both profiles gave the naked trap entry the 4-byte alignment `mtvec` needs; the ordinary functions were emitted `.p2align 1`",
        check: |bench, probes| {
            for release in [true, false] {
                let (dir, answer) = bench.cargo("nakedprobe", &probes["nakedprobe"], release, &["--emit", "asm"])?;
                expect(&answer, true, "")?;
                let asm = fs::read_to_string(built(&dir, release, "nakedprobe", ".s")?).map_err(|e| e.to_string())?;
                for (part, want) in [("10trap_entry", 2), ("5reset", 2), ("7handler", 1), ("7install", 1)] {
                    let got = alignment(&asm, part);
                    if got != Some(want) {
                        return Err(format!("`{part}` emitted `.p2align {got:?}`, not {want} (release: {release})"));
                    }
                }
            }
            Ok(())
        },
    },
    Premise {
        says: "§14.1: `call {h}` and `la t0, {e}` assembled to references to the mangled symbols of `handler` and `trap_entry`; no name was written",
        check: |bench, probes| {
            let (dir, answer) = bench.cargo("nakedprobe", &probes["nakedprobe"], true, &["--emit", "asm"])?;
            expect(&answer, true, "")?;
            let asm = fs::read_to_string(built(&dir, true, "nakedprobe", ".s")?).map_err(|e| e.to_string())?;
            for needle in ["call\t_ZN10nakedprobe7handler17h", "%pcrel_hi(_ZN10nakedprobe10trap_entry17h"] {
                if !asm.contains(needle) {
                    return Err(format!("the emitted assembly holds no `{needle}`"));
                }
            }
            Ok(())
        },
    },
    Premise {
        says: "§14.2: the assembler reads a label number of `2^32` or more modulo `2^32`",
        check: |bench, _| {
            let (_, aliased) = naked(bench, "label_aliased.rs", "\"4294967296:\", \"nop\", \"j 0b\"", "")?;
            expect(&aliased, true, "")?;
            let (_, other) = naked(bench, "label_other.rs", "\"4294967297:\", \"nop\", \"j 0b\"", "")?;
            expect(&other, false, "directional label undefined")
        },
    },
    Premise {
        says: "§14.2: the assembler reads a larger integer modulo `2^64`: `addi a0, a0, 18446744073709549568` assembled as `-2048`",
        check: |bench, _| {
            let (dir, wrapped) = naked(bench, "integer_wrapped.rs", "\"addi a0, a0, 18446744073709549568\", \"ret\"", "")?;
            expect(&wrapped, true, "")?;
            let bytes = object(&dir, "integer_wrapped.rs")?;
            let elf = Elf::parse(&bytes)?;
            let symbol = elf.function("1f17h").ok_or("no function `f`")?;
            let (at, code) = elf.code(symbol)?;
            let first = instructions(at, code)?.first().map(|(_, d)| d.kind);
            if first != Some(crate::elf::Kind::Addi { imm: -2048 }) {
                return Err(format!("the first instruction is {first:?}, not `addi` of -2048"));
            }
            let (_, other) = naked(bench, "integer_other.rs", "\"addi a0, a0, 18446744073709549567\", \"ret\"", "")?;
            expect(&other, false, "[-2048, 2047]")
        },
    },
    Premise {
        says: "§14.2: a `const` operand wider than 64 bits fails to assemble",
        check: |bench, _| {
            let (_, wide) = naked(bench, "const_wide.rs", "\"addi a0, a0, {c}\", \"ret\"", ", c = const 18446744073709551616u128")?;
            expect(&wide, false, "unknown operand")?;
            let (_, narrow) = naked(bench, "const_narrow.rs", "\"addi a0, a0, {c}\", \"ret\"", ", c = const 16u128")?;
            expect(&narrow, true, "")
        },
    },
    Premise {
        says: "§14.2: two `in` operands of one value may share one register (measured on the pin, in release builds)",
        check: |bench, _| {
            let source = format!(
                "{PRELUDE}#[inline(never)]\npub fn shared(x: usize) {{\n    unsafe {{ ::core::arch::asm!(\"csrw mscratch, {{a}}\", \"csrw mtvec, {{b}}\", a = in(reg) x, b = in(reg) x) }}\n}}\n"
            );
            let (dir, answer) = bench.rustc(
                "shared.rs",
                &source,
                &["--crate-type", "rlib", "-C", "panic=abort", "-C", "opt-level=2", "--emit", "asm"],
                &[],
            )?;
            expect(&answer, true, "")?;
            let asm = fs::read_to_string(dir.join("shared.s")).map_err(|e| e.to_string())?;
            let scratch = operands(&asm, "csrw\tmscratch,");
            let vector = operands(&asm, "csrw\tmtvec,");
            match (scratch.as_slice(), vector.as_slice()) {
                ([a], [b]) if a == b => Ok(()),
                _ => Err(format!("the two `in` operands were given {scratch:?} and {vector:?}")),
            }
        },
    },
    Premise {
        says: "§14.3: `call` writes `ra` and `tail` writes `t1`; `la` and `li` write only their register operand, a large `li` expanding into several instructions on it alone",
        check: |bench, _| {
            for (file, template, operands, written) in [
                ("writes_li.rs", "\"li a0, 1311768467463790320\", \"ret\"", "", vec![10]),
                ("writes_la.rs", "\"la a1, {t}\", \"ret\"", ", t = sym target", vec![11]),
                ("writes_call.rs", "\"call {t}\", \"ret\"", ", t = sym target", vec![1]),
                ("writes_tail.rs", "\"tail {t}\"", ", t = sym target", vec![6]),
            ] {
                let (dir, answer) = naked(bench, file, template, operands)?;
                expect(&answer, true, "")?;
                let bytes = object(&dir, file)?;
                let elf = Elf::parse(&bytes)?;
                let got = body_writes(&elf, "1f17h")?;
                if got != written {
                    return Err(format!("{template} writes registers {got:?}, not {written:?}"));
                }
            }
            Ok(())
        },
    },
    Premise {
        says: "§14.4: `rustc --print cfg` for the target gives `panic=\"abort\"`: the target's default",
        check: |bench, _| {
            let answer = bench.run("rustc", &["--print", "cfg", "--target", TARGET], &[], &bench.dir);
            expect(&answer, true, "panic=\"abort\"")
        },
    },
    Premise {
        says: "§14.4: `bin_main.rs` builds; with `-C panic=unwind` it is answered \"unwinding panics are not supported without std\"",
        check: |bench, probes| {
            let (_, plain) = bench.rustc("bin_main.rs", &probes["bin_main.rs"], &["--crate-type", "bin"], &[])?;
            expect(&plain, true, "")?;
            let (_, unwind) = bench.rustc("bin_main.rs", &probes["bin_main.rs"], &["--crate-type", "bin", "-C", "panic=unwind"], &[])?;
            expect(&unwind, false, "unwinding panics are not supported without std")
        },
    },
    Premise {
        says: "§14.4: with `-C panic=immediate-abort`, \"`-Cpanic=immediate-abort` requires `-Zunstable-options` and a nightly compiler\"",
        check: |bench, probes| {
            let (_, answer) = bench.rustc(
                "bin_main.rs",
                &probes["bin_main.rs"],
                &["--crate-type", "bin", "-C", "panic=immediate-abort"],
                &[],
            )?;
            expect(&answer, false, "`-Cpanic=immediate-abort` requires `-Zunstable-options` and a nightly compiler")
        },
    },
    Premise {
        says: "§14.4: even with `RUSTC_BOOTSTRAP=1` and `-Zunstable-options` the binary is refused, \"the crate `core` was compiled with a panic strategy which is incompatible with `immediate-abort`\", while a library builds",
        check: |bench, probes| {
            let args = ["-Zunstable-options", "-C", "panic=immediate-abort"];
            let bootstrap = [("RUSTC_BOOTSTRAP", "1")];
            let (_, binary) = bench.rustc("bin_main.rs", &probes["bin_main.rs"], &[&["--crate-type", "bin"][..], &args].concat(), &bootstrap)?;
            expect(&binary, false, "the crate `core` was compiled with a panic strategy which is incompatible with `immediate-abort`")?;
            let (_, library) = bench.rustc("bin_main.rs", &probes["bin_main.rs"], &[&["--crate-type", "rlib"][..], &args].concat(), &bootstrap)?;
            expect(&library, true, "")
        },
    },
    Premise {
        says: "§14.4: nor is `core` rebuilt: `RUSTC_BOOTSTRAP=1 cargo build -Zbuild-std=core` is refused, \"unable to build with the standard library\", on a toolchain installed from `rust-toolchain.toml` alone",
        check: |bench, probes| {
            let dir = bench.dir.join("build_std");
            fs::create_dir_all(dir.join("src")).map_err(|e| e.to_string())?;
            fs::write(
                dir.join("Cargo.toml"),
                "[package]\nname = \"build_std\"\nversion = \"0.1.0\"\nedition = \"2021\"\n\n[workspace]\n",
            )
            .map_err(|e| e.to_string())?;
            fs::write(dir.join("src/main.rs"), &probes["bin_main.rs"]).map_err(|e| e.to_string())?;
            let answer = bench.run(
                "cargo",
                &["build", "-Zbuild-std=core", "--target", TARGET],
                &[("RUSTC_BOOTSTRAP", "1")],
                &dir,
            );
            expect(&answer, false, "unable to build with the standard library")
        },
    },
    Premise {
        says: "§14.4: `two_handlers.rs`, built as `bin_main.rs` with `-C panic=abort`, is answered \"found duplicate lang item `panic_impl`\" (E0152)",
        check: |bench, probes| {
            let (_, answer) = bench.rustc(
                "two_handlers.rs",
                &probes["two_handlers.rs"],
                &["--crate-type", "bin", "-C", "panic=abort"],
                &[],
            )?;
            expect(&answer, false, "found duplicate lang item `panic_impl`")
        },
    },
    Premise {
        says: "§14.4: `lib.rs` calls `core::panicking::panic_bounds_check` and `core::panicking::panic_fmt`, which end in the handler",
        check: |bench, probes| {
            let (dir, answer) = bench.cargo("probe_lib", &probes["lib.rs"], true, &["--emit", "obj"])?;
            expect(&answer, true, "")?;
            let bytes = fs::read(built(&dir, true, "probe_lib", ".o")?).map_err(|e| e.to_string())?;
            let elf = Elf::parse(&bytes)?;
            let undefined = elf.undefined();
            for callee in ["core9panicking18panic_bounds_check", "core9panicking9panic_fmt"] {
                if !undefined.iter().any(|u| u.contains(callee)) {
                    return Err(format!("`lib.rs` calls no `{callee}`: {undefined:?}"));
                }
            }
            // That they end in the handler is read in a linked binary: a bounds check's panic, `panic_fmt`, then the
            // handler's own symbol.
            let source = "#![no_std]\n#![no_main]\n#[panic_handler]\nfn on_panic(_: &core::panic::PanicInfo) -> ! {\n    loop {}\n}\n\
                 #[unsafe(no_mangle)]\npub extern \"C\" fn _start() -> ! {\n    let v = [1u8, 2];\n    let i = unsafe { core::ptr::read_volatile(&7usize) };\n    \
                 core::hint::black_box(v[i]);\n    loop {}\n}\n";
            let (dir, linked) = bench.rustc(
                "reaches.rs",
                source,
                &["--crate-type", "bin", "-C", "panic=abort", "-C", "opt-level=2", "-o", "reaches"],
                &[],
            )?;
            expect(&linked, true, "")?;
            let bytes = fs::read(dir.join("reaches")).map_err(|e| e.to_string())?;
            let elf = Elf::parse(&bytes)?;
            let calls = |from: &str, to: &str| -> Result<(), String> {
                let caller = elf.function(from).ok_or_else(|| format!("no `{from}` in the binary"))?;
                let callee = elf.function(to).ok_or_else(|| format!("no `{to}` in the binary"))?;
                let (at, code) = elf.code(caller)?;
                if targets(&instructions(at, code)?).contains(&callee.value) {
                    Ok(())
                } else {
                    Err(format!("`{from}` does not call `{to}`"))
                }
            };
            calls("_start", "core9panicking18panic_bounds_check")?;
            calls("core9panicking18panic_bounds_check", "core9panicking9panic_fmt")?;
            calls("core9panicking9panic_fmt", "17rust_begin_unwind")
        },
    },
    Premise {
        says: "§14.4: `cj.rs` assembles `j` to the two-byte compressed `c.j`",
        check: |bench, probes| {
            let (dir, answer) = bench.rustc(
                "cj.rs",
                &probes["cj.rs"],
                &["--crate-type", "rlib", "-C", "opt-level=2", "-C", "panic=abort", "--emit", "obj"],
                &[],
            )?;
            expect(&answer, true, "")?;
            let bytes = object(&dir, "cj.rs")?;
            let elf = Elf::parse(&bytes)?;
            let symbol = elf.function("5jumps17h").ok_or("no function `jumps`")?;
            let (at, code) = elf.code(symbol)?;
            let body = instructions(at, code)?;
            match body.as_slice() {
                [(_, j), (_, mret)]
                    if j.len == 2
                        && j.kind == (crate::elf::Kind::CompressedJump { imm: 2 })
                        && mret.kind == crate::elf::Kind::Mret =>
                {
                    Ok(())
                }
                _ => Err(format!("`jumps` assembled to {body:?}")),
            }
        },
    },
];

/// Run every premise; exit 0 when each holds.
pub fn run(root: &Path) -> i32 {
    let text = match fs::read_to_string(root.join(RECORD)) {
        Ok(t) => t,
        Err(e) => {
            eprintln!("pin-premises: {RECORD}: {e}");
            return 2;
        }
    };
    let probes = match recorded(&text) {
        Ok(p) => p,
        Err(e) => {
            eprintln!("pin-premises: {RECORD}: {e}");
            return 2;
        }
    };
    let dir = root.join("target/pin-premises");
    let _ = fs::remove_dir_all(&dir);
    if let Err(e) = fs::create_dir_all(&dir) {
        eprintln!("pin-premises: {}: {e}", dir.display());
        return 2;
    }
    let bench = Bench { dir };
    let channel = match pinned_channel(root) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("pin-premises: {e}");
            return 2;
        }
    };
    let version = bench.run("rustc", &["-V"], &[], &bench.dir);
    if !version.text.starts_with(&format!("rustc {channel} ")) {
        eprintln!(
            "pin-premises: the compiler is `{}`, not the pinned {channel}",
            version.text.trim()
        );
        return 2;
    }
    println!(
        "pin-premises: {} probes read at their hashes; {}",
        probes.len(),
        version.text.trim()
    );
    let mut broken = 0;
    for premise in PREMISES {
        match (premise.check)(&bench, &probes) {
            Ok(()) => println!("  holds   {}", premise.says),
            Err(why) => {
                broken += 1;
                println!("  BROKEN  {}\n          {why}", premise.says);
            }
        }
    }
    println!(
        "pin-premises: {} of {} hold",
        PREMISES.len() - broken,
        PREMISES.len()
    );
    i32::from(broken > 0)
}

#[cfg(test)]
mod tests {
    use super::{alignment, operands, recorded, PROBES};

    const RECORD_TEXT: &str =
        include_str!("../../docs/specs/catalog/decision_catalog-records-port.md");

    #[test]
    fn the_record_prints_every_probe_at_its_hash() {
        let probes = recorded(RECORD_TEXT).unwrap_or_else(|e| panic!("{e}"));
        assert_eq!(
            probes.keys().map(String::as_str).collect::<Vec<_>>().len(),
            PROBES.len()
        );
        assert!(probes["two_handlers.rs"].contains("mod other"));
    }

    #[test]
    fn a_probe_changed_without_its_hash_is_refused() {
        let changed = RECORD_TEXT.replacen(
            "fn a(_: &core::panic::PanicInfo) -> ! { loop {} }\n  ```",
            "fn a(_: &core::panic::PanicInfo) -> ! { loop { } }\n  ```",
            1,
        );
        assert_ne!(changed, RECORD_TEXT, "the fixture must change a probe");
        let err = recorded(&changed).unwrap_err();
        assert!(err.contains("hashes to"), "{err}");
        let missing = RECORD_TEXT.replacen("`cj.rs` (`", "`cj.rs` (", 1);
        assert!(recorded(&missing).unwrap_err().contains("`cj.rs`"));
    }

    #[test]
    fn the_alignment_read_is_the_symbols_own() {
        // A naked function's `.p2align` comes before its `.globl`, an ordinary one's after; each before the label.
        let asm = "\t.section\t.text.a\n\t.p2align\t2\n\t.globl\ta\na:\n\tret\n                   \t.section\t.text.b\n\t.globl\tb\n\t.p2align\t1\nb:\n\tret\n                   \t.section\t.text.c\nc:\n\t.p2align\t3\n";
        assert_eq!(alignment(asm, "a"), Some(2));
        assert_eq!(alignment(asm, "b"), Some(1));
        assert_eq!(
            alignment(asm, "c"),
            None,
            "an alignment after the label is not the symbol's"
        );
        assert_eq!(alignment(asm, "d"), None);
    }

    #[test]
    fn the_register_read_is_the_lines_last_operand() {
        let asm = "\tcsrw\tmscratch, a0\n\tcsrw\tmtvec, a1\n";
        assert_eq!(operands(asm, "csrw\tmscratch,"), vec!["a0"]);
        assert_eq!(operands(asm, "csrw\tmtvec,"), vec!["a1"]);
    }
}
