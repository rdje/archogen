//! Model §3 rule 3 through composition and through a chain (leaf `M3.4.8`, `docs/decisions/decision_absence-per-provider.md`):
//! an absence is one declaration's statement, so model §2 rule 1 no longer refuses an abstract platform's absence beside
//! an offer elsewhere — and the exclusion a refinement owes is kept by what the concrete platform composes, and owed to
//! every platform up the chain it refines, so no absence is lost on the way.

use eadl_front::SourceMap;
use eadl_model::check::{check, shipped_registry};
use eadl_model::profile::RT_STATIC_UP_V1;

fn root() -> std::path::PathBuf {
    std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .expect("two levels below the root")
        .to_path_buf()
}

/// The language's check of `text`: its verdict's slug and its diagnostics, rendered.
fn checked(text: &str) -> (String, String) {
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root().join(r)).expect("a kind module"),
        )
    })
    .collect();
    let mut sources = SourceMap::new();
    let registry = shipped_registry(&mut sources, &kinds).expect("the kinds load");
    let id = sources.add("t.eadl", text).expect("small");
    let outcome = check(&sources, id, &registry, &RT_STATIC_UP_V1);
    let rendered = outcome
        .diagnostics
        .iter()
        .map(|d| d.render(&sources))
        .collect::<Vec<_>>()
        .join("\n");
    (outcome.verdict.slug().to_owned(), rendered)
}

#[test]
fn an_abstract_platform_s_absence_is_kept_through_what_the_refinement_composes() {
    let (verdict, rendered) = checked(
        "(eadl-version eadl/1)
(defblock uart.debug (offers debug-port))
(defplatform board (absent debug-port))
(defplatform board.rev2 (refines board) (requires (uses uart.debug)))
",
    );
    assert_eq!(verdict, "infeasible-configuration", "{rendered}");
    assert!(
        rendered.contains("`board.rev2` offers `debug-port` through `uart.debug`, which it composes, which `board` declares absent"),
        "{rendered}"
    );
    assert!(
        rendered.contains("violated obligation `exclusion`"),
        "{rendered}"
    );
}

#[test]
fn an_absence_is_owed_through_every_refinement_of_a_chain() {
    let (verdict, rendered) = checked(
        "(eadl-version eadl/1)
(defplatform board (absent debug-port))
(defplatform board.rev2 (refines board))
(defplatform board.rev3 (refines board.rev2) (offers debug-port))
",
    );
    assert_eq!(verdict, "infeasible-configuration", "{rendered}");
    assert!(
        rendered.contains(
            "`board.rev3` offers `debug-port`, which `board` declares absent, and `board.rev2` refines it in turn"
        ),
        "{rendered}"
    );
}

#[test]
fn two_unrelated_declarations_are_two_statements_and_a_kept_exclusion_is_valid() {
    // Model §2 rule 1 per declaration: a block declaring a fact absent beside one offering it refuses nothing, and a
    // refinement composing only a block that keeps the absence keeps it.
    let (verdict, rendered) = checked(
        "(eadl-version eadl/1)
(defblock uart.a (offers observable-output) (absent debug-port))
(defblock uart.b (offers observable-output debug-port))
(defplatform board (absent debug-port))
(defplatform board.rev2 (refines board) (requires (uses uart.a)))
",
    );
    assert_eq!(verdict, "ok", "{rendered}");
}

#[test]
fn every_uses_a_platform_writes_is_composition_its_own_and_a_nested_requires_too() {
    // Review round 2's R2-1: composition is every `uses` a block or a platform writes (model §8 rules 1 and 5), so the
    // exclusion is not dodged by a top-level `uses` or one inside a nested `requires`.
    for (case, composing) in [
        ("a top-level uses", "(uses uart.debug)"),
        (
            "a nested requires",
            "(requires (requires (uses uart.debug)))",
        ),
    ] {
        let (verdict, rendered) = checked(&format!(
            "(eadl-version eadl/1)
(defblock uart.debug (offers debug-port))
(defplatform board (absent debug-port))
(defplatform board.rev2 (refines board) {composing})
"
        ));
        assert_eq!(verdict, "infeasible-configuration", "{case}: {rendered}");
        assert!(
            rendered.contains("through `uart.debug`, which it composes"),
            "{case}: {rendered}"
        );
    }
}

#[test]
fn a_platform_s_absence_covers_what_it_composes() {
    // Review round 2's R2-2: one scope for a declaration's absence, itself and what it is made of. A platform composing
    // a block that offers what it declares absent says both of itself, `invalid-description` (model §2 rule 1), as a
    // refinement repeating that composition breaks the exclusion (§3 rule 3) — neither is accepted.
    let (verdict, rendered) = checked(
        "(eadl-version eadl/1)
(defblock uart.debug (offers debug-port))
(defplatform board (absent debug-port) (requires (uses uart.debug)))
",
    );
    assert_eq!(verdict, "invalid-description", "{rendered}");
    assert!(
        rendered.contains("`debug-port` is declared absent by `board` and offered by `uart.debug`, which it composes"),
        "{rendered}"
    );
    let (verdict, rendered) = checked(
        "(eadl-version eadl/1)
(defblock uart.debug (offers debug-port))
(defplatform board (absent debug-port) (requires (uses uart.debug)))
(defplatform board.rev2 (refines board) (requires (uses uart.debug)))
",
    );
    assert_eq!(verdict, "invalid-description", "{rendered}");
}
