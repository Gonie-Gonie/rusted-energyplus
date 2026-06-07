---
status: active
claim_level: none
owner: runtime
last_reviewed: 2026-06-07
---

# Execution Plan

`ExecutionPlan` records coarse runtime stages and graph-derived work ordering.
It is currently an architecture boundary and diagnostic summary.

The plan is useful for:

- validating graph connectivity
- making runtime stages inspectable
- preparing trace/report infrastructure

It does not imply that all EnergyPlus algorithms behind those stages have been
ported.

`ExecutionPlan` is the place where compatibility-mode ordering becomes
explicit. The default compatibility path must preserve EnergyPlus-aligned
barriers before fast or experimental scheduling is considered.

v0.10 adds thermostat and IdealLoads ordering placeholders:

- `EvaluateZoneThermostat`
- `EvaluateIdealLoadsAirSystem`

They are typed-graph readiness markers for `ideal_loads_thermostat_001`, not
HVAC load-conformance markers.

Future execution stages should represent environment, warmup, zone, system,
plant, reporting, callback, and output barriers. EMS, PythonPlugin, API
actuators, and other callbacks must become explicit invalidation points before
dependent caches can be reused across them.
