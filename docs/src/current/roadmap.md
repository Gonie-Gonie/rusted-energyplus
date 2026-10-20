---
status: active
claim_level: none
owner: core
last_reviewed: 2026-10-20
---

# Roadmap

The fixed CON-01 port and its accepted results are recorded in
[Current Status](current-status.md). Current priorities are:

1. Connect the completed Source reported-result identities and timestamps to
   oracle generation/comparison. Until then, use the separate-run
   [fixed-case verification procedure](../guides/verify-porting.md).
2. Make setup, build, two-engine execution and comparison a documented command
   that a fresh checkout can reproduce, with inspectable result files.
3. Expand input coverage only from a new explicit source/branch/state contract.
   Broader HVAC, sizing, Plant, EMS, AFN and fenestration remain separate work.

Use focused comparisons for changed calculations, retained evidence for
unchanged producers, and one relevant integration check per milestone.
Historical versions are in Git and release notes. Keep this page to the next
actions rather than appending completed steps or per-run logs.
