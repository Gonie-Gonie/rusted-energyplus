---
status: active
claim_level: none
owner: core
last_reviewed: 2026-06-07
---

# v1 Scope

v1.0 is a substantial compatibility draft for EnergyPlus 26.1.0. It is not full
EnergyPlus compatibility, but it must be much more than a toy or a pair of
no-mass fixtures.

## Required Release Evidence

v1.0 requires:

- locked EnergyPlus 26.1.0 oracle and source reference.
- Tier A conformance cases, approximately 15 to 25.
- Tier B diagnostic or baseline cases, approximately 30 to 60.
- object coverage matrix.
- variable and meter coverage matrix.
- case coverage matrix.
- release-level conformance index.
- performance report.
- stability and failure-mode report.
- known gaps.
- no false-conformance guard.

## Candidate Families

Tier A candidates should come from small deterministic fixtures and selected
official EnergyPlus testfiles:

| Domain | Candidate families |
|---|---|
| envelope and zone | `1ZoneUncontrolled`, `1ZoneUncontrolled3SurfaceZone`, simple window variants, simple shading variants |
| controlled zone | selected thermostat and IdealLoads cases |
| air-side | selected PTAC/unitary case, selected `5ZoneAirCooled` subset, selected CAV/simple air-loop case |
| plant | pump plus load-profile fixture, purchased heating/cooling fixture, selected boiler case, selected chiller/cooling case if mature |
| meters | facility electricity, gas, heating, and cooling meters for declared systems |

## Allowed Claims

v1.0 may claim only:

- declared Tier A cases.
- declared variables and meters.
- declared tolerances.
- EnergyPlus 26.1.0 oracle.
- fixed known limitations.

## Not Allowed

v1.0 must not claim:

- full EnergyPlus compatibility.
- all IDF or epJSON compatibility.
- all HVAC compatibility.
- all plant compatibility.
- autosizing compatibility unless explicitly declared.
- EMS, PythonPlugin, AirflowNetwork, daylighting, or advanced reporting unless
  explicitly declared.

## Freeze Requirements

Before v1.0 RCs, the project must freeze:

- Tier A case list.
- Tier B diagnostic list.
- object coverage matrix.
- variable and meter coverage matrix.
- tolerance policy.
- excluded features.
- known gaps.
- performance benchmark cases and measurement conditions.
