# Oh-My-Pi Governance Adoption Baseline

Baseline captured before implementation on branch `chore/omp-governance-adoption` at revision `4aaf04e2313abb94335c9a4a5f7b24a669f76600`.

## Working tree

- Current worktree: clean.
- Linked worktrees: clean at review time; no tracked or untracked changes reported.
- No files in the adoption scope existed before this change except `AGENTS.md`, `CONTRIBUTING.md`, and `.gitignore`.

## Existing canonical files

- `AGENTS.md`
- `CONTRIBUTING.md`
- `.gitignore`

## Approved additions

- `.omp/APPEND_SYSTEM.md`
- `.omp/rules/non-negotiables.md`
- `.omp/skills/enterprise-sdlc/SKILL.md`
- `.omp/skills/deep-research/SKILL.md`
- `.omp/agents/qa-verifier.md`
- `.omp/agents/evidence-researcher.md`
- `governance/project.schema.json`
- `governance/knowledge-graph.schema.json`
- `governance/project.json`
- `scripts/agent/verify.py`
- `scripts/agent/validate_knowledge_graph.py`
- `docs/templates/PLAN.md`
- `docs/templates/RESEARCH_REPORT.md`
- `docs/templates/SPEC.md`
- `docs/templates/THREAT_MODEL.md`
- `docs/adr/0000-template.md`
- `docs/engineering/HUMAN_APPROVALS.md`
- `docs/engineering/RESEARCH_PROTOCOL.md`
- `docs/engineering/SOURCES.md`
- `docs/knowledge/graph.json`

## Approved modifications

- Merge compatible kit controls into `AGENTS.md`.
- Merge compatible kit contribution controls into `CONTRIBUTING.md`.
- Add only required verifier/report ignore entries to `.gitignore`.

## Explicitly deferred

- `.omp/config.yml` pending local oh-my-pi installation/version confirmation.
- `SECURITY.md` pending a real reporting channel and owner.
- GitHub CI, pull-request, and issue templates pending repository-specific GitHub decisions.
- Generic engineering handbook files not listed above pending a decision on mandatory adoption.
- Release profile and deployment controls; no repository evidence supports them.
- Legacy harness retirement; no files will be deleted.
