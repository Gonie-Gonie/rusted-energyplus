---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-07
---

# Algorithm Porting Readiness

Purpose: define the gate that must pass before v0.8 heat-balance algorithm work
can be treated as conformance candidate work.

## Required Before Algorithm Porting

- reference source checkout available through `source-smoke`
- source files listed in `heat-balance-source-map.md`
- routine map for `ManageHeatBalance`, `InitHeatBalance`,
  `CalcHeatBalanceOutsideSurf`, `CalcHeatBalanceInsideSurf`,
  `ManageZoneAirUpdates`, and `correctZoneAirTemps`
- output variable map for first zone and surface heat-balance variables
- first case set identified:
  `heat_balance_uncontrolled_001`, `heat_balance_nomass_001`, and
  `heat_balance_mass_001`
- unsupported boundaries documented for windows, solar, infiltration, HVAC,
  warmup convergence, sizing periods, moisture, and plant

## Allowed In v0.7

- source-map documentation
- output-variable map documentation
- diagnostic-only report and trace improvements
- case design notes with no conformance claim
- release gates that fail when source-map documents are missing

## Not Allowed In v0.7

- heat-balance conformance claim
- zone-temperature pass wording
- tolerance-gated `Zone Mean Air Temperature` promotion
- surface heat-balance conformance claim
- HVAC, plant, solar, fenestration, warmup, or sizing-period conformance claim

## v0.8 Entry Rule

The first v0.8 heat-balance implementation PR must name:

- the EnergyPlus source file and routine being ported
- the Rust module or state field being changed
- the output variable affected
- the diagnostic or conformance case exercising the change
- the current evidence level

If any item is missing, the change stays diagnostic-only or is blocked from the
heat-balance conformance path.

## v0.8 Applied Entry

`heat_balance_nomass_001` is the first applied entry. It names:

- EnergyPlus source: `DataHeatBalance.hh`
- reference constants: `DataHeatBalance::ZoneInitialTemp` and
  `DataHeatBalance::SurfInitialTemp`
- Rust state: `HeatBalanceSimulationOptions` and `HeatBalanceState`
  initialization
- output variable: hourly `Zone Mean Air Temperature`
- evidence level: conformance for the declared no-mass adiabatic case only
