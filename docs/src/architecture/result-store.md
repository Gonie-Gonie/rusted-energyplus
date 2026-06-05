---
status: active
claim_level: diagnostic
owner: runtime
last_reviewed: 2026-06-05
---

# Result Store

`ResultStore` is the runtime-native container for output series.

The current `first-zone` command writes a diagnostic `ResultStore`. That path
exercises runtime plumbing only and is not heat-balance conformance evidence.

Future conformance cases should write Rust result artifacts from `ResultStore`
or successor output stores and compare them through declared output requests.

