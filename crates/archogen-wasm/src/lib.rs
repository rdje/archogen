//! The WebAssembly binding over the engine API (leaf `API.5`), as `docs/decisions/decision_wasm-binding.md`
//! decides it.
//!
//! A page loads the `cdylib` this crate builds for `wasm32-unknown-unknown`, and drives it through three exports:
//!
//! 1. [`archogen_input`] gives it a buffer of the request's length, and the page writes the request there;
//! 2. [`archogen_check`] reads the request, answers it and returns the response's length;
//! 3. [`archogen_output`] gives the response's address, and the page reads it.
//!
//! The request is length-framed ([`request`]), and the response is JSON in one fixed encoding ([`json`]). The
//! judgement is the engine API's, [`archogen_api::check_with`] under the instance's own budget. Everything between
//! the buffers and the API is [`answer`], a pure function the host's tests call directly.
//!
//! ⛔ **No `unsafe` block.** Rust hands out the addresses of buffers it owns, which is a safe cast, and the page
//! writes and reads linear memory between calls, when no Rust borrow is live. The lint's only exceptions are the
//! three `#[no_mangle]` attributes, because the compiler counts an unmangled symbol as unsafe code (the record's
//! §3). ⛔ **No imports**: the module can reach nothing outside itself, which `API.5.3` checks on the artifact.

#![deny(unsafe_code)]

pub mod json;
pub mod request;

use std::cell::RefCell;

use archogen_api::{check_with, Limits, MemoryModules, Request, Response, SourceMap, Status};

/// The request's format identifier, the first field of every request (the record's §4).
pub const REQUEST_FORMAT: &str = "archogen-wasm-request/1";

/// The response's format identifier, the first key of every response (the record's §6).
pub const RESPONSE_FORMAT: &str = "archogen-wasm-response/1";

/// The largest request [`archogen_input`] allocates for: four times the API's budget. It only stops an allocation
/// the budget would refuse anyway; the budget itself is [`Limits::DEFAULT`], which the instance sets.
pub const INPUT_CAP: usize = 4 * archogen_api::DEFAULT_BYTES;

/// Answer one framed request: the whole binding, without the buffers.
///
/// A request that breaks the framing is answered with status `usage` and a note naming the field and its byte
/// offset, and the description is not read. Otherwise the answer is the engine API's.
#[must_use]
pub fn answer(bytes: &[u8]) -> String {
    let response = match request::parse(bytes) {
        Ok(framed) => {
            let modules = framed
                .modules
                .iter()
                .fold(MemoryModules::new(), |modules, (name, text)| {
                    modules.with(name, text)
                });
            check_with(
                &Request {
                    name: &framed.name,
                    text: &framed.text,
                    profile: framed.profile.as_deref(),
                    modules: &modules,
                },
                Limits::DEFAULT,
            )
        }
        Err(error) => Response {
            version: archogen_api::VERSION,
            engine: archogen_api::ENGINE,
            status: Status::Usage,
            notes: vec![error.to_string()],
            hint: Some(format!(
                "write the request as `{REQUEST_FORMAT}` frames it: see docs/decisions/decision_wasm-binding.md §4"
            )),
            diagnostics: Vec::new(),
            sources: SourceMap::new(),
            judged: None,
        },
    };
    json::encode(&response)
}

thread_local! {
    /// The request, as the page wrote it.
    static INPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    /// The last response, until the next call of any export.
    static OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Replace the input buffer with `len` zero bytes and return its address, where the page writes the request. When
/// `len` is above [`INPUT_CAP`], allocate nothing and return a null address.
#[allow(unsafe_code)] // the record's §3: `#[no_mangle]` is the lint's only exception
#[no_mangle]
pub extern "C" fn archogen_input(len: usize) -> *mut u8 {
    OUTPUT.with(|output| output.borrow_mut().clear());
    INPUT.with(|input| {
        let mut input = input.borrow_mut();
        if len > INPUT_CAP {
            *input = Vec::new();
            return core::ptr::null_mut();
        }
        *input = vec![0; len];
        input.as_mut_ptr()
    })
}

/// Answer the request in the input buffer, empty the input buffer, keep the response in the output buffer, and
/// return the response's length in bytes.
#[allow(unsafe_code)] // the record's §3: `#[no_mangle]` is the lint's only exception
#[no_mangle]
pub extern "C" fn archogen_check() -> usize {
    let request = INPUT.with(|input| core::mem::take(&mut *input.borrow_mut()));
    let response = answer(&request).into_bytes();
    let length = response.len();
    OUTPUT.with(|output| *output.borrow_mut() = response);
    length
}

/// The address of the last response, valid until the next call of any export.
#[allow(unsafe_code)] // the record's §3: `#[no_mangle]` is the lint's only exception
#[no_mangle]
pub extern "C" fn archogen_output() -> *const u8 {
    OUTPUT.with(|output| output.borrow().as_ptr())
}
