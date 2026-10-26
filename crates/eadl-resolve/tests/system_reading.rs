//! `RP-H4`: the generator's reading of a system, `eadl_resolve::joint::System::read`, held equal to the configuration
//! checker's, `archogen_plan_check::joint::system`, which a context fenced from the generator wrote from
//! `docs/semantics/model.md` §8 alone (leaves `M3.5.5.8` and `M3.5.5.7.1`) — two readings written apart, compared by a
//! derivation of its own in `trust/roots.eadl` (leaf `M3.5.5.7.2`).
//!
//! ⭐ **One program, two readings.** Each description under the conformance suite's roots
//! (`docs/semantics/conformance.md`) and each joint fixture is read as the checker's executable reads it,
//! `check::plan_file`; where the program it becomes is one root instance, both readings take that instance's
//! declarations, so they read the same forms by the same names. Every system among them must then have the same
//! hardware — rule 7's blocks and platforms — reach the same declarations, and serve the same requesters: its own
//! level, its tasks and every declaration it reaches (rules 1 and 7). A disagreement is a defect in one of them, found
//! by a machine rather than by the next reader.
//!
//! ⭐ **The checker's other reading of rule 7 too.** Checks (2) and (3) read a system's hardware with
//! `requesters::hardware`, which a context that had read the generator's wrote: it is held to check (5)'s on every
//! system as well (leaf `M3.5.5.7.3`), so the checker reads one hardware for every check.
//!
//! ⭐ **And every shape the tracked systems do not hold.** The tracked systems are written to be read, so few name a
//! service from a block, a declaration from inside `offers` or a task's `needs`: a second leg generates systems — a
//! seeded sequence, the same on every run — whose declarations name each other at every position the language gives
//! `uses` and `needs`, and at those that are no edge, `offers` and `refines`, beside a name no declaration bears, and
//! holds the two readings equal on each.
//!
//! ⚠️ **What it does not compare.** A program of several instances: `System::read` reads one instance's declarations by
//! their own names, so the generator's reading of a program is `M3.4`'s, and the comparison takes such programs when it
//! lands; every program skipped must be a module case, so an example that starts importing fails here first. The rules
//! that judge an assignment's bindings, past rules 1 and 7: `M3.5.5.7.3` settles with the generator the readings §8
//! left open to the fenced context.

use std::collections::BTreeSet;
use std::path::{Path, PathBuf};

use archogen_plan_check::assignment::Requester;
use archogen_plan_check::check::{plain, plan_file};
use archogen_plan_check::{joint, requesters as checks};
use eadl_front::module::DirectoryModules;
use eadl_front::{read, Form, Program, SourceMap};
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::joint::System;

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// The suite's roots, as the manifest's `suite-roots` table lists them: each row's first cell, a directory in
/// backticks.
fn suite_roots(root: &Path) -> Vec<String> {
    let manifest = std::fs::read_to_string(root.join("docs/semantics/conformance.md"))
        .expect("the conformance manifest");
    manifest
        .lines()
        .skip_while(|line| line.trim() != "<!-- machine-read: suite-roots -->")
        .skip(3)
        .take_while(|line| line.starts_with('|'))
        .filter_map(|row| row.split('|').nth(1))
        .map(|cell| cell.trim().trim_matches('`').to_owned())
        .collect()
}

/// Every `.eadl` file under `dir`, in name order.
fn descriptions(dir: &Path, out: &mut Vec<PathBuf>) {
    let mut entries: Vec<PathBuf> = std::fs::read_dir(dir)
        .unwrap_or_else(|e| panic!("{}: {e}", dir.display()))
        .map(|entry| entry.expect("a directory entry").path())
        .collect();
    entries.sort();
    for entry in entries {
        if entry.is_dir() {
            descriptions(&entry, out);
        } else if entry.extension().is_some_and(|e| e == "eadl") {
            out.push(entry);
        }
    }
}

/// The name a declaration bears.
fn name_of(form: &Form) -> Option<&str> {
    form.items().get(1).and_then(Form::as_symbol)
}

/// A requester as the generator names it: a task `SYSTEM/TASK`.
fn generator_name(requester: &Requester) -> String {
    match requester {
        Requester::Named(name) => name.clone(),
        Requester::Task { system, task } => format!("{system}/{task}"),
    }
}

/// Every requester the description could name: each declaration, and each task of each system.
fn requesters(declarations: &[Form]) -> BTreeSet<Requester> {
    let mut all = BTreeSet::new();
    for form in declarations {
        let Some(name) = name_of(form) else {
            continue;
        };
        all.insert(Requester::Named(name.to_owned()));
        if form.head() == Some("defsystem") {
            for clause in form.items().iter().filter(|c| c.head() == Some("task")) {
                if let Some(task) = name_of(clause) {
                    all.insert(Requester::Task {
                        system: name.to_owned(),
                        task: task.to_owned(),
                    });
                }
            }
        }
    }
    all
}

/// Both readings of every system `program`'s one root instance declares, each difference pushed onto `wrong` as
/// `shown` names it; the number of systems compared.
fn compare(shown: &str, program: &Program, wrong: &mut Vec<String>) -> usize {
    let declarations = &program.instances[0].declarations;
    let candidates = requesters(declarations);
    let mut compared = 0;
    for decl in declarations
        .iter()
        .filter(|d| d.head() == Some("defsystem"))
    {
        let Some(name) = name_of(decl) else {
            continue;
        };
        compared += 1;
        let generator = System::read(decl, declarations);
        let Some(checker) = joint::system(program, name) else {
            wrong.push(format!(
                "{shown}: `{name}` is a system to the generator and none to the checker"
            ));
            continue;
        };
        if generator.hardware != checker.hardware {
            wrong.push(format!(
                "{shown}: `{name}`'s hardware — the generator's {:?}, the checker's {:?}",
                generator.hardware, checker.hardware
            ));
        }
        let shared = checks::hardware(program, &checks::declarations(program), name);
        if shared != checker.hardware {
            wrong.push(format!(
                "{shown}: `{name}`'s hardware — checks (2) and (3)'s {shared:?}, check (5)'s {:?}",
                checker.hardware
            ));
        }
        if generator.reached != checker.reached {
            wrong.push(format!(
                "{shown}: what `{name}` reaches — the generator's {:?}, the checker's {:?}",
                generator.reached, checker.reached
            ));
        }
        for requester in candidates.iter().chain(&checker.served) {
            let by_generator = generator.serves(&generator_name(requester));
            if by_generator != checker.served.contains(requester) {
                wrong.push(format!(
                    "{shown}: `{name}` serves `{}` to the {} alone",
                    requester.canonical(),
                    if by_generator { "generator" } else { "checker" }
                ));
            }
        }
    }
    compared
}

#[test]
fn the_generator_and_the_checker_read_every_system_alike() {
    let root = repo_root();
    let roots = suite_roots(&root);
    assert!(
        roots.len() >= 5,
        "the manifest lists {roots:?} — its table moved or the reader broke"
    );
    let mut files = Vec::new();
    for dir in &roots {
        descriptions(&root.join(dir), &mut files);
    }
    descriptions(&root.join("crates/eadl-resolve/tests/joint"), &mut files);
    let (mut compared, mut several, mut wrong) = (0, Vec::new(), Vec::new());
    for file in &files {
        let shown = file
            .strip_prefix(&root)
            .unwrap_or(file)
            .display()
            .to_string();
        let text = std::fs::read_to_string(file).unwrap_or_else(|e| panic!("{shown}: {e}"));
        let mut sources = SourceMap::new();
        let id = sources
            .add(shown.clone(), text)
            .expect("a description within the source map's bounds");
        let modules = DirectoryModules::new(file.parent().expect("a file's directory"));
        let (program, _) = plan_file(
            &mut sources,
            id,
            &modules,
            &RT_STATIC_UP_V1,
            "plan.eadl",
            "",
        );
        if program.instances.len() != 1 {
            several.push(shown);
            continue;
        }
        compared += compare(&shown, &program, &mut wrong);
    }
    assert!(
        compared >= 30,
        "compared {compared} system(s) over {} description(s) — the walk lost its corpus",
        files.len()
    );
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
    let outside: Vec<&String> = several
        .iter()
        .filter(|shown| !shown.starts_with("docs/semantics/modules/"))
        .collect();
    assert!(
        outside.is_empty(),
        "programs outside the module cases that are not one root instance — several instances, which `System::read` \
         does not read, or none, a module that does not elaborate — so the comparison skips them; `M3.4`'s reading of \
         a program is due: {outside:?}"
    );
}

/// A small deterministic sequence (xorshift64*), so the generated systems are the same on every run.
struct Seeded(u64);

impl Seeded {
    fn next(&mut self) -> u64 {
        self.0 ^= self.0 >> 12;
        self.0 ^= self.0 << 25;
        self.0 ^= self.0 >> 27;
        self.0.wrapping_mul(0x2545_f491_4f6c_dd1d)
    }

    fn below(&mut self, n: usize) -> usize {
        usize::try_from(self.next() % u64::try_from(n).expect("a small bound"))
            .expect("below a usize bound")
    }

    fn pick<'a>(&mut self, of: &[&'a str]) -> &'a str {
        of[self.below(of.len())]
    }
}

/// Every name a generated declaration may write: each declaration's, a dotted one among them, the system's own, and
/// one no declaration bears.
const TARGETS: &[&str] = &["p", "q", "a", "b", "soc.c", "s", "u", "k", "sys", "ghost"];

/// The clauses a generated declaration of `kind` writes, naming `TARGETS`: the positions the kind modules give `uses`
/// and `needs`, and those that are no edge.
fn clauses(kind: &str, rng: &mut Seeded) -> String {
    let edge: &[&str] = match kind {
        "system" => &[
            "(uses {})",
            "(needs {})",
            "(requires (uses {}))",
            "(requires (needs {}))",
        ],
        "task" => &["(uses {})", "(needs {})"],
        "platform" => &[
            "(uses {})",
            "(needs {})",
            "(requires (uses {}))",
            "(offers ({} true))",
            "(refines {})",
        ],
        "block" => &[
            "(uses {})",
            "(needs {})",
            "(requires (needs {}))",
            "(offers ({} true))",
        ],
        _ => &["(uses {})", "(needs {})", "(requires (uses {}))"],
    };
    let mut out = String::new();
    for _ in 0..rng.below(3) {
        let shape = rng.pick(edge);
        out.push(' ');
        out.push_str(&shape.replace("{}", rng.pick(TARGETS)));
    }
    out
}

/// One generated description: two platforms, three blocks, two services, a policy and a system of two tasks, each
/// naming others at random positions; the system's `platform` clause present or not.
fn generated(rng: &mut Seeded) -> String {
    let mut text = String::new();
    for (head, name, kind) in [
        ("defplatform", "p", "platform"),
        ("defplatform", "q", "platform"),
        ("defblock", "a", "block"),
        ("defblock", "b", "block"),
        ("defblock", "soc.c", "block"),
        ("defservice", "s", "service"),
        ("defservice", "u", "service"),
        ("defpolicy", "k", "policy"),
    ] {
        text.push_str(&format!("({head} {name}{})\n", clauses(kind, rng)));
    }
    let platform = match rng.below(3) {
        0 => String::new(),
        1 => format!(" (platform (uses {}))", rng.pick(TARGETS)),
        _ => format!(" (platform (requires (uses {})))", rng.pick(TARGETS)),
    };
    text.push_str(&format!(
        "(defsystem sys{platform}{} (task t1 (deadline 1 ms) (priority 1){}) (task t2 (deadline 1 ms) (priority 2){}))\n",
        clauses("system", rng),
        clauses("task", rng),
        clauses("task", rng)
    ));
    text
}

#[test]
fn the_generator_and_the_checker_read_every_generated_system_alike() {
    let mut rng = Seeded(0x0005_eed0_fa11_5157);
    let mut wrong = Vec::new();
    let cases = 4000;
    for case in 0..cases {
        let text = generated(&mut rng);
        let mut sources = SourceMap::new();
        let id = sources
            .add(format!("generated-{case}.eadl"), text.clone())
            .expect("a small description");
        let (document, diagnostics) = read(&sources, id);
        assert!(
            !diagnostics.has_errors(),
            "case {case} does not read:\n{text}"
        );
        let program = plain(document.forms);
        let before = wrong.len();
        assert_eq!(compare(&format!("case {case}"), &program, &mut wrong), 1);
        if wrong.len() > before {
            wrong.push(format!("  — case {case}:\n{text}"));
        }
    }
    assert!(wrong.is_empty(), "{}", wrong.join("\n"));
}
