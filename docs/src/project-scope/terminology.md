---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-06-05
---

# Terminology

Use these terms consistently:

| Term | Meaning |
|---|---|
| oracle | Locked EnergyPlus 26.1.0 executable and outputs |
| reference source | Local EnergyPlus 26.1.0 source checkout used for porting maps |
| smoke | Execution or extraction succeeded |
| diagnostic-only | Values may be extracted or deltas printed, but no tolerance gate exists |
| baseline-only | EnergyPlus artifacts were generated, but Rust output is not compared |
| conformance | Rust output is compared with EnergyPlus using declared tolerances and a blocking gate |
| regression | Current Rust behavior is compared with an accepted Rust baseline |

Avoid generic claims. Prefer explicit labels such as `smoke_ok`, `extracted`,
`baseline_generated`, `tolerance_pass`, `tolerance_fail`, `unsupported`, and
`not_claimed`.

