//! The capability vocabulary as a typed table: `docs/semantics/vocabulary/vocabulary.eadl`, read against the kind
//! `docs/semantics/kinds/deffact.eadl` (`docs/decisions/decision_substitutability-relation.md` §1.1; leaf
//! `M3.1.2.2`).
//!
//! ⭐ **The relation asks this table, and nothing else, for a fact's domain, role and direction.** Every fact the
//! relation judges is an entry here; a constraint on a fact it does not declare is `unsupported-profile` (record
//! §1.1). The table is built in two passes. First, each entry's **frame** is held to the kind with
//! `eadl_model::kind::validate`, the one validator every declaration meets. Then the reader checks what no frame
//! can say: that a domain, a role, a direction or a rule is one the record lists; that a direction is one its
//! domain admits (record §2's table); that `implies` is written only on a set fact and names its members; that
//! `derived-from`, `reads` and a group name declared facts; that a derivation names the rule that computes it, with
//! the inputs that rule reads; that `derived-from` and `reads` close no cycle (record §1.1, §9); that no name is
//! declared twice (reference §7 rule 6); and that no fact is named like a clause word of the description kinds,
//! since an offer or a requirement on it would read as a clause (record §1.1, R31 1).
//!
//! ⛔ **Written from the record, not from the executable model.** [`crate::model::vocab`] holds `/1` by hand, on
//! purpose, so a fault in reading the file cannot also be the model's: `tests/vocabulary.rs` holds this table to
//! the model's entry by entry.
//!
//! The shipped vocabulary and its kind are embedded at compile time, as the shipped kind modules are
//! (`crates/archogen-api/src/lib.rs`'s `KIND_MODULES`): a language definition a file in some directory could
//! shadow is one an accident can change.

use std::collections::{BTreeMap, BTreeSet};

use eadl_front::{read, Form, SourceMap, Span};
use eadl_model::check::shipped_registry;
use eadl_model::kind::{duplicate_names, validate, Registry};
use eadl_model::Dimension;

/// Where the shipped kind `deffact` lives, relative to the repository root.
pub const KIND_PATH: &str = "docs/semantics/kinds/deffact.eadl";

/// Where the shipped vocabulary `/1` lives, relative to the repository root.
pub const VOCABULARY_PATH: &str = "docs/semantics/vocabulary/vocabulary.eadl";

const KIND_TEXT: &str = include_str!("../../../docs/semantics/kinds/deffact.eadl");
const VOCABULARY_TEXT: &str = include_str!("../../../docs/semantics/vocabulary/vocabulary.eadl");

/// The head every entry is written with.
const ENTRY: &str = "deffact";

/// A fact's value kind (record §2).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Domain {
    /// `true` or `false`; a bare offer is `true`.
    Boolean,
    /// A non-negative integer, `(pow2 N)` included.
    Count,
    /// A number and a unit of one dimension.
    Quantity(Dimension),
    /// Two quantities of one dimension, `lo ≤ hi`, or a point.
    Interval(Dimension),
    /// One of the alternatives, written lowest first when `ordered`.
    Enumeration {
        /// The alternatives, in the order written.
        alternatives: Vec<String>,
        /// Whether the entry declares a total order, so `at-least` and `at-most` read it.
        ordered: bool,
    },
    /// A non-empty set of the alternatives, without repetition.
    Set(Vec<String>),
    /// A boolean head whose sub-facts are facts in their own right.
    Group(Vec<String>),
}

impl Domain {
    /// Whether a fact of this domain may take `direction` (record §2's Direction column).
    #[must_use]
    pub fn admits(&self, direction: Direction) -> bool {
        use Direction::{AtLeast, AtMost, Exact, Includes, Within};
        match self {
            Self::Boolean | Self::Group(_) => direction == Exact,
            Self::Count | Self::Quantity(_) => matches!(direction, AtLeast | AtMost | Exact),
            Self::Interval(_) => matches!(direction, Within | Exact),
            Self::Enumeration { ordered, .. } => {
                direction == Exact || (*ordered && matches!(direction, AtLeast | AtMost))
            }
            Self::Set(_) => matches!(direction, Includes | Exact),
        }
    }
}

/// Whom a fact speaks for (record §1.1, §3 rules 4 and 6).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Role {
    /// A promise the offering provider makes, what it accepts of its caller included.
    Guarantee,
    /// The requiring side's word about itself; it constrains no provider.
    Statement,
}

/// How an offered value satisfies a required one (record §2's table).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    /// A larger value satisfies.
    AtLeast,
    /// A smaller value satisfies.
    AtMost,
    /// Only an equal value satisfies; a description writes it `exactly`.
    Exact,
    /// The offered set contains every required member.
    Includes,
    /// The required point or interval lies inside the offered interval.
    Within,
}

impl Direction {
    fn named(word: &str) -> Option<Self> {
        Some(match word {
            "at-least" => Self::AtLeast,
            "at-most" => Self::AtMost,
            "exact" => Self::Exact,
            "includes" => Self::Includes,
            "within" => Self::Within,
            _ => return None,
        })
    }

    /// The word the vocabulary writes.
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::AtLeast => "at-least",
            Self::AtMost => "at-most",
            Self::Exact => "exact",
            Self::Includes => "includes",
            Self::Within => "within",
        }
    }
}

/// An engine rule a derived fact names (record §4's table).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rule {
    /// `(modulus − 1) / rate` seconds, when `wrap-behavior` is `modular` or undescribed.
    HorizonFromModulusAndRate,
}

impl Rule {
    /// Every rule the engine has.
    pub const ALL: &'static [Self] = &[Self::HorizonFromModulusAndRate];

    fn named(word: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|rule| rule.name() == word)
    }

    /// The name a `rule` clause writes.
    #[must_use]
    pub const fn name(self) -> &'static str {
        match self {
            Self::HorizonFromModulusAndRate => "horizon-from-modulus-and-rate",
        }
    }

    /// Why `fact`, whose inputs are `required` and `optional`, is not what this rule computes, if it is not.
    fn mismatch(self, fact: &Fact, required: &[&Fact], optional: &[&Fact]) -> Option<String> {
        match self {
            Self::HorizonFromModulusAndRate => {
                if fact.domain != Domain::Quantity(Dimension::Time) {
                    return Some(format!("`{}` computes a quantity of time", self.name()));
                }
                let inputs_are = matches!(
                    required,
                    [modulus, rate] if modulus.domain == Domain::Count
                        && rate.domain == Domain::Quantity(Dimension::Frequency)
                );
                if !inputs_are {
                    return Some(format!(
                        "`{}` is derived from a count, the modulus, then a quantity of frequency, the rate",
                        self.name()
                    ));
                }
                let optional_is = match optional {
                    [] => true,
                    [wrap] => matches!(
                        &wrap.domain,
                        Domain::Enumeration { alternatives, .. }
                            if alternatives.iter().any(|a| a == "modular")
                    ),
                    _ => false,
                };
                if !optional_is {
                    return Some(format!(
                        "`{}` reads at most one optional input, an enumeration holding `modular`",
                        self.name()
                    ));
                }
                None
            }
        }
    }
}

/// One entry of the vocabulary.
#[derive(Debug, Clone)]
pub struct Fact {
    /// Its name.
    pub name: String,
    /// What it means, one line.
    pub doc: String,
    /// Its value kind.
    pub domain: Domain,
    /// Whom it speaks for.
    pub role: Role,
    /// How an offered value satisfies a required one.
    pub direction: Direction,
    /// The facts its rule requires, in the order the rule reads them.
    pub derived_from: Vec<String>,
    /// The facts its rule reads beside those, optionally.
    pub reads: Vec<String>,
    /// The rule that computes it, for a derived fact.
    pub rule: Option<Rule>,
    /// For a set fact, the members every valued constraint on it holds beside what it writes.
    pub implies: Vec<String>,
    /// Where the entry is written.
    pub span: Span,
}

impl Fact {
    /// For an ordered enumeration, its alternatives lowest first; empty otherwise.
    #[must_use]
    pub fn order(&self) -> &[String] {
        match &self.domain {
            Domain::Enumeration {
                alternatives,
                ordered: true,
            } => alternatives,
            _ => &[],
        }
    }
}

/// Why the vocabulary, or one entry of it, is refused.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    /// The kind or the vocabulary does not read, or the kind does not load.
    Unreadable,
    /// A declaration other than a `deffact` entry.
    NotAnEntry,
    /// An entry's frame is not the one `deffact` declares.
    Frame,
    /// A name declared twice (reference §7 rule 6).
    Duplicate,
    /// A fact named like a clause word of the description kinds (record §1.1, R31 1).
    ClauseWord,
    /// A domain record §2 does not list, or written wrong.
    Domain,
    /// A role other than `guarantee` and `statement`.
    Role,
    /// A direction other than the five, or one its domain does not admit.
    Direction,
    /// `derived-from`, `reads` or a group naming a fact the vocabulary does not declare.
    UndeclaredFact,
    /// `derived-from` and `reads` closing a cycle (record §1.1, §9).
    Cycle,
    /// A rule the engine does not have; a derivation without its rule, or a rule without its inputs; inputs that are
    /// not the rule's.
    Rule,
    /// `implies` on a fact that is not a set, empty, or naming what is not a member of the set.
    Implies,
}

/// One refusal: what, where, and the entry it is about.
#[derive(Debug, Clone)]
pub struct Refusal {
    /// Which rule refuses it.
    pub cause: Cause,
    /// The entry it is about, when it is about one.
    pub fact: Option<String>,
    /// Where.
    pub span: Option<Span>,
    /// What is wrong, in a sentence.
    pub detail: String,
}

impl Refusal {
    fn new(
        cause: Cause,
        fact: Option<&str>,
        span: Option<Span>,
        detail: impl Into<String>,
    ) -> Self {
        Self {
            cause,
            fact: fact.map(str::to_string),
            span,
            detail: detail.into(),
        }
    }

    /// The refusal as one line: `path:line:column: [fact] detail`.
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
        let fact = self
            .fact
            .as_deref()
            .map(|f| format!("`{f}`: "))
            .unwrap_or_default();
        format!("{place}{fact}{}", self.detail)
    }
}

/// The capability vocabulary, typed: every fact the relation can judge.
#[derive(Debug, Clone)]
pub struct Vocabulary {
    facts: Vec<Fact>,
    index: BTreeMap<String, usize>,
    clause_words: BTreeSet<String>,
}

impl Vocabulary {
    /// The vocabulary `eadl/1` ships, read against the description kinds `kinds`, whose clause words no fact may be
    /// named like.
    ///
    /// # Errors
    ///
    /// Every [`Refusal`] the reader finds; the shipped files are held to none by `tests/vocabulary.rs`.
    pub fn shipped(sources: &mut SourceMap, kinds: &Registry) -> Result<Self, Vec<Refusal>> {
        Self::read(
            sources,
            (KIND_PATH, KIND_TEXT),
            (VOCABULARY_PATH, VOCABULARY_TEXT),
            kinds,
        )
    }

    /// Read a vocabulary: `kind` is the module declaring `deffact` and `vocabulary` its entries, each a `(name,
    /// text)` pair added to `sources`, so every refusal's span points into them.
    ///
    /// # Errors
    ///
    /// Every [`Refusal`] found, never only the first: a vocabulary with three faults costs one edit cycle.
    pub fn read(
        sources: &mut SourceMap,
        kind: (&str, &str),
        vocabulary: (&str, &str),
        kinds: &Registry,
    ) -> Result<Self, Vec<Refusal>> {
        let entries_kind = shipped_registry(sources, &[(kind.0.to_string(), kind.1.to_string())])
            .map_err(|errors| {
            errors
                .into_iter()
                .map(|d| Refusal::new(Cause::Unreadable, None, Some(d.primary.span), d.message))
                .collect::<Vec<_>>()
        })?;
        let id = sources
            .add(vocabulary.0.to_string(), vocabulary.1.to_string())
            .map_err(|_| {
                vec![Refusal::new(
                    Cause::Unreadable,
                    None,
                    None,
                    format!("{} is too large to address", vocabulary.0),
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
        let mut facts = Vec::new();
        for form in &declarations {
            if form.head() != Some(ENTRY) {
                refusals.push(Refusal::new(
                    Cause::NotAnEntry,
                    None,
                    Some(form.span()),
                    "the vocabulary holds `deffact` entries and nothing else",
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
            match entry(form) {
                Ok(fact) => facts.push(fact),
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
        let words = clause_words(kinds);
        refusals.extend(across(&facts, &words));
        if refusals.is_empty() {
            let index = facts
                .iter()
                .enumerate()
                .map(|(i, fact)| (fact.name.clone(), i))
                .collect();
            Ok(Self {
                facts,
                index,
                clause_words: words,
            })
        } else {
            Err(refusals)
        }
    }

    /// The entry named `name`, if the vocabulary declares one.
    #[must_use]
    pub fn fact(&self, name: &str) -> Option<&Fact> {
        self.index.get(name).map(|&i| &self.facts[i])
    }

    /// Whether `name` is a clause word of the description kinds the vocabulary was read against: a list headed by
    /// one is a clause, never an offer or a constraint (record §1, R30 1).
    #[must_use]
    pub fn is_clause_word(&self, name: &str) -> bool {
        self.clause_words.contains(name)
    }

    /// Every entry, in the order written.
    pub fn facts(&self) -> impl Iterator<Item = &Fact> {
        self.facts.iter()
    }

    /// How many entries it holds.
    #[must_use]
    pub fn len(&self) -> usize {
        self.facts.len()
    }

    /// Whether it holds none.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.facts.is_empty()
    }
}

/// Every clause word of the description kinds: any clause a kind of `kinds` declares (record §1).
#[must_use]
pub fn clause_words(kinds: &Registry) -> BTreeSet<String> {
    kinds
        .heads()
        .into_iter()
        .filter_map(|head| kinds.kind(head))
        .flat_map(|kind| kind.clauses.iter().map(|clause| clause.head.clone()))
        .collect()
}

fn name_of(form: &Form) -> Option<&str> {
    form.items().get(1).and_then(Form::as_symbol)
}

/// The symbols of a clause's rest, refusing anything else and any repetition.
fn symbols(
    form: &Form,
    rest: &[Form],
    fact: &str,
    cause: Cause,
    what: &str,
) -> Result<Vec<String>, Refusal> {
    let mut seen = BTreeSet::new();
    let mut out = Vec::new();
    for item in rest {
        let Some(name) = item.as_symbol() else {
            return Err(Refusal::new(
                cause,
                Some(fact),
                Some(item.span()),
                format!("{what} are names"),
            ));
        };
        if !seen.insert(name) {
            return Err(Refusal::new(
                cause,
                Some(fact),
                Some(item.span()),
                format!("`{name}` written twice among {what}"),
            ));
        }
        out.push(name.to_string());
    }
    if out.is_empty() {
        return Err(Refusal::new(
            cause,
            Some(fact),
            Some(form.span()),
            format!("{what}: none written"),
        ));
    }
    Ok(out)
}

fn dimension(word: &str) -> Option<Dimension> {
    [
        Dimension::Time,
        Dimension::Frequency,
        Dimension::Information,
        Dimension::Dimensionless,
    ]
    .into_iter()
    .find(|d| d.slug() == word)
}

/// A `domain` clause's value (record §1.1's `domain` row, §2).
fn domain(form: &Form, fact: &str) -> Result<Domain, Refusal> {
    let wrong = |span: Span, detail: String| {
        Err(Refusal::new(Cause::Domain, Some(fact), Some(span), detail))
    };
    let rest = &form.items()[1..];
    let Some(word) = rest.first().and_then(Form::as_symbol) else {
        return wrong(
            form.span(),
            "a domain begins with its kind's name".to_string(),
        );
    };
    let args = &rest[1..];
    match word {
        "boolean" | "count" if args.is_empty() => Ok(if word == "boolean" {
            Domain::Boolean
        } else {
            Domain::Count
        }),
        "boolean" | "count" => wrong(form.span(), format!("`{word}` takes nothing after it")),
        "quantity" | "interval" => match args {
            [d] => match d.as_symbol().and_then(dimension) {
                Some(dim) if word == "quantity" => Ok(Domain::Quantity(dim)),
                Some(dim) => Ok(Domain::Interval(dim)),
                None => wrong(
                    d.span(),
                    "a dimension is one of time, frequency, information, dimensionless".to_string(),
                ),
            },
            _ => wrong(form.span(), format!("`{word}` takes one dimension")),
        },
        "enumeration" => match args {
            [ordered @ Form::List { .. }] if ordered.head() == Some("ordered") => {
                symbols(ordered, &ordered.items()[1..], fact, Cause::Domain, "an order's alternatives").map(
                    |alternatives| Domain::Enumeration {
                        alternatives,
                        ordered: true,
                    },
                )
            }
            _ => symbols(form, args, fact, Cause::Domain, "an enumeration's alternatives").map(
                |alternatives| Domain::Enumeration {
                    alternatives,
                    ordered: false,
                },
            ),
        },
        "set" => symbols(form, args, fact, Cause::Domain, "a set's alternatives").map(Domain::Set),
        "group" => symbols(form, args, fact, Cause::Domain, "a group's sub-facts").map(Domain::Group),
        _ => wrong(
            rest[0].span(),
            format!(
                "`{word}` is not a domain: boolean, count, quantity, interval, enumeration, set or group"
            ),
        ),
    }
}

/// The one symbol a `(holds values symbol)` clause holds; the frame has checked there is one.
fn word(form: &Form) -> &str {
    form.items()
        .get(1)
        .and_then(Form::as_symbol)
        .unwrap_or_default()
}

/// One entry whose frame `deffact` admits, read into a [`Fact`].
fn entry(form: &Form) -> Result<Fact, Vec<Refusal>> {
    let name = name_of(form).unwrap_or_default().to_string();
    let mut refusals = Vec::new();
    let (mut doc, mut domain_of, mut role, mut direction) = (String::new(), None, None, None);
    let (mut derived_from, mut reads, mut rule, mut implies) =
        (Vec::new(), Vec::new(), None, Vec::new());
    for clause in &form.items()[2..] {
        let rest = &clause.items()[1..];
        match clause.head() {
            Some("doc") => {
                doc = match rest.first() {
                    Some(Form::Str { value, .. }) => value.clone(),
                    _ => String::new(),
                };
            }
            Some("domain") => match domain(clause, &name) {
                Ok(d) => domain_of = Some(d),
                Err(r) => refusals.push(r),
            },
            Some("role") => match word(clause) {
                "guarantee" => role = Some(Role::Guarantee),
                "statement" => role = Some(Role::Statement),
                other => refusals.push(Refusal::new(
                    Cause::Role,
                    Some(&name),
                    Some(clause.span()),
                    format!("`{other}` is not a role: guarantee or statement"),
                )),
            },
            Some("direction") => match Direction::named(word(clause)) {
                Some(d) => direction = Some(d),
                None => refusals.push(Refusal::new(
                    Cause::Direction,
                    Some(&name),
                    Some(clause.span()),
                    format!(
                        "`{}` is not a direction: at-least, at-most, exact, includes or within",
                        word(clause)
                    ),
                )),
            },
            Some("derived-from") => {
                match symbols(clause, rest, &name, Cause::Rule, "a derivation's inputs") {
                    Ok(inputs) => derived_from = inputs,
                    Err(r) => refusals.push(r),
                }
            }
            Some("reads") => {
                match symbols(clause, rest, &name, Cause::Rule, "a rule's optional inputs") {
                    Ok(inputs) => reads = inputs,
                    Err(r) => refusals.push(r),
                }
            }
            Some("rule") => match Rule::named(word(clause)) {
                Some(r) => rule = Some(r),
                None => refusals.push(Refusal::new(
                    Cause::Rule,
                    Some(&name),
                    Some(clause.span()),
                    format!("`{}` is not a rule the engine has", word(clause)),
                )),
            },
            Some("implies") => {
                match symbols(clause, rest, &name, Cause::Implies, "implied members") {
                    Ok(members) => implies = members,
                    Err(r) => refusals.push(r),
                }
            }
            // The frame admits no other clause.
            _ => {}
        }
    }
    match (domain_of, role, direction) {
        (Some(domain), Some(role), Some(direction)) if refusals.is_empty() => Ok(Fact {
            name,
            doc,
            domain,
            role,
            direction,
            derived_from,
            reads,
            rule,
            implies,
            span: form.span(),
        }),
        _ => {
            // Unreachable while the frame holds `domain`, `role` and `direction` to exactly once; refused rather than
            // dropped should the frame ever stop doing so, since an entry dropped in silence is a fact unjudged.
            if refusals.is_empty() {
                refusals.push(Refusal::new(
                    Cause::Frame,
                    Some(&name),
                    Some(form.span()),
                    "an entry writes its domain, its role and its direction",
                ));
            }
            Err(refusals)
        }
    }
}

/// What each entry must agree with of the others, and of the description kinds.
fn across(facts: &[Fact], words: &BTreeSet<String>) -> Vec<Refusal> {
    let by_name: BTreeMap<&str, &Fact> = facts.iter().map(|f| (f.name.as_str(), f)).collect();
    let mut refusals = Vec::new();
    let mut refuse = |cause: Cause, fact: &Fact, detail: String| {
        refusals.push(Refusal::new(
            cause,
            Some(&fact.name),
            Some(fact.span),
            detail,
        ));
    };
    for fact in facts {
        if words.contains(&fact.name) {
            refuse(
                Cause::ClauseWord,
                fact,
                "a fact named like a clause word: an offer or a requirement on it would read as the clause".to_string(),
            );
        }
        if !fact.domain.admits(fact.direction) {
            refuse(
                Cause::Direction,
                fact,
                format!(
                    "its domain does not admit the direction `{}`",
                    fact.direction.word()
                ),
            );
        }
        let named = fact.derived_from.iter().chain(&fact.reads);
        let members = match &fact.domain {
            Domain::Group(subs) => subs.as_slice(),
            _ => &[],
        };
        for other in named.chain(members) {
            if !by_name.contains_key(other.as_str()) {
                refuse(
                    Cause::UndeclaredFact,
                    fact,
                    format!("`{other}` is not a fact the vocabulary declares"),
                );
            }
        }
        match (fact.rule, fact.derived_from.is_empty()) {
            (None, false) => refuse(
                Cause::Rule,
                fact,
                "a derivation names the rule that computes it".to_string(),
            ),
            (Some(rule), true) => refuse(
                Cause::Rule,
                fact,
                format!(
                    "`{}` is a derivation, and names the facts it is derived from",
                    rule.name()
                ),
            ),
            (None, true) if !fact.reads.is_empty() => refuse(
                Cause::Rule,
                fact,
                "`reads` names a rule's optional inputs, and the entry names no rule".to_string(),
            ),
            (Some(rule), false) => {
                let lookup = |names: &[String]| {
                    names
                        .iter()
                        .filter_map(|n| by_name.get(n.as_str()).copied())
                        .collect::<Vec<_>>()
                };
                let (required, optional) = (lookup(&fact.derived_from), lookup(&fact.reads));
                let all_declared =
                    required.len() == fact.derived_from.len() && optional.len() == fact.reads.len();
                if let Some(why) = all_declared
                    .then(|| rule.mismatch(fact, &required, &optional))
                    .flatten()
                {
                    refuse(Cause::Rule, fact, why);
                }
            }
            (None, true) => {}
        }
        if !fact.implies.is_empty() {
            match &fact.domain {
                Domain::Set(alternatives) => {
                    for member in &fact.implies {
                        if !alternatives.contains(member) {
                            refuse(
                                Cause::Implies,
                                fact,
                                format!("`{member}` is not a member of the set"),
                            );
                        }
                    }
                }
                _ => refuse(
                    Cause::Implies,
                    fact,
                    "`implies` names members of a set, and the fact is not one".to_string(),
                ),
            }
        }
    }
    refusals.extend(cycles(facts));
    refusals
}

/// Every cycle `derived-from` and `reads` close among declared facts, each reported once at the entry where the
/// walk meets it again (record §1.1, §9; R9 I12, R10 J12).
fn cycles(facts: &[Fact]) -> Vec<Refusal> {
    #[derive(Clone, Copy, PartialEq, Eq)]
    enum Mark {
        Unvisited,
        OnPath,
        Done,
    }
    fn visit(
        at: usize,
        facts: &[Fact],
        by_name: &BTreeMap<&str, usize>,
        marks: &mut [Mark],
        path: &mut Vec<usize>,
        out: &mut Vec<Refusal>,
    ) {
        marks[at] = Mark::OnPath;
        path.push(at);
        let fact = &facts[at];
        for next in fact.derived_from.iter().chain(&fact.reads) {
            let Some(&n) = by_name.get(next.as_str()) else {
                continue;
            };
            match marks[n] {
                Mark::Unvisited => visit(n, facts, by_name, marks, path, out),
                Mark::OnPath => {
                    let start = path.iter().position(|&p| p == n).unwrap_or(0);
                    let mut names: Vec<&str> = path[start..]
                        .iter()
                        .map(|&p| facts[p].name.as_str())
                        .collect();
                    names.push(facts[n].name.as_str());
                    out.push(Refusal::new(
                        Cause::Cycle,
                        Some(&fact.name),
                        Some(fact.span),
                        format!(
                            "`derived-from` and `reads` close a cycle: {}",
                            names.join(" → ")
                        ),
                    ));
                }
                Mark::Done => {}
            }
        }
        path.pop();
        marks[at] = Mark::Done;
    }
    let by_name: BTreeMap<&str, usize> = facts
        .iter()
        .enumerate()
        .map(|(i, f)| (f.name.as_str(), i))
        .collect();
    let mut marks = vec![Mark::Unvisited; facts.len()];
    let mut out = Vec::new();
    for start in 0..facts.len() {
        if marks[start] == Mark::Unvisited {
            visit(
                start,
                facts,
                &by_name,
                &mut marks,
                &mut Vec::new(),
                &mut out,
            );
        }
    }
    out
}
