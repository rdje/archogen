//! Format `/2`'s decision procedures as an executable model (`docs/decisions/decision_runtime-plan-format-2.md`;
//! `docs/decisions/decision_executable-design-reviews.md`). `/2` becomes code leaf by leaf (`M4.1.7.6`). Three rounds of its review worked its formulas by hand, and each
//! found one wrong, the record's own worked example among them. So each procedure the record states is transcribed here, rule by rule and citing its
//! section, optimised for nothing and on `std` alone; a bounded exhaustive checker holds what the record claims of each over every small input; and each
//! probe of a round is a row of a corpus with the outcome the record gives it. `M4.1.7`'s lowering is to be held against this over the same universe.
//! Where the record takes a rule from code that exists, the joint rules' reading of a memory map, the model is held to that code and not to a reading of it
//! (round 4, RP4-5): `the_model_refuses_a_map_as_the_joint_rules_do`, the one test here that calls another crate.
//!
//! ⭐ **The lowering held to the model** (leaf `M4.1.7.6.1` on): each procedure the library builds is held to its transcription here by an agreement check
//! over the model's own inputs — each small region, unit and set of figures it enumerates, and each probe of a round — written beside the model's own
//! assertion so that every case the model runs is one the library runs too. The model stays on `std` and its own rules; only those checks call
//! `archogen_plan`, and a check that disagrees is the library's defect or the model's, settled against the record.
//!
//! What is modelled: §1's code identity, by the record's own table of lines; §4's epoch and the reach it asks of one write; §5's floor, limit and
//! cursor, and each stack's sum; §6's priorities. What is not: the text of a plan's lines, which the lowering's goldens will hold (`M4.1.7`).

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{read, Form, SourceMap, Verdict};
use eadl_model::check::shipped_registry;
use eadl_resolve::joint::{requesters, Finding, Joint, Requester, System};
use eadl_resolve::offer::{self, Provider};
use eadl_resolve::vocabulary::Vocabulary;

const RECORD: &str = include_str!("../../../docs/decisions/decision_runtime-plan-format-2.md");

// ---- the library beside the model (leaf `M4.1.7.6.1`)

/// A model region as the library's span.
fn span(region: Region) -> archogen_plan::layout::Span {
    archogen_plan::layout::Span {
        base: u128::from(region.base),
        end: u128::from(region.end),
    }
}

/// A model map's reserved ranges as the library's.
fn reserved(raw: &[(u64, Option<u64>)]) -> Vec<archogen_plan::layout::Reserved> {
    raw.iter()
        .map(|&(base, bytes)| archogen_plan::layout::Reserved {
            base: u128::from(base),
            bytes: bytes.map(u128::from),
        })
        .collect()
}

/// A model unit as the library's: a stack or a table, its alignment never zero.
fn unit_of(unit: Unit) -> archogen_plan::layout::Unit {
    let align = std::num::NonZeroU128::new(u128::from(unit.align)).expect("an alignment");
    if unit.stack {
        archogen_plan::layout::Unit::stack(u128::from(unit.bytes), u128::from(unit.guard), align)
    } else {
        assert_eq!(unit.guard, 0, "a table carries no guard");
        archogen_plan::layout::Unit::table(u128::from(unit.bytes), align)
    }
}

/// §5's floor, the library's held to the model's wherever rule 10 refuses nothing of the map.
fn floor_held(region: Region, raw: &[(u64, Option<u64>)]) -> Result<u64, BTreeSet<Refused>> {
    let model = floor(region, raw);
    if let Ok(at) = model {
        let holes = archogen_plan::layout::holes(span(region), &reserved(raw));
        assert_eq!(
            holes
                .iter()
                .map(|hole| (hole.span.base, hole.span.end, hole.sized))
                .collect::<Vec<_>>(),
            holes_in(region, raw)
                .into_iter()
                .map(|(base, end, sized)| (u128::from(base), u128::from(end), sized))
                .collect::<Vec<_>>(),
            "the holes of {region:?} {raw:?}"
        );
        assert_eq!(
            archogen_plan::layout::floor(span(region), &holes),
            u128::from(at),
            "the floor of {region:?} {raw:?}"
        );
    }
    model
}

/// §5's formula and limit, the library's held to the model's: the same units laid or the same one refused, and where all fit, the same limit and the
/// same holes truncating it.
fn lay_held(
    region: Region,
    raw: &[(u64, Option<u64>)],
    units: &[Option<Unit>],
    floor: u64,
    want: &Result<Vec<(u64, u64)>, Refused>,
) {
    let holes = archogen_plan::layout::holes(span(region), &reserved(raw));
    let library: Vec<Option<archogen_plan::layout::Unit>> =
        units.iter().map(|unit| unit.map(unit_of)).collect();
    let got = archogen_plan::layout::lay(span(region), &holes, &library, u128::from(floor));
    let as_model = got
        .as_ref()
        .map(|laid| {
            laid.iter()
                .map(|at| {
                    (
                        u64::try_from(at.base).expect("a base"),
                        u64::try_from(at.end).expect("a top"),
                    )
                })
                .collect::<Vec<_>>()
        })
        .map_err(|refused| Refused::NoRoom(refused.unit));
    assert_eq!(&as_model, want, "{region:?} {raw:?} {units:?}");
    if let (Ok(laid), Ok(spans)) = (want, &got) {
        let owed = units.iter().any(Option::is_none);
        let (limit_at, names) = limit(region, &holes_in(region, raw), floor, laid, owed);
        let library =
            archogen_plan::layout::limit(span(region), &holes, u128::from(floor), spans, owed);
        assert_eq!(
            library.limit,
            limit_at.map(u128::from),
            "{region:?} {raw:?} {units:?}"
        );
        assert_eq!(
            library
                .truncated_by
                .iter()
                .map(|hole| hole.span.base)
                .collect::<Vec<_>>(),
            names.into_iter().map(u128::from).collect::<Vec<_>>(),
            "{region:?} {raw:?} {units:?}"
        );
    }
}

// ---- §5: memory

/// A placement region, `[base, end)`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct Region {
    base: u64,
    end: u64,
}

/// What is refused of a region's memory, or of a unit in it.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
enum Refused {
    /// Model §8 rule 10: "placement memory reserved in every byte leaves nothing to place in", "a byte surely reserved being a sized hole's or the base of
    /// one with no size".
    MemoryAllHoles,
    /// Rule 10: "Where a placement region's holes, those with a size at their size and those with none so taken, leave it no free byte and the bytes surely
    /// reserved do not fill it".
    HoleUnsized,
    /// Rule 10: a reserved range that "has no size and a base below placement memory, whether they overlap is `missing-fact`".
    PlacementUnsized,
    /// §5: "if base < F: refuse, 'no room'", naming the unit by its place in the order.
    NoRoom(usize),
}

/// Each hole as the bytes it takes of the region, `[base, end)`, and whether it had a size. A hole is a reserved range that surely overlaps the region, by a
/// sized one's bytes or the base of one with no size, which is then "taken to reach that region's end" (model §8 rule 10; §5, "A hole with no size").
fn holes_in(region: Region, holes: &[(u64, Option<u64>)]) -> Vec<(u64, u64, bool)> {
    holes
        .iter()
        .filter(|&&(base, size)| base < region.end && base + size.unwrap_or(1) > region.base)
        .map(|&(base, size)| {
            (
                base,
                size.map_or(region.end, |size| base + size),
                size.is_some(),
            )
        })
        .filter(|&(base, end, _)| base < end)
        .map(|(base, end, sized)| (base.max(region.base), end.min(region.end), sized))
        .collect()
}

/// Whether `ranges` cover the region whole.
fn covers(region: Region, ranges: &[(u64, u64)]) -> bool {
    let mut at = region.base;
    while let Some(&(_, end)) = ranges.iter().find(|&&(base, end)| base <= at && at < end) {
        at = end;
    }
    at >= region.end
}

/// What model §8 rule 10 refuses of a placement region's reserved ranges before anything is laid, each by the name `Joint::map` gives it (§5, "Which regions"),
/// whatever order the ranges are written in: first the bytes surely reserved, which no size repairs, and only then each hole to the region's end.
fn mapped(region: Region, raw: &[(u64, Option<u64>)]) -> BTreeSet<Refused> {
    let mut said = BTreeSet::new();
    if raw
        .iter()
        .any(|&(base, size)| size.is_none() && base < region.base)
    {
        said.insert(Refused::PlacementUnsized);
    }
    let surely: Vec<(u64, u64)> = raw
        .iter()
        .map(|&(base, size)| (base, base + size.unwrap_or(1)))
        .collect();
    let taken: Vec<(u64, u64)> = holes_in(region, raw)
        .iter()
        .map(|&(base, end, _)| (base, end))
        .collect();
    if covers(region, &surely) {
        said.insert(Refused::MemoryAllHoles);
    } else if covers(region, &taken) {
        said.insert(Refused::HoleUnsized);
    }
    said
}

/// §5, "The image lies at the bottom": "`F` is the least address at or above the placement region's base that no hole covers: the base, moved
/// past each hole that covers it, one after another", where rule 10 refuses nothing of the region; and what it refuses where it does.
fn floor(region: Region, raw: &[(u64, Option<u64>)]) -> Result<u64, BTreeSet<Refused>> {
    let said = mapped(region, raw);
    if !said.is_empty() {
        return Err(said);
    }
    let holes = holes_in(region, raw);
    let mut floor = region.base;
    while let Some(&(_, end, _)) = holes
        .iter()
        .find(|&&(base, end, _)| base <= floor && floor < end)
    {
        floor = end;
    }
    Ok(floor)
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

/// §5: "`L` is the less of the lowest allocation's base and the base of the lowest hole above `F`", "`owed` while the lowest allocation is, the
/// lowest hole above `F` then standing in `truncated-by` as the most the limit can be", and "`truncated-by` names each hole whose base is `L`". The limit, nothing where it
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
                let Ok(floor) = floor_held(region, raw) else {
                    continue;
                };
                let holes = holes_in(region, raw);
                for units in &orders {
                    let want = reference(region, &holes, units, floor);
                    for pick in picks {
                        assert_eq!(
                            lay(region, &holes, units, floor, pick),
                            want,
                            "{region:?} {raw:?} {units:?}"
                        );
                    }
                    lay_held(region, raw, units, floor, &want);
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
    let floor_of = |holes: &[(u64, Option<u64>)]| floor_held(ram, holes);
    let refused = |kinds: &[Refused]| Err(kinds.iter().copied().collect::<BTreeSet<Refused>>());
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
        refused(&[Refused::MemoryAllHoles])
    );
    assert_eq!(
        floor_of(&[(0x8000_0000, Some(64 * KIB)), (0x8001_0000, None)]),
        refused(&[Refused::HoleUnsized])
    );
    // RP4-5: a hole of no size at the region's one byte surely reserves it, which no size repairs; and what is refused moves with no order of the list.
    let byte = Region {
        base: 256,
        end: 257,
    };
    assert_eq!(
        floor(byte, &[(255, Some(1)), (256, None)]),
        refused(&[Refused::MemoryAllHoles])
    );
    let three = [(255, None), (255, Some(1)), (255, Some(2))];
    let mut back = three;
    back.reverse();
    assert_eq!(
        floor(byte, &three),
        refused(&[Refused::MemoryAllHoles, Refused::PlacementUnsized])
    );
    assert_eq!(floor(byte, &back), floor(byte, &three));
    assert_eq!(
        floor(byte, &[(255, None)]),
        refused(&[Refused::PlacementUnsized])
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

/// A description of one system on one platform whose `ram` is the region, with a block holding each reserved range.
fn a_map(region: Region, raw: &[(u64, Option<u64>)]) -> String {
    let reserved: String = raw
        .iter()
        .enumerate()
        .map(|(at, (base, size))| match size {
            Some(size) => {
                format!(" (region h{at} (base {base}) (size {size} byte) (reserved true))")
            }
            None => format!(" (region h{at} (base {base}) (reserved true))"),
        })
        .collect();
    format!(
        "(defblock fw (offers (core-count 1 tick){reserved}))\n(defplatform soc (offers (core-count 1 tick) (region ram (base {}) (size {} byte) (executable true))) (requires (uses fw)))\n(defsystem s (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1)) (platform (uses soc)))\n",
        region.base,
        region.end - region.base
    )
}

/// What `Joint::map` finds of that description's one system, by the names the model gives each (`crates/eadl-resolve/src/joint.rs`).
fn the_joint_rules_say(text: &str, vocabulary: &Vocabulary) -> BTreeSet<Refused> {
    let mut sources = SourceMap::new();
    let id = sources.add("map.eadl", text).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(!diagnostics.has_errors(), "{text}");
    let declarations: Vec<Form> = document.forms;
    let hardware = |form: &Form| matches!(form.head(), Some("defblock" | "defplatform"));
    let providers: Vec<Provider> = declarations
        .iter()
        .filter(|form| hardware(form))
        .map(|form| {
            offer::read(form, vocabulary).unwrap_or_else(|refusal| panic!("{refusal:?}: {text}"))
        })
        .collect();
    let asking: Vec<Requester> = declarations
        .iter()
        .filter(|form| !hardware(form))
        .flat_map(|form| {
            requesters(form, vocabulary).unwrap_or_else(|refusal| panic!("{refusal:?}: {text}"))
        })
        .collect();
    let declared = |name: &str| {
        declarations
            .iter()
            .find(|form| form.items().get(1).and_then(Form::as_symbol) == Some(name))
            .expect("declared")
    };
    let pairs: Vec<(Provider, &Form)> = providers
        .iter()
        .map(|provider| (provider.clone(), declared(&provider.name)))
        .collect();
    let joint = Joint::new(&pairs, &asking, vocabulary)
        .unwrap_or_else(|findings| panic!("{} finding(s): {text}", findings.len()));
    let system = System::read(declared("s"), &declarations);
    let map = joint.map(&system);
    map.refused
        .iter()
        .chain(&map.unknowns)
        .map(|finding| match finding {
            Finding::MemoryAllHoles { .. } => Refused::MemoryAllHoles,
            Finding::HoleUnsized { .. } => Refused::HoleUnsized,
            Finding::PlacementUnsized { .. } => Refused::PlacementUnsized,
            other => panic!("{other:?}: {text}"),
        })
        .collect()
}

/// The record gives each refusal of a map "by the code the joint rules give it", so the model's are held to `Joint::map` itself and to no reading of rule 10: over
/// every region of up to 4 bytes with up to two reserved ranges of each size and of none, based from one below it to its end, the two say the same, and where they
/// say nothing the model has a floor.
#[test]
fn the_model_refuses_a_map_as_the_joint_rules_do() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|relative| {
        (
            (*relative).to_string(),
            std::fs::read_to_string(root.join(relative)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the shipped kinds read");
    let vocabulary = Vocabulary::shipped(&mut SourceMap::new(), &registry)
        .expect("the shipped vocabulary reads");
    let (mut maps, mut by_what_is_said): (u64, BTreeMap<Vec<Refused>, u64>) = (0, BTreeMap::new());
    for size in 1..=4 {
        let region = Region {
            base: 256,
            end: 256 + size,
        };
        let mut each: Vec<(u64, Option<u64>)> = Vec::new();
        for at in region.base - 1..=region.end {
            each.extend(
                [None, Some(1), Some(2), Some(3), Some(5)]
                    .into_iter()
                    .map(|size| (at, size)),
            );
        }
        let mut sets: Vec<Vec<(u64, Option<u64>)>> = vec![Vec::new()];
        for (place, first) in each.iter().enumerate() {
            sets.push(vec![*first]);
            sets.extend(each[place + 1..].iter().map(|second| vec![*first, *second]));
        }
        for raw in &sets {
            let model = mapped(region, raw);
            assert_eq!(
                model,
                the_joint_rules_say(&a_map(region, raw), &vocabulary),
                "{region:?} {raw:?}"
            );
            assert_eq!(
                floor(region, raw).is_ok(),
                model.is_empty(),
                "{region:?} {raw:?}"
            );
            let mut back = raw.clone();
            back.reverse();
            assert_eq!(
                mapped(region, &back),
                model,
                "{region:?} {raw:?}: the order of the list moved it"
            );
            maps += 1;
            *by_what_is_said
                .entry(model.into_iter().collect())
                .or_default() += 1;
        }
    }
    assert_eq!(maps, 1_124);
    // Each of the six outcomes is among them, so none is held by having no case.
    assert_eq!(by_what_is_said.len(), 6, "{by_what_is_said:?}");
}

// ---- §5: the stacks

/// A stack's figures, by the names §5's table of figures gives them. A name of several figures, each node's or each route's, holds each of them.
type Figures = BTreeMap<&'static str, Vec<u64>>;

/// A cell of §5's table of sums, read as it is written: sums of names and `max(…, …)`. A name of several figures is the largest of them, and nothing where there is none
/// (§5, "A name that is several figures"); `fault-use` is "the `fault-entry-stack` plus the larger of the `fault-path-stack`, the panic's route from the
/// handler's call, and the `fault-check-stack`, a check's and a trap's" (rounds 18 and 20, RP18-1, RP20-1).
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
            return self.value("fault-entry-stack")
                + self
                    .value("fault-path-stack")
                    .max(self.value("fault-check-stack"));
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
        // The boot's code calls each step, the catalog's own initialisation, the application's, which calls nodes, and its last act, which calls the transition, the idle context following,
        // measured from the transition's call, so what the transition leaves beneath it is in its figure (round 18, RP18-2).
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
    // A fault takes one entry of the fault path, the panic's through the generated handler or the one a check's call and the trap path reach (round 18, RP18-1);
    // and a first fault raised in the handler before its call takes the other entry's route on top of what the handler ran (round 20, RP20-1).
    deepest(&on_it)
        + one("fault-entry-stack")
        + one("fault-path-stack").max(one("fault-check-stack"))
        + one("trap-frame")
}

/// §5: the fault path has two entries and a fault takes one, the panic's through the generated handler, or the one a check's call and the trap path reach; so a
/// stack holds the larger route. It held the panic's alone: a check ending in a deliberate trap at a node's deepest point, its route 450 beside the panic's 400,
/// stood at 1662 bytes where the sum gave 1612 (round 18, RP18-1, its reader's case). And a first fault raised in the generated handler before its call takes
/// that route on top of the handler's 40: 1702 stood where round 18's sum gave 1662 (round 20, RP20-1, its reader's case).
#[test]
fn a_check_s_route_into_the_fault_path_is_held_too() {
    let figures: Figures = [
        ("call-frame", vec![48]),
        ("stack-bound", vec![512]),
        ("node-stack", vec![380]),
        ("primitive-stack", vec![96]),
        ("completion-stack", vec![64]),
        ("fault-entry-stack", vec![40]),
        ("fault-path-stack", vec![360]),
        ("fault-check-stack", vec![450]),
        ("trap-frame", vec![272]),
    ]
    .into_iter()
    .collect();
    assert_eq!(
        bytes("a task's", &figures).0,
        48 + 512 + 380 + 40 + 450 + 272
    );
    assert_eq!(
        bytes("a task's", &figures).0,
        standing("a task's", &figures)
    );
    // The idle context is measured from the transition's call, so a transition that leaves its frame beneath it is in its figure (RP18-2).
    assert!(RECORD
        .contains("from the transition's call, so what the transition leaves beneath it is in it"));
}

/// §5: each fault figure is bounded to the runtime's end, a fault taken on the way among it, its frame and the trap path's entry, as the fault contract's rule 7
/// ends one at once: so a second trap on a fault's route is in the figure, and no sum adds a second `trap-frame`. None held it before (round 19, RP19-1).
#[test]
fn a_fault_taken_on_the_way_is_in_the_fault_figures() {
    for figure in ["fault-path-stack", "fault-check-stack"] {
        let row = RECORD
            .lines()
            .find(|line| line.starts_with(&format!("| `{figure}` |")))
            .unwrap_or_else(|| panic!("a row of `{figure}`"));
        assert!(
            row.contains("to the runtime's end, a fault taken on the way, its frame and the trap path's entry, among it"),
            "{row}"
        );
    }
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
    for name in [
        "fault-entry-stack",
        "fault-path-stack",
        "fault-check-stack",
        "trap-frame",
    ] {
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

/// A unit's `bytes` as the library sums it from the same figures (leaf `M4.1.7.6.1`): a name of several figures the largest of them, the boot's
/// `stack-bound` none where the application states no initialisation.
fn library_bytes(unit: &str, figures: &Figures) -> u128 {
    let one = |name: &str| {
        figures
            .get(name)
            .and_then(|values| values.first())
            .map_or(0, |&value| u128::from(value))
    };
    let all = |name: &str| -> Vec<u128> {
        figures.get(name).map_or_else(Vec::new, |values| {
            values.iter().map(|&value| u128::from(value)).collect()
        })
    };
    let fault = archogen_plan::layout::Fault {
        entry: one("fault-entry-stack"),
        path: one("fault-path-stack"),
        check: one("fault-check-stack"),
    };
    match unit {
        "a task's" => archogen_plan::layout::TaskStack {
            call_frame: one("call-frame"),
            stack_bound: one("stack-bound"),
            node_stack: all("node-stack"),
            primitive_stack: all("primitive-stack"),
            completion_stack: one("completion-stack"),
            fault,
            trap_frame: one("trap-frame"),
        }
        .bytes(),
        "the interrupt's" => archogen_plan::layout::InterruptStack {
            trap_path_stack: one("trap-path-stack"),
            call_frame: one("call-frame"),
            service_stack: all("service-stack"),
            timer_service_stack: Some(one("timer-service-stack")),
            fault,
            trap_frame: one("trap-frame"),
        }
        .bytes(),
        "the boot's" => archogen_plan::layout::BootStack {
            call_frame: one("call-frame"),
            initialisation_stack: all("initialisation-stack"),
            initialisation_path_stack: one("initialisation-path-stack"),
            stack_bound: all("stack-bound").first().copied(),
            node_stack: all("node-stack"),
            last_act_stack: one("last-act-stack"),
            transition_stack: one("transition-stack"),
            idle_stack: one("idle-stack"),
            fault,
            trap_frame: one("trap-frame"),
        }
        .bytes(),
        other => panic!("no stack is {other}"),
    }
}

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
            assert_eq!(
                library_bytes(unit, &figures),
                u128::from(bytes(unit, &figures).0),
                "the library's sum of {unit}: {figures:?}"
            );
            run += 1;
        }
    }
    // Twice what it was before round 18: `fault-check-stack` is varied too, as none and as one (RP18-1).
    assert_eq!(run, 2 * (5_400 + 1_080 + 106_920));
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
            .chain(&[
                "fault-entry-stack",
                "fault-path-stack",
                "fault-check-stack",
                "trap-frame",
            ])
            .map(|name| (*name, vec![1]))
            .collect();
        let (_, in_its_cell) = bytes(unit, &given);
        // What a unit is given here is what its cell names, so the checker above varies every figure of a sum and no other.
        let mut wanted: Vec<&str> = in_its_cell
            .iter()
            .map(String::as_str)
            .filter(|name| *name != "fault-use")
            .chain(["fault-entry-stack", "fault-path-stack", "fault-check-stack"])
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
                ("fault-check-stack", &[0]),
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
                ("fault-check-stack", &[0]),
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
            ("fault-check-stack", &[0]),
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
            ("fault-check-stack", &[0]),
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

/// §4, "The last act's order" (round 4, RP4-1): what the last act arms or writes for an alarm is generated code, which no port's record bounds. Done after
/// the read it stands between the read and the enabling, past the lead; done before it, as §4 orders the act, the read comes after and nothing moves.
#[test]
fn what_the_last_act_arms_it_arms_before_its_read() {
    // RP4-1's case, in hundredths of a microsecond: a lead of 0.10 + 0.30 + 2.00, a read taken at 100.09 µs, and one wake armed in 1.00 µs after the write.
    let (tick, write, transition, arm) = (10, 30, 200, 100);
    let lead = tick + write + transition;
    assert_eq!(
        (epoch(10_009, tick, lead), 10_009 + write + arm + transition),
        (10_240, 10_339)
    );
    // In §4's order the arm is done when the read is taken, however long it took: the epoch is at or after the enabling at every phase.
    for armed_for in 0..=3 * arm {
        let read_at = 10_009 + armed_for;
        assert!(
            epoch(read_at, tick, lead) >= read_at + write + transition,
            "{armed_for}"
        );
    }
}

/// §3, `RA-H7`: whether a task that reaches an alarm's node owes the application's statement of it. An alarm on the counter itself that is no wake, as the
/// scheduling checker's model asks it: a wake is written by its own service alone, which no task signs for (round 4, RP4-3).
fn asks_the_application(wake: bool, on_the_counter: bool) -> bool {
    !wake && on_the_counter
}

#[test]
fn a_wake_asks_no_statement_of_the_application() {
    assert_eq!(
        [(false, true), (false, false), (true, true), (true, false)]
            .map(|(wake, counter)| asks_the_application(wake, counter)),
        [true, false, false, false]
    );
    // The checker's model gates the application's statement by the same two, and a change there is a change here.
    let checker = include_str!("../../rt-analysis/tests/alarm_model.rs");
    assert!(checker.contains("let on_the_counter = !line.wake && line.counter_itself;"));
}

/// §3: what `(owed …)` gains of an alarm by the realization is each input `decision_release-alarm.md` §2 gives the realization to state, a source's among them
/// (round 4, RP4-4): so a plan with an alarm cannot say `(complete yes)` with its acknowledge point unstated. The source's inputs are read from that record's
/// table, each phrase mapped to its fact, so an input the table gains fails here: written from memory, the list left out `external`, whether its interrupt
/// reaches the controller, on which every `J` rests (round 21, RP21-1).
#[test]
fn what_an_alarm_owes_as_a_source_has_its_owed_fact() {
    const AS_A_SOURCE: [(&str, &str); 6] = [
        ("`C_s`", "service-cost"),
        ("the acknowledge point", "acknowledge-point"),
        ("deferred work", "deferred-work"),
        ("`no-application-code`", "no-application-code"),
        ("`one-request-per-arrival`", "one-request-per-arrival"),
        ("whether it reaches the controller", "external"),
    ];
    let alarm = include_str!("../../../docs/decisions/decision_release-alarm.md");
    let row = alarm
        .lines()
        .find(|line| line.starts_with("| `C_s`, the acknowledge point"))
        .expect("the release alarm's row of what a source owes");
    let first = row.split(" | ").next().expect("its first cell");
    let phrases: Vec<&str> = AS_A_SOURCE.iter().map(|(phrase, _)| *phrase).collect();
    assert_eq!(
        first.trim_start_matches("| "),
        phrases.join(", "),
        "the table's inputs as a source"
    );
    let owed = RECORD
        .split("**`(owed …)`, as `/2` writes it.**")
        .nth(1)
        .expect("what `/2` owes");
    let of_an_alarm = owed
        .split("of an alarm by `realization`,")
        .nth(1)
        .expect("an alarm's");
    let of_an_alarm = &of_an_alarm[..of_an_alarm.find(';').expect("its end")];
    for fact in [
        "floor",
        "step",
        "reach-ahead",
        "lag",
        "service-cost",
        "acknowledge-point",
        "deferred-work",
        "no-application-code",
        "one-request-per-arrival",
    ]
    .into_iter()
    .chain(AS_A_SOURCE.iter().map(|(_, fact)| *fact))
    {
        assert!(
            of_an_alarm.contains(&format!("`{fact}`")),
            "an alarm's `{fact}` is owed by no line: {of_an_alarm}"
        );
    }
}

/// §3: a source on a target owes `external`, which the composition gates every `J` on (`compose.rs`, `external.<source>`), by the realization, the
/// supplier of its generated service, as `archogen_catalog::selection` and the catalog record's §12 group `external.<source>` with `service.<source>`,
/// read from that group's supplier alone; `/1` §2.5 gives it the simulator on the playground alone (round 22, RP22-1). Round 27 gave it the controller's
/// record, which no reader reads it from where no record supplies the service (round 30, RP30-1). `(realization-fact …)` carries it, an alarm's, and a
/// called node's two facts below.
#[test]
fn a_source_s_external_comes_with_its_service() {
    let owed = RECORD
        .split("**`(owed …)`, as `/2` writes it.**")
        .nth(1)
        .expect("what `/2` owes");
    // Its subject is the source's, `(source NAME)`, as `/1` writes the source's other facts (round 23, RP23-5).
    let of_a_route = owed
        .split("of a source on a target by `realization`,")
        .nth(1)
        .expect("a source's");
    let selection = include_str!("../../archogen-catalog/src/selection.rs");
    assert!(
        selection.contains("\"one-request-per-arrival.\",\n        \"external.\","),
        "grouped with its service"
    );
    let model = include_str!("../../../docs/semantics/model.md");
    assert!(model.contains(
        "a source's `external.<source>` being stated with its service by that service's supplier"
    ));
    let of_a_route = &of_a_route[..of_a_route.find(';').expect("its end")];
    assert!(of_a_route.contains("`external`"), "{of_a_route}");
    assert!(of_a_route.contains("`(source NAME)`"), "{of_a_route}");
    let playground = include_str!("../../../docs/decisions/decision_runtime-plan.md");
    assert!(playground.contains("its `external`, is the simulator's (`RTP-H9`)"));
    let carried = RECORD
        .split("`(realization-fact …)` writes")
        .nth(1)
        .expect("the realization fact's sentence");
    let carried = &carried[..carried.find(": `VALUE`").expect("its list")];
    for fact in [
        "external",
        "primitives-out-of-line",
        "masking-balanced",
        "holds-under-preemption",
    ] {
        assert!(
            carried.contains(&format!("`{fact}`")),
            "`{fact}` has no line: {carried}"
        );
    }
}

/// §3 and the ledger: each fact `(realization-fact …)` says this record adds to what the realization owes is handed to the leaf that defines its file,
/// `RPF-H5`, and that leaf quotes it. Before, `RPF-H3` handed over what `/1` owes alone, so no leaf had to write the facts rounds 21 and 22 added, and every
/// target plan with a source, an alarm or a node a job calls stayed `(complete no)` (round 23, RP23-1).
#[test]
fn each_fact_this_record_adds_to_the_realization_is_handed_to_its_format() {
    let carried = RECORD
        .split("`(realization-fact …)` writes")
        .nth(1)
        .expect("the realization fact's sentence");
    let carried = &carried[..carried.find(": `VALUE`").expect("its list")];
    let added = carried
        .split("or this record adds, ")
        .nth(1)
        .expect("what this record adds");
    let facts: Vec<&str> = added.split('`').skip(1).step_by(2).collect();
    assert_eq!(
        facts,
        [
            "external",
            "primitives-out-of-line",
            "masking-balanced",
            "holds-under-preemption"
        ]
    );
    let row = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H5` | `M4.2` |"))
        .expect("the hand-off of what this record adds");
    for fact in &facts {
        assert!(
            row.contains(&format!("`{fact}`")),
            "`{fact}` handed to no leaf: {row}"
        );
    }
    let tasks = include_str!("../../../docs/tasks/M4.md");
    assert!(tasks.contains("- [RPF-H5] "), "`M4.2` quotes it");
}

/// §3: the realization's own costs, a called node's `call-cost` and `masked-runs` and an alarm's or a source's `service-cost`, owe the variant's
/// condition 8, as a catalog cost's `holds-under-preemption` is for a record's: generated code is in no record, and its generator states the same. Before,
/// no line asked it, and a complete plan left the checker to default a condition its condition 9 says is never defaulted (round 23, RP23-2).
#[test]
fn the_realization_s_own_costs_owe_condition_8() {
    let variant = include_str!("../../../docs/specs/analysis/decision_runtime-analysis-variant.md");
    assert!(variant.contains("Condition 8 is each catalog cost's `holds-under-preemption`"));
    let owed = RECORD
        .split("**`(owed …)`, as `/2` writes it.**")
        .nth(1)
        .expect("what `/2` owes");
    // On a target alone, the simulator stating it on the playground (round 25, RP25-2).
    let of_the_costs = owed
        .split("of each node whose context line holds `job`, each alarm and each source on a target by `realization`,")
        .nth(1)
        .expect("the realization's costs'");
    let of_the_costs = &of_the_costs[..of_the_costs.find(';').expect("its end")];
    // An alarm's `lag`, `λ`, a time of generated code added where no `γ` is charged, among them (round 24, RP24-3).
    for word in [
        "`holds-under-preemption`",
        "`call-cost`",
        "`masked-runs`",
        "`service-cost`",
        "`lag`",
    ] {
        assert!(of_the_costs.contains(word), "{word}: {of_the_costs}");
    }
}

/// §3 and §5: each piece of generated code the `call-frame` row says runs beneath what it calls is timed inside a part a record already bounds, the
/// catalog's image-specific figures being measured on the image, generated code included, or needs no time, for its reason. Round 25 charged a job's start
/// and the dispatch by figures of their own, which the catalog's `switch`, `completion`, `C_rel` and `C_s` already hold, so the composition would have
/// charged them twice, and a `service-cost` read from the service's call would have left out the trap path's entry; both are withdrawn, and the bare
/// subject `dispatch`, which a node of that name shared, with them (round 26, RP26-1, RP26-2).
#[test]
fn each_piece_of_generated_code_is_timed_once() {
    const TIMED: [(&str, &str); 4] = [
        (
            "a job's start beneath its entry point",
            "a job's start, before its entry point's call and after its return, inside the catalog's `switch`",
        ),
        (
            "the boot's code beneath each step",
            "the boot's code before the first enabling of interrupts",
        ),
        (
            "the dispatch beneath each service",
            "the dispatch beneath each service inside the timer service's `C_rel` and each service's `C_s`",
        ),
        (
            "core's panic frames where that code panics itself (RP21-2)",
            "core's panic frames on a path that ends the run",
        ),
    ];
    let row = RECORD
        .lines()
        .find(|line| line.starts_with("| `call-frame` | a stack | the realization |"))
        .expect("the `call-frame` row");
    let pieces = row
        .split("beneath what it calls there: ")
        .nth(1)
        .expect("its pieces");
    let pieces = &pieces[..pieces.rfind(" |").expect("its end")];
    let pieces: Vec<&str> = pieces
        .split(", ")
        .map(|piece| piece.strip_prefix("and ").unwrap_or(piece))
        .collect();
    let phrases: Vec<&str> = TIMED.iter().map(|(phrase, _)| *phrase).collect();
    assert_eq!(pieces, phrases, "the row's pieces");
    for (_, said) in TIMED {
        assert!(RECORD.contains(said), "where it is timed: {said}");
    }
    // The premises: the catalog's figures hold the image's generated code, and a service begins when its trap is taken.
    let catalog =
        include_str!("../../../docs/specs/catalog/decision_catalog-records-variant-inputs.md");
    assert!(catalog
        .contains("`completion` and `masked.completion` include the image's code: generated"));
    let variant = include_str!("../../../docs/specs/analysis/decision_runtime-analysis-variant.md");
    assert!(variant.contains("A service starts at its trap's entry"));
    assert!(RECORD.contains("spans its trap from entry to exit, as the variant's `C_s` does"));
    // Nothing in the record's rules charges it again; its review table keeps round 25's account.
    let rules = &RECORD[..RECORD.find("\n## Review\n").expect("the review")];
    for withdrawn in ["`job-start-cost`", "`dispatch-cost`", "| `RPF-H7` |"] {
        assert!(!rules.contains(withdrawn), "{withdrawn} charges it twice");
    }
}

/// The ledger and the variant: what condition 8 owes of the realization is read by the checker's leaf, `RPF-H6` to `M2.21.2`, quoted there, and the
/// variant's condition 8 names it beside the catalog's and the application's. Before, nothing read it, so a file stating `no` gave a complete plan the
/// checker admitted (round 24, RP24-1).
#[test]
fn the_realization_s_condition_8_is_read_by_the_checker() {
    let row = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H6` | `M2.21.2` |"))
        .expect("the hand-off to the checker's leaf");
    assert!(row.contains("`holds-under-preemption`"), "{row}");
    assert!(row.contains("never defaulted"), "{row}");
    assert!(
        row.contains("on a target"),
        "the simulator's on the playground: {row}"
    );
    let tasks = include_str!("../../../docs/tasks/M2.md");
    assert!(tasks.contains("- [RPF-H6] "), "`M2.21.2` quotes it");
    let variant = include_str!("../../../docs/specs/analysis/decision_runtime-analysis-variant.md");
    let condition = variant
        .split("Condition 8 is each catalog cost's `holds-under-preemption`")
        .nth(1)
        .expect("the variant's condition 8");
    let condition = &condition[..condition.find(". ").expect("its end")];
    assert!(
        condition.contains("the realization's `holds-under-preemption`"),
        "{condition}"
    );
}

/// §3 and the ledger: every fact and figure the owed list owes by the realization, read from the list itself, is handed to the leaf that writes its file,
/// by name in `RPF-H5`, or, a figure of a stack or the last act's lead, by the phrase `RPF-H3` gives it; a called node's `call-cost` and `masked-runs`,
/// named where its other facts are, are `/1`'s, handed by `RTP-H2`. Before, a row read one sentence's list, and an alarm's figures and what it owes as a
/// source were handed to no leaf (round 24, RP24-2).
#[test]
fn every_fact_the_realization_owes_is_handed_to_its_format() {
    const BY_RPF_H3: [(&str, &str); 7] = [
        ("node-stack", "each node's stack use from a call"),
        ("service-stack", "each service's"),
        ("initialisation-stack", "each initialisation step's"),
        ("fault-entry-stack", "its fault handler's"),
        ("last-act-stack", "its last act's"),
        (
            "call-frame",
            "what it holds on each stack beneath what it calls",
        ),
        ("last-act-lead", "its `last-act-lead`"),
    ];
    const OF_SLASH_1: [&str; 2] = ["call-cost", "masked-runs"];
    let row = |id: &str| {
        RECORD
            .lines()
            .find(|line| line.starts_with(&format!("| `{id}` | `M4.2` |")))
            .unwrap_or_else(|| panic!("`{id}`"))
    };
    let (h3, h5) = (row("RPF-H3"), row("RPF-H5"));
    let owed = RECORD
        .split("**`(owed …)`, as `/2` writes it.**")
        .nth(1)
        .expect("what `/2` owes");
    let owed = &owed[..owed.find("**Orders.**").expect("its end")];
    let mut found = Vec::new();
    for clause in owed
        .split([';', '\n'])
        .filter(|clause| clause.contains("by `realization`"))
    {
        let after = &clause[clause.find("by `realization`").expect("its owner")..];
        for fact in after.split('`').skip(1).step_by(2) {
            let named =
                !fact.is_empty() && fact.chars().all(|c| c.is_ascii_lowercase() || c == '-');
            if named && fact != "realization" && !found.contains(&fact) {
                found.push(fact);
            }
        }
    }
    for fact in [
        "floor",
        "lag",
        "acknowledge-point",
        "external",
        "masking-balanced",
        "holds-under-preemption",
        "node-stack",
        "last-act-lead",
    ] {
        assert!(found.contains(&fact), "the list read `{fact}`: {found:?}");
    }
    for fact in found {
        if let Some((_, phrase)) = BY_RPF_H3.iter().find(|(figure, _)| *figure == fact) {
            assert!(h3.contains(phrase), "`{fact}`: {phrase}");
        } else if !OF_SLASH_1.contains(&fact) {
            assert!(
                h5.contains(&format!("`{fact}`")),
                "`{fact}` handed to no leaf: {h5}"
            );
        }
    }
}

/// §7: one order over every refusal a plan on a target makes, §6's among them: a route whose `external` the realization's file states `no` is read only
/// from that file, after its code identity, and before placing, whose figures come from it. Before, §7 did not place it, and an input meeting it and no
/// room had two outcomes (round 23, RP23-3). Where one place names more than one code, they stand as `check`'s precedence ranks them, read from
/// `eadl_front::Verdict` itself, so one input gives the code `check` gives it: before, the two files' codes and the target's three stood in word order, no
/// placement memory after a silent count (round 35, RP35-4). A target's own refusals at lowering are `check`'s judgement of its file alone, by `DC-H3`,
/// so whatever `check` refuses of it stands there with `check`'s code: before, the profile's refusal of a machine of two cores had no place at lowering, and
/// a silent count beside it gave `missing-fact` where `check` gives `unsupported-profile`; and the build's two refusals of a plan stood in no order (round 36,
/// RP36-1, RP36-2).
#[test]
fn section_7_places_every_refusal_in_one_order() {
    const ORDER: [&str; 24] = [
        "a target's own refusals",
        "`check`'s judgement of the target's file alone as `DC-H3` runs it",
        "the file not reading",
        "more than one core",
        "a count of non-maskable interrupts or firmware traps above zero",
        "no placement memory",
        "either count silent",
        "the realization's file not reading",
        "the application's file not reading",
        "`/1`'s list in its order",
        "an assignment of the other kind",
        "the assignment's `(target …)` digest other than the file's",
        "its platform other than the file's beside the file's digest",
        "a finding of the generator's joint rules over the target's hardware",
        "an application's unmet",
        "too few levels while lowering the plan's code",
        "a realization of another code identity",
        "a route whose `external` the realization's file states `no`",
        "no room while placing",
        "The build refuses a plan before anything of the application's crate",
        "a task released by `(unknown CAUSE)`",
        "a plan whose generated code reads an owed line",
        "the crate's own refusals, `AI-H3`'s and `AI-H4`'s, follow",
        "A bound or a list of counts whose `binary` names another image",
    ];
    // The places that name more than one code, each with its codes in the order the record writes them.
    const RANKED: [&[(&str, &str)]; 2] = [
        &[
            (
                "a count of non-maskable interrupts or firmware traps above zero",
                "unsupported-profile",
            ),
            ("no placement memory", "infeasible-configuration"),
            ("either count silent", "missing-fact"),
        ],
        &[
            ("the realization's file not reading", "tool-failure"),
            ("the application's file not reading", "invalid-description"),
        ],
    ];
    let order = "A plan on a target adds those above, in this record's order";
    // The order's own sentence, read with its soft breaks as spaces: the bullet names an application's unmet before it, as a refusal `/1` cannot make.
    let flat = RECORD.split_whitespace().collect::<Vec<_>>().join(" ");
    let sentence = &flat[flat.find(order).expect("§7's order")..];
    let at: Vec<usize> = ORDER
        .iter()
        .map(|item| {
            sentence
                .find(item)
                .unwrap_or_else(|| panic!("`{item}` has no place: {sentence}"))
        })
        .collect();
    assert!(at.windows(2).all(|pair| pair[0] < pair[1]), "{at:?}");
    for group in RANKED {
        for (item, code) in group {
            let from = at[ORDER
                .iter()
                .position(|said| said == item)
                .expect("in the order")];
            let next = at
                .iter()
                .copied()
                .filter(|place| *place > from)
                .min()
                .unwrap_or(sentence.len());
            assert!(
                sentence[from..next].contains(&format!("`{code}`")),
                "`{item}` is `{code}`"
            );
        }
        for pair in group.windows(2) {
            let rank = |code: &str| Verdict::parse(code).expect("a verdict").precedence();
            assert!(
                rank(pair[0].1) > rank(pair[1].1),
                "{pair:?} as `check` ranks them"
            );
        }
    }
    assert!(sentence.contains("as `check`'s precedence ranks the three"));
    assert!(sentence.contains("as that precedence ranks the two"));
    let first = |holding: &[&str]| ORDER.iter().copied().find(|item| holding.contains(item));
    // The reader's case: a route stated not to reach the controller, and no room for the boot stack, refuses as §6 does, naming the source.
    assert_eq!(
        first(&[
            "a route whose `external` the realization's file states `no`",
            "no room while placing"
        ]),
        Some("a route whose `external` the realization's file states `no`")
    );
    // The reader's case of round 34: a file that does not read beside a stale program digest is the file's refusal, read first (RP34-3).
    assert_eq!(
        first(&[
            "the application's file not reading",
            "`/1`'s list in its order"
        ]),
        Some("the application's file not reading")
    );
    // The reader's case of round 33: a result record saying `ok` over a target the joint rules find lacking is `/1`'s `tool-failure` in its place, before
    // any of `/2`'s own (RP33-2).
    assert_eq!(
        first(&[
            "a finding of the generator's joint rules over the target's hardware",
            "no room while placing"
        ]),
        Some("a finding of the generator's joint rules over the target's hardware")
    );
    // The reader's cases of round 35: both files malformed is the realization's `tool-failure`; a target silent on non-maskable interrupts with no
    // executable region is `infeasible-configuration`, as `check` gives each (RP35-4).
    assert_eq!(
        first(&[
            "the application's file not reading",
            "the realization's file not reading"
        ]),
        Some("the realization's file not reading")
    );
    assert_eq!(
        first(&["either count silent", "no placement memory"]),
        Some("no placement memory")
    );
    // The reader's cases of round 36: a target of two cores silent on non-maskable interrupts is the profile's `unsupported-profile`, as `check` gives it;
    // and a plan whose lead is owed holding a task released by `(unknown CAUSE)` is `/1`'s refusal of the task, the build's first (RP36-1, RP36-2).
    assert_eq!(
        first(&["more than one core", "either count silent"]),
        Some("more than one core")
    );
    assert_eq!(
        first(&[
            "a plan whose generated code reads an owed line",
            "a task released by `(unknown CAUSE)`"
        ]),
        Some("a task released by `(unknown CAUSE)`")
    );
    assert!(sentence.contains(
        "whatever else that judgement refuses of the file standing as `check` places it"
    ));
    let correspondence = include_str!("../../../docs/decisions/decision_device-correspondence.md");
    assert!(correspondence.contains("runs `archogen check`'s judgement on the target's file alone"));
    assert!(correspondence
        .contains("what the profile asks of a machine, one active core among it, is judged of any target named"));
    let playground = include_str!("../../../docs/decisions/decision_runtime-plan.md");
    assert!(playground.contains(
        "a plan holding a task released by `(unknown CAUSE)` refused naming the task and its cause (`missing-fact`"
    ));
    assert!(playground
        .contains("`unsupported-profile` for `(own-faster ITEM)` and `(own-absent ITEM)`"));
    assert!(sentence.contains("first a task released by `(unknown CAUSE)`, as `/1`'s does and by its codes, naming the task and its cause (`RTP-H1`)"));
    assert!(!sentence.contains("each `missing-fact`, naming the task and its cause"));
    // A source's `external` is the realization's again, so its `no` is read where an alarm's is (round 30, RP30-1).
    assert!(!RECORD.contains("a source whose `external` its controller's record states `no`"));
    assert!(RECORD.contains("refuses the system `unsupported-profile`, naming its source"));
}

/// §3: a node a job calls owes, beside `/1`'s `call-cost` and `masked-runs`, what the release alarm's §4 says it holds of a primitive besides, each phrase
/// mapped to its fact: so a fact that sentence gains fails here. Before, no line owed either, and the composition gates every composite on the first
/// (`compose.rs`, `primitives-out-of-line`; round 22, RP22-2).
#[test]
fn a_node_a_job_calls_owes_a_primitive_s_other_facts() {
    const OF_A_PRIMITIVE: [(&str, &str); 2] = [
        ("`primitives-out-of-line`", "primitives-out-of-line"),
        (
            "that a call returns with the masking balanced",
            "masking-balanced",
        ),
    ];
    let alarm = include_str!("../../../docs/decisions/decision_release-alarm.md");
    let said = alarm
        .split("It holds a primitive's other facts too, ")
        .nth(1)
        .expect("the release alarm's §4 sentence");
    let said = &said[..said
        .find(", each owed by the realization")
        .expect("its end")];
    let phrases: Vec<&str> = OF_A_PRIMITIVE.iter().map(|(phrase, _)| *phrase).collect();
    assert_eq!(said, phrases.join(" and "), "the facts §4 names");
    let owed = RECORD
        .split("**`(owed …)`, as `/2` writes it.**")
        .nth(1)
        .expect("what `/2` owes");
    let of_a_node = owed
        .split("of a node whose context line holds `job` by `realization`,")
        .nth(1)
        .expect("a called node's");
    let of_a_node = &of_a_node[..of_a_node.find(';').expect("its end")];
    for (_, fact) in OF_A_PRIMITIVE {
        assert!(
            of_a_node.contains(&format!("`{fact}`")),
            "a called node's `{fact}` is owed by no line: {of_a_node}"
        );
    }
}

/// §6: which routes reach the controller is each one's `external`: a source that does not is outside the composition, so a route whose file states `no`
/// refuses the system and one owed counts as reaching it. Every route of a plan then reaches the controller, and [`priority`]'s `rank − 1` routes ahead is
/// exact (round 22, RP22-1).
fn reaching(external: &[Option<bool>]) -> Result<u64, usize> {
    match external.iter().position(|stated| *stated == Some(false)) {
        Some(at) => Err(at),
        None => Ok(u64::try_from(external.len()).expect("few")),
    }
}

/// Over every route list of up to 4 routes, each `external` yes, no or owed: a `no` refuses, naming the first; otherwise every route counts, so the routes
/// ahead of rank `r` number `r − 1`.
#[test]
fn every_route_of_a_plan_reaches_the_controller() {
    let states = [Some(true), Some(false), None];
    for count in 0..=4u32 {
        for choice in 0..3usize.pow(count) {
            let external: Vec<Option<bool>> = (0..count)
                .map(|at| states[choice / 3usize.pow(at) % 3])
                .collect();
            match reaching(&external) {
                Err(at) => assert_eq!(external[at], Some(false), "{external:?}"),
                Ok(routes) => {
                    assert!(
                        external.iter().all(|stated| *stated != Some(false)),
                        "{external:?}"
                    );
                    assert_eq!(routes, u64::from(count));
                    for rank in 1..=routes {
                        let ahead = u64::try_from(
                            external[..usize::try_from(rank - 1).expect("few")].len(),
                        )
                        .expect("few");
                        assert_eq!(priority(8, routes, rank), (routes <= 8).then(|| 8 - ahead));
                    }
                }
            }
        }
    }
    assert!(RECORD.contains(
        "a route whose `external` the file states `no` refuses the system `unsupported-profile`"
    ));
}

/// The ledger: on a target the checker reads a source's whole service group, `external.<source>` among it, from the realization, the generated service in a
/// record's place as an alarm's (`RA-H5`), as the catalog's selection groups it. Before round 27, the records gave the group to a catalog record and
/// `compose.rs` knew no realization as its supplier (RP27-1); round 27 then split `external.<source>` off to the controller's record, which no reader reads
/// it from (round 30, RP30-1).
#[test]
fn a_source_s_service_group_is_read_where_it_is_stated() {
    let row = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H8` | `M2.21.2` |"))
        .expect("the hand-off to the checker's leaf");
    for said in [
        "`C_s`",
        "`external.<source>`",
        "`RA-H5`",
        "`archogen_catalog::selection`",
        "never defaulted",
    ] {
        assert!(row.contains(said), "{said}: {row}");
    }
    let tasks = include_str!("../../../docs/tasks/M2.md");
    assert!(tasks.contains("- [RPF-H8] "), "`M2.21.2` quotes it");
    let h5 = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H5` | `M4.2` |"))
        .expect("the realization's format");
    assert!(
        h5.contains("a source's `external`"),
        "the realization's format states it: {h5}"
    );
}

/// §1: the code identity's catalog digest is of each record's code hash, its lines but its `timing-model`, and no other facet's bound reads a timing
/// model: so no image-specific cost, its value or the image its evidence names, moves the code identity, and each can name the image it was measured on;
/// the one catalog figure the identity holds is the timer's `compare-rounding`, on its own kept line (round 28, RP28-2).
/// Before, the digest held every cost's `binary`, which names an image by an identity that holds the digest, a fixed point no image reaches, so no
/// production claim on a board could stand (round 27, RP27-2).
#[test]
fn the_catalog_digest_holds_no_cost() {
    assert!(RECORD.contains("its code hash, the sha256 of the lines `archogen_catalog::hash`'s `Catalog::hashes` hashes for that record but its `timing-model` line"));
    let hash = include_str!("../../archogen-catalog/src/hash.rs");
    assert!(
        hash.contains("format!(\"timing-model {}\", bound(FacetKind::TimingModel)),"),
        "a line of its own"
    );
    let arms = hash
        .split("match facet {\n            FacetKind::Contract => {")
        .nth(1)
        .expect("the facets' bounds");
    let arms = &arms[..arms
        .find("            FacetKind::TimingModel => {")
        .expect("the timing model's arm")];
    assert!(
        !arms.contains("TimingModel"),
        "a contract's, implementation's or behaviour model's bound reads no timing model"
    );
    let row = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H9` | `M4.10` |"))
        .expect("the hand-off to the catalog's leaf");
    assert!(row.contains("`timing-model`"), "{row}");
    let tasks = include_str!("../../../docs/tasks/M4.md");
    assert!(tasks.contains("- [RPF-H9] "), "`M4.10` quotes it");
}

/// §3 and the ledger: every fact the owed list owes by the catalog, read from the list itself, is one a catalog record is handed to state: by the phrase
/// `RPF-H1` gives it, or, `compare-rounding`, by the catalog record's §12, older than `/2`. Before, a source's `external`, owed by its controller's record
/// since round 27, was in no hand-off, so a builder following `RPF-H1` wrote a record without it and every target plan with a source stayed `(complete
/// no)` (round 28, RP28-1).
#[test]
fn every_fact_the_catalog_owes_is_handed_to_its_records() {
    const BY_RPF_H1: [(&str, &str); 16] = [
        ("trap-frame", "the trap frame"),
        ("guard", "the stack guard"),
        ("alignment", "each stack's and table's alignment"),
        (
            "fault-path-stack",
            "the fault path's from each of its entries",
        ),
        (
            "fault-check-stack",
            "the fault path's from each of its entries",
        ),
        ("idle-stack", "the idle context's"),
        ("trap-path-stack", "the trap path's"),
        ("timer-service-stack", "the timer service's"),
        ("primitive-stack", "each primitive's"),
        ("completion-stack", "the completion path's"),
        ("initialisation-path-stack", "its own initialisation's"),
        ("transition-stack", "the transition's"),
        (
            "table-size",
            "each table a generated system holds as a base and a size",
        ),
        (
            "controller-line",
            "the line of each source and of each alarm wired through it",
        ),
        ("epoch-lead", "the port's `epoch-lead`"),
        ("controller-levels", "a controller's levels"),
    ];
    let h1 = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H1` | `M4.10` |"))
        .expect("the catalog's hand-off");
    let catalog =
        include_str!("../../../docs/specs/catalog/decision_catalog-records-variant-inputs.md");
    let owed = RECORD
        .split("**`(owed …)`, as `/2` writes it.**")
        .nth(1)
        .expect("what `/2` owes");
    let owed = &owed[..owed.find("**Orders.**").expect("its end")];
    let mut found = Vec::new();
    // Each owing by the catalog, to the end of its sentence, clause or line: one sentence's subject is not the last one's fact.
    for (at, _) in owed.match_indices("by `catalog`") {
        let after = &owed[at..];
        let end = [after.find(';'), after.find('\n'), after.find(". ")]
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(after.len());
        for fact in after[..end].split('`').skip(1).step_by(2) {
            let named =
                !fact.is_empty() && fact.chars().all(|c| c.is_ascii_lowercase() || c == '-');
            if named && fact != "catalog" && !found.contains(&fact) {
                found.push(fact);
            }
        }
    }
    for fact in [
        "controller-line",
        "controller-levels",
        "trap-frame",
        "compare-rounding",
        "epoch-lead",
    ] {
        assert!(found.contains(&fact), "the list read `{fact}`: {found:?}");
    }
    for fact in found {
        if fact == "compare-rounding" {
            assert!(
                catalog.contains("`compare-rounding`, a timing cost, not a fact"),
                "catalog §12 has a record state it"
            );
        } else {
            let (_, phrase) = BY_RPF_H1
                .iter()
                .find(|(owed, _)| *owed == fact)
                .unwrap_or_else(|| {
                    panic!("`{fact}` is owed by the catalog and mapped to no hand-off")
                });
            assert!(
                h1.contains(phrase),
                "`{fact}` handed to no record: {phrase}"
            );
        }
    }
    let tasks = include_str!("../../../docs/tasks/M4.md");
    assert!(tasks.contains("- [RPF-H1] "), "`M4.10` quotes it");
    // Who a fact is of, too: a route's `controller-line` is owed of each route that reaches the controller, an alarm's as a source's, one for each channel
    // of its device, so the hand-off names each (round 32, RP32-1).
    assert!(
        RECORD.contains("its alarms take them in order"),
        "an alarm takes a line"
    );
    assert!(
        h1.contains("the line of each source and of each alarm wired through it, one for each channel of an alarm's device"),
        "{h1}"
    );
    // A fact of a service's group comes from the group's supplier, which no catalog record is for a generated service, so the catalog owes none of them
    // (round 30, RP30-1): read from `archogen_catalog::selection`'s own table.
    let selection = include_str!("../../archogen-catalog/src/selection.rs");
    let service = selection
        .split("const SERVICE: [&str; 5] = [")
        .nth(1)
        .expect("the service group");
    let service = &service[..service.find("];").expect("its end")];
    let members: Vec<&str> = service
        .split('"')
        .skip(1)
        .step_by(2)
        .map(|member| member.trim_end_matches('.'))
        .collect();
    assert!(members.contains(&"external"), "{members:?}");
    for (at, _) in owed.match_indices("by `catalog`") {
        let after = &owed[at..];
        let end = [after.find(';'), after.find('\n'), after.find(". ")]
            .into_iter()
            .flatten()
            .min()
            .unwrap_or(after.len());
        for fact in after[..end].split('`').skip(1).step_by(2) {
            assert!(
                !members.contains(&fact),
                "`{fact}` is a service's, owed by the catalog"
            );
        }
    }
}

/// §1: what moves the code identity said as it is. No cost of a timing model moves it but those its kept lines write, the epoch's `epoch-lead`, which the
/// generated code adds where it reads an epoch, and the timer's `compare-rounding`, which the checker reads there and no generated code does, each by value,
/// whose `binary` stands on no kept line (round 34, RP34-1; round 35, RP35-3); and the facts a timing model tables, `eager-switching` among them, move it not, as `archogen_catalog` tables that one
/// in the timing model. Before, the record said no cost moves it and that it moves with any record's facts (round 28, RP28-2); and a `(record ID "HEX")`
/// line named no hash, where each record now has two (RP28-4).
#[test]
fn what_moves_the_code_identity_is_said_as_it_is() {
    for false_said in [
        "so that no cost moves the code identity and",
        "moves with any record's code or facts and with no cost",
    ] {
        assert!(!RECORD.contains(false_said), "{false_said}");
    }
    assert!(RECORD.contains(
        "what the catalog states that the code identity holds is what its kept lines write: the epoch's `epoch-lead` and each route's `(controller LINE PRIORITY)`, which the generated code reads"
    ));
    assert!(RECORD.contains(
        "So does a figure the checker reads there and no generated code does: the timer's `(rounding FIGURE)`, and the lead of `(epoch none (lead FIGURE))`"
    ));
    assert!(RECORD
        .contains("so that no image-specific cost moves the code identity but the epoch's lead"));
    assert!(RECORD.contains("a fixed point an image reaches (RP34-1)"));
    assert!(RECORD.contains("`eager-switching` among them"));
    let record = include_str!("../../archogen-catalog/src/record.rs");
    assert!(
        record.contains("(\"eager-switching\", FacetKind::TimingModel, true),"),
        "tabled in the timing model"
    );
    assert!(
        the_record_s_table().contains(&("timer".to_owned(), Identity::Kept)),
        "the timer's line is kept"
    );
    assert!(RECORD.contains("A `(record ID \"HEX\")` line writes a record's record hash"));
}

/// §3: a subject that is no node is a parenthesized form, which no node's name, a bare symbol, can be: every subject the record declares — the owed line's,
/// `SUBJECT`'s and each owing's in the owed list — is a node's `NODE`, a metavariable whose forms are parenthesized, or a parenthesized form, and no example
/// line writes one of those forms bare. `/1` pins a block named `timer`, and a service named `fault` or `epoch` is a description the check accepts, so a
/// bare `timer`, `epoch` or `fault` named two things under one text (round 29, RP29-1); round 26 removed one such subject, `dispatch`, and left the class.
#[test]
fn no_subject_but_a_node_s_is_a_bare_word() {
    let declared = |subject: &str| {
        subject.starts_with('(') || subject.chars().all(|c| c.is_ascii_uppercase() || c == '-')
    };
    let playground = include_str!("../../../docs/decisions/decision_runtime-plan.md");
    assert!(
        playground.contains("a block named `timer`"),
        "a node may bear such a name"
    );
    let gains = RECORD
        .split("An owed line's subject gains ")
        .nth(1)
        .expect("the owed line's subjects");
    let gains = &gains[..gains.find(" a generated figure is of.").expect("its end")];
    let mut forms: Vec<&str> = gains.split('`').skip(1).step_by(2).collect();
    let subject = RECORD
        .lines()
        .find(|line| {
            line.starts_with(
                "- `SUBJECT`, of what the generated code states, is what that code is of:",
            )
        })
        .expect("`SUBJECT`'s bullet");
    forms.extend(
        subject
            .split('`')
            .collect::<Vec<_>>()
            .windows(2)
            .filter(|pair| pair[1].starts_with(" for "))
            .map(|pair| pair[0]),
    );
    let owed = RECORD
        .split("**`(owed …)`, as `/2` writes it.**")
        .nth(1)
        .expect("what `/2` owes");
    let owed = &owed[..owed.find("**Orders.**").expect("its end")];
    // Each owing that declares its subject, "of `X` by `OWNER`": a subject written in words, a node whose context line holds `job`, is a node's.
    for (at, _) in owed.match_indices("` by `") {
        let before = &owed[..at];
        let open = before.rfind('`').expect("its opening");
        if before[..open].ends_with("of ") || before[..open].ends_with("Of ") {
            forms.push(&before[open + 1..]);
        }
    }
    for must in [
        "(timer)",
        "(epoch)",
        "(fault)",
        "(controller)",
        "NODE",
        "UNIT",
    ] {
        assert!(forms.contains(&must), "`{must}` is declared: {forms:?}");
    }
    for form in &forms {
        assert!(
            declared(form),
            "`{form}` is a bare word a node's name could be: {forms:?}"
        );
    }
    for form in forms
        .iter()
        .filter(|form| form.starts_with('(') && !form.contains(' '))
    {
        let bare = form.trim_matches(|c| c == '(' || c == ')');
        for head in [
            "(owed ",
            "(realization-figure ",
            "(realization-fact ",
            "(realization ",
        ] {
            assert!(
                !RECORD.contains(&format!("{head}{bare} ")),
                "an example writes `{bare}` bare after `{head}`"
            );
        }
    }
}

/// §6 and §3: `(controller owed)` stands until a record states the source's line and the controller's levels, and each is owed apart where none states
/// it. Before, the line was owed whichever was missing, so a stated line was owed and unstated levels were not (round 29, RP29-2).
#[test]
fn the_controller_owes_what_no_record_states() {
    assert!(RECORD.contains(
        "`(owed (controller) controller-levels (by catalog))` where none states its levels"
    ));
    assert!(RECORD.contains(
        "`(owed ITEM controller-line (by catalog))` where none states the source's line"
    ));
}

/// §3 against the composite-inputs record's table: no fact it calls hardware is owed by the realization but as that table has it, "from the record that
/// supplies `service.<source>`", which is the realization for a generated service, an alarm's by `RA-H5` and a source's by `RPF-H8`. Round 29 wrote this row
/// to hold round 27's owner, a source's `external` the controller's record's, against the table it reads (round 30, RP30-1).
#[test]
fn no_hardware_fact_is_owed_by_the_realization_but_as_its_table_has_it() {
    let inputs = include_str!("../../../docs/specs/catalog/decision_runtime-composite-inputs.md");
    let hardware: Vec<&str> = inputs
        .lines()
        .filter_map(|line| {
            let cells: Vec<&str> = line.split(" | ").collect();
            (line.starts_with("| `") && cells.len() > 2 && cells[1].starts_with("hardware"))
                .then(|| cells[0].trim_start_matches("| ").trim_matches('`'))
        })
        .map(|name| name.split('.').next().unwrap_or(name))
        .collect();
    assert!(hardware.contains(&"external"), "{hardware:?}");
    assert!(inputs
        .contains("hardware, one per source, from the record that supplies `service.<source>`"));
    let alarm = include_str!("../../../docs/decisions/decision_release-alarm.md");
    assert!(alarm.contains("anchored by the realization and not by a catalog record (`RA-H5`)"));
    let owed = RECORD
        .split("**`(owed …)`, as `/2` writes it.**")
        .nth(1)
        .expect("what `/2` owes");
    let owed = &owed[..owed.find("**Orders.**").expect("its end")];
    for clause in owed
        .split([';', '\n'])
        .filter(|clause| clause.contains("by `realization`"))
    {
        let at = clause.find("by `realization`").expect("its owner");
        for fact in clause[at..].split('`').skip(1).step_by(2) {
            if hardware.contains(&fact) {
                assert!(
                    fact == "external"
                        && (clause[..at].contains("of an alarm")
                            || clause[..at].contains("of a source")),
                    "`{fact}`, a hardware fact, owed by the realization: {}",
                    &clause[..at.min(200)]
                );
            }
        }
    }
}

/// The book's `/2` section: each owner it names for a fact is the one the record's clause names, each pair read on both sides, so a change of owner in the
/// record fails here until the book moves with it; and it names no owner the record has withdrawn. Before, round 30 moved a source's `external` back to
/// the realization and the section still gave it the controller's catalog record, which no reader reads it from (round 31, RP31-1).
#[test]
fn the_book_s_section_names_each_owner_as_the_record_does() {
    const OWNERS: [(&str, &str); 4] = [
        (
            "whether each reaches it is owed\nuntil the realization, the generated code's own statement, says, for a source as for an alarm",
            "which the realization states with its generated service, a source's as an alarm's",
        ),
        (
            "A size only a catalog record can state stays owed until one does",
            "Until a record states one, it is owed `(by catalog)`",
        ),
        (
            "the catalog's for the\ntimer service and the generated code's own for a generated one",
            "the `service-cost` the realization states of an alarm or a source spans its trap from entry to exit",
        ),
        (
            "and an alarm's lag, says too that it holds under any preemption",
            "each source on a target by `realization`, `holds-under-preemption`",
        ),
    ];
    let book = include_str!("../../../docs/book/src/runtime-plan.md");
    let section = book
        .split("## On a real machine: format `/2`")
        .nth(1)
        .expect("the book's `/2` section");
    let section = &section[..section.find("\n## ").unwrap_or(section.len())];
    for (said, clause) in OWNERS {
        assert!(section.contains(said), "the book says: {said}");
        assert!(RECORD.contains(clause), "the record's clause: {clause}");
    }
    assert!(
        !section.contains("controller's catalog record says, for a source"),
        "an owner the record withdrew"
    );
}

/// The record's quotes of the model are the model's text: each phrase quoted after "model §N rule M" is found in `docs/semantics/model.md`, its spaces
/// and line breaks read as one. Before, §2 quoted rule 8 as "its line and context", where round 30 had made the model read "contexts" (round 31, RP31-4).
#[test]
fn every_quote_of_the_model_is_the_model_s_text() {
    let flat = |text: &str| text.split_whitespace().collect::<Vec<_>>().join(" ");
    let model = flat(include_str!("../../../docs/semantics/model.md"));
    let mut quotes = Vec::new();
    for (at, _) in RECORD.match_indices("model §") {
        let after = &RECORD[at..];
        let Some(open) = after.find('"') else {
            continue;
        };
        let named = &after[..open];
        if open > 40 || named.contains('.') || named.contains(')') {
            continue;
        }
        let quoted = &after[open + 1..];
        let quote = &quoted[..quoted.find('"').expect("it closes")];
        quotes.push(quote);
        assert!(
            model.contains(&flat(quote)),
            "{named}\"{quote}\" is no text of the model"
        );
    }
    assert!(quotes.len() >= 2, "{quotes:?}");
}

/// The book's `/2` section, every sentence of it, in order and whole: each that states a rule is bound to the record's clause it restates, both read,
/// and each other is held descriptive, so a sentence added, changed anywhere or dropped fails here, and so does a clause the record moves. Before, rounds
/// 23, 27, 28, 31 and 32 each found a sentence of the section the record had moved past (round 32, RP32-2, RP32-3); and the row held each sentence's
/// opening alone, so an ending could move unread (round 33, RP33-4).
#[test]
fn every_sentence_of_the_book_s_section_is_bound_to_the_record() {
    const SENTENCES: [(&str, Option<&str>); 19] = [
        ("Everything above describes `archogen-runtime-plan/1`, a system on the hosted playground.", None),
        ("A real machine needs more, and the design of `/2`, `docs/decisions/decision_runtime-plan-format-2.md`, not yet built (`M4.1.7`), says what: which machine it is, by the digest of its description; which device releases tasks and which each alarm takes, with each count written as the device states it, so that the checker alone converts it; where each stack lies; and what the interrupt controller is told.", None),
        ("It also carries the application's inputs.", None),
        ("A size only a catalog record can state stays owed until one does, and every allocation after it is owed too, since no address below it is known.", Some("Until a record states one, it is owed `(by catalog)`")),
        ("The controller is told a line and a priority for each source and each alarm wired through it, in the order the plan ranks them; one whose interrupt bypasses it is outside what the checker composes, so a system holding one is refused, and whether each reaches it is owed until the realization, the generated code's own statement, says, for a source as for an alarm.", Some("which the realization states with its generated service, a source's as an alarm's")),
        ("The generated code around each job's start and each interrupt's dispatch is timed inside the catalog's figures for the switch and the completion, and inside each interrupt service's own cost, the catalog's for the timer service and the generated code's own for a generated one, each measured on the image that holds it.", Some("the dispatch beneath each service inside the timer service's `C_rel` and each service's `C_s`")),
        ("On a machine, each cost that statement gives, and an alarm's lag, says too that it holds under any preemption, as a catalog record's does, and the checker reads it so, never assuming it; the last act's lead needs no such word, as it runs before interrupts are first enabled.", Some("the last act's `last-act-lead` runs before interrupts are first enabled")),
        ("Stacks are laid from the top of a region down and the image from the bottom up, so a change in the image's size moves no allocation.", Some("laid from the top of the region down, by a cursor that only descends")),
        ("A stack's size is what its owners state, by one table of sums: what generated code holds beneath what it calls, a task's own bound, the most the runtime's code adds in a call, what a fault adds on top of the deepest of those, since a failed check or a panic can arrive there, and the frame the port saves.", Some("What runs on a stack runs in turn")),
        ("What the generated code uses of a stack comes of that code, so a first plan generates it with those figures owed, and a second, from a file stating them, generates the same code: what an image holds of its plan leaves out the figures of that code and of the application's, and where each came from, by one table of the plan's lines, and keeps the version of what generated it, on the hosted playground as on a machine.", Some("**Two plans, one code.**")),
        ("A figure the generated code itself reads is another matter, the port's lead, which it adds to its read, and the line and priority it writes to the controller: a plan where either is owed generates no code until a record states it.", Some("the build generates no code of such a plan and refuses it `missing-fact`")),
        ("Time on a machine is counted from a read: the last step of initialisation reads the counter and counts every release from that read plus the port's bound on everything from it to the first enabling of interrupts, so no release stands before interrupts are on, and the checker holds that one write of the timer reaches that far, the port's lead and the timer's rounding added.", Some("One write reaches the first compare")),
        ("What initialisation arms for an alarm it arms before that read, so that nothing of the generated code's own lies between the read and the enabling but the two calls into the port's code, which the port's bound counts, a limit the record states.", Some("The instructions of those two calls are the generated code's own and lie in that span")),
        ("A wake armed there must not come due before interrupts are on, so its period outlasts what the generated code states of that step and the port's bound together, which the checker holds; on the hosted playground the simulator turns interrupts on at time zero, where its wakes are armed, so nothing is held there.", Some("the scheduling checker holds each wake's `P`")),
        ("Every other alarm's compare is given a far-off placeholder in that step, at the farthest point on its compare's step that one write reaches, and the checker holds that it too lies past the enabling, counting each read, and each delay's write, from a tick of its own counter before it and the placeholder a whole step short of what one write reaches.", Some("the generated code writes a placeholder for each alarm that is no wake")),
        ("And an alarm's deadlines are read as its own counter's time, which wraps, where it has no reader or one not bound at an extender over that same device, and as the epoch's where each reader of its time is.", Some("`BASE` is `counter`, the time base being the counter itself, or `epoch`")),
        ("The record's formulas are not left to reading.", None),
        ("A model beside it, `crates/archogen-plan/tests/format2_model.rs`, runs each one: where the image may lie and where each stack is laid, against a reference that tries every address in turn, over every small region; each stack's sum, read from the record's own table, against what can stand on that stack at once; and the read that time is counted from, at every phase of the counter's tick.", None),
        ("Where a rule is already code, what the joint rules refuse of a memory map, the model is held to that code and not to a reading of it.", None),
    ];
    let book = include_str!("../../../docs/book/src/runtime-plan.md");
    let section = book
        .split("## On a real machine: format `/2`")
        .nth(1)
        .expect("the book's `/2` section");
    let section = &section[..section.find("\n## ").unwrap_or(section.len())];
    let flat = section.split_whitespace().collect::<Vec<_>>().join(" ");
    let mut sentences = Vec::new();
    let mut start = 0;
    for (at, _) in flat.match_indices(". ") {
        if flat[at + 2..].starts_with(|c: char| c.is_ascii_uppercase()) {
            sentences.push(&flat[start..=at]);
            start = at + 2;
        }
    }
    sentences.push(&flat[start..]);
    assert_eq!(sentences.len(), SENTENCES.len(), "{sentences:#?}");
    let flat_record = RECORD.split_whitespace().collect::<Vec<_>>().join(" ");
    for (sentence, (whole, clause)) in sentences.iter().zip(SENTENCES) {
        assert_eq!(*sentence, whole, "the book's sentence moved");
        if let Some(clause) = clause {
            assert!(
                flat_record.contains(clause),
                "the record's clause: {clause}"
            );
        }
    }
}

/// §1, §3, §4 and §6: every figure a record states that the generated code reads stands on a kept line, so a figure restated moves the code with it,
/// and a plan where one is owed generates no code, the build refusing it: the epoch's `epoch-lead`, which the last act adds to its read, and each route's
/// controller line and priority, which the code writes. The timer's `compare-rounding` stands on a kept line too, for the checker, and no generated code
/// reads it, the catalog's timer-service code rounding as its implementation does, so a plan where it is owed generates. Before, `E` stood on no line
/// (round 33, RP33-1); a read figure owed left what is generated unsaid (RP34-2); and the controller's line was left out of what the code reads, the
/// rounding put in it, against `RA-H6` (round 35, RP35-2, RP35-3).
#[test]
fn every_figure_the_generated_code_reads_stands_on_a_kept_line() {
    // Each figure a kept line writes of a record: its line's head, its slot in §3's form, the owed form that withholds the code, and whether the code reads it.
    const STATED: [(&str, &str, &str, &str, bool); 3] = [
        (
            "epoch-lead",
            "epoch",
            "(lead FIGURE)",
            "`(epoch after-read (lead owed))`",
            true,
        ),
        (
            "controller-line",
            "route",
            "(controller LINE PRIORITY)",
            "`(controller owed)`",
            true,
        ),
        (
            "compare-rounding",
            "timer",
            "(rounding FIGURE)",
            "`(rounding owed)`",
            false,
        ),
    ];
    let kept = the_record_s_table();
    let row = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H10` | `M4.2` |"))
        .expect("the generated code's hand-off");
    let flat = RECORD.split_whitespace().collect::<Vec<_>>().join(" ");
    let first = &flat[flat
        .find("A first plan with the figures owed")
        .expect("§1's first plan")..];
    let first = &first[..first.find("(RP34-2, RP35-2, RP35-6)").expect("its end")];
    for (figure, line, slot, owed, read) in STATED {
        assert!(
            kept.contains(&(line.to_owned(), Identity::Kept)),
            "`{line}` is kept"
        );
        let form = RECORD
            .lines()
            .find(|text| text.starts_with(&format!("({line} ")) && text.contains(slot))
            .unwrap_or_else(|| panic!("`{line}`'s form holds `{slot}`"));
        assert!(form.contains(slot), "{form}");
        assert!(RECORD.contains(&format!("`{figure}`")), "`{figure}`");
        assert_eq!(row.contains(owed), read, "`RPF-H10` and {owed}: {row}");
        assert_eq!(first.contains(owed), read, "§1 and {owed}: {first}");
    }
    assert!(RECORD
        .contains("It is `compare-rounding` on the timer's line and `epoch-lead` on the epoch's"));
    assert!(RECORD.contains("the `r` it read plus the `E` its line writes"));
    assert!(
        RECORD.contains("The generated code reads `(enabled …)` and `(route …)` and writes them")
    );
    // The code passes the write the line's `E`, writes each route's line and priority, and reads no rounding of the plan (rounds 34 and 35).
    for said in [
        "the `E` its plan's `(epoch after-read …)` line writes",
        "writes each route's line and priority as its `(controller …)` writes them",
        "reads no rounding of the plan, an alarm's compare rounding as `RA-H6` says",
        "refusing it `missing-fact`, naming each such line",
    ] {
        assert!(row.contains(said), "`RPF-H10`: {said}");
    }
    assert!(!RECORD.contains("rounds each compare as its timer line's rounding writes"));
    assert!(RECORD.contains(
        "the catalog's `timer-service` record's code, whose rounding is `compare-rounding`, which the timer's line writes for the checker and no generated code reads"
    ));
    // The checker reads `E` from the epoch's line and from no selection (rounds 34 and 35, RP34-1, RP35-5).
    assert!(!RECORD.contains("The platform's timing figures, `S`, `E`,"));
    assert!(RECORD.contains(
        "The platform's timing figures `S`, `W_wake`, `γ` and `δ` are the catalog's, read by the checker through `(catalog …)`'s selection, and a plan writes none of them."
    ));
    assert!(RECORD.contains(
        "the epoch's line writes wherever the checker holds a lead, and the checker reads it there and nowhere else"
    ));
    let tasks = include_str!("../../../docs/tasks/M4.md");
    assert!(tasks.contains("- [RPF-H10] "), "`M4.2` quotes it");
}

/// §4: the epoch's line as it writes it, of a hosted plan or of a target's with a timer or none and an alarm or none (lifted out of its test by leaf
/// `M4.1.7.6.2`, so the library's is held to it).
fn epoch_line(hosted: bool, timer: bool, alarm: bool, lead: &str) -> String {
    match (hosted, timer, alarm) {
        (true, _, _) => "(epoch simulator)".to_owned(),
        (false, true, _) => format!("(epoch after-read (lead {lead}))"),
        (false, false, true) => format!("(epoch none (lead {lead}))"),
        (false, false, false) => "(epoch none)".to_owned(),
    }
}

/// §3 and §4: the epoch's line carries a lead wherever the checker holds one, and only there: under a timer the lead the last act adds, and under
/// `(timer none)` with an alarm the lead each wake's and placeholder's distance is held against, which no generated code reads; a hosted set is asked
/// none, and with neither timer nor alarm none is held. So the checker reads `E` off one line in every plan, and `(owed (epoch) epoch-lead (by catalog))`
/// stands exactly where that line writes `(lead owed)`. Before, `(timer none)` with an alarm wrote `(epoch none)`, the lead held and on no line, and §3
/// owed it where §4 said it was owed only by a line that never said so (round 35, RP35-1).
#[test]
fn the_epoch_s_line_carries_a_lead_wherever_the_checker_holds_one() {
    let line = epoch_line;
    // Where `RPF-H4` has the checker hold the port's lead: the first compare's hold under a timer, each wake's and placeholder's under a timer or none,
    // and nothing of a hosted set.
    let h4 = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H4` | `M2.21.2` |"))
        .expect("the checker's hand-off");
    for said in [
        "reads the port's `epoch-lead` as its plan's epoch line writes it, under `(timer none)` as under a timer, and no other",
        "a set with `(timer none)` is asked neither",
        "under `(timer none)` as under a timer, rounding `P` down and each lead up",
        "a hosted set is asked none of it",
    ] {
        assert!(h4.contains(said), "`RPF-H4`: {said}");
    }
    let holds = |hosted: bool, timer: bool, alarm: bool| !hosted && (timer || alarm);
    let forms: Vec<&str> = RECORD
        .lines()
        .find(|text| text.starts_with("(epoch simulator) | "))
        .expect("§3's epoch forms")
        .split(" | ")
        .collect();
    let mut written = BTreeSet::new();
    for hosted in [false, true] {
        for timer in [false, true] {
            for alarm in [false, true] {
                for lead in [
                    "owed",
                    "(value 2.4us) (record port.lead) (category measured)",
                ] {
                    let text = line(hosted, timer, alarm, lead);
                    let shape = text
                        .replace("(lead owed)", "(lead FIGURE)")
                        .replace(&format!("(lead {lead})"), "(lead FIGURE)");
                    assert!(
                        forms.contains(&shape.as_str()),
                        "{shape} is one of §3's forms: {forms:?}"
                    );
                    written.insert(shape);
                    assert_eq!(
                        text.contains("(lead "),
                        holds(hosted, timer, alarm),
                        "{text}"
                    );
                    // The owed line stands exactly where the line writes the lead owed, which only a line holding the lead can.
                    let owes = text.contains("(lead owed)");
                    assert_eq!(
                        owes,
                        holds(hosted, timer, alarm) && lead == "owed",
                        "{text}"
                    );
                }
            }
        }
    }
    assert_eq!(
        written.len(),
        forms.len(),
        "every form is written: {written:?}"
    );
    assert!(RECORD.contains(
        "of `(epoch)` by `catalog`, `epoch-lead` (§4), where its line says `(lead owed)`"
    ));
    assert!(RECORD
        .contains("writes `(epoch none)`, and `(epoch none (lead FIGURE))` where it has an alarm"));
    assert!(!RECORD.contains("no lead is owed but where the plan has an alarm"));
}

/// §4: the holds this record adds to the checker's sums are each one `RPF-H4` hands `M2.21.2`, the first compare's, each wake's, each placeholder's and the
/// refusal of a window or a step past `2^64 − 1` ticks, and the sentence that counts them names each. Before, it said it added one, the first compare's,
/// where `RPF-H4` and §4 itself add four (round 36, RP36-3).
#[test]
fn the_holds_this_record_adds_are_those_rpf_h4_hands_the_checker() {
    // Each hold, as §4's sentence names it and as `RPF-H4` writes it.
    const ADDED: [(&str, &str); 4] = [
        ("the first compare's above", "plus the release timer's `ρ` within what one write reaches on that timer's device"),
        ("each wake's `P`", "it holds each wake's `P`, its `step`, at no less than the realization's `last-act-lead` plus the port's `epoch-lead`"),
        ("each placeholder's distance", "it holds each alarm that is no wake the same way, its placeholder's distance in place of `P`"),
        ("a window or a step past `2^64 − 1` ticks refused `unsupported-profile`", "it refuses a window or a step past `2^64 − 1` ticks"),
    ];
    let h4 = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H4` | `M2.21.2` |"))
        .expect("the checker's hand-off");
    let flat = RECORD.split_whitespace().collect::<Vec<_>>().join(" ");
    let at = flat
        .find("The checker's sums** are `RA-H2`'s")
        .expect("§4's sums");
    let sentence = &flat[at..at + flat[at..].find("(RP36-3)").expect("its end")];
    assert!(
        sentence.contains("this record adds four holds"),
        "{sentence}"
    );
    for (named, handed) in ADDED {
        assert!(sentence.contains(named), "§4 names {named}");
        assert!(h4.contains(handed), "`RPF-H4`: {handed}");
    }
    assert!(!RECORD.contains("and this record adds one, the first compare's above"));
}

/// Every refusal the record states has its stage: the search's and the checker's at `check` and `resolve`, the checker's of a plan, `M4.4`'s or the
/// build's, or the lowering's at a place §7's order names, so no refusal is two outcomes of one input. Before, §5 gave a map finding the joint rules' own
/// code and §7 added "`missing-fact` from the joint rules" beside `/1`'s kept `tool-failure` for the same finding (round 33, RP33-2; RP7-4 and RP23-3's
/// class).
#[test]
fn every_refusal_the_record_states_has_its_stage() {
    const STAGES: [(&str, &str); 41] = [
        ("Its refusals come in the order it reads a plan in today (`DC-H2`)", "the checker"),
        ("An assignment of the other kind, a `/3` lowered or checked with no `--target` or a `/2` with one, is `stale-input`", "lowering: an assignment of the other kind"),
        ("A record whose target's digest differs, or of the other kind as above, is `stale-input`", "the checker"),
        ("one naming another platform beside the given file's digest is `tool-failure`, in the judgement", "the checker"),
        ("its `HEX` decides: one other than the given file's is another run's, `stale-input`", "lowering: the assignment's `(target …)` digest other than the file's"),
        ("A `NAME` other than the one platform of the file whose `HEX` it writes is a record no run writes", "lowering: its platform other than the file's beside the file's digest"),
        ("`(compare-magnitude true)` beside a window is `invalid-description`, naming the device", "lowering: a target's own refusals"),
        ("**No room** is `infeasible-configuration`", "lowering: no room while placing"),
        ("as it holds `SHORTEST + 2ρ`, failing `unsupported-profile` and naming it", "the checker"),
        ("a machine the profile refuses as the profile says", "lowering: a target's own refusals"),
        (
            "and the build generates no code of a plan whose epoch line writes `(epoch after-read (lead owed))`",
            "the build",
        ),
        ("the build generates no code of such a plan and refuses it `missing-fact`", "the build"),
        ("which is no refusal, the figures being what such a build makes", "none"),
        ("The build refuses a plan before anything of the application's crate", "the build"),
        ("a system with placement memory staying refused as `/1` refuses it", "lowering: `/1`'s list in its order"),
        ("a lowering given one of another code refuses it `stale-input`", "lowering: a realization of another code identity"),
        ("a file of the one is refused by a lowering of the other", "lowering: a realization of another code identity"),
        ("each other thing `unmet` reports refused `invalid-description`", "lowering: an application's unmet"),
        ("being the lowering's to judge on a target and no refusal there", "none"),
        ("a target with no placement memory is refused `infeasible-configuration`", "lowering: a target's own refusals"),
        ("A target that does not state a fact a system needs is refused `missing-fact` by what asks", "check and resolve"),
        ("a target stating either above zero is refused `unsupported-profile`", "lowering: a target's own refusals"),
        ("one silent on either is refused `missing-fact`", "lowering: a target's own refusals"),
        ("one silent is refused `missing-fact`, naming it (`M4.1.7.4.3`)", "lowering: a target's own refusals"),
        ("This refuses too a target whose non-maskable interrupts are all unresumable", "lowering: a target's own refusals"),
        ("a device stating none has its system refused `missing-fact` before any plan", "check and resolve"),
        ("past `2^64 − 1` ticks refused `unsupported-profile`", "the checker"),
        ("A device whose compare could be disarmed and whose reach is shorter than those is refused", "the checker"),
        ("at a reach of 2 ms a lead of 3 ms is refused", "the checker"),
        ("its findings refuse at `check` and `resolve`", "lowering: a finding of the generator's joint rules over the target's hardware"),
        ("the system is then refused for no room", "lowering: no room while placing"),
        ("which `M4.4` refuses naming the hole", "M4.4"),
        ("the model's refusals are held to that code itself", "none"),
        ("`Joint::map` refuses more", "check and resolve"),
        ("a route whose `external` the file states `no` refuses the system `unsupported-profile`", "lowering: a route whose `external` the realization's file states `no`"),
        ("Where the routes outnumber the levels the system is refused `unsupported-profile`", "lowering: too few levels while lowering the plan's code"),
        ("**A refusal `/1` makes that `/2` does not:**", "none"),
        ("A bound or a list of counts whose `binary` names another image is refused `stale-input`", "the build"),
        ("is refused naming the size it would need where a hole truncates it", "M4.4"),
        ("is refused by a lowering of another's", "lowering: a realization of another code identity"),
        ("it refuses a window or a step past `2^64 − 1` ticks", "the checker"),
    ];
    const EXEMPT: [&str; 5] = [
        "if base < F: refuse",
        "which may refuse what holds",
        "and the refusal says so",
        "so a second fact refused the target it was for",
        "is written owed and refused by nothing",
    ];
    let rules = &RECORD[..RECORD.find("\n## Review\n").expect("the review")];
    let flat = rules.split_whitespace().collect::<Vec<_>>().join(" ");
    let order = "A plan on a target adds those above, in this record's order";
    let sentence = &flat[flat.find(order).expect("§7's order")..];
    for (said, stage) in STAGES {
        assert!(flat.contains(said), "the record says: {said}");
        if let Some(place) = stage.strip_prefix("lowering: ") {
            assert!(
                sentence.contains(place),
                "`{place}` has its place in §7's order"
            );
        }
    }
    let mut start = 0;
    let mut pieces = Vec::new();
    for (at, _) in flat.match_indices(". ") {
        pieces.push(&flat[start..=at]);
        start = at + 2;
    }
    pieces.push(&flat[start..]);
    // A sentence that withholds what a stage makes, code or an image, is a stage's outcome as a refusal is: before, the stage row read "refus" alone, and
    // "generates no code" and "links no image" stood with no verdict and no place (round 35, RP35-6). So is one that names a status as what happens, each
    // of the vocabulary's own statuses but `ok`: before, "`(compare-magnitude true)` beside a window is `invalid-description`" said no "refus" and had no
    // stage, where it decides whether a system on that target lands (round 38, RP38-2).
    let statuses: Vec<String> = archogen_outcome::Status::ALL
        .iter()
        .filter(|status| status.slug() != "ok")
        .map(|status| format!("`{}`", status.slug()))
        .collect();
    for piece in pieces.iter().filter(|piece| {
        (["refus", "generates no", "links no"]
            .iter()
            .any(|word| piece.contains(word))
            || statuses
                .iter()
                .any(|status| piece.contains(status.as_str())))
            && !piece.contains(order)
    }) {
        assert!(
            STAGES.iter().any(|(said, _)| piece.contains(said))
                || EXEMPT.iter().any(|said| piece.contains(said)),
            "a refusal with no stage: {piece}"
        );
    }
}

/// §3, `R`: a rate in hertz, exact, a whole number or `N/D` in lowest terms, as `/1` writes a duration (round 4, RP4-7).
fn rate(numerator: u64, denominator: u64) -> String {
    let (mut a, mut b) = (numerator, denominator);
    while b != 0 {
        (a, b) = (b, a % b);
    }
    match denominator / a {
        1 => (numerator / a).to_string(),
        over => format!("{}/{over}", numerator / a),
    }
}

/// §3, "Orders": `region` lines by base, then name, then provider, so two blocks' regions of one name at one base have an order (round 4, RP4-7).
fn in_the_plan_s_order<'r>(
    mut regions: Vec<(u64, &'r str, &'r str)>,
) -> Vec<(u64, &'r str, &'r str)> {
    regions.sort_unstable();
    regions
}

#[test]
fn a_rate_and_a_region_have_one_text_and_one_place() {
    assert_eq!(
        [
            rate(10_000_000, 1),
            rate(3, 2),
            rate(6, 4),
            rate(32_768, 1),
            rate(1, 3)
        ],
        ["10000000", "3/2", "3/2", "32768", "1/3"]
    );
    let written = vec![
        (4096, "hole", "fw.b"),
        (4096, "hole", "fw.a"),
        (0, "ram", "soc"),
        (4096, "boot", "fw.b"),
    ];
    let mut back = written.clone();
    back.reverse();
    assert_eq!(
        in_the_plan_s_order(written.clone()),
        in_the_plan_s_order(back)
    );
    assert_eq!(
        in_the_plan_s_order(written),
        [
            (0, "ram", "soc"),
            (4096, "boot", "fw.b"),
            (4096, "hole", "fw.a"),
            (4096, "hole", "fw.b")
        ]
    );
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
    ("emitter", Identity::Kept),
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
    ("clock", Identity::Kept),
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

/// [`preimage`], the library's held to it, and the library's identity the sha256 of it (leaf `M4.1.7.6.2`).
fn preimage_held(plan: &str) -> String {
    let model = preimage(plan);
    assert_eq!(
        archogen_plan::identity::preimage(plan).as_deref(),
        Ok(model.as_str()),
        "the library's preimage of {plan}"
    );
    assert_eq!(
        archogen_plan::identity::code_identity(plan),
        Ok(archogen_digest::Digest::of(model.as_bytes()))
    );
    model
}

/// The library's table of lines as the model's: each head, and what the identity does with it.
fn library_lines() -> Vec<(String, Identity)> {
    archogen_plan::identity::LINES
        .iter()
        .map(|(head, treatment)| {
            (
                (*head).to_owned(),
                match treatment {
                    archogen_plan::identity::Treatment::Kept => Identity::Kept,
                    archogen_plan::identity::Treatment::LeftOut => Identity::LeftOut,
                    archogen_plan::identity::Treatment::EntryPoint => Identity::EntryPoint,
                },
            )
        })
        .collect()
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
    assert_eq!(
        library_lines(),
        model,
        "the library's table is the model's (leaf `M4.1.7.6.2`)"
    );
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
/// code moves it: an entry point, the application's sources, the catalog, a route's level. Every line here is a placeholder of its form and none a whole plan's,
/// which no lowering writes yet, the lines kept as much as those the identity leaves out, which stand for their heads alone: the row asks only which heads move
/// the identity, and a line left out moves none whatever it holds (rounds 16 and 17, RP16-3, RP17-3).
#[test]
fn two_plans_of_one_code_have_one_preimage_and_code_moves_it() {
    let shared_by = |entry: &str, digest: &str, catalog: &str, level: u64, emitter: u64| {
        vec![
            "(format \"archogen-runtime-plan/2\")".to_owned(),
            "(profile rt-static-up-v1)".to_owned(),
            "(system heartbeat)".to_owned(),
            "(target target.riscv-virt-up \"aa\")".to_owned(),
            format!("(emitter archogen-emit/{emitter})"),
            format!("(catalog \"{catalog}\" riscv-virt-up)"),
            format!("(code app \"../app\" \"sha256:{digest}\")"),
            "(task 0 beat (period 1/100 s))".to_owned(),
            format!("(route 1 (source uart.rx) (controller 10 {level}))"),
            format!("(task-inputs (task heartbeat beat) (entry-point \"{entry}\") (stack-bound (value 512) (unit byte)))"),
            "(initialisation-inputs none)".to_owned(),
        ]
    };
    let shared = |entry: &str, digest: &str, catalog: &str, level: u64| {
        shared_by(entry, digest, catalog, level, 1)
    };
    let first = |lines: Vec<String>| {
        let mut plan = lines;
        plan.insert(5, "(application \"bb\")".to_owned());
        plan.extend(["(allocation (stack boot) owed (needs (realization (step 1) initialisation-stack)))", "(image (floor 2147483648) (limit owed) (truncated-by))", "(owed (step 1) initialisation-stack (by realization))", "(complete no)"].map(str::to_owned));
        plan.join("\n") + "\n"
    };
    let second = |lines: Vec<String>| {
        let mut plan = lines;
        plan.insert(5, "(application \"cc\")".to_owned());
        plan.insert(6, "(realization \"dd\")".to_owned());
        plan.insert(7, "(record port.riscv \"ee\")".to_owned());
        plan.extend(["(declared leaves-interrupt-hardware-alone yes (clause \"…\") (basis \"it touches no device\"))", "(realization-figure (step 1) initialisation-stack (value 128) (unit byte))", "(realization-fact (source uart.rx) acknowledge-point entry)", "(allocation (stack boot) (base 2281700288) (guard 64) (bytes 1024) (align 16) (origin (record port.riscv guard assumed)))", "(image (floor 2147483648) (limit 2281700288) (truncated-by))", "(complete yes)"].map(str::to_owned));
        plan.join("\n") + "\n"
    };
    let one = preimage_held(&first(shared("app::beat", "00", "c1", 7)));
    assert_eq!(
        one,
        preimage_held(&second(shared("app::beat", "00", "c1", 7)))
    );
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
        // RP4-2: an engine rebuilt with another emitter generates other code of the same plan, and its version is a line the identity keeps.
        shared_by("app::beat", "00", "c1", 7, 2),
    ] {
        assert_ne!(preimage_held(&first(moved)), one);
    }
    let named = preimage_held("(initialisation-inputs (entry-point \"app::init\") (stack-bound (value 256) (unit byte)) (node-calls (call console.write 1)))\n");
    assert_eq!(
        named,
        "(initialisation-inputs (entry-point \"app::init\"))\n"
    );
}

// ---- round 5: an alarm's time base, a wake's first point, a hosted plan's lines

/// Where a reader of an alarm's time is bound, as §3's `BASE` reads it off the plan's bindings.
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Reader {
    /// At the alarm's own device.
    AtTheDevice,
    /// At an `epoch-extender` over the alarm's own device.
    ExtendedOverIt,
    /// On another device.
    OnAnother,
    /// At an `epoch-extender` over another device.
    ExtendedOverAnother,
}

const READERS: [Reader; 4] = [
    Reader::AtTheDevice,
    Reader::ExtendedOverIt,
    Reader::OnAnother,
    Reader::ExtendedOverAnother,
];

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Base {
    Counter,
    Epoch,
}

/// §3, `BASE`: a wake's is `counter`; any other alarm's is `epoch` where it has a reader of time and each is bound at an `epoch-extender` over the alarm's own
/// device, and `counter` otherwise.
fn base(wake: bool, readers: &[Reader]) -> Base {
    if !wake
        && !readers.is_empty()
        && readers
            .iter()
            .all(|reader| *reader == Reader::ExtendedOverIt)
    {
        Base::Epoch
    } else {
        Base::Counter
    }
}

/// Why: an alarm compares on its own device's counter, and a value written to that compare is unambiguous without `R`, `room` and `RA-H7`'s statement only where it
/// was read as that counter's own extended time. A value read of the raw counter wraps with it, and one read of another clock, extended or not, says nothing of
/// this counter's wraps.
fn reads_this_counter_s_extended_time(reader: Reader) -> bool {
    reader == Reader::ExtendedOverIt
}

/// §3's sentence as round 4 left it, read the other way round 5 found it could be: "`counter` where any is bound at the device itself, or there is none", and
/// `epoch` of everything else.
fn base_as_the_second_clause_read(readers: &[Reader]) -> Base {
    if readers.is_empty() || readers.contains(&Reader::AtTheDevice) {
        Base::Counter
    } else {
        Base::Epoch
    }
}

/// Every set of up to three readers has one `BASE`, and it is `epoch` only where every value its compare is written with is that counter's own extended time
/// (round 5, RP5-1). The sentence's other reading gave `epoch` on a raw counter: a reader on another device, or at an extender over another.
#[test]
fn an_alarm_s_base_is_the_counter_unless_each_reader_is_extended_over_its_own_device() {
    let mut sets: Vec<Vec<Reader>> = vec![Vec::new()];
    for a in READERS {
        sets.push(vec![a]);
        for b in READERS {
            sets.push(vec![a, b]);
            for c in READERS {
                sets.push(vec![a, b, c]);
            }
        }
    }
    let mut the_other_reading_differs = Vec::new();
    for readers in &sets {
        let safe_as_epoch = !readers.is_empty()
            && readers
                .iter()
                .all(|reader| reads_this_counter_s_extended_time(*reader));
        assert_eq!(
            base(false, readers) == Base::Epoch,
            safe_as_epoch,
            "{readers:?}"
        );
        assert_eq!(base(true, readers), Base::Counter, "a wake's, {readers:?}");
        if base_as_the_second_clause_read(readers) != base(false, readers) {
            assert_eq!(base(false, readers), Base::Counter, "{readers:?}");
            the_other_reading_differs.push(readers.clone());
        }
    }
    // Its reader's two descriptions: a time base on another device, and one at an extender over another.
    assert!(the_other_reading_differs.contains(&vec![Reader::OnAnother]));
    assert!(the_other_reading_differs.contains(&vec![Reader::ExtendedOverAnother]));
    assert_eq!(
        base(false, &[Reader::ExtendedOverIt, Reader::ExtendedOverIt]),
        Base::Epoch
    );
    assert_eq!(
        base(false, &[Reader::ExtendedOverIt, Reader::AtTheDevice]),
        Base::Counter
    );
    // The record's sentence is this rule's: one test and an otherwise, which no set of readers falls between.
    assert!(RECORD.contains(
        "It is `epoch` where there is such a reader and each is bound at an `epoch-extender` over the alarm's own device, and `counter` otherwise: where any is bound at the device itself, on another device or at an extender over another, or there is none"
    ));
}

/// The facts of the function `counter` that are no reader's of a time base though no requester consumes them: those `RA-H8` adds, which describe a compare and
/// what it is wired to, held to the release alarm's own record by `the_facts_that_are_no_reader_s_are_the_release_alarm_s_own_five`.
const DESCRIBES_A_COMPARE: [&str; 5] = [
    "alarm-wiring",
    "alarm-kind",
    "compare-step",
    "compare-window",
    "compare-magnitude",
];

/// §3 reads a reader of time as bound "for a fact of the function `counter` that is not consumed": in the shipped vocabulary the consumed facts of that function
/// are the two a requester programs a compare by, so being bound for a deadline makes no reader; and the four `RA-H8` adds, which cannot be consumed, the record
/// names, none being in the vocabulary before `M4.1.7.3` (round 5, RP5-1).
#[test]
fn what_programs_a_compare_is_consumed_and_what_describes_one_is_named() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|relative| {
        (
            (*relative).to_string(),
            std::fs::read_to_string(root.join(relative)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the shipped kinds read");
    let vocabulary = Vocabulary::shipped(&mut SourceMap::new(), &registry)
        .expect("the shipped vocabulary reads");
    let of_the_counter = |fact: &&eadl_resolve::vocabulary::Fact| {
        fact.function == Some(eadl_resolve::vocabulary::Function::Of("counter".to_owned()))
    };
    let consumed: BTreeSet<&str> = vocabulary
        .facts()
        .filter(of_the_counter)
        .filter(|fact| fact.usage == eadl_resolve::vocabulary::Usage::Consumed)
        .map(|fact| fact.name.as_str())
        .collect();
    assert_eq!(
        consumed,
        BTreeSet::from(["absolute-deadline", "relative-delay"])
    );
    for named in DESCRIBES_A_COMPARE {
        // None is in the vocabulary before `M4.1.7.3`; one that has joined it is the counter's and is not consumed, which is why the record names it.
        if let Some(fact) = vocabulary.fact(named) {
            assert!(
                of_the_counter(&fact) && fact.usage == eadl_resolve::vocabulary::Usage::Shared,
                "`{named}`"
            );
        }
        assert!(
            RECORD.contains(&format!("`{named}`")),
            "the record names `{named}`"
        );
    }
    assert!(RECORD
        .contains("and is none of those `RA-H8` has rule 14 read as no reader of a time base"));
}

/// §4, "A wake's first point": the last act arms a wake before it reads the counter for the epoch, for `P` past the wake's own read. Whether a set is admitted
/// for it: `P` no less than the realization's `last-act-lead`, from a tick of that read's own counter before it to the act's read for the epoch, or under
/// `(timer none)` to its call of the port's transition, and the port's `epoch-lead`, from there to the first enabling (round 13, RP13-6).
fn a_wake_s_step_outlasts_the_last_act(step: u64, act_lead: u64, lead: u64) -> bool {
    step >= act_lead + lead
}

/// Over every small tick, bound and phase: where the condition holds a wake's first point stands at or after the first enabling, so no wake arrives in
/// initialisation; and where it does not, some phase has it due before (round 5, RP5-2).
#[test]
fn a_wake_s_first_point_stands_at_or_after_the_first_enabling_where_the_condition_holds() {
    for wake_tick in 1..=3_u64 {
        for tick in 1..=3_u64 {
            for arms_bound in 0..=3_u64 {
                for write_bound in 0..=2_u64 {
                    for transition_bound in 0..=2_u64 {
                        // What each record bounds: the realization's from a wake's read, up to its counter's tick old, to the act's read; the port's from that read on.
                        let (act_lead, lead) = (
                            wake_tick + arms_bound,
                            tick + write_bound + transition_bound,
                        );
                        let mut least = None;
                        for step in 0..=act_lead + lead + 2 {
                            let mut early = false;
                            for armed_at in 0..2 * wake_tick {
                                for arms in 0..=arms_bound {
                                    for write in 0..=write_bound {
                                        for transition in 0..=transition_bound {
                                            // The wake's read is the tick at or before the instant it is taken, and its first point `P` past that.
                                            let point = armed_at - armed_at % wake_tick + step;
                                            let enabling = armed_at + arms + write + transition;
                                            early |= point < enabling;
                                        }
                                    }
                                }
                            }
                            if a_wake_s_step_outlasts_the_last_act(step, act_lead, lead) {
                                assert!(!early, "{wake_tick} {tick} {arms_bound} {write_bound} {transition_bound} {step}");
                            }
                            if !early && least.is_none() {
                                least = Some(step);
                            }
                        }
                        // And it asks little more than what can happen: the least step that is never early is short of the condition's by the port's own
                        // tick, which the epoch's read needs and a wake's does not, and by one unit, a read being under a tick old.
                        assert_eq!(
                            act_lead + lead - least.expect("some step is never early"),
                            tick + 1
                        );
                    }
                }
            }
        }
    }
    // RP5-2's case: §4's lead of 2.40 µs, and a wake of `P` 2 µs armed before the read, in hundredths of a microsecond.
    assert!(!a_wake_s_step_outlasts_the_last_act(200, 0, 240));
    assert!(a_wake_s_step_outlasts_the_last_act(240 + 100, 100, 240));
    // The record's sentence is this condition's, and the hand-off's.
    assert!(RECORD.contains("the scheduling checker holds each wake's `P` at no less than `last-act-lead` plus the port's `epoch-lead`"));
    assert!(RECORD.contains("it holds each wake's `P`, its `step`, at no less than the realization's `last-act-lead` plus the port's `epoch-lead`"));
}

/// The heads a hosted plan writes beyond `/1`'s, from the record's own table after `<!-- machine-read: hosted-lines -->`.
fn the_hosted_lines() -> Vec<String> {
    let after = RECORD
        .split("<!-- machine-read: hosted-lines -->")
        .nth(1)
        .expect("the record marks what a hosted plan writes");
    after
        .lines()
        .skip_while(|line| !line.starts_with('|'))
        .take_while(|line| line.starts_with('|'))
        .skip(2)
        .flat_map(|row| {
            let cell = row
                .trim_matches('|')
                .split('|')
                .next()
                .expect("a first cell")
                .to_owned();
            cell.split(',')
                .map(|head| head.trim().trim_matches('`').to_owned())
                .collect::<Vec<_>>()
        })
        .collect()
}

/// What a hosted plan has none of, §1: "none of §5's placement, allocations or image".
const NOT_HOSTED: [&str; 3] = ["placement", "allocation", "image"];

/// A hosted plan's code is generated too, so every line the identity keeps is one a hosted plan writes: as `/1` writes it, by the heads its goldens hold, or
/// among those the record lists beyond `/1`'s, but for what a hosted plan has none of. `(emitter …)` was kept and in neither (round 5, RP5-3).
#[test]
fn every_line_the_identity_keeps_is_one_a_hosted_plan_writes() {
    let mut as_one_writes: BTreeSet<String> = BTreeSet::new();
    for plan in [
        include_str!("goldens/epoch-extender.plan"),
        include_str!("goldens/heartbeat.plan"),
        include_str!("goldens/sporadic.plan"),
    ] {
        as_one_writes.extend(
            plan.lines()
                .filter(|line| line.starts_with('('))
                .map(|line| head(line).to_owned()),
        );
    }
    let hosted = the_hosted_lines();
    for listed in &hosted {
        assert!(
            LINES.iter().any(|(known, _)| known == listed),
            "the record lists `({listed} …)` of a hosted plan, which §3's forms do not write"
        );
        assert!(!NOT_HOSTED.contains(&listed.as_str()), "`{listed}`");
    }
    for (kept, does) in LINES {
        if *does == Identity::LeftOut || NOT_HOSTED.contains(kept) {
            continue;
        }
        assert!(
            as_one_writes.contains(*kept) || hosted.iter().any(|listed| listed == kept),
            "the identity keeps `({kept} …)`, and a hosted plan writes it neither as `/1` does nor by the record's list"
        );
    }
    assert!(hosted.iter().any(|listed| listed == "emitter"));
}

// ---- round 6: a hosted line's items, the facts that are no reader's, a hosted wake, and which way a step rounds

/// `/1`'s golden plans, the corpus a hosted plan's lines are held against.
const GOLDEN_PLANS: [&str; 3] = [
    include_str!("goldens/epoch-extender.plan"),
    include_str!("goldens/heartbeat.plan"),
    include_str!("goldens/sporadic.plan"),
];

/// A line that holds a route's item is not as `/1` writes it: in `/2` an alarm's item is `(alarm NODE)` wherever it stands (§3). So each head the identity keeps
/// under which `/1`'s goldens write an item stands in the record's table of what a hosted plan writes otherwise. The row before this compared heads alone and took
/// `enabled`, `context` and `obligation` for `/1`'s, two texts and two code identities of one plan (round 6, RP6-1).
#[test]
fn every_kept_line_that_holds_an_item_is_one_a_hosted_plan_writes_otherwise() {
    let hosted = the_hosted_lines();
    let mut holding: BTreeSet<String> = BTreeSet::new();
    for plan in GOLDEN_PLANS {
        holding.extend(
            plan.lines()
                .filter(|line| line.contains("(timer-route "))
                .map(|line| head(line).to_owned()),
        );
    }
    for expected in ["route", "enabled", "context", "obligation"] {
        assert!(
            holding.contains(expected),
            "`/1`'s goldens write an item under `({expected} …)`: {holding:?}"
        );
    }
    for under in &holding {
        let kept = LINES
            .iter()
            .any(|(known, does)| known == under && *does != Identity::LeftOut);
        assert!(
            !kept || hosted.contains(under),
            "`({under} …)` holds a route's item, which `/2` writes otherwise, and the record's table of a hosted plan's lines lacks it"
        );
    }
    // And a hosted route ends in no `(controller …)`: §1 and §6 say so in one sentence each.
    assert!(RECORD.contains("a hosted route writes no `(controller …)`"));
    assert!(RECORD.contains("On a target each route that reaches a controller writes `(controller LINE PRIORITY)` last on its line"));
}

const RELEASE_ALARM: &str = include_str!("../../../docs/decisions/decision_release-alarm.md");

/// The names a text writes between backticks that are made as a fact's is.
fn facts_named(text: &str) -> BTreeSet<String> {
    text.split('`')
        .skip(1)
        .step_by(2)
        .filter(|name| !name.is_empty() && name.chars().all(|c| c.is_ascii_lowercase() || c == '-'))
        .map(str::to_owned)
        .collect()
}

/// The facts `RA-H8` adds are read off the release alarm's own record, never listed here from memory: those its hand-off names before "join the vocabulary", and
/// those the first cell of a row of its inputs' table names where the row says the fact is `RA-H8`'s. Round 5 listed four, and the record has five, an alarm's kind
/// among them, so a service stating one read as a reader of time at the device itself (round 6, RP6-2).
#[test]
fn the_facts_that_are_no_reader_s_are_the_release_alarm_s_own_five() {
    let hand_off = RELEASE_ALARM
        .lines()
        .find(|line| line.starts_with("| `RA-H8` |"))
        .expect("the release alarm's hand-off");
    let joining = hand_off
        .split("join the vocabulary")
        .next()
        .expect("what joins")
        .splitn(4, '|')
        .nth(3)
        .expect("its obligation");
    let mut named = facts_named(joining);
    for row in RELEASE_ALARM
        .lines()
        .filter(|line| line.starts_with("| `") && line.contains("(`RA-H8`)"))
    {
        named.extend(facts_named(row.split('|').nth(1).expect("a first cell")));
    }
    let model: BTreeSet<String> = DESCRIBES_A_COMPARE
        .iter()
        .map(|&name| name.to_owned())
        .collect();
    assert_eq!(named, model);
    assert_eq!(model.len(), 5);
    for name in &model {
        assert!(
            RECORD.contains(&format!("`{name}`")),
            "the record names `{name}`"
        );
    }
    assert!(RECORD.contains(
        "`alarm-wiring`, `alarm-kind`, `compare-step`, `compare-window` and `compare-magnitude`"
    ));
}

/// §4: on the hosted playground the simulator enables at time zero and a wake is armed there, so its first point is `P` past the enabling whatever `P` is, and
/// nothing is held of it (round 6, RP6-3).
#[test]
fn a_hosted_wake_has_nothing_to_outlast() {
    for step in 0..=8_u64 {
        let (enabling, armed_at) = (0_u64, 0_u64);
        assert!(armed_at + step >= enabling);
        assert!(a_wake_s_step_outlasts_the_last_act(step, 0, 0));
    }
    assert!(RECORD.contains("A hosted plan owes neither figure and is held to nothing here"));
    assert!(
        RECORD.contains("a hosted set is asked none of it, the simulator enabling at time zero")
    );
}

/// §4: in the hold of a wake's `P` the step rounds down and each lead up, to whatever unit the checker counts in: then the hold of what was rounded is the hold
/// of what was stated. Rounded up, as the inputs' table rounds `P` where a sum is charged with it, a wake due before the enabling is admitted (round 6, RP6-4).
#[test]
fn a_wake_s_step_rounds_down_where_a_sum_lies_within_it() {
    let down = |value: u64, unit: u64| value - value % unit;
    let up = |value: u64, unit: u64| value.div_ceil(unit) * unit;
    let mut rounded_up_admits_an_early_wake = false;
    for unit in 1..=8_u64 {
        for step in 0..=40_u64 {
            for act_lead in 0..=12_u64 {
                for lead in 0..=12_u64 {
                    let stated = a_wake_s_step_outlasts_the_last_act(step, act_lead, lead);
                    if a_wake_s_step_outlasts_the_last_act(
                        down(step, unit),
                        up(act_lead, unit),
                        up(lead, unit),
                    ) {
                        assert!(stated, "{unit} {step} {act_lead} {lead}");
                    }
                    rounded_up_admits_an_early_wake |= !stated
                        && a_wake_s_step_outlasts_the_last_act(
                            up(step, unit),
                            up(act_lead, unit),
                            up(lead, unit),
                        );
                }
            }
        }
    }
    assert!(rounded_up_admits_an_early_wake);
    assert!(RECORD.contains("`P` rounds down in that hold and each lead up"));
}

// ---- round 7: the last act's first read, a hosted plan's owed lines, each alarm's route

/// What the last act arms or writes before its read for the epoch: a wake, `P` past its own read, or a placeholder, written at the farthest one write reaches on
/// its device, `write`, past its own read (§4).
#[derive(Clone, Copy)]
struct Written {
    read_at: u64,
    distance: u64,
}

/// §4: `last-act-lead` is the realization's bound on everything from the first read any arm or placeholder of the last act takes, up to a tick of that counter old,
/// to the act's read for the epoch, or under `(timer none)` to its call of the port's transition; `epoch-lead` the port's from there to the first enabling. Whether
/// a set is admitted: each distance no less than the two (round 13, RP13-6).
fn the_last_act_is_held(written: &[Written], act_lead: u64, lead: u64) -> bool {
    written.iter().all(|each| each.distance >= act_lead + lead)
}

/// When what was written comes due: its distance past the counter's value read at `read_at`, which is up to a tick old.
fn comes_due(each: &Written, tick: u64) -> u64 {
    each.read_at - each.read_at % tick + each.distance
}

/// RP7-1's small universe, each as `[tick, write, transition, first, second, act]`: the counter's tick, the port's write and transition, the last act's first
/// and second reads, and its read for the epoch after both.
fn small_last_acts() -> Vec<[u64; 6]> {
    let mut all = Vec::new();
    for tick in 1..=2_u64 {
        for write in 0..=1_u64 {
            for transition in 0..=2_u64 {
                for first in 0..=3_u64 {
                    for second in first..=first + 3 {
                        for act in second..=second + 2 {
                            all.push([tick, write, transition, first, second, act]);
                        }
                    }
                }
            }
        }
    }
    all
}

/// Over every small tick, phase and order of a wake and a placeholder: where the hold is met no point either writes comes due before the first enabling. Round 6
/// had `last-act-lead` run from a wake's read alone, so a placeholder read before the wake came due early in 864 of its reader's small cases (round 7, RP7-1).
#[test]
fn every_arm_and_placeholder_of_the_last_act_is_held_from_its_first_read() {
    let mut from_the_wake_alone_admits_an_early_one = false;
    for [tick, write, transition, first, second, act] in small_last_acts() {
        let lead = tick + write + transition;
        // The realization's bound, from the first read, a tick of its counter old, to the act's read.
        let act_lead = act - first + tick;
        let enabling = act + write + transition;
        for distance in 0..=act_lead + lead + 2 {
            let at = |read_at| Written { read_at, distance };
            // A placeholder and a wake, in each order: round 6's lead ran from the wake's read.
            for (placeholder, wake) in [(first, second), (second, first)] {
                let written = [at(placeholder), at(wake)];
                let early = written.iter().any(|each| comes_due(each, tick) < enabling);
                if the_last_act_is_held(&written, act_lead, lead) {
                    assert!(
                        !early,
                        "{tick} {write} {transition} {first} {second} {act} {distance}"
                    );
                }
                let round_6 = act - wake + tick;
                from_the_wake_alone_admits_an_early_one |=
                    early && the_last_act_is_held(&written, round_6, lead);
            }
        }
    }
    assert!(from_the_wake_alone_admits_an_early_one);
    // RP7-1's case, in ticks: a placeholder read at 0, a wake at 5, the act's read at 6; a tick of 1, a write of 1, a transition of 2.
    let [placeholder_read, wake_read, act, tick, write, transition] = [0, 5, 6, 1, 1, 2];
    let lead = tick + write + transition;
    let placeholder = Written {
        read_at: placeholder_read,
        distance: 6,
    };
    assert!(
        the_last_act_is_held(&[placeholder], act - wake_read + tick, lead),
        "round 6's lead admits it"
    );
    assert!(
        comes_due(&placeholder, tick) < act + write + transition,
        "and it comes due before the enabling"
    );
    assert!(
        !the_last_act_is_held(&[placeholder], act - placeholder_read + tick, lead),
        "this one does not"
    );
    assert!(RECORD.contains(SPAN));
    assert!(RECORD.contains("each alarm that is no wake has that reach less one of its line's `step`, its placeholder's distance, held at no less than `last-act-lead` plus the port's `epoch-lead`"));
}

/// §1: a hosted plan's owed lines are §3's, a route's item as `/2` writes it, less what only §5 or the two leads read; and the row of the table that says so is
/// read here, since round 6's said "as `/1` writes each", which owes an alarm `arrival-bound` and none of what a source owes (round 7, RP7-2).
#[test]
fn a_hosted_plan_owes_what_section_3_owes() {
    let after = RECORD
        .split("<!-- machine-read: hosted-lines -->")
        .nth(1)
        .expect("the table");
    let owed = after
        .lines()
        .take_while(|line| line.starts_with('|') || line.is_empty())
        .find(|line| {
            line.starts_with("| `")
                && line
                    .split('|')
                    .nth(1)
                    .is_some_and(|cell| cell.contains("`owed`"))
        })
        .expect("the table's owed row");
    assert!(owed.contains("§3's"), "{owed}");
    assert!(!owed.contains("as `/1` writes each"), "{owed}");
    // A hosted route writes no `(controller …)`, so it owes neither the line nor the levels (round 30, RP30-2).
    for what in [
        "`epoch-lead`",
        "`last-act-lead`",
        "§5",
        "`controller-line`",
        "`controller-levels`",
    ] {
        assert!(owed.contains(what), "{owed} lacks {what}");
    }
}

/// §3: `/1` writes one route for a device, its requesters and costs merged; `/2` writes one for each alarm on it, with the requesters `/1` names that reach the
/// alarm's node and the costs `/1` writes of that node. A route's requester names the node it reaches, and a cost its node, as `/1` writes `(costs ((NODE KIND) …))`.
fn split_by_alarm<'a>(
    requesters: &[(&'a str, &'a str)],
    costs: &[(&'a str, &'a str)],
    alarms: &[&'a str],
) -> Vec<(&'a str, Vec<&'a str>, Vec<&'a str>)> {
    alarms
        .iter()
        .map(|&node| {
            let mine = |list: &[(&'a str, &'a str)]| {
                list.iter()
                    .filter(|(at, _)| *at == node)
                    .map(|(_, what)| *what)
                    .collect::<Vec<_>>()
            };
            (node, mine(requesters), mine(costs))
        })
        .collect()
}

/// Each alarm's route carries its own requesters and costs, and together they are `/1`'s, none twice: two builders write one route for each (round 7, RP7-3).
#[test]
fn each_alarm_s_route_carries_its_own_requesters_and_costs() {
    // RP7-3's case: an epoch extender over D used by `time.monotonic`, and `s` bound at D directly.
    let extender = "(adapter epoch-extender D)";
    let requesters = [(extender, "time.monotonic"), ("s", "s")];
    let costs = [(extender, "wake")];
    let routes = split_by_alarm(&requesters, &costs, &[extender, "s"]);
    assert_eq!(
        routes,
        [
            (extender, vec!["time.monotonic"], vec!["wake"]),
            ("s", vec!["s"], vec![])
        ]
    );
    let all: usize = routes
        .iter()
        .map(|(_, requesters, costs)| requesters.len() + costs.len())
        .sum();
    assert_eq!(all, requesters.len() + costs.len());
    assert!(
        RECORD.contains("`/1`'s one route of a device stands as one route for each alarm on it")
    );
}

/// §3, `BASE`: the readers of an alarm's time are the requesters the alarm's own requester reaches, itself not among them, as model §8 rule 14 counts "every other
/// requester": the requester of a deadline binds what its alarm asks of its device at that device, so counting it made every set that can reach a verdict read on the
/// counter (round 7, RP7-6).
#[test]
fn an_alarm_s_own_requester_is_no_reader_of_its_time() {
    assert!(RECORD.contains("each requester the requester bound for its deadline reaches through `needs` and `uses`, itself not among them, as rule 14 counts every other requester"));
    // An extended reader alone, the deadline's requester binding what it asks at the device: the alarm's own time base.
    assert_eq!(base(false, &[Reader::ExtendedOverIt]), Base::Epoch);
}

// ---- round 8: each read's own counter, a placeholder on its step, a device of two alarms, a clock's order

/// §4: `last-act-lead` is the greatest, over each read any arm or placeholder of the last act takes, of the time from a tick of that read's own counter
/// before it to the act's read for the epoch. Each read is `(read_at, tick)`, its counter's tick.
fn last_act_lead(reads: &[(u64, u64)], act: u64) -> u64 {
    reads
        .iter()
        .map(|&(read_at, tick)| act - read_at + tick)
        .max()
        .unwrap_or(0)
}

/// §4's words for the span, which `RPF-H3` carries word for word.
const SPAN: &str = "the greatest, over each read any arm or placeholder of the last act takes and each delay it writes, of the time from a tick of that read's or write's own counter before it to the act's read for the epoch, or under `(timer none)` to its call of the port's transition";

/// Two reads of the last act on counters of their own ticks, each as `[release, write, transition, first, first_tick, second, second_tick, act]`: the
/// release timer's tick, the port's write and transition, the two reads and their counters' ticks, and the act's read after both.
fn two_counters() -> Vec<[u64; 8]> {
    let mut all = Vec::new();
    for release in 1..=2_u64 {
        for write in 0..=1_u64 {
            for transition in 0..=2_u64 {
                for first in 0..=10_u64 {
                    for second in first..=10 {
                        for [first_tick, second_tick] in
                            [[1, 3], [1, 10], [3, 10], [10, 1], [10, 3], [3, 3]]
                        {
                            for act in second..=second + 2 {
                                all.push([
                                    release,
                                    write,
                                    transition,
                                    first,
                                    first_tick,
                                    second,
                                    second_tick,
                                    act,
                                ]);
                            }
                        }
                    }
                }
            }
        }
    }
    all
}

/// Over two reads on counters of different ticks: where every distance is no less than the span plus `epoch-lead`, nothing either writes comes due before
/// the first enabling. Round 7's span ran from the first read, a tick of its own counter old, and a later read of a coarser counter returned an older value
/// (round 8, RP8-1); and `RPF-H3` carried it with no tick at all (RP8-2).
#[test]
fn the_last_act_s_span_runs_from_each_read_less_its_own_counter_s_tick() {
    let mut from_the_first_read_admits_an_early_one = false;
    for [release, write, transition, first, first_tick, second, second_tick, act] in two_counters()
    {
        let lead = release + write + transition;
        let enabling = act + write + transition;
        let reads = [(first, first_tick), (second, second_tick)];
        let span = last_act_lead(&reads, act);
        let round_7 = act - first + first_tick;
        for distance in 0..=span.max(round_7) + lead + 1 {
            let early = reads
                .iter()
                .any(|&(read_at, tick)| comes_due(&Written { read_at, distance }, tick) < enabling);
            if distance >= span + lead {
                assert!(
                    !early,
                    "{reads:?} {act} {release} {write} {transition} {distance}"
                );
            }
            from_the_first_read_admits_an_early_one |= early && distance >= round_7 + lead;
        }
    }
    assert!(from_the_first_read_admits_an_early_one);
    // RP8-1's case: a read at 3 of tick 1, one at 9 of tick 10 returning 0, the act's read at 9; a release tick, a write and a transition of 1 each.
    let (lead, enabling) = (1 + 1 + 1, 9 + 1 + 1);
    let wake = Written {
        read_at: 9,
        distance: 10,
    };
    assert!(
        wake.distance >= (9 - 3 + 1) + lead && comes_due(&wake, 10) < enabling,
        "round 7's span admits it"
    );
    assert!(
        wake.distance < last_act_lead(&[(3, 1), (9, 10)], 9) + lead,
        "this one does not"
    );
    // RP8-2's case: `RPF-H3`'s words as round 7 carried them, with no tick: a wake of tick 4 read at 3, the act's read at 4.
    let (lead, enabling) = (1 + 1 + 1, 4 + 1 + 1);
    let wake = Written {
        read_at: 3,
        distance: 4,
    };
    assert!(
        wake.distance >= (4 - 3) + lead && comes_due(&wake, 4) < enabling,
        "those words admit it"
    );
    assert!(
        wake.distance < last_act_lead(&[(3, 4)], 4) + lead,
        "section 4's do not"
    );
    assert!(RECORD.contains(SPAN));
    let rpf_h3 = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H3` |"))
        .expect("RPF-H3");
    assert!(
        rpf_h3.contains(SPAN),
        "RPF-H3 carries section 4's span word for word"
    );
}

/// §4: a placeholder is written at the farthest point on its compare's step that one write reaches, the greatest value on that step at or below what it read
/// plus its reach.
fn placeholder_point(value: u64, reach: u64, step: u64) -> u64 {
    (value + reach) / step * step
}

/// A placeholder of the last act, each as `[tick, step, write, transition, read_at, act]`, its step a whole number of ticks.
fn placeholders() -> Vec<[u64; 6]> {
    let mut all = Vec::new();
    for tick in 1..=2_u64 {
        for steps in [1_u64, 2, 4, 8] {
            for write in 0..=1_u64 {
                for transition in 0..=2_u64 {
                    for read_at in 0..=8_u64 {
                        for act in read_at..=read_at + 4 {
                            all.push([tick, tick * steps, write, transition, read_at, act]);
                        }
                    }
                }
            }
        }
    }
    all
}

/// The point a placeholder is written at can fall up to a step less a tick short of the reach, so what is held is that reach less one step: where it is no
/// less than the span plus `epoch-lead`, no placeholder comes due before the first enabling. Round 7 held the whole reach (round 8, RP8-3).
#[test]
fn a_placeholder_is_held_at_its_reach_less_one_step() {
    let mut the_whole_reach_admits_an_early_one = false;
    for [tick, step, write, transition, read_at, act] in placeholders() {
        let lead = tick + write + transition;
        let enabling = act + write + transition;
        let span = last_act_lead(&[(read_at, tick)], act);
        let value = read_at - read_at % tick;
        for reach in step..=span + lead + 2 * step {
            let early = placeholder_point(value, reach, step) < enabling;
            if reach - step >= span + lead {
                assert!(
                    !early,
                    "{tick} {step} {write} {transition} {read_at} {act} {reach}"
                );
            }
            the_whole_reach_admits_an_early_one |= early && reach >= span + lead;
        }
    }
    assert!(the_whole_reach_admits_an_early_one);
    // RP8-3's case: tick 1, step 4, read at 3, the act's at 7, a write and a transition of 1: a reach of 8 met round 7's hold, and the point is 8, before 9.
    assert_eq!(placeholder_point(3, 8, 4), 8);
    assert!(8 >= last_act_lead(&[(3, 1)], 7) + 3 && 8 - 4 < last_act_lead(&[(3, 1)], 7) + 3);
    // And `write` is the release alarm's sum alone, never a placeholder's distance (RP8-4).
    assert!(!RECORD.contains("its `write` (`RA-H2`)"));
    let rpf_h4 = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H4` |"))
        .expect("RPF-H4");
    assert!(!rpf_h4.contains("to a `write` no less than"), "{rpf_h4}");
    assert!(
        rpf_h4.contains("its placeholder's distance in place of `P`"),
        "{rpf_h4}"
    );
}

/// §3 on a device of two alarms: each line `/1` writes of its one route is written once for each alarm. A route carries the requesters that reach its
/// alarm's node, so one reaching both stands on both, and the costs `/1` writes of that node, each on one route; an instance's context names the alarm of
/// the node it is; and an obligation is one for each alarm, as `RA-H1` asks (round 8, RP8-6).
#[test]
fn a_device_of_two_alarms_writes_each_line_once_for_each() {
    // RP8-6's case: `b` bound to a deadline-over-delay adapter over D, and `time.monotonic` to an epoch extender over D, using `b`.
    let extender = "(adapter epoch-extender D)";
    let delay = "(adapter deadline-over-delay D)";
    let alarms = [extender, delay];
    let reaches = [
        (extender, "time.monotonic"),
        (delay, "b"),
        (delay, "time.monotonic"),
    ];
    let costs = [(extender, "wake")];
    let routes = split_by_alarm(&reaches, &costs, &alarms);
    assert_eq!(
        routes,
        [
            (extender, vec!["time.monotonic"], vec!["wake"]),
            (delay, vec!["b", "time.monotonic"], vec![])
        ]
    );
    let costs_written: usize = routes.iter().map(|(_, _, costs)| costs.len()).sum();
    assert_eq!(costs_written, costs.len(), "each cost on one route");
    // An instance's `(interrupt (alarm NODE))` is its own alarm, and no instance names another's.
    let interrupt = |instance: &str| alarms.into_iter().find(|node| *node == instance);
    assert_eq!(interrupt(extender), Some(extender));
    assert_eq!(interrupt(delay), Some(delay));
    assert_eq!(interrupt("time.monotonic"), None);
    // An `(obligation route (alarm NODE) …)` for each route, and so one for each alarm, where `/1` writes one for the device.
    let obliged: Vec<&str> = routes.iter().map(|(node, _, _)| *node).collect();
    assert_eq!(obliged, alarms, "one for each alarm");
    for words in [
        "a requester on the route of each node it reaches",
        "for each alarm whose reach holds the line's subject",
        "an `(obligation route …)` line once for each alarm",
    ] {
        assert!(RECORD.contains(words), "{words}");
    }
}

/// §3: `clock` lines have an order and owe what they say `unstated`, so two clock lines have one text and an unstated modulus is never complete
/// (round 8, RP8-5).
#[test]
fn clock_lines_have_an_order_and_owe_what_they_leave_unstated() {
    let orders = RECORD
        .lines()
        .find(|line| line.starts_with("**Orders.**"))
        .expect("the orders");
    assert!(
        orders.contains("`clock` lines by `PROVIDER`'s text"),
        "{orders}"
    );
    assert!(RECORD.contains("An owed line's subject gains `(alarm NODE)`, `(clock PROVIDER)`,"));
    assert!(RECORD.contains("of a simulated alarm or a clock by `description`, `tick-rate` and `counter-modulus`, where its line says `unstated`"));
}

// ---- round 9: a delay's write in the span, each alarm's reach in a context, a whole-number compare's reach

/// A delay of the last act and a wake's read, each as `[tick, step, port_write, transition, written_at, read_at, act]`: the counter's tick, the delay's step, the
/// port's write and transition, when the delay is written and when the wake reads, and the act's read after both.
fn delays_and_reads() -> Vec<[u64; 7]> {
    let mut all = Vec::new();
    for tick in 1..=2_u64 {
        for step in [tick, 2 * tick] {
            for port_write in 0..=1_u64 {
                for transition in 0..=2_u64 {
                    for written_at in 0..=6_u64 {
                        for read_at in 0..=6_u64 {
                            for act in written_at.max(read_at)..=written_at.max(read_at) + 2 {
                                all.push([
                                    tick, step, port_write, transition, written_at, read_at, act,
                                ]);
                            }
                        }
                    }
                }
            }
        }
    }
    all
}

/// §4: a delay counts from its write and takes no read, so its write is in the span as a read is, from a tick of its counter before it. Round 8's span ran over
/// reads alone, and a placeholder on a delay written early came due before the first enabling (round 9, RP9-1).
#[test]
fn a_delay_s_placeholder_is_held_from_its_write() {
    let mut over_reads_alone_admits_an_early_one = false;
    for [tick, step, port_write, transition, written_at, read_at, act] in delays_and_reads() {
        let lead = tick + port_write + transition;
        let enabling = act + port_write + transition;
        // Both the write and the read are on the delay's counter here; each is held from a tick of it before it.
        let span = last_act_lead(&[(written_at, tick), (read_at, tick)], act);
        let round_8 = last_act_lead(&[(read_at, tick)], act);
        for reach in step..=span.max(round_8) + lead + 2 * step {
            // The delay counts the farthest whole number of steps one write reaches, from the tick its write lands in.
            let due = written_at - written_at % tick + placeholder_point(0, reach, step);
            let early = due < enabling;
            if reach - step >= span + lead {
                assert!(
                    !early,
                    "{tick} {step} {port_write} {transition} {written_at} {read_at} {act} {reach}"
                );
            }
            over_reads_alone_admits_an_early_one |= early && reach - step >= round_8 + lead;
        }
    }
    assert!(over_reads_alone_admits_an_early_one);
    // RP9-1's case, in tenths of a microsecond: the delay written at 0, a wake of tick 10 reading at 50, the act's read at 60, `epoch-lead` 1 + 3 + 20.
    let (lead, enabling) = (1 + 3 + 20, 60 + 3 + 20);
    let (reach, step) = (60, 10);
    assert!(
        reach - step >= last_act_lead(&[(50, 10)], 60) + lead
            && placeholder_point(0, reach, step) < enabling
    );
    assert!(reach - step < last_act_lead(&[(0, 10), (50, 10)], 60) + lead);
    assert!(RECORD.contains(SPAN));
}

/// §3: the alarms an instance's context line names, of each alarm's reach, which `alarm_reach` derives as `contexts_of` derives one route's.
fn contexts<'a>(reaches: &[(&'a str, &[&'a str])]) -> BTreeMap<&'a str, Vec<&'a str>> {
    let mut named: BTreeMap<&str, Vec<&str>> = BTreeMap::new();
    for &(alarm, reach) in reaches {
        for &instance in reach {
            named.entry(instance).or_default().push(alarm);
        }
    }
    named
}

/// [`alarm_reach`], the library's held to it (leaf `M4.1.7.6.5`): its `reaches` the edges' closure, what a name reaches at any depth.
fn reach_held<'a>(
    device: &'a str,
    node: &'a str,
    delivers_to: &[&'a str],
    edges: &[(&'a str, &'a str)],
) -> Vec<&'a str> {
    let model = alarm_reach(device, node, delivers_to, edges);
    let closure = |from: &str| -> BTreeSet<String> {
        let mut all: BTreeSet<String> = BTreeSet::new();
        let mut todo = vec![from.to_owned()];
        while let Some(at) = todo.pop() {
            for (_, to) in edges.iter().filter(|(f, _)| *f == at) {
                if all.insert((*to).to_owned()) {
                    todo.push((*to).to_owned());
                }
            }
        }
        all
    };
    let library = archogen_plan::alarm::reach(
        device,
        node,
        &delivers_to
            .iter()
            .map(|d| (*d).to_owned())
            .collect::<Vec<_>>(),
        closure,
    );
    assert_eq!(
        library.iter().map(String::as_str).collect::<Vec<_>>(),
        model,
        "the library's reach of {node}"
    );
    model
}

/// [`contexts`], the library's held to it (leaf `M4.1.7.6.5`).
fn contexts_held<'a>(reaches: &[(&'a str, &[&'a str])]) -> BTreeMap<&'a str, Vec<&'a str>> {
    let model = contexts(reaches);
    let library = archogen_plan::alarm::contexts(
        &reaches
            .iter()
            .map(|(alarm, reach)| {
                (
                    (*alarm).to_owned(),
                    reach.iter().map(|r| (*r).to_owned()).collect(),
                )
            })
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        library
            .iter()
            .map(|(subject, alarms)| (
                subject.as_str(),
                alarms.iter().map(String::as_str).collect::<Vec<_>>()
            ))
            .collect::<BTreeMap<_, _>>(),
        model,
        "the library's contexts"
    );
    model
}

/// An instance one alarm's code reaches names that alarm too, the device among them, and one no alarm's reaches names none: round 8 had each instance name its own
/// alarm alone, false where an expiry's delivery reaches the wake's instance (round 9, RP9-2).
#[test]
fn a_context_names_each_alarm_whose_reach_holds_it() {
    // RP9-2's case: the shipped deadline over a delay, `time.monotonic` asking far enough to read through an epoch extender over the same timer. A wake runs its
    // own code alone; the expiry delivers to `time.deadline`, which needs `time.monotonic`.
    let wake = "(adapter epoch-extender timer.delay)";
    let expiry = "(adapter deadline-over-delay timer.delay)";
    // Derived, never written by hand (round 10, RP10-1): `time.monotonic` bound to the extender, `time.deadline` to the delivering adapter and needing
    // `time.monotonic`, each adapter over the timer.
    let edges = [
        ("time.monotonic", wake),
        ("time.deadline", expiry),
        ("time.deadline", "time.monotonic"),
        (wake, "timer.delay"),
        (expiry, "timer.delay"),
    ];
    let wake_reach = reach_held("timer.delay", wake, &[], &edges);
    let expiry_reach = reach_held("timer.delay", expiry, &["time.deadline"], &edges);
    let named = contexts_held(&[(wake, &wake_reach), (expiry, &expiry_reach)]);
    assert_eq!(
        named[wake],
        [wake, expiry],
        "the extender's line holds the expiry's alarm too"
    );
    assert_eq!(
        named["timer.delay"],
        [wake, expiry],
        "the device holds both"
    );
    assert_eq!(named["time.monotonic"], [expiry]);
    assert_eq!(named["time.deadline"], [expiry]);
    assert!(!named.contains_key("an.unreached.node"));
    assert!(RECORD.contains("for each alarm whose reach holds the line's subject"));
}

/// §4: what one write reaches, as `decision_release-alarm.md` §2's `write` row and the checker's model say, in ticks: the less of the line's `reach` and what
/// the compare's reading leaves of its modulus, its window less one tick, or, reading whole numbers, the operating horizon less one tick.
fn one_write_reaches(reach: u128, modulus: u128, leaves_by: u128) -> u128 {
    reach.min(modulus - leaves_by - 1)
}

/// A compare that reads whole numbers reaches no farther than the operating horizon leaves of its modulus, which round 8's placeholder left out, holding the
/// line's `reach` alone (round 9, RP9-3).
#[test]
fn a_whole_number_compare_s_placeholder_is_held_within_its_horizon() {
    // RP9-3's case: 1 GHz, a modulus 2,000 ticks past the operating horizon's count, `reach` 1 ms, a step of one tick, leads of 3 µs, all in nanoseconds.
    let horizon = 3_155_760_000_u128 * 1_000_000_000;
    let (reach, modulus, step, leads) = (1_000_000_u128, horizon + 2_000, 1_u128, 3_000_u128);
    let reached = one_write_reaches(reach, modulus, horizon);
    assert_eq!(
        reached, 1_999,
        "what the checker's `what_one_write_reaches` gives"
    );
    assert!(reach - step >= leads, "round 8's reading admits it");
    assert!(reached - step < leads, "this one does not");
    // A windowed compare's limit is its window's, as before.
    assert_eq!(one_write_reaches(reach, 256, 16), 239);
    for words in [
        "its line's `reach`, and on a compare the less of that and what its reading leaves of its modulus",
        "`what_one_write_reaches`",
    ] {
        assert!(RECORD.contains(words), "{words}");
    }
    let rpf_h4 = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H4` |"))
        .expect("RPF-H4");
    assert!(
        rpf_h4.contains(
            "its line's `reach`, and on a compare the less of that and what its reading leaves of its modulus"
        ),
        "{rpf_h4}"
    );
}

// ---- round 10: an alarm's reach as `contexts_of` derives one route's, and ties by the interrupt's text

/// §3, `joint.rs`'s `contexts_of` for one alarm's route alone: the device, the alarm's own node, the requesters bound to that node where its kind delivers,
/// and every node one of these reaches, by `edges`, each a node and what it reaches directly.
fn alarm_reach<'a>(
    device: &'a str,
    node: &'a str,
    delivers_to: &[&'a str],
    edges: &[(&'a str, &'a str)],
) -> Vec<&'a str> {
    let mut all: BTreeSet<&str> = [device, node]
        .into_iter()
        .chain(delivers_to.iter().copied())
        .collect();
    loop {
        let more: Vec<&str> = edges
            .iter()
            .filter(|(from, to)| all.contains(from) && !all.contains(to))
            .map(|&(_, to)| to)
            .collect();
        if more.is_empty() {
            return all.into_iter().collect();
        }
        all.extend(more);
    }
}

/// An expiry's interrupt runs the code of the requester it delivers to, as `/1` gives it: the corpus's `dod-bare` lowers `time.deadline` with the timer's
/// route in its context. Round 9's list, the instance and what it reaches, left that requester out, being bound to the instance and not reached by it
/// (round 10, RP10-1).
#[test]
fn an_expiry_s_interrupt_names_the_requester_it_delivers_to() {
    let expiry = "(adapter deadline-over-delay timer.delay)";
    let edges = [("time.deadline", expiry), (expiry, "timer.delay")];
    let reach = reach_held("timer.delay", expiry, &["time.deadline"], &edges);
    let named = contexts_held(&[(expiry, &reach)]);
    assert_eq!(
        named["time.deadline"],
        [expiry],
        "as `dod-bare` lowers it under `/1`"
    );
    let round_9 = alarm_reach("timer.delay", expiry, &[], &edges);
    assert!(
        !round_9.contains(&"time.deadline"),
        "round 9's words left it out"
    );
    assert!(RECORD.contains("the requesters bound to it where its kind delivers"));
}

/// §3: ranks follow the release alarm's §3, what states no value last and ties by the interrupt's text as an item writes it, `(alarm NODE)` or
/// `(source NAME)`, as that record and `/1` §2.5 break them. Each is `(value, item)`.
fn ranked(mut items: Vec<(Option<u64>, &str)>) -> Vec<&str> {
    items.sort_by(|a, b| (a.0.is_none(), a.0, a.1).cmp(&(b.0.is_none(), b.0, b.1)));
    items.into_iter().map(|(_, item)| item).collect()
}

/// Two items stating no value tie, and the interrupt's text breaks it, never `NODE`'s, which gave two orders and two identities (round 10, RP10-2).
#[test]
fn ranks_tie_by_the_interrupt_s_text() {
    // RP10-2's case: a source `button` with no stated separation and a service `zz.watchdog` bound at a timer asking no `delivery-bound`.
    let order = ranked(vec![
        (None, "(source button)"),
        (None, "(alarm zz.watchdog)"),
        (Some(10), "(alarm a.fast)"),
    ]);
    assert_eq!(
        order,
        ["(alarm a.fast)", "(alarm zz.watchdog)", "(source button)"]
    );
    // By the node's text `button` would come first: the record does not say so.
    assert!("button" < "zz.watchdog" && order[1] == "(alarm zz.watchdog)");
    assert!(RECORD.contains("ties by the interrupt's text as an item writes it"));
    assert!(!RECORD.contains("ties by `NODE`'s text"));
}

// ---- round 11: the enabled set under no timer, a placement obligation for each hole, what an owed unit waits on

/// §3: the `(enabled …)` line, `timer` then the routes by rank, and no `timer` where the plan writes `(timer none)`: the release alarm's §1 and model §8
/// rule 8 enable no release timer a system has none of.
fn enabled<'a>(timer: bool, routes: &[&'a str]) -> Vec<&'a str> {
    let timer: &[&str] = if timer { &["timer"] } else { &[] };
    timer.iter().chain(routes).copied().collect()
}

/// Under `(timer none)` the line holds no `timer`: `/1`'s order, read alone, gave a system whose only task is sporadic `(enabled timer)`, a trap the release
/// alarm calls unexpected, and two identities (round 11, RP11-1).
#[test]
fn no_timer_is_enabled_under_timer_none() {
    assert_eq!(
        enabled(false, &[]),
        Vec::<&str>::new(),
        "a sporadic task alone on `riscv-virt-up`"
    );
    assert_eq!(enabled(false, &["(source button)"]), ["(source button)"]);
    assert_eq!(
        enabled(true, &["(source button)"]),
        ["timer", "(source button)"]
    );
    assert!(RECORD.contains("its `(enabled …)` holds no `timer`"));
}

/// §3: one `(obligation placement (region PROVIDER NAME))` for each `Obligation::Placement` `Joint::map` reports, each executable region that is not reserved,
/// by their text, as `/1`'s (round 13, RP13-2). Each region is `(base, name, provider, executable, reserved)`.
fn placement_obligations(regions: Vec<(u64, &str, &str, bool, bool)>) -> Vec<String> {
    let mut lines: Vec<String> = regions
        .into_iter()
        .filter(|&(_, _, _, executable, reserved)| executable && !reserved)
        .map(|(_, name, provider, _, _)| {
            format!("(obligation placement (region {provider} {name}))")
        })
        .collect();
    // By their text, as `/1` orders every `obligation` line (round 13, RP13-2).
    lines.sort();
    lines
}

/// Two executable regions give two lines, and two providers' regions of one name two texts: round 10 wrote one, for the placement region, by its name alone
/// (round 11, RP11-2).
#[test]
fn each_placement_obligation_has_a_line_of_its_own() {
    // RP11-2's case: a platform with two executable regions, `sram1` and `sram2`; and a reserved one, which no placement takes.
    let lines = placement_obligations(vec![
        (0x2000_0000, "sram2", "soc", true, false),
        (0x1000_0000, "sram1", "soc", true, false),
        (0x3000_0000, "boot", "soc", true, true),
    ]);
    assert_eq!(
        lines,
        [
            "(obligation placement (region soc sram1))",
            "(obligation placement (region soc sram2))"
        ]
    );
    let two_rams = placement_obligations(vec![
        (0, "ram", "a", true, false),
        (0, "ram", "b", true, false),
    ]);
    assert_eq!(two_rams.len(), 2);
    assert_ne!(two_rams[0], two_rams[1]);
    assert!(RECORD.contains("one line for each `Obligation::Placement` `Joint::map` reports"));
}

/// §5: each owed unit's `(needs …)`: the figures it lacks of its own, then `(unit UNIT)` naming the last owed unit before it, so one that lacks nothing of its
/// own is never written with an empty list. Each unit is `(name, what it lacks of its own)`, in §5's order.
fn owed_needs<'a>(units: &[(&'a str, &[&'a str])]) -> Vec<(&'a str, Vec<String>)> {
    let mut last_owed: Option<&str> = None;
    let mut out = Vec::new();
    for &(name, lacks) in units {
        if lacks.is_empty() && last_owed.is_none() {
            continue;
        }
        let mut needs: Vec<String> = lacks
            .iter()
            .map(|fact| format!("(record {fact})"))
            .collect();
        if let Some(earlier) = last_owed {
            needs.push(format!("(unit {earlier})"));
        }
        out.push((name, needs));
        last_owed = Some(name);
    }
    out
}

/// [`owed_needs`], the library's held to it: each lack a record's fact, as the model writes one (leaf `M4.1.7.6.1`).
fn owed_held<'a>(units: &[(&'a str, &[&'a str])]) -> Vec<(&'a str, Vec<String>)> {
    let model = owed_needs(units);
    let library = archogen_plan::layout::owed_needs(
        &units
            .iter()
            .map(|(name, lacks)| {
                (
                    (*name).to_owned(),
                    lacks
                        .iter()
                        .map(|fact| archogen_plan::layout::Need {
                            owner: archogen_plan::layout::Owner::Record,
                            fact: (*fact).to_owned(),
                        })
                        .collect(),
                )
            })
            .collect::<Vec<_>>(),
    );
    assert_eq!(
        library,
        model
            .iter()
            .map(|(name, needs)| ((*name).to_owned(), needs.clone()))
            .collect::<Vec<_>>(),
        "the library's needs of {units:?}"
    );
    model
}

/// A unit owed only because an earlier one is names that unit: round 10's words allowed `(needs)` or a copy of the earlier unit's needs (round 11, RP11-3).
#[test]
fn an_owed_unit_names_what_it_waits_on() {
    // RP11-3's case: every figure stated but `idle-stack`, which the boot's stack lacks; the interrupt stack lacks nothing of its own.
    let owed = owed_held(&[("boot", &["idle-stack"]), ("interrupt", &[])]);
    assert_eq!(
        owed,
        [
            ("boot", vec!["(record idle-stack)".to_string()]),
            ("interrupt", vec!["(unit boot)".to_string()])
        ]
    );
    assert!(
        owed_held(&[("boot", &[]), ("interrupt", &[])]).is_empty(),
        "nothing lacking, nothing owed"
    );
    assert!(RECORD.contains("`(unit UNIT)` naming the last such before it"));
}

// ---- round 12: the span under no timer, one reading of a compare, the release timer's counts past 64 bits

/// Under `(timer none)` the act reads nothing for an epoch, so the span runs to its call of the port's transition, and `RPF-H3` says so: ended at the act's
/// last write, a wake came due before the enabling (round 12, RP12-1).
#[test]
fn under_no_timer_the_span_ends_at_the_transition_call() {
    // RP12-1's case: a wake read at 0 on a counter of tick 1, a placeholder written at 5, the call of the transition at 9, `epoch-lead` 4, the enabling at 11.
    let (lead, enabling, call) = (4, 11, 9);
    let reads = [(0, 1), (5, 1)];
    let wake = Written {
        read_at: 0,
        distance: 10,
    };
    assert!(
        wake.distance >= last_act_lead(&reads, 5) + lead && comes_due(&wake, 1) < enabling,
        "ended at the last write"
    );
    assert!(
        wake.distance < last_act_lead(&reads, call) + lead,
        "ended at the call"
    );
    assert!(SPAN.ends_with("or under `(timer none)` to its call of the port's transition"));
    let rpf_h3 = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H3` |"))
        .expect("RPF-H3");
    assert!(
        rpf_h3.contains(SPAN),
        "RPF-H3 carries the span under no timer too"
    );
}

/// §2: how a device's compare is read, of its `compare-window` and its `compare-magnitude`: a window, whole numbers, nothing stated, or refused.
#[derive(Debug, PartialEq)]
enum Reading {
    Window(u64),
    WholeNumbers,
    Unstated,
    Refused,
}

fn compare_reading(window: Option<u64>, magnitude: Option<bool>) -> Reading {
    match (window, magnitude) {
        (Some(_), Some(true)) => Reading::Refused,
        (None, Some(true)) => Reading::WholeNumbers,
        (Some(window), _) => Reading::Window(window),
        (None, _) => Reading::Unstated,
    }
}

/// `(compare-magnitude false)` says nothing, so beside a window the device reads by its window, and only `(compare-magnitude true)` beside one is refused:
/// round 11's "a magnitude beside a window" read two ways (round 12, RP12-2).
#[test]
fn a_compare_is_read_one_way() {
    assert_eq!(
        compare_reading(Some(16), Some(false)),
        Reading::Window(16),
        "RP12-2's case"
    );
    assert_eq!(compare_reading(Some(16), None), Reading::Window(16));
    assert_eq!(compare_reading(Some(16), Some(true)), Reading::Refused);
    assert_eq!(compare_reading(None, Some(true)), Reading::WholeNumbers);
    assert_eq!(compare_reading(None, Some(false)), Reading::Unstated);
    assert!(RECORD.contains("`(compare-magnitude true)` beside a window is `invalid-description`"));
    // And the release timer's counts past 64 bits are refused as an alarm's are (RP12-3).
    let rpf_h4 = RECORD
        .lines()
        .find(|line| line.starts_with("| `RPF-H4` |"))
        .expect("RPF-H4");
    assert!(
        rpf_h4.contains("the release timer's or an alarm's"),
        "{rpf_h4}"
    );
}

/// §2: what checks `riscv-virt-up`'s `non-maskable-interrupts`. Not `riscv,isa` holding no `smrnmi` alone: the privileged specification gives every hart
/// unresumable non-maskable interrupts, and Smrnmi adds only resumable ones, so the device tree says that none resumes and no more (round 13, RP13-1). The
/// pinned QEMU's own source says the rest, that its hart takes none but Smrnmi's and no device connects one, and the agreement holds both (leaf `M4.1.7.3.4`).
#[test]
fn no_count_is_said_checked_that_nothing_checks() {
    let row = RECORD
        .lines()
        .find(|line| line.starts_with("| `non-maskable-interrupts`, a count |"))
        .expect("its row");
    assert!(
        !row.contains("which says so of the emulated hart alone"),
        "{row}"
    );
    assert!(
        row.contains("no `smrnmi` on the dump")
            && row.contains("'s source, which takes none but Smrnmi's"),
        "{row}"
    );
    let agreement = include_str!("../../../xtask/src/target.rs");
    assert!(
        agreement.contains("REQUIRES_NON_MASKABLE_INTERRUPTS")
            && agreement.contains("const NON_MASKABLE_READ_AT: &str")
    );
    assert!(RECORD.contains(
        "The base machine-level architecture supports only unresumable non-maskable interrupts (UNMIs)"
    ));
}

/// §3: placement obligations stand by their text, as `/1` orders every `obligation` line, `/2` keeping its orders: in the region lines' order as well, two
/// builders following the two rules wrote two code identities (round 13, RP13-2).
#[test]
fn placement_obligations_stand_by_their_text() {
    let lines = placement_obligations(vec![
        (0x1000_0000, "sram", "soc", true, false),
        (0x8000_0000, "dram", "soc", true, false),
    ]);
    assert_eq!(
        lines,
        [
            "(obligation placement (region soc dram))",
            "(obligation placement (region soc sram))"
        ],
        "RP13-2's case"
    );
    let lines = placement_obligations(vec![
        (0, "ram", "z.soc", true, false),
        (0x1000, "ram", "a.soc", true, false),
    ]);
    assert_eq!(
        lines,
        [
            "(obligation placement (region a.soc ram))",
            "(obligation placement (region z.soc ram))"
        ]
    );
    assert!(RECORD.contains("`obligation` and `owed` lines by their text, as `/1`'s"));
}

/// §1: the catalog's digest is of one line for each record, its id, a space and its hash as `Digest` displays one, `sha256:` and its hex, by id, each ending in a
/// line feed, with nothing before the first: three preimages fitted the record's words (round 13, RP13-3).
fn catalog_preimage(records: &[(&str, &str)]) -> String {
    let mut sorted = records.to_vec();
    sorted.sort();
    sorted
        .iter()
        .map(|(id, hex)| format!("{id} sha256:{hex}\n"))
        .collect()
}

/// [`catalog_preimage`], the library's held to it, each hash read as [`archogen_digest::Digest`] parses its written form (leaf `M4.1.7.6.2`).
fn catalog_held(records: &[(&str, &str)]) -> String {
    let model = catalog_preimage(records);
    let library: Vec<(String, archogen_digest::Digest)> = records
        .iter()
        .map(|(id, hex)| {
            (
                (*id).to_owned(),
                archogen_digest::Digest::parse(&format!("sha256:{hex}")).expect("a digest"),
            )
        })
        .collect();
    assert_eq!(archogen_plan::identity::catalog_preimage(&library), model);
    assert_eq!(
        archogen_plan::identity::catalog_digest(&library),
        archogen_digest::Digest::of(model.as_bytes())
    );
    model
}

#[test]
fn the_catalog_digest_has_one_preimage() {
    let (a, b) = ("0".repeat(64), "f".repeat(64));
    assert_eq!(
        catalog_held(&[("uart.ns16550a", b.as_str()), ("port.riscv", a.as_str())]),
        format!("port.riscv sha256:{a}\nuart.ns16550a sha256:{b}\n")
    );
    assert_eq!(
        catalog_held(&[]),
        "",
        "an empty catalog: nothing before the first, and no line"
    );
    assert_eq!(
        catalog_held(&[("port.riscv", b.as_str()), ("port.riscv", a.as_str())]),
        format!("port.riscv sha256:{a}\nport.riscv sha256:{b}\n"),
        "one id twice: by its hash"
    );
    assert!(RECORD.contains("with nothing before the first"));
    // The display the record names is `Digest`'s own.
    let digest = include_str!("../../archogen-digest/src/lib.rs");
    assert!(digest.contains("write!(f, \"sha256:{}\", self.hex())"));
}

/// §1, §5 and §7: every refusal names its code (round 13, RP13-4), and each of inputs that do not belong together is the status vocabulary's `stale-input`, a
/// step's output from other inputs, whose repair is to run that step again: the assignment's target, a record's, a realization of another code identity
/// and a bound measured on another image. Only a file a person writes, the application's, is `invalid-description` where it is not the plan's. Before, a
/// realization of another code identity was `invalid-description`, which says the description is malformed, and a bound of another image named no code
/// (round 37, RP37-1). A refusal kept from `/1` gives `/2`'s repair: a hosted system with placement memory names a target, not `/1`'s wait (RP37-3).
#[test]
fn every_refusal_names_its_code() {
    // Each refusal of inputs that do not belong together, as the record says it, and whether a person writes the input it refuses.
    const MISMATCHED: [(&str, bool); 8] = [
        ("An assignment of the other kind, a `/3` lowered or checked with no `--target` or a `/2` with one, is `stale-input`", false),
        ("the assignment's `(target …)` digest other than the file's taken with its program digest, `stale-input`", false),
        ("A record whose target's digest differs, or of the other kind as above, is `stale-input`", false),
        ("its `HEX` decides: one other than the given file's is another run's, `stale-input`", false),
        ("a lowering given one of another code refuses it `stale-input`, \"measure again\", naming both identities", false),
        ("then a realization of another code identity, `stale-input`", false),
        ("A bound or a list of counts whose `binary` names another image is refused `stale-input`", false),
        ("its own `(target …)` against the plan's by the lowering, `invalid-description` where they differ", true),
    ];
    let flat = RECORD.split_whitespace().collect::<Vec<_>>().join(" ");
    for (said, by_a_person) in MISMATCHED {
        assert!(flat.contains(said), "the record says: {said}");
        let code = if by_a_person {
            "`invalid-description`"
        } else {
            "`stale-input`"
        };
        assert!(said.contains(code), "{said} is {code}");
    }
    for false_said in [
        "a lowering given one of another code refuses it `invalid-description`",
        "then a realization of another code identity, `invalid-description`",
    ] {
        assert!(!flat.contains(false_said), "{false_said}");
    }
    // A `/3` naming another platform beside the given file's own digest is of this run, the digest deciding, and no run writes it: `tool-failure`, as a
    // record naming another profile or system, in §1, the checker's bullet and §7 alike. Before, the record said "a record whose target differs" is
    // `stale-input`, which read two ways, and the checker's two paths gave the two answers (leaf `M4.1.7.5.6`).
    for misnamed in [
        "A `NAME` other than the one platform of the file whose `HEX` it writes is a record no run writes, since that file's forms name one platform: `tool-failure`",
        "one naming another platform beside the given file's digest is `tool-failure`",
        "its platform other than the file's beside the file's digest `tool-failure`",
    ] {
        assert!(flat.contains(misnamed), "the record says: {misnamed}");
    }
    // The reader's case of round 38: resolved on a target and built with no `--target`, the assignment is a target's `/3` read as the hosted `/2`. The two
    // kinds are of other runs, not of a failure, so `stale-input`; any other format stays `/1`'s `tool-failure` (RP38-1).
    let refusal = |format: &str, target: bool| match (format, target) {
        ("archogen-assignment/3", true) | ("archogen-assignment/2", false) => None,
        ("archogen-assignment/3", false) | ("archogen-assignment/2", true) => Some("stale-input"),
        _ => Some("tool-failure"),
    };
    assert_eq!(refusal("archogen-assignment/3", false), Some("stale-input"));
    assert_eq!(refusal("archogen-assignment/2", true), Some("stale-input"));
    assert_eq!(refusal("archogen-assignment/1", true), Some("tool-failure"));
    assert_eq!(refusal("archogen-assignment/3", true), None);
    let assignment = include_str!("../../eadl-resolve/src/plan.rs");
    assert!(
        assignment.contains("pub const FORMAT: &str = \"archogen-assignment/2\";"),
        "today's reader, the hosted one"
    );
    assert!(flat.contains("An assignment on a target is `archogen-assignment/3`"));
    assert!(flat.contains("any other format stays `/1`'s `tool-failure`"));
    // The vocabulary's own word for it, which a script branches on.
    let status = include_str!("../../archogen-outcome/src/status.rs");
    assert!(status.contains("/// The inputs of the invocation do not belong together — a plan resolved from another description, a build made"));
    // `/1`'s refusal of placement memory, kept, tells its reader to wait for `/2`: a `/2` lowering names a target instead.
    let semantics = include_str!("../../../docs/semantics/model.md");
    assert!(semantics
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
        .contains("refused when it is read, whether or not anything binds it"));
    assert!(
        flat.contains("refused when the file is read, whether or not anything binds the device")
    );
    let lowering = include_str!("../src/lib.rs");
    assert!(lowering.contains("describe a hosted platform, or wait for `/2`"));
    assert!(flat.contains(
        "a system with placement memory staying refused as `/1` refuses it, `unsupported-profile`, its repair naming a target in place of `/1`'s wait for `/2`"
    ));
}

/// §3: which of its five facts an alarm owes, by what the checker asks of it: `floor` and `service-cost` of every alarm, `step` of a wake, `reach-ahead` of an
/// expiry whose time base is the counter itself, `lag` of one on a target. Owed all five, an expiry owed a `step` it has none of and a simulated alarm a `lag`
/// that §3 calls nothing, and such a plan was never `(complete yes)` (round 13, RP13-5).
fn alarm_owes(wake: bool, counter_itself: bool, simulated: bool) -> Vec<&'static str> {
    let mut owed = vec!["floor"];
    if wake {
        owed.push("step");
    }
    if !wake && counter_itself {
        owed.push("reach-ahead");
    }
    if !simulated {
        owed.push("lag");
    }
    owed.push("service-cost");
    owed
}

#[test]
fn each_alarm_owes_what_the_checker_asks_of_it() {
    assert_eq!(
        alarm_owes(false, false, false),
        ["floor", "lag", "service-cost"],
        "an expiry on the epoch"
    );
    assert_eq!(
        alarm_owes(false, true, false),
        ["floor", "reach-ahead", "lag", "service-cost"]
    );
    assert_eq!(
        alarm_owes(true, true, false),
        ["floor", "step", "lag", "service-cost"],
        "a wake asks no `R`"
    );
    assert_eq!(
        alarm_owes(true, false, true),
        ["floor", "step", "service-cost"],
        "a simulated wake"
    );
    // The checker's model gates each by the same, and a change there is a change here.
    let checker = include_str!("../../rt-analysis/tests/alarm_model.rs");
    for gate in [
        "(line.t_s.is_none(), \"T_s\")",
        "(!simulated && line.lag.is_none(), \"λ\")",
        "(line.c_s.is_none(), \"C_s\")",
        "(line.wake && line.p.is_none(), \"P\")",
        "(on_the_counter && line.r.is_none(), \"R\")",
    ] {
        assert!(checker.contains(gate), "{gate}");
    }
    assert!(RECORD.contains(
        "`step` of a wake, `reach-ahead` of an expiry whose time base is the counter itself"
    ));
}

/// §5: what an allocation's `(origin …)` lists. Every figure its sum reads, each name's figures all and not its largest alone, since the stack rests on each: one
/// assumed smaller than another stated may be the largest. And an item written once however many figures write it, two of one record's facts being one text
/// (round 14, RP14-1).
fn origin_items(figures: &[(&str, &str, &str, u64)]) -> (Vec<String>, &'static str) {
    const WEAKEST_FIRST: [&str; 4] = [
        "assumed",
        "observed-maximum",
        "externally-supplied",
        "analytically-established",
    ];
    // By owner, `application`, `record`, `realization`, then by text, as §3's orders say (round 15, RP15-2).
    let rank = |item: &str| {
        ["(application ", "(record ", "(realization "]
            .iter()
            .position(|owner| item.starts_with(owner))
            .unwrap_or(3)
    };
    let mut items: Vec<String> = figures
        .iter()
        .map(|(owner, of, category, _)| format!("({owner} {of} {category})"))
        .collect();
    items.sort_by(|a, b| (rank(a), a).cmp(&(rank(b), b)));
    items.dedup();
    let weakest = WEAKEST_FIRST
        .into_iter()
        .find(|category| figures.iter().any(|figure| figure.2 == *category))
        .expect("a figure");
    (items, weakest)
}

/// [`origin_items`], the library's held to it (leaf `M4.1.7.6.1`): each figure's `of` read as its subject, a record's id or whom the application's
/// file says it of, then its fact.
fn origin_held(figures: &[(&str, &str, &str, u64)]) -> (Vec<String>, &'static str) {
    let model = origin_items(figures);
    let library: Vec<archogen_plan::layout::Origin> = figures
        .iter()
        .map(|(owner, of, category, _)| {
            let (whom, fact) = of.rsplit_once(' ').expect("whom and the fact");
            archogen_plan::layout::Origin {
                source: match *owner {
                    "application" => archogen_plan::layout::Source::Application {
                        of: whom.to_owned(),
                    },
                    "record" => archogen_plan::layout::Source::Record {
                        id: whom.to_owned(),
                    },
                    "realization" => archogen_plan::layout::Source::Realization {
                        subject: whom.to_owned(),
                    },
                    other => panic!("no owner is {other}"),
                },
                fact: fact.to_owned(),
                category: match *category {
                    "assumed" => archogen_plan::layout::Category::Assumed,
                    "observed-maximum" => archogen_plan::layout::Category::ObservedMaximum,
                    "externally-supplied" => archogen_plan::layout::Category::ExternallySupplied,
                    "analytically-established" => {
                        archogen_plan::layout::Category::AnalyticallyEstablished
                    }
                    other => panic!("no category is {other}"),
                },
            }
        })
        .collect();
    let (items, weakest) = archogen_plan::layout::origin(&library);
    assert_eq!(items, model.0, "the library's origin of {figures:?}");
    assert_eq!(
        weakest.map(archogen_plan::layout::Category::as_str),
        Some(model.1)
    );
    model
}

#[test]
fn an_origin_lists_every_figure_its_sum_reads_once() {
    // RP14-1's case: `console.write` 380 observed, `timer.read` 200 assumed, `mask` and `unmask` 96 each from one API record.
    let (items, weakest) = origin_held(&[
        ("realization", "console node-stack", "observed-maximum", 380),
        ("realization", "timer node-stack", "assumed", 200),
        (
            "record",
            "api.riscv primitive-stack",
            "externally-supplied",
            96,
        ),
        (
            "record",
            "api.riscv primitive-stack",
            "externally-supplied",
            96,
        ),
    ]);
    assert_eq!(weakest, "assumed", "the smaller figure is entered too");
    assert_eq!(
        items,
        [
            "(record api.riscv primitive-stack externally-supplied)",
            "(realization console node-stack observed-maximum)",
            "(realization timer node-stack assumed)"
        ],
        "one record's two figures are one item, by owner and then by text (round 15, RP15-2)"
    );
    assert!(RECORD.contains("each name's figures all and not its largest alone"));
    assert!(RECORD.contains("an item is written once however many figures write it"));
}

/// §5: an allocation's `origin` names what sizes and places its unit too, the guard and each alignment the catalog states of it, though no sum reads them: round 14's
/// "every figure the sum reads" left them out where the catalog's numbers put them in, and `/1` §4 `.7` asks their category in the plan (round 15, RP15-1).
#[test]
fn an_origin_names_what_sizes_and_places_its_unit() {
    let (items, weakest) = origin_held(&[
        (
            "realization",
            "console node-stack",
            "analytically-established",
            380,
        ),
        ("record", "port.riscv guard", "assumed", 64),
        ("record", "port.riscv alignment", "assumed", 16),
    ]);
    assert_eq!(
        weakest, "assumed",
        "RP15-1's case: the guard's and alignment's category is the allocation's too"
    );
    assert_eq!(
        items,
        [
            "(record port.riscv alignment assumed)",
            "(record port.riscv guard assumed)",
            "(realization console node-stack analytically-established)"
        ]
    );
    assert!(RECORD.contains("and the guard and each alignment the catalog states of it, which size and place it though no sum reads them"));
}

/// §5: an allocation's `origin` names each list its sums take their largest over too, a task's `node-calls` and `os-calls` and the initialisation's `node-calls`,
/// each by its own category: which figures the largest is taken over rests on them, as the composite-inputs record's §6 reads them, "parts of that order, though
/// they add nothing" (round 16, RP16-1).
#[test]
fn an_origin_names_the_lists_its_largest_is_taken_over() {
    // RP16-1's case: a stack bound and its figures established, the node calls assumed.
    let (items, weakest) = origin_held(&[
        (
            "application",
            "(task s t) stack-bound",
            "analytically-established",
            512,
        ),
        ("application", "(task s t) node-calls", "assumed", 1),
        (
            "realization",
            "console.write node-stack",
            "analytically-established",
            380,
        ),
        ("record", "port.riscv guard", "externally-supplied", 64),
    ]);
    assert_eq!(
        weakest, "assumed",
        "the stack rests on the list as on its figures"
    );
    assert_eq!(items[0], "(application (task s t) node-calls assumed)");
    assert!(RECORD.contains("and each list its sums take their largest over, a task's `node-calls` and `os-calls` and the initialisation's `node-calls`, by its own category"));
}

// ---- the library's small procedures beside the model's (leaf `M4.1.7.6.2`)

/// §3, §4 and §6's small procedures, the library's held to the model's over every small input: a route's priority and which reach the controller,
/// the ranks, the `(enabled …)` line, an alarm's `BASE`, what it owes and whether a task owes the application's word of it, a compare's reading, a rate,
/// the regions' order and the epoch's line.
#[test]
fn the_library_s_small_procedures_are_the_model_s() {
    use archogen_plan::{alarm, controller, layout};
    for levels in 0..=8 {
        for routes in 0..=9 {
            for rank in 0..=10 {
                assert_eq!(
                    controller::priority(levels, routes, rank),
                    priority(levels, routes, rank),
                    "{levels} {routes} {rank}"
                );
            }
        }
    }
    let externals = [Some(true), Some(false), None];
    let mut lists: Vec<Vec<Option<bool>>> = vec![Vec::new()];
    for _ in 0..4 {
        let longer: Vec<Vec<Option<bool>>> = lists
            .iter()
            .filter(|list| list.len() == lists.iter().map(Vec::len).max().unwrap_or(0))
            .flat_map(|list| {
                externals
                    .iter()
                    .map(move |external| [list.clone(), vec![*external]].concat())
            })
            .collect();
        lists.extend(longer);
    }
    assert_eq!(lists.len(), 1 + 3 + 9 + 27 + 81);
    for list in &lists {
        assert_eq!(controller::reaching(list), reaching(list), "{list:?}");
    }
    let items = [
        (None, "(source button)"),
        (None, "(alarm zz.watchdog)"),
        (Some(10), "(alarm a.fast)"),
        (Some(3), "(source b)"),
        (Some(10), "(alarm a.slow)"),
    ];
    for first in 0..items.len() {
        for second in 0..items.len() {
            for third in 0..items.len() {
                let chosen = vec![items[first], items[second], items[third]];
                assert_eq!(
                    controller::ranked(chosen.clone()),
                    ranked(chosen.clone()),
                    "{chosen:?}"
                );
            }
        }
    }
    for timer in [false, true] {
        for routes in [
            &[][..],
            &["(alarm a)"][..],
            &["(source s)", "(alarm a)"][..],
        ] {
            assert_eq!(
                controller::enabled(timer, routes),
                enabled(timer, routes),
                "{timer} {routes:?}"
            );
        }
    }
    let readers_of = |model: Reader| match model {
        Reader::AtTheDevice => alarm::Reader::AtTheDevice,
        Reader::ExtendedOverIt => alarm::Reader::ExtendedOverIt,
        Reader::OnAnother => alarm::Reader::OnAnother,
        Reader::ExtendedOverAnother => alarm::Reader::ExtendedOverAnother,
    };
    let mut sets: Vec<Vec<Reader>> = vec![Vec::new()];
    for a in READERS {
        sets.push(vec![a]);
        for b in READERS {
            sets.push(vec![a, b]);
            for c in READERS {
                sets.push(vec![a, b, c]);
            }
        }
    }
    for wake in [false, true] {
        for set in &sets {
            let library = alarm::base(
                wake,
                &set.iter().copied().map(readers_of).collect::<Vec<_>>(),
            );
            let model = match base(wake, set) {
                Base::Counter => alarm::Base::Counter,
                Base::Epoch => alarm::Base::Epoch,
            };
            assert_eq!(library, model, "{wake} {set:?}");
        }
        for on_the_counter in [false, true] {
            assert_eq!(
                alarm::asks_application(wake, on_the_counter),
                asks_the_application(wake, on_the_counter)
            );
            for simulated in [false, true] {
                assert_eq!(
                    alarm::owes(wake, on_the_counter, simulated),
                    alarm_owes(wake, on_the_counter, simulated)
                );
            }
        }
    }
    for window in [None, Some(0), Some(5)] {
        for magnitude in [None, Some(false), Some(true)] {
            let library = match alarm::compare_reading(window, magnitude) {
                alarm::Reading::Window(window) => Reading::Window(window),
                alarm::Reading::WholeNumbers => Reading::WholeNumbers,
                alarm::Reading::Unstated => Reading::Unstated,
                alarm::Reading::Refused => Reading::Refused,
            };
            assert_eq!(
                library,
                compare_reading(window, magnitude),
                "{window:?} {magnitude:?}"
            );
        }
    }
    for numerator in 0..=24_u64 {
        for denominator in 1..=24_u64 {
            assert_eq!(
                alarm::rate(
                    u128::from(numerator),
                    std::num::NonZeroU128::new(u128::from(denominator)).expect("not zero")
                ),
                rate(numerator, denominator),
                "{numerator}/{denominator}"
            );
        }
    }
    let regions = [
        (0x8000_0000_u64, "ram", "b"),
        (0x8000_0000, "ram", "a"),
        (0x1000, "rom", "a"),
        (0x8000_0000, "io", "z"),
    ];
    for first in 0..regions.len() {
        for second in 0..regions.len() {
            let chosen = vec![
                regions[first],
                regions[second],
                regions[(first + second) % regions.len()],
            ];
            let mut library: Vec<(u128, &str, &str)> = chosen
                .iter()
                .map(|&(base, name, provider)| (u128::from(base), name, provider))
                .collect();
            layout::in_order(&mut library);
            assert_eq!(
                library,
                in_the_plan_s_order(chosen.clone())
                    .into_iter()
                    .map(|(base, name, provider)| (u128::from(base), name, provider))
                    .collect::<Vec<_>>(),
                "{chosen:?}"
            );
        }
    }
    for hosted in [false, true] {
        for timer in [false, true] {
            for alarmed in [false, true] {
                for lead in [
                    "owed",
                    "(value 2.4us) (record port.lead) (category measured)",
                ] {
                    assert_eq!(
                        alarm::epoch_line(hosted, timer, alarmed, lead),
                        epoch_line(hosted, timer, alarmed, lead)
                    );
                }
            }
        }
    }
}
