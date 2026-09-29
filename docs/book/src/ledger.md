# What this project relies on from outside

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
| Scope | independent execution of the `riscv-virt-up` target's binaries, and the device tree it generates, compared with the platform fixture by `M2.8` |
| Known limitations | a virtual platform: not a board and not a timing reference (§19). It is an independent *implementation*, not independent truth, because it reads the same specifications this project does (`docs/decisions/decision_emulator-independence-retained.md`). Its own `virt` machine documentation is not yet held (`REQ-007`, requested) |
| Revalidation trigger | `scripts/target_emulator.sh --check` reporting a mismatch against the pin; any QEMU upgrade; `TARGET_VERIFIED` changing |
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
| Source | the Rust toolchain: `rustc`, Cargo, rustfmt and clippy |
| Version | channel `stable`; measured `rustc 1.95.0 (59807616e 2026-04-14)` and `cargo 1.95.0 (f2d3ce0bd 2026-03-21)` |
| Pinned at | `toml:rust-toolchain.toml:channel` |
| Retrieved | `2026-09-29` |
| Hash | not captured |
| Scope | every build and test. Also Cargo's workspace discovery, which the root `Cargo.toml`'s `exclude` relies on: Cargo keeps walking up past a workspace that excludes a package. That was measured on Cargo 1.95.0 by the arms of `LS-001`'s re-measurement |
| Known limitations | ⚠️ `stable` is a moving channel, not a version: two machines, or one machine a month apart, build with different compilers (`PROGRAM.30`) |
| Revalidation trigger | `rustc --version` differing from the one measured here; any toolchain release |
| Named as | `rustc`, `Cargo` |

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
| Pinned at | `env:.github/ci-tools.env:MDBOOK_VERSION_PINNED` — in CI only: `scripts/ci_provision.sh` installs that release (`PROGRAM.10.4`), and a developer's `book` step still builds with whatever is installed (`PROGRAM.30`) |
| Retrieved | `2026-09-29`; the release assets `2026-09-30` |
| Hash | sha256 of the release tarballs, recorded in `.github/ci-tools.env` and checked before use: `084e4342…1f6d` (`x86_64-unknown-linux-gnu`), `da2f5565…4222` (`aarch64-apple-darwin`) — each equal to the digest GitHub publishes for the asset |
| Scope | the `book` step of the `integration` tier |
| Known limitations | pinned in CI, not locally. A release that changes how heading anchors are derived would silently break every `ledger.md#…` link on this page |
| Revalidation trigger | `mdbook --version` changing; `MDBOOK_VERSION_PINNED` moving |
| Named as | `mdBook` |

## `github-actions`

| Field | Value |
| --- | --- |
| Source | the actions the CI workflows run |
| Version | `actions/checkout@v4`; `dtolnay/rust-toolchain@stable`; `actions/cache@v4` |
| Pinned at | `uses:actions/checkout`, `uses:dtolnay/rust-toolchain`, `uses:actions/cache` |
| Retrieved | `2026-09-13`; `actions/cache` `2026-09-30` |
| Hash | not captured — all three are tags, not commits |
| Scope | the CI runs in `.github/workflows/`; `actions/cache` keeps the `integration` job's pinned tools between runs, keyed on the files that pin them |
| Known limitations | a tag moves, so a CI run is not reproducible from its commit alone (`PROGRAM.30`) |
| Revalidation trigger | any tag moving; any workflow edit |
| Named as | `actions/checkout`, `dtolnay/rust-toolchain`, `actions/cache` |

## `miri`

| Field | Value |
| --- | --- |
| Source | Miri, Rust's undefined-behaviour interpreter, from the `nightly` toolchain |
| Version | `miri 0.1.0 (809936eac6 2026-09-12)` on `rustc 1.100.0-nightly (809936eac 2026-09-12)` |
| Pinned at | not pinned — the `extended` tier's `miri` step runs whatever `nightly` is installed (`PROGRAM.30`) |
| Retrieved | `2026-09-29` |
| Hash | not captured |
| Scope | the `extended` tier's `miri` step (`scripts/extended_miri.sh`): every test target of every workspace crate that finished within 300 s under Miri when measured (29 of 34), after a seeded dangling-pointer read has been refused by the same wiring |
| Known limitations | §19: it checks the executions the tests drive, for the undefined behaviour it can detect; passing does not establish soundness. This workspace has no `unsafe` code, so today a pass says little beyond "the standard library was used soundly on these paths", which is why the step arms itself first |
| Revalidation trigger | a `nightly` update; the first `unsafe` block in the workspace |
| Named as | `Miri` |
