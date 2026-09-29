//! The command surface, as data.
//!
//! `ROADMAP.md` §10.2 states an interface target: seven commands that the finished toolchain
//! must offer. Declaring them as one table — rather than as parallel `match` arms and a
//! hand-written help string — is what keeps `archogen --help` from drifting away from what the
//! parser actually accepts. The help text is *rendered from* this table, and the parser
//! *validates against* this table, so a new option cannot be documented without being
//! accepted, or accepted without being documented. A unit test asserts that property.
//!
//! Every command carries a [`Maturity`], and it has three states rather than two. A command can
//! be **built** to the §10.2 contract, **unimplemented**, or — the state `build` is in —
//! **experimental**: it runs, over a deliberately narrow path, and says so. `ROADMAP.md` §12 S0
//! requires exactly that of the early generation path ("Mark the output experimental, with no
//! claim of OS completeness or real-time assurance"), and two states cannot express it. Marking
//! it built would promise the §10.2 command; marking it unimplemented would deny a command that
//! works. Both are false, and the second is the more corrosive kind: a help text that lies about
//! a gap teaches users to stop reading it.
//!
//! Every non-built state names the task-tree leaf that changes it, so a user who hits a gap
//! learns where the work is tracked instead of filing a bug against a known limitation.

/// One option a command accepts.
pub struct OptionSpec {
    /// The long spelling, without the leading `--`.
    pub long: &'static str,
    /// The value placeholder, or `None` for a flag that takes no value.
    pub value: Option<&'static str>,
    /// One-line help.
    pub help: &'static str,
    /// Whether omitting it is a usage error.
    pub required: bool,
}

/// One positional argument.
pub struct PositionalSpec {
    /// The placeholder shown in usage.
    pub name: &'static str,
    /// One-line help.
    pub help: &'static str,
}

/// How much of the §10.2 contract a command actually delivers.
pub enum Maturity {
    /// Built to the §10.2 contract.
    Built,
    /// It runs, over a deliberately narrow path, and both the help text and the command's own
    /// output say so.
    Experimental {
        /// One line: what it actually does today.
        scope: &'static str,
        /// The task-tree leaf that completes it to the §10.2 contract.
        completed_by: &'static str,
    },
    /// Part of the §10.2 interface target, not built.
    Unimplemented {
        /// The task-tree leaf that owns building it.
        owner: &'static str,
    },
}

impl Maturity {
    /// The task-tree leaf that changes this state, or `None` for a built command.
    #[must_use]
    pub const fn owner(&self) -> Option<&'static str> {
        match self {
            Self::Built => None,
            Self::Experimental { completed_by, .. } => Some(completed_by),
            Self::Unimplemented { owner } => Some(owner),
        }
    }

    /// The tag shown beside the command in `archogen --help`.
    #[must_use]
    pub fn tag(&self) -> String {
        match self {
            Self::Built => String::new(),
            Self::Experimental {
                scope,
                completed_by,
            } => format!("   [experimental — {scope}; completed by leaf {completed_by}]"),
            Self::Unimplemented { owner } => {
                format!("   [unimplemented — tracked by leaf {owner}]")
            }
        }
    }
}

/// One command of the §10.2 surface.
pub struct CommandSpec {
    /// The subcommand word.
    pub name: &'static str,
    /// One-line summary, shown in the command list.
    pub summary: &'static str,
    /// The positional arguments, in order. All are required.
    pub positionals: &'static [PositionalSpec],
    /// The options, in display order.
    pub options: &'static [OptionSpec],
    /// How much of the §10.2 contract this command delivers today.
    pub maturity: Maturity,
}

impl CommandSpec {
    /// The `USAGE:` line body, e.g. `archogen check <DESCRIPTION> --profile <PROFILE>`.
    #[must_use]
    pub fn usage(&self) -> String {
        let mut out = format!("archogen {}", self.name);
        for positional in self.positionals {
            out.push_str(&format!(" <{}>", positional.name));
        }
        for option in self.options {
            let rendered = match option.value {
                Some(value) => format!("--{} <{}>", option.long, value),
                None => format!("--{}", option.long),
            };
            out.push_str(&if option.required {
                format!(" {rendered}")
            } else {
                format!(" [{rendered}]")
            });
        }
        out
    }

    /// Look up an option by its long spelling.
    #[must_use]
    pub fn option(&self, long: &str) -> Option<&OptionSpec> {
        self.options.iter().find(|option| option.long == long)
    }
}

/// A profile selector, accepted by every command that admits a system.
const PROFILE: OptionSpec = OptionSpec {
    long: "profile",
    value: Some("PROFILE"),
    help: "the supported profile to admit the system under, e.g. rt-static-up-v1",
    required: false,
};

/// Refuse to acquire or re-resolve anything: fail on a missing or mismatched input (§10.3).
const LOCKED: OptionSpec = OptionSpec {
    long: "locked",
    value: None,
    help: "fail on any missing or mismatched locked input instead of re-resolving",
    required: false,
};

/// The seven commands of the §10.2 interface target, in the order the roadmap lists them.
pub const COMMANDS: &[CommandSpec] = &[
    CommandSpec {
        name: "check",
        // ⛔ Each clause is a capability, coupled to it by `tests/module_files.rs`. This line said
        // "elaborate and type-check a description" while nothing in production called the elaborator
        // (leaf `M1.29.1`); `M1.29.2` wired elaboration in, and an elaborated tree is not type-checked
        // until `M1.29.3` — so the two halves are stated separately until the second is true of both.
        summary: "elaborate a module tree, and type-check a description, against a profile",
        positionals: &[PositionalSpec {
            name: "DESCRIPTION",
            help: "path to the eADL system description",
        }],
        options: &[PROFILE],
        // Built by leaf M1.8. Changing this row is what removes the command's tag from
        // `archogen --help`, in the same change that makes it real — the help text is rendered
        // from this table, so the two cannot disagree.
        maturity: Maturity::Built,
    },
    CommandSpec {
        name: "resolve",
        summary: "resolve providers and resources into an independently checked build plan",
        positionals: &[PositionalSpec {
            name: "DESCRIPTION",
            help: "path to the eADL system description",
        }],
        options: &[
            PROFILE,
            LOCKED,
            OptionSpec {
                long: "out",
                value: Some("PLAN"),
                help: "where to write the resolved plan",
                required: true,
            },
        ],
        maturity: Maturity::Unimplemented { owner: "M3.4" },
    },
    CommandSpec {
        name: "build",
        summary: "generate, assemble and build a complete system and its simulator",
        positionals: &[PositionalSpec {
            name: "DESCRIPTION",
            help: "path to the eADL system description",
        }],
        options: &[
            PROFILE,
            LOCKED,
            OptionSpec {
                long: "out",
                value: Some("DIR"),
                help: "the dedicated output directory for generated artifacts",
                required: true,
            },
        ],
        // ⚠️ Runs today over the S0 path only (leaf `S0.3`): one fixed engine-owned realization,
        // periodic releases, the hosted playground. §12 S0 requires that output to be marked
        // experimental, so the surface marks the command.
        maturity: Maturity::Experimental {
            scope: "the S0 path: one fixed realization, periodic releases, hosted playground",
            completed_by: "M4.2",
        },
    },
    CommandSpec {
        name: "analyze",
        summary: "run an analysis for one named property over a build",
        positionals: &[PositionalSpec {
            name: "BUILD",
            help: "path to a build produced by `archogen build`",
        }],
        options: &[OptionSpec {
            long: "property",
            value: Some("PROPERTY"),
            help: "the named property to analyze, e.g. deadlines",
            required: true,
        }],
        maturity: Maturity::Unimplemented { owner: "M2.6" },
    },
    CommandSpec {
        name: "verify",
        summary: "run a verification tier over a build",
        positionals: &[PositionalSpec {
            name: "BUILD",
            help: "path to a build produced by `archogen build`",
        }],
        options: &[OptionSpec {
            long: "tier",
            value: Some("TIER"),
            help: "focused | integration | extended | hardware | assurance",
            required: true,
        }],
        maturity: Maturity::Unimplemented { owner: "PROGRAM.3" },
    },
    CommandSpec {
        name: "explain",
        summary: "explain how one requirement was realized, or why it could not be",
        positionals: &[PositionalSpec {
            name: "BUILD",
            help: "path to a build produced by `archogen build`",
        }],
        options: &[OptionSpec {
            long: "requirement",
            value: Some("REQUIREMENT"),
            help: "the requirement ID to explain, e.g. timer.deadline",
            required: true,
        }],
        maturity: Maturity::Unimplemented { owner: "M3.4" },
    },
    CommandSpec {
        name: "replay",
        summary: "replay a recorded failure manifest and check its identity",
        positionals: &[PositionalSpec {
            name: "MANIFEST",
            help: "path to a replay manifest recorded by a failing run",
        }],
        options: &[],
        maturity: Maturity::Unimplemented { owner: "M4.7" },
    },
];

/// Look up a command by name.
#[must_use]
pub fn command(name: &str) -> Option<&'static CommandSpec> {
    COMMANDS.iter().find(|spec| spec.name == name)
}

#[cfg(test)]
mod tests {
    use super::{command, COMMANDS};

    #[test]
    fn the_surface_is_exactly_the_roadmap_interface_target() {
        // ROADMAP.md §10.2 lists these seven and no others. A command added here without a
        // roadmap change is scope drift; one removed is a broken user contract.
        let names: Vec<&str> = COMMANDS.iter().map(|spec| spec.name).collect();
        assert_eq!(
            names,
            vec!["check", "resolve", "build", "analyze", "verify", "explain", "replay"]
        );
    }

    #[test]
    fn command_names_are_unique() {
        for spec in COMMANDS {
            assert!(command(spec.name).is_some());
        }
        let mut names: Vec<&str> = COMMANDS.iter().map(|spec| spec.name).collect();
        let total = names.len();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), total);
    }

    #[test]
    fn option_names_are_unique_within_a_command() {
        for spec in COMMANDS {
            let mut longs: Vec<&str> = spec.options.iter().map(|option| option.long).collect();
            let total = longs.len();
            longs.sort_unstable();
            longs.dedup();
            assert_eq!(longs.len(), total, "duplicate option in `{}`", spec.name);
        }
    }

    #[test]
    fn every_incomplete_command_names_the_leaf_that_completes_it() {
        // A gap must route the user to tracked work, never to a dead end — and that holds for
        // the experimental state too, where the temptation to leave it unsaid is strongest.
        for spec in COMMANDS {
            if let Some(owner) = spec.maturity.owner() {
                assert!(
                    owner.contains('.') && owner.chars().next().is_some_and(char::is_uppercase),
                    "`{}` names `{owner}`, which is not a task-tree leaf id",
                    spec.name
                );
            }
        }
    }

    #[test]
    fn only_a_built_command_carries_no_tag() {
        // The tag is the whole mechanism: a command that runs but is narrower than its §10.2
        // contract must not render as if it were finished.
        for spec in COMMANDS {
            assert_eq!(
                spec.maturity.tag().is_empty(),
                matches!(spec.maturity, super::Maturity::Built),
                "`{}` renders the wrong tag for its maturity",
                spec.name
            );
        }
    }

    #[test]
    fn the_experimental_tag_says_both_what_it_does_and_what_completes_it() {
        let build = command("build").expect("build exists");
        let tag = build.maturity.tag();
        assert!(tag.contains("experimental"), "{tag}");
        assert!(tag.contains("S0"), "{tag}");
        assert!(tag.contains("M4.2"), "{tag}");
    }

    #[test]
    fn usage_renders_required_options_bare_and_optional_ones_bracketed() {
        let resolve = command("resolve").expect("resolve exists");
        let usage = resolve.usage();
        assert!(
            usage.starts_with("archogen resolve <DESCRIPTION>"),
            "{usage}"
        );
        assert!(usage.contains("[--profile <PROFILE>]"), "{usage}");
        assert!(usage.contains("[--locked]"), "{usage}");
        assert!(usage.contains(" --out <PLAN>"), "{usage}");
    }
}
