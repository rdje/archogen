# Reproducing LS-008

Self-contained. Everything this issue needs is described here and held in this directory; nothing
outside it is read.

## Environment the measurement was taken on

| Component | Revision |
| --- | --- |
| LinkedSpec | `2ac834913d85c32f532be9b0aab63644838a577a` |
| RGX | `f6e5acdc99720349d1e3ecef9f821f365c4db19c` |
| PGEN | `d9d41c28dca86dd9ec4a6f3668c3a8c71cecf97d` |
| rustc | `1.95.0 (59807616e 2026-04-14)` |
| cargo | `1.95.0 (f2d3ce0bd 2026-03-21)` |
| Platform | Darwin arm64 |
| Consuming application | a Cargo workspace root carrying `exclude = ["vendor/linkedspec"]` |

## Two ways to reproduce

**Replay a log you already have.** If a preparation run's log was captured, measure it directly, with
the exit code of the run that wrote it:

```sh
bash repro.sh --log <path-to-log> --exit <exit-code>
```

This needs nothing but Bash, `wc` and `grep`, and it takes a second.

**Run the preparation.** This takes several minutes and writes the log being measured, so it needs the
checkout and storage below and room on the volume for the log. At the measured size that is most of a
gigabyte.

## 1. Get a LinkedSpec checkout

```sh
git clone https://github.com/rdje/linkedspec.git vendor/linkedspec
git -C vendor/linkedspec checkout 2ac834913
git -C vendor/linkedspec submodule update --init rgx
git -C vendor/linkedspec/rgx submodule update --init subs/pgen
```

Both nested checkouts are needed. Nothing further down is, so a recursive update is not required.

## 2. Application-local storage

`repro.sh --run` drives the vendor's documented storage wrapper. It derives every path from the
application root and keeps it on the application's volume:

| Variable | Value |
| --- | --- |
| `LINKEDSPEC_PROJECT_DATA_ROOT` | `<app-root>/.app-data/linkedspec` |
| `LINKEDSPEC_CACHE_ROOT` | `<app-root>/.app-data/linkedspec/cache` |
| `LINKEDSPEC_SCRATCH_ROOT` | `<app-root>/.app-data/linkedspec/scratch` |
| `CARGO_HOME` | `<app-root>/.app-data/cargo-home` |
| `CARGO_TARGET_DIR` | `<app-root>/.app-data/target` |

Ignore `.app-data/` in the application repository.

## 3. Run it

From the application root:

```sh
bash <this-directory>/repro.sh --run
```

The checkout defaults to `<app-root>/vendor/linkedspec`; pass `--checkout <dir>` otherwise. The run
performs the published contract's steps for a **fresh** preparation. It removes the checkout's
generated parser and its PGEN `rust/target`, keeping a copy of the generated sources under
`.app-data/ls008/`. Then it runs `make -C <checkout>/rgx bootstrap` through the storage wrapper. Both
removed paths are ignored build products of the checkout. The log is kept under `.app-data/ls008/`
and reported, never deleted.

## The verdict

`repro.sh` exits `0` when a successful run's log is at least 100 MiB (reproduced), `3` when it is
below that (changed), and `2` when there is no log or the run did not succeed. `bash repro.sh
--self-test` checks that logic against synthetic logs.
