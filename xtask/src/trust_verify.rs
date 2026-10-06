//! `cargo xtask trust-verify <package>` — the trust gate's case 4, judged where an inventory is consumed (leaf
//! `M3.6.3.5`, `docs/specs/trust/decision_trust-inventory.md` §6, ledger `TI-H8`).
//!
//! ⭐ **The gate cannot find its own inventory stale** (R1 A8): it builds the inventory it judges. A package is
//! different — it carries an inventory, a report and artifacts made at some commit with some toolchain, and the claims
//! in it rest on those being one build. This verifier refuses a package whose parts do not agree.
//!
//! ⛔ **The package is `M4.7`'s, and provisional here.** The record leaves the package, and how a result names the
//! program that produced it, to `M4.7` (§5, `TI-H14`), and asks `M3.6.3` for the verifier and its tests on package
//! directories made for the purpose. So the layout below is what the verifier reads, the input `M4.7`'s packaging
//! must meet (`TI-H15`), and it moves with the verifier if `M4.7` needs more:
//!
//! ```text
//! <package>/manifest.json                   {"format": "archogen-assurance-package/0", "commit", "toolchain",
//!                                             "artifacts": [{"program", "path", "sha256"}],
//!                                             "results": [{"claim", "role", "producer", "subject"}],
//!                                             "handed": [{"root", "path", "sha256"}]}
//! <package>/trust/trust-dependencies.json   the inventory (`cargo xtask trust-inventory`)
//! <package>/trust/report.txt                the gate's report (`cargo xtask trust-gate`)
//! <package>/<each artifact's path>
//! ```
//!
//! Exit 0: verified. 1: refused, every refusal `trust-inventory-stale`. 2: the package is not a directory.

use std::fs;
use std::path::Path;

use archogen_evidence::sha256::Digest;

use crate::json::{self, Json};

/// The manifest's format, which the verifier reads and nothing else.
pub const FORMAT: &str = "archogen-assurance-package/0";

const CODE: &str = "trust-inventory-stale";

fn text<'a>(j: &'a Json, key: &str) -> &'a str {
    j.get(key).and_then(Json::as_str).unwrap_or("")
}

/// Every refusal of the package at `dir`; empty when it is verified.
///
/// # Errors
///
/// `dir` is not a directory.
pub fn verify(dir: &Path) -> Result<Vec<String>, String> {
    if !dir.is_dir() {
        return Err(format!("{} is not a directory", dir.display()));
    }
    let mut refused = Vec::new();
    let mut refuse = |why: String| refused.push(format!("{CODE}: {why}"));
    let read_json = |rel: &str| -> Option<Json> {
        fs::read_to_string(dir.join(rel))
            .ok()
            .and_then(|t| json::parse(&t).ok())
    };
    let Some(manifest) = read_json("manifest.json") else {
        refuse("the package has no `manifest.json` the verifier can read".to_owned());
        return Ok(refused);
    };
    if text(&manifest, "format") != FORMAT {
        refuse(format!(
            "`manifest.json` is not `{FORMAT}`: `{}`",
            text(&manifest, "format")
        ));
        return Ok(refused);
    }
    let inventory_path = "trust/trust-dependencies.json";
    let Ok(inventory_bytes) = fs::read(dir.join(inventory_path)) else {
        refuse(format!("the inventory is missing: no `{inventory_path}`"));
        return Ok(refused);
    };
    let Ok(inventory) = json::parse(&String::from_utf8_lossy(&inventory_bytes)) else {
        refuse(format!("`{inventory_path}` is not an inventory"));
        return Ok(refused);
    };

    // The build identity: the package's commit and toolchain (case 4).
    let identity = |k: &str| {
        inventory
            .get("identity")
            .and_then(|i| i.get(k))
            .and_then(Json::as_str)
            .unwrap_or("")
            .to_owned()
    };
    for (key, what) in [("commit", "commit"), ("rustc", "toolchain")] {
        let mine = text(&manifest, if key == "rustc" { "toolchain" } else { key });
        if identity(key) != mine || mine.is_empty() {
            refuse(format!(
                "the inventory is of another build: its {what} is `{}`, the package's `{mine}`",
                identity(key).lines().next().unwrap_or("")
            ));
        }
    }

    // The report names the inventory it describes by sha256 (R6 9).
    let inventory_sha = Digest::of(&inventory_bytes).hex();
    match fs::read_to_string(dir.join("trust/report.txt")) {
        Err(_) => refuse("the report is missing: no `trust/report.txt`".to_owned()),
        Ok(report) => {
            let named = report
                .lines()
                .find_map(|l| l.strip_prefix("inventory: sha256 "))
                .unwrap_or("");
            if named != inventory_sha {
                refuse(format!(
                    "the report is of another inventory: it names `{named}`, the package's is `{inventory_sha}`"
                ));
            }
        }
    }

    // Each program the inventory records: its role, its pair if it is the harness, its artifact's sha256 and the data
    // its form declares.
    let programs = inventory
        .get("programs")
        .map(Json::elements)
        .unwrap_or_default();
    let program = |name: &str| programs.iter().find(|p| text(p, "name") == name);
    let artifact_sha = |p: &Json| {
        p.get("artifact")
            .map(|a| text(a, "sha256"))
            .unwrap_or("")
            .to_owned()
    };

    // The artifacts the package ships are the trust build's: the bytes, the listed digest and the inventory's agree.
    for a in manifest
        .get("artifacts")
        .map(Json::elements)
        .unwrap_or_default()
    {
        let (name, path, listed) = (text(a, "program"), text(a, "path"), text(a, "sha256"));
        let Some(p) = program(name) else {
            refuse(format!(
                "the artifact `{path}` names `{name}`, which the inventory does not record"
            ));
            continue;
        };
        let actual = fs::read(dir.join(path))
            .map(|b| Digest::of(&b).hex())
            .unwrap_or_default();
        let inventoried = artifact_sha(p);
        if actual != listed || listed != inventoried {
            refuse(format!(
                "`{name}`'s artifact `{path}` is not the inventory's: its bytes `{}`, listed `{listed}`, inventoried `{inventoried}`",
                if actual.is_empty() { "missing" } else { &actual }
            ));
        }
    }

    // A result names the program that produced it: its role's root, or for the reference model the harness of its
    // pair, so a result a debug build or another commit's checker produced does not pass (R5 4).
    for r in manifest
        .get("results")
        .map(Json::elements)
        .unwrap_or_default()
    {
        let (claim, role, producer) = (text(r, "claim"), text(r, "role"), text(r, "producer"));
        let roots_of_role: Vec<&Json> = programs
            .iter()
            .filter(|p| text(p, "role") == role)
            .collect();
        let by_root = roots_of_role.iter().any(|p| artifact_sha(p) == producer);
        let by_harness = role == "reference-model"
            && programs.iter().any(|h| {
                text(h, "role") == "harness"
                    && artifact_sha(h) == producer
                    && h.get("pair")
                        .map(Json::elements)
                        .unwrap_or_default()
                        .iter()
                        .filter_map(Json::as_str)
                        .any(|n| roots_of_role.iter().any(|p| text(p, "name") == n))
            });
        if producer.is_empty() || !(by_root || by_harness) {
            refuse(format!(
                "the result `{claim}` names `{producer}` as its producer, which is neither the inventoried artifact of a `{role}` root nor its pair's harness"
            ));
        }
    }

    // A dependency handed to a root at run time is declared by its form, by path and the inventory's sha256 (R5 6).
    for h in manifest
        .get("handed")
        .map(Json::elements)
        .unwrap_or_default()
    {
        let (root, path, sha) = (text(h, "root"), text(h, "path"), text(h, "sha256"));
        let declared = program(root)
            .and_then(|p| p.get("data"))
            .and_then(|d| d.get(path))
            .and_then(Json::as_str);
        if declared != Some(sha) {
            refuse(format!(
                "`{path}` is handed to `{root}`, whose form does not declare it with the inventory's sha256 `{sha}`"
            ));
        }
    }
    Ok(refused)
}

/// `cargo xtask trust-verify <package>`.
pub fn run(args: &[&str]) -> i32 {
    let [package] = args else {
        eprintln!("trust-verify: write `cargo xtask trust-verify <package>`");
        return 2;
    };
    match verify(Path::new(package)) {
        Ok(refused) if refused.is_empty() => {
            println!("trust-verify: {package} verified — its inventory, report, artifacts, results and handed dependencies are one build's");
            0
        }
        Ok(refused) => {
            for r in &refused {
                eprintln!("{r}");
            }
            eprintln!(
                "trust-verify: {package} refused — {} refusal(s)",
                refused.len()
            );
            1
        }
        Err(e) => {
            eprintln!("trust-verify: {e}");
            2
        }
    }
}

#[cfg(test)]
mod tests {
    //! Each refusal of `TI-H8` on a package directory made for it, under `target/trust-verify-tests/`.

    use super::{verify, FORMAT};
    use archogen_evidence::sha256::Digest;
    use std::path::{Path, PathBuf};

    const CHECKER: &[u8] = b"the checker's executable";
    const HARNESS: &[u8] = b"the comparison harness";
    const TABLE: &[u8] = b"a fixed table";

    fn sha(b: &[u8]) -> String {
        Digest::of(b).hex()
    }

    /// A package every check passes, as `M4.7`'s packaging is to write one; `edit` changes it before it is written.
    fn package(name: &str, edit: impl FnOnce(&mut Parts)) -> PathBuf {
        let dir = Path::new(env!("CARGO_MANIFEST_DIR"))
            .parent()
            .unwrap()
            .join("target/trust-verify-tests")
            .join(name);
        let _ = std::fs::remove_dir_all(&dir);
        let mut parts = Parts::default();
        edit(&mut parts);
        let inventory = format!(
            r#"{{"identity":{{"commit":"c0ffee","rustc":"rustc 1.95.0\nhost: x"}},"programs":[
              {{"name":"chk","role":"scheduling-checker","pair":[],"artifact":{{"path":"release/chk","sha256":"{}"}},"data":{{"docs/table.txt":"{}"}}}},
              {{"name":"refm","role":"reference-model","pair":[],"artifact":{{"path":"release/librefm.rlib","sha256":"r"}},"data":{{}}}},
              {{"name":"imp","role":"implementation","pair":[],"artifact":{{"path":"release/libimp.rlib","sha256":"i"}},"data":{{}}}},
              {{"name":"diff","role":"harness","pair":["refm","imp"],"artifact":{{"path":"release/diff","sha256":"{}"}},"data":{{}}}}]}}"#,
            sha(CHECKER),
            sha(TABLE),
            sha(HARNESS)
        );
        let write = |rel: &str, bytes: &[u8]| {
            let p = dir.join(rel);
            std::fs::create_dir_all(p.parent().unwrap()).unwrap();
            std::fs::write(p, bytes).unwrap();
        };
        let inventory = parts.inventory.take().unwrap_or(inventory);
        if !parts.no_inventory {
            write("trust/trust-dependencies.json", inventory.as_bytes());
        }
        let named = parts
            .report_names
            .clone()
            .unwrap_or_else(|| sha(inventory.as_bytes()));
        if !parts.no_report {
            write(
                "trust/report.txt",
                format!("trust-gate report\ninventory: sha256 {named}\n").as_bytes(),
            );
        }
        write(
            "artifacts/chk",
            parts.checker_bytes.as_deref().unwrap_or(CHECKER),
        );
        write("artifacts/diff", HARNESS);
        let manifest = format!(
            r#"{{"format":"{FORMAT}","commit":"{}","toolchain":"rustc 1.95.0\nhost: x",
              "artifacts":[{{"program":"chk","path":"artifacts/chk","sha256":"{}"}},{{"program":"diff","path":"artifacts/diff","sha256":"{}"}}],
              "results":[{{"claim":"schedulable","role":"scheduling-checker","producer":"{}","subject":"plan"}},
                         {{"claim":"reference-agrees","role":"reference-model","producer":"{}","subject":"trace"}}{}],
              "handed":[{{"root":"chk","path":"docs/table.txt","sha256":"{}"}}]}}"#,
            parts.commit.as_deref().unwrap_or("c0ffee"),
            sha(CHECKER),
            sha(HARNESS),
            parts.producer.clone().unwrap_or_else(|| sha(CHECKER)),
            sha(HARNESS),
            parts.implementation_producer.as_ref().map_or(String::new(), |p| format!(
                r#",{{"claim":"implemented","role":"implementation","producer":"{p}","subject":"trace"}}"#
            )),
            parts.handed.clone().unwrap_or_else(|| sha(TABLE)),
        );
        if !parts.no_manifest {
            write("manifest.json", manifest.as_bytes());
        }
        dir
    }

    #[derive(Default)]
    struct Parts {
        no_manifest: bool,
        no_inventory: bool,
        no_report: bool,
        inventory: Option<String>,
        commit: Option<String>,
        report_names: Option<String>,
        checker_bytes: Option<Vec<u8>>,
        producer: Option<String>,
        implementation_producer: Option<String>,
        handed: Option<String>,
    }

    fn refused(dir: &Path, want: &str) {
        let r = verify(dir).expect("a directory");
        assert!(
            r.iter().any(|x| x.contains(want)),
            "no refusal says `{want}`: {r:#?}"
        );
        assert!(
            r.iter().all(|x| x.starts_with("trust-inventory-stale: ")),
            "{r:#?}"
        );
    }

    #[test]
    fn a_package_whose_parts_are_one_build_s_is_verified() {
        let r = verify(&package("whole", |_| {})).expect("a directory");
        assert!(r.is_empty(), "{r:#?}");
    }

    #[test]
    fn a_package_without_its_manifest_or_its_inventory_is_refused() {
        refused(
            &package("no-manifest", |p| p.no_manifest = true),
            "no `manifest.json`",
        );
        refused(
            &package("no-inventory", |p| p.no_inventory = true),
            "the inventory is missing",
        );
    }

    #[test]
    fn an_inventory_of_another_commit_or_toolchain_is_refused() {
        refused(
            &package("other-commit", |p| p.commit = Some("deadbeef".to_owned())),
            "the inventory is of another build: its commit is `c0ffee`, the package's `deadbeef`",
        );
        refused(
            &package("other-toolchain", |p| {
                p.inventory = Some(r#"{"identity":{"commit":"c0ffee","rustc":"rustc 1.94.0\nhost: x"},"programs":[]}"#.to_owned());
            }),
            "its toolchain is `rustc 1.94.0`",
        );
    }

    #[test]
    fn an_artifact_that_is_not_the_inventory_s_is_refused() {
        refused(
            &package("other-artifact", |p| {
                p.checker_bytes = Some(b"a debug build".to_vec())
            }),
            "`chk`'s artifact `artifacts/chk` is not the inventory's",
        );
    }

    #[test]
    fn a_report_missing_or_of_another_inventory_is_refused() {
        refused(
            &package("no-report", |p| p.no_report = true),
            "the report is missing",
        );
        refused(
            &package("other-report", |p| p.report_names = Some("0".repeat(64))),
            "the report is of another inventory",
        );
    }

    #[test]
    fn a_result_produced_by_another_program_is_refused() {
        // R5 4: a result a debug build or another commit's checker produced, and one claiming the harness for the
        // scheduling checker's role.
        refused(
            &package("other-producer", |p| {
                p.producer = Some(sha(b"another commit's checker"))
            }),
            "the result `schedulable` names",
        );
        refused(
            &package("harness-for-checker", |p| p.producer = Some(sha(HARNESS))),
            "neither the inventoried artifact of a `scheduling-checker` root nor its pair's harness",
        );
        // TI-H14: the harness produces the reference model's results alone — not the implementation's, though the
        // implementation is in its pair.
        refused(
            &package("harness-for-implementation", |p| {
                p.implementation_producer = Some(sha(HARNESS));
            }),
            "the result `implemented` names",
        );
        let r = verify(&package("implementation-by-its-root", |p| {
            p.implementation_producer = Some("i".to_owned());
        }))
        .expect("a directory");
        assert!(
            r.is_empty(),
            "the implementation's own artifact produces its results: {r:#?}"
        );
    }

    #[test]
    fn a_handed_dependency_the_root_s_form_does_not_declare_is_refused() {
        refused(
            &package("undeclared-handed", |p| {
                p.handed = Some(sha(b"another table"))
            }),
            "`docs/table.txt` is handed to `chk`, whose form does not declare it",
        );
    }

    #[test]
    fn a_path_that_is_not_a_directory_is_not_judged() {
        assert!(verify(Path::new("/nonexistent/package")).is_err());
    }
}
