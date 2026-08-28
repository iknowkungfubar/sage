# Design Sources and Evidence Notes

Research cutoff: 2026-08-26. Prefer the linked primary/current sources over this summary and recheck them when upgrading the kit.

## oh-my-pi implementation and documentation

- [oh-my-pi repository](https://github.com/can1357/oh-my-pi) — upstream source inspected at commit `a4dbcda37ad3fc89b8812d08765d93d4e060101d`, package version 18.0.6.
- [System prompt customization](https://github.com/can1357/oh-my-pi/blob/main/docs/system-prompt-customization.md) — `APPEND_SYSTEM.md` preserves the default prompt; `SYSTEM.md` replaces its unique tool/workflow policy while retaining generated context.
- [Context files](https://github.com/can1357/oh-my-pi/blob/main/docs/context-files.md) — instruction discovery, precedence, shadowing, and always-apply rules.
- [Skills](https://github.com/can1357/oh-my-pi/blob/main/docs/skills.md) — non-recursive `.omp/skills/<name>/SKILL.md` discovery and runtime loading.
- [Tool approval mode](https://github.com/can1357/oh-my-pi/blob/main/docs/approval-mode.md) — read/write/exec tiers, per-tool policy, headless subagent behavior.
- [Task agent discovery](https://github.com/can1357/oh-my-pi/blob/main/docs/task-agent-discovery.md) — custom agent schema, role routing, structured output, recursion controls.
- [Autonomous memory](https://github.com/can1357/oh-my-pi/blob/main/docs/memory.md) — memory is heuristic and must be verified against current repository evidence.

## Agent/workflow engineering

- Anthropic, [Building Effective AI Agents](https://www.anthropic.com/engineering/building-effective-agents) — start with the simplest effective pattern; routing, parallelization, orchestrator-workers, and evaluator-optimizer have distinct use cases.
- Anthropic, [How we built our multi-agent research system](https://www.anthropic.com/engineering/multi-agent-research-system) — parallel research architecture, delegation, evaluation, and production lessons.
- OpenAI, [Guardrails and human review](https://developers.openai.com/api/docs/guides/agents/guardrails-approvals) — automatic validation plus human approval controls continue/pause/stop decisions.
- OpenAI, [Evaluate agent workflows](https://developers.openai.com/api/docs/guides/agent-evals) — trace- and dataset-based evaluation for agent behavior.
- Yang et al., [SWE-agent: Agent-Computer Interfaces Enable Automated Software Engineering](https://arxiv.org/abs/2405.15793) — agent-computer interface design materially affects software-agent performance.
- Jimenez et al. and later long-horizon work, including [SWE-Marathon](https://arxiv.org/abs/2606.07682) — current frontier systems still struggle on extended engineering tasks; multi-layer executable verification remains necessary.

## Software delivery and secure development

- DORA, [2025 State of AI-assisted Software Development](https://dora.dev/dora-report-2025/) and [Working in small batches](https://dora.dev/capabilities/working-in-small-batches/) — AI amplifies the surrounding system; small batches improve reviewability and delivery outcomes.
- NIST, [SP 800-218 Secure Software Development Framework](https://csrc.nist.gov/pubs/sp/800/218/final) and [SP 800-218 Rev. 1 initial public draft](https://csrc.nist.gov/pubs/sp/800/218/r1/ipd) — prepare, protect, produce, and respond practices across the SDLC. Rev. 1 was still a draft at the research cutoff.
- NIST, [SP 800-218A](https://csrc.nist.gov/pubs/sp/800/218/a/final) — secure development practices for generative AI and dual-use foundation models.
- OWASP, [Top 10 for Agentic Applications 2026](https://genai.owasp.org/resource/owasp-top-10-for-agentic-applications-for-2026/) — goal hijacking, tool misuse, identity abuse, memory poisoning, inter-agent risks, cascading failures, trust exploitation, and rogue agents.
- OWASP, [Top 10 for LLM Applications 2026](https://genai.owasp.org/resource/owasp-genai-llm-top-10-2026/) — current LLM application security risks.
- OpenSSF, [Concise Guide for Developing More Secure Software](https://best.openssf.org/Concise-Guide-for-Developing-More-Secure-Software.html) — development, build, and distribution security practices.
- SLSA, [Specification v1.2 provenance](https://slsa.dev/spec/v1.2/build-provenance) — verifiable artifact origin and build information.

## GitHub enforcement

- GitHub, [Protected branches](https://docs.github.com/repositories/configuring-branches-and-merges-in-your-repository/defining-the-mergeability-of-pull-requests/about-protected-branches) — required reviews and status checks.
- GitHub, [Dependency review](https://docs.github.com/code-security/supply-chain-security/understanding-your-software-supply-chain/about-dependency-review) — PR-level dependency change analysis.
- GitHub, [Code scanning merge protection](https://docs.github.com/code-security/concepts/code-scanning/merge-protection) — ruleset enforcement for code scanning results.

## Knowledge graphs

Repository knowledge graphs are promising but not a universal proven requirement. The kit therefore uses a small, evidence-backed graph rather than making a graph database mandatory.

- Zhang et al., [Knowledge Graph Based Repository-Level Code Generation](https://arxiv.org/abs/2505.14394) — structural/relational repository representations for code generation.
- KGCompass, [Knowledge Graph Enhanced Repository-Level Software Repair](https://arxiv.org/abs/2503.21710) — links code structure and repository metadata for repair/localization.

## Interpretation

The control graph and graph-of-loops design are an inference and synthesis from established workflow/state-machine/control practices plus current agent research. The phrase itself is newer and should not be treated as a certification, standard, or guarantee. The enforceable value comes from explicit state, typed edges, bounded retries, independent verification, permissions, CI, and human gates.
