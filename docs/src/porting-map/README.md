---
status: archived
claim_level: none
owner: runtime
last_reviewed: 2026-06-05
---

# Porting Map Legacy Note

This note is retained as the original porting-map template. Active maps now
live under the specific pages linked from `SUMMARY.md`.

Every implementation that follows EnergyPlus behavior should record:

- Rust file
- EnergyPlus source file or documentation section
- EnergyPlus version/tag
- observed behavior
- compatibility tests
- known tolerances

Example:

```text
Rust file:
  crates/ep_raw_model/src/lib.rs

Reference:
  .reference/energyplus-src/26.1.0/schema/Energy+.schema.epJSON

Notes:
  RawModel preserves original epJSON object/field names before typed
  normalization.
```
