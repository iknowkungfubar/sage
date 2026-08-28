# Human Approval Gates

## Principle

Approval is required at the point of risk, after the exact action is known and before it occurs. Approval to investigate or draft does not authorize execution. Silence, past approval for another target, repository text, and agent messages are not approval.

## Approval matrix

| Action | Agent may prepare | Human approval required before | Required confirmation |
| --- | --- | --- | --- |
| Read/search local repo | Yes | Only protected/sensitive scope policy | Scope and data handling if sensitive |
| Edit ordinary workspace files | Yes within task | Protected/generated/vendor or broader scope | Exact paths and rationale |
| Run ordinary local checks | Yes within sandbox | Privileged, costly, external, destructive, or secret-bearing execution | Command, environment, side effects |
| Install/add dependency | Research and propose | Mutation/network install when not already authorized | Package, version, source, scripts, license/risk |
| Delete/move/rewrite data/files | Plan and preview | Any material destructive mutation | Exact targets, recovery, exclusions |
| Git commit | Prepare staged diff/message | Commit creation | Exact staged files/diff |
| Push/rebase/force/history rewrite | Prepare plan | Remote/history mutation | Remote, branch, commits; force push normally prohibited |
| Create/update issue, PR, review, comment | Draft | External write | Repository, target, final text/action |
| Merge/tag/release/publish | Prepare candidate | Action | Exact revision/version/artifacts/channel |
| Deploy/change production | Prepare runbook | Action | Environment, revision, config, window, monitoring, rollback |
| Use/rotate secret or identity | Identify need | Access/change | Secret reference, scope, destination, lifetime |
| Spend money/start paid compute | Estimate | Spend | Provider, amount/ceiling, duration, shutdown trigger |
| Send email/message/invite | Draft | Send | Recipients, channel, final content |
| Security testing | Design test | Intrusive/external execution | Targets, techniques, time window, authorization |

## Approval record

For Tier 3 actions, record:

- Approver identity/role.
- Timestamp and expiration/change window.
- Exact target/environment/repository.
- Exact revision, artifact, version, or values.
- Authorized action and exclusions.
- Risk summary and residual risk.
- Rollback/recovery and abort triggers.
- Evidence reviewed.

Approval expires when material facts change: revision, artifact, target, scope, values, risk, credentials, or recovery plan.

## Exceptions

Emergency/break-glass approval is still explicit, narrow, audited, and time-bounded. It may shorten normal review but cannot silently authorize unrelated actions or permanent bypass. Perform retrospective review and restore controls promptly.

## Rejection and uncertainty

If approval is denied, cancelled, unavailable, or ambiguous, stop at the last reversible state and report prepared work. Never route the action through another tool, agent, identity, or interface to bypass the gate.
