//! Every refusal the relation makes reports as a diagnostic with the record's code, a message naming what is wrong,
//! a label where it is written, and a repair direction (`docs/semantics/model.md` §7 rule 6, §5.5; leaf `M3.1.2.6`).

use eadl_front::{SourceId, Span};
use eadl_resolve::refusal::{Cause, Code, Refusal};

/// Every cause, held exhaustive by the `match`: a cause added without a line here fails to compile.
fn every_cause() -> Vec<Cause> {
    use Cause::*;
    let all = vec![
        DeclarationNamedLikeFact,
        ItemNamesNothing,
        ClauseInOffer,
        ListInAbsent,
        StatementOffered,
        StatementAbsent,
        OutsideDomain,
        EmptyValue,
        ReversedInterval,
        NotWholeBits,
        ZeroModulus,
        DirectionNameAsWrapper,
        BoundOnBoolean,
        DirectionAgainstFact,
        OfferedAndAbsent,
        TwoValues,
        BoundBesideValue,
        DerivedBesideInput,
        DerivedAbsentBesideInput,
        ModulusAboveWidth,
        ModulusBesideSaturating,
        NotAConstraint,
        BareNameInRequires,
        NoConstraintWritten,
        ClauseInRequires,
        NotPlatformItem,
        OperandNamesNothing,
        ListOperand,
        UsesNamesFact,
        NeedsStatement,
        Contradiction,
        UndeclaredFact,
        PastTheArithmetic,
    ];
    for cause in &all {
        match cause {
            DeclarationNamedLikeFact
            | ItemNamesNothing
            | ClauseInOffer
            | ListInAbsent
            | StatementOffered
            | StatementAbsent
            | OutsideDomain
            | EmptyValue
            | ReversedInterval
            | NotWholeBits
            | ZeroModulus
            | DirectionNameAsWrapper
            | BoundOnBoolean
            | DirectionAgainstFact
            | OfferedAndAbsent
            | TwoValues
            | BoundBesideValue
            | DerivedBesideInput
            | DerivedAbsentBesideInput
            | ModulusAboveWidth
            | ModulusBesideSaturating
            | NotAConstraint
            | BareNameInRequires
            | NoConstraintWritten
            | ClauseInRequires
            | NotPlatformItem
            | OperandNamesNothing
            | ListOperand
            | UsesNamesFact
            | NeedsStatement
            | Contradiction
            | UndeclaredFact
            | PastTheArithmetic => {}
        }
    }
    all
}

#[test]
fn every_cause_reports_its_code_its_fact_its_place_and_a_repair() {
    let span = Span {
        source: SourceId(0),
        start: 3,
        end: 9,
    };
    for cause in every_cause() {
        let d = Refusal::new(cause, Some("tick-unit"), span, "what is wrong").diagnostic();
        let want = match cause {
            Cause::UndeclaredFact | Cause::PastTheArithmetic => Code::UnsupportedProfile,
            _ => Code::InvalidDescription,
        };
        assert_eq!(d.code, want.slug(), "{cause:?}");
        assert_eq!(d.message, "`tick-unit`: what is wrong", "{cause:?}");
        assert_eq!(d.primary.span, span, "{cause:?}");
        assert!(
            d.repair.len() > 10,
            "{cause:?}: §5.5 asks every diagnostic for a repair direction"
        );
    }
    let unnamed =
        Refusal::new(Cause::ItemNamesNothing, None, span, "an item names nothing").diagnostic();
    assert_eq!(unnamed.message, "an item names nothing");
}
