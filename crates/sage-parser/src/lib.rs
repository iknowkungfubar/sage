//! Deterministic SAGE source parsing.
//!
//! [TECH] This crate currently implements the application-declaration parser slice. Entity
//! parsing, the full AST, recovery, and diagnostics are planned for later roadmap issues.
//! [ELI5] This is the compiler's first careful-reading step: it recognizes the application's name
//! and where that declaration appears, without guessing about the rest of the file.

use sage_syntax::{SourceFile, Span};

/// The application declaration recognized by the current parser slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedApplication {
    name: String,
    span: Span,
}

impl ParsedApplication {
    /// Returns the application's exact source spelling.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the span of the declaration, excluding its newline.
    pub const fn span(&self) -> Span {
        self.span
    }
}

/// Deterministic syntax failures from [`parse_application`].
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ParseError {
    /// The source does not begin with an application declaration after blank lines.
    MissingApplicationDeclaration,
    /// The `application` keyword is not followed by a name.
    MissingApplicationName,
    /// The name is not an ASCII upper identifier, or the declaration has extra characters.
    InvalidUpperIdentifier,
    /// The application declaration is not terminated by LF, CRLF, or bare CR.
    MissingNewline,
    /// A source offset cannot be represented by the `u32`-based span type.
    SourceTooLarge,
}

/// Parses the first application declaration in a source file.
///
/// Leading blank lines are allowed. The declaration itself must be an exact application line,
/// while any text after its newline is intentionally left unparsed for later parser slices.
pub fn parse_application(source: &SourceFile) -> Result<ParsedApplication, ParseError> {
    let text = source.text();
    let bytes = text.as_bytes();
    let _source_length = u32::try_from(bytes.len()).map_err(|_| ParseError::SourceTooLarge)?;

    let mut start = 0;
    while let Some(newline_length) = blank_line_newline_length(bytes, start) {
        start += newline_length;
    }

    if !bytes[start..].starts_with(b"application") {
        return Err(ParseError::MissingApplicationDeclaration);
    }

    let keyword_end = start + b"application".len();
    if !bytes
        .get(keyword_end)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        return Err(ParseError::MissingApplicationName);
    }

    let mut name_start = keyword_end;
    while bytes
        .get(name_start)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        name_start += 1;
    }

    if name_start == bytes.len() || bytes[name_start] == b'\n' || bytes[name_start] == b'\r' {
        return Err(ParseError::MissingApplicationName);
    }

    if !bytes[name_start].is_ascii_uppercase() {
        return Err(ParseError::InvalidUpperIdentifier);
    }

    let mut name_end = name_start + 1;
    while bytes
        .get(name_end)
        .is_some_and(|byte| is_identifier_tail(*byte))
    {
        name_end += 1;
    }

    let newline_length = match bytes.get(name_end) {
        Some(b'\n') | Some(b'\r') => newline_length_at(bytes, name_end),
        Some(_) => return Err(ParseError::InvalidUpperIdentifier),
        None => return Err(ParseError::MissingNewline),
    };

    let start = u32::try_from(start).map_err(|_| ParseError::SourceTooLarge)?;
    let end = u32::try_from(name_end).map_err(|_| ParseError::SourceTooLarge)?;
    let span = Span::new(source.id(), start, end).ok_or(ParseError::SourceTooLarge)?;
    let name_start = u32::try_from(name_start).map_err(|_| ParseError::SourceTooLarge)?;
    let name_end = u32::try_from(name_end).map_err(|_| ParseError::SourceTooLarge)?;
    let name_span =
        Span::new(source.id(), name_start, name_end).ok_or(ParseError::SourceTooLarge)?;
    let name = source
        .slice(name_span)
        .ok_or(ParseError::SourceTooLarge)?
        .to_owned();

    // The newline length is deliberately consumed only for validation. The returned span ends
    // before it, and the rest of the source remains outside this parser slice.
    let _ = newline_length;
    Ok(ParsedApplication { name, span })
}

fn is_horizontal_space(byte: u8) -> bool {
    matches!(byte, b' ' | b'\t')
}

fn is_identifier_tail(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'_'
}

fn newline_length_at(bytes: &[u8], index: usize) -> usize {
    if bytes[index] == b'\r' && bytes.get(index + 1) == Some(&b'\n') {
        2
    } else {
        1
    }
}

fn blank_line_newline_length(bytes: &[u8], start: usize) -> Option<usize> {
    let mut index = start;
    while bytes
        .get(index)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        index += 1;
    }

    match bytes.get(index) {
        Some(b'\n') | Some(b'\r') => Some(newline_length_at(bytes, index)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::{parse_application, ParseError};
    use sage_syntax::{SourceFile, SourceId};

    fn source(text: &str) -> SourceFile {
        SourceFile::new(SourceId::new(7), "test.sage", text)
    }

    #[test]
    fn parses_lf_and_preserves_name_and_declaration_span() {
        let file = source("application Inventory\n");
        let application = parse_application(&file).expect("valid application");

        assert_eq!(application.name(), "Inventory");
        assert_eq!(application.span().source(), file.id());
        assert_eq!(
            file.slice(application.span()),
            Some("application Inventory")
        );
        assert_eq!(application.span().start(), 0);
        assert_eq!(application.span().end(), 21);
    }

    #[test]
    fn accepts_crlf_and_bare_cr() {
        for text in ["application Inventory\r\n", "application Inventory\r"] {
            let file = source(text);
            let application = parse_application(&file).expect("valid application");
            assert_eq!(
                file.slice(application.span()),
                Some("application Inventory")
            );
        }
    }

    #[test]
    fn accepts_leading_blank_lines_and_horizontal_spacing() {
        let file = source(" \t\r\n\t\napplication\t\tInventory\n");
        let application = parse_application(&file).expect("valid application");

        assert_eq!(application.name(), "Inventory");
        assert_eq!(
            file.slice(application.span()),
            Some("application\t\tInventory")
        );
    }

    #[test]
    fn leaves_later_text_unparsed() {
        let file = source("application Inventory\nnot SAGE yet\n");
        assert!(parse_application(&file).is_ok());
    }

    #[test]
    fn rejects_missing_declaration() {
        assert_eq!(
            parse_application(&source("")),
            Err(ParseError::MissingApplicationDeclaration)
        );
        assert_eq!(
            parse_application(&source("\n \t\r\n")),
            Err(ParseError::MissingApplicationDeclaration)
        );
    }

    #[test]
    fn rejects_keyword_prefix() {
        assert_eq!(
            parse_application(&source("applicationInventory\n")),
            Err(ParseError::MissingApplicationName)
        );
    }

    #[test]
    fn rejects_missing_and_invalid_names() {
        for text in ["application\n", "application   \r\n"] {
            assert_eq!(
                parse_application(&source(text)),
                Err(ParseError::MissingApplicationName)
            );
        }
        for text in [
            "application inventory\n",
            "application 1Inventory\n",
            "application Inventory-name\n",
        ] {
            assert_eq!(
                parse_application(&source(text)),
                Err(ParseError::InvalidUpperIdentifier)
            );
        }
    }

    #[test]
    fn rejects_trailing_spaces_or_text_after_name() {
        for text in ["application Inventory \n", "application Inventory extra\n"] {
            assert_eq!(
                parse_application(&source(text)),
                Err(ParseError::InvalidUpperIdentifier)
            );
        }
    }

    #[test]
    fn rejects_missing_final_newline() {
        assert_eq!(
            parse_application(&source("application Inventory")),
            Err(ParseError::MissingNewline)
        );
    }
}
