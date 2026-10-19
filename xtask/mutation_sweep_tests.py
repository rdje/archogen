"""Real sweep CLI probes, actual Cargo arguments and compiled candidates."""
import ast
import bisect
import hashlib
import json
import os
from pathlib import Path
import re
import signal
import shlex
import subprocess
import sys
import tempfile
import textwrap
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
assert os.environ.get('ARCHOGEN_SWEEP_SOURCE') == str(root.resolve()), 'source hint is not the canonical own root'
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

    def run_sweep(self, record, *, original=ORIGINAL, mutated=None, candidate=None, source_hint=None):
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
        if source_hint is None:
            env.pop("ARCHOGEN_SWEEP_SOURCE", None)
        else:
            env["ARCHOGEN_SWEEP_SOURCE"] = source_hint
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


class SelectionIdTests(SweepFixture):
    def selected(self, ids, *, operator="bool", listing=False, original=ORIGINAL,
                 record="from true\nto false\nwhy fixture\n"):
        (self.base / "invoked").unlink(missing_ok=True)
        (self.base / "sample.rs").write_bytes(original.encode())
        (self.base / "equivalents").write_text(record)
        env = dict(os.environ, PROBE_ROOT=str(self.base), PROBE_ORIGINAL=original,
                   PROBE_MUTATED=original.replace("    true\n", "    false\n"),
                   PATH=str(self.base / "bin") + os.pathsep + os.environ["PATH"])
        command = ["bash", str(SCRIPT), "sample.rs", "--package", "synthetic", "--op", operator,
                   "--equivalents", "equivalents"]
        for ident in ids:
            command.extend(["--only", ident])
        if listing:
            command.append("--list")
        result = subprocess.run(command, env=env, capture_output=True, text=True, timeout=20)
        self.assertEqual((self.base / "sample.rs").read_bytes(), original.encode())
        copied = self.base / "target/mutation-sweep/tree/sample.rs"
        if copied.exists() and not listing and result.returncode != 2:
            self.assertEqual(copied.read_bytes(), original.encode())
        return result

    def test_unknown_mixed_stale_and_operator_excluded_ids_refuse_before_copy_or_command(self):
        cases = [(["L999999.not-a-current-id.0"], "bool", ORIGINAL),
                 (["L2.bool.0", "L999999.not-a-current-id.0"], "bool", ORIGINAL),
                 (["L2.bool.0"], "bool", "\n" + ORIGINAL),
                 (["L2.bool.0"], "cmp", ORIGINAL),
                 ([""], "bool", ORIGINAL),
                 (["bad\nRunning `rustc`"], "bool", ORIGINAL)]
        tree = self.base / "target/mutation-sweep/tree"
        for ids, operator, original in cases:
            for listing in [False, True]:
                with self.subTest(ids=ids, operator=operator, listing=listing):
                    tree.mkdir(parents=True, exist_ok=True)
                    (tree / "retained").write_bytes(b"old evidence\0")
                    before = {p.name: p.read_bytes() for p in tree.iterdir()}
                    result = self.selected(ids, operator=operator, listing=listing, original=original)
                    self.assert_exit(result, 2)
                    self.assertIn("unknown selected mutation ID", result.stderr)
                    self.assertFalse((self.base / "invoked").exists())
                    self.assertEqual({p.name: p.read_bytes() for p in tree.iterdir()}, before)

    def test_valid_and_duplicate_ids_keep_one_attempt_and_restoration(self):
        for ids in [[], ["L2.bool.0"], ["L2.bool.0", "L2.bool.0"]]:
            with self.subTest(ids=ids):
                result = self.selected(ids)
                self.assert_exit(result, 0)
                self.assertIn("1 equivalent", result.stdout)
                self.assertIn("bool=0/1", result.stdout)
                self.assertEqual((self.base / "invoked").read_text(), "called\ncalled\n")
                self.assertEqual((self.base / "target/mutation-sweep/tree/sample.rs").read_bytes(), ORIGINAL.encode())

    def test_valid_listing_remains_complete_without_commands(self):
        original = ORIGINAL.replace("    true\n", "    true && false\n")
        result = self.selected(["L2.bool.0"], listing=True, original=original)
        self.assert_exit(result, 0)
        self.assertIn("L2.bool.0", result.stdout)
        self.assertIn("L2.bool.1", result.stdout)
        self.assertFalse((self.base / "invoked").exists())
        self.assertFalse((self.base / "target/mutation-sweep/tree").exists())

    def test_empty_discovery_without_selected_ids_remains_an_explicit_exploration(self):
        original = "fn sample() {}\n#[cfg(test)]\nmod tests {}\n"
        result = self.selected([], original=original, record="")
        self.assert_exit(result, 0)
        self.assertIn("none attempted", result.stdout)
        self.assertEqual((self.base / "invoked").read_text(), "called\n")
        self.assertEqual((self.base / "target/mutation-sweep/tree/sample.rs").read_bytes(), original.encode())


class BaselineEvidenceTests(SweepFixture):
    def failing_baseline(self, kind):
        source = self.base / "sample.rs"
        source.write_bytes(ORIGINAL.encode())
        stdout = (b"raw stdout \x00\xff\r\nfailures:\n    tests::omega\n    tests::alpha\n"
                  b"test result: FAILED. 0 passed; 2 failed; 0 ignored; 0 measured; 0 filtered out\n")
        if kind != "killed":
            stdout = b"raw stdout \x00\xff\r\n"
        stderr = (b"error: could not compile `synthetic`\n" if kind == "unviable" else b"") + b"raw stderr \xfe\x00\r\n"
        cargo = self.base / "bin/cargo"
        cargo.write_text('''#!/usr/bin/env python3
import os, pathlib, signal, sys, time
root = pathlib.Path(os.environ['PROBE_ROOT'])
assert pathlib.Path('sample.rs').read_bytes() == ''' + repr(ORIGINAL.encode()) + '''
assert os.environ['ARCHOGEN_SWEEP_SOURCE'] == str(root.resolve())
with (root / 'invoked').open('a') as out:
    out.write('called\\n')
os.write(1, ''' + repr(stdout) + ')\nos.write(2, ' + repr(stderr) + ')\n' +
                         ("time.sleep(20)\n" if kind == "timeout" else
                          "os.kill(os.getpid(), signal.SIGTERM)\n" if kind == "signal" else
                          "sys.exit(101)\n" if kind in {"killed", "unviable"} else "sys.exit(23)\n"))
        cargo.chmod(0o755)
        env = dict(os.environ, PROBE_ROOT=str(self.base),
                   PATH=str(self.base / "bin") + os.pathsep + os.environ["PATH"])
        result = subprocess.run(["bash", str(SCRIPT), "sample.rs", "--package", "synthetic",
                                 "--op", "bool", "--timeout", "1"], env=env,
                                capture_output=True, text=True, timeout=20)
        self.assert_exit(result, 2)
        self.assertEqual(source.read_bytes(), ORIGINAL.encode())
        self.assertEqual((self.base / "target/mutation-sweep/tree/sample.rs").read_bytes(), ORIGINAL.encode())
        return result, stdout, stderr

    def test_failed_baselines_retain_raw_streams_actual_status_and_finishing_names(self):
        receipts = self.base / "target/mutation-sweep/baseline-failures"
        for kind, outcome, status, names in [("killed", "killed", 101, ["tests::alpha", "tests::omega"]),
                                             ("unviable", "unviable", 101, []),
                                             ("unjudged", "unjudged", 23, []),
                                             ("signal", "unjudged", -signal.SIGTERM, []),
                                             ("timeout", "timeout", -signal.SIGKILL, []),
                                             ("killed", "killed", 101, ["tests::alpha", "tests::omega"])]:
            with self.subTest(kind=kind):
                retained = {p: p.read_bytes() for p in receipts.glob("*/*")}
                (self.base / "invoked").unlink(missing_ok=True)
                result, stdout, stderr = self.failing_baseline(kind)
                paths = set(receipts.glob("*/receipt.json")) - set(retained)
                self.assertEqual(len(paths), 1, result.stdout + result.stderr)
                path = paths.pop()
                receipt = json.loads(path.read_bytes())
                self.assertEqual(receipt["schema_version"], 1)
                self.assertEqual(receipt["source"], "sample.rs")
                self.assertEqual(receipt["source_sha256"], hashlib.sha256(ORIGINAL.encode()).hexdigest())
                self.assertEqual(receipt["command"], ["cargo", "test", "-q", "-p", "synthetic", "--"])
                self.assertEqual(receipt["cwd"], "target/mutation-sweep/tree")
                self.assertEqual(receipt["outcome"], outcome)
                self.assertEqual(receipt["exit_status"], status)
                self.assertEqual(receipt["failed_tests"], names)
                self.assertGreaterEqual(receipt["seconds"], 0)
                self.assertIn(str(path.relative_to(self.base)), result.stderr)
                for stream, expected in [("stdout", stdout), ("stderr", stderr)]:
                    record = receipt[stream]
                    relative = Path(record["path"])
                    self.assertFalse(relative.is_absolute())
                    self.assertNotIn("..", relative.parts)
                    data = (self.base / relative).read_bytes()
                    self.assertEqual(data, expected)
                    self.assertEqual(record["bytes"], len(expected))
                    self.assertEqual(record["sha256"], hashlib.sha256(expected).hexdigest())
                self.assertEqual((self.base / "invoked").read_text(), "called\n", "baseline must not retry")
                for old, data in retained.items():
                    self.assertEqual(old.read_bytes(), data, "a later failure replaced retained evidence")

    def test_actual_rust_test_baseline_retains_its_finishing_failure(self):
        original = ORIGINAL.replace("mod tests {}", "mod tests { #[test] fn actual_baseline_failure() { assert!(false); } }")
        (self.base / "sample.rs").write_text(original)
        cargo = self.base / "bin/cargo"
        cargo.write_text('''#!/usr/bin/env python3
import os, pathlib, subprocess, sys
root = pathlib.Path(os.environ['PROBE_ROOT'])
with (root / 'invoked').open('a') as out:
    out.write('called\\n')
binary = root / 'baseline-tests'
compiled = subprocess.run(['rustc', '--edition=2021', '--test', 'sample.rs', '-o', str(binary)], capture_output=True)
assert compiled.returncode == 0, compiled.stderr
result = subprocess.run([str(binary)], capture_output=True)
sys.stdout.buffer.write(result.stdout)
sys.stderr.buffer.write(result.stderr)
sys.exit(result.returncode)
''')
        cargo.chmod(0o755)
        env = dict(os.environ, PROBE_ROOT=str(self.base),
                   PATH=str(self.base / "bin") + os.pathsep + os.environ["PATH"])
        result = subprocess.run(["bash", str(SCRIPT), "sample.rs", "--package", "synthetic", "--op", "bool"],
                                env=env, capture_output=True, text=True, timeout=20)
        self.assert_exit(result, 2)
        paths = list((self.base / "target/mutation-sweep/baseline-failures").glob("*/receipt.json"))
        self.assertEqual(len(paths), 1)
        receipt = json.loads(paths[0].read_bytes())
        self.assertEqual((receipt["outcome"], receipt["exit_status"], receipt["failed_tests"]),
                         ("killed", 101, ["tests::actual_baseline_failure"]))
        stdout = (self.base / receipt["stdout"]["path"]).read_bytes()
        self.assertIn(b"thread 'tests::actual_baseline_failure'", stdout)
        self.assertIn(b"test result: FAILED.", stdout)
        self.assertEqual((self.base / "invoked").read_text(), "called\n")
        self.assertEqual((self.base / "sample.rs").read_bytes(), original.encode())
        self.assertEqual((self.base / "target/mutation-sweep/tree/sample.rs").read_bytes(), original.encode())

    def test_successful_baseline_does_not_emit_failure_receipts(self):
        result = self.run_sweep("from true\nto false\nwhy fixture\n")
        self.assert_exit(result, 0)
        self.assertFalse((self.base / "target/mutation-sweep/baseline-failures").exists())
        self.assertEqual((self.base / "invoked").read_text(), "called\ncalled\n")

    def test_evidence_write_failure_is_explicit_and_never_retries(self):
        blocked = self.base / "target/mutation-sweep/baseline-failures"
        blocked.parent.mkdir(parents=True)
        blocked.write_bytes(b"retained obstruction\0")
        result, _, _ = self.failing_baseline("killed")
        self.assertIn("could not retain baseline evidence", result.stderr)
        self.assertEqual(blocked.read_bytes(), b"retained obstruction\0")
        self.assertEqual((self.base / "invoked").read_text(), "called\n")


class SourceBoundaryTests(SweepFixture):
    def source_index(self, entries):
        cargo = self.base / "bin/cargo"
        cargo.write_text('''#!/usr/bin/env python3
import json, os, pathlib
root = pathlib.Path(os.environ['PROBE_ROOT'])
source = os.environ['PROBE_SOURCE']
with (root / 'observations').open('a') as out:
    out.write(json.dumps({'original': (root / source).read_text(),
                          'copy': pathlib.Path(source).read_text()}) + '\\n')
''')
        cargo.chmod(0o755)
        git = self.base / "bin/git"
        git.write_text(GIT.replace("b'100644 ' + b'0' * 40 + b' 0\\tsample.rs\\0'", repr(
            b"".join(mode.encode() + b" " + b"0" * 40 + b" " + stage.encode() + b"\t" + name.encode() + b"\0"
                     for mode, stage, name in entries))))
        git.chmod(0o755)

    def invoke_source(self, source, *, listing=False):
        env = dict(os.environ, PROBE_ROOT=str(self.base), PROBE_SOURCE=source,
                   PATH=str(self.base / "bin") + os.pathsep + os.environ["PATH"])
        return subprocess.run(["bash", str(SCRIPT), source, "--package", "synthetic", "--op", "bool"]
                              + (["--list"] if listing else []), env=env, capture_output=True,
                              text=True, timeout=20)

    def test_absolute_source_refuses_before_mutating_the_original(self):
        original = self.base / "sample.rs"
        original.write_text(ORIGINAL)
        self.source_index([("100644", "0", "sample.rs")])
        result = self.invoke_source(str(original))
        self.assert_exit(result, 2)
        self.assertIn("source path", result.stderr)
        self.assertFalse((self.base / "observations").exists())
        self.assertEqual(original.read_bytes(), ORIGINAL.encode())
        self.assertFalse((self.base / "target/mutation-sweep/tree").exists())

    def test_unsafe_sources_refuse_before_copy_or_command_including_listing(self):
        original = self.base / "sample.rs"
        original.write_text(ORIGINAL)
        (self.base / "untracked.rs").write_text(ORIGINAL)
        for name in ["gitlink.rs", "indexed-link.rs", "conflicted.rs"]:
            (self.base / name).write_text(ORIGINAL)
        (self.base / "directory.rs").mkdir()
        (self.base / "linked.rs").symlink_to("sample.rs")
        (self.base / "misdeclared.rs").symlink_to("sample.rs")
        (self.base / "linked-parent").symlink_to(".")
        tree = self.base / "target/mutation-sweep/tree"
        tree.mkdir(parents=True)
        (tree / "retained").write_bytes(b"previous evidence\0")
        self.source_index([("100644", "0", "sample.rs"), ("120000", "0", "linked.rs"),
                           ("100644", "0", "misdeclared.rs"), ("100644", "0", "linked-parent/sample.rs"),
                           ("100644", "0", "directory.rs"), ("160000", "0", "gitlink.rs"),
                           ("100644", "0", "missing.rs"), ("120000", "0", "indexed-link.rs"),
                           ("100644", "1", "conflicted.rs")])
        cases = ["", "nested//sample.rs", str(original), "../" + self.base.name + "/sample.rs", "nested/../sample.rs",
                 "./sample.rs", "untracked.rs", "linked.rs", "misdeclared.rs",
                 "linked-parent/sample.rs", "directory.rs", "gitlink.rs", "indexed-link.rs", "missing.rs", "conflicted.rs"]
        for source in cases:
            for listing in [False, True]:
                with self.subTest(source=source, listing=listing):
                    result = self.invoke_source(source, listing=listing)
                    self.assert_exit(result, 2)
                    self.assertIn("source path", result.stderr)
                    self.assertFalse((self.base / "observations").exists())
                    self.assertEqual(original.read_bytes(), ORIGINAL.encode())
                    self.assertEqual(list(tree.iterdir()), [tree / "retained"])
                    self.assertEqual((tree / "retained").read_bytes(), b"previous evidence\0")
        self.assertTrue((self.base / "linked.rs").is_symlink())
        self.assertTrue((self.base / "linked-parent").is_symlink())

    def test_regular_indexed_relative_sources_mutate_only_the_copy_and_restore(self):
        for source, mode in [("sample.rs", "100644"), ("space ' quote/line\nµ.rs", "100755")]:
            with self.subTest(source=source, mode=mode):
                original = self.base / source
                original.parent.mkdir(parents=True, exist_ok=True)
                original.write_bytes(ORIGINAL.encode())
                original.chmod(int(mode[-3:], 8))
                self.source_index([(mode, "0", source)])
                (self.base / "observations").unlink(missing_ok=True)
                listing = self.invoke_source(source, listing=True)
                self.assert_exit(listing, 0)
                self.assertIn("L2.bool.0", listing.stdout)
                self.assertFalse((self.base / "observations").exists())
                result = self.invoke_source(source)
                self.assert_exit(result, 1)
                self.assertIn("1 survived", result.stdout)
                observed = [json.loads(row) for row in (self.base / "observations").read_text().splitlines()]
                self.assertEqual(observed, [{"original": ORIGINAL, "copy": ORIGINAL},
                                           {"original": ORIGINAL, "copy": ORIGINAL.replace("    true\n", "    false\n")}])
                copied = self.base / "target/mutation-sweep/tree" / source
                self.assertEqual(original.read_bytes(), ORIGINAL.encode())
                self.assertEqual(copied.read_bytes(), ORIGINAL.encode())
                self.assertEqual(original.stat().st_mode & 0o777, int(mode[-3:], 8))
                self.assertEqual(copied.stat().st_mode & 0o777, int(mode[-3:], 8))


class SourceHintTests(SweepFixture):
    def test_own_canonical_root_replaces_missing_empty_relative_and_unrelated_hints(self):
        for hint in [None, "", "not-a-repository", str(ROOT)]:
            with self.subTest(inherited=hint):
                result = self.run_sweep("from true\nto false\nwhy fixture\n", source_hint=hint)
                self.assert_exit(result, 0)
                self.assertEqual((self.base / "invoked").read_text(), "called\ncalled\n")


class OperatorCountTests(SweepFixture):
    def test_real_compiler_outcomes_count_only_attempted_operators(self):
        original = ("fn sample(values: &[i32], needle: i32) -> bool {\n"
                    "    let gate = true;\n    gate && values.contains(&needle)\n}\n"
                    "#[cfg(test)]\nmod tests {\n    #[test]\n    fn middle() { assert!(super::sample(&[2, 7, 13], 7)); }\n}\n")
        (self.base / "sample.rs").write_text(original)
        cargo = self.base / "bin/cargo"
        cargo.write_text('''#!/usr/bin/env python3
import json, os, pathlib, subprocess, sys
root = pathlib.Path(os.environ['PROBE_ROOT'])
binary = root / 'probe-tests'
compile = subprocess.run(['rustc', '--edition=2021', '--test', 'sample.rs', '-o', str(binary)], capture_output=True, text=True)
record = {'compile': compile.returncode, 'stderr': compile.stderr}
if compile.returncode:
    print(compile.stderr, file=sys.stderr)
    print('error: could not compile `synthetic`', file=sys.stderr)
    code = compile.returncode
else:
    run = subprocess.run([str(binary)], capture_output=True, text=True)
    print(run.stdout, end='')
    print(run.stderr, end='', file=sys.stderr)
    record['test'] = run.returncode
    code = run.returncode
with (root / 'outcomes').open('a') as out:
    out.write(json.dumps(record) + '\\n')
sys.exit(code)
''')
        cargo.chmod(0o755)
        env = dict(os.environ, PROBE_ROOT=str(self.base), TMPDIR=str(self.base),
                   CARGO_HOME=str(ROOT / "target/cargo-home"),
                   PATH=str(self.base / "bin") + os.pathsep + os.environ["PATH"])
        command = ["bash", str(SCRIPT), "sample.rs", "--package", "synthetic", "--op", "bool", "--op", "contains",
                   "--timeouts-fail"]
        listing = subprocess.run(command + ["--list"], env=env, capture_output=True, text=True, timeout=20)
        self.assert_exit(listing, 0)
        self.assertNotIn("unviable by operator", listing.stdout)
        self.assertFalse((self.base / "outcomes").exists(), "listing invoked the compiler")
        ids = re.findall(r"^(L\d+\.(?:bool|contains)\.\d+) ", listing.stdout, re.M)
        self.assertEqual(len(ids), 5)
        refused = subprocess.run(command + ["--only", "not-a-candidate"], env=env,
                                 capture_output=True, text=True, timeout=20)
        self.assert_exit(refused, 2)
        self.assertIn("unknown selected mutation ID", refused.stderr)
        self.assertFalse((self.base / "outcomes").exists(), "unknown selection invoked the compiler")
        self.assertFalse((self.base / "target/mutation-sweep/tree").exists())
        self.assertEqual((self.base / "sample.rs").read_bytes(), original.encode())
        cases = [([], "bool=0/1, contains=2/4", (6, 2, 3)),
                 (["--only", ids[0]], "bool=0/1", (2, 0, 1)),
                 (["--only", ids[2]], "contains=1/1", (2, 1, 0))]
        for selection, summary, expected in cases:
            with self.subTest(selection=selection):
                (self.base / "outcomes").unlink(missing_ok=True)
                result = subprocess.run(command + selection, env=env, capture_output=True, text=True, timeout=30)
                self.assert_exit(result, 0)
                self.assertIn("mutation-sweep: unviable by operator: " + summary + " (unviable/attempted)", result.stdout)
                outcomes = [json.loads(line) for line in (self.base / "outcomes").read_text().splitlines()]
                observed = (len(outcomes), sum(row["compile"] != 0 for row in outcomes),
                            sum(row.get("test", 0) != 0 for row in outcomes))
                self.assertEqual(observed, expected)
                self.assertEqual(outcomes[0].get("test"), 0, "baseline must pass and stay outside counts")
                self.assertTrue(all(row["compile"] == 1 and "error[E0308]" in row["stderr"]
                                    for row in outcomes if row["compile"] != 0))
                self.assertEqual((self.base / "sample.rs").read_bytes(), original.encode())
                self.assertEqual((self.base / "target/mutation-sweep/tree/sample.rs").read_bytes(), original.encode())


class DiscoveryFixture(SweepFixture):
    def discovery(self, source, operator="iter"):
        # Load the actual discovery/replacement functions without invoking the CLI or copying its algorithm.
        python = SCRIPT.read_text().split("<<'PY'\n", 1)[1].rsplit("\nPY", 1)[0]
        functions = [node for node in ast.parse(python).body if isinstance(node, ast.FunctionDef)]
        lines = source.split("\n")
        namespace = dict(re=re, json=json, bisect=bisect, lines=lines, first=0, last=lines.index("#[cfg(test)]"))
        exec(compile(ast.Module(body=functions, type_ignores=[]), str(SCRIPT), "exec"), namespace)
        candidates = [m for m in namespace["mutants"]() if m[2] == operator]
        return namespace, candidates

    def bounded(self, command, *, expect_type_error=False):
        env = dict(os.environ, TMPDIR=str(self.base), CARGO_HOME=str(ROOT / "target/cargo-home"))
        process = subprocess.Popen(command, cwd=self.base, env=env, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, text=True, start_new_session=True)
        try:
            stdout, stderr = process.communicate(timeout=60)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.communicate()
            self.fail("compiled sweep probe command timed out: " + repr(command))
        self.assertEqual(process.returncode, 1 if expect_type_error else 0, stdout + stderr)
        if expect_type_error:
            self.assertEqual(command[0], "rustc")
            self.assertRegex(stderr, r"error\[E(?:0308|0277)\]")
        return stdout


class ObserverCloseTests(DiscoveryFixture):
    # These are premise controls outside the semantic parent mutation population. A changed
    # producer/caller/source must invalidate the proof, not become an artificial killing test.
    def test_close_reason_requires_current_production_and_caller_premises(self):
        expected = {
            'xtask/src/trust_compiler_observer.rs': '58b1a5e56f61488cb3d6eefc6bea0809f076d378fa93d4a04df84649caf7ac98',
            'xtask/src/trust_compiler.rs': '9dbac3db6e10908fa1028d7464adcd6af092468f6ed455cadcafb07e787e0db2',
            'xtask/src/trust.rs': 'd4b2ae51fa78f79c770cb7c9f84dbcf06578be12735461c328a28e509e8ab88c',
        }
        def verify(inputs):
            for path, digest in expected.items():
                production = inputs[path].split('\n#[cfg(test)]\n', 1)[0]
                self.assertEqual(hashlib.sha256(production.encode()).hexdigest(), digest,
                                 'changed close-proof source/caller requires revalidation: ' + path)
        inputs = {path: (ROOT / path).read_text() for path in expected}
        verify(inputs)
        for path in inputs:
            with self.subTest(path=path), self.assertRaisesRegex(AssertionError, 'requires revalidation'):
                verify({**inputs, path: '// changed premise\n' + inputs[path]})
        source = inputs['xtask/src/trust_compiler_observer.rs']
        namespace, candidates = self.discovery(source, 'stmt')
        key = ('drop(file);', '// drop(file);')
        family = [m for m in candidates if namespace['mutation_key'](m) == key]
        self.assertEqual(len(family), 1, 'require the actual unique native omission')
        records = namespace['load_equivalents'](ROOT / 'xtask/trust_compiler_observer.equivalents')
        self.assertEqual(set(records), {key})
        self.assertIn('channel = "1.95.0"', (ROOT / 'rust-toolchain.toml').read_text())

    @unittest.skipUnless(sys.platform == 'darwin', 'native close trace uses the public Mach-O interface')
    def test_actual_close_omission_preserves_completed_protocol_but_changes_timing(self):
        source = (ROOT / 'xtask/src/trust_compiler_observer.rs').read_text()
        namespace, candidates = self.discovery(source, 'stmt')
        candidate, = [m for m in candidates if namespace['mutation_key'](m) == ('drop(file);', '// drop(file);')]
        self.assertIn('rustc 1.95.0 ', self.bounded(['rustc', '-vV']))
        expected = subprocess.check_output(['rustc', '-vV'], cwd=self.base)
        library = self.base / 'close.dylib'
        self.bounded(['cc', '-Wall', '-Wextra', '-Werror', '-dynamiclib',
                      str(ROOT / 'scripts/fixtures/compiler_provenance_close.c'), '-o', str(library)])
        for label, text in [('original', source), ('omitted', namespace['apply_mutant'](candidate))]:
            path = self.base / (label + '.rs')
            path.write_text(text)
            self.bounded(['rustc', '--edition=2021', '--crate-name', 'observer_close', '-O', str(path),
                          '-o', str(self.base / label)])
        results = {}
        for mode in ['normal', 'pass', 'link-error', 'unlink-error', 'close-error']:
            for label in ['original', 'omitted']:
                with self.subTest(mode=mode, label=label):
                    case = self.base / (mode + '-' + label)
                    records = case / 'records'
                    records.mkdir(parents=True)
                    trace = case / 'trace'
                    env = dict(os.environ, ARCHOGEN_COMPILER_RECORDS=str(records),
                               ARCHOGEN_COMPILER_SESSION='close-proof', ARCHOGEN_COMPILER_ROLE='root')
                    for name in ['DYLD_INSERT_LIBRARIES', 'LD_PRELOAD', 'ARCHOGEN_TEST_IO_ERROR',
                                 'ARCHOGEN_TEST_LINK_ERROR', 'ARCHOGEN_TEST_UNLINK_ERROR',
                                 'ARCHOGEN_TEST_CLOSE_ERROR']:
                        env.pop(name, None)
                    if mode != 'normal':
                        env.update(DYLD_INSERT_LIBRARIES='close.dylib', ARCHOGEN_TEST_IO_TRACE=str(trace))
                        if mode.endswith('-error'):
                            env['ARCHOGEN_TEST_' + mode.upper().replace('-', '_')] = '1'
                    process = subprocess.Popen([str(self.base / label), 'rustc', '-vV'], cwd=self.base,
                                               env=env, stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                               start_new_session=True)
                    try:
                        stdout, stderr = process.communicate(timeout=45)
                    except subprocess.TimeoutExpired:
                        os.killpg(process.pid, signal.SIGKILL)
                        process.communicate()
                        self.fail('actual close observer timed out')
                    failed = mode in ['link-error', 'unlink-error']
                    self.assertEqual(process.returncode, 2 if failed else 0, stderr)
                    self.assertEqual(stdout, expected)
                    self.assertEqual(bool(stderr), failed)
                    names = sorted(path.name for path in records.iterdir())
                    partial, completed = '.' + str(process.pid) + '.part', str(process.pid) + '.json'
                    self.assertEqual(names, sorted([partial, completed] if mode == 'unlink-error'
                                                  else [partial] if mode == 'link-error' else [completed]))
                    normalized = []
                    for path in records.iterdir():
                        actual = json.loads(path.read_bytes())
                        wanted = dict(argv_hex=[b'rustc'.hex(), b'-vV'.hex()],
                                      cwd_hex=os.fsencode(self.base).hex(), exit=0, pid=process.pid,
                                      role_hex=b'root'.hex(), schema='archogen-compiler-provenance/1',
                                      session_hex=b'close-proof'.hex(), signal=None)
                        self.assertEqual(actual, wanted)
                        actual.pop('pid')
                        normalized.append((path.name.endswith('.part'), actual))
                    lines = trace.read_text().splitlines() if trace.exists() else []
                    self.assertTrue(all('pid=' + str(process.pid) + ' ' in line for line in lines), lines)
                    events = [line.split()[0] for line in lines]
                    if mode == 'normal':
                        self.assertEqual(events, [])
                    else:
                        tail = ['link'] if mode == 'link-error' else ['link', 'unlink']
                        wanted = (['loaded', 'full-sync', 'closed'] + tail if label == 'original'
                                  else ['loaded', 'full-sync'] + tail + ['closed'])
                        self.assertEqual(events, wanted)
                        sync_fd = lines[1].split('fd=')[1]
                        self.assertGreaterEqual(int(sync_fd), 0)
                        for line in lines:
                            if line.startswith(('link ', 'unlink ')):
                                self.assertEqual(line.split('fd=')[1], '-1' if label == 'original' else sync_fd)
                    results[mode, label] = (sorted(normalized, key=lambda row: row[0]), stderr, events)
            a, b = results[mode, 'original'], results[mode, 'omitted']
            self.assertEqual(a[:2], b[:2], 'independent protocol expectations must agree')
            with self.assertRaisesRegex(AssertionError, 'wrong negative protocol oracle'):
                self.assertNotEqual(a[:2], b[:2], 'wrong negative protocol oracle')
            if mode != 'normal':
                with self.assertRaisesRegex(AssertionError, 'overbroad timing equivalence'):
                    self.assertEqual(a[2], b[2], 'overbroad timing equivalence')


class LogicTests(DiscoveryFixture):
    def test_each_operand_deletion_compiles_and_matches_its_boolean_truth_table(self):
        cases = [
            ("a && b && c", [lambda a, b, c: b and c, lambda a, b, c: a and c, lambda a, b, c: a and b],
             lambda a, b, c: a and b and c),
            ("a || b || c", [lambda a, b, c: b or c, lambda a, b, c: a or c, lambda a, b, c: a or b],
             lambda a, b, c: a or b or c),
            ("a && b || c", [lambda a, b, c: b or c, lambda a, b, c: a or c, lambda a, b, c: a and b],
             lambda a, b, c: a and b or c),
            ("a || b && c", [lambda a, b, c: b and c, lambda a, b, c: a and c, lambda a, b, c: a or b],
             lambda a, b, c: a or b and c),
        ]
        programs, assertions = [], []
        for case, (condition, expected, baseline) in enumerate(cases):
            source = ("fn sample(a: bool, b: bool, c: bool) -> bool {\n"
                      f"    if {condition} {{\n        true\n    }} else {{ false }}\n}}\n"
                      "#[cfg(test)]\nmod tests {}\n")
            namespace, candidates = self.discovery(source, "logic")
            self.assertEqual(len(candidates), 5)  # Two connective flips precede the three operand deletions.
            candidates = candidates[2:]
            for number, (mutant, oracle) in enumerate(zip([None] + candidates, [baseline] + expected)):
                name = f"logic_{case}_{number}"
                text = source if mutant is None else namespace["apply_mutant"](mutant)
                programs.append(text.split("#[cfg(test)]", 1)[0].replace("fn sample", "fn " + name, 1))
                for a in [False, True]:
                    for b in [False, True]:
                        for c in [False, True]:
                            values = ", ".join(str(x).lower() for x in [a, b, c])
                            assertions.append(f'assert_eq!({name}({values}), {str(oracle(a, b, c)).lower()}, "{name}");')
        program = self.base / "logic.rs"
        program.write_text("#![allow(unused_variables)]\n" + "\n".join(programs)
                           + "\nfn main() {\n" + "\n".join(assertions) + "\n}\n")
        binary = self.base / "logic"
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.bounded([str(binary)])
        self.assertEqual(len(assertions), 128)
        print("logic probes: 12 actual candidates and four baselines compile; 128 truth-table observations")

    def test_middle_deletions_land_and_restore_through_real_cli(self):
        for condition in ["a && b && c", "a || b || c", "a && b || c", "a || b && c"]:
            source = ("fn sample(a: bool, b: bool, c: bool) -> bool {\n"
                      f"    if {condition} {{\n        true\n    }} else {{ false }}\n}}\n"
                      "#[cfg(test)]\nmod tests {}\n")
            namespace, candidates = self.discovery(source, "logic")
            mutant = candidates[3]
            before, after = namespace["mutation_key"](mutant)
            record = "from-json " + json.dumps(before) + "\nto-json " + json.dumps(after) + "\nwhy fixture\n"
            result = self.run_sweep(record, original=source, mutated=namespace["apply_mutant"](mutant), candidate=mutant)
            self.assert_exit(result, 0)
            self.assertEqual((self.base / "invoked").read_text(), "called\ncalled\n")


class ClosureTests(DiscoveryFixture):
    def test_actual_closure_candidates_compile_and_select_the_expected_values(self):
        p = lambda x: x > 0
        q = lambda x: x < 10
        r = lambda x: x != 7
        both = lambda x: p(x) and q(x)
        either = lambda x: p(x) or q(x)
        cases = [
            ("|x| *x > 0 && *x < 10", both, [q, p]),
            ("|x| *x > 0 || *x < 10", either, [q, p]),
            ("|x| *x > 0 && *x < 10 || *x != 7", lambda x: both(x) or r(x),
             [lambda x: q(x) or r(x), lambda x: p(x) or r(x), both]),
            ("|x| ((*x > 0 && *x < 10))", both, [q, p]),
            ("|x| { *x > 0 && *x < 10 }", both, [q, p]),
            ("|x| { return *x > 0 && *x < 10; }", both, [q, p]),
            ("|x: &i32| -> bool { *x > 0 && *x < 10 }", both, [q, p]),
            ("move |x|*x>0&&*x<10", both, [q, p]),
            ("|x| (|| *x > 0 && *x < 10)()", both, [q, p]),
            ("|x| std::convert::identity::<fn(i32) -> bool,>(positive)(*x) && *x < 10", both, [q, p]),
            ('|x| *x > 0 && "x&&y" == "x&&y" && *x < 10', both, [q, both, p]),
            ("|x| (0..2).any(|_| *x > 0 && *x < 10) && *x != 7", lambda x: both(x) and r(x),
             [r, both, lambda x: q(x) and r(x), lambda x: p(x) and r(x)]),
            ("|x| { let y = *x; y > 0 && y < 10 }", both, [q, p]),
            ("|x| **std::convert::identity::<&&i32>(&x) > 0 && *x < 10", both, [q, p]),
            ("|x| *x > 0 || *x < 10 && *x != 7", lambda x: p(x) or q(x) and r(x),
             [lambda x: q(x) and r(x), lambda x: p(x) and r(x), either]),
            ('|x| *x > 0 && "x||y" == "x||y"', p, [lambda x: True, p]),
        ]
        programs, assertions, total = ["fn positive(x: i32) -> bool { x > 0 }"], [], 0
        for case, (closure, baseline, expected) in enumerate(cases):
            source = ("fn sample(values: Vec<i32>) -> Vec<i32> {\n"
                      f"    values.into_iter().filter({closure}).collect()\n}}\n#[cfg(test)]\nmod tests {{}}\n")
            namespace, candidates = self.discovery(source, "clogic")
            self.assertEqual(len(candidates), len(expected), closure)
            total += len(candidates)
            for number, (mutant, oracle) in enumerate(zip([None] + candidates, [baseline] + expected)):
                name = f"closure_{case}_{number}"
                text = source if mutant is None else namespace["apply_mutant"](mutant)
                programs.append(text.split("#[cfg(test)]", 1)[0].replace("fn sample", "fn " + name, 1))
                for values in [[], [7], [-1, 2, 7, 13, 20]]:
                    selected = [x for x in values if oracle(x)]
                    assertions.append(f'assert_eq!({name}(vec!{values}), vec!{selected}, "{name}");')
        self.assertEqual((total, len(assertions)), (37, 159))
        program = self.base / "closures.rs"
        program.write_text("#![allow(unused_variables, unused_parens)]\n" + "\n".join(programs)
                           + "\nfn main() {\n" + "\n".join(assertions) + "\n}\n")
        binary = self.base / "closures"
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.bounded([str(binary)])
        print("closure probes: 37 actual candidates and 16 baselines compile; 159 value-selection observations")

    def test_literal_comment_multiline_and_test_scopes_are_excluded(self):
        source = r'''fn sample<'a>(values: &'a [i32], a: bool, b: bool) {
    let _ghost = r#"|x| *x > 0 && *x < 10"#;
    /* outer /* nested |x| a && b */ || a || b */
    let _bitwise = 1 | 2;
    let _logical = a || b;
    let _simple = || a;
    let _multiline = |x: i32|
        x > 0 && x < 10;
    let _ = values.iter().filter(|x| "a && b" == "a && b" && **x > 0).count();
}
#[cfg(test)]
mod tests { fn fixture() { let _ = |x: i32| x > 0 && x < 10; } }
'''
        namespace, candidates = self.discovery(source, "clogic")
        self.assertEqual(len(candidates), 2)
        self.assertTrue(all(m[:3] == (8, 9, "clogic") for m in candidates))
        for number, mutant in enumerate([None] + candidates):
            text = source if mutant is None else namespace["apply_mutant"](mutant)
            program = self.base / f"closure_scope_{number}.rs"
            program.write_text(text.split("#[cfg(test)]", 1)[0] + "\nfn main() { sample(&[2, 7], true, false); }\n")
            binary = self.base / f"closure_scope_{number}"
            self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
            self.bounded([str(binary)])

    def test_closure_spans_land_and_restore_through_real_cli(self):
        for closure in ["|x| *x > 0 && *x < 10 || *x != 7",
                        "|x: &i32| -> bool { *x > 0 && *x < 10 }"]:
            source = ("fn sample(values: Vec<i32>) -> Vec<i32> {\n"
                      f"    values.into_iter().filter({closure}).collect()\n}}\n#[cfg(test)]\nmod tests {{}}\n")
            namespace, candidates = self.discovery(source, "clogic")
            self.assertEqual(len(candidates), 3 if "||" in closure else 2)
            for mutant in candidates:
                before, after = namespace["mutation_key"](mutant)
                record = "from-json " + json.dumps(before) + "\nto-json " + json.dumps(after) + "\nwhy fixture\n"
                result = self.run_sweep(record, original=source, mutated=namespace["apply_mutant"](mutant), candidate=mutant)
                self.assert_exit(result, 0)
                self.assertEqual((self.base / "invoked").read_text(), "called\ncalled\n")



class CommandDelimiterTests(DiscoveryFixture):
    @unittest.skipUnless(os.name == "posix", "the proof is scoped to Cargo's Unix formatter")
    def test_actual_delimiter_candidates_hold_only_the_fixed_cargo_domain(self):
        current = (ROOT / "xtask/src/trust.rs").read_text()
        self.assertNotIn("fn split_command(", current.split("#[cfg(test)]", 1)[0],
                         "delimiter parsing must remain outside production provenance")
        start = current.index("    fn split_command(")
        end = current.index("    /// Frame Cargo", start)
        source = (textwrap.dedent(current[start:end]) + "\n/// Frame Cargo: historical fixture boundary\n"
                  + "#[cfg(test)]\nmod tests {}\n")
        self.assertIn("commit-hash: f2d3ce0bd7f24a49f8f72d9000448f8838c4e850",
                      self.bounded(["cargo", "--version", "--verbose"]),
                      "a producer change requires revalidating the proof")
        namespace, _ = self.discovery(source)
        lines = namespace["lines"]
        first = source[:source.index("fn split_command(")].count("\n")
        last = source[:source.index("/// Frame Cargo")].count("\n")
        keys = {
            ("let mut any = false;", "let mut any = true;"),
            ("if any || !cur.is_empty() {", "if true {"),
            ("any = false;", "any = true;"),
            ("any = false;", "// any = false;"),
        }
        family = [m for m in namespace["mutants"]()
                  if first <= m[0] < last and namespace["mutation_key"](m) in keys]
        self.assertEqual(len(family), 5, "both guards and the three state edits must be actual candidates")
        self.assertEqual({namespace["mutation_key"](m) for m in family}, keys)

        def function(text, name):
            start = text.index("fn split_command(")
            end = text.index("\n/// Frame Cargo", start)
            return text[start:end].replace("fn split_command(", "fn " + name + "(", 1)

        functions = [function(source, "baseline")]
        functions += [function(namespace["apply_mutant"](m), "variant_" + str(i))
                      for i, m in enumerate(family)]
        # Independent encoded inputs/argv expectations. Actual producer controls live in trust::tests;
        # these compiler-backed controls hold the exact replacements, not another escape implementation.
        cases = [
            ("rustc", ["rustc"]),
            ("rustc ''", ["rustc", ""]),
            ("rustc '' --cfg ''", ["rustc", "", "--cfg", ""]),
            ("rustc --out-dir 'a b'", ["rustc", "--out-dir", "a b"]),
            ("rustc --out-dir 'a\n     Running `fake`\rb\tc'",
             ["rustc", "--out-dir", "a\n     Running `fake`\rb\tc"]),
            (r"rustc 'a'\''b' 'c'\!'d' '\literal'",
             ["rustc", "a'b", "c!d", r"\literal"]),
            ("rustc 'λ' '\\ntext'", ["rustc", "λ", r"\ntext"]),
        ]
        literal = lambda text: json.dumps(text, ensure_ascii=False)
        checks = []
        for command, expected in cases:
            expected_rust = "[" + ", ".join(map(literal, expected)) + "]"
            for name in ["baseline"] + ["variant_" + str(i) for i in range(len(family))]:
                checks.append(f'assert_eq!({name}({literal(command)}), {expected_rust}, "{name} domain");')
        for i, mutant in enumerate(family):
            before, after = namespace["mutation_key"](mutant)
            if before == "let mut any = false;" or before.startswith("if ") and lines[mutant[0]].startswith("    if "):
                command, original, changed = "", [], [""]
            elif before.startswith("if "):
                command, original, changed = " rustc", ["rustc"], ["", "rustc"]
            elif after == "any = true;":
                command, original, changed = "rustc ", ["rustc"], ["rustc", ""]
            else:
                command, original, changed = "rustc '' ", ["rustc", ""], ["rustc", "", ""]
            for name, expected in [("baseline", original), ("variant_" + str(i), changed)]:
                expected_rust = "[" + ", ".join(map(literal, expected)) + "]"
                # The empty expected value needs an explicit type for Rust's assert_eq inference.
                expected_rust = "Vec::<String>::new()" if not expected else expected_rust
                checks.append(f'assert_eq!({name}({literal(command)}), {expected_rust}, "{name} outside");')
            checks.append(f'assert_ne!(baseline({literal(command)}), variant_{i}({literal(command)}), "overbroad proof {i}");')
        guard = next(i for i, m in enumerate(family)
                     if namespace["mutation_key"](m)[0].startswith("if ") and not lines[m[0]].startswith("    if "))
        main = ("\nfn main() {\n" + "\n".join(checks)
                + '\nmatch std::env::args().nth(1).as_deref() {\n'
                + f' Some("wrong-positive") => assert_eq!(variant_{guard}(" rustc"), baseline(" rustc"), "wrong positive oracle"),\n'
                + ' Some("wrong-negative") => assert_ne!(variant_0("rustc \'\'"), baseline("rustc \'\'"), "wrong negative oracle"),\n'
                + ' _ => {}\n}\nprintln!("delimiter controls: encoded boundaries held; wider-domain claims refused");\n}\n')
        program, binary = self.base / "cargo-delimiters.rs", self.base / "cargo-delimiters"
        program.write_text("\n".join(functions) + main)
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertIn("encoded boundaries held; wider-domain claims refused", self.bounded([str(binary)]))
        for argument, message in [("wrong-positive", "wrong positive oracle"),
                                  ("wrong-negative", "wrong negative oracle")]:
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), argument])
        records = namespace["load_equivalents"](ROOT / "xtask/trust.retired-equivalents")
        active = namespace["load_equivalents"](ROOT / "xtask/trust.equivalents")
        self.assertEqual(set(records), keys, "preserve every retired delimiter reason exactly")
        self.assertTrue(keys.isdisjoint(active), "historical formatter proofs cannot classify production")
        for mutant in family:
            self.assertIn(namespace["mutation_key"](mutant), records,
                          "each historical delimiter proof retains its reason")


class DiagnosticFlagTests(DiscoveryFixture):
    def producer_argv(self, label, *, level=None, separate=None, operand="unused_variables",
                      library_source="pub fn c() {}\n", release_strip=None, release_incremental=None):
        """One real pinned Cargo compilation, with an independently specified argv shape."""
        project = self.base / ("producer-" + label)
        (project / "src").mkdir(parents=True)
        manifest = ('[workspace]\n[package]\nname = "flagproof"\nversion = "0.1.0"\n'
                    'edition = "2021"\n')
        if level is not None:
            manifest += f'[lints.rust]\nunused_variables = "{level}"\n'
        profile = []
        if release_strip is not None:
            profile.append("strip = " + json.dumps(release_strip))
        if release_incremental is not None:
            profile.append("incremental = " + json.dumps(release_incremental))
        if profile:
            manifest += "[profile.release]\n" + "\n".join(profile) + "\n"
        (project / "Cargo.toml").write_text(manifest)
        (project / "src/lib.rs").write_text(library_source)
        target = project / "target"
        env = {key: os.environ[key] for key in ("PATH", "HOME")}
        env.update(RUSTUP_HOME=os.environ.get("RUSTUP_HOME", str(Path(env["HOME"]) / ".rustup")),
                   RUSTUP_TOOLCHAIN="1.95.0", RUSTC="rustc", CARGO_HOME=str(project / "cargo-home"),
                   CARGO_TARGET_DIR=str(target), TMPDIR=str(self.base), TMP=str(self.base), TEMP=str(self.base))

        def cargo(arguments):
            process = subprocess.Popen(["cargo"] + arguments, cwd=project, env=env,
                                       stdout=subprocess.PIPE, stderr=subprocess.PIPE,
                                       text=True, start_new_session=True)
            try:
                stdout, stderr = process.communicate(timeout=60)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.communicate()
                self.fail("actual Cargo flag producer timed out: " + label)
            self.assertEqual(process.returncode, 0, stdout + stderr)
            return stderr

        cargo(["generate-lockfile", "--offline"])
        arguments = ["rustc" if separate else "build", "--lib", "--release", "--locked", "--offline",
                     "--no-default-features", "--message-format=json", "-v", "-p", "flagproof"]
        if separate:
            arguments += ["--", separate, operand]
        stderr = cargo(arguments)
        commands = [line.strip()[len("Running `"):-1] for line in stderr.splitlines()
                    if line.strip().startswith("Running `rustc ") and line.strip().endswith("`")]
        self.assertEqual(len(commands), 1, stderr)
        # The normalization helper is under proof, not the command decoder. shlex is an independent
        # Unix decoder for these generated simple paths and the two known quoted cfg operands.
        argv = shlex.split(commands[0])
        metadata = [arg for arg in argv if re.fullmatch(r"metadata=[0-9a-f]{16}", arg)]
        filename = [arg for arg in argv if re.fullmatch(r"extra-filename=-[0-9a-f]{16}", arg)]
        self.assertEqual(len(metadata), 1, argv)
        self.assertEqual(len(filename), 1, argv)
        expected = ["rustc", "--crate-name", "flagproof", "--edition=2021", "src/lib.rs",
                    "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat",
                    "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3",
                    "-C", "embed-bitcode=no"]
        if level:
            expected += ["--" + level + "=unused_variables"]
        expected += ["--check-cfg", "cfg(docsrs,test)", "--check-cfg", "cfg(feature, values())",
                     "-C", metadata[0], "-C", filename[0], "--out-dir", str(target / "release/deps")]
        if release_incremental:
            expected += ["-C", "incremental=" + str(target / "release/incremental")]
        expected += ["-C", "strip=debuginfo", "-L", "dependency=" + str(target / "release/deps")]
        if separate:
            expected += [separate, operand]
        self.assertEqual(argv, expected, "actual pinned producer argv requires revalidation on drift")
        return argv

    def color_override_argv(self, label, color):
        """Keep the ordinary request, raw override request and actual compiler argv separate."""
        ordinary = self.producer_argv(label)
        project = self.base / ("producer-" + label)
        compiler_dir = project / "bin"
        compiler_dir.mkdir()
        compiler = compiler_dir / "color-rustc"
        compiler.write_text(r'''#!/usr/bin/env python3
import json, os, sys
from pathlib import Path
arguments = [arg for arg in sys.argv[1:] if not arg.startswith(("--json=", "--error-format="))]
if "--crate-name" in arguments and arguments[arguments.index("--crate-name") + 1] == "flagproof" and "src/lib.rs" in arguments:
    with Path(__file__).with_name("forwarded.jsonl").open("a") as output:
        output.write(json.dumps(["rustc"] + arguments) + "\n")
os.execvp("rustc", ["rustc"] + arguments)
''')
        compiler.chmod(0o755)
        target = project / "color-target"
        self.assertFalse(target.exists(), "the wider build must not reuse a prior artifact")
        env = {key: os.environ[key] for key in ("PATH", "HOME")}
        env.update(PATH=str(compiler_dir) + os.pathsep + env["PATH"],
                   RUSTUP_HOME=os.environ.get("RUSTUP_HOME", str(Path(env["HOME"]) / ".rustup")),
                   RUSTUP_TOOLCHAIN="1.95.0", RUSTC="rustc", CARGO_HOME=str(project / "color-cargo-home"),
                   CARGO_TARGET_DIR=str(target), TMPDIR=str(self.base), TMP=str(self.base), TEMP=str(self.base))
        command = ["cargo", "rustc", "--lib", "--release", "--locked", "--offline",
                   "--no-default-features", "--message-format=json", "-v", "-p", "flagproof",
                   "--", "--color", color]

        def cargo(expected):
            process = subprocess.Popen(command, cwd=project, env=env, stdout=subprocess.PIPE,
                                       stderr=subprocess.PIPE, text=True, start_new_session=True)
            try:
                stdout, stderr = process.communicate(timeout=60)
            except subprocess.TimeoutExpired:
                os.killpg(process.pid, signal.SIGKILL)
                process.communicate()
                self.fail("actual color Cargo producer timed out: " + label)
            self.assertEqual(process.returncode, expected, stdout + stderr)
            return stderr

        refused = cargo(101)
        self.assertIn("cannot specify the `--color` option with `--json`", refused)
        self.assertFalse(list((target / "release/deps").glob("libflagproof-*.rlib")),
                         "a compiler-refused combination is not a successful wider build")
        env["RUSTC"] = "color-rustc"
        stderr = cargo(0)
        commands = [line.strip()[len("Running `"):-1] for line in stderr.splitlines()
                    if line.strip().startswith("Running `color-rustc ") and line.strip().endswith("`")]
        self.assertEqual(len(commands), 1, stderr)
        raw = shlex.split(commands[0])
        metadata = [arg for arg in raw if re.fullmatch(r"metadata=[0-9a-f]{16}", arg)]
        filename = [arg for arg in raw if re.fullmatch(r"extra-filename=-[0-9a-f]{16}", arg)]
        self.assertEqual(len(metadata), 1, raw)
        self.assertEqual(len(filename), 1, raw)
        expected = ["color-rustc", "--crate-name", "flagproof", "--edition=2021", "src/lib.rs",
                    "--error-format=json", "--json=diagnostic-rendered-ansi,artifacts,future-incompat",
                    "--crate-type", "lib", "--emit=dep-info,metadata,link", "-C", "opt-level=3",
                    "-C", "embed-bitcode=no", "--check-cfg", "cfg(docsrs,test)", "--check-cfg",
                    "cfg(feature, values())", "-C", metadata[0], "-C", filename[0], "--out-dir",
                    str(target / "release/deps"), "-C", "strip=debuginfo", "-L",
                    "dependency=" + str(target / "release/deps"), "--color", color]
        self.assertEqual(raw, expected, "independent raw Cargo request")
        records = [json.loads(line) for line in (compiler_dir / "forwarded.jsonl").read_text().splitlines()]
        self.assertEqual(len(records), 1, "one physical compilation, excluding version/probing calls")
        forwarded = records[0]
        # Independent compiler-argv prefix and the already checked, unchanged tail. The raw Cargo
        # formatter options are not described as arguments executed by this forwarding compiler.
        physical = ["rustc", "--crate-name", "flagproof", "--edition=2021", "src/lib.rs"] + expected[7:]
        self.assertEqual(forwarded, physical, "independent actual compiler argv")
        self.assertNotEqual(raw, forwarded)
        suffix = filename[0].split("=", 1)[1]
        artifact = target / "release/deps" / ("libflagproof" + suffix + ".rlib")
        self.assertTrue(artifact.read_bytes().startswith(b"!<arch>\n"), "actual new library archive")
        consumer, binary = project / "artifact-consumer.rs", project / "artifact-consumer"
        consumer.write_text("fn main() { flagproof::c(); }\n")
        self.bounded(["rustc", "--edition=2021", str(consumer), "--extern", "flagproof=" + str(artifact),
                      "-o", str(binary)])
        self.bounded([str(binary)])
        return ordinary, raw, forwarded

    @unittest.skipUnless(os.name == "posix", "the pinned producer/decoder premise is Unix")
    def test_actual_lint_table_candidates_hold_only_the_inventory_caller_domain(self):
        source = (ROOT / "xtask/src/trust.rs").read_text()
        self.assertIn('c.env("RUSTC", "rustc");', source)
        self.assertIn("commit-hash: f2d3ce0bd7f24a49f8f72d9000448f8838c4e850",
                      self.bounded(["cargo", "--version", "--verbose"]),
                      "a changed producer requires source/caller revalidation")
        namespace, candidates = self.discovery(source, "bool")
        flags = ["-W", "-A", "-D", "-F", "--warn", "--allow", "--deny", "--forbid"]
        family = []
        for flag in flags:
            key = (f'("{flag}", true),', f'("{flag}", false),')
            matches = [m for m in candidates if namespace["mutation_key"](m) == key]
            self.assertEqual(len(matches), 1, "the consuming-bit switch must be an actual unique candidate")
            family += matches

        def module(text, name):
            start = text.index("const DROPPED:")
            end = text.index("/// `path` resolved", start)
            body = text[start:end]
            self.assertEqual(body.count("fn normalise("), 1)
            return "mod " + name + " {\n" + body.replace("fn normalise(", "pub fn normalise(", 1) + "}\n"

        modules = [module(source, "baseline")]
        modules += [module(namespace["apply_mutant"](mutant), "variant_" + str(index))
                    for index, mutant in enumerate(family)]
        # This expected vector comes from the pinned compiler command contract, independently of normalise.
        expected = ["--crate-name", "flagproof", "--edition=2021", "src/lib.rs", "--crate-type", "lib",
                    "--emit=dep-info,metadata,link", "-C opt-level=3", "-C embed-bitcode=no", "-C strip=debuginfo"]
        literal = lambda value: json.dumps(value, ensure_ascii=False)
        rust_vector = lambda values: "vec![" + ", ".join(literal(value) + ".to_owned()" for value in values) + "]"
        checks = []
        names = ["baseline"] + ["variant_" + str(index) for index in range(len(family))]
        ordinary = None
        for level in [None, "allow", "warn", "deny", "forbid"]:
            argv = self.producer_argv(level or "ordinary", level=level)
            if level is None:
                ordinary = argv
            for name in names:
                checks.append(f'assert_eq!({name}::normalise(&{rust_vector(argv)}, "unused-scratch/"), '
                              f'{rust_vector(expected)}, "{name} actual {level or "ordinary"}");')
        wider = []
        for index, flag in enumerate(flags):
            # These succeed in real Cargo/rustc. The inventory itself never invokes cargo rustc or
            # supplies trailing compiler arguments, which is the essential boundary of the proof.
            argv = self.producer_argv("separate-" + str(index), separate=flag)
            wider.append(argv)
            checks += [f'assert_eq!(baseline::normalise(&{rust_vector(argv)}, "unused-scratch/"), '
                       f'{rust_vector(expected)}, "baseline wider {flag}");',
                       f'assert_eq!(variant_{index}::normalise(&{rust_vector(argv)}, "unused-scratch/"), '
                       f'{rust_vector(expected + ["unused_variables"])}, "variant wider {flag}");']
            # A bare key in an operand is not a visited key: another unchanged table entry consumes it.
            # This helper-boundary control makes no claim that its synthetic argv is a Cargo build.
            consumed = ["rustc", "--out-dir", flag, "retained"]
            for name in ["baseline", "variant_" + str(index)]:
                checks.append(f'assert_eq!({name}::normalise(&{rust_vector(consumed)}, "unused-scratch/"), '
                              f'{rust_vector(["retained"])}, "{name} consumed value {flag}");')
        main = ("fn main() {\n" + "\n".join(checks)
                + '\nmatch std::env::args().nth(1).as_deref() {\n'
                + f' Some("wrong-positive") => assert_eq!(baseline::normalise(&{rust_vector(wider[0])}, "unused-scratch/"), '
                + f'variant_0::normalise(&{rust_vector(wider[0])}, "unused-scratch/"), "wrong positive oracle"),\n'
                + f' Some("wrong-negative") => assert_ne!(baseline::normalise(&{rust_vector(ordinary)}, "unused-scratch/"), '
                + f'variant_0::normalise(&{rust_vector(ordinary)}, "unused-scratch/"), "wrong negative oracle"),\n'
                + ' _ => {}\n}\nprintln!("lint controls: actual argv held; wider-domain claims refused");\n}\n')
        program, binary = self.base / "lint-candidates.rs", self.base / "lint-candidates"
        program.write_text("\n".join(modules) + main)
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertIn("actual argv held; wider-domain claims refused", self.bounded([str(binary)]))
        for argument, message in [("wrong-positive", "wrong positive oracle"),
                                  ("wrong-negative", "wrong negative oracle")]:
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), argument])
        records = namespace["load_equivalents"](ROOT / "xtask/trust.equivalents")
        for mutant in family:
            self.assertIn(namespace["mutation_key"](mutant), records,
                          "each actual lint proof requires its own per-file reason")


    def codegen_key_controls(self, key, consumed_operand, *, incremental=False):
        """Actual constructor edits, independently specified producer values and scoped counterexamples."""
        self.assertIn("commit-hash: f2d3ce0bd7f24a49f8f72d9000448f8838c4e850",
                      self.bounded(["cargo", "-vV"]))
        source = (ROOT / "xtask/src/trust.rs").read_text()
        namespace, directions = self.discovery(source, operator="dir")
        _, literals = self.discovery(source, operator="lit")
        original = f'|| v.starts_with("{key}=")'
        line = next(index for index, text in enumerate(source.split("\n")) if text.strip() == original)
        keys = [("dir", (original, f'|| v.contains("{key}=")')),
                ("lit", (original, f'|| v.starts_with("{key}")'))]
        family = [m for m in directions + literals if m[0] == line
                  and (m[2], namespace["mutation_key"](m)) in keys]
        self.assertEqual([(m[2], namespace["mutation_key"](m)) for m in family], keys)

        def module(text, name):
            first = text.index("const DROPPED:")
            last = text.index("\n/// `path` resolved lexically", first)
            body = text[first:last]
            self.assertEqual(body.count("fn normalise("), 1)
            return "mod " + name + " {\n" + body.replace("fn normalise(", "pub fn normalise(", 1) + "}\n"

        modules = [module(source, "baseline")]
        modules += [module(namespace["apply_mutant"](m), "variant_" + str(index))
                    for index, m in enumerate(family)]
        operand = "llvm-args=--pass-remarks=" + key + "=inside"
        producer = dict(library_source="pub fn c() -> u32 { 37 }\n")
        if incremental:
            producer["release_incremental"] = True
        ordinary = self.producer_argv(key + "-ordinary", **producer)
        wider = self.producer_argv(key + "-wider", separate="-C", operand=operand, **producer)
        if incremental:
            for label, argv in [("ordinary", ordinary), ("wider", wider)]:
                cache = self.base / ("producer-" + key + "-" + label) / "target/release/incremental"
                self.assertIn("incremental=" + str(cache), argv)
                self.assertTrue(cache.is_dir(), cache)
                self.assertTrue(any(path.is_file() for path in cache.rglob("*")),
                                "the enabled compiler did not populate its explicit local cache")
        self.assertEqual(wider[-2:], ["-C", operand])
        with self.assertRaisesRegex(AssertionError, "wrong forwarded compiler argv"):
            self.assertEqual(wider[-2:], [], "wrong forwarded compiler argv")
        for label, argv in [("ordinary", ordinary), ("wider", wider)]:
            extra = next(arg.removeprefix("extra-filename=") for arg in argv
                         if re.fullmatch(r"extra-filename=-[0-9a-f]{16}", arg))
            artifact = (self.base / ("producer-" + key + "-" + label) / "target/release/deps"
                        / ("libflagproof" + extra + ".rlib"))
            self.assertTrue(artifact.is_file(), artifact)
            consumer = self.base / (key + "-consumer-" + label + ".rs")
            binary = self.base / (key + "-consumer-" + label)
            consumer.write_text('fn main() { assert_eq!(flagproof::c(), 37); println!("{}", flagproof::c()); }\n')
            self.bounded(["rustc", "--edition=2021", "--crate-name", "codegen_consumer", str(consumer),
                          "--extern", "flagproof=" + str(artifact), "-o", str(binary)])
            self.assertEqual(self.bounded([str(binary)]), "37\n")
        with self.assertRaisesRegex(AssertionError, re.escape(
                "incorrect value `" + key + "=inside` for codegen option `strip`")):
            self.producer_argv(key + "-refused-strip", release_strip=key + "=inside")

        expected = ["--crate-name", "flagproof", "--edition=2021", "src/lib.rs", "--crate-type", "lib",
                    "--emit=dep-info,metadata,link", "-C opt-level=3", "-C embed-bitcode=no", "-C strip=debuginfo"]
        vector = lambda values: ("vec![" + ", ".join(json.dumps(value) + ".to_owned()" for value in values) + "]"
                                 if values else "Vec::<String>::new()")
        checks = []
        for name in ["baseline", "variant_0", "variant_1"]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(ordinary)}, "unused-scratch/"), {vector(expected)});')
        for name, output in [("baseline", expected + ["-C " + operand]), ("variant_0", expected),
                             ("variant_1", expected + ["-C " + operand])]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(wider)}, "unused-scratch/"), {vector(output)});')
        # These helper-boundary inputs are explicitly not successful compiler producers.
        synthetic = ["rustc", "-C", key + "X=inside"]
        for name, output in [("baseline", ["-C " + key + "X=inside"]),
                             ("variant_0", ["-C " + key + "X=inside"]), ("variant_1", [])]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(synthetic)}, "unused-scratch/"), {vector(output)});')
        refused = ["rustc", "-C", "strip=" + key + "=inside"]
        for name, output in [("baseline", ["-C strip=" + key + "=inside"]), ("variant_0", []),
                             ("variant_1", ["-C strip=" + key + "=inside"])]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(refused)}, "unused-scratch/"), {vector(output)});')
        consumed = ["rustc", "-C", consumed_operand, "retained"]
        for name in ["baseline", "variant_0", "variant_1"]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(consumed)}, "unused-scratch/"), {vector(["retained"])});')
        main = ("fn main() {\n" + "\n".join(checks)
                + '\nmatch std::env::args().nth(1).as_deref() {\n'
                + f' Some("wrong-positive") => assert_eq!(baseline::normalise(&{vector(wider)}, "unused-scratch/"), '
                + f'variant_0::normalise(&{vector(wider)}, "unused-scratch/"), "wrong positive oracle"),\n'
                + f' Some("wrong-negative") => assert_ne!(baseline::normalise(&{vector(ordinary)}, "unused-scratch/"), '
                + f'variant_1::normalise(&{vector(ordinary)}, "unused-scratch/"), "wrong negative oracle"),\n'
                + f' Some("unsupported-helper") => assert_eq!(baseline::normalise(&{vector(synthetic)}, "unused-scratch/"), '
                + f'variant_1::normalise(&{vector(synthetic)}, "unused-scratch/"), "unsupported helper input oracle"),\n'
                + ' _ => {}\n}\nprintln!("codegen controls: actual variants and consumed compiler artifacts held");\n}\n')
        program, binary = self.base / (key + "-candidates.rs"), self.base / (key + "-candidates")
        program.write_text("\n".join(modules) + main)
        self.bounded(["rustc", "--edition=2021", "--crate-name", "codegen_candidates", str(program), "-o", str(binary)])
        self.assertIn("actual variants and consumed compiler artifacts held", self.bounded([str(binary)]))
        for argument, message in [("wrong-positive", "wrong positive oracle"),
                                  ("wrong-negative", "wrong negative oracle"),
                                  ("unsupported-helper", "unsupported helper input oracle")]:
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), argument])
        return namespace, family

    @unittest.skipUnless(os.name == "posix", "the pinned producer/decoder premise is Unix")
    def test_actual_extra_filename_candidates_require_pinned_codegen_constructors(self):
        namespace, family = self.codegen_key_controls(
            "extra-filename", "incremental=cache/extra-filename=inside")
        records = namespace["load_equivalents"](ROOT / "xtask/trust.equivalents")
        for mutant in family:
            self.assertIn(namespace["mutation_key"](mutant), records,
                          "each actual extra-filename proof requires its own per-file reason")


    @unittest.skipUnless(os.name == "posix", "the pinned producer/decoder premise is Unix")
    def test_actual_incremental_candidates_require_pinned_codegen_constructors(self):
        namespace, family = self.codegen_key_controls(
            "incremental", "metadata=incremental=inside", incremental=True)
        records = namespace["load_equivalents"](ROOT / "xtask/trust.equivalents")
        for mutant in family:
            self.assertIn(namespace["mutation_key"](mutant), records,
                          "each actual incremental proof requires its own per-file reason")


    @unittest.skipUnless(os.name == "posix", "the pinned producer/decoder premise is Unix")
    def test_actual_metadata_candidates_require_pinned_codegen_constructors(self):
        self.assertIn("commit-hash: f2d3ce0bd7f24a49f8f72d9000448f8838c4e850",
                      self.bounded(["cargo", "-vV"]))
        source = (ROOT / "xtask/src/trust.rs").read_text()
        namespace, directions = self.discovery(source, operator="dir")
        _, literals = self.discovery(source, operator="lit")
        line = next(index for index, text in enumerate(source.split("\n"))
                    if text.strip() == 'if v.starts_with("metadata=")')
        keys = [("dir", ('if v.starts_with("metadata=")', 'if v.contains("metadata=")')),
                ("lit", ('if v.starts_with("metadata=")', 'if v.starts_with("metadata")'))]
        family = [mutant for mutant in directions + literals if mutant[0] == line
                  and (mutant[2], namespace["mutation_key"](mutant)) in keys]
        self.assertEqual([(m[2], namespace["mutation_key"](m)) for m in family], keys)

        def module(text, name):
            first = text.index("const DROPPED:")
            last = text.index("\n/// `path` resolved lexically", first)
            body = text[first:last]
            self.assertEqual(body.count("fn normalise("), 1)
            return "mod " + name + " {\n" + body.replace("fn normalise(", "pub fn normalise(", 1) + "}\n"

        modules = [module(source, "baseline")]
        modules += [module(namespace["apply_mutant"](m), "variant_" + str(index))
                    for index, m in enumerate(family)]
        operand = "llvm-args=--pass-remarks=metadata=inside"
        ordinary = self.producer_argv("metadata-ordinary", library_source="pub fn c() -> u32 { 37 }\n")
        wider = self.producer_argv("metadata-wider", separate="-C", operand=operand,
                                   library_source="pub fn c() -> u32 { 37 }\n")
        # Public Cargo forwarded these literal arguments; an oracle silently omitting them must fail.
        self.assertEqual(wider[-2:], ["-C", operand])
        with self.assertRaisesRegex(AssertionError, "wrong forwarded compiler argv"):
            self.assertEqual(wider[-2:], [], "wrong forwarded compiler argv")
        for label, argv in [("ordinary", ordinary), ("wider", wider)]:
            extra = next(arg.removeprefix("extra-filename=") for arg in argv
                         if re.fullmatch(r"extra-filename=-[0-9a-f]{16}", arg))
            artifact = (self.base / ("producer-metadata-" + label) / "target/release/deps"
                        / ("libflagproof" + extra + ".rlib"))
            self.assertTrue(artifact.is_file(), artifact)
            consumer = self.base / ("metadata-consumer-" + label + ".rs")
            binary = self.base / ("metadata-consumer-" + label)
            consumer.write_text('fn main() { assert_eq!(flagproof::c(), 37); println!("{}", flagproof::c()); }\n')
            self.bounded(["rustc", "--edition=2021", "--crate-name", "metadata_consumer", str(consumer),
                          "--extern", "flagproof=" + str(artifact), "-o", str(binary)])
            self.assertEqual(self.bounded([str(binary)]), "37\n")

        # Cargo forwards strip strings, but a failed compilation never reaches inventory normalise.
        # This is a refused producer/premise control, never a successful wider-domain example.
        with self.assertRaisesRegex(AssertionError, re.escape(
                "incorrect value `metadata=inside` for codegen option `strip`")):
            self.producer_argv("metadata-refused-strip", release_strip="metadata=inside")

        # Independent literal contract: never use baseline normalise as the expected result.
        expected = ["--crate-name", "flagproof", "--edition=2021", "src/lib.rs", "--crate-type", "lib",
                    "--emit=dep-info,metadata,link", "-C opt-level=3", "-C embed-bitcode=no", "-C strip=debuginfo"]
        vector = lambda values: ("vec![" + ", ".join(json.dumps(value) + ".to_owned()" for value in values) + "]"
                                 if values else "Vec::<String>::new()")
        checks = []
        for name in ["baseline", "variant_0", "variant_1"]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(ordinary)}, "unused-scratch/"), {vector(expected)});')
        for name, output in [("baseline", expected + ["-C " + operand]), ("variant_0", expected),
                             ("variant_1", expected + ["-C " + operand])]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(wider)}, "unused-scratch/"), {vector(output)});')
        # Explicit helper-boundary premise controls, not successful compiler invocations.
        synthetic = ["rustc", "-C", "metadataX=inside"]
        for name, output in [("baseline", ["-C metadataX=inside"]), ("variant_0", ["-C metadataX=inside"]),
                             ("variant_1", [])]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(synthetic)}, "unused-scratch/"), {vector(output)});')
        refused_argv = ["rustc", "-C", "strip=metadata=inside"]
        for name, output in [("baseline", ["-C strip=metadata=inside"]), ("variant_0", []),
                             ("variant_1", ["-C strip=metadata=inside"])]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(refused_argv)}, "unused-scratch/"), {vector(output)});')
        consumed = ["rustc", "-C", "incremental=cache/metadata=inside", "retained"]
        for name in ["baseline", "variant_0", "variant_1"]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(consumed)}, "unused-scratch/"), {vector(["retained"])});')
        main = ("fn main() {\n" + "\n".join(checks)
                + '\nmatch std::env::args().nth(1).as_deref() {\n'
                + f' Some("wrong-positive") => assert_eq!(baseline::normalise(&{vector(wider)}, "unused-scratch/"), '
                + f'variant_0::normalise(&{vector(wider)}, "unused-scratch/"), "wrong positive oracle"),\n'
                + f' Some("wrong-negative") => assert_ne!(baseline::normalise(&{vector(ordinary)}, "unused-scratch/"), '
                + f'variant_1::normalise(&{vector(ordinary)}, "unused-scratch/"), "wrong negative oracle"),\n'
                + f' Some("unsupported-helper") => assert_eq!(baseline::normalise(&{vector(synthetic)}, "unused-scratch/"), '
                + f'variant_1::normalise(&{vector(synthetic)}, "unused-scratch/"), "unsupported helper input oracle"),\n'
                + ' _ => {}\n}\nprintln!("metadata controls: actual variants and consumed compiler artifacts held");\n}\n')
        program, binary = self.base / "metadata-candidates.rs", self.base / "metadata-candidates"
        program.write_text("\n".join(modules) + main)
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertIn("actual variants and consumed compiler artifacts held", self.bounded([str(binary)]))
        for argument, message in [("wrong-positive", "wrong positive oracle"),
                                  ("wrong-negative", "wrong negative oracle"),
                                  ("unsupported-helper", "unsupported helper input oracle")]:
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), argument])
        records = namespace["load_equivalents"](ROOT / "xtask/trust.equivalents")
        for mutant in family:
            self.assertIn(namespace["mutation_key"](mutant), records,
                          "each actual metadata proof requires its own per-file reason")


    @unittest.skipUnless(os.name == "posix", "the pinned producer/decoder premise is Unix")
    def test_actual_cap_lints_candidate_requires_local_unit_premises(self):
        source = (ROOT / "xtask/src/trust.rs").read_text()
        self.assertIn("commit-hash: f2d3ce0bd7f24a49f8f72d9000448f8838c4e850",
                      self.bounded(["cargo", "--version", "--verbose"]),
                      "a changed producer requires source/caller revalidation")
        namespace, candidates = self.discovery(source, "bool")
        key = ('("--cap-lints", true),', '("--cap-lints", false),')
        family = [mutant for mutant in candidates if namespace["mutation_key"](mutant) == key]
        self.assertEqual(len(family), 1, "the cap consuming-bit edit must be an actual unique candidate")

        def module(text, name):
            start = text.index("const DROPPED:")
            end = text.index("/// `path` resolved", start)
            body = text[start:end]
            self.assertEqual(body.count("fn normalise("), 1)
            return "mod " + name + " {\n" + body.replace("fn normalise(", "pub fn normalise(", 1) + "}\n"

        modules = [module(source, "baseline"), module(namespace["apply_mutant"](family[0]), "variant")]
        expected = ["--crate-name", "flagproof", "--edition=2021", "src/lib.rs", "--crate-type", "lib",
                    "--emit=dep-info,metadata,link", "-C opt-level=3", "-C embed-bitcode=no", "-C strip=debuginfo"]
        literal = lambda value: json.dumps(value, ensure_ascii=False)
        vector = lambda values: "vec![" + ", ".join(literal(value) + ".to_owned()" for value in values) + "]"
        checks = []
        local = None
        for level in [None, "warn"]:
            argv = self.producer_argv("cap-local-" + (level or "ordinary"), level=level)
            self.assertNotIn("--cap-lints", argv, "actual local path package must show warnings")
            if level is None:
                local = argv
            for name in ["baseline", "variant"]:
                checks.append(f'assert_eq!({name}::normalise(&{vector(argv)}, "unused-scratch/"), '
                              f'{vector(expected)}, "{name} actual local {level or "ordinary"}");')
        # The inventory never supplies these trailing compiler arguments. Cargo/rustc accepts them,
        # so this wider-domain difference cannot be explained away as a compiler refusal.
        wider = self.producer_argv("cap-explicit", separate="--cap-lints", operand="allow")
        checks += [f'assert_eq!(baseline::normalise(&{vector(wider)}, "unused-scratch/"), '
                   f'{vector(expected)}, "baseline explicit cap");',
                   f'assert_eq!(variant::normalise(&{vector(wider)}, "unused-scratch/"), '
                   f'{vector(expected + ["allow"])}, "variant explicit cap");']
        consumed = ["rustc", "--out-dir", "--cap-lints", "retained"]
        for name in ["baseline", "variant"]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(consumed)}, "unused-scratch/"), '
                          f'{vector(["retained"])}, "{name} consumed key value");')
        main = ("fn main() {\n" + "\n".join(checks)
                + '\nmatch std::env::args().nth(1).as_deref() {\n'
                + f' Some("wrong-positive") => assert_eq!(baseline::normalise(&{vector(wider)}, "unused-scratch/"), '
                + f'variant::normalise(&{vector(wider)}, "unused-scratch/"), "wrong positive oracle"),\n'
                + f' Some("wrong-negative") => assert_ne!(baseline::normalise(&{vector(local)}, "unused-scratch/"), '
                + f'variant::normalise(&{vector(local)}, "unused-scratch/"), "wrong negative oracle"),\n'
                + ' _ => {}\n}\nprintln!("cap controls: actual local argv held; explicit-cap equivalence refused");\n}\n')
        program, binary = self.base / "cap-candidate.rs", self.base / "cap-candidate"
        program.write_text("\n".join(modules) + main)
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertIn("actual local argv held; explicit-cap equivalence refused", self.bounded([str(binary)]))
        for argument, message in [("wrong-positive", "wrong positive oracle"),
                                  ("wrong-negative", "wrong negative oracle")]:
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), argument])
        records = namespace["load_equivalents"](ROOT / "xtask/trust.equivalents")
        self.assertIn(key, records, "the actual cap proof requires its per-file reason")


    @unittest.skipUnless(os.name == "posix", "the pinned producer/decoder premise is Unix")
    def test_actual_color_candidate_requires_the_fixed_inventory_formatter(self):
        source = (ROOT / "xtask/src/trust.rs").read_text()
        self.assertIn('c.env("RUSTC", "rustc");', source, "the forwarding mode is outside this caller")
        self.assertIn("commit-hash: f2d3ce0bd7f24a49f8f72d9000448f8838c4e850",
                      self.bounded(["cargo", "--version", "--verbose"]),
                      "a changed producer requires source/caller revalidation")
        namespace, candidates = self.discovery(source, "bool")
        key = ('("--color", true),', '("--color", false),')
        family = [mutant for mutant in candidates if namespace["mutation_key"](mutant) == key]
        self.assertEqual(len(family), 1, "the color consuming-bit edit must be an actual unique candidate")

        def module(text, name):
            start = text.index("const DROPPED:")
            end = text.index("/// `path` resolved", start)
            body = text[start:end]
            self.assertEqual(body.count("fn normalise("), 1)
            return "mod " + name + " {\n" + body.replace("fn normalise(", "pub fn normalise(", 1) + "}\n"

        modules = [module(source, "baseline"), module(namespace["apply_mutant"](family[0]), "variant")]
        expected = ["--crate-name", "flagproof", "--edition=2021", "src/lib.rs", "--crate-type", "lib",
                    "--emit=dep-info,metadata,link", "-C opt-level=3", "-C embed-bitcode=no", "-C strip=debuginfo"]
        literal = lambda value: json.dumps(value, ensure_ascii=False)
        vector = lambda values: "vec![" + ", ".join(literal(value) + ".to_owned()" for value in values) + "]"
        checks = []
        first = None
        for color in ["always", "never", "auto"]:
            ordinary, raw, forwarded = self.color_override_argv("color-" + color, color)
            self.assertNotIn("--color", ordinary)
            if first is None:
                first = ordinary, raw, forwarded
            for name in ["baseline", "variant"]:
                checks.append(f'assert_eq!({name}::normalise(&{vector(ordinary)}, "unused-scratch/"), '
                              f'{vector(expected)}, "{name} actual ordinary");')
            # Raw Cargo request and physically forwarded compiler argv are distinct, checked inputs.
            # Only the forwarded argv is described as executed by rustc in this successful wider mode.
            for description, argv in [("raw Cargo request", raw), ("physical compiler", forwarded)]:
                checks += [f'assert_eq!(baseline::normalise(&{vector(argv)}, "unused-scratch/"), '
                           f'{vector(expected)}, "baseline {description} {color}");',
                           f'assert_eq!(variant::normalise(&{vector(argv)}, "unused-scratch/"), '
                           f'{vector(expected + [color])}, "variant {description} {color}");']
        consumed = ["rustc", "--out-dir", "--color", "retained"]
        for name in ["baseline", "variant"]:
            checks.append(f'assert_eq!({name}::normalise(&{vector(consumed)}, "unused-scratch/"), '
                          f'{vector(["retained"])}, "{name} consumed key value");')
        ordinary, raw, forwarded = first
        main = ("fn main() {\n" + "\n".join(checks)
                + '\nmatch std::env::args().nth(1).as_deref() {\n'
                + f' Some("wrong-positive") => assert_eq!(baseline::normalise(&{vector(forwarded)}, "unused-scratch/"), '
                + f'variant::normalise(&{vector(forwarded)}, "unused-scratch/"), "wrong positive oracle"),\n'
                + f' Some("wrong-negative") => assert_ne!(baseline::normalise(&{vector(ordinary)}, "unused-scratch/"), '
                + f'variant::normalise(&{vector(ordinary)}, "unused-scratch/"), "wrong negative oracle"),\n'
                + f' Some("wrong-forwarded") => assert_eq!({vector(raw)}, {vector(forwarded)}, "wrong forwarded argv oracle"),\n'
                + ' _ => {}\n}\nprintln!("color controls: ordinary/raw/physical argv held; wider claims refused");\n}\n')
        program, binary = self.base / "color-candidate.rs", self.base / "color-candidate"
        program.write_text("\n".join(modules) + main)
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertIn("ordinary/raw/physical argv held; wider claims refused", self.bounded([str(binary)]))
        for argument, message in [("wrong-positive", "wrong positive oracle"),
                                  ("wrong-negative", "wrong negative oracle"),
                                  ("wrong-forwarded", "wrong forwarded argv oracle")]:
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), argument])
        records = namespace["load_equivalents"](ROOT / "xtask/trust.equivalents")
        self.assertIn(key, records, "the actual color proof requires its per-file reason")


class BasenameComparisonTests(DiscoveryFixture):
    def test_actual_basename_comparison_uses_distinct_same_string_positions(self):
        source = (ROOT / "xtask/src/trust.rs").read_text()
        namespace, comparisons = self.discovery(source, operator="cmp")
        original = "(Some(dash), Some(dot)) if dash < dot => {"
        replacement = "(Some(dash), Some(dot)) if dash <= dot => {"
        family = [m for m in comparisons if namespace["mutation_key"](m) == (original, replacement)]
        self.assertEqual(len(family), 1)

        def module(text, name):
            first = text.index("const DROPPED:")
            last = text.index("\n/// `path` resolved lexically", first)
            body = text[first:last]
            self.assertIn("match (file.rfind('-'), file.rfind('.'))", body)
            self.assertEqual(body.count("fn normalise("), 1)
            return "mod " + name + " {\n" + body.replace("fn normalise(", "pub fn normalise(", 1) + "}\n"

        modules = [module(source, "original"), module(namespace["apply_mutant"](family[0]), "variant")]
        program, binary = self.base / "basename-candidates.rs", self.base / "basename-candidates"
        program.write_text("\n".join(modules) + r'''
fn original_config(file: &str) -> Vec<String> {
    original::normalise(&vec!["rustc".to_owned(), "--extern".to_owned(), format!("common={file}")], "")
}
fn variant_config(file: &str) -> Vec<String> {
    variant::normalise(&vec!["rustc".to_owned(), "--extern".to_owned(), format!("common={file}")], "")
}
fn search_property(file: &str, first: char, second: char) {
    if let (Some(a), Some(b)) = (file.rfind(first), file.rfind(second)) {
        assert_ne!(a, b, "distinct search pattern premise");
        assert_eq!(a < b, a <= b, "comparison predicate under unequal indices");
    }
}
fn main() {
    let cases = [
        ("", ""), ("plain", "plain"), ("-", "-"), (".", "."), ("-.", "."),
        ("a.b-c", "a.b-c"), ("a-.b", "a.b"),
        ("libcommon-abcd.rlib", "libcommon.rlib"),
        ("cache=one=two/libcommon-abcd.rmeta", "libcommon.rmeta"),
        ("dir-with.dots/name-123.so", "name.so"),
        ("a-foo-123.ext.more", "a-foo.more"), ("é-Ω.λ", "é.λ"),
    ];
    for (file, stem) in cases {
        let expected = vec![format!("--extern common={stem}")];
        assert_eq!(original_config(file), expected, "independent original output {file:?}");
        assert_eq!(variant_config(file), expected, "independent actual edit output {file:?}");
        search_property(file.rsplit('/').next().unwrap(), '-', '.');
    }
    // A bounded helper-input grid supplements the source proof; it is not an exhaustive universe claim.
    let mut level = vec![String::new()];
    let mut visited = 0usize;
    for _ in 0..=6 {
        for file in &level {
            search_property(file, '-', '.');
            assert_eq!(original_config(file), variant_config(file));
            visited += 1;
        }
        level = level.iter().flat_map(|prefix| ['a', '-', '.', 'é'].map(|ch| format!("{prefix}{ch}"))).collect();
    }
    assert_eq!(visited, 5461);
    match std::env::args().nth(1).as_deref() {
        Some("wrong-negative") => assert_ne!(original_config("libcommon-abcd.rlib"),
                                             variant_config("libcommon-abcd.rlib"), "wrong negative oracle"),
        // This relaxes the search premise of the predicate argument only. It does not claim that
        // changing the search in normalise must change its final string output.
        Some("same-search") => search_property("a.b", '.', '.'),
        Some("wrong-broad") => {
            let equal = "a.b".rfind('.').unwrap();
            assert_eq!(equal < equal, equal <= equal, "wrong arbitrary-index comparison oracle");
        }
        _ => {}
    }
    println!("basename controls: literal outputs and 5461 generated strings held");
}
''')
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertIn("literal outputs and 5461 generated strings held", self.bounded([str(binary)]))
        for argument, message in [("wrong-negative", "wrong negative oracle"),
                                  ("same-search", "distinct search pattern premise"),
                                  ("wrong-broad", "wrong arbitrary-index comparison oracle")]:
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), argument])
        records = namespace["load_equivalents"](ROOT / "xtask/trust.equivalents")
        self.assertIn(namespace["mutation_key"](family[0]), records,
                      "the actual basename comparison proof requires its own per-file reason")


class IteratorTests(DiscoveryFixture):
    def test_cargo_absolute_prefix_edits_preserve_line_selection(self):
        import hashlib

        source = (ROOT / "xtask/src/trust.rs").read_text()
        gate = (ROOT / "xtask/src/trust_gate.rs").read_text()
        main = (ROOT / "xtask/src/main.rs").read_text()
        controller = (ROOT / "xtask/src/trust_compiler.rs").read_text()
        observer = (ROOT / "xtask/src/trust_compiler_observer.rs").read_text()
        artifact = (ROOT / "xtask/src/trust_artifact.rs").read_text()
        catalog = (ROOT / "xtask/src/catalog_build.rs").read_text()
        environment = catalog[catalog.index("pub fn environment("):catalog.index("\nfn command(")]

        def premises(trust, gate, main, controller=controller, observer=observer, environment=environment, artifact=artifact):
            # These are proof-premise guards, not expected diagnostic output. Any
            # production/caller change needs the absolute-cwd argument revalidated.
            root = main[main.index("fn repo_root()"):main.index("\nfn on_path(")]
            entry = main[main.index("fn main()"):main.index("\n#[cfg(test)]", main.index("fn main()"))]
            guarded = {
                "trust production": (trust.split("#[cfg(test)]", 1)[0],
                    "036f650dc082d9d43956865ef67fb16f761c4b3ede54c0567f8936733322bef4"),
                "compiler controller": (controller.split("#[cfg(test)]", 1)[0],
                    "2364508f25c23a72db2fdca5e9f41280883a3b0bb3577a2a69e65ebbac2b5c69"),
                "compiler observer": (observer.split("#[cfg(test)]", 1)[0],
                    "c625e10018af3a1b4fd8c96ece4b21b5afdba53cd9b77bd8c619ab6a2037743f"),
                "artifact binding": (artifact.split("#[cfg(test)]", 1)[0],
                    "f51d00153db5f9be189efbe73ddb3ebd468cd1e48de93882d4c6e90406a2faa2"),
                "base environment": (environment,
                    "8b6fa81a2d715c2e42f8ab75a02b6e0c037c97d2f55e71c44c21e4a5516ee74c"),
                "gate production": (gate.split("#[cfg(test)]", 1)[0],
                    "8ad36a27c5e585660af256b59050f4393b1fa8601f591bd59de9ba2ffbfb7961"),
                "repository root": (root, "84ec4298740c835ae19ed9f2cff11f0f985cd59a4f3c5c205e9abd47805b6d77"),
                "CLI entry": (entry, "3bf4d646e31f96b58fe18697db897614d1e1aac4e11b585e0d02cca641b4ab62"),
            }
            for label, (text, expected) in guarded.items():
                self.assertEqual(hashlib.sha256(text.encode()).hexdigest(), expected,
                                 "revalidate absolute cwd proof: " + label)
            return root

        root_function = premises(source, gate, main)
        for altered in [
            (source.replace('let tree_dir = out.join("tree");',
                            'let tree_dir = PathBuf::from("tree");', 1), gate, main),
            (source, gate.replace("let repo = match repo.canonicalize() {",
                                 "let repo = match Ok::<_, std::io::Error>(repo.to_path_buf()) {", 1), main),
            (source, gate, main.replace('.expect("xtask/ sits at the repository root")',
                                        '.expect("widened root contract")', 1)),
            (source, gate, main, controller.replace('"ARCHOGEN_COMPILER_ROLE".to_owned()',
                                                    '"CARGO_TARGET_DIR".to_owned()', 1)),
            (source, gate, main, controller, observer.replace('.args(&argv[1..])', '.args(&argv[2..])', 1)),
            (source, gate, main, controller, observer,
             environment.replace('"CARGO_TARGET_DIR".to_owned()', '"OTHER_TARGET".to_owned()', 1)),
            (source, gate, main, controller, observer, environment,
             artifact.replace('.current_dir(tree)', '.current_dir(".")', 1)),
        ]:
            with self.assertRaisesRegex(AssertionError, "revalidate absolute cwd proof"):
                premises(*altered)

        selected = []
        for operator, original, replacement in [
            ("cond", "                if prefix.contains('\\n') {", "                if true {"),
            ("cmp", "                .any(|&(begin, end)| begin <= start && start < end)",
                    "                .any(|&(begin, end)| begin < start && start < end)"),
        ]:
            namespace, candidates = self.discovery(source, operator)
            matches = [m for m in candidates
                       if "\n".join(namespace["lines"][m[0]:m[1]]) == original and m[3] == replacement]
            self.assertEqual(len(matches), 1, "one exact native proof edit")
            selected.append((namespace, matches[0]))

        def module(text, name):
            body = text[text.index("fn cargo_error_line("):text.index("\n/// What cargo printed:")]
            return (f"mod {name} {{\nuse std::{{collections::BTreeSet, fs, path::Path}};\n"
                    + body.replace("fn cargo_error_line(", "pub fn cargo_error_line(", 1) + "\n}\n")

        modules = [module(source, "original")]
        modules += [module(namespace["apply_mutant"](edit), name)
                    for (namespace, edit), name in zip(selected, ("all_prefixes", "strict_begin"))]
        program = self.base / "absolute-prefixes.rs"
        binary = self.base / "absolute-prefixes"
        program.write_text("\n".join(modules) + r'''
fn main() {
    use std::{fs, path::{Path, PathBuf}};
    let base = PathBuf::from(std::env::args().nth(1).unwrap());
    assert!(base.is_absolute());
    fs::create_dir_all(&base).unwrap();
    let physical = base.join("physical\nerror-real");
    let alias = base.join("alias'\\\nerror-alias");
    fs::create_dir_all(&physical).unwrap();
    std::os::unix::fs::symlink(&physical, &alias).unwrap();
    let literal = "error: actual diagnostic";
    for cwd in [base.join("ordinary"), base.join("with\nerror-cwd"), base.join("ends\n"), alias] {
        fs::create_dir_all(&cwd).unwrap();
        for target in [None, Some(base.join("artifact\nerror-target")), Some(PathBuf::from("relative\nerror-target"))] {
            let env = target.as_ref().map(|p| vec![("CARGO_TARGET_DIR".to_owned(), p.display().to_string())])
                .unwrap_or_default();
            let mut records = String::new();
            for path in std::iter::once(cwd.clone()).chain(target.map(|p| if p.is_absolute() {p} else {cwd.join(p)})) {
                for spelling in std::iter::once(path.clone()).chain(fs::canonicalize(&path).ok()) {
                    records.push_str(&format!("warning: {}\n", spelling.display()));
                    records.push_str(&format!("Running '{}': refused\n", spelling.display().to_string().replace('\'', "'\\''")));
                }
            }
            for (stderr, expected) in [(String::new(), ""), (literal.to_owned(), literal),
                (format!("{records}{literal}\n"), literal), (format!("{records}{literal}\r\n"), literal)] {
                assert_eq!(original::cargo_error_line(&stderr, &cwd, &env), expected);
                assert_eq!(all_prefixes::cargo_error_line(&stderr, &cwd, &env), expected);
                assert_eq!(strict_begin::cargo_error_line(&stderr, &cwd, &env), expected);
            }
        }
    }
    let cwd = base.join("ordinary");
    let mut words = vec![String::new()];
    let mut frontier = vec![String::new()];
    for _ in 0..4 {
        frontier = frontier.iter().flat_map(|prefix| ["e", "r", "o", "/", "\n", "\r", "λ", "'"]
            .map(|part| format!("{prefix}{part}"))).collect();
        words.extend(frontier.clone());
    }
    for stderr in &words {
        let expected = original::cargo_error_line(stderr, &cwd, &[]);
        assert_eq!(all_prefixes::cargo_error_line(stderr, &cwd, &[]), expected);
        assert_eq!(strict_begin::cargo_error_line(stderr, &cwd, &[]), expected);
    }
    // Wider relative paths invalidate the absolute-prefix premise, concretely.
    assert_eq!(original::cargo_error_line("error: actual\n", Path::new("error"), &[]), "error: actual");
    assert_eq!(all_prefixes::cargo_error_line("error: actual\n", Path::new("error"), &[]), "");
    assert_eq!(original::cargo_error_line("error\nx\nerror: actual\n", Path::new("error\nx"), &[]), "error: actual");
    assert_eq!(strict_begin::cargo_error_line("error\nx\nerror: actual\n", Path::new("error\nx"), &[]), "error");
    match std::env::args().nth(2).as_deref() {
        Some("wrong-absolute") => assert_ne!(original::cargo_error_line(literal, &cwd, &[]),
            strict_begin::cargo_error_line(literal, &cwd, &[]), "wrong absolute oracle"),
        Some("wrong-relative") => assert_eq!(original::cargo_error_line("error: actual\n", Path::new("error"), &[]),
            all_prefixes::cargo_error_line("error: actual\n", Path::new("error"), &[]), "wrong relative oracle"),
        Some("wrong-empty") => assert_eq!(original::cargo_error_line("", &cwd, &[]), literal, "wrong empty oracle"),
        _ => {}
    }
    println!("actual absolute-prefix helpers held; {} generated strings; relative wider controls differ", words.len());
}
''')
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertIn("4681 generated strings", self.bounded([str(binary), str(self.base / "normal")]))
        for argument, message in [("wrong-absolute", "wrong absolute oracle"),
                                  ("wrong-relative", "wrong relative oracle"),
                                  ("wrong-empty", "wrong empty oracle")]:
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), str(self.base / argument), argument])

        # Compile the actual repository-root function through Cargo, using a
        # relative manifest argument and an alias. This is producer evidence;
        # the universal argument and pinned source are recorded in the book.
        root = self.base / "owned-root"
        package = root / "xtask"
        (package / "src").mkdir(parents=True)
        (package / "Cargo.toml").write_text(
            '[package]\nname = "prefix-root-proof"\nversion = "0.1.0"\nedition = "2021"\n[workspace]\n')
        (package / "src/main.rs").write_text(
            "use std::path::{Path, PathBuf};\n" + root_function + r'''
fn main() {
    let root = repo_root();
    assert!(root.is_absolute());
    assert_eq!(root.canonicalize().unwrap(),
        PathBuf::from(std::env::args().nth(1).unwrap()).canonicalize().unwrap());
    println!("actual Cargo root is absolute");
}
''')
        (self.base / "root-alias").symlink_to(root, target_is_directory=True)
        self.assertIn("actual Cargo root is absolute", self.bounded([
            "env", "CARGO_TARGET_DIR=" + str(self.base / "cargo-target"), "cargo", "run",
            "--quiet", "--offline", "--manifest-path", "root-alias/xtask/Cargo.toml", "--", str(root)]))
        records = selected[0][0]["load_equivalents"](ROOT / "xtask/trust.equivalents")
        for namespace, edit in selected:
            self.assertTrue(namespace["mutation_key"](edit) in records,
                            "the actual absolute-prefix edit requires its precise supported-domain reason")

    def test_actual_cargo_path_end_loops_preserve_the_bounded_collections(self):
        source = (ROOT / "xtask/src/trust.rs").read_text()
        first = source.index("fn cargo_error_line(")
        last = source.index("\n/// What cargo printed:", first)
        helper = source[first:last]
        namespace, candidates = self.discovery(source)
        constructor = """fn cargo_error_line(stderr: &str, cwd: &Path, env: &[(String, String)]) -> String {
    let mut paths = vec![cwd.to_path_buf()];
    if let Some((_, target)) = env.iter().rfind(|(key, _)| key == "CARGO_TARGET_DIR") {
        let target = Path::new(target);
        paths.push(if target.is_absolute() {
            target.to_path_buf()
        } else {
            cwd.join(target)
        });
    }
    let mut prefixes = BTreeSet::new();
"""
        self.assertTrue(helper.startswith(constructor), "revalidate the exact at-most-two path constructor")
        outer = "    for path in paths {"
        inner = "        for spelling in std::iter::once(path).chain(canonical) {"
        self.assertIn(outer + "\n        let canonical = fs::canonicalize(&path).ok();\n" + inner, helper)
        families = []
        for header in (outer, inner):
            family = [m for m in candidates if "\n".join(namespace["lines"][m[0]:m[1]]) == header]
            self.assertEqual(len(family), 3)
            self.assertIn("first ||", family[2][3], "select the actual ends edit")
            families.append(family)

        def module(text, name):
            body = text[text.index("fn cargo_error_line("):text.index("\n/// What cargo printed:")]
            return (f"mod {name} {{\nuse std::{{collections::BTreeSet, fs, path::Path}};\n"
                    + body.replace("fn cargo_error_line(", "pub fn cargo_error_line(", 1) + "\n}\n")

        modules = [module(source, "original")]
        modules += [module(namespace["apply_mutant"](family[2]), name)
                    for family, name in zip(families, ("path_ends", "spelling_ends"))]

        def trace(name, arguments, header, item):
            return (f"fn {name}({arguments}) -> Vec<std::path::PathBuf> {{\n"
                    "    let mut seen = Vec::new();\n" + header + "\n"
                    f"        seen.push({item});\n    }}\n    seen\n}}\n")

        functions = []
        for suffix, header in zip(("paths", "spellings"), (outer, inner)):
            arguments = ("paths: Vec<std::path::PathBuf>" if suffix == "paths"
                         else "path: std::path::PathBuf, canonical: Option<std::path::PathBuf>")
            item = "path" if suffix == "paths" else "spelling"
            family = families[0 if suffix == "paths" else 1]
            for name, prefix in zip(("original", "first", "last", "ends"), [header] + [m[3] for m in family]):
                functions.append(trace(name + "_" + suffix, arguments, prefix, item))
            # The same actual edit compiles on a wider constructor but then loses the middle.
            wider = arguments.replace("Option<std::path::PathBuf>", "Vec<std::path::PathBuf>")
            functions.append(trace("wider_" + suffix, wider, family[2][3], item))
        program, binary = self.base / "cargo-path-ends.rs", self.base / "cargo-path-ends"
        program.write_text("\n".join(modules + functions) + r'''
fn main() {
    use std::{fs, path::PathBuf};
    let base = PathBuf::from(std::env::args().nth(1).unwrap());
    let cwd = base.join("with\nerror-cwd");
    let target = base.join("with'\\\nerror-target");
    let real = base.join("with\nerror-real");
    let alias = base.join("with\nerror-alias");
    for path in [&cwd, &target, &real] { fs::create_dir_all(path).unwrap(); }
    std::os::unix::fs::symlink(&real, &alias).unwrap();
    let expected = "error: could not compile `a` (bin \"a\") due to 1 previous error";
    for working in [&cwd, &alias, &base.join("missing\nerror-cwd")] {
        for artifact in [None, Some(target.clone()), Some(alias.clone()), Some(PathBuf::from("relative\nerror-target"))] {
            let env: Vec<_> = artifact.as_ref().map(|p|
                vec![("CARGO_TARGET_DIR".to_owned(), p.display().to_string())]).unwrap_or_default();
            let mut records = String::new();
            let resolved_artifact = artifact.map(|p| if p.is_absolute() { p } else { working.join(p) });
            for path in [working.to_path_buf()].into_iter().chain(resolved_artifact) {
                for spelling in std::iter::once(path.clone()).chain(fs::canonicalize(&path).ok()) {
                    records.push_str(&format!("warning: {}\n", spelling.display()));
                    records.push_str(&format!("Running '{}': refused\n",
                        spelling.display().to_string().replace('\'', "'\\''")));
                }
            }
            for stderr in [String::new(), format!("{records}{expected}\n"), format!("{records}{expected}\r\n")] {
                let literal = if stderr.is_empty() { "" } else { expected };
                assert_eq!(original::cargo_error_line(&stderr, working, &env), literal);
                assert_eq!(path_ends::cargo_error_line(&stderr, working, &env), literal);
                assert_eq!(spelling_ends::cargo_error_line(&stderr, working, &env), literal);
            }
        }
    }
    let (a, b, c) = (PathBuf::from("cwd"), PathBuf::from("target"), PathBuf::from("third"));
    for paths in [vec![a.clone()], vec![a.clone(), b.clone()], vec![a.clone(), a.clone()]] {
        assert_eq!(original_paths(paths.clone()), paths);
        assert_eq!(ends_paths(paths.clone()), paths);
        assert_eq!(first_paths(paths.clone()), vec![paths[0].clone()]);
        assert_eq!(last_paths(paths.clone()), vec![paths.last().unwrap().clone()]);
    }
    for canonical in [None, Some(b.clone()), Some(a.clone())] {
        let expected = match canonical.clone() { None => vec![a.clone()], Some(p) => vec![a.clone(), p] };
        assert_eq!(original_spellings(a.clone(), canonical.clone()), expected);
        assert_eq!(ends_spellings(a.clone(), canonical.clone()), expected);
        assert_eq!(first_spellings(a.clone(), canonical.clone()), vec![a.clone()]);
        assert_eq!(last_spellings(a.clone(), canonical.clone()), vec![canonical.unwrap_or(a.clone())]);
    }
    assert_eq!(wider_paths(vec![a.clone(), b.clone(), c.clone()]), vec![a.clone(), c.clone()]);
    assert_eq!(wider_spellings(a.clone(), vec![b.clone(), c.clone()]), vec![a.clone(), c.clone()]);
    assert_ne!(wider_paths(vec![a.clone(), b.clone(), c.clone()]), vec![a.clone(), b.clone(), c.clone()]);
    match std::env::args().nth(2).as_deref() {
        Some("wrong-positive") => assert_eq!(first_paths(vec![a.clone(), b.clone()]), vec![a, b], "wrong positive oracle"),
        Some("wrong-negative") => assert_ne!(ends_paths(vec![a.clone(), b.clone()]), vec![a, b], "wrong negative oracle"),
        Some("wrong-wider") => assert_eq!(wider_spellings(a.clone(), vec![b.clone(), c.clone()]), vec![a, b, c], "wrong wider oracle"),
        _ => {}
    }
    println!("cargo path ends: actual helpers and ordered traces held; wider constructors refused");
}
''')
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        fixture = self.base / "normal"
        self.assertIn("actual helpers and ordered traces held", self.bounded([str(binary), str(fixture)]))
        for argument, message in (("wrong-positive", "wrong positive oracle"),
                                  ("wrong-negative", "wrong negative oracle"),
                                  ("wrong-wider", "wrong wider oracle")):
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), str(self.base / argument), argument])
        records = namespace["load_equivalents"](ROOT / "xtask/trust.equivalents")
        for family in families:
            self.assertTrue(namespace["mutation_key"](family[2]) in records,
                            "the actual bounded Cargo path loop requires its own precise reason")

    def test_the_actual_two_member_pair_loop_preserves_order_and_early_returns(self):
        source = (ROOT / "xtask/src/trust.rs").read_text()
        self.assertIn("pub pair: Option<(String, String)>", source)
        self.assertIn("let Some((a, b)) = &h.pair else { continue };", source)
        namespace, candidates = self.discovery(source)
        header = "        for x in [a, b] {"
        family = [m for m in candidates if "\n".join(namespace["lines"][m[0]:m[1]]) == header]
        self.assertEqual(len(family), 3, "one actual fixed-pair loop with all position variants")

        def traced(name, arguments, prefix):
            return (f"fn {name}({arguments}, stop: Option<usize>) -> Vec<usize> {{\n"
                    "    let mut out = Vec::new();\n" + prefix + "\n"
                    "        let identity = x as *const String as usize;\n"
                    "        out.push(identity);\n"
                    "        if stop == Some(identity) { return out; }\n"
                    "    }\n    out\n}\n")

        names = ["pair_original", "pair_first", "pair_last", "pair_ends"]
        prefixes = [header] + [m[3] for m in family]
        functions = [traced(name, "a: &String, b: &String", prefix)
                     for name, prefix in zip(names, prefixes)]
        # A wider array is outside the argument: obtain its actual ends rewrite too.
        triple = traced("sample", "a: &String, b: &String, c: &String", "    for x in [a, b, c] {")
        triple += "#[cfg(test)]\nmod tests {}\n"
        triple_namespace, triple_family = self.discovery(triple)
        self.assertEqual(len(triple_family), 3)
        functions += [triple.split("#[cfg(test)]", 1)[0].replace("fn sample", "fn triple_original", 1),
                      triple_namespace["apply_mutant"](triple_family[2]).split("#[cfg(test)]", 1)[0]
                      .replace("fn sample", "fn triple_ends", 1)]
        main = r'''
fn main() {
    let (a, b, c) = (String::from("same"), String::from("same"), String::from("third"));
    for (left, right) in [(&a, &b), (&b, &a), (&a, &a)] {
        let left_id = left as *const String as usize;
        let right_id = right as *const String as usize;
        for stop in [None, Some(left_id), Some(right_id)] {
            let expected = if stop == Some(left_id) { vec![left_id] } else { vec![left_id, right_id] };
            assert_eq!(pair_original(left, right, stop), expected);
            assert_eq!(pair_ends(left, right, stop), expected);
            assert_eq!(pair_first(left, right, stop), vec![left_id]);
            assert_eq!(pair_last(left, right, stop), vec![right_id]);
        }
    }
    let aid = &a as *const String as usize;
    let bid = &b as *const String as usize;
    let cid = &c as *const String as usize;
    assert_eq!(triple_original(&a, &b, &c, None), vec![aid, bid, cid]);
    assert_eq!(triple_ends(&a, &b, &c, None), vec![aid, cid]);
    assert_ne!(triple_original(&a, &b, &c, None), triple_ends(&a, &b, &c, None));
    match std::env::args().nth(1).as_deref() {
        Some("wrong-positive") => assert_eq!(pair_first(&a, &b, None), vec![aid, bid], "wrong positive oracle"),
        Some("wrong-negative") => assert_ne!(pair_ends(&a, &b, None), pair_original(&a, &b, None), "wrong negative oracle"),
        _ => {}
    }
    println!("pair-loop controls: reference order, aliases and early returns held; wider-array equivalence refused");
}
'''
        program, binary = self.base / "pair-loop.rs", self.base / "pair-loop"
        program.write_text("\n".join(functions) + main)
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertIn("reference order, aliases and early returns held", self.bounded([str(binary)]))
        for argument, message in (("wrong-positive", "wrong positive oracle"),
                                  ("wrong-negative", "wrong negative oracle")):
            with self.assertRaisesRegex(AssertionError, message):
                self.bounded([str(binary), argument])
        record_path = ROOT / "xtask/trust.equivalents"
        records = namespace["load_equivalents"](record_path) if record_path.exists() else {}
        self.assertIn(namespace["mutation_key"](family[2]), records,
                      "the two-member proof needs its per-file equivalence record")

    def test_owned_borrowed_patterns_labels_and_fresh_names_compile_and_select_positions(self):
        # Each input/output expectation is a semantic observation, independent of the replacement's spelling.
        forms = [
            ("vec", "", "x", "values", "out.push(x);"),
            ("vec_ref", "", "&x", "&values", "out.push(x);"),
            ("vec_iter", "", "&x", "values.iter()", "out.push(x);"),
            ("vec_into_iter", "", "x", "values.into_iter()", "out.push(x);"),
            ("slice", "", "&x", "values.as_slice()", "out.push(x);"),
            ("set", "    let values: std::collections::BTreeSet<_> = values.into_iter().collect();\n",
             "x", "values", "out.push(x);"),
            ("set_ref", "    let values: std::collections::BTreeSet<_> = values.into_iter().collect();\n",
             "&x", "&values", "out.push(x);"),
            ("tuple", "", "(_, x)", "values.into_iter().enumerate()", "out.push(x);"),
            ("label", "", "x", "values", "if x == 7 { continue 'items; } out.push(x);"),
            # The header is line four: a source name equal to the first proposed helper prefix must stay bound.
            ("collision", "    let __archogen_sweep_4_first = 0;\n", "x", "values",
             "out.push(x + __archogen_sweep_4_first);"),
            ("array", "", "x", "values", "out.push(x);"),
            ("array_ref", "", "&x", "&values", "out.push(x);"),
        ]
        programs, assertions = [], []
        for name, setup, pattern, expression, body in forms:
            array = name.startswith("array")
            signature = "fn sample<const N: usize>(values: [i32; N])" if array else "fn sample(values: Vec<i32>)"
            label = "'items: " if name == "label" else ""
            source = (signature + " -> Vec<i32> {\n    let mut out = Vec::new();\n" + setup
                      + f"    {label}for {pattern} in {expression} {{ // trailing header comment\n"
                      + "        " + body + "\n    }\n    out\n}\n#[cfg(test)]\nmod tests {}\n")
            namespace, candidates = self.discovery(source)
            self.assertEqual(len(candidates), 3, name)
            # Baseline and all three real candidates go through the installed Rust compiler together.
            for position, mutant in enumerate([None] + candidates):
                function = name + "_" + str(position)
                text = source if mutant is None else namespace["apply_mutant"](mutant)
                self.assertIn("// trailing header comment", text)
                programs.append(text.split("#[cfg(test)]", 1)[0].replace("fn sample", "fn " + function, 1))
                inputs = ["[]", "[7]", "[2, 7, 9, 13]"]
                outputs = [[], [] if name == "label" else [7],
                           [[2, 7, 9, 13] if name != "label" else [2, 9, 13], [2], [13], [2, 13]][position]]
                for argument, expected in zip(inputs, outputs):
                    argument = argument if array else "vec!" + argument
                    assertions.append(f'assert_eq!({function}({argument}), vec!{expected}, "{function}");')
        program = self.base / "iterators.rs"
        program.write_text("#![allow(unused_variables)]\n" + "\n".join(programs)
                           + "\nfn main() {\n" + "\n".join(assertions)
                           + f'\nprintln!("iterator probes: {len(assertions)} observations");\n}}\n')
        binary = self.base / "iterators"
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertEqual(self.bounded([str(binary)]).strip(), "iterator probes: 144 observations")

    def test_mask_excludes_literal_comment_headers_and_preserves_offsets_and_labels(self):
        ghost = "\nfor fake in fake_values {\n}\n"
        literals = ['"' + ghost + '"', 'b"' + ghost + '"', 'c"' + ghost + '"',
                    'r##"' + ghost + '"##', 'br#"' + ghost + '"#', 'cr"' + ghost + '"',
                    "/* outer\n/* nested */" + ghost + "*/"]
        source = "fn sample<'a>(values: &'a [i32]) {\n"
        source += "\n".join("    let _ghost = " + literal + ("();" if literal.startswith("/*") else ";")
                            for literal in literals)
        source += ("\n    let _chars = ('{', b'}', '\\'', '\\\\', '\\n', '\\x7f', '\\u{1f980}');\n"
                   "    // for fake in values {\n"
                   "    'items: for &x in values {\n        if x == 7 { continue 'items; }\n    }\n"
                   "}\n#[cfg(test)]\nmod tests {}\n")
        namespace, candidates = self.discovery(source)
        self.assertEqual(len(candidates), 3)
        self.assertTrue(all(source.split("\n")[m[0]].strip().startswith("'items: for") for m in candidates))
        masked = namespace["mask_rust"](source)
        self.assertEqual(len(masked), len(source))
        self.assertEqual([i for i, c in enumerate(masked) if c == "\n"],
                         [i for i, c in enumerate(source) if c == "\n"])
        self.assertNotIn("fake_values", masked)
        self.assertIn("<'a>(values: &'a [i32])", masked)
        self.assertIn("'items: for &x in values", masked)
        self.assertIn("continue 'items", masked)
        programs = [namespace["apply_mutant"](m).split("#[cfg(test)]", 1)[0]
                    .replace("fn sample", "fn sample_" + str(i), 1) for i, m in enumerate(candidates)]
        program = self.base / "literals.rs"
        program.write_text("#![allow(unused_variables)]\n" + "\n".join(programs)
                           + "\nfn main() { sample_0(&[2, 7]); sample_1(&[2, 7]); sample_2(&[2, 7]); }\n")
        binary = self.base / "literals"
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.bounded([str(binary)])


class ScopeTests(DiscoveryFixture):
    def test_branch_scopes_exclude_nested_loops_and_closures_but_keep_plain_if_scopes(self):
        source = r'''fn copies<const N: usize>(x: i32) -> [i32; N] { [x; N] }
fn sample(values: &[i32]) {
    for &x in values { // outer
        if x > 0 { // outer_if
            if x > 1 { // plain_nested_if
            }
        } else if x < 0 { // outer_else_if
        }
        for y in { values } {
            if *y > 0 { // inner_for_block_operand
            }
        }
        for (left, right) in
            values.iter().zip(values.iter())
        {
            if left == right { // inner_multiline_for
            }
        }
        for y in copies::<{ 2 }>(x) {
            if y > 0 { // inner_for_const_generic
            }
        }
        'again: while x < 0 {
            if x > 0 { // inner_while
            }
            break 'again;
        }
        while { x < 0 } {
            if x > 0 { // inner_while_block_operand
            }
            break;
        }
        while if x < 0 { true } else { false } {
            if x > 0 { // inner_while_if_operand
            }
            break;
        }
        while match x { 0 => false, _ => false } {
            if x > 0 { // inner_while_match_operand
            }
            break;
        }
        struct Record { n: i32 }
        while let Record { n: y } = (Record { n: x }) {
            if y > 0 { // inner_while_struct_pattern
            }
            break;
        }
        while 'a' == 'b' {
            if x > 0 { // inner_while_char_operands
            }
            break;
        }
        if x > 0 { // after_char_loop
        }
        while "a" == "b" {
            if x > 0 { // inner_while_string_operands
            }
            break;
        }
        if x > 0 { // after_string_loop
        }
        loop {
            if x > 0 { // inner_loop
            }
            break;
        }
        let _block = |value: i32| {
            if value > 0 { // block_closure
            }
        };
        let _expr = |value: i32|
            if value > 0 { // expression_closure
                true
            } else { false };
        let _typed = || -> Result<i32, &'static str> {
            if x > 0 { // typed_closure
            }
            Ok(x)
        };
        let _async = async || {
            if x > 0 { // async_closure
            }
        };
        let _mapped: Vec<_> = values.iter().map(|value|
            if *value > 0 { // argument_expression_closure
                true
            } else { false }).collect();
        let bits = (x | 1) | 2;
        let r#loop = bits;
        if r#loop > 0 { // after_scopes_and_bitwise
        }
        if x > 0 || bits > 0 { // boolean_or
        }
        if let Some(y) = values.first() { // if_let_is_outside_the_operator
            let _ = y;
        }
    }
}
#[cfg(test)]
mod tests {}
'''
        namespace, _ = self.discovery(source)
        lines = source.split("\n")
        outer = next(i for i, line in enumerate(lines) if "// outer" in line)
        targets = namespace["guard_targets"](outer)
        actual = {lines[n].split("// ", 1)[1] for n, _ in targets}
        self.assertEqual(actual, {"outer_if", "plain_nested_if", "outer_else_if",
                                  "after_char_loop", "after_string_loop", "after_scopes_and_bitwise", "boolean_or"})
        for n, (first, last) in targets:
            condition = lines[n][first:last]
            self.assertTrue(condition.startswith(("x ", "r#loop ")), condition)
        inner = next(i for i, line in enumerate(lines) if "for y in { values }" in line)
        self.assertEqual({lines[n].split("// ", 1)[1] for n, _ in namespace["guard_targets"](inner)},
                         {"inner_for_block_operand"})
        _, guards = self.discovery(source, "guard")
        outer_guards = [m for m in guards if m[0] == outer]
        self.assertEqual(len(outer_guards), 3 * len(targets))
        self.assertEqual({lines[m[1] - 1].split("// ", 1)[1] for m in outer_guards}, actual)
        # The source is real stable Rust, not just a token-shaped fixture. Empty input makes runtime finite.
        program = self.base / "scopes.rs"
        program.write_text("#![allow(unused_variables, unused_braces, irrefutable_let_patterns)]\n"
                           + source.split("#[cfg(test)]", 1)[0] + "\nfn main() { sample(&[]); }\n")
        binary = self.base / "scopes"
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.bounded([str(binary)])


class GuardTests(DiscoveryFixture):
    def test_complete_literal_conditions_compile_without_truncated_quotes_or_comments(self):
        conditions = [
            'x.to_string() != "7"',
            'x.to_string() != r#"7"#',
            'x.to_string().as_bytes() != b"7"',
            'x.to_string().as_bytes() != br#"7"#',
            "char::from_digit(x as u32, 10).unwrap_or('x') != '7'",
            "x as u8 + b'0' != b'7'",
            'x != 7 /* trailing { } comment */',
            'x.to_string() != "brace {"',
            'x.to_string() != r##"end \" {"##',
            'x.to_string().as_bytes() != c"7".to_bytes()',
        ]
        programs, assertions = [], []
        for number, condition in enumerate(conditions):
            source = ("fn sample(values: &[i32]) -> Vec<i32> {\n    let mut out = Vec::new();\n"
                      "    for &x in values {\n        if " + condition + " { // branch comment\n"
                      "            out.push(x);\n        }\n    }\n    out\n}\n#[cfg(test)]\nmod tests {}\n")
            namespace, candidates = self.discovery(source, "guard")
            targets = namespace["guard_targets"](2)
            self.assertEqual(len(targets), 1)
            line, (first, last) = targets[0]
            self.assertEqual(source.split("\n")[line][first:last], condition)
            self.assertEqual(len(candidates), 3)
            for position, mutant in enumerate([None] + candidates):
                function = f"literal_{number}_{position}"
                text = source if mutant is None else namespace["apply_mutant"](mutant)
                self.assertIn("// branch comment", text)
                programs.append(text.split("#[cfg(test)]", 1)[0].replace("fn sample", "fn " + function, 1))
                for values in [[], [7], [2, 7, 9, 13]]:
                    expected = [x for i, x in enumerate(values)
                                if (number in (7, 8) or x != 7)
                                and (position == 0 or position == 1 and i == 0
                                     or position == 2 and i == len(values) - 1
                                     or position == 3 and i in (0, len(values) - 1))]
                    assertions.append(f'assert_eq!({function}(&{values}), vec!{expected}, "{function}");')
        program = self.base / "literal_guards.rs"
        program.write_text("\n".join(programs) + "\nfn main() {\n" + "\n".join(assertions) + "\n}\n")
        binary = self.base / "literal_guards"
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.bounded([str(binary)])

    def test_owned_borrowed_guards_preserve_body_work_and_condition_evaluation(self):
        forms = [
            ("vec", "", "x", "values"),
            ("vec_ref", "", "&x", "&values"),
            ("vec_iter", "", "&x", "values.iter()"),
            ("vec_into_iter", "", "x", "values.into_iter()"),
            ("slice", "", "&x", "values.as_slice()"),
            ("set", "    let values: std::collections::BTreeSet<_> = values.into_iter().collect();\n", "x", "values"),
            ("set_ref", "    let values: std::collections::BTreeSet<_> = values.into_iter().collect();\n", "&x", "&values"),
            ("tuple", "", "(_, x)", "values.into_iter().enumerate()"),
            ("label", "", "x", "values"),
            ("collision", "    let __archogen_sweep_4_first = 0;\n", "x", "values"),
            ("array", "", "x", "values"),
            ("array_ref", "", "&x", "&values"),
        ]
        programs, assertions = [], []
        for name, setup, pattern, expression in forms:
            array = name.startswith("array")
            signature = "fn sample<const N: usize>(values: [i32; N])" if array else "fn sample(values: Vec<i32>)"
            label = "'items: " if name == "label" else ""
            extra = "        if x == 7 { continue 'items; }\n" if name == "label" else ""
            if name == "collision":
                extra = "        let x = x + __archogen_sweep_4_first;\n"
            source = (signature + " -> (Vec<i32>, Vec<i32>, usize) {\n"
                      "    let (mut seen, mut out, mut checks) = (Vec::new(), Vec::new(), 0);\n" + setup
                      + f"    {label}for {pattern} in {expression} {{ // preserve header\n" + extra
                      + "        seen.push(x);\n"
                      "        if { checks += 1; x != 7 } { // first branch\n"
                      "            out.push(x);\n        } else if x == 7 { // second branch\n"
                      "            out.push(-x);\n        }\n    }\n    (seen, out, checks)\n}\n"
                      "#[cfg(test)]\nmod tests {}\n")
            namespace, candidates = self.discovery(source, "guard")
            self.assertEqual(len(candidates), 6, name)
            first_branch = next(i for i, line in enumerate(source.split("\n")) if "// first branch" in line)
            for number, mutant in enumerate([None] + candidates):
                function = f"{name}_{number}"
                text = source if mutant is None else namespace["apply_mutant"](mutant)
                self.assertIn("// preserve header", text)
                programs.append(text.split("#[cfg(test)]", 1)[0].replace("fn sample", "fn " + function, 1))
                for values in [[], [7], [2, 7, 9, 13]]:
                    reached = [x for x in values if name != "label" or x != 7]
                    selected = []
                    for i, x in enumerate(values):
                        if name == "label" and x == 7:
                            continue
                        # Independent branch/position oracle; every condition is evaluated before its gate.
                        permitted = (mutant is None or (number - 1) % 3 == 0 and i == 0
                                     or (number - 1) % 3 == 1 and i == len(values) - 1
                                     or (number - 1) % 3 == 2 and i in (0, len(values) - 1))
                        primary = x != 7 and (mutant is None or mutant[1] - 1 != first_branch or permitted)
                        secondary = x == 7 and (mutant is None or mutant[1] - 1 == first_branch or permitted)
                        if primary:
                            selected.append(x)
                        elif secondary:
                            selected.append(-x)
                    argument = str(values) if array else "vec!" + str(values)
                    assertions.append(f'assert_eq!({function}({argument}), (vec!{reached}, vec!{selected}, '
                                      f'{len(reached)}), "{function}");')
        program = self.base / "guards.rs"
        program.write_text("#![allow(unused_variables, unused_braces)]\n" + "\n".join(programs)
                           + "\nfn main() {\n" + "\n".join(assertions)
                           + f'\nprintln!("guard probes: {len(assertions)} observations");\n}}\n')
        binary = self.base / "guards"
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertEqual(self.bounded([str(binary)]).strip(), "guard probes: 252 observations")

    def test_real_guard_candidates_land_and_restore_copied_source(self):
        source = ("fn sample(values: &[i32]) -> i32 {\n    let mut out = 0;\n"
                  "    for &x in values {\n        if x > 0 {\n            out += x;\n        }\n"
                  "        if x < 0 {\n            out -= x;\n        }\n    }\n    out\n}\n"
                  "#[cfg(test)]\nmod tests {}\n")
        namespace, candidates = self.discovery(source, "guard")
        self.assertEqual(len(candidates), 6)
        for mutant in (candidates[0], candidates[-1]):
            before, after = namespace["mutation_key"](mutant)
            record = "from-json " + json.dumps(before) + "\nto-json " + json.dumps(after) + "\nwhy fixture\n"
            for equivalent in (False, True):
                with self.subTest(stop=mutant[1], equivalent=equivalent):
                    result = self.run_sweep(record if equivalent else "", original=source,
                                            mutated=namespace["apply_mutant"](mutant), candidate=mutant)
                    self.assert_exit(result, 0 if equivalent else 1)
                    self.assertEqual((self.base / "invoked").read_text(), "called\ncalled\n")


class ChainTests(DiscoveryFixture):
    def test_actual_owned_borrowed_chain_variants_compile_and_preserve_order(self):
        forms = ["vec", "vec_ref", "slice", "array", "array_ref", "set", "set_ref"]
        programs, assertions = [], []
        for name in forms:
            borrowed = name.endswith("_ref") or name == "slice"
            if name.startswith("array"):
                signature = "fn sample<const N: usize>(values: " + ("&" if borrowed else "") + "[i32; N])"
            elif name == "slice":
                signature = "fn sample(values: &[i32])"
            else:
                signature = "fn sample(values: " + ("&" if name == "vec_ref" else "") + "Vec<i32>)"
            setup = "    let values = std::collections::BTreeSet::from_iter(values);\n" if name.startswith("set") else ""
            iteration = "iter" if borrowed else "into_iter"
            value = "**x" if borrowed else "*x"
            suffix = ".copied()" if borrowed else ""
            tail = "[&17, &19]" if borrowed else "[17, 19]"
            source = (signature + " -> Vec<i32> {\n" + setup + "    values\n"
                      f"        .{iteration}() // iter site\n"
                      "        .filter(|x| { // filter site\n"
                      f"            ({value} != 7) && Some({value}).is_some()\n"
                      "        }) // filter end\n"
                      f"        .chain({tail}){suffix} // chain site\n"
                      "        .collect()\n}\n#[cfg(test)]\nmod tests {}\n")
            namespace, candidates = self.discovery(source, "chain")
            self.assertEqual(len(candidates), 6, name)
            original_lines = source.split("\n")
            groups = {site: [m for m in candidates if "// " + site + " site" in original_lines[m[0]]]
                      for site in ("iter", "filter", "chain")}
            self.assertEqual([len(groups[site]) for site in groups], [2, 3, 1])
            for number, mutant in enumerate([None] + candidates):
                function = f"chain_{name}_{number}"
                text = source if mutant is None else namespace["apply_mutant"](mutant)
                programs.append(text.split("#[cfg(test)]", 1)[0].replace("fn sample", "fn " + function, 1))
                for supplied in [[], [7], [2, 7, 9, 13], [7, 2, 9, 7]]:
                    values = sorted(set(supplied)) if name.startswith("set") else supplied[:]
                    site = next((site for site in groups if mutant in groups[site]), None)
                    variant = groups[site].index(mutant) if site else None
                    if site == "iter":
                        values = values[:1] if variant == 0 else values[-1:]
                    values = values if site == "filter" and variant == 2 else [x for x in values if x != 7]
                    if site == "filter" and variant in (0, 1):
                        values = values[:1] if variant == 0 else values[-1:]
                    expected = values if site == "chain" else values + [17, 19]
                    argument = str(supplied) if name.startswith("array") or name == "slice" else "vec!" + str(supplied)
                    if borrowed and name not in ("set_ref",):
                        argument = "&" + argument
                    assertions.append(f'assert_eq!({function}({argument}), vec!{expected}, "{function}");')
        program = self.base / "chains.rs"
        program.write_text("#![allow(unused_parens)]\n" + "\n".join(programs)
                           + "\nfn main() {\n" + "\n".join(assertions)
                           + f'\nprintln!("chain probes: {len(assertions)} observations");\n}}\n')
        binary = self.base / "chains"
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.assertEqual(self.bounded([str(binary)]).strip(), "chain probes: 196 observations")

    def test_lexical_calls_exclude_literals_comments_and_nonempty_iter_arguments(self):
        source = '''struct Widget;
impl Widget { fn iter(&self, _argument: i32) {} }
fn sample() {
    let _ghost = r#".iter().filter(|x| (x)).chain([1])("#;
    /* outer .into_iter() /* .filter([1]) */ .chain([2]) */
    Widget.iter(42);
    let values = [1, 2];
    let _: Vec<_> = values.r#iter(/* no arguments */).collect();
}
#[cfg(test)]
mod tests { fn fixture() { let _: Vec<_> = [7].iter().collect(); } }
'''
        namespace, candidates = self.discovery(source, "chain")
        self.assertEqual(len(candidates), 2)
        self.assertTrue(all("no arguments" in source.split("\n")[m[0]] for m in candidates))
        programs = [source.split("#[cfg(test)]", 1)[0].replace("fn sample", "fn baseline", 1)]
        for number, mutant in enumerate(candidates):
            # Reuse the item definitions once; only the sample function differs in these compiled candidates.
            text = namespace["apply_mutant"](mutant).split("fn sample", 1)[1].split("#[cfg(test)]", 1)[0]
            programs.append("fn candidate_" + str(number) + text)
        program = self.base / "lexical_chains.rs"
        program.write_text("\n".join(programs) + "\nfn main() { baseline(); candidate_0(); candidate_1(); }\n")
        binary = self.base / "lexical_chains"
        self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)])
        self.bounded([str(binary)])

    def test_multiline_filter_candidates_land_and_restore_copied_source(self):
        source = ("fn sample(values: Vec<i32>) -> Vec<i32> {\n    values.into_iter()\n"
                  "        .filter(|x| {\n            Some(*x).is_some() && *x != 7\n        })\n"
                  "        .collect()\n}\n#[cfg(test)]\nmod tests {}\n")
        namespace, candidates = self.discovery(source, "chain")
        candidates = [m for m in candidates if m[0] == 2]
        self.assertEqual(len(candidates), 3)
        for mutant in candidates:
            self.assertEqual(mutant[1], 5)
            before, after = namespace["mutation_key"](mutant)
            record = "from-json " + json.dumps(before) + "\nto-json " + json.dumps(after) + "\nwhy fixture\n"
            result = self.run_sweep(record, original=source, mutated=namespace["apply_mutant"](mutant), candidate=mutant)
            self.assert_exit(result, 0)
            self.assertEqual((self.base / "invoked").read_text(), "called\ncalled\n")


class ContainsTests(DiscoveryFixture):
    def test_function_type_arrows_keep_the_complete_generic_receiver_viable(self):
        source = ("fn one() -> i32 { 1 }\nfn two() -> i32 { 2 }\nfn three() -> i32 { 3 }\n"
                  "fn four() -> i32 { 4 }\n"
                  "fn sample(values: Vec<fn() -> i32>, needle: fn() -> i32) -> bool {\n"
                  "    std::convert::identity::<Vec<fn() -> i32>>(values).contains(&needle)\n"
                  "}\n#[cfg(test)]\nmod tests {}\n")
        namespace, candidates = self.discovery(source, "contains")
        self.assertEqual(len(candidates), 4)
        for number, mutant in enumerate([None] + candidates):
            viable = number in (0, 1, 3)
            text = source if mutant is None else namespace["apply_mutant"](mutant)
            assertions = []
            for values in [[], ["one"], ["one", "two", "three"]]:
                selected = values if number == 0 else values[:1] if number in (1, 2) else values[-1:]
                for needle in ["one", "two", "three", "four"]:
                    expected = str(needle in selected).lower()
                    assertions.append(f"assert_eq!(sample(vec![{', '.join(values)}], {needle}), {expected});")
            program = self.base / f"function_membership_{number}.rs"
            program.write_text(text.split("#[cfg(test)]", 1)[0] + "\nfn main() {\n"
                               + "\n".join(assertions) + "\n}\n")
            binary = self.base / f"function_membership_{number}"
            self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)], expect_type_error=not viable)
            if viable:
                self.bounded([str(binary)])

    def compile_membership_case(self, source, argument, cases, *, copied, negate=False):
        namespace, candidates = self.discovery(source, "contains")
        self.assertEqual(len(candidates), 4)
        compiled, refused, observations = 0, 0, 0
        for number, mutant in enumerate([None] + candidates):
            viable = mutant is None or copied is not None and ((number - 1) % 2 == 1) == copied
            text = source if mutant is None else namespace["apply_mutant"](mutant)
            assertions = []
            for values, needle in cases:
                if mutant is None:
                    expected = needle in values
                else:
                    endpoint = values[:1] if number in (1, 2) else values[-1:]
                    expected = bool(endpoint) and needle == endpoint[0]
                expected = expected != negate
                assertions.append(f'assert_eq!(sample({argument(values)}, {json.dumps(needle)}), '
                                  f'{str(expected).lower()});')
            program = self.base / f"membership_{number}.rs"
            program.write_text(text.split("#[cfg(test)]", 1)[0] + "\nfn main() {\n"
                               + "\n".join(assertions) + "\n}\n")
            binary = self.base / f"membership_{number}"
            self.bounded(["rustc", "--edition=2021", str(program), "-o", str(binary)], expect_type_error=not viable)
            if viable:
                self.bounded([str(binary)])
                observations += len(cases)
                compiled += mutant is not None
            else:
                refused += 1
        return len(candidates), compiled, refused, observations

    def test_direct_vectors_copied_borrowed_sets_and_unviable_types(self):
        numeric = [(values, needle) for values in [[], [7], [2, 7, 9, 13]] for needle in [0, 2, 7, 13]]
        strings = [(values, needle) for values in [[], ["m"], ["a", "m", "z"]]
                   for needle in ["none", "a", "m", "z"]]
        cases = [
            ("Vec<i32>", "i32", "&needle", lambda values: "vec!" + str(values), numeric, False),
            ("&[i32]", "i32", "&needle", lambda values: "&" + str(values), numeric, False),
            ("std::collections::BTreeSet<&'static str>", "&str", "needle",
             lambda values: "std::collections::BTreeSet::from_iter(" + json.dumps(values) + ")", strings, True),
            ("std::collections::BTreeSet<String>", "&str", "needle",
             lambda values: "std::collections::BTreeSet::from_iter(" + json.dumps(values)
             + ".into_iter().map(str::to_owned))", strings, None),
        ]
        results = []
        for collection, needle, query, argument, observations, copied in cases:
            with self.subTest(collection=collection):
                source = (f"fn sample(values: {collection}, needle: {needle}) -> bool {{\n"
                          "    let _ghost = r#\".contains(42)\"#; /* .contains(99) */\n"
                          f"    values.contains({query})\n}}\n#[cfg(test)]\n"
                          "mod tests { fn fixture() { let _ = [7].contains(&7); } }\n")
                results.append(self.compile_membership_case(source, argument, observations, copied=copied))
        totals = tuple(sum(result[n] for result in results) for n in range(4))
        self.assertEqual(totals, (16, 6, 10, 120))
        print("contains type probes: 16 candidates, 6 viable, 10 type refusals; 120 observations")

    def test_receiver_boundaries_preserve_fields_groups_calls_generics_and_negation(self):
        forms = [
            ("holder.values", "    let holder = Holder { values };\n", False),
            ("(&values)", "", False),
            ("values.as_slice()", "", False),
            ("Some(values).unwrap()", "", False),
            ("(values, ()).0", "", False),
            ("make::<i32>(values)", "", False),
            ("std::convert::identity::<Vec<i32>>(values)", "", False),
            ("reads[&0]", "    let reads = std::collections::BTreeMap::from([(0, values)]);\n", True),
        ]
        observations = [(values, needle) for values in [[], [7], [2, 7, 9, 13]] for needle in [0, 2, 7, 13]]
        results = []
        for receiver, setup, negate in forms:
            with self.subTest(receiver=receiver):
                source = ("struct Holder { values: Vec<i32> }\n"
                          "fn make<T>(values: Vec<T>) -> Vec<T> { values }\n"
                          "fn sample(values: Vec<i32>, needle: i32) -> bool {\n" + setup
                          + "    " + ("!" if negate else "") + receiver
                          + ".contains(&needle)\n}\n#[cfg(test)]\nmod tests {}\n")
                results.append(self.compile_membership_case(source, lambda values: "vec!" + str(values), observations,
                                                            copied=False, negate=negate))
        totals = tuple(sum(result[n] for result in results) for n in range(4))
        self.assertEqual(totals, (32, 16, 16, 288))
        print("contains receiver probes: 32 candidates, 16 viable, 16 type refusals; 288 observations")

    def test_multiline_receiver_and_argument_land_and_restore(self):
        source = ("fn sample(values: Vec<i32>, needle: i32) -> bool {\n"
                  "    !std::convert::identity::<Vec<i32>>(\n        values,\n    ).contains(\n"
                  "        &needle, // retain this argument comment\n    )\n}\n#[cfg(test)]\nmod tests {}\n")
        namespace, candidates = self.discovery(source, "contains")
        self.assertEqual(len(candidates), 4)
        for mutant in candidates:
            self.assertEqual((mutant[0], mutant[1]), (1, 6))
            before, after = namespace["mutation_key"](mutant)
            self.assertIn("// retain this argument comment\n", after)
            record = "from-json " + json.dumps(before) + "\nto-json " + json.dumps(after) + "\nwhy fixture\n"
            result = self.run_sweep(record, original=source, mutated=namespace["apply_mutant"](mutant), candidate=mutant)
            self.assert_exit(result, 0)
            self.assertEqual((self.base / "invoked").read_text(), "called\ncalled\n")


if __name__ == "__main__":
    unittest.main()
