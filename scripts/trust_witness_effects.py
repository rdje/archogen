#!/usr/bin/env python3
"""Retain reviewed span-cohort classifications and compiler-backed selection contrasts (M3.6.9.5.4.2)."""
import argparse
import copy
import json
import os
from pathlib import Path
import re
import subprocess
import sys

sys.dont_write_bytecode = True
import trust_adequacy as adequacy
import trust_witness as witness

# These are reviewed semantic assertions, not conclusions inferred from verdict pairs.
# The book gives their proofs and the scope of every absence reason.
WITNESSES = {
    "the-first-generator-alone": "L1851.iter.740",
    "the-last-generator-alone": "L1851.iter.741",
    "the-first-reader-alone": "L1849.chain.1219",
    "the-last-reader-alone": "L1849.chain.1220",
    "chain-and-blob-rules-on-each-clause-s-first-entry-alone": "L273.iter.106",
    "chain-and-blob-rules-on-each-clause-s-last-entry-alone": "L273.iter.107",
    "both-clauses-the-first-input-alone": "L192.contains.224",
    "both-clauses-the-last-input-alone": "L192.contains.226",
    "both-clauses-the-last-generator-alone": "L191.chain.220",
    "held-judged-for-the-first-alone": "L1857.iter.749",
    "the-last-live-form-alone": "L253.chain.244",
    "a-harness-judged-by-its-first-role-alone": "L1838.clogic.1411",
    "a-build-read-naming-its-first-generator-alone": "L2014.iter.829",
}
ABSENCES = {
    "a-program-target-judged-for-its-first-reader-alone": "reader-specific result filtering absent; guards change build continuation",
    "a-program-target-judged-for-its-last-reader-alone": "reader-specific result filtering absent; guards change build continuation",
    "a-program-target-judged-when-first-alone": "if-let branch absent from one-line boolean guard shapes",
    "a-program-target-judged-when-last-alone": "if-let branch absent from one-line boolean guard shapes",
    "a-program-target-built-when-first-alone": "generator-file position gate absent; cohort narrows owned targets or build guards",
    "a-program-target-built-when-last-alone": "generator-file position gate absent; cohort narrows owned targets or build guards",
    "a-lone-rs-refused-when-first-alone": "multiline else-if guard absent; only extension equality changes",
    "a-lone-rs-refused-when-last-alone": "multiline else-if guard absent; only extension equality changes",
    "a-middle-script-never-judged": "script-specific position predicate absent; iterator ends also drop middle Rust targets",
    "a-middle-reader-never-judged": "chain has first/last but no two-ends variant; other candidates change generators or membership",
    "a-middle-input-never-reached": "input-only position predicate absent; iterator ends narrow both roles",
    "an-implementation-reader-never-judged": "implementation-role predicate absent; iterator positions are role-independent",
    "a-marked-build-read-dropped-beside-other-generators": "form-generator association predicate absent; guards narrow by build-read position",
    "only-a-form-s-first-input-reached": "input-only first predicate absent; iterator first narrows both roles",
    "an-input-reached-through-a-link": "input-only link predicate absent; guards/endpoints/constants also affect generator links",
    "a-harness-never-judged-against-a-program-target": "harness-reader predicate absent; cohort changes held packages or continuation",
    "a-data-reader-never-judged": "runtime-data reader predicate absent; chain/membership endpoints are role-independent",
    "a-middle-generator-never-reached": "generator-only position predicate absent; iterator ends narrow inputs too",
    "a-transitive-generator-build-read-dropped": "dependency-depth predicate absent; cohort narrows unit/file positions",
}


def selections(manifest):
    rows = {row["suffix"]: row for row in manifest["rows"]}
    candidates = {(filename, candidate["id"]): candidate for filename, data in manifest["files"].items()
                  for candidate in data["candidates"]}
    if set(rows) != set(WITNESSES) | set(ABSENCES) or set(WITNESSES) & set(ABSENCES):
        raise ValueError("reviewed classification differs from accepted census")
    for suffix, identity in WITNESSES.items():
        if identity not in rows[suffix]["candidates"]:
            raise ValueError("reviewed witness outside measured span cohort")
    return rows, candidates


def control_program(manifest, false_positive=False, false_negative=False):
    """Project actual edits into typed selection traces; the book bounds what these controls establish."""
    rows, candidates = selections(manifest)
    functions, checks = [], []

    def exact(suffix):
        return rows[suffix]["exact_replacement"]

    def candidate(suffix, identity=None):
        return candidates[rows[suffix]["file"], identity or WITNESSES[suffix]]["replacement"]

    def traced(name, arguments, prefix, item):
        functions.append("fn %s(%s) -> Vec<String> { let mut out = Vec::new(); %s out.push(%s.clone()); } out }"
                         % (name, arguments, prefix, item))

    loop_cases = [
        ("the-first-generator-alone", "g: &Generated", "file", "g"),
        ("the-last-generator-alone", "g: &Generated", "file", "g"),
        ("chain-and-blob-rules-on-each-clause-s-first-entry-alone", "files: &Vec<String>", "f", "v"),
        ("chain-and-blob-rules-on-each-clause-s-last-entry-alone", "files: &Vec<String>", "f", "v"),
        ("held-judged-for-the-first-alone", "held: &BTreeSet<String>", "pkg", "s"),
        ("a-build-read-naming-its-first-generator-alone", "generators: &BTreeSet<String>", "g", "s"),
    ]
    for i, (suffix, arguments, item, argument) in enumerate(loop_cases):
        prefix = next(line for line in exact(suffix).splitlines() if line.strip().startswith("for " + item + " in"))
        mutated = candidate(suffix, "L1851.iter.741" if false_positive and i == 0 else None)
        traced("exact_loop%d" % i, arguments, prefix, item)
        traced("candidate_loop%d" % i, arguments, mutated, item)
        assertion = "assert_ne!" if false_negative and i == 0 else "assert_eq!"
        checks.append("%s(exact_loop%d(&%s), candidate_loop%d(&%s));" % (assertion, i, argument, i, argument))

    def readers(name, expression):
        functions.append("fn %s(roots: &Roots, reads: &BTreeMap<String, BTreeSet<String>>, g: &Generated) -> Vec<String> {"
                         "let readers: Vec<&Program> = roots.programs.iter() %s "
                         "readers.into_iter().map(|p| p.name.clone()).collect() }" % (name, expression))

    for i, suffix in enumerate(("the-first-reader-alone", "the-last-reader-alone")):
        expression = exact(suffix).split("for file", 1)[0].strip()
        readers("exact_reader%d" % i, expression)
        readers("candidate_reader%d" % i, candidate(suffix).strip() + ".collect();")
    middle = exact("a-middle-reader-never-judged").split("for file", 1)[0].strip()
    readers("middle_readers", middle)

    for i, suffix in enumerate(("both-clauses-the-first-input-alone", "both-clauses-the-last-input-alone")):
        for label, expression in (("exact", exact(suffix)), ("candidate", candidate(suffix))):
            functions.append("fn %s_member%d(f: &Generated) -> BTreeSet<String> { f.generators.iter() %s "
                             ".cloned().collect() }" % (label, i, expression))

    for i, (suffix, identity) in enumerate((("a-middle-input-never-reached", "L341.iter.138"),
                                           ("only-a-form-s-first-input-reached", "L341.iter.136"),
                                           ("a-middle-generator-never-reached", "L341.iter.138"))):
        lines = exact(suffix).split("let p =", 1)[0].splitlines()
        start = next(k for k, line in enumerate(lines) if line.strip().startswith(("for f in", "for (k, f)", "for (index, f)")))
        for label, prefix in (("exact", "\n".join(lines[start:])), ("candidate", candidate(suffix, identity))):
            functions.append("fn %s_provenance%d(g: &Generated) -> Vec<(String, String)> { let mut out = Vec::new(); "
                             "for (role, files) in [(\"generator\", &g.generators), (\"input\", &g.inputs)] { %s "
                             "out.push((role.to_owned(), f.clone())); } } out }" % (label, i, prefix))

    prefix = exact("a-middle-script-never-judged").split("// What", 1)[0]
    traced("exact_scripts", "g: &Generated", prefix, "file")
    traced("candidate_scripts", "g: &Generated", candidate("a-middle-script-never-judged", "L1851.iter.742"), "file")
    # The exact role-gated edits preserve the other role's middle entries. The generic loop edit does not.
    negative_checks = ["assert_ne!(exact_provenance%d(&g), candidate_provenance%d(&g));" % (i, i) for i in range(3)]
    negative_checks += ["assert_ne!(exact_scripts(&scripts), candidate_scripts(&scripts));",
                        "assert_ne!(middle_readers(&roots, &reads, &g), candidate_reader0(&roots, &reads, &g));",
                        "assert_ne!(middle_readers(&roots, &reads, &g), candidate_reader1(&roots, &reads, &g));"]
    suffix = "a-transitive-generator-build-read-dropped"
    # A depth gate retains own and direct packages regardless of unit/file positions. Neither loop narrowing does.
    lines = exact(suffix).split("for f in", 1)[0].splitlines()
    start = next(i for i, line in enumerate(lines) if line.strip() == "for u in &units {")
    prefix = "\n".join(lines[start:])
    functions.append("fn exact_units(pkg: &String, graph: &[(String, String, String)], units: Vec<Unit>) -> Vec<String> {"
                     "let mut out = Vec::new(); %s for f in &u.files { out.push(f.clone()); } } out }" % prefix)
    for i, identity in enumerate(("L1940.iter.803", "L1940.iter.804", "L1940.iter.805",
                                  "L1941.iter.806", "L1941.iter.807", "L1941.iter.808")):
        replacement = candidate(suffix, identity)
        loops = replacement + "for f in &u.files {" if i < 3 else "for u in &units {" + replacement
        functions.append("fn candidate_units%d(units: Vec<Unit>) -> Vec<String> { let mut out = Vec::new(); %s "
                         "out.push(f.clone()); } } out }" % (i, loops))
        negative_checks.append("assert_ne!(exact_units(&pkg, &graph, units.clone()), candidate_units%d(units.clone()));" % i)
    return r'''
#![allow(unused_variables)]
use std::collections::{BTreeMap, BTreeSet};
struct Generated { path: String, generators: Vec<String>, inputs: Vec<String> }
struct Program { name: String }
struct Roots { programs: Vec<Program> }
#[derive(Clone)]
struct Unit { package: String, files: Vec<String> }
fn words(values: &[&str]) -> Vec<String> { values.iter().map(|x| (*x).to_owned()).collect() }
''' + "\n".join(functions) + r'''
fn main() {
    for n in 0..=5 {
        let v: Vec<String> = (0..n).map(|i| format!("{i}")).collect();
        let s: BTreeSet<String> = v.iter().cloned().collect();
        let g = Generated { path: "p".into(), generators: v.clone(), inputs: Vec::new() };
''' + "\n".join(checks) + r'''
        let roots = Roots { programs: v.iter().cloned().map(|name| Program { name }).collect() };
        for mask in 0..(1usize << n) {
            let reads: BTreeMap<String, BTreeSet<String>> = v.iter().enumerate().map(|(i, name)|
                (name.clone(), if mask & (1 << i) != 0 { [g.path.clone()].into() } else { BTreeSet::new() })).collect();
            assert_eq!(exact_reader0(&roots, &reads, &g), candidate_reader0(&roots, &reads, &g));
            assert_eq!(exact_reader1(&roots, &reads, &g), candidate_reader1(&roots, &reads, &g));
            let inputs: Vec<String> = v.iter().enumerate().filter(|(i, _)| mask & (1 << i) != 0).map(|(_, x)| x.clone()).collect();
            let mut leading = words(&["unmatched"]); leading.extend(inputs.iter().cloned());
            let mut trailing = inputs.clone(); trailing.push("unmatched".into());
            let mut duplicate = inputs.clone(); duplicate.extend(inputs.iter().cloned());
            for inputs in [inputs, leading, trailing, duplicate] {
                let f = Generated { path: "p".into(), generators: v.clone(), inputs };
                assert_eq!(exact_member0(&f), candidate_member0(&f));
                assert_eq!(exact_member1(&f), candidate_member1(&f));
            }
        }
    }
    let g = Generated { path: "p".into(), generators: words(&["ga", "gb", "gc"]), inputs: words(&["ia", "ib", "ic"]) };
    let scripts = Generated { path: "p".into(), generators: words(&["a.sh", "b.rs", "c.sh"]), inputs: Vec::new() };
    let roots = Roots { programs: words(&["a", "b", "c"]).into_iter().map(|name| Program { name }).collect() };
    let reads: BTreeMap<String, BTreeSet<String>> = roots.programs.iter().map(|p| (p.name.clone(), [g.path.clone()].into())).collect();
    let pkg = "root".to_owned();
    let graph = [(pkg.clone(), "direct".to_owned(), "lib".to_owned())];
    let units = vec![Unit { package: pkg.clone(), files: words(&["a"]) },
                     Unit { package: "direct".into(), files: words(&["b"]) },
                     Unit { package: "transitive".into(), files: words(&["c"]) }];
''' + "\n".join(negative_checks) + '\nprintln!("trust-effects: selection checks and contrasting controls passed");\n}\n'


def compile_control(root, manifest, name, **kwargs):
    directory = root / "target/trust-witness-effects"
    directory.mkdir(parents=True, exist_ok=True)
    source, binary = directory / (name + ".rs"), directory / name
    source.write_text(control_program(manifest, **kwargs))
    env = dict(os.environ, CARGO_HOME=str(root / "target/cargo-home"), TMPDIR=str(root / "target/tmp"))
    edition = witness.formatter_command(root)[2]
    compile_result = subprocess.run(["rustc", "--edition=" + edition, str(source), "-o", str(binary)],
                                    cwd=root, env=env, text=True, capture_output=True, timeout=60)
    if compile_result.returncode:
        raise ValueError("selection control did not compile: " + compile_result.stderr)
    result = subprocess.run([str(binary)], cwd=root, env=env, text=True, capture_output=True, timeout=60)
    return dict(source_sha256=adequacy.digest(source.read_bytes()), exit_code=result.returncode,
                stdout=result.stdout, stderr=result.stderr)


def spelling_audit(root, data):
    """Check every selected first/last/ends spelling; census only the actual plain headers in these source spans."""
    groups = {}
    for filename, file_data in data["files"].items():
        intervals = [row["span"] for row in data["rows"] if row["file"] == filename]
        source = (root / filename).read_text().split("\n")
        # This independent census is intentionally scoped to the reviewed plain, one-line source headers.
        # It is not a replacement for the sweep's general Rust lexical support.
        headers = {i for i, line in enumerate(source) if any(lo <= i < hi for lo, hi in intervals)
                   and re.fullmatch(r"\s*for .+ in .+ \{\s*", line)}
        iterator_headers = set()
        for candidate in file_data["candidates"]:
            operator = candidate["operator"]
            if operator not in ("iter", "guard"):
                continue
            lo, hi = candidate["span"]
            if operator == "iter":
                iterator_headers.add(lo)
                gate = candidate["replacement"].rsplit("; if !", 1)[-1].split("{ continue; }", 1)[0].strip()
            else:
                gate = candidate["replacement"].splitlines()[-1].rsplit(" && ", 1)[-1].rsplit("{", 1)[0].strip()
            gate = re.sub(r"__archogen_sweep_\w+_(first|last)\b", r"\1", gate)
            if gate not in ("first", "last", "(first || last)"):
                raise ValueError("unrecognized narrowing spelling: " + candidate["id"])
            key = (filename, operator, lo, hi)
            group = groups.setdefault(key, {})
            if gate in group:
                raise ValueError("duplicate narrowing position")
            if set(candidate["outcomes"]) != set(adequacy.POPULATIONS) or any(
                    outcome["outcome"] not in ("killed", "survived") for outcome in candidate["outcomes"].values()):
                raise ValueError("promised narrowing lacks viable native spelling: " + candidate["id"])
            group[gate] = dict(id=candidate["id"], outcomes=candidate["outcomes"])
        if iterator_headers != headers:
            raise ValueError("selected iterator families differ from in-span source-header census")
    if not groups or any(set(group) != {"first", "last", "(first || last)"} for group in groups.values()):
        raise ValueError("missing promised first/last/ends spelling")
    result = [dict(file=filename, operator=operator, span=[lo, hi], positions=positions)
              for (filename, operator, lo, hi), positions in groups.items()]
    counts = {operator: sum(group["operator"] == operator for group in result) for operator in ("iter", "guard")}
    return dict(schema="archogen-trust-narrowing-spellings/1", scope="initial measured census-span cohort only",
                groups=result, family_counts=counts, unique_candidates=sum(len(group) for group in groups.values()),
                native_attempts=sum(len(group) * len(adequacy.POPULATIONS) for group in groups.values()),
                repair_required=False)


def spelling_controls(root, manifest):
    data = copy.deepcopy(manifest)
    for file_data in data["files"].values():
        for candidate in file_data["candidates"]:
            candidate["outcomes"] = {population: dict(outcome="survived") for population in adequacy.POPULATIONS}
    spelling_audit(root, data)
    filename = next(iter(data["files"]))
    candidates = data["files"][filename]["candidates"]
    first = next(i for i, c in enumerate(candidates) if c["operator"] == "iter")
    for arm in ("missing", "family", "duplicate", "unviable", "timeout", "unjudged", "population"):
        changed = copy.deepcopy(data)
        selected = changed["files"][filename]["candidates"]
        if arm == "missing":
            del selected[first]
        elif arm == "family":
            lo = selected[first]["span"][0]
            selected[:] = [c for c in selected if c["operator"] != "iter" or c["span"][0] != lo]
        elif arm == "duplicate":
            selected.append(copy.deepcopy(selected[first]))
        elif arm == "population":
            del selected[first]["outcomes"]["parent"]
        else:
            selected[first]["outcomes"]["parent"]["outcome"] = arm
        try:
            spelling_audit(root, changed)
        except ValueError:
            continue
        raise ValueError("spelling refusal control accepted: " + arm)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--self-test", action="store_true")
    parser.add_argument("--evidence")
    parser.add_argument("--output")
    parser.add_argument("--spellings", action="store_true", help="audit all selected narrowing variants and native viability")
    args = parser.parse_args()
    root = Path(__file__).resolve().parent.parent
    if args.self_test:
        manifest = adequacy.plan(root)
        spelling_controls(root, manifest)
        result = compile_control(root, manifest, "positive")
        if result["exit_code"] != 0 or "contrasting controls passed" not in result["stdout"]:
            raise ValueError("positive selection controls failed: " + str(result))
        for name in ("false_positive", "false_negative"):
            result = compile_control(root, manifest, name, **{name: True})
            if result["exit_code"] != 101 or "assertion `left" not in result["stderr"]:
                raise ValueError("seeded false semantic claim was not refused: " + str(result))
        print("trust-effects: OK — compiled selection controls; wrong traces and seven spelling breaches refused")
        return 0
    if not args.evidence or not args.output:
        parser.error("--evidence and --output required")
    original = adequacy.local_output(root, args.evidence)
    output = adequacy.local_output(root, args.output)
    data = json.loads(original.read_text())
    regenerated = witness.evidence(root, data["plan"], [m["measurement"] for m in data["measurements"]])
    if regenerated != data:
        raise ValueError("evidence differs from re-audited native/source/syntax data")
    protected = [original, root / data["plan"]]
    protected += [root / m[k] for m in data["measurements"] for k in ("measurement", "log")]
    if output.resolve() in {p.resolve() for p in protected}:
        raise ValueError("output must not overwrite evidence inputs")
    if args.spellings:
        result = spelling_audit(root, data)
        result.update(evidence=args.evidence, evidence_sha256=adequacy.digest(original.read_bytes()),
                      producer_sha256=adequacy.digest(Path(__file__).read_bytes()))
        output.parent.mkdir(parents=True, exist_ok=True)
        output.write_text(json.dumps(result, indent=2) + "\n")
        print("trust-effects: %d iterator / %d guard families; %d viable candidates in both populations; no spelling repair"
              % (result["family_counts"]["iter"], result["family_counts"]["guard"], result["unique_candidates"]))
        return 0
    rows, candidates = selections(data)
    controls = compile_control(root, data, "measured")
    if controls["exit_code"] != 0:
        raise ValueError("compiled effect controls failed: " + str(controls))
    classified = []
    for suffix, row in rows.items():
        item = dict(suffix=suffix, file=row["file"], findings=row["findings"])
        if suffix in WITNESSES:
            identity = WITNESSES[suffix]
            measured = candidates[row["file"], identity]["outcomes"]
            if any(result["outcome"] not in ("killed", "survived") for result in measured.values()):
                raise ValueError("reviewed witness has no viable native outcome")
            item.update(witness=identity, kind="syntax" if identity in row["exact"]["syntax_match_ids"] else "effect",
                        outcomes=measured)
        else:
            item.update(witness=None, reason=ABSENCES[suffix])
        classified.append(item)
    result = dict(schema="archogen-trust-witness-classification/1", scope="reviewed initial replacement-span cohort only",
                  evidence=args.evidence, evidence_sha256=adequacy.digest(original.read_bytes()),
                  producer_sha256=adequacy.digest(Path(__file__).read_bytes()), controls=controls, rows=classified)
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(result, indent=2) + "\n")
    print("trust-effects: %d reviewed rows; %d viable witnesses, %d scoped absences; compiled selection controls passed"
          % (len(classified), len(WITNESSES), len(ABSENCES)))
    return 0


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (OSError, ValueError, subprocess.SubprocessError) as error:
        print("trust-effects: refused: " + str(error), file=sys.stderr)
        sys.exit(2)
