//! §12's selection, conflicts and groups (`M2.7.3.5.2`).

mod common;

use archogen_catalog::hash::Catalog;
use archogen_catalog::history::History;
use archogen_catalog::record::FacetKind;
use archogen_catalog::replay::replay;
use archogen_catalog::selection::{Selection, Supplied};
use archogen_catalog::tree::Tree;
use archogen_catalog::Code;
use common::*;

const P: &str = "rt-static-up-v1";

/// A record `id` on `targets`, with these behavioral facts, timing facts and costs; its facts are `unknown`.
fn rec(
    id: &str,
    targets: &str,
    behavior: &[&str],
    timing: &[&str],
    costs: &[(&str, &str)],
) -> String {
    let fact = |name: &&str| format!("(fact {name} (unknown \"not known\"))");
    let behavior: Vec<String> = behavior.iter().map(fact).collect();
    let timing: Vec<String> = timing.iter().map(fact).collect();
    let costs: Vec<String> = costs
        .iter()
        .map(|(name, target)| format!("(cost {name} (target {target}) (unknown \"not measured\"))"))
        .collect();
    let record = edit(
        &edit(
            &bare(),
            "(facts (fact one-processor yes (locator (file \"docs/example/model.txt\")) (basis \"the model says so\"))))",
            &format!("(facts {}))", behavior.join(" ")),
        ),
        "(timing-model (version \"0.1.0\") (none \"the costs of a machine are its devices'\"))",
        &format!(
            "(timing-model (version \"0.1.0\") (sources) (measured-with) (facts {}) (costs {}))",
            timing.join(" "),
            costs.join(" ")
        ),
    );
    edit(
        &edit(
            &record,
            "(catalog-record example.base",
            &format!("(catalog-record {id}"),
        ),
        "(targets example-target)",
        &format!("(targets {targets})"),
    )
}

fn catalog(records: &[&str]) -> Catalog {
    Catalog::read(tree(records, &[])).unwrap_or_else(|e| panic!("{e}"))
}

fn on(target: Option<&str>) -> Selection<'_> {
    Selection { profile: P, target }
}

/// The id a lookup finds, or `None`.
fn found(c: &Catalog, target: Option<&str>, facet: FacetKind, name: &str) -> Option<String> {
    c.lookup(on(target), facet, name)
        .unwrap_or_else(|e| panic!("{e}"))
        .map(|f| f.id.to_owned())
}

#[test]
fn facts_come_from_records_whose_profile_and_targets_admit_the_selection() {
    let a = rec("rec.a", "example-target", &["one-processor"], &[], &[]);
    let b = rec("rec.b", "twin-target", &["one-processor"], &[], &[]);
    let c = catalog(&[&a, &b]);
    c.check_selections().unwrap_or_else(|e| panic!("{e}"));
    assert_eq!(
        found(&c, Some("example-target"), BEHAVIOR, "one-processor").as_deref(),
        Some("rec.a")
    );
    assert_eq!(
        found(&c, Some("twin-target"), BEHAVIOR, "one-processor").as_deref(),
        Some("rec.b")
    );
    assert_eq!(
        found(&c, None, BEHAVIOR, "one-processor"),
        None,
        "no target: only `(targets any)`"
    );
    assert_eq!(
        found(&c, Some("example-target"), TIMING, "one-processor"),
        None,
        "by facet"
    );
    let other_profile = c
        .lookup(
            Selection {
                profile: "rt-other-v1",
                target: Some("example-target"),
            },
            BEHAVIOR,
            "one-processor",
        )
        .unwrap();
    assert_eq!(other_profile, None, "by profile");
    let any = rec("rec.any", "any", &["compare-level"], &[], &[]);
    let c = catalog(&[&a, &any]);
    for target in [Some("example-target"), Some("twin-target"), None] {
        assert_eq!(
            found(&c, target, BEHAVIOR, "compare-level").as_deref(),
            Some("rec.any"),
            "{target:?}"
        );
    }
}

#[test]
fn costs_come_only_on_their_own_target_and_never_with_no_target() {
    let a = rec("rec.a", "any", &[], &[], &[("dispatch", "example-target")]);
    let c = catalog(&[&a]);
    let hit = c
        .lookup(on(Some("example-target")), TIMING, "dispatch")
        .unwrap()
        .unwrap();
    assert_eq!(hit.id, "rec.a");
    assert!(matches!(hit.value, Supplied::Cost(cost) if cost.target == "example-target"));
    assert_eq!(
        found(&c, Some("twin-target"), TIMING, "dispatch"),
        None,
        "another target"
    );
    assert_eq!(
        found(&c, None, TIMING, "dispatch"),
        None,
        "no target reads no cost"
    );
    let other_profile = Selection {
        profile: "rt-other-v1",
        target: Some("example-target"),
    };
    assert_eq!(
        c.lookup(other_profile, TIMING, "dispatch").unwrap(),
        None,
        "by profile"
    );
}

#[test]
fn two_records_supplying_one_name_under_one_selection_are_refused() {
    let a = rec("rec.a", "example-target", &["one-processor"], &[], &[]);
    let known = rec("rec.b", "example-target", &["one-processor"], &[], &[]);
    let c = catalog(&[&a, &known]);
    let r = c.check_selections().unwrap_err();
    assert_eq!(r.code, Code::Conflict, "{r}");
    assert!(
        r.message.contains("rec.a") && r.message.contains("rec.b"),
        "{r}"
    );
    assert_eq!(
        c.lookup(on(Some("example-target")), BEHAVIOR, "one-processor")
            .unwrap_err()
            .code,
        Code::Conflict
    );
    // `any` meets a named target there.
    let any = rec("rec.any", "any", &["one-processor"], &[], &[]);
    assert_eq!(
        catalog(&[&a, &any]).check_selections().unwrap_err().code,
        Code::Conflict
    );
    // Costs: one name on one target.
    let ca = rec(
        "rec.a",
        "example-target",
        &[],
        &[],
        &[("dispatch", "example-target")],
    );
    let cb = rec("rec.b", "any", &[], &[], &[("dispatch", "example-target")]);
    assert_eq!(
        catalog(&[&ca, &cb]).check_selections().unwrap_err().code,
        Code::Conflict
    );
    let cc = rec("rec.b", "any", &[], &[], &[("dispatch", "twin-target")]);
    catalog(&[&ca, &cc])
        .check_selections()
        .unwrap_or_else(|e| panic!("two targets: {e}"));
    // With no named target at all, the selection with no target is still checked.
    let mut bare_tree = Tree::default();
    for (id, text) in [
        ("rec.a", rec("rec.a", "any", &["one-processor"], &[], &[])),
        ("rec.b", rec("rec.b", "any", &["one-processor"], &[], &[])),
    ] {
        bare_tree.insert(
            format!("catalog/experimental/{id}.catalog"),
            text.as_bytes(),
        );
    }
    bare_tree.insert("docs/example/model.txt", b"model\n".to_vec());
    let r = Catalog::read(bare_tree)
        .unwrap()
        .check_selections()
        .unwrap_err();
    assert_eq!(r.code, Code::Conflict, "{r}");
    assert!(r.message.contains("no target"), "{r}");
}

#[test]
fn a_grouped_name_comes_from_its_anchors_record() {
    // Every member of every group, typed from §12.
    for (anchor, member, member_facet) in [
        ("switch", "eager-switching", TIMING),
        ("switch", "interrupts-do-not-nest", BEHAVIOR),
        ("switch", "services-preempt-every-task", BEHAVIOR),
        ("switch", "pending-taken-and-transitions-unmasked", BEHAVIOR),
        ("switch", "pending-taken-after-unmask", BEHAVIOR),
        ("switch", "no-empty-claim", BEHAVIOR),
        ("switch", "one-claim-per-trap", BEHAVIOR),
        ("switch", "starts-by-transition", BEHAVIOR),
        ("service.uart", "no-application-code.uart", BEHAVIOR),
        ("service.uart", "acknowledge-at-entry.uart", BEHAVIOR),
        ("service.uart", "defers-nothing.uart", BEHAVIOR),
        ("service.uart", "one-request-per-arrival.uart", BEHAVIOR),
        ("service.uart", "external.uart", BEHAVIOR),
        ("timer-service", "timer-event-driven", BEHAVIOR),
        ("timer-service", "compare-rounds-up", BEHAVIOR),
        ("timer-service", "due-check-matches-compare", BEHAVIOR),
        ("timer-service", "no-early-release", BEHAVIOR),
        ("timer-service", "raised-only-when-due", BEHAVIOR),
        ("timer-service", "only-timer-releases-timer-tasks", BEHAVIOR),
        ("timer-service", "reprograms-only-in-service", BEHAVIOR),
        ("timer-service", "compare-rounding", TIMING),
        ("completion", "api.mask", TIMING),
        ("completion", "masked.mask", TIMING),
        ("completion", "masked.completion", TIMING),
        ("completion", "no-suspension-primitive", BEHAVIOR),
        ("completion", "no-scheduler-lock-primitive", BEHAVIOR),
        ("completion", "preemptive-everywhere", BEHAVIOR),
        ("completion", "sections-mask-every-interrupt", BEHAVIOR),
        ("completion", "releases-never-latched", BEHAVIOR),
        ("completion", "primitives-out-of-line", BEHAVIOR),
    ] {
        let is_cost = matches!(
            member,
            "compare-rounding" | "api.mask" | "masked.mask" | "masked.completion"
        );
        let supplier = |id: &str, target: &str| {
            if is_cost {
                rec(id, "example-target", &[], &[], &[(member, target)])
            } else if member_facet == TIMING {
                rec(id, "example-target", &[], &[member], &[])
            } else {
                rec(id, "example-target", &[member], &[], &[])
            }
        };
        let anchored = rec("rec.a", "any", &[], &[], &[(anchor, "example-target")]);
        let elsewhere = supplier("rec.b", "example-target");
        let c = catalog(&[&anchored, &elsewhere]);
        let r = c.check_selections().unwrap_err();
        assert_eq!(r.code, Code::Field, "{member} beside {anchor}: {r}");
        assert!(r.message.contains(&format!("`{anchor}` group")), "{r}");
        assert_eq!(
            found(&c, Some("example-target"), member_facet, member),
            None,
            "{member}: only from the anchor's record"
        );
        // Where no record supplies the anchor, the group is neither checked nor read.
        let alone = catalog(&[&elsewhere]);
        alone
            .check_selections()
            .unwrap_or_else(|e| panic!("{member} alone: {e}"));
        assert_eq!(
            found(&alone, Some("example-target"), member_facet, member),
            None,
            "{member} not read"
        );
        // Anchor and member from one record: read from it, and never with no target.
        let both = if is_cost {
            rec(
                "rec.a",
                "any",
                &[],
                &[],
                &[(anchor, "example-target"), (member, "example-target")],
            )
        } else if member_facet == TIMING {
            rec(
                "rec.a",
                "any",
                &[],
                &[member],
                &[(anchor, "example-target")],
            )
        } else {
            rec(
                "rec.a",
                "any",
                &[member],
                &[],
                &[(anchor, "example-target")],
            )
        };
        // A record supplying `switch` is the port, and owes §14.4's statement.
        let both = if anchor == "switch" {
            ported(&both, "example-target")
        } else {
            both
        };
        let c = catalog(&[&both, &convention()]);
        c.check_selections()
            .unwrap_or_else(|e| panic!("{member} with {anchor}: {e}"));
        assert_eq!(
            found(&c, Some("example-target"), member_facet, member).as_deref(),
            Some("rec.a"),
            "{member}"
        );
        assert_eq!(
            found(&c, None, member_facet, member),
            None,
            "{member}: no target reads no cost-anchored group"
        );
    }
    // The rule is checked only where the anchor is supplied.
    let anchored = ported(
        &rec("rec.a", "any", &[], &[], &[("switch", "twin-target")]),
        "twin-target",
    );
    let elsewhere = rec(
        "rec.b",
        "example-target",
        &["interrupts-do-not-nest"],
        &[],
        &[],
    );
    catalog(&[&anchored, &elsewhere, &convention()])
        .check_selections()
        .unwrap_or_else(|e| panic!("{e}"));
    // Another source's names are not this source's group.
    let anchored = rec(
        "rec.a",
        "any",
        &[],
        &[],
        &[("service.uart", "example-target")],
    );
    let rtc = rec("rec.b", "example-target", &["external.rtc"], &[], &[]);
    catalog(&[&anchored, &rtc])
        .check_selections()
        .unwrap_or_else(|e| panic!("{e}"));
}

#[test]
fn the_replay_refuses_a_conflict() {
    let a = rec("rec.a", "example-target", &["one-processor"], &[], &[]);
    let b = rec("rec.b", "example-target", &["one-processor"], &[], &[]);
    let mut h = History::default();
    add(&mut h, &n('1'), &[], tree(&[&a, &b], &[]), None);
    assert_eq!(replay(&h, &[], &n('1')).unwrap_err().code, Code::Conflict);
}
