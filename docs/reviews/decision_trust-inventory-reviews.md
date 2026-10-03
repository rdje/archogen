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

**Round 2**, `2026-10-03`, a new context that had not read round 1: it re-measured §1 and every pairwise intersection,
found the copy rule silent on today's tree (no two tracked files under `crates/`, `xtask/` and the kind modules share a
sha256), confirmed each reused catalog rule is real, compiled a three-line `global_asm!` crate with the pinned rustc to
show that a `.incbin` file's bytes reach the rlib and not its `.d`, and read cargo's `compute_metadata` and GitHub's
rules for required checks. There were 18 findings, 12 of them defects, and the verdict was "not acceptable as it
stands". The sharpest was B1: a form proposed in a pull request could never pass that pull request's required check,
so F30 would have blocked for ever. The answer gives each item one of three outcomes — refused, pending, accepted —
so a review is its merge, and makes F30 `not-established` before the named commit, as the catalog's claims are.

| Finding | Defect | Answer |
| --- | --- | --- |
| B1 — a form or roots change could never pass its own pull request's required check: accepted only once on `main` | yes | §5 and §6: three outcomes; pending items are reported on a pull request and fail only the release's assurance step; the protected merge is the review |
| B2 — no bootstrap: the first `trust/roots.eadl` refused for ever or accepted by its author; new packages stalled unannounced | yes | §5: before the named commit everything is pending and F30 `not-established`; the first acceptance is the first protected pull request proposing the files whole; `M3.5` and `M4.2` say their programs are pending classifications |
| B3 — `global_asm!`'s `.incbin` reads a file no dependency information names (measured) | yes | §3: assembler directives refused in a root's closure, as the catalog refuses them |
| B4 — runtime data both roots read at one path not a shared item | yes | §4: a runtime-data file both roots declare, at one path or two |
| B5 — one executable running two roles makes every package shared, losing case 1 in noise | yes | §2: refused, `trust-shared-program`; `M2.21` filed, since `archogen analyze` would do it and its declared owner `M2.6` closed without it |
| B6 — programs are targets: `src/bin/*.rs` in a classified package arrives unrooted | yes | §2: every executable target classified; unclassified is pending |
| B7 — the reference model's root contradicted the executable rule; the differential harness and its adapter outside every inventory | yes | §2: the exception stated; §4: the comparison harness an item the pair shares |
| B8 — `-C metadata` and `cfg` differ by host, so every accepted item fires across machines | yes | §2 and §3: one host, the CI runner's, named by the baseline; elsewhere the gate reports and compares nothing; `-C metadata` and `extra-filename` left out, `env-dep` paths relative |
| B9 — an edition change reaches every shared package untouched | yes | §3: the whole rustc invocation, `--edition` included |
| B10 — case 5's example (adding a non-root package) failed the gate | yes | §6: a package with no executable target is case 5; one with an executable target a pending classification |
| B11 — case 4 bound the inventory to the shipped artifacts, not to the program that produced the results | yes | §6: each result names its producer's sha256; `trust-verify` refuses another producer and any result of a library-rooted role; `M4.7` carries it |
| B12 — `CARGO_HOME` only created, never emptied; build scripts not refused before building | yes | §3: `CARGO_HOME` made anew every run; build scripts and macros refused from `cargo metadata` before any build |
| B13 — the reuses need adapting | no | `M3.6.2`'s goal names each adaptation |
| B14 — the refusal stated as coverage owed | no | confirmed; unchanged |
| B15 — host-only builds leave target-gated code unseen | no | §3's outside list states it |
| B16 — acceptance trusts code ownership of `trust/` from the named commit | no | §5: trusted from the named commit on, `M2.7.6`'s design |
| B17 — a path-filtered required check is left pending | no | §7: CI unfiltered |
| B18 — wording: four roots and one empty role; the step-1 bullet | no | the step-1 bullet marked superseded |
