//! The plan the search writes (leaf `M3.4.3`): `archogen-assignment/1` by the generator's own code, held to the
//! format's golden byte for byte, its program text and plain program held to the configuration checker's over every
//! tracked description, and a resolved example's plan read back by the checker's reader and accepted by its checks.
//! The checker is a development dependency here alone: the generator compiles none of it.

use std::path::{Path, PathBuf};
use std::process::Command;

use archogen_plan_check::assignment::{self, Assignment};
use archogen_plan_check::check::{self, plan_file};
use eadl_front::module::DirectoryModules;
use eadl_front::{Form, Program, SourceMap};
use eadl_model::check::shipped_registry;
use eadl_model::profile::RT_STATIC_UP_V1;
use eadl_resolve::plan::{self, FORMAT};
use eadl_resolve::search::{search, Limits, Policy, Resolution, Search, Who};
use eadl_resolve::vocabulary::Vocabulary;

fn root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn vocabulary() -> Vocabulary {
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root().join(r)).expect("a kind module"),
        )
    })
    .collect();
    let registry = shipped_registry(&mut SourceMap::new(), &kinds).expect("the kinds load");
    Vocabulary::shipped(&mut SourceMap::new(), &registry).expect("the shipped vocabulary reads")
}

/// The program the description at `path` becomes, as the checker elaborates it.
fn program_of(path: &Path) -> Program {
    let mut sources = SourceMap::new();
    let id = sources
        .add(
            path.display().to_string(),
            std::fs::read_to_string(path).expect("readable"),
        )
        .expect("small");
    let modules = DirectoryModules::new(path.parent().expect("a directory"));
    plan_file(
        &mut sources,
        id,
        &modules,
        &RT_STATIC_UP_V1,
        "plan.json",
        "",
    )
    .0
}

/// Every tracked description, the vendors' reproduction inputs and the sealed set's place apart.
fn tracked() -> Vec<PathBuf> {
    let output = Command::new("git")
        .args(["ls-files", "-z", "*.eadl"])
        .current_dir(root())
        .output()
        .expect("git runs");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .expect("UTF-8 paths")
        .split('\0')
        .filter(|path| {
            !path.is_empty()
                && !path.starts_with("docs/feedback/")
                && !path.starts_with("docs/evaluation/")
        })
        .map(|path| root().join(path))
        .collect()
}

#[test]
fn the_writer_writes_the_format_s_golden() {
    // `/2` (leaf `M3.2.5`, `docs/specs/adapters/decision_engine-adapters.md` §5): the checker's golden, written apart
    // from this writer, binds a shared and two exclusive engine adapter instances — one serving a name, one a task —
    // beside the plain bindings, and this writer writes it byte for byte from the resolution the search would hold.
    assert_eq!(
        FORMAT,
        assignment::FORMAT,
        "the two constants name one format"
    );
    let timer = "platform.timer.timer.counter";
    let v = vocabulary();
    let catalog = eadl_resolve::adapter::Catalog::shipped(&mut SourceMap::new(), &v)
        .expect("the shipped catalog reads");
    let instance = |kind: &str, serves: Option<&str>| {
        let name = eadl_resolve::adapter::instance_name(kind, timer, serves);
        eadl_resolve::adapter::Instance {
            kind: kind.to_owned(),
            over: timer.to_owned(),
            serves: serves.map(str::to_owned),
            provider: eadl_resolve::offer::Provider {
                name,
                span: eadl_front::Span::new(eadl_front::SourceId(0), 0, 0),
                facts: std::collections::BTreeMap::new(),
            },
            beneath: catalog.kind(kind).expect("a kind").beneath_names.clone(),
        }
    };
    let extender = instance("epoch-extender", None);
    let for_name = instance("deadline-over-delay", Some("clock.time.monotonic"));
    let for_task = instance("deadline-over-delay", Some("app.rt/sensor"));
    let named = || Who::Named("clock.time.monotonic".to_owned());
    let task = || Who::Task {
        system: "app.rt".to_owned(),
        task: "sensor".to_owned(),
    };
    let mut bindings = std::collections::BTreeMap::new();
    for (who, fact, provider) in [
        (named(), "absolute-deadline", &for_name),
        (named(), "counter-width", &extender),
        (named(), "unambiguous-horizon", &extender),
        (task(), "absolute-deadline", &for_task),
        (task(), "tick-rate", &extender),
    ] {
        bindings.insert((who, fact.to_owned()), provider.provider.name.clone());
    }
    let resolution = Resolution {
        system: "app.rt".to_owned(),
        placement: Some(("platform.soc.bus".to_owned(), "soc.ram".to_owned())),
        bindings,
        obligations: Vec::new(),
        steps: 1,
        instances: vec![extender.clone(), for_name.clone(), for_task.clone()],
    };
    let golden = std::fs::read_to_string(
        root().join("crates/archogen-plan-check/tests/goldens/archogen-assignment-2.golden"),
    )
    .expect("the golden");
    assert_eq!(
        plan::write(
            &resolution,
            RT_STATIC_UP_V1.id,
            "f116a4715daf8b404d559b210f6f0d09c060158b1c1493fb133bf50197545620"
        ),
        golden
    );
}

#[test]
fn the_program_text_and_the_plain_program_are_the_checker_s_on_every_tracked_description() {
    let mut programs = 0;
    for path in tracked() {
        let program = program_of(&path);
        assert_eq!(
            plan::program_text(&program),
            assignment::program_text(&program),
            "{}",
            path.display()
        );
        // A description that does not elaborate leaves no instance: its text is still compared, empty on both sides.
        if let Some(root) = program.instances.last() {
            let declarations: Vec<Form> = root.declarations.clone();
            assert_eq!(
                plan::program_text(&plan::plain(declarations.clone())),
                assignment::program_text(&check::plain(declarations)),
                "{}",
                path.display()
            );
        }
        programs += 1;
    }
    assert!(programs > 100, "{programs} description(s)");
}

#[test]
fn a_resolved_example_s_plan_is_read_back_and_accepted_by_the_checker() {
    let v = vocabulary();
    let path = root().join("examples/s0-heartbeat/system.eadl");
    let program = program_of(&path);
    let declarations = &program.root().declarations;
    let system = declarations
        .iter()
        .find(|f| f.head() == Some("defsystem"))
        .expect("a system");
    let Search::Resolved(resolution) = search(
        declarations,
        system,
        RT_STATIC_UP_V1.caller_level,
        &v,
        &Policy::default(),
        Limits::DEFAULT,
    ) else {
        panic!("s0-heartbeat resolves");
    };
    let text = plan::write(
        &resolution,
        RT_STATIC_UP_V1.id,
        &plan::program_digest(&program),
    );
    let read = Assignment::read(&text).expect("the checker's reader reads it");
    assert_eq!(read.to_canonical(), text, "one text for what it holds");
    let mut sources = SourceMap::new();
    let id = sources
        .add(
            path.display().to_string(),
            std::fs::read_to_string(&path).expect("readable"),
        )
        .expect("small");
    let modules = DirectoryModules::new(path.parent().expect("a directory"));
    let (_, verdict) = plan_file(
        &mut sources,
        id,
        &modules,
        &RT_STATIC_UP_V1,
        "plan.json",
        &text,
    );
    assert_eq!(verdict, Ok(()), "{text}");
}

/// Whether the language's check accepts the description at `path`, as the engine API checks it: a module file
/// elaborated and its program checked, a plain one checked as read.
fn language_accepts(path: &Path, text: &str) -> bool {
    let kinds: Vec<(String, String)> = [
        "docs/semantics/kinds/core.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ]
    .iter()
    .map(|r| {
        (
            (*r).to_string(),
            std::fs::read_to_string(root().join(r)).expect("a kind module"),
        )
    })
    .collect();
    let mut sources = SourceMap::new();
    let registry = shipped_registry(&mut sources, &kinds).expect("the kinds load");
    let id = sources
        .add(path.display().to_string(), text.to_owned())
        .expect("small");
    let module = eadl_front::read(&sources, id)
        .0
        .forms
        .iter()
        .any(|form| form.head() == Some("defmodule"));
    let outcome = if module {
        let modules = DirectoryModules::new(path.parent().expect("a directory"));
        let (program, diagnostics) = eadl_front::elaborate_source(&mut sources, &modules, id);
        if diagnostics.has_errors() {
            return false;
        }
        eadl_model::check::check_program(&program, &registry, &RT_STATIC_UP_V1)
    } else {
        eadl_model::check::check(&sources, id, &registry, &RT_STATIC_UP_V1)
    };
    outcome.verdict.is_ok()
}

#[test]
fn the_positive_corpus_every_plan_the_search_writes_the_checker_accepts_and_twice_alike() {
    let policy = Policy {
        adapters: eadl_resolve::adapter::Catalog::shipped(&mut SourceMap::new(), &vocabulary())
            .expect("the shipped catalog reads"),
        ..Policy::default()
    };
    // `M3.5`'s filing to `M3.4`: every plan of the positive corpus is accepted by `archogen-plan-check`. The corpus is
    // derived: every tracked description the language's check accepts, as the engine API runs it before `resolve`
    // searches — never the checker's answer to an empty plan, whose check (1) refuses it before it re-checks the
    // description — and each system the search resolves, a module tree's among them.
    let v = vocabulary();
    let (mut resolved, mut refused) = (0, 0);
    for path in tracked() {
        let text = std::fs::read_to_string(&path).expect("readable");
        let judge = |plan: &str| {
            let mut sources = SourceMap::new();
            let id = sources
                .add(path.display().to_string(), text.clone())
                .expect("small");
            let modules = DirectoryModules::new(path.parent().expect("a directory"));
            plan_file(
                &mut sources,
                id,
                &modules,
                &RT_STATIC_UP_V1,
                "plan.json",
                plan,
            )
        };
        if !language_accepts(&path, &text) {
            continue;
        }
        let (program, _) = judge("");
        // A module tree is read as the language checks it, its names qualified (leaf `M3.4.10`).
        let flattened;
        let declarations: &[Form] = if program.instances.len() == 1 {
            &program.root().declarations
        } else {
            flattened = eadl_model::check::resolved_declarations(&program).0;
            &flattened
        };
        for system in declarations
            .iter()
            .filter(|f| f.head() == Some("defsystem"))
        {
            // With the engine's own adapters, as it runs (leaf `M3.2.5`): a plan binding an instance is the checker's to
            // judge like any other.
            let searched = || {
                search(
                    declarations,
                    system,
                    RT_STATIC_UP_V1.caller_level,
                    &v,
                    &policy,
                    Limits::DEFAULT,
                )
            };
            // A refusal is searched once — F11's fixture spends the whole budget — and a plan twice, to hold it alike.
            let Search::Resolved(once) = searched() else {
                refused += 1;
                continue;
            };
            let Search::Resolved(twice) = searched() else {
                panic!("{}: resolved once and not twice", path.display());
            };
            let digest = plan::program_digest(&program);
            let text_once = plan::write(&once, RT_STATIC_UP_V1.id, &digest);
            assert_eq!(
                text_once,
                plan::write(&twice, RT_STATIC_UP_V1.id, &digest),
                "{}",
                path.display()
            );
            let (_, verdict) = judge(&text_once);
            assert_eq!(verdict, Ok(()), "{}: {text_once}", path.display());
            resolved += 1;
        }
    }
    // A corpus of one outcome compares nothing: plans written, and systems the search refuses beside them.
    assert!(
        resolved > 0 && refused > 0,
        "{resolved} resolved, {refused} refused"
    );
}
