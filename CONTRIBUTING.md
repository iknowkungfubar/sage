# Contributing to SAGE

Thank you for contributing to SAGE.

SAGE is an experimental programming language and unified development environment focused on reducing accidental complexity while preserving deterministic semantics, safety, and transparency.

Because language-design decisions can have long-term consequences, contributions should favor correctness and clarity over feature count.

## Read First

Before making substantial changes, read:

1. `README.md`
2. `AGENTS.md`
3. `docs/DESIGN.md`
4. `docs/LANGUAGE.md`
5. `docs/GRAMMAR.md`
6. `docs/ARCHITECTURE.md`

If the change affects a documented architecture decision, also read the relevant files under `docs/adr/`.

## Development Requirements

The repository currently contains a virtual Cargo workspace and six minimal crate skeletons. Compiler behavior and CLI commands remain planned work; the `sage-cli` binary is currently only a placeholder.

The SAGE reference implementation uses Rust.

Use the project's configured toolchain when one exists. Otherwise use the current stable Rust toolchain.

Verify:

```bash
rustc --version
cargo --version
```

## Building

```bash
cargo build --workspace
```

## Running Tests

```bash
cargo test --workspace
```

Focused package tests are encouraged during development, even though the current packages are minimal skeletons and do not yet contain compiler behavior:

```bash
cargo test -p sage-parser
cargo test -p sage-semantic
```

## Formatting

Apply formatting when needed:

```bash
cargo fmt --all
```

Check formatting with the workspace-wide validation form:

```bash
cargo fmt --all -- --check
```

Do not introduce unrelated formatting changes into focused pull requests.

## Linting

```bash
cargo clippy --workspace --all-targets --all-features -- -D warnings
```

## Standard Validation

Before considering a normal code change complete:

```bash
cargo check --workspace
cargo fmt --all -- --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace
```

If a command cannot be run because of an environment limitation, state that clearly.

Do not claim checks passed when they were not executed.

## Governed Verification

Use the repository-defined profiles in `governance/project.json` through:

```bash
python3 scripts/agent/verify.py --config governance/project.json --profile quick
```

The verifier is fail-closed for malformed configuration, unknown gates, unresolved placeholders, and repository-integrity failures. Report the exact command and evidence when a required check cannot run.

## Contribution Philosophy

Before proposing a language feature, ask:

1. What application problem does this solve?
2. Is this essential complexity or accidental complexity?
3. Could SAGE eliminate the concept instead?
4. Can an existing language construct express it?
5. Is the syntax deterministic?
6. Can compiler errors explain misuse clearly?
7. Does it expose backend or infrastructure machinery?
8. Can beginners ignore the feature until they need it?
9. Does it constrain future platforms?
10. Can `sage explain` make the resulting behavior transparent?

A feature should not be added simply because another programming language provides it.

## Current Scope

The current milestone is the language kernel.

Contributions should generally focus on:

- source handling,
- source spans,
- lexer/parser behavior,
- AST,
- names,
- semantic validation,
- primitive types,
- optional values,
- initial values,
- SAGE IR,
- diagnostics,
- formatter,
- CLI,
- tests,
- compiler documentation.

Unless explicitly agreed upon, avoid adding:

- HTTP servers,
- database frameworks,
- authentication,
- frontend frameworks,
- deployment tooling,
- containers,
- cloud infrastructure,
- concurrency systems,
- package registries,
- LLM runtime features.

## Language Changes

Changes to SAGE syntax or semantics require special care.

A language change should normally include:

- parser changes,
- semantic changes if applicable,
- positive tests,
- negative tests,
- diagnostic updates,
- `docs/LANGUAGE.md` updates,
- `docs/GRAMMAR.md` updates,
- relevant example updates.

Significant changes should include an ADR.

## Grammar Changes

If you modify the accepted source grammar:

1. update `docs/GRAMMAR.md`,
2. update parser tests,
3. update formatting behavior if necessary,
4. update diagnostics,
5. verify existing valid examples.

Do not allow parser behavior and documented grammar to drift apart.

## Diagnostics

Diagnostics are part of SAGE's user experience.

Prefer messages that explain:

- what happened,
- where it happened,
- what SAGE expected,
- what SAGE found,
- how to correct it when a deterministic correction exists.

Avoid exposing internal compiler terminology unnecessarily.

Diagnostic snapshots should be reviewed intentionally.

## Source Spans

Preserve accurate source spans.

Do not discard source locations during parsing or semantic lowering merely because a particular implementation step does not currently need them.

## Error Handling

Malformed SAGE programs are expected input.

They should not ordinarily trigger:

```rust
unwrap()
expect()
panic!()
```

Use structured error and diagnostic paths.

Panics should indicate compiler defects, not user mistakes.

## Testing Expectations

Relevant behavior changes should have automated tests.

Use the smallest suitable test layer.

### Parser tests

For grammar and source-structure behavior.

### Semantic tests

For names, types, optionality, defaults, and normalization.

### Compile-fail tests

For invalid programs.

### Diagnostic snapshots

For user-visible diagnostics.

### CLI integration tests

For exit codes, stdout, stderr, and file handling.

### Regression tests

A reproducible bug fix should normally include a test that fails before the fix and passes afterward.

## Canonical Example

The following is the planned canonical inventory source and target behavior; it is not currently compiled by the repository:

```sage
application Inventory

A Product has:
    name as text
    description as optional text
    quantity as whole number, initially 0
    active as yes or no, initially yes
```

The planned CLI interface is:

```bash
sage check examples/inventory.sage
sage explain examples/inventory.sage
```

These are planned interfaces, not commands contributors can claim are currently working. The `sage-cli` binary remains a placeholder.

## Rust Style

Prefer idiomatic, readable Rust.

In general:

- use strong domain types,
- use enums for closed sets,
- keep functions focused,
- keep ownership boundaries clear,
- prefer exhaustive matching where useful,
- avoid unnecessary cloning,
- avoid speculative generic abstractions,
- avoid global mutable state,
- explain invariants rather than obvious code.

Do not introduce `unsafe` without a demonstrated requirement.

## Dependencies

Before adding one, consider maintenance status, ecosystem maturity, license, transitive cost, architectural coupling, deterministic behavior, and whether the problem is difficult to implement correctly.

## Architecture Decision Records

Use ADRs for durable architectural decisions.

Read `docs/adr/README.md` before creating one.

## Pull Requests

Before any GitHub write, run the repository gate:

```bash
turinos gate --repo <path> --intent "..."
```

The operator manually merges pull requests. Direct push and PR commands are not the normal contribution path.

A good pull request should explain:

### What changed

A concise description of the behavior or architecture change.

### Why

The problem being solved.

### Testing

Commands run and relevant tests added.

### Language impact

State whether syntax or semantics changed.

### Documentation

List relevant documentation updates.

### Follow-up work

Only include follow-up items that are directly related and intentionally deferred.

## Avoid Scope Creep

Do not combine unrelated work unless it is inseparable.

## Bug Reports

Useful bug reports should include:

- SAGE version or commit,
- operating system,
- exact source program,
- command executed,
- expected behavior,
- actual behavior,
- diagnostic output.

## Feature Proposals

For significant language features, start with a written proposal before a large implementation.

A proposal should describe:

- problem,
- examples,
- proposed syntax,
- proposed semantics,
- alternatives considered,
- interaction with existing features,
- diagnostics,
- potential future constraints.

## Documentation

Clearly distinguish:

- implemented,
- accepted but not yet implemented,
- exploratory.

Do not describe speculative features as existing functionality.

## Commit Messages

Prefer concise imperative messages such as:

```text
Add optional field parsing
Validate initial value types
Preserve spans during semantic lowering
Improve duplicate field diagnostics
```

## Security

Do not commit credentials, tokens, private keys, secrets, or production configuration.

Treat compiler input as untrusted.

## AI Policy

AI tools may be used to help develop SAGE.

However, SAGE itself must not require AI or an LLM for parsing, compilation, type checking, semantic validation, or execution correctness.

## Code of Engineering Practice

When uncertain, prefer:

- simpler semantics,
- fewer concepts,
- better diagnostics,
- explicit invariants,
- tests,
- documented decisions.

> **Express intent. Hide machinery. Preserve control.**
