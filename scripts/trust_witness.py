#!/usr/bin/env python3
"""Re-audit initial measurements and retain syntax/diff evidence; no semantic classification."""
import argparse
import copy
import difflib
import json
import os
from pathlib import Path
import subprocess
import sys
import tomllib
import unittest

sys.dont_write_bytecode = True
import trust_adequacy as adequacy


def command_for(filename, population, candidates):
    command = ["bash", adequacy.SWEEP, filename, "--package", "xtask", "--timeout", "180", "--timeouts-fail"]
    filters = ["route_tests"] if population == "route" else adequacy.POPULATIONS["parent"][3:]
    command += ["--filter=" + item for item in filters]
    for candidate in candidates:
        command += ["--only", candidate["id"]]
    return command


def audit_record(record, log, manifest, plan_path, plan_hash, filename, population):
    """Bind native results to the intended selection, command, source and actual log, not its desired verdict."""
    expected = dict(schema="archogen-trust-adequacy-measurement/1", status="measured", file=filename,
                    population=population, plan=plan_path, plan_sha256=plan_hash,
                    input_sha256=manifest["input_sha256"], complete_identities=True, baseline_passed=True,
                    restored=True, copied_sha256=manifest["input_sha256"][filename],
                    command=command_for(filename, population, manifest["files"][filename]["candidates"]))
    for key, value in expected.items():
        if key not in record or record[key] != value:
            raise ValueError("measurement binding differs: " + key)
    audited = adequacy.audit_measurement(log, manifest["files"][filename], filename, record["raw_exit_code"])
    for key, value in audited.items():
        if record.get(key) != value:
            raise ValueError("measurement differs from native log: " + key)
    if audited["counts"].get("timeout", 0):
        raise ValueError("initial measurement contains a timeout")
    return {entry["id"]: entry for entry in audited["entries"]}


def apply_exact(source, original, replacement):
    if source.count(original) != 1:
        raise ValueError("exact replacement needs one original occurrence")
    return source.replace(original, replacement, 1)


def apply_candidate(source, candidate):
    lines = source.split("\n")
    lo, hi = candidate["span"]
    if not 0 <= lo < hi <= len(lines) or "\n".join(lines[lo:hi]) != candidate["original"]:
        raise ValueError("candidate original/span differs")
    # Same half-open line replacement as the actual sweep's apply_mutant, with the old text verified above.
    lines[lo:hi] = candidate["replacement"].split("\n")
    return "\n".join(lines)


def formatter_command(root):
    workspace = tomllib.loads((root / "Cargo.toml").read_text())
    package = tomllib.loads((root / "xtask/Cargo.toml").read_text())["package"]
    edition = package["edition"]
    if edition == {"workspace": True}:
        edition = workspace["workspace"]["package"]["edition"]
    if edition not in ("2015", "2018", "2021", "2024"):
        raise ValueError("unrecognized xtask Rust edition")
    return ["rustfmt", "--edition", edition, "--emit", "stdout", "--config", "skip_children=true"]


def formatted(root, source):
    env = dict(os.environ, CARGO_HOME=str(root / "target/cargo-home"), TMPDIR=str(root / "target/tmp"))
    result = subprocess.run(formatter_command(root),
                            cwd=root, env=env, input=source, text=True, capture_output=True, timeout=60)
    if result.returncode:
        return dict(status="refused", stderr=result.stderr, exit_code=result.returncode)
    return dict(status="formatted", sha256=adequacy.digest(result.stdout.encode()))


def syntax_matches(exact, candidate):
    # A pair of parser errors has no usable identity, even if its diagnostics or missing hashes agree.
    return exact["status"] == candidate["status"] == "formatted" and exact["sha256"] == candidate["sha256"]


def patch(source, mutant, filename):
    return "".join(difflib.unified_diff(source.splitlines(keepends=True), mutant.splitlines(keepends=True),
                                        fromfile=filename, tofile=filename + " (mutated)"))


def evidence(root, plan_path, measurement_paths):
    plan_bytes = (root / plan_path).read_bytes()
    manifest = json.loads(plan_bytes)
    adequacy.validate_inputs(root, manifest["input_sha256"])
    if manifest != adequacy.plan(root):
        raise ValueError("manifest differs from actual full plan")
    wanted = {(filename, population) for filename in manifest["files"] for population in adequacy.POPULATIONS}
    measurements, metadata = {}, []
    sources = {filename: (root / filename).read_text() for filename in manifest["files"]}
    source_files = [*sources, "xtask/src/trust_gate.rs"]
    originals = {filename: (root / filename).read_bytes() for filename in source_files}
    formatter_inputs = {p: (root / p).read_bytes() for p in ("Cargo.toml", "xtask/Cargo.toml", "rust-toolchain.toml")}
    for path in measurement_paths:
        adequacy.local_output(root, path)
        raw = (root / path).read_bytes()
        record = json.loads(raw)
        identity = (record.get("file"), record.get("population"))
        if identity not in wanted or identity in measurements:
            raise ValueError("foreign or duplicate measurement population")
        filename, population = identity
        logfile = adequacy.local_output(root, record["log"])
        log = logfile.read_bytes()
        measurements[identity] = audit_record(record, log.decode(), manifest, plan_path,
                                               adequacy.digest(plan_bytes), filename, population)
        commit = record["source_commit"]
        if len(commit) != 40 or any(c not in "0123456789abcdef" for c in commit):
            raise ValueError("invalid recorded source commit")
        # Read only explicit known paths at a recorded commit, never scan repository history or case blobs.
        for source_file in source_files:
            committed = subprocess.check_output(["git", "show", commit + ":" + source_file], cwd=root)
            copied = (root / "target/mutation-sweep/tree" / source_file).read_bytes()
            if committed != originals[source_file] or copied != originals[source_file]:
                raise ValueError("committed/current/restored source differs: " + source_file)
        metadata.append(dict(file=filename, population=population, measurement=path,
                             measurement_sha256=adequacy.digest(raw), log=record["log"],
                             log_sha256=adequacy.digest(log), source_commit=commit,
                             recorded_producer_sha256=record["producer_sha256"]))
    if set(measurements) != wanted:
        raise ValueError("four complete initial populations required")
    cache = {}

    def describe(source, mutant, filename):
        sha = adequacy.digest(mutant.encode())
        if sha not in cache:
            cache[sha] = formatted(root, mutant)
        return dict(source_sha256=sha, syntax=cache[sha], diff=patch(source, mutant, filename))

    files = {}
    for filename, file_plan in manifest["files"].items():
        source = sources[filename]
        candidates = []
        for candidate in file_plan["candidates"]:
            item = dict(candidate)
            item.update(describe(source, apply_candidate(source, candidate), filename))
            item["outcomes"] = {population: measurements[filename, population][candidate["id"]]
                                for population in adequacy.POPULATIONS}
            candidates.append(item)
        files[filename] = dict(candidates=candidates)
    rows = []
    for row in manifest["rows"]:
        source, filename = sources[row["file"]], row["file"]
        exact = describe(source, apply_exact(source, row["exact_original"], row["exact_replacement"]), filename)
        exact["syntax_match_ids"] = [candidate["id"] for candidate in files[filename]["candidates"]
                                     if candidate["id"] in row["candidates"]
                                     and syntax_matches(exact["syntax"], candidate["syntax"])]
        rows.append(dict(row, exact=exact))
    adequacy.validate_inputs(root, manifest["input_sha256"])
    if (root / plan_path).read_bytes() != plan_bytes or any((root / p).read_bytes() != b for p, b in originals.items()):
        raise ValueError("evidence inputs changed during execution")
    if any((root / p).read_bytes() != b for p, b in formatter_inputs.items()):
        raise ValueError("formatter inputs changed during execution")
    return dict(schema="archogen-trust-witness-evidence/1",
                scope="initial measured span cohort; syntax identity and native outcomes, no semantic classification",
                plan=plan_path, plan_sha256=adequacy.digest(plan_bytes), input_sha256=manifest["input_sha256"],
                source_sha256={p: adequacy.digest(b) for p, b in originals.items()},
                rustfmt=subprocess.check_output(["rustfmt", "--version"], cwd=root, text=True).strip(),
                formatter_command=formatter_command(root),
                formatter_input_sha256={p: adequacy.digest(b) for p, b in formatter_inputs.items()},
                producer_sha256=adequacy.digest(Path(__file__).read_bytes()), measurements=metadata, files=files, rows=rows)


class Controls(unittest.TestCase):
    def test_formatter_ignores_layout_but_preserves_conditions_and_literal_contents(self):
        root = Path(__file__).resolve().parent.parent
        original = formatted(root, 'fn f(a: bool) -> bool { let s = "a b"; a }\n')
        layout = formatted(root, 'fn f(a:bool)->bool{let s="a b";a}\n')
        self.assertTrue(syntax_matches(original, layout))
        for source in ('fn f(a: bool) -> bool { let s = "ab"; a }\n',
                       'fn f(a: bool) -> bool { let s = "a b"; !a }\n', 'fn broken( {\n'):
            self.assertFalse(syntax_matches(original, formatted(root, source)))
        refused = formatted(root, 'fn broken( {\n')
        self.assertEqual(refused["status"], "refused")
        self.assertFalse(syntax_matches(refused, refused))

    def test_replacement_uses_full_interval_and_refuses_wrong_original(self):
        source = "fn f() {\n    a();\n    b();\n}\n"
        candidate = dict(span=[1, 3], original="    a();\n    b();", replacement="    c();")
        self.assertEqual(apply_candidate(source, candidate), "fn f() {\n    c();\n}\n")
        self.assertEqual(apply_exact(source, "a();", "z();"), source.replace("a();", "z();"))
        for changed in (dict(candidate, span=[1, 2]), dict(candidate, original="wrong"), dict(candidate, span=[-1, 3])):
            with self.assertRaises(ValueError):
                apply_candidate(source, changed)
        for original in ("missing", "\n"):
            with self.assertRaises(ValueError):
                apply_exact(source, original, "other")

    def test_measurement_binding_refuses_population_selection_source_and_replacement_drift(self):
        filename = "f.rs"
        candidates = [dict(id="L2.bool.0", span=[1, 2], operator="bool", original="    true", replacement="    false")]
        manifest = dict(input_sha256={filename: "source"}, files={filename: dict(
            candidates=candidates, production_range=[0, 4], listed_count=1)})
        log = ("mutation-sweep: the unmutated copy passes (0s); 1 mutation(s) of f.rs, lines 1 to 4\n"
               "  ✓ L2.bool.0          killed     true ⟶ false — tests::fails\n"
               "mutation-sweep: 1 killed\nmutation-sweep: unviable by operator: bool=0/1 (unviable/attempted)\n")
        record = dict(adequacy.audit_measurement(log, manifest["files"][filename], filename, 0),
                      schema="archogen-trust-adequacy-measurement/1", status="measured", file=filename,
                      population="route", plan="target/plan.json", plan_sha256="plan", input_sha256={filename: "source"},
                      restored=True, copied_sha256="source", command=command_for(filename, "route", candidates))
        args = (log, manifest, "target/plan.json", "plan", filename, "route")
        self.assertEqual(set(audit_record(record, *args)), {"L2.bool.0"})
        for key, value in (("population", "parent"), ("command", command_for(filename, "parent", candidates)),
                           ("command", record["command"][:-2]), ("input_sha256", {filename: "other"}),
                           ("restored", False), ("plan_sha256", "other"), ("entries", [])):
            changed = copy.deepcopy(record)
            changed[key] = value
            with self.assertRaises(ValueError):
                audit_record(changed, *args)
        with self.assertRaises(ValueError):
            audit_record(record, log.replace("⟶ false", "⟶ other"), *args[1:])


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--plan")
    parser.add_argument("--measurement", action="append", default=[])
    parser.add_argument("--output")
    args = parser.parse_args()
    if args.self_test:
        result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Controls))
        return 0 if result.wasSuccessful() else 1
    if not args.plan or not args.measurement or not args.output:
        parser.error("--plan, four --measurement arguments and --output required")
    root = Path(__file__).resolve().parent.parent
    output = adequacy.local_output(root, args.output)
    adequacy.local_output(root, args.plan)
    inputs = [root / args.plan, *(root / path for path in args.measurement)]
    if output.resolve() in {p.resolve() for p in inputs}:
        raise ValueError("output must not overwrite an input")
    result = evidence(root, args.plan, args.measurement)
    if output.resolve() in {(root / m["log"]).resolve() for m in result["measurements"]}:
        raise ValueError("output must not overwrite a native log")
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2) + "\n")
    matches = sum(bool(row["exact"]["syntax_match_ids"]) for row in result["rows"])
    print("trust-witness: four native populations re-audited; %d rows, %d with formatted syntax matches; semantics unclassified"
          % (len(result["rows"]), matches))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print("trust-witness: refused: " + str(error), file=sys.stderr)
        sys.exit(2)
