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

## LS-001 and LS-004 — no build needed

These two reproduce from a checkout alone, in about a minute:

```sh
bash repro/LS-001-workspace-collision.sh /path/to/linkedspec
```

It constructs the layout the upstream guide documents — an application repository that is a
Cargo workspace, with LinkedSpec vendored at `vendor/linkedspec` — then asserts the defect
reproduces on both affected manifests, applies the proposed fix, and asserts both then pass.
Exit `0` means **both** halves behaved as described. `LS-004` is visible in the same run's
output, in the bootstrap's behaviour after the first `cargo` failure.

## LS-002, LS-003, LS-006 and LS-007 — a working build

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

```sh
bash repro/LS-002-003-lispish-probes.sh \
  --bin     "$CARGO_TARGET_DIR/debug/lispish_file" \
  --grammar "$APP_ROOT/vendor/linkedspec/specs/Lispish.spec" \
  --probes  ./evidence/probes
```

Compare against `evidence/probes/EXPECTED.txt`, which is the output frozen at the revisions
above with absolute paths replaced by `<PROBE>`.

**7. The `LS-002` headline** uses a real file rather than a probe:

```sh
"$CARGO_TARGET_DIR/debug/lispish_file" \
  --grammar "$APP_ROOT/vendor/linkedspec/specs/Lispish.spec" \
  evidence/system.eadl
```

`evidence/system.eadl` is a genuine archogen description with four top-level forms, copied into
this directory so the tracker is self-contained.

## The probe corpus

`evidence/probes/` holds twelve `.eadl` inputs, each isolating one construct drawn from real
eADL. They are plain S-expressions; no eADL semantics are needed to read them.

| Probe | Isolates | Issue |
| --- | --- | --- |
| `01-multiple-top-level-forms` | two forms on separate lines | LS-002 |
| `02-hex-literal-underscore` | `0x1000_0000` survival | LS-006 |
| `03-number-unit-tokens` | `(period 10 ms)` token kinds | LS-003 |
| `04-quoted-string` | `(name "ARCHOGEN")` | LS-003 |
| `05-bare-symbol` | `(name ARCHOGEN)` | LS-003 |
| `06-unterminated-form` | `(defsystem heartbeat` | LS-002 (the passing case) |
| `07-trailing-garbage` | valid form then junk | LS-002 |
| `08-comment-no-newline` | `;` comment at end of file | — |
| `09-unbalanced-close` | `(defblock console.uart))` | LS-002 |
| `10-semantically-invalid` | syntactically fine, semantically wrong | — (control) |
| `11-adjacent-fragment-join` | `(a" b"[c]{d})` | LS-007 |
| `12-two-forms-one-line` | two forms on one line | LS-002 |

Probes `08` and `10` are controls: they are expected to behave correctly and are included so a
future run can tell a real regression from a change in the probes themselves.
