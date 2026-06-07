---
status: active
claim_level: planning-guard
owner: conformance
last_reviewed: 2026-06-07
---

# Output Variable Source Map

Reference version: EnergyPlus 26.1.0

Purpose: map the first heat-balance candidate output variables to EnergyPlus
source files and Rust result locations before v0.8 conformance work begins.
Only `heat_balance_nomass_001` promotes a mapped variable in v0.8.

## Candidate Variables

| Variable | Frequency | EnergyPlus source | Rust source or target | Current level |
|---|---|---|---|---|
| `Zone Mean Air Temperature` | hourly | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | `ResultStore` series from heat-balance trace | conformance for `heat_balance_nomass_001`; diagnostic otherwise |
| `Zone Total Internal Convective Heating Rate` | hourly | `src/EnergyPlus/InternalHeatGains.cc` | `simulate_zone_internal_convective_gains` | smoke |
| `Zone Air Heat Balance Internal Convective Heat Gain Rate` | hourly | `src/EnergyPlus/ZoneTempPredictorCorrector.cc`; `src/EnergyPlus/InternalHeatGains.cc` | future `ep_runtime::zone_air` report state | mapped-not-ported |
| `Zone Air Heat Balance Surface Convection Rate` | hourly | `src/EnergyPlus/ZoneTempPredictorCorrector.cc`; `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | future surface convection sum | mapped-not-ported |
| `Zone Air Heat Balance Air Energy Storage Rate` | hourly | `src/EnergyPlus/ZoneTempPredictorCorrector.cc` | future zone air storage term | mapped-not-ported |
| `Surface Inside Face Temperature` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | future `SurfaceHeatBalanceState` | mapped-not-ported |
| `Surface Outside Face Temperature` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | future `SurfaceHeatBalanceState` | mapped-not-ported |
| `Surface Inside Face Conduction Heat Transfer Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | future opaque conduction result | mapped-not-ported |
| `Surface Outside Face Conduction Heat Transfer Rate` | hourly | `src/EnergyPlus/HeatBalanceSurfaceManager.cc` | future opaque conduction result | mapped-not-ported |
| `Site Outdoor Air Drybulb Temperature` | hourly | `src/EnergyPlus/WeatherManager.cc` | EPW weather trace | smoke |
| `Schedule Value` | hourly | output processor plus schedule managers | schedule trace | smoke |

## Registration Boundary

- EnergyPlus output variables are registered through `SetupOutputVariable`.
- Rust output variables must be declared in case manifests before comparison.
- Rust values must be written to `ResultStore` or a successor output store with
  key, variable, frequency, class, source, and timestamp semantics.
- A console-only value is not a v0.8 conformance variable.

## Promotion Requirements

A variable can move from `diagnostic-only` or `smoke` to `conformance` only
when all of these exist:

- case manifest with the requested output
- EnergyPlus baseline artifact containing the requested variable
- Rust result artifact for the same key, variable, and frequency
- timestamp alignment rule
- tolerance policy
- compare-summary row with first divergence information
- blocking release gate

## Explicit Non-Claims

The current `Zone Mean Air Temperature` diagnostic report has
`tolerance_policy: none` and `status: extracted`. It is useful for locating
deltas, but it is not a zone heat-balance conformance result.

The v0.8 `heat_balance_nomass_001` report is a separate conformance result for
hourly `Zone Mean Air Temperature` only. It requires a case manifest,
zone-state tolerance, markdown/JSON report artifacts, and a blocking gate.
