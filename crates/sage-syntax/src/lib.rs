//! SAGE syntax definitions.

// [TECH] This crate owns the source-facing syntax model, including AST data and source spans;
// syntax nodes should preserve enough provenance for later diagnostics.
// [ELI5] This is the compiler's record of what the programmer wrote, with locations attached so
// later stages can explain problems in the original file.

/// An opaque stable identity handle for a source file.
///
/// IDs are supplied by the source owner or registry and remain stable for the source-file
/// lifetime. They identify a source file; they are not file names or byte offsets.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct SourceId(u32);

/// A one-based line and column position in source text.
///
/// Lines and columns count from one. Columns count Unicode scalar values rather than UTF-8
/// bytes.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct LineColumn {
    line: u32,
    column: u32,
}

impl LineColumn {
    /// Returns the one-based line number.
    pub const fn line(self) -> u32 {
        self.line
    }

    /// Returns the one-based column number.
    pub const fn column(self) -> u32 {
        self.column
    }
}

impl SourceId {
    /// Creates a source identity from a caller-supplied numeric value.
    pub const fn new(value: u32) -> Self {
        Self(value)
    }

    /// Returns the numeric value of this source identity.
    pub const fn get(self) -> u32 {
        self.0
    }
}

/// A source-linked half-open byte range `[start, end)`.
///
/// Spans use byte offsets into the original UTF-8 source text. Empty spans are valid;
/// reversed ranges are rejected by [`Span::new`].
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Span {
    source: SourceId,
    start: u32,
    end: u32,
}

impl Span {
    /// Creates a span, returning `None` when `end` precedes `start`.
    pub const fn new(source: SourceId, start: u32, end: u32) -> Option<Self> {
        if end < start {
            None
        } else {
            Some(Self { source, start, end })
        }
    }

    /// Returns the source identity associated with this span.
    pub const fn source(self) -> SourceId {
        self.source
    }

    /// Returns the inclusive start byte offset.
    pub const fn start(self) -> u32 {
        self.start
    }

    /// Returns the exclusive end byte offset.
    pub const fn end(self) -> u32 {
        self.end
    }

    /// Returns the number of bytes in this span.
    pub const fn len(self) -> u32 {
        self.end - self.start
    }

    /// Returns whether this span contains no bytes.
    pub const fn is_empty(self) -> bool {
        self.start == self.end
    }

    /// Returns whether `offset` is within this half-open span.
    pub const fn contains(self, offset: u32) -> bool {
        self.start <= offset && offset < self.end
    }
}

/// An owned source identity, name, and exact source text.
///
/// `SourceFile` is intentionally limited to these source data. [`SourceFile::slice`] provides the
/// safe borrowed boundary from source spans to exact source text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    id: SourceId,
    name: String,
    text: String,
}

impl SourceFile {
    /// Creates a source file from string-like name and text values.
    ///
    /// The text is stored exactly as provided; it is not normalized or interpreted.
    pub fn new(id: SourceId, name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            id,
            name: name.into(),
            text: text.into(),
        }
    }

    /// Creates a source file from bytes, rejecting invalid UTF-8.
    pub fn from_utf8(
        id: SourceId,
        name: impl Into<String>,
        bytes: &[u8],
    ) -> Result<Self, std::str::Utf8Error> {
        let text = std::str::from_utf8(bytes)?;

        Ok(Self::new(id, name, text))
    }

    /// Returns the source identity.
    pub const fn id(&self) -> SourceId {
        self.id
    }

    /// Returns the source name.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the exact source text.
    pub fn text(&self) -> &str {
        &self.text
    }

    /// Returns the exact borrowed source text covered by `span`.
    ///
    /// Slicing succeeds only when the span belongs to this source, its offsets are in bounds,
    /// and both offsets are UTF-8 scalar boundaries. Empty spans therefore return `Some("")`.
    /// Invalid spans return `None` rather than panicking or altering the source text.
    pub fn slice(&self, span: Span) -> Option<&str> {
        if span.source() != self.id {
            return None;
        }

        self.text
            .get(usize::try_from(span.start()).ok()?..usize::try_from(span.end()).ok()?)
    }

    /// Returns the one-based line and column at a UTF-8 byte offset.
    ///
    /// The offset must be at a Unicode scalar boundary and no greater than the source length.
    /// Newline bytes belong to the preceding line: an offset at a newline is reported at the
    /// current line and column, while an offset after the complete newline sequence begins the
    /// next line at column one. LF, CRLF, and bare CR are each one newline. EOF is valid and is
    /// reported as the current position (or line one, column one for an empty file).
    pub fn line_column(&self, offset: u32) -> Option<LineColumn> {
        let offset = usize::try_from(offset).ok()?;
        if offset > self.text.len() || !self.text.is_char_boundary(offset) {
            return None;
        }

        let mut line = 1;
        let mut column = 1;
        let mut chars = self.text.char_indices().peekable();

        while let Some((index, character)) = chars.next() {
            if index == offset {
                return Some(LineColumn { line, column });
            }

            match character {
                '\r' => {
                    if chars.peek().is_some_and(|&(next_index, next_character)| {
                        next_index == index + 1 && next_character == '\n'
                    }) {
                        if offset == index + 1 {
                            return Some(LineColumn { line, column });
                        }
                        chars.next();
                    }
                    line += 1;
                    column = 1;
                }
                '\n' => {
                    line += 1;
                    column = 1;
                }
                _ => column += 1,
            }
        }

        Some(LineColumn { line, column })
    }
}

#[cfg(test)]
mod tests {
    use super::{LineColumn, SourceFile, SourceId, Span};

    #[test]
    fn source_id_exposes_its_value() {
        let id = SourceId::new(42);

        assert_eq!(id.get(), 42);
    }

    #[test]
    fn span_constructs_and_exposes_accessors() {
        let source = SourceId::new(42);
        let span = Span::new(source, 3, 11).expect("valid span");

        assert_eq!(span.source(), source);
        assert_eq!(span.start(), 3);
        assert_eq!(span.end(), 11);
        assert_eq!(span.len(), 8);
        assert!(!span.is_empty());
    }

    #[test]
    fn span_preserves_source_identity() {
        let first = Span::new(SourceId::new(1), 0, 1).expect("valid span");
        let second = Span::new(SourceId::new(2), 0, 1).expect("valid span");

        assert_ne!(first, second);
        assert_eq!(first.source(), SourceId::new(1));
        assert_eq!(second.source(), SourceId::new(2));
    }

    #[test]
    fn span_contains_offsets_using_half_open_boundaries() {
        let span = Span::new(SourceId::new(1), 3, 7).expect("valid span");

        assert!(!span.contains(2));
        assert!(span.contains(3));
        assert!(span.contains(6));
        assert!(!span.contains(7));
    }

    #[test]
    fn span_allows_empty_ranges() {
        let span = Span::new(SourceId::new(1), 5, 5).expect("empty span is valid");

        assert_eq!(span.len(), 0);
        assert!(span.is_empty());
        assert!(!span.contains(5));
    }

    #[test]
    fn span_rejects_reversed_ranges() {
        assert_eq!(Span::new(SourceId::new(1), 8, 7), None);
    }

    #[test]
    fn source_ids_are_distinct_values() {
        assert_ne!(SourceId::new(1), SourceId::new(2));
    }

    #[test]
    fn constructs_and_exposes_name_and_text() {
        let source = SourceFile::new(SourceId::new(1), "inventory.sage", "application Inventory");

        assert_eq!(source.name(), "inventory.sage");
        assert_eq!(source.text(), "application Inventory");
    }

    #[test]
    fn preserves_newlines_and_non_ascii_text_exactly() {
        let text = "application Café\r\n\tname as text\n";
        let source = SourceFile::new(SourceId::new(2), "café.sage", text);

        assert_eq!(source.text(), text);
        assert_eq!(source.name(), "café.sage");
    }

    #[test]
    fn permits_empty_text() {
        let source = SourceFile::new(SourceId::new(3), "empty.sage", "");

        assert_eq!(source.text(), "");
    }

    #[test]
    fn accepts_valid_utf8_bytes_exactly() {
        let bytes = b"caf\xc3\xa9\r\n\tname\0 as text\n";
        let source = SourceFile::from_utf8(SourceId::new(4), "inventory.sage", bytes).unwrap();

        assert_eq!(source.text().as_bytes(), bytes);
        assert_eq!(source.text(), "café\r\n\tname\0 as text\n");
    }

    #[test]
    fn rejects_invalid_utf8_bytes() {
        let result = SourceFile::from_utf8(SourceId::new(5), "invalid.sage", b"valid\xff text");

        assert!(result.is_err());
    }

    #[test]
    fn line_column_supports_empty_files_and_eof() {
        let empty = SourceFile::new(SourceId::new(9), "empty.sage", "");
        assert_eq!(
            empty.line_column(0),
            Some(LineColumn { line: 1, column: 1 })
        );

        let source = SourceFile::new(SourceId::new(10), "source.sage", "abc");
        assert_eq!(
            source.line_column(3),
            Some(LineColumn { line: 1, column: 4 })
        );
    }

    #[test]
    fn line_column_counts_ascii_columns() {
        let source = SourceFile::new(SourceId::new(11), "source.sage", "abc");

        assert_eq!(
            source.line_column(0),
            Some(LineColumn { line: 1, column: 1 })
        );
        assert_eq!(
            source.line_column(2),
            Some(LineColumn { line: 1, column: 3 })
        );
    }

    #[test]
    fn line_column_counts_unicode_scalars_not_bytes() {
        let source = SourceFile::new(SourceId::new(12), "source.sage", "aé界b");

        assert_eq!(
            source.line_column(1),
            Some(LineColumn { line: 1, column: 2 })
        );
        assert_eq!(
            source.line_column(3),
            Some(LineColumn { line: 1, column: 3 })
        );
        assert_eq!(
            source.line_column(6),
            Some(LineColumn { line: 1, column: 4 })
        );
        assert_eq!(
            source.line_column(7),
            Some(LineColumn { line: 1, column: 5 })
        );
    }

    #[test]
    fn line_column_handles_all_supported_line_endings() {
        let source = SourceFile::new(SourceId::new(13), "source.sage", "a\nb\r\nc\rd");

        assert_eq!(
            source.line_column(2),
            Some(LineColumn { line: 2, column: 1 })
        );
        assert_eq!(
            source.line_column(5),
            Some(LineColumn { line: 3, column: 1 })
        );
        assert_eq!(
            source.line_column(7),
            Some(LineColumn { line: 4, column: 1 })
        );
        assert_eq!(
            source.line_column(8),
            Some(LineColumn { line: 4, column: 2 })
        );
    }

    #[test]
    fn line_column_keeps_newline_bytes_on_preceding_line() {
        let source = SourceFile::new(SourceId::new(14), "source.sage", "a\r\nb");

        assert_eq!(
            source.line_column(1),
            Some(LineColumn { line: 1, column: 2 })
        );
        assert_eq!(
            source.line_column(2),
            Some(LineColumn { line: 1, column: 2 })
        );
        assert_eq!(
            source.line_column(3),
            Some(LineColumn { line: 2, column: 1 })
        );
    }

    #[test]
    fn line_column_rejects_non_boundary_and_out_of_range_offsets() {
        let source = SourceFile::new(SourceId::new(15), "source.sage", "aé");

        assert_eq!(source.line_column(2), None);
        assert_eq!(source.line_column(4), None);
    }

    #[test]
    fn slice_returns_normal_ascii_text() {
        let source = SourceFile::new(SourceId::new(16), "source.sage", "application Inventory");
        let span = Span::new(source.id(), 12, 21).expect("valid span");

        assert_eq!(source.slice(span), Some("Inventory"));
    }

    #[test]
    fn slice_returns_exact_unicode_text_on_scalar_boundaries() {
        let source = SourceFile::new(SourceId::new(17), "source.sage", "aé界b");
        let span = Span::new(source.id(), 1, 6).expect("valid span");

        assert_eq!(source.slice(span), Some("é界"));
    }

    #[test]
    fn slice_returns_empty_text_for_empty_spans() {
        let source = SourceFile::new(SourceId::new(18), "source.sage", "text");
        let span = Span::new(source.id(), 2, 2).expect("valid empty span");

        assert_eq!(source.slice(span), Some(""));
    }

    #[test]
    fn slice_rejects_spans_from_another_source() {
        let source = SourceFile::new(SourceId::new(19), "source.sage", "text");
        let span = Span::new(SourceId::new(20), 0, 4).expect("valid span");

        assert_eq!(source.slice(span), None);
    }

    #[test]
    fn slice_rejects_out_of_bounds_spans() {
        let source = SourceFile::new(SourceId::new(21), "source.sage", "text");
        let span = Span::new(source.id(), 0, 5).expect("valid span");

        assert_eq!(source.slice(span), None);
    }

    #[test]
    fn slice_rejects_interior_multi_byte_boundaries() {
        let source = SourceFile::new(SourceId::new(22), "source.sage", "aé");
        let span = Span::new(source.id(), 1, 2).expect("valid range");

        assert_eq!(source.slice(span), None);
    }

    #[test]
    fn retains_id_through_new() {
        let id = SourceId::new(6);
        let source = SourceFile::new(id, "inventory.sage", "application Inventory");

        assert_eq!(source.id(), id);
    }

    #[test]
    fn retains_id_through_from_utf8() {
        let id = SourceId::new(7);
        let source = SourceFile::from_utf8(id, "inventory.sage", b"application Inventory").unwrap();

        assert_eq!(source.id(), id);
    }

    #[test]
    fn clones_and_compares_as_a_value() {
        let source = SourceFile::new(SourceId::new(8), "inventory.sage", "application Inventory");
        let clone = source.clone();

        assert_eq!(source, clone);
        assert_eq!(format!("{source:?}"), format!("{clone:?}"));
    }
}
