//! The language version a description states — §8 of `docs/semantics/reference.md`.
//!
//! ⛔ **The identifier is a form, not a comment header, and that is measured rather than preferred.**
//! §3 states that canonical form carries **no comment**, and §12 M4 hashes canonical text — so a
//! version living in trivia is a version the hashed artifact does not capture: two descriptions
//! differing only in the language version they claim would hash identically, and any tool that strips
//! comments would strip the lock with them. [`crate::form::Document::comment_headers`] exists and §5
//! makes one shape of comment data, so the header route is *available*. It is still wrong here.
//!
//! ⭐ **It is also not a grammar change.** `docs/semantics/grammar.md` fixes *shape* and names no
//! construct vocabulary — its own table says "the language is extended by `defkind`, not by editing
//! this file", and `document = { trivia } , { form , { trivia } } , end` already admits any
//! s-expression. So `(eadl-version eadl/1)` is well-formed today and `conformance.rs`'s recognizer,
//! derived from that grammar, accepts it unchanged. What this module adds is **meaning**, which is the
//! reference's half of the split the two normative documents exist to keep apart.

use crate::diagnostic::{Diagnostic, Diagnostics, Label};
use crate::form::{Document, Form};

/// The head symbol of the identifier form: `(eadl-version eadl/1)`.
///
/// ⛔ **Not `version`, and the collision is measured rather than anticipated.** §6 already uses
/// `version` as a clause of `defmodule` — `(version <major> <minor>)` for the module's own version and
/// `(version (at-least 1 2))` for an import's requirement — with four §4 rows and an implemented reader
/// in `crates/eadl-front/src/module.rs` behind it. One name carrying two different versions in one file
/// is a collision a reader can only resolve by nesting depth, which is the kind of rule that looks
/// unambiguous until some tool forgets the depth.
pub const HEAD: &str = "eadl-version";

/// The only language version this toolchain reads.
///
/// `/` is an ordinary `symbol_char` in `docs/semantics/grammar.md` (`any - whitespace - "(" - ")" -
/// quote - ";"`), so `eadl/1` is **one bare symbol** and needs no quoting — which matters, because a
/// quoted `"eadl/1"` is a different atom and §8 refuses it rather than accepting the two as
/// interchangeable spellings of one thing.
pub const EADL_1: &str = "eadl/1";

/// Whether `form` is a language-version identifier.
///
/// ⭐ **One predicate, every consumer.** Two layers have to recognise the identifier and treat it as
/// what it is — a statement about the document, not a declaration:
///
/// * `crates/eadl-front/src/module.rs` skips it when counting a module file's top-level forms, or §6's
///   "exactly one" would refuse a module that states its own version.
/// * `crates/eadl-model/src/check.rs` skips it before the schema pass, or §7's registry refuses it as
///   `schema-unknown-kind` — which it is, and correctly: it is not a kind.
///
/// ⛔ Both were measured, not anticipated. The second was found by a test that ran the real path and
/// printed `error[schema-unknown-kind]: \`eadl-version\` is not a known kind`, in a layer no frontend
/// test can see. A rule each consumer re-implements is a rule the next consumer lacks — the reasoning
/// that put `M1.13.1`'s escape rule in the printer, one layer up.
#[must_use]
pub fn is_identifier(form: &Form) -> bool {
    form.head() == Some(HEAD)
}

impl Document {
    /// The language version this description states, or `None` when it states none.
    ///
    /// ⭐ `None` is **not** "unknown". §8 rule 2 makes absence denote [`EADL_1`] *by rule*, so a reader
    /// can determine the version from the description alone — which is the entire point of having an
    /// identifier rather than an assumption. It is a rule with a named expiry: `eadl/1` is the last
    /// version for which absence is permitted, because a default that outlives the version it defaults
    /// to is exactly how a description silently changes meaning.
    ///
    /// An identifier this toolchain does not read (`eadl/2`) is refused by [`state`], so this returns
    /// `None` for it rather than a version nothing here can honour.
    #[must_use]
    pub fn stated_version(&self) -> Option<&str> {
        self.forms.iter().find_map(|form| {
            (form.head() == Some(HEAD))
                .then(|| form.items().get(1))
                .flatten()
                .and_then(Form::as_symbol)
                .filter(|stated| *stated == EADL_1)
        })
    }

    /// The language version this description denotes: the one it states, or the one §8 rule 2 supplies.
    ///
    /// This is the value §15's "a source description retains its meaning under its locked semantic
    /// version" is about, and it is a single constant today because this toolchain reads one version.
    /// It is a method rather than a constant so that the day a second version exists, every caller is
    /// already asking the description instead of the toolchain.
    #[must_use]
    pub fn language_version(&self) -> &'static str {
        EADL_1
    }
}

/// Check the identifier forms a document carries, pushing one diagnostic per problem.
///
/// ⭐ **Called from [`crate::read`], not from a caller of it** — so every consumer of a `Document` sees
/// the same verdict, including the ones that do not know this rule exists. `M1.13.1` put the escape
/// rule in the printer for the same reason: a rule that lives in one consumer is a rule the next
/// consumer does not have, and `Form` is constructible outside the frontend.
///
/// Every problem is collected rather than stopping at the first, which is what `module.rs`'s
/// `read_module` does and why: a malformed header usually has more than one thing wrong with it.
pub fn state(document: &Document, diagnostics: &mut Diagnostics) {
    let mut stated: Option<&Form> = None;
    for form in &document.forms {
        if !is_identifier(form) {
            continue;
        }
        if let Some(first) = stated {
            diagnostics.push(
                Diagnostic::error(
                    "language-version-duplicated",
                    "this description states its language version twice",
                    Label::new(form.span(), "the second statement"),
                    "keep one `(eadl-version eadl/1)` and delete this form",
                )
                .with_secondary(Label::new(first.span(), "already stated here")),
            );
            continue;
        }
        stated = Some(form);
        check(form, diagnostics);
    }
}

/// Check one `(eadl-version …)` form's arity, argument kind, and value.
fn check(form: &Form, diagnostics: &mut Diagnostics) {
    let items = form.items();
    let Some(argument) = items.get(1) else {
        diagnostics.push(Diagnostic::error(
            "language-version-missing",
            "this form states no language version",
            Label::new(form.span(), "no identifier"),
            "write `(eadl-version eadl/1)`",
        ));
        return;
    };
    if let Some(extra) = items.get(2) {
        diagnostics.push(Diagnostic::error(
            "language-version-extra-argument",
            "a language version is one identifier, and this form carries more",
            Label::new(extra.span(), "not part of the identifier"),
            "write `(eadl-version eadl/1)` — one bare symbol, and nothing after it",
        ));
    }
    match argument.as_symbol() {
        None => diagnostics.push(Diagnostic::error(
            "language-version-not-an-identifier",
            format!(
                "a language version is a bare symbol, and this one is {}",
                argument.kind()
            ),
            Label::new(argument.span(), "not a symbol"),
            "write `(eadl-version eadl/1)`; ⛔ `\"eadl/1\"` is a string and is not the same atom",
        )),
        Some(EADL_1) => {}
        Some(other) => diagnostics.push(Diagnostic::error(
            "language-version-unknown",
            format!("this toolchain does not read language version `{other}`"),
            Label::new(argument.span(), "not a version this toolchain reads"),
            format!(
                "write `(eadl-version {EADL_1})`, the only version this toolchain reads — a \
                 description is not silently re-read as an older or newer language"
            ),
        )),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::source::SourceMap;
    use crate::{read, Form};

    /// Read one description and return its forms, diagnostics and source map.
    fn parse(text: &str) -> (Document, Diagnostics, SourceMap) {
        let mut sources = SourceMap::new();
        let id = sources
            .add("test.eadl", text)
            .expect("an in-memory source always registers");
        let (document, diagnostics) = read(&sources, id);
        (document, diagnostics, sources)
    }

    fn parse_ok(text: &str) -> Document {
        let (document, diagnostics, sources) = parse(text);
        assert!(
            diagnostics.is_empty(),
            "expected a clean read of {text:?}:\n{}",
            diagnostics.render(&sources)
        );
        document
    }

    #[test]
    fn a_stated_identifier_reads_cleanly_and_is_reported() {
        let document = parse_ok("(eadl-version eadl/1)\n(defsystem s)");
        assert_eq!(document.stated_version(), Some("eadl/1"));
        assert_eq!(document.language_version(), "eadl/1");
    }

    #[test]
    fn an_absent_identifier_is_not_a_defect_and_still_denotes_eadl_1() {
        // ⭐ §8 rule 2, and the reason it is a rule rather than a default: a reader can determine the
        // version from the description alone, because the reference says what absence denotes. The
        // frozen LinkedSpec evidence fixtures depend on this — their bytes are the reproduction of
        // another project's defect, so they cannot be edited to carry an identifier, and nothing in
        // `crates/` reads them (measured: `grep -rn docs/feedback --include='*.rs' crates/` finds only
        // a comment in an example's usage line).
        let document = parse_ok("(defsystem s)\n(task a (period 10 ms))");
        assert_eq!(document.stated_version(), None);
        assert_eq!(
            document.language_version(),
            "eadl/1",
            "absence must denote eadl/1 by rule, not leave the version undetermined"
        );
    }

    #[test]
    fn a_version_this_toolchain_does_not_read_is_refused_rather_than_re_interpreted() {
        for stated in ["eadl/2", "eadl/0", "eadl", "EADL/1", "eadl/1.0"] {
            let (document, diagnostics, sources) = parse(&format!("(eadl-version {stated})"));
            let rendered = diagnostics.render(&sources);
            assert!(
                rendered.contains("language-version-unknown"),
                "`{stated}` was not refused:\n{rendered}"
            );
            assert_eq!(
                document.stated_version(),
                None,
                "`{stated}` must not be reported as a version nothing here can honour"
            );
        }
    }

    #[test]
    fn every_malformed_identifier_is_reported_with_its_own_code() {
        // One code per rule, so a repair direction can be specific: §4 rule 3 makes a code name a
        // rule and not a call site, and five distinct rules get five distinct codes.
        let cases = [
            ("(eadl-version)", "language-version-missing"),
            (
                "(eadl-version \"eadl/1\")",
                "language-version-not-an-identifier",
            ),
            ("(eadl-version 1)", "language-version-not-an-identifier"),
            (
                "(eadl-version (eadl 1))",
                "language-version-not-an-identifier",
            ),
            (
                "(eadl-version eadl/1 eadl/1)",
                "language-version-extra-argument",
            ),
        ];
        for (text, code) in cases {
            let (_, diagnostics, sources) = parse(text);
            let rendered = diagnostics.render(&sources);
            assert!(
                rendered.contains(code),
                "{text:?} did not produce `{code}`:\n{rendered}"
            );
        }
    }

    #[test]
    fn a_string_identifier_is_not_the_same_atom_as_the_symbol_one() {
        // ⛔ The repair text says so out loud, because `"eadl/1"` looks like the same thing typed
        // differently and §2 makes a string and a symbol different values, never interchangeable
        // spellings.
        let (_, diagnostics, sources) = parse("(eadl-version \"eadl/1\")");
        let rendered = diagnostics.render(&sources);
        assert!(
            rendered.contains("language-version-not-an-identifier"),
            "{rendered}"
        );
        assert!(
            rendered.contains("not a symbol"),
            "the label must say what it found, not only that it is wrong:\n{rendered}"
        );
    }

    #[test]
    fn stating_the_version_twice_is_reported_against_the_second_form() {
        let (_, diagnostics, sources) = parse("(eadl-version eadl/1)\n(eadl-version eadl/1)");
        let rendered = diagnostics.render(&sources);
        assert!(
            rendered.contains("language-version-duplicated"),
            "a second identifier was accepted:\n{rendered}"
        );
        assert!(
            rendered.contains("already stated here"),
            "the duplicate must point at the first statement, or the author has to find it:\n{rendered}"
        );
    }

    #[test]
    fn a_nested_identifier_is_not_one_and_does_not_fire_the_rule() {
        // The rule is about the *document's* version, so it reads top-level forms only. A clause named
        // `eadl-version` inside a declaration is that declaration's business — the same reason §6's
        // `version` clause is not confused with it.
        let document = parse_ok("(defsystem s (eadl-version eadl/1))");
        assert_eq!(document.stated_version(), None);
        assert_eq!(document.language_version(), "eadl/1");
    }

    #[test]
    fn the_identifier_survives_canonicalization_which_is_why_it_is_a_form() {
        // ⭐ The whole reason §8 rule 1 rejects the comment-header route, executed rather than argued:
        // §3 drops comments, so a version in trivia would not be in the hashed artifact.
        let with_comment = parse_ok("; eadl-version: eadl/1\n(defsystem s)");
        let with_form = parse_ok("(eadl-version eadl/1)\n(defsystem s)");
        assert!(
            !with_comment.to_canonical().contains("eadl"),
            "a comment header was expected to be dropped by canonical form, and was not: {:?}",
            with_comment.to_canonical()
        );
        assert!(
            with_form.to_canonical().contains("(eadl-version eadl/1)"),
            "the identifier did not survive canonicalization, so §12 M4's hash would not capture \
             it: {:?}",
            with_form.to_canonical()
        );
        assert_eq!(with_comment.stated_version(), None);
    }

    #[test]
    fn a_form_that_is_not_a_list_never_reaches_the_arity_checks() {
        // `head()` is `None` for an atom, so a bare top-level symbol named `eadl-version` is that
        // declaration's problem and not this rule's — pinned so the filter cannot silently widen.
        let document = parse_ok("eadl-version");
        assert!(matches!(document.forms[0], Form::Symbol { .. }));
        assert_eq!(document.stated_version(), None);
    }
}
