# SAGE Language Reference

**Project:** SAGE  
**Meaning:** Software Abstraction and Generation Environment  
**Status:** Experimental / pre-release  
**Document purpose:** Define the currently accepted SAGE language semantics

## 1. Purpose

This document defines the SAGE language as it exists or has been formally accepted for the current implementation milestone.

It answers:

> **What does valid SAGE source mean?**

- `DESIGN.md` describes philosophy and long-term direction.
- `LANGUAGE.md` describes accepted language behavior.
- `GRAMMAR.md` defines formal source structure.
- `ARCHITECTURE.md` describes how the compiler implements the language.

Proposed syntax should not be added here until formally accepted.

## 2. Current Language Version

The initial language kernel is referred to as **SAGE 0.1**.

It currently defines:

- application declarations,
- entity declarations,
- field declarations,
- built-in primitive types,
- explicit optionality,
- initial values,
- deterministic naming,
- basic semantic validation.

It does not yet define executable statements, queries, functions, persistence, UI, networking, or concurrency.

## 3. Source Files

SAGE source files use the extension:

```text
.sage
```

A source file is UTF-8 text.

## 4. Application Declaration

Every SAGE application contains exactly one application declaration.

```sage
application Inventory
```

A complete minimal program is:

```sage
application Inventory
```

Multiple application declarations in one compilation unit are invalid.

## 5. Application Names

Examples:

```sage
application Inventory
application HelpDesk
application CustomerPortal
```

Application names preserve their source spelling.

## 6. Entity Declarations

An entity represents a meaningful application concept.

```sage
A Product has:
    name as text
    quantity as whole number
```

An entity is not defined as a database table, Rust struct, JavaScript object, or class.

## 7. Entity Names

Entity names should begin with an uppercase letter in the current language style.

Examples:

```text
Product
Customer
Order
Invoice
```

Multi-word entity names are not currently part of SAGE 0.1.

## 8. Fields

Fields describe properties of an entity.

```sage
A Product has:
    name as text
    quantity as whole number
```

A field declaration consists of a field name, `as`, a type, and an optional initial-value clause.

## 9. Field Names

Initial SAGE 0.1 field names are simple lower-case identifiers.

Examples:

```sage
name as text
quantity as whole number
active as yes or no
```

Future versions may support multi-word field names.

## 10. Duplicate Fields

An entity may not contain duplicate fields.

```sage
A Product has:
    name as text
    name as optional text
```

is invalid.

## 11. Built-In Types

### `text`

```sage
name as text
```

String literals use double quotes:

```sage
status as text, initially "pending"
```

### `whole number`

```sage
quantity as whole number
quantity as whole number, initially 0
```

### `decimal number`

```sage
rating as decimal number
rating as decimal number, initially 4.5
```

Decimal representation should use deterministic semantics once finalized.

### `yes or no`

```sage
active as yes or no
active as yes or no, initially yes
```

Boolean literals are `yes` and `no`.

## 12. Semantic Types

Future semantic types may include:

```text
email
url
date
date and time
identifier
money
```

These should carry semantic behavior rather than being aliases for primitive strings or numbers.

## 13. Required Fields

Fields are required by default.

```sage
name as text
```

## 14. Optional Fields

Optional values must be explicit.

```sage
description as optional text
```

Conceptually:

```text
Optional<Text>
```

SAGE does not use sentinel values such as `""`, `0`, or `false` to represent absence.

## 15. Optional Type Syntax

```sage
field-name as optional type
```

Nested optionality should be rejected in SAGE 0.1.

## 16. Initial Values

```sage
quantity as whole number, initially 0
active as yes or no, initially yes
status as text, initially "pending"
```

Initial values are statically validated against the field type.

## 17. Initial-Value Type Checking

Valid:

```sage
quantity as whole number, initially 0
```

Invalid:

```sage
quantity as whole number, initially "zero"
```

SAGE does not silently coerce unrelated literal types.

## 18. Optional Fields and Initial Values

An optional field may have an initial value:

```sage
description as optional text, initially "Not provided"
```

## 19. Literals

SAGE 0.1 recognizes text, whole-number, decimal-number, and Boolean literals.

Examples:

```text
"hello"
0
42
3.14
yes
no
```

String escape rules should be documented once finalized.

## 20. Whitespace

Whitespace separates lexical elements.

SAGE uses indentation to represent nested declaration blocks.

## 21. Blank Lines

Blank lines may separate top-level declarations and should not change program semantics.

## 22. Comments

Comments are not part of SAGE 0.1 unless implemented explicitly.

Do not assume a comment syntax until one is formally chosen.

## 23. Naming Normalization

SAGE may normalize user-facing names into canonical identifiers.

Normalization must be deterministic, stable, collision-aware, and preserve original display spelling.

## 24. Case Sensitivity

Case sensitivity is conservative until a naming ADR explicitly specifies otherwise.

## 25. Unknown Types

Unknown types are invalid.

```sage
name as string
```

should produce a specific unknown-type diagnostic if `string` is not recognized.

## 26. Unknown Syntax

The parser must reject unsupported constructs.

SAGE should not guess or silently reinterpret malformed source.

## 27. Determinism

A SAGE program must not rely on LLM interpretation, probabilistic parsing, conversational context, or semantic guessing.

## 28. Diagnostics

Invalid SAGE programs should produce user-oriented diagnostics with source location and actionable help when deterministic.

## 29. Canonical SAGE 0.1 Example

```sage
application Inventory

A Product has:
    name as text
    description as optional text
    quantity as whole number, initially 0
    active as yes or no, initially yes
```

## 30. Current Invalid Examples

Duplicate field:

```sage
A Product has:
    name as text
    name as optional text
```

Wrong initial value:

```sage
A Product has:
    quantity as whole number, initially "zero"
```

Unknown type:

```sage
A Product has:
    name as string
```

Missing entity body:

```sage
A Product has:
```

Invalid indentation:

```sage
A Product has:
name as text
```

## 31. Future Language Areas

Outside the current specification:

- expressions,
- comparisons,
- logical operators,
- rules,
- computed fields,
- relationships,
- functions,
- commands,
- mutation,
- events,
- queries,
- collections,
- persistence,
- tests,
- modules,
- packages,
- imports,
- concurrency,
- capabilities,
- networking,
- UI.

## 32. Compatibility

SAGE is pre-release.

Breaking language changes may occur while the design is being validated.

## 33. Language Design Rule

Before adding syntax, ask:

> Does this represent meaning the programmer needs to express?

If not, consider whether the compiler, runtime, or platform should handle it instead.

## 34. North Star

SAGE should remain readable, precise, safe, explainable, and progressively powerful.

> **Express intent. Hide machinery. Preserve control.**
