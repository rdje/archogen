#!/usr/bin/env python3
"""Plan census-span measurements and compile bounded traces of actual non-progress edits."""
import argparse
import ast
import bisect
from collections import Counter
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import subprocess
import sys
import time
import tomllib
import unittest

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


class Controls(unittest.TestCase):
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
    args = parser.parse_args()
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
