# SAGE Design

**Project:** SAGE  
**Meaning:** Software Abstraction and Generation Environment  
**Status:** Experimental / pre-release; repository scaffolded, compiler behavior not yet implemented
**Document purpose:** Product and language design principles

This document describes SAGE's design targets and roadmap. The repository currently contains the Rust workspace and crate scaffolding; the language kernel is an implementation target, not an implemented compiler. Examples that describe language behavior are target, possible, or future behavior unless explicitly stated otherwise.

## 1. Overview

SAGE is a deterministic programming language and unified application-development environment designed to reduce accidental complexity in software development.

Its central idea is:

> **The programmer expresses what the software means. SAGE handles as much implementation machinery as reasonably possible.**

SAGE is intended to eventually unify capabilities that are commonly spread across programming languages, frameworks, package managers, database tooling, validation libraries, API frameworks, UI frameworks, testing frameworks, deployment tooling, observability tooling, security configuration, and infrastructure glue.

The objective is not to pretend that software is inherently simple.

The objective is to prevent every application developer from repeatedly managing complexity that can instead be handled safely and consistently by the programming system.

SAGE does **not** require AI or an LLM to understand or execute programs.

Its language is deterministic and formally parseable.

## 2. Problem Statement

Modern application development often requires developers to manage large amounts of complexity unrelated to the application itself.

A relatively ordinary application may require knowledge of language syntax, runtime behavior, package managers, dependency resolution, web frameworks, frontend frameworks, HTTP, JSON, authentication, authorization, database drivers, SQL, ORM systems, migrations, configuration systems, asynchronous programming, testing frameworks, build tools, containers, deployment systems, cloud infrastructure, monitoring, logging, and security practices.

Many of these concepts are important internally.

They do not necessarily need to be exposed directly to every application developer.

SAGE attempts to move appropriate complexity from every individual application into the language, compiler, runtime, standard platform, and tooling.

## 3. Essential vs. Accidental Complexity

### Essential complexity

Essential complexity comes from the problem domain itself.

For example, a financial application may genuinely need concepts such as accounts, transactions, balances, reconciliation, permissions, and audit history.

SAGE cannot and should not hide these concepts.

### Accidental complexity

Accidental complexity comes from implementation mechanisms rather than the application domain.

Examples include serialization boilerplate, ORM setup, route registration, dependency injection configuration, thread management, asynchronous callback machinery, schema-migration boilerplate, bundler configuration, container manifests, and repetitive validation code.

SAGE should remove, generate, derive, or safely automate these mechanisms whenever doing so is practical and understandable.

## 4. Project Thesis

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
SAGE intermediate representation
    ↓
Platform / runtime
    ↓
Working software
```

The programmer should primarily express data, relationships, constraints, rules, calculations, behavior, events, queries, permissions, interactions, workflows, and tests.

## 5. Controlled Natural Language

SAGE uses a **controlled natural-language style**.

It should resemble structured English while remaining deterministic and formally specified.

A target SAGE application might look like this (the compiler does not yet implement it):

```sage
application Inventory

A Product has:
    name as text
    description as optional text
    quantity as whole number, initially 0
    active as yes or no, initially yes
```

SAGE is not intended to parse arbitrary English.

The compiler must never require probabilistic guessing about program meaning.

## 6. Design Goals

### Low cognitive load

Common application behavior should require as few concepts as practical.

### Determinism

Given equivalent source, compiler version, configuration, and dependency state, SAGE must interpret the program consistently.

### Safe defaults

Idiomatic SAGE should generally produce safe behavior by default.

### Declarative first

SAGE should prefer declarations of intent over implementation procedure.

### Progressive disclosure

SAGE should introduce concepts only when they become useful.

### One obvious path

Common operations should have one strongly preferred SAGE representation.

### Transparent abstraction

SAGE should hide unnecessary mechanics without hiding important reasoning.

### Human-centered errors

Diagnostics are part of the language.

## 7. Non-Goals

SAGE is not intended to:

- parse arbitrary conversational English,
- use LLMs to resolve ambiguous syntax,
- eliminate essential domain complexity,
- hide all implementation details permanently,
- replace every programming paradigm immediately,
- support every computing domain in its first versions,
- reproduce every feature from existing languages,
- optimize for clever or extremely terse syntax,
- make unsafe behavior easier than safe behavior.

## 8. Initial Application Domain

SAGE should initially focus on:

> **business applications, internal tools, automation, APIs, and data-oriented application software.**

These domains commonly include entities, records, relationships, validation, forms, tables, users, permissions, reports, workflows, notifications, scheduled tasks, APIs, and persistent data.

## 9. Language Model

SAGE source should describe application meaning in increasingly expressive layers. The following forms are target language behavior, not currently implemented compiler behavior.

### Applications

```sage
application Inventory
```

### Entities

```sage
A Product has:
    name as text
    quantity as whole number
```

### Fields

```sage
A Customer has:
    name as text
    email as email
```

## 10. Type System Direction

The initial target primitive types are planned to include:

```text
text
whole number
decimal number
yes or no
```

Expected semantic types include:

```text
email
url
date
date and time
identifier
money
```

## 11. Optionality

SAGE has no implicit null values.

Required:

```sage
name as text
```

Optional:

```sage
description as optional text
```

## 12. Initial Values

The following is the target initial-value form:

```sage
quantity as whole number, initially 0
active as yes or no, initially yes
```

Initial values must be statically type checked where possible.

## 13. Naming

SAGE should support human-readable identifiers while preserving source spelling, display name, and canonical identifier.

Normalization must be deterministic and collisions must produce diagnostics.

## 14. Future Relationships

Possible direction:

```sage
An Order belongs to a Customer.

A Customer has many Orders.
```

The exact syntax is not yet finalized.

## 15. Future Rules

Possible direction:

```sage
A Product is low on stock when:
    quantity is less than 5
```

## 16. Future Behavior

Possible direction:

```sage
When an Order is created:
    reduce inventory
    send confirmation email
```

## 17. Future Queries

Possible direction:

```sage
Products where:
    quantity is less than 5
    active is yes
```

## 18. Future Functions

Possible direction:

```sage
To calculate discount for a Customer:
    if the Customer is premium:
        return 20 percent

    otherwise:
        return 5 percent
```

Exact syntax remains undecided.

## 19. Future Tests

Possible direction:

```sage
test "premium customers receive a discount":

    given a Customer:
        total purchases is USD 1500

    expect:
        premium is yes
```

## 20. Future UI Model

Possible direction:

```sage
page Inventory:

    heading "Inventory"

    table of Products:
        show name
        show quantity
```

## 21. Future Persistence Model

Persistence should eventually be integrated with the semantic application model while keeping entities independent from a particular database implementation.

## 22. Future Identity and Authorization

Possible direction:

```sage
Users sign in with email.

Every Order belongs to a User.

Users may see only their own Orders.
```

## 23. Future Concurrency

SAGE should minimize the need for application developers to manually reason about threads, promises, event loops, and callback chains.

## 24. Capability Security

Future SAGE programs should not automatically receive unrestricted access to filesystem, networking, shell execution, environment variables, secrets, or external services.

## 25. Diagnostics Design

Diagnostics should explain what failed, where it failed, what SAGE expected, what SAGE found, and how it can be corrected when known.

## 26. Explainability

```bash
sage explain program.sage
```

should help developers understand what SAGE derived.

## 27. Unified Tooling

Expected direction:

```bash
sage new
sage run
sage check
sage test
sage fmt
sage explain
sage build
sage deploy
```

## 28. Canonical Formatting

SAGE should have one official source format.

Formatting should be deterministic, semantics preserving, and idempotent.

## 29. Intermediate Representation

SAGE should compile source into a semantic intermediate representation independent of surface syntax.

## 30. Multiple Backends

SAGE should not be semantically tied to a single execution backend.

## 31. Platform Model

Possible future platforms include Web, Automation, Desktop, Data, and Server platforms.

## 32. Development Phases

### Phase 0 — Foundation

Partially complete: the repository structure, Rust workspace scaffolding, and foundational documentation are present. CI and durable architectural decisions remain foundation work; the compiler stages and CLI are still planned rather than implemented.

### Phase 1 — Language Kernel

Parser, AST, source spans, entities, fields, types, optional values, initial values, semantic validation, IR, diagnostics, CLI, formatter.

### Phase 2 — Expressions and Behavior

Literals, expressions, comparisons, conditions, rules, reusable behavior, in-memory execution.

### Phase 3 — Queries and Persistence

Collections, relationships, declarative queries, SQLite reference persistence, schema derivation, migrations.

### Phase 4 — Application Runtime

HTTP, APIs, pages, forms, validation, sessions.

### Phase 5 — Identity and Security

Users, authentication, authorization, permissions, capabilities, secrets.

### Phase 6 — Events and Automation

Events, jobs, scheduling, notifications, external services, structured concurrency.

### Phase 7 — Production Platform

Build artifacts, observability, configuration, deployment, production persistence, reproducible environments.

## 33. Language Design Questions

Before introducing a major feature, ask:

1. What application concept does this represent?
2. Is the complexity essential or accidental?
3. Can SAGE eliminate the concept entirely?
4. Does an existing construct already express it?
5. Is the syntax deterministic?
6. Is the behavior easy to explain?
7. Can errors be reported clearly?
8. Does it compose with existing semantics?
9. Does it expose implementation machinery?
10. Can beginners ignore it until needed?
11. Does it unnecessarily constrain future backends?
12. Can `sage explain` make its behavior transparent?

## 34. Design Constraints

Strong defaults:

- no AI dependency,
- no implicit null,
- no semantic guessing,
- no premature backend coupling,
- no unnecessary duplication,
- no opaque automation,
- no feature imitation without justification.

## 35. Target SAGE 0.1 Language Subset

SAGE 0.1 is the target initial language subset, not an implemented compiler. The planned subset covers application declarations, entities, fields, primitive types, optional fields, initial values, semantic validation, normalized IR, and diagnostics. The current repository provides scaffolding and documentation for these capabilities; it does not yet compile or validate SAGE programs.

## 36. Success Criteria

SAGE succeeds if developers can build increasingly sophisticated software while needing to personally manage fewer unrelated implementation mechanisms.

## 37. Project North Star

The implementation beneath SAGE may be sophisticated.

That sophistication belongs in the programming system when it can be handled safely, generally, deterministically, and transparently.

> **Express intent. Hide machinery. Preserve control.**
