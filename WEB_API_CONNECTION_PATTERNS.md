# Akrion Sim web/API connection patterns

Status: organization architecture guidance, tracked by [DEN-4262](https://linear.app/denman/issue/DEN-4262/document-akrion-sim-webapi-connection-patterns).

This policy applies to traditional simulation web/BFF, API, scheduler, result, telemetry, and background services.

## Four supported avenues

| Avenue | Appropriate use | Boundary |
| --- | --- | --- |
| Direct database read | Named non-sensitive public/reference or reproducible result projection with a measured need | Never identity, tenant-private models, job ownership, quota, billing, scheduler state, authorization, or writes; require distinct `SELECT`-only, `READ ONLY`, non-owner, `NOBYPASSRLS` access |
| Stateless HTTP/JSON | Default synchronous web-to-API path | Required for private reads, job submission/cancellation, resource allocation, billing, administration, and every mutation |
| Stateful TCP | Authorized simulation-output or measured telemetry stream after API authorization | Never job submission, quota, billing, persistence, or authorization authority; require ADR, mTLS/delegated identity, bounded frames, deadlines, backpressure, sequence handling, and reconnect policy |
| NATS/message queue | Durable post-commit scheduling effects, result processing, exports, and notifications | Never login, job acceptance, quota reservation, billing approval, or immediate response; require transactional outbox and idempotent consumers |

HTTP is the default. TCP may carry bounded simulation streams; job commands and state changes remain API mutations. NATS carries durable post-commit effects.

## Decision and ownership

1. Tenant-private data, authorization, job submission/cancellation, resource allocation, quota, billing, and every mutation use HTTP.
2. Immediate authoritative answers use HTTP with typed/versioned interfaces, bounded bodies/timeouts, correlation context, and idempotency.
3. Post-commit scheduling and result effects are inserted into a transactional outbox and delivered through NATS.
4. A measured simulation/telemetry stream may use TCP after an ADR and short-lived API authorization.
5. Direct reads remain limited to documented safe/reproducible projections under a dedicated restricted role.

The web/BFF owns HTML, secure opaque sessions, CSRF, and authorization-code plus PKCE. The API owns product authorization, job commands, quotas, and state transitions. A core/data package owns typed mappings and transaction helpers. The canonical migration repository owns DDL; services verify compatibility and never migrate production at startup.

Shared Auth proves identity and assurance, not model, job, quota, or result permission. Validate realm, issuer, audience, tenant, app/client, scopes, session, freshness, and assurance. Protected introspection keeps the service credential and user's token distinct. Never log tokens, cookies, PKCE material, private models, credentials, billing data, or raw introspection responses.

Use immutable dependency revisions. `opto-sync` supports declared synchronization/outbox flows, `ores-otel` provides bounded redacted telemetry and trace context, and `zed-pkg` records dependency provenance. None relocates API authorization, scheduler authority, or schema ownership.

## Operational requirements

- Bound bodies, frames, deadlines, retries, queues, buffers, and per-tenant resource use.
- Re-authorize streams on reconnect/expiry; make job commands idempotent and consumers duplicate-safe.
- Fail closed; never use a direct query to replace failed API authorization or quota decisions.
- Code comments identify the avenue and why the simulation/job constraints are satisfied.
- Every TCP or direct-read exception has an ADR, owner, measurements, and review/expiry date.

This document is the durable organization policy; repository ADRs may impose stricter controls.
