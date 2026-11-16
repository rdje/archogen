//! `archogen-runtime-plan/2` read back (`docs/decisions/decision_runtime-plan-format-2.md` §3; leaf `M4.1.7.7`): what a
//! consumer of a plan on a target, or on the hosted playground with its application's inputs, reads, typed, from the one
//! canonical text — "consumers read `/2` as they read `/1`: by a reader held to its goldens, never by matching text".
//!
//! ⭐ **The text is the plan, and this reads exactly it**, as [`crate::contents`] reads `/1`, on the same forms: each
//! line is one form, its tokens parted by a single space, and the forms come in §3's order, [`crate::identity::LINES`]'s;
//! a line is taken only if writing back what was read gives that line's bytes, so a plan has one spelling, and a value no
//! writer writes, a count with a leading zero, a rate not in lowest terms, a power of two written past where an integer
//! holds it, is refused rather than normalised. Every plan the lowering writes goes through here before it is returned
//! ([`crate::lower_hosted`], [`crate::lower_on_target`]), and `tests/format2_reader.rs` holds it to `/2`'s goldens.
//!
//! What a consumer must look inside is typed: the target, what generates the code, the epoch, the timer, each alarm's and
//! clock's counter, each route with its controller, each region, the placement, each allocation and the image, each
//! context and what is owed. What a consumer compares whole stays the text §3 writes it as: a name, a requester, a node,
//! the application's lines, and a figure a catalog record will state (`M4.1.7.6.10`). The realization's lines are kept as
//! text too, their file's reader being `M4.2`'s with its format (`RPF-H3`).

use eadl_model::rational::Rational;

use crate::contents::{
    duration, duration_under, hex_under, instance, int, one_under, parse, task, texts_under, Bind,
    Depends, Instance, NodeKind, ObligationLine, Owed, Separation, Task, Tree,
};
use crate::identity;

/// The machine a plan is of (§1, "The target in the identity", and "The hosted playground").
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Target {
    /// `(target hosted-playground)`: no machine.
    #[default]
    HostedPlayground,
    /// `(target NAME "HEX")`: the file's one platform and the sha256 of its forms' program text.
    Machine {
        /// `NAME`, the platform.
        platform: String,
        /// `HEX`.
        digest: String,
    },
}

/// A figure a catalog record states, or owed until one does (§3, `FIGURE`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Figure {
    /// `owed`.
    Owed,
    /// `(value DUR) (record ID) (category CATEGORY)`, by its text.
    Stated(String),
}

/// A count as §3 writes one, unconverted: an `INT` to 2^64 − 1, and `(pow2 P)` past that.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Count {
    /// An `INT`.
    Int(u64),
    /// `(pow2 P)`, `P` at least 64.
    Pow2(u32),
}

/// A compare's window (§3, `COUNTER`'s `W`).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Window {
    /// A count of ticks.
    Ticks(Count),
    /// `magnitude`: the device reads a value and the counter as whole numbers.
    Magnitude,
}

/// A device's counter as the target states it (§3, `COUNTER`).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Counter {
    /// `R`, in hertz, exact.
    pub tick_rate: Rational,
    /// `M`.
    pub modulus: Count,
    /// What one write reaches.
    pub reach: Rational,
    /// `S`, a step of its compare.
    pub step: Count,
    /// `W`, none on a delay.
    pub window: Option<Window>,
}

/// The hosted playground's counter in place of `COUNTER` (§3, `SIMULATED`): each as the description states it, or
/// `unstated`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Simulated {
    /// `R`, or nothing where it is `unstated`.
    pub tick_rate: Option<Rational>,
    /// `M`, or nothing where it is `unstated`.
    pub modulus: Option<Count>,
}

/// The epoch's line (§3, §4).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Epoch {
    /// `(epoch simulator)`, a hosted plan's.
    #[default]
    Simulator,
    /// `(epoch after-read (lead FIGURE))`, under a release timer.
    AfterRead(Figure),
    /// `(epoch none)`, or `(epoch none (lead FIGURE))` with an alarm and no release timer.
    None(Option<Figure>),
}

/// The release timer's line (§3).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub enum Timer {
    /// `(timer simulator (releases …))`: the tasks it releases, each by its identity.
    Simulator(Vec<String>),
    /// `(timer none)`: no task is released by the timer.
    #[default]
    None,
    /// `(timer (device NODE) COUNTER (horizon DUR) (rounding FIGURE) (first-compare transition) (releases …))`.
    Device {
        /// The device, by its text.
        device: String,
        /// Its counter.
        counter: Counter,
        /// Its `unambiguous-horizon`.
        horizon: Rational,
        /// `compare-rounding`.
        rounding: Figure,
        /// The tasks it releases, each by its identity.
        releases: Vec<String>,
    },
}

/// What an alarm's line says of its device's counter.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Reading {
    /// On a target, `COUNTER`.
    Counter(Counter),
    /// On the hosted playground, `SIMULATED`.
    Simulated(Simulated),
}

/// An `(alarm …)` line (§3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Alarm {
    /// `NODE`, by its text.
    pub node: String,
    /// The device it programs, by its text.
    pub device: String,
    /// Its device's counter.
    pub reading: Reading,
    /// `KIND`: `level-compare` or `delay`.
    pub kind: String,
    /// `BASE`: `counter` or `epoch`.
    pub time_base: String,
    /// `(horizon DUR)`, nothing where it is `none`.
    pub horizon: Option<Rational>,
    /// `(delivery-bound DUR)`, nothing where it is `none`.
    pub delivery_bound: Option<Rational>,
    /// `(asked-reach DUR)`, nothing where it is `none`.
    pub asked_reach: Option<Rational>,
}

/// A `(clock …)` line, a hosted plan's (§3).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Clock {
    /// The device a hosted time base reads, by its text.
    pub provider: String,
    /// Its counter, as the description states it.
    pub simulated: Simulated,
}

/// An interrupt, as `/2` writes one (§3): the release timer, an alarm by its node, or a source.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Item {
    /// The release timer.
    Timer,
    /// `(alarm NODE)`, by `NODE`'s text.
    Alarm(String),
    /// `(source NAME)`.
    Source(String),
}

/// A route's controller (§6): its line and priority, or owed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Controller {
    /// `(controller owed)`.
    Owed,
    /// `(controller LINE PRIORITY)`.
    Line {
        /// `LINE`.
        line: u64,
        /// `PRIORITY`.
        priority: u64,
    },
}

/// A `route` line: `/1`'s, its item `/2`'s, with `(controller …)` last where it reaches one (§3, §6).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Route {
    /// Its rank among the routes.
    pub rank: u64,
    /// Its interrupt.
    pub item: Item,
    /// Its arrivals' separation.
    pub separation: Separation,
    /// The level its interrupt is delivered at.
    pub level: String,
    /// Who relies on it, each by its text.
    pub requesters: Vec<String>,
    /// Each cost on it: the instance's text and the cost's name in the kind.
    pub costs: Vec<(String, String)>,
    /// Its controller, where it reaches one.
    pub controller: Option<Controller>,
}

/// A context a node or a task runs in (§3): `/1`'s, an interrupt's item `/2`'s, the release timer's routine among them
/// on a target, where the release requester's device takes it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Context {
    /// A job.
    Job,
    /// An interrupt's routine.
    Interrupt(Item),
    /// Initialisation.
    Initialisation,
}

/// A `(region …)` line (§3, §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Region {
    /// Its name.
    pub name: String,
    /// Its provider, by its text.
    pub provider: String,
    /// Its base.
    pub base: u64,
    /// Its size, nothing where it is `unsized`.
    pub size: Option<u64>,
    /// `USE`: `placement`, `executable`, `reserved` or `device`.
    pub usage: String,
    /// The requesters bound to its provider, each by its text.
    pub requesters: Vec<String>,
}

/// A laid allocation's figures (§3, §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Laid {
    /// Its base.
    pub base: u64,
    /// Its guard.
    pub guard: u64,
    /// Its bytes.
    pub bytes: u64,
    /// Its alignment.
    pub align: u64,
    /// Where each figure came from, each `ORIGIN` by its text.
    pub origin: Vec<String>,
}

/// An `(allocation …)` line (§3, §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Allocation {
    /// `UNIT`, by its text.
    pub unit: String,
    /// Its figures where it is laid, nothing where it is owed.
    pub laid: Option<Laid>,
    /// What an owed unit waits on, each `NEED` by its text; nothing for a laid one.
    pub needs: Vec<String>,
}

/// The `(image …)` line (§3, §5).
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Image {
    /// `F`.
    pub floor: u64,
    /// `L`, nothing while it is owed.
    pub limit: Option<u64>,
    /// Each hole `truncated-by` names, its provider and its name.
    pub truncated_by: Vec<(String, String)>,
}

/// What a plan of `/2` holds, read from its text (§3).
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct Contents {
    /// The profile it was lowered under.
    pub profile: String,
    /// Its system.
    pub system: String,
    /// The machine it is of.
    pub target: Target,
    /// The digest of the elaborated program.
    pub program: String,
    /// The digest of the assignment record.
    pub assignment: String,
    /// The adapter catalog's version.
    pub adapters: String,
    /// `(emitter VERSION)`: what generates the code.
    pub emitter: String,
    /// `(catalog "HEX" …)`'s digest.
    pub catalog_digest: String,
    /// `(catalog … NAME)`'s name, nothing where it is `none`, a hosted plan's.
    pub catalog_target: Option<String>,
    /// `(application "HEX")`, where an application's file is given.
    pub application: Option<String>,
    /// `(realization "HEX")`, where a realization's file is given.
    pub realization: Option<String>,
    /// `(code …)`, by its text.
    pub code: Option<String>,
    /// Each `(record ID "HEX")`.
    pub records: Vec<(String, String)>,
    /// Its tasks, in runtime order, each as `/1` writes one.
    pub tasks: Vec<Task>,
    /// Each `(task-inputs …)`: the task's identity and the line's text.
    pub task_inputs: Vec<(String, String)>,
    /// `(declared …)`, by its text.
    pub declared: Option<String>,
    /// `(initialisation-inputs …)`, by its text.
    pub initialisation_inputs: Option<String>,
    /// Each `(realization-figure …)`, by its text.
    pub realization_figures: Vec<String>,
    /// Each `(realization-fact …)`, by its text.
    pub realization_facts: Vec<String>,
    /// The record's bindings.
    pub binds: Vec<Bind>,
    /// Its adapters' instances.
    pub instances: Vec<Instance>,
    /// The epoch's line.
    pub epoch: Epoch,
    /// The release timer's line.
    pub timer: Timer,
    /// Its alarms.
    pub alarms: Vec<Alarm>,
    /// Its clocks, a hosted plan's.
    pub clocks: Vec<Clock>,
    /// Its routes.
    pub routes: Vec<Route>,
    /// The interrupts it enables.
    pub enabled: Vec<Item>,
    /// Its regions.
    pub regions: Vec<Region>,
    /// `(placement PROVIDER REGION)`, a target's.
    pub placement: Option<(String, String)>,
    /// Its allocations, a target's.
    pub allocations: Vec<Allocation>,
    /// The image, a target's.
    pub image: Option<Image>,
    /// Its nodes, each by its text, and each's kind.
    pub nodes: Vec<(String, NodeKind)>,
    /// What runs in which contexts, by the subject's text.
    pub contexts: Vec<(String, Vec<Context>)>,
    /// The initialisation graph's edges.
    pub depends: Vec<Depends>,
    /// The initialisation's steps, each the nodes of one component, by their text.
    pub steps: Vec<Vec<String>>,
    /// What the plan's realization must discharge, a placement obligation among them.
    pub obligations: Vec<ObligationLine>,
    /// What each owner owes.
    pub owed: Vec<Owed>,
    /// Whether nothing is owed.
    pub complete: bool,
}

/// The forms written at most once; the others are written once for each thing they say (§3).
const ONCE: [&str; 21] = [
    "format",
    "profile",
    "system",
    "target",
    "program",
    "assignment",
    "adapters",
    "emitter",
    "catalog",
    "application",
    "realization",
    "code",
    "declared",
    "initialisation-inputs",
    "epoch",
    "timer",
    "enabled",
    "interrupt-model",
    "placement",
    "image",
    "complete",
];

/// A task's inputs, each clause in `decision_application-inputs.md`'s order, as §3's `(task-inputs …)` writes them: the
/// bounds and lists, then the yes-or-no facts, then `deadlines-within-reach`.
const TASK_INPUTS: [&str; 12] = [
    "entry-point",
    "execution-bound",
    "os-calls",
    "node-calls",
    "masked-runs",
    "stack-bound",
    "suspends",
    "locks-scheduler",
    "shares-outside-sections",
    "masks-only-through-api",
    "figures-under-preemption",
    "deadlines-within-reach",
];

/// The forms every plan writes (§3).
const ALWAYS: [&str; 14] = [
    "format",
    "profile",
    "system",
    "target",
    "program",
    "assignment",
    "adapters",
    "emitter",
    "catalog",
    "epoch",
    "timer",
    "enabled",
    "interrupt-model",
    "complete",
];

/// A digest, `"HEX"`: 64 lowercase hexadecimal digits, as `(program …)` writes one.
fn hex(tree: &Tree) -> Result<String, String> {
    hex_under(
        &Tree::List(vec![Tree::Atom("hex".to_owned()), tree.clone()]),
        "hex",
    )
}

/// `(HEAD VALUE)`'s one atom.
fn atom_under(tree: &Tree, head: &str) -> Result<String, String> {
    match tree.under(head)? {
        [value] => Ok(value.atom()?.to_owned()),
        _ => Err(format!("`{}` holds no one name", tree.text())),
    }
}

/// A count, an `INT` or `(pow2 P)` past 2^64 − 1, as §3 writes one (`crate::alarm::count`).
fn count(tree: &Tree) -> Result<Count, String> {
    match tree {
        Tree::Atom(_) => int(tree).map(Count::Int),
        Tree::List(_) => match tree.under("pow2")? {
            [power] => {
                let power = int(power)?;
                if (64..=126).contains(&power) {
                    u32::try_from(power)
                        .map(Count::Pow2)
                        .map_err(|_| format!("`{}` is no count", tree.text()))
                } else {
                    Err(format!(
                        "`{}` is not written as §3 writes a count",
                        tree.text()
                    ))
                }
            }
            _ => Err(format!("`{}` is no count", tree.text())),
        },
        Tree::Quoted(_) => Err(format!("`{}` is no count", tree.text())),
    }
}

/// A rate in hertz, exact, a whole number or `N/D` in lowest terms (§3, `R`; `crate::alarm::rate`).
fn rate(tree: &Tree) -> Result<Rational, String> {
    let text = tree.atom()?;
    let part = |part: &str| -> Result<i128, String> {
        part.parse()
            .map_err(|_| format!("`{text}` is no rate in hertz"))
    };
    let value = match text.split_once('/') {
        Some((numerator, denominator)) => Rational::new(part(numerator)?, part(denominator)?),
        None => Rational::new(part(text)?, 1),
    }
    .ok_or_else(|| format!("`{text}` has no value"))?;
    if crate::contents::written(value) == text && value.is_positive() {
        Ok(value)
    } else {
        Err(format!("`{text}` is not written as §3 writes a rate"))
    }
}

/// `(HEAD DUR)` or `(HEAD none)`.
fn duration_or_none(tree: &Tree, head: &str) -> Result<Option<Rational>, String> {
    match tree.under(head)? {
        [Tree::Atom(none)] if none == "none" => Ok(None),
        [number, unit] => duration(number, unit).map(Some),
        _ => Err(format!("`{}` holds no duration", tree.text())),
    }
}

/// `FIGURE`'s items: `owed`, or `(value …) (record …) (category …)`.
fn figure(items: &[Tree], form: &Tree) -> Result<Figure, String> {
    match items {
        [Tree::Atom(owed)] if owed == "owed" => Ok(Figure::Owed),
        [value, record, category]
            if value.head() == Some("value")
                && record.head() == Some("record")
                && category.head() == Some("category") =>
        {
            Ok(Figure::Stated(
                items.iter().map(Tree::text).collect::<Vec<_>>().join(" "),
            ))
        }
        _ => Err(format!("no figure: `{}`", form.text())),
    }
}

/// `(HEAD FIGURE)`.
fn figure_under(tree: &Tree, head: &str) -> Result<Figure, String> {
    figure(tree.under(head)?, tree)
}

/// `COUNTER`, the first items of `items`: its rate, modulus, reach, step and window, the window none on a delay. Returns
/// it and how many items it took.
fn counter(items: &[Tree]) -> Result<(Counter, usize), String> {
    let [tick_rate, modulus, reach, step, rest @ ..] = items else {
        return Err("a counter holds its rate, modulus, reach and step".to_owned());
    };
    let window = match rest.first() {
        Some(window) if window.head() == Some("window") => Some(match window.under("window")? {
            [Tree::Atom(word)] if word == "magnitude" => Window::Magnitude,
            [ticks] => Window::Ticks(count(ticks)?),
            _ => return Err(format!("no window: `{}`", window.text())),
        }),
        _ => None,
    };
    let one = |tree: &Tree, head: &str| -> Result<Tree, String> {
        match tree.under(head)? {
            [value] => Ok(value.clone()),
            _ => Err(format!("`{}` holds no one value", tree.text())),
        }
    };
    Ok((
        Counter {
            tick_rate: rate(&one(tick_rate, "tick-rate")?)?,
            modulus: count(&one(modulus, "modulus")?)?,
            reach: duration_under(reach, "reach")?,
            step: count(&one(step, "step")?)?,
            window,
        },
        4 + usize::from(window.is_some()),
    ))
}

/// `SIMULATED`: `(simulated (tick-rate R|unstated) (modulus M|unstated))`.
fn simulated(tree: &Tree) -> Result<Simulated, String> {
    let [tick_rate, modulus] = tree.under("simulated")? else {
        return Err(format!("no simulated counter: `{}`", tree.text()));
    };
    let stated = |tree: &Tree, head: &str| -> Result<Option<Tree>, String> {
        match tree.under(head)? {
            [Tree::Atom(word)] if word == "unstated" => Ok(None),
            [value] => Ok(Some(value.clone())),
            _ => Err(format!("`{}` holds no one value", tree.text())),
        }
    };
    Ok(Simulated {
        tick_rate: stated(tick_rate, "tick-rate")?
            .map(|value| rate(&value))
            .transpose()?,
        modulus: stated(modulus, "modulus")?
            .map(|value| count(&value))
            .transpose()?,
    })
}

/// An interrupt, as `/2` writes one: `timer`, `(alarm NODE)` or `(source NAME)`.
fn item(tree: &Tree) -> Result<Item, String> {
    match tree {
        Tree::Atom(atom) if atom == "timer" => Ok(Item::Timer),
        Tree::List(items) if items.len() == 2 && tree.head() == Some("alarm") => {
            Ok(Item::Alarm(items[1].text()))
        }
        Tree::List(items) if items.len() == 2 && tree.head() == Some("source") => {
            Ok(Item::Source(items[1].atom()?.to_owned()))
        }
        _ => Err(format!("no interrupt: `{}`", tree.text())),
    }
}

fn route(items: &[Tree]) -> Result<Route, String> {
    let (route, controller) = match items {
        [rank, interrupt, separation, level, deliverable, requesters, costs] => (
            [
                rank,
                interrupt,
                separation,
                level,
                deliverable,
                requesters,
                costs,
            ],
            None,
        ),
        [rank, interrupt, separation, level, deliverable, requesters, costs, controller] => (
            [
                rank,
                interrupt,
                separation,
                level,
                deliverable,
                requesters,
                costs,
            ],
            Some(controller),
        ),
        _ => return Err("a route line holds seven forms and its controller".to_owned()),
    };
    let [rank, interrupt, separation, level, deliverable, requesters, costs] = route;
    let item = item(interrupt)?;
    let separation = match separation.under("separation")? {
        [Tree::Atom(word)] if word == "unknown" => Separation::Unknown,
        [Tree::Atom(word)] if word == "absent" => Separation::Absent,
        [Tree::Atom(word)] if word == "programmed" => Separation::Programmed,
        [number, unit] => Separation::Of(duration(number, unit)?),
        _ => return Err(format!("no separation: `{}`", separation.text())),
    };
    match (&item, separation) {
        (Item::Timer, _) => return Err("the release timer is no route".to_owned()),
        (Item::Source(_), Separation::Programmed) => {
            return Err("a source's separation is never programmed".to_owned())
        }
        (Item::Alarm(_), separation) if separation != Separation::Programmed => {
            return Err("an alarm's separation is programmed".to_owned())
        }
        _ => {}
    }
    if one_under(deliverable, "deliverable")? != "yes" {
        return Err(format!("no route: `{}`", deliverable.text()));
    }
    let costs = costs
        .under("costs")?
        .iter()
        .map(|cost| match cost {
            Tree::List(pair) => match pair.as_slice() {
                [instance, name] => Ok((instance.text(), name.atom()?.to_owned())),
                _ => Err(format!("no cost: `{}`", cost.text())),
            },
            _ => Err(format!("no cost: `{}`", cost.text())),
        })
        .collect::<Result<_, String>>()?;
    let controller = controller
        .map(|controller| match controller.under("controller")? {
            [Tree::Atom(word)] if word == "owed" => Ok(Controller::Owed),
            [line, priority] => Ok(Controller::Line {
                line: int(line)?,
                priority: int(priority)?,
            }),
            _ => Err(format!("no controller: `{}`", controller.text())),
        })
        .transpose()?;
    Ok(Route {
        rank: int(rank)?,
        item,
        separation,
        level: one_under(level, "level")?,
        requesters: texts_under(requesters, "requesters")?,
        costs,
        controller,
    })
}

fn context(tree: &Tree) -> Result<Context, String> {
    match tree {
        Tree::Atom(word) if word == "job" => Ok(Context::Job),
        Tree::Atom(word) if word == "initialisation" => Ok(Context::Initialisation),
        _ => match tree.under("interrupt")? {
            [interrupt] => Ok(Context::Interrupt(item(interrupt)?)),
            _ => Err(format!("no context: `{}`", tree.text())),
        },
    }
}

fn alarm(items: &[Tree], form: &Tree) -> Result<Alarm, String> {
    let [node, device, rest @ ..] = items else {
        return Err(format!("no alarm: `{}`", form.text()));
    };
    let (reading, taken) = match rest.first() {
        Some(first) if first.head() == Some("simulated") => {
            (Reading::Simulated(simulated(first)?), 1)
        }
        _ => {
            let (counter, taken) = counter(rest)?;
            (Reading::Counter(counter), taken)
        }
    };
    let [kind, time_base, horizon, delivery_bound, asked_reach] = &rest[taken..] else {
        return Err(format!("no alarm: `{}`", form.text()));
    };
    let kind = atom_under(kind, "kind")?;
    if !matches!(kind.as_str(), "level-compare" | "delay") {
        return Err(format!("no alarm's kind: `{kind}`"));
    }
    let time_base = atom_under(time_base, "time-base")?;
    if !matches!(time_base.as_str(), "counter" | "epoch") {
        return Err(format!("no time base: `{time_base}`"));
    }
    Ok(Alarm {
        node: node.text(),
        device: atom_under(device, "device")?,
        reading,
        kind,
        time_base,
        horizon: duration_or_none(horizon, "horizon")?,
        delivery_bound: duration_or_none(delivery_bound, "delivery-bound")?,
        asked_reach: duration_or_none(asked_reach, "asked-reach")?,
    })
}

fn timer(items: &[Tree], form: &Tree) -> Result<Timer, String> {
    match items {
        [Tree::Atom(simulator), releases] if simulator == "simulator" => {
            Ok(Timer::Simulator(texts_under(releases, "releases")?))
        }
        [Tree::Atom(none)] if none == "none" => Ok(Timer::None),
        [device, rest @ ..] if device.head() == Some("device") => {
            let (counter, taken) = counter(rest)?;
            let [horizon, rounding, first, releases] = &rest[taken..] else {
                return Err(format!("no timer: `{}`", form.text()));
            };
            if first.text() != "(first-compare transition)" {
                return Err(format!("no first compare: `{}`", first.text()));
            }
            Ok(Timer::Device {
                device: atom_under(device, "device")?,
                counter,
                horizon: duration_under(horizon, "horizon")?,
                rounding: figure_under(rounding, "rounding")?,
                releases: texts_under(releases, "releases")?,
            })
        }
        _ => Err(format!("no timer: `{}`", form.text())),
    }
}

fn region(items: &[Tree], form: &Tree) -> Result<Region, String> {
    let [name, provider, base, size, usage, requesters] = items else {
        return Err(format!("no region: `{}`", form.text()));
    };
    let size = match size.under("size")? {
        [Tree::Atom(word)] if word == "unsized" => None,
        [value] => Some(int(value)?),
        _ => return Err(format!("no size: `{}`", size.text())),
    };
    let usage = atom_under(usage, "use")?;
    if !matches!(
        usage.as_str(),
        "placement" | "executable" | "reserved" | "device"
    ) {
        return Err(format!("no region's use: `{usage}`"));
    }
    Ok(Region {
        name: name.atom()?.to_owned(),
        provider: provider.atom()?.to_owned(),
        base: match base.under("base")? {
            [value] => int(value)?,
            _ => return Err(format!("no base: `{}`", base.text())),
        },
        size,
        usage,
        requesters: texts_under(requesters, "requesters")?,
    })
}

fn allocation(items: &[Tree], form: &Tree) -> Result<Allocation, String> {
    let figure = |tree: &Tree, head: &str| -> Result<u64, String> {
        match tree.under(head)? {
            [value] => int(value),
            _ => Err(format!("`{}` holds no one integer", tree.text())),
        }
    };
    match items {
        [unit, Tree::Atom(owed), needs] if owed == "owed" => {
            let needs = texts_under(needs, "needs")?;
            if needs.is_empty() {
                return Err("an owed allocation names what it waits on (§5)".to_owned());
            }
            Ok(Allocation {
                unit: unit.text(),
                laid: None,
                needs,
            })
        }
        [unit, base, guard, bytes, align, origin] => Ok(Allocation {
            unit: unit.text(),
            laid: Some(Laid {
                base: figure(base, "base")?,
                guard: figure(guard, "guard")?,
                bytes: figure(bytes, "bytes")?,
                align: figure(align, "align")?,
                origin: texts_under(origin, "origin")?,
            }),
            needs: Vec::new(),
        }),
        _ => Err(format!("no allocation: `{}`", form.text())),
    }
}

fn image(items: &[Tree], form: &Tree) -> Result<Image, String> {
    let [floor, limit, truncated_by] = items else {
        return Err(format!("no image: `{}`", form.text()));
    };
    Ok(Image {
        floor: match floor.under("floor")? {
            [value] => int(value)?,
            _ => return Err(format!("no floor: `{}`", floor.text())),
        },
        limit: match limit.under("limit")? {
            [Tree::Atom(word)] if word == "owed" => None,
            [value] => Some(int(value)?),
            _ => return Err(format!("no limit: `{}`", limit.text())),
        },
        truncated_by: truncated_by
            .under("truncated-by")?
            .iter()
            .map(|hole| match hole {
                Tree::List(pair) => match pair.as_slice() {
                    [provider, name] => Ok((provider.atom()?.to_owned(), name.atom()?.to_owned())),
                    _ => Err(format!("no hole: `{}`", hole.text())),
                },
                _ => Err(format!("no hole: `{}`", hole.text())),
            })
            .collect::<Result<_, String>>()?,
    })
}

/// One line taken into what has been read.
#[allow(clippy::too_many_lines)]
fn take(head: &str, items: &[Tree], form: &Tree, out: &mut Contents) -> Result<(), String> {
    match head {
        "format" => match items {
            [Tree::Quoted(format)] if format == crate::FORMAT_2 => {}
            _ => return Err(format!("not `{}`: `{}`", crate::FORMAT_2, form.text())),
        },
        "profile" => out.profile = one_under(form, head)?,
        "system" => out.system = one_under(form, head)?,
        "target" => {
            out.target = match items {
                [Tree::Atom(word)] if word == "hosted-playground" => Target::HostedPlayground,
                [platform, digest] => Target::Machine {
                    platform: platform.atom()?.to_owned(),
                    digest: hex(digest)?,
                },
                _ => return Err(format!("no target: `{}`", form.text())),
            };
        }
        "program" => out.program = hex_under(form, head)?,
        "assignment" => out.assignment = hex_under(form, head)?,
        "adapters" => out.adapters = one_under(form, head)?,
        "emitter" => out.emitter = atom_under(form, head)?,
        "catalog" => match items {
            [digest, name] => {
                out.catalog_digest = hex(digest)?;
                let name = name.atom()?;
                out.catalog_target = (name != "none").then(|| name.to_owned());
            }
            _ => return Err(format!("no catalog: `{}`", form.text())),
        },
        "application" => out.application = Some(hex_under(form, head)?),
        "realization" => out.realization = Some(hex_under(form, head)?),
        "code" => match items {
            [name, Tree::Quoted(_), Tree::Quoted(_)] => {
                name.atom()?;
                out.code = Some(form.text());
            }
            _ => return Err(format!("no code: `{}`", form.text())),
        },
        "record" => match items {
            [id, digest] => out.records.push((id.atom()?.to_owned(), hex(digest)?)),
            _ => return Err(format!("no record: `{}`", form.text())),
        },
        "task" => out.tasks.push(task(items)?),
        "task-inputs" => match items {
            [identity, clauses @ ..] if identity.head() == Some("task") => {
                identity.under("task")?;
                let heads: Vec<Option<&str>> = clauses.iter().map(Tree::head).collect();
                if heads != TASK_INPUTS.map(Some) {
                    return Err(format!(
                        "a task's inputs hold {} in that order: `{}`",
                        TASK_INPUTS.join(", "),
                        form.text()
                    ));
                }
                out.task_inputs.push((identity.text(), form.text()));
            }
            _ => return Err(format!("no task's inputs: `{}`", form.text())),
        },
        "declared" => match items {
            [Tree::Atom(fact), Tree::Atom(said), clause, basis]
                if fact == "leaves-interrupt-hardware-alone"
                    && matches!(said.as_str(), "yes" | "no")
                    && clause.head() == Some("clause")
                    && basis.head() == Some("basis") =>
            {
                out.declared = Some(form.text());
            }
            _ => return Err(format!("no declaration: `{}`", form.text())),
        },
        "initialisation-inputs" => match items {
            [Tree::Atom(none)] if none == "none" => out.initialisation_inputs = Some(form.text()),
            [entry, ..] if entry.head() == Some("entry-point") => {
                out.initialisation_inputs = Some(form.text());
            }
            _ => return Err(format!("no initialisation's inputs: `{}`", form.text())),
        },
        "realization-figure" => out.realization_figures.push(form.text()),
        "realization-fact" => out.realization_facts.push(form.text()),
        "bind" => match items {
            [requester, fact, provider] => out.binds.push(Bind {
                requester: requester.text(),
                fact: fact.atom()?.to_owned(),
                provider: provider.text(),
            }),
            _ => return Err(format!("no binding: `{}`", form.text())),
        },
        "instance" => out.instances.push(instance(items)?),
        "epoch" => {
            out.epoch = match items {
                [Tree::Atom(word)] if word == "simulator" => Epoch::Simulator,
                [Tree::Atom(word), lead] if word == "after-read" => {
                    Epoch::AfterRead(figure_under(lead, "lead")?)
                }
                [Tree::Atom(word)] if word == "none" => Epoch::None(None),
                [Tree::Atom(word), lead] if word == "none" => {
                    Epoch::None(Some(figure_under(lead, "lead")?))
                }
                _ => return Err(format!("no epoch: `{}`", form.text())),
            };
        }
        "timer" => out.timer = timer(items, form)?,
        "alarm" => out.alarms.push(alarm(items, form)?),
        "clock" => match items {
            [provider, counter] => out.clocks.push(Clock {
                provider: provider.atom()?.to_owned(),
                simulated: simulated(counter)?,
            }),
            _ => return Err(format!("no clock: `{}`", form.text())),
        },
        "route" => out.routes.push(route(items)?),
        "enabled" => {
            out.enabled = items.iter().map(item).collect::<Result<_, _>>()?;
        }
        "interrupt-model" => {
            if form.text() != "(interrupt-model (nesting no) (masking all))" {
                return Err(format!("no interrupt model: `{}`", form.text()));
            }
        }
        "region" => out.regions.push(region(items, form)?),
        "placement" => match items {
            [provider, region] => {
                out.placement = Some((provider.atom()?.to_owned(), region.atom()?.to_owned()));
            }
            _ => return Err(format!("no placement: `{}`", form.text())),
        },
        "allocation" => out.allocations.push(allocation(items, form)?),
        "image" => out.image = Some(image(items, form)?),
        "node" => match items {
            [node, Tree::Atom(kind)] => {
                let kind = match kind.as_str() {
                    "block" => NodeKind::Block,
                    "platform" => NodeKind::Platform,
                    "instance" => NodeKind::Instance,
                    "service" => NodeKind::Service,
                    "policy" => NodeKind::Policy,
                    _ => return Err(format!("no node's kind: `{kind}`")),
                };
                out.nodes.push((node.text(), kind));
            }
            _ => return Err(format!("no node: `{}`", form.text())),
        },
        "context" => match items {
            [subject, contexts @ ..] if !contexts.is_empty() => out.contexts.push((
                subject.text(),
                contexts.iter().map(context).collect::<Result<_, _>>()?,
            )),
            _ => return Err(format!("no context: `{}`", form.text())),
        },
        "depends" => match items {
            [from, to, because] => out.depends.push(Depends {
                from: from.text(),
                to: to.text(),
                because: texts_under(because, "because")?,
            }),
            _ => return Err(format!("no edge: `{}`", form.text())),
        },
        "step" => match items {
            [number, nodes @ ..] => {
                let number = int(number)?;
                if number != out.steps.len() as u64 + 1 {
                    return Err(format!("step {number} out of order"));
                }
                if nodes.is_empty() {
                    return Err(format!("step {number} initialises no node"));
                }
                out.steps.push(nodes.iter().map(Tree::text).collect());
            }
            [] => return Err("a step line is numbered".to_owned()),
        },
        "obligation" => match items {
            [kind, subject, discharged] => out.obligations.push(ObligationLine {
                kind: kind.atom()?.to_owned(),
                subject: subject.text(),
                discharged_by: texts_under(discharged, "discharged-by")?,
            }),
            _ => return Err(format!("no obligation: `{}`", form.text())),
        },
        "owed" => match items {
            [subject, fact, by] => out.owed.push(Owed {
                subject: subject.text(),
                fact: fact.atom()?.to_owned(),
                by: one_under(by, "by")?,
            }),
            _ => return Err(format!("no owed fact: `{}`", form.text())),
        },
        "complete" => match items {
            [Tree::Atom(word)] if word == "yes" => out.complete = true,
            [Tree::Atom(word)] if word == "no" => out.complete = false,
            _ => return Err(format!("not `yes` or `no`: `{}`", form.text())),
        },
        _ => return Err(format!("no form of the plan: `{head}`")),
    }
    Ok(())
}

/// The bindings' order, the assignment record's (`eadl_resolve::plan`'s `class`): a name, a task, an instance, the
/// release requester, each by its text, then the fact.
fn bind_key(bind: &Bind) -> (u8, &str, &str) {
    let class = if bind.requester.starts_with("(task ") {
        1
    } else if bind.requester.starts_with("(adapter ") {
        2
    } else if bind.requester.starts_with("(release ") {
        3
    } else {
        0
    };
    (class, bind.requester.as_str(), bind.fact.as_str())
}

/// Whether the line just taken stands in its section's order (§3, "Orders"), `previous` the line before it of the same
/// head, if any: so two lines of one section swapped are refused too, and a plan has one order as it has one spelling.
fn in_order(head: &str, line: &str, previous: Option<&str>, out: &Contents) -> Result<(), String> {
    let after = |earlier: bool| {
        if earlier {
            Ok(())
        } else {
            Err(format!("`{head}` lines stand out of §3's order"))
        }
    };
    let last_two = |len: usize| len >= 2;
    match head {
        // By their text (§3, "Orders"; `/1` §3's).
        "record" | "instance" | "node" | "context" | "obligation" | "owed"
        | "realization-figure" | "realization-fact" => {
            after(previous.is_none_or(|previous| previous < line))
        }
        "bind" if last_two(out.binds.len()) => {
            let n = out.binds.len();
            after(bind_key(&out.binds[n - 2]) < bind_key(&out.binds[n - 1]))
        }
        // By rank, then name, each its runtime index: how many tasks rank below it (`/1` §2.2).
        "task" => {
            let n = out.tasks.len();
            let task = &out.tasks[n - 1];
            let below = out.tasks.iter().filter(|t| t.rank < task.rank).count() as u64;
            if task.index != below {
                return Err(format!(
                    "task {} is not its runtime index {below}",
                    task.index
                ));
            }
            after(
                n < 2 || {
                    let earlier = &out.tasks[n - 2];
                    (earlier.rank, &earlier.name) < (task.rank, &task.name)
                },
            )
        }
        // By runtime index: the task lines' order.
        "task-inputs" => {
            let n = out.task_inputs.len();
            match out.tasks.get(n - 1) {
                Some(task) if task.identity == out.task_inputs[n - 1].0 => Ok(()),
                _ => Err("a task's inputs stand in the task lines' order".to_owned()),
            }
        }
        "route" => {
            let n = out.routes.len() as u64;
            if out.routes[out.routes.len() - 1].rank == n {
                Ok(())
            } else {
                Err(format!(
                    "route {} is not rank {n}",
                    out.routes[out.routes.len() - 1].rank
                ))
            }
        }
        "alarm" if last_two(out.alarms.len()) => {
            let n = out.alarms.len();
            after(out.alarms[n - 2].node < out.alarms[n - 1].node)
        }
        "clock" if last_two(out.clocks.len()) => {
            let n = out.clocks.len();
            after(out.clocks[n - 2].provider < out.clocks[n - 1].provider)
        }
        "region" if last_two(out.regions.len()) => {
            let n = out.regions.len();
            let key = |r: &Region| (r.base, r.name.clone(), r.provider.clone());
            after(key(&out.regions[n - 2]) < key(&out.regions[n - 1]))
        }
        "depends" if last_two(out.depends.len()) => {
            let n = out.depends.len();
            let key = |d: &Depends| (d.from.clone(), d.to.clone());
            after(key(&out.depends[n - 2]) < key(&out.depends[n - 1]))
        }
        _ => Ok(()),
    }
}

/// What a plan's lines say together that no one line says (§1, §3): a hosted plan has the simulator's epoch and timer,
/// no catalog target, no placement, allocation or image; a plan on a target has none of the simulator's, its catalog
/// target, its placement and its image; an application's lines come together; and a plan holds a task.
fn whole(out: &Contents) -> Result<(), String> {
    match &out.target {
        Target::HostedPlayground => {
            if out.catalog_target.is_some() {
                return Err("a hosted plan's catalog names no target".to_owned());
            }
            if out.epoch != Epoch::Simulator || !matches!(out.timer, Timer::Simulator(_)) {
                return Err("a hosted plan's epoch and timer are the simulator's".to_owned());
            }
            if out.placement.is_some() || !out.allocations.is_empty() || out.image.is_some() {
                return Err("a hosted plan lays out no memory (§1)".to_owned());
            }
            if out
                .alarms
                .iter()
                .any(|alarm| !matches!(alarm.reading, Reading::Simulated(_)))
            {
                return Err("a hosted plan's alarm is simulated".to_owned());
            }
        }
        Target::Machine { .. } => {
            if out.catalog_target.is_none() {
                return Err("a plan on a target names its catalog target".to_owned());
            }
            if out.epoch == Epoch::Simulator || matches!(out.timer, Timer::Simulator(_)) {
                return Err("a plan on a target has no simulator".to_owned());
            }
            if out.placement.is_none() || out.image.is_none() {
                return Err("a plan on a target writes its placement and its image (§5)".to_owned());
            }
            if !out.clocks.is_empty()
                || out
                    .alarms
                    .iter()
                    .any(|alarm| !matches!(alarm.reading, Reading::Counter(_)))
            {
                return Err(
                    "a plan on a target reads each counter as its device states it".to_owned(),
                );
            }
        }
    }
    let application = [
        out.application.is_some(),
        out.code.is_some(),
        out.declared.is_some(),
        out.initialisation_inputs.is_some(),
    ];
    if application.iter().any(|given| *given) && !application.iter().all(|given| *given) {
        return Err("an application's lines come together: `application`, `code`, `declared` and `initialisation-inputs`".to_owned());
    }
    if out.application.is_none() && !out.task_inputs.is_empty() {
        return Err("a task's inputs stand with an application's file".to_owned());
    }
    if out.application.is_some() && out.task_inputs.len() != out.tasks.len() {
        return Err("an application's file gives each task its inputs".to_owned());
    }
    if out.tasks.is_empty() {
        return Err("a plan holds at least one task (§1)".to_owned());
    }
    // A route's controller: on a target each route writes one, every route reaching the controller (§6); a hosted route
    // writes none (§1, RP30-2).
    let machine = matches!(out.target, Target::Machine { .. });
    if out
        .routes
        .iter()
        .any(|route| route.controller.is_some() != machine)
    {
        return Err(
            "a route writes `(controller …)` on a target and none on the hosted playground (§6)"
                .to_owned(),
        );
    }
    // The epoch's line as §4 writes it: after the read under a timer, and with a lead with an alarm and no timer.
    if machine {
        let fits = match (&out.timer, &out.epoch) {
            (Timer::Device { .. }, Epoch::AfterRead(_)) => true,
            (Timer::None, Epoch::None(lead)) => lead.is_some() != out.alarms.is_empty(),
            _ => false,
        };
        if !fits {
            return Err("the epoch's line is after the read under a release timer, and carries a lead with an alarm (§4)".to_owned());
        }
    }
    // What initialisation runs: each node a step names, and nothing else (`/1` §2.6, §2.7).
    let stepped: std::collections::BTreeSet<&str> =
        out.steps.iter().flatten().map(String::as_str).collect();
    if let Some((subject, _)) = out.contexts.iter().find(|(subject, contexts)| {
        contexts.contains(&Context::Initialisation) != stepped.contains(subject.as_str())
    }) {
        return Err(format!(
            "`{subject}` runs in initialisation where a step names it, and only there"
        ));
    }
    // `(enabled …)`: `timer`, where the plan has a release timer, then each route by rank (§3; RP11-1).
    let enabled: Vec<Item> = (!matches!(out.timer, Timer::None))
        .then_some(Item::Timer)
        .into_iter()
        .chain(out.routes.iter().map(|route| route.item.clone()))
        .collect();
    if out.enabled != enabled {
        return Err(
            "`enabled` names the release timer, where there is one, then each route by rank"
                .to_owned(),
        );
    }
    // The allocations in §5's order: the boot stack, the interrupt stack, each task's by runtime index, then each table.
    if !out.allocations.is_empty() {
        let stacks: Vec<String> = ["(stack boot)".to_owned(), "(stack interrupt)".to_owned()]
            .into_iter()
            .chain(
                out.tasks
                    .iter()
                    .map(|task| format!("(stack {})", task.identity)),
            )
            .collect();
        let units: Vec<&str> = out.allocations.iter().map(|a| a.unit.as_str()).collect();
        let tables = &units[stacks.len().min(units.len())..];
        if units.len() < stacks.len()
            || units[..stacks.len()] != stacks.iter().map(String::as_str).collect::<Vec<_>>()[..]
            || tables.iter().any(|unit| !unit.starts_with("(table "))
            || tables.windows(2).any(|pair| pair[0] >= pair[1])
        {
            return Err(
                "the allocations stand in §5's order: boot, interrupt, each task's, each table"
                    .to_owned(),
            );
        }
    }
    if out.complete != out.owed.is_empty() {
        return Err("`complete` says whether anything is owed".to_owned());
    }
    Ok(())
}

/// What `text` holds, or why it is not the canonical text of a plan of `/2`, naming the line.
///
/// # Errors
///
/// A line that does not read, is not written as §3 writes it, or stands out of §3's order; a form written once missing
/// or written twice; lines that do not stand together as §1 and §3 say; a text with no last line end, or with a blank
/// line.
pub fn read(text: &str) -> Result<Contents, String> {
    let Some(body) = text.strip_suffix('\n') else {
        return Err("a plan's last line ends with a newline".to_owned());
    };
    let order = |head: &str| identity::LINES.iter().position(|(name, _)| *name == head);
    let mut out = Contents::default();
    let mut section = 0;
    let mut seen = vec![false; identity::LINES.len()];
    let mut previous_line: Option<(&'static str, &str)> = None;
    for (number, line) in body.split('\n').enumerate() {
        let at = |why: String| format!("line {}: {why}", number + 1);
        let form = parse(line).map_err(&at)?;
        if form.text() != line {
            return Err(at("not written as §3 writes it".to_owned()));
        }
        let head = form
            .head()
            .ok_or_else(|| at("no form of the plan".to_owned()))?;
        let place = order(head).ok_or_else(|| at(format!("no form of the plan: `{head}`")))?;
        if place < section {
            return Err(at(format!(
                "`{head}` stands after `{}`",
                identity::LINES[section].0
            )));
        }
        if seen[place] && ONCE.contains(&head) {
            return Err(at(format!("`{head}` is written once")));
        }
        section = place;
        seen[place] = true;
        let Tree::List(items) = &form else {
            return Err(at("no form of the plan".to_owned()));
        };
        take(head, &items[1..], &form, &mut out).map_err(&at)?;
        let previous = previous_line
            .filter(|(previous_head, _)| *previous_head == head)
            .map(|(_, previous)| previous);
        in_order(head, line, previous, &out).map_err(&at)?;
        previous_line = Some((identity::LINES[place].0, line));
    }
    for head in ALWAYS {
        if order(head).is_none_or(|place| !seen[place]) {
            return Err(format!("a plan writes `{head}`"));
        }
    }
    whole(&out)?;
    Ok(out)
}
