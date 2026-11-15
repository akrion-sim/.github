# Rust-first tooling admission

This dependency-free Rust CLI enforces the incremental first-party tooling
policy in this repository. It checks **added/changed** test and tooling Python
files, and **added** Python runner invocations in shell, Make/Just and GitHub
Actions workflow changes. Unchanged legacy Python scripts are not grandfathered
for new edits; they must be migrated to Rust with behavior-preserving tests.

A generated Python *client* library and a documented Python *application*
implementation are not first-party test/CI scripts.

Run locally from the repository root:

```sh
cargo fmt --manifest-path tools/rust-script-policy/Cargo.toml -- --check
cargo clippy --manifest-path tools/rust-script-policy/Cargo.toml --all-targets -- -D warnings
cargo test --manifest-path tools/rust-script-policy/Cargo.toml
cargo run --manifest-path tools/rust-script-policy/Cargo.toml -- --base <40-hex-base> --head <40-hex-head>
```

The guard is intentionally scoped to this repository's Git diff. GitHub does
not automatically inherit workflows or custom agent instructions from the
public organization `.github` repository. Other repositories should adopt
the same Rust policy gate explicitly. Source-specific tests and their
existing validation remain required.

This checker does **not** replace or validate the legacy relationship registry
Python tools. Migrate and run parity checks before removing those validators.
