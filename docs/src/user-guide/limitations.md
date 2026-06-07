---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-06-07
---

# Limitations

Current public scope does not include general heat-balance, HVAC, plant, meter,
or ExampleFiles conformance. Tolerance-gated numerical conformance is limited
to the declared v0.8/v0.9 no-mass cases plus the v0.22 `Schedule Value` and
dry-bulb hourly variables.

v0.23 adds static EIO model evidence for the official `1ZoneUncontrolled`
ExampleFile only. That does not imply dynamic heat-balance, HVAC, plant,
solar, fenestration, sizing, warmup, meter, or broad ExampleFiles
compatibility.

v0.24 adds runtime output registry, meter diagnostic, and ResultStore
infrastructure. It does not add new numerical conformance or meter
conformance.

v0.25 adds opaque no-mass adiabatic and interzone boundary handling for the
runtime heat-balance state. It does not add general heat-balance compatibility
or new promoted numerical variables beyond the declared existing cases.

For the formal non-goals list, see `project-scope/non-goals.md`.
