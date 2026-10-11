#!/usr/bin/env bash
# scripts/mutation_sweep.sh — a systematic mutation sweep of one Rust file's code (leaf `M3.6.6.1`).
#
# ⭐ WHY THIS EXISTS. The mutation catalogue (`xtask/mutations.txt`, `cargo xtask mutate`) holds the defects someone
# thought of. Four review rounds of `docs/specs/trust/decision_trust-generated-sources.md` in a row (its R7 to R10)
# each found a rule the record states, and the code keeps, that some change could remove with every test green — a
# rule nobody had thought to catalogue. A reader finds those one at a time; a sweep finds them all at once: it applies
# every mutation a fixed list of operators makes to the file's code, one at a time, and runs the tests against each.
# A mutation the tests do not notice is either a rule no test holds, which gains a test, or a change that alters
# nothing the file promises — an equivalent mutation, listed with its reason. The sweep is then the claim's evidence:
# "every mutation these operators make is killed, or listed as equivalent".
#
# THE OPERATORS, over each code line from the file's first `fn` to its `#[cfg(test)]`, comment lines skipped:
#   eq       `==` ⇄ `!=`                         cmp      `<` `<=` `>` `>=`, each to its neighbour
#   logic    `&&` ⇄ `||`, and each operand of a top-level `&&`/`||` in an `if`/`while` condition dropped
#   cond     an `if` condition → `true` and → `false`; a `while` condition → `false`
#   not      a `!` before a name or a call dropped        bool     `true` ⇄ `false`
#   int      an integer literal n in `+= n`, `+ n`, `-= n` → n + 1 and, from 1, n − 1
#   stmt     a statement line ending in `;` deleted      ws       `is_whitespace()` → `is_ascii_whitespace()`
#   case     `.to_lowercase()` and `.to_ascii_lowercase()` dropped     alt      each alternative of a `matches!` dropped
#   dir      `rsplit` ⇄ `split`, `rsplit_once` ⇄ `split_once`, `starts_with` → `contains`
#   lit      a string literal of two or more characters passed to `at`, `starts_with`, `strip_prefix` or `contains`,
#            its last character dropped and its first   chr      `'\n'` ⇄ `'\r'` in a comparison
#   iter     a one-line `for P in E {` narrowed to its first, last or two end elements, for owned/borrowed E;
#            a lexical mask excludes headers inside comments and Rust literals, preserving bindings and labels
#
# HOW IT RUNS, so the working tree is never touched: the tracked files as they stand in the working tree are copied to
# `target/mutation-sweep/tree/` (no gitlink), built there with `target/mutation-sweep/target/` as the target directory,
# and each mutation is written to the copy's file, tested with `cargo test -q -p <package> -- <filters>` under a
# timeout that kills the whole process group, and restored, read back byte for byte. The unmutated copy must pass
# first. An outcome is `killed` (a test failed — named), `timeout` (the tests did not finish: a hang the tests reach),
# `unviable` (does not compile — no mutation), `survived`, or `unjudged` (the command failed without a failed libtest
# summary and a failure-list name). An invocation error or crash without that evidence is never a kill.
#
# THE EQUIVALENTS FILE (`--equivalents <path>`): one entry per survivor judged equivalent, three lines and a blank —
#   from <the original line, trimmed>
#   to <the mutated line, trimmed>
#   why <one line: why the change alters nothing the file promises>
# For a multiline source span or replacement, use `from-json <JSON string>` / `to-json <JSON string>` instead.
# Raw fields keep backslashes literal. JSON fields decode escapes; only the whole span's outer whitespace is trimmed.
# Each entry requires exactly one source, one replacement and a nonempty reason. Unknown/duplicate/missing/empty
# fields and duplicate entries refuse, including the last entry without a trailing blank line.
# Matched by text, not line number, so an edit elsewhere keeps them.
#
# CONTRACT: exit 0 — every mutation killed, timed out, unviable or listed equivalent, and every listed equivalent
# still made and still surviving; 1 — a survivor not listed, or a listed one stale; 2 — could not run (the unmutated
# copy fails, a bad argument, a restore that did not read back, or an unjudged run). With `--timeouts-fail`, a timeout
# instead fails with exit 1, even if an equivalents entry names it. Ordinary timeout handling is unchanged.
# Writes only under `target/mutation-sweep/`.
# `--list` prints the mutations without running them.
#
# Usage: scripts/mutation_sweep.sh <file> --package <pkg> [--filter <test>]... [--equivalents <path>]
#        [--timeout <seconds>] [--timeouts-fail] [--only <id>]... [--list]
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
exec python3 - "$@" <<'PY'
import argparse, json, os, re, shutil, signal, subprocess, sys, time

ap = argparse.ArgumentParser(prog="mutation_sweep.sh")
ap.add_argument("file")
ap.add_argument("--package", required=True)
ap.add_argument("--filter", action="append", default=[])
ap.add_argument("--equivalents")
ap.add_argument("--timeout", type=int, default=180)
ap.add_argument("--timeouts-fail", action="store_true")
ap.add_argument("--only", action="append", default=[])
ap.add_argument("--list", action="store_true")
ap.add_argument("--op", action="append", default=[])  # only these operators: a run to explore, never the claim
a = ap.parse_args()

ROOT = os.getcwd()
WORK = os.path.join(ROOT, "target", "mutation-sweep")
TREE = os.path.join(WORK, "tree")
TARGET = os.path.join(WORK, "target")

src = open(a.file, encoding="utf-8").read()
lines = src.split("\n")
try:
    first = next(i for i, l in enumerate(lines) if re.match(r"\s*(pub(\(crate\))? )?fn ", l))
    last = next(i for i, l in enumerate(lines) if l.strip() == "#[cfg(test)]")
except StopIteration:
    print("mutation-sweep: %s has no `fn` before a `#[cfg(test)]`" % a.file, file=sys.stderr); sys.exit(2)

def operands(cond):
    """Top-level `&&`/`||` operands of a condition, as (start, end) spans."""
    spans, depth, start, i = [], 0, 0, 0
    while i < len(cond):
        c = cond[i]
        if c in "([{": depth += 1
        elif c in ")]}": depth -= 1
        elif depth == 0 and cond.startswith((" && ", " || "), i):
            spans.append((start, i)); i += 4; start = i; continue
        i += 1
    spans.append((start, len(cond)))
    return spans if len(spans) > 1 else []

def mask_rust(text):
    """Hide comments/literals without changing offsets or newlines; lifetimes and loop labels remain code."""
    masked, i = list(text), 0
    raw_literal = re.compile(r'(?:br|cr|r)(#*)"')
    char_literal = re.compile(r"(?:b)?'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'")
    def erase(start, stop):
        for k in range(start, stop):
            if text[k] != "\n":
                masked[k] = " "
    while i < len(text):
        start = i
        if text.startswith("//", i):
            end = text.find("\n", i)
            i = len(text) if end < 0 else end
        elif text.startswith("/*", i):
            depth, i = 1, i + 2
            while i < len(text) and depth:
                if text.startswith("/*", i):
                    depth, i = depth + 1, i + 2
                elif text.startswith("*/", i):
                    depth, i = depth - 1, i + 2
                else:
                    i += 1
        else:
            raw = raw_literal.match(text, i) if i == 0 or not (text[i - 1].isalnum() or text[i - 1] == "_") else None
            char = char_literal.match(text, i)
            if raw:
                end = text.find('"' + raw.group(1), raw.end())
                i = len(text) if end < 0 else end + 1 + len(raw.group(1))
            elif text[i] == '"':
                i += 1
                while i < len(text):
                    if text[i] == "\\":
                        i = min(i + 2, len(text))
                    elif text[i] == '"':
                        i += 1
                        break
                    else:
                        i += 1
            elif char:
                i = char.end()
            else:
                i += 1
                continue
        erase(start, i)
    return "".join(masked)

def loop_header(n, masked):
    match = re.match(r"^(?P<indent>[ \t]*)(?:(?P<label>'\w+:)\s+)?for\s+(?P<pattern>.+?)\s+in\s+(?P<expr>.+?)\s*\{\s*$", masked)
    if not match:
        return None
    line = lines[n]
    pattern = line[slice(*match.span("pattern"))].strip()
    expression = line[slice(*match.span("expr"))].strip()
    prefix = "__archogen_sweep_%d_" % (n + 1)
    source = "\n".join(lines)
    while prefix in source:
        prefix += "_"
    it, k, first_flag, last_flag = [prefix + name for name in ("it", "k", "first", "last")]
    header = (match.group("indent") + "let mut %s = (%s).into_iter().peekable(); let mut %s = 0usize; " % (it, expression, k)
              + ((match.group("label") + " ") if match.group("label") else "")
              + "while let Some(%s) = %s.next() { let (%s, %s) = (%s == 0, %s.peek().is_none()); %s += 1;"
              % (pattern, it, first_flag, last_flag, k, it, k))
    tail = line[masked.rfind("{") + 1:]
    return header, tail, (first_flag, last_flag, "(%s || %s)" % (first_flag, last_flag))

def apply_mutant(m):
    n, stop, _, new = m
    mutated = lines[:]
    mutated[n:stop] = new.split("\n")
    return "\n".join(mutated)

def mutation_key(m):
    n, stop, _, new = m
    return "\n".join(lines[n:stop]).strip(), new.strip()

def display(text):
    # Keep each result on one line while preserving multiline text unambiguously.
    return json.dumps(text, ensure_ascii=False) if "\n" in text else text

def load_equivalents(path):
    result, cur = {}, {}
    def finish():
        if not cur:
            return
        if set(cur) != {"from", "to", "why"}:
            raise ValueError("an entry requires exactly from/to/why")
        key = cur["from"], cur["to"]
        if key in result:
            raise ValueError("a duplicate equivalence entry")
        result[key] = cur["why"]
        cur.clear()
    with open(path, encoding="utf-8") as source:
        for number, raw in enumerate(source.read().split("\n"), 1):
            if raw.startswith("#"):
                continue
            if not raw.strip():
                finish()
                continue
            field, separator, value = raw.partition(" ")
            if not separator or field not in {"from", "to", "why", "from-json", "to-json"}:
                raise ValueError("line %d: unknown or malformed field" % number)
            key = field.removesuffix("-json")
            if key in cur:
                raise ValueError("line %d: duplicate %s field" % (number, key))
            if field.endswith("-json"):
                value = json.loads(value)
                if not isinstance(value, str):
                    raise ValueError("line %d: %s must be a JSON string" % (number, field))
            value = value.strip()
            if not value:
                raise ValueError("line %d: empty %s field" % (number, key))
            cur[key] = value
    finish()
    return result

def mutants():
    out = []
    seen = set()
    masked = mask_rust("\n".join(lines)).split("\n")
    def add(n, op, new, stop=None):
        stop = n + 1 if stop is None else stop
        if not first <= n < stop <= last:
            raise ValueError("mutation interval is outside the production range")
        if new != "\n".join(lines[n:stop]) and (n, stop, new) not in seen:
            seen.add((n, stop, new))
            out.append((n, stop, op, new))
    for n in range(first, last):
        l = lines[n]
        t = l.strip()
        if not t or t.startswith("//"):
            continue
        code = l.split(" // ")[0]
        for m in re.finditer(r"==|!=", code):
            add(n, "eq", l[:m.start()] + ("!=" if m.group() == "==" else "==") + l[m.end():])
        for m in re.finditer(r" (<=|>=|<|>) ", code):
            for to in {"<": ["<="], "<=": ["<"], ">": [">="], ">=": [">"]}[m.group(1)]:
                add(n, "cmp", l[:m.start(1)] + to + l[m.end(1):])
        for m in re.finditer(r" (&&|\|\|) ", code):
            add(n, "logic", l[:m.start(1)] + ("||" if m.group(1) == "&&" else "&&") + l[m.end(1):])
        cm = re.match(r"^(\s*(?:\} else )?(?:if|while) )(.*)( \{)\s*$", l)
        if cm and not cm.group(2).startswith("let "):
            head, cond, tail = cm.groups()
            for s, e in operands(cond):
                rest = (cond[:s] + cond[e:]).strip()
                rest = re.sub(r"^(&&|\|\|) ", "", rest); rest = re.sub(r" (&&|\|\|)$", "", rest)
                rest = rest.replace("  ", " ")
                add(n, "logic", head + rest + tail)
            if "while" in head:
                add(n, "cond", head + "false" + tail)
            else:
                add(n, "cond", head + "true" + tail); add(n, "cond", head + "false" + tail)
        for m in re.finditer(r"(?<![A-Za-z0-9_])!(?=[A-Za-z_(])", code):  # a negation, not a macro's `!`
            if code[m.start():m.start() + 2] != "!=":
                add(n, "not", l[:m.start()] + l[m.end():])
        for m in re.finditer(r"\b(true|false)\b", code):
            add(n, "bool", l[:m.start()] + ("false" if m.group() == "true" else "true") + l[m.end():])
        for m in re.finditer(r"(\+=|-=|\+) (\d+)\b", code):
            k = int(m.group(2))
            for to in ([k + 1] + ([k - 1] if k >= 1 else [])):
                add(n, "int", l[:m.start(2)] + str(to) + l[m.end(2):])
        # A whole statement: not a declaration, and not the closing or continuing line of a longer expression.
        if t.endswith(";") and not t.startswith(("let ", "return", "use ", "pub ", "const ", "static ", "}", ")", "||",
                                                 "&&", ".")) and "=>" not in t:
            add(n, "stmt", re.match(r"^\s*", l).group() + "// " + t)
        for m in re.finditer(r"\bis_whitespace\(\)", code):
            add(n, "ws", l[:m.start()] + "is_ascii_whitespace()" + l[m.end():])
        for m in re.finditer(r"\.to_(ascii_)?lowercase\(\)", code):
            add(n, "case", l[:m.start()] + ".to_string()" + l[m.end():])
        for m in re.finditer(r"\b(rsplit_once|split_once|rsplit|split|starts_with)\(", code):
            to = {"rsplit_once": "split_once", "split_once": "rsplit_once", "rsplit": "split", "split": "rsplit",
                  "starts_with": "contains"}[m.group(1)]
            add(n, "dir", l[:m.start(1)] + to + l[m.end(1):])
        for m in re.finditer(r'\b(?:at|starts_with|strip_prefix|contains)\((?:[^,()"]*,\s*)?"((?:[^"\\]|\\.)*)"\)', code):
            lit = m.group(1)
            if len(lit) >= 2 and "\\" not in lit:
                add(n, "lit", l[:m.start(1)] + lit[:-1] + l[m.end(1):])
                add(n, "lit", l[:m.start(1)] + lit[1:] + l[m.end(1):])
        for m in re.finditer(r"[!=]= '\\([nr])'", code):  # in a comparison; a pushed separator is either
            add(n, "chr", l[:m.start(1)] + ("r" if m.group(1) == "n" else "n") + l[m.end(1):])
        if " | " in code and "=>" not in code:
            parts = [p for p in re.split(r"( \| )", code)]
            alts = parts[0::2]
            if len(alts) > 1:
                for k in range(len(alts)):
                    keep = alts[:k] + alts[k + 1:]
                    lead = re.match(r"^\s*", alts[0]).group() if k == 0 else ""
                    new = " | ".join(x.strip() if (i or k == 0) else x for i, x in enumerate(keep))
                    if k == 0:
                        new = lead + new.lstrip()
                    add(n, "alt", new + l[len(code):])
        header = loop_header(n, masked[n])
        if header:
            rewritten, tail, positions = header
            for position in positions:
                add(n, "iter", rewritten + " if !%s { continue; }" % position + tail)
    return out

ms = [m for m in mutants() if not a.op or m[2] in a.op]
ids = ["L%d.%s.%d" % (n + 1, op, k) for k, (n, _, op, _) in enumerate(ms)]
if a.list:
    for i, m in zip(ids, ms):
        before, after = mutation_key(m)
        print("%-18s %s\n%18s ⟶ %s" % (i, display(before), "", display(after)))
    print("mutation-sweep: %d mutation(s) of %s, lines %d to %d" % (len(ms), a.file, first + 1, last))
    sys.exit(0)

equiv = {}
if a.equivalents:
    try:
        equiv = load_equivalents(a.equivalents)
    except (OSError, ValueError) as error:
        print("mutation-sweep: cannot read equivalents %s: %s" % (a.equivalents, error), file=sys.stderr)
        sys.exit(2)

# A fresh copy of the tracked tree as the working tree holds it; the target directory is kept between runs.
if os.path.isdir(TREE):
    shutil.rmtree(TREE)
os.makedirs(TREE)
staged = subprocess.run(["git", "ls-files", "-s", "-z"], capture_output=True, text=True, check=True).stdout
paths = [e.split("\t", 1)[1] for e in staged.split("\0") if e and not e.startswith("160000 ")]
for p in paths:
    if os.path.lexists(p) and not os.path.isdir(p):
        d = os.path.join(TREE, os.path.dirname(p)); os.makedirs(d, exist_ok=True)
        # A fresh mtime, not the original's: an older one than the last mutant's build would let cargo reuse it.
        shutil.copy(p, os.path.join(TREE, p), follow_symlinks=False)
target_file = os.path.join(TREE, a.file)
original = open(target_file, "rb").read()
env = dict(os.environ, CARGO_TARGET_DIR=TARGET)
cmd = ["cargo", "test", "-q", "-p", a.package, "--"] + a.filter

def failed_tests(stdout):
    names, pending = set(), None
    for line in stdout.splitlines():
        if line == "failures:":
            pending = []
        elif line.startswith("test result: FAILED."):
            if pending is not None:
                names.update(pending)
            pending = None
        elif line.strip():
            match = re.fullmatch(r"    ([\w:]+)", line)
            if pending is not None and match:
                pending.append(match.group(1))
            else:
                pending = None
    return sorted(names)

def run():
    t0 = time.time()
    p = subprocess.Popen(cmd, cwd=TREE, env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE, start_new_session=True)
    try:
        so, se = p.communicate(timeout=a.timeout)
    except subprocess.TimeoutExpired:
        os.killpg(p.pid, signal.SIGKILL); p.communicate()
        return "timeout", [], time.time() - t0
    so, se = so.decode("utf-8", "replace"), se.decode("utf-8", "replace")
    if p.returncode == 0:
        return "survived", [], time.time() - t0
    if "could not compile" in se and "test result" not in so:
        return "unviable", [next((x for x in se.splitlines() if x.startswith("error")), "")], time.time() - t0
    failed = failed_tests(so)
    if failed and any(l.startswith("test result: FAILED.") for l in so.splitlines()):
        return "killed", failed, time.time() - t0
    detail = next((l for l in se.splitlines() if l.strip()), "nonzero exit without a finishing, named test failure")
    return "unjudged", [detail], time.time() - t0

base, _, secs = run()
if base != "survived":
    print("mutation-sweep: could not run — the unmutated copy does not pass (%s, %.0fs): %s" % (base, secs, " ".join(cmd)), file=sys.stderr)
    sys.exit(2)
print("mutation-sweep: the unmutated copy passes (%.0fs); %d mutation(s) of %s, lines %d to %d" % (secs, len(ms), a.file, first + 1, last))

counts, unlisted, seen_equiv = {}, [], set()
try:
    for i, m in zip(ids, ms):
        if a.only and i not in a.only:
            continue
        mutated = apply_mutant(m)
        open(target_file, "w", encoding="utf-8").write(mutated)
        if open(target_file, encoding="utf-8").read() != mutated:
            print("mutation-sweep: %s did not land in %s" % (i, target_file), file=sys.stderr); sys.exit(2)
        try:
            outcome, detail, secs = run()
        finally:
            open(target_file, "wb").write(original)
            if open(target_file, "rb").read() != original:
                print("mutation-sweep: %s did not read back after %s" % (target_file, i), file=sys.stderr); sys.exit(2)
        key = mutation_key(m)
        if outcome == "survived" and key in equiv:
            outcome = "equivalent"; seen_equiv.add(key)
        counts[outcome] = counts.get(outcome, 0) + 1
        mark = {"killed": "✓", "timeout": "✗" if a.timeouts_fail else "✓", "unviable": "·",
                "equivalent": "=", "survived": "✗", "unjudged": "?"}[outcome]
        print("  %s %-18s %-10s %s ⟶ %s%s" % (mark, i, outcome, display(key[0]), display(key[1]),
              (" — " + ", ".join(detail)) if detail else ""), flush=True)
        if outcome == "unjudged":
            print("mutation-sweep: %s unjudged; resolve the command failure and rerun" % i, file=sys.stderr)
            sys.exit(2)
        if outcome == "survived" or (outcome == "timeout" and a.timeouts_fail):
            unlisted.append((i, key))
except KeyboardInterrupt:
    open(target_file, "wb").write(original); raise
stale = [k for k in equiv if k not in seen_equiv] if not a.only else []
print("mutation-sweep: " + ", ".join("%d %s" % (v, k) for k, v in sorted(counts.items())))
for i, (f, t) in unlisted:
    print("  survivor %s: %s ⟶ %s" % (i, display(f), display(t)), file=sys.stderr)
for f, t in stale:
    print("  a listed equivalent no longer made or no longer surviving: %s ⟶ %s" % (display(f), display(t)), file=sys.stderr)
sys.exit(1 if unlisted or stale else 0)
PY
