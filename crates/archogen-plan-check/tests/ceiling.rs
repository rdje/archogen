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
    std::fs::write(dir.join("d.eadl"), "x ".repeat(4 * 1024 * 1024)).expect("written");
    std::fs::write(dir.join("p"), "(junk)").expect("written");
    let (code, said) = checker("2", &dir, &["d.eadl", "p"]);
    assert_eq!(code, Status::AnalysisInconclusive.code(), "{said}");
    assert!(said.contains(VARIABLE), "{said}");

    let (code, said) = checker("64", &dir, &["d\nall clear.eadl", "p"]);
    assert_eq!(code, Status::Usage.code(), "{said}");
    assert!(said.contains("line break"), "{said}");

    let (code, said) = checker("many", &dir, &["d.eadl", "p"]);
    assert_eq!(code, Status::Usage.code(), "{said}");
    assert!(said.contains(VARIABLE), "{said}");
}
