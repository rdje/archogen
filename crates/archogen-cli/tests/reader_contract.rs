//! The reader's contract, seen through `archogen check`: any text is answered with a verdict, never a crash.
//!
//! ⛔ Leaf `M1.35`: a string holding an unknown escape before a multi-byte character — `"a\éb"` — made the
//! reader panic, and `archogen check` exited 101 where §5.5's contract is a diagnostic and exit 10. The call
//! below is in-process, so a panic fails this test rather than being reported as a status.

use std::path::Path;

use archogen_cli::{run, Status};

/// Write `text` into this test's own scratch directory under `CARGO_TARGET_TMPDIR`, on the repository's
/// volume, and return its path. The directory is created here because cargo creates the tmpdir when it
/// builds a test binary, not when it runs one.
fn write(leg: &str, text: &str) -> String {
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("reader-contract")
        .join(leg);
    std::fs::create_dir_all(&dir).expect("the target tmpdir is creatable");
    let path = dir.join("case.eadl");
    std::fs::write(&path, text).expect("the scratch directory is writable");
    path.display().to_string()
}

#[test]
fn an_unknown_escape_before_a_multibyte_character_is_an_invalid_description_not_a_crash() {
    for (leg, escaped) in [("two-byte", "é"), ("four-byte", "🙂")] {
        let path = write(
            leg,
            &format!("(eadl-version eadl/1)\n(defkind x (doc \"a\\{escaped}b\"))\n"),
        );
        let mut out: Vec<u8> = Vec::new();
        let mut err: Vec<u8> = Vec::new();
        let status = run(
            ["check", path.as_str()].iter().map(|s| (*s).to_string()),
            &mut out,
            &mut err,
        );
        let err = String::from_utf8(err).expect("utf-8 stderr");
        assert_eq!(status, Status::InvalidDescription, "{leg}: {err}");
        assert!(err.contains("read-bad-escape"), "{leg}: {err}");
        assert!(err.contains(&format!("`\\{escaped}`")), "{leg}: {err}");
    }
}
