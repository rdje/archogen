//! The host build's answer to each description named on the command line, one JSON response per line: the side of
//! `scripts/wasm_binding.sh`'s comparison that did not go through WebAssembly (`docs/decisions/decision_wasm-binding.md`
//! §8).
//!
//! Each request is what `archogen check <path>` asks: the path as the description's name, the default profile,
//! and as modules every `*.eadl` file of the description's own directory under its stem, which is where the
//! command line's directory source finds a module (`<dir>/<module>.eadl`). The harness builds the same request in
//! JavaScript, so the two sides differ only in how they reached `answer`.

use std::fs;
use std::path::Path;
use std::process::ExitCode;

use archogen_wasm::answer;
use archogen_wasm::request::{frame, Framed};

/// The request `archogen check <path>` makes, framed.
fn request(path: &str) -> Result<Vec<u8>, String> {
    let text = fs::read_to_string(path).map_err(|error| format!("{path}: {error}"))?;
    let dir = Path::new(path).parent().unwrap_or_else(|| Path::new(""));
    let mut modules = Vec::new();
    let entries = fs::read_dir(if dir.as_os_str().is_empty() {
        Path::new(".")
    } else {
        dir
    })
    .map_err(|error| format!("{}: {error}", dir.display()))?;
    let mut paths: Vec<_> = entries
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .collect();
    paths.sort();
    for module in paths {
        if module.extension().and_then(|e| e.to_str()) != Some("eadl") || !module.is_file() {
            continue;
        }
        let Some(stem) = module.file_stem().and_then(|s| s.to_str()) else {
            continue;
        };
        let text = fs::read_to_string(&module)
            .map_err(|error| format!("{}: {error}", module.display()))?;
        modules.push((stem.to_string(), text));
    }
    Ok(frame(&Framed {
        name: path.to_string(),
        text,
        profile: None,
        modules,
    }))
}

fn main() -> ExitCode {
    let mut failed = false;
    for path in std::env::args().skip(1) {
        match request(&path) {
            Ok(bytes) => println!("{}", answer(&bytes)),
            Err(error) => {
                eprintln!("answers: {error}");
                failed = true;
            }
        }
    }
    if failed {
        ExitCode::FAILURE
    } else {
        ExitCode::SUCCESS
    }
}
