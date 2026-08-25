# SAGE Architecture Decision Records

This directory contains Architecture Decision Records (ADRs) for SAGE.

ADRs document important technical decisions that have long-term consequences for the language, compiler, runtime, tooling, or project architecture.

They exist so future contributors can understand not only **what decision was made**, but also **why it was made**.

Each ADR records one durable architectural, language, or tooling decision. It is not a feature checklist, implementation plan, or speculative placeholder.

## When to Create an ADR

Create an ADR when a decision is:

- architecturally significant,
- difficult or expensive to reverse,
- likely to affect multiple subsystems,
- likely to constrain future development,
- important enough that future contributors may otherwise revisit it without context.

Examples include:

- reference implementation language,
- parser architecture,
- source-span representation,
- identifier normalization,
- decimal-number semantics,
- SAGE IR design,
- runtime architecture,
- persistence abstraction,
- package identity,
- concurrency model,
- capability/security model.

## When Not to Create an ADR

Do not create ADRs for routine local implementation details such as renaming a private function, splitting a large module, adding a small helper type, changing local variable names, adding a test fixture, or fixing a parser bug without changing grammar.

## File Naming

Use sequential numeric prefixes.

```text
0001-rust-reference-implementation.md
0002-source-span-representation.md
0003-parser-strategy.md
0004-name-normalization.md
```

Use lowercase kebab-case after the number.

Do not renumber accepted ADRs.

## ADR Status

Use one of:

- **Proposed**
- **Accepted**
- **Rejected**
- **Superseded**

When superseding an ADR, reference the replacement ADR.

## ADR Template

```markdown
# ADR-NNNN: Decision Title

**Status:** Proposed
**Date:** YYYY-MM-DD

## Context

Describe the problem and why a decision is required.

## Decision

State the chosen approach clearly.

## Consequences

Describe meaningful benefits, costs, limitations, and future implications.

## Alternatives Considered

Describe serious alternatives and why they were not selected.

## References

Link the motivating issue and the review or implementation pull request, along with relevant design documents, specifications, or research.
```

## ADR Workflow

For a significant architectural change:

1. identify the single decision and its motivating issue,
2. create a proposed ADR and link the issue in `References`,
3. discuss or review the consequences and link the review or implementation PR,
4. propose the ADR before implementation when practical,
5. implement only after the direction is accepted when practical,
6. update the ADR status after the decision,
7. keep the ADR permanently in history.

Rejected and superseded ADRs remain in the repository so the decision history is preserved.

## Relationship to Other Documentation

- `docs/DESIGN.md` explains product and language philosophy.
- `docs/LANGUAGE.md` defines accepted language semantics.
- `docs/GRAMMAR.md` defines accepted source grammar.
- `docs/ARCHITECTURE.md` describes the target architecture and the current repository scaffolding/status.
- ADRs explain why important architectural choices were made.

If an accepted ADR changes architecture or language behavior, update the appropriate primary documentation as well.

## Decision Authority

An accepted ADR should represent an intentional project decision.

Do not use ADRs to silently introduce major language changes.

Language syntax or semantic decisions should also update `docs/LANGUAGE.md` and `docs/GRAMMAR.md` where applicable.

## Superseding Decisions

Do not rewrite old ADRs to make history appear cleaner.

Instead:

1. create a new ADR,
2. explain the new context,
3. mark the old ADR as superseded,
4. link the two.

## Initial ADR Candidates

This repository may have no ADR files yet. Candidate topics are not ADRs until the underlying decision has actually been made; do not create placeholder records.

Likely early ADR topics include:

```text
0001-rust-reference-implementation.md
0002-source-span-representation.md
0003-parser-strategy.md
0004-name-normalization.md
0005-decimal-number-representation.md
```

Create them only when the underlying decisions have actually been made.

## Guiding Principle

An ADR should answer the question a future maintainer is likely to ask:

> **Why did they build it this way?**
