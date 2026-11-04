//! What the tests of `archogen build` share (leaf `M4.2.3.1`): the configuration checker in this process.
//!
//! ⭐ **The checker's own judgement, and no stand-in.** [`Checker`] takes the arguments the front hands
//! `archogen-plan-check`, judges with the function that executable calls, and writes the record it would write. A test
//! therefore runs a real plan past the real rules without an install, which the front's own tests (`tests/resolve.rs`,
//! with a fake that writes the record it is told) do not. The executable's half is `scripts/resolve_installed.sh`'s.

use std::io::Write;
use std::path::Path;

use archogen_cli::consume::PLAN_CHECKER;
use archogen_cli::launch::{Ended, Launch, Launched, Unlaunched};
use archogen_cli::{run_with_input, Status};
use archogen_digest::Digest;
use archogen_outcome::record::Producer;
use archogen_plan_check::judgement::{judge, profile, Inputs};
use eadl_front::DirectoryModules;

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
        let plan_bytes = std::fs::read(plan).expect("the plan the front wrote");
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
        std::fs::write(result, record.write()).expect("the record written");
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
