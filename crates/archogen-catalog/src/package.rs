//! §3's package rules beyond the sets (`M2.7.3.7`): what a manifest, its workspace manifest, a cargo configuration
//! file and a package's Rust source may not hold.
//!
//! Each rule is fail-closed and lexical. The manifest rules read the dialect by TOML's meaning, so a key cannot hide
//! behind another spelling. The token rules are a scanner over bytes: comments and the contents of literals are not
//! tokens, so a refused word in a comment or a string is admitted, and a refused word as a token anywhere is not. It
//! is pure, so the gate (`M2.7.4`) runs the same scanner over every path the compiler's dependency information lists.

use crate::manifest::{Manifest, Value};
use crate::tree::{self, Tree};

/// Identifiers refused anywhere as tokens: assembly, file inclusion, symbol and linkage control, global hooks, and
/// macro definition. `macro`, a keyword, is refused with them.
pub const REFUSED: [&str; 15] = [
    "asm",
    "global_asm",
    "naked_asm",
    "include",
    "include_str",
    "include_bytes",
    "debugger_visualizer",
    "no_mangle",
    "export_name",
    "link_section",
    "panic_handler",
    "global_allocator",
    "alloc_error_handler",
    "macro_rules",
    "macro",
];

/// Identifiers refused inside an attribute or a macro invocation's arguments, where they could become an attribute.
pub const ATTRIBUTE_WORDS: [&str; 3] = ["path", "link", "used"];

/// Keys refused in any table, whatever their value: each can make the package a procedural macro or another crate.
const CRATE_KIND_KEYS: [&str; 4] = ["proc-macro", "proc_macro", "crate-type", "crate_type"];

/// The tables whose `path` key names a target's source.
const TARGET_TABLES: [&str; 5] = ["lib", "bin", "test", "example", "bench"];

/// Rust's keywords, which a `!` after does not make a macro invocation (`if !(…)`).
pub(crate) const KEYWORDS: [&str; 39] = [
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "yield",
];

/// Whether `path` holds `key` as one of its segments.
fn holds(path: &[String], key: &str) -> bool {
    path.iter().any(|segment| segment == key)
}

/// The rules on a package's own manifest, `m` read from `manifest_path`, the package at `dir` of `tree`.
///
/// # Errors
///
/// Why the package is refused, in one sentence naming the manifest.
pub fn check_manifest(
    tree: &Tree,
    dir: &str,
    manifest_path: &str,
    m: &Manifest,
) -> Result<(), String> {
    let build_rs = tree::join(dir, "build.rs");
    if tree.is_file(&build_rs) {
        return Err(format!("`{build_rs}`: a package has no build script"));
    }
    if let Some(build) = m.get(&["package", "build"]) {
        if *build != Value::Bool(false) {
            return Err(format!(
                "`{manifest_path}`: `package.build` is other than `false`, a build script"
            ));
        }
    }
    for (path, _) in &m.values {
        if let Some(key) = CRATE_KIND_KEYS.iter().find(|k| holds(path, k)) {
            return Err(format!(
                "`{manifest_path}`: `{}` holds `{key}`, which can make the package another kind of crate",
                path.join(".")
            ));
        }
        if holds(path, "rustflags") {
            return Err(format!(
                "`{manifest_path}`: `{}` passes compiler flags",
                path.join(".")
            ));
        }
        if path.last().map(String::as_str) == Some("optional")
            && path.iter().any(|s| s.ends_with("dependencies"))
            && m.get(&path.iter().map(String::as_str).collect::<Vec<_>>())
                == Some(&Value::Bool(true))
        {
            return Err(format!(
                "`{manifest_path}`: `{}` makes a dependency optional, a second configuration",
                path.join(".")
            ));
        }
    }
    for table in &m.tables {
        if let Some(key) = CRATE_KIND_KEYS.iter().find(|k| holds(table, k)) {
            return Err(format!(
                "`{manifest_path}`: the table `{}` holds `{key}`",
                table.join(".")
            ));
        }
    }
    if m.get(&["package", "workspace"]).is_some() {
        return Err(format!(
            "`{manifest_path}`: `package.workspace` points cargo at another workspace"
        ));
    }
    if m.has_table(&["features"]) {
        return Err(format!(
            "`{manifest_path}`: `[features]` makes a second configuration"
        ));
    }
    if m.get(&["cargo-features"]).is_some() {
        return Err(format!(
            "`{manifest_path}`: `cargo-features` passes unstable flags"
        ));
    }
    for (path, value) in &m.values {
        let is_target = matches!(
            path.as_slice(),
            [table, key] if TARGET_TABLES.contains(&table.as_str()) && key == "path"
        ) || matches!(
            path.as_slice(),
            [table, index, key] if TARGET_TABLES.contains(&table.as_str()) && index.starts_with('[') && key == "path"
        );
        if !is_target {
            continue;
        }
        let Value::Str(source) = value else {
            return Err(format!(
                "`{manifest_path}`: `{}` is a string",
                path.join(".")
            ));
        };
        // `resolve` reads a leading `/` as nothing, and cargo as the file system's root.
        let inside = !source.starts_with('/')
            && tree::resolve(dir, source).is_some_and(|p| p.starts_with(&format!("{dir}/")));
        if !inside {
            return Err(format!(
                "`{manifest_path}`: `{}` = `{source}` leaves `{dir}`",
                path.join(".")
            ));
        }
    }
    Ok(())
}

/// The rules on a workspace manifest, `ws` read from `ws_path`.
///
/// # Errors
///
/// Why every package of the workspace is refused.
pub fn check_workspace(ws_path: &str, ws: &Manifest) -> Result<(), String> {
    for table in ["patch", "replace"] {
        if ws.has_table(&[table]) {
            return Err(format!(
                "`{ws_path}`: `[{table}]` swaps a dependency's source"
            ));
        }
    }
    if ws.get(&["cargo-features"]).is_some() {
        return Err(format!(
            "`{ws_path}`: `cargo-features` passes unstable flags"
        ));
    }
    if let Some((path, _)) = ws.values.iter().find(|(p, _)| holds(p, "rustflags")) {
        return Err(format!(
            "`{ws_path}`: `{}` passes compiler flags",
            path.join(".")
        ));
    }
    Ok(())
}

/// The rule on a cargo configuration file on a package's ancestor path: every key's path begins with `alias`,
/// however it is written, under a header, dotted, or in an inline table.
///
/// # Errors
///
/// Why the file, and every package below it, is refused.
pub fn check_config(path: &str, m: &Manifest) -> Result<(), String> {
    if let Some((key, _)) = m
        .values
        .iter()
        .find(|(p, _)| p.first().map(String::as_str) != Some("alias"))
    {
        return Err(format!(
            "`{path}`: `{}` is configuration other than an alias, which `/1` admits none of",
            key.join(".")
        ));
    }
    if let Some(table) = m
        .tables
        .iter()
        .find(|t| t.first().map(String::as_str) != Some("alias"))
    {
        return Err(format!(
            "`{path}`: the table `{}` is configuration other than an alias",
            table.join(".")
        ));
    }
    Ok(())
}

/// Where a token rule refused a source: 1-based line and column, and why.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Found {
    /// The line.
    pub line: usize,
    /// The column, in bytes.
    pub column: usize,
    /// Why.
    pub why: String,
}

/// What a token is ([`tokens`]).
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Kind {
    /// An identifier, by its name, `r#asm`'s among them; `true` when it holds a byte outside ASCII.
    Ident(String, bool),
    /// A string literal of any kind.
    Str,
    /// Any other literal, or a lifetime.
    Other,
    /// A punctuation character.
    Punct(char),
    /// An opening delimiter.
    Open(char),
    /// A closing delimiter.
    Close(char),
}

/// One token of a Rust source ([`tokens`]).
#[derive(Debug, Clone)]
pub struct Token {
    /// What it is.
    pub kind: Kind,
    /// Its first byte's offset.
    pub at: usize,
    /// The offset just past it.
    pub end: usize,
}

/// Rust's tokens in `source`, comments dropped and literals kept whole — the tokens §3's rules read, which the trust
/// instrument also reads to find a refused site's extent (`docs/decisions/decision_trust-inventory.md` §3).
///
/// # Errors
///
/// The offset and reason where `source` does not tokenize.
pub fn tokens(source: &[u8]) -> Result<Vec<Token>, (usize, String)> {
    let mut out = Vec::new();
    let mut i = 0;
    let n = source.len();
    let ident_byte = |b: u8| b.is_ascii_alphanumeric() || b == b'_' || b >= 0x80;
    while i < n {
        let b = source[i];
        let next = source.get(i + 1).copied();
        if b.is_ascii_whitespace() {
            i += 1;
        } else if b == b'/' && next == Some(b'/') {
            while i < n && source[i] != b'\n' {
                i += 1;
            }
        } else if b == b'/' && next == Some(b'*') {
            let start = i;
            let mut depth = 0usize;
            loop {
                if i + 1 >= n {
                    return Err((start, "a block comment is not closed".to_owned()));
                }
                if source[i] == b'/' && source[i + 1] == b'*' {
                    depth += 1;
                    i += 2;
                } else if source[i] == b'*' && source[i + 1] == b'/' {
                    depth -= 1;
                    i += 2;
                    if depth == 0 {
                        break;
                    }
                } else {
                    i += 1;
                }
            }
        } else if let Some(end) = string_at(source, i)? {
            out.push(Token {
                kind: Kind::Str,
                at: i,
                end,
            });
            i = end;
        } else if b == b'\'' {
            let start = i;
            // A byte character, `b'x'`, reads as the identifier `b` and a character: no rule tells them apart.
            i += 1;
            if source.get(i) == Some(&b'\\') {
                i += 2;
                while i < n && source[i] != b'\'' {
                    i += 1;
                }
                i += 1;
            } else {
                // One character, then `'` for a character literal; otherwise a lifetime or label.
                let len = utf8_len(source.get(i).copied().unwrap_or(0));
                if source.get(i + len) == Some(&b'\'') {
                    i += len + 1;
                } else {
                    let name_start = i;
                    while i < n && ident_byte(source[i]) {
                        i += 1;
                    }
                    if source[name_start..i].iter().any(|&c| c >= 0x80) {
                        return Err((start, "a lifetime that is not ASCII".to_owned()));
                    }
                }
            }
            out.push(Token {
                kind: Kind::Other,
                at: start,
                end: i,
            });
        } else if b.is_ascii_digit() {
            let start = i;
            while i < n && (source[i].is_ascii_alphanumeric() || source[i] == b'_') {
                i += 1;
            }
            if i + 1 < n && source[i] == b'.' && source[i + 1].is_ascii_digit() {
                i += 1;
                while i < n && (source[i].is_ascii_alphanumeric() || source[i] == b'_') {
                    i += 1;
                }
            }
            out.push(Token {
                kind: Kind::Other,
                at: start,
                end: i,
            });
        } else if ident_byte(b) {
            let start = i;
            // `r#asm` reads as `r`, `#` and `asm`: the name is a token either way, and a raw identifier's `#` is never
            // followed by `[` or `!`, so no rule can tell the two readings apart.
            let name_start = i;
            while i < n && ident_byte(source[i]) {
                i += 1;
            }
            let bytes = &source[name_start..i];
            let non_ascii = bytes.iter().any(|&c| c >= 0x80);
            let name = String::from_utf8_lossy(bytes).into_owned();
            out.push(Token {
                kind: Kind::Ident(name, non_ascii),
                at: start,
                end: i,
            });
        } else if matches!(b, b'(' | b'[' | b'{') {
            out.push(Token {
                kind: Kind::Open(char::from(b)),
                at: i,
                end: i + 1,
            });
            i += 1;
        } else if matches!(b, b')' | b']' | b'}') {
            out.push(Token {
                kind: Kind::Close(char::from(b)),
                at: i,
                end: i + 1,
            });
            i += 1;
        } else {
            // A macro invocation's `!` is the token just before its delimiter, so `!=` never reads as one.
            out.push(Token {
                kind: Kind::Punct(char::from(b)),
                at: i,
                end: i + 1,
            });
            i += 1;
        }
    }
    Ok(out)
}

/// The length of the UTF-8 sequence a leading byte starts.
fn utf8_len(b: u8) -> usize {
    match b {
        0xf0..=0xff => 4,
        0xe0..=0xef => 3,
        0xc0..=0xdf => 2,
        _ => 1,
    }
}

/// When a string literal of any kind starts at `i` (`"`, `b"`, `c"`, raw `r#*"`, `br#*"`, `cr#*"`), the index just
/// past it.
fn string_at(source: &[u8], i: usize) -> Result<Option<usize>, (usize, String)> {
    let mut j = i;
    if matches!(source.get(j), Some(b'b' | b'c')) {
        j += 1;
    }
    let raw = source.get(j) == Some(&b'r') && matches!(source.get(j + 1), Some(b'"' | b'#'));
    if raw {
        j += 1;
        let mut hashes = 0;
        while source.get(j) == Some(&b'#') {
            hashes += 1;
            j += 1;
        }
        if source.get(j) != Some(&b'"') {
            return Ok(None);
        }
        j += 1;
        loop {
            match source.get(j) {
                None => return Err((i, "a raw string is not closed".to_owned())),
                Some(b'"')
                    if source[j + 1..]
                        .iter()
                        .take(hashes)
                        .filter(|&&c| c == b'#')
                        .count()
                        == hashes
                        && source.len() >= j + 1 + hashes =>
                {
                    return Ok(Some(j + 1 + hashes));
                }
                _ => j += 1,
            }
        }
    }
    if source.get(j) != Some(&b'"') {
        return Ok(None);
    }
    j += 1;
    loop {
        match source.get(j) {
            None => return Err((i, "a string is not closed".to_owned())),
            Some(b'\\') => j += 2,
            Some(b'"') => return Ok(Some(j + 1)),
            _ => j += 1,
        }
    }
}

/// What an open delimiter opened.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Frame {
    Attribute,
    MacroArguments,
    Plain,
}

/// §3's token rules over one Rust source file of a package no `assembly` declaration names.
///
/// # Errors
///
/// The first token the rules refuse, with its line and column, or a source that does not tokenize.
pub fn scan(source: &[u8]) -> Result<(), Found> {
    scan_with(source, None)
}

/// §3's token rules over one Rust source file of a package an `assembly` declaration names for `architecture`:
/// `asm` and `naked_asm` admitted only as §14.2 admits them, each invocation held to its rules
/// ([`crate::assembly`]).
///
/// # Errors
///
/// The first token the rules refuse, with its line and column, or a source that does not tokenize.
pub fn scan_assembly(source: &[u8], architecture: &str) -> Result<(), Found> {
    scan_with(source, Some(architecture))
}

fn scan_with(source: &[u8], architecture: Option<&str>) -> Result<(), Found> {
    let at = |offset: usize, why: String| {
        let line = source[..offset].iter().filter(|&&b| b == b'\n').count() + 1;
        let column = offset
            - source[..offset]
                .iter()
                .rposition(|&b| b == b'\n')
                .map_or(0, |p| p + 1)
            + 1;
        Found { line, column, why }
    };
    let tokens = tokens(source).map_err(|(offset, why)| at(offset, why))?;
    let structure = architecture.map(|_| crate::assembly::Structure::of(source, &tokens));
    let mut stack: Vec<Frame> = Vec::new();
    for (k, token) in tokens.iter().enumerate() {
        let prev = k.checked_sub(1).map(|p| &tokens[p].kind);
        let before = k.checked_sub(2).map(|p| &tokens[p].kind);
        match &token.kind {
            Kind::Open(_) => {
                let frame = match (before, prev) {
                    (_, Some(Kind::Punct('#')))
                    | (Some(Kind::Punct('#')), Some(Kind::Punct('!')))
                        if token.kind == Kind::Open('[') =>
                    {
                        Frame::Attribute
                    }
                    (Some(Kind::Ident(name, _)), Some(Kind::Punct('!')))
                        if !KEYWORDS.contains(&name.as_str()) =>
                    {
                        Frame::MacroArguments
                    }
                    _ => Frame::Plain,
                };
                stack.push(frame);
            }
            Kind::Close(_) => {
                stack.pop();
            }
            Kind::Ident(name, non_ascii) => {
                if *non_ascii {
                    return Err(at(token.at, "an identifier that is not ASCII".to_owned()));
                }
                if let (Some(architecture), Some(structure), "asm" | "naked_asm") =
                    (architecture, &structure, name.as_str())
                {
                    crate::assembly::check(source, &tokens, structure, k, architecture)
                        .map_err(|(offset, why)| at(offset, why))?;
                } else if REFUSED.contains(&name.as_str()) {
                    return Err(at(token.at, format!("the identifier `{name}`")));
                }
                let enclosed = stack.iter().any(|f| *f != Frame::Plain);
                if enclosed && ATTRIBUTE_WORDS.contains(&name.as_str()) {
                    return Err(at(
                        token.at,
                        format!("`{name}` inside an attribute or a macro's arguments"),
                    ));
                }
                if name == "extern" {
                    let in_macro = stack.contains(&Frame::MacroArguments);
                    let next = tokens.get(k + 1).map(|t| &t.kind);
                    let after = match next {
                        Some(Kind::Str) => tokens.get(k + 2).map(|t| &t.kind),
                        other => other,
                    };
                    let admitted = matches!(next, Some(Kind::Ident(n, _)) if n == "crate")
                        || (!in_macro && matches!(after, Some(Kind::Ident(n, _)) if n == "fn"));
                    if !admitted {
                        return Err(at(
                            token.at,
                            "`extern` other than `extern crate` or an `extern fn` outside a macro's arguments"
                                .to_owned(),
                        ));
                    }
                }
            }
            Kind::Str | Kind::Other | Kind::Punct(_) => {}
        }
    }
    Ok(())
}
