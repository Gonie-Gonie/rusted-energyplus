---
status: active
claim_level: diagnostic
owner: docs
last_reviewed: 2026-06-05
---

# Diagnostics

Diagnostic commands are development tools. They can extract values and produce
trace artifacts before conformance evidence exists.

Current diagnostic-only commands include:

- `run first-zone`
- `compare zone-temperature`

They must keep `conformance_claim: false` and must not be used as heat-balance
conformance evidence.

