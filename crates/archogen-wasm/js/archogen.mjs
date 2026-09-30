// The loader of archogen's wasm binding (docs/decisions/decision_wasm-binding.md §9).
//
// The one file a page and the repository's harness both load (scripts/wasm_binding.mjs), so what the harness
// checks is what a page runs. It uses only `WebAssembly`, `TextEncoder` and `TextDecoder`, which browsers and Node
// both provide, and it gives the module nothing to import: the module imports nothing (§7).

/** The request's format, the first field of every request (§4). */
export const REQUEST_FORMAT = "archogen-wasm-request/1";

/** A 4-byte little-endian integer. */
function u32(value) {
  const bytes = new Uint8Array(4);
  new DataView(bytes.buffer).setUint32(0, value, true);
  return bytes;
}

/**
 * Frame a request (§4): the format, the name, the text, the profile (empty for the default), the number of
 * modules, then each module's name and text. Every field is a length, then that many bytes of UTF-8.
 */
export function frame({ name, text, profile = "", modules = {} }) {
  const encoder = new TextEncoder();
  const parts = [];
  const field = (value) => {
    const bytes = encoder.encode(value);
    parts.push(u32(bytes.length), bytes);
  };
  field(REQUEST_FORMAT);
  field(name);
  field(text);
  field(profile ?? "");
  const entries = Object.entries(modules);
  parts.push(u32(entries.length));
  for (const [module, moduleText] of entries) {
    field(module);
    field(moduleText);
  }
  const out = new Uint8Array(parts.reduce((total, part) => total + part.length, 0));
  let at = 0;
  for (const part of parts) {
    out.set(part, at);
    at += part.length;
  }
  return out;
}

/**
 * Instantiate the binding from the module's bytes. The result answers a request as the response's JSON text
 * (`answer`) or as the parsed response (`check`).
 */
export async function instantiate(bytes) {
  const { instance } = await WebAssembly.instantiate(bytes, {});
  const { memory, archogen_input, archogen_check, archogen_output } = instance.exports;
  const decoder = new TextDecoder("utf-8", { fatal: true });

  /** The response to one request, as the JSON text the module wrote (§6). */
  function answer(request) {
    const bytes = frame(request);
    // Addresses and lengths are `usize`, 32 bits on wasm32, which JavaScript receives as a signed `i32`: `>>> 0`
    // reads them unsigned.
    const at = archogen_input(bytes.length) >>> 0;
    if (at === 0) {
      throw new RangeError(`the request is ${bytes.length} bytes, above what the module accepts`);
    }
    // A view is taken after each call, because a call that grows the memory detaches the views taken before it.
    new Uint8Array(memory.buffer, at, bytes.length).set(bytes);
    const length = archogen_check() >>> 0;
    const output = archogen_output() >>> 0;
    return decoder.decode(new Uint8Array(memory.buffer, output, length));
  }

  return {
    answer,
    /** The response to one request, parsed. */
    check: (request) => JSON.parse(answer(request)),
  };
}
