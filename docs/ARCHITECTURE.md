# SAGE Architecture

**Project:** SAGE  
**Meaning:** Software Abstraction and Generation Environment  
**Status:** Initial compiler architecture  
**Implementation language:** Rust  
**Document purpose:** Describe the implemented and intended technical architecture of the SAGE compiler and tooling

## 1. Scope

This document describes the architecture of the current SAGE implementation.

The current development phase is the **language kernel**.

The architecture is intentionally designed to support future expansion without prematurely implementing higher-level systems.

## 2. Architectural Overview

```text
┌─────────────────────┐
│     SAGE Source     │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Source Representation│
│   + Source Spans     │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Lexer / Parser      │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Syntax / AST        │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Name Resolution     │
│ Semantic Analysis   │
│ Type Validation     │
└──────────┬──────────┘
           │
           ▼
┌─────────────────────┐
│ Normalized SAGE IR  │
└──────────┬──────────┘
           │
           ├───────────────┐
           ▼               ▼
┌─────────────────┐ ┌─────────────────┐
│ `sage explain`  │ │ Future Backends │
└─────────────────┘ └─────────────────┘
```

Diagnostics may be emitted from multiple stages.

The CLI coordinates the pipeline but should not own compiler semantics.

## 3. Architectural Principles

### Syntax is not semantics

The parser determines how source is structured. Semantic analysis determines what the source means.

### The IR is not the AST

The AST represents source structure. The SAGE IR represents normalized application meaning.

### Diagnostics are architecture, not decoration

Every compiler stage must preserve sufficient context for useful diagnostics.

### Compiler stages should have clear ownership

Avoid architectures where parser code performs type checking, semantic analysis performs formatting, the CLI resolves names, or the IR directly generates terminal diagnostics.

### Backend independence

The language frontend should not depend on a web framework, SQL, JavaScript, Rust code generation, or a particular runtime.

## 4. Repository Structure

```text
sage/
├── AGENTS.md
├── Cargo.toml
├── README.md
├── LICENSE
├── docs/
│   ├── DESIGN.md
│   ├── ARCHITECTURE.md
│   └── adr/
├── crates/
│   ├── sage-cli/
│   ├── sage-syntax/
│   ├── sage-parser/
│   ├── sage-semantic/
│   ├── sage-ir/
│   └── sage-diagnostics/
├── examples/
└── tests/
```

Crates should represent meaningful dependency or ownership boundaries.

## 5. Workspace Dependency Direction

Avoid cyclic ownership.

For example:

- `sage-ir` should not depend on `sage-cli`,
- `sage-parser` should not depend on the runtime,
- `sage-semantic` should not depend on terminal rendering.

## 6. Source Representation

The compiler requires a stable source abstraction.

Conceptually:

```rust
struct SourceFile {
    id: SourceId,
    name: String,
    text: String,
}
```

Important requirements:

- stable source identity,
- offsets refer back to original source,
- diagnostics can recover file and text information,
- UTF-8 works correctly.

## 7. Source Spans

Source spans are mandatory.

Conceptually:

```rust
struct Span {
    source: SourceId,
    start: u32,
    end: u32,
}
```

Offsets should have clearly documented semantics.

Half-open ranges `[start, end)` are recommended unless an ADR selects otherwise.

## 8. Spanned Values

A reusable abstraction may be appropriate:

```rust
struct Spanned<T> {
    value: T,
    span: Span,
}
```

Use spans where they support diagnostics, source navigation, explainability, and later tooling.

## 9. Lexing Strategy

SAGE may use a distinct lexer, parser-integrated tokenization, or another deterministic strategy.

Lexing responsibilities may include identifiers, keywords, punctuation, string literals, numeric literals, indentation, and line breaks.

## 10. Indentation

SAGE uses indentation where it improves readability and structural clarity.

The parser must define indentation behavior deterministically.

## 11. Parser Responsibilities

The parser should recognize grammar structure, preserve source spans, and recover safely where appropriate.

It should not execute user code or perform backend work.

## 12. Parse Recovery

A safe strategy is:

1. identify the malformed construct,
2. emit a diagnostic,
3. synchronize at a known structural boundary,
4. resume parsing.

Recovery must never silently alter program meaning.

## 13. Abstract Syntax Tree

A conceptual initial model:

```rust
struct Application {
    name: Name,
    entities: Vec<EntityDecl>,
    span: Span,
}

struct EntityDecl {
    name: Name,
    fields: Vec<FieldDecl>,
    span: Span,
}

struct FieldDecl {
    name: Name,
    type_expr: TypeExpr,
    initial_value: Option<Literal>,
    span: Span,
}
```

## 14. Syntax Types

Initial syntax-level type expressions include:

```text
text
whole number
decimal number
yes or no
optional <type>
```

## 15. Literals

Initial literals include text strings, whole numbers, decimal numbers, and yes/no booleans.

Decimal representation requires an intentional decision before semantics are considered stable.

## 16. Names

Source-level names should retain display spelling and source span.

Name normalization belongs in semantic analysis or a shared name component.

## 17. Name Resolution

Initial responsibilities include duplicate applications, duplicate entities, duplicate fields, canonical-name collisions, and future reference resolution.

## 18. Semantic Analysis

Initial responsibilities include application validation, name normalization, duplicate detection, type resolution, optionality, initial-value validation, and construction of normalized semantic objects.

## 19. Type Representation

Conceptually:

```rust
enum Type {
    Text,
    WholeNumber,
    DecimalNumber,
    Boolean,
    Optional(Box<Type>),
}
```

Future semantic types may include `Email`, `Url`, `Date`, `DateTime`, `Identifier`, and `Money`.

## 20. Optionality Representation

Optionality must remain explicit.

The compiler must never represent absence using empty text, zero, false, or another sentinel value.

## 21. Initial-Value Validation

Semantic analysis validates initial values and produces typed diagnostics for mismatches.

## 22. SAGE Intermediate Representation

Conceptually:

```rust
struct IrApplication {
    name: ApplicationName,
    entities: Vec<IrEntity>,
}

struct IrEntity {
    name: EntityName,
    fields: Vec<IrField>,
}

struct IrField {
    name: FieldName,
    ty: Type,
    initial_value: Option<Value>,
}
```

The IR should contain normalized meaning rather than parser-specific forms.

## 23. Stable IDs

As the language grows, declarations should likely use stable internal IDs when reference resolution or cross-linking makes them useful.

## 24. Diagnostics Architecture

Diagnostics should be represented structurally.

Conceptually:

```rust
struct Diagnostic {
    code: DiagnosticCode,
    severity: Severity,
    message: String,
    primary: Label,
    secondary: Vec<Label>,
    notes: Vec<String>,
    help: Vec<String>,
}
```

Terminal rendering belongs in a presentation layer.

## 25. Diagnostic Codes

Potential categories:

```text
SAGE-PARSE-001
SAGE-NAME-001
SAGE-TYPE-001
SAGE-SEMANTIC-001
```

## 26. CLI Architecture

The CLI should parse arguments, load source, invoke compiler APIs, render diagnostics or results, and return meaningful exit status.

The CLI should not contain semantic compiler logic.

## 27. `sage check`

Conceptual flow:

```text
CLI
 ↓
load source
 ↓
parse
 ↓
semantic analysis
 ↓
IR validation
 ↓
success or diagnostics
```

## 28. `sage explain`

`explain` renders semantic meaning in a human-oriented form.

Do not simply print raw Rust debug structures.

## 29. Formatter Architecture

The formatter should be deterministic, idempotent, and semantics preserving.

## 30. Compiler API

The compiler should eventually expose a reusable library interface independent of the CLI.

## 31. Error Handling

Malformed user source is expected input and must not ordinarily cause `panic!()`, `unwrap()`, or `expect()`.

## 32. `unsafe` Policy

The language kernel should normally require no unsafe Rust.

## 33. Dependency Policy

Dependencies should be evaluated for maturity, maintenance, license compatibility, API stability, dependency weight, architectural coupling, and deterministic behavior.

## 34. Testing Architecture

Use parser unit tests, parser error tests, semantic tests, IR tests, diagnostic snapshot tests, CLI integration tests, and regression tests.

## 35. Canonical Test Program

```sage
application Inventory

A Product has:
    name as text
    description as optional text
    quantity as whole number, initially 0
    active as yes or no, initially yes
```

## 36. Build and Validation

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## 37. Architecture Decision Records

Significant technical decisions should be recorded under `docs/adr/`.

## 38. Future Runtime Boundary

```text
SAGE Source
    ↓
Frontend Compiler
    ↓
SAGE IR
    ↓
Platform Interface
    ↓
Runtime / Backend
```

The runtime does not belong inside the language frontend.

## 39. Future Platform Interface

Possible capabilities include persistence, HTTP, UI, identity, email, scheduling, filesystem, network, and secrets.

Do not create speculative platform interfaces until concrete use cases justify them.

## 40. Future Persistence Architecture

Entity semantics must remain independent from SQL concepts.

## 41. Future Query Architecture

Declarative queries should first normalize into a backend-independent query representation.

## 42. Future Execution Architecture

The first behavior implementation should favor semantic clarity and debugging over maximum performance.

## 43. Future Concurrency Architecture

Concurrency should eventually be structured.

Do not design it before synchronous behavior semantics are stable.

## 44. Future Package Architecture

Package support is intentionally deferred.

## 45. Future Tooling Architecture

The compiler library should eventually support CLI, formatter, language server, IDE, documentation generator, static analysis, and build tools.

## 46. Observability of Compiler Decisions

As SAGE gains automation, compiler decisions should retain provenance.

## 47. Stability Boundaries

During early development, internal Rust APIs are unstable.

Public language semantics deserve greater care.

## 48. Architectural Anti-Patterns

Avoid:

- parser-driven runtime,
- stringly typed semantic model,
- CLI-owned semantics,
- AST-as-IR,
- backend leakage,
- diagnostic panics,
- premature generalization,
- premature performance optimization.

## 49. Current Architecture Definition of Done

The initial compiler architecture is established when SAGE can load source, preserve spans, parse applications/entities/fields, resolve primitive types, represent optional fields, validate initial values, detect duplicates, lower to normalized IR, produce structured diagnostics, support `sage check`, support `sage explain`, and pass relevant tests.

## 50. Architectural North Star

The desired relationship is:

```text
more sophistication inside SAGE
                ↓
less repeated machinery in applications
```

> **Express intent. Hide machinery. Preserve control.**
