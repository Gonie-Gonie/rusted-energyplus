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
| outside face temperature | `CalcHeatBalanceOutsideSurf` in `HeatBalanceSurfaceManager.cc` | outside face state with weather, solar, exterior convection, and boundary conditions | roof/wall diagnostic uses weather/solar exterior forcing; floor/other and full DOE-2/radiation iteration remain partial |
| inside face temperature | `CalcHeatBalanceInsideSurf` in `HeatBalanceSurfaceManager.cc` | inside face state with zone air, convection, radiant exchange, and internal gains | CTF subset solver uses zone air, TARP convection, damping, and OtherEquipment radiant source slots; full radiation/internal source wiring remains partial |
| opaque conduction histories | `SurfCTFConstInPart`, `SurfCTFConstOutPart`, `SurfInsideFluxHist`, and `SurfOutsideFluxHist` in `HeatBalanceSurfaceManager.cc` | CTF coefficient and history state per opaque surface | EIO-seeded diagnostic histories exist; native mass-material coefficient generation and full iteration parity remain unported |
| adiabatic boundary | surface boundary condition handling | inside/outside equality for adiabatic no-mass cases | conformance for declared local case |
| interzone boundary | adjacent surface/zone lookup | resolved target surface and zone IDs | smoke-tested |
| reporting | `SetupOutputVariable` registration | `ResultStore` series per key/variable/frequency | official diagnostic now compares roof plus wall/floor decomposition series |
| inside/outside iteration | inside surface heat-balance iteration manager | repeat inside/outside face balance passes without advancing timestep histories more than once | default remains one pass; all-CTF surface-iter3 probe is diagnostic-only |

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
Official ExampleFile surface balances now have roof, wall, and floor
inside/outside face-temperature plus selected conduction deltas in the
warmup-aware diagnostic report, but they remain failing diagnostic candidates
until every declared hourly surface delta is below tolerance. Construction layer
stacks are preserved for future CTF work, but the runtime does not yet generate
or advance EnergyPlus CTF coefficient histories for mass-material constructions.
The diagnostic timestep path now feeds the existing roof/wall exterior
weather/solar balance into the CTF boundary driver, which improves wall, roof,
and MAT series while exposing that the zone opaque aggregate still depends on
unported floor mass CTF and full surface iteration parity. Exposed wet exterior
surfaces now follow the EnergyPlus rain branch in diagnostic form: liquid
precipitation uses the hourly interpolation threshold from `WeatherManager.cc`,
wet timesteps mix `SurfHConvExt = 1000.0` into the hourly report coefficient,
and the exterior convection reference temperature uses a bounded outdoor
wet-bulb approximation. Inside-surface radiant/source terms now have explicit
runtime slots matching EnergyPlus `SurfTempTerm` inputs, and the OtherEquipment
radiant fraction is distributed to inside surfaces with EnergyPlus inside-layer
area-absorptance normalization; outside-layer absorptance remains the exterior
solar/longwave input. Shortwave, additional source, HVAC radiant, and full
radiation coupling remain future wiring rather than promoted parity. A
source-anchored ScriptF interior-longwave probe matches the
`1ZoneUncontrolled` EIO factor orientation, but remains diagnostic-only because
exact longwave feedback without the rest of the EnergyPlus surface/zone coupling
regresses the active storage and aggregate rows. A diagnostic surface-iter3 lane
can repeat the inside/outside face balance within one zone timestep while
advancing CTF histories only once, so iteration sensitivity can be measured
before changing the default path. The compiler now preserves explicit
`SurfaceConvectionAlgorithm:Outside,DOE-2`, and the heat-balance shell uses that
setting for the default exterior coefficient path; the DOE-2 probe lanes remain
useful for isolating coefficient changes from the quick-conduction outside-face
branch.
