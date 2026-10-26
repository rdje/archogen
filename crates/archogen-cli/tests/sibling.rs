//! The front's launcher (leaf `M3.5.1.4.1`), run as an install runs it: the `archogen` executable copied into a scratch
//! directory beside a sibling and the install manifest, `archogen-install/1`, that names them both. The sibling here is
//! a second copy of `archogen` under the configuration checker's name — this package compiles no checker, and what is
//! held is the front's half: that it runs a sibling the manifest vouches for, with the arguments it was given, an
//! empty standard input and its output and exit code relayed, and refuses one it cannot vouch for.

use std::io::Read;
use std::path::{Path, PathBuf};
use std::process::{Command, Output, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::time::{Duration, Instant};

use archogen_digest::Digest;
use archogen_install::{Build, Manifest, Program, FILE};

const FRONT: &str = env!("CARGO_BIN_EXE_archogen");

/// The sibling's name in an install.
const CHECKER: &str = "archogen-plan-check";

/// A fresh scratch directory under the target directory, canonical, one per case.
fn scratch() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "front-sibling-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    std::fs::canonicalize(&dir).expect("a canonical scratch directory")
}

/// What a case changes in the install it builds.
#[derive(Default)]
struct Spoil {
    /// The directory the manifest names, when not the install's own.
    directory: Option<String>,
    /// The front's sha256 the manifest names, when not the front's own.
    front: Option<Digest>,
    /// No sibling in the manifest.
    no_sibling: bool,
    /// No manifest at all.
    no_manifest: bool,
}

/// An install in a fresh directory: the front, the sibling and a manifest naming them, spoilt as `spoil` says; the
/// installed front's path.
fn install(spoil: &Spoil) -> PathBuf {
    let dir = scratch();
    let bytes = std::fs::read(FRONT).expect("the built front");
    for name in ["archogen", CHECKER] {
        std::fs::write(dir.join(name), &bytes).expect("a program placed");
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;
            std::fs::set_permissions(dir.join(name), std::fs::Permissions::from_mode(0o755))
                .expect("an executable program");
        }
    }
    if spoil.no_manifest {
        return dir.join("archogen");
    }
    let digest = Digest::of(&bytes);
    let siblings = if spoil.no_sibling {
        Vec::new()
    } else {
        vec![Program {
            name: CHECKER.to_owned(),
            role: "configuration-checker".to_owned(),
            sha256: digest,
        }]
    };
    let manifest = Manifest::new(
        spoil
            .directory
            .clone()
            .unwrap_or_else(|| dir.to_str().unwrap().to_owned()),
        Build {
            commit: "0".repeat(40),
            tree: "1".repeat(40),
            rustc: "rustc 1.95.0".to_owned(),
            host: "test-host".to_owned(),
        },
        Digest::of(b"an inventory"),
        Program {
            name: "archogen".to_owned(),
            role: "generator".to_owned(),
            sha256: spoil.front.unwrap_or(digest),
        },
        siblings,
    )
    .expect("a manifest");
    std::fs::write(dir.join(FILE), manifest.write()).expect("the manifest written");
    dir.join("archogen")
}

/// Run the installed `front` with `args`, its standard input held open and never written, so a sibling that read it
/// would wait: the process's output, or a failure after a minute.
fn run(front: &Path, args: &[&str]) -> Output {
    let mut child = Command::new(front)
        .args(args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("the front starts");
    let _held = child.stdin.take();
    let started = Instant::now();
    loop {
        if let Some(status) = child.try_wait().expect("the front's state") {
            let (mut stdout, mut stderr) = (Vec::new(), Vec::new());
            child
                .stdout
                .take()
                .unwrap()
                .read_to_end(&mut stdout)
                .unwrap();
            child
                .stderr
                .take()
                .unwrap()
                .read_to_end(&mut stderr)
                .unwrap();
            return Output {
                status,
                stdout,
                stderr,
            };
        }
        if started.elapsed() > Duration::from_secs(60) {
            let _ = child.kill();
            panic!(
                "`archogen {}` did not end: a sibling read the front's standard input",
                args.join(" ")
            );
        }
        std::thread::sleep(Duration::from_millis(20));
    }
}

fn text(bytes: &[u8]) -> String {
    String::from_utf8_lossy(bytes).into_owned()
}

#[test]
fn a_sibling_the_install_vouches_for_runs_with_its_arguments_and_its_output_and_code_relayed() {
    let front = install(&Spoil::default());
    // The sibling — `archogen` under the checker's name — answers `--version` with its own line, which the front would
    // have answered itself had it read the argument.
    let output = run(&front, &["plan-check", "--version"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
    assert!(
        text(&output.stdout).starts_with("archogen "),
        "{}",
        text(&output.stdout)
    );
    // Its exit code is relayed: a command it does not know is its usage error.
    let output = run(&front, &["plan-check", "frobnicate"]);
    assert_eq!(output.status.code(), Some(2));
    assert!(
        text(&output.stderr).contains("unknown command `frobnicate`"),
        "{}",
        text(&output.stderr)
    );
    // Its help is its own.
    let output = run(&front, &["help", "plan-check"]);
    assert_eq!(output.status.code(), Some(0));
    assert!(
        text(&output.stdout).contains("USAGE"),
        "{}",
        text(&output.stdout)
    );
    // Its standard input is empty: a sibling serving messages from it ends at once, though the front's is held open.
    let output = run(&front, &["plan-check", "mcp"]);
    assert_eq!(output.status.code(), Some(0), "{}", text(&output.stderr));
}

#[test]
fn a_sibling_the_install_cannot_vouch_for_is_refused_before_it_runs() {
    let elsewhere = scratch();
    for (case, spoil, said) in [
        (
            "no manifest",
            Spoil {
                no_manifest: true,
                ..Spoil::default()
            },
            "has no install manifest, archogen-install.json",
        ),
        (
            "a manifest of another directory",
            Spoil {
                directory: Some(elsewhere.to_str().unwrap().to_owned()),
                ..Spoil::default()
            },
            "so it describes another install",
        ),
        (
            "another front",
            Spoil {
                front: Some(Digest::of(b"another front")),
                ..Spoil::default()
            },
            "this `archogen` is not the one the install",
        ),
        (
            "no such sibling",
            Spoil {
                no_sibling: true,
                ..Spoil::default()
            },
            "has no `archogen-plan-check`",
        ),
    ] {
        let front = install(&spoil);
        let output = run(&front, &["plan-check", "--version"]);
        assert_eq!(output.status.code(), Some(70), "{case}");
        let err = text(&output.stderr);
        assert!(err.contains("archogen: tool-failure: "), "{case}: {err}");
        assert!(err.contains(said), "{case}: {err}");
        assert!(output.stdout.is_empty(), "{case}: the sibling ran");
    }
    // A sibling whose bytes are not the manifest's sha256: a stale or hand-copied program.
    let front = install(&Spoil::default());
    std::fs::write(front.with_file_name(CHECKER), b"#!/bin/sh\necho ran\n").unwrap();
    let output = run(&front, &["plan-check", "--version"]);
    assert_eq!(output.status.code(), Some(70));
    assert!(
        text(&output.stderr)
            .contains("is not the one the install placed: its sha256 is not the manifest's"),
        "{}",
        text(&output.stderr)
    );
    assert!(output.stdout.is_empty(), "the sibling ran");
}
