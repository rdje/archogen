//! What the tests of `archogen build` share (leaf `M4.2.3.1`): the configuration checker in this process.
//!
//! ⭐ **The checker's own judgement, and no stand-in.** [`Checker`] takes the arguments the front hands
//! `archogen-plan-check`, judges with the function that executable calls, and writes the record it would write. A test
//! therefore runs a real plan past the real rules without an install, which the front's own tests (`tests/resolve.rs`,
//! with a fake that writes the record it is told) do not. The executable's half is `scripts/resolve_installed.sh`'s.

use std::io::Write;
use std::path::{Path, PathBuf};

use archogen_cli::consume::PLAN_CHECKER;
use archogen_cli::launch::{Ended, Launch, Launched, Unlaunched};
use archogen_cli::{run_with_input, Status};
use archogen_digest::Digest;
use archogen_outcome::record::Producer;
use archogen_plan_check::judgement::{judge, profile, Inputs};
use eadl_front::DirectoryModules;

/// The rows of the table in `chapter` after `<!-- machine-read: name -->`, each row its cells, code spans unwrapped, its header
/// and its rule left out. Only the test binaries that hold a table to its chapter call it.
#[allow(dead_code)]
pub fn table_rows(chapter: &str, name: &str) -> Vec<Vec<String>> {
    let marker = format!("<!-- machine-read: {name} -->");
    let (_, after) = chapter.split_once(&marker).expect("the table's marker");
    after
        .lines()
        .skip_while(|line| line.trim().is_empty())
        .take_while(|line| line.starts_with('|'))
        .skip(2)
        .map(|row| {
            row.trim_matches('|')
                .split('|')
                .map(|cell| cell.trim().trim_matches('`').to_owned())
                .collect()
        })
        .collect()
}

/// A path under `root` of exactly `length` bytes, in names short enough to take a suffix.
#[allow(dead_code)]
pub fn a_path_of_length(root: &Path, length: usize) -> PathBuf {
    let mut path = root.to_path_buf();
    let mut left = length - root.as_os_str().len();
    while left > 0 {
        let name = (left - 1).min(200);
        if left - 1 - name == 1 {
            // A remainder of one byte would be a separator and no name.
            path.push("x".repeat(name - 1));
            left -= name;
        } else {
            path.push("x".repeat(name));
            left -= name + 1;
        }
    }
    path
}

/// The length at which this host first refuses a path by its length alone, whatever the path names.
#[allow(dead_code)]
pub fn path_limit(root: &Path) -> usize {
    (root.as_os_str().len() + 10..1 << 16)
        .find(|length| {
            std::fs::symlink_metadata(a_path_of_length(root, *length))
                .is_err_and(|error| error.kind() == std::io::ErrorKind::InvalidFilename)
        })
        .expect("a path this long is too long")
}

/// The configuration checker, judging in this process.
pub struct Checker;

impl Launch for Checker {
    fn launch(
        &self,
        program: &str,
        args: &[String],
        _out: &mut dyn Write,
        _err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched> {
        assert_eq!(program, PLAN_CHECKER);
        // `--profile P --result R -- DESCRIPTION PLAN`, as the front hands them over.
        let (asked, result, description, plan) = (&args[1], &args[3], &args[5], &args[6]);
        let text = std::fs::read_to_string(description).expect("the description");
        // As the real one does: what it cannot read or write leaves no record of it, and ends with its failure.
        let could_not = || {
            Ok(Launched {
                ended: Ended::Code(Status::ToolFailure.code()),
                sha256: checker_digest(),
                front: front_digest(),
            })
        };
        let Ok(plan_bytes) = std::fs::read(plan) else {
            return could_not();
        };
        let modules = DirectoryModules::new(
            Path::new(description)
                .parent()
                .unwrap_or_else(|| Path::new("")),
        );
        let record = judge(
            Producer {
                name: PLAN_CHECKER.to_owned(),
                sha256: checker_digest(),
            },
            &Inputs {
                description_name: description,
                description: &text,
                modules: &modules,
                profile: profile(Some(asked)).expect("a profile the front supports"),
                plan_name: plan,
                plan: &plan_bytes,
            },
        )
        .expect("judged");
        if std::fs::write(result, record.write()).is_err() {
            let _ = std::fs::remove_file(result);
            return could_not();
        }
        Ok(Launched {
            ended: Ended::Code(record.status().code()),
            sha256: checker_digest(),
            front: front_digest(),
        })
    }
}

/// The checker's sha256 as a launcher would have verified it.
pub fn checker_digest() -> Digest {
    Digest::of(b"archogen-plan-check")
}

/// The front's sha256 as a launcher would have verified it.
pub fn front_digest() -> Digest {
    Digest::of(b"archogen")
}

/// Run `archogen` with `args` through `launcher`: the status, standard output and standard error.
pub fn invoke(args: &[&str], launcher: &dyn Launch) -> (Status, String, String) {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let status = run_with_input(
        args.iter().map(|s| (*s).to_owned()),
        &mut std::io::empty(),
        launcher,
        &mut out,
        &mut err,
    );
    (
        status,
        String::from_utf8(out).expect("utf-8"),
        String::from_utf8(err).expect("utf-8"),
    )
}
