//! The repository's tiered verification runner (`ROADMAP.md` §4.2 `xtask`, §14.3).
//!
//! ```console
//! $ cargo xtask verify --tier focused
//! $ cargo xtask verify --list
//! ```
//!
//! # The rule that shapes everything here
//!
//! §14.3 ends with one sentence that is easy to agree with and hard to implement:
//!
//! > **A required tool skipped or unavailable is reported as such, not a passed check.**
//!
//! A runner that silently drops a step it cannot run turns an *absence of evidence* into an
//! *appearance of evidence*, and it does so exactly when the missing step is the one that
//! mattered. So a tier has three outcomes here, not two:
//!
//! | Verdict | Exit | Means |
//! |---|---|---|
//! | `passed` | `0` | every step ran and succeeded |
//! | `failed` | `1` | a step ran and failed |
//! | `incomplete` | `20` | nothing failed, and something could not be run |
//!
//! `incomplete` is the interesting one. It is **not** a pass, and it is not a failure either —
//! nothing is broken, the evidence is simply not there. Today that is the honest verdict for four
//! of the five tiers, and the runner names what is missing and who owns supplying it, so the gap
//! is a routed item rather than a silence.
//!
//! # Two kinds of "cannot run"
//!
//! They are different and are kept apart, because the response differs:
//!
//! * **unavailable** — the step exists, and a *tool* is missing from this machine. Install it.
//! * **not built** — the step does not exist yet. It names the task-tree leaf that owns building
//!   it, so a reader learns where the work is tracked instead of concluding the project forgot.
//!
//! And a third, which §14.3 names in the same paragraph: a **quarantine** — the step exists, its
//! tool is present, it ran, and what it needs to reach a verdict is owned by a leaf that has not
//! delivered it. §14.3: "Quarantine requires a named issue, owner, affected claim, and bounded
//! scope", so a quarantine is a row of [`QUARANTINES`] carrying exactly those, and nothing else
//! can make a step's exit count as an absence (leaf `PROGRAM.10.1`).

mod catalog_build;
mod catalog_check;
mod dtb;
mod elf;
mod generated_header;
mod json;
mod mutation;
mod premises;
mod target;
mod trust;
mod trust_gate;
mod trust_verify;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

/// How many trailing lines of each stream of a failing step's output to show. Enough to carry a
/// test failure's assertion diff and the doctrine driver's whole report — **32** lines of stdout
/// with one breach, measured `2026-09-30` — short enough that a tier's report stays readable.
const TAIL_LINES: usize = 40;

/// The exit code by which a step says *I could not be run* rather than *I ran and disagreed* —
/// the same `20` the tiers and the checker's own contract use for `incomplete`.
const COULD_NOT_RUN: i32 = 20;

/// §14.3: "Quarantine requires a named issue, owner, affected claim, and bounded scope."
///
/// ⛔ What a quarantine buys, and nothing more: the named step may exit [`COULD_NOT_RUN`], and the
/// runner counts that as an absence — the tier `incomplete`, never `passed`. The same step exiting
/// `1` is still a failure; a step no row names exiting `20` is still a failure; and the named step
/// *passing* is refused as a stale quarantine, so the leaf that closes the gap deletes the row in
/// the same commit instead of leaving an exemption nobody needs to notice.
struct Quarantine {
    /// The bounded scope: this one step, and only its exit `20`.
    step: &'static str,
    /// What cannot be run, and why.
    issue: &'static str,
    /// The task-tree leaf that lifts it.
    owner: &'static str,
    /// The claim that stays unproven while it stands.
    claim: &'static str,
}

/// The runtime core's Rust target: a `target-env:<file>:<key>` requirement, resolved from the pinned target's `.env`
/// when the step runs, so the triple is declared once, there (leaf `M2.8.3.3`).
const RUST_TARGET_REQUIREMENT: &str = "target-env:targets/riscv-virt-up.env:RUST_TARGET";

/// A step's requirement as the checks below read it: a `target-env:<file>:<key>` form becomes `target:<value>`, the
/// value read from `<file>` under `root`. Anything else is returned as it is.
fn resolve_requirement(root: &Path, requirement: &str) -> Result<String, String> {
    let Some((file, key)) = requirement
        .strip_prefix("target-env:")
        .and_then(|rest| rest.split_once(':'))
    else {
        return Ok(requirement.to_string());
    };
    let text = std::fs::read_to_string(root.join(file)).map_err(|e| format!("{file}: {e}"))?;
    target::env(&text)
        .get(key)
        .map(|value| format!("target:{value}"))
        .ok_or_else(|| format!("{file} declares no {key}"))
}

/// Every quarantine in the repository, so "what is quarantined right now?" has one answer.
const QUARANTINES: &[Quarantine] = &[];

/// One step of a tier.
struct Step {
    /// Short name, shown in the report.
    name: &'static str,
    /// What passing it establishes. Kept next to the step so a tier reads as an argument rather
    /// than a list of commands.
    proves: &'static str,
    /// What running it takes.
    action: Action,
}

enum Action {
    /// Run a command from the repository root; a non-zero exit fails the step.
    Run {
        program: &'static str,
        args: &'static [&'static str],
        /// A tool that must be on `PATH` first. Absent → the step is **unavailable**, never a
        /// pass.
        requires: Option<&'static str>,
        /// Why its absence matters, shown with the unavailable report.
        matters: &'static str,
    },
    /// The step does not exist yet. Names the leaf that owns building it.
    NotBuilt {
        owner: &'static str,
        note: &'static str,
    },
    /// A gate whose pass is not yet the step's: run from the repository root, any non-zero exit **fails** the step —
    /// a refusal, or a gate that could not judge — and exit 0 leaves it **not built**, for what `owner` still has to
    /// supply. So the step is never Passed before that leaf lands (`M3.6.3.4`, ledger `TI-H7`).
    Gate {
        program: &'static str,
        args: &'static [&'static str],
        owner: &'static str,
        note: &'static str,
    },
}

/// What a [`Action::Gate`] step concluded from its gate's exit code: never Passed.
fn gate_outcome(code: Option<i32>) -> Outcome {
    match code {
        Some(0) => Outcome::NotBuilt,
        _ => Outcome::Failed,
    }
}

/// One §14.3 tier.
struct Tier {
    name: &'static str,
    when: &'static str,
    steps: &'static [Step],
}

/// A step's result.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Outcome {
    Passed,
    Failed,
    Unavailable,
    NotBuilt,
    /// Ran, exited [`COULD_NOT_RUN`], and a row of [`QUARANTINES`] names it.
    Quarantined,
}

/// How a finished step's exit is read — pure, so the table below is tested rather than trusted.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Judgement {
    Passed,
    /// Passed while a quarantine still names it: the gap is closed and the exemption is not.
    Stale,
    Quarantined,
    /// Exited `20` with no quarantine to say who owns the gap — a failure, with a hint.
    Undeclared,
    Failed,
}

/// `code` is the exit code, `None` when the process was ended by a signal.
fn judge(code: Option<i32>, quarantined: bool) -> Judgement {
    match (code, quarantined) {
        (Some(0), false) => Judgement::Passed,
        (Some(0), true) => Judgement::Stale,
        (Some(COULD_NOT_RUN), true) => Judgement::Quarantined,
        (Some(COULD_NOT_RUN), false) => Judgement::Undeclared,
        _ => Judgement::Failed,
    }
}

/// A tier's verdict, derived from its steps.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Verdict {
    Passed,
    Failed,
    Incomplete,
}

impl Verdict {
    /// ⭐ The whole point of this module, in four lines. A failure outranks an absence, because a
    /// broken thing is more urgent than a missing measurement — but an absence NEVER becomes a
    /// pass, however many steps around it succeeded.
    fn of(outcomes: &[Outcome]) -> Self {
        if outcomes.contains(&Outcome::Failed) {
            Self::Failed
        } else if outcomes.iter().any(|o| {
            matches!(
                o,
                Outcome::Unavailable | Outcome::NotBuilt | Outcome::Quarantined
            )
        }) {
            Self::Incomplete
        } else {
            Self::Passed
        }
    }

    const fn code(self) -> i32 {
        match self {
            Self::Passed => 0,
            Self::Failed => 1,
            Self::Incomplete => 20,
        }
    }

    const fn slug(self) -> &'static str {
        match self {
            Self::Passed => "passed",
            Self::Failed => "failed",
            Self::Incomplete => "incomplete",
        }
    }
}

const FMT: Step = Step {
    name: "fmt",
    proves: "every Rust source is in canonical format",
    action: Action::Run {
        program: "cargo",
        args: &["fmt", "--all", "--", "--check"],
        requires: None,
        matters: "",
    },
};

const CLIPPY: Step = Step {
    name: "clippy",
    proves: "no lint fires anywhere, including in tests and examples",
    action: Action::Run {
        program: "cargo",
        args: &[
            "clippy",
            "--all-targets",
            "--all-features",
            "--",
            "-D",
            "warnings",
        ],
        requires: None,
        matters: "",
    },
};

const TESTS: Step = Step {
    name: "tests",
    proves: "every contract test passes, F28 and the semantic corpus included",
    action: Action::Run {
        program: "cargo",
        args: &["test", "--all"],
        requires: None,
        matters: "",
    },
};

const DOCTRINES: Step = Step {
    name: "doctrines",
    proves: "every repository invariant holds on the working tree",
    action: Action::Run {
        program: "scripts/check_doctrines.sh",
        args: &[],
        requires: None,
        matters: "",
    },
};

/// Every gate's own RED arms, discovered by census (leaf `PROGRAM.28`). A doctrine gate proves the tree is
/// clean; its `--self-test` proves the gate can still fail — and until this step nothing re-ran those arms.
/// ~40 s, which is why it is here and not on the pre-commit path.
const SELF_TESTS: Step = Step {
    name: "self-tests",
    proves: "every doctrine gate's RED arms still fire — a gate that stopped being able to fail is caught here",
    action: Action::Run {
        program: "scripts/run_self_tests.sh",
        args: &[],
        requires: None,
        matters: "",
    },
};

/// The §14.3 tiers, as data.
///
/// ⚠️ **`focused` runs the whole test suite, and that is a measured decision rather than a
/// reading of §14.3.** The tier is defined as "format/type checks and *affected* contract
/// tests", and selecting affected tests needs change-impact machinery. Measured on this tree:
/// `cargo test --all` takes **2.6 s** warm, and the three focused steps together **2.9 s**. At
/// that size the machinery would cost more than it saves and would be one more thing to be wrong.
/// Revisit when the suite stops fitting in an edit loop — the number above is what to compare
/// against, which is why it is written down.
const TIERS: &[Tier] = &[
    Tier {
        name: "focused",
        when: "each edit loop, and every ordinary commit",
        steps: &[FMT, CLIPPY, TESTS],
    },
    Tier {
        name: "integration",
        when: "before a push, and before closing a milestone",
        steps: &[
            FMT,
            CLIPPY,
            TESTS,
            DOCTRINES,
            SELF_TESTS,
            Step {
                name: "book",
                proves: "the mdBook builds, with its pinned release — it is the director's window, so a broken book is a broken deliverable",
                action: Action::Run {
                    program: "scripts/build_book.sh",
                    args: &[],
                    requires: Some("mdbook"),
                    matters: "the book is the project's public surface and is required to stay in \
                              lockstep with the code; `cargo install mdbook`",
                },
            },
            Step {
                name: "no-std-build",
                proves: "the runtime core compiles for a bare-metal target (§14.3's \"compile targets\")",
                action: Action::Run {
                    // The triple is the pinned target's, declared once in its `.env` (leaf `M2.8.3.3`).
                    program: "scripts/no_std_build.sh",
                    args: &[],
                    requires: Some(RUST_TARGET_REQUIREMENT),
                    matters: "§3.1 fixes the runtime as a \"Rust no_std core\", and `#![no_std]` \
                              being active in a host build is evidence that it *can* be, not that \
                              it *does* build for a target. `rustup target add` the `RUST_TARGET` of \
                              `targets/riscv-virt-up.env`",
                },
            },
            Step {
                name: "pin-premises",
                proves: "every premise the catalog rests on about the pinned toolchain still holds: the port record's, \
                         each probe it prints read at its hash (§14.1–§14.4), and the gate's three claims (§3)",
                action: Action::Run {
                    program: "cargo",
                    args: &["xtask", "pin-premises"],
                    requires: Some(RUST_TARGET_REQUIREMENT),
                    matters: "the port's assembly format and the catalog's gate rest on what the pinned compiler, \
                              assembler, linker, Cargo and rustup do; a pin bump that changes one must fail here \
                              before any record rests on it (leaves `M2.12.4.4`, `M2.7.4.1`). `rustup target add` \
                              the `RUST_TARGET` of `targets/riscv-virt-up.env`",
                },
            },
            Step {
                name: "wasm-build",
                proves: "the engine's I/O-free crates, derived from the workspace, compile for the browser target",
                action: Action::Run {
                    program: "scripts/wasm_build.sh",
                    args: &[],
                    requires: Some("target:wasm32-unknown-unknown"),
                    matters: "the programmatic-interface decision promises a wasm binding, and whether the \
                              engine compiles for the browser is measured here rather than assumed \
                              (leaf `API.1`). `rust-toolchain.toml` lists the target, so `rustup \
                              toolchain install` adds it",
                },
            },
            Step {
                name: "wasm-binding",
                proves: "the browser artifact imports nothing, exports what its record lists, and answers every tracked \
                         description byte for byte as the host build does, with archogen check's exit code",
                action: Action::Run {
                    program: "scripts/wasm_binding.sh",
                    args: &[],
                    requires: Some("node"),
                    matters: "the wasm binding (leaf `API.5.3`) is checked on the artifact a page loads, through the \
                              loader a page uses, and a JavaScript runtime is what runs it. Without one, nothing \
                              shows that the compiled module agrees with the host build or the command line",
                },
            },
            Step {
                name: "emulator",
                proves: "the pinned riscv-virt-up configuration renders and its toolchain is present (§3.2)",
                action: Action::Run {
                    program: "scripts/target_emulator.sh",
                    args: &["--check"],
                    requires: Some("qemu-system-riscv64"),
                    matters: "§14.3 puts \"selected emulator runs\" in this tier. Without QEMU \
                              there is no independent execution of a target binary at all — see \
                              docs/targets/first-target.md and finding 2 in \
                              docs/decisions/decision_findings-for-director-review.md",
                },
            },
            Step {
                name: "spike",
                proves: "code runs on the verified target: boot, a timer interrupt taken and returned from, the \
                         context preserved, output on the UART — and a clobbered context is caught (M2.8.4)",
                action: Action::Run {
                    program: "scripts/target_spike.sh",
                    args: &[],
                    requires: Some("qemu-system-riscv64"),
                    matters: "M2.8's architecture spike: the first binary run on `riscv-virt-up`, the \
                              mechanisms the runtime rests on exercised on the emulated hardware, with a \
                              negative control built in so the context check is shown able to fail",
                },
            },
        ],
    },
    Tier {
        name: "extended",
        when: "scheduled, or when a change touches parsing, arithmetic or event ordering",
        steps: &[
            Step {
                name: "fuzz",
                proves: "the reader and the exact arithmetic hold eight properties on a fixed and a fresh seed, after six known-false arms are refuted (§13.3)",
                action: Action::Run {
                    program: "scripts/extended_fuzz.sh",
                    args: &[],
                    requires: None,
                    matters: "",
                },
            },
            Step {
                name: "mutation",
                proves: "every catalogued defect is still caught by the test written for it, and the recorded blind spot still survives the corpus that has it",
                action: Action::Run {
                    program: "cargo",
                    args: &["xtask", "mutate"],
                    requires: None,
                    matters: "",
                },
            },
            Step {
                name: "miri",
                proves: "Miri still refuses a seeded dangling-pointer read, and every crate it can run is free of the UB it detects",
                action: Action::Run {
                    program: "scripts/extended_miri.sh",
                    args: &[],
                    requires: Some("component:nightly:miri"),
                    matters: "§13.3 selects Miri for the hosted Rust logic, and records that \
                              passing it does not establish soundness; `rustup +nightly component \
                              add miri`",
                },
            },
        ],
    },
    Tier {
        name: "hardware",
        when: "a change to target support, and every release gate",
        steps: &[Step {
            name: "board",
            proves: "the generated system runs on the named physical target (§12 M5)",
            action: Action::NotBuilt {
                owner: "M5.1",
                note: "NO BOARD HAS BEEN PROCURED. All seven facts §3.2 requires are recorded as \
                       `unrecorded` in docs/targets/first-target.md. This tier cannot pass, and \
                       §12 M5 forbids claiming board support without it — which is exactly why \
                       it is reported rather than omitted",
            },
        }],
    },
    Tier {
        name: "assurance",
        when: "every supported release",
        steps: &[
            Step {
                name: "trust-inventory",
                proves: "no undeclared dependency is shared between generator and checker (F30, §4.4)",
                action: Action::Gate {
                    program: "cargo",
                    args: &["xtask", "trust-gate"],
                    owner: "M3.6.5",
                    note: "the trust gate refused nothing; what two roots share is established only once \
                           every form is accepted, which M3.6.5 reads where reviews are protected",
                },
            },
            Step {
                name: "claim-completeness",
                proves: "no report renders while a named property is unanswered (§7.1)",
                action: Action::NotBuilt {
                    owner: "M4.8",
                    note: "the claim vocabulary exists and is tested (crates/archogen-evidence), \
                           but nothing yet generates a report for it to govern",
                },
            },
            Step {
                name: "identity",
                proves: "source and binary identities match the artifacts they describe (§7.5)",
                action: Action::NotBuilt {
                    owner: "M4.7",
                    note: "no plan hash, binary hash or replay manifest is produced yet",
                },
            },
        ],
    },
];

fn tier(name: &str) -> Option<&'static Tier> {
    TIERS.iter().find(|tier| tier.name == name)
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .expect("xtask/ sits at the repository root")
        .to_path_buf()
}

fn on_path(tool: &str) -> bool {
    // A rustup TARGET is not an executable and will never be on `PATH`; ask rustup which are
    // installed. Without this, a step that needs a cross-compilation target would look like a
    // missing program, and the fix a reader tried would be the wrong one.
    if let Some(triple) = tool.strip_prefix("target:") {
        return Command::new("rustup")
            .args(["target", "list", "--installed"])
            .output()
            .is_ok_and(|out| {
                String::from_utf8_lossy(&out.stdout)
                    .lines()
                    .any(|line| line.trim() == triple)
            });
    }
    // A rustup COMPONENT of a named toolchain — `component:nightly:miri`. Miri ships with `nightly`, and this
    // repository pins `stable`, so asking plain `cargo miri` asks the wrong toolchain: measured, the step
    // reported "not on PATH" on a machine where `cargo +nightly miri --version` answered (leaf `PROGRAM.9.1`).
    if let Some((toolchain, component)) = tool
        .strip_prefix("component:")
        .and_then(|rest| rest.split_once(':'))
    {
        return Command::new("rustup")
            .args([&format!("+{toolchain}"), "component", "list", "--installed"])
            .output()
            .is_ok_and(|out| {
                String::from_utf8_lossy(&out.stdout).lines().any(|line| {
                    line.trim().starts_with(&format!("{component}-")) || line.trim() == component
                })
            });
    }
    // `cargo miri` is a cargo subcommand, not a bare executable; ask cargo for it.
    if let Some(sub) = tool.strip_prefix("cargo-") {
        return Command::new("cargo")
            .args([sub, "--version"])
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .is_ok_and(|status| status.success());
    }
    std::env::var_os("PATH")
        .is_some_and(|paths| std::env::split_paths(&paths).any(|dir| dir.join(tool).is_file()))
}

/// What a missing tool means. Where the environment has claimed to supply every tool the tier
/// requires (`--provisioned`, which CI passes), a missing one is that claim failing — a broken
/// provisioning, not an absence the tier may report as `incomplete`
/// (`docs/decisions/decision_incomplete-blocking-policy.md`, leaf `PROGRAM.10.3`).
const fn missing_tool(provisioned: bool) -> Outcome {
    if provisioned {
        Outcome::Failed
    } else {
        Outcome::Unavailable
    }
}

fn run_step(step: &Step, root: &Path, provisioned: bool) -> Outcome {
    match &step.action {
        Action::NotBuilt { owner, note } => {
            println!("  ⚠  {:<18} NOT BUILT — tracked by leaf {owner}", step.name);
            println!("     {note}");
            Outcome::NotBuilt
        }
        Action::Gate {
            program,
            args,
            owner,
            note,
        } => {
            let started = Instant::now();
            let output = Command::new(program)
                .args(*args)
                .current_dir(root)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();
            let elapsed = started.elapsed().as_secs_f64();
            match output {
                Ok(output) => {
                    let outcome = gate_outcome(output.status.code());
                    if outcome == Outcome::NotBuilt {
                        println!(
                            "  ⚠  {:<18} {elapsed:>6.2}s  NOT BUILT — its gate passed; tracked by leaf {owner}",
                            step.name
                        );
                        println!("     {note}");
                    } else {
                        println!("  ❌ {:<18} {elapsed:>6.2}s  FAILED", step.name);
                        print_tail(&output);
                        println!("     re-run it directly: {program} {}", args.join(" "));
                    }
                    outcome
                }
                Err(error) => {
                    println!("  ❌ {:<18} could not run `{program}`: {error}", step.name);
                    Outcome::Failed
                }
            }
        }
        Action::Run {
            program,
            args,
            requires,
            matters,
        } => {
            if let Some(requirement) = requires {
                let tool = match resolve_requirement(root, requirement) {
                    Ok(tool) => tool,
                    Err(reason) => {
                        println!(
                            "  ❌ {:<18} FAILED — its requirement cannot be read: {reason}",
                            step.name
                        );
                        return Outcome::Failed;
                    }
                };
                let tool = tool.as_str();
                if !on_path(tool) {
                    // A rustup target is not an executable, and telling a reader it is "not on
                    // PATH" sends them to fix the wrong thing.
                    let missing = if let Some(triple) = tool.strip_prefix("target:") {
                        format!("the `{triple}` target is not installed")
                    } else if let Some((toolchain, component)) = tool
                        .strip_prefix("component:")
                        .and_then(|rest| rest.split_once(':'))
                    {
                        format!("the `{component}` component is not installed for the `{toolchain}` toolchain")
                    } else {
                        format!("`{tool}` is not on PATH")
                    };
                    let outcome = missing_tool(provisioned);
                    if outcome == Outcome::Failed {
                        println!(
                            "  ❌ {:<18} UNAVAILABLE on a provisioned environment — {missing}",
                            step.name
                        );
                        println!(
                            "     --provisioned says this environment supplies every tool the tier \
                             needs, so this is its provisioning failing, not an absence"
                        );
                    } else {
                        println!("  ⚠  {:<18} UNAVAILABLE — {missing}", step.name);
                    }
                    println!("     {matters}");
                    return outcome;
                }
            }
            let started = Instant::now();
            // ⛔ BOTH streams are captured, and this was measured rather than anticipated: the
            // first run of this runner reported `fmt FAILED` with no reason at all, because
            // `cargo fmt --check` writes its diff to STDOUT while the runner only kept stderr. A
            // failing step that cannot say why is a step a developer learns to re-run by hand,
            // which is the same as not having a runner.
            let status = Command::new(program)
                .args(*args)
                .current_dir(root)
                .stdout(Stdio::piped())
                .stderr(Stdio::piped())
                .output();
            let elapsed = started.elapsed().as_secs_f64();
            let quarantine = QUARANTINES.iter().find(|q| q.step == step.name);
            match status {
                Ok(output) => match judge(output.status.code(), quarantine.is_some()) {
                    Judgement::Passed => {
                        println!("  ✅ {:<18} {elapsed:>6.2}s  {}", step.name, step.proves);
                        Outcome::Passed
                    }
                    Judgement::Quarantined => {
                        let q =
                            quarantine.expect("judged quarantined only when a row names the step");
                        println!(
                            "  ⚠  {:<18} {elapsed:>6.2}s  QUARANTINED — could not be run; leaf {} owns the gap",
                            step.name, q.owner
                        );
                        println!("     issue: {}", q.issue);
                        println!("     unproven while it stands: {}", q.claim);
                        print_tail(&output);
                        Outcome::Quarantined
                    }
                    Judgement::Stale => {
                        let q = quarantine.expect("judged stale only when a row names the step");
                        println!("  ❌ {:<18} {elapsed:>6.2}s  STALE QUARANTINE", step.name);
                        println!(
                            "     the step passed, and QUARANTINES still exempts it for leaf {} — the gap is \
                             closed, so delete its row in xtask/src/main.rs in the same commit",
                            q.owner
                        );
                        Outcome::Failed
                    }
                    judgement @ (Judgement::Undeclared | Judgement::Failed) => {
                        println!("  ❌ {:<18} {elapsed:>6.2}s  FAILED", step.name);
                        print_tail(&output);
                        if judgement == Judgement::Undeclared {
                            println!(
                                "     exit {COULD_NOT_RUN} says \"could not be run\", and no row of QUARANTINES \
                                 names this step — §14.3 needs an issue, owner, claim and scope before an \
                                 absence is anything but a failure"
                            );
                        }
                        println!("     re-run it directly: {program} {}", args.join(" "));
                        Outcome::Failed
                    }
                },
                Err(error) => {
                    // A program that will not start is not a passed check either.
                    println!("  ❌ {:<18} could not run `{program}`: {error}", step.name);
                    Outcome::Failed
                }
            }
        }
    }
}

/// The last [`TAIL_LINES`] lines of each stream a step wrote to — stdout first, then stderr.
///
/// ⛔ **Both**, measured rather than anticipated, a second time: choosing stderr whenever it was
/// non-empty left a failing `doctrines` step reporting `=== 1 doctrine breach(es) — commit
/// blocked ===` and nothing else, because the driver writes each breach to stdout and only its
/// summary to stderr (leaf `PROGRAM.10.1`). A step that cannot say why it failed is re-run by
/// hand, and in CI its log is the only view there is.
fn tail_lines(stdout: &str, stderr: &str) -> Vec<String> {
    let mut shown = Vec::new();
    for stream in [stdout, stderr] {
        let lines: Vec<&str> = stream.lines().collect();
        let omitted = lines.len().saturating_sub(TAIL_LINES);
        if omitted > 0 {
            shown.push(format!("… {omitted} earlier line(s) omitted"));
        }
        shown.extend(lines[omitted..].iter().map(|line| (*line).to_string()));
    }
    shown
}

fn print_tail(output: &std::process::Output) {
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);
    for line in tail_lines(&stdout, &stderr) {
        println!("     {line}");
    }
}

fn verify(name: &str, provisioned: bool) -> i32 {
    let Some(tier) = tier(name) else {
        eprintln!("xtask: unknown tier `{name}`");
        eprintln!("  hint: {}", tier_names());
        return 2;
    };
    let root = repo_root();
    println!("tier: {} — {}", tier.name, tier.when);
    if provisioned {
        println!("  (provisioned: a missing tool is a failure, not an absence)");
    }
    let outcomes: Vec<Outcome> = tier
        .steps
        .iter()
        .map(|step| run_step(step, &root, provisioned))
        .collect();

    let count = |want: Outcome| outcomes.iter().filter(|o| **o == want).count();
    let verdict = Verdict::of(&outcomes);
    println!(
        "tier {}: {} — {} passed, {} failed, {} unavailable, {} not built, {} quarantined",
        tier.name,
        verdict.slug(),
        count(Outcome::Passed),
        count(Outcome::Failed),
        count(Outcome::Unavailable),
        count(Outcome::NotBuilt),
        count(Outcome::Quarantined)
    );
    if verdict == Verdict::Incomplete {
        println!(
            "  ⚠  incomplete is NOT a pass. §14.3: \"a required tool skipped or unavailable is \
             reported as such, not a passed check\"."
        );
        if count(Outcome::Quarantined) > 0 {
            println!(
                "  ⚠  and a quarantine is an absence on terms. §14.3: \"Quarantine requires a named \
                 issue, owner, affected claim, and bounded scope\" — each is printed above."
            );
        }
    }
    verdict.code()
}

/// `cargo xtask dtb-summary <file.dtb>`: the device tree as the fixture records it (leaf `M2.8.2`).
fn dtb_summary(file: &str) -> i32 {
    match std::fs::read(file)
        .map_err(|e| e.to_string())
        .and_then(|b| dtb::parse(&b))
    {
        Ok(root) => {
            print!("{}", dtb::render(&root));
            0
        }
        Err(reason) => {
            eprintln!("xtask dtb-summary: {file}: {reason}");
            1
        }
    }
}

/// `cargo xtask target-agreement <target.env> <file.dtb>`: the target's eADL description against the device tree —
/// 0 agree, 1 disagree (leaf `M2.8.3.2`, `docs/decisions/decision_target-platform-description.md`).
fn target_agreement(env_file: &str, dtb_file: &str) -> i32 {
    let env = match std::fs::read_to_string(env_file) {
        Ok(text) => target::env(&text),
        Err(reason) => {
            eprintln!("xtask target-agreement: {env_file}: {reason}");
            return 1;
        }
    };
    let Some(description_file) = env.get("PLATFORM_DESCRIPTION").cloned() else {
        eprintln!("xtask target-agreement: {env_file} names no PLATFORM_DESCRIPTION");
        return 1;
    };
    // Relative to where the command runs, as the `.env`'s own paths are: the repository root for the tier step and
    // for a person, a scratch repository for `target_emulator.sh`'s self-test.
    let description = match std::fs::read_to_string(&description_file) {
        Ok(text) => text,
        Err(reason) => {
            eprintln!("xtask target-agreement: {description_file}: {reason}");
            return 1;
        }
    };
    let root = match std::fs::read(dtb_file)
        .map_err(|e| e.to_string())
        .and_then(|b| dtb::parse(&b))
    {
        Ok(root) => root,
        Err(reason) => {
            eprintln!("xtask target-agreement: {dtb_file}: {reason}");
            return 1;
        }
    };
    let wrong = target::disagreements(&env, &description, &description_file, &root);
    if wrong.is_empty() {
        println!("target-agreement: {description_file} agrees with the device tree in {dtb_file}");
        return 0;
    }
    eprintln!(
        "target-agreement: {description_file} disagrees with {dtb_file} in {} way(s):",
        wrong.len()
    );
    for line in &wrong {
        eprintln!("  {line}");
    }
    1
}

/// `cargo xtask dtb-check <file.dtb> <fixture.md>`: the dump against the fixture — 0 agree, 1 disagree.
fn dtb_check(file: &str, fixture: &str) -> i32 {
    let fresh = match std::fs::read(file)
        .map_err(|e| e.to_string())
        .and_then(|b| dtb::parse(&b))
    {
        Ok(root) => dtb::render(&root),
        Err(reason) => {
            eprintln!("xtask dtb-check: {file}: {reason}");
            return 1;
        }
    };
    let recorded = match std::fs::read_to_string(fixture) {
        Ok(text) => text,
        Err(reason) => {
            eprintln!("xtask dtb-check: {fixture}: {reason}");
            return 1;
        }
    };
    let wrong = dtb::disagreements(&fresh, &recorded);
    if wrong.is_empty() {
        println!("dtb-check: the platform presents exactly what {fixture} records");
        return 0;
    }
    eprintln!(
        "dtb-check: the platform disagrees with {fixture} in {} way(s):",
        wrong.len()
    );
    for line in &wrong {
        eprintln!("  {line}");
    }
    eprintln!(
        "  a QEMU that presents another platform under the pinned options is another target (§3.2)"
    );
    1
}

fn tier_names() -> String {
    TIERS
        .iter()
        .map(|tier| tier.name)
        .collect::<Vec<_>>()
        .join(" | ")
}

fn list() {
    for tier in TIERS {
        println!("{} — {}", tier.name, tier.when);
        for step in tier.steps {
            let shape = match &step.action {
                Action::NotBuilt { owner, .. } => format!("not built (leaf {owner})"),
                Action::Gate { owner, .. } => format!("a gate; not built (leaf {owner})"),
                Action::Run {
                    requires: Some(tool),
                    ..
                } => format!("needs `{tool}`"),
                Action::Run { .. } => "always runnable".to_string(),
            };
            println!("  {:<18} {:<26} {}", step.name, shape, step.proves);
            if let Some(q) = QUARANTINES.iter().find(|q| q.step == step.name) {
                println!(
                    "  {:<18} quarantined: exit {COULD_NOT_RUN} is an absence until leaf {}",
                    "", q.owner
                );
            }
        }
        println!();
    }
}

fn help() {
    println!("xtask — the archogen repository's tiered verification runner (ROADMAP.md §14.3)");
    println!();
    println!("USAGE:");
    println!("    cargo xtask verify --tier <TIER> [--provisioned]");
    println!("    cargo xtask verify --list");
    println!("    cargo xtask dtb-summary <file.dtb>");
    println!("    cargo xtask dtb-check <file.dtb> <fixture.md>");
    println!("    cargo xtask pin-premises");
    println!("    cargo xtask catalog-check --index [--bless] | --commit <sha> | --base <dir> --judged <dir> --base-commit <sha> --judged-commit <sha>");
    println!("    cargo xtask trust-inventory [--commit <rev>] [--out <dir>]");
    println!("    cargo xtask trust-baseline --propose [--commit <rev>] [--out <file>]");
    println!(
        "    cargo xtask trust-gate [--repo <dir>] [--commit <rev>] [--base <rev>] [--out <dir>]"
    );
    println!("    cargo xtask trust-verify <package>");
    println!();
    println!("TIERS:");
    for tier in TIERS {
        println!("    {:<14} {}", tier.name, tier.when);
    }
    println!();
    println!("EXIT CODES:");
    println!("     0  passed      every step ran and succeeded");
    println!("     1  failed      a step ran and failed");
    println!("     2  usage       the command line was malformed");
    println!("    20  incomplete  nothing failed, and something could not be run —");
    println!("                    NOT a pass (ROADMAP.md §14.3)");
}

fn main() {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let code = match args
        .iter()
        .map(String::as_str)
        .collect::<Vec<_>>()
        .as_slice()
    {
        [] | ["--help"] | ["-h"] | ["help"] => {
            help();
            0
        }
        ["verify", "--list"] => {
            list();
            0
        }
        ["verify", "--tier", name] => verify(name, false),
        ["verify", "--tier", name, "--provisioned"]
        | ["verify", "--provisioned", "--tier", name] => verify(name, true),
        ["dtb-summary", file] => dtb_summary(file),
        ["dtb-check", file, fixture] => dtb_check(file, fixture),
        ["target-agreement", env, dtb] => target_agreement(env, dtb),
        ["mutate"] => mutation::run(&repo_root(), &[]),
        ["pin-premises"] => premises::run(&repo_root()),
        ["catalog-check", rest @ ..] => catalog_check::run(&repo_root(), rest),
        ["trust-inventory", rest @ ..] => trust::run(&repo_root(), rest),
        ["trust-baseline", rest @ ..] => trust_gate::run(&repo_root(), rest),
        ["trust-gate", rest @ ..] => trust_gate::run_gate(&repo_root(), rest),
        ["trust-verify", rest @ ..] => trust_verify::run(rest),
        ["mutate", "--only", ids @ ..] if !ids.is_empty() => mutation::run(
            &repo_root(),
            &ids.iter().map(|s| (*s).to_string()).collect::<Vec<_>>(),
        ),
        other => {
            eprintln!("xtask: unrecognized arguments {other:?}");
            eprintln!("  hint: cargo xtask verify --tier <{}>", tier_names());
            2
        }
    };
    std::process::exit(code);
}

#[cfg(test)]
mod tests {
    use super::{
        gate_outcome, judge, missing_tool, tail_lines, tier, Action, Judgement, Outcome, Verdict,
        QUARANTINES, TAIL_LINES, TIERS,
    };

    /// The leaf a step routes its gap to, and why: a step not built, or a gate whose pass is not yet the step's.
    fn routed(action: &Action) -> Option<(&'static str, &'static str)> {
        match action {
            Action::NotBuilt { owner, note } | Action::Gate { owner, note, .. } => {
                Some((owner, note))
            }
            Action::Run { .. } => None,
        }
    }

    #[test]
    fn the_five_tiers_of_the_roadmap_are_the_five_tiers_here() {
        // §14.3 names exactly these. A tier invented here is scope drift; one missing is a gate
        // nobody can run.
        let names: Vec<&str> = TIERS.iter().map(|t| t.name).collect();
        assert_eq!(
            names,
            vec![
                "focused",
                "integration",
                "extended",
                "hardware",
                "assurance"
            ]
        );
        for name in &names {
            assert!(tier(name).is_some());
        }
    }

    #[test]
    fn an_absence_never_becomes_a_pass() {
        // ⭐ The rule §14.3 states and this runner exists to implement.
        assert_eq!(
            Verdict::of(&[Outcome::Passed, Outcome::Passed]),
            Verdict::Passed
        );
        assert_eq!(
            Verdict::of(&[Outcome::Passed, Outcome::Unavailable]),
            Verdict::Incomplete
        );
        assert_eq!(
            Verdict::of(&[Outcome::Passed, Outcome::NotBuilt]),
            Verdict::Incomplete
        );
        // …not even when every other step passed and the absence is the only blemish.
        assert_ne!(
            Verdict::of(&[Outcome::Passed, Outcome::Passed, Outcome::NotBuilt]),
            Verdict::Passed
        );
    }

    #[test]
    fn a_failure_outranks_an_absence() {
        // A broken thing is more urgent than a missing measurement, so it is what the exit code
        // reports — but the absence is still counted and printed.
        assert_eq!(
            Verdict::of(&[Outcome::Failed, Outcome::Unavailable]),
            Verdict::Failed
        );
        assert_eq!(
            Verdict::of(&[Outcome::NotBuilt, Outcome::Failed]),
            Verdict::Failed
        );
    }

    #[test]
    fn a_quarantine_is_an_absence_and_nothing_else() {
        // ⭐ §14.3's quarantine as a table (leaf `PROGRAM.10.1`): one step, one exit code.
        assert_eq!(judge(Some(0), false), Judgement::Passed);
        assert_eq!(judge(Some(20), true), Judgement::Quarantined);
        // The quarantined step disagreeing is still a failure: the exemption covers "could not
        // be run", never "ran and found a mismatch".
        assert_eq!(judge(Some(1), true), Judgement::Failed);
        assert_eq!(judge(None, true), Judgement::Failed);
        // "Could not be run" from a step no row names is a failure — otherwise any step could
        // exempt itself by choosing its exit code.
        assert_eq!(judge(Some(20), false), Judgement::Undeclared);
        // A quarantined step that passes has outlived its gap.
        assert_eq!(judge(Some(0), true), Judgement::Stale);
        // And the tier: an absence, never a pass, and outranked by a failure.
        assert_eq!(
            Verdict::of(&[Outcome::Passed, Outcome::Quarantined]),
            Verdict::Incomplete
        );
        assert_eq!(
            Verdict::of(&[Outcome::Quarantined, Outcome::Failed]),
            Verdict::Failed
        );
    }

    #[test]
    fn on_a_provisioned_environment_a_missing_tool_is_a_failure() {
        // `PROGRAM.10.3`'s policy: CI passes `--provisioned`, so what can leave a tier `incomplete`
        // there is only what a leaf owns — a quarantine or a step not built — never a tool the
        // workflow forgot to install. Locally an absent tool stays an absence.
        assert_eq!(missing_tool(false), Outcome::Unavailable);
        assert_eq!(missing_tool(true), Outcome::Failed);
        assert_eq!(
            Verdict::of(&[Outcome::Passed, missing_tool(true)]),
            Verdict::Failed
        );
        assert_eq!(
            Verdict::of(&[Outcome::Passed, missing_tool(false)]),
            Verdict::Incomplete
        );
    }

    #[test]
    fn a_failing_step_shows_the_cause_on_whichever_stream_it_was_written() {
        // The doctrine driver's shape, measured: each doctrine's line on stdout, the summary on
        // stderr. The breach is on the stream the old view dropped.
        let mut stdout: Vec<String> = (0..30).map(|i| format!("  ✅ DOCTRINE-{i}")).collect();
        stdout.insert(
            1,
            "  ❌ MEMORY-ARCH  the first doctrine, so the earliest line".to_string(),
        );
        let shown = tail_lines(
            &stdout.join("\n"),
            "=== 1 doctrine breach(es) — commit blocked ===",
        );
        assert!(
            shown.iter().any(|l| l.contains("❌ MEMORY-ARCH")),
            "{shown:?}"
        );
        assert!(
            shown.iter().any(|l| l.contains("commit blocked")),
            "{shown:?}"
        );
        // …and a long stream is still bounded, saying how much it left out.
        let long: String = (0..TAIL_LINES + 7).map(|i| format!("line {i}\n")).collect();
        let shown = tail_lines(&long, "");
        assert_eq!(shown.len(), TAIL_LINES + 1);
        assert_eq!(shown[0], "… 7 earlier line(s) omitted");
    }

    #[test]
    fn every_quarantine_carries_the_four_fields_and_names_one_step_that_runs() {
        for q in QUARANTINES {
            let named: Vec<&Action> = TIERS
                .iter()
                .flat_map(|t| t.steps)
                .filter(|step| step.name == q.step)
                .map(|step| &step.action)
                .collect();
            assert!(
                !named.is_empty(),
                "a quarantine names step `{}`, which no tier has",
                q.step
            );
            assert!(
                named.iter().all(|a| matches!(a, Action::Run { .. })),
                "`{}` is not built — there is nothing to quarantine",
                q.step
            );
            assert_eq!(
                QUARANTINES.iter().filter(|o| o.step == q.step).count(),
                1,
                "two quarantines for `{}`",
                q.step
            );
            assert!(
                q.issue.len() > 40 && q.claim.len() > 40,
                "`{}`'s quarantine does not say what is missing and what stays unproven",
                q.step
            );
            assert!(
                q.owner.contains('.') && q.owner.chars().next().is_some_and(char::is_uppercase),
                "`{}`'s owner `{}` is not a leaf id",
                q.step,
                q.owner
            );
        }
    }

    #[test]
    fn an_empty_tier_is_not_a_pass_worth_having() {
        // Vacuously it passes; the guard is that no tier is empty, so the vacuous case is
        // unreachable. A tier with no steps would report `passed` and prove nothing.
        assert_eq!(Verdict::of(&[]), Verdict::Passed);
        for tier in TIERS {
            assert!(!tier.steps.is_empty(), "tier `{}` has no steps", tier.name);
        }
    }

    #[test]
    fn exit_codes_are_distinct_and_only_passed_succeeds() {
        for verdict in [Verdict::Passed, Verdict::Failed, Verdict::Incomplete] {
            assert_eq!(verdict.code() == 0, verdict == Verdict::Passed);
        }
        assert_ne!(Verdict::Failed.code(), Verdict::Incomplete.code());
    }

    #[test]
    fn every_not_built_step_names_a_task_tree_leaf() {
        // A gap must route to tracked work. A step that said only "not implemented" would teach
        // a reader that the project forgot rather than that the work is scheduled.
        for tier in TIERS {
            for step in tier.steps {
                if let Some((owner, note)) = routed(&step.action) {
                    assert!(
                        owner.contains('.') && owner.chars().next().is_some_and(char::is_uppercase),
                        "`{}`'s owner `{owner}` is not a leaf id",
                        step.name
                    );
                    assert!(
                        note.len() > 40,
                        "`{}` does not say why it matters",
                        step.name
                    );
                }
            }
        }
    }

    #[test]
    fn a_gate_step_fails_on_any_refusal_and_is_never_passed() {
        // Ledger `TI-H7`: exit 0 leaves the step not built, for the acceptance `M3.6.5` owns; any other exit — a
        // refusal, a gate that could not judge, a signal — fails it.
        assert_eq!(gate_outcome(Some(0)), Outcome::NotBuilt);
        for code in [Some(1), Some(2), Some(20), None] {
            assert_eq!(gate_outcome(code), Outcome::Failed, "{code:?}");
        }
        let step = tier("assurance")
            .expect("the assurance tier")
            .steps
            .iter()
            .find(|s| s.name == "trust-inventory")
            .expect("its trust-inventory step");
        let Action::Gate {
            program,
            args,
            owner,
            ..
        } = &step.action
        else {
            panic!("the trust-inventory step runs the gate");
        };
        assert_eq!(
            (*program, *args, *owner),
            ("cargo", &["xtask", "trust-gate"][..], "M3.6.5")
        );
    }

    #[test]
    fn every_named_leaf_is_declared_by_a_task_tree() {
        // ⭐ The leg that makes routing real rather than decorative. A step naming `PROGRAM.9`
        // when no tree declares it sends a reader to a dead end, and the shape test above cannot
        // see the difference — `M9.9` has exactly the same shape as `M4.8`. This is the same
        // check `scripts/check_s0_retirement.sh` applies to the retirement record.
        let trees = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask/ sits at the repository root")
            .join("docs/tasks");
        let declared: String = std::fs::read_dir(&trees)
            .expect("the task trees exist")
            .filter_map(Result::ok)
            .filter(|entry| entry.path().extension().is_some_and(|e| e == "md"))
            .filter_map(|entry| std::fs::read_to_string(entry.path()).ok())
            .collect();
        for tier in TIERS {
            for step in tier.steps {
                if let Some((owner, _)) = routed(&step.action) {
                    assert!(
                        declared.contains(&format!("- ID: `{owner}`")),
                        "step `{}` names leaf `{owner}`, which no tree under docs/tasks/ declares",
                        step.name
                    );
                }
            }
        }
        for q in QUARANTINES {
            assert!(
                declared.contains(&format!("- ID: `{}`", q.owner)),
                "the `{}` quarantine names leaf `{}`, which no tree under docs/tasks/ declares",
                q.step,
                q.owner
            );
        }
    }

    #[test]
    fn every_tool_gated_step_says_why_the_tool_matters() {
        // An "unavailable" line that does not say what is lost invites the reader to ignore it.
        for tier in TIERS {
            for step in tier.steps {
                if let super::Action::Run {
                    requires: Some(tool),
                    matters,
                    ..
                } = &step.action
                {
                    assert!(
                        matters.len() > 40,
                        "`{}` gates on `{tool}` without saying what its absence costs",
                        step.name
                    );
                }
            }
        }
    }

    #[test]
    fn every_requirement_is_a_form_the_probe_understands() {
        // A requirement the probe does not recognise falls through to a PATH lookup of the literal string, so a
        // typo such as `component:miri` would report "unavailable" for ever while the tool sat installed — the
        // same misreport `PROGRAM.9.1` found.
        for tier in TIERS {
            for step in tier.steps {
                if let super::Action::Run {
                    requires: Some(tool),
                    ..
                } = &step.action
                {
                    let understood = if let Some(rest) = tool.strip_prefix("component:") {
                        rest.split_once(':').is_some_and(|(toolchain, component)| {
                            !toolchain.is_empty()
                                && !component.is_empty()
                                && !component.contains(':')
                        })
                    } else if let Some(triple) = tool.strip_prefix("target:") {
                        !triple.is_empty() && !triple.contains(':')
                    } else if let Some(rest) = tool.strip_prefix("target-env:") {
                        // `target-env:<file>:<key>`, resolved from the file when the step runs (leaf `M2.8.3.3`).
                        rest.split_once(':').is_some_and(|(file, key)| {
                            !file.is_empty() && !key.is_empty() && !key.contains(':')
                        })
                    } else {
                        !tool.is_empty() && !tool.contains(':') && !tool.contains('/')
                    };
                    assert!(
                        understood,
                        "`{}` requires `{tool}`, a form the probe does not understand",
                        step.name
                    );
                }
            }
        }
    }
}

/// The runtime core's Rust target is declared once, in the pinned target's `.env` (leaf `M2.8.3.3`).
#[cfg(test)]
mod rust_target {
    use super::{repo_root, resolve_requirement, target, RUST_TARGET_REQUIREMENT};

    fn declared() -> std::collections::BTreeMap<String, String> {
        target::env(
            &std::fs::read_to_string(repo_root().join("targets/riscv-virt-up.env"))
                .expect("the .env"),
        )
    }

    #[test]
    fn the_step_s_requirement_is_the_env_s_triple() {
        let env = declared();
        assert_eq!(
            resolve_requirement(&repo_root(), RUST_TARGET_REQUIREMENT),
            Ok(format!("target:{}", env["RUST_TARGET"]))
        );
    }

    #[test]
    fn the_isa_feature_set_is_the_triple_s_own() {
        // The triple's architecture part names the base and the extensions the ISA key does; the two keys are one fact
        // written twice for two readers, so they are held together.
        let env = declared();
        let arch = env["RUST_TARGET"].split('-').next().expect("a triple");
        let extensions = arch
            .strip_prefix("riscv64")
            .expect("a 64-bit RISC-V triple");
        assert_eq!(env["RUST_TARGET_ISA"], format!("rv64{extensions}"));
    }

    #[test]
    fn the_toolchain_file_installs_the_triple_and_nothing_else_declares_it() {
        let env = declared();
        let triple = &env["RUST_TARGET"];
        let toolchain = std::fs::read_to_string(repo_root().join("rust-toolchain.toml"))
            .expect("the toolchain file");
        assert!(
            toolchain.contains(&format!("\"{triple}\"")),
            "rust-toolchain.toml does not install {triple}"
        );
        // No second declaration in the runner. Built at run time, so this file does not contain it either.
        let needle = format!("riscv64{}", "imac-unknown-none-elf");
        for file in [
            "xtask/src/main.rs",
            "xtask/src/target.rs",
            "scripts/no_std_build.sh",
        ] {
            let text = std::fs::read_to_string(repo_root().join(file)).expect(file);
            assert!(
                !text.contains(&needle),
                "{file} declares the triple again; read RUST_TARGET instead"
            );
        }
    }

    #[test]
    fn a_requirement_whose_key_is_missing_cannot_be_read() {
        let dir = repo_root().join("target/doctrine_scratch/rust_target");
        std::fs::create_dir_all(&dir).expect("scratch");
        std::fs::write(dir.join("x.env"), "OTHER=1\n").expect("written");
        let resolved = resolve_requirement(
            &repo_root(),
            "target-env:target/doctrine_scratch/rust_target/x.env:RUST_TARGET",
        );
        assert_eq!(
            resolved,
            Err("target/doctrine_scratch/rust_target/x.env declares no RUST_TARGET".into())
        );
        let _ = std::fs::remove_dir_all(&dir);
    }
}
