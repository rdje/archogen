# LS-001 — Documented vendoring layout does not build inside a Cargo workspace

| Field | Value |
| --- | --- |
| **ID** | `LS-001` |
| **State** | `fixed-upstream` |
| **Severity** | Blocker |
| **Kind** | Build |
| **Component** | `examples/integration/rust`, `rgx/subs/pgen/rust` |
| **Affects** | LinkedSpec `ad290bdb4`, PGEN `db6f8c68` |
| **Reproducer** | [`repro.sh`](repro.sh) (self-checking) |
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

`repro.sh /path/to/linkedspec` builds the documented layout from any
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
bash repro.sh /path/to/linkedspec
```

| Item | Where |
| --- | --- |
| Reproducer | [`repro.sh`](repro.sh) |
| Recorded run | [`evidence/OBSERVED.txt`](evidence/OBSERVED.txt) |
| Environment and pins | [`SETUP.md`](SETUP.md) — in this directory |

**Exit code is the verdict:** `0` = reproduces as recorded; `3` = behaviour **changed**, which
may mean this issue is fixed; `2` = could not run.

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

## History

- `2026-09-20` — opened by archogen; reproduced 2/2 and fix validated 2/2 on `ad290bdb4`.

- `2026-09-27` — LinkedSpec: fixed-upstream in published `fd3e328d5dd5c80981a1c3b8496a27270291f7b8`; response above names the remedy and adoption contract. ARCHOGEN verification pending.
