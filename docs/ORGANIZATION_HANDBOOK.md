# akrion-sim organization handbook

> Shared operating defaults for repositories maintained under **akrion-sim**. Repository-local policy may strengthen these rules but should not silently weaken them.

## Mission

akrion-sim maintains simulation software, reusable models, scenario tooling, and analysis infrastructure. This `.github` repository is the canonical home for organization-wide community health files, reusable templates, engineering policy, and planning links.

## Repository contract

Each active repository must document purpose, ownership, maturity, supported environments, reproducible development and test commands, authoritative model and data formats, release and rollback procedures, compatibility policy, and GitHub Project/Linear links. Simulation components must also document assumptions, units, random seeds, time semantics, determinism, calibration inputs, numerical tolerances, and validation limits.

## Change and review workflow

1. Anchor work in an issue, Linear item, or documented maintenance objective.
2. Keep branches and pull requests focused.
3. Explain motivation, assumptions, scope, risk, validation, compatibility, migration, and rollback.
4. Test deterministic replay, edge cases, numerical boundaries, invalid inputs, and performance as relevant.
5. Resolve conflicts semantically by reconstructing both sides' intent.
6. Prefer squash merges for focused work unless commit structure materially improves auditability.

## Evidence and quality

Pull requests should include reproducible commands, fixtures and seeds, expected and observed results, tolerance rationale, negative-path coverage, documentation updates, and CI or local-equivalent evidence. Model changes require comparison against trusted baselines and clear interpretation of changed outputs.

## Security and data

Never commit credentials, private datasets, proprietary scenarios, or sensitive logs. Use synthetic or approved sanitized fixtures. Follow `SECURITY.md` for private vulnerability reporting and pin dependencies, actions, containers, and generated inputs where reproducibility matters.

## Documentation and decisions

Keep examples executable, links current, assumptions explicit, units consistent, and model boundaries clear. Record architectural, model, calibration, compatibility, and operational decisions that future maintainers would otherwise have to rediscover.

## Planning ownership

GitHub owns code, reviews, checks, releases, and delivery evidence. Linear owns priority, dependencies, sequencing, and cross-project planning. The organization GitHub Project is the cross-repository execution view; see `PROJECTS.md` for routing details.

## Organization health

- [ ] Profiles, descriptions, topics, and READMEs are current.
- [ ] Contribution, security, support, governance, issue, and PR guidance is present.
- [ ] Models document assumptions, units, seeds, tolerances, and validation limits.
- [ ] Required checks reflect correctness, determinism, performance, and supply-chain risk.
- [ ] Stale repositories are archived or clearly marked.
- [ ] Project links resolve and completed work is reflected in GitHub and Linear.
