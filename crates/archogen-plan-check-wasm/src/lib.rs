//! The configuration checker's browser module (leaf `M3.5.7.1`), as `docs/decisions/decision_wasm-binding.md` §11
//! decides it: its own module, never the generator's, so a page judging a plan runs no generator code.
//!
//! A page loads the `cdylib` this crate builds for `wasm32-unknown-unknown`, and drives it through three exports, as
//! it drives the generator's module through its own three:
//!
//! 1. [`plan_check_input`] gives it a buffer of the request's length, and the page writes the request there;
//! 2. [`plan_check`] reads the request, answers it and returns the answer's length;
//! 3. [`plan_check_output`] gives the answer's address, and the page reads it.
//!
//! The request is length-framed ([`request`]): the module's own digest, which the loader computes from the bytes it
//! instantiated since a module cannot read its own file; the description, its modules and the profile; and the plan.
//! The answer is the configuration checker's result record, `archogen-result/1`, as
//! [`archogen_plan_check::judgement::judge`] writes it — the record the executable and the MCP tool write for the same
//! texts, its producer [`PRODUCER`] and that digest. A request it cannot judge is answered [`REFUSAL_FORMAT`], and no
//! record: nothing was judged. Everything between the buffers and the judgement is [`answer`], a pure function the
//! host's tests call directly.
//!
//! ⛔ **No `unsafe` block.** Rust hands out the addresses of buffers it owns, which is a safe cast, and the page writes
//! and reads linear memory between calls, when no Rust borrow is live. The lint's only exceptions are the three
//! `#[no_mangle]` attributes, because the compiler counts an unmangled symbol as unsafe code. ⛔ **No imports**: the
//! module can reach nothing outside itself.

#![deny(unsafe_code)]

pub mod request;

use std::cell::RefCell;

use archogen_json::{self as json, Number, Value};
use archogen_outcome::record::Producer;
use archogen_outcome::Status;
use archogen_plan_check::judgement::{self, Inputs, Unjudged, PLAN_NAME};
use eadl_front::module::MemoryModules;

/// The request's format identifier, the first field of every request (the record's §11).
pub const REQUEST_FORMAT: &str = "archogen-plan-check-wasm-request/1";

/// The refusal's format identifier, the first key of the answer to a request the module cannot judge.
pub const REFUSAL_FORMAT: &str = "archogen-plan-check-wasm-refusal/1";

/// The producer every record the module writes names: a name of its own, so no reader takes a page's record for the
/// installed executable's, whose name the front requires.
pub const PRODUCER: &str = "archogen-plan-check-wasm";

/// The largest request [`plan_check_input`] allocates for: twice the checker's budget for the texts
/// ([`judgement::BYTES`]), the rest room for names and frames. The budget itself is counted on the texts alone.
pub const INPUT_CAP: usize = 2 * judgement::BYTES;

/// Answer one framed request: the whole module, without the buffers.
///
/// The result record for a request it judges; a refusal, `usage`, for one it cannot — a frame broken, a digest
/// misspelt, another profile, texts past the budget — and `tool-failure` where its own verdict makes no record, as the
/// executable refuses both.
#[must_use]
pub fn answer(bytes: &[u8]) -> String {
    let framed = match request::parse(bytes) {
        Ok(framed) => framed,
        Err(error) => return refusal(Status::Usage, &error.to_string()),
    };
    let texts = [framed.text.as_str(), framed.plan.as_str()]
        .into_iter()
        .chain(framed.modules.iter().map(|(_, text)| text.as_str()));
    if let Err(why) = judgement::budget(texts) {
        return refusal(Status::Usage, &why);
    }
    let profile = match judgement::profile(framed.profile.as_deref()) {
        Ok(profile) => profile,
        Err(why) => return refusal(Status::Usage, &why),
    };
    let modules = framed
        .modules
        .iter()
        .fold(MemoryModules::new(), |modules, (name, text)| {
            modules.with(name, text)
        });
    let producer = Producer {
        name: PRODUCER.to_owned(),
        sha256: framed.producer,
    };
    let inputs = Inputs {
        description_name: &framed.name,
        description: &framed.text,
        modules: &modules,
        profile,
        plan_name: PLAN_NAME,
        plan: framed.plan.as_bytes(),
    };
    match judgement::judge(producer, &inputs) {
        Ok(record) => record.write(),
        Err(Unjudged::TooLarge(error)) => refusal(
            Status::Usage,
            &format!("the description is too large to read: {error}"),
        ),
        Err(Unjudged::Record(error)) => refusal(
            Status::ToolFailure,
            &format!("its own result does not make a record: {error}"),
        ),
    }
}

/// A refusal: its format, its status and exit code, and what to change — one line, as a record is.
fn refusal(status: Status, note: &str) -> String {
    let mut text = json::write(&Value::Object(vec![
        (
            "format".to_owned(),
            Value::String(REFUSAL_FORMAT.to_owned()),
        ),
        ("status".to_owned(), Value::String(status.slug().to_owned())),
        (
            "exit".to_owned(),
            Value::Number(Number::from(i64::from(status.code()))),
        ),
        ("note".to_owned(), Value::String(note.to_owned())),
    ]));
    text.push('\n');
    text
}

thread_local! {
    /// The request, as the page wrote it.
    static INPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
    /// The last answer, until the next call of any export.
    static OUTPUT: RefCell<Vec<u8>> = const { RefCell::new(Vec::new()) };
}

/// Replace the input buffer with `len` zero bytes and return its address, where the page writes the request. When
/// `len` is above [`INPUT_CAP`], allocate nothing and return a null address.
#[allow(unsafe_code)] // the record's §3: `#[no_mangle]` is the lint's only exception
#[no_mangle]
pub extern "C" fn plan_check_input(len: usize) -> *mut u8 {
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

/// Answer the request in the input buffer, empty the input buffer, keep the answer in the output buffer, and return
/// the answer's length in bytes.
#[allow(unsafe_code)] // the record's §3: `#[no_mangle]` is the lint's only exception
#[no_mangle]
pub extern "C" fn plan_check() -> usize {
    let request = INPUT.with(|input| core::mem::take(&mut *input.borrow_mut()));
    let answer = answer(&request).into_bytes();
    let length = answer.len();
    OUTPUT.with(|output| *output.borrow_mut() = answer);
    length
}

/// The address of the last answer, valid until the next call of any export.
#[allow(unsafe_code)] // the record's §3: `#[no_mangle]` is the lint's only exception
#[no_mangle]
pub extern "C" fn plan_check_output() -> *const u8 {
    OUTPUT.with(|output| output.borrow().as_ptr())
}
