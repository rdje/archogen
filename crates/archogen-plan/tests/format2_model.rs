//! Format `/2`'s decision procedures as an executable model (`docs/decisions/decision_runtime-plan-format-2.md`;
//! `docs/decisions/decision_executable-design-reviews.md`). Nothing of `/2` is code yet. Three rounds of its review worked its formulas by hand, and each
//! found one wrong, the record's own worked example among them. So each procedure the record states is transcribed here, rule by rule and citing its
//! section, optimised for nothing and on `std` alone; a bounded exhaustive checker holds what the record claims of each over every small input; and each
//! probe of a round is a row of a corpus with the outcome the record gives it. `M4.1.7`'s lowering is to be held against this over the same universe.
//!
//! What is modelled: §1's code identity, by the record's own table of lines; §4's epoch and the reach it asks of one write; §5's floor, limit and
//! cursor, and each stack's sum; §6's priorities. What is not: the text of a plan's lines, which the lowering's goldens will hold (`M4.1.7`).

use std::collections::BTreeMap;

const RECORD: &str = include_str!("../../../docs/decisions/decision_runtime-plan-format-2.md");

// ---- §5: memory

/// A placement region, `[base, end)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Region {
    base: u64,
    end: u64,
}

/// What the record refuses of a region's memory, or of a unit in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Refused {
    /// §5: "Holes covering the region from its base to its end are `MemoryAllHoles`."
    MemoryAllHoles,
    /// §5: a hole with no size "that so leaves no byte free is `HoleUnsized`".
    HoleUnsized,
    /// §5: "if base < F: refuse, 'no room'", naming the unit by its place in the order.
    NoRoom(usize),
}

/// Each hole as the bytes it takes of the region, `[base, end)`: one of no size reaches the region's end (§5, "A hole with no size"; model §8 rule
/// 10), and one that takes none of the region is none of its holes. The second of a pair says whether it had a size.
fn holes_in(region: Region, holes: &[(u64, Option<u64>)]) -> Vec<(u64, u64, bool)> {
    holes
        .iter()
        .map(|&(base, size)| {
            (
                base,
                size.map_or(region.end, |size| base + size),
                size.is_some(),
            )
        })
        .filter(|&(base, end, _)| base < region.end && end > region.base && base < end)
        .map(|(base, end, sized)| (base.max(region.base), end.min(region.end), sized))
        .collect()
}

/// §5, "The image lies at the bottom": "`F` is the least address at or above the placement region's base that no hole covers: the base, moved
/// past each hole that covers it, one after another". Refused where that leaves no byte.
fn floor(region: Region, holes: &[(u64, u64, bool)]) -> Result<u64, Refused> {
    let (mut floor, mut by_one_unsized) = (region.base, false);
    while let Some(&(_, end, sized)) = holes
        .iter()
        .find(|&&(base, end, _)| base <= floor && floor < end)
    {
        floor = end;
        by_one_unsized |= !sized;
    }
    match (floor >= region.end, by_one_unsized) {
        (false, _) => Ok(floor),
        (true, false) => Err(Refused::MemoryAllHoles),
        (true, true) => Err(Refused::HoleUnsized),
    }
}

/// A unit to lay: a stack or a table, its bytes, its guard and its alignment (§5, "The formula").
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Unit {
    stack: bool,
    bytes: u64,
    guard: u64,
    align: u64,
}

/// Where the formula tries a unit with the cursor at `cursor`: `[base, top)`, or nothing where it would lie below address nothing.
/// "top := a stack's: cursor rounded down to A; a table's: cursor. base := top - bytes - guard, a table's rounded down to A."
fn tried(unit: Unit, cursor: u64) -> Option<(u64, u64)> {
    let top = if unit.stack {
        cursor - cursor % unit.align
    } else {
        cursor
    };
    let base = top.checked_sub(unit.bytes + unit.guard)?;
    Some((
        if unit.stack {
            base
        } else {
            base - base % unit.align
        },
        top,
    ))
}

/// Which hole the formula takes where a unit meets several: the record says "that hole's base", so the layout must not hang on the choice.
type Pick = fn(&[(u64, u64)]) -> (u64, u64);

/// §5, "The formula", as written: the cursor starts at the region's end and only descends; a unit that meets a hole is laid again with the
/// cursor at that hole's base, `pick` choosing the hole where it meets several; one whose base is below `F` is refused. An owed unit, one
/// with no figure, makes every later unit owed, so the units laid are those before the first owed one.
fn lay(
    region: Region,
    holes: &[(u64, u64, bool)],
    units: &[Option<Unit>],
    floor: u64,
    pick: Pick,
) -> Result<Vec<(u64, u64)>, Refused> {
    let (mut cursor, mut laid) = (region.end, Vec::new());
    for (place, unit) in units.iter().map_while(|unit| *unit).enumerate() {
        let (base, top) = loop {
            let Some((base, top)) = tried(unit, cursor) else {
                return Err(Refused::NoRoom(place));
            };
            let met: Vec<(u64, u64)> = holes
                .iter()
                .filter(|&&(hole, end, _)| hole < top && end > base)
                .map(|&(hole, end, _)| (hole, end))
                .collect();
            if met.is_empty() {
                break (base, top);
            }
            cursor = pick(&met).0;
        };
        if base < floor {
            return Err(Refused::NoRoom(place));
        }
        laid.push((base, top));
        cursor = base;
    }
    Ok(laid)
}

/// What the formula is for, one address at a time: each unit at the highest place at or below the cursor where it meets no hole.
fn reference(
    region: Region,
    holes: &[(u64, u64, bool)],
    units: &[Option<Unit>],
    floor: u64,
) -> Result<Vec<(u64, u64)>, Refused> {
    let (mut cursor, mut laid) = (region.end, Vec::new());
    for (place, unit) in units.iter().map_while(|unit| *unit).enumerate() {
        let mut at = cursor;
        let (base, top) = loop {
            match tried(unit, at) {
                Some((base, top)) if base >= floor => {
                    if holes
                        .iter()
                        .all(|&(hole, end, _)| !(hole < top && end > base))
                    {
                        break (base, top);
                    }
                    at -= 1;
                }
                _ => return Err(Refused::NoRoom(place)),
            }
        };
        laid.push((base, top));
        cursor = base;
    }
    Ok(laid)
}

/// §5: "`L` is the less of the lowest allocation's base and the base of the lowest hole above `F`", "`owed` while the lowest allocation is, a
/// hole's base then still naming a limit in `truncated-by`", and "`truncated-by` names each hole whose base is `L`". The limit, nothing where it
/// is owed, and the bases `truncated-by` names.
fn limit(
    region: Region,
    holes: &[(u64, u64, bool)],
    floor: u64,
    laid: &[(u64, u64)],
    an_owed_unit: bool,
) -> (Option<u64>, Vec<u64>) {
    let above = holes
        .iter()
        .map(|&(base, _, _)| base)
        .filter(|&base| base > floor)
        .min();
    let lowest = laid.last().map_or(region.end, |&(base, _)| base);
    let limit = above.map_or(lowest, |above| above.min(lowest));
    let names = |limit: u64| {
        holes
            .iter()
            .map(|&(base, _, _)| base)
            .filter(|&base| base == limit)
            .collect()
    };
    match (an_owed_unit, above) {
        (true, Some(above)) => (None, names(above)),
        (true, None) => (None, Vec::new()),
        (false, _) => (Some(limit), names(limit)),
    }
}

/// What the record claims of a layout, each a sentence of §5: a unit lies at or above `F` and below the one before it, meets no hole, has its
/// alignment, and the image's range `[F, L)` holds no hole and no unit.
fn holds(
    region: Region,
    holes: &[(u64, u64, bool)],
    units: &[Option<Unit>],
    floor: u64,
    laid: &[(u64, u64)],
) {
    assert!(
        holes
            .iter()
            .all(|&(base, end, _)| !(base <= floor && floor < end)),
        "F lies in a hole"
    );
    let mut last = region.end;
    for (&(base, top), unit) in laid.iter().zip(units.iter().flatten()) {
        assert!(
            floor <= base && top <= last,
            "a unit past the floor or the one before it"
        );
        assert!(
            holes
                .iter()
                .all(|&(hole, end, _)| !(hole < top && end > base)),
            "a unit meets a hole"
        );
        if unit.stack {
            assert!(
                top % unit.align == 0 && top - base == unit.bytes + unit.guard,
                "a stack's top or its size"
            );
        } else {
            assert!(
                base % unit.align == 0 && base + unit.bytes + unit.guard <= top,
                "a table's base or its size"
            );
        }
        last = base;
    }
    let owed = units.iter().any(Option::is_none);
    if let (Some(limit), _) = limit(region, holes, floor, laid, owed) {
        assert!(floor <= limit, "the image's range is none");
        assert!(
            holes
                .iter()
                .all(|&(hole, end, _)| !(hole < limit && end > floor)),
            "the image's range meets a hole"
        );
        assert!(
            laid.iter().all(|&(base, _)| base >= limit),
            "the image's range meets a unit"
        );
    }
}

/// Every region of up to 6 bytes at two bases, with up to two holes of each size and of none, and up to two units of each shape: the formula
/// lays each as the reference does whichever hole it takes where a unit meets several, and each layout holds what §5 claims.
#[test]
fn every_small_region_is_laid_as_the_reference_lays_it() {
    let shapes: Vec<Unit> = [true, false]
        .into_iter()
        .flat_map(|stack| {
            [1, 2].into_iter().flat_map(move |bytes| {
                [0, 1].into_iter().flat_map(move |guard| {
                    [1, 2, 4].into_iter().map(move |align| Unit {
                        stack,
                        bytes,
                        guard,
                        align,
                    })
                })
            })
        })
        .filter(|unit| unit.stack || unit.guard == 0)
        .collect();
    let mut orders: Vec<Vec<Option<Unit>>> = vec![Vec::new()];
    for first in &shapes {
        orders.push(vec![Some(*first)]);
        for second in &shapes {
            orders.push(vec![Some(*first), Some(*second)]);
        }
        orders.push(vec![None, Some(*first)]);
        orders.push(vec![Some(*first), None]);
    }
    let picks: [Pick; 2] = [
        |met| *met.iter().min().expect("one"),
        |met| *met.iter().max().expect("one"),
    ];
    let (mut regions, mut refused, mut unsized_holes) = (0_u64, 0_u64, 0_u64);
    for base in [0, 3] {
        for size in 1..=6 {
            let region = Region {
                base,
                end: base + size,
            };
            let mut each: Vec<(u64, Option<u64>)> = Vec::new();
            for at in base.saturating_sub(1)..=region.end {
                each.extend([None, Some(1), Some(2)].into_iter().map(|size| (at, size)));
            }
            let mut sets: Vec<Vec<(u64, Option<u64>)>> = vec![Vec::new()];
            for (place, first) in each.iter().enumerate() {
                sets.push(vec![*first]);
                sets.extend(each[place..].iter().map(|second| vec![*first, *second]));
            }
            for raw in &sets {
                let holes = holes_in(region, raw);
                let Ok(floor) = floor(region, &holes) else {
                    continue;
                };
                for units in &orders {
                    let want = reference(region, &holes, units, floor);
                    for pick in picks {
                        assert_eq!(
                            lay(region, &holes, units, floor, pick),
                            want,
                            "{region:?} {raw:?} {units:?}"
                        );
                    }
                    regions += 1;
                    unsized_holes += u64::from(raw.iter().any(|(_, size)| size.is_none()));
                    match &want {
                        Ok(laid) => holds(region, &holes, units, floor, laid),
                        Err(_) => refused += 1,
                    }
                }
            }
        }
    }
    assert_eq!(
        (regions, refused, unsized_holes),
        (535_148, 390_698, 228_916)
    );
}

/// The rounds' probes of §5's memory, each with the outcome the record gives it.
#[test]
fn the_reviews_probes_of_memory_come_out_as_the_record_says() {
    const KIB: u64 = 1 << 10;
    const MIB: u64 = 1 << 20;
    let ram = Region {
        base: 0x8000_0000,
        end: 0x8000_0000 + 128 * MIB,
    };
    let floor_of = |holes: &[(u64, Option<u64>)]| floor(ram, &holes_in(ram, holes));
    // RP2-1: two holes at the base, touching and overlapping. RP3-11: the second from `0x8000_8000`, of 64 KiB and of 96.
    assert_eq!(
        floor_of(&[(0x8000_0000, Some(64 * KIB)), (0x8001_0000, Some(64 * KIB))]),
        Ok(0x8002_0000)
    );
    assert_eq!(
        floor_of(&[(0x8000_0000, Some(64 * KIB)), (0x8000_8000, Some(64 * KIB))]),
        Ok(0x8001_8000)
    );
    assert_eq!(
        floor_of(&[(0x8000_0000, Some(64 * KIB)), (0x8000_8000, Some(96 * KIB))]),
        Ok(0x8002_0000)
    );
    assert_eq!(floor_of(&[]), Ok(0x8000_0000));
    assert_eq!(
        floor_of(&[(0x8000_0000, Some(128 * MIB))]),
        Err(Refused::MemoryAllHoles)
    );
    assert_eq!(
        floor_of(&[(0x8000_0000, Some(64 * KIB)), (0x8001_0000, None)]),
        Err(Refused::HoleUnsized)
    );
    // RP2-12: a hole 16 bytes above the floor leaves the image 16 bytes, however much lies free above it.
    let holes = holes_in(ram, &[(0x8000_0010, Some(16))]);
    let stack = Some(Unit {
        stack: true,
        bytes: 4096,
        guard: 64,
        align: 16,
    });
    let laid = lay(ram, &holes, &[stack], 0x8000_0000, |met| met[0]).expect("it fits");
    assert_eq!(
        limit(ram, &holes, 0x8000_0000, &laid, false),
        (Some(0x8000_0010), vec![0x8000_0010])
    );
    // An owed unit makes every later one owed, and the limit with them: a hole above the floor still names where the image must stop.
    let laid = lay(ram, &holes, &[stack, None, stack], 0x8000_0000, |met| {
        met[0]
    })
    .expect("the first is laid");
    assert_eq!(laid.len(), 1);
    assert_eq!(
        limit(ram, &holes, 0x8000_0000, &laid, true),
        (None, vec![0x8000_0010])
    );
    // A hole of no size takes the region's top: the first unit lies below it.
    let holes = holes_in(ram, &[(0x8700_0000, None)]);
    let laid = lay(ram, &holes, &[stack], 0x8000_0000, |met| met[0]).expect("it fits");
    assert_eq!(laid, [(0x8700_0000 - 4096 - 64, 0x8700_0000)]);
    // No room names the unit: a region of 4 KiB cannot hold that stack.
    let small = Region {
        base: 0x1000,
        end: 0x2000,
    };
    assert_eq!(
        lay(small, &[], &[stack], 0x1000, |met| met[0]),
        Err(Refused::NoRoom(0))
    );
}

// ---- §5: the stacks

/// A stack's figures, by the names §5's table of figures gives them. A name of several figures, each node's or each route's, holds each of them.
type Figures = BTreeMap<&'static str, Vec<u64>>;

/// A cell of §5's table of sums, read as it is written: sums of names and `max(…, …)`. A name of several figures is the largest of them, and nothing where there is none
/// (§5, "A name that is several figures"); `fault-use` is "the `fault-entry-stack` plus the `fault-path-stack`".
struct Cell<'c> {
    rest: &'c str,
    figures: &'c Figures,
    named: Vec<String>,
}

impl Cell<'_> {
    fn eat(&mut self, what: &str) -> bool {
        match self.rest.trim_start().strip_prefix(what) {
            Some(rest) => {
                self.rest = rest;
                true
            }
            None => false,
        }
    }

    fn sum(&mut self) -> u64 {
        let mut total = self.term();
        while self.eat("+") {
            total += self.term();
        }
        total
    }

    fn term(&mut self) -> u64 {
        if self.eat("max(") {
            let mut most = self.sum();
            while self.eat(",") {
                most = most.max(self.sum());
            }
            assert!(self.eat(")"), "a `max` left open at `{}`", self.rest);
            return most;
        }
        let rest = self.rest.trim_start();
        let end = rest
            .find(|c: char| !(c.is_ascii_lowercase() || c == '-'))
            .unwrap_or(rest.len());
        let (name, after) = rest.split_at(end);
        assert!(!name.is_empty(), "no figure at `{rest}`");
        self.rest = after;
        self.named.push(name.to_owned());
        self.value(name)
    }

    fn value(&self, name: &str) -> u64 {
        if name == "fault-use" {
            return self.value("fault-entry-stack") + self.value("fault-path-stack");
        }
        let figures = self.figures.get(name).unwrap_or_else(|| {
            panic!("a sum names `{name}`, which this stack was given no figure of")
        });
        figures.iter().max().copied().unwrap_or(0)
    }
}

/// The cell §5's table of sums writes for `unit`, after `<!-- machine-read: stack-sums -->`.
fn cell(unit: &str) -> &'static str {
    let (_, table) = RECORD
        .split_once("<!-- machine-read: stack-sums -->")
        .expect("the record's table of sums");
    let row = table
        .lines()
        .find(|line| line.starts_with(&format!("| {unit} |")))
        .unwrap_or_else(|| panic!("no row for {unit}"));
    row.split('|')
        .map(str::trim)
        .nth(2)
        .expect("its cell")
        .trim_matches('`')
}

/// A unit's `bytes`, as its cell is written, and each figure the cell names.
fn bytes(unit: &str, figures: &Figures) -> (u64, Vec<String>) {
    let mut cell = Cell {
        rest: cell(unit),
        figures,
        named: Vec::new(),
    };
    let total = cell.sum();
    assert!(
        cell.rest.trim().is_empty(),
        "the cell of {unit} goes on: `{}`",
        cell.rest
    );
    (total, cell.named)
}

/// What runs on a stack: code of a depth of its own, and what it calls, each in turn.
struct Runs {
    own: u64,
    calls: Vec<Runs>,
}

/// The most of it that stands at once: its own, and the deepest of what it calls.
fn deepest(runs: &Runs) -> u64 {
    runs.own + runs.calls.iter().map(deepest).max().unwrap_or(0)
}

/// What can stand on each stack at once, by who calls whom there (§5's table of figures, "What it bounds"), a fault arriving on top of the deepest of it, and one trap frame
/// (§5, "A fault is added, and never taken among the largest").
fn standing(unit: &str, figures: &Figures) -> u64 {
    let one = |name: &str| figures[name].first().copied().unwrap_or(0);
    let each = |names: &[&str]| -> Vec<Runs> {
        names
            .iter()
            .flat_map(|name| {
                figures[*name].iter().map(|&own| Runs {
                    own,
                    calls: Vec::new(),
                })
            })
            .collect()
    };
    let on_it = match unit {
        // Generated code starts a job, whose own code calls a node, a primitive or the completion path.
        "a task's" => Runs {
            own: one("call-frame"),
            calls: vec![Runs {
                own: one("stack-bound"),
                calls: each(&["node-stack", "primitive-stack", "completion-stack"]),
            }],
        },
        // The trap path calls the dispatch, which calls one service.
        "the interrupt's" => Runs {
            own: one("trap-path-stack"),
            calls: vec![Runs {
                own: one("call-frame"),
                calls: each(&["service-stack", "timer-service-stack"]),
            }],
        },
        // The boot's code calls each step, the catalog's own initialisation, the application's, which calls nodes, and its last act, which calls the transition, the idle context following.
        "the boot's" => {
            let mut calls = each(&["initialisation-stack", "initialisation-path-stack"]);
            calls.extend(figures["stack-bound"].iter().map(|&own| Runs {
                own,
                calls: each(&["node-stack"]),
            }));
            calls.push(Runs {
                own: one("last-act-stack"),
                calls: each(&["transition-stack", "idle-stack"]),
            });
            Runs {
                own: one("call-frame"),
                calls,
            }
        }
        other => panic!("no stack is {other}"),
    };
    deepest(&on_it) + one("fault-entry-stack") + one("fault-path-stack") + one("trap-frame")
}

/// Every set of small figures for a stack: each figure of one value in turn, and each of several as none, one or two.
fn every(
    singles: &[&'static str],
    several: &[&'static str],
    optional: &[&'static str],
) -> Vec<Figures> {
    let mut sets: Vec<Figures> = vec![BTreeMap::new()];
    let mut widen = |name: &'static str, values: &[Vec<u64>]| {
        sets = sets
            .iter()
            .flat_map(|set| {
                values.iter().map(move |value| {
                    set.iter()
                        .map(|(k, v)| (*k, v.clone()))
                        .chain([(name, value.clone())])
                        .collect()
                })
            })
            .collect();
    };
    for name in singles {
        widen(name, &[vec![0], vec![1], vec![3]]);
    }
    for name in several {
        widen(name, &[vec![], vec![0], vec![2], vec![1, 2], vec![2, 1]]);
    }
    for name in optional {
        widen(name, &[vec![], vec![0], vec![2]]);
    }
    for name in ["fault-entry-stack", "fault-path-stack", "trap-frame"] {
        widen(name, &[vec![0], vec![1]]);
    }
    sets
}

/// A unit of §5's table of sums: its name there, its figures of one value, those of several, and those it may have none of.
type Stack = (
    &'static str,
    &'static [&'static str],
    &'static [&'static str],
    &'static [&'static str],
);

/// The units of §5's table of sums, each with its figures: of one value, of several, and one a stack may have none of, the application's initialisation.
const UNITS: &[Stack] = &[
    (
        "a task's",
        &["call-frame", "stack-bound", "completion-stack"],
        &["node-stack", "primitive-stack"],
        &[],
    ),
    (
        "the interrupt's",
        &["trap-path-stack", "call-frame", "timer-service-stack"],
        &["service-stack"],
        &[],
    ),
    (
        "the boot's",
        &[
            "call-frame",
            "initialisation-path-stack",
            "last-act-stack",
            "transition-stack",
            "idle-stack",
        ],
        &["initialisation-stack", "node-stack"],
        &["stack-bound"],
    ),
];

/// Each stack's sum, as the record's cell writes it, is the most that can stand on that stack and no more, over every small set of figures.
#[test]
fn each_stack_s_sum_is_the_most_that_can_stand_on_it() {
    let mut run = 0_u64;
    for (unit, singles, several, optional) in UNITS {
        for figures in every(singles, several, optional) {
            // With no initialisation of the application's there is nothing it calls: the boot's `node-stack` figures are those its `node-calls` name.
            if optional.iter().any(|name| figures[*name].is_empty())
                && !figures["node-stack"].is_empty()
            {
                continue;
            }
            assert_eq!(
                bytes(unit, &figures).0,
                standing(unit, &figures),
                "{unit}: {figures:?}"
            );
            run += 1;
        }
    }
    assert_eq!(run, 5_400 + 1_080 + 106_920);
}

/// The record's table of figures after `<!-- machine-read: stack-figures -->`: each figure and its owner.
fn the_figures() -> Vec<(String, String)> {
    let (_, table) = RECORD
        .split_once("<!-- machine-read: stack-figures -->")
        .expect("the record's table of figures");
    table
        .lines()
        .skip_while(|line| !line.starts_with('|'))
        .take_while(|line| line.starts_with('|'))
        .skip(2)
        .map(|row| {
            let cells: Vec<&str> = row.trim_matches('|').split('|').map(str::trim).collect();
            (cells[0].trim_matches('`').to_owned(), cells[2].to_owned())
        })
        .collect()
}

/// A sum names no figure the table of figures does not bound, the table bounds none no sum names, each has one of three owners, and each the application does not own has
/// its owed fact among those `(owed …)` gains: so a figure cannot enter a sum that nobody is asked for.
#[test]
fn each_figure_of_a_sum_is_bounded_owned_and_owed() {
    let figures = the_figures();
    let mut named: Vec<String> = Vec::new();
    for (unit, singles, several, optional) in UNITS {
        let given: Figures = singles
            .iter()
            .chain(*several)
            .chain(*optional)
            .chain(&["fault-entry-stack", "fault-path-stack", "trap-frame"])
            .map(|name| (*name, vec![1]))
            .collect();
        let (_, in_its_cell) = bytes(unit, &given);
        // What a unit is given here is what its cell names, so the checker above varies every figure of a sum and no other.
        let mut wanted: Vec<&str> = in_its_cell
            .iter()
            .map(String::as_str)
            .filter(|name| *name != "fault-use")
            .chain(["fault-entry-stack", "fault-path-stack"])
            .collect();
        let mut had: Vec<&str> = given.keys().copied().collect();
        wanted.sort_unstable();
        wanted.dedup();
        had.sort_unstable();
        assert_eq!(wanted, had, "{unit}");
        named.extend(wanted.into_iter().map(str::to_owned));
    }
    named.sort();
    named.dedup();
    let mut bounded: Vec<String> = figures.iter().map(|(figure, _)| figure.clone()).collect();
    bounded.sort();
    assert_eq!(named, bounded, "the sums' figures and the table's differ");
    let owed = RECORD
        .split("**`(owed …)`, as `/2` writes it.**")
        .nth(1)
        .expect("what `/2` owes");
    let owed = &owed[..owed.find("\n\n").expect("its end")];
    for (figure, owner) in &figures {
        assert!(
            ["the application", "the realization", "the catalog"].contains(&owner.as_str()),
            "`{figure}` is owned by {owner}"
        );
        assert!(
            owner == "the application" || owed.contains(&format!("`{figure}`")),
            "nothing owes `{figure}`"
        );
    }
}

/// The rounds' probes of the stacks, figures chosen for each sum and stated by no record.
#[test]
fn the_reviews_probes_of_the_stacks_come_out_as_the_record_says() {
    let of = |unit: &str, given: &[(&'static str, &[u64])]| {
        let figures: Figures = given
            .iter()
            .map(|(name, values)| (*name, values.to_vec()))
            .collect();
        assert_eq!(bytes(unit, &figures).0, standing(unit, &figures));
        bytes(unit, &figures).0
    };
    // RP2-2: a task of 512, a node's 380, a primitive's 96, a completion's 64, a fault's 400 and a frame of 272; and §5's worked sum, under a start of 48.
    let task = |start: u64, frame: u64| {
        of(
            "a task's",
            &[
                ("call-frame", &[start]),
                ("stack-bound", &[512]),
                ("node-stack", &[380]),
                ("primitive-stack", &[96]),
                ("completion-stack", &[64]),
                ("fault-entry-stack", &[100]),
                ("fault-path-stack", &[300]),
                ("trap-frame", &[frame]),
            ],
        )
    };
    assert_eq!(task(0, 272), 1564);
    assert_eq!(task(48, 272), 1612);
    // Taking the fault among the largest gave 1184, and 1232 under that start: under what a check failing inside that node needs with no frame at all.
    assert_eq!(
        512 + [380, 96, 64, 400].into_iter().max().expect("one") + 272,
        1184
    );
    assert_eq!(
        48 + 512 + [380, 96, 64, 400].into_iter().max().expect("one") + 272,
        1232
    );
    assert_eq!((task(0, 0), task(48, 0)), (1292, 1340));
    // RP3-1: an initialisation of 256 at its bound calls a driver using 300 where a check fails, the idle context 64. The bound taken bare gave 928, and 976 under a start of 48.
    let boot = |start: u64, frame: u64, driver: &[u64]| {
        of(
            "the boot's",
            &[
                ("call-frame", &[start]),
                ("initialisation-stack", &[]),
                ("initialisation-path-stack", &[0]),
                ("stack-bound", &[256]),
                ("node-stack", driver),
                ("last-act-stack", &[0]),
                ("transition-stack", &[0]),
                ("idle-stack", &[64]),
                ("fault-entry-stack", &[100]),
                ("fault-path-stack", &[300]),
                ("trap-frame", &[frame]),
            ],
        )
    };
    assert_eq!((boot(0, 272, &[]), boot(48, 272, &[])), (928, 976));
    assert_eq!((boot(0, 0, &[300]), boot(48, 0, &[300])), (956, 1004));
    assert_eq!(boot(48, 272, &[300]), 1276);
    // With no application's initialisation the boot's stack holds the steps, the catalog's own, and the last act under the transition or the idle context.
    let bare = of(
        "the boot's",
        &[
            ("call-frame", &[48]),
            ("initialisation-stack", &[128, 300]),
            ("initialisation-path-stack", &[200]),
            ("stack-bound", &[]),
            ("node-stack", &[]),
            ("last-act-stack", &[80]),
            ("transition-stack", &[96]),
            ("idle-stack", &[64]),
            ("fault-entry-stack", &[100]),
            ("fault-path-stack", &[300]),
            ("trap-frame", &[272]),
        ],
    );
    assert_eq!(bare, 48 + 300 + 400 + 272);
    // The trap path stands under the dispatch and the service it calls: a service of 200 and the timer's of 260 under a path of 120 and a dispatch of 40.
    let interrupt = of(
        "the interrupt's",
        &[
            ("trap-path-stack", &[120]),
            ("call-frame", &[40]),
            ("service-stack", &[200, 90]),
            ("timer-service-stack", &[260]),
            ("fault-entry-stack", &[100]),
            ("fault-path-stack", &[300]),
            ("trap-frame", &[272]),
        ],
    );
    assert_eq!(interrupt, 120 + 40 + 260 + 400 + 272);
}

// ---- §6: the controller

/// §6, "Ranks onto levels": "Rank 1 takes the highest level, and each rank after it the next", "`PRIORITY` is `controller-levels` less the
/// number of such routes ahead of it", and "Where the routes outnumber the levels the system is refused".
fn priority(levels: u64, routes: u64, rank: u64) -> Option<u64> {
    (routes <= levels && (1..=routes).contains(&rank)).then(|| levels - (rank - 1))
}

/// Over every controller of up to 8 levels: each route has a level of its own, a higher rank a higher level, the lowest above the threshold.
#[test]
fn each_rank_has_a_level_of_its_own_in_the_plan_s_order() {
    for levels in 1..=8 {
        for routes in 0..=9 {
            let given: Vec<Option<u64>> = (1..=routes)
                .map(|rank| priority(levels, routes, rank))
                .collect();
            if routes > levels {
                assert!(given.iter().all(Option::is_none), "{levels} {routes}");
                continue;
            }
            let given: Vec<u64> = given
                .into_iter()
                .map(|level| level.expect("a level"))
                .collect();
            assert!(
                given.windows(2).all(|pair| pair[0] == pair[1] + 1),
                "{given:?}"
            );
            assert!(
                given.iter().all(|&level| (1..=levels).contains(&level)),
                "{given:?}"
            );
            assert_eq!(given.first().copied(), (routes > 0).then_some(levels));
        }
    }
}

// ---- §4: time on a machine

/// §4, "The epoch": the last act of initialisation reads the counter at `r` and counts releases from `e = r + E`, `E` the port's `epoch-lead`:
/// "one tick of the counter, a read being up to that old, the first compare's write, and the transition". In units of time finer than a tick:
/// the read's count is the tick at or before the instant it is taken.
fn epoch(read_at: u64, tick: u64, lead: u64) -> u64 {
    read_at - read_at % tick + lead
}

/// Over every phase of a read within its tick, and every write and transition up to their bounds: the epoch is at or after the first enabling,
/// and by no more than the lead. And the transition's bound alone does not do, which round 2 found.
#[test]
fn the_epoch_is_at_or_after_the_first_enabling_at_every_phase_of_its_read() {
    let mut the_transition_alone_fails = false;
    for tick in 1..=6 {
        for write_bound in 0..=4 {
            for transition_bound in 0..=4 {
                let lead = tick + write_bound + transition_bound;
                for read_at in 0..3 * tick {
                    for write in 0..=write_bound {
                        for transition in 0..=transition_bound {
                            let enabling = read_at + write + transition;
                            let e = epoch(read_at, tick, lead);
                            assert!(
                                enabling <= e && e - enabling <= lead,
                                "{tick} {read_at} {write} {transition}"
                            );
                            the_transition_alone_fails |=
                                epoch(read_at, tick, transition_bound) < enabling;
                        }
                    }
                }
            }
        }
    }
    assert!(the_transition_alone_fails);
    // RP2-7's case, in hundredths of a microsecond: 10 MHz, a read of 1000 taken at 100.09 µs, a write of 0.30 and a transition of 2.00.
    assert!(epoch(10_009, 10, 200) < 10_009 + 30 + 200);
    assert!(epoch(10_009, 10, 10 + 30 + 200) >= 10_009 + 30 + 200);
}

/// §4, "One write reaches the first compare": the first compare is written at the ceiling of `e` on the compare's step, so it stands at most
/// `E + ρ` past the read, which one write must reach, `H`. Whether a set is admitted for it, by the figures a plan's line and the catalog give.
fn the_first_compare_is_within_reach(lead: u64, step: u64, reach: u64) -> bool {
    lead + step <= reach
}

/// Over every small tick, lead and step: the first compare, at the ceiling of the epoch on its step, stands no further past its read than the
/// condition holds within one write. And RP3-3's case is refused: a reach of 2 ms under a lead of 3 ms.
#[test]
fn the_first_compare_stands_within_one_write_where_the_condition_holds() {
    for tick in 1..=4 {
        for lead in tick..=3 * tick + 2 {
            for step in 1..=3 {
                for read_at in 0..2 * tick {
                    let read = read_at - read_at % tick;
                    let e = epoch(read_at, tick, lead);
                    let compare = e.div_ceil(step * tick) * step * tick;
                    assert!(
                        compare - read < lead + step * tick,
                        "{tick} {lead} {step} {read_at}"
                    );
                    for reach in 0..=lead + step * tick + 1 {
                        if the_first_compare_is_within_reach(lead, step * tick, reach) {
                            assert!(compare - read <= reach);
                        }
                    }
                }
            }
        }
    }
    assert!(!the_first_compare_is_within_reach(3_000, 1, 2_000));
    assert!(the_first_compare_is_within_reach(1_000, 1, 2_000));
}

// ---- §1: the code identity

/// What the code identity does with a line of a plan, by the line's head (§1's table).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Identity {
    /// The line stands in the preimage as written.
    Kept,
    /// The line is left out: a figure, where one came from, or what is said of the code and is no code.
    LeftOut,
    /// The line is written with its entry point alone, or as `none`.
    EntryPoint,
}

/// §1's table, as this model holds it: each head a line of `/2` can begin with, and what the identity does with it.
const LINES: &[(&str, Identity)] = &[
    ("format", Identity::Kept),
    ("profile", Identity::Kept),
    ("system", Identity::Kept),
    ("target", Identity::Kept),
    ("program", Identity::Kept),
    ("assignment", Identity::Kept),
    ("adapters", Identity::Kept),
    ("catalog", Identity::Kept),
    ("application", Identity::LeftOut),
    ("realization", Identity::LeftOut),
    ("code", Identity::Kept),
    ("record", Identity::LeftOut),
    ("task", Identity::Kept),
    ("task-inputs", Identity::EntryPoint),
    ("declared", Identity::LeftOut),
    ("initialisation-inputs", Identity::EntryPoint),
    ("realization-figure", Identity::LeftOut),
    ("realization-fact", Identity::LeftOut),
    ("bind", Identity::Kept),
    ("instance", Identity::Kept),
    ("epoch", Identity::Kept),
    ("timer", Identity::Kept),
    ("alarm", Identity::Kept),
    ("route", Identity::Kept),
    ("enabled", Identity::Kept),
    ("interrupt-model", Identity::Kept),
    ("region", Identity::Kept),
    ("placement", Identity::Kept),
    ("allocation", Identity::LeftOut),
    ("image", Identity::LeftOut),
    ("node", Identity::Kept),
    ("context", Identity::Kept),
    ("depends", Identity::Kept),
    ("step", Identity::Kept),
    ("obligation", Identity::Kept),
    ("owed", Identity::LeftOut),
    ("complete", Identity::LeftOut),
];

/// A line's head: the name after its opening parenthesis.
fn head(line: &str) -> &str {
    let rest = line.strip_prefix('(').unwrap_or(line);
    rest.split([' ', ')']).next().unwrap_or(rest)
}

/// §1, "The code identity": its preimage, "the plan's canonical text, each line ending in a line feed", each line kept, left out or written with
/// its entry point alone, by its head.
fn preimage(plan: &str) -> String {
    let by_head: BTreeMap<&str, Identity> = LINES.iter().copied().collect();
    let mut out = String::new();
    for line in plan.lines() {
        match by_head
            .get(head(line))
            .unwrap_or_else(|| panic!("no line of `/2` begins `{}`", head(line)))
        {
            Identity::Kept => out.push_str(line),
            Identity::LeftOut => continue,
            Identity::EntryPoint => {
                let subject = if head(line) == "task-inputs" {
                    let from = line.find("(task ").expect("its task");
                    format!(
                        "{} ",
                        &line[from..=from + line[from..].find(')').expect("it closes")]
                    )
                } else {
                    String::new()
                };
                let said = match line.find("(entry-point \"") {
                    Some(from) => line
                        [from..=from + line[from..].find("\")").expect("it closes") + 1]
                        .to_owned(),
                    None => "none".to_owned(),
                };
                out.push_str(&format!("({} {subject}{said})", head(line)));
            }
        }
        out.push('\n');
    }
    out
}

/// The record's own table of lines, after `<!-- machine-read: code-identity -->`: each head and what the identity does with it.
fn the_record_s_table() -> Vec<(String, Identity)> {
    let after = RECORD
        .split("<!-- machine-read: code-identity -->")
        .nth(1)
        .expect("the record marks its table");
    after
        .lines()
        .skip_while(|line| !line.starts_with('|'))
        .take_while(|line| line.starts_with('|'))
        .skip(2)
        .map(|row| {
            let cells: Vec<&str> = row.trim_matches('|').split('|').map(str::trim).collect();
            let does = match cells[1] {
                "kept" => Identity::Kept,
                "left out" => Identity::LeftOut,
                "its entry point" => Identity::EntryPoint,
                other => panic!("`{other}` is nothing the identity does with a line"),
            };
            (cells[0].trim_matches('`').to_owned(), does)
        })
        .collect()
}

/// The heads of the lines §3's forms block writes, in its order.
fn the_forms_heads() -> Vec<String> {
    let block = RECORD
        .split("```text\n(format \"archogen-runtime-plan/2\")")
        .nth(1)
        .expect("§3's forms");
    let block = &block[..block.find("```").expect("it closes")];
    let mut heads = vec!["format".to_owned()];
    for line in block.lines() {
        let code = line.split(';').next().unwrap_or(line);
        let mut rest = code;
        while let Some(at) = rest.find('(') {
            // A line's alternatives stand side by side, `(a …) | (b …)`, and several lines can share one of the block's: each opens at depth nothing.
            let depth = code[..code.len() - rest.len() + at]
                .chars()
                .fold(0_i64, |depth, c| {
                    depth + i64::from(c == '(') - i64::from(c == ')')
                });
            if depth == 0 {
                let name = head(&rest[at..]).to_owned();
                if name != "…" && !heads.contains(&name) {
                    heads.push(name);
                }
            }
            rest = &rest[at + 1..];
        }
    }
    heads
}

/// The record states what the identity does with a line once, in a table this model reads: the table and the model say the same, and every line
/// §3's forms write has its row, so a line added to the format cannot go unclassified.
#[test]
fn the_record_s_table_of_lines_is_the_model_s_and_holds_every_form() {
    let table = the_record_s_table();
    let model: Vec<(String, Identity)> = LINES
        .iter()
        .map(|(head, does)| ((*head).to_owned(), *does))
        .collect();
    assert_eq!(table, model);
    let heads = the_forms_heads();
    for head in &heads {
        assert!(
            LINES.iter().any(|(known, _)| known == head),
            "§3 writes a line `({head} …)` that §1's table does not hold"
        );
    }
    for (head, _) in LINES {
        assert!(
            heads.iter().any(|known| known == head),
            "§1's table holds `({head} …)`, which §3's forms do not write"
        );
    }
}

/// Two plans of one system, a first with every generated figure owed and a second with a realization's file, have one preimage; and what is
/// code moves it: an entry point, the application's sources, the catalog, a route's level.
#[test]
fn two_plans_of_one_code_have_one_preimage_and_code_moves_it() {
    let shared = |entry: &str, digest: &str, catalog: &str, level: u64| {
        vec![
            "(format \"archogen-runtime-plan/2\")".to_owned(),
            "(profile rt-static-up-v1)".to_owned(),
            "(system heartbeat)".to_owned(),
            "(target riscv-virt-up \"aa\")".to_owned(),
            format!("(catalog \"{catalog}\" riscv-virt-up)"),
            format!("(code app \"../app\" \"sha256:{digest}\")"),
            "(task 0 beat (period 1/100 s))".to_owned(),
            format!("(route 1 (source uart.rx) (controller 10 {level}))"),
            format!("(task-inputs (task heartbeat beat) (entry-point \"{entry}\") (stack-bound (value 512) (unit byte)))"),
            "(initialisation-inputs none)".to_owned(),
        ]
    };
    let first = |lines: Vec<String>| {
        let mut plan = lines;
        plan.insert(5, "(application \"bb\")".to_owned());
        plan.extend(["(allocation (stack boot) owed (needs (realization (step 1) initialisation-stack)))", "(image (floor 2147483648) (limit owed))", "(owed (step 1) initialisation-stack (by realization))", "(complete no)"].map(str::to_owned));
        plan.join("\n") + "\n"
    };
    let second = |lines: Vec<String>| {
        let mut plan = lines;
        plan.insert(5, "(application \"cc\")".to_owned());
        plan.insert(6, "(realization \"dd\")".to_owned());
        plan.insert(7, "(record port.riscv \"ee\")".to_owned());
        plan.extend(["(declared leaves-interrupt-hardware-alone yes (clause \"…\") (basis \"it touches no device\"))", "(realization-figure (step 1) initialisation-stack (value 128) (unit byte))", "(realization-fact (source uart.rx) acknowledge-point entry)", "(allocation (stack boot) (base 2281701376) (guard 64) (bytes 1024) (align 16))", "(image (floor 2147483648) (limit 2281701312))", "(complete yes)"].map(str::to_owned));
        plan.join("\n") + "\n"
    };
    let one = preimage(&first(shared("app::beat", "00", "c1", 7)));
    assert_eq!(one, preimage(&second(shared("app::beat", "00", "c1", 7))));
    assert!(
        one.contains("(task-inputs (task heartbeat beat) (entry-point \"app::beat\"))\n")
            && one.contains("(initialisation-inputs none)\n"),
        "{one}"
    );
    assert!(
        !one.contains("stack-bound") && !one.contains("owed") && !one.contains("complete"),
        "{one}"
    );
    for moved in [
        shared("app::tick", "00", "c1", 7),
        shared("app::beat", "11", "c1", 7),
        shared("app::beat", "00", "c2", 7),
        shared("app::beat", "00", "c1", 6),
    ] {
        assert_ne!(preimage(&first(moved)), one);
    }
    let named = preimage("(initialisation-inputs (entry-point \"app::init\") (stack-bound (value 256) (unit byte)) (node-calls (call console.write 1)))\n");
    assert_eq!(
        named,
        "(initialisation-inputs (entry-point \"app::init\"))\n"
    );
}
