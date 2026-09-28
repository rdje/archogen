//! `M1.11` — the grammar is normative, and this is what makes that true rather than claimed.
//!
//! `docs/semantics/grammar.md` says what a well-formed eADL description **is**. This file reads
//! the EBNF block out of that document, builds a recognizer from it, and requires the recognizer
//! and `crates/eadl-front/src/reader.rs` to agree on every description the repository contains.
//!
//! ⛔ **The direction matters.** Before this, the reader was the only definition of the format and
//! every test validated against it, which made it unfalsifiable: there was no input that could
//! show the reader wrong, because whatever it did was by definition correct. A grammar that is
//! merely *written down* changes nothing — it becomes prose that drifts, which is the failure
//! `M1.9` (a profile of prose), `S0.1` (an unwritten comparison direction) and `M2.2` (five
//! unwritten semantics) each found at smaller scale. So the grammar is **executed**.
//!
//! ⭐ And as in `M2.2`, agreement is the weak result. The recognizer and the reader were written
//! years apart in intent — one from the corpus and the roadmap, one by hand — so a description
//! they classify differently is either a reader defect or a gap in the grammar. Those are the
//! findings; the agreements are the floor.
//!
//! ⚠️ **Honest limit**, narrowed once already by measurement. The first version of this file
//! compared only *acceptance*, and a mutation proved that too weak: dropping `_` from hexadecimal
//! literals left every test green while `(base 0x1000_0000)` became the two forms `4096` and
//! `_0000`. Token **segmentation** is now compared too, and that mutation fails.
//!
//! What remains outside: this does not establish that the reader builds the right *tree* from
//! those tokens, nor that it assigns the right *value* to each. A reader that tokenized
//! identically and mis-nested, or that read `1.5` as three halves, would still pass. Nesting is
//! the corpus suites' business (`corpus.rs`, the semantic cases); value exactness belongs to the
//! language reference, leaf `M1.12`.

use std::cell::RefCell;
use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use eadl_front::{read, Form, SourceMap};

mod common;

use common::reference_table::{machine_table, StringValue, Value};

/// The normative grammar, and the normative reference whose literal table it is compared against.
///
/// `include_str!` rather than `fs::read_to_string` for the reason `corpus.rs` gives: if either
/// document moves or is deleted, this crate **stops compiling** instead of silently gating nothing.
/// It also lets a RED arm mutate the grammar text and prove the comparison can fail on that side.
const GRAMMAR_DOCUMENT: &str = include_str!("../../../docs/semantics/grammar.md");
const REFERENCE: &str = include_str!("../../../docs/semantics/reference.md");

// ── the EBNF dialect ─────────────────────────────────────────────────────────────────────────
//
// Exactly the operators `docs/semantics/grammar.md` documents, and no others: a notation that can
// express more than the recognizer implements is a notation that will eventually be used to write
// something nobody checks.

#[derive(Debug, Clone, PartialEq, Eq)]
enum Expr {
    /// A literal string, matched exactly.
    Literal(String),
    /// An inclusive character range, `"a" .. "z"`.
    Range(char, char),
    /// Any one character.
    Any,
    /// End of input. Consumes nothing.
    End,
    /// Another production.
    Rule(String),
    /// `a , b` — concatenation.
    Seq(Vec<Expr>),
    /// `a | b` — ordered choice; the first alternative that matches wins.
    Choice(Vec<Expr>),
    /// `[ a ]`.
    Optional(Box<Expr>),
    /// `{ a }`.
    Repeat(Box<Expr>),
    /// `a - b` — matches `a`, then fails if the same text also matches `b`.
    Except(Box<Expr>, Box<Expr>),
    /// `? a` — must match, consumes nothing.
    Lookahead(Box<Expr>),
}

/// Split the EBNF into tokens. Small enough to be obviously right, which matters: a bug here
/// would make the grammar mean something other than what the document says.
fn lex(src: &str) -> Vec<String> {
    let mut out = Vec::new();
    let chars: Vec<char> = src.chars().collect();
    let mut i = 0;
    while i < chars.len() {
        let c = chars[i];
        if c.is_whitespace() {
            i += 1;
        } else if c == '"' || c == '\'' {
            // A quoted literal, with backslash escapes.
            let quote = c;
            let mut lit = String::from("\"");
            i += 1;
            while i < chars.len() && chars[i] != quote {
                if chars[i] == '\\' && i + 1 < chars.len() {
                    lit.push(match chars[i + 1] {
                        'n' => '\n',
                        't' => '\t',
                        'r' => '\r',
                        other => other,
                    });
                    i += 2;
                } else {
                    lit.push(chars[i]);
                    i += 1;
                }
            }
            i += 1;
            out.push(lit);
        } else if c == '.' && i + 1 < chars.len() && chars[i + 1] == '.' {
            out.push("..".into());
            i += 2;
        } else if "=;|()[]{}-?,".contains(c) {
            out.push(c.to_string());
            i += 1;
        } else if c.is_alphanumeric() || c == '_' {
            let start = i;
            while i < chars.len() && (chars[i].is_alphanumeric() || chars[i] == '_') {
                i += 1;
            }
            out.push(chars[start..i].iter().collect());
        } else {
            panic!("the EBNF block contains a character the notation does not define: {c:?}");
        }
    }
    out
}

struct Parser {
    tokens: Vec<String>,
    at: usize,
}

impl Parser {
    fn peek(&self) -> Option<&str> {
        self.tokens.get(self.at).map(String::as_str)
    }

    fn eat(&mut self, want: &str) -> bool {
        if self.peek() == Some(want) {
            self.at += 1;
            true
        } else {
            false
        }
    }

    /// `choice = seq , { "|" , seq }`
    fn choice(&mut self) -> Expr {
        let mut alts = vec![self.seq()];
        while self.eat("|") {
            alts.push(self.seq());
        }
        if alts.len() == 1 {
            alts.remove(0)
        } else {
            Expr::Choice(alts)
        }
    }

    /// `seq = term , { "," , term }`
    fn seq(&mut self) -> Expr {
        let mut items = vec![self.term()];
        while self.eat(",") {
            items.push(self.term());
        }
        if items.len() == 1 {
            items.remove(0)
        } else {
            Expr::Seq(items)
        }
    }

    /// A term, plus any trailing `.. ` range or `- exclusion` suffixes.
    fn term(&mut self) -> Expr {
        let mut base = self.primary();
        loop {
            if self.eat("..") {
                let Expr::Literal(low) = base else {
                    panic!("a range's lower bound must be a literal");
                };
                let Expr::Literal(high) = self.primary() else {
                    panic!("a range's upper bound must be a literal");
                };
                base = Expr::Range(
                    low.chars().next().expect("one character"),
                    high.chars().next().expect("one character"),
                );
            } else if self.eat("-") {
                base = Expr::Except(Box::new(base), Box::new(self.primary()));
            } else {
                return base;
            }
        }
    }

    fn primary(&mut self) -> Expr {
        if self.eat("?") {
            return Expr::Lookahead(Box::new(self.primary()));
        }
        if self.eat("(") {
            let inner = self.choice();
            assert!(self.eat(")"), "unclosed ( in the EBNF");
            return inner;
        }
        if self.eat("[") {
            let inner = self.choice();
            assert!(self.eat("]"), "unclosed [ in the EBNF");
            return Expr::Optional(Box::new(inner));
        }
        if self.eat("{") {
            let inner = self.choice();
            assert!(self.eat("}"), "unclosed {{ in the EBNF");
            return Expr::Repeat(Box::new(inner));
        }
        let token = self
            .tokens
            .get(self.at)
            .unwrap_or_else(|| panic!("the EBNF ended unexpectedly"))
            .clone();
        self.at += 1;
        if let Some(literal) = token.strip_prefix('"') {
            return Expr::Literal(literal.to_string());
        }
        match token.as_str() {
            "any" => Expr::Any,
            "end" => Expr::End,
            name => Expr::Rule(name.to_string()),
        }
    }
}

/// The grammar, read from the normative document.
struct Grammar {
    rules: BTreeMap<String, Expr>,
}

impl Grammar {
    fn load() -> Self {
        Self::from_document(GRAMMAR_DOCUMENT)
    }

    /// Build the recognizer from a document's fenced `ebnf` block.
    ///
    /// ⭐ Takes the text rather than a path, so a RED arm can feed it a **mutated** grammar and prove
    /// the comparison with the reference fails on the grammar's side too — not only when someone
    /// mistypes a row of the table.
    fn from_document(text: &str) -> Self {
        let block = text
            .split_once("```ebnf")
            .expect("grammar.md carries an ```ebnf block")
            .1
            .split_once("```")
            .expect("the ebnf block is closed")
            .0;

        // ⛔ Lex the WHOLE block before splitting on `;`. Splitting the raw text would cut the
        // `comment` production in half, because `;` is also a literal *in* the language being
        // described — caught by this file's own tests on their first run, which is the argument
        // for having them.
        let tokens = lex(block);
        let mut rules = BTreeMap::new();
        for production in tokens.split(|token| token == ";") {
            if production.is_empty() {
                continue;
            }
            assert!(
                production.len() >= 3 && production[1] == "=",
                "a production that is not `name = expr`: {production:?}"
            );
            let name = production[0].clone();
            let mut parser = Parser {
                tokens: production[2..].to_vec(),
                at: 0,
            };
            let expr = parser.choice();
            assert_eq!(
                parser.at,
                parser.tokens.len(),
                "trailing tokens in production `{name}`"
            );
            rules.insert(name, expr);
        }
        assert!(
            rules.contains_key("document"),
            "the grammar has no `document` production"
        );
        Self { rules }
    }

    /// Whether `input` conforms. Backtracking recognizer; the grammar is small and the corpus is
    /// tiny, so clarity is worth more than speed here.
    fn accepts(&self, input: &str) -> bool {
        self.segment(input).is_some()
    }

    /// Recognize `input` and return the **byte span of every atom** of the accepted parse, in
    /// source order — or `None` if it does not conform.
    ///
    /// ⭐ Acceptance alone is too weak a conformance claim, and that was measured rather than
    /// supposed. Mutating the reader to drop `_` from hexadecimal literals left every test green
    /// while `(base 0x1000_0000)` silently became the two forms `4096` and `_0000` — a
    /// memory-mapped base address of `0x10000000` read as `4096`, with both the grammar and the
    /// reader "accepting" the file. Two implementations can agree on the *language* and disagree
    /// on the *tokens*, and the second disagreement is the one that changes what a system means.
    fn segment(&self, input: &str) -> Option<Vec<(usize, usize)>> {
        let chars: Vec<char> = input.chars().collect();
        // The recognizer indexes characters; the reader reports byte offsets.
        let mut byte_at: Vec<usize> = Vec::with_capacity(chars.len() + 1);
        let mut offset = 0;
        for c in &chars {
            byte_at.push(offset);
            offset += c.len_utf8();
        }
        byte_at.push(offset);

        let atoms: RefCell<Vec<(usize, usize)>> = RefCell::new(Vec::new());
        // `document` ends with `end`, so full consumption is the grammar's requirement rather
        // than this function's.
        let ok = self.matches(
            &Expr::Rule("document".into()),
            &chars,
            0,
            &mut |_| true,
            &atoms,
        );
        ok.then(|| {
            let mut spans: Vec<(usize, usize)> = atoms
                .borrow()
                .iter()
                .map(|(start, end)| (byte_at[*start], byte_at[*end]))
                .collect();
            spans.sort_unstable();
            spans
        })
    }

    /// Match `expr` at `at`, calling `k` with **every** end position it could produce, and
    /// returning whether any continuation succeeded.
    ///
    /// ⛔ **Continuation-passing, not "return one end position".** The first version of this
    /// returned a single `Option<usize>`, which cannot backtrack out of an alternative once a
    /// later term fails: for `atom = ( string | number | symbol ) , ? delimiter` on the input
    /// `10ms`, `number` matched `10`, the lookahead then failed, and the recognizer gave up
    /// without ever trying `symbol`. It produced the right verdict for the wrong reason — and a
    /// red arm that weakened `symbol_start` did not fire, which is how the defect was found. A
    /// recognizer whose language differs from the document's is worse than no recognizer, because
    /// the document is normative and nothing would have been checking it.
    fn matches(
        &self,
        expr: &Expr,
        input: &[char],
        at: usize,
        k: &mut dyn FnMut(usize) -> bool,
        atoms: &RefCell<Vec<(usize, usize)>>,
    ) -> bool {
        match expr {
            Expr::Any => at < input.len() && k(at + 1),
            Expr::End => at == input.len() && k(at),
            Expr::Literal(text) => {
                let want: Vec<char> = text.chars().collect();
                input.len() >= at + want.len()
                    && input[at..at + want.len()] == want[..]
                    && k(at + want.len())
            }
            Expr::Range(low, high) => {
                matches!(input.get(at), Some(c) if low <= c && c <= high) && k(at + 1)
            }
            Expr::Rule(name) => {
                let rule = self
                    .rules
                    .get(name)
                    .unwrap_or_else(|| panic!("the grammar references an undefined rule `{name}`"));
                if name != "atom" {
                    return self.matches(rule, input, at, k, atoms);
                }
                // Record the atom only while the parse that contains it is still alive: push
                // before continuing, pop if the continuation ultimately fails. Without the pop a
                // backtracked branch would leave phantom tokens behind.
                self.matches(
                    rule,
                    input,
                    at,
                    &mut |end| {
                        atoms.borrow_mut().push((at, end));
                        if k(end) {
                            true
                        } else {
                            atoms.borrow_mut().pop();
                            false
                        }
                    },
                    atoms,
                )
            }
            Expr::Seq(items) => self.match_seq(items, input, at, k, atoms),
            Expr::Choice(alts) => alts
                .iter()
                .any(|alt| self.matches(alt, input, at, k, atoms)),
            // Greedy first, then empty — with full backtracking the order only decides which
            // successful parse is found first, never whether one is found.
            Expr::Optional(inner) => self.matches(inner, input, at, k, atoms) || k(at),
            Expr::Repeat(inner) => self.match_repeat(inner, input, at, k, atoms),
            Expr::Except(base, forbidden) => self.matches(
                base,
                input,
                at,
                &mut |end| {
                    let mut excluded = false;
                    self.matches(
                        forbidden,
                        input,
                        at,
                        &mut |bad| {
                            excluded |= bad == end;
                            false
                        },
                        atoms,
                    );
                    !excluded && k(end)
                },
                atoms,
            ),
            // Consumes nothing, and only needs ONE way to match.
            Expr::Lookahead(inner) => {
                let mut ok = false;
                self.matches(
                    inner,
                    input,
                    at,
                    &mut |_| {
                        ok = true;
                        true
                    },
                    atoms,
                );
                ok && k(at)
            }
        }
    }

    fn match_seq(
        &self,
        items: &[Expr],
        input: &[char],
        at: usize,
        k: &mut dyn FnMut(usize) -> bool,
        atoms: &RefCell<Vec<(usize, usize)>>,
    ) -> bool {
        match items.split_first() {
            None => k(at),
            Some((head, rest)) => self.matches(
                head,
                input,
                at,
                &mut |next| self.match_seq(rest, input, next, k, atoms),
                atoms,
            ),
        }
    }

    fn match_repeat(
        &self,
        inner: &Expr,
        input: &[char],
        at: usize,
        k: &mut dyn FnMut(usize) -> bool,
        atoms: &RefCell<Vec<(usize, usize)>>,
    ) -> bool {
        // More first, then stop. The `next > at` guard is what keeps a nullable body from
        // looping forever.
        let more = self.matches(
            inner,
            input,
            at,
            &mut |next| next > at && self.match_repeat(inner, input, next, k, atoms),
            atoms,
        );
        more || k(at)
    }
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// Every `.eadl` file the repository contains — the grammar's conformance suite.
fn corpus() -> Vec<(String, String)> {
    let root = repo_root();
    let mut found = Vec::new();
    let mut stack = vec![root.join("docs/semantics"), root.join("examples")];
    while let Some(dir) = stack.pop() {
        let Ok(entries) = std::fs::read_dir(&dir) else {
            continue;
        };
        for entry in entries.filter_map(Result::ok) {
            let path = entry.path();
            if path.is_dir() {
                stack.push(path);
            } else if path.extension().is_some_and(|e| e == "eadl") {
                let relative = path
                    .strip_prefix(&root)
                    .expect("inside the repository")
                    .display()
                    .to_string();
                found.push((relative, std::fs::read_to_string(&path).expect("readable")));
            }
        }
    }
    found.sort();
    found
}

/// Run `body` with a large stack.
///
/// ⛔ A continuation-passing backtracking recognizer recurses roughly once per input character,
/// and corpus descriptions run to a few thousand. The default 2 MiB test stack overflows on them
/// — measured, not anticipated. Growing the stack is the honest fix: the alternative is
/// rewriting the recognizer into an explicit machine, which would make it harder to check
/// against the notation it is supposed to implement, and *that* correspondence is the whole
/// point of this file.
fn with_deep_stack<T: Send + 'static>(body: impl FnOnce() -> T + Send + 'static) -> T {
    std::thread::Builder::new()
        .stack_size(256 * 1024 * 1024)
        .spawn(body)
        .expect("a thread")
        .join()
        .expect("the recognizer did not panic")
}

/// Whether the shipped reader accepts `text` without a diagnostic.
fn reader_accepts(name: &str, text: &str) -> bool {
    let mut sources = SourceMap::new();
    let id = sources.add(name, text.to_string()).expect("small");
    let (_, diagnostics) = read(&sources, id);
    !diagnostics.has_errors()
}

#[test]
fn the_grammar_document_parses_as_the_notation_it_documents() {
    // If this fails, the normative document contains something the notation cannot express — so
    // the grammar says more than anything checks, which is the state this leaf exists to end.
    let grammar = Grammar::load();
    assert!(
        grammar.rules.len() >= 15,
        "{} productions",
        grammar.rules.len()
    );
    for name in [
        "document", "form", "list", "atom", "symbol", "number", "string",
    ] {
        assert!(grammar.rules.contains_key(name), "no `{name}` production");
    }
}

#[test]
fn the_grammar_and_the_reader_agree_on_the_whole_corpus() {
    with_deep_stack(|| {
        // ⭐ The load-bearing test. Every description the repository ships must be accepted by both.
        let grammar = Grammar::load();
        let corpus = corpus();
        assert!(
            corpus.len() >= 50,
            "only {} descriptions found",
            corpus.len()
        );

        let mut disagreements = Vec::new();
        for (name, text) in &corpus {
            let by_grammar = grammar.accepts(text);
            let by_reader = reader_accepts(name, text);
            if by_grammar != by_reader {
                disagreements.push(format!(
                    "  {name}: grammar {} / reader {}",
                    if by_grammar { "accepts" } else { "rejects" },
                    if by_reader { "accepts" } else { "rejects" }
                ));
            }
        }
        assert!(
            disagreements.is_empty(),
            "{} of {} descriptions are classified differently by the normative grammar and the \
         reader. Each is either a reader defect or a gap in docs/semantics/grammar.md:\n{}",
            disagreements.len(),
            corpus.len(),
            disagreements.join("\n")
        );
    });
}

#[test]
fn the_grammar_and_the_reader_agree_on_malformed_input() {
    with_deep_stack(|| {
        // Acceptance agreement is half the claim; both must also reject the same things, or the
        // grammar is merely permissive enough to have agreed by accident.
        let grammar = Grammar::load();
        let malformed = [
            ("unclosed list", "(defblock a"),
            ("stray close", "(defblock a))"),
            ("unterminated string", "(a \"unfinished)"),
            ("digit-initial symbol", "(period 10ms)"),
            ("bare number with unit glued", "(counter-width 32bit)"),
            ("unterminated string at eof", "(a \""),
        ];
        let mut wrong = Vec::new();
        for (why, text) in malformed {
            let by_grammar = grammar.accepts(text);
            let by_reader = reader_accepts("malformed", text);
            if by_grammar || by_reader {
                wrong.push(format!(
                    "  {why} ({text:?}): grammar {} / reader {}",
                    if by_grammar { "ACCEPTS" } else { "rejects" },
                    if by_reader { "ACCEPTS" } else { "rejects" }
                ));
            }
        }
        assert!(
            wrong.is_empty(),
            "malformed input was accepted:\n{}",
            wrong.join("\n")
        );
    });
}

#[test]
fn the_recognizer_is_not_vacuously_permissive() {
    with_deep_stack(|| {
        // ⛔ A recognizer that accepted everything would make the agreement test pass and prove
        // nothing — the blind-spot failure recorded in
        // docs/knowledge/a-gate-is-only-as-sharp-as-its-fixtures.md. These must be rejected.
        let grammar = Grammar::load();
        for text in ["(", ")", "(a", "a)", "\"", "(a \"b)", "((("] {
            assert!(!grammar.accepts(text), "the grammar accepts {text:?}");
        }
        // …and these must be accepted, or it is vacuously restrictive instead.
        for text in [
            "",
            "()",
            "(a)",
            "(a b)",
            "; just a comment\n",
            "(a 1 -2 0x1f 1.5 \"s\")",
        ] {
            assert!(grammar.accepts(text), "the grammar rejects {text:?}");
        }
    });
}

/// Every atom span the reader produced, in source order.
fn reader_atoms(name: &str, text: &str) -> Option<Vec<(usize, usize)>> {
    let mut sources = SourceMap::new();
    let id = sources.add(name, text.to_string()).expect("small");
    let (document, diagnostics) = read(&sources, id);
    if diagnostics.has_errors() {
        return None;
    }
    let mut spans = Vec::new();
    fn walk(form: &Form, spans: &mut Vec<(usize, usize)>) {
        match form {
            Form::List { items, .. } => {
                for item in items {
                    walk(item, spans);
                }
            }
            other => {
                let span = other.span();
                spans.push((span.start as usize, span.end as usize));
            }
        }
    }
    for form in &document.forms {
        walk(form, &mut spans);
    }
    spans.sort_unstable();
    Some(spans)
}

#[test]
fn the_grammar_and_the_reader_tokenize_the_corpus_identically() {
    // ⭐ The strongest claim this file makes, and the one acceptance-agreement could not support.
    //
    // Two implementations can agree on the *language* and still disagree on the *tokens*, and the
    // token disagreement is the one that changes what a system means. Measured: with `_` dropped
    // from hexadecimal literals, the reader turned `(base 0x1000_0000)` into the two forms `4096`
    // and `_0000` — a base address of 0x10000000 read as 4096 — and every acceptance test stayed
    // green. Comparing segmentation is what closes that.
    with_deep_stack(|| {
        let grammar = Grammar::load();
        let corpus = corpus();
        let mut wrong = Vec::new();
        for (name, text) in &corpus {
            let (Some(by_grammar), Some(by_reader)) =
                (grammar.segment(text), reader_atoms(name, text))
            else {
                continue; // acceptance disagreements are the other test's business
            };
            if by_grammar != by_reader {
                let first = by_grammar
                    .iter()
                    .zip(&by_reader)
                    .find(|(a, b)| a != b)
                    .map_or_else(
                        || "differing token counts".to_string(),
                        |((gs, ge), (rs, re))| {
                            format!(
                                "grammar sees {:?} at {gs}..{ge}, reader sees {:?} at {rs}..{re}",
                                &text[*gs..*ge],
                                &text[*rs..*re]
                            )
                        },
                    );
                wrong.push(format!(
                    "  {name}: {} tokens vs {} — {first}",
                    by_grammar.len(),
                    by_reader.len()
                ));
            }
        }
        assert!(
            wrong.is_empty(),
            "{} of {} descriptions are tokenized differently by the normative grammar and the \
             reader:\n{}",
            wrong.len(),
            corpus.len(),
            wrong.join("\n")
        );
    });
}

#[test]
fn the_conformance_probes_exercise_every_production() {
    // ⛔ A COVERAGE FLOOR, added because the corpus turned out not to be one. The whole corpus
    // contains exactly ONE number with a digit separator (`0x1000_0000`, hexadecimal), so nothing
    // in it exercised decimal separators, signs, escapes, or several other productions — and a
    // conformance suite that never reaches a production is not evidence about it. These probes
    // are the suite proper; the corpus is the regression set.
    with_deep_stack(|| {
        let grammar = Grammar::load();
        let probes: &[(&str, &str)] = &[
            ("empty document", ""),
            ("comment only", "; nothing here\n"),
            ("comment without a newline at eof", "; unterminated comment"),
            ("empty list", "()"),
            ("nested lists", "(a (b (c)))"),
            ("symbol with punctuation", "(a.b-c/d:e)"),
            ("operator-shaped symbols", "(= >= <= != *)"),
            ("plain integer", "(n 42)"),
            ("signed integers", "(n -1 +2)"),
            ("integer with separators", "(n 1_000_000)"),
            ("decimal", "(n 1.5)"),
            ("decimal with separators", "(n 1_0.0_5)"),
            ("signed decimal", "(n -0.25)"),
            ("hexadecimal", "(n 0xff)"),
            ("hexadecimal with separators", "(n 0x1000_0000)"),
            ("uppercase hexadecimal", "(n 0xDEADBEEF)"),
            ("empty string", "(s \"\")"),
            ("string with escapes", "(s \"a\\\"b\\\\c\\nd\\te\")"),
            ("string containing delimiters", "(s \"(;)\")"),
            ("crlf line endings", "(a)\r\n(b)\r\n"),
            ("tabs as whitespace", "(a\tb)"),
            ("comment inside a list", "(a ; why\n b)"),
            ("top-level atom", "bare"),
        ];
        let mut wrong = Vec::new();
        for (why, text) in probes {
            let by_grammar = grammar.accepts(text);
            let by_reader = reader_accepts("probe", text);
            if !by_grammar || !by_reader {
                wrong.push(format!(
                    "  {why} ({text:?}): grammar {} / reader {}",
                    if by_grammar { "accepts" } else { "REJECTS" },
                    if by_reader { "accepts" } else { "REJECTS" }
                ));
            }
        }
        assert!(
            wrong.is_empty(),
            "conformance probes that should be well-formed were not accepted:\n{}",
            wrong.join("\n")
        );
    });
}

// ── the literal space: the grammar and the reference must agree ────────────────────────────────
//
// `M1.12.2`. The properties above compared this recognizer with the reader over the corpus and over
// one probe per production, and three literal forms still disagreed with
// `docs/semantics/reference.md`: `0X10` and `0x_10`, which the reader read and the grammar refused,
// and `\0`, which the grammar admitted and the reader refused. None of the three appears in any
// description the repository ships — three `git grep` censuses, three empty results — so no input
// either check had could reach them.
//
// ⭐ The population therefore comes from the reference's own table rather than from the corpus. That
// is the only way to enumerate a *space* rather than a sample: the table is a systematic enumeration
// of literal forms, and this leg asks the grammar for a verdict on every one of them. It is also the
// remedy for the blind spot the probes have — one probe per production never combines two, and every
// one of these divergences lived in a combination.

/// What the grammar said about every literal the reference states, and where the two disagree.
struct LiteralSpace {
    /// Rows the recognizer accepted.
    accepted: usize,
    /// Rows the recognizer rejected.
    rejected: usize,
    /// One message per disagreement.
    violations: Vec<String>,
}

/// Compare the recognizer's verdict with the reference's, for every row of both literal tables.
///
/// No `with_deep_stack`: the inputs are single literals, and the deep stack exists for corpus
/// descriptions thousands of characters long, where a continuation-passing recognizer recurses once
/// per character.
fn literal_space(reference: &str, grammar: &Grammar) -> LiteralSpace {
    let mut space = LiteralSpace {
        accepted: 0,
        rejected: 0,
        violations: Vec::new(),
    };
    for (line, literal, well_formed) in literal_rows(reference) {
        let accepted = grammar.accepts(&literal);
        if accepted {
            space.accepted += 1;
        } else {
            space.rejected += 1;
        }
        if accepted != well_formed {
            space.violations.push(format!(
                "docs/semantics/reference.md:{line}: the reference says `{literal}` is {}, and the \
                 recognizer derived from docs/semantics/grammar.md {} it",
                if well_formed {
                    "well-formed"
                } else {
                    "NOT well-formed"
                },
                if accepted { "accepts" } else { "rejects" }
            ));
        }
    }
    space
}

/// Every literal row of both tables, as `(line, literal, whether the reference calls it well-formed)`.
///
/// Rows this file cannot read are skipped rather than reported: a cell the notation cannot express is
/// `reference.rs`'s complaint, and reporting it here too would make one defect look like two.
fn literal_rows(reference: &str) -> Vec<(usize, String, bool)> {
    let mut rows = Vec::new();
    for (line, cells) in machine_table(reference, "number-values") {
        let (Some(literal), Some(stated)) = (cells.first(), cells.get(1)) else {
            continue;
        };
        if let Some(value) = Value::parse(stated) {
            rows.push((line, literal.clone(), value.well_formed()));
        }
    }
    for (line, cells) in machine_table(reference, "string-values") {
        let (Some(literal), Some(stated)) = (cells.first(), cells.get(1)) else {
            continue;
        };
        if let Some(value) = StringValue::parse(stated) {
            rows.push((line, literal.clone(), value.well_formed()));
        }
    }
    rows
}

#[test]
fn the_grammar_and_the_reference_agree_on_the_literal_space() {
    let grammar = Grammar::load();
    let space = literal_space(REFERENCE, &grammar);

    // ⛔ Not vacuous, in both directions. A leg over zero rows proves nothing, and a leg in which the
    // recognizer gave only one verdict would pass on a recognizer that accepts everything or rejects
    // everything — the same false green `the_recognizer_is_not_vacuously_permissive` refuses.
    assert!(
        space.accepted > 0 && space.rejected > 0,
        "the literal tables gave the recognizer nothing to discriminate: {} accepted, {} rejected",
        space.accepted,
        space.rejected
    );
    assert!(
        space.violations.is_empty(),
        "the normative grammar and the normative reference disagree about the literal space:\n\n{}\n\n\
         One of the two documents states a rule the language does not have. They are both normative, \
         so this is a decision to make and record, not a row to adjust until the test goes quiet.",
        space.violations.join("\n\n")
    );
}

// Each arm mutates one side and asserts the specific complaint, with the count pinned. Arms 2 and 3
// are the two divergences this leaf was written for, restored: they prove that reverting either fix
// fails the build, which is the difference between a repaired defect and an impossible one.

#[test]
fn arm_1_a_reference_row_that_flips_its_verdict_is_reported() {
    let grammar = Grammar::load();
    let mutated = REFERENCE.replace(
        "| `0X10` | `integer 16` | `16` |",
        "| `0X10` | `error read-malformed-number` | — |",
    );
    assert_ne!(
        mutated, REFERENCE,
        "the mutation did not apply — a false green"
    );
    let space = literal_space(&mutated, &grammar);
    assert_eq!(
        space.violations.len(),
        1,
        "expected one violation; got:\n{}",
        space.violations.join("\n\n")
    );
    assert!(
        space.violations[0].contains("`0X10` is NOT well-formed"),
        "the violation does not name the row and the claim: {}",
        space.violations[0]
    );
}

#[test]
fn arm_2_a_grammar_that_stops_admitting_the_uppercase_prefix_is_reported() {
    // Finding F-A restored: the reader has always read `0X10`, and the grammar refused it.
    let mutated = GRAMMAR_DOCUMENT.replace(r#"( "0x" | "0X" ) , hex_digit"#, r#""0x" , hex_digit"#);
    assert_ne!(
        mutated, GRAMMAR_DOCUMENT,
        "the mutation did not apply — a false green"
    );
    let grammar = Grammar::from_document(&mutated);
    assert!(
        !grammar.accepts("0X10"),
        "the mutated grammar still accepts `0X10`, so this arm proves nothing"
    );
    let space = literal_space(REFERENCE, &grammar);
    assert_eq!(
        space.violations.len(),
        1,
        "expected one violation; got:\n{}",
        space.violations.join("\n\n")
    );
    assert!(
        space.violations[0].contains("`0X10` is well-formed"),
        "the violation does not name the row and the claim: {}",
        space.violations[0]
    );
}

#[test]
fn arm_3_a_grammar_that_admits_a_null_escape_is_reported() {
    // Finding F-C restored: `escape` used to admit `"0"`, and the reader has never implemented `\0`.
    let mutated = GRAMMAR_DOCUMENT.replace(
        r#"escape          = "\\" , ( quote | "\\" | "n" | "t" | "r" ) ;"#,
        r#"escape          = "\\" , ( quote | "\\" | "n" | "t" | "r" | "0" ) ;"#,
    );
    assert_ne!(
        mutated, GRAMMAR_DOCUMENT,
        "the mutation did not apply — a false green"
    );
    let grammar = Grammar::from_document(&mutated);
    assert!(
        grammar.accepts(r#""nul\0here""#),
        "the mutated grammar still rejects `\\0`, so this arm proves nothing"
    );
    let space = literal_space(REFERENCE, &grammar);
    assert_eq!(
        space.violations.len(),
        1,
        "expected one violation; got:\n{}",
        space.violations.join("\n\n")
    );
    assert!(
        space.violations[0].contains("NOT well-formed"),
        "the violation does not name the claim: {}",
        space.violations[0]
    );
}
