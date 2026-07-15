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
`PsyHgAirFnWTdb`, `PsyHFnTdbW`, `PsyHFnTdbW_fast`, `PsyCpAirFnW`, and
`PsyCpAirFnW_fast` tickets are `state_mapped`; the other 45 ledger routines
remain `source_mapped`. All 53 retain
`required_for_full_domain = false`. Before any ticket is promoted further, its
Rust target, source-vector tests, compile-variant obligations, diagnostic
behavior where applicable, and external evidence boundary must be recorded.
This map adds no routine to the project-contract required set and does not
establish psychrometrics implementation or conformance completion.
