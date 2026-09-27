# Reproducing LS-004

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
| PGEN | `d9d41c28dca86dd9ec4a6f3668c3a8c71cecf97d` |
| rustc | `1.95.0 (59807616e 2026-04-14)` |
| cargo | `1.95.0 (f2d3ce0bd 2026-03-21)` |
| Platform | Darwin arm64 |
| Consuming application | a Cargo workspace root carrying `exclude = ["vendor/linkedspec"]` |

## 1. Get a LinkedSpec checkout

```sh
git clone https://github.com/rdje/linkedspec.git
git -C linkedspec checkout 2ac834913      # the re-measurement; ad290bdb4 for the original
git -C linkedspec submodule update --init rgx
git -C linkedspec/rgx submodule update --init subs/pgen
```

Both nested checkouts are needed. `git submodule update --init --recursive` also works but pulls
optional repositories this issue does not need.

## 2. Application-local storage

The re-measurement drives the vendor's documented storage wrapper, so the consuming application
exports its own data root first. Every path is derived at runtime from the application root and stays
on the application's volume:

```sh
APP_ROOT=$(pwd -P)
export LINKEDSPEC_PROJECT_DATA_ROOT="$APP_ROOT/.app-data/linkedspec"
export LINKEDSPEC_CACHE_ROOT="$LINKEDSPEC_PROJECT_DATA_ROOT/cache"
export LINKEDSPEC_SCRATCH_ROOT="$LINKEDSPEC_PROJECT_DATA_ROOT/scratch"
export CARGO_HOME="$APP_ROOT/.app-data/cargo-home"
export CARGO_TARGET_DIR="$APP_ROOT/.app-data/target"
```

`remeasure.sh` sets these itself for the child process, from `--app-root` (default: the repository
the script lives in). Ignore `.app-data/` in the application repository.

## 3. Run the instruments

```sh
bash remeasure.sh self-test                        # 9 classifier arms; no checkout, no network
bash remeasure.sh arm failure --store <empty-dir>  # one empty-store offline control
bash remeasure.sh arm success                      # fresh preparation; needs network the first time
bash remeasure.sh arm reuse                        # prepared reuse; expect a no-op
bash repro.sh /path/to/linkedspec                  # HISTORICAL — see the warning in README.md
```

What the arms do, and what they cost:

| Arm | Writes | Needs | Rough cost |
| --- | --- | --- | --- |
| `self-test` | a temp dir it removes | nothing | seconds |
| `arm failure` | removes `generated/` in the checkout; creates the empty store | `make`, `cargo` | under a minute — it fails at the first prerequisite |
| `arm success` | removes `generated/` and the nested `rust/target`, then regenerates both | network for the first package resolution | minutes; the log is hundreds of megabytes |
| `arm reuse` | nothing | a prepared checkout | seconds |

⚠️ `arm failure` and `arm success` **remove build output inside the checkout**: `generated/`, and for
the success arm the nested `rust/target`. That is the vendor's own published regeneration step —
`make bootstrap` is idempotent on *existence*, so after a pin bump the generated parser must be
removed or the run silently reuses the previous pin's. `remeasure.sh` backs the existing `generated/`
up under the application's data root first and verifies the copy by file count. No tracked content of
either submodule is modified.

⚠️ `arm success` needs a network the first time. Afterwards the retained package store allows
`CARGO_NET_OFFLINE=true`.

## Verdict

⚠️ The two instruments answer different questions, so their exit codes mean different things.

`remeasure.sh` — "is the reported defect gone at this revision?":

| Exit | Meaning |
| --- | --- |
| `0` | Gone, for the arm run: a failure control stopped at the first missing prerequisite and claimed nothing, or preparation succeeded and produced the parser |
| `1` | Still present: it continued past a failed prerequisite, or claimed a seed it did not produce |
| `2` | Could not run, or could not decide: a prerequisite is missing, a failure control exited `0`, or a preparation failed for another reason |

`repro.sh` — "does the recorded observation still reproduce?":

| Exit | Meaning |
| --- | --- |
| `0` | The recorded observation still reproduces — the defect is present |
| `3` | Behaviour **changed**; that may mean fixed, or only that the provocation no longer applies |
| `2` | Could not run (missing argument or prerequisite) |

The run recorded at the original revisions is frozen in
[`evidence/OBSERVED.txt`](evidence/OBSERVED.txt). The re-measurement at `2ac834913` — all four arms,
their log excerpts, the log-volume measurement and how the prepared state was preserved across the
controls — is frozen in [`evidence/REMEASURED.txt`](evidence/REMEASURED.txt).
