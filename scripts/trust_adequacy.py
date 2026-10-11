#!/usr/bin/env python3
"""Plan census-span measurements and compile bounded traces of actual non-progress edits."""
import argparse
import ast
import bisect
from collections import Counter
from concurrent.futures import ThreadPoolExecutor
from contextlib import contextmanager
import hashlib
import json
import os
from pathlib import Path
import re
import shutil
import signal
import stat
import subprocess
import sys
import time
import tempfile
import tomllib
import unittest
from unittest import mock

DECISION = "docs/specs/trust/decision_trust-generated-adequacy.md"
CATALOGUE = "xtask/mutations.txt"
SWEEP = "scripts/mutation_sweep.sh"
MARKER = "<!-- machine-read: parent-mutation-census -->"
POPULATIONS = {
    "route": ["-p", "xtask", "route_tests"],
    "parent": ["-p", "xtask", "--", "trust::", "trust_generated::", "trust_gate::", "--skip", "route_tests"],
}


def digest(data):
    return hashlib.sha256(data).hexdigest()


def census(text):
    if text.count(MARKER) != 1:
        raise ValueError("expected one census marker")
    table = text.split(MARKER, 1)[1].split("\n\n", 1)[0].strip().splitlines()
    if table[:2] != ["| Suffix | Source findings | Catalogue origin and file |", "| --- | --- | --- |"]:
        raise ValueError("unrecognized census header")
    rows = []
    for line in table[2:]:
        match = re.fullmatch(r"\| `([^`]+)` \| ([^|]+) \| (existing|added), `(trust(?:_generated)?\.rs)` \|", line)
        if not match:
            raise ValueError("malformed census row: " + line)
        suffix, findings, origin, filename = match.groups()
        rows.append(dict(suffix=suffix, findings=findings.strip(), origin=origin, file="xtask/src/" + filename))
    if not rows or len({r["suffix"] for r in rows}) != len(rows):
        raise ValueError("empty or duplicate census")
    return rows


def catalogue(text):
    """Read the public four-space block format; native catalogue contracts separately hold its full semantics."""
    lines = iter(text.splitlines())
    entries = {}

    def field(key):
        line = next(lines).rstrip()
        if not line.startswith(key + " "):
            raise ValueError("expected catalogue field " + key)
        return line[len(key) + 1:]

    def block(stop):
        body = []
        for line in lines:
            if line.rstrip() == stop:
                return "\n".join(body)
            if line and not line.startswith("    "):
                raise ValueError("catalogue block needs four-space indentation")
            body.append(line[4:] if line else "")
        raise ValueError("unterminated catalogue block")

    try:
        for line in lines:
            line = line.rstrip()
            if not line or line.startswith("#"):
                continue
            if not line.startswith("mutation "):
                raise ValueError("expected mutation identity")
            identity = line[len("mutation "):]
            if not identity or identity in entries:
                raise ValueError("empty or duplicate mutation identity")
            entry = dict(file=field("file"), test=field("test").split(), expect=field("expect"), why=field("why"))
            if next(lines).rstrip() != "from":
                raise ValueError("expected from block")
            entry["from"] = block("to")
            entry["to"] = block("end")
            if not entry["from"] or entry["from"] == entry["to"]:
                raise ValueError("empty or unchanged mutation")
            entries[identity] = entry
    except StopIteration as error:
        raise ValueError("incomplete catalogue entry") from error
    return entries


def bind_pairs(rows, entries):
    wanted = {p + "-" + r["suffix"] for r in rows for p in POPULATIONS}
    found = {i for i in entries if i.startswith(("route-", "parent-"))}
    if found != wanted:
        raise ValueError("census/population identity mismatch")
    for row in rows:
        twins = [entries[p + "-" + row["suffix"]] for p in POPULATIONS]
        for population, entry in zip(POPULATIONS, twins):
            if entry["test"] != POPULATIONS[population] or entry["expect"] != "killed":
                raise ValueError("wrong population command/verdict: " + row["suffix"])
            if entry["file"] != row["file"]:
                raise ValueError("wrong source file: " + row["suffix"])
        if any(twins[0][key] != twins[1][key] for key in ("file", "from", "to")):
            raise ValueError("exact twin drift: " + row["suffix"])


def source_span(source, original):
    starts = [m.start() for m in re.finditer(re.escape(original), source)]
    if len(starts) != 1:
        raise ValueError("exact original block must occur once")
    start, end = starts[0], starts[0] + len(original)
    lo = source.count("\n", 0, start)
    # Catalogue blocks can be inline fragments or include their last newline. Include exactly the lines touched
    # by their characters; a terminal newline does not also select the following, untouched line.
    return [lo, source.count("\n", 0, end - 1) + 1]


def overlaps(left, right):
    return left[0] < right[1] and right[0] < left[1]


def discover(script, source):
    """Load actual discovery functions, not a second implementation of the operators."""
    python = script.split("<<'PY'\n", 1)[1].rsplit("\nPY", 1)[0]
    functions = [n for n in ast.parse(python).body if isinstance(n, ast.FunctionDef)]
    lines = source.split("\n")
    try:
        first = next(i for i, line in enumerate(lines) if re.match(r"\s*(pub(\(crate\))? )?fn ", line))
        last = next(i for i, line in enumerate(lines) if line.strip() == "#[cfg(test)]")
    except StopIteration as error:
        raise ValueError("source lacks production range") from error
    if first >= last:
        raise ValueError("source has no production function before tests")
    namespace = dict(re=re, json=json, bisect=bisect, lines=lines, first=first, last=last)
    exec(compile(ast.Module(body=functions, type_ignores=[]), SWEEP, "exec"), namespace)
    candidates, listing = [], []
    for index, mutation in enumerate(namespace["mutants"]()):
        lo, hi, operator, replacement = mutation
        identity = "L%d.%s.%d" % (lo + 1, operator, index)
        before, after = namespace["mutation_key"](mutation)
        candidates.append(dict(id=identity, span=[lo, hi], operator=operator,
                               original="\n".join(lines[lo:hi]), replacement=replacement))
        listing.append("%-18s %s\n%18s ⟶ %s" % (identity, namespace["display"](before), "", namespace["display"](after)))
    return candidates, listing, [first, last]


def verify_listing(actual, listing, filename, production_range):
    first, last = production_range
    footer = "mutation-sweep: %d mutation(s) of %s, lines %d to %d" % (len(listing), filename, first + 1, last)
    if actual != "\n".join(listing + [footer]) + "\n":
        raise ValueError("actual sweep CLI listing differs from loaded candidates")


def plan(root):
    inputs = {p: (root / p).read_bytes() for p in (DECISION, CATALOGUE, SWEEP)}
    rows, entries = census(inputs[DECISION].decode()), catalogue(inputs[CATALOGUE].decode())
    if len(rows) != 32:
        raise ValueError("accepted census must have 32 rows; run native adequacy contracts")
    bind_pairs(rows, entries)
    files = {}
    for filename in sorted({r["file"] for r in rows}):
        inputs[filename] = (root / filename).read_bytes()
        source = inputs[filename].decode()
        candidates, listing, production_range = discover(inputs[SWEEP].decode(), source)
        result = subprocess.run(["bash", SWEEP, filename, "--package", "xtask", "--list"], cwd=root,
                                capture_output=True, text=True, check=True, timeout=60)
        verify_listing(result.stdout, listing, filename, production_range)
        selected = set()
        for row in (r for r in rows if r["file"] == filename):
            entry = entries["route-" + row["suffix"]]
            row["span"] = source_span(source, entry["from"])
            if not production_range[0] <= row["span"][0] < row["span"][1] <= production_range[1]:
                raise ValueError("census span outside production range: " + row["suffix"])
            row["candidates"] = [m["id"] for m in candidates if overlaps(row["span"], m["span"])]
            row["exact_original"] = entry["from"]
            row["exact_replacement"] = entry["to"]
            selected.update(row["candidates"])
        files[filename] = dict(production_range=production_range, listed_count=len(candidates),
                               candidates=[m for m in candidates if m["id"] in selected], cli_listing_verified=True)
    for filename, data in inputs.items():
        if (root / filename).read_bytes() != data:
            raise ValueError("input changed during planning: " + filename)
    return dict(schema="archogen-trust-adequacy-plan/1", scope="overlap planning, no measured semantic witness",
                span_convention="zero-based half-open enclosing source lines; includes unchanged exact-entry context",
                input_sha256={p: digest(data) for p, data in inputs.items()}, populations=POPULATIONS,
                rows=rows, files=files)


def display(text):
    return json.dumps(text, ensure_ascii=False) if "\n" in text else text


def selected_equivalents(file_plan):
    """Bind explicit native-file snapshots to selected text; this does not prove their reasons true."""
    records = file_plan.get("equivalents", [])
    if not isinstance(records, list):
        raise ValueError("equivalents must be a list of selected reason records")
    selected = {(c["original"].strip(), c["replacement"].strip()) for c in file_plan["candidates"]}
    result = {}
    for record in records:
        if not isinstance(record, dict) or set(record) != {"original", "replacement", "reason"}:
            raise ValueError("equivalent requires exactly original/replacement/reason")
        if any(not isinstance(value, str) or not value.strip() for value in record.values()):
            raise ValueError("equivalent fields must be nonempty strings")
        key = record["original"].strip(), record["replacement"].strip()
        if "\n" in record["reason"] or "\r" in record["reason"]:
            raise ValueError("equivalent reason must occupy one line")
        if key not in selected or key in result:
            raise ValueError("unmatched or duplicate selected equivalent")
        result[key] = record["reason"].strip()
    return result


def audit_measurement(log, file_plan, filename, exit_code):
    """Require every selected actual replacement, named finishing kill and native terminal count."""
    candidates = {m["id"]: m for m in file_plan["candidates"]}
    equivalents = selected_equivalents(file_plan)
    records = {}
    first, last = file_plan["production_range"]
    baseline = (r"mutation-sweep: the unmutated copy passes \([0-9]+s\); "
                + str(file_plan["listed_count"]) + r" mutation\(s\) of " + re.escape(filename)
                + r", lines " + str(first + 1) + " to " + str(last))
    if len(re.findall("^" + baseline + "$", log, re.MULTILINE)) != 1:
        raise ValueError("missing or wrong native baseline/scope")
    marks = dict(killed="✓", survived="✗", unviable="·", timeout="✗", unjudged="?", equivalent="=")
    for line in log.splitlines():
        match = re.match(r"^  ([✓✗·?=]) (\S+) +(\w+) +", line)
        if not match:
            continue
        mark, identity, outcome = match.groups()
        if identity not in candidates or identity in records or outcome not in marks or mark != marks[outcome]:
            raise ValueError("foreign, duplicate or invalid native result")
        candidate = candidates[identity]
        expected = "  %s %-18s %-10s %s ⟶ %s" % (
            mark, identity, outcome, display(candidate["original"].strip()), display(candidate["replacement"].strip()))
        if not line.startswith(expected):
            raise ValueError("native replacement text drift: " + identity)
        detail = line[len(expected):]
        key = candidate["original"].strip(), candidate["replacement"].strip()
        if key in equivalents and outcome != "equivalent":
            raise ValueError("listed equivalent did not survive: " + identity)
        names = []
        if outcome == "killed":
            if not re.fullmatch(r" — [\w:]+(?:, [\w:]+)*", detail):
                raise ValueError("native kill lacks finishing test names: " + identity)
            names = detail[3:].split(", ")
        elif outcome == "equivalent":
            if key not in equivalents or detail:
                raise ValueError("unlisted or detailed native equivalent: " + identity)
        elif outcome in ("survived", "timeout") and detail:
            raise ValueError("unexpected native outcome detail: " + identity)
        records[identity] = dict(id=identity, outcome=outcome, names=names, detail=detail)
        if outcome == "equivalent":
            records[identity]["reason"] = equivalents[key]
    if records.keys() != candidates.keys():
        raise ValueError("native result identities incomplete")
    counts = Counter(r["outcome"] for r in records.values())
    if counts["unjudged"]:
        raise ValueError("native run is unjudged")
    summary = "mutation-sweep: " + ", ".join("%d %s" % (count, outcome) for outcome, count in sorted(counts.items()))
    operators = {}
    for identity, record in records.items():
        op = candidates[identity]["operator"]
        attempted, unviable = operators.get(op, (0, 0))
        operators[op] = (attempted + 1, unviable + (record["outcome"] == "unviable"))
    operator_summary = "mutation-sweep: unviable by operator: " + ", ".join(
        "%s=%d/%d" % (op, unviable, attempted) for op, (attempted, unviable) in sorted(operators.items())) + " (unviable/attempted)"
    for expected in (summary, operator_summary):
        if log.splitlines().count(expected) != 1:
            raise ValueError("native terminal counts disagree")
    if exit_code != (1 if counts["survived"] or counts["timeout"] else 0):
        raise ValueError("native exit disagrees with strict measured outcomes")
    return dict(entries=list(records.values()), counts=dict(counts), operator_counts={
        op: dict(attempted=a, unviable=u) for op, (a, u) in operators.items()}, complete_identities=True,
        baseline_passed=True, raw_exit_code=exit_code)


PROGRESS_CASES = [
    ("            i += if *takes { 2 } else { 1 };", "            // i += if *takes { 2 } else { 1 };",
     ["rustc", "--out-dir", "out"], None),
    ("            i += 1;", "            i += 0;", ["rustc", "--error-format=json"], None),
    ("            i += 1;", "            // i += 1;", ["rustc", "--error-format=json"], None),
    ("                    i += 2;", "                    // i += 2;", ["rustc", "-C", "metadata=aaa"], None),
    ("                i += 2;", "                // i += 2;", ["rustc", "-C", "opt-level=3"],
     'out.push(format!("-C {v}"));'),
]


def progress_cases(source, candidates):
    """Bind source/replacement pairs to actual discovery IDs, allowing unrelated line-number changes."""
    first = source[:source.index("fn normalise(")].count("\n")
    last = source[:source.index("/// `path` resolved")].count("\n")
    lines = source.split("\n")
    result = {}
    for original, replacement, arguments, previous in PROGRESS_CASES:
        matches = [candidate for candidate in candidates
                   if first <= candidate["span"][0] < candidate["span"][1] <= last
                   and (candidate["original"], candidate["replacement"]) == (original, replacement)
                   and (previous is None or lines[candidate["span"][0] - 1].strip() == previous)]
        if len(matches) != 1:
            raise ValueError("progress candidate identities missing or ambiguous")
        result[matches[0]["id"]] = arguments
    return result


def progress_program(source, candidates, false_positive=False, false_negative=False):
    """Observe actual index edits, stopping before a fourth body evaluation can consume resources."""
    by_id = {candidate["id"]: candidate for candidate in candidates}
    cases = progress_cases(source, candidates)
    lines = source.split("\n")
    constants = source[source.index("const DROPPED:"):source.index("/// A unit's configuration, normalised")]

    def traced(text, name):
        body = text[text.index("fn normalise("):text.index("/// `path` resolved")]
        signature = "-> Vec<String> {"
        loop = "    while i < tokens.len() {\n"
        tail = "    out\n}"
        if any(body.count(part) != 1 for part in (signature, loop, tail)):
            raise ValueError("normalise trace landmarks changed")
        return body.replace("fn normalise(", "fn " + name + "(", 1).replace(
            signature, "-> (Vec<String>, Vec<usize>) {", 1).replace(
            loop, "    let mut visited = Vec::new();\n" + loop
            + "        visited.push(i);\n        if visited.len() == 4 { return (out, visited); }\n", 1).replace(
            tail, "    (out, visited)\n}", 1)

    functions = [traced(source, "baseline")]
    checks = []
    for index, (identity, arguments) in enumerate(cases.items()):
        candidate = by_id[identity]
        lo, hi = candidate["span"]
        if hi - lo != 1:
            raise ValueError("progress candidate outside normalise")
        if "\n".join(lines[lo:hi]) != candidate["original"]:
            raise ValueError("progress candidate source drift")
        mutated = "\n".join(lines[:lo] + [candidate["replacement"]] + lines[hi:])
        name = "mutant_" + str(index)
        functions.append(traced(source if false_positive and index == 0 else mutated, name))
        args = ", ".join(json.dumps(arg) for arg in arguments)
        expected = "vec![2; 4]" if false_negative and index == 0 else "vec![1; 4]"
        checks.append('''let tokens = [%s].map(str::to_owned);
let original = baseline(&tokens, "scratch/");
let changed = %s(&tokens, "scratch/");
assert_eq!(original.1, vec![1], "%s baseline");
assert_eq!(changed.1, %s, "%s actual non-progress");
println!("%s: baseline indices {:?}; mutant indices {:?}; mutant values {:?}", original.1, changed.1, changed.0);'''
                      % (args, name, identity, expected, identity, identity))
    return constants + "\n".join(functions) + "\nfn main() {\n" + "\n".join(checks) + "\n}\n"


def progress_control(root, output):
    filename = "xtask/src/trust.rs"
    source = (root / filename).read_text()
    candidates, listing, interval = discover((root / SWEEP).read_text(), source)
    cases = progress_cases(source, candidates)
    actual = subprocess.check_output(["bash", SWEEP, filename, "--package", "xtask", "--list"], cwd=root, text=True)
    verify_listing(actual, listing, filename, interval)
    edition = tomllib.loads((root / "Cargo.toml").read_text())["workspace"]["package"]["edition"]
    directory = output.parent / (output.stem + "-controls")
    directory.mkdir(parents=True, exist_ok=True)
    env = dict(os.environ, CARGO_HOME=str(root / "target/cargo-home"), TMPDIR=str(root / "target/tmp"))
    runs = []
    for name, options, expected in (("actual", {}, 0), ("false-positive", dict(false_positive=True), 101),
                                    ("false-negative", dict(false_negative=True), 101)):
        code, binary = directory / (name + ".rs"), directory / name
        code.write_text(progress_program(source, candidates, **options))
        compiled = subprocess.run(["rustc", "--edition=" + edition, str(code), "-o", str(binary)],
                                  cwd=root, env=env, text=True, capture_output=True, timeout=60)
        if compiled.returncode:
            raise ValueError("progress control did not compile: " + compiled.stderr)
        run = subprocess.run([str(binary)], cwd=root, env=env, text=True, capture_output=True, timeout=5)
        if run.returncode != expected:
            raise ValueError("progress control exit disagrees: " + name)
        if name != "actual" and next(iter(cases)) + " actual non-progress" not in run.stderr:
            raise ValueError("wrong oracle did not fail at its intended assertion: " + name)
        if name == "actual" and run.stdout.splitlines() != [
                identity + ": baseline indices [1]; mutant indices [1, 1, 1, 1]; mutant values "
                + ('["-C opt-level=3", "-C opt-level=3", "-C opt-level=3"]' if arguments[-1] == "opt-level=3" else "[]")
                for identity, arguments in cases.items()]:
            raise ValueError("actual progress traces differ")
        runs.append(dict(name=name, source_sha256=digest(code.read_bytes()), exit_code=run.returncode,
                         stdout=run.stdout, stderr=run.stderr))
    result = dict(schema="archogen-trust-progress/1", cases=list(cases), runs=runs,
                  scope="instrumented index traces of five actual edits; no OS crash classification",
                  production_range=interval, listed_count=len(candidates), cli_listing_verified=True,
                  input_sha256={path: digest((root / path).read_bytes()) for path in (filename, SWEEP, "Cargo.toml")},
                  producer_sha256=digest(Path(__file__).read_bytes()))
    output.write_text(json.dumps(result, indent=2) + "\n")
    return result


def local_output(root, path):
    output = Path(path)
    if output.is_absolute() or not (root / output).resolve().is_relative_to(root / "target"):
        raise ValueError("output must remain under this repository's target/")
    return root / output


def relative_input_path(value):
    if not isinstance(value, str) or not value or "\0" in value or any(
            part in ("", ".", "..") for part in value.split("/")):
        raise ValueError("input path must be canonical and repository-relative")
    return value


def census_data_path(root, value, output=False):
    """Reject symlinks and other volumes before reading or creating project-owned data."""
    parts = relative_input_path(value).split("/")
    if output and (parts[0] != "target" or len(parts) < 2):
        raise ValueError("census data must remain under this repository's target/")
    if output and ".git" in parts:
        raise ValueError("census data must not write Git metadata")
    device = root.stat().st_dev
    path = root
    for index, part in enumerate(parts):
        path = path / part
        try:
            info = path.lstat()
        except FileNotFoundError:
            # No later component can already exist below a missing ancestor.
            return root / value
        if stat.S_ISLNK(info.st_mode) or info.st_dev != device:
            raise ValueError("symlink or other-volume census path: " + repr(value))
        if output and stat.S_ISDIR(info.st_mode) and os.path.lexists(path / ".git"):
            raise ValueError("census data must not cross a nested Git boundary")
        if index < len(parts) - 1 and not stat.S_ISDIR(info.st_mode):
            raise ValueError("census path ancestor is not a directory: " + repr(value))
    return path


def index_inputs(root):
    """Read only the public Git index; a gitlink's contents are never opened."""
    top = subprocess.check_output(["git", "rev-parse", "--show-toplevel"], cwd=root, text=True).removesuffix("\n")
    if Path(top).resolve() != root.resolve():
        raise ValueError("census root is not its own Git worktree")
    raw = subprocess.check_output(["git", "ls-files", "--stage", "-z"], cwd=root)
    if not raw or not raw.endswith(b"\0"):
        raise ValueError("empty or unterminated input index")
    regular, gitlinks = {}, {}
    for record in raw[:-1].split(b"\0"):
        metadata, separator, name = record.partition(b"\t")
        match = re.fullmatch(rb"(100644|100755|160000) ([0-9a-f]{40}|[0-9a-f]{64}) 0", metadata)
        if not separator or not match:
            raise ValueError("unsupported mode or unmerged input index")
        name = relative_input_path(name.decode("utf-8"))
        if name in regular or name in gitlinks:
            raise ValueError("duplicate input index path")
        mode, oid = (field.decode("ascii") for field in match.groups())
        if mode == "160000":
            gitlinks[name] = oid
        else:
            regular[name] = dict(index_mode=mode, index_oid=oid)
    if not regular:
        raise ValueError("input index has no regular files")
    commit = subprocess.check_output(["git", "rev-parse", "--verify", "HEAD"], cwd=root, text=True).strip()
    if not re.fullmatch(r"[0-9a-f]{40}|[0-9a-f]{64}", commit):
        raise ValueError("unrecognized input commit identity")
    return dict(source_commit=commit, regular=regular, gitlinks=gitlinks)


def system_sha256(path):
    # Hash stdin, so filenames containing tabs, newlines, backslashes or Unicode are not parsed as output.
    if shutil.which("sha256sum"):
        command = ["sha256sum"]
    elif shutil.which("shasum"):
        command = ["shasum", "-a", "256"]
    else:
        raise FileNotFoundError("independent system SHA-256 requires sha256sum or shasum on PATH")
    with path.open("rb") as source:
        result = subprocess.run(command, stdin=source, capture_output=True, check=True, timeout=30)
    match = re.fullmatch(rb"([0-9a-f]{64}) [ *]-\r?\n", result.stdout)
    if not match:
        raise ValueError("unrecognized system SHA-256 output")
    return match[1].decode("ascii")


def regular_input(root, item):
    name, metadata = item
    path = census_data_path(root, name)
    before = path.lstat()
    if not stat.S_ISREG(before.st_mode):
        raise ValueError("tracked input is not a regular file: " + repr(name))
    data = path.read_bytes()
    python_hash = digest(data)
    if system_sha256(path) != python_hash:
        raise ValueError("Python/system input hashes differ: " + repr(name))
    after = path.lstat()
    stable = lambda info: (info.st_dev, info.st_ino, info.st_mode, info.st_size, info.st_mtime_ns, info.st_ctime_ns)
    if stable(before) != stable(after) or len(data) != before.st_size:
        raise ValueError("input changed while being hashed: " + repr(name))
    return name, dict(metadata, mode=stat.S_IMODE(before.st_mode), size=len(data), sha256=python_hash)


def tracked_inputs(root):
    before = index_inputs(root)
    with ThreadPoolExecutor(max_workers=4) as pool:
        regular = dict(pool.map(lambda item: regular_input(root, item), before["regular"].items()))
    if index_inputs(root) != before:
        raise ValueError("input index or commit changed during census")
    return dict(schema="archogen-tracked-inputs/1", source_commit=before["source_commit"],
                producer_sha256=digest(Path(__file__).read_bytes()), regular=regular, gitlinks=before["gitlinks"])


def unique_json_fields(pairs):
    result = {}
    for key, value in pairs:
        if key in result:
            raise ValueError("duplicate snapshot JSON field: " + repr(key))
        result[key] = value
    return result


def load_input_snapshot(raw):
    data = json.loads(raw, object_pairs_hook=unique_json_fields)
    if not isinstance(data, dict) or set(data) != {
            "schema", "source_commit", "producer_sha256", "regular", "gitlinks"}:
        raise ValueError("unrecognized tracked-input snapshot fields")
    def hexadecimal(value, pattern):
        return isinstance(value, str) and re.fullmatch(pattern, value) is not None
    if data["schema"] != "archogen-tracked-inputs/1" or not hexadecimal(
            data["source_commit"], r"[0-9a-f]{40}|[0-9a-f]{64}") or not hexadecimal(
            data["producer_sha256"], r"[0-9a-f]{64}"):
        raise ValueError("unrecognized tracked-input snapshot identity")
    if not isinstance(data["regular"], dict) or not data["regular"] or not isinstance(data["gitlinks"], dict):
        raise ValueError("snapshot requires regular input and gitlink maps")
    for name, entry in data["regular"].items():
        relative_input_path(name)
        if not isinstance(entry, dict) or set(entry) != {"index_mode", "index_oid", "mode", "size", "sha256"}:
            raise ValueError("unrecognized regular input fields")
        if entry["index_mode"] not in ("100644", "100755") or not hexadecimal(
                entry["index_oid"], r"[0-9a-f]{40}|[0-9a-f]{64}") or not hexadecimal(entry["sha256"], r"[0-9a-f]{64}"):
            raise ValueError("invalid regular input identity")
        if type(entry["size"]) is not int or entry["size"] < 0 or type(entry["mode"]) is not int or not (
                0 <= entry["mode"] <= 0o7777):
            raise ValueError("invalid regular input size/mode")
    for name, oid in data["gitlinks"].items():
        relative_input_path(name)
        if name in data["regular"] or not hexadecimal(oid, r"[0-9a-f]{40}|[0-9a-f]{64}"):
            raise ValueError("invalid or overlapping gitlink identity")
    return data


def compare_input_snapshot(root, expected):
    actual = tracked_inputs(root)
    for kind in ("regular", "gitlinks"):
        for name in sorted(set(expected[kind]) | set(actual[kind])):
            if expected[kind].get(name) != actual[kind].get(name):
                raise ValueError("tracked input differs: " + repr(name))
    if actual != expected:
        raise ValueError("snapshot commit or producer differs")
    return actual


def input_census_command(root, output_name, snapshot_name=None):
    output = census_data_path(root, output_name, output=True)
    if output.exists() and not stat.S_ISREG(output.lstat().st_mode):
        raise ValueError("census output must be a regular file")
    if snapshot_name is not None:
        relative_input_path(snapshot_name)
        snapshot_path = census_data_path(root, snapshot_name, output=True)
        if snapshot_path == output or (snapshot_path.exists() and output.exists() and snapshot_path.samefile(output)):
            raise ValueError("verification result must not overwrite its input snapshot")
    indexed = index_inputs(root)
    if output_name in indexed["regular"]:
        raise ValueError("census output must not overwrite a tracked input")
    if any(output_name == name or output_name.startswith(name + "/") for name in indexed["gitlinks"]):
        raise ValueError("census output must not enter a gitlink")
    output.parent.mkdir(parents=True, exist_ok=True)
    result = dict(schema="archogen-tracked-input-check/1", status="pending", snapshot=snapshot_name)
    def save(data):
        # Replace our directory entry atomically; truncating an existing hardlink could alter another input.
        temporary = None
        try:
            with tempfile.NamedTemporaryFile(mode="w", encoding="utf-8", dir=output.parent,
                                             prefix=output.name + ".", delete=False) as stream:
                temporary = Path(stream.name)
                stream.write(json.dumps(data, indent=2, ensure_ascii=False) + "\n")
            temporary.replace(output)
        finally:
            if temporary is not None:
                temporary.unlink(missing_ok=True)
    save(result)  # A later refusal must not leave a previous successful result in place.
    try:
        if snapshot_name is None:
            data = tracked_inputs(root)
        else:
            path = census_data_path(root, snapshot_name, output=True)
            if not stat.S_ISREG(path.lstat().st_mode):
                raise ValueError("census snapshot must be a regular file")
            raw = path.read_bytes()
            result["snapshot_sha256"] = digest(raw)
            expected = load_input_snapshot(raw)
            data = compare_input_snapshot(root, expected)
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        save(dict(result, status="refused", error=str(error).replace(str(root) + os.sep, "")))
        raise
    if snapshot_name is None:
        save(data)
    else:
        save(dict(result, status="restored", regular_count=len(data["regular"]), gitlink_count=len(data["gitlinks"]),
                  source_commit=data["source_commit"], producer_sha256=data["producer_sha256"]))
    print("trust-adequacy: %s %d regular inputs; Python/system hashes agree; %d gitlink identities, metadata only" % (
        "captured" if snapshot_name is None else "restored", len(data["regular"]), len(data["gitlinks"])))


def validate_inputs(root, hashes):
    expected_paths = {DECISION, CATALOGUE, SWEEP, "xtask/src/trust.rs", "xtask/src/trust_generated.rs"}
    if not isinstance(hashes, dict) or set(hashes) != expected_paths:
        raise ValueError("unrecognized plan input paths")
    for path, expected in hashes.items():
        if not isinstance(expected, str) or not re.fullmatch(r"[0-9a-f]{64}", expected):
            raise ValueError("invalid plan input digest: " + path)
        if digest((root / path).read_bytes()) != expected:
            raise ValueError("stale plan input: " + path)


def measure(root, plan_path, filename, population, output):
    manifest = json.loads((root / plan_path).read_text())
    if manifest.get("schema") != "archogen-trust-adequacy-plan/1" or manifest.get("populations") != POPULATIONS:
        raise ValueError("unrecognized measurement plan/populations")
    validate_inputs(root, manifest["input_sha256"])
    if manifest != plan(root):
        raise ValueError("measurement manifest differs from actual complete candidate plan")
    file_plan = manifest["files"][filename]
    command = ["bash", SWEEP, filename, "--package", "xtask", "--timeout", "180", "--timeouts-fail"]
    filters = ["route_tests"] if population == "route" else POPULATIONS["parent"][3:]
    command += ["--filter=" + item for item in filters]
    for candidate in file_plan["candidates"]:
        command += ["--only", candidate["id"]]
    output.parent.mkdir(parents=True, exist_ok=True)
    logfile = output.with_suffix(".log")
    local_output(root, str(logfile.relative_to(root)))
    if output.resolve() == (root / plan_path).resolve() or logfile.resolve() == (root / plan_path).resolve():
        raise ValueError("result/log must not overwrite its plan")
    result = dict(schema="archogen-trust-adequacy-measurement/1", status="running", file=filename,
                  population=population, plan=plan_path, plan_sha256=digest((root / plan_path).read_bytes()),
                  command=command, log=str(logfile.relative_to(root)), input_sha256=manifest["input_sha256"],
                  producer_sha256=digest(Path(__file__).read_bytes()),
                  source_commit=subprocess.check_output(["git", "rev-parse", "HEAD"], cwd=root, text=True).strip())
    def save():
        output.write_text(json.dumps(result, indent=2) + "\n")
    save()
    original = (root / filename).read_bytes()
    started = time.monotonic()
    env = dict(os.environ, CARGO_HOME=str(root / "target/cargo-home"), TMPDIR=str(root / "target/tmp"))
    with logfile.open("w") as log:
        process = subprocess.Popen(command, cwd=root, env=env, stdout=log, stderr=subprocess.STDOUT, start_new_session=True)
        try:
            exit_code = process.wait()
        except BaseException:
            try:
                os.killpg(process.pid, signal.SIGKILL)
            except ProcessLookupError:
                pass
            process.wait()
            copied = root / "target/mutation-sweep/tree" / filename
            if copied.exists():
                copied.write_bytes(original)
            result.update(status="interrupted", elapsed_seconds=time.monotonic() - started)
            save()
            raise
    result.update(raw_exit_code=exit_code, elapsed_seconds=time.monotonic() - started, status="audit-pending")
    save()
    try:
        result.update(audit_measurement(logfile.read_text(), file_plan, filename, exit_code))
        for path, expected in manifest["input_sha256"].items():
            if digest((root / path).read_bytes()) != expected:
                raise ValueError("measurement changed source input: " + path)
        if (root / "target/mutation-sweep/tree" / filename).read_bytes() != original:
            raise ValueError("native copy did not restore")
        result.update(status="measured", restored=True, copied_sha256=digest(original))
    except (OSError, ValueError) as error:
        result.update(status="audit-failed", error=str(error))
        save()
        raise
    save()
    print("trust-adequacy: %s %s measured; %s; raw exit %d; complete identities and restored source" % (
        filename, population, result["counts"], exit_code))


@contextmanager
def input_census_fixture():
    parent = Path(__file__).resolve().parent.parent
    folder = census_data_path(parent, "target/tracked-input-controls", output=True)
    folder.mkdir(parents=True, exist_ok=True)
    with tempfile.TemporaryDirectory(prefix="case-", dir=folder) as temporary:
        root = Path(temporary)
        def git(*arguments):
            return subprocess.check_output(["git", "-C", str(root), *arguments], stderr=subprocess.STDOUT)
        git("init", "-q")
        (root / "nested").mkdir()
        names = ["plain.txt", "nested/input.bin", "space name", "line\nname", "tab\tname", "back\\slash", "quote'\"", "é"]
        for name in names:
            (root / name).write_bytes(b"fixture\0bytes\n")
        git("add", "--", *names)
        git("update-index", "--add", "--cacheinfo", "160000," + "1" * 40 + ",opaque")
        # Even a broken symlink at the gitlink is opaque; the census must not traverse it.
        (root / "opaque").symlink_to("absent-submodule")
        git("-c", "user.name=Input controls", "-c", "user.email=input-controls@example.invalid",
            "-c", "commit.gpgsign=false", "-c", "core.hooksPath=disabled-hooks", "commit", "-qm", "fixture")
        yield root, git


class Controls(unittest.TestCase):
    def test_input_census_never_writes_tracked_inputs_gitlinks_or_nested_git_metadata(self):
        with input_census_fixture() as (root, git):
            (root / "target").mkdir()
            path = root / "target/indexed"
            path.write_bytes(b"tracked input")
            git("add", "--", "target/indexed")
            with self.assertRaisesRegex(ValueError, "overwrite a tracked"):
                input_census_command(root, "target/indexed")
            self.assertEqual(path.read_bytes(), b"tracked input")
            git("update-index", "--add", "--cacheinfo", "160000," + "1" * 40 + ",target/pin")
            with self.assertRaisesRegex(ValueError, "enter a gitlink"):
                input_census_command(root, "target/pin/result.json")
            self.assertFalse((root / "target/pin").exists())
            (root / "target/other").mkdir()
            (root / "target/other/.git").write_text("synthetic Git boundary marker")
            with self.assertRaisesRegex(ValueError, "nested Git boundary"):
                input_census_command(root, "target/other/result.json")
            self.assertFalse((root / "target/other/result.json").exists())
            with self.assertRaisesRegex(ValueError, "Git metadata"):
                input_census_command(root, "target/.git")

    def test_system_hash_uses_available_tools_and_refuses_absence(self):
        with input_census_fixture() as (root, _):
            path = root / "plain.txt"
            installed = {name: shutil.which(name) for name in ("sha256sum", "shasum")}
            self.assertTrue(any(installed.values()), "no independent system SHA-256 available")
            for name, program in installed.items():
                if program is None:
                    continue
                with mock.patch.object(shutil, "which", side_effect=lambda value: installed[value] if value == name else None):
                    for raw in (b"abc", b"contrasting bytes\0\n"):
                        path.write_bytes(raw)
                        self.assertEqual(system_sha256(path), digest(raw))
            with mock.patch.object(shutil, "which", return_value=None):
                with self.assertRaisesRegex(FileNotFoundError, "requires sha256sum or shasum"):
                    system_sha256(path)

    def test_input_census_round_trip_preserves_special_names_and_opaque_gitlinks(self):
        with input_census_fixture() as (root, _):
            snapshot = tracked_inputs(root)
            self.assertEqual(set(snapshot["regular"]), {
                "plain.txt", "nested/input.bin", "space name", "line\nname", "tab\tname", "back\\slash", "quote'\"", "é"})
            self.assertEqual(snapshot["gitlinks"], {"opaque": "1" * 40})
            self.assertEqual(load_input_snapshot(json.dumps(snapshot)), snapshot)
            self.assertEqual(compare_input_snapshot(root, snapshot), snapshot)

    def test_input_census_refuses_changed_missing_added_index_modes_and_pins(self):
        with input_census_fixture() as (root, git):
            snapshot = tracked_inputs(root)
            path = root / "plain.txt"
            original = path.read_bytes()
            path.write_bytes(original + b"changed")
            with self.assertRaisesRegex(ValueError, "tracked input differs.*plain"):
                compare_input_snapshot(root, snapshot)
            path.unlink()
            with self.assertRaises(FileNotFoundError):
                compare_input_snapshot(root, snapshot)
            path.write_bytes(original)
            path.chmod(snapshot["regular"]["plain.txt"]["mode"] ^ stat.S_IXUSR)
            with self.assertRaisesRegex(ValueError, "tracked input differs.*plain"):
                compare_input_snapshot(root, snapshot)
            path.chmod(snapshot["regular"]["plain.txt"]["mode"])
            git("update-index", "--chmod=+x", "--", "plain.txt")
            with self.assertRaisesRegex(ValueError, "tracked input differs.*plain"):
                compare_input_snapshot(root, snapshot)
            git("update-index", "--chmod=-x", "--", "plain.txt")
            (root / "added").write_bytes(b"new")
            git("add", "--", "added")
            with self.assertRaisesRegex(ValueError, "tracked input differs.*added"):
                compare_input_snapshot(root, snapshot)
            git("rm", "--cached", "--", "added")
            path.write_bytes(b"new index")
            git("add", "--", "plain.txt")
            path.write_bytes(original)
            with self.assertRaisesRegex(ValueError, "tracked input differs.*plain"):
                compare_input_snapshot(root, snapshot)
            git("add", "--", "plain.txt")
            git("update-index", "--cacheinfo", "160000," + "2" * 40 + ",opaque")
            with self.assertRaisesRegex(ValueError, "tracked input differs.*opaque"):
                compare_input_snapshot(root, snapshot)
            git("update-index", "--cacheinfo", "160000," + "1" * 40 + ",opaque")
            self.assertEqual(compare_input_snapshot(root, snapshot), snapshot)

    def test_input_census_refuses_unmerged_and_symlinked_regular_inputs(self):
        with input_census_fixture() as (root, git):
            path = root / "plain.txt"
            path.unlink()
            path.symlink_to("nested/input.bin")
            with self.assertRaisesRegex(ValueError, "symlink"):
                tracked_inputs(root)
            path.unlink()
            path.write_bytes(b"fixture\0bytes\n")
            (root / "nested").rename(root / "actual-nested")
            (root / "nested").symlink_to("actual-nested", target_is_directory=True)
            with self.assertRaisesRegex(ValueError, "symlink"):
                tracked_inputs(root)
            (root / "nested").unlink()
            (root / "actual-nested").rename(root / "nested")
            oid = git("rev-parse", "HEAD:plain.txt").decode().strip()
            subprocess.run(["git", "-C", str(root), "update-index", "--index-info"],
                           input=("0 " + "0" * 40 + "\tplain.txt\n100644 " + oid + " 1\tplain.txt\n").encode(),
                           check=True, capture_output=True)
            with self.assertRaisesRegex(ValueError, "unmerged"):
                tracked_inputs(root)

    def test_input_census_refuses_hash_disagreement_unavailable_tools_and_moving_index(self):
        with input_census_fixture() as (root, _):
            with mock.patch(__name__ + ".system_sha256", return_value="0" * 64):
                with self.assertRaisesRegex(ValueError, "hashes differ"):
                    tracked_inputs(root)
            with mock.patch(__name__ + ".system_sha256", side_effect=FileNotFoundError("shasum unavailable")):
                with self.assertRaisesRegex(FileNotFoundError, "shasum unavailable"):
                    tracked_inputs(root)
            initial = index_inputs(root)
            with mock.patch(__name__ + ".index_inputs", side_effect=[
                    initial, dict(initial, source_commit="0" * 40)]):
                with self.assertRaisesRegex(ValueError, "changed during census"):
                    tracked_inputs(root)

    def test_input_census_paths_refuse_noncanonical_symlink_and_other_volume_storage(self):
        with input_census_fixture() as (root, _):
            for value in ("", "/outside", "../outside", "target/../outside", "target//double", "./target/data", "target/"):
                with self.subTest(value=value), self.assertRaises(ValueError):
                    census_data_path(root, value, output=True)
            with self.assertRaisesRegex(ValueError, "target"):
                census_data_path(root, "outside", output=True)
            (root / "target").symlink_to("nested", target_is_directory=True)
            with self.assertRaisesRegex(ValueError, "symlink"):
                census_data_path(root, "target/data", output=True)
            (root / "target").unlink()
            original = Path.lstat
            def other_volume(path):
                info = original(path)
                return mock.Mock(st_dev=info.st_dev + 1, st_mode=info.st_mode) if path == root / "nested" else info
            with mock.patch.object(Path, "lstat", other_volume):
                with self.assertRaisesRegex(ValueError, "other-volume"):
                    census_data_path(root, "nested/input.bin")

    def test_input_snapshot_refuses_malformed_fields_paths_and_duplicate_json_keys(self):
        with input_census_fixture() as (root, _):
            snapshot = tracked_inputs(root)
            invalid = [None, [], {}, dict(snapshot, extra=True), dict(snapshot, schema="other"),
                       dict(snapshot, source_commit="wrong"), dict(snapshot, producer_sha256=12),
                       dict(snapshot, regular={}), dict(snapshot, gitlinks=[])]
            regular = snapshot["regular"]
            entry = regular["plain.txt"]
            for changed in (None, dict(entry, extra=True), dict(entry, size=True), dict(entry, size=-1),
                            dict(entry, mode=False), dict(entry, mode=0o10000), dict(entry, index_mode="120000"),
                            dict(entry, index_oid="bad"), dict(entry, sha256="bad")):
                invalid.append(dict(snapshot, regular={**regular, "plain.txt": changed}))
            for name in ("/outside", "../outside", "nested//input", "nested/../input"):
                invalid.append(dict(snapshot, regular={name: entry}))
            invalid += [dict(snapshot, gitlinks={"plain.txt": "1" * 40}),
                        dict(snapshot, gitlinks={"../outside": "1" * 40}), dict(snapshot, gitlinks={"opaque": "bad"})]
            for changed in invalid:
                with self.subTest(changed=changed), self.assertRaises(ValueError):
                    load_input_snapshot(json.dumps(changed))
            with self.assertRaisesRegex(ValueError, "duplicate"):
                load_input_snapshot('{"schema":1,"schema":2}')

    def test_input_check_replaces_stale_success_and_preserves_its_snapshot(self):
        with input_census_fixture() as (root, _):
            snapshot_name, result_name = "target/inputs.json", "target/result.json"
            input_census_command(root, snapshot_name)
            raw = (root / snapshot_name).read_bytes()
            input_census_command(root, result_name, snapshot_name)
            self.assertEqual(json.loads((root / result_name).read_bytes())["status"], "restored")
            (root / "plain.txt").write_bytes(b"changed")
            with self.assertRaisesRegex(ValueError, "differs"):
                input_census_command(root, result_name, snapshot_name)
            self.assertEqual(json.loads((root / result_name).read_bytes())["status"], "refused")
            with self.assertRaisesRegex(ValueError, "overwrite"):
                input_census_command(root, snapshot_name, snapshot_name)
            os.link(root / snapshot_name, root / "target/alias.json")
            with self.assertRaisesRegex(ValueError, "overwrite"):
                input_census_command(root, "target/alias.json", snapshot_name)
            with self.assertRaises(ValueError):
                input_census_command(root, snapshot_name, "target//inputs.json")
            self.assertEqual((root / snapshot_name).read_bytes(), raw)
            (root / snapshot_name).write_bytes(b"not JSON")
            with self.assertRaises(ValueError):
                input_census_command(root, result_name, snapshot_name)
            self.assertEqual(json.loads((root / result_name).read_bytes())["status"], "refused")
            (root / snapshot_name).unlink()
            with self.assertRaises(FileNotFoundError):
                input_census_command(root, result_name, snapshot_name)
            self.assertEqual(json.loads((root / result_name).read_bytes())["status"], "refused")
            self.assertNotIn(str(root), (root / result_name).read_text())

    def test_input_census_refuses_nonregular_evidence_before_opening_it(self):
        with input_census_fixture() as (root, _):
            (root / "target").mkdir()
            fifo = root / "target/fifo"
            os.mkfifo(fifo)
            with self.assertRaisesRegex(ValueError, "output must be a regular"):
                input_census_command(root, "target/fifo")
            with mock.patch.object(Path, "read_bytes", side_effect=AssertionError("opened nonregular evidence")):
                with self.assertRaisesRegex(ValueError, "snapshot must be a regular"):
                    input_census_command(root, "target/result.json", "target/fifo")

    def test_input_census_output_replacement_preserves_hardlinked_tracked_bytes(self):
        with input_census_fixture() as (root, _):
            (root / "target").mkdir()
            original = (root / "plain.txt").read_bytes()
            os.link(root / "plain.txt", root / "target/result.json")
            input_census_command(root, "target/result.json")
            self.assertEqual((root / "plain.txt").read_bytes(), original)
            snapshot = load_input_snapshot((root / "target/result.json").read_bytes())
            self.assertEqual(compare_input_snapshot(root, snapshot), snapshot)

    def test_input_census_cli_refuses_mixed_modes_and_missing_output(self):
        script = str(Path(__file__).resolve())
        for options in (["--snapshot-inputs", "--progress"], ["--snapshot-inputs", "--self-test"],
                        ["--snapshot-inputs", "--measure", "xtask/src/trust.rs"],
                        ["--check-inputs", "target/inputs.json", "--plan", "target/plan.json"],
                        ["--snapshot-inputs", "--check-inputs", "target/inputs.json"], ["--snapshot-inputs"]):
            result = subprocess.run([sys.executable, "-I", "-B", script, *options], capture_output=True,
                                    text=True, timeout=5)
            self.assertEqual(result.returncode, 2, result.stdout + result.stderr)
            self.assertIn("error:", result.stderr)

    def test_actual_progress_traces_and_both_wrong_oracles(self):
        root = Path(__file__).resolve().parent.parent
        result = progress_control(root, root / "target/trust-progress-selftest.json")
        source = (root / "xtask/src/trust.rs").read_text()
        candidates, _, _ = discover((root / SWEEP).read_text(), source)
        self.assertEqual(result["cases"], list(progress_cases(source, candidates)))
        self.assertEqual([run["exit_code"] for run in result["runs"]], [0, 101, 101])

    def test_progress_refuses_missing_identity_and_wrong_source(self):
        root = Path(__file__).resolve().parent.parent
        source = (root / "xtask/src/trust.rs").read_text()
        candidates, _, _ = discover((root / SWEEP).read_text(), source)
        with self.assertRaisesRegex(ValueError, "identities missing"):
            progress_program(source, [])
        first_identity = next(iter(progress_cases(source, candidates)))
        duplicate = next(candidate for candidate in candidates if candidate["id"] == first_identity)
        with self.assertRaisesRegex(ValueError, "ambiguous"):
            progress_program(source, candidates + [dict(duplicate, id="duplicate")])
        changed = source.replace(PROGRESS_CASES[-1][0] + "\n", "                i += 3;\n", 1)
        with self.assertRaisesRegex(ValueError, "source drift"):
            progress_program(changed, candidates)
        shifted = [dict(candidate, id="shifted-" + candidate["id"]) for candidate in candidates]
        self.assertEqual(list(progress_cases(source, shifted)),
                         ["shifted-" + identity for identity in progress_cases(source, candidates)])

    def test_manifest_paths_refuse_before_reading_any_unrecognized_input(self):
        paths = {DECISION, CATALOGUE, SWEEP, "xtask/src/trust.rs", "xtask/src/trust_generated.rs"}
        hashes = {path: "0" * 64 for path in paths}
        for changed in (None, list(paths), {}, {**hashes, "../outside": "0" * 64},
                        {**hashes, "/outside": "0" * 64}, {k: v for k, v in hashes.items() if k != SWEEP}):
            # None cannot read a path: a mistaken acceptance would fail with TypeError, not the required refusal.
            with self.assertRaisesRegex(ValueError, "unrecognized plan input paths"):
                validate_inputs(None, changed)
        root = Path(__file__).resolve().parent.parent
        valid = {path: digest((root / path).read_bytes()) for path in paths}
        validate_inputs(root, valid)
        for value in ("wrong", 12, "0" * 64):
            with self.assertRaises(ValueError):
                validate_inputs(root, {**valid, SWEEP: value})

    def test_result_audit_refuses_missing_duplicate_foreign_text_unnamed_counts_and_exit(self):
        file_plan = dict(production_range=[0, 4], listed_count=1, candidates=[
            dict(id="L2.bool.0", span=[1, 2], operator="bool", original="    true", replacement="    false")])
        baseline = "mutation-sweep: the unmutated copy passes (0s); 1 mutation(s) of f.rs, lines 1 to 4\n"
        entry = "  ✓ L2.bool.0          killed     true ⟶ false — tests::fails\n"
        footer = "mutation-sweep: 1 killed\nmutation-sweep: unviable by operator: bool=0/1 (unviable/attempted)\n"
        log = baseline + entry + footer
        self.assertEqual(audit_measurement(log, file_plan, "f.rs", 0)["entries"][0]["names"], ["tests::fails"])
        for changed in (baseline + footer, baseline + entry + entry + footer, log.replace("L2.bool.0", "L2.bool.9"),
                        log.replace("⟶ false", "⟶ other"), log.replace(" — tests::fails", ""),
                        log.replace("1 killed", "2 killed"), log.replace("bool=0/1", "bool=1/1"),
                        log.replace("the unmutated copy passes", "baseline missing")):
            with self.assertRaises(ValueError):
                audit_measurement(changed, file_plan, "f.rs", 0)
        with self.assertRaises(ValueError):
            audit_measurement(log, file_plan, "f.rs", 1)
        for outcome, mark, detail, code in (("survived", "✗", "", 1), ("timeout", "✗", "", 1),
                                           ("unviable", "·", " — error[E0308]: mismatched types", 0)):
            changed = baseline + entry.replace("✓", mark).replace("killed    ", "%-10s" % outcome).replace(" — tests::fails", detail)
            changed += footer.replace("1 killed", "1 " + outcome).replace("bool=0/1", "bool=%d/1" % (outcome == "unviable"))
            self.assertEqual(audit_measurement(changed, file_plan, "f.rs", code)["counts"], {outcome: 1})

    def test_equivalent_results_require_selected_reason_records(self):
        candidate = dict(id="L2.bool.0", span=[1, 2], operator="bool", original="    true", replacement="    false")
        record = dict(original="true", replacement="false", reason="fixture metadata, not a semantic proof")
        file_plan = dict(production_range=[0, 4], listed_count=1, candidates=[candidate], equivalents=[record])
        baseline = "mutation-sweep: the unmutated copy passes (0s); 1 mutation(s) of f.rs, lines 1 to 4\n"
        entry = "  = L2.bool.0          equivalent true ⟶ false\n"
        footer = "mutation-sweep: 1 equivalent\nmutation-sweep: unviable by operator: bool=0/1 (unviable/attempted)\n"
        log = baseline + entry + footer
        result = audit_measurement(log, file_plan, "f.rs", 0)
        self.assertEqual(result["counts"], {"equivalent": 1})
        self.assertEqual(result["entries"][0]["names"], [])
        self.assertEqual(result["entries"][0]["reason"], record["reason"])
        # The native parser trims outer whitespace; duplicate detection must use that same key.
        padded = {**record, "original": "  true  ", "replacement": " false ", "reason": " fixture reason "}
        self.assertEqual(audit_measurement(log, {**file_plan, "equivalents": [padded]}, "f.rs", 0)
                         ["entries"][0]["reason"], "fixture reason")
        malformed = (None, "not records", record, [], [None], [{}], [{**record, "extra": "field"}],
                     [{k: v for k, v in record.items() if k != "reason"}], [record, record], [record, padded])
        for field in record:
            malformed += tuple([{**record, field: value}] for value in (None, 1, "", "   "))
        malformed += ([{**record, "reason": "first\nsecond"}], [{**record, "reason": "first\rsecond"}],
                      [{**record, "reason": "\nfixture reason"}], [{**record, "reason": "fixture reason\r"}],
                      [{**record, "original": "other"}], [{**record, "replacement": "other"}],
                      [record, {**record, "original": "other"}])
        for records in malformed:
            with self.subTest(records=records), self.assertRaises(ValueError):
                audit_measurement(log, {**file_plan, "equivalents": records}, "f.rs", 0)
        for changed, code in ((log, 1), (log.replace("equivalent true", "equivalent other"), 0),
                              (log.replace("⟶ false", "⟶ other"), 0), (log.replace("  =", "  ✓"), 0),
                              (log.replace("⟶ false", "⟶ false — tests::not_a_kill"), 0),
                              (log.replace("1 equivalent", "2 equivalent"), 0)):
            with self.subTest(log=changed, code=code), self.assertRaises(ValueError):
                audit_measurement(changed, file_plan, "f.rs", code)
        with self.assertRaises(ValueError):
            audit_measurement(log, {k: v for k, v in file_plan.items() if k != "equivalents"}, "f.rs", 0)

    def test_equivalence_records_cannot_relabel_non_equivalent_outcomes(self):
        candidate = dict(id="L2.bool.0", span=[1, 2], operator="bool", original="true", replacement="false")
        file_plan = dict(production_range=[0, 4], listed_count=1, candidates=[candidate], equivalents=[
            dict(original="true", replacement="false", reason="fixture metadata, not a semantic proof")])
        baseline = "mutation-sweep: the unmutated copy passes (0s); 1 mutation(s) of f.rs, lines 1 to 4\n"
        for outcome, mark, detail, code in (("killed", "✓", " — tests::fails", 0), ("survived", "✗", "", 1),
                                           ("unviable", "·", " — error[E0308]", 0), ("timeout", "✗", "", 1),
                                           ("unjudged", "?", " — incomplete", 2)):
            log = baseline + "  %s %-18s %-10s true ⟶ false%s\n" % (mark, candidate["id"], outcome, detail)
            log += "mutation-sweep: 1 %s\nmutation-sweep: unviable by operator: bool=%d/1 (unviable/attempted)\n" % (
                outcome, outcome == "unviable")
            with self.subTest(outcome=outcome), self.assertRaises(ValueError):
                audit_measurement(log, file_plan, "f.rs", code)

    def test_complete_line_spans_and_half_open_boundaries(self):
        source = "const X: bool = true;\nfn f() {\n    a();\n    b();\n}\n"
        self.assertEqual(source_span(source, "    a();\n    b();"), [2, 4])
        self.assertEqual(source_span(source, "a();"), [2, 3])
        self.assertEqual(source_span(source, "    a();\n"), [2, 3])
        self.assertEqual(source_span(source, "    a();\n    b();\n}"), [2, 5])
        with self.assertRaises(ValueError):
            source_span(source, "missing")
        with self.assertRaises(ValueError):
            source_span(source + source, "    a();")
        self.assertFalse(overlaps([2, 4], [0, 2]))
        self.assertFalse(overlaps([2, 4], [4, 6]))
        for candidate in ([1, 3], [3, 5], [2, 4], [0, 7], [2, 3]):
            self.assertTrue(overlaps([2, 4], candidate))

    def test_listing_refuses_missing_reordered_changed_and_extra_candidates(self):
        listing = ["L2.bool.0         true\n                   ⟶ false", "L3.bool.1         false\n                   ⟶ true"]
        footer = "mutation-sweep: 2 mutation(s) of f.rs, lines 1 to 4\n"
        actual = "\n".join(listing) + "\n" + footer
        verify_listing(actual, listing, "f.rs", [0, 4])
        for changed in (actual.replace(listing[0] + "\n", ""), "\n".join(reversed(listing)) + "\n" + footer,
                        actual.replace("L2.bool.0", "L2.bool.9"), actual.replace("⟶ false", "⟶ true"),
                        actual + listing[0], actual.replace("lines 1 to 4", "lines 1 to 5")):
            with self.assertRaisesRegex(ValueError, "CLI listing differs"):
                verify_listing(changed, listing, "f.rs", [0, 4])

    def test_catalogue_pairs_refuse_population_identity_and_block_drift(self):
        rows = [dict(suffix="sample", file="xtask/src/trust.rs")]
        text = ""
        for population, args in POPULATIONS.items():
            text += ("mutation " + population + "-sample\nfile xtask/src/trust.rs\ntest " + " ".join(args)
                     + "\nexpect killed\nwhy control\nfrom\n        true\nto\n        false\nend\n\n")
        entries = catalogue(text)
        bind_pairs(rows, entries)
        self.assertEqual(entries["route-sample"]["from"], "    true")
        for changed in (text.replace("parent-sample", "parent-extra"), text.replace("expect killed", "expect survives", 1),
                        text.replace("test -p xtask route_tests", "test -p xtask trust::"),
                        text.replace("        false", "        other", 1),
                        text.replace("file xtask/src/trust.rs", "file xtask/src/trust_generated.rs", 1)):
            with self.assertRaises(ValueError):
                bind_pairs(rows, catalogue(changed))
        for changed in (text + text, text.replace("        true", "true", 1), text.replace("\nend\n", "\n", 1),
                        text[:text.index("\nwhy")], text.replace("\nfrom\n", "\nwrong\n", 1)):
            with self.assertRaises(ValueError):
                catalogue(changed)

    def test_census_refuses_missing_duplicate_and_malformed_rows(self):
        text = MARKER + "\n| Suffix | Source findings | Catalogue origin and file |\n| --- | --- | --- |\n"
        row = "| `sample` | R1-D1 | existing, `trust.rs` |\n"
        self.assertEqual(census(text + row)[0]["findings"], "R1-D1")
        for changed in (text, text + row + row, text + row.replace("trust.rs", "other.rs"), text + row + MARKER):
            with self.assertRaises(ValueError):
                census(changed)

    def test_actual_discovery_preserves_scope_raw_text_and_global_ids(self):
        script = (Path(__file__).resolve().parent / "mutation_sweep.sh").read_text()
        source = "const OUTSIDE: bool = true;\nfn sample() -> bool {\n    true\n}\n#[cfg(test)]\nmod tests {}\n"
        candidates, listing, interval = discover(script, source)
        self.assertEqual(interval, [1, 4])
        self.assertEqual(candidates, [dict(id="L3.bool.0", span=[2, 3], operator="bool", original="    true", replacement="    false")])
        self.assertEqual(listing, ["L3.bool.0          true\n                   ⟶ false"])
        with self.assertRaises(ValueError):
            discover(script, "#[cfg(test)]\nfn only_test() {}")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", help="repository-relative JSON file under target/")
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--progress", action="store_true", help="compile bounded traces of five actual non-progress edits")
    parser.add_argument("--measure", choices=["xtask/src/trust_generated.rs", "xtask/src/trust.rs"])
    parser.add_argument("--population", choices=list(POPULATIONS))
    parser.add_argument("--plan", help="repository-relative candidate manifest")
    modes = parser.add_mutually_exclusive_group()
    modes.add_argument("--snapshot-inputs", action="store_true", help="capture indexed input bytes/modes and gitlink identities")
    modes.add_argument("--check-inputs", help="verify restoration against a repository-relative snapshot under target/")
    args = parser.parse_args()
    if args.snapshot_inputs or args.check_inputs is not None:
        if args.self_test or args.progress or args.measure or args.population or args.plan:
            parser.error("input census cannot be combined with planning, measurement or self-test options")
        if not args.output:
            parser.error("input census requires --output")
        input_census_command(Path(__file__).resolve().parent.parent, args.output, args.check_inputs)
        return 0
    if args.self_test:
        result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Controls))
        return 0 if result.wasSuccessful() else 1
    if not args.output:
        parser.error("--output or --self-test required")
    root = Path(__file__).resolve().parent.parent
    output = local_output(root, args.output)
    if args.progress:
        if args.measure or args.population or args.plan:
            parser.error("--progress cannot be combined with measurement options")
        output.parent.mkdir(parents=True, exist_ok=True)
        result = progress_control(root, output)
        print("trust-adequacy: %d actual non-progress edits; compiled traces and two rejecting oracles pass; no OS crash claim"
              % len(result["cases"]))
        return 0
    if args.measure:
        if not args.population or not args.plan:
            parser.error("--measure requires --population and --plan")
        local_output(root, args.plan)
        measure(root, args.plan, args.measure, args.population, output)
        return 0
    if args.population or args.plan:
        parser.error("--population/--plan require --measure")
    manifest = plan(root)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(manifest, indent=2) + "\n")
    selected = sum(len(f["candidates"]) for f in manifest["files"].values())
    print("trust-adequacy: %d census rows; %d unique file/candidates selected; actual CLI listings agree; planning only"
          % (len(manifest["rows"]), selected))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print("trust-adequacy: could not plan: " + str(error), file=sys.stderr)
        sys.exit(2)
