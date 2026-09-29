#!/usr/bin/env bash
# scripts/ci_rehearse.sh — run the `integration` CI job's commands on this machine, against the repository as the
# runner receives it (leaf `PROGRAM.10.4`).
#
# ⭐ WHY. This repository's CI cannot be watched from here, and a workflow is only evidence once it has run. What
# can be reproduced is everything that is not the runner's operating system: a checkout of one commit, made the
# way `actions/checkout` makes it (`git init`, fetch the commit with its history, `checkout -B main`), so nothing
# untracked, ignored or built is there; no submodule initialised; no global or system git configuration and no
# identity the job did not set; `GITHUB_PATH` and `GITHUB_STEP_SUMMARY` honoured as Actions honours them.
#
# USAGE: scripts/ci_rehearse.sh [<commit>]
#   Default: the index and working tree as `git stash create` records them — tracked changes and staged new
#   files, no ref moved — so a change to the job can be rehearsed before it is committed; HEAD when clean.
#
# The pinned tools are taken from this checkout's `target/ci/tools/` (what the job's cache restores), linked into
# the rehearsal; the rehearsal builds everything else from scratch in `target/ci/rehearsal/`.
#
# ⚠️ HONEST LIMIT: this machine's userland (BSD `sed`/`awk`, Homebrew `bash`), not the runner's GNU one; the apt
# step and the actions themselves are not run. `PROGRAM.10.5` reads the first real run for what only it can show.
#
# CONTRACT: exit = the job's: 0 passes, anything else is the code of the step that failed.
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"

WORK="$ROOT/target/ci/rehearsal"
REPO="$WORK/repo"
note() { printf 'ci-rehearse: %s\n' "$1" >&2; }

sha="${1:-}"
if [ -z "$sha" ]; then
  sha="$(git stash create)"
  [ -n "$sha" ] || sha="$(git rev-parse HEAD)"
fi
sha="$(git rev-parse --verify "$sha^{commit}")" || { note "no such commit: ${1:-}"; exit 2; }
[ -d "$ROOT/target/ci/tools/bin" ] || { note "no provisioned tools in target/ci/tools — run scripts/ci_provision.sh first"; exit 2; }

# The checkout, as actions/checkout makes it. An unreferenced commit (a stash) is fetched by its id.
rm -rf "$WORK"; mkdir -p "$REPO"
git -C "$REPO" init -q
git -C "$REPO" fetch -q --upload-pack='git -c uploadpack.allowAnySHA1InWant=true upload-pack' \
  "$ROOT" "+$sha:refs/remotes/origin/main" || { note "could not fetch $sha"; exit 2; }
git -C "$REPO" checkout -q -B main refs/remotes/origin/main
note "rehearsing $(git -C "$REPO" rev-parse --short HEAD) in ${REPO#"$ROOT"/} — $(git -C "$REPO" ls-files | wc -l | tr -d ' ') tracked files, nothing else"

# The cache the job restores.
mkdir -p "$REPO/target/ci"
ln -s "$ROOT/target/ci/tools" "$REPO/target/ci/tools"

# The job's environment: no git configuration it did not bring, Actions' two files.
: > "$WORK/github_path"; : > "$WORK/step_summary"
job() {
  env -u GIT_AUTHOR_NAME -u GIT_AUTHOR_EMAIL -u GIT_COMMITTER_NAME -u GIT_COMMITTER_EMAIL \
    GIT_CONFIG_NOSYSTEM=1 GIT_CONFIG_GLOBAL=/dev/null \
    GIT_CONFIG_COUNT=1 GIT_CONFIG_KEY_0=user.useConfigOnly GIT_CONFIG_VALUE_0=true \
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
