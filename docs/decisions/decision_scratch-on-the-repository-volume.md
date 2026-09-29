# Scratch lives on this repository's volume — and the change the scaffold still owes

- **Type:** `decision`
- **Date:** `2026-09-29`
- **Status:** `active`
- **Owner / source:** the director's data-locality rule ("never default to `/tmp`, `/private/tmp`, user-home
  caches, or any other off-volume location"); measured and enforced by leaf `PROGRAM.29`
- **External sources:** [LinkedSpec](../book/src/ledger.md#linkedspec) · [bedrock](../book/src/ledger.md#bedrock) · [the Rust toolchain](../book/src/ledger.md#rust-toolchain) — version, scope and limits in the ledger

## The fact / decision

Everything this repository owns makes its temporary files and directories under `$ROOT/target/`, never in the
system temporary directory. A gate writes under `target/doctrine_scratch/<name>.XXXXXX`. A feedback script,
which must stand alone in someone else's checkout, derives its root from its own location, using the enclosing
work tree if there is one and its own directory otherwise, and writes under `<root>/target/feedback_scratch/`.
`SCRATCH-LOCALITY` (`scripts/check_scratch_locality.sh`) enforces it on every commit.

Four sites in two **scaffold-owned** files cannot be fixed here. The change they need is written below for
whoever owns the scaffold. ⛔ It is never sent there by an archogen agent
([[decision_repository-boundary-read-only]]).

## Why

- **Every call was off the volume.** A logging `mktemp` placed first on `PATH` recorded **26** calls in one
  run of the doctrine driver and the self-test runner, from five gates, all under `$TMPDIR`. On macOS that is
  `/var/folders/…/T/`, on the system volume, so the rule is broken even though no path says `/tmp`.
- **Reading the code found fewer sites than there were.** The census that filed the leaf listed 8 call sites in
  6 files under `scripts/`. The gate, reading every tracked script, found **18 in 15**. Ten sit under `docs/`:
  the eight LinkedSpec feedback reproducers and re-measurements, and two probe scripts kept as leaf artifacts.
  A census scoped to the folder where the thing was expected is not a census of the thing.
- **Nothing was left behind:** 0 of 26 paths before the fix and 0 of 58 after still existed once the run
  ended. The breach is where the files are written, not files left over. That is why the priority was medium
  and not high.
- **A fixture's location is part of the fixture.** Moving scratch under `target/` broke two self-tests:
  - `FEEDBACK-SELF-CONTAINED`'s went **vacuous**: 4 of 6 red arms fired, because git ignores all of `target/`
    and the check's file lister asked git.
  - LS-001's re-measurement **false-failed** one arm: 4 of 5, because Cargo walks up past a workspace that
    excludes a package, and above the fixture it now found this repository's root.

  Both are fixed at the cause. The lister reads a `$ROOT/target/` fixture with `find`, and the root
  `Cargo.toml` excludes `target`, which is the remedy the LS-001 report itself documents. LS-001's self-test
  now checks that no workspace encloses its scratch, and says "could not run" (exit 2) instead of failing an
  arm when one does. That was falsified by removing the exclusion again.

## The change the scaffold owes (recorded, not sent)

Read at `bedrock` `5af0c1c` (`bedrock-scaffold 0.10.0`), read-only. This repository runs `0.8.1`.

| File | Here (`0.8.1`) | Upstream (`0.10.0`) |
| --- | --- | --- |
| `scripts/check_task_acceptance.sh` | `:184` `SELF="$(mktemp -d)"` (its self-test) · `:358` `tmp="$(mktemp -d)"` | `:53` `tmp="$(mktemp -d)"` |
| `scripts/check_waiver_routing.sh` | `:73` `added_file="$(mktemp)"` · `:93` `win_file="$(mktemp)"` | the same two lines |
| `scripts/update_scaffold.sh` | fixed here — **project-owned**: not in its own `NEUTRAL` array | `:37` `tmp="$(mktemp -d)"` |

The proposed change uses the same idiom as this repository's own gates:

```bash
mkdir -p "$ROOT/target/doctrine_scratch"
tmp="$(mktemp -d "$ROOT/target/doctrine_scratch/task_acceptance.XXXXXX")"; trap 'rm -rf "$tmp"' EXIT
```

`check_waiver_routing.sh` makes two files inside a loop and has **no `trap`**, so an interrupt between the
`mktemp` and the `rm -f` leaks one. One directory made up front closes both problems:

```bash
mkdir -p "$ROOT/target/doctrine_scratch"
work="$(mktemp -d "$ROOT/target/doctrine_scratch/waiver_routing.XXXXXX")"; trap 'rm -rf "$work"' EXIT
# …
added_file="$work/added.txt"; printf '%s\n' "$added" > "$added_file"
# …
win_file="$work/window.txt"; printf '%s\n' "$win" > "$win_file"
```

Two cautions for the scaffold, which serves more than Rust projects:

- `target/` is Cargo's convention. A neutral spine should derive the scratch root, for example a
  `DOCTRINE_SCRATCH` defaulting to `$ROOT/target/doctrine_scratch`, and make sure it is ignored.
- In a Rust project, a Cargo fixture under `target/` binds to the root workspace unless that workspace
  excludes `target`. That is the second breakage above.

## How to apply

- **A new script** makes scratch with `mktemp [-d] "$ROOT/target/…XXXXXX"`. The gate refuses any other form,
  including `-t`, `--tmpdir`, a relative template and a template on another path.
- **A fixture that models "outside a repository" or "outside a workspace"** checks that precondition before its
  arms, and reports "could not run" where it fails, as LS-001's re-measurement does. Otherwise a location
  change reads as a broken instrument, or worse, as a passing one.
- **On a scaffold sync** (`PROGRAM.26`): upstream's updater still makes a bare `mktemp`. The adopted copy keeps
  this repository's fix, and `SCRATCH-LOCALITY` refuses the adoption otherwise. If a sync brings a fixed
  `check_task_acceptance.sh` or `check_waiver_routing.sh`, the gate's list of scaffold-owned sites shrinks by
  itself.
