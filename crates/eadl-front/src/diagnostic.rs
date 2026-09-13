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
/// One source of truth for what a check concluded. `osgen-cli` maps it to an exit code and a
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
    /// span, so it lands under the right text in a line containing non-ASCII prose.
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
        let line_text = source.line_text(position.line);
        let gutter_width = position.line.to_string().len();
        let pad = " ".repeat(gutter_width);

        // Caret width in characters, never zero: a zero-width span still has to point at
        // something, and a caret of width 0 renders as nothing at all.
        let snippet = sources.snippet(label.span);
        let caret_width = snippet.chars().count().max(1);
        let indent = " ".repeat((position.column as usize).saturating_sub(1));

        format!(
            "  --> {}:{}:{}\n{pad} |\n{} | {}\n{pad} | {}{}{}\n",
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
            }
        )
    }
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
    use super::{Diagnostic, Diagnostics, Label, Severity};
    use crate::source::{SourceMap, Span};

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
}
