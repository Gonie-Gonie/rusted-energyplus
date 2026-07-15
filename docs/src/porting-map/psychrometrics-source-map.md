---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-07-15
---

# Psychrometrics Source Map

Reference version: EnergyPlus 26.1.0

Reference source root:

```text
.reference/energyplus-src/26.1.0/
```

This inventory pins the public, namespace-level psychrometric free-function
interface before implementation work is promoted. It is a source-order
planning guard, not an implementation, numerical-equivalence, or conformance
claim.

## Inventory Rule

The inventory was read directly from
`src/EnergyPlus/Psychrometrics.hh` in declaration order and checked against
`src/EnergyPlus/Psychrometrics.cc`. Overloads and the cache/no-cache
declarations of the same C++ identifier are one logical ticket. Error helpers
guarded by `EP_psych_errors`, raw cache delegates, lifecycle/reporting
functions, and the broken-line `F7` declaration are included. This produces
exactly **53 logical identifiers**.

Line numbers below refer to the pinned EnergyPlus 26.1.0 tree. Owner names
only an existing partial analogue or the intended Rust module; it does not say
that the EnergyPlus routine has been ported.

## Source-Interface Order

| Order | Logical identifier | Category | Source interface / implementation | Compile condition or overload | Current Rust owner | Test obligation |
|---:|---|---|---|---|---|---|
| 1 | `InitializePsychRoutines` | cache lifecycle | `Psychrometrics.hh:498`; `Psychrometrics.cc:110` | always declared; initializes each cache enabled by its `EP_cache_*` macro | intended `ep_runtime::psychrometrics` cache owner; unassigned | prove fresh-state initialization and independent initialization of all four cache families |
| 2 | `ShowPsychrometricSummary` | statistics/reporting | `Psychrometrics.hh:500`; `Psychrometrics.cc:136` | always present; summary body is active only with `EP_psych_stats` | intended diagnostics owner; unassigned | prove stats-enabled totals/hit counts and stats-disabled no-op behavior |
| 3 | `PsyRhoAirFnPbTdbW_error` | diagnostics | `Psychrometrics.hh:503`; `Psychrometrics.cc:198` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | prove the exact negative-density trigger and immediate severe/continue/timestamp/fatal message flow, including caller versus unknown context |
| 4 | `PsyRhoAirFnPbTdbW` | moist-air density | `Psychrometrics.hh:513,549` (inline implementations) | two always-present stateful/stateless overloads; one logical ticket | partial analogue: `ep_runtime::psychrometrics::energyplus_moist_air_density_kg_per_m3` | compare both overloads over source vectors, humidity floor behavior, diagnostics path, and mutual equivalence where domains overlap |
| 5 | `PsyRhoAirFnPbTdbW_fast` | moist-air density fast path | `Psychrometrics.hh:576` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | compare with the ordinary density routine and lock the fast-path input-domain preconditions |
| 6 | `PsyHfgAirFnWTdb` | latent enthalpy | `Psychrometrics.hh:593` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | coefficient-vector and temperature/humidity boundary parity |
| 7 | `PsyHgAirFnWTdb` | water-vapor gas enthalpy | `Psychrometrics.hh:623` (inline) | always present | partial analogue: `ep_runtime::psychrometrics::energyplus_water_vapor_gas_enthalpy_j_per_kg` | source-vector parity including ignored-`W` semantics and temperature limits |
| 8 | `PsyHFnTdbW` | moist-air enthalpy | `Psychrometrics.hh:648` (inline) | always present | partial analogue: `ep_runtime::ideal_loads::calc::psychrometrics::moist_air_enthalpy_j_per_kg` | coefficient-vector parity, humidity floor/domain behavior, and inverse round trips |
| 9 | `PsyHFnTdbW_fast` | moist-air enthalpy fast path | `Psychrometrics.hh:668` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | ordinary/fast equivalence across the documented valid domain |
| 10 | `PsyCpAirFnW` | moist-air specific heat | `Psychrometrics.hh:679` (inline) | always present; owns a function-local last-input cache | partial analogue: `ep_runtime::psychrometrics::energyplus_moist_air_specific_heat_j_per_kg_k` | source-vector parity plus repeated, alternating, and multistate cache-isolation probes |
| 11 | `PsyCpAirFnW_fast` | specific-heat fast path | `Psychrometrics.hh:718` (inline) | always present; owns a function-local last-input cache | intended `ep_runtime::psychrometrics`; unassigned | ordinary/fast equivalence and cache-hit/miss independence |
| 12 | `PsyTdbFnHW` | dry-bulb inversion | `Psychrometrics.hh:743` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | enthalpy round trips, denominator edge cases, and source-vector parity |
| 13 | `PsyRhovFnTdbRhLBnd0C` | vapor density | `Psychrometrics.hh:764` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | lower-bound-at-0-C branch, RH limits, and pressure-independent source vectors |
| 14 | `PsyRhovFnTdbWPb` | vapor density | `Psychrometrics.hh:789` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | humidity-ratio/pressure vectors, clamp rules, and inverse RH consistency |
| 15 | `PsyRhovFnTdbWPb_fast` | vapor-density fast path | `Psychrometrics.hh:815` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | ordinary/fast equivalence and valid-domain boundaries |
| 16 | `PsyRhFnTdbRhovLBnd0C_error` | diagnostics | `Psychrometrics.hh:826`; `Psychrometrics.cc:222` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | exact out-of-range trigger, caller text, recurrence suppression, and error-state mutation |
| 17 | `PsyRhFnTdbRhovLBnd0C` | relative humidity | `Psychrometrics.hh:835` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | 0-C lower-bound branch, clamping/error thresholds, and vapor-density round trips |
| 18 | `PsyTwbFnTdbWPb` | wet-bulb solve and cache | `Psychrometrics.hh:879,895`; `Psychrometrics.cc:281,356` | cached and no-cache same-name variants selected by `EP_cache_PsyTwbFnTdbWPb`; one logical ticket | partial analogue: `ep_runtime::psychrometrics::energyplus_outdoor_wet_bulb_c` | cached/no-cache/raw agreement, iteration convergence/failure, cache-key quantization, and caller-context behavior |
| 19 | `PsyTwbFnTdbWPb_raw` | wet-bulb raw solve | `Psychrometrics.hh:886`; `Psychrometrics.cc:347` | exists only with `EP_cache_PsyTwbFnTdbWPb` | partial analogue: `ep_runtime::psychrometrics::energyplus_outdoor_wet_bulb_c` | direct raw-vector parity and identity with cache misses |
| 20 | `PsyVFnTdbWPb_error` | diagnostics | `Psychrometrics.hh:905`; `Psychrometrics.cc:569` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | exact invalid-volume trigger, caller text, recurrence suppression, and error-state mutation |
| 21 | `PsyVFnTdbWPb` | moist-air specific volume | `Psychrometrics.hh:914` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | source vectors, invalid-result fallback, and density reciprocal relationship within source tolerances |
| 22 | `PsyWFnTdbH_error` | diagnostics | `Psychrometrics.hh:954`; `Psychrometrics.cc:606` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | negative-humidity trigger, corrected value, recurrence suppression, and caller context |
| 23 | `PsyWFnTdbH` | humidity-ratio inversion | `Psychrometrics.hh:962` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | enthalpy round trips, humidity floor/correction branches, and source vectors |
| 24 | `PsyPsatFnTemp_raw` | saturation pressure raw path | `Psychrometrics.hh:1002`; `Psychrometrics.cc:642` | exists only with `EP_cache_PsyPsatFnTemp`; internal formula selects non-IF97 versus `EP_IF97` branch | partial analogue: private `energyplus_psychrometric_saturation_pressure_pa` | raw branch vectors across ice/water boundaries, range guards, and both IF97 compile branches |
| 25 | `PsyPsatFnTemp` | saturation pressure and cache | `Psychrometrics.hh:1016,1066`; cached inline in header, no-cache implementation `Psychrometrics.cc:649` | variants selected by `EP_cache_PsyPsatFnTemp`; one logical ticket | partial analogue: private saturation-pressure helper and cache-temperature quantizer | cached/no-cache/raw agreement, cache-key quantization/collisions, range guards, and repeated-call stability |
| 26 | `PsyTsatFnHPb_raw` | saturation temperature from enthalpy/pressure raw path | `Psychrometrics.hh:1074`; `Psychrometrics.cc:900` | exists only with `EP_cache_PsyTsatFnHPb` | intended `ep_runtime::psychrometrics`; unassigned | raw inversion vectors, convergence/limits, and identity with cache misses |
| 27 | `PsyTsatFnHPb` | saturation temperature from enthalpy/pressure and cache | `Psychrometrics.hh:1079,1123`; cached inline in header, no-cache implementation `Psychrometrics.cc:906` | variants selected by `EP_cache_PsyTsatFnHPb`; one logical ticket | intended `ep_runtime::psychrometrics`; unassigned | cached/no-cache/raw agreement, two-input cache key, convergence, and boundary vectors |
| 28 | `PsyRhovFnTdbRh` | vapor density | `Psychrometrics.hh:1131` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | temperature/RH vectors, physical-domain limits, and reciprocal RH checks |
| 29 | `PsyRhFnTdbRhov_error` | diagnostics | `Psychrometrics.hh:1161`; `Psychrometrics.cc:1075` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | exact RH-bound trigger, caller text, recurrence suppression, and error-state mutation |
| 30 | `PsyRhFnTdbRhov` | relative humidity | `Psychrometrics.hh:1169` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | vapor-density round trips, clamp/error thresholds, and temperature extremes |
| 31 | `PsyRhFnTdbWPb_error` | diagnostics | `Psychrometrics.hh:1215`; `Psychrometrics.cc:1133` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | exact RH-bound trigger, caller context, recurrence suppression, and corrected return path |
| 32 | `PsyRhFnTdbWPb` | relative humidity | `Psychrometrics.hh:1223` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | humidity/pressure vectors, clamp/error thresholds, and humidity-ratio round trips |
| 33 | `PsyWFnTdpPb_error` | diagnostics | `Psychrometrics.hh:1272`; `Psychrometrics.cc:1191` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | pressure-crossing correction loop, exact trigger, caller context, and recurrence suppression |
| 34 | `PsyWFnTdpPb` | humidity ratio from dew point | `Psychrometrics.hh:1281` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | dew-point/pressure vectors, pressure-crossing correction, and dew-point round trips |
| 35 | `PsyWFnTdbRhPb_error` | diagnostics | `Psychrometrics.hh:1333`; `Psychrometrics.cc:1228` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | negative-humidity trigger, caller context, recurrence suppression, and denominator-approach diagnostics |
| 36 | `PsyWFnTdbRhPb` | humidity ratio from RH | `Psychrometrics.hh:1342` (inline) | always present | partial analogue: `ep_runtime::psychrometrics::energyplus_psychrometric_humidity_ratio_from_rh` | source vectors, 1000-Pa denominator floor, 1e-5 humidity floor, and inverse RH checks |
| 37 | `PsyWFnTdbTwbPb_temperature_error` | diagnostics | `Psychrometrics.hh:1391`; `Psychrometrics.cc:786` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | wet-bulb-above-dry-bulb threshold, clamp, caller text, and recurrence suppression |
| 38 | `PsyWFnTdbTwbPb_humidity_error` | diagnostics | `Psychrometrics.hh:1398`; `Psychrometrics.cc:822` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | negative-humidity trigger, RH fallback path, caller text, and recurrence suppression |
| 39 | `PsyWFnTdbTwbPb` | humidity ratio from wet bulb | `Psychrometrics.hh:1408` (inline) | always present | partial analogue: private `energyplus_psychrometric_humidity_ratio_from_wet_bulb_guess` | formula vectors, wet-bulb clamp, negative-humidity fallback, and wet-bulb round trips |
| 40 | `PsyHFnTdbRhPb` | enthalpy from RH | `Psychrometrics.hh:1462` (inline) | always present | composable from partial Rust analogues; no direct owner | composed source-vector parity and equality with `PsyWFnTdbRhPb` then `PsyHFnTdbW` |
| 41 | `PsyTsatFnPb_raw` | saturation temperature from pressure raw path | `Psychrometrics.hh:1490`; `Psychrometrics.cc:1266` | exists only with `EP_cache_PsyTsatFnPb` | intended `ep_runtime::psychrometrics`; unassigned | raw inversion vectors, convergence/range guards, and identity with cache misses |
| 42 | `PsyTsatFnPb` | saturation temperature from pressure and cache/interpolation | `Psychrometrics.hh:1495,1523`; cached inline in header, no-cache implementation `Psychrometrics.cc:1272` | variants selected by `EP_cache_PsyTsatFnPb`; one logical ticket | intended `ep_runtime::psychrometrics`; unassigned | cached/no-cache/raw agreement, interpolation toggle, key quantization, saved-state behavior, and limits |
| 43 | `PsyTdpFnWPb` | dew point from humidity ratio | `Psychrometrics.hh:1529` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | humidity floor, pressure vectors, saturation-temperature dependency, and round trips |
| 44 | `PsyTdpFnTdbTwbPb_error` | diagnostics | `Psychrometrics.hh:1556`; `Psychrometrics.cc:861` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | dew-point-above-wet-bulb threshold, clamp, caller text, and recurrence suppression |
| 45 | `PsyTdpFnTdbTwbPb` | dew point from dry/wet bulb | `Psychrometrics.hh:1566` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | composed humidity/dew-point vectors, 1e-5 floor, and wet-bulb upper clamp |
| 46 | `F6` | polynomial helper | `Psychrometrics.hh:1600` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | Horner-order coefficient vectors, signs, zeroes, and floating-point evaluation order |
| 47 | `F7` | scaled polynomial helper | `Psychrometrics.hh:1605` (broken-line inline declaration) | always present | intended `ep_runtime::psychrometrics`; unassigned | Horner-order coefficient vectors and exact final `1.0E10` scaling; keep this ticket in every count audit |
| 48 | `CPCW` | chilled-water specific heat | `Psychrometrics.hh:1611` (inline) | always present; temperature argument intentionally unused | intended `ep_runtime::psychrometrics`; unassigned | exact 4180 J/(kg K) result over representative and extreme temperatures |
| 49 | `CPHW` | hot-water specific heat | `Psychrometrics.hh:1624` (inline) | always present; temperature argument intentionally unused | intended `ep_runtime::psychrometrics`; unassigned | exact 4180 J/(kg K) result over representative and extreme temperatures |
| 50 | `RhoH2O` | water density | `Psychrometrics.hh:1637` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | polynomial coefficient vectors across the documented temperature range and boundary handling |
| 51 | `PsyDeltaHSenFnTdb2Tdb1W` | sensible enthalpy delta | `Psychrometrics.hh:1654` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | sign convention, 1e-5 humidity floor, zero delta, and equality with the stated enthalpy subtraction |
| 52 | `PsyDeltaHSenFnTdb2W2Tdb1W1` | sensible enthalpy delta | `Psychrometrics.hh:1679` (inline) | always present | intended `ep_runtime::psychrometrics`; unassigned | minimum-humidity selection, direction/sign, and delegation equality with routine 51 |
| 53 | `CSplineint` | spline interpolation | `Psychrometrics.hh:1698`; `Psychrometrics.cc:1450` | always present | intended `ep_runtime::psychrometrics`; unassigned | pinned table knots, between-knot interpolation, endpoint/range behavior, and sample-count handling |

## Compile-Time Variant Boundary

Unless `EP_nocache_Psychrometrics` is set, the EnergyPlus header enables
`EP_cache_PsyTwbFnTdbWPb`, `EP_cache_PsyPsatFnTemp`,
`EP_cache_PsyTsatFnPb`, and `EP_cache_PsyTsatFnHPb`. The cached and
no-cache declarations of each same-name routine remain one ticket; raw
delegates remain their own tickets because they are separately named source
interfaces. `EP_psych_errors` controls the eleven named error helpers in the
table. `EP_psych_stats` controls counters/reporting inside routines rather
than creating additional logical interfaces.

## Cache And State Ownership

`src/EnergyPlus/PsychCacheData.hh` owns the four large cache arrays, cache
entry structs and precision/mask policy, plus the optional psychrometric call
and cache-hit counters. `InitializePsychRoutines` initializes those arrays
under their individual compile macros. The Rust port must give this state an
explicit runtime owner and prove that separate simulation states cannot share
cache entries or diagnostics.

`PsychrometricsData` in `Psychrometrics.hh:1702` separately owns
`iconvTol`, the last/saved pressure and saturation-temperature values,
`iPsyErrIndex`, the diagnostic string and reporting switch, and
`useInterpolationPsychTsatFnPb`. These fields are lifecycle state, not
permission to substitute process-global mutable state. The local last-input
caches in `PsyCpAirFnW` and `PsyCpAirFnW_fast` also require explicit
thread/state-isolation decisions in Rust.

## Explicitly Excluded Member Routines

The following three `PsychrometricsData` member overrides occur after the
namespace-level interface and are deliberately excluded from the 53
free-function tickets:

- `PsychrometricsData::init_constant_state` (`Psychrometrics.hh:1714`)
- `PsychrometricsData::init_state` (`Psychrometrics.hh:1719`)
- `PsychrometricsData::clear_state` (`Psychrometrics.hh:1723`)

`init_constant_state` delegates to `InitializePsychRoutines`;
`clear_state` resets the module diagnostic/interpolation state. Their
behavior still constrains the cache/state lifecycle described above, but they
are member overrides rather than top-level `Psychrometrics` namespace free
functions.

## Promotion Boundary

Every ledger routine remains `source_mapped` with
`required_for_full_domain = false`. Before any ticket is promoted, its Rust
target, state ownership, source-vector tests, compile-variant obligations,
diagnostic behavior where applicable, and external evidence boundary must be
recorded. This map adds no routine to the project-contract required set and
does not establish psychrometrics implementation or conformance completion.
