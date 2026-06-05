---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
---

# Evidence Levels

Every command, report, and milestone should identify its evidence level.

| Level | Name | Meaning | Can support public compatibility claim? |
|---|---|---|---|
| E0 | Setup | Toolchain, oracle, or reference source exists | No |
| E1 | Smoke | A command ran and produced expected files | No |
| E2 | Diagnostic | Values were extracted and deltas may be printed, but no tolerance is enforced | No |
| E3 | Baseline-only | EnergyPlus oracle artifacts were generated from a manifest | No |
| E4 | Conformance | Rust output is compared against EnergyPlus with declared tolerance and blocking gate | Yes, for that case and variable only |
| E5 | Regression | Current Rust behavior is compared against accepted Rust baseline | Yes, for non-oracle regression only |
| E6 | Performance | Runtime, memory, or profile counters are compared under fixed conditions | Yes, for performance claims only |

The word `pass` should be read together with its evidence level. A smoke pass is
not a conformance pass.

