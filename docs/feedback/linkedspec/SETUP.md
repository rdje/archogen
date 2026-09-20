# Reproducing this tracker's measurements

Everything here was measured on the revisions and toolchain below. Nothing in this directory
needs archogen to run.

## Exact revisions measured

| Component | Revision |
| --- | --- |
| LinkedSpec | `ad290bdb427bc19a5af81de0f0b07e119c8999ff` |
| RGX | `8763a0e6bea97879f027237439d57725f83ead23` |
| PGEN | `db6f8c6836fefa5a57b1337d3ffbf6f15774089f` |

| Tool | Version |
| --- | --- |
| rustc | `1.95.0 (59807616e 2026-04-14)` |
| cargo | `1.95.0 (f2d3ce0bd 2026-03-21)` |
| Platform | Darwin arm64 |

LinkedSpec `ad290bdb4` is the commit whose message is
`BACKEND-INTEGRATION-GUIDES.2.2 - deliver Rust Lispish file integration and deployment`.

## LS-001, LS-004 and LS-005 — no build needed

These three reproduce from a checkout alone, in about a minute each:

```sh
bash issues/LS-001-cargo-workspace-collision/repro.sh /path/to/linkedspec
bash issues/LS-004-bootstrap-false-success/repro.sh  /path/to/linkedspec
bash issues/LS-005-guide-ordering/repro.sh           /path/to/linkedspec
```

It constructs the layout the upstream guide documents — an application repository that is a
Cargo workspace, with LinkedSpec vendored at `vendor/linkedspec` — then asserts the defect
reproduces on both affected manifests, applies the proposed fix, and asserts both then pass.
`LS-001`'s reproducer exits `0` only if **both** halves behaved as described — the defect
reproduces *and* the proposed fix resolves it. `LS-004` provokes the same collision to make a
prerequisite fail, then checks what the bootstrap does next. `LS-005` reads the guide and checks
its section order; it needs no build at all.

## LS-002, LS-003, LS-006 and LS-007 — one shared build

These need the Lispish example binary. The steps below are the upstream guide's own, plus the
`LS-001` workaround without which they do not complete.

**1. Vendor LinkedSpec and its nested dependencies.**

```sh
git submodule add https://github.com/rdje/linkedspec.git vendor/linkedspec
git -C vendor/linkedspec submodule update --init rgx
git -C vendor/linkedspec/rgx submodule update --init subs/pgen
```

**2. Apply the `LS-001` workaround.** Without this, both the bootstrap and the example build
fail — see `issues/LS-001-cargo-workspace-collision.md`. Skip this step only if LinkedSpec has
adopted the proposed fix:

```sh
printf '\n[workspace]\n' >> vendor/linkedspec/examples/integration/rust/Cargo.toml
printf '\n[workspace]\n' >> vendor/linkedspec/rgx/subs/pgen/rust/Cargo.toml
```

**3. Point the build at an application-owned data root**, as the guide requires:

```sh
APP_ROOT=$(pwd -P)
export LINKEDSPEC_PROJECT_DATA_ROOT="$APP_ROOT/.app-data/linkedspec"
export LINKEDSPEC_CACHE_ROOT="$LINKEDSPEC_PROJECT_DATA_ROOT/cache"
export LINKEDSPEC_SCRATCH_ROOT="$LINKEDSPEC_PROJECT_DATA_ROOT/scratch"
export CARGO_HOME="$APP_ROOT/.app-data/cargo-home"
export CARGO_TARGET_DIR="$APP_ROOT/.app-data/target"
```

**4. Bootstrap PGEN's generated parser sources.** Fresh checkouts do not ship them:

```sh
CARGO_TARGET_DIR="$APP_ROOT/vendor/linkedspec/rgx/subs/pgen/rust/target" \
CARGO_NET_OFFLINE=false \
bash vendor/linkedspec/tools/project_data_run.sh \
  make -C vendor/linkedspec/rgx/subs/pgen/rust \
  SHELL=/bin/bash regex_parser_bootstrap
```

Expected tail: `✅ Bootstrap complete. generated/regex_parser.rs is ready…` at `rc=0`.

**5. Build the example:**

```sh
bash vendor/linkedspec/tools/run_cargo_local.sh build \
  --manifest-path "$APP_ROOT/vendor/linkedspec/examples/integration/rust/Cargo.toml" \
  --bin lispish_file
```

Observed here: `Finished \`dev\` profile` in 33.94s, producing
`$CARGO_TARGET_DIR/debug/lispish_file`.

**6. Run the probe corpus:**

Each of the four takes the same two arguments and compares against its **own**
`evidence/EXPECTED.txt`:

```sh
for id in LS-002-multi-form-truncation LS-003-token-kind-erasure \
          LS-006-hex-underscore-withdrawn LS-007-adjacent-fragment-join; do
  bash "issues/$id/repro.sh" \
    --bin     "$CARGO_TARGET_DIR/debug/lispish_file" \
    --grammar "$APP_ROOT/vendor/linkedspec/specs/Lispish.spec"
done
```

Exit `0` means the frozen observation still reproduces; exit `3` means behaviour changed and the
script prints the difference.

**7. The `LS-002` headline** uses a real file rather than a probe. It is included in that
issue's own evidence and run by its `repro.sh`, but can be checked directly:

```sh
"$CARGO_TARGET_DIR/debug/lispish_file" \
  --grammar "$APP_ROOT/vendor/linkedspec/specs/Lispish.spec" \
  issues/LS-002-multi-form-truncation/evidence/system.eadl
```

That file is a genuine archogen description with four top-level forms, vendored into the issue so
the sub-tree is self-contained.

## Where the inputs live

There is no shared corpus: each issue owns the inputs that demonstrate it, under its own
`evidence/`. They are plain S-expressions — no eADL semantics are needed to read them.

| Issue | Inputs | Isolates |
| --- | --- | --- |
| `LS-002` | `01-multiple-top-level-forms`, `07-trailing-garbage`, `09-unbalanced-close`, `12-two-forms-one-line`, `system.eadl` | input after the first form is discarded |
| `LS-002` | `06-unterminated-form` | the case that **is** rejected — the gap is trailing input, not malformed input |
| `LS-002` | `08-comment-no-newline`, `10-semantically-invalid` | controls — expected to behave correctly |
| `LS-003` | `03-number-unit-tokens`, `04-quoted-string`, `05-bare-symbol` | token kind erased from the result |
| `LS-006` | `02-hex-literal-underscore` | withdrawn — the literal survives intact |
| `LS-007` | `11-adjacent-fragment-join` | adjacent fragments concatenate |

The two controls in `LS-002` are deliberate: they are expected to behave correctly, so a future
run can tell a real regression from a change in the inputs themselves.

`LS-001`, `LS-004` and `LS-005` need no inputs — they act on a LinkedSpec checkout directly, and
each keeps the recorded run of its reproducer in `evidence/OBSERVED.txt`.
