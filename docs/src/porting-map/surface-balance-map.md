---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-12
---

# Surface Inside and Outside Balance Map

Reference version: EnergyPlus 26.1.0

Purpose: record the minimum inside/outside surface balance detail required
before official ExampleFile surface temperatures can be promoted.

## Source Anchors

| Balance side | EnergyPlus source | Required Rust target | Current status |
|---|---|---|---|
| outside face temperature | `CalcHeatBalanceOutsideSurf` in `HeatBalanceSurfaceManager.cc` | outside face state with weather, solar, exterior convection, and boundary conditions | roof/wall diagnostic uses weather/solar exterior forcing plus DOE-2, terrain-adjusted surface wind speed, and sky/air/ground exterior longwave helpers; floor/other and full iteration remain partial |
| inside face temperature | `CalcHeatBalanceInsideSurf` in `HeatBalanceSurfaceManager.cc` | inside face state with zone air, convection, radiant exchange, and internal gains | CTF subset solver uses zone air, TARP convection, damping, and OtherEquipment radiant source slots; full radiation/internal source wiring remains partial |
| opaque conduction histories | `SurfCTFConstInPart`, `SurfCTFConstOutPart`, `SurfInsideFluxHist`, and `SurfOutsideFluxHist` in `HeatBalanceSurfaceManager.cc` | CTF coefficient and history state per opaque surface | EIO-seeded diagnostic histories exist; native mass-material coefficient generation and full iteration parity remain unported |
| adiabatic boundary | surface boundary condition handling | inside/outside equality for adiabatic no-mass cases | conformance for declared local case |
| ground boundary | `Site:GroundTemperature:BuildingSurface` defaulting and surface boundary handling | explicit ground temperature source before full ground-model objects | uses EnergyPlus default building-surface ground temperature of 18 C until `Site:GroundTemperature:*` object parsing is ported |
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
Official ExampleFile surface balances now also promote the named
`1ZoneUncontrolled` roof, wall, and floor inside/outside face-temperature plus
selected inside/outside conduction deltas in the compatibility-candidate
report. Floor `Surface Heat Storage Rate`, radiation, solar, convection
coefficient, and broader decomposition rows remain diagnostic-only.
Construction layer stacks are preserved for future native CTF work; the
candidate uses EIO-seeded CTF coefficients to keep coefficient mismatch outside
the promoted claim.
The diagnostic timestep path now feeds the existing roof/wall exterior
weather/solar balance into the CTF boundary driver, which improves wall, roof,
and MAT series while exposing that the zone opaque aggregate still depends on
unported floor mass CTF and full surface iteration parity. Exposed wet exterior
surfaces now follow the EnergyPlus rain branch in diagnostic form: liquid
precipitation uses the hourly interpolation threshold from `WeatherManager.cc`,
wet timesteps mix `SurfHConvExt = 1000.0` into the hourly report coefficient,
and the exterior convection reference temperature uses a bounded outdoor
wet-bulb approximation. The run-period and warmup shells now pass timestep
weather context for dry-bulb, rain, exterior wind speed/direction, and exterior
solar forcing, then average surface temperatures plus surface/zone
conduction/source rows over the zone timesteps for hourly diagnostics.
Exterior longwave follows the EnergyPlus
diagnostic split into `SurfHSkyExt`, `SurfHAirExt`, and `SurfHGrdExt`,
including `SurfAirSkyRadSplit`, and both the outside-face balance equivalent
radiation term and the net thermal radiation report row share that helper.
Exterior convection now uses the EnergyPlus `SetSurfaceWindSpeedAt` terrain and
surface-centroid profile before DOE-2/MoWITT forced-convection terms, so
diagnostic coefficients use timestep `SurfOutWindSpeed`-shaped local wind
instead of raw hourly EPW wind speed.
Inside-surface radiant/source terms now have explicit runtime slots matching
EnergyPlus `SurfTempTerm` inputs, and the OtherEquipment radiant fraction is
distributed to inside surfaces with EnergyPlus inside-layer area-absorptance
normalization; outside-layer absorptance remains the exterior solar/longwave
input. Shortwave, additional source, HVAC radiant, and full radiation coupling
remain future wiring rather than promoted parity. A
grey direct-exchange interior-longwave diagnostic uses EnergyPlus fixed surface
view factors before applying the grey-pair exchange emissivity, which pulls the
new latent floor inside longwave row much closer while leaving floor storage as
the active top bottleneck. The
source-anchored ScriptF interior-longwave probe matches the
`1ZoneUncontrolled` EIO final view-factor generation and factor orientation, but
remains diagnostic-only because exact longwave feedback without the rest of the
EnergyPlus surface/zone coupling regresses the active storage and aggregate
rows. A diagnostic surface-iter3 lane
can repeat the inside/outside face balance within one zone timestep while
advancing CTF histories only once, so iteration sensitivity can be measured
before changing the default path. The compiler now preserves explicit
`SurfaceConvectionAlgorithm:Outside,DOE-2`, and the heat-balance shell uses that
setting for the default exterior coefficient path; the DOE-2 probe lanes remain
useful for isolating coefficient changes from the quick-conduction outside-face
branch.

The June 2026 EnergyPlus source audit fixes the next floor CTF boundary. In
`CalcHeatBalanceInsideSurf2CTFOnly`, EnergyPlus preloads
`SurfTempOutHist` from `SurfOutsideTempHist(1)`, solves `SurfTempInTmp`, copies
it to `SurfTempIn`, writes `SurfTempOut` for reporting from
`SurfOutsideTempHist(1)`, and only then synchronizes interzone outside-history
slots from the paired inside history. `UpdateThermalHistories` later computes
current inside and outside fluxes from that current outside-history slot plus
current `SurfTempIn`, writes the conduction report variables, and then shifts
the CTF histories. The active floor-storage work should therefore keep report,
inside-CTF, and committed-history snapshots separate instead of replacing all
adiabatic or interzone outside states with the current inside face.
