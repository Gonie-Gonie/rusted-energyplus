---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# ExampleFile Suite

The ExampleFile suite is the future bridge from reduced smoke fixtures to real
EnergyPlus compatibility reports. It uses selected EnergyPlus ExampleFiles and
testfiles, but every selected case must still declare its output variables,
meters, evidence level, report artifacts, and gate policy.

Related policy documents:

- [ExampleFiles Coverage Plan](examplefiles-coverage-plan.md)
- [Case Tier Policy](case-tier-policy.md)
- [Output Variable Matrix](output-variable-matrix.md)
- [Report Format](report-format.md)

## Evidence Tiers

| Tier | Purpose | Default claim boundary |
|---|---|---|
| Tier A | small deterministic release-gate candidates | may become conformance only with tolerances, reports, and blocking gates |
| Tier B | nightly or scheduled diagnostics | diagnostic-only or baseline-only |
| Tier C | broad coverage expansion and complex systems | baseline-only by default |

## Domain Coverage

| Domain | Example focus | Earliest milestone |
|---|---|---|
| parser/schema | minimal IDF and epJSON fixtures | v0.1 |
| harness/reporting | patched output requests and oracle baselines | v0.2 |
| input interpretation | object/default/reference coverage | v0.3 |
| time/weather/schedule | weather and schedule time series | v0.4 |
| static geometry and gains | EIO geometry, materials, and nominal gains | v0.5 |
| output/trace/compare | selected outputs, deltas, divergence points, report index | v0.6 |
| heat balance | no-HVAC or HVAC-disabled uncontrolled cases | v0.8 |
| fenestration/radiation | simple window and shading cases | v0.9 |
| IdealLoads | thermostat and IdealLoads examples | v0.10 |
| air-side HVAC | selected fan/coil/PTAC cases | v0.11 |
| plant | selected boiler/chiller/pump cases | v0.12 |

Each tier must declare its variables and meters before it supports a claim.
Running an EnergyPlus example, producing finite samples, or matching sample
counts is not enough for conformance.
