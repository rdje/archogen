//! Every outcome `archogen_api::check` gives, each with its status (leaf `API.3.2`).
//!
//! The population is the one `archogen check` answers: a judged description (every §5.5 verdict the semantic
//! corpus reaches, derived from the cases' own headers), a module tree, and the requests the API does not
//! judge — a profile nobody supports, a kind module, an import that exists and cannot be read. ⚠️ A description
//! too large to address (over 4 GiB) is a stated limit here, as it is in `docs/semantics/reference.md` §4:
//! no test builds one.

use std::path::{Path, PathBuf};

use archogen_api::{
    check, MemoryModules, ModuleSource, NoModules, Request, Response, Status, KIND_MODULE_OWNER,
    OPERATIONS, VERSION,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn check_file(relative: &str, profile: Option<&str>) -> Response {
    let text =
        std::fs::read_to_string(repo_root().join(relative)).expect("the fixture is readable");
    check(&Request {
        name: relative,
        text: &text,
        profile,
        modules: &NoModules,
    })
}

/// The invariant every response keeps: judged exactly when its status came from a verdict, and never a note
/// on a judged one.
fn assert_consistent(what: &str, response: &Response) {
    assert_eq!(response.version, VERSION, "{what}");
    match &response.judged {
        Some(judged) => {
            assert_eq!(
                response.status,
                Status::from_verdict(judged.verdict),
                "{what}: a judged response's status is its verdict"
            );
            assert!(response.notes.is_empty(), "{what}: {:?}", response.notes);
        }
        None => {
            assert_ne!(
                response.status,
                Status::Ok,
                "{what}: not judged, and yet ok"
            );
            assert!(
                !response.notes.is_empty(),
                "{what}: not judged, and no note says why"
            );
        }
    }
}

#[test]
fn an_accepted_description_is_judged_ok_and_says_what_it_was_judged_under() {
    let response = check_file("examples/periodic-three/system.eadl", None);
    assert_consistent("periodic-three", &response);
    assert_eq!(
        response.status,
        Status::Ok,
        "{}",
        response.render_diagnostics()
    );
    let judged = response.judged.as_ref().expect("judged");
    assert_eq!(judged.language, "eadl/1");
    assert_eq!(judged.profile, "rt-static-up-v1");
    assert!(!judged.declarations.is_empty());
    assert!(judged.instances.is_none(), "not a module tree");
    assert!(response.diagnostics.is_empty());
}

#[test]
fn every_case_of_the_semantic_corpus_gets_the_verdict_it_declares() {
    let dir = repo_root().join("docs/semantics/cases");
    let mut cases: Vec<PathBuf> = std::fs::read_dir(&dir)
        .expect("the corpus is readable")
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| path.extension().is_some_and(|ext| ext == "eadl"))
        .collect();
    cases.sort();
    assert!(!cases.is_empty(), "no case under docs/semantics/cases/");
    let mut reached = std::collections::BTreeSet::new();
    for path in &cases {
        let relative = path
            .strip_prefix(repo_root())
            .expect("under the root")
            .display()
            .to_string();
        let text = std::fs::read_to_string(path).expect("readable");
        let expect = text
            .lines()
            .find_map(|line| line.strip_prefix("; expect: "))
            .unwrap_or_else(|| panic!("{relative} declares no verdict"))
            .trim()
            .to_string();
        let response = check_file(&relative, None);
        assert_consistent(&relative, &response);
        assert_eq!(
            response.status.slug(),
            expect,
            "{relative}\n{}",
            response.render_diagnostics()
        );
        reached.insert(expect);
    }
    // More than one verdict, or the corpus is not exercising the projection at all.
    assert!(reached.len() > 1, "{reached:?}");
}

#[test]
fn a_profile_nobody_supports_is_not_judged() {
    let response = check_file(
        "examples/periodic-three/system.eadl",
        Some("no-such-profile"),
    );
    assert_consistent("unsupported profile", &response);
    assert_eq!(response.status, Status::UnsupportedProfile);
    assert!(response.judged.is_none());
    assert_eq!(
        response.notes,
        ["`no-such-profile` is not a supported profile"]
    );
    assert!(
        response
            .hint
            .as_deref()
            .is_some_and(|hint| hint.contains("rt-static-up-v1")),
        "{:?}",
        response.hint
    );
}

#[test]
fn a_kind_module_is_not_judged_and_names_the_leaf_that_will_load_one() {
    let response = check_file("docs/semantics/kinds/os-rt.eadl", None);
    assert_consistent("kind module", &response);
    assert_eq!(response.status, Status::Unimplemented);
    assert!(
        response.notes[0]
            .contains("docs/semantics/kinds/os-rt.eadl:27:1 declares a kind, `(defkind task …)`"),
        "{:?}",
        response.notes
    );
    assert!(
        response
            .hint
            .as_deref()
            .is_some_and(|hint| hint.contains(&format!("task-tree leaf {KIND_MODULE_OWNER}"))),
        "{:?}",
        response.hint
    );
    assert!(
        response.diagnostics.is_empty(),
        "not judged, so no verdict's diagnostics"
    );
}

/// A source whose one module exists and cannot be read.
struct Unreadable;

impl ModuleSource for Unreadable {
    fn load(&self, _module: &str) -> Option<(String, String)> {
        None
    }

    fn unreadable(&self) -> Vec<(String, String)> {
        vec![("hw.timer.eadl".to_string(), "permission denied".to_string())]
    }
}

#[test]
fn an_import_that_exists_and_cannot_be_read_fails_the_request_and_is_not_called_missing() {
    let response = check(&Request {
        name: "app.eadl",
        text:
            "(defmodule app (version 1 0) (import hw.timer (as timer) (version (at-least 1 0))))\n",
        profile: None,
        modules: &Unreadable,
    });
    assert_consistent("unreadable import", &response);
    assert_eq!(response.status, Status::Usage);
    assert_eq!(
        response.notes,
        ["cannot read hw.timer.eadl, which app.eadl imports: permission denied"]
    );
    assert!(
        response.diagnostics.is_empty(),
        "module-not-found would be a false statement about the description: {}",
        response.render_diagnostics()
    );
}

#[test]
fn a_module_tree_in_memory_is_elaborated_and_judged() {
    let modules = MemoryModules::new().with("hw.timer", "(defmodule hw.timer (version 1 0))\n");
    let response = check(&Request {
        name: "app.eadl",
        text:
            "(defmodule app (version 1 0) (import hw.timer (as timer) (version (at-least 1 0))))\n",
        profile: None,
        modules: &modules,
    });
    assert_consistent("module tree", &response);
    assert_eq!(
        response.status,
        Status::Ok,
        "{}",
        response.render_diagnostics()
    );
    let instances = response
        .judged
        .as_ref()
        .and_then(|judged| judged.instances.clone())
        .expect("a module tree has instances");
    // Children before parents: elaboration order, the one `module-circular-import` depends on (§6).
    assert_eq!(instances, ["timer = hw.timer 1.0", "(root) = app 1.0"]);
}

#[test]
fn a_description_that_does_not_read_gets_the_read_pass_verdict() {
    let response = check(&Request {
        name: "broken.eadl",
        text: "(defkind widget\n",
        profile: None,
        modules: &NoModules,
    });
    assert_consistent("does not read", &response);
    assert_eq!(response.status, Status::InvalidDescription);
    assert_eq!(response.diagnostics[0].code, "read-unclosed-list");
}

#[test]
fn the_version_and_the_operations_are_declared() {
    assert_eq!(VERSION.to_string(), "1.1");
    assert_eq!(OPERATIONS, ["check"]);
}

/// The version the response's shape below belongs to.
const SHAPE_OF: archogen_api::Version = archogen_api::Version { major: 1, minor: 1 };

#[test]
fn the_response_shape_is_the_one_its_version_declares() {
    // ⛔ Exhaustive on purpose: no `..`. A field added to or removed from `Response` or `Judgement` stops this
    // compiling, here, beside the version the shape belongs to. Adding one is a minor (`VERSION` and `SHAPE_OF`
    // move together); removing one or changing its meaning is a new major (`docs/decisions/decision_engine-api.md`).
    let response = check_file("examples/periodic-three/system.eadl", None);
    let Response {
        version,
        engine: _,
        status: _,
        notes: _,
        hint: _,
        diagnostics: _,
        sources: _,
        judged,
    } = response;
    let archogen_api::Judgement {
        verdict: _,
        language: _,
        profile: _,
        description: _,
        declarations: _,
        instances: _,
        closure: _,
    } = judged.expect("judged");
    assert_eq!(
        version, SHAPE_OF,
        "the response's shape was recorded for {SHAPE_OF}"
    );
}

#[test]
fn the_same_request_answers_the_same_twice_and_names_the_build() {
    // An instance holds no state between requests (`docs/decisions/decision_api-instance.md`), so a response is
    // a function of its request and the build. Compared whole, through `Debug`: status, notes, every diagnostic
    // and span, the sources, and the judgement.
    for relative in [
        "examples/periodic-three/system.eadl",
        "docs/semantics/cases/missing-refinement-target.eadl",
        "docs/semantics/kinds/os-rt.eadl",
    ] {
        let first = check_file(relative, None);
        let second = check_file(relative, None);
        assert_eq!(format!("{first:?}"), format!("{second:?}"), "{relative}");
        assert_eq!(first.engine, archogen_api::ENGINE);
        assert_eq!(first.engine, env!("CARGO_PKG_VERSION"));
    }
}
