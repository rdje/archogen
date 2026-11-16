//! The checker's process has the ceiling `archogen`'s has (leaf `M4.2.3.3.22`): it holds a description to judge it, so a shape that costs the search costs it too.

use std::process::{Command, Stdio};

use archogen_outcome::Status;
use archogen_transport::ceiling::VARIABLE;

fn checker(ceiling: &str, dir: &std::path::Path, args: &[&str]) -> (i32, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_archogen-plan-check"))
        .args(args)
        .current_dir(dir)
        .env(VARIABLE, ceiling)
        .stdin(Stdio::null())
        .output()
        .expect("the checker runs");
    (
        output.status.code().expect("it ended with a code"),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

#[test]
fn the_checker_stops_at_its_ceiling() {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("plan-check-ceiling-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    std::fs::write(dir.join("p"), "(junk)").expect("written");
    // Well above what the program itself takes, so this is the fixture's doing, as the small one beside it shows.
    std::fs::write(dir.join("small.eadl"), "x ").expect("written");
    let (code, said) = checker("32", &dir, &["small.eadl", "p"]);
    assert_ne!(code, Status::AnalysisInconclusive.code(), "{said}");
    std::fs::write(dir.join("d.eadl"), "x ".repeat(32 * 1024 * 1024)).expect("written");
    let (code, said) = checker("32", &dir, &["d.eadl", "p"]);
    assert_eq!(code, Status::AnalysisInconclusive.code(), "{said}");
    assert!(said.contains(VARIABLE), "{said}");
    assert!(said.starts_with("archogen-plan-check:"), "{said}");
    std::fs::remove_file(dir.join("d.eadl")).expect("removed");

    let (code, said) = checker("64", &dir, &["d\nall clear.eadl", "p"]);
    assert_eq!(code, Status::Usage.code(), "{said}");
    assert!(said.contains("line break"), "{said}");
    assert!(said.contains("hint:"), "{said}");

    let (code, said) = checker("many", &dir, &["d.eadl", "p"]);
    assert_eq!(code, Status::Usage.code(), "{said}");
    assert!(said.contains(VARIABLE), "{said}");
    assert!(said.contains("hint:"), "{said}");
    let (code, said) = checker("x\nall clear: accepted\n", &dir, &["d.eadl", "p"]);
    assert_eq!(code, Status::Usage.code(), "{said}");
    assert!(
        !said
            .lines()
            .any(|line| line.trim() == "all clear: accepted"),
        "{said}"
    );
}

/// The checker names itself by its own file's sha256, read in pieces (leaf `PROGRAM.151.6`): a ceiling below that
/// file's size still lets it judge, on any host, where a build with its debug information linked in is the larger.
#[test]
fn the_checker_s_own_size_takes_nothing_of_its_ceiling() {
    let dir = std::path::Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join(format!("plan-check-own-size-{}", std::process::id()));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    std::fs::write(dir.join("p"), "(junk)").expect("written");
    std::fs::write(dir.join("small.eadl"), "x ").expect("written");
    let own = std::fs::metadata(env!("CARGO_BIN_EXE_archogen-plan-check"))
        .expect("the built checker")
        .len();
    let ceiling = ((own / 2) >> 20).max(1);
    let (code, said) = checker(&ceiling.to_string(), &dir, &["small.eadl", "p"]);
    assert_ne!(
        code,
        Status::AnalysisInconclusive.code(),
        "a ceiling of {ceiling} MiB, half of its own {own} bytes: {said}"
    );
    let _ = std::fs::remove_dir_all(&dir);
}
