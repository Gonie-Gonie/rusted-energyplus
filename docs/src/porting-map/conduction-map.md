---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-08
---

# Conduction Map

Reference version: EnergyPlus 26.1.0

Purpose: separate the currently promoted no-mass conduction evidence from the
future official ExampleFile transient conduction work.

## Output Variables

| Variable | Current Rust source | Current claim | Official ExampleFile status |
|---|---|---|---|
| `Surface Inside Face Conduction Heat Transfer Rate` | steady `SurfaceHeatBalanceState` CTF inside flux shell | no-mass adiabatic conformance only | baseline + diagnostic candidate |
| `Surface Inside Face Conduction Heat Transfer Rate per Area` | rate divided by surface area | no-mass adiabatic conformance only | baseline candidate |
| `Surface Outside Face Conduction Heat Transfer Rate` | steady `SurfaceHeatBalanceState` CTF outside flux shell with EnergyPlus output sign | no-mass adiabatic conformance only | baseline candidate |
| `Surface Outside Face Conduction Heat Transfer Rate per Area` | outside rate divided by surface area | no-mass adiabatic conformance only | baseline candidate |
| `Zone Opaque Surface Inside Faces Conduction Rate` | sum of surface heat gain to zone | no-mass adiabatic conformance only | baseline + diagnostic candidate |

## Source Anchors

| EnergyPlus area | Source anchor | Rust target |
|---|---|---|
| CTF setup | construction/material CTF routines and `DataHeatBalance` histories | `SurfaceCtfState` coefficients and histories |
| inside conduction reporting | `HeatBalanceSurfaceManager.cc` output registration and update | `ResultStore` surface conduction series |
| outside conduction reporting | `HeatBalanceSurfaceManager.cc` output registration and update | outside face conduction series |
| zone opaque aggregate | advanced report variables for opaque surface sums | zone aggregate conduction series |

## Promotion Requirements

Official ExampleFile conduction cannot be promoted until the Rust side carries
the same transient conduction history semantics as the selected EnergyPlus
case. A zero no-mass adiabatic pass is useful but does not prove CTF parity.
The current runtime has per-surface CTF coefficient/history slots, advances CTF
history constants, and can seed CTF rows from EnergyPlus EIO output for
diagnostic isolation. The default official diagnostic path only seeds
steady/no-mass `#CTFs <= 1` rows while mass-material CTF rows are isolated from
the current simplified face-temperature/history shell; enabling mass CTF rows at
this stage over-amplifies latent floor history. The official dynamic diagnostic
JSON/markdown report records this as `ctf_seed_policy: steady-no-mass-only` and
lists skipped mass constructions such as `FLOOR (#CTFs=5)`. Developers can
temporarily set `RUSTED_ENERGYPLUS_HEAT_BALANCE_CTF_SEED_POLICY=all-eio` to
reproduce mass CTF over-amplification without changing the official diagnostic
default or making a conformance claim. Roof/wall exterior
weather/solar forcing now feeds the diagnostic CTF boundary driver for
run-period timesteps, and the official diagnostic manifest now includes
wall/floor surface
decomposition rows so aggregate cancellation does not hide the next bottleneck.
The aggregate zone conduction series remains blocked by unported mass-material
floor CTF histories and the full surface iteration order. Native
EnergyPlus-equivalent mass-material CTF coefficient generation, DOE-2 outside
convection, full inside-surface iteration order, and radiation coefficient
updates are still unported. The timestep shell now uses the EnergyPlus TARP
inside natural convection coefficient in the inside CTF balance.
`official_1zone_uncontrolled_dynamic_diagnostic_001` is the current failing
diagnostic gate for that promotion path.
