---
status: active
claim_level: diagnostic
owner: runtime
last_reviewed: 2026-06-07
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

v0.6 diagnostic artifacts include:

- `compare-zone` MAT `compare-summary.json` and `compare-report.md`
- manifest-driven MAT diagnostic reports under `.runtime/conformance-diagnostic`
- compare regression `trace.json`, `compare-summary.json`,
  `compare-report.md`, and `profile-summary.json`

Zone-temperature diagnostics must keep extraction-only semantics until a future
heat-balance conformance milestone declares tolerances and blocking gates.
