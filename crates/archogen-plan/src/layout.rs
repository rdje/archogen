//! Format `/2`'s memory procedures (`docs/decisions/decision_runtime-plan-format-2.md` §5; leaf `M4.1.7.6.1`): where the
//! image may lie, where each allocation lies, what an owed allocation waits on, what each stack holds, and what its
//! figures rest on.
//!
//! Each procedure is the record's, and `tests/format2_model.rs` holds it to the model's transcription of the same rule over
//! the model's own inputs, each small region, unit and set of figures it enumerates. Nothing here writes a plan's line:
//! the lowering does, from what these give.
//!
//! ⭐ **Addresses are `u128`.** A region may end at 2^64, one past the last byte a 64-bit address names, and `/2` writes
//! an `INT` up to 2^64 − 1 (`crate::contents`). So the procedures compute in `u128` and never wrap, and what the lowering
//! writes it checks against the plan's own limit there.

use std::fmt;
use std::num::NonZeroU128;

/// A range of addresses, `[base, end)`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Span {
    /// Its first address.
    pub base: u128,
    /// One past its last.
    pub end: u128,
}

impl Span {
    /// The range of `bytes` bytes from `base`.
    #[must_use]
    pub fn of(base: u128, bytes: u128) -> Self {
        Self {
            base,
            end: base.saturating_add(bytes),
        }
    }

    /// Whether it shares an address with `other`.
    #[must_use]
    pub fn meets(self, other: Span) -> bool {
        self.base < other.end && other.base < self.end
    }
}

/// A reserved range as the target's hardware states it: its base, and its size where one is written (model §8 rule 10).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Reserved {
    /// Its first byte.
    pub base: u128,
    /// How many bytes it holds, when its size is written.
    pub bytes: Option<u128>,
}

/// A hole: what a reserved range surely takes of the placement region, clipped to it (§5, "A hole with no size").
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Hole {
    /// The bytes it takes of the region.
    pub span: Span,
    /// Whether its size was written; one with none reaches the region's end.
    pub sized: bool,
    /// The reserved range it is, by its place among those given, so what writes `truncated-by` can name it.
    pub of: usize,
}

/// Each hole of `region`: a reserved range that surely overlaps it, by a sized range's bytes or the base of one with no
/// size, which is "taken to reach that region's end" (model §8 rule 10; §5, "A hole with no size"), clipped to the region.
/// `Joint::map` has already refused a map whose ranges rule 10 refuses, so these are a map's holes it accepted.
#[must_use]
pub fn holes(region: Span, reserved: &[Reserved]) -> Vec<Hole> {
    reserved
        .iter()
        .enumerate()
        .filter(|(_, range)| Span::of(range.base, range.bytes.unwrap_or(1)).meets(region))
        .filter_map(|(of, range)| {
            let end = range
                .bytes
                .map_or(region.end, |bytes| range.base.saturating_add(bytes));
            let span = Span {
                base: range.base.max(region.base),
                end: end.min(region.end),
            };
            (span.base < span.end).then_some(Hole {
                span,
                sized: range.bytes.is_some(),
                of,
            })
        })
        .collect()
}

/// §5, "The image lies at the bottom": "`F` is the least address at or above the placement region's base that no hole
/// covers: the base, moved past each hole that covers it, one after another, so past two that touch or overlap." Where
/// the holes leave the region no byte, which rule 10 refuses before any lowering, it is the region's end.
#[must_use]
pub fn floor(region: Span, holes: &[Hole]) -> u128 {
    let mut floor = region.base;
    while let Some(hole) = holes
        .iter()
        .find(|hole| hole.span.base <= floor && floor < hole.span.end)
    {
        floor = hole.span.end;
    }
    floor
}

/// What a unit is: a stack, whose top is aligned and whose guard lies at its low end, or a table, whose base is aligned
/// and which carries no guard (§5, "The formula").
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Shape {
    /// A stack, growing down to its guard.
    Stack,
    /// A table a generated system holds in memory.
    Table,
}

/// A unit to lay: its shape, its bytes, its guard and its alignment (§5, "The formula"): "A unit is `[base, base + guard +
/// bytes)`, its guard at the low end ... Tables carry `(guard 0)`."
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Unit {
    shape: Shape,
    bytes: u128,
    guard: u128,
    align: NonZeroU128,
}

impl Unit {
    /// A stack of `bytes`, its guard of `guard` below them, its top aligned to `align`.
    #[must_use]
    pub fn stack(bytes: u128, guard: u128, align: NonZeroU128) -> Self {
        Self {
            shape: Shape::Stack,
            bytes,
            guard,
            align,
        }
    }

    /// A table of `bytes`, its base aligned to `align`, with no guard.
    #[must_use]
    pub fn table(bytes: u128, align: NonZeroU128) -> Self {
        Self {
            shape: Shape::Table,
            bytes,
            guard: 0,
            align,
        }
    }

    /// Its shape.
    #[must_use]
    pub fn shape(&self) -> Shape {
        self.shape
    }

    /// Its bytes, beside its guard.
    #[must_use]
    pub fn bytes(&self) -> u128 {
        self.bytes
    }

    /// Its guard: `0` for a table.
    #[must_use]
    pub fn guard(&self) -> u128 {
        self.guard
    }

    /// Its alignment: a stack's top's, a table's base's.
    #[must_use]
    pub fn align(&self) -> NonZeroU128 {
        self.align
    }

    /// Where the formula tries it with the cursor at `cursor`: "top := a stack's: cursor rounded down to A; a table's:
    /// cursor. base := top - bytes - guard, a table's rounded down to A." Nothing where it would lie below address nothing.
    fn tried(&self, cursor: u128) -> Option<Span> {
        let align = self.align.get();
        let top = match self.shape {
            Shape::Stack => cursor - cursor % align,
            Shape::Table => cursor,
        };
        let base = top.checked_sub(self.bytes.checked_add(self.guard)?)?;
        let base = match self.shape {
            Shape::Stack => base,
            Shape::Table => base - base % align,
        };
        Some(Span { base, end: top })
    }
}

/// "No room" (§5): the unit at this place in §5's order would lie below the floor, or below address nothing.
/// `infeasible-configuration`, naming the unit, its size and what the region holds, with the repair to give the system
/// a larger region or to lower a bound — sentences the lowering writes, which knows the unit's name.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct NoRoom {
    /// The unit's place in the order given.
    pub unit: usize,
}

impl fmt::Display for NoRoom {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "no room for unit {} of the order", self.unit)
    }
}

/// §5, "The formula": units are laid from the top of `region` down, by a cursor that only descends; a unit that meets a
/// hole is laid again with the cursor at that hole's base; one whose base is below `floor` is refused, "no room". An owed
/// unit, `None`, makes every later unit owed, so the units laid are those before the first owed one, each its `[base,
/// top)`. Where a unit meets several holes, the formula takes "that hole's base", and the layout does not hang on which:
/// the model holds the formula so over every small region, taking the lowest and the highest.
///
/// # Errors
///
/// [`NoRoom`], naming the first unit that does not fit.
pub fn lay(
    region: Span,
    holes: &[Hole],
    units: &[Option<Unit>],
    floor: u128,
) -> Result<Vec<Span>, NoRoom> {
    let (mut cursor, mut laid) = (region.end, Vec::new());
    for (place, unit) in units.iter().map_while(|unit| *unit).enumerate() {
        let span = loop {
            let span = unit.tried(cursor).ok_or(NoRoom { unit: place })?;
            match holes
                .iter()
                .filter(|hole| hole.span.meets(span))
                .map(|hole| hole.span.base)
                .min()
            {
                // The hole's base lies below the unit's top, so the cursor descends, and the loop ends.
                Some(base) => cursor = base,
                None => break span,
            }
        };
        if span.base < floor {
            return Err(NoRoom { unit: place });
        }
        laid.push(span);
        cursor = span.base;
    }
    Ok(laid)
}

/// The image's limit and what truncates it (§5): "`L` is the less of the lowest allocation's base and the base of the
/// lowest hole above `F`"; "`owed` while the lowest allocation is, the lowest hole above `F` then standing in
/// `truncated-by` as the most the limit can be"; and "`truncated-by` names each hole whose base is `L`".
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Limit {
    /// `L`, or nothing while it is owed.
    pub limit: Option<u128>,
    /// Each hole `truncated-by` names, in the order given.
    pub truncated_by: Vec<Hole>,
}

/// [`Limit`] of an image above `floor` in `region`, `laid` the units laid, in order, and `owed` whether a unit is owed.
#[must_use]
pub fn limit(region: Span, holes: &[Hole], floor: u128, laid: &[Span], owed: bool) -> Limit {
    let above = holes
        .iter()
        .map(|hole| hole.span.base)
        .filter(|&base| base > floor)
        .min();
    let lowest = laid.last().map_or(region.end, |span| span.base);
    let at = |base: u128| -> Vec<Hole> {
        holes
            .iter()
            .filter(|hole| hole.span.base == base)
            .copied()
            .collect()
    };
    match (owed, above) {
        (true, Some(above)) => Limit {
            limit: None,
            truncated_by: at(above),
        },
        (true, None) => Limit {
            limit: None,
            truncated_by: Vec::new(),
        },
        (false, _) => {
            let limit = above.map_or(lowest, |above| above.min(lowest));
            Limit {
                limit: Some(limit),
                truncated_by: at(limit),
            }
        }
    }
}

// ---- §5: the stacks

/// The fault path's figures, which every stack adds on top of the deepest of what it holds (§5, "A fault is added, and
/// never taken among the largest"): `fault-use` is "the `fault-entry-stack` plus the larger of the `fault-path-stack`, the
/// panic's route from the handler's call, and the `fault-check-stack`, a check's and a trap's".
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Fault {
    /// `fault-entry-stack`, the generated handler's.
    pub entry: u128,
    /// `fault-path-stack`, the catalog's.
    pub path: u128,
    /// `fault-check-stack`, the catalog's.
    pub check: u128,
}

impl Fault {
    /// `fault-use`.
    #[must_use]
    pub fn used(&self) -> u128 {
        self.entry.saturating_add(self.path.max(self.check))
    }
}

/// The largest of a name's figures, and nothing where there is none (§5, "A name that is several figures").
fn largest(figures: &[u128]) -> u128 {
    figures.iter().copied().max().unwrap_or(0)
}

/// Sums that never wrap: a figure is a count of bytes a plan writes up to 2^64 − 1, and a sum past it is refused by what
/// writes it.
fn sum(terms: &[u128]) -> u128 {
    terms
        .iter()
        .fold(0, |total, &term| total.saturating_add(term))
}

/// A task's stack's figures (§5's table of sums, "a task's").
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct TaskStack {
    /// `call-frame`, the realization's frame beneath the job's code.
    pub call_frame: u128,
    /// `stack-bound`, the task's own, the application's.
    pub stack_bound: u128,
    /// `node-stack` of each node its `node-calls` name.
    pub node_stack: Vec<u128>,
    /// `primitive-stack` of each primitive its `os-calls` name.
    pub primitive_stack: Vec<u128>,
    /// `completion-stack`.
    pub completion_stack: u128,
    /// The fault path's.
    pub fault: Fault,
    /// `trap-frame`.
    pub trap_frame: u128,
}

impl TaskStack {
    /// `call-frame + stack-bound + max(node-stack, primitive-stack, completion-stack) + fault-use + trap-frame`.
    #[must_use]
    pub fn bytes(&self) -> u128 {
        sum(&[
            self.call_frame,
            self.stack_bound,
            largest(&self.node_stack)
                .max(largest(&self.primitive_stack))
                .max(self.completion_stack),
            self.fault.used(),
            self.trap_frame,
        ])
    }
}

/// The interrupt stack's figures (§5's table of sums, "the interrupt's").
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct InterruptStack {
    /// `trap-path-stack`.
    pub trap_path_stack: u128,
    /// `call-frame`, the dispatch's.
    pub call_frame: u128,
    /// `service-stack` of each route.
    pub service_stack: Vec<u128>,
    /// `timer-service-stack`, nothing under `(timer none)`, there being no timer service (RP12-4).
    pub timer_service_stack: Option<u128>,
    /// The fault path's.
    pub fault: Fault,
    /// `trap-frame`.
    pub trap_frame: u128,
}

impl InterruptStack {
    /// `trap-path-stack + call-frame + max(service-stack, timer-service-stack) + fault-use + trap-frame`.
    #[must_use]
    pub fn bytes(&self) -> u128 {
        sum(&[
            self.trap_path_stack,
            self.call_frame,
            largest(&self.service_stack).max(self.timer_service_stack.unwrap_or(0)),
            self.fault.used(),
            self.trap_frame,
        ])
    }
}

/// The boot stack's figures (§5's table of sums, "the boot's").
#[derive(Debug, Clone, PartialEq, Eq, Default)]
pub struct BootStack {
    /// `call-frame`.
    pub call_frame: u128,
    /// `initialisation-stack` of each step.
    pub initialisation_stack: Vec<u128>,
    /// `initialisation-path-stack`, the catalog's own initialisation.
    pub initialisation_path_stack: u128,
    /// `stack-bound` of the application's initialisation, nothing where it has none.
    pub stack_bound: Option<u128>,
    /// `node-stack` of each node the initialisation's `node-calls` name.
    pub node_stack: Vec<u128>,
    /// `last-act-stack`.
    pub last_act_stack: u128,
    /// `transition-stack`.
    pub transition_stack: u128,
    /// `idle-stack`, measured from the transition's call.
    pub idle_stack: u128,
    /// The fault path's.
    pub fault: Fault,
    /// `trap-frame`.
    pub trap_frame: u128,
}

impl BootStack {
    /// `call-frame + max(initialisation-stack, initialisation-path-stack, stack-bound + node-stack, last-act-stack +
    /// max(transition-stack, idle-stack)) + fault-use + trap-frame`. With no initialisation of the application's there is
    /// nothing it calls, so `stack-bound + node-stack` is nothing.
    #[must_use]
    pub fn bytes(&self) -> u128 {
        let application = self
            .stack_bound
            .map_or(0, |bound| bound.saturating_add(largest(&self.node_stack)));
        sum(&[
            self.call_frame,
            largest(&self.initialisation_stack)
                .max(self.initialisation_path_stack)
                .max(application)
                .max(
                    self.last_act_stack
                        .saturating_add(self.transition_stack.max(self.idle_stack)),
                ),
            self.fault.used(),
            self.trap_frame,
        ])
    }
}

// ---- §5: what an owed unit waits on, and what a laid one rests on

/// Who would state a figure an owed unit lacks (§3, `NEED`), in the order a line's items take: `application`,
/// `record`, `realization` — no record being named where none states it.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Owner {
    /// The application's file, of a task or the system's initialisation: `OF` as an owed line's subject is written.
    Application {
        /// Whom the file says it of.
        of: String,
    },
    /// A catalog record, which none names yet.
    Record,
    /// What the generated code states, of `SUBJECT`.
    Realization {
        /// What the code is of.
        subject: String,
    },
}

/// A figure an owed unit lacks (§3, `NEED`): `(application OF FACT)`, `(record FACT)` or `(realization SUBJECT FACT)`.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Need {
    /// Who would state it.
    pub owner: Owner,
    /// The fact.
    pub fact: String,
}

impl fmt::Display for Need {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.owner {
            Owner::Application { of } => write!(f, "(application {of} {})", self.fact),
            Owner::Record => write!(f, "(record {})", self.fact),
            Owner::Realization { subject } => write!(f, "(realization {subject} {})", self.fact),
        }
    }
}

/// The rank of a need's owner on a line: `application`, `record`, `realization` (§3, "Orders").
fn rank(owner: &Owner) -> u8 {
    match owner {
        Owner::Application { .. } => 0,
        Owner::Record => 1,
        Owner::Realization { .. } => 2,
    }
}

/// Each owed unit's `(needs …)` (§5, "An owed unit makes every later unit owed"): the figures not yet stated of its own,
/// by owner, then by text (§3, "Orders"), then, where an earlier unit is owed, `(unit UNIT)` naming the last such before
/// it, so a unit that lacks nothing of its own writes `(needs (unit UNIT))` and never an empty list. `units` is each
/// unit's text and what it lacks of its own, in §5's order; the result is each owed unit's text and its items' text.
#[must_use]
pub fn owed_needs(units: &[(String, Vec<Need>)]) -> Vec<(String, Vec<String>)> {
    let mut last_owed: Option<&str> = None;
    let mut owed = Vec::new();
    for (unit, lacks) in units {
        if lacks.is_empty() && last_owed.is_none() {
            continue;
        }
        let mut own: Vec<(u8, String)> = lacks
            .iter()
            .map(|need| (rank(&need.owner), need.to_string()))
            .collect();
        own.sort();
        own.dedup();
        let mut needs: Vec<String> = own.into_iter().map(|(_, text)| text).collect();
        if let Some(earlier) = last_owed {
            needs.push(format!("(unit {earlier})"));
        }
        owed.push((unit.clone(), needs));
        last_owed = Some(unit);
    }
    owed
}

/// A figure's `CATEGORY`, `ROADMAP.md` §7.3's four, weakest first.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Category {
    /// `assumed`.
    Assumed,
    /// `observed-maximum`.
    ObservedMaximum,
    /// `externally-supplied`.
    ExternallySupplied,
    /// `analytically-established`.
    AnalyticallyEstablished,
}

impl Category {
    /// Its text, as the application's format writes it.
    #[must_use]
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Assumed => "assumed",
            Self::ObservedMaximum => "observed-maximum",
            Self::ExternallySupplied => "externally-supplied",
            Self::AnalyticallyEstablished => "analytically-established",
        }
    }
}

/// Who states a figure a laid unit's sum reads (§3, `ORIGIN`), in the order a line's items take.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Source {
    /// The application's file, of `OF`.
    Application {
        /// Whom the file says it of.
        of: String,
    },
    /// The catalog record of this id.
    Record {
        /// Its id.
        id: String,
    },
    /// What the generated code states, of `SUBJECT`.
    Realization {
        /// What the code is of.
        subject: String,
    },
}

impl Source {
    /// Its rank on a line: `application`, `record`, `realization` (§3, "Orders").
    fn rank(&self) -> u8 {
        match self {
            Self::Application { .. } => 0,
            Self::Record { .. } => 1,
            Self::Realization { .. } => 2,
        }
    }
}

/// A figure a laid unit's sum reads (§3, `ORIGIN`): who states it, the fact, and its category.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Origin {
    /// Who states it.
    pub source: Source,
    /// The fact.
    pub fact: String,
    /// Its category.
    pub category: Category,
}

impl fmt::Display for Origin {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let category = self.category.as_str();
        match &self.source {
            Source::Application { of } => write!(f, "(application {of} {} {category})", self.fact),
            Source::Record { id } => write!(f, "(record {id} {} {category})", self.fact),
            Source::Realization { subject } => {
                write!(f, "(realization {subject} {} {category})", self.fact)
            }
        }
    }
}

/// What an allocation's `(origin …)` lists, and the weakest category among them (§5; RP14-1): every figure its sum reads,
/// each name's figures all and not its largest alone, since the stack rests on each; an item written once however many
/// figures write it; by owner, then by text (§3, "Orders"). Nothing where no figure is given.
#[must_use]
pub fn origin(figures: &[Origin]) -> (Vec<String>, Option<Category>) {
    let mut items: Vec<(u8, String)> = figures
        .iter()
        .map(|figure| (figure.source.rank(), figure.to_string()))
        .collect();
    items.sort();
    items.dedup();
    (
        items.into_iter().map(|(_, text)| text).collect(),
        figures.iter().map(|figure| figure.category).min(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    fn align(value: u128) -> NonZeroU128 {
        NonZeroU128::new(value).expect("an alignment")
    }

    /// §5's worked guard: "top `0x8800_0000`, `bytes` 1613 and guard 64 lay the guard at `[0x87FF_F973, 0x87FF_F9B3)`".
    #[test]
    fn a_guard_s_edges_fall_where_bytes_puts_them() {
        let region = Span {
            base: 0x8000_0000,
            end: 0x8800_0000,
        };
        let laid = lay(
            region,
            &[],
            &[Some(Unit::stack(1613, 64, align(16)))],
            region.base,
        )
        .expect("it fits");
        assert_eq!(
            laid,
            [Span {
                base: 0x87FF_F973,
                end: 0x8800_0000
            }]
        );
        assert_eq!(laid[0].base + 64, 0x87FF_F9B3, "the guard's top");
    }

    /// A region may end at 2^64, past the last address a `u64` holds: the procedures compute in `u128` and lay it whole.
    #[test]
    fn a_region_ending_at_two_to_the_sixty_four_is_laid_whole() {
        let top = 1_u128 << 64;
        let region = Span {
            base: top - 0x1000,
            end: top,
        };
        let holes = holes(
            region,
            &[Reserved {
                base: top - 0x1000,
                bytes: Some(0x100),
            }],
        );
        let floor = floor(region, &holes);
        assert_eq!(floor, top - 0xF00);
        let laid = lay(
            region,
            &holes,
            &[Some(Unit::stack(0x200, 0, align(16)))],
            floor,
        )
        .expect("it fits");
        assert_eq!(
            laid,
            [Span {
                base: top - 0x200,
                end: top
            }]
        );
        let limit = limit(region, &holes, floor, &laid, false);
        assert_eq!(limit.limit, Some(top - 0x200));
        assert!(limit.truncated_by.is_empty());
    }

    /// A unit larger than what lies below the cursor is no room, never a wrapped address.
    #[test]
    fn a_unit_past_address_nothing_is_no_room() {
        let region = Span {
            base: 0,
            end: 0x100,
        };
        assert_eq!(
            lay(region, &[], &[Some(Unit::stack(u128::MAX, 1, align(1)))], 0),
            Err(NoRoom { unit: 0 })
        );
        assert_eq!(
            lay(
                region,
                &[],
                &[
                    Some(Unit::table(0x80, align(1))),
                    Some(Unit::table(0x81, align(1)))
                ],
                0
            ),
            Err(NoRoom { unit: 1 })
        );
    }

    /// Under `(timer none)` there is no timer service, so its figure is nothing (RP12-4).
    #[test]
    fn under_no_timer_the_timer_service_is_nothing() {
        let stack = |timer| InterruptStack {
            trap_path_stack: 32,
            call_frame: 16,
            service_stack: vec![8, 24],
            timer_service_stack: timer,
            fault: Fault {
                entry: 4,
                path: 40,
                check: 50,
            },
            trap_frame: 272,
        };
        assert_eq!(stack(None).bytes(), 32 + 16 + 24 + 4 + 50 + 272);
        assert_eq!(stack(None).bytes(), stack(Some(0)).bytes());
        assert_eq!(stack(Some(100)).bytes(), 32 + 16 + 100 + 4 + 50 + 272);
    }

    /// A need's and an origin's text, by owner and then by text, a unit waiting on an earlier one last (§3, "Orders"; §5).
    #[test]
    fn needs_and_origins_are_written_by_owner_then_text() {
        let owed = owed_needs(&[
            (
                "(stack boot)".to_owned(),
                vec![
                    Need {
                        owner: Owner::Realization {
                            subject: "(epoch)".to_owned(),
                        },
                        fact: "last-act-stack".to_owned(),
                    },
                    Need {
                        owner: Owner::Record,
                        fact: "trap-frame".to_owned(),
                    },
                    Need {
                        owner: Owner::Application {
                            of: "heartbeat".to_owned(),
                        },
                        fact: "initialisation".to_owned(),
                    },
                ],
            ),
            ("(stack interrupt)".to_owned(), Vec::new()),
        ]);
        assert_eq!(
            owed,
            [
                (
                    "(stack boot)".to_owned(),
                    vec![
                        "(application heartbeat initialisation)".to_owned(),
                        "(record trap-frame)".to_owned(),
                        "(realization (epoch) last-act-stack)".to_owned(),
                    ]
                ),
                (
                    "(stack interrupt)".to_owned(),
                    vec!["(unit (stack boot))".to_owned()]
                ),
            ]
        );
        assert_eq!(origin(&[]), (Vec::new(), None));
    }
}
