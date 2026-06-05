---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-05
---

# CI Policy

Local and CI gates should include:

- `cargo fmt --all -- --check`
- `cargo clippy --workspace --all-targets -- -D warnings`
- `cargo test --workspace`
- docs build
- conformance schema smoke
- false-conformance guard
- source/oracle availability checks where the environment supports them

Conformance gates must fail when declared tolerance policy fails.

