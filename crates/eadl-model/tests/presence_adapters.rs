//! Presence and the engine's adapters (leaf `M3.2.4`, `docs/specs/adapters/decision_engine-adapters.md` §7, `SR-H9`):
//! model §2 rules 2 and 3 yield for a fact a kind adds where one block or platform offers everything the kind needs
//! beneath, answer `missing-fact` where one offers part of it and leaves the rest undescribed, and read as before
//! otherwise — a fact no kind adds, or one some declaration offers, untouched.

use eadl_front::{read, SourceMap};
use eadl_model::presence::{adapter_paths, FactMap, PresenceReport};

fn check(text: &str) -> (PresenceReport, String) {
    let mut sources = SourceMap::new();
    let id = sources.add("t.eadl", text).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(!diagnostics.has_errors(), "{text}");
    let mut map = FactMap::new();
    for form in &document.forms {
        map.collect(form);
    }
    let report = map.check();
    let rendered = report
        .diagnostics
        .iter()
        .map(|d| d.render(&sources))
        .collect::<Vec<_>>()
        .join("\n");
    (report, rendered)
}

/// A deadline service on a timer offering `timer` — the facts written there — and declaring `absent`.
fn deadline_over(timer: &str, absent: &str) -> String {
    format!(
        "(defsystem app.rt (uses time.deadline))
(defservice time.deadline (needs absolute-deadline))
(defblock timer.delay (offers {timer}) {absent})
"
    )
}

#[test]
fn presence_reads_the_catalog_s_names_as_the_record_writes_them() {
    let paths = adapter_paths();
    let kinds: Vec<&str> = paths.iter().map(|p| p.kind.as_str()).collect();
    assert_eq!(kinds, ["deadline-over-delay", "epoch-extender"]);
    assert_eq!(
        paths[0].adds,
        [
            "absolute-deadline",
            "supported-horizon",
            "delivery-bound",
            "channels"
        ]
    );
    assert_eq!(paths[1].adds, ["counter-width"]);
    assert_eq!(
        paths[1].beneath,
        [
            "relative-delay",
            "counter-modulus",
            "tick-rate",
            "observation-coherent",
            "wrap-behavior"
        ]
    );
}

#[test]
fn a_path_by_name_leaves_the_fact_to_the_search() {
    // `timer.delay` offers everything `deadline-over-delay` needs beneath: `absolute-deadline`, absent there or
    // undescribed, is the search's (`SR-H9`).
    let all = "relative-delay counter-modulus tick-rate observation-coherent";
    for absent in ["(absent absolute-deadline)", ""] {
        let (report, rendered) = check(&deadline_over(all, absent));
        assert!(report.is_admissible(), "{absent}: {rendered}");
        assert!(report.missing.is_empty(), "{absent}: {:?}", report.missing);
    }
}

#[test]
fn a_partial_path_is_a_missing_fact_naming_what_to_describe() {
    // `uc3`'s shape: the timer leaves `counter-modulus` undescribed, declared absent or not the deadline is a missing
    // fact whose repair is the modulus.
    let part = "relative-delay tick-rate observation-coherent";
    for absent in ["(absent absolute-deadline)", ""] {
        let (report, rendered) = check(&deadline_over(part, absent));
        assert_eq!(report.diagnostics.len(), 1, "{absent}: {rendered}");
        assert_eq!(report.diagnostics[0].code, "missing-fact", "{rendered}");
        assert!(
            rendered.contains(
                "`timer.delay` offers part of that and leaves `counter-modulus` undescribed"
            ),
            "{rendered}"
        );
        assert!(
            rendered.contains("describe `counter-modulus` at `timer.delay`"),
            "{rendered}"
        );
    }
}

#[test]
fn without_a_path_the_rules_read_as_before_and_say_what_an_adapter_needs() {
    // The timer declares `counter-modulus` absent, so describing nothing makes a path: rule 2 stands, its hint naming
    // the kind; and a timer stating none of it the same, for rule 3.
    let (report, rendered) = check(&deadline_over(
        "relative-delay tick-rate observation-coherent",
        "(absent absolute-deadline counter-modulus)",
    ));
    assert_eq!(report.diagnostics.len(), 1, "{rendered}");
    assert_eq!(report.diagnostics[0].code, "infeasible-configuration");
    assert!(
        rendered.contains("the engine's adapter `deadline-over-delay` offers it over a block or platform offering"),
        "{rendered}"
    );
    let (report, rendered) = check(&deadline_over("observable-output", ""));
    assert_eq!(report.diagnostics[0].code, "missing-fact", "{rendered}");
    assert!(
        rendered.contains("nothing describes it") && rendered.contains("`deadline-over-delay`"),
        "{rendered}"
    );
}

#[test]
fn a_fact_no_kind_adds_is_judged_as_before() {
    // F04's shape: `wrap-behavior` is named beneath by the extender, so no kind adds it, and it stays missing.
    let (report, rendered) = check(
        "(defsystem app.rt (uses time.monotonic))
(defservice time.monotonic (needs counter-width wrap-behavior))
(defblock timer.counter (offers counter-width relative-delay counter-modulus tick-rate observation-coherent))
",
    );
    assert_eq!(report.missing, ["wrap-behavior"], "{rendered}");
    assert!(!rendered.contains("adapter"), "{rendered}");
}
