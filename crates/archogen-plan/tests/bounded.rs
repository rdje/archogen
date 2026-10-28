//! The runtime plan's bounded exhaustive checker (leaf `M4.1.13`), which
//! `docs/decisions/decision_executable-design-reviews.md` item 1 asks of a design whose subject is a decision
//! procedure: it "enumerates a finite universe of inputs" and holds each outcome to the record. Six universes of small
//! descriptions are generated in memory, every combination of a few independent choices, so that each is exhaustive
//! over its bounds: three services and their `uses` edges; two services with adapter instances; releases and routes of
//! every separation class; requesters of every kind; two tasks, periodic and sporadic, sharing services that take a
//! channel, with an obliged `delivery-bound` bound and bare and each task writing `(deadline-from release)` or not; and
//! receivers of interrupt code, two tasks and the system's own level each programming an alarm itself, receiving an
//! expiry or neither, in both runtime orders.
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
//! facts; and the last line, `(complete no)`. It derives which of §1's refusals is due, where these universes reach
//! one, and holds the lowering's refusal to its code, to the subjects it names — each told by its exact text, a task's
//! `(task SYSTEM NAME)` form or the system's own level's name as a word of its own, never by a substring — in §1's
//! order, and to what each would receive. The reading is itself held to the record's three worked texts (§3).
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

/// What a source's block states of its own `min-arrival-separation`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Separation {
    /// Stated, in milliseconds.
    Stated(u64),
    /// Not named at all.
    Unstated,
    /// Declared absent.
    Absent,
}

/// A block: its offers, as written, and what the reading needs of it.
#[derive(Debug, Clone)]
struct Block {
    name: String,
    offers: Vec<String>,
    absent: Vec<String>,
    /// For a block offering `interrupt-source`: what it states of its separation.
    separation: Option<Separation>,
}

/// What one requester writes: the vocabulary facts its `needs` lists, the declarations its `uses` lists, each
/// `(FACT (at-most N ms))` bound it writes, and any other requirement clause, as written.
#[derive(Debug, Clone, Default)]
struct Side {
    needs: Vec<String>,
    uses: Vec<String>,
    /// Each `(FACT (at-most N ms))` the requester writes, as `(FACT, N)`.
    at_most: Vec<(String, u64)>,
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
        for (fact, ms) in &self.at_most {
            out.push_str(&format!(" ({fact} (at-most {ms} ms))"));
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

/// A task's release model, in milliseconds.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Release {
    Period(u64),
    MinSeparation(u64),
}

#[derive(Debug, Clone)]
struct Task {
    name: String,
    release: Release,
    priority: u32,
    /// Whether it writes `(deadline-from release)`.
    deadline_from: bool,
    side: Side,
}

/// One description of one system, on one platform `soc` that uses every block.
#[derive(Debug, Clone)]
struct Description {
    system: String,
    blocks: Vec<Block>,
    services: Vec<Service>,
    tasks: Vec<Task>,
    /// The system's own level.
    own: Side,
}

impl Description {
    /// The description's text, as the language reads it.
    fn text(&self) -> String {
        let mut out = String::from("(eadl-version eadl/1)\n");
        for block in &self.blocks {
            out.push_str(&format!(
                "(defblock {} (offers {})",
                block.name,
                block.offers.join(" ")
            ));
            if !block.absent.is_empty() {
                out.push_str(&format!(" (absent {})", block.absent.join(" ")));
            }
            out.push_str(")\n");
        }
        let blocks: Vec<&str> = self.blocks.iter().map(|b| b.name.as_str()).collect();
        out.push_str(&format!(
            "(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses {})))\n",
            blocks.join(" ")
        ));
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
        out.push_str(&format!("(defsystem {}\n", self.system));
        for task in &self.tasks {
            let (release, ms) = match task.release {
                Release::Period(ms) => ("period", ms),
                Release::MinSeparation(ms) => ("min-separation", ms),
            };
            out.push_str(&format!(
                "  (task {} ({release} {ms} ms) (deadline {ms} ms){} (priority {}){})\n",
                task.name,
                if task.deadline_from {
                    " (deadline-from release)"
                } else {
                    ""
                },
                task.priority,
                task.side.clauses()
            ));
        }
        let own = self.own.clauses();
        if !own.is_empty() {
            out.push_str(&format!("  (requires{own})\n"));
        }
        out.push_str("  (platform (uses soc)))\n");
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
    if let Separation::Stated(ms) = separation {
        offers.push(format!("(min-arrival-separation {ms} ms)"));
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
        absent: Vec::new(),
        separation: None,
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
                .push(("delivery-bound".to_string(), DELIVERY_BOUND_MS));
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
                    release: Release::Period(10),
                    priority: 1,
                    deadline_from: true,
                    side: Side {
                        uses: task_uses.clone(),
                        ..Side::default()
                    },
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
                                release: Release::Period(10),
                                priority: 1,
                                deadline_from: true,
                                side: named(Side::default(), subset(t_set), by_needs),
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
            Release::MinSeparation(20)
        } else {
            Release::Period(20)
        },
        priority: 1,
        deadline_from: true,
        side: t,
    }];
    if let Some(sporadic) = u_kind {
        tasks.push(Task {
            name: "u".to_string(),
            release: if sporadic {
                Release::MinSeparation(40)
            } else {
                Release::Period(40)
            },
            priority: 2,
            deadline_from: true,
            side: u,
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
                            Release::MinSeparation(20)
                        } else {
                            Release::Period(20)
                        },
                        priority: 1,
                        deadline_from: true,
                        side: t,
                    }],
                    own,
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
                                release: Release::Period(10),
                                priority: 1,
                                deadline_from: t_from,
                                side: named(Side::default(), subset(t_set), false),
                            },
                            Task {
                                name: "u".to_string(),
                                release: Release::MinSeparation(20),
                                priority: 2,
                                deadline_from: u_from,
                                side: named(Side::default(), subset(u_set), false),
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
                                        Release::MinSeparation(20)
                                    } else {
                                        Release::Period(20)
                                    },
                                    priority,
                                    deadline_from: from,
                                    side: Side {
                                        needs: programs.needs(),
                                        ..Side::default()
                                    },
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

fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

/// §2.2: "Each is one reduced rational number of seconds, `N s` or `N/D s`, with `D > 1` and `N` and `D` coprime".
fn duration(ms: u64) -> String {
    let g = gcd(ms, 1000);
    let (n, d) = (ms / g, 1000 / g);
    if d == 1 {
        format!("{n} s")
    } else {
        format!("{n}/{d} s")
    }
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

/// The record's reading of one system: every line of the kinds it derives.
struct Reading {
    lines: Vec<String>,
    /// The classes of the record's rules this system exercises, so that a universe's reach is counted.
    classes: BTreeSet<&'static str>,
}

/// The plan `docs/decisions/decision_runtime-plan.md` gives `description` with the assignment `binds`, for the lines
/// the reading derives.
fn read_the_record(description: &Description, binds: &[Bind]) -> Reading {
    let system = description.system.as_str();
    let task_text = |task: &Task| format!("(task {system} {})", task.name);
    let declared: BTreeSet<&str> = description
        .services
        .iter()
        .map(|s| s.name.as_str())
        .chain(description.blocks.iter().map(|b| b.name.as_str()))
        .collect();

    // §2: a requester reaches what it names by `uses` or `needs` — a declaration, never a vocabulary fact — and what
    // it is bound to as its provider, at any depth.
    let mut edges: BTreeMap<String, BTreeSet<String>> = BTreeMap::new();
    let mut name = |who: String, side: &Side| {
        for named in side.uses.iter().chain(&side.needs) {
            if declared.contains(named.as_str()) {
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
    for bind in binds {
        edges
            .entry(bind.requester.clone())
            .or_default()
            .insert(bind.provider.clone());
    }

    // §2: "A **node** below is a node of the joint rules' initialization graph (§2.7): a provider an active binding
    // names (a block, a platform or an adapter's instance), a service, or a policy". A task and the system's own level
    // are requesters but never nodes. A service is the system's where the system or a task of it reaches it.
    let mut served = reach(&edges, system);
    for task in &description.tasks {
        served.extend(reach(&edges, &task_text(task)));
    }
    let mut nodes: BTreeSet<String> = binds.iter().map(|b| b.provider.clone()).collect();
    for service in &description.services {
        if served.contains(&service.name) {
            nodes.insert(service.name.clone());
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
    let separation_of = |provider: &str| -> Separation {
        description
            .blocks
            .iter()
            .find(|b| b.name == provider)
            .and_then(|b| b.separation)
            .unwrap_or_else(|| panic!("`{provider}` is a source block of the description"))
    };

    // §2.5, the order among routes: "Each route gets a distinct **rank**, rank `1` taken first": the sources with a
    // stated separation, ascending by it, ties broken on their interrupt's text; then the sources whose separation is
    // `unknown` or `absent`, by their interrupt's text; then the timer routes, by their interrupt's text.
    let mut ranked: Vec<&Route> = routes.values().collect();
    ranked.sort_by_key(|route| {
        let (class, by) = if route.source {
            match separation_of(&route.provider) {
                Separation::Stated(ms) => (0, ms),
                Separation::Unstated | Separation::Absent => (1, 0),
            }
        } else {
            (2, 0)
        };
        (class, by, route.item.clone())
    });
    let mut classes = BTreeSet::new();
    let stated: Vec<u64> = routes
        .values()
        .filter(|r| r.source)
        .filter_map(|r| match separation_of(&r.provider) {
            Separation::Stated(ms) => Some(ms),
            _ => None,
        })
        .collect();
    if stated
        .iter()
        .enumerate()
        .any(|(i, a)| stated[i + 1..].contains(a))
    {
        classes.insert("rank: two stated separations tied");
    }
    let unstated = routes.values().any(|r| {
        r.source
            && matches!(
                separation_of(&r.provider),
                Separation::Unstated | Separation::Absent
            )
    });
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
    let side_of = |requester: &str| -> Option<&Side> {
        if requester == system {
            return Some(&description.own);
        }
        description
            .tasks
            .iter()
            .find(|t| task_text(t) == requester)
            .map(|t| &t.side)
            .or_else(|| {
                description
                    .services
                    .iter()
                    .find(|s| s.name == requester)
                    .map(|s| &s.side)
            })
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
                    .map(|(_, ms)| *ms)
                    .min()
            });
            match bound {
                Some(ms) => {
                    classes.insert("instance: an obliged fact with its bound");
                    line.push_str(&format!(" (obliged {fact} {})", duration(ms)));
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
                        }
                    }
                } else {
                    classes.insert("context: a wake running the instance alone");
                }
            } else if nodes.contains(&bind.requester) && route.requesters.contains(&bind.requester)
            {
                base.insert(bind.requester.clone());
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

    // §2.2, what releases each task: `timer` for a periodic task; for a task with a minimum separation, the external
    // source whose route names the task or a requester it reaches, `(source NAME every)` "when the source's stated
    // separation is at least the task's"; `unknown` otherwise — no source's route reaching it, several, a source
    // stating no separation or declaring it absent, or one shorter than the task's. "A timer route releases no task."
    let mut released_by = BTreeMap::new();
    for task in &description.tasks {
        let trigger = match task.release {
            Release::Period(_) => "timer".to_string(),
            Release::MinSeparation(own) => {
                let mut reached = reach(&edges, &task_text(task));
                reached.insert(task_text(task));
                let sources: Vec<&Route> = routes
                    .values()
                    .filter(|r| r.source && r.requesters.iter().any(|q| reached.contains(q)))
                    .collect();
                let (trigger, class) = match sources.as_slice() {
                    [one] => match separation_of(&one.provider) {
                        Separation::Stated(ms) if ms >= own => (
                            format!("(source {} every)", one.provider),
                            if ms == own {
                                "trigger: every, equal separations"
                            } else {
                                "trigger: every, a longer separation"
                            },
                        ),
                        Separation::Stated(_) => (
                            "unknown".to_string(),
                            "trigger: unknown, a shorter separation",
                        ),
                        Separation::Unstated => (
                            "unknown".to_string(),
                            "trigger: unknown, no separation stated",
                        ),
                        Separation::Absent => (
                            "unknown".to_string(),
                            "trigger: unknown, the separation absent",
                        ),
                    },
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
    // as zero where none is stated", and "its overrun policy, `fault` where none is stated" — these descriptions state
    // neither; and "what it names by `uses` or `needs` that is a service or a policy, each once: a fact or a block it
    // names is no member", by name.
    let declared_services: BTreeSet<&str> = description
        .services
        .iter()
        .map(|s| s.name.as_str())
        .collect();
    let mut task_lines: Vec<(usize, String)> = Vec::new();
    for task in &description.tasks {
        let index = description
            .tasks
            .iter()
            .filter(|other| other.priority < task.priority)
            .count();
        let (release, ms) = match task.release {
            Release::Period(ms) => ("period", ms),
            Release::MinSeparation(ms) => ("min-separation", ms),
        };
        let members: BTreeSet<&str> = task
            .side
            .uses
            .iter()
            .chain(&task.side.needs)
            .map(String::as_str)
            .filter(|n| declared_services.contains(n))
            .collect();
        if task.deadline_from {
            classes.insert("task: deadline-from written");
        } else {
            classes.insert("task: deadline-from owed");
        }
        task_lines.push((
            index,
            format!(
                "(task {index} {} (rank {}) ({release} {}) (deadline {}) {} (jitter 0 s) (overrun fault) \
                 (released-by {}) {})",
                task_text(task),
                task.priority,
                duration(ms),
                duration(ms),
                if task.deadline_from {
                    "(deadline-from release)"
                } else {
                    "(deadline-from)"
                },
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
                Separation::Stated(ms) => duration(ms),
                Separation::Unstated => "unknown".to_string(),
                Separation::Absent => "absent".to_string(),
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
    // tasks ... are none". Composition is a block's or a platform's `uses` (rule 1); no block or platform here requires
    // anything, so no binding crosses it, which the reading asserts rather than transcribes.
    #[derive(Default)]
    struct Reasons {
        facts: BTreeSet<String>,
        named: bool,
    }
    let mut graph: BTreeMap<(String, String), Reasons> = BTreeMap::new();
    for bind in binds {
        assert!(
            !description.blocks.iter().any(|b| b.name == bind.requester) && bind.requester != "soc",
            "the reading transcribes no composition, and `{}` is a block or a platform requiring",
            bind.requester
        );
        if nodes.contains(&bind.requester)
            && nodes.contains(&bind.provider)
            && bind.requester != bind.provider
        {
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

    // §2.9: "A service or a policy that takes a channel is **multiplexed** where it has two or more **users**: each of
    // the system's tasks that reaches it (§2), directly or through services, policies and bindings; and each interrupt
    // whose context line (§2.6) holds it, but for its own timer route — at the provider whose channel it takes, or that
    // the instance whose channel it takes programs — where nothing else that route runs reaches it". "the system's own
    // level is none".
    let mut multiplexed = BTreeSet::new();
    for service in description
        .services
        .iter()
        .map(|s| &s.name)
        .filter(|s| nodes.contains(*s))
    {
        let channels: Vec<&Bind> = binds
            .iter()
            .filter(|b| &b.requester == service && CONSUMED.contains(&b.fact.as_str()))
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
            if reach(&edges, &task_text(task)).contains(service) {
                users.insert(task_text(task));
            }
        }
        for (item, run) in &runs {
            if !run.contains(service) {
                continue;
            }
            if own_routes.contains(item) {
                // "that route's own code delivering to it is no caller, but another node's code there calling it is"
                if run
                    .iter()
                    .any(|n| n != service && reach(&edges, n).contains(service))
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
            multiplexed.insert(service.clone());
            classes.insert("multiplex: multiplexed");
            let jobs = description
                .tasks
                .iter()
                .filter(|t| users.contains(&task_text(t)))
                .count();
            if jobs == 0 {
                classes.insert("multiplex: multiplexed by interrupts alone");
            } else if jobs >= 2 {
                classes.insert("multiplex: multiplexed by two jobs");
            }
            if description
                .services
                .iter()
                .any(|s| &s.name == service && s.policy)
            {
                classes.insert("multiplex: a policy multiplexed");
            }
        } else if users.len() == 1 {
            classes.insert("multiplex: a channel with one user");
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
    for service in &multiplexed {
        obligations.push(format!(
            "(obligation multiplex {service} (discharged-by M4.2 M2.21.2))"
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
        if !task.deadline_from {
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
    for service in &multiplexed {
        owe(service, "overhead", "realization");
    }
    for route in routes.values() {
        if route.source {
            if separation_of(&route.provider) == Separation::Unstated {
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

    Reading { lines, classes }
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

/// The kinds of line the reading derives, as the checker reports them: every line of a plan.
const KINDS_OF_LINE: [&str; 15] = [
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
];

/// Which of the kinds `line` is: `multiplex` is the `(obligation multiplex …)` lines and the `overhead` they owe
/// (§2.9).
fn kind_of_line(line: &str) -> Option<&'static str> {
    let head = line.split(' ').next().unwrap_or_default();
    match head {
        "(format" | "(profile" | "(system" | "(program" | "(assignment" | "(adapters" => {
            Some("identity")
        }
        "(interrupt-model" => Some("interrupt-model"),
        "(complete" => Some("complete"),
        "(task" => Some("task"),
        "(bind" => Some("bind"),
        "(depends" => Some("depends"),
        "(step" => Some("step"),
        "(context" => Some("context"),
        "(route" => Some("route"),
        "(enabled" => Some("enabled"),
        "(timer" => Some("timer"),
        "(instance" => Some("instance"),
        "(obligation" if line.starts_with("(obligation multiplex ") => Some("multiplex"),
        "(obligation" => Some("obligation"),
        "(owed" => {
            let form = form(line);
            if text_of(&items(&form)[2]) == "overhead" {
                Some("multiplex")
            } else {
                Some("owed")
            }
        }
        _ => None,
    }
}

fn by_kind<'a>(lines: impl IntoIterator<Item = &'a str>) -> BTreeMap<&'static str, Vec<String>> {
    let mut out: BTreeMap<&'static str, Vec<String>> =
        KINDS_OF_LINE.iter().map(|k| (*k, Vec::new())).collect();
    for line in lines {
        if let Some(kind) = kind_of_line(line) {
            out.get_mut(kind).expect("a kind").push(line.to_string());
        }
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

/// A refusal §1 gives one system.
struct Due {
    code: &'static str,
    /// The subjects it names, in §1's order: a task's `(task SYSTEM NAME)`, or the system's name for its own level.
    subjects: Vec<String>,
    /// The instances whose `expiry` a receiver would receive, by their `(adapter …)` text.
    expiries: Vec<String>,
    /// The providers at which a receiver would receive the alarm it programs itself, its timer route.
    alarms: Vec<String>,
    /// The classes of §1's rules it exercises.
    classes: BTreeSet<&'static str>,
}

/// §1, the refusals these universes can reach, the first that applies, from the description and the record's own
/// `bind` lines: "They are taken in the order written, this list and then the next, ... and the first that applies is
/// the one given; a bullet over tasks is taken task by task in runtime order".
/// - Of the first list: "a periodic task bound to an interrupt source itself, which states a second release model":
///   `tool-failure`, "naming the task and the clause". (`M1.43` has the language refuse it, and "a description the
///   language's check refuses is no input".)
/// - Of the second, `unsupported-profile`: "a task, or the system's own level, that would receive an interrupt's code:
///   a timer route that names it, bound to its provider itself, or an adapter instance with a cost running its
///   requester's code (`expiry`) whose requester it is, the one an exclusive instance serves or one bound to a shared
///   instance. ... It names every such receiver — each task in runtime order, then the system's own level — with what
///   it would receive, so no order among them decides the message".
///
/// No other bullet applies here: no description names a system, every duration is a few milliseconds, no system has
/// placement memory, each declares a task, and every deadline is measured from its release or from nothing written.
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
            return Some(Due {
                code: "tool-failure",
                subjects: vec![task_text(task)],
                expiries: Vec::new(),
                alarms: Vec::new(),
                classes: BTreeSet::from(["refusal: a periodic task bound to a source itself"]),
            });
        }
    }
    let mut due = Due {
        code: "unsupported-profile",
        subjects: Vec::new(),
        expiries: Vec::new(),
        alarms: Vec::new(),
        classes: BTreeSet::new(),
    };
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
                    due.expiries.push(bind.provider.clone());
                    due.classes.insert("receivers: an expiry");
                }
            } else if CONSUMED.contains(&bind.fact.as_str()) {
                receives = true;
                due.alarms.push(bind.provider.clone());
                due.classes.insert("receivers: an alarm programmed itself");
            }
        }
        if receives {
            due.subjects.push(requester);
        }
    }
    if due.subjects.is_empty() {
        return None;
    }
    due.classes.insert(match due.subjects.len() {
        1 => "receivers: one",
        2 => "receivers: two",
        _ => "receivers: three",
    });
    let tasks: Vec<&String> = due.subjects.iter().filter(|s| *s != system).collect();
    if tasks.len() < due.subjects.len() {
        due.classes.insert(if tasks.is_empty() {
            "receivers: the system's own level alone"
        } else {
            "receivers: tasks and the system's own level"
        });
    }
    if tasks.windows(2).any(|w| w[0] > w[1]) {
        due.classes
            .insert("receivers: runtime order unlike the tasks' text order");
    }
    Some(due)
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

/// Where the lowering's `refusal` is not the one `due`: its code, the subjects it names in their order, and what it
/// says they would receive. §1 fixes the receivers and their order, not the message's wording of what each receives;
/// so an expiry is held to its instance's text, written whole, and an alarm a receiver programs itself to its
/// provider's name, written outside any instance's text — an interrupt being "named by its kind and its provider" (§2).
fn refusal_disagrees(due: &Due, refusal: &Refusal, system: &str) -> Option<String> {
    let mut wrong = Vec::new();
    if refusal.code != due.code {
        wrong.push(format!(
            "its code is `{}`, the record's `{}`",
            refusal.code, due.code
        ));
    }
    let named = subjects_named(&refusal.message, system);
    if named != due.subjects {
        wrong.push(format!(
            "it names {named:?}, the record {:?}, in that order",
            due.subjects
        ));
    }
    let forms = forms_in(&refusal.message);
    let words = words_outside_instances(&refusal.message);
    let unsaid: Vec<&String> = due
        .expiries
        .iter()
        .filter(|instance| !forms.contains(*instance))
        .chain(
            due.alarms
                .iter()
                .filter(|provider| !words.contains(*provider)),
        )
        .collect();
    if !unsaid.is_empty() {
        wrong.push(format!(
            "it does not name what would be received: {unsaid:?}"
        ));
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
                        due.subjects.join(", ")
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
                Some(due) => refusal_disagrees(due, &refusal, &case.description.system),
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
                            due.subjects.join(", ")
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
            deadline_from,
            side: Side {
                uses: names(uses),
                ..Side::default()
            },
        };
    let plain = |name: &str, offers: &[&str]| Block {
        name: name.to_string(),
        offers: names(offers),
        absent: Vec::new(),
        separation: None,
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
            task("beat", Release::Period(10), 1, true, &["console.write"]),
            task("chime", Release::Period(30), 2, true, &["console.write"]),
        ],
        own: uses(&["console.write"]),
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
            Release::Period(10),
            1,
            true,
            &["time.monotonic"],
        )],
        own: uses(&["time.monotonic"]),
    };
    let sporadic = Description {
        system: "sporadic".to_string(),
        blocks: vec![source_block("dev.e", Separation::Stated(20), "")],
        services: vec![service("evt", Counter::None, true, Vec::new())],
        tasks: vec![task("s", Release::MinSeparation(10), 1, false, &["evt"])],
        own: uses(&["evt"]),
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
