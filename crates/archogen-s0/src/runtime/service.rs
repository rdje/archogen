//! The S0 realization of the `console.write` service.
//!
//! Emitted into the generated crate byte for byte, like its sibling `rt`. The description says
//! it needs `observable-output`; choosing *standard output on the hosted playground* to realize
//! that is an engine decision, and this file is where it lives.

use super::rt::Output;

/// The hosted-playground console.
pub struct Console;

impl Console {
    /// Open the console.
    #[must_use]
    pub const fn new() -> Self {
        Self
    }
}

impl Default for Console {
    fn default() -> Self {
        Self::new()
    }
}

impl Output for Console {
    // S0-ASSUMPTION: stdout-is-observable-output — `observable-output` is realized by writing to
    // standard output, with no device model behind it. `M4.3` supplies the modeled device.
    fn line(&mut self, text: &str) {
        println!("{text}");
    }
}
