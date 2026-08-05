# Desktop application allocation

Verified **2026-08-05**.

Akrion Sim is a **strong candidate** for a paired native desktop simulation workbench:

- Rust: [`akrion-sim/akrion-desktop.rs`](https://github.com/akrion-sim/akrion-desktop.rs) — **proposed**, not yet verified as a published repository.
- Flutter: [`akrion-sim/akrion-flutter`](https://github.com/akrion-sim/akrion-flutter) — **proposed**, not yet verified as a published repository.

These names are proposed allocation targets, not proof that either remote exists and not a claim that implementation work is approved or complete. Promote the pair from proposed to planned only when scope, ownership, milestones, and repository creation are accepted in Linear.

## Product boundary

The pair should cover semantic parity for scenario editing, model and parameter management, batch runs, deterministic replay, timeline scrubbing, result comparison, visualization, import/export, local datasets, experiment bundles, offline execution, and failure recovery.

A Rust simulation core may be shared through an explicit library, FFI, or service boundary, but the Flutter application remains independently buildable, testable, and releasable. Shared schemas, run manifests, fixtures, seeds, golden results, and conformance tests should be versioned deliberately.

## Feature-delivery rule

Once planned, every desktop-facing change must inspect both implementations, define shared acceptance criteria, update both or record an explicit no-change rationale, and report Rust and Flutter status separately.

## Project routing

- GitHub Project: [`akrion-sim-project` — Project 1](https://github.com/orgs/akrion-sim/projects/1)
- Linear project: `github.com/akrion-sim`
- Central registry: [`ORESoftware/project-registry`](https://github.com/ORESoftware/project-registry/blob/main/registry/desktop-applications.json)
- Portfolio rollout: [`DEN-2469`](https://linear.app/denman/issue/DEN-2469/roll-out-paired-rust-flutter-desktop-repositories-across-the-portfolio)

Promotion, repository creation, renames, transfers, archival, or platform-status changes must update this document, Linear, the central registry, and both companion repositories together.
