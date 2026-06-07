---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Project Contract

rusted-energyplus targets compatibility with the locked EnergyPlus 26.1.0
oracle. The Rust implementation must preserve EnergyPlus engineering behavior
unless a change is explicitly isolated in an experimental mode.

The machine-readable source of this contract is `specs/project_contract.toml`.

Allowed optimization areas are Rust representation, data layout, execution
planning, caching, tracing, diagnostics, result storage, numerical
implementation within declared tolerance, and code organization.

Forbidden areas without experimental isolation are engineering algorithm
changes, timestep semantic changes, setpoint-manager timing changes, and plant
dispatch semantic changes.

A compatibility claim requires:

```text
case manifest
+ declared variables or meters
+ tolerance rules
+ EnergyPlus oracle baseline
+ Rust result artifact
+ compare-summary.json
+ compare-report.md
+ blocking gate
```

Markdown wording, smoke tests, diagnostics, and performance results do not
create compatibility claims.
