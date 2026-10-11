#!/usr/bin/env python3
"""Plan complete census-span operator measurements, without claiming semantic reach (M3.6.9.5.1)."""
import argparse
import ast
import bisect
import hashlib
import json
from pathlib import Path
import re
import subprocess
import sys
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


class Controls(unittest.TestCase):
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
    args = parser.parse_args()
    if args.self_test:
        result = unittest.TextTestRunner(verbosity=2).run(unittest.defaultTestLoader.loadTestsFromTestCase(Controls))
        return 0 if result.wasSuccessful() else 1
    if not args.output:
        parser.error("--output or --self-test required")
    root = Path(__file__).resolve().parent.parent
    output = Path(args.output)
    if output.is_absolute() or not (root / output).resolve().is_relative_to(root / "target"):
        parser.error("output must remain under this repository's target/")
    manifest = plan(root)
    (root / output).parent.mkdir(parents=True, exist_ok=True)
    (root / output).write_text(json.dumps(manifest, indent=2) + "\n")
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
