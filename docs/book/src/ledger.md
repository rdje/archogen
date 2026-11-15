# What this project relies on from outside

## The idea, in plain words

No project is built from nothing. archogen runs its systems on an emulator, builds them with a compiler, makes this
book with a book generator, and reads other people's specifications: of processors, of interrupt controllers, of
other real-time operating systems. Each of these was written by someone else, changes on its own schedule, and has
limits its authors know better than anyone here. This page is the list. For each outside source it says which
version is relied on, what it is relied on for, what it cannot tell you, and what should make someone check it
again — so that a claim about an outside tool lives here, dated, rather than as a timeless sentence somewhere else
that quietly goes stale.

> **In one minute, for engineers.** `ROADMAP.md` §15's dependency ledger: one entry per external source — a tool, a
> vendored checkout, a corpus, a template or a specification — giving its version, where that version is pinned,
> when it was retrieved, its hash where captured, its scope, its known limitations, what triggers revalidation, and
> the names it is cited by. `SOURCE-LEDGER` (`scripts/check_source_ledger.sh`) requires every pin the repository
> holds to belong to an entry with the same version, every chapter or decision that names a source to link its
> entry, and every field filled with absolute dates. It finds a claim by the source's name, so a source never listed
> is invisible to it.

## How it works

Everything below comes from outside this repository: a tool, a checkout, a corpus or a
template. Each entry says which version is relied on and where that version is pinned, when it was
last read, what it is relied on *for*, what it cannot tell you, and what should make someone look
again. `ROADMAP.md` §15 asks for exactly this, and for claims about external tools to live here
"rather than as timeless assertions inside architecture decisions". A decision or a chapter that
names one of these sources links to its entry.

`SOURCE-LEDGER` (`scripts/check_source_ledger.sh`) keeps the entries honest in three ways:

- **Pins.** Every version this repository pins is found by reading the repository: its vendored
  checkouts, the emulator release in `targets/riscv-virt-up.env`, the CI job's tools in
  `.github/ci-tools.env`, `DOCTRINE_VERSION`, `rust-toolchain.toml` and the CI workflows' actions. Each must belong to an entry that carries
  the same version, so a pin cannot move without its entry moving too.
- **Citations.** A chapter or decision record that names a source listed here must link to its
  entry, and every link must lead to one.
- **Entries.** Every field is filled in, every date is absolute, and no version is given as
  "latest".

⚠️ It finds a claim by the source's **name**. A claim about something this page does not list at
all is not seen, and that part stays a matter for review. `ROADMAP.md` §19's prior-art sources,
checked on `2026-09-13`, are design references rather than dependencies. One gets an entry here
the day a chapter or a decision relies on it, as Miri did.

## `qemu`

| Field | Value |
| --- | --- |
| Source | QEMU, the `qemu-system-riscv64` emulator |
| Version | `11.1.1` |
| Pinned at | `env:targets/riscv-virt-up.env:QEMU_VERSION_PINNED` |
| Retrieved | `2026-09-28`; the release's source tarball again, sha256 `079ffbff…2482`, CI's, `2026-11-15` (`M4.1.7.3.4`) |
| Hash | sha256 `03725d89f81f327c7e95dd6129dc7e9a6440d49d7158ade0d558efa9d6bd56f3` of the installed binary, captured `2026-09-29` on this machine. Another machine's build of the same release differs, which is why `scripts/target_emulator.sh --check` compares the version and not the digest. What CI builds from is fixed instead: the release's source tarball, sha256 `079ffbff…2482`, recorded in `.github/ci-tools.env` on `2026-09-30` and checked before the build (`PROGRAM.10.4`) — one derivation, over HTTPS from download.qemu.org; its GPG signature is not checked |
| Scope | independent execution of the `riscv-virt-up` target's binaries, and the device tree it generates: kept as `docs/targets/riscv-virt-up.dtb`, rendered into `docs/targets/riscv-virt-up.dtb.summary.md`, and re-dumped and compared on every `--check` (`M2.8.2`). And, read on `2026-09-30` from the release's source, `hw/intc/sifive_plic.c` at tag `v11.1.1` (file sha256 `ffdafc9a…`), the controller behaviour `decision_runtime-composite-inputs.md` rests on (`M2.10.1`, round 6), and the runtime variant's condition 5 with it (`M2.11`): a read of a context's claim register claims (lines 163–174), and `sifive_plic_irq_request` sets a source's pending bit on every raise of its line, claimed or not, and ignores a lowered one (lines 353–360). And the hart's order among pending interrupts, below (round 7). And, for the target's `REQUIRES_NON_MASKABLE_INTERRUPTS` (`M4.1.7.3.4`), read on `2026-11-15` from the same tarball: a RISC-V hart marks a non-maskable interrupt pending and takes one only under Smrnmi, `target/riscv/tcg/cpu_helper.c` lines 339–348 and 2082 (sha256 `172bd47c…`), and aborts the emulator on an M-mode double trap without it, line 2317; its `riscv.cpu.rnmi` input is made at `target/riscv/tcg/tcg-cpu.c` lines 1682–1683 (sha256 `562e89fd…`), and no file under `hw/` connects it; and a hart offers Smrnmi exactly where its ISA string names it, `target/riscv/cpu.c` line 267 (sha256 `47a2a332…`) |
| Known limitations | a virtual platform: not a board and not a timing reference (§19). It is an independent *implementation*, not independent truth, because it reads the same specifications this project does (`docs/decisions/decision_emulator-independence-retained.md`). Without Smaia, which the target's `rv64` and device tree lack, it takes pending machine interrupts lowest cause first (`target/riscv/cpu.c` lines 880–883, sha256 `47a2a332…`; `target/riscv/tcg/cpu_helper.c` lines 368–373, sha256 `172bd47c…`, both at `v11.1.1`), the timer (7) before external interrupts (11), which is not the privileged specification's order (MEI before MSI before MTI). Its own `virt` machine documentation is not yet held (`REQ-007`, requested) |
| Revalidation trigger | `scripts/target_emulator.sh --check` reporting a mismatch against the pin or the device-tree fixture; any QEMU upgrade; `TARGET_VERIFIED` changing |
| Named as | `QEMU`, `qemu-system-riscv64` |

## `linkedspec`

| Field | Value |
| --- | --- |
| Source | LinkedSpec, vendored at `vendor/linkedspec` |
| Version | commit `2ac834913d85c32f532be9b0aab63644838a577a` |
| Pinned at | `gitlink:vendor/linkedspec` |
| Retrieved | `2026-09-27` |
| Hash | the commit id; no archive digest captured |
| Scope | development-time evaluation of its Rust s-expression reader, and the outbound bug register `docs/feedback/linkedspec/`. No crate of this workspace depends on it |
| Known limitations | its own documented bootstrap moves and dirties the checkouts nested inside it, which `REPOSITORY-BOUNDARY` deliberately does not police. Each report says what was measured at which revision |
| Revalidation trigger | the gitlink moving; a new published head; any report re-measured |
| Named as | `LinkedSpec` |

## `rgx`

| Field | Value |
| --- | --- |
| Source | RGX, a checkout nested inside LinkedSpec |
| Version | commit `f6e5acdc99720349d1e3ecef9f821f365c4db19c`, as LinkedSpec `2ac834913` pins it |
| Pinned at | not pinned here — LinkedSpec's own pin of its nested checkout decides it |
| Retrieved | `2026-09-27` |
| Hash | the commit id |
| Scope | LinkedSpec's published bootstrap builds it; report `LS-004` measures that bootstrap |
| Known limitations | reached only through LinkedSpec's documented build, never directly |
| Revalidation trigger | the LinkedSpec pin moving |
| Named as | `RGX` |

## `pgen`

| Field | Value |
| --- | --- |
| Source | PGEN, the parser generator LinkedSpec's bootstrap uses |
| Version | commit `d9d41c28dca86dd9ec4a6f3668c3a8c71cecf97d`, at which the re-measured reports regenerated their parser |
| Pinned at | not pinned here — reached through LinkedSpec's bootstrap |
| Retrieved | `2026-09-27` |
| Hash | the commit id |
| Scope | the reports' re-measurement environment (`docs/feedback/linkedspec/`) |
| Known limitations | as for RGX |
| Revalidation trigger | the LinkedSpec pin moving |
| Named as | `PGEN` |

## `chipdoc`

| Field | Value |
| --- | --- |
| Source | `chipdoc`, a read-only corpus of processor, microcontroller, board, ISA, interface and protocol documents, entered by its `AGENTS.md`, where a need for any digital chip's document starts |
| Version | `226cfb0`, its revision of `2026-10-31`, as read on `2026-11-05`. ⚠️ **The revision was not recorded** when it was first read, on `2026-09-27`. What was used then is recorded: the RISC-V ISA at `risc-v/isa/pinned/v20260120`; PLIC 1.0.0, ACLINT 1.0-rc4, TL16C550C and Devicetree v0.4 |
| Pinned at | not pinned — kept outside this repository and supplied at run time as `ARCHOGEN_CHIPDOC_ROOT`; never a build dependency |
| Retrieved | `2026-11-05` |
| Hash | not captured — no figure has been adopted from it yet. The first one adopted must cite the digest its `--retrieve` prints |
| Scope | processor, microcontroller, board, ISA, RISC-V, devicetree, interface, protocol and peripheral documents; what it lacks, by operator-relayed request (`REQ-006` and `REQ-007` fulfilled) |
| Known limitations | a document is a source, not a fact (§9). Most of the corpus may be read and quoted but not republished, so a document is never copied into this public repository. Its board PDFs are readable here through the shell's `pdftotext` (`docs/decisions/reference_external-document-source-chipdoc.md`) |
| Revalidation trigger | the next time it is queried, which must record the revision; any figure adopted from it |
| Named as | `chipdoc` |

## `bedrock`

| Field | Value |
| --- | --- |
| Source | the `bedrock` scaffold — the discipline spine's template: doctrine gates, hooks and spine documents |
| Version | `bedrock-scaffold 0.10.0` |
| Pinned at | `file:DOCTRINE_VERSION` |
| Retrieved | `2026-09-30`, read-only, at upstream revision `5af0c1c5b9cc2a65f51ecda1c0937fbc234c84da` |
| Hash | not captured as a digest. The revision above is the identity: `scripts/update_scaffold.sh` is that revision's file plus one recorded hunk, and `VISIBILITY.md` and `DOCTRINE_VERSION` are its bytes (`docs/decisions/decision_scaffold-updater-adopted.md`) |
| Scope | the files named in `scripts/update_scaffold.sh`'s `NEUTRAL` and `SEED_ONCE` arrays, and the outbound register `docs/feedback/bedrock/` |
| Known limitations | seven neutral files carry project content and differ from upstream by design; a sync sets the template's copies aside in `.bedrock-incoming/` and never overwrites. Two scratch sites in a file held for the scaffold write off this volume (`docs/decisions/decision_scratch-on-the-repository-volume.md`) |
| Revalidation trigger | `DOCTRINE_VERSION` changing; an upstream release |
| Named as | `bedrock` |

## `semulith`

| Field | Value |
| --- | --- |
| Source | `semulith`, a sibling repository building CPU and DSP models, which names this project as a consumer |
| Version | commit `cfeea9253ea4a8426373efc6c4375b489b0c8ca6` |
| Pinned at | not pinned — a pointer to a sibling, not a dependency |
| Retrieved | `2026-09-29` |
| Hash | the commit id |
| Scope | where the seam between the two projects is written down (`docs/decisions/reference_sibling-project-semulith.md`) |
| Known limitations | a pointer goes stale when the other repository moves; nothing here builds against it |
| Revalidation trigger | this project's typed eADL interface changing, which that project's integration leaves wait on |
| Named as | `semulith` |

## `rust-toolchain`

| Field | Value |
| --- | --- |
| Source | the Rust toolchain: `rustc`, Cargo, rustup, rustfmt and clippy |
| Version | `1.95.0` — `rustc 1.95.0 (59807616e 2026-04-14)`, `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| Pinned at | `toml:rust-toolchain.toml:channel` — with its components (`rustfmt`, `clippy`) and targets (`riscv64imac-unknown-none-elf`, `wasm32-unknown-unknown`); CI installs exactly that file (`rustup toolchain install`) |
| Retrieved | `2026-09-29`; pinned `2026-09-30` (`PROGRAM.30`); the Cargo Book's `cargo test`, profiles and targets pages of `1.95.0`, `2026-10-23` (`M3.5.2.6.3`); the rustdoc book's documentation-tests page and the Cargo Book's targets page, read from the installed toolchain's documentation `2026-10-23` (`M3.5.2.8.2`); the Cargo Book's build-scripts page, the same way, `2026-10-24` (`M3.5.2.9.3`); its `cargo rustc` page, the same way, `2026-11-14` (`M4.12.1.13`) |
| Hash | binary digests not captured; pinned Cargo source digests are in `docs/specs/trust/flag-normalization-domain.md`; the rustdoc book's documentation-tests page sha256 `88ca4eb4d8a6b39999469843db72360d9de1c9cfeabc4b3e555421abfe358649`, the Cargo Book's targets page `adcc2447522337e6ff28827c6058037749a9af7e260e36d51d3e75a0ed29789c`, its build-scripts page `932885d6d779ceb1db45f86f16aecc16324d615e09cd4d70298092dcfc17e8f1`, its `cargo rustc` page `3adc5733aeb7601e03a59c2395c2f87e9aecccc33542251a6d65b5cf06303d52`, its profiles page `fd9918fbd686a5e0dfa6b030b85d85e7f8f122e21b5102c78e2a60151ae1ab6b`, as shipped |
| Scope | every build and test. Also Cargo's workspace discovery, which the root `Cargo.toml`'s `exclude` relies on: Cargo keeps walking up past a workspace that excludes a package. That was measured on Cargo 1.95.0 by the arms of `LS-001`'s re-measurement. The catalog gate (`decision_catalog-records.md` §3, leaf `M2.7.4`) rests on three claims about it: that `rustc`'s dependency information names every source file the compiler read, a file an assembler directive's `.incbin` or `.include` reads apart (Known limitations), that `rustc --print sysroot` names the toolchain's own files, and that rustup and Cargo discover `rust-toolchain.toml` and `.cargo/config.toml` from the working directory upward and nowhere else but `CARGO_HOME`. Each was measured on the pin `2026-10-03` and is re-measured, with a control, on every integration run by `cargo xtask pin-premises` (`M2.7.4.1`). And how rustc reads a `#!` line, which the generated-sources record's §3 follows: re-measured by `the_shebang_rule_is_rustc_s`. And the trust gate's census (`decision_trust-role-programs.md` §3's table) rests on the pin's Cargo Book: `cargo test`'s target selection and `--all-targets`, the `test` profile inheriting `dev`'s `debug-assertions`, and a target's `test`, `bench`, `harness` and `required-features` fields — each measured on the pin `2026-10-23` (`M3.5.2.6.3`), and since the census builds nothing held by its oracle, which builds its corpus in each of those configurations (`M3.5.2.7.3`) — and on the pin's rustdoc book: indented code blocks, `compile_fail`, and the `doctest` field "only relevant for libraries" (`M3.5.2.8.2`). And the application inputs' `code` digest (`decision_application-inputs.md`, "Which code", `M4.12.1.6`) is of what `rustc`'s dependency information names, read by its shape: a rule for each file made, a line for each file read, one for each variable of the environment, a space of a name after a backslash and nothing else of one escaped. `crates/archogen-plan/tests/code_digest.rs` measures that shape on the pin at every run. The same record quotes the Cargo Book's build-scripts page for where a build script writes and for `cargo::rustc-cfg`, and its `cargo rustc` page for one target compiled where flags are passed, round 13's reason for the digest's compiles building the library alone, which since round 16 is the copy holding no binary's files (`M4.12.1.13`, `M4.12.1.20`), and for those flags reaching the final compile alone, why the image's root is remapped by `--config` (`M4.12.1.16`); and its profiles page for the workspace's root's profile alone read, a dependency's ignored, the image's compile of the copy taking the root's, for `debug-assertions` choosing code, for configuration overriding `Cargo.toml`, and for `package."*"` reaching no workspace member, beside a precedence measured on the pin, a manifest's setting of a package by its name over configuration (`M4.12.1.18`, `M4.12.1.19`); and the `cargo rustc` page again for `test` and `bench` building in a mode of their own. |
| Known limitations | the dependency information does not name a native library reached through `#[link]` or a linker argument, nor code a symbol reaches only at link time, nor a file an `.incbin` or an `.include` directive inside `global_asm!`, `asm!` or `naked_asm!` reads — measured on the pin `2026-10-03`, `.include` `2026-11-10` by `what_an_assembler_directive_reads_is_in_no_account`, what the file holds moving the object, `.incbin`'s bytes and `.include`'s text as assembled, and its name in no `.d` — which is why the catalog and the trust gate refuse all three lexically, a refusal no toolchain bump needs re-measured. `rustfmt --check` reading stdin prints its diff and still exits 0 (measured on the pin `2026-10-03`), so `RUST-FORMAT` compares `--emit stdout` with its input instead. Not the newest release: on `2026-09-30` CI's former `@stable` would have taken a newer compiler than this machine's `stable`. `1.98.0` was measured passing the same `fmt`, `clippy -D warnings` and test suite that day, which is where a bump starts. On macOS Cargo's `unpacked` default keeps every object file built, 1 454 953 by `2026-10-06`: the dev profile sets `off` (`PROGRAM.58`) |
| Revalidation trigger | the channel line changing; a toolchain release worth adopting |
| Named as | `rustc`, `Cargo`, `rustup` |

Producer observations, normalization proofs, counterexamples and revalidation records are in
[Annex C](annex-trust-proofs.md#compiler-records-and-normalization).

## `rust-reference`

| Field | Value |
| --- | --- |
| Source | The Rust Reference, as the pinned toolchain ships it in its documentation: the chapters "Inline assembly" and "Code generation attributes", plus "Loops and other breakable expressions" and "Destructors", "Modules" and "Debugger attributes", "Procedural macros" and "Crates and source files" |
| Version | the Reference shipped with `1.95.0`, the toolchain the `rust-toolchain` entry pins |
| Pinned at | not pinned separately — it ships with that toolchain; no build or check reads it |
| Retrieved | `2026-10-02`, read from the installed `1.95.0` toolchain's documentation directory; loop/drop pages read `2026-10-20`; "Modules" fetched from `doc.rust-lang.org/1.95.0` `2026-10-23` (`M3.5.2.7.1`); "Modules" and "Debugger attributes" read from the installed toolchain's documentation `2026-10-23` (`M3.5.2.8.1`); "Procedural macros" and "Crates and source files" read the same way `2026-11-10` (`M4.12.1.6`) |
| Hash | sha256 `136397d86124651a6c89c2d6bb45175506b4ba04d33e53f352c66251d084796d` of the inline-assembly chapter's page, and `aa9c1a0b0a2d615a690abfbc95819a4cdede12fa70478620ed017c4d2c35e734` of the code-generation attributes chapter's page, as shipped; loop page `9557c8f7721de48768ca20dff48be250fb5c7de369c0a2df730809c19a0f6c4f`, destructor page `6813f53a9061e1dd20f58c87eac237868497bdbe760e29239b359a89e8adea09`, modules page `cf4585c8347c819fe8e8e556feee46ddec1676092037492889249d149a4787b1`, debugger attributes page `cb813d565a23efc4f18ce4ba5916469e68204319454c27948abe0dcf7c212eb4`, procedural macros page `38a54dd5edf76a714275d537dc2eb75cd2942ed48735ac62c55264b9ed25442b`, crates and source files page `d87d90e3ffe053a6f07911643b21a8c2c08fe42b1c64cb8a400cb3df01dc5af6` |
| Scope | what the port's catalog record rests on (`M2.12`), quoted in `decision_catalog-records-port.md` §14.1 and §14.2, the alignment one stated as an absence: what a `sym` operand may name and that its mangled name is substituted; `naked_asm!` and `global_asm!` taking only `sym` and `const` operands; directives outside the listed subset being "assembler-specific"; no attribute setting a function's alignment; that registers not declared as outputs must keep their values, and callee-saved registers theirs across a naked function; that execution must not fall through past a naked function's end; that a `lateout` may share an input's register; that an input narrower than its register leaves the upper bits undefined; and, from the code-generation chapter, that `#[track_caller]` adds an implicit parameter to the function ABI; normal for-body completion and an unlabeled continue target the loop head, and exited nested drop scopes order inner to outer, as used by the [environment-tail proof](annex-trust-proofs.md#environment-loop-tail-control-proof); and the trust census's reading of a module's file, the `path` attribute and a `debugger_visualizer` file (`decision_trust-role-programs.md` §3's table, `M3.5.2.7.1`, `M3.5.2.8.1`), where it is silent the pin's own refusal measured; and, quoted in `decision_application-inputs.md`'s "Which code" (`M4.12.1.6`), that each compilation processes a single crate, so another crate is no source of an application's digest |
| Known limitations | the Reference describes the language; what the compiler emits, such as a naked function's alignment, is measured on the pin rather than guaranteed (`M2.12.1`) |
| Revalidation trigger | the toolchain pin moving |
| Named as | `Rust Reference` |

## `fsmgen`

| Field | Value |
| --- | --- |
| Source | fsmgen, a sibling repository whose live-document size-containment doctrine this project adopted |
| Version | commit `58d5edef7` for the doctrine, adopted `2026-11-13` (`PROGRAM.137`), first `0fa794310`; `727e0d086` for its adoption guide |
| Pinned at | not pinned — the adopted copy is this repository's own `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, whose adoption note records the source revision and the digest of the copied body |
| Retrieved | `2026-09-30`; `2026-11-13` for `58d5edef7` |
| Hash | sha256 `f5fbff0506537e23ca602e0006588ff1e76bfe4b4c056628d527ef8752261e4b` of the copied body (source lines 191–549) |
| Scope | the neutral doctrine body, copied verbatim; nothing else — no threshold, path or conclusion of fsmgen's |
| Known limitations | a template, not an upstream: a later revision there changes nothing here until it is reviewed and adopted by an owned leaf |
| Revalidation trigger | a new fsmgen revision of the doctrine or its adoption guide |
| Named as | `fsmgen` |

## `mdbook`

| Field | Value |
| --- | --- |
| Source | mdBook, which builds this book |
| Version | `v0.5.2`, measured; `0.5.2` pinned for CI |
| Pinned at | `env:.github/ci-tools.env:MDBOOK_VERSION_PINNED` — CI installs that release (`scripts/ci_provision.sh`, `PROGRAM.10.4`), and the `book` step everywhere refuses any other (`scripts/build_book.sh`, `PROGRAM.30`) |
| Retrieved | `2026-09-29`; the release assets `2026-09-30` |
| Hash | sha256 of the release tarballs, recorded in `.github/ci-tools.env` and checked before use: `084e4342…1f6d` (`x86_64-unknown-linux-gnu`), `da2f5565…4222` (`aarch64-apple-darwin`) — each equal to the digest GitHub publishes for the asset |
| Scope | the `book` step of the `integration` tier |
| Known limitations | heading-anchor changes can break `ledger.md#…` links, so the `book` step checks the version. The pinned parser mishandles bare CR and closing-fence tabs; `scripts/build_book.sh` invokes `scripts/check_book_sources.py` to refuse those source forms before publication. See the [guard and upstream-adoption record](annex-trust-proofs.md#publication-guard-for-renderer-control-bytes) |
| Revalidation trigger | `mdbook --version` changing; `MDBOOK_VERSION_PINNED` moving |
| Named as | `mdBook` |

## `github-actions`

| Field | Value |
| --- | --- |
| Source | the actions the CI workflows run |
| Version | `actions/checkout@11d5960a326750d5838078e36cf38b85af677262` (`v4.4.0`); `actions/cache@0057852bfaa89a56745cba8c7296529d2fc39830` (`v4.3.0`) — what the moving `v4` tags resolved to on `2026-09-30` |
| Pinned at | `uses:actions/checkout`, `uses:actions/cache` |
| Retrieved | `2026-09-30`, by `git ls-remote` of each action's tags |
| Hash | the commit ids above: an action is referenced by commit, not by tag (`PROGRAM.30`) |
| Scope | the CI runs in `.github/workflows/`; `actions/cache` keeps the `integration` job's pinned tools between runs, keyed on the files that pin them. The Rust toolchain is no longer an action: each job runs `rustup toolchain install`, which reads `rust-toolchain.toml` |
| Known limitations | a commit does not move, so a newer release of an action arrives only by a deliberate edit. The runner image itself is named (`ubuntu-24.04` for `integration`) but not pinned to a build |
| Revalidation trigger | any workflow edit; a security advisory for either action |
| Named as | `actions/checkout`, `actions/cache` |

## `github-actions-syntax`

| Field | Value |
| --- | --- |
| Source | GitHub's documentation, the metadata syntax reference for actions (`docs.github.com/en/actions/reference/workflows-and-actions/metadata-syntax`) |
| Version | the page as published on `2026-10-02`; the documentation carries no version |
| Pinned at | not pinned — read on `2026-10-02`; no build or check reads it |
| Retrieved | `2026-10-02`, by `curl` of the page |
| Hash | sha256 `94bf89e3feacc7cf4e9ebc24d6f0790784d055f3e5c7acf9e9311f19fff9cfe5` of the page as fetched |
| Scope | how an action's inputs reach it: "The environment variable created converts input names to uppercase letters and replaces spaces with `_` characters", and "We recommend using lowercase input ids" — so two input names of one `with:` that differ only in case meet in one variable, which `WORKFLOW-TOKENS` refuses (`M2.7.6.4`) |
| Known limitations | the page does not say which of two such inputs wins, nor whether the workflow reader accepts a key twice; the gate refuses both rather than rely on either |
| Revalidation trigger | a change to the gate's YAML reader; a change to how GitHub passes inputs |
| Named as | `metadata syntax reference` |

## `mcp-specification`

| Field | Value |
| --- | --- |
| Source | the Model Context Protocol specification: its schemas and its published pages |
| Version | revision `2026-07-28` (the current one) and `2025-11-25` (the last initialization-based one); schemas read at commit `3098fe94caa1b9e0afaaa6d30e040b61d5802471` of `github.com/modelcontextprotocol/specification`, the head of `main` when read |
| Pinned at | not pinned — read on `2026-10-01`; no build or check reads it |
| Retrieved | `2026-10-01`: `schema/2026-07-28/schema.ts` and `schema/2025-11-25/schema.ts` at that commit, and the pages on versioning, the base protocol, `server/discover`, the stdio transport, tools and the `2025-11-25` lifecycle at `modelcontextprotocol.io/specification/`; `2026-10-02`, at the same commit, the `2025-11-25` lifecycle page, both revisions' tools pages and the `2025-11-25` changelog; `2026-10-26`, at the same commit, `schema/2026-07-28/schema.ts` again, for `Result`'s `_meta` and `resultType` when the framing's handling of a tool's result changed (leaf `M3.5.1.6.3`), its sha256 unchanged |
| Hash | sha256 `742750af0bb8c716e7030c4977c992b55d1adc4407e9e66997db5846baedc2cd` of `2026-07-28/schema.ts`, `e74b56e73b2e37bdb595f74ba22e428ad7f07aa3519355ba661d681298ed38ac` of `2025-11-25/schema.ts`; of the pages read `2026-10-02`: lifecycle `45a6e8b7fb8c96e7b9ba1b0a3c727e8451c1e55bf56bb62f3ab63fddc365b919`, `2025-11-25` tools `39e56ad4f3d1ff1cb28ee62283e02947cd97db8aa6190782d629f4562a0f354c`, `2026-07-28` tools `ed550806a58eb7744b858fb9f26001aa5e55dac9e2babe88317cc81d9d4c490d`, changelog `d21083d5ada5706026550ed13ec2ccfb0a8b3b271918698c33d18d2c41d942ae` |
| Scope | the protocol `docs/decisions/decision_mcp-server.md` implements: per-request `_meta` and its required fields, `-32602` and `-32022`, `server/discover`, `resultType`, the stdio framing and shutdown, `tools/list` and `tools/call` with `isError` and `structuredContent`, and the `2025-11-25` `initialize` handshake a dual-era server answers |
| Known limitations | the pages are read as published, and the schema, which the specification calls "the source of truth", is pinned by commit; a revision after `2026-07-28` may change what a server must do |
| Revalidation trigger | a new protocol revision; any change to the server's protocol handling |
| Named as | `MCP` |

## `rp2350`

| Field | Value |
| --- | --- |
| Source | Raspberry Pi's RP2350 datasheet, the microcontroller on the Raspberry Pi Pico 2 |
| Version | the datasheet's own colophon: build-date `2025-07-29`, build-version `d126e9e-clean` |
| Pinned at | not pinned — read on `2026-10-02` at `https://datasheets.raspberrypi.com/rp2350/rp2350-datasheet.pdf`; no build or check reads it |
| Retrieved | `2026-10-02` |
| Hash | sha256 `2877d0f270fb6d6a57943bee58aaad536aa027bea1e5b1c4ce2541a3230d4be8` of the PDF read |
| Scope | two statements, quoted in *Where generated systems run* as an example of the kind of board the first one will be: "Dual Cortex-M33 or Hazard3 processors at 150 MHz", and "RISC-V architecture support is implemented by dynamically swapping the Cortex-M33 (Armv8-M) processors with Hazard3 (RV32IMAC+) processors" |
| Known limitations | an example, not a selection: no board is chosen (`M5`), and nothing here says this one meets the roadmap's criteria for a first board |
| Revalidation trigger | a board being selected; a decision resting on more of the chip |
| Named as | `RP2350` |

## `node`

| Field | Value |
| --- | --- |
| Source | Node.js, the JavaScript runtime in which the `integration` tier's `wasm-binding` and `plan-check-wasm` steps run the browser artifacts |
| Version | `v26.8.1` on the machine that wrote this entry |
| Pinned at | not pinned — the steps use only `WebAssembly`, `TextEncoder`, `TextDecoder`, `crypto.subtle.digest` and `node:fs`, `node:path` and `node:url`, which every maintained release provides; on CI it is the runner image's own Node |
| Retrieved | `2026-09-30`; revalidated `2026-10-27` (leaf `M3.5.7.2`), the configuration checker's loader digesting its module with Web Crypto: `nodejs.org/api/globals.html`, the global `crypto`, "Added in: v17.6.0, v16.15.0", "v19.0.0 No longer behind `--experimental-global-webcrypto` CLI flag", "v23.0.0 No longer experimental"; and `developer.mozilla.org/en-US/docs/Web/API/SubtleCrypto/digest`: "This feature is available only in secure contexts (HTTPS), in some or all supporting browsers", its `data` "An ArrayBuffer, a TypedArray or a DataView", `"SHA-256"` among its algorithms |
| Hash | of Node itself, not captured; of the pages read `2026-10-27`, sha256 `e82363a1be5a4f66df0c7955136b64ca9ed5d43b079e00d7458b0f2d369814fd` (Node's globals) and `4ce4ce7096a93e55dc1ab3fc5eff4004ed774be10119970b790cd06597961de6` (Mozilla's `digest` page) |
| Scope | `scripts/wasm_binding.mjs` and the loader `crates/archogen-wasm/js/archogen.mjs`: reading the artifact's imports and exports with `WebAssembly.Module`, and running it as a page runs it (`API.5.3`); `scripts/plan_check_wasm.mjs` and the loader `crates/archogen-plan-check-wasm/js/plan-check.mjs` likewise for the configuration checker's module, which digests the module's bytes with `crypto.subtle` (`M3.5.7.2`) |
| Known limitations | a browser's engine is not Node's. The loaders use only interfaces both provide, and the page (`API.5.4`) is where a browser runs the generator's; a browser offers `crypto.subtle` only to a secure context, so a page loading the checker's module over plain HTTP cannot digest it, and its loader then fails rather than name another producer. Node before `v19.0.0` needs the flag for the global |
| Revalidation trigger | a loader or a harness using an interface beyond those listed; a runner image without Node, or with one before `v19.0.0` |
| Named as | `Node` |

## `miri`

| Field | Value |
| --- | --- |
| Source | Miri, Rust's undefined-behaviour interpreter, from the `nightly` toolchain |
| Version | `miri 0.1.0 (809936eac6 2026-09-12)` on `rustc 1.100.0-nightly (809936eac 2026-09-12)` |
| Pinned at | not pinned — the `extended` tier's `miri` step runs whatever `nightly` is installed (`PROGRAM.30`) |
| Retrieved | `2026-09-29` |
| Hash | not captured |
| Scope | the `extended` tier's `miri` step (`scripts/extended_miri.sh`): every test target of every workspace crate that finished within 300 s under Miri when measured (29 of 34), after a seeded dangling-pointer read has been refused by the same wiring |
| Known limitations | §19: it checks the executions the tests drive, for the undefined behaviour it can detect; passing does not establish soundness. The product holds no `unsafe` block. The first in the workspace is test code: `crates/archogen-wasm/tests/binding.rs` writes into the binding's buffer through a raw pointer, as a page writes into its memory, and on `2026-09-30` that test passed under this Miri (`API.5.2`). Beyond it, a pass says little more than "the standard library was used soundly on these paths", which is why the step arms itself first |
| Revalidation trigger | a `nightly` update; an `unsafe` block in production code |
| Named as | `Miri` |

## `riscv-privileged`

| Field | Value |
| --- | --- |
| Source | The RISC-V Instruction Set Manual, Volume II, Privileged Architecture: its Machine-Level ISA chapter, and its CSR listing chapter for `mhartid`, read on the RISC-V ratified specifications library |
| Version | Machine-Level ISA, Version 1.13, in the library's `v20260120` release, the release `chipdoc` holds as `risc-v/isa/pinned/v20260120` |
| Pinned at | not pinned — read at `https://docs.riscv.org/reference/isa/v20260120/priv/machine.html`; no build or check reads it |
| Retrieved | `2026-09-30`; its vectored-mode sentence, and the CSR listing chapter, the same release's `priv-csrs` page, `2026-10-02`; §2.1.4, Reset, and §2.1.7.1.3, Priority and Matching Logic, `2026-10-22`; the machine timer's two sentences, `chipdoc`'s `risc-v/isa/pinned/v20260120/priv/machine.html`, `2026-11-07`; its sentence on `mtime`'s frequency, the same file, its digest read again and the same, `2026-11-10`; its sentence on non-maskable interrupts, the same file, and the Smrnmi chapter's on unresumable ones, `priv/rnmi.html`, `2026-11-14` (`M4.1.7.2.13`); the machine timer's sentences on `mtime`'s wrap, its 64-bit precision and RV64's atomic accesses, the same file, its digest read again and the same, `2026-11-15` (`M4.1.7.3.3`) |
| Hash | `chipdoc`'s `priv/machine.html`, which the machine timer's sentences and the non-maskable interrupts' are quoted from: sha256 `b63fe33517fef2dabe53cdbc45b05d656bf808eebf83f0af5a4ab74549617fea`; its `priv/rnmi.html`, which the Smrnmi sentence is quoted from: sha256 `315d3f608046aa418822b1358363bb6361cadfe39a8e29fd4d06b7b1dc21b20a`. Not captured for the rendered page the Scope's other sentences are quoted from; the third sentence's source file is hashed under Known limitations; no figure is adopted |
| Scope | the sentences `decision_runtime-composite-inputs.md` (`M2.10.1`) and the variant's §6 and condition 6 rest on, two here and a third under Known limitations: "Multiple simultaneous interrupts destined for M-mode are handled in the following decreasing priority order: MEI, MSI, MTI, SEI, SSI, STI, LCOFI"; and interrupt-trap conditions "must also be evaluated immediately following the execution of an xRET instruction or an explicit write to a CSR on which these interrupt trap conditions expressly depend (including mip, mie, mstatus, and mideleg)"; and, for the port's catalog record (`M2.12`), the alignment of `mtvec`'s base and where a trap lands from it, from the machine-level chapter, and that `mhartid` is read-only, from the CSR listing chapter, each quoted in `decision_catalog-records-port.md`; and, for `rt-static-up-v1`'s Privilege row (`M3.3.4.2`), §2.1.4's "Upon reset, a hart’s privilege mode is set to M."; and, for the target's `REQUIRES_PRIVILEGE` (`M3.3.4.4`), §2.1.7.1.3's "If no PMP entry matches an M-mode memory operation, the operation succeeds." and "If the L bit is clear and the privilege mode of the access is M, the operation succeeds.", with §2.1.4's "Writable PMP registers' A and L fields are set to 0, unless the platform mandates a different reset value…"; and, for the release alarm's `compare-window` (`M4.1.6.14`), the machine timer's "A machine timer interrupt becomes pending whenever `mtime` contains a value greater than or equal to `mtimecmp`, treating the values as unsigned integers." and, for format `/2`'s `available-in-state` (`M4.1.7.2.2`), "`mtime` must increment at constant frequency, and the platform must provide a mechanism for determining the period of an `mtime` tick."; and "The interrupt remains posted until `mtimecmp` becomes greater than `mtime`", read in `chipdoc`'s copy of this release, whose page holds the timer section the rendering cut off; and, for the target's `REQUIRES_RELEASE_TIMER` (`M4.1.7.3.3`), its counter's width and wrap among it (`M4.1.7.3.5`), "The `mtime` register will wrap around if the count overflows.", "The `mtime` register has a 64-bit precision on all RV32 and RV64 systems." and "For RV64, naturally aligned 64-bit memory accesses to the `mtime` and `mtimecmp` registers are additionally supported and are atomic.", from the same copy |
| Known limitations | a specification says what a conforming hart does. Whether the target's hart and QEMU conform is a platform fact that the catalog states and a review checks. The Advanced Interrupt Architecture, v1.0 §4.1, read the same day, lets `iprio` give a major interrupt "nominally the same priority as a machine-level external interrupt with priority number" n. So the default order holds only without it, which is why each source's `external.<source>` and the target's `one-external-controller` are facts of their own. A third sentence, which the pinned rendering cuts off before its timer section, was read on `2026-09-30` from the manual's source, `src/priv/machine.adoc` at `51c1291` on `main` (line 2566, file sha256 `d5a817c8…`): "If the result of the comparison between `mtime` and `mtimecmp` changes, it is guaranteed to be reflected in MTIP eventually, but not necessarily immediately." It is why a late compare write is charged apart from `L`, and the variant's *taken* in condition 6 (`M2.11`) |
| Revalidation trigger | a newer ratified release of the privileged architecture; a target whose interrupts go through the Advanced Interrupt Architecture or platform-local causes |
| Named as | `privileged specification` |

## `riscv-plic`

| Field | Value |
| --- | --- |
| Source | The RISC-V Platform-Level Interrupt Controller Specification |
| Version | 1.0.0, dated 3/2023, read as `riscv-plic.adoc` at commit `f8ec1b7` of `riscv/riscv-plic-spec`, file sha256 `7209e2d8…` |
| Pinned at | not pinned — read on `2026-09-30`; no build or check reads it |
| Retrieved | `2026-09-30` |
| Hash | the file's sha256 above; no figure is adopted |
| Scope | the sentences `decision_runtime-composite-inputs.md` rests on (`M2.10.1`), and the runtime variant's condition 5 with them (`decision_runtime-analysis-variant.md`, `M2.11`). For `no-empty-claim`: notifications "might take some time to be received at the targets"; a notification holds "an EIP value that was valid at some point in the past"; a claim returns "zero if there is no pending interrupt". For `one-request-per-arrival.<source>`: on a completion, "if the interrupt is level-triggered and the interrupt is still asserted, a new interrupt request will be forwarded to the PLIC core". For the claim rules: a claim is "a non-idempotent memory-mapped I/O control register read", "always legal … even if the EIP is not set"; a handler may "check the local meip/seip/ueip bits before exiting"; "The gateway will only forward additional interrupts … after receiving the completion message" |
| Known limitations | a specification says what a conforming controller does, and says nothing of how long a notification takes. Whether a platform's port waits for the notification to reflect its last claim is the fact a catalog record states and a review checks |
| Revalidation trigger | a newer ratified release of the PLIC, or a target with another external interrupt controller |
| Named as | `PLIC specification` |

## `freertos-kernel`

| Field | Value |
| --- | --- |
| Source | FreeRTOS-Kernel, the kernel source of FreeRTOS: its `tasks.c` |
| Version | `tasks.c` on `main` at commit `8be86d4a24fd4091f8f4192018423ab590f408db`, the branch head when read |
| Pinned at | not pinned — read at `https://raw.githubusercontent.com/FreeRTOS/FreeRTOS-Kernel/main/tasks.c`; no build or check reads it |
| Retrieved | `2026-10-01` |
| Hash | sha256 `6de295f333f31818de546d6eb72b877c5a6964980f14f80bc1e17b19b7ff9fc4` of the file read |
| Scope | one comment, quoted as a precedent for the fault contract's rule 1 (findings §6 (b)): in `xTaskResumeAll`, "If any ticks occurred while the scheduler was suspended then they should be processed now. This ensures the tick count does not slip, and that any delayed tasks are resumed at the correct time." (lines 4141–4144) |
| Known limitations | a precedent from another kernel, not a requirement on this one, and source code rather than a specification: the comment describes what that kernel does, not what a profile must |
| Revalidation trigger | a decision resting on more of FreeRTOS's behaviour than this comment |
| Named as | `FreeRTOS` |

## `osek-os`

| Field | Value |
| --- | --- |
| Source | OSEK/VDX Operating System Specification |
| Version | 2.2.3, read from the copy at `https://www.irisa.fr/alf/downloads/puaut/TPNXT/images/os223.pdf` |
| Pinned at | not pinned — read on `2026-10-01` and again on `2026-10-28`, the same bytes; no build or check reads it |
| Retrieved | `2026-10-01`; `2026-10-28` |
| Hash | sha256 `a0bb10b0b1413b3d95a40de117d3bedd3bbb74e2cae23c074c5532a95fa715cf` of the PDF read, both times |
| Scope | two sentences, quoted as a precedent and a correction for findings §6 (b): `ActivateTask` (§13.2.3.1), "If E_OS_LIMIT is returned the activation is ignored", with the status "Too many task activations of `<TaskID>`, E_OS_LIMIT"; and "The error hook routine (ErrorHook) is called if a system service returns a StatusType value not equal to E_OK"; and, for the runtime plan's multiplexing (`decision_runtime-plan.md` §2.9, round 7), §8.5's "The ceiling priority shall be set at least to the highest priority of all tasks that access a resource or any of the resources linked to this resource", and §8.6's ("OSEK Priority Ceiling Protocol with extensions for interrupt levels") "The ceiling priority shall be set at least to the highest priority of all tasks and interrupt routines that access a resource or any of the resources linked to this resource", a shared resource's users counted as the tasks and interrupt routines that access it (round 8; read whole from the PDF's text `2026-10-28`, round 18's R3) |
| Known limitations | a precedent, not a requirement on this profile; the copy read is a mirror of the consortium's document, not the consortium's own site |
| Revalidation trigger | a decision resting on more of OSEK's behaviour, or a copy from the standard's owner differing from the mirror |
| Named as | `OSEK` |

## `autosar-os`

| Field | Value |
| --- | --- |
| Source | AUTOSAR Classic Platform, Specification of Operating System (`AUTOSAR_CP_SWS_OS`) |
| Version | Release R24-11, read at `https://www.autosar.org/fileadmin/standards/R24-11/CP/AUTOSAR_CP_SWS_OS.pdf` |
| Pinned at | not pinned — read on `2026-10-01`; no build or check reads it |
| Retrieved | `2026-10-01` |
| Hash | sha256 `6ec1915808e8819c6552bcc69fe935d8664f7bf0b8f9eb55810377e79ff8ae44` of the PDF read |
| Scope | three requirements, quoted as a precedent and its qualification for findings §6 (a): [SWS_Os_00239] "If a Task returns from the entry function without making a TerminateTask or ChainTask call and interrupts are still disabled, the Operating System module shall enable them"; [SWS_Os_00069], which reports that return to the `ErrorHook` as `E_OS_MISSINGEND`; and [SWS_Os_00093], under which a service called with interrupts disabled is ignored with `E_OS_DISABLEDINT`. [SWS_Os_00465] is quoted in `docs/decisions/decision_runtime-plan.md`'s alternatives, for an enforced inter-arrival time the runtime plan does not adopt: "The Operating System module shall limit the inter-arrival time of Tasks to one per OsTaskTimeFrame" |
| Known limitations | a precedent, not a requirement on this profile: AUTOSAR treats the return as an error it recovers from, where the fault contract's rule 4 makes the completion ordinary |
| Revalidation trigger | a later AUTOSAR release renumbering or changing these requirements; a decision resting on more of AUTOSAR OS |
| Named as | `AUTOSAR` |

## `rfc-8259`

| Field | Value |
| --- | --- |
| Source | [RFC 8259: The JavaScript Object Notation Data Interchange Format](https://www.rfc-editor.org/rfc/rfc8259.html), primary RFC Editor publication |
| Version | RFC 8259, December 2017 |
| Pinned at | not pinned — fixed document edition named here; no build fetches it |
| Retrieved | `2026-10-11`; §8.1 re-read `2026-10-12` |
| Hash | not captured; the primary publication was read online without a retained byte artifact |
| Scope | §6 number grammar, §7 string/escape grammar, §8.1 interchange encoding, §8.2 Unicode caveats and §9 parser limits; tooling JSON validation repairs in M3.6.9.6.3.2.2.9.6 |
| Known limitations | grammar reference, not a conformance certificate. The tooling stores numbers as text and strings as Rust scalar strings; its existing lone-surrogate refusal is narrower than §8.2's grammar. §9 permits extensions, while this reader's contract is strict |
| Revalidation trigger | parser/representation changes, relevant errata or adoption of a replacement edition |
| Named as | `RFC 8259` |

## `linux-timekeeping`

| Field | Value |
| --- | --- |
| Source | The Linux kernel documentation, *Clock sources, Clock events, sched_clock() and delay timers* |
| Version | the page as served for kernel documentation `7.3.0`, the version its sidebar shows |
| Pinned at | not pinned — read at `https://docs.kernel.org/timers/timekeeping.html`; no build or check reads it |
| Retrieved | `2026-10-27` |
| Hash | sha256 `9553bd8bcc8d9f4b3064496a5aff4005b13dedbf8ca43b1ee553e2bff0f70d4e` of the HTML read |
| Scope | one sentence, quoted as precedent for `deadline-over-delay` in `docs/specs/adapters/decision_engine-adapters.md` §2: clock events "take a desired time specification value and calculate the values to poke into hardware timer registers" |
| Known limitations | another kernel's documentation, describing what it does: a precedent for a mechanism, not a requirement on this engine, and a page that may change between documentation versions |
| Revalidation trigger | a decision resting on more of the kernel's clock-event behaviour than these sentences |
| Named as | `clock event` |

## `linux-sched-clock`

| Field | Value |
| --- | --- |
| Source | The Linux kernel source, `kernel/time/sched_clock.c` |
| Version | tag `v6.12` |
| Pinned at | not pinned — read at `https://raw.githubusercontent.com/torvalds/linux/v6.12/kernel/time/sched_clock.c`; no build or check reads it |
| Retrieved | `2026-10-27` |
| Hash | sha256 `575b92848e1bbe8339887f94befd9758fe330dc03649bfdd94e459143505a6f5` of the file read |
| Scope | the header comment and three mechanisms, quoted as precedent for `epoch-extender` in `docs/specs/adapters/decision_engine-adapters.md` §2: "Generic sched_clock() support, to extend low level hardware time counters to full 64-bit ns values" (lines 3–4); the epoch advanced by `update_sched_clock`, "Atomically update the sched_clock() epoch", from a timer restarted every `wrap_kt`, "Calculate how many nanosecs until we risk wrapping" (lines 136–161, 184–186); and reads under a `seqcount_latch_t` (line 39) |
| Known limitations | source code of another kernel: what it does, not what an adapter here must, and its update interval is computed by that kernel's own helper, whose margin this engine does not inherit |
| Revalidation trigger | an obligation of `epoch-extender` resting on more of this file than these lines |
| Named as | `sched_clock` |

## `nygard-adr`

| Field | Value |
| --- | --- |
| Source | Michael Nygard, "Documenting Architecture Decisions", Cognitect blog, November 15, 2011 |
| Version | the page as published `2011-11-15` |
| Pinned at | not pinned — read at `https://www.cognitect.com/blog/2011/11/15/documenting-architecture-decisions`; no build or check reads it |
| Retrieved | `2026-10-27` |
| Hash | sha256 `01bdb22a561924b5eb072b501b530d14157d4a674561dac25cfff3197eb04faf` of the page read |
| Scope | the practice `docs/decisions/decision_ceilings-bound-reads.md` follows for a superseded record: "If a decision is reversed, we will keep the old one around, but mark it as superseded. (It's still relevant to know that it was the decision, but is no longer the decision.)"; and that the records are kept one file each in the repository, "under doc/arch/adr-NNN.md", numbered, numbers never reused |
| Known limitations | a practitioner's article, not a standard; it says nothing of how large a folder of records may grow, and this project's file names are by subject, not numbered |
| Revalidation trigger | a lifecycle rule for records resting on more of the article than these sentences |
| Named as | `Nygard` |

## `davis-speedup-factors`

| Field | Value |
| --- | --- |
| Source | Robert I. Davis, Alan Burns, Sanjoy Baruah, Thomas Rothvoß, Laurent George, Oliver Gettings, "Exact Comparison of Fixed Priority and EDF Scheduling based on Speedup Factors for both Pre-emptive and Non-pre-emptive Paradigms", *Real-Time Systems*, 2015 |
| Version | the authors' copy, eighteen pages, as served from the University of York |
| Pinned at | not pinned — read at `https://www-users.cs.york.ac.uk/rd17/papers/RTS2015Speedup.pdf`, which since answers `301` to `https://www-users.york.ac.uk/~rd17/papers/RTS2015Speedup.pdf`, the same bytes read again on `2026-10-28`; no build or check reads it |
| Retrieved | `2026-10-27` |
| Hash | sha256 `3f7f38344030aa5a3deb141e8b339ef4381512342835764604628f30029dd952` of the PDF read |
| Scope | its §1.3's account of fixed-priority non-pre-emptive scheduling: "George et al. (1996) showed that unlike in the pre-emptive case, DMPO is not optimal for constrained-deadline task sets scheduled by FP-NP. Further, they showed that the Optimal Priority Assignment algorithm given by Audsley (1991) is applicable". `docs/decisions/decision_runtime-plan.md` §2.5 quotes its first sentence's claim, for why no optimality is claimed for the plan's order among interrupt sources; the glossary's DMPO, EDF and FP-NP cite it |
| Known limitations | a secondary account of George et al. (1996), whose report was not read; it concerns tasks, and the plan applies it to interrupt sources served without nesting, which the variant's condition 5 makes non-pre-emptive among themselves |
| Revalidation trigger | the plan's source order resting on more than the non-optimality of deadline-monotonic ordering for tasks |
| Named as | `Davis` |

## `github-merge-queue`

| Field | Value |
| --- | --- |
| Source | GitHub changelog, "Pull request merge queue is now generally available", `2023-07-12` |
| Version | the post as served on `2026-10-27` |
| Pinned at | not pinned — read at `https://github.blog/changelog/2023-07-12-pull-request-merge-queue-is-now-generally-available/`; no build or check reads it |
| Retrieved | `2026-10-27` |
| Hash | sha256 `651a42b7567f8494b9a30e4aa0d1f301545ef56d2d82dad9cbcaab5a679a565a` of the page read |
| Scope | one sentence, quoted in `docs/decisions/decision_main-protection.md` §1 for why `rdje/archogen`, a repository owned by a personal account, uses "branches up to date" rather than a merge queue: "Merge queue is available on private and public repos on the GitHub Enterprise Cloud plan and all public repos owned by organizations." |
| Known limitations | a 2023 announcement; GitHub's current documentation pages read the same day state no availability either way |
| Revalidation trigger | a merge queue becoming available to repositories owned by personal accounts, or the repository moving to an organization |
| Named as | `merge queue is now generally available` |

## `github-machine-account`

| Field | Value |
| --- | --- |
| Source | GitHub's Terms of Service, section B, "Account Terms" |
| Version | the page as served on `2026-10-27` |
| Pinned at | not pinned — read at `https://docs.github.com/en/site-policy/github-terms/github-terms-of-service`; no build or check reads it |
| Retrieved | `2026-10-27` |
| Hash | sha256 `f8f54975ce02aecbaf5001a0f9ba869229734eb83652c3a37dfe8bf6d338aaa6` of the page read |
| Scope | two sentences, quoted in `docs/decisions/decision_main-protection.md` §2 for the agents' push identity: "A machine account is used exclusively for performing automated tasks." and "You may maintain no more than one free machine account in addition to your free Personal Account." |
| Known limitations | terms GitHub may change; the arrangement rests on the director keeping a single personal account and a single machine account |
| Revalidation trigger | a change to GitHub's account terms, or a second machine account wanted |
| Named as | `GitHub's terms of service` |

## `github-personal-access-tokens`

| Field | Value |
| --- | --- |
| Source | GitHub Docs, "Managing your personal access tokens" |
| Version | the page as served on `2026-10-28` |
| Pinned at | not pinned — read at `https://docs.github.com/en/authentication/keeping-your-account-and-data-secure/managing-your-personal-access-tokens`; no build or check reads it |
| Retrieved | `2026-10-28` |
| Hash | sha256 `519aa3e0846f1ae2582e612ca78e01b3fb64efeca7d0f273ddfe6b1bcae98f4c` of the page read |
| Scope | two passages, quoted in `docs/decisions/decision_main-protection.md` §2 for why the agents' token is a classic one: "Only personal access tokens (classic) have write access for public repositories that are not owned by you or an organization that you are not a member of", and fine-grained tokens' listed gap, "to contribute to repositories where the user is an outside or repository collaborator" |
| Known limitations | a gap GitHub lists as current, which a later release may close; the token's single scope, `public_repo`, is the agents' choice, not the page's |
| Revalidation trigger | fine-grained tokens gaining collaborator access, or the repository becoming private |
| Named as | `Managing your personal access tokens` |

## `github-available-rules`

| Field | Value |
| --- | --- |
| Source | GitHub Docs, "Available rules for rulesets" |
| Version | the page as served on `2026-10-28` |
| Pinned at | not pinned — read at `https://docs.github.com/en/repositories/configuring-branches-and-merges-in-your-repository/managing-rulesets/available-rules-for-rulesets`; no build or check reads it |
| Retrieved | `2026-10-28` |
| Hash | sha256 `123f74f6f83d8828089e9b0492fb502a978647245e96518a5b7d4d01c75426fa` of the page read |
| Scope | two sentences, quoted in `docs/decisions/decision_main-protection.md` §1 for a key of `main`'s ruleset that GitHub filled and the agents did not send: the additional approval is "enabled by default, for both new and existing rulesets", and "When Copilot opens a pull request that isn't attributed to a person, the ruleset requires one more approval than the number you configured." |
| Known limitations | the page marks the feature "in public preview and subject to change"; the key's API name, `require_extra_approval_for_unattributed_changes`, is the read-back's, not the page's |
| Revalidation trigger | the key changing meaning or default, or a Copilot agent opening pull requests here |
| Named as | `Available rules for rulesets` |

## `github-approving-reviews`

| Field | Value |
| --- | --- |
| Source | GitHub Docs, "Approving a pull request with required reviews" |
| Version | the page as served on `2026-10-28` |
| Pinned at | not pinned — read at `https://docs.github.com/en/pull-requests/collaborating-with-pull-requests/reviewing-changes-in-pull-requests/approving-a-pull-request-with-required-reviews`; no build or check reads it |
| Retrieved | `2026-10-28` |
| Hash | sha256 `4049bd95a5e05385e5bbfaacacf26794b2b448f1f40107a744db024d494628d5` of the page read |
| Scope | one sentence, quoted in `docs/decisions/decision_main-protection.md` §4 for why a pull request the director opens cannot merge when every path is the director's: "Pull request authors cannot approve their own pull requests." |
| Known limitations | the page states the rule without naming the author an account or a commit's author; the record reads it as the account that opened the pull request |
| Revalidation trigger | GitHub letting an author approve their own pull request, or the director wanting to merge their own |
| Named as | `Approving a pull request with required reviews` |

## `github-code-owners`

| Field | Value |
| --- | --- |
| Source | GitHub Docs, "About code owners" |
| Version | the page as served on `2026-10-28` |
| Pinned at | not pinned — read at `https://docs.github.com/en/repositories/managing-your-repositorys-settings-and-features/customizing-your-repository/about-code-owners`; no build or check reads it |
| Retrieved | `2026-10-28` |
| Hash | sha256 `061a75d9570661cb4dd145f98764bc1df4ed0812fd8b964b0455fb895630aefa` of the page read |
| Scope | one sentence, quoted in `docs/decisions/decision_main-protection.md` §4 for why the ownership file binds only once it is on `main`: "For code owners to receive review requests, the `CODEOWNERS` file must be on the base branch of the pull request." |
| Known limitations | the sentence speaks of review requests; the record reads the code-owner review rule as reading the same file |
| Revalidation trigger | the hosting reading the ownership file from anywhere but the pull request's base |
| Named as | `About code owners` |

## `github-commit-signing`

| Field | Value |
| --- | --- |
| Source | GitHub Docs, "About commit signature verification" |
| Version | the page as served on `2026-10-28` |
| Pinned at | not pinned — read at `https://docs.github.com/en/authentication/managing-commit-signature-verification/about-commit-signature-verification`; no build or check reads it |
| Retrieved | `2026-10-28` |
| Hash | sha256 `ceb91d84ea8747f8a7dfb159817eee3cf8abdf808546d6f2f201cd00881d0ab0` of the page read |
| Scope | one sentence, quoted in `docs/decisions/decision_main-protection.md` §3 for why the director merges a batch in the web interface rather than by auto-merge: "GitHub will automatically use GPG to sign commits you make using the web interface." |
| Known limitations | the page says nothing of a merge made by auto-merge or through the API; the record takes no position on those, and uses the route the page names |
| Revalidation trigger | the page documenting signatures on merges made otherwise, or the named commit's check finding a merge unsigned |
| Named as | `About commit signature verification` |

## `davis-burns-priority-assignment`

| Field | Value |
| --- | --- |
| Source | Robert I. Davis, Alan Burns, "Priority Assignment for Global Fixed Priority Pre-emptive Scheduling in Multiprocessor Real-Time Systems", University of York, 2009 |
| Version | the authors' report, fifteen pages, as served from the University of York |
| Pinned at | not pinned — read at `https://cs.york.ac.uk/rts/static/papers/R:Davis:2009a.pdf`, which answers `301` to `https://www.cs.york.ac.uk/rts/static/papers/R:Davis:2009a.pdf`; no build or check reads it |
| Retrieved | `2026-10-28` |
| Hash | sha256 `b3c6f0c54340284f0cb38c73287557153a5d0afbe34f1cd660b08a346e29d4a2` of the PDF read |
| Scope | one sentence of its §4.1, quoted in `docs/decisions/decision_runtime-plan.md` §2.5 for why no optimal order is claimed among the plan's interrupt sources: "For OPA-incompatible schedulability tests, optimal priority assignment remains an open problem". Its conditions are quoted from their general statement, the ledger's `davis-2016-priority-assignment` |
| Known limitations | stated for global fixed-priority tests on several processors; the record applies the condition as the general requirement of Audsley's algorithm that the report proves it is, to the composition's bound over interrupt sources served without nesting |
| Revalidation trigger | the composition's bound changing so that a source's bound no longer depends on the order of the sources ahead of it |
| Named as | `Davis and Burns` |

## `davis-2016-priority-assignment`

| Field | Value |
| --- | --- |
| Source | Robert I. Davis, Liliana Cucu-Grosjean, Marko Bertogna, Alan Burns, "A Review of Priority Assignment in Real-Time Systems", *Journal of Systems Architecture* 65, 2016 |
| Version | the accepted version, twenty-four pages, as deposited in White Rose Research Online |
| Pinned at | not pinned — read at `https://eprints.whiterose.ac.uk/id/eprint/99190/1/JSA2016Priority.pdf`; no build or check reads it |
| Retrieved | `2026-10-28` |
| Hash | sha256 `08d6b8ba154f9e95a5c397861c2609d9cc88c60ad391121b1730f710d6b3e235` of the PDF read |
| Scope | its §V's "three simple Conditions are both sufficient and necessary for Audsley's algorithm to provide optimal priority assignment with respect to a given schedulability test S" (read again from the PDF, its hash the same, `2026-10-28`, the runtime plan's round 20) and the first of them, stated for any schedulability test, quoted in `docs/decisions/decision_runtime-plan.md` §2.5: "The schedulability of a task k may, according to test S, depend on any independent properties of tasks with priorities higher than k, but not on any properties of those tasks that depend on their relative priority ordering." The same section lists sporadic task sets under non-pre-emptive scheduling among the cases the algorithm applies to, given a test meeting the conditions |
| Known limitations | a review; the conditions are reproduced from Davis and Burns's report, and the record applies them to the composition's bound over interrupt sources served without nesting |
| Revalidation trigger | the composition's bound changing so that a source's bound no longer depends on the order of the sources ahead of it |
| Named as | `A Review of Priority Assignment in Real-Time Systems` |
