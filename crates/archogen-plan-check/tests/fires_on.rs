//! The `fires on` cells whose verb is `plan` (leaf `M3.5.8`), run where the configuration checker lives: each normative
//! document's diagnostics table read by the shared `reference_table` reader, each `plan T <plan> R` cell run through
//! the `archogen-plan-check` executable — `T` written as `app.eadl` in a scratch directory, `R` as `plan.eadl` with
//! `<digest>` the digest of the program `T` becomes — and the row's code required among the record's findings, the
//! record's status its exit code. The generator's driver, `crates/archogen-cli/tests/fires_on.rs`, parses these cells
//! and runs none, so the generator's package compiles no checker; every row whose code begins `plan-` carries such a
//! cell, so none is left to neither.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_outcome::record::Record;
use archogen_plan_check::assignment::program_digest;
use archogen_plan_check::check::plan_file;
use eadl_front::module::DirectoryModules;
use eadl_front::SourceMap;
use eadl_model::profile::RT_STATIC_UP_V1;

#[allow(dead_code)]
#[path = "../../eadl-front/tests/common/reference_table.rs"]
mod reference_table;

use reference_table::{machine_table, source_text};

const EXE: &str = env!("CARGO_BIN_EXE_archogen-plan-check");

/// A fresh scratch directory under the target directory, one per cell.
fn scratch() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "fires-on-plan-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

/// Run one `plan` cell's input: the codes the record's findings carry, or why the cell could not run.
fn fire(rest: &str) -> Result<BTreeSet<String>, String> {
    let (description, record) = rest
        .split_once("<plan>")
        .ok_or("no `<plan>` between the description and the record")?;
    let decode = |text: &str| {
        source_text(text.trim())
            .ok_or_else(|| format!("`{text}` holds a marker that decodes to nothing"))
    };
    let (description, record) = (decode(description)?, decode(record)?);
    let dir = scratch();
    let app = dir.join("app.eadl");
    std::fs::write(&app, &description).map_err(|e| e.to_string())?;
    let mut sources = SourceMap::new();
    let id = sources
        .add(app.display().to_string(), description)
        .map_err(|e| e.to_string())?;
    let modules = DirectoryModules::new(&dir);
    let (program, _) = plan_file(
        &mut sources,
        id,
        &modules,
        &RT_STATIC_UP_V1,
        "plan.eadl",
        "",
    );
    let plan = dir.join("plan.eadl");
    std::fs::write(&plan, record.replace("<digest>", &program_digest(&program)))
        .map_err(|e| e.to_string())?;
    let result = dir.join("result.json");
    let output = Command::new(EXE)
        .arg("--result")
        .arg(&result)
        .arg("--")
        .arg(&app)
        .arg(&plan)
        .env_clear()
        .output()
        .map_err(|e| e.to_string())?;
    let record = std::fs::read(&result).map_err(|e| {
        format!(
            "no record: {e}; stderr: {}",
            String::from_utf8_lossy(&output.stderr)
        )
    })?;
    let record = Record::read(&record, 1 << 20).map_err(|e| e.to_string())?;
    if output.status.code() != Some(record.status().code()) {
        return Err(format!(
            "exit {:?} and the record's status `{}` disagree",
            output.status.code(),
            record.status().slug()
        ));
    }
    Ok(record.findings().iter().map(|f| f.code.clone()).collect())
}

/// Every normative document under `docs/semantics/` that carries a diagnostics table, as the generator's driver finds
/// them.
fn normative_documents() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let mut out: Vec<(String, String)> = std::fs::read_dir(root.join("docs/semantics"))
        .expect("docs/semantics")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "md"))
        .filter_map(|path| {
            let text = std::fs::read_to_string(&path).ok()?;
            text.contains("<!-- machine-read: diagnostics -->")
                .then(|| (path.display().to_string(), text))
        })
        .collect();
    out.sort();
    out
}

#[test]
fn every_plan_cell_fires_its_code_through_the_executable() {
    let documents = normative_documents();
    assert!(
        documents.len() >= 2,
        "found {documents:?} — the walk is broken"
    );
    let (mut wrong, mut ran) = (Vec::new(), 0);
    for (path, text) in &documents {
        for (line, cells) in machine_table(text, "diagnostics") {
            let (Some(code), Some(cell)) = (cells.first(), cells.get(3)) else {
                continue;
            };
            let rest = cell.trim().strip_prefix("plan ");
            match rest {
                None if code.starts_with("plan-") => wrong.push(format!(
                    "{path}:{line}: `{code}` is a plan code, and its `fires on` cell is no `plan` input"
                )),
                None => {}
                Some(rest) => {
                    ran += 1;
                    match fire(rest) {
                        Ok(fired) if fired.contains(code) => {}
                        Ok(fired) => wrong.push(format!(
                            "{path}:{line}: `{code}`'s input fires {fired:?}, not `{code}`"
                        )),
                        Err(why) => wrong.push(format!("{path}:{line}: `{code}`'s input does not run: {why}")),
                    }
                }
            }
        }
    }
    assert!(
        ran >= 6,
        "ran {ran} plan cell(s) — the tables lost their plan rows"
    );
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
