# LS-004 — PGEN bootstrap continues past a failed `cargo` and reports a false seed

| Field | Value |
| --- | --- |
| **ID** | `LS-004` |
| **State** | `open` |
| **Severity** | Moderate |
| **Kind** | Robustness |
| **Component** | `rgx/subs/pgen/rust` Makefile, target `regex_parser_bootstrap` |
| **Affects** | PGEN `db6f8c68` |
| **Reproducer** | Part 1 of [`repro.sh`](repro.sh) (self-checking) |
| **Reported by** | archogen, first consumer of the Rust backend |

## Summary

With [`LS-001`](../LS-001-cargo-workspace-collision/README.md) unfixed, `make … regex_parser_bootstrap`
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

## Reproduce

Everything needed is in this directory. No build required.

```sh
bash repro.sh /path/to/linkedspec
```

| Item | Where |
| --- | --- |
| Reproducer | [`repro.sh`](repro.sh) |
| Recorded run | [`evidence/OBSERVED.txt`](evidence/OBSERVED.txt) |
| Environment and pins | [`../../SETUP.md`](../../SETUP.md) |

**Exit code is the verdict:** `0` = reproduces as recorded; `3` = behaviour **changed**, which
may mean this issue is fixed; `2` = could not run.

## History

- `2026-09-20` — opened by archogen; observed while diagnosing `LS-001` on `db6f8c68`.
