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

/// An owned source identity, name, and exact source text.
///
/// `SourceFile` is intentionally limited to these source data. Byte spans, line and column
/// lookup, and source slicing are separate concerns for later stages.
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
}

#[cfg(test)]
mod tests {
    use super::{SourceFile, SourceId};

    #[test]
    fn source_id_exposes_its_value() {
        let id = SourceId::new(42);

        assert_eq!(id.get(), 42);
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
