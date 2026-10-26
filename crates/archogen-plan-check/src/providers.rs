//! Check (2)'s provider-level refusals (leaf `M3.5.4.6`), written from `docs/semantics/model.md` §8 alone and never
//! from the generator's reading of it: rule 3, a provider offering a consumed function `true` while it declares
//! `channels` absent or `0`; rule 5, a region a provider offers that is not read whole — a base, a positive whole
//! number of bytes, each flag once; rule 10, over each system's hardware (rule 7), placement memory overlapped by a
//! device's registers or by another placement region, or reserved in every byte. Each is `invalid-description`, an
//! overlap a region without a size leaves undecided `missing-fact`, a size past the exact arithmetic
//! `unsupported-profile` — description-wide, bound or not, composed or not, the code [`PLAN_DESCRIPTION`].

use std::collections::{BTreeMap, BTreeSet};

use archogen_outcome::record::{Finding, Severity};
use archogen_outcome::Status;
use eadl_front::{Form, Program, SourceMap, Span};
use eadl_model::quantity::{Dimension, Quantity};

use crate::check::{at, Verdict, PLAN_DESCRIPTION};
use crate::model::vocab::{Domain, VOCABULARY};

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
}

/// Whether two regions' bytes meet: surely, surely not, or undecided where one states no size and the other's bytes
/// lie at or above its base (rule 5).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Overlap {
    Yes,
    No,
    Undecided,
}

fn overlap(a: &Region, b: &Region) -> Overlap {
    if a.base < b.known_end() && b.base < a.known_end() {
        return Overlap::Yes;
    }
    let above = |sizeless: &Region, other: &Region| {
        sizeless.size.is_none() && other.known_end() > sizeless.base
    };
    if above(a, b) || above(b, a) {
        Overlap::Undecided
    } else {
        Overlap::No
    }
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
    // Every declaration by its qualified name, with the instance that wrote it.
    let mut declarations: BTreeMap<String, (usize, &Form)> = BTreeMap::new();
    for instance in &program.instances {
        for form in &instance.declarations {
            if let Some(name) = form.items().get(1).and_then(Form::as_symbol) {
                declarations.insert(instance.qualify(name), (instance.id, form));
            }
        }
    }
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
    for (qualified, (id, form)) in &declarations {
        if form.head() == Some("defsystem") {
            let hardware = hardware(program, &declarations, *id, form);
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

/// Rule 7: a system's hardware — the blocks and platforms its `platform` clause, its own `uses` and `needs` and its
/// tasks' name, and every block or platform those name in turn through `uses` or `needs`, a clause of their own or
/// inside `requires`, at any depth; `refines` no edge.
fn hardware(
    program: &Program,
    declarations: &BTreeMap<String, (usize, &Form)>,
    id: usize,
    system: &Form,
) -> BTreeSet<String> {
    let is_hardware = |name: &String| {
        declarations
            .get(name)
            .is_some_and(|(_, form)| matches!(form.head(), Some("defblock" | "defplatform")))
    };
    let mut seeds = Vec::new();
    names(system, &mut seeds, true);
    let mut reached = BTreeSet::new();
    let mut todo: Vec<String> = resolve(program, id, &seeds)
        .into_iter()
        .filter(is_hardware)
        .collect();
    while let Some(provider) = todo.pop() {
        if !reached.insert(provider.clone()) {
            continue;
        }
        let (pid, form) = declarations[&provider];
        let mut named = Vec::new();
        names(form, &mut named, false);
        todo.extend(
            resolve(program, pid, &named)
                .into_iter()
                .filter(is_hardware),
        );
    }
    reached
}

/// The operands of every `uses` and `needs` a declaration writes — its own, inside `requires`, and, for a system,
/// inside its `platform` clause and its tasks.
fn names<'f>(form: &'f Form, out: &mut Vec<(&'f str, Span)>, system: bool) {
    for clause in &form.items()[1..] {
        match clause.head() {
            Some("uses" | "needs") => out.extend(
                clause.items()[1..]
                    .iter()
                    .filter_map(|f| f.as_symbol().map(|name| (name, f.span()))),
            ),
            Some("requires") => names(clause, out, false),
            Some("platform" | "task") if system => names(clause, out, false),
            _ => {}
        }
    }
}

fn resolve(program: &Program, id: usize, names: &[(&str, Span)]) -> Vec<String> {
    names
        .iter()
        .filter_map(|(name, span)| program.resolve(id, name, *span).ok().flatten())
        .collect()
}

/// Rule 10 over one system's hardware's regions: placement memory overlapped by a device's registers or by another
/// placement region, or reserved in every byte; each pair said once across systems.
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
                        "whether `{}`'s region `{}` lies in `{}`'s placement memory `{}` is undecided: a region states \
                         no size (rule 10, as rule 5 reads it)",
                        other.provider, other.name, p.provider, p.name
                    ),
                    other.span,
                    "state each region's `(size …)`",
                ),
                Overlap::No => {}
            }
        }
    }
    for (i, a) in placements.iter().enumerate() {
        for b in &placements[i + 1..] {
            let (first, second) = if (a.provider.as_str(), a.name.as_str())
                <= (b.provider.as_str(), b.name.as_str())
            {
                (a, b)
            } else {
                (b, a)
            };
            match overlap(first, second) {
                Overlap::Yes => say(
                    Status::InvalidDescription,
                    format!(
                        "`{}`'s placement memory `{}` and `{}`'s `{}` overlap, two claims on the same bytes (rule 10)",
                        first.provider, first.name, second.provider, second.name
                    ),
                    second.span,
                    "move one placement region off the other, or mark the shared range `(reserved true)`",
                ),
                Overlap::Undecided => say(
                    Status::MissingFact,
                    format!(
                        "whether `{}`'s placement memory `{}` and `{}`'s `{}` overlap is undecided: a region states no \
                         size (rule 10, as rule 5 reads it)",
                        first.provider, first.name, second.provider, second.name
                    ),
                    second.span,
                    "state each region's `(size …)`",
                ),
                Overlap::No => {}
            }
        }
    }
    for p in &placements {
        let Some(size) = p.size else { continue };
        let mut holes: Vec<(i128, i128)> = owned
            .iter()
            .filter(|r| r.reserved)
            .filter_map(|r| {
                r.size
                    .map(|s| (r.base.max(p.base), (r.base + s).min(p.base + size)))
            })
            .filter(|(start, end)| start < end)
            .collect();
        holes.sort_unstable();
        let mut covered = p.base;
        for (start, end) in holes {
            if start > covered {
                break;
            }
            covered = covered.max(end);
        }
        if covered >= p.base + size {
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
        }
    }
}
