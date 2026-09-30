//! The engine API chapter's example is what the example prints (leaf `API.3.4`).
//!
//! `docs/book/src/engine-api.md` shows `cargo run -q -p archogen-api --example in_memory` and its output. The
//! example is compiled into this test and run in-process, and the lines under the command must equal what it
//! prints, so the chapter cannot keep showing output the API no longer gives.

#[path = "../examples/in_memory.rs"]
#[allow(dead_code)]
mod example;

use std::path::Path;

#[test]
fn the_books_example_is_what_the_example_prints() {
    let command = "$ cargo run -q -p archogen-api --example in_memory";
    let chapter = std::fs::read_to_string(
        Path::new(env!("CARGO_MANIFEST_DIR")).join("../../docs/book/src/engine-api.md"),
    )
    .expect("the engine API chapter is readable");
    let shown: Vec<&str> = chapter
        .lines()
        .skip_while(|line| *line != command)
        .skip(1)
        .take_while(|line| *line != "```")
        .collect();
    assert!(
        !shown.is_empty(),
        "docs/book/src/engine-api.md no longer shows `{command}`"
    );
    let mut printed: Vec<u8> = Vec::new();
    example::run(&mut printed);
    let printed = String::from_utf8(printed).expect("utf-8");
    assert_eq!(
        shown,
        printed.lines().collect::<Vec<_>>(),
        "the chapter's transcript differs from a run"
    );
}
