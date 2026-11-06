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
use std::sync::atomic::{AtomicUsize, Ordering};

/// The variable that sets the ceiling, in MiB.
pub const VARIABLE: &str = "ARCHOGEN_MEMORY_MIB";

/// The ceiling with nothing set, in MiB.
pub const DEFAULT_MIB: usize = 2048;

/// `analysis-inconclusive`, what a resource limit is (`ROADMAP.md` §5.5). `crates/archogen-cli/tests/hosted_limits.rs`
/// holds that this is its code: a dependency for one number would put the vocabulary beneath the transport.
const INCONCLUSIVE: i32 = 14;

/// What stopping says. It is written whole, with nothing allocated.
const SAID: &[u8] = b"archogen: analysis-inconclusive: this run would hold more memory than its ceiling, so it stopped, and nothing was judged\n  hint: shorten what it was given, or raise ARCHOGEN_MEMORY_MIB\n";

static HELD: AtomicUsize = AtomicUsize::new(0);
static MOST: AtomicUsize = AtomicUsize::new(DEFAULT_MIB << 20);

/// The allocator: the system's, counted.
pub struct Ceiling;

/// Take the ceiling from the environment.
///
/// # Errors
///
/// What is wrong with the value, which is no whole number of MiB a process could hold.
pub fn from_environment() -> Result<(), String> {
    let Some(value) = std::env::var_os(VARIABLE) else {
        return Ok(());
    };
    let mib = value
        .to_str()
        .and_then(|text| text.parse::<usize>().ok())
        .filter(|mib| (1..=usize::MAX >> 20).contains(mib));
    let Some(mib) = mib else {
        return Err(format!(
            "{VARIABLE} is `{}`, which is no whole number of MiB",
            value.to_string_lossy()
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
        let _ = std::io::stderr().write_all(SAID);
        std::process::exit(INCONCLUSIVE);
    }
}

// SAFETY: each call is `System`'s, with what it was given; this only counts, and stops the process before a call
// past the ceiling. It never unwinds: `holds` exits, and writes to standard error, which allocates nothing.
unsafe impl GlobalAlloc for Ceiling {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        holds(layout.size());
        // SAFETY: as above.
        unsafe { System.alloc(layout) }
    }

    unsafe fn alloc_zeroed(&self, layout: Layout) -> *mut u8 {
        holds(layout.size());
        // SAFETY: as above.
        unsafe { System.alloc_zeroed(layout) }
    }

    unsafe fn dealloc(&self, at: *mut u8, layout: Layout) {
        HELD.fetch_sub(layout.size(), Ordering::Relaxed);
        // SAFETY: as above.
        unsafe { System.dealloc(at, layout) };
    }

    unsafe fn realloc(&self, at: *mut u8, layout: Layout, new_size: usize) -> *mut u8 {
        if new_size > layout.size() {
            holds(new_size - layout.size());
        } else {
            HELD.fetch_sub(layout.size() - new_size, Ordering::Relaxed);
        }
        // SAFETY: as above.
        unsafe { System.realloc(at, layout, new_size) }
    }
}
