# Execution Plan: {{TITLE}}

Candidate state: {{BRANCH_OR_REVISION}}
Risk tier: 0 | 1 | 2 | 3
Objective: {{OBJECTIVE}}

## Acceptance criteria

- `AC-1`: ...

## Assumptions and constraints

- Verified: ...
- Inferred: ...
- Unknown/blocking: ...

## Impact map

Components, interfaces, producers/consumers, trust boundaries, tests, docs, migration, and recovery paths.

## Steps

| ID | State | Outcome and owned scope | Depends on | ACs | Verification | Recovery/abort |
| --- | --- | --- | --- | --- | --- | --- |
| `S1` | proposed | ... | — | `AC-1` | ... | ... |

Allowed states: `proposed`, `ready`, `active`, `verifying`, `review`, `accepted`, `blocked`, `failed`, `cancelled`.

Exactly one shared-tree implementation step may be `active`. Parallel writers require isolated worktrees or disjoint declared ownership.

## Evidence ledger

| Claim/AC | Evidence | Result | Candidate state | Notes |
| --- | --- | --- | --- | --- |
| ... | ... | ... | ... | ... |

## Decisions

| Decision | Evidence/tradeoff | Alternatives rejected | ADR needed? |
| --- | --- | --- | --- |
| ... | ... | ... | Yes/No |

## Human gates

| Action | Exact target/values | Approver | Status/expiry | Recovery |
| --- | --- | --- | --- | --- |
| ... | ... | ... | ... | ... |

## Handoff

Outcome, changed behavior, checks, independent findings/dispositions, residual risk, blockers, and next safe action.
