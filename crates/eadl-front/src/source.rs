//! Sources, byte spans, and the line/column mapping used to render them.
//!
//! Every later layer reports through a [`Span`]. `ROADMAP.md` §5.5 requires *every* diagnostic
//! to carry source spans, and §12 M1's exit gate is that "invalid examples produce
//! source-localized diagnostics" — so spans are not a debugging affordance added later, they
//! are the first thing built and the thing everything else is threaded through.
//!
//! Offsets are **byte** offsets, and columns are counted in characters. Mixing the two is the
//! classic way a caret lands in the wrong place the first time a description contains a
//! non-ASCII character, and eADL descriptions carry prose in comments.

/// Identifies one source within a [`SourceMap`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceId(pub u32);

/// A half-open byte range `[start, end)` within one source.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Span {
    /// Which source.
    pub source: SourceId,
    /// Inclusive start byte offset.
    pub start: u32,
    /// Exclusive end byte offset.
    pub end: u32,
}

impl Span {
    /// A span over `[start, end)`.
    #[must_use]
    pub const fn new(source: SourceId, start: u32, end: u32) -> Self {
        Self { source, start, end }
    }

    /// A zero-width span at `offset`, for pointing at a position rather than a range.
    #[must_use]
    pub const fn at(source: SourceId, offset: u32) -> Self {
        Self {
            source,
            start: offset,
            end: offset,
        }
    }

    /// The smallest span covering both. Panics in debug if they are from different sources,
    /// which would silently produce a nonsense range.
    #[must_use]
    pub fn merge(self, other: Self) -> Self {
        debug_assert_eq!(
            self.source, other.source,
            "cannot merge spans from different sources"
        );
        Self {
            source: self.source,
            start: self.start.min(other.start),
            end: self.end.max(other.end),
        }
    }

    /// Length in bytes.
    #[must_use]
    pub const fn len(self) -> u32 {
        self.end.saturating_sub(self.start)
    }

    /// Whether the span covers no bytes.
    #[must_use]
    pub const fn is_empty(self) -> bool {
        self.end <= self.start
    }
}

/// A 1-based line and column, as a human reads them.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Position {
    /// 1-based line number.
    pub line: u32,
    /// 1-based column, counted in characters, not bytes.
    pub column: u32,
}

/// One loaded source file.
#[derive(Debug, Clone)]
pub struct Source {
    /// The path as the user gave it, repo-root-relative wherever possible.
    pub name: String,
    /// The full text.
    pub text: String,
    /// Byte offset of the start of each line. Always begins with 0.
    line_starts: Vec<u32>,
}

impl Source {
    fn new(name: String, text: String) -> Self {
        let mut line_starts = vec![0];
        for (offset, byte) in text.bytes().enumerate() {
            if byte == b'\n' {
                // `offset + 1` cannot overflow u32 for any source we will accept; sources are
                // bounded well below 4 GiB and `load` rejects anything larger.
                line_starts.push(u32::try_from(offset + 1).unwrap_or(u32::MAX));
            }
        }
        Self {
            name,
            text,
            line_starts,
        }
    }

    /// The 1-based line/column of a byte offset, clamped to the end of the text.
    #[must_use]
    pub fn position(&self, offset: u32) -> Position {
        let offset = offset.min(u32::try_from(self.text.len()).unwrap_or(u32::MAX));
        // The index of the last line start that is <= offset.
        let line_index = match self.line_starts.binary_search(&offset) {
            Ok(index) => index,
            Err(index) => index.saturating_sub(1),
        };
        let line_start = self.line_starts[line_index] as usize;
        let column = self.text[line_start..offset as usize].chars().count() + 1;
        Position {
            line: u32::try_from(line_index + 1).unwrap_or(u32::MAX),
            column: u32::try_from(column).unwrap_or(u32::MAX),
        }
    }

    /// The text of a 1-based line, without its terminator.
    #[must_use]
    pub fn line_text(&self, line: u32) -> &str {
        let index = (line as usize).saturating_sub(1);
        let Some(&start) = self.line_starts.get(index) else {
            return "";
        };
        let end = self
            .line_starts
            .get(index + 1)
            .map_or(self.text.len(), |&next| next as usize);
        self.text[start as usize..end].trim_end_matches(['\n', '\r'])
    }

    /// How many lines the source has.
    #[must_use]
    pub fn line_count(&self) -> u32 {
        u32::try_from(self.line_starts.len()).unwrap_or(u32::MAX)
    }
}

/// Every source the frontend has read.
#[derive(Debug, Clone, Default)]
pub struct SourceMap {
    sources: Vec<Source>,
}

/// Why a source could not be added.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SourceError {
    /// The text is larger than spans can address.
    TooLarge {
        /// The source's name.
        name: String,
        /// Its length in bytes.
        bytes: usize,
    },
}

impl core::fmt::Display for SourceError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::TooLarge { name, bytes } => write!(
                f,
                "`{name}` is {bytes} bytes, larger than a span can address ({} max)",
                u32::MAX
            ),
        }
    }
}

impl SourceMap {
    /// An empty map.
    #[must_use]
    pub fn new() -> Self {
        Self::default()
    }

    /// Add a source and return its id.
    ///
    /// # Errors
    ///
    /// Returns [`SourceError::TooLarge`] rather than truncating. A silently truncated source
    /// parses to something that is not what the author wrote, which is the worst outcome
    /// available.
    pub fn add(
        &mut self,
        name: impl Into<String>,
        text: impl Into<String>,
    ) -> Result<SourceId, SourceError> {
        let name = name.into();
        let text = text.into();
        if u32::try_from(text.len()).is_err() {
            return Err(SourceError::TooLarge {
                bytes: text.len(),
                name,
            });
        }
        let id = SourceId(u32::try_from(self.sources.len()).unwrap_or(u32::MAX));
        self.sources.push(Source::new(name, text));
        Ok(id)
    }

    /// The source behind an id.
    #[must_use]
    pub fn get(&self, id: SourceId) -> Option<&Source> {
        self.sources.get(id.0 as usize)
    }

    /// The text a span covers, or `""` if the span is out of range.
    #[must_use]
    pub fn snippet(&self, span: Span) -> &str {
        let Some(source) = self.get(span.source) else {
            return "";
        };
        let start = (span.start as usize).min(source.text.len());
        let end = (span.end as usize).min(source.text.len()).max(start);
        source.text.get(start..end).unwrap_or("")
    }
}

#[cfg(test)]
mod tests {
    use super::{SourceError, SourceMap, Span};

    fn map(text: &str) -> (SourceMap, super::SourceId) {
        let mut map = SourceMap::new();
        let id = map.add("t.eadl", text).expect("small enough");
        (map, id)
    }

    #[test]
    fn the_first_byte_is_line_one_column_one() {
        let (map, id) = map("abc\ndef\n");
        let source = map.get(id).unwrap();
        let start = source.position(0);
        assert_eq!((start.line, start.column), (1, 1));
    }

    #[test]
    fn a_position_after_a_newline_is_the_next_line() {
        let (map, id) = map("abc\ndef\n");
        let source = map.get(id).unwrap();
        let p = source.position(4);
        assert_eq!((p.line, p.column), (2, 1));
        let q = source.position(6);
        assert_eq!((q.line, q.column), (2, 3));
    }

    #[test]
    fn columns_count_characters_not_bytes() {
        // ⭐ The classic caret-misplacement bug. eADL descriptions carry prose in comments, so
        // a non-ASCII character before the error is not hypothetical — the boundary corpus is
        // full of `§` and em dashes.
        let (map, id) = map("; § — ok\n(x)\n");
        let source = map.get(id).unwrap();
        let newline = "; § — ok".len() as u32;
        let p = source.position(newline);
        assert_eq!(p.line, 1);
        assert_eq!(
            p.column, 9,
            "column must count the 8 characters, not the bytes"
        );
    }

    #[test]
    fn an_offset_past_the_end_clamps_instead_of_panicking() {
        let (map, id) = map("ab");
        let source = map.get(id).unwrap();
        let p = source.position(9999);
        assert_eq!((p.line, p.column), (1, 3));
    }

    #[test]
    fn line_text_strips_the_terminator_and_handles_crlf() {
        let (map, id) = map("one\r\ntwo\n");
        let source = map.get(id).unwrap();
        assert_eq!(source.line_text(1), "one");
        assert_eq!(source.line_text(2), "two");
        assert_eq!(source.line_text(99), "");
    }

    #[test]
    fn merging_spans_covers_both() {
        let (_, id) = map("abcdef");
        let a = Span::new(id, 1, 2);
        let b = Span::new(id, 4, 5);
        let merged = a.merge(b);
        assert_eq!((merged.start, merged.end), (1, 5));
    }

    #[test]
    fn snippet_returns_the_covered_text_and_never_panics_out_of_range() {
        let (map, id) = map("(hello)");
        assert_eq!(map.snippet(Span::new(id, 1, 6)), "hello");
        assert_eq!(map.snippet(Span::new(id, 1, 9999)), "hello)");
        assert_eq!(map.snippet(Span::new(super::SourceId(42), 0, 1)), "");
    }

    #[test]
    fn a_zero_width_span_is_empty_and_has_no_length() {
        let (_, id) = map("x");
        let span = Span::at(id, 3);
        assert!(span.is_empty());
        assert_eq!(span.len(), 0);
    }

    #[test]
    fn source_error_displays_the_limit() {
        let error = SourceError::TooLarge {
            name: "big.eadl".into(),
            bytes: 5_000_000_000,
        };
        assert!(error.to_string().contains("big.eadl"));
        assert!(error.to_string().contains("larger than a span can address"));
    }
}
