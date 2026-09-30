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
use std::collections::{BTreeMap, BTreeSet};
use std::path::{Path, PathBuf};

use eadl_front::{read, Form, SourceMap};

mod common;

use common::reference_table::{machine_table, source_text, StringValue, Value};

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
                    // ⭐ `\u{…}` is the same escape the language has, and it is what lets the notation
                    // name a control character at all: `control` is a set of ranges whose bounds are
                    // invisible bytes, and a grammar that cannot write one cannot exclude one. The
                    // supported escapes in an EBNF literal are therefore the language's own, which
                    // keeps one spelling of `\u{1b}` meaning one thing in both documents.
                    if chars[i + 1] == 'u' && i + 2 < chars.len() && chars[i + 2] == '{' {
                        let mut at = i + 3;
                        let mut digits = String::new();
                        while at < chars.len() && chars[at] != '}' {
                            digits.push(chars[at]);
                            at += 1;
                        }
                        assert!(at < chars.len(), "an EBNF literal has an unclosed `\\u{{`");
                        let code = u32::from_str_radix(&digits, 16).unwrap_or_else(|_| {
                            panic!("`\\u{{{digits}}}` in the EBNF is not hexadecimal")
                        });
                        lit.push(
                            char::from_u32(code)
                                .unwrap_or_else(|| panic!("`\\u{{{digits}}}` names no character")),
                        );
                        i = at + 1;
                        continue;
                    }
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
        let recognition = self.recognize(input);
        recognition.accepted.then_some(recognition.atoms)
    }

    /// Recognize `input` and report what the recognition reached (leaf `M1.27`): the atoms and the
    /// productions of the **accepted** derivation, and every production that fired as an exclusion.
    ///
    /// ⭐ Two kinds of reach, because the grammar has two kinds of production. Most are *used*: they
    /// match text in the derivation that wins. `control` is only ever *excluded*
    /// (`any - quote - "\\" - control`), so no accepted derivation can contain it. It is reached when
    /// the exclusion fires — when it matches the very text the base matched, and so refuses it.
    fn recognize(&self, input: &str) -> Recognition {
        let chars: Vec<char> = input.chars().collect();
        // The recognizer indexes characters; the reader reports byte offsets.
        let mut byte_at: Vec<usize> = Vec::with_capacity(chars.len() + 1);
        let mut offset = 0;
        for c in &chars {
            byte_at.push(offset);
            offset += c.len_utf8();
        }
        byte_at.push(offset);

        let trace = Trace::default();
        // `document` ends with `end`, so full consumption is the grammar's requirement rather
        // than this function's.
        let accepted = self.matches(
            &Expr::Rule("document".into()),
            &chars,
            0,
            &mut |_| true,
            &trace,
        );
        let mut atoms: Vec<(usize, usize)> = trace
            .atoms
            .borrow()
            .iter()
            .map(|(start, end)| (byte_at[*start], byte_at[*end]))
            .collect();
        atoms.sort_unstable();
        let used = if accepted {
            trace.rules.borrow().iter().map(|r| r.to_string()).collect()
        } else {
            BTreeSet::new()
        };
        let excluded = trace
            .exclusions
            .borrow()
            .iter()
            .map(|r| r.to_string())
            .collect();
        Recognition {
            accepted,
            atoms,
            used,
            excluded,
        }
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
    fn matches<'g>(
        &'g self,
        expr: &Expr,
        input: &[char],
        at: usize,
        k: &mut dyn FnMut(usize) -> bool,
        trace: &Trace<'g>,
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
                let (name, rule) = self
                    .rules
                    .get_key_value(name)
                    .unwrap_or_else(|| panic!("the grammar references an undefined rule `{name}`"));
                // Record the production, and the atom if it is one, only while the parse that
                // contains it is still alive: push before continuing, roll back if the continuation
                // ultimately fails. Without the rollback a backtracked branch would leave phantom
                // tokens behind, and phantom coverage: `sign` tried on `-` and abandoned for
                // `symbol` would count as reached.
                self.matches(
                    rule,
                    input,
                    at,
                    &mut |end| {
                        let mark = trace.mark();
                        trace.rules.borrow_mut().push(name);
                        if name == "atom" {
                            trace.atoms.borrow_mut().push((at, end));
                        }
                        if k(end) {
                            true
                        } else {
                            trace.rollback(mark);
                            false
                        }
                    },
                    trace,
                )
            }
            Expr::Seq(items) => self.match_seq(items, input, at, k, trace),
            Expr::Choice(alts) => alts
                .iter()
                .any(|alt| self.matches(alt, input, at, k, trace)),
            // Greedy first, then empty — with full backtracking the order only decides which
            // successful parse is found first, never whether one is found.
            Expr::Optional(inner) => self.matches(inner, input, at, k, trace) || k(at),
            Expr::Repeat(inner) => self.match_repeat(inner, input, at, k, trace),
            Expr::Except(base, forbidden) => self.matches(
                base,
                input,
                at,
                &mut |end| {
                    let mut excluded = false;
                    let mark = trace.mark();
                    self.matches(
                        forbidden,
                        input,
                        at,
                        &mut |bad| {
                            if bad == end {
                                excluded = true;
                                // What refused the text: the productions of the forbidden match.
                                let fired = trace.rules.borrow()[mark.0..].to_vec();
                                trace.exclusions.borrow_mut().extend(fired);
                            }
                            false
                        },
                        trace,
                    );
                    !excluded && k(end)
                },
                trace,
            ),
            // Consumes nothing, and only needs ONE way to match. Its continuation answers `true`
            // whatever follows, so what it recorded is rolled back here if what follows fails.
            Expr::Lookahead(inner) => {
                let mark = trace.mark();
                let mut ok = false;
                self.matches(
                    inner,
                    input,
                    at,
                    &mut |_| {
                        ok = true;
                        true
                    },
                    trace,
                );
                if ok && k(at) {
                    true
                } else {
                    trace.rollback(mark);
                    false
                }
            }
        }
    }

    fn match_seq<'g>(
        &'g self,
        items: &[Expr],
        input: &[char],
        at: usize,
        k: &mut dyn FnMut(usize) -> bool,
        trace: &Trace<'g>,
    ) -> bool {
        match items.split_first() {
            None => k(at),
            Some((head, rest)) => self.matches(
                head,
                input,
                at,
                &mut |next| self.match_seq(rest, input, next, k, trace),
                trace,
            ),
        }
    }

    fn match_repeat<'g>(
        &'g self,
        inner: &Expr,
        input: &[char],
        at: usize,
        k: &mut dyn FnMut(usize) -> bool,
        trace: &Trace<'g>,
    ) -> bool {
        // More first, then stop. The `next > at` guard is what keeps a nullable body from
        // looping forever.
        let more = self.matches(
            inner,
            input,
            at,
            &mut |next| next > at && self.match_repeat(inner, input, next, k, trace),
            trace,
        );
        more || k(at)
    }
}

/// What one recognition leaves behind, kept only while the derivation that made it is alive.
#[derive(Default)]
struct Trace<'g> {
    /// Every atom of the live derivation, as character offsets.
    atoms: RefCell<Vec<(usize, usize)>>,
    /// Every production the live derivation completed, in completion order.
    rules: RefCell<Vec<&'g str>>,
    /// Every production that matched the text an exclusion refused. Never rolled back: a firing
    /// is a firing, whether or not the recognition went on to accept.
    exclusions: RefCell<BTreeSet<&'g str>>,
}

impl Trace<'_> {
    fn mark(&self) -> (usize, usize) {
        (self.rules.borrow().len(), self.atoms.borrow().len())
    }

    fn rollback(&self, (rules, atoms): (usize, usize)) {
        self.rules.borrow_mut().truncate(rules);
        self.atoms.borrow_mut().truncate(atoms);
    }
}

/// The verdict on one input, and what reaching it reached.
struct Recognition {
    accepted: bool,
    /// The atoms of the accepted derivation, as byte spans in source order; empty if refused.
    atoms: Vec<(usize, usize)>,
    /// The productions of the accepted derivation; empty if refused.
    used: BTreeSet<String>,
    /// The productions that fired as an exclusion anywhere in the recognition.
    excluded: BTreeSet<String>,
}

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(Path::parent)
        .expect("crates/<name>/ is two levels below the root")
        .to_path_buf()
}

/// The conformance suite: the population `docs/semantics/conformance.md` declares.
///
/// ⛔ **Enumerated from the manifest, not from a root list restated here.** This function used to walk
/// `docs/semantics` and `examples` itself, and `reference.rs` walked the same two directories in its own
/// copy — so the suite's scope was a claim in two files and a rule in neither, and a description added
/// under a third directory would have been outside one walk, both, or neither depending on which author
/// remembered. `common::suite` is the one reader; `conformance_suite.rs` checks the manifest's own rules.
fn corpus() -> Vec<(String, String)> {
    common::suite::population(&repo_root()).unwrap_or_else(|problems| {
        panic!(
            "{} declares a suite that cannot be enumerated, so this file would be green on an empty \
             population:\n\n{}",
            common::suite::MANIFEST_PATH,
            problems.join("\n")
        )
    })
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

/// Inputs both the grammar and the reader must refuse. Each also reaches what refuses it: the
/// control-character probe is the one that fires `control`'s exclusion (`M1.27`).
const MALFORMED: &[(&str, &str)] = &[
    ("unclosed list", "(defblock a"),
    ("stray close", "(defblock a))"),
    ("unterminated string", "(a \"unfinished)"),
    ("digit-initial symbol", "(period 10ms)"),
    ("bare number with unit glued", "(counter-width 32bit)"),
    ("unterminated string at eof", "(a \""),
    ("string glued to a symbol", "(\"b\"c)"),
    ("string glued to a number", "(a \"b\"1)"),
    ("raw control character in a string", "(s \"a\u{7}b\")"),
];

/// The well-formed probes: a coverage floor the corpus is not (see the test that runs them).
const PROBES: &[(&str, &str)] = &[
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
    ("string with a general escape", "(s \"a\\u{1b}b\")"),
    ("raw tab inside a string", "(s \"a\tb\")"),
    ("string containing delimiters", "(s \"(;)\")"),
    ("adjacent strings", "(s \"a\"\"b\")"),
    ("a string then a list", "(s \"a\"(b))"),
    ("crlf line endings", "(a)\r\n(b)\r\n"),
    ("tabs as whitespace", "(a\tb)"),
    ("comment inside a list", "(a ; why\n b)"),
    ("top-level atom", "bare"),
];

/// Every production of `grammar`, with the probes that reach it (`M1.27`). A well-formed probe
/// reaches the productions its accepted derivation uses and any exclusion that fired on the way; a
/// malformed one reaches only the exclusions that fired, since a refused input has no derivation.
/// A production no probe reaches maps to an empty list.
fn reach(
    grammar: &Grammar,
    probes: &[(&str, &str)],
    malformed: &[(&str, &str)],
) -> BTreeMap<String, Vec<String>> {
    let mut reach: BTreeMap<String, Vec<String>> = grammar
        .rules
        .keys()
        .map(|name| (name.clone(), Vec::new()))
        .collect();
    for (why, text) in probes.iter().chain(malformed) {
        let recognition = grammar.recognize(text);
        for name in recognition.used.iter().chain(&recognition.excluded) {
            let probes = reach.get_mut(name).expect("a production the grammar has");
            if !probes.iter().any(|p| p == why) {
                probes.push(why.to_string());
            }
        }
    }
    reach
}

/// The productions `reach` found no probe for.
fn unreached(reach: &BTreeMap<String, Vec<String>>) -> Vec<&str> {
    reach
        .iter()
        .filter(|(_, probes)| probes.is_empty())
        .map(|(name, _)| name.as_str())
        .collect()
}

#[test]
fn the_grammar_and_the_reader_agree_on_malformed_input() {
    with_deep_stack(|| {
        // Acceptance agreement is half the claim; both must also reject the same things, or the
        // grammar is merely permissive enough to have agreed by accident.
        let grammar = Grammar::load();
        let mut wrong = Vec::new();
        for (why, text) in MALFORMED {
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
        let mut wrong = Vec::new();
        for (why, text) in PROBES {
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

        // ⭐ The name's claim, measured (`M1.27`): until this leg the test asserted only that each
        // probe is accepted, and nothing connected a probe to a production.
        let reach = reach(&grammar, PROBES, MALFORMED);
        assert!(!reach.is_empty(), "the grammar has no productions to reach");
        // The map itself, for `--nocapture`: the evidence a reviewer reads, not a figure to copy.
        for (production, probes) in &reach {
            println!("reach: {production:<16} {}", probes.join(" · "));
        }
        let missing = unreached(&reach);
        assert!(
            missing.is_empty(),
            "{} of {} productions are reached by no probe — add one that uses each, or for an \
             exclusion one it refuses: {missing:?}",
            missing.len(),
            reach.len()
        );
    });
}

#[test]
fn a_production_tried_and_abandoned_is_not_reached() {
    with_deep_stack(|| {
        // `-` alone is a symbol: `number` tries `sign` on it, finds no digit, and gives it up.
        // Without the rollback in `Grammar::matches` that abandoned `sign` would count as reached,
        // and the coverage leg would be satisfied by a branch no derivation kept.
        let grammar = Grammar::load();
        let alone = grammar.recognize("(a -)");
        assert!(alone.accepted);
        assert!(alone.used.contains("symbol"), "{:?}", alone.used);
        assert!(!alone.used.contains("sign"), "{:?}", alone.used);
        let signed = grammar.recognize("(a -1)");
        assert!(signed.used.contains("sign"), "{:?}", signed.used);
        // …and a refused input has no derivation, so nothing is used by it.
        assert!(grammar.recognize("(a").used.is_empty());
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
    for (line, cell, text, well_formed) in literal_rows(reference) {
        let accepted = grammar.accepts(&text);
        if accepted {
            space.accepted += 1;
        } else {
            space.rejected += 1;
        }
        if accepted != well_formed {
            space.violations.push(format!(
                "docs/semantics/reference.md:{line}: the reference says `{cell}` is {}, and the \
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

/// Every literal row of both tables, as `(line, the cell as written, the text it denotes, whether the
/// reference calls it well-formed)`.
///
/// The cell and the text it denotes are carried separately, because a row about a raw control byte is
/// written `<0x1b>` and feeding *that* to the recognizer would ask it about the wrong input — while
/// printing the decoded byte in a violation message would put an invisible character in the failure
/// output. `source_text` is the one decoder, shared with `reference.rs`.
///
/// Rows this file cannot read are skipped rather than reported: a cell the notation cannot express is
/// `reference.rs`'s complaint, and reporting it here too would make one defect look like two.
fn literal_rows(reference: &str) -> Vec<(usize, String, String, bool)> {
    let mut rows = Vec::new();
    for (line, cells) in machine_table(reference, "number-values") {
        let (Some(literal), Some(stated)) = (cells.first(), cells.get(1)) else {
            continue;
        };
        if let (Some(value), Some(text)) = (Value::parse(stated), source_text(literal)) {
            rows.push((line, literal.clone(), text, value.well_formed()));
        }
    }
    for (line, cells) in machine_table(reference, "string-values") {
        let (Some(literal), Some(stated)) = (cells.first(), cells.get(1)) else {
            continue;
        };
        if let (Some(value), Some(text)) = (StringValue::parse(stated), source_text(literal)) {
            rows.push((line, literal.clone(), text, value.well_formed()));
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
        r#"escape          = "\\" , ( quote | "\\" | "n" | "t" | "r" | unicode_escape ) ;"#,
        r#"escape          = "\\" , ( quote | "\\" | "n" | "t" | "r" | unicode_escape | "0" ) ;"#,
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

#[test]
fn arm_4_a_grammar_that_admits_a_raw_control_character_is_reported() {
    // Finding F-G restored. `string_char` was `any - quote - "\\"`, which admits every control byte,
    // while the reader refused the same byte between forms as "a stray control character" — so the
    // language had the rule and stopped applying it at the opening quote. Reverting the exclusion must
    // fail, on exactly the rows that state it.
    let mutated = GRAMMAR_DOCUMENT.replace(
        r#"string_char     = escape | ( any - quote - "\\" - control ) ;"#,
        r#"string_char     = escape | ( any - quote - "\\" ) ;"#,
    );
    assert_ne!(
        mutated, GRAMMAR_DOCUMENT,
        "the mutation did not apply — a false green"
    );
    let grammar = Grammar::from_document(&mutated);
    assert!(
        grammar.accepts("\"a\u{1b}b\""),
        "the mutated grammar still rejects a raw ESC, so this arm proves nothing"
    );
    let space = literal_space(REFERENCE, &grammar);
    // Four: the raw ESC, the raw NUL, the raw C1 control and the raw line feed. The raw TAB row is
    // well-formed and stays accepted, so it is this arm's green control — the exception is pinned by
    // the same leg that pins the rule.
    assert_eq!(
        space.violations.len(),
        4,
        "expected the four raw-control-character rows; got:\n{}",
        space.violations.join("\n\n")
    );
    assert!(
        space.violations
            .iter()
            .all(|each| each.contains("NOT well-formed")),
        "a violation that is not about well-formedness means the count is right for the wrong reason"
    );
}

#[test]
fn arm_5_a_grammar_that_drops_the_general_escape_is_reported() {
    // The other half of F-G: `\u{…}` is what makes the escape set sufficient, so §3 can escape a
    // control character and still produce text the language reads back (§3 rule 5). Drop it from
    // `escape` and every row that writes a character by code point becomes unreadable.
    let mutated = GRAMMAR_DOCUMENT.replace(
        r#"escape          = "\\" , ( quote | "\\" | "n" | "t" | "r" | unicode_escape ) ;"#,
        r#"escape          = "\\" , ( quote | "\\" | "n" | "t" | "r" ) ;"#,
    );
    assert_ne!(
        mutated, GRAMMAR_DOCUMENT,
        "the mutation did not apply — a false green"
    );
    let grammar = Grammar::from_document(&mutated);
    assert!(
        !grammar.accepts(r#""a\u{1b}b""#),
        "the mutated grammar still accepts `\\u{{…}}`, so this arm proves nothing"
    );
    let space = literal_space(REFERENCE, &grammar);
    // Six: the four rows the reference calls well-formed and the two it calls `refused`, all of which
    // a recognizer must accept. A refusal because a value is out of domain is a *well-formed* input,
    // which is the distinction `Value`'s docs call load-bearing.
    assert_eq!(
        space.violations.len(),
        6,
        "expected the six code-point rows; got:\n{}",
        space.violations.join("\n\n")
    );
    assert!(
        space.violations
            .iter()
            .all(|each| each.contains("is well-formed")),
        "a violation that is not about well-formedness means the count is right for the wrong reason"
    );
}

#[test]
fn a_lookahead_that_leads_nowhere_is_not_reached() {
    with_deep_stack(|| {
        // The shipped grammar cannot show this: its one lookahead is `? delimiter`, whose productions
        // every accepted probe reaches anyway. A lookahead's continuation answers `true` whatever
        // follows, so only the rollback in `Expr::Lookahead` removes `peek` when `"ab"` then fails
        // and the second alternative is the one that accepts.
        let grammar = Grammar::from_document(
            "```ebnf\ndocument = ( ? peek , \"ab\" , end ) | ( \"a\" , \"c\" , end ) ;\npeek = \"a\" ;\n```",
        );
        let recognition = grammar.recognize("ac");
        assert!(recognition.accepted);
        assert!(!recognition.used.contains("peek"), "{:?}", recognition.used);
        assert!(grammar.recognize("ab").used.contains("peek"));
    });
}

#[test]
fn arm_6_a_production_that_loses_its_only_probe_is_reported() {
    with_deep_stack(|| {
        // `unicode_escape` is reached by one probe. Deleting it must leave the production
        // unreached, or the coverage leg is satisfied by something else and proves nothing.
        let grammar = Grammar::load();
        let without: Vec<(&str, &str)> = PROBES
            .iter()
            .copied()
            .filter(|(why, _)| *why != "string with a general escape")
            .collect();
        assert_eq!(
            without.len() + 1,
            PROBES.len(),
            "the probe this arm deletes is gone"
        );
        let missing = unreached(&reach(&grammar, &without, MALFORMED)).join(" ");
        assert_eq!(missing, "unicode_escape");
    });
}

#[test]
fn arm_7_an_exclusion_that_loses_its_refused_probe_is_reported() {
    with_deep_stack(|| {
        // `control` is only ever an exclusion, reached by the one malformed probe it refuses.
        let grammar = Grammar::load();
        let without: Vec<(&str, &str)> = MALFORMED
            .iter()
            .copied()
            .filter(|(why, _)| *why != "raw control character in a string")
            .collect();
        assert_eq!(
            without.len() + 1,
            MALFORMED.len(),
            "the probe this arm deletes is gone"
        );
        let missing = unreached(&reach(&grammar, PROBES, &without)).join(" ");
        assert_eq!(missing, "control");
    });
}

#[test]
fn arm_8_a_production_added_without_a_probe_is_reported() {
    with_deep_stack(|| {
        // A grammar that grows a production no probe reaches must fail the leg: the population is
        // the grammar's own productions, never a list kept beside them.
        let number = "number          = hexadecimal | decimal | integer ;";
        assert!(
            GRAMMAR_DOCUMENT.contains(number),
            "the line this arm mutates moved"
        );
        let mutated = GRAMMAR_DOCUMENT.replace(
            number,
            "number          = binary | hexadecimal | decimal | integer ;\n\
             binary          = \"0b\" , ( \"0\" | \"1\" ) , { \"0\" | \"1\" } ;",
        );
        let grammar = Grammar::from_document(&mutated);
        assert!(
            grammar.rules.contains_key("binary"),
            "the mutation did not apply"
        );
        let missing = unreached(&reach(&grammar, PROBES, MALFORMED)).join(" ");
        assert_eq!(missing, "binary");
    });
}
