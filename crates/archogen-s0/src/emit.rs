//! The Rust emitter: an S0 plan in, a compilable crate on disk out.
//!
//! `ROADMAP.md` §12 S0 fixes the boundary this module lives on: *"all templates, service code,
//! and dispatch choices stay inside the engine."* So what is written here is exactly three
//! things — a manifest, a **table** specialized from the plan, and a verbatim copy of the
//! engine-owned runtime in [`crate::runtime`]. No behavior is generated; behavior is *copied*
//! from source that is compiled and tested inside this crate.
//!
//! That split is what §1.1 permits — "Generation may emit bindings, specialized code, tables,
//! layouts, and copied or linked components" — and it is why `rt.rs` is not a string literal. A
//! template held as a string is Rust that nothing type-checks until a user compiles the output;
//! held as a module and read with `include_str!`, the same bytes are compiled here, tested here,
//! and shipped. [`runtime_files`] and its test keep the two copies identical.
//!
//! # Determinism
//!
//! §10.3 requires "deterministic generated sources from locked inputs". Nothing written here
//! depends on the clock, the filesystem order, the output path, or the host: the same plan
//! produces the same bytes. A timestamp in a header comment would quietly cost that, so there
//! is none.

use std::io;
use std::path::{Path, PathBuf};

use eadl_front::SourceMap;

use crate::interpret::Plan;
use crate::provenance::{self, Record};

/// The engine-owned sources, as `(file name in the generated crate, contents)`.
///
/// Byte-identical to the modules of [`crate::runtime`], which are compiled and tested as part of
/// this crate — see `emitted_runtime_is_the_reviewed_runtime`.
#[must_use]
pub fn runtime_files() -> [(&'static str, &'static str); 2] {
    [
        ("rt.rs", include_str!("runtime/rt.rs")),
        ("service.rs", include_str!("runtime/service.rs")),
    ]
}

/// What was written, for the command to report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Generated {
    /// The output directory.
    pub dir: PathBuf,
    /// Every file written, relative to `dir`, in write order.
    pub files: Vec<String>,
    /// The generated package's name.
    pub package: String,
    /// The provenance records, also written to `provenance.json`.
    pub provenance: Vec<Record>,
}

/// Write the generated crate for `plan` into `dir`, creating it if needed.
///
/// # Errors
///
/// Any filesystem error, with the path that failed.
pub fn emit(plan: &Plan, sources: &SourceMap, dir: &Path) -> io::Result<Generated> {
    // S0-ASSUMPTION: single-fixed-realization — one realization is selected unconditionally;
    // there is no candidate enumeration and no provider search. `M3.1` supplies both.
    // S0-ASSUMPTION: plan-is-not-independently-checked — the plan consumed here is the one the
    // interpreter produced, and nothing re-validates it against the description. `M3.5` does.
    // S0-ASSUMPTION: no-assurance-report — no per-property report is produced, so the build
    // makes no claim of any kind. `M4.8` produces one.
    let package = package_name(&plan.name);
    let src = dir.join("src");
    std::fs::create_dir_all(&src)?;

    let (main, records) = main_rs(plan);

    let mut files = Vec::new();
    write(dir, "Cargo.toml", &manifest(&package), &mut files)?;
    write(dir, "src/main.rs", &main, &mut files)?;
    for (name, contents) in runtime_files() {
        write(dir, &format!("src/{name}"), contents, &mut files)?;
    }
    write(
        dir,
        "provenance.json",
        &provenance::render(plan, &records, sources),
        &mut files,
    )?;

    Ok(Generated {
        dir: dir.to_path_buf(),
        files,
        package,
        provenance: records,
    })
}

fn write(dir: &Path, relative: &str, contents: &str, files: &mut Vec<String>) -> io::Result<()> {
    std::fs::write(dir.join(relative), contents)?;
    files.push(relative.to_string());
    Ok(())
}

/// A Cargo package name for a system name.
///
/// eADL names are dotted (`app.heartbeat`); Cargo's are not. The mapping is total and
/// deterministic, and it is prefixed so a generated package can never be mistaken for a
/// hand-written one in a build log. Runs of separators collapse, which makes the mapping
/// non-injective — `a.b` and `a..b` both land on `s0-a-b`. That is deliberate: the package name
/// is a label in a build log, and the thing that identifies a build is the description it came
/// from, not the name Cargo prints.
#[must_use]
pub fn package_name(system: &str) -> String {
    let mut body = String::with_capacity(system.len());
    for ch in system.chars() {
        if ch.is_ascii_alphanumeric() {
            body.push(ch.to_ascii_lowercase());
        } else if !body.ends_with('-') {
            body.push('-');
        }
    }
    format!("s0-{}", body.trim_matches('-'))
}

/// The generated manifest.
///
/// `[workspace]` is not decoration: without it, Cargo walks up from the output directory, finds
/// whatever workspace the user generated into, and refuses to build a package that is not one of
/// its members. An empty table makes the generated crate its own workspace root, so it builds
/// wherever it is written — which is what "from a clean local build directory" in F28 requires.
fn manifest(package: &str) -> String {
    format!(
        "{HEADER}\
         [package]\n\
         name = \"{package}\"\n\
         version = \"0.0.0\"\n\
         edition = \"2021\"\n\
         publish = false\n\n\
         # Its own workspace root: the generated crate builds wherever it is written.\n\
         [workspace]\n\n\
         # The S0 path generates nothing that needs a dependency.\n\
         [dependencies]\n"
    )
}

const HEADER: &str = "\
# GENERATED by archogen — the experimental S0 path. Do not edit.\n\
#\n\
# Re-generate with `archogen build <description> --out <dir>`; an edit here is lost on the next\n\
# build and makes the output stop corresponding to any description.\n\
#\n\
# EXPERIMENTAL. This carries no timing, assurance, or OS-completeness claim whatsoever. It is\n\
# the ROADMAP.md §12 S0 prototype, which exists to find pipeline mistakes early, and M4 replaces\n\
# it entirely.\n\n";

const RUST_HEADER: &str = "\
//! GENERATED by archogen — the experimental S0 path. Do not edit.\n\
//!\n\
//! Re-generate with `archogen build <description> --out <dir>`; an edit here is lost on the next\n\
//! build and makes the output stop corresponding to any description.\n\
//!\n\
//! EXPERIMENTAL. This carries no timing, assurance, or OS-completeness claim whatsoever. It is\n\
//! the ROADMAP.md §12 S0 prototype, which exists to find pipeline mistakes early, and M4\n\
//! replaces it entirely.\n";

/// The one specialized file: the plan as a table, and the call that runs it.
///
/// Returns the text **and** a provenance record for every declaration it writes. Building the
/// file as a line vector rather than a string is what makes those line numbers exact: a record
/// that names a line which does not contain what it claims is worse than no record, because it
/// sends a reader somewhere confidently wrong.
fn main_rs(plan: &Plan) -> (String, Vec<Record>) {
    let mut lines: Vec<String> = Vec::new();
    let mut records: Vec<Record> = Vec::new();

    for line in RUST_HEADER.lines() {
        lines.push(line.to_string());
    }
    lines.push("//!".into());
    lines.push(format!(
        "//! System `{}`: {} task(s), hyperperiod {} ms.",
        plan.name,
        plan.tasks.len(),
        plan.horizon_ms
    ));
    lines.push(String::new());
    lines.push("mod rt;".into());
    lines.push("mod service;".into());
    lines.push(String::new());

    lines.push("/// The system's declared name.".into());
    lines.push(format!("const SYSTEM: &str = \"{}\";", plan.name));
    records.push(Record {
        declaration: "system",
        name: plan.name.clone(),
        generated_file: "src/main.rs",
        generated_line: lines.len(),
        span: plan.span,
        via: "interpret: the name of the single `defsystem`",
    });
    lines.push(String::new());

    lines.push(
        "/// The observation horizon: the least common multiple of the declared periods.".into(),
    );
    lines.push(format!("const HORIZON_MS: u64 = {};", plan.horizon_ms));
    records.push(Record {
        declaration: "hyperperiod",
        name: format!("{} ms", plan.horizon_ms),
        generated_file: "src/main.rs",
        generated_line: lines.len(),
        span: plan.span,
        via: "interpret: lcm of every declared period, in whole milliseconds",
    });
    lines.push(String::new());

    lines.push(
        "/// The workload, as the plan fixed it. Ordering here is declaration order; the runtime"
            .into(),
    );
    lines.push("/// orders releases by time and then by priority rank.".into());
    lines.push("static TASKS: &[rt::Task] = &[".into());
    for task in &plan.tasks {
        lines.push(format!(
            "    rt::Task {{ name: \"{}\", period_ms: {}, priority: {} }},",
            task.name, task.period_ms, task.priority
        ));
        records.push(Record {
            declaration: "task",
            name: task.name.clone(),
            generated_file: "src/main.rs",
            generated_line: lines.len(),
            span: task.span,
            via: "interpret: a `(task …)` clause of the system, period converted to whole milliseconds",
        });
    }
    lines.push("];".into());
    lines.push(String::new());

    lines.push("fn main() {".into());
    lines.push("    // `console.write` was realized by the hosted-playground console.".into());
    lines.push("    let mut console = service::Console::new();".into());
    lines.push("    rt::run(SYSTEM, TASKS, HORIZON_MS, &mut console);".into());
    lines.push("}".into());

    let mut text = lines.join("\n");
    text.push('\n');
    (text, records)
}

#[cfg(test)]
mod tests {
    use super::{main_rs, manifest, package_name, runtime_files};
    use crate::interpret::{Plan, Task};
    use eadl_front::{SourceId, Span};

    fn plan() -> Plan {
        let span = Span::new(SourceId(0), 0, 1);
        Plan {
            name: "heartbeat".into(),
            tasks: vec![
                Task {
                    name: "beat".into(),
                    period_ms: 10,
                    priority: 1,
                    span,
                },
                Task {
                    name: "chime".into(),
                    period_ms: 30,
                    priority: 2,
                    span,
                },
            ],
            horizon_ms: 30,
            span,
        }
    }

    #[test]
    fn emitted_runtime_is_the_reviewed_runtime() {
        // ⭐ The property that makes "reviewed reusable Rust" mean something. The bytes written
        // into the generated crate are the same bytes this crate compiles and tests, so a change
        // to the runtime cannot reach users without going through review here.
        let emitted: Vec<&str> = runtime_files().iter().map(|(_, text)| *text).collect();
        assert_eq!(emitted[0], include_str!("runtime/rt.rs"));
        assert_eq!(emitted[1], include_str!("runtime/service.rs"));
        assert!(
            emitted.iter().all(|text| !text.contains("crate::")),
            "the emitted runtime must not reference `crate::`: in the generated crate these are \
             sibling modules at the root, so only `super::` resolves in both layouts"
        );
    }

    #[test]
    fn generation_is_deterministic() {
        // §10.3: "deterministic generated sources from locked inputs". A timestamp or a path in
        // a header would quietly cost this.
        assert_eq!(main_rs(&plan()).0, main_rs(&plan()).0);
        assert!(
            !main_rs(&plan()).0.contains("2026"),
            "no date may be emitted"
        );
    }

    #[test]
    fn every_provenance_record_names_a_line_that_contains_it() {
        // ⭐ The property that makes a provenance record worth having. A record pointing at a
        // line which does not contain what it claims is worse than no record at all: it sends a
        // reader somewhere confidently wrong. Checked here against the generated side; the test
        // suite checks the source side against the description.
        let (text, records) = main_rs(&plan());
        let lines: Vec<&str> = text.lines().collect();
        assert_eq!(records.len(), 4, "system, hyperperiod, and one per task");
        for record in &records {
            let line = lines
                .get(record.generated_line - 1)
                .unwrap_or_else(|| panic!("line {} is past the end", record.generated_line));
            let needle = match record.declaration {
                "hyperperiod" => "HORIZON_MS".to_string(),
                _ => record.name.clone(),
            };
            assert!(
                line.contains(&needle),
                "record {:?} points at line {} — `{line}` — which does not contain `{needle}`",
                record.declaration,
                record.generated_line
            );
        }
    }

    #[test]
    fn the_specialized_file_carries_the_plan_and_nothing_invented() {
        let (text, _) = main_rs(&plan());
        assert!(
            text.contains("const SYSTEM: &str = \"heartbeat\";"),
            "{text}"
        );
        assert!(text.contains("const HORIZON_MS: u64 = 30;"), "{text}");
        assert!(
            text.contains("rt::Task { name: \"beat\", period_ms: 10, priority: 1 },"),
            "{text}"
        );
        assert!(
            text.contains("rt::Task { name: \"chime\", period_ms: 30, priority: 2 },"),
            "{text}"
        );
    }

    #[test]
    fn every_generated_file_says_it_is_generated_and_experimental() {
        // §12 S0: "Mark the output experimental, with no claim of OS completeness or real-time
        // assurance." A file that does not say so is one someone will later quote as evidence.
        for text in [manifest("s0-heartbeat"), main_rs(&plan()).0] {
            assert!(text.contains("GENERATED by archogen"), "{text}");
            assert!(text.contains("Do not edit"), "{text}");
            assert!(text.contains("EXPERIMENTAL"), "{text}");
        }
    }

    #[test]
    fn the_manifest_makes_the_generated_crate_its_own_workspace() {
        // Without this, generating into a directory inside any Cargo workspace produces a crate
        // that refuses to build — "current package believes it's in a workspace when it's not".
        assert!(manifest("s0-x").contains("\n[workspace]\n"));
    }

    #[test]
    fn package_names_are_total_and_prefixed() {
        assert_eq!(package_name("heartbeat"), "s0-heartbeat");
        assert_eq!(package_name("app.heart_beat"), "s0-app-heart-beat");
        assert_eq!(package_name("Weird..Name."), "s0-weird-name");
    }
}
