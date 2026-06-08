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
| outside face temperature | `CalcHeatBalanceOutsideSurf` in `HeatBalanceSurfaceManager.cc` | outside face state with weather, solar, exterior convection, and boundary conditions | roof diagnostic uses weather/solar exterior forcing; non-roof and full DOE-2/radiation iteration remain partial |
| inside face temperature | `CalcHeatBalanceInsideSurf` in `HeatBalanceSurfaceManager.cc` | inside face state with zone air, convection, radiant exchange, and internal gains | zone-temperature mirror only |
| opaque conduction histories | `SurfCTFConstInPart`, `SurfCTFConstOutPart`, `SurfInsideFluxHist`, and `SurfOutsideFluxHist` in `HeatBalanceSurfaceManager.cc` | CTF coefficient and history state per opaque surface | layer-stack input exists; CTF histories not ported |
| adiabatic boundary | surface boundary condition handling | inside/outside equality for adiabatic no-mass cases | conformance for declared local case |
| interzone boundary | adjacent surface/zone lookup | resolved target surface and zone IDs | smoke-tested |
| reporting | `SetupOutputVariable` registration | `ResultStore` series per key/variable/frequency | official diagnostic now compares selected roof inside/outside temperatures and conduction series |

## Promotion Requirements

- outside and inside face states must be separate computed states, not aliases,
  unless the EnergyPlus source path also implies equality for that boundary.
- surface orientation, area, construction layer, and boundary target must be
  validated before dynamic promotion.
- opaque conduction must use EnergyPlus CTF coefficients and history updates
  before official non-no-mass surface conduction variables can be promoted.
- dynamic official ExampleFile failures remain diagnostic until their deltas
  are below tolerance for every declared output.

## Current Boundary

No-mass adiabatic surface temperatures and zero-conduction series are promoted.
Official ExampleFile surface balances now have selected roof inside/outside
face-temperature and conduction deltas in the warmup-aware diagnostic report,
but they remain failing diagnostic candidates until every declared hourly
surface delta is below tolerance. Construction layer stacks are preserved for
future CTF work, but the runtime does not yet generate or advance EnergyPlus CTF
coefficient histories for mass-material constructions. The diagnostic timestep
path now feeds the existing roof exterior weather/solar balance into the CTF
boundary driver, which improves selected roof and MAT series while exposing that
the zone opaque aggregate still depends on unported floor mass CTF and full
surface iteration parity.
