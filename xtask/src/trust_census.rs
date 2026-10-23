//! What a target compiles, read from its sources with nothing built (`docs/specs/trust/decision_trust-role-programs.md`
//! §3; round 10's change of method): from its crate root, every file `mod`, `#[path]` and the `include!` family reach,
//! whatever their `cfg`, so no profile, feature, flag or host decides it. A site the census cannot follow by its
//! text alone — no single string literal, an alias or a macro's, a path out of the tree or through a symbolic link, a
//! file that does not tokenize, a doc test's own — is returned unfollowed, for the inventory to refuse unless an
//! admission names it.
//!
//! The Rust reference of the pin decides where a module's file is: "When the module does not have a `path` attribute,
//! the path to the file mirrors the logical module path"; "For `path` attributes on modules not inside inline module
//! blocks, the file path is relative to the directory the source file is located"; inside inline module blocks it is
//! relative to the mod-rs file's directory "including the inline module components as directories", and for a
//! non-mod-rs file "the path starts with a directory with the name of the non-mod-rs module". Where the reference is
//! silent — the children of a module a `path` attribute loads, the `mod` items of a file an `include!` brings in — the
//! pin decides, measured `2026-10-23` by its own refusal of a missing child: "create file \"src/pl/child.rs\" or
//! \"src/pl/child/mod.rs\"" for `src/pl/loaded.rs`, "\"src/inc/c.rs\"" for `src/inc/i.rs` — each under the loaded
//! file's own directory, which the census's oracle holds on every build of its corpus.

use std::collections::BTreeSet;

use archogen_catalog::package::{tokens, Kind, Token};
use archogen_catalog::tree::Tree;
use archogen_evidence::sha256::Digest;

/// The rule an admission of an unfollowed site names.
pub(crate) const RULE: &str = "inclusion";

/// A site the census cannot follow by its text alone.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub(crate) struct Unfollowed {
    /// The file holding it.
    pub file: String,
    /// Its line, 1-based.
    pub line: usize,
    /// The sha256 of its extent's text, which an admission names.
    pub sha256: String,
    /// Why it cannot be followed.
    pub why: String,
}

/// What a crate root reaches.
#[derive(Debug, Default)]
pub(crate) struct Reach {
    /// Every file it reaches, its own among them, each a blob of the tree.
    pub files: BTreeSet<String>,
    /// Those it reaches through no item under `#[cfg(test)]`: what it compiles when built for another crate, or for
    /// itself as no test — `cfg(test)` is set only on the crate a test harness builds, never on a dependency.
    pub nontest: BTreeSet<String>,
    /// Every site it cannot follow.
    pub unfollowed: BTreeSet<Unfollowed>,
}

/// Where an absolute path, or one climbing out of the tree, lands: `outer(from, rel)` names the file of the tree it
/// reaches on disk, from the directory `from`, or none for a file outside the tree, which joins no role.
pub(crate) type Outer<'a> = &'a dyn Fn(&str, &str) -> Option<String>;

/// Every file the crate root `root` reaches in `tree`, `links` the tree's symbolic links.
pub(crate) fn reach(tree: &Tree, links: &BTreeSet<String>, root: &str, outer: Outer) -> Reach {
    let mut out = Reach::default();
    let mut seen: BTreeSet<(String, Vec<String>, bool)> = BTreeSet::new();
    // Each file to read, with the directories its `mod name;` children may lie under, and whether it is reached only
    // through an item under `#[cfg(test)]`.
    let mut todo: Vec<(String, Vec<String>, bool)> =
        vec![(root.to_owned(), vec![parent(root)], false)];
    while let Some((file, bases, test_only)) = todo.pop() {
        if !seen.insert((file.clone(), bases.clone(), test_only)) {
            continue;
        }
        let Some(source) = tree.get(&file) else {
            continue;
        };
        out.files.insert(file.clone());
        if !test_only {
            out.nontest.insert(file.clone());
        }
        let scan = match scan(&file, source) {
            Ok(scan) => scan,
            Err((at, why)) => {
                out.unfollowed.insert(Unfollowed {
                    file: file.clone(),
                    line: line_of(source, at),
                    sha256: Digest::of(source).hex(),
                    why: format!(
                        "does not tokenize ({why}), so the census cannot read what it includes"
                    ),
                });
                continue;
            }
        };
        out.unfollowed.extend(scan.unfollowed);
        let dir = parent(&file);
        // A path resolved, through no symbolic link, to a blob of the tree; none for a file the tree does not hold.
        let follow = |rel: &str, from: &str, site: &Site| -> Result<Option<String>, Unfollowed> {
            let resolved = match resolve(from, rel) {
                Ok(Some(path)) => Ok(path),
                // Out of the tree's text: followed if it lands back inside the written tree, else nothing of it is a
                // role package's.
                Ok(None) => match outer(from, rel) {
                    Some(path) => Ok(path),
                    None => return Ok(None),
                },
                Err(why) => Err(why),
            };
            match resolved {
                Err(why) => Err(site.unfollowed(&file, why)),
                Ok(path) if through_link(&path, links) => Err(site.unfollowed(
                    &file,
                    "reaches a symbolic link, whose file the census does not read",
                )),
                Ok(path) => Ok(tree.is_file(&path).then_some(path)),
            }
        };
        let found = |r: Result<Option<String>, Unfollowed>, out: &mut Reach| match r {
            Err(u) => {
                out.unfollowed.insert(u);
                None
            }
            Ok(path) => path,
        };
        for m in &scan.modules {
            match m.paths.as_slice() {
                // `mod name;`: its file mirrors its module path, under each directory this file's children may lie in.
                [] => {
                    for dir in bases.iter().flat_map(|base| expansions(base, &m.inline)) {
                        for candidate in [format!("{}.rs", m.name), format!("{}/mod.rs", m.name)] {
                            if let Some(path) = found(follow(&candidate, &dir, &m.site), &mut out) {
                                todo.push((
                                    path,
                                    vec![join(&dir, &m.name)],
                                    test_only || m.test_only,
                                ));
                            }
                        }
                    }
                }
                // `#[path = "…"] mod name;`: relative to this file's directory, or inside inline modules to it with the
                // inline components — after the non-mod-rs file's own name, when it is one. Each `path` attribute the
                // item carries, `cfg_attr`'s in every configuration, is followed.
                rels => {
                    let mut froms = Vec::new();
                    if m.inline.is_empty() {
                        froms.push(dir.clone());
                    } else {
                        froms.extend(expansions(&dir, &m.inline));
                        if let Some(stem) = non_mod_rs_stem(&file, root) {
                            froms.extend(expansions(&join(&dir, &stem), &m.inline));
                        }
                    }
                    for (from, rel) in froms.iter().flat_map(|f| rels.iter().map(move |r| (f, r))) {
                        if let Some(path) = found(follow(rel, from, &m.site), &mut out) {
                            let children = children_of_path_module(&path);
                            todo.push((path, children, test_only || m.test_only));
                        }
                    }
                }
            }
        }
        for inc in &scan.includes {
            match &inc.literal {
                None => {
                    out.unfollowed.insert(inc.site.unfollowed(
                        &file,
                        "is no single string literal, so the file it names is decided by its expansion",
                    ));
                }
                Some(rel) => {
                    let Some(path) = found(follow(rel, &dir, &inc.site), &mut out) else {
                        continue;
                    };
                    match inc.what {
                        // An `include!`d file is read as Rust, its `mod` children under its own directory, as the pin
                        // resolves them (the module documentation's measurement).
                        What::Rust => {
                            let children = vec![parent(&path)];
                            todo.push((path, children, test_only || inc.test_only));
                        }
                        What::Data => {
                            if !(test_only || inc.test_only) {
                                out.nontest.insert(path.clone());
                            }
                            out.files.insert(path);
                        }
                        // A document a doc attribute includes: its code blocks are doc tests, read for their own sites.
                        What::Doc => {
                            if let Some(text) = tree.get(&path) {
                                for site in doc_sites(&numbered(&String::from_utf8_lossy(text), 1))
                                {
                                    out.unfollowed.insert(Unfollowed {
                                        file: path.clone(),
                                        line: site.line,
                                        sha256: Digest::of(site.extent.as_bytes()).hex(),
                                        why: DOC_TEST.to_owned(),
                                    });
                                }
                            }
                            if !(test_only || inc.test_only) {
                                out.nontest.insert(path.clone());
                            }
                            out.files.insert(path);
                        }
                    }
                }
            }
        }
    }
    out
}

const DOC_TEST: &str = "is in a doc test, whose files rustdoc resolves apart from the crate";

/// Where a site is: its line and the text of its extent.
#[derive(Debug, Clone)]
struct Site {
    line: usize,
    extent: String,
}

impl Site {
    fn unfollowed(&self, file: &str, why: &str) -> Unfollowed {
        Unfollowed {
            file: file.to_owned(),
            line: self.line,
            sha256: Digest::of(self.extent.as_bytes()).hex(),
            why: why.to_owned(),
        }
    }
}

/// A `mod name;` declaration.
#[derive(Debug)]
struct Module {
    name: String,
    /// Its `path` attributes, decoded: one, or one a `cfg_attr` each — every one a candidate.
    paths: Vec<String>,
    /// The inline modules it is declared in, outermost first, each by its `path` attributes or its name: every
    /// alternative a directory its children may lie under.
    inline: Vec<Vec<String>>,
    /// Whether it lies in an item under `#[cfg(test)]`.
    test_only: bool,
    site: Site,
}

/// What an inclusion brings in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum What {
    /// `include!`: Rust.
    Rust,
    /// `include_str!` or `include_bytes!`: data.
    Data,
    /// `include_str!` inside a `doc` attribute: a document whose code blocks are doc tests.
    Doc,
}

/// An `include!`, `include_str!` or `include_bytes!` invocation.
#[derive(Debug)]
struct Include {
    what: What,
    /// Its one string literal, decoded; none when it is no single string literal.
    literal: Option<String>,
    /// Whether it lies in an item under `#[cfg(test)]`.
    test_only: bool,
    site: Site,
}

/// One file's sites.
#[derive(Debug, Default)]
struct Scan {
    modules: Vec<Module>,
    includes: Vec<Include>,
    unfollowed: Vec<Unfollowed>,
}

const INCLUDES: [&str; 3] = ["include", "include_str", "include_bytes"];

fn name(t: &Token) -> Option<&str> {
    match &t.kind {
        Kind::Ident(n, _) => Some(n.strip_prefix("r#").unwrap_or(n)),
        _ => None,
    }
}

/// The index of the delimiter closing the one opened at `open`, or the last token's.
fn closing(ts: &[Token], open: usize) -> usize {
    let mut depth = 0usize;
    for (i, t) in ts.iter().enumerate().skip(open) {
        match t.kind {
            Kind::Open(_) => depth += 1,
            Kind::Close(_) => {
                depth = depth.saturating_sub(1);
                if depth == 0 {
                    return i;
                }
            }
            _ => {}
        }
    }
    ts.len().saturating_sub(1)
}

/// The sites of one Rust source, its doc text's among them.
fn scan(file: &str, source: &[u8]) -> Result<Scan, (usize, String)> {
    let ts = tokens(source)?;
    let mut out = Scan::default();
    let test_only = test_only_spans(&ts);
    let in_test = |k: usize| test_only.iter().any(|&(a, b)| a <= k && k <= b);
    let text =
        |a: usize, b: usize| String::from_utf8_lossy(&source[ts[a].at..ts[b].end]).into_owned();
    let site = |a: usize, b: usize| Site {
        line: line_of(source, ts[a].at),
        extent: text(a, b),
    };
    // Inline modules open: (the brace depth inside them, their directory component).
    let mut inline: Vec<(usize, Vec<String>)> = Vec::new();
    // The closing delimiter of the `macro_rules!` body open, if any.
    let mut macro_end: Option<usize> = None;
    // Whether a `use` item is open, up to its `;`.
    let mut in_use = false;
    let mut depth = 0usize;
    // A `path` attribute seen, carried to the `mod` item it precedes: its value and its `#`'s index.
    let mut pending_path: Vec<(String, usize)> = Vec::new();
    let mut i = 0;
    while i < ts.len() {
        if macro_end.is_some_and(|end| i > end) {
            macro_end = None;
        }
        let in_macro = macro_end.is_some();
        // An attribute, `#[…]` or `#![…]`, read whole: its `path`, its `doc` text, its inclusions.
        if ts[i].kind == Kind::Punct('#') {
            if in_macro && ts.get(i + 1).is_some_and(|t| t.kind == Kind::Punct('$')) {
                out.unfollowed.push(site(i, i + 1).unfollowed(
                    file,
                    "is an attribute a macro assembles, so the module it may load is decided by its expansion",
                ));
            }
            let bang = ts.get(i + 1).is_some_and(|t| t.kind == Kind::Punct('!'));
            let open = if bang { i + 2 } else { i + 1 };
            if ts.get(open).is_some_and(|t| t.kind == Kind::Open('[')) {
                let close = closing(&ts, open);
                let mut j = open + 1;
                while j < close {
                    let key = name(&ts[j]);
                    let assigned = ts.get(j + 1).is_some_and(|t| t.kind == Kind::Punct('='));
                    let value = ts.get(j + 2).filter(|t| t.kind == Kind::Str);
                    if key == Some("path") && assigned {
                        match value.and_then(|v| decode(&source[v.at..v.end])) {
                            Some(p) if !in_macro => pending_path.push((p, i)),
                            _ => out.unfollowed.push(site(i, close).unfollowed(
                                file,
                                if in_macro {
                                    "is a `path` attribute inside a macro, so the file it names is decided by its \
                                     expansion"
                                } else {
                                    "is a `path` attribute whose value is no single string literal"
                                },
                            )),
                        }
                    }
                    if key == Some("doc") && assigned {
                        if let Some(v) = value {
                            let text = decode(&source[v.at..v.end]).unwrap_or_default();
                            for s in doc_sites(&numbered(&text, line_of(source, v.at))) {
                                out.unfollowed.push(s.unfollowed(file, DOC_TEST));
                            }
                        }
                    }
                    if let Some(end) =
                        inclusion(&ts, j, source, file, in_macro, in_test(j), &mut out, &site)
                    {
                        j = end;
                    }
                    j += 1;
                }
                i = close + 1;
                continue;
            }
        }
        match &ts[i].kind {
            Kind::Open('{') => depth += 1,
            Kind::Close('}') => {
                if inline.last().is_some_and(|(d, _)| *d == depth) {
                    inline.pop();
                }
                depth = depth.saturating_sub(1);
                pending_path.clear();
            }
            Kind::Punct(';') => {
                in_use = false;
                pending_path.clear();
            }
            _ => {}
        }
        match name(&ts[i]) {
            Some("use") => in_use = true,
            Some("macro_rules") if ts.get(i + 1).is_some_and(|t| t.kind == Kind::Punct('!')) => {
                // `macro_rules! name { … }`: a body whose inclusions its expansion decides; one named for an inclusion
                // macro shadows it.
                if ts
                    .get(i + 2)
                    .and_then(name)
                    .is_some_and(|n| INCLUDES.contains(&n))
                {
                    out.unfollowed.push(site(i, i + 2).unfollowed(file, ALIAS));
                }
                if let Some(open) = (i + 2..ts.len()).find(|&k| matches!(ts[k].kind, Kind::Open(_)))
                {
                    if macro_end.is_none() {
                        macro_end = Some(closing(&ts, open));
                    }
                }
            }
            Some("mod") => {
                let declared = ts.get(i + 1).and_then(name).map(str::to_owned);
                let ends = ts.get(i + 2).map(|t| t.kind.clone());
                if in_macro {
                    if ends == Some(Kind::Punct(';'))
                        || ts.get(i + 1).is_some_and(|t| t.kind == Kind::Punct('$'))
                    {
                        out.unfollowed
                            .push(site(i, (i + 2).min(ts.len() - 1)).unfollowed(
                            file,
                            "is a module a macro declares, so its file is decided by its expansion",
                        ));
                    }
                } else if let Some(n) = declared {
                    let paths = std::mem::take(&mut pending_path);
                    match ends {
                        Some(Kind::Punct(';')) => out.modules.push(Module {
                            site: site(paths.first().map_or(i, |(_, at)| *at), i + 2),
                            paths: paths.into_iter().map(|(p, _)| p).collect(),
                            inline: inline.iter().map(|(_, c)| c.clone()).collect(),
                            test_only: in_test(i),
                            name: n,
                        }),
                        // Its brace is counted when the loop reaches it.
                        Some(Kind::Open('{')) => {
                            let alternatives = if paths.is_empty() {
                                vec![n]
                            } else {
                                paths.into_iter().map(|(p, _)| p).collect()
                            };
                            inline.push((depth + 1, alternatives));
                        }
                        _ => {}
                    }
                }
            }
            Some(n) if INCLUDES.contains(&n) && in_use => {
                out.unfollowed.push(site(i, i).unfollowed(file, ALIAS));
            }
            Some(_) => {
                if let Some(end) =
                    inclusion(&ts, i, source, file, in_macro, in_test(i), &mut out, &site)
                {
                    i = end + 1;
                    continue;
                }
            }
            None => {}
        }
        i += 1;
    }
    for s in doc_sites(&doc_comments(source, &ts)) {
        out.unfollowed.push(s.unfollowed(file, DOC_TEST));
    }
    Ok(out)
}

const ALIAS: &str = "imports or shadows an inclusion macro, so no literal pins what it includes";

/// The inclusion invoked at `i`, if one is: recorded in `out`, its closing delimiter's index returned.
#[allow(clippy::too_many_arguments)]
fn inclusion(
    ts: &[Token],
    i: usize,
    source: &[u8],
    file: &str,
    in_macro: bool,
    test_only: bool,
    out: &mut Scan,
    site: &dyn Fn(usize, usize) -> Site,
) -> Option<usize> {
    let n = name(&ts[i]).filter(|n| INCLUDES.contains(n))?;
    let invoked = ts.get(i + 1).is_some_and(|t| t.kind == Kind::Punct('!'))
        && ts
            .get(i + 2)
            .is_some_and(|t| matches!(t.kind, Kind::Open(_)));
    if !invoked {
        return None;
    }
    let close = closing(ts, i + 2);
    let literal = match ts.get(i + 3..close).unwrap_or_default() {
        [one] if one.kind == Kind::Str => decode(&source[one.at..one.end]),
        [one, comma] if one.kind == Kind::Str && comma.kind == Kind::Punct(',') => {
            decode(&source[one.at..one.end])
        }
        _ => None,
    };
    let what = match n {
        "include" => What::Rust,
        _ if within_doc(ts, i) => What::Doc,
        _ => What::Data,
    };
    if in_macro {
        out.unfollowed.push(site(i, close).unfollowed(
            file,
            "is an inclusion inside a macro, so the file it names is decided by its expansion",
        ));
    } else {
        out.includes.push(Include {
            what,
            literal,
            test_only,
            site: site(i, close),
        });
    }
    Some(close)
}

/// The token spans of the items under exactly `#[cfg(test)]` — from the attribute to the item's `;` or its closing
/// brace — and the whole file under `#![cfg(test)]`. Any other `cfg` is read as on, so the census over-counts.
fn test_only_spans(ts: &[Token]) -> Vec<(usize, usize)> {
    let is = |k: usize, kind: Kind| ts.get(k).is_some_and(|t| t.kind == kind);
    let named = |k: usize, n: &str| ts.get(k).and_then(name) == Some(n);
    let mut out = Vec::new();
    let mut i = 0;
    while i < ts.len() {
        let inner = is(i, Kind::Punct('#')) && is(i + 1, Kind::Punct('!'));
        let open = if inner { i + 2 } else { i + 1 };
        let cfg_test = is(i, Kind::Punct('#'))
            && is(open, Kind::Open('['))
            && named(open + 1, "cfg")
            && is(open + 2, Kind::Open('('))
            && named(open + 3, "test")
            && is(open + 4, Kind::Close(')'))
            && is(open + 5, Kind::Close(']'));
        if !cfg_test {
            i += 1;
            continue;
        }
        if inner {
            out.push((0, ts.len()));
            break;
        }
        // The item the attribute stands on: past further attributes, to its `;` or its first brace's close.
        let mut k = open + 6;
        while is(k, Kind::Punct('#')) {
            let o = if is(k + 1, Kind::Punct('!')) {
                k + 2
            } else {
                k + 1
            };
            k = closing(ts, o) + 1;
        }
        let mut end = ts.len().saturating_sub(1);
        let mut j = k;
        while j < ts.len() {
            match ts[j].kind {
                Kind::Punct(';') => {
                    end = j;
                    break;
                }
                Kind::Open('{') => {
                    end = closing(ts, j);
                    break;
                }
                Kind::Open(_) => j = closing(ts, j),
                _ => {}
            }
            j += 1;
        }
        out.push((i, end));
        i = end + 1;
    }
    out
}

/// Whether the invocation at `i` is a `doc` attribute's value: `doc =` before it.
fn within_doc(ts: &[Token], i: usize) -> bool {
    i >= 2 && ts[i - 1].kind == Kind::Punct('=') && name(&ts[i - 2]) == Some("doc")
}

/// The doc comments of `source` — `///`, `//!`, `/** */` and `/*! */`, which the lexer drops — as numbered lines of
/// text, in order.
fn doc_comments(source: &[u8], ts: &[Token]) -> Vec<(usize, String)> {
    let mut gaps = Vec::new();
    let mut at = 0;
    for t in ts {
        gaps.push((at, t.at));
        at = t.end;
    }
    gaps.push((at, source.len()));
    let mut out = Vec::new();
    for (a, b) in gaps {
        let gap = &source[a..b];
        let mut k = 0;
        while k < gap.len() {
            let rest = &gap[k..];
            if rest.starts_with(b"//") {
                let end = rest.iter().position(|&c| c == b'\n').unwrap_or(rest.len());
                let line = &rest[..end];
                if (line.starts_with(b"///") && !line.starts_with(b"////"))
                    || line.starts_with(b"//!")
                {
                    let body = &line[3..];
                    let body = body.strip_prefix(b" ").unwrap_or(body);
                    out.push((
                        line_of(source, a + k),
                        String::from_utf8_lossy(body).into_owned(),
                    ));
                }
                k += end.max(1);
            } else if rest.starts_with(b"/*") {
                let end = rest
                    .windows(2)
                    .position(|w| w == b"*/")
                    .map_or(rest.len(), |p| p + 2);
                let block = &rest[..end];
                let doc =
                    (block.starts_with(b"/**") && !block.starts_with(b"/***") && block != b"/**/")
                        || block.starts_with(b"/*!");
                if doc && block.len() >= 5 {
                    let body = String::from_utf8_lossy(&block[3..block.len() - 2]).into_owned();
                    out.extend(numbered(&body, line_of(source, a + k)));
                }
                k += end.max(1);
            } else {
                k += 1;
            }
        }
    }
    out
}

/// `text`'s lines, numbered from `first`.
fn numbered(text: &str, first: usize) -> Vec<(usize, String)> {
    text.lines()
        .enumerate()
        .map(|(n, l)| (first + n, l.to_owned()))
        .collect()
}

/// The inclusion sites of the code blocks of doc text, its lines numbered: every `include!`, `include_str!`,
/// `include_bytes!` and `path = …` in a block rustdoc may compile.
fn doc_sites(lines: &[(usize, String)]) -> Vec<Site> {
    let mut out = Vec::new();
    // The block open: whether rustdoc compiles it, and its lines.
    let mut block: Option<(bool, Vec<(usize, String)>)> = None;
    for (n, line) in lines {
        let trimmed = line.trim_start();
        if let Some(info) = trimmed.strip_prefix("```") {
            match block.take() {
                Some((true, code)) => out.extend(code_sites(&code)),
                Some((false, _)) => {}
                None => {
                    // rustdoc compiles a block with no language, `rust`, or only its own attributes.
                    let compiled = info.split([',', ' ']).filter(|w| !w.is_empty()).all(|w| {
                        matches!(
                            w,
                            "rust"
                                | "should_panic"
                                | "no_run"
                                | "compile_fail"
                                | "test_harness"
                                | "standalone_crate"
                        ) || w.starts_with("edition")
                            || w.starts_with('E')
                    });
                    block = Some((compiled, Vec::new()));
                }
            }
        } else if let Some((true, code)) = block.as_mut() {
            // A hidden line, `# …`, is compiled as any other.
            code.push((*n, trimmed.strip_prefix("# ").unwrap_or(trimmed).to_owned()));
        }
    }
    if let Some((true, code)) = block {
        out.extend(code_sites(&code));
    }
    out
}

fn code_sites(code: &[(usize, String)]) -> Vec<Site> {
    let text: String = code.iter().map(|(_, l)| format!("{l}\n")).collect();
    let Ok(ts) = tokens(text.as_bytes()) else {
        return Vec::new();
    };
    let mut out = Vec::new();
    for (i, t) in ts.iter().enumerate() {
        let hit = name(t).is_some_and(|n| INCLUDES.contains(&n))
            || (name(t) == Some("path")
                && ts.get(i + 1).is_some_and(|u| u.kind == Kind::Punct('=')));
        if hit {
            let row = text[..t.at].matches('\n').count();
            out.push(Site {
                line: code.get(row).map_or(0, |(n, _)| *n),
                extent: text[t.at..t.end].to_owned(),
            });
        }
    }
    out
}

/// A string literal's value: a plain or raw string, its escapes undone; none for a byte or C string.
fn decode(lit: &[u8]) -> Option<String> {
    let s = std::str::from_utf8(lit).ok()?;
    if let Some(raw) = s.strip_prefix('r') {
        let hashes = raw.len() - raw.trim_start_matches('#').len();
        let inner = raw.get(hashes + 1..raw.len().checked_sub(hashes + 1)?)?;
        return Some(inner.to_owned());
    }
    let inner = s.strip_prefix('"')?.strip_suffix('"')?;
    let mut out = String::new();
    let mut chars = inner.chars().peekable();
    while let Some(c) = chars.next() {
        if c != '\\' {
            out.push(c);
            continue;
        }
        match chars.next()? {
            'n' => out.push('\n'),
            't' => out.push('\t'),
            'r' => out.push('\r'),
            '0' => out.push('\0'),
            '\\' => out.push('\\'),
            '\'' => out.push('\''),
            '"' => out.push('"'),
            'x' => {
                let hex: String = [chars.next()?, chars.next()?].iter().collect();
                out.push(char::from(u8::from_str_radix(&hex, 16).ok()?));
            }
            'u' => {
                if chars.next()? != '{' {
                    return None;
                }
                let hex: String = chars
                    .by_ref()
                    .take_while(|&c| c != '}')
                    .filter(|&c| c != '_')
                    .collect();
                out.push(char::from_u32(u32::from_str_radix(&hex, 16).ok()?)?);
            }
            // A line continuation: the newline and the whitespace after it are dropped.
            '\n' => {
                while chars.peek().is_some_and(|c| c.is_whitespace()) {
                    chars.next();
                }
            }
            _ => return None,
        }
    }
    Some(out)
}

/// A crate root as the tree holds it: cargo's metadata names a target's root as its manifest's directory joined with
/// the target's `path`, `..` kept (`crates/r/../c/src/lib.rs`); none for one out of the tree's text.
pub(crate) fn normalized(path: &str) -> Option<String> {
    resolve("", path).ok().flatten()
}

/// `rel` resolved against the directory `from` of the written tree, lexically: none for an absolute path or one that
/// climbs out of the tree, which only the disk can place.
fn resolve(from: &str, rel: &str) -> Result<Option<String>, &'static str> {
    if rel.starts_with('/') {
        return Ok(None);
    }
    if rel.contains('\\') || rel.contains(':') {
        return Err(
            "names a path with a backslash or a drive, which the census cannot read on every host",
        );
    }
    let mut parts: Vec<&str> = from.split('/').filter(|p| !p.is_empty()).collect();
    for p in rel.split('/') {
        match p {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return Ok(None);
                }
            }
            p => parts.push(p),
        }
    }
    Ok(Some(parts.join("/")))
}

/// Whether `path`, or a directory it lies in, is one of `links`.
fn through_link(path: &str, links: &BTreeSet<String>) -> bool {
    let mut prefix = String::new();
    for part in path.split('/') {
        if !prefix.is_empty() {
            prefix.push('/');
        }
        prefix.push_str(part);
        if links.contains(&prefix) {
            return true;
        }
    }
    false
}

/// The directory a `path`-loaded module's own `mod` children lie under: its file's, as the pin resolves them (the module
/// documentation's measurement).
fn children_of_path_module(path: &str) -> Vec<String> {
    vec![parent(path)]
}

/// A non-mod-rs file's module name: none for the crate root and a `mod.rs`.
fn non_mod_rs_stem(file: &str, root: &str) -> Option<String> {
    let name = file.rsplit('/').next().unwrap_or(file);
    (file != root && name != "mod.rs").then(|| name.strip_suffix(".rs").unwrap_or(name).to_owned())
}

fn parent(path: &str) -> String {
    path.rsplit_once('/')
        .map_or(String::new(), |(d, _)| d.to_owned())
}

fn join(dir: &str, part: &str) -> String {
    if dir.is_empty() {
        part.to_owned()
    } else {
        format!("{dir}/{part}")
    }
}

/// Every directory `parts`' alternatives make under `dir`, one component each, outermost first.
fn expansions(dir: &str, parts: &[Vec<String>]) -> Vec<String> {
    parts
        .iter()
        .fold(vec![dir.to_owned()], |dirs, alternatives| {
            dirs.iter()
                .flat_map(|d| alternatives.iter().map(move |a| join(d, a)))
                .collect()
        })
}

fn line_of(source: &[u8], at: usize) -> usize {
    source[..at.min(source.len())]
        .iter()
        .filter(|&&b| b == b'\n')
        .count()
        + 1
}

#[cfg(test)]
#[path = "trust_census_oracle.rs"]
mod oracle;

#[cfg(test)]
mod tests {
    use super::{reach, Reach};
    use archogen_catalog::tree::Tree;
    use std::collections::BTreeSet;

    fn tree(files: &[(&str, &str)]) -> Tree {
        Tree::new(
            files
                .iter()
                .map(|(p, t)| ((*p).to_owned(), t.as_bytes().to_vec())),
        )
    }

    fn of(files: &[(&str, &str)], root: &str) -> Reach {
        reach(&tree(files), &BTreeSet::new(), root, &|_, _| None)
    }

    fn files(r: &Reach) -> Vec<&str> {
        r.files.iter().map(String::as_str).collect()
    }

    fn whys(r: &Reach) -> Vec<(String, usize, String)> {
        r.unfollowed
            .iter()
            .map(|u| (u.file.clone(), u.line, u.why.clone()))
            .collect()
    }

    #[test]
    fn a_module_s_file_mirrors_its_module_path() {
        // The reference's table: `lib.rs`'s `mod util;` is `util.rs` or `util/mod.rs`; `util.rs`'s `mod config;` is
        // `util/config.rs`; a file no module names is not reached.
        let r = of(
            &[
                ("p/src/lib.rs", "mod util;\nmod other;\n"),
                ("p/src/util.rs", "mod config;\n"),
                ("p/src/util/config.rs", ""),
                ("p/src/other/mod.rs", "mod deep;\n"),
                ("p/src/other/deep.rs", ""),
                ("p/src/unreached.rs", ""),
            ],
            "p/src/lib.rs",
        );
        assert_eq!(
            files(&r),
            [
                "p/src/lib.rs",
                "p/src/other/deep.rs",
                "p/src/other/mod.rs",
                "p/src/util.rs",
                "p/src/util/config.rs"
            ]
        );
        assert!(r.unfollowed.is_empty());
    }

    #[test]
    fn a_path_attribute_resolves_as_the_reference_says() {
        // The reference's tables: outside inline modules relative to the file's directory; inside them, with the inline
        // components, after a non-mod-rs file's own name; an inline module's `path` its children's directory.
        let r = of(
            &[
                ("p/src/lib.rs", "mod a;\n"),
                ("p/src/a.rs", "mod b;\n"),
                ("p/src/a/b.rs", "#[path = \"foo.rs\"]\nmod c;\nmod inline {\n    #[path = \"other.rs\"]\n    mod inner;\n}\n"),
                ("p/src/a/foo.rs", ""),
                ("p/src/a/b/inline/other.rs", ""),
                ("q/src/lib.rs", "#[path = \"thread_files\"]\nmod thread {\n    #[path = \"tls.rs\"]\n    mod local_data;\n}\n"),
                ("q/src/thread_files/tls.rs", ""),
            ],
            "p/src/lib.rs",
        );
        for f in ["p/src/a/foo.rs", "p/src/a/b/inline/other.rs"] {
            assert!(r.files.contains(f), "{f}: {:?}", files(&r));
        }
        let q = of(
            &[
                ("q/src/lib.rs", "#[path = \"thread_files\"]\nmod thread {\n    #[path = \"tls.rs\"]\n    mod local_data;\n}\n"),
                ("q/src/thread_files/tls.rs", ""),
            ],
            "q/src/lib.rs",
        );
        assert!(
            q.files.contains("q/src/thread_files/tls.rs"),
            "{:?}",
            files(&q)
        );
    }

    #[test]
    fn every_spelling_of_an_inclusion_is_followed_whatever_its_cfg() {
        // Rounds 8 and 10: a delimiter, an escape, a raw string, a `cfg_attr`, a `cfg` off in every build the census
        // could make — each names its file, which the census follows into another package.
        for (case, source) in [
            (
                "parentheses",
                "const T: &str = include_str!(\"../../c/src/lib.rs\");\n",
            ),
            (
                "brackets",
                "const T: &str = include_str![\"../../c/src/lib.rs\"];\n",
            ),
            (
                "braces",
                "const T: &str = include_str!{\"../../c/src/lib.rs\"};\n",
            ),
            (
                "escaped",
                "const T: &str = include_str!(\"../../\\x63/src/lib.rs\");\n",
            ),
            (
                "raw",
                "const T: &str = include_str!(r#\"../../c/src/lib.rs\"#);\n",
            ),
            (
                "a path module",
                "#[path = \"../../c/src/lib.rs\"]\nmod checker;\n",
            ),
            (
                "cfg_attr",
                "#[cfg_attr(test, path = \"../../c/src/lib.rs\")]\nmod checker;\n",
            ),
            (
                "a cfg off",
                "#[cfg(not(debug_assertions))]\n#[path = \"../../c/src/lib.rs\"]\nmod checker;\n",
            ),
            (
                "doctest cfg",
                "#[cfg(doctest)]\n#[doc = include_str!(\"../../c/src/lib.rs\")]\npub struct R;\n",
            ),
            ("include", "include!(\"../../c/src/lib.rs\");\n"),
            (
                "a qualified path",
                "const T: &[u8] = ::core::include_bytes!(\"../../c/src/lib.rs\");\n",
            ),
            (
                "trailing comma",
                "const T: &str = include_str!(\"../../c/src/lib.rs\",);\n",
            ),
        ] {
            let r = of(
                &[
                    ("r/tests/t.rs", source),
                    ("c/src/lib.rs", "pub fn x() {}\n"),
                ],
                "r/tests/t.rs",
            );
            assert!(r.files.contains("c/src/lib.rs"), "{case}: {:?}", files(&r));
            assert!(r.unfollowed.is_empty(), "{case}: {:?}", whys(&r));
        }
    }

    #[test]
    fn a_site_no_literal_pins_is_unfollowed() {
        // Default-deny: what the census cannot follow by its text is returned, for the inventory to refuse unless an
        // admission names it.
        for (case, source, why) in [
            (
                "concat",
                "const T: &str = include_str!(concat!(\"../\", \"x\"));\n",
                "no single string literal",
            ),
            (
                "env",
                "const T: &str = include_str!(env!(\"X\"));\n",
                "no single string literal",
            ),
            (
                "an alias",
                "use std::include_str as inc;\n",
                "imports or shadows",
            ),
            (
                "a macro of that name",
                "macro_rules! include_str { () => {} }\n",
                "imports or shadows",
            ),
            (
                "inside a macro",
                "macro_rules! m { ($p:literal) => { include_str!($p) }; }\n",
                "inside a macro",
            ),
            (
                "an assembled attribute",
                "macro_rules! m { ($b:tt) => { # $b mod evil; }; }\n",
                "a macro assembles",
            ),
            (
                "a declared module",
                "macro_rules! m { ($n:ident) => { mod $n; }; }\n",
                "a macro declares",
            ),
            (
                "a doc test",
                "/// ```\n/// include!(\"../../c/src/lib.rs\");\n/// ```\npub fn d() {}\n",
                "doc test",
            ),
            (
                "a doc attribute's doc test",
                "#[doc = \"```\\ninclude!(\\\"x\\\");\\n```\"]\npub fn d() {}\n",
                "doc test",
            ),
        ] {
            let r = of(&[("r/tests/t.rs", source)], "r/tests/t.rs");
            assert!(
                r.unfollowed.iter().any(|u| u.why.contains(why)),
                "{case}: {:?}",
                whys(&r)
            );
        }
        // A name that only spells an inclusion macro — a variable, a field — is none.
        let r = of(
            &[(
                "r/tests/t.rs",
                "fn f(include: bool) -> bool { let include_str = include; include_str }\n",
            )],
            "r/tests/t.rs",
        );
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
        // A doc block rustdoc does not compile is not read.
        let r = of(
            &[(
                "r/src/lib.rs",
                "/// ```text\n/// include!(\"x\");\n/// ```\npub fn d() {}\n",
            )],
            "r/src/lib.rs",
        );
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
    }

    #[test]
    fn a_document_s_doc_tests_and_a_link_and_an_unreadable_file() {
        // A document a doc attribute includes is reached, and its doc tests' sites unfollowed; a symbolic link is not
        // read through; a reached file that does not tokenize is unfollowed, an unreached one not read.
        let r = of(
            &[
                (
                    "r/src/lib.rs",
                    "#![doc = include_str!(\"../README.md\")]\nmod bad;\n",
                ),
                (
                    "r/README.md",
                    "Text.\n\n```\ninclude!(\"../c/src/lib.rs\");\n```\n",
                ),
                ("r/src/bad.rs", "fn f() { /* unclosed\n"),
                ("r/src/unreached.rs", "fn f() { /* unclosed\n"),
            ],
            "r/src/lib.rs",
        );
        assert!(r.files.contains("r/README.md"), "{:?}", files(&r));
        let w = whys(&r);
        assert!(
            w.iter()
                .any(|(f, l, why)| f == "r/README.md" && *l == 4 && why.contains("doc test")),
            "{w:?}"
        );
        assert!(
            w.iter()
                .any(|(f, _, why)| f == "r/src/bad.rs" && why.contains("does not tokenize")),
            "{w:?}"
        );
        assert!(
            !w.iter().any(|(f, _, _)| f == "r/src/unreached.rs"),
            "{w:?}"
        );
        let links: BTreeSet<String> = ["r/link".to_owned()].into();
        let t = tree(&[("r/tests/t.rs", "#[path = \"../link/lib.rs\"]\nmod l;\n")]);
        let r = reach(&t, &links, "r/tests/t.rs", &|_, _| None);
        assert!(
            r.unfollowed.iter().any(|u| u.why.contains("symbolic link")),
            "{:?}",
            whys(&r)
        );
    }

    #[test]
    fn a_path_out_of_the_tree_s_text_is_followed_where_it_lands() {
        // An absolute path or one climbing out of the tree names a file only the disk can place: followed when it lands
        // back inside the written tree, nothing otherwise, since no file outside the tree is a role package's.
        let sources = [
            (
                "r/tests/t.rs",
                "#[path = \"../../../tree/c/src/lib.rs\"]\nmod a;\n#[path = \"/elsewhere/x.rs\"]\nmod b;\n",
            ),
            ("c/src/lib.rs", ""),
        ];
        let inside = |from: &str, rel: &str| {
            (from == "r/tests" && rel == "../../../tree/c/src/lib.rs")
                .then(|| "c/src/lib.rs".to_owned())
        };
        let r = reach(&tree(&sources), &BTreeSet::new(), "r/tests/t.rs", &inside);
        assert_eq!(files(&r), ["c/src/lib.rs", "r/tests/t.rs"]);
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
        let r = reach(
            &tree(&sources),
            &BTreeSet::new(),
            "r/tests/t.rs",
            &|_, _| None,
        );
        assert_eq!(files(&r), ["r/tests/t.rs"]);
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
    }

    #[test]
    fn a_file_reached_through_cfg_test_alone_counts_for_its_crate_s_tests_alone() {
        // `cfg(test)` is set only on the crate a test harness builds: what an item under exactly `#[cfg(test)]`, or a
        // file under `#![cfg(test)]`, reaches counts for its own test units, never for it as a dependency or for its
        // non-test units; any other `cfg` is read as on.
        let r = of(
            &[
                (
                    "p/src/lib.rs",
                    "#[cfg(test)]\nmod tests;\n#[cfg(test)]\nconst T: &str = include_str!(\"../../c/a.txt\");\n\
                     #[cfg(any(test, feature = \"x\"))]\nconst U: &str = include_str!(\"../../c/b.txt\");\nmod inner;\n",
                ),
                ("p/src/tests.rs", "const V: &str = include_str!(\"../../c/c.txt\");\n"),
                ("p/src/inner.rs", "#![cfg(test)]\nconst W: &str = include_str!(\"../../c/d.txt\");\n"),
                ("c/a.txt", ""),
                ("c/b.txt", ""),
                ("c/c.txt", ""),
                ("c/d.txt", ""),
            ],
            "p/src/lib.rs",
        );
        for f in ["c/a.txt", "c/b.txt", "c/c.txt", "c/d.txt", "p/src/tests.rs"] {
            assert!(r.files.contains(f), "{f}: {:?}", files(&r));
        }
        let nontest: Vec<&str> = r.nontest.iter().map(String::as_str).collect();
        assert_eq!(nontest, ["c/b.txt", "p/src/inner.rs", "p/src/lib.rs"]);
    }

    #[test]
    fn every_path_attribute_a_module_carries_is_followed() {
        // The oracle's first finding: two `cfg_attr` paths on one module, each another configuration's — each followed,
        // and an inline module's alternatives each a directory its children may lie under.
        let r = of(
            &[
                (
                    "p/src/lib.rs",
                    "#[cfg_attr(not(test), path = \"ca.rs\")]\n#[cfg_attr(test, path = \"cb.rs\")]\nmod ca;\n\
                     #[cfg_attr(test, path = \"x\")]\n#[cfg_attr(not(test), path = \"y\")]\nmod m {\n    mod z;\n}\n",
                ),
                ("p/src/ca.rs", ""),
                ("p/src/cb.rs", ""),
                ("p/src/x/z.rs", ""),
                ("p/src/y/z.rs", ""),
            ],
            "p/src/lib.rs",
        );
        for f in ["p/src/ca.rs", "p/src/cb.rs", "p/src/x/z.rs", "p/src/y/z.rs"] {
            assert!(r.files.contains(f), "{f}: {:?}", files(&r));
        }
    }
}
