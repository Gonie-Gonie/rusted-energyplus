---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# Legacy Milestones

v0.1 through v0.15 are the Historical Pre-Alpha Evidence Series. They remain
valuable, but they are not compatibility maturity levels. Each entry below is
an interpretation table for existing docs, scripts, and release artifacts.

| Milestone | Historical evidence | Public compatibility boundary |
|---|---|---|
| v0.1 | RawModel/TypedModel intake preview, package basics | no runtime conformance |
| v0.2 | conformance harness, baseline generation, report skeleton contract | harness only |
| v0.3 | input interpretation, name/reference diagnostics | input interpretation only |
| v0.4 | RunPeriod, EPW, and schedule smoke evidence | listed time/weather/schedule variables only; not broad conformance |
| v0.5 | static geometry, construction/material, nominal internal-gain smoke evidence | input/static evidence only |
| v0.6 | ResultStore, OutputRegistry, trace, compare/report infrastructure | no heat-balance claim |
| v0.7 | EnergyPlus source maps and algorithm porting readiness | planning guard |
| v0.8 | first tolerance-gated no-mass heat-balance case | `heat_balance_nomass_001` MAT only |
| v0.9 | first tolerance-gated no-mass surface-state case | `surface_temperature_nomass_001` declared variables only |
| v0.10 | IdealLoads/thermostat typed graph and baseline-only output availability | no IdealLoads load conformance |
| v0.11 | air-side node baseline plus Rust projection plumbing | diagnostic-only; no node or HVAC conformance |
| v0.12 | node source map and policy hardening | planning guard |
| v0.13 | PlantLoop typed graph skeleton | smoke only; no plant numerical conformance |
| v0.14 | plant source map | planning guard |
| v0.15 | PlantLoadProfile plant-loop oracle baseline rows | diagnostic-only; no plant numerical conformance |

## Additional Diagnostic Addenda

The Rust plant-state projection added after v0.15 is kept as a diagnostic
addendum. It writes `plant-state-summary.md` and `plant-state-summary.json` for
the v0.15 plant diagnostic fixture and keeps these markers:

```text
comparison_class: diagnostic-only
conformance_claim: false
algorithm_parity: false
tolerance_policy: none
```

This addendum is useful runtime artifact plumbing, but it is not a plant loop
solver, operation-scheme, flow-balancing, meter, sizing, or ExampleFiles
compatibility claim.

## Promotion Rule

A historical or addendum artifact may be promoted only when it gains:

- case manifest.
- declared variables and meters.
- EnergyPlus oracle baseline.
- Rust result artifact produced by the relevant algorithm path.
- tolerance policy.
- compare report and summary JSON.
- blocking release or CI gate.

Until then, it remains smoke, baseline-only, diagnostic-only, or source-map
evidence.
