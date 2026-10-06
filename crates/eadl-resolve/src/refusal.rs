//! What the relation refuses of what a description writes, and why
//! (`docs/decisions/decision_substitutability-relation.md` §8; leaf `M3.1.2`).
//!
//! ⭐ **Typed causes, one per rule.** Each cause names one rule of the record's §8, so a test can say which rule
//! refused an input, not only that something did. Which code each cause is reported under, `invalid-description`,
//! `unsupported-profile` or a code of its own, is `docs/semantics/model.md`'s to decide when it becomes normative
//! over this crate (record §8, `SR-H5`, leaf `M3.1.2.6`); until then [`Cause::code`] gives the record's two.

use eadl_front::Span;

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
}

impl Cause {
    /// The record's code for this cause (§8).
    #[must_use]
    pub const fn code(self) -> Code {
        Code::InvalidDescription
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
}
