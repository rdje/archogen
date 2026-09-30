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
//   node scripts/wasm_binding.mjs show <module.wasm> <file>
//     Inspect it, then print what the page (crates/archogen-wasm/page/) shows for the file's text typed into it:
//     the page's own `show`, over the page's own request.
//   node scripts/wasm_binding.mjs page <module.wasm> <file>
//     Inspect it, then run the page's own wiring (page.mjs) against a stand-in document whose `fetch` reads the
//     file the page's relative URL names: on load it must show its default description's answer (read from
//     index.html), and after Check with the file's text, that text's answer. Exit 1 on either difference.

import { readFileSync, readdirSync, statSync } from "node:fs";
import { dirname, join } from "node:path";
import { fileURLToPath } from "node:url";
import { instantiate } from "../crates/archogen-wasm/js/archogen.mjs";
import { request as pageRequest, show, wire } from "../crates/archogen-wasm/page/page.mjs";

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
if (!["inspect", "answer", "show", "page"].includes(mode) || !wasm) {
  console.error(
    "usage: node scripts/wasm_binding.mjs inspect <module.wasm> | answer <module.wasm> <description>… | " +
      "show <module.wasm> <file> | page <module.wasm> <file>",
  );
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
if (mode === "show") {
  // The page's own request for the file's text, typed in with the default profile.
  const binding = await instantiate(bytes);
  process.stdout.write(show(binding.check(pageRequest(readFileSync(paths[0], "utf8")))) + "\n");
}
if (mode === "page") {
  const binding = await instantiate(bytes);
  const expected = (text) => show(binding.check(pageRequest(text)));
  const html = readFileSync(new URL("../crates/archogen-wasm/page/index.html", import.meta.url), "utf8");
  const found = html.match(/<textarea id="description"[^>]*>([\s\S]*?)<\/textarea>/);
  if (!found) {
    console.error("wasm-binding: crates/archogen-wasm/page/index.html has no textarea#description");
    process.exit(1);
  }
  const unescape = (text) => text.replaceAll("&lt;", "<").replaceAll("&gt;", ">").replaceAll("&amp;", "&");
  const elements = {
    description: { value: unescape(found[1]) },
    profile: { value: "" },
    output: { textContent: "" },
    check: { listeners: [], addEventListener(type, listener) { if (type === "click") this.listeners.push(listener); } },
  };
  let fetched = "";
  globalThis.fetch = async (url) => {
    fetched = fileURLToPath(url);
    const file = readFileSync(fetched);
    return { ok: true, status: 200, arrayBuffer: async () => file.buffer.slice(file.byteOffset, file.byteOffset + file.byteLength) };
  };
  await wire({ getElementById: (id) => elements[id] });
  const problems = [];
  if (fetched === "") {
    problems.push("the page never fetched its module");
  } else if (readFileSync(fetched).compare(bytes) !== 0) {
    problems.push(`the page fetched ${fetched}, which is not the module under test`);
  }
  if (elements.output.textContent !== expected(elements.description.value)) {
    problems.push(`on load the page showed:\n${elements.output.textContent}`);
  }
  elements.description.value = readFileSync(paths[0], "utf8");
  elements.check.listeners.forEach((listener) => listener());
  if (elements.output.textContent !== expected(elements.description.value)) {
    problems.push(`after Check the page showed:\n${elements.output.textContent}`);
  }
  for (const problem of problems) {
    console.error(`wasm-binding: ${problem}`);
  }
  process.exit(problems.length > 0 ? 1 : 0);
}
