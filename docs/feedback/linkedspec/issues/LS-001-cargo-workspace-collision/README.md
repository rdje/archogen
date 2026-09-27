# LS-001 — Documented vendoring layout does not build inside a Cargo workspace

| Field | Value |
| --- | --- |
| **ID** | `LS-001` |
| **State** | `verified` |
| **Severity** | Blocker |
| **Kind** | Build |
| **Component** | `examples/integration/rust`, `rgx/subs/pgen/rust` |
| **Affects** | LinkedSpec `ad290bdb4`, PGEN `db6f8c68` |
| **Fixed in** | LinkedSpec `effe3e7b2`, published in `fd3e328d5`; re-measured by archogen at `2ac834913` |
| **Reproducer** | [`repro.sh`](repro.sh) (self-checking; **historical** — see the warning below) |
| **Re-measurement** | [`remeasure.sh`](remeasure.sh) — the documented route, at any revision |
| **Reported by** | archogen, first consumer of the Rust backend |

## Summary

The upstream guide's setup is `git submodule add … vendor/linkedspec` inside an application
repository. When that application is a Cargo **workspace** — the ordinary shape of a multi-crate
Rust application, and what archogen is — Cargo's upward workspace auto-discovery walks from each
vendored manifest to the consumer's workspace root and binds it there. Every vendored package
that does not declare its own `[workspace]` then refuses to build.

## Observed

```text
error: current package believes it's in a workspace when it's not:
current:   <app>/vendor/linkedspec/examples/integration/rust/Cargo.toml
workspace: <app>/Cargo.toml
```

It is not confined to the example. It also stops **the PGEN bootstrap**, which the guide requires
before any build can succeed:

```text
current:   <app>/vendor/linkedspec/rgx/subs/pgen/rust/Cargo.toml
workspace: <app>/Cargo.toml
```

The bootstrap exits `rc=101`; see `LS-004` for how it
behaves on the way there.

## Scope measured

17 vendored `Cargo.toml` files in this checkout. **Three** declare `[workspace]` —
`rust/Cargo.toml`, `rgx/Cargo.toml`, `rgx/fuzz/Cargo.toml`. The consequence is worth stating
precisely, because it is half good news:

- `rust/Cargo.toml` **is** a workspace root, so `linkedspec-runtime` — the crate a consumer adds
  as a path dependency — is insulated. **The library route is sound.**
- The packages a consumer is told to *build and copy from* are not. So the documented onboarding
  path fails at step one while the underlying product is fine.

## Expected

The documented layout builds.

## Proposed fix

Add an empty `[workspace]` table to every manifest a consumer is expected to build outside
LinkedSpec's own workspaces — at minimum `examples/integration/rust/Cargo.toml` and
`rgx/subs/pgen/rust/Cargo.toml`. It is inert inside LinkedSpec's own CI and makes those packages
self-contained wherever they are vendored.

Documenting `exclude = ["vendor"]` for the consumer's workspace root is a reasonable *addition*,
but it should not be the only remedy: it requires every consumer to hit the failure first.

## Verification

⚠️ **`repro.sh` is the historical instrument, and two things about it matter now.** First, its
Part 2 *patches the vendored manifests* — inside an rsync'd copy under `.repro-work/`, never in the
checkout you pass it, but a patch all the same — and the vendor's guide at the adopted revision
forbids exactly that as a remedy ("do not modify dependency manifests or add vendored crates to the
application's workspace members to work around this error"). Second, its exit contract as shipped is
`0` / `1` / `2`, **not** the `3` an earlier revision of this page documented: at a fixed revision
Part 1 reports `UNEXPECTED PASS` and the script exits `1` with
`RESULT: did not behave as described`. That documentation drift was found during the
re-measurement and corrected here; the script and its frozen output are unchanged.

Use [`remeasure.sh`](remeasure.sh) for a verdict at any revision.

As originally recorded, `repro.sh /path/to/linkedspec` builds the documented layout from any
LinkedSpec checkout, asserts both manifests fail with that exact error, applies the proposed fix,
and asserts both then pass. It exits `0` only if both halves behave as described:

```text
== Part 1: the documented layout, as shipped ==
  REPRODUCED: examples/integration/rust/Cargo.toml -> 'believes it's in a workspace when it's not'
  REPRODUCED: rgx/subs/pgen/rust/Cargo.toml -> 'believes it's in a workspace when it's not'
== Part 2: the proposed fix — an empty [workspace] table in each manifest ==
  FIXED: examples/integration/rust/Cargo.toml
  FIXED: rgx/subs/pgen/rust/Cargo.toml

RESULT: reproduced (2/2) and fixed by the proposal (2/2).
```

After applying the same fix locally, the PGEN bootstrap and the `lispish_file` build both
succeed. Every observation in `LS-002`, `LS-003`, `LS-006` and `LS-007` was taken from that
working build.

## Reproduce

Everything needed is in this directory. No build required.

```sh
bash remeasure.sh /path/to/linkedspec   # is the defect gone at this revision? (writes nothing)
bash remeasure.sh --self-test           # does that verdict discriminate? (5 arms)
bash repro.sh /path/to/linkedspec       # the historical observation; see the warning above
```

| Item | Where |
| --- | --- |
| Re-measurement instrument | [`remeasure.sh`](remeasure.sh) |
| Re-measurement at `2ac834913` | [`evidence/REMEASURED.txt`](evidence/REMEASURED.txt) |
| Reproducer (historical) | [`repro.sh`](repro.sh) |
| Recorded run (historical) | [`evidence/OBSERVED.txt`](evidence/OBSERVED.txt) |
| Environment and pins | [`SETUP.md`](SETUP.md) — in this directory |

**Exit codes are the verdict, and the two instruments differ.** `remeasure.sh`: `0` = the defect is
**gone**; `1` = **still present**; `2` = could not run or not applicable (including a failure that
is not this defect). `repro.sh`: `0` = reproduces as recorded, i.e. the defect is **present**;
`1` = did not behave as described, which at a fixed revision is the expected outcome;
`2` = could not run.

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

The documented workspace route is fixed by LinkedSpec
`effe3e7b2544abf79f7786a7aa54e77b1893880e`, included in published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`.
The integration example has its own workspace boundary; the enclosing application's documented
exclusion and RGX public preparation complete the supported vendoring route. Upstream verified
nine workspace controls and fresh native consumers. Follow the
[workspace guide](https://github.com/rdje/linkedspec/blob/fd3e328d5dd5c80981a1c3b8496a27270291f7b8/docs/linkedspec-book/src/public-api/integration-rust.md#applications-with-a-cargo-workspace).
The original reproducer and proposed dependency patch are historical evidence; upstream
verification used public preparation and workspace behavior without inspecting or patching
dependency implementations. ARCHOGEN's own adoption and verification remain pending.

⚠️ The last line above was true when the notice was recorded and is **superseded** by the
re-measurement below, which is archogen's own.

## Re-measurement at `2ac834913` — 2026-09-27 (archogen)

**Verdict: the reported defect is gone**, carried by **both** halves of the remedy. State moved
`fixed-upstream` → `verified` on archogen's own rerun. Revision measured: LinkedSpec
`2ac834913d85c32f532be9b0aab63644838a577a` with nested RGX `f6e5acdc9`; the consuming application is
this repository, whose workspace root is the layout the report was written against.

Every probe is `cargo metadata --no-deps --offline` — no build, no network, and nothing written into
the checkout. Both halves were measured separately, before and after adopting the consumer's
documented exclusion:

| Manifest the documented route touches | Own `[workspace]` boundary | Before the exclusion | After it |
| --- | --- | --- | --- |
| `examples/integration/rust/Cargo.toml` | **yes** at this revision | resolves, `rc=0` | resolves, `rc=0` |
| `rgx/subs/pgen/rust/Cargo.toml` | no | **COLLIDES**, `rc=101` | resolves, `rc=0` |

`remeasure.sh` → `rc=1` (`STILL PRESENT`) before and `rc=0` (`GONE`) after; `--self-test` →
`5/5 arms passed`, one of which is the reported shape (no boundary, no exclusion) and must come
back `1`, one is a manifest failing for an unrelated reason and must be refused rather than counted,
and one is an application root with no workspace at all, where the collision cannot occur.

**What upstream changed, precisely.** The report asked for an empty `[workspace]` table in both
manifests, "at minimum". At this revision the integration example — the manifest a consumer builds
directly — carries its own boundary, so it is fixed **at the source** and no longer depends on the
consumer. The nested PGEN package does **not** carry one: it is a transitive dependency whose
preparation RGX's public bootstrap owns, and the supported route covers it with the application
root's documented `exclude = ["vendor/linkedspec"]` instead. That is a different remedy from the one
proposed, and it works — archogen adopted the exclusion in its own workspace root and the collision
is gone, with the workspace itself unchanged (`9` members, `0` vendored packages among them).

⚠️ **What a consumer must therefore still do.** The exclusion is load-bearing for the nested
manifest. A workspace consumer who skips it still hits `rc=101` on `rgx/subs/pgen/rust/Cargo.toml`,
with cargo's own message naming both remedies. The guide states the exclusion as a required step
before any metadata or build command, so this reads as a documented requirement rather than a
remaining defect — but it is the part of the report that was answered by documentation instead of
by a manifest change, and it is recorded here so that nobody reads `verified` as "nothing to do".

**Not claimed here:** no build, no bootstrap and no link against the runtime. Manifest resolution
inside the application workspace is what this report was about; the bootstrap is `LS-004`'s subject.
Full output, including both arms and the workspace census:
[`evidence/REMEASURED.txt`](evidence/REMEASURED.txt). The original frozen observation,
[`evidence/OBSERVED.txt`](evidence/OBSERVED.txt), is preserved unchanged.

## History

- `2026-09-20` — opened by archogen; reproduced 2/2 and fix validated 2/2 on `ad290bdb4`.

- `2026-09-27` — LinkedSpec: fixed-upstream in published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`; response above names the remedy and adoption contract. ARCHOGEN verification pending.

- `2026-09-27` — archogen: re-measured at `2ac834913` with `remeasure.sh` (`5/5` self-test arms) — `rc=1` before the consumer's documented exclusion, `rc=0` after. The example manifest is fixed at the source; the nested PGEN manifest is covered by the exclusion, which archogen adopted in its own workspace root. State `fixed-upstream` → **`verified`**. Evidence: `evidence/REMEASURED.txt`. The same pass corrected this page's `repro.sh` exit contract, which documented a `3` the script never returns.
