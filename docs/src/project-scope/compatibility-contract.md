---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-05
---

# Compatibility Contract

The target is compatibility with the locked EnergyPlus 26.1.0 oracle.

The Rust project does not replace EnergyPlus engineering algorithms. Any
optimization must preserve EnergyPlus behavior or explicitly document a measured
difference. Optimization may happen in:

- data layout
- numerical implementation details
- execution planning
- caching
- tracing
- diagnostics
- code organization

A compatibility claim requires:

```text
case manifest
+ declared output variables
+ declared tolerance rules
+ generated EnergyPlus oracle baseline
+ generated Rust result
+ compare-summary.json
+ compare-report.md
+ blocking gate in script or CI
```

No diagnostic command, smoke script, or runtime prototype may override this
contract.

