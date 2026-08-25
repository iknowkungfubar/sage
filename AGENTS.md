# AGENTS.md

## SAGE — Software Abstraction and Generation Environment

This file defines the standing engineering instructions for AI coding agents working in the SAGE repository.

Read this file before making changes.

SAGE is an experimental programming language and unified development environment whose purpose is to **reduce accidental complexity in software development without sacrificing correctness, transparency, or advanced capability**.

SAGE is deterministic. AI may be used to develop SAGE, but **AI is not required to parse, compile, validate, execute, or deploy SAGE programs**.

SAGE uses a deterministic **controlled natural language**, not unrestricted English.

## 1. Instruction Priority

When instructions conflict, use this priority order:

1. Explicit instructions from the current user/task
2. The nearest applicable `AGENTS.md`
3. This root `AGENTS.md`
4. Project design and architecture documents
5. Existing established code conventions
6. Reasonable engineering judgment

Nested `AGENTS.md` files may refine rules for a particular crate or subsystem.

They should not silently violate project-wide architectural principles defined here.

## 2. Project Mission

SAGE stands for:

> **Software Abstraction and Generation Environment**

The core idea is:

> **The programmer expresses what the software means. SAGE handles as much implementation machinery as reasonably possible.**

Another useful formulation is:

> **Program the application, not the plumbing.**

SAGE should allow developers to describe application concepts such as data, relationships, rules, behavior, workflows, queries, permissions, interfaces, events, integrations, and tests without requiring them to manually assemble unnecessary infrastructure.

The long-term conceptual pipeline is:

```text
Human intent
    ↓
SAGE source
    ↓
Parser
    ↓
Semantic application model
    ↓
Validation
    ↓
SAGE IR
    ↓
Platform / runtime
    ↓
Working software
```

## 3. Primary Design Values

### Reduce accidental complexity

Ask:

> Does this concept represent application meaning, or are we exposing implementation machinery?

If it is primarily machinery, first consider whether SAGE can own it.

### Controlled natural language

Good:

```sage
A Product has:
    name as text
    quantity as whole number
```

Bad:

```text
Products should sort of have a name and probably a quantity.
```

The compiler must never need to guess what a program means.

### Deterministic semantics

Given the same source, compiler version, configuration, and dependencies, SAGE should produce the same interpretation.

Do not introduce LLM inference, probabilistic parsing, or heuristic semantic guessing into the compiler.

### Declarative before imperative

Prefer expressing **what** is wanted.

### Progressive disclosure

Simple problems should require simple concepts.

Advanced capabilities should appear only when needed.

### Safe defaults

The easiest idiomatic SAGE solution should normally also be safe.

### One obvious path

Avoid introducing multiple competing ways to perform basic operations.

### Transparent abstraction

SAGE may hide implementation details. It must not make important behavior unknowable.

### Human-centered diagnostics

Compiler messages are part of the language design.

### Prefer fewer concepts

Prefer deleting a concept over adding syntax for managing that concept.

## 4. Current Development Phase

The current priority is the **SAGE language kernel**.

Unless the task explicitly changes scope, focus on:

- source representation
- source spans
- lexical/syntactic analysis
- AST
- name resolution
- semantic analysis
- type checking
- SAGE IR
- diagnostics
- formatting
- CLI tooling
- tests
- architecture documentation

Canonical initial source:

```sage
application Inventory

A Product has:
    name as text
    description as optional text
    quantity as whole number, initially 0
    active as yes or no, initially yes
```

Initial useful commands:

```bash
sage check <file>
sage explain <file>
```

## 5. Current Non-Goals

Unless explicitly requested, do not expand the current implementation into:

- HTTP servers
- frontend frameworks
- REST frameworks
- database ORMs
- production databases
- authentication
- authorization
- cloud deployment
- Docker orchestration
- Kubernetes
- distributed systems
- package registries
- plugin marketplaces
- graphical IDEs
- editor extensions
- language servers
- arbitrary shell execution
- native optimization pipelines
- concurrency systems
- AI-assisted compiler semantics
- LLM integration

## 6. Technology

The reference implementation is written in **Rust**.

Prefer:

- safe Rust
- explicit types at architectural boundaries
- deterministic behavior
- clear ownership
- readable code
- small focused abstractions
- good error propagation
- comprehensive testing

Avoid `unsafe` unless there is a demonstrated requirement.

## 7. Expected Repository Structure

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

Do not preserve this structure blindly if implementation evidence suggests a simpler arrangement.

## 8. Architecture Boundaries

Maintain separation between:

```text
source syntax
    ↓
parsed representation
    ↓
semantic meaning
    ↓
SAGE IR
    ↓
runtime/backend
```

In particular:

- parser code should not implement runtime behavior,
- AST nodes should not become database models,
- source spelling should not define runtime semantics,
- diagnostics should not depend on raw backend errors,
- SAGE IR should not be a renamed AST,
- semantic analysis should not be performed implicitly inside code generation.

## 9. Source Syntax vs. Semantic Meaning

Source:

```sage
A Product has:
    quantity as whole number, initially 0
```

may lower conceptually to:

```text
Entity {
    name: Product,
    fields: [
        Field {
            name: quantity,
            type: WholeNumber,
            required: true,
            default: Integer(0)
        }
    ]
}
```

Downstream systems should not depend on textual parser productions.

## 10. SAGE IR

The SAGE intermediate representation represents **program meaning**, not syntax.

It should favor explicit semantics, normalized representations, stable internal types, and minimal source-specific trivia.

## 11. Source Spans Are Mandatory

Do not discard source-location information during parsing.

Declarations and semantically meaningful nodes should preserve enough provenance to generate accurate diagnostics.

## 12. Parsing Rules

The SAGE parser is deterministic.

Do not implement unrestricted English parsing.

Do not silently reinterpret malformed source.

If syntax is ambiguous:

1. reject it,
2. produce a useful diagnostic,
3. improve the grammar if needed.

## 13. Language Design Discipline

Do not casually invent new SAGE syntax while implementing unrelated tasks.

For every proposed language feature, consider:

- What user problem does this solve?
- Is this essential or accidental complexity?
- Can an existing construct express it?
- Is the syntax deterministic?
- Can it be explained easily?
- Does it compose with existing syntax?
- Can it produce precise errors?
- Does it constrain future evolution?
- Does it expose implementation details?
- Can a beginner ignore it until needed?

## 14. Initial Type Model

Initial built-in types:

```text
text
whole number
decimal number
yes or no
```

Leave room for semantic types such as:

```text
email
url
date
date and time
identifier
money
```

## 15. Optionality

SAGE has **no implicit null**.

Fields are required unless explicitly optional.

```sage
name as text
description as optional text
```

Do not use sentinel values such as empty strings to represent absence.

## 16. Default Values

Defaults must be validated against field types.

Valid:

```sage
quantity as whole number, initially 0
active as yes or no, initially yes
```

Invalid:

```sage
quantity as whole number, initially "zero"
```

Do not defer obvious static errors to runtime.

## 17. Naming and Normalization

Preserve both human-readable names and canonical internal identifiers when appropriate.

Normalization must be deterministic.

Detect collisions and reject them clearly.

## 18. Diagnostics

Diagnostics are a first-class API.

Prefer structured diagnostics internally.

Potential categories:

```text
SAGE-PARSE-xxx
SAGE-NAME-xxx
SAGE-TYPE-xxx
SAGE-SEMANTIC-xxx
```

Invalid user source should produce a diagnostic, not a panic.

## 19. Error Handling

Expected user mistakes should use normal result/error paths.

Do not use `unwrap()`, `expect()`, or `panic!()` for ordinary malformed SAGE input.

## 20. Rust Code Style

Run:

```bash
cargo fmt
```

and:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

Prefer clear names, small focused functions, typed domain concepts, exhaustive matches where useful, and comments explaining **why**.

Avoid clever macros without substantial benefit, giant functions, broad `String` usage for semantic concepts, premature generic abstractions, speculative traits, excessive cloning, and global mutable state.

## 21. Dependency Policy

Dependencies are allowed when they materially improve correctness or maintainability.

Before adding one, consider maturity, maintenance, API fit, transitive weight, determinism, architecture, and licensing.

## 22. Build Commands

Baseline checks:

```bash
cargo build --workspace
cargo test --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## 23. Required Agent Development Loop

For nontrivial code changes:

1. Inspect relevant code, tests, `AGENTS.md`, architecture docs, and ADRs.
2. Understand subsystem ownership and invariants.
3. Make the smallest coherent change.
4. Run focused tests.
5. Run full relevant validation.
6. Inspect the diff.
7. Report accurately.

Never claim tests passed if they were not run.

## 24. Testing Philosophy

Every behavior change should have appropriate automated coverage.

Use parser tests, compile-fail tests, semantic tests, IR tests, diagnostic snapshot tests, CLI integration tests, and regression tests as appropriate.

## 25. Golden Example

Maintain a canonical example similar to:

```sage
application Inventory

A Product has:
    name as text
    description as optional text
    quantity as whole number, initially 0
    active as yes or no, initially yes
```

## 26. `sage explain`

`explain` represents the rule:

> **SAGE may abstract implementation, but its reasoning should be inspectable.**

Do not turn it into an unstable dump of internal Rust objects.

## 27. Formatting

SAGE should eventually have one canonical formatter.

Formatting should be deterministic, idempotent, and semantics preserving.

## 28. Documentation

Keep documentation synchronized with implemented behavior.

Use:

- `README.md` for project introduction and practical usage
- `DESIGN.md` for product/language design
- `LANGUAGE.md` for accepted semantics
- `GRAMMAR.md` for formal source structure
- `ARCHITECTURE.md` for actual compiler architecture
- ADRs for durable technical decisions

## 29. Architecture Decision Records

Create an ADR for significant choices such as parser strategy, source-span model, IR architecture, name normalization, error recovery, runtime architecture, package model, or concurrency model.

## 30. Comments and TODOs

Comments should explain non-obvious reasoning or invariants.

TODOs should be concrete and scoped.

## 31. Backward Compatibility

During early pre-release development, clean language design is more important than preserving accidental behavior.

However, do not introduce breaking syntax casually.

## 32. Performance

Correctness and architecture currently take precedence over optimization.

Measure before nontrivial optimization work.

## 33. Security

Treat compiler input as untrusted.

Malformed SAGE source should not execute code, execute shell commands, access arbitrary network resources, access unrelated filesystem data, or panic because of ordinary malformed input.

## 34. No AI Runtime Dependency

Do not introduce any requirement that source code be sent to an LLM, that an LLM determine syntax, resolve ambiguity, infer types, generate required runtime code, or make compilation decisions.

## 35. Avoid Premature Framework Building

The intended progression is roughly:

```text
Phase 1
Language kernel

Phase 2
Expressions and behavior

Phase 3
Queries and persistence

Phase 4
Application runtime

Phase 5
Identity and permissions

Phase 6
Events and automation

Phase 7
Production platform
```

Establish meaning before automating infrastructure around that meaning.

## 36. Do Not Recreate Existing Languages Accidentally

Do not assume SAGE needs equivalents of every feature found in Rust, Python, JavaScript, Java, or C++.

Each feature must justify itself in terms of SAGE's programming model.

## 37. Do Not Optimize for Clever Syntax

The goal is **low cognitive load, not minimum keystrokes**.

## 38. Avoid Hidden Magic

When SAGE automatically derives something, the architecture should eventually make it possible to inspect what was derived, why, from which declaration, and what defaults were used.

## 39. Keep User-Facing Concepts Separate From Compiler Concepts

Users should see terms like `Product`, `field`, `whole number`, and `initial value`, not internal terms such as `AstEntityNode` or `LoweringContext`.

## 40. Git Hygiene

Keep changes focused.

Do not mix unrelated feature development, formatting, dependency upgrades, broad refactoring, and documentation rewrites unless genuinely inseparable.

## 41. Commit Messages

Prefer concise imperative messages such as:

```text
Add entity field parsing
Validate field default types
Preserve source spans in semantic model
Improve duplicate-field diagnostics
```

## 42. Before Adding a New Feature

Ask:

1. What problem does this solve?
2. Is it within the current milestone?
3. Is the complexity essential or accidental?
4. Can SAGE make the concept unnecessary?
5. Can an existing feature express it?
6. Is the syntax deterministic?
7. Can errors be explained clearly?
8. Does it compose with existing semantics?
9. Does it leak backend details?
10. Does it preserve progressive disclosure?
11. Does it require a new architectural abstraction?
12. Does it need an ADR?
13. What tests define correct behavior?
14. What should `sage explain` reveal?

## 43. Before Refactoring

Refactor when there is evidence such as duplicated logic, incorrect ownership boundaries, difficult testing, growing conditional complexity, inability to implement the requested feature cleanly, or architecture contradicting documented intent.

## 44. When Requirements Are Underspecified

For local implementation ambiguity:

1. inspect existing conventions,
2. choose the simplest deterministic behavior consistent with SAGE's principles,
3. test it,
4. document meaningful decisions.

## 45. Things Agents Must Not Do

Unless explicitly authorized, do not:

- implement unrelated future phases,
- introduce LLM dependencies,
- silently alter language semantics,
- silently accept ambiguous source,
- use `unwrap()` for malformed user programs,
- discard source spans,
- expose raw Rust debug structures as stable user output,
- add large dependencies for trivial functionality,
- create unnecessary crates,
- create speculative abstraction layers,
- implement multiple competing syntax forms,
- hide failing tests,
- delete tests simply to make CI pass,
- weaken validation to accommodate a test case,
- modify expected diagnostic snapshots without reviewing the change,
- claim commands were run when they were not,
- describe unfinished functionality as complete.

## 46. Definition of Done

For an ordinary compiler feature or bug fix:

- implementation is coherent with architecture,
- new behavior has tests,
- regressions have regression tests,
- diagnostics remain human-readable,
- source spans remain correct,
- documentation is updated when behavior or architecture changed,
- formatting passes,
- Clippy passes,
- relevant tests pass,
- full workspace tests pass when practical,
- no unrelated changes remain,
- no temporary debugging code remains.

## 47. Agent Completion Report

At the end of a meaningful task, report:

### Changed

What was implemented or modified.

### Validation

Exact relevant commands executed and whether they passed.

### Design Decisions

Only meaningful decisions that affect future work.

### Remaining Issues

Known limitations, failing checks, or intentionally deferred work.

## 48. Guiding Test for Every Decision

Ask:

> **Does this help the developer express application meaning, or does it merely make them manage more machinery?**

Ask:

> **Can SAGE explain exactly what it did and why?**

Ask:

> **Can we remove a concept instead?**

## 49. Project North Star

SAGE should eventually make code like this unsurprising:

```sage
application Inventory

A Product has:
    name as text
    quantity as whole number, initially 0

A Product is low on stock when:
    quantity is less than 5
```

while the platform safely handles increasing amounts of implementation machinery beneath it.

## 50. Final Principle

When choosing between making the SAGE compiler more sophisticated or making every SAGE programmer repeatedly manage the same complexity, prefer putting that sophistication into SAGE when it can be done safely, deterministically, transparently, and generally.

> **Express intent. Hide machinery. Preserve control.**
