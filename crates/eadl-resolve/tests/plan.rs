//! The plan the search writes (leaf `M3.4.3`): `archogen-assignment/1` by the generator's own code, held to the
//! format's golden byte for byte, its program text and plain program held to the configuration checker's over every
//! tracked description, and a resolved example's plan read back by the checker's reader and accepted by its checks;
//! the generator's own reader of the record, and the bindings it rebuilds (leaf `M4.1.3`).
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
    // The sealed set is excluded by the listing itself, never by a filter after it (`docs/evaluation/README.md`).
    let output = Command::new("git")
        .args([
            "ls-files",
            "-z",
            "--",
            "*.eadl",
            ":!docs/evaluation",
            ":!docs/feedback",
        ])
        .current_dir(root())
        .output()
        .expect("git runs");
    assert!(output.status.success());
    String::from_utf8(output.stdout)
        .expect("UTF-8 paths")
        .split('\0')
        .filter(|path| !path.is_empty())
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
    let for_task = instance("deadline-over-delay", Some("(task app.rt sensor)"));
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
            // Leaf `M4.1.3`: the generator's own reader reads the record back to the same bytes, and the bindings it
            // rebuilds over the description give the joint rules the search's own obligations.
            let record =
                plan::read(&text_once).unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            assert_eq!(record.text(), text_once, "{}", path.display());
            let rebuilt = plan::rebuild(
                &record,
                declarations,
                RT_STATIC_UP_V1.caller_level,
                &v,
                &policy.adapters,
            )
            .unwrap_or_else(|e| panic!("{}: {e}", path.display()));
            let report = rebuilt.joint.check(&rebuilt.bindings);
            assert!(report.holds(), "{}: {report:?}", path.display());
            assert_eq!(report.obligations, once.obligations, "{}", path.display());
            resolved += 1;
        }
    }
    // A corpus of one outcome compares nothing: plans written, and systems the search refuses beside them.
    assert!(
        resolved > 0 && refused > 0,
        "{resolved} resolved, {refused} refused"
    );
}

/// The heartbeat's record, as the search writes it, with its description's declarations.
fn heartbeat_record() -> (String, Vec<Form>) {
    let path = root().join("examples/s0-heartbeat/system.eadl");
    let program = program_of(&path);
    let declarations = program.root().declarations.clone();
    let system = declarations
        .iter()
        .find(|f| f.head() == Some("defsystem"))
        .expect("a system")
        .clone();
    let policy = Policy {
        adapters: eadl_resolve::adapter::Catalog::shipped(&mut SourceMap::new(), &vocabulary())
            .expect("the shipped catalog reads"),
        ..Policy::default()
    };
    let Search::Resolved(resolution) = search(
        &declarations,
        &system,
        RT_STATIC_UP_V1.caller_level,
        &vocabulary(),
        &policy,
        Limits::DEFAULT,
    ) else {
        panic!("the heartbeat resolves");
    };
    (
        plan::write(
            &resolution,
            RT_STATIC_UP_V1.id,
            &plan::program_digest(&program),
        ),
        declarations,
    )
}

#[test]
fn the_reader_accepts_only_what_its_writer_writes() {
    // Leaf `M4.1.3`: a record read back writes back the same bytes, so any other spelling of it is refused.
    let (text, _) = heartbeat_record();
    assert_eq!(plan::read(&text).expect("it reads").text(), text);
    let refused = |changed: String, about: &str| {
        let why = plan::read(&changed).expect_err(&changed);
        assert!(why.0.contains(about), "{changed}: {why}");
    };
    refused(
        text.replace(FORMAT, "archogen-assignment/1"),
        "names the format",
    );
    refused(text.replace("(bind ", "(bind  "), "canonical form");
    refused(format!("; a comment\n{text}"), "no comment");
    refused(
        text.replace("(adapters adapters/1)\n", ""),
        "`(adapters …)`",
    );
    refused(
        format!("{text}(bind console.write observable-output console.uart)\n"),
        "sorted",
    );
    refused(text.replace("(program \"", "(program \"A"), "hexadecimal");
    refused(
        format!("{text}(placement host.playground ram)\n"),
        "where only",
    );
}

#[test]
fn a_record_rebuilds_only_over_what_its_description_requires() {
    // Leaf `M4.1.3`: every binding is matched to a requirement its requester writes, and none is left unbound.
    let (text, declarations) = heartbeat_record();
    let v = vocabulary();
    let catalog = eadl_resolve::adapter::Catalog::shipped(&mut SourceMap::new(), &v)
        .expect("the shipped catalog reads");
    let rebuilt = |record: &str| {
        plan::rebuild(
            &plan::read(record).expect("it reads"),
            &declarations,
            RT_STATIC_UP_V1.caller_level,
            &v,
            &catalog,
        )
    };
    let whole = rebuilt(&text).expect("the search's record rebuilds");
    assert_eq!(whole.bindings.len(), 1);
    assert!(whole.joint.check(&whole.bindings).holds());
    let refused = |changed: String, about: &str| {
        let why = rebuilt(&changed).expect_err(&changed);
        assert!(why.0.contains(about), "{changed}: {why}");
    };
    refused(
        text.replace("(bind console.write observable-output console.uart)\n", ""),
        "unbound",
    );
    refused(
        text.replace("console.write observable-output", "console.write tick-rate"),
        "does not require",
    );
    refused(
        text.replace("console.uart)", "no.such.device)"),
        "is no provider",
    );
    refused(
        text.replace("(system heartbeat)", "(system nowhere)"),
        "no system",
    );
}

/// `text`, a plain description, its one system searched: the record written, the checker's verdict on it, and whether the
/// generator's own reader rebuilds it and its joint rules hold.
fn resolved_and_judged(text: &str) -> (String, bool, bool) {
    let mut sources = SourceMap::new();
    let id = sources
        .add("description.eadl", text.to_owned())
        .expect("small");
    let modules = DirectoryModules::new(root());
    let (program, _) = plan_file(
        &mut sources,
        id,
        &modules,
        &RT_STATIC_UP_V1,
        "plan.json",
        "",
    );
    let declarations = program.root().declarations.clone();
    let system = declarations
        .iter()
        .find(|f| f.head() == Some("defsystem"))
        .expect("a system")
        .clone();
    let v = vocabulary();
    let catalog = eadl_resolve::adapter::Catalog::shipped(&mut SourceMap::new(), &v)
        .expect("the shipped catalog reads");
    let policy = Policy {
        adapters: catalog.clone(),
        ..Policy::default()
    };
    let Search::Resolved(resolution) = search(
        &declarations,
        &system,
        RT_STATIC_UP_V1.caller_level,
        &v,
        &policy,
        Limits::DEFAULT,
    ) else {
        panic!("it resolves");
    };
    let record = plan::write(
        &resolution,
        RT_STATIC_UP_V1.id,
        &plan::program_digest(&program),
    );
    let mut sources = SourceMap::new();
    let id = sources
        .add("description.eadl", text.to_owned())
        .expect("small");
    let (_, verdict) = plan_file(
        &mut sources,
        id,
        &modules,
        &RT_STATIC_UP_V1,
        "plan.json",
        &record,
    );
    let rebuilt = plan::rebuild(
        &plan::read(&record).expect("it reads"),
        &declarations,
        RT_STATIC_UP_V1.caller_level,
        &v,
        &catalog,
    )
    .is_ok_and(|r| r.joint.check(&r.bindings).holds());
    (record, verdict.is_ok(), rebuilt)
}

#[test]
fn a_qualifier_binds_nothing_where_its_requester_relies_on_a_function() {
    // Leaf `M4.1.8`: model §8 rule 13's qualifier "makes no binding of its own" where its requester relies on a
    // function, `evt`'s `interrupt` here; the joint rules judge it at the provider bound for one, so `dev.b`, which
    // meets no `sleep`, is passed over. The search once wrote the qualifier's own binding, which the checker refuses.
    let header = "(eadl-version eadl/1)
(defblock dev.b (offers (interrupt-source true) (available-in-state run) (reachable-at-privilege machine)))
(defblock dev.a (offers (interrupt-source true) (available-in-state run sleep) (reachable-at-privilege machine)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses dev.a dev.b)))
";
    let relied = format!(
        "{header}(defservice evt (requires (needs interrupt-source) (available-in-state run sleep)))
(defsystem s
  (task t (min-separation 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses evt))
  (platform (uses soc)))
"
    );
    let (record, accepted, rebuilt) = resolved_and_judged(&relied);
    assert!(!record.contains("available-in-state"), "{record}");
    assert!(
        record.contains("(bind evt interrupt-source dev.a)\n"),
        "{record}"
    );
    assert!(accepted, "{record}");
    assert!(rebuilt, "{record}");
    // A requester relying on no function binds its qualifier as any requirement.
    let alone = format!(
        "{header}(defservice power (requires (available-in-state run sleep)))
(defsystem s
  (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses power))
  (platform (uses soc)))
"
    );
    let (record, accepted, rebuilt) = resolved_and_judged(&alone);
    assert!(
        record.contains("(bind power available-in-state dev.a)\n"),
        "{record}"
    );
    assert!(accepted, "{record}");
    assert!(rebuilt, "{record}");
}

#[test]
fn a_level_a_requester_states_binds_no_provider() {
    // Leaf `M4.1.9`: `reachable-at-privilege` is judged against the level the profile gives the system (model §8 rule
    // 9), never at a provider of its own, as `docs/semantics/boundary/accept/access-authority.eadl` writes it. The
    // search once wrote its binding, which the checker refuses.
    let text = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defplatform host.p (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses console.uart)))
(defservice console.write (requires (needs observable-output) (reachable-at-privilege machine)))
(defsystem sh
  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (platform (uses host.p)))
";
    let (record, accepted, rebuilt) = resolved_and_judged(text);
    assert!(!record.contains("reachable-at-privilege"), "{record}");
    assert!(
        record.contains("(bind console.write observable-output console.uart)\n"),
        "{record}"
    );
    assert!(accepted, "{record}");
    assert!(rebuilt, "{record}");
}

/// `text`, a plain description, and `record`, a hand-written record whose `"PROGRAM"` stands for its program's digest:
/// whether the configuration checker accepts the record, and the generator's rebuild of it.
fn judged_record(text: &str, record: &str) -> (bool, Result<(), String>) {
    let mut sources = SourceMap::new();
    let id = sources
        .add("description.eadl", text.to_owned())
        .expect("small");
    let modules = DirectoryModules::new(root());
    let (program, _) = plan_file(
        &mut sources,
        id,
        &modules,
        &RT_STATIC_UP_V1,
        "plan.json",
        "",
    );
    let record = record.replace("PROGRAM", &plan::program_digest(&program));
    let mut sources = SourceMap::new();
    let id = sources
        .add("description.eadl", text.to_owned())
        .expect("small");
    let (_, verdict) = plan_file(
        &mut sources,
        id,
        &modules,
        &RT_STATIC_UP_V1,
        "plan.json",
        &record,
    );
    let v = vocabulary();
    let catalog = eadl_resolve::adapter::Catalog::shipped(&mut SourceMap::new(), &v)
        .expect("the shipped catalog reads");
    let rebuilt = plan::rebuild(
        &plan::read(&record).expect("it reads"),
        &program.root().declarations,
        RT_STATIC_UP_V1.caller_level,
        &v,
        &catalog,
    )
    .map(|_| ())
    .map_err(|e| e.0);
    (verdict.is_ok(), rebuilt)
}

#[test]
fn a_record_rebuilds_only_the_instances_its_system_s_requesters_are_bound_to() {
    // Leaf `M4.1.10`: an instance is a requester because it is bound, and an exclusive one serves the one requester bound
    // to it (`docs/specs/adapters/decision_engine-adapters.md` §4 and §5). The checker refused both records below, and
    // the rebuild once accepted them, the lowering then writing obligations no requester owed.
    let orphan = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defblock timer.short
  (offers (relative-delay true) (counter-modulus 65536) (tick-rate 10 MHz) (wrap-behavior modular)
    (observation-coherent true) (channels 1 tick) (reachable-at-privilege machine)))
(defplatform host.p
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart timer.short)))
(defservice console.write (requires (needs observable-output)))
(defsystem orph
  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses console.write))
  (platform (uses host.p)))
";
    let header = |system: &str| {
        format!(
            "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system {system})\n(program \"PROGRAM\")\n(adapters adapters/1)\n"
        )
    };
    let extender = "(adapter epoch-extender timer.short)";
    let mut record = header("orph") + "(bind console.write observable-output console.uart)\n";
    for fact in [
        "counter-modulus",
        "observation-coherent",
        "relative-delay",
        "tick-rate",
        "wrap-behavior",
    ] {
        record.push_str(&format!("(bind {extender} {fact} timer.short)\n"));
    }
    let (accepted, rebuilt) = judged_record(orphan, &record);
    assert!(!accepted, "{record}");
    let why = rebuilt.expect_err(&record);
    assert!(
        why.contains(
            "`(adapter epoch-extender timer.short)`, which no requester `orph` serves is bound to"
        ),
        "{why}"
    );

    let misnamed = "(eadl-version eadl/1)
(defblock timer.delay
  (offers (relative-delay true) (counter-modulus 4294967296) (tick-rate 1 MHz) (observation-coherent true)
    (channels 1 tick) (reachable-at-privilege machine)))
(defplatform soc.d (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.delay)))
(defservice dl (requires (needs absolute-deadline)))
(defservice ghost (requires (absolute-deadline (delivery-bound (at-most 7 us)))))
(defsystem mi
  (task x (period 25 ms) (deadline 25 ms) (deadline-from release) (priority 1) (uses dl))
  (platform (uses soc.d)))
";
    let ghost = "(adapter deadline-over-delay timer.delay ghost)";
    let mut record = header("mi") + &format!("(bind dl absolute-deadline {ghost})\n");
    for fact in [
        "counter-modulus",
        "observation-coherent",
        "relative-delay",
        "tick-rate",
    ] {
        record.push_str(&format!("(bind {ghost} {fact} timer.delay)\n"));
    }
    let (accepted, rebuilt) = judged_record(misnamed, &record);
    assert!(!accepted, "{record}");
    let why = rebuilt.expect_err(&record);
    assert!(
        why.contains("an exclusive instance, which serves `ghost` alone"),
        "{why}"
    );
    // Named for the requester it serves, the same record rebuilds, and the checker accepts it.
    let own = record.replace(" ghost)", " dl)");
    let (accepted, rebuilt) = judged_record(misnamed, &own);
    assert!(accepted, "{own}");
    assert_eq!(rebuilt, Ok(()), "{own}");
}

#[test]
fn a_record_rebuilds_only_where_each_provider_meets_its_requirements() {
    // Leaf `M4.1.12`: the search binds a provider only where the relation finds it meeting the pair, and the checker
    // refuses one that does not (`plan-unsatisfied`). The rebuild once accepted the swapped record below, and the
    // lowering wrote a source at the console.
    let description = "(eadl-version eadl/1)
(defblock console.uart (offers (observable-output true) (reachable-at-privilege machine)))
(defblock dev.e (offers (interrupt-source true) (min-arrival-separation 20 ms) (reachable-at-privilege machine)))
(defplatform host.p
  (offers (core-count 1 tick) (reachable-at-privilege machine))
  (requires (uses console.uart dev.e)))
(defservice console.write (requires (needs observable-output)))
(defservice evt (requires (needs interrupt-source)))
(defsystem rel
  (task s (min-separation 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses evt console.write))
  (platform (uses host.p)))
";
    let header = "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system rel)\n(program \"PROGRAM\")\n(adapters adapters/1)\n";
    let swapped = format!(
        "{header}(bind console.write observable-output dev.e)\n(bind evt interrupt-source console.uart)\n"
    );
    let (accepted, rebuilt) = judged_record(description, &swapped);
    assert!(!accepted, "{swapped}");
    let why = rebuilt.expect_err(&swapped);
    assert!(why.contains("to `dev.e`, which does not meet it"), "{why}");
    let right = format!(
        "{header}(bind console.write observable-output console.uart)\n(bind evt interrupt-source dev.e)\n"
    );
    let (accepted, rebuilt) = judged_record(description, &right);
    assert!(accepted, "{right}");
    assert_eq!(rebuilt, Ok(()), "{right}");
}

#[test]
fn a_block_s_own_requirements_are_bound_and_rebuilt() {
    // Leaf `M4.1.14`: a block is a requester for what it writes at its own level (model §8 rule 1), its `uses` being
    // composition. The search once took no requirement of a block's, and the checker refused its record.
    let text = "(eadl-version eadl/1)
(defblock timer.d (offers (relative-delay true) (channels 2 tick) (reachable-at-privilege machine)))
(defblock dma.x
  (offers (observable-output true) (reachable-at-privilege machine))
  (requires (needs relative-delay)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.d dma.x)))
(defservice out (requires (needs observable-output)))
(defsystem bl
  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses out))
  (platform (uses soc)))
";
    let (record, accepted, rebuilt) = resolved_and_judged(text);
    assert!(
        record.contains("(bind dma.x relative-delay timer.d)\n"),
        "{record}"
    );
    assert!(accepted, "{record}");
    assert!(rebuilt, "{record}");
    // A record leaving it unbound no longer rebuilds.
    let header = "(format \"archogen-assignment/2\")\n(profile rt-static-up-v1)\n(system bl)\n(program \"PROGRAM\")\n(adapters adapters/1)\n";
    let (accepted, rebuilt) = judged_record(
        text,
        &format!("{header}(bind out observable-output dma.x)\n"),
    );
    assert!(!accepted);
    let why = rebuilt.expect_err("unbound");
    assert!(why.contains("`dma.x`'s `relative-delay` unbound"), "{why}");
}

#[test]
fn a_block_bound_for_requirements_of_its_own_is_a_node_of_rule_12_s_graph() {
    // Leaf `M4.1.15`: a block the system serves that is bound for requirements of its own is a requester whose code an
    // interrupt can run — here its deadline instance's expiry — so it is a node, a provider's, with the edge from the
    // instance bound for it, though nothing binds to the block itself.
    let text = "(eadl-version eadl/1)
(defblock timer.d
  (offers (relative-delay true) (counter-modulus 65536) (tick-rate 10 MHz) (observation-coherent true)
    (channels 2 tick) (reachable-at-privilege machine)))
(defblock dma.x (offers (reachable-at-privilege machine)) (requires (needs absolute-deadline)))
(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses timer.d dma.x)))
(defsystem be
  (task a (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1))
  (platform (uses soc)))
";
    let (record, accepted, rebuilt) = resolved_and_judged(text);
    assert!(accepted && rebuilt, "{record}");
    let mut sources = SourceMap::new();
    let id = sources
        .add("description.eadl", text.to_owned())
        .expect("small");
    let (program, _) = plan_file(
        &mut sources,
        id,
        &DirectoryModules::new(root()),
        &RT_STATIC_UP_V1,
        "plan.json",
        "",
    );
    let v = vocabulary();
    let catalog = eadl_resolve::adapter::Catalog::shipped(&mut SourceMap::new(), &v)
        .expect("the shipped catalog reads");
    let r = plan::rebuild(
        &plan::read(&record).expect("it reads"),
        &program.root().declarations,
        RT_STATIC_UP_V1.caller_level,
        &v,
        &catalog,
    )
    .expect("it rebuilds");
    let graph = r.joint.initialization(&r.system, &r.bindings);
    assert!(
        graph
            .nodes
            .iter()
            .any(|(n, k)| n == "dma.x" && *k == eadl_resolve::joint::Node::Provider),
        "{:?}",
        graph.nodes
    );
    assert!(
        graph
            .edges
            .iter()
            .any(|(from, to, _)| from.starts_with("(adapter deadline-over-delay") && to == "dma.x"),
        "{:?}",
        graph.edges
    );
}
