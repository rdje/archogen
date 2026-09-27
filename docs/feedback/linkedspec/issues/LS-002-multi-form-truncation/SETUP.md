# Reproducing LS-002

Self-contained. Everything this issue needs is described here and held in this directory;
nothing outside it is read.

## Environment the original observation was measured on

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

## Environment the re-measurement was taken on

| Component | Revision |
| --- | --- |
| LinkedSpec | `2ac834913d85c32f532be9b0aab63644838a577a` — the consumer's adopted pin |
| RGX | `f6e5acdc99720349d1e3ecef9f821f365c4db19c` |
| PGEN | `d9d41c28dca86dd9ec4a6f3668c3a8c71cecf97d`, parser regenerated at this pin |
| rustc | `1.95.0 (59807616e 2026-04-14)` |
| cargo | `1.95.0 (f2d3ce0bd 2026-03-21)` |
| python3 | `3.14.7` — needed to count top-level forms; the instrument refuses without it |
| Platform | Darwin arm64 |

---

# Part A — re-measuring at the current pin (the route this row is verified on)

## A1. Get a checkout and prepare it

```sh
git clone https://github.com/rdje/linkedspec.git
git -C linkedspec checkout 2ac834913
git -C linkedspec submodule update --init rgx
git -C linkedspec/rgx submodule update --init subs/pgen
```

If LinkedSpec is vendored inside a Cargo workspace, that workspace root needs
`exclude = ["vendor/linkedspec"]` before any metadata or build command — see `LS-001`.

Application-local storage, every path derived at runtime and kept on the application's volume:

```sh
APP_ROOT=$(pwd -P)
export LINKEDSPEC_PROJECT_DATA_ROOT="$APP_ROOT/.app-data/linkedspec"
export LINKEDSPEC_CACHE_ROOT="$LINKEDSPEC_PROJECT_DATA_ROOT/cache"
export LINKEDSPEC_SCRATCH_ROOT="$LINKEDSPEC_PROJECT_DATA_ROOT/scratch"
export CARGO_HOME="$APP_ROOT/.app-data/cargo-home"
export CARGO_TARGET_DIR="$APP_ROOT/.app-data/target"      # name it after the pin when updating
```

Then RGX's **published** preparation interface — not PGEN's internal targets:

```sh
CARGO_NET_OFFLINE=false \
bash linkedspec/tools/project_data_run.sh env -u CARGO_TARGET_DIR \
  make -C "$APP_ROOT/linkedspec/rgx" bootstrap
```

⚠️ `make bootstrap` is idempotent on **existence**. When moving to a new pin, remove
`linkedspec/rgx/subs/pgen/generated` first or the run reuses the previous pin's parser and exits `0`
having done nothing. See `LS-004`.

## A2. Build both consumers

```sh
bash linkedspec/tools/run_cargo_local.sh build --bins --offline --locked \
  --manifest-path examples/integration/rust/Cargo.toml
```

Observed at these revisions: `Finished dev profile` in 1m 02s, producing `sexpr_file` and
`lispish_file` under `$CARGO_TARGET_DIR/debug/`.

## A3. Run both instruments

```sh
bash remeasure.sh --sexpr-bin     "$CARGO_TARGET_DIR/debug/sexpr_file" \
                  --sexpr-grammar "$APP_ROOT/linkedspec/specs/SExprDocumentV1.spec" \
                  --lispish-bin   "$CARGO_TARGET_DIR/debug/lispish_file" \
                  --lispish-grammar "$APP_ROOT/linkedspec/specs/Lispish.spec"
bash remeasure.sh --self-test     # 9 evaluator arms; no binaries needed
bash repro.sh --bin "$CARGO_TARGET_DIR/debug/lispish_file" \
              --grammar "$APP_ROOT/linkedspec/specs/Lispish.spec"
```

The `--lispish-*` pair is optional and runs the historical route as the regression guard the
completion notice asks consumers to retain.

---

# Part B — the historical route, as originally measured

Kept because `evidence/EXPECTED.txt` was produced by it and `repro.sh` still replays it. Two of these
steps are **superseded** and are marked so; do not use them on a current checkout.

## B1. Checkout at the measured revision

```sh
git clone https://github.com/rdje/linkedspec.git
git -C linkedspec checkout ad290bdb4
git -C linkedspec submodule update --init rgx
git -C linkedspec/rgx submodule update --init subs/pgen
```

## B2. ⛔ SUPERSEDED — work around LS-001 by patching two manifests

```sh
printf '\n[workspace]\n' >> linkedspec/examples/integration/rust/Cargo.toml
printf '\n[workspace]\n' >> linkedspec/rgx/subs/pgen/rust/Cargo.toml
```

Do not do this on a current checkout. `LS-001` is fixed: the example carries its own workspace
boundary, and the enclosing application's `exclude` entry covers the rest. The vendor's guide now
forbids modifying dependency manifests as a workaround, and a consumer does not edit a vendored tree.

## B3. Storage, as in A1

## B4. ⛔ SUPERSEDED — bootstrap through PGEN's internal make target

```sh
CARGO_TARGET_DIR="$APP_ROOT/linkedspec/rgx/subs/pgen/rust/target" \
CARGO_NET_OFFLINE=false \
bash linkedspec/tools/project_data_run.sh \
  make -C linkedspec/rgx/subs/pgen/rust SHELL=/bin/bash regex_parser_bootstrap
```

Use A1's published RGX interface instead. The guide states that applications should not reproduce
PGEN's internal generation steps. This was also the run in which `LS-004` was observed.

## B5. Build the Lispish example

```sh
bash linkedspec/tools/run_cargo_local.sh build \
  --manifest-path "$APP_ROOT/linkedspec/examples/integration/rust/Cargo.toml" \
  --bin lispish_file
```

Observed at those revisions: `Finished dev profile` in 33.94s.

## B6. Run the reproducer

```sh
bash repro.sh \
  --bin     "$CARGO_TARGET_DIR/debug/lispish_file" \
  --grammar "$APP_ROOT/linkedspec/specs/Lispish.spec"
```

---

## Verdict

⚠️ The two instruments answer different questions, so their exit codes mean different things.

`remeasure.sh` — "is the defect gone on the document route?":

| Exit | Meaning |
| --- | --- |
| `0` | Gone — every probe behaved as archogen's written expectation says |
| `1` | Still present — a form was dropped on a success exit, or unusable input was accepted |
| `2` | Could not run, or could not decide: a binary, grammar or `python3` is missing, or a document expected to be accepted was rejected (a different defect, not scored as either) |

`repro.sh` — "does the frozen historical observation still reproduce?":

| Exit | Meaning |
| --- | --- |
| `0` | It does — the extraction route still truncates, which is now its documented contract |
| `3` | Behaviour **changed**; the script prints the difference |
| `2` | Could not run (missing argument or prerequisite) |

Inputs are in [`evidence/`](evidence/), one `.eadl` file per case. The observation frozen at the
original revisions is [`evidence/EXPECTED.txt`](evidence/EXPECTED.txt); the re-measurement at
`2ac834913`, on both routes, is [`evidence/REMEASURED.txt`](evidence/REMEASURED.txt).
