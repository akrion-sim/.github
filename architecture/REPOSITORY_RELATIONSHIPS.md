# `akrion-sim` repository relationships

Generated from reviewed policy and the current **public** repository inventory.

- Public repositories declared: **3**
- Private repository names withheld: **8**
- Relationship edges: **4**

## Repository roles

| Repository | Role | Lifecycle |
|---|---|---|
| [`.github`](https://github.com/akrion-sim/.github) | `organization_governance` | `active` |
| [`akrion-sim.github.io`](https://github.com/akrion-sim/akrion-sim.github.io) | `site` | `active` |
| [`akrion-soccer-engine-rs`](https://github.com/akrion-sim/akrion-soccer-engine-rs) | `uncategorized` | `active` |

## Declared edges

| From | Relationship | To | Status/basis |
|---|---|---|---|
| `akrion-sim/.github` | `governs` | `akrion-sim/akrion-sim.github.io` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `akrion-sim/.github` | `governs` | `akrion-sim/akrion-soccer-engine-rs` | `inferred` / `role-convention`: organization defaults, safety, and relationship declarations |
| `organization://akrion-sim` | `researches_with` | `organization://usa-acc` | `declared` / `explicit-product-decision`: simulation and control-system model exchange |
| `organization://akrion-sim` | `packaged_via` | `platform://zed-pkg` | `platform-default` / `platform-policy`: Zed resolves artifacts while submodules compose editable source |

## Composition, service, and observability contract

Git submodules compose editable source; Zed packages resolve packages/artifacts; dual-managed commits must match. Production deploys immutable image digests, not runtime source builds. Cross-service access uses APIs/SDKs/events rather than another service database. MCP uses the product API/SDK. Services emit OpenTelemetry traces, bounded metrics, and correlated structured logs.

## Privacy boundary

This public registry deliberately omits private repository names and edges; the count above makes the boundary explicit.
