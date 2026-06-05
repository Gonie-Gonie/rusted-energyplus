---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
---

# Conformance Overview

Conformance means Rust output is compared against EnergyPlus oracle output with
declared variables, declared tolerances, human-reviewable reports, and a
blocking gate.

The current conformance infrastructure is still being built. Some existing
scripts are smoke or diagnostic gates; they are not conformance evidence unless
their case manifest and report contract say so.

Minimum conformance bundle:

```text
case.toml
output requests
tolerance policy
EnergyPlus baseline artifacts
Rust result artifacts
compare-summary.json
compare-report.md
blocking script or CI gate
```

