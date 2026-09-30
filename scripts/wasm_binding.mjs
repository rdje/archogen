// scripts/wasm_binding.mjs — the wasm binding's artifact, as a page would run it (leaf `API.5.3`,
// docs/decisions/decision_wasm-binding.md §7 and §8). Driven by scripts/wasm_binding.sh.
//
//   node scripts/wasm_binding.mjs inspect <module.wasm>
//     Read the artifact with the platform's own parser: it must import nothing, and export exactly the record's
//     functions, its memory and the linker's two globals. Exit 0 as decided, 1 otherwise, naming each difference.
//   node scripts/wasm_binding.mjs answer <module.wasm> <description.eadl>…
//     Inspect it, then answer each description through crates/archogen-wasm/js/archogen.mjs, the loader a page
//     loads, one JSON response per line. The request is what `archogen check <path>` asks: the path as the name,
//     the default profile, and every `*.eadl` of the description's directory as a module under its stem.

import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { instantiate } from "../crates/archogen-wasm/js/archogen.mjs";

/** What the record's §2 lets the artifact export, and of which kind. */
const EXPORTS = new Map([
  ["memory", "memory"],
  ["archogen_input", "function"],
  ["archogen_check", "function"],
  ["archogen_output", "function"],
  ["__data_end", "global"],
  ["__heap_base", "global"],
]);

/** Every way the artifact differs from what the record decides; empty when it agrees. */
function inspect(bytes) {
  const module = new WebAssembly.Module(bytes);
  const problems = [];
  for (const { module: from, name, kind } of WebAssembly.Module.imports(module)) {
    problems.push(`imports ${kind} \`${from}.${name}\` — the module may import nothing (§7)`);
  }
  const exported = new Map(WebAssembly.Module.exports(module).map(({ name, kind }) => [name, kind]));
  for (const [name, kind] of exported) {
    if (!EXPORTS.has(name)) {
      problems.push(`exports ${kind} \`${name}\`, which the record does not list (§2)`);
    } else if (EXPORTS.get(name) !== kind) {
      problems.push(`exports \`${name}\` as a ${kind}, and the record says ${EXPORTS.get(name)} (§2)`);
    }
  }
  for (const [name, kind] of EXPORTS) {
    if (!exported.has(name)) {
      problems.push(`does not export ${kind} \`${name}\` (§2)`);
    }
  }
  return problems;
}

/** The request `archogen check <path>` makes. */
function request(path) {
  const dir = dirname(path);
  const modules = {};
  for (const entry of readdirSync(dir).sort()) {
    const file = join(dir, entry);
    if (entry.endsWith(".eadl") && statSync(file).isFile()) {
      modules[entry.slice(0, -".eadl".length)] = readFileSync(file, "utf8");
    }
  }
  return { name: path, text: readFileSync(path, "utf8"), profile: "", modules };
}

const [mode, wasm, ...paths] = process.argv.slice(2);
if (!["inspect", "answer"].includes(mode) || !wasm) {
  console.error("usage: node scripts/wasm_binding.mjs inspect <module.wasm> | answer <module.wasm> <description>…");
  process.exit(2);
}
const bytes = readFileSync(wasm);
const problems = inspect(bytes);
for (const problem of problems) {
  console.error(`wasm-binding: ${wasm} ${problem}`);
}
if (problems.length > 0) {
  process.exit(1);
}
if (mode === "answer") {
  const binding = await instantiate(bytes);
  for (const path of paths) {
    process.stdout.write(binding.answer(request(path)) + "\n");
  }
}
