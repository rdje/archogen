//! The `fires on` notation: a diagnostics-table cell that is an **input**, run through the pipeline it names
//! (leaf `M1.26.2.1`; the column itself lands in the normative documents under `M1.26.2.2`).
//!
//! ⭐ **Why a cell must be executable.** A table that states a code, checked only against the source that emits
//! it, stays green for a call site no input can reach: the scan finds `Diagnostic::error("…")` in the
//! production half and asks nothing more. `module-too-large`'s row stated a mechanism that exists nowhere, and
//! both census legs were green. A cell that runs, and must fire its row's code, is what makes an unreachable
//! row fail.
//!
//! **The verbs**, because inputs reach codes through different doors:
//! - `check <text>` — the text is a description, checked by `archogen check` from a scratch directory;
//! - `modules <text> <mod NAME> <text> …` — a description, and each `<mod NAME>` segment a module file
//!   `NAME.eadl` beside it; five `module-*` codes fire only over two or more modules;
//! - `kinds <text>` — a kind module, loaded by `shipped_registry`, the loader `archogen check` uses. The CLI
//!   registers only the kinds it embeds, so no description can reach a `defkind`-level `schema-*` code. After
//!   an optional `<validate>`, declarations are validated against the registry that module built;
//! - `none: <reason>` — no writable input fires the code. The reason is review, and it may not be empty.
//!
//! `<0xNN>` names a character, decoded by the same `reference_table` reader the reference's other tables use —
//! one reader for one notation. A cell nothing can parse is a **violation**, never a skip.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_cli::run;
use eadl_front::SourceMap;
use eadl_model::check::shipped_registry;
use eadl_model::kind::validate;

#[allow(dead_code)]
#[path = "../../eadl-front/tests/common/reference_table.rs"]
mod reference_table;

use reference_table::{machine_table, source_text};

/// What a `fires on` cell asks for.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Probe {
    Check(String),
    Modules(String, Vec<(String, String)>),
    /// A kind module, and the declarations validated against the registry it builds (`<validate>`).
    Kinds(String, String),
    Limit(String),
}

/// Parse a cell into a probe, or say why it cannot be one.
fn parse_cell(cell: &str) -> Result<Probe, String> {
    let cell = cell.trim();
    if let Some(reason) = cell.strip_prefix("none:") {
        let reason = reason.trim();
        return if reason.is_empty() {
            Err(
                "`none:` states a limit and gives no reason — the reason is what a reviewer reads"
                    .to_string(),
            )
        } else {
            Ok(Probe::Limit(reason.to_string()))
        };
    }
    let (verb, rest) = cell
        .split_once(' ')
        .ok_or_else(|| format!("`{cell}` names no verb and no input"))?;
    let decode = |text: &str| {
        source_text(text.trim())
            .ok_or_else(|| {
                format!(
                    "`{text}` holds a marker that decodes to nothing — `<0xNN>` must name a character, \
                     and `<N*C>` one character, at least once"
                )
            })
    };
    match verb {
        "check" => Ok(Probe::Check(decode(rest)?)),
        "kinds" => {
            let (module, declarations) = rest.split_once("<validate>").unwrap_or((rest, ""));
            Ok(Probe::Kinds(decode(module)?, decode(declarations)?))
        }
        "modules" => {
            let mut segments = rest.split("<mod ");
            let root = decode(segments.next().unwrap_or(""))?;
            let mut modules = Vec::new();
            for segment in segments {
                let (name, text) = segment
                    .split_once('>')
                    .ok_or_else(|| format!("`<mod {segment}` is not closed"))?;
                if name.is_empty() || name.contains(char::is_whitespace) {
                    return Err(format!("`<mod {name}>` does not name a module"));
                }
                modules.push((name.to_string(), decode(text)?));
            }
            if modules.is_empty() {
                return Err(
                    "`modules` names no module — use `check` for one description".to_string(),
                );
            }
            Ok(Probe::Modules(root, modules))
        }
        other => Err(format!(
            "`{other}` is not a verb — the verbs are `check`, `modules`, `kinds` and `none:`"
        )),
    }
}

/// A fresh scratch directory under `CARGO_TARGET_TMPDIR`, one per probe, so parallel tests share nothing.
fn scratch() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR"))
        .join("fires-on")
        .join(format!(
            "{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::SeqCst)
        ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("the scratch directory");
    dir
}

/// The codes `archogen check` renders for `path`.
fn checked_codes(path: &Path) -> BTreeSet<String> {
    let (mut out, mut err) = (Vec::new(), Vec::new());
    let _ = run(
        ["check".to_string(), path.display().to_string()],
        &mut out,
        &mut err,
    );
    let text = format!(
        "{}{}",
        String::from_utf8_lossy(&err),
        String::from_utf8_lossy(&out)
    );
    let mut codes = BTreeSet::new();
    for piece in text.split("error[").skip(1) {
        if let Some((code, _)) = piece.split_once(']') {
            codes.insert(code.to_string());
        }
    }
    codes
}

/// Run a probe; the codes it fires. A limit fires nothing.
fn fire(probe: &Probe) -> BTreeSet<String> {
    match probe {
        Probe::Limit(_) => BTreeSet::new(),
        Probe::Check(text) => {
            let dir = scratch();
            let path = dir.join("system.eadl");
            std::fs::write(&path, text).expect("writable");
            checked_codes(&path)
        }
        Probe::Modules(root, modules) => {
            let dir = scratch();
            let path = dir.join("system.eadl");
            std::fs::write(&path, root).expect("writable");
            for (name, text) in modules {
                std::fs::write(dir.join(format!("{name}.eadl")), text).expect("writable");
            }
            checked_codes(&path)
        }
        Probe::Kinds(module, declarations) => {
            // The production loader itself — the one `archogen check` builds its registry with — so a kind
            // module's codes fire exactly as they would for a shipped one. Then each declaration after
            // `<validate>` is validated against that registry, which is the only door to a code that fires on
            // a kind a clause names and no registry holds: no description can register a kind.
            let mut sources = SourceMap::new();
            match shipped_registry(&mut sources, &[("kinds.eadl".to_string(), module.clone())]) {
                Err(diagnostics) => diagnostics.iter().map(|d| d.code.to_string()).collect(),
                Ok(registry) => {
                    let id = sources
                        .add("declarations.eadl", declarations.clone())
                        .expect("a small text");
                    let (document, read) = eadl_front::read(&sources, id);
                    let mut codes: BTreeSet<String> =
                        read.items().iter().map(|d| d.code.to_string()).collect();
                    for form in &document.forms {
                        codes.extend(validate(&registry, form).iter().map(|d| d.code.to_string()));
                    }
                    codes
                }
            }
        }
    }
}

/// Every way a diagnostics table's `fires on` column disagrees with the pipeline: a row with no cell, a cell
/// that is not a probe, and a probe that does not fire its row's code.
fn fires_violations(path: &str, document: &str) -> Vec<String> {
    let mut out = Vec::new();
    let rows = machine_table(document, "diagnostics");
    if rows.is_empty() {
        out.push(format!(
            "{path} carries no diagnostics table, so no cell was executed"
        ));
    }
    for (line, cells) in rows {
        let Some(code) = cells.first() else { continue };
        let Some(cell) = cells.get(3) else {
            out.push(format!("{path}:{line}: `{code}` has no `fires on` cell"));
            continue;
        };
        match parse_cell(cell) {
            Err(reason) => out.push(format!(
                "{path}:{line}: `{code}`'s `fires on` cell is not an input: {reason}"
            )),
            Ok(Probe::Limit(_)) => {}
            Ok(probe) => {
                let fired = fire(&probe);
                if !fired.contains(code) {
                    out.push(format!(
                        "{path}:{line}: `{code}`'s input fires {fired:?}, not `{code}` — a row whose input no longer \
                         reaches its code reads exactly like a live rule"
                    ));
                }
            }
        }
    }
    out
}

// ── the arms, against inline tables (the real documents are `M1.26.2.2`'s) ─────────────────────────

fn table(rows: &[&str]) -> String {
    let mut text = "<!-- machine-read: diagnostics -->\n| code | when it fires | what to do | fires on |\n| --- | --- | --- | --- |\n".to_string();
    for row in rows {
        text.push_str(row);
        text.push('\n');
    }
    text
}

fn assert_reported(wrong: &[String], expected: usize, needles: &[&str]) {
    assert_eq!(
        wrong.len(),
        expected,
        "the leg reported:\n{}",
        wrong.join("\n")
    );
    for needle in needles {
        assert!(
            wrong.iter().any(|w| w.contains(needle)),
            "no violation mentions {needle:?}:\n{}",
            wrong.join("\n")
        );
    }
}

#[test]
fn a_check_cell_that_fires_its_code_passes() {
    let doc = table(&["| `read-unclosed-list` | … | … | `check (defsystem heartbeat` |"]);
    assert_reported(&fires_violations("fixture.md", &doc), 0, &[]);
}

#[test]
fn a_cell_that_fires_another_code_is_reported_with_what_it_fired() {
    let doc = table(&["| `read-unexpected-close` | … | … | `check (defsystem heartbeat` |"]);
    assert_reported(
        &fires_violations("fixture.md", &doc),
        1,
        &[
            "`read-unexpected-close`'s input fires",
            "read-unclosed-list",
        ],
    );
}

#[test]
fn a_character_marker_is_decoded_by_the_shared_reader() {
    // A raw escape byte between forms is the input `read-unexpected-character` fires on, and it cannot be
    // written in a cell except by name.
    let doc = table(&["| `read-unexpected-character` | … | … | `check (a)<0x1b>` |"]);
    assert_reported(&fires_violations("fixture.md", &doc), 0, &[]);
}

#[test]
fn a_modules_cell_writes_the_module_files_beside_the_description() {
    let doc = table(&[
        "| `module-name-mismatch` | … | … | `modules (defmodule app (version 1 0) (import hw.misnamed)) <mod hw.misnamed>(defmodule hw.other (version 1 0))` |",
    ]);
    assert_reported(&fires_violations("fixture.md", &doc), 0, &[]);
}

#[test]
fn a_kinds_cell_runs_the_kind_loader() {
    let doc = table(&["| `schema-not-a-kind` | … | … | `kinds (defservice x)` |"]);
    assert_reported(&fires_violations("fixture.md", &doc), 0, &[]);
}

#[test]
fn declarations_after_validate_are_checked_against_the_module_s_registry() {
    // A kind whose clause names a kind nothing registers: only validation of a declaration can see it.
    let doc = table(&[
        "| `schema-unknown-referenced-kind` | … | … | `kinds (defkind defthing (doc \"a thing\") (name required) (clause c (cardinality one) (holds kind nosuch))) <validate>(defthing x (c (y)))` |",
    ]);
    assert_reported(&fires_violations("fixture.md", &doc), 0, &[]);
}

#[test]
fn a_stated_limit_passes_and_an_empty_one_is_reported() {
    let doc = table(&[
        "| `module-too-large` | … | … | `none: a module over 4 GiB, which no fixture can be` |",
        "| `quantity-invalid` | … | … | `none:` |",
    ]);
    assert_reported(
        &fires_violations("fixture.md", &doc),
        1,
        &["`quantity-invalid`", "gives no reason"],
    );
}

#[test]
fn a_cell_that_is_not_an_input_is_a_violation_never_a_skip() {
    let doc = table(&[
        "| `read-bad-escape` | … | … | `parse (a)` |",
        "| `read-bad-escape` | … | … | `` |",
        "| `read-bad-escape` | … | … | `check (a)<0xd800>` |",
        "| `module-not-found` | … | … | `modules (import a (as a))` |",
        "| `read-nesting-too-deep` | … | … | `check <0*(>` |",
        "| `read-nesting-too-deep` | … | … | `check <3*()>` |",
    ]);
    assert_reported(
        &fires_violations("fixture.md", &doc),
        6,
        &[
            "`parse` is not a verb",
            "names no verb",
            "decodes to nothing",
            "names no module",
            "`<0*(>` holds a marker",
            "`<3*()>` holds a marker",
        ],
    );
}

#[test]
fn a_repetition_marker_is_decoded_by_the_shared_reader() {
    // Nesting past the limit is the input `read-nesting-too-deep` fires on, and 514 parentheses in a cell
    // could be read by nobody. One fewer level is under the limit and fires nothing, so the count is exact.
    let doc = table(&["| `read-nesting-too-deep` | … | … | `check <257*(><257*)>` |"]);
    assert_reported(&fires_violations("fixture.md", &doc), 0, &[]);
    let doc = table(&["| `read-nesting-too-deep` | … | … | `check <256*(><256*)>` |"]);
    assert_reported(
        &fires_violations("fixture.md", &doc),
        1,
        &["read-nesting-too-deep"],
    );
}

#[test]
fn a_row_without_the_column_is_reported() {
    let doc = "<!-- machine-read: diagnostics -->\n| code | when it fires | what to do |\n| --- | --- | --- |\n| `read-unclosed-list` | … | … |\n";
    assert_reported(
        &fires_violations("fixture.md", doc),
        1,
        &["has no `fires on` cell"],
    );
}

#[test]
fn a_document_with_no_diagnostics_table_is_reported() {
    assert_reported(
        &fires_violations("fixture.md", "no table here"),
        1,
        &["carries no diagnostics table"],
    );
}

// ── the column over the real documents (leaf `M1.26.2.2`) ───────────────────────────────────────────

/// Every normative document under `docs/semantics/` that carries a diagnostics table — derived, so a third
/// document is walked the day it is written.
fn normative_documents() -> Vec<(String, String)> {
    let root = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf();
    let mut out: Vec<(String, String)> = std::fs::read_dir(root.join("docs/semantics"))
        .expect("docs/semantics")
        .flatten()
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|e| e == "md"))
        .filter_map(|path| {
            let text = std::fs::read_to_string(&path).ok()?;
            text.contains("<!-- machine-read: diagnostics -->")
                .then(|| {
                    (
                        path.strip_prefix(&root)
                            .expect("under the root")
                            .display()
                            .to_string(),
                        text,
                    )
                })
        })
        .collect();
    out.sort();
    out
}

#[test]
fn every_row_of_every_normative_diagnostics_table_fires_its_code() {
    let documents = normative_documents();
    assert!(
        documents.len() >= 2,
        "found {documents:?} — the walk is broken, not the documents"
    );
    let mut wrong = Vec::new();
    for (path, text) in &documents {
        wrong.extend(fires_violations(path, text));
    }
    assert!(
        wrong.is_empty(),
        "{} row(s) whose input does not fire its code:\n\n{}",
        wrong.len(),
        wrong.join("\n\n")
    );
}
