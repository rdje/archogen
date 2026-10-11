"""Real sweep CLI probes and compiled candidates; no fixture Git/Cargo build."""
import ast
import bisect
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


class IteratorTests(DiscoveryFixture):
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
