// The configuration checker's browser module checked as an artifact (leaf `M3.5.7.2`,
// docs/decisions/decision_wasm-binding.md §11). Driven by scripts/plan_check_wasm.sh.
//
//   node scripts/plan_check_wasm.mjs inspect ARTIFACT
//       exit 1 unless the artifact imports nothing and exports exactly what §11 lists
//   node scripts/plan_check_wasm.mjs answer ARTIFACT CASES
//       the artifact's answer to each case of CASES (the host example's cases.jsonl), one per line, reached through
//       crates/archogen-plan-check-wasm/js/plan-check.mjs as a page reaches it
//   node scripts/plan_check_wasm.mjs dirs CASES
//       each case's directory and description name, tab-separated, for the executable's run
//   node scripts/plan_check_wasm.mjs compare CASES HOST WASM EXEC
//       exit 1 unless, case by case, the artifact's answer is the host build's byte for byte, and agrees with the
//       executable's run over the case's files (EXEC: its exit code, a tab, its standard output): a record equal to
//       the executable's but for its producer, with the executable's exit code; a refusal where the executable
//       refused the call, exit 2 and no record. One text is set aside: the repair of a `module-not-found` finding,
//       which the executable writes naming the file it looked for, and the module, given texts, cannot — the one
//       place the module source describes itself (`eadl_front::module::ModuleSource::describe_missing`)

import { readFileSync } from "node:fs";
import { resolve } from "node:path";
import { pathToFileURL } from "node:url";

const RECORD_FORMAT = "archogen-result/1";
const EXPORTS = [
  "memory:memory",
  "plan_check:function",
  "plan_check_input:function",
  "plan_check_output:function",
  "__data_end:global",
  "__heap_base:global",
];

/** The lines of `file`, its last newline not a line of its own. */
function lines(file) {
  const text = readFileSync(file, "utf8");
  return text === "" ? [] : text.replace(/\n$/, "").split("\n");
}

function inspect(artifact) {
  const module = new WebAssembly.Module(readFileSync(artifact));
  const problems = [];
  for (const { module: from, name, kind } of WebAssembly.Module.imports(module)) {
    problems.push(`the artifact imports ${kind} \`${from}.${name}\`, and it may import nothing`);
  }
  const exported = WebAssembly.Module.exports(module).map(({ name, kind }) => `${name}:${kind}`);
  for (const entry of exported) {
    if (!EXPORTS.includes(entry)) {
      const [name, kind] = entry.split(":");
      problems.push(`the artifact exports ${kind} \`${name}\`, which the record does not list`);
    }
  }
  for (const entry of EXPORTS) {
    if (!exported.includes(entry)) {
      const [name, kind] = entry.split(":");
      problems.push(`the artifact does not export the ${kind} \`${name}\` the record lists`);
    }
  }
  return problems;
}

async function answer(artifact, cases) {
  const loader = await import(
    pathToFileURL(resolve("crates/archogen-plan-check-wasm/js/plan-check.mjs")).href
  );
  const module = await loader.instantiate(readFileSync(artifact));
  for (const line of lines(cases)) {
    const { name, description, modules, plan } = JSON.parse(line);
    process.stdout.write(module.answer({ name, description, modules, plan }));
  }
}

/** The code whose repair names how its module source looks for a module. */
const MISSING = "module-not-found";

/** The record, without its producer and its missing modules' repairs, as one comparable text. */
function comparable(record) {
  const { producer, findings, ...rest } = record;
  const kept = (findings ?? []).map(({ repair, ...finding }) =>
    finding.code === MISSING ? finding : { ...finding, repair },
  );
  return JSON.stringify({ ...rest, findings: kept });
}

function compare(cases, host, wasm, exec) {
  const [described, native, artifact, ran] = [cases, host, wasm, exec].map(lines);
  const problems = [];
  let missing = 0;
  described.forEach((line, index) => {
    const { case: label } = JSON.parse(line);
    if (index >= native.length) return problems.push(`${label}: the host build gave no answer`);
    if (index >= artifact.length) return problems.push(`${label}: the artifact gave no answer`);
    if (index >= ran.length) return problems.push(`${label}: the executable did not run`);
    if (native[index] !== artifact[index]) {
      return problems.push(`${label}: the artifact's answer differs from the host build's`);
    }
    const answered = JSON.parse(artifact[index]);
    const tab = ran[index].indexOf("\t");
    const code = Number(ran[index].slice(0, tab));
    const output = ran[index].slice(tab + 1);
    if (answered.format === RECORD_FORMAT) {
      if (answered.producer?.name !== "archogen-plan-check-wasm") {
        return problems.push(`${label}: the artifact's record names \`${answered.producer?.name}\` as its producer`);
      }
      if (output === "") {
        return problems.push(`${label}: the artifact judged it, and the executable exited ${code} with no record`);
      }
      const record = JSON.parse(output);
      if (code !== answered.exit) {
        return problems.push(`${label}: the artifact's record says exit ${answered.exit}, and the executable exits ${code}`);
      }
      if (comparable(record) !== comparable(answered)) {
        return problems.push(`${label}: the artifact's record differs from the executable's beyond its producer`);
      }
      if (answered.findings.some(({ code }) => code === MISSING)) missing += 1;
    } else if (code !== 2 || output !== "") {
      return problems.push(`${label}: the artifact refused it (${answered.note}), and the executable exited ${code}`);
    }
  });
  if (native.length > described.length || artifact.length > described.length || ran.length > described.length) {
    problems.push("a side gave more answers than there are cases");
  }
  return { problems, count: described.length, missing };
}

const [command, ...args] = process.argv.slice(2);
switch (command) {
  case "inspect": {
    const problems = inspect(args[0]);
    for (const problem of problems) console.error(`plan-check-wasm: ${problem}`);
    process.exit(problems.length ? 1 : 0);
    break;
  }
  case "answer":
    await answer(args[0], args[1]);
    break;
  case "dirs":
    for (const line of lines(args[0])) {
      const { dir, name } = JSON.parse(line);
      console.log(`${dir}\t${name}`);
    }
    break;
  case "compare": {
    const { problems, count, missing } = compare(...args);
    for (const problem of problems) console.log(problem);
    if (problems.length) process.exit(1);
    console.log(`${count} case(s), ${missing} with a module not found, whose repair alone was set aside,`);
    break;
  }
  default:
    console.error("usage: node scripts/plan_check_wasm.mjs inspect|answer|dirs|compare …");
    process.exit(2);
}
