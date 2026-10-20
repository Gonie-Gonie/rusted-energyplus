---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-06-05
---

# Introduction

`rusted-energyplus` is a Rust-only EnergyPlus-compatible porting project. The
project is organized around strict oracle evidence, not demos.

The locked initial oracle is EnergyPlus 26.1.0. A feature may be implemented,
smoked, or diagnostic-only before it is conformance evidence. A compatibility
claim requires declared cases, variables, tolerances, reports, and a blocking
gate.

Use [Current Status](current/current-status.md) for scope and limitations,
[Setup](guides/setup.md) and [Run an IDF](guides/run-arbitrary-idf.md) to execute
models, and [Verify the port](guides/verify-porting.md) to inspect Oracle/Rust
comparisons. [Developer Workflow](guides/developer-workflow.md) covers focused
checks. Technical ownership and claim rules start at the
[project contract](current/project-contract.md) and
[architecture overview](current/architecture-overview.md).

