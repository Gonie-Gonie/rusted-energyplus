---
status: active
claim_level: diagnostic
owner: runtime
last_reviewed: 2026-06-05
---

# Diagnostics and Trace

Diagnostics and traces are allowed before conformance evidence exists.

They must identify their class:

```text
comparison_class: smoke | diagnostic-only | conformance | regression | performance
conformance_claim: true | false
tolerance_policy: none | case.toml | default-<milestone>
```

Diagnostic traces help locate differences. They do not by themselves support a
compatibility claim.

