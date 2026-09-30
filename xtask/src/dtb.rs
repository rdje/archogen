//! A flattened device tree, read and rendered as stable text (leaf `M2.8.2`).
//!
//! ⭐ **Why this exists.** §3.2: "Pin the emulator configuration, inspect the produced hardware description, and
//! verify agreement with the eADL platform fixture." The produced hardware description is the device tree QEMU
//! hands its guest (`scripts/target_emulator.sh --dump-dtb`). `docs/targets/riscv-virt-up.dtb.summary.md` holds
//! it rendered as text; `cargo xtask dtb-check` renders a fresh dump the same way and requires the two to be
//! identical, so a QEMU that presents another platform under the pinned options fails the emulator step instead
//! of silently invalidating every observation taken against it.
//!
//! Read here, with no dependency, rather than through `dtc`: the format is a header, a token stream and a string
//! table, and a tool the check depends on is one more thing to provision and pin.
//!
//! **One property is masked**, and only by name: QEMU writes 32 fresh random bytes to `/chosen/rng-seed` on every
//! boot — two dumps with the pinned options, measured `2026-09-30`, differ in that property and nowhere else. Its
//! presence and length are kept; its value is not a fact about the platform.

use std::fmt::Write as _;

const MAGIC: u32 = 0xd00d_feed;
const BEGIN_NODE: u32 = 1;
const END_NODE: u32 = 2;
const PROP: u32 = 3;
const NOP: u32 = 4;
const END: u32 = 9;

/// Properties whose value changes on every boot, as `(node path, property)`.
pub const VOLATILE: &[(&str, &str)] = &[("/chosen", "rng-seed")];

/// One node: its full path, its properties in order, its children in order.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Node {
    pub path: String,
    pub properties: Vec<(String, Vec<u8>)>,
    pub children: Vec<Node>,
}

fn be32(bytes: &[u8], at: usize) -> Result<u32, String> {
    bytes
        .get(at..at + 4)
        .map(|b| u32::from_be_bytes([b[0], b[1], b[2], b[3]]))
        .ok_or_else(|| format!("the blob ends inside a word at byte {at}"))
}

fn cstring(bytes: &[u8], at: usize) -> Result<&str, String> {
    let tail = bytes
        .get(at..)
        .ok_or_else(|| format!("a string starts past the end, at byte {at}"))?;
    let end = tail
        .iter()
        .position(|b| *b == 0)
        .ok_or_else(|| format!("the string at byte {at} is never terminated"))?;
    std::str::from_utf8(&tail[..end]).map_err(|_| format!("the string at byte {at} is not UTF-8"))
}

const fn align4(n: usize) -> usize {
    (n + 3) & !3
}

/// Read a flattened device tree blob.
///
/// # Errors
///
/// The reason the bytes are not a well-formed blob, with the offset that shows it.
pub fn parse(bytes: &[u8]) -> Result<Node, String> {
    if be32(bytes, 0)? != MAGIC {
        return Err("not a flattened device tree: the magic number is wrong".to_string());
    }
    let structure = be32(bytes, 8)? as usize;
    let strings = be32(bytes, 12)? as usize;
    let mut at = structure;
    let mut stack: Vec<Node> = Vec::new();
    let mut root: Option<Node> = None;
    loop {
        let token = be32(bytes, at)?;
        at += 4;
        match token {
            BEGIN_NODE => {
                let name = cstring(bytes, at)?;
                at = align4(at + name.len() + 1);
                let path = match stack.last() {
                    None => "/".to_string(),
                    Some(parent) if parent.path == "/" => format!("/{name}"),
                    Some(parent) => format!("{}/{name}", parent.path),
                };
                stack.push(Node {
                    path,
                    properties: Vec::new(),
                    children: Vec::new(),
                });
            }
            END_NODE => {
                let node = stack
                    .pop()
                    .ok_or_else(|| format!("a node ends that never began, at byte {}", at - 4))?;
                match stack.last_mut() {
                    Some(parent) => parent.children.push(node),
                    None => root = Some(node),
                }
            }
            PROP => {
                let len = be32(bytes, at)? as usize;
                let name = cstring(bytes, strings + be32(bytes, at + 4)? as usize)?.to_string();
                let value = bytes
                    .get(at + 8..at + 8 + len)
                    .ok_or_else(|| format!("property `{name}` runs past the end"))?
                    .to_vec();
                at = align4(at + 8 + len);
                stack
                    .last_mut()
                    .ok_or_else(|| format!("property `{name}` outside any node"))?
                    .properties
                    .push((name, value));
            }
            NOP => {}
            END => break,
            other => return Err(format!("unknown token {other:#x} at byte {}", at - 4)),
        }
    }
    if !stack.is_empty() {
        return Err("the blob ends with a node still open".to_string());
    }
    root.ok_or_else(|| "the blob holds no root node".to_string())
}

/// A property's value as text: a string list when every segment is printable, else 32-bit cells, else bytes.
fn value_text(value: &[u8]) -> String {
    if value.is_empty() {
        return String::new();
    }
    if value.last() == Some(&0) {
        let segments: Vec<&[u8]> = value[..value.len() - 1].split(|b| *b == 0).collect();
        if segments
            .iter()
            .all(|s| !s.is_empty() && s.iter().all(|b| b.is_ascii_graphic() || *b == b' '))
        {
            let quoted: Vec<String> = segments
                .iter()
                .map(|s| format!("\"{}\"", String::from_utf8_lossy(s)))
                .collect();
            return format!(" = {}", quoted.join(", "));
        }
    }
    if value.len().is_multiple_of(4) {
        let cells: Vec<String> = value
            .chunks(4)
            .map(|c| format!("{:#x}", u32::from_be_bytes([c[0], c[1], c[2], c[3]])))
            .collect();
        return format!(" = <{}>", cells.join(" "));
    }
    let bytes: Vec<String> = value.iter().map(|b| format!("{b:02x}")).collect();
    format!(" = [{}]", bytes.join(" "))
}

/// The tree as stable text: each node's path, then its properties indented, in the blob's order.
#[must_use]
pub fn render(root: &Node) -> String {
    let mut out = String::new();
    let mut pending = vec![root];
    while let Some(node) = pending.pop() {
        let _ = writeln!(out, "{}", node.path);
        for (name, value) in &node.properties {
            let masked = VOLATILE
                .iter()
                .any(|(path, property)| *path == node.path && property == name);
            if masked {
                let _ = writeln!(
                    out,
                    "  {name} = <{} bytes, random at every boot>",
                    value.len()
                );
            } else {
                let _ = writeln!(out, "  {name}{}", value_text(value));
            }
        }
        pending.extend(node.children.iter().rev());
    }
    out
}

/// The generated block of a fixture: the lines between its two markers.
///
/// # Errors
///
/// When either marker is missing.
pub fn generated_block(fixture: &str) -> Result<String, String> {
    let begin = "<!-- BEGIN GENERATED: cargo xtask dtb-summary -->";
    let end = "<!-- END GENERATED -->";
    let start = fixture
        .find(begin)
        .ok_or("the fixture has no BEGIN GENERATED marker")?;
    let stop = fixture
        .find(end)
        .ok_or("the fixture has no END GENERATED marker")?;
    let inside = &fixture[start + begin.len()..stop];
    let inside = inside
        .trim_start_matches('\n')
        .trim_start_matches("```text\n")
        .trim_end_matches('\n')
        .trim_end_matches("```")
        .trim_end_matches('\n');
    Ok(format!("{inside}\n"))
}

/// Every difference between a fresh rendering and the fixture's, and every path the fixture's prose names that the
/// rendering does not hold.
#[must_use]
pub fn disagreements(fresh: &str, fixture: &str) -> Vec<String> {
    let mut wrong = Vec::new();
    let recorded = match generated_block(fixture) {
        Ok(block) => block,
        Err(reason) => return vec![reason],
    };
    if recorded != fresh {
        let (a, b): (Vec<&str>, Vec<&str>) = (recorded.lines().collect(), fresh.lines().collect());
        for line in &a {
            if !b.contains(line) {
                wrong.push(format!("recorded, and no longer presented: {line}"));
            }
        }
        for line in &b {
            if !a.contains(line) {
                wrong.push(format!("presented, and not recorded: {line}"));
            }
        }
        if wrong.is_empty() {
            wrong.push("the same lines, in another order".to_string());
        }
    }
    // The prose outside the block names nodes by path; each must be a node the platform presents.
    let paths: Vec<&str> = fresh.lines().filter(|l| l.starts_with('/')).collect();
    for cited in fixture
        .split('`')
        .skip(1)
        .step_by(2)
        .filter(|s| s.starts_with('/') && !s.contains(' '))
    {
        if !paths.contains(&cited) {
            wrong.push(format!(
                "the fixture's prose names `{cited}`, which the platform does not present"
            ));
        }
    }
    wrong
}

#[cfg(test)]
mod tests {
    use super::{disagreements, parse, render, Node};

    /// A blob built by hand: `/` with `#address-cells`, a `/chosen` with a seed, and a `/soc/uart@10` node.
    fn blob(seed: u8, uart: &str) -> Vec<u8> {
        let strings = b"#address-cells\0rng-seed\0compatible\0".to_vec();
        let mut s: Vec<u8> = Vec::new();
        let word = |s: &mut Vec<u8>, w: u32| s.extend_from_slice(&w.to_be_bytes());
        let name = |s: &mut Vec<u8>, n: &str| {
            s.extend_from_slice(n.as_bytes());
            s.push(0);
            while !s.len().is_multiple_of(4) {
                s.push(0);
            }
        };
        let prop = |s: &mut Vec<u8>, off: u32, v: &[u8]| {
            s.extend_from_slice(&3u32.to_be_bytes());
            s.extend_from_slice(&u32::try_from(v.len()).expect("small").to_be_bytes());
            s.extend_from_slice(&off.to_be_bytes());
            s.extend_from_slice(v);
            while !s.len().is_multiple_of(4) {
                s.push(0);
            }
        };
        word(&mut s, 1);
        name(&mut s, "");
        prop(&mut s, 0, &2u32.to_be_bytes());
        word(&mut s, 1);
        name(&mut s, "chosen");
        prop(&mut s, 15, &[seed; 8]);
        word(&mut s, 2);
        word(&mut s, 1);
        name(&mut s, "soc");
        word(&mut s, 1);
        name(&mut s, uart);
        prop(&mut s, 24, b"ns16550a\0");
        word(&mut s, 2);
        word(&mut s, 2);
        word(&mut s, 2);
        word(&mut s, 9);
        let header = 40u32;
        let mut out = Vec::new();
        for w in [
            0xd00d_feed,
            0,
            header,
            header + u32::try_from(s.len()).expect("small"),
            0,
            17,
            16,
            0,
            u32::try_from(strings.len()).expect("small"),
            u32::try_from(s.len()).expect("small"),
        ] {
            out.extend_from_slice(&u32::to_be_bytes(w));
        }
        out.extend_from_slice(&s);
        out.extend_from_slice(&strings);
        out
    }

    #[test]
    fn a_hand_built_blob_reads_back_as_written() {
        let root: Node = parse(&blob(7, "uart@10")).expect("well formed");
        let text = render(&root);
        assert_eq!(
            text,
            "/\n  #address-cells = <0x2>\n/chosen\n  rng-seed = <8 bytes, random at every boot>\n/soc\n/soc/uart@10\n  compatible = \"ns16550a\"\n"
        );
    }

    #[test]
    fn two_boots_that_differ_only_in_the_seed_render_the_same() {
        assert_eq!(
            render(&parse(&blob(1, "uart@10")).expect("ok")),
            render(&parse(&blob(2, "uart@10")).expect("ok"))
        );
    }

    #[test]
    fn a_moved_device_is_a_disagreement_and_names_both_lines() {
        let recorded = render(&parse(&blob(1, "uart@10")).expect("ok"));
        let moved = render(&parse(&blob(1, "uart@20")).expect("ok"));
        let fixture = format!("<!-- BEGIN GENERATED: cargo xtask dtb-summary -->\n```text\n{recorded}```\n<!-- END GENERATED -->\n");
        let wrong = disagreements(&moved, &fixture);
        assert_eq!(wrong.len(), 2, "{wrong:?}");
        assert!(
            wrong[0].contains("no longer presented: /soc/uart@10"),
            "{wrong:?}"
        );
        assert!(wrong[1].contains("not recorded: /soc/uart@20"), "{wrong:?}");
        assert!(disagreements(&recorded, &fixture).is_empty());
    }

    #[test]
    fn prose_naming_a_node_the_platform_lacks_is_a_disagreement() {
        let recorded = render(&parse(&blob(1, "uart@10")).expect("ok"));
        let fixture = format!("The console is `/soc/uart@99`.\n<!-- BEGIN GENERATED: cargo xtask dtb-summary -->\n```text\n{recorded}```\n<!-- END GENERATED -->\n");
        let wrong = disagreements(&recorded, &fixture);
        assert_eq!(wrong.len(), 1, "{wrong:?}");
        assert!(wrong[0].contains("`/soc/uart@99`"), "{wrong:?}");
    }

    #[test]
    fn a_fixture_without_its_markers_is_refused_not_passed() {
        let recorded = render(&parse(&blob(1, "uart@10")).expect("ok"));
        assert_eq!(disagreements(&recorded, "no markers here").len(), 1);
    }

    #[test]
    fn the_kept_dump_renders_to_exactly_the_fixture() {
        // Offline, so it runs everywhere: the measured dump beside the fixture, rendered now, against the block the
        // fixture carries. The emulator step does the same with a fresh dump where QEMU is installed.
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask/ sits at the repository root");
        let dump =
            std::fs::read(root.join("docs/targets/riscv-virt-up.dtb")).expect("the kept dump");
        let fixture =
            std::fs::read_to_string(root.join("docs/targets/riscv-virt-up.dtb.summary.md"))
                .expect("the fixture");
        let wrong = disagreements(&render(&parse(&dump).expect("a device tree")), &fixture);
        assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    }

    #[test]
    fn a_blob_that_is_not_one_is_refused_with_a_reason() {
        assert!(parse(b"not a device tree at all")
            .unwrap_err()
            .contains("magic"));
        let mut truncated = blob(1, "uart@10");
        truncated.truncate(60);
        assert!(parse(&truncated).is_err());
    }
}
