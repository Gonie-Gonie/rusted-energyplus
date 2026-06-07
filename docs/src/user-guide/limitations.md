---
status: active
claim_level: none
owner: docs
last_reviewed: 2026-06-08
---

# Limitations

Current public scope does not include general heat-balance, HVAC, plant, meter,
or ExampleFiles conformance. Tolerance-gated numerical conformance is limited
to the declared v0.8/v0.9 no-mass cases, the v0.22 `Schedule Value` and
dry-bulb hourly variables, and the v0.26 internal convective gain hourly
variable.

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

v0.26 promotes only `Zone Total Internal Convective Heating Rate` for
`internal_gains_001`. It does not claim zone air temperature response to
internal gains, radiant/latent coupling, HVAC, plant, meters, or broad
heat-balance compatibility.

v0.27 adds a user-facing support coverage report for tracked inputs, outputs,
and algorithm families. v0.28 enriches input object coverage metadata with
first evidence and support boundaries. v0.29 enriches output variable coverage
metadata with strongest-evidence first references and support boundaries.
v0.30 enriches algorithm coverage metadata with first evidence and support
boundaries. v0.31 adds the release evidence asset manifest. v0.32 adds the
user coverage handbook. These reporting updates do not promote new numerical
conformance.

For the formal non-goals list, see `project-scope/non-goals.md`.
