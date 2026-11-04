//! The emitter (leaf `M4.2.2`, `docs/decisions/decision_hosted-build.md`): a generated system, from a description to its trace,
//! the plan accepted only by lowering it again, what cannot be built refused, and each copy held to the engine's source.

use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_digest::Digest;
use archogen_emit::{copies, emit, Emitted, Inputs};
use archogen_outcome::record::Producer;
use archogen_plan::{lower_checked, Acceptance, Generator, Refusal};
use archogen_plan_check::judgement::{judge, Inputs as Judged};
use eadl_front::module::MemoryModules;
use eadl_front::{Program, SourceMap};
use eadl_model::check::{check, shipped_registry};
use eadl_model::kind::Registry;
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::adapter::Catalog;
use eadl_resolve::plan;
use eadl_resolve::search::{search, Limits, Policy, Search};
use eadl_resolve::vocabulary::Vocabulary;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn kinds(sources: &mut SourceMap) -> Registry {
    let modules: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|path| {
        (
            (*path).to_owned(),
            fs::read_to_string(root().join(path)).expect("a kind module"),
        )
    })
    .collect();
    shipped_registry(sources, &modules).expect("the kinds load")
}

/// A description's inputs to a build, as `archogen build` makes them: the search's assignment, the checker's result.
struct Build {
    sources: SourceMap,
    kinds: Registry,
    program: Program,
    assignment: String,
    result: String,
    plan: String,
}

fn generator() -> Generator {
    Generator {
        version: "0.1.0".to_owned(),
        sha256: Digest::of(b"the generator"),
    }
}

fn build_of(text: &str) -> Build {
    let mut sources = SourceMap::new();
    let kinds = kinds(&mut sources);
    let id = sources
        .add("description.eadl", text.to_owned())
        .expect("small");
    let outcome = check(&sources, id, &kinds, &RT_STATIC_UP_V1);
    assert!(outcome.is_ok(), "the language refuses: {text}");
    let program = plan::plain(outcome.declarations);
    let vocabulary = Vocabulary::shipped(&mut SourceMap::new(), &kinds).expect("the vocabulary");
    let adapters = Catalog::shipped(&mut SourceMap::new(), &vocabulary).expect("the catalog");
    let declarations = &program.root().declarations;
    let system = declarations
        .iter()
        .find(|f| f.head() == Some("defsystem"))
        .expect("a system");
    let policy = Policy {
        adapters,
        ..Policy::default()
    };
    let Search::Resolved(resolution) = search(
        declarations,
        system,
        RT_STATIC_UP_V1.caller_level,
        &vocabulary,
        &policy,
        Limits::DEFAULT,
    ) else {
        panic!("the search resolves no plan for {text}");
    };
    let assignment = plan::write(
        &resolution,
        RT_STATIC_UP_V1.id,
        &plan::program_digest(&program),
    );
    let record = judge(
        Producer {
            name: "archogen-plan-check".to_owned(),
            sha256: Digest::of(b"the checker"),
        },
        &Judged {
            description_name: "description.eadl",
            description: text,
            modules: &MemoryModules::new(),
            profile: &RT_STATIC_UP_V1,
            plan_name: "plan.eadl",
            plan: assignment.as_bytes(),
        },
    )
    .expect("judged");
    assert!(record.accepts(), "{:?}", record.findings());
    let result = record.write();
    let plan = lower_checked(
        &program,
        &assignment,
        &result,
        &kinds,
        generator(),
        Acceptance::CheckedByFront,
    )
    .map_or_else(|_| String::new(), |lowered| lowered.plan.text().to_owned());
    Build {
        sources,
        kinds,
        program,
        assignment,
        result,
        plan,
    }
}

impl Build {
    fn inputs(&self, acceptance: Acceptance) -> Inputs<'_> {
        Inputs {
            program: &self.program,
            sources: &self.sources,
            description: "description.eadl",
            assignment: &self.assignment,
            result: &self.result,
            plan: &self.plan,
            kinds: &self.kinds,
            acceptance,
        }
    }

    fn emitted(&self) -> Result<Emitted, Refusal> {
        emit(&self.inputs(Acceptance::CheckedByFront), generator())
    }
}

fn fixture(name: &str) -> String {
    let path = root().join(format!("crates/archogen-plan/tests/corpus/{name}.eadl"));
    let text = fs::read_to_string(path).expect("the fixture");
    if text.contains("(eadl-version") {
        text
    } else {
        format!("(eadl-version eadl/1)\n{text}")
    }
}

fn file<'e>(emitted: &'e Emitted, path: &str) -> &'e str {
    &emitted
        .files
        .iter()
        .find(|f| f.path == path)
        .unwrap_or_else(|| panic!("no `{path}`"))
        .contents
}

/// The generated package written under the repository's `target/`, built and run: its stdout.
fn run(emitted: &Emitted, name: &str) -> String {
    let dir = root().join("target/emit-tests").join(name);
    let _ = fs::remove_dir_all(&dir);
    for file in &emitted.files {
        let path = dir.join(&file.path);
        fs::create_dir_all(path.parent().expect("a parent")).expect("a directory");
        fs::write(path, &file.contents).expect("written");
    }
    let output = Command::new(env!("CARGO"))
        .args(["run", "-q", "--offline", "--manifest-path"])
        .arg(dir.join("Cargo.toml"))
        .env("CARGO_TARGET_DIR", dir.join("target"))
        .output()
        .expect("cargo runs");
    assert!(
        output.status.success(),
        "{}",
        String::from_utf8_lossy(&output.stderr)
    );
    String::from_utf8(output.stdout).expect("text")
}

#[test]
fn a_generated_system_builds_runs_and_writes_its_exact_trace() {
    let emitted = build_of(&fixture("worked-heartbeat"))
        .emitted()
        .expect("emitted");
    assert_eq!(emitted.tasks, 2);
    assert_eq!(emitted.ticks_per_second, 100);
    assert_eq!(emitted.horizon, 3);
    assert!(!emitted.experimental);
    let trace = run(&emitted, "heartbeat");
    let events: Vec<&str> = trace.lines().filter(|l| !l.starts_with(';')).collect();
    assert_eq!(
        events[..5],
        [
            "0 s initialised 1 console.uart",
            "0 s initialised 2 console.write",
            "0 s enabled",
            "0 s timer-programmed 0",
            "0 s pending timer",
        ]
    );
    let releases: Vec<&str> = events
        .iter()
        .copied()
        .filter(|l| l.contains("release"))
        .collect();
    assert_eq!(
        releases,
        [
            "0 s release beat",
            "0 s release chime",
            "1/100 s release beat",
            "1/50 s release beat"
        ]
    );
    // It says what it is, and what it did not do.
    assert!(
        trace.starts_with("; archogen-playground-trace/1\n; system heartbeat\n; plan sha256:"),
        "{trace}"
    );
    assert!(trace.contains("; no job runs application code"), "{trace}");
    assert!(trace.contains("; the plan is not complete"), "{trace}");
    assert!(!trace.contains("; experimental"), "{trace}");
}

#[test]
fn the_trace_the_tables_name_is_the_simulator_s() {
    let emitted = build_of(&fixture("worked-heartbeat"))
        .emitted()
        .expect("emitted");
    assert!(file(&emitted, "src/main.rs").contains(sim_playground::TRACE_FORMAT));
}

#[test]
fn emitting_twice_writes_the_same_bytes() {
    let build = build_of(&fixture("sporadic"));
    assert_eq!(
        build.emitted().expect("emitted"),
        build.emitted().expect("emitted")
    );
}

#[test]
fn a_task_a_source_releases_is_reported_and_runs_at_its_stated_separation() {
    let emitted = build_of(&fixture("sporadic")).emitted().expect("emitted");
    assert!(!emitted.source_triggers.is_empty());
    let trace = run(&emitted, "sporadic");
    let (task, source) = &emitted.source_triggers[0];
    assert!(
        trace.contains(&format!(" pending source {source}\n")),
        "{trace}"
    );
    assert!(trace.contains(&format!(" release {task}\n")), "{trace}");
}

#[test]
fn a_plan_is_accepted_only_by_lowering_it_again() {
    let mut build = build_of(&fixture("worked-heartbeat"));
    build.plan = build.plan.replace("(rank 2)", "(rank 3)");
    let refusal = build.emitted().expect_err("refused");
    assert_eq!(refusal.code, "tool-failure");
    assert!(
        refusal.message.contains("not what its inputs lower to"),
        "{}",
        refusal.message
    );
}

#[test]
fn a_plan_taken_as_text_is_built_marked_experimental() {
    let build = build_of(&fixture("worked-heartbeat"));
    let emitted = emit(&build.inputs(Acceptance::TakenAsText), generator()).expect("emitted");
    assert!(emitted.experimental);
    assert!(file(&emitted, "src/main.rs")
        .contains("experimental: the checker's acceptance was taken as text"));
    assert!(file(&emitted, "provenance.json").contains("\"experimental\": true"));
}

#[test]
fn a_task_the_plan_cannot_say_what_releases_is_refused_with_its_cause_s_code() {
    // Each cause the routes leave, from the corpus's own fixtures (`RTP-H1`): every one `missing-fact`, repaired by the
    // task's own `(needs interrupt-source)`. The task's own causes, which the search refuses first, are the unit test's.
    for (name, cause) in [
        ("trigger-unreached", "unreached"),
        ("trigger-absent-by-handler", "(absent"),
        ("trigger-faster", "(faster"),
        ("trigger-several", "(several"),
        ("source-separation-bare", "(unstated"),
    ] {
        let refusal = build_of(&fixture(name)).emitted().expect_err(name);
        assert_eq!(refusal.code, "missing-fact", "{name}: {}", refusal.message);
        assert!(
            refusal.message.contains("cannot say what releases"),
            "{name}: {}",
            refusal.message
        );
        assert!(
            refusal.message.contains(cause),
            "{name}: {}",
            refusal.message
        );
    }
}

#[test]
fn each_copy_is_the_engine_s_source_byte_for_byte_and_none_is_left_out() {
    // Moved with what it copies, deliberately: the change that moves the runtime moves what every generated system holds.
    let refresh = std::env::var_os("ARCHOGEN_BLESS_COPIES").is_some();
    for (path, contents) in copies() {
        let source =
            fs::read_to_string(root().join("crates").join(path)).expect("the engine's source");
        if refresh {
            fs::write(
                root().join("crates/archogen-emit/copies").join(path),
                &source,
            )
            .expect("the copy");
            continue;
        }
        assert!(
            source == *contents,
            "`crates/archogen-emit/copies/{path}` is not `crates/{path}`: refresh every copy with \
             ARCHOGEN_BLESS_COPIES=1 cargo test -p archogen-emit --test emit, and rerun"
        );
    }
    for krate in ["rt-core", "sim-playground"] {
        for entry in fs::read_dir(root().join("crates").join(krate).join("src")).expect("a crate") {
            let path = format!(
                "{krate}/src/{}",
                entry.expect("an entry").file_name().to_string_lossy()
            );
            assert!(
                copies().iter().any(|(copied, _)| *copied == path),
                "`{path}` is in no generated system"
            );
        }
    }
}

#[test]
fn provenance_says_what_each_table_entry_came_from() {
    let emitted = build_of(&fixture("worked-heartbeat"))
        .emitted()
        .expect("emitted");
    let provenance = file(&emitted, "provenance.json");
    assert!(
        provenance.contains("\"format\": \"archogen-build-provenance/1\""),
        "{provenance}"
    );
    assert!(
        provenance.contains("\"acceptance\": \"checked-by-front\""),
        "{provenance}"
    );
    assert!(
        provenance.contains("\"table\": \"TASKS[0]\""),
        "{provenance}"
    );
    // Checkable from both ends: the generated line is the entry, and the source line is the task.
    let main = file(&emitted, "src/main.rs");
    let line: usize = provenance
        .split("\"table\": \"TASKS[0]\", \"generated\": { \"file\": \"src/main.rs\", \"line\": ")
        .nth(1)
        .and_then(|rest| rest.split(' ').next())
        .and_then(|n| n.parse().ok())
        .expect("a line");
    assert!(main
        .lines()
        .nth(line - 1)
        .expect("the line")
        .contains("name: \"beat\""));
    let text = fixture("worked-heartbeat");
    let source: usize = provenance
        .split("\"source\": { \"file\": \"description.eadl\", \"line\": ")
        .nth(1)
        .and_then(|rest| rest.split(',').next())
        .and_then(|n| n.parse().ok())
        .expect("a source line");
    assert!(text
        .lines()
        .nth(source - 1)
        .expect("the line")
        .contains("(task beat"));
}
