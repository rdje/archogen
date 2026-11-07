//! The emitter (`ROADMAP.md` §4.2 `archogen-emit`, §10.1 step 6; `docs/decisions/decision_hosted-build.md`): a checked runtime
//! plan turned into the files of a generated system, for the hosted playground. It does no I/O: it returns the files, and
//! the command writes them.
//!
//! ⭐ **Behavior is copied and never generated.** What it specializes is data, the tables of `src/main.rs`. What runs is
//! `crates/rt-core` and `crates/sim-playground`, byte for byte, which the engine compiles and tests; a test holds each copy to
//! its source, so "reviewed" and "shipped" cannot part (`ROADMAP.md` §1.1: "copied or linked components").
//!
//! ⭐ **A plan is accepted only by lowering it again** from the same inputs and comparing bytes (`RTP-H3`), so a plan that
//! reached here by any route is the plan the checked inputs make, or nothing is generated.
//!
//! Each job's body is empty until `M4.12`'s inputs give it code, and the trace's header says so (`RTP-H1`).

#![forbid(unsafe_code)]

use archogen_digest::Digest;
use archogen_plan::contents::{Contents, Item, NodeKind, Release, Route, Separation, Trigger};
use archogen_plan::{lower_checked, Acceptance, Generator, Lowered, Refusal};
use eadl_front::{Form, Program, SourceMap};
use eadl_model::check::resolved_declarations;
use eadl_model::kind::Registry;

/// The format of `provenance.json`.
pub const PROVENANCE_FORMAT: &str = "archogen-build-provenance/5";

/// The provenance's name in a package.
pub const PROVENANCE_FILE: &str = "provenance.json";

/// The assignment's name in a package.
pub const ASSIGNMENT_FILE: &str = "assignment.eadl";

/// The checker's result record's name in a package.
pub const RESULT_FILE: &str = "plan-check.json";

/// What stands for a provenance's own digest while it is taken: its `self` member, which covers the rest of its text.
const SELF_BLANK: &str = "sha256:0000000000000000000000000000000000000000000000000000000000000000";

/// Whether a provenance's text is what its own digest says: an edit shows, and a forger rewriting the digest too does not.
#[must_use]
pub fn provenance_is_whole(text: &str) -> bool {
    const MEMBER: &str = "\"self\": \"";
    let Some(start) = text.find(MEMBER).map(|at| at + MEMBER.len()) else {
        return false;
    };
    let Some(end) = start
        .checked_add(SELF_BLANK.len())
        .filter(|end| text.is_char_boundary(*end) && *end <= text.len())
    else {
        return false;
    };
    let blanked = format!("{}{SELF_BLANK}{}", &text[..start], &text[end..]);
    text[start..end] == format!("sha256:{}", Digest::of(blanked.as_bytes()).hex())
}

/// The most timer releases and source arrivals a run of the hosted playground takes: a hyperperiod has no bound of its
/// own, and one that fits `u64` can still be a run that never ends (the record's §3).
pub const MOST_RELEASES: u64 = 1 << 20;

/// The engine's sources every generated system holds, by the path each takes there, byte for byte.
///
/// ⛔ **This package's own copies, under `copies/`, held to their sources by `tests/emit.rs`.** The generator never reads
/// a file of `rt-core`, the implementation's role package: a build that did would be a program of two roles to the trust
/// inventory (`trust-shared-program`). Here the runtime is text the generator writes out and never compiles
/// (`docs/decisions/decision_hosted-build.md` §1).
const COPIES: [(&str, &str); 4] = [
    (
        "rt-core/src/lib.rs",
        include_str!("../copies/rt-core/src/lib.rs"),
    ),
    (
        "rt-core/src/scheduler.rs",
        include_str!("../copies/rt-core/src/scheduler.rs"),
    ),
    (
        "rt-core/src/fault.rs",
        include_str!("../copies/rt-core/src/fault.rs"),
    ),
    (
        "sim-playground/src/lib.rs",
        include_str!("../copies/sim-playground/src/lib.rs"),
    ),
];

/// The engine's sources a generated system copies, each by the path it takes there and its bytes.
#[must_use]
pub const fn copies() -> &'static [(&'static str, &'static str)] {
    &COPIES
}

/// What a build is made from: the inputs the plan is lowered from, and the plan.
#[derive(Clone, Copy)]
pub struct Inputs<'a> {
    /// The elaborated program the language's check accepted.
    pub program: &'a Program,
    /// The sources its spans point into.
    pub sources: &'a SourceMap,
    /// The description's name among them, which a provenance entry names each source relative to.
    pub description: &'a str,
    /// The language's own definitions among them, each its name and its text, which every build holds in its generator
    /// and none records. By both: a description of yours may lie at a path a definition is named by.
    pub language: &'a [(String, String)],
    /// The assignment record, as the text written.
    pub assignment: &'a str,
    /// The configuration checker's result record for it, as the text written.
    pub result: &'a str,
    /// The runtime plan's text.
    pub plan: &'a str,
    /// The language definition.
    pub kinds: &'a Registry,
    /// How the checker's acceptance was obtained.
    pub acceptance: Acceptance,
}

/// One generated file.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct File {
    /// Its path, relative to the output directory.
    pub path: String,
    /// Its contents.
    pub contents: String,
}

/// What was generated.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Emitted {
    /// The generated package's name.
    pub package: String,
    /// Every file, in the order they are written.
    pub files: Vec<File>,
    /// The tasks' count, for the command to report.
    pub tasks: usize,
    /// Each task a source releases, by the plan's inference: the task's name and the source's (`RTP-H1`).
    pub source_triggers: Vec<(String, String)>,
    /// One second in the run's ticks.
    pub ticks_per_second: u64,
    /// One past the last instant the run reaches, in ticks.
    pub horizon: u64,
    /// Whether the plan's acceptance was taken as text, so the build is experimental.
    pub experimental: bool,
}

fn refuse(code: &'static str, message: impl Into<String>) -> Refusal {
    Refusal {
        code,
        message: message.into(),
    }
}

/// A task released by `(unknown CAUSE)` refused, naming the task and its cause, with its repair (`RTP-H1`).
fn unknown_trigger(task: &str, cause: &str) -> Refusal {
    let (code, repair) = if cause.starts_with("(own-faster") || cause.starts_with("(own-absent") {
        (
            "unsupported-profile",
            "the analysis does not admit that source for this task: bind one at least as far apart as the task",
        )
    } else if cause.starts_with("(own-") {
        (
            "missing-fact",
            "state a `min-arrival-separation` at least the task's for its own source",
        )
    } else if cause.starts_with("(unstated") {
        (
            "missing-fact",
            "write the task's own `(needs interrupt-source)`, so the search binds it a source at least as far apart, and \
             state that source's `min-arrival-separation`",
        )
    } else {
        (
            "missing-fact",
            "write the task's own `(needs interrupt-source)`, so the search binds it a source at least as far apart",
        )
    };
    refuse(
        code,
        format!("the plan cannot say what releases task `{task}`, `{cause}`: {repair}"),
    )
}

const fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        (a, b) = (b, a % b);
    }
    a
}

/// `a` and `b`'s least common multiple, where it fits.
fn lcm(a: u64, b: u64) -> Option<u64> {
    (a / gcd(a, b).max(1)).checked_mul(b)
}

/// What the run repeats, in ticks: each one's denominator divides the tick (the record's §2).
struct Ticks {
    per_second: u64,
}

impl Ticks {
    fn of(&self, what: &str, seconds: eadl_model::rational::Rational) -> Result<u64, Refusal> {
        let denominator = u64::try_from(seconds.denominator()).ok();
        let numerator = u64::try_from(seconds.numerator()).ok();
        let ticks = match (numerator, denominator) {
            (Some(numerator), Some(denominator)) => {
                numerator.checked_mul(self.per_second / denominator.max(1))
            }
            _ => None,
        };
        ticks.ok_or_else(|| past_ticks(what))
    }
}

fn past_ticks(what: &str) -> Refusal {
    refuse(
        "unsupported-profile",
        format!(
            "{what} does not fit this build's integer ticks, and a build never rounds a duration: its tick is the second over the least common denominator of the repeated durations, each read as a fraction of a second, and the longest must come to under 2^64 of it: choose durations of a plainer fraction of a second, or a longest nearer the shortest"
        ),
    )
}

/// What the route is, for a refusal or a trace.
fn route_name(route: &Route) -> String {
    match &route.item {
        Item::Source(name) => format!("source `{name}`"),
        Item::TimerRoute(name) => format!("timer route `{name}`"),
        Item::Timer => "the release timer".to_owned(),
    }
}

/// Why `route`'s source does not arrive in a run, if it does not: a run never claims what it did not exercise (the
/// record's §3; `RTP-H9`). A route the plan does not enable raises nothing, whatever its separation (`RTP-H1`).
fn silent(contents: &Contents, route: &Route) -> Option<&'static str> {
    if !contents.enabled.contains(&route.item) {
        return Some("is not enabled");
    }
    match route.separation {
        Separation::Of(separation) if separation.is_positive() => None,
        Separation::Of(_) => Some("states a separation of zero"),
        Separation::Unknown | Separation::Absent | Separation::Programmed => {
            Some("states no separation")
        }
    }
}

/// What the run repeats, each by what a refusal would call it: the period of each task the timer releases, and the separation
/// of each source that arrives. A task a source releases repeats only as that source does, so its own minimum separation
/// is no part of the run; nor is a deadline or a jitter, which no table holds (the record's §2 and §3).
fn repeating(contents: &Contents) -> Vec<(String, eadl_model::rational::Rational)> {
    let mut out = Vec::new();
    for task in &contents.tasks {
        if let (Trigger::Timer, Release::Period(period)) = (&task.released_by, task.release) {
            out.push((format!("task `{}`'s period", task.name), period));
        }
    }
    for route in &contents.routes {
        if let (Item::Source(_), None, Separation::Of(separation)) =
            (&route.item, silent(contents, route), route.separation)
        {
            out.push((format!("{}'s separation", route_name(route)), separation));
        }
    }
    out
}

/// The tick: the least common multiple of the denominators of what the run repeats (the record's §2).
fn tick(contents: &Contents) -> Result<Ticks, Refusal> {
    let mut per_second = 1_u64;
    for (what, seconds) in repeating(contents) {
        let denominator = u64::try_from(seconds.denominator()).map_err(|_| past_ticks(&what))?;
        per_second = lcm(per_second, denominator).ok_or_else(|| past_ticks(&what))?;
    }
    Ok(Ticks { per_second })
}

/// One hyperperiod in ticks of what the run repeats, within [`MOST_RELEASES`] (the record's §3).
fn horizon_of(contents: &Contents, ticks: &Ticks) -> Result<u64, Refusal> {
    let mut each = Vec::new();
    for (what, seconds) in repeating(contents) {
        let value = ticks.of(&what, seconds)?;
        each.push((what, value));
    }
    let mut horizon = 1_u64;
    for (what, value) in &each {
        horizon = lcm(horizon, *value).ok_or_else(|| {
            refuse(
                "unsupported-profile",
                format!(
                    "one hyperperiod of this plan, with {what}, does not fit this build's integer ticks: choose periods \
                     with a smaller common multiple"
                ),
            )
        })?;
    }
    let releases = each
        .iter()
        .filter(|(_, value)| *value > 0)
        .fold(0_u64, |total, (_, value)| {
            total.saturating_add(horizon / value)
        });
    if releases > MOST_RELEASES {
        return Err(refuse(
            "unsupported-profile",
            format!(
                "one hyperperiod of this plan is {releases} releases and arrivals over {horizon} ticks, more than the \
                 {MOST_RELEASES} a run of the hosted playground takes: choose periods with a smaller common multiple"
            ),
        ));
    }
    Ok(horizon)
}

/// A Rust string literal.
fn literal(text: &str) -> String {
    format!("{text:?}")
}

/// The most of a system's name that a package's name holds.
const MOST_IN_A_PACKAGE_NAME: usize = 128;

/// A Cargo package name for a system's name: total and deterministic, and a label, never its identity.
fn package_name(system: &str) -> String {
    let mut body = String::with_capacity(system.len());
    for ch in system.chars() {
        if ch.is_ascii_alphanumeric() {
            body.push(ch.to_ascii_lowercase());
        } else if !body.ends_with('-') {
            body.push('-');
        }
    }
    // Cargo makes file names of it, with a hash and more, and a file name holds 255 bytes: the system's own name can
    // pass that, and what tells two systems apart is the plan's digest. Each character here is one byte.
    let cut = &body[..body.len().min(MOST_IN_A_PACKAGE_NAME)];
    format!("system-{}", cut.trim_matches('-'))
}

/// The manifest of the system's own package, the root of its workspace, so it builds wherever it is written.
fn root_manifest(package: &str) -> String {
    format!(
        "# GENERATED by archogen from a runtime plan. Do not edit: an edit makes the output stop corresponding to any plan.\n\
         [package]\n\
         name = \"{package}\"\n\
         version = \"0.0.0\"\n\
         edition = \"2021\"\n\
         publish = false\n\n\
         [workspace]\n\
         members = [\"rt-core\", \"sim-playground\"]\n\n\
         [dependencies]\n\
         rt-core = {{ path = \"rt-core\" }}\n\
         sim-playground = {{ path = \"sim-playground\" }}\n"
    )
}

/// The manifest of a copied crate: the copy's own, where the engine's reads the engine's workspace.
fn copied_manifest(name: &str, dependencies: &str) -> String {
    format!(
        "# GENERATED by archogen: the manifest of a copy of the engine's own crate. Do not edit.\n\
         [package]\n\
         name = \"{name}\"\n\
         version = \"0.0.0\"\n\
         edition = \"2021\"\n\
         publish = false\n\n\
         [dependencies]\n\
         {dependencies}"
    )
}

/// What the trace's header says the run did not exercise, and what it is (the record's §3).
fn header(
    contents: &Contents,
    ticks: &Ticks,
    horizon: u64,
    plan: &Digest,
    experimental: bool,
) -> Vec<String> {
    let mut lines = vec![
        sim_format().to_owned(),
        format!("system {}", contents.system),
        format!("plan sha256:{}", plan.hex()),
        format!(
            "scenario one hyperperiod, {horizon} ticks of 1/{} s; each source at its stated separation, from time zero",
            ticks.per_second
        ),
    ];
    for (name, kind) in &contents.nodes {
        match kind {
            NodeKind::Block => lines.push(format!("device {name} block")),
            NodeKind::Platform => lines.push(format!("device {name} platform")),
            NodeKind::Instance | NodeKind::Service | NodeKind::Policy => {}
        }
    }
    for route in &contents.routes {
        match &route.item {
            Item::Source(name) => {
                if let Some(why) = silent(contents, route) {
                    lines.push(format!("source {name} {why}, so it does not arrive"));
                }
            }
            Item::TimerRoute(name) => {
                lines.push(format!(
                    "timer route {name} is programmed by code no job holds yet, so it does not arrive"
                ));
            }
            Item::Timer => {}
        }
    }
    lines.push(
        "no job runs application code: each body is empty, so only the releases are informative"
            .to_owned(),
    );
    if !contents.complete {
        lines.push(
            "the plan is not complete: what it owes is unmet, and this run establishes no property"
                .to_owned(),
        );
    }
    if experimental {
        lines.push(
            "experimental: the checker's acceptance was taken as text, not checked by this build"
                .to_owned(),
        );
    }
    lines
}

/// The trace format this emitter's generated `main` writes.
const fn sim_format() -> &'static str {
    "archogen-playground-trace/4"
}

/// One line of a generated table, with where it came from.
struct Entry {
    table: String,
    line: usize,
    plan_line: String,
    source: Option<(String, u32, u32)>,
}

/// Every task form of `system` in the program, by its name, with its span's place: what provenance reads.
fn task_places(inputs: &Inputs<'_>, system: &str) -> Vec<(String, String, u32, u32)> {
    let flattened;
    let declarations: &[Form] = if inputs.program.instances.len() == 1 {
        &inputs.program.root().declarations
    } else {
        flattened = resolved_declarations(inputs.program).0;
        &flattened
    };
    declarations
        .iter()
        .filter(|d| d.head() == Some("defsystem"))
        .filter(|d| d.items().get(1).and_then(Form::as_symbol) == Some(system))
        .flat_map(|d| d.items().iter().filter(|i| i.head() == Some("task")))
        .filter_map(|task| {
            let name = task.items().get(1).and_then(Form::as_symbol)?.to_owned();
            let source = inputs.sources.get(task.span().source)?;
            let position = source.position(task.span().start);
            let file = relative_to_description(inputs.description, &source.name);
            Some((name, file, position.line, position.column))
        })
        .collect()
}

/// `name`, a source's path as its caller gave it, relative to the description's directory: nothing a package holds
/// depends on where it was built. A module outside that directory is named by its file alone.
fn relative_to_description(description: &str, name: &str) -> String {
    let directory = description
        .rfind('/')
        .map_or("", |slash| &description[..=slash]);
    let inside = name
        .strip_prefix(directory)
        .filter(|rest| !rest.starts_with("../"));
    inside.map_or_else(
        || name.rsplit('/').next().unwrap_or(name).to_owned(),
        str::to_owned,
    )
}

/// `src/main.rs`, and each table entry's provenance.
#[allow(clippy::too_many_lines)]
fn main_rs(
    inputs: &Inputs<'_>,
    contents: &Contents,
    ticks: &Ticks,
    horizon: u64,
    plan: &Digest,
    experimental: bool,
) -> Result<(String, Vec<Entry>), Refusal> {
    let places = task_places(inputs, &contents.system);
    let sources: Vec<(&str, Option<u64>, bool)> = {
        let mut routes: Vec<_> = contents.routes.iter().collect();
        routes.sort_by_key(|route| route.rank);
        let mut out = Vec::new();
        for route in routes {
            match &route.item {
                Item::Source(name) => {
                    let separation = match route.separation {
                        Separation::Of(separation) if silent(contents, route).is_none() => Some(
                            ticks.of(&format!("{}'s separation", route_name(route)), separation)?,
                        ),
                        _ => None,
                    };
                    out.push((name.as_str(), separation, false));
                }
                Item::TimerRoute(name) => out.push((name.as_str(), None, true)),
                Item::Timer => {}
            }
        }
        out
    };
    // What the plan's `node` lines write a device, and never the description (`RTP-H9`).
    let devices: Vec<(&str, &str)> = contents
        .nodes
        .iter()
        .filter_map(|(name, kind)| match kind {
            NodeKind::Block => Some((name.as_str(), "Block")),
            NodeKind::Platform => Some((name.as_str(), "Platform")),
            NodeKind::Instance | NodeKind::Service | NodeKind::Policy => None,
        })
        .collect();
    let plan_lines: Vec<&str> = inputs.plan.lines().collect();
    let mut lines: Vec<String> = vec![
        "//! GENERATED by archogen from a runtime plan. Do not edit: an edit makes the output stop corresponding to any plan.".to_owned(),
        "//!".to_owned(),
        "//! The tables below are the plan's, and all that is specialized: what runs is `rt-core` and `sim-playground`, copies of".to_owned(),
        "//! the engine's own crates. This system runs no application code, and carries no timing or assurance claim.".to_owned(),
        String::new(),
        "use std::process::ExitCode;".to_owned(),
        String::new(),
        "use rt_core::OverrunPolicy;".to_owned(),
        "use sim_playground::{run, Device, DeviceKind, Ended, Observer, Source, Stamped, System, Task, Trigger};".to_owned(),
        String::new(),
    ];
    let mut entries = Vec::new();
    lines.push(format!(
        "const TASKS: [Task<'static>; {}] = [",
        contents.tasks.len()
    ));
    let timer_released = contents
        .tasks
        .iter()
        .any(|task| matches!(task.released_by, Trigger::Timer));
    if timer_released && !contents.enabled.contains(&Item::Timer) {
        return Err(refuse(
            "tool-failure",
            "the plan releases a task by the timer, and does not enable the timer",
        ));
    }
    for (slot, task) in contents.tasks.iter().enumerate() {
        // The table's slot is the plan's index, so what the scheduler calls a task is what the plan does (`RTP-H1`).
        if task.index != slot as u64 {
            return Err(refuse(
                "tool-failure",
                format!(
                    "task `{}` is the plan's task {}, in the table's slot {slot}",
                    task.name, task.index
                ),
            ));
        }
        let policy = match task.overrun.as_str() {
            "fault" => "OverrunPolicy::Fault",
            "skip-late-job" => "OverrunPolicy::SkipLateJob",
            other => {
                return Err(refuse(
                    "unsupported-profile",
                    format!("task `{}` writes the overrun policy `{other}`, which this build has no realization of", task.name),
                ))
            }
        };
        let trigger = match &task.released_by {
            Trigger::Timer => {
                let Release::Period(period) = task.release else {
                    return Err(refuse(
                        "tool-failure",
                        format!(
                            "task `{}` is released by the timer with no period",
                            task.name
                        ),
                    ));
                };
                format!(
                    "Trigger::Timer({})",
                    ticks.of(&format!("task `{}`'s period", task.name), period)?
                )
            }
            Trigger::Source(name) => {
                let place = sources
                    .iter()
                    .position(|(source, _, alarm)| source == name && !alarm)
                    .ok_or_else(|| {
                        refuse(
                            "tool-failure",
                            format!(
                                "task `{}` is released by `{name}`, which the plan routes nowhere",
                                task.name
                            ),
                        )
                    })?;
                format!("Trigger::Source({place})")
            }
            Trigger::Unknown(cause) => return Err(unknown_trigger(&task.name, cause)),
        };
        let rank = i64::try_from(task.rank).map_err(|_| {
            refuse(
                "unsupported-profile",
                format!("task `{}`'s rank does not fit the runtime", task.name),
            )
        })?;
        lines.push(format!(
            "    Task {{ name: {}, rank: {rank}, policy: {policy}, trigger: {trigger}, body: &[] }},",
            literal(&task.name)
        ));
        let plan_line = plan_lines
            .iter()
            .find(|line| line.starts_with(&format!("(task {} ", task.index)))
            .map_or_else(String::new, |line| (*line).to_owned());
        let source = places
            .iter()
            .find(|(name, ..)| *name == task.name)
            .map(|(_, file, line, column)| (file.clone(), *line, *column));
        entries.push(Entry {
            table: format!("TASKS[{}]", task.index),
            line: lines.len(),
            plan_line,
            source,
        });
    }
    lines.push("];".to_owned());
    lines.push(String::new());
    lines.push(format!(
        "const SOURCES: [Source<'static>; {}] = [",
        sources.len()
    ));
    for (slot, (name, separation, alarm)) in sources.iter().enumerate() {
        lines.push(format!(
            "    Source {{ name: {}, separation: {separation:?}, alarm: {alarm} }},",
            literal(name)
        ));
        let route = if *alarm {
            format!("(timer-route {name})")
        } else {
            format!("(source {name})")
        };
        entries.push(Entry {
            table: format!("SOURCES[{slot}]"),
            line: lines.len(),
            plan_line: plan_lines
                .iter()
                .find(|line| line.starts_with("(route ") && line.contains(&route))
                .map_or_else(String::new, |line| (*line).to_owned()),
            source: None,
        });
    }
    lines.push("];".to_owned());
    lines.push(String::new());
    // What a system with no device would not use, and a warning its builder would read as ours.
    if devices.is_empty() {
        for line in &mut lines {
            if line.starts_with("use sim_playground::") {
                *line = line.replace(" DeviceKind,", "");
            }
        }
    }
    lines.push(format!("const DEVICES: [Device; {}] = [", devices.len()));
    for (slot, (name, kind)) in devices.iter().enumerate() {
        lines.push(format!(
            "    Device {{ name: {}, kind: DeviceKind::{kind} }},",
            literal(name)
        ));
        let opens = format!("(node {name} ");
        entries.push(Entry {
            table: format!("DEVICES[{slot}]"),
            line: lines.len(),
            plan_line: plan_lines
                .iter()
                .find(|line| line.starts_with(&opens))
                .map_or_else(String::new, |line| (*line).to_owned()),
            source: None,
        });
    }
    lines.push("];".to_owned());
    lines.push(String::new());
    lines.push(format!(
        "const INITIALISATION: [&[&str]; {}] = [",
        contents.steps.len()
    ));
    for (slot, step) in contents.steps.iter().enumerate() {
        let nodes: Vec<String> = step.iter().map(|node| literal(node)).collect();
        lines.push(format!("    &[{}],", nodes.join(", ")));
        let opens = format!("(step {} ", slot + 1);
        entries.push(Entry {
            table: format!("INITIALISATION[{slot}]"),
            line: lines.len(),
            plan_line: plan_lines
                .iter()
                .find(|line| line.starts_with(&opens))
                .map_or_else(String::new, |line| (*line).to_owned()),
            source: None,
        });
    }
    lines.push("];".to_owned());
    lines.push(String::new());
    let header = header(contents, ticks, horizon, plan, experimental);
    lines.push(format!("const HEADER: [&str; {}] = [", header.len()));
    for line in &header {
        lines.push(format!("    {},", literal(line)));
    }
    lines.push("];".to_owned());
    lines.push(String::new());
    lines.push("const SYSTEM: System<'static> = System {".to_owned());
    lines.push(format!("    name: {},", literal(&contents.system)));
    lines.push(format!("    ticks_per_second: {},", ticks.per_second));
    lines.push(format!("    horizon: {horizon},"));
    lines.push("    tasks: &TASKS,".to_owned());
    lines.push("    sources: &SOURCES,".to_owned());
    lines.push("    devices: &DEVICES,".to_owned());
    lines.push("    initialisation: &INITIALISATION,".to_owned());
    lines.push("};".to_owned());
    lines.push(String::new());
    lines.extend(
        [
            "struct Print;",
            "",
            "impl Observer for Print {",
            "    fn observe(&mut self, event: Stamped<'_>) {",
            "        println!(\"{event}\");",
            "    }",
            "}",
            "",
            "fn main() -> ExitCode {",
            "    for line in HEADER {",
            "        println!(\"; {line}\");",
            "    }",
        ]
        .map(str::to_owned),
    );
    lines.push(format!(
        "    match run::<{}>(&SYSTEM, &mut Print) {{",
        contents.tasks.len()
    ));
    lines.extend(
        [
            "        Ended::Horizon => ExitCode::SUCCESS,",
            "        Ended::Halted => ExitCode::from(1),",
            "        Ended::NoProgress => {",
            "            eprintln!(\"this run took too many steps with time standing still, and stopped there\");",
            "            ExitCode::from(1)",
            "        }",
            "        Ended::NotBooted => {",
            "            eprintln!(\"the generated tables are not the runtime's: a defect of the engine, so report it\");",
            "            ExitCode::from(70)",
            "        }",
            "    }",
            "}",
        ]
        .map(str::to_owned),
    );
    let mut text = lines.join("\n");
    text.push('\n');
    Ok((text, entries))
}

/// `provenance.json`: what the plan was lowered from, and what each table entry came from (the record's §1; §10.1).
fn provenance(
    lowered: &Lowered,
    contents: &Contents,
    description: &str,
    entries: &[Entry],
    files: &[File],
) -> String {
    let quote = |text: &str| {
        let mut out = String::new();
        archogen_json::write_string(&mut out, text);
        out
    };
    let mut out = String::from("{\n");
    out.push_str(&format!("  \"format\": {},\n", quote(PROVENANCE_FORMAT)));
    out.push_str(&format!("  \"self\": {},\n", quote(SELF_BLANK)));
    out.push_str(&format!("  \"system\": {},\n", quote(&contents.system)));
    out.push_str(&format!("  \"description\": {},\n", quote(description)));
    out.push_str(&format!(
        "  \"plan\": {},\n",
        quote(&format!("sha256:{}", lowered.plan.sha256().hex()))
    ));
    let origin = &lowered.provenance;
    out.push_str(&format!(
        "  \"generator\": {{ \"version\": {}, \"sha256\": {} }},\n",
        quote(&origin.generator.version),
        quote(&format!("sha256:{}", origin.generator.sha256.hex()))
    ));
    out.push_str(&format!(
        "  \"result\": {},\n  \"checker\": {},\n",
        quote(&format!("sha256:{}", origin.result.hex())),
        quote(&format!("sha256:{}", origin.checker.hex()))
    ));
    let (acceptance, experimental) = match origin.acceptance {
        Acceptance::CheckedByFront => ("checked-by-front", false),
        Acceptance::TakenAsText => ("taken-as-text", true),
    };
    out.push_str(&format!(
        "  \"acceptance\": {},\n  \"experimental\": {experimental},\n  \"entries\": [\n",
        quote(acceptance)
    ));
    for (place, entry) in entries.iter().enumerate() {
        let source = entry.source.as_ref().map_or_else(
            || "null".to_owned(),
            |(file, line, column)| {
                format!(
                    "{{ \"file\": {}, \"line\": {line}, \"column\": {column} }}",
                    quote(file)
                )
            },
        );
        out.push_str(&format!(
            "    {{ \"table\": {}, \"generated\": {{ \"file\": \"src/main.rs\", \"line\": {} }}, \"plan\": {}, \"source\": {source} }}{}\n",
            quote(&entry.table),
            entry.line,
            quote(&entry.plan_line),
            if place + 1 == entries.len() { "" } else { "," }
        ));
    }
    // Every other file of the package, its length and its digest: what `archogen build` may replace, and `archogen explain` speak
    // for. The length is what lets a later command say a file has changed without reading however much it has grown to.
    out.push_str("  ],\n  \"files\": [\n");
    for (place, file) in files.iter().enumerate() {
        let name = quote(&file.path);
        let digest = quote(&format!(
            "sha256:{}",
            Digest::of(file.contents.as_bytes()).hex()
        ));
        let bytes = file.contents.len();
        let comma = if place + 1 == files.len() { "" } else { "," };
        out.push_str(&format!(
            "    {{ \"path\": {name}, \"bytes\": {bytes}, \"sha256\": {digest} }}{comma}\n"
        ));
    }
    out.push_str("  ]\n}\n");
    // Taken over all of the above with the member blank, as `provenance_is_whole` takes it.
    let own = format!("sha256:{}", Digest::of(out.as_bytes()).hex());
    out.replacen(SELF_BLANK, &own, 1)
}

/// The files of the system `inputs` describe.
///
/// # Errors
///
/// The lowering's refusals; `tool-failure` where `inputs.plan` is not what its inputs lower to (`RTP-H3`); a task the plan
/// cannot say what releases, as `RTP-H1` gives each cause; a duration past this build's integer ticks.
pub fn emit(inputs: &Inputs<'_>, generator: Generator) -> Result<Emitted, Refusal> {
    let lowered = lower_checked(
        inputs.program,
        inputs.assignment,
        inputs.result,
        inputs.kinds,
        generator,
        inputs.acceptance,
    )?;
    if lowered.plan.text() != inputs.plan {
        return Err(refuse(
            "tool-failure",
            "the runtime plan given is not what its inputs lower to: resolve and lower it again",
        ));
    }
    let contents = lowered.plan.contents();
    // A task nothing can release is said first: what a run would cost means nothing until it can run (`RTP-H1`).
    for task in &contents.tasks {
        if let Trigger::Unknown(cause) = &task.released_by {
            return Err(unknown_trigger(&task.name, cause));
        }
    }
    let ticks = tick(contents)?;
    let horizon = horizon_of(contents, &ticks)?;
    let experimental = inputs.acceptance == Acceptance::TakenAsText;
    let (main, entries) = main_rs(
        inputs,
        contents,
        &ticks,
        horizon,
        &lowered.plan.sha256(),
        experimental,
    )?;
    let package = package_name(&contents.system);
    let mut files = Vec::from([
        File {
            path: "Cargo.toml".to_owned(),
            contents: root_manifest(&package),
        },
        File {
            path: "src/main.rs".to_owned(),
            contents: main,
        },
        File {
            path: "rt-core/Cargo.toml".to_owned(),
            contents: copied_manifest("rt-core", ""),
        },
        File {
            path: "sim-playground/Cargo.toml".to_owned(),
            contents: copied_manifest("sim-playground", "rt-core = { path = \"../rt-core\" }\n"),
        },
    ]);
    files.extend(COPIES.iter().map(|(path, contents)| File {
        path: (*path).to_owned(),
        contents: (*contents).to_owned(),
    }));
    files.push(File {
        path: "plan.txt".to_owned(),
        contents: inputs.plan.to_owned(),
    });
    // What it was made from: the description and every module it imports, under the names they import each other by, once
    // each however many import it, so a later command reads what this build read (`archogen explain`; the record's §1).
    let mut recorded = std::collections::BTreeSet::new();
    for source in inputs.sources.iter() {
        if inputs
            .language
            .iter()
            .any(|(name, text)| *name == source.name && *text == source.text)
        {
            continue;
        }
        let name = relative_to_description(inputs.description, &source.name);
        if recorded.insert(name.clone()) {
            files.push(File {
                path: format!("description/{name}"),
                contents: source.text.clone(),
            });
        }
    }
    files.push(File {
        path: ASSIGNMENT_FILE.to_owned(),
        contents: inputs.assignment.to_owned(),
    });
    files.push(File {
        path: RESULT_FILE.to_owned(),
        contents: inputs.result.to_owned(),
    });
    let provenance = provenance(
        &lowered,
        contents,
        &relative_to_description(inputs.description, inputs.description),
        &entries,
        &files,
    );
    files.push(File {
        path: PROVENANCE_FILE.to_owned(),
        contents: provenance,
    });
    let source_triggers = contents
        .tasks
        .iter()
        .filter_map(|task| match &task.released_by {
            Trigger::Source(source) => Some((task.name.clone(), source.clone())),
            _ => None,
        })
        .collect();
    Ok(Emitted {
        package,
        files,
        tasks: contents.tasks.len(),
        source_triggers,
        ticks_per_second: ticks.per_second,
        horizon,
        experimental,
    })
}

#[cfg(test)]
mod tests {
    use super::{header, horizon_of, main_rs, silent, tick, unknown_trigger, Digest, Inputs};
    use archogen_plan::contents::{read, Contents};

    /// The sporadic golden's plan, with one edit: the emitter accepts no plan it did not lower, so what the lowering
    /// never writes is reached here.
    fn plan(edit: impl Fn(&str) -> String) -> Contents {
        golden("sporadic", edit)
    }

    fn golden(name: &str, edit: impl Fn(&str) -> String) -> Contents {
        let path = format!(
            "{}/../archogen-plan/tests/goldens/{name}.plan",
            env!("CARGO_MANIFEST_DIR")
        );
        read(&edit(&std::fs::read_to_string(path).expect("the golden"))).expect("a plan")
    }

    #[test]
    fn a_route_the_plan_does_not_enable_does_not_arrive_and_the_trace_says_so() {
        // The enabled set configured as the plan writes it (`RTP-H1`).
        let contents =
            plan(|text| text.replace("(enabled timer (source dev.e))", "(enabled timer)"));
        assert_eq!(contents.enabled.len(), 1, "{:?}", contents.enabled);
        let route = &contents.routes[0];
        assert_eq!(silent(&contents, route), Some("is not enabled"));
        let ticks = tick(&contents).expect("a tick");
        let horizon = horizon_of(&contents, &ticks).expect("a horizon");
        let lines = header(&contents, &ticks, horizon, &Digest::of(b"p"), false);
        assert!(
            lines.contains(&"source dev.e is not enabled, so it does not arrive".to_owned()),
            "{lines:?}"
        );
    }

    #[test]
    fn a_table_s_slot_is_the_plan_s_index() {
        let contents = plan(|text| text.replace("(task 0 ", "(task 7 "));
        let program = eadl_front::Program {
            instances: Vec::new(),
        };
        let sources = eadl_front::SourceMap::new();
        let kinds = eadl_model::kind::Registry::default();
        let inputs = Inputs {
            program: &program,
            sources: &sources,
            description: "d",
            language: &[],
            assignment: "",
            result: "",
            plan: "",
            kinds: &kinds,
            acceptance: archogen_plan::Acceptance::TakenAsText,
        };
        let ticks = tick(&contents).expect("a tick");
        let horizon = horizon_of(&contents, &ticks).expect("a horizon");
        let refusal = main_rs(
            &inputs,
            &contents,
            &ticks,
            horizon,
            &Digest::of(b"p"),
            false,
        )
        .err()
        .expect("refused");
        assert_eq!(refusal.code, "tool-failure");
        assert!(
            refusal.message.contains("the table's slot 0"),
            "{}",
            refusal.message
        );
    }

    #[test]
    fn a_duration_or_a_horizon_past_the_ticks_is_refused_naming_what() {
        // A period past the ticks, and a source's separation.
        let contents = golden("heartbeat", |text| {
            text.replace("(period 3/100 s)", "(period 18446744073709551557 s)")
        });
        let ticks = tick(&contents).expect("a tick");
        let refusal = horizon_of(&contents, &ticks).expect_err("refused");
        assert_eq!(refusal.code, "unsupported-profile");
        assert!(
            refusal.message.contains("task `chime`'s period"),
            "{}",
            refusal.message
        );
        let contents =
            plan(|text| text.replace("(separation 1/50 s)", "(separation 36893488147419103232 s)"));
        let ticks = tick(&contents).expect("a tick");
        let refusal = horizon_of(&contents, &ticks).expect_err("refused");
        assert!(
            refusal.message.contains("source `dev.e`"),
            "{}",
            refusal.message
        );
        // Each of two primes fits, and their least common multiple does not.
        let contents = golden("heartbeat", |text| {
            text.replace("(period 1/100 s)", "(period 4294967311 s)")
                .replace("(period 3/100 s)", "(period 4294967357 s)")
        });
        let ticks = tick(&contents).expect("a tick");
        let refusal = horizon_of(&contents, &ticks).expect_err("refused");
        assert!(
            refusal.message.contains("one hyperperiod of this plan"),
            "{}",
            refusal.message
        );
    }

    #[test]
    fn only_what_the_run_repeats_sets_its_tick_and_its_horizon() {
        // A deadline in nanoseconds, which no table holds, moves neither.
        let contents = golden("heartbeat", |text| {
            text.replace("(deadline 1/100 s)", "(deadline 1/1000000000 s)")
        });
        assert_eq!(tick(&contents).expect("a tick").per_second, 100);
        // A task a source releases repeats only as the source does: its own minimum separation is no part of the run.
        let contents =
            plan(|text| text.replace("(min-separation 1/100 s)", "(min-separation 7/1000 s)"));
        let ticks = tick(&contents).expect("a tick");
        assert_eq!(ticks.per_second, 50);
        assert_eq!(horizon_of(&contents, &ticks).expect("a horizon"), 1);
        // A source that does not arrive does not either.
        let contents =
            plan(|text| text.replace("(enabled timer (source dev.e))", "(enabled timer)"));
        assert_eq!(tick(&contents).expect("a tick").per_second, 1);
    }

    #[test]
    fn a_provenance_is_whole_only_as_it_was_written() {
        let blank = format!(
            "{{\n  \"self\": \"{}\",\n  \"a\": 1\n}}\n",
            super::SELF_BLANK
        );
        let own = format!("sha256:{}", Digest::of(blank.as_bytes()).hex());
        let written = blank.replacen(super::SELF_BLANK, &own, 1);
        assert!(super::provenance_is_whole(&written));
        assert!(!super::provenance_is_whole(
            &written.replace("\"a\": 1", "\"a\": 2")
        ));
        assert!(!super::provenance_is_whole(&blank), "a blank is no digest");
        assert!(!super::provenance_is_whole("{}"), "no member");
        assert!(
            !super::provenance_is_whole("{\"self\": \"short\"}"),
            "too short to hold one"
        );
    }

    #[test]
    fn each_cause_has_its_own_code_and_repair() {
        // The search refuses a task's own causes first (`RTP-H24`), so no plan the search writes holds one: `RTP-H1` gives
        // them all the same. `(unstated ITEM)` has the second repair `RTP-H1` gives it.
        for (cause, code, repair) in [
            (
                "(own-unstated (source s))",
                "missing-fact",
                "state a `min-arrival-separation`",
            ),
            (
                "(own-faster (source s))",
                "unsupported-profile",
                "at least as far apart",
            ),
            (
                "(own-absent (source s))",
                "unsupported-profile",
                "at least as far apart",
            ),
            (
                "(unstated (source s))",
                "missing-fact",
                "state that source's `min-arrival-separation`",
            ),
            (
                "(faster (source s))",
                "missing-fact",
                "`(needs interrupt-source)`",
            ),
            (
                "(absent (source s))",
                "missing-fact",
                "`(needs interrupt-source)`",
            ),
            (
                "(several (source s) (source t))",
                "missing-fact",
                "`(needs interrupt-source)`",
            ),
            ("unreached", "missing-fact", "`(needs interrupt-source)`"),
        ] {
            let refusal = unknown_trigger("t", cause);
            assert_eq!(refusal.code, code, "{cause}");
            assert!(refusal.message.contains(cause), "{}", refusal.message);
            assert!(refusal.message.contains(repair), "{}", refusal.message);
        }
    }
}
