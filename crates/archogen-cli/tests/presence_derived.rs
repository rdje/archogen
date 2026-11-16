//! `archogen check` reads a fact the capability vocabulary derives as the search derives it (model §2 rule 3; leaf
//! `M3.4.17.1`).
//!
//! The rule is the model crate's, and that crate lies below the vocabulary's: a check is told what the vocabulary
//! derives by the caller that reads it. So this holds that the command is told, on the reader's own case: a service
//! needs `unambiguous-horizon`, which no declaration states and the search computes of the timer's modulus and rate.
//! Before this `check` said `missing-fact` of a description `resolve` bound.

use std::fs;
use std::path::Path;

use archogen_cli::{run, Status};

/// What `archogen COMMAND` says of `text`: its status, and everything it wrote, the plan among it where it resolves, unchecked, the search being what is asked here.
fn said(command: &str, name: &str, text: &str) -> (Status, String) {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("presence-derived");
    fs::create_dir_all(&dir).expect("the scratch directory");
    let (file, plan) = (dir.join(name), dir.join(format!("{name}.plan")));
    let _ = fs::remove_file(&plan);
    fs::write(&file, format!("(eadl-version eadl/1)\n{text}\n")).expect("written");
    let mut words = vec![command.to_string(), file.display().to_string()];
    if command == "resolve" {
        words.extend([
            "--out".to_string(),
            plan.display().to_string(),
            "--unchecked".to_string(),
        ]);
    }
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let status = run(words, &mut out, &mut err);
    (
        status,
        format!(
            "{}{}{}",
            String::from_utf8_lossy(&out),
            String::from_utf8_lossy(&err),
            fs::read_to_string(&plan).unwrap_or_default()
        ),
    )
}

const DERIVED: &str = "
(defblock own.timer (offers
    (relative-delay true) (channels 2 tick) (counter-modulus (pow2 32)) (wrap-behavior modular) (tick-rate 10 MHz) (observation-coherent true) (reachable-at-privilege machine)))
(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses own.timer)))
(defservice uh (requires (needs unambiguous-horizon)))
(defsystem s (requires (uses uh)) (platform (uses soc.here)))";

#[test]
fn check_and_resolve_agree_on_a_need_the_vocabulary_derives() {
    let (status, written) = said("check", "derived.eadl", DERIVED);
    assert_eq!(status, Status::Ok, "{written}");
    let (status, written) = said("resolve", "derived.eadl", DERIVED);
    assert_eq!(status, Status::Ok, "{written}");
    assert!(
        written.contains("(bind uh unambiguous-horizon own.timer)"),
        "the search derives it at the timer and binds it there: {written}"
    );
}

#[test]
fn the_command_names_what_a_block_leaves_undescribed_of_the_rule() {
    let partial = DERIVED.replace(" (tick-rate 10 MHz)", "");
    assert_ne!(partial, DERIVED);
    let (status, written) = said("check", "partial.eadl", &partial);
    assert_eq!(status, Status::MissingFact, "{written}");
    assert!(
        written.contains("the vocabulary derives it of `counter-modulus`, `tick-rate`")
            && written
                .contains("`own.timer` offers part of that and leaves `tick-rate` undescribed"),
        "{written}"
    );
}
