# SAGE Grammar

**Project:** SAGE  
**Version:** Initial SAGE 0.1 grammar  
**Status:** Accepted target grammar for SAGE 0.1; parser implementation pending
**Document purpose:** Define the deterministic target source grammar for the language kernel

## 1. Scope

This document normatively defines the accepted target structure of SAGE 0.1 source, independently of the current parser implementation status. It describes the source form that a conforming future parser is expected to recognize; the repository does not yet implement that parser.

The target grammar covers:

- application declarations,
- entity declarations,
- field declarations,
- primitive types,
- optional types,
- initial values,
- literals,
- indentation.

When parser implementation begins, its behavior must conform to this document. Any discrepancy between the implementation and this accepted target grammar must be resolved rather than allowed to persist.

## 2. Grammar Notation

This document uses a simplified EBNF-like notation.

```text
rule ::= production
```

Choice:

```text
a | b
```

Optional:

```text
[ value ]
```

Repetition:

```text
{ value }
```

One or more:

```text
value+
```

Terminals appear in quotes.

Indentation is represented using `INDENT`, `DEDENT`, and `NEWLINE`.

## 3. Top-Level Grammar

```text
source_file ::=
    blank_line*
    application_decl
    top_level_separator*
    entity_decl*
    blank_line*
    EOF
```

The SAGE 0.1 target grammar requires exactly one application declaration before entity declarations.

## 4. Application Declaration

```text
application_decl ::=
    "application"
    horizontal_space+
    application_name
    NEWLINE
```

## 5. Application Name

```text
application_name ::= <upper_identifier>
```

## 6. Entity Declaration

```text
entity_decl ::=
    "A"
    horizontal_space+
    entity_name
    horizontal_space+
    "has:"
    NEWLINE
    INDENT
    field_decl+
    DEDENT
```

## 7. Entity Name

```text
entity_name ::= <upper_identifier>
```

## 8. Field Declaration

```text
field_decl ::=
    field_name
    horizontal_space+
    "as"
    horizontal_space+
    type_expr
    initial_clause?
    NEWLINE
```

## 9. Field Name

Initial grammar:

```text
field_name ::= <lower_identifier>
```

Multi-word fields are not stable SAGE 0.1 syntax.

## 10. Type Expressions

```text
type_expr ::=
      primitive_type
    | optional_type
```

## 11. Primitive Types

```text
primitive_type ::=
      "text"
    | "whole" horizontal_space+ "number"
    | "decimal" horizontal_space+ "number"
    | "yes" horizontal_space+ "or" horizontal_space+ "no"
```

## 12. Optional Types

```text
optional_type ::=
    "optional"
    horizontal_space+
    primitive_type
```

Nested optionals are invalid in SAGE 0.1.

## 13. Initial Clause

```text
initial_clause ::=
    ","
    horizontal_space*
    "initially"
    horizontal_space+
    literal
```

## 14. Literals

```text
literal ::=
      text_literal
    | whole_number_literal
    | decimal_number_literal
    | boolean_literal
```

## 15. Text Literals

Conceptual grammar:

```text
text_literal ::=
    '"'
    text_character*
    '"'
```

Escape sequences should be specified before stable release.

## 16. Whole-Number Literals

Conceptual grammar:

```text
whole_number_literal ::=
    [ "-" ]
    digit+
```

If negative numbers are not supported in the first parser implementation, narrow the grammar accordingly.

## 17. Decimal Literals

Conceptual grammar:

```text
decimal_number_literal ::=
    [ "-" ]
    digit+
    "."
    digit+
```

Ambiguous forms such as `.`, `1.`, or `.5` should be rejected unless explicitly adopted.

## 18. Boolean Literals

```text
boolean_literal ::=
      "yes"
    | "no"
```

## 19. Identifiers

Initial lexical rules:

```text
upper_identifier ::=
    upper_alpha
    { alpha | digit | "_" }

lower_identifier ::=
    lower_alpha
    { alpha | digit | "_" }
```

Unicode identifier support should be an explicit future decision.

## 20. Reserved Words

Initial reserved words and phrases include:

```text
application
A
has
as
optional
initially
text
whole
number
decimal
yes
or
no
```

## 21. Newlines

The target parser should accept common platform line endings and normalize them to the same logical newline representation.

## 22. Horizontal Whitespace

```text
horizontal_space ::= " " | "\t"
```

Indentation policy may prohibit tabs at line beginnings.

## 23. Indentation Model

Indentation introduces nested blocks. Structural indentation is the count of leading ASCII spaces
at a line start. Tabs in that leading structural prefix are invalid; a tab after content begins is
ordinary content.

Conceptually:

```text
A Product has: NEWLINE
INDENT
name as text NEWLINE
quantity as whole number NEWLINE
DEDENT
```

A block's child lines must use one consistent indentation width. Blank lines are neutral and do
not change indentation or terminate a block by themselves. The formatter may eventually
standardize on four spaces.

The current parser implementation exposes only deterministic line-prefix inspection through
`parse_indentation_at`; full `INDENT`/`DEDENT` handling and parent/child validation remain future
parser work.

## 24. Tabs

Tabs are not permitted at line beginnings when they are part of structural indentation. This
policy applies after any leading spaces as well. Tabs after the first non-whitespace content byte
are not indentation and are left to later content parsing.

## 25. Blank Lines

Blank lines may appear after the application declaration, between entity declarations, and at the end of the file.

Blank lines inside an indented entity block should not terminate the block by themselves.

## 26. Comments

The accepted SAGE 0.1 target currently defines no comment syntax. Comment syntax remains an open decision; do not assume one until it is formally specified.

## 27. Example Parse

Source:

```sage
application Inventory

A Product has:
    name as text
    quantity as whole number, initially 0
```

Conceptual syntax:

```text
SourceFile
├── ApplicationDecl
│   └── name: Inventory
└── EntityDecl
    ├── name: Product
    └── fields
        ├── FieldDecl
        │   ├── name: name
        │   └── type: Text
        └── FieldDecl
            ├── name: quantity
            ├── type: WholeNumber
            └── initial: Integer(0)
```

## 28. Parse vs. Semantic Errors

```sage
quantity as whole number, initially "zero"
```

is syntactically valid but semantically invalid.

A conforming parser should accept it as syntactically valid; the semantic analyzer should reject it.

## 29. Duplicate Declarations

Duplicate names are semantic errors, not grammar errors.

## 30. Unknown Types

Preferred direction:

> Preserve enough structure to produce a specific "unknown type" diagnostic rather than a generic parse failure.

## 31. Parser Recovery

Potential synchronization points for the future parser include newline, the next field at the current indentation, and the next top-level entity declaration.

Recovery must not silently invent semantics.

## 32. Grammar Evolution

When modifying the grammar:

1. update this document,
2. update parser tests,
3. update `LANGUAGE.md` when semantics change,
4. update examples,
5. add diagnostics for newly invalid constructs,
6. consider an ADR for significant syntax decisions.

## 33. Canonical SAGE 0.1 Grammar

```text
source_file ::=
    blank_line*
    application_decl
    top_level_separator*
    entity_decl*
    blank_line*
    EOF

application_decl ::=
    "application" SP+ application_name NEWLINE

application_name ::=
    upper_identifier

entity_decl ::=
    "A" SP+ entity_name SP+ "has:" NEWLINE
    INDENT
    field_decl+
    DEDENT

entity_name ::=
    upper_identifier

field_decl ::=
    field_name SP+ "as" SP+ type_expr initial_clause? NEWLINE

field_name ::=
    lower_identifier

type_expr ::=
      primitive_type
    | "optional" SP+ primitive_type

primitive_type ::=
      "text"
    | "whole" SP+ "number"
    | "decimal" SP+ "number"
    | "yes" SP+ "or" SP+ "no"

initial_clause ::=
    "," SP* "initially" SP+ literal

literal ::=
      text_literal
    | whole_number_literal
    | decimal_number_literal
    | boolean_literal

boolean_literal ::=
      "yes"
    | "no"
```

## 34. Grammar Principle

> **Readable does not mean guessable.**
