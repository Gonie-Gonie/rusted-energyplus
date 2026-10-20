---
status: active
claim_level: none
owner: core
last_reviewed: 2026-10-20
---

# Roadmap

The fixed CON-01 port and its accepted results are recorded in
[Current Status](current-status.md). Current priorities are:

1. Expand the valid input domains of the objects in the
   [example object union](../generated/example-object-union.md), working through
   project/time/site, schedules, materials/constructions, zones/surfaces/gains,
   controls/equipment and Plant dependencies. Register names before resolving
   cyclic references; preserve actual source execution order. For each object,
   review every field, default, enum/numeric domain and cross-field/reference
   condition. Example values are intake facts, not a limit on the intended
   supported domain. Record reviewed field boundaries in the existing object
   registry and admit runtime combinations through capabilities only after
   the [same-input unit, connection and production checks](project-contract.md#reviewing-a-porting-change).
2. Connect the completed Source reported-result identities and timestamps to
   oracle generation/comparison. Until then, use the separate-run
   [fixed-case verification procedure](../guides/verify-porting.md).
3. Make setup, build, two-engine execution and comparison a documented command
   that a fresh checkout can reproduce, with inspectable result files.

The current [capability registry](../../../specs/capabilities.toml) remains the
runtime admission boundary while this expansion proceeds. A union entry or a
field-level success does not establish arbitrary object-combination support;
broader HVAC, sizing, Plant, EMS, AFN and fenestration need their own connected
implementation and evidence.

Use focused comparisons for changed calculations, retained evidence for
unchanged producers, and one relevant integration check per milestone.
Historical versions are in Git and release notes. Keep this page to the next
actions rather than appending completed steps or per-run logs.
