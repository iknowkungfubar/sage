# SAGE Architecture

**Project:** SAGE  
**Meaning:** Software Abstraction and Generation Environment  
**Status:** Target architecture plus current repository scaffolding
**Implementation language:** Rust  
**Document purpose:** Describe the target technical architecture of the SAGE compiler and tooling, and distinguish it from the current scaffold

## 1. Scope

This document defines the target architecture for SAGE and records how the current repository scaffolding is organized. It does not describe an end-to-end implementation: the repository currently contains a virtual Cargo workspace and six minimal crate skeletons, while compiler stages and CLI commands remain planned work.

The current development phase is the **language kernel**.

The target architecture is intentionally designed to support future expansion without prematurely implementing higher-level systems.

## 2. Architectural Overview

The following is the **planned target pipeline**, not an active end-to-end pipeline in the current scaffold. Its stages and boundaries guide future implementation.

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

The current repository has the following structure. The six crates are minimal skeletons; their target ownership boundaries are described below and do not imply implemented compiler behavior.

```text
sage/
├── .gitignore
├── AGENTS.md
├── Cargo.lock
├── Cargo.toml
├── CONTRIBUTING.md
├── LICENSE
├── README.md
├── ROADMAP.md
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

The initial source abstraction lives in `sage-syntax` and owns an explicit opaque source identity,
a source name, and exact source text. It is an architectural starting point, not the complete source
model.

```rust
pub struct SourceFile {
    id: SourceId,
    name: String,
    text: String,
}
```

`SourceId` is a stable handle supplied externally by the source owner or registry. Allocation and
registry policy are intentionally not implemented in `sage-syntax`; IDs remain stable for the
source-file lifetime and are not file names or offsets. `SourceFile` stores valid UTF-8 source text
in a Rust `String`. Its `from_utf8` byte-input constructor validates with the standard library and
rejects invalid UTF-8; no replacement or normalization occurs, so valid input is preserved exactly.
The implemented `Span` value type links a `SourceId` to a half-open byte range over those original
UTF-8 bytes. Future source infrastructure must preserve the original text so diagnostics can
recover source identity, file, and text information without changing its contents.

`sage-syntax` also provides the public `LineColumn` value type and
`SourceFile::line_column(offset)`. The lookup accepts offsets from zero through the source's byte
length, but only at UTF-8 scalar boundaries; invalid or out-of-range offsets return `None`. Its
line and column values are one-based, with columns counting Unicode scalar values rather than
bytes. Empty files and EOF at the end of a file map to the current position, initially line 1,
column 1.

LF (`\\n`), CRLF (`\\r\\n`), and bare CR (`\\r`) each count as one newline. Newline bytes belong to
the preceding line: offsets at any byte of a newline remain at the preceding line and its current
column, while an offset after the complete newline sequence starts the next line at column 1.
The implementation scans the exact source text linearly for each lookup and does not normalize the
text or add source slicing, parser integration, or caching.

## 7. Source Spans

Source spans are mandatory. `sage-syntax` provides `Span` as an immutable value type containing a
`SourceId` and a half-open byte range `[start, end)` over the original UTF-8 bytes. Its offsets are
`u32` values; empty spans are allowed, and `Span::new` rejects reversed ranges where `end < start`.

Checking whether a span's offsets are within a particular `SourceFile`, converting spans to
line/column positions, and source slicing remain separate concerns. `Span` does not perform
automatic source lookup. A precomputed line map or optimized lookup, and integration from spans
into line/column diagnostics, remain future concerns.

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

The target compiler should represent diagnostics structurally. The current crates do not yet implement this pipeline.

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

## 26. CLI Architecture (Planned)

The planned CLI should parse arguments, load source, invoke compiler APIs, render diagnostics or results, and return meaningful exit status.

The current `sage-cli` is a placeholder and does not implement these commands. The CLI should not contain semantic compiler logic.

## 27. Planned `sage check`

The following is the conceptual target flow; `sage check` is not implemented in the current scaffold.

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

## 28. Planned `sage explain`

`explain` is intended to render semantic meaning in a human-oriented form. It is not implemented in the current scaffold.

Do not simply print raw Rust debug structures.

## 29. Planned Formatter Architecture

The future formatter should be deterministic, idempotent, and semantics preserving. No formatter is implemented in the current scaffold.

## 30. Planned Compiler API

The compiler should eventually expose a reusable library interface independent of the CLI. No compiler API or pipeline is implemented in the current scaffold.

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

These commands validate the current Rust workspace scaffold and remain the workspace-wide checks as implementation proceeds:

```bash
cargo check --workspace
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

## 49. Target Architecture Definition of Done

The target language-kernel architecture is complete when SAGE can load source, preserve spans, parse applications/entities/fields, resolve primitive types, represent optional fields, validate initial values, detect duplicates, lower to normalized IR, produce structured diagnostics, support `sage check`, support `sage explain`, and pass relevant tests.

The current repository scaffold does not meet this definition of done yet. It provides the workspace, six crate skeletons, and documentation; the compiler pipeline, semantic behavior, formatter, compiler API, and CLI commands remain to be implemented.

## 50. Architectural North Star

The desired relationship is:

```text
more sophistication inside SAGE
                ↓
less repeated machinery in applications
```

> **Express intent. Hide machinery. Preserve control.**
