//! What the relation refuses of what a description writes, and why
//! (`docs/decisions/decision_substitutability-relation.md` §8; leaf `M3.1.2`).
//!
//! ⭐ **Typed causes, one per rule.** Each cause names one rule of the record's §8, so a test can say which rule
//! refused an input, not only that something did. Which code each cause is reported under, `invalid-description`,
//! `unsupported-profile` or a code of its own, is `docs/semantics/model.md`'s to decide when it becomes normative
//! over this crate (record §8, `SR-H5`). It decides the record's two (`docs/semantics/model.md` §7): every cause is
//! reported under `invalid-description` or `unsupported-profile`, named in the diagnostic's message, and kept typed
//! here for any consumer that must tell one rule from another (leaf `M3.1.2.6`).

use eadl_front::{Diagnostic, Label, Span};

/// The record's two codes for the relation's refusals (§8).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Code {
    /// Malformed, contradictory or ill-typed: the description says something it cannot mean.
    InvalidDescription,
    /// Past what the implemented semantics decides: an undeclared fact constrained, or the exact arithmetic's limit.
    UnsupportedProfile,
}

impl Code {
    /// The code as the toolchain prints it.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::InvalidDescription => "invalid-description",
            Self::UnsupportedProfile => "unsupported-profile",
        }
    }
}

/// One rule of §8 that a written offer, absence, requirement or declaration breaks.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cause {
    /// A declaration whose local name is a vocabulary fact, a provider's own included (§1.1, R16 1, R17 R5).
    DeclarationNamedLikeFact,
    /// An item of `offers` or `absent` that names nothing: a number, a string, `()`, a list headed by none (R23 3).
    ItemNamesNothing,
    /// An item of `offers` that is or holds a clause, read by nothing (§1, R27 2, R28 3).
    ClauseInOffer,
    /// A list inside `absent`, whose rest would be dropped and the whole fact read absent (§8, R18 2, R26 1).
    ListInAbsent,
    /// An offer of a statement fact (§3 rule 6).
    StatementOffered,
    /// A statement fact declared absent (§3 rule 6, R17 R4).
    StatementAbsent,
    /// A value outside its fact's domain (§2).
    OutsideDomain,
    /// A set written with no member where a value is required, or `(f (exactly))` (§2, R13 M11).
    EmptyValue,
    /// An interval with `lo > hi` (§2, R12 L12).
    ReversedInterval,
    /// A value of information that is not a positive whole number of bits (§4, R17 6).
    NotWholeBits,
    /// A `counter-modulus` of 0 (§4, R17 6).
    ZeroModulus,
    /// A list headed by a direction's name, `includes`, `within` or `exact`: a requirement writes its value and the
    /// direction is the fact's (§8, R15 16, R16 4).
    DirectionNameAsWrapper,
    /// A boolean or a group's head offered with a bound other than `exactly` (§2, R9 I7, R10 J4).
    BoundOnBoolean,
    /// A bound written in a direction other than the fact's, `exactly` apart (§1.1).
    DirectionAgainstFact,
    /// One provider offering and declaring absent one fact (model §2 rule 1).
    OfferedAndAbsent,
    /// One provider offering one declared fact with two values (§5).
    TwoValues,
    /// A bound beside a value of the same fact (§2, §5, R10 J8).
    BoundBesideValue,
    /// A derived fact offered beside a fact its rule reads (§4).
    DerivedBesideInput,
    /// A derived fact declared absent beside a fact its rule reads (§4, R16 8).
    DerivedAbsentBesideInput,
    /// A `counter-modulus` above a valued `2^width` (§4, R3 C12).
    ModulusAboveWidth,
    /// A `counter-modulus` beside `(wrap-behavior saturating)` (§4, R1 A14).
    ModulusBesideSaturating,
    /// An item of `requires` that is not a list headed by a name: a number, a string, `()`, a list headed by none
    /// (§1, R22 1).
    NotAConstraint,
    /// A bare name inside `requires`, fact or not: it states no constraint, and presence is `(needs f)` (§1, R21 3).
    BareNameInRequires,
    /// `(f)` inside `requires`: no constraint written (§1, R14 6).
    NoConstraintWritten,
    /// A clause but `needs`, `uses` and `requires` written inside `requires`, at any depth, where a constraint
    /// stands (§1, R29 1).
    ClauseInRequires,
    /// An item of a system's `platform` clause but `needs`, `uses` and `requires` (§1, R29 1).
    NotPlatformItem,
    /// An operand of `needs` or `uses` that names nothing (§1, R23 3).
    OperandNamesNothing,
    /// A list inside `needs` or `uses`, whatever its head: what follows it would be dropped unjudged (§1, R14 7,
    /// R26 1).
    ListOperand,
    /// A `uses` naming a vocabulary fact: a fact is needed, never used (§1, R15 1).
    UsesNamesFact,
    /// A `needs` of a statement fact (§3 rule 6).
    NeedsStatement,
    /// One side's constraints on one fact that no value satisfies together (§3 rule 5, ROADMAP.md §5.3).
    Contradiction,
    /// A constraint on a fact the vocabulary does not declare, `(x)` or `(x v)` (§1.1, §8; R19 7).
    UndeclaredFact,
    /// What a side requires cannot be decided within the exact arithmetic: an interval whose endpoints cannot be
    /// ordered, or constraints neither proven contradictory nor proven to hold together (§2, §3 rule 5; R18 7, R27 1).
    PastTheArithmetic,
}

impl Cause {
    /// The record's code for this cause (§8).
    #[must_use]
    pub const fn code(self) -> Code {
        match self {
            Self::UndeclaredFact | Self::PastTheArithmetic => Code::UnsupportedProfile,
            _ => Code::InvalidDescription,
        }
    }

    /// What the author does about it: the repair direction §5.5 requires of every diagnostic.
    #[must_use]
    pub const fn repair(self) -> &'static str {
        match self {
            Self::DeclarationNamedLikeFact => {
                "rename the declaration: a fact's name belongs to the vocabulary, and a declaration called by it \
                 captures every requirement on the fact"
            }
            Self::ItemNamesNothing => "write a fact's name, or a list headed by one",
            Self::ClauseInOffer => "move the clause out of `offers`: an offer is a fact and its value",
            Self::ListInAbsent => "name the fact alone inside `absent`; a value belongs in `offers` or `requires`",
            Self::StatementOffered | Self::StatementAbsent => {
                "remove it: the requiring side writes this fact about itself, in `requires`"
            }
            Self::OutsideDomain => {
                "write a value of the fact's domain, which docs/semantics/vocabulary/vocabulary.eadl states"
            }
            Self::EmptyValue => "write the value: at least one member, or the bound's amount",
            Self::ReversedInterval => "write the lower endpoint first: `(range lo hi)` with `lo` at most `hi`",
            Self::NotWholeBits => "write a positive whole number of bits",
            Self::ZeroModulus => "write the count at which the counter wraps, which is positive",
            Self::DirectionNameAsWrapper => {
                "write the value alone, or under `at-least`, `at-most` or `exactly`: the direction is the fact's"
            }
            Self::BoundOnBoolean => "write `true` or `false`, alone or under `exactly`",
            Self::DirectionAgainstFact => "write the bound in the fact's own direction, or under `exactly`",
            Self::OfferedAndAbsent | Self::Contradiction => {
                "remove one of the two: only the author knows which was meant"
            }
            Self::TwoValues => "offer the fact once: a provider with two values is two providers",
            Self::BoundBesideValue => "keep the value or the bound, not both",
            Self::DerivedBesideInput | Self::DerivedAbsentBesideInput => {
                "remove the derived fact: the engine computes it from the inputs this provider offers"
            }
            Self::ModulusAboveWidth => "offer a modulus the register's width holds, or the width that holds it",
            Self::ModulusBesideSaturating => {
                "remove one: a saturating counter wraps at no modulus, and a modulus is where a counter wraps"
            }
            Self::NotAConstraint => "write a constraint: a list headed by a fact's name, such as `(tick-unit ns)`",
            Self::BareNameInRequires | Self::NoConstraintWritten => {
                "write a value or a bound after the fact, or `(needs f)` for presence"
            }
            Self::ClauseInRequires => {
                "move the clause out of `requires`, which holds constraints, `needs`, `uses` and `requires`"
            }
            Self::NotPlatformItem => "write `needs`, `uses` or `requires` inside `platform`, and nothing else",
            Self::OperandNamesNothing => "write a name",
            Self::ListOperand => "name the fact or the declaration alone; a value or a constraint belongs in `requires`",
            Self::UsesNamesFact => "write `(needs f)`: a fact is needed, never used",
            Self::NeedsStatement => "write the statement's value in `requires`",
            Self::UndeclaredFact => {
                "constrain a fact the vocabulary declares (docs/semantics/vocabulary/vocabulary.eadl); a new fact \
                 enters it with a migration note first"
            }
            Self::PastTheArithmetic => {
                "write the amounts in one unit, or in units whose conversion stays within the exact arithmetic"
            }
        }
    }
}

/// One refusal: the rule, the fact it is about, where it is written, and what is wrong in a sentence.
#[derive(Debug, Clone, PartialEq)]
pub struct Refusal {
    /// Which rule refuses it.
    pub cause: Cause,
    /// The fact it is about, when it is about one.
    pub fact: Option<String>,
    /// Where it is written.
    pub span: Span,
    /// What is wrong.
    pub detail: String,
}

impl Refusal {
    /// A refusal of `cause` at `span`.
    #[must_use]
    pub fn new(cause: Cause, fact: Option<&str>, span: Span, detail: impl Into<String>) -> Self {
        Self {
            cause,
            fact: fact.map(str::to_string),
            span,
            detail: detail.into(),
        }
    }

    /// The code it is reported under.
    #[must_use]
    pub const fn code(&self) -> Code {
        self.cause.code()
    }

    /// The refusal as the toolchain reports a diagnostic: its code, a message naming the fact and what is wrong, a
    /// label where it is written, and the cause's repair direction (`docs/semantics/model.md` §7).
    #[must_use]
    pub fn diagnostic(&self) -> Diagnostic {
        let message = match &self.fact {
            Some(fact) => format!("`{fact}`: {}", self.detail),
            None => self.detail.clone(),
        };
        let label = Label::new(self.span, "written here");
        match self.code() {
            Code::InvalidDescription => {
                Diagnostic::error("invalid-description", message, label, self.cause.repair())
            }
            Code::UnsupportedProfile => {
                Diagnostic::error("unsupported-profile", message, label, self.cause.repair())
            }
        }
    }
}
