//! Argument parsing and help rendering, both driven by [`crate::spec`].
//!
//! The parser is hand-written and dependency-free (see
//! `docs/decisions/decision_zero-dependency-engine-core.md`). The surface is small, fixed by
//! `ROADMAP.md` §10.2, and every diagnostic it emits is part of the user contract — which is
//! exactly the case where owning the code costs less than owning a dependency's wording.

use crate::spec::{command, CommandSpec, Maturity, COMMANDS};
use crate::status::Status;

/// What one command line asked for.
#[derive(Debug, PartialEq, Eq)]
pub enum Invocation {
    /// `archogen` with no arguments, `--help`, or `archogen help <command>`.
    Help(Option<&'static str>),
    /// `--version`.
    Version,
    /// A command to run.
    Run(Parsed),
}

/// A command line that parsed cleanly against its [`CommandSpec`].
#[derive(Debug, PartialEq, Eq)]
pub struct Parsed {
    /// The command that was named.
    pub name: &'static str,
    /// Positional values, in declaration order.
    pub positionals: Vec<String>,
    /// Options that were supplied, in the order they appeared. A flag carries `None`.
    pub options: Vec<(&'static str, Option<String>)>,
}

impl Parsed {
    /// The spec this invocation was validated against.
    #[must_use]
    pub fn spec(&self) -> &'static CommandSpec {
        command(self.name).expect("a parsed command always has a spec")
    }

    /// The value supplied for `long`, if any. `None` for an absent option *and* for a flag.
    #[must_use]
    pub fn value(&self, long: &str) -> Option<&str> {
        self.options
            .iter()
            .find(|(name, _)| *name == long)
            .and_then(|(_, value)| value.as_deref())
    }

    /// Whether a valueless flag was supplied.
    #[must_use]
    pub fn flag(&self, long: &str) -> bool {
        self.options.iter().any(|(name, _)| *name == long)
    }
}

/// A refused command line: what was wrong, and what to do instead.
#[derive(Debug, PartialEq, Eq)]
pub struct Refusal {
    /// The outcome to exit with. Always [`Status::Usage`] from this module.
    pub status: Status,
    /// What was wrong, one line.
    pub message: String,
    /// A concrete repair direction (`ROADMAP.md` §5.5 requires one on every diagnostic).
    pub repair: String,
}

fn refuse(message: impl Into<String>, repair: impl Into<String>) -> Refusal {
    Refusal {
        status: Status::Usage,
        message: message.into(),
        repair: repair.into(),
    }
}

/// Parse a command line, excluding the program name.
///
/// # Errors
///
/// Returns a [`Refusal`] when the command line names no known command, supplies an unknown
/// option, omits a required option or positional, or supplies a value where none is taken.
pub fn parse<I, S>(args: I) -> Result<Invocation, Refusal>
where
    I: IntoIterator<Item = S>,
    S: Into<String>,
{
    let args: Vec<String> = args.into_iter().map(Into::into).collect();
    let Some(first) = args.first() else {
        return Ok(Invocation::Help(None));
    };

    match first.as_str() {
        "-h" | "--help" => return Ok(Invocation::Help(None)),
        "-V" | "--version" => return Ok(Invocation::Version),
        "help" => {
            let Some(name) = args.get(1) else {
                return Ok(Invocation::Help(None));
            };
            let spec = command(name).ok_or_else(|| unknown_command(name))?;
            return Ok(Invocation::Help(Some(spec.name)));
        }
        _ => {}
    }

    if first.starts_with('-') {
        return Err(refuse(
            format!("`{first}` is an option, but a command was expected"),
            "run `archogen --help` for the command list".to_string(),
        ));
    }

    let spec = command(first).ok_or_else(|| unknown_command(first))?;
    let rest = &args[1..];

    // `--help` anywhere in a command's arguments asks for that command's help, before any
    // requirement is enforced: a user who does not yet know the options cannot be expected to
    // supply them in order to ask what they are.
    if rest.iter().any(|arg| arg == "-h" || arg == "--help") {
        return Ok(Invocation::Help(Some(spec.name)));
    }

    parse_command(spec, rest).map(Invocation::Run)
}

fn unknown_command(name: &str) -> Refusal {
    let known: Vec<&str> = COMMANDS.iter().map(|spec| spec.name).collect();
    refuse(
        format!("unknown command `{name}`"),
        format!("known commands: {}", known.join(", ")),
    )
}

fn parse_command(spec: &'static CommandSpec, args: &[String]) -> Result<Parsed, Refusal> {
    let mut positionals: Vec<String> = Vec::new();
    let mut options: Vec<(&'static str, Option<String>)> = Vec::new();
    let mut index = 0;

    while index < args.len() {
        let arg = &args[index];
        index += 1;

        let Some(body) = arg.strip_prefix("--") else {
            if arg.starts_with('-') && arg.len() > 1 {
                return Err(refuse(
                    format!("`{arg}` is not an option of `archogen {}`", spec.name),
                    format!(
                        "options are written `--name`; see `archogen help {}`",
                        spec.name
                    ),
                ));
            }
            positionals.push(arg.clone());
            continue;
        };

        // Accept both `--name value` and `--name=value`; the second form is unambiguous when
        // a value itself begins with a dash.
        let (long, inline) = match body.split_once('=') {
            Some((long, value)) => (long, Some(value.to_string())),
            None => (body, None),
        };

        let Some(option) = spec.option(long) else {
            return Err(unknown_option(spec, long));
        };

        if options.iter().any(|(name, _)| *name == option.long) {
            return Err(refuse(
                format!("`--{long}` was supplied more than once"),
                "supply each option at most once".to_string(),
            ));
        }

        match (option.value, inline) {
            (None, Some(_)) => {
                return Err(refuse(
                    format!("`--{long}` is a flag and takes no value"),
                    format!("write `--{long}` on its own"),
                ));
            }
            (None, None) => options.push((option.long, None)),
            (Some(placeholder), Some(value)) => {
                if value.is_empty() {
                    return Err(missing_value(long, placeholder));
                }
                options.push((option.long, Some(value)));
            }
            (Some(placeholder), None) => {
                let Some(value) = args.get(index) else {
                    return Err(missing_value(long, placeholder));
                };
                index += 1;
                options.push((option.long, Some(value.clone())));
            }
        }
    }

    check_arity(spec, &positionals)?;
    check_required(spec, &options)?;

    Ok(Parsed {
        name: spec.name,
        positionals,
        options,
    })
}

fn unknown_option(spec: &'static CommandSpec, long: &str) -> Refusal {
    let known: Vec<String> = spec
        .options
        .iter()
        .map(|option| format!("--{}", option.long))
        .collect();
    let repair = if known.is_empty() {
        format!("`archogen {}` takes no options", spec.name)
    } else {
        format!("`archogen {}` accepts: {}", spec.name, known.join(", "))
    };
    refuse(
        format!("unknown option `--{long}` for `archogen {}`", spec.name),
        repair,
    )
}

fn missing_value(long: &str, placeholder: &str) -> Refusal {
    refuse(
        format!("`--{long}` requires a value"),
        format!("write `--{long} <{placeholder}>`"),
    )
}

fn check_arity(spec: &'static CommandSpec, positionals: &[String]) -> Result<(), Refusal> {
    let expected = spec.positionals.len();
    if positionals.len() == expected {
        return Ok(());
    }
    if positionals.len() < expected {
        let missing = &spec.positionals[positionals.len()];
        return Err(refuse(
            format!("`archogen {}` is missing <{}>", spec.name, missing.name),
            format!("{} — {}", spec.usage(), missing.help),
        ));
    }
    Err(refuse(
        format!(
            "`archogen {}` takes {expected} positional argument(s), but {} were supplied",
            spec.name,
            positionals.len()
        ),
        spec.usage(),
    ))
}

fn check_required(
    spec: &'static CommandSpec,
    options: &[(&'static str, Option<String>)],
) -> Result<(), Refusal> {
    for option in spec.options {
        if option.required && !options.iter().any(|(name, _)| *name == option.long) {
            let placeholder = option.value.unwrap_or("VALUE");
            return Err(refuse(
                format!("`archogen {}` requires `--{}`", spec.name, option.long),
                format!("{} — {}", spec.usage(), option.help),
            )
            .with_placeholder(placeholder));
        }
    }
    Ok(())
}

impl Refusal {
    fn with_placeholder(mut self, placeholder: &str) -> Self {
        if !self.repair.contains(placeholder) {
            self.repair = format!("{} (value: <{placeholder}>)", self.repair);
        }
        self
    }
}

/// The top-level help text.
#[must_use]
pub fn help_overview() -> String {
    let width = COMMANDS
        .iter()
        .map(|spec| spec.name.len())
        .max()
        .unwrap_or(0);

    let mut out = String::new();
    out.push_str(
        "archogen — generate a specialized operating system from a functional eADL description\n\n",
    );
    out.push_str(
        "USAGE:\n    archogen <COMMAND> [OPTIONS]\n    archogen help <COMMAND>\n\nCOMMANDS:\n",
    );
    for spec in COMMANDS {
        out.push_str(&format!(
            "    {:width$}  {}{}\n",
            spec.name,
            spec.summary,
            spec.maturity.tag(),
            width = width
        ));
    }
    out.push_str("\nGLOBAL OPTIONS:\n");
    out.push_str(
        "    -h, --help       print this help, or `archogen help <COMMAND>` for one command\n",
    );
    out.push_str("    -V, --version    print the version\n");
    out.push_str("\nEXIT CODES:\n");
    for status in Status::ALL {
        out.push_str(&format!("    {:3}  {}\n", status.code(), status.slug()));
    }
    out
}

/// Help for one command.
#[must_use]
pub fn help_command(name: &str) -> String {
    let spec = command(name).expect("help is only rendered for a known command");
    let mut out = format!(
        "archogen {} — {}\n\nUSAGE:\n    {}\n",
        spec.name,
        spec.summary,
        spec.usage()
    );

    if !spec.positionals.is_empty() {
        out.push_str("\nARGUMENTS:\n");
        for positional in spec.positionals {
            out.push_str(&format!("    <{}>  {}\n", positional.name, positional.help));
        }
    }
    if !spec.options.is_empty() {
        out.push_str("\nOPTIONS:\n");
        for option in spec.options {
            let rendered = match option.value {
                Some(value) => format!("--{} <{}>", option.long, value),
                None => format!("--{}", option.long),
            };
            let required = if option.required { " (required)" } else { "" };
            out.push_str(&format!("    {rendered}  {}{required}\n", option.help));
        }
    }
    match &spec.maturity {
        Maturity::Built => {}
        Maturity::Experimental {
            scope,
            completed_by,
        } => {
            out.push_str(&format!(
                "\nSTATUS: EXPERIMENTAL\n    This command runs, but only over {scope}.\n    Its output carries no timing, assurance, or OS-completeness claim. Completing it\n    to the ROADMAP.md §10.2 contract is task-tree leaf {completed_by}\n    (see docs/TASK_TREE.md).\n"
            ));
        }
        Maturity::Unimplemented { owner } => {
            out.push_str(&format!(
                "\nSTATUS:\n    Not implemented yet. This command is part of the interface target in\n    ROADMAP.md §10.2; the work is tracked by task-tree leaf {owner}\n    (see docs/TASK_TREE.md). Invoking it exits {}.\n",
                Status::Unimplemented.code()
            ));
        }
    }
    out
}
