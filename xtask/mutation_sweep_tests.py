"""Real sweep CLI probes and compiled iterator candidates; no fixture Git/Cargo build."""
import ast
import json
import os
from pathlib import Path
import re
import signal
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


class IteratorTests(SweepFixture):
    def discovery(self, source):
        # Load the actual discovery/replacement functions without invoking the CLI or copying its algorithm.
        python = SCRIPT.read_text().split("<<'PY'\n", 1)[1].rsplit("\nPY", 1)[0]
        functions = [node for node in ast.parse(python).body if isinstance(node, ast.FunctionDef)]
        lines = source.split("\n")
        namespace = dict(re=re, json=json, lines=lines, first=0, last=lines.index("#[cfg(test)]"))
        exec(compile(ast.Module(body=functions, type_ignores=[]), str(SCRIPT), "exec"), namespace)
        candidates = [m for m in namespace["mutants"]() if m[2] == "iter"]
        return namespace, candidates

    def bounded(self, command):
        env = dict(os.environ, TMPDIR=str(self.base), CARGO_HOME=str(ROOT / "target/cargo-home"))
        process = subprocess.Popen(command, cwd=self.base, env=env, stdout=subprocess.PIPE,
                                   stderr=subprocess.PIPE, text=True, start_new_session=True)
        try:
            stdout, stderr = process.communicate(timeout=60)
        except subprocess.TimeoutExpired:
            os.killpg(process.pid, signal.SIGKILL)
            process.communicate()
            self.fail("iterator probe command timed out: " + repr(command))
        self.assertEqual(process.returncode, 0, stdout + stderr)
        return stdout

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


if __name__ == "__main__":
    unittest.main()
