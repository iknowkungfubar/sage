//! SAGE syntax definitions.

// [TECH] This crate owns the source-facing syntax model, including AST data and source spans;
// syntax nodes should preserve enough provenance for later diagnostics.
// [ELI5] This is the compiler's record of what the programmer wrote, with locations attached so
// later stages can explain problems in the original file.

/// An owned source name and its exact source text.
///
/// `SourceFile` is intentionally limited to owning these two pieces of source data. Source IDs,
/// spans, line and column lookup, and source slicing are separate concerns for later stages.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SourceFile {
    name: String,
    text: String,
}

impl SourceFile {
    /// Creates a source file from string-like name and text values.
    ///
    /// The text is stored exactly as provided; it is not normalized or interpreted.
    pub fn new(name: impl Into<String>, text: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            text: text.into(),
        }
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
    use super::SourceFile;

    #[test]
    fn constructs_and_exposes_name_and_text() {
        let source = SourceFile::new("inventory.sage", "application Inventory");

        assert_eq!(source.name(), "inventory.sage");
        assert_eq!(source.text(), "application Inventory");
    }

    #[test]
    fn preserves_newlines_and_non_ascii_text_exactly() {
        let text = "application Café\r\n\tname as text\n";
        let source = SourceFile::new("café.sage", text);

        assert_eq!(source.text(), text);
        assert_eq!(source.name(), "café.sage");
    }

    #[test]
    fn permits_empty_text() {
        let source = SourceFile::new("empty.sage", "");

        assert_eq!(source.text(), "");
    }

    #[test]
    fn clones_and_compares_as_a_value() {
        let source = SourceFile::new("inventory.sage", "application Inventory");
        let clone = source.clone();

        assert_eq!(source, clone);
        assert_eq!(format!("{source:?}"), format!("{clone:?}"));
    }
}
