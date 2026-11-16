// The loader of the configuration checker's browser module (docs/decisions/decision_wasm-binding.md §11).
//
// The one file a page and the repository's harness both load (scripts/plan_check_wasm.mjs), so what the harness
// checks is what a page runs. It uses only `WebAssembly`, `crypto.subtle`, `TextEncoder` and `TextDecoder`, which
// browsers and Node both provide, and it gives the module nothing to import: the module imports nothing.

/** The request's format, the first field of every request (§11). */
export const REQUEST_FORMAT = "archogen-plan-check-wasm-request/1";

/** A 4-byte little-endian integer. */
function u32(value) {
  const bytes = new Uint8Array(4);
  new DataView(bytes.buffer).setUint32(0, value, true);
  return bytes;
}

/** `bytes`' SHA-256, written as the result record writes a digest: `sha256:` and 64 lowercase hexadecimal digits. */
export async function digest(bytes) {
  const hash = new Uint8Array(await crypto.subtle.digest("SHA-256", bytes));
  return "sha256:" + Array.from(hash, (byte) => byte.toString(16).padStart(2, "0")).join("");
}

/**
 * Frame a request (§11): the format, the module's digest, the description's name and text, the profile (empty for
 * the default), the number of modules, then each module's name and text, and the plan. Every field is a length, then
 * that many bytes of UTF-8.
 */
export function frame({ producer, name = "description.eadl", description, profile = "", modules = {}, plan }) {
  const encoder = new TextEncoder();
  const parts = [];
  const field = (value) => {
    const bytes = encoder.encode(value);
    parts.push(u32(bytes.length), bytes);
  };
  field(REQUEST_FORMAT);
  field(producer);
  field(name);
  field(description);
  field(profile ?? "");
  const entries = Object.entries(modules);
  parts.push(u32(entries.length));
  for (const [module, moduleText] of entries) {
    field(module);
    field(moduleText);
  }
  field(plan);
  const out = new Uint8Array(parts.reduce((total, part) => total + part.length, 0));
  let at = 0;
  for (const part of parts) {
    out.set(part, at);
    at += part.length;
  }
  return out;
}

/**
 * Instantiate the module from its bytes, digesting them once: the digest is the producer every record names, since a
 * module cannot read its own file. The result answers a request — `{ description, plan, name, profile, modules }` —
 * as the text the module wrote (`answer`), or parsed (`check`): the result record, `archogen-result/1`, or a refusal,
 * `archogen-plan-check-wasm-refusal/1`, told apart by their `format`.
 */
export async function instantiate(bytes) {
  const producer = await digest(bytes);
  const { instance } = await WebAssembly.instantiate(bytes, {});
  const { memory, plan_check_input, plan_check, plan_check_output } = instance.exports;
  const decoder = new TextDecoder("utf-8", { fatal: true });

  /** The answer to one request, as the text the module wrote. */
  function answer(request) {
    const bytes = frame({ ...request, producer });
    // Addresses and lengths are `usize`, 32 bits on wasm32, which JavaScript receives as a signed `i32`: `>>> 0`
    // reads them unsigned.
    const at = plan_check_input(bytes.length) >>> 0;
    if (at === 0) {
      throw new RangeError(`the request is ${bytes.length} bytes, above what the module accepts`);
    }
    // A view is taken after each call, because a call that grows the memory detaches the views taken before it.
    new Uint8Array(memory.buffer, at, bytes.length).set(bytes);
    const length = plan_check() >>> 0;
    const output = plan_check_output() >>> 0;
    return decoder.decode(new Uint8Array(memory.buffer, output, length));
  }

  return {
    producer,
    answer,
    /** The answer to one request, parsed. */
    check: (request) => JSON.parse(answer(request)),
  };
}
