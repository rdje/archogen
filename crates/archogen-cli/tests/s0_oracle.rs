//! The S0 oracle: what the generated `heartbeat` system must be observed to do.
//!
//! `ROADMAP.md` §12 S0 requires this file to exist **before** the thing it judges:
//!
//! > Before generating, write an independent expected-output assertion for this limited
//! > behavior. […] Changing only a comment or embedding a prebuilt output without consuming the
//! > functional input does not satisfy the gate.
//!
//! So the independence claim is structural rather than promised. The emitter does not exist
//! when this lands (leaf `S0.1`; the emitter is `S0.3`), so the frozen observations in
//! `examples/s0-heartbeat/expected/` cannot have been read off it. And this file lives in
//! `tests/`, which Rust cannot link into a library — no future emitter can call the derivation
//! below, however convenient that would be. It must be written from the contract in
//! `examples/s0-heartbeat/README.md`.
//!
//! # What is asserted, and when
//!
//! | Assertion | Assertable |
//! |---|---|
//! | the three fixtures are accepted by `archogen check` | now |
//! | each frozen observation follows from its description | now |
//! | the unsupported fixture has no derivable observation | now |
//! | the changed case observes something materially different | now |
//! | the generated executable actually prints it | leaf `S0.4` |
//!
//! The last row is not silently skipped — [`build_is_not_yet_assertable`] pins the *current*
//! behavior of `archogen build` and fails the moment it becomes real, which is what forces
//! `S0.4` to wire the real comparison instead of quietly inheriting a green test.
//!
//! # Disclosed shared dependency (§4.4)
//!
//! This oracle shares the eADL reader (`eadl-front`) with the toolchain, and nothing else. A
//! reader bug that mis-parses `(period 10 ms)` would mislead both sides identically. That is an
//! accepted, named dependency: building a second parser to avoid it would contradict §4.1
//! ("do not build a second incompatible parser") and would be the more dangerous kind of green.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use archogen_cli::{run, Status};
use eadl_front::{read, Form, SourceMap};

/// The three cases of the corpus, as `(description, expected observation)`.
const CASES: &[(&str, &str)] = &[
    (
        "examples/s0-heartbeat/system.eadl",
        "examples/s0-heartbeat/expected/system.txt",
    ),
    (
        "examples/s0-heartbeat/system-changed.eadl",
        "examples/s0-heartbeat/expected/system-changed.txt",
    ),
    (
        "examples/s0-heartbeat/system-unsupported.eadl",
        "examples/s0-heartbeat/expected/system-unsupported.txt",
    ),
];

// ── the observation contract ─────────────────────────────────────────────────────────────────
//
// The five rules of `examples/s0-heartbeat/README.md`, implemented here independently of any
// generator. Read that file first: this is the second statement of the contract, not the first.

/// One release event.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Release {
    /// Modeled time in whole milliseconds.
    at_ms: i64,
    /// The task's priority rank — ascending is highest-first
    /// (`docs/decisions/decision_priority-comparison-direction.md`).
    priority: i64,
    /// The task's declared name.
    task: String,
}

/// What the contract yields for a description.
#[derive(Debug, PartialEq, Eq)]
enum Derived {
    /// The exact bytes the generated executable must write to standard output.
    Observation(String),
    /// No release schedule follows from the description, so no observation exists.
    Refusal {
        /// The §5.5 result the build must report.
        code: &'static str,
        /// The clause the refusal must name.
        names: String,
    },
}

/// Derive the observation the contract requires for one description.
///
/// # Panics
///
/// On a description this contract does not cover at all — no system, an unnamed task, a period
/// that is not a whole number of milliseconds. These are *not* refusals: a refusal is a defined
/// answer about a described system, and panicking keeps a malformed fixture from being read as
/// one. The S0 restriction to whole milliseconds is recorded for retirement in the corpus
/// README.
fn derive(forms: &[Form]) -> Derived {
    let system = forms
        .iter()
        .find(|form| form.head() == Some("defsystem"))
        .expect("the S0 corpus declares exactly one system");
    let system_name = declared_name(system);

    let mut releases: Vec<Release> = Vec::new();
    let mut periods: Vec<i64> = Vec::new();
    let mut tasks: Vec<(String, i64, i64)> = Vec::new();

    for task in system.items().iter().filter(|c| c.head() == Some("task")) {
        let name = declared_name(task);
        let priority = match clause_value(task, "priority") {
            Some(Form::Integer { value, .. }) => *value,
            _ => panic!("task `{name}` has no integer priority"),
        };
        // Rule 5: a minimum separation says how close two releases may be, never when one
        // happens. There is no schedule to derive, so the contract refuses rather than
        // inventing an arrival pattern the description did not state.
        if clause_value(task, "period").is_none() && clause_value(task, "min-separation").is_some()
        {
            return Derived::Refusal {
                code: "unsupported-profile",
                names: "min-separation".to_string(),
            };
        }
        let period = whole_milliseconds(task, "period");
        periods.push(period);
        tasks.push((name, priority, period));
    }
    assert!(!tasks.is_empty(), "the system declares no tasks");

    // Rule 1: the horizon is the hyperperiod — a function of the description, not a knob.
    let horizon = periods
        .iter()
        .copied()
        .reduce(lcm)
        .expect("at least one task");

    // Rule 2: a task of period T is released at every t = k·T strictly inside the horizon.
    for (name, priority, period) in &tasks {
        let mut at = 0;
        while at < horizon {
            releases.push(Release {
                at_ms: at,
                priority: *priority,
                task: name.clone(),
            });
            at += period;
        }
    }

    // Rule 3: ascending time, then ascending priority rank. §3.1 requires unique priorities, so
    // this is a total order and no further tie-break can ever be reached.
    releases.sort_by_key(|release| (release.at_ms, release.priority));

    // Rule 4.
    let mut out = format!("system {system_name}\n");
    for release in &releases {
        out.push_str(&format!("release {} ms {}\n", release.at_ms, release.task));
    }
    out.push_str(&format!(
        "summary hyperperiod {horizon} ms releases {}\n",
        releases.len()
    ));
    Derived::Observation(out)
}

/// The declared name of a `(defsystem NAME …)` or `(task NAME …)` form.
fn declared_name(form: &Form) -> String {
    form.items()
        .get(1)
        .and_then(Form::as_symbol)
        .unwrap_or_else(|| panic!("`{}` has no name", form.head().unwrap_or("<form>")))
        .to_string()
}

/// The first value of a named clause, e.g. `10` in `(period 10 ms)`.
fn clause_value<'a>(form: &'a Form, clause: &str) -> Option<&'a Form> {
    form.items()
        .iter()
        .find(|item| item.head() == Some(clause))
        .and_then(|item| item.items().get(1))
}

/// Read a duration clause as a whole number of milliseconds.
fn whole_milliseconds(task: &Form, clause: &str) -> i64 {
    let name = declared_name(task);
    let item = task
        .items()
        .iter()
        .find(|item| item.head() == Some(clause))
        .unwrap_or_else(|| panic!("task `{name}` has no `{clause}`"));
    let value = match item.items().get(1) {
        Some(Form::Integer { value, .. }) => *value,
        other => panic!("task `{name}`: `{clause}` is {other:?}, not a whole number"),
    };
    let unit = item.items().get(2).and_then(Form::as_symbol);
    assert_eq!(
        unit,
        Some("ms"),
        "task `{name}`: the S0 realization reads whole milliseconds only, not `{unit:?}`"
    );
    assert!(value > 0, "task `{name}`: `{clause}` must be positive");
    value
}

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    a / gcd(a, b) * b
}

// ── the frozen expectations ──────────────────────────────────────────────────────────────────

/// One frozen expected-observation file: its `; key: value` headers and its literal body.
struct Expectation {
    headers: BTreeMap<String, String>,
    /// The exact bytes the generated executable must write to standard output. Empty for a
    /// case whose expected outcome is a refusal.
    body: String,
}

impl Expectation {
    fn header(&self, key: &str) -> &str {
        self.headers
            .get(key)
            .unwrap_or_else(|| panic!("the expectation has no `{key}:` header"))
    }

    fn exit(&self, key: &str) -> i32 {
        self.header(key)
            .parse()
            .unwrap_or_else(|e| panic!("`{key}:` is not an exit code: {e}"))
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

fn read_repo_file(relative: &str) -> String {
    let path = repo_root().join(relative);
    std::fs::read_to_string(&path).unwrap_or_else(|e| panic!("cannot read {}: {e}", path.display()))
}

fn parse_description(relative: &str) -> Vec<Form> {
    let text = read_repo_file(relative);
    let mut sources = SourceMap::new();
    let id = sources.add(relative, text).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "{relative} did not read:\n{}",
        diagnostics.render(&sources)
    );
    document.forms
}

/// Load one frozen expectation.
///
/// The header block is the leading run of `;` lines and is parsed by the shipped reader's
/// `comment_headers`, so the wrapped-value rule the boundary corpus already relies on applies
/// here unchanged — one format, one implementation, one place to get it wrong.
fn load_expectation(relative: &str) -> Expectation {
    let text = read_repo_file(relative);
    let mut header_text = String::new();
    let mut body = String::new();
    let mut in_header = true;
    for line in text.lines() {
        if in_header && line.starts_with(';') {
            header_text.push_str(line);
            header_text.push('\n');
        } else {
            in_header = false;
            body.push_str(line);
            body.push('\n');
        }
    }
    let mut sources = SourceMap::new();
    let id = sources.add(relative, header_text).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(
        !diagnostics.has_errors(),
        "{relative}: the header block did not read:\n{}",
        diagnostics.render(&sources)
    );
    assert!(
        document.forms.is_empty(),
        "{relative}: the header block must be comments only"
    );
    Expectation {
        headers: document.comment_headers().into_iter().collect(),
        body,
    }
}

/// Run one `archogen` invocation against a repo-root-relative description.
fn invoke(args: &[&str]) -> (Status, String) {
    let mut out: Vec<u8> = Vec::new();
    let mut err: Vec<u8> = Vec::new();
    let status = run(args.iter().map(|s| (*s).to_string()), &mut out, &mut err);
    let mut text = String::from_utf8(out).expect("utf-8 stdout");
    text.push_str(&String::from_utf8(err).expect("utf-8 stderr"));
    (status, text)
}

fn absolute(relative: &str) -> String {
    repo_root().join(relative).display().to_string()
}

// ── the assertions ───────────────────────────────────────────────────────────────────────────

#[test]
fn every_expected_file_declares_the_whole_schema() {
    // A frozen expectation that forgets to say which command it constrains, or with what exit
    // code, is not an expectation — it is a comment that happens to sit next to some text.
    for (_, expected) in CASES {
        let expectation = load_expectation(expected);
        for key in [
            "case",
            "description",
            "check-exit",
            "build-exit",
            "observation",
            "rationale",
            "written",
        ] {
            assert!(
                expectation.headers.contains_key(key),
                "{expected} has no `{key}:` header"
            );
        }
        let observation = expectation.header("observation");
        assert!(
            matches!(observation, "stdout" | "refusal"),
            "{expected}: `observation: {observation}` is neither `stdout` nor `refusal`"
        );
        assert_eq!(
            expectation.body.trim().is_empty(),
            observation == "refusal",
            "{expected}: a refusal has no standard output, and an observation is not empty"
        );
        if observation == "refusal" {
            for key in ["diagnostic-code", "names", "missing-capability", "owner"] {
                assert!(
                    expectation.headers.contains_key(key),
                    "{expected}: a refusal must record `{key}:` — §5.4 requires the missing \
                     engine capability to be named, not the request declared impossible"
                );
            }
        }
    }
}

#[test]
fn every_expectation_points_at_its_own_description() {
    for (description, expected) in CASES {
        let expectation = load_expectation(expected);
        assert_eq!(
            expectation.header("description"),
            *description,
            "{expected} names a different description"
        );
    }
}

#[test]
fn every_fixture_is_accepted_by_archogen_check() {
    // ⭐ Including the unsupported one. Its refusal belongs to generation, not to the frontend:
    // §3.1 admits sporadic releases, so a description that declares one is valid eADL inside the
    // profile. A corpus where `check` rejected it would be testing M1, not S0.
    for (description, expected) in CASES {
        let expectation = load_expectation(expected);
        let (status, output) = invoke(&["check", &absolute(description)]);
        assert_eq!(
            status.code(),
            expectation.exit("check-exit"),
            "{description}: `archogen check` disagreed with `check-exit:`\n{output}"
        );
    }
}

#[test]
fn each_frozen_observation_is_what_its_description_says() {
    // The load-bearing assertion. Each expected file is re-derived from its description through
    // an implementation of the contract that no generator wrote, and compared byte for byte.
    for (description, expected) in CASES {
        let expectation = load_expectation(expected);
        if expectation.header("observation") != "stdout" {
            continue;
        }
        let derived = derive(&parse_description(description));
        let Derived::Observation(text) = derived else {
            panic!("{description}: the contract refuses a case frozen as an observation");
        };
        assert_eq!(
            text, expectation.body,
            "{expected} is not what {description} says"
        );
    }
}

#[test]
fn the_unsupported_case_has_no_derivable_observation() {
    let (description, expected) = CASES[2];
    let expectation = load_expectation(expected);
    let Derived::Refusal { code, names } = derive(&parse_description(description)) else {
        panic!("{description}: a sporadic release must not yield an observation");
    };
    assert_eq!(code, expectation.header("diagnostic-code"));
    assert_eq!(names, expectation.header("names"));
    assert_eq!(
        Status::UnsupportedProfile.code(),
        expectation.exit("build-exit"),
        "the frozen exit code must be the one the status vocabulary assigns"
    );
}

#[test]
fn the_changed_case_observes_something_materially_different() {
    // §12 S0: "Changing only a comment or embedding a prebuilt output without consuming the
    // functional input does not satisfy the gate." Asserting merely that the two differ would
    // pass on a changed whitespace, so the *shape* of the difference is pinned: a shorter
    // horizon, fewer releases, and one event gone.
    let base = load_expectation(CASES[0].1).body;
    let changed = load_expectation(CASES[1].1).body;
    assert_ne!(base, changed);
    assert!(
        base.contains("summary hyperperiod 30 ms releases 4"),
        "{base}"
    );
    assert!(
        changed.contains("summary hyperperiod 20 ms releases 3"),
        "{changed}"
    );
    assert!(
        base.contains("release 20 ms beat") && !changed.contains("release 20 ms beat"),
        "the shorter horizon must drop the 20 ms release"
    );
    assert_eq!(
        base.lines().count(),
        changed.lines().count() + 1,
        "one release is lost, so one line is lost"
    );
}

#[test]
fn the_derivation_reads_the_description_rather_than_a_constant() {
    // ⛔ The RED ARM (`TOOLBOX.md`): a check only ever seen green has not been shown to check
    // anything. Mutate one period in memory and the derived observation must follow — a new
    // horizon is not reached here, but `beat` moves from 10 ms to 15 ms and one release is lost.
    let text = read_repo_file(CASES[0].0).replace("(period 10 ms)", "(period 15 ms)");
    let mut sources = SourceMap::new();
    let id = sources.add("mutated", text).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(!diagnostics.has_errors());
    let Derived::Observation(mutated) = derive(&document.forms) else {
        panic!("the mutation is still a periodic system");
    };
    assert_eq!(
        mutated,
        "system heartbeat\n\
         release 0 ms beat\n\
         release 0 ms chime\n\
         release 15 ms beat\n\
         summary hyperperiod 30 ms releases 3\n"
    );
    assert_ne!(mutated, load_expectation(CASES[0].1).body);
}

#[test]
fn the_priority_rank_decides_the_order_of_coincident_releases() {
    // The second red arm, for rule 3. Swapping the two priorities must swap the two events at
    // t = 0 and change nothing else — which is only true if a lower rank is a higher priority
    // (`docs/decisions/decision_priority-comparison-direction.md`).
    let text = read_repo_file(CASES[0].0)
        .replace("(priority 1)", "(priority 9)")
        .replace("(priority 2)", "(priority 1)");
    let mut sources = SourceMap::new();
    let id = sources.add("swapped", text).expect("small");
    let (document, diagnostics) = read(&sources, id);
    assert!(!diagnostics.has_errors());
    let Derived::Observation(swapped) = derive(&document.forms) else {
        panic!("swapping priorities does not change the release model");
    };
    let lines: Vec<&str> = swapped.lines().collect();
    assert_eq!(lines[1], "release 0 ms chime");
    assert_eq!(lines[2], "release 0 ms beat");
    assert_eq!(
        lines.last().copied(),
        Some("summary hyperperiod 30 ms releases 4"),
        "only the order of the coincident pair may change"
    );
}

#[test]
fn build_is_not_yet_assertable() {
    // ⛔ TRIPWIRE, not a skip. §14.3: "a required tool skipped or unavailable is reported as
    // such, not a passed check." The frozen `build-exit:` codes cannot be asserted until leaf
    // `S0.3` builds the emitter, so this pins what `archogen build` does *today* and fails the
    // moment that changes — which is what forces `S0.4` to replace it with the real end-to-end
    // comparison instead of inheriting a green test that checks nothing.
    for (description, expected) in CASES {
        let (status, output) = invoke(&["build", &absolute(description), "--out", "build/s0"]);
        assert_eq!(
            status,
            Status::Unimplemented,
            "`archogen build` is real now — leaf S0.4 must replace this tripwire with the \
             end-to-end comparison against {expected}\n{output}"
        );
        let _ = description;
    }
}
