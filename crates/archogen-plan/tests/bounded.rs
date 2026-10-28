//! The runtime plan's bounded exhaustive checker (leaf `M4.1.13`), which
//! `docs/decisions/decision_executable-design-reviews.md` item 1 asks of a design whose subject is a decision
//! procedure: it "enumerates a finite universe of inputs" and holds each outcome to the record. Thirteen universes of
//! small descriptions are generated in memory, every combination of a few independent choices, so that each is
//! exhaustive over its bounds: three services and their `uses` edges; two services with adapter instances; releases and
//! routes of every separation class; requesters of every kind; two tasks, periodic and sporadic, sharing services that
//! take a channel, with an obliged `delivery-bound` bound and bare and each task writing `(deadline-from release)` or
//! not; receivers of interrupt code, two tasks and the system's own level each programming an alarm itself, receiving
//! an expiry or neither, in both runtime orders; after round 12 (R12-R4), blocks and platforms with requirements of
//! their own, bound to by something or by nothing; every jitter, overrun policy and deadline reference a task line
//! states; durations past the exact arithmetic, each kind named in §1's order, and durations at its edge, written
//! exactly however they are spelled; and a second system, named by `uses` or `needs` or not, each lowered; and, after
//! round 13 (R13-R1 to R13-R3), blocks and platforms whose `uses` or `needs` name a service, a policy, a system or
//! another block, alone, several at once or beside each other refusal of §1 that can co-apply, on the lowered
//! system's hardware or on another's; a block a service names, served though not the system's hardware, bound for
//! requirements of its own; and requesters naming several systems, held to the naming-system refusal's order.
//!
//! Each system is run as the engine runs it, as `common::lowered` does: the language's check, the search, the `/2`
//! writer, and the lowering. **Every `archogen-assignment/2` record the search writes is judged by the configuration
//! checker**, `archogen_plan_check::check::plan_file`, as `crates/eadl-resolve/tests/plan.rs` judges one, and a record
//! it refuses is a finding naming that record. Each plan is held to an **independent reading** of
//! `docs/decisions/decision_runtime-plan.md`, written below as plain, unoptimised Rust from the record's text alone,
//! each rule citing the section it transcribes.
//!
//! ⭐ **The assignment is input.** The reading takes the description and the record's own `bind` lines — what the
//! search chose, never something predicted — and derives from them every line of the plan: §2.1's identity, the
//! record's own and its sha256; §2.2's task lines, their index, rank, durations, deadline reference, trigger and
//! `(uses …)` item; §2.3's `bind` lines, the record's copied; §2.4's instance lines, an obliged fact with its
//! requester's bound or bare; the release timer's line, each route with its separation and rank, the enabled set and
//! the interrupt model (§2.5); §2.6's `(context …)` lines; §2.7's `(depends …)` and `(step …)` lines, from model §8
//! rule 12's graph, a component of several members one step; §2.9's obligations, `multiplex` among them; §2.10's owed
//! facts; and the last line, `(complete no)`. **Every line of the plan is compared, in its place**: the plan's whole
//! text is held to the reading's, the forms in §3's order across kinds as well as within one, and a line of a head §3
//! does not write is a disagreement of its own, never dropped. It derives which of §1's refusals is due, where these
//! universes reach one, and holds the lowering's refusal to its code, to the subjects it names — each told by its exact
//! text, a task's `(task SYSTEM NAME)` form or the system's own level's name as a word of its own, never by a
//! substring — in §1's order, and to what each would receive. The reading is itself held to the record's three worked
//! texts (§3), whole.
//!
//! ⛔ **Independence.** The reading was written by contexts that never read `crates/archogen-plan/src/lib.rs` nor the
//! corpus's outcomes. A disagreement is a finding, never something to make disappear: decide from the record's text
//! which side is right, and keep a reading the record supports failing on its system.
//!
//! Each universe's size is asserted — the systems generated and how each fared, lowered or not — with every system the
//! search resolves judged by the configuration checker, and each class of the record's rules it is built to reach, so
//! that a generator cannot shrink unnoticed; a system the language or the search refuses is counted, never dropped.

mod common;

use std::collections::{BTreeMap, BTreeSet};

use archogen_digest::Digest;
use archogen_plan::{lower, Refusal};
use archogen_plan_check::check::plan_file;
use eadl_front::module::DirectoryModules;
use eadl_front::{Form, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::adapter::Offered;
use eadl_resolve::plan;
use eadl_resolve::search::{search, Limits, Policy, Search};
use eadl_resolve::vocabulary::Usage;

// =====================================================================================================================
// The descriptions, generated in memory
// =====================================================================================================================

/// A duration as a description spells it: a whole number of milliseconds, or a spelling written as given.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Dur {
    Ms(u64),
    Spelled(&'static str),
}

/// A time of `1e-47 s`, whose denominator lies past signed 128-bit integers (§2.2), which the language still reads:
/// it reads a decimal of at most 38 fraction digits whose digits make at most `i64::MAX`.
const PAST: &str = "0.00000000000000000000000000000000000001 ns";
/// `9223372036854775807/10^38 s`: within the arithmetic, its digits the most the language reads, and compared with
/// [`WIDE_LONGER`] by cross-multiplication past 128 bits.
const WIDE: &str = "0.00000000000000000009223372036854775807 s";
/// [`WIDE`] in milliseconds: the same time, spelled otherwise.
const WIDE_IN_MS: &str = "0.00000000000000009223372036854775807 ms";
/// `922337203685477581/10^37 s`, a little longer than [`WIDE`].
const WIDE_LONGER: &str = "0.0000000000000000000922337203685477581 s";
/// `1/10^38 s`, spelled with a denominator of `10^47` that only its reduction brings within the arithmetic.
const REDUCED: &str = "0.00000000000000000000000000001000000000 ns";
/// `5^13/10^47 s = 1/(8192·10^34) s`: its reduced denominator, `8.192e37`, the nearest below `i128::MAX` a spelling
/// of the language reaches.
const TIGHT_WITHIN: &str = "0.00000000000000000000000000001220703125 ns";
/// `5^12/10^47 s = 1/(4096·10^35) s`: its reduced denominator, `4.096e38`, the nearest above `i128::MAX`.
const TIGHT_PAST: &str = "0.00000000000000000000000000000244140625 ns";

impl Dur {
    fn spelling(self) -> String {
        match self {
            Self::Ms(ms) => format!("{ms} ms"),
            Self::Spelled(text) => text.to_string(),
        }
    }

    /// Its seconds (§2.2), or `None` where they lie past the engine's exact arithmetic.
    fn seconds(self) -> Option<Ratio> {
        Ratio::of(&self.spelling())
    }
}

/// A reduced non-negative rational number of seconds, its numerator and denominator within signed 128-bit integers
/// (§2.2: "`N` and `D` lie within the engine's exact arithmetic, signed 128-bit integers").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Ratio {
    n: u128,
    d: u128,
}

/// `a * b`, exactly, as its high and low 128 bits.
fn wide_product(a: u128, b: u128) -> (u128, u128) {
    const LOW: u128 = u64::MAX as u128;
    let (a1, a0) = (a >> 64, a & LOW);
    let (b1, b0) = (b >> 64, b & LOW);
    let (p00, p01, p10, p11) = (a0 * b0, a0 * b1, a1 * b0, a1 * b1);
    let middle = (p00 >> 64) + (p01 & LOW) + (p10 & LOW);
    let low = (p00 & LOW) | (middle << 64);
    let high = p11 + (p01 >> 64) + (p10 >> 64) + (middle >> 64);
    (high, low)
}

impl Ord for Ratio {
    /// Exact: `n1/d1` against `n2/d2` by `n1·d2` against `n2·d1`, each product in 256 bits.
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        wide_product(self.n, other.d).cmp(&wide_product(other.n, self.d))
    }
}

impl PartialOrd for Ratio {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ratio {
    const ZERO: Self = Self { n: 0, d: 1 };

    /// The seconds `NUMBER UNIT` spells, reduced, or `None` where the reduced numerator or denominator lies past
    /// `i128::MAX`.
    fn of(spelling: &str) -> Option<Self> {
        let (number, unit) = spelling.split_once(' ').expect("NUMBER UNIT");
        let exponent: u32 = match unit {
            "s" => 0,
            "ms" => 3,
            "us" => 6,
            "ns" => 9,
            other => panic!("the reading knows no unit of time `{other}`"),
        };
        let (whole, fraction) = number.split_once('.').unwrap_or((number, ""));
        let mut n: u128 = format!("{whole}{fraction}").parse().expect("digits");
        if n == 0 {
            return Some(Self::ZERO);
        }
        // The denominator is `10^k = 2^k 5^k`: reduce by every 2 and 5 the numerator shares.
        let k = u32::try_from(fraction.len()).expect("short") + exponent;
        let (mut twos, mut fives) = (k, k);
        while twos > 0 && n.is_multiple_of(2) {
            n /= 2;
            twos -= 1;
        }
        while fives > 0 && n.is_multiple_of(5) {
            n /= 5;
            fives -= 1;
        }
        let d = 2u128
            .checked_pow(twos)?
            .checked_mul(5u128.checked_pow(fives)?)?;
        let max = u128::try_from(i128::MAX).expect("positive");
        (n <= max && d <= max).then_some(Self { n, d })
    }

    /// §2.2: "Each is one reduced rational number of seconds, `N s` or `N/D s`, with `D > 1`".
    fn text(self) -> String {
        if self.d == 1 {
            format!("{} s", self.n)
        } else {
            format!("{}/{} s", self.n, self.d)
        }
    }
}

/// What a source's block states of its own `min-arrival-separation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Separation {
    /// Stated, in milliseconds.
    Stated(u64),
    /// Not named at all.
    Unstated,
    /// Declared absent.
    Absent,
    /// Stated once in each of these spellings.
    Spelled(&'static [&'static str]),
}

/// What the reading takes of a source's separation (§2.5, §1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Sep {
    Stated(Ratio),
    Unstated,
    Absent,
    /// A spelling lies past the exact arithmetic: one alone, or one of two spellings, whose comparison then overflows
    /// it (round 12, R12-D2: "a source separation whose spellings overflow the arithmetic" is "Refused past the
    /// arithmetic, as §1 says").
    Past,
}

impl Separation {
    fn read(self) -> Sep {
        match self {
            Self::Stated(ms) => Sep::Stated(Dur::Ms(ms).seconds().expect("a few milliseconds")),
            Self::Unstated => Sep::Unstated,
            Self::Absent => Sep::Absent,
            Self::Spelled(spellings) => {
                let read: Option<Vec<Ratio>> = spellings.iter().map(|s| Ratio::of(s)).collect();
                match read {
                    None => Sep::Past,
                    Some(times) => {
                        assert!(
                            times.windows(2).all(|w| w[0] == w[1]),
                            "the universes spell one separation, never two"
                        );
                        Sep::Stated(times[0])
                    }
                }
            }
        }
    }
}

/// A block, or a platform: its offers, as written, what it requires at its own level, and what the reading needs of
/// it. Its `uses` is composition, never a binding (model §8 rule 1).
#[derive(Debug, Clone, Default)]
struct Block {
    name: String,
    offers: Vec<String>,
    absent: Vec<String>,
    /// For a block offering `interrupt-source`: what it states of its separation.
    separation: Option<Separation>,
    /// Its `requires` clause: what it needs, and what it composes by `uses`.
    side: Side,
}

impl Block {
    /// Its form, `defblock` or `defplatform`.
    fn text(&self, head: &str) -> String {
        let mut out = format!("({head} {} (offers {})", self.name, self.offers.join(" "));
        if !self.absent.is_empty() {
            out.push_str(&format!(" (absent {})", self.absent.join(" ")));
        }
        let requires = self.side.clauses();
        if !requires.is_empty() {
            out.push_str(&format!(" (requires{requires})"));
        }
        out.push_str(")\n");
        out
    }
}

/// What one requester writes: the vocabulary facts its `needs` lists, the declarations its `uses` lists, each
/// `(FACT (at-most DURATION))` bound it writes, and any other requirement clause, as written.
#[derive(Debug, Clone, Default)]
struct Side {
    needs: Vec<String>,
    uses: Vec<String>,
    /// Each `(FACT (at-most DURATION))` the requester writes.
    at_most: Vec<(String, Dur)>,
    other: Vec<String>,
}

impl Side {
    /// The side's clauses, each preceded by a space.
    fn clauses(&self) -> String {
        let mut out = String::new();
        for clause in &self.other {
            out.push(' ');
            out.push_str(clause);
        }
        for (fact, bound) in &self.at_most {
            out.push_str(&format!(" ({fact} (at-most {}))", bound.spelling()));
        }
        if !self.needs.is_empty() {
            out.push_str(&format!(" (needs {})", self.needs.join(" ")));
        }
        if !self.uses.is_empty() {
            out.push_str(&format!(" (uses {})", self.uses.join(" ")));
        }
        out
    }
}

/// A service, or a policy (`defpolicy`), which §2.9 holds alike.
#[derive(Debug, Clone)]
struct Service {
    name: String,
    policy: bool,
    side: Side,
}

/// A task's release model.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Release {
    Period(Dur),
    MinSeparation(Dur),
}

impl Release {
    fn duration(self) -> Dur {
        match self {
            Self::Period(d) | Self::MinSeparation(d) => d,
        }
    }

    /// Its clause's name, which §1 names a duration past the arithmetic by.
    fn clause(self) -> &'static str {
        match self {
            Self::Period(_) => "period",
            Self::MinSeparation(_) => "min-separation",
        }
    }
}

#[derive(Debug, Clone)]
struct Task {
    name: String,
    release: Release,
    /// Its deadline, where it is not its release's duration.
    deadline: Option<Dur>,
    /// Its `jitter` clause, where it writes one.
    jitter: Option<Dur>,
    /// Its `on-overrun` clause, where it writes one.
    overrun: Option<&'static str>,
    priority: u32,
    /// The event its `deadline-from` clause names, where it writes one.
    deadline_from: Option<&'static str>,
    side: Side,
}

impl Default for Task {
    fn default() -> Self {
        Self {
            name: "t".to_string(),
            release: Release::Period(Dur::Ms(10)),
            deadline: None,
            jitter: None,
            overrun: None,
            priority: 1,
            deadline_from: Some("release"),
            side: Side::default(),
        }
    }
}

impl Task {
    fn deadline(&self) -> Dur {
        self.deadline.unwrap_or(self.release.duration())
    }

    fn text(&self) -> String {
        format!(
            "  (task {} ({} {}) (deadline {}){}{}{} (priority {}){})\n",
            self.name,
            self.release.clause(),
            self.release.duration().spelling(),
            self.deadline().spelling(),
            self.jitter
                .map_or(String::new(), |j| format!(" (jitter {})", j.spelling())),
            self.deadline_from
                .map_or(String::new(), |e| format!(" (deadline-from {e})")),
            self.overrun
                .map_or(String::new(), |p| format!(" (on-overrun {p})")),
            self.priority,
            self.side.clauses()
        )
    }
}

/// A system of a description: its tasks, its own level and the platform it uses.
#[derive(Debug, Clone)]
struct System {
    name: String,
    tasks: Vec<Task>,
    own: Side,
    platform: String,
}

impl System {
    fn text(&self) -> String {
        let mut out = format!("(defsystem {}\n", self.name);
        for task in &self.tasks {
            out.push_str(&task.text());
        }
        let own = self.own.clauses();
        if !own.is_empty() {
            out.push_str(&format!("  (requires{own})\n"));
        }
        out.push_str(&format!("  (platform (uses {})))\n", self.platform));
        out
    }
}

/// One description and the system of it lowered: by default on one platform `soc` that uses every block.
#[derive(Debug, Clone)]
struct Description {
    system: String,
    blocks: Vec<Block>,
    /// Its platforms; none writes the one `soc` composing every block.
    platforms: Vec<Block>,
    services: Vec<Service>,
    tasks: Vec<Task>,
    /// The system's own level.
    own: Side,
    /// The platform the system's `platform` clause uses.
    platform: String,
    /// The description's other systems, written after the one lowered, or before it where `others_first`.
    others: Vec<System>,
    others_first: bool,
}

impl Default for Description {
    fn default() -> Self {
        Self {
            system: "sys".to_string(),
            blocks: Vec::new(),
            platforms: Vec::new(),
            services: Vec::new(),
            tasks: Vec::new(),
            own: Side::default(),
            platform: "soc".to_string(),
            others: Vec::new(),
            others_first: false,
        }
    }
}

impl Description {
    /// Its platforms: those written, or the one `soc` composing every block.
    fn platforms(&self) -> Vec<Block> {
        if !self.platforms.is_empty() {
            return self.platforms.clone();
        }
        vec![Block {
            name: "soc".to_string(),
            offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
            side: Side {
                uses: self.blocks.iter().map(|b| b.name.clone()).collect(),
                ..Side::default()
            },
            ..Block::default()
        }]
    }

    /// Every block and platform it declares.
    fn hardware(&self) -> Vec<Block> {
        self.blocks
            .iter()
            .cloned()
            .chain(self.platforms())
            .collect()
    }

    /// Every system it declares, the one lowered first.
    fn systems(&self) -> Vec<System> {
        std::iter::once(self.lowered())
            .chain(self.others.iter().cloned())
            .collect()
    }

    /// The system lowered, as a [`System`].
    fn lowered(&self) -> System {
        System {
            name: self.system.clone(),
            tasks: self.tasks.clone(),
            own: self.own.clone(),
            platform: self.platform.clone(),
        }
    }

    /// The description's text, as the language reads it.
    fn text(&self) -> String {
        let mut out = String::from("(eadl-version eadl/1)\n");
        for block in &self.blocks {
            out.push_str(&block.text("defblock"));
        }
        for platform in self.platforms() {
            out.push_str(&platform.text("defplatform"));
        }
        for service in &self.services {
            // A service writes its `requires` clause, empty where it requires nothing: the language asks for one.
            out.push_str(&format!(
                "({} {} (requires{}))\n",
                if service.policy {
                    "defpolicy"
                } else {
                    "defservice"
                },
                service.name,
                service.side.clauses()
            ));
        }
        if self.others_first {
            for other in &self.others {
                out.push_str(&other.text());
            }
        }
        out.push_str(&self.lowered().text());
        if !self.others_first {
            for other in &self.others {
                out.push_str(&other.text());
            }
        }
        out
    }
}

/// One generated system: what it varies, and its description.
#[derive(Debug, Clone)]
struct Case {
    label: String,
    description: Description,
}

fn names(items: &[&str]) -> Vec<String> {
    items.iter().map(|s| (*s).to_string()).collect()
}

/// A source's block: `interrupt-source`, its separation as `separation` says, and the `available-in-state` it offers.
fn source_block(name: &str, separation: Separation, states: &str) -> Block {
    let mut offers = names(&["(interrupt-source true)"]);
    match separation {
        Separation::Stated(ms) => offers.push(format!("(min-arrival-separation {ms} ms)")),
        Separation::Spelled(spellings) => {
            for spelling in spellings {
                offers.push(format!("(min-arrival-separation {spelling})"));
            }
        }
        Separation::Unstated | Separation::Absent => {}
    }
    if !states.is_empty() {
        offers.push(format!("(available-in-state {states})"));
    }
    offers.push("(reachable-at-privilege machine)".to_string());
    Block {
        name: name.to_string(),
        offers,
        absent: if separation == Separation::Absent {
            names(&["min-arrival-separation"])
        } else {
            Vec::new()
        },
        separation: Some(separation),
        side: Side::default(),
    }
}

/// One block raising both kinds of interrupt (§2: "one provider can raise a timer route and a source"): a timer with
/// what both adapter kinds need beneath, and a source stating 20 ms.
fn timer_and_source_block(name: &str, channels: u32) -> Block {
    let mut block = timer_block(name, channels, true);
    let at = block.offers.len() - 1;
    block
        .offers
        .insert(at, "(min-arrival-separation 20 ms)".to_string());
    block
        .offers
        .insert(at, "(interrupt-source true)".to_string());
    block.separation = Some(Separation::Stated(20));
    block
}

/// A timer: a block offering `relative-delay` with `channels` channels and, for `adapters`, what both adapter kinds
/// need beneath (`docs/semantics/adapters/adapters.eadl`) — but `counter-width`, so that an epoch extender's requester
/// is met by no block.
fn timer_block(name: &str, channels: u32, adapters: bool) -> Block {
    let mut offers = names(&["(relative-delay true)"]);
    if adapters {
        offers.extend(names(&[
            "(counter-modulus 65536)",
            "(tick-rate 10 MHz)",
            "(wrap-behavior modular)",
            "(observation-coherent true)",
        ]));
    }
    offers.push(format!("(channels {channels} tick)"));
    offers.push("(reachable-at-privilege machine)".to_string());
    Block {
        name: name.to_string(),
        offers,
        ..Block::default()
    }
}

/// Every declaration `from` reaches through `uses` and `needs` among `services`, at any depth.
fn uses_closure(services: &[Service], from: &[String]) -> BTreeSet<String> {
    let mut reached = BTreeSet::new();
    let mut todo: Vec<String> = from.to_vec();
    while let Some(at) = todo.pop() {
        if reached.insert(at.clone()) {
            if let Some(service) = services.iter().find(|s| s.name == at) {
                todo.extend(service.side.uses.iter().cloned());
                todo.extend(service.side.needs.iter().cloned());
            }
        }
    }
    reached
}

/// The system's own level reaches, by `uses` or by `needs`, each service no task reaches, so that every service is
/// served.
fn own_level(services: &[Service], tasks: &[Task], by_needs: bool) -> Side {
    let from: Vec<String> = tasks
        .iter()
        .flat_map(|t| t.side.uses.iter().chain(&t.side.needs).cloned())
        .collect();
    let reached = uses_closure(services, &from);
    let unreached = services
        .iter()
        .map(|s| s.name.clone())
        .filter(|s| !reached.contains(s))
        .collect();
    named(Side::default(), unreached, by_needs)
}

/// `side` naming `declarations` too, by `needs` or by `uses`.
fn named(mut side: Side, declarations: Vec<String>, by_needs: bool) -> Side {
    if by_needs {
        side.needs.extend(declarations);
    } else {
        side.uses.extend(declarations);
    }
    side
}

/// What a service of the services universes takes of a counter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Counter {
    /// Nothing.
    None,
    /// A channel of a timer: `(needs relative-delay)`.
    Timer,
    /// An absolute deadline, which only a `deadline-over-delay` instance offers: `(needs absolute-deadline)`, its
    /// obliged `delivery-bound` left bare.
    Deadline,
    /// The same, writing `(delivery-bound (at-most 5 ms))`, the bound the instance then takes on (§2.4).
    DeadlineBound,
    /// A counter with a horizon no block offers, which only an `epoch-extender` instance meets.
    Extender,
}

/// The bound a [`Counter::DeadlineBound`] requester writes on `delivery-bound`, in milliseconds.
const DELIVERY_BOUND_MS: u64 = 5;

impl Counter {
    fn word(self) -> &'static str {
        match self {
            Self::None => "-",
            Self::Timer => "timer",
            Self::Deadline => "deadline",
            Self::DeadlineBound => "deadline<=5ms",
            Self::Extender => "extender",
        }
    }
}

fn service(name: &str, counter: Counter, source: bool, uses: Vec<String>) -> Service {
    let mut side = Side {
        uses,
        ..Side::default()
    };
    match counter {
        Counter::None => {}
        Counter::Timer => side.needs.push("relative-delay".to_string()),
        Counter::Deadline => side.needs.push("absolute-deadline".to_string()),
        Counter::DeadlineBound => {
            side.needs.push("absolute-deadline".to_string());
            side.at_most
                .push(("delivery-bound".to_string(), Dur::Ms(DELIVERY_BOUND_MS)));
        }
        Counter::Extender => {
            side.other
                .push("(unambiguous-horizon (at-least 60 s))".to_string());
            side.needs.extend(names(&[
                "counter-width",
                "tick-rate",
                "observation-coherent",
            ]));
        }
    }
    if source {
        side.needs.push("interrupt-source".to_string());
    }
    Service {
        name: name.to_string(),
        policy: false,
        side,
    }
}

/// Every ordered pair of distinct indices below `n`: the `uses` edges possible among `n` services.
fn pairs(n: usize) -> Vec<(usize, usize)> {
    let mut out = Vec::new();
    for i in 0..n {
        for j in 0..n {
            if i != j {
                out.push((i, j));
            }
        }
    }
    out
}

/// **Services and their callers** (§2.6, §2.9), three services: each takes a channel of the one timer, handles the
/// one source, both or neither; every set of `uses` edges among them, cycles included; a periodic task using every
/// subset of them; the system's own level using each service the task does not reach.
fn three_services() -> Vec<Case> {
    const SERVICES: [&str; 3] = ["a", "b", "c"];
    let edges = pairs(3);
    let mut cases = Vec::new();
    for capabilities in 0..4usize.pow(3) {
        for edge_set in 0..1usize << edges.len() {
            for task_set in 0..1usize << SERVICES.len() {
                let mut services = Vec::new();
                let mut label = String::from("caps=");
                for (i, name) in SERVICES.iter().enumerate() {
                    let cap = capabilities / 4usize.pow(u32::try_from(i).expect("small")) % 4;
                    let counter = if cap & 1 == 1 {
                        Counter::Timer
                    } else {
                        Counter::None
                    };
                    let source = cap & 2 == 2;
                    let uses = edges
                        .iter()
                        .enumerate()
                        .filter(|(k, (from, _))| *from == i && edge_set >> k & 1 == 1)
                        .map(|(_, (_, to))| SERVICES[*to].to_string())
                        .collect();
                    label.push_str(&format!(
                        "{name}:{}{} ",
                        counter.word(),
                        if source { "+source" } else { "" }
                    ));
                    services.push(service(name, counter, source, uses));
                }
                let task_uses: Vec<String> = SERVICES
                    .iter()
                    .enumerate()
                    .filter(|(i, _)| task_set >> i & 1 == 1)
                    .map(|(_, s)| (*s).to_string())
                    .collect();
                let tasks = vec![Task {
                    name: "t".to_string(),
                    release: Release::Period(Dur::Ms(10)),
                    priority: 1,
                    deadline_from: Some("release"),
                    side: Side {
                        uses: task_uses.clone(),
                        ..Side::default()
                    },
                    ..Task::default()
                }];
                let edge_words: Vec<String> = edges
                    .iter()
                    .enumerate()
                    .filter(|(k, _)| edge_set >> k & 1 == 1)
                    .map(|(_, (from, to))| format!("{}>{}", SERVICES[*from], SERVICES[*to]))
                    .collect();
                label.push_str(&format!(
                    "uses=[{}] t=[{}]",
                    edge_words.join(","),
                    task_uses.join(",")
                ));
                let own = own_level(&services, &tasks, false);
                cases.push(Case {
                    label,
                    description: Description {
                        system: "sys".to_string(),
                        blocks: vec![
                            timer_block("timer.d", 3, false),
                            source_block("dev.s", Separation::Stated(20), ""),
                        ],
                        services,
                        tasks,
                        own,
                        ..Description::default()
                    },
                });
            }
        }
    }
    cases
}

/// The hardware of the adapters universe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Shape {
    /// One timer with three channels, and a source apart.
    OneTimer,
    /// Two timers with one channel each, and a source apart.
    TwoTimers,
    /// One block that is both the timer, with three channels, and the source.
    Both,
}

/// **Services with adapter instances** (§2.6, §2.9, §2.4), two declarations `a` and `b`, `b` a service or a policy:
/// each takes a channel of a timer, a `deadline-over-delay` instance's channel or an `epoch-extender` instance's
/// counter, or none, and handles the one source or not; every set of edges between them; a periodic task naming every
/// subset of them; every name written by `uses`, or every one by `needs`; one timer and a source, two timers and a
/// source, or one block raising both.
fn two_services_with_adapters() -> Vec<Case> {
    const SERVICES: [&str; 2] = ["a", "b"];
    const COUNTERS: [Counter; 4] = [
        Counter::None,
        Counter::Timer,
        Counter::Deadline,
        Counter::Extender,
    ];
    let edges = pairs(2);
    let subset = |set: usize| -> Vec<String> {
        SERVICES
            .iter()
            .enumerate()
            .filter(|(i, _)| set >> i & 1 == 1)
            .map(|(_, s)| (*s).to_string())
            .collect()
    };
    let mut cases = Vec::new();
    for shape in [Shape::OneTimer, Shape::TwoTimers, Shape::Both] {
        for b_policy in [false, true] {
            for by_needs in [false, true] {
                for capabilities in 0..8usize.pow(2) {
                    for edge_set in 0..1usize << edges.len() {
                        for t_set in 0..1usize << SERVICES.len() {
                            let mut services = Vec::new();
                            let mut label = format!(
                                "shape={shape:?} b={} naming={} caps=",
                                if b_policy { "policy" } else { "service" },
                                if by_needs { "needs" } else { "uses" }
                            );
                            for (i, name) in SERVICES.iter().enumerate() {
                                let cap =
                                    capabilities / 8usize.pow(u32::try_from(i).expect("small")) % 8;
                                let counter = COUNTERS[cap % 4];
                                let source = cap / 4 == 1;
                                let to: Vec<String> = edges
                                    .iter()
                                    .enumerate()
                                    .filter(|(k, (from, _))| *from == i && edge_set >> k & 1 == 1)
                                    .map(|(_, (_, to))| SERVICES[*to].to_string())
                                    .collect();
                                label.push_str(&format!(
                                    "{name}:{}{} ",
                                    counter.word(),
                                    if source { "+source" } else { "" }
                                ));
                                let mut declared = service(name, counter, source, Vec::new());
                                declared.side = named(declared.side, to, by_needs);
                                declared.policy = b_policy && *name == "b";
                                services.push(declared);
                            }
                            let tasks = vec![Task {
                                name: "t".to_string(),
                                release: Release::Period(Dur::Ms(10)),
                                priority: 1,
                                deadline_from: Some("release"),
                                side: named(Side::default(), subset(t_set), by_needs),
                                ..Task::default()
                            }];
                            let edge_words: Vec<String> = edges
                                .iter()
                                .enumerate()
                                .filter(|(k, _)| edge_set >> k & 1 == 1)
                                .map(|(_, (from, to))| {
                                    format!("{}>{}", SERVICES[*from], SERVICES[*to])
                                })
                                .collect();
                            label.push_str(&format!(
                                "edges=[{}] t=[{}]",
                                edge_words.join(","),
                                subset(t_set).join(",")
                            ));
                            let own = own_level(&services, &tasks, by_needs);
                            let blocks = match shape {
                                Shape::OneTimer => vec![
                                    timer_block("timer.d", 3, true),
                                    source_block("dev.s", Separation::Stated(20), ""),
                                ],
                                Shape::TwoTimers => vec![
                                    timer_block("timer.d", 1, true),
                                    timer_block("timer.e", 1, true),
                                    source_block("dev.s", Separation::Stated(20), ""),
                                ],
                                Shape::Both => vec![timer_and_source_block("dev.t", 3)],
                            };
                            cases.push(Case {
                                label,
                                description: Description {
                                    system: "sys".to_string(),
                                    blocks,
                                    services,
                                    tasks,
                                    own,
                                    ..Description::default()
                                },
                            });
                        }
                    }
                }
            }
        }
    }
    cases
}

/// Who a source of the releases universe is reached by.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Path {
    /// Task `t` binds `interrupt-source` itself.
    DirectT,
    /// Task `u` binds it itself.
    DirectU,
    /// The system's own level binds it itself.
    DirectSys,
    /// A service `evt.X` binds it, used by `t`.
    ServiceT,
    /// `evt.X` used by a service `h.X` that `t` uses.
    ChainT,
    /// `evt.X` used by `u`.
    ServiceU,
    /// `evt.X` used by both tasks.
    ServiceTU,
    /// `evt.X` used by the system's own level alone, reaching no task.
    ServiceSys,
}

/// The separations a source may state, against task `t`'s 20 ms and `u`'s 40 ms: shorter, equal and longer than each,
/// unstated, and declared absent.
const SEPARATIONS: [Separation; 6] = [
    Separation::Stated(10),
    Separation::Stated(20),
    Separation::Stated(40),
    Separation::Stated(80),
    Separation::Unstated,
    Separation::Absent,
];

/// **Releases and routes** (§2.2, §2.5): task `t`, periodic or sporadic at 20 ms; task `u` absent, periodic or
/// sporadic at 40 ms; one or two sources, each of every separation class, reaching a task directly, through a service
/// or through two, or reaching none; an alarm `t` uses, or none. The first source, `dev.z`, is declared first and the
/// second, `dev.a`, is met only by a service requiring `(available-in-state run sleep)`, which it alone offers, so the
/// two are distinct and their texts' order is not their declaration order. (Names written by `needs` are the adapters
/// universe's; here every declaration is named by `uses`.)
fn releases_and_routes() -> Vec<Case> {
    let mut cases = Vec::new();
    for alarm in [false, true] {
        for t_sporadic in [false, true] {
            for u_kind in [None, Some(false), Some(true)] {
                let mut first_paths = vec![Path::DirectT, Path::DirectSys];
                let mut second_paths = Vec::new();
                for path in [Path::ServiceT, Path::ChainT, Path::ServiceSys] {
                    first_paths.push(path);
                    second_paths.push(path);
                }
                if u_kind.is_some() {
                    first_paths.push(Path::DirectU);
                    for path in [Path::ServiceU, Path::ServiceTU] {
                        first_paths.push(path);
                        second_paths.push(path);
                    }
                }
                let mut sources: Vec<Vec<(Separation, Path)>> = Vec::new();
                for &s1 in &SEPARATIONS {
                    for &p1 in &first_paths {
                        sources.push(vec![(s1, p1)]);
                        for &s2 in &SEPARATIONS {
                            for &p2 in &second_paths {
                                sources.push(vec![(s1, p1), (s2, p2)]);
                            }
                        }
                    }
                }
                for chosen in sources {
                    cases.push(release_case(alarm, t_sporadic, u_kind, &chosen));
                }
            }
        }
    }
    cases
}

fn release_case(
    alarm: bool,
    t_sporadic: bool,
    u_kind: Option<bool>,
    sources: &[(Separation, Path)],
) -> Case {
    const BLOCKS: [&str; 2] = ["dev.z", "dev.a"];
    const STATES: [&str; 2] = ["run", "run sleep"];
    let mut t = Side::default();
    let mut u = Side::default();
    let mut own = Side::default();
    let mut blocks = Vec::new();
    let mut services = Vec::new();
    // A declaration named by `uses`.
    let name_in = |side: &mut Side, declaration: &str| side.uses.push(declaration.to_string());
    let mut label = format!(
        "t={} u={} alarm={} sources=",
        if t_sporadic { "sporadic" } else { "periodic" },
        match u_kind {
            None => "-",
            Some(false) => "periodic",
            Some(true) => "sporadic",
        },
        if alarm { "yes" } else { "no" }
    );
    for (i, (separation, path)) in sources.iter().enumerate() {
        let block = BLOCKS[i];
        let suffix = &block[4..];
        blocks.push(source_block(block, *separation, STATES[i]));
        label.push_str(&format!("{block}:{separation:?}:{path:?} "));
        let evt = format!("evt.{suffix}");
        let mut handler = service(&evt, Counter::None, true, Vec::new());
        if i == 1 {
            handler
                .side
                .other
                .push("(available-in-state run sleep)".to_string());
        }
        match path {
            Path::DirectT => t.needs.push("interrupt-source".to_string()),
            Path::DirectU => u.needs.push("interrupt-source".to_string()),
            Path::DirectSys => own.needs.push("interrupt-source".to_string()),
            Path::ServiceT => name_in(&mut t, &evt),
            Path::ChainT => {
                let chain = format!("h.{suffix}");
                name_in(&mut t, &chain);
                let mut through = service(&chain, Counter::None, false, Vec::new());
                name_in(&mut through.side, &evt);
                services.push(through);
            }
            Path::ServiceU => name_in(&mut u, &evt),
            Path::ServiceTU => {
                name_in(&mut t, &evt);
                name_in(&mut u, &evt);
            }
            Path::ServiceSys => name_in(&mut own, &evt),
        }
        if !matches!(path, Path::DirectT | Path::DirectU | Path::DirectSys) {
            services.push(handler);
        }
    }
    if alarm {
        blocks.push(timer_block("timer.d", 1, false));
        services.push(service("alarm", Counter::Timer, false, Vec::new()));
        name_in(&mut t, "alarm");
    }
    let mut tasks = vec![Task {
        name: "t".to_string(),
        release: if t_sporadic {
            Release::MinSeparation(Dur::Ms(20))
        } else {
            Release::Period(Dur::Ms(20))
        },
        priority: 1,
        deadline_from: Some("release"),
        side: t,
        ..Task::default()
    }];
    if let Some(sporadic) = u_kind {
        tasks.push(Task {
            name: "u".to_string(),
            release: if sporadic {
                Release::MinSeparation(Dur::Ms(40))
            } else {
                Release::Period(Dur::Ms(40))
            },
            priority: 2,
            deadline_from: Some("release"),
            side: u,
            ..Task::default()
        });
    }
    Case {
        label: label.trim_end().to_string(),
        description: Description {
            system: "sys".to_string(),
            blocks,
            services,
            tasks,
            own,
            ..Description::default()
        },
    }
}

/// Who requires in the requesters universe.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Requester {
    /// The periodic task `t` itself.
    PeriodicTask,
    /// The sporadic task `t` itself.
    SporadicTask,
    /// The system's own level, beside a periodic task `t` naming nothing.
    OwnLevel,
    /// A service `s` the periodic task `t` names.
    Service,
}

/// **Requesters of every kind** (§1, §2.2, §2.6): a periodic task, a sporadic task, the system's own level or a
/// service, requiring a timer's channel, an absolute deadline, an extended counter or a source; with one timer that
/// both adapter kinds can program, and one source stating 20 ms.
fn requesters_of_every_kind() -> Vec<Case> {
    let mut cases = Vec::new();
    for requester in [
        Requester::PeriodicTask,
        Requester::SporadicTask,
        Requester::OwnLevel,
        Requester::Service,
    ] {
        for (counter, source) in [
            (Counter::Timer, false),
            (Counter::Deadline, false),
            (Counter::Extender, false),
            (Counter::None, true),
        ] {
            // What the requester writes: a service's requirements, the same for each kind of requester.
            let wanted = service("s", counter, source, Vec::new());
            let mut t = Side::default();
            let mut own = Side::default();
            let mut services = Vec::new();
            match requester {
                Requester::PeriodicTask | Requester::SporadicTask => {
                    // A task writes no constraint clause, the language's `task` having none: its extended counter
                    // is still met by the extender alone, no block offering `counter-width`.
                    t = wanted.side;
                    t.other.clear();
                }
                Requester::OwnLevel => own = wanted.side,
                Requester::Service => {
                    t.uses.push("s".to_string());
                    services.push(wanted);
                }
            }
            cases.push(Case {
                label: format!(
                    "requester={requester:?} requires={}{}",
                    counter.word(),
                    if source { "+source" } else { "" }
                ),
                description: Description {
                    system: "sys".to_string(),
                    blocks: vec![
                        timer_block("timer.d", 1, true),
                        source_block("dev.s", Separation::Stated(20), ""),
                    ],
                    services,
                    tasks: vec![Task {
                        name: "t".to_string(),
                        release: if requester == Requester::SporadicTask {
                            Release::MinSeparation(Dur::Ms(20))
                        } else {
                            Release::Period(Dur::Ms(20))
                        },
                        priority: 1,
                        deadline_from: Some("release"),
                        side: t,
                        ..Task::default()
                    }],
                    own,
                    ..Description::default()
                },
            });
        }
    }
    cases
}

/// **Two tasks sharing services that take a channel** (§2.2, §2.4, §2.6, §2.7, §2.9, §2.10): a periodic task `t` and
/// a sporadic task `u`, each writing `(deadline-from release)` or not; two services `a` and `b`, each taking a channel
/// of the one timer, an absolute deadline from a `deadline-over-delay` instance with its obliged `delivery-bound` bare
/// or bounded, or nothing, and handling the one source or not; every set of `uses` edges between them; each task using
/// every subset of them; the system's own level using each service no task reaches. Two jobs calling one channel, an
/// expiry running a node both jobs reach, a cycle making one initialisation step of two members.
fn two_tasks_sharing_services() -> Vec<Case> {
    const SERVICES: [&str; 2] = ["a", "b"];
    const COUNTERS: [Counter; 4] = [
        Counter::None,
        Counter::Timer,
        Counter::Deadline,
        Counter::DeadlineBound,
    ];
    let edges = pairs(2);
    let subset = |set: usize| -> Vec<String> {
        SERVICES
            .iter()
            .enumerate()
            .filter(|(i, _)| set >> i & 1 == 1)
            .map(|(_, s)| (*s).to_string())
            .collect()
    };
    let mut cases = Vec::new();
    for capabilities in 0..8usize.pow(2) {
        for edge_set in 0..1usize << edges.len() {
            for t_set in 0..1usize << SERVICES.len() {
                for u_set in 0..1usize << SERVICES.len() {
                    for from_set in 0..4usize {
                        let mut services = Vec::new();
                        let mut label = String::from("caps=");
                        for (i, name) in SERVICES.iter().enumerate() {
                            let cap =
                                capabilities / 8usize.pow(u32::try_from(i).expect("small")) % 8;
                            let counter = COUNTERS[cap % 4];
                            let source = cap / 4 == 1;
                            let uses = edges
                                .iter()
                                .enumerate()
                                .filter(|(k, (from, _))| *from == i && edge_set >> k & 1 == 1)
                                .map(|(_, (_, to))| SERVICES[*to].to_string())
                                .collect();
                            label.push_str(&format!(
                                "{name}:{}{} ",
                                counter.word(),
                                if source { "+source" } else { "" }
                            ));
                            services.push(service(name, counter, source, uses));
                        }
                        let (t_from, u_from) = (from_set & 1 == 1, from_set & 2 == 2);
                        let tasks = vec![
                            Task {
                                name: "t".to_string(),
                                release: Release::Period(Dur::Ms(10)),
                                priority: 1,
                                deadline_from: t_from.then_some("release"),
                                side: named(Side::default(), subset(t_set), false),
                                ..Task::default()
                            },
                            Task {
                                name: "u".to_string(),
                                release: Release::MinSeparation(Dur::Ms(20)),
                                priority: 2,
                                deadline_from: u_from.then_some("release"),
                                side: named(Side::default(), subset(u_set), false),
                                ..Task::default()
                            },
                        ];
                        let edge_words: Vec<String> = edges
                            .iter()
                            .enumerate()
                            .filter(|(k, _)| edge_set >> k & 1 == 1)
                            .map(|(_, (from, to))| format!("{}>{}", SERVICES[*from], SERVICES[*to]))
                            .collect();
                        label.push_str(&format!(
                            "uses=[{}] t=[{}]{} u=[{}]{}",
                            edge_words.join(","),
                            subset(t_set).join(","),
                            if t_from { "+from" } else { "" },
                            subset(u_set).join(","),
                            if u_from { "+from" } else { "" },
                        ));
                        let own = own_level(&services, &tasks, false);
                        cases.push(Case {
                            label,
                            description: Description {
                                system: "sys".to_string(),
                                blocks: vec![
                                    timer_block("timer.d", 3, true),
                                    source_block("dev.s", Separation::Stated(20), ""),
                                ],
                                services,
                                tasks,
                                own,
                                ..Description::default()
                            },
                        });
                    }
                }
            }
        }
    }
    cases
}

/// What one requester of the receivers universe programs itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Programs {
    Nothing,
    /// An alarm: `(needs relative-delay)`, bound to the timer itself.
    Alarm,
    /// An absolute deadline: `(needs absolute-deadline)`, met by its own `deadline-over-delay` instance, whose
    /// `expiry` runs its requester's code.
    Deadline,
}

impl Programs {
    /// Both an alarm and a deadline for one requester is no choice here: the two facts are of one function, which
    /// model §8 rule 13 has one provider meet, and neither the timer nor the instance offers both, so the search
    /// concludes `missing-fact` on every such system.
    const ALL: [Self; 3] = [Self::Nothing, Self::Alarm, Self::Deadline];

    fn needs(self) -> Vec<String> {
        match self {
            Self::Nothing => Vec::new(),
            Self::Alarm => names(&["relative-delay"]),
            Self::Deadline => names(&["absolute-deadline"]),
        }
    }
}

/// **Receivers of interrupt code** (§1): tasks `t` and `u`, each periodic or sporadic, each writing `(deadline-from
/// release)` or not, `t` first in runtime order or `u`; each of them and the system's own level programming nothing, an
/// alarm itself, or an absolute deadline whose expiry it receives; over one timer with channels for every requester.
/// So §1's refusal naming every receiver, each task in runtime order and then the system's own level, meets every set
/// of receivers in both runtime orders.
fn receivers_of_interrupt_code() -> Vec<Case> {
    let mut cases = Vec::new();
    for t_programs in Programs::ALL {
        for u_programs in Programs::ALL {
            for own_programs in Programs::ALL {
                for u_first in [false, true] {
                    for kinds in 0..4usize {
                        for from_set in 0..4usize {
                            let task = |name: &str,
                                        sporadic: bool,
                                        priority: u32,
                                        from: bool,
                                        programs: Programs| {
                                Task {
                                    name: name.to_string(),
                                    release: if sporadic {
                                        Release::MinSeparation(Dur::Ms(20))
                                    } else {
                                        Release::Period(Dur::Ms(20))
                                    },
                                    priority,
                                    deadline_from: from.then_some("release"),
                                    side: Side {
                                        needs: programs.needs(),
                                        ..Side::default()
                                    },
                                    ..Task::default()
                                }
                            };
                            let (t_sporadic, u_sporadic) = (kinds & 1 == 1, kinds & 2 == 2);
                            let (t_from, u_from) = (from_set & 1 == 1, from_set & 2 == 2);
                            let (t_priority, u_priority) = if u_first { (2, 1) } else { (1, 2) };
                            cases.push(Case {
                                label: format!(
                                    "t:{}{}:{t_programs:?} u:{}{}:{u_programs:?} own:{own_programs:?} first={}",
                                    if t_sporadic { "sporadic" } else { "periodic" },
                                    if t_from { "+from" } else { "" },
                                    if u_sporadic { "sporadic" } else { "periodic" },
                                    if u_from { "+from" } else { "" },
                                    if u_first { "u" } else { "t" },
                                ),
                                description: Description {
                                    system: "sys".to_string(),
                                    blocks: vec![timer_block("timer.d", 6, true)],
                                    services: Vec::new(),
                                    tasks: vec![
                                        task("t", t_sporadic, t_priority, t_from, t_programs),
                                        task("u", u_sporadic, u_priority, u_from, u_programs),
                                    ],
                                    own: Side {
                                        needs: own_programs.needs(),
                                        ..Side::default()
                                    },
                                    ..Description::default()
                                },
                            });
                        }
                    }
                }
            }
        }
    }
    cases
}

/// What a block or a platform of the hardware universe requires at its own level (model §8 rule 1: "any other
/// declaration, for what it writes at its own level").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Requires {
    Nothing,
    /// A timer's channel: `(needs relative-delay)`, the alarm it programs itself.
    Alarm,
    /// An absolute deadline: `(needs absolute-deadline)`, met by its own `deadline-over-delay` instance, whose expiry
    /// runs its code.
    Deadline,
    /// An interrupt source: `(needs interrupt-source)`, its handler run in the source's interrupt.
    Source,
    /// Another block's output: `(needs observable-output)`.
    Output,
}

impl Requires {
    const ALL: [Self; 5] = [
        Self::Nothing,
        Self::Alarm,
        Self::Deadline,
        Self::Source,
        Self::Output,
    ];

    fn needs(self) -> Vec<String> {
        match self {
            Self::Nothing => Vec::new(),
            Self::Alarm => names(&["relative-delay"]),
            Self::Deadline => names(&["absolute-deadline"]),
            Self::Source => names(&["interrupt-source"]),
            Self::Output => names(&["observable-output"]),
        }
    }
}

/// Who binds to what a block or a platform of the hardware universe offers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Binder {
    Nothing,
    /// A service the system's own level uses.
    OwnService,
    /// A service task `t` uses.
    TService,
    /// A service task `u` uses.
    UService,
    /// A service both tasks use.
    BothService,
    /// Task `t` itself.
    TDirect,
    /// The system's own level itself.
    OwnDirect,
    /// A service task `u` uses that also handles the source, so that the source's interrupt reaches the block or the
    /// platform through the binding, beside `u`'s job.
    UHandler,
}

impl Binder {
    const ALL: [Self; 8] = [
        Self::Nothing,
        Self::OwnService,
        Self::TService,
        Self::UService,
        Self::BothService,
        Self::TDirect,
        Self::OwnDirect,
        Self::UHandler,
    ];
}

/// What names the service `alarm`, which task `t` uses, by a `uses` of its own: nothing, `dma.x` or `soc` — hardware
/// naming what is not hardware, refused before any plan since round 13 (§1).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Composer {
    Nothing,
    Block,
    Platform,
}

/// **Hardware with requirements of its own** (§2, §2.6, §2.7, §2.9, §2.10; round 12, R12-R4): a block `dma.x`
/// offering `transfer-engine` and the platform `soc` offering `core-count`, each requiring nothing, a timer's channel,
/// an absolute deadline, the source or another block's output; each bound to by nothing, by a service the system's own
/// level, task `t`, task `u` or both use, by `t` or the system's own level itself, or by a service `u` uses that
/// handles the source; the service `alarm`, which `t`
/// uses, named by nothing, by `dma.x`'s `uses` or by `soc`'s; and task `u` periodic or sporadic. So hardware is a node
/// bound to by something or by nothing, runs in its own alarm's, its expiry's and its source's interrupts, is
/// multiplexed by two jobs, releases a sporadic task it handles a source for, and a task reaches the platform through a
/// binding — while what a block or a platform composes is never reached through it, and no edge crosses composition.
/// Since round 13 (R13-D3), a block's or a platform's `uses` naming `alarm` is refused before any plan (§1), naming it.
fn requiring_hardware() -> Vec<Case> {
    let mut cases = Vec::new();
    for dma in Requires::ALL {
        for soc in Requires::ALL {
            for dma_binder in Binder::ALL {
                for soc_binder in Binder::ALL {
                    for composer in [Composer::Nothing, Composer::Block, Composer::Platform] {
                        for u_sporadic in [false, true] {
                            cases.push(hardware_case(
                                (dma, dma_binder),
                                (soc, soc_binder),
                                composer,
                                u_sporadic,
                            ));
                        }
                    }
                }
            }
        }
    }
    cases
}

fn hardware_case(
    (dma, dma_binder): (Requires, Binder),
    (soc, soc_binder): (Requires, Binder),
    composer: Composer,
    u_sporadic: bool,
) -> Case {
    let mut t = Side {
        uses: names(&["alarm"]),
        ..Side::default()
    };
    let mut u = Side::default();
    let mut own = Side::default();
    let mut services = vec![service("alarm", Counter::Timer, false, Vec::new())];
    for (binder, via, fact) in [
        (dma_binder, "xfer", "transfer-engine"),
        (soc_binder, "cpu", "core-count"),
    ] {
        let mut through = Service {
            name: via.to_string(),
            policy: false,
            side: Side {
                needs: names(&[fact]),
                ..Side::default()
            },
        };
        match binder {
            Binder::Nothing => {}
            Binder::OwnService => own.uses.push(via.to_string()),
            Binder::UHandler => {
                through.side.needs.push(INTERRUPT_FACT.to_string());
                u.uses.push(via.to_string());
            }
            Binder::TService => t.uses.push(via.to_string()),
            Binder::UService => u.uses.push(via.to_string()),
            Binder::BothService => {
                t.uses.push(via.to_string());
                u.uses.push(via.to_string());
            }
            Binder::TDirect => t.needs.push(fact.to_string()),
            Binder::OwnDirect => own.needs.push(fact.to_string()),
        }
        if matches!(
            binder,
            Binder::OwnService
                | Binder::TService
                | Binder::UService
                | Binder::BothService
                | Binder::UHandler
        ) {
            services.push(through);
        }
    }
    let mut dma_block = Block {
        name: "dma.x".to_string(),
        offers: names(&["(transfer-engine true)", "(reachable-at-privilege machine)"]),
        side: Side {
            needs: dma.needs(),
            ..Side::default()
        },
        ..Block::default()
    };
    if composer == Composer::Block {
        dma_block.side.uses.push("alarm".to_string());
    }
    let blocks = vec![
        timer_block("timer.d", 4, true),
        source_block("dev.s", Separation::Stated(20), ""),
        Block {
            name: "uart.c".to_string(),
            offers: names(&[
                "(observable-output true)",
                "(reachable-at-privilege machine)",
            ]),
            ..Block::default()
        },
        dma_block,
    ];
    let mut composed: Vec<String> = blocks.iter().map(|b| b.name.clone()).collect();
    if composer == Composer::Platform {
        composed.push("alarm".to_string());
    }
    let platform = Block {
        name: "soc".to_string(),
        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
        side: Side {
            needs: soc.needs(),
            uses: composed,
            ..Side::default()
        },
        ..Block::default()
    };
    let tasks = vec![
        Task {
            name: "t".to_string(),
            release: Release::Period(Dur::Ms(10)),
            priority: 1,
            side: t,
            ..Task::default()
        },
        Task {
            name: "u".to_string(),
            release: if u_sporadic {
                Release::MinSeparation(Dur::Ms(20))
            } else {
                Release::Period(Dur::Ms(20))
            },
            priority: 2,
            side: u,
            ..Task::default()
        },
    ];
    Case {
        label: format!(
            "dma.x:{dma:?} by {dma_binder:?}; soc:{soc:?} by {soc_binder:?}; alarm composed by {composer:?}; u {}",
            if u_sporadic { "sporadic" } else { "periodic" }
        ),
        description: Description {
            blocks,
            platforms: vec![platform],
            services,
            tasks,
            own,
            ..Description::default()
        },
    }
}

/// The jitters a task of the task-clauses universe writes: none, zero, 1500 us, one longer than its period or
/// separation, and one past the arithmetic.
const JITTERS: [Option<Dur>; 5] = [
    None,
    Some(Dur::Ms(0)),
    Some(Dur::Spelled("1500 us")),
    Some(Dur::Ms(50)),
    Some(Dur::Spelled(PAST)),
];

/// The overrun policies a task writes: none, `fault` and `skip-late-job`, the two the profile performs.
const OVERRUNS: [Option<&str>; 3] = [None, Some("fault"), Some("skip-late-job")];

/// The deadline's reference: `release`, none written, and another event.
const REFERENCES: [Option<&str>; 3] = [Some("release"), None, Some("activation")];

/// **What a task line states** (§2.2, §1; round 12, R12-R4): a periodic task `t` and a sporadic task `u`, either
/// first in runtime order, each writing every jitter of [`JITTERS`], every overrun policy of [`OVERRUNS`] and every
/// deadline reference of [`REFERENCES`]. So "its release jitter, written as zero where none is stated" and "its overrun
/// policy, `fault` where none is stated" are read, and a deadline from another event is refused task by task in
/// runtime order, after a jitter past the arithmetic, the second list's first bullet.
fn task_clauses() -> Vec<Case> {
    let mut cases = Vec::new();
    for u_first in [false, true] {
        for t_jitter in JITTERS {
            for t_overrun in OVERRUNS {
                for t_from in REFERENCES {
                    for u_jitter in JITTERS {
                        for u_overrun in OVERRUNS {
                            for u_from in REFERENCES {
                                let (t_priority, u_priority) =
                                    if u_first { (2, 1) } else { (1, 2) };
                                let task =
                                    |name: &str,
                                     release: Release,
                                     priority: u32,
                                     jitter: Option<Dur>,
                                     overrun: Option<&'static str>,
                                     from: Option<&'static str>,
                                     uses: &str| Task {
                                        name: name.to_string(),
                                        release,
                                        jitter,
                                        overrun,
                                        priority,
                                        deadline_from: from,
                                        side: Side {
                                            uses: names(&[uses]),
                                            ..Side::default()
                                        },
                                        ..Task::default()
                                    };
                                cases.push(Case {
                                    label: format!(
                                        "t:{t_jitter:?},{t_overrun:?},{t_from:?} u:{u_jitter:?},{u_overrun:?},{u_from:?} \
                                         first={}",
                                        if u_first { "u" } else { "t" }
                                    ),
                                    description: Description {
                                        blocks: vec![
                                            Block {
                                                name: "uart.c".to_string(),
                                                offers: names(&[
                                                    "(observable-output true)",
                                                    "(reachable-at-privilege machine)",
                                                ]),
                                                ..Block::default()
                                            },
                                            source_block("dev.s", Separation::Stated(40), ""),
                                        ],
                                        services: vec![
                                            Service {
                                                name: "log".to_string(),
                                                policy: false,
                                                side: Side {
                                                    needs: names(&["observable-output"]),
                                                    ..Side::default()
                                                },
                                            },
                                            service("evt", Counter::None, true, Vec::new()),
                                        ],
                                        tasks: vec![
                                            task(
                                                "t",
                                                Release::Period(Dur::Ms(20)),
                                                t_priority,
                                                t_jitter,
                                                t_overrun,
                                                t_from,
                                                "log",
                                            ),
                                            task(
                                                "u",
                                                Release::MinSeparation(Dur::Ms(40)),
                                                u_priority,
                                                u_jitter,
                                                u_overrun,
                                                u_from,
                                                "evt",
                                            ),
                                        ],
                                        ..Description::default()
                                    },
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    cases
}

/// What one source of the arithmetic universe states of its separation.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SourceState {
    /// A separation within the arithmetic.
    Within,
    /// One spelling past it.
    Past,
    /// Two spellings, one within and one past, whose comparison overflows it.
    Overflowing,
    /// One spelling past it, the source bound by nothing, so that the plan writes no separation of it.
    PastUnbound,
}

/// What a `deadline-over-delay` requester of the arithmetic universe writes on `delivery-bound`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum BoundState {
    Bare,
    Within,
    Past,
}

/// The separations of the arithmetic universe's sources, `dev.z` of 20 ms and `dev.a` of 40 ms, and at the
/// arithmetic's edge `dev.z` of [`WIDE`] in two agreeing spellings and `dev.a` of [`WIDE_LONGER`]; each also with a
/// spelling past the arithmetic beside it, [`PAST`] or, at the edge, [`TIGHT_PAST`].
const Z_WITHIN: [&[&str]; 2] = [&["20 ms"], &[WIDE, WIDE_IN_MS]];
const Z_OVERFLOWING: [&[&str]; 2] = [&["20 ms", PAST], &[WIDE, TIGHT_PAST]];
const A_WITHIN: [&[&str]; 2] = [&["40 ms"], &[WIDE_LONGER]];
const A_OVERFLOWING: [&[&str]; 2] = [&["40 ms", PAST], &[WIDE_LONGER, TIGHT_PAST]];
const ONLY_PAST: [&[&str]; 2] = [&[PAST], &[TIGHT_PAST]];

/// **Durations past the arithmetic, and at its edge** (§1, §2.2, §2.4, §2.5; round 12, R12-R4): a periodic task `t`
/// using a source `dev.z` and two `deadline-over-delay` requesters `dl.a` and `dl.b`, and a sporadic task `u` using a
/// source `dev.a`, either first in runtime order; each of `t`'s period, deadline and jitter past the arithmetic or not,
/// and `u`'s three past or none; each source within, past in one spelling, overflowing in two, or past and bound by
/// nothing; each requester's bound bare, within or past; and every value within the arithmetic a few milliseconds, or
/// at its edge — denominators of `10^37`, `10^38` and `8.192e37`, past it by the least a spelling reaches, spellings
/// only their reduction brings within, and comparisons whose cross-products pass 128 bits. So §1 names each duration past the arithmetic in its order, and a duration
/// within it is written exactly however it is spelled.
fn past_the_arithmetic() -> Vec<Case> {
    const SOURCES: [SourceState; 4] = [
        SourceState::Within,
        SourceState::Past,
        SourceState::Overflowing,
        SourceState::PastUnbound,
    ];
    const BOUNDS: [BoundState; 3] = [BoundState::Bare, BoundState::Within, BoundState::Past];
    let mut cases = Vec::new();
    for edge in [false, true] {
        for u_first in [false, true] {
            for t_past in 0..8usize {
                for u_past in [false, true] {
                    for z in SOURCES {
                        for a in SOURCES {
                            for dl_a in BOUNDS {
                                for dl_b in BOUNDS {
                                    cases.push(arithmetic_case(
                                        edge,
                                        u_first,
                                        (t_past, u_past),
                                        (z, a),
                                        (dl_a, dl_b),
                                    ));
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    cases
}

fn arithmetic_case(
    edge: bool,
    u_first: bool,
    (t_past, u_past): (usize, bool),
    (z, a): (SourceState, SourceState),
    (dl_a, dl_b): (BoundState, BoundState),
) -> Case {
    let at = usize::from(edge);
    let within = |plain: Dur, wide: &'static str| if edge { Dur::Spelled(wide) } else { plain };
    let past = Dur::Spelled(if edge { TIGHT_PAST } else { PAST });
    let mut blocks = vec![timer_block("timer.d", 2, true)];
    let mut services = Vec::new();
    let mut t = Side {
        uses: names(&["dl.a", "dl.b"]),
        ..Side::default()
    };
    let mut u = Side::default();
    for (block, state, within_spellings, overflowing, states, handler, side) in [
        (
            "dev.z",
            z,
            Z_WITHIN[at],
            Z_OVERFLOWING[at],
            "",
            "evt.z",
            &mut t,
        ),
        (
            "dev.a",
            a,
            A_WITHIN[at],
            A_OVERFLOWING[at],
            "run sleep",
            "evt.a",
            &mut u,
        ),
    ] {
        let spellings = match state {
            SourceState::Within => within_spellings,
            SourceState::Past | SourceState::PastUnbound => ONLY_PAST[at],
            SourceState::Overflowing => overflowing,
        };
        blocks.push(source_block(block, Separation::Spelled(spellings), states));
        if state != SourceState::PastUnbound {
            let mut evt = service(handler, Counter::None, true, Vec::new());
            if !states.is_empty() {
                evt.side
                    .other
                    .push(format!("(available-in-state {states})"));
            }
            services.push(evt);
            side.uses.push(handler.to_string());
        }
    }
    for (name, state, wide) in [("dl.a", dl_a, WIDE), ("dl.b", dl_b, TIGHT_WITHIN)] {
        let mut deadline = service(name, Counter::Deadline, false, Vec::new());
        match state {
            BoundState::Bare => {}
            BoundState::Within => deadline.side.at_most.push((
                "delivery-bound".to_string(),
                within(Dur::Ms(DELIVERY_BOUND_MS), wide),
            )),
            BoundState::Past => deadline
                .side
                .at_most
                .push(("delivery-bound".to_string(), past)),
        }
        services.push(deadline);
    }
    let (t_priority, u_priority) = if u_first { (2, 1) } else { (1, 2) };
    let pick = |is_past: bool, within: Dur| if is_past { past } else { within };
    let tasks = vec![
        Task {
            name: "t".to_string(),
            release: Release::Period(pick(t_past & 1 == 1, within(Dur::Ms(20), WIDE_LONGER))),
            deadline: Some(pick(t_past & 2 == 2, within(Dur::Ms(20), WIDE))),
            jitter: if t_past & 4 == 4 {
                Some(past)
            } else if edge {
                Some(Dur::Spelled(REDUCED))
            } else {
                None
            },
            priority: t_priority,
            side: t,
            ..Task::default()
        },
        Task {
            name: "u".to_string(),
            release: Release::MinSeparation(pick(u_past, within(Dur::Ms(40), WIDE))),
            deadline: Some(pick(u_past, within(Dur::Ms(40), WIDE_IN_MS))),
            jitter: u_past.then_some(past),
            priority: u_priority,
            side: u,
            ..Task::default()
        },
    ];
    Case {
        label: format!(
            "{} first={} t-past={}{}{} u-past={u_past} dev.z:{z:?} dev.a:{a:?} dl.a:{dl_a:?} dl.b:{dl_b:?}",
            if edge { "edge" } else { "plain" },
            if u_first { "u" } else { "t" },
            if t_past & 1 == 1 { "period," } else { "" },
            if t_past & 2 == 2 { "deadline," } else { "" },
            if t_past & 4 == 4 { "jitter" } else { "" },
        ),
        description: Description {
            blocks,
            services,
            tasks,
            ..Description::default()
        },
    }
}

/// **A second system** (§1, §2; round 12, R12-R4 and R12-R1): systems `sys` and `other` in one description, each
/// lowered. A system is named by `uses`, or every one by `needs`, at each set of five sites — `sys`'s task, `sys`'s own
/// level and a service `sys`'s task uses naming `other`, and `other`'s task and a service only `other`'s task uses
/// naming `sys`; the service `alarm`, taking a channel, used by `sys`'s task alone or by both systems' tasks; `other`
/// on `sys`'s platform or on its own, which composes a block requiring a timer's channel; each system's own level using
/// nothing or a service. So §1's naming-system refusal reads the requesters the lowered system serves and no other —
/// a system reached served for its own level alone — the generator's closure is held to the configuration checker on
/// each, and a service two systems use, and hardware another system serves, are read system by system.
fn two_systems() -> Vec<Case> {
    let mut cases = Vec::new();
    for sites in 0..32usize {
        for by_needs in [false, true] {
            for shared_alarm in [false, true] {
                for own_platform in [false, true] {
                    for sys_own in 0..2usize {
                        for other_own in 0..3usize {
                            let [first, second] = two_systems_case(
                                sites,
                                by_needs,
                                shared_alarm,
                                own_platform,
                                (sys_own, other_own),
                            );
                            cases.push(first);
                            cases.push(second);
                        }
                    }
                }
            }
        }
    }
    cases
}

fn two_systems_case(
    sites: usize,
    by_needs: bool,
    shared_alarm: bool,
    own_platform: bool,
    (sys_own, other_own): (usize, usize),
) -> [Case; 2] {
    let site = |bit: usize| sites >> bit & 1 == 1;
    let name_in = |side: &mut Side, name: &str| {
        if by_needs {
            side.needs.push(name.to_string());
        } else {
            side.uses.push(name.to_string());
        }
    };
    let simple = |name: &str, needs: &str| Service {
        name: name.to_string(),
        policy: false,
        side: Side {
            needs: names(&[needs]),
            ..Side::default()
        },
    };
    let mut services = vec![
        simple("log", "observable-output"),
        simple("alarm", "relative-delay"),
    ];
    let mut t = Side {
        uses: names(&["log", "alarm"]),
        ..Side::default()
    };
    let mut o = Side {
        uses: names(&["log"]),
        ..Side::default()
    };
    if shared_alarm {
        o.uses.push("alarm".to_string());
    }
    let mut sys_level = Side::default();
    if sys_own == 1 {
        sys_level.uses.push("log".to_string());
    }
    let mut other_level = Side::default();
    match other_own {
        1 => other_level.uses.push("log".to_string()),
        2 => other_level.uses.push("alarm".to_string()),
        _ => {}
    }
    if site(0) {
        name_in(&mut t, "other");
    }
    if site(1) {
        name_in(&mut sys_level, "other");
    }
    if site(2) {
        let mut bridge = Service {
            name: "bridge.s".to_string(),
            policy: false,
            side: Side::default(),
        };
        name_in(&mut bridge.side, "other");
        services.push(bridge);
        t.uses.push("bridge.s".to_string());
    }
    if site(3) {
        name_in(&mut o, "sys");
    }
    if site(4) {
        let mut bridge = Service {
            name: "bridge.o".to_string(),
            policy: false,
            side: Side::default(),
        };
        name_in(&mut bridge.side, "sys");
        services.push(bridge);
        o.uses.push("bridge.o".to_string());
    }
    let uart = Block {
        name: "uart.c".to_string(),
        offers: names(&[
            "(observable-output true)",
            "(reachable-at-privilege machine)",
        ]),
        ..Block::default()
    };
    let mut blocks = vec![timer_block("timer.d", 4, false), uart];
    let platform = |name: &str, composes: &[&str]| Block {
        name: name.to_string(),
        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
        side: Side {
            uses: names(composes),
            ..Side::default()
        },
        ..Block::default()
    };
    let mut platforms = vec![platform("host.p", &["timer.d", "uart.c"])];
    if own_platform {
        blocks.push(timer_block("timer.q", 2, false));
        blocks.push(Block {
            name: "dma.q".to_string(),
            offers: names(&["(transfer-engine true)", "(reachable-at-privilege machine)"]),
            side: Side {
                needs: names(&["relative-delay"]),
                ..Side::default()
            },
            ..Block::default()
        });
        platforms.push(platform("host.q", &["uart.c", "timer.q", "dma.q"]));
    }
    let sys = System {
        name: "sys".to_string(),
        tasks: vec![Task {
            name: "t".to_string(),
            side: t,
            ..Task::default()
        }],
        own: sys_level,
        platform: "host.p".to_string(),
    };
    let other = System {
        name: "other".to_string(),
        tasks: vec![Task {
            name: "o".to_string(),
            release: Release::Period(Dur::Ms(20)),
            side: o,
            ..Task::default()
        }],
        own: other_level,
        platform: if own_platform { "host.q" } else { "host.p" }.to_string(),
    };
    let label = format!(
        "naming={} by {} alarm={} other on {} sys-own={sys_own} other-own={other_own}",
        [
            "t>other",
            "sys>other",
            "bridge.s>other",
            "o>sys",
            "bridge.o>sys",
        ]
        .iter()
        .enumerate()
        .filter(|(bit, _)| site(*bit))
        .map(|(_, s)| *s)
        .collect::<Vec<_>>()
        .join(","),
        if by_needs { "needs" } else { "uses" },
        if shared_alarm { "both" } else { "sys" },
        if own_platform { "host.q" } else { "host.p" },
    );
    let lowering = |lowered: &System, other: &System, others_first: bool| Case {
        label: format!("{label}: lowering {}", lowered.name),
        description: Description {
            system: lowered.name.clone(),
            blocks: blocks.clone(),
            platforms: platforms.clone(),
            services: services.clone(),
            tasks: lowered.tasks.clone(),
            own: lowered.own.clone(),
            platform: lowered.platform.clone(),
            others: vec![other.clone()],
            others_first,
        },
    };
    [lowering(&sys, &other, false), lowering(&other, &sys, true)]
}

/// What a block's or a platform's `requires` names beside what it requires: each name written by `needs` (`true`) or
/// by `uses` (`false`).
type Naming = &'static [(bool, &'static str)];

/// What `timer.d`, the provider `alarm` is bound to, names: nothing, or `alarm` itself, by `uses` or by `needs`.
const TIMER_NAMING: [Naming; 3] = [&[], &[(false, "alarm")], &[(true, "alarm")]];
/// What `dma.x` names: nothing; the policy `pol.p` by each clause; the service `only.s`, which nothing else names, by
/// `needs`; the block `uart.c` it is bound to, by each clause, and the timer by `needs` — hardware, no refusal, its
/// `uses` composition and its `needs` not (model §8 rule 5's "`uses` alone is composition"); the system `other`; and two
/// operands at once, written against their texts' order.
const DMA_NAMING: [Naming; 9] = [
    &[],
    &[(false, "pol.p")],
    &[(true, "pol.p")],
    &[(true, "only.s")],
    &[(false, "uart.c")],
    &[(true, "uart.c")],
    &[(true, "timer.d")],
    &[(false, "other")],
    &[(false, "pol.p"), (false, "only.s")],
];
/// What the platform `soc` names beside the blocks it composes: nothing; `only.s`; `pol.p` by `needs`; `alarm`, which
/// task `t` uses, and `only.s`; or the block `dev.s` by `needs`, hardware again.
const SOC_NAMING: [Naming; 5] = [
    &[],
    &[(false, "only.s")],
    &[(true, "pol.p")],
    &[(false, "alarm"), (false, "only.s")],
    &[(true, "dev.s")],
];
/// What `blk.q`, a block only the system `other`'s platform composes, names: nothing, or `only.s`.
const BLKQ_NAMING: [Naming; 2] = [&[], &[(false, "only.s")]];

/// Which other refusal of §1 the hardware-naming universe's task `t` would also meet: none; the first list's before
/// it — `t` periodic and bound to the source itself, or `t` naming the system `other` — or the second list's after it,
/// a jitter past the arithmetic, a deadline from another event, or an alarm `t` programs itself.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Also {
    Nothing,
    PeriodicSource,
    NamesSystem,
    JitterPast,
    DeadlineFromOther,
    Receiver,
}

impl Also {
    const ALL: [Self; 6] = [
        Self::Nothing,
        Self::PeriodicSource,
        Self::NamesSystem,
        Self::JitterPast,
        Self::DeadlineFromOther,
        Self::Receiver,
    ];
}

/// `side` naming each of `naming` by its clause.
fn naming(mut side: Side, naming: Naming) -> Side {
    for (by_needs, name) in naming {
        if *by_needs {
            side.needs.push((*name).to_string());
        } else {
            side.uses.push((*name).to_string());
        }
    }
    side
}

/// **Hardware naming what is not hardware** (§1, §2, §2.7; round 13, R13-R2's "a service only composed, a block's
/// `needs` naming a declaration"): `timer.d`, which `alarm` is bound to, naming `alarm`; the block `dma.x`, bound for
/// an output of its own and bound to by the service `xfer` task `u` uses, naming a policy, a service only it names, a
/// system, two operands at once, or another block — no refusal; the platform `soc` naming a service only it names,
/// the service a task uses, a policy, or another block; `blk.q`, on the platform only the system `other` uses, naming
/// a service; each by `uses` or `needs`; and task `t` meeting each other refusal of §1 that can co-apply, of the first
/// list before this one or of the second after it. So the refusal is taken in its place, names each block or
/// platform and its operand by the namer's text and then the operand's, reads hardware the lowered system does not
/// serve, and a block naming a block is lowered: across a binding, its `uses` composition making no edge, its `needs`
/// leaving the edge standing (§2.7, rule 12).
fn hardware_naming() -> Vec<Case> {
    let mut cases = Vec::new();
    for timer in TIMER_NAMING {
        for dma in DMA_NAMING {
            for soc in SOC_NAMING {
                for blk_q in BLKQ_NAMING {
                    for also in Also::ALL {
                        cases.push(hardware_naming_case((timer, dma, soc, blk_q), also));
                    }
                }
            }
        }
    }
    cases
}

fn hardware_naming_case(
    (timer, dma, soc, blk_q): (Naming, Naming, Naming, Naming),
    also: Also,
) -> Case {
    let words = |naming: Naming| -> String {
        let named: Vec<String> = naming
            .iter()
            .map(|(by_needs, name)| format!("{}:{name}", if *by_needs { "needs" } else { "uses" }))
            .collect();
        format!("[{}]", named.join(","))
    };
    let mut timer_d = timer_block("timer.d", 4, false);
    timer_d.side = naming(Side::default(), timer);
    let dma_x = Block {
        name: "dma.x".to_string(),
        offers: names(&["(transfer-engine true)", "(reachable-at-privilege machine)"]),
        side: naming(
            Side {
                needs: names(&["observable-output"]),
                ..Side::default()
            },
            dma,
        ),
        ..Block::default()
    };
    let blocks = vec![
        timer_d,
        source_block("dev.s", Separation::Stated(20), ""),
        Block {
            name: "uart.c".to_string(),
            offers: names(&[
                "(observable-output true)",
                "(reachable-at-privilege machine)",
            ]),
            ..Block::default()
        },
        dma_x,
        Block {
            name: "blk.q".to_string(),
            offers: names(&["(debug-port true)", "(reachable-at-privilege machine)"]),
            side: naming(Side::default(), blk_q),
            ..Block::default()
        },
    ];
    let platform = |name: &str, composes: &[&str], extra: Naming| Block {
        name: name.to_string(),
        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
        side: naming(
            Side {
                uses: names(composes),
                ..Side::default()
            },
            extra,
        ),
        ..Block::default()
    };
    let platforms = vec![
        platform("soc", &["timer.d", "dev.s", "uart.c", "dma.x"], soc),
        platform("host.q", &["blk.q"], &[]),
    ];
    let simple = |name: &str, policy: bool, needs: &[&str]| Service {
        name: name.to_string(),
        policy,
        side: Side {
            needs: names(needs),
            ..Side::default()
        },
    };
    let services = vec![
        simple("alarm", false, &["relative-delay"]),
        simple("xfer", false, &["transfer-engine"]),
        simple("only.s", false, &["relative-delay"]),
        simple("pol.p", true, &[]),
    ];
    let mut t = Task {
        name: "t".to_string(),
        side: Side {
            uses: names(&["alarm"]),
            ..Side::default()
        },
        ..Task::default()
    };
    match also {
        Also::Nothing => {}
        Also::PeriodicSource => t.side.needs.push(INTERRUPT_FACT.to_string()),
        Also::NamesSystem => t.side.uses.push("other".to_string()),
        Also::JitterPast => t.jitter = Some(Dur::Spelled(PAST)),
        Also::DeadlineFromOther => t.deadline_from = Some("activation"),
        Also::Receiver => t.side.needs.push("relative-delay".to_string()),
    }
    let u = Task {
        name: "u".to_string(),
        release: Release::Period(Dur::Ms(20)),
        priority: 2,
        side: Side {
            uses: names(&["xfer"]),
            ..Side::default()
        },
        ..Task::default()
    };
    let other = System {
        name: "other".to_string(),
        tasks: vec![Task {
            name: "o".to_string(),
            release: Release::Period(Dur::Ms(20)),
            ..Task::default()
        }],
        own: Side::default(),
        platform: "host.q".to_string(),
    };
    Case {
        label: format!(
            "timer.d names {} dma.x names {} soc names {} blk.q names {} t also {also:?}",
            words(timer),
            words(dma),
            words(soc),
            words(blk_q)
        ),
        description: Description {
            blocks,
            platforms,
            services,
            tasks: vec![t, u],
            others: vec![other],
            ..Description::default()
        },
    }
}

/// How the block `blk` of the served-block universe is named.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ServedBy {
    /// A service `drv` names it by `uses`, and task `t` uses `drv`.
    TDrvUses,
    /// `drv` names it by `needs`, and `t` uses `drv`.
    TDrvNeeds,
    /// `drv`, used by task `u`.
    UDrv,
    /// `drv`, used by both tasks.
    BothDrv,
    /// `drv`, used by the system's own level.
    OwnDrv,
    /// `drv`, used by nothing: `blk` is served by no requester.
    NoDrv,
    /// Task `t` names `blk` itself: hardware (model §8 rule 7).
    TDirect,
}

impl ServedBy {
    const ALL: [Self; 7] = [
        Self::TDrvUses,
        Self::TDrvNeeds,
        Self::UDrv,
        Self::BothDrv,
        Self::OwnDrv,
        Self::NoDrv,
        Self::TDirect,
    ];
}

/// **A block a service names** (§2; model §8 rule 7; round 13, R13-R2: "the reading took rule 7's 'hardware' where §2
/// says 'serves'"): a block `blk` no platform composes, requiring nothing, a timer's channel, an absolute deadline, the
/// source or an output; named by a service `drv` that task `t`, task `u`, both, the system's own level or nothing uses,
/// by `uses` or `needs`, or by task `t` itself; its own offer bound to by a service `t` uses or by nothing; and task
/// `u` periodic or sporadic. So a block "the system serves that is bound for requirements of its own" is a node though
/// it is no hardware — "A declaration of another kind ends the path: a block that only a service names is not
/// hardware" — and runs where what reaches it runs, while a requester bound to it, outside the hardware, is the
/// search's conflict (model §8 rule 7).
fn served_blocks() -> Vec<Case> {
    let mut cases = Vec::new();
    for by in ServedBy::ALL {
        for requires in Requires::ALL {
            for bound_to in [false, true] {
                for u_sporadic in [false, true] {
                    cases.push(served_block_case(by, requires, bound_to, u_sporadic));
                }
            }
        }
    }
    cases
}

fn served_block_case(by: ServedBy, requires: Requires, bound_to: bool, u_sporadic: bool) -> Case {
    let mut t = Side::default();
    let mut u = Side::default();
    let mut own = Side::default();
    let mut services = Vec::new();
    let mut drv = Service {
        name: "drv".to_string(),
        policy: false,
        side: Side::default(),
    };
    if by == ServedBy::TDrvNeeds {
        drv.side.needs.push("blk".to_string());
    } else {
        drv.side.uses.push("blk".to_string());
    }
    match by {
        ServedBy::TDrvUses | ServedBy::TDrvNeeds => t.uses.push("drv".to_string()),
        ServedBy::UDrv => u.uses.push("drv".to_string()),
        ServedBy::BothDrv => {
            t.uses.push("drv".to_string());
            u.uses.push("drv".to_string());
        }
        ServedBy::OwnDrv => own.uses.push("drv".to_string()),
        ServedBy::NoDrv => {}
        ServedBy::TDirect => t.uses.push("blk".to_string()),
    }
    if by != ServedBy::TDirect {
        services.push(drv);
    }
    if bound_to {
        services.push(Service {
            name: "xfer".to_string(),
            policy: false,
            side: Side {
                needs: names(&["transfer-engine"]),
                ..Side::default()
            },
        });
        t.uses.push("xfer".to_string());
    }
    let blocks = vec![
        timer_block("timer.d", 4, true),
        source_block("dev.s", Separation::Stated(20), ""),
        Block {
            name: "uart.c".to_string(),
            offers: names(&[
                "(observable-output true)",
                "(reachable-at-privilege machine)",
            ]),
            ..Block::default()
        },
        Block {
            name: "blk".to_string(),
            offers: names(&["(transfer-engine true)", "(reachable-at-privilege machine)"]),
            side: Side {
                needs: requires.needs(),
                ..Side::default()
            },
            ..Block::default()
        },
    ];
    let platform = Block {
        name: "soc".to_string(),
        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
        side: Side {
            uses: names(&["timer.d", "dev.s", "uart.c"]),
            ..Side::default()
        },
        ..Block::default()
    };
    let tasks = vec![
        Task {
            name: "t".to_string(),
            side: t,
            ..Task::default()
        },
        Task {
            name: "u".to_string(),
            release: if u_sporadic {
                Release::MinSeparation(Dur::Ms(20))
            } else {
                Release::Period(Dur::Ms(20))
            },
            priority: 2,
            side: u,
            ..Task::default()
        },
    ];
    Case {
        label: format!(
            "blk:{requires:?} named {by:?}; its offer bound to {}; u {}",
            if bound_to { "by xfer" } else { "by nothing" },
            if u_sporadic { "sporadic" } else { "periodic" }
        ),
        description: Description {
            blocks,
            platforms: vec![platform],
            services,
            tasks,
            own,
            ..Description::default()
        },
    }
}

/// **The naming-system refusal's order** (§1; round 13, R13-R1: "By the namer's text, then the system's"): the
/// lowered system `sys` with tasks `t`, first in runtime order, and `a`; the systems `zz` and `yy`, declared in that
/// order; task `a`, task `t`, the service `m.s` that `t` uses and `sys`'s own level each naming nothing, `yy`, `zz`
/// or both; every name written by `uses` or every one by `needs`; and `zz`'s own level naming `yy` or not. So the
/// namers' texts — `(task sys a)`, `(task sys t)`, `m.s`, `sys`, and `zz` once it is reached — are unlike their
/// runtime and declaration orders, and one namer can name two systems.
fn naming_order() -> Vec<Case> {
    const SYSTEMS: [&[&str]; 4] = [&[], &["yy"], &["zz"], &["zz", "yy"]];
    let mut cases = Vec::new();
    for a in SYSTEMS {
        for t in SYSTEMS {
            for m in SYSTEMS {
                for own in SYSTEMS {
                    for by_needs in [false, true] {
                        for zz_names_yy in [false, true] {
                            let side = |base: &[&str], named: &[&str]| {
                                named_in(
                                    Side {
                                        uses: names(base),
                                        ..Side::default()
                                    },
                                    named,
                                    by_needs,
                                )
                            };
                            let system = |name: &str, task: &str, own: Side| System {
                                name: name.to_string(),
                                tasks: vec![Task {
                                    name: task.to_string(),
                                    release: Release::Period(Dur::Ms(20)),
                                    ..Task::default()
                                }],
                                own,
                                platform: "soc".to_string(),
                            };
                            let zz_own = if zz_names_yy {
                                side(&[], &["yy"])
                            } else {
                                Side::default()
                            };
                            cases.push(Case {
                                label: format!(
                                    "a>{a:?} t>{t:?} m.s>{m:?} sys>{own:?} by {} zz>yy={zz_names_yy}",
                                    if by_needs { "needs" } else { "uses" }
                                ),
                                description: Description {
                                    blocks: vec![Block {
                                        name: "uart.c".to_string(),
                                        offers: names(&[
                                            "(observable-output true)",
                                            "(reachable-at-privilege machine)",
                                        ]),
                                        ..Block::default()
                                    }],
                                    services: vec![Service {
                                        name: "m.s".to_string(),
                                        policy: false,
                                        side: side(&[], m),
                                    }],
                                    tasks: vec![
                                        Task {
                                            name: "t".to_string(),
                                            priority: 1,
                                            side: side(&["m.s"], t),
                                            ..Task::default()
                                        },
                                        Task {
                                            name: "a".to_string(),
                                            release: Release::Period(Dur::Ms(20)),
                                            priority: 2,
                                            side: side(&[], a),
                                            ..Task::default()
                                        },
                                    ],
                                    own: side(&[], own),
                                    others: vec![
                                        system("zz", "z", zz_own),
                                        system("yy", "y", Side::default()),
                                    ],
                                    others_first: true,
                                    ..Description::default()
                                },
                            });
                        }
                    }
                }
            }
        }
    }
    cases
}

/// `side` naming `declarations` too, by `needs` or by `uses`.
fn named_in(side: Side, declarations: &[&str], by_needs: bool) -> Side {
    named(side, names(declarations), by_needs)
}

// =====================================================================================================================
// The reading: docs/decisions/decision_runtime-plan.md, §2 and §3, transcribed rule by rule
// =====================================================================================================================

/// The facts a requester bound to takes one of the provider's channels: those
/// `docs/semantics/vocabulary/vocabulary.eadl` marks `(use consumed)` — a timer route's facts (§2.9: "A channel taken
/// on a consumed fact makes a timer route").
const CONSUMED: [&str; 2] = ["absolute-deadline", "relative-delay"];

/// The interrupt fact a source's requester is bound for (§2.6: "for the interrupt fact where the route is a source").
const INTERRUPT_FACT: &str = "interrupt-source";

/// What the reading takes of an adapter kind, transcribed from `docs/semantics/adapters/adapters.eadl` and held to the
/// shipped catalog by `the_reading_transcribes_the_shipped_vocabulary_and_catalog`.
struct AdapterKind {
    name: &'static str,
    /// The facts it offers `(obliged)`, in offer order (§2.4).
    obliged: &'static [&'static str],
    /// Its obligations, in written order (§2.4).
    obligations: &'static [&'static str],
    /// Its costs, in written order (§2.4).
    costs: &'static [&'static str],
    /// Whether its cost interrupt runs its requester's code (§2.6: "the plan reads `expiry`, a delivery to its
    /// requester, as running it, and `wake`, an epoch advance no requester observes but by reading, as running the
    /// instance's alone").
    runs_requester: bool,
}

const KINDS: [AdapterKind; 2] = [
    AdapterKind {
        name: "deadline-over-delay",
        obliged: &["delivery-bound"],
        obligations: &["delivery", "chain"],
        costs: &["expiry"],
        runs_requester: true,
    },
    AdapterKind {
        name: "epoch-extender",
        obliged: &[],
        obligations: &["update", "coherent"],
        costs: &["wake"],
        runs_requester: false,
    },
];

/// The facts owed for each task to the application (§2.10's first row).
const TASK_FACTS: [&str; 11] = [
    "entry-point",
    "execution-bound",
    "execution-category",
    "figures-under-preemption",
    "masked-category",
    "masked-runs",
    "masks-only-through-api",
    "os-calls",
    "stack-bound",
    "stack-category",
    "task-facts",
];

/// The facts owed for each source to the realization (§2.10).
const SOURCE_FACTS: [&str; 5] = [
    "service-cost",
    "acknowledge-point",
    "deferred-work",
    "no-application-code",
    "one-request-per-arrival",
];

/// The facts owed for each timer route to the realization (§2.10).
const TIMER_ROUTE_FACTS: [&str; 2] = ["service-cost", "arrival-bound"];

/// An instance's text: `(adapter KIND OVER)` or `(adapter KIND OVER REQUESTER)` (§2.4).
fn is_instance(name: &str) -> bool {
    name.starts_with("(adapter ")
}

fn kind_of(instance: &str) -> &'static AdapterKind {
    let kind = instance["(adapter ".len()..]
        .split(' ')
        .next()
        .expect("a kind");
    KINDS
        .iter()
        .find(|k| k.name == kind)
        .unwrap_or_else(|| panic!("the reading knows no adapter kind `{kind}`"))
}

/// §3: "A list with no members is written with its head alone".
fn list<I, S>(head: &str, items: I) -> String
where
    I: IntoIterator<Item = S>,
    S: AsRef<str>,
{
    let mut out = format!("({head}");
    for item in items {
        out.push(' ');
        out.push_str(item.as_ref());
    }
    out.push(')');
    out
}

/// One `(bind REQ FACT PROVIDER)` line of the plan, its three tokens as written (§2.3).
#[derive(Debug, Clone, PartialEq, Eq)]
struct Bind {
    requester: String,
    fact: String,
    provider: String,
}

/// One route as `Joint::check` reports it, "one for each provider and kind" (§2.5).
#[derive(Debug, Clone)]
struct Route {
    /// Its interrupt: `(source NAME)` or `(timer-route NAME)` (§2).
    item: String,
    provider: String,
    source: bool,
    /// "the requesters the route names" (§2.5).
    requesters: BTreeSet<String>,
    /// `(INSTANCE NAME)`, each cost of an instance that programs this provider (§2.5).
    costs: BTreeSet<String>,
}

/// What a requester reaches (§2): "A requester **reaches** what it names by `uses` or `needs`, and what it is bound
/// to as its provider, at any depth".
fn reach(edges: &BTreeMap<String, BTreeSet<String>>, from: &str) -> BTreeSet<String> {
    let mut reached = BTreeSet::new();
    let mut todo: Vec<&String> = edges.get(from).into_iter().flatten().collect();
    while let Some(at) = todo.pop() {
        if reached.insert(at.clone()) {
            todo.extend(edges.get(at).into_iter().flatten());
        }
    }
    reached
}

/// Whether comparing `a` with `b` by cross-multiplication in signed 128 bits would overflow: the reading compares
/// exactly, and counts the systems that need it.
fn past_128_bits(a: Ratio, b: Ratio) -> bool {
    let max = u128::try_from(i128::MAX).expect("positive");
    [(a.n, b.d), (b.n, a.d)]
        .iter()
        .any(|(x, y)| x.checked_mul(*y).is_none_or(|p| p > max))
}

/// Model §8 rule 7: "A system's **hardware** is the blocks and platforms its `platform` clause, its own `uses` and
/// `needs` or its tasks' name, and every block or platform those blocks and platforms name in turn through `uses` or
/// `needs` ... at any depth. A declaration of another kind ends the path: a block that only a service names is not
/// hardware."
fn system_hardware(description: &Description) -> BTreeSet<String> {
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let mut todo: Vec<String> = std::iter::once(description.platform.clone())
        .chain(description.own.uses.iter().cloned())
        .chain(description.own.needs.iter().cloned())
        .chain(
            description
                .tasks
                .iter()
                .flat_map(|t| t.side.uses.iter().chain(&t.side.needs).cloned()),
        )
        .collect();
    let mut out = BTreeSet::new();
    while let Some(at) = todo.pop() {
        if let Some(block) = hardware.get(&at) {
            if out.insert(at.clone()) {
                todo.extend(
                    block
                        .side
                        .uses
                        .iter()
                        .chain(&block.side.needs)
                        .filter(|n| hardware.contains_key(n.as_str()))
                        .cloned(),
                );
            }
        }
    }
    out
}

/// The blocks and platforms the system **serves** (§2: "a block or a platform the system serves that is bound for
/// requirements of its own"): its hardware (rule 7), and each block or platform among "a declaration its closure
/// reaches through `uses` and `needs`" from "its own level, one of its tasks" (model §8 rule 7) — so a block only a
/// service names is served, though it is no hardware (round 13, R13-R2: the reading had taken rule 7's "hardware"
/// where §2 says "serves"). A system named is refused before any plan (§1), so the closure stops at one.
fn served_hardware(description: &Description) -> BTreeSet<String> {
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let mut out = system_hardware(description);
    let mut seen = BTreeSet::new();
    let mut todo: Vec<Side> = description
        .tasks
        .iter()
        .map(|t| t.side.clone())
        .chain(std::iter::once(description.own.clone()))
        .collect();
    while let Some(side) = todo.pop() {
        for named in side.uses.iter().chain(&side.needs) {
            if !seen.insert(named.clone()) {
                continue;
            }
            if let Some(service) = description.services.iter().find(|s| &s.name == named) {
                todo.push(service.side.clone());
            } else if let Some(block) = hardware.get(named) {
                out.insert(named.clone());
                todo.push(block.side.clone());
            }
        }
    }
    out
}

/// The record's reading of one system: every line of the kinds it derives.
struct Reading {
    lines: Vec<String>,
    /// The classes of the record's rules this system exercises, so that a universe's reach is counted.
    classes: BTreeSet<&'static str>,
    /// Where the record's bindings contradict what the reading derives from the description itself.
    findings: Vec<String>,
}

/// The plan `docs/decisions/decision_runtime-plan.md` gives `description` with the assignment `binds`, for the lines
/// the reading derives.
fn read_the_record(description: &Description, binds: &[Bind]) -> Reading {
    let system = description.system.as_str();
    let task_text = |task: &Task| format!("(task {system} {})", task.name);
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let platforms: BTreeSet<String> = description
        .platforms()
        .into_iter()
        .map(|p| p.name)
        .collect();
    let declared_services: BTreeSet<&str> = description
        .services
        .iter()
        .map(|s| s.name.as_str())
        .collect();
    let systems: BTreeSet<String> = description.systems().into_iter().map(|s| s.name).collect();
    let declared = |name: &str| {
        declared_services.contains(name) || hardware.contains_key(name) || systems.contains(name)
    };
    let mut classes = BTreeSet::new();
    let mut findings = Vec::new();

    // §2: a requester reaches what it names by `uses` or `needs` — a declaration, never a vocabulary fact — and what
    // it is bound to as its provider, at any depth, "but for a block's or a platform's `uses` or `needs`, which names
    // only hardware (§1) and is never a call — its `uses` composition (rule 1; rule 5's "`uses` alone is composition"),
    // its `needs` a block it brings into the system's hardware (rule 7): hardware reaches only what its own requirements
    // are bound to". So a block's or a platform's names make no edge here, whatever they name.
    let mut edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut name = |who: String, side: &Side| {
        for named in side.uses.iter().chain(&side.needs) {
            if declared(named) {
                edges.entry(who.clone()).or_default().insert(named.clone());
            }
        }
    };
    for service in &description.services {
        name(service.name.clone(), &service.side);
    }
    for task in &description.tasks {
        name(task_text(task), &task.side);
    }
    name(system.to_string(), &description.own);
    for block in hardware.values() {
        // §1: a block's or a platform's `uses` or `needs` "naming a declaration that is neither a block nor a
        // platform" is refused before any plan.
        assert!(
            block
                .side
                .uses
                .iter()
                .chain(&block.side.needs)
                .all(|n| !declared(n) || hardware.contains_key(n.as_str())),
            "`{}` names what is not hardware, which §1 refuses before any plan",
            block.name
        );
        if block
            .side
            .needs
            .iter()
            .any(|n| hardware.contains_key(n.as_str()))
        {
            classes.insert("hardware: a block's or a platform's `needs` naming a block");
        }
    }
    for bind in binds {
        edges
            .entry(bind.requester.clone())
            .or_default()
            .insert(bind.provider.clone());
    }

    // Model §8 rule 7's hardware, and §2's hardware the system serves: its hardware and what its closure reaches.
    let system_hardware = system_hardware(description);
    let served_hardware = served_hardware(description);

    // §2: "A **node** below is a node of the joint rules' initialization graph (§2.7): a provider an active binding
    // names (a block, a platform or an adapter's instance), a block or a platform the system serves that is bound for
    // requirements of its own — a requester like any other ... — a service, or a policy". A task and the system's own
    // level are requesters but never nodes. A service is the system's where the system or a task of it reaches it.
    let mut served = reach(&edges, system);
    for task in &description.tasks {
        served.extend(reach(&edges, &task_text(task)));
    }
    let mut nodes: BTreeSet<String> = binds.iter().map(|b| b.provider.clone()).collect();
    for bind in binds {
        if hardware.contains_key(&bind.requester) {
            if served_hardware.contains(&bind.requester) {
                nodes.insert(bind.requester.clone());
                if !system_hardware.contains(&bind.requester) {
                    classes.insert(
                        "served: a block a service names, no hardware, bound for requirements of its own",
                    );
                }
            } else {
                findings.push(format!(
                    "`{}` is bound for requirements of its own, and is no block or platform `{system}` serves (§2; \
                     model §8 rule 7)",
                    bind.requester
                ));
            }
        }
    }
    for service in &description.services {
        if served.contains(&service.name) {
            nodes.insert(service.name.clone());
        }
    }
    // Another system's requesters are none of this one's (model §8 rule 7; §1's refusal scoped to "the requesters the
    // system serves", round 12's R12-R1).
    let names_a_system = |side: &Side| {
        side.uses
            .iter()
            .chain(&side.needs)
            .any(|n| systems.contains(n))
    };
    if description
        .others
        .iter()
        .any(|o| names_a_system(&o.own) || o.tasks.iter().any(|t| names_a_system(&t.side)))
        || description
            .services
            .iter()
            .any(|s| !served.contains(&s.name) && names_a_system(&s.side))
    {
        classes.insert("naming: a system another system's requester names, outside this one's");
    }
    if description.others.iter().any(|o| {
        o.tasks.iter().any(|t| {
            t.side
                .uses
                .iter()
                .chain(&t.side.needs)
                .any(|n| declared_services.contains(n.as_str()) && nodes.contains(n))
        })
    }) {
        classes.insert("second system: a service another system's task also uses");
    }
    if hardware
        .iter()
        .any(|(name, block)| !block.side.needs.is_empty() && !served_hardware.contains(name))
    {
        classes.insert("second system: requiring hardware the lowered system does not serve");
    }
    for node in nodes.iter().filter(|n| hardware.contains_key(n.as_str())) {
        if binds.iter().any(|b| &b.requester == node) {
            classes.insert(if platforms.contains(node) {
                "hardware: a platform bound for requirements of its own"
            } else {
                "hardware: a block bound for requirements of its own"
            });
            classes.insert(if binds.iter().any(|b| &b.provider == node) {
                "hardware: requiring, and bound to by a requester"
            } else {
                "hardware: requiring, and bound to by nothing"
            });
        }
    }

    // §2.5: "each route `Joint::check` reports, one for each provider and kind (model §8 rule 8; `Obligation::Route`),
    // its interrupt a timer route or a source, with the requesters the route names". A requester bound to a provider
    // for `interrupt-source` relies on its source; one bound for a consumed fact, on its alarm. An instance is no route
    // of its own: the route is its provider's, and names the requesters bound to the instance (the worked epoch
    // extender: "the route names the requester bound to the instance, `time.monotonic`").
    let relying = |requester: &str| -> BTreeSet<String> {
        if is_instance(requester) {
            binds
                .iter()
                .filter(|b| b.provider == requester)
                .map(|b| b.requester.clone())
                .collect()
        } else {
            BTreeSet::from([requester.to_string()])
        }
    };
    let mut routes: BTreeMap<String, Route> = BTreeMap::new();
    for bind in binds.iter().filter(|b| !is_instance(&b.provider)) {
        let source = bind.fact == INTERRUPT_FACT;
        let timer = CONSUMED.contains(&bind.fact.as_str());
        if !source && !timer {
            continue;
        }
        let item = if source {
            format!("(source {})", bind.provider)
        } else {
            format!("(timer-route {})", bind.provider)
        };
        let route = routes.entry(item.clone()).or_insert_with(|| Route {
            item,
            provider: bind.provider.clone(),
            source,
            requesters: BTreeSet::new(),
            costs: BTreeSet::new(),
        });
        route.requesters.extend(relying(&bind.requester));
        // §2.5: "Each cost is written on that route's line, with the instance that takes it. A shared instance's cost
        // counts once".
        if timer && is_instance(&bind.requester) {
            for cost in kind_of(&bind.requester).costs {
                route.costs.insert(format!("({} {cost})", bind.requester));
            }
        }
    }
    let separation_of = |provider: &str| -> Sep {
        hardware
            .get(provider)
            .and_then(|b| b.separation)
            .unwrap_or_else(|| panic!("`{provider}` is a source block of the description"))
            .read()
    };
    let stated_of = |route: &Route| -> Option<Ratio> {
        match separation_of(&route.provider) {
            Sep::Stated(time) => Some(time),
            Sep::Unstated | Sep::Absent => None,
            Sep::Past => panic!("a separation past the arithmetic is refused before any plan (§1)"),
        }
    };

    // §2.5, the order among routes: "Each route gets a distinct **rank**, rank `1` taken first": the sources with a
    // stated separation, ascending by it, ties broken on their interrupt's text; then the sources whose separation is
    // `unknown` or `absent`, by their interrupt's text; then the timer routes, by their interrupt's text.
    let rank_key = |route: &Route| -> (u8, Ratio, String) {
        if route.source {
            match stated_of(route) {
                Some(time) => (0, time, route.item.clone()),
                None => (1, Ratio::ZERO, route.item.clone()),
            }
        } else {
            (2, Ratio::ZERO, route.item.clone())
        }
    };
    let mut ranked: Vec<&Route> = routes.values().collect();
    ranked.sort_by_key(|route| rank_key(route));
    let stated: Vec<Ratio> = routes
        .values()
        .filter(|r| r.source)
        .filter_map(stated_of)
        .collect();
    for (i, a) in stated.iter().enumerate() {
        for b in &stated[i + 1..] {
            if a == b {
                classes.insert("rank: two stated separations tied");
            }
            if past_128_bits(*a, *b) {
                classes.insert("exact: two separations compared past 128 bits");
            }
        }
    }
    let unstated = routes.values().any(|r| r.source && stated_of(r).is_none());
    if !stated.is_empty() && unstated && routes.values().any(|r| !r.source) {
        classes.insert("rank: every class in one system");
    }

    let mut lines = Vec::new();

    // §2.4 and §3: `(instance INSTANCE OBLIGED … (obligations NAME …) (costs NAME …))`, by text; each fact the kind
    // offers `obliged` written with "its value, the requirement's value at that exclusive instance", `(obliged FACT
    // DUR)`, or, "where the requirement writes no bound, the fact alone", `(obliged FACT)`. The value is "the tightest
    // bound its requester writes on that fact" (the adapters record §1, item 5): here the least `at-most` written by
    // the instance's one requester, the last item of its `(adapter KIND OVER REQUESTER)` text.
    let instances: BTreeSet<String> = binds
        .iter()
        .flat_map(|b| [&b.requester, &b.provider])
        .filter(|n| is_instance(n))
        .cloned()
        .collect();
    let side_of = |requester: &str| -> Option<Side> {
        if requester == system {
            return Some(description.own.clone());
        }
        description
            .tasks
            .iter()
            .find(|t| task_text(t) == requester)
            .map(|t| t.side.clone())
            .or_else(|| {
                description
                    .services
                    .iter()
                    .find(|s| s.name == requester)
                    .map(|s| s.side.clone())
            })
            .or_else(|| hardware.get(requester).map(|b| b.side.clone()))
    };
    for instance in &instances {
        let kind = kind_of(instance);
        let mut line = format!("(instance {instance}");
        let instance_form = form(instance);
        let requester = items(&instance_form).get(3).map(text_of);
        for fact in kind.obliged {
            let bound = requester.as_deref().and_then(side_of).and_then(|side| {
                side.at_most
                    .iter()
                    .filter(|(f, _)| f == fact)
                    .map(|(_, bound)| {
                        bound
                            .seconds()
                            .expect("a bound past the arithmetic is refused before any plan (§1)")
                    })
                    .min()
            });
            match bound {
                Some(time) => {
                    classes.insert("instance: an obliged fact with its bound");
                    if time.d > u128::from(u64::MAX) {
                        classes.insert("exact: an obliged bound at the arithmetic's edge");
                    }
                    line.push_str(&format!(" (obliged {fact} {})", time.text()));
                }
                None => {
                    classes.insert("instance: an obliged fact bare");
                    line.push_str(&format!(" (obliged {fact})"));
                }
            }
        }
        line.push_str(&format!(
            " {} {})",
            list("obligations", kind.obligations),
            list("costs", kind.costs)
        ));
        lines.push(line);
    }

    // §2.6, execution contexts. For a route `I` at provider `X`, the code its interrupt runs: `X` itself; a node the
    // route names that is bound to `X` itself, for the interrupt fact where the route is a source, or for a consumed
    // fact where it is a timer route; for a timer route at `X`, an instance bound to `X` for a consumed fact; a node
    // the route names through such an instance, when the instance's kind runs its requester's code in that interrupt;
    // and every node one of these reaches. "A task or the system's own level that a route names adds no code".
    let mut runs: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for route in routes.values() {
        let mut base = BTreeSet::from([route.provider.clone()]);
        for bind in binds.iter().filter(|b| b.provider == route.provider) {
            let for_this_kind = if route.source {
                bind.fact == INTERRUPT_FACT
            } else {
                CONSUMED.contains(&bind.fact.as_str())
            };
            if !for_this_kind {
                continue;
            }
            if is_instance(&bind.requester) {
                base.insert(bind.requester.clone());
                if kind_of(&bind.requester).runs_requester {
                    classes.insert("context: an expiry running its requester");
                    for through in binds.iter().filter(|b| b.provider == bind.requester) {
                        if nodes.contains(&through.requester)
                            && route.requesters.contains(&through.requester)
                        {
                            base.insert(through.requester.clone());
                            if hardware.contains_key(&through.requester) {
                                classes.insert("hardware: an expiry running its code");
                            }
                        }
                    }
                } else {
                    classes.insert("context: a wake running the instance alone");
                }
            } else if nodes.contains(&bind.requester) && route.requesters.contains(&bind.requester)
            {
                base.insert(bind.requester.clone());
                if hardware.contains_key(&bind.requester) {
                    classes.insert(if route.source {
                        "hardware: a source's handler"
                    } else {
                        "hardware: an alarm it programs itself"
                    });
                }
            }
        }
        let mut all = base.clone();
        for node in &base {
            all.extend(
                reach(&edges, node)
                    .into_iter()
                    .filter(|n| nodes.contains(n)),
            );
        }
        if all.iter().any(|n| {
            hardware.contains_key(n)
                && !base.contains(n)
                && binds.iter().any(|b| &b.requester == n)
                && binds
                    .iter()
                    .any(|b| &b.provider == n && base.contains(&b.requester))
        }) {
            classes.insert("hardware: in an interrupt through a binding to it");
        }
        runs.insert(route.item.clone(), all);
    }
    // §2.6: "`job` for a task, and for every node a task reaches. ... a task's line is `job` alone".
    let mut in_a_job = BTreeSet::new();
    for task in &description.tasks {
        in_a_job.extend(
            reach(&edges, &task_text(task))
                .into_iter()
                .filter(|n| nodes.contains(n)),
        );
    }
    // Hardware's names are no call (§2): what a block or a platform composes by `uses`, or names by `needs`, is run by
    // none of the contexts it runs in, but where something else puts it there.
    for (name, block) in &hardware {
        if !nodes.contains(name) {
            continue;
        }
        let composed = || block.side.uses.iter().filter(|n| nodes.contains(*n));
        if in_a_job.contains(name) {
            if platforms.contains(name) {
                classes.insert("composition: a platform a task reaches through a binding");
            }
            if composed().any(|n| !in_a_job.contains(n)) {
                classes.insert("composition: what a node in a job composes is in none");
            }
            if block
                .side
                .needs
                .iter()
                .any(|n| nodes.contains(n) && !in_a_job.contains(n))
            {
                classes.insert("hardware: what a node in a job names by `needs` is in none");
            }
        }
        for run in runs.values().filter(|run| run.contains(name)) {
            if composed().any(|n| !run.contains(n)) {
                classes
                    .insert("composition: what a node in an interrupt composes is not run there");
            }
        }
    }

    // §2.2, what releases each task: `timer` for a periodic task; for a task with a minimum separation, the external
    // source whose route names the task or a requester it reaches, `(source NAME every)` "when the source's stated
    // separation is at least the task's"; `unknown` otherwise — no source's route reaching it, several, a source
    // stating no separation or declaring it absent, or one shorter than the task's. "A timer route releases no task."
    let mut released_by = BTreeMap::new();
    for task in &description.tasks {
        let trigger = match task.release {
            Release::Period(_) => "timer".to_string(),
            Release::MinSeparation(own) => {
                let own = own
                    .seconds()
                    .expect("a separation past the arithmetic is refused before any plan (§1)");
                let mut reached = reach(&edges, &task_text(task));
                reached.insert(task_text(task));
                let sources: Vec<&Route> = routes
                    .values()
                    .filter(|r| r.source && r.requesters.iter().any(|q| reached.contains(q)))
                    .collect();
                let (trigger, class) = match sources.as_slice() {
                    [one] => {
                        if one.requesters.iter().any(|q| hardware.contains_key(q)) {
                            classes.insert("trigger: a source a block or a platform handles");
                        }
                        match separation_of(&one.provider) {
                            Sep::Stated(time) => {
                                if past_128_bits(time, own) {
                                    classes.insert("exact: a source's and a task's separations compared past 128 bits");
                                }
                                if time >= own {
                                    (
                                        format!("(source {} every)", one.provider),
                                        if time == own {
                                            "trigger: every, equal separations"
                                        } else {
                                            "trigger: every, a longer separation"
                                        },
                                    )
                                } else {
                                    (
                                        "unknown".to_string(),
                                        "trigger: unknown, a shorter separation",
                                    )
                                }
                            }
                            Sep::Unstated => (
                                "unknown".to_string(),
                                "trigger: unknown, no separation stated",
                            ),
                            Sep::Absent => (
                                "unknown".to_string(),
                                "trigger: unknown, the separation absent",
                            ),
                            Sep::Past => panic!("refused before any plan (§1)"),
                        }
                    }
                    [] => (
                        "unknown".to_string(),
                        "trigger: unknown, no source reaching it",
                    ),
                    _ => ("unknown".to_string(), "trigger: unknown, several sources"),
                };
                classes.insert(class);
                trigger
            }
        };
        released_by.insert(task_text(task), trigger);
    }

    // §2.2 and §3: `(task INDEX (task SYSTEM NAME) (rank INT) RELEASE (deadline DUR) REFERENCE (jitter DUR) (overrun
    // POLICY) (released-by TRIGGER) (uses NAME …))`, by index. The index is "the number of tasks of smaller rank", the
    // rank the task's `priority` (`decision_priority-comparison-direction.md`: "`N` is a rank"); the reference
    // `(deadline-from release)` where the description writes it, else `(deadline-from)`; "its release jitter, written
    // as zero where none is stated", and "its overrun policy, `fault` where none is stated", a `POLICY` "written as the
    // description writes it" (§3); and "what it names by `uses` or `needs` that is a service or a policy, each once: a
    // fact or a block it names is no member", by name.
    let mut task_lines: Vec<(usize, String)> = Vec::new();
    for task in &description.tasks {
        let index = description
            .tasks
            .iter()
            .filter(|other| other.priority < task.priority)
            .count();
        let within = |d: Dur| {
            d.seconds()
                .expect("a duration past the arithmetic is refused before any plan (§1)")
        };
        let release = within(task.release.duration());
        let deadline = within(task.deadline());
        let jitter = task.jitter.map_or(Ratio::ZERO, within);
        let members: BTreeSet<&str> = task
            .side
            .uses
            .iter()
            .chain(&task.side.needs)
            .map(String::as_str)
            .filter(|n| declared_services.contains(n))
            .collect();
        let reference = match task.deadline_from {
            Some("release") => {
                classes.insert("task: deadline-from written");
                "(deadline-from release)"
            }
            None => {
                classes.insert("task: deadline-from owed");
                "(deadline-from)"
            }
            Some(other) => panic!("a deadline from `{other}` is refused before any plan (§1)"),
        };
        classes.insert(match task.jitter {
            None => "task: jitter zero where none is stated",
            Some(_) if jitter == Ratio::ZERO => "task: jitter written zero",
            Some(_) if jitter > release => "task: jitter longer than its release's duration",
            Some(_) => "task: jitter written",
        });
        classes.insert(match task.overrun {
            None => "task: overrun fault where none is stated",
            Some("fault") => "task: overrun fault as written",
            Some(_) => "task: overrun another policy as written",
        });
        if [release, deadline, jitter]
            .iter()
            .any(|t| t.d > u128::from(u64::MAX))
        {
            classes.insert("exact: a task's duration at the arithmetic's edge");
        }
        task_lines.push((
            index,
            format!(
                "(task {index} {} (rank {}) ({} {}) (deadline {}) {reference} (jitter {}) (overrun {}) \
                 (released-by {}) {})",
                task_text(task),
                task.priority,
                task.release.clause(),
                release.text(),
                deadline.text(),
                jitter.text(),
                task.overrun.unwrap_or("fault"),
                released_by[&task_text(task)],
                list("uses", members)
            ),
        ));
    }
    task_lines.sort();
    lines.extend(task_lines.into_iter().map(|(_, line)| line));

    // §2.5, the release timer "releases every task whose trigger is `timer`", `releases` items by index (§3); a task's
    // index is "the number of tasks of smaller rank" (§2.2), its rank its priority's place.
    let mut by_index: Vec<&Task> = description.tasks.iter().collect();
    by_index.sort_by_key(|t| t.priority);
    let released: Vec<String> = by_index
        .iter()
        .map(|t| task_text(t))
        .filter(|t| released_by[t] == "timer")
        .collect();
    lines.push(format!("(timer simulator {})", list("releases", &released)));

    // §2.5 and §3: `(route INT ITEM (separation DUR|unknown|absent|programmed) (level LEVEL) (deliverable yes)
    // (requesters REQ …) (costs (INSTANCE NAME) …))`, by rank. A source's separation is "the duration the provider
    // states", `absent` "where it declares the fact absent", `unknown` "where it states none"; a timer route's
    // `programmed`. The level is the profile's caller level, `machine` for `rt-static-up-v1` (§1).
    for (index, route) in ranked.iter().enumerate() {
        let separation = if route.source {
            match separation_of(&route.provider) {
                Sep::Stated(time) => {
                    if time.d > u128::from(u64::MAX) {
                        classes.insert("exact: a separation at the arithmetic's edge");
                    }
                    time.text()
                }
                Sep::Unstated => "unknown".to_string(),
                Sep::Absent => "absent".to_string(),
                Sep::Past => panic!("refused before any plan (§1)"),
            }
        } else {
            "programmed".to_string()
        };
        lines.push(format!(
            "(route {} {} (separation {separation}) (level machine) (deliverable yes) {} {})",
            index + 1,
            route.item,
            list("requesters", &route.requesters),
            list("costs", &route.costs)
        ));
    }
    // §2.5, the enabled set: `timer`; each route — "`timer`, then the routes by rank" (§3).
    lines.push(list(
        "enabled",
        std::iter::once("timer").chain(ranked.iter().map(|r| r.item.as_str())),
    ));
    // §2.5, the interrupt model: "interrupts do not nest, and every masked section masks every interrupt"; and §2.10,
    // "The plan's last line says whether it is complete, and `/1`'s never is".
    lines.push("(interrupt-model (nesting no) (masking all))".to_string());
    lines.push("(complete no)".to_string());

    // §2.6 and §3: one `(context SUBJECT CONTEXT …)` line for each task and each node, by subject's text; within a line
    // `job`, then each `(interrupt …)` by its text, then `initialisation` — "for every node".
    let mut contexts: Vec<(String, String)> = Vec::new();
    for task in &description.tasks {
        contexts.push((
            task_text(task),
            format!("(context {} job)", task_text(task)),
        ));
    }
    for node in &nodes {
        let mut parts = Vec::new();
        if in_a_job.contains(node) {
            parts.push("job".to_string());
        }
        let mut interrupts: Vec<String> = runs
            .iter()
            .filter(|(_, run)| run.contains(node))
            .map(|(item, _)| format!("(interrupt {item})"))
            .collect();
        interrupts.sort();
        if interrupts.len() >= 2 {
            classes.insert("context: a node in two interrupts");
        }
        parts.extend(interrupts);
        parts.push("initialisation".to_string());
        contexts.push((
            node.clone(),
            format!("(context {node} {})", parts.join(" ")),
        ));
    }
    contexts.sort();
    lines.extend(contexts.into_iter().map(|(_, line)| line));

    // §2.7: the graph is `Joint::initialization`, model §8 rule 12: "An edge runs from a provider to each requester
    // bound to it, except across composition, at any depth, or to itself, and from a service or policy to each that
    // names it by `uses` or `needs`; each carries its reason", between nodes — "the system's own level ... and its
    // tasks ... are none". Composition is a block's or a platform's `uses` alone (rule 1; rule 5: "names it through
    // `uses`, at any depth through blocks and platforms, whose `uses` alone is composition"): a binding whose provider
    // composes its requester, or whose requester composes its provider, makes no edge. §2.7: "an edge across
    // composition, a `uses`, is none either, while one to a block a `needs` names stands (rule 12)". An instance
    // composes nothing, and "the initialization graph keeps its edges, its device before it and it before its
    // requesters" (rule 15). §2.7: "a block's or a platform's `uses` or `needs` giving none", no named edge; one naming
    // anything but hardware is refused (§1).
    let composes = |from: &str| -> BTreeSet<String> {
        let mut reached = BTreeSet::new();
        let mut todo = vec![from.to_string()];
        while let Some(at) = todo.pop() {
            if let Some(block) = hardware.get(&at) {
                for named in block
                    .side
                    .uses
                    .iter()
                    .filter(|n| hardware.contains_key(n.as_str()))
                {
                    if reached.insert(named.clone()) {
                        todo.push(named.clone());
                    }
                }
            }
        }
        reached
    };
    #[derive(Default)]
    struct Reasons {
        facts: BTreeSet<String>,
        named: bool,
    }
    let mut graph: BTreeMap<(String, String), Reasons> = BTreeMap::new();
    for bind in binds {
        if nodes.contains(&bind.requester)
            && nodes.contains(&bind.provider)
            && bind.requester != bind.provider
        {
            if composes(&bind.provider).contains(&bind.requester)
                || composes(&bind.requester).contains(&bind.provider)
            {
                classes.insert("initialisation: no edge across composition");
                continue;
            }
            let by_needs = |a: &str, b: &str| {
                hardware
                    .get(a)
                    .is_some_and(|block| block.side.needs.iter().any(|n| n == b))
            };
            if by_needs(&bind.provider, &bind.requester)
                || by_needs(&bind.requester, &bind.provider)
            {
                classes.insert(
                    "initialisation: an edge across a block's `needs` naming a block stands",
                );
            }
            if hardware.contains_key(&bind.requester) {
                classes.insert("initialisation: an edge to a block or a platform bound for its own requirements");
            }
            graph
                .entry((bind.provider.clone(), bind.requester.clone()))
                .or_default()
                .facts
                .insert(bind.fact.clone());
        }
    }
    for namer in description
        .services
        .iter()
        .filter(|s| nodes.contains(&s.name))
    {
        for named in namer.side.uses.iter().chain(&namer.side.needs) {
            if declared_services.contains(named.as_str())
                && nodes.contains(named)
                && *named != namer.name
            {
                graph
                    .entry((named.clone(), namer.name.clone()))
                    .or_default()
                    .named = true;
                classes.insert("initialisation: a named edge");
            }
        }
    }
    // "one line for each ordered pair of nodes an edge joins, the node the edge orders earlier first ... naming each
    // reason", by "the earlier node's text, then the later's; within a line, `(binding …)` reasons by fact, then
    // `named`" (§3).
    for ((earlier, later), reasons) in &graph {
        let mut because: Vec<String> = reasons
            .facts
            .iter()
            .map(|f| format!("(binding {f})"))
            .collect();
        if reasons.named {
            because.push("named".to_string());
        }
        lines.push(format!(
            "(depends {earlier} {later} {})",
            list("because", because)
        ));
    }
    // "Its steps are its strongly connected components, in a topological order taken by repeatedly choosing, among the
    // components whose predecessors have all been taken, the one whose least member's text is least. A step names its
    // members by their text, in order."
    let after = |node: &str| -> BTreeSet<String> {
        let mut reached = BTreeSet::new();
        let mut todo = vec![node.to_string()];
        while let Some(at) = todo.pop() {
            for (earlier, later) in graph.keys() {
                if *earlier == at && reached.insert(later.clone()) {
                    todo.push(later.clone());
                }
            }
        }
        reached
    };
    let reached: BTreeMap<&String, BTreeSet<String>> =
        nodes.iter().map(|n| (n, after(n))).collect();
    let components: BTreeSet<Vec<String>> = nodes
        .iter()
        .map(|n| {
            nodes
                .iter()
                .filter(|m| *m == n || (reached[n].contains(*m) && reached[*m].contains(n)))
                .cloned()
                .collect()
        })
        .collect();
    let mut taken: BTreeSet<String> = BTreeSet::new();
    let mut step = 0;
    while taken.len() < nodes.len() {
        let ready = components
            .iter()
            .filter(|c| !taken.contains(&c[0]))
            .filter(|c| {
                graph.keys().all(|(earlier, later)| {
                    !c.contains(later) || c.contains(earlier) || taken.contains(earlier)
                })
            })
            .min_by(|a, b| a[0].cmp(&b[0]))
            .expect("a component whose predecessors are all taken");
        if ready.len() >= 2 {
            classes.insert("initialisation: a component of several members");
        }
        step += 1;
        lines.push(format!("(step {step} {})", ready.join(" ")));
        taken.extend(ready.iter().cloned());
    }

    // §2.9: "A node that takes a channel — a service, a policy, or a block or platform with requirements of its own;
    // an instance aside, whose kind's obligations say how it shares — is **multiplexed** where it has two or more
    // **users**: each of the system's tasks that reaches it (§2), directly or through services, policies and bindings;
    // and each interrupt whose context line (§2.6) holds it, but for its own timer route — at the provider whose
    // channel it takes, or that the instance whose channel it takes programs — where nothing else that route runs
    // reaches it". "the system's own level is none".
    let mut multiplexed = BTreeSet::new();
    for node in nodes.iter().filter(|n| !is_instance(n)) {
        let channels: Vec<&Bind> = binds
            .iter()
            .filter(|b| &b.requester == node && CONSUMED.contains(&b.fact.as_str()))
            .collect();
        if channels.is_empty() {
            continue; // it takes no channel
        }
        let mut own_routes = BTreeSet::new();
        for channel in channels {
            if is_instance(&channel.provider) {
                for programmed in binds.iter().filter(|b| {
                    b.requester == channel.provider && CONSUMED.contains(&b.fact.as_str())
                }) {
                    own_routes.insert(format!("(timer-route {})", programmed.provider));
                }
            } else {
                own_routes.insert(format!("(timer-route {})", channel.provider));
            }
        }
        let mut users = BTreeSet::new();
        for task in &description.tasks {
            if reach(&edges, &task_text(task)).contains(node) {
                users.insert(task_text(task));
            }
        }
        for (item, run) in &runs {
            if !run.contains(node) {
                continue;
            }
            if own_routes.contains(item) {
                // "that route's own code delivering to it is no caller, but another node's code there calling it is"
                if run
                    .iter()
                    .any(|n| n != node && reach(&edges, n).contains(node))
                {
                    users.insert(item.clone());
                    classes
                        .insert("multiplex: its own route a user, another node there reaching it");
                } else {
                    classes.insert("multiplex: its own route no user");
                }
            } else {
                users.insert(item.clone());
                classes.insert("multiplex: another route a user");
            }
        }
        if users.len() >= 2 {
            multiplexed.insert(node.clone());
            classes.insert("multiplex: multiplexed");
            if hardware.contains_key(node) {
                classes.insert("multiplex: a block or a platform multiplexed");
            }
            let jobs = description
                .tasks
                .iter()
                .filter(|t| users.contains(&task_text(t)))
                .count();
            if jobs == 0 {
                classes.insert("multiplex: multiplexed by interrupts alone");
            } else if jobs >= 2 {
                classes.insert("multiplex: multiplexed by two jobs");
            } else {
                classes.insert("multiplex: multiplexed by a job and an interrupt");
            }
            if description
                .services
                .iter()
                .any(|s| &s.name == node && s.policy)
            {
                classes.insert("multiplex: a policy multiplexed");
            }
        } else if users.len() == 1 {
            classes.insert("multiplex: a channel with one user");
            if hardware.contains_key(node) {
                classes.insert("multiplex: a block or a platform's channel with one user");
            }
        }
    }

    // §2.9's table and §3: `(obligation KIND SUBJECT (discharged-by LEAF …))`, by text.
    let mut obligations = Vec::new();
    for instance in &instances {
        for obligation in kind_of(instance).obligations {
            obligations.push(format!(
                "(obligation {obligation} {instance} (discharged-by M4.2 M4.8))"
            ));
        }
    }
    for node in &multiplexed {
        obligations.push(format!(
            "(obligation multiplex {node} (discharged-by M4.2 M2.21.2))"
        ));
    }
    for route in routes.values() {
        obligations.push(format!(
            "(obligation route {} (discharged-by M4.2 M4.3))",
            route.item
        ));
    }
    obligations.sort();
    lines.extend(obligations);

    // §2.10's table and §3: `(owed SUBJECT FACT (by OWNER))`, by text.
    let mut owed = Vec::new();
    let mut owe = |subject: &str, fact: &str, owner: &str| {
        owed.push(format!("(owed {subject} {fact} (by {owner}))"));
    };
    for task in &description.tasks {
        for fact in TASK_FACTS {
            owe(&task_text(task), fact, "application");
        }
        if task.deadline_from.is_none() {
            owe(&task_text(task), "deadline-from", "description");
        }
        if released_by[&task_text(task)] == "unknown" {
            owe(&task_text(task), "trigger", "description");
        }
    }
    owe(system, "leaves-interrupt-hardware-alone", "application");
    for node in &nodes {
        owe(node, "contexts", "realization");
        owe(node, "no-wait", "realization");
        if in_a_job.contains(node) {
            owe(node, "call-cost", "realization");
        }
    }
    for instance in &instances {
        let kind = kind_of(instance);
        if !kind.costs.is_empty() {
            owe(instance, "cost-terms", "realization");
        }
        for fact in kind.obliged {
            owe(instance, fact, "realization");
        }
    }
    for node in &multiplexed {
        owe(node, "overhead", "realization");
    }
    for route in routes.values() {
        if route.source {
            if separation_of(&route.provider) == Sep::Unstated {
                owe(&route.item, "separation", "description");
            }
            for fact in SOURCE_FACTS {
                owe(&route.item, fact, "realization");
            }
        } else {
            for fact in TIMER_ROUTE_FACTS {
                owe(&route.item, fact, "realization");
            }
        }
    }
    owed.sort();
    lines.extend(owed);

    Reading {
        lines,
        classes,
        findings,
    }
}

// =====================================================================================================================
// Reading a plan's lines
// =====================================================================================================================

/// A form of a plan's line.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Sx {
    Atom(String),
    List(Vec<Sx>),
}

/// `line`, one form of canonical text (§3).
fn form(line: &str) -> Sx {
    let mut stack: Vec<Vec<Sx>> = vec![Vec::new()];
    let mut atom = String::new();
    for ch in line.chars() {
        if matches!(ch, '(' | ')' | ' ') {
            if !atom.is_empty() {
                stack
                    .last_mut()
                    .expect("open")
                    .push(Sx::Atom(std::mem::take(&mut atom)));
            }
            if ch == '(' {
                stack.push(Vec::new());
            } else if ch == ')' {
                let done = stack.pop().expect("balanced");
                stack.last_mut().expect("balanced").push(Sx::List(done));
            }
        } else {
            atom.push(ch);
        }
    }
    assert!(atom.is_empty() && stack.len() == 1, "one form: {line}");
    stack
        .pop()
        .expect("one")
        .into_iter()
        .next()
        .expect("a form")
}

/// A form's canonical text (§3).
fn text_of(sx: &Sx) -> String {
    match sx {
        Sx::Atom(atom) => atom.clone(),
        Sx::List(items) => list(&text_of(&items[0]), items[1..].iter().map(text_of)),
    }
}

fn items(sx: &Sx) -> &[Sx] {
    match sx {
        Sx::List(items) => items,
        Sx::Atom(atom) => panic!("a list, not `{atom}`"),
    }
}

/// The plan's `bind` lines.
fn binds_of(plan: &str) -> Vec<Bind> {
    plan.lines()
        .filter(|line| line.starts_with("(bind "))
        .map(|line| {
            let form = form(line);
            let items = items(&form);
            assert_eq!(items.len(), 4, "{line}");
            Bind {
                requester: text_of(&items[1]),
                fact: text_of(&items[2]),
                provider: text_of(&items[3]),
            }
        })
        .collect()
}

/// §3's forms, by head, in "The forms come in the order below": the order of a plan's lines across kinds.
const FORMS: [&str; 19] = [
    "format",
    "profile",
    "system",
    "program",
    "assignment",
    "adapters",
    "task",
    "bind",
    "instance",
    "timer",
    "route",
    "enabled",
    "interrupt-model",
    "context",
    "depends",
    "step",
    "obligation",
    "owed",
    "complete",
];

/// A line's form, by its place in [`FORMS`], or `None` where §3 writes no form of its head.
fn form_rank(line: &str) -> Option<usize> {
    let head = line
        .strip_prefix('(')
        .and_then(|rest| rest.split([' ', ')']).next())?;
    FORMS.iter().position(|form| *form == head)
}

/// The kinds of line the reading derives, as the checker reports them: every line of a plan, and `unknown`, the lines
/// of a head §3 does not write, of which the reading expects none (round 13, R13-R3: such a line is a disagreement,
/// never dropped).
const KINDS_OF_LINE: [&str; 16] = [
    "identity",
    "task",
    "bind",
    "context",
    "depends",
    "step",
    "multiplex",
    "route",
    "enabled",
    "timer",
    "instance",
    "obligation",
    "owed",
    "interrupt-model",
    "complete",
    "unknown",
];

/// Which of the kinds `line` is: `multiplex` is the `(obligation multiplex …)` lines and the `overhead` they owe
/// (§2.9); `unknown`, a line of a head §3 does not write.
fn kind_of_line(line: &str) -> &'static str {
    let Some(rank) = form_rank(line).filter(|_| one_form(line)) else {
        return "unknown";
    };
    match FORMS[rank] {
        "format" | "profile" | "system" | "program" | "assignment" | "adapters" => "identity",
        "obligation" if line.starts_with("(obligation multiplex ") => "multiplex",
        // `(owed SUBJECT FACT (by OWNER))`: the fact is the word before its last form.
        "owed" => {
            let fact = line
                .rsplit_once(" (by ")
                .and_then(|(before, _)| before.rsplit(' ').next());
            if fact == Some("overhead") {
                "multiplex"
            } else {
                "owed"
            }
        }
        other => KINDS_OF_LINE
            .iter()
            .copied()
            .find(|kind| *kind == other)
            .expect("each other form a kind of its own"),
    }
}

/// Whether `line` is one balanced form, `(` first and the `)` closing it last (§3: "each line is one form").
fn one_form(line: &str) -> bool {
    let mut depth = 0usize;
    for (at, ch) in line.char_indices() {
        match ch {
            '(' => depth += 1,
            ')' => {
                if depth == 0 {
                    return false;
                }
                depth -= 1;
                if depth == 0 && at + 1 != line.len() {
                    return false;
                }
            }
            _ if depth == 0 => return false,
            _ => {}
        }
    }
    depth == 0 && line.starts_with('(')
}

fn by_kind<'a>(lines: impl IntoIterator<Item = &'a str>) -> BTreeMap<&'static str, Vec<String>> {
    let mut out: BTreeMap<&'static str, Vec<String>> =
        KINDS_OF_LINE.iter().map(|k| (*k, Vec::new())).collect();
    for line in lines {
        out.get_mut(kind_of_line(line))
            .expect("a kind")
            .push(line.to_string());
    }
    out
}

// =====================================================================================================================
// The checker
// =====================================================================================================================

/// What the reading and the plan say of one kind of line, where they differ.
struct Disagreement {
    kind: &'static str,
    expected: Vec<String>,
    written: Vec<String>,
}

/// What judging one plan gave.
struct Judged {
    /// The kinds of line the plan agrees with the reading on.
    agreed: Vec<&'static str>,
    disagreements: Vec<Disagreement>,
    classes: BTreeSet<&'static str>,
}

/// The plan's identity (§2.1, §3): "the format, the profile, the system, the program's digest, the assignment's sha256
/// and the adapter catalog's version" — the profile, the system, the program's digest and the catalog's version as the
/// record writes them, its digest being the program's (the configuration checker's check (1) holds it), and the
/// sha256 of the record's text. Where `elided`, the two digests are written `…`, as the record's worked texts write
/// them.
fn identity(record: &str, elided: bool) -> Vec<String> {
    let line = |head: &str| -> String {
        record
            .lines()
            .find(|l| l.starts_with(&format!("({head} ")))
            .unwrap_or_else(|| panic!("the record's `{head}` line"))
            .to_string()
    };
    let (program, assignment) = if elided {
        (
            "(program \"…\")".to_string(),
            "(assignment \"…\")".to_string(),
        )
    } else {
        (
            line("program"),
            format!("(assignment \"{}\")", Digest::of(record.as_bytes()).hex()),
        )
    };
    vec![
        "(format \"archogen-runtime-plan/1\")".to_string(),
        line("profile"),
        line("system"),
        program,
        assignment,
        line("adapters"),
    ]
}

/// One system's plan judged against the reading, which takes the assignment from `record`, the `/2` record lowered:
/// §2.3's `bind` lines are that record's, "copied as written".
fn judge(description: &Description, record: &str, plan: &str) -> Judged {
    judge_elided(description, record, plan, false)
}

/// [`judge`], the identity's two digests written `…` where `elided`.
fn judge_elided(description: &Description, record: &str, plan: &str, elided: bool) -> Judged {
    let reading = read_the_record(description, &binds_of(record));
    let mut expected = by_kind(reading.lines.iter().map(String::as_str));
    expected.insert("identity", identity(record, elided));
    expected.insert(
        "bind",
        record
            .lines()
            .filter(|line| line.starts_with("(bind "))
            .map(str::to_string)
            .collect(),
    );
    let written = by_kind(plan.lines());
    let mut agreed = Vec::new();
    let mut disagreements = Vec::new();
    if !reading.findings.is_empty() {
        disagreements.push(Disagreement {
            kind: "nodes",
            expected: reading.findings.clone(),
            written: Vec::new(),
        });
    }
    for kind in KINDS_OF_LINE {
        if expected[kind] == written[kind] {
            agreed.push(kind);
        } else {
            disagreements.push(Disagreement {
                kind,
                expected: expected[kind].clone(),
                written: written[kind].clone(),
            });
        }
    }
    // The whole text, every line in its place (round 13, R13-R3): §3's "The forms come in the order below", each form's
    // lines in the order §3 gives within it. Each form's lines but `obligation`'s and `owed`'s are of one kind, already
    // in their order, which a stable sort by form keeps; those two forms' lines, `multiplex`'s among them, are "by their
    // text". A line of a head §3 does not write has no place: the reading expects none, so a plan holding one differs
    // here too.
    let by_text = |line: &str| {
        matches!(
            form_rank(line).map(|r| FORMS[r]),
            Some("obligation" | "owed")
        )
    };
    let mut whole: Vec<String> = expected
        .iter()
        .filter(|(kind, _)| **kind != "unknown")
        .flat_map(|(_, lines)| lines.iter().cloned())
        .collect();
    // `sort_by_cached_key` is stable: it sorts each key with its index.
    whole.sort_by_cached_key(|line| (form_rank(line), by_text(line).then(|| line.clone())));
    // §3: "each line is one form and ends with a single newline, the last line included", "and no blank line": the
    // plan's bytes, not its lines alone.
    let canonical: String = whole.iter().map(|line| format!("{line}\n")).collect();
    let lines: Vec<String> = plan.split('\n').map(str::to_string).collect();
    if plan == canonical {
        agreed.push("whole text");
    } else {
        // The text split at each newline: its last piece empty where it ends with one.
        let wanted: Vec<String> = canonical.split('\n').map(str::to_string).collect();
        let at = lines
            .iter()
            .zip(&wanted)
            .position(|(l, w)| l != w)
            .unwrap_or_else(|| lines.len().min(wanted.len()));
        disagreements.push(Disagreement {
            kind: "whole text",
            expected: std::iter::once(format!("from line {}:", at + 1))
                .chain(wanted.iter().skip(at).take(3).cloned())
                .collect(),
            written: std::iter::once(format!("from line {}:", at + 1))
                .chain(lines.iter().skip(at).take(3).cloned())
                .collect(),
        });
    }
    Judged {
        agreed,
        disagreements,
        classes: reading.classes,
    }
}

/// What the engine did with one system, run as `common::lowered` runs it — the language's check, the search, the
/// `/2` writer and the lowering — with the `archogen-assignment/2` record kept and judged by the configuration checker,
/// `archogen_plan_check::check::plan_file`, as `crates/eadl-resolve/tests/plan.rs`'s `resolved_and_judged` judges one.
enum Flow {
    /// The language's check refused the description: its verdict. Such a description is no input (§1).
    Rejected(String),
    /// The search resolved no plan: its verdict.
    Unresolved(String),
    /// The search resolved one.
    Resolved {
        /// The record the search's writer wrote.
        record: String,
        /// The configuration checker's verdict on it, where it refuses.
        checker: Option<String>,
        /// The plan's text, or the lowering's refusal.
        lowered: Result<String, Refusal>,
    },
}

fn flow(text: &str, system: &str) -> Flow {
    let program = match common::checked(text) {
        Ok(program) => program,
        Err(verdict) => return Flow::Rejected(verdict.to_string()),
    };
    let vocabulary = common::vocabulary();
    let catalog = common::catalog();
    let declarations = &program.root().declarations;
    let decl = declarations
        .iter()
        .find(|f| {
            f.head() == Some("defsystem")
                && f.items().get(1).and_then(Form::as_symbol) == Some(system)
        })
        .expect("the system")
        .clone();
    let policy = Policy {
        adapters: catalog.clone(),
        ..Policy::default()
    };
    let resolution = match search(
        declarations,
        &decl,
        RT_STATIC_UP_V1.caller_level,
        &vocabulary,
        &policy,
        Limits::DEFAULT,
    ) {
        Search::Resolved(resolution) => resolution,
        Search::Refused { verdict, .. } => return Flow::Unresolved(format!("{verdict:?}")),
        Search::Inconclusive { .. } => return Flow::Unresolved("inconclusive".to_string()),
    };
    let record = plan::write(
        &resolution,
        RT_STATIC_UP_V1.id,
        &plan::program_digest(&program),
    );
    let mut sources = SourceMap::new();
    let id = sources
        .add("description.eadl", text.to_owned())
        .expect("small");
    let (_, verdict) = plan_file(
        &mut sources,
        id,
        &DirectoryModules::new(common::root()),
        &RT_STATIC_UP_V1,
        "plan.json",
        &record,
    );
    let checker = verdict.err().map(|refused| {
        let findings: Vec<String> = refused
            .findings
            .iter()
            .map(|f| format!("{}: {}", f.code, f.message))
            .collect();
        format!("{}; {}", refused.status.slug(), findings.join("; "))
    });
    let lowered = lower(&program, &record, &vocabulary, &catalog).map(|plan| plan.text());
    Flow::Resolved {
        record,
        checker,
        lowered,
    }
}

/// What a refusal §1 gives names.
enum Names {
    /// One task, `(task SYSTEM NAME)`: "a bullet over tasks is taken task by task in runtime order", and the first
    /// task it applies to is the one given.
    Task(String),
    /// The naming-system refusal: each requester the system serves that names a system, with the system it names —
    /// "naming each such name, by the namer's text and then the system's" (round 13, R13-R1), texts compared byte by
    /// byte (§2).
    Systems(Vec<(String, String)>),
    /// The refusal of hardware naming what is not hardware: each block or platform whose `uses` or `needs` names a
    /// declaration that is neither, with that operand — "naming each, by the block's or platform's text and then the
    /// operand's".
    Operands(Vec<(String, String)>),
    /// The durations past the arithmetic, "naming each — the tasks' in runtime order, then the sources' by their
    /// interrupt's text, then the bounds by their instance's text", each task's clauses "in the order the bullet
    /// writes them": a task's as `(task SYSTEM NAME) CLAUSE`, a source's as `(source NAME)`, a bound's as `INSTANCE
    /// FACT`.
    Durations(Vec<String>),
    /// The interrupt-code refusal: every receiver, in §1's order — a task's `(task SYSTEM NAME)`, or the system's name
    /// for its own level — with the instances whose `expiry` and the providers whose alarm a receiver would receive.
    Receivers {
        subjects: Vec<String>,
        expiries: Vec<String>,
        alarms: Vec<String>,
    },
}

/// A refusal §1 gives one system.
struct Due {
    code: &'static str,
    names: Names,
    /// The classes of §1's rules it exercises.
    classes: BTreeSet<&'static str>,
}

impl Due {
    /// What it names, for a reproducer.
    fn summary(&self) -> String {
        match &self.names {
            Names::Task(task) => task.clone(),
            Names::Systems(pairs) | Names::Operands(pairs) => pairs
                .iter()
                .map(|(namer, named)| format!("{namer} naming {named}"))
                .collect::<Vec<_>>()
                .join(", "),
            Names::Durations(items) => items.join(", "),
            Names::Receivers { subjects, .. } => subjects.join(", "),
        }
    }
}

/// §1: "a declaration or a task the system serves naming a system by `uses` or `needs`", each with the system it
/// names, "by the namer's text and then the system's". The requesters a system serves are "its own level, one of its
/// tasks, or a declaration its closure reaches through `uses` and `needs`" (model §8 rule 7) — round 12's R12-R1
/// scoped the refusal to them — and its hardware, §2's "a block or a platform the system serves"; a system so reached
/// is served for its own level, its tasks being that system's.
fn systems_named(description: &Description) -> Vec<(String, String)> {
    let systems: BTreeMap<String, System> = description
        .systems()
        .into_iter()
        .map(|s| (s.name.clone(), s))
        .collect();
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let mut todo: Vec<(String, Side)> = description
        .tasks
        .iter()
        .map(|t| {
            (
                format!("(task {} {})", description.system, t.name),
                t.side.clone(),
            )
        })
        .chain(std::iter::once((
            description.system.clone(),
            description.own.clone(),
        )))
        .chain(
            system_hardware(description)
                .into_iter()
                .map(|name| (name.clone(), hardware[&name].side.clone())),
        )
        .collect();
    let mut seen = BTreeSet::new();
    let mut out = BTreeSet::new();
    while let Some((namer, side)) = todo.pop() {
        if !seen.insert(namer.clone()) {
            continue;
        }
        for named in side.uses.iter().chain(&side.needs) {
            if let Some(other) = systems.get(named) {
                out.insert((namer.clone(), named.clone()));
                todo.push((named.clone(), other.own.clone()));
            } else if let Some(service) = description.services.iter().find(|s| &s.name == named) {
                todo.push((named.clone(), service.side.clone()));
            } else if let Some(block) = hardware.get(named) {
                todo.push((named.clone(), block.side.clone()));
            }
        }
    }
    // A set of pairs iterates by its first text, then its second, each compared byte by byte.
    out.into_iter().collect()
}

/// §1: "a block's or a platform's `uses` or `needs` naming a declaration that is neither a block nor a platform",
/// "naming each, by the block's or platform's text and then the operand's". Unlike the bullet before it, scoped to "a
/// declaration or a task the system serves", this one names no scope: every block and platform the description
/// declares is read, another system's hardware among them.
fn hardware_naming_other_kinds(description: &Description) -> Vec<(String, String)> {
    let hardware: BTreeSet<String> = description.hardware().into_iter().map(|b| b.name).collect();
    let others: BTreeSet<String> = description
        .services
        .iter()
        .map(|s| s.name.clone())
        .chain(description.systems().into_iter().map(|s| s.name))
        .filter(|name| !hardware.contains(name))
        .collect();
    let mut out = BTreeSet::new();
    for block in description.hardware() {
        for named in block.side.uses.iter().chain(&block.side.needs) {
            if others.contains(named) {
                out.insert((block.name.clone(), named.clone()));
            }
        }
    }
    out.into_iter().collect()
}

/// §1, the refusals these universes can reach, the first that applies, from the description and the record's own
/// `bind` lines: "They are taken in the order written, this list and then the next, ... and the first that applies is
/// the one given; a bullet over tasks is taken task by task in runtime order, each task's clauses in the order the
/// bullet writes them".
/// - Of the first list, in its order: "a periodic task bound to an interrupt source itself, which states a second
///   release model": `tool-failure`, "naming the task and the clause"; then "a declaration or a task the system
///   serves naming a system by `uses` or `needs`": `tool-failure`, "naming each such name, by the namer's text and
///   then the system's"; then "a block's or a platform's `uses` or `needs` naming a declaration that is neither a block
///   nor a platform": `tool-failure`, "naming each, by the block's or platform's text and then the operand's".
///   (`M1.43`, `M1.46` and `M1.47` have the language refuse them, and "a description the language's check refuses is
///   no input".)
/// - Of the second, `unsupported-profile`, in its order: "a duration the plan would write past the engine's exact
///   arithmetic (§2.2): a task's period, minimum separation, deadline or jitter, a source's separation, or an obliged
///   bound, naming each — the tasks' in runtime order, then the sources' by their interrupt's text, then the bounds by
///   their instance's text"; "a task whose deadline is measured from an event other than its release"; and "a task, or
///   the system's own level, that would receive an interrupt's code: a timer route that names it, bound to its
///   provider itself, or an adapter instance with a cost running its requester's code (`expiry`) whose requester it
///   is, the one an exclusive instance serves or one bound to a shared instance. ... It names every such receiver —
///   each task in runtime order, then the system's own level — with what it would receive, so no order among them
///   decides the message".
///
/// No other bullet applies here: no system has placement memory and each declares a task.
fn refusal_the_record_gives(description: &Description, binds: &[Bind]) -> Option<Due> {
    let system = description.system.as_str();
    let task_text = |task: &Task| format!("(task {system} {})", task.name);
    let mut by_index: Vec<&Task> = description.tasks.iter().collect();
    by_index.sort_by_key(|t| t.priority);
    for task in &by_index {
        if matches!(task.release, Release::Period(_))
            && binds
                .iter()
                .any(|b| b.requester == task_text(task) && b.fact == INTERRUPT_FACT)
        {
            let mut classes = BTreeSet::from(["refusal: a periodic task bound to a source itself"]);
            if !hardware_naming_other_kinds(description).is_empty() {
                classes.insert("periodic: before hardware naming what is not hardware");
            }
            return Some(Due {
                code: "tool-failure",
                names: Names::Task(task_text(task)),
                classes,
            });
        }
    }

    let named = systems_named(description);
    if !named.is_empty() {
        let mut classes = BTreeSet::from(["refusal: a requester naming a system"]);
        for (namer, _) in &named {
            classes.insert(if namer.starts_with("(task ") {
                "naming: a task naming a system"
            } else if namer == system {
                "naming: the system's own level naming a system"
            } else if description.services.iter().any(|s| &s.name == namer) {
                "naming: a service naming a system"
            } else {
                "naming: a named system's own level naming a system"
            });
        }
        if named.len() >= 2 {
            classes.insert("naming: several names");
        }
        if named.windows(2).any(|w| w[0].0 == w[1].0) {
            classes.insert("naming: one namer naming two systems");
        }
        if !hardware_naming_other_kinds(description).is_empty() {
            classes.insert("naming: before hardware naming what is not hardware");
        }
        let hardware_names: BTreeSet<String> =
            description.hardware().into_iter().map(|b| b.name).collect();
        if named
            .iter()
            .any(|(namer, _)| hardware_names.contains(namer))
        {
            classes.insert("naming: a block or a platform naming a system");
        }
        return Some(Due {
            code: "tool-failure",
            names: Names::Systems(named),
            classes,
        });
    }

    // The first list's last bullet, after the naming-system one: hardware naming what is not hardware.
    let operands = hardware_naming_other_kinds(description);
    if !operands.is_empty() {
        let mut classes = BTreeSet::from(["refusal: hardware naming what is not hardware"]);
        let platforms: BTreeSet<String> = description
            .platforms()
            .into_iter()
            .map(|p| p.name)
            .collect();
        let served = served_hardware(description);
        let named_by = |namer: &str, operand: &str, needs: bool| {
            description.hardware().iter().any(|b| {
                b.name == namer
                    && if needs {
                        b.side.needs.iter().any(|n| n == operand)
                    } else {
                        b.side.uses.iter().any(|n| n == operand)
                    }
            })
        };
        for (namer, operand) in &operands {
            let kind = if description
                .services
                .iter()
                .any(|s| &s.name == operand && s.policy)
            {
                "policy"
            } else if description.services.iter().any(|s| &s.name == operand) {
                "service"
            } else {
                "system"
            };
            let by = if named_by(namer, operand, true) {
                "needs"
            } else {
                "uses"
            };
            let who = if platforms.contains(namer) {
                "a platform"
            } else {
                "a block"
            };
            classes.insert(match (who, by, kind) {
                ("a block", "uses", "service") => "operand: a block's `uses` naming a service",
                ("a block", "uses", "policy") => "operand: a block's `uses` naming a policy",
                ("a block", "uses", _) => "operand: a block's `uses` naming a system",
                ("a block", _, "service") => "operand: a block's `needs` naming a service",
                ("a block", _, "policy") => "operand: a block's `needs` naming a policy",
                ("a block", _, _) => "operand: a block's `needs` naming a system",
                (_, "uses", "service") => "operand: a platform's `uses` naming a service",
                (_, "uses", "policy") => "operand: a platform's `uses` naming a policy",
                (_, "uses", _) => "operand: a platform's `uses` naming a system",
                (_, _, "service") => "operand: a platform's `needs` naming a service",
                (_, _, "policy") => "operand: a platform's `needs` naming a policy",
                _ => "operand: a platform's `needs` naming a system",
            });
            if !served.contains(namer) {
                classes.insert("operand: hardware the lowered system does not serve");
            }
            if binds
                .iter()
                .any(|b| &b.requester == operand && &b.provider == namer)
            {
                classes.insert("operand: a block naming the service bound to it");
            }
            let named_elsewhere = description
                .tasks
                .iter()
                .map(|t| &t.side)
                .chain(std::iter::once(&description.own))
                .chain(description.services.iter().map(|s| &s.side))
                .any(|side| side.uses.iter().chain(&side.needs).any(|n| n == operand));
            if kind == "service" && !named_elsewhere {
                classes.insert("operand: a service only hardware names");
            }
        }
        if operands.len() >= 2 {
            classes.insert("operand: several, named in order");
        }
        if operands.windows(2).any(|w| w[0].0 == w[1].0) {
            classes.insert("operand: one namer naming two");
        }
        if operands.windows(2).any(|w| w[0].1 > w[1].1) {
            classes.insert("operand: the namers' order unlike the operands'");
        }
        // The second list's bullets it comes before, where they would apply too.
        if description.tasks.iter().any(|t| {
            t.release.duration().seconds().is_none()
                || t.deadline().seconds().is_none()
                || t.jitter.is_some_and(|j| j.seconds().is_none())
        }) {
            classes.insert("operand: before a duration past the arithmetic");
        }
        if description
            .tasks
            .iter()
            .any(|t| t.deadline_from.is_some_and(|e| e != "release"))
        {
            classes.insert("operand: before a deadline from another event");
        }
        if binds.iter().any(|b| {
            (b.requester == system || by_index.iter().any(|t| task_text(t) == b.requester))
                && if is_instance(&b.provider) {
                    kind_of(&b.provider).runs_requester
                } else {
                    CONSUMED.contains(&b.fact.as_str())
                }
        }) {
            classes.insert("operand: before a receiver of interrupt code");
        }
        return Some(Due {
            code: "tool-failure",
            names: Names::Operands(operands),
            classes,
        });
    }

    // Durations past the arithmetic: each task's, in runtime order; each source's whose route the plan would write;
    // each obliged bound an instance would take on.
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let mut past = Vec::new();
    for task in &by_index {
        if task.release.duration().seconds().is_none() {
            past.push(format!("{} {}", task_text(task), task.release.clause()));
        }
        if task.deadline().seconds().is_none() {
            past.push(format!("{} deadline", task_text(task)));
        }
        if task.jitter.is_some_and(|j| j.seconds().is_none()) {
            past.push(format!("{} jitter", task_text(task)));
        }
    }
    let tasks_past = past.len();
    let sources: BTreeSet<String> = binds
        .iter()
        .filter(|b| b.fact == INTERRUPT_FACT && !is_instance(&b.provider))
        .filter(|b| {
            hardware
                .get(&b.provider)
                .and_then(|block| block.separation)
                .is_some_and(|s| s.read() == Sep::Past)
        })
        .map(|b| format!("(source {})", b.provider))
        .collect();
    let sources_past = sources.len();
    past.extend(sources);
    let mut bounds = BTreeSet::new();
    for instance in binds.iter().map(|b| &b.provider).filter(|p| is_instance(p)) {
        let instance_form = form(instance);
        let Some(requester) = items(&instance_form).get(3).map(text_of) else {
            continue;
        };
        let side = description
            .services
            .iter()
            .find(|s| s.name == requester)
            .map(|s| s.side.clone())
            .or_else(|| {
                description
                    .tasks
                    .iter()
                    .find(|t| task_text(t) == requester)
                    .map(|t| t.side.clone())
            })
            .or_else(|| hardware.get(&requester).map(|b| b.side.clone()));
        for fact in kind_of(instance).obliged {
            if side.as_ref().is_some_and(|side| {
                side.at_most
                    .iter()
                    .any(|(f, bound)| f == fact && bound.seconds().is_none())
            }) {
                bounds.insert(format!("{instance} {fact}"));
            }
        }
    }
    let bounds_past = bounds.len();
    past.extend(bounds);
    if !past.is_empty() {
        let mut classes = BTreeSet::from(["refusal: a duration past the arithmetic"]);
        if tasks_past > 0 {
            classes.insert("past: a task's clause");
        }
        if sources_past > 0 {
            classes.insert("past: a source's separation");
        }
        if bounds_past > 0 {
            classes.insert("past: an obliged bound");
        }
        if [tasks_past, sources_past, bounds_past]
            .iter()
            .filter(|n| **n > 0)
            .count()
            == 3
        {
            classes.insert("past: a task's, a source's and a bound at once");
        }
        if past.len() >= 2 {
            classes.insert("past: several, named in order");
        }
        if description
            .tasks
            .iter()
            .any(|t| t.deadline_from.is_some_and(|e| e != "release"))
        {
            classes.insert("past: before a deadline from another event");
        }
        return Some(Due {
            code: "unsupported-profile",
            names: Names::Durations(past),
            classes,
        });
    }

    for task in &by_index {
        if task.deadline_from.is_some_and(|e| e != "release") {
            return Some(Due {
                code: "unsupported-profile",
                names: Names::Task(task_text(task)),
                classes: BTreeSet::from(["refusal: a deadline measured from another event"]),
            });
        }
    }

    let mut subjects = Vec::new();
    let mut expiries = Vec::new();
    let mut alarms = Vec::new();
    let mut classes = BTreeSet::new();
    let requesters = by_index
        .iter()
        .map(|t| task_text(t))
        .chain(std::iter::once(system.to_string()));
    for requester in requesters {
        let mut receives = false;
        for bind in binds.iter().filter(|b| b.requester == requester) {
            if is_instance(&bind.provider) {
                if kind_of(&bind.provider).runs_requester {
                    receives = true;
                    expiries.push(bind.provider.clone());
                    classes.insert("receivers: an expiry");
                }
            } else if CONSUMED.contains(&bind.fact.as_str()) {
                receives = true;
                alarms.push(bind.provider.clone());
                classes.insert("receivers: an alarm programmed itself");
            }
        }
        if receives {
            subjects.push(requester);
        }
    }
    if subjects.is_empty() {
        return None;
    }
    classes.insert(match subjects.len() {
        1 => "receivers: one",
        2 => "receivers: two",
        _ => "receivers: three",
    });
    let tasks: Vec<&String> = subjects.iter().filter(|s| *s != system).collect();
    if tasks.len() < subjects.len() {
        classes.insert(if tasks.is_empty() {
            "receivers: the system's own level alone"
        } else {
            "receivers: tasks and the system's own level"
        });
    }
    if tasks.windows(2).any(|w| w[0] > w[1]) {
        classes.insert("receivers: runtime order unlike the tasks' text order");
    }
    Some(Due {
        code: "unsupported-profile",
        names: Names::Receivers {
            subjects,
            expiries,
            alarms,
        },
        classes,
    })
}

/// A refusal's message read as forms: its parenthesised forms as trees, every other word an atom; backquotes, quotes,
/// commas and semicolons separate words. Prose is no canonical text, so an unbalanced parenthesis is closed rather
/// than refused.
fn prose(message: &str) -> Vec<Sx> {
    fn flush(atom: &mut String, stack: &mut [Vec<Sx>]) {
        if !atom.is_empty() {
            stack
                .last_mut()
                .expect("open")
                .push(Sx::Atom(std::mem::take(atom)));
        }
    }
    let mut stack: Vec<Vec<Sx>> = vec![Vec::new()];
    let mut atom = String::new();
    for ch in message.chars() {
        match ch {
            '(' => {
                flush(&mut atom, &mut stack);
                stack.push(Vec::new());
            }
            ')' => {
                flush(&mut atom, &mut stack);
                if stack.len() > 1 {
                    let done = stack.pop().expect("open");
                    stack.last_mut().expect("open").push(Sx::List(done));
                }
            }
            c if c.is_whitespace() || matches!(c, '`' | '"' | ',' | ';') => {
                flush(&mut atom, &mut stack);
            }
            c => atom.push(c),
        }
    }
    flush(&mut atom, &mut stack);
    while stack.len() > 1 {
        let done = stack.pop().expect("open");
        stack.last_mut().expect("open").push(Sx::List(done));
    }
    stack.pop().expect("one")
}

/// A form's canonical text, `()` included.
fn prose_text(sx: &Sx) -> String {
    match sx {
        Sx::List(items) if items.is_empty() => "()".to_string(),
        other => text_of(other),
    }
}

/// Every parenthesised form of a refusal's message, at any depth, by its canonical text.
fn forms_in(message: &str) -> BTreeSet<String> {
    fn walk(sx: &Sx, out: &mut BTreeSet<String>) {
        if let Sx::List(items) = sx {
            out.insert(prose_text(sx));
            for item in items {
                walk(item, out);
            }
        }
    }
    let mut out = BTreeSet::new();
    for sx in prose(message) {
        walk(&sx, &mut out);
    }
    out
}

/// Every word of a refusal's message outside an instance's `(adapter …)` text, inside other forms included.
fn words_outside_instances(message: &str) -> BTreeSet<String> {
    fn walk(sx: &Sx, out: &mut BTreeSet<String>) {
        match sx {
            Sx::Atom(atom) => {
                out.insert(atom.clone());
            }
            Sx::List(items) => {
                if items.first() != Some(&Sx::Atom("adapter".to_string())) {
                    for item in items {
                        walk(item, out);
                    }
                }
            }
        }
    }
    let mut out = BTreeSet::new();
    for sx in prose(message) {
        walk(&sx, &mut out);
    }
    out
}

/// The subjects a refusal's message names, each once, in the order it first names them: each `(task SYSTEM NAME)` form,
/// and the system's own level by the system's name as a word of its own — compared whole, never as a substring of
/// another text (round 11, R11-R3), so the `sys` inside `(task sys t)` names no system. An instance's `(adapter …)`
/// text is a name of its own: the requester written inside it is the instance's, no subject of the message.
fn subjects_named(message: &str, system: &str) -> Vec<String> {
    fn word(atom: &str) -> &str {
        let atom = atom
            .strip_suffix("'s")
            .or_else(|| atom.strip_suffix("’s"))
            .unwrap_or(atom);
        atom.trim_end_matches(['.', ':', '!', '?'])
            .trim_matches(['\'', '‘', '’'])
    }
    fn walk(sx: &Sx, system: &str, out: &mut Vec<String>) {
        let mut name = |subject: String| {
            if !out.contains(&subject) {
                out.push(subject);
            }
        };
        match sx {
            Sx::Atom(atom) => {
                if word(atom) == system {
                    name(system.to_string());
                }
            }
            Sx::List(items) => {
                let head = match items.first() {
                    Some(Sx::Atom(head)) => head.as_str(),
                    _ => "",
                };
                if head == "adapter" {
                    return;
                }
                if head == "task"
                    && items.len() == 3
                    && items.iter().all(|i| matches!(i, Sx::Atom(_)))
                {
                    name(prose_text(sx));
                    return;
                }
                for item in items {
                    walk(item, system, out);
                }
            }
        }
    }
    let mut out = Vec::new();
    for sx in prose(message) {
        walk(&sx, system, &mut out);
    }
    out
}

/// The durations a refusal's message names past the arithmetic, in the order it names them: a task's clause as
/// `(task SYSTEM NAME) CLAUSE`, the task the last `(task …)` form named and the clause a form headed by `period`,
/// `min-separation`, `deadline` or `jitter`; a source as its `(source NAME)` form; and a bound as its instance's
/// `(adapter …)` text, with the obliged fact the message names after it.
fn durations_named(message: &str) -> Vec<String> {
    const CLAUSES: [&str; 4] = ["period", "min-separation", "deadline", "jitter"];
    let obliged: BTreeSet<&str> = KINDS
        .iter()
        .flat_map(|k| k.obliged.iter().copied())
        .collect();
    let mut out: Vec<String> = Vec::new();
    let mut task: Option<String> = None;
    let mut instance: Option<usize> = None;
    for sx in prose(message) {
        match &sx {
            Sx::List(items) => {
                let head = match items.first() {
                    Some(Sx::Atom(head)) => head.as_str(),
                    _ => "",
                };
                if head == "task"
                    && items.len() == 3
                    && items.iter().all(|i| matches!(i, Sx::Atom(_)))
                {
                    task = Some(prose_text(&sx));
                    instance = None;
                } else if CLAUSES.contains(&head) {
                    if let Some(task) = &task {
                        out.push(format!("{task} {head}"));
                    }
                } else if head == "source" {
                    task = None;
                    instance = None;
                    out.push(prose_text(&sx));
                } else if head == "adapter" {
                    task = None;
                    out.push(prose_text(&sx));
                    instance = Some(out.len() - 1);
                }
            }
            Sx::Atom(atom) => {
                if let Some(at) = instance {
                    if obliged.contains(atom.as_str()) {
                        out[at].push_str(&format!(" {atom}"));
                        instance = None;
                    }
                }
            }
        }
    }
    out
}

/// The pairs a refusal's message names as `NAMER names OPERAND` — or `NAMER names the system OPERAND`, or several
/// operands joined by `and` — each once, in the order it first names them: the namer a `(task …)` form or a name, and
/// each operand one of `declared`, the description's declarations. §1 fixes the pairs and their order, not the
/// message's other words.
fn pairs_named_in(message: &str, declared: &BTreeSet<String>) -> Vec<(String, String)> {
    // A comma stays a word of its own here, so that a list of operands is read to its end and no further; a semicolon
    // or a colon ends one.
    const COMMA: &str = "\u{2063}comma\u{2063}";
    let words = prose(&message.replace(',', &format!(" {COMMA} ")));
    let atom = |at: usize, word: &str| matches!(words.get(at), Some(Sx::Atom(a)) if a == word);
    let mut out: Vec<(String, String)> = Vec::new();
    for at in 1..words.len() {
        if !atom(at, "names") {
            continue;
        }
        let namer = prose_text(&words[at - 1]);
        let mut next = at + 1;
        if atom(next, "the") && atom(next + 1, "system") {
            next += 2;
        }
        while let Some(Sx::Atom(word)) = words.get(next) {
            let word = word.trim_end_matches([':', '.']);
            if declared.contains(word) && !atom(next + 1, "names") {
                let pair = (namer.clone(), word.to_string());
                if !out.contains(&pair) {
                    out.push(pair);
                }
                if words
                    .get(next)
                    .is_some_and(|w| matches!(w, Sx::Atom(a) if a.ends_with([':', '.'])))
                {
                    break;
                }
            } else if word != "and" && word != COMMA {
                break;
            }
            next += 1;
        }
    }
    out
}

/// Where the lowering's `refusal` is not the one `due`: its code, and what it names. Of the interrupt-code refusal,
/// the subjects it names in their order, and what it says they would receive: §1 fixes the receivers and their order,
/// not the message's wording of what each receives; so an expiry is held to its instance's text, written whole, and
/// an alarm a receiver programs itself to its provider's name, written outside any instance's text — an interrupt
/// being "named by its kind and its provider" (§2). Of the durations past the arithmetic, each, in §1's order. Of the
/// naming-system refusal, each requester and the system it names, by the namer's text and then the system's; of the
/// refusal of hardware naming what is not hardware, each block or platform and its operand, by the namer's text and
/// then the operand's.
fn refusal_disagrees(due: &Due, refusal: &Refusal, description: &Description) -> Option<String> {
    let system = description.system.as_str();
    let declared: BTreeSet<String> = description
        .services
        .iter()
        .map(|s| s.name.clone())
        .chain(description.hardware().into_iter().map(|b| b.name))
        .chain(description.systems().into_iter().map(|s| s.name))
        .collect();
    let mut wrong = Vec::new();
    if refusal.code != due.code {
        wrong.push(format!(
            "its code is `{}`, the record's `{}`",
            refusal.code, due.code
        ));
    }
    match &due.names {
        Names::Task(task) => {
            let named = subjects_named(&refusal.message, system);
            if named != [task.clone()] {
                wrong.push(format!("it names {named:?}, the record [{task:?}]"));
            }
        }
        Names::Systems(pairs) => {
            let named = pairs_named_in(&refusal.message, &declared);
            if named != *pairs {
                wrong.push(format!(
                    "it names {named:?} as naming a system, the record {pairs:?}, in that order"
                ));
            }
        }
        Names::Operands(pairs) => {
            let named = pairs_named_in(&refusal.message, &declared);
            if named != *pairs {
                wrong.push(format!(
                    "it names {named:?} as hardware naming what is not hardware, the record {pairs:?}, in that \
                     order"
                ));
            }
        }
        Names::Durations(items) => {
            let named = durations_named(&refusal.message);
            if named != *items {
                wrong.push(format!(
                    "it names {named:?} past the arithmetic, the record {items:?}, in that order"
                ));
            }
        }
        Names::Receivers {
            subjects,
            expiries,
            alarms,
        } => {
            let named = subjects_named(&refusal.message, system);
            if named != *subjects {
                wrong.push(format!(
                    "it names {named:?}, the record {subjects:?}, in that order"
                ));
            }
            let forms = forms_in(&refusal.message);
            let words = words_outside_instances(&refusal.message);
            let unsaid: Vec<&String> = expiries
                .iter()
                .filter(|instance| !forms.contains(*instance))
                .chain(alarms.iter().filter(|provider| !words.contains(*provider)))
                .collect();
            if !unsaid.is_empty() {
                wrong.push(format!(
                    "it does not name what would be received: {unsaid:?}"
                ));
            }
        }
    }
    (!wrong.is_empty()).then(|| wrong.join("; "))
}

/// How one system fared.
struct Outcome {
    /// `lowered`, `rejected` by the language, `unresolved` by the search, or `refused` by the lowering.
    class: &'static str,
    /// The verdict of a system not lowered.
    verdict: String,
    /// Whether the configuration checker judged its record: every system the search resolves.
    judged: bool,
    /// The kinds of line a plan agrees with the reading on.
    agreed: Vec<&'static str>,
    /// The classes of the record's rules the system exercises.
    classes: BTreeSet<&'static str>,
    /// Where the code and the record's reading disagree, or the checker refuses the record: each a reproducer.
    findings: Vec<String>,
}

fn outcome(universe: &str, case: &Case) -> Outcome {
    let text = case.description.text();
    let mut out = Outcome {
        class: "lowered",
        verdict: String::new(),
        judged: false,
        agreed: Vec::new(),
        classes: BTreeSet::new(),
        findings: Vec::new(),
    };
    let (record, checker, lowered) = match flow(&text, &case.description.system) {
        Flow::Rejected(verdict) => {
            out.class = "rejected";
            out.verdict = verdict;
            return out;
        }
        Flow::Unresolved(verdict) => {
            out.class = "unresolved";
            out.verdict = verdict;
            return out;
        }
        Flow::Resolved {
            record,
            checker,
            lowered,
        } => (record, checker, lowered),
    };
    out.judged = true;
    let reproducer = |written: &str, what: &str| {
        format!(
            "== {universe}: {}\n-- the description\n{text}-- the record\n{record}{written}{what}",
            case.label
        )
    };
    if let Some(refused) = &checker {
        out.findings.push(reproducer(
            "",
            &format!(
                "-- the configuration checker refuses the record the search wrote: {refused}\n"
            ),
        ));
    }
    let due = refusal_the_record_gives(&case.description, &binds_of(&record));
    match lowered {
        Ok(plan) => {
            if let Some(due) = &due {
                out.findings.push(reproducer(
                    &format!("-- the code writes\n{plan}"),
                    &format!(
                        "-- the record refuses it: `{}`, naming {} (§1)\n",
                        due.code,
                        due.summary()
                    ),
                ));
                return out;
            }
            let judged = judge(&case.description, &record, &plan);
            out.agreed = judged.agreed;
            out.classes = judged.classes;
            if !judged.disagreements.is_empty() {
                let mut what = String::new();
                for d in &judged.disagreements {
                    what.push_str(&format!(
                        "-- {}: the record's reading expects\n{}\n-- {}: the plan writes\n{}\n",
                        d.kind,
                        d.expected.join("\n"),
                        d.kind,
                        d.written.join("\n")
                    ));
                }
                out.findings
                    .push(reproducer(&format!("-- the code writes\n{plan}"), &what));
            }
        }
        Err(refusal) => {
            out.class = "refused";
            out.verdict = refusal.to_string();
            let wrong = match &due {
                Some(due) => refusal_disagrees(due, &refusal, &case.description),
                None => Some("the record refuses it for none of §1's reasons".to_string()),
            };
            match (wrong, due) {
                (None, Some(due)) => out.classes = due.classes,
                (Some(wrong), due) => out.findings.push(reproducer(
                    &format!("-- the code writes\nrefused {refusal}\n"),
                    &format!(
                        "-- {wrong}{}\n",
                        due.map_or(String::new(), |due| format!(
                            " (§1: `{}`, naming {})",
                            due.code,
                            due.summary()
                        ))
                    ),
                )),
                (None, None) => unreachable!("a refusal with none due disagrees"),
            }
        }
    }
    out
}

/// What a universe gave.
#[derive(Debug, Default)]
struct Tally {
    generated: usize,
    lowered: usize,
    rejected: usize,
    unresolved: usize,
    refused: usize,
    /// The systems whose record the configuration checker judged.
    judged: usize,
    /// For each kind of line, the lowered systems whose plan agrees with the reading on it.
    agreed: BTreeMap<&'static str, usize>,
    /// For each class of the record's rules, the systems exercising it.
    classes: BTreeMap<&'static str, usize>,
    /// Each verdict of a system not lowered, with how many gave it and the first that did.
    verdicts: BTreeMap<String, (usize, String)>,
    findings: Vec<String>,
    elapsed: std::time::Duration,
}

/// Every case lowered and judged, on every core: the cases are independent.
fn run(universe: &str, cases: &[Case]) -> Tally {
    let start = std::time::Instant::now();
    let threads = std::thread::available_parallelism().map_or(1, std::num::NonZeroUsize::get);
    let mut outcomes: Vec<(usize, Outcome)> = std::thread::scope(|scope| {
        let workers: Vec<_> = (0..threads)
            .map(|first| {
                scope.spawn(move || {
                    cases
                        .iter()
                        .enumerate()
                        .skip(first)
                        .step_by(threads)
                        .map(|(i, case)| (i, outcome(universe, case)))
                        .collect::<Vec<_>>()
                })
            })
            .collect();
        workers
            .into_iter()
            .flat_map(|w| w.join().expect("a worker"))
            .collect()
    });
    outcomes.sort_by_key(|(i, _)| *i);
    let mut tally = Tally {
        generated: cases.len(),
        elapsed: start.elapsed(),
        ..Tally::default()
    };
    for (i, outcome) in outcomes {
        match outcome.class {
            "lowered" => tally.lowered += 1,
            "rejected" => tally.rejected += 1,
            "unresolved" => tally.unresolved += 1,
            _ => tally.refused += 1,
        }
        if outcome.judged {
            tally.judged += 1;
        }
        if outcome.class != "lowered" {
            tally
                .verdicts
                .entry(format!("{} {}", outcome.class, outcome.verdict))
                .or_insert((0, cases[i].label.clone()))
                .0 += 1;
        }
        for kind in outcome.agreed {
            *tally.agreed.entry(kind).or_default() += 1;
        }
        for class in outcome.classes {
            *tally.classes.entry(class).or_default() += 1;
        }
        tally.findings.extend(outcome.findings);
    }
    tally
}

/// The universe's sizes held to `sizes` — generated, lowered, rejected by the language, unresolved by the search,
/// refused by the lowering — every system the search resolved judged by the configuration checker, each class of
/// `exercised` reached by some system, and every finding, in one failure naming each.
fn hold(
    universe: &str,
    tally: &Tally,
    sizes: (usize, usize, usize, usize, usize),
    exercised: &[&str],
) {
    let got = (
        tally.generated,
        tally.lowered,
        tally.rejected,
        tally.unresolved,
        tally.refused,
    );
    eprintln!(
        "{universe}, in {:.1?}: {} generated, {} lowered, {} rejected, {} unresolved, {} refused; {} records judged by \
         the configuration checker\n  agreed: {:?}\n  classes: {:?}\n  not lowered: {:?}",
        tally.elapsed,
        got.0,
        got.1,
        got.2,
        got.3,
        got.4,
        tally.judged,
        tally.agreed,
        tally.classes,
        tally.verdicts
    );
    let mut problems = String::new();
    if got != sizes {
        problems.push_str(&format!(
            "{universe}'s sizes (generated, lowered, rejected, unresolved, refused) moved: {got:?}, held {sizes:?}; \
             not lowered: {:?}\n",
            tally.verdicts
        ));
    }
    if tally.judged != tally.lowered + tally.refused {
        problems.push_str(&format!(
            "{universe}: {} records judged by the configuration checker, of {} the search resolved\n",
            tally.judged,
            tally.lowered + tally.refused
        ));
    }
    let missed: Vec<&&str> = exercised
        .iter()
        .filter(|class| !tally.classes.contains_key(**class))
        .collect();
    if !missed.is_empty() {
        problems.push_str(&format!("{universe} no longer exercises {missed:?}\n"));
    }
    if !tally.findings.is_empty() {
        problems.push_str(&format!(
            "{universe}: {} findings, a record the configuration checker refuses or a plan the record's reading does \
             not give:\n{}",
            tally.findings.len(),
            tally.findings.join("\n")
        ));
    }
    assert!(problems.is_empty(), "{problems}");
}

#[test]
fn three_services_their_channels_sources_and_callers() {
    let cases = three_services();
    // 4 capabilities for each of 3 services, each of the 6 `uses` edges present or not, each of 3 services used by the
    // task or not.
    assert_eq!(cases.len(), 4usize.pow(3) * (1 << 6) * (1 << 3));
    let tally = run("three services", &cases);
    hold(
        "three services",
        &tally,
        (32_768, 32_768, 0, 0, 0),
        &[
            "initialisation: a named edge",
            "initialisation: a component of several members",
            "context: a node in two interrupts",
            "multiplex: multiplexed",
            "multiplex: multiplexed by interrupts alone",
            "multiplex: a channel with one user",
            "multiplex: its own route no user",
            "multiplex: its own route a user, another node there reaching it",
            "multiplex: another route a user",
        ],
    );
}

#[test]
fn two_services_with_adapter_instances() {
    let cases = two_services_with_adapters();
    // 3 shapes; `b` a service or a policy; names by `uses` or `needs`; 8 capabilities for each of 2 services; each of
    // 2 edges; each of 2 services named by the task or not.
    assert_eq!(cases.len(), 3 * 2 * 2 * 8usize.pow(2) * (1 << 2) * (1 << 2));
    let tally = run("two services with adapters", &cases);
    hold(
        "two services with adapters",
        &tally,
        (12_288, 12_032, 0, 256, 0),
        &[
            "instance: an obliged fact bare",
            "initialisation: a named edge",
            "initialisation: a component of several members",
            "context: an expiry running its requester",
            "context: a wake running the instance alone",
            "context: a node in two interrupts",
            "multiplex: multiplexed",
            "multiplex: multiplexed by interrupts alone",
            "multiplex: a policy multiplexed",
            "multiplex: a channel with one user",
            "multiplex: its own route no user",
            "multiplex: its own route a user, another node there reaching it",
            "multiplex: another route a user",
        ],
    );
}

#[test]
fn releases_and_routes_of_every_separation_class() {
    let cases = releases_and_routes();
    // An alarm or none; `t` of 2 kinds; `u` absent or of 2 kinds. For each source, 6 separations; the first source has
    // 5 paths, 8 with task `u`, the second 3, or 5 with `u`.
    let per = |first: usize, second: usize| 6 * first + 6 * first * 6 * second;
    assert_eq!(cases.len(), 2 * 2 * (per(5, 3) + 2 * per(8, 5)));
    let tally = run("releases and routes", &cases);
    hold(
        "releases and routes",
        &tally,
        (14_184, 12_468, 0, 0, 1_716),
        &[
            "refusal: a periodic task bound to a source itself",
            "initialisation: a named edge",
            "trigger: every, equal separations",
            "trigger: every, a longer separation",
            "trigger: unknown, a shorter separation",
            "trigger: unknown, no separation stated",
            "trigger: unknown, the separation absent",
            "trigger: unknown, no source reaching it",
            "trigger: unknown, several sources",
            "rank: two stated separations tied",
            "rank: every class in one system",
        ],
    );
}

#[test]
fn requesters_of_every_kind_refused_or_lowered() {
    let cases = requesters_of_every_kind();
    // 4 requesters, each requiring one of 4 things.
    assert_eq!(cases.len(), 4 * 4);
    let tally = run("requesters of every kind", &cases);
    hold(
        "requesters of every kind",
        &tally,
        (16, 9, 0, 0, 7),
        &[
            "refusal: a periodic task bound to a source itself",
            "receivers: an alarm programmed itself",
            "receivers: an expiry",
            "receivers: the system's own level alone",
            "instance: an obliged fact bare",
            "context: a wake running the instance alone",
            "context: an expiry running its requester",
            "trigger: every, equal separations",
            "trigger: unknown, no source reaching it",
        ],
    );
}

#[test]
fn two_tasks_sharing_services_that_take_a_channel() {
    let cases = two_tasks_sharing_services();
    // 8 capabilities for each of 2 services, each of 2 `uses` edges, each of 2 services used by `t` or not and by `u`
    // or not, and each task writing `(deadline-from release)` or not.
    assert_eq!(
        cases.len(),
        8usize.pow(2) * (1 << 2) * (1 << 2) * (1 << 2) * 4
    );
    let tally = run("two tasks sharing services", &cases);
    hold(
        "two tasks sharing services",
        &tally,
        (16_384, 16_384, 0, 0, 0),
        &[
            "task: deadline-from written",
            "task: deadline-from owed",
            "trigger: every, equal separations",
            "trigger: unknown, no source reaching it",
            "instance: an obliged fact with its bound",
            "instance: an obliged fact bare",
            "context: an expiry running its requester",
            "context: a node in two interrupts",
            "initialisation: a named edge",
            "initialisation: a component of several members",
            "multiplex: multiplexed",
            "multiplex: multiplexed by two jobs",
            "multiplex: a channel with one user",
            "multiplex: its own route no user",
            "multiplex: its own route a user, another node there reaching it",
            "multiplex: another route a user",
        ],
    );
}

#[test]
fn receivers_of_interrupt_code_each_named_in_order() {
    let cases = receivers_of_interrupt_code();
    // What `t`, `u` and the system's own level each program, of 3; which task is first; each task of 2 kinds; each
    // writing `(deadline-from release)` or not.
    assert_eq!(cases.len(), 3usize.pow(3) * 2 * 4 * 4);
    let tally = run("receivers of interrupt code", &cases);
    hold(
        "receivers of interrupt code",
        &tally,
        (864, 32, 0, 0, 832),
        &[
            "task: deadline-from written",
            "task: deadline-from owed",
            "receivers: one",
            "receivers: two",
            "receivers: three",
            "receivers: an alarm programmed itself",
            "receivers: an expiry",
            "receivers: the system's own level alone",
            "receivers: tasks and the system's own level",
            "receivers: runtime order unlike the tasks' text order",
        ],
    );
}

#[test]
fn requiring_hardware_its_nodes_contexts_and_multiplexing() {
    let cases = requiring_hardware();
    // What `dma.x` and `soc` each require, of 5; who binds to each, of 8; what composes `alarm`, of 3; `u` of 2 kinds.
    assert_eq!(cases.len(), 5 * 5 * 8 * 8 * 3 * 2);
    let tally = run("requiring hardware", &cases);
    // A third of it lowered: `alarm` named by `dma.x` or `soc` is refused (§1, round 13).
    hold(
        "requiring hardware",
        &tally,
        (9_600, 3_200, 0, 0, 6_400),
        &[
            "refusal: hardware naming what is not hardware",
            "operand: a block's `uses` naming a service",
            "operand: a platform's `uses` naming a service",
            "hardware: a block bound for requirements of its own",
            "hardware: a platform bound for requirements of its own",
            "hardware: requiring, and bound to by nothing",
            "hardware: requiring, and bound to by a requester",
            "hardware: an alarm it programs itself",
            "hardware: an expiry running its code",
            "hardware: a source's handler",
            "hardware: in an interrupt through a binding to it",
            "initialisation: no edge across composition",
            "initialisation: an edge to a block or a platform bound for its own requirements",
            "multiplex: a block or a platform multiplexed",
            "multiplex: multiplexed by two jobs",
            "multiplex: multiplexed by a job and an interrupt",
            "multiplex: a block or a platform's channel with one user",
            "multiplex: its own route no user",
            "multiplex: another route a user",
            "composition: a platform a task reaches through a binding",
            "composition: what a node in a job composes is in none",
            "composition: what a node in an interrupt composes is not run there",
            "trigger: a source a block or a platform handles",
            "trigger: every, equal separations",
            "trigger: unknown, no source reaching it",
        ],
    );
}

#[test]
fn task_clauses_jitter_overrun_and_reference() {
    let cases = task_clauses();
    // Which task is first; for each of 2 tasks, 5 jitters, 3 overrun policies and 3 references.
    assert_eq!(cases.len(), 2 * (5 * 3 * 3) * (5 * 3 * 3));
    let tally = run("task clauses", &cases);
    hold(
        "task clauses",
        &tally,
        (4_050, 1_152, 0, 0, 2_898),
        &[
            "task: jitter zero where none is stated",
            "task: jitter written zero",
            "task: jitter written",
            "task: jitter longer than its release's duration",
            "task: overrun fault where none is stated",
            "task: overrun fault as written",
            "task: overrun another policy as written",
            "task: deadline-from written",
            "task: deadline-from owed",
            "refusal: a deadline measured from another event",
            "refusal: a duration past the arithmetic",
            "past: a task's clause",
            "past: several, named in order",
            "past: before a deadline from another event",
            "trigger: every, equal separations",
        ],
    );
}

#[test]
fn durations_past_the_arithmetic_and_at_its_edge() {
    let cases = past_the_arithmetic();
    // Values plain or at the edge; which task is first; `t`'s three clauses each past or not, `u`'s all or none; each
    // of 2 sources of 4 states; each of 2 bounds of 3.
    assert_eq!(cases.len(), 2 * 2 * 8 * 2 * 4 * 4 * 3 * 3);
    let tally = run("past the arithmetic", &cases);
    hold(
        "past the arithmetic",
        &tally,
        (9_216, 64, 0, 0, 9_152),
        &[
            "refusal: a duration past the arithmetic",
            "past: a task's clause",
            "past: a source's separation",
            "past: an obliged bound",
            "past: a task's, a source's and a bound at once",
            "past: several, named in order",
            "exact: a task's duration at the arithmetic's edge",
            "exact: a separation at the arithmetic's edge",
            "exact: an obliged bound at the arithmetic's edge",
            "exact: two separations compared past 128 bits",
            "exact: a source's and a task's separations compared past 128 bits",
            "trigger: every, a longer separation",
            "trigger: every, equal separations",
            "trigger: unknown, no source reaching it",
            "instance: an obliged fact with its bound",
            "instance: an obliged fact bare",
        ],
    );
}

#[test]
fn a_second_system_named_or_not() {
    let cases = two_systems();
    // 5 naming sites, each present or not; by `uses` or `needs`; `alarm` used by one system or both; `other` on `sys`'s
    // platform or its own; `sys`'s own level of 2 and `other`'s of 3; each description lowered for each system.
    assert_eq!(cases.len(), (1 << 5) * 2 * 2 * 2 * 2 * 3 * 2);
    for pair in cases.chunks(2) {
        assert_eq!(
            pair[0].description.text(),
            pair[1].description.text(),
            "one description, lowered for each of its systems"
        );
    }
    let tally = run("a second system", &cases);
    hold(
        "a second system",
        &tally,
        (3_072, 576, 0, 0, 2_496),
        &[
            "refusal: a requester naming a system",
            "naming: a task naming a system",
            "naming: the system's own level naming a system",
            "naming: a service naming a system",
            "naming: several names",
            "naming: a system another system's requester names, outside this one's",
            "second system: a service another system's task also uses",
            "second system: requiring hardware the lowered system does not serve",
        ],
    );
}

#[test]
fn hardware_naming_what_is_not_hardware_refused_in_its_place() {
    let cases = hardware_naming();
    // What `timer.d` names, of 3; `dma.x`, of 9; `soc`, of 5; `blk.q`, of 2; the other refusal `t` meets, of 6.
    assert_eq!(cases.len(), 3 * 9 * 5 * 2 * 6);
    let tally = run("hardware naming", &cases);
    // Lowered: the 8 whose hardware names hardware alone — `timer.d` and `blk.q` nothing, `dma.x` nothing or a block (4),
    // `soc` nothing or a block (2) — and whose task `t` meets no other refusal; every other refused.
    hold(
        "hardware naming",
        &tally,
        (1_620, 8, 0, 0, 1_612),
        &[
            "refusal: hardware naming what is not hardware",
            "operand: a block's `uses` naming a service",
            "operand: a block's `needs` naming a service",
            "operand: a block's `uses` naming a policy",
            "operand: a block's `needs` naming a policy",
            "operand: a platform's `uses` naming a service",
            "operand: a platform's `needs` naming a policy",
            "operand: a block naming the service bound to it",
            "operand: a service only hardware names",
            "operand: hardware the lowered system does not serve",
            "operand: several, named in order",
            "operand: one namer naming two",
            "operand: the namers' order unlike the operands'",
            "operand: before a duration past the arithmetic",
            "operand: before a deadline from another event",
            "operand: before a receiver of interrupt code",
            "periodic: before hardware naming what is not hardware",
            "naming: before hardware naming what is not hardware",
            "naming: a block or a platform naming a system",
            "hardware: a block's or a platform's `needs` naming a block",
            "initialisation: no edge across composition",
            "initialisation: an edge across a block's `needs` naming a block stands",
        ],
    );
}

#[test]
fn a_block_a_service_names_served_though_no_hardware() {
    let cases = served_blocks();
    // How `blk` is named, of 7; what it requires, of 5; its offer bound to or not; `u` of 2 kinds.
    assert_eq!(cases.len(), 7 * 5 * 2 * 2);
    let tally = run("served blocks", &cases);
    // Unresolved: the 60 where `xfer` needs `blk`'s offer and `blk` is no hardware (6 namings of 7, 5 requirements, 2
    // kinds of `u`): a requester bound outside the hardware is a conflict (model §8 rule 7), and nothing else offers
    // `transfer-engine`, so the search concludes `missing-fact`.
    hold(
        "served blocks",
        &tally,
        (140, 80, 0, 60, 0),
        &[
            "served: a block a service names, no hardware, bound for requirements of its own",
            "hardware: a block bound for requirements of its own",
            "hardware: an alarm it programs itself",
            "hardware: an expiry running its code",
            "hardware: a source's handler",
            "trigger: a source a block or a platform handles",
        ],
    );
}

#[test]
fn several_systems_named_in_the_namers_order() {
    let cases = naming_order();
    // What each of 4 namers names, of 4; by `uses` or `needs`; `zz`'s own level naming `yy` or not.
    assert_eq!(cases.len(), 4usize.pow(4) * 2 * 2);
    let tally = run("naming order", &cases);
    // Lowered: the 4 where no namer names a system (by either clause, `zz` naming `yy` or not, `zz` never reached).
    hold(
        "naming order",
        &tally,
        (1_024, 4, 0, 0, 1_020),
        &[
            "refusal: a requester naming a system",
            "naming: a task naming a system",
            "naming: the system's own level naming a system",
            "naming: a service naming a system",
            "naming: a named system's own level naming a system",
            "naming: several names",
            "naming: one namer naming two systems",
        ],
    );
}

#[test]
fn the_reading_s_arithmetic_is_exact() {
    // The reading's seconds, held to values derived by hand from §2.2's rule — "one reduced rational number of
    // seconds ... `N` and `D` lie within ... signed 128-bit integers" — so that a fault of the reading's arithmetic shows
    // here, never as a verdict on the code.
    let seconds = |spelling: &str| Ratio::of(spelling).map(Ratio::text);
    assert_eq!(seconds("20 ms").as_deref(), Some("1/50 s"));
    assert_eq!(seconds("1500 us").as_deref(), Some("3/2000 s"));
    assert_eq!(seconds("0 ms").as_deref(), Some("0 s"));
    assert_eq!(
        seconds(WIDE).as_deref(),
        Some("9223372036854775807/100000000000000000000000000000000000000 s")
    );
    assert_eq!(seconds(WIDE_IN_MS), seconds(WIDE));
    assert_eq!(
        seconds(WIDE_LONGER).as_deref(),
        Some("922337203685477581/10000000000000000000000000000000000000 s")
    );
    assert_eq!(
        seconds(REDUCED).as_deref(),
        Some("1/100000000000000000000000000000000000000 s")
    );
    // 5^13/10^47 = 1/(2^13·10^34); 5^12/10^47 = 1/(2^12·10^35), past `i128::MAX` = 170141183460469231731687303715884105727.
    assert_eq!(
        seconds(TIGHT_WITHIN).as_deref(),
        Some("1/81920000000000000000000000000000000000 s")
    );
    assert_eq!(seconds(TIGHT_PAST), None);
    assert_eq!(seconds(PAST), None);
    // Exact comparison, where cross-multiplication in 128 bits overflows.
    let of = |spelling: &str| Ratio::of(spelling).expect("within");
    assert!(of(WIDE) < of(WIDE_LONGER));
    assert!(past_128_bits(of(WIDE), of(WIDE_LONGER)));
    // 1/10^38 < 1/(8.192·10^37) < 9223372036854775807/10^38.
    assert!(of(REDUCED) < of(TIGHT_WITHIN) && of(TIGHT_WITHIN) < of(WIDE));
    assert!(of(WIDE_LONGER) < of("20 ms"));
}

#[test]
fn the_reading_transcribes_the_shipped_vocabulary_and_catalog() {
    // The reading's constants are its transcription of `vocabulary.eadl` and `adapters.eadl`, held here to the shipped
    // vocabulary and catalog, so that a change to either fails here rather than in the reading's verdicts.
    let vocabulary = common::vocabulary();
    let consumed: BTreeSet<&str> = vocabulary
        .facts()
        .filter(|f| f.usage == Usage::Consumed)
        .map(|f| f.name.as_str())
        .collect();
    assert_eq!(consumed, BTreeSet::from(CONSUMED));
    let catalog = common::catalog();
    assert_eq!(
        catalog
            .kinds
            .iter()
            .map(|k| k.name.as_str())
            .collect::<Vec<_>>(),
        KINDS.iter().map(|k| k.name).collect::<Vec<_>>()
    );
    for kind in &KINDS {
        let shipped = catalog.kind(kind.name).expect("a shipped kind");
        let obliged: Vec<&str> = shipped
            .offers
            .iter()
            .filter(|o| o.offered == Offered::Obliged)
            .map(|o| o.fact.as_str())
            .collect();
        assert_eq!(obliged, kind.obliged, "{}", kind.name);
        let names = |owed: &[eadl_resolve::adapter::Owed]| -> Vec<String> {
            owed.iter().map(|o| o.name.clone()).collect()
        };
        assert_eq!(
            names(&shipped.obligations),
            kind.obligations,
            "{}",
            kind.name
        );
        assert_eq!(names(&shipped.costs), kind.costs, "{}", kind.name);
    }
}

/// The record's worked plans (§3), in its order, each with its digests elided.
fn worked_plans() -> Vec<String> {
    let record =
        std::fs::read_to_string(common::root().join("docs/decisions/decision_runtime-plan.md"))
            .expect("the record");
    let mut out = Vec::new();
    let mut open: Option<String> = None;
    for line in record.lines() {
        match (&mut open, line.trim_start()) {
            (None, "```text") => open = Some(String::new()),
            (Some(block), "```") => {
                // A worked plan, not §3's template of the forms: its digests are elided.
                if block.starts_with("(format \"archogen-runtime-plan/1\")")
                    && block.contains("(program \"…\")")
                {
                    out.push(std::mem::take(block));
                }
                open = None;
            }
            (Some(block), _) => {
                block.push_str(line);
                block.push('\n');
            }
            (None, _) => {}
        }
    }
    out
}

#[test]
fn the_reading_gives_the_record_s_worked_texts() {
    // The reading held to the plans the record writes in full (§3), from their inputs transcribed here and their own
    // `bind` lines, for every kind of line it derives: a fault of the reading shows on the record's own text, never on
    // the code's.
    let task =
        |name: &str, release: Release, priority: u32, deadline_from: bool, uses: &[&str]| Task {
            name: name.to_string(),
            release,
            priority,
            deadline_from: deadline_from.then_some("release"),
            side: Side {
                uses: names(uses),
                ..Side::default()
            },
            ..Task::default()
        };
    let plain = |name: &str, offers: &[&str]| Block {
        name: name.to_string(),
        offers: names(offers),
        ..Block::default()
    };
    let uses = |declarations: &[&str]| Side {
        uses: names(declarations),
        ..Side::default()
    };
    let heartbeat = Description {
        system: "heartbeat".to_string(),
        blocks: vec![plain("console.uart", &["(observable-output true)"])],
        services: vec![Service {
            name: "console.write".to_string(),
            policy: false,
            side: Side {
                needs: names(&["observable-output"]),
                ..Side::default()
            },
        }],
        tasks: vec![
            task(
                "beat",
                Release::Period(Dur::Ms(10)),
                1,
                true,
                &["console.write"],
            ),
            task(
                "chime",
                Release::Period(Dur::Ms(30)),
                2,
                true,
                &["console.write"],
            ),
        ],
        own: uses(&["console.write"]),
        ..Description::default()
    };
    let extender = Description {
        system: "short-counter".to_string(),
        blocks: vec![plain("timer.short", &["(relative-delay true)"])],
        services: vec![service(
            "time.monotonic",
            Counter::Extender,
            false,
            Vec::new(),
        )],
        tasks: vec![task(
            "beat",
            Release::Period(Dur::Ms(10)),
            1,
            true,
            &["time.monotonic"],
        )],
        own: uses(&["time.monotonic"]),
        ..Description::default()
    };
    let sporadic = Description {
        system: "sporadic".to_string(),
        blocks: vec![source_block("dev.e", Separation::Stated(20), "")],
        services: vec![service("evt", Counter::None, true, Vec::new())],
        tasks: vec![task(
            "s",
            Release::MinSeparation(Dur::Ms(10)),
            1,
            false,
            &["evt"],
        )],
        own: uses(&["evt"]),
        ..Description::default()
    };
    let worked = worked_plans();
    assert_eq!(worked.len(), 3);
    for (description, plan) in [heartbeat, extender, sporadic].iter().zip(&worked) {
        // The record is the plan's own lines: its `bind` lines are the assignment the record names, and its identity's
        // digests are elided.
        let judged = judge_elided(description, plan, plan, true);
        let shown: Vec<String> = judged
            .disagreements
            .iter()
            .map(|d| {
                format!(
                    "{}: the reading\n{}\nthe record\n{}",
                    d.kind,
                    d.expected.join("\n"),
                    d.written.join("\n")
                )
            })
            .collect();
        assert!(
            shown.is_empty(),
            "{}:\n{}",
            description.system,
            shown.join("\n")
        );
    }
}
