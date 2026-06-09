---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-06-09
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
| `Surface Heat Storage Rate per Area` | storage rate divided by surface area | diagnostic-only | official dynamic diagnostic candidate |
| `Zone Opaque Surface Inside Faces Conduction Rate` | sum of surface heat gain to zone | no-mass adiabatic conformance only | baseline + diagnostic candidate |
| `Zone Opaque Surface Outside Faces Conduction Rate` | sum of surface outside-face conduction rates | diagnostic-only | official dynamic diagnostic candidate |

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
zone aggregate conduction or air-balance rates. Extending that lane with the
EnergyPlus quick-conduction outside-face solve (`CTFCross[0] > 0.01`) and
precomputed CTF history constants moves the active best lane again: floor
inside conduction drops to RMSE `812.566220`, floor outside conduction to
`397.351373`, floor heat storage to `1198.781640`, and zone aggregate
conduction to `84.217233`. Raising that same lane to five surface passes
lowers the active floor/aggregate bottlenecks further (`800.087434` floor
inside conduction RMSE, `386.128809` floor outside conduction RMSE,
`1174.412273` floor heat-storage RMSE, and `78.393234` zone aggregate
conduction RMSE), while an ad hoc eight-pass run lowers storage a little more
but gives back zone aggregate conduction and MAT. The floor storage row remains
the top balanced-lane diagnostic bottleneck once it is visible. DOE-2 exterior
convection and grey interior-longwave forks lower the floor storage row further
(`752.765953` and `579.551277` RMSE respectively). Combining the two forks
pushes MAT to RMSE `0.972533`, floor inside conduction to `293.417817`, and
floor storage to `575.885599`, but still gives back floor outside conduction
(`423.487145` RMSE), storage max-abs (`8287.121494`), and zone aggregate
conduction (`104.246599` RMSE). Promotion still needs the remaining EnergyPlus
radiation/coefficient coupling, source coupling, predictor/corrector order, and
CTF history commit parity rather than simply enabling the floor-conduction-best
lane. Extending the previous-inside solve
to adiabatic boundaries nudges floor inside temperature and inside conduction
slightly lower (`923.728787` inside conduction RMSE), but it does not improve
floor storage (`1422.231349` RMSE versus `1422.193225`) or zone aggregate
conduction, so the adiabatic boundary probe remains a diagnostic fork rather
than the active best lane. The interleaved grey interior-longwave lane later
made the floor outside flux/reporting order explicit: a per-pass adiabatic
previous-inside toggle stayed below the diagnostic movement threshold, but
freezing the adiabatic outside-face CTF balance at the timestep-start inside
temperature across interleaved passes matches EnergyPlus'
outside-balance-before-inside-loop reporting order and lowers floor outside
conduction from `399.588084` to `50.562260` RMSE, floor storage from
`369.424200` to `119.606076`, and zone outside aggregate conduction from
`328.987074` to `155.538581`. The remaining conduction target is now the
exterior source coupling behind roof convection/radiation and the zone outside
aggregate, not adiabatic floor flux parity alone. With inside longwave and
convection latent diagnostics exposed, the active grey interior-longwave path
now uses EnergyPlus fixed direct surface view factors; this reduces the
previously hidden floor inside longwave/convection bottleneck but leaves floor
storage and inside/outside floor conduction as the next conduction-facing rows.
Follow-up probes after that change keep the blocker on source/coupling order:
forcing Rust warmup to the EnergyPlus 20-day run-period count barely moves the
active top rows, and switching the active interleaved grey-longwave lane to an
EnergyPlus InitHeatBalance-shaped initial CTF seed is numerically identical
after warmup. On the non-active quick-outside epseed lane, that seed improves
first-sample floor history terms but regresses coupled floor
longwave/convection and zone-air rows. A non-interleaved grey-longwave
twenty-pass probe also reduces first-sample floor history deltas but regresses
floor storage and outside conduction, so the best active direction remains
interleaved surface/zone coupling with a narrower CTF history/source update
fix. The expanded first-sample CTF delta table now shows that the active
mass-floor current zero-term deltas are larger than the history deltas, so the
next conduction target includes face-temperature/current-term alignment rather
than history-vector contents alone. A
source recheck of EnergyPlus 26.1.0 `UpdateThermalHistories` also rules out an
outside-face report sign flip as the next correction: EnergyPlus computes
current `Qout` into `SurfOutsideFluxHist(1)`, reports
`SurfOpaqOutFaceCondFlux = -SurfOutsideFluxHist(1)`, and derives storage as
`-(SurfOpaqInsFaceCond + SurfOpaqOutFaceCond)`. Rust's report helpers now have
a unit guard for that sign/storage convention. The active all-CTF interleaved
grey-longwave lane still puts the maximum floor storage/conduction delta at
run-period sample 0 (`813.384496 W` storage, `465.262159 W` inside conduction,
and `348.122337 W` outside conduction), so the next CTF-facing blocker is the
warmup-to-run-period history handoff plus coupled source/history update order,
not another report-sign adjustment. Raising that same active lane to 100
surface iterations does not reduce the bottleneck, moving floor storage RMSE
from `108.672323` to `108.676973`, so the next correction is not a larger
fixed iteration count. The compact digest now also ranks first reported sample
bottlenecks; the active lane's first sample has `871.308445 W` zone outside
opaque aggregate delta, `813.384495 W` floor storage delta, and
`404.796794 W` floor inside net-longwave delta. The digest also records
Rust-only first-sample CTF component rows. In the active lane,
`ZN001:FLR001` reports `430.781218 W` inside conduction from
`12.337332 W` current outside-temperature, `-810.853101 W` current
inside-temperature, and `1229.296987 W` history terms, plus
`317.019053 W` outside conduction from `-990.426013 W` current
outside-temperature, `10.100465 W` current inside-temperature, and
`1297.344600 W` history terms. No-mass roof/wall component rows have zero
history terms and inside/outside conduction cancellation at the first sample,
so the remaining first-sample storage blocker is concentrated in mass-floor
history/source handoff rather than in every surface's current-temperature
terms. The new oracle-inferred history delta table makes that mismatch direct:
using the oracle first-sample floor temperatures and conduction rates with the
EIO zero CTF coefficients gives `-650.814857 W` inside history and
`-471.682586 W` outside history, while Rust carries `1229.296987 W` and
`1297.344600 W`, leaving `1880.111844 W` inside and `1769.027186 W` outside
history-term deltas. A
full ScriptF interior-longwave source is also not a promotion shortcut in the
current shell: even a one-pass ScriptF lane diverges to multi-kW floor storage
and roof inside-longwave errors, so ScriptF parity has to be paired with the
full EnergyPlus inside-surface iteration/convergence and source update order.
Adding the EnergyPlus advanced outside-face zone aggregate as a latent
diagnostic row exposed outside aggregate conduction as a second default bottleneck
(`2024.075950` RMSE) and the top current quick-boundary bottleneck:
quick-outside iter3 now honors the source-declared DOE-2 outside convection
setting and lowers it to `799.673332`, matching the explicit quick-outside
plus DOE-2 iter3 lane. This ties the remaining floor storage and
outside-face aggregate errors to the exterior face/source/history path rather
than only the inside aggregate cancellation row.
Roof/wall exterior weather/solar forcing now feeds the diagnostic CTF
boundary driver for run-period timesteps, and the official diagnostic manifest
now includes wall/floor surface decomposition rows, including floor
outside-face conduction, per-area floor conduction, and floor heat-storage
diagnostics, plus wall/roof outside convection/radiation/solar source rows,
raising the tracked official dynamic series count to 99 after the floor
storage per-area row was added, so aggregate cancellation does not hide the
next bottleneck. The
dynamic probe summary now ranks each lane's top inside-face and outside-face
conduction driver surfaces and records the best lane per surface. It also pairs
surface heat-storage/conduction RMSE with inside/outside face-temperature RMSE
and reports the W-rate RMSE per C of face-temperature RMSE, keeping aggregate
conduction regressions and CTF amplification tied to wall/floor/roof source rows
before a runtime change is promoted.
Porting the EnergyPlus Perez anisotropic sky diffuse multiplier for exterior
incident solar then moves the active quick-boundary lane away from thousand-Watt
wall solar source errors. Aligning solar position and shadowing-period
coefficients to EnergyPlus' non-leap weather ordinal for TMY records removes
the source-year leap-day drift visible in late-year roof/wall solar samples,
and splitting the exterior solar calculation so beam uses the
shadowing-period table while Perez sky diffuse and ground reflection use the
current timestep `SOLCOS` lowers the wall incident solar per-area RMSE to
`0.126385`, `0.567803`, `0.242010`, and `0.369024` W/m2 for WALL001 through
WALL004. Beam incident RMSE remains near zero and ground diffuse is now
zero-delta in the active lane; the remaining sub-W/m2 incident residual is in
the sky diffuse component. The leading residuals have moved to outside-face
aggregate conduction (`84.810714` RMSE), inside-face aggregate conduction
(`65.975683` RMSE), and the mass floor/history coupling path (`56.528269`
RMSE floor storage). The official dynamic manifest now also compares beam,
sky diffuse, and ground diffuse exterior incident solar component rows so
future solar fixes can isolate which source term moved.
The manifest now also includes the floor `Surface Heat Storage Rate per Area`
row so the leading storage bottleneck is visible in both whole-surface and
area-normalized form.
The aggregate zone conduction series remains blocked by unported mass-material
floor CTF histories and the full surface iteration order. Native
EnergyPlus-equivalent mass-material CTF coefficient generation, full
inside-surface iteration order, exterior radiation coupling, and radiation
coefficient updates are still unported. The timestep shell now uses the
EnergyPlus TARP inside natural convection coefficient in the inside CTF balance,
preserves the previous inside face temperature for the EnergyPlus-style
iterative damping term before the zone-air predictor overwrites current face
estimates, and honors explicit `SurfaceConvectionAlgorithm:Outside,DOE-2` for
the exterior convection coefficient. A current-inside adiabatic-history commit
probe was kept as diagnostic evidence but rejected as a promotion candidate:
in the active third-order/weather-storage/balance-surfconv lane it worsens
floor storage RMSE from `54.593582` to `453.783584` and floor outside
conduction RMSE from `23.282797` to `446.456057`, so the floor path should next
target full EnergyPlus surface/air iteration cadence rather than a one-point
self-adiabatic outside history overwrite.
A timestep-start reference-air surface-solve probe was also kept as diagnostic
evidence. It is a better candidate than the adiabatic-history overwrite for the
current cadence question: MAT RMSE improves from `0.069817` to `0.031508`, floor
face-temperature RMSE drops from about `0.0534` to `0.0322`, aggregate
inside-face conduction falls from `43.069343` to `27.427925`, and floor storage
edges down from `54.593582` to `54.561792`. It is still not promotion-ready
because aggregate outside-face conduction worsens from `20.119228` to
`29.132671`, floor inside-face longwave worsens from `16.615214` to
`31.074699`, and the zone-air latent rows regress. The next conduction step
should therefore keep the frozen reference-air clue but pair it with full
inside longwave/source-order and outside aggregate reporting parity work.
A current-pass interior longwave sibling of the frozen reference-air probe was
then tested against the same annual diagnostic. It is numerically almost
neutral: floor storage improves only from `54.561792` to `54.558577`, floor
inside conduction from `31.673961` to `31.672094`, and outside aggregate
conduction from `29.132671` to `29.131216`, while the zone-air latent rows stay
regressed. That rules out longwave sampling cadence alone as the next
conduction promotion lever.
Applying EnergyPlus' `MaxAllowedDelTemp = 0.002 C` inside-surface convergence
cutoff to that lane is more useful: floor storage drops to `52.022146`, floor
inside/outside conduction to `30.201354`/`21.976058`, and outside aggregate
conduction to `27.990507`. This suggests the active floor conduction path was
slightly over-iterated by a fixed twenty-pass loop. The remaining blocker is
still source/history parity because the first-sample floor storage max-abs is
about `701.319969` and the zone-air latent rows regress slightly.
`official_1zone_uncontrolled_dynamic_diagnostic_001` is the current failing
diagnostic gate for that promotion path.
