# The trust-dependency inventory: the independent reviews, and the answer to every finding

- **Type:** `decision`
- **Date:** `2026-10-03`
- **Status:** `active`
- **Owner / source:** leaf `M3.6.1` (`docs/tasks/M3.md`). This is the review history of
  [[decision_trust-inventory]], kept apart from it as `docs/reviews/INDEX.md` describes.

## The fact / decision

`M3.6.1`'s acceptance is a review, by a context that did not write the record, finding that the gate as specified
exercises each of `ROADMAP.md` §14.4's five cases, that no input a root's build reads can be shared without being
reported, and that the baseline cannot be accepted by its author. Each round below is one such review, with every
finding and the answer the record gives it.

Each round is a new context, read-only, which had not written the record. It is given:
- the record;
- `ROADMAP.md` §4.2, §4.4, §10.3 and §14.4;
- `docs/decisions/decision_zero-dependency-engine-core.md` and `decision_findings-for-director-review.md` §11;
- `xtask/src/catalog_build.rs`, which reads cargo's metadata and the compiler's dependency information today;
- the workspace's manifests, and cargo's and rustc's documentation for `cargo metadata` and dependency information.

It is barred from other implementation, and may check external documentation on the web.

## Rounds

**Round 1**, `2026-10-03`: the reviewer re-measured §1 with `cargo metadata` and the dependency information under
`target/`, confirmed every fact, found cases 1, plain-source 2 and 5 working, and found the volume case-insensitive.
There were 20 findings, 11 of them defects, and the verdict was "not acceptable as it stands". Most answers reuse what
the catalog's build checker already does — build from the commit's blobs, a cleared environment, configurations
above the repository refused, build scripts, procedural macros and link inputs refused — so the record takes rules
already reviewed rather than writing new ones. Measured while answering: dependency information names an included
file unnormalised (`crates/archogen-api/src/../../../docs/semantics/kinds/core.eadl`), and the five roots of the
answered §1 still share nothing.

| Finding | Defect | Answer |
| --- | --- | --- |
| A1 — the generator rooted at the `archogen-s0` library, not the `archogen` executable that runs `archogen build`, so the kind modules and `archogen-api` it compiles in were invisible | yes | §2: a root is the executable that runs a role, the library only where none runs it yet; §1: the generator is `archogen`, its reach and the kind modules measured |
| A2 — the reference model's counterpart, `rt-core`, not a root | yes | §2: for each reference model, the implementation it validates is a root; §1 lists `rt-core` |
| A3 — `trust/roots.eadl` the author's to change, and nothing checking a role is filled | yes | §2 and §5: the roots change only under §5's review; every workspace package is classified, `trust-unrooted-package` otherwise |
| A4 — "a review is accepted" not mechanical; `trust/` outside findings §11's code-owned paths | yes | §5: acceptance is the catalog's premise 3 — a hosting-signed merge on `origin/main`'s first-parent chain after the named commit — with the gate a required check; findings §11 amended to add `trust/` and the gate's code |
| A5 — build-script reads, procedural-macro reads and native link inputs outside rustc's dependency information | yes | §3: each refused in a root's closure, `trust-undeclared-input`, as the catalog refuses them, until a leaf designs its coverage |
| A6 — `RUSTFLAGS`, `[env]`, profile overrides, `RUSTC_WRAPPER`, configuration above the repository; a build-configuration file used as data hidden by its exclusion | yes | §3: the catalog's cleared environment and configuration refusals; each unit's `--cfg` and codegen arguments and `env-dep` values in an item's configuration; a configuration file read as data is an ordinary file |
| A7 — builds over the working tree, so an uncommitted revert masks a committed change | yes | §3: built from the commit's blobs, never the working tree |
| A8 — case 4 had no consumer: the gate builds its own inventory | yes | §6: judged at packaging; the package ships the trust build's artifacts, equality by construction, no reproducibility assumed; `trust-verify`; `M4.7`'s acceptance amended |
| A9 — one file under two paths: a symlink, a case alias | yes | §3: names resolved lexically, a symbolic link refused, case twins refused by the written tree |
| A10 — no feature set recorded; features from workspace-wide metadata would fabricate case 5 | yes | §3 and §6: configuration from each root's own units, never from workspace unification |
| A11 — a verbatim copy unreported | yes | §4: a non-empty file whose sha256 appears in both roots under different paths is a shared item; edited copies stated as unseen |
| A12 — whole-file hashing over-approximates | no | §4 declares it |
| A13 — case 5's sentence inexact | no | §6: the gate compares shared items only; the identity is recorded, not compared |
| A14 — `std` and the compiler shared by every root | no | §3: accepted infrastructure by toolchain identity, its bump reviewed where it is ledgered |
| A15 — a baseline form whose item is no longer shared | no | §5: refused until removed |
| A16 — the CI trigger | no | §7: any path in any root's inventory, `trust/` and the gate's code |
| A17 — the exclusions | no | sound once A1 is answered; §2 classifies `xtask` and the wasm module as not roots |
| A18 — `catalog_build::dep_info` drops `env-dep` values and scans `deps/` only | no | `M3.6.2`'s; build scripts are refused, so no `build/` unit |
| A19 — `/1`, a root reaching itself, "no feature" | no | §1 and §2: the root's package included; `--no-default-features` and no `--features` |
| A20 — the fixed runtime-data statement goes false at `M2.7.5` | no | §2 and §3: runtime data a per-root declaration |
