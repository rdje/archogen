//! What a refused run leaves in its stage, held by sweeping what a host can do rather than by naming shapes (`docs/decisions/decision_hosted-build.md` §5; the hosted build's round 15). The
//! rule is one: a run removes what it wrote, and a stage that stands is said. So nothing of a run's stands after a refusal no other program had a hand in, and what another program
//! put there stands after one it did, whatever it reads as.
#![cfg(unix)]

mod common;

use std::fs;
use std::os::unix::fs::{symlink, PermissionsExt};
use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_cli::consume::RECORD;
use archogen_cli::launch::{Ended, Launch, Launched, Unlaunched};
use archogen_cli::status::Status;
use common::{checker_digest, front_digest, invoke, Checker};

/// What tells the child test what to run.
const ARGS: &str = "ARCHOGEN_STAGE_LEFT_ARGS";

fn description() -> String {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .join("../../examples/s0-heartbeat/system.eadl")
        .display()
        .to_string()
}

fn scratch(name: &str) -> PathBuf {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!("stage-left-{name}"));
    reopened_and_gone(&dir);
    fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

/// `dir`, everything in it opened to its owner first, gone.
fn reopened_and_gone(dir: &Path) {
    fn open(at: &Path) {
        let _ = fs::set_permissions(at, fs::Permissions::from_mode(0o700));
        if let Ok(entries) = fs::read_dir(at) {
            for entry in entries.flatten() {
                if entry.file_type().is_ok_and(|kind| kind.is_dir()) {
                    open(&entry.path());
                }
            }
        }
    }
    open(dir);
    if fs::remove_dir_all(dir).is_err() {
        let _ = fs::remove_file(dir);
    }
}

/// The command, in a process of its own: a umask or a limit on a file's size cannot be set in this one, whose other tests it would bind.
#[test]
fn child() {
    let Ok(args) = std::env::var(ARGS) else {
        return;
    };
    let args: Vec<&str> = args.split('\n').collect();
    let (status, out, err) = invoke(&args, &Checker);
    println!("status: {}", status.slug());
    println!("{out}{err}");
}

/// `args` run under what `imposed` sets: the status, and what it said.
fn under(imposed: &str, args: &[String]) -> (String, String) {
    let done = Command::new("sh")
        .arg("-c")
        .arg(format!(
            "{imposed}; trap '' XFSZ; exec \"$0\" --exact child --nocapture --test-threads=1"
        ))
        .arg(std::env::current_exe().expect("this test"))
        .env(ARGS, args.join("\n"))
        .output()
        .expect("it runs");
    let said = String::from_utf8_lossy(&done.stdout).into_owned();
    let status = said
        .lines()
        .find_map(|line| line.split_once("status: ").map(|(_, status)| status))
        .unwrap_or_else(|| {
            panic!(
                "{imposed}: {args:?}: no status: {said}{}",
                String::from_utf8_lossy(&done.stderr)
            )
        })
        .to_owned();
    (status, said)
}

/// Each way into a stage: the command, and where it writes, by what its `--out` lies under.
fn every_way_in(root: &Path) -> Vec<Vec<String>> {
    let out = |name: &str| root.join(name).display().to_string();
    let command = |verb: &str, at: String, extra: &[&str]| {
        let mut args = vec![verb.to_owned(), description(), "--out".to_owned(), at];
        args.extend(extra.iter().map(|word| (*word).to_owned()));
        args
    };
    vec![
        command("build", out("x"), &[]),
        command("build", out("made/on/the/way/x"), &[]),
        command("resolve", out("p"), &[]),
        command("resolve", out("p"), &["--unchecked"]),
        command("resolve", out("made/on/the/way/p"), &[]),
    ]
}

/// After a refusal no other program had a hand in, nothing it made stands: not a stage, not a directory it made on the way, not part of a file. So the next run has nothing to refuse.
fn leaves_nothing(root: &Path, imposed: &str, args: &[String]) {
    let (status, said) = under(imposed, args);
    if status != Status::Ok.slug() {
        let left: Vec<_> = fs::read_dir(root)
            .expect("the root")
            .flatten()
            .map(|entry| entry.path())
            .collect();
        assert!(
            left.is_empty(),
            "{imposed}: {args:?}: {status}: left {left:?}: {said}"
        );
    }
    for entry in fs::read_dir(root).expect("the root").flatten() {
        reopened_and_gone(&entry.path());
    }
}

#[test]
fn no_umask_leaves_anything_of_a_refused_run() {
    let root = scratch("umask");
    for mask in [
        "000", "022", "027", "077", "100", "177", "200", "277", "300", "377", "400", "477", "500",
        "577", "600", "677", "700", "777",
    ] {
        for args in every_way_in(&root) {
            leaves_nothing(&root, &format!("umask {mask}"), &args);
        }
    }
    reopened_and_gone(&root);
}

#[test]
fn no_limit_on_a_file_leaves_anything_of_a_refused_run() {
    let root = scratch("limit");
    // In the shell's blocks of 512 bytes, from no file at all to files cut anywhere among what a build writes.
    for blocks in [0, 1, 2, 4, 8, 16, 32, 64, 128, 256] {
        for args in every_way_in(&root) {
            leaves_nothing(&root, &format!("ulimit -f {blocks}"), &args);
        }
    }
    reopened_and_gone(&root);
}

#[test]
fn a_name_past_what_a_path_holds_leaves_nothing_of_a_refused_run() {
    let root = scratch("long-name");
    // A module's own name is what fits or does not, where the package's names fit: `description/<name>`.
    let long = "n".repeat(200);
    let description = root.join(format!("{long}.eadl"));
    fs::copy(self::description(), &description).expect("a copy");
    let elsewhere = scratch("long-name-out");
    // Deep enough that the package's own names fit and `description/<name>` does not.
    let mut out = elsewhere.clone();
    let limit = common::path_limit(&elsewhere);
    while out.as_os_str().len() + 300 < limit {
        out.push("d".repeat(40));
    }
    fs::create_dir_all(&out).expect("the way");
    let deepest = out.join("x");
    let args = vec![
        "build".to_owned(),
        description.display().to_string(),
        "--out".to_owned(),
        deepest.display().to_string(),
    ];
    let (status, said) = under("true", &args);
    if status != Status::Ok.slug() {
        let left: Vec<_> = fs::read_dir(&out)
            .expect("the place")
            .flatten()
            .map(|entry| entry.path())
            .collect();
        assert!(left.is_empty(), "{status}: left {left:?}: {said}");
    }
    reopened_and_gone(&root);
    reopened_and_gone(&elsewhere);
}

/// What comes to stand at a name the run wrote, while the checker runs.
#[derive(Clone, Copy)]
enum Comes {
    /// Another file there, of the same length, closed.
    AnotherClosed,
    /// Another file there, of the same length.
    Another,
    /// The file edited where it is, to the same length, and closed.
    EditedClosed,
    /// Another file of another length, closed.
    OtherLengthClosed,
}

/// The checker, standing in: it finds `name` changed under it, and writes no record, as it would for what it cannot read.
struct Replacing {
    name: &'static str,
    comes: Comes,
}

impl Launch for Replacing {
    fn launch(
        &self,
        _program: &str,
        args: &[String],
        _out: &mut dyn std::io::Write,
        _err: &mut dyn std::io::Write,
    ) -> Result<Launched, Unlaunched> {
        // `--profile P --result R -- DESCRIPTION PLAN`, as the front hands them over.
        let stage = Path::new(&args[3]).parent().expect("a stage");
        let file = stage.join(self.name);
        let length = fs::metadata(&file).expect("what it wrote").len();
        let same: Vec<u8> = vec![b'm'; usize::try_from(length).expect("small")];
        let closed = |at: &Path| {
            fs::set_permissions(at, fs::Permissions::from_mode(0o000)).expect("its mode");
        };
        // By a rename, so that it is another file from its first instant.
        let another = |bytes: &[u8]| {
            let beside = stage.join("came-from-elsewhere");
            fs::write(&beside, bytes).expect("written");
            fs::rename(&beside, &file).expect("moved");
        };
        match self.comes {
            Comes::AnotherClosed => {
                another(&same);
                closed(&file);
            }
            Comes::Another => another(&same),
            Comes::EditedClosed => {
                std::thread::sleep(std::time::Duration::from_millis(5));
                fs::write(&file, &same).expect("edited");
                closed(&file);
            }
            Comes::OtherLengthClosed => {
                another(b"mine");
                closed(&file);
            }
        }
        Ok(Launched {
            ended: Ended::Code(Status::ToolFailure.code()),
            sha256: checker_digest(),
            front: front_digest(),
        })
    }
}

#[test]
fn what_comes_at_a_name_a_run_wrote_stays_and_is_said_however_like_it_it_is() {
    for (verb, out, stage, name) in [
        ("build", "x", "x.building", archogen_emit::ASSIGNMENT_FILE),
        ("resolve", "p", "p.part", "plan"),
    ] {
        for (shape, comes) in [
            ("another-closed", Comes::AnotherClosed),
            ("another", Comes::Another),
            ("edited-closed", Comes::EditedClosed),
            ("other-length-closed", Comes::OtherLengthClosed),
        ] {
            let root = scratch(&format!("{verb}-{shape}"));
            let (status, written, err) = invoke(
                &[
                    verb,
                    &description(),
                    "--out",
                    &root.join(out).display().to_string(),
                ],
                &Replacing { name, comes },
            );
            let said = format!("{written}{err}");
            assert_eq!(status, Status::ToolFailure, "{verb} {shape}: {said}");
            let file = root.join(stage).join(name);
            assert!(
                file.is_file() || fs::symlink_metadata(&file).is_ok(),
                "{verb} {shape}: {said}"
            );
            fs::set_permissions(&file, fs::Permissions::from_mode(0o600)).expect("reopened");
            let bytes = fs::read(&file).expect("kept");
            assert!(
                bytes == b"mine" || bytes.iter().all(|byte| *byte == b'm'),
                "{verb} {shape}: {said}"
            );
            assert!(
                said.contains("the next run refuses while it stands"),
                "{verb} {shape}: {said}"
            );
            reopened_and_gone(&root);
        }
    }
}

/// What stands at a name once the checker has ended.
#[derive(Clone, Copy)]
enum Put {
    Pipe,
    Link,
}

/// The checker, judging, and then what no reader should wait on or follow at `name`.
struct PuttingAfter {
    name: &'static str,
    put: Put,
}

impl Launch for PuttingAfter {
    fn launch(
        &self,
        program: &str,
        args: &[String],
        out: &mut dyn std::io::Write,
        err: &mut dyn std::io::Write,
    ) -> Result<Launched, Unlaunched> {
        let launched = Checker.launch(program, args, out, err);
        let stage = Path::new(&args[3]).parent().expect("a stage");
        let file = stage.join(self.name);
        match self.put {
            Put::Pipe => {
                fs::remove_file(&file).expect("what it wrote");
                let done = Command::new("mkfifo").arg(&file).status().expect("mkfifo");
                assert!(done.success());
            }
            Put::Link => {
                let beside = stage.join("came-from-elsewhere");
                fs::rename(&file, &beside).expect("moved");
                symlink(&beside, &file).expect("a link");
            }
        }
        launched
    }
}

#[test]
fn what_is_no_file_at_a_name_is_refused_rather_than_waited_on_or_followed() {
    for (verb, out, stage, name, put) in [
        ("resolve", "p", "p.part", "plan", Put::Pipe),
        ("resolve", "p", "p.part", "plan", Put::Link),
        ("resolve", "p", "p.part", RECORD, Put::Pipe),
        ("resolve", "p", "p.part", RECORD, Put::Link),
        ("build", "x", "x.building", RECORD, Put::Pipe),
        ("build", "x", "x.building", RECORD, Put::Link),
    ] {
        let root = scratch(&format!("no-file-{verb}-{name}"));
        let at = root.join(out);
        let (status, written, err) = invoke(
            &[verb, &description(), "--out", &at.display().to_string()],
            &PuttingAfter { name, put },
        );
        let said = format!("{written}{err}");
        assert_eq!(status, Status::ToolFailure, "{verb} {name}: {said}");
        assert!(
            fs::symlink_metadata(&at).is_err(),
            "{verb} {name}: nothing lands: {said}"
        );
        assert!(root.join(stage).is_dir(), "{verb} {name}: {said}");
        assert!(
            said.contains("the next run refuses"),
            "{verb} {name}: {said}"
        );
        reopened_and_gone(&root);
    }
}

#[test]
fn a_path_past_what_a_path_holds_is_said_as_that_and_not_as_no_description() {
    let root = scratch("past-the-limit");
    let limit = common::path_limit(&root);
    let path = common::a_path_of_length(&root, limit);
    let (status, _, err) = invoke(
        &[
            "explain",
            &path.display().to_string(),
            "--requirement",
            "observable-output",
        ],
        &Checker,
    );
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("too long for this host"), "{err}");
    reopened_and_gone(&root);
}
