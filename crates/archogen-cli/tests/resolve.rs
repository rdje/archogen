//! `archogen resolve` (leaf `M3.4.4`): one system searched, the plan written whole, and the configuration checker run
//! over it through the launcher the caller hands over — here a fake that writes the record the case asks for, where
//! the front told it to, as `archogen-plan-check` would — so the front's half is held without a process: the plan,
//! the command's status the checker's verdict, the downgrade `--unchecked` says it is, and every refusal before a plan
//! exists writing none.

#[allow(dead_code)]
mod common;

use std::cell::RefCell;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_cli::consume::PLAN_CHECKER;
use archogen_cli::launch::{Ended, Launch, Launched, Unlaunched};
use archogen_cli::{run_with_input, Status};
use archogen_digest::Digest;
use archogen_outcome::record::{Finding, Judged, Label, Producer, Record, Severity, Subject};

/// The checker's sha256 as the launcher verified it.
fn verified() -> Digest {
    Digest::of(b"the checker that ran")
}

/// A fresh scratch directory under the target directory, one per case.
fn scratch() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "resolve-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("two levels below the root")
        .to_path_buf()
}

/// How the fake checker answers: the status its record and its exit code give.
#[derive(Clone)]
enum Answer {
    /// A record of this status, and the exit code it carries.
    Record(Status),
    /// The launcher could not start it.
    Unlaunched,
    /// It ran and wrote no record, as when the place given it cannot take one.
    NoRecord,
    /// It reached its memory ceiling, and wrote none.
    AtTheCeiling,
    /// A record whose status is not what it exits with.
    Unsound,
    /// It accepted, and this directory was closed as it did.
    Closing(PathBuf),
    /// It accepted, and something came in the stage beside its record.
    Stray,
    /// It accepted, and the stage it wrote in was swapped for a link to where it went.
    Swapping,
    /// It accepted, and the plan it judged was edited.
    Editing,
    /// It accepted, and the plan it judged was taken away, and a directory holding a file put at its name.
    Removing,
    /// It wrote its record, and the stage was closed to this user.
    ClosingStage,
    /// It wrote its record, and the directory the stage lies in was closed to this user.
    ClosingParent,
    /// Something came in the stage, and it wrote no record.
    StrayNoRecord,
    /// Something came at its record's own name, so it refused and wrote none.
    AtTheRecord,
    /// It wrote its record, and another directory came to stand at the stage's name, the record gone with what moved.
    Replacing,
}

/// A launcher standing in for `archogen-plan-check`: it reads the plan the front names, writes a record about it where
/// the front told it to, and ends with the record's code.
struct Checker {
    answer: Answer,
    asked: RefCell<Vec<Vec<String>>>,
}

impl Checker {
    fn answering(answer: Answer) -> Self {
        Self {
            answer,
            asked: RefCell::new(Vec::new()),
        }
    }
}

impl Launch for Checker {
    fn launch(
        &self,
        program: &str,
        args: &[String],
        _out: &mut dyn Write,
        err: &mut dyn Write,
    ) -> Result<Launched, Unlaunched> {
        assert_eq!(program, PLAN_CHECKER);
        self.asked.borrow_mut().push(args.to_vec());
        let status = match self.answer {
            Answer::Unlaunched => {
                return Err(Unlaunched {
                    message: "no install beside the front".to_owned(),
                    repair: "install the toolchain".to_owned(),
                })
            }
            Answer::Record(status) => status,
            Answer::Unsound
            | Answer::Stray
            | Answer::Swapping
            | Answer::Editing
            | Answer::Removing
            | Answer::ClosingStage
            | Answer::ClosingParent
            | Answer::Replacing => Status::Ok,
            Answer::Closing(ref dir) => {
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    std::fs::set_permissions(dir, std::fs::Permissions::from_mode(0o555))
                        .expect("its mode");
                }
                let _ = dir;
                Status::Ok
            }
            Answer::AtTheRecord => {
                std::fs::write(&args[3], "mine").expect("written");
                return Ok(Launched {
                    ended: Ended::Code(Status::ToolFailure.code()),
                    sha256: verified(),
                    front: Digest::of(b"the front that ran it"),
                });
            }
            Answer::StrayNoRecord => {
                // `--profile P --result R -- DESCRIPTION PLAN`, as the front hands them over.
                let stage = Path::new(&args[3]).parent().expect("a stage");
                std::fs::write(stage.join("mine.txt"), "mine").expect("written");
                return Ok(Launched {
                    ended: Ended::Code(Status::ToolFailure.code()),
                    sha256: verified(),
                    front: Digest::of(b"the front that ran it"),
                });
            }
            Answer::NoRecord => {
                return Ok(Launched {
                    ended: Ended::Code(Status::ToolFailure.code()),
                    sha256: verified(),
                    front: Digest::of(b"the front that ran it"),
                })
            }
            Answer::AtTheCeiling => {
                return Ok(Launched {
                    ended: Ended::Code(Status::AnalysisInconclusive.code()),
                    sha256: verified(),
                    front: Digest::of(b"the front that ran it"),
                })
            }
        };
        // `--profile P --result R -- DESCRIPTION PLAN`, as the front hands them over.
        let (profile, result, plan) = (&args[1], &args[3], &args[6]);
        let text = std::fs::read_to_string(plan).expect("the plan the front wrote");
        let program = text
            .lines()
            .find_map(|line| line.strip_prefix("(program \"")?.strip_suffix("\")"))
            .expect("the plan names its program");
        let findings = if status == Status::Ok {
            Vec::new()
        } else {
            vec![Finding {
                code: "plan-unsatisfied".to_owned(),
                severity: Severity::Error,
                message: "a binding the relation does not satisfy".to_owned(),
                primary: Label {
                    source: "plan.json".to_owned(),
                    line: 5,
                    column: 1,
                    message: "here".to_owned(),
                },
                secondary: Vec::new(),
                repair: "resolve again".to_owned(),
            }]
        };
        let record = Record::new(
            Producer {
                name: PLAN_CHECKER.to_owned(),
                sha256: verified(),
            },
            Subject {
                program: Digest::parse(&format!("sha256:{program}")).expect("a digest"),
                judged: Judged::Plan(Digest::of(text.as_bytes())),
            },
            profile.clone(),
            status,
            findings,
        )
        .expect("a record");
        std::fs::write(result, record.write()).expect("the record written");
        let stage = Path::new(result).parent().expect("a stage");
        match self.answer {
            Answer::Stray => std::fs::write(stage.join("mine.txt"), "mine").expect("written"),
            Answer::Editing => std::fs::write(stage.join("plan"), "mine").expect("written"),
            #[cfg(unix)]
            Answer::ClosingStage => {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(stage, std::fs::Permissions::from_mode(0o000))
                    .expect("its mode");
            }
            #[cfg(unix)]
            Answer::ClosingParent => {
                use std::os::unix::fs::PermissionsExt;
                std::fs::set_permissions(
                    stage.parent().expect("a parent"),
                    std::fs::Permissions::from_mode(0o000),
                )
                .expect("its mode");
            }
            Answer::Removing => {
                std::fs::remove_file(stage.join("plan")).expect("removed");
                std::fs::create_dir(stage.join("plan")).expect("a directory");
                std::fs::write(stage.join("plan/mine.txt"), "mine").expect("written");
            }
            Answer::Replacing => {
                std::fs::rename(stage, stage.with_extension("moved")).expect("moved");
                std::fs::create_dir(stage).expect("a directory");
                std::fs::write(stage.join("mine.txt"), "mine").expect("written");
            }
            #[cfg(unix)]
            Answer::Swapping => {
                let moved = stage.with_extension("moved");
                std::fs::rename(stage, &moved).expect("moved");
                std::os::unix::fs::symlink(&moved, stage).expect("a link");
            }
            _ => {}
        }
        let _ = writeln!(err, "{PLAN_CHECKER}: {}", status.slug());
        Ok(Launched {
            ended: Ended::Code(if matches!(self.answer, Answer::Unsound) {
                Status::ToolFailure.code()
            } else {
                status.code()
            }),
            sha256: verified(),
            front: Digest::of(b"the front that ran it"),
        })
    }
}

/// Run `archogen resolve` with `args` through `launcher`: the status, standard output and standard error.
fn resolve(args: &[&str], launcher: &dyn Launch) -> (Status, String, String) {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let status = run_with_input(
        std::iter::once("resolve").chain(args.iter().copied()),
        &mut std::io::empty(),
        launcher,
        &mut out,
        &mut err,
    );
    (
        status,
        String::from_utf8(out).expect("UTF-8"),
        String::from_utf8(err).expect("UTF-8"),
    )
}

fn heartbeat() -> String {
    root()
        .join("examples/s0-heartbeat/system.eadl")
        .display()
        .to_string()
}

#[test]
fn a_resolved_plan_is_checked_and_the_checker_s_verdict_is_the_status() {
    let dir = scratch();
    let plan = dir.join("out/plan.json");
    let checker = Checker::answering(Answer::Record(Status::Ok));
    let (status, out, err) = resolve(&[&heartbeat(), "--out", plan.to_str().unwrap()], &checker);
    assert_eq!(status, Status::Ok, "{err}");
    let text = std::fs::read_to_string(&plan).expect("the plan is written, its directory made");
    assert!(text.starts_with(
        "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system heartbeat)\n"
    ));
    assert!(
        text.ends_with("(bind console.write observable-output console.uart)\n"),
        "{text}"
    );
    assert!(out.contains("resolved system `heartbeat`"), "{out}");
    assert!(
        out.contains("experimental:") && out.contains("completed by M3.7"),
        "{out}"
    );
    assert!(
        out.contains("checked by `archogen-plan-check`: accepted"),
        "{out}"
    );
    assert!(dir.join("out/plan.json.check/plan-check.json").is_file());
    let asked = checker.asked.borrow();
    assert_eq!(asked.len(), 1);
    assert_eq!(asked[0][0..2], ["--profile", "rt-static-up-v1"]);
    assert!(
        asked[0][3].ends_with("plan.json.part/plan-check.json"),
        "{:?}",
        asked[0]
    );
}

/// `archogen resolve` of `description` at `plan`, with a checker answering `answer`, `extra` arguments added: its status, and all
/// it said.
fn resolve_of(
    description: &str,
    plan: &str,
    extra: &[&str],
    answer: Answer,
) -> Option<(Status, String)> {
    let checker = Checker::answering(answer);
    let mut args = vec![description.to_owned()];
    args.extend(["--out".to_owned(), plan.to_owned()]);
    args.extend(extra.iter().map(ToString::to_string));
    let args: Vec<&str> = args.iter().map(String::as_str).collect();
    let (status, out, err) = resolve(&args, &checker);
    Some((status, format!("{out}{err}")))
}

/// `archogen resolve` of the heartbeat at `plan`, with a checker that accepts.
fn resolve_at(plan: &str, extra: &[&str]) -> Option<(Status, String)> {
    resolve_of(&heartbeat(), plan, extra, Answer::Record(Status::Ok))
}

/// Whether a closed mode binds whoever runs this: not for root, for whom a closed directory still takes a file.
#[cfg(unix)]
fn modes_bind(root: &Path) -> bool {
    use std::os::unix::fs::PermissionsExt;
    let closed = root.join("does-a-mode-bind");
    std::fs::create_dir_all(&closed).expect("a directory");
    std::fs::set_permissions(&closed, std::fs::Permissions::from_mode(0o555)).expect("its mode");
    let binds = std::fs::write(closed.join("a-file"), "").is_err();
    std::fs::set_permissions(&closed, std::fs::Permissions::from_mode(0o755)).expect("its mode");
    binds
}

/// An earlier plan, as the engine's own reader of the format takes one.
const AN_EARLIER_PLAN: &str = "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system earlier)\n\
     (program \"0000000000000000000000000000000000000000000000000000000000000000\")\n(adapters adapters/1)\n";

/// What stands where the tests run, at any depth, to hold that a name refused makes nothing: a stage for `.` would be a name here.
#[cfg(unix)]
fn all_here() -> std::collections::BTreeMap<String, Vec<u8>> {
    common::everything_under(Path::new("."))
}

/// An earlier plan at `plan`.
#[cfg(unix)]
fn an_earlier_plan(plan: &Path) {
    std::fs::write(plan, AN_EARLIER_PLAN).expect("written");
}

/// A checked plan at `plan`, with its record, and that record's bytes.
#[cfg(unix)]
fn a_checked_plan(plan: &Path) -> Vec<u8> {
    let (status, said) = resolve_at(&plan.display().to_string(), &[]).expect("the route runs");
    assert_eq!(status, Status::Ok, "{said}");
    let mut record = plan.as_os_str().to_owned();
    record.push(".check/plan-check.json");
    std::fs::read(PathBuf::from(record)).expect("its record")
}

/// One route of `docs/book/src/cli.md`'s table: what it sets up under the directory it is given, runs and leaves. `None` where a
/// closed mode binds nobody, so the route cannot be taken.
#[cfg(unix)]
type Route = fn(&Path) -> Option<(Status, String)>;

/// Each route of that table, by the name its row gives it.
#[cfg(unix)]
fn plan_routes() -> Vec<(&'static str, Route)> {
    vec![
        ("no-name", |_| {
            let before = all_here();
            let outcome = resolve_at(".", &[]);
            assert_eq!(all_here(), before, "something was made");
            outcome
        }),
        ("empty-name", |_| {
            let before = all_here();
            let outcome = resolve_at("", &[]);
            assert_eq!(all_here(), before, "something was made");
            outcome
        }),
        ("a-trailing-dot", |root| {
            let before = common::everything_under(root);
            let outcome = resolve_at(&format!("{}/.", root.join("q").display()), &[]);
            assert_eq!(common::everything_under(root), before, "something was made");
            outcome
        }),
        ("a-trailing-separator", |root| {
            let before = common::everything_under(root);
            let outcome = resolve_at(&format!("{}/", root.join("q").display()), &[]);
            assert_eq!(common::everything_under(root), before, "something was made");
            outcome
        }),
        ("back-through", |root| {
            std::fs::create_dir(root.join("d")).expect("a directory");
            let before = common::everything_under(root);
            let outcome = resolve_at(&root.join("missing/../d").display().to_string(), &[]);
            assert_eq!(common::everything_under(root), before, "something was made");
            outcome
        }),
        ("a-directory", |root| {
            std::fs::create_dir(root.join("d")).expect("a directory");
            let outcome = resolve_at(&root.join("d").display().to_string(), &[]);
            assert!(!root.join("d.part").exists());
            outcome
        }),
        ("a-link", |root| {
            std::fs::write(root.join("target"), "mine").expect("written");
            std::os::unix::fs::symlink(root.join("target"), root.join("p")).expect("a link");
            let outcome = resolve_at(&root.join("p").display().to_string(), &[]);
            assert!(std::fs::symlink_metadata(root.join("p"))
                .expect("kept")
                .is_symlink());
            assert_eq!(
                std::fs::read_to_string(root.join("target")).expect("kept"),
                "mine"
            );
            outcome
        }),
        ("a-pipe", |root| {
            let pipe = root.join("p");
            let made = std::process::Command::new("mkfifo")
                .arg(&pipe)
                .status()
                .expect("mkfifo runs");
            assert!(made.success());
            let outcome = resolve_at(&pipe.display().to_string(), &[]);
            assert!(!std::fs::metadata(&pipe).expect("kept").is_file());
            outcome
        }),
        ("no-plan", |root| {
            std::fs::write(root.join("p"), "mine").expect("written");
            let outcome = resolve_at(&root.join("p").display().to_string(), &[]);
            assert_eq!(
                std::fs::read_to_string(root.join("p")).expect("kept"),
                "mine"
            );
            assert!(!root.join("p.part").exists() && !root.join("p.check").exists());
            outcome
        }),
        ("a-plan-with-notes-in-it", |root| {
            // Every shape of a note that leaves a plan no longer in canonical form: what is written there is someone's, wherever it is. One that keeps
            // the form cannot be told from the search's own, `a-canonical-edit`'s row.
            let plan = root.join("p");
            let mut outcome = None;
            for notes in [
                format!("{AN_EARLIER_PLAN}; three weeks of notes\n"),
                format!("; three weeks of notes\n{AN_EARLIER_PLAN}"),
                // A line out of its sorted place, which holds nothing a comment does.
                format!("{AN_EARLIER_PLAN}(bind z.z observable-output console.uart)\n(bind a.a observable-output console.uart)\n"),
                AN_EARLIER_PLAN.replace(
                    "(system earlier)",
                    "(system earlier) ; chosen by hand (see Ana)",
                ),
                format!("{AN_EARLIER_PLAN}(note three weeks of them)\n"),
                AN_EARLIER_PLAN.replace(")\n(adapters", ") (adapters"),
                "(format \"archogen-assignment/2\")\n(my notes, in parentheses)\n".to_owned(),
                "(format \"archogen-assignment/1\")\n".to_owned(),
            ] {
                std::fs::write(&plan, &notes).expect("written");
                outcome = resolve_at(&plan.display().to_string(), &[]);
                assert_eq!(std::fs::read_to_string(&plan).expect("kept"), notes);
                let (status, _) = outcome.as_ref().expect("the route runs");
                assert_eq!(*status, Status::Usage, "{notes}");
            }
            outcome
        }),
        ("a-canonical-edit", |root| {
            // What cannot be told from the search's own plan. The chapter says so, and this holds what happens to it.
            let plan = root.join("p");
            let edited =
                format!("{AN_EARLIER_PLAN}(bind console.write observable-output console.alt)\n");
            assert!(archogen_api::is_a_plan(&edited), "{edited}");
            std::fs::write(&plan, &edited).expect("written");
            let outcome = resolve_at(&plan.display().to_string(), &[]);
            let written = std::fs::read_to_string(&plan).expect("a plan");
            assert_ne!(written, edited);
            // One binding for another, as well as one added.
            let replaced = written.replace("console.uart", "console.alt");
            assert_ne!(
                replaced, written,
                "a plan binding nothing tests nothing: {written}"
            );
            assert!(archogen_api::is_a_plan(&replaced), "{replaced}");
            std::fs::write(&plan, &replaced).expect("written");
            resolve_at(&plan.display().to_string(), &[])?;
            assert_eq!(std::fs::read_to_string(&plan).expect("a plan"), written);
            outcome
        }),
        ("a-plan-past-what-its-reader-holds", |root| {
            // Past what the engine's reader holds, and told without reading it: a sparse terabyte beginning as a plan.
            let plan = root.join("p");
            std::fs::write(&plan, "(format \"archogen-assignment/").expect("written");
            let grown = std::fs::OpenOptions::new()
                .write(true)
                .open(&plan)
                .expect("opened");
            grown.set_len(1 << 40).expect("a sparse file");
            drop(grown);
            let outcome = resolve_at(&plan.display().to_string(), &[]);
            assert_eq!(std::fs::metadata(&plan).expect("kept").len(), 1 << 40);
            assert!(!root.join("p.part").exists() && !root.join("p.check").exists());
            outcome
        }),
        ("closed-to-reads", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            let plan = root.join("p");
            an_earlier_plan(&plan);
            std::fs::set_permissions(&plan, std::fs::Permissions::from_mode(0o000))
                .expect("its mode");
            let outcome = resolve_at(&plan.display().to_string(), &[]);
            std::fs::set_permissions(&plan, std::fs::Permissions::from_mode(0o600))
                .expect("its mode");
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            outcome
        }),
        ("its-own-description", |root| {
            let copy = root.join("s.eadl");
            std::fs::copy(heartbeat(), &copy).expect("copied");
            let before = std::fs::read(&copy).expect("read");
            let name = copy.display().to_string();
            for extra in [&[][..], &["--unchecked"][..]] {
                let (status, _) = resolve_of(&name, &name, extra, Answer::Record(Status::Ok))?;
                assert_eq!(status, Status::Usage);
                assert_eq!(std::fs::read(&copy).expect("kept"), before);
            }
            resolve_of(&name, &name, &[], Answer::Record(Status::Ok))
        }),
        ("its-own-description-by-a-descriptor", |root| {
            // However the description is named, what stands at `--out` is no plan.
            let copy = root.join("s.eadl");
            std::fs::copy(heartbeat(), &copy).expect("copied");
            let before = std::fs::read(&copy).expect("read");
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg("\"$0\" resolve /dev/fd/3 --out s.eadl --unchecked 3< s.eadl")
                .arg(env!("CARGO_BIN_EXE_archogen"))
                .current_dir(root)
                .output()
                .expect("the shell runs");
            assert_eq!(std::fs::read(&copy).expect("kept"), before, "{output:?}");
            assert_eq!(
                output.status.code(),
                Some(Status::Usage.code()),
                "{output:?}"
            );
            Some((
                Status::Usage,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }),
        ("a-module-it-imports", |root| {
            let corpus = crate::root().join("crates/archogen-plan/tests/corpus/modules");
            let main = root.join("app.main.eadl");
            for entry in std::fs::read_dir(&corpus).expect("the corpus") {
                let entry = entry.expect("an entry");
                std::fs::copy(entry.path(), root.join(entry.file_name())).expect("copied");
            }
            let module = root.join("hw.dev.eadl");
            let before = std::fs::read(&module).expect("read");
            let outcome = resolve_of(
                &main.display().to_string(),
                &module.display().to_string(),
                &[],
                Answer::Record(Status::Ok),
            );
            assert_eq!(std::fs::read(&module).expect("kept"), before);
            outcome
        }),
        ("reads-differently", |root| {
            // `/dev/stdin` reads empty the second time here; where a platform reopens it, this route is not taken.
            if !cfg!(target_os = "macos") {
                return None;
            }
            std::fs::copy(heartbeat(), root.join("s.eadl")).expect("copied");
            let before = common::everything_under(root);
            let output = std::process::Command::new("sh")
                .arg("-c")
                .arg("\"$0\" resolve /dev/stdin --out p < s.eadl")
                .arg(env!("CARGO_BIN_EXE_archogen"))
                .current_dir(root)
                .output()
                .expect("the shell runs");
            assert_eq!(common::everything_under(root), before, "something was made");
            assert_eq!(
                output.status.code(),
                Some(Status::Usage.code()),
                "{output:?}"
            );
            Some((
                Status::Usage,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }),
        ("a-module-that-reads-once", |root| {
            let corpus = crate::root().join("crates/archogen-plan/tests/corpus/modules");
            for entry in std::fs::read_dir(&corpus).expect("the corpus") {
                let entry = entry.expect("an entry");
                std::fs::copy(entry.path(), root.join(entry.file_name())).expect("copied");
            }
            let module = root.join("hw.dev.eadl");
            let text = std::fs::read(&module).expect("read");
            std::fs::remove_file(&module).expect("removed");
            let made = std::process::Command::new("mkfifo")
                .arg(&module)
                .status()
                .expect("mkfifo runs");
            assert!(made.success());
            let writer = module.clone();
            let before = common::everything_under(root);
            let feeding = std::thread::spawn(move || std::fs::write(writer, text));
            let outcome = resolve_of(
                &root.join("app.main.eadl").display().to_string(),
                &root.join("p").display().to_string(),
                &[],
                Answer::Record(Status::Ok),
            );
            feeding.join().expect("the writer").expect("written");
            assert_eq!(common::everything_under(root), before, "something was made");
            outcome
        }),
        ("reads-once", |root| {
            let pipe = root.join("once.eadl");
            let made = std::process::Command::new("mkfifo")
                .arg(&pipe)
                .status()
                .expect("mkfifo runs");
            assert!(made.success());
            let plan = root.join("p");
            let record = a_checked_plan(&plan);
            let (writer, text) = (pipe.clone(), std::fs::read(heartbeat()).expect("read"));
            let feeding = std::thread::spawn(move || std::fs::write(writer, text));
            let before = std::fs::read(&plan).expect("read");
            let outcome = resolve_of(
                &pipe.display().to_string(),
                &plan.display().to_string(),
                &[],
                Answer::Record(Status::Ok),
            );
            feeding.join().expect("the writer").expect("written");
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(
                std::fs::read(root.join("p.check/plan-check.json")).expect("kept"),
                record
            );
            assert!(!root.join("p.part").exists());
            outcome
        }),
        ("part-stands", |root| {
            std::fs::write(root.join("p.part"), "mine").expect("written");
            let outcome = resolve_at(&root.join("p").display().to_string(), &[]);
            assert_eq!(
                std::fs::read_to_string(root.join("p.part")).expect("kept"),
                "mine"
            );
            assert!(!root.join("p").exists());
            outcome
        }),
        ("check-stands", |root| {
            an_earlier_plan(&root.join("p"));
            std::fs::write(root.join("p.check"), "mine").expect("written");
            let outcome = resolve_at(&root.join("p").display().to_string(), &[]);
            assert_eq!(
                std::fs::read_to_string(root.join("p")).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert_eq!(
                std::fs::read_to_string(root.join("p.check")).expect("kept"),
                "mine"
            );
            outcome
        }),
        ("check-a-link", |root| {
            an_earlier_plan(&root.join("p"));
            std::fs::create_dir(root.join("elsewhere")).expect("a directory");
            std::os::unix::fs::symlink(root.join("elsewhere"), root.join("p.check"))
                .expect("a link");
            let outcome = resolve_at(&root.join("p").display().to_string(), &[]);
            assert!(std::fs::symlink_metadata(root.join("p.check"))
                .expect("kept")
                .is_symlink());
            assert_eq!(
                std::fs::read_to_string(root.join("p")).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert_eq!(
                std::fs::read_dir(root.join("elsewhere"))
                    .expect("read")
                    .count(),
                0
            );
            outcome
        }),
        ("check-a-link-unchecked", |root| {
            // A link is not followed to clear anything: what it points at is not this run's.
            std::fs::create_dir(root.join("other")).expect("a directory");
            let kept = a_checked_plan(&root.join("first"));
            std::fs::write(root.join("other/plan-check.json"), &kept).expect("written");
            std::os::unix::fs::symlink(root.join("other"), root.join("p.check")).expect("a link");
            let outcome = resolve_at(&root.join("p").display().to_string(), &["--unchecked"]);
            assert!(root.join("p").is_file());
            assert_eq!(
                std::fs::read(root.join("other/plan-check.json")).expect("kept"),
                kept
            );
            assert!(std::fs::symlink_metadata(root.join("p.check"))
                .expect("kept")
                .is_symlink());
            outcome
        }),
        ("record-no-record", |root| {
            // A description kept where the record goes, for one.
            std::fs::create_dir(root.join("p.check")).expect("a directory");
            let record = root.join("p.check/plan-check.json");
            std::fs::copy(heartbeat(), &record).expect("copied");
            let before = std::fs::read(&record).expect("read");
            let outcome = resolve_at(&root.join("p").display().to_string(), &[]);
            assert_eq!(std::fs::read(&record).expect("kept"), before);
            assert!(!root.join("p").exists());
            outcome
        }),
        ("record-no-record-unchecked", |root| {
            std::fs::create_dir(root.join("p.check")).expect("a directory");
            let record = root.join("p.check/plan-check.json");
            std::fs::copy(heartbeat(), &record).expect("copied");
            let before = std::fs::read(&record).expect("read");
            let outcome = resolve_at(&root.join("p").display().to_string(), &["--unchecked"]);
            assert_eq!(std::fs::read(&record).expect("kept"), before);
            assert!(root.join("p").is_file());
            outcome
        }),
        ("check-reused", |root| {
            // An earlier record of another description, so that one left in place could not pass for the new one.
            let plan = root.join("p");
            let other =
                crate::root().join("crates/eadl-resolve/tests/search/adapter-epoch-extender.eadl");
            let (status, said) = resolve_of(
                &other.display().to_string(),
                &plan.display().to_string(),
                &[],
                Answer::Record(Status::Ok),
            )?;
            assert_eq!(status, Status::Ok, "{said}");
            let record = std::fs::read(root.join("p.check/plan-check.json")).expect("its record");
            std::fs::write(root.join("p.check/mine.txt"), "mine").expect("written");
            let outcome = resolve_at(&plan.display().to_string(), &[]);
            assert_ne!(
                std::fs::read(root.join("p.check/plan-check.json")).expect("written"),
                record
            );
            assert!(root.join("p.check/mine.txt").is_file());
            assert!(!root.join("p.check/plan-check.json.part").exists());
            assert!(!root.join("p.check/plan-check.json.earlier").exists());
            outcome
        }),
        ("a-record-waiting", |root| {
            std::fs::create_dir(root.join("p.check")).expect("a directory");
            std::fs::write(root.join("p.check/plan-check.json.part"), "mine").expect("written");
            let outcome = resolve_at(&root.join("p").display().to_string(), &[]);
            assert_eq!(
                std::fs::read_to_string(root.join("p.check/plan-check.json.part")).expect("kept"),
                "mine"
            );
            assert!(!root.join("p").exists());
            outcome
        }),
        ("check-closed", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            let plan = root.join("p");
            an_earlier_plan(&plan);
            std::fs::create_dir(root.join("p.check")).expect("a directory");
            std::fs::set_permissions(root.join("p.check"), std::fs::Permissions::from_mode(0o555))
                .expect("its mode");
            let outcome = resolve_at(&plan.display().to_string(), &[]);
            std::fs::set_permissions(root.join("p.check"), std::fs::Permissions::from_mode(0o755))
                .expect("its mode");
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert!(!root.join("p.part").exists());
            outcome
        }),
        ("check-unsearchable", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            let plan = root.join("p");
            let record = a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            std::fs::set_permissions(root.join("p.check"), std::fs::Permissions::from_mode(0o000))
                .expect("its mode");
            let outcome = resolve_at(&plan.display().to_string(), &[]);
            std::fs::set_permissions(root.join("p.check"), std::fs::Permissions::from_mode(0o755))
                .expect("its mode");
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(
                std::fs::read(root.join("p.check/plan-check.json")).expect("kept"),
                record
            );
            assert!(!root.join("p.part").exists());
            outcome
        }),
        ("check-closed-unchecked", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            // A plan with no acceptance may not stand beside a record, and this record cannot be moved aside.
            let plan = root.join("p");
            let record = a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            std::fs::set_permissions(root.join("p.check"), std::fs::Permissions::from_mode(0o555))
                .expect("its mode");
            let outcome = resolve_at(&plan.display().to_string(), &["--unchecked"]);
            std::fs::set_permissions(root.join("p.check"), std::fs::Permissions::from_mode(0o755))
                .expect("its mode");
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(
                std::fs::read(root.join("p.check/plan-check.json")).expect("kept"),
                record
            );
            assert!(!root.join("p.part").exists());
            let (_, said) = outcome.as_ref().expect("the route runs");
            assert!(said.contains("its directory's"), "{said}");
            outcome
        }),
        ("check-unsearchable-unchecked", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            // It cannot tell what it would leave beside the plan, so nothing moves.
            let plan = root.join("p");
            let record = a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            std::fs::set_permissions(root.join("p.check"), std::fs::Permissions::from_mode(0o000))
                .expect("its mode");
            let outcome = resolve_at(&plan.display().to_string(), &["--unchecked"]);
            std::fs::set_permissions(root.join("p.check"), std::fs::Permissions::from_mode(0o755))
                .expect("its mode");
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(
                std::fs::read(root.join("p.check/plan-check.json")).expect("kept"),
                record
            );
            outcome
        }),
        ("record-unreadable", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            // A record this user cannot read, or an empty file at such a mode, whose mode the new record would otherwise take.
            let plan = root.join("p");
            a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            let record = root.join("p.check/plan-check.json");
            let held = std::fs::read(&record).expect("read");
            std::fs::set_permissions(&record, std::fs::Permissions::from_mode(0o000))
                .expect("its mode");
            let outcome = resolve_at(&plan.display().to_string(), &[]);
            std::fs::set_permissions(&record, std::fs::Permissions::from_mode(0o600))
                .expect("its mode");
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(std::fs::read(&record).expect("kept"), held);
            std::fs::write(&record, "").expect("emptied");
            std::fs::set_permissions(&record, std::fs::Permissions::from_mode(0o000))
                .expect("its mode");
            let (status, said) = resolve_at(&plan.display().to_string(), &[])?;
            std::fs::set_permissions(&record, std::fs::Permissions::from_mode(0o600))
                .expect("its mode");
            assert_eq!(status, Status::Usage, "{said}");
            assert!(said.contains("cannot read"), "{said}");
            outcome
        }),
        ("record-unreadable-unchecked", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            let plan = root.join("p");
            a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            let record = root.join("p.check/plan-check.json");
            let held = std::fs::read(&record).expect("read");
            std::fs::set_permissions(&record, std::fs::Permissions::from_mode(0o000))
                .expect("its mode");
            let outcome = resolve_at(&plan.display().to_string(), &["--unchecked"]);
            std::fs::set_permissions(&record, std::fs::Permissions::from_mode(0o600))
                .expect("its mode");
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(std::fs::read(&record).expect("kept"), held);
            assert!(!root.join("p.part").exists());
            outcome
        }),
        ("a-record-keeps-its-mode", |root| {
            use std::os::unix::fs::PermissionsExt;
            let plan = root.join("p");
            a_checked_plan(&plan);
            let record = root.join("p.check/plan-check.json");
            std::fs::set_permissions(&record, std::fs::Permissions::from_mode(0o600))
                .expect("its mode");
            let outcome = resolve_at(&plan.display().to_string(), &[]);
            let kept = std::fs::metadata(&record)
                .expect("it stands")
                .permissions()
                .mode();
            assert_eq!(kept & 0o777, 0o600);
            outcome
        }),
        ("flagged-plan", |root| {
            // A flag stops the last step alone, and every step before it goes back. `chflags` is not on every platform.
            let plan = root.join("p");
            let record = a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            let flagged = std::process::Command::new("chflags")
                .arg("uchg")
                .arg(&plan)
                .status();
            if !flagged.is_ok_and(|status| status.success()) {
                return None;
            }
            let other =
                crate::root().join("crates/eadl-resolve/tests/search/adapter-epoch-extender.eadl");
            let outcome = resolve_of(
                &other.display().to_string(),
                &plan.display().to_string(),
                &[],
                Answer::Record(Status::Ok),
            );
            let _ = std::process::Command::new("chflags")
                .arg("nouchg")
                .arg(&plan)
                .status();
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(
                std::fs::read(root.join("p.check/plan-check.json")).expect("kept"),
                record
            );
            assert!(!root.join("p.part").exists());
            assert!(!root.join("p.check/plan-check.json.part").exists());
            assert!(!root.join("p.check/plan-check.json.earlier").exists());
            outcome
        }),
        ("flagged-plan-unchecked", |root| {
            // What this run did not write stays, a file at the name a record waits at among it.
            let plan = root.join("p");
            let (status, said) = resolve_at(&plan.display().to_string(), &["--unchecked"])?;
            assert_eq!(status, Status::Ok, "{said}");
            std::fs::create_dir(root.join("p.check")).expect("a directory");
            std::fs::write(root.join("p.check/plan-check.json.part"), "mine").expect("written");
            let flagged = std::process::Command::new("chflags")
                .arg("uchg")
                .arg(&plan)
                .status();
            if !flagged.is_ok_and(|status| status.success()) {
                return None;
            }
            let before = std::fs::read(&plan).expect("read");
            let outcome = resolve_at(&plan.display().to_string(), &["--unchecked"]);
            let _ = std::process::Command::new("chflags")
                .arg("nouchg")
                .arg(&plan)
                .status();
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(
                std::fs::read_to_string(root.join("p.check/plan-check.json.part")).expect("kept"),
                "mine"
            );
            outcome
        }),
        ("append-only-check", |root| {
            // `PLAN.check` takes a new name and lets none go: what cannot be taken back is said, with where it is.
            let plan = root.join("p");
            let check = root.join("p.check");
            std::fs::create_dir(&check).expect("a directory");
            let flagged = std::process::Command::new("chflags")
                .arg("uappnd")
                .arg(&check)
                .status();
            if !flagged.is_ok_and(|status| status.success()) {
                return None;
            }
            let outcome = resolve_at(&plan.display().to_string(), &[]);
            let _ = std::process::Command::new("chflags")
                .arg("nouappnd")
                .arg(&check)
                .status();
            assert!(!plan.exists());
            assert!(check.join("plan-check.json.part").is_file());
            let said = outcome.as_ref().map_or("", |(_, said)| said.as_str());
            assert!(said.contains("could not be taken back"), "{said}");
            assert!(said.contains("plan-check.json.part"), "{said}");
            outcome
        }),
        ("flagged-record", |root| {
            // The record's own name found closed before the plan moves: a plan does not land to be refused after.
            let plan = root.join("p");
            let record = a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            let name = root.join("p.check/plan-check.json");
            let flagged = std::process::Command::new("chflags")
                .arg("uchg")
                .arg(&name)
                .status();
            if !flagged.is_ok_and(|status| status.success()) {
                return None;
            }
            let other =
                crate::root().join("crates/eadl-resolve/tests/search/adapter-epoch-extender.eadl");
            let outcome = resolve_of(
                &other.display().to_string(),
                &plan.display().to_string(),
                &[],
                Answer::Record(Status::Ok),
            );
            let _ = std::process::Command::new("chflags")
                .arg("nouchg")
                .arg(&name)
                .status();
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(std::fs::read(&name).expect("kept"), record);
            assert!(!root.join("p.part").exists());
            outcome
        }),
        ("came-in-stage", |root| {
            let plan = root.join("p");
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::Stray,
            );
            assert!(plan.is_file());
            assert!(root.join("p.check/plan-check.json").is_file());
            assert_eq!(
                std::fs::read_to_string(root.join("p.part/mine.txt")).expect("kept"),
                "mine"
            );
            let (status, said) = resolve_at(&plan.display().to_string(), &[])?;
            assert_eq!(status, Status::Usage, "{said}");
            assert!(said.contains("p.part exists"), "{said}");
            outcome
        }),
        ("plan-edited-meanwhile", |root| {
            let plan = root.join("p");
            an_earlier_plan(&plan);
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::Editing,
            );
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert_eq!(
                std::fs::read_to_string(root.join("p.part/plan")).expect("kept"),
                "mine"
            );
            outcome
        }),
        ("closed-while-it-ran", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            let plan = root.join("p");
            an_earlier_plan(&plan);
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::ClosingStage,
            );
            std::fs::set_permissions(root.join("p.part"), std::fs::Permissions::from_mode(0o700))
                .expect("its mode");
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert!(root.join("p.part").is_dir());
            let said = outcome.as_ref().map_or("", |(_, said)| said.as_str());
            assert!(said.contains("could not be removed"), "{said}");
            outcome
        }),
        ("parent-shut-meanwhile", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            let parent = root.join("in");
            std::fs::create_dir(&parent).expect("a directory");
            let plan = parent.join("p");
            an_earlier_plan(&plan);
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::ClosingParent,
            );
            std::fs::set_permissions(&parent, std::fs::Permissions::from_mode(0o755))
                .expect("its mode");
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert!(parent.join("p.part").is_dir());
            let said = outcome.as_ref().map_or("", |(_, said)| said.as_str());
            assert!(said.contains("cannot be looked at"), "{said}");
            outcome
        }),
        ("came-at-the-record", |root| {
            let plan = root.join("p");
            an_earlier_plan(&plan);
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::AtTheRecord,
            );
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert_eq!(
                std::fs::read_to_string(root.join("p.part/plan-check.json")).expect("kept"),
                "mine"
            );
            let said = outcome.as_ref().map_or("", |(_, said)| said.as_str());
            assert!(said.contains("remove the stage"), "{said}");
            outcome
        }),
        ("junk", |root| {
            let file = root.join("junk.eadl");
            std::fs::write(&file, "x\n".repeat(5000)).expect("written");
            let plan = root.join("p");
            let before = common::everything_under(root);
            let outcome = resolve_of(
                &file.display().to_string(),
                &plan.display().to_string(),
                &[],
                Answer::Record(Status::Ok),
            );
            assert_eq!(common::everything_under(root), before, "something was made");
            outcome
        }),
        ("many-unmet", |root| {
            // A search says no more than a check does, of what nothing meets.
            let file = root.join("u.eadl");
            let mut text = String::from("(eadl-version eadl/1)\n(defblock blk (offers (bus-width 8 bit) (reachable-at-privilege machine)))\n(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses blk)))\n");
            let mut uses = String::new();
            for i in 0..1500 {
                text.push_str(&format!(
                    "(defservice svc{i} (requires (needs bus-width) (bus-width 64 bit)))\n"
                ));
                uses.push_str(&format!("(uses svc{i}) "));
            }
            text.push_str(&format!("(defsystem m (task t (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 1) (uses svc0) (on-overrun fault)) (requires {uses}) (platform (uses soc)))\n"));
            std::fs::write(&file, text).expect("written");
            let plan = root.join("p");
            let before = common::everything_under(root);
            let outcome = resolve_of(
                &file.display().to_string(),
                &plan.display().to_string(),
                &[],
                Answer::Record(Status::Ok),
            );
            assert_eq!(common::everything_under(root), before, "something was made");
            outcome
        }),
        ("came-on-a-refusal", |root| {
            // What came stays, with the stage, and it is said; what this run wrote goes.
            let plan = root.join("p");
            an_earlier_plan(&plan);
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::StrayNoRecord,
            );
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert_eq!(
                std::fs::read_to_string(root.join("p.part/mine.txt")).expect("kept"),
                "mine"
            );
            assert!(
                !root.join("p.part/plan").exists(),
                "what this run wrote stays"
            );
            let said = outcome.as_ref().map_or("", |(_, said)| said.as_str());
            assert!(said.contains("holds what this run did not write"), "{said}");
            outcome
        }),
        ("plan-gone-meanwhile", |root| {
            // What came is not this run's: the stage stays, with it, for whoever looks.
            let plan = root.join("p");
            an_earlier_plan(&plan);
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::Removing,
            );
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert_eq!(
                std::fs::read_to_string(root.join("p.part/plan/mine.txt")).expect("kept"),
                "mine"
            );
            outcome
        }),
        ("stage-replaced", |root| {
            // Another directory now at the stage's name, this run's gone from under it: what came is not this run's to remove.
            let plan = root.join("p");
            an_earlier_plan(&plan);
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::Replacing,
            );
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert_eq!(
                std::fs::read_to_string(root.join("p.part/mine.txt")).expect("kept"),
                "mine"
            );
            outcome
        }),
        ("stage-swapped", |root| {
            let plan = root.join("p");
            an_earlier_plan(&plan);
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::Swapping,
            );
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert!(std::fs::symlink_metadata(root.join("p.part"))
                .expect("kept")
                .is_symlink());
            outcome
        }),
        ("check-stands-unchecked", |root| {
            std::fs::write(root.join("p.check"), "mine").expect("written");
            let outcome = resolve_at(&root.join("p").display().to_string(), &["--unchecked"]);
            assert!(root.join("p").is_file());
            assert_eq!(
                std::fs::read_to_string(root.join("p.check")).expect("kept"),
                "mine"
            );
            outcome
        }),
        ("no-room-name", |root| {
            let long = root.join("a/b").join("x".repeat(252));
            let before = common::everything_under(root);
            let outcome = resolve_at(&long.display().to_string(), &[]);
            assert_eq!(common::everything_under(root), before, "something was made");
            outcome
        }),
        ("no-room-check", |root| {
            let plan = root.join("x".repeat(250));
            an_earlier_plan(&plan);
            let outcome = resolve_at(&plan.display().to_string(), &[]);
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            outcome
        }),
        ("no-room-check-unchecked", |root| {
            // `--unchecked` needs no `.check`, so a name with no room for one is no refusal.
            let plan = root.join("x".repeat(250));
            let outcome = resolve_at(&plan.display().to_string(), &["--unchecked"]);
            assert!(plan.is_file());
            outcome
        }),
        ("no-room-record", |root| {
            // Room for the plan, for its directory and for its record, and none for the name the checker writes it at first.
            let limit = common::path_limit(root);
            let parent = common::a_path_of_length(root, limit - 26);
            let outcome = resolve_at(&parent.join("p").display().to_string(), &[]);
            assert_eq!(std::fs::read_dir(root).expect("a directory").count(), 0);
            outcome
        }),
        ("closed-parent", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            let closed = root.join("closed");
            std::fs::create_dir(&closed).expect("a directory");
            std::fs::set_permissions(&closed, std::fs::Permissions::from_mode(0o555))
                .expect("its mode");
            let outcome = resolve_at(&closed.join("a/p").display().to_string(), &[]);
            assert_eq!(std::fs::read_dir(&closed).expect("a directory").count(), 0);
            std::fs::set_permissions(&closed, std::fs::Permissions::from_mode(0o755))
                .expect("its mode");
            outcome
        }),
        ("file-on-the-way", |root| {
            std::fs::write(root.join("f"), "mine").expect("written");
            let outcome = resolve_at(&root.join("f/x").display().to_string(), &[]);
            assert_eq!(
                std::fs::read_to_string(root.join("f")).expect("kept"),
                "mine"
            );
            outcome
        }),
        ("link-on-the-way", |root| {
            std::os::unix::fs::symlink("nowhere", root.join("dangling")).expect("a link");
            let outcome = resolve_at(&root.join("dangling/x").display().to_string(), &[]);
            assert!(std::fs::symlink_metadata(root.join("dangling"))
                .expect("kept")
                .is_symlink());
            outcome
        }),
        ("link-to-a-directory-on-the-way", |root| {
            std::fs::create_dir(root.join("real")).expect("a directory");
            std::os::unix::fs::symlink(root.join("real"), root.join("link")).expect("a link");
            let outcome = resolve_at(&root.join("link/p").display().to_string(), &[]);
            assert!(root.join("real/p").is_file());
            outcome
        }),
        ("link-loop-on-the-way", |root| {
            std::os::unix::fs::symlink(root.join("b"), root.join("a")).expect("a link");
            std::os::unix::fs::symlink(root.join("a"), root.join("b")).expect("a link");
            let outcome = resolve_at(&root.join("a/p").display().to_string(), &[]);
            for link in ["a", "b"] {
                assert!(std::fs::symlink_metadata(root.join(link))
                    .expect("kept")
                    .is_symlink());
            }
            outcome
        }),
        ("absent", |root| {
            let outcome = resolve_at(&root.join("a/b/p").display().to_string(), &[]);
            assert!(root.join("a/b/p").is_file());
            assert!(root.join("a/b/p.check/plan-check.json").is_file());
            assert!(!root.join("a/b/p.part").exists());
            outcome
        }),
        ("over-an-empty-file", |root| {
            std::fs::write(root.join("p"), "").expect("written");
            let outcome = resolve_at(&root.join("p").display().to_string(), &[]);
            assert!(std::fs::metadata(root.join("p")).expect("it stands").len() > 0);
            outcome
        }),
        ("over-a-plan", |root| {
            use std::os::unix::fs::PermissionsExt;
            let mut outcome = None;
            for mode in [0o640, 0o600, 0o400] {
                let plan = root.join(format!("p{mode:o}"));
                an_earlier_plan(&plan);
                std::fs::set_permissions(&plan, std::fs::Permissions::from_mode(mode))
                    .expect("its mode");
                outcome = resolve_at(&plan.display().to_string(), &[]);
                let kept = std::fs::metadata(&plan)
                    .expect("it stands")
                    .permissions()
                    .mode();
                assert_eq!(kept & 0o777, mode);
                assert_ne!(
                    std::fs::read_to_string(&plan).expect("written"),
                    AN_EARLIER_PLAN
                );
            }
            outcome
        }),
        ("unchecked-over-a-checked", |root| {
            let plan = root.join("p");
            a_checked_plan(&plan);
            let outcome = resolve_at(&plan.display().to_string(), &["--unchecked"]);
            assert!(!root.join("p.check/plan-check.json").exists());
            assert!(!root.join("p.check/plan-check.json.part").exists());
            outcome
        }),
        ("refused", |root| {
            let plan = root.join("p");
            let record = a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::Record(Status::ToolFailure),
            );
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(
                std::fs::read(root.join("p.check/plan-check.json")).expect("kept"),
                record
            );
            assert!(root.join("p.part/plan").is_file());
            assert!(root.join("p.part/plan-check.json").is_file());
            outcome
        }),
        ("record-unsound", |root| {
            let plan = root.join("p");
            an_earlier_plan(&plan);
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::Unsound,
            );
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert!(root.join("p.part/plan").is_file());
            assert!(root.join("p.part/plan-check.json").is_file());
            outcome
        }),
        ("no-record", |root| {
            let plan = root.join("p");
            let record = a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::NoRecord,
            );
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(
                std::fs::read(root.join("p.check/plan-check.json")).expect("kept"),
                record
            );
            assert!(!root.join("p.part").exists());
            outcome
        }),
        ("at-the-ceiling", |root| {
            let plan = root.join("p");
            let record = a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::AtTheCeiling,
            );
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(
                std::fs::read(root.join("p.check/plan-check.json")).expect("kept"),
                record
            );
            assert!(!root.join("p.part").exists());
            outcome
        }),
        ("line-break-in-the-whole-path", |root| {
            let real = root.join("a\nb");
            std::fs::create_dir(&real).expect("a directory");
            std::fs::copy(heartbeat(), real.join("h.eadl")).expect("copied");
            std::os::unix::fs::symlink(&real, root.join("link")).expect("a link");
            let description = root.join("link/h.eadl").display().to_string();
            let before = common::everything_under(root);
            let outcome = resolve_of(
                &description,
                &root.join("p").display().to_string(),
                &[],
                Answer::Record(Status::Ok),
            );
            assert_eq!(common::everything_under(root), before, "something was made");
            // Unchecked, nothing is handed to a checker, so the same path is no refusal and the plan is written (R29-4).
            let unchecked = root.join("u");
            let (status, words) = resolve_of(
                &description,
                &unchecked.display().to_string(),
                &["--unchecked"],
                Answer::Record(Status::Ok),
            )
            .expect("the route runs");
            assert_eq!(status, Status::Ok, "{words}");
            assert!(unchecked.is_file());
            outcome
        }),
        ("no-checker", |root| {
            // The real executable, and no install beside it.
            let plan = root.join("p");
            an_earlier_plan(&plan);
            let output = std::process::Command::new(env!("CARGO_BIN_EXE_archogen"))
                .args(["resolve", &heartbeat(), "--out"])
                .arg(&plan)
                .output()
                .expect("the executable runs");
            assert_eq!(
                std::fs::read_to_string(&plan).expect("kept"),
                AN_EARLIER_PLAN
            );
            assert!(!root.join("p.part").exists() && !root.join("p.check").exists());
            assert_eq!(
                output.status.code(),
                Some(Status::ToolFailure.code()),
                "{output:?}"
            );
            Some((
                Status::ToolFailure,
                String::from_utf8_lossy(&output.stderr).to_string(),
            ))
        }),
        ("closed-meanwhile", |root| {
            use std::os::unix::fs::PermissionsExt;
            if !modes_bind(root) {
                return None;
            }
            // The checker has accepted, and the place cannot take what it accepted: neither name moves.
            let parent = root.join("d");
            std::fs::create_dir(&parent).expect("a directory");
            let plan = parent.join("p");
            let record = a_checked_plan(&plan);
            let before = std::fs::read(&plan).expect("read");
            let outcome = resolve_of(
                &heartbeat(),
                &plan.display().to_string(),
                &[],
                Answer::Closing(parent.join("p.check")),
            );
            std::fs::set_permissions(
                parent.join("p.check"),
                std::fs::Permissions::from_mode(0o755),
            )
            .expect("its mode");
            assert_eq!(std::fs::read(&plan).expect("kept"), before);
            assert_eq!(
                std::fs::read(parent.join("p.check/plan-check.json")).expect("kept"),
                record
            );
            assert!(!parent.join("p.part").exists());
            outcome
        }),
        ("a-flag-as-its-value", |root| {
            // The parser takes the next word as `--out`'s value, whatever it spells.
            let output = std::process::Command::new(env!("CARGO_BIN_EXE_archogen"))
                .args([
                    "resolve",
                    &heartbeat(),
                    "--out",
                    "--unchecked",
                    "--unchecked",
                ])
                .current_dir(root)
                .output()
                .expect("the executable runs");
            assert!(root.join("--unchecked").is_file());
            let said = format!(
                "{}{}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            let status = if output.status.success() {
                Status::Ok
            } else {
                Status::ToolFailure
            };
            Some((status, said))
        }),
    ]
}

#[test]
fn every_plan_the_engine_writes_is_a_plan_the_next_run_may_replace() {
    // The reader is the engine's own, so its writer's every form is exercised here: a plain one, a module tree, adapters.
    let dir = scratch();
    let mut plans = 0;
    for corpus in [
        "crates/eadl-resolve/tests/search",
        "crates/archogen-plan/tests/corpus",
    ] {
        for entry in std::fs::read_dir(root().join(corpus)).expect("the corpus") {
            let description = entry.expect("an entry").path();
            if description
                .extension()
                .is_none_or(|extension| extension != "eadl")
            {
                continue;
            }
            let name = description.display().to_string();
            let plan = dir.join(format!("plan-{plans}"));
            let out = plan.display().to_string();
            let unchecked = ["--unchecked"];
            let (first, _) = resolve_of(&name, &out, &unchecked, Answer::Record(Status::Ok))
                .expect("the route runs");
            if first != Status::Ok {
                continue;
            }
            plans += 1;
            let (again, said) = resolve_of(&name, &out, &unchecked, Answer::Record(Status::Ok))
                .expect("the route runs");
            assert_eq!(again, Status::Ok, "{name}: {said}");
        }
    }
    assert!(plans > 10, "only {plans} plans were written");
}

#[test]
#[cfg(unix)]
fn every_route_of_out_does_what_the_chapter_s_table_says() {
    let chapter =
        std::fs::read_to_string(root().join("docs/book/src/cli.md")).expect("the chapter");
    let rows = common::table_rows(&chapter, "plan-routes");
    let routes = plan_routes();
    let mut named: Vec<&str> = routes.iter().map(|(name, _)| *name).collect();
    let mut listed: Vec<&str> = rows.iter().map(|row| row[0].as_str()).collect();
    named.sort_unstable();
    listed.sort_unstable();
    assert_eq!(listed, named, "the table's routes and the fixtures differ");
    for row in &rows {
        let name = row[0].as_str();
        let (_, route) = routes
            .iter()
            .find(|(fixture, _)| *fixture == name)
            .expect("each row has its fixture");
        let Some((status, said)) = route(&scratch()) else {
            continue;
        };
        assert_eq!(status.slug(), row[2], "`{name}`: {said}");
        assert!(
            said.contains(&row[3]),
            "`{name}` does not say `{}`: {said}",
            row[3]
        );
    }
}

#[test]
fn a_plan_the_checker_refuses_is_a_tool_failure_naming_both() {
    let dir = scratch();
    let plan = dir.join("plan.json");
    let checker = Checker::answering(Answer::Record(Status::ToolFailure));
    let (status, _, err) = resolve(&[&heartbeat(), "--out", plan.to_str().unwrap()], &checker);
    assert_eq!(status, Status::ToolFailure, "{err}");
    assert!(
        err.contains("`archogen-plan-check` refuses the plan the search wrote"),
        "{err}"
    );
}

#[test]
fn a_checker_that_cannot_run_is_a_tool_failure_with_its_reason() {
    let dir = scratch();
    let plan = dir.join("plan.json");
    let (status, _, err) = resolve(
        &[&heartbeat(), "--out", plan.to_str().unwrap()],
        &Checker::answering(Answer::Unlaunched),
    );
    assert_eq!(status, Status::ToolFailure, "{err}");
    assert!(err.contains("no install beside the front"), "{err}");
}

#[test]
fn unchecked_writes_the_plan_runs_nothing_and_says_so() {
    let dir = scratch();
    let plan = dir.join("plan.json");
    let checker = Checker::answering(Answer::Unlaunched);
    let (status, out, err) = resolve(
        &[&heartbeat(), "--out", plan.to_str().unwrap(), "--unchecked"],
        &checker,
    );
    assert_eq!(status, Status::Ok, "{err}");
    assert!(plan.is_file());
    assert!(out.contains("resolved system `heartbeat`"), "{out}");
    assert!(
        err.contains("`--unchecked`") && err.contains("not checked"),
        "{err}"
    );
    assert!(checker.asked.borrow().is_empty(), "nothing was run");
}

#[test]
fn a_plan_binding_an_engine_adapter_is_said_to_be_conditional_on_what_it_owes() {
    // Leaf `M3.2.6.1`, `docs/specs/adapters/decision_engine-adapters.md` §9: the epoch extender's obligations and its
    // cost, each named; the heartbeat's plan, binding no adapter, says nothing of the kind.
    let dir = scratch();
    let plan = dir.join("plan");
    let fixture = root().join("crates/eadl-resolve/tests/search/adapter-epoch-extender.eadl");
    let checker = Checker::answering(Answer::Unlaunched);
    let (status, out, err) = resolve(
        &[
            fixture.to_str().unwrap(),
            "--out",
            plan.to_str().unwrap(),
            "--unchecked",
        ],
        &checker,
    );
    assert_eq!(status, Status::Ok, "{err}");
    assert!(
        out.contains("  conditional: it binds engine adapters, and holds once each obligation below is discharged"),
        "{out}"
    );
    let owed: Vec<&str> = out
        .lines()
        .filter_map(|line| line.strip_prefix("    (adapter epoch-extender timer.short) "))
        .map(|rest| rest.split_once(':').map_or(rest, |(head, _)| head))
        .collect();
    assert_eq!(
        owed,
        [
            "obligation `update`",
            "obligation `coherent`",
            "interrupt `wake`"
        ],
        "{out}"
    );
    let (status, out, err) = resolve(
        &[&heartbeat(), "--out", plan.to_str().unwrap(), "--unchecked"],
        &checker,
    );
    assert_eq!(status, Status::Ok, "{err}");
    assert!(!out.contains("conditional"), "{out}");
}

#[test]
fn every_refusal_before_a_plan_writes_none_and_runs_nothing() {
    let dir = scratch();
    let write = |name: &str, text: &str| {
        let path = dir.join(name);
        std::fs::write(&path, text).unwrap();
        path.display().to_string()
    };
    let platform = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output)))
";
    let system = |name: &str| {
        format!(
            "(defsystem {name}
  (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (requires (uses console.write))
  (platform (uses soc)))
"
        )
    };
    let two = write(
        "two.eadl",
        &format!("{platform}{}{}", system("a"), system("b")),
    );
    let none = write("none.eadl", platform);
    let invalid = write(
        "invalid.eadl",
        "(eadl-version eadl/1)\n(defservice broken (requries (needs x)))\n",
    );
    write(
        "hw.board.eadl",
        "(defmodule hw.board (version 1 0) (export soc console.uart)
  (defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
  (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart))))",
    );
    let instances = write(
        "app.eadl",
        "(defmodule app (version 1 0) (import hw.board (as hw))
  (defservice console.write (requires (needs observable-output)))
  (defsystem s
    (task beat (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
    (requires (uses console.write))
    (platform (uses hw.soc))))",
    );
    let periodic = root()
        .join("examples/periodic-three/system.eadl")
        .display()
        .to_string();
    let plan = dir.join("plan.json");
    let out = plan.to_str().unwrap();
    for (case, args, status, says) in [
        (
            "--locked",
            vec![heartbeat(), "--locked".to_owned()],
            Status::Unimplemented,
            "`--locked` is not implemented",
        ),
        (
            "an invalid description",
            vec![invalid],
            Status::InvalidDescription,
            "nothing was resolved",
        ),
        ("no system", vec![none], Status::Usage, "declares no system"),
        (
            "two systems",
            vec![two.clone()],
            Status::Usage,
            "`--system` on the command line: `a`, `b`",
        ),
        (
            "a system it does not declare",
            vec![two, "--system".to_owned(), "c".to_owned()],
            Status::Usage,
            "declares no system `c`",
        ),
        (
            "nothing meets a requirement",
            vec![periodic],
            Status::MissingFact,
            "system `periodic-three` of",
        ),
    ] {
        let checker = Checker::answering(Answer::Record(Status::Ok));
        let mut argv: Vec<&str> = args.iter().map(String::as_str).collect();
        argv.extend(["--out", out]);
        let (got, _, err) = resolve(&argv, &checker);
        assert_eq!(got, status, "{case}: {err}");
        assert!(err.contains(says), "{case}: {err}");
        assert!(!plan.exists(), "{case}: a plan was written");
        assert!(
            checker.asked.borrow().is_empty(),
            "{case}: the checker was run"
        );
    }
    // A module tree of several instances resolves, its names qualified as the language qualifies them (leaf `M3.4.10`).
    let (got, _, err) = resolve(
        &[&instances, "--out", out, "--unchecked"],
        &Checker::answering(Answer::Unlaunched),
    );
    assert_eq!(got, Status::Ok, "{err}");
    let written = std::fs::read_to_string(&plan).expect("a plan");
    assert!(
        written.ends_with("(bind console.write observable-output hw.console.uart)\n"),
        "{written}"
    );
    std::fs::remove_file(&plan).unwrap();
    // `--system` naming one of two resolves it.
    let two = dir.join("two.eadl").display().to_string();
    let (got, out_text, err) = resolve(
        &[&two, "--system", "b", "--out", out, "--unchecked"],
        &Checker::answering(Answer::Unlaunched),
    );
    assert_eq!(got, Status::Ok, "{err}");
    assert!(out_text.contains("resolved system `b`"), "{out_text}");
}

/// A plan past what any small description writes is still a plan, and the next run may replace it. Canonical lines, as the search sorts them.
#[test]
#[cfg(unix)]
fn a_large_plan_is_a_plan_the_next_run_may_replace() {
    let root = scratch();
    let plan = root.join("p");
    let mut text = AN_EARLIER_PLAN.to_owned();
    let mut line = 0;
    while text.len() <= 1 << 24 {
        text.push_str(&format!(
            "(bind t{line:08}.write observable-output console.alt)\n"
        ));
        line += 1;
    }
    assert!(archogen_api::is_a_plan(&text), "a plan");
    std::fs::write(&plan, &text).expect("written");
    let (status, said) = resolve_at(&plan.display().to_string(), &[]).expect("the route runs");
    assert_eq!(status, Status::Ok, "{said}");
    assert_ne!(std::fs::read_to_string(&plan).expect("a plan"), text);
}

/// A record past a mebibyte is one the checker can write, so the next run reads it as one and may replace it.
#[test]
#[cfg(unix)]
fn a_large_record_is_a_record_the_next_run_may_replace() {
    let root = scratch();
    let plan = root.join("p");
    a_checked_plan(&plan);
    let record = root.join("p.check/plan-check.json");
    let large = std::fs::read_to_string(&record)
        .expect("the record")
        .replace(PLAN_CHECKER, &"x".repeat(1 << 21));
    assert!(large.len() > 1 << 21, "{}", large.len());
    std::fs::write(&record, large).expect("written");
    let (status, said) = resolve_at(&plan.display().to_string(), &[]).expect("the route runs");
    assert_eq!(status, Status::Ok, "{said}");
    assert!(std::fs::metadata(&record).expect("a record").len() < 1 << 20);
}

/// A plan it cannot read back is said as what it is, the umask, and no stage is left. Not "changed", since nothing came.
#[test]
#[cfg(unix)]
fn a_plan_it_cannot_read_back_is_put_down_to_the_umask() {
    let root = scratch();
    let description = heartbeat();
    let plan = root.join("p");
    let output = std::process::Command::new("/bin/sh")
        .args(["-c", "umask 0477; exec \"$0\" \"$@\""])
        .arg(env!("CARGO_BIN_EXE_archogen"))
        .arg("resolve")
        .arg(&description)
        .arg("--out")
        .arg(&plan)
        .arg("--unchecked")
        .output()
        .expect("the executable runs");
    let said = String::from_utf8_lossy(&output.stderr).into_owned();
    assert_eq!(output.status.code(), Some(Status::Usage.code()), "{said}");
    assert!(
        said.contains("read back") && said.contains("umask"),
        "{said}"
    );
    assert!(!said.contains("changed while"), "{said}");
    assert!(!plan.exists() && !root.join("p.part").exists(), "{said}");
}
