#!/usr/bin/env bash
# scripts/ci_rehearse.sh — run the `integration` CI job's commands on this machine, against the repository as the
# runner receives it (leaf `PROGRAM.10.4`).
#
# ⭐ WHY. This repository's CI cannot be watched from here, and a workflow is only evidence once it has run. What
# can be reproduced is everything that is not the runner's operating system: a checkout of one commit, made the
# way `actions/checkout` makes it (`git init`, fetch the commit with its history, `checkout -B main`), so nothing
# untracked, ignored or built is there; no submodule initialised; no global or system git configuration and no
# identity the job did not set, in a home of the job's own; `GITHUB_PATH` and `GITHUB_STEP_SUMMARY` honoured as
# Actions honours them.
#
# USAGE: scripts/ci_rehearse.sh [<commit>]
#   Default: the index and working tree as `git stash create` records them — tracked changes and staged new
#   files, no ref moved — so a change to the job can be rehearsed before it is committed; HEAD when clean.
#
# The pinned tools are taken from this checkout's `target/ci/tools/` (what the job's cache restores), linked into
# the rehearsal; the rehearsal builds everything else from scratch in `../.archogen-ci-rehearsal/`.
#
# ⭐ WHERE, and why not under `target/`. A checkout inside this repository has this repository's
# `.cargo/config.toml` on every build's directory path: cargo reads it, the trust instrument refuses it ("a cargo
# configuration above the repository"), and the runner's checkout has none above it (`PROGRAM.65`). So the rehearsal
# checks out beside this repository, on its volume — the one exception to scratch under `target/`, ruled by the
# director `2026-10-06` (`docs/decisions/decision_ci-rehearsal-beside-the-repository.md`) — and removes the directory
# when it ends; `CI_REHEARSE_KEEP=1` keeps it. Before anything runs it checks that no cargo configuration lies above
# the checkout, and says "could not run", exit 2, if one does.
#
# ⚠️ HONEST LIMIT: this machine's userland (BSD `sed`/`awk`, Homebrew `bash`), not the runner's GNU one; the apt
# step and the actions themselves are not run. `PROGRAM.10.5` reads the first real run for what only it can show.
#
# CONTRACT: exit = the job's: 0 passes, anything else is the code of the step that failed; 2 when it could not run.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

WORK="$(dirname "$ROOT")/.archogen-ci-rehearsal"
REPO="$WORK/repo"
note() { printf 'ci-rehearse: %s\n' "$1" >&2; }
case "$WORK" in /*/.archogen-ci-rehearsal) ;; *) note "no directory beside the repository: $WORK"; exit 2 ;; esac

# The checkout's directory path holds no cargo configuration, as the runner's does not.
dir="$WORK"
while :; do
  for c in "$dir/.cargo/config" "$dir/.cargo/config.toml"; do
    if [ -e "$c" ]; then note "could not run: a cargo configuration lies above the checkout, $c"; exit 2; fi
  done
  [ "$dir" = / ] && break
  dir="$(dirname "$dir")"
done

sha="${1:-}"
if [ -z "$sha" ]; then
  sha="$(git stash create)"
  [ -n "$sha" ] || sha="$(git rev-parse HEAD)"
fi
sha="$(git rev-parse --verify "$sha^{commit}")" || { note "no such commit: ${1:-}"; exit 2; }
[ -d "$ROOT/target/ci/tools/bin" ] || { note "no provisioned tools in target/ci/tools — run scripts/ci_provision.sh first"; exit 2; }

# The checkout, as actions/checkout makes it. An unreferenced commit (a stash) is fetched by its id.
rm -rf "$WORK"; mkdir -p "$REPO"
[ "${CI_REHEARSE_KEEP:-}" = 1 ] || trap 'rm -rf "$WORK"' EXIT
git -C "$REPO" init -q
git -C "$REPO" fetch -q --upload-pack='git -c uploadpack.allowAnySHA1InWant=true upload-pack' \
  "$ROOT" "+$sha:refs/remotes/origin/main" || { note "could not fetch $sha"; exit 2; }
git -C "$REPO" checkout -q -B main refs/remotes/origin/main
note "rehearsing $(git -C "$REPO" rev-parse --short HEAD) in ${REPO#"$ROOT"/} — $(git -C "$REPO" ls-files | wc -l | tr -d ' ') tracked files, nothing else"

# The cache the job restores.
mkdir -p "$REPO/target/ci"
ln -s "$ROOT/target/ci/tools" "$REPO/target/ci/tools"

# The job's environment: no git configuration it did not bring, Actions' two files. The runner's home holds no git
# identity, and a variable cannot say so to every process: the catalog checker's git, and the catalog's and the
# trust instrument's builds, clear their environment and pass back `HOME`, so no `GIT_CONFIG_*` variable reached
# them, and the checker's git read this machine's `~/.gitconfig` (`PROGRAM.64`). So the job gets a home of its own,
# whose one file guesses no identity, and the toolchain by `RUSTUP_HOME`, which those processes pass back too.
: > "$WORK/github_path"; : > "$WORK/step_summary"
JOB_HOME="$WORK/home"; mkdir -p "$JOB_HOME"
printf '[user]\n\tuseConfigOnly = true\n' > "$JOB_HOME/.gitconfig"
TOOLCHAIN_HOME="${RUSTUP_HOME:-$HOME/.rustup}"; CRATES_HOME="${CARGO_HOME:-$HOME/.cargo}"
job() {
  env -u GIT_AUTHOR_NAME -u GIT_AUTHOR_EMAIL -u GIT_COMMITTER_NAME -u GIT_COMMITTER_EMAIL -u XDG_CONFIG_HOME \
    GIT_CONFIG_NOSYSTEM=1 HOME="$JOB_HOME" RUSTUP_HOME="$TOOLCHAIN_HOME" CARGO_HOME="$CRATES_HOME" \
    GITHUB_ACTIONS=true GITHUB_PATH="$WORK/github_path" GITHUB_STEP_SUMMARY="$WORK/step_summary" \
    PATH="$(paste -sd: "$WORK/github_path" | sed 's/$/:/')$PATH" \
    bash -c "cd '$REPO' && $1"
}

note "step: Provision mdbook and QEMU at their pins"
job "bash scripts/ci_provision.sh" || { rc=$?; note "the provisioning step failed (exit $rc)"; exit "$rc"; }
note "step: Integration tier"
job "bash scripts/ci_integration.sh"; rc=$?
if [ -s "$WORK/step_summary" ]; then note "the job summary:"; sed 's/^/    /' "$WORK/step_summary" >&2; fi
note "the job would $([ "$rc" -eq 0 ] && echo pass || echo "fail (exit $rc)")"
exit "$rc"
