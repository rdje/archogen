#!/usr/bin/env bash
# scripts/open_batch.sh — a batch of local `main` sent to GitHub as one pull request, and local `main` brought back
# in step once it has merged (leaf `M2.7.6.6.4`, `docs/decisions/decision_main-protection.md` §3).
#
# ⭐ WHY THIS EXISTS. `main` on GitHub changes only by a merged pull request that the director approved (ruleset
# `25999805`). The agents keep committing on local `main`; a batch carries those commits to GitHub as the machine
# account, `rdje-bot`, so the author is never the approver. Done by hand, each step is one a slip makes wrong: a
# push to the wrong remote, a review summary written for another tree, a pull request opened with the director's
# token, a local `main` that no longer descends from what GitHub merged.
#
# THE TWO STEPS:
#   open --summary <file> [--restore-ci <url>] [--dry-run]
#     - local `main`, clean, holding `origin/main` and ahead of it;
#     - due: `scripts/push_cadence.sh` says a push is due (exit 3), or `--restore-ci` names the failing run, the
#       ruling's one exception; the director is told first, which no script can check;
#     - the summary, a fresh agent context's review of the batch, names the batch's tip by its full hash, so it was
#       written for this tree and no other;
#     - one batch open at a time: no `batch/` branch may be on `agent` already;
#     - pushes the tip to `batch/<YYYY-MM-DD>-<the tip's first 8 hex digits>` on the remote `agent`, the machine
#       account's. GitHub deletes a branch once it merges, so a number counted from the branches would repeat;
#     - opens one pull request into `main` with the machine account's token, read from the macOS Keychain item
#       `archogen-rdje-bot-token` into the one command that needs it, never printed or written;
#     - its body the summary, the commit list and the director's step: approve, then merge in the web interface
#       with a merge commit, which GitHub signs.
#   update --summary <file> [--restore-ci <url>] [--dry-run]
#     - the open batch takes local `main`'s new commits, a fix for its failing checks among them: the same checks
#       as `open`, the batch's branch the only one on `agent` and its tip held by local `main`;
#     - a fast-forward push, never a force, and a comment on its pull request with the summary and the commits
#       added. The director's approval is dismissed by the push and given again (`dismiss_stale_reviews_on_push`).
#   sync [--dry-run]
#     - after a batch has merged: local `main` fast-forwards to `origin/main` when it can;
#     - when the agents committed after opening the batch, `origin/main` is a merge local `main` lacks. It is merged
#       locally only if GitHub's merge changed no file of the batch (its tree is its second parent's) and the local
#       merge leaves local `main`'s tree as it is. The merge changes no file, so it names no work unit: its subject
#       is `BATCH-LANDED`. A later batch then holds `origin/main`, as "branches up to date" requires.
#
# ⚠️ HONEST LIMIT: the director's approval and the merge are the director's, in the web interface. This script
# never approves, never merges on GitHub, and never pushes to `main`.
#
# CONTRACT: exit 0 = done (or, with `--dry-run`, would be done); 1 = refused, explained on stderr; 2 = usage.
# `--self-test` runs its arms in scratch repositories under `target/doctrine_scratch/`, with stand-ins for `gh` and
# `security` on the `PATH`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
REPO="rdje/archogen"
KEYCHAIN_ITEM="archogen-rdje-bot-token"
BODY_LIMIT=60000

say() { printf 'open-batch: %s\n' "$1"; }
refuse() { printf 'open-batch: %s\n' "$1" >&2; exit 1; }

in_step_ready() { # local main, clean, origin/main fetched
  [ "$(git symbolic-ref -q --short HEAD)" = "main" ] || refuse "not on \`main\` — a batch is local \`main\`"
  [ -z "$(git status --porcelain)" ] || refuse "the working tree is not clean — commit or stash first"
  git fetch -q origin main || refuse "\`git fetch origin main\` failed — the distance cannot be measured"
  git rev-parse -q --verify refs/remotes/origin/main >/dev/null || refuse "no \`origin/main\` after the fetch"
}

batch_args() { # sets summary, restore, dry from the step's arguments
  summary=""; restore=""; dry=0
  while [ $# -gt 0 ]; do
    case "$1" in
      --summary) summary="${2:-}"; shift 2 || refuse "--summary needs a file" ;;
      --restore-ci) restore="${2:-}"; shift 2 || refuse "--restore-ci needs the failing run's URL" ;;
      --dry-run) dry=1; shift ;;
      *) echo "usage: bash scripts/open_batch.sh open|update --summary <file> [--restore-ci <url>] [--dry-run]" >&2; exit 2 ;;
    esac
  done
  [ -n "$summary" ] || refuse "a batch needs --summary, a fresh agent context's review of it"
}

batch_ready() { # local main in step with origin/main and ahead of it, the push due, the summary bound to the tip
  in_step_ready
  git merge-base --is-ancestor refs/remotes/origin/main HEAD ||
    refuse "local \`main\` does not hold \`origin/main\` — run \`bash scripts/open_batch.sh sync\` first"
  tip="$(git rev-parse HEAD)"
  ahead="$(git rev-list --count refs/remotes/origin/main..HEAD)"
  [ "$ahead" -gt 0 ] || refuse "local \`main\` is not ahead of \`origin/main\` — nothing to send"
  if [ -n "$restore" ]; then
    case "$restore" in https://github.com/*/actions/runs/*) ;; *) refuse "--restore-ci takes the failing run's URL, \`https://github.com/…/actions/runs/…\`" ;; esac
    due="to restore CI, $restore"
  else
    bash scripts/push_cadence.sh >/dev/null 2>&1
    [ $? -eq 3 ] || refuse "no push is due: \`bash scripts/push_cadence.sh\` does not say so, and no --restore-ci run is named"
    due="at the push cadence"
  fi
  [ -f "$summary" ] && [ -s "$summary" ] || refuse "the summary \`$summary\` is missing or empty"
  grep -qF -- "$tip" "$summary" ||
    refuse "the summary does not name the batch's tip \`$tip\` — it was written for another tree"
  git remote get-url agent >/dev/null 2>&1 || refuse "no remote \`agent\`, the machine account's (§2 of the record)"
  open_refs="$(git ls-remote --heads agent 'refs/heads/batch/*')" ||
    refuse "\`git ls-remote agent\` failed — whether a batch is open cannot be told"
}

write_body() { # $1 = the body's path, $2 = the commits' base, $3 = its heading
  local list count shown=0 line
  list="$(git log --reverse --format='- `%h` %s' "$2..HEAD")"
  count="$(printf '%s\n' "$list" | wc -l | tr -d ' ')"
  {
    cat "$summary"
    printf '\n\n## The batch\n\n'
    printf -- '- Tip: `%s`, %s commit(s) ahead of `origin/main`, sent %s.\n' "$tip" "$ahead" "$due"
    printf -- '- For the director: approve, then merge in the web interface with **Create a merge commit**, which GitHub signs (`docs/decisions/decision_main-protection.md` §3).\n'
    printf '\n## %s\n\n' "$3"
  } > "$1"
  while IFS= read -r line; do
    if [ $(( $(wc -c < "$1") + ${#line} + 200 )) -gt "$BODY_LIMIT" ]; then
      printf -- '- … and %s more: `git log --reverse %s..%s`\n' "$((count - shown))" \
        "$(git rev-parse --short "$2")" "$(git rev-parse --short HEAD)" >> "$1"
      break
    fi
    printf '%s\n' "$line" >> "$1"; shown=$((shown + 1))
  done <<< "$list"
  printf '\n🤖 Generated with [Claude Code](https://claude.com/claude-code)\n' >> "$1"
}

with_token() { # runs `gh` with the machine account's token, read from the Keychain into this one command
  GH_TOKEN="$(security find-generic-password -s "$KEYCHAIN_ITEM" -w)" gh "$@"
}

open_batch() {
  local summary restore dry tip ahead due open_refs name first title body url
  batch_args "$@"; batch_ready
  [ -z "$open_refs" ] ||
    refuse "a batch is still open on GitHub, $(printf '%s' "$open_refs" | awk '{print $2}' | sed 's|refs/heads/||' | tr '\n' ' ')— \`update\` it, or have it closed and its branch deleted"
  name="batch/$(date -u +%F)-$(git rev-parse --short=8 HEAD)"
  first="$(git rev-list --reverse refs/remotes/origin/main..HEAD | head -n 1)"
  title="$name: $ahead commit(s), $(git log -1 --format=%s "$first" | cut -d' ' -f1) to $(git log -1 --format=%s HEAD | cut -d' ' -f1)"
  mkdir -p "$ROOT/target/batches"; body="$ROOT/target/batches/${name#batch/}.md"
  write_body "$body" refs/remotes/origin/main "Commits"
  if [ "$dry" -eq 1 ]; then
    say "dry run: would push \`$tip\` to \`$name\` on \`agent\` and open \"$title\"; the body is $body"; return 0
  fi
  git push -q agent "$tip:refs/heads/$name" || refuse "the push of \`$name\` to \`agent\` failed"
  url="$(with_token pr create --repo "$REPO" --base main --head "$name" --title "$title" --body-file "$body")" ||
    refuse "\`$name\` is pushed, and opening its pull request failed — open it with the same body, $body"
  printf '%s %s %s\n' "$name" "$tip" "$url" >> "$ROOT/target/batches/opened.txt"
  say "opened $url — $name at \`$tip\`; the director approves and merges it"
}

update_batch() {
  local summary restore dry tip ahead due open_refs name old body
  batch_args "$@"; batch_ready
  [ -n "$open_refs" ] || refuse "no batch is open on GitHub — \`open\` one"
  [ "$(printf '%s\n' "$open_refs" | wc -l | tr -d ' ')" -eq 1 ] ||
    refuse "more than one batch branch is on GitHub — one batch is open at a time; have the others closed"
  old="$(printf '%s' "$open_refs" | awk '{print $1}')"; name="$(printf '%s' "$open_refs" | awk '{print $2}' | sed 's|refs/heads/||')"
  [ "$old" != "$tip" ] || refuse "\`$name\` already holds local \`main\` — nothing to add"
  git cat-file -e "$old^{commit}" 2>/dev/null && git merge-base --is-ancestor "$old" HEAD ||
    refuse "local \`main\` does not hold \`$name\`'s tip \`$old\` — the batch moved without this \`main\`"
  mkdir -p "$ROOT/target/batches"; body="$ROOT/target/batches/${name#batch/}-$(git rev-parse --short=8 HEAD).md"
  write_body "$body" "$old" "Added to the batch"
  if [ "$dry" -eq 1 ]; then
    say "dry run: would push \`$tip\` to \`$name\` on \`agent\`, a fast-forward from \`$old\`; the comment is $body"; return 0
  fi
  git push -q agent "$tip:refs/heads/$name" || refuse "the push of \`$tip\` to \`$name\` failed"
  with_token pr comment "$name" --repo "$REPO" --body-file "$body" >/dev/null ||
    refuse "\`$name\` now holds \`$tip\`, and commenting on its pull request failed — comment with $body"
  printf '%s %s update\n' "$name" "$tip" >> "$ROOT/target/batches/opened.txt"
  say "updated $name to \`$tip\`; the director's approval, if given, is stale and is given again"
}

sync_batch() {
  local dry=0 head merged msg
  while [ $# -gt 0 ]; do
    case "$1" in
      --dry-run) dry=1; shift ;;
      *) echo "usage: bash scripts/open_batch.sh sync [--dry-run]" >&2; exit 2 ;;
    esac
  done
  in_step_ready
  head="$(git rev-parse HEAD)"; merged="$(git rev-parse refs/remotes/origin/main)"
  if [ "$head" = "$merged" ]; then say "local \`main\` is \`origin/main\`"; return 0; fi
  if git merge-base --is-ancestor HEAD "$merged"; then
    [ "$dry" -eq 1 ] && { say "dry run: would fast-forward local \`main\` to \`$merged\`"; return 0; }
    git merge -q --ff-only "$merged" || refuse "the fast-forward to \`$merged\` failed"
    say "local \`main\` fast-forwarded to \`origin/main\`, \`$merged\`"; return 0
  fi
  if git merge-base --is-ancestor "$merged" HEAD; then
    say "local \`main\` holds \`origin/main\` and is $(git rev-list --count "$merged"..HEAD) ahead — nothing has landed"
    return 0
  fi
  [ "$(git rev-list --parents -n 1 "$merged" | wc -w | tr -d ' ')" -eq 3 ] ||
    refuse "\`origin/main\` is \`$merged\`, no merge of two parents, and local \`main\` does not hold it — not a batch's landing"
  git merge-base --is-ancestor "$merged^1" HEAD && git merge-base --is-ancestor "$merged^2" HEAD ||
    refuse "local \`main\` lacks a parent of \`origin/main\`'s merge — it merged something no batch of this \`main\` sent"
  [ "$(git rev-parse "$merged^{tree}")" = "$(git rev-parse "$merged^2^{tree}")" ] ||
    refuse "GitHub's merge \`$merged\` changed files of the batch it merged — read it before anything merges it here"
  local result
  result="$(git merge-tree --write-tree HEAD "$merged" 2>/dev/null)" ||
    refuse "local \`main\` and \`origin/main\` do not merge cleanly"
  [ "$result" = "$(git rev-parse HEAD^{tree})" ] ||
    refuse "merging \`origin/main\` would change local \`main\`'s files — not a batch's landing"
  msg="BATCH-LANDED $(git log -1 --format=%s "$merged" | cut -c1-60): origin/main at $(git rev-parse --short "$merged")"
  [ "$dry" -eq 1 ] && { say "dry run: would merge \`$merged\` into local \`main\`, its files unchanged: $msg"; return 0; }
  git merge -q --no-ff --no-commit "$merged" >/dev/null 2>&1 || refuse "\`git merge --no-commit\` of \`$merged\` failed"
  git commit -q -m "$msg" || refuse "the commit of the landing merge failed — \`git merge --abort\` restores local \`main\`"
  [ "$(git rev-parse HEAD^{tree})" = "$result" ] || refuse "the landing merge's tree is not local \`main\`'s — read HEAD"
  say "local \`main\` now holds \`origin/main\`, \`$merged\`; its files unchanged"
}

self_test() {
  local arms=0 ok=0 base="$ROOT/target/doctrine_scratch/open_batch" work stubs remote out rc
  work="$base/work"; stubs="$base/stubs"; remote="$base/remote.git"
  stub() { # the stand-ins: `gh` records its arguments and answers a URL; `security` answers a fake token
    rm -rf "$stubs"; mkdir -p "$stubs"
    printf '#!/usr/bin/env bash\n[ "${GH_TOKEN:-}" = "fake-token" ] || { echo "gh: no token" >&2; exit 4; }\nprintf "%%s\\n" "$*" >> "%s/gh.log"\necho https://github.com/rdje/archogen/pull/1\n' "$stubs" > "$stubs/gh"
    printf '#!/usr/bin/env bash\necho fake-token\n' > "$stubs/security"
    chmod +x "$stubs/gh" "$stubs/security"
  }
  g() { git -C "$work" -c user.name=arm -c user.email=arm@example.invalid "$@"; }
  fresh() { # $1 = commits ahead, $2 = the cadence's threshold
    rm -rf "$work" "$remote"; mkdir -p "$work/scripts" "$work/docs/decisions"
    git init -q --bare "$remote"; git init -q -b main "$work"
    cp "$ROOT/scripts/push_cadence.sh" "$work/scripts/"
    printf '# Push cadence: `N = %s` commits ahead\n\n- **Threshold:** `PUSH_AT_COMMITS_AHEAD=%s`\n' "$2" "$2" \
      > "$work/docs/decisions/decision_push-cadence.md"
    printf 'target/\n' > "$work/.gitignore"
    g add -A; g commit -q -m base
    g remote add origin "$remote"; g remote add agent "$remote"; g push -q origin main
    local i=0
    while [ "$i" -lt "$1" ]; do printf '%s\n' "$i" > "$work/f$i"; g add -A; g commit -q -m "ARCHOGEN-T-$i c$i"; i=$((i + 1)); done
    mkdir -p "$work/target"; g rev-parse HEAD > "$work/target/summary.md"
    stub
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry, then the arguments
    local name="$1" want="$2" must="$3"; shift 3
    arms=$((arms + 1))
    out="$(cd "$work" && PATH="$stubs:$PATH" GIT_AUTHOR_NAME=arm GIT_AUTHOR_EMAIL=arm@example.invalid \
      GIT_COMMITTER_NAME=arm GIT_COMMITTER_EMAIL=arm@example.invalid bash "$SELF" "$@" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! grep -qF -- "$must" <<< "$out"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  land() { # GitHub's merge of the open batch into main, made in the bare remote as the web interface would; the
    # branch is deleted after, as `delete_branch_on_merge` does
    local b t
    b="$(git -C "$remote" for-each-ref --format='%(refname)' refs/heads/batch/)"
    t="$(git -C "$remote" rev-parse "$b^{tree}")"
    git -C "$remote" update-ref refs/heads/main "$(git -C "$remote" -c user.name=gh -c user.email=gh@example.invalid \
      commit-tree "$t" -p refs/heads/main -p "$b" -m "Merge pull request #1 from rdje/${b#refs/heads/}")"
    git -C "$remote" update-ref -d "$b"
  }
  opened() { (cd "$work" && PATH="$stubs:$PATH" bash "$SELF" open --summary target/summary.md >/dev/null 2>&1); }
  later() { printf '%s\n' "$1" > "$work/later-$1"; g add -A; g commit -q -m "ARCHOGEN-T-9 later $1"; g rev-parse HEAD > "$work/target/summary.md"; }
  local name
  fresh 3 3; name="batch/$(date -u +%F)-$(g rev-parse --short=8 HEAD)"
  arm "a due batch opens, pushed to agent, its pull request by the machine token" 0 "opened https://github.com/rdje/archogen/pull/1" open --summary target/summary.md
  arms=$((arms + 1))
  if [ "$(git -C "$remote" rev-parse "refs/heads/$name")" = "$(g rev-parse HEAD)" ] &&
     grep -qF -- "--base main --head $name" "$stubs/gh.log" &&
     [ "$(git -C "$remote" rev-parse refs/heads/main)" != "$(g rev-parse HEAD)" ]; then
    ok=$((ok + 1)); echo "  ✅ the tip is on its batch branch, named for the day and the tip, never on main"
  else echo "SELF-TEST: the batch branch, the pull request or main is wrong" >&2; fi
  arm "a second batch while one is open is refused" 1 "still open" open --summary target/summary.md
  arm "an update with nothing new is refused" 1 "nothing to add" update --summary target/summary.md
  later a; arm "an update adds the new commits to the open batch and comments" 0 "updated $name" update --summary target/summary.md
  arms=$((arms + 1))
  if [ "$(git -C "$remote" rev-parse "refs/heads/$name")" = "$(g rev-parse HEAD)" ] && grep -qF -- "pr comment $name" "$stubs/gh.log"; then
    ok=$((ok + 1)); echo "  ✅ the open batch's branch fast-forwards to the new tip"
  else echo "SELF-TEST: the update did not move the batch's branch, or did not comment" >&2; fi
  later b; printf 'a review of the last tree\n' > "$work/target/summary.md"
  arm "an update whose summary does not name the new tip is refused" 1 "written for another tree" update --summary target/summary.md
  git -C "$remote" update-ref "refs/heads/$name" "$(git -C "$remote" -c user.name=gh -c user.email=gh@example.invalid \
    commit-tree "$(git -C "$remote" rev-parse "refs/heads/$name^{tree}")" -p "refs/heads/$name" -m "pushed elsewhere")"
  g rev-parse HEAD > "$work/target/summary.md"
  arm "an update to a batch that moved without this main is refused" 1 "the batch moved without" update --summary target/summary.md
  fresh 3 3; arm "an update with no open batch is refused" 1 "no batch is open" update --summary target/summary.md
  fresh 2 3; arm "a batch below the cadence is refused" 1 "no push is due" open --summary target/summary.md
  fresh 2 3; arm "a batch to restore CI opens below the cadence" 0 "dry run" open --summary target/summary.md --restore-ci https://github.com/rdje/archogen/actions/runs/1 --dry-run
  fresh 2 3; arm "--restore-ci without a run's URL is refused" 1 "takes the failing run's URL" open --summary target/summary.md --restore-ci soon
  fresh 3 3; printf 'a review of another tree\n' > "$work/target/summary.md"
  arm "a summary that does not name the tip is refused" 1 "written for another tree" open --summary target/summary.md
  fresh 3 3; arm "a batch with no summary is refused" 1 "needs --summary" open
  fresh 3 3; printf 'x\n' > "$work/f0"; arm "a dirty tree is refused" 1 "not clean" open --summary target/summary.md
  fresh 3 3; g checkout -q -b other; arm "a branch other than main is refused" 1 "not on \`main\`" open --summary target/summary.md
  fresh 3 3; g remote remove agent; arm "no agent remote is refused" 1 "no remote \`agent\`" open --summary target/summary.md
  fresh 3 3; printf '#!/usr/bin/env bash\necho other-token\n' > "$stubs/security"
  arm "a pull request is opened with the Keychain's token or not at all" 1 "opening its pull request failed" open --summary target/summary.md
  fresh 0 3; arm "nothing ahead is refused" 1 "not ahead" open --summary target/summary.md
  # The landing: GitHub merges the batch; local main fast-forwards, or merges it with its files unchanged.
  fresh 3 3; opened; land
  arm "after the merge, local main fast-forwards" 0 "fast-forwarded" sync
  fresh 3 3; opened; land; later c
  local before; before="$(g rev-parse HEAD^{tree})"
  arm "after the merge, later work keeps its files and gains the landing" 0 "its files unchanged" sync
  arms=$((arms + 1))
  if [ "$(g rev-parse HEAD^{tree})" = "$before" ] && g merge-base --is-ancestor "$(git -C "$remote" rev-parse refs/heads/main)" HEAD &&
     g log -1 --format=%s | grep -q '^BATCH-LANDED '; then ok=$((ok + 1)); echo "  ✅ the landing merge changes no file and names no work unit"
  else echo "SELF-TEST: the landing merge changed a file, or local main lacks origin/main" >&2; fi
  arm "a sync in step says so" 0 "holds \`origin/main\`" sync
  g rev-parse HEAD > "$work/target/summary.md"
  arm "after the landing, the next batch opens" 0 "dry run" open --summary target/summary.md --restore-ci https://github.com/rdje/archogen/actions/runs/2 --dry-run
  fresh 3 3; opened
  local b t2; b="$(git -C "$remote" for-each-ref --format='%(refname)' refs/heads/batch/)"
  t2="$(git -C "$remote" -c user.name=gh -c user.email=gh@example.invalid commit-tree \
    "$(git -C "$remote" rev-parse "refs/heads/main^{tree}")" -p refs/heads/main -p "$b" -m tampered)"
  git -C "$remote" update-ref refs/heads/main "$t2"
  later d
  arm "a merge whose tree is not the batch's is refused" 1 "changed files of the batch" sync
  rm -rf "$base"
  echo "open-batch self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  open) shift; open_batch "$@" ;;
  update) shift; update_batch "$@" ;;
  sync) shift; sync_batch "$@" ;;
  --self-test) self_test; exit $? ;;
  *) echo "usage: bash scripts/open_batch.sh open|update --summary <file> [--restore-ci <url>] [--dry-run] | sync [--dry-run] | --self-test" >&2; exit 2 ;;
esac
