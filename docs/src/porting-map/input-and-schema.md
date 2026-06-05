---
status: active
claim_level: smoke
owner: core
last_reviewed: 2026-06-05
---

# Input and Schema

Current Rust side:

- `ep_raw_model` preserves epJSON object trees.
- `ep_compiler` creates typed preview objects for declared seed families.
- unsupported object types remain visible as raw-only coverage.

Reference:

- `.runtime/energyplus/26.1.0/Energy+.schema.epJSON`
- `.reference/energyplus-src/26.1.0/idd`

Next evidence target:

- field-default comparison
- unsupported object classification
- IDD/schema mapping report

