#!/usr/bin/env bash
# scripts/check_code_owners.sh — CODE-OWNERS: the repository's one ownership file is the one its protection record
# writes, and it leaves no path without an owner (leaf `M2.7.6.5`).
#
# ⭐ WHY THIS EXISTS. `main`'s ruleset requires a code owner's approval (`docs/decisions/decision_main-protection.md`
# §1), and the catalog's premise 3 rests on it: the agents author every change, and the director, the owner of every
# path, approves it. The hosting reads the ownership file and skips what it cannot read — "If any line in your
# CODEOWNERS file contains invalid syntax, that line will be skipped" (the ledger's `github-code-owners`) — so a
# skipped line, a second file read first, or a rule naming no owner would leave a path unowned with nothing failing.
# This gate holds the file to the record, and the record's text to the rules GitHub states.
#
# THE RULE:
#   - one file named `CODEOWNERS` in the tree, at `.github/CODEOWNERS`. GitHub searches `.github/`, the root and
#     `docs/` "in that order and use[s] the first one it finds"; a file anywhere else is read by nobody, and is
#     refused so that no reader takes it for the one in force;
#   - its bytes are the text block the record gives under **`.github/CODEOWNERS`**, so a change to the owners is a
#     change to the record first;
#   - every line blank, a comment, or a pattern with one owner or more, each `@user`, `@org/team` or an email
#     address; no pattern using `!`, `[ ]`, or a `\` before `#`, the gitignore syntax GitHub's page says "doesn't
#     work"; under 3 MB, the size over which the file "will not be loaded";
#   - its first rule's pattern `*`. The last matching pattern wins, so with `*` first and an owner on every rule, no
#     path is left without one.
#
# ⚠️ HONEST LIMIT: whether each owner has write access, and whether the hosting reads the file without error, is
# the hosting's (`GET /repos/{owner}/{repo}/codeowners/errors` on a pushed ref, read at the first batch,
# `M2.7.6.6.5`). The file binds only from the pull request's base, so a commit cannot show it in force.
#
# CONTRACT: exit code is the verdict; explains on stderr; read-only. `--self-test` runs its arms in scratch
# directories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/code_owners"
RECORD="docs/decisions/decision_main-protection.md"

check() {
  python3 - "$RECORD" <<'PY'
import re, subprocess, sys

RECORD = sys.argv[1]
WHERE = ".github/CODEOWNERS"
HEADING = "**`.github/CODEOWNERS`**"
LIMIT = 3 * 1000 * 1000
OWNER = re.compile(r"^(@[A-Za-z0-9](?:[A-Za-z0-9-]*[A-Za-z0-9])?(?:/[A-Za-z0-9._-]+)?|[^@\s]+@[^@\s]+\.[^@\s]+)$")

def record_block(text):
    """The text block that follows the record's paragraph naming the file: the lines between its ```text fence and
    the fence that closes it, each with its newline."""
    lines = text.splitlines(keepends=True)
    starts = [i for i, l in enumerate(lines) if l.startswith(HEADING)]
    if len(starts) != 1:
        return None, "the record names `.github/CODEOWNERS` in %d paragraph(s), not one" % len(starts)
    i = starts[0] + 1
    while i < len(lines) and not lines[i].startswith("```"):
        if lines[i].startswith(("**", "#")):
            return None, "the record's paragraph on `.github/CODEOWNERS` is followed by no text block"
        i += 1
    if i >= len(lines) or lines[i].rstrip("\n") != "```text":
        return None, "the record's paragraph on `.github/CODEOWNERS` is followed by no ```text block"
    body = []
    for l in lines[i + 1:]:
        if l.rstrip("\n") == "```":
            return "".join(body), None
        body.append(l)
    return None, "the record's ```text block for `.github/CODEOWNERS` is never closed"

def rules(text):
    """The file read as GitHub's page describes it: each failure a sentence, and the first rule's pattern."""
    fails, first = [], None
    if len(text.encode("utf-8")) >= LIMIT:
        fails.append("%s is 3 MB or more — GitHub does not load it" % WHERE)
    for n, line in enumerate(text.split("\n"), 1):
        body = line.strip()
        if not body or body.startswith("#"):
            continue
        fields = body.split()
        pattern, owners = fields[0], []
        for f in fields[1:]:
            if f.startswith("#"):
                break
            owners.append(f)
        if first is None:
            first = (n, pattern)
        if pattern.startswith("!"):
            fails.append("%s:%d: `%s` negates a pattern — GitHub's page: \"doesn't work\"" % (WHERE, n, pattern))
        if "[" in pattern or "]" in pattern:
            fails.append("%s:%d: `%s` uses a `[ ]` range — GitHub's page: \"doesn't work\"" % (WHERE, n, pattern))
        if pattern.startswith("\\#"):
            fails.append("%s:%d: `%s` escapes a `#` — GitHub's page: \"doesn't work\"" % (WHERE, n, pattern))
        if not owners:
            fails.append("%s:%d: `%s` names no owner — the paths it matches are left without one" % (WHERE, n, pattern))
        for o in owners:
            if not OWNER.match(o):
                fails.append("%s:%d: `%s` is no `@user`, `@org/team` or email address" % (WHERE, n, o))
    if first is None:
        fails.append("%s holds no rule — every path is left without an owner" % WHERE)
    elif first[1] != "*":
        fails.append("%s:%d: the first rule's pattern is `%s`, not `*` — a path no rule matches has no owner"
                     % (WHERE, first[0], first[1]))
    return fails

files = subprocess.run(["git", "ls-files", "-c", "-o", "--exclude-standard", "-z"],
                       capture_output=True, text=True).stdout.split("\0")
named = sorted(f for f in files if f and f.rsplit("/", 1)[-1] == "CODEOWNERS")
fails = []
for f in named:
    if f != WHERE:
        fails.append("%s: an ownership file other than `%s` — the repository has one" % (f, WHERE))
if WHERE not in named:
    fails.append("%s is missing — `main`'s ruleset requires a code owner's approval (%s §1)" % (WHERE, RECORD))
else:
    try:
        text = open(WHERE, encoding="utf-8", newline="").read()
    except (OSError, UnicodeDecodeError) as e:
        text = None
        fails.append("%s does not read as UTF-8 text: %s" % (WHERE, e))
    try:
        want, why = record_block(open(RECORD, encoding="utf-8", newline="").read())
    except (OSError, UnicodeDecodeError) as e:
        want, why = None, "%s does not read: %s" % (RECORD, e)
    if why:
        fails.append(why)
    if text is not None:
        if want is not None and text != want:
            fails.append("%s differs from the text block %s §1 gives — change the record first" % (WHERE, RECORD))
        fails.extend(rules(text))
for f in fails:
    print("CODE-OWNERS: " + f, file=sys.stderr)
if fails:
    print("CODE-OWNERS: %d breach(es) — one ownership file, the record's, leaving no path without an owner"
          % len(fails), file=sys.stderr)
    sys.exit(1)
print("code-owners: OK (%s is the record's text; every path owned)" % WHERE)
PY
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest" base record
  base='# every path is the director'"'"'s
* @rdje
'
  record() { # $1 = the text block's body
    printf '# r\n\n**`.github/CODEOWNERS`**, the only ownership file:\n\n```text\n%s```\n\n## Why\n' "$1"
  }
  fresh() { # $1 = the file's text, $2 = the record's block body (the file's text when absent)
    rm -rf "$work"; mkdir -p "$work/.github" "$work/docs/decisions"; git -C "$work" init -q
    printf '%s' "$1" > "$work/.github/CODEOWNERS"
    record "${2-$1}" > "$work/docs/decisions/decision_main-protection.md"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! grep -qF -- "$must" <<< "$out"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh "$base"; arm "the record's file passes" 0 "every path owned"
  fresh "$base"; rm "$work/.github/CODEOWNERS"; arm "a missing file is refused" 1 "is missing"
  fresh "$base"; printf '* @rdje\n' > "$work/CODEOWNERS"; arm "a second file at the root is refused" 1 "CODEOWNERS: an ownership file other than"
  fresh "$base"; mkdir -p "$work/docs"; printf '* @rdje\n' > "$work/docs/CODEOWNERS"
  arm "a file under docs/ is refused" 1 "docs/CODEOWNERS: an ownership file other than"
  fresh "$base" $'* @someone-else\n'; arm "a file the record does not give is refused" 1 "differs from the text block"
  fresh "$base"; printf '# r\n\nno paragraph\n' > "$work/docs/decisions/decision_main-protection.md"
  arm "a record naming no file is refused" 1 "in 0 paragraph(s)"
  fresh $'* @rdje\n/trust/\n'; arm "a rule with no owner is refused" 1 "names no owner"
  fresh $'* @rdje\n/trust/ # owned by nobody\n'; arm "an inline comment is no owner" 1 "names no owner"
  fresh $'/trust/ @rdje\n* @rdje\n'; arm "a first rule other than * is refused" 1 "not \`*\`"
  fresh $'* @rdje\n!/trust/ @rdje\n'; arm "a negated pattern is refused" 1 "negates a pattern"
  fresh $'* @rdje\n/[ab]/ @rdje\n'; arm "a [ ] range is refused" 1 "range"
  fresh $'* @rdje\n\\#x @rdje\n'; arm "an escaped # is refused" 1 "escapes a"
  fresh $'* rdje\n'; arm "an owner without @ is refused" 1 "is no \`@user\`"
  fresh $'# only comments\n'; arm "a file with no rule is refused" 1 "holds no rule"
  fresh $'* @rdje @org/team user@example.com # inline\n'; arm "a user, a team and an email address pass" 0 "every path owned"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the repository's ownership file passes"
  else echo "SELF-TEST: the repository's ownership file is refused — run the check to see why" >&2; fi
  echo "code-owners self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") check; exit $? ;;
  *) echo "usage: bash scripts/check_code_owners.sh [--self-test]" >&2; exit 2 ;;
esac
