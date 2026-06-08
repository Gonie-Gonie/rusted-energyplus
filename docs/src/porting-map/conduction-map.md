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
| CTF setup | construction/material CTF routines and `DataHeatBalance` histories | future construction transfer function state |
| inside conduction reporting | `HeatBalanceSurfaceManager.cc` output registration and update | `ResultStore` surface conduction series |
| outside conduction reporting | `HeatBalanceSurfaceManager.cc` output registration and update | outside face conduction series |
| zone opaque aggregate | advanced report variables for opaque surface sums | zone aggregate conduction series |

## Promotion Requirements

Official ExampleFile conduction cannot be promoted until the Rust side carries
the same transient conduction history semantics as the selected EnergyPlus
case. A zero no-mass adiabatic pass is useful but does not prove CTF parity.
The current runtime has steady CTF zero-term slots for opaque surfaces;
EnergyPlus mass-material CTF coefficients and history constants are not yet
advanced.
`official_1zone_uncontrolled_dynamic_diagnostic_001` is the current failing
diagnostic gate for that promotion path.
