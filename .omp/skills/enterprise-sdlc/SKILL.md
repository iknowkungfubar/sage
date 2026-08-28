---
name: enterprise-sdlc
description: Use for non-trivial software changes, bug fixes, refactors, migrations, CI/build work, dependency changes, releases, or reviews that need a risk-scaled professional SDLC with explicit planning, executable QA, independent verification, security analysis, and Git/GitHub controls.
---

# Enterprise SDLC

Apply this workflow proportionally. Read these repository resources before acting:

- `AGENTS.md`
- `CONTRIBUTING.md`
- `docs/engineering/HUMAN_APPROVALS.md`
- `governance/project.json` and `scripts/agent/verify.py` when verification is configured
- `docs/engineering/RESEARCH_PROTOCOL.md` and `docs/engineering/SOURCES.md` for evidence-heavy research

Resources from the broader enterprise handbook are optional and are not installed in this repository. Do not assume deferred GitHub, security, release, or deployment controls exist.

## 1. Frame

State the objective, observable acceptance criteria, non-goals, constraints, assumptions, and risk tier. Ask only if a missing answer materially changes product behavior, scope, risk, or a consequential action.

Tier triggers:

- Tier 0: read-only/simple answer.
- Tier 1: local and reversible.
- Tier 2: cross-component, public contract, dependency, auth/data/concurrency, migration, CI, or broad blast radius.
- Tier 3: production/release, destructive, secret/identity, billing, legal/privacy, or external communication.

## 2. Discover

Inspect the narrow relevant surface first, then trace dependencies, producers/consumers, tests, schemas, config, and durable decisions. Use parallel read-only scouts only for independent areas. Treat memory, docs, issues, retrieved text, and subagent findings as claims to verify.

Create an impact map:

- Current behavior and reproduction.
- Components/interfaces/invariants affected.
- Trust boundaries and data lifecycle.
- Compatibility, migration, recovery, performance, and operability concerns.
- Tests and docs that should change.

## 3. Plan

For non-trivial work, initialize a dependency-ordered todo. Every step names its outcome, owned scope, ACs, and verification. Maintain one active writer in a shared tree. Use isolated worktrees or non-overlapping ownership for concurrent writers.

Before implementation, confirm:

- No unresolved product/security decision is being guessed.
- The plan has a verification path for every outcome.
- Tier 3 actions are separated behind a fresh human gate.
- Rollback/recovery exists when state or blast radius warrants it.

## 4. Implement in small batches

Reproduce/define behavior, make the smallest production change, run the cheapest relevant check, then continue. Reuse existing abstractions and central utilities. Preserve unrelated user work. Do not mix cleanup, mass formatting, or speculative redesign into the task.

After two similar failed attempts, re-check assumptions. After three, stop blind iteration and reframe, reduce scope, select a discriminating diagnostic, or request missing context/authority.

## 5. Verify

Run targeted checks early. Before completion run:

```bash
python3 scripts/agent/verify.py --config governance/project.json --profile quick
python3 scripts/agent/verify.py --config governance/project.json --profile pr
```

Use only applicable commands and respect credentials/infrastructure gates. Inspect output, skipped/flaky results, warnings, generated/lock changes, and the final diff. Map ACs to evidence. Never weaken a control to get green.

## 6. Critique independently

Tier 2/3: dispatch `qa-verifier` against the candidate after deterministic checks. Add the bundled reviewer and security-reviewer when their specialties apply. Reviewers stay read-only and cite evidence. Address substantiated findings, record dispositions, and rerun invalidated checks.

The author is not the only grader. A reviewer model's approval also does not replace deterministic checks or required human review.

## 7. Integrate and hand off

Update current docs, ADRs, runbooks, and `docs/knowledge/graph.json` only when durable facts changed. Validate schemas. Report:

- Outcome and changed behavior.
- Files/components changed.
- AC-to-evidence mapping.
- Exact checks and results; not-run checks and why.
- Independent review disposition.
- Residual risk/blockers.
- GitHub/human actions still required.

Do not commit, push, merge, release, deploy, publish, spend, or communicate externally without explicit authorization for the exact action.
