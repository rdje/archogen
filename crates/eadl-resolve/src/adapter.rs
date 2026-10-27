//! The engine's adapters (leaf `M3.2.2`; `docs/specs/adapters/decision_engine-adapters.md` §1, §2): the catalog the
//! search binds instances from, read from `docs/semantics/adapters/adapters.eadl`, one `defadapter` entry per kind,
//! written with the kind `docs/semantics/kinds/defadapter.eadl`.
//!
//! ⭐ **Data, read twice.** This reader is the generator's; the configuration checker judges a plan by its own table,
//! `archogen_plan_check::adapters`, written from the record and never from this code, and `tests/adapters.rs` holds the
//! two equal kind by kind (record §1.2), as `tests/vocabulary.rs` holds the vocabulary.
//!
//! A kind's `beneath` is read as a side by the relation's own reader ([`requirement::read_side`]), and its literal
//! offers, with its level as `reachable-at-privilege`, by the relation's offer reader ([`offer::read`]): so every
//! refusal those readers make of a description's side or offers is made of a kind's (record §1.6), and what this module
//! adds is only what a description cannot write — a value passed through, an obliged bound, the instance's shape.

use std::collections::BTreeSet;

use eadl_front::{read, Form, SourceMap, Span};
use eadl_model::check::shipped_registry;
use eadl_model::kind::{duplicate_names, validate};

use crate::offer;
use crate::requirement::{read_side, Requirement};
use crate::value::Value;
use crate::vocabulary::{Direction, Domain, Function, Usage, Vocabulary};

/// The catalog's version: the identifier a plan of `archogen-assignment/2` names it by (record §5).
pub const VERSION: &str = "adapters/1";

/// Where the kind is, as refusals name it.
const KIND_PATH: &str = "docs/semantics/kinds/defadapter.eadl";
/// The kind the catalog is written with, embedded so the generator reads the catalog it was built with.
const KIND_TEXT: &str = include_str!("../../../docs/semantics/kinds/defadapter.eadl");
/// Where the catalog is, as refusals name it.
const CATALOG_PATH: &str = "docs/semantics/adapters/adapters.eadl";
/// The catalog, `adapters/1`.
const CATALOG_TEXT: &str = include_str!("../../../docs/semantics/adapters/adapters.eadl");

/// The head of an entry.
const ENTRY: &str = "defadapter";

/// What an instance offers of one fact (record §1.3).
#[derive(Debug, Clone, PartialEq)]
pub enum Offered {
    /// `(f v)`: the value as written, every form after the fact.
    Value(Vec<Form>),
    /// `(f (from g))`: the provider's own value of `g`, offered or derived there.
    From(String),
    /// `(f (obliged))`: the tightest bound the instance's requester writes on `f` (record §1.5).
    Obliged,
}

/// One item of a kind's `offers`.
#[derive(Debug, Clone, PartialEq)]
pub struct Offer {
    /// The fact.
    pub fact: String,
    /// How the instance offers it.
    pub offered: Offered,
    /// Where it is written.
    pub span: Span,
}

impl Offer {
    /// A value's forms as written, one space between them — the spelling the checker's table writes; `None` for an
    /// offer that is no value.
    #[must_use]
    pub fn value_text(&self) -> Option<String> {
        match &self.offered {
            Offered::Value(forms) => Some(
                forms
                    .iter()
                    .map(Form::to_canonical)
                    .collect::<Vec<_>>()
                    .join(" "),
            ),
            _ => None,
        }
    }
}

/// An obligation or a cost interrupt: its name and what it means.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Owed {
    /// Its name.
    pub name: String,
    /// What it means, one line.
    pub doc: String,
}

/// One kind of the catalog.
#[derive(Debug, Clone, PartialEq)]
pub struct Kind {
    /// Its name, the identity a plan names it by.
    pub name: String,
    /// What it does, one line.
    pub doc: String,
    /// Where its entry is written.
    pub span: Span,
    /// What it requires of the one provider it programs, as the relation reads a side.
    pub beneath: Vec<Requirement>,
    /// Every fact its `beneath` names — `needs` operands and constraint heads — in written order.
    pub beneath_names: Vec<String>,
    /// Each constraint its `beneath` writes beside its `needs`, as `(fact, value)`, the value's forms as written.
    pub beneath_constraints: Vec<(String, String)>,
    /// What an instance offers, in written order.
    pub offers: Vec<Offer>,
    /// The level of `reachable-at-privilege`'s domain its code runs at (model §8 rule 9, amended by the record §4).
    pub level: String,
    /// What the realization owes, in written order.
    pub obligations: Vec<Owed>,
    /// The interrupts its code takes, in written order.
    pub costs: Vec<Owed>,
    /// Whether it crosses a mediation boundary (`SR-H8`).
    pub mediates: bool,
}

impl Kind {
    /// Whether it is exclusive: its offers include `channels`, so each instance serves one requester (record §1.4).
    #[must_use]
    pub fn exclusive(&self) -> bool {
        self.offers.iter().any(|offer| offer.fact == "channels")
    }

    /// What it adds (record §7.1): each fact its offers name that is not passed through from itself and that its
    /// `beneath` does not name, in written order.
    #[must_use]
    pub fn adds(&self) -> Vec<&str> {
        self.offers
            .iter()
            .filter(|offer| !matches!(&offer.offered, Offered::From(from) if *from == offer.fact))
            .filter(|offer| !self.beneath_names.contains(&offer.fact))
            .map(|offer| offer.fact.as_str())
            .collect()
    }

    /// Its offer of `fact`, if it offers it.
    #[must_use]
    pub fn offer(&self, fact: &str) -> Option<&Offer> {
        self.offers.iter().find(|offer| offer.fact == fact)
    }
}

/// Why the catalog, or an entry of it, is refused (record §1.6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    /// The kind or the catalog does not read, or the kind does not load.
    Unreadable,
    /// A declaration other than a `defadapter` entry.
    NotAnEntry,
    /// An entry's frame is not the one `defadapter` declares.
    Frame,
    /// A kind named twice.
    Duplicate,
    /// An offer naming no fact of the vocabulary, or `(from g)` naming none.
    UnknownFact,
    /// An item of `offers` that is no `(fact …)`, or one `(from …)` or `(obliged)` misspells.
    NotAnOffer,
    /// A fact offered twice.
    OfferedTwice,
    /// `(f (from g))` across two domains or dimensions.
    FromAcrossDomains,
    /// An obliged fact that is not a `quantity` bound, `at-most` or `at-least`.
    ObligedNotABound,
    /// An obliged fact of a shared kind.
    ObligedByShared,
    /// A qualifier, `(function each)`, offered otherwise than passed through from itself.
    QualifierNotPassedThrough,
    /// An exclusive kind whose `channels` is not one.
    ExclusiveChannels,
    /// An exclusive kind whose `beneath` takes no channel.
    ExclusiveTakesNoChannel,
    /// A `beneath` the relation's side reader refuses.
    Beneath,
    /// A literal offer the relation's offer reader refuses, a statement fact's among them.
    Offer,
    /// A level outside `reachable-at-privilege`'s domain.
    Level,
    /// A `mediates` other than `true` or `false`.
    Mediates,
    /// An item of `obligations` or `costs` that is no `(obligation NAME (doc "…"))` or `(interrupt NAME (doc "…"))`.
    NotAnObligation,
}

/// One refusal of the catalog.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusal {
    /// Which rule refuses it.
    pub cause: Cause,
    /// The kind it is about, when it is about one.
    pub kind: Option<String>,
    /// Where.
    pub span: Option<Span>,
    /// What is wrong.
    pub detail: String,
}

impl Refusal {
    fn new(
        cause: Cause,
        kind: Option<&str>,
        span: Option<Span>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            cause,
            kind: kind.map(str::to_string),
            span,
            detail: detail.into(),
        }
    }

    /// The refusal as one line: `path:line:column: [kind] detail`.
    #[must_use]
    pub fn render(&self, sources: &SourceMap) -> String {
        let place = self
            .span
            .and_then(|span| {
                let source = sources.get(span.source)?;
                let at = source.position(span.start);
                Some(format!("{}:{}:{}: ", source.name, at.line, at.column))
            })
            .unwrap_or_default();
        let kind = self
            .kind
            .as_deref()
            .map(|k| format!("`{k}`: "))
            .unwrap_or_default();
        format!("{place}{kind}{}", self.detail)
    }
}

/// The engine's adapters: every kind, in the catalog's order.
#[derive(Debug, Clone, PartialEq)]
pub struct Catalog {
    /// The kinds, in written order — the order the search tries them in (record §3.1).
    pub kinds: Vec<Kind>,
}

impl Catalog {
    /// The catalog `adapters/1`, read against the shipped vocabulary.
    ///
    /// # Errors
    ///
    /// Every [`Refusal`] the reader finds; the shipped files are held to none by `tests/adapters.rs`.
    pub fn shipped(sources: &mut SourceMap, vocabulary: &Vocabulary) -> Result<Self, Vec<Refusal>> {
        Self::read(
            sources,
            (KIND_PATH, KIND_TEXT),
            (CATALOG_PATH, CATALOG_TEXT),
            vocabulary,
        )
    }

    /// A catalog that holds no kind: the search with it is the search over the description's own providers alone.
    #[must_use]
    pub const fn empty() -> Self {
        Self { kinds: Vec::new() }
    }

    /// The kind named `name`, if the catalog holds one.
    #[must_use]
    pub fn kind(&self, name: &str) -> Option<&Kind> {
        self.kinds.iter().find(|kind| kind.name == name)
    }

    /// Read a catalog: `kind` is the module declaring `defadapter` and `catalog` its entries, each a `(name, text)` pair
    /// added to `sources`, so every refusal's span points into them.
    ///
    /// # Errors
    ///
    /// Every [`Refusal`] found, never only the first.
    pub fn read(
        sources: &mut SourceMap,
        kind: (&str, &str),
        catalog: (&str, &str),
        vocabulary: &Vocabulary,
    ) -> Result<Self, Vec<Refusal>> {
        let entries_kind = shipped_registry(sources, &[(kind.0.to_string(), kind.1.to_string())])
            .map_err(|errors| {
            errors
                .into_iter()
                .map(|d| Refusal::new(Cause::Unreadable, None, Some(d.primary.span), d.message))
                .collect::<Vec<_>>()
        })?;
        let id = sources
            .add(catalog.0.to_string(), catalog.1.to_string())
            .map_err(|_| {
                vec![Refusal::new(
                    Cause::Unreadable,
                    None,
                    None,
                    format!("{} is too large to address", catalog.0),
                )]
            })?;
        let (document, diagnostics) = read(sources, id);
        if diagnostics.has_errors() {
            return Err(diagnostics
                .items()
                .iter()
                .map(|d| Refusal::new(Cause::Unreadable, None, Some(d.primary.span), &d.message))
                .collect());
        }
        let declarations: Vec<&Form> = document.declarations().collect();
        let mut refusals = Vec::new();
        let mut kinds = Vec::new();
        for form in &declarations {
            if form.head() != Some(ENTRY) {
                refusals.push(Refusal::new(
                    Cause::NotAnEntry,
                    None,
                    Some(form.span()),
                    "the catalog holds `defadapter` entries and nothing else",
                ));
                continue;
            }
            let frame = validate(&entries_kind, form);
            if !frame.is_empty() {
                refusals.extend(frame.into_iter().map(|d| {
                    Refusal::new(Cause::Frame, name_of(form), Some(d.primary.span), d.message)
                }));
                continue;
            }
            match entry(sources, form, vocabulary) {
                Ok(kind) => kinds.push(kind),
                Err(mut wrong) => refusals.append(&mut wrong),
            }
        }
        let entries: Vec<&Form> = declarations
            .iter()
            .copied()
            .filter(|form| form.head() == Some(ENTRY))
            .collect();
        refusals.extend(
            duplicate_names(&entries_kind, &entries)
                .into_iter()
                .map(|d| Refusal::new(Cause::Duplicate, None, Some(d.primary.span), d.message)),
        );
        if refusals.is_empty() {
            Ok(Self { kinds })
        } else {
            Err(refusals)
        }
    }
}

/// An entry's name.
fn name_of(form: &Form) -> Option<&str> {
    form.items().get(1).and_then(Form::as_symbol)
}

/// The clause headed `head` of an entry, its frame already validated.
fn clause<'f>(form: &'f Form, head: &str) -> Option<&'f Form> {
    form.items()
        .iter()
        .skip(2)
        .find(|item| item.head() == Some(head))
}

/// A clause's items after its head.
fn operands(clause: &Form) -> &[Form] {
    clause.items().get(1..).unwrap_or_default()
}

/// A string's text.
fn text(form: &Form) -> Option<&str> {
    match form {
        Form::Str { value, .. } => Some(value),
        _ => None,
    }
}

/// Read a form's canonical text back as forms, in a source of its own: the relation's readers read declarations, and a
/// kind's `beneath` and literal offers are read by them as a declaration of their own, so its refusals are theirs.
fn synthetic(sources: &mut SourceMap, label: &str, text: &str) -> Option<Form> {
    let id = sources.add(label.to_string(), text.to_string()).ok()?;
    let (document, diagnostics) = read(sources, id);
    if diagnostics.has_errors() {
        return None;
    }
    let first = document.declarations().next().cloned();
    first
}

/// Read one entry whose frame holds.
fn entry(
    sources: &mut SourceMap,
    form: &Form,
    vocabulary: &Vocabulary,
) -> Result<Kind, Vec<Refusal>> {
    let name = name_of(form).unwrap_or_default().to_string();
    let mut refusals = Vec::new();
    let refuse = |cause: Cause, span: Span, detail: String| {
        Refusal::new(cause, Some(&name), Some(span), detail)
    };
    let doc = clause(form, "doc")
        .and_then(|c| operands(c).first())
        .and_then(text)
        .unwrap_or_default()
        .to_string();

    // `beneath`, read as a side by the relation's reader.
    let beneath_clause = clause(form, "beneath").expect("the frame requires `beneath`");
    let beneath_items: Vec<String> = operands(beneath_clause)
        .iter()
        .map(Form::to_canonical)
        .collect();
    let mut beneath_names = Vec::new();
    let mut beneath_constraints = Vec::new();
    for item in operands(beneath_clause) {
        match item.head() {
            Some("needs") => beneath_names.extend(
                operands(item)
                    .iter()
                    .filter_map(Form::as_symbol)
                    .map(str::to_string),
            ),
            Some(fact) => {
                beneath_names.push(fact.to_string());
                beneath_constraints.push((
                    fact.to_string(),
                    operands(item)
                        .iter()
                        .map(Form::to_canonical)
                        .collect::<Vec<_>>()
                        .join(" "),
                ));
            }
            None => {}
        }
    }
    let side = synthetic(
        sources,
        &format!("{CATALOG_PATH} (beneath of {name})"),
        &format!(
            "(defservice adapter.{name} (requires {}))",
            beneath_items.join(" ")
        ),
    );
    let beneath = match side.as_ref().map(|decl| read_side(decl, vocabulary)) {
        Some(Ok(requirements)) => requirements,
        Some(Err(why)) => {
            refusals.extend(why.iter().map(|r| {
                refuse(
                    Cause::Beneath,
                    beneath_clause.span(),
                    format!("its `beneath` is refused as a side: {}", r.detail),
                )
            }));
            Vec::new()
        }
        None => {
            refusals.push(refuse(
                Cause::Beneath,
                beneath_clause.span(),
                "its `beneath` does not read as a side".to_string(),
            ));
            Vec::new()
        }
    };

    // `offers`, each item a value, a pass-through or an obliged bound.
    let offers_clause = clause(form, "offers").expect("the frame requires `offers`");
    let mut offers: Vec<Offer> = Vec::new();
    let mut seen = BTreeSet::new();
    for item in operands(offers_clause) {
        let (Some(fact), rest) = (item.head(), operands(item)) else {
            refusals.push(refuse(
                Cause::NotAnOffer,
                item.span(),
                format!("`{}` is no `(fact …)`", item.to_canonical()),
            ));
            continue;
        };
        if !seen.insert(fact.to_string()) {
            refusals.push(refuse(
                Cause::OfferedTwice,
                item.span(),
                format!("`{fact}` is offered twice"),
            ));
            continue;
        }
        let Some(entry) = vocabulary.fact(fact) else {
            refusals.push(refuse(
                Cause::UnknownFact,
                item.span(),
                format!("`{fact}` is no fact of the vocabulary"),
            ));
            continue;
        };
        let offered = match rest {
            [single] if single.head() == Some("from") => match operands(single) {
                [from] if from.as_symbol().is_some() => {
                    let from = from.as_symbol().unwrap_or_default();
                    match vocabulary.fact(from) {
                        None => {
                            refusals.push(refuse(
                                Cause::UnknownFact,
                                single.span(),
                                format!("`{fact}` is passed through from `{from}`, no fact of the vocabulary"),
                            ));
                            continue;
                        }
                        Some(source) if source.domain != entry.domain => {
                            refusals.push(refuse(
                                Cause::FromAcrossDomains,
                                single.span(),
                                format!("`{fact}` is passed through from `{from}`, a fact of another domain"),
                            ));
                            continue;
                        }
                        Some(_) => Offered::From(from.to_string()),
                    }
                }
                _ => {
                    refusals.push(refuse(
                        Cause::NotAnOffer,
                        single.span(),
                        format!("`{fact}`'s `from` names no one fact"),
                    ));
                    continue;
                }
            },
            [single] if single.head() == Some("obliged") => {
                if !operands(single).is_empty() {
                    refusals.push(refuse(
                        Cause::NotAnOffer,
                        single.span(),
                        format!("`{fact}`'s `(obliged)` takes nothing"),
                    ));
                    continue;
                }
                let bound = matches!(entry.domain, Domain::Quantity(_))
                    && matches!(entry.direction, Direction::AtMost | Direction::AtLeast);
                if !bound {
                    refusals.push(refuse(
                        Cause::ObligedNotABound,
                        single.span(),
                        format!("`{fact}` is obliged, and only a quantity bound, `at-most` or `at-least`, may be"),
                    ));
                    continue;
                }
                Offered::Obliged
            }
            _ => Offered::Value(rest.to_vec()),
        };
        if entry.function == Some(Function::Each) && offered != Offered::From(fact.to_string()) {
            refusals.push(refuse(
                Cause::QualifierNotPassedThrough,
                item.span(),
                format!(
                    "`{fact}` is a qualifier, offered only passed through from itself: a kind's level is its `level`"
                ),
            ));
            continue;
        }
        offers.push(Offer {
            fact: fact.to_string(),
            offered,
            span: item.span(),
        });
    }

    // `level`, judged as an offer of `reachable-at-privilege`, beside the literal offers the offer reader judges.
    let level_clause = clause(form, "level").expect("the frame requires `level`");
    let level = operands(level_clause)
        .first()
        .and_then(Form::as_symbol)
        .unwrap_or_default()
        .to_string();
    let literal: Vec<String> = offers
        .iter()
        .filter_map(|offer| {
            offer
                .value_text()
                .map(|value| format!("({} {value})", offer.fact))
        })
        .collect();
    let block = synthetic(
        sources,
        &format!("{CATALOG_PATH} (offers of {name})"),
        &format!(
            "(defblock adapter.{name} (offers {} (reachable-at-privilege {level})))",
            literal.join(" ")
        ),
    );
    let provider = match block.as_ref().map(|decl| offer::read(decl, vocabulary)) {
        Some(Ok(provider)) => Some(provider),
        Some(Err(why)) => {
            for r in &why {
                let at_level = r.fact.as_deref() == Some("reachable-at-privilege");
                refusals.push(refuse(
                    if at_level { Cause::Level } else { Cause::Offer },
                    if at_level {
                        level_clause.span()
                    } else {
                        offers_clause.span()
                    },
                    if at_level {
                        format!("its level `{level}` is refused: {}", r.detail)
                    } else {
                        format!("its offers are refused as a provider's: {}", r.detail)
                    },
                ));
            }
            None
        }
        None => {
            refusals.push(refuse(
                Cause::Offer,
                offers_clause.span(),
                "its offers do not read as a provider's".to_string(),
            ));
            None
        }
    };

    // The instance's shape: exclusive or shared (record §1.4).
    let exclusive = offers.iter().any(|offer| offer.fact == "channels");
    if exclusive {
        let one = provider.as_ref().is_some_and(|p| {
            matches!(p.state("channels"), Some(offer::State::Valued(values)) if values.first() == Some(&Value::Count(1)))
        });
        if !one {
            refusals.push(refuse(
                Cause::ExclusiveChannels,
                offers_clause.span(),
                "an exclusive kind offers `(channels 1 tick)`: one requester per instance"
                    .to_string(),
            ));
        }
        let takes = beneath.iter().any(|requirement| {
            let consumed = vocabulary
                .fact(requirement.fact())
                .is_some_and(|f| f.usage == Usage::Consumed);
            consumed
                && match requirement {
                    Requirement::Presence { .. } | Requirement::Group { .. } => true,
                    Requirement::Constraint { value, .. } => *value == Value::Bool(true),
                    Requirement::Statement { .. } => false,
                }
        });
        if !takes {
            refusals.push(refuse(
                Cause::ExclusiveTakesNoChannel,
                beneath_clause.span(),
                "an exclusive kind takes a channel beneath: its `beneath` needs a consumed fact"
                    .to_string(),
            ));
        }
    } else if let Some(obliged) = offers
        .iter()
        .find(|offer| offer.offered == Offered::Obliged)
    {
        refusals.push(refuse(
            Cause::ObligedByShared,
            obliged.span,
            format!(
                "`{}` is obliged by a shared kind, whose instance serves several requesters and so has no one bound",
                obliged.fact
            ),
        ));
    }

    let owed = |head: &str, item_head: &str, refusals: &mut Vec<Refusal>| -> Vec<Owed> {
        let Some(clause) = clause(form, head) else {
            return Vec::new();
        };
        let mut out = Vec::new();
        for item in operands(clause) {
            let parts = item.items();
            let well_formed = item.head() == Some(item_head)
                && parts.len() == 3
                && parts[1].as_symbol().is_some()
                && parts[2].head() == Some("doc")
                && parts[2].items().len() == 2
                && text(&parts[2].items()[1]).is_some();
            if well_formed {
                out.push(Owed {
                    name: parts[1].as_symbol().unwrap_or_default().to_string(),
                    doc: text(&parts[2].items()[1]).unwrap_or_default().to_string(),
                });
            } else {
                refusals.push(Refusal::new(
                    Cause::NotAnObligation,
                    Some(&name),
                    Some(item.span()),
                    format!("an item of `{head}` is `({item_head} NAME (doc \"…\"))`"),
                ));
            }
        }
        out
    };
    let obligations = owed("obligations", "obligation", &mut refusals);
    let costs = owed("costs", "interrupt", &mut refusals);

    let mediates = match clause(form, "mediates").and_then(|c| operands(c).first()) {
        None => false,
        Some(value) => match value.as_symbol() {
            Some("true") => true,
            Some("false") => false,
            _ => {
                refusals.push(refuse(
                    Cause::Mediates,
                    value.span(),
                    format!(
                        "`mediates` is `true` or `false`, not `{}`",
                        value.to_canonical()
                    ),
                ));
                false
            }
        },
    };

    if refusals.is_empty() {
        Ok(Kind {
            name,
            doc,
            span: form.span(),
            beneath,
            beneath_names,
            beneath_constraints,
            offers,
            level,
            obligations,
            costs,
            mediates,
        })
    } else {
        Err(refusals)
    }
}
