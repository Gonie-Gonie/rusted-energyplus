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
| 4 | `PsyRhoAirFnPbTdbW` | moist-air density | `Psychrometrics.hh:513,549` (inline implementations) | two always-present stateful/stateless overloads; one logical ticket | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_rho_air_fn_pb_tdb_w`; guarded legacy wrapper: `energyplus_moist_air_density_kg_per_m3` | compare both overloads over source vectors, humidity floor behavior, diagnostics path, and mutual equivalence where domains overlap |
| 5 | `PsyRhoAirFnPbTdbW_fast` | moist-air density fast path | `Psychrometrics.hh:576` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_rho_air_fn_pb_tdb_w_fast` | compare with the ordinary density routine and lock the fast-path input-domain preconditions |
| 6 | `PsyHfgAirFnWTdb` | latent enthalpy | `Psychrometrics.hh:593` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_hfg_air_fn_w_tdb` | coefficient-vector and temperature/humidity boundary parity |
| 7 | `PsyHgAirFnWTdb` | water-vapor gas enthalpy | `Psychrometrics.hh:623` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_hg_air_fn_w_tdb`; legacy one-argument wrapper: `energyplus_water_vapor_gas_enthalpy_j_per_kg` | source-vector parity including ignored-`W` semantics and temperature limits |
| 8 | `PsyHFnTdbW` | moist-air enthalpy | `Psychrometrics.hh:648` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_h_fn_tdb_w`; separate partial legacy analogue: `ep_runtime::ideal_loads::calc::psychrometrics::moist_air_enthalpy_j_per_kg` | coefficient-vector parity, humidity floor/domain behavior, and inverse round trips |
| 9 | `PsyHFnTdbW_fast` | moist-air enthalpy fast path | `Psychrometrics.hh:668` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_h_fn_tdb_w_fast` | ordinary/fast equivalence across the documented valid domain |
| 10 | `PsyCpAirFnW` | moist-air specific heat | `Psychrometrics.hh:679` (inline) | always present; owns a function-local last-input cache | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_cp_air_fn_w`; guarded legacy wrapper: `energyplus_moist_air_specific_heat_j_per_kg_k` | source-vector parity plus repeated, alternating, and multistate cache-isolation probes |
| 11 | `PsyCpAirFnW_fast` | specific-heat fast path | `Psychrometrics.hh:718` (inline) | always present; owns a function-local last-input cache | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_cp_air_fn_w_fast` | ordinary/fast equivalence and cache-hit/miss independence |
| 12 | `PsyTdbFnHW` | dry-bulb inversion | `Psychrometrics.hh:743` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_tdb_fn_h_w`; separate partial legacy analogue: `ep_runtime::ideal_loads::outdoor_air::psychrometrics::dry_bulb_from_enthalpy_and_humidity_ratio` | source vectors, humidity-floor/IEEE edges, and source-evaluation round trips |
| 13 | `PsyRhovFnTdbRhLBnd0C` | vapor density | `Psychrometrics.hh:764` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_rhov_fn_tdb_rh_lbnd0c` | unclamped below-zero-C temperatures, raw RH/IEEE edges, and pressure-independent source vectors |
| 14 | `PsyRhovFnTdbWPb` | vapor density | `Psychrometrics.hh:789` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_rhov_fn_tdb_w_pb` | humidity-ratio floor, pressure/source vectors, IEEE edges, and future inverse-RH consistency |
| 15 | `PsyRhovFnTdbWPb_fast` | vapor-density fast path | `Psychrometrics.hh:815` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_rhov_fn_tdb_w_pb_fast` | ordinary/fast valid-domain equivalence, debug assertion, and `NDEBUG` raw no-floor behavior |
| 16 | `PsyRhFnTdbRhovLBnd0C_error` | diagnostics | `Psychrometrics.hh:826`; `Psychrometrics.cc:222` | compiled only with `EP_psych_errors` | intended per-simulation psychrometric diagnostics owner; unassigned | strict high/low triggers, warmup suppression, first/caller text, and shared high/low recurring-warning state |
| 17 | `PsyRhFnTdbRhovLBnd0C` | relative humidity | `Psychrometrics.hh:835` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_rh_fn_tdb_rhov_lbnd0c`; stateful statistics/diagnostics adapter unassigned | positive-vapor ternary, raw supplied-temperature behavior with no 0-C clamp, correction/error thresholds, and vapor-density round trips |
| 18 | `PsyTwbFnTdbWPb` | wet-bulb solve and cache | `Psychrometrics.hh:879,895`; `Psychrometrics.cc:281,356` | cached and no-cache same-name variants selected by `EP_cache_PsyTwbFnTdbWPb`; one logical ticket | partial analogue: `ep_runtime::psychrometrics::energyplus_outdoor_wet_bulb_c` | cached/no-cache/raw agreement, iteration convergence/failure, cache-key quantization, and caller-context behavior |
| 19 | `PsyTwbFnTdbWPb_raw` | wet-bulb raw solve | `Psychrometrics.hh:886`; `Psychrometrics.cc:347` | exists only with `EP_cache_PsyTwbFnTdbWPb` | partial analogue: `ep_runtime::psychrometrics::energyplus_outdoor_wet_bulb_c` | direct raw-vector parity and identity with cache misses |
| 20 | `PsyVFnTdbWPb_error` | diagnostics | `Psychrometrics.hh:905`; `Psychrometrics.cc:569` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | exact invalid-volume trigger, caller text, recurrence suppression, and error-state mutation |
| 21 | `PsyVFnTdbWPb` | moist-air specific volume | `Psychrometrics.hh:914` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_v_fn_tdb_w_pb`; stateful statistics/diagnostics adapter unassigned | source vectors, invalid-result fallback, and density reciprocal relationship within source tolerances |
| 22 | `PsyWFnTdbH_error` | diagnostics | `Psychrometrics.hh:954`; `Psychrometrics.cc:606` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | negative-humidity trigger, corrected value, recurrence suppression, and caller context |
| 23 | `PsyWFnTdbH` | humidity-ratio inversion | `Psychrometrics.hh:962` (inline) | always present | canonical numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_w_fn_tdb_h`; stateful statistics/diagnostics adapter unassigned | enthalpy round trips, humidity floor/correction branches, and source vectors |
| 24 | `PsyPsatFnTemp_raw` | saturation pressure raw path | `Psychrometrics.hh:1002`; `Psychrometrics.cc:642` | exists only with `EP_cache_PsyPsatFnTemp`; internal formula selects non-IF97 versus `EP_IF97` branch | canonical default non-IF97 numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_psat_fn_temp_raw`; stateful statistics/diagnostics and IF97 branch unassigned | raw branch vectors across ice/water boundaries, range guards, and both IF97 compile branches |
| 25 | `PsyPsatFnTemp` | saturation pressure and cache | `Psychrometrics.hh:1016,1066`; cached inline in header, no-cache implementation `Psychrometrics.cc:649` | variants selected by `EP_cache_PsyPsatFnTemp`; one logical ticket | partial finite numerical projection: private saturation-pressure compatibility helper and cache-temperature quantizer; cache owner unassigned | cached/no-cache/raw agreement, cache-key quantization/collisions, range guards, and repeated-call stability |
| 26 | `PsyTsatFnHPb_raw` | saturation temperature from enthalpy/pressure raw path | `Psychrometrics.hh:1074`; `Psychrometrics.cc:900` | exists only with `EP_cache_PsyTsatFnHPb` | intended `ep_runtime::psychrometrics`; unassigned | raw inversion vectors, convergence/limits, and identity with cache misses |
| 27 | `PsyTsatFnHPb` | saturation temperature from enthalpy/pressure and cache | `Psychrometrics.hh:1079,1123`; cached inline in header, no-cache implementation `Psychrometrics.cc:906` | variants selected by `EP_cache_PsyTsatFnHPb`; one logical ticket | intended `ep_runtime::psychrometrics`; unassigned | cached/no-cache/raw agreement, two-input cache key, convergence, and boundary vectors |
| 28 | `PsyRhovFnTdbRh` | vapor density | `Psychrometrics.hh:1131` (inline) | always present | canonical default-build numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_rhov_fn_tdb_rh`; stateful saturation-pressure cache/statistics/diagnostics adapter unassigned | temperature/RH vectors, physical-domain limits, and reciprocal RH checks |
| 29 | `PsyRhFnTdbRhov_error` | diagnostics | `Psychrometrics.hh:1161`; `Psychrometrics.cc:1075` | compiled only with `EP_psych_errors` | intended per-simulation diagnostics owner; unassigned | exact RH-bound trigger, caller text, recurrence suppression, and error-state mutation |
| 30 | `PsyRhFnTdbRhov` | relative humidity | `Psychrometrics.hh:1169` (inline) | always present | canonical default-build numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_rh_fn_tdb_rhov`; stateful statistics/cache/diagnostics adapter unassigned | vapor-density round trips, clamp/error thresholds, and temperature extremes |
| 31 | `PsyRhFnTdbWPb_error` | diagnostics | `Psychrometrics.hh:1215`; `Psychrometrics.cc:1133` | compiled only with `EP_psych_errors` | intended diagnostics owner; unassigned | exact RH-bound trigger, caller context, recurrence suppression, and corrected return path |
| 32 | `PsyRhFnTdbWPb` | relative humidity | `Psychrometrics.hh:1223` (inline) | always present | canonical ordinary-finite default-build numerical scaffold: `ep_runtime::psychrometrics::energyplus_psy_rh_fn_tdb_w_pb`; stateful statistics/cache/diagnostics adapter unassigned | humidity/pressure vectors, clamp/error thresholds, and humidity-ratio round trips |
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

## CP56-2 Numerical Scaffold: Density And Specific Heat

This section promotes only the source/state understanding for a selected
direct-formula pair. It does not promote either routine to `implemented`,
does not claim that an existing Rust analogue has EnergyPlus-equivalent edge
or diagnostic behavior, and does not add external evidence, family gating,
conformance, or a project-contract obligation. Both tickets remain under the
parent algorithm's `status = "scaffold"` and `claim_level = "none"` boundary.
The current pure numerical helpers and local bit-pattern, floor, IEEE-edge,
legacy-wrapper, and call-stability tests are in
`crates/ep_runtime/src/psychrometrics.rs` and
`crates/ep_runtime/src/psychrometrics_tests.rs`. Those Rust-only checks are
scaffold evidence, not an external EnergyPlus parity oracle.

### `PsyRhoAirFnPbTdbW` (`psy_rho_air_fn_pb_tdb_w`)

The ordinary source formula is in `Psychrometrics.hh:513-546`; the stateless
overload repeats it at `Psychrometrics.hh:549-574`. Both calculate
`pb / (287.0 * (tdb + Constant::Kelvin) * (1.0 + 1.6077687 * max(dw,
1.0e-5)))`. The numerical result has no cache or history. The overload that
accepts `EnergyPlusData &state` differs only when `EP_psych_errors` is enabled
and the calculated density is negative: it delegates to
`PsyRhoAirFnPbTdbW_error` (`Psychrometrics.cc:198-218`), which emits a severe
message, an input continuation, a caller-or-`Unknown` timestamp, and then a
fatal error. That stateful diagnostic branch remains deferred.

<!-- routine-state-contract:v1 begin psy_rho_air_fn_pb_tdb_w -->
PsyRhoAirFnPbTdbW

read_state:
- arguments `pb`, `tdb`, and `dw`; `CalledFrom` and `EnergyPlusData &state` are read only by the enabled negative-density diagnostics path

write_state:
- ordinary formula and stateless overload write no state; enabled negative-density diagnostics mutate the EnergyPlus error stream and terminate through the fatal-error path

history_state_ownership:
- no cross-call history or cache; density output is a pure function of `pb`, `tdb`, `max(dw, 1.0e-5)`, and `Constant::Kelvin` before optional diagnostics

unsupported_state:
- `EP_psych_errors` severe/continue/timestamp/fatal diagnostic state delegated to `PsyRhoAirFnPbTdbW_error`

inactive_branches:
- when `EP_psych_errors` is disabled, the state argument and `CalledFrom` are unused and the negative-density diagnostic branch is compiled out

unsupported_active_branches:
- stateful overload negative-density branch with severe, input-context continuation, caller-or-Unknown timestamp, and fatal termination when `EP_psych_errors` is enabled

not_claimed_branches:
- numerical density parity, humidity-floor and nonphysical-input edges, overload parity, and all diagnostic side effects
<!-- routine-state-contract:v1 end psy_rho_air_fn_pb_tdb_w -->

### `PsyCpAirFnW` (`psy_cp_air_fn_w`)

The source at `Psychrometrics.hh:679-715` reads `dw` plus two function-local
static values, `dwSave` and `cpaSave`, both initialized to `-100.0`. An exact
`dwSave == dw` hit returns `cpaSave`; otherwise the routine computes
`1.00484e3 + max(dw, 1.0e-5) * 1.85895e3` and replaces both saved values. For
physical humidity-ratio inputs the cache changes work, not the returned
formula. The initialization sentinel is nevertheless observable: the first
process call with `dw == -100.0` returns the initial `cpaSave == -100.0`
instead of evaluating the formula. The static locals are process/function
history rather than `EnergyPlusData`-owned state. Rust currently has no mutable
cache for this routine, so cache history, sentinel behavior, cross-simulation
sharing, and thread/isolation policy remain deferred.

<!-- routine-state-contract:v1 begin psy_cp_air_fn_w -->
PsyCpAirFnW

read_state:
- argument `dw` and function-local static `dwSave`/`cpaSave`, both initialized to `-100.0`, are read before the cache-hit comparison

write_state:
- cache miss writes `dwSave = dw` and `cpaSave = 1.00484e3 + max(dw, 1.0e-5) * 1.85895e3`; cache hit writes no state

history_state_ownership:
- EnergyPlus owns one function-local static last-call cache shared across calls and simulation states; Rust currently owns no mutable cache for this routine

unsupported_state:
- the function-local `dwSave`/`cpaSave` cache, including first-call `dw == -100.0` sentinel collision that returns `-100.0`

inactive_branches:
- none; the last-call cache is unconditional in the pinned EnergyPlus 26.1.0 source

unsupported_active_branches:
- cache hit/miss history behavior, cross-simulation/process sharing, and sentinel-collision behavior; the cache is output-neutral for physical humidity-ratio inputs

not_claimed_branches:
- external EnergyPlus numerical parity, C++ last-call-cache work/history parity under repeated or alternating calls, sentinel collision, and state/thread-isolation behavior
<!-- routine-state-contract:v1 end psy_cp_air_fn_w -->

## CP56-3 Direct Formula And Fast-Path Scaffold

This checkpoint adds pure Rust numerical helpers for routines 5 through 9 and
11 in source-interface order. The helpers and their local pinned-formula,
evaluation-order, humidity-floor, ignored-argument, IEEE-edge, debug-assertion,
release no-floor, repeated-call, and legacy-wrapper tests live in
`crates/ep_runtime/src/psychrometrics.rs` and
`crates/ep_runtime/src/psychrometrics_tests.rs`. These checks are Rust-only
source-transcription evidence, not output captured from an external EnergyPlus
oracle. The six tickets advance only to `state_mapped`; they do not advance to
`implemented`, add conformance evidence, or change the parent algorithm's
`status = "scaffold"` and `claim_level = "none"` boundary.

The existing IdealLoads `moist_air_enthalpy_j_per_kg` helper remains separate:
it has no `1.0e-5` humidity floor and groups the expression through kJ units,
which can differ from the source-order formula by one ULP. Replacing its
downstream consumers is deferred until that compatibility impact and the
related inversion routines are handled explicitly. The existing one-argument
water-vapor enthalpy API remains a bit-preserving wrapper over the new
two-argument `PsyHgAirFnWTdb` numerical helper.

### `PsyRhoAirFnPbTdbW_fast` (`psy_rho_air_fn_pb_tdb_w_fast`)

The source at `Psychrometrics.hh:576-590` asserts `dw >= 1.0e-5`, evaluates
the density formula without a humidity floor, and optionally delegates a
negative result to `PsyRhoAirFnPbTdbW_error`. Because this fast overload does
not supply `CalledFrom`, the enabled error helper uses its empty default and
reports the `Unknown` caller timestamp before fatal termination. Rust uses
`debug_assert!` and a pure raw-formula helper; exact assertion termination and
the optional EnergyPlus diagnostic state remain deferred.

<!-- routine-state-contract:v1 begin psy_rho_air_fn_pb_tdb_w_fast -->
PsyRhoAirFnPbTdbW_fast

read_state:
- arguments `pb`, `tdb`, and `dw`; the debug assertion reads `dw`, and enabled `EP_psych_errors` diagnostics inspect a negative `rhoair` result

write_state:
- the numerical formula writes no state; enabled negative-density diagnostics mutate the EnergyPlus error stream and terminate through `PsyRhoAirFnPbTdbW_error`

history_state_ownership:
- no cross-call numerical history or cache; the source result is a pure function of `pb`, `tdb`, and already-adjusted `dw` before optional diagnostics

unsupported_state:
- `EP_psych_errors` severe/continue/Unknown-timestamp/fatal diagnostic state and exact C++ assertion-abort diagnostics

inactive_branches:
- `NDEBUG` removes the `dw >= 1.0e-5` assertion; disabling `EP_psych_errors` removes the negative-density diagnostic branch

unsupported_active_branches:
- assertion-enabled invalid-`dw` termination parity and enabled negative-density severe/continue/Unknown-timestamp/fatal behavior

not_claimed_branches:
- external EnergyPlus numerical parity, invalid-precondition behavior outside the fast domain, diagnostic side effects, and C++ abort versus Rust panic equivalence
<!-- routine-state-contract:v1 end psy_rho_air_fn_pb_tdb_w_fast -->

### `PsyHfgAirFnWTdb` (`psy_hfg_air_fn_w_tdb`)

The source at `Psychrometrics.hh:593-620` intentionally ignores `w`, floors
temperature with `max(T, 0.0)`, and subtracts the fluid enthalpy term from the
gas enthalpy term. The Rust helper preserves those two separate terms instead
of algebraically combining coefficients, retaining the source behavior where
positive infinity or an overflowing maximum finite temperature can produce
`infinity - infinity` and therefore NaN.

<!-- routine-state-contract:v1 begin psy_hfg_air_fn_w_tdb -->
PsyHfgAirFnWTdb

read_state:
- arguments `w` and `T`; `w` is intentionally ignored and `T` is read through source `max(T, 0.0)` semantics

write_state:
- no state; the routine computes the gas enthalpy term minus the fluid enthalpy term without mutation

history_state_ownership:
- no cross-call history or cache; output depends only on `max(T, 0.0)` and the source evaluation order

unsupported_state:
- none; the source routine has no mutable state or cache

inactive_branches:
- none; the routine is always present and has no compile-time variant

unsupported_active_branches:
- none; there is no stateful or compile-conditional active branch

not_claimed_branches:
- external EnergyPlus numerical parity, full IEEE-edge parity, and downstream latent-energy integration
<!-- routine-state-contract:v1 end psy_hfg_air_fn_w_tdb -->

### `PsyHgAirFnWTdb` (`psy_hg_air_fn_w_tdb`)

The source at `Psychrometrics.hh:623-645` ignores `w` and evaluates
`2500940.0 + 1858.95 * T`. The canonical Rust helper retains the two source
arguments; the pre-existing one-argument runtime API delegates with a dummy
humidity ratio because the value cannot affect the result.

<!-- routine-state-contract:v1 begin psy_hg_air_fn_w_tdb -->
PsyHgAirFnWTdb

read_state:
- arguments `w` and `T`; `w` is intentionally ignored and `T` supplies the linear water-vapor gas enthalpy term

write_state:
- no state; the routine evaluates `2500940.0 + 1858.95 * T` without mutation

history_state_ownership:
- no cross-call history or cache; output is a pure function of `T`

unsupported_state:
- none; the source routine has no mutable state or cache

inactive_branches:
- none; the routine is always present and has no compile-time variant

unsupported_active_branches:
- none; there is no stateful or compile-conditional active branch

not_claimed_branches:
- external EnergyPlus numerical parity, ignored-`w` parity across all call sites, and full IEEE-edge parity
<!-- routine-state-contract:v1 end psy_hg_air_fn_w_tdb -->

### `PsyHFnTdbW` (`psy_h_fn_tdb_w`)

The ordinary source formula at `Psychrometrics.hh:648-665` evaluates
`1.00484e3 * TDB + max(dW, 1.0e-5) * (2.50094e6 + 1.85895e3 * TDB)`.
The source max operation retains a NaN first argument rather than normalizing
it to the floor. Rust uses the shared source-compatible floor helper and keeps
the arithmetic in J/kg source order.

<!-- routine-state-contract:v1 begin psy_h_fn_tdb_w -->
PsyHFnTdbW

read_state:
- arguments `TDB` and `dW`; source `max(dW, 1.0e-5)` preserves a NaN first argument and supplies the humidity-ratio floor

write_state:
- no state; the dry-air and humidity-weighted enthalpy terms are evaluated in source order without mutation

history_state_ownership:
- no cross-call history or cache; output is a pure function of `TDB` and floored `dW`

unsupported_state:
- none; the source routine has no mutable state or cache

inactive_branches:
- none; the routine is always present and has no compile-time variant

unsupported_active_branches:
- none; there is no stateful or compile-conditional active branch

not_claimed_branches:
- external EnergyPlus numerical parity, full humidity/temperature edge parity, inverse round trips, and downstream IdealLoads replacement
<!-- routine-state-contract:v1 end psy_h_fn_tdb_w -->

### `PsyHFnTdbW_fast` (`psy_h_fn_tdb_w_fast`)

The source at `Psychrometrics.hh:668-676` asserts `dW >= 1.0e-5` and then
uses the same enthalpy expression without a floor. Rust mirrors the source
build split with `debug_assert!`: debug builds reject an invalid precondition,
while release builds retain the raw no-floor calculation. Exact C++ assertion
abort text and process behavior are outside this scaffold.

<!-- routine-state-contract:v1 begin psy_h_fn_tdb_w_fast -->
PsyHFnTdbW_fast

read_state:
- arguments `TDB` and already-adjusted `dW`; an assertion reads `dW >= 1.0e-5` before the unclamped formula

write_state:
- no state; the unclamped enthalpy formula evaluates without mutation after the assertion

history_state_ownership:
- no cross-call history or cache; output is a pure function of `TDB` and caller-adjusted `dW`

unsupported_state:
- exact C++ assertion-abort diagnostics; the numerical source routine has no mutable state

inactive_branches:
- `NDEBUG` removes the `dW >= 1.0e-5` assertion and leaves the raw unclamped expression active

unsupported_active_branches:
- assertion-enabled invalid-`dW` termination parity and exact C++ abort versus Rust panic behavior

not_claimed_branches:
- external EnergyPlus numerical parity and invalid-precondition behavior outside the documented fast domain
<!-- routine-state-contract:v1 end psy_h_fn_tdb_w_fast -->

### `PsyCpAirFnW_fast` (`psy_cp_air_fn_w_fast`)

The source at `Psychrometrics.hh:718-738` asserts `dw >= 1.0e-5` before it
reads its separate function-local `dwSave`/`cpaSave` cache. A valid-domain miss
evaluates `1.00484e3 + dw * 1.85895e3`, and a repeated exact input returns the
saved result. The cache changes work and history, not valid-domain output. In
an `NDEBUG` build, the first invalid call with `dw == -100.0` collides with both
initial sentinels and returns `-100.0`; assertion-enabled builds terminate
before reading the cache. Rust keeps the valid-domain numerical path pure and
defers that cache, sentinel, sharing, and concurrency policy.

<!-- routine-state-contract:v1 begin psy_cp_air_fn_w_fast -->
PsyCpAirFnW_fast

read_state:
- argument `dw` is checked by `assert(dw >= 1.0e-5)` before function-local static `dwSave`/`cpaSave`, both initialized to `-100.0`, are read for an exact cache hit

write_state:
- cache miss writes `dwSave = dw` and `cpaSave = 1.00484e3 + dw * 1.85895e3`; cache hit writes no state

history_state_ownership:
- EnergyPlus owns one function-local static last-call cache shared across calls and simulation states; Rust keeps the valid-domain numerical helper pure

unsupported_state:
- the function-local `dwSave`/`cpaSave` cache, cross-simulation sharing, concurrency policy, and the `NDEBUG` first-call `dw == -100.0` sentinel collision

inactive_branches:
- `NDEBUG` removes the pre-cache `dw >= 1.0e-5` assertion; the last-call cache itself is unconditional

unsupported_active_branches:
- cache hit/miss history, process-wide sharing, sentinel-collision behavior, and exact assertion-enabled C++ abort versus Rust panic parity

not_claimed_branches:
- external EnergyPlus numerical parity, C++ cache work/history parity, sentinel collision, and state/thread-isolation behavior
<!-- routine-state-contract:v1 end psy_cp_air_fn_w_fast -->

## CP56-4 Dry-Bulb Inversion And Vapor-Density Scaffold

This checkpoint adds pure Rust numerical helpers for routines 12 through 15 in
source-interface order. The helpers and their local pinned-formula,
source-evaluation round-trip, humidity-floor, unclamped-temperature/RH,
pressure, IEEE-edge, debug-assertion, release no-floor, and ordinary/fast tests
live in `crates/ep_runtime/src/psychrometrics.rs` and
`crates/ep_runtime/src/psychrometrics_inverse_density_tests.rs`. These checks
are Rust-only source-transcription evidence, not output captured from an
external EnergyPlus oracle. The four tickets advance only to `state_mapped`;
they do not advance to `implemented`, add conformance evidence, or change the
parent algorithm's `status = "scaffold"` and `claim_level = "none"` boundary.

The existing IdealLoads moist-air enthalpy inverse remains separate. Its
arithmetic grouping and lack of the canonical `1.0e-5` humidity floor can
produce different results, so replacing its downstream consumers is deferred
until the paired enthalpy/inverse compatibility boundary is handled explicitly.

### `PsyTdbFnHW` (`psy_tdb_fn_h_w`)

The source at `Psychrometrics.hh:743-761` first assigns
`W = max(dW, 1.0e-5)` and then evaluates
`(H - 2.50094e6 * W) / (1.00484e3 + 1.85895e3 * W)`. Rust computes the
source-compatible humidity floor once and preserves the source numerator and
denominator grouping. The routine has no cache, diagnostics, or mutable state.

<!-- routine-state-contract:v1 begin psy_tdb_fn_h_w -->
PsyTdbFnHW

read_state:
- arguments `H` and `dW`; source `max(dW, 1.0e-5)` preserves a NaN first argument and supplies the humidity-ratio floor

write_state:
- no state; the dry-bulb inverse is evaluated from `H` and floored `dW` without mutation

history_state_ownership:
- no cross-call history or cache; output is a pure function of `H` and floored `dW`

unsupported_state:
- none; the source routine has no mutable state or cache

inactive_branches:
- none; the routine is always present and has no compile-time variant

unsupported_active_branches:
- none; there is no stateful or compile-conditional active branch

not_claimed_branches:
- external EnergyPlus numerical parity, full IEEE-edge parity, and downstream IdealLoads inverse replacement
<!-- routine-state-contract:v1 end psy_tdb_fn_h_w -->

### `PsyRhovFnTdbRhLBnd0C` (`psy_rhov_fn_tdb_rh_lbnd0c`)

Despite the historical identifier, the source at
`Psychrometrics.hh:764-786` does not lower-bound `Tdb` at 0 C and does not
clamp `RH`. It evaluates the supplied values directly through
`RH / (461.52 * (Tdb + Constant::Kelvin)) *
exp(23.7093 - 4111.0 / ((Tdb + Constant::Kelvin) - 35.45))`. Rust preserves
that unclamped expression. Platform-level `exp` last-bit and floating-point
exception behavior remain outside this scaffold.

<!-- routine-state-contract:v1 begin psy_rhov_fn_tdb_rh_lbnd0c -->
PsyRhovFnTdbRhLBnd0C

read_state:
- arguments `Tdb` and `RH` exactly as supplied; despite the historical `LBnd0C` name, the routine applies neither a 0 C temperature lower bound nor RH validation/clamping

write_state:
- no state; the pressure-independent exponential vapor-density formula evaluates without mutation

history_state_ownership:
- no cross-call history or cache; output is a pure function of raw `Tdb`, raw `RH`, `Constant::Kelvin`, and the source exponential expression

unsupported_state:
- none; the source routine has no mutable state or cache

inactive_branches:
- none; the routine is always present and has no compile-time variant

unsupported_active_branches:
- none; there is no stateful or compile-conditional active branch

not_claimed_branches:
- external EnergyPlus numerical parity, cross-platform `std::exp` last-bit and floating-point exception parity, and inverse-relative-humidity integration
<!-- routine-state-contract:v1 end psy_rhov_fn_tdb_rh_lbnd0c -->

### `PsyRhovFnTdbWPb` (`psy_rhov_fn_tdb_w_pb`)

The ordinary source at `Psychrometrics.hh:789-812` first assigns
`W = max(dW, 1.0e-5)` and then computes
`W * PB / (461.52 * (Tdb + Constant::Kelvin) * (W + 0.62198))`. Rust applies
the shared source-compatible floor once and preserves the source rational
grouping. Temperature and pressure are not validated or clamped.

<!-- routine-state-contract:v1 begin psy_rhov_fn_tdb_w_pb -->
PsyRhovFnTdbWPb

read_state:
- arguments `Tdb`, `dW`, and `PB`; source `max(dW, 1.0e-5)` preserves a NaN first argument and supplies the humidity-ratio floor

write_state:
- no state; the vapor-density rational expression evaluates without mutation

history_state_ownership:
- no cross-call history or cache; output is a pure function of `Tdb`, `PB`, and floored `dW`

unsupported_state:
- none; the source routine has no mutable state or cache

inactive_branches:
- none; the routine is always present and has no compile-time variant

unsupported_active_branches:
- none; there is no stateful or compile-conditional active branch

not_claimed_branches:
- external EnergyPlus numerical parity, full nonphysical-input/IEEE-edge parity, inverse-relative-humidity integration, and downstream surface or EMPD migration
<!-- routine-state-contract:v1 end psy_rhov_fn_tdb_w_pb -->

### `PsyRhovFnTdbWPb_fast` (`psy_rhov_fn_tdb_w_pb_fast`)

The source at `Psychrometrics.hh:815-822` asserts `dW >= 1.0e-5` and evaluates
the same rational expression with the caller-supplied humidity ratio and no
floor. Rust mirrors the source build split with `debug_assert!`: debug builds
reject an invalid precondition, while release builds retain the raw no-floor
calculation. Exact C++ assertion abort text and process behavior are outside
this scaffold.

<!-- routine-state-contract:v1 begin psy_rhov_fn_tdb_w_pb_fast -->
PsyRhovFnTdbWPb_fast

read_state:
- arguments `Tdb`, caller-adjusted `dW`, and `PB`; an assertion reads `dW >= 1.0e-5` before the unclamped formula

write_state:
- no state; the unclamped vapor-density rational expression evaluates without mutation after the assertion

history_state_ownership:
- no cross-call history or cache; output is a pure function of `Tdb`, `PB`, and caller-adjusted `dW`

unsupported_state:
- exact C++ assertion-abort diagnostics; the numerical source routine has no mutable state

inactive_branches:
- `NDEBUG` removes the `dW >= 1.0e-5` assertion and leaves the raw unclamped expression active

unsupported_active_branches:
- assertion-enabled invalid-`dW` termination parity and exact C++ abort versus Rust panic behavior

not_claimed_branches:
- external EnergyPlus numerical parity, C++ `NDEBUG` versus Rust debug-assertion build-policy equivalence, invalid-precondition behavior outside the documented fast domain, and downstream fast-call-site migration
<!-- routine-state-contract:v1 end psy_rhov_fn_tdb_w_pb_fast -->

## CP56-5 Lower-Bound Relative-Humidity Numerical Scaffold

This checkpoint adds the pure Rust numerical return path for routine 17. Its
local EMS source-fixture, forward/inverse, positive-vapor ternary,
out-of-range-only correction, below-zero-temperature, NaN/infinity,
signed-zero, zero-Kelvin, and exponential-pole tests live in
`crates/ep_runtime/src/psychrometrics.rs` and
`crates/ep_runtime/src/psychrometrics_relative_humidity_tests.rs`. These are
Rust-only source-transcription checks, not output captured from an external
EnergyPlus oracle. Routine 17 advances only to `state_mapped`; it does not
advance to `implemented`, add conformance evidence, or change the parent
algorithm's `status = "scaffold"` and `claim_level = "none"` boundary.

Routine 16 remains `source_mapped`. EnergyPlus enables `USE_PSYCH_ERRORS` by
default and leaves `USE_PSYCH_STATS` disabled by default. The error helper owns
per-`EnergyPlusData` warning history that the current Rust runtime cannot
honestly replace with a process-global or pure predicate. The optional
`EP_psych_stats` counter is also excluded from the pure helper. This split
preserves the numerical return while keeping the default oracle-build warning
and recurrence behavior explicitly deferred.

### `PsyRhFnTdbRhovLBnd0C_error` (`psy_rh_fn_tdb_rhov_lbnd0c_error`)

The helper is declared at `Psychrometrics.hh:825-832` and implemented under
`EP_psych_errors` at `Psychrometrics.cc:221-277`. Routine 17 calls it only for
raw relative humidity strictly below `-0.05` or strictly above `1.01`; the
exact endpoints are corrected silently. During warmup the helper changes no
diagnostic state. On the first non-warmup occurrence it writes the formatted
input scratch string, warning, caller-or-`Unknown` timestamp, input details,
and direction-specific reset text. Every non-warmup occurrence then updates
one recurring-warning index, count, minimum, and maximum in percent. High and
low excursions share that one index, so a first high warning suppresses the
later low first-message path and vice versa.

The error index, scratch string, warning stream and totals, recurring-warning
table, and SQLite/callback effects are per-simulation `EnergyPlusData` state.
Rust has no owner for that state in this checkpoint; routine 16 therefore has
no Rust implementation or completion claim.

### `PsyRhFnTdbRhovLBnd0C` (`psy_rh_fn_tdb_rhov_lbnd0c`)

The source at `Psychrometrics.hh:834-875` first increments its per-function
call count only when `EP_psych_stats` is enabled. It then evaluates the
left-associated exponential expression only when `Rhovapor > 0.0`; negative,
signed-zero, and NaN vapor-density inputs return literal positive zero without
reading `Tdb`. Despite the historical identifier, the supplied temperature is
not clamped to 0 C. A raw result inside `[0.0, 1.0]`, including values below
`0.01`, is returned unchanged. Only a negative raw result is corrected to
`0.01`, and only a result above one is corrected to `1.0`. The optional error
helper sees the raw value before that correction.

<!-- routine-state-contract:v1 begin psy_rh_fn_tdb_rhov_lbnd0c -->
PsyRhFnTdbRhovLBnd0C

read_state:
- arguments `Tdb` and `Rhovapor`; `EP_psych_stats` reads the per-state call counter, while `EP_psych_errors` reads `WarmupFlag` and the shared error index and reads `CalledFrom` only for the first non-warmup extreme-range warning

write_state:
- the numerical formula and return correction write no state; `EP_psych_stats` increments the per-state call count, while the enabled extreme-range helper mutates warning, scratch-string, error-index, and recurring-warning state only outside warmup

history_state_ownership:
- numerical output is a pure function of raw `Tdb` and `Rhovapor`; optional call statistics and one shared high/low recurring-warning history belong to each `EnergyPlusData` instance

unsupported_state:
- `EP_psych_stats` call counting plus `EP_psych_errors` warmup suppression, scratch string, warning stream and totals, caller timestamp, shared recurrence index/count/min/max, SQLite, and callback state

inactive_branches:
- disabling `EP_psych_stats` compiles out the call-count increment; disabling `EP_psych_errors` compiles out the extreme-range helper call and routine 16 itself without changing the numerical correction

unsupported_active_branches:
- default errors-enabled strict `< -0.05` or `> 1.01` warning flow, first-versus-recurring behavior, shared high/low recurrence, and the optional statistics-enabled every-call counter

not_claimed_branches:
- external EnergyPlus numerical parity, cross-platform `std::exp` last-bit and floating-point exception parity, exact diagnostic formatting/side effects, statistics history, and downstream surface, EMPD, room-air, or EMS migration
<!-- routine-state-contract:v1 end psy_rh_fn_tdb_rhov_lbnd0c -->

## Routines 18-19 Cached Wet-Bulb Deferral Boundary

The two wet-bulb tickets remain `source_mapped`. The current Rust
`energyplus_outdoor_wet_bulb_c` helper is a production weather adapter that
accepts relative humidity in percent, validates finite inputs and pressure,
converts RH to humidity ratio, returns `Option<f64>`, and lets its callers fall
back to dry-bulb temperature. EnergyPlus routines 18 and 19 instead accept a
humidity ratio directly, continue through many non-finite or nonphysical
inputs, and return `Real64`. Renaming that adapter, delegating either canonical
ticket to it, or migrating weather callers would therefore change the source
contract.

### `PsyTwbFnTdbWPb` (`psy_twb_fn_tdb_w_pb`)

The default cache-enabled build owns 1,048,576 direct-mapped entries in each
`PsychrometricCacheData`. Each entry stores the upper 32 bits of the `Tdb`, `W`,
and `Pb` bit patterns plus the result: `twbprecision_bits = 20` makes
`Grid_Shift = 32`, and the index is
`(Tdb_tag ^ W_tag ^ Pb_tag) & 0xFFFFF`. A miss overwrites all three tags and
evaluates `PsyTwbFnTdbWPb_raw` on reconstructed inputs whose lower 32 bits are
zero. A hit returns the saved value without replaying raw diagnostics,
`CalledFrom`, raw call/iteration statistics, the saved boiling-pressure state,
or nested saturation caches. Fresh entries contain three zero tags and a zero
result, so an all-zero-tag lookup is a source-defined initial hit rather than a
miss. Optional statistics still count every public cache lookup.

With `EP_nocache_Psychrometrics`, the same public name compiles the raw body
directly on the original inputs and `PsyTwbFnTdbWPb_raw` is absent. That switch
also disables the nested `PsyPsatFnTemp` and `PsyTsatFnPb` caches, so a hybrid
Rust path with only the outer cache removed would not represent the upstream
no-cache variant. Promotion requires a per-simulation cache owner, exact tag,
sentinel, collision, and initialization tests, cache-hit side-effect tests,
independent-state isolation, and separate default-cache and no-cache evidence.

### `PsyTwbFnTdbWPb_raw` (`psy_twb_fn_tdb_w_pb_raw`)

The named raw routine exists only in the cache-enabled build and bypasses only
the outer wet-bulb cache. It reads `iconvTol`, `last_Patm`, `last_tBoil`, the
warmup flag, three shared warning indices, and optional call/iteration
statistics. A changed pressure calls `PsyTsatFnPb` and writes the exact-pressure
`last_Patm`/`last_tBoil` pair; every iteration calls `PsyPsatFnTemp`, so the raw
routine still reads and mutates nested cache, interpolation, saved-value,
diagnostic, and statistics state.

Numerically, every negative humidity ratio is reset to `1.0e-5`, while only
values `<= -0.0001` enter the humidity-warning branch. The solver starts at
`TDB`, limits a guess at or above `tBoil - 0.09` to `tBoil - 0.1`, applies the
separate nonnegative and negative wet-bulb formulas, and calls
`General::Iterate` with a 100-iteration limit. Temperature inputs `<= -100` or
`>= 200`, significant negative humidity, and iteration failure may warn outside
warmup but do not replace the returned numerical continuation. The final cap is
the ordered comparison `if (TWB > TDB) TWB = TDB`, not an IEEE-NaN-changing
`min` operation.

Promotion requires canonical `PsyTsatFnPb` and `PsyPsatFnTemp` dependencies,
an explicit owner for the saved pressure/boiling pair and diagnostic/statistics
history, upstream below-freezing and near-zero vectors, boiling-limit and
nonconvergence vectors, negative-humidity threshold tests, and NaN, infinity,
signed-zero, denominator, and final-cap evidence. A pure arithmetic scaffold
would not represent the named raw interface while those active default-build
dependencies remain absent.

## CP56-6 Specific-Volume Numerical Scaffold

This checkpoint advances only `PsyVFnTdbWPb` to `state_mapped` and leaves its
separately named diagnostics helper `PsyVFnTdbWPb_error` at `source_mapped`.
The parent inventory remains `status = "scaffold"`, `claim_level = "none"`,
and all 53 routines remain outside the full-domain required set.

EnergyPlus enables `USE_PSYCH_ERRORS` by default and leaves
`USE_PSYCH_STATS` disabled by default. The Rust helper preserves the numerical
result independently of those branches: source-style `max(dW, 1.0e-5)` keeps a
first-argument NaN, the specific-volume expression remains left-associated,
and every strictly negative calculated volume returns literal `0.83`. A
calculated negative zero and NaN bypass the ordered `< 0.0` fallback.

Pinned evidence includes the EnergyPlus functional-API input
`(24 C, 0.009, 101325 Pa)`, the EMS fixture `(30 C, 0.01, 101325 Pa)`, ordinary
formula vectors, humidity-floor and IEEE edges, and repeated/alternating calls.
The density reciprocal check deliberately uses source tolerance because
`PsyVFnTdbWPb` and `PsyRhoAirFnPbTdbW` retain different legacy constants.

### `PsyVFnTdbWPb_error` (`psy_v_fn_tdb_w_pb_error`)

The main routine calls this helper only for a pre-correction calculated volume
`V <= -0.01`, and the helper repeats that comparison. Warmup suppresses every
message and state mutation. On the first qualifying non-warmup occurrence, the
helper writes its scratch string twice, emits the main warning, uses
`CalledFrom` or `Routine=Unknown,` in the timestamp, records the already-floored
humidity ratio, and explains the `0.83` fallback. Every qualifying non-warmup
occurrence, including the first, updates one dedicated recurring-warning
index, count, minimum, and maximum in `m3/kg` and the warning total. Subsequent
occurrences neither read `CalledFrom` nor overwrite the scratch string.

The scratch string, warning stream and totals, timestamp context,
recurring-warning table, SQLite, and callback effects are per-simulation
`EnergyPlusData` state. Rust has no owner for that state in this checkpoint;
routine 20 therefore has no Rust implementation or completion promotion.

### `PsyVFnTdbWPb` (`psy_v_fn_tdb_w_pb`)

The canonical Rust numerical scaffold is
`ep_runtime::psychrometrics::energyplus_psy_v_fn_tdb_w_pb`. Optional
statistics count every call before the formula. The default errors-enabled
branch observes the uncorrected result only at `V <= -0.01`; values in
`(-0.01, 0.0)` silently receive the same unconditional `0.83` return. The
stateful counter and diagnostics behavior remain explicitly unsupported.

<!-- routine-state-contract:v1 begin psy_v_fn_tdb_w_pb -->
PsyVFnTdbWPb

read_state:
- arguments `TDB`, `dW`, and `PB`; `EP_psych_stats` reads the per-state call counter, while `EP_psych_errors` reads `WarmupFlag` and the dedicated specific-volume error index and reads `CalledFrom` only for the first non-warmup `V <= -0.01` warning

write_state:
- the numerical formula and `V < 0.0` fallback write no state; `EP_psych_stats` increments the per-state call count, while `PsyVFnTdbWPb_error` mutates warning, scratch-string, error-index, and recurring-warning state only outside warmup

history_state_ownership:
- numerical output is a pure function of raw `TDB`, source-floored `dW`, and raw `PB`; optional call statistics and one specific-volume recurring-warning history belong to each `EnergyPlusData` instance

unsupported_state:
- `EP_psych_stats` call counting plus `EP_psych_errors` warmup suppression, scratch string, warning stream and totals, caller timestamp, dedicated recurrence index/count/min/max, SQLite, and callback state

inactive_branches:
- disabling `EP_psych_stats` compiles out the call-count increment; disabling `EP_psych_errors` compiles out the `V <= -0.01` helper call and routine 20 without changing the unconditional `V < 0.0` fallback to `0.83`

unsupported_active_branches:
- default errors-enabled `V <= -0.01` warning flow, first-versus-recurring behavior, caller context, per-state recurrence, and the optional statistics-enabled every-call counter

not_claimed_branches:
- external EnergyPlus numerical parity, cross-platform floating-point last-bit and exception parity, exact diagnostic formatting/side effects, statistics history, and downstream EMS, C API, or HVAC call-site migration
<!-- routine-state-contract:v1 end psy_v_fn_tdb_w_pb -->

## CP56-7 Humidity-Ratio Inversion Numerical Scaffold

This checkpoint advances only `PsyWFnTdbH` to `state_mapped` and leaves its
separately named diagnostics helper `PsyWFnTdbH_error` at `source_mapped`.
The parent inventory remains `status = "scaffold"`, `claim_level = "none"`,
and all 53 routines remain outside the full-domain required set.

The Rust helper preserves the source grouping
`(H - 1.00484e3 * TDB) / (2.50094e6 + 1.85895e3 * TDB)`, followed by the
ordered `W < 0.0` correction to literal `1.0e-5`. This is not a `max`
operation: positive results below `1.0e-5` remain unchanged, NaN and negative
zero bypass the correction, and negative infinity is corrected. Tests also
cover the denominator pole and both adjacent representable temperatures.

Pinned evidence includes the EnergyPlus EMS vector
`(20 C, 30000 J/kg) -> 0.00390178711`, the functional-API vector
`(24 C, 48000 J/kg) -> approximately 0.009`, direct upstream unit-test inputs,
canonical `PsyHFnTdbW` round trips, IEEE edges, and repeated/alternating calls.
These checks are Rust-only source-transcription evidence, not output captured
from an external EnergyPlus oracle.

### `PsyWFnTdbH_error` (`psy_w_fn_tdb_h_error`)

With errors enabled, the main routine dispatches diagnostics only for the raw
`W <= -0.0001` and `SuppressWarnings = false`; routine 22 then rechecks strict
`W < -0.0001`. Exact `-0.0001` therefore enters the helper but causes no
diagnostic mutation. Warmup suppresses all helper messages and mutations. The
first qualifying non-warmup warning writes scratch text twice and reads
`CalledFrom` for its timestamp; every qualifying non-warmup occurrence updates
one dedicated recurrence index, count, minimum, and maximum with `[]` units.
The helper never corrects `W`; routine 23 performs the unconditional ordered
negative correction after optional diagnostic dispatch.

The scratch string, warning stream and totals, caller timestamp,
recurring-warning table, SQLite, callback effects, and optional statistics are
per-`EnergyPlusData` state. Rust has no owner for that state in this checkpoint;
routine 22 therefore has no Rust implementation or completion promotion.

### `PsyWFnTdbH` (`psy_w_fn_tdb_h`)

The canonical Rust numerical scaffold is
`ep_runtime::psychrometrics::energyplus_psy_w_fn_tdb_h`. Existing private
IdealLoads humidity-ratio inversions remain separate partial analogues: their
kJ-scaled regrouping and caller-side flooring require a later compatibility
audit before migration. Optional statistics count every canonical call, and
the stateful statistics/diagnostics behavior remains explicitly unsupported.

<!-- routine-state-contract:v1 begin psy_w_fn_tdb_h -->
PsyWFnTdbH

read_state:
- arguments `TDB` and `H`; `EP_psych_stats` reads the per-state call counter; with `EP_psych_errors` enabled, `SuppressWarnings` is read only after a calculated `W <= -0.0001`, and routine 22 reads `WarmupFlag` and the dedicated humidity-ratio error index only for `W < -0.0001`, then reads `CalledFrom` only for the first such non-warmup warning

write_state:
- the numerical formula and ordered `W < 0.0` fallback write no state; `EP_psych_stats` increments the per-state call count, while routine 22 mutates warning, scratch-string, error-index, and recurring-warning state only for non-warmup `W < -0.0001`

history_state_ownership:
- numerical output is a pure function of raw `TDB` and `H`; `CalledFrom` and `SuppressWarnings` never change the result, while optional call statistics and one humidity-ratio recurring-warning history belong to each `EnergyPlusData` instance

unsupported_state:
- `EP_psych_stats` call counting plus `EP_psych_errors` warmup suppression, scratch string, warning stream and totals, caller timestamp, dedicated recurrence index/count/min/max with `[]` units, SQLite, and callback state

inactive_branches:
- disabling `EP_psych_stats` compiles out the call-count increment; disabling `EP_psych_errors` compiles out the diagnostic dispatch and routine 22 without changing the unconditional floor of every strictly negative `W` to `1.0e-5`
- with errors enabled, `SuppressWarnings = true` skips diagnostic dispatch for significant negative `W`, while warmup suppresses routine-22 messages and mutations; neither branch changes the numerical return

unsupported_active_branches:
- default errors-enabled dispatch at `W <= -0.0001 && !SuppressWarnings`, including the routine-22 strict `W < -0.0001` recheck that makes exact `-0.0001` a diagnostic no-op, plus first-versus-recurring behavior, caller context, per-state recurrence, and the optional statistics-enabled every-call counter

not_claimed_branches:
- external EnergyPlus numerical parity, cross-platform floating-point last-bit and exception parity, exact diagnostic formatting/side effects, statistics history, equivalence with existing ideal-loads approximations, and downstream C API, EMS, sizing, coil, or HVAC call-site migration
<!-- routine-state-contract:v1 end psy_w_fn_tdb_h -->

## CP56-8 Raw Saturation-Pressure Numerical Scaffold

This checkpoint advances only `PsyPsatFnTemp_raw` to `state_mapped` and leaves
the public cached/no-cache `PsyPsatFnTemp` ticket at `source_mapped`. The parent
inventory remains `status = "scaffold"`, `claim_level = "none"`, and all 53
routines remain outside the full-domain required set.

The standard EnergyPlus build enables psychrometric caching and errors, leaves
psychrometric statistics disabled, and does not define `EP_IF97`. The canonical
Rust raw helper therefore implements only the default Hyland-Wexler numerical
body. With `Tkel = T + 273.15`, source order is: the low constant
`0.001405102123874164` for `Tkel < 173.15`, the ice expression for
`Tkel < 273.16`, the default liquid-water expression for `Tkel <= 473.15`, and
the high constant `1555073.745636215` otherwise. The inactive `EP_IF97` branch
replaces only the liquid expression and high constant.

Those comparisons are deliberately stated in terms of calculated `Tkel`.
Binary rounding makes exact `-100 C` take the low-constant branch, exact
`0.01 C` remain on the ice branch, and exact `200 C` take the liquid branch.
The diagnostic trigger is separate and inclusive on original input
`T <= -100.0 || T >= 200.0`, so the endpoint warning behavior cannot be
inferred from the numerical branch alone. Raw negative infinity returns the
low constant; positive infinity and NaN fall through to the high constant;
signed zero produces one common ice-branch result.

Pinned evidence covers the direct EnergyPlus EMS 30 C fixture
`4246.030243592 Pa`, the functional-API 24 C example near `2985 Pa`, low/ice/
liquid/high branch vectors, the rounded triple-point and endpoint boundaries,
IEEE inputs, and repeated/alternating calls. Exact Rust bits pin only the local
source transcription; external EnergyPlus and cross-platform `exp`/`log`
last-bit equivalence remain unclaimed.

The pre-existing private `energyplus_psychrometric_saturation_pressure_pa`
remains a guarded compatibility adapter rather than either canonical ticket.
It rejects non-finite values, truncates finite input to the default cache
representative, and now delegates the formula to the raw helper. Its two direct
callers and all downstream Weather, moisture, and IdealLoads call sites retain
their existing `Option<f64>` behavior; none are migrated by this checkpoint.

### `PsyPsatFnTemp_raw` (`psy_psat_fn_temp_raw`)

The canonical Rust numerical scaffold is
`ep_runtime::psychrometrics::energyplus_psy_psat_fn_temp_raw`. Raw range
warnings and statistics remain per-simulation state and are not emulated by
the pure helper.

<!-- routine-state-contract:v1 begin psy_psat_fn_temp_raw -->
PsyPsatFnTemp_raw

read_state:
- arguments `T` and `CalledFrom`; `EP_psych_stats` reads the per-state raw saturation-pressure call counter, while `EP_psych_errors` reads `WarmupFlag` on every raw evaluation and, for non-warmup `T <= -100.0` or `T >= 200.0`, reads the dedicated saturation-pressure error index and reads `CalledFrom` only for the first such warning

write_state:
- the piecewise saturation-pressure calculation writes no state; `EP_psych_stats` increments the per-state raw call counter before the range check, while `EP_psych_errors` mutates warning, error-index, and recurring-warning state only for out-of-range calls outside warmup

history_state_ownership:
- the numerical result has no cross-call history and is a pure function of raw `T` plus the selected non-IF97 or `EP_IF97` compile branch; `CalledFrom` never changes the result, while optional raw-call statistics and one saturation-pressure recurring-warning history belong to each `EnergyPlusData` instance

unsupported_state:
- `EP_psych_stats` raw-call counting plus `EP_psych_errors` warmup suppression, warning stream and totals, caller timestamp, dedicated recurrence index/count/min/max with `C` units, SQLite, and callback state

inactive_branches:
- with `EP_nocache_Psychrometrics`, the separately named routine 24 is absent and the same body is compiled as public `PsyPsatFnTemp` on the original unquantized input
- disabling `EP_psych_stats` removes the raw-call increment, while disabling `EP_psych_errors` removes the inclusive endpoint/out-of-range warning flow without changing the pressure result
- defining `EP_IF97` replaces the liquid-water polynomial and the above-200 C constant while leaving the ice branch unchanged; the standard EnergyPlus build does not select this branch

unsupported_active_branches:
- default errors-enabled warnings for `T <= -100.0` and `T >= 200.0`, including warmup suppression, first-versus-recurring behavior, caller context, and per-state recurrence, plus the optional statistics-enabled every-raw-call counter

not_claimed_branches:
- external EnergyPlus numerical parity, cross-platform `exp`/`log` last-bit and floating-point exception parity, the `EP_IF97` variant, exact diagnostic/statistics side effects, cached-wrapper or no-cache-public-interface parity, and downstream C API, EMS, moisture, weather, or HVAC call-site migration
<!-- routine-state-contract:v1 end psy_psat_fn_temp_raw -->

### `PsyPsatFnTemp` Cache Deferral (`psy_psat_fn_temp`)

The default wrapper owns 1,048,576 direct-mapped per-`EnergyPlusData` entries,
uses signed `bits(T) >> 28` tags, indexes with `tag & 0xFFFFF`, compares the full
tag, and evaluates routine 24 at a representative whose low 28 bits are zero.
Same-tag inputs alias; different tags sharing a hash evict and recompute. Cache
hits skip raw statistics, diagnostics, and `CalledFrom`, while the optional
public lookup counter still increments. A warmup miss can therefore suppress a
later non-warmup warning until eviction.

Each entry initializes to `iTdb = -1000` and `Psat = 0.0`. The reachable
negative-NaN tag `-1000` (representative bits `0xffffffc180000000`) is a fresh
false hit returning `0.0`; after a colliding overwrite, the same input misses
and raw NaN returns the high constant. This makes routine 25 numerically
history-dependent outside the physical finite domain. `InitializePsychRoutines`
refills the array, and cache lifecycle and independent-state ownership remain
unimplemented in Rust. The no-cache build additionally removes the named raw
interface and applies the body to original, unquantized `T`. Routine 25 cannot
advance until those cache, sentinel, compile-variant, diagnostic, statistics,
and state-isolation contracts have an explicit Rust owner and tests.

## Routines 26-27 Enthalpy/Pressure Saturation Deferral Boundary

Both enthalpy/pressure saturation tickets remain `source_mapped`. The
subsequent routines 28 and 30 promotion does not change this deferral. The
current private IdealLoads
`saturation_temperature_from_enthalpy_and_pressure_c` helper is not a
canonical analogue. It rejects non-finite enthalpy, non-finite pressure, and
nonpositive pressure, performs 80 bisections over `[-100, 200] C`, and composes
the guarded RH humidity-ratio adapter with an IdealLoads enthalpy helper.
EnergyPlus instead uses a nine-piece polynomial seed, conditionally performs a
secant correction through other stateful psychrometric routines, and exposes a
history-dependent default cache. Renaming or delegating either canonical
ticket to the private helper would change its source contract.

### `PsyTsatFnHPb_raw` (`psy_tsat_fn_h_pb_raw`)

The named raw routine exists only in the default cache-enabled build. It first
forms `HH = H + 17863.7` and gives the relative-error denominator a signed
nonzero floor: `Hloc = max(1.0e-5, H)` when `H >= 0.0`, otherwise
`min(-1.0e-5, H)`. A binary search over
`[-42400, -22138, -670.12, 27297, 75222, 183790, 475770, 1544500,
3835300, 45866000]` selects one of nine source-ordered Horner polynomials.
Only `HH < -42400` and `HH > 45866000` are clamped before the outer
polynomials; the errors-enabled diagnostic test is separately inclusive at
the two endpoints.

Pressure does not affect that seed. The correction runs only when
`abs(PB - 101330) / 101330 > 0.01`; finite values at or inside the exact
one-percent boundary keep the seed, and a NaN pressure also skips correction
because the ordered comparison is false. When the predicate is true, the
routine evaluates saturated enthalpy at the seed through
`PsyWFnTdbTwbPb(state, T1, T1, PB, CalledFrom)` and `PsyHFnTdbW`, then starts
a secant correction from `T2 = 0.9 * T1`. It accepts relative enthalpy error
`<= 1.0e-5` or exact `Y2 == Y1`. The source loop tests
`IterCount <= 30` before incrementing, so it can evaluate 31 iterations. If no
break assigns a corrected temperature, the returned `T` remains the original
polynomial seed rather than the last `T2`. A break on iteration 31 still leaves
`IterCount > 30`, so the first out-of-range diagnostic path can label that
successful break as nonconvergence.

That correction is not a pure arithmetic tail. Routine 39 calls the deferred
routine-25 saturation-pressure cache, owns statistics and two diagnostics
paths, and can fall back through routine 36 after a negative humidity result.
The seed itself uses the separately inventoried `F6` and `F7` routines 46 and
47. Implementing only the standard-pressure polynomial, or privately
duplicating those later routines, would neither preserve the named raw
interface nor follow the source-order checkpoint policy.
Although the cached-build raw and no-cache public bodies are textually the
same, their transitive routine-25 calls are not: the default build sees
cache-representative temperatures while the no-cache build evaluates the
original temperature. Bitwise raw/no-cache identity is therefore not assumed.

With default errors enabled, `HH <= -42400 || HH >= 45866000` warns only
outside warmup and updates the dedicated recurring-warning history. The local
`FlagError` is set only inside the first-warning block where that history index
is still zero. Consequently, only the first non-warmup out-of-range call emits
the initial-temperature continuation and can emit the severe nonconvergence
follow-up; later recurring calls and warmup calls cannot. Optional statistics
increment the `TsatFnHPb` counter on every raw evaluation but never accumulate
this routine's iteration count. Calls in the pressure-correction path
additionally read and mutate the nested routines' cache, diagnostic,
statistics, and caller-context state.

The direct v26.1 unit evidence covers all nine polynomial regions, the low and
high clamps, one cache miss, and the 91325 Pa correction result
`18.819 C`. It does not cover exact polynomial boundaries, the exact pressure
band edges, IEEE inputs, `Y2 == Y1`, 31-iteration failure, warmup and recurring
diagnostics, or nested cache effects. The C/Python functional example only
prints an approximate result, and the EMS fixtures do not assert a numerical
oracle.

### `PsyTsatFnHPb` (`psy_tsat_fn_h_pb`)

The default wrapper owns 1,048,576 direct-mapped entries per
`EnergyPlusData`. It treats each `f64` bit pattern as signed `Int64`, shifts
both tags right by 24 (`tsat_hbp_precision_bits = 28`), and indexes
`(H_tag ^ Pb_tag) & 0xFFFFF` while comparing both full tags. On a miss it
writes the tags and calls routine 26 with the original `H` and `Pb`, not
representatives with their low bits cleared. Different same-tag inputs
therefore reuse whichever original input populated the entry first; finite
inputs can have first-writer-dependent results. XOR collisions evict the
entry, and hits skip raw diagnostics, raw statistics, nested calls, and
`CalledFrom`.

Fresh entries are `(iH = 0, iPb = 0, Tsat = 0.0)`, so a lookup whose two tags
are both zero is an initial false hit returning zero without evaluating raw.
`InitializePsychRoutines` refills the array, but
`PsychrometricCacheData::clear_state()` does not reset it. The optional wrapper
statistics increment is also source-observable: the header increments
`TwbFnTdbWPb_cache`, not `TsatFnHPb`. The no-cache compile variant removes the
named raw routine and compiles the same raw body directly as public
`PsyTsatFnHPb` on every original input.

The upstream test labelled as a cache hit first stores `(H, 101325)` and later
looks up `(H, 101330)`; those pressures have different full tags, so the test
does not exercise a hit. There is no upstream sentinel, same-tag alias,
collision, initialization-versus-clear, independent-state, hit-side-effect,
or cached-versus-no-cache test.

Promotion of these tickets requires source-order implementations of `F6` and
`F7`; canonical routine-25, routine-36, and routine-39 dependency contracts;
all polynomial-boundary, clamp, signed-floor, IEEE, exact-pressure-band,
convergence, and failure vectors; per-state warning/statistics isolation; and
an exact two-input cache owner with sentinel, same-tag, collision, lifecycle,
wrong-counter, and hit-side-effect tests. Default-cache and
`EP_nocache_Psychrometrics` oracle evidence and downstream C API, EMS, coil,
heat-recovery, and HVAC migration remain separate requirements.

## CP56-10 Vapor-Density And Relative-Humidity Numerical Scaffold

This checkpoint advances `PsyRhovFnTdbRh` and `PsyRhFnTdbRhov` to
`state_mapped` while leaving their stateful saturation-pressure dependency and
the separately named `PsyRhFnTdbRhov_error` helper unimplemented. The
inventory is now 35 source-mapped and 18 state-mapped routines. The parent
inventory remains `status = "scaffold"`, `claim_level = "none"`, and all 53
routines remain outside the full-domain required set.

Both Rust helpers model the standard EnergyPlus build's numerical projection:
they truncate the saturation-pressure input to the routine-25 cache
representative whose low 28 bits are zero, evaluate the default non-IF97 raw
formula, and then use the original unquantized `Tdb` in the outer ideal-gas
arithmetic. They do not own the 1,048,576-entry cache, replay hit/miss side
effects, or represent the no-cache or IF97 compile variants. The pre-existing
`LBnd0C` forward and inverse helpers remain separate empirical-exponential
routines and are not aliases for routines 28 or 30.

Pinned evidence covers the EnergyPlus EMS vectors `(30 C, 0.5)` to
`0.015174171 kg/m3` and `(30 C, 0.01 kg/m3)` to `0.3295072808`, a non-grid
temperature that distinguishes the default cache representative from the
no-cache raw value, valid-domain round trips, unbounded forward RH inputs,
inverse correction branches, the positive-vapor lazy gate, absolute-zero
denominators, signed zero, NaN, and infinities. These are source-fixture and
local formula tests, not an external EnergyPlus domain claim.

### `PsyRhovFnTdbRh` (`psy_rhov_fn_tdb_rh`)

The source unconditionally calls `PsyPsatFnTemp(state, Tdb, CalledFrom)` and
then evaluates `(Psat * RH) / (461.52 * (Tdb + 273.15))`. It has no local
statistics counter, validation, clamp, or diagnostic branch. Negative and
supersaturated RH therefore remain linear inputs, and even zero or NaN RH
still performs the nested saturation-pressure lookup and its side effects.

<!-- routine-state-contract:v1 begin psy_rhov_fn_tdb_rh -->
PsyRhovFnTdbRh

read_state:
- arguments `Tdb`, `RH`, and `CalledFrom`; the default build reads the routine-25 signed temperature tag, direct-mapped cache entry, and optional public lookup counter, while a miss reads raw saturation-pressure statistics, warmup, range-warning history, and caller context

write_state:
- the outer ideal-gas arithmetic writes no state; every cached saturation-pressure lookup may increment its optional wrapper counter, while a miss overwrites one cache entry and may update raw-call statistics and non-warmup range-warning state

history_state_ownership:
- for ordinary finite inputs, the default non-IF97 numerical projection is a deterministic function of original `Tdb` and `RH` plus the tag-derived saturation-pressure representative; cache hit, miss, collision, initialization, and warning/statistics history belong to each `EnergyPlusData` instance

unsupported_state:
- the routine-25 cache array and lifecycle, signed tag/hash/sentinel/collision behavior, public and raw counters, warmup suppression, warning stream and totals, caller timestamp, recurrence state, SQLite, and callback state

inactive_branches:
- `EP_nocache_Psychrometrics` removes the nested cache and named raw interface, evaluates saturation pressure at original unquantized `Tdb`, and can change ordinary non-grid results
- disabling `EP_psych_stats` removes nested counters; disabling `EP_psych_errors` removes nested range diagnostics; defining `EP_IF97` replaces the liquid-water saturation-pressure branch

unsupported_active_branches:
- the default cache lookup, representative-temperature raw miss, hit suppression of `CalledFrom` and raw side effects, sentinel/collision/lifecycle behavior, errors-enabled out-of-range diagnostics, and optional statistics-enabled counters

not_claimed_branches:
- external EnergyPlus numerical parity, cached/no-cache/IF97 equivalence, exact cache or diagnostic/statistics side effects, and C API, EMS, HAMT, EMPD, surface, or other downstream migration
<!-- routine-state-contract:v1 end psy_rhov_fn_tdb_rh -->

### `PsyRhFnTdbRhov_error` Deferral (`psy_rh_fn_tdb_rhov_error`)

The errors-only helper remains `source_mapped`. It acts only for strict
`RHValue > 1.01` or, in an `else if` branch, strict `RHValue < -0.05`;
exact endpoints and NaN are no-ops. Warmup suppresses every mutation. High and
low excursions share one `RhFnTdbRhov` error index and recurring min/max
history, so whichever direction occurs first owns the only detailed
caller-aware warning; a later opposite excursion contributes only to the
shared recurring record. The high reset text says `100.0 %` and the low text
says `1%`. A per-simulation warning owner, alternating-direction recurrence
tests, and exact formatting/callback/SQLite evidence are required before this
ticket can advance.

### `PsyRhFnTdbRhov` (`psy_rh_fn_tdb_rhov`)

Optional statistics increment before any input gate. The numerical body calls
saturation pressure only when `Rhovapor > 0.0`, using the fixed nested caller
name `PsyRhFnTdbRhov` rather than the external `CalledFrom`. Negative values,
signed zero, negative infinity, and NaN vapor density return literal positive
zero without a nested lookup. A raw result inside `[0.0, 1.0]`, including
values below `0.01`, is returned unchanged. Only a negative result becomes
`0.01` and only a result above one becomes `1.0`; the optional error helper
sees the raw value before that correction and uses external `CalledFrom`.

<!-- routine-state-contract:v1 begin psy_rh_fn_tdb_rhov -->
PsyRhFnTdbRhov

read_state:
- arguments `Tdb`, `Rhovapor`, and `CalledFrom`; optional statistics read the routine-30 counter on every call, positive vapor reads routine-25 cache/raw state with a fixed nested caller name, and extreme raw RH reads warmup plus the shared routine-29 warning index and external caller context

write_state:
- the numerical calculation and ordered correction write no state; optional statistics increment the routine-30 counter before the positivity gate, positive vapor may mutate nested saturation-pressure cache/statistics/diagnostics state, and non-warmup extreme raw RH may mutate the shared high/low warning history

history_state_ownership:
- for a fixed compile variant, the numerical projection is an input function, while the routine-25 hit/miss history and routine-29 shared high/low recurrence history affect nested side effects and belong to each `EnergyPlusData` instance

unsupported_state:
- the routine-30 call counter; routine-25 cache, lifecycle, counters, and range diagnostics; routine-29 warmup suppression, warning stream and totals, scratch string, caller timestamp, shared recurrence index/count/min/max, SQLite, and callback state

inactive_branches:
- nonpositive or NaN `Rhovapor` skips saturation pressure and routine-29 dispatch but does not skip the optional routine-30 call counter
- `EP_nocache_Psychrometrics` evaluates exact-temperature saturation pressure on every positive-vapor call; disabling statistics or errors removes the corresponding state mutations; `EP_IF97` changes the liquid-water dependency

unsupported_active_branches:
- default cached saturation-pressure lookup and hit/miss side effects, optional every-call statistics, and default errors-enabled dispatch only for strict raw `RH < -0.05` or `RH > 1.01` with warmup and shared high/low recurrence

not_claimed_branches:
- external EnergyPlus numerical parity, cached/no-cache/IF97 equivalence, exact diagnostic/statistics/cache behavior, threshold nextafter messaging, and C API, EMS, HAMT, EMPD, room-air, surface, or other downstream migration
<!-- routine-state-contract:v1 end psy_rh_fn_tdb_rhov -->

## CP56-11 Humidity-Ratio Relative-Humidity Numerical Scaffold

This checkpoint advances `PsyRhFnTdbWPb` to `state_mapped` while leaving its
separately named `PsyRhFnTdbWPb_error` helper unimplemented. The inventory is
now 34 source-mapped and 19 state-mapped routines. The parent inventory
remains `status = "scaffold"`, `claim_level = "none"`, and all 53 routines
remain outside the full-domain required set.

The Rust helper preserves the source's unsimplified evaluation order:
unconditional saturation pressure first, first-argument-NaN-preserving
`max(dW, 1.0e-5)` semantics, degree of saturation
`W / (0.62198 * PWS / (PB - PWS))`, relative humidity
`U / (1.0 - (1.0 - U) * (PWS / PB))`, and correction only when the raw result
is outside `[0.0, 1.0]`. It models the ordinary-finite standard-build
projection by evaluating saturation pressure at the routine-25 cache
representative. It does not own or replay that cache.

Pinned evidence covers the EnergyPlus EMS vector
`(30 C, 0.01, 101325 Pa) -> 0.377598442`, the C/Python API's print-only
`(24 C, 0.009, 101325 Pa)` vector, a non-grid temperature that separates the
default cache representative from no-cache evaluation, valid-domain inverse
round trips, the humidity-ratio floor, low-positive preservation, high/low
correction, pressure zero and saturation singularities, NaN, and infinities.
These are source-fixture and local formula tests, not external EnergyPlus
domain, API, EMS-dispatch, or downstream integration claims.

### `PsyRhFnTdbWPb_error` Deferral (`psy_rh_fn_tdb_w_pb_error`)

The errors-only helper remains `source_mapped`. It acts only for strict
`RHValue > 1.01` or, in an `else if` branch, strict `RHValue < -0.05`; exact
endpoints and NaN are no-ops. Warmup suppresses every mutation. High and low
excursions share one `RhFnTdbWPb` error index and recurring min/max history,
so whichever direction occurs first owns the only detailed warning. The
reported humidity ratio is the already-floored `W`. The high reset text says
`100.0%`, the low text says `1%`, a nonempty caller has no trailing comma,
and an empty caller produces `Routine=Unknown,`. A per-simulation warning
owner, alternating-direction recurrence tests, and exact
formatting/timestamp/callback/SQLite evidence are required before this ticket
can advance.

### `PsyRhFnTdbWPb` (`psy_rh_fn_tdb_w_pb`)

Optional own statistics increment before all other work. Saturation pressure
is then looked up unconditionally, even for NaN humidity ratio or singular
pressure. An empty external `CalledFrom` becomes fixed `PsyRhFnTdbWPb` for
the nested saturation-pressure call, but the original empty view reaches the
routine-31 helper and therefore reports `Unknown`. A raw result in
`[0.0, 1.0]`, including positive values below `0.01` and signed zero, is
returned unchanged. Only a negative result becomes `0.01` and only a result
above one becomes `1.0`; the optional error helper sees the raw result before
that correction.

<!-- routine-state-contract:v1 begin psy_rh_fn_tdb_w_pb -->
PsyRhFnTdbWPb

read_state:
- arguments `TDB`, `dW`, `PB`, and `CalledFrom`; optional statistics read the routine-32 counter before all work, every call reads routine-25 signed tag/hash/cache-entry and wrapper-counter state, a miss reads raw saturation-pressure statistics/range-warning state plus nested caller context, and an extreme raw RH reads routine-31 warmup/shared-error-index and external-caller state

write_state:
- the humidity floor, unsimplified formula, and ordered correction write no state; optional own statistics increment on every call before saturation pressure, the nested lookup may increment its wrapper counter and on a miss overwrite a cache entry and mutate raw statistics/range diagnostics, and a non-warmup extreme raw RH may mutate routine-31 warning, scratch-string, and shared recurrence state

history_state_ownership:
- ordinary finite default non-IF97 output is a deterministic function of the original inputs and tag-derived saturation-pressure representative; cache hit/miss/collision history changes nested side effects, the reachable negative-NaN tag `-1000` can make full-source numerical output history-dependent through the fresh-entry sentinel, and cache plus high/low recurrence state belong to each `EnergyPlusData` instance

unsupported_state:
- the routine-32 call counter; routine-25 cache array, lifecycle, signed tag/hash/sentinel/collision behavior, public/raw counters, range diagnostics, warmup, and nested caller; routine-31 warning stream/totals, scratch string, caller timestamp, shared recurrence index/count/min/max, SQLite, and callback state

inactive_branches:
- an in-range raw RH skips routine-31 dispatch but never skips the optional own counter or unconditional saturation-pressure lookup; disabling statistics or errors removes the corresponding mutations
- `EP_nocache_Psychrometrics` evaluates saturation pressure at original unquantized `TDB` on every call and can change ordinary non-grid results; `EP_IF97` changes the liquid-water dependency

unsupported_active_branches:
- the default cached saturation-pressure lookup, representative miss, hit suppression of nested raw side effects/caller context, negative-NaN sentinel and lifecycle behavior, optional every-call statistics, and default errors-enabled strict `RH < -0.05` or `RH > 1.01` flow with warmup and shared high/low recurrence

not_claimed_branches:
- external EnergyPlus numerical parity, cached/no-cache/IF97 equivalence, full IEEE and negative-NaN sentinel parity, exact diagnostic/statistics/cache behavior, threshold nextafter messaging, API or EMS dispatch, and coil, heat-recovery, thermal-comfort, weather, zone, or other downstream migration
<!-- routine-state-contract:v1 end psy_rh_fn_tdb_w_pb -->

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

The `PsyRhoAirFnPbTdbW`, `PsyRhoAirFnPbTdbW_fast`, `PsyHfgAirFnWTdb`,
`PsyHgAirFnWTdb`, `PsyHFnTdbW`, `PsyHFnTdbW_fast`, `PsyCpAirFnW`,
`PsyCpAirFnW_fast`, `PsyTdbFnHW`, `PsyRhovFnTdbRhLBnd0C`,
`PsyRhovFnTdbWPb`, `PsyRhovFnTdbWPb_fast`, `PsyRhFnTdbRhovLBnd0C`,
`PsyVFnTdbWPb`, `PsyWFnTdbH`, `PsyPsatFnTemp_raw`, `PsyRhovFnTdbRh`,
`PsyRhFnTdbRhov`, and `PsyRhFnTdbWPb` tickets are `state_mapped`; the other
34 ledger routines remain `source_mapped`. All 53 retain
`required_for_full_domain = false`. Before any ticket is promoted further, its
Rust target, source-vector tests, compile-variant obligations, diagnostic
behavior where applicable, and external evidence boundary must be recorded.
This map adds no routine to the project-contract required set and does not
establish psychrometrics implementation or conformance completion.
