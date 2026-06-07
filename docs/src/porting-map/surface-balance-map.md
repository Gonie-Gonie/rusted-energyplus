---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-08
---

# Surface Inside and Outside Balance Map

Reference version: EnergyPlus 26.1.0

Purpose: record the minimum inside/outside surface balance detail required
before official ExampleFile surface temperatures can be promoted.

## Source Anchors

| Balance side | EnergyPlus source | Required Rust target | Current status |
|---|---|---|---|
| outside face temperature | `CalcHeatBalanceOutsideSurf` in `HeatBalanceSurfaceManager.cc` | outside face state with weather, solar, exterior convection, and boundary conditions | dry-bulb/boundary shell only |
| inside face temperature | `CalcHeatBalanceInsideSurf` in `HeatBalanceSurfaceManager.cc` | inside face state with zone air, convection, radiant exchange, and internal gains | zone-temperature mirror only |
| adiabatic boundary | surface boundary condition handling | inside/outside equality for adiabatic no-mass cases | conformance for declared local case |
| interzone boundary | adjacent surface/zone lookup | resolved target surface and zone IDs | smoke-tested |
| reporting | `SetupOutputVariable` registration | `ResultStore` series per key/variable/frequency | official diagnostic now compares selected roof inside/outside temperatures and conduction series |

## Promotion Requirements

- outside and inside face states must be separate computed states, not aliases,
  unless the EnergyPlus source path also implies equality for that boundary.
- surface orientation, area, construction layer, and boundary target must be
  validated before dynamic promotion.
- dynamic official ExampleFile failures remain diagnostic until their deltas
  are below tolerance for every declared output.

## Current Boundary

No-mass adiabatic surface temperatures and zero-conduction series are promoted.
Official ExampleFile surface balances now have selected roof inside/outside
face-temperature and conduction deltas in the warmup-aware diagnostic report,
but they remain failing diagnostic candidates until every declared hourly
surface delta is below tolerance.
