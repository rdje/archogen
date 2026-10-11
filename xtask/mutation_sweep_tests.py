"""Contract probes for the real sweep CLI; synthetic commands, no fixture Git/Cargo build."""
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(sys.argv.pop(1)).resolve()
SCRIPT = ROOT / "scripts/mutation_sweep.sh"
ORIGINAL = 'fn sample() -> bool {\n    true\n}\n#[cfg(test)]\nmod tests {}\n'
GIT = '''#!/usr/bin/env python3
import os, sys
if sys.argv[1:] == ['rev-parse', '--show-toplevel']:
    print(os.environ['PROBE_ROOT'])
elif sys.argv[1:] == ['ls-files', '-s', '-z']:
    sys.stdout.buffer.write(b'100644 ' + b'0' * 40 + b' 0\\tsample.rs\\0')
else:
    sys.exit(2)
'''
CARGO = '''#!/usr/bin/env python3
import os, pathlib
root = pathlib.Path(os.environ['PROBE_ROOT'])
state = root / 'invoked'
text = pathlib.Path('sample.rs').read_text()
expected = os.environ['PROBE_MUTATED'] if state.exists() else os.environ['PROBE_ORIGINAL']
assert text == expected, repr((text, expected))
with state.open('a') as out:
    out.write('called\\n')
'''


class SweepFixture(unittest.TestCase):
    def setUp(self):
        scratch = ROOT / "target/doctrine_scratch"
        scratch.mkdir(parents=True, exist_ok=True)
        self.work = tempfile.TemporaryDirectory(prefix="sweep-spans.", dir=scratch)
        self.addCleanup(self.work.cleanup)
        self.base = Path(self.work.name)
        (self.base / "bin").mkdir()
        for name, text in [("git", GIT), ("cargo", CARGO)]:
            path = self.base / "bin" / name
            path.write_text(text)
            path.chmod(0o755)

    def run_sweep(self, record, *, original=ORIGINAL, mutated=None, candidate=None):
        (self.base / "invoked").unlink(missing_ok=True)
        (self.base / "sample.rs").write_bytes(original.encode())
        (self.base / "equivalents").write_text(record)
        script = SCRIPT
        if candidate is not None:
            # Supply a synthetic multi-line candidate at the discovery/runner boundary. The actual CLI,
            # equivalence reader, tracked copy, landing, command and restoration are left intact.
            text = SCRIPT.read_text()
            marker = next(line for line in text.splitlines() if line.startswith("ms = "))
            text = text.replace(marker, "ms = " + repr([candidate]), 1)
            script = self.base / "sweep.sh"
            script.write_text(text)
        mutated = mutated if mutated is not None else original.replace("    true\n", "    false\n")
        env = dict(os.environ, PROBE_ROOT=str(self.base), PROBE_ORIGINAL=original,
                   PROBE_MUTATED=mutated, PATH=str(self.base / "bin") + os.pathsep + os.environ["PATH"])
        result = subprocess.run(["bash", str(script), "sample.rs", "--package", "synthetic", "--op", "bool",
                                 "--equivalents", "equivalents"], env=env, capture_output=True, text=True, timeout=20)
        self.assertEqual((self.base / "sample.rs").read_bytes(), original.encode())
        copied = self.base / "target/mutation-sweep/tree/sample.rs"
        if copied.exists():
            self.assertEqual(copied.read_bytes(), original.encode())
        return result

    def assert_exit(self, result, expected):
        self.assertEqual(result.returncode, expected, result.stdout + result.stderr)
        self.assertNotIn("Traceback", result.stderr)


class EquivalenceTests(SweepFixture):
    def test_malformed_records_refuse_before_any_command_at_separator_and_eof(self):
        records = [
            "from true\nto false",                              # missing reason
            "from true\nto false\nwhy ",                       # empty reason
            "from true\nfrom true\nto false\nwhy same",        # duplicate raw field
            'from true\nfrom-json "true"\nto false\nwhy same', # duplicate canonical field
            "from true\nto false\nwhy same\nextra value",      # unknown field
            'from-json "true\nto false\nwhy same',              # malformed JSON
            'from-json 7\nto false\nwhy same',                   # non-string JSON
            'from-json "  "\nto false\nwhy same',              # empty source
            'from true\nto-json ""\nwhy same',                 # empty replacement
            "from true\nto false\nwhy same\n\nfrom true\nto false\nwhy again", # duplicate entry
        ]
        for record in records:
            for ending in ["", "\n\n"]:
                with self.subTest(record=record, separator=bool(ending)):
                    result = self.run_sweep(record + ending)
                    self.assert_exit(result, 2)
                    self.assertFalse((self.base / "invoked").exists(), "bad records reached Cargo")

    def test_raw_and_json_single_line_records_keep_literal_backslashes(self):
        original = ORIGINAL.replace("    true", r'    true // literal \n')
        mutated = original.replace("    true", "    false")
        before, after = original.splitlines()[1].strip(), mutated.splitlines()[1].strip()
        for encoded in [False, True]:
            for ending in ["", "\n\n"]:
                with self.subTest(encoded=encoded, separator=bool(ending)):
                    record = (f"from-json {json.dumps(before)}\nto-json {json.dumps(after)}" if encoded
                              else f"from {before}\nto {after}") + "\nwhy fixture equivalence" + ending
                    result = self.run_sweep(record, original=original, mutated=mutated)
                    self.assert_exit(result, 0)
                    self.assertIn("1 equivalent", result.stdout)
                    self.assertEqual((self.base / "invoked").read_text(), "called\ncalled\n")


class SpanTests(SweepFixture):
    def test_expansion_and_contraction_land_exactly_and_restore(self):
        original = 'fn sample() -> bool {\n    let first = true;\n    first\n}\n#[cfg(test)]\nmod tests {}\n'
        for stop, replacement in [(2, "    let first = false;\n    let second = true;"),
                                  (3, "    false")]:
            before = "\n".join(original.split("\n")[1:stop]).strip()
            mutated_lines = original.split("\n")
            mutated_lines[1:stop] = replacement.split("\n")
            mutated = "\n".join(mutated_lines)
            for equivalent in [False, True]:
                with self.subTest(stop=stop, equivalent=equivalent):
                    record = (f"from-json {json.dumps(before)}\nto-json {json.dumps(replacement.strip())}"
                              "\nwhy synthetic survivor") if equivalent else ""
                    result = self.run_sweep(record, original=original, mutated=mutated,
                                            candidate=(1, stop, "synthetic", replacement))
                    self.assert_exit(result, 0 if equivalent else 1)
                    self.assertIn("1 equivalent" if equivalent else "1 survived", result.stdout)
                    self.assertEqual((self.base / "invoked").read_text(), "called\ncalled\n")


if __name__ == "__main__":
    unittest.main()
