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
    assert_eq!(
        FORMAT,
        assignment::FORMAT,
        "the two constants name one format"
    );
    let mut bindings = std::collections::BTreeMap::new();
    for (who, fact, provider) in [
        (
            Who::Named("clock.time.monotonic".to_owned()),
            "counter-width",
            "platform.timer.timer.counter",
        ),
        (
            Who::Named("clock.time.monotonic".to_owned()),
            "unambiguous-horizon",
            "platform.timer.timer.counter",
        ),
        (
            Who::Task {
                system: "app.rt".to_owned(),
                task: "sensor".to_owned(),
            },
            "time.monotonic",
            "platform.timer.timer.counter",
        ),
    ] {
        bindings.insert((who, fact.to_owned()), provider.to_owned());
    }
    let resolution = Resolution {
        system: "app.rt".to_owned(),
        placement: Some(("platform.soc.bus".to_owned(), "soc.ram".to_owned())),
        bindings,
        obligations: Vec::new(),
        steps: 1,
    };
    let golden = std::fs::read_to_string(
        root().join("crates/archogen-plan-check/tests/goldens/archogen-assignment-1.golden"),
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
