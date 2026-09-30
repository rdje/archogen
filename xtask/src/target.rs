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
    for clause in &items[2..] {
        let parts = clause.items();
        match clause.head() {
            Some("base") => base = parts.get(1).and_then(integer),
            Some("size") => {
                size = parts
                    .get(1)
                    .zip(parts.get(2))
                    .and_then(|(v, u)| quantity(v, u))
            }
            Some("executable") => {
                executable = parts.get(1).and_then(Form::as_symbol) == Some("true")
            }
            Some("reserved") => {}
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
                            _ => {}
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
        ] {
            assert!(
                env.contains_key(key),
                "the .env no longer names {key}, so this leg compares less"
            );
        }
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
