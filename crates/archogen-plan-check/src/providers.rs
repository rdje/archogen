//! Check (2)'s provider-level refusals (leaf `M3.5.4.6`), written from `docs/semantics/model.md` §8 alone and never
//! from the generator's reading of it: rule 3, a provider offering a consumed function `true` while it declares
//! `channels` absent or `0`; rule 5, a region a provider offers that is not read whole — a base, a positive whole
//! number of bytes, each flag once; rule 10, over each system's hardware (rule 7), placement memory overlapped by a
//! device's registers or by another placement region, or reserved in every byte, a byte surely reserved being a sized
//! hole's or the base of one with no size. Each is `invalid-description`, a size past the exact arithmetic
//! `unsupported-profile`. Rule 10's `missing-fact` is placement memory with no size; a device's registers or a reserved
//! range with no size whose base lies below placement memory, where only that size says whether they overlap; and holes
//! of no size, each taken to reach the memory's end, that leave it no free byte while the bytes surely reserved do not
//! fill it. All of them description-wide, bound or not, composed or not, the code [`PLAN_DESCRIPTION`].

use std::collections::{BTreeMap, BTreeSet};

use archogen_outcome::record::{Finding, Severity};
use archogen_outcome::Status;
use eadl_front::{Form, Program, SourceMap, Span};
use eadl_model::quantity::{Dimension, Quantity};

use crate::check::{at, Verdict, PLAN_DESCRIPTION};
use crate::model::vocab::{Domain, VOCABULARY};
use crate::requesters::{declarations, hardware};

/// A region a provider offers, read whole (rule 5): its bytes `[base, base + size)`, or at least its base byte where
/// it states no size.
#[derive(Debug, Clone)]
struct Region {
    provider: String,
    name: String,
    base: i128,
    size: Option<i128>,
    executable: bool,
    reserved: bool,
    span: Span,
}

impl Region {
    /// Rule 10's placement memory: executable and not reserved.
    const fn placement(&self) -> bool {
        self.executable && !self.reserved
    }

    /// A device's registers: neither executable nor reserved.
    const fn device(&self) -> bool {
        !self.executable && !self.reserved
    }

    /// The bytes it is known to own: all of them, or its base byte alone.
    fn known_end(&self) -> i128 {
        self.base + self.size.unwrap_or(1)
    }

    /// Whether the bytes it is known to own meet those `other` is known to own.
    fn meets(&self, other: &Self) -> bool {
        self.base < other.known_end() && other.base < self.known_end()
    }
}

/// Whether a region against placement memory overlaps it: surely, surely not, or undecided where the region states no
/// size and the memory's bytes lie above its base. A base inside the memory decides it, rule 5's base byte being one of
/// the memory's, so undecided is a base below the memory (rule 10, as rule 5 reads it).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Overlap {
    Yes,
    No,
    Undecided,
}

fn overlap(memory: &Region, region: &Region) -> Overlap {
    if memory.meets(region) {
        Overlap::Yes
    } else if region.size.is_none() && region.base < memory.base {
        Overlap::Undecided
    } else {
        Overlap::No
    }
}

/// Whether `ranges`, each `[from, to)`, leave no byte of `[start, end)` uncovered.
pub(crate) fn fills(
    ranges: impl IntoIterator<Item = (i128, i128)>,
    start: i128,
    end: i128,
) -> bool {
    let mut ranges: Vec<(i128, i128)> = ranges.into_iter().collect();
    ranges.sort_unstable();
    let mut covered = start;
    for (from, to) in ranges {
        if from > covered {
            break;
        }
        covered = covered.max(to);
    }
    covered >= end
}

/// One refusal, before it is a finding.
struct Refused {
    status: Status,
    message: String,
    span: Span,
    repair: &'static str,
}

/// Check (2)'s provider-level refusals over the elaborated `program`, `sources` its files.
///
/// # Errors
///
/// The [`Verdict`] refusing the description: `invalid-description` where any rule refuses it, else
/// `unsupported-profile`, else `missing-fact`, every refusal a finding.
pub fn refusals(sources: &SourceMap, program: &Program) -> Result<(), Verdict> {
    let mut refused = Vec::new();
    let declarations = declarations(program);
    let mut regions: BTreeMap<String, Vec<Region>> = BTreeMap::new();
    for (qualified, (_, form)) in &declarations {
        if matches!(form.head(), Some("defblock" | "defplatform")) {
            channels(qualified, form, &mut refused);
            regions.insert(
                qualified.clone(),
                read_regions(qualified, form, &mut refused),
            );
        }
    }
    let mut said = BTreeSet::new();
    for (qualified, (_, form)) in &declarations {
        if form.head() == Some("defsystem") {
            let hardware = hardware(program, &declarations, qualified);
            let owned: Vec<&Region> = hardware
                .iter()
                .filter_map(|provider| regions.get(provider))
                .flatten()
                .collect();
            placement(qualified, &owned, &mut said, &mut refused);
        }
    }
    if refused.is_empty() {
        return Ok(());
    }
    let status = [
        Status::InvalidDescription,
        Status::UnsupportedProfile,
        Status::MissingFact,
    ]
    .into_iter()
    .find(|status| refused.iter().any(|r| r.status == *status))
    .unwrap_or(Status::InvalidDescription);
    Err(Verdict {
        status,
        findings: refused
            .into_iter()
            .map(|r| Finding {
                code: PLAN_DESCRIPTION.to_owned(),
                severity: Severity::Error,
                message: r.message,
                primary: at(sources, r.span, "here"),
                secondary: Vec::new(),
                repair: r.repair.to_owned(),
            })
            .collect(),
    })
}

/// The items of each top-level clause of `form` headed `head`.
fn clauses<'f>(form: &'f Form, head: &'f str) -> impl Iterator<Item = &'f Form> + 'f {
    form.items()
        .iter()
        .filter(move |clause| clause.head() == Some(head))
        .flat_map(|clause| &clause.items()[1..])
}

/// Rule 3: a provider offering a consumed function `true` — the fact bare, `(f true)`, or its group — while it
/// declares `channels` absent or offers `(channels 0)`.
fn channels(provider: &str, form: &Form, refused: &mut Vec<Refused>) {
    let consumed = |name: &str| VOCABULARY.iter().find(|e| e.name == name && e.consumed);
    let offered = clauses(form, "offers").find_map(|offer| {
        let (name, rest) = match offer {
            Form::Symbol { name, .. } => (name.as_str(), &[][..]),
            Form::List { items, .. } => (items.first()?.as_symbol()?, &items[1..]),
            _ => return None,
        };
        let entry = consumed(name)?;
        let as_true = match rest {
            [] => true,
            [Form::Symbol { name, .. }] => name == "true",
            _ => matches!(entry.domain, Domain::Group(_)),
        };
        as_true.then_some((name, offer.span()))
    });
    let Some((function, span)) = offered else {
        return;
    };
    let absent = clauses(form, "absent").any(|f| f.as_symbol() == Some("channels"));
    let zero = clauses(form, "offers").any(|offer| {
        matches!(offer.items(), [head, Form::Integer { value: 0, .. }] if head.as_symbol() == Some("channels"))
    });
    if absent || zero {
        refused.push(Refused {
            status: Status::InvalidDescription,
            message: format!(
                "`{provider}` offers the consumed function `{function}` and {} — a requester bound to it takes a \
                 channel it cannot have (rule 3)",
                if absent { "declares `channels` absent" } else { "offers `(channels 0)`" }
            ),
            span,
            repair: "offer `(channels N)`, N at least 1, or offer the function `false`",
        });
    }
}

/// Rule 5: every region a provider offers, those read whole returned; every other refused.
fn read_regions(provider: &str, form: &Form, refused: &mut Vec<Refused>) -> Vec<Region> {
    let mut out = Vec::new();
    for offer in clauses(form, "offers").filter(|offer| offer.head() == Some("region")) {
        let mut refuse = |status, why: String| {
            refused.push(Refused {
                status,
                message: format!("`{provider}` offers a region not read whole: {why} (rule 5)"),
                span: offer.span(),
                repair: "write `(region NAME (base B) (size S))`, `B` a non-negative address, `S` a positive whole \
                         number of bytes, and `(executable …)` and `(reserved …)` at most once each, `true` or `false`",
            });
        };
        let Some(name) = offer.items().get(1).and_then(Form::as_symbol) else {
            refuse(Status::InvalidDescription, "it names nothing".to_owned());
            continue;
        };
        let (mut base, mut size, mut executable, mut reserved) = (None, None, None, None);
        let mut whole = true;
        for sub in &offer.items()[2..] {
            let key = sub.head().unwrap_or("");
            let seen = match key {
                "base" => base.is_some(),
                "size" => size.is_some(),
                "executable" => executable.is_some(),
                "reserved" => reserved.is_some(),
                _ => {
                    refuse(
                        Status::InvalidDescription,
                        format!("`{}` is no sub-form a region takes", sub.to_canonical()),
                    );
                    whole = false;
                    continue;
                }
            };
            if seen {
                refuse(
                    Status::InvalidDescription,
                    format!("`{key}` is written twice"),
                );
                whole = false;
                continue;
            }
            match (key, &sub.items()[1..]) {
                ("base", [Form::Integer { value, .. }]) if *value >= 0 => {
                    base = Some(i128::from(*value))
                }
                ("size", [number, unit]) => match bytes(number, unit, sub.span()) {
                    Ok(n) => size = Some(n),
                    Err((status, why)) => {
                        refuse(status, why);
                        whole = false;
                    }
                },
                ("executable", [flag]) if matches!(flag.as_symbol(), Some("true" | "false")) => {
                    executable = Some(flag.as_symbol() == Some("true"));
                }
                ("reserved", [flag]) if matches!(flag.as_symbol(), Some("true" | "false")) => {
                    reserved = Some(flag.as_symbol() == Some("true"));
                }
                _ => {
                    refuse(
                        Status::InvalidDescription,
                        format!("`{}` holds no value its sub-form takes", sub.to_canonical()),
                    );
                    whole = false;
                }
            }
        }
        let Some(base) = base else {
            if whole {
                refuse(Status::InvalidDescription, format!("`{name}` has no base"));
            }
            continue;
        };
        if whole {
            out.push(Region {
                provider: provider.to_owned(),
                name: name.to_owned(),
                base,
                size,
                executable: executable.unwrap_or(false),
                reserved: reserved.unwrap_or(false),
                span: offer.span(),
            });
        }
    }
    out
}

/// A size in bytes: a positive whole number of them, else the status refusing it and why.
fn bytes(number: &Form, unit: &Form, within: Span) -> Result<i128, (Status, String)> {
    let quantity = Quantity::read(Some(number), Some(unit), within).map_err(|d| {
        let status = if d.code == "quantity-overflow" {
            Status::UnsupportedProfile
        } else {
            Status::InvalidDescription
        };
        (status, format!("its size does not read: {}", d.message))
    })?;
    if quantity.unit.dimension != Dimension::Information {
        return Err((
            Status::InvalidDescription,
            format!("its size is no amount of storage, `{}`", quantity.unit.name),
        ));
    }
    let bits = quantity.in_base().map_err(|_| {
        (
            Status::UnsupportedProfile,
            "its size lies past the exact arithmetic".to_owned(),
        )
    })?;
    let whole = bits.is_integer() && bits.is_positive() && bits.numerator() % 8 == 0;
    if !whole {
        return Err((
            Status::InvalidDescription,
            "its size is no positive whole number of bytes".to_owned(),
        ));
    }
    Ok(bits.numerator() / 8)
}

/// Rule 10 over one system's hardware's regions: placement memory with no size, overlapped by a device's registers or
/// by another placement region, against a region whose overlap no size decides, or left nothing by its holes; each
/// refusal said once across systems.
fn placement(
    system: &str,
    owned: &[&Region],
    said: &mut BTreeSet<String>,
    refused: &mut Vec<Refused>,
) {
    let mut say = |status, message: String, span, repair| {
        if said.insert(message.clone()) {
            refused.push(Refused {
                status,
                message,
                span,
                repair,
            });
        }
    };
    let placements: Vec<&Region> = owned.iter().copied().filter(|r| r.placement()).collect();
    for p in placements.iter().filter(|p| p.size.is_none()) {
        say(
            Status::MissingFact,
            format!(
                "`{}`'s placement memory `{}` has no size: nothing can be laid out in it (rule 10)",
                p.provider, p.name
            ),
            p.span,
            "state the memory's `(size …)`",
        );
    }
    for p in &placements {
        for other in owned.iter().copied().filter(|r| r.device()) {
            match overlap(p, other) {
                Overlap::Yes => say(
                    Status::InvalidDescription,
                    format!(
                        "`{}`'s region `{}` lies in `{}`'s placement memory `{}`, two claims on the same bytes (rule 10)",
                        other.provider, other.name, p.provider, p.name
                    ),
                    other.span,
                    "mark the range `(reserved true)`, or move it out of the memory",
                ),
                Overlap::Undecided => say(
                    Status::MissingFact,
                    format!(
                        "whether `{}`'s region `{}` lies in `{}`'s placement memory `{}` is undecided: it states no size \
                         and the memory's bytes lie above its base (rule 10, as rule 5 reads it)",
                        other.provider, other.name, p.provider, p.name
                    ),
                    other.span,
                    "state the region's `(size …)`",
                ),
                Overlap::No => {}
            }
        }
    }
    // A reserved range against placement memory is a hole the plan avoids, so its extent is the plan's to know: one
    // whose overlap no size decides leaves the description a fact short, as a device's does (rule 10).
    for p in &placements {
        for hole in owned.iter().copied().filter(|r| r.reserved) {
            if overlap(p, hole) == Overlap::Undecided {
                say(
                    Status::MissingFact,
                    format!(
                        "whether `{}`'s reserved range `{}` is a hole in `{}`'s placement memory `{}` is undecided: it \
                         states no size and the memory's bytes lie above its base (rule 10, as rule 5 reads it)",
                        hole.provider, hole.name, p.provider, p.name
                    ),
                    hole.span,
                    "state the range's `(size …)`",
                );
            }
        }
    }
    for (i, a) in placements.iter().enumerate() {
        for b in placements[i + 1..].iter().filter(|b| a.meets(b)) {
            let (first, second) = if (a.provider.as_str(), a.name.as_str())
                <= (b.provider.as_str(), b.name.as_str())
            {
                (a, b)
            } else {
                (b, a)
            };
            say(
                Status::InvalidDescription,
                format!(
                    "`{}`'s placement memory `{}` and `{}`'s `{}` overlap, two claims on the same bytes (rule 10)",
                    first.provider, first.name, second.provider, second.name
                ),
                second.span,
                "move one placement region off the other, or mark the shared range `(reserved true)`",
            );
        }
    }
    // What the holes leave of a region with a size. The bytes surely reserved are what each hole is known to own; the
    // conservative extent takes each hole of no size whose base lies inside to the region's end.
    let holes: Vec<&Region> = owned.iter().copied().filter(|r| r.reserved).collect();
    for p in &placements {
        let Some(size) = p.size else { continue };
        let (start, end) = (p.base, p.base + size);
        let surely = || holes.iter().map(|r| (r.base, r.known_end()));
        if fills(surely(), start, end) {
            say(
                Status::InvalidDescription,
                format!(
                    "`{}`'s placement memory `{}` is reserved in every byte, leaving nothing to place in for `{system}` \
                     (rule 10)",
                    p.provider, p.name
                ),
                p.span,
                "leave some of the memory unreserved, or offer placement memory elsewhere",
            );
            continue;
        }
        let open: Vec<&Region> = holes
            .iter()
            .copied()
            .filter(|r| r.size.is_none() && (start..end).contains(&r.base))
            .collect();
        let Some(first) = open.first() else { continue };
        if fills(
            surely().chain(open.iter().map(|r| (r.base, end))),
            start,
            end,
        ) {
            let named: Vec<String> = open
                .iter()
                .map(|r| format!("`{}`'s `{}`", r.provider, r.name))
                .collect();
            say(
                Status::MissingFact,
                format!(
                    "`{}`'s placement memory `{}` has no free byte for `{system}` with its holes of no size, {}, taken \
                     to reach its end, and the bytes surely reserved do not fill it (rule 10)",
                    p.provider,
                    p.name,
                    named.join(", ")
                ),
                first.span,
                "state the `(size …)` of each hole of no size",
            );
        }
    }
}
