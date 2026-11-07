//! What a request costs, measured and not said (leaf `M4.2.3.3.21`, and `M4.2.3.3.22`). Rounds of the hosted build's review found a shape a sentence of a
//! chapter was false of, so the shapes are here, each counted by what it allocates at its peak, and the chapter says this file holds them.
//! Every shape a reader found that cost more than in proportion to its text is a row, and a new one is a row before it is a sentence. A row states what its text
//! reaches, since a fixture a schema refuses costs nothing and holds nothing, and what compares pairs says how many, since that is what it grows with.
//! No list of shapes is whole: what a process may hold at all is `archogen_transport::ceiling`'s, and `crates/archogen-cli/tests/hosted_limits.rs` holds that.

use std::alloc::{GlobalAlloc, Layout, System};
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_api::{
    check, explain_with, resolve_with, Explain, Limits, MemoryModules, NoModules, Request, Resolve,
    Status,
};

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

const HEAD: &str = "(eadl-version eadl/1)\n";
const PLATFORM: &str = "(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses blk)))\n";
const A_TASK: &str = "(task t (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 1) (on-overrun fault))";
const A_TIMER: &str = "(offers (relative-delay true) (counter-width 16 bit) (counter-modulus 65536) (tick-rate 10 MHz) (wrap-behavior modular) (observation-coherent true) (channels 1 tick) (reachable-at-privilege machine))";

fn lines(n: usize, each: impl Fn(usize) -> String) -> String {
    (0..n).map(each).collect()
}

fn uses(prefix: &str, n: usize) -> String {
    (0..n).map(|i| format!("(uses {prefix}{i}) ")).collect()
}

/// Regions of one block at one base, which `check` accepts and a search compares by the pair.
fn regions(n: usize) -> String {
    let each = lines(n, |i| {
        format!("(region r{i} (base 0x1000) (size 4 KiB) (executable true))\n")
    });
    format!("{HEAD}(defblock blk (offers (reachable-at-privilege machine)\n{each}))\n{PLATFORM}(defsystem m {A_TASK} (platform (uses soc)))\n")
}

/// A service needing many facts, in a name as long as a name is, and a system asking for it: what the closure says of each.
fn facts(n: usize) -> String {
    let name = "n".repeat(256);
    let each = lines(n, |i| format!("(needs f{i}) "));
    let offered = lines(n, |i| format!("(f{i} true) "));
    format!("{HEAD}(defblock blk (offers (reachable-at-privilege machine) {offered}))\n{PLATFORM}(defservice {name} (requires {each}))\n(defsystem m (task t (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 1) (uses {name}) (on-overrun fault)) (requires (uses {name})) (platform (uses soc)))\n")
}

/// One system of many tasks.
fn tasks(n: usize) -> String {
    let each = lines(n, |i| {
        format!("(task t{i} (period 100 ms) (deadline 100 ms) (deadline-from release) (priority {}) (on-overrun fault))\n", i + 1)
    });
    format!("{HEAD}(defblock blk (offers (reachable-at-privilege machine)))\n{PLATFORM}(defsystem m\n{each}(platform (uses soc)))\n")
}

/// `n` offers of one platform, refined `m` ways.
fn refined(n: usize, m: usize) -> String {
    let offers = lines(n, |i| format!("(o{i} true) "));
    let ways = lines(m, |j| {
        format!("(defplatform r{j} (offers (x true)) (refines a))\n")
    });
    format!("{HEAD}(defplatform a (offers {offers}))\n{ways}")
}

/// Sizeless device regions on two bound providers, a finding per pair.
fn sizeless(n: usize) -> String {
    let a = lines(n, |i| format!("(region a{i} (base {i}))\n"));
    let b = lines(n, |i| format!("(region b{i} (base {}))\n", 1_000_000 + i));
    format!("{HEAD}(defblock pa (offers (relative-delay true) (channels 1 tick) (reachable-at-privilege machine)\n{a}))\n(defblock pb (offers (observable-output true) (reachable-at-privilege machine)\n{b}))\n(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses pa) (uses pb)))\n(defservice sa (requires (needs relative-delay)))\n(defservice sb (requires (needs observable-output)))\n(defsystem m (task t (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 1) (uses sa) (on-overrun fault)) (requires (uses sa) (uses sb)) (platform (uses soc)))\n")
}

/// Memory and reserved regions at one base under a long owner: each memory keeps each hole.
fn holes(n: usize) -> String {
    let owner = "b".repeat(256);
    let memory = lines(n, |i| {
        format!("(region r{i} (base 0x1000) (size 4 KiB) (executable true))\n")
    });
    let reserved = lines(n, |i| {
        format!("(region z{i} (base 0x1000) (size 4 KiB) (reserved true))\n")
    });
    format!("{HEAD}(defblock {owner} (offers (reachable-at-privilege machine)\n{memory}{reserved}))\n(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses {owner})))\n(defsystem m {A_TASK} (platform (uses soc)))\n")
}

/// Requesters of what an adapter derives, over many timers: each pair tries an instance.
fn through_an_adapter(r: usize, p: usize) -> String {
    let blocks = lines(p, |i| format!("(defblock b{i} {A_TIMER})\n"));
    let each_use = uses("b", p);
    let services = lines(r, |i| {
        format!("(defservice s{i} (requires (unambiguous-horizon (at-least 60 s)) (needs counter-width tick-rate observation-coherent)))\n")
    });
    let asked = uses("s", r);
    format!("{HEAD}{blocks}(defplatform plat (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires {each_use}))\n{services}(defsystem m (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses s0)) (requires {asked}) (platform (uses plat)))\n")
}

/// Requesters nothing meets, each refused by each provider.
fn unmet(r: usize, p: usize) -> String {
    let blocks = lines(p, |i| {
        format!("(defblock b{i} (offers (bus-width 8 bit) (reachable-at-privilege machine)))\n")
    });
    let each_use = uses("b", p);
    let services = lines(r, |i| {
        format!("(defservice s{i} (requires (needs bus-width) (bus-width 64 bit)))\n")
    });
    let asked = uses("s", r);
    format!("{HEAD}{blocks}(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires {each_use}))\n{services}(defsystem m (task t (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 1) (uses s0) (on-overrun fault)) (requires {asked}) (platform (uses soc)))\n")
}

/// Requesters many providers meet.
fn met(r: usize, p: usize) -> String {
    let blocks = lines(p, |i| {
        format!(
            "(defblock b{i} (offers (observable-output true) (reachable-at-privilege machine)))\n"
        )
    });
    let each_use = uses("b", p);
    let services = lines(r, |i| {
        format!("(defservice s{i} (requires (needs observable-output)))\n")
    });
    let asked = uses("s", r);
    format!("{HEAD}{blocks}(defplatform plat (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires {each_use}))\n{services}(defsystem m (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses s0) (on-overrun fault)) (requires {asked}) (platform (uses plat)))\n")
}

/// Tasks reaching services through one hub, each service a multiplexer of them all.
fn multiplexed(m: usize, n: usize) -> String {
    let services = lines(n, |i| {
        format!("(defservice s{i} (requires (needs relative-delay)))\n")
    });
    let each_use = uses("s", n);
    let each_task = lines(m, |i| {
        format!("(task t{i} (period 100 ms) (deadline 100 ms) (deadline-from release) (priority {}) (uses hub) (on-overrun fault))\n", i + 1)
    });
    format!("{HEAD}(defblock blk (offers (relative-delay true) (channels {} tick) (reachable-at-privilege machine)))\n{PLATFORM}{services}(defservice hub (requires {each_use}))\n(defsystem m\n{each_task}(requires (uses hub)) (platform (uses soc)))\n", n + 1)
}

/// A closure in a module tree as deep as one goes, with aliases as long as a tree's names can make them.
fn deep(facts: usize) -> (String, MemoryModules, usize) {
    let alias = "a".repeat(250);
    let each = lines(facts, |i| format!("(needs f{i}) "));
    let offered = lines(facts, |i| format!("f{i} "));
    let bottom = format!("{HEAD}(defmodule m15 (version 1 0) (export svc soc)\n (defservice svc (requires {each}))\n (defblock dev (offers {offered} (reachable-at-privilege machine)))\n (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses dev))))\n");
    let mut size = bottom.len();
    let mut modules = MemoryModules::new().with("m15", &bottom);
    for k in (1..15).rev() {
        let level = format!("{HEAD}(defmodule m{k} (version 1 0) (import m{} (as {alias}) (version (at-least 1 0))) (export svc soc)\n (defservice svc (requires (uses {alias}.svc)))\n (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses {alias}.soc))))\n", k + 1);
        size += level.len();
        modules = modules.with(&format!("m{k}"), &level);
    }
    (format!("{HEAD}(defmodule m0 (version 1 0) (import m1 (as {alias}) (version (at-least 1 0))) (export app)\n (defsystem app (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses {alias}.svc) (on-overrun fault)) (requires (uses {alias}.svc)) (platform (uses {alias}.soc))))\n"), modules, size)
}

/// One shape: its text, what its text reaches, and how much a byte and a pair of it may cost beyond the floor.
struct Row {
    shape: &'static str,
    /// The request's name for its text, which each answer quotes.
    name: String,
    text: String,
    modules: Option<MemoryModules>,
    modules_size: usize,
    checked: Status,
    resolved: Status,
    /// What `explain` is asked of, a fact the shape's requesters ask where it has any, and what that answers.
    fact: &'static str,
    explained: Status,
    /// What a byte of its text may cost, by what its shape was measured to.
    per_byte: usize,
    /// How many pairs it compares, and what each may cost; what grows with its text alone has none.
    pairs: usize,
    per_pair: usize,
}

fn row(shape: &'static str, text: String, checked: Status, resolved: Status) -> Row {
    Row {
        shape,
        name: "shape.eadl".to_owned(),
        text,
        modules: None,
        modules_size: 0,
        checked,
        resolved,
        fact: "observable-output",
        explained: resolved,
        per_byte: 100,
        pairs: 0,
        per_pair: 0,
    }
}

impl Row {
    fn pairs(mut self, pairs: usize, per_pair: usize) -> Self {
        self.pairs = pairs;
        self.per_pair = per_pair;
        self
    }

    fn asks(mut self, fact: &'static str, explained: Status) -> Self {
        self.fact = fact;
        self.explained = explained;
        self
    }

    fn per_byte(mut self, per_byte: usize) -> Self {
        self.per_byte = per_byte;
        self
    }

    fn named(mut self, name: String) -> Self {
        self.name = name;
        self
    }

    fn with(mut self, modules: MemoryModules, size: usize) -> Self {
        self.modules = Some(modules);
        self.modules_size = size;
        self
    }
}

/// Each shape a round of the review found. Held at this size, since a test must end, each allowed about what its largest command was measured to hold, its three commands sharing that allowance, so that the largest's cost doubling fails it, and another's only once it passes that. Each states what its text reaches, in all three commands.
fn rows() -> Vec<Row> {
    use Status::{InfeasibleConfiguration as Infeasible, InvalidDescription as Invalid, Ok};
    let junk = |shape, text| row(shape, text, Invalid, Invalid);
    let (deep_text, deep_modules, deep_size) = deep(500);
    vec![
        junk("junk that does not read", "\0".repeat(128 * 1024)).per_byte(30),
        junk("junk on one line", "x ".repeat(64 * 1024)),
        junk("junk on many", "x\n".repeat(64 * 1024)),
        junk("malformed numbers", "1.2.3 ".repeat(20_000)).per_byte(10),
        // No requirement to ask of: `explain` says so.
        row("a system of tasks", tasks(1000), Ok, Ok)
            .asks("observable-output", Status::Usage)
            .per_byte(30),
        junk(
            "a fact stated both ways",
            format!(
                "{HEAD}(defblock b (offers {}) (absent {}))\n",
                "f ".repeat(1000),
                "f ".repeat(1000)
            ),
        )
        .per_byte(300),
        row(
            "one offer refined many ways",
            refined(500, 500),
            Infeasible,
            Infeasible,
        )
        .per_byte(50),
        row("regions at one base", regions(800), Ok, Invalid).per_byte(30),
        // A fact the vocabulary does not declare is no binding's, so `explain` has nothing to say of it.
        row("a long name in each fact", facts(1000), Ok, Ok)
            .asks("f0", Status::Usage)
            .per_byte(50),
        row(
            "a long source's name, answered of each requester",
            met(500, 1),
            Ok,
            Ok,
        )
        .named("s".repeat(65_536))
        .per_byte(30),
        row("a closure in a deep module tree", deep_text, Ok, Ok)
            .with(deep_modules, deep_size)
            .asks("f0", Status::Usage)
            .per_byte(300),
        row(
            "sizeless regions on two devices",
            sizeless(150),
            Ok,
            Status::MissingFact,
        )
        .pairs(22_500, 800),
        row(
            "memory and reserved regions at one base",
            holes(300),
            Ok,
            Invalid,
        )
        .pairs(90_000, 320),
        row(
            "requesters over timers through an adapter",
            through_an_adapter(20, 20),
            Ok,
            Status::AnalysisInconclusive,
        )
        .asks("tick-rate", Status::AnalysisInconclusive)
        .pairs(400, 12 * 1024),
        row(
            "requesters each refused by each provider",
            unmet(100, 100),
            Ok,
            Infeasible,
        )
        .asks("bus-width", Infeasible)
        .per_byte(150),
        row(
            "requesters each met by each provider",
            met(100, 100),
            Ok,
            Ok,
        )
        .pairs(10_000, 350),
        row(
            "tasks through a hub of multiplexed services",
            multiplexed(100, 100),
            Ok,
            Ok,
        )
        .asks("relative-delay", Ok)
        .per_byte(30),
    ]
}

#[test]
fn what_a_request_holds_is_within_each_row_s_allowance() {
    let mut said = Vec::new();
    let mut over = Vec::new();
    for row in rows() {
        let no_modules = NoModules;
        let request = Request {
            name: &row.name,
            text: &row.text,
            profile: None,
            modules: row.modules.as_ref().map_or(&no_modules, |modules| modules),
        };
        let began = std::time::Instant::now();
        let mut status = Status::Ok;
        let checked = peak_of(|| {
            let answer = check(&request);
            status = answer.status;
            std::hint::black_box(answer.render_diagnostics());
        });
        let check_reached = status;
        let resolved = peak_of(|| {
            let answer = resolve_with(
                &request,
                &Resolve {
                    system: None,
                    unchecked: true,
                },
                Limits::DEFAULT,
            );
            status = answer.response.status;
            std::hint::black_box(answer.response.render_diagnostics());
        });
        let resolve_reached = status;
        let explained = peak_of(|| {
            let answer = explain_with(
                &request,
                &Explain {
                    system: None,
                    fact: row.fact,
                    requester: None,
                    task: None,
                },
                Limits::DEFAULT,
            );
            status = answer.response.status;
            std::hint::black_box(answer.response.render_diagnostics());
            std::hint::black_box(answer.accounts.len());
        });
        let explain_reached = status;
        // A floor for what any request holds, so much for each byte of its text and of its name, and so much for each pair it compares.
        let allowed = 2 * 1024 * 1024
            + row.per_byte * (row.text.len() + row.name.len() + row.modules_size)
            + row.per_pair * row.pairs;
        said.push(format!(
            "{:.1} s, {}: {} bytes, {} pairs, allowed {} KB: check {} KB `{}`, resolve {} KB `{}`, explain {} KB `{}`",
            began.elapsed().as_secs_f64(),
            row.shape,
            row.text.len(),
            row.pairs,
            allowed >> 10,
            checked >> 10,
            check_reached.slug(),
            resolved >> 10,
            resolve_reached.slug(),
            explained >> 10,
            explain_reached.slug(),
        ));
        if check_reached != row.checked
            || resolve_reached != row.resolved
            || explain_reached != row.explained
        {
            over.push(format!(
                "{}: reaches `{}`, `{}` and `{}`, and the row says `{}`, `{}` and `{}`",
                row.shape,
                check_reached.slug(),
                resolve_reached.slug(),
                explain_reached.slug(),
                row.checked.slug(),
                row.resolved.slug(),
                row.explained.slug()
            ));
        }
        for (what, held) in [
            ("check", checked),
            ("resolve", resolved),
            ("explain", explained),
        ] {
            if held > allowed {
                over.push(format!("{}: {what} holds {held} of {allowed}", row.shape));
            }
        }
    }
    println!("{}", said.join("\n"));
    assert!(over.is_empty(), "{}", over.join("\n"));
}
