//! Deterministic SAGE source parsing.
//!
//! [TECH] This crate implements composable application-declaration, entity-header, field-prefix,
//! indentation-prefix, and exact `text` primitive-type parser slices. Whole-number, decimal,
//! Boolean, optional, literal, initial-value, full AST, recovery, and structured diagnostic
//! parsing remain planned work.
//! [ELI5] This is the compiler's first careful-reading step: it recognizes an application's name,
//! an entity header, a field's `name as` prefix, the exact `text` type keyword, or the spaces at
//! a line's start and where that declaration appears, without guessing about later text.

use sage_syntax::{SourceFile, Span};

/// The leading indentation of one source line.
///
/// `width` is the count of leading ASCII spaces. Blank lines are neutral: they always have width
/// zero and do not participate in structural indentation decisions.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub struct Indentation {
    width: u32,
    blank: bool,
}

impl Indentation {
    /// Returns the number of leading ASCII spaces on this content line.
    pub const fn width(self) -> u32 {
        self.width
    }

    /// Returns whether this is a blank, structurally neutral line.
    pub const fn is_blank(self) -> bool {
        self.blank
    }
}

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

/// The field prefix recognized by the current parser slice.
///
/// The span covers the field name through the end of the `as` keyword. It excludes the required
/// whitespace after `as` and any following type text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedField {
    name: String,
    span: Span,
}

impl ParsedField {
    /// Returns the field's exact source spelling.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the span of the parsed `name as` prefix.
    pub const fn span(&self) -> Span {
        self.span
    }
}

/// The entity header recognized by the current parser slice.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParsedEntity {
    name: String,
    span: Span,
}

impl ParsedEntity {
    /// Returns the entity's exact source spelling.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// Returns the span from `A` through `has:`, excluding its newline.
    pub const fn span(&self) -> Span {
        self.span
    }
}

/// Deterministic syntax failures from the parser slices.
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
    /// No entity declaration starts at the supplied offset (after blank lines).
    MissingEntityDeclaration,
    /// An entity declaration starts, but its name is absent.
    MissingEntityName,
    /// An entity name is not a valid ASCII upper identifier.
    InvalidEntityName,
    /// A valid entity name is not followed by the required `has:` clause.
    MissingHasClause,
    /// The entity header contains unexpected syntax.
    MalformedEntityHeader,
    /// No field declaration starts at the supplied offset (after blank lines).
    MissingFieldDeclaration,
    /// A field declaration starts, but its name is absent.
    MissingFieldName,
    /// A field name is not a valid ASCII lower identifier.
    InvalidFieldName,
    /// A valid field name is not followed by the required `as` clause.
    MissingAsClause,
    /// The field prefix contains malformed `as` syntax.
    MalformedFieldPrefix,
    /// The required whitespace after `as` is not followed by type-start text.
    MissingFieldTypeStart,
    /// No text primitive type starts at the supplied offset.
    MissingTextType,
    /// The source at the supplied offset is not the exact `text` primitive type.
    InvalidTextType,
    /// A source offset cannot be represented by the `u32`-based span type.
    SourceTooLarge,
    /// The requested indentation offset is outside the source file.
    InvalidIndentationOffset,
    /// The requested indentation offset is not the beginning of a line.
    IndentationNotAtLineStart,
    /// A tab occurs in the structural indentation prefix.
    TabInIndentation,
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

/// Parses the indentation prefix at a byte offset that begins a source line.
///
/// Leading ASCII spaces define the width. Tabs in that structural prefix are rejected, while
/// tabs after the first non-whitespace byte are ordinary source content. Blank lines are neutral
/// and return width zero. This primitive does not maintain an INDENT/DEDENT stack or parse line
/// content.
pub fn parse_indentation_at(source: &SourceFile, offset: u32) -> Result<Indentation, ParseError> {
    let bytes = source.text().as_bytes();
    let source_length = u32::try_from(bytes.len()).map_err(|_| ParseError::SourceTooLarge)?;
    if offset > source_length {
        return Err(ParseError::InvalidIndentationOffset);
    }

    let start = usize::try_from(offset).map_err(|_| ParseError::InvalidIndentationOffset)?;
    if start != 0 {
        let previous = bytes.get(
            start
                .checked_sub(1)
                .ok_or(ParseError::InvalidIndentationOffset)?,
        );
        let at_line_start = match previous {
            Some(b'\n') => true,
            Some(b'\r') => bytes.get(start) != Some(&b'\n'),
            _ => false,
        };
        if !at_line_start {
            return Err(ParseError::IndentationNotAtLineStart);
        }
    }

    let mut index = start;
    let mut width = 0_u32;
    while let Some(byte) = bytes.get(index).copied() {
        match byte {
            b' ' => {
                width = width.checked_add(1).ok_or(ParseError::SourceTooLarge)?;
                index = index.checked_add(1).ok_or(ParseError::SourceTooLarge)?;
            }
            b'\t' => return Err(ParseError::TabInIndentation),
            b'\n' | b'\r' => {
                return Ok(Indentation {
                    width: 0,
                    blank: true,
                });
            }
            _ => {
                return Ok(Indentation {
                    width,
                    blank: false,
                });
            }
        }
    }

    Ok(Indentation {
        width: 0,
        blank: true,
    })
}

/// Parses a field's `name as` prefix at a byte offset, leaving its type text unparsed.
///
/// Zero or more horizontal-space-only blank lines may precede the prefix. The offset is a byte
/// offset so callers can compose this slice with later parsing stages.
pub fn parse_field_at(source: &SourceFile, offset: u32) -> Result<ParsedField, ParseError> {
    let bytes = source.text().as_bytes();
    let _source_length = u32::try_from(bytes.len()).map_err(|_| ParseError::SourceTooLarge)?;
    let mut start = usize::try_from(offset).map_err(|_| ParseError::SourceTooLarge)?;
    if start > bytes.len() {
        return Err(ParseError::MissingFieldDeclaration);
    }

    while let Some(newline_length) = blank_line_newline_length(bytes, start) {
        start = start
            .checked_add(newline_length)
            .ok_or(ParseError::SourceTooLarge)?;
    }

    let first = match bytes.get(start) {
        Some(byte) if *byte == b'\n' || *byte == b'\r' => return Err(ParseError::MissingFieldName),
        Some(byte) => *byte,
        None => return Err(ParseError::MissingFieldDeclaration),
    };
    if !first.is_ascii_lowercase() {
        return Err(ParseError::InvalidFieldName);
    }

    let name_end = scan_field_name_end(bytes, start)?;
    if bytes.get(name_end).is_some_and(|byte| *byte >= 0x80) {
        return Err(ParseError::InvalidFieldName);
    }
    if !bytes
        .get(name_end)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        return Err(ParseError::MissingAsClause);
    }

    let mut as_start = name_end;
    while bytes
        .get(as_start)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        as_start = as_start.checked_add(1).ok_or(ParseError::SourceTooLarge)?;
    }
    if !bytes[as_start..].starts_with(b"as") {
        return Err(ParseError::MissingAsClause);
    }

    let as_end = as_start.checked_add(2).ok_or(ParseError::SourceTooLarge)?;
    if bytes
        .get(as_end)
        .is_some_and(|byte| is_identifier_tail(*byte))
    {
        return Err(ParseError::MalformedFieldPrefix);
    }
    if !bytes
        .get(as_end)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        return if bytes
            .get(as_end)
            .is_some_and(|byte| matches!(*byte, b'\n' | b'\r'))
        {
            Err(ParseError::MissingFieldTypeStart)
        } else {
            Err(ParseError::MalformedFieldPrefix)
        };
    }

    let mut type_start = as_end;
    while bytes
        .get(type_start)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        type_start = type_start
            .checked_add(1)
            .ok_or(ParseError::SourceTooLarge)?;
    }
    if bytes
        .get(type_start)
        .is_none_or(|byte| matches!(*byte, b'\n' | b'\r'))
    {
        return Err(ParseError::MissingFieldTypeStart);
    }

    let start_u32 = u32::try_from(start).map_err(|_| ParseError::SourceTooLarge)?;
    let as_end_u32 = u32::try_from(as_end).map_err(|_| ParseError::SourceTooLarge)?;
    let span = Span::new(source.id(), start_u32, as_end_u32).ok_or(ParseError::SourceTooLarge)?;
    let name_start_u32 = u32::try_from(start).map_err(|_| ParseError::SourceTooLarge)?;
    let name_end_u32 = u32::try_from(name_end).map_err(|_| ParseError::SourceTooLarge)?;
    let name_span =
        Span::new(source.id(), name_start_u32, name_end_u32).ok_or(ParseError::SourceTooLarge)?;
    let name = source
        .slice(name_span)
        .ok_or(ParseError::SourceTooLarge)?
        .to_owned();

    Ok(ParsedField { name, span })
}

/// The exact `text` primitive-type parser result.
///
/// This is a parser result containing source provenance, not a semantic type or AST node. The
/// span covers only the four-byte `text` keyword; following type or field text remains unparsed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ParsedTextType {
    span: Span,
}

impl ParsedTextType {
    /// Returns the span of the exact `text` keyword.
    pub const fn span(self) -> Span {
        self.span
    }
}

/// Parses the exact `text` primitive type at a byte offset.
///
/// Leading spaces and tabs at the supplied offset are skipped, but newlines are never crossed.
/// Only the keyword is consumed; a following initial clause or any other text is left for a later
/// parser slice.
pub fn parse_text_type_at(source: &SourceFile, offset: u32) -> Result<ParsedTextType, ParseError> {
    let bytes = source.text().as_bytes();
    let source_length = u32::try_from(bytes.len()).map_err(|_| ParseError::SourceTooLarge)?;
    let mut start = usize::try_from(offset).map_err(|_| ParseError::SourceTooLarge)?;
    if offset > source_length || start > bytes.len() {
        return Err(ParseError::MissingTextType);
    }

    while bytes
        .get(start)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        start = start.checked_add(1).ok_or(ParseError::SourceTooLarge)?;
    }

    match bytes.get(start) {
        None | Some(b'\n') | Some(b'\r') => return Err(ParseError::MissingTextType),
        Some(b't') => {}
        Some(_) => return Err(ParseError::InvalidTextType),
    }
    let end = start
        .checked_add(b"text".len())
        .ok_or(ParseError::SourceTooLarge)?;
    if bytes.get(start..end) != Some(b"text") {
        return Err(ParseError::InvalidTextType);
    }

    if bytes
        .get(end)
        .is_some_and(|byte| !matches!(*byte, b'\n' | b'\r' | b',' | b' ' | b'\t'))
    {
        return Err(ParseError::InvalidTextType);
    }

    let start = u32::try_from(start).map_err(|_| ParseError::SourceTooLarge)?;
    let end = u32::try_from(end).map_err(|_| ParseError::SourceTooLarge)?;
    let span = Span::new(source.id(), start, end).ok_or(ParseError::SourceTooLarge)?;
    Ok(ParsedTextType { span })
}

fn scan_field_name_end(bytes: &[u8], start: usize) -> Result<usize, ParseError> {
    let mut end = start.checked_add(1).ok_or(ParseError::SourceTooLarge)?;
    while bytes.get(end).is_some_and(|byte| is_identifier_tail(*byte)) {
        end = end.checked_add(1).ok_or(ParseError::SourceTooLarge)?;
    }
    Ok(end)
}

/// Parses an entity header at a byte offset, leaving its field body unparsed.
pub fn parse_entity_at(source: &SourceFile, offset: u32) -> Result<ParsedEntity, ParseError> {
    let bytes = source.text().as_bytes();
    let _source_length = u32::try_from(bytes.len()).map_err(|_| ParseError::SourceTooLarge)?;
    let mut start = usize::try_from(offset).map_err(|_| ParseError::SourceTooLarge)?;
    if start > bytes.len() {
        return Err(ParseError::MissingEntityDeclaration);
    }

    while let Some(newline_length) = blank_line_newline_length(bytes, start) {
        start += newline_length;
    }

    if bytes.get(start) != Some(&b'A') {
        return Err(ParseError::MissingEntityDeclaration);
    }

    let after_a = start + 1;
    if !bytes
        .get(after_a)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        return Err(ParseError::MissingEntityName);
    }

    let mut name_start = after_a;
    while bytes
        .get(name_start)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        name_start += 1;
    }
    if bytes.get(name_start).is_none() || matches!(bytes[name_start], b'\n' | b'\r') {
        return Err(ParseError::MissingEntityName);
    }
    if !bytes[name_start].is_ascii_uppercase() {
        return Err(ParseError::InvalidEntityName);
    }

    let mut name_end = name_start + 1;
    while bytes
        .get(name_end)
        .is_some_and(|byte| is_identifier_tail(*byte))
    {
        name_end += 1;
    }
    if bytes.get(name_end).is_none() || matches!(bytes[name_end], b'\n' | b'\r') {
        return Err(ParseError::MissingHasClause);
    }
    if !is_horizontal_space(bytes[name_end]) {
        return Err(ParseError::InvalidEntityName);
    }

    let mut clause_start = name_end;
    while bytes
        .get(clause_start)
        .is_some_and(|byte| is_horizontal_space(*byte))
    {
        clause_start += 1;
    }
    if !bytes[clause_start..].starts_with(b"has:") {
        return if bytes[clause_start..].starts_with(b"has") {
            Err(ParseError::MalformedEntityHeader)
        } else {
            Err(ParseError::MissingHasClause)
        };
    }
    let end = clause_start + b"has:".len();
    let newline_length = match bytes.get(end) {
        Some(b'\n') | Some(b'\r') => newline_length_at(bytes, end),
        Some(_) => return Err(ParseError::MalformedEntityHeader),
        None => return Err(ParseError::MissingNewline),
    };

    let start_u32 = u32::try_from(start).map_err(|_| ParseError::SourceTooLarge)?;
    let end_u32 = u32::try_from(end).map_err(|_| ParseError::SourceTooLarge)?;
    let span = Span::new(source.id(), start_u32, end_u32).ok_or(ParseError::SourceTooLarge)?;
    let name_start_u32 = u32::try_from(name_start).map_err(|_| ParseError::SourceTooLarge)?;
    let name_end_u32 = u32::try_from(name_end).map_err(|_| ParseError::SourceTooLarge)?;
    let name_span =
        Span::new(source.id(), name_start_u32, name_end_u32).ok_or(ParseError::SourceTooLarge)?;
    let name = source
        .slice(name_span)
        .ok_or(ParseError::SourceTooLarge)?
        .to_owned();

    let _ = newline_length;
    Ok(ParsedEntity { name, span })
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
    use super::{
        parse_application, parse_entity_at, parse_field_at, parse_indentation_at,
        parse_text_type_at, ParseError,
    };
    use sage_syntax::{SourceFile, SourceId};

    fn source(text: &str) -> SourceFile {
        SourceFile::new(SourceId::new(7), "test.sage", text)
    }

    #[test]
    fn parses_indentation_widths_and_blank_lines() {
        for (text, expected_width, blank) in [
            ("content", 0, false),
            ("    content", 4, false),
            ("  content", 2, false),
            ("      content", 6, false),
            (" \n", 0, true),
            ("    \r\n", 0, true),
            ("  \r", 0, true),
            ("\n", 0, true),
            ("", 0, true),
        ] {
            let indentation = parse_indentation_at(&source(text), 0).expect("valid line start");
            assert_eq!(indentation.width(), expected_width);
            assert_eq!(indentation.is_blank(), blank);
        }
    }

    #[test]
    fn parses_indentation_at_each_supported_line_start() {
        let file = source("root\n    child\r\n\r    sibling");
        let child_offset = 5;
        let sibling_offset = 17;

        assert_eq!(
            parse_indentation_at(&file, child_offset).unwrap().width(),
            4
        );
        assert!(parse_indentation_at(&file, 16).unwrap().is_blank());
        assert_eq!(
            parse_indentation_at(&file, sibling_offset).unwrap().width(),
            4
        );
    }

    #[test]
    fn rejects_tabs_only_in_structural_prefix() {
        for text in ["\tcontent", "  \tcontent", "\t\n"] {
            assert_eq!(
                parse_indentation_at(&source(text), 0),
                Err(ParseError::TabInIndentation)
            );
        }

        let file = source("content\tstill content");
        assert_eq!(parse_indentation_at(&file, 0).unwrap().width(), 0);
    }

    #[test]
    fn rejects_invalid_indentation_offsets() {
        let file = source("content\n    child");

        assert_eq!(
            parse_indentation_at(&file, 3),
            Err(ParseError::IndentationNotAtLineStart)
        );
        assert_eq!(
            parse_indentation_at(&file, 99),
            Err(ParseError::InvalidIndentationOffset)
        );
        assert_eq!(
            parse_indentation_at(&file, 9),
            Err(ParseError::IndentationNotAtLineStart)
        );
    }

    #[test]
    fn indentation_widths_are_orderable_for_future_block_consistency() {
        let file = source("    child\n      deeper");
        let child = parse_indentation_at(&file, 0).unwrap();
        let deeper = parse_indentation_at(&file, 10).unwrap();

        assert!(child < deeper);
        assert_eq!(child.width(), 4);
        assert_eq!(deeper.width(), 6);
        assert!(!child.is_blank());
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

    #[test]
    fn parses_entity_header_and_preserves_exact_name_and_span() {
        let file = source("A Product has:\n");
        let entity = parse_entity_at(&file, 0).expect("valid entity header");

        assert_eq!(entity.name(), "Product");
        assert_eq!(entity.span().source(), file.id());
        assert_eq!(file.slice(entity.span()), Some("A Product has:"));
        assert_eq!(entity.span().start(), 0);
        assert_eq!(entity.span().end(), 14);
    }

    #[test]
    fn accepts_entity_newline_styles_and_horizontal_spacing() {
        for text in [
            "A\tProduct  has:\n",
            "A Product has:\r\n",
            "A Product has:\r",
        ] {
            let file = source(text);
            assert_eq!(
                parse_entity_at(&file, 0).expect("valid entity").name(),
                "Product"
            );
        }
    }

    #[test]
    fn skips_leading_blank_separator_lines() {
        let file = source(" \t\r\n\t\nA Product has:\n");
        let entity = parse_entity_at(&file, 0).expect("valid entity");

        assert_eq!(entity.span().start(), 6);
        assert_eq!(file.slice(entity.span()), Some("A Product has:"));
    }

    #[test]
    fn leaves_later_entity_body_text_unparsed() {
        let file = source("A Product has:\n    name as text\n");
        assert!(parse_entity_at(&file, 0).is_ok());
    }

    #[test]
    fn parses_entity_at_composed_offset_after_application() {
        let file = source("application Inventory\n\nA Product has:\n");
        let application = parse_application(&file).expect("valid application");
        let entity = parse_entity_at(&file, application.span().end()).expect("valid entity");

        assert_eq!(entity.name(), "Product");
        assert_eq!(entity.span().start(), 23);
    }

    #[test]
    fn rejects_application_at_entity_offset() {
        assert_eq!(
            parse_entity_at(&source("application Inventory\n"), 0),
            Err(ParseError::MissingEntityDeclaration)
        );
    }

    #[test]
    fn rejects_missing_and_invalid_entity_names() {
        for text in ["A\n", "A   \r\n", "AProduct has:\n"] {
            assert!(matches!(
                parse_entity_at(&source(text), 0),
                Err(ParseError::MissingEntityName)
            ));
        }
        for text in ["A product has:\n", "A 1Product has:\n", "A Café has:\n"] {
            assert!(matches!(
                parse_entity_at(&source(text), 0),
                Err(ParseError::InvalidEntityName)
            ));
        }
    }

    #[test]
    fn rejects_missing_or_malformed_has_clause() {
        assert_eq!(
            parse_entity_at(&source("A Product\n"), 0),
            Err(ParseError::MissingHasClause)
        );
        assert_eq!(
            parse_entity_at(&source("A Product is:\n"), 0),
            Err(ParseError::MissingHasClause)
        );
        assert_eq!(
            parse_entity_at(&source("A Product hasx:\n"), 0),
            Err(ParseError::MalformedEntityHeader)
        );
    }

    #[test]
    fn rejects_trailing_header_text_and_missing_newline() {
        for text in ["A Product has: \n", "A Product has: extra\n"] {
            assert_eq!(
                parse_entity_at(&source(text), 0),
                Err(ParseError::MalformedEntityHeader)
            );
        }
        assert_eq!(
            parse_entity_at(&source("A Product has:"), 0),
            Err(ParseError::MissingNewline)
        );
    }

    #[test]
    fn parses_field_prefix_and_preserves_exact_name_and_span() {
        let file = source("quantity_2 as text\n");
        let field = parse_field_at(&file, 0).expect("valid field prefix");

        assert_eq!(field.name(), "quantity_2");
        assert_eq!(field.span().source(), file.id());
        assert_eq!(file.slice(field.span()), Some("quantity_2 as"));
        assert_eq!(field.span().start(), 0);
        assert_eq!(field.span().end(), 13);
    }

    #[test]
    fn accepts_horizontal_spacing_and_leading_blank_lines() {
        let file = source(" \t\r\n\t\nname\t\tas\t\ttext\n");
        let field = parse_field_at(&file, 0).expect("valid field prefix");

        assert_eq!(field.name(), "name");
        assert_eq!(file.slice(field.span()), Some("name\t\tas"));
    }

    #[test]
    fn parses_field_at_composed_nonzero_offset_and_leaves_type_unparsed() {
        let file = source("header\nname as future type, initially anything\n");
        let field = parse_field_at(&file, 7).expect("valid field prefix");

        assert_eq!(field.name(), "name");
        assert_eq!(file.slice(field.span()), Some("name as"));
    }

    #[test]
    fn parses_text_type_and_preserves_exact_keyword_span() {
        let file = source("text");
        let text = parse_text_type_at(&file, 0).expect("valid text type");

        assert_eq!(text.span().source(), file.id());
        assert_eq!(text.span().start(), 0);
        assert_eq!(text.span().end(), 4);
        assert_eq!(file.slice(text.span()), Some("text"));
    }

    #[test]
    fn parses_text_type_after_field_prefix_spacing() {
        let file = source("name as   text, initially anything");
        let text = parse_text_type_at(&file, 7).expect("valid text type");

        assert_eq!(text.span().start(), 10);
        assert_eq!(file.slice(text.span()), Some("text"));
    }

    #[test]
    fn accepts_text_type_delimiters_without_parsing_following_text() {
        for suffix in ["\n", "\r\n", ", initially anything", " whatever", ""] {
            let file = source(&format!("text{suffix}"));
            assert!(parse_text_type_at(&file, 0).is_ok(), "suffix: {suffix:?}");
        }
    }

    #[test]
    fn rejects_missing_and_invalid_text_types() {
        for input in ["", "\n", "\r", " \t\r\n"] {
            assert_eq!(
                parse_text_type_at(&source(input), 0),
                Err(ParseError::MissingTextType)
            );
        }
        for input in [
            "textual",
            "text_1",
            "text1",
            "Text",
            "tExT",
            "têxt",
            "whole number",
        ] {
            assert_eq!(
                parse_text_type_at(&source(input), 0),
                Err(ParseError::InvalidTextType)
            );
        }
    }

    #[test]
    fn rejects_offsets_that_cannot_start_a_text_type() {
        let file = source("text\ntext");
        assert_eq!(
            parse_text_type_at(&file, 4),
            Err(ParseError::MissingTextType)
        );
        assert_eq!(
            parse_text_type_at(&file, 99),
            Err(ParseError::MissingTextType)
        );
    }

    #[test]
    fn rejects_field_prefix_errors() {
        for text in ["", "\n", " \t\r\n"] {
            assert!(matches!(
                parse_field_at(&source(text), 0),
                Err(ParseError::MissingFieldDeclaration | ParseError::MissingFieldName)
            ));
        }
        for text in ["Name as text", "1name as text", "naïve as text"] {
            assert_eq!(
                parse_field_at(&source(text), 0),
                Err(ParseError::InvalidFieldName)
            );
        }
        for text in ["nameas text", "nameas text", "name a text", "name as-text"] {
            let expected = if text == "name as-text" {
                ParseError::MalformedFieldPrefix
            } else {
                ParseError::MissingAsClause
            };
            assert_eq!(parse_field_at(&source(text), 0), Err(expected));
        }
        assert_eq!(
            parse_field_at(&source("name as"), 0),
            Err(ParseError::MalformedFieldPrefix)
        );
        for text in ["name as\n", "name as \n", "name as \r\n", "name as \r"] {
            assert_eq!(
                parse_field_at(&source(text), 0),
                Err(ParseError::MissingFieldTypeStart)
            );
        }
        assert_eq!(
            parse_field_at(&source("name as text"), 99),
            Err(ParseError::MissingFieldDeclaration)
        );
    }
}
