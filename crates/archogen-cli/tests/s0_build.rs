//! Leaf `S0.3`: `archogen build` writes a generated crate, and that crate compiles.
//!
//! This is the emitter's own acceptance, and it stops one step short of F28 on purpose:
//! **compiling** is `S0.3`, **running and comparing against the frozen observation** is `S0.4`.
//! Keeping them apart matters because they fail for different reasons — a crate that does not
//! compile is a generator bug, while a crate that compiles and prints the wrong thing is a
//! realization bug, and a single test that did both would report them identically.
//!
//! ⛔ **Nothing here reads the generated Rust to check what it says.** That is the oracle's job
//! (`s0_oracle.rs`) and the oracle does it through the *observable* — the bytes the program
//! prints — never through the emitted table. A test that asserted "the generated file contains
//! `HORIZON_MS = 30`" would be an assertion about the generator's internals written in the
//! generator's own vocabulary, and it would keep passing through any change that kept the
//! spelling and broke the meaning.
//!
//! Everything is written under `CARGO_TARGET_TMPDIR` — inside `target/`, on the repository's own
//! volume and derived from it, never `/tmp`.

use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_cli::{run, Status};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn description(relative: &str) -> String {
    repo_root().join(relative).display().to_string()
}

/// A fresh output directory for one case. Removed first, so every build starts clean —
/// §12 S0's exit gate is written "from a clean local build directory".
fn out_dir(case: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("s0-build")
        .join(case);
    let _ = std::fs::remove_dir_all(&dir);
    dir
}

fn build(args: &[&str]) -> (Status, String, String) {
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(args.iter().map(|s| (*s).to_string()), &mut out, &mut err);
    (
        status,
        String::from_utf8(out).expect("utf-8 stdout"),
        String::from_utf8(err).expect("utf-8 stderr"),
    )
}

#[test]
fn the_base_fixture_generates_a_crate() {
    let dir = out_dir("base");
    let (status, out, err) = build(&[
        "build",
        &description("examples/s0-heartbeat/system.eadl"),
        "--out",
        &dir.display().to_string(),
    ]);
    assert_eq!(status, Status::Ok, "{err}");
    for file in [
        "Cargo.toml",
        "src/main.rs",
        "src/rt.rs",
        "src/service.rs",
        "provenance.json",
    ] {
        assert!(dir.join(file).is_file(), "{file} was not written");
    }
    assert!(out.contains("experimental"), "{out}");
}

#[test]
#[cfg_attr(
    miri,
    ignore = "runs cargo as a child process, which Miri does not support"
)]
fn the_generated_crate_compiles() {
    // `S0.3`'s acceptance, in one line: *"`archogen build` on the fixture writes a generated
    // crate that compiles."*
    let dir = out_dir("compiles");
    let (status, _, err) = build(&[
        "build",
        &description("examples/s0-heartbeat/system.eadl"),
        "--out",
        &dir.display().to_string(),
    ]);
    assert_eq!(status, Status::Ok, "{err}");

    // `env!`, not `option_env!(..).expect(..)`: an absent `CARGO` then fails the *build* of this
    // test rather than its run. §14.3 requires an unavailable tool to be reported as unavailable
    // and never as a passed check, and a compile error is the loudest form of that.
    let output = Command::new(env!("CARGO"))
        .arg("build")
        .arg("--manifest-path")
        .arg(dir.join("Cargo.toml"))
        // Its own target directory: the generated crate must not share, or contend for the
        // lock on, the workspace's.
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .output()
        .expect("cargo is runnable");
    assert!(
        output.status.success(),
        "the generated crate did not compile:\n{}",
        String::from_utf8_lossy(&output.stderr)
    );
}

#[test]
fn the_unsupported_fixture_writes_nothing_at_all() {
    // ⭐ A refusal that had already created a directory would leave a half-built artifact behind
    // for someone to find and mistake for output. Nothing is written before the plan exists.
    let dir = out_dir("unsupported");
    let (status, out, err) = build(&[
        "build",
        &description("examples/s0-heartbeat/system-unsupported.eadl"),
        "--out",
        &dir.display().to_string(),
    ]);
    assert_eq!(status, Status::UnsupportedProfile);
    assert_eq!(status.code(), 12);
    assert!(!dir.exists(), "a refused build created {}", dir.display());
    assert!(
        out.is_empty(),
        "a refused build must print nothing to stdout: {out}"
    );
    assert!(err.contains("min-separation"), "{err}");
    assert!(
        err.contains("M4.3"),
        "the refusal must name the leaf that supplies it: {err}"
    );
    assert!(err.contains("nothing was generated"), "{err}");
}

#[test]
fn generation_is_reproducible_from_a_clean_directory() {
    // §10.3: "Reproducibility means identical semantic plans and deterministic generated sources
    // from locked inputs." Two builds of one description, each from a removed directory, must
    // produce identical bytes — otherwise no later artifact hash means anything.
    let first = out_dir("repro-1");
    let second = out_dir("repro-2");
    let source = description("examples/s0-heartbeat/system.eadl");
    for dir in [&first, &second] {
        let (status, _, err) = build(&["build", &source, "--out", &dir.display().to_string()]);
        assert_eq!(status, Status::Ok, "{err}");
    }
    for file in [
        "Cargo.toml",
        "src/main.rs",
        "src/rt.rs",
        "src/service.rs",
        "provenance.json",
    ] {
        assert_eq!(
            std::fs::read(first.join(file)).expect("written"),
            std::fs::read(second.join(file)).expect("written"),
            "{file} differs between two builds of the same description"
        );
    }
}

#[test]
fn a_description_that_does_not_check_does_not_build() {
    // There is one frontend. `build` runs the same `eadl_model::check` as `check`, and reports
    // *its* verdict — so `build` can never accept something `check` rejects.
    let dir = out_dir("refused");
    let (status, _, err) = build(&[
        "build",
        &description("examples/bounded-queue/system.eadl"),
        "--out",
        &dir.display().to_string(),
    ]);
    assert_eq!(
        status,
        Status::UnsupportedProfile,
        "uc4 must stay refused: {err}"
    );
    assert!(!dir.exists(), "a refused build created {}", dir.display());
}

#[test]
fn locked_is_refused_rather_than_accepted_and_ignored() {
    // ⛔ The S0 path emits no lock data, so it cannot honor `--locked` — and a build that took
    // the flag silently would be an unlocked build wearing a locked build's label. §10.3 makes
    // lock data a build output; leaf `M4.1` owns producing it.
    let dir = out_dir("locked");
    let (status, out, err) = build(&[
        "build",
        &description("examples/s0-heartbeat/system.eadl"),
        "--out",
        &dir.display().to_string(),
        "--locked",
    ]);
    assert_eq!(status, Status::Unimplemented);
    assert_eq!(status.code(), 20);
    assert!(!dir.exists(), "a refused build created {}", dir.display());
    assert!(out.is_empty(), "{out}");
    assert!(err.contains("M4.1"), "{err}");
}

#[test]
fn a_description_with_no_system_says_there_is_nothing_to_build() {
    let dir = out_dir("no-system");
    let tmp = Path::new(env!("CARGO_TARGET_TMPDIR"));
    // ⛔ Cargo creates `CARGO_TARGET_TMPDIR` when it BUILDS this test binary, not when it runs it, so
    // a cached binary plus a cleaned `target/tmp` makes the write below panic. Every other site in
    // this suite either passes the path to the CLI as `--out` or calls `create_dir_all` first; this
    // was the only one writing into the root. Measured 2026-09-27: the sole cold-run failure.
    std::fs::create_dir_all(tmp).expect("the target tmpdir is creatable");
    let source = tmp.join("s0-build-library.eadl");
    std::fs::write(
        &source,
        "(defblock console.uart (offers (observable-output true)))\n",
    )
    .expect("writable");
    let (status, _, err) = build(&[
        "build",
        &source.display().to_string(),
        "--out",
        &dir.display().to_string(),
    ]);
    assert_eq!(status, Status::InvalidDescription);
    assert!(err.contains("declares no system"), "{err}");
    assert!(
        err.contains("library, not a system"),
        "the refusal should say what the file is, not only what it is not: {err}"
    );
}

/// ⛔ Leaf `M1.28.1`: the refusal an author sees is a statement about the **description**, and its exit
/// code is the one §5.5 gives a malformed description.
///
/// This is the end-to-end arm for the rule `Verdict::of_code` now carries in one place. The fixture is
/// the committed S0 description with one symbol changed — `(period 10 ms)` → `(period 10 parsec)` — so
/// `interpret` refuses it with `quantity-unknown-unit`, a code that is **not** a §5.5 verdict slug.
/// `build_cmd.rs` used to classify that as `ToolFailure` and exit **70**, which
/// `crates/archogen-cli/src/status.rs:8` reserves for a failure of the invocation, so an author who
/// mistyped a unit was told the toolchain had broken.
///
/// ⚠️ **What this arm does and does not pin.** It pins the *classification*, not the reachability of the
/// diagnostic: `archogen check` still accepts these bytes, because three consumers of `Quantity::read`
/// discard what it finds, and that is leaf `M1.28.2`. When `.2` lands, `check` refuses the description
/// first and this build stops reaching `interpret` — the assertion is `Status::InvalidDescription` either
/// way, which is why it is written against the status and not against the path that produced it.
#[test]
fn a_build_refused_for_a_bad_unit_is_a_malformed_description_not_a_tool_failure() {
    let dir = out_dir("bad-unit");
    let tmp = Path::new(env!("CARGO_TARGET_TMPDIR"));
    std::fs::create_dir_all(tmp).expect("the target tmpdir is creatable");

    let committed = description("examples/s0-heartbeat/system.eadl");
    let text = std::fs::read_to_string(&committed)
        .unwrap_or_else(|e| panic!("cannot read {committed}: {e}"));
    let corrupted = text.replacen("(period 10 ms)", "(period 10 parsec)", 1);
    assert_ne!(
        corrupted, text,
        "the fixture no longer holds `(period 10 ms)`, so this arm mutated nothing"
    );
    let source = tmp.join("s0-build-bad-unit.eadl");
    std::fs::write(&source, &corrupted).expect("writable");

    let (status, out, err) = build(&[
        "build",
        &source.display().to_string(),
        "--out",
        &dir.display().to_string(),
    ]);

    assert!(
        err.contains("quantity-unknown-unit"),
        "the refusal must name the unit the table does not hold: {err}"
    );
    assert_eq!(
        status,
        Status::InvalidDescription,
        "a description the S0 path cannot realize is malformed, not a toolchain failure: {err}"
    );
    assert_eq!(status.code(), 10, "the contract `status.rs` publishes");
    assert_ne!(
        status,
        Status::ToolFailure,
        "exit 70 is reserved for the invocation; reporting it here tells the author to file a bug \
         about the tool for a symbol they mistyped: {err}"
    );
    assert!(
        out.is_empty(),
        "a refused build must not also report success: {out}"
    );
}
