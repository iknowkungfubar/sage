# Threat Model: {{SYSTEM_OR_CHANGE}}

Status: Draft | Reviewed | Accepted
Owner: {{OWNER}}
Candidate/spec: {{REFERENCE}}
Last reviewed: {{DATE}}

## Scope and security objectives

In-scope components/data flows and explicit exclusions. Define confidentiality, integrity, availability, authenticity, authorization, privacy, and safety objectives as applicable.

## Assets and data

| Asset/data | Sensitivity | Owner | Storage/transit | Retention/deletion |
| --- | --- | --- | --- | --- |
| ... | ... | ... | ... | ... |

## Actors and capabilities

Legitimate roles, external attackers, malicious/compromised insiders, compromised dependencies/services, and agent/tool identities.

## Architecture and trust boundaries

Describe entry points, processes, identities, data stores, external services, privilege transitions, and egress. Link diagrams/code/schema evidence.

## Abuse cases

| ID | Attacker goal/path | Preconditions | Impact | Existing controls | Gap |
| --- | --- | --- | --- | --- | --- |
| `T-1` | ... | ... | ... | ... | ... |

Include authorization bypass, injection, unsafe parsing/execution, replay/idempotency, resource exhaustion, sensitive-data leakage, dependency/build compromise, monitoring/recovery failure, and agentic goal/tool/memory/inter-agent abuse where applicable.

## Controls and verification

| Threat | Prevent | Detect | Recover | Negative/abuse test | Owner |
| --- | --- | --- | --- | --- | --- |
| `T-1` | ... | ... | ... | ... | ... |

## Residual risk

State remaining likelihood/impact, evidence, assumptions, expiry/review trigger, and authorized risk owner acceptance. Never label risk “accepted” without an identifiable authorized owner.

## Review checklist

- Source-to-sink paths and intervening controls inspected.
- Authentication and object/action authorization negatives tested.
- Secrets/PII minimized and redacted.
- Resource/retry/concurrency bounds established.
- Dependencies/build/release trust reviewed.
- Monitoring, incident response, and recovery validated.
- Security reviewer findings resolved or explicitly accepted.
