//! The shape archogen's reader gives a file, in the form `scripts/third_opinion.sh` compares with an independent
//! recognizer's (leaf `M1.22`): one line per top-level form, a list as parentheses around its items, an atom as
//! its **raw source text**. On a read error it prints `error` and the codes, and nothing else.
//!
//! ⚠️ Raw text and nesting only — token boundaries and document completeness, which is all
//! `docs/semantics/grammar.md` defines. No kind, no value: whether `-1` is an integer is eADL's exactness rule, not
//! a question of syntax, and the recognizer it is compared with classifies its atoms by its own rules.
//!
//! A development-time tool: nothing in the engine depends on it, and it adds no dependency to the workspace.

use eadl_front::{read, Form, SourceMap};

fn main() {
    let Some(path) = std::env::args().nth(1) else {
        eprintln!("usage: reader_shape <file.eadl>");
        std::process::exit(2);
    };
    let text = match std::fs::read_to_string(&path) {
        Ok(text) => text,
        Err(error) => {
            eprintln!("reader_shape: {path}: {error}");
            std::process::exit(2);
        }
    };
    let mut sources = SourceMap::new();
    let Ok(id) = sources.add(path.clone(), text.clone()) else {
        eprintln!("reader_shape: {path}: too large to address");
        std::process::exit(2);
    };
    let (document, diagnostics) = read(&sources, id);
    if diagnostics.has_errors() {
        let codes: Vec<&str> = diagnostics.items().iter().map(|d| d.code).collect();
        println!("error {}", codes.join(" "));
        return;
    }
    for form in &document.forms {
        println!("{}", shape(form, &text));
    }
}

fn shape(form: &Form, text: &str) -> String {
    match form {
        Form::List { items, .. } => format!(
            "({})",
            items
                .iter()
                .map(|item| shape(item, text))
                .collect::<Vec<_>>()
                .join(" ")
        ),
        atom => {
            let span = atom.span();
            text.get(span.start as usize..span.end as usize)
                .unwrap_or("")
                .to_string()
        }
    }
}
