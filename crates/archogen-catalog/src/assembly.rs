//! §14.2's invocation rules (`M2.12.4.2`): in a package an `assembly` declaration names, `asm` and `naked_asm`
//! are admitted only as `::core::arch::asm!(` or `::core::arch::naked_asm!(`, each invocation held to the record's
//! rules (`docs/specs/catalog/decision_catalog-records-port.md` §14.2, §14.3).
//!
//! > So every reach into other code is a Rust path the compiler resolves, or an address the code computes. Nothing
//! > else assembly can do is admitted: no directive, no symbol written by name, no `global_asm!`.
//!
//! The check reads tokens, as §3's rules do: `Structure::of` finds, once per file, each delimiter's partner and
//! which `{` opens a function's, an `impl`'s or a `trait`'s body; `check` then reads one invocation — where it
//! lies, its arguments in their order, and each template line by the dialect's signatures. Every refusal is the
//! package's, so the caller files it as `catalog-source`.

use std::collections::BTreeMap;

use crate::dialect::{self, Operand};
use crate::package::{Kind, Token, KEYWORDS};

/// What a `{` opens, when it opens an item's body.
#[derive(Debug, Clone, PartialEq, Eq)]
enum Body {
    /// A function's: the architecture its outer `#[cfg(all(target_arch = "…", target_os = "none"))]` names, and
    /// whether it has type or const parameters, an argument-position `impl Trait` counting as one.
    Function { cfg: Option<String>, generic: bool },
    /// An `impl` block's, and whether it has type or const parameters.
    Impl { generic: bool },
    /// A `trait`'s.
    Trait,
}

/// A file's delimiters, and the item each `{` opens: what [`check`] reads an invocation's place by.
#[derive(Debug)]
pub(crate) struct Structure {
    /// For each delimiter, its partner's index.
    partner: Vec<Option<usize>>,
    /// For each token, the innermost open delimiter enclosing it.
    parent: Vec<Option<usize>>,
    /// The `{` tokens that open an item's body.
    bodies: BTreeMap<usize, Body>,
}

fn ident(tokens: &[Token], i: usize) -> Option<&str> {
    match &tokens.get(i)?.kind {
        Kind::Ident(name, _) => Some(name),
        _ => None,
    }
}

fn is(tokens: &[Token], i: usize, kind: &Kind) -> bool {
    tokens.get(i).is_some_and(|t| t.kind == *kind)
}

const COLON: Kind = Kind::Punct(':');

impl Structure {
    /// The structure of `tokens`, read from `source`.
    pub(crate) fn of(source: &[u8], tokens: &[Token]) -> Self {
        let n = tokens.len();
        let mut partner = vec![None; n];
        let mut parent = vec![None; n];
        let mut stack: Vec<usize> = Vec::new();
        for (i, t) in tokens.iter().enumerate() {
            match t.kind {
                Kind::Close(_) => {
                    if let Some(open) = stack.pop() {
                        partner[open] = Some(i);
                        partner[i] = Some(open);
                    }
                    parent[i] = stack.last().copied();
                }
                Kind::Open(_) => {
                    parent[i] = stack.last().copied();
                    stack.push(i);
                }
                _ => parent[i] = stack.last().copied(),
            }
        }
        let mut structure = Self {
            partner,
            parent,
            bodies: BTreeMap::new(),
        };
        for i in 0..n {
            match ident(tokens, i) {
                Some("fn")
                    if ident(tokens, i + 1).is_some_and(|name| !KEYWORDS.contains(&name)) =>
                {
                    let mut j = i + 2;
                    let mut generic = false;
                    if is(tokens, j, &Kind::Punct('<')) {
                        let (params, end) = structure.generics(source, tokens, j);
                        generic = params;
                        j = end;
                    }
                    if is(tokens, j, &Kind::Open('(')) {
                        if let Some(close) = structure.partner[j] {
                            generic |= (j..close).any(|p| ident(tokens, p) == Some("impl"));
                            j = close + 1;
                        }
                    }
                    if let Some(body) = structure.body_after(tokens, j) {
                        let cfg = outer_cfg(source, tokens, &structure.partner, i);
                        structure
                            .bodies
                            .insert(body, Body::Function { cfg, generic });
                    }
                }
                Some("impl") if item_position(tokens, i) => {
                    let mut j = i + 1;
                    let mut generic = false;
                    if is(tokens, j, &Kind::Punct('<')) {
                        let (params, end) = structure.generics(source, tokens, j);
                        generic = params;
                        j = end;
                    }
                    if let Some(body) = structure.body_after(tokens, j) {
                        structure.bodies.insert(body, Body::Impl { generic });
                    }
                }
                Some("trait") => {
                    if let Some(body) = structure.body_after(tokens, i + 1) {
                        structure.bodies.insert(body, Body::Trait);
                    }
                }
                _ => {}
            }
        }
        structure
    }

    /// Skip a balanced `<…>` starting at `from`, delimiter groups whole and `->` not closing it: the index past
    /// it, or the end.
    fn skip_angles(&self, tokens: &[Token], from: usize) -> usize {
        let mut depth = 0usize;
        let mut j = from;
        while j < tokens.len() {
            match tokens[j].kind {
                Kind::Open(_) => match self.partner[j] {
                    Some(close) => j = close,
                    None => return tokens.len(),
                },
                Kind::Punct('<') => depth += 1,
                Kind::Punct('>') if j > 0 && tokens[j - 1].kind == Kind::Punct('-') => {}
                Kind::Punct('>') => {
                    depth = depth.saturating_sub(1);
                    if depth == 0 {
                        return j + 1;
                    }
                }
                _ => {}
            }
            j += 1;
        }
        tokens.len()
    }

    /// Whether the generic parameters at `from`, a `<`, include a type or const parameter, and the index past them.
    fn generics(&self, source: &[u8], tokens: &[Token], from: usize) -> (bool, usize) {
        let end = self.skip_angles(tokens, from);
        let mut generic = false;
        let mut first = true;
        let mut depth = 0usize;
        let mut j = from + 1;
        while j + 1 < end {
            let t = &tokens[j];
            if first {
                let lifetime = t.kind == Kind::Other && source.get(t.at) == Some(&b'\'');
                generic |= !lifetime;
                first = false;
            }
            match t.kind {
                Kind::Open(_) => {
                    if let Some(close) = self.partner[j] {
                        j = close;
                    }
                }
                Kind::Punct('<') => depth += 1,
                Kind::Punct('>') if tokens[j - 1].kind == Kind::Punct('-') => {}
                Kind::Punct('>') => depth = depth.saturating_sub(1),
                Kind::Punct(',') if depth == 0 => first = true,
                _ => {}
            }
            j += 1;
        }
        (generic, end)
    }

    /// The `{` that opens the body of the item whose header continues at `from`: the first `{` before a `;` at the
    /// header's depth, delimiter groups and `<…>` skipped whole.
    fn body_after(&self, tokens: &[Token], from: usize) -> Option<usize> {
        let mut j = from;
        while j < tokens.len() {
            match tokens[j].kind {
                Kind::Open('{') => return Some(j),
                Kind::Open(_) => j = self.partner[j]? + 1,
                Kind::Close(_) | Kind::Punct(';') => return None,
                Kind::Punct('<') => j = self.skip_angles(tokens, j),
                _ => j += 1,
            }
        }
        None
    }
}

/// Whether the `impl` at `i` begins an item, not an `impl Trait` type: what precedes it ends an item or a
/// statement, opens a block, or is `unsafe` or `default`.
fn item_position(tokens: &[Token], i: usize) -> bool {
    match i.checked_sub(1).map(|p| &tokens[p].kind) {
        None | Some(Kind::Close('}' | ']') | Kind::Open('{') | Kind::Punct(';')) => true,
        Some(Kind::Ident(name, _)) => name == "unsafe" || name == "default",
        _ => false,
    }
}

/// The architecture an outer `#[cfg(all(target_arch = "…", target_os = "none"))]` of the function whose `fn` is
/// at `at` names, compared as tokens: the attributes before its qualifiers, read backwards.
fn outer_cfg(
    source: &[u8],
    tokens: &[Token],
    partner: &[Option<usize>],
    at: usize,
) -> Option<String> {
    let mut j = at.checked_sub(1)?;
    loop {
        match &tokens[j].kind {
            Kind::Ident(q, _)
                if matches!(
                    q.as_str(),
                    "pub" | "const" | "async" | "unsafe" | "extern" | "default" | "safe"
                ) => {}
            Kind::Str => {}
            Kind::Close(')') => {
                let open = partner[j]?;
                if ident(tokens, open.checked_sub(1)?) != Some("pub") {
                    break;
                }
                j = open;
            }
            _ => break,
        }
        j = j.checked_sub(1)?;
    }
    let mut found = None;
    while tokens[j].kind == Kind::Close(']') {
        let open = partner[j]?;
        let hash = open.checked_sub(1)?;
        if tokens[hash].kind != Kind::Punct('#') {
            break;
        }
        found = found.or_else(|| cfg_of(source, &tokens[hash..=j]));
        j = hash.checked_sub(1)?;
    }
    found
}

/// The architecture `#[cfg(all(target_arch = "<a>", target_os = "none"))]` names, when `attribute` is exactly it.
fn cfg_of(source: &[u8], attribute: &[Token]) -> Option<String> {
    let text = |t: &Token| String::from_utf8_lossy(&source[t.at..t.end]).into_owned();
    let words: Vec<String> = attribute.iter().map(text).collect();
    let expected = [
        "#",
        "[",
        "cfg",
        "(",
        "all",
        "(",
        "target_arch",
        "=",
        "",
        ",",
        "target_os",
        "=",
        "\"none\"",
        ")",
        ")",
        "]",
    ];
    if words.len() != expected.len() {
        return None;
    }
    let same = words
        .iter()
        .zip(expected)
        .enumerate()
        .all(|(k, (w, e))| k == 8 || w == e);
    let arch = words[8].strip_prefix('"')?.strip_suffix('"')?;
    same.then(|| arch.to_owned())
}

/// What a named operand is (§14.2).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Bound {
    Sym,
    Const,
    In,
    Out,
    InOut,
}

/// One invocation, read: what a template line is checked against.
struct Invocation<'a> {
    naked: bool,
    architecture: &'a str,
    operands: BTreeMap<String, Bound>,
}

type Refused = (usize, String);

/// §14.2's rules over the invocation whose `asm` or `naked_asm` is token `k`.
///
/// # Errors
///
/// The first rule it breaks, with the offset it points at.
pub(crate) fn check(
    source: &[u8],
    tokens: &[Token],
    structure: &Structure,
    k: usize,
    architecture: &str,
) -> Result<(), Refused> {
    let name = ident(tokens, k).unwrap_or_default();
    let at = tokens[k].at;
    let path = [
        COLON,
        COLON,
        Kind::Ident("core".into(), false),
        COLON,
        COLON,
        Kind::Ident("arch".into(), false),
        COLON,
        COLON,
    ];
    let sequence = k >= 8
        && path
            .iter()
            .enumerate()
            .all(|(d, kind)| tokens[k - 8 + d].kind == *kind)
        && is(tokens, k + 1, &Kind::Punct('!'))
        && is(tokens, k + 2, &Kind::Open('('))
        && !k
            .checked_sub(9)
            .is_some_and(|p| matches!(tokens[p].kind, Kind::Ident(..) | Kind::Punct('>' | ':')));
    if !sequence {
        return Err((
            at,
            format!("`{name}` other than in `::core::arch::{name}!(`, written out in full"),
        ));
    }
    let mut chain = Vec::new();
    let mut p = structure.parent[k];
    while let Some(open) = p {
        if let Some(body) = structure.bodies.get(&open) {
            chain.push(body);
        }
        p = structure.parent[open];
    }
    let innermost = chain.iter().find_map(|b| match b {
        Body::Function { cfg, .. } => Some(cfg),
        _ => None,
    });
    match innermost {
        None => return Err((at, format!("`{name}!` inside no function"))),
        Some(cfg) if cfg.as_deref() != Some(architecture) => {
            return Err((
                at,
                format!(
                    "the innermost function around `{name}!` does not carry `#[cfg(all(target_arch = \"{architecture}\", target_os = \"none\"))]`"
                ),
            ))
        }
        Some(_) => {}
    }
    let open = k + 2;
    let close = structure.partner[open].ok_or((at, format!("`{name}!(` is not closed")))?;
    let mut invocation = Invocation {
        naked: name == "naked_asm",
        architecture,
        operands: BTreeMap::new(),
    };
    let mut templates: Vec<(usize, String)> = Vec::new();
    let mut options = false;
    let arguments = split(tokens, &structure.partner, open + 1, close)?;
    for (s, e) in arguments {
        let first = &tokens[s];
        let refuse = |why: String| Err((first.at, why));
        if e - s == 1 && first.kind == Kind::Str {
            if !invocation.operands.is_empty() || options {
                return refuse(
                    "a template string after an operand: out of §14.2's order".to_owned(),
                );
            }
            let raw = String::from_utf8_lossy(&source[first.at..first.end]).into_owned();
            let Some(line) = raw.strip_prefix('"').and_then(|r| r.strip_suffix('"')) else {
                return refuse("a template string that is raw or prefixed".to_owned());
            };
            if line.contains('\\') || line.contains('\n') {
                return refuse("a template string that holds `\\` or a line feed".to_owned());
            }
            templates.push((first.at, line.to_owned()));
        } else if ident(tokens, s) == Some("options") && is(tokens, s + 1, &Kind::Open('(')) {
            if invocation.naked {
                return refuse("`options` in `naked_asm!`".to_owned());
            }
            if options {
                return refuse("a second `options`".to_owned());
            }
            let inner = structure.partner[s + 1].unwrap_or(e);
            let nostack = inner == e - 1
                && ident(tokens, s + 2) == Some("nostack")
                && (inner == s + 3 || (inner == s + 4 && is(tokens, s + 3, &Kind::Punct(','))));
            if !nostack {
                return refuse("an option other than `nostack`".to_owned());
            }
            options = true;
        } else if let (Some(operand), true) =
            (ident(tokens, s), is(tokens, s + 1, &Kind::Punct('=')))
        {
            if options {
                return refuse("an operand after `options`: out of §14.2's order".to_owned());
            }
            if templates.is_empty() {
                return refuse(
                    "an operand before any template string: out of §14.2's order".to_owned(),
                );
            }
            if KEYWORDS.contains(&operand) {
                return refuse(format!("the keyword `{operand}` as an operand's name"));
            }
            let bound = operand_kind(tokens, structure, s + 2, e, invocation.naked)
                .map_err(|why| (first.at, why))?;
            if invocation
                .operands
                .insert(operand.to_owned(), bound)
                .is_some()
            {
                return refuse(format!("the operand `{operand}` named twice"));
            }
        } else if first.kind == Kind::Punct('#') {
            return refuse("an attribute on an argument".to_owned());
        } else if let Some(word) = ident(tokens, s) {
            return refuse(format!("`{word}`: an argument §14.2 does not admit"));
        } else {
            return refuse("an argument §14.2 does not admit".to_owned());
        }
    }
    if templates.is_empty() {
        return Err((at, format!("`{name}!` holds no template string")));
    }
    if invocation.operands.values().any(|b| *b == Bound::Sym)
        && chain.iter().any(|b| {
            matches!(
                b,
                Body::Function { generic: true, .. } | Body::Impl { generic: true } | Body::Trait
            )
        })
    {
        return Err((
            at,
            "a `sym` operand where an enclosing function, `impl` or `trait` has type or const parameters, or inside a `trait`".to_owned(),
        ));
    }
    invocation.lines(&templates)
}

/// The arguments between `from` and `close`, each a token range, split at commas outside any group; a comma after
/// the last is admitted.
fn split(
    tokens: &[Token],
    partner: &[Option<usize>],
    from: usize,
    close: usize,
) -> Result<Vec<(usize, usize)>, Refused> {
    let mut out = Vec::new();
    if from == close {
        return Ok(out);
    }
    let mut start = from;
    let mut j = from;
    while j < close {
        match tokens[j].kind {
            Kind::Open(_) => j = partner[j].unwrap_or(close) + 1,
            Kind::Punct(',') => {
                out.push((start, j));
                j += 1;
                start = j;
            }
            _ => j += 1,
        }
    }
    out.push((start, close));
    if out.len() > 1 && out.last().is_some_and(|(s, e)| s == e) {
        out.pop();
    }
    if let Some((s, _)) = out.iter().find(|(s, e)| s == e) {
        return Err((tokens[(*s).min(close)].at, "an empty argument".to_owned()));
    }
    Ok(out)
}

/// The kind of the operand whose value starts at token `from` (after `<name> =`) and ends before `end`.
fn operand_kind(
    tokens: &[Token],
    structure: &Structure,
    from: usize,
    end: usize,
    naked: bool,
) -> Result<Bound, String> {
    match ident(tokens, from) {
        Some("sym") => {
            sym_path(tokens, from + 1, end)?;
            Ok(Bound::Sym)
        }
        Some("const") => {
            if from + 1 >= end {
                return Err("a `const` operand with no expression".to_owned());
            }
            Ok(Bound::Const)
        }
        Some(direction @ ("in" | "out" | "inout")) => {
            if naked {
                return Err(format!(
                    "`{direction}` in `naked_asm!`, which admits `sym` and `const` operands only"
                ));
            }
            let class = is(tokens, from + 1, &Kind::Open('('))
                && ident(tokens, from + 2) == Some("reg")
                && is(tokens, from + 3, &Kind::Close(')'));
            if !class {
                return Err("an explicit register, or a class other than `(reg)`".to_owned());
            }
            let expression = from + 4;
            let mut arrow = None;
            let mut j = expression;
            while j < end {
                match tokens[j].kind {
                    Kind::Open(_) => j = structure.partner[j].unwrap_or(end),
                    Kind::Punct('=') if is(tokens, j + 1, &Kind::Punct('>')) => {
                        arrow = arrow.or(Some(j));
                    }
                    _ => {}
                }
                j += 1;
            }
            let empty = match arrow {
                Some(a) => a == expression || a + 2 >= end,
                None => expression >= end,
            };
            if empty {
                return Err(format!("an `{direction}` operand with no expression"));
            }
            match (direction, arrow) {
                ("inout", _) => Ok(Bound::InOut),
                (_, Some(_)) => Err(format!("`=>` after an `{direction}` operand")),
                ("in", None) => Ok(Bound::In),
                _ => Ok(Bound::Out),
            }
        }
        Some(late @ ("lateout" | "inlateout")) => Err(format!(
            "`{late}`, refused: the compiler may give its register to an input"
        )),
        Some(other) => Err(format!("a `{other}` operand, which §14.2 does not admit")),
        None => Err("an operand §14.2 does not admit".to_owned()),
    }
}

/// A `sym` operand's path (§14.2): identifiers joined by `::`, optionally beginning `crate::` or `self::`, or with
/// one or more `super::`; no `Self` segment, no generic arguments.
fn sym_path(tokens: &[Token], from: usize, end: usize) -> Result<(), String> {
    let refuse = || {
        Err("a `sym` path §14.2 does not admit: identifiers joined by `::`, optionally after `crate::`, `self::` or `super::`, with no `Self` and no generic arguments".to_owned())
    };
    let separator =
        |j: usize| j + 1 < end && tokens[j].kind == COLON && tokens[j + 1].kind == COLON;
    let mut j = from;
    match ident(tokens, j) {
        Some("crate" | "self") => {
            if !separator(j + 1) {
                return refuse();
            }
            j += 3;
        }
        _ => {
            while ident(tokens, j) == Some("super") {
                if !separator(j + 1) {
                    return refuse();
                }
                j += 3;
            }
        }
    }
    loop {
        match ident(tokens, j) {
            Some(name) if j < end && !KEYWORDS.contains(&name) => j += 1,
            _ => return refuse(),
        }
        if j == end {
            return Ok(());
        }
        if !separator(j) {
            return refuse();
        }
        j += 2;
    }
}

/// A label number (§14.2): `0`, or one to four digits not beginning with `0`.
fn is_label_number(text: &str) -> bool {
    text == "0"
        || (!text.is_empty()
            && text.len() <= 4
            && !text.starts_with('0')
            && text.bytes().all(|b| b.is_ascii_digit()))
}

/// An integer (§14.2): `0`, or a decimal not beginning with `0`, optionally after `-` (`-0` refused), within the
/// signed 64-bit range.
fn integer(text: &str) -> Result<(), String> {
    let digits = text.strip_prefix('-').unwrap_or(text);
    if digits.is_empty() || !digits.bytes().all(|b| b.is_ascii_digit()) {
        return Err(format!("`{text}` is not an integer"));
    }
    if digits.len() > 1 && digits.starts_with('0') {
        return Err(format!(
            "`{text}` has a leading `0`, which the assembler reads as octal"
        ));
    }
    if text == "-0" {
        return Err("`-0`".to_owned());
    }
    match text.parse::<i64>() {
        Ok(_) => Ok(()),
        Err(_) => Err(format!(
            "`{text}` is outside the signed 64-bit range, which the assembler reads modulo 2^64"
        )),
    }
}

impl Invocation<'_> {
    /// Every template line, then the labels they reference and how a naked body ends.
    fn lines(&self, templates: &[(usize, String)]) -> Result<(), Refused> {
        let mut labels: Vec<(usize, &str)> = Vec::new();
        let mut references: Vec<(usize, usize, String)> = Vec::new();
        let mut last = None;
        for (index, (at, line)) in templates.iter().enumerate() {
            let refuse = |why: String| Err((*at, format!("template line `{line}`: {why}")));
            if let Some(why) = forbidden(line) {
                return refuse(why.to_owned());
            }
            if let Some(number) = line.strip_suffix(':') {
                if self.naked {
                    if !is_label_number(number) {
                        return refuse(
                            "a label number is `0`, or one to four digits not beginning with `0`"
                                .to_owned(),
                        );
                    }
                    labels.push((index, number));
                    last = None;
                    continue;
                }
                return refuse(
                    "a label line in `asm!`, whose every line is an inline instruction".to_owned(),
                );
            }
            let (word, rest) = match line.split_once(' ') {
                Some((w, r)) => (w, Some(r)),
                None => (line.as_str(), None),
            };
            if word.starts_with('.') {
                return refuse("a directive".to_owned());
            }
            let Some(mnemonic) = dialect::mnemonic(self.architecture, word) else {
                return refuse(format!("`{word}` is not a mnemonic §14.3 lists"));
            };
            if !self.naked && !mnemonic.inline {
                return refuse(format!(
                    "`{word}` is not marked inline, so `asm!` does not admit it"
                ));
            }
            let operands: Vec<&str> = match rest {
                None => Vec::new(),
                Some(r) => r.split(", ").collect(),
            };
            if operands.len() != mnemonic.signature.len() {
                return refuse(format!(
                    "`{word}` takes {} operands, written `<mnemonic> <a>, <b>`",
                    mnemonic.signature.len()
                ));
            }
            for (position, (text, kind)) in operands.iter().zip(mnemonic.signature).enumerate() {
                // Which register a line writes matters only to an `asm!`'s placeholders, and every inline
                // mnemonic with a leading `R` writes it (§14.3).
                let written = position == 0 && *kind == Operand::Register;
                self.operand(text, *kind, written, word)
                    .or_else(|why| refuse(format!("`{text}`: {why}")))?;
                if *kind == Operand::Label {
                    references.push((index, *at, (*text).to_owned()));
                }
            }
            last = Some(word);
        }
        for (index, at, text) in references {
            let (number, direction) = text.split_at(text.len() - 1);
            let found = if direction == "b" {
                labels.iter().any(|(l, n)| *l < index && *n == number)
            } else {
                labels.iter().any(|(l, n)| *l > index && *n == number)
            };
            if !found {
                return Err((
                    at,
                    format!("`{text}` resolves to no label line in its invocation"),
                ));
            }
        }
        if self.naked && !last.is_some_and(dialect::ends_a_body) {
            let at = templates.last().map_or(0, |(a, _)| *a);
            return Err((
                at,
                "a naked body's last line is not one that never falls through: `mret`, `ret`, `jr`, `tail` or `j`"
                    .to_owned(),
            ));
        }
        Ok(())
    }

    /// One operand of kind `kind`; `written` when its signature writes the register.
    fn operand(&self, text: &str, kind: Operand, written: bool, word: &str) -> Result<(), String> {
        match kind {
            Operand::Register => self.register(text, written),
            Operand::System => {
                if !dialect::is_system_register(text) {
                    return Err("not a system register §14.3 lists".to_owned());
                }
                if text == "mhartid" && word != "csrr" {
                    return Err(
                        "`mhartid` is read-only, admitted only as `csrr`'s system register"
                            .to_owned(),
                    );
                }
                Ok(())
            }
            Operand::Integer => match self.placeholder(text)? {
                Some(Bound::Const) => Ok(()),
                Some(_) => Err(
                    "a placeholder for an operand that is not `const`, in an integer's position"
                        .to_owned(),
                ),
                None => integer(text),
            },
            Operand::Memory => {
                let (offset, register) = text
                    .strip_suffix(')')
                    .and_then(|t| t.split_once('('))
                    .ok_or_else(|| {
                        "a memory operand is an integer, then a register in parentheses".to_owned()
                    })?;
                self.operand(offset, Operand::Integer, false, word)?;
                self.register(register, false)
            }
            Operand::Code => match self.placeholder(text)? {
                Some(Bound::Sym) => Ok(()),
                _ => Err(
                    "a code position takes a placeholder for a `sym` operand, and nothing else"
                        .to_owned(),
                ),
            },
            Operand::Label => {
                // The number is checked by resolution: only a label line's, which is one, resolves.
                let ok = text
                    .strip_suffix('b')
                    .or_else(|| text.strip_suffix('f'))
                    .is_some();
                if ok {
                    Ok(())
                } else {
                    Err("a label is a label number, then `b` or `f`".to_owned())
                }
            }
        }
    }

    /// A register operand: in `asm!`, `zero` or a placeholder for a `reg` operand of the right direction; in
    /// `naked_asm!`, a register §14.3 lists.
    fn register(&self, text: &str, written: bool) -> Result<(), String> {
        match self.placeholder(text)? {
            Some(Bound::Sym | Bound::Const) => {
                Err("a placeholder for a `sym` or `const` operand, in a register's position".to_owned())
            }
            Some(Bound::In) if written => Err("the line writes it, and it is an `in` operand".to_owned()),
            Some(Bound::Out) if !written => Err("the line reads it, and it is an `out` operand".to_owned()),
            Some(_) => Ok(()),
            None if text == "zero" => Ok(()),
            None if self.naked && dialect::is_register(text) => Ok(()),
            None if !self.naked && dialect::is_register(text) => Err(
                "in `asm!`, a register operand is `zero` or a placeholder, so the compiler knows every register it touches"
                    .to_owned(),
            ),
            None => Err("not a register §14.3 lists".to_owned()),
        }
    }

    /// When `text` is a placeholder, the operand it names: `{<identifier>}`, no modifier, no space.
    fn placeholder(&self, text: &str) -> Result<Option<Bound>, String> {
        let Some(inner) = text.strip_prefix('{').and_then(|t| t.strip_suffix('}')) else {
            if text.contains('{') || text.contains('}') {
                return Err("a placeholder is `{<identifier>}`, alone in its position".to_owned());
            }
            return Ok(None);
        };
        if inner.is_empty() || inner.bytes().all(|b| b.is_ascii_digit()) {
            return Err("`{}` and `{<digits>}` are refused: name the operand".to_owned());
        }
        let identifier = inner
            .bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'_')
            && !inner.as_bytes()[0].is_ascii_digit();
        if !identifier {
            return Err("a placeholder with a modifier or a space".to_owned());
        }
        self.operands
            .get(inner)
            .copied()
            .map(Some)
            .ok_or_else(|| format!("`{{{inner}}}` names no operand"))
    }
}

/// What no template line may hold, wherever it stands.
fn forbidden(line: &str) -> Option<&'static str> {
    if line.contains('\t') {
        Some("a tab")
    } else if line.contains('#') || line.contains("//") || line.contains("/*") {
        Some("a comment")
    } else if line.contains(';') {
        Some("`;`")
    } else if line.contains('%') {
        Some("a relocation operator `%`")
    } else if line.contains("{{") || line.contains("}}") {
        Some("`{{` or `}}`")
    } else {
        None
    }
}
