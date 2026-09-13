//! Kinds and schemas: what a declaration may say, and how a new one is declared.
//!
//! `ROADMAP.md` §5.6 gives the `defkind` facility a job and, in the same breath, a prohibition:
//!
//! > The initial `defkind` facility defines feature declarations and well-formedness. **It must
//! > not become a host-code evaluator or an implementation template language.** Its use of the
//! > same syntax as other declarations does not make new functionality executable without
//! > corresponding engine knowledge.
//!
//! And §2's correction table settles where the trust sits:
//!
//! > Surface kinds can share the registry; **a small trusted semantic foundation remains
//! > explicit**. The registry cannot silently introduce new trusted axioms.
//!
//! So this module has exactly one primitive that is *not* declared in eADL: `defkind` itself,
//! whose meaning is this Rust code. Every other kind — `defblock`, `defplatform`, `defservice`,
//! `defpolicy`, `defsystem` — is declared in `docs/semantics/kinds/core.eadl` using `defkind`,
//! and is therefore no more privileged than a kind a user adds tomorrow.
//!
//! That boundary is the point. If a future change needs a second trusted primitive, it has to
//! be written here, in Rust, where it is visible — which is what "cannot silently introduce new
//! trusted axioms" means in practice.
//!
//! # What a schema checks, and what it does not
//!
//! A schema validates the **declaration frame**: is this a known kind, does it carry a name if
//! it needs one, are its clauses known, do they appear the right number of times, and are their
//! values the right shape. It does **not** yet interpret the constraint vocabulary inside a
//! clause — `(at-least 60 s)` is nested forms here, and becomes a checked quantity at `M1.3`.
//! Claiming otherwise would be the more dangerous kind of green.
//!
//! One consequence is worth stating rather than discovering: a clause declared `(holds forms)`
//! is **opaque**, so a forbidden construct nested inside one is invisible to the schema. The
//! boundary classifier catches those, because it walks the whole tree. Measured on the corpus,
//! the schema refuses 10 of the 11 rejected cases and the eleventh — `execution-bound`, which
//! hides `wcet` inside `(task …)` — is caught only by the classifier. The two mechanisms are
//! independent and their reach is not identical; `tests/kinds.rs` pins the split in both
//! directions so a change to it has to be deliberate.

use std::collections::BTreeMap;

use eadl_front::{Diagnostic, Form, Label};

use crate::boundary;

/// How many times a clause may appear in one declaration.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cardinality {
    /// Exactly once.
    One,
    /// Zero or one time.
    AtMostOne,
    /// At least once.
    OneOrMore,
    /// Any number of times, including none.
    Any,
}

impl Cardinality {
    /// Parse the surface spelling.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "one" => Self::One,
            "at-most-one" => Self::AtMostOne,
            "one-or-more" => Self::OneOrMore,
            "any" => Self::Any,
            _ => return None,
        })
    }

    /// The surface spelling.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::One => "one",
            Self::AtMostOne => "at-most-one",
            Self::OneOrMore => "one-or-more",
            Self::Any => "any",
        }
    }

    /// Every spelling, for a diagnostic that lists the alternatives.
    pub const ALL: &'static [Self] = &[Self::One, Self::AtMostOne, Self::OneOrMore, Self::Any];

    /// Whether `count` occurrences satisfy this cardinality.
    #[must_use]
    pub const fn admits(self, count: usize) -> bool {
        match self {
            Self::One => count == 1,
            Self::AtMostOne => count <= 1,
            Self::OneOrMore => count >= 1,
            Self::Any => true,
        }
    }

    /// How to describe the requirement in a diagnostic.
    #[must_use]
    pub const fn expectation(self) -> &'static str {
        match self {
            Self::One => "exactly once",
            Self::AtMostOne => "at most once",
            Self::OneOrMore => "at least once",
            Self::Any => "any number of times",
        }
    }
}

/// What a clause is allowed to contain.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Holds {
    /// Nested forms, not interpreted at this layer.
    ///
    /// ⚠️ Opaque: a forbidden construct nested inside such a clause is invisible to the schema.
    /// That is why [`Holds::Kind`] exists — every clause moved from `forms` to `kind` extends
    /// the schema's reach by exactly what that clause contains.
    Forms,
    /// A fixed sequence of scalar values.
    Values(Vec<ValueType>),
    /// Each occurrence is itself a declaration of the named kind, validated recursively.
    Kind(String),
}

/// A scalar value type.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ValueType {
    /// A bare atom.
    Symbol,
    /// An exact integer.
    Integer,
    /// An exact decimal.
    Decimal,
    /// An integer or a decimal.
    Number,
    /// A quoted string.
    Str,
    /// Anything scalar.
    Any,
}

impl ValueType {
    /// Parse the surface spelling.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        Some(match text {
            "symbol" => Self::Symbol,
            "integer" => Self::Integer,
            "decimal" => Self::Decimal,
            "number" => Self::Number,
            "string" => Self::Str,
            "any" => Self::Any,
            _ => return None,
        })
    }

    /// The surface spelling.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Symbol => "symbol",
            Self::Integer => "integer",
            Self::Decimal => "decimal",
            Self::Number => "number",
            Self::Str => "string",
            Self::Any => "any",
        }
    }

    /// Whether a form is this type.
    #[must_use]
    pub fn admits(self, form: &Form) -> bool {
        match (self, form) {
            (Self::Any, Form::List { .. }) => false,
            (Self::Any, _) => true,
            (Self::Symbol, Form::Symbol { .. })
            | (Self::Integer, Form::Integer { .. })
            | (Self::Decimal, Form::Decimal { .. })
            | (Self::Str, Form::Str { .. })
            | (Self::Number, Form::Integer { .. } | Form::Decimal { .. }) => true,
            _ => false,
        }
    }
}

/// Whether a declaration carries a name after its head.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NameRule {
    /// `(defservice time.monotonic …)` — the name is required.
    Required,
    /// The declaration has no name.
    Forbidden,
}

/// One clause a kind admits.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ClauseDef {
    /// The clause's list head.
    pub head: String,
    /// How often it may appear.
    pub cardinality: Cardinality,
    /// What it may contain.
    pub holds: Holds,
}

/// A declaration kind.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct KindDef {
    /// The declaration's list head, e.g. `defservice`.
    pub head: String,
    /// One line saying what it is for. Required: a kind nobody can explain is a kind nobody
    /// should be adding.
    pub doc: String,
    /// Whether it carries a name.
    pub name: NameRule,
    /// The clauses it admits, in declaration order.
    pub clauses: Vec<ClauseDef>,
}

impl KindDef {
    /// Look up a clause by head.
    #[must_use]
    pub fn clause(&self, head: &str) -> Option<&ClauseDef> {
        self.clauses.iter().find(|clause| clause.head == head)
    }
}

/// The kinds a description may use.
#[derive(Debug, Clone, Default)]
pub struct Registry {
    kinds: BTreeMap<String, KindDef>,
}

impl Registry {
    /// An empty registry. `defkind` is always understood; it is this module, not an entry.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Register a kind.
    ///
    /// # Errors
    ///
    /// Refuses a duplicate head. Redefining a kind silently is how a locked description quietly
    /// changes meaning, which §15 forbids outright.
    pub fn register(&mut self, kind: KindDef) -> Result<(), Box<Diagnostic>> {
        if self.kinds.contains_key(&kind.head) {
            return Err(Box::new(Diagnostic::error(
                "schema-duplicate-kind",
                format!("kind `{}` is already defined", kind.head),
                Label::new(
                    eadl_front::Span::new(eadl_front::SourceId(0), 0, 0),
                    "second definition",
                ),
                "remove one definition; redefining a kind would silently change what already-written descriptions mean",
            )));
        }
        self.kinds.insert(kind.head.clone(), kind);
        Ok(())
    }

    /// The kind behind a head.
    #[must_use]
    pub fn kind(&self, head: &str) -> Option<&KindDef> {
        self.kinds.get(head)
    }

    /// Every registered head, sorted.
    #[must_use]
    pub fn heads(&self) -> Vec<&str> {
        self.kinds.keys().map(String::as_str).collect()
    }

    /// How many kinds are registered.
    #[must_use]
    pub fn len(&self) -> usize {
        self.kinds.len()
    }

    /// Whether nothing is registered.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.kinds.is_empty()
    }
}

/// The name of the one trusted primitive. Its meaning is this module's code, not a declaration.
pub const DEFKIND: &str = "defkind";

/// Read a `(defkind …)` form into a [`KindDef`].
///
/// # Errors
///
/// Returns every problem found, rather than the first: a malformed kind definition usually has
/// more than one thing wrong with it, and reporting them one per edit cycle is a poor trade for
/// the small amount of code it saves.
pub fn read_kind(form: &Form) -> Result<KindDef, Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let items = form.items();

    if form.head() != Some(DEFKIND) {
        return Err(vec![Diagnostic::error(
            "schema-not-a-kind",
            format!(
                "expected a `{DEFKIND}` declaration, found `{}`",
                form.head().unwrap_or(form.kind())
            ),
            Label::new(form.span(), "not a kind definition"),
            format!("kind definitions are written `({DEFKIND} <head> (doc \"…\") (name …) (clause …) …)`"),
        )]);
    }

    // ⛔ §5.6: a kind defines *well-formedness*, never behavior. The boundary classifier runs
    // over the definition itself, so `(defkind x (implementation …))` is refused by the same
    // machine that refuses it in an ordinary declaration — there is no back door through the
    // facility that declares the language.
    if let Err(diagnostic) = boundary::check(form) {
        errors.push(*diagnostic);
    }

    let head = match items.get(1).and_then(Form::as_symbol) {
        Some(name) => name.to_string(),
        None => {
            errors.push(Diagnostic::error(
                "schema-missing-kind-head",
                "a kind definition must name the declaration head it defines",
                Label::new(form.span(), "no head given"),
                format!("write `({DEFKIND} <head> …)`, e.g. `({DEFKIND} defservice …)`"),
            ));
            String::new()
        }
    };

    let mut doc = None;
    let mut name_rule = None;
    let mut clauses: Vec<ClauseDef> = Vec::new();

    for item in items.iter().skip(2) {
        match item.head() {
            Some("doc") => match item.items().get(1) {
                Some(Form::Str { value, .. }) => doc = Some(value.clone()),
                _ => errors.push(Diagnostic::error(
                    "schema-bad-doc",
                    "`doc` takes one quoted string",
                    Label::new(item.span(), "expected a string"),
                    "write `(doc \"what this kind is for\")`",
                )),
            },
            Some("name") => match item.items().get(1).and_then(Form::as_symbol) {
                Some("required") => name_rule = Some(NameRule::Required),
                Some("forbidden") => name_rule = Some(NameRule::Forbidden),
                _ => errors.push(Diagnostic::error(
                    "schema-bad-name-rule",
                    "`name` takes `required` or `forbidden`",
                    Label::new(item.span(), "expected `required` or `forbidden`"),
                    "write `(name required)` when the declaration is written `(<head> <name> …)`",
                )),
            },
            Some("clause") => match read_clause(item) {
                Ok(clause) => {
                    if clauses.iter().any(|existing| existing.head == clause.head) {
                        errors.push(Diagnostic::error(
                            "schema-duplicate-clause",
                            format!("clause `{}` is declared twice in this kind", clause.head),
                            Label::new(item.span(), "second declaration"),
                            "declare each clause once; use `(cardinality any)` to allow repetition in a description",
                        ));
                    } else {
                        clauses.push(clause);
                    }
                }
                Err(mut found) => errors.append(&mut found),
            },
            _ => errors.push(Diagnostic::error(
                "schema-unknown-kind-field",
                format!(
                    "`{}` is not part of a kind definition",
                    item.head().unwrap_or(item.kind())
                ),
                Label::new(item.span(), "unknown here"),
                "a kind definition holds `doc`, `name` and `clause` — and nothing else, because it defines well-formedness rather than behavior",
            )),
        }
    }

    let Some(doc) = doc else {
        errors.push(Diagnostic::error(
            "schema-missing-doc",
            "a kind definition must say what the kind is for",
            Label::new(form.span(), "no `doc` clause"),
            "write `(doc \"one line saying what this kind describes\")` — a kind nobody can explain is a kind nobody should be adding",
        ));
        return Err(errors);
    };

    if errors.is_empty() {
        Ok(KindDef {
            head,
            doc,
            name: name_rule.unwrap_or(NameRule::Required),
            clauses,
        })
    } else {
        Err(errors)
    }
}

fn read_clause(form: &Form) -> Result<ClauseDef, Vec<Diagnostic>> {
    let mut errors = Vec::new();
    let items = form.items();

    let head = match items.get(1).and_then(Form::as_symbol) {
        Some(name) => name.to_string(),
        None => {
            return Err(vec![Diagnostic::error(
                "schema-missing-clause-head",
                "a clause declaration must name the clause",
                Label::new(form.span(), "no clause name"),
                "write `(clause <name> (cardinality …) (holds …))`",
            )]);
        }
    };

    let mut cardinality = None;
    let mut holds = None;

    for item in items.iter().skip(2) {
        match item.head() {
            Some("cardinality") => {
                match item
                    .items()
                    .get(1)
                    .and_then(Form::as_symbol)
                    .and_then(Cardinality::parse)
                {
                    Some(value) => cardinality = Some(value),
                    None => errors.push(Diagnostic::error(
                        "schema-bad-cardinality",
                        "unknown cardinality",
                        Label::new(item.span(), "not a cardinality"),
                        format!(
                            "the cardinalities are {}",
                            Cardinality::ALL
                                .iter()
                                .map(|c| format!("`{}`", c.slug()))
                                .collect::<Vec<_>>()
                                .join(", ")
                        ),
                    )),
                }
            }
            Some("holds") => match read_holds(item) {
                Ok(value) => holds = Some(value),
                Err(diagnostic) => errors.push(*diagnostic),
            },
            _ => errors.push(Diagnostic::error(
                "schema-unknown-clause-field",
                format!(
                    "`{}` is not part of a clause declaration",
                    item.head().unwrap_or(item.kind())
                ),
                Label::new(item.span(), "unknown here"),
                "a clause declaration holds `cardinality` and `holds`",
            )),
        }
    }

    if errors.is_empty() {
        Ok(ClauseDef {
            head,
            cardinality: cardinality.unwrap_or(Cardinality::Any),
            holds: holds.unwrap_or(Holds::Forms),
        })
    } else {
        Err(errors)
    }
}

fn read_holds(form: &Form) -> Result<Holds, Box<Diagnostic>> {
    let items = form.items();
    match items.get(1).and_then(Form::as_symbol) {
        Some("forms") => Ok(Holds::Forms),
        Some("kind") => match items.get(2).and_then(Form::as_symbol) {
            Some(name) => Ok(Holds::Kind(name.to_string())),
            None => Err(Box::new(Diagnostic::error(
                "schema-bad-holds",
                "`holds kind` must name the kind",
                Label::new(form.span(), "no kind named"),
                "write `(holds kind task)` — each occurrence of the clause is then validated \
                 as a declaration of that kind",
            ))),
        },
        Some("values") => {
            let mut types = Vec::new();
            for item in items.iter().skip(2) {
                match item.as_symbol().and_then(ValueType::parse) {
                    Some(value) => types.push(value),
                    None => {
                        return Err(Box::new(Diagnostic::error(
                            "schema-bad-value-type",
                            "unknown value type",
                            Label::new(item.span(), "not a value type"),
                            "the value types are `symbol`, `integer`, `decimal`, `number`, `string`, `any`",
                        )));
                    }
                }
            }
            Ok(Holds::Values(types))
        }
        _ => Err(Box::new(Diagnostic::error(
            "schema-bad-holds",
            "`holds` takes `forms`, `values <type>…`, or `kind <name>`",
            Label::new(form.span(), "expected `forms`, `values` or `kind`"),
            "write `(holds forms)` for opaque nested content, `(holds values number symbol)` \
             for a quantity, or `(holds kind task)` to validate each occurrence as a \
             declaration of that kind",
        ))),
    }
}

/// Validate one declaration against the registry.
///
/// Returns every problem found. An empty result means the declaration's **frame** is
/// well-formed — see this module's header for what that does and does not cover.
#[must_use]
pub fn validate(registry: &Registry, form: &Form) -> Vec<Diagnostic> {
    let mut errors = Vec::new();

    let Some(head) = form.head() else {
        return vec![Diagnostic::error(
            "schema-not-a-declaration",
            "a top-level form must be a declaration",
            Label::new(form.span(), "expected `(<kind> …)`"),
            "declarations are written `(defservice time.monotonic …)`",
        )];
    };

    let Some(kind) = registry.kind(head) else {
        return vec![unknown_kind(registry, form, head)];
    };

    let items = form.items();
    let mut clause_start = 1;

    match kind.name {
        NameRule::Required => match items.get(1) {
            Some(Form::Symbol { .. }) => clause_start = 2,
            _ => errors.push(Diagnostic::error(
                "schema-missing-name",
                format!("`{head}` declarations carry a name"),
                Label::new(form.span(), "no name after the kind"),
                format!("write `({head} <name> …)` — {}", kind.doc),
            )),
        },
        NameRule::Forbidden => {}
    }

    let mut counts: BTreeMap<&str, usize> = BTreeMap::new();

    for item in items.iter().skip(clause_start) {
        let Some(clause_head) = item.head() else {
            errors.push(Diagnostic::error(
                "schema-not-a-clause",
                format!(
                    "a `{head}` declaration holds clauses, not a bare {}",
                    item.kind()
                ),
                Label::new(item.span(), "expected `(<clause> …)`"),
                format!("the clauses of `{head}` are {}", clause_list(kind)),
            ));
            continue;
        };

        let Some(clause) = kind.clause(clause_head) else {
            errors.push(unknown_clause(kind, item, clause_head));
            continue;
        };

        *counts.entry(clause.head.as_str()).or_default() += 1;

        match &clause.holds {
            Holds::Values(types) => check_values(item, types, &mut errors),
            // ⭐ Recursion is what closes the schema's reach. A clause declared `(holds kind
            // task)` is validated as a full declaration, so a forbidden construct nested inside
            // it — an execution bound inside a task, say — is seen here rather than only by the
            // boundary classifier walking the whole tree.
            Holds::Kind(name) => match registry.kind(name) {
                Some(_) => errors.extend(validate(registry, item)),
                None => errors.push(Diagnostic::error(
                    "schema-unknown-referenced-kind",
                    format!(
                        "clause `{}` of `{head}` is declared to hold kind `{name}`, which is not registered",
                        clause.head
                    ),
                    Label::new(item.span(), "cannot be validated"),
                    format!(
                        "register the module that defines `{name}` — the workload kinds live in \
                         `docs/semantics/kinds/os-rt.eadl`"
                    ),
                )),
            },
            Holds::Forms => {}
        }
    }

    for clause in &kind.clauses {
        let count = counts.get(clause.head.as_str()).copied().unwrap_or(0);
        if !clause.cardinality.admits(count) {
            errors.push(Diagnostic::error(
                "schema-cardinality",
                format!(
                    "`{head}` requires the `{}` clause {}, found {count}",
                    clause.head,
                    clause.cardinality.expectation()
                ),
                Label::new(form.span(), "in this declaration"),
                if count == 0 {
                    format!("add a `({} …)` clause", clause.head)
                } else {
                    format!(
                        "keep {} `({} …)` clause(s)",
                        clause.cardinality.expectation(),
                        clause.head
                    )
                },
            ));
        }
    }

    errors
}

fn check_values(clause: &Form, types: &[ValueType], errors: &mut Vec<Diagnostic>) {
    let values: Vec<&Form> = clause.items().iter().skip(1).collect();
    if values.len() != types.len() {
        errors.push(Diagnostic::error(
            "schema-arity",
            format!(
                "`{}` takes {} value(s), found {}",
                clause.head().unwrap_or("clause"),
                types.len(),
                values.len()
            ),
            Label::new(clause.span(), "wrong number of values"),
            format!(
                "write `({} {})`",
                clause.head().unwrap_or("clause"),
                types
                    .iter()
                    .map(|t| format!("<{}>", t.slug()))
                    .collect::<Vec<_>>()
                    .join(" ")
            ),
        ));
        return;
    }
    for (value, expected) in values.iter().zip(types.iter()) {
        if !expected.admits(value) {
            errors.push(Diagnostic::error(
                "schema-type",
                format!("expected a {}, found a {}", expected.slug(), value.kind()),
                Label::new(value.span(), format!("not a {}", expected.slug())),
                format!("write a {} here", expected.slug()),
            ));
        }
    }
}

fn clause_list(kind: &KindDef) -> String {
    if kind.clauses.is_empty() {
        return "none".to_string();
    }
    kind.clauses
        .iter()
        .map(|clause| format!("`{}`", clause.head))
        .collect::<Vec<_>>()
        .join(", ")
}

fn unknown_kind(registry: &Registry, form: &Form, head: &str) -> Diagnostic {
    if let Some(construct) = boundary::forbidden(head) {
        // Prefer the boundary's wording: "`implementation` is not a known kind" is true and
        // useless, while the boundary refusal says what the construct is and where it belongs.
        return boundary::check(form).map_or_else(
            |diagnostic| *diagnostic,
            |()| {
                Diagnostic::error(
                    "schema-unknown-kind",
                    format!("`{head}` is not a known kind"),
                    Label::new(form.span(), "unknown kind"),
                    construct.belongs.to_string(),
                )
            },
        );
    }
    let known = registry.heads();
    let suggestion = closest(head, &known);
    Diagnostic::error(
        "schema-unknown-kind",
        format!("`{head}` is not a known kind"),
        Label::new(form.span(), "unknown kind"),
        match suggestion {
            Some(near) => format!(
                "did you mean `{near}`? the known kinds are {}",
                join(&known)
            ),
            None => format!("the known kinds are {}", join(&known)),
        },
    )
}

fn unknown_clause(kind: &KindDef, item: &Form, clause_head: &str) -> Diagnostic {
    if boundary::forbidden(clause_head).is_some() {
        if let Err(diagnostic) = boundary::check(item) {
            return *diagnostic;
        }
    }
    let known: Vec<&str> = kind.clauses.iter().map(|c| c.head.as_str()).collect();
    let suggestion = closest(clause_head, &known);
    Diagnostic::error(
        "schema-unknown-clause",
        format!(
            "`{}` declarations have no `{clause_head}` clause",
            kind.head
        ),
        Label::new(item.span(), "unknown clause"),
        match suggestion {
            Some(near) => format!(
                "did you mean `{near}`? the clauses of `{}` are {}",
                kind.head,
                join(&known)
            ),
            None => format!("the clauses of `{}` are {}", kind.head, join(&known)),
        },
    )
}

fn join(items: &[&str]) -> String {
    if items.is_empty() {
        return "none".to_string();
    }
    items
        .iter()
        .map(|item| format!("`{item}`"))
        .collect::<Vec<_>>()
        .join(", ")
}

/// The closest known name within a small edit distance, for a "did you mean" hint.
///
/// Bounded deliberately: suggesting `defsystem` for `implementation` would be worse than
/// suggesting nothing, because it sends the author to rename rather than to reconsider.
fn closest<'a>(given: &str, known: &[&'a str]) -> Option<&'a str> {
    let limit = (given.len() / 3).clamp(1, 3);
    known
        .iter()
        .map(|candidate| (edit_distance(given, candidate), *candidate))
        .filter(|(distance, _)| *distance <= limit)
        .min_by_key(|(distance, candidate)| (*distance, candidate.len()))
        .map(|(_, candidate)| candidate)
}

/// Levenshtein distance, two rows at a time.
fn edit_distance(a: &str, b: &str) -> usize {
    let a: Vec<char> = a.chars().collect();
    let b: Vec<char> = b.chars().collect();
    if a.is_empty() {
        return b.len();
    }
    let mut previous: Vec<usize> = (0..=b.len()).collect();
    let mut current = vec![0; b.len() + 1];
    for (i, ca) in a.iter().enumerate() {
        current[0] = i + 1;
        for (j, cb) in b.iter().enumerate() {
            let substitution = previous[j] + usize::from(ca != cb);
            current[j + 1] = substitution.min(previous[j + 1] + 1).min(current[j] + 1);
        }
        std::mem::swap(&mut previous, &mut current);
    }
    previous[b.len()]
}
