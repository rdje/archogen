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
#   guard    the same loop rewrite without skipping elements; one `if C {` or `} else if C {` in its body,
#            outside nested loops/closures, becomes `(C) && first`, last or ends; C is still evaluated first
#   chain    `.iter()`/`.into_iter()` and `.filter(...)` followed by `.take(1)` or `.rev().take(1)`;
#            a filter or chain call dropped, with balanced multiline arguments and lexical exclusions
#   contains `X.contains(Y)` narrowed to a first/last comparison with `Some(Y)`, direct and copied-element forms;
#            grouped postfix receivers and multiline arguments retained; compilation decides type viability
#   clogic   each top-level `&&`/`||` operand of a one-line closure condition dropped with one separator;
#            balanced groups/generics and lexical exclusions retain nested scopes and complete literals
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
# Sources must be normalized repository-relative, indexed stage-zero regular files with no linked parent.
# Invalid source paths refuse with exit 2 before listing, replacing a retained copy or running commands.
# Writes only under `target/mutation-sweep/`.
# Each test command receives ARCHOGEN_SWEEP_SOURCE as this sweep's canonical own Git root, replacing inherited
# values. The tests alone bind that source to the exact target/mutation-sweep/tree copy before admitting its config.
# A failed baseline retains raw stdout/stderr and a source/command/status/names receipt under
# `target/mutation-sweep/baseline-failures/`, prints its relative path and exits 2 without retry.
# Evidence-write failures are explicit and remain exit 2. Successful baselines create no failure receipt.
# Selected IDs must all belong to the current operator-filtered discovery, including with --list.
# Unknown/mixed/stale/excluded IDs refuse before replacing the copy or running commands; duplicates run once.
# `--list` prints all discovered mutations without running them.
# Completed runs also report per-operator `unviable/attempted` counts, including zero refusals. The baseline,
# listed-only and filtered-out candidates are excluded; neither an attempted nor an unviable count is coverage.
#
# Usage: scripts/mutation_sweep.sh <file> --package <pkg> [--filter <test>]... [--equivalents <path>]
#        [--timeout <seconds>] [--timeouts-fail] [--only <id>]... [--list]
set -uo pipefail
ROOT="$(git rev-parse --show-toplevel)"; cd "$ROOT"
exec python3 - "$@" <<'PY'
import argparse, bisect, hashlib, json, os, re, shutil, signal, stat, subprocess, sys, tempfile, time

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

ROOT = os.path.realpath(os.getcwd())
WORK = os.path.join(ROOT, "target", "mutation-sweep")
TREE = os.path.join(WORK, "tree")
TARGET = os.path.join(WORK, "target")

# Resolve the selected input before discovery, deletion of an older copy or any test command.
# Index mode alone is insufficient: the working file or one of its parents may now be a link.
try:
    if os.path.isabs(a.file) or any(part in {"", ".", ".."} for part in a.file.split(os.sep)):
        raise ValueError("expected a normalized repository-relative path")
    staged = subprocess.run(["git", "ls-files", "-s", "-z"], capture_output=True, text=True, check=True).stdout
    selected = [entry.split("\t", 1)[0].split() for entry in staged.split("\0")
                if entry and entry.split("\t", 1)[1] == a.file]
    if len(selected) != 1 or selected[0][0] not in {"100644", "100755"} or selected[0][2] != "0":
        raise ValueError("expected one indexed regular file at stage zero")
    current = ROOT
    parts = a.file.split(os.sep)
    for number, part in enumerate(parts):
        current = os.path.join(current, part)
        mode = os.lstat(current).st_mode
        if not (stat.S_ISREG(mode) if number == len(parts) - 1 else stat.S_ISDIR(mode)):
            raise ValueError("expected regular source and directory parents without links")
    src = open(a.file, encoding="utf-8").read()
except (OSError, ValueError, subprocess.CalledProcessError) as error:
    print("mutation-sweep: invalid source path %r: %s" % (a.file, error), file=sys.stderr)
    sys.exit(2)
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

def operand_deletions(cond, spans):
    """Drop the first operand with its following separator; otherwise drop its preceding separator."""
    for index, (start, end) in enumerate(spans):
        if index == 0:
            yield cond[:start] + cond[spans[1][0]:]
        else:
            yield cond[:spans[index - 1][1]] + cond[end:]

def mask_rust(text, literal_marks=False):
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
        literal = False
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
                literal = True
                end = text.find('"' + raw.group(1), raw.end())
                i = len(text) if end < 0 else end + 1 + len(raw.group(1))
            elif text[i] == '"':
                literal = True
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
                literal = True
                i = char.end()
            else:
                i += 1
                continue
        erase(start, i)
        if literal and literal_marks:
            masked[start] = "L"
    return "".join(masked)

def closure_conditions(source):
    """One-line closure boolean spans, with top-level connective operands; not a complete Rust parser."""
    masked = mask_rust(source, literal_marks=True)
    tokens = list(re.finditer(r"r#\w+|'\w+|\w+|::|=>|\|\||&&|==|!=|<=|>=|[^\s]", masked))
    words = [token.group() for token in tokens]
    pairs, stack = {}, []
    for i, word in enumerate(words):
        if word in ("(", "[", "{"):
            stack.append(i)
        elif word in (")", "]", "}"):
            if not stack or words[stack[-1]] != {")": "(", "]": "[", "}": "{"}[word]:
                raise ValueError("unbalanced Rust source groups")
            pairs[stack.pop()] = i
    if stack:
        raise ValueError("unbalanced Rust source groups")

    def angle_end(i):
        depth = 0
        while i < len(words):
            if i in pairs:
                i = pairs[i] + 1
                continue
            if words[i] == "<":
                depth += 1
            elif words[i] == ">" and words[i - 1] != "-":
                depth -= 1
                if depth == 0:
                    return i + 1
            i += 1
        raise ValueError("unterminated Rust generic arguments")

    def advance(i):
        if i in pairs:
            return pairs[i] + 1
        if words[i] == "<" and i and words[i - 1] == "::":
            return angle_end(i)
        return i + 1

    contexts = {"=", "(", "[", "{", ",", "=>", "return", "break", "move", "async", "const", "&", "*", "!"}
    parameter_ends = set()
    for opening, word in enumerate(words):
        if word not in ("|", "||") or (opening and words[opening - 1] not in contexts
                                           and opening - 1 not in parameter_ends):
            continue
        body = opening + 1
        if word == "|":
            while body < len(words) and words[body] != "|":
                if words[body] in (";", "}"):
                    break
                body = angle_end(body) if words[body] == "<" else advance(body)
            if body == len(words) or words[body] != "|":
                continue
            parameter_ends.add(body)
            body += 1
        else:
            parameter_ends.add(opening)
        if words[body:body + 2] == ["-", ">"]:
            body += 2
            while body < len(words) and words[body] != "{":
                body = angle_end(body) if words[body] == "<" else advance(body)
        if body >= len(words):
            continue
        end = body
        if words[body] == "{":
            end = pairs[body] + 1
            raw_end = tokens[end - 1].end()
        else:
            while end < len(words) and words[end] not in (",", ";", ")", "]", "}"):
                end = advance(end)
            raw_end = tokens[end].start() if end < len(words) else len(source)
        if "\n" in source[tokens[opening].start():raw_end]:
            continue
        begin, stop = body, end
        if words[body] == "{":
            begin, stop = body + 1, pairs[body]
            i = begin
            while i < stop:
                if words[i] == ";":
                    if i == stop - 1 and words[begin] == "return":
                        stop = i
                        break
                    begin = i + 1
                i = advance(i)
        if begin < stop and words[begin] == "return":
            begin += 1
        while begin < stop and words[begin] == "(" and pairs[begin] == stop - 1:
            begin, stop = begin + 1, stop - 1
        if begin >= stop or words[begin] in ("|", "||", "move", "async", "let", "if", "match", "for", "while", "loop"):
            continue
        start = tokens[begin].start()
        finish = tokens[stop].start() if stop < len(tokens) else len(source)
        spans, operand, i = [], 0, begin
        while i < stop:
            if words[i] in ("&&", "||"):
                spans.append((operand, tokens[i].start() - start))
                operand = tokens[i].end() - start
            i = advance(i)
        if spans:
            spans.append((operand, finish - start))
            yield start, finish, spans

def guard_targets(n):
    """One-line branch headers in this loop, outside nested loops/closures; source lines, never token spellings."""
    source = "\n".join(lines)
    masked = mask_rust(source, literal_marks=True)
    starts = [0]
    for line in lines[:-1]:
        starts.append(starts[-1] + len(line) + 1)
    tokens = list(re.finditer(r"r#\w+|'\w+|\w+|::|=>|\|\||&&|\.\.=|\.\.|==|!=|<=|>=|[^\s]", masked))
    words = [token.group() for token in tokens]
    pairs, stack = {}, []
    for i, word in enumerate(words):
        if word in ("(", "[", "{"):
            stack.append(i)
        elif word in (")", "]", "}"):
            if not stack or words[stack[-1]] != {")": "(", "]": "[", "}": "{"}[word]:
                raise ValueError("unbalanced Rust source groups")
            pairs[stack.pop()] = i
    if stack:
        raise ValueError("unbalanced Rust source groups")
    masked_lines = masked.split("\n")
    if not loop_header(n, masked_lines[n]):
        return []
    opening = starts[n] + masked_lines[n].rfind("{")
    outer = next(i for i, token in enumerate(tokens) if token.start() == opening)
    stop = pairs[outer]

    def angle_end(i):
        depth = 0
        while i < stop:
            word = words[i]
            if word in ("(", "[", "{"):
                i = pairs[i] + 1
                continue
            if word == "<":
                depth += 1
            elif word == ">":
                depth -= 1
                if depth == 0:
                    return i + 1
            i += 1
        raise ValueError("unterminated Rust generic arguments")

    def loop_body(i):
        if words[i] == "loop":
            return i + 1 if words[i + 1] == "{" else None
        j = i + 1
        if words[i] == "for":
            if words[j] == "<":  # a higher-ranked type bound, not an expression loop
                return None
            while j < stop and words[j] != "in":
                j = pairs[j] + 1 if j in pairs else j + 1
            j += 1
        return header_body(j)

    def if_end(i):
        body = header_body(i + 1)
        if body is None:
            return stop
        j = pairs[body] + 1
        if j < stop and words[j] == "else":
            j += 1
            if words[j] == "if":
                return if_end(j)
            if words[j] == "{":
                return pairs[j] + 1
        return j

    def header_body(j):
        # Conditions/iterator operands can themselves be blocks, if/match/loop expressions or let chains.
        # Only an opening brace after a completed operand starts the loop's body. Balanced groups are atomic.
        operand = True
        while j < stop:
            word = words[j]
            if word == "{" and not operand:
                return j
            if word in ("(", "[", "{"):
                j, operand = pairs[j] + 1, False
                continue
            if word == "if" and operand:
                j, operand = if_end(j), False
                continue
            if word in ("for", "while", "loop") and operand:
                body = loop_body(j)
                if body is not None:
                    j, operand = pairs[body] + 1, False
                    continue
            if word == "match" and operand:
                body = header_body(j + 1)
                if body is not None:
                    j, operand = pairs[body] + 1, False
                    continue
            if word == "let":
                j += 1
                while j < stop and words[j] != "=":
                    j = pairs[j] + 1 if j in pairs else j + 1
                operand = True
            elif word == "<" and j > 0 and words[j - 1] == "::":
                j, operand = angle_end(j), False
                continue
            elif word in ("unsafe", "async", "const", "try", "move", "return", "break") and operand:
                pass
            elif word in ("+", "-", "*", "/", "%", "&", "|", "^", "!", "=", "==", "!=", "<", ">",
                          "<=", ">=", "&&", "||", "..", "..=", "as"):
                operand = True
            elif word not in (".", "::", "?", ","):
                operand = False
            j += 1
        return None

    def closure_end(i):
        contexts = {"=", "(", "[", "{", ",", "=>", "return", "break", "move", "async", "const", "&", "*", "!"}
        if words[i] not in ("|", "||") or (i and words[i - 1] not in contexts):
            return None
        j = i + 1
        if words[i] == "|":
            while j < stop and words[j] != "|":
                if words[j] in (";", "}"):
                    return None
                j = pairs[j] + 1 if j in pairs else j + 1
            j += 1
        if words[j:j + 2] == ["-", ">"]:
            j += 2
            while j < stop and words[j] != "{":
                j = angle_end(j) if words[j] == "<" else pairs[j] + 1 if j in pairs else j + 1
            if j < stop:
                return pairs[j] + 1
        # An expression closure extends to its containing delimiter; balanced body groups do not terminate it.
        while j < stop and words[j] not in (",", ";", ")", "]", "}"):
            j = angle_end(j) if words[j] == "<" and words[j - 1] == "::" else pairs[j] + 1 if j in pairs else j + 1
        return j

    excluded = []
    for i in range(outer + 1, stop):
        if words[i] in ("for", "while", "loop"):
            body = loop_body(i)
            if body is not None:
                excluded.append((tokens[i].start(), tokens[pairs[body]].end()))
        end = closure_end(i)
        if end is not None:
            excluded.append((tokens[i].start(), tokens[end].start()))
    targets = []
    for line in range(n + 1, len(lines)):
        if starts[line] >= tokens[stop].start():
            break
        match = re.match(r"^(\s*(?:\}\s*else\s+)?if\s+)(.+?)(\s*\{)\s*$", masked_lines[line])
        if match and not match.group(2).startswith("let "):
            position = starts[line] + len(match.group(1))
            if not any(a <= position < b for a, b in excluded):
                # Mask spaces at a literal's end are still source characters, including its closing quote.
                end = len(lines[line][:masked_lines[line].rfind("{")].rstrip())
                targets.append((line, (match.start(2), end)))
    return targets

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

def method_calls(names):
    """Dot-method spans through their closing parenthesis, excluding comments and literal contents."""
    masked = mask_rust("\n".join(lines))
    pairs, stack = {}, []
    for offset, char in enumerate(masked):
        if char == "(":
            stack.append(offset)
        elif char == ")":
            if not stack:
                raise ValueError("unbalanced Rust call parentheses")
            pairs[stack.pop()] = offset
    if stack:
        raise ValueError("unbalanced Rust call parentheses")
    for match in re.finditer(r"\.\s*(?:r#)?(\w+)\s*\(", masked):
        if match.group(1) in names:
            opening = match.end() - 1
            yield match.group(1), match.start(), opening, pairs[opening] + 1

def receiver_start(masked, offset):
    """Start of a postfix receiver: paths, fields, grouped operands, calls, indices, macros and generics."""
    tokens = list(re.finditer(r"r#\w+|'\w+|\w+|::|[^\s]", masked))
    words = [token.group() for token in tokens]
    pairs, stack = {}, []
    for i, word in enumerate(words):
        if word in ("(", "[", "{"):
            stack.append(i)
        elif word in (")", "]", "}"):
            if not stack or words[stack[-1]] != {")": "(", "]": "[", "}": "{"}[word]:
                raise ValueError("unbalanced Rust receiver groups")
            pairs[i] = stack.pop()
    if stack:
        raise ValueError("unbalanced Rust receiver groups")
    forbidden = {"return", "if", "else", "while", "for", "in", "let", "match", "break", "continue",
                 "fn", "pub", "impl", "where", "as", "move", "async", "unsafe", "const", "static",
                 "type", "struct", "enum", "use", "mod"}

    def primary(i):
        if i < 0:
            return None
        word = words[i]
        if word == "?":
            return primary(i - 1)
        if i in pairs:
            # A preceding atom is the callee/indexed value; otherwise this group is the receiver itself.
            begin = pairs[i]
            previous = begin - 1
            if previous >= 0 and words[previous] == "!":
                head = primary(previous - 1)
            else:
                head = primary(previous)
            if head is not None:
                begin = head
        elif word == ">":
            # Walk generic arguments backwards, skipping balanced groups inside their type expressions.
            depth, j = 1, i - 1
            while j >= 0:
                if j in pairs:
                    j = pairs[j] - 1
                    continue
                if words[j] == ">" and (j == 0 or words[j - 1] != "-"):
                    depth += 1
                elif words[j] == "<":
                    depth -= 1
                    if depth == 0:
                        break
                j -= 1
            if j < 0:
                return None
            begin = j
            if j > 0 and words[j - 1] == "::":
                head = primary(j - 2)
                if head is None:
                    return None
                begin = head
        elif re.fullmatch(r"(?:r#)?\w+", word) and word not in forbidden:
            begin = i
        else:
            return None
        if begin > 0 and words[begin - 1] in (".", "::"):
            head = primary(begin - 2)
            if head is not None:
                begin = head
            elif words[begin - 1] == "::":
                begin -= 1
            else:
                return None
        return begin

    last_token = bisect.bisect_left([token.start() for token in tokens], offset) - 1
    begin = primary(last_token)
    return tokens[begin].start() if begin is not None else None

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
    source = "\n".join(lines)
    starts = [0] + [match.end() for match in re.finditer("\n", source)]
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
            for rest in operand_deletions(cond, operands(cond)):
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
            for target, (begin, end) in guard_targets(n):
                condition = lines[target][begin:end]
                for position in positions:
                    rewritten_condition = "(%s) && %s" % (condition, position)
                    branch = lines[target][:begin] + rewritten_condition + lines[target][end:]
                    replacement = [rewritten + tail] + lines[n + 1:target] + [branch]
                    add(n, "guard", "\n".join(replacement), stop=target + 1)
    masked_source = "\n".join(masked)
    marked_source = mask_rust(source, literal_marks=True)
    for method, begin, opening, end in method_calls({"iter", "into_iter", "filter", "chain", "contains"}):
        n = bisect.bisect_right(starts, begin) - 1
        final = bisect.bisect_right(starts, end - 1) - 1
        if not first <= n <= final < last:
            continue
        if method == "contains":
            receiver = receiver_start(marked_source, begin)
            if receiver is None or not marked_source[opening + 1:end - 1].strip():
                continue
            n = bisect.bisect_right(starts, receiver) - 1
            if n < first:
                continue
            for endpoint in ("first", "last"):
                for copied in ("", ".copied()"):
                    replacement = "((%s).%s()%s == Some(%s))" % (
                        source[receiver:begin], endpoint, copied, source[opening + 1:end - 1])
                    text = lines[n][:receiver - starts[n]] + replacement + lines[final][end - starts[final]:]
                    add(n, "contains", text, stop=final + 1)
            continue
        if method in {"iter", "into_iter"} and masked_source[opening + 1:end - 1].strip():
            continue
        call = source[begin:end]
        replacements = [call + ".take(1)", call + ".rev().take(1)"] if method != "chain" else []
        if method in {"filter", "chain"}:
            replacements.append("/* %s dropped */" % method)
        for replacement in replacements:
            text = lines[n][:begin - starts[n]] + replacement + lines[final][end - starts[final]:]
            add(n, "chain", text, stop=final + 1)
    for begin, end, spans in closure_conditions(source):
        n = bisect.bisect_right(starts, begin) - 1
        if not first <= n < last:
            continue
        for replacement in operand_deletions(source[begin:end], spans):
            text = lines[n][:begin - starts[n]] + replacement + lines[n][end - starts[n]:]
            add(n, "clogic", text)
    return out

ms = [m for m in mutants() if not a.op or m[2] in a.op]
ids = ["L%d.%s.%d" % (n + 1, op, k) for k, (n, _, op, _) in enumerate(ms)]
unknown = sorted(set(a.only) - set(ids))
if unknown:
    print("mutation-sweep: unknown selected mutation ID(s): " + ", ".join(repr(i) for i in unknown), file=sys.stderr)
    sys.exit(2)
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
paths = [e.split("\t", 1)[1] for e in staged.split("\0") if e and not e.startswith("160000 ")]
for p in paths:
    if os.path.lexists(p) and not os.path.isdir(p):
        d = os.path.join(TREE, os.path.dirname(p)); os.makedirs(d, exist_ok=True)
        # A fresh mtime, not the original's: an older one than the last mutant's build would let cargo reuse it.
        shutil.copy(p, os.path.join(TREE, p), follow_symlinks=False)
target_file = os.path.join(TREE, a.file)
original = open(target_file, "rb").read()
env = dict(os.environ, CARGO_TARGET_DIR=TARGET, ARCHOGEN_SWEEP_SOURCE=ROOT)
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
    def finish(outcome, detail):
        return outcome, detail, time.time() - t0, (so, se, p.returncode)
    try:
        so, se = p.communicate(timeout=a.timeout)
    except subprocess.TimeoutExpired:
        os.killpg(p.pid, signal.SIGKILL)
        so, se = p.communicate()
        return finish("timeout", [])
    stdout, stderr = so.decode("utf-8", "replace"), se.decode("utf-8", "replace")
    if p.returncode == 0:
        return finish("survived", [])
    if "could not compile" in stderr and "test result" not in stdout:
        return finish("unviable", [next((x for x in stderr.splitlines() if x.startswith("error")), "")])
    failed = failed_tests(stdout)
    if failed and any(l.startswith("test result: FAILED.") for l in stdout.splitlines()):
        return finish("killed", failed)
    detail = next((l for l in stderr.splitlines() if l.strip()), "nonzero exit without a finishing, named test failure")
    return finish("unjudged", [detail])

def retain_baseline(outcome, detail, seconds, evidence):
    directory = os.path.join(WORK, "baseline-failures")
    os.makedirs(directory, exist_ok=True)
    receipt_dir = tempfile.mkdtemp(prefix="failure.", dir=directory)
    stdout, stderr, status = evidence
    receipt = dict(schema_version=1, source=a.file, source_sha256=hashlib.sha256(original).hexdigest(),
                   command=cmd, cwd=os.path.relpath(TREE, ROOT), outcome=outcome,
                   exit_status=status, seconds=seconds, detail=detail,
                   failed_tests=failed_tests(stdout.decode("utf-8", "replace")))
    for name, data in [("stdout", stdout), ("stderr", stderr)]:
        path = os.path.join(receipt_dir, name + ".bin")
        with open(path, "xb") as stream:
            stream.write(data)
        receipt[name] = dict(path=os.path.relpath(path, ROOT), bytes=len(data),
                             sha256=hashlib.sha256(data).hexdigest())
    path = os.path.join(receipt_dir, "receipt.json")
    with open(path, "x", encoding="utf-8") as stream:
        json.dump(receipt, stream, indent=2)
        stream.write("\n")
    return os.path.relpath(path, ROOT)

base, detail, secs, evidence = run()
if base != "survived":
    print("mutation-sweep: could not run — the unmutated copy does not pass (%s, %.0fs): %s" % (base, secs, " ".join(cmd)), file=sys.stderr)
    try:
        receipt = retain_baseline(base, detail, secs, evidence)
    except OSError as error:
        print("mutation-sweep: could not retain baseline evidence: %s" % error, file=sys.stderr)
    else:
        print("mutation-sweep: baseline evidence: " + receipt, file=sys.stderr)
    sys.exit(2)
print("mutation-sweep: the unmutated copy passes (%.0fs); %d mutation(s) of %s, lines %d to %d" % (secs, len(ms), a.file, first + 1, last))

counts, unlisted, seen_equiv, operator_counts = {}, [], set(), {}
try:
    for i, m in zip(ids, ms):
        if a.only and i not in a.only:
            continue
        mutated = apply_mutant(m)
        open(target_file, "w", encoding="utf-8").write(mutated)
        if open(target_file, encoding="utf-8").read() != mutated:
            print("mutation-sweep: %s did not land in %s" % (i, target_file), file=sys.stderr); sys.exit(2)
        try:
            outcome, detail, secs, _ = run()
        finally:
            open(target_file, "wb").write(original)
            if open(target_file, "rb").read() != original:
                print("mutation-sweep: %s did not read back after %s" % (target_file, i), file=sys.stderr); sys.exit(2)
        key = mutation_key(m)
        if outcome == "survived" and key in equiv:
            outcome = "equivalent"; seen_equiv.add(key)
        counts[outcome] = counts.get(outcome, 0) + 1
        attempted, unviable = operator_counts.get(m[2], (0, 0))
        operator_counts[m[2]] = (attempted + 1, unviable + (outcome == "unviable"))
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
print("mutation-sweep: unviable by operator: " + (
    ", ".join("%s=%d/%d" % (op, unviable, attempted)
              for op, (attempted, unviable) in sorted(operator_counts.items())) or "none attempted")
    + " (unviable/attempted)")
for i, (f, t) in unlisted:
    print("  survivor %s: %s ⟶ %s" % (i, display(f), display(t)), file=sys.stderr)
for f, t in stale:
    print("  a listed equivalent no longer made or no longer surviving: %s ⟶ %s" % (display(f), display(t)), file=sys.stderr)
sys.exit(1 if unlisted or stale else 0)
PY
