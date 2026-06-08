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
| `Surface Heat Storage Rate` | EnergyPlus-style `-(inside + outside)` storage report derived from surface conduction rates | diagnostic-only | official dynamic diagnostic candidate |
| `Zone Opaque Surface Inside Faces Conduction Rate` | sum of surface heat gain to zone | no-mass adiabatic conformance only | baseline + diagnostic candidate |

## Source Anchors

| EnergyPlus area | Source anchor | Rust target |
|---|---|---|
| CTF setup | construction/material CTF routines and `DataHeatBalance` histories | `SurfaceCtfState` coefficients and histories |
| inside conduction reporting | `HeatBalanceSurfaceManager.cc` output registration and update | `ResultStore` surface conduction series |
| outside conduction reporting | `HeatBalanceSurfaceManager.cc` output registration and update | outside face conduction series |
| surface storage reporting | `HeatBalanceSurfaceManager.cc` `SurfOpaqStorageCond = -(SurfOpaqInsFaceCond + SurfOpaqOutFaceCond)` | derived surface heat storage series |
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
default or making a conformance claim. The default diagnostic keeps floor inside
conduction as the top bottleneck because the adiabatic mass-material floor is
not seeded into the simplified CTF shell and therefore reports zero inside
conduction in the Rust lane, while the all-CTF probe moves the top
bottleneck to zone air heat-balance storage/convection and worsens the zone
aggregate conduction row, confirming the current blocker is the mass CTF
face/history coupling rather than EIO coefficient availability. The
all-CTF-plus-20-day-warmup probe shows only negligible movement from the
all-CTF lane, so the remaining floor delta is not explained by Rust's early
warmup convergence alone. The all-CTF surface-iter3 probe lowers the
zone-air storage/convection bottleneck relative to all-CTF, but it does not
improve the floor conduction row, so iteration sensitivity and mass-floor CTF
history parity remain separate work items. Ad hoc iter5/iter8 runs lowered some
peak errors but raised the top RMSE relative to iter3, so iter3 is the current
tracked probe count rather than a promotion setting. A steady/no-mass default
surface-iter3 trial regressed the analytical zone-air storage guard, so surface
iteration is kept as an all-CTF diagnostic probe instead of a default setting.
The analytical surface-first zone-air probe improves MAT and inside-face
temperature RMSE, but leaves the top floor conduction bottleneck unchanged and
raises the aggregate conduction RMSE, so call-order progress still needs to be
paired with mass-floor CTF/history parity before promotion. Combining all-CTF
seeding with analytical surface-first correction lowers MAT, floor inside
conduction, and zone aggregate conduction RMSE compared with either isolated
probe, while keeping floor inside conduction as the top bottleneck. That
combined lane confirms the next promotion blocker is not coefficient
availability alone or zone-air call order alone, but the mass-floor
face/history coupling that remains after those two probes are joined.
Adding three surface-balance passes on top of that combined lane lowers the
floor inside/outside conduction and zone aggregate conduction RMSE further,
while slightly worsening MAT, so surface iteration is a real conduction lever
but still has to be paired with the zone-air correction order before promotion.
The all-CTF analytical coupled probe applies one same-timestep surface rebalance
after the analytical MAT correction. It lowers aggregate and floor conduction
relative to the combined surface-first lane, but not as far as the tracked
iter3 lane, and it slightly worsens MAT/air-storage relative to surface-first.
That keeps the active blocker on coherent surface iteration plus zone-air
correction/history semantics rather than a one-pass feedback loop alone. When
that coupled rebalance is paired with three surface passes, the probe becomes
the current best lane for zone aggregate conduction and the latent zone-air
surface-convection/storage rows. A follow-on previous-inside outdoor boundary
probe slightly improves floor inside/outside temperatures and conduction
(`923.733908` inside conduction RMSE and `507.588138` outside conduction RMSE),
and lowers the newly tracked floor heat-storage RMSE from `2725.712393` in the
default lane to `1422.193225`, but it does not beat the coupled iter3 lane for
zone aggregate conduction or air-balance rates. The floor storage row becomes
the top diagnostic bottleneck once it is visible. MAT still stays best in the
one-pass all-CTF analytical surface-first lane, so promotion still needs the
EnergyPlus outside-surface quick-conduction/source coupling, history commit,
and predictor/corrector order rather than simply enabling the
floor-conduction-best lane. Extending the previous-inside solve to adiabatic
boundaries nudges floor inside temperature and inside conduction slightly lower
(`923.728787` inside conduction RMSE), but it does not improve floor storage
(`1422.231349` RMSE versus `1422.193225`) or zone aggregate conduction, so the
adiabatic boundary probe remains a diagnostic fork rather than the active best
lane.
Roof/wall exterior weather/solar forcing now feeds the diagnostic CTF
boundary driver for run-period timesteps, and the official diagnostic manifest
now includes wall/floor surface decomposition rows, including floor
outside-face conduction, per-area floor conduction, and floor heat-storage
diagnostics, so aggregate cancellation does not hide the next bottleneck.
The aggregate zone conduction series remains blocked by unported mass-material
floor CTF histories and the full surface iteration order. Native
EnergyPlus-equivalent mass-material CTF coefficient generation, DOE-2 outside
convection, full inside-surface iteration order, and radiation coefficient
updates are still unported. The timestep shell now uses the EnergyPlus TARP
inside natural convection coefficient in the inside CTF balance and preserves
the previous inside face temperature for the EnergyPlus-style iterative damping
term before the zone-air predictor overwrites current face estimates.
`official_1zone_uncontrolled_dynamic_diagnostic_001` is the current failing
diagnostic gate for that promotion path.
