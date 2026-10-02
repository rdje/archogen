//! Diagnostics and how they are rendered.
//!
//! `ROADMAP.md` §5.5 fixes what a diagnostic must carry:
//!
//! > Each diagnostic carries source spans, the relevant requirement and offer IDs, the
//! > supported profile, a small explanatory conflict set where available, and a concrete
//! > repair direction.
//!
//! The reader can only supply the first and the last of those, so the rest are optional
//! fields that later layers fill in. They live here rather than in each layer's own type
//! because a user should not be able to tell, from the shape of a message, which pass produced
//! it — and because a repair direction that is optional in the type is a repair direction that
//! goes missing under deadline. [`Diagnostic::new`] requires one.

use crate::source::{SourceMap, Span};

/// The `ROADMAP.md` §5.5 result vocabulary.
///
/// One source of truth for what a check concluded. `archogen-cli` maps it to an exit code and a
/// test asserts that mapping is total, so the two cannot drift — a second enum spelling the
/// same seven words would be the drift.
///
/// `tool-failure` is deliberately here too: §5.5 lists it, and a check that failed internally
/// must be distinguishable from one that concluded something. It is "never reported as a valid
/// system".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum Verdict {
    /// The description was accepted.
    Ok,
    /// Malformed, contradictory, or ill-typed input.
    InvalidDescription,
    /// Relevant contract information is unavailable.
    MissingFact,
    /// Requested behavior or analysis lies outside implemented semantics.
    UnsupportedProfile,
    /// Supported constraints have no satisfying assignment.
    InfeasibleConfiguration,
    /// A resource limit or unresolved bound prevented a conclusion.
    AnalysisInconclusive,
    /// A sufficient analysis did not establish the requested property.
    NotEstablished,
    /// A validated witness violates a named property.
    Counterexample,
    /// Internal failure or an unavailable required tool.
    ToolFailure,
}

impl Verdict {
    /// Every verdict, in the order §5.5 lists them.
    pub const ALL: &'static [Self] = &[
        Self::Ok,
        Self::InvalidDescription,
        Self::MissingFact,
        Self::UnsupportedProfile,
        Self::InfeasibleConfiguration,
        Self::AnalysisInconclusive,
        Self::NotEstablished,
        Self::Counterexample,
        Self::ToolFailure,
    ];

    /// The stable machine-readable name, exactly as §5.5 spells it.
    #[must_use]
    pub const fn slug(self) -> &'static str {
        match self {
            Self::Ok => "ok",
            Self::InvalidDescription => "invalid-description",
            Self::MissingFact => "missing-fact",
            Self::UnsupportedProfile => "unsupported-profile",
            Self::InfeasibleConfiguration => "infeasible-configuration",
            Self::AnalysisInconclusive => "analysis-inconclusive",
            Self::NotEstablished => "not-established",
            Self::Counterexample => "counterexample",
            Self::ToolFailure => "tool-failure",
        }
    }

    /// Parse the machine-readable name.
    #[must_use]
    pub fn parse(text: &str) -> Option<Self> {
        Self::ALL.iter().copied().find(|v| v.slug() == text)
    }

    /// The verdict a diagnostic's code carries, with the one default every consumer shares.
    ///
    /// ⭐ **A code that is not a §5.5 verdict slug is a statement about the description, not about the
    /// toolchain.** `quantity-unknown-unit`, `schema-arity` and `read-unclosed-list` each name a rule the
    /// language enforces and none of them is a verdict name, so the verdict for a description that breaks
    /// one is `InvalidDescription`. Defaulting the other way tells an author to file a bug about the
    /// toolchain for a symbol they mistyped, and §5.5 is explicit that a tool failure is never a statement
    /// about the system — the `tool-failure` diagnostic `crates/eadl-model/src/check.rs` builds says so in
    /// its own repair direction.
    ///
    /// ⛔ **One accessor, because this rule had three consumers and two answers.**
    /// `crates/eadl-model/src/check.rs` defaulted it to `InvalidDescription` twice while
    /// `crates/archogen-cli/src/build_cmd.rs` defaulted it to `ToolFailure` once, so `archogen check` and
    /// `archogen build` classified the *same* diagnostic differently and the one facing the author was the
    /// wrong one: a build refusing `(period 10 parsec)` exited **70**. A rule each consumer re-implements
    /// is a rule the next consumer lacks, which is why `M1.13.4.1` extracted
    /// `crate::language_version::declarations()` for the language-version identifier and this extracts the
    /// same rule for the verdict — see leaf `M1.28.1` in `docs/tasks/M1.md`.
    #[must_use]
    pub fn of_code(code: &str) -> Self {
        Self::parse(code).unwrap_or(Self::InvalidDescription)
    }

    /// Whether the description was accepted.
    #[must_use]
    pub const fn is_ok(self) -> bool {
        matches!(self, Self::Ok)
    }

    /// How severe this verdict is, for choosing which of several to report.
    ///
    /// ⭐ The ordering is not arbitrary and it is not severity-of-consequence. It is
    /// **what-to-fix-first**: a malformed description makes every later answer meaningless, so
    /// `invalid-description` outranks everything; an unsupported request should be reported
    /// before a missing fact, because describing the missing fact would be wasted work on a
    /// system the profile will refuse anyway.
    #[must_use]
    pub const fn precedence(self) -> u8 {
        match self {
            Self::ToolFailure => 100,
            Self::InvalidDescription => 90,
            Self::UnsupportedProfile => 80,
            Self::InfeasibleConfiguration => 70,
            Self::MissingFact => 60,
            Self::Counterexample => 50,
            Self::NotEstablished => 40,
            Self::AnalysisInconclusive => 30,
            Self::Ok => 0,
        }
    }
}

/// How bad it is.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Severity {
    /// The description cannot be used.
    Error,
    /// Worth saying; the description is still usable.
    Warning,
}

impl Severity {
    /// The rendered word.
    #[must_use]
    pub const fn label(self) -> &'static str {
        match self {
            Self::Error => "error",
            Self::Warning => "warning",
        }
    }
}

/// A span with something to say about it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Label {
    /// What to point at.
    pub span: Span,
    /// Why it is being pointed at.
    pub message: String,
}

impl Label {
    /// A label on `span`.
    #[must_use]
    pub fn new(span: Span, message: impl Into<String>) -> Self {
        Self {
            span,
            message: message.into(),
        }
    }
}

/// One diagnostic.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Diagnostic {
    /// Error or warning.
    pub severity: Severity,
    /// A stable machine-readable code, e.g. `read-unclosed-list`. Users grep these.
    pub code: &'static str,
    /// The one-line summary.
    pub message: String,
    /// The primary location. Rendered with a caret.
    pub primary: Label,
    /// Further locations — the opening paren of an unclosed list, the first of two conflicting
    /// declarations.
    pub secondary: Vec<Label>,
    /// What to do about it. §5.5 requires one, so the constructor does too.
    pub repair: String,
    /// Requirement and offer IDs this diagnostic concerns, once such things exist.
    pub related_ids: Vec<String>,
    /// The supported profile in force, once profiles are threaded through.
    pub profile: Option<String>,
}

impl Diagnostic {
    /// An error, with its repair direction.
    #[must_use]
    pub fn error(
        code: &'static str,
        message: impl Into<String>,
        primary: Label,
        repair: impl Into<String>,
    ) -> Self {
        Self {
            severity: Severity::Error,
            code,
            message: message.into(),
            primary,
            secondary: Vec::new(),
            repair: repair.into(),
            related_ids: Vec::new(),
            profile: None,
        }
    }

    /// Add a secondary label.
    #[must_use]
    pub fn with_secondary(mut self, label: Label) -> Self {
        self.secondary.push(label);
        self
    }

    /// Render in the style a caret-and-line-number reader expects.
    ///
    /// The caret is placed by character column, and its width is the character width of the
    /// span **on the line shown**, so it lands under the right text in a line containing non-ASCII
    /// prose and never runs past that line; a span that continues onto later lines says which one it
    /// ends on.
    #[must_use]
    pub fn render(&self, sources: &SourceMap) -> String {
        let mut out = format!(
            "{}[{}]: {}\n",
            self.severity.label(),
            self.code,
            self.message
        );
        out.push_str(&self.render_label(sources, &self.primary, '^'));
        for label in &self.secondary {
            out.push_str(&self.render_label(sources, label, '-'));
        }
        out.push_str(&format!("  = hint: {}\n", self.repair));
        if !self.related_ids.is_empty() {
            out.push_str(&format!("  = related: {}\n", self.related_ids.join(", ")));
        }
        if let Some(profile) = &self.profile {
            out.push_str(&format!("  = profile: {profile}\n"));
        }
        out
    }

    fn render_label(&self, sources: &SourceMap, label: &Label, marker: char) -> String {
        let Some(source) = sources.get(label.span.source) else {
            return format!("  --> <unknown source>: {}\n", label.message);
        };
        let position = source.position(label.span.start);
        let whole_line = source.line_text(position.line);
        let gutter_width = position.line.to_string().len();
        let pad = " ".repeat(gutter_width);

        // ⛔ The caret covers the span **on the line printed above it**, and no further (leaf `M1.31`).
        // It used to be sized by the whole span, so a label over a seven-line `(defmodule …)` drew 236
        // carets under a 21-character line — wrapping the terminal and pointing at nothing, in 10 of the
        // 78 tracked descriptions. The part of the span past this line is not drawn; it is *said*, so a
        // reader can still tell a one-line span from the first line of a longer one.
        //
        // Width in characters, never zero: a zero-width span still has to point at something, and a
        // caret of width 0 renders as nothing at all.
        let covered = sources.snippet(label.span);
        let on_this_line = covered
            .split('\n')
            .next()
            .unwrap_or_default()
            .trim_end_matches('\r');
        let column = (position.column as usize).saturating_sub(1);
        // ⛔ A line longer than `EXCERPT` characters is quoted as a window around the span (leaf `API.6.6`). Quoted
        // whole, a 1 MB line of unclosed forms was printed once per diagnostic, 257 times: an answer of 1.5 GB for a
        // request of 1 MB, from `archogen check`, the engine API and the MCP server alike (`API.6.5`'s review, D3).
        let (line_text, column, room) = excerpt(whole_line, column);
        let caret_width = on_this_line.chars().count().min(room).max(1);
        let indent = " ".repeat(column);

        // Counted over the covered text rather than found by offset arithmetic: `end - 1` can land inside
        // a multi-byte character, and `position` slices at the offset it is given — measured, twelve
        // reference legs panicked on exactly that. A span that ends by covering a newline has not
        // continued onto the next line, so one trailing newline is not a continuation.
        let continued_lines = covered
            .strip_suffix('\n')
            .unwrap_or(covered)
            .matches('\n')
            .count();
        let continuation = if continued_lines > 0 {
            format!(
                " (continues to line {})",
                position.line as usize + continued_lines
            )
        } else {
            String::new()
        };

        format!(
            "  --> {}:{}:{}\n{pad} |\n{} | {}\n{pad} | {}{}{}{}\n",
            source.name,
            position.line,
            position.column,
            position.line,
            line_text,
            indent,
            marker.to_string().repeat(caret_width),
            if label.message.is_empty() {
                String::new()
            } else {
                format!(" {}", label.message)
            },
            continuation
        )
    }
}

/// The longest line a diagnostic quotes whole, in characters. A longer line is quoted as a window of this many
/// characters around the span, with `…` where it was cut, so a diagnostic's size does not grow with its line's.
pub const EXCERPT: usize = 160;

/// How many characters of context an excerpt keeps before its span.
const BEFORE: usize = 40;

/// The text to quote for a line, the span's column in that text (0-based), and how many characters the caret may
/// cover from there before the quoted text ends: the line itself when it is at most [`EXCERPT`] characters, and
/// otherwise a window of that many around `column`, marked `…` at each cut end.
fn excerpt(line: &str, column: usize) -> (String, usize, usize) {
    let length = line.chars().count();
    if length <= EXCERPT {
        return (
            line.to_owned(),
            column,
            length.saturating_sub(column).max(1),
        );
    }
    let mut start = column.saturating_sub(BEFORE);
    let end = (start + EXCERPT).min(length);
    if end - start < EXCERPT {
        start = end.saturating_sub(EXCERPT);
    }
    let mut text = String::new();
    let mut shifted = column - start;
    if start > 0 {
        text.push('…');
        shifted += 1;
    }
    text.extend(line.chars().skip(start).take(end - start));
    if end < length {
        text.push('…');
    }
    (text, shifted, end.saturating_sub(column).max(1))
}

/// A set of diagnostics collected during one pass.
#[derive(Debug, Clone, Default)]
pub struct Diagnostics {
    items: Vec<Diagnostic>,
}

impl Diagnostics {
    /// An empty set.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Record one.
    pub fn push(&mut self, diagnostic: Diagnostic) {
        self.items.push(diagnostic);
    }

    /// Everything recorded, in the order it was recorded.
    #[must_use]
    pub fn items(&self) -> &[Diagnostic] {
        &self.items
    }

    /// Whether any error was recorded. Warnings do not count.
    #[must_use]
    pub fn has_errors(&self) -> bool {
        self.items
            .iter()
            .any(|item| item.severity == Severity::Error)
    }

    /// Whether nothing was recorded at all.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.items.is_empty()
    }

    /// How many were recorded.
    #[must_use]
    pub fn len(&self) -> usize {
        self.items.len()
    }

    /// Render all of them.
    #[must_use]
    pub fn render(&self, sources: &SourceMap) -> String {
        self.items
            .iter()
            .map(|item| item.render(sources))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

#[cfg(test)]
mod tests {
    use super::{Diagnostic, Diagnostics, Label, Severity, Verdict, EXCERPT};
    use crate::source::{SourceMap, Span};

    // ── `Verdict::of_code`: one rule, one default, every consumer ────────────────────────────
    //
    // ⛔ These are the RED arms for leaf `M1.28.1`. The rule had three consumers and two answers:
    // `check.rs` defaulted a code that is not a verdict slug to `InvalidDescription` and
    // `build_cmd.rs` to `ToolFailure`, so the same diagnostic was classified differently by
    // `archogen check` and `archogen build` — and the one facing the author was the wrong one, exiting
    // 70 for a mistyped unit symbol.

    #[test]
    fn a_code_that_is_not_a_verdict_slug_says_something_about_the_description() {
        // Real codes, taken from the emitters rather than invented: a unit the table does not hold, a
        // clause with the wrong number of values, an unclosed list. None is a §5.5 verdict name, and
        // each is a rule the language enforces — so each is a malformed description, never a broken
        // toolchain.
        for code in [
            "quantity-unknown-unit",
            "quantity-non-positive-frequency",
            "schema-arity",
            "read-unclosed-list",
            "refinement-violated",
            "boundary-implementation-in-description",
        ] {
            let verdict = Verdict::of_code(code);
            assert_eq!(
                verdict,
                Verdict::InvalidDescription,
                "`{code}` is a rule the language enforces, so a description that breaks it is \
                 malformed; §5.5 reserves `tool-failure` for the toolchain"
            );
            assert_ne!(
                verdict,
                Verdict::ToolFailure,
                "`{code}` classified as a toolchain failure tells the author to file a bug about \
                 the tool for a symbol they mistyped"
            );
        }
    }

    #[test]
    fn every_verdict_slug_still_maps_back_to_its_own_verdict() {
        // The accessor must not shadow `parse`: a consumer that relied on `unsupported-profile`
        // arriving as `UnsupportedProfile` is the reason the rule exists at all, and a default that
        // swallowed the slugs would pass the arm above while breaking every verdict-shaped code.
        for verdict in Verdict::ALL {
            assert_eq!(
                &Verdict::of_code(verdict.slug()),
                verdict,
                "`{}` is a §5.5 verdict name and must arrive as itself",
                verdict.slug()
            );
        }
    }

    #[test]
    fn the_default_is_never_acceptance() {
        // ⭐ The failure this pins is silent: a default of `Ok` would make an unrecognized code
        // *accept* the description, which no reading of §5.5 permits. `parse` returns `None` for a
        // code it does not know, and `None` means "not `ok`".
        for code in ["", "nonsense", "eadl-version", "ok-but-not-really"] {
            let verdict = Verdict::of_code(code);
            assert!(
                !verdict.is_ok(),
                "an unrecognized code `{code}` was classified as acceptance"
            );
            assert!(
                verdict.precedence() > Verdict::Ok.precedence(),
                "an unrecognized code `{code}` must outrank `ok`, so a description carrying one is \
                 never reported as valid"
            );
        }
        // And the slug `ok` itself is the one code that does mean acceptance, so the arm above is not
        // green on a default that refuses everything.
        assert!(Verdict::of_code("ok").is_ok());
    }

    #[test]
    fn a_rendered_diagnostic_carries_location_caret_and_repair() {
        let mut sources = SourceMap::new();
        let id = sources
            .add("t.eadl", "(defservice time\n  (bad 1)\n")
            .unwrap();
        let diagnostic = Diagnostic::error(
            "read-example",
            "something is wrong here",
            Label::new(Span::new(id, 19, 22), "this atom"),
            "write it differently",
        );
        let rendered = diagnostic.render(&sources);
        assert!(
            rendered.starts_with("error[read-example]: something is wrong here"),
            "{rendered}"
        );
        assert!(rendered.contains("--> t.eadl:2:3"), "{rendered}");
        assert!(rendered.contains("^^^ this atom"), "{rendered}");
        assert!(
            rendered.contains("= hint: write it differently"),
            "{rendered}"
        );
    }

    #[test]
    fn the_caret_lands_under_the_right_text_after_non_ascii_prose() {
        // ⭐ The bug this test exists to prevent: counting bytes instead of characters puts the
        // caret several columns right of the problem, and only on the lines that carry prose.
        let mut sources = SourceMap::new();
        let text = "; § — a comment\n(bad)\n";
        let id = sources.add("t.eadl", text).unwrap();
        let open = text.find("(bad)").unwrap() as u32;
        let diagnostic = Diagnostic::error(
            "read-example",
            "m",
            Label::new(Span::new(id, open, open + 5), ""),
            "r",
        );
        let rendered = diagnostic.render(&sources);
        assert!(rendered.contains("--> t.eadl:2:1"), "{rendered}");
        let caret_line = rendered
            .lines()
            .find(|line| line.contains("^^^^^"))
            .expect("a caret line");
        // The caret must start immediately after the gutter, i.e. no leading indent.
        assert!(
            caret_line.trim_start().starts_with("| ^^^^^"),
            "{caret_line}"
        );
    }

    #[test]
    fn a_zero_width_span_still_renders_a_caret() {
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", "(a\n").unwrap();
        let diagnostic = Diagnostic::error(
            "read-example",
            "m",
            Label::new(Span::at(id, 3), "here"),
            "r",
        );
        let rendered = diagnostic.render(&sources);
        assert!(
            rendered.contains('^'),
            "a zero-width span rendered no caret:\n{rendered}"
        );
    }

    /// The marker line of the first label `render` draws.
    fn marker_line(rendered: &str) -> &str {
        rendered
            .lines()
            .find(|line| line.trim_start().starts_with("| ") && line.contains(['^', '-']))
            .expect("a marker line")
    }

    #[test]
    fn a_multi_line_span_is_clipped_to_its_first_line_and_says_where_it_ends() {
        // ⭐ Leaf `M1.31`: the renderer drew one caret per character of the WHOLE span under its first
        // line — 236 under `(defmodule app.system`, the book's own opening example.
        let mut sources = SourceMap::new();
        let text = "(defmodule app.system\n  (version 1 0)\n  (export app.rt))\n";
        let id = sources.add("t.eadl", text).unwrap();
        let end = u32::try_from(text.trim_end().len()).unwrap();
        let rendered = Diagnostic::error(
            "read-example",
            "m",
            Label::new(Span::new(id, 0, end), "the whole form"),
            "r",
        )
        .render(&sources);
        assert_eq!(
            marker_line(&rendered),
            format!(
                "  | {} the whole form (continues to line 3)",
                "^".repeat("(defmodule app.system".len())
            ),
            "{rendered}"
        );
    }

    #[test]
    fn a_span_starting_mid_line_is_clipped_to_the_rest_of_that_line() {
        let mut sources = SourceMap::new();
        let text = "  (implementation\n    (step 1))\n";
        let id = sources.add("t.eadl", text).unwrap();
        let end = u32::try_from(text.trim_end().len()).unwrap();
        let rendered = Diagnostic::error(
            "read-example",
            "m",
            Label::new(Span::new(id, 2, end), ""),
            "r",
        )
        .render(&sources);
        assert_eq!(
            marker_line(&rendered),
            format!(
                "  |   {} (continues to line 2)",
                "^".repeat("(implementation".len())
            ),
            "{rendered}"
        );
    }

    #[test]
    fn a_span_that_ends_by_covering_its_newline_has_not_continued() {
        // `end` is exclusive, so a span over `(a)\n` ends ON line 1 — the continuation note would be false.
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", "(a)\n(b)\n").unwrap();
        let rendered = Diagnostic::error(
            "read-example",
            "m",
            Label::new(Span::new(id, 0, 4), "x"),
            "r",
        )
        .render(&sources);
        assert_eq!(marker_line(&rendered), "  | ^^^ x", "{rendered}");
    }

    #[test]
    fn a_multi_line_span_ending_inside_non_ascii_text_renders_without_panicking() {
        // ⛔ Measured while writing this leaf: finding the last line by `position(end - 1)` put the offset
        // inside a multi-byte character, `position` sliced there, and twelve reference legs panicked. The
        // continuation is now counted over the covered text, which has no byte offset to get wrong.
        // The span's LAST character is the two-byte `é`, so `end - 1` is not a character boundary.
        let mut sources = SourceMap::new();
        let text = "(a\n  é)\n";
        let id = sources.add("t.eadl", text).unwrap();
        let end = u32::try_from(text.find(')').unwrap()).unwrap();
        let rendered = Diagnostic::error(
            "read-example",
            "m",
            Label::new(Span::new(id, 0, end), ""),
            "r",
        )
        .render(&sources);
        assert_eq!(
            marker_line(&rendered),
            "  | ^^ (continues to line 2)",
            "{rendered}"
        );
    }

    #[test]
    fn secondary_labels_render_with_their_own_location() {
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", "(open\n  x\n").unwrap();
        let diagnostic = Diagnostic::error(
            "read-unclosed-list",
            "this list is never closed",
            Label::new(Span::at(id, 10), "end of file"),
            "add a matching `)`",
        )
        .with_secondary(Label::new(Span::new(id, 0, 1), "opened here"));
        let rendered = diagnostic.render(&sources);
        assert!(rendered.contains("--> t.eadl:1:1"), "{rendered}");
        assert!(rendered.contains("- opened here"), "{rendered}");
    }

    #[test]
    fn an_unknown_source_degrades_instead_of_panicking() {
        let sources = SourceMap::new();
        let diagnostic = Diagnostic::error(
            "read-example",
            "m",
            Label::new(Span::new(super::super::source::SourceId(7), 0, 1), "x"),
            "r",
        );
        assert!(diagnostic.render(&sources).contains("<unknown source>"));
    }

    #[test]
    fn warnings_do_not_make_a_set_erroneous() {
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", "x").unwrap();
        let mut set = Diagnostics::new();
        assert!(set.is_empty());
        let mut warning = Diagnostic::error("w", "m", Label::new(Span::at(id, 0), ""), "r");
        warning.severity = Severity::Warning;
        set.push(warning);
        assert_eq!(set.len(), 1);
        assert!(!set.has_errors());
        set.push(Diagnostic::error(
            "e",
            "m",
            Label::new(Span::at(id, 0), ""),
            "r",
        ));
        assert!(set.has_errors());
    }

    /// The quoted source line of a rendered single-label diagnostic: the line after the gutter's ` |`.
    fn quoted_line(rendered: &str) -> &str {
        rendered
            .lines()
            .find(|line| line.contains(" | ") && !line.trim_start().starts_with('|'))
            .and_then(|line| line.split_once(" | "))
            .map(|(_, text)| text)
            .expect("a quoted line")
    }

    fn render_at(text: &str, start: u32, end: u32) -> String {
        let mut sources = SourceMap::new();
        let id = sources.add("t.eadl", text).unwrap();
        Diagnostic::error(
            "read-example",
            "m",
            Label::new(Span::new(id, start, end), ""),
            "r",
        )
        .render(&sources)
    }

    #[test]
    fn a_line_at_the_excerpt_bound_is_quoted_whole_and_one_longer_is_cut() {
        let at = "a".repeat(EXCERPT);
        assert_eq!(quoted_line(&render_at(&at, 0, 1)), at);
        let longer = "a".repeat(EXCERPT + 1);
        let quoted = quoted_line(&render_at(&longer, 0, 1)).to_owned();
        assert!(
            quoted.ends_with('…') && !quoted.starts_with('…'),
            "{quoted}"
        );
        assert_eq!(quoted.chars().count(), EXCERPT + 1);
    }

    /// Leaf `API.6.6`: on a long line, the caret still lands under the span's first character — at the line's
    /// start, its middle and its end — and the quoted text is a bounded window, cut with `…`.
    #[test]
    fn a_long_lines_excerpt_puts_the_caret_under_its_span() {
        for at in [0_usize, 5_000, 9_999] {
            let mut text: Vec<char> = "é".repeat(10_000).chars().collect();
            text[at] = 'X';
            let text: String = text.into_iter().collect();
            let start = u32::try_from(text.char_indices().nth(at).unwrap().0).unwrap();
            let rendered = render_at(&text, start, start + 1);
            let quoted = quoted_line(&rendered);
            let marker = marker_line(&rendered);
            assert!(quoted.chars().count() <= EXCERPT + 2, "{quoted}");
            let caret = marker.trim_start_matches(['|', ' ']).len();
            let caret_at = marker.chars().count() - caret - "  | ".chars().count();
            assert_eq!(
                quoted.chars().nth(caret_at),
                Some('X'),
                "at {at}: {quoted}\n{marker}"
            );
            assert!(rendered.len() < 2_000, "at {at}: {} bytes", rendered.len());
        }
    }
}
