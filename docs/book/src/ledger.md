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
| Retrieved | `2026-09-28` |
| Hash | sha256 `03725d89f81f327c7e95dd6129dc7e9a6440d49d7158ade0d558efa9d6bd56f3` of the installed binary, captured `2026-09-29` on this machine. Another machine's build of the same release differs, which is why `scripts/target_emulator.sh --check` compares the version and not the digest. What CI builds from is fixed instead: the release's source tarball, sha256 `079ffbff…2482`, recorded in `.github/ci-tools.env` on `2026-09-30` and checked before the build (`PROGRAM.10.4`) — one derivation, over HTTPS from download.qemu.org; its GPG signature is not checked |
| Scope | independent execution of the `riscv-virt-up` target's binaries, and the device tree it generates: kept as `docs/targets/riscv-virt-up.dtb`, rendered into `docs/targets/riscv-virt-up.dtb.summary.md`, and re-dumped and compared on every `--check` (`M2.8.2`). And, read on `2026-09-30` from the release's source, `hw/intc/sifive_plic.c` at tag `v11.1.1` (file sha256 `ffdafc9a…`), the controller behaviour `decision_runtime-composite-inputs.md` rests on (`M2.10.1`, round 6), and the runtime variant's condition 5 with it (`M2.11`): a read of a context's claim register claims (lines 163–174), and `sifive_plic_irq_request` sets a source's pending bit on every raise of its line, claimed or not, and ignores a lowered one (lines 353–360). And the hart's order among pending interrupts, below (round 7) |
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
| Source | `chipdoc`, a read-only corpus of chip, ISA, interface and protocol documents |
| Version | ⚠️ **the checkout's revision was not recorded** when it was read. What was used is recorded: the RISC-V ISA at `risc-v/isa/pinned/v20260120`; PLIC 1.0.0, ACLINT 1.0-rc4, TL16C550C and Devicetree v0.4 |
| Pinned at | not pinned — kept off this volume and supplied at run time as `ARCHOGEN_CHIPDOC_ROOT`; never a build dependency |
| Retrieved | `2026-09-27` |
| Hash | not captured — no figure has been adopted from it yet. The first one adopted must cite its family's `SHA256SUMS` digest |
| Scope | ISA, RISC-V, devicetree and peripheral specifications, and board documents, by operator-relayed request (`REQ-006` fulfilled, `REQ-007` requested) |
| Known limitations | a document is a source, not a fact (§9). Its board PDFs are readable here through the shell's `pdftotext` (`docs/decisions/reference_external-document-source-chipdoc.md`) |
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
| Scope | the files named in `scripts/update_scaffold.sh`'s `NEUTRAL` and `SEED_ONCE` arrays |
| Known limitations | seven neutral files carry project content and differ from upstream by design; a sync sets the template's copies aside in `.bedrock-incoming/` and never overwrites. Four scratch sites in scaffold-owned files write off this volume (`docs/decisions/decision_scratch-on-the-repository-volume.md`) |
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
| Retrieved | `2026-09-29`; pinned `2026-09-30` (`PROGRAM.30`) |
| Hash | not captured |
| Scope | every build and test. Also Cargo's workspace discovery, which the root `Cargo.toml`'s `exclude` relies on: Cargo keeps walking up past a workspace that excludes a package. That was measured on Cargo 1.95.0 by the arms of `LS-001`'s re-measurement. The catalog gate (`decision_catalog-records.md` §3, leaf `M2.7.4`) rests on three claims about it: that `rustc`'s dependency information names every source file the compiler read, that `rustc --print sysroot` names the toolchain's own files, and that rustup and Cargo discover `rust-toolchain.toml` and `.cargo/config.toml` from the working directory upward and nowhere else but `CARGO_HOME`. None of the three is measured yet: `M2.7.4` measures each before the gate relies on it |
| Known limitations | the dependency information does not name a native library reached through `#[link]` or a linker argument, nor code a symbol reaches only at link time, which is why the catalog refuses both lexically. Not the newest release: on `2026-09-30` CI's former `@stable` would have taken a newer compiler than this machine's `stable`. `1.98.0` was measured passing the same `fmt`, `clippy -D warnings` and test suite that day, which is where a bump starts |
| Revalidation trigger | the channel line changing; a toolchain release worth adopting |
| Named as | `rustc`, `Cargo`, `rustup` |

## `rust-reference`

| Field | Value |
| --- | --- |
| Source | The Rust Reference, as the pinned toolchain ships it in its documentation: the chapters "Inline assembly" and "Code generation attributes" |
| Version | the Reference shipped with `1.95.0`, the toolchain the `rust-toolchain` entry pins |
| Pinned at | not pinned separately — it ships with that toolchain; no build or check reads it |
| Retrieved | `2026-10-02`, read from the installed `1.95.0` toolchain's documentation directory |
| Hash | sha256 `136397d86124651a6c89c2d6bb45175506b4ba04d33e53f352c66251d084796d` of the inline-assembly chapter's page, and `aa9c1a0b0a2d615a690abfbc95819a4cdede12fa70478620ed017c4d2c35e734` of the code-generation attributes chapter's page, as shipped |
| Scope | what the port's catalog record rests on (`M2.12`), each quoted verbatim in `decision_catalog-records-port.md` §14.1 and §14.2: what a `sym` operand may name and that its mangled name is substituted; `naked_asm!` and `global_asm!` taking only `sym` and `const` operands; directives outside the listed subset being "assembler-specific"; no attribute setting a function's alignment; that registers not declared as outputs must keep their values, and callee-saved registers theirs across a naked function; that `noreturn` code must not fall through; that a `lateout` may share an input's register; and that an input narrower than its register leaves the upper bits undefined |
| Known limitations | the Reference describes the language; what the compiler emits, such as a naked function's alignment, is measured on the pin rather than guaranteed (`M2.12.1`) |
| Revalidation trigger | the toolchain pin moving |
| Named as | `Rust Reference` |

## `fsmgen`

| Field | Value |
| --- | --- |
| Source | fsmgen, a sibling repository whose live-document size-containment doctrine this project adopted |
| Version | commit `0fa794310` for the doctrine; `727e0d086` for its adoption guide |
| Pinned at | not pinned — the adopted copy is this repository's own `LIVE_DOCUMENT_SIZE_CONTAINMENT.md`, whose adoption note records the source revision and the digest of the copied body |
| Retrieved | `2026-09-30` |
| Hash | sha256 `af130de4d7bbeef6db532b0cea4ba25a07ce131c6a44511cbe25d8a91c037aa2` of the copied body (source lines 190–529) |
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
| Known limitations | a release that changes how heading anchors are derived would silently break every `ledger.md#…` link on this page — which is why the `book` step checks the version rather than trusting it |
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
| Retrieved | `2026-10-01`: `schema/2026-07-28/schema.ts` and `schema/2025-11-25/schema.ts` at that commit, and the pages on versioning, the base protocol, `server/discover`, the stdio transport, tools and the `2025-11-25` lifecycle at `modelcontextprotocol.io/specification/`; `2026-10-02`, at the same commit, the `2025-11-25` lifecycle page, both revisions' tools pages and the `2025-11-25` changelog |
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
| Source | Node.js, the JavaScript runtime in which the `integration` tier's `wasm-binding` step runs the browser artifact |
| Version | `v26.8.1` on the machine that wrote this entry |
| Pinned at | not pinned — the step uses only `WebAssembly`, `TextEncoder`, `TextDecoder` and `node:fs`, which every maintained release provides; on CI it is the runner image's own Node |
| Retrieved | `2026-09-30` |
| Hash | not captured |
| Scope | `scripts/wasm_binding.mjs` and the loader `crates/archogen-wasm/js/archogen.mjs`: reading the artifact's imports and exports with `WebAssembly.Module`, and running it as a page runs it (`API.5.3`) |
| Known limitations | a browser's engine is not Node's. The loader uses only interfaces both provide, and the page (`API.5.4`) is where a browser runs it |
| Revalidation trigger | the loader or the harness using an interface beyond those four; a runner image without Node |
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
| Source | The RISC-V Instruction Set Manual, Volume II, Privileged Architecture: its Machine-Level ISA chapter, read on the RISC-V ratified specifications library |
| Version | Machine-Level ISA, Version 1.13, in the library's `v20260120` release, the release `chipdoc` holds as `risc-v/isa/pinned/v20260120` |
| Pinned at | not pinned — read at `https://docs.riscv.org/reference/isa/v20260120/priv/machine.html`; no build or check reads it |
| Retrieved | `2026-09-30` |
| Hash | not captured for the rendered page the Scope's sentences are quoted from; the third sentence's source file is hashed under Known limitations; no figure is adopted |
| Scope | the sentences `decision_runtime-composite-inputs.md` (`M2.10.1`) and the variant's §6 and condition 6 rest on, two here and a third under Known limitations: "Multiple simultaneous interrupts destined for M-mode are handled in the following decreasing priority order: MEI, MSI, MTI, SEI, SSI, STI, LCOFI"; and interrupt-trap conditions "must also be evaluated immediately following the execution of an xRET instruction or an explicit write to a CSR on which these interrupt trap conditions expressly depend (including mip, mie, mstatus, and mideleg)"; and, for the port's catalog record (`M2.12.1`), why its trap entry must be aligned: "The value in the `BASE` field must always be aligned on a 4-byte boundary, and the `MODE` setting may impose additional alignment constraints on the value in the `BASE` field"; and, from the same release's CSR listing, that `mhartid` is read-only: "Attempts to write a read-only register raise illegal-instruction exceptions" |
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
| Pinned at | not pinned — read on `2026-10-01`; no build or check reads it |
| Retrieved | `2026-10-01` |
| Hash | sha256 `a0bb10b0b1413b3d95a40de117d3bedd3bbb74e2cae23c074c5532a95fa715cf` of the PDF read |
| Scope | two sentences, quoted as a precedent and a correction for findings §6 (b): `ActivateTask` (§13.2.3.1), "If E_OS_LIMIT is returned the activation is ignored", with the status "Too many task activations of `<TaskID>`, E_OS_LIMIT"; and "The error hook routine (ErrorHook) is called if a system service returns a StatusType value not equal to E_OK" |
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
| Scope | three requirements, quoted as a precedent and its qualification for findings §6 (a): [SWS_Os_00239] "If a Task returns from the entry function without making a TerminateTask or ChainTask call and interrupts are still disabled, the Operating System module shall enable them"; [SWS_Os_00069], which reports that return to the `ErrorHook` as `E_OS_MISSINGEND`; and [SWS_Os_00093], under which a service called with interrupts disabled is ignored with `E_OS_DISABLEDINT` |
| Known limitations | a precedent, not a requirement on this profile: AUTOSAR treats the return as an error it recovers from, where the fault contract's rule 4 makes the completion ordinary |
| Revalidation trigger | a later AUTOSAR release renumbering or changing these requirements; a decision resting on more of AUTOSAR OS |
| Named as | `AUTOSAR` |
