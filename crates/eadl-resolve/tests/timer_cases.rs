//! Every row of `docs/decisions/decision_substitutability-relation.md` §6, the timer cases of `ROADMAP.md` §5.2, a
//! test of the production relation (`SR-H1`, leaf `M3.1.2.5`); and §5's enumeration, a provider's answer listed with
//! its outcome, the value found, the derivation used and the direction.

use eadl_front::{read, Form, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_model::Rational;
use eadl_resolve::offer::{self, Provider};
use eadl_resolve::refusal::Cause;
use eadl_resolve::relation::{enumerate, enumerate_clause, judge, Outcome, Verdict};
use eadl_resolve::requirement::{self, Requirement};
use eadl_resolve::value::Value;
use eadl_resolve::vocabulary::{Direction, Rule, Vocabulary};

fn vocabulary() -> Vocabulary {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf();
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root.join(r)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the kinds load");
    Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
}

fn form(text: &str) -> Form {
    let mut sources = SourceMap::new();
    let id = sources.add("t.eadl", text).expect("small");
    let (doc, diags) = read(&sources, id);
    assert!(!diags.has_errors(), "{text}");
    doc.forms.into_iter().next().expect("one form")
}

fn provider(v: &Vocabulary, name: &str, body: &str) -> Provider {
    offer::read(&form(&format!("(defblock {name} {body})")), v)
        .unwrap_or_else(|r| panic!("{body}: {r:?}"))
}

fn refused(v: &Vocabulary, body: &str) -> Vec<Cause> {
    offer::read(&form(&format!("(defblock p {body})")), v)
        .err()
        .map(|r| r.iter().map(|x| x.cause).collect())
        .unwrap_or_default()
}

fn need(v: &Vocabulary, text: &str) -> Requirement {
    requirement::read_constraint(&form(text), v).unwrap_or_else(|r| panic!("{text}: {r:?}"))
}

fn presence(v: &Vocabulary, fact: &str) -> Requirement {
    requirement::read_needs(&form(fact), v)
        .expect("reads")
        .expect("a fact")
}

fn verdict(v: &Vocabulary, body: &str, required: &str) -> Verdict {
    judge(&provider(v, "p", body), &need(v, required), v)
}

#[test]
fn wrap_interval_the_horizon_is_derived_and_more_of_it_satisfies() {
    let v = vocabulary();
    let wanted = "(unambiguous-horizon (at-least 60 s))";
    assert_eq!(
        verdict(
            &v,
            "(offers (counter-modulus 4294967296) (tick-rate 10 MHz))",
            wanted
        ),
        Verdict::Satisfied
    );
    assert_eq!(
        verdict(
            &v,
            "(offers (counter-modulus 65536) (tick-rate 10 MHz))",
            wanted
        ),
        Verdict::Refused
    );
    for written in ["(unambiguous-horizon 3600 s)", "unambiguous-horizon"] {
        for beside in [
            "(counter-modulus 65536)",
            "(tick-rate 10 MHz)",
            "(wrap-behavior modular)",
            "tick-rate",
        ] {
            assert_eq!(
                refused(&v, &format!("(offers {written} {beside})")),
                [Cause::DerivedBesideInput],
                "{written} beside {beside}"
            );
        }
    }
    // Offered alone, the horizon is a claim, believed: an epoch extender's, a provider in its own right (§4).
    assert_eq!(
        verdict(&v, "(offers (unambiguous-horizon 3600 s))", wanted),
        Verdict::Satisfied
    );
}

#[test]
fn read_atomicity_true_or_bare_satisfies_and_false_or_absent_does_not() {
    let v = vocabulary();
    let wanted = "(observation-coherent true)";
    assert_eq!(
        verdict(&v, "(offers (observation-coherent true))", wanted),
        Verdict::Satisfied
    );
    assert_eq!(
        verdict(&v, "(offers observation-coherent)", wanted),
        Verdict::Satisfied
    );
    assert_eq!(
        verdict(&v, "(offers (observation-coherent false))", wanted),
        Verdict::Refused
    );
    assert_eq!(
        verdict(&v, "(absent observation-coherent)", wanted),
        Verdict::Absent
    );
}

#[test]
fn programming_range_a_group_is_its_head_and_its_sub_facts() {
    let v = vocabulary();
    let range = "(supported-horizon (at-least 10 s))";
    assert_eq!(
        verdict(
            &v,
            "(offers (absolute-deadline true) (supported-horizon 3600 s))",
            range
        ),
        Verdict::Satisfied
    );
    assert_eq!(
        verdict(
            &v,
            "(offers (absolute-deadline true) (supported-horizon 5 s))",
            range
        ),
        Verdict::Refused
    );
    let group =
        "(absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 50 us)))";
    assert_eq!(
        verdict(
            &v,
            "(offers (absolute-deadline true) (supported-horizon 3600 s) (delivery-bound 20 us))",
            group
        ),
        Verdict::Satisfied
    );
    // `timer.delay` today: the head declared absent, so the group is absent there (§6; R1 A7, A13).
    assert_eq!(
        verdict(
            &v,
            "(offers relative-delay) (absent absolute-deadline)",
            group
        ),
        Verdict::Absent
    );
}

#[test]
fn power_state_the_offered_set_covers_every_state_required_and_run() {
    let v = vocabulary();
    let idle = "(available-in-state idle)";
    assert_eq!(
        verdict(&v, "(offers (available-in-state run idle))", idle),
        Verdict::Satisfied
    );
    assert_eq!(
        verdict(&v, "(offers (available-in-state run))", idle),
        Verdict::Refused
    );
    assert_eq!(
        verdict(&v, "(offers (available-in-state idle))", idle),
        Verdict::Refused,
        "idle alone lacks run"
    );
}

#[test]
fn access_privilege_a_stronger_precondition_is_never_a_stronger_capability() {
    let v = vocabulary();
    let supervisor = "(reachable-at-privilege supervisor)";
    assert_eq!(
        verdict(
            &v,
            "(offers (reachable-at-privilege supervisor machine))",
            supervisor
        ),
        Verdict::Satisfied
    );
    // Inclusion reads no order: a machine-only function does not serve a supervisor caller.
    assert_eq!(
        verdict(&v, "(offers (reachable-at-privilege machine))", supervisor),
        Verdict::Refused
    );
    assert_eq!(
        verdict(&v, "(offers (reachable-at-privilege user))", supervisor),
        Verdict::Refused
    );
    assert_eq!(
        verdict(&v, "(offers uart)", supervisor),
        Verdict::Undescribed
    );
    assert_eq!(
        verdict(&v, "(absent reachable-at-privilege)", supervisor),
        Verdict::Absent
    );
    assert_eq!(
        verdict(&v, "(offers reachable-at-privilege)", supervisor),
        Verdict::Unknown
    );
}

#[test]
fn mediation_is_the_requirer_s_word_and_constrains_no_provider() {
    let v = vocabulary();
    for word in [
        "(or-through-mediation allowed)",
        "(or-through-mediation (exactly forbidden))",
    ] {
        assert_eq!(
            verdict(&v, "(offers uart)", word),
            Verdict::Statement,
            "{word}"
        );
    }
    assert_eq!(
        refused(&v, "(offers (or-through-mediation allowed))"),
        [Cause::StatementOffered]
    );
}

#[test]
fn output_units_ns_satisfies_ns_and_us_does_not() {
    let v = vocabulary();
    assert_eq!(
        verdict(&v, "(offers (tick-unit ns))", "(tick-unit ns)"),
        Verdict::Satisfied
    );
    assert_eq!(
        verdict(&v, "(offers (tick-unit us))", "(tick-unit ns)"),
        Verdict::Refused
    );
}

#[test]
fn more_bits_satisfy_a_capacity_and_not_an_interface() {
    let v = vocabulary();
    assert_eq!(
        verdict(
            &v,
            "(offers (counter-width 64 bit))",
            "(counter-width 32 bit)"
        ),
        Verdict::Satisfied
    );
    assert_eq!(
        verdict(
            &v,
            "(offers (counter-width 64 bit))",
            "(counter-width (exactly 32 bit))"
        ),
        Verdict::Refused
    );
    // An abstract platform's bound is the same bound owed by a refinement: no value, so unknown to a valued
    // constraint, and present to presence (§2, R8 H5).
    let bound = provider(
        &v,
        "soc.abstract",
        "(offers (counter-width (at-least 32 bit)))",
    );
    assert_eq!(
        judge(&bound, &need(&v, "(counter-width 32 bit)"), &v),
        Verdict::Unknown
    );
    assert_eq!(
        judge(&bound, &presence(&v, "counter-width"), &v),
        Verdict::Satisfied
    );
}

#[test]
fn the_enumeration_lists_every_provider_with_its_outcome_value_derivation_and_direction() {
    let v = vocabulary();
    let providers = [
        provider(
            &v,
            "timer.wide",
            "(offers (counter-modulus 4294967296) (tick-rate 10 MHz))",
        ),
        provider(
            &v,
            "timer.narrow",
            "(offers (counter-modulus 65536) (tick-rate 10 MHz))",
        ),
        provider(
            &v,
            "timer.gone",
            "(offers (tick-rate 10 MHz)) (absent counter-modulus)",
        ),
        provider(
            &v,
            "timer.bare",
            "(offers (counter-modulus 65536) tick-rate)",
        ),
        provider(&v, "console.uart", "(offers uart)"),
    ];
    let wanted = need(&v, "(unambiguous-horizon (at-least 60 s))");
    let list = enumerate(&wanted, &providers, &v);
    let names: Vec<&str> = list.iter().map(|c| c.provider.as_str()).collect();
    assert_eq!(
        names,
        [
            "timer.wide",
            "timer.narrow",
            "timer.gone",
            "timer.bare",
            "console.uart"
        ],
        "in the order given"
    );
    let verdicts: Vec<Verdict> = list.iter().map(|c| c.verdict).collect();
    assert_eq!(
        verdicts,
        [
            Verdict::Satisfied,
            Verdict::Refused,
            Verdict::Absent,
            Verdict::Unknown,
            Verdict::Undescribed
        ]
    );
    assert!(list.iter().all(|c| c.direction == Direction::AtLeast));
    // The value found and the derivation used: (2^32 − 1) / 10 MHz = 429.4967295 s.
    match &list[0].outcome {
        Some(Outcome::Derived {
            value: Value::Quantity(h),
            rule,
        }) => {
            assert_eq!(*rule, Rule::HorizonFromModulusAndRate);
            assert_eq!(
                h.in_base().expect("fits"),
                Rational::new(4_294_967_295, 10_000_000).expect("fits")
            );
        }
        other => panic!("{other:?}"),
    }
    assert_eq!(list[2].outcome, Some(Outcome::Absent));
    assert_eq!(list[3].outcome, Some(Outcome::Unknown));
    assert_eq!(list[4].outcome, Some(Outcome::Undescribed));
    // Per clause and provider, rule 5's verdict; the statement counts for no provider.
    let clause = requirement::read_clause(
        &form("(requires (unambiguous-horizon (at-least 60 s)) (or-through-mediation allowed))"),
        &v,
    )
    .expect("reads");
    let per_provider = enumerate_clause(&clause, &providers, &v);
    assert_eq!(per_provider[0], ("timer.wide".to_string(), true));
    assert!(per_provider[1..].iter().all(|(_, holds)| !holds));
}
