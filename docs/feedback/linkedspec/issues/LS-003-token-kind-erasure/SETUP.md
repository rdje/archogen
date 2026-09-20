# Reproducing LS-003

Self-contained. Everything this issue needs is described here and held in this directory;
nothing outside it is read.

## Environment this was measured on

| Component | Revision |
| --- | --- |
| LinkedSpec | `ad290bdb427bc19a5af81de0f0b07e119c8999ff` |
| RGX | `8763a0e6bea97879f027237439d57725f83ead23` |
| PGEN | `db6f8c6836fefa5a57b1337d3ffbf6f15774089f` |
| rustc | `1.95.0 (59807616e 2026-04-14)` |
| cargo | `1.95.0 (f2d3ce0bd 2026-03-21)` |
| Platform | Darwin arm64 |

LinkedSpec `ad290bdb4` is the commit whose message is
`BACKEND-INTEGRATION-GUIDES.2.2 - deliver Rust Lispish file integration and deployment`.

## 1. Get a LinkedSpec checkout, with nested dependencies

```sh
git clone https://github.com/rdje/linkedspec.git
git -C linkedspec checkout ad290bdb4
git -C linkedspec submodule update --init rgx
git -C linkedspec/rgx submodule update --init subs/pgen
```

## 2. Work around LS-001

`LS-001` (a separate issue in this tracker) prevents the build from completing when
LinkedSpec is vendored inside a Cargo workspace, and also stops the PGEN bootstrap below.
Two manifests need an empty `[workspace]` table. **Skip this step if LinkedSpec has adopted
that fix:**

```sh
printf '\n[workspace]\n' >> linkedspec/examples/integration/rust/Cargo.toml
printf '\n[workspace]\n' >> linkedspec/rgx/subs/pgen/rust/Cargo.toml
```

## 3. Point the build at a local data root

```sh
APP_ROOT=$(pwd -P)
export LINKEDSPEC_PROJECT_DATA_ROOT="$APP_ROOT/.app-data/linkedspec"
export LINKEDSPEC_CACHE_ROOT="$LINKEDSPEC_PROJECT_DATA_ROOT/cache"
export LINKEDSPEC_SCRATCH_ROOT="$LINKEDSPEC_PROJECT_DATA_ROOT/scratch"
export CARGO_HOME="$APP_ROOT/.app-data/cargo-home"
export CARGO_TARGET_DIR="$APP_ROOT/.app-data/target"
```

## 4. Bootstrap PGEN's generated parser sources

Fresh checkouts do not ship them:

```sh
CARGO_TARGET_DIR="$APP_ROOT/linkedspec/rgx/subs/pgen/rust/target" \
CARGO_NET_OFFLINE=false \
bash linkedspec/tools/project_data_run.sh \
  make -C linkedspec/rgx/subs/pgen/rust SHELL=/bin/bash regex_parser_bootstrap
```

Expected tail: `✅ Bootstrap complete. generated/regex_parser.rs is ready…` at `rc=0`.

## 5. Build the Lispish example

```sh
bash linkedspec/tools/run_cargo_local.sh build \
  --manifest-path "$APP_ROOT/linkedspec/examples/integration/rust/Cargo.toml" \
  --bin lispish_file
```

Observed at these revisions: `Finished \`dev\` profile` in 33.94s, producing
`$CARGO_TARGET_DIR/debug/lispish_file`.

## 6. Run the reproducer

```sh
bash repro.sh \
  --bin     "$CARGO_TARGET_DIR/debug/lispish_file" \
  --grammar "$APP_ROOT/linkedspec/specs/Lispish.spec"
```

## Verdict

The exit code is the verdict — no output parsing needed:

| Exit | Meaning |
| --- | --- |
| `0` | The frozen observation still reproduces — the defect is present |
| `3` | Behaviour **changed**; the script prints the difference. This may mean the issue is fixed |
| `2` | Could not run (missing argument or prerequisite) |

Inputs are in [`evidence/`](evidence/), one `.eadl` file per case; the observation frozen at
the revisions above is [`evidence/EXPECTED.txt`](evidence/EXPECTED.txt).
