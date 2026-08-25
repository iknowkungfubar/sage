# SAGE Roadmap

**Project:** SAGE — Software Abstraction and Generation Environment  
**Status:** Pre-release / experimental

This roadmap describes the intended development sequence for SAGE.

It is directional rather than contractual.

The project should learn from each milestone before committing deeply to the next.

## Roadmap Categories

### Committed

Work that belongs to the current or immediately upcoming milestone.

### Likely

Work that fits the current direction but may change based on implementation experience.

### Exploratory

Ideas worth investigating but not yet accepted as project commitments.

AI agents and contributors must not interpret exploratory items as implementation requirements.

# v0.1 — Language Kernel

**Status:** Committed

Goal:

> Establish a deterministic SAGE frontend with a coherent source model, semantic model, diagnostics system, and CLI.

Canonical source:

```sage
application Inventory

A Product has:
    name as text
    description as optional text
    quantity as whole number, initially 0
    active as yes or no, initially yes
```

## Repository Foundation

- [ ] Rust workspace
- [ ] repository structure
- [ ] `README.md`
- [ ] `AGENTS.md`
- [ ] `CONTRIBUTING.md`
- [ ] `docs/DESIGN.md`
- [ ] `docs/ARCHITECTURE.md`
- [ ] `docs/LANGUAGE.md`
- [ ] `docs/GRAMMAR.md`
- [ ] ADR process
- [ ] CI validation

## Source Infrastructure

- [ ] source-file abstraction
- [ ] UTF-8 handling
- [ ] source IDs
- [ ] byte spans
- [ ] line/column lookup
- [ ] source slicing utilities

## Lexer / Parser

- [ ] application declaration
- [ ] entity declaration
- [ ] field declaration
- [ ] indentation handling
- [ ] text type
- [ ] whole-number type
- [ ] decimal-number type
- [ ] Boolean type
- [ ] optional type syntax
- [ ] initial-value clause
- [ ] text literals
- [ ] integer literals
- [ ] decimal literals
- [ ] Boolean literals
- [ ] parser recovery
- [ ] parser diagnostics

## AST

- [ ] source-level application representation
- [ ] entity declarations
- [ ] field declarations
- [ ] type expressions
- [ ] literals
- [ ] source provenance

## Semantic Analysis

- [ ] application validation
- [ ] duplicate application detection
- [ ] duplicate entity detection
- [ ] duplicate field detection
- [ ] canonical-name normalization
- [ ] name-collision detection
- [ ] type resolution
- [ ] optionality normalization
- [ ] initial-value type checking

## SAGE IR

- [ ] normalized application
- [ ] normalized entities
- [ ] normalized fields
- [ ] semantic types
- [ ] normalized values
- [ ] source provenance where useful

## Diagnostics

- [ ] structured diagnostic type
- [ ] stable diagnostic categories
- [ ] primary source labels
- [ ] secondary source labels
- [ ] notes
- [ ] deterministic help suggestions
- [ ] terminal renderer
- [ ] snapshot tests

## CLI

- [ ] `sage check`
- [ ] `sage explain`
- [ ] non-zero failure exit status
- [ ] human-readable semantic output

## Formatter

- [ ] formatter design
- [ ] canonical indentation
- [ ] deterministic formatting
- [ ] formatter idempotence
- [ ] `sage fmt`

## Testing

- [ ] parser tests
- [ ] compile-fail tests
- [ ] semantic tests
- [ ] IR tests
- [ ] diagnostic snapshots
- [ ] CLI integration tests
- [ ] canonical `inventory.sage`
- [ ] regression-test policy

## v0.1 Exit Criteria

```bash
sage check examples/inventory.sage
sage explain examples/inventory.sage
```

must work, and invalid programs must produce useful source-aware diagnostics instead of crashes.

# v0.2 — Expressions and Rules

**Status:** Likely

Goal:

> Allow SAGE to express derived application meaning.

Potential language areas:

- [ ] numeric expressions
- [ ] text expressions
- [ ] Boolean expressions
- [ ] comparisons
- [ ] `and`
- [ ] `or`
- [ ] `not`
- [ ] field references
- [ ] entity-scoped expressions
- [ ] computed properties
- [ ] declarative rules
- [ ] expression type checking

Example direction:

```sage
A Product is low on stock when:
    quantity is less than 5
```

# v0.3 — Executable Behavior

**Status:** Likely

Potential work:

- [ ] interpreter architecture
- [ ] runtime values
- [ ] local variables
- [ ] conditions
- [ ] actions
- [ ] entity creation
- [ ] entity modification
- [ ] result/error model
- [ ] reusable behavior
- [ ] first-class SAGE tests
- [ ] runtime diagnostics

# v0.4 — Relationships and Collections

**Status:** Likely

Potential work:

- [ ] one-to-one relationships
- [ ] one-to-many relationships
- [ ] ownership
- [ ] collections
- [ ] relationship validation
- [ ] referential semantics
- [ ] collection expressions

# v0.5 — Queries

**Status:** Likely

Potential work:

- [ ] query semantic model
- [ ] filters
- [ ] ordering
- [ ] projection
- [ ] limits
- [ ] aggregation
- [ ] query IR
- [ ] in-memory query backend
- [ ] query explainability

# v0.6 — Persistence

**Status:** Likely

Potential work:

- [ ] persistence semantic layer
- [ ] schema derivation
- [ ] SQLite reference backend
- [ ] schema validation
- [ ] indexes
- [ ] constraints
- [ ] migrations
- [ ] persistent relationships
- [ ] generated query plans
- [ ] `sage explain` persistence output

# v0.7 — Application Runtime

**Status:** Likely

Potential work:

- [ ] runtime host
- [ ] HTTP capability
- [ ] request/response model
- [ ] generated endpoints
- [ ] validation
- [ ] sessions
- [ ] configuration
- [ ] structured logging
- [ ] development server

# v0.8 — UI Model

**Status:** Exploratory

Potential capabilities:

- [ ] pages
- [ ] headings
- [ ] forms
- [ ] tables
- [ ] buttons
- [ ] navigation
- [ ] validation messages
- [ ] generated accessible UI

# v0.9 — Identity and Authorization

**Status:** Exploratory

Potential work:

- [ ] user identity model
- [ ] authentication
- [ ] session security
- [ ] permissions
- [ ] ownership rules
- [ ] authorization policies
- [ ] secure defaults
- [ ] secrets
- [ ] capability manifests

# v0.10 — Events and Automation

**Status:** Exploratory

Potential work:

- [ ] events
- [ ] scheduled jobs
- [ ] background work
- [ ] notifications
- [ ] external APIs
- [ ] email
- [ ] retry semantics
- [ ] idempotency
- [ ] structured concurrency

# v0.11 — Platform and Capability Model

**Status:** Exploratory

Possible platform capabilities:

```text
Persistence
HTTP
UI
Identity
Email
Scheduling
Filesystem
Network
Secrets
```

# v0.12 — Package and Module Model

**Status:** Exploratory

Potential work:

- [ ] modules
- [ ] imports
- [ ] package identity
- [ ] dependency graph
- [ ] lock file
- [ ] reproducible resolution
- [ ] package integrity
- [ ] registry design
- [ ] offline operation

# v1.0 — Stable Core

**Status:** Exploratory

Possible stability requirements include:

- well-defined language specification,
- stable core syntax,
- stable semantic model,
- compatibility policy,
- mature diagnostics,
- deterministic formatter,
- useful application runtime,
- package/reproducibility story,
- security model,
- documented migration policy,
- substantial test suite,
- practical real-world applications built in SAGE.

# Parallel Engineering Tracks

## Developer Experience

- [ ] fast compiler feedback
- [ ] high-quality diagnostics
- [ ] canonical formatter
- [ ] `sage explain`
- [ ] editor protocol support
- [ ] documentation generation

## Compiler Quality

- [ ] fuzz testing
- [ ] parser robustness
- [ ] incremental compilation research
- [ ] benchmark suite
- [ ] memory profiling
- [ ] compiler crash reporting

## Language Specification

- [ ] grammar formalization
- [ ] semantic specification
- [ ] compatibility rules
- [ ] diagnostic catalog
- [ ] conformance tests

## Security

- [ ] untrusted input hardening
- [ ] dependency auditing
- [ ] capability model
- [ ] sandboxing research
- [ ] supply-chain integrity

# Explicitly Not on the Near-Term Roadmap

- operating-system kernels,
- device drivers,
- embedded microcontrollers,
- GPU programming,
- high-performance scientific computing,
- AAA game engines,
- manual memory management,
- arbitrary C ABI replacement,
- unrestricted natural-language programming,
- LLM-required compilation.

# Roadmap Rule

Do not implement a later roadmap item merely because it appears here.

Before moving forward:

1. validate the current milestone,
2. review lessons learned,
3. update the design,
4. choose the smallest useful next vertical slice.

> **Establish meaning before automating machinery around it.**
