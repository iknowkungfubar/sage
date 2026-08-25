# SAGE

**Software Abstraction and Generation Environment**

SAGE is an experimental programming language and unified development environment designed to reduce accidental complexity in software development.

Its core idea is simple:

> **Express what the software means. Let SAGE handle as much of the machinery as possible.**

SAGE uses a deterministic, controlled natural-language style intended to make programs easier to read and write without depending on AI or LLMs for parsing, compilation, or execution.

## Status

> **SAGE is in very early development and is not ready for production use.**

The current milestone is the **language kernel**: establishing the parser, semantic model, type-system foundations, intermediate representation, diagnostics, formatter, and command-line tooling. The repository currently contains the Rust workspace and six crate skeletons; implementation of the compiler pipeline and CLI commands is planned work.

The project is intentionally avoiding premature work on web frameworks, databases, deployment, authentication, and other higher-level systems until the core language semantics are stable.

## Why SAGE?

Modern application development often requires developers to understand and configure far more than the application itself.

A relatively ordinary application may involve:

- a programming language,
- a frontend framework,
- a backend framework,
- HTTP routing,
- JSON serialization,
- authentication,
- authorization,
- database drivers,
- SQL,
- ORM configuration,
- schema migrations,
- asynchronous programming,
- dependency management,
- testing frameworks,
- container configuration,
- deployment tooling,
- observability,
- infrastructure configuration.

Many of these mechanisms are necessary somewhere.

The question SAGE asks is:

> **Why should every application developer have to manage all of them manually?**

SAGE aims to move reusable implementation complexity into the programming system itself.

## What SAGE Looks Like

A simple SAGE application might look like this:

```sage
application Inventory

A Product has:
    name as text
    description as optional text
    quantity as whole number, initially 0
    active as yes or no, initially yes
```

The goal is for the source code to describe application concepts directly:

- `Product` is an application entity.
- `name` is required text.
- `description` may be absent.
- `quantity` is a whole number with an initial value of `0`.
- `active` is a Boolean value initially set to `yes`.

The programmer should not need to specify implementation details that SAGE can safely derive.

## Design Philosophy

SAGE is guided by several principles.

### Reduce accidental complexity

Application developers should primarily work with application concepts rather than infrastructure machinery.

### Controlled natural language

SAGE should be readable like structured English, but it is **not unrestricted English**.

The grammar is deterministic.

The compiler must never guess what the programmer means.

### Declarative first

Prefer describing desired results over manually specifying every implementation step.

### Safe defaults

The easiest idiomatic solution should generally also be a safe solution.

### Progressive disclosure

Simple programs should require only simple concepts.

Advanced functionality should appear when it is actually needed.

### Transparent abstraction

SAGE may hide implementation details, but important decisions must remain inspectable.

### Human-centered diagnostics

Compiler errors should explain the problem in terms that application developers can understand.

### One obvious path

Common operations should have one strongly preferred SAGE representation rather than several competing approaches.

### No AI dependency

AI may help develop SAGE or someday assist SAGE developers, but AI is not part of the language's correctness model.

SAGE compilation and execution must remain deterministic and possible without an LLM.

## Essential vs. Accidental Complexity

SAGE does not attempt to pretend that software has no inherent complexity.

A financial application genuinely needs concepts such as accounts, transactions, balances, permissions, and reconciliation. Those are examples of **essential complexity** because they belong to the problem being solved.

SAGE instead targets **accidental complexity** such as:

- repetitive serialization code,
- ORM setup,
- route registration,
- dependency-injection configuration,
- schema-migration boilerplate,
- manual asynchronous plumbing,
- repetitive validation code,
- build configuration.

The goal is to eliminate or automate those mechanisms when doing so is safe and predictable.

## Target Language Scope

The initial language subset is planned to cover the SAGE language frontend.

The planned initial language subset includes:

- application declarations,
- entities,
- fields,
- primitive types,
- optional fields,
- initial values,
- name validation,
- type validation,
- normalized semantic representation,
- structured diagnostics.

The planned initial built-in types include:

```text
text
whole number
decimal number
yes or no
```

Future semantic types are expected to include concepts such as:

```text
email
url
date
date and time
identifier
money
```

## CLI Roadmap

The `sage-cli` binary currently is only a placeholder. It does not yet implement `sage check`, `sage explain`, or any other SAGE command; later CLI roadmap issues will implement them.

The following is the planned target interface, not currently working functionality:

```bash
sage check <file>
sage explain <file>
sage fmt
sage run
sage test
sage build
sage deploy
```

### Planned `sage check`

Validate a SAGE source file.

```bash
sage check examples/inventory.sage
```

The intended behavior is a non-zero exit status and a human-readable diagnostic for invalid source.

### Planned `sage explain`

Inspect SAGE's normalized understanding of a program.

```bash
sage explain examples/inventory.sage
```

Possible output:

```text
Application Inventory

Entity Product
    name: Text
    description: Optional<Text>
    quantity: WholeNumber = 0
    active: Boolean = true
```

`explain` is an important part of SAGE's design philosophy:

> **Abstraction should reduce complexity without making behavior unknowable.**

## Architecture

The planned SAGE frontend is designed around a compiler pipeline similar to:

```text
SAGE source
    │
    ▼
Source representation
and source spans
    │
    ▼
Lexer / parser
    │
    ▼
Syntax / AST
    │
    ▼
Name resolution
Semantic analysis
Type validation
    │
    ▼
Normalized SAGE IR
    │
    ├──────────────► planned `sage explain`
    │
    ▼
Future runtimes
and backends
```

The source language is intentionally kept independent from any specific backend.

See:

- [`docs/DESIGN.md`](docs/DESIGN.md)
- [`docs/LANGUAGE.md`](docs/LANGUAGE.md)
- [`docs/GRAMMAR.md`](docs/GRAMMAR.md)
- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- [`AGENTS.md`](AGENTS.md)
- [`CONTRIBUTING.md`](CONTRIBUTING.md)
- [`ROADMAP.md`](ROADMAP.md)

## Repository Structure

```text
sage/
├── AGENTS.md
├── CONTRIBUTING.md
├── ROADMAP.md
├── README.md
├── Cargo.toml
├── docs/
│   ├── DESIGN.md
│   ├── LANGUAGE.md
│   ├── GRAMMAR.md
│   ├── ARCHITECTURE.md
│   └── adr/
│       └── README.md
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

## Current Workspace Validation

SAGE's reference implementation is written in Rust. The current repository can be validated through these Rust workspace commands.

Check the workspace:

```bash
cargo check --workspace
```

Build the workspace:

```bash
cargo build --workspace
```

Run the test suite:

```bash
cargo test --workspace
```

Check formatting:

```bash
cargo fmt --all -- --check
```

Run Clippy:

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

The `sage-cli` binary is currently only a placeholder. The `sage check` and `sage explain` examples below describe the planned CLI interface and do not work yet.

## Example

Create `examples/inventory.sage`:

```sage
application Inventory

A Product has:
    name as text
    description as optional text
    quantity as whole number, initially 0
    active as yes or no, initially yes
```

The planned CLI usage is:

```bash
sage check examples/inventory.sage
sage explain examples/inventory.sage
```

These commands are not implemented yet.

## Diagnostics

Diagnostics are treated as part of SAGE's user-facing design.

SAGE should aim for diagnostics similar to:

```text
error[SAGE-TYPE-001]: initial value has the wrong type

  --> examples/inventory.sage:6:41
   |
 6 |     quantity as whole number, initially "zero"
   |                                         ^^^^^^
   |
   = expected: whole number
   = found: text
   = help: use a numeric value such as `0`
```

Malformed user programs should produce diagnostics, not compiler panics.

## No Implicit Null

Fields are required unless explicitly optional.

Required:

```sage
name as text
```

Optional:

```sage
description as optional text
```

Missing values should remain explicit in the type system.

## What SAGE Is Not

SAGE is not:

- a natural-language chatbot for generating code,
- an LLM-based compiler,
- a traditional no-code platform,
- unrestricted English programming,
- a wrapper around a particular web framework,
- a replacement for understanding the application's actual domain,
- an attempt to hide every implementation detail forever.

Its goal is:

> **Developers should not need to repeatedly understand and manually manage machinery that the programming system can safely own.**

## Long-Term Direction

SAGE is initially focused on:

> **business applications, internal tools, automation, APIs, and data-driven software.**

A possible future SAGE application might look like:

```sage
application Inventory

A Product has:
    name as text
    quantity as whole number, initially 0

A Product is low on stock when:
    quantity is less than 5
```

Future language capabilities may include relationships, expressions, rules, reusable behavior, declarative queries, persistence, tests, events, scheduled jobs, HTTP services, application pages, forms, identity, authorization, external integrations, and structured concurrency.

These features are intentionally being developed incrementally.

## Contributing

Before contributing, read:

1. [`AGENTS.md`](AGENTS.md)
2. [`CONTRIBUTING.md`](CONTRIBUTING.md)
3. [`docs/DESIGN.md`](docs/DESIGN.md)
4. [`docs/LANGUAGE.md`](docs/LANGUAGE.md)
5. [`docs/GRAMMAR.md`](docs/GRAMMAR.md)
6. [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)

## Guiding Questions

When deciding whether something belongs in SAGE:

> **Does this help the programmer express application meaning, or does it merely make them manage more machinery?**

When deciding whether an abstraction hides too much:

> **Can SAGE explain what it did and why?**

When considering another language feature:

> **Could we remove a concept instead?**

## Project North Star

SAGE succeeds if increasingly sophisticated applications can be expressed while requiring developers to personally manage fewer unrelated implementation mechanisms.

The implementation beneath SAGE may become complex.

That complexity should live inside the programming system when it can be handled safely, deterministically, generally, and transparently.

> **Express intent. Hide machinery. Preserve control.**
