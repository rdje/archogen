//! `cargo xtask mutate` — the `extended` tier's `mutation` step (leaf `PROGRAM.9.3`).
//!
//! Every leaf that fixed a defect ran mutation controls by hand: put the defect back, watch a test fail,
//! restore. That is evidence, but not a repeatable tier — the next change can quietly make a test stop
//! catching its defect, and nothing re-asks. This harness re-asks, from a catalog (`xtask/mutations.txt`):
//! each entry is an exact text in one file, a replacement, the `cargo test` arguments that must see it, and
//! whether they are expected to **kill** it or — for a blind spot recorded on purpose — to let it **survive**.
//!
//! ⭐ Every step is checked rather than assumed (`docs/knowledge/verify-the-mutation-applied.md`):
//! - the text must occur **exactly once**, so an edit cannot land somewhere unintended;
//! - the mutated file is read back and must differ from the original by exactly that replacement;
//! - a mutation that does not **compile** is a broken catalog entry, never counted as "killed";
//! - the original is restored and read back **byte for byte**, whatever happened — a `Drop` guard covers a
//!   panic, and a sentinel file under `target/` makes an interrupted run refuse to start until the file it
//!   left mutated is restored.
//!
//! Exit 0: every entry did what the catalog expects. 1: a mutation survived that should have been killed, or
//! the reverse. 2: the catalog itself is broken (unparsable, a text not found once, a mutation that does not
//! compile) or an interrupted run is pending.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::Instant;

/// One catalog entry.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Mutation {
    pub id: String,
    pub file: String,
    pub test: Vec<String>,
    pub expect: Expect,
    pub why: String,
    pub from: String,
    pub to: String,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Expect {
    /// The test arguments must fail with the mutation in place.
    Killed,
    /// A blind spot kept on purpose, as a reproduction: the test arguments must still pass.
    Survives,
}

/// Parse the catalog. The format, one entry at a time:
///
/// ```text
/// mutation <id>
/// file <path>
/// test <cargo test arguments>
/// expect killed|survives
/// why <one line>
/// from
///     <exact text, indented four spaces>
/// to
///     <replacement, indented four spaces>
/// end
/// ```
///
/// Lines starting with `#` outside a block are comments.
///
/// # Errors
///
/// A description of the first malformed line, with its number.
pub fn parse(text: &str) -> Result<Vec<Mutation>, String> {
    let mut out = Vec::new();
    let mut lines = text.lines().enumerate().peekable();
    while let Some((n, line)) = lines.next() {
        let line = line.trim_end();
        if line.is_empty() || line.starts_with('#') {
            continue;
        }
        let Some(id) = line.strip_prefix("mutation ") else {
            return Err(format!(
                "line {}: expected `mutation <id>`, found `{line}`",
                n + 1
            ));
        };
        let mut field = |key: &str| -> Result<String, String> {
            let (m, line) = lines
                .next()
                .ok_or_else(|| format!("entry `{id}`: ends before `{key}`"))?;
            line.trim_end()
                .strip_prefix(&format!("{key} "))
                .map(str::to_string)
                .ok_or_else(|| format!("line {}: expected `{key} …` in entry `{id}`", m + 1))
        };
        let file = field("file")?;
        let test = field("test")?
            .split_whitespace()
            .map(str::to_string)
            .collect();
        let expect = match field("expect")?.as_str() {
            "killed" => Expect::Killed,
            "survives" => Expect::Survives,
            other => {
                return Err(format!(
                    "entry `{id}`: `expect {other}` — write `killed` or `survives`"
                ))
            }
        };
        let why = field("why")?;
        let mut block = |opening: &str, closing: &str| -> Result<String, String> {
            match lines.next() {
                Some((_, l)) if l.trim_end() == opening => {}
                _ => return Err(format!("entry `{id}`: expected `{opening}`")),
            }
            let mut body = Vec::new();
            loop {
                let (m, l) = lines
                    .next()
                    .ok_or_else(|| format!("entry `{id}`: `{opening}` block never closed"))?;
                if l.trim_end() == closing {
                    break;
                }
                if l.is_empty() {
                    body.push(String::new());
                    continue;
                }
                let stripped = l.strip_prefix("    ").ok_or_else(|| {
                    format!(
                        "line {}: a `{opening}` line must be indented four spaces",
                        m + 1
                    )
                })?;
                body.push(stripped.to_string());
            }
            if body.is_empty() {
                return Err(format!("entry `{id}`: an empty `{opening}` block"));
            }
            Ok(body.join("\n"))
        };
        let from = block("from", "to")?;
        // The `to` block's opening line was consumed as `from`'s closing one; read to `end` directly.
        let mut to_lines = Vec::new();
        loop {
            let (m, l) = lines
                .next()
                .ok_or_else(|| format!("entry `{id}`: `to` block never closed"))?;
            if l.trim_end() == "end" {
                break;
            }
            let stripped = if l.is_empty() {
                ""
            } else {
                l.strip_prefix("    ").ok_or_else(|| {
                    format!("line {}: a `to` line must be indented four spaces", m + 1)
                })?
            };
            to_lines.push(stripped.to_string());
        }
        let to = to_lines.join("\n");
        if from == to {
            return Err(format!("entry `{id}`: `from` and `to` are the same text"));
        }
        if out.iter().any(|m: &Mutation| m.id == id) {
            return Err(format!("entry `{id}` appears twice"));
        }
        out.push(Mutation {
            id: id.to_string(),
            file,
            test,
            expect,
            why,
            from,
            to,
        });
    }
    if out.is_empty() {
        return Err(
            "the catalog has no entry — an empty catalog is a breach, not a pass".to_string(),
        );
    }
    Ok(out)
}

/// Apply a mutation to a file's text: `Ok(mutated)` only when `from` occurs exactly once.
///
/// # Errors
///
/// How many times `from` occurs, when it is not once.
pub fn apply(original: &str, m: &Mutation) -> Result<String, String> {
    let count = original.matches(m.from.as_str()).count();
    if count != 1 {
        return Err(format!(
            "`{}`: its `from` text occurs {count} time(s) in {} — it must occur exactly once",
            m.id, m.file
        ));
    }
    Ok(original.replacen(m.from.as_str(), m.to.as_str(), 1))
}

/// Writes the original back when dropped — on success, on error, and on a panic.
struct Restore {
    path: PathBuf,
    original: Vec<u8>,
    sentinel: PathBuf,
}

impl Drop for Restore {
    fn drop(&mut self) {
        let _ = fs::write(&self.path, &self.original);
        if fs::read(&self.path).is_ok_and(|now| now == self.original) {
            let _ = fs::remove_file(&self.sentinel);
        }
    }
}

enum Seen {
    /// The names of the tests that failed, as cargo lists them.
    Killed(Vec<String>),
    Survived,
    DoesNotCompile(String),
}

fn run_one(root: &Path, m: &Mutation, scratch: &Path) -> Result<(Seen, f64), String> {
    let path = root.join(&m.file);
    let original =
        fs::read(&path).map_err(|e| format!("`{}`: cannot read {}: {e}", m.id, m.file))?;
    let text = String::from_utf8(original.clone())
        .map_err(|_| format!("`{}`: {} is not UTF-8", m.id, m.file))?;
    let mutated = apply(&text, m)?;
    let sentinel = scratch.join("ACTIVE");
    fs::write(scratch.join(format!("{}.orig", m.id)), &original)
        .map_err(|e| format!("cannot back up: {e}"))?;
    fs::write(&sentinel, format!("{}\n{}\n", m.file, m.id))
        .map_err(|e| format!("cannot write the sentinel: {e}"))?;
    let guard = Restore {
        path: path.clone(),
        original: original.clone(),
        sentinel,
    };
    fs::write(&path, mutated.as_bytes())
        .map_err(|e| format!("`{}`: cannot write {}: {e}", m.id, m.file))?;
    // Read back: the file on disk is the mutation, not the original and not something else.
    let on_disk = fs::read(&path).map_err(|e| format!("cannot read back {}: {e}", m.file))?;
    if on_disk != mutated.as_bytes() || on_disk == original {
        return Err(format!(
            "`{}`: the mutation did not land in {}",
            m.id, m.file
        ));
    }
    let started = Instant::now();
    let output = Command::new("cargo")
        .arg("test")
        .arg("-q")
        .args(&m.test)
        .current_dir(root)
        .output()
        .map_err(|e| format!("`{}`: cannot run cargo: {e}", m.id))?;
    let elapsed = started.elapsed().as_secs_f64();
    drop(guard);
    let restored =
        fs::read(&path).map_err(|e| format!("cannot read {} after restoring: {e}", m.file))?;
    if restored != original {
        return Err(format!(
            "`{}`: {} is NOT byte-identical to its original after restoring — restore it from {}",
            m.id,
            m.file,
            scratch.join(format!("{}.orig", m.id)).display()
        ));
    }
    let stderr = String::from_utf8_lossy(&output.stderr);
    let stdout = String::from_utf8_lossy(&output.stdout);
    let outcome = if output.status.success() {
        Seen::Survived
    } else if stderr.contains("could not compile") && !stdout.contains("test result") {
        let first = stderr
            .lines()
            .find(|l| l.starts_with("error"))
            .unwrap_or("")
            .to_string();
        Seen::DoesNotCompile(first)
    } else {
        Seen::Killed(failed_tests(&stdout))
    };
    // The backup is evidence only until the restore is verified, which it now is.
    let _ = fs::remove_file(scratch.join(format!("{}.orig", m.id)));
    Ok((outcome, elapsed))
}

/// The tests cargo lists under `failures:` — the names that make a kill evidence rather than a claim.
fn failed_tests(stdout: &str) -> Vec<String> {
    let mut names = Vec::new();
    let mut in_list = false;
    for line in stdout.lines() {
        if line == "failures:" {
            in_list = true;
            continue;
        }
        if in_list {
            match line.strip_prefix("    ") {
                Some(name) if !name.is_empty() && !name.contains(' ') => {
                    names.push(name.to_string())
                }
                _ if names.is_empty() => {}
                _ => in_list = false,
            }
        }
    }
    names.sort();
    names.dedup();
    names
}

/// Run the catalog, or the entries named in `only`. Returns the process exit code.
#[must_use]
pub fn run(root: &Path, only: &[String]) -> i32 {
    let scratch = root.join("target/doctrine_scratch/mutation");
    if let Ok(pending) = fs::read_to_string(scratch.join("ACTIVE")) {
        eprintln!(
            "xtask mutate: an earlier run was interrupted with a file still mutated:\n{pending}  restore it \
             (the original is beside the sentinel, in {}), then delete the sentinel",
            scratch.display()
        );
        return 2;
    }
    if let Err(e) = fs::create_dir_all(&scratch) {
        eprintln!("xtask mutate: cannot create {}: {e}", scratch.display());
        return 2;
    }
    let catalog = match fs::read_to_string(root.join("xtask/mutations.txt"))
        .map_err(|e| e.to_string())
        .and_then(|t| parse(&t))
    {
        Ok(catalog) => catalog,
        Err(e) => {
            eprintln!("xtask mutate: the catalog is broken: {e}");
            return 2;
        }
    };
    let unknown: Vec<&String> = only
        .iter()
        .filter(|id| !catalog.iter().any(|m| &m.id == *id))
        .collect();
    if !unknown.is_empty() {
        eprintln!("xtask mutate: no catalog entry named {unknown:?}");
        return 2;
    }
    let (mut as_expected, mut wrong, mut broken) = (0, 0, 0);
    for m in catalog
        .iter()
        .filter(|m| only.is_empty() || only.contains(&m.id))
    {
        match run_one(root, m, &scratch) {
            Ok((Seen::DoesNotCompile(first), _)) => {
                broken += 1;
                eprintln!("  ✗ {}: the mutation does not compile — a broken catalog entry, not a kill: {first}", m.id);
            }
            Ok((outcome, secs)) => {
                let (got, by) = match &outcome {
                    Seen::Killed(names) if names.is_empty() => (Expect::Killed, String::new()),
                    Seen::Killed(names) => (Expect::Killed, format!(" by {}", names.join(", "))),
                    _ => (Expect::Survives, String::new()),
                };
                let verb = if got == Expect::Killed {
                    "killed"
                } else {
                    "survived"
                };
                if got == m.expect {
                    as_expected += 1;
                    println!(
                        "  ✓ {:<42} {verb}{by} ({secs:.1}s) — cargo test {}",
                        m.id,
                        m.test.join(" ")
                    );
                } else {
                    wrong += 1;
                    eprintln!(
                        "  ✗ {}: {verb}{by}, expected {} ({secs:.1}s) — cargo test {}\n      {}",
                        m.id,
                        if m.expect == Expect::Killed {
                            "killed"
                        } else {
                            "to survive"
                        },
                        m.test.join(" "),
                        m.why
                    );
                }
            }
            Err(e) => {
                broken += 1;
                eprintln!("  ✗ {e}");
            }
        }
    }
    if broken > 0 {
        eprintln!("xtask mutate: {broken} broken entr(ies) — fix the catalog");
        return 2;
    }
    if wrong > 0 {
        eprintln!(
            "xtask mutate: {wrong} of {} did not do what the catalog expects",
            as_expected + wrong
        );
        return 1;
    }
    println!("mutate: OK — {as_expected} mutation(s), each killed or surviving exactly as the catalog expects");
    0
}

#[cfg(test)]
mod tests {
    use super::{apply, parse, Expect};

    const ONE: &str = "\
# a comment
mutation flip
file src/x.rs
test -p x --lib
expect killed
why a reason
from
    a < b
to
    a > b
end
";

    #[test]
    fn an_entry_parses_into_its_fields() {
        let catalog = parse(ONE).expect("valid");
        assert_eq!(catalog.len(), 1);
        let m = &catalog[0];
        assert_eq!(
            (m.id.as_str(), m.file.as_str(), m.expect),
            ("flip", "src/x.rs", Expect::Killed)
        );
        assert_eq!(m.test, ["-p", "x", "--lib"]);
        assert_eq!((m.from.as_str(), m.to.as_str()), ("a < b", "a > b"));
    }

    #[test]
    fn a_block_keeps_its_inner_indentation_and_blank_lines() {
        let text = ONE.replace("    a < b\n", "    fn f() {\n\n        a < b\n    }\n");
        let m = &parse(&text).expect("valid")[0];
        assert_eq!(m.from, "fn f() {\n\n    a < b\n}");
    }

    #[test]
    fn a_malformed_catalog_is_refused_with_its_line() {
        assert!(parse("").unwrap_err().contains("no entry"));
        assert!(parse("mutation x\nfile f\n")
            .unwrap_err()
            .contains("ends before `test`"));
        assert!(parse(&ONE.replace("expect killed", "expect maybe"))
            .unwrap_err()
            .contains("expect maybe"));
        assert!(parse(&ONE.replace("    a > b", "a > b"))
            .unwrap_err()
            .contains("indented four spaces"));
        assert!(parse(&ONE.replace("    a > b", "    a < b"))
            .unwrap_err()
            .contains("the same text"));
        assert!(parse(&format!("{ONE}{ONE}"))
            .unwrap_err()
            .contains("appears twice"));
    }

    #[test]
    fn a_kill_names_the_tests_cargo_lists_as_failed() {
        let stdout = "running 3 tests\n..F\nfailures:\n\n---- tests::b stdout ----\nboom\n\n\nfailures:\n    tests::b\n    tests::c\n\ntest result: FAILED. 1 passed; 2 failed\n";
        assert_eq!(super::failed_tests(stdout), ["tests::b", "tests::c"]);
        assert!(super::failed_tests("test result: ok. 3 passed\n").is_empty());
    }

    #[test]
    fn every_entry_of_the_real_catalog_names_a_text_its_file_holds_once() {
        // The catalog runs whole only in the `extended` tier, so an entry whose source moved stayed broken from
        // `API.4.2` until `PROGRAM.61` found it. This leg needs no build and no mutation: it parses the real catalog
        // and applies each entry to its file in memory, so `cargo test` — the focused tier and CI — refuses the
        // entry in the commit that moves its text (leaf `PROGRAM.61`).
        let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .expect("xtask/ is one level below the root");
        let catalog = parse(
            &std::fs::read_to_string(root.join("xtask/mutations.txt"))
                .expect("the catalog is readable"),
        )
        .expect("the catalog parses");
        let broken: Vec<String> = catalog
            .iter()
            .filter_map(|m| match std::fs::read_to_string(root.join(&m.file)) {
                Ok(original) => apply(&original, m).err(),
                Err(e) => Some(format!("`{}`: {} cannot be read: {e}", m.id, m.file)),
            })
            .collect();
        assert!(broken.is_empty(), "{}", broken.join("\n"));
    }

    #[test]
    fn a_text_that_is_not_there_exactly_once_is_not_applied() {
        let m = &parse(ONE).expect("valid")[0];
        assert_eq!(apply("x = a < b;", m).expect("once"), "x = a > b;");
        assert!(apply("x = 1;", m).unwrap_err().contains("0 time(s)"));
        assert!(apply("a < b; a < b;", m).unwrap_err().contains("2 time(s)"));
    }
}
