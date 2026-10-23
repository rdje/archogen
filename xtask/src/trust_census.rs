//! What a target compiles, read from its sources with nothing built (`docs/specs/trust/decision_trust-role-programs.md`
//! §3; round 10's change of method): from its crate root, every file `mod`, `#[path]` and the `include!` family reach,
//! whatever their `cfg`, so no profile, feature, flag or host decides it. Every token that can make rustc read a file
//! — a `mod`, an inclusion or assembly macro, a `path`, `doc` or `debugger_visualizer` attribute — is read whole or
//! returned unfollowed, for the inventory to refuse unless an admission names it, so a spelling the census does not
//! know is a refusal, never a miss (round 11's change of method): no single string literal, an alias or a macro's, a
//! module a macro's expansion places, an assembly directive reading a file, a path out of the tree, through a symbolic
//! link or spelt as hosts read apart, a file that does not tokenize, a doc test's own. And it reads only what it can
//! check it reads as rustc does (round 12's change of method): outside comments and literals, a file's every byte is
//! printable ASCII or a space, a tab, a form feed, a newline or a carriage return — the whitespace rustc and the
//! catalog's lexer both skip — and a file holding any other there, or opening with a shebang line rustc would remove,
//! is refused whole, so no Unicode space, control byte or byte-order mark can part what the census reads from what
//! rustc does.
//!
//! The Rust reference of the pin decides where a module's file is: "When the module does not have a `path` attribute,
//! the path to the file mirrors the logical module path"; "For `path` attributes on modules not inside inline module
//! blocks, the file path is relative to the directory the source file is located"; inside inline module blocks it is
//! relative to the mod-rs file's directory "including the inline module components as directories", and for a
//! non-mod-rs file "the path starts with a directory with the name of the non-mod-rs module". Where the reference is
//! silent — the children of a module a `path` attribute loads, the `mod` items of a file an `include!` brings in — the
//! pin decides, measured `2026-10-23` by its own refusal of a missing child: "create file \"src/pl/child.rs\" or
//! \"src/pl/child/mod.rs\"" for `src/pl/loaded.rs`, "\"src/inc/c.rs\"" for `src/inc/i.rs` — each under the loaded
//! file's own directory, which the census's oracle holds on every build of its corpus. A `debugger_visualizer` file is
//! "a path relative to the source file".

use std::collections::{BTreeMap, BTreeSet};

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
    /// Those it reaches through no module under `#[cfg(test)]`: what it compiles when built for another crate, or for
    /// itself as no test — `cfg(test)` is set only on the crate a test harness builds, never on a dependency.
    pub nontest: BTreeSet<String>,
    /// Every site it cannot follow.
    pub unfollowed: BTreeSet<Unfollowed>,
}

const OUTSIDE: &str =
    "names a path outside the commit's tree — absolute, or climbing out of it — whose file the census cannot read";
const HOSTS: &str = "names a path hosts read apart — a backslash, a drive, a component ending in a dot or a space, \
                     or a byte outside ASCII";
const CASE: &str =
    "names a file the commit holds only in another case, or in two, which a case-insensitive host reads as one";
const LINK: &str = "reaches a symbolic link, whose target the census does not read";

/// The commit's sources as the census reads them: its tree, its symbolic links, and each blob's path folded to ASCII
/// lower case, so a name only a case-insensitive host resolves is known.
pub(crate) struct Sources<'a> {
    tree: &'a Tree,
    links: &'a BTreeSet<String>,
    folded: BTreeMap<String, usize>,
}

impl<'a> Sources<'a> {
    pub(crate) fn new(tree: &'a Tree, links: &'a BTreeSet<String>) -> Self {
        let mut folded: BTreeMap<String, usize> = BTreeMap::new();
        for path in tree.paths() {
            *folded.entry(path.to_ascii_lowercase()).or_default() += 1;
        }
        Self {
            tree,
            links,
            folded,
        }
    }

    /// The blob `rel` names from the directory `from`; none for a file the commit holds in no case, which no build of
    /// it can read.
    fn file(&self, from: &str, rel: &str) -> Result<Option<String>, &'static str> {
        let path = resolve(from, rel, self.links)?;
        let folds = self
            .folded
            .get(&path.to_ascii_lowercase())
            .copied()
            .unwrap_or(0);
        match (self.tree.is_file(&path), folds) {
            (true, 1) => Ok(Some(path)),
            (false, 0) => Ok(None),
            _ => Err(CASE),
        }
    }

    /// A target's crate root as the tree holds it — cargo's metadata names it as its manifest's directory joined with
    /// the target's `path`, `..` kept (`crates/r/../c/src/lib.rs`) — or why the census cannot read it.
    pub(crate) fn root(&self, src: &str) -> Result<String, &'static str> {
        self.file("", src)?.ok_or("is no file of the commit")
    }

    /// Every file the crate root `root` reaches, its doc text read for triggers when rustdoc tests it, a library's
    /// (`doc_tested`; round 11's R9) — the `doctest` field "is only relevant for libraries, it has no effect on other
    /// sections".
    pub(crate) fn reach(&self, root: &str, doc_tested: bool) -> Reach {
        let mut out = Reach::default();
        let mut doc = DocText::default();
        // Each file to read, with the directories its `mod name;` children may lie under, whether it is reached only
        // through a module under `#[cfg(test)]`, and whether it is loaded as a module, so its own leading
        // `#![cfg(test)]` holds.
        let mut seen: BTreeSet<(String, Vec<String>, bool, bool)> = BTreeSet::new();
        let mut todo: Vec<(String, Vec<String>, bool, bool)> =
            vec![(root.to_owned(), vec![parent(root)], false, true)];
        while let Some(entry) = todo.pop() {
            if !seen.insert(entry.clone()) {
                continue;
            }
            let (file, bases, test_only, module) = entry;
            let Some(source) = self.tree.get(&file) else {
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
                        why: format!("{why}, so the census cannot read what it includes"),
                    });
                    continue;
                }
            };
            // A module file whose leading inner attributes hold `#![cfg(test)]` is read by every build declaring it, but
            // what it reaches by a test's alone.
            let test_only = test_only || (module && scan.test_file);
            out.unfollowed.extend(scan.unfollowed);
            if doc_tested {
                doc.read(&file, &scan.doc);
            }
            let dir = parent(&file);
            let follow =
                |rel: &str, from: &str, site: &Site| -> Result<Option<String>, Unfollowed> {
                    self.file(from, rel)
                        .map_err(|why| site.unfollowed(&file, why))
                };
            let found = |r: Result<Option<String>, Unfollowed>, out: &mut Reach| match r {
                Err(u) => {
                    out.unfollowed.insert(u);
                    None
                }
                Ok(path) => path,
            };
            for m in &scan.modules {
                let test_only = test_only || m.test_only;
                // `mod name;`: its file mirrors its module path, under each directory this file's children may lie in —
                // read beside any `path` attribute too, since a `cfg_attr`'s holds only where its `cfg` does, and rustc
                // reads the module's own file elsewhere (round 12's D2).
                for dir in bases.iter().flat_map(|base| expansions(base, &m.inline)) {
                    for candidate in [format!("{}.rs", m.name), format!("{}/mod.rs", m.name)] {
                        if let Some(path) = found(follow(&candidate, &dir, &m.site), &mut out) {
                            todo.push((path, vec![join(&dir, &m.name)], test_only, true));
                        }
                    }
                }
                // `#[path = "…"] mod name;`: relative to this file's directory, or inside inline modules to it with the
                // inline components — after the non-mod-rs file's own name, when it is one. Each `path` attribute the
                // item carries, `cfg_attr`'s in every configuration, is followed.
                match m.paths.as_slice() {
                    [] => {}
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
                        for (from, rel) in
                            froms.iter().flat_map(|f| rels.iter().map(move |r| (f, r)))
                        {
                            if let Some(path) = found(follow(rel, from, &m.site), &mut out) {
                                let children = children_of_path_module(&path);
                                todo.push((path, children, test_only, true));
                            }
                        }
                    }
                }
            }
            for inc in &scan.includes {
                let test_only = test_only || inc.test_only;
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
                            // resolves them (the module documentation's measurement); no module, its own
                            // `#![cfg(test)]` is none.
                            What::Rust => {
                                let children = vec![parent(&path)];
                                todo.push((path, children, test_only, false));
                            }
                            What::Data => {
                                if !test_only {
                                    out.nontest.insert(path.clone());
                                }
                                out.files.insert(path);
                            }
                            // A document a doc attribute includes: doc text of the library, read whole.
                            What::Doc => {
                                if let Some(text) = self.tree.get(&path).filter(|_| doc_tested) {
                                    let whole = Fragment {
                                        at: 0,
                                        line: 1,
                                        text: String::from_utf8_lossy(text).into_owned(),
                                    };
                                    doc.read(&path, &[whole]);
                                }
                                if !test_only {
                                    out.nontest.insert(path.clone());
                                }
                                out.files.insert(path);
                            }
                        }
                    }
                }
            }
        }
        out.unfollowed.extend(doc.refused());
        out
    }
}

const DOC_TEST: &str = "is a word in doc text that can make a doc test read a file, prose or code alike — no markdown \
                        is read — which the census does not follow";

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
    /// Whether it lies in a module under `#[cfg(test)]`.
    test_only: bool,
    site: Site,
}

/// What an inclusion brings in.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum What {
    /// `include!`: Rust.
    Rust,
    /// `include_str!`, `include_bytes!` or a `debugger_visualizer` file: data.
    Data,
    /// `include_str!` inside a `doc` attribute: a document whose code blocks are doc tests.
    Doc,
}

/// An `include!`, `include_str!` or `include_bytes!` invocation, or a `debugger_visualizer` file.
#[derive(Debug)]
struct Include {
    what: What,
    /// Its one string literal, decoded; none when it is no single string literal.
    literal: Option<String>,
    /// Whether it lies in a module under `#[cfg(test)]`.
    test_only: bool,
    site: Site,
}

/// One file's sites.
#[derive(Debug, Default)]
struct Scan {
    modules: Vec<Module>,
    includes: Vec<Include>,
    unfollowed: Vec<Unfollowed>,
    /// Its doc text: every doc comment and `doc` literal.
    doc: Vec<Fragment>,
    /// Whether its leading inner attributes hold exactly `#![cfg(test)]`.
    test_file: bool,
}

const INCLUDES: [&str; 3] = ["include", "include_str", "include_bytes"];

/// The assembly macros, whose templates the assembler reads, directives and all.
const ASM: [&str; 3] = ["asm", "global_asm", "naked_asm"];

/// The assembler's directives that read a file — `.incbin`, `.include` — or assemble one from parts, which no reading
/// of the template can follow; matched in any case, anywhere in it.
const DIRECTIVES: [&str; 6] = ["incbin", "include", ".macro", ".irp", ".rept", ".altmacro"];

/// Rust's keywords, strict and reserved: one before `!` names no macro (`if !(…)`); a raw one does.
const KEYWORDS: &[&str] = &[
    "as", "async", "await", "break", "const", "continue", "crate", "dyn", "else", "enum", "extern",
    "false", "fn", "for", "if", "impl", "in", "let", "loop", "match", "mod", "move", "mut", "pub",
    "ref", "return", "self", "Self", "static", "struct", "super", "trait", "true", "type",
    "unsafe", "use", "where", "while", "abstract", "become", "box", "do", "final", "gen", "macro",
    "override", "priv", "try", "typeof", "unsized", "virtual", "yield",
];

const ALIAS: &str =
    "imports or shadows an inclusion or assembly macro, so no literal pins what it reads";

/// An identifier's name, a raw one's without its `r#`.
fn name(t: &Token) -> Option<&str> {
    match &t.kind {
        Kind::Ident(n, _) => Some(n.strip_prefix("r#").unwrap_or(n)),
        _ => None,
    }
}

/// Whether `t` is the keyword `k`: a raw identifier of that name is none.
fn keyword(t: &Token, k: &str) -> bool {
    matches!(&t.kind, Kind::Ident(n, _) if n == k)
}

/// The lexer's tokens, each raw identifier one: it reads `r#type` as `r`, `#` and `type`, touching (round 11's D3).
fn merged(ts: Vec<Token>) -> Vec<Token> {
    let mut out: Vec<Token> = Vec::with_capacity(ts.len());
    let mut i = 0;
    while i < ts.len() {
        if let (Kind::Ident(r, _), Some(hash), Some(ident)) =
            (&ts[i].kind, ts.get(i + 1), ts.get(i + 2))
        {
            if let Kind::Ident(n, wide) = &ident.kind {
                if r == "r"
                    && hash.kind == Kind::Punct('#')
                    && ts[i].end == hash.at
                    && hash.end == ident.at
                {
                    out.push(Token {
                        kind: Kind::Ident(format!("r#{n}"), *wide),
                        at: ts[i].at,
                        end: ident.end,
                    });
                    i += 3;
                    continue;
                }
            }
        }
        out.push(ts[i].clone());
        i += 1;
    }
    out
}

/// Where `source` leaves the census's alphabet, and why, if it does (round 12's change of method). The catalog's lexer
/// skips a space, a tab, a form feed, a newline and a carriage return, and reads every other byte outside a comment or
/// a literal as part of a token: a byte outside ASCII joins an identifier, a control byte is a punctuation token. So
/// an identifier holding a byte outside ASCII — a byte-order mark, a Unicode space such as U+200E, or a name — or a
/// punctuation token outside printable ASCII is where rustc and the census may read the file apart, and refused; and so
/// is a first line `#!` that is no inner attribute, a shebang rustc removes before it reads.
fn outside_alphabet(source: &[u8], ts: &[Token]) -> Option<(usize, String)> {
    let inner_attribute = ts.len() >= 3
        && ts[0].at == 0
        && ts[1].kind == Kind::Punct('!')
        && ts[2].kind == Kind::Open('[');
    if source.starts_with(b"#!") && !inner_attribute {
        return Some((
            0,
            "opens with a shebang line, which rustc removes before it reads".to_owned(),
        ));
    }
    ts.iter().find_map(|t| match &t.kind {
        Kind::Ident(_, true) => Some((
            t.at,
            "holds a character outside ASCII outside a comment or a literal — a byte-order mark, a Unicode space, a \
             name — outside the census's alphabet"
                .to_owned(),
        )),
        Kind::Punct(c) if !(' '..='~').contains(c) => Some((
            t.at,
            format!("holds the byte {:#04x} outside a comment or a literal, outside the census's alphabet", *c as u32),
        )),
        _ => None,
    })
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

/// Where a token lies: in a `macro_rules!` body, whose expansion decides what its tokens name, or in another macro's
/// arguments, whose expansion may place, move or drop what they declare.
#[derive(Debug, Clone, Copy, Default)]
struct Region {
    body: bool,
    args: bool,
}

/// Each token's region: a body from `macro_rules! name`'s delimiter, arguments from any other `name!`'s.
fn regions(ts: &[Token]) -> Vec<Region> {
    let mut out = vec![Region::default(); ts.len()];
    let opens = |k: usize| ts.get(k).is_some_and(|t| matches!(t.kind, Kind::Open(_)));
    for (i, t) in ts.iter().enumerate() {
        let Kind::Ident(n, _) = &t.kind else {
            continue;
        };
        if KEYWORDS.contains(&n.as_str())
            || !ts.get(i + 1).is_some_and(|b| b.kind == Kind::Punct('!'))
        {
            continue;
        }
        let (open, body) = if name(t) == Some("macro_rules")
            && ts.get(i + 2).and_then(name).is_some()
            && opens(i + 3)
        {
            (i + 3, true)
        } else if opens(i + 2) {
            (i + 2, false)
        } else {
            continue;
        };
        let close = closing(ts, open);
        for r in out.iter_mut().take(close).skip(open + 1) {
            if body {
                r.body = true;
            } else {
                r.args = true;
            }
        }
    }
    out
}

/// The sites of one Rust source, its doc text's among them.
fn scan(file: &str, source: &[u8]) -> Result<Scan, (usize, String)> {
    let ts = tokens(source).map_err(|(at, why)| (at, format!("does not tokenize ({why})")))?;
    if let Some(outside) = outside_alphabet(source, &ts) {
        return Err(outside);
    }
    let ts = merged(ts);
    let regions = regions(&ts);
    let mut out = Scan {
        test_file: test_file(&ts),
        ..Scan::default()
    };
    let test_only = test_only_spans(&ts, &regions);
    let in_test = |k: usize| test_only.iter().any(|&(a, b)| a <= k && k <= b);
    let is = |k: usize, kind: Kind| ts.get(k).is_some_and(|t| t.kind == kind);
    let text =
        |a: usize, b: usize| String::from_utf8_lossy(&source[ts[a].at..ts[b].end]).into_owned();
    let site = |a: usize, b: usize| Site {
        line: line_of(source, ts[a].at),
        extent: text(a, b),
    };
    // Inline modules open: (the brace depth inside them, their directory component).
    let mut inline: Vec<(usize, Vec<String>)> = Vec::new();
    // Whether a `use` item is open, up to its `;`.
    let mut in_use = false;
    let mut depth = 0usize;
    // A `path` attribute seen, carried to the `mod` item it precedes: its value and its `#`'s index.
    let mut pending_path: Vec<(String, usize)> = Vec::new();
    let mut i = 0;
    while i < ts.len() {
        let region = regions[i];
        // An attribute, `#[…]` or `#![…]`, read whole: its `path`, its `doc` text, its `debugger_visualizer` file, its
        // inclusions.
        if ts[i].kind == Kind::Punct('#') {
            if region.body && is(i + 1, Kind::Punct('$')) {
                out.unfollowed.push(site(i, i + 1).unfollowed(
                    file,
                    "is an attribute a macro assembles, so the module it may load is decided by its expansion",
                ));
            }
            let bang = is(i + 1, Kind::Punct('!'));
            let open = if bang { i + 2 } else { i + 1 };
            if is(open, Kind::Open('[')) {
                let close = closing(&ts, open);
                let mut j = open + 1;
                while j < close {
                    let key = name(&ts[j]);
                    let assigned = is(j + 1, Kind::Punct('='));
                    let value = ts.get(j + 2).filter(|t| t.kind == Kind::Str);
                    if keyword(&ts[j], "mod") {
                        out.unfollowed.push(site(i, close).unfollowed(
                            file,
                            "is a `mod` inside an attribute, so the module it may declare is decided by the \
                             attribute's expansion",
                        ));
                    }
                    if key == Some("path") && assigned {
                        match value.and_then(|v| decode(&source[v.at..v.end])) {
                            // An inner `path` names its own module's directory, which the census does not apply
                            // (round 12's D3).
                            _ if bang => out.unfollowed.push(site(i, close).unfollowed(
                                file,
                                "is an inner `path` attribute, naming its own module's directory, which the census \
                                 does not apply",
                            )),
                            // Checked as it is read, an inline module's directory among them (round 12's D4).
                            Some(p) if !region.body => match path_spelling(&p) {
                                Some(why) => out.unfollowed.push(site(i, close).unfollowed(file, why)),
                                None => pending_path.push((p, i)),
                            },
                            _ => out.unfollowed.push(site(i, close).unfollowed(
                                file,
                                if region.body {
                                    "is a `path` attribute inside a macro, so the file it names is decided by its \
                                     expansion"
                                } else {
                                    "is a `path` attribute whose value is no single string literal"
                                },
                            )),
                        }
                    }
                    if key == Some("doc") && assigned {
                        match value {
                            Some(v) => match decode(&source[v.at..v.end]) {
                                Some(text) => out.doc.push(Fragment {
                                    at: v.at,
                                    line: line_of(source, v.at),
                                    text,
                                }),
                                // Never read as empty: what it holds is unknown (round 12's D5).
                                None => out.unfollowed.push(site(i, close).unfollowed(
                                    file,
                                    "is a `doc` literal the census cannot decode, so the doc tests it holds are unread",
                                )),
                            },
                            // `doc = include_str!("…")`: the inclusion, read below, is the document.
                            None if doc_included(&ts, j + 2, close) => {}
                            None => out.unfollowed.push(site(i, close).unfollowed(
                                file,
                                "is a `doc` attribute whose text no literal gives, so the doc tests it holds are \
                                 decided by its expansion",
                            )),
                        }
                    }
                    if matches!(key, Some("natvis_file" | "gdb_script_file")) && assigned {
                        match value.and_then(|v| decode(&source[v.at..v.end])) {
                            Some(p) if !region.body => out.includes.push(Include {
                                what: What::Data,
                                literal: Some(p),
                                test_only: in_test(j),
                                site: site(i, close),
                            }),
                            _ => out.unfollowed.push(site(i, close).unfollowed(
                                file,
                                "is a `debugger_visualizer` file no single literal names, or a macro's",
                            )),
                        }
                    }
                    if let Some(end) = inclusion(
                        &ts,
                        j,
                        source,
                        file,
                        region.body,
                        in_test(j),
                        &mut out,
                        &site,
                    ) {
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
        if keyword(&ts[i], "use") {
            // `use<…>` is a precise capture, which begins no item.
            in_use = in_use || !is(i + 1, Kind::Punct('<'));
        } else if keyword(&ts[i], "mod") {
            // A `mod` read whole, or refused: outside any macro, `mod name;` or `mod name { … }`; in a macro's body or
            // arguments only an inline module, which loads no file, the expansion deciding every other.
            let declared = ts
                .get(i + 1)
                .filter(|t| matches!(t.kind, Kind::Ident(..)))
                .and_then(name)
                .map(str::to_owned);
            let inline_module = declared.is_some() && is(i + 2, Kind::Open('{'));
            let paths = std::mem::take(&mut pending_path);
            let at = site(i, (i + 2).min(ts.len() - 1));
            if region.body {
                let metavariable = is(i + 1, Kind::Punct('$'))
                    && ts
                        .get(i + 2)
                        .is_some_and(|t| matches!(t.kind, Kind::Ident(..)))
                    && is(i + 3, Kind::Open('{'));
                if !(inline_module || metavariable) {
                    out.unfollowed.push(at.unfollowed(
                        file,
                        "is a module a macro declares, so its file is decided by its expansion",
                    ));
                }
            } else if region.args {
                if !inline_module {
                    out.unfollowed.push(at.unfollowed(
                        file,
                        "is a module in a macro's arguments, so its file is decided by the macro's expansion",
                    ));
                }
            } else {
                match (declared, inline_module) {
                    (Some(n), false) if is(i + 2, Kind::Punct(';')) => out.modules.push(Module {
                        site: site(paths.first().map_or(i, |(_, at)| *at), i + 2),
                        paths: paths.into_iter().map(|(p, _)| p).collect(),
                        inline: inline.iter().map(|(_, c)| c.clone()).collect(),
                        test_only: in_test(i),
                        name: n,
                    }),
                    // Its brace is counted when the loop reaches it.
                    // Its directory each `path` attribute names, and its own name, which rustc takes where a `cfg_attr`'s
                    // `path` does not hold (round 12's D2).
                    (Some(n), true) => {
                        let mut alternatives: Vec<String> = paths.into_iter().map(|(p, _)| p).collect();
                        alternatives.push(n);
                        inline.push((depth + 1, alternatives));
                    }
                    _ => out.unfollowed.push(at.unfollowed(
                        file,
                        "is a `mod` the census cannot read whole, as `mod name;` or `mod name { … }`",
                    )),
                }
            }
        } else if let Some(n) = name(&ts[i]) {
            let shadows = n == "macro_rules"
                && is(i + 1, Kind::Punct('!'))
                && ts
                    .get(i + 2)
                    .and_then(name)
                    .is_some_and(|m| INCLUDES.contains(&m) || ASM.contains(&m));
            let imports = in_use
                && (INCLUDES.contains(&n)
                    || (ASM.contains(&n) && ts.get(i + 1).is_some_and(|t| keyword(t, "as"))));
            if shadows || imports {
                out.unfollowed
                    .push(site(i, (i + 2).min(ts.len() - 1)).unfollowed(file, ALIAS));
            } else if let Some(end) = inclusion(
                &ts,
                i,
                source,
                file,
                region.body,
                in_test(i),
                &mut out,
                &site,
            )
            .or_else(|| assembly(&ts, i, source, file, region.body, &mut out, &site))
            {
                i = end + 1;
                continue;
            }
        }
        i += 1;
    }
    out.doc.extend(doc_comments(source, &ts));
    Ok(out)
}

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

/// The assembly macro invoked at `i`, if one is: a template the census cannot read whole, or holding a directive that
/// reads a file or assembles one, refused (round 11's R10); its closing delimiter's index returned.
fn assembly(
    ts: &[Token],
    i: usize,
    source: &[u8],
    file: &str,
    in_macro: bool,
    out: &mut Scan,
    site: &dyn Fn(usize, usize) -> Site,
) -> Option<usize> {
    name(&ts[i]).filter(|n| ASM.contains(n))?;
    let invoked = ts.get(i + 1).is_some_and(|t| t.kind == Kind::Punct('!'))
        && ts
            .get(i + 2)
            .is_some_and(|t| matches!(t.kind, Kind::Open(_)));
    if !invoked {
        return None;
    }
    let close = closing(ts, i + 2);
    let inner = ts.get(i + 3..close).unwrap_or_default();
    let assembled = inner
        .windows(2)
        .any(|w| matches!(w[0].kind, Kind::Ident(..)) && w[1].kind == Kind::Punct('!'));
    let directive = inner.iter().filter(|t| t.kind == Kind::Str).any(|t| {
        decode(&source[t.at..t.end]).is_none_or(|s| {
            let s = s.to_ascii_lowercase();
            DIRECTIVES.iter().any(|d| s.contains(d))
        })
    });
    let why = if in_macro {
        Some("is an assembly macro inside a macro, so its template is decided by its expansion")
    } else if assembled {
        Some("is an assembly template a macro assembles, so the census cannot read its directives")
    } else if directive {
        Some("is an assembly template holding a directive that reads a file or assembles one")
    } else {
        None
    };
    if let Some(why) = why {
        out.unfollowed.push(site(i, close).unfollowed(file, why));
    }
    Some(close)
}

/// Whether the tokens from `k` up to the attribute's `close` are one `include_str!(…)`, a path to it allowed: a `doc`
/// attribute's value read as a document.
fn doc_included(ts: &[Token], k: usize, close: usize) -> bool {
    let colons = |at: usize| {
        ts.get(at).is_some_and(|t| t.kind == Kind::Punct(':'))
            && ts.get(at + 1).is_some_and(|t| t.kind == Kind::Punct(':'))
    };
    let mut at = k;
    if colons(at) {
        at += 2;
    }
    while ts
        .get(at)
        .is_some_and(|t| matches!(t.kind, Kind::Ident(..)))
        && colons(at + 1)
    {
        at += 3;
    }
    let invoked = ts.get(at).and_then(name) == Some("include_str")
        && ts.get(at + 1).is_some_and(|t| t.kind == Kind::Punct('!'))
        && ts
            .get(at + 2)
            .is_some_and(|t| matches!(t.kind, Kind::Open(_)));
    if !invoked {
        return false;
    }
    let end = closing(ts, at + 2) + 1;
    end == close
        || ts
            .get(end)
            .is_some_and(|t| matches!(t.kind, Kind::Punct(',') | Kind::Close(_)))
}

/// Whether `#![cfg(test)]` or `#[cfg(test)]`, exactly, begins at `i`: `cfg` and `test` no raw identifiers.
fn cfg_test(ts: &[Token], i: usize, inner: bool) -> bool {
    let is = |k: usize, kind: Kind| ts.get(k).is_some_and(|t| t.kind == kind);
    let word = |k: usize, w: &str| ts.get(k).is_some_and(|t| keyword(t, w));
    let open = if inner { i + 2 } else { i + 1 };
    is(i, Kind::Punct('#'))
        && (!inner || is(i + 1, Kind::Punct('!')))
        && is(open, Kind::Open('['))
        && word(open + 1, "cfg")
        && is(open + 2, Kind::Open('('))
        && word(open + 3, "test")
        && is(open + 4, Kind::Close(')'))
        && is(open + 5, Kind::Close(']'))
}

/// Whether a file's leading inner attributes hold exactly `#![cfg(test)]`: a module file so marked compiles for its
/// crate's own test units alone, though every build declaring it reads the file.
fn test_file(ts: &[Token]) -> bool {
    let mut k = 0;
    while ts.get(k).is_some_and(|t| t.kind == Kind::Punct('#'))
        && ts.get(k + 1).is_some_and(|t| t.kind == Kind::Punct('!'))
        && ts.get(k + 2).is_some_and(|t| t.kind == Kind::Open('['))
    {
        if cfg_test(ts, k, true) {
            return true;
        }
        k = closing(ts, k + 2) + 1;
    }
    false
}

/// The token spans counted for their crate's own test units alone: a `mod` item under exactly `#[cfg(test)]`, outside
/// any macro — the attribute, then only attributes and a visibility before `mod name;` or `mod name { … }` — from the
/// attribute to the item's end. Nothing else narrows: `cfg(test)` on any other item, a field, a variant, a parameter,
/// an arm or a statement, or inside a macro, whose expansion may move it, is read as on, so the census over-counts
/// (round 11's D2).
fn test_only_spans(ts: &[Token], regions: &[Region]) -> Vec<(usize, usize)> {
    let is = |k: usize, kind: Kind| ts.get(k).is_some_and(|t| t.kind == kind);
    let mut out = Vec::new();
    let mut i = 0;
    while i < ts.len() {
        let outside = regions.get(i).is_some_and(|r| !r.body && !r.args);
        if !(outside && cfg_test(ts, i, false)) {
            i += 1;
            continue;
        }
        let mut k = i + 7;
        while is(k, Kind::Punct('#')) && is(k + 1, Kind::Open('[')) {
            k = closing(ts, k + 1) + 1;
        }
        if ts.get(k).is_some_and(|t| keyword(t, "pub")) {
            k += 1;
            if is(k, Kind::Open('(')) {
                k = closing(ts, k) + 1;
            }
        }
        let module = ts.get(k).is_some_and(|t| keyword(t, "mod"))
            && ts
                .get(k + 1)
                .is_some_and(|t| matches!(t.kind, Kind::Ident(..)));
        let end = if !module {
            None
        } else if is(k + 2, Kind::Punct(';')) {
            Some(k + 2)
        } else if is(k + 2, Kind::Open('{')) {
            Some(closing(ts, k + 2))
        } else {
            None
        };
        match end {
            Some(end) => {
                out.push((i, end));
                i = end + 1;
            }
            None => i += 1,
        }
    }
    out
}

/// Whether the invocation at `i` is a `doc` attribute's value: `doc =` before it, or before the path ending in it.
fn within_doc(ts: &[Token], i: usize) -> bool {
    let colons = |k: usize| {
        k >= 2 && ts[k - 1].kind == Kind::Punct(':') && ts[k - 2].kind == Kind::Punct(':')
    };
    let mut s = i;
    while colons(s) {
        if s >= 3 && matches!(ts[s - 3].kind, Kind::Ident(..)) {
            s -= 3;
        } else {
            s -= 2;
            break;
        }
    }
    s >= 2 && ts[s - 1].kind == Kind::Punct('=') && name(&ts[s - 2]) == Some("doc")
}

/// The doc comments of `source` — `///`, `//!`, `/** */` and `/*! */`, which the lexer drops — each a fragment of its
/// doc text, a block comment's to the `*/` closing it, nested ones within.
fn doc_comments(source: &[u8], ts: &[Token]) -> Vec<Fragment> {
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
                    out.push(Fragment {
                        at: a + k,
                        line: line_of(source, a + k),
                        text: String::from_utf8_lossy(&line[3..]).into_owned(),
                    });
                }
                k += end.max(1);
            } else if rest.starts_with(b"/*") {
                // Rust's block comments nest: this one ends at the `*/` closing its own `/*`.
                let mut depth = 0usize;
                let mut end = rest.len();
                let mut j = 0;
                while j + 1 < rest.len() {
                    if &rest[j..j + 2] == b"/*" {
                        depth += 1;
                        j += 2;
                    } else if &rest[j..j + 2] == b"*/" {
                        depth -= 1;
                        j += 2;
                        if depth == 0 {
                            end = j;
                            break;
                        }
                    } else {
                        j += 1;
                    }
                }
                let block = &rest[..end];
                let doc =
                    (block.starts_with(b"/**") && !block.starts_with(b"/***") && block != b"/**/")
                        || block.starts_with(b"/*!");
                if doc && block.len() >= 5 {
                    out.push(Fragment {
                        at: a + k,
                        line: line_of(source, a + k),
                        text: String::from_utf8_lossy(&block[3..block.len() - 2]).into_owned(),
                    });
                }
                k += end.max(1);
            } else {
                k += 1;
            }
        }
    }
    out
}

/// One piece of a file's doc text: a doc comment's body, a `doc` attribute's literal, or a document one includes.
#[derive(Debug, Clone)]
struct Fragment {
    /// Its first byte's offset in the source, which orders it among the file's others.
    at: usize,
    /// Its first line, 1-based.
    line: usize,
    text: String,
}

/// The words in doc text that can make a doc test read a file — an inclusion or assembly macro before `!`, an alias
/// or a comment, a `mod` before its name, a debugger visualizer; an assembler directive needs an assembly macro, or a
/// macro of the library's, whose definition is refused — each found by
/// its word alone, with no markdown read, so no fence, indentation or info string hides one; read past any whitespace
/// and any character outside printable ASCII, U+200E's and U+200F's among them (round 12's D6); and the pieces joined
/// in source order, as rustdoc joins an item's, so no split hides one, and only a word ending the whole is a trigger by
/// its place alone (round 12's R11).
/// One a backtick opens, with no quote or star between, is exempt: no compiled Rust holds a backtick outside a literal
/// or comment, so a doc test holding it there either holds the word inside that literal or comment, or cannot
/// compile — whatever a file it includes says, unless a `compile_fail` block names an error code (`exempt`).
#[derive(Debug, Default)]
struct DocText {
    /// Each trigger: its site, and whether a backtick opens it.
    triggers: Vec<(Unfollowed, bool)>,
    /// Whether any doc text read names an error code, `E` and four digits, as a `compile_fail` block's may.
    error_code: bool,
}

impl DocText {
    /// The triggers of `file`'s doc text, its pieces joined in source order by a newline.
    fn read(&mut self, file: &str, fragments: &[Fragment]) {
        let mut fragments: Vec<&Fragment> = fragments.iter().collect();
        fragments.sort_by_key(|f| f.at);
        // The joined text's lines, each with the source line it came from: a trigger's site is its line, the extent an
        // admission names.
        let lines: Vec<(usize, &str)> = fragments
            .iter()
            .flat_map(|f| f.text.split('\n').enumerate().map(|(k, l)| (f.line + k, l)))
            .collect();
        let text = lines.iter().map(|(_, l)| *l).collect::<Vec<_>>().join("\n");
        let text = text.as_str();
        let starts: Vec<usize> = std::iter::once(0)
            .chain(text.match_indices('\n').map(|(k, _)| k + 1))
            .collect();
        let bytes = text.as_bytes();
        let word = |b: u8| b.is_ascii_alphanumeric() || b == b'_';
        let mut k = 0;
        while k < bytes.len() {
            if !word(bytes[k]) {
                k += 1;
                continue;
            }
            let a = k;
            while k < bytes.len() && word(bytes[k]) {
                k += 1;
            }
            let w = &text[a..k];
            if w.len() == 5 && w.starts_with('E') && w[1..].bytes().all(|b| b.is_ascii_digit()) {
                self.error_code = true;
            }
            // Past what rustc may read as whitespace: the text's own, a block comment's stars, a control character and
            // any character outside ASCII.
            let skip =
                |c: char| c.is_whitespace() || c == '*' || c.is_ascii_control() || !c.is_ascii();
            let rest = text[k..].trim_start_matches(skip);
            let then_word = |x: &str| {
                rest.strip_prefix(x)
                    .is_some_and(|r| !r.bytes().next().is_some_and(word))
            };
            let trigger = match w {
                "include" | "include_str" | "include_bytes" | "asm" | "global_asm"
                | "naked_asm" => {
                    rest.is_empty()
                        || rest.starts_with('!')
                        || rest.starts_with("/*")
                        || rest.starts_with("//")
                        || then_word("as")
                }
                // Before anything that can begin a name: a letter, `_`, `$`, `r#`, or any character outside ASCII.
                "mod" => {
                    let next = text[k..].trim_start_matches(|c: char| {
                        c.is_ascii_whitespace() || c == '*' || c.is_ascii_control()
                    });
                    next.is_empty()
                        || next.starts_with(|c: char| {
                            c == '$' || c == '_' || c.is_ascii_alphabetic() || !c.is_ascii()
                        })
                }
                "debugger_visualizer" => true,
                _ => false,
            };
            if !trigger {
                continue;
            }
            let run = &text[text[..a].rfind(char::is_whitespace).map_or(0, |s| s + 1)..a];
            let exempt = run
                .rfind('`')
                .is_some_and(|bt| !run[bt + 1..].contains(['"', '\'', '*']));
            let row = starts.partition_point(|&s| s <= a) - 1;
            let (line, extent) = lines[row];
            let extent = extent.trim_end_matches('\r');
            self.triggers.push((
                Unfollowed {
                    file: file.to_owned(),
                    line,
                    sha256: Digest::of(extent.as_bytes()).hex(),
                    why: DOC_TEST.to_owned(),
                },
                exempt,
            ));
        }
    }

    /// The triggers refused: every one but those a backtick opens, unless the library's doc text names an error code.
    fn refused(self) -> impl Iterator<Item = Unfollowed> {
        let error_code = self.error_code;
        self.triggers
            .into_iter()
            .filter(move |(_, exempt)| error_code || !exempt)
            .map(|(u, _)| u)
    }
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
            // A line continuation: the newline and the whitespace after it are dropped — a CRLF is a newline to rustc,
            // which normalises it before it reads (round 12's D5).
            '\r' if chars.peek() == Some(&'\n') => {
                chars.next();
                while chars.peek().is_some_and(|c| c.is_whitespace()) {
                    chars.next();
                }
            }
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

/// Why a `path` attribute's value cannot name a file every host reads alike in the tree, if it cannot: an absolute path,
/// or one spelt with a backslash, a drive, a component ending in a dot or a space, or a byte outside ASCII. A path
/// climbing out of the tree is refused as it is resolved (`resolve`).
fn path_spelling(p: &str) -> Option<&'static str> {
    if p.starts_with('/') {
        return Some(OUTSIDE);
    }
    let apart = p.contains('\\')
        || p.contains(':')
        || p.split('/')
            .filter(|c| !matches!(*c, "" | "." | ".."))
            .any(|c| !c.is_ascii() || c.ends_with('.') || c.ends_with(' '));
    apart.then_some(HOSTS)
}

/// `rel` resolved against the directory `from` of the tree, lexically and component by component: each component
/// reached is checked against `links` before the next applies, so a link's `..` — its target's parent on disk — is
/// never read as the link's (round 11's D4). An absolute path, one climbing out of the tree, and a spelling hosts read
/// apart are refused: none names a file the census can place in the commit (round 11's R10, R16).
fn resolve(from: &str, rel: &str, links: &BTreeSet<String>) -> Result<String, &'static str> {
    if rel.starts_with('/') {
        return Err(OUTSIDE);
    }
    if rel.contains('\\') || rel.contains(':') {
        return Err(HOSTS);
    }
    let mut parts: Vec<&str> = Vec::new();
    for p in from
        .split('/')
        .filter(|p| !p.is_empty())
        .chain(rel.split('/'))
    {
        match p {
            "" | "." => {}
            ".." => {
                if parts.pop().is_none() {
                    return Err(OUTSIDE);
                }
            }
            p => {
                if !p.is_ascii() || p.ends_with('.') || p.ends_with(' ') {
                    return Err(HOSTS);
                }
                parts.push(p);
                if links.contains(&parts.join("/")) {
                    return Err(LINK);
                }
            }
        }
    }
    Ok(parts.join("/"))
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
    use super::{Reach, Sources};
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
        with_links(files, &[], root)
    }

    fn with_links(files: &[(&str, &str)], links: &[&str], root: &str) -> Reach {
        let links: BTreeSet<String> = links.iter().map(|l| (*l).to_owned()).collect();
        Sources::new(&tree(files), &links).reach(root, true)
    }

    /// What `root` reaches as a crate rustdoc does not test: a binary's, a test's.
    fn untested(files: &[(&str, &str)], root: &str) -> Reach {
        Sources::new(&tree(files), &BTreeSet::new()).reach(root, false)
    }

    fn refused(r: &Reach, why: &str) -> bool {
        r.unfollowed.iter().any(|u| u.why.contains(why))
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
        // Prose naming an inclusion in backticks is no doc test.
        let r = of(
            &[(
                "r/src/lib.rs",
                "/// The `include!` and `::core::include_str!` macros, a `mod name;`, an `.incbin`.\npub fn d() {}\n",
            )],
            "r/src/lib.rs",
        );
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
    }

    #[test]
    fn every_doc_test_rustdoc_may_compile_is_refused_however_it_is_spelt() {
        // Round 11's D1: rustdoc compiles a fenced block of backticks or tildes, of any length, "indented code blocks",
        // a block whose info string is `rust,foo`, `ignore` or `ignore-windows`, a test split over `#[doc]` attributes or
        // mixed with comments, a CRLF file, a nested block comment's — so no markdown is read: every trigger a doc-tested
        // library's doc text holds is refused, whatever the block around it, a `text` block's too.
        for (case, source) in [
            ("tildes", "/// ~~~\n/// include!(\"../../c/x.rs\");\n/// ~~~\npub fn d() {}\n"),
            ("four backticks", "/// ````\n/// include!(\"../../c/x.rs\");\n/// ````\npub fn d() {}\n"),
            ("indented", "/// Text.\n///\n///     include!(\"../../c/x.rs\");\npub fn d() {}\n"),
            ("rust,foo", "/// ```rust,foo\n/// include!(\"../../c/x.rs\");\n/// ```\npub fn d() {}\n"),
            ("ignore", "/// ```ignore\n/// include!(\"../../c/x.rs\");\n/// ```\npub fn d() {}\n"),
            ("ignore-windows", "/// ```ignore-windows\n/// include!(\"../../c/x.rs\");\n/// ```\npub fn d() {}\n"),
            ("text", "/// ```text\n/// include!(\"x\");\n/// ```\npub fn d() {}\n"),
            (
                "split over attributes",
                "#[doc = \"```\"]\n#[doc = \"include\"]\n#[doc = \"!(\\\"x\\\");\"]\n#[doc = \"```\"]\npub fn d() {}\n",
            ),
            ("comments and attributes", "/// ```\n#[doc = \"include!(\\\"x\\\");\"]\n/// ```\npub fn d() {}\n"),
            ("CRLF", "/// ```\r\n/// include!(\"x\");\r\n/// ```\r\npub fn d() {}\r\n"),
            ("a block comment", "/**\n * ```\n * include\n * !(\"x\");\n * ```\n */\npub fn d() {}\n"),
            ("a nested block comment", "/** a /* b */ ```\ninclude!(\"x\");\n``` */\npub fn d() {}\n"),
            ("an inner doc", "//! ```\n//! include_bytes!(\"x\");\n//! ```\n"),
            ("a module", "/// ```\n/// #[path = \"x.rs\"] mod m;\n/// fn main() {}\n/// ```\npub fn d() {}\n"),
            ("assembly", "/// ```\n/// core::arch::global_asm!(\".incbin \\\"x\\\"\");\n/// ```\npub fn d() {}\n"),
            ("an alias", "/// ```\n/// use std::include_str as s;\n/// ```\npub fn d() {}\n"),
            (
                "an include directive",
                "/// ```\n/// core::arch::global_asm!(\".include \\\"x.s\\\"\");\n/// ```\npub fn d() {}\n",
            ),
            (
                "an assembly macro alone",
                "/// ```\n/// core::arch::global_asm!(concat!(\".inc\", \"bin x\"));\n/// ```\npub fn d() {}\n",
            ),
            (
                "a comment before its `!`",
                "/// ```\n/// include/* c */!(\"x\");\n/// ```\npub fn d() {}\n",
            ),
            (
                "a fragment's end",
                "/// ```\n/// const T: &str = include_str\n/// !(\"x\");\n/// ```\npub fn d() {}\n",
            ),
            ("a visualizer", "/// ```\n/// #![debugger_visualizer(gdb_script_file = \"x.py\")]\n/// ```\npub fn d() {}\n"),
        ] {
            let r = of(&[("r/src/lib.rs", source)], "r/src/lib.rs");
            assert!(refused(&r, "doc test"), "{case}: {:?}", whys(&r));
        }
        // A document a `doc` attribute includes is the library's doc text too, an indented block's trigger refused there.
        let r = of(
            &[
                ("r/src/lib.rs", "#![doc = include_str!(\"../README.md\")]\n"),
                ("r/README.md", "Text.\n\n    include!(\"x\");\n"),
            ],
            "r/src/lib.rs",
        );
        assert!(
            r.unfollowed
                .iter()
                .any(|u| u.file == "r/README.md" && u.line == 3 && u.why.contains("doc test")),
            "{:?}",
            whys(&r)
        );
    }

    #[test]
    fn a_backtick_exempts_prose_unless_a_doc_test_names_an_error_code() {
        // No compiled Rust holds a backtick outside a literal or comment, so a word a backtick opens, with no quote or
        // star between, is inside one or follows a stray backtick, and its doc test cannot compile whatever a file it
        // includes says — unless a `compile_fail` block names an error code, which rustdoc may check: then no word is
        // exempt. A quote between reopens it; an English "include" without `!` is no trigger.
        let prose = "/// The `include!(\"x\")` macro, `std::include_str!`, these include the rest.\npub fn d() {}\n";
        assert!(of(&[("r/src/lib.rs", prose)], "r/src/lib.rs")
            .unfollowed
            .is_empty());
        let reopened = "/// ```\n/// let s = \"`\";include!(\"x\");\n/// ```\npub fn d() {}\n";
        let r = of(&[("r/src/lib.rs", reopened)], "r/src/lib.rs");
        assert!(refused(&r, "doc test"), "{:?}", whys(&r));
        let r = of(
            &[
                ("r/src/lib.rs", "mod m;\n"),
                (
                    "r/src/m.rs",
                    "/// ```compile_fail,E0308\n/// let x: u8 = \"\";\n/// ```\npub fn e() {}\n",
                ),
                ("r/src/other.rs", ""),
                ("r/README.md", ""),
            ],
            "r/src/lib.rs",
        );
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
        let r = of(
            &[
                ("r/src/lib.rs", &format!("mod m;\n{prose}")),
                (
                    "r/src/m.rs",
                    "/// ```compile_fail,E0308\n/// let x: u8 = \"\";\n/// ```\npub fn e() {}\n",
                ),
            ],
            "r/src/lib.rs",
        );
        // The prose's line, its two words one site, refused once a block names an error code.
        assert!(
            r.unfollowed
                .iter()
                .any(|u| u.file == "r/src/lib.rs" && u.line == 2 && u.why.contains("doc test")),
            "{:?}",
            whys(&r)
        );
    }

    #[test]
    fn the_census_reads_its_alphabet_alone_and_refuses_what_lies_outside() {
        // Round 12's D1, D5: rustc reads a vertical tab, U+200E and U+2028 as whitespace, removes a byte-order mark and
        // a shebang line, and normalises a CRLF; the catalog's lexer does none of these. So a file holding, outside a
        // comment or a literal, a byte outside printable ASCII and its four whitespace characters, or opening with a
        // shebang, is refused whole — never read apart from rustc. Inside a comment or a literal any character stays.
        let c = ("c/src/lib.rs", "pub fn x() {}\n");
        for (case, source, why) in [
            (
                "a vertical tab before `!`",
                "const T: &str = include_str\x0b!(\"../../c/src/lib.rs\");\n",
                "byte 0x0b",
            ),
            (
                "U+200E before `!`",
                "const T: &str = include_str\u{200e}!(\"../../c/src/lib.rs\");\n",
                "outside ASCII",
            ),
            (
                "U+2028 after `mod`",
                "#[path = \"../../c/src/lib.rs\"]\nmod\u{2028}r;\n",
                "outside ASCII",
            ),
            (
                "a vertical tab in an attribute",
                "#\x0b[path = \"../../c/src/lib.rs\"]\nmod lib;\n",
                "byte 0x0b",
            ),
            (
                "a byte-order mark",
                "\u{feff}include!(\"../../c/src/lib.rs\");\n",
                "outside ASCII",
            ),
            (
                "a shebang line",
                "#!/x \"\nconst T: &str = include_str!(\"../../c/src/lib.rs\"); // \"\n",
                "shebang",
            ),
        ] {
            let r = of(&[("r/tests/t.rs", source), c], "r/tests/t.rs");
            assert!(refused(&r, why), "{case}: {:?}", whys(&r));
            assert!(!r.files.contains("c/src/lib.rs"), "{case}: {:?}", files(&r));
        }
        // A character outside ASCII inside a comment or a literal, and an inner attribute first, are read as ever.
        let r = of(
            &[
                (
                    "r/tests/t.rs",
                    "#![allow(unused)] // — a comment\nconst S: &str = \"é\u{2028}\";\nconst T: &str = include_str!(\"../../c/src/lib.rs\");\n",
                ),
                c,
            ],
            "r/tests/t.rs",
        );
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
        assert!(r.files.contains("c/src/lib.rs"), "{:?}", files(&r));
        // `\` before a CRLF continues a literal's line, as before a newline.
        let r = of(
            &[
                (
                    "r/tests/t.rs",
                    "const T: &str = include_str!(\"../../c/\\\r\n    src/lib.rs\");\r\n",
                ),
                c,
            ],
            "r/tests/t.rs",
        );
        assert!(r.files.contains("c/src/lib.rs"), "{:?}", whys(&r));
        // A `doc` literal that does not decode is refused, never read as empty.
        let r = of(
            &[("r/src/lib.rs", "#[doc = \"\\q\"]\npub fn f() {}\n")],
            "r/src/lib.rs",
        );
        assert!(refused(&r, "cannot decode"), "{:?}", whys(&r));
    }

    #[test]
    fn doc_text_is_read_past_every_space_and_joined_so_prose_stays_prose() {
        // Round 12's D6: a trigger past U+200E or U+200F, and `mod` before a name outside ASCII, are triggers. R11: the
        // pieces joined in source order, a word ending a line is read with the next, so prose ending a line in
        // `include` or `asm`, or `include/exclude`, is no trigger — but a split over attributes still is.
        for (case, source) in [
            ("U+200E before `!`", "/// ```\n/// const _: &str = include_str\u{200e}!(\"x\");\n/// ```\npub fn d() {}\n"),
            ("U+200F after `mod`", "/// ```\n/// #[path = \"x.rs\"] mod\u{200f}x;\n/// fn main() {}\n/// ```\npub fn d() {}\n"),
            ("a name outside ASCII", "/// ```\n/// #[path = \"x.rs\"] mod é;\n/// fn main() {}\n/// ```\npub fn d() {}\n"),
            ("split over attributes", "#[doc = \"include\"]\n#[doc = \"!(\\\"x\\\");\"]\npub fn d() {}\n"),
        ] {
            let r = of(&[("r/src/lib.rs", source)], "r/src/lib.rs");
            assert!(refused(&r, "doc test"), "{case}: {:?}", whys(&r));
        }
        for (case, source) in [
            (
                "`include` ending a line",
                "/// Which files to include\n/// and which to leave.\npub fn d() {}\n",
            ),
            (
                "`include/exclude`",
                "/// The include/exclude patterns.\npub fn d() {}\n",
            ),
            (
                "`asm` ending a line",
                "/// Written in asm\n/// by hand.\npub fn d() {}\n",
            ),
        ] {
            let r = of(&[("r/src/lib.rs", source)], "r/src/lib.rs");
            assert!(r.unfollowed.is_empty(), "{case}: {:?}", whys(&r));
        }
    }

    #[test]
    fn every_path_is_followed_with_the_module_s_own_file_or_refused() {
        // Round 12's D2: a `cfg_attr`'s `path` holds only where its `cfg` does, and rustc reads the module's own file
        // elsewhere — so it is read beside every `path`, a module's file and an inline module's directory alike. D3: an
        // inner `path` names its own module's directory, which the census does not apply, so it is refused. D4: every
        // `path` value is checked as it is read, an inline module's among them.
        let r = of(
            &[
                (
                    "p/src/lib.rs",
                    "#[cfg_attr(test, path = \"mock.rs\")]\npub mod m;\n\
                     #[cfg_attr(test, path = \"elsewhere\")]\nmod inl {\n    mod z;\n}\n",
                ),
                ("p/src/mock.rs", ""),
                (
                    "p/src/m.rs",
                    "const M: &str = include_str!(\"../../c/m.txt\");\n",
                ),
                (
                    "p/src/inl/z.rs",
                    "const Z: &str = include_str!(\"../../../c/z.txt\");\n",
                ),
                ("c/m.txt", ""),
                ("c/z.txt", ""),
            ],
            "p/src/lib.rs",
        );
        for f in [
            "p/src/mock.rs",
            "p/src/m.rs",
            "c/m.txt",
            "p/src/inl/z.rs",
            "c/z.txt",
        ] {
            assert!(r.files.contains(f), "{f}: {:?}", files(&r));
            assert!(r.nontest.contains(f), "{f}: {:?}", r.nontest);
        }
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
        for (case, source, why) in [
            (
                "an inner path",
                "mod a {\n    #![path = \"../../c/src\"]\n    pub mod lib;\n}\n",
                "inner `path`",
            ),
            (
                "an absolute inline path",
                "#[path = \"/abs/c/src\"]\nmod a {\n    pub mod lib;\n}\n",
                "outside the commit's tree",
            ),
            (
                "a backslash in an inline path",
                "#[path = \"..\\\\c\"]\nmod a {\n    pub mod lib;\n}\n",
                "hosts read apart",
            ),
            (
                "a name outside ASCII in a module's path",
                "#[path = \"é.rs\"]\nmod a;\n",
                "hosts read apart",
            ),
        ] {
            let r = of(
                &[("p/src/lib.rs", source), ("c/src/lib.rs", "")],
                "p/src/lib.rs",
            );
            assert!(refused(&r, why), "{case}: {:?}", whys(&r));
        }
    }

    #[test]
    fn doc_text_is_read_for_doc_tested_libraries_alone() {
        // Round 11's R9: rustdoc tests a library's doc text alone — "This is only relevant for libraries, it has no
        // effect on other sections" — so a binary's or a test's doc test is no site; its code still is.
        let files = [(
            "r/tests/t.rs",
            "/// ```\n/// include!(\"x\");\n/// ```\nfn d() {}\nconst T: &str = include_str!(concat!(\"a\"));\n",
        )];
        let r = untested(&files, "r/tests/t.rs");
        assert_eq!(whys(&r).len(), 1, "{:?}", whys(&r));
        assert!(refused(&r, "no single string literal"), "{:?}", whys(&r));
        let r = of(&files, "r/tests/t.rs");
        assert!(refused(&r, "doc test"), "{:?}", whys(&r));
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
        let r = with_links(
            &[("r/tests/t.rs", "#[path = \"../link/lib.rs\"]\nmod l;\n")],
            &["r/link"],
            "r/tests/t.rs",
        );
        assert!(refused(&r, "symbolic link"), "{:?}", whys(&r));
    }

    #[test]
    fn a_path_meets_the_links_at_every_component_and_hosts_read_none_apart() {
        // Round 11's D4: a link's `..` is its target's parent on disk, so a path climbing back out of one is refused as
        // the link is, and a link in an inline module's directory too. R10 and R16: a path outside the tree's text,
        // absolute or climbing out, names no file of the commit; and a spelling a host reads apart — a backslash, a
        // drive, a trailing dot or space, a byte outside ASCII, a name the commit holds only in another case or in two
        // — is refused, never followed nor dropped.
        let c = ("c/src/lib.rs", "pub fn x() {}\n");
        for (case, source, links, why) in [
            (
                "back out of a link",
                "#[path = \"../link/../../c/src/lib.rs\"]\nmod l;\n",
                &["r/link"][..],
                "symbolic link",
            ),
            (
                "an inclusion through a link",
                "const T: &str = include_str!(\"../link/x.txt\");\n",
                &["r/link"][..],
                "symbolic link",
            ),
            (
                "an inline module's directory a link",
                "mod sub {\n    #[path = \"x.rs\"]\n    mod x;\n}\n",
                &["r/tests/sub"][..],
                "symbolic link",
            ),
            (
                "absolute",
                "#[path = \"/elsewhere/x.rs\"]\nmod b;\n",
                &[][..],
                "outside the commit's tree",
            ),
            (
                "climbing out",
                "const T: &str = include_str!(\"../../../x.txt\");\n",
                &[][..],
                "outside the commit's tree",
            ),
            (
                "a backslash",
                "const T: &str = include_str!(\"..\\\\..\\\\c\\\\src\\\\lib.rs\");\n",
                &[][..],
                "hosts read apart",
            ),
            (
                "a trailing dot",
                "const T: &str = include_str!(\"../../c/src/lib.rs.\");\n",
                &[][..],
                "hosts read apart",
            ),
            (
                "outside ASCII",
                "const T: &str = include_str!(\"../../c/src/lïb.rs\");\n",
                &[][..],
                "hosts read apart",
            ),
            (
                "another case",
                "const T: &str = include_str!(\"../../C/src/lib.rs\");\n",
                &[][..],
                "another case",
            ),
        ] {
            let r = with_links(&[("r/tests/t.rs", source), c], links, "r/tests/t.rs");
            assert!(refused(&r, why), "{case}: {:?}", whys(&r));
            assert!(!r.files.contains("c/src/lib.rs"), "{case}: {:?}", files(&r));
        }
        let r = of(
            &[
                (
                    "r/tests/t.rs",
                    "const T: &str = include_str!(\"../../c/a.txt\");\n",
                ),
                ("c/a.txt", ""),
                ("c/A.txt", ""),
            ],
            "r/tests/t.rs",
        );
        assert!(refused(&r, "another case"), "{:?}", whys(&r));
        // A name the commit does not hold at all reaches nothing: no build of the commit can read it.
        let r = of(
            &[(
                "r/tests/t.rs",
                "const T: &str = include_str!(\"absent.txt\");\nmod absent;\n",
            )],
            "r/tests/t.rs",
        );
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
        assert_eq!(files(&r), ["r/tests/t.rs"]);
        // A crate root is read as cargo's metadata names it, `..` kept, and refused out of the tree or through a link.
        let t = tree(&[c]);
        let links: BTreeSet<String> = ["l".to_owned()].into();
        let sources = Sources::new(&t, &links);
        assert_eq!(
            sources.root("r/../c/src/lib.rs").as_deref(),
            Ok("c/src/lib.rs")
        );
        assert!(sources.root("../outside/lib.rs").is_err());
        assert!(sources.root("l/../c/src/lib.rs").is_err());
        assert!(sources.root("c/src/absent.rs").is_err());
    }

    #[test]
    fn a_file_reached_through_cfg_test_alone_counts_for_its_crate_s_tests_alone() {
        // `cfg(test)` is set only on the crate a test harness builds: what a module under exactly `#[cfg(test)]`, or a
        // module file whose leading inner attributes hold `#![cfg(test)]`, reaches counts for its own test units, never
        // for it as a dependency or for its non-test units. Round 11's D2: nothing else narrows — `cfg(test)` on another
        // item, a field, a variant, a parameter, an arm or a statement, `#![cfg(test)]` in an inline module or an
        // `include!`d file, a module under it inside a macro — and any other `cfg` is read as on.
        let r = of(
            &[
                (
                    "p/src/lib.rs",
                    "#[cfg(test)]\nmod tests;\n#[cfg(test)]\n#[allow(unused)]\npub(crate) mod more {\n    \
                     const M: &str = include_str!(\"../../c/m.txt\");\n}\n\
                     #[cfg(test)]\nconst T: &str = include_str!(\"../../c/a.txt\");\n\
                     #[cfg(any(test, feature = \"x\"))]\nconst U: &str = include_str!(\"../../c/b.txt\");\n\
                     pub struct S {\n    #[cfg(test)]\n    a: u8,\n}\nconst F: &str = include_str!(\"../../c/field.txt\");\n\
                     pub enum E {\n    #[cfg(test)]\n    A,\n}\nconst V: &str = include_str!(\"../../c/variant.txt\");\n\
                     pub fn g(#[cfg(test)] _a: u8) {}\nconst P: &str = include_str!(\"../../c/param.txt\");\n\
                     pub fn h(x: u8) -> u8 {\n    match x {\n        #[cfg(test)]\n        0 => 1,\n        _ => 2,\n    }\n}\n\
                     const A: &str = include_str!(\"../../c/arm.txt\");\n\
                     mod im {\n    #![cfg(test)]\n    const I: &str = include_str!(\"../../c/inline.txt\");\n}\n\
                     wrap! {\n    #[cfg(test)]\n    mod wrapped {\n        const W: &str = include_str!(\"../../c/macro.txt\");\n    }\n}\n\
                     include!(\"inc.rs\");\nmod inner;\n",
                ),
                ("p/src/tests.rs", "const V: &str = include_str!(\"../../c/c.txt\");\n"),
                ("p/src/inc.rs", "#![cfg(test)]\nconst N: &str = include_str!(\"../../c/included.txt\");\n"),
                ("p/src/inner.rs", "#![allow(unused)]\n#![cfg(test)]\nconst W: &str = include_str!(\"../../c/d.txt\");\n"),
                ("c/a.txt", ""),
                ("c/b.txt", ""),
                ("c/c.txt", ""),
                ("c/d.txt", ""),
                ("c/m.txt", ""),
                ("c/field.txt", ""),
                ("c/variant.txt", ""),
                ("c/param.txt", ""),
                ("c/arm.txt", ""),
                ("c/inline.txt", ""),
                ("c/macro.txt", ""),
                ("c/included.txt", ""),
            ],
            "p/src/lib.rs",
        );
        for f in ["c/c.txt", "c/d.txt", "c/m.txt", "p/src/tests.rs"] {
            assert!(r.files.contains(f), "{f}: {:?}", files(&r));
            assert!(!r.nontest.contains(f), "{f}: {:?}", r.nontest);
        }
        for f in [
            "c/a.txt",
            "c/b.txt",
            "c/field.txt",
            "c/variant.txt",
            "c/param.txt",
            "c/arm.txt",
            "c/inline.txt",
            "c/macro.txt",
            "c/included.txt",
            "p/src/inc.rs",
            "p/src/inner.rs",
            "p/src/lib.rs",
        ] {
            assert!(r.nontest.contains(f), "{f}: {:?}", r.nontest);
        }
    }

    #[test]
    fn a_raw_identifier_and_every_mod_are_read_whole_or_refused() {
        // Round 11's D3: `r#type` is one identifier, its module `type.rs`, an inline one `type/`; and a `mod` the census
        // cannot read whole — in a macro's arguments, declared by a macro, inside an attribute, or spelt otherwise — is
        // refused, never passed. A macro's inline module loads no file.
        let r = of(
            &[
                (
                    "p/src/lib.rs",
                    "mod r#type;\nmod r#impl {\n    mod inner;\n}\nconst T: &str = r#include_str!(\"t.txt\");\n",
                ),
                ("p/src/type.rs", ""),
                ("p/src/impl/inner.rs", ""),
                ("p/src/t.txt", ""),
            ],
            "p/src/lib.rs",
        );
        for f in ["p/src/type.rs", "p/src/impl/inner.rs", "p/src/t.txt"] {
            assert!(r.files.contains(f), "{f}: {:?}", files(&r));
        }
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
        for (case, source, why) in [
            (
                "in a macro's arguments",
                "wrap! { mod x; }\n",
                "a macro's arguments",
            ),
            (
                "a macro adding its `;`",
                "semi!(mod x);\n",
                "a macro's arguments",
            ),
            (
                "a macro's",
                "macro_rules! m { ($n:ident) => { mod $n; }; }\n",
                "a macro declares",
            ),
            (
                "in an attribute",
                "#[gen(mod x;)]\nfn f() {}\n",
                "inside an attribute",
            ),
            ("spelt otherwise", "mod x\n", "cannot read whole"),
        ] {
            let r = of(
                &[("p/src/lib.rs", source), ("p/src/x.rs", "")],
                "p/src/lib.rs",
            );
            assert!(refused(&r, why), "{case}: {:?}", whys(&r));
        }
        let r = of(
            &[(
                "p/src/lib.rs",
                "macro_rules! m { ($n:ident) => { mod $n { pub fn f() {} } mod fixed {} }; }\nwrap! { mod y { } }\n",
            )],
            "p/src/lib.rs",
        );
        assert!(r.unfollowed.is_empty(), "{:?}", whys(&r));
    }

    #[test]
    fn every_attribute_and_macro_that_reads_a_file_is_read_whole_or_refused() {
        // Round 11's R9 and R10: a precise capture begins no `use` item; a `doc` attribute's text is a literal or a
        // document `include_str!` names, by any path, else refused; a `debugger_visualizer` file is followed as data; an
        // assembly template holding a directive that reads a file or assembles one, or a macro, refused, as an alias of
        // an assembly macro is; a plain template and import not.
        let r = of(
            &[
                (
                    "r/src/lib.rs",
                    "pub fn f() -> impl Sized + use<> {}\nconst T: &str = include_str!(\"t.txt\");\n\
                     #![debugger_visualizer(gdb_script_file = \"../../c/pretty.py\")]\n\
                     #[doc = ::core::include_str!(\"../README.md\")]\npub struct R;\n\
                     use core::arch::asm;\npub unsafe fn n() { asm!(\"nop\") }\n",
                ),
                ("r/src/t.txt", ""),
                ("c/pretty.py", ""),
                ("r/README.md", "```\ninclude!(\"x\");\n```\n"),
            ],
            "r/src/lib.rs",
        );
        for f in ["r/src/t.txt", "c/pretty.py", "r/README.md"] {
            assert!(r.files.contains(f), "{f}: {:?}", files(&r));
        }
        let w = whys(&r);
        assert_eq!(w.len(), 1, "{w:?}");
        assert!(
            w[0].0 == "r/README.md" && w[0].2.contains("doc test"),
            "{w:?}"
        );
        for (case, source, why) in [
            (
                "a doc a macro gives",
                "#[doc = concat!(\"a\", \"b\")]\npub fn f() {}\n",
                "no literal gives",
            ),
            (
                "a doc a metavariable gives",
                "macro_rules! m { ($d:expr) => { #[doc = $d] pub fn f() {} }; }\n",
                "no literal gives",
            ),
            (
                "a visualizer no literal names",
                "#![debugger_visualizer(natvis_file = concat!(\"a\"))]\n",
                "debugger_visualizer",
            ),
            (
                "an incbin",
                "core::arch::global_asm!(\".data\\n.INCBIN \\\"../c/blob.bin\\\"\");\n",
                "a directive",
            ),
            (
                "an include directive",
                "core::arch::global_asm!(\".include \\\"x.s\\\"\");\n",
                "a directive",
            ),
            (
                "a template a macro gives",
                "core::arch::global_asm!(include_str!(\"boot.s\"));\n",
                "a macro assembles",
            ),
            (
                "in a macro",
                "macro_rules! m { ($t:literal) => { core::arch::global_asm!($t); }; }\n",
                "inside a macro",
            ),
            (
                "an alias",
                "use core::arch::global_asm as g;\n",
                "imports or shadows",
            ),
        ] {
            let r = of(&[("r/src/lib.rs", source)], "r/src/lib.rs");
            assert!(refused(&r, why), "{case}: {:?}", whys(&r));
        }
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
