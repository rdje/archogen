//! The runtime plan's bounded exhaustive checker (leaf `M4.1.13`), which
//! `docs/decisions/decision_executable-design-reviews.md` item 1 asks of a design whose subject is a decision
//! procedure: it "enumerates a finite universe of inputs" and holds each outcome to the record. Universes of small
//! descriptions are generated in memory, every combination of a few independent choices, so that each is exhaustive
//! over its bounds: three services and their `uses` edges; two services with adapter instances; releases and routes of
//! every separation class; requesters of every kind; two tasks, periodic and sporadic, sharing services that take a
//! channel, with an obliged `delivery-bound` bound and bare and each task writing `(deadline-from release)` or not;
//! receivers of interrupt code, two tasks and the system's own level each programming an alarm itself, receiving an
//! expiry or neither, in both runtime orders; after round 12 (R12-R4), blocks and platforms with requirements of their
//! own, bound to by something or by nothing; every jitter, overrun policy and deadline reference a task line states;
//! durations past the exact arithmetic, each kind named in §1's order, and durations at its edge, written exactly
//! however they are spelled; and a second system, named by `uses` or `needs` or not, each lowered; after round 13
//! (R13-R1 to R13-R3), blocks and platforms whose `uses` or `needs` name a service, a policy, a system or another
//! block, alone, several at once or beside each other refusal of §1 that can co-apply, on the lowered system's hardware
//! or on another's; a block a service names, served though not the system's hardware, bound for requirements of its
//! own; and requesters naming several systems, held to the naming-system refusal's order; and, after round 14 (R14-D1,
//! R14-R1, R14-R6), names one of which prefixes another — `m` and `m.s`, `y` and `y.b`, `d` and `d.x` — and names
//! continuing with `!`, a byte below `)`, so that each order §1 holds by text is held where a message's words would
//! give another: the naming-system refusal's namers and systems, tasks among them; the hardware-naming refusal's
//! blocks, platforms and operands, a platform's `needs` naming a service and its `uses` a policy, and a system named on
//! hardware the lowered system does not serve; the sources and the bounds past the arithmetic, and one requester's two
//! bounds on its instance's one obliged fact; receivers of interrupt code over two providers one of whose names
//! prefixes the other's, each requester programming an alarm, a deadline or both, which the search refuses
//! (`missing-fact`) unless one provider offers both, so that no system it resolves gives one receiver two things; and a
//! provider whose own requirement is bound to the node that takes its channel, whose own route then counts among the
//! node's users; and, after round 15 (R15-D2, R15-R2, R15-R5), tasks, the system's own level and a service bound to a
//! timer route's provider by a binding that takes no channel (model §8 rule 2) — a shared fact, the route naming them
//! through an instance or not at all, or a consumed fact met by `false` — beside one programming an alarm or receiving
//! an expiry there, in both runtime orders; periodic tasks bound to a source's block for another fact than
//! `interrupt-source`, beside one bound for it, in both runtime orders, and `interrupt-source` met by `false`, which is no
//! source; hardware the lowered system does not serve, another system's or none's, naming a service the lowered system
//! serves, a policy, a system or the lowered system itself, which neither naming refusal takes, each description
//! lowered for each of its systems; a platform the lowered system serves naming a system by `uses` or by `needs`; and a
//! node in a job naming by `needs` a node no job reaches; and, after round 16 (R16-D3, R16-R1, R16-R2, R16-B1,
//! R16-B2), sporadic tasks bound themselves for `interrupt-source`, beside the routes of another source and of their
//! own source's handler they reach, and beside a task binding none, released by those routes; a platform and the
//! blocks it composes, directly or through another block, each bound to the other, one to the other or neither, under
//! names that order each composite before its part or after it; and the system's own level, a service, a policy, a
//! platform and a block each served by an exclusive `deadline-over-delay` instance, writing its obliged bound bare,
//! within the arithmetic or past it, a platform's, a block's, a policy's and a service's bound within it each counted
//! on its instance's line; and, after round 17 (R17-D1, R17-R2, R17-R6), a block and a platform that a service and a
//! policy name, by `uses` or by `needs`, under names that order each namer before what it names or after it, the named
//! block or platform a provider, bound for requirements of its own, both or neither, composed by the platform or served
//! through its namer alone, the namers used by a task, by the system's own level alone, or the policy by nothing; and a
//! timer and a source a service or a policy names, bound to its namer for an alarm, for its interrupt, through the
//! instance it programs or not at all, whose own interrupt runs that namer — so that rule 12's named edge from hardware
//! to its namer is held, alone or beside a binding's reason, and each node in a job owes its call cost and its masked
//! runs; and, after round 18 (R18-D1, R18-D3, R18-R5), a block and a platform requiring nothing that a task, a service,
//! a policy, a source's handler and the system's own level each name or not, by `uses` or by `needs`, composed by the
//! platform or by nothing, the services used by the task or by the system's own level alone, bound to by nothing or by
//! an unrelated requester alone — so that hardware a served task, service or policy names is a node bound or not, with
//! its contexts, step, edges and owed facts, which a binding elsewhere leaves as the name gives them, the system's own
//! level's name and a platform's composition making none, and a node owes its bounds with no category of their own;
//! and, after round 19 (R19-D2), no universe of its own but every task of every universe owing the application its
//! `os-calls-category` beside its `os-calls`, each of the application's bounds and counts owed with its evidence
//! category a fact of its own, the reading's facts of a task, a source and a timer route held to §2.10's table and §3's
//! `FACT` list as the record writes them.
//!
//! Each system is run as the engine runs it, as `common::lowered` does: the language's check, the search, the `/2`
//! writer, and the lowering. **Every `archogen-assignment/2` record the search writes is judged by the configuration
//! checker**, `archogen_plan_check::check::plan_file`, as `crates/eadl-resolve/tests/plan.rs` judges one, and a record
//! it refuses is a finding naming that record — but for one class the reading derives and holds by count, a system's
//! side the search reads apart, the search's defect (`M3.4`; model §7 rule 4, §8 rule 1), on which the lowering is
//! still held to the reading while it does not read the checker's result record, its third input; once it does, §1
//! refuses each of those systems `tool-failure`, "naming both verdicts", before anything is lowered (R16-R2). Each plan
//! is held to an **independent reading** of
//! `docs/decisions/decision_runtime-plan.md`, written below as plain, unoptimised Rust from the record's text alone,
//! each rule citing the section it transcribes.
//!
//! ⭐ **The assignment is input.** The reading takes the description and the record's own `bind` lines — what the search
//! chose, never something predicted — and derives from them every line of the plan: §2.1's identity, the record's own
//! and its sha256; §2.2's task lines, their index, rank, durations, deadline reference, trigger and `(uses …)` item;
//! §2.3's `bind` lines, the record's copied; §2.4's instance lines, an obliged fact with its requester's bound or bare;
//! the release timer's line, each route with its separation and rank, the enabled set and the interrupt model (§2.5);
//! §2.6's `(context …)` lines; §2.7's `(depends …)` and `(step …)` lines, from model §8 rule 12's graph, a component of
//! several members one step; §2.9's obligations, `multiplex` among them; §2.10's owed facts, each task's bounds and
//! counts owed to the application each with a category of its own, every bound owed to the realization with no category
//! beside it; and the last line, `(complete no)`. **Every line of the plan is compared, in
//! its place**: the plan's whole text is held to the reading's, the forms in §3's order across kinds as well as within
//! one, and a line of a head §3 does not write is a disagreement of its own, never dropped. It derives which of §1's
//! refusals is due, where these universes reach one, and holds the lowering's refusal to its code, to the subjects it
//! names — each told by its exact text, a task's `(task SYSTEM NAME)` form or the system's own level's name as a word
//! of its own, never by a substring — in §1's order, each kind by its texts compared byte by byte, "never by its
//! message's words"; to the clause it names, where §1 names one; and to what each receiver would receive, in §1's order
//! — its alarms, then its expiries. The reading is itself held to the record's three worked texts (§3), whole, and its
//! readers of a refusal's prose to messages written in words of their own.
//!
//! ⛔ **Independence.** The reading was written by contexts that never read `crates/archogen-plan/src/lib.rs` nor the
//! corpus's outcomes. A disagreement is a finding, never something to make disappear: decide from the record's text
//! which side is right, and keep a reading the record supports failing on its system.
//!
//! Each universe's size is asserted — the systems generated and how each fared, lowered or not — with every system the
//! search resolves judged by the configuration checker, and each class of the record's rules it is built to reach, so
//! that a generator cannot shrink unnoticed; a system the language or the search refuses is counted, never dropped. A
//! branch of the reading that no system can reach is stated so, its reason in its class's name, `unreachable: …`, and a
//! universe reaching one fails. Where the record says what a binding elsewhere leaves unchanged, each system's plan is
//! also held line by line to its twin's.

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
/// or through two, or reaching none — a task bound to one itself released by it alone, whatever other source it
/// reaches (round 16, R16-B1); an alarm `t` uses, or none. The first source, `dev.z`, is declared first and the
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
    /// `expiry` runs its requester's code — or by a block offering it, an alarm then.
    Deadline,
    /// Both: `(needs relative-delay absolute-deadline)`.
    Both,
}

impl Programs {
    /// Both an alarm and a deadline for one requester is no choice here: the two facts are of one function, `counter`,
    /// which the search binds to one provider whole, and neither the timer nor the instance offers both, so it
    /// concludes `missing-fact` on every such system — which the one-receiver universe holds.
    const ALL: [Self; 3] = [Self::Nothing, Self::Alarm, Self::Deadline];
    /// Every choice, both among them.
    const WITH_BOTH: [Self; 4] = [Self::Nothing, Self::Alarm, Self::Deadline, Self::Both];

    fn needs(self) -> Vec<String> {
        match self {
            Self::Nothing => Vec::new(),
            Self::Alarm => names(&["relative-delay"]),
            Self::Deadline => names(&["absolute-deadline"]),
            Self::Both => names(&["relative-delay", "absolute-deadline"]),
        }
    }
}

/// The counters of the one-receiver universe: `d` and `d.x`, each with four channels.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Counters {
    /// Two timers, each offering `relative-delay` and what a `deadline-over-delay` instance needs beneath.
    TwoTimers,
    /// The timer `d` offering `absolute-deadline` itself too, and the timer `d.x`: one requester may bind both facts
    /// to `d`, one alarm.
    OneOffersBoth,
    /// The timer `d`, and `d.x` offering `absolute-deadline` alone.
    DeadlineApart,
}

/// **One receiver's alarms and expiries** (§1; round 14, R14-R6: "Its alarms, then its expiries, each by its text"):
/// tasks `t` and `t!`, either first in runtime order, and the system's own level, each programming nothing, an alarm,
/// an absolute deadline or both; over two counters `d` and `d.x`, one name a prefix of the other, of each shape of
/// [`Counters`]. One receiver can receive two things only by two providers, an alarm at one and an alarm or an expiry
/// at another; the search binds a requester's `counter` facts to one provider whole, so it refuses every such system
/// (`missing-fact`), and the systems it refuses are counted. Where one provider offers both facts, a requester binding
/// both there programs one alarm, named once. So §1's order within one receiver is reachable by no system the search
/// resolves, and its order across receivers, each named with what it receives, is held over providers and instances
/// one of whose texts prefixes another's.
fn receivers_within_one() -> Vec<Case> {
    let mut cases = Vec::new();
    for counters in [
        Counters::TwoTimers,
        Counters::OneOffersBoth,
        Counters::DeadlineApart,
    ] {
        for t_programs in Programs::WITH_BOTH {
            for bang_programs in Programs::WITH_BOTH {
                for own_programs in Programs::WITH_BOTH {
                    for bang_first in [false, true] {
                        let task = |name: &str, priority: u32, programs: Programs| Task {
                            name: name.to_string(),
                            priority,
                            side: Side {
                                needs: programs.needs(),
                                ..Side::default()
                            },
                            ..Task::default()
                        };
                        let (t_priority, bang_priority) = if bang_first { (2, 1) } else { (1, 2) };
                        let mut d = timer_block("d", 4, true);
                        let d_x = match counters {
                            Counters::TwoTimers | Counters::OneOffersBoth => {
                                timer_block("d.x", 4, true)
                            }
                            Counters::DeadlineApart => Block {
                                name: "d.x".to_string(),
                                offers: names(&[
                                    "(absolute-deadline true)",
                                    "(channels 4 tick)",
                                    "(reachable-at-privilege machine)",
                                ]),
                                ..Block::default()
                            },
                        };
                        if counters == Counters::OneOffersBoth {
                            d.offers.insert(1, "(absolute-deadline true)".to_string());
                        }
                        cases.push(Case {
                            label: format!(
                                "{counters:?} t:{t_programs:?} t!:{bang_programs:?} own:{own_programs:?} first={}",
                                if bang_first { "t!" } else { "t" }
                            ),
                            description: Description {
                                blocks: vec![d, d_x],
                                tasks: vec![
                                    task("t", t_priority, t_programs),
                                    task("t!", bang_priority, bang_programs),
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
    cases
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
/// binding — while what a block or a platform composes is never reached through it, composition giving no edge, and
/// the platform's binding to a block it composes keeps its edge (§2.7; round 16, R16-B2). Since round 13 (R13-D3), a
/// block's or a platform's `uses` naming `alarm` is refused before any plan (§1), naming it.
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

/// The separations of the arithmetic universe's sources, `dev` of 20 ms and `dev!` of 40 ms, and at the
/// arithmetic's edge `dev` of [`WIDE`] in two agreeing spellings and `dev!` of [`WIDE_LONGER`]; each also with a
/// spelling past the arithmetic beside it, [`PAST`] or, at the edge, [`TIGHT_PAST`].
const Z_WITHIN: [&[&str]; 2] = [&["20 ms"], &[WIDE, WIDE_IN_MS]];
const Z_OVERFLOWING: [&[&str]; 2] = [&["20 ms", PAST], &[WIDE, TIGHT_PAST]];
const A_WITHIN: [&[&str]; 2] = [&["40 ms"], &[WIDE_LONGER]];
const A_OVERFLOWING: [&[&str]; 2] = [&["40 ms", PAST], &[WIDE_LONGER, TIGHT_PAST]];
const ONLY_PAST: [&[&str]; 2] = [&[PAST], &[TIGHT_PAST]];

/// **Durations past the arithmetic, and at its edge** (§1, §2.2, §2.4, §2.5; round 12, R12-R4): a periodic task `t`
/// using a source `dev` and two `deadline-over-delay` requesters `dl` and `dl!`, and a sporadic task `u` using a
/// source `dev!`, either first in runtime order; each of `t`'s period, deadline and jitter past the arithmetic or not,
/// and `u`'s three past or none; each source within, past in one spelling, overflowing in two, or past and bound by
/// nothing; each requester's bound bare, within or past; and every value within the arithmetic a few milliseconds, or
/// at its edge — denominators of `10^37`, `10^38` and `8.192e37`, past it by the least a spelling reaches, spellings
/// only their reduction brings within, and comparisons whose cross-products pass 128 bits. The sources' texts,
/// `(source dev!)` before `(source dev)`, and the instances', `(adapter deadline-over-delay timer.d dl!)` before
/// `(adapter deadline-over-delay timer.d dl)`, are ordered unlike their names and their declarations, `!` lying below
/// `)` (round 14, R14-D1). So §1 names each duration past the arithmetic in its order, each kind by its texts, and a
/// duration within it is written exactly however it is spelled.
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
        uses: names(&["dl", "dl!"]),
        ..Side::default()
    };
    let mut u = Side::default();
    for (block, state, within_spellings, overflowing, states, handler, side) in [
        ("dev", z, Z_WITHIN[at], Z_OVERFLOWING[at], "", "e", &mut t),
        (
            "dev!",
            a,
            A_WITHIN[at],
            A_OVERFLOWING[at],
            "run sleep",
            "e!",
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
    for (name, state, wide) in [("dl", dl_a, WIDE), ("dl!", dl_b, TIGHT_WITHIN)] {
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
            "{} first={} t-past={}{}{} u-past={u_past} dev:{z:?} dev!:{a:?} dl:{dl_a:?} dl!:{dl_b:?}",
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
/// `uses` composition and its `needs` not (model §8 rule 5's "`uses` alone is composition"); the block `blk.n` by
/// `needs`, a node no job reaches, `dma.x` running in `u`'s (round 15, R15-R5); the system `other`; and two operands at
/// once, written against their texts' order.
const DMA_NAMING: [Naming; 10] = [
    &[],
    &[(false, "pol.p")],
    &[(true, "pol.p")],
    &[(true, "only.s")],
    &[(false, "uart.c")],
    &[(true, "uart.c")],
    &[(true, "timer.d")],
    &[(true, "blk.n")],
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
/// platform and its operand by the namer's text and then the operand's, takes no hardware the lowered system does not
/// serve (round 15, R15-R2: "This refusal and the one above take only what the system serves"), and a block naming a
/// block is lowered: its `uses` composition and its `needs` each making no edge of their own, and a binding across
/// either keeping its edge (§2.7, rule 12; round 16, R16-B2) — `dma.x`, in `u`'s job, naming by `needs` `blk.n`,
/// which no job reaches (R15-R5).
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
        // A node only as hardware bound for a requirement of its own, which nothing reaches.
        Block {
            name: "blk.n".to_string(),
            offers: names(&["(debug-port true)", "(reachable-at-privilege machine)"]),
            side: Side {
                needs: names(&["observable-output"]),
                ..Side::default()
            },
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
        platform(
            "soc",
            &["timer.d", "dev.s", "uart.c", "dma.x", "blk.n"],
            soc,
        ),
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

/// What a block of the prefixed hardware-naming universe names: nothing; the services `m.s` and `m` by `uses`, written
/// against their texts' order; the policy `m!` by `needs`; or the system `y` by `needs`.
const PREFIXED_BLOCK_NAMING: [Naming; 4] = [
    &[],
    &[(false, "m.s"), (false, "m")],
    &[(true, "m!")],
    &[(true, "y")],
];
/// What the platform `soc` names beside the blocks it composes: nothing; the service `m.s` by `needs`; the policy `m!`
/// by `uses`; or `m` by `needs` and `m!` by `uses`.
const PREFIXED_SOC_NAMING: [Naming; 4] = [
    &[],
    &[(true, "m.s")],
    &[(false, "m!")],
    &[(true, "m"), (false, "m!")],
];
/// What the block `q`, composed only by the platform of the system `y`, names: nothing; `y` by `needs`; the lowered
/// system `sys` by `uses`; or `m.s` by `uses` and `sys` by `needs`.
const Q_NAMING: [Naming; 4] = [
    &[],
    &[(true, "y")],
    &[(false, "sys")],
    &[(false, "m.s"), (true, "sys")],
];

/// **Hardware naming what is not hardware, over names one of which prefixes another** (§1; round 14, R14-D1 and
/// R14-R1): the timer `d`, to which the service `m` task `t` uses is bound, the blocks `d.x` and `d!`, and the platform
/// `soc` composing the three, of the lowered system `sys`; and the block `q`, composed only by `host.y`, the platform
/// of the system `y`. Each of `d`, `d.x` and `d!` names one choice of [`PREFIXED_BLOCK_NAMING`], `soc` one of
/// [`PREFIXED_SOC_NAMING`] and `q` one of [`Q_NAMING`]. So the namers' texts, `d`, `d!`, `d.x`, `q`, `soc`, and the
/// operands', `m`, `m!`, `m.s`, `sys`, `y`, are ordered unlike the words of their messages, one a prefix of another;
/// a platform's `needs` names a service and its `uses` a policy; a system is named — the lowered one among them — on
/// hardware the lowered system does not serve, which neither naming refusal takes since round 15 (R15-R2); and
/// served blocks naming a system meet the naming-system refusal first, their texts ordered alike — and with them `q`,
/// once `y` is reached, which serves what it writes at its own level, its `platform` clause among it (model §8 rules 1
/// and 7).
fn prefixed_hardware_naming() -> Vec<Case> {
    const NAMERS: usize = 5;
    let options = PREFIXED_BLOCK_NAMING.len();
    let mut cases = Vec::new();
    for choice in 0..options.pow(u32::try_from(NAMERS).expect("small")) {
        let pick =
            |namer: usize| choice / options.pow(u32::try_from(namer).expect("small")) % options;
        let (d, d_x, d_bang) = (
            PREFIXED_BLOCK_NAMING[pick(0)],
            PREFIXED_BLOCK_NAMING[pick(1)],
            PREFIXED_BLOCK_NAMING[pick(2)],
        );
        let (soc, q) = (PREFIXED_SOC_NAMING[pick(3)], Q_NAMING[pick(4)]);
        let words = |naming: Naming| -> String {
            let named: Vec<String> = naming
                .iter()
                .map(|(by_needs, name)| {
                    format!("{}:{name}", if *by_needs { "needs" } else { "uses" })
                })
                .collect();
            format!("[{}]", named.join(","))
        };
        let plain = |name: &str, offer: &str, named: Naming| Block {
            name: name.to_string(),
            offers: names(&[offer, "(reachable-at-privilege machine)"]),
            side: naming(Side::default(), named),
            ..Block::default()
        };
        let mut timer = timer_block("d", 2, false);
        timer.side = naming(Side::default(), d);
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
        let simple = |name: &str, policy: bool, needs: &[&str]| Service {
            name: name.to_string(),
            policy,
            side: Side {
                needs: names(needs),
                ..Side::default()
            },
        };
        cases.push(Case {
            label: format!(
                "d names {} d.x names {} d! names {} soc names {} q names {}",
                words(d),
                words(d_x),
                words(d_bang),
                words(soc),
                words(q)
            ),
            description: Description {
                blocks: vec![
                    timer,
                    plain("d.x", "(debug-port true)", d_x),
                    plain("d!", "(transfer-engine true)", d_bang),
                    plain("q", "(debug-port true)", q),
                ],
                platforms: vec![
                    platform("soc", &["d", "d.x", "d!"], soc),
                    platform("host.y", &["q"], &[]),
                ],
                services: vec![
                    simple("m", false, &["relative-delay"]),
                    simple("m.s", false, &[]),
                    simple("m!", true, &[]),
                ],
                tasks: vec![Task {
                    name: "t".to_string(),
                    side: Side {
                        uses: names(&["m"]),
                        ..Side::default()
                    },
                    ..Task::default()
                }],
                others: vec![System {
                    name: "y".to_string(),
                    tasks: vec![Task {
                        name: "z".to_string(),
                        release: Release::Period(Dur::Ms(20)),
                        ..Task::default()
                    }],
                    own: Side::default(),
                    platform: "host.y".to_string(),
                }],
                ..Description::default()
            },
        });
    }
    cases
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
/// search's conflict (model §8 rule 7); and, since round 18 (R18-D1), one a served service or task names is a node
/// bound for nothing, one only a service the system does not serve names none.
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

/// **One instance's obliged bounds** (§1, §2.4; round 14, R14-R6: "one instance's obliged bounds by the fact's text"):
/// a `deadline-over-delay` requester `dl`, which task `t` uses, writing on `delivery-bound` a bound of 5 ms or one past
/// the arithmetic, and beside it none, a second of 3 ms or a second past the arithmetic. `adapters/1`'s kinds offer one
/// obliged fact at most (`the_reading_transcribes_the_shipped_vocabulary_and_catalog` holds the shipped catalog to
/// that), so an instance owes at most one bound and no order among its facts is reachable. A requirement writing one
/// fact's bound twice gives one obliged value, "the tightest bound its requester writes on that fact" (the adapters
/// record §1, item 5), named once where it lies past the arithmetic; the search refuses (`unsupported-profile`) the
/// two where one bound lies within the arithmetic and the other past it.
fn one_instance_s_bounds() -> Vec<Case> {
    let word = |bound: Dur| {
        if bound.seconds().is_some() {
            bound.spelling()
        } else {
            "past".to_string()
        }
    };
    let mut cases = Vec::new();
    for first in [Dur::Ms(DELIVERY_BOUND_MS), Dur::Spelled(PAST)] {
        for second in [None, Some(Dur::Ms(3)), Some(Dur::Spelled(TIGHT_PAST))] {
            let mut dl = service("dl", Counter::Deadline, false, Vec::new());
            dl.side.at_most.push(("delivery-bound".to_string(), first));
            if let Some(second) = second {
                dl.side.at_most.push(("delivery-bound".to_string(), second));
            }
            cases.push(Case {
                label: format!(
                    "dl writes {} and {}",
                    word(first),
                    second.map_or_else(|| "nothing".to_string(), word)
                ),
                description: Description {
                    blocks: vec![timer_block("timer.d", 2, true)],
                    services: vec![dl],
                    tasks: vec![Task {
                        name: "t".to_string(),
                        side: Side {
                            uses: names(&["dl"]),
                            ..Side::default()
                        },
                        ..Task::default()
                    }],
                    ..Description::default()
                },
            });
        }
    }
    cases
}

/// What the timer of the own-route universe requires: nothing, `dma.x`'s `transfer-engine`, `soc`'s `core-count`, or
/// both.
const TIMER_NEEDS: [&[&str]; 4] = [
    &[],
    &["transfer-engine"],
    &["core-count"],
    &["transfer-engine", "core-count"],
];

/// **A provider bound to the node that takes its channel** (§2.6, §2.9; round 14, R14-R1): the timer `timer.d`,
/// requiring one choice of [`TIMER_NEEDS`]; the block `dma.x`, offering `transfer-engine`, and the platform `soc`,
/// offering `core-count` and composing both, each taking a channel of the timer — an alarm it programs itself, or an
/// absolute deadline through a `deadline-over-delay` instance — or none; task `t` using `xfer`, which needs
/// `transfer-engine`, `cpu`, which needs `core-count`, both or neither; and task `u` using nothing, `xfer` or `cpu`.
/// So where the timer's own requirement is bound to the node taking its channel, that node's own timer route runs the
/// timer's code, which reaches it — "nothing else that route runs reaches it" fails, and the route is one of its users
/// (§2.9: "that route's own code delivering to it is no caller, but another node's code there calling it is") — and
/// where it is not, the route delivering to it is no caller; the timer and the block it is bound to make one
/// initialisation step of two members, and so do the timer and the platform composing it where each is bound to the
/// other, a binding keeping its edge across composition (§2.7; round 16, R16-B2).
fn own_route_through_its_provider() -> Vec<Case> {
    const TAKES: [Requires; 3] = [Requires::Nothing, Requires::Alarm, Requires::Deadline];
    const T_USES: [&[&str]; 4] = [&[], &["xfer"], &["cpu"], &["xfer", "cpu"]];
    const U_USES: [&[&str]; 3] = [&[], &["xfer"], &["cpu"]];
    let mut cases = Vec::new();
    for timer_needs in TIMER_NEEDS {
        for dma in TAKES {
            for soc in TAKES {
                for t_uses in T_USES {
                    for u_uses in U_USES {
                        let mut timer = timer_block("timer.d", 4, true);
                        timer.side.needs = names(timer_needs);
                        let dma_x = Block {
                            name: "dma.x".to_string(),
                            offers: names(&[
                                "(transfer-engine true)",
                                "(reachable-at-privilege machine)",
                            ]),
                            side: Side {
                                needs: dma.needs(),
                                ..Side::default()
                            },
                            ..Block::default()
                        };
                        let platform = Block {
                            name: "soc".to_string(),
                            offers: names(&[
                                "(core-count 1 tick)",
                                "(reachable-at-privilege machine)",
                            ]),
                            side: Side {
                                needs: soc.needs(),
                                uses: names(&["timer.d", "dma.x"]),
                                ..Side::default()
                            },
                            ..Block::default()
                        };
                        let services = [("xfer", "transfer-engine"), ("cpu", "core-count")]
                            .iter()
                            .filter(|(name, _)| t_uses.contains(name) || u_uses.contains(name))
                            .map(|(name, fact)| Service {
                                name: (*name).to_string(),
                                policy: false,
                                side: Side {
                                    needs: names(&[fact]),
                                    ..Side::default()
                                },
                            })
                            .collect();
                        let task = |name: &str, priority: u32, uses: &[&str]| Task {
                            name: name.to_string(),
                            release: Release::Period(Dur::Ms(10 * u64::from(priority))),
                            priority,
                            side: Side {
                                uses: names(uses),
                                ..Side::default()
                            },
                            ..Task::default()
                        };
                        cases.push(Case {
                            label: format!(
                                "timer.d needs {timer_needs:?}; dma.x takes {dma:?}; soc takes {soc:?}; t uses \
                                 {t_uses:?}; u uses {u_uses:?}"
                            ),
                            description: Description {
                                blocks: vec![timer, dma_x],
                                platforms: vec![platform],
                                services,
                                tasks: vec![task("t", 1, t_uses), task("u", 2, u_uses)],
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

/// What each namer of the naming-order universe names: nothing; `y.b`; `y.b` and `y`; or `y!` and `y` — each pair
/// written against its texts' order, one text a prefix of the other.
const NAMED_SYSTEMS: [&[&str]; 4] = [&[], &["y.b"], &["y.b", "y"], &["y!", "y"]];

/// **The naming-system refusal's order, over names one of which prefixes another** (§1; round 13, R13-R1: "By the
/// namer's text, then the system's"; round 14, R14-D1: "a closing backquote sorts after `.`, so `y.b` is named before
/// `y`"): the lowered system `sys` with tasks `t`, first in runtime order, and `t!`; the services `m.s` and `m`,
/// declared in that order, both used by `t`; the systems `y.b`, `y!` and `y`, declared in that order; task `t`, task
/// `t!`, `m.s`, `m` and `sys`'s own level each naming one choice of [`NAMED_SYSTEMS`]; every name written by `uses` or
/// every one by `needs`; and `y.b`'s own level naming `y` or not. So the namers' texts — `(task sys t!)`, `(task sys
/// t)`, `m`, `m.s`, `sys`, and `y.b` once it is reached — and the systems' — `y`, `y!`, `y.b` — are each unlike
/// their runtime, declaration and name orders and the order their messages' words would give: `!` lies below the `)`
/// that closes `(task sys t)`, and a closing backquote above `!` and `.`. One namer names two systems, and a system
/// reached names another.
fn naming_order() -> Vec<Case> {
    const NAMERS: usize = 5;
    let mut cases = Vec::new();
    for choice in 0..NAMED_SYSTEMS
        .len()
        .pow(u32::try_from(NAMERS).expect("small"))
    {
        let pick = |namer: usize| {
            NAMED_SYSTEMS[choice
                / NAMED_SYSTEMS
                    .len()
                    .pow(u32::try_from(namer).expect("small"))
                % NAMED_SYSTEMS.len()]
        };
        let [t, t_bang, m_s, m, own] = [pick(0), pick(1), pick(2), pick(3), pick(4)];
        for by_needs in [false, true] {
            for yb_names_y in [false, true] {
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
                let system = |name: &str, own: Side| System {
                    name: name.to_string(),
                    tasks: vec![Task {
                        name: "z".to_string(),
                        release: Release::Period(Dur::Ms(20)),
                        ..Task::default()
                    }],
                    own,
                    platform: "soc".to_string(),
                };
                let service = |name: &str, named: &[&str]| Service {
                    name: name.to_string(),
                    policy: false,
                    side: side(&[], named),
                };
                cases.push(Case {
                    label: format!(
                        "t>{t:?} t!>{t_bang:?} m.s>{m_s:?} m>{m:?} sys>{own:?} by {} y.b>y={yb_names_y}",
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
                        services: vec![service("m.s", m_s), service("m", m)],
                        tasks: vec![
                            Task {
                                name: "t".to_string(),
                                priority: 1,
                                side: side(&["m.s", "m"], t),
                                ..Task::default()
                            },
                            Task {
                                name: "t!".to_string(),
                                release: Release::Period(Dur::Ms(20)),
                                priority: 2,
                                side: side(&[], t_bang),
                                ..Task::default()
                            },
                        ],
                        own: side(&[], own),
                        others: vec![
                            system(
                                "y.b",
                                if yb_names_y {
                                    side(&[], &["y"])
                                } else {
                                    Side::default()
                                },
                            ),
                            system("y!", Side::default()),
                            system("y", Side::default()),
                        ],
                        others_first: true,
                        ..Description::default()
                    },
                });
            }
        }
    }
    cases
}

/// `side` naming `declarations` too, by `needs` or by `uses`.
fn named_in(side: Side, declarations: &[&str], by_needs: bool) -> Side {
    named(side, names(declarations), by_needs)
}

/// What one requester of the bound-otherwise universe requires, of the timer `d`, which offers `observable-output`
/// too, or of `nd`, which offers `relative-delay` as `false` (round 15, R15-D2: the receivers' "bound to its provider
/// itself" named no fact, "where the code, §2.6 and model §8 rule 2 read one").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OnRoute {
    Nothing,
    /// `(needs observable-output)`: bound to `d` for a shared fact, which never takes a channel (model §8 rule 2).
    Shared,
    /// `(needs counter-width tick-rate observation-coherent)`: an extended counter only the shared `epoch-extender`
    /// instance over `d` offers, whose own binding beneath makes `d`'s timer route, "which carries the requesters bound
    /// to it" (rule 15); its `wake` runs the instance's code alone (§2.6).
    Wake,
    /// Both: named by `d`'s timer route, and bound to `d` itself for a fact that takes no channel.
    WakeShared,
    /// `(needs relative-delay)`: an alarm it programs at `d` itself.
    Alarm,
    /// `(needs relative-delay observable-output)`: an alarm it programs at `d` itself, and `d`'s output beside.
    AlarmShared,
    /// `(needs absolute-deadline observable-output)`: an expiry its own `deadline-over-delay` instance over `d`
    /// delivers, and `d`'s output beside.
    ExpiryShared,
    /// `(relative-delay false)`, met by `nd`: a consumed fact at `false`, which "takes none" (rule 2) and makes no route
    /// (rule 8: "bound through a consumed fact at `true`"). A task writes no constraint clause, so the language refuses
    /// one writing it.
    False,
}

impl OnRoute {
    fn side(self) -> Side {
        let needs: &[&str] = match self {
            Self::Nothing | Self::False => &[],
            Self::Shared => &["observable-output"],
            Self::Wake => &["counter-width", "tick-rate", "observation-coherent"],
            Self::WakeShared => &[
                "counter-width",
                "tick-rate",
                "observation-coherent",
                "observable-output",
            ],
            Self::Alarm => &["relative-delay"],
            Self::AlarmShared => &["relative-delay", "observable-output"],
            Self::ExpiryShared => &["absolute-deadline", "observable-output"],
        };
        Side {
            needs: names(needs),
            other: if self == Self::False {
                names(&["(relative-delay false)"])
            } else {
                Vec::new()
            },
            ..Side::default()
        }
    }
}

/// **Bound to a timer route's provider otherwise** (§1, §2.6, §2.9; round 15, R15-D2): tasks `t` and `u`, either first
/// in runtime order, and the system's own level, over the timer `d`, which offers `observable-output` too, and `nd`,
/// which offers `relative-delay` as `false`; each programming an alarm at `d` beside `d`'s output, an expiry beside it,
/// counting an extended counter whose instance's channel is `d`'s, with `d`'s output beside or not, or requiring `d`'s
/// output alone, or nothing; `t` and the system's own level also `(relative-delay false)`; and a service `s` both tasks
/// use, declared or not, counting the extended counter beside `d`'s output, programming an alarm at `d`, or requiring
/// `(relative-delay false)`. So §1's receivers are each "bound to its provider itself by a binding that takes a channel
/// (model §8 rule 2)", never by a binding that takes none — one a timer route names through an instance, or one the route
/// does not name, beside one programming an alarm there — §2.6's code a timer route runs is a node bound for "a consumed
/// fact", and §2.9's multiplexed node is one "that takes a channel", a consumed fact at `false` taking none.
fn bound_otherwise_on_a_route() -> Vec<Case> {
    const T: [OnRoute; 7] = [
        OnRoute::Nothing,
        OnRoute::Shared,
        OnRoute::Wake,
        OnRoute::WakeShared,
        OnRoute::AlarmShared,
        OnRoute::ExpiryShared,
        OnRoute::False,
    ];
    const U: [OnRoute; 3] = [OnRoute::Nothing, OnRoute::WakeShared, OnRoute::AlarmShared];
    const S: [Option<OnRoute>; 4] = [
        None,
        Some(OnRoute::WakeShared),
        Some(OnRoute::Alarm),
        Some(OnRoute::False),
    ];
    let mut cases = Vec::new();
    for t_requires in T {
        for u_requires in U {
            for own_requires in T {
                for s_requires in S {
                    for u_first in [false, true] {
                        let mut d = timer_block("d", 6, true);
                        let at = d.offers.len() - 2;
                        d.offers.insert(at, "(observable-output true)".to_string());
                        let nd = Block {
                            name: "nd".to_string(),
                            offers: names(&[
                                "(relative-delay false)",
                                "(reachable-at-privilege machine)",
                            ]),
                            ..Block::default()
                        };
                        let uses: &[&str] = if s_requires.is_some() { &["s"] } else { &[] };
                        let task = |name: &str, priority: u32, requires: OnRoute| Task {
                            name: name.to_string(),
                            release: Release::Period(Dur::Ms(20)),
                            priority,
                            side: Side {
                                uses: names(uses),
                                ..requires.side()
                            },
                            ..Task::default()
                        };
                        let (t_priority, u_priority) = if u_first { (2, 1) } else { (1, 2) };
                        cases.push(Case {
                            label: format!(
                                "t:{t_requires:?} u:{u_requires:?} own:{own_requires:?} s:{s_requires:?} first={}",
                                if u_first { "u" } else { "t" }
                            ),
                            description: Description {
                                blocks: vec![d, nd],
                                services: s_requires
                                    .map(|requires| Service {
                                        name: "s".to_string(),
                                        policy: false,
                                        side: requires.side(),
                                    })
                                    .into_iter()
                                    .collect(),
                                tasks: vec![
                                    task("t", t_priority, t_requires),
                                    task("u", u_priority, u_requires),
                                ],
                                own: own_requires.side(),
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

/// What one requester of the periodic universe requires, of the source `dev`, which offers `debug-port` too, or of
/// `ns`, which offers `interrupt-source` as `false` (round 15, R15-D2: the periodic task's "bound to an interrupt source
/// itself" named no fact).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum OfSource {
    Nothing,
    /// `(needs debug-port)`: bound to `dev`, a block offering `interrupt-source`, for another fact.
    Other,
    /// `(needs interrupt-source)`: bound to `dev` for its interrupt.
    Source,
    /// `(needs interrupt-source debug-port)`: both.
    Both,
    /// `(interrupt-source false)`, met by `ns`: bound for `interrupt-source`, its requirement met by `false`, which is no
    /// source (model §8 rule 8: "bound through `interrupt-source` at `true`"). A task writes no constraint clause, so
    /// the language refuses one writing it.
    False,
}

impl OfSource {
    fn side(self) -> Side {
        let needs: &[&str] = match self {
            Self::Nothing | Self::False => &[],
            Self::Other => &["debug-port"],
            Self::Source => &["interrupt-source"],
            Self::Both => &["interrupt-source", "debug-port"],
        };
        Side {
            needs: names(needs),
            other: if self == Self::False {
                names(&["(interrupt-source false)"])
            } else {
                Vec::new()
            },
            ..Side::default()
        }
    }
}

/// **A periodic task bound to a source's block otherwise** (§1, §2.2, §2.5; round 15, R15-D2): periodic tasks `t` and
/// `p`, either first in runtime order, a sporadic task `u`, and the system's own level, over the source `dev`, which
/// offers `debug-port` too, and `ns`, which offers `interrupt-source` as `false`; `t` and `p` each requiring `dev`'s
/// interrupt, its `debug-port`, both or nothing, `t` also `(interrupt-source false)`; the system's own level each of
/// those but both; and a service `e` that `u` uses, declared or not, requiring `dev`'s interrupt or `(interrupt-source
/// false)`. So §1's periodic refusal takes a task "bound itself for `interrupt-source`, its requirement met by `true`",
/// in runtime order past one bound to the same block for another fact, and a binding at `false` is no route and
/// releases no task.
fn periodic_bound_otherwise() -> Vec<Case> {
    const T: [OfSource; 5] = [
        OfSource::Nothing,
        OfSource::Other,
        OfSource::Source,
        OfSource::Both,
        OfSource::False,
    ];
    const P: [OfSource; 4] = [
        OfSource::Nothing,
        OfSource::Other,
        OfSource::Source,
        OfSource::Both,
    ];
    const OWN: [OfSource; 4] = [
        OfSource::Nothing,
        OfSource::Other,
        OfSource::Source,
        OfSource::False,
    ];
    const E: [Option<OfSource>; 3] = [None, Some(OfSource::Source), Some(OfSource::False)];
    let mut cases = Vec::new();
    for t_requires in T {
        for p_requires in P {
            for own_requires in OWN {
                for e_requires in E {
                    for p_first in [false, true] {
                        let mut dev = source_block("dev", Separation::Stated(20), "");
                        let at = dev.offers.len() - 1;
                        dev.offers.insert(at, "(debug-port true)".to_string());
                        let ns = Block {
                            name: "ns".to_string(),
                            offers: names(&[
                                "(interrupt-source false)",
                                "(reachable-at-privilege machine)",
                            ]),
                            ..Block::default()
                        };
                        let periodic = |name: &str, priority: u32, requires: OfSource| Task {
                            name: name.to_string(),
                            release: Release::Period(Dur::Ms(20)),
                            priority,
                            side: requires.side(),
                            ..Task::default()
                        };
                        let (t_priority, p_priority) = if p_first { (2, 1) } else { (1, 2) };
                        cases.push(Case {
                            label: format!(
                                "t:{t_requires:?} p:{p_requires:?} own:{own_requires:?} e:{e_requires:?} first={}",
                                if p_first { "p" } else { "t" }
                            ),
                            description: Description {
                                blocks: vec![dev, ns],
                                services: e_requires
                                    .map(|requires| Service {
                                        name: "e".to_string(),
                                        policy: false,
                                        side: requires.side(),
                                    })
                                    .into_iter()
                                    .collect(),
                                tasks: vec![
                                    periodic("t", t_priority, t_requires),
                                    periodic("p", p_priority, p_requires),
                                    Task {
                                        name: "u".to_string(),
                                        release: Release::MinSeparation(Dur::Ms(20)),
                                        priority: 3,
                                        side: Side {
                                            uses: if e_requires.is_some() {
                                                names(&["e"])
                                            } else {
                                                Vec::new()
                                            },
                                            ..Side::default()
                                        },
                                        ..Task::default()
                                    },
                                ],
                                own: own_requires.side(),
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

/// What a block or a platform the lowered system does not serve names (round 15, R15-R2): nothing; the service
/// `alarm`, which the lowered system's task uses, by `uses`; the policy `pol.p` by `needs`; the system `sys` by `uses`;
/// or the system `other` by `needs`.
const UNSERVED_NAMING: [Naming; 5] = [
    &[],
    &[(false, "alarm")],
    &[(true, "pol.p")],
    &[(false, "sys")],
    &[(true, "other")],
];
/// What `soc`, the platform of `sys`, names beside the blocks it composes (round 15, R15-R5): nothing; the policy
/// `pol.p` by `uses`; the system `other`, by `uses` or by `needs`; or the service `via.s`, which names `lone` by `uses`,
/// so that `lone` is served through what hardware the system serves names (§1: "serving what it names").
const SOC_NAMES: [Naming; 5] = [
    &[],
    &[(false, "pol.p")],
    &[(false, "other")],
    &[(true, "other")],
    &[(false, "via.s")],
];

/// **Hardware the lowered system does not serve** (§1; round 15, R15-R2 and R15-R5): the system `sys`, its task `t`
/// using `alarm` bound to `timer.d` and its task `u` using `xfer` bound to `dma.x`, on the platform `soc` composing both;
/// the system `other` on `host.q`, which composes `blk.q`; and the block `lone`, which no platform composes and only the
/// service `via.s` names. Each of `blk.q`, `host.q` and `lone` names one choice of [`UNSERVED_NAMING`], and `soc` one of
/// [`SOC_NAMES`]; each description is lowered for `sys` and for `other`. So hardware a system does not serve
/// names what is not hardware — a service it serves, a policy, a system, the lowered system itself — and is refused by
/// neither naming bullet, "nothing else reach[ing] its plan"; the same hardware, served when its own system is lowered,
/// is refused, a platform naming a system by `uses` or `needs` meeting the naming-system refusal first; a system
/// reached by a platform serves its own `platform` clause, whose platform names the lowered one; and `lone`, served
/// once `soc` names `via.s`, is refused for what it names beside `soc`.
fn unserved_hardware_naming() -> Vec<Case> {
    let options = UNSERVED_NAMING.len();
    let mut cases = Vec::new();
    for choice in 0..options.pow(3) {
        let pick = |namer: u32| UNSERVED_NAMING[choice / options.pow(namer) % options];
        let (blk_q, host_q, lone) = (pick(0), pick(1), pick(2));
        for soc in SOC_NAMES {
            let words = |naming: Naming| -> String {
                let named: Vec<String> = naming
                    .iter()
                    .map(|(by_needs, name)| {
                        format!("{}:{name}", if *by_needs { "needs" } else { "uses" })
                    })
                    .collect();
                format!("[{}]", named.join(","))
            };
            let plain = |name: &str, offer: &str, named: Naming| Block {
                name: name.to_string(),
                offers: names(&[offer, "(reachable-at-privilege machine)"]),
                side: naming(Side::default(), named),
                ..Block::default()
            };
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
            let simple = |name: &str, policy: bool, needs: &[&str]| Service {
                name: name.to_string(),
                policy,
                side: Side {
                    needs: names(needs),
                    ..Side::default()
                },
            };
            let task = |name: &str, period: u64, priority: u32, uses: &[&str]| Task {
                name: name.to_string(),
                release: Release::Period(Dur::Ms(period)),
                priority,
                side: Side {
                    uses: names(uses),
                    ..Side::default()
                },
                ..Task::default()
            };
            let sys = System {
                name: "sys".to_string(),
                tasks: vec![task("t", 10, 1, &["alarm"]), task("u", 20, 2, &["xfer"])],
                own: Side::default(),
                platform: "soc".to_string(),
            };
            let other = System {
                name: "other".to_string(),
                tasks: vec![task("o", 20, 1, &[])],
                own: Side::default(),
                platform: "host.q".to_string(),
            };
            let label = format!(
                "blk.q names {} host.q names {} lone names {} soc names {}",
                words(blk_q),
                words(host_q),
                words(lone),
                words(soc)
            );
            let lowering = |lowered: &System, beside: &System, others_first: bool| Case {
                label: format!("{label}: lowering {}", lowered.name),
                description: Description {
                    system: lowered.name.clone(),
                    blocks: vec![
                        timer_block("timer.d", 2, false),
                        plain("dma.x", "(transfer-engine true)", &[]),
                        plain("blk.q", "(debug-port true)", blk_q),
                        plain("lone", "(debug-port true)", lone),
                    ],
                    platforms: vec![
                        platform("soc", &["timer.d", "dma.x"], soc),
                        platform("host.q", &["blk.q"], host_q),
                    ],
                    services: vec![
                        simple("alarm", false, &["relative-delay"]),
                        simple("xfer", false, &["transfer-engine"]),
                        simple("pol.p", true, &[]),
                        Service {
                            name: "via.s".to_string(),
                            policy: false,
                            side: Side {
                                uses: names(&["lone"]),
                                ..Side::default()
                            },
                        },
                    ],
                    tasks: lowered.tasks.clone(),
                    own: lowered.own.clone(),
                    platform: lowered.platform.clone(),
                    others: vec![beside.clone()],
                    others_first,
                },
            };
            cases.push(lowering(&sys, &other, false));
            cases.push(lowering(&other, &sys, true));
        }
    }
    cases
}

/// What a task of the own-sources universe uses: nothing, the handler `evt.a` of the source `dev.a`, the handler
/// `evt.z` of the source `dev.z`, or both.
const HANDLERS_USED: [&[&str]; 4] = [&[], &["evt.a"], &["evt.z"], &["evt.a", "evt.z"]];

/// **Sporadic tasks and their own sources** (§2.2; round 16, R16-B1: "a sporadic task bound itself for
/// `interrupt-source` is released by that source; the routes it reaches decide only where it binds none"): the
/// sporadic task `t`, at 20 ms, bound itself for `interrupt-source` or not, and the sporadic task `u`, at 40 ms, binding
/// none; `t` using one choice of [`HANDLERS_USED`] and `u` one of the first three; the sources `dev.z` and `dev.a`, each
/// stating every class of [`SEPARATIONS`], declared in either order. Each handler is held to its own source by the
/// power states it requires, `(available-in-state …)`, which only that source offers — a qualifier judged at every
/// provider bound for the handler (model §8 rule 13); a task writes no constraint clause, so which source `t`'s own
/// binding takes is the search's, read from the record. So a task's own source decides alone: beside another source's
/// route it reaches, whether that one alone would release it every or leave it unknown; beside its own source's
/// handler, that route naming a requester it reaches too; and a task binding none, beside it, is released by the routes
/// it reaches.
fn own_sources() -> Vec<Case> {
    let mut cases = Vec::new();
    for a_first in [false, true] {
        for t_bound in [false, true] {
            for t_uses in HANDLERS_USED {
                for u_uses in &HANDLERS_USED[..3] {
                    for z in SEPARATIONS {
                        for a in SEPARATIONS {
                            cases.push(own_sources_case(
                                a_first,
                                (t_bound, t_uses),
                                u_uses,
                                (z, a),
                            ));
                        }
                    }
                }
            }
        }
    }
    cases
}

fn own_sources_case(
    a_first: bool,
    (t_bound, t_uses): (bool, &[&str]),
    u_uses: &[&str],
    (z, a): (Separation, Separation),
) -> Case {
    let mut blocks = vec![
        source_block("dev.z", z, "run idle"),
        source_block("dev.a", a, "run sleep"),
    ];
    if a_first {
        blocks.reverse();
    }
    let handler = |name: &str, states: &str| {
        let mut evt = service(name, Counter::None, true, Vec::new());
        evt.side
            .other
            .push(format!("(available-in-state {states})"));
        evt
    };
    let services = [("evt.a", "run sleep"), ("evt.z", "run idle")]
        .iter()
        .filter(|(name, _)| t_uses.contains(name) || u_uses.contains(name))
        .map(|(name, states)| handler(name, states))
        .collect();
    let sporadic = |name: &str, ms: u64, priority: u32, bound: bool, uses: &[&str]| Task {
        name: name.to_string(),
        release: Release::MinSeparation(Dur::Ms(ms)),
        priority,
        side: Side {
            needs: if bound {
                names(&[INTERRUPT_FACT])
            } else {
                Vec::new()
            },
            uses: names(uses),
            ..Side::default()
        },
        ..Task::default()
    };
    Case {
        label: format!(
            "{} first; t {}uses {t_uses:?}; u uses {u_uses:?}; dev.z:{z:?} dev.a:{a:?}",
            if a_first { "dev.a" } else { "dev.z" },
            if t_bound {
                "bound itself for interrupt-source, "
            } else {
                ""
            },
        ),
        description: Description {
            blocks,
            services,
            tasks: vec![
                sporadic("t", 20, 1, t_bound, t_uses),
                sporadic("u", 40, 2, false, u_uses),
            ],
            ..Description::default()
        },
    }
}

/// The names of the composition universe's platform and its two blocks, `[platform, block, block]`: each composite's
/// text below its part's, or above it.
const COMPOSITION_NAMES: [[&str; 3]; 2] = [["n.a", "n.m", "n.z"], ["n.z", "n.m", "n.a"]];

/// What one declaration of the composition universe requires of the other two: nothing, the first's offer, the
/// second's, or both.
const OF_THE_OTHERS: [[bool; 2]; 4] = [[false, false], [true, false], [false, true], [true, true]];

/// **A binding across composition** (§2.7; model §8 rule 12; round 16, R16-B2: "a binding keeps its edge, composition
/// alone giving none"): the platform `P`, the system's, offering `core-count`; the block `B`, offering
/// `transfer-engine`; and the block `C`, offering `debug-port`. `P` composes `B` and `C`, or `B` and, through `B`, `C`;
/// each of the three requires each choice of [`OF_THE_OTHERS`] of the other two's offers, its only provider; and the
/// three are named by one choice of [`COMPOSITION_NAMES`], each composite's text below its part's or above it. So a
/// composite is bound to its part, its part to it, or both, directly and at depth, a platform and a block each the
/// composite, under names whose order differs from the binding's: the edge a binding gives orders two steps where
/// their texts would order them the other way, and a composite and its part bound each to the other are one step.
fn bound_across_composition() -> Vec<Case> {
    const OFFERS: [(&str, &str); 3] = [
        ("core-count", "(core-count 1 tick)"),
        ("transfer-engine", "(transfer-engine true)"),
        ("debug-port", "(debug-port true)"),
    ];
    let mut cases = Vec::new();
    for names_of in COMPOSITION_NAMES {
        for deep in [false, true] {
            for choice in 0..OF_THE_OTHERS.len().pow(3) {
                let requires = |who: usize| -> Vec<String> {
                    let wanted = OF_THE_OTHERS
                        [choice / OF_THE_OTHERS.len().pow(u32::try_from(who).expect("small")) % 4];
                    let others: Vec<usize> = (0..3).filter(|i| *i != who).collect();
                    others
                        .iter()
                        .zip(wanted)
                        .filter(|(_, wanted)| *wanted)
                        .map(|(other, _)| OFFERS[*other].0.to_string())
                        .collect()
                };
                let declared = |who: usize, uses: &[&str]| Block {
                    name: names_of[who].to_string(),
                    offers: names(&[OFFERS[who].1, "(reachable-at-privilege machine)"]),
                    side: Side {
                        needs: requires(who),
                        uses: names(uses),
                        ..Side::default()
                    },
                    ..Block::default()
                };
                let [p, b, c] = names_of;
                let (p_uses, b_uses): (Vec<&str>, Vec<&str>) = if deep {
                    (vec![b], vec![c])
                } else {
                    (vec![b, c], vec![])
                };
                cases.push(Case {
                    label: format!(
                        "{p} composes {p_uses:?}, {b} composes {b_uses:?}; {p} needs {:?}, {b} needs {:?}, {c} needs \
                         {:?}",
                        requires(0),
                        requires(1),
                        requires(2)
                    ),
                    description: Description {
                        blocks: vec![declared(1, &b_uses), declared(2, &[])],
                        platforms: vec![declared(0, &p_uses)],
                        tasks: vec![Task::default()],
                        platform: p.to_string(),
                        ..Description::default()
                    },
                });
            }
        }
    }
    cases
}

/// What one requester of the obliged-bounds universe writes: no requirement, `(needs absolute-deadline)` with its
/// instance's obliged `delivery-bound` bare, with a bound of 5 ms, or with one past the arithmetic.
const DEADLINE_STATES: [Option<BoundState>; 4] = [
    None,
    Some(BoundState::Bare),
    Some(BoundState::Within),
    Some(BoundState::Past),
];

/// **The obliged bounds of every requester an instance serves** (§1, §2.4; round 16, R16-R1: the reading took "an
/// obliged bound's requester among services, tasks and hardware, never the own level"): the system's own level, the
/// service `dl` and the policy `pol` its task `t` uses, its platform `soc` and the block `blk` it composes, each
/// writing one choice of [`DEADLINE_STATES`], each met by an exclusive `deadline-over-delay` instance over the one timer.
/// So §1 names each bound past the arithmetic by its instance's text, the system's own level's among them, ahead of
/// that level's expiry, which a bound within the arithmetic leaves the refusal of its receiver (§1); and a plan writes
/// a platform's, a block's, a policy's and a service's bound on its instance's line.
fn obliged_bounds_of_every_requester() -> Vec<Case> {
    let options = DEADLINE_STATES.len();
    let mut cases = Vec::new();
    for choice in 0..options.pow(5) {
        let pick = |who: u32| DEADLINE_STATES[choice / options.pow(who) % options];
        let side = |state: Option<BoundState>| -> Side {
            let mut side = Side::default();
            if let Some(state) = state {
                side.needs.push("absolute-deadline".to_string());
                match state {
                    BoundState::Bare => {}
                    BoundState::Within => side
                        .at_most
                        .push(("delivery-bound".to_string(), Dur::Ms(DELIVERY_BOUND_MS))),
                    BoundState::Past => side
                        .at_most
                        .push(("delivery-bound".to_string(), Dur::Spelled(PAST))),
                }
            }
            side
        };
        let (own, dl, pol, soc, blk) = (pick(0), pick(1), pick(2), pick(3), pick(4));
        let mut platform = Block {
            name: "soc".to_string(),
            offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
            side: side(soc),
            ..Block::default()
        };
        platform.side.uses = names(&["timer.d", "blk"]);
        cases.push(Case {
            label: format!("own:{own:?} dl:{dl:?} pol:{pol:?} soc:{soc:?} blk:{blk:?}"),
            description: Description {
                blocks: vec![
                    timer_block("timer.d", 5, true),
                    Block {
                        name: "blk".to_string(),
                        offers: names(&[
                            "(transfer-engine true)",
                            "(reachable-at-privilege machine)",
                        ]),
                        side: side(blk),
                        ..Block::default()
                    },
                ],
                platforms: vec![platform],
                services: vec![
                    Service {
                        name: "dl".to_string(),
                        policy: false,
                        side: side(dl),
                    },
                    Service {
                        name: "pol".to_string(),
                        policy: true,
                        side: side(pol),
                    },
                ],
                tasks: vec![Task {
                    side: Side {
                        uses: names(&["dl", "pol"]),
                        ..Side::default()
                    },
                    ..Task::default()
                }],
                own: side(own),
                ..Description::default()
            },
        });
    }
    cases
}

/// What a namer of the named-hardware universe names, and by which clause.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum HardwareNamed {
    Nothing,
    BlockByUses,
    BlockByNeeds,
    PlatformByUses,
    PlatformByNeeds,
}

impl HardwareNamed {
    const ALL: [Self; 5] = [
        Self::Nothing,
        Self::BlockByUses,
        Self::BlockByNeeds,
        Self::PlatformByUses,
        Self::PlatformByNeeds,
    ];

    /// `side` naming what this names, of `block` and `platform`.
    fn name(self, side: &mut Side, block: &str, platform: &str) {
        match self {
            Self::Nothing => {}
            Self::BlockByUses => side.uses.push(block.to_string()),
            Self::BlockByNeeds => side.needs.push(block.to_string()),
            Self::PlatformByUses => side.uses.push(platform.to_string()),
            Self::PlatformByNeeds => side.needs.push(platform.to_string()),
        }
    }
}

/// The names of the named-hardware universe's service and policy, `[service, policy]`: each text below every
/// hardware text, `m.…`, so that a named edge orders its two nodes against their texts, or above them.
const NAMER_NAMES: [[&str; 2]; 2] = [["a.s", "a.p"], ["z.s", "z.p"]];

/// Who uses the named-hardware universe's service and policy.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum NamersUsed {
    /// Task `t` uses both.
    ByTask,
    /// The system's own level uses both: neither is in a job.
    ByOwnLevel,
    /// Task `t` uses the service, and nothing the policy: the system does not serve it, so it is no node, and what it
    /// names it serves none of (model §8 rule 7).
    PolicyByNothing,
}

const NAMERS_USED: [NamersUsed; 3] = [
    NamersUsed::ByTask,
    NamersUsed::ByOwnLevel,
    NamersUsed::PolicyByNothing,
];

/// **Hardware a service or a policy names** (§2.7; model §8 rule 12; round 17, R17-D1: "a service or a policy naming a
/// block or a platform gets no initialisation edge, though §2 makes that name a call"; R17-R2): the block `m.blk`,
/// offering `transfer-engine`, composed by the platform `m.soc` or by nothing; `m.soc`, the system's, offering
/// `core-count` and composing `m.out`, which offers `observable-output`; each of `m.blk` and `m.soc` requiring
/// `observable-output` or nothing; a service and a policy named by one choice of [`NAMER_NAMES`], each naming one choice
/// of [`HardwareNamed`], the service bound to `m.blk` for `transfer-engine` or not and the policy to `m.soc` for
/// `core-count` or not; and one choice of [`NAMERS_USED`]. So a block or a platform a service or a policy names is a
/// node as a provider, as bound for requirements of its own, as both, or, since round 18 (R18-D1), as named alone, its
/// edge `named` alone, and a policy the system does not serve is no namer; the edge's line holds a binding's reason
/// beside `named`, or `named` alone,
/// ordering two steps unlike their texts or like them; a block no platform composes is served through its namer alone,
/// a requester bound to it being the search's conflict (model §8 rule 7); and each node is in a job or in none, owing
/// its call cost and its masked runs where it is in one.
fn named_hardware() -> Vec<Case> {
    const BLOCK: &str = "m.blk";
    const PLATFORM: &str = "m.soc";
    let mut cases = Vec::new();
    for [s, p] in NAMER_NAMES {
        for s_names in HardwareNamed::ALL {
            for p_names in HardwareNamed::ALL {
                for (choice, users) in (0..1u32 << 5).flat_map(|c| NAMERS_USED.map(|u| (c, u))) {
                    let bit = |i: u32| choice >> i & 1 == 1;
                    let (blk_requires, s_binds, composed, soc_requires, p_binds) =
                        (bit(0), bit(1), bit(2), bit(3), bit(4));
                    let output = |requires: bool| {
                        if requires {
                            names(&["observable-output"])
                        } else {
                            Vec::new()
                        }
                    };
                    let block = Block {
                        name: BLOCK.to_string(),
                        offers: names(&[
                            "(transfer-engine true)",
                            "(reachable-at-privilege machine)",
                        ]),
                        side: Side {
                            needs: output(blk_requires),
                            ..Side::default()
                        },
                        ..Block::default()
                    };
                    let mut composes = names(&["m.out"]);
                    if composed {
                        composes.push(BLOCK.to_string());
                    }
                    let platform = Block {
                        name: PLATFORM.to_string(),
                        offers: names(&["(core-count 1 tick)", "(reachable-at-privilege machine)"]),
                        side: Side {
                            needs: output(soc_requires),
                            uses: composes,
                            ..Side::default()
                        },
                        ..Block::default()
                    };
                    let mut service = Service {
                        name: s.to_string(),
                        policy: false,
                        side: Side::default(),
                    };
                    if s_binds {
                        service.side.needs.push("transfer-engine".to_string());
                    }
                    s_names.name(&mut service.side, BLOCK, PLATFORM);
                    let mut policy = Service {
                        name: p.to_string(),
                        policy: true,
                        side: Side::default(),
                    };
                    if p_binds {
                        policy.side.needs.push("core-count".to_string());
                    }
                    p_names.name(&mut policy.side, BLOCK, PLATFORM);
                    let using = |used: &[&str]| Side {
                        uses: names(used),
                        ..Side::default()
                    };
                    let (task_side, own) = match users {
                        NamersUsed::ByTask => (using(&[s, p]), Side::default()),
                        NamersUsed::ByOwnLevel => (Side::default(), using(&[s, p])),
                        NamersUsed::PolicyByNothing => (using(&[s]), Side::default()),
                    };
                    cases.push(Case {
                        label: format!(
                            "{s} names {s_names:?}{}; {p} names {p_names:?}{}; {BLOCK} requires {}, composed {}; \
                             {PLATFORM} requires {}; used {users:?}",
                            if s_binds { ", bound to m.blk" } else { "" },
                            if p_binds { ", bound to m.soc" } else { "" },
                            if blk_requires { "output" } else { "nothing" },
                            if composed { "by m.soc" } else { "by nothing" },
                            if soc_requires { "output" } else { "nothing" },
                        ),
                        description: Description {
                            blocks: vec![
                                Block {
                                    name: "m.out".to_string(),
                                    offers: names(&[
                                        "(observable-output true)",
                                        "(reachable-at-privilege machine)",
                                    ]),
                                    ..Block::default()
                                },
                                block,
                            ],
                            platforms: vec![platform],
                            services: vec![service, policy],
                            tasks: vec![Task {
                                side: task_side,
                                ..Task::default()
                            }],
                            own,
                            platform: PLATFORM.to_string(),
                            ..Description::default()
                        },
                    });
                }
            }
        }
    }
    cases
}

/// What the namer of the interrupts' named-hardware universe requires: nothing, an alarm it programs itself, an
/// absolute deadline through an exclusive `deadline-over-delay` instance, the source, or the source beside either.
const NAMER_REQUIRES: [&[&str]; 6] = [
    &[],
    &["relative-delay"],
    &["absolute-deadline"],
    &["interrupt-source"],
    &["relative-delay", "interrupt-source"],
    &["absolute-deadline", "interrupt-source"],
];

/// **Hardware a service or a policy names, in an interrupt** (§2.6, §2.7; round 17, R17-D1): the timer `m.tmr` and the
/// source `m.src`, both composed by the platform `m.soc`; one namer, `a.x` or `z.x`, its text below the hardware's or
/// above it, a service or a policy, requiring one choice of [`NAMER_REQUIRES`] and naming one choice of
/// [`HardwareNamed`] — `m.tmr` for the block, `m.src` for the platform's place, so that it names the timer or the source
/// by `uses` or by `needs`; and used by the task `t` or by the system's own level alone. So the named block is a
/// provider bound to its namer directly, for a channel or for its interrupt, or through the instance it programs, or a
/// provider of nothing, a node as named alone since round 18 (R18-D1); and the edge from it, beside a binding's reason
/// or not, joins a namer running in the named block's own interrupt.
fn named_hardware_in_interrupts() -> Vec<Case> {
    let mut cases = Vec::new();
    for namer in ["a.x", "z.x"] {
        for requires in NAMER_REQUIRES {
            for named in HardwareNamed::ALL {
                for policy in [false, true] {
                    for by_task in [false, true] {
                        let mut side = Side {
                            needs: names(requires),
                            ..Side::default()
                        };
                        named.name(&mut side, "m.tmr", "m.src");
                        let users = Side {
                            uses: names(&[namer]),
                            ..Side::default()
                        };
                        let (task_side, own) = if by_task {
                            (users, Side::default())
                        } else {
                            (Side::default(), users)
                        };
                        cases.push(Case {
                            label: format!(
                                "{} {namer} needs {requires:?}, names {}; used by {}",
                                if policy { "policy" } else { "service" },
                                match named {
                                    HardwareNamed::Nothing => "nothing",
                                    HardwareNamed::BlockByUses => "m.tmr by uses",
                                    HardwareNamed::BlockByNeeds => "m.tmr by needs",
                                    HardwareNamed::PlatformByUses => "m.src by uses",
                                    HardwareNamed::PlatformByNeeds => "m.src by needs",
                                },
                                if by_task {
                                    "t"
                                } else {
                                    "the system's own level"
                                }
                            ),
                            description: Description {
                                blocks: vec![
                                    timer_block("m.tmr", 4, true),
                                    source_block("m.src", Separation::Stated(20), ""),
                                ],
                                platforms: vec![Block {
                                    name: "m.soc".to_string(),
                                    offers: names(&[
                                        "(core-count 1 tick)",
                                        "(reachable-at-privilege machine)",
                                    ]),
                                    side: Side {
                                        uses: names(&["m.tmr", "m.src"]),
                                        ..Side::default()
                                    },
                                    ..Block::default()
                                }],
                                services: vec![Service {
                                    name: namer.to_string(),
                                    policy,
                                    side,
                                }],
                                tasks: vec![Task {
                                    side: task_side,
                                    ..Task::default()
                                }],
                                own,
                                platform: "m.soc".to_string(),
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

/// What the unbound-hardware universe names: the block `m.blk` composed by the platform, `m.blk` composed by nothing,
/// or the platform `m.soc` itself, composing `m.blk`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Unbound {
    Composed,
    Alone,
    Platform,
}

impl Unbound {
    const ALL: [Self; 3] = [Self::Composed, Self::Alone, Self::Platform];

    /// The block or platform named.
    fn name(self) -> &'static str {
        match self {
            Self::Composed | Self::Alone => "m.blk",
            Self::Platform => "m.soc",
        }
    }

    /// The fact it offers that an unrelated requester can be bound to it for.
    fn offer(self) -> &'static str {
        match self {
            Self::Composed | Self::Alone => "transfer-engine",
            Self::Platform => "core-count",
        }
    }
}

/// The namers of the unbound-hardware universe, each naming the hardware or not: the task `t`, the service `a.s`, the
/// policy `a.p`, the source's handler `a.h` and the system's own level.
const UNBOUND_NAMERS: [&str; 5] = ["t", "a.s", "a.p", "a.h", "sys"];

/// **Named hardware bound for nothing** (§2, §2.6, §2.7, §2.10; model §8 rule 12; round 18, R18-D1: "hardware a
/// service, a policy or a task names, a call, and nothing binds places no code: no context, no step, nothing owed — and
/// placed when an unrelated binding makes it a node"; R18-D3): the block `m.blk`, offering `transfer-engine` and
/// requiring nothing, composed by the platform `m.soc` or by nothing, or `m.soc` itself, offering `core-count`,
/// composing the source `m.src` always and `m.blk` then; named, by `uses` or by `needs`, by every subset of
/// [`UNBOUND_NAMERS`] — the task `t`, the service `a.s`, the policy `a.p`, the source's handler `a.h`, bound to `m.src`
/// for `interrupt-source`, and the system's own level; the three used by `t` or by the system's own level alone; and
/// beside it nothing, or the service `w`, used by the system's own level alone and naming nothing, bound to it for its
/// offer. So hardware a served task, service or policy names is a node bound for nothing and bound to by nothing, in a
/// job, in its handler's interrupt or in neither, its edge to each service or policy naming it `named` alone, ordering
/// the hardware's step before a namer whose text is less; the system's own level naming it makes none, nor does a
/// platform composing a block nothing else names (composition is no call, §2); and a binding elsewhere, `w`'s, changes
/// nothing the name gives — the pair held line by line in its test. A block no platform composes and no task nor the
/// system's own level names is no hardware (model §8 rule 7), so `w` bound to it is the search's conflict.
fn unbound_named_hardware() -> Vec<Case> {
    let mut cases = Vec::new();
    for target in Unbound::ALL {
        for naming in 0..1u32 << UNBOUND_NAMERS.len() {
            for by_needs in [false, true] {
                for used_by_task in [false, true] {
                    for bound_elsewhere in [false, true] {
                        let names_it = |namer: &str| {
                            let at = UNBOUND_NAMERS
                                .iter()
                                .position(|n| *n == namer)
                                .expect("a namer");
                            naming >> at & 1 == 1
                        };
                        let side_of_namer = |namer: &str, mut side: Side| {
                            if names_it(namer) {
                                side = named(side, vec![target.name().to_string()], by_needs);
                            }
                            side
                        };
                        let namer = |name: &str, policy: bool, needs: &[&str]| Service {
                            name: name.to_string(),
                            policy,
                            side: side_of_namer(
                                name,
                                Side {
                                    needs: names(needs),
                                    ..Side::default()
                                },
                            ),
                        };
                        let mut services = vec![
                            namer("a.s", false, &[]),
                            namer("a.p", true, &[]),
                            namer("a.h", false, &["interrupt-source"]),
                        ];
                        let namers = names(&["a.s", "a.p", "a.h"]);
                        let (task_uses, mut own_uses) = if used_by_task {
                            (namers, Vec::new())
                        } else {
                            (Vec::new(), namers)
                        };
                        if bound_elsewhere {
                            services.push(Service {
                                name: "w".to_string(),
                                policy: false,
                                side: Side {
                                    needs: names(&[target.offer()]),
                                    ..Side::default()
                                },
                            });
                            own_uses.push("w".to_string());
                        }
                        let task_side = side_of_namer(
                            "t",
                            Side {
                                uses: task_uses,
                                ..Side::default()
                            },
                        );
                        let own = side_of_namer(
                            "sys",
                            Side {
                                uses: own_uses,
                                ..Side::default()
                            },
                        );
                        let mut composes = names(&["m.src"]);
                        if target != Unbound::Alone {
                            composes.push("m.blk".to_string());
                        }
                        let namers: Vec<&str> = UNBOUND_NAMERS
                            .iter()
                            .copied()
                            .filter(|n| names_it(n))
                            .collect();
                        cases.push(Case {
                            label: format!(
                                "{target:?} {} named by {namers:?} by {}; a.s, a.p and a.h used by {}; {}",
                                target.name(),
                                if by_needs { "needs" } else { "uses" },
                                if used_by_task {
                                    "t"
                                } else {
                                    "the system's own level"
                                },
                                if bound_elsewhere {
                                    "w bound to it"
                                } else {
                                    "nothing bound to it"
                                }
                            ),
                            description: Description {
                                blocks: vec![
                                    source_block("m.src", Separation::Stated(20), ""),
                                    Block {
                                        name: "m.blk".to_string(),
                                        offers: names(&[
                                            "(transfer-engine true)",
                                            "(reachable-at-privilege machine)",
                                        ]),
                                        ..Block::default()
                                    },
                                ],
                                platforms: vec![Block {
                                    name: "m.soc".to_string(),
                                    offers: names(&[
                                        "(core-count 1 tick)",
                                        "(reachable-at-privilege machine)",
                                    ]),
                                    side: Side {
                                        uses: composes,
                                        ..Side::default()
                                    },
                                    ..Block::default()
                                }],
                                services,
                                tasks: vec![Task {
                                    side: task_side,
                                    ..Task::default()
                                }],
                                own,
                                platform: "m.soc".to_string(),
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

/// The facts owed for each task to the application (§2.10's first row). §2.2: "Each bound and count among them is owed
/// with `ROADMAP.md` §7.3's record of a bound ... its evidence category a fact of its own" — the execution bound's
/// `execution-category`, the masked runs' `masked-category`, the stack bound's `stack-category`, and, after round 19
/// (R19-D2), "each OS call it may make, with the most one job makes, `n_{i,p}`, and those counts' evidence category",
/// `os-calls-category`.
const TASK_FACTS: [&str; 12] = [
    "entry-point",
    "execution-bound",
    "execution-category",
    "figures-under-preemption",
    "masked-category",
    "masked-runs",
    "masks-only-through-api",
    "os-calls",
    "os-calls-category",
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

/// What `requester` writes at its own level: a task's `(task SYSTEM NAME)` side, the system's own level's by the system's
/// name, a service's, a policy's, a block's or a platform's.
fn side_of(description: &Description, requester: &str) -> Option<Side> {
    if requester == description.system {
        return Some(description.own.clone());
    }
    description
        .tasks
        .iter()
        .find(|t| format!("(task {} {})", description.system, t.name) == requester)
        .map(|t| t.side.clone())
        .or_else(|| {
            description
                .services
                .iter()
                .find(|s| s.name == requester)
                .map(|s| s.side.clone())
        })
        .or_else(|| {
            description
                .hardware()
                .into_iter()
                .find(|b| b.name == requester)
                .map(|b| b.side)
        })
}

/// Whether the requirement `bind` meets is met by the value `true` (model §8 rule 2: "`(needs f)`, `(f true)` or the
/// group whose head it is. `(f false)` takes none"). An instance requires what its kind writes `beneath`, each a
/// `(needs …)` (`docs/semantics/adapters/adapters.eadl`).
fn met_by_true(description: &Description, bind: &Bind) -> bool {
    if is_instance(&bind.requester) {
        return true;
    }
    let side = side_of(description, &bind.requester)
        .unwrap_or_else(|| panic!("the reading knows no requester `{}`", bind.requester));
    if side.needs.contains(&bind.fact) {
        return true;
    }
    for clause in &side.other {
        let clause = form(clause);
        let clause = items(&clause);
        if clause.first().map(text_of).as_deref() == Some(bind.fact.as_str()) {
            return !matches!(clause.get(1), Some(Sx::Atom(value)) if value == "false");
        }
    }
    panic!(
        "the reading finds no requirement of `{}` on `{}`",
        bind.requester, bind.fact
    )
}

/// Model §8 rule 2: "A binding takes a channel when its requirement is on a consumed fact and is met by the value
/// `true` ... `(f false)` takes none, and a shared fact never does" — and so makes its provider's timer route (rule 8:
/// "a *timer*, bound through a consumed fact at `true`"; §2.9: "A channel taken on a consumed fact makes a timer
/// route").
fn takes_channel(description: &Description, bind: &Bind) -> bool {
    CONSUMED.contains(&bind.fact.as_str()) && met_by_true(description, bind)
}

/// Model §8 rule 8: "a *source*, bound through `interrupt-source` at `true`" — and §1's "bound itself for
/// `interrupt-source`, its requirement met by `true` as model §8 rule 2 reads one".
fn relies_on_source(description: &Description, bind: &Bind) -> bool {
    bind.fact == INTERRUPT_FACT && met_by_true(description, bind)
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
/// requirements of its own"; §1: "This refusal and the one above take only what the system serves"): each block or
/// platform among "a declaration its closure reaches through `uses` and `needs`" from "its own level, one of its tasks"
/// (model §8 rule 7) — its own level writing its `platform` clause too (model §8 rule 1: "what it writes at its own level
/// outside its tasks"), so its hardware (rule 7) is among them — the closure passing through every service, policy,
/// block and platform it reaches, as the joint rules do, "serving what it names" of hardware too (§1). So a block only a
/// service names is served, though it is no hardware (round 13, R13-R2: the reading had taken rule 7's "hardware" where
/// §2 says "serves"). A system named is refused before any plan (§1), so the closure goes no further into one.
fn served_hardware(description: &Description) -> BTreeSet<String> {
    served_hardware_by(description, true)
}

/// [`served_hardware`], the closure passing through what a block or a platform names that is not hardware where
/// `through_operands`, and stopping there otherwise.
fn served_hardware_by(description: &Description, through_operands: bool) -> BTreeSet<String> {
    let hardware: BTreeMap<String, Block> = description
        .hardware()
        .into_iter()
        .map(|b| (b.name.clone(), b))
        .collect();
    let mut out = BTreeSet::new();
    let mut seen = BTreeSet::new();
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
    while let Some(named) = todo.pop() {
        if !seen.insert(named.clone()) {
            continue;
        }
        if let Some(service) = description.services.iter().find(|s| s.name == named) {
            todo.extend(service.side.uses.iter().chain(&service.side.needs).cloned());
        } else if let Some(block) = hardware.get(&named) {
            out.insert(named.clone());
            todo.extend(
                block
                    .side
                    .uses
                    .iter()
                    .chain(&block.side.needs)
                    .filter(|n| through_operands || hardware.contains_key(n.as_str()))
                    .cloned(),
            );
        }
    }
    assert!(
        system_hardware(description).is_subset(&out),
        "the system's hardware is among what it serves"
    );
    out
}

/// Each block or platform `description` declares and the lowered system does not serve, with each declaration it names
/// by `uses` or `needs` that is neither a block nor a platform: hardware §1's refusal does not take (round 15, R15-R2:
/// "This refusal and the one above take only what the system serves ...: nothing else reaches its plan").
fn unserved_naming_other_kinds(description: &Description) -> Vec<(String, String)> {
    let served = served_hardware(description);
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
        if served.contains(&block.name) {
            continue;
        }
        for named in block.side.uses.iter().chain(&block.side.needs) {
            if others.contains(named) {
                out.insert((block.name.clone(), named.clone()));
            }
        }
    }
    out.into_iter().collect()
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
    let served_hardware = served_hardware(description);
    for block in hardware.values() {
        // §1: "a block or a platform the system serves naming, by `uses` or `needs`, a declaration that is neither a
        // block nor a platform" is refused before any plan; one the system does not serve may, "nothing else" reaching
        // its plan (round 15, R15-R2).
        assert!(
            !served_hardware.contains(&block.name)
                || block
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

    // Model §8 rule 7's hardware; §2's hardware the system serves, its hardware and what its closure reaches, is above.
    let system_hardware = system_hardware(description);
    // Round 15 (R15-R2): hardware the system does not serve, naming what is not hardware, and the system lowered.
    let served_services: BTreeSet<String> = {
        let mut reached = reach(&edges, system);
        for task in &description.tasks {
            reached.extend(reach(&edges, &task_text(task)));
        }
        reached
    };
    for (namer, operand) in unserved_naming_other_kinds(description) {
        let block = &hardware[&namer];
        let platform = platforms.contains(&namer);
        let by_needs = block.side.needs.contains(&operand);
        classes.insert(if systems.contains(&operand) {
            match (platform, by_needs) {
                (true, false) => "unserved: a platform's `uses` naming a system",
                (true, true) => "unserved: a platform's `needs` naming a system",
                (false, false) => "unserved: a block's `uses` naming a system",
                (false, true) => "unserved: a block's `needs` naming a system",
            }
        } else if description
            .services
            .iter()
            .any(|s| s.name == operand && s.policy)
        {
            if platform {
                "unserved: a platform naming a policy"
            } else {
                "unserved: a block naming a policy"
            }
        } else if platform {
            "unserved: a platform naming a service"
        } else {
            "unserved: a block naming a service"
        });
        if operand == system {
            classes.insert("unserved: hardware naming the lowered system");
        }
        if served_services.contains(&operand) {
            classes.insert("unserved: hardware naming a service the lowered system serves");
        }
        if !description.systems().iter().any(|s| {
            s.platform == namer
                || hardware
                    .get(&s.platform)
                    .is_some_and(|p| p.side.uses.contains(&namer))
        }) {
            classes.insert("unserved: hardware no system's platform composes");
        }
    }

    // §2: "A **node** below is a node of the joint rules' initialization graph (§2.7): a provider an active binding
    // names (a block, a platform or an adapter's instance), a block or a platform the system serves that is bound for
    // requirements of its own — a requester like any other ... — a block or a platform the system serves that a served
    // task, service or policy names by `uses` or `needs`, whose code that name calls (`M4.1.18`), a service, or a
    // policy". A task and the system's own level are requesters but never nodes. A service is the system's where the
    // system or a task of it reaches it.
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
    // Round 18 (R18-D1; `M4.1.18`): hardware a served task, service or policy names by `uses` or `needs` is a node,
    // bound or not — model §8 rule 12's "each block or platform a served task, service or policy names by `uses` or
    // `needs`, a call". Every task of the system is served (rule 7: "its own level, one of its tasks"). The system's own
    // level is no such namer: rule 12 lists it among what is "none" — it "initialises nothing" — and its `uses` "says what
    // the system includes" (rule 4); nor is a block or a platform, whose names are "never a call" (§2). Each called
    // block or platform, with who names it: the namer's text, its kind and whether by `needs`.
    let mut called: BTreeMap<String, BTreeSet<(String, &'static str, bool)>> = BTreeMap::new();
    {
        let mut call = |namer: String, kind: &'static str, side: &Side| {
            for (named, by_needs) in side
                .uses
                .iter()
                .map(|n| (n, false))
                .chain(side.needs.iter().map(|n| (n, true)))
            {
                if hardware.contains_key(named) {
                    called.entry(named.clone()).or_default().insert((
                        namer.clone(),
                        kind,
                        by_needs,
                    ));
                }
            }
        };
        for task in &description.tasks {
            call(task_text(task), "task", &task.side);
        }
        for service in description
            .services
            .iter()
            .filter(|s| served.contains(&s.name))
        {
            // A source's handler: a service or a policy bound for `interrupt-source` at `true` (§2.6; model §8 rule 8).
            let handler = binds
                .iter()
                .any(|b| b.requester == service.name && relies_on_source(description, b));
            let kind = match (service.policy, handler) {
                (_, true) => "handler",
                (true, false) => "policy",
                (false, false) => "service",
            };
            call(service.name.clone(), kind, &service.side);
        }
    }
    for (block, namers) in &called {
        assert!(
            served_hardware.contains(block),
            "what a served requester names, the system serves (model §8 rule 7)"
        );
        nodes.insert(block.clone());
        let provider = binds.iter().any(|b| &b.provider == block);
        let requiring = binds.iter().any(|b| &b.requester == block);
        if requiring {
            continue; // a node as bound for requirements of its own too, round 12's (R12-D1)
        }
        if provider {
            // "placed when an unrelated binding makes it a node" (R18-D1): a provider only of requesters that do not
            // name it, the name giving what it gives beside that binding.
            if binds
                .iter()
                .filter(|b| &b.provider == block)
                .all(|b| !namers.iter().any(|(namer, _, _)| *namer == b.requester))
            {
                classes.insert(
                    "named: hardware named and bound for nothing, a provider only of requesters that do not name it",
                );
            }
            continue;
        }
        for (_, kind, by_needs) in namers {
            classes.insert(match (*kind, platforms.contains(block)) {
                ("task", false) => "named: a block a task names, bound for nothing, a node",
                ("task", true) => "named: a platform a task names, bound for nothing, a node",
                ("service", false) => "named: a block a service names, bound for nothing, a node",
                ("service", true) => "named: a platform a service names, bound for nothing, a node",
                ("policy", false) => "named: a block a policy names, bound for nothing, a node",
                ("policy", true) => "named: a platform a policy names, bound for nothing, a node",
                (_, false) => "named: a block a source's handler names, bound for nothing, a node",
                (_, true) => {
                    "named: a platform a source's handler names, bound for nothing, a node"
                }
            });
            classes.insert(if *by_needs {
                "named: hardware named by `needs`, bound for nothing, a node"
            } else {
                "named: hardware named by `uses`, bound for nothing, a node"
            });
        }
        if !system_hardware.contains(block) {
            classes.insert(
                "named: a block no hardware, which a service or a policy names, bound for nothing, a node",
            );
        }
    }
    // What names hardware and makes it no node: the system's own level, a service or a policy the system does not
    // serve, or a block or a platform composing it, whose names are "never a call" (§2) — where nothing else makes it
    // one.
    for (name, _) in hardware.iter().filter(|(name, _)| !nodes.contains(*name)) {
        let names_it = |side: &Side| side.uses.iter().chain(&side.needs).any(|n| n == name);
        if called.keys().any(|c| hardware[c].side.uses.contains(name)) {
            classes.insert(
                "named: a block hardware a served requester names composes, nothing calling nor binding it, no node",
            );
        }
        if names_it(&description.own) {
            classes.insert("named: hardware the system's own level names, no node");
        }
        if description
            .services
            .iter()
            .any(|s| !served.contains(&s.name) && names_it(&s.side))
        {
            classes.insert(
                "named: hardware a service or a policy the system does not serve names, no node",
            );
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
    // for `interrupt-source` at `true` relies on its source; one bound for a consumed fact at `true`, taking a channel,
    // on its alarm (model §8 rules 2 and 8) — a fact at `false`, or a shared fact, makes no route. An instance is no
    // route of its own: the route is its provider's, and names the requesters bound to the instance (the worked epoch
    // extender: "the route names the requester bound to the instance, `time.monotonic`"; rule 15: "its own binding
    // beneath making its device's timer route, which carries the requesters bound to it").
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
        let source = relies_on_source(description, bind);
        let timer = takes_channel(description, bind);
        if bind.fact == INTERRUPT_FACT && !source {
            classes.insert(
                "bound otherwise: a requester bound for `interrupt-source` at `false`, no route",
            );
        }
        if CONSUMED.contains(&bind.fact.as_str()) && !timer {
            classes.insert(
                "bound otherwise: a requester bound for a consumed fact at `false`, no channel",
            );
        }
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
    for instance in &instances {
        let kind = kind_of(instance);
        let mut line = format!("(instance {instance}");
        let instance_form = form(instance);
        let requester = items(&instance_form).get(3).map(text_of);
        for fact in kind.obliged {
            let bound = requester
                .as_deref()
                .and_then(|r| side_of(description, r))
                .and_then(|side| {
                    side.at_most
                        .iter()
                        .filter(|(f, _)| f == fact)
                        .map(|(_, bound)| {
                            bound.seconds().expect(
                                "a bound past the arithmetic is refused before any plan (§1)",
                            )
                        })
                        .min()
                });
            match bound {
                Some(time) => {
                    classes.insert("instance: an obliged fact with its bound");
                    let requester = requester.as_deref().expect("a bound's requester");
                    // The bound's requester, of every kind (round 17, R17-R6: "a service's bound within the arithmetic
                    // generated and counted by no class").
                    classes.insert(if platforms.contains(requester) {
                        "instance: a platform's obliged bound"
                    } else if hardware.contains_key(requester) {
                        "instance: a block's obliged bound"
                    } else if let Some(service) =
                        description.services.iter().find(|s| s.name == requester)
                    {
                        if service.policy {
                            "instance: a policy's obliged bound"
                        } else {
                            "instance: a service's obliged bound"
                        }
                    } else {
                        // A task or the system's own level: the instance's expiry runs its code, which §1 refuses
                        // before any plan ("an adapter instance with a cost running its requester's code (`expiry`)
                        // whose requester it is").
                        "unreachable: a task's or the system's own level's obliged bound in a plan, §1 refusing the \
                         expiry it would receive"
                    });
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
            // "for the interrupt fact where the route is a source, or for a consumed fact where it is a timer route":
            // the binding the route names it for (model §8 rules 2 and 8).
            let for_this_kind = if route.source {
                relies_on_source(description, bind)
            } else {
                takes_channel(description, bind)
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
    // Round 18 (R18-D1): hardware a served requester names, bound for nothing and bound to by nothing, runs where its
    // namers run — "`job` ... for every node a task reaches", and "every node one of these reaches" in an interrupt
    // (§2.6) — or, its namers used by the system's own level alone, in no job.
    for block in called.keys().filter(|b| {
        !binds
            .iter()
            .any(|bind| &bind.provider == *b || &bind.requester == *b)
    }) {
        classes.insert(if in_a_job.contains(block) {
            "named: hardware named and bound for nothing, in a job"
        } else {
            "named: hardware named and bound for nothing, in no job"
        });
        if runs.values().any(|run| run.contains(block)) {
            classes.insert(
                "named: hardware named and bound for nothing, in an interrupt that runs its namer",
            );
        }
    }

    // §2.2, what releases each task: `timer` for a periodic task; for a task with a minimum separation, "the source it
    // is bound to itself for `interrupt-source` met by `true`, where it is bound to one ... and otherwise the external
    // source whose route names the task or a requester it reaches" (round 16, R16-B1), `(source NAME every)` "when the
    // source's stated separation is at least the task's"; `unknown` otherwise — "where the task binds no source itself
    // and no source's route reaches it, or several do", a source stating no separation or declaring it absent, or one
    // shorter than the task's. "A timer route releases no task." So the routes a task reaches decide only where it binds
    // no source itself: its own source decides alone, whatever else reaches it.
    let mut released_by = BTreeMap::new();
    for task in &description.tasks {
        let trigger = match task.release {
            Release::Period(_) => "timer".to_string(),
            Release::MinSeparation(own) => {
                let own = own
                    .seconds()
                    .expect("a separation past the arithmetic is refused before any plan (§1)");
                // What the source `route` gives the task: its trigger, and the class of §2.2's rule it meets.
                let decide = |route: &Route| -> (String, &'static str) {
                    match separation_of(&route.provider) {
                        Sep::Stated(time) if time >= own => (
                            format!("(source {} every)", route.provider),
                            if time == own {
                                "trigger: every, equal separations"
                            } else {
                                "trigger: every, a longer separation"
                            },
                        ),
                        Sep::Stated(_) => (
                            "unknown".to_string(),
                            "trigger: unknown, a shorter separation",
                        ),
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
                };
                let mut reached = reach(&edges, &task_text(task));
                reached.insert(task_text(task));
                let sources: Vec<&Route> = routes
                    .values()
                    .filter(|r| r.source && r.requesters.iter().any(|q| reached.contains(q)))
                    .collect();
                // The source the task is bound to itself, for `interrupt-source` met by `true` (model §8 rule 2): one
                // requirement, so one binding, at most.
                let bound_itself: Vec<&Bind> = binds
                    .iter()
                    .filter(|b| {
                        b.requester == task_text(task)
                            && !is_instance(&b.provider)
                            && relies_on_source(description, b)
                    })
                    .collect();
                assert!(
                    bound_itself.len() <= 1,
                    "one requirement on `interrupt-source`, one binding"
                );
                let mut deciding: Option<&Route> = None;
                let (trigger, class) = if let Some(own_source) = bound_itself.first() {
                    let route = &routes[&format!("(source {})", own_source.provider)];
                    classes.insert("trigger: its own source");
                    let others: Vec<&&Route> = sources
                        .iter()
                        .filter(|r| r.provider != route.provider)
                        .collect();
                    let decided = decide(route);
                    if others.is_empty() {
                        classes.insert("trigger: its own source, no other reaching it");
                    } else {
                        classes.insert("trigger: its own source, beside another reaching it");
                    }
                    // Another alone would decide otherwise: the record's own source, never the routes.
                    if let [other] = others.as_slice() {
                        if decide(other).0 != decided.0 {
                            classes.insert(if decided.0 == "unknown" {
                                "trigger: its own source leaving it unknown, beside one that alone would release \
                                 it every"
                            } else {
                                "trigger: its own source releasing it every, beside one that alone would leave it \
                                 unknown"
                            });
                        }
                    }
                    if route
                        .requesters
                        .iter()
                        .any(|q| *q != task_text(task) && reached.contains(q))
                    {
                        classes.insert(
                            "trigger: its own source, reached through a requester its route names too",
                        );
                    }
                    deciding = Some(route);
                    decided
                } else {
                    // Beside another task bound to a source itself: this one, binding none, is released by the routes
                    // it reaches, one of them that task's own source where it reaches its handler.
                    let others_own: BTreeSet<&String> = binds
                        .iter()
                        .filter(|b| {
                            b.requester.starts_with("(task ")
                                && b.requester != task_text(task)
                                && !is_instance(&b.provider)
                                && relies_on_source(description, b)
                        })
                        .map(|b| &b.provider)
                        .collect();
                    if !others_own.is_empty() {
                        classes
                            .insert("trigger: binding none, beside a task bound to its own source");
                        if sources.iter().any(|r| others_own.contains(&r.provider)) {
                            classes.insert(
                                "trigger: binding none, reaching the handler of another task's own source",
                            );
                        }
                    }
                    match sources.as_slice() {
                        [one] => {
                            if one.requesters.iter().any(|q| hardware.contains_key(q)) {
                                classes.insert("trigger: a source a block or a platform handles");
                            }
                            deciding = Some(*one);
                            decide(one)
                        }
                        [] => (
                            "unknown".to_string(),
                            "trigger: unknown, no source reaching it",
                        ),
                        _ => ("unknown".to_string(), "trigger: unknown, several sources"),
                    }
                };
                if let Some(Sep::Stated(time)) = deciding.map(|r| separation_of(&r.provider)) {
                    if past_128_bits(time, own) {
                        classes.insert(
                            "exact: a source's and a task's separations compared past 128 bits",
                        );
                    }
                }
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
    // bound to it, whatever composes what, but to itself — composition alone giving none (`M4.1.16`) — and from each
    // node a service or a policy names by `uses` or `needs`, a block or a platform included (`M4.1.17`), to that namer
    // — a block's or a platform's own names giving none; each carries its reason", between nodes — "the system's own
    // level ... and its tasks ... are none". §2.7: "naming each reason `Joint::initialization` gives, a binding's fact or
    // `named` for a `uses` or `needs` — a block's or a platform's `uses` or `needs` giving none, hardware's names never a
    // call (§2), while a service's or a policy's naming a block or a platform, a call, gives one (`M4.1.17`);
    // composition, a `uses`, gives no edge either, while a binding keeps its edge whatever composes what, a composite
    // and its part each bound to the other being one component" (round 16, R16-B2; round 17, R17-D1). Composition is a
    // block's or a platform's `uses` alone (rule 1; rule 5: "names it through `uses`, at any depth through blocks and
    // platforms, whose `uses` alone is composition"): it gives no edge of its own, and takes none a binding gives. An
    // instance composes nothing, and "the initialization graph keeps its edges, its device before it and it before its
    // requesters" (rule 15). A block's or a platform's names make no named edge; one naming anything but hardware is
    // refused (§1). A block or a platform a served service or policy names is a node whatever binds it (§2; round 18,
    // R18-D1, `M4.1.18`), so its named edge stands bound or not; a task naming one gives none, a task being no node.
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
    // The edges a binding gives across composition, `(earlier, later)`: the provider before its requester.
    let mut across: BTreeSet<(String, String)> = BTreeSet::new();
    for bind in binds {
        if nodes.contains(&bind.requester)
            && nodes.contains(&bind.provider)
            && bind.requester != bind.provider
        {
            let composite_bound = composes(&bind.requester).contains(&bind.provider);
            let part_bound = composes(&bind.provider).contains(&bind.requester);
            if composite_bound || part_bound {
                across.insert((bind.provider.clone(), bind.requester.clone()));
                let (composite, part) = if composite_bound {
                    classes.insert("initialisation: a composite bound to its part, its edge kept");
                    (&bind.requester, &bind.provider)
                } else {
                    classes.insert("initialisation: a part bound to its composite, its edge kept");
                    (&bind.provider, &bind.requester)
                };
                if !hardware[composite].side.uses.contains(part) {
                    classes.insert(
                        "initialisation: an edge across composition at depth, through another part, kept",
                    );
                }
                if !platforms.contains(composite) {
                    classes.insert(
                        "initialisation: a block composing a block, an edge across it kept",
                    );
                }
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
    // The named edges from hardware, `(earlier, later)`: the block or the platform before the service or the policy
    // naming it.
    let mut named_hardware: BTreeSet<(String, String)> = BTreeSet::new();
    for namer in description
        .services
        .iter()
        .filter(|s| nodes.contains(&s.name))
    {
        for (named, by_needs) in namer
            .side
            .uses
            .iter()
            .map(|n| (n, false))
            .chain(namer.side.needs.iter().map(|n| (n, true)))
        {
            if !declared(named) || *named == namer.name {
                continue;
            }
            if !hardware.contains_key(named) {
                if nodes.contains(named) {
                    graph
                        .entry((named.clone(), namer.name.clone()))
                        .or_default()
                        .named = true;
                    classes.insert("initialisation: a named edge");
                }
                continue;
            }
            // Round 18 (R18-D1) inverted round 17's class, "hardware a service or a policy names, no node, no edge": §2
            // makes every block or platform a served service or policy names a node, bound or not, as `called` reads it
            // above — the reading's own construction, so asserted rather than counted.
            assert!(
                nodes.contains(named),
                "hardware a served service or policy names is a node (§2, `M4.1.18`)"
            );
            graph
                .entry((named.clone(), namer.name.clone()))
                .or_default()
                .named = true;
            named_hardware.insert((named.clone(), namer.name.clone()));
            classes.insert(match (namer.policy, platforms.contains(named), by_needs) {
                (false, false, false) => {
                    "initialisation: a named edge from a block a service names by `uses`"
                }
                (false, false, true) => {
                    "initialisation: a named edge from a block a service names by `needs`"
                }
                (false, true, false) => {
                    "initialisation: a named edge from a platform a service names by `uses`"
                }
                (false, true, true) => {
                    "initialisation: a named edge from a platform a service names by `needs`"
                }
                (true, false, false) => {
                    "initialisation: a named edge from a block a policy names by `uses`"
                }
                (true, false, true) => {
                    "initialisation: a named edge from a block a policy names by `needs`"
                }
                (true, true, false) => {
                    "initialisation: a named edge from a platform a policy names by `uses`"
                }
                (true, true, true) => {
                    "initialisation: a named edge from a platform a policy names by `needs`"
                }
            });
            let provider = binds.iter().any(|b| &b.provider == named);
            let requiring = binds.iter().any(|b| &b.requester == named);
            classes.insert(match (provider, requiring) {
                (true, false) => "initialisation: a named edge from hardware a provider alone",
                (false, true) => {
                    "initialisation: a named edge from hardware bound for its own requirements alone"
                }
                (true, true) => {
                    "initialisation: a named edge from hardware a provider and bound for its own requirements"
                }
                (false, false) => {
                    "initialisation: a named edge from hardware bound for nothing and bound to by nothing"
                }
            });
            if !system_hardware.contains(named) {
                classes.insert(
                    "initialisation: a named edge from a block no hardware, served through its namer",
                );
            }
            // Beside the path a binding gives through an instance: the device, then the instance, then the namer.
            if binds.iter().any(|beneath| {
                &beneath.provider == named
                    && is_instance(&beneath.requester)
                    && binds
                        .iter()
                        .any(|b| b.requester == namer.name && b.provider == beneath.requester)
            }) {
                classes.insert(
                    "initialisation: a named edge from hardware an instance programs, beside the instance's path",
                );
            }
            if routes
                .values()
                .any(|r| &r.provider == named && runs[&r.item].contains(&namer.name))
            {
                classes.insert(
                    "initialisation: a named edge from hardware whose own interrupt runs its namer",
                );
            }
        }
    }
    // A service or a policy the system does not serve is no node, so no edge reaches it, whatever it names.
    if description.services.iter().any(|s| {
        !nodes.contains(&s.name)
            && s.side
                .uses
                .iter()
                .chain(&s.side.needs)
                .any(|n| hardware.contains_key(n.as_str()) && nodes.contains(n))
    }) {
        classes.insert(
            "initialisation: hardware a node, named by a service or a policy the system does not serve, no edge",
        );
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
    // Round 16 (R16-B2): "a composite and its part each bound to the other being one component"; and an edge across
    // composition between two components, its earlier node's text the greater, so that the texts alone would order the
    // two the other way.
    for (earlier, later) in &across {
        if reached[earlier].contains(later) && reached[later].contains(earlier) {
            if across.contains(&(later.clone(), earlier.clone())) {
                classes.insert(
                    "initialisation: a composite and its part bound each to the other, one component",
                );
            }
        } else if earlier > later {
            classes.insert(
                "initialisation: an edge across composition ordering two steps unlike their texts",
            );
        }
    }
    // Round 17 (R17-D1): a named edge from hardware on the line a binding's reason holds too, `(binding …)` reasons
    // first, then `named` (§3); and one, `named` its one reason and the graph's one path between its two nodes, that
    // orders the earlier, whose text is the greater, before the later.
    for (earlier, later) in &named_hardware {
        if !graph[&(earlier.clone(), later.clone())].facts.is_empty() {
            classes
                .insert("initialisation: a named edge from hardware beside a binding's, one line");
            continue;
        }
        let mut reached = BTreeSet::new();
        let mut todo = vec![earlier.clone()];
        while let Some(at) = todo.pop() {
            for (from, to) in graph.keys() {
                if *from == at && !(from == earlier && to == later) && reached.insert(to.clone()) {
                    todo.push(to.clone());
                }
            }
        }
        if !reached.contains(later) && earlier > later {
            classes.insert(
                "initialisation: a named edge from hardware, the one path between two steps, ordering them unlike \
                 their texts",
            );
        }
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
            .filter(|b| &b.requester == node && takes_channel(description, b))
            .collect();
        if channels.is_empty() {
            continue; // it takes no channel (model §8 rule 2)
        }
        let mut own_routes = BTreeSet::new();
        for channel in channels {
            if is_instance(&channel.provider) {
                for programmed in binds
                    .iter()
                    .filter(|b| b.requester == channel.provider && takes_channel(description, b))
                {
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
                    // The route's provider, `(timer-route X)`'s `X`, bound for a requirement of its own to the node.
                    let provider = &item["(timer-route ".len()..item.len() - 1];
                    if binds
                        .iter()
                        .any(|b| b.requester == provider && &b.provider == node)
                    {
                        classes.insert(
                            "multiplex: its own route a user, the route's provider bound to it",
                        );
                    }
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
        // "each task | `entry-point`, ... `os-calls`, `os-calls-category`, ... | `application`" — every task, whatever
        // it is released by or calls (§2.2, §2.10; round 19, R19-D2).
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
    // "each node | `contexts`, `no-wait`, `runtime-discipline` | `realization`" — the last the composite-inputs record's
    // §2 fact for the code the realization generates for it (round 16, R16-D3); and "each node whose context line holds
    // `job` | `call-cost`, the time one call of its code takes in a job; `masked-runs`, its code's masked runs there |
    // `realization`" — "A node in a job owes its masked runs as a task does" (round 17, R17-R2). No category is owed
    // beside them: "Every bound owed to the realization — a call cost, masked runs, a cost's terms, an overhead, a
    // service cost, an arrival bound, an obliged fact — is owed with `ROADMAP.md` §7.3's record of a bound ... The
    // realization states the record with the bound", while "The application's categories are facts of their own"
    // (§2.10; round 18, R18-D3) — so a task's `masked-category` stays, beside its `os-calls-category` (round 19,
    // R19-D2), and a node owes none.
    for node in &nodes {
        owe(node, "contexts", "realization");
        owe(node, "no-wait", "realization");
        owe(node, "runtime-discipline", "realization");
        if in_a_job.contains(node) {
            owe(node, "call-cost", "realization");
            owe(node, "masked-runs", "realization");
            classes.insert(if is_instance(node) {
                "owed: an instance in a job, its call cost and masked runs"
            } else if hardware.contains_key(node) {
                "owed: a block or a platform in a job, its call cost and masked runs"
            } else {
                "owed: a service or a policy in a job, its call cost and masked runs"
            });
        } else {
            classes.insert("owed: a node in no job, neither its call cost nor masked runs");
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
    /// task it applies to is the one given; with the clause, where §1 names one — "naming the task and the clause —
    /// for the last, its `period`" (round 14, R14-R5).
    Task {
        task: String,
        clause: Option<&'static str>,
    },
    /// The naming-system refusal: each requester the system serves that names a system, with the system it names —
    /// "naming each such name, by the namer's text and then the system's" (round 13, R13-R1), texts compared byte by
    /// byte (§2).
    Systems(Vec<(String, String)>),
    /// The refusal of hardware naming what is not hardware: each block or platform the system serves whose `uses` or
    /// `needs` names a declaration that is neither, with that operand — "naming each, by the block's or platform's text
    /// and then the operand's".
    Operands(Vec<(String, String)>),
    /// The durations past the arithmetic, "naming each — the tasks' in runtime order, then the sources' by their
    /// interrupt's text, then the bounds by their instance's text and then the fact's", each task's clauses "in the
    /// order the bullet writes them": a task's as `(task SYSTEM NAME) CLAUSE`, a source's as `(source NAME)`, a
    /// bound's as `INSTANCE FACT`.
    Durations(Vec<String>),
    /// The interrupt-code refusal: every receiver, in §1's order — "each task in runtime order, then the system's own
    /// level" — each with what it would receive.
    Receivers(Vec<Receiver>),
}

/// One receiver of interrupt code (§1): a task's `(task SYSTEM NAME)`, or the system's name for its own level, with
/// what it would receive — "first the alarms it programs, each naming the provider, then its expiries, each naming
/// the instance that delivers it, each kind by those texts" (round 14, R14-R6). A set iterates by its texts, each
/// compared byte by byte; an alarm is one for each provider, "an interrupt ... named by its kind and its provider"
/// (§2), however many consumed facts the receiver binds there.
struct Receiver {
    subject: String,
    alarms: BTreeSet<String>,
    expiries: BTreeSet<String>,
}

impl Receiver {
    /// What it would receive, in §1's order: its alarms' providers, then its expiries' instances.
    fn received(&self) -> impl Iterator<Item = &String> {
        self.alarms.iter().chain(&self.expiries)
    }
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
            Names::Task { task, clause } => {
                clause.map_or_else(|| task.clone(), |clause| format!("{task} {clause}"))
            }
            Names::Systems(pairs) | Names::Operands(pairs) => pairs
                .iter()
                .map(|(namer, named)| format!("{namer} naming {named}"))
                .collect::<Vec<_>>()
                .join(", "),
            Names::Durations(items) => items.join(", "),
            Names::Receivers(receivers) => receivers
                .iter()
                .map(|r| {
                    format!(
                        "{} receiving {}",
                        r.subject,
                        r.received().cloned().collect::<Vec<_>>().join(" then ")
                    )
                })
                .collect::<Vec<_>>()
                .join(", "),
        }
    }
}

/// Whether one of `texts` is a proper prefix of another (round 14, R14-R1: "names one of which prefixes another").
fn prefixed<'a>(texts: impl IntoIterator<Item = &'a String> + Clone) -> bool {
    texts.clone().into_iter().any(|a| {
        texts
            .clone()
            .into_iter()
            .any(|b| b.len() > a.len() && b.starts_with(a.as_str()))
    })
}

/// Whether `text` holds a byte a symbol may hold below `)` — `!`, `#`, `$`, `%`, `&` or `'`
/// (`docs/semantics/grammar.md`'s `symbol_char`) — so that a name continuing with it sorts before the `)` that closes a
/// shorter one's form.
fn below_paren(text: &str) -> bool {
    text.bytes()
        .any(|b| matches!(b, b'!' | b'#' | b'$' | b'%' | b'&' | b'\''))
}

/// Whether `items`, held in the order §1 states — each by its text, compared byte by byte — would be ordered otherwise
/// by `worded`, the words a message names each with (round 14, R14-D1: "a closing backquote sorts after `.`, so `y.b`
/// is named before `y`").
fn worded_otherwise<T: Clone>(items: &[T], worded: impl Fn(&T) -> String) -> bool {
    let mut by_words = items.to_vec();
    by_words.sort_by_key(|item| worded(item));
    by_words.iter().map(&worded).collect::<Vec<_>>()
        != items.iter().map(&worded).collect::<Vec<_>>()
}

/// The classes of round 14 (R14-D1, R14-R1) a refusal naming pairs reaches, `pairs` held in §1's order — "by the
/// namer's text and then the system's" for `naming`, "by the block's or platform's text and then the operand's" for
/// `operand`: one namer's text a prefix of another's; one named text a prefix of another's; a name continuing with a
/// byte below `)`; tasks' texts, `(task SYSTEM NAME)`, ordered unlike their names; and an order the message's words
/// would give otherwise, each pair worded as `` `NAMER` names `OPERAND` ``.
fn name_order_classes(
    pairs: &[(String, String)],
    refusal: &str,
    classes: &mut BTreeSet<&'static str>,
) {
    let pick = |naming: &'static str, operand: &'static str| {
        if refusal == "naming" {
            naming
        } else {
            operand
        }
    };
    let namers: BTreeSet<&String> = pairs.iter().map(|(namer, _)| namer).collect();
    let named: BTreeSet<&String> = pairs.iter().map(|(_, named)| named).collect();
    if prefixed(namers.iter().copied()) {
        classes.insert(pick(
            "naming: one namer's text a prefix of another's",
            "operand: one namer's text a prefix of another's",
        ));
    }
    if prefixed(named.iter().copied()) {
        classes.insert(pick(
            "naming: one system's text a prefix of another's",
            "operand: one operand's text a prefix of another's",
        ));
    }
    if namers.iter().chain(&named).any(|text| below_paren(text)) {
        classes.insert(pick(
            "naming: a name continuing with a byte below `)`",
            "operand: a name continuing with a byte below `)`",
        ));
    }
    let tasks: Vec<&String> = namers
        .iter()
        .copied()
        .filter(|namer| namer.starts_with("(task "))
        .collect();
    if worded_otherwise(&tasks, |task| {
        task.trim_end_matches(')')
            .rsplit(' ')
            .next()
            .expect("a name")
            .to_string()
    }) {
        classes.insert(pick(
            "naming: tasks' texts ordered unlike their names",
            // §1's hardware-naming bullet names "a block or a platform the system serves", never a task.
            "unreachable: a task among the namers of hardware naming what is not hardware, blocks and platforms alone",
        ));
    }
    if worded_otherwise(pairs, |(namer, named)| format!("`{namer}` names `{named}`")) {
        classes.insert(pick(
            "naming: an order the message's words would give otherwise",
            "operand: an order the message's words would give otherwise",
        ));
    }
}

/// §1: "a declaration or a task the system serves naming a system by `uses` or `needs`", each with the system it
/// names, "by the namer's text and then the system's". The requesters a system serves are "its own level, one of its
/// tasks, or a declaration its closure reaches through `uses` and `needs`" (model §8 rule 7) — round 12's R12-R1
/// scoped the refusal to them — and its hardware, §2's "a block or a platform the system serves". A system so reached
/// is a requester "for what it writes at its own level outside its tasks" (model §8 rule 1), its tasks being that
/// system's: its `requires`, and its `platform` clause, `(platform (uses P))`, whose `uses` the closure follows as it
/// follows the lowered system's own, through `P` to the blocks `P` composes (round 14's widening; the second widening
/// had stopped at its `requires`, which the joint rules do not).
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
                let mut own = other.own.clone();
                own.uses.push(other.platform.clone());
                todo.push((named.clone(), own));
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

/// §1: "a block or a platform the system serves naming, by `uses` or `needs`, a declaration that is neither a block nor
/// a platform", "naming each, by the block's or platform's text and then the operand's". Since round 15 (R15-R2) it
/// takes "only what the system serves, as the bullet over tasks takes its tasks: nothing else reaches its plan", as the
/// naming-system bullet before it does: a block or a platform of another system, or of none, is read by neither.
fn hardware_naming_other_kinds(description: &Description) -> Vec<(String, String)> {
    let served = served_hardware(description);
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
        if !served.contains(&block.name) {
            continue;
        }
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
/// - Of the first list, in its order: "a periodic task bound itself for `interrupt-source`, its requirement met by
///   `true` as model §8 rule 2 reads one, which states a second release model": `tool-failure`, "naming the task and
///   the clause"; then "a declaration or a task the system serves naming a system by `uses` or `needs`":
///   `tool-failure`, "naming each such name, by the namer's text and then the system's"; then "a block or a platform the
///   system serves naming, by `uses` or `needs`, a declaration that is neither a block nor a platform": `tool-failure`,
///   "naming each, by the block's or platform's text and then the operand's" — "This refusal and the one above take
///   only what the system serves". (`M1.43`, `M1.46` and `M1.47` have the language refuse them, and "a description the
///   language's check refuses is no input".)
/// - Of the second, `unsupported-profile`, in its order: "a duration the plan would write past the engine's exact
///   arithmetic (§2.2): a task's period, minimum separation, deadline or jitter, a source's separation, or an obliged
///   bound, naming each — the tasks' in runtime order, then the sources' by their interrupt's text, then the bounds by
///   their instance's text"; "a task whose deadline is measured from an event other than its release"; and "a task, or
///   the system's own level, that would receive an interrupt's code: a timer route that names it, bound to its
///   provider itself by a binding that takes a channel (model §8 rule 2) — an alarm it programs, as §2.6 reads that
///   binding — or an adapter instance with a cost running its requester's code (`expiry`) whose requester it is, the one
///   an exclusive instance serves or one bound to a shared instance. ... It names every such receiver — each task in
///   runtime order, then the system's own level — with what it would receive".
///
/// No other bullet applies here: no system has placement memory and each declares a task.
fn refusal_the_record_gives(description: &Description, binds: &[Bind]) -> Option<Due> {
    let system = description.system.as_str();
    let task_text = |task: &Task| format!("(task {system} {})", task.name);
    let mut by_index: Vec<&Task> = description.tasks.iter().collect();
    by_index.sort_by_key(|t| t.priority);
    // A periodic task bound to a source's block, `interrupt-source` true among its offers, for another fact: "bound
    // itself for `interrupt-source`" fails, and it is no second release model (round 15, R15-D2).
    let bound_otherwise = |task: &Task| {
        binds.iter().any(|b| {
            b.requester == task_text(task)
                && !relies_on_source(description, b)
                && description
                    .hardware()
                    .iter()
                    .any(|h| h.name == b.provider && h.separation.is_some())
        })
    };
    for (index, task) in by_index.iter().enumerate() {
        if matches!(task.release, Release::Period(_))
            && binds
                .iter()
                .any(|b| b.requester == task_text(task) && relies_on_source(description, b))
        {
            let mut classes = BTreeSet::from(["refusal: a periodic task bound to a source itself"]);
            if !hardware_naming_other_kinds(description).is_empty() {
                classes.insert("periodic: before hardware naming what is not hardware");
            }
            if by_index[..index]
                .iter()
                .any(|t| matches!(t.release, Release::Period(_)) && bound_otherwise(t))
            {
                classes.insert(
                    "periodic: one bound to the source's block otherwise first in runtime order, the one bound for \
                     `interrupt-source` named",
                );
            }
            if bound_otherwise(task) {
                classes.insert(
                    "periodic: the one named bound to the source's block for another fact too",
                );
            }
            return Some(Due {
                code: "tool-failure",
                names: Names::Task {
                    task: task_text(task),
                    clause: Some("period"),
                },
                classes,
            });
        }
    }

    let named = systems_named(description);
    if !named.is_empty() {
        let mut classes = BTreeSet::from(["refusal: a requester naming a system"]);
        let hardware_names: BTreeSet<String> =
            description.hardware().into_iter().map(|b| b.name).collect();
        let platform_names: BTreeSet<String> = description
            .platforms()
            .into_iter()
            .map(|p| p.name)
            .collect();
        for (namer, named_system) in &named {
            classes.insert(if namer.starts_with("(task ") {
                "naming: a task naming a system"
            } else if namer == system {
                "naming: the system's own level naming a system"
            } else if description.services.iter().any(|s| &s.name == namer) {
                "naming: a service naming a system"
            } else if hardware_names.contains(namer) {
                "naming: a block or a platform naming a system"
            } else {
                "naming: a named system's own level naming a system"
            });
            // Round 15 (R15-R5): which clause of which kind of hardware names the system.
            if hardware_names.contains(namer) {
                let by_needs = side_of(description, namer)
                    .is_some_and(|side| side.needs.contains(named_system));
                classes.insert(match (platform_names.contains(namer), by_needs) {
                    (true, false) => "naming: a platform's `uses` naming a system",
                    (true, true) => "naming: a platform's `needs` naming a system",
                    (false, false) => "naming: a block's `uses` naming a system",
                    (false, true) => "naming: a block's `needs` naming a system",
                });
            }
            if named_system == system {
                classes.insert("naming: the lowered system itself named");
            }
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
        let served = served_hardware(description);
        if named
            .iter()
            .any(|(namer, _)| hardware_names.contains(namer) && !served.contains(namer))
        {
            classes.insert(
                "naming: hardware served through a reached system's platform naming a system",
            );
        }
        name_order_classes(&named, "naming", &mut classes);
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
                ("a block", _, "service") => "operand: a block's `needs` naming a service",
                ("a block", _, "policy") => "operand: a block's `needs` naming a policy",
                (_, "uses", "service") => "operand: a platform's `uses` naming a service",
                (_, "uses", "policy") => "operand: a platform's `uses` naming a policy",
                (_, _, "service") => "operand: a platform's `needs` naming a service",
                (_, _, "policy") => "operand: a platform's `needs` naming a policy",
                // Both bullets take what the system serves, the naming-system one first: hardware the system serves
                // naming a system is that bullet's, and hardware it does not serve is neither's (R15-R2).
                _ => "unreachable: a system the operand of hardware the system serves, the naming-system refusal first",
            });
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
        if !unserved_naming_other_kinds(description).is_empty() {
            classes.insert(
                "operand: hardware the system does not serve naming what is not hardware too, not named",
            );
        }
        let served_otherwise = served_hardware_by(description, false);
        if operands
            .iter()
            .any(|(namer, _)| !served_otherwise.contains(namer))
        {
            classes.insert("operand: hardware served only through what served hardware names");
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
        name_order_classes(&operands, "operand", &mut classes);
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
                    takes_channel(description, b)
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
    let platforms: BTreeSet<String> = description
        .platforms()
        .into_iter()
        .map(|p| p.name)
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
    // A set of texts iterates by each, compared byte by byte: "the sources' by their interrupt's text".
    let sources: BTreeSet<String> = binds
        .iter()
        .filter(|b| relies_on_source(description, b) && !is_instance(&b.provider))
        .filter(|b| {
            hardware
                .get(&b.provider)
                .and_then(|block| block.separation)
                .is_some_and(|s| s.read() == Sep::Past)
        })
        .map(|b| format!("(source {})", b.provider))
        .collect();
    let sources_past = sources.len();
    let source_texts: Vec<String> = sources.iter().cloned().collect();
    let sources_unlike_names = worded_otherwise(&source_texts, |text| {
        text["(source ".len()..text.len() - 1].to_string()
    });
    past.extend(sources);
    // "then the bounds by their instance's text and then the fact's": a set of pairs iterates by the first, then the
    // second. An exclusive instance's requester is any requester it can serve (§2.4: "the requirement's value at that
    // exclusive instance"; model §8 rule 1): a task, a service or a policy, a block or a platform, or the system's own
    // level, "for what it writes at its own level outside its tasks" (round 16, R16-R1: the reading had looked among
    // services, tasks and hardware alone).
    let mut bounds: BTreeSet<(String, String)> = BTreeSet::new();
    let mut bounded_requesters: BTreeSet<&'static str> = BTreeSet::new();
    for instance in binds.iter().map(|b| &b.provider).filter(|p| is_instance(p)) {
        let instance_form = form(instance);
        let Some(requester) = items(&instance_form).get(3).map(text_of) else {
            continue;
        };
        let side = side_of(description, &requester);
        for fact in kind_of(instance).obliged {
            if side.as_ref().is_some_and(|side| {
                side.at_most
                    .iter()
                    .any(|(f, bound)| f == fact && bound.seconds().is_none())
            }) {
                bounds.insert((instance.clone(), (*fact).to_string()));
                bounded_requesters.insert(if requester == system {
                    "past: an obliged bound of the system's own level"
                } else if platforms.contains(&requester) {
                    "past: an obliged bound of a platform"
                } else if hardware.contains_key(&requester) {
                    "past: an obliged bound of a block"
                } else if description
                    .services
                    .iter()
                    .any(|s| s.name == requester && s.policy)
                {
                    "past: an obliged bound of a policy"
                } else {
                    "past: an obliged bound of a service"
                });
            }
        }
    }
    let bounds_past = bounds.len();
    let instance_texts: Vec<String> = bounds.iter().map(|(i, _)| i.clone()).collect();
    // An exclusive instance's text ends with its requester's: the bounds ordered by their requesters' names otherwise.
    let bounds_unlike_names = worded_otherwise(&instance_texts, |text| {
        text.trim_end_matches(')')
            .rsplit(' ')
            .next()
            .expect("a requester")
            .to_string()
    });
    past.extend(
        bounds
            .into_iter()
            .map(|(instance, fact)| format!("{instance} {fact}")),
    );
    if !past.is_empty() {
        let mut classes = BTreeSet::from(["refusal: a duration past the arithmetic"]);
        if sources_unlike_names {
            classes.insert("past: the sources' texts ordered unlike their providers' names");
        }
        if bounds_unlike_names {
            classes.insert("past: the bounds' instances ordered unlike their requesters' names");
        }
        if tasks_past > 0 {
            classes.insert("past: a task's clause");
        }
        if sources_past > 0 {
            classes.insert("past: a source's separation");
        }
        if bounds_past > 0 {
            classes.insert("past: an obliged bound");
        }
        classes.extend(bounded_requesters);
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
                names: Names::Task {
                    task: task_text(task),
                    clause: None,
                },
                classes: BTreeSet::from(["refusal: a deadline measured from another event"]),
            });
        }
    }

    // "It names every such receiver — each task in runtime order, then the system's own level — with what it would
    // receive: first the alarms it programs, each naming the provider, then its expiries, each naming the instance that
    // delivers it, each kind by those texts."
    let mut receivers = Vec::new();
    let mut classes = BTreeSet::new();
    let requesters = by_index
        .iter()
        .map(|t| task_text(t))
        .chain(std::iter::once(system.to_string()));
    for requester in requesters {
        let mut receiver = Receiver {
            subject: requester.clone(),
            alarms: BTreeSet::new(),
            expiries: BTreeSet::new(),
        };
        let mut consumed_at: BTreeMap<String, usize> = BTreeMap::new();
        for bind in binds.iter().filter(|b| b.requester == requester) {
            if is_instance(&bind.provider) {
                if kind_of(&bind.provider).runs_requester {
                    receiver.expiries.insert(bind.provider.clone());
                    classes.insert("receivers: an expiry");
                }
            } else if takes_channel(description, bind) {
                // "a timer route that names it, bound to its provider itself by a binding that takes a channel (model
                // §8 rule 2)": the route at that provider names it (rule 8), and the binding is the alarm it programs.
                receiver.alarms.insert(bind.provider.clone());
                *consumed_at.entry(bind.provider.clone()).or_default() += 1;
                classes.insert("receivers: an alarm programmed itself");
            }
        }
        // What else it is bound to at a provider it receives from, for a fact that takes no channel there.
        for bind in binds.iter().filter(|b| b.requester == requester) {
            if is_instance(&bind.provider) || takes_channel(description, bind) {
                continue;
            }
            if receiver.alarms.contains(&bind.provider) {
                classes.insert("receivers: an alarm, its receiver bound to its provider for a fact taking no channel too");
            }
            if receiver
                .expiries
                .iter()
                .any(|instance| text_of(&items(&form(instance))[2]) == bind.provider)
            {
                classes.insert(
                    "receivers: an expiry, its receiver bound to the instance's provider for a fact taking no channel too",
                );
            }
        }
        if consumed_at.values().any(|facts| *facts >= 2) {
            classes.insert("receivers: one alarm for two consumed facts at one provider");
        }
        if receiver.alarms.len() + receiver.expiries.len() >= 2 {
            classes.insert("receivers: one receiver receiving two");
        }
        if receiver.received().next().is_some() {
            receivers.push(receiver);
        }
    }
    if receivers.is_empty() {
        return None;
    }
    classes.insert(match receivers.len() {
        1 => "receivers: one",
        2 => "receivers: two",
        _ => "receivers: three",
    });
    let tasks: Vec<&String> = receivers
        .iter()
        .map(|r| &r.subject)
        .filter(|s| *s != system)
        .collect();
    if tasks.len() < receivers.len() {
        classes.insert(if tasks.is_empty() {
            "receivers: the system's own level alone"
        } else {
            "receivers: tasks and the system's own level"
        });
    }
    if tasks.windows(2).any(|w| w[0] > w[1]) {
        classes.insert("receivers: runtime order unlike the tasks' text order");
    }
    // The providers programmed, an alarm's or an instance's `OVER`, one's name a prefix of another's.
    let programmed: BTreeSet<String> = receivers
        .iter()
        .flat_map(|r| r.received())
        .map(|item| {
            if is_instance(item) {
                text_of(&items(&form(item))[2])
            } else {
                item.clone()
            }
        })
        .collect();
    if prefixed(&programmed) {
        classes.insert("receivers: providers one of whose names prefixes another's");
    }
    Some(Due {
        code: "unsupported-profile",
        names: Names::Receivers(receivers),
        classes,
    })
}

/// Round 15's classes (R15-D2), which §1's two refusals keyed by their bindings meet whatever the system's outcome: a
/// task or the system's own level that a timer route names — bound for a consumed fact at `true` to the route's
/// provider, or to an instance whose own binding beneath is there (model §8 rules 2, 8 and 15) — and is bound to that
/// provider itself only for a fact that takes no channel, so receives no alarm there ("bound to its provider itself by a
/// binding that takes a channel"); one bound to a timer route's provider for such a fact, which the route does not name;
/// either beside one programming an alarm at that provider; and a periodic task bound to a source's block for another
/// fact than `interrupt-source`, so no second release model ("bound itself for `interrupt-source`").
fn keyed_classes(description: &Description, binds: &[Bind]) -> BTreeSet<&'static str> {
    let system = description.system.as_str();
    let task_text = |task: &Task| format!("(task {system} {})", task.name);
    let requesters: Vec<(String, bool)> = description
        .tasks
        .iter()
        .map(|t| (task_text(t), true))
        .chain(std::iter::once((system.to_string(), false)))
        .collect();
    // Each timer route's provider, with the requesters it names.
    let mut routes: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    for bind in binds
        .iter()
        .filter(|b| !is_instance(&b.provider) && takes_channel(description, b))
    {
        let named = routes.entry(bind.provider.clone()).or_default();
        if is_instance(&bind.requester) {
            named.extend(
                binds
                    .iter()
                    .filter(|b| b.provider == bind.requester)
                    .map(|b| b.requester.clone()),
            );
        } else {
            named.insert(bind.requester.clone());
        }
    }
    let programs_at = |requester: &str, provider: &str| {
        binds.iter().any(|b| {
            b.requester == requester && b.provider == provider && takes_channel(description, b)
        })
    };
    let mut classes = BTreeSet::new();
    for (requester, is_task) in &requesters {
        for bind in binds.iter().filter(|b| {
            &b.requester == requester && !is_instance(&b.provider) && !takes_channel(description, b)
        }) {
            let Some(named) = routes.get(&bind.provider) else {
                continue;
            };
            if programs_at(requester, &bind.provider) {
                continue; // a receiver of that alarm, the receivers' class
            }
            classes.insert(match (is_task, named.contains(requester)) {
                (true, true) => {
                    "bound otherwise: a task a timer route names, bound to its provider for a fact taking no channel"
                }
                (true, false) => {
                    "bound otherwise: a task bound to a timer route's provider for a fact taking no channel, the \
                     route not naming it"
                }
                (false, true) => {
                    "bound otherwise: the system's own level a timer route names, bound to its provider for a fact \
                     taking no channel"
                }
                (false, false) => {
                    "bound otherwise: the system's own level bound to a timer route's provider for a fact taking no \
                     channel, the route not naming it"
                }
            });
            if requesters
                .iter()
                .any(|(other, _)| other != requester && programs_at(other, &bind.provider))
            {
                classes.insert(
                    "bound otherwise: beside a task or the system's own level programming an alarm at that provider",
                );
            }
        }
    }
    let sources: BTreeSet<String> = description
        .hardware()
        .into_iter()
        .filter(|b| b.separation.is_some())
        .map(|b| b.name)
        .collect();
    for task in description
        .tasks
        .iter()
        .filter(|t| matches!(t.release, Release::Period(_)))
    {
        if binds.iter().any(|b| {
            b.requester == task_text(task)
                && sources.contains(&b.provider)
                && !relies_on_source(description, b)
        }) {
            classes.insert("periodic: a periodic task bound to a source's block for another fact");
        }
    }
    classes
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

/// Whether a refusal's message names `clause`, outside an instance's text: a form it heads, such as `(period 20 ms)`, or
/// the word itself.
fn clause_named(message: &str, clause: &str) -> bool {
    fn walk(sx: &Sx, clause: &str) -> bool {
        match sx {
            Sx::Atom(atom) => atom == clause,
            Sx::List(items) => {
                items.first() != Some(&Sx::Atom("adapter".to_string()))
                    && items.iter().any(|item| walk(item, clause))
            }
        }
    }
    prose(message).iter().any(|sx| walk(sx, clause))
}

/// What a refusal's message names each receiver receiving, in the order it names them, as `(SUBJECT, ITEM)`: read
/// clause by clause, a clause ending at a `;`, each naming one subject — a `(task …)` form or the system's own level's
/// name, as [`subjects_named`] reads them — with each instance's `(adapter …)` text it holds and each of `hardware` it
/// names outside such a text, an alarm's provider. A clause naming something received beside no subject or several
/// says not who receives it, and is returned as such. §1 fixes who receives what and in which order, not the words.
fn received_named(
    message: &str,
    system: &str,
    hardware: &BTreeSet<String>,
) -> Result<Vec<(String, String)>, String> {
    fn walk(sx: &Sx, hardware: &BTreeSet<String>, out: &mut Vec<String>) {
        match sx {
            Sx::Atom(atom) => {
                if hardware.contains(atom) && !out.contains(atom) {
                    out.push(atom.clone());
                }
            }
            Sx::List(items) => {
                let head = match items.first() {
                    Some(Sx::Atom(head)) => head.as_str(),
                    _ => "",
                };
                if head == "adapter" {
                    let text = prose_text(sx);
                    if !out.contains(&text) {
                        out.push(text);
                    }
                } else if head != "task" {
                    // A `(task …)` form names a subject, never what is received.
                    for item in items {
                        walk(item, hardware, out);
                    }
                }
            }
        }
    }
    let mut out = Vec::new();
    for clause in message.split(';') {
        let mut received = Vec::new();
        for sx in prose(clause) {
            walk(&sx, hardware, &mut received);
        }
        if received.is_empty() {
            continue;
        }
        match subjects_named(clause, system).as_slice() {
            [subject] => out.extend(received.into_iter().map(|item| (subject.clone(), item))),
            subjects => {
                return Err(format!(
                    "a clause names {received:?} received beside {subjects:?}, saying not who receives it: {clause:?}"
                ))
            }
        }
    }
    Ok(out)
}

/// Where the lowering's `refusal` is not the one `due`: its code, and what it names. Of a refusal naming a task, the
/// task, and the clause where §1 names one. Of the interrupt-code refusal, the subjects it names in their order, and
/// what it says each would receive, in §1's order — its alarms, then its expiries, each kind by its texts: §1 fixes the
/// receivers, what each receives and their order, not the message's wording; so an expiry is held to its instance's
/// text, written whole, and an alarm a receiver programs itself to its provider's name, written outside any instance's
/// text — an interrupt being "named by its kind and its provider" (§2). Of the durations past the arithmetic, each, in
/// §1's order. Of the naming-system refusal, each requester and the system it names, by the namer's text and then the
/// system's; of the refusal of hardware naming what is not hardware, each block or platform and its operand, by the
/// namer's text and then the operand's — each text compared byte by byte, "never by its message's words".
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
        Names::Task { task, clause } => {
            let named = subjects_named(&refusal.message, system);
            if named != [task.clone()] {
                wrong.push(format!("it names {named:?}, the record [{task:?}]"));
            }
            if let Some(clause) = clause {
                if !clause_named(&refusal.message, clause) {
                    wrong.push(format!("it does not name the clause `{clause}`"));
                }
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
        Names::Receivers(receivers) => {
            let subjects: Vec<String> = receivers.iter().map(|r| r.subject.clone()).collect();
            let named = subjects_named(&refusal.message, system);
            if named != subjects {
                wrong.push(format!(
                    "it names {named:?}, the record {subjects:?}, in that order"
                ));
            }
            let forms = forms_in(&refusal.message);
            let words = words_outside_instances(&refusal.message);
            let unsaid: Vec<&String> = receivers
                .iter()
                .flat_map(|r| r.expiries.iter())
                .filter(|instance| !forms.contains(*instance))
                .chain(
                    receivers
                        .iter()
                        .flat_map(|r| r.alarms.iter())
                        .filter(|provider| !words.contains(*provider)),
                )
                .collect();
            if !unsaid.is_empty() {
                wrong.push(format!(
                    "it does not name what would be received: {unsaid:?}"
                ));
            }
            // Each receiver with what it would receive, in §1's order (round 14, R14-R6).
            let hardware: BTreeSet<String> =
                description.hardware().into_iter().map(|b| b.name).collect();
            let expected: Vec<(String, String)> = receivers
                .iter()
                .flat_map(|r| r.received().map(|item| (r.subject.clone(), item.clone())))
                .collect();
            match received_named(&refusal.message, system, &hardware) {
                Ok(received) if received == expected => {}
                Ok(received) => wrong.push(format!(
                    "it names each receiver receiving {received:?}, the record {expected:?}, in that order"
                )),
                Err(unpaired) => wrong.push(unpaired),
            }
        }
    }
    (!wrong.is_empty()).then(|| wrong.join("; "))
}

/// The class of the systems whose record the configuration checker refuses because the search reads one system's side
/// apart, held by name and count rather than as a finding: the defect is the search's, leaf `M3.4`'s, neither the
/// runtime plan record's nor the lowering's. Model §8 rule 1: "Two tasks of one system are two requesters, though §7
/// reads them as one side"; §7 rule 4: "Every `requires`, `needs` and `uses` a declaration writes ... is read as one
/// side, so a contradiction split across two clauses is one: constraints on one fact that no value satisfies together
/// are `invalid-description`". The search refuses such a contradiction inside one requester's clauses, but reads the
/// system's own level and each task as sides apart, so it writes a record for a system whose own level writes `(f
/// false)` while a task of it needs `f`, and the checker (`archogen-plan-check`) refuses it. When `M3.4` reads the
/// system's side whole, the search refuses these systems itself, this class empties, and the universes holding it move
/// from judged to unresolved.
///
/// ⚠️ **Until then, these systems' outcomes are the lowering's without its third input** (round 16, R16-R2). The
/// lowering takes "the configuration checker's result record, `archogen-result/1`, for that assignment and that
/// program" (§1, its third input), whose intake is unbuilt (R16-R6: `M4.1.4`'s production half), so the harness lowers
/// the record alone and the lowering is held to the reading on these systems, plan or refusal. Once the lowering reads
/// the result record, the checker's refusal reaches it, and §1 refuses each of these systems before anything is
/// lowered — "its status is not `ok`: `tool-failure`, naming both verdicts, the generator having accepted what the
/// checker refused" — ahead of every refusal of the record's tasks and of every plan: the class's systems then move,
/// in each universe holding it, from lowered or refused as the reading says to refused `tool-failure`, and this
/// checker reads the result record with them.
const CHECKER_REFUSES: &str =
    "checker refuses: a system's side the search reads apart (M3.4); `tool-failure` once \
                               the lowering reads the checker's result (§1)";

/// The fact on which the lowered system's side, read whole — its own level's clauses and its tasks' (model §8 rule 1;
/// §7 rule 4) — requires both `true` and `false`, a requirement `(needs f)` or `(f true)` beside `(f false)`.
fn side_read_apart(description: &Description) -> Option<String> {
    let mut values: BTreeMap<String, BTreeSet<bool>> = BTreeMap::new();
    for side in std::iter::once(&description.own).chain(description.tasks.iter().map(|t| &t.side)) {
        for fact in &side.needs {
            values.entry(fact.clone()).or_default().insert(true);
        }
        for clause in &side.other {
            let clause = form(clause);
            let clause = items(&clause);
            if let (Some(Sx::Atom(fact)), Some(Sx::Atom(value))) = (clause.first(), clause.get(1)) {
                if value == "true" || value == "false" {
                    values
                        .entry(fact.clone())
                        .or_default()
                        .insert(value == "true");
                }
            }
        }
    }
    values
        .into_iter()
        .find(|(_, values)| values.len() == 2)
        .map(|(fact, _)| fact)
}

/// How one system fared.
struct Outcome {
    /// `lowered`, `rejected` by the language, `unresolved` by the search, or `refused` by the lowering.
    class: &'static str,
    /// The verdict of a system not lowered.
    verdict: String,
    /// Whether the configuration checker judged its record: every system the search resolves.
    judged: bool,
    /// Whether the configuration checker refused its record.
    checker_refused: bool,
    /// The kinds of line a plan agrees with the reading on.
    agreed: Vec<&'static str>,
    /// The classes of the record's rules the system exercises.
    classes: BTreeSet<&'static str>,
    /// Where the code and the record's reading disagree, or the checker refuses the record: each a reproducer.
    findings: Vec<String>,
    /// The plan the code writes, where it writes one and the universe keeps them.
    plan: Option<String>,
}

fn outcome(universe: &str, case: &Case, keep: bool) -> Outcome {
    let text = case.description.text();
    let mut out = Outcome {
        class: "lowered",
        verdict: String::new(),
        judged: false,
        checker_refused: false,
        agreed: Vec::new(),
        classes: BTreeSet::new(),
        findings: Vec::new(),
        plan: None,
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
    // The checker's verdict, against what the reading derives of it: every record accepted, but where the system's side
    // read whole contradicts itself, which the checker refuses `invalid-description` naming the fact ([`CHECKER_REFUSES`]).
    out.checker_refused = checker.is_some();
    let apart = side_read_apart(&case.description);
    let held = match (&checker, &apart) {
        (Some(refused), Some(fact))
            if refused.starts_with("invalid-description;")
                && refused.contains(&format!("`{fact}`")) =>
        {
            true
        }
        (Some(refused), _) => {
            out.findings.push(reproducer(
                "",
                &format!(
                    "-- the configuration checker refuses the record the search wrote: {refused}\n"
                ),
            ));
            false
        }
        (None, Some(fact)) => {
            out.findings.push(reproducer(
                "",
                &format!(
                    "-- the configuration checker accepts the record, and `{}`'s side, read whole, requires `{fact}` \
                     both `true` and `false` (model §7 rule 4; §8 rule 1)\n",
                    case.description.system
                ),
            ));
            false
        }
        (None, None) => false,
    };
    let due = refusal_the_record_gives(&case.description, &binds_of(&record));
    match lowered {
        Ok(plan) => {
            if keep {
                out.plan = Some(plan.clone());
            }
            if let Some(due) = &due {
                out.findings.push(reproducer(
                    &format!("-- the code writes\n{plan}"),
                    &format!(
                        "-- the record refuses it: `{}`, naming {} (§1)\n",
                        due.code,
                        due.summary()
                    ),
                ));
                if held {
                    out.classes.insert(CHECKER_REFUSES);
                }
                return out;
            }
            let judged = judge(&case.description, &record, &plan);
            out.agreed = judged.agreed;
            out.classes = judged.classes;
            out.classes
                .extend(keyed_classes(&case.description, &binds_of(&record)));
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
                (None, Some(due)) => {
                    out.classes = due.classes;
                    out.classes
                        .extend(keyed_classes(&case.description, &binds_of(&record)));
                }
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
    if held {
        out.classes.insert(CHECKER_REFUSES);
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
    /// The systems whose record the configuration checker refused.
    checker_refused: usize,
    /// For each kind of line, the lowered systems whose plan agrees with the reading on it.
    agreed: BTreeMap<&'static str, usize>,
    /// For each class of the record's rules, the systems exercising it.
    classes: BTreeMap<&'static str, usize>,
    /// Each verdict of a system not lowered, with how many gave it and the first that did.
    verdicts: BTreeMap<String, (usize, String)>,
    /// Each system the search resolved no plan for: its label, with the search's verdict.
    unresolved_systems: BTreeMap<String, String>,
    findings: Vec<String>,
    elapsed: std::time::Duration,
}

/// Every case lowered and judged, on every core: the cases are independent.
fn run(universe: &str, cases: &[Case]) -> Tally {
    run_keeping(universe, cases, false).0
}

/// [`run`], with the plan the code writes for each case, where it writes one and `keep`: kept only where a test holds
/// plans against each other, so that the large universes hold none in memory.
fn run_keeping(universe: &str, cases: &[Case], keep: bool) -> (Tally, Vec<Option<String>>) {
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
                        .map(|(i, case)| (i, outcome(universe, case, keep)))
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
    let mut plans = Vec::with_capacity(cases.len());
    for (i, outcome) in outcomes {
        plans.push(outcome.plan);
        match outcome.class {
            "lowered" => tally.lowered += 1,
            "rejected" => tally.rejected += 1,
            "unresolved" => tally.unresolved += 1,
            _ => tally.refused += 1,
        }
        if outcome.judged {
            tally.judged += 1;
        }
        if outcome.checker_refused {
            tally.checker_refused += 1;
        }
        if outcome.class == "unresolved" {
            tally
                .unresolved_systems
                .insert(cases[i].label.clone(), outcome.verdict.clone());
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
    (tally, plans)
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
    // Every record the checker refuses is one of [`CHECKER_REFUSES`], held only where a universe names that class.
    let held = tally.classes.get(CHECKER_REFUSES).copied().unwrap_or(0);
    if tally.checker_refused != held {
        problems.push_str(&format!(
            "{universe}: the configuration checker refuses {} records, {held} of them held as `{CHECKER_REFUSES}`\n",
            tally.checker_refused
        ));
    }
    if held > 0 && !exercised.contains(&CHECKER_REFUSES) {
        problems.push_str(&format!(
            "{universe}: {held} records the configuration checker refuses, `{CHECKER_REFUSES}`, which this universe \
             does not hold\n"
        ));
    }
    let missed: Vec<&&str> = exercised
        .iter()
        .filter(|class| !tally.classes.contains_key(**class))
        .collect();
    // A branch the reading states unreachable, its reason in its name, reached by no system of any universe.
    let unreachable: Vec<(&&str, &usize)> = tally
        .classes
        .iter()
        .filter(|(class, _)| class.starts_with("unreachable: "))
        .collect();
    if !unreachable.is_empty() {
        problems.push_str(&format!(
            "{universe} reaches a branch the reading states unreachable: {unreachable:?}\n"
        ));
    }
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
            "owed: an instance in a job, its call cost and masked runs",
            "owed: a service or a policy in a job, its call cost and masked runs",
            "owed: a block or a platform in a job, its call cost and masked runs",
            "owed: a node in no job, neither its call cost nor masked runs",
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
            "trigger: its own source",
            "trigger: its own source, no other reaching it",
            "trigger: its own source, beside another reaching it",
            "trigger: its own source leaving it unknown, beside one that alone would release it every",
            "trigger: its own source releasing it every, beside one that alone would leave it unknown",
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
            "initialisation: a composite bound to its part, its edge kept",
            "initialisation: an edge across composition ordering two steps unlike their texts",
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
            "past: the sources' texts ordered unlike their providers' names",
            "past: the bounds' instances ordered unlike their requesters' names",
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
    // What `timer.d` names, of 3; `dma.x`, of 10; `soc`, of 5; `blk.q`, of 2; the other refusal `t` meets, of 6.
    assert_eq!(cases.len(), 3 * 10 * 5 * 2 * 6);
    let tally = run("hardware naming", &cases);
    // Lowered: the 20 whose hardware `sys` serves names hardware alone — `timer.d` nothing, `dma.x` nothing or a block
    // (5), `soc` nothing or a block (2) — whatever `blk.q`, which `sys` does not serve, names (2; round 15, R15-R2), and
    // whose task `t` meets no other refusal; every other refused.
    hold(
        "hardware naming",
        &tally,
        (1_800, 20, 0, 0, 1_780),
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
            "operand: hardware the system does not serve naming what is not hardware too, not named",
            "unserved: a block naming a service",
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
            "hardware: what a node in a job names by `needs` is in none",
            "initialisation: a composite bound to its part, its edge kept",
            "initialisation: a block composing a block, an edge across it kept",
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
            "initialisation: a named edge from a block a service names by `uses`",
            "initialisation: a named edge from a block a service names by `needs`",
            "initialisation: a named edge from a block no hardware, served through its namer",
            "initialisation: a named edge from hardware bound for nothing and bound to by nothing",
            "named: a block a service names, bound for nothing, a node",
            "named: a block a task names, bound for nothing, a node",
            "named: a block no hardware, which a service or a policy names, bound for nothing, a node",
            "named: hardware named and bound for nothing, in a job",
            "named: hardware named and bound for nothing, in no job",
            "named: hardware a service or a policy the system does not serve names, no node",
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
    // What each of 5 namers names, of 4; by `uses` or `needs`; `y.b`'s own level naming `y` or not.
    assert_eq!(cases.len(), 4usize.pow(5) * 2 * 2);
    let tally = run("naming order", &cases);
    // Lowered: the 4 where no namer names a system (by either clause, `y.b` naming `y` or not, `y.b` never reached).
    hold(
        "naming order",
        &tally,
        (4_096, 4, 0, 0, 4_092),
        &[
            "refusal: a requester naming a system",
            "naming: a task naming a system",
            "naming: the system's own level naming a system",
            "naming: a service naming a system",
            "naming: a named system's own level naming a system",
            "naming: several names",
            "naming: one namer naming two systems",
            "naming: one namer's text a prefix of another's",
            "naming: one system's text a prefix of another's",
            "naming: a name continuing with a byte below `)`",
            "naming: tasks' texts ordered unlike their names",
            "naming: an order the message's words would give otherwise",
        ],
    );
}

#[test]
fn several_blocks_and_platforms_naming_prefixed_names_in_their_texts_order() {
    let cases = prefixed_hardware_naming();
    // What each of `d`, `d.x`, `d!`, `soc` and `q` names, of 4.
    assert_eq!(cases.len(), 4usize.pow(5));
    let tally = run("prefixed hardware naming", &cases);
    // Lowered: the 4 where no block or platform `sys` serves names anything but hardware, whatever `q`, which it serves
    // only once `y` is reached, names (round 15, R15-R2); every other refused, by the naming-system refusal where a
    // block `sys` serves names `y`, else by hardware naming what is not hardware.
    hold(
        "prefixed hardware naming",
        &tally,
        (1_024, 4, 0, 0, 1_020),
        &[
            "refusal: hardware naming what is not hardware",
            "refusal: a requester naming a system",
            "naming: a block or a platform naming a system",
            "naming: hardware served through a reached system's platform naming a system",
            "naming: one namer's text a prefix of another's",
            "naming: a name continuing with a byte below `)`",
            "naming: an order the message's words would give otherwise",
            "operand: a block's `uses` naming a service",
            "operand: a block's `needs` naming a policy",
            "operand: a platform's `needs` naming a service",
            "operand: a platform's `uses` naming a policy",
            "operand: a block naming the service bound to it",
            "operand: a service only hardware names",
            "operand: hardware the system does not serve naming what is not hardware too, not named",
            "unserved: a block naming a service",
            "unserved: a block's `uses` naming a system",
            "unserved: a block's `needs` naming a system",
            "unserved: hardware naming the lowered system",
            "operand: several, named in order",
            "operand: one namer naming two",
            "operand: one namer's text a prefix of another's",
            "operand: one operand's text a prefix of another's",
            "operand: a name continuing with a byte below `)`",
            "operand: an order the message's words would give otherwise",
        ],
    );
}

#[test]
fn one_receiver_s_alarms_and_expiries() {
    let cases = receivers_within_one();
    // 3 shapes of counters; what `t`, `t!` and the system's own level each program, of 4; which task is first.
    assert_eq!(cases.len(), 3 * 4usize.pow(3) * 2);
    let tally = run("one receiver", &cases);
    // Unresolved: the 148 where some requester programs both and no one provider offers both (37 choices of the 64
    // with a `Both`, 2 shapes, 2 orders), each `missing-fact`: one receiver may receive one thing, never two. Lowered:
    // the 6 where nothing is programmed. Every other refused, naming its receivers.
    let both_apart: BTreeSet<String> = cases
        .iter()
        .filter(|c| c.label.contains("Both") && !c.label.starts_with("OneOffersBoth"))
        .map(|c| c.label.clone())
        .collect();
    assert_eq!(both_apart.len(), 148);
    assert_eq!(
        tally.unresolved_systems.keys().cloned().collect::<BTreeSet<_>>(),
        both_apart,
        "the systems the search resolves no plan for are those where one requester programs both apart"
    );
    assert!(
        tally
            .unresolved_systems
            .values()
            .all(|verdict| verdict == "MissingFact"),
        "the search's answer: {:?}",
        tally.unresolved_systems
    );
    hold(
        "one receiver",
        &tally,
        (384, 6, 0, 148, 230),
        &[
            "receivers: an alarm programmed itself",
            "receivers: an expiry",
            "receivers: one alarm for two consumed facts at one provider",
            "receivers: three",
            "receivers: tasks and the system's own level",
            "receivers: runtime order unlike the tasks' text order",
            "receivers: providers one of whose names prefixes another's",
        ],
    );
    assert!(
        !tally
            .classes
            .contains_key("receivers: one receiver receiving two"),
        "a receiver receiving two things is a system the search resolves no plan for"
    );
}

#[test]
fn one_instance_owes_one_bound_at_most() {
    let cases = one_instance_s_bounds();
    // A first bound of 2 values, a second of 3 (none among them).
    assert_eq!(cases.len(), 2 * 3);
    let tally = run("one instance's bounds", &cases);
    // One bound within, or two: lowered, the tightest obliged. One past, or two: refused, naming the bound once. One
    // within and one past: the search's answer, `unsupported-profile`.
    let mixed = BTreeSet::from([
        "dl writes 5 ms and past".to_string(),
        "dl writes past and 3 ms".to_string(),
    ]);
    assert_eq!(
        tally
            .unresolved_systems
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        mixed,
        "the search resolves no plan where one fact's bounds lie one within the arithmetic, one past it: {:?}",
        tally.unresolved_systems
    );
    assert!(
        tally
            .unresolved_systems
            .values()
            .all(|verdict| verdict == "UnsupportedProfile"),
        "the search's answer: {:?}",
        tally.unresolved_systems
    );
    hold(
        "one instance's bounds",
        &tally,
        (6, 2, 0, 2, 2),
        &[
            "instance: an obliged fact with its bound",
            "refusal: a duration past the arithmetic",
            "past: an obliged bound",
        ],
    );
}

#[test]
fn a_provider_bound_to_the_node_taking_its_channel() {
    let cases = own_route_through_its_provider();
    // What the timer requires, of 4; what `dma.x` and `soc` each take, of 3; what `t` uses, of 4, and `u`, of 3.
    assert_eq!(cases.len(), 4 * 3 * 3 * 4 * 3);
    let tally = run("own route", &cases);
    hold(
        "own route",
        &tally,
        (432, 432, 0, 0, 0),
        &[
            "multiplex: its own route a user, the route's provider bound to it",
            "multiplex: its own route a user, another node there reaching it",
            "multiplex: its own route no user",
            "multiplex: a block or a platform multiplexed",
            "multiplex: multiplexed by a job and an interrupt",
            "multiplex: a block or a platform's channel with one user",
            "hardware: a block bound for requirements of its own",
            "hardware: a platform bound for requirements of its own",
            "hardware: an alarm it programs itself",
            "hardware: an expiry running its code",
            "initialisation: a component of several members",
            "initialisation: a composite bound to its part, its edge kept",
            "initialisation: a part bound to its composite, its edge kept",
            "initialisation: a composite and its part bound each to the other, one component",
            "composition: a platform a task reaches through a binding",
        ],
    );
}

#[test]
fn bound_to_a_timer_route_s_provider_by_a_binding_taking_no_channel() {
    let cases = bound_otherwise_on_a_route();
    // What `t` and the system's own level each require, of 7; `u`, of 3; `s`, of 4; which task is first.
    assert_eq!(cases.len(), 7 * 3 * 7 * 4 * 2);
    let tally = run("bound otherwise on a route", &cases);
    // Rejected: the 168 where `t` writes `(relative-delay false)` (3 choices of `u`, 7 of the system's own level, 4 of
    // `s`, 2 orders), `invalid-description`: a task writes no constraint clause.
    let writes_false: BTreeSet<String> = cases
        .iter()
        .filter(|c| c.label.starts_with("t:False "))
        .map(|c| c.label.clone())
        .collect();
    assert_eq!(writes_false.len(), 168);
    assert_eq!(
        tally
            .verdicts
            .get("rejected invalid-description")
            .map(|v| v.0),
        Some(168),
        "the language refuses a task's constraint clause: {:?}",
        tally.verdicts
    );
    // Held, not findings: the 64 whose own level writes `(relative-delay false)` beside a task needing `relative-delay`
    // (8 of the 18 choices of `t` and `u` the language accepts, 4 of `s`, 2 orders), whose record the configuration
    // checker refuses `invalid-description` — the search's defect, `M3.4`'s ([`CHECKER_REFUSES`]; model §7 rule 4, §8
    // rule 1). Every other record it accepts; the lowering, which does not read the checker's result yet, is held to
    // the reading on these too, and once it reads it they move to `tool-failure` (§1; R16-R2).
    assert_eq!(
        tally.classes.get(CHECKER_REFUSES),
        Some(&64),
        "{CHECKER_REFUSES}"
    );
    assert_eq!(tally.checker_refused, 64);
    hold(
        "bound otherwise on a route",
        &tally,
        (1_176, 320, 168, 0, 688),
        &[
            CHECKER_REFUSES,
            "bound otherwise: a task a timer route names, bound to its provider for a fact taking no channel",
            "bound otherwise: the system's own level a timer route names, bound to its provider for a fact taking no \
             channel",
            "bound otherwise: a task bound to a timer route's provider for a fact taking no channel, the route not \
             naming it",
            "bound otherwise: the system's own level bound to a timer route's provider for a fact taking no channel, \
             the route not naming it",
            "bound otherwise: beside a task or the system's own level programming an alarm at that provider",
            "bound otherwise: a requester bound for a consumed fact at `false`, no channel",
            "receivers: an alarm, its receiver bound to its provider for a fact taking no channel too",
            "receivers: an expiry, its receiver bound to the instance's provider for a fact taking no channel too",
            "receivers: one",
            "receivers: two",
            "receivers: three",
            "receivers: tasks and the system's own level",
            "receivers: runtime order unlike the tasks' text order",
            "context: a wake running the instance alone",
            "multiplex: multiplexed by two jobs",
        ],
    );
}

#[test]
fn a_periodic_task_bound_to_a_source_s_block_for_another_fact() {
    let cases = periodic_bound_otherwise();
    // What `t` requires, of 5; `p`, of 4; the system's own level, of 4; `e`, of 3; which periodic task is first.
    assert_eq!(cases.len(), 5 * 4 * 4 * 3 * 2);
    let tally = run("periodic bound otherwise", &cases);
    // Rejected: the 96 where `t` writes `(interrupt-source false)` (4 choices of `p`, 4 of the system's own level, 3 of
    // `e`, 2 orders), `invalid-description`: a task writes no constraint clause.
    assert_eq!(
        tally
            .verdicts
            .get("rejected invalid-description")
            .map(|v| v.0),
        Some(96),
        "the language refuses a task's constraint clause: {:?}",
        tally.verdicts
    );
    // Held, not findings: the 72 whose own level writes `(interrupt-source false)` beside `t` or `p` needing
    // `interrupt-source` (12 of the 16 choices of `t` and `p` the language accepts, 3 of `e`, 2 orders), whose record
    // the configuration checker refuses `invalid-description` — the search's defect, `M3.4`'s ([`CHECKER_REFUSES`];
    // model §7 rule 4, §8 rule 1). Every other record it accepts; the lowering, which does not read the checker's
    // result yet, is held to the reading on these too, and once it reads it they move to `tool-failure` (§1; R16-R2).
    assert_eq!(
        tally.classes.get(CHECKER_REFUSES),
        Some(&72),
        "{CHECKER_REFUSES}"
    );
    assert_eq!(tally.checker_refused, 72);
    hold(
        "periodic bound otherwise",
        &tally,
        (480, 96, 96, 0, 288),
        &[
            CHECKER_REFUSES,
            "refusal: a periodic task bound to a source itself",
            "periodic: a periodic task bound to a source's block for another fact",
            "periodic: one bound to the source's block otherwise first in runtime order, the one bound for \
             `interrupt-source` named",
            "periodic: the one named bound to the source's block for another fact too",
            "bound otherwise: a requester bound for `interrupt-source` at `false`, no route",
            "trigger: every, equal separations",
            "trigger: unknown, no source reaching it",
        ],
    );
}

#[test]
fn hardware_the_lowered_system_does_not_serve_naming_what_is_not_hardware() {
    let cases = unserved_hardware_naming();
    // What each of `blk.q`, `host.q` and `lone` names, of 5; `soc`, of 5; each description lowered for each system.
    assert_eq!(cases.len(), 5usize.pow(3) * 5 * 2);
    for pair in cases.chunks(2) {
        assert_eq!(
            pair[0].description.text(),
            pair[1].description.text(),
            "one description, lowered for each of its systems"
        );
    }
    let tally = run("unserved hardware naming", &cases);
    // Unresolved: the 232 lowering `other` that serve `alarm`: the 225 where `blk.q` or `host.q`, which `other` serves,
    // names it (9 of the 25 choices of the two, 5 of `lone`, 5 of `soc`); and the 7 where neither does, one names `sys`
    // by `uses`, so that `sys`'s `platform` clause is served and with it `soc`, which names `via.s`, which names `lone`,
    // which names `alarm` (7 of the 16 choices of the two). The joint rules serve what hardware names (§1: "serving what
    // it names") and what a system named writes at its own level, "its `platform` clause included", and `alarm`'s
    // `relative-delay` has no provider among `other`'s hardware, so the search concludes `missing-fact` (model §8 rule
    // 7).
    let serves_alarm: BTreeSet<String> = cases
        .iter()
        .filter(|c| {
            let names =
                |namer: &str, naming: &str| c.label.contains(&format!("{namer} names [{naming}]"));
            c.label.ends_with("lowering other")
                && (names("blk.q", "uses:alarm")
                    || names("host.q", "uses:alarm")
                    || ((names("blk.q", "uses:sys") || names("host.q", "uses:sys"))
                        && names("soc", "uses:via.s")
                        && names("lone", "uses:alarm")))
        })
        .map(|c| c.label.clone())
        .collect();
    assert_eq!(serves_alarm.len(), 232);
    assert_eq!(
        tally
            .unresolved_systems
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        serves_alarm,
        "the systems the search resolves no plan for are those `other` lowered serving `alarm`"
    );
    assert!(
        tally
            .unresolved_systems
            .values()
            .all(|verdict| verdict == "MissingFact"),
        "the search's answer: {:?}",
        tally.unresolved_systems
    );
    // Lowered: lowering `sys`, the 125 where `soc` names nothing, whatever `blk.q`, `host.q` and `lone`, which `sys`
    // does not serve, name; lowering `other`, the 25 where `blk.q` and `host.q` name nothing, whatever `soc` and `lone`
    // name. Every other refused or unresolved.
    hold(
        "unserved hardware naming",
        &tally,
        (1_250, 150, 0, 232, 868),
        &[
            "unserved: a block naming a service",
            "unserved: a block naming a policy",
            "unserved: a block's `uses` naming a system",
            "unserved: a block's `needs` naming a system",
            "unserved: a platform naming a service",
            "unserved: a platform naming a policy",
            "unserved: a platform's `uses` naming a system",
            "unserved: a platform's `needs` naming a system",
            "unserved: hardware naming the lowered system",
            "unserved: hardware naming a service the lowered system serves",
            "unserved: hardware no system's platform composes",
            "refusal: hardware naming what is not hardware",
            "operand: a platform's `uses` naming a policy",
            "operand: a platform's `uses` naming a service",
            "operand: hardware the system does not serve naming what is not hardware too, not named",
            "operand: hardware served only through what served hardware names",
            "refusal: a requester naming a system",
            "naming: a platform's `uses` naming a system",
            "naming: a platform's `needs` naming a system",
            "naming: a block's `uses` naming a system",
            "naming: a block's `needs` naming a system",
            "naming: the lowered system itself named",
        ],
    );
}

#[test]
fn sporadic_tasks_released_by_their_own_sources() {
    let cases = own_sources();
    // Which source is declared first; `t` bound itself or not; what `t` uses, of 4, and `u`, of 3; each of 2 sources'
    // separations, of 6.
    assert_eq!(cases.len(), 2 * 2 * 4 * 3 * 6 * 6);
    let tally = run("own sources", &cases);
    hold(
        "own sources",
        &tally,
        (1_728, 1_728, 0, 0, 0),
        &[
            "trigger: its own source",
            "trigger: its own source, no other reaching it",
            "trigger: its own source, beside another reaching it",
            "trigger: its own source leaving it unknown, beside one that alone would release it every",
            "trigger: its own source releasing it every, beside one that alone would leave it unknown",
            "trigger: its own source, reached through a requester its route names too",
            "trigger: binding none, beside a task bound to its own source",
            "trigger: binding none, reaching the handler of another task's own source",
            "trigger: every, equal separations",
            "trigger: every, a longer separation",
            "trigger: unknown, a shorter separation",
            "trigger: unknown, no separation stated",
            "trigger: unknown, the separation absent",
            "trigger: unknown, no source reaching it",
            "trigger: unknown, several sources",
            "rank: two stated separations tied",
        ],
    );
}

#[test]
fn a_binding_keeps_its_edge_across_composition() {
    let cases = bound_across_composition();
    // 2 namings; `P` composing `C` itself or through `B`; what each of 3 declarations requires of the other two, of 4.
    assert_eq!(cases.len(), 2 * 2 * 4usize.pow(3));
    let tally = run("across composition", &cases);
    hold(
        "across composition",
        &tally,
        (256, 256, 0, 0, 0),
        &[
            "initialisation: a composite bound to its part, its edge kept",
            "initialisation: a part bound to its composite, its edge kept",
            "initialisation: a composite and its part bound each to the other, one component",
            "initialisation: an edge across composition ordering two steps unlike their texts",
            "initialisation: an edge across composition at depth, through another part, kept",
            "initialisation: a block composing a block, an edge across it kept",
            "initialisation: a component of several members",
            "hardware: a block bound for requirements of its own",
            "hardware: a platform bound for requirements of its own",
            "hardware: requiring, and bound to by a requester",
            "hardware: requiring, and bound to by nothing",
        ],
    );
}

#[test]
fn every_requester_s_obliged_bound_named_past_the_arithmetic() {
    let cases = obliged_bounds_of_every_requester();
    // What each of 5 requesters writes, of 4.
    assert_eq!(cases.len(), 4usize.pow(5));
    let tally = run("obliged bounds", &cases);
    // Lowered: the 81 where the system's own level requires no deadline and no bound lies past the arithmetic (3 choices
    // of each of the 4 others). Every other refused: naming each bound past the arithmetic, where one is; else the
    // system's own level, receiving its instance's expiry (§1).
    hold(
        "obliged bounds",
        &tally,
        (1_024, 81, 0, 0, 943),
        &[
            "refusal: a duration past the arithmetic",
            "past: an obliged bound",
            "past: an obliged bound of the system's own level",
            "past: an obliged bound of a service",
            "past: an obliged bound of a policy",
            "past: an obliged bound of a platform",
            "past: an obliged bound of a block",
            "past: several, named in order",
            "receivers: an expiry",
            "receivers: the system's own level alone",
            "instance: an obliged fact with its bound",
            "instance: an obliged fact bare",
            "instance: a platform's obliged bound",
            "instance: a block's obliged bound",
            "instance: a policy's obliged bound",
            "instance: a service's obliged bound",
            "hardware: an expiry running its code",
            "context: an expiry running its requester",
        ],
    );
}

#[test]
fn a_named_edge_from_hardware_a_service_or_a_policy_names() {
    let cases = named_hardware();
    // 2 namings; what the service and the policy each name, of 5; `m.blk` and `m.soc` each requiring an output or not;
    // the service bound to `m.blk` or not, the policy to `m.soc` or not; `m.blk` composed or not; who uses the two, of
    // 3.
    assert_eq!(cases.len(), 2 * 5 * 5 * (1 << 5) * 3);
    let tally = run("named hardware", &cases);
    // Unresolved: the 1,200 where the service needs `m.blk`'s offer and no platform composes `m.blk` (2 namings, 25
    // choices of what the two name, 8 of the other three, 3 of who uses them), each `missing-fact`: a block only a
    // service or a policy names is no hardware, a requester bound to it is a conflict (model §8 rule 7), and nothing
    // else offers `transfer-engine`. Every other lowered.
    let bound_outside: BTreeSet<String> = cases
        .iter()
        .filter(|c| {
            c.label.contains(", bound to m.blk;") && c.label.contains("composed by nothing")
        })
        .map(|c| c.label.clone())
        .collect();
    assert_eq!(bound_outside.len(), 1_200);
    assert_eq!(
        tally
            .unresolved_systems
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        bound_outside,
        "the systems the search resolves no plan for are those bound to `m.blk` outside the hardware"
    );
    assert!(
        tally
            .unresolved_systems
            .values()
            .all(|verdict| verdict == "MissingFact"),
        "the search's answer: {:?}",
        tally.unresolved_systems
    );
    hold(
        "named hardware",
        &tally,
        (4_800, 3_600, 0, 1_200, 0),
        &[
            "initialisation: a named edge from a block a service names by `uses`",
            "initialisation: a named edge from a block a service names by `needs`",
            "initialisation: a named edge from a platform a service names by `uses`",
            "initialisation: a named edge from a platform a service names by `needs`",
            "initialisation: a named edge from a block a policy names by `uses`",
            "initialisation: a named edge from a block a policy names by `needs`",
            "initialisation: a named edge from a platform a policy names by `uses`",
            "initialisation: a named edge from a platform a policy names by `needs`",
            "initialisation: a named edge from hardware a provider alone",
            "initialisation: a named edge from hardware bound for its own requirements alone",
            "initialisation: a named edge from hardware a provider and bound for its own requirements",
            "initialisation: a named edge from a block no hardware, served through its namer",
            "initialisation: a named edge from hardware beside a binding's, one line",
            "initialisation: a named edge from hardware, the one path between two steps, ordering them unlike their \
             texts",
            "initialisation: a named edge from hardware bound for nothing and bound to by nothing",
            "initialisation: hardware a node, named by a service or a policy the system does not serve, no edge",
            "named: a block a service names, bound for nothing, a node",
            "named: a platform a policy names, bound for nothing, a node",
            "named: hardware a service or a policy the system does not serve names, no node",
            "initialisation: a composite bound to its part, its edge kept",
            "owed: a service or a policy in a job, its call cost and masked runs",
            "owed: a block or a platform in a job, its call cost and masked runs",
            "owed: a node in no job, neither its call cost nor masked runs",
            "served: a block a service names, no hardware, bound for requirements of its own",
        ],
    );
}

#[test]
fn a_named_edge_from_hardware_whose_interrupt_runs_its_namer() {
    let cases = named_hardware_in_interrupts();
    // 2 namings; what the namer requires, of 6, and names, of 5; a service or a policy; used by `t` or the system's own
    // level.
    assert_eq!(cases.len(), 2 * 6 * 5 * 2 * 2);
    let tally = run("named hardware in interrupts", &cases);
    hold(
        "named hardware in interrupts",
        &tally,
        (240, 240, 0, 0, 0),
        &[
            "initialisation: a named edge from a block a service names by `uses`",
            "initialisation: a named edge from a block a service names by `needs`",
            "initialisation: a named edge from a block a policy names by `uses`",
            "initialisation: a named edge from a block a policy names by `needs`",
            "initialisation: a named edge from hardware a provider alone",
            "initialisation: a named edge from hardware beside a binding's, one line",
            "initialisation: a named edge from hardware an instance programs, beside the instance's path",
            "initialisation: a named edge from hardware whose own interrupt runs its namer",
            "initialisation: a named edge from hardware bound for nothing and bound to by nothing",
            "named: hardware named and bound for nothing, in a job",
            "named: hardware named and bound for nothing, in no job",
            "context: an expiry running its requester",
            "multiplex: multiplexed by a job and an interrupt",
            "owed: a service or a policy in a job, its call cost and masked runs",
            "owed: a block or a platform in a job, its call cost and masked runs",
            "owed: an instance in a job, its call cost and masked runs",
            "owed: a node in no job, neither its call cost nor masked runs",
        ],
    );
}

/// Whether `line` names `name` as a word of its own: a plan's tokens are separated by spaces and parentheses (§3).
fn mentions(line: &str, name: &str) -> bool {
    line.split([' ', '(', ')']).any(|token| token == name)
}

#[test]
fn hardware_a_served_requester_names_a_node_bound_or_not() {
    let cases = unbound_named_hardware();
    // What is named, of 3; which of 5 namers name it; by `uses` or `needs`; the three services used by `t` or by the
    // system's own level; `w` bound to it or not.
    assert_eq!(cases.len(), 3 * (1 << 5) * 2 * 2 * 2);
    let (tally, plans) = run_keeping("unbound named hardware", &cases, true);
    // Unresolved: the 32 where `w` is bound to `m.blk`, which no platform composes and neither `t` nor the system's own
    // level names (8 subsets of the three services, 2 clauses, 2 users), each `missing-fact`: a block only a service or
    // a policy names is no hardware, a requester bound to it is a conflict (model §8 rule 7), and nothing else offers
    // `transfer-engine`. Every other lowered.
    let outside: BTreeSet<String> = cases
        .iter()
        .filter(|c| {
            c.label.starts_with("Alone ")
                && c.label.ends_with("w bound to it")
                && !c.label.contains("\"t\"")
                && !c.label.contains("\"sys\"")
        })
        .map(|c| c.label.clone())
        .collect();
    assert_eq!(outside.len(), 32);
    assert_eq!(
        tally
            .unresolved_systems
            .keys()
            .cloned()
            .collect::<BTreeSet<_>>(),
        outside,
        "the systems the search resolves no plan for are those binding `w` to `m.blk` outside the hardware"
    );
    assert!(
        tally
            .unresolved_systems
            .values()
            .all(|verdict| verdict == "MissingFact"),
        "the search's answer: {:?}",
        tally.unresolved_systems
    );
    hold(
        "unbound named hardware",
        &tally,
        (768, 736, 0, 32, 0),
        &[
            "named: a block a task names, bound for nothing, a node",
            "named: a platform a task names, bound for nothing, a node",
            "named: a block a service names, bound for nothing, a node",
            "named: a platform a service names, bound for nothing, a node",
            "named: a block a policy names, bound for nothing, a node",
            "named: a platform a policy names, bound for nothing, a node",
            "named: a block a source's handler names, bound for nothing, a node",
            "named: a platform a source's handler names, bound for nothing, a node",
            "named: hardware named by `uses`, bound for nothing, a node",
            "named: hardware named by `needs`, bound for nothing, a node",
            "named: a block no hardware, which a service or a policy names, bound for nothing, a node",
            "named: hardware named and bound for nothing, in a job",
            "named: hardware named and bound for nothing, in no job",
            "named: hardware named and bound for nothing, in an interrupt that runs its namer",
            "named: hardware named and bound for nothing, a provider only of requesters that do not name it",
            "named: hardware the system's own level names, no node",
            "named: a block hardware a served requester names composes, nothing calling nor binding it, no node",
            "initialisation: a named edge from a block a service names by `uses`",
            "initialisation: a named edge from a block a service names by `needs`",
            "initialisation: a named edge from a platform a service names by `uses`",
            "initialisation: a named edge from a platform a service names by `needs`",
            "initialisation: a named edge from a block a policy names by `uses`",
            "initialisation: a named edge from a block a policy names by `needs`",
            "initialisation: a named edge from a platform a policy names by `uses`",
            "initialisation: a named edge from a platform a policy names by `needs`",
            "initialisation: a named edge from hardware bound for nothing and bound to by nothing",
            "initialisation: a named edge from hardware a provider alone",
            "initialisation: a named edge from a block no hardware, served through its namer",
            "initialisation: a named edge from hardware, the one path between two steps, ordering them unlike their \
             texts",
            "owed: a block or a platform in a job, its call cost and masked runs",
            "owed: a service or a policy in a job, its call cost and masked runs",
            "owed: a node in no job, neither its call cost nor masked runs",
        ],
    );
    // "placed when an unrelated binding makes it a node" (R18-D1): each system beside its twin, the one with `w` bound
    // to the named hardware, where a served task, service or policy names it — `w`, used by the system's own level
    // alone, runs in no job and no interrupt, so a provider of it takes no context from it (§2.6). Every line naming the
    // hardware, its context, its edges to its namers and what it owes, is the same in both, but for `w`'s own lines and
    // the steps, which `w`'s node renumbers. Compared: 3 targets × 30 subsets with a served namer × 2 clauses × 2 users,
    // less the 28 whose twin the search refuses (7 subsets of the three services alone × 2 × 2).
    let mut compared = 0;
    for (pair, plans) in cases.chunks(2).zip(plans.chunks(2)) {
        let [alone, beside] = pair else {
            unreachable!("pairs")
        };
        assert_eq!(
            alone.label.strip_suffix("nothing bound to it"),
            beside.label.strip_suffix("w bound to it"),
            "a system and its twin"
        );
        let target = if alone.label.starts_with("Platform ") {
            "m.soc"
        } else {
            "m.blk"
        };
        let served_namer = ["\"t\"", "\"a.s\"", "\"a.p\"", "\"a.h\""]
            .iter()
            .any(|n| alone.label.contains(n));
        let (Some(without), Some(with)) = (&plans[0], &plans[1]) else {
            continue;
        };
        if !served_namer {
            continue;
        }
        let naming = |plan: &str| -> Vec<String> {
            plan.lines()
                .filter(|l| mentions(l, target) && !mentions(l, "w") && !l.starts_with("(step "))
                .map(str::to_string)
                .collect()
        };
        assert!(
            naming(without)
                .iter()
                .any(|l| l.starts_with(&format!("(context {target} "))),
            "{}: `{target}`, named by a served requester, a node\n{without}",
            alone.label
        );
        assert_eq!(
            naming(without),
            naming(with),
            "{}: `w`'s binding changes what the name gives",
            alone.label
        );
        compared += 1;
    }
    assert_eq!(compared, 3 * 30 * 2 * 2 - 28);
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
fn the_message_readers_keep_the_order_a_message_gives() {
    // The checker's readers of a refusal's prose, held to messages written here in words of their own, so that a
    // fault of a reader shows here, never as a verdict on the code: each returns what a message names in the order it
    // names it, which the checker then holds to §1's order — so a message sorted by its words, `y.b` before `y`, is
    // read as written and differs from the reading.
    let declared: BTreeSet<String> = ["m", "m.s", "y", "y.b", "y!"]
        .iter()
        .map(|s| (*s).to_string())
        .collect();
    let pair = |namer: &str, named: &str| (namer.to_string(), named.to_string());
    assert_eq!(
        pairs_named_in(
            "refused: `(task sys t!)` names the system `y`; `m.s` names `y.b`, `y` and `y!`: no system is used",
            &declared
        ),
        [
            pair("(task sys t!)", "y"),
            pair("m.s", "y.b"),
            pair("m.s", "y"),
            pair("m.s", "y!")
        ]
    );
    let hardware: BTreeSet<String> = ["d", "d.x"].iter().map(|s| (*s).to_string()).collect();
    let expiry = "(adapter deadline-over-delay d (task sys t))";
    assert_eq!(
        received_named(
            &format!(
                "`{expiry}` delivers an expiry to `(task sys t)`; `(task sys t!)` programs an alarm at `d.x` \
                 itself; `sys` programs alarms at `d` and `d.x`: none is placed"
            ),
            "sys",
            &hardware
        ),
        Ok(vec![
            pair("(task sys t)", expiry),
            pair("(task sys t!)", "d.x"),
            pair("sys", "d"),
            pair("sys", "d.x"),
        ])
    );
    assert!(received_named(
        "`(task sys t)` and `sys` program an alarm at `d`",
        "sys",
        &hardware
    )
    .is_err());
    assert_eq!(
        durations_named(
            "past it: `(source dev!)` states a separation; `(source dev)` too; \
             `(adapter deadline-over-delay timer.d dl!)` is obliged to a `delivery-bound`"
        ),
        [
            "(source dev!)",
            "(source dev)",
            "(adapter deadline-over-delay timer.d dl!) delivery-bound"
        ]
    );
    assert!(clause_named(
        "`(task sys t)` writes `(period 20 ms)` and is bound to a source",
        "period"
    ));
    assert!(!clause_named(
        "`(task sys t)` is bound to a source itself",
        "period"
    ));
    // The reading's order, by texts compared byte by byte, against the words a message names them with.
    let texts =
        |items: &[&str]| -> Vec<String> { items.iter().map(|s| (*s).to_string()).collect() };
    assert!(worded_otherwise(&texts(&["y", "y.b"]), |t| format!(
        "`{t}`"
    )));
    assert!(worded_otherwise(&texts(&["m", "m!"]), |t| format!("`{t}`")));
    assert!(!worded_otherwise(&texts(&["m!", "m.s"]), |t| format!(
        "`{t}`"
    )));
    let tasks = texts(&["(task sys t!)", "(task sys t)"]);
    assert!(
        tasks[0] < tasks[1],
        "`!` lies below the `)` that closes `(task sys t)`"
    );
    assert!(worded_otherwise(&tasks, |t| t
        .trim_end_matches(')')
        .rsplit(' ')
        .next()
        .expect("a name")
        .to_string()));
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
    let record = the_record();
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

/// The record's text.
fn the_record() -> String {
    std::fs::read_to_string(common::root().join("docs/decisions/decision_runtime-plan.md"))
        .expect("the record")
}

/// The words a text writes between backquotes, in its order.
fn backquoted(text: &str) -> Vec<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .map(str::to_string)
        .collect()
}

/// The facts §2.10's table owes `subject`, as its row writes them: `| SUBJECT | FACTS | OWNER |`, with that owner.
fn owed_row(record: &str, subject: &str) -> (Vec<String>, String) {
    let rows: Vec<Vec<&str>> = record
        .lines()
        .map(str::trim)
        .filter(|line| line.starts_with('|') && line.ends_with('|'))
        .map(|line| line.split('|').map(str::trim).collect::<Vec<_>>())
        .filter(|cells| cells.len() == 5 && cells[1] == subject)
        .collect();
    assert_eq!(rows.len(), 1, "§2.10's one row for {subject}");
    let owner = backquoted(rows[0][3]);
    assert_eq!(owner.len(), 1, "{subject}'s one owner");
    (backquoted(rows[0][2]), owner[0].clone())
}

/// §3's `FACT` list: "`FACT` in `owed` is one of: ...", up to "or an obliged fact's name".
fn fact_list(record: &str) -> BTreeSet<String> {
    let opening = "- `FACT` in `owed` is one of:";
    let from = record.find(opening).expect("§3's `FACT` list") + opening.len();
    let to = from
        + record[from..]
            .find("or an obliged")
            .expect("its last member, an obliged fact's name");
    backquoted(&record[from..to]).into_iter().collect()
}

#[test]
fn the_reading_gives_the_record_s_worked_texts() {
    // The reading's facts of a task, a source and a timer route, held to §2.10's table and §3's `FACT` list as the
    // record writes them (round 19, R19-D2: a task's `os-calls-category`).
    let record = the_record();
    let listed = fact_list(&record);
    for (subject, facts, owner) in [
        ("each task", &TASK_FACTS[..], "application"),
        ("each source", &SOURCE_FACTS[..], "realization"),
        ("each timer route", &TIMER_ROUTE_FACTS[..], "realization"),
    ] {
        assert_eq!(
            owed_row(&record, subject),
            (names(facts), owner.to_string()),
            "§2.10's row for {subject}"
        );
        for fact in facts {
            assert!(listed.contains(*fact), "§3's `FACT` list lacks `{fact}`");
        }
    }

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
