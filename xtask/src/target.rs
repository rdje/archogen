//! The §3.2 agreement between a target's eADL description and the device tree its emulator presents (leaf
//! `M2.8.3.2`, `docs/decisions/decision_target-platform-description.md`).
//!
//! One fact per requirement the target's `.env` names, each compared with the device-tree node it names **by what
//! that node is** — the `memory` node, the `sifive,clint0` node, the node `/chosen`'s `stdout-path` names — never by
//! address alone. A region the rules do not cover is itself a disagreement: a fact nothing compares is an
//! assertion, which is what `M2.8`'s reframing exists to end.

use std::collections::BTreeMap;

use archogen_api::{check, NoModules, Request, Status};
use eadl_front::{read, Form, SourceMap};

use crate::dtb::Node;

/// `key=value` lines of a target's `.env`; comments and blank lines skipped.
#[must_use]
pub fn env(text: &str) -> BTreeMap<String, String> {
    text.lines()
        .map(str::trim)
        .filter(|line| !line.is_empty() && !line.starts_with('#'))
        .filter_map(|line| line.split_once('='))
        .map(|(k, v)| (k.trim().to_string(), v.trim().to_string()))
        .collect()
}

/// A region of the description: its name, base and size in bytes, and whether it says `(executable true)`.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Region {
    name: String,
    base: u64,
    size: u64,
    executable: bool,
}

/// What the description states, gathered from its declarations.
#[derive(Debug, Default)]
struct Described {
    cores: Option<u64>,
    /// `(tick-rate …)` in Hz, and the block that offers it.
    tick_rate: Option<(u64, String)>,
    /// The block that offers `(observable-output true)`.
    output_block: Option<String>,
    /// Every region, with the declaration that offers it.
    regions: Vec<(String, Region)>,
    /// The blocks the platform `uses`.
    used: Vec<String>,
    /// Every block and platform declared, in written order.
    declared: Vec<String>,
    /// The levels each declaration states it is reachable from, `(reachable-at-privilege …)`.
    levels: BTreeMap<String, Vec<String>>,
    /// What a block states of the release timer (`decision_release-alarm.md` §1, `RA-H8`): the block, the fact, and
    /// the words after its name (leaf `M4.1.7.3.3`).
    release: Vec<(String, String, Vec<String>)>,
    /// The platform's `catalog-target`, the name its catalog records go by (`decision_runtime-plan-format-2.md` §2).
    catalog_target: Option<String>,
    /// The platform's `firmware-traps`, how many traps firmware beneath the image takes.
    firmware_traps: Option<u64>,
    /// The platform's `non-maskable-interrupts`, how many sources no mask stops.
    non_maskable: Option<u64>,
}

/// The QEMU release whose source was read for what it raises that no mask stops (`docs/book/src/ledger.md`'s `qemu`, leaf
/// `M4.1.7.3.4`): its RISC-V hart takes no non-maskable interrupt but Smrnmi's, and no device connects one.
const NON_MASKABLE_READ_AT: &str = "11.1.1";

/// The facts the release requester asks of the timer and its compare, which the release timer's block states, and what a system reads of
/// its counter beside them, its width and its wrap (`decision_device-correspondence.md`, `DC-H1`; leaf `M4.1.7.3.5`).
const RELEASE: [&str; 12] = [
    "relative-delay",
    "channels",
    "alarm-wiring",
    "alarm-kind",
    "counter-modulus",
    "compare-step",
    "compare-magnitude",
    "observation-coherent",
    "available-in-state",
    "supported-horizon",
    "counter-width",
    "wrap-behavior",
];

/// The words of `forms`: an integer by its digits, a symbol by its name, a list by its own words.
fn words(forms: &[Form]) -> Vec<String> {
    forms
        .iter()
        .map(|f| match f {
            Form::Integer { value, .. } => value.to_string(),
            Form::List { .. } => words(f.items()).join(" "),
            _ => f.as_symbol().unwrap_or("?").to_string(),
        })
        .collect()
}

/// A quantity `(n unit)` in the base unit, bytes or Hz. `None` for a unit the agreement does not compare.
fn quantity(value: &Form, unit: &Form) -> Option<u64> {
    let Form::Integer { value, .. } = value else {
        return None;
    };
    let factor: u64 = match unit.as_symbol()? {
        "byte" | "Hz" => 1,
        "KiB" => 1 << 10,
        "MiB" => 1 << 20,
        "GiB" => 1 << 30,
        "kHz" => 1_000,
        "MHz" => 1_000_000,
        "GHz" => 1_000_000_000,
        _ => return None,
    };
    u64::try_from(*value).ok()?.checked_mul(factor)
}

fn integer(form: &Form) -> Option<u64> {
    match form {
        Form::Integer { value, .. } => u64::try_from(*value).ok(),
        _ => None,
    }
}

/// `(region name (base …) (size …) [(executable true)])`, or why it cannot be read.
fn region(form: &Form) -> Result<Region, String> {
    let items = form.items();
    let name = items
        .get(1)
        .and_then(Form::as_symbol)
        .ok_or("a region without a name")?
        .to_string();
    let (mut base, mut size, mut executable) = (None, None, false);
    let mut seen: Vec<&str> = Vec::new();
    for clause in &items[2..] {
        let parts = clause.items();
        // Model §8 rule 5's grammar, as the joint rules read it (leaf `M3.3.5.1`): each sub-form at most once.
        if let Some(head) = clause.head() {
            if seen.contains(&head) {
                return Err(format!("region `{name}` has two `{head}`s"));
            }
            seen.push(head);
        }
        match clause.head() {
            Some("base") => base = parts.get(1).and_then(integer),
            Some("size") => {
                size = parts
                    .get(1)
                    .zip(parts.get(2))
                    .and_then(|(v, u)| quantity(v, u))
            }
            Some("executable") => {
                executable = match (parts.len(), parts.get(1).and_then(Form::as_symbol)) {
                    (2, Some("true")) => true,
                    (2, Some("false")) => false,
                    _ => {
                        return Err(format!(
                            "region `{name}`'s `executable` is `true` or `false`"
                        ))
                    }
                }
            }
            // `reserved` included: the agreement compares no reserved range, so the target may claim none.
            other => {
                return Err(format!(
                    "region `{name}` has a clause `{other:?}` the agreement does not read"
                ))
            }
        }
    }
    Ok(Region {
        base: base.ok_or_else(|| format!("region `{name}` has no readable base"))?,
        size: size
            .ok_or_else(|| format!("region `{name}` has no size in byte, KiB, MiB or GiB"))?,
        executable,
        name,
    })
}

fn describe(text: &str, name: &str) -> Result<Described, Vec<String>> {
    let mut sources = SourceMap::new();
    let id = sources
        .add(name.to_string(), text.to_string())
        .map_err(|e| vec![format!("{name}: {e:?}")])?;
    let (document, diagnostics) = read(&sources, id);
    if diagnostics.has_errors() {
        return Err(vec![format!(
            "{name} does not read: {}",
            diagnostics.render(&sources)
        )]);
    }
    let mut found = Described::default();
    let mut problems = Vec::new();
    for declaration in document.declarations() {
        let owner = declaration
            .items()
            .get(1)
            .and_then(Form::as_symbol)
            .unwrap_or("?")
            .to_string();
        let platform = declaration.head() == Some("defplatform");
        if matches!(declaration.head(), Some("defblock" | "defplatform")) {
            found.declared.push(owner.clone());
        }
        for clause in declaration.items().iter().skip(2) {
            match clause.head() {
                Some("offers") => {
                    for offer in &clause.items()[1..] {
                        let parts = offer.items();
                        match offer.head() {
                            Some("core-count") if platform => {
                                found.cores = parts.get(1).and_then(integer);
                            }
                            Some("tick-rate") => {
                                found.tick_rate = parts
                                    .get(1)
                                    .zip(parts.get(2))
                                    .and_then(|(v, u)| quantity(v, u))
                                    .map(|hz| (hz, owner.clone()));
                            }
                            Some("observable-output")
                                if parts.get(1).and_then(Form::as_symbol) == Some("true") =>
                            {
                                found.output_block = Some(owner.clone());
                            }
                            Some("region") => match region(offer) {
                                Ok(r) => found.regions.push((owner.clone(), r)),
                                Err(e) => problems.push(e),
                            },
                            Some(fact) if RELEASE.contains(&fact) && !platform => {
                                found.release.push((
                                    owner.clone(),
                                    fact.to_string(),
                                    words(&parts[1..]),
                                ));
                            }
                            Some("catalog-target") if platform => {
                                found.catalog_target =
                                    parts.get(1).and_then(Form::as_symbol).map(str::to_string);
                            }
                            Some("firmware-traps") if platform => {
                                found.firmware_traps = parts.get(1).and_then(integer);
                            }
                            Some("non-maskable-interrupts") if platform => {
                                found.non_maskable = parts.get(1).and_then(integer);
                            }
                            Some("reachable-at-privilege") => {
                                found.levels.insert(
                                    owner.clone(),
                                    parts[1..]
                                        .iter()
                                        .filter_map(Form::as_symbol)
                                        .map(str::to_string)
                                        .collect(),
                                );
                            }
                            // The target states nothing the agreement cannot check (leaf `M3.3.4.4`).
                            _ => problems.push(format!(
                                "`{owner}` offers `{}`, which the agreement does not compare",
                                offer.head().or_else(|| offer.as_symbol()).unwrap_or("?")
                            )),
                        }
                    }
                }
                Some("requires") if platform => {
                    for need in &clause.items()[1..] {
                        if need.head() == Some("uses") {
                            found.used.extend(
                                need.items()[1..]
                                    .iter()
                                    .filter_map(Form::as_symbol)
                                    .map(str::to_string),
                            );
                        }
                    }
                }
                Some(other) if matches!(declaration.head(), Some("defblock" | "defplatform")) => {
                    problems.push(format!(
                        "`{owner}` writes a `{other}` clause, which the agreement does not read"
                    ));
                }
                _ => {}
            }
        }
    }
    if problems.is_empty() {
        Ok(found)
    } else {
        Err(problems)
    }
}

/// The machine-timer lines of a CLINT's `interrupts-extended`, each cause 7, the privileged specification's machine-timer
/// interrupt, with whether it reaches a hart's own interrupt controller, `riscv,cpu-intc`: a specifier is a phandle and as
/// many cells as the controller it names says, by its `#interrupt-cells`.
fn timer_lines(clint: &Node, nodes: &[(&Node, (usize, usize))]) -> Vec<bool> {
    let Some(bytes) = property(clint, "interrupts-extended") else {
        return Vec::new();
    };
    let cells_of: Vec<u32> = bytes
        .chunks(4)
        .filter_map(|c| c.try_into().ok().map(u32::from_be_bytes))
        .collect();
    let mut out = Vec::new();
    let mut at = 0;
    while at < cells_of.len() {
        let handle = u64::from(cells_of[at]);
        let target = nodes
            .iter()
            .map(|(n, _)| *n)
            .find(|n| property(n, "phandle").and_then(cells) == Some(handle));
        let width = target
            .and_then(|n| property(n, "#interrupt-cells"))
            .and_then(cells)
            .and_then(|w| usize::try_from(w).ok())
            .unwrap_or(1);
        if width == 0 {
            break;
        }
        if cells_of.get(at + 1) == Some(&7) {
            out.push(target.is_some_and(|n| {
                strings(n, "compatible")
                    .iter()
                    .any(|c| c == "riscv,cpu-intc")
            }));
        }
        at += 1 + width;
    }
    out
}

/// A duration `(n unit)` as a fraction of a second, `(n, per)`, for the units a horizon is written in.
fn seconds(words: &[String]) -> Option<(u128, u128)> {
    let n: u128 = words.first()?.parse().ok()?;
    let per: u128 = match words.get(1)?.as_str() {
        "s" => 1,
        "ms" => 1_000,
        "us" => 1_000_000,
        "ns" => 1_000_000_000,
        _ => return None,
    };
    Some((n, per))
}

/// A node's property as bytes.
fn property<'n>(node: &'n Node, name: &str) -> Option<&'n [u8]> {
    node.properties
        .iter()
        .find(|(n, _)| n == name)
        .map(|(_, v)| v.as_slice())
}

/// The strings of a string or string-list property.
fn strings(node: &Node, name: &str) -> Vec<String> {
    property(node, name)
        .map(|v| {
            v.split(|b| *b == 0)
                .filter(|s| !s.is_empty())
                .map(|s| String::from_utf8_lossy(s).into_owned())
                .collect()
        })
        .unwrap_or_default()
}

fn cells(bytes: &[u8]) -> Option<u64> {
    let mut value: u64 = 0;
    if !bytes.len().is_multiple_of(4) || bytes.len() > 8 {
        return None;
    }
    for chunk in bytes.chunks(4) {
        value = (value << 32) | u64::from(u32::from_be_bytes(chunk.try_into().ok()?));
    }
    Some(value)
}

/// Every node, with its parent's `#address-cells` and `#size-cells` (defaults 2 and 1).
fn flatten<'n>(
    node: &'n Node,
    cells_of_parent: (usize, usize),
    out: &mut Vec<(&'n Node, (usize, usize))>,
) {
    out.push((node, cells_of_parent));
    let own = (
        property(node, "#address-cells")
            .and_then(cells)
            .map_or(2, |c| c as usize),
        property(node, "#size-cells")
            .and_then(cells)
            .map_or(1, |c| c as usize),
    );
    for child in &node.children {
        flatten(child, own, out);
    }
}

/// A node's first `reg` entry as `(base, size)`.
fn reg(node: &Node, (address, size): (usize, usize)) -> Option<(u64, u64)> {
    let bytes = property(node, "reg")?;
    let (a, s) = (address * 4, size * 4);
    if bytes.len() < a + s {
        return None;
    }
    Some((cells(&bytes[..a])?, cells(&bytes[a..a + s])?))
}

/// Compare region `r` with `node`'s first `reg` entry, recording any difference under `what`.
fn compare(r: &Region, what: &str, node: Option<(&Node, (usize, usize))>, wrong: &mut Vec<String>) {
    let Some((node, parent)) = node else {
        wrong.push(format!(
            "{what}: the device tree has no node for region `{}`",
            r.name
        ));
        return;
    };
    match reg(node, parent) {
        Some((base, size)) if (base, size) == (r.base, r.size) => {}
        Some((base, size)) => wrong.push(format!(
            "{what}: region `{}` is {:#x} bytes at {:#x}, and {} is {size:#x} bytes at {base:#x}",
            r.name, r.size, r.base, node.path
        )),
        None => wrong.push(format!("{what}: {} has no readable `reg`", node.path)),
    }
}

/// Every way `description` and the device tree `root` disagree, for the requirements in `env`.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn disagreements(
    env: &BTreeMap<String, String>,
    description: &str,
    name: &str,
    root: &Node,
) -> Vec<String> {
    let mut wrong = Vec::new();

    // `archogen check` admits the description against the profile.
    let response = check(&Request {
        name,
        text: description,
        profile: None,
        modules: &NoModules,
        target: None,
    });
    if response.status != Status::Ok {
        wrong.push(format!(
            "archogen check does not admit {name}: {} {}",
            response.status.slug(),
            response.render_diagnostics()
        ));
    }
    let described = match describe(description, name) {
        Ok(described) => described,
        Err(problems) => {
            wrong.extend(problems);
            return wrong;
        }
    };
    let mut nodes = Vec::new();
    flatten(root, (2, 1), &mut nodes);
    let find = |pred: &dyn Fn(&Node) -> bool| nodes.iter().find(|(n, _)| pred(n)).copied();
    let region_of = |owner: &str| {
        described
            .regions
            .iter()
            .find(|(o, _)| o == owner)
            .map(|(_, r)| r)
    };
    let mut covered: Vec<&str> = Vec::new();
    let requires = |key: &str| env.get(key).map(String::as_str) == Some("yes");

    // REQUIRES_CORES: `core-count` against the `cpu` nodes under `/cpus`.
    if let Some(wanted) = env.get("REQUIRES_CORES") {
        let harts = nodes
            .iter()
            .filter(|(n, _)| {
                n.path.starts_with("/cpus/cpu@") && strings(n, "device_type") == ["cpu"]
            })
            .count() as u64;
        match described.cores {
            None => wrong.push("REQUIRES_CORES: the platform offers no `core-count`".into()),
            Some(cores) if cores != harts => wrong.push(format!(
                "REQUIRES_CORES: the description offers core-count {cores}, and the device tree has {harts} hart(s)"
            )),
            Some(cores) if wanted.parse::<u64>().ok() != Some(cores) => wrong.push(format!(
                "REQUIRES_CORES: the .env requires {wanted}, and the description offers {cores}"
            )),
            Some(_) => {}
        }
    }

    // REQUIRES_EXECUTABLE_RAM: the executable region against the `memory` node.
    if requires("REQUIRES_EXECUTABLE_RAM") {
        match described.regions.iter().find(|(_, r)| r.executable) {
            None => {
                wrong.push("REQUIRES_EXECUTABLE_RAM: no region says `(executable true)`".into())
            }
            Some((_, r)) => {
                covered.push(&r.name);
                let memory = find(&|n: &Node| strings(n, "device_type") == ["memory"]);
                compare(r, "REQUIRES_EXECUTABLE_RAM", memory, &mut wrong);
            }
        }
    }

    // REQUIRES_TIMER: the tick-rate block's region against the CLINT, and the tick-rate against the timebase.
    if requires("REQUIRES_TIMER") {
        match &described.tick_rate {
            None => wrong.push("REQUIRES_TIMER: no block offers a `tick-rate`".into()),
            Some((hz, owner)) => {
                if !described.used.contains(owner) {
                    wrong.push(format!("REQUIRES_TIMER: `{owner}` offers the tick-rate, and the platform does not use it"));
                }
                match region_of(owner) {
                    None => wrong.push(format!(
                        "REQUIRES_TIMER: `{owner}` offers no region to compare"
                    )),
                    Some(r) => {
                        covered.push(&r.name);
                        let clint = find(&|n: &Node| {
                            strings(n, "compatible")
                                .iter()
                                .any(|c| c == "sifive,clint0")
                        });
                        compare(r, "REQUIRES_TIMER", clint, &mut wrong);
                    }
                }
                let timebase = find(&|n: &Node| n.path == "/cpus")
                    .and_then(|(n, _)| property(n, "timebase-frequency"))
                    .and_then(cells);
                match timebase {
                    None => wrong.push("REQUIRES_TIMER: /cpus has no timebase-frequency".into()),
                    Some(f) if f != *hz => wrong.push(format!(
                        "REQUIRES_TIMER: the description offers tick-rate {hz} Hz, and /cpus's timebase-frequency is {f} Hz"
                    )),
                    Some(_) => {}
                }
            }
        }
    }

    // REQUIRES_OBSERVABLE_OUTPUT: the output block's region against the node `stdout-path` names.
    if requires("REQUIRES_OBSERVABLE_OUTPUT") {
        match &described.output_block {
            None => wrong.push(
                "REQUIRES_OBSERVABLE_OUTPUT: no block offers `(observable-output true)`".into(),
            ),
            Some(owner) => {
                if !described.used.contains(owner) {
                    wrong.push(format!(
                        "REQUIRES_OBSERVABLE_OUTPUT: `{owner}` offers the output, and the platform does not use it"
                    ));
                }
                let stdout = find(&|n: &Node| n.path == "/chosen")
                    .and_then(|(n, _)| strings(n, "stdout-path").into_iter().next());
                if stdout.is_none() {
                    wrong.push("REQUIRES_OBSERVABLE_OUTPUT: /chosen names no stdout-path".into());
                }
                match region_of(owner) {
                    None => wrong.push(format!(
                        "REQUIRES_OBSERVABLE_OUTPUT: `{owner}` offers no region to compare"
                    )),
                    Some(r) => {
                        covered.push(&r.name);
                        let console = stdout.as_ref().and_then(|path| {
                            find(&|n: &Node| {
                                &n.path == path
                                    && strings(n, "compatible").iter().any(|c| c == "ns16550a")
                            })
                        });
                        compare(r, "REQUIRES_OBSERVABLE_OUTPUT", console, &mut wrong);
                    }
                }
            }
        }
    }

    // REQUIRES_PRIVILEGE: every block and the platform states the level the .env names. The device tree carries no
    // level; the value's source is the privileged specification and the spike's run at it (the target decision §1).
    match env.get("REQUIRES_PRIVILEGE") {
        Some(wanted) => {
            for owner in &described.declared {
                match described.levels.get(owner) {
                    None => wrong.push(format!("REQUIRES_PRIVILEGE: `{owner}` states no level")),
                    Some(levels) if levels.iter().any(|l| l == wanted) => {}
                    Some(levels) => wrong.push(format!(
                        "REQUIRES_PRIVILEGE: `{owner}` is reachable from {}, and the .env requires {wanted}",
                        levels.join(" ")
                    )),
                }
            }
        }
        None => {
            for owner in described.levels.keys() {
                wrong.push(format!(
                    "`{owner}` states a level, and the .env requires none to compare it with"
                ));
            }
        }
    }

    // REQUIRES_RELEASE_TIMER: what the release requester asks of the timer (`decision_release-alarm.md` §1, `RA-H8`), stated
    // by the block that offers the tick-rate, the CLINT. Its delay, its lines and their wiring are read off the CLINT's
    // `interrupts-extended`; its modulus, its compare's kind, step and magnitude, its states and its coherence are the
    // privileged specification's machine timer, which the node compatible with `riscv,clint0` is: "The `mtime` register
    // has a 64-bit precision on all RV32 and RV64 systems", "will wrap around if the count overflows", "A machine timer
    // interrupt becomes pending whenever `mtime` contains a value greater than or equal to `mtimecmp`, treating the values
    // as unsigned integers", "`mtime` must increment at constant frequency", and on RV64 64-bit accesses "are atomic"; its
    // horizon, a capacity, at most what one compare reaches, `(2^64 − 1)` ticks of `/cpus`'s timebase (leaf `M4.1.7.3.3`). The same two
    // sentences give what a system reads of the counter, `DC-H1`'s: its width, 64 bits, and its wrap, modular (leaf `M4.1.7.3.5`).
    if requires("REQUIRES_RELEASE_TIMER") {
        let owner = described.tick_rate.as_ref().map(|(_, o)| o.clone());
        for (o, f, _) in &described.release {
            if Some(o) != owner.as_ref() {
                wrong.push(format!(
                    "REQUIRES_RELEASE_TIMER: `{o}` states `{f}`, and only the block offering the tick-rate is the release timer"
                ));
            }
        }
        let stated = |fact: &str| {
            described
                .release
                .iter()
                .find(|(o, f, _)| Some(o) == owner.as_ref() && f == fact)
                .map(|(_, _, w)| w.clone())
        };
        let clint = find(&|n: &Node| strings(n, "compatible").iter().any(|c| c == "riscv,clint0"));
        let lines = clint
            .map(|(n, _)| timer_lines(n, &nodes))
            .unwrap_or_default();
        let rv64 = find(&|n: &Node| n.path.starts_with("/cpus/cpu@"))
            .is_some_and(|(n, _)| strings(n, "riscv,isa-base") == ["rv64i"]);
        let gives = |word: &str| vec![word.to_string()];
        let expected: [(&str, &str, Vec<String>); 11] = [
            (
                "relative-delay",
                "the CLINT",
                gives(if lines.is_empty() { "false" } else { "true" }),
            ),
            ("channels", "the CLINT", gives(&lines.len().to_string())),
            (
                "alarm-wiring",
                "the CLINT",
                gives(if !lines.is_empty() && lines.iter().all(|l| *l) {
                    "core"
                } else {
                    "controller"
                }),
            ),
            (
                "alarm-kind",
                "the privileged specification",
                gives("level-compare"),
            ),
            (
                "counter-modulus",
                "the privileged specification",
                gives("pow2 64"),
            ),
            ("compare-step", "the privileged specification", gives("1")),
            (
                "compare-magnitude",
                "the privileged specification",
                gives("true"),
            ),
            (
                "observation-coherent",
                "the privileged specification on RV64",
                gives(if rv64 { "true" } else { "false" }),
            ),
            (
                "available-in-state",
                "the privileged specification",
                vec!["idle".to_string(), "run".to_string()],
            ),
            (
                "counter-width",
                "the privileged specification",
                vec!["64".to_string(), "bit".to_string()],
            ),
            (
                "wrap-behavior",
                "the privileged specification",
                gives("modular"),
            ),
        ];
        match (&owner, clint) {
            (Some(_), Some(_)) => {}
            _ => wrong.push("REQUIRES_RELEASE_TIMER: no tick-rate block, or no node compatible with `riscv,clint0`".into()),
        }
        let owner_name = owner.clone().unwrap_or_else(|| "?".to_string());
        for (fact, source, want) in &expected {
            match stated(fact) {
                None => wrong.push(format!(
                    "REQUIRES_RELEASE_TIMER: `{owner_name}` states no `{fact}`"
                )),
                Some(mut got) => {
                    // A count's unit is the language's, and a set's members are in no order.
                    if got.last().map(String::as_str) == Some("tick") {
                        got.pop();
                    }
                    let mut compared = got.clone();
                    if *fact == "available-in-state" {
                        compared.sort();
                    }
                    if &compared != want {
                        wrong.push(format!(
                            "REQUIRES_RELEASE_TIMER: `{owner_name}` states `({fact} {})`, and {source} gives `{}`",
                            got.join(" "),
                            want.join(" ")
                        ));
                    }
                }
            }
        }
        let timebase = find(&|n: &Node| n.path == "/cpus")
            .and_then(|(n, _)| property(n, "timebase-frequency"))
            .and_then(cells);
        match (stated("supported-horizon").as_deref().and_then(seconds), timebase) {
            (Some((n, per)), Some(hz)) => {
                // n / per seconds of hz ticks a second, held within 2^64 − 1 ticks, in exact integers.
                if n.saturating_mul(u128::from(hz)) > ((1_u128 << 64) - 1).saturating_mul(per) {
                    wrong.push(format!(
                        "REQUIRES_RELEASE_TIMER: `{owner_name}` states a supported-horizon beyond what one compare reaches, 2^64 − 1 ticks at {hz} Hz"
                    ));
                }
            }
            (None, _) => wrong.push(format!(
                "REQUIRES_RELEASE_TIMER: `{owner_name}` states no supported-horizon in s, ms, us or ns"
            )),
            (Some(_), None) => wrong.push("REQUIRES_RELEASE_TIMER: /cpus has no timebase-frequency".into()),
        }
    } else if let Some((o, f, _)) = described.release.first() {
        wrong.push(format!(
            "`{o}` states `{f}`, and the .env requires no release timer to compare it with"
        ));
    }

    // REQUIRES_CATALOG_TARGET: the platform's `catalog-target`, the name its catalog records go by, which is the
    // `.env`'s own target (`decision_runtime-plan-format-2.md` §2, leaf `M4.1.7.3.3`).
    if requires("REQUIRES_CATALOG_TARGET") {
        let target = env.get("TARGET_ID").map_or("none", String::as_str);
        match &described.catalog_target {
            None => wrong.push("REQUIRES_CATALOG_TARGET: the platform offers no `catalog-target`".into()),
            Some(got) if got != target => wrong.push(format!(
                "REQUIRES_CATALOG_TARGET: the platform offers catalog-target `{got}`, and the .env's target is `{target}`"
            )),
            Some(_) => {}
        }
    } else if described.catalog_target.is_some() {
        wrong.push(
            "the platform states a `catalog-target`, and the .env requires none to compare it with"
                .into(),
        );
    }

    // REQUIRES_FIRMWARE_TRAPS: the platform's `firmware-traps`, 0 only where the emulator starts the image with no firmware
    // beneath it, the `.env`'s `QEMU_BIOS=none`; any other count nothing here checks (`decision_runtime-plan-format-2.md` §2).
    if requires("REQUIRES_FIRMWARE_TRAPS") {
        let bios = env.get("QEMU_BIOS").map_or("unset", String::as_str);
        match described.firmware_traps {
            None => wrong.push("REQUIRES_FIRMWARE_TRAPS: the platform offers no `firmware-traps`".into()),
            Some(0) if bios == "none" => {}
            Some(n) => wrong.push(format!(
                "REQUIRES_FIRMWARE_TRAPS: the platform offers firmware-traps {n}, and the .env starts the image with QEMU_BIOS={bios}"
            )),
        }
    } else if described.firmware_traps.is_some() {
        wrong.push(
            "the platform states `firmware-traps`, and the .env requires none to compare it with"
                .into(),
        );
    }

    // REQUIRES_NON_MASKABLE_INTERRUPTS: the platform's `non-maskable-interrupts`, 0 only where no hart of the dump offers
    // `smrnmi` and the pinned release is the one whose source was read: in QEMU 11.1.1 a RISC-V hart marks pending and takes a
    // non-maskable interrupt only under Smrnmi (`target/riscv/tcg/cpu_helper.c`, lines 339-348 and 2082), aborts the emulator
    // on an M-mode double trap without it (line 2317), and no device in `hw/` connects its `riscv.cpu.rnmi` input
    // (`target/riscv/tcg/tcg-cpu.c`, lines 1682-1683); a hart offers Smrnmi exactly where its ISA string names it
    // (`target/riscv/cpu.c`, line 267). A count other than 0 nothing here checks (`decision_runtime-plan-format-2.md` §2).
    if requires("REQUIRES_NON_MASKABLE_INTERRUPTS") {
        let rnmi = nodes.iter().any(|(n, _)| {
            n.path.starts_with("/cpus/cpu@")
                && (strings(n, "riscv,isa-extensions")
                    .iter()
                    .any(|e| e == "smrnmi")
                    || strings(n, "riscv,isa")
                        .iter()
                        .any(|isa| isa.split('_').any(|e| e == "smrnmi")))
        });
        let pinned = env
            .get("QEMU_VERSION_PINNED")
            .map_or("unset", String::as_str);
        match described.non_maskable {
            None => wrong.push(
                "REQUIRES_NON_MASKABLE_INTERRUPTS: the platform offers no `non-maskable-interrupts`".into(),
            ),
            Some(0) if rnmi => wrong.push(
                "REQUIRES_NON_MASKABLE_INTERRUPTS: the platform offers non-maskable-interrupts 0, and a hart of the device tree offers `smrnmi`".into(),
            ),
            Some(0) if pinned != NON_MASKABLE_READ_AT => wrong.push(format!(
                "REQUIRES_NON_MASKABLE_INTERRUPTS: the platform offers non-maskable-interrupts 0, and the .env pins QEMU {pinned} where the release whose source was read is {NON_MASKABLE_READ_AT}: read the new release's for what it raises"
            )),
            Some(0) => {}
            Some(n) => wrong.push(format!(
                "REQUIRES_NON_MASKABLE_INTERRUPTS: the platform offers non-maskable-interrupts {n}, a count nothing here checks"
            )),
        }
    } else if described.non_maskable.is_some() {
        wrong.push("the platform states `non-maskable-interrupts`, and the .env requires none to compare it with".into());
    }

    // A region no rule compared is an assertion nothing checks.
    for (_, r) in &described.regions {
        if !covered.contains(&r.name.as_str()) {
            wrong.push(format!(
                "region `{}` is compared by no requirement's rule",
                r.name
            ));
        }
    }
    wrong
}

#[cfg(test)]
mod tests {
    use super::{disagreements, env};
    use crate::dtb::{self, Node};
    use std::path::PathBuf;

    fn root() -> PathBuf {
        PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask sits in the repository root")
            .to_path_buf()
    }

    fn kept() -> (std::collections::BTreeMap<String, String>, String, Node) {
        let env = env(
            &std::fs::read_to_string(root().join("targets/riscv-virt-up.env")).expect("the .env"),
        );
        let description = std::fs::read_to_string(root().join(&env["PLATFORM_DESCRIPTION"]))
            .expect("the description");
        let tree = dtb::parse(
            &std::fs::read(root().join("docs/targets/riscv-virt-up.dtb")).expect("the kept dump"),
        )
        .expect("the kept dump parses");
        (env, description, tree)
    }

    /// The disagreements after replacing `from` by `to` in the description, which must occur exactly once.
    fn with(from: &str, to: &str) -> Vec<String> {
        let (env, description, tree) = kept();
        assert_eq!(
            description.matches(from).count(),
            1,
            "the arm's text `{from}` moved"
        );
        disagreements(&env, &description.replacen(from, to, 1), "arm.eadl", &tree)
    }

    fn reported(wrong: &[String], needle: &str) {
        assert!(
            wrong.iter().any(|w| w.contains(needle)),
            "expected `{needle}` among {wrong:#?}"
        );
    }

    #[test]
    fn the_kept_description_agrees_with_the_kept_device_tree() {
        let (env, description, tree) = kept();
        let wrong = disagreements(&env, &description, "targets/riscv-virt-up.eadl", &tree);
        assert!(wrong.is_empty(), "{wrong:#?}");
        for key in [
            "REQUIRES_CORES",
            "REQUIRES_TIMER",
            "REQUIRES_OBSERVABLE_OUTPUT",
            "REQUIRES_EXECUTABLE_RAM",
            "REQUIRES_PRIVILEGE",
            "REQUIRES_RELEASE_TIMER",
            "REQUIRES_CATALOG_TARGET",
            "REQUIRES_FIRMWARE_TRAPS",
            "REQUIRES_NON_MASKABLE_INTERRUPTS",
        ] {
            assert!(
                env.contains_key(key),
                "the .env no longer names {key}, so this leg compares less"
            );
        }
        // The catalog's name is held to `TARGET_ID`, which is the `.env`'s stem (`decision_runtime-plan-format-2.md` §2).
        assert_eq!(
            env.get("TARGET_ID").map(String::as_str),
            Some("riscv-virt-up")
        );
        assert!(root().join("targets/riscv-virt-up.env").is_file());
    }

    /// Leaf `M4.1.7.3.3`: what the release requester asks of the timer, each fact against its source, and the platform's
    /// two facts format `/2` reads, each reported when it differs (`decision_release-alarm.md` §1, `RA-H8`;
    /// `decision_runtime-plan-format-2.md` §2).
    #[test]
    fn each_fact_format_2_reads_is_reported_when_it_differs() {
        for (from, to, needle) in [
            ("(relative-delay true)", "", "states no `relative-delay`"),
            (
                "(channels 1 tick)",
                "(channels 2 tick)",
                "`(channels 2)`, and the CLINT gives `1`",
            ),
            (
                "(alarm-wiring core)",
                "(alarm-wiring controller)",
                "`(alarm-wiring controller)`, and the CLINT gives `core`",
            ),
            (
                "(alarm-kind level-compare)",
                "(alarm-kind delay)",
                "`(alarm-kind delay)`",
            ),
            (
                "(counter-modulus (pow2 64))",
                "(counter-modulus (pow2 32))",
                "`(counter-modulus pow2 32)`",
            ),
            (
                "(compare-step 1 tick)",
                "(compare-step 2 tick)",
                "`(compare-step 2)`",
            ),
            (
                "(compare-magnitude true)",
                "(compare-magnitude false)",
                "`(compare-magnitude false)`",
            ),
            (
                "(observation-coherent true)",
                "",
                "states no `observation-coherent`",
            ),
            (
                "(available-in-state run idle)",
                "(available-in-state run)",
                "`(available-in-state run)`",
            ),
            (
                "(supported-horizon 1844674407370 s)",
                "(supported-horizon 1844674407371 s)",
                "beyond what one compare reaches",
            ),
            (
                "(counter-width 64 bit)",
                "(counter-width 32 bit)",
                "`(counter-width 32 bit)`, and the privileged specification gives `64 bit`",
            ),
            (
                "(wrap-behavior modular)",
                "(wrap-behavior saturating)",
                "`(wrap-behavior saturating)`, and the privileged specification gives `modular`",
            ),
            (
                "(catalog-target riscv-virt-up)",
                "(catalog-target riscv-virt)",
                "catalog-target `riscv-virt`",
            ),
            (
                "(firmware-traps 0)",
                "(firmware-traps 1)",
                "firmware-traps 1",
            ),
        ] {
            reported(&with(from, to), needle);
        }
        // The device tree's side: a CLINT whose timer line is gone has no delay, no line and no wiring to compare.
        let (env, description, mut tree) = kept();
        fn clint(node: &mut Node) -> Option<&mut Node> {
            if node.path.starts_with("/soc/clint@") {
                return Some(node);
            }
            node.children.iter_mut().find_map(clint)
        }
        let node = clint(&mut tree).expect("the kept dump's CLINT");
        let lines = node
            .properties
            .iter_mut()
            .find(|(n, _)| n == "interrupts-extended")
            .expect("its interrupts-extended");
        lines.1.truncate(8);
        let wrong = disagreements(&env, &description, "targets/riscv-virt-up.eadl", &tree);
        reported(&wrong, "`(channels 1)`, and the CLINT gives `0`");
        reported(
            &wrong,
            "`(relative-delay true)`, and the CLINT gives `false`",
        );
    }

    #[test]
    fn each_compared_fact_is_reported_when_it_differs() {
        reported(
            &with("(core-count 1 tick)", "(core-count 2 tick)"),
            "REQUIRES_CORES",
        );
        reported(
            &with("(size 128 MiB)", "(size 64 MiB)"),
            "REQUIRES_EXECUTABLE_RAM",
        );
        reported(
            &with(" (executable true)", ""),
            "no region says `(executable true)`",
        );
        reported(
            &with("(base 0x0200_0000)", "(base 0x0300_0000)"),
            "REQUIRES_TIMER",
        );
        reported(
            &with("(tick-rate 10 MHz)", "(tick-rate 12 MHz)"),
            "timebase-frequency is 10000000 Hz",
        );
        reported(
            &with("(size 256 byte)", "(size 512 byte)"),
            "REQUIRES_OBSERVABLE_OUTPUT",
        );
        reported(
            &with("(uses target.timer target.console)", "(uses target.timer)"),
            "does not use it",
        );
        reported(
            &with(
                "(executable true)))",
                "(executable true))\n    (region target.extra (base 0x0010_0000) (size 4 KiB)))",
            ),
            "region `target.extra` is compared by no requirement's rule",
        );
        reported(
            &with(
                "(defplatform target.riscv-virt-up",
                "(defplatfrom target.riscv-virt-up",
            ),
            "archogen check does not admit",
        );
    }

    /// Leaf `M4.1.7.3.4`: the count of non-maskable interrupts, 0 only where the dump's harts offer no `smrnmi` and the pin is
    /// the release whose source was read, which takes no non-maskable interrupt but Smrnmi's and connects none.
    #[test]
    fn the_non_maskable_count_is_held_to_the_dump_and_the_reading() {
        reported(
            &with("(non-maskable-interrupts 0)", "(non-maskable-interrupts 1)"),
            "non-maskable-interrupts 1",
        );
        let (env, description, mut tree) = kept();
        fn hart(node: &mut Node) -> Option<&mut Node> {
            if node.path.starts_with("/cpus/cpu@") {
                return Some(node);
            }
            node.children.iter_mut().find_map(hart)
        }
        let extensions = hart(&mut tree)
            .expect("the kept dump's hart")
            .properties
            .iter_mut()
            .find(|(n, _)| n == "riscv,isa-extensions")
            .expect("its extensions");
        extensions.1.extend_from_slice(b"smrnmi\0");
        reported(
            &disagreements(&env, &description, "targets/riscv-virt-up.eadl", &tree),
            "offers `smrnmi`",
        );
        let (mut moved, description, tree) = kept();
        moved.insert("QEMU_VERSION_PINNED".to_string(), "11.2.0".to_string());
        reported(
            &disagreements(&moved, &description, "targets/riscv-virt-up.eadl", &tree),
            "whose source was read is 11.1.1",
        );
    }

    #[test]
    fn what_the_agreement_does_not_read_is_refused() {
        // Leaf `M3.3.4.4`: the target states nothing it cannot check, so an offer or a clause no rule reads is a
        // disagreement, as an uncompared region already was.
        reported(
            &with("(tick-rate 10 MHz)", "(tick-rate 10 MHz) (debug-port true)"),
            "`target.timer` offers `debug-port`, which the agreement does not compare",
        );
        reported(
            &with(
                "(defblock target.console\n",
                "(defblock target.console\n  (absent debug-port)\n",
            ),
            "`target.console` writes a `absent` clause, which the agreement does not read",
        );
    }

    #[test]
    fn a_region_s_sub_forms_follow_the_joint_rules_grammar() {
        // Leaf `M3.3.5.1`: the same refusals as `crates/eadl-resolve/src/joint.rs`'s reader, and `reserved` refused
        // until the agreement compares it.
        reported(
            &with("(executable true)", "(executable yes)"),
            "region `target.ram`'s `executable` is `true` or `false`",
        );
        reported(
            &with("(executable true)", "(executable true) (executable true)"),
            "region `target.ram` has two `executable`s",
        );
        reported(
            &with("(size 256 byte)", "(size 256 byte) (reserved true)"),
            "region `target.console` has a clause `Some(\"reserved\")` the agreement does not read",
        );
    }

    #[test]
    fn every_block_and_the_platform_state_the_required_level() {
        // Leaf `M3.3.4.4`: REQUIRES_PRIVILEGE against each declaration's `reachable-at-privilege`.
        let (env, _, _) = kept();
        assert_eq!(
            env.get("REQUIRES_PRIVILEGE").map(String::as_str),
            Some("machine")
        );
        // The timer's last fact before its level, since leaf `M4.1.7.3.3` gave it the release timer's facts.
        let timer = "(supported-horizon 1844674407370 s)\n    (reachable-at-privilege machine)";
        reported(
            &with(timer, "(supported-horizon 1844674407370 s)"),
            "REQUIRES_PRIVILEGE: `target.timer` states no level",
        );
        reported(
            &with(
                timer,
                "(supported-horizon 1844674407370 s)\n    (reachable-at-privilege supervisor)",
            ),
            "REQUIRES_PRIVILEGE: `target.timer` is reachable from supervisor, and the .env requires machine",
        );
    }

    #[test]
    fn a_node_is_matched_by_what_it_is_not_by_address_alone() {
        // The UART at the same address, but no longer the node `stdout-path` names: the console region has no node.
        let (env, description, mut tree) = kept();
        fn chosen(node: &mut Node) -> Option<&mut Node> {
            if node.path == "/chosen" {
                return Some(node);
            }
            node.children.iter_mut().find_map(chosen)
        }
        let node = chosen(&mut tree).expect("/chosen");
        let before = node.properties.len();
        node.properties.retain(|(name, _)| name != "stdout-path");
        node.properties
            .push(("stdout-path".into(), b"/soc/rtc@101000\0".to_vec()));
        assert_eq!(node.properties.len(), before, "the edit applied");
        let wrong = disagreements(&env, &description, "arm.eadl", &tree);
        reported(
            &wrong,
            "the device tree has no node for region `target.console`",
        );
    }
}
