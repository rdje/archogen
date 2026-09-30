// The page's logic (leaf `API.5.4`): what it shows for a response, and how it wires the document to the binding.
//
// `show` is plain text from a response, and runs in Node as well as in a browser, so the harness
// (scripts/wasm_binding.sh) holds the book's transcript of the page to it. The binding is reached through
// ../js/archogen.mjs, the loader the harness checks, so the page runs what was checked.

import { instantiate } from "../js/archogen.mjs";

/** Where the artifact is, relative to this file, once `cargo build --release -p archogen-wasm --target
 * wasm32-unknown-unknown` has built it. */
export const ARTIFACT = "../../../target/wasm32-unknown-unknown/release/archogen_wasm.wasm";

/** The page's request for a description typed into it: named `description.eadl`, with no modules. */
export function request(text, profile = "") {
  return { name: "description.eadl", text, profile, modules: {} };
}

/**
 * What the page shows for a response: the outcome and its exit code, then what the command line would print —
 * the diagnostics, or the notes and hint of a request that was not judged, or what an accepted description was
 * judged under.
 */
export function show(response) {
  const lines = [`${response.status} (exit ${response.exit})`];
  for (const note of response.notes) {
    lines.push(`note: ${note}`);
  }
  if (response.hint !== null) {
    lines.push(`hint: ${response.hint}`);
  }
  if (response.judged !== null && response.status === "ok") {
    const { profile, language, declarations } = response.judged;
    lines.push(`accepted against profile ${profile} (${language}), ${declarations} declaration(s)`);
  }
  if (response.rendered !== "") {
    lines.push(response.rendered.replace(/\n$/, ""));
  }
  return lines.join("\n");
}

/**
 * Wire a document: the description typed in, the profile, the button, and where the answer goes. The page calls it
 * on load; the harness calls it with a stand-in document, so the wiring it checks is this one.
 */
export async function wire(document) {
  const output = document.getElementById("output");
  let binding;
  try {
    const response = await fetch(new URL(ARTIFACT, import.meta.url));
    if (!response.ok) {
      throw new Error(`${response.status} fetching the module`);
    }
    binding = await instantiate(await response.arrayBuffer());
  } catch (error) {
    output.textContent =
      `The module could not be loaded (${error.message}). Build it first:\n` +
      "  cargo build --release -p archogen-wasm --target wasm32-unknown-unknown";
    return;
  }
  const check = () => {
    const response = binding.check(
      request(document.getElementById("description").value, document.getElementById("profile").value.trim()),
    );
    output.textContent = show(response);
  };
  document.getElementById("check").addEventListener("click", check);
  check();
}

if (typeof document !== "undefined") {
  wire(document);
}
