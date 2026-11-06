//! What a request costs, measured and not said (leaf `M4.2.3.3.21`). Four rounds of the hosted build's review found a shape a sentence of the
//! chapter was false of, so the shapes are here, each counted by what it allocates at its peak, and the sentence says this file holds them.
//! Every shape a reader found that cost more than in proportion to its text is a row, and a new one is a row before it is a sentence.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_api::{check, resolve_with, Limits, NoModules, Request, Resolve};

struct Counted;

static NOW: AtomicUsize = AtomicUsize::new(0);
static PEAK: AtomicUsize = AtomicUsize::new(0);

// SAFETY: each call is `System`'s, with what it was given; this only counts.
unsafe impl GlobalAlloc for Counted {
    unsafe fn alloc(&self, layout: Layout) -> *mut u8 {
        let now = NOW.fetch_add(layout.size(), Ordering::Relaxed) + layout.size();
        PEAK.fetch_max(now, Ordering::Relaxed);
        // SAFETY: as above.
        unsafe { System.alloc(layout) }
    }

    unsafe fn dealloc(&self, at: *mut u8, layout: Layout) {
        NOW.fetch_sub(layout.size(), Ordering::Relaxed);
        // SAFETY: as above.
        unsafe { System.dealloc(at, layout) };
    }
}

#[global_allocator]
static COUNTED: Counted = Counted;

/// What `run` holds at its most, above what stood when it began.
fn peak_of(run: impl FnOnce()) -> usize {
    let base = NOW.load(Ordering::Relaxed);
    PEAK.store(base, Ordering::Relaxed);
    run();
    PEAK.load(Ordering::Relaxed) - base
}

const PLATFORM: &str = "(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses blk)))\n";

/// Regions of one block at one base, which `check` accepts and a search compares by the pair.
fn regions(n: usize) -> String {
    let each: String = (0..n)
        .map(|i| format!("(region r{i} (base 0x1000) (size 4 KiB) (executable true))\n"))
        .collect();
    format!("(defblock blk (offers (reachable-at-privilege machine)\n{each}))\n{PLATFORM}(defsystem m (task t (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 1) (on-overrun fault)) (platform (uses soc)))\n")
}

/// A service needing many facts, in a name as long as a name is: what the closure says of each.
fn facts(n: usize) -> String {
    let name = "n".repeat(256);
    let each: String = (0..n).map(|i| format!("(needs f{i}) ")).collect();
    format!("(defservice {name} (requires {each}))\n")
}

/// Each shape a round of the review found, with how much a byte of it may cost beyond a floor. Held at this size, since a test must not itself endanger the host.
fn shapes() -> Vec<(&'static str, String)> {
    vec![
        ("junk that does not read", "\0".repeat(128 * 1024)),
        ("junk on one line", "x ".repeat(64 * 1024)),
        ("junk on many", "x\n".repeat(64 * 1024)),
        ("malformed numbers", "1.2.3 ".repeat(20_000)),
        (
            "a system of tasks",
            "(defsystem s (task t))\n".repeat(5_000),
        ),
        (
            "a fact stated both ways",
            format!(
                "(defblock b (offers {}) (absent {}))\n",
                "f ".repeat(1000),
                "f ".repeat(1000)
            ),
        ),
        (
            "one offer refined many ways",
            format!(
                "(defblock a (offers {}))\n{}",
                (0..1000)
                    .map(|i| format!("(o{i} true)"))
                    .collect::<Vec<_>>()
                    .join(" "),
                (0..1000)
                    .map(|i| format!("(defblock r{i} (refines a))\n"))
                    .collect::<String>()
            ),
        ),
        ("regions at one base", regions(1600)),
        ("a long name in each fact", facts(2000)),
    ]
}

#[test]
fn what_a_request_holds_is_within_what_the_chapter_says() {
    let mut said = Vec::new();
    for (shape, text) in shapes() {
        let request = Request {
            name: "shape.eadl",
            text: &text,
            profile: None,
            modules: &NoModules,
        };
        let checked = peak_of(|| {
            let answer = check(&request);
            std::hint::black_box(answer.render_diagnostics());
        });
        let resolved = peak_of(|| {
            let answer = resolve_with(
                &request,
                &Resolve {
                    system: None,
                    unchecked: true,
                },
                Limits::DEFAULT,
            );
            std::hint::black_box(answer.response.render_diagnostics());
        });
        // A floor for what any request holds, and so much for each byte of its text.
        let allowed = 64 * 1024 * 1024 + 400 * text.len();
        said.push(format!(
            "{shape}: {} bytes, check {} MB, resolve {} MB",
            text.len(),
            checked >> 20,
            resolved >> 20
        ));
        assert!(
            checked <= allowed,
            "{shape}: check holds {checked} of {allowed}"
        );
        assert!(
            resolved <= allowed,
            "{shape}: resolve holds {resolved} of {allowed}"
        );
    }
    println!("{}", said.join("\n"));
}
