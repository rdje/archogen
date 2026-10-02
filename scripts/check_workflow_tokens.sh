#!/usr/bin/env bash
# scripts/check_workflow_tokens.sh — WORKFLOW-TOKENS: every workflow holds a read-only token and keeps no credentials
# (leaf `M2.7.6.1`).
#
# ⭐ WHY THIS EXISTS. The catalog's check is trusted only if nothing a pull request controls can post its verdict
# (premise 3 of `docs/specs/catalog/decision_catalog-records.md`). Catalog round 15 found the route a pull request
# has (U4, `docs/reviews/decision_catalog-records-reviews.md`): "a workflow running pull-request code holds a token
# that can post the check's verdict". Its answer — "a read-only token by default, `contents: read` and no persisted
# credentials, enforced by a doctrine check" — is this gate's rule. Every workflow here runs on `push`, which a pull
# request's branch in this repository also triggers, so the rule takes every workflow, not the `pull_request` ones.
#
# THE RULE, for every `.github/workflows/*.yml` or `*.yaml`:
#   - a top-level `permissions:` — `{}`, `read-all`, or a block whose every scope is `read` or `none`;
#   - every other `permissions:`, a job's, held to the same: no job grants a write scope;
#   - every step that uses `actions/checkout` sets `persist-credentials: false` in its `with:` block, so the token
#     is not left in the clone's git configuration for the steps that follow;
#   - no `pull_request_target` or `workflow_run` trigger: both run with the base repository's token, the first on
#     what a pull request controls, the second on what a pull request's run left behind;
#   - no local action (`uses: ./…`), whose steps this gate cannot see.
# The gate reads a subset of YAML: block mappings and sequences, plain or quoted scalars, block scalars (whose text
# it skips), and flow sequences of scalars. It refuses what it cannot read with certainty — an anchor, an alias, a
# merge key, a flow mapping other than `permissions: {}`, a document marker, a tab, a line of no form it knows — so
# it fails closed rather than passing a file it misread. Since review round 2 was not completed (leaf `M2.7.6.4`), it
# also refuses a quoted key, a key written twice in one mapping, and two keys of one mapping that differ only in case:
# which of two values a parser keeps is the parser's, and GitHub turns an action's input names into environment
# variables by "convert[ing] input names to uppercase letters" (its metadata-syntax reference, the ledger's
# `github-actions-syntax`), so `PERSIST-CREDENTIALS` and `persist-credentials` would meet in one variable.
#
# ⚠️ HONEST LIMIT: the repository's default token, and every other setting findings §11 lists, is the hosting's; a
# commit cannot show it. This gate holds the half a commit can.
#
# CONTRACT: exit code is the verdict; explains on stderr, each refusal as `path:line`; read-only. `--self-test` runs
# its arms in scratch directories under `target/doctrine_scratch/`.
set -uo pipefail
SELF="$0"
case "$SELF" in /*) ;; *) SELF="$PWD/$SELF" ;; esac
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
SCRATCH="$ROOT/target/doctrine_scratch/workflow_tokens"

check() {
  python3 - <<'PY'
import re, subprocess, sys

FORBIDDEN_TRIGGERS = {"pull_request_target", "workflow_run"}
KEY = re.compile(r"""^(?P<key>"[^"]*"|'[^']*'|[^\s"'#&*!|>{}\[\],][^:]*?)\s*:(?:\s+(?P<val>.*))?$""")

def unquote(text):
    """A scalar's value. A single-quoted one doubles its quote and has no other escape; a double-quoted one with an
    escape never reaches here, since `read` refuses it."""
    text = text.strip()
    if len(text) >= 2 and text[0] == text[-1] == "'":
        return text[1:-1].replace("''", "'")
    if len(text) >= 2 and text[0] == text[-1] == '"':
        return text[1:-1]
    return text

def has_escape(line):
    """Whether a double-quoted scalar on the line holds a backslash: YAML resolves `\\x61` to `a` where this reader
    would not, so a key or value written with one is refused rather than misread (review round 1, V1)."""
    quote = None
    for i, ch in enumerate(line):
        if quote:
            if ch == quote:
                quote = None
            elif quote == '"' and ch == "\\":
                return True
        elif ch in "\"'" and (i == 0 or line[i - 1] in " :[,-"):
            quote = ch
    return False

def strip_comment(line):
    """The line without a trailing comment: a `#` at the start or after a space, outside quotes."""
    quote = None
    for i, ch in enumerate(line):
        if quote:
            if ch == quote:
                quote = None
        elif ch in "\"'" and (i == 0 or line[i - 1] in " :[,-"):
            quote = ch
        elif ch == "#" and (i == 0 or line[i - 1] in " \t"):
            return line[:i].rstrip()
    return line.rstrip()

def flow_items(val):
    """A flow sequence of scalars, `[a, b]`, as its items; None for anything else that opens a flow."""
    inner = val[1:-1] if val.startswith("[") and val.endswith("]") else None
    if inner is None or any(c in inner for c in "[]{}"):
        return None
    return [unquote(x) for x in inner.split(",") if x.strip()]

def read(path, text):
    """The file as entries (line, path of ancestor keys, key, value), or a refusal."""
    entries, stack, block, items = [], [], None, 0
    for n, raw in enumerate(text.split("\n"), 1):
        if "\t" in raw:
            return None, "%s:%d: a tab — write the file with spaces" % (path, n)
        indent = len(raw) - len(raw.lstrip(" "))
        if block is not None:
            if not raw.strip() or indent > block:
                continue
            block = None
        line = strip_comment(raw)
        if not line.strip():
            continue
        if has_escape(line):
            return None, "%s:%d: an escape in a double-quoted scalar — write it plain or single-quoted" % (path, n)
        body = line.strip()
        if body in ("---", "...") or body.startswith("--- "):
            return None, "%s:%d: a document marker — one document per workflow" % (path, n)
        # A sequence item: `- key: value` opens an item whose keys sit two columns in; `- value` is a scalar item.
        while body == "-" or body.startswith("- "):
            while stack and stack[-1][0] >= indent:
                stack.pop()
            items += 1
            stack.append((indent, "-%d" % items))
            rest = body[1:].lstrip(" ")
            indent += len(body) - len(rest)
            body = rest
            if not body:
                break
            if not KEY.match(body):
                if body[0] in "&*":
                    return None, "%s:%d: an anchor or alias — write the value out" % (path, n)
                if body[0] in "{":
                    return None, "%s:%d: a flow mapping — write it as a block" % (path, n)
                entries.append((n, [k for _, k in stack], None, unquote(body)))
                body = ""
                break
        if not body:
            continue
        m = KEY.match(body)
        if not m:
            return None, "%s:%d: a line the gate cannot read — `%s`" % (path, n, body[:60])
        if m.group("key")[:1] in ("\"", "'"):
            return None, "%s:%d: a quoted key — write it plain" % (path, n)
        key, val = unquote(m.group("key")), (m.group("val") or "").strip()
        if key == "<<":
            return None, "%s:%d: a merge key — write the mapping out" % (path, n)
        if val[:1] in ("&", "*"):
            return None, "%s:%d: an anchor or alias — write the value out" % (path, n)
        if val.startswith("{") and not (key == "permissions" and val == "{}"):
            return None, "%s:%d: a flow mapping — write it as a block" % (path, n)
        if val.startswith("[") and flow_items(val) is None:
            return None, "%s:%d: a nested flow collection — write it as a block" % (path, n)
        while stack and stack[-1][0] >= indent:
            stack.pop()
        entries.append((n, [k for _, k in stack], key, val))
        stack.append((indent, key))
        if re.fullmatch(r"[|>][-+0-9]*", val):
            block = indent
    seen = {}
    for n, parents, key, _ in entries:
        if key is None:
            continue
        first = seen.setdefault((tuple(parents), key.casefold()), (n, key))
        if first[0] != n:
            if first[1] == key:
                return None, ("%s:%d: `%s` written twice in one mapping (first on line %d) — which counts is the "
                              "parser's" % (path, n, key, first[0]))
            return None, ("%s:%d: `%s` and `%s` (line %d) differ only in case in one mapping — write one"
                          % (path, n, key, first[1], first[0]))
    return entries, None

def children(entries, parent):
    return [e for e in entries if e[1] == parent and e[2] is not None]

def check(path, text):
    entries, refusal = read(path, text)
    if refusal:
        return [refusal]
    fails = []
    top_permissions = [e for e in entries if e[1] == [] and e[2] == "permissions"]
    if not top_permissions:
        fails.append("%s: no top-level `permissions:` — the token would take the repository's default" % path)
    for n, parents, key, val in entries:
        if key != "permissions":
            continue
        where = "a job's" if parents else "the top-level"
        if val:
            if unquote(val) not in ("{}", "read-all"):
                fails.append("%s:%d: %s `permissions: %s` — only `{}`, `read-all` or a block of `read`/`none`"
                             % (path, n, where, val))
            continue
        scopes = children(entries, parents + ["permissions"])
        if not scopes:
            fails.append("%s:%d: %s `permissions:` is empty — write `{}` for none" % (path, n, where))
        for sn, _, scope, level in scopes:
            if unquote(level) not in ("read", "none"):
                fails.append("%s:%d: %s permission `%s: %s` — no workflow grants a write scope"
                             % (path, sn, where, scope, level))
    on = [e for e in entries if e[1] == [] and e[2] in ("on", "true")]
    if not on:
        fails.append("%s: no top-level `on:` the gate can read" % path)
    for n, _, key, val in on:
        triggers = set()
        if val:
            triggers.update(flow_items(val) or [unquote(val)])
        for tn, parents, tkey, tval in entries:
            if parents[:1] == [key] and len(parents) >= 1:
                if len(parents) == 1 and tkey is not None:
                    triggers.add(tkey)
                elif len(parents) == 2 and parents[1].startswith("-") and tkey is None:
                    triggers.add(tval)
        for t in sorted(triggers & FORBIDDEN_TRIGGERS):
            fails.append("%s:%d: the `%s` trigger runs with the base repository's token — not allowed" % (path, n, t))
    for n, parents, key, val in entries:
        if key != "uses":
            continue
        action = unquote(val)
        if action.startswith("./"):
            fails.append("%s:%d: a local action `%s` — the gate cannot see its steps" % (path, n, action))
            continue
        if not re.match(r"actions/checkout(@|$)", action):
            continue
        if not parents or not parents[-1].startswith("-"):
            continue
        settings = {k: unquote(v) for _, _, k, v in children(entries, parents + ["with"])}
        if settings.get("persist-credentials") != "false":
            fails.append("%s:%d: `%s` without `persist-credentials: false` in its `with:` — the token stays in the clone"
                         % (path, n, action))
    return fails

files = subprocess.run(["git", "ls-files", "-c", "-o", "--exclude-standard", "--", ".github/workflows"],
                       capture_output=True, text=True).stdout.split()
workflows = sorted(f for f in files if f.endswith((".yml", ".yaml")))
fails = []
for f in workflows:
    try:
        text = open(f, encoding="utf-8").read()
    except FileNotFoundError:
        continue
    fails.extend(check(f, text))
for f in fails:
    print("WORKFLOW-TOKENS: " + f, file=sys.stderr)
if fails:
    print("WORKFLOW-TOKENS: %d breach(es) — every workflow holds a read-only token and keeps no credentials"
          % len(fails), file=sys.stderr)
    sys.exit(1)
print("workflow-tokens: OK (%d workflow(s): a read-only token, no persisted credentials, no base-token trigger)"
      % len(workflows))
PY
}

self_test() {
  local arms=0 ok=0 work="$SCRATCH/selftest" base
  base='name: w
on:
  push:
  pull_request:
permissions:
  contents: read
jobs:
  j:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@0123456789abcdef # a pinned commit
        with:
          fetch-depth: 0
          persist-credentials: false
      - name: a script whose text looks like a refusal
        run: |
          echo "permissions: write-all"
          - uses: actions/checkout@x
          on: pull_request_target
'
  fresh() { # $1 = the workflow's text
    rm -rf "$work"; mkdir -p "$work/.github/workflows"; git -C "$work" init -q
    printf '%s' "$1" > "$work/.github/workflows/w.yml"
  }
  variant() { # $1 = text to replace in the base, $2 = its replacement
    python3 -c 'import sys; b, o, n = sys.argv[1:4]; assert b.count(o) == 1, o; sys.stdout.write(b.replace(o, n))' \
      "$base" "$1" "$2"
  }
  arm() { # $1 = name, $2 = expected rc, $3 = text the output must carry
    local name="$1" want="$2" must="$3" out rc
    arms=$((arms + 1))
    out="$(cd "$work" && bash "$SELF" 2>&1)"; rc=$?
    if [ "$rc" -ne "$want" ]; then
      echo "SELF-TEST: $name — expected exit $want, got $rc" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    if [ -n "$must" ] && ! printf '%s' "$out" | grep -qF -- "$must"; then
      echo "SELF-TEST: $name — not about \`$must\`:" >&2; printf '%s\n' "$out" | sed -n '1,3p' | sed 's/^/    /' >&2; return
    fi
    ok=$((ok + 1)); echo "  ✅ $name"
  }
  fresh "$base"; arm "a read-only workflow passes, its script's text not read as keys" 0 "1 workflow(s)"
  fresh "$(variant $'permissions:\n  contents: read\n' '')"
  arm "no top-level permissions is refused" 1 "no top-level \`permissions:\`"
  fresh "$(variant $'permissions:\n  contents: read\n' $'permissions: write-all\n')"
  arm "write-all is refused" 1 "\`permissions: write-all\`"
  fresh "$(variant $'  contents: read\n' $'  contents: read\n  statuses: write\n')"
  arm "a top-level write scope is refused" 1 "\`statuses: write\`"
  fresh "$(variant $'    runs-on: ubuntu-latest\n' $'    runs-on: ubuntu-latest\n    permissions:\n      checks: write\n')"
  arm "a job's write scope is refused" 1 "a job's permission \`checks: write\`"
  fresh "$(variant $'          persist-credentials: false\n' '')"
  arm "a checkout that keeps its token is refused" 1 "without \`persist-credentials: false\`"
  fresh "$(variant 'persist-credentials: false' 'persist-credentials: true')"
  arm "persist-credentials true is refused" 1 "without \`persist-credentials: false\`"
  fresh "$(variant $'  pull_request:\n' $'  pull_request_target:\n')"
  arm "a pull_request_target trigger is refused" 1 "the \`pull_request_target\` trigger"
  fresh "$(variant $'on:\n  push:\n  pull_request:\n' $'on: [push, workflow_run]\n')"
  arm "a workflow_run trigger in a flow list is refused" 1 "the \`workflow_run\` trigger"
  fresh "$(variant $'          fetch-depth: 0\n' $'          fetch-depth: &depth 0\n')"
  arm "an anchor is refused" 1 "an anchor or alias"
  fresh "$(variant $'        with:\n          fetch-depth: 0\n          persist-credentials: false\n' $'        with: {persist-credentials: false}\n')"
  arm "a flow mapping is refused" 1 "a flow mapping"
  fresh "$(variant $'      - name: a script' $'      - uses: ./.github/actions/mine\n      - name: a script')"
  arm "a local action is refused" 1 "a local action"
  fresh "$(variant $'  push:\n' $'\tpush:\n')"
  arm "a tab is refused" 1 "a tab"
  fresh "$(variant $'  pull_request:\n' $'  "pull_request_t\\x61rget":\n')"
  arm "an escaped key is refused, not misread (round 1, V1)" 1 "an escape in a double-quoted scalar"
  fresh "$(variant $'          persist-credentials: false\n      - name: a script' $'      - uses: actions/setup-node@0123456789abcdef\n        with:\n          persist-credentials: false\n      - name: a script')"
  arm "a sibling step's setting does not cover a checkout (round 1, V2)" 1 "without \`persist-credentials: false\`"
  fresh "$(variant $'          persist-credentials: false\n' $'          persist-credentials: false\n          persist-credentials: true\n')"
  arm "a key written twice in one mapping is refused" 1 "written twice in one mapping"
  fresh "$(variant $'          persist-credentials: false\n' $'          persist-credentials: false\n          PERSIST-CREDENTIALS: true\n')"
  arm "keys that differ only in case are refused" 1 "differ only in case"
  fresh "$(variant $'permissions:\n  contents: read\n' $'permissions:\n  contents: read\n\'permissions\': write-all\n')"
  arm "a quoted key is refused" 1 "a quoted key"
  rm -rf "$work"
  arms=$((arms + 1))
  if bash "$SELF" >/dev/null 2>&1; then ok=$((ok + 1)); echo "  ✅ the repository's workflows pass"
  else echo "SELF-TEST: the repository's workflows are refused — run the check to see why" >&2; fi
  echo "workflow-tokens self-test: $ok pass / $((arms - ok)) fail ($arms arms)"
  [ "$ok" -eq "$arms" ]
}

case "${1:-}" in
  --self-test) self_test; exit $? ;;
  "") check; exit $? ;;
  *) echo "usage: bash scripts/check_workflow_tokens.sh [--self-test]" >&2; exit 2 ;;
esac
