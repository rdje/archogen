//! `archogen check` reads a fact the capability vocabulary does not declare on each system's own hardware (model §2
//! and §8 rule 7; leaf `M3.4.16`).
//!
//! The rule is the model crate's, `eadl_model::check::check_with_vocabulary`, and that crate lies below the
//! vocabulary's: a check told no vocabulary reads every description whole. So this holds that the command is told, on
//! the reader's own case: a service needs what only a block outside the system's hardware offers, the search binds
//! nothing for a fact the vocabulary does not declare, and before this the command said `ok` of a machine without the
//! device.

use std::fs;
use std::path::Path;

use archogen_cli::{run, Status};

/// What `archogen check` says of `text`: its status, and everything it wrote.
fn checked(name: &str, text: &str) -> (Status, String) {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join("presence-hardware");
    fs::create_dir_all(&dir).expect("the scratch directory");
    let file = dir.join(name);
    fs::write(&file, format!("(eadl-version eadl/1)\n{text}\n")).expect("written");
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let status = run(
        ["check".to_string(), file.display().to_string()],
        &mut out,
        &mut err,
    );
    (
        status,
        format!(
            "{}{}",
            String::from_utf8_lossy(&out),
            String::from_utf8_lossy(&err)
        ),
    )
}

const OUTSIDE: &str = "
(defblock accel (offers sha-accel (reachable-at-privilege machine)))
(defplatform soc.here (offers (core-count 1 tick) (reachable-at-privilege machine)))
(defservice hash (requires (needs sha-accel)))
(defsystem s (requires (uses hash)) (platform (uses soc.here)))";

#[test]
fn the_command_refuses_a_need_only_a_block_outside_the_hardware_answers() {
    let (status, said) = checked("outside.eadl", OUTSIDE);
    assert_eq!(status, Status::MissingFact, "{said}");
    assert!(
        said.contains(
            "`sha-accel` is required by this system and nothing of its hardware describes it"
        ) && said.contains("`accel` offers it"),
        "{said}"
    );
}

#[test]
fn the_command_accepts_it_where_the_platform_composes_the_block() {
    let composed = OUTSIDE.replace(
        "(reachable-at-privilege machine)))\n(defservice",
        "(reachable-at-privilege machine)) (requires (uses accel)))\n(defservice",
    );
    assert_ne!(composed, OUTSIDE);
    let (status, said) = checked("composed.eadl", &composed);
    assert_eq!(status, Status::Ok, "{said}");
}
