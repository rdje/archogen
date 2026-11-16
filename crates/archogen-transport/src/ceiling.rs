//! What one process of this tool may hold at once (`docs/decisions/decision_hosted-build.md` §5; leaf `M4.2.3.3.22`).
//!
//! Twenty-two rounds of review found shape after shape that costs more than its text: what compares pairs grows with
//! the pairs, and a request within its byte budget could ask for more than the machine has. A list of shapes is never
//! whole, so this counts what the process holds as it allocates it. Past the ceiling the run stops there, saying so,
//! under the status a resource limit has. An allocation cannot be unwound, so it stops the process and not the request.
//!
//! The ceiling is the instance's, as a byte budget is: it is read from the environment the program starts in, and
//! nothing a request says reaches it.

use std::alloc::{GlobalAlloc, Layout, System};
use std::io::Write;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

/// The variable that sets the ceiling, in MiB.
pub const VARIABLE: &str = "ARCHOGEN_MEMORY_MIB";

/// The ceiling with nothing set, in MiB.
pub const DEFAULT_MIB: usize = 2048;

/// `analysis-inconclusive`, what a resource limit is (`ROADMAP.md` §5.5). `crates/archogen-cli/tests/hosted_limits.rs`
/// holds that this is its code: a dependency for one number would put the vocabulary beneath the transport.
const INCONCLUSIVE: i32 = 14;

/// What stopping says, by each program. It is written whole, with nothing allocated. It says no more than it knows: what a run had
/// already come to stands where it was written, and what it was holding may be a file it was not given.
const SAID: &[u8] = b"archogen: analysis-inconclusive: this run reached its memory ceiling and stopped here, so it came to no conclusion\n  hint: raise ARCHOGEN_MEMORY_MIB, or give it less to read\n";
const SAID_BY_THE_CHECKER: &[u8] = b"archogen-plan-check: analysis-inconclusive: this run reached its memory ceiling and stopped here, so it came to no conclusion\n  hint: raise ARCHOGEN_MEMORY_MIB, or give it less to read\n";

static HELD: AtomicUsize = AtomicUsize::new(0);
static MOST: AtomicUsize = AtomicUsize::new(DEFAULT_MIB << 20);
static IS_THE_CHECKER: AtomicBool = AtomicBool::new(false);

/// The allocator: the system's, counted.
pub struct Ceiling;

/// Take the ceiling from the environment.
///
/// # Errors
///
/// What is wrong with the value, which is no whole number of MiB a process could hold.
pub fn from_environment(is_the_checker: bool) -> Result<(), String> {
    IS_THE_CHECKER.store(is_the_checker, Ordering::Relaxed);
    let Some(value) = std::env::var_os(VARIABLE) else {
        return Ok(());
    };
    let mib = value
        .to_str()
        .and_then(|text| text.parse::<usize>().ok())
        .filter(|mib| (1..=usize::MAX >> 20).contains(mib));
    let Some(mib) = mib else {
        return Err(format!(
            "{VARIABLE} is `{}`, and a ceiling is a whole number of MiB from 1 to {}",
            value.to_string_lossy().escape_debug(),
            usize::MAX >> 20
        ));
    };
    MOST.store(mib << 20, Ordering::Relaxed);
    Ok(())
}

/// Count `more`, stopping the process where that passes the ceiling.
fn holds(more: usize) {
    let now = HELD.fetch_add(more, Ordering::Relaxed).saturating_add(more);
    if now > MOST.load(Ordering::Relaxed) {
        // What stopping itself allocates is not stopped.
        MOST.store(usize::MAX, Ordering::Relaxed);
        let said = if IS_THE_CHECKER.load(Ordering::Relaxed) {
            SAID_BY_THE_CHECKER
        } else {
            SAID
        };
        let _ = std::io::stderr().write_all(said);
        std::process::exit(INCONCLUSIVE);
    }
}

// SAFETY: each call is `System`'s, with what it was given; this only counts, and stops the process before a call
// past the ceiling. It never unwinds: `holds` exits, and writes to standard error, which allocates nothing.
//
// A null answer gives nothing, and what was counted for it is given back, so a fallible allocation that fails holds
// no part of the ceiling (leaf `PROGRAM.150`): an infallible one ends the process there, and `try_reserve` goes on.
unsafe impl GlobalAlloc for Ceiling {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        holds(layout.size());
        // SAFETY: as above.
        let at = unsafe { System.alloc(layout) };
        if at.is_null() {
            HELD.fetch_sub(layout.size(), Ordering::Relaxed);
        }
        at
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        holds(layout.size());
        // SAFETY: as above.
        let at = unsafe { System.alloc_zeroed(layout) };
        if at.is_null() {
            HELD.fetch_sub(layout.size(), Ordering::Relaxed);
        }
        at
    }

    unsafe fn dealloc(&self, at: *mut u8, layout: Layout) {
        HELD.fetch_sub(layout.size(), Ordering::Relaxed);
        // SAFETY: as above.
        unsafe { System.dealloc(at, layout) };
    }

    unsafe fn realloc(&self, at: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        let old = layout.size();
        if new_size > old {
            holds(new_size - old);
        } else {
            HELD.fetch_sub(old - new_size, Ordering::Relaxed);
        }
        // SAFETY: as above.
        let moved = unsafe { System.realloc(at, layout, new_size) };
        if moved.is_null() {
            // The block stands where it was, at its old size.
            if new_size > old {
                HELD.fetch_sub(new_size - old, Ordering::Relaxed);
            } else {
                HELD.fetch_add(old - new_size, Ordering::Relaxed);
            }
        }
        moved
    }
}
