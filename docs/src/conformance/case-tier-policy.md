---
status: active
claim_level: none
owner: conformance
last_reviewed: 2026-06-07
---

# Case Tier Policy

EnergyPlus ExampleFiles and testfiles must be introduced by tier. A larger IDF
does not create a stronger claim unless its requested variables, artifacts,
tolerances, and gate are declared.

| Tier | Use | Gate policy |
|---|---|---|
| Tier A | small deterministic release-gate candidates | may become blocking when reports and tolerances exist |
| Tier B | nightly or scheduled diagnostic cases | non-blocking unless explicitly promoted |
| Tier C | broad coverage expansion and complex systems | baseline-only or diagnostic-only by default |

## Tier A

Tier A cases should be small, deterministic, quick enough for local release
verification, and narrow enough that unsupported features can be listed without
ambiguity.

Candidate families:

- `1ZoneUncontrolled.idf`
- `1ZoneUncontrolled3SurfaceZone.idf`
- selected no-HVAC or HVAC-disabled one-zone cases

## Tier B

Tier B cases are useful for coverage pressure and nightly diagnostics. They may
contain windows, shading, simple HVAC, or multiple zones, but they remain
diagnostic until their object coverage and variable list are locked.

Candidate families:

- `1ZoneUncontrolled_win_1.idf`
- `1ZoneUncontrolled_win_2.idf`
- `4ZoneWithShading_Simple_*.idf`
- `5ZoneAirCooled.idf`

## Tier C

Tier C cases are broad exploration inputs. They can contain EMS, Python plugins,
AirflowNetwork, daylighting, complex HVAC, or plant systems. These cases should
not block releases until promoted to Tier A or Tier B with a reduced claim.

## Promotion Rule

A case can move to a stronger tier only when it has:

- stable case manifest
- declared output variables and meters
- EnergyPlus oracle baseline artifacts
- Rust artifacts, when claiming conformance
- compare summary and human-readable report
- explicit unsupported-feature list
- release or CI gate policy
