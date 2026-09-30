//! A module tree is bounded (`docs/semantics/reference.md` §6 rule 11, leaf `M1.39`): at most
//! [`MAX_INSTANCES`] instances, and import chains at most [`MAX_IMPORT_DEPTH`] modules long.
//!
//! Each limit is pinned at its edge — the largest tree it admits is admitted, one more is refused — and
//! elaboration is shown to stop at the first refusal, so a tree far past a limit costs no more than one at it.
//! Before the limits, a fan-out of 19 small modules elaborated into half a million instances and a chain of
//! 3 000 overflowed the stack.

use eadl_front::module::{elaborate, MemoryModules, MAX_IMPORT_DEPTH, MAX_INSTANCES};
use eadl_front::SourceMap;

/// Elaborate `root` and return `(instances, diagnostic codes)`.
fn run(modules: &MemoryModules, root: &str) -> (usize, Vec<&'static str>) {
    let mut sources = SourceMap::new();
    let (program, diagnostics) = elaborate(&mut sources, modules, root);
    let codes = diagnostics.items().iter().map(|d| d.code).collect();
    (program.instances.len(), codes)
}

/// A chain of `length` modules, `c0` importing `c1` importing … the last, which imports nothing.
fn chain(length: usize) -> MemoryModules {
    let mut modules = MemoryModules::new();
    for i in 0..length {
        let text = if i + 1 == length {
            format!("(defmodule c{i} (version 1 0))\n")
        } else {
            format!(
                "(defmodule c{i} (version 1 0) (import c{} (as next)))\n",
                i + 1
            )
        };
        modules = modules.with(&format!("c{i}"), &text);
    }
    modules
}

/// `root` importing `leaf` `imports` times, under distinct aliases: `imports + 1` instances.
fn wide(imports: usize) -> MemoryModules {
    let aliases: String = (0..imports)
        .map(|i| format!(" (import leaf (as x{i}))"))
        .collect();
    MemoryModules::new()
        .with(
            "root",
            &format!("(defmodule root (version 1 0){aliases})\n"),
        )
        .with("leaf", "(defmodule leaf (version 1 0))\n")
}

#[test]
fn the_longest_chain_the_limit_admits_is_admitted_and_one_more_is_refused() {
    let (instances, codes) = run(&chain(MAX_IMPORT_DEPTH), "c0");
    assert!(codes.is_empty(), "{codes:?}");
    assert_eq!(instances, MAX_IMPORT_DEPTH);
    let (_, codes) = run(&chain(MAX_IMPORT_DEPTH + 1), "c0");
    assert_eq!(codes, ["module-import-too-deep"]);
}

#[test]
fn a_chain_far_past_the_limit_is_one_refusal_and_not_a_stack_overflow() {
    // ⛔ Three thousand links aborted the process before `M1.39`.
    let (instances, codes) = run(&chain(3_000), "c0");
    assert_eq!(codes, ["module-import-too-deep"]);
    // The modules already begun finish on the way back up; nothing past the refused link is read.
    assert_eq!(instances, MAX_IMPORT_DEPTH);
}

#[test]
fn the_largest_tree_the_limit_admits_is_admitted_and_one_more_instance_is_refused() {
    let (instances, codes) = run(&wide(MAX_INSTANCES - 1), "root");
    assert!(codes.is_empty(), "{codes:?}");
    assert_eq!(instances, MAX_INSTANCES);
    let (instances, codes) = run(&wide(MAX_INSTANCES), "root");
    assert_eq!(codes, ["module-too-many-instances"]);
    // The root and the leaves imported before the refused one finish: the limit's worth, and no more.
    assert_eq!(instances, MAX_INSTANCES);
}

#[test]
fn a_fan_out_far_past_the_limit_stops_at_the_first_refusal() {
    // Each module imports the next twice: 2^(links+1) − 1 instances. Twelve links would be 8 191; the depth stays
    // under its limit, so the instance limit is the one met, once, and elaboration stops there.
    let mut modules = MemoryModules::new();
    let links = 12;
    for i in 0..links {
        modules = modules.with(
            &format!("f{i}"),
            &format!(
                "(defmodule f{i} (version 1 0) (import f{0} (as x)) (import f{0} (as y)))\n",
                i + 1
            ),
        );
    }
    modules = modules.with(
        &format!("f{links}"),
        &format!("(defmodule f{links} (version 1 0))\n"),
    );
    let (instances, codes) = run(&modules, "f0");
    assert_eq!(codes, ["module-too-many-instances"]);
    assert!(instances <= MAX_INSTANCES, "{instances}");
}
