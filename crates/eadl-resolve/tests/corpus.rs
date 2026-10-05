//! The falsification corpus (leaf `M3.1.1.1`): the record's §6 cases and the probes its review rounds ran, each a
//! fixture with the outcome the record gives it, named by the round and finding it comes from.
//!
//! ⛔ **A fixed defect stays fixed.** A later round's finding is a defect when it carries a reproducer — a fixture
//! this file, or the checker, gets wrong — so every finding that came with one lands here first, failing, and then
//! the rule that makes it pass (`docs/decisions/decision_executable-design-reviews.md`).

use eadl_front::{read, Form, SourceMap};
use eadl_resolve::model::{
    clause_satisfied, judge, read_constraint, read_declaration_name, read_needs, read_provider,
    read_uses, NotJudged, Verdict,
};

fn forms(text: &str) -> Vec<Form> {
    let mut sources = SourceMap::new();
    let id = sources.add("t.eadl", text).expect("small");
    let (doc, diags) = read(&sources, id);
    assert!(!diags.has_errors(), "{text}: {}", diags.render(&sources));
    doc.forms
}

/// What a fixture expects.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Want {
    Judged(Verdict),
    /// The provider is refused, `invalid-description`.
    ProviderInvalid,
    /// The requirement is refused, `invalid-description`.
    RequirementInvalid,
    /// The requirement is `unsupported-profile`.
    RequirementUnsupported,
}

use Verdict::{Absent, Refused, Satisfied, Statement, Undescribed, Unknown, Unsupported};
use Want::{Judged, ProviderInvalid, RequirementInvalid, RequirementUnsupported};

/// One fixture: where it comes from, the provider's clauses, the requirement — an item of `requires`, or
/// `needs X`, `uses X` — and the outcome the record gives it.
struct Case(&'static str, &'static str, &'static str, Want);

const CASES: &[Case] = &[
    // §6 — the timer cases of §5.2.
    Case(
        "§6 wrap interval",
        "(offers (counter-modulus 4294967296) (tick-rate 10 MHz))",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Satisfied),
    ),
    Case(
        "§6 wrap interval, 16-bit",
        "(offers (counter-modulus 65536) (tick-rate 10 MHz))",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Refused),
    ),
    Case(
        "§6 wrap interval, a horizon beside its modulus",
        "(offers (counter-modulus 65536) (unambiguous-horizon 3600 s))",
        "(unambiguous-horizon (at-least 60 s))",
        ProviderInvalid,
    ),
    Case(
        "§6 wrap interval, a horizon beside a bare wrap-behavior",
        "(offers wrap-behavior (unambiguous-horizon 3600 s))",
        "(unambiguous-horizon (at-least 60 s))",
        ProviderInvalid,
    ),
    Case(
        "§6 read atomicity, true",
        "(offers (observation-coherent true))",
        "(observation-coherent true)",
        Judged(Satisfied),
    ),
    Case(
        "§6 read atomicity, bare",
        "(offers observation-coherent)",
        "(observation-coherent true)",
        Judged(Satisfied),
    ),
    Case(
        "§6 read atomicity, false",
        "(offers (observation-coherent false))",
        "(observation-coherent true)",
        Judged(Refused),
    ),
    Case(
        "§6 read atomicity, absent",
        "(absent observation-coherent)",
        "(observation-coherent true)",
        Judged(Absent),
    ),
    Case(
        "§6 programming range",
        "(offers (absolute-deadline true) (supported-horizon 3600 s))",
        "(absolute-deadline (supported-horizon (at-least 10 s)))",
        Judged(Satisfied),
    ),
    Case(
        "§6 programming range, 5 s",
        "(offers (absolute-deadline true) (supported-horizon 5 s))",
        "(absolute-deadline (supported-horizon (at-least 10 s)))",
        Judged(Refused),
    ),
    Case(
        "§6 programming range, the head absent",
        "(absent absolute-deadline) (offers (supported-horizon 3600 s))",
        "(absolute-deadline (supported-horizon (at-least 10 s)))",
        Judged(Absent),
    ),
    Case(
        "§6 power state",
        "(offers (available-in-state run idle))",
        "(available-in-state idle)",
        Judged(Satisfied),
    ),
    Case(
        "§6 power state, run alone",
        "(offers (available-in-state run))",
        "(available-in-state idle)",
        Judged(Refused),
    ),
    Case(
        "§6 power state, idle alone (R16 P2)",
        "(offers (available-in-state idle))",
        "(available-in-state idle)",
        Judged(Refused),
    ),
    Case(
        "§6 access privilege",
        "(offers (reachable-at-privilege supervisor machine))",
        "(reachable-at-privilege supervisor)",
        Judged(Satisfied),
    ),
    Case(
        "§6 access privilege, machine alone",
        "(offers (reachable-at-privilege machine))",
        "(reachable-at-privilege supervisor)",
        Judged(Refused),
    ),
    Case(
        "§6 access privilege, user alone",
        "(offers (reachable-at-privilege user))",
        "(reachable-at-privilege supervisor)",
        Judged(Refused),
    ),
    Case(
        "§6 access privilege, bare (R13 M3)",
        "(offers reachable-at-privilege)",
        "(reachable-at-privilege supervisor)",
        Judged(Unknown),
    ),
    Case(
        "§6 access privilege, absent",
        "(absent reachable-at-privilege)",
        "(reachable-at-privilege supervisor)",
        Judged(Absent),
    ),
    Case(
        "§6 access privilege, nothing stated",
        "",
        "(reachable-at-privilege supervisor)",
        Judged(Undescribed),
    ),
    Case(
        "§6 mediation is a statement",
        "",
        "(or-through-mediation forbidden)",
        Judged(Statement),
    ),
    Case(
        "§6 output units",
        "(offers (tick-unit ns))",
        "(tick-unit ns)",
        Judged(Satisfied),
    ),
    Case(
        "§6 output units, us",
        "(offers (tick-unit us))",
        "(tick-unit ns)",
        Judged(Refused),
    ),
    Case(
        "§6 more bits",
        "(offers (counter-width 64 bit))",
        "(counter-width 32 bit)",
        Judged(Satisfied),
    ),
    Case(
        "§6 more bits, exactly",
        "(offers (counter-width 64 bit))",
        "(counter-width (exactly 32 bit))",
        Judged(Refused),
    ),
    Case(
        "§6 more bits, the same amount",
        "(offers (counter-width 4 byte))",
        "(counter-width (exactly 32 bit))",
        Judged(Satisfied),
    ),
    // §4's worked numbers.
    Case(
        "§4 reload counter",
        "(offers (counter-modulus 1000000) (tick-rate 10 MHz))",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Refused),
    ),
    Case(
        "§4 a 16-bit counter at 18 Hz holds an hour (R7 G6)",
        "(offers (counter-modulus 65536) (tick-rate 18 Hz))",
        "(unambiguous-horizon (at-least 3600 s))",
        Judged(Satisfied),
    ),
    Case(
        "§4 a horizon offered beside a width alone stands (R7 G6)",
        "(offers (counter-width 16 bit) (unambiguous-horizon 3600 s))",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Satisfied),
    ),
    Case(
        "§4 the target's mtime",
        "(offers (counter-modulus (pow2 64)) (tick-rate 10 MHz))",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Satisfied),
    ),
    Case(
        "§4 no modulus, no horizon",
        "(offers (counter-width 32 bit) (tick-rate 10 MHz))",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Undescribed),
    ),
    // Review findings with a reproducer.
    Case(
        "R14 N9 the endpoint",
        "(offers (counter-modulus 600000000) (tick-rate 10 MHz))",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Refused),
    ),
    Case(
        "R3 C5 a bare rate",
        "(offers (counter-modulus 4294967296) tick-rate)",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Unknown),
    ),
    Case(
        "R7 G3 an optional input absent",
        "(offers (counter-modulus 4294967296) (tick-rate 10 MHz)) (absent wrap-behavior)",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Undescribed),
    ),
    Case(
        "R12 L3 a required input absent",
        "(offers (tick-rate 10 MHz)) (absent counter-modulus)",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Absent),
    ),
    Case(
        "R12 L3 a bare horizon beside an absent input",
        "(offers unambiguous-horizon) (absent counter-modulus)",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Unknown),
    ),
    Case(
        "R16 P8 a derived fact absent beside its grounds",
        "(offers (counter-modulus 65536) (tick-rate 10 MHz)) (absent unambiguous-horizon)",
        "(unambiguous-horizon (at-least 60 s))",
        ProviderInvalid,
    ),
    Case(
        "R9 I5 saturating beside a modulus",
        "(offers (counter-modulus 65536) (wrap-behavior saturating))",
        "(counter-modulus 65536)",
        ProviderInvalid,
    ),
    Case(
        "R3 C12 a modulus above 2^width",
        "(offers (counter-width 32 bit) (counter-modulus 8589934592))",
        "(counter-modulus 8589934592)",
        ProviderInvalid,
    ),
    Case(
        "R3 C12 a modulus of exactly 2^width",
        "(offers (counter-width 32 bit) (counter-modulus 4294967296))",
        "(counter-modulus 4294967296)",
        Judged(Satisfied),
    ),
    Case(
        "R8 H3 a width of 1.5 bit",
        "(offers (counter-width 1.5 bit))",
        "(counter-width 1 bit)",
        ProviderInvalid,
    ),
    Case(
        "R9 I13 a modulus of 0",
        "(offers (counter-modulus 0))",
        "(counter-modulus 1)",
        ProviderInvalid,
    ),
    Case(
        "R7 G1 (pow2 127) is no count",
        "(offers (counter-modulus (pow2 127)))",
        "(counter-modulus 1)",
        ProviderInvalid,
    ),
    Case(
        "R13 M10 the arithmetic's limit",
        "(offers (counter-modulus (pow2 126)) (tick-rate 0.25 Hz))",
        "(unambiguous-horizon (at-least 60 s))",
        Judged(Unsupported),
    ),
    Case(
        "R10 J4 (exactly false) meets no presence",
        "(offers (observation-coherent (exactly false)))",
        "needs observation-coherent",
        Judged(Refused),
    ),
    Case(
        "R7 G2 false meets no presence",
        "(offers (observation-coherent false))",
        "needs observation-coherent",
        Judged(Refused),
    ),
    Case(
        "R8 H5 an abstract bound meets presence",
        "(offers (counter-width (at-least 32 bit)))",
        "needs counter-width",
        Judged(Satisfied),
    ),
    Case(
        "R8 H5 an abstract bound is no value",
        "(offers (counter-width (at-least 32 bit)))",
        "(counter-width 32 bit)",
        Judged(Unknown),
    ),
    Case(
        "R7 G10 a derivation meets presence",
        "(offers (counter-modulus 65536) (tick-rate 10 MHz))",
        "needs unambiguous-horizon",
        Judged(Satisfied),
    ),
    Case(
        "R10 J8 a bound beside a value",
        "(offers (counter-width 32 bit) (counter-width (at-least 16 bit)))",
        "(counter-width 16 bit)",
        ProviderInvalid,
    ),
    Case(
        "R5 E10 two values",
        "(offers (tick-rate 10 MHz) (tick-rate 20 MHz))",
        "(tick-rate 10 MHz)",
        ProviderInvalid,
    ),
    Case(
        "R15 O13 the same value spelt twice",
        "(offers (tick-rate 10 MHz) (tick-rate 10000 kHz))",
        "(tick-rate 10 MHz)",
        Judged(Satisfied),
    ),
    Case(
        "R13 M7 a bare offer beside a value is that value",
        "(offers tick-rate (tick-rate 10 MHz))",
        "(tick-rate 10 MHz)",
        Judged(Satisfied),
    ),
    Case(
        "R14 N8 a bare boolean beside false",
        "(offers observation-coherent (observation-coherent false))",
        "(observation-coherent true)",
        ProviderInvalid,
    ),
    Case(
        "R11 K9 a bare boolean beside true",
        "(offers observation-coherent (observation-coherent true))",
        "(observation-coherent true)",
        Judged(Satisfied),
    ),
    Case(
        "model §2 rule 1 offered and absent",
        "(offers uart) (absent uart)",
        "(uart true)",
        ProviderInvalid,
    ),
    Case(
        "R12 L6 an offer of a statement fact",
        "(offers (or-through-mediation allowed))",
        "(uart true)",
        ProviderInvalid,
    ),
    Case(
        "R12 L6 a needs of a statement fact",
        "",
        "needs or-through-mediation",
        RequirementInvalid,
    ),
    Case(
        "R14 N6 a bare name inside requires",
        "",
        "observation-coherent",
        RequirementInvalid,
    ),
    Case(
        "R14 N6 (f) inside requires",
        "",
        "(observation-coherent)",
        RequirementInvalid,
    ),
    Case(
        "R14 N7 a value inside needs",
        "",
        "needs (tick-unit us)",
        RequirementInvalid,
    ),
    Case(
        "R15 O1 a uses naming a fact",
        "",
        "uses observation-coherent",
        RequirementInvalid,
    ),
    Case(
        "R15 O1 a uses naming a fact's value",
        "",
        "uses (tick-unit us)",
        RequirementInvalid,
    ),
    Case(
        "R15 O16 an includes wrapper",
        "",
        "(available-in-state (includes idle))",
        RequirementInvalid,
    ),
    Case(
        "R16 P4 an exact wrapper",
        "",
        "(tick-unit (exact ns))",
        RequirementInvalid,
    ),
    Case(
        "R2 B4 a direction against the fact's",
        "",
        "(unambiguous-horizon (at-most 60 s))",
        RequirementInvalid,
    ),
    Case(
        "R2 B4 exactly is always written",
        "(offers (counter-modulus 4294967296) (tick-rate 10 MHz))",
        "(unambiguous-horizon (exactly 429.4967295 s))",
        Judged(Satisfied),
    ),
    Case(
        "R12 L12 an interval with lo > hi",
        "",
        "(frequency (range 300 MHz 1 MHz))",
        RequirementInvalid,
    ),
    Case(
        "R12 L12 (f) offered for a set is bare",
        "(offers (available-in-state))",
        "(available-in-state idle)",
        Judged(Unknown),
    ),
    Case(
        "R13 M11 (f (exactly)) offered for a set",
        "(offers (available-in-state (exactly)))",
        "(available-in-state idle)",
        ProviderInvalid,
    ),
    Case(
        "R3 C4 priorities are exact",
        "(offers (priorities static unique))",
        "(priorities static)",
        Judged(Refused),
    ),
    Case(
        "R3 C4 priorities written alike",
        "(offers (priorities unique static))",
        "(priorities static unique)",
        Judged(Satisfied),
    ),
    Case(
        "§2 within, a point inside",
        "(offers (frequency (range 1 MHz 200 MHz)))",
        "(frequency 10 MHz)",
        Judged(Satisfied),
    ),
    Case(
        "§2 within, a range outside",
        "(offers (frequency (range 1 MHz 200 MHz)))",
        "(frequency (range 50 MHz 300 MHz))",
        Judged(Refused),
    ),
    Case(
        "§2 another dimension",
        "",
        "(counter-width 32 MHz)",
        RequirementInvalid,
    ),
    Case(
        "§8 a fact the vocabulary does not declare",
        "",
        "(ordering (before a b))",
        RequirementUnsupported,
    ),
    Case(
        "§2 a nested group offer",
        "(offers (absolute-deadline (supported-horizon 3600 s)))",
        "(absolute-deadline true)",
        ProviderInvalid,
    ),
    Case(
        "§2 a group head offered false",
        "(offers (absolute-deadline false) (supported-horizon 3600 s))",
        "(absolute-deadline (supported-horizon (at-least 10 s)))",
        Judged(Refused),
    ),
    Case(
        "§2 a boolean head with a bound",
        "(offers (absolute-deadline (at-least true)))",
        "(absolute-deadline true)",
        ProviderInvalid,
    ),
    Case(
        "R16 P1 a declaration named like a fact",
        "",
        "decl observation-coherent",
        RequirementInvalid,
    ),
    Case(
        "R17 A1 exactly on a power state asks for run too",
        "(offers (available-in-state run idle))",
        "(available-in-state (exactly idle))",
        Judged(Satisfied),
    ),
    Case(
        "R17 A2 exactly idle refused to an idle-only provider",
        "(offers (available-in-state idle))",
        "(available-in-state (exactly idle))",
        Judged(Refused),
    ),
    Case(
        "R17 C1 a statement written under exactly",
        "",
        "(or-through-mediation (exactly allowed))",
        Judged(Statement),
    ),
    Case(
        "R17 D1 a group, unknown before refused",
        "(offers (absolute-deadline true) supported-horizon (delivery-bound 1 ms))",
        "(absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 50 us)))",
        Judged(Refused),
    ),
    Case(
        "R17 D2 the same group, the other order",
        "(offers (absolute-deadline true) supported-horizon (delivery-bound 1 ms))",
        "(absolute-deadline (delivery-bound (at-most 50 us)) (supported-horizon (at-least 10 s)))",
        Judged(Refused),
    ),
    Case(
        "R17 D3 a group whose sub-fact is undescribed",
        "(offers (absolute-deadline true))",
        "(absolute-deadline (supported-horizon (at-least 10 s)))",
        Judged(Undescribed),
    ),
    Case(
        "R17 D4 absent before unknown",
        "(offers (absolute-deadline true) delivery-bound) (absent supported-horizon)",
        "(absolute-deadline (supported-horizon (at-least 10 s)) (delivery-bound (at-most 50 us)))",
        Judged(Absent),
    ),
    Case(
        "R17 D5 the same, the other order",
        "(offers (absolute-deadline true) delivery-bound) (absent supported-horizon)",
        "(absolute-deadline (delivery-bound (at-most 50 us)) (supported-horizon (at-least 10 s)))",
        Judged(Absent),
    ),
    Case(
        "R17 E2 the same value twice whose comparison overflows",
        "(offers (delivery-bound 0.0000000000000000000000000000001 ns) (delivery-bound 0.0000000000000000000000000000001 ns))",
        "(delivery-bound (at-most 1 us))",
        Judged(Unsupported),
    ),
    Case(
        "R17 E1 the value once whose comparison overflows",
        "(offers (delivery-bound 0.0000000000000000000000000000001 ns))",
        "(delivery-bound (at-most 1 us))",
        Judged(Unsupported),
    ),
    Case(
        "R17 F7 a negative bus width offered",
        "(offers (bus-width -8 bit))",
        "(bus-width 8 bit)",
        ProviderInvalid,
    ),
    Case(
        "R17 F6 a bus width of 1.5 bit offered",
        "(offers (bus-width 1.5 bit))",
        "(bus-width 8 bit)",
        ProviderInvalid,
    ),
    Case(
        "R17 F1 a width of 1.5 bit required",
        "(offers (counter-width 32 bit))",
        "(counter-width (at-least 1.5 bit))",
        RequirementInvalid,
    ),
    Case(
        "R17 F2 a width of 0 bit required",
        "(offers (counter-width 32 bit))",
        "(counter-width 0 bit)",
        RequirementInvalid,
    ),
    Case(
        "R17 F3 an abstract bound of 0 bit",
        "(offers (counter-width (at-least 0 bit)))",
        "(counter-width 8 bit)",
        ProviderInvalid,
    ),
    Case(
        "R17 F4 a modulus of 0 required",
        "",
        "(counter-modulus 0)",
        RequirementInvalid,
    ),
    Case(
        "R17 F5 a modulus of exactly 0 required",
        "",
        "(counter-modulus (exactly 0))",
        RequirementInvalid,
    ),
    Case(
        "R17 R4 a statement fact declared absent",
        "(absent or-through-mediation)",
        "(uart true)",
        ProviderInvalid,
    ),    Case(
        "R18 1 two values beside an overflow written first",
        "(offers (delivery-bound 0.0000000000000000000000000000001 ns) (delivery-bound 1 ms) (delivery-bound 2 ms))",
        "(delivery-bound (at-most 1 us))",
        ProviderInvalid,
    ),
    Case(
        "R18 1 the same, the overflow written last",
        "(offers (delivery-bound 1 ms) (delivery-bound 2 ms) (delivery-bound 0.0000000000000000000000000000001 ns))",
        "(delivery-bound (at-most 1 us))",
        ProviderInvalid,
    ),
    Case(
        "R18 1 two values beside an overflow, for presence",
        "(offers (supported-horizon 0.0000000000000000000000000000001 ns) (supported-horizon 10 s) (supported-horizon 20 s))",
        "needs supported-horizon",
        ProviderInvalid,
    ),
    Case(
        "R18 1 one value twice beside an overflow is unsupported",
        "(offers (delivery-bound 0.0000000000000000000000000000001 ns) (delivery-bound 1 ms) (delivery-bound 1000 us))",
        "(delivery-bound (at-most 1 us))",
        Judged(Unsupported),
    ),
    Case(
        "R18 2 a value inside absent",
        "(absent (available-in-state sleep))",
        "(available-in-state idle)",
        ProviderInvalid,
    ),
    Case(
        "R18 2 an input's value inside absent",
        "(offers (tick-rate 10 MHz)) (absent (counter-modulus 65536))",
        "(unambiguous-horizon (at-least 60 s))",
        ProviderInvalid,
    ),
    Case(
        "R18 2 a name inside absent stays an absence",
        "(absent available-in-state)",
        "(available-in-state idle)",
        Judged(Absent),
    ),
    Case(
        "R18 7 an interval /1 can write is ordered exactly: lo > hi, refused",
        "(offers (frequency (range 9.000000000000000001 Hz 0.0000000000000000000000000000001 Hz)))",
        "(frequency 9.2 Hz)",
        ProviderInvalid,
    ),
];

fn run(case: &Case) -> Want {
    let Case(_, offers, requirement, _) = case;
    let p = match read_provider(&forms(&format!("(defblock p {offers})"))[0]) {
        Ok(p) => p,
        Err(_) => return ProviderInvalid,
    };
    let verdict = if let Some(name) = requirement.strip_prefix("decl ") {
        return match read_declaration_name(name) {
            Ok(()) => Judged(Satisfied),
            Err(_) => RequirementInvalid,
        };
    } else if let Some(item) = requirement.strip_prefix("uses ") {
        return match read_uses(&forms(item)[0]) {
            Ok(()) => Judged(Satisfied),
            Err(_) => RequirementInvalid,
        };
    } else if let Some(item) = requirement.strip_prefix("needs ") {
        read_needs(&forms(item)[0])
    } else {
        read_constraint(&forms(requirement)[0])
    };
    match verdict {
        Ok(r) => Judged(judge(&p, &r)),
        Err(NotJudged::Invalid(_)) => RequirementInvalid,
        Err(NotJudged::Unsupported(_)) => RequirementUnsupported,
        Err(NotJudged::NotAFact) => panic!("{}: not a fact", case.0),
    }
}

#[test]
fn every_fixture_gets_the_outcome_the_record_gives_it() {
    let mut wrong = Vec::new();
    for case in CASES {
        let got = run(case);
        if got != case.3 {
            wrong.push(format!(
                "{}: {} against `{}` — the record says {:?}, the model {:?}",
                case.0, case.1, case.2, case.3, got
            ));
        }
    }
    println!("{} fixtures", CASES.len());
    assert!(
        wrong.is_empty(),
        "{} of {} fixtures:\n{}",
        wrong.len(),
        CASES.len(),
        wrong.join("\n")
    );
}

#[test]
fn a_provider_named_like_a_fact_is_refused() {
    // R17 R5: the declaration-name rule (§1.1; R16 1) holds of a provider's own name.
    let provider = &forms("(defblock observation-coherent (offers uart))")[0];
    assert!(read_provider(provider).is_err());
}

#[test]
fn a_clause_is_satisfied_when_each_constraint_is_a_statement_apart() {
    // R18 6: rule 5's clause, modelled so the production relation is held to it (`SR-H7`).
    let p = read_provider(&forms("(defblock p (offers (uart true) (preemptive false)))")[0])
        .expect("reads");
    let c = |t: &str| read_constraint(&forms(t)[0]).expect("a constraint");
    assert!(clause_satisfied(
        &p,
        &[c("(uart true)"), c("(or-through-mediation allowed)")]
    ));
    assert!(!clause_satisfied(
        &p,
        &[c("(uart true)"), c("(preemptive true)")]
    ));
    assert!(!clause_satisfied(
        &p,
        &[c("(uart true)"), c("(debug-port true)")]
    ));
    assert!(clause_satisfied(&p, &[]));
}
