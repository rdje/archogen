//! What holds however a description is shaped (leaf `M4.2.3.3.22`; `docs/book/src/engine-api.md`): one process has a ceiling, and it stops every shape that
//! reaches it, under the status of a resource limit, with nothing judged; a cap on what is said moves no verdict; the language's limit on a name is on what a person
//! writes; and what a run says of a name, a path or an argument is one line of its own words. These run the real program, since a ceiling is the program's.

#[allow(dead_code)]
mod common;

use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::sync::atomic::{AtomicUsize, Ordering};

use archogen_cli::Status;
use archogen_transport::ceiling::VARIABLE;
use common::{invoke, Checker};

const HEAD: &str = "(eadl-version eadl/1)\n";
const PLATFORM: &str = "(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses blk)))\n";
const A_TASK: &str = "(task t (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 1) (on-overrun fault))";
const A_TIMER: &str = "(offers (relative-delay true) (counter-width 16 bit) (counter-modulus 65536) (tick-rate 10 MHz) (wrap-behavior modular) (observation-coherent true) (channels 1 tick) (reachable-at-privilege machine))";

fn scratch() -> PathBuf {
    static NEXT: AtomicUsize = AtomicUsize::new(0);
    let dir = Path::new(env!("CARGO_TARGET_TMPDIR")).join(format!(
        "hosted-limits-{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    ));
    let _ = std::fs::remove_dir_all(&dir);
    std::fs::create_dir_all(&dir).expect("a scratch directory");
    dir
}

fn lines(n: usize, each: impl Fn(usize) -> String) -> String {
    (0..n).map(each).collect()
}

fn uses(prefix: &str, n: usize) -> String {
    (0..n).map(|i| format!("(uses {prefix}{i}) ")).collect()
}

fn sizeless(n: usize) -> String {
    let a = lines(n, |i| format!("(region a{i} (base {i}))\n"));
    let b = lines(n, |i| format!("(region b{i} (base {}))\n", 1_000_000 + i));
    format!("{HEAD}(defblock pa (offers (relative-delay true) (channels 1 tick) (reachable-at-privilege machine)\n{a}))\n(defblock pb (offers (observable-output true) (reachable-at-privilege machine)\n{b}))\n(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses pa) (uses pb)))\n(defservice sa (requires (needs relative-delay)))\n(defservice sb (requires (needs observable-output)))\n(defsystem m (task t (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 1) (uses sa) (on-overrun fault)) (requires (uses sa) (uses sb)) (platform (uses soc)))\n")
}

fn through_an_adapter(r: usize, p: usize) -> String {
    let blocks = lines(p, |i| format!("(defblock b{i} {A_TIMER})\n"));
    let each_use = uses("b", p);
    let services = lines(r, |i| {
        format!("(defservice s{i} (requires (unambiguous-horizon (at-least 60 s)) (needs counter-width tick-rate observation-coherent)))\n")
    });
    let asked = uses("s", r);
    format!("{HEAD}{blocks}(defplatform plat (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires {each_use}))\n{services}(defsystem m (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses s0)) (requires {asked}) (platform (uses plat)))\n")
}

fn holes(n: usize) -> String {
    let memory = lines(n, |i| {
        format!("(region r{i} (base 0x1000) (size 4 KiB) (executable true))\n")
    });
    let reserved = lines(n, |i| {
        format!("(region z{i} (base 0x1000) (size 4 KiB) (reserved true))\n")
    });
    format!("{HEAD}(defblock blk (offers (reachable-at-privilege machine)\n{memory}{reserved}))\n{PLATFORM}(defsystem m {A_TASK} (platform (uses soc)))\n")
}

fn unmet(r: usize, p: usize) -> String {
    let blocks = lines(p, |i| {
        format!("(defblock b{i} (offers (bus-width 8 bit) (reachable-at-privilege machine)))\n")
    });
    let each_use = uses("b", p);
    let services = lines(r, |i| {
        format!("(defservice s{i} (requires (needs bus-width) (bus-width 64 bit)))\n")
    });
    let asked = uses("s", r);
    format!("{HEAD}{blocks}(defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires {each_use}))\n{services}(defsystem m (task t (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 1) (uses s0) (on-overrun fault)) (requires {asked}) (platform (uses soc)))\n")
}

/// Blocks named outside the system, and a contradiction inside it.
fn named_outside(n: usize) -> String {
    let regions = "(region a (base 0x1000) (size 4 KiB) (executable true)) (region b (base 0x1000) (size 4 KiB) (executable true))";
    let outside = lines(n, |i| {
        format!("(defblock o{i} (offers (observable-output true)))\n")
    });
    let each_use = uses("o", n);
    format!("{HEAD}(defblock blk (offers (relative-delay true) (channels 1 tick) (reachable-at-privilege machine) {regions}))\n{PLATFORM}{outside}(defservice svc (requires (needs relative-delay) {each_use}))\n(defsystem m (task t (period 100 ms) (deadline 100 ms) (deadline-from release) (priority 1) (uses svc) (on-overrun fault)) (requires (uses svc)) (platform (uses soc)))\n")
}

/// The real program, in `dir`, with `ceiling` in MiB: its exit code and standard error.
fn run_in(dir: &Path, ceiling: &str, args: &[&str]) -> (i32, String) {
    let output = Command::new(env!("CARGO_BIN_EXE_archogen"))
        .args(args)
        .current_dir(dir)
        .env(VARIABLE, ceiling)
        .stdin(Stdio::null())
        .output()
        .expect("the program runs");
    (
        output.status.code().expect("it ended with a code"),
        String::from_utf8_lossy(&output.stderr).into_owned(),
    )
}

/// Each shape a review found to cost by the pair stops at the ceiling, writing no plan, and a small one of the same shape does not reach it.
#[test]
fn every_shape_stops_at_the_ceiling_with_nothing_judged() {
    let inconclusive = Status::AnalysisInconclusive.code();
    for (shape, large, small, small_ends) in [
        (
            "requesters over timers through an adapter",
            through_an_adapter(200, 200),
            through_an_adapter(2, 2),
            Status::Ok,
        ),
        (
            "sizeless regions on two devices",
            sizeless(1000),
            sizeless(2),
            Status::MissingFact,
        ),
        (
            "memory and reserved regions at one base",
            holes(2000),
            holes(2),
            Status::InvalidDescription,
        ),
    ] {
        let dir = scratch();
        let file = dir.join("s.eadl");
        std::fs::write(&file, large).expect("written");
        let (code, said) = run_in(
            &dir,
            "16",
            &["resolve", "s.eadl", "--out", "p", "--unchecked"],
        );
        assert_eq!(
            code,
            inconclusive,
            "{shape}: {}",
            &said[..said.len().min(400)]
        );
        assert!(said.contains("analysis-inconclusive"), "{shape}: {said}");
        assert!(said.contains(VARIABLE), "{shape}: {said}");
        assert!(!dir.join("p").exists(), "{shape}");
        std::fs::write(&file, small).expect("written");
        let (code, said) = run_in(
            &dir,
            "16",
            &["resolve", "s.eadl", "--out", "p", "--unchecked"],
        );
        assert_eq!(code, small_ends.code(), "{shape}: {said}");
    }
}

/// The ceiling is `analysis-inconclusive`'s code, which the transport cannot name, and what is no ceiling is refused before anything runs.
#[test]
fn a_ceiling_is_a_whole_number_of_mib() {
    let dir = scratch();
    for bad in ["many", "0", "-1", "1.5", ""] {
        let (code, said) = run_in(&dir, bad, &["check", "s.eadl"]);
        assert_eq!(code, Status::Usage.code(), "`{bad}`: {said}");
        assert!(said.contains(VARIABLE), "`{bad}`: {said}");
    }
    std::fs::write(dir.join("s.eadl"), HEAD).expect("written");
    let (code, said) = run_in(&dir, "4096", &["check", "s.eadl"]);
    assert_eq!(code, Status::Ok.code(), "{said}");
}

/// What the search keeps of one kind goes by that kind: a contradiction stands past a thousand names outside the system, and so does its status.
#[test]
fn a_cap_on_what_is_said_moves_no_verdict() {
    for n in [1, 1000, 1001, 1100] {
        let dir = scratch();
        std::fs::write(dir.join("s.eadl"), named_outside(n)).expect("written");
        let (code, said) = run_in(
            &dir,
            "2048",
            &["resolve", "s.eadl", "--out", "p", "--unchecked"],
        );
        assert_eq!(
            code,
            Status::InvalidDescription.code(),
            "{n} outside: {}",
            &said[..said.len().min(400)]
        );
    }
}

/// A module tree whose every written symbol is within the limit, and whose plan's names pass it, resolves, is checked and builds, and a symbol past it is still refused.
#[test]
fn the_limit_on_a_name_is_on_what_a_person_writes() {
    let dir = scratch();
    let (x, y, l) = ("x".repeat(85), "y".repeat(85), "l".repeat(85));
    std::fs::write(dir.join("low.eadl"), format!("{HEAD}(defmodule low (version 1 0) (export {l} dev)\n  (defservice {l} (requires (needs relative-delay)))\n  (defblock dev (offers (relative-delay true) (channels 1 tick) (reachable-at-privilege machine))))\n")).expect("written");
    std::fs::write(dir.join("mid.eadl"), format!("{HEAD}(defmodule mid (version 1 0) (import low (as {y}) (version (at-least 1 0))) (export svc soc)\n  (defservice svc (requires (uses {y}.{l})))\n  (defplatform soc (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses {y}.dev))))\n")).expect("written");
    let main = dir.join("app.main.eadl");
    std::fs::write(&main, format!("{HEAD}(defmodule app.main (version 1 0) (import mid (as {x}) (version (at-least 1 0))) (export app)\n  (defsystem app\n    (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses {x}.svc) (on-overrun fault))\n    (requires (uses {x}.svc))\n    (platform (uses {x}.soc))))\n")).expect("written");
    let file = main.display().to_string();
    let plan = dir.join("p").display().to_string();
    let (status, out, err) = invoke(&["resolve", &file, "--out", &plan], &Checker);
    assert_eq!(status, Status::Ok, "{out}\n{err}");
    let package = dir.join("b").display().to_string();
    let (status, out, err) = invoke(&["build", &file, "--out", &package], &Checker);
    assert_eq!(status, Status::Ok, "{out}\n{err}");
    // The record's name is past a symbol's limit, so this held only by reading it as a record.
    let written = std::fs::read_to_string(dir.join("p")).expect("the plan");
    assert!(written.contains(&format!("{x}.{y}.{l}")), "{written}");

    let long = dir.join("long.eadl");
    let name = "n".repeat(257);
    std::fs::write(
        &long,
        format!("{HEAD}(defblock {name} (offers (reachable-at-privilege machine)))\n"),
    )
    .expect("written");
    let (status, _, err) = invoke(&["check", &long.display().to_string()], &Checker);
    assert_eq!(status, Status::InvalidDescription, "{err}");
    assert!(err.contains("read-name-too-long"), "{err}");
}

/// A system's name as long as a name is gives a package name a file name can hold, with a hash and more beside it.
#[test]
fn a_package_s_name_fits_a_file_name() {
    let dir = scratch();
    let name = "s".repeat(256);
    let file = dir.join("s.eadl");
    std::fs::write(&file, format!("{HEAD}(defblock blk (offers (reachable-at-privilege machine)))\n{PLATFORM}(defsystem {name} {A_TASK} (platform (uses soc)))\n")).expect("written");
    let package = dir.join("b");
    let (status, out, err) = invoke(
        &[
            "build",
            &file.display().to_string(),
            "--out",
            &package.display().to_string(),
        ],
        &Checker,
    );
    assert_eq!(status, Status::Ok, "{out}\n{err}");
    let manifest = std::fs::read_to_string(package.join("Cargo.toml")).expect("the manifest");
    let line = manifest
        .lines()
        .find(|line| line.starts_with("name = "))
        .expect("a name");
    assert!(line.len() <= 150, "{line}");
}

/// Past a few labels an error says how many more there are: its labels are bounded as its sentences are, and its status stands.
#[test]
fn one_error_says_a_bounded_number_of_places() {
    let dir = scratch();
    std::fs::write(dir.join("s.eadl"), unmet(50, 200)).expect("written");
    let (code, said) = run_in(
        &dir,
        "2048",
        &["resolve", "s.eadl", "--out", "p", "--unchecked"],
    );
    assert_eq!(
        code,
        Status::InfeasibleConfiguration.code(),
        "{}",
        &said[..said.len().min(400)]
    );
    assert!(
        said.contains("more like this go unsaid"),
        "{}",
        &said[..said.len().min(4000)]
    );
    assert!(said.len() < 2 * 1024 * 1024, "{}", said.len());
}

/// What a run says of a path is one line of its own words, whatever the path holds, and an argument it refuses is said as printable as any.
#[test]
fn what_is_said_of_a_path_or_an_argument_is_printable_on_its_own_line() {
    let dir = scratch();
    let file = dir.join("x\nall clear: accepted\n.eadl");
    std::fs::write(&file, "(junk").expect("written");
    let (status, out, err) = invoke(&["check", &file.display().to_string()], &Checker);
    assert_eq!(status, Status::Usage, "{err}");
    assert!(err.contains("line break"), "{err}");
    for line in out.lines().chain(err.lines()) {
        assert_ne!(line.trim(), "all clear: accepted", "{err}");
    }
    for args in [
        vec!["check", "f", "--bogus\u{1b}[31m"],
        vec!["bogus\u{1b}[31m"],
        vec!["help", "bogus\u{1b}[31m"],
    ] {
        let (_, out, err) = invoke(&args, &Checker);
        for said in [&out, &err] {
            assert!(!said.contains('\u{1b}'), "{args:?}: {said:?}");
        }
    }
}

/// A source's name reaches a client through `explain`'s clauses within what a name is said in, and not whole per clause.
#[test]
fn a_source_s_name_is_fitted_in_what_a_client_reads() {
    let blocks =
        "(defblock b0 (offers (observable-output true) (reachable-at-privilege machine)))\n";
    let services = lines(1000, |i| {
        format!("(defservice s{i} (requires (needs observable-output)))\n")
    });
    let asked = uses("s", 1000);
    let text = format!("{HEAD}{blocks}(defplatform plat (offers (core-count 1 tick) (reachable-at-privilege machine)) (requires (uses b0)))\n{services}(defsystem m (task t (period 10 ms) (deadline 10 ms) (deadline-from release) (priority 1) (uses s0) (on-overrun fault)) (requires {asked}) (platform (uses plat)))\n");
    let name = "s".repeat(65_536);
    let quoted = text.replace('\n', "\\n");
    let dir = scratch();
    let messages = format!("{{\"jsonrpc\":\"2.0\",\"id\":0,\"method\":\"initialize\",\"params\":{{\"protocolVersion\":\"2025-11-25\",\"capabilities\":{{}},\"clientInfo\":{{\"name\":\"t\",\"version\":\"0\"}}}}}}\n{{\"jsonrpc\":\"2.0\",\"method\":\"notifications/initialized\"}}\n{{\"jsonrpc\":\"2.0\",\"id\":1,\"method\":\"tools/call\",\"params\":{{\"name\":\"explain\",\"arguments\":{{\"description\":\"{quoted}\",\"name\":\"{name}\",\"requirement\":\"observable-output\"}}}}}}\n");
    let input = dir.join("in.json");
    std::fs::write(&input, messages).expect("written");
    let output = Command::new(env!("CARGO_BIN_EXE_archogen"))
        .arg("mcp")
        .stdin(std::fs::File::open(&input).expect("opened"))
        .output()
        .expect("the server runs");
    assert!(output.status.success(), "{:?}", output.status);
    let answer = String::from_utf8_lossy(&output.stdout);
    assert!(
        answer.contains("\"clauses\""),
        "{}",
        &answer[..answer.len().min(600)]
    );
    assert!(
        output.stdout.len() < 16 * 1024 * 1024,
        "{}",
        output.stdout.len()
    );
}
