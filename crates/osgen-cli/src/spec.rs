//! The command surface, as data.
//!
//! `ROADMAP.md` §10.2 states an interface target: seven commands that the finished toolchain
//! must offer. Declaring them as one table — rather than as parallel `match` arms and a
//! hand-written help string — is what keeps `osgen --help` from drifting away from what the
//! parser actually accepts. The help text is *rendered from* this table, and the parser
//! *validates against* this table, so a new option cannot be documented without being
//! accepted, or accepted without being documented. A unit test asserts that property.
//!
//! Every command carries `owner`: the task-tree leaf that implements it. An `unimplemented`
//! result quotes it, so a user who hits an unbuilt command learns where the work is tracked
//! instead of filing a bug against a known gap.

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
    /// The task-tree leaf that owns implementing this command, or `None` once it is built.
    pub owner: Option<&'static str>,
}

impl CommandSpec {
    /// The `USAGE:` line body, e.g. `osgen check <DESCRIPTION> --profile <PROFILE>`.
    #[must_use]
    pub fn usage(&self) -> String {
        let mut out = format!("osgen {}", self.name);
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
        summary: "elaborate and type-check a description against a profile",
        positionals: &[PositionalSpec {
            name: "DESCRIPTION",
            help: "path to the eADL system description",
        }],
        options: &[PROFILE],
        owner: Some("M1.8"),
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
        owner: Some("M3.4"),
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
        owner: Some("M4.2"),
    },
    CommandSpec {
        name: "analyze",
        summary: "run an analysis for one named property over a build",
        positionals: &[PositionalSpec {
            name: "BUILD",
            help: "path to a build produced by `osgen build`",
        }],
        options: &[OptionSpec {
            long: "property",
            value: Some("PROPERTY"),
            help: "the named property to analyze, e.g. deadlines",
            required: true,
        }],
        owner: Some("M2.6"),
    },
    CommandSpec {
        name: "verify",
        summary: "run a verification tier over a build",
        positionals: &[PositionalSpec {
            name: "BUILD",
            help: "path to a build produced by `osgen build`",
        }],
        options: &[OptionSpec {
            long: "tier",
            value: Some("TIER"),
            help: "focused | integration | extended | hardware | assurance",
            required: true,
        }],
        owner: Some("PROGRAM.3"),
    },
    CommandSpec {
        name: "explain",
        summary: "explain how one requirement was realized, or why it could not be",
        positionals: &[PositionalSpec {
            name: "BUILD",
            help: "path to a build produced by `osgen build`",
        }],
        options: &[OptionSpec {
            long: "requirement",
            value: Some("REQUIREMENT"),
            help: "the requirement ID to explain, e.g. timer.deadline",
            required: true,
        }],
        owner: Some("M3.4"),
    },
    CommandSpec {
        name: "replay",
        summary: "replay a recorded failure manifest and check its identity",
        positionals: &[PositionalSpec {
            name: "MANIFEST",
            help: "path to a replay manifest recorded by a failing run",
        }],
        options: &[],
        owner: Some("M4.7"),
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
    fn every_unbuilt_command_names_the_leaf_that_owns_it() {
        // An `unimplemented` result must route the user to tracked work, never to a dead end.
        for spec in COMMANDS {
            if let Some(owner) = spec.owner {
                assert!(
                    owner.contains('.') && owner.chars().next().is_some_and(char::is_uppercase),
                    "`{}` owner `{owner}` is not a task-tree leaf id",
                    spec.name
                );
            }
        }
    }

    #[test]
    fn usage_renders_required_options_bare_and_optional_ones_bracketed() {
        let resolve = command("resolve").expect("resolve exists");
        let usage = resolve.usage();
        assert!(usage.starts_with("osgen resolve <DESCRIPTION>"), "{usage}");
        assert!(usage.contains("[--profile <PROFILE>]"), "{usage}");
        assert!(usage.contains("[--locked]"), "{usage}");
        assert!(usage.contains(" --out <PLAN>"), "{usage}");
    }
}
