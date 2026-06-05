---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-05
---

# Model Compiler

The compiler converts `RawModel` objects into typed EnergyPlus-domain structs.
The current compiler is a preview for declared seed object families.

Compiler output should separate:

- typed objects
- raw-only objects
- missing-reference diagnostics
- unsupported objects
- coverage status

Typed support is an input interpretation claim only. It is not a runtime
simulation claim.

