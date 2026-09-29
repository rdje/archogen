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

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

/// How many trailing lines of a failing step's output to show. Enough to carry a test failure's
/// assertion diff, short enough that a tier's report stays readable.
const TAIL_LINES: usize = 24;

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
        } else if outcomes
            .iter()
            .any(|o| matches!(o, Outcome::Unavailable | Outcome::NotBuilt))
        {
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
                proves: "the mdBook builds — it is the director's window, so a broken book is a broken deliverable",
                action: Action::Run {
                    program: "mdbook",
                    args: &["build", "docs/book"],
                    requires: Some("mdbook"),
                    matters: "the book is the project's public surface and is required to stay in \
                              lockstep with the code; `cargo install mdbook`",
                },
            },
            Step {
                name: "no-std-build",
                proves: "the runtime core compiles for a bare-metal target (§14.3's \"compile targets\")",
                action: Action::Run {
                    program: "cargo",
                    args: &[
                        "build",
                        "--quiet",
                        "-p",
                        "rt-core",
                        "--target",
                        "riscv64imac-unknown-none-elf",
                    ],
                    requires: Some("target:riscv64imac-unknown-none-elf"),
                    matters: "§3.1 fixes the runtime as a \"Rust no_std core\", and `#![no_std]` \
                              being active in a host build is evidence that it *can* be, not that \
                              it *does* build for a target. `rustup target add \
                              riscv64imac-unknown-none-elf`",
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
        ],
    },
    Tier {
        name: "extended",
        when: "scheduled, or when a change touches parsing, arithmetic or event ordering",
        steps: &[
            Step {
                name: "fuzz",
                proves: "the reader and the checked arithmetic survive a fuzz corpus (§13.3)",
                action: Action::NotBuilt {
                    owner: "PROGRAM.9",
                    note: "no fuzz target exists yet; §13.3 asks for property tests and fuzzing \
                           over parsing, constraints, checked arithmetic and event sequences",
                },
            },
            Step {
                name: "mutation",
                proves: "the suite is sensitive to the defects it claims to catch",
                action: Action::NotBuilt {
                    owner: "PROGRAM.9",
                    note: "mutation controls are run by hand, per leaf, and recorded in each \
                           acceptance checklist — which is evidence but not a repeatable tier. \
                           One of them found a real blind spot in F28 (see \
                           docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md)",
                },
            },
            Step {
                name: "miri",
                proves: "the hosted logic Miri can execute is free of the UB it detects",
                action: Action::Run {
                    program: "cargo",
                    args: &["miri", "test"],
                    requires: Some("cargo-miri"),
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
                action: Action::NotBuilt {
                    owner: "M3.6",
                    note: "the trust-dependency inventory and its reviewed baseline do not exist \
                           yet; §14.4 requires the pipeline, not the generator, to produce them",
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

fn run_step(step: &Step, root: &Path) -> Outcome {
    match &step.action {
        Action::NotBuilt { owner, note } => {
            println!("  ⚠  {:<18} NOT BUILT — tracked by leaf {owner}", step.name);
            println!("     {note}");
            Outcome::NotBuilt
        }
        Action::Run {
            program,
            args,
            requires,
            matters,
        } => {
            if let Some(tool) = requires {
                if !on_path(tool) {
                    // A rustup target is not an executable, and telling a reader it is "not on
                    // PATH" sends them to fix the wrong thing.
                    let missing = tool.strip_prefix("target:").map_or_else(
                        || format!("`{tool}` is not on PATH"),
                        |triple| format!("the `{triple}` target is not installed"),
                    );
                    println!("  ⚠  {:<18} UNAVAILABLE — {missing}", step.name);
                    println!("     {matters}");
                    return Outcome::Unavailable;
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
            match status {
                Ok(output) if output.status.success() => {
                    println!("  ✅ {:<18} {elapsed:>6.2}s  {}", step.name, step.proves);
                    Outcome::Passed
                }
                Ok(output) => {
                    println!("  ❌ {:<18} {elapsed:>6.2}s  FAILED", step.name);
                    let stderr = String::from_utf8_lossy(&output.stderr);
                    let stdout = String::from_utf8_lossy(&output.stdout);
                    let detail = if stderr.trim().is_empty() {
                        stdout
                    } else {
                        stderr
                    };
                    let lines: Vec<&str> = detail.lines().collect();
                    let tail = lines.len().saturating_sub(TAIL_LINES);
                    if tail > 0 {
                        println!("     … {tail} earlier line(s) omitted");
                    }
                    for line in &lines[tail..] {
                        println!("     {line}");
                    }
                    println!("     re-run it directly: {program} {}", args.join(" "));
                    Outcome::Failed
                }
                Err(error) => {
                    // A program that will not start is not a passed check either.
                    println!("  ❌ {:<18} could not run `{program}`: {error}", step.name);
                    Outcome::Failed
                }
            }
        }
    }
}

fn verify(name: &str) -> i32 {
    let Some(tier) = tier(name) else {
        eprintln!("xtask: unknown tier `{name}`");
        eprintln!("  hint: {}", tier_names());
        return 2;
    };
    let root = repo_root();
    println!("tier: {} — {}", tier.name, tier.when);
    let outcomes: Vec<Outcome> = tier
        .steps
        .iter()
        .map(|step| run_step(step, &root))
        .collect();

    let count = |want: Outcome| outcomes.iter().filter(|o| **o == want).count();
    let verdict = Verdict::of(&outcomes);
    println!(
        "tier {}: {} — {} passed, {} failed, {} unavailable, {} not built",
        tier.name,
        verdict.slug(),
        count(Outcome::Passed),
        count(Outcome::Failed),
        count(Outcome::Unavailable),
        count(Outcome::NotBuilt)
    );
    if verdict == Verdict::Incomplete {
        println!(
            "  ⚠  incomplete is NOT a pass. §14.3: \"a required tool skipped or unavailable is \
             reported as such, not a passed check\"."
        );
    }
    verdict.code()
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
                Action::Run {
                    requires: Some(tool),
                    ..
                } => format!("needs `{tool}`"),
                Action::Run { .. } => "always runnable".to_string(),
            };
            println!("  {:<18} {:<26} {}", step.name, shape, step.proves);
        }
        println!();
    }
}

fn help() {
    println!("xtask — the archogen repository's tiered verification runner (ROADMAP.md §14.3)");
    println!();
    println!("USAGE:");
    println!("    cargo xtask verify --tier <TIER>");
    println!("    cargo xtask verify --list");
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
        ["verify", "--tier", name] => verify(name),
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
    use super::{tier, Outcome, Verdict, TIERS};

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
                if let super::Action::NotBuilt { owner, note } = &step.action {
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
                if let super::Action::NotBuilt { owner, .. } = &step.action {
                    assert!(
                        declared.contains(&format!("- ID: `{owner}`")),
                        "step `{}` names leaf `{owner}`, which no tree under docs/tasks/ declares",
                        step.name
                    );
                }
            }
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
}
