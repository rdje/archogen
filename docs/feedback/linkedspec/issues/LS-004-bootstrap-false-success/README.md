# LS-004 — PGEN bootstrap continues past a failed `cargo` and reports a false seed

| Field | Value |
| --- | --- |
| **ID** | `LS-004` |
| **State** | `verified` |
| **Severity** | Moderate |
| **Kind** | Robustness |
| **Component** | `rgx/subs/pgen/rust` Makefile, target `regex_parser_bootstrap` |
| **Affects** | PGEN `db6f8c68` |
| **Fixed in** | RGX `f6e5acdc9`, adopted by LinkedSpec `fd3e328d5`; re-measured by archogen at `2ac834913` |
| **Reproducer** | Part 1 of [`repro.sh`](repro.sh) — **historical**; see the warning below |
| **Re-measurement** | [`remeasure.sh`](remeasure.sh) — RGX's published interface, four arms |
| **Reported by** | archogen, first consumer of the Rust backend |

## Summary

With `LS-001` unfixed, `make … regex_parser_bootstrap`
does not stop at the first `cargo` failure. It ran two failing `cargo` commands, then printed:

```text
/bin/bash: ./target/debug/ast_pipeline: No such file or directory
🌱 generated/ebnf.rs seeded.
```

and only failed at the end with `Error 101`.

## Two distinct problems

1. **It proceeds after a failed prerequisite.** The exit code is eventually correct, but the run
   keeps doing work on a foundation it knows is missing, so the reported error ends up far from
   the real cause. A consumer reads the tail of the log, sees the seeding message, and looks in
   the wrong place.
2. **`🌱 generated/ebnf.rs seeded.` was not true.** After that run, `generated/` existed and was
   **empty**. A success message with nothing behind it is worse than silence — this is the single
   line that most delayed diagnosis here.

## Expected

Fail at the first failed prerequisite, and do not claim a seed that did not happen.

## Proposed fix

Fail fast on a non-zero `cargo` status in the bootstrap recipe, and make the seeding message
conditional on the file existing afterwards.

## Note

This issue is independent of `LS-001`. Fixing `LS-001` hides it, because the `cargo` calls stop
failing; the bootstrap would still swallow a failure from any other cause.

That prediction is what the re-measurement below had to work around: with `LS-001` fixed, the
historical provocation no longer fails, so the failure is provoked instead by an **empty offline
package store** — a different cause, the same code path.

## Reproduce

Everything needed is in this directory, but unlike the other issues this one runs a real
preparation: the arms need a checkout with its nested submodules initialized, and the success arm
needs network access for the first package resolution.

⚠️ **`repro.sh` must not be re-run at this revision.** It provokes the failure through `LS-001`'s
workspace collision — now fixed — and it does so by calling PGEN's internal make target
(`regex_parser_bootstrap`) directly. The vendor's guide now states that "Applications should not
reproduce PGEN's internal generation steps or modify either submodule", so that is no longer a
supported way to reach this code. Its frozen observation is preserved as the record of the original
defect, and [`remeasure.sh`](remeasure.sh) drives the published interface instead.

```sh
bash remeasure.sh self-test                        # 9 classifier arms; no checkout needed
bash remeasure.sh arm failure --store <empty-dir>  # one empty-store offline control
bash remeasure.sh arm success                      # fresh preparation; needs network
bash remeasure.sh arm reuse                        # prepared reuse; expect a no-op
```

| Item | Where |
| --- | --- |
| Re-measurement instrument | [`remeasure.sh`](remeasure.sh) |
| Re-measurement at `2ac834913` | [`evidence/REMEASURED.txt`](evidence/REMEASURED.txt) |
| Reproducer (historical) | [`repro.sh`](repro.sh) |
| Recorded run (historical) | [`evidence/OBSERVED.txt`](evidence/OBSERVED.txt) |
| Environment and pins | [`SETUP.md`](SETUP.md) — in this directory |

**Exit codes are the verdict, and the two instruments differ.** `remeasure.sh`: `0` = the defect is
**gone** for that arm; `1` = **still present**; `2` = could not run, or could not decide. `repro.sh`:
`0` = the recorded observation still reproduces, i.e. the defect is **present**; `3` = behaviour
**changed**; `2` = could not run.

## Re-measurement at `2ac834913` — 2026-09-27 (archogen)

**Verdict: the reported defect is gone**, on all four arms. State moved `fixed-upstream` →
`verified` on archogen's own rerun. Measured through RGX's **published** downstream interface —
`make -C <checkout>/rgx bootstrap`, via the vendor's documented storage wrapper — at LinkedSpec
`2ac834913`, RGX `f6e5acdc9`, PGEN `d9d41c28`.

| Arm | `make` exit | What the run did | Verdict |
| --- | --- | --- | --- |
| failure control 1 — empty offline package store | `2` | one `error:` line, no later named step, no seed claim, `generated/` empty | **gone** |
| failure control 2 — a second, independent empty store | `2` | identical | **gone** |
| fresh preparation | `0` | `✅ Bootstrap complete.`, 12 files, parser digest `50eec63c9ba79b16` → `196db2eefed767ff` | **gone** |
| prepared reuse | `0` | `PGEN parser already generated — nothing to bootstrap.`, digest unchanged | **gone** |

Both symptoms the report named are absent from every failing run: preparation stops at the first
missing prerequisite — `error: no matching package named … found`, then `make[1]: *** Error 101`
and `make: *** Error 2`, with nothing after them — and no completed-tense seed claim is printed, so
`🌱 generated/ebnf.rs seeded.` over an empty directory, the line that most delayed the original
diagnosis, cannot occur. The reuse arm also behaves as the guide says it should: a successful repeat
prints the no-op line rather than the completion banner, and only the exit status distinguishes it
from a failure.

⭐ **Two things the classifier had to get right, which is why it has 9 arms.**

1. **Tense.** `🌱 Seeding generated/ebnf.rs (one-time bootstrap, Rust frontend)...` is printed
   *before* the step that can fail, and is ordinary build logging; `generated/ebnf.rs seeded.`
   claims completion and is the only form that can be false. Counting both would report every
   healthy run as defective.
2. **A seed claim indicts a run only if the run failed.** The first version of this instrument
   printed `symptom 1 — PRESENT` over a clean, successful preparation. Arm 9 pins the correction
   down, and arm 1 is the historical log from `evidence/OBSERVED.txt`, which must still come back
   `STILL PRESENT`.

⚠️ **The checkout arrived carrying a parser from the previous pin.** `generated/` held sources dated
`2026-09-20`, generated at PGEN `db6f8c68`, and RGX's published contract warns that
`make bootstrap` is idempotent **on existence** — so a plain rerun would have reused the old pin's
parser and measured nothing at all. The regeneration step the contract names was followed (remove
`generated/`, and do not trust a `target/` from before the bump); the stale sources are kept under
the application's data root, and the fresh digest differs from them.

⚠️ **A cost finding, tracked separately.** The successful preparation's full log was 753 MB and
4 008 986 lines, 1 151 376 of them `[PGEN][DBG]` progress lines from the generator. The guide tells a
consumer to "preserve its exit status and full log" when this command fails. That volume is a
property of the interface rather than evidence about this defect, so it is reported as its own issue
and not folded into this verdict.

**Not claimed here:** nothing was built on top of the prepared parser by this re-measurement, and no
consumer's results are measured — `LS-002` and `LS-003` own those. One platform (Darwin arm64,
Rust 1.95.0), one published interface. Full output of all four arms, including the log excerpts:
[`evidence/REMEASURED.txt`](evidence/REMEASURED.txt). The original frozen observation,
[`evidence/OBSERVED.txt`](evidence/OBSERVED.txt), is preserved unchanged.

## State values

This issue's **State** field is one of:

| State | Meaning | Who sets it |
| --- | --- | --- |
| `open` | Reported with a reproducer; no upstream response yet | archogen |
| `acknowledged` | LinkedSpec has confirmed the behaviour | LinkedSpec |
| `by-design` | Confirmed intentional; archogen must adapt or route around it | LinkedSpec |
| `fixed-upstream` | Fixed in a named revision, not yet re-measured by archogen | LinkedSpec |
| `verified` | archogen re-ran the reproducer against that revision and it passes | archogen |
| `withdrawn` | archogen raised it in error; the correction is recorded above | archogen |
| `no-action` | Reproduces as described, but no change is requested | archogen |

To respond, edit the **State** field in the table at the top of this file and add a dated line
to the History below.

## Upstream response — 2026-09-27

LinkedSpec `fd3e328d5dd5c80981a1c3b8496a27270291f7b8` adopts the publicly verified RGX
`f6e5acdc99720349d1e3ecef9f821f365c4db19c` remedy. The public `make bootstrap` failure path
now stops at the first missing prerequisite without later dependent steps or false seed-success
text; fresh success and prepared reuse are preserved. Full LinkedSpec canonical acceptance
and exact remote publication are verified. See [the completion notice](UPSTREAM.md) for
evidence, adoption instructions and the remaining ARCHOGEN verification step.

## History

- `2026-09-20` — opened by archogen; observed while diagnosing `LS-001` on `db6f8c68`.

- `2026-09-27` — LinkedSpec: fixed-upstream in published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`; response above names the remedy and adoption contract. ARCHOGEN verification pending.

- `2026-09-27` — archogen: re-measured at `2ac834913` / RGX `f6e5acdc9` / PGEN `d9d41c28` through RGX's **published** `make bootstrap` (`remeasure.sh`, 9 classifier arms). Two independent empty-store offline controls exit `2` at the first missing prerequisite with no later named step and no seed claim; a fresh preparation exits `0` and regenerates the parser (`50eec63c9ba79b16` → `196db2eefed767ff`); prepared reuse exits `0` as the documented no-op. State `fixed-upstream` → **`verified`**. Evidence: `evidence/REMEASURED.txt`. The historical `repro.sh` was deliberately **not** re-run — it provokes through `LS-001` and calls a PGEN-internal target, which the guide now tells consumers not to do. A separate cost finding (a 753 MB log from one successful run) is tracked as its own issue, not folded into this verdict.

- `2026-09-30` — archogen: erratum in `evidence/REMEASURED.txt` §6, which stated the successful run's log as "41 lines" — a transcription error in prose no instrument printed. It had 4 008 986 lines, as this README says; a fresh run at the same revisions, filed as `LS-008`, measured the same. The verdict above is unaffected.
