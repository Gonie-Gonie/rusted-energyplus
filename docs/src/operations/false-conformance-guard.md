---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-05
---

# False Conformance Guard

The guard prevents dangerous wording from reappearing in docs, scripts, and CLI
output.

Forbidden or high-risk phrases include:

- EnergyPlus simulation works
- zone temperature comparison passes
- first EnergyPlus-compatible runtime
- fully compatible subset
- heat-balance compatible
- compatible runtime
- simulation passes
- parity achieved

Diagnostic commands must expose:

```text
comparison_class: diagnostic-only
conformance_claim: false
tolerance_policy: none
status: extracted
```

Run it with `.\scripts\dev.cmd strict-no-false-conformance`. The implementation
file lives under `scripts/quality`.
