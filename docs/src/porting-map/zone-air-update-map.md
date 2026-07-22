---
status: active
claim_level: planning-guard
owner: runtime
last_reviewed: 2026-07-14
---

# Zone Air Update Map

Reference version: EnergyPlus 26.1.0

Purpose: define the promoted official ExampleFile zone-air boundary and what
must remain outside the claim until broader EnergyPlus zone-air parity exists.

## Source Anchors

| EnergyPlus area | Source anchor | Rust target | Current status |
|---|---|---|---|
| zone predictor/corrector driver | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ManageZoneAirUpdates` | `manage_zone_air_updates_stage`; `advance_heat_balance_state_one_timestep` successor | CP195 required source-mapped dispatcher; Rust metadata and selector-ignoring closures remain scaffold |
| Zone setpoint and control input | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::GetZoneAirSetPoints` | bounded compiler thermostat/humidistat subset; `get_zone_air_set_points_compat` identity closure | CP196 required source-mapped 16-family transaction; no whole-routine Rust parity |
| adaptive-comfort weather running averages | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::CalculateMonthlyRunningAverageDryBulb` | typed EPW parser and hourly dry-bulb series only | CP197 non-required source-mapped raw-file helper; no Rust rolling-average or adaptive-control parity |
| adaptive-comfort setpoint schedule | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::CalculateAdaptiveComfortSetPointSchl` | none | CP198 non-required source-mapped strict ASH/CEN design-day and daily schedule writer; no Rust adaptive-control parity |
| Zone setpoint runtime initialization | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::InitZoneAirSetPoints` | `init_zone_air_set_points_compat` identity closure only | CP199 required source-mapped one-time allocation/output, environment reset, validation, and demand-limiting transaction; no whole-routine Rust parity |
| Zone/Space begin-environment history initialization | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::beginEnvironmentInit` | Zone-only run initialization and later bounded history state | CP200 required source-mapped 26-write four-slot environment reset; no exact Rust Zone/Space lifecycle parity |
| Zone/Space heat-balance output registration | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::setUpOutputVars` | adjacent Zone mean-air ResultStore series only | CP201 required source-mapped four-row Zone and simulation-Space OutputProcessor binding; no exact Rust identity, field, timestep, pointer, or lifecycle parity |
| Zone/Space system-load prediction dispatch | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::PredictSystemLoads` | `predict_system_loads_compat` identity closure around an adjacent Zone-only temperature/history update | CP202 required source-mapped staged/on-off control, Zone/Space child dispatch, and final mode memory; no exact Rust load or lifecycle parity |
| Zone/Space record-level load prediction | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::predictSystemLoad` | adjacent guarded Zone-only coefficient/capacitance/history helpers and a separate no-OA ThirdOrder humidity subset | CP203 required source-mapped coefficient, AFN/history, and sensible-then-moisture dispatch transaction; no exact Rust Zone/Space lifecycle or demand parity |
| Zone thermostat setpoint calculation | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::CalcZoneAirTempSetPoints` | `calc_zone_air_temp_set_points_compat` identity closure plus a constant-DualSetpoint IdealLoads diagnostic series | CP204 required source-mapped schedule/control branch, optimum-start, fault, comfort, and final EMS transaction; no heat-balance Rust implementation |
| mean air temperature histories | `MAT`, `XMAT`, `XM2T`, `XM3T`, `ZoneAirTemp` | `ZoneHeatBalanceState::previous_mean_air_temperatures_c` | placeholder history |
| air capacitance | zone volume, multipliers, moist-air density and specific heat | `ZoneHeatBalanceState::air_heat_capacity_j_per_k` plus psychrometric helper shell | promoted candidate updates `AirPowerCap` from weather-context pressure/RH proxy for the declared case; owned `ZoneAirHumRat` still pending for broader claims |
| internal convective gains | `InternalHeatGains.cc` | `simulate_zone_internal_convective_gains`, heat-balance gain input | convective gain case only |
| surface convection coupling | `HeatBalanceSurfaceManager.cc` | future surface convection aggregate | not ported |
| HVAC and infiltration coupling | zone equipment and air balance managers | future zone load inputs | not ported |

`HVACManager.cc` calls `ManageZoneAirUpdates` for setpoint acquisition,
prediction, and correction. It is a predictor/corrector orchestration routine,
not a direct child of the `ManageHeatBalance` -> `ManageSurfaceHeatBalance` ->
`ManageAirHeatBalance` call chain.

### CP195 `ManageZoneAirUpdates` source map

`ManageZoneAirUpdates(EnergyPlusData &state,
DataHeatBalFanSys::PredictorCorrectorCtrl const UpdateType,
Real64 &ZoneTempChange, bool const ShortenTimeStepSys,
bool const UseZoneTimeStepHistory, Real64 const PriorTimeStep)` is declared at
`ZoneTempPredictorCorrector.hh` lines 264-270 and implemented at
`ZoneTempPredictorCorrector.cc` lines 195-244. The selector enum is defined at
`DataHeatBalFanSys.hh` lines 70-80. Although the declaration comment mentions
only setpoint, prediction, and correction, the implementation accepts six
named work selectors plus a silent default.

Every entry executes the same ordered prefix before inspecting `UpdateType`:

1. When `DataZoneControlsData::GetZoneAirStatsInputFlag` is true, lines
   215-216 call CP196 `GetZoneAirSetPoints`.
2. Line 217 clears that shared latch only after CP196 returns.
3. Line 220 calls `InitZoneAirSetPoints` unconditionally, including for
   history and invalid/default selectors.
4. Lines 222-243 then dispatch at most one branch child.

| Selector | Selected child and forwarded arguments | Direct wrapper/output effect |
|---|---|---|
| `GetZoneSetPoints` | `CalcZoneAirTempSetPoints(state)` | leaves `ZoneTempChange` unchanged |
| `PredictStep` | `PredictSystemLoads(state, ShortenTimeStepSys, UseZoneTimeStepHistory, PriorTimeStep)` | forwards all three timestep inputs and leaves `ZoneTempChange` unchanged |
| `CorrectStep` | `correctZoneAirTemps(state, UseZoneTimeStepHistory)` | assigns the returned maximum temperature change to caller-owned `ZoneTempChange` only after the child returns |
| `RevertZoneTimestepHistories` | `RevertZoneTimestepHistories(state)` | leaves `ZoneTempChange` unchanged |
| `PushZoneTimestepHistories` | `PushZoneTimestepHistories(state)` | leaves `ZoneTempChange` unchanged |
| `PushSystemTimestepHistories` | `PushSystemTimestepHistories(state)` | leaves `ZoneTempChange` unchanged |
| `Invalid`, `Num`, or an out-of-range cast | no branch child | silently returns after the common input/init prefix and preserves `ZoneTempChange` |

The direct child boundaries are CP196 `GetZoneAirSetPoints` at lines 246-2174,
`InitZoneAirSetPoints` at lines 2350-2816, CP202 `PredictSystemLoads` at lines
2870-3145, CP203 `ZoneSpaceHeatBalanceData::predictSystemLoad` at lines
3146-3257, CP204 `CalcZoneAirTempSetPoints` at lines 3259-3460,
`correctZoneAirTemps` at lines 3817-3861,
`PushZoneTimestepHistories` at lines 4167-4185,
`PushSystemTimestepHistories` at lines 4277-4295, and
`RevertZoneTimestepHistories` at lines 4372-4389. CP195 does not normalize
their differing Zone/Space guards or state ownership. In particular, prediction
and history children generally use `doSpaceHeatBalance`, while the correction
wrapper's active Space correction requires
`doSpaceHeatBalanceSimulation && !DoingSizing`.

There are nine literal production calls in two parent routines.
`HVACManager::ManageHVAC` owns seven: Get at lines 224-229, initial Predict at
262-267, initial Correct at 294-299, shortened-system-step Predict at 346-351,
Correct at 374-379, PushSystem at 388-393, and the end-of-Zone-step PushZone at
579-584. Thus an adaptive Zone timestep can repeat the stateful
Predict/Correct/PushSystem sequence before one final Zone-history push.
`SimulationManager::Resimulate` lines 2915-2937 conditionally performs only
Get at 2917-2922 and Predict at 2929-2930 before `SimHVAC`; it supplies false
shortening, the current history selector, and zero prior timestep. No literal
production caller selects Revert. CP195 is therefore HVAC timestep and
resimulation orchestration, not a direct child of
`ManageHeatBalance -> ManageSurfaceHeatBalance -> ManageAirHeatBalance`. The
standard CP191 branch reaches the ordinary HVAC parent; an external HVAC
callback bypasses that parent unless external code invokes equivalent work.

CP195's only direct persistent state write is the shared input latch. Every
other mutation belongs to a child, except the caller-owned
`ZoneTempChange` assignment after a returned Correct child.
`correctZoneAirTemps` initializes a nonnegative maximum, corrects Zones and
eligible Spaces, and returns the maximum reached change; CP195 performs no
additional computation or range check. The other five named selectors and the
default leave the reference exactly as supplied.

A CP196 non-return preserves its parser/diagnostic prefix and the true input
latch, suppressing Init and dispatch; same-state retry enters CP196 again. An
Init non-return occurs after a successful latch clear, preserves its own
prefix, and suppresses dispatch; retry skips CP196 but reruns Init. A branch
non-return preserves both common phases plus the branch prefix. In the Correct
case the outer assignment has not completed, so the old `ZoneTempChange`
survives. Warnings and recurring diagnostics inside children can return
normally and are not translated by CP195. The wrapper has no selector
validation, error/status argument, catch, cleanup, rollback, or transaction.

After successful one-time input, normal repeat skips CP196 but always reruns
Init and the selected child. Prediction, correction, and all history operations
are stateful; push and revert calls in particular are not idempotent.
`GetZoneAirStatsInputFlag` is owned at `DataZoneControls.hh` line 287 and reset
true by `DataZoneControlsData::clear_state` at lines 304-325.
`ZoneTempPredictorCorrectorData::clear_state` placement-news the predictor
latches, errors, histories, and Zone/Space arenas at
`ZoneTempPredictorCorrector.hh` lines 441-444. Clearing only the control owner
rearms input over possibly retained predictor/output state, while clearing only
the predictor owner leaves input skipped. A clean replay also requires the
relevant HeatBalFanSys, ZoneEnergyDemand, HeatBalance, RoomAir, LoopNode,
OutputProcessor, schedule, environment, and child-specific owners to be reset
and reconstructed.

The C++ unit tree contains zero direct CP195 calls and no test names a
`PredictorCorrectorCtrl` selector. Direct child calls total nine Get, four
Init, 21 setpoint calculation, 16 Predict, and five Correct calls, with zero
calls to any of the three history children. Those fixtures cover selected
thermostat/setpoint values, Get failure diagnostics, shortened Predict inputs,
and bounded hybrid-model correction state. They do not cover the wrapper's
common-prefix order, successful latch clear, exactly-one-child dispatch,
invalid/default path, `ZoneTempChange` preservation or overwrite, production
caller sequence, history semantics, partial failure, retry, or coordinated
reset.

Rust's `manage_zone_air_updates_stage` at
`zone_predictor_corrector.rs` lines 40-46 is metadata.
`manage_zone_air_updates_compat` at lines 55-60 ignores its
`_update_type` and passes one arbitrary closure through an identity source-order
wrapper. The Rust selector enum omits Invalid and Num, and the wrapper has no
shared input latch, unconditional Init call, switch/default behavior,
`ZoneTempChange` reference, selector validation, status, or rollback. Live
timestep code invokes only Predict and Correct selector shells. The Predict
closure manually nests empty Get/Init/Calc wrappers and a Zone-history push
around bounded coefficient/load work; the Correct closure performs bounded
temperature/humidity work and optional history synchronization. These
caller-assembled closures are not CP195's six-way dispatch topology. Rust tests assert
metadata/prebinding, arbitrary identity-wrapper ordering, one bounded adaptive
history result, and isolated coefficient/analytical/third-order numerics, but
none calls the selector wrapper or proves latch, dispatch, failure, retry, or
reset parity.

CP195 therefore keeps the existing
`zone_temp_predictor_corrector_source_order.routine.manage_zone_air_updates`
required and `source_mapped`, while the algorithm remains `scaffold` with
`claim_level = none`. It adds no routine or project entry, EnergyPlus source
inventory, Rust target, code, state, test, support, capability, output
implementation, comparator, manifest, numerical, performance, or conformance
promotion. Existing promoted MAT and related bounded output evidence remains
adjacent result evidence, not dispatcher/lifecycle parity. The inventory stays
at 32 algorithms and 203 routines, split 58 `state_mapped` plus 145
`source_mapped`, with 82 required; the heat-balance project list stays 51.

### CP196 `GetZoneAirSetPoints` source map

`GetZoneAirSetPoints(EnergyPlusData &state)` is declared at
`ZoneTempPredictorCorrector.hh` line 272 and implemented at
`ZoneTempPredictorCorrector.cc` lines 246-2174. It has no return value,
explicit `return`, status argument, or internal latch. The local
`ErrorsFound` starts false at line 274; most input errors accumulate while
later phases continue, and only lines 2171-2172 convert that aggregate into a
fatal. The routine itself never reads or clears
`GetZoneAirStatsInputFlag`.

This is one fall-through input transaction, not a single thermostat reader.
The direct body selects 15 InputProcessor object families in 18
`getObjectItem` passes, while its unconditional HybridModel child owns the
sixteenth family:

| Ordered phase | Source lines | Principal work |
|---|---|---|
| ordinary thermostat pre-scan and expansion | 324-520 | counts `ZoneControl:Thermostat` objects, resolves Zone or ZoneList targets, allocates expanded controlled-Zone state, binds the control-type schedule and up to four setpoint-type/name pairs, and records cutout delta |
| four ordinary setpoint families | 521-671 | reads SingleHeating, SingleCooling, SingleHeatingOrCooling, and DualSetpoint names and schedule pointers |
| ordinary reference and schedule validation | 673-808 | links named setpoint records, validates control types required by schedules, and contains an effectively inert missing-type warning pass |
| humidity control | 809-873 | reads `ZoneControl:Humidistat`, binds the Zone and humidifying/dehumidifying schedules, and writes the Zone inverse index |
| thermal-comfort thermostat | 874-1164 | expands Zone/ZoneList targets, validates People inputs and averaging, dry-bulb bounds, the comfort control schedule, and up to four comfort-type/name pairs |
| four Fanger setpoint families | 1166-1330 | binds `[-3,3]` PMV schedules for the four comfort control types |
| comfort reference and schedule validation | 1332-1447 | links comfort heat/cool schedules and checks schedule/type consistency |
| Hybrid Model input | 1449-1450 | unconditionally calls `HybridModel::GetHybridModelZone(state)` even when the local aggregate already holds errors |
| Zone capacitance multipliers | 1452-1566 | assigns sensible, moisture, CO2, and generic-contaminant multipliers and always writes their averaged EIO row |
| operative temperature and adaptive comfort | 1568-1757 | applies Constant or Scheduled radiative fractions, conditionally builds adaptive schedules through CP197 and CP198, and registers operative-temperature outputs |
| temperature-and-humidity overcool | 1758-1918 | binds dehumidification, overcool mode/range, and control-ratio state through parent-thermostat or expanded-Zone paths |
| staged dual setpoint | 1919-2169 | expands staged controls, binds base schedules, stage counts, throttling ranges, ordered offsets, and `StageZoneLogic` |
| terminal decision | 2171-2174 | fatals on the accumulated local flag or falls through normally |

The ordinary thermostat pre-scan stores `TStatObjects` and the expanded
count. Any invalid Zone/ZoneList name zeros the complete
`NumTempControlledZones` at lines 372-376, suppressing even otherwise-valid
second-pass entries. A successful second pass rejects duplicate Zone
assignment, writes `Zone(...).TempControlledZoneIndex`, checks the control
schedule only over `[0,4]`, and accepts up to four type/name pairs. Repeating a
type overwrites its earlier name. A positive cutout delta increments
`NumOnOffCtrZone`; its combination with SingleHeatingOrCooling warns but
returns. The four setpoint families bind schedule pointers but impose no local
temperature-value bounds. Missing referenced objects and schedule-required
types become terminal errors. A control schedule that is always zero emits a
severe message without setting `ErrorsFound`. The final ordinary
`MustHave`/`DidHave` warning pass has no producer for either flag in this
routine and is effectively inert.

Humidistat input allocates its own arena and uniqueness set, resolves direct
Zones, writes `humidityControlZoneIndex`, and requires the humidifying
schedule. A blank dehumidifying field aliases the humidifying pointer at lines
865-867; no relative-humidity schedule value range is checked here.

Thermal-comfort input repeats Zone/ZoneList expansion, requires a People
object for every valid controlled Zone, and checks activity `[72,909]`,
work-efficiency `[0,1]`, clothing `[0,2]`, air-velocity presence, dry-bulb
bounds `[0,50]`, and a `[0,4]` control schedule. SpecificObject validates only
global People-name presence, not same-Zone ownership. The activity-range
severe and equal dry-bulb min/max severe do not set the local fatal flag.
Comfort keyword diagnostics pass the field name rather than the invalid value,
and the later reference link indexes the `FindItem` result without a local
zero guard. The comfort `DidHave` path mirrors declared types rather than
actual schedule values, so its final missing-type warning is likewise
effectively inert.

`HybridModel::GetHybridModelZone` can fatal before all later phases and before
CP196's aggregate decision. The capacitance phase writes 1.0 to every Zone
when no object exists. With objects, a blank target updates the rolling default
and later occurrences win; named Zone or ZoneList records mark customized
Zones, and remaining Zones receive the final default. The ZoneList loop at
lines 1511-1518 uses `ZonePtrNum < NumOfZones` and therefore omits the final
member. The four arithmetic inputs receive no local range or finite check.
Even with prior accumulated errors, lines 1565-1566 append the averaged
multiplier EIO header and row.

Operative-temperature input sets `AnyOpTempControl` from object presence and
accepts a parent thermostat name, expanding it across all member Zones, or one
expanded controlled-Zone name. Constant and Scheduled are the only valid
modes; fixed and scheduled radiative fractions are constrained to
`[0,0.9)`. A non-None adaptive model can call CP197
`CalculateMonthlyRunningAverageDryBulb` and CP198
`CalculateAdaptiveComfortSetPointSchl` once under the shared
`AdapComfortDailySetPointSchedule.initialized` guard; only CP198 sets that
flag true. A missing weather file fatals immediately in CP197. Output registration
differs by branch: the parent-name path registers every expanded Zone even
after an invalid mode, while the direct-name path registers only for a valid
Constant or Scheduled mode.

Temperature-and-humidity input also has distinct direct-expanded-Zone and
parent-thermostat branches. Both bind dehumidification and constrain scheduled
overcool range to `[0,3]`, constant range to `[0,3]`, and control ratio only
from below. The direct Scheduled branch does not store the input control
ratio, whereas the parent branch always stores and checks it. Diagnostics are
generally emitted only for the first Zone of a parent expansion.

The staged pre-scan does not use a staged-local error flag. Lines 1971-1974
test the routine-wide `ErrorsFound`, so any earlier fatal-target error adds the
staged invalid-name diagnostic, zeros `NumStageCtrZone`, and suppresses the
entire staged second pass. In the ZoneList branch the start pointer is written
to `TempControlledZoneStartPtr` instead of
`StageControlledZoneStartPtr`. Stage counts are assigned from real inputs to
integers before the original `[1,4]` values are range-checked. Heating offsets
must be nonpositive and strictly decrease; cooling offsets must be nonnegative
and strictly increase. The heating throttling-range check reads numeric field
1 instead of the stored field 2. An unresolved staged Zone leaves
`ActualZoneNum` at zero, but line 2020 still attempts
`StageZoneLogic(0) = true` before continuing. Missing all four compatible
unitary or
one-stage setpoint-manager families produces only a warning.

There are three literal production call sites. CP195
`ManageZoneAirUpdates` lines 215-217,
`KivaManager::setupKivaInstances` at
`HeatBalanceKivaManager.cc` lines 672-674, and
`VerifyThermostatInZone` lines 5689-5691 all test the same
`GetZoneAirStatsInputFlag` and clear it only after CP196 returns. The first
successful caller owns the one-time transaction; the other two then skip it.
`VerifyThermostatInZone` is reached from `ZoneEquipmentManager.cc` line 812.
CP196 itself neither knows which caller won nor clears the latch.

Most validation failures retain allocated arenas, linked schedules, Zone
indices, flags, multiplier writes, output registrations, EIO and diagnostic
prefixes before the terminal fatal. A Hybrid or adaptive helper fatal exits
earlier with only the reached prefix. There is no catch, cleanup, rollback, or
same-state retry guard. Because every production wrapper leaves the latch true
on non-return, a caught failure re-enters CP196 against its partially allocated
state and is not idempotent. `DataZoneControlsData::clear_state` resets the
latch and its control arenas, while
`ZoneTempPredictorCorrectorData::clear_state` separately reconstructs counts,
schedule arrays, uniqueness and adaptive state. Clean replay also requires
Zone, HybridModel/RoomAir, OutputProcessor, InputProcessor, schedule, weather,
file/output, and diagnostic owners to be reset and reconstructed.

The EnergyPlus unit tree directly calls CP196 nine times across six files.
Six calls principally prepare Kiva, heat-pump, availability, or unitary
fixtures. `ZoneTempPredictorCorrector_ReportingTest` covers valid ordinary
control families through later setpoint/load assertions;
`ZoneTempPredictorCorrector_WrongControlTypeSchedule` asserts the terminal
schedule mismatch plus the coupled staged diagnostic; and
`GetZoneAirSetPoints_Test` asserts the missing setpoint-reference failure.
No direct test covers repeat/retry, shared-latch timing, partial state after
failure, reset, a valid staged path, overcool input, positive ZoneList
expansion, capacitance EIO, or the complete operative/comfort transaction.
CP197 has no direct unit call; its later schedule helper is tested separately
and does not establish CP196 topology.

Rust's `get_zone_air_set_points_compat` at
`zone_predictor_corrector.rs` lines 62-65 only executes an arbitrary closure.
Its one live use at `heat_balance/timestep.rs` lines 302-306 manually nests
the Init and Calc identity wrappers; no test calls the alias directly.
`ExecutionStep::EvaluateZoneThermostat` is planning metadata inside the broader
zone-update stage, not a CP196 input step.

The compiler provides adjacent typed subsets at `compiler.rs` lines
12161-12372 for `ThermostatSetpoint:DualSetpoint`, direct-Zone
`ZoneControl:Thermostat`, and `ZoneControl:Humidistat`. It accepts only the
DualSetpoint control type, cannot expand ZoneLists, requires both humidistat
schedules instead of source aliasing, rejects duplicate names rather than
preserving source lookup behavior, and omits source counts, inverse Zone
indices, schedule-domain/type checks, all other input families, Hybrid and
adaptive work, EIO/output registration, ordered diagnostics, partial state,
latch, retry, and reset. The CLI's bounded IdealLoads consumer selects the
first dual control and ignores the control-type schedule. Parser, graph, and
execution-plan tests plus a non-conformance thermostat smoke case therefore
remain adjacent evidence only.

CP196 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.get_zone_air_set_points`
immediately after `routine.manage_zone_air_updates` and adds
`get_zone_air_set_points` at the same project-contract boundary. The algorithm
remains a `scaffold` with `claim_level = none`. This checkpoint adds no new
EnergyPlus source inventory, Rust target, code, mapped state, support,
capability, output implementation, comparator, manifest, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 204 routines, split 58 `state_mapped` plus 146 `source_mapped`, with 83
required; the heat-balance project list becomes 52.

### CP197 `CalculateMonthlyRunningAverageDryBulb` source map

`CalculateMonthlyRunningAverageDryBulb(EnergyPlusData &state,
Array1D<Real64> &runningAverageASH, Array1D<Real64> &runningAverageCEN)` is
declared at `ZoneTempPredictorCorrector.hh` line 284 and implemented at
`ZoneTempPredictorCorrector.cc` lines 2176-2275. It returns `void`, trusts
both caller arrays to be one-based and at least `NumDaysInYear` long, and has
no status, error argument, internal initialized flag, array clearing, bounds
validation, catch, or rollback.

The body is a fixed positional file transaction:

| Ordered phase | Source lines | Principal work |
|---|---|---|
| local allocation | 2193-2200 | allocates zeroed `adaptiveTemp` and `dailyDryTemp` arrays of `NumDaysInYear`; `adaptiveTemp` is never used |
| path check, open, and fixed skip | 2202-2207 | checks `inputWeatherFilePath`, opens a transient input stream as `CalcThermalComfortAdaptive`, and discards nine lines |
| hourly extraction and daily buckets | 2208-2222 | reads exactly `NumDaysInYear * 24` more lines, removes six comma fields, converts field 7 through `StrToReal`, adds one twenty-fourth to a positional daily bucket, and closes the stream |
| ASH running outputs | 2224-2249 | fills the caller-owned nominal 30-day array with separate wrapped and non-wrapped loops |
| CEN running outputs | 2251-2268 | fills the caller-owned nominal 7-day array with separate wrapped and non-wrapped loops |
| missing-path terminal | 2270-2275 | emits a fatal for a nonexistent weather path or falls through normally |

There are exactly two production call occurrences, both inside CP196
`GetZoneAirSetPoints` while reading
`ZoneControl:Thermostat:OperativeTemperature`. The expanded thermostat branch
at lines 1658-1662 calls CP197 at line 1661, and the direct controlled-Zone
branch at lines 1727-1732 calls it at line 1731. Each site requires a valid
non-None adaptive model and the shared
`AdapComfortDailySetPointSchedule.initialized` flag to be false, creates fresh
zero-filled ASH and CEN arrays, calls CP197, and immediately passes both arrays
to CP198 `CalculateAdaptiveComfortSetPointSchl`. CP197 neither reads nor writes
that flag; only CP198 sets it true at line 2347. Once CP198 returns, later
operative-control objects skip both helpers.

The routine does not use WeatherManager's decoded records or calendar. It
assumes 24 rows per day, ignores record dates, hours, data periods,
records-per-hour, leap metadata, and all `readLine` EOF/good flags. EnergyPlus
v26.1's canonical EPW readers identify eight header records, but CP197
unconditionally consumes nine. With an ordinary EPW, daily bucket `d` for
`1..N-1` therefore contains day `d` hours 2-24 plus day `d+1` hour 1. Bucket
`N` contains its final 23 hours plus one empty EOF read. `StrToReal` returns
`-99999.0` for empty or invalid text without a CP197 diagnostic, so that last
bucket is corrupt; neither running loop ever references bucket `N`. Missing
commas, truncated data, invalid numeric text, EPW missing-value sentinels,
extra rows, and a state/file day-count mismatch likewise receive no local
validation.

The running-window arithmetic is also source-specific rather than a clean
30-day/7-day mean:

- For ASH output day `d <= 31`, the wrapped path adds days `1..d-1` and
  `N+d-31..N-1`, giving 30 values and excluding bucket `N`, then divides by 30.
- For `d >= 32`, it adds `d-31..d-1` inclusively, giving 31 values, and still
  divides by 30.
- For CEN output day `d <= 8`, the wrapped path adds days `1..d-1` and
  `N+d-8..N-1`, giving 7 values and excluding bucket `N`, then divides by 7.
- For `d >= 9`, it adds `d-8..d-1` inclusively, giving 8 values, and still
  divides by 7.

ASH day 31 and CEN day 8 have the intended sample count and consecutive
bucket-index range, but those buckets still reflect the one-hour EPW shift;
the transition on the following day changes the sample count. Each output cell
self-accumulates as `x = x + avgDryBulb` and then uses `/=`. Production
callers avoid retained input only by allocating fresh zeros, while a direct
call that reuses either output array is non-idempotent.

A nonexistent path fatals at lines 2271-2273 before either output array is
touched. An existing but unreadable path fatals from the open helper at the
same pre-output phase. A readable malformed or short file normally completes
with sentinel-contaminated values; CP198 can then allocate and commit adaptive
schedules and set the shared flag, suppressing ordinary recalculation. A CP197
fatal prevents CP198, leaves that flag false, and propagates through CP196; the
production wrapper also leaves `GetZoneAirStatsInputFlag` true because it
clears only after CP196 returns. CP197 owns no persistent reset state itself.
Standalone replay needs a restored readable file plus fresh or cleared caller
arrays. Replay through CP196 additionally needs the already documented
ZoneControls/input prefix reset, and committed adaptive state is reset only by
full `ZoneTempPredictorCorrectorData::clear_state` reconstruction.

No EnergyPlus unit test directly or indirectly executes CP197. The
`ZoneTempPredictorCorrector_AdaptiveThermostat` fixture calls CP198 three times
with synthetic constant arrays, so it proves neither file extraction nor any
running-window branch. The nine direct CP196 fixture calls do not reach an
adaptive operative-control branch. Missing/open failure, nine-line skip,
malformed fields, EOF, day-count mismatch, wrapped and regular arithmetic,
reused arrays, guard timing, retry, and reset are all uncovered.

Rust has no adaptive-comfort or ASH/CEN running-average implementation, state,
or operative-temperature control type. `weather.rs` lines 10, 144-190, and 1060-1213 provide typed EPW records, a
`DATA PERIODS`-aware rich parser, a fixed-eight-header legacy dry-bulb loader,
and numeric conversion errors; `WeatherTimestepSeries` lines 257-458 retains
hourly dry bulb and builds interpolated timestep samples. Focused tests at
`runtime/tests/part02.rs` lines 521-568 prove two dry-bulb values after eight
headers. These APIs preserve different parsing and failure semantics and never
form CP197's positional daily buckets, cyclic windows, sentinels, output-array
mutation, or shared adaptive lifecycle. Execution-plan thermostat metadata and
the compiler's direct-Zone DualSetpoint/humidistat subset add no adjacent
adaptive implementation.

CP197 adds non-required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.calculate_monthly_running_average_dry_bulb`
immediately after `routine.get_zone_air_set_points`. The heat-balance project
contract remains unchanged because this is an optional adaptive-comfort child.
The algorithm remains a `scaffold` with `claim_level = none`. This checkpoint
adds no new EnergyPlus source inventory, Rust target, code, mapped state,
support, capability, output implementation, comparator, manifest, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 205 routines, split 58 `state_mapped` plus 147 `source_mapped`, with 83
required; the heat-balance project list remains 52.

### CP198 `CalculateAdaptiveComfortSetPointSchl` source map

`CalculateAdaptiveComfortSetPointSchl(EnergyPlusData &state,
Array1D<Real64> const &runningAverageASH,
Array1D<Real64> const &runningAverageCEN)` is declared at
`ZoneTempPredictorCorrector.hh` lines 286-287 and implemented at
`ZoneTempPredictorCorrector.cc` lines 2277-2348. It returns `void`, leaves
both const caller arrays untouched, trusts them to be one-based and at least
`NumDaysInYear` long, and owns no status, error argument, diagnostic, file I/O,
catch, cleanup, or rollback.

The body preserves this fixed order:

| Ordered phase | Source lines | Principal work |
|---|---|---|
| locals and state alias | 2288-2291 | fixes summer design-day type index 9, zeros the local gross temperature, and aliases predictor/corrector state |
| shared design-day vector | 2293-2314 | scans every stored design day and conditionally writes ASH slots 0-2 and CEN slots 3-6 |
| daily allocation | 2316-2322 | allocates three ASH arrays followed by four CEN arrays, each sized to `NumDaysInYear` |
| daily values | 2325-2346 | visits days `1..NumDaysInYear` and writes the three ASH cells before the four CEN cells |
| commit flag | 2347 | sets `AdapComfortDailySetPointSchedule.initialized = true` only after all prior work returns |

There are exactly two production call occurrences, both inside CP196
`GetZoneAirSetPoints` operative-temperature input branches. Lines 1661-1662
call CP197 then CP198 for an expanded thermostat target, and lines 1731-1732
do the same for a direct controlled Zone. Each site first requires a valid
non-None adaptive model and the shared initialized flag to be false, and
allocates fresh zeroed running-average arrays before CP197 fills them. CP198
has no local guard and therefore runs on a direct call even when the flag is
already true. A design-day-only adaptive case still enters CP197 first and
therefore still requires its weather file.

For a summer design day, the gross approximate dry bulb is `(MaxDryBulb + (MaxDryBulb - DailyDBRange)) / 2 = MaxDryBulb - DailyDBRange / 2`.

Only `DayType == 9` participates. Strict `10 < T < 33.5` writes the ASH
central, 90-percent upper, and 80-percent upper slots 0-2 as
`0.31 * T + 17.8`, `0.31 * T + 20.3`, and `0.31 * T + 21.3`. Strict
`10 < T < 30` independently writes CEN central and categories I-III slots 3-6
as `0.33 * T + 18.8`, `+20.8`, `+21.8`, and `+22.8`. The three standalone
semicolons among the CEN assignments are inert.

The vector is shared across all summer design days rather than stored per day.
The scan has no break or invalid branch, so the last qualifying design day wins
independently for each family. ASH and CEN can consequently retain values from
different design days; for example, exact 30 rewrites ASH while CEN retains an
earlier/default group. Nonsummer, out-of-range, and nonfinite design values
perform no write. The member declaration
`std::array<Real64, 7> AdapComfortSetPointSummerDesDay = {-1}` uses C++
aggregate initialization, yielding `[-1, 0, 0, 0, 0, 0, 0]` rather than seven
`-1` sentinels. Full predictor/corrector owner reconstruction is the only
local reset.

The daily schedule uses the same formulas but has explicit invalid branches:

| Family | Open valid range | Written values in fixed order | Otherwise |
|---|---|---|---|
| ASH | `10 < runningAverageASH(day) < 33.5` | `0.31*T + 17.8`, `+20.3`, `+21.3` | all three `-1` |
| CEN | `10 < runningAverageCEN(day) < 30` | `0.33*T + 18.8`, `+20.8`, `+21.8`, `+22.8` | all four `-1` |

Thus exact 10, exact 30 for CEN, exact 33.5 for ASH, NaN, and either infinity
take their family's `-1` branch. Longer input tails are ignored. A complete
pass overwrites every daily cell; design slots remain sticky where no
qualifying write occurs. The final initialized assignment is the only
source-authored commit boundary.

Although CP198 emits no authored error, a low-level allocation failure, invalid
owner state, or undersized input can prevent return. Design slots are reached
before any daily allocation, allocations are sequential, and each daily
iteration writes ASH before CEN. Such a non-return can retain a design prefix,
an allocation prefix, and daily writes while leaving the production flag
false. There is no rollback. On a direct failed repeat, the flag can instead
remain true from a prior success alongside partial new state because CP198
does not clear it at entry.

CP198 also precedes CP196's overcool, staged-dual, and accumulated-error fatal
tail. If CP198 succeeds and CP196 later fatals, the schedules and true flag
remain committed while the outer `GetZoneAirStatsInputFlag` remains true.
Same-state retry reruns CP196 but skips CP197 and CP198. A clean replay needs
the broader CP196 owners reset plus the placement-new
`ZoneTempPredictorCorrectorData::clear_state` reconstruction at header lines
441-443.

The only production consumer of the committed arrays and design vector is
`AdjustOperativeSetPointsforAdapComfort` at lines 5899-5964. It is called from
`CalcZoneAirTempSetPoints` for SingleCool line 3331, SingleHeatCool line 3347,
and the high/cooling half of DualHeatCool line 3379. Run-period environments
select one daily array by adaptive model and `DayOfYear`. DesignDay and
HVACSizeDesignDay environments use the shared vector only when the current
design day is type 9; model indices 2-8 map to slots 0-6.

That downstream consumer first truncates the incoming `Real64` baseline into
an `int`. After choosing a candidate, it restores that integer when the
candidate is lower and then when it equals `-1`. Consequently an invalid
schedule normally restores a truncated baseline, a valid candidate between
the truncated and original values can lower the original, and NaN passes both
tests. These are consumer semantics, not CP198 validation. The result feeds
`setptAdapComfortCool` before operative-to-air conversion and the registered
`Zone Adaptive Comfort Operative Temperature Set Point` output.

`ZoneTempPredictorCorrector_AdaptiveThermostat` is the only C++ fixture that
calls CP198. It invokes the routine three times on the same owner despite the
first successful call setting the flag: uniform zero and 40 arrays assert
seven day-1 `-1` values, then uniform 25 arrays assert the seven day-1 formulas
and the true flag. One summer design day with maximum 30 and range 10 produces
gross 25, but the test asserts only design slots 0 and 3. Its four downstream
consumer calls cover central ASH, central CEN, a manually forced `-1` fallback,
and one dual-setpoint case. Days 2 through N, leap years, zero/short/long input
shape, strict thresholds, mixed and nonfinite inputs, five design outputs,
no/multiple/nonsummer design days, asymmetric defaults, independent
last-writer retention, partial failure, CP197 integration, production guard,
retry, and reset remain uncovered.

Rust contains no adaptive-comfort, ASH55/CEN15251, operative-temperature
thermostat, CP198 formula, state, or output implementation. Its typed
thermostat/compiler boundary covers only direct-Zone DualSetpoint and
humidistat subsets. `EvaluateZoneThermostat` is execution-plan metadata and
the pipeline occurrence formats a trace label; neither is a schedule
calculation or consumer.

CP198 adds non-required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.calculate_adaptive_comfort_set_point_schl`
immediately after `routine.calculate_monthly_running_average_dry_bulb`. The
heat-balance project contract remains unchanged because this helper is reached
only by optional adaptive operative-temperature input. The algorithm remains
a `scaffold` with `claim_level = none`. This checkpoint adds no EnergyPlus
source inventory, Rust target, code, mapped state, support, capability, output
implementation, comparator, manifest, numerical, performance, or conformance
promotion. The inventory becomes 32 algorithms and 206 routines, split 58
`state_mapped` plus 148 `source_mapped`, with 83 required; the heat-balance
project list remains 52.

### CP199 `InitZoneAirSetPoints` source map

`InitZoneAirSetPoints(EnergyPlusData &state)` is declared at
`ZoneTempPredictorCorrector.hh` line 274 and implemented at
`ZoneTempPredictorCorrector.cc` lines 2350-2816. It returns `void` and combines
one-time allocation and output binding, environment/day lifecycle, controlled
Zone verification, demand-limit mutation, and fatal handling without a status
argument, catch, cleanup, rollback, or transaction.

The body preserves these source-ordered phases:

| Ordered phase | Source lines | Principal work and commit |
|---|---|---|
| aliases and counts | 2367-2376 | binds predictor/corrector and HeatBalFanSys state plus the current Zone count |
| one-time arrays and outputs | 2378-2618 | allocates/dimensions cross-owner state, warning-checks surface reference-air consistency, registers output bundles, then clears `InitZoneAirSetPointsOneTimeFlag` |
| begin-environment reset | 2622-2665 | resets selected Zone/Space heat-balance, setpoint, demand, deadband, return-air, and hybrid state, then clears `MyEnvrnFlag` |
| environment rearm | 2668-2670 | sets `MyEnvrnFlag` true whenever `BeginEnvrnFlag` is false |
| begin-day latch | 2672-2679 | toggles `MyDayFlag` without initializing any other data |
| ordinary temperature controls | 2681-2741 | optionally verifies equipment configuration and applies strict demand clamps |
| comfort controls | 2743-2807 | optionally verifies configuration and applies inclusive demand clamps after the ordinary loop |
| sticky fatal and checked tail | 2809-2815 | fatals on retained `ErrorsFound`; otherwise marks controls checked when equipment input is ready |

There are two literal production call occurrences. CP195
`ManageZoneAirUpdates` line 220 invokes CP199 unconditionally after its
optional CP196 input/latch phase and before every update-type dispatch.
HVACManager therefore reaches CP199 on Get, Predict, Correct, shortened-step
Predict/Correct/PushSystem repeats, and final PushZone calls. DemandManager
updates thermostat demand flags after the first HVAC simulation pass, so the
following Correct call can be the first CP199 entry that applies those clamps.

CP192 `initializeForExternalHVACManager` line 4612 is the other caller. It
jumps directly to CP199 when an external callback exists and the separate
global initialized flag is false; it does not acquire Zone setpoint input
first. The external initializer also does not set that global flag, so CP199
can be called before each callback while its own completed one-time phase
remains suppressed. CP199 does not reconcile later changes in Zone/control
counts after that one-time commit.

The default-true one-time block first allocates or dimensions:

- one `ZoneTstatSetpt` per Zone;
- `LoadCorrectionFactor` at zero, ordinary control type at Uncontrolled, and
  ordinary control report at zero;
- comfort control type/report and Fanger records only when at least one comfort
  Zone is already declared;
- `Setback`, `DeadBandOrSetback`, and `CurDeadBandOrSetback` at false;
- four zero ZoneList and four zero ZoneGroup sensible heat/cool energy/rate
  arrays;
- six zero previous measured temperature/humidity arrays for hybrid modeling;
- Zone sensible and moisture demand objects; and
- Space sensible and moisture demand objects when either Space heat-balance
  simulation or sizing is active.

For each Zone, the routine walks stored Space membership and each Space's
inclusive heat-transfer-surface range. The first reached surface chooses a
reference-air type; every later mismatch emits the source warning, including
repeated mismatches, but does not set `ErrorsFound`. A Zone with no reached
surface emits nothing.

Output setup then preserves several distinct boundaries:

- CP201 `ZoneSpaceHeatBalanceData::setUpOutputVars` registers four
  heat-balance variables for every Zone and, only during Space simulation,
  every stored Space.
- Each Zone sensible-demand child registers ten variables plus one when staged;
  each moisture child registers six plus six more under `DoLatentSizing`.
  Space simulation registers the same bundles for Spaces.
- Sensible heating/cooling energy meters attach to Zones when Space simulation
  is off and to Spaces when it is on.
- CP199 directly registers six Zone values: thermostat air temperature,
  control type, heating setpoint, cooling setpoint, adaptive-comfort setpoint,
  and load correction factor.
- It adds three variables per comfort-control entry and four each per ZoneList
  and ZoneGroup.

The staged flag is read from `StageZoneLogic` only when that array is
allocated. Zone and Space names, multipliers, membership, demand and
heat-balance arenas, surface ranges/reference types, and OutputProcessor state
are trusted without local shape or consistency validation. Only after every
allocation, warning scan, child setup, and direct registration returns does
line 2618 clear the one-time latch.

When `MyEnvrnFlag && BeginEnvrnFlag` is true, CP199 calls CP200
`ZoneSpaceHeatBalanceData::beginEnvironmentInit` for every Zone heat-balance
record and, under current `doSpaceHeatBalance`, every Space record. It then:

- zeros `setpt`, `setptAdapComfortCool`, `setptLo`, and `setptHi`;
- sets every load correction factor to one and ordinary control type to
  Uncontrolled;
- invokes begin-environment helpers for all Zone demand records and current
  active-Space demand records;
- clears `DeadBandOrSetback` and every Zone's `NoHeatToReturnAir`; and
- zeros all six hybrid measured-history arrays.

The source does not directly reset `setptLoAver` or `setptHiAver`,
`TempControlTypeRpt`, comfort type/report/Fanger state, `Setback`,
`CurDeadBandOrSetback`, or ZoneList/ZoneGroup load totals in this phase. CP200
separately seeds its humidity histories from current outdoor humidity and
zeros selected temperature/load fields; CP200 maps that child separately
below rather than treating it as a CP199 implementation claim. `MyEnvrnFlag` becomes
false only after the complete reset. It is rearmed only on a call observing
`BeginEnvrnFlag == false`.

The begin-day phase has no data initialization. A true `MyDayFlag` with
`BeginDayFlag` true only clears the latch, and any call with the global flag
false rearms it.

After lifecycle work, CP199 loops all ordinary temperature-control entries and
then all comfort-control entries. If Zone-equipment input is filled and
`ControlledZonesChecked` is false, it calls
`VerifyControlledZoneForThermostat`, which performs a Zone-name lookup in
`ZoneEquipConfig`. A missing match emits one Severe and one Continue diagnostic
per reached entry and sets the persistent `ErrorsFound` true. Verification is
deferred while equipment input is not ready.

Demand limiting is independent of that verification condition and uses each
record's unchecked `ActualZoneNum`. Each family switch is reached only when
that control record's `ManageDemand` is true:

| Control family and selector | Trigger | Reached writes |
|---|---|---|
| ordinary SingleHeat | `setpt > HeatingResetLimit` | `setptLo = setpt = HeatingResetLimit` |
| ordinary SingleCool | `setpt < CoolingResetLimit` | `setptHi = setpt = CoolingResetLimit` |
| ordinary SingleHeatCool | `setpt > HeatingResetLimit || setpt < CoolingResetLimit` | ordinary type/report become Dual, low/high seed from `setpt`, then strict low/high clamps |
| ordinary DualHeatCool | independent `setptLo > HeatingResetLimit` and `setptHi < CoolingResetLimit` | only reached low/high clamps; type/report are retained |
| comfort SingleHeat | `setpt >= HeatingResetLimit` | low/current setpoint clamp plus ordinary type/report SingleHeat |
| comfort SingleCool | `setpt <= CoolingResetLimit` | high/current setpoint clamp plus ordinary type/report SingleCool |
| comfort SingleHeatCool | `setpt >= HeatingResetLimit || setpt <= CoolingResetLimit` | ordinary type/report become Dual, low/high seed from `setpt`, then inclusive low/high clamps |
| comfort DualHeatCool | always enters selector body; clamps use inclusive low/high tests | ordinary type/report always become Dual plus any reached clamps |
| either default selector | none | no write |

The routine performs no local finite-value, reset-limit-order, enum, index, or
array-shape validation. NaN comparisons are false. Because the comfort loop
runs second, a Zone represented in both families can finish with comfort-loop
ordinary type/report and setpoint writes.

After both loops, any retained `ErrorsFound` causes the fatal at lines
2809-2811. This member defaults false, is written only by CP199, and is never
cleared at entry, environment boundaries, or successful paths. Demand clamps
and all earlier one-time/environment effects therefore precede the fatal.
Only normal passage beyond it can set `ControlledZonesChecked = true` when
Zone-equipment input is filled. With a missing configuration, the checked flag
stays false; a caught same-state retry can repeat diagnostics and clamps before
the sticky fatal.

Failure location controls retry behavior. A non-return before line 2618 can
leave allocation or output-registration prefixes while the one-time latch
remains true, so retry attempts that phase again. A later failure skips a
successfully committed one-time phase. A begin-environment non-return before
line 2665 retains its latch true and can repeat a partial reset. There is no
rollback. Predictor/corrector placement-new restores its five local flags to
true/true/true/false/false, but HeatBalFanSys, ZoneEnergyDemand, HeatBalance,
ZoneControls, ZoneEquipment, Surface, environment, and OutputProcessor owners
hold the arrays, inputs, histories, and registrations, so clean replay requires
coordinated reset.

Exactly four C++ unit calls invoke CP199 directly:
`HVACUnitaryBypassVAV.unit.cc` lines 659 and 1668 and
`SystemReports.unit.cc` lines 204 and 365. All four use it only as fixture
setup with begin-environment, comfort/demand, and controlled-Zone validation
inactive; none asserts a CP199 field, output registration, or latch. Exactly
56 active `ManageSimulation` unit tests reach CP199 transitively and assert only
downstream or no-throw outcomes; `EMSManager.unit.cc` line 1123 terminates
in the preceding EMS call and does not reach CP199. No focused test covers
reference-air warnings, staged or
Space outputs, environment omissions, begin-day behavior, ordinary/comfort
selector boundaries, sticky fatal, external-HVAC ordering, partial effects,
retry, or reset.

Rust defines `init_zone_air_set_points_compat` only as an identity closure.
Its sole live use manually nests Get -> Init -> empty Calc inside the Predict
shell; its Correct shell omits Init, unlike CP195's selector-independent source
call. Rust separately exposes the exact heating and cooling thermostat output
names by repeating the first DualSetpoint's constant schedules into CLI
`ObservedSeries`. Its `ZoneSysEnergyDemand` is a limited Copy snapshot holding
a Zone ID and four demand values and is reconstructed by bounded IdealLoads
paths. Neither adjacency provides CP199's allocations, sequenced demand state,
environment reset, output registration/time-step/store/meter contract,
verification, demand-limit mutations, flags, failure semantics, or callers.

CP199 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.init_zone_air_set_points`
immediately after `routine.calculate_adaptive_comfort_set_point_schl`. The
heat-balance project contract adds `init_zone_air_set_points` after
`get_zone_air_set_points` and before `update_final_surface_heat_balance`
because required CP195 calls it for every selector. The algorithm remains a
`scaffold` with `claim_level = none`. This checkpoint adds no EnergyPlus source
inventory, Rust target, code, mapped state, support, capability, output
implementation, comparator, manifest, numerical, performance, or conformance
promotion. The inventory becomes 32 algorithms and 207 routines, split 58
`state_mapped` plus 149 `source_mapped`, with 84 required; the heat-balance
project list becomes 53.

### CP200 `ZoneSpaceHeatBalanceData::beginEnvironmentInit` source map

`ZoneSpaceHeatBalanceData::beginEnvironmentInit(EnergyPlusData &state)` is
declared at `ZoneTempPredictorCorrector.hh` line 213 and implemented at
`ZoneTempPredictorCorrector.cc` lines 2818-2836. It is a shared member of the
Zone and Space heat-balance record base and has no record identity, count,
mode test, or latch of its own.

The only two direct call expressions are inside CP199's
`MyEnvrnFlag && BeginEnvrnFlag` block:

1. lines 2623-2625 visit every stored `zoneHeatBalance` record;
2. lines 2626-2630 then visit every stored `spaceHeatBalance` record only when
   current `doSpaceHeatBalance` is true.

The parent therefore completes all Zone calls before any Space call. The
range loops follow stored vector elements rather than logical Zone or Space
counts. CP195 can reach the parent from every update selector, while CP192 can
reach it through external-HVAC initialization, but CP199's environment latch
limits normal CP200 execution.

CP200 performs this exact write sequence:

| Sequence | Targets | Assigned value |
|---|---|---|
| for each `i = 0..3` | `ZTM[i]` | `0.0` |
| same iteration | `WPrevZoneTS[i]` | current `OutHumRat` |
| same iteration | `DSWPrevZoneTS[i]` | current `OutHumRat` |
| same iteration | `WPrevZoneTSTemp[i]` | `0.0` |
| after the loop | `WTimeMinusP`, `W1`, `WMX`, `WM2`, in that order | current `OutHumRat` |
| final scalar tail | `airHumRatTemp`, `tempIndLoad`, `tempDepLoad`, `airRelHum`, `AirPowerCap`, `T1`, in that order | `0.0` |

The total is 26 overwrites: 12 outdoor-humidity values and 14 zeros. No target
value is read before replacement. In particular CP200 does not write
`airHumRat`, `airHumRatAvg`, `airHumRatAvgComf`, MAT/MRT/ZT/XMAT/DSXMAT/TMX/TM2,
the main temperature/load coefficients other than the listed load scalars, or
any other unlisted `ZoneSpaceHeatBalanceData` field.

`ZTM`, `WPrevZoneTS`, `DSWPrevZoneTS`, and `WPrevZoneTSTemp` are fixed
`std::array<Real64, 4>` fields, so literal indices 0 through 3 have no dynamic
shape dependency. `OutHumRat` is copied without arithmetic, finite/sign/range
validation, clamp, or diagnostic. Negative, infinite, and quiet-NaN input
therefore reaches every one of the 12 humidity targets. The source contains 12
RHS uses, but a non-volatile scalar does not establish an observable physical
load count; concurrent mutation would be a data race rather than a supported
mixed snapshot.

The helper is not marked `noexcept`, but valid-state execution contains no
allocation, checked index, formatting, diagnostic, virtual child, or other
ordinary catchable failure source. It returns no status and owns no catch,
cleanup, or rollback. Null or dangling owner access is undefined behavior, not
a modeled recoverable error path. A complete repeat with an unchanged stable
`OutHumRat` is overwrite-idempotent; if outdoor humidity changes, the 12
humidity targets become the last-call value.

Only CP199 supplies lifecycle control. Its environment latch becomes false
after every parent reset completes and is rearmed only by a later call that
observes `BeginEnvrnFlag == false`. If Space heat balance is false at the first
environment entry, CP200 skips all Space records and the parent still clears
the latch; enabling Space later in the same uninterrupted true interval does
not replay them. Predictor/corrector owner reconstruction discards the record
vectors, and newly allocated records regain declaration defaults until CP200
runs again.

There are zero direct CP200 unit calls and zero direct assertions of its
targets. The four direct CP199 fixture calls all keep `BeginEnvrnFlag` false
and never enter CP200. Of 57 active `ManageSimulation` unit calls, the
`EMSManager.unit.cc` line-1123 case exits in the preceding EMS callback. The
other 56 reach CP199, but `WeatherManager_SetRainFlag` has zero Zones, so only
55 execute a Zone CP200 call. Seven `SizingManager.unit.cc` cases also exercise
sizing-Space state.
`HeatBalanceAirManager_GetMixingAndCrossMixing` enables simulation-Space heat
balance and calls `ManageSimulation` at line 864, so it exercises that Space
path as well. Their assertions remain downstream or no-throw evidence rather
than this reset contract.

Rust has no CP200 function or environment-gated Zone/Space reset. Heat-balance
construction creates Zone-only state with configurable three-element
temperature histories and three-element humidity histories seeded from a
fixed default. When a weather series exists,
`seed_zone_air_humidity_ratios_from_weather_series` then overwrites current and
averaged Zone humidity plus both three-slot histories once from the first
weather sample. That seed touches fields CP200 retains and is not a
per-environment member call.

Rust has no exact four-slot `ZTM`, `WPrevZoneTS`, `DSWPrevZoneTS`, and
`WPrevZoneTSTemp` set, Space heat-balance records, 26-field
touched-versus-retained boundary, or outer environment gate. Its coefficient
representation begins from a zero value but is computed again before
initialization returns, rather than reproducing CP200's field-level reset.
Later bounded predictor/corrector history and coefficient work does not
establish CP200 parity.

CP200 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_space_heat_balance_begin_environment_init`
immediately after `routine.init_zone_air_set_points`. The heat-balance project
contract adds `zone_space_heat_balance_begin_environment_init` after
`init_zone_air_set_points`. CP201 now follows that entry before
`update_final_surface_heat_balance`. The
algorithm remains a `scaffold` with `claim_level = none`. No EnergyPlus source
inventory, Rust target, code, mapped state, support, capability, output
implementation, comparator, manifest, numerical, performance, or conformance
promotion is added. The inventory becomes 32 algorithms and 208 routines,
split 58 `state_mapped` plus 150 `source_mapped`, with 85 required; the
heat-balance project list becomes 54.

### CP201 `ZoneSpaceHeatBalanceData::setUpOutputVars` source map

`ZoneSpaceHeatBalanceData::setUpOutputVars(EnergyPlusData &state,
std::string_view prefix, std::string const &name)` is declared at
`ZoneTempPredictorCorrector.hh` line 215 and implemented at
`ZoneTempPredictorCorrector.cc` lines 2838-2868. It is a shared Zone/Space
record member, but it receives only a prefix and key and owns no record
identity, count, mode, membership, or lifecycle guard.

Its only production call expressions are inside CP199's one-time Zone loop:

1. line 2443 calls every `zoneHeatBalance(zoneNum)` for
   `zoneNum = 1..NumOfZones` with prefix `Zone` and that Zone's stored name;
2. lines 2444-2448 immediately walk the current Zone's `spaceIndexes` and call
   `spaceHeatBalance(spaceNum)` with prefix `Space` and that Space's stored name
   only when `doSpaceHeatBalanceSimulation` is true.

The order is therefore Zone 1, its stored Spaces, the remaining Zone 1 output
bundles, Zone 2, and so on. It is not CP200's all-Zones-then-all-Spaces order.
Duplicate Space membership, invalid indices, repeated or blank names, and
prefix/key consistency are trusted. Sizing-only `doSpaceHeatBalance` does not
open the Space path. If `N` Zones are visited and `S` nested Space memberships
are reached, CP201 runs for `R = N + S` records and makes exactly `4R` child
setup calls.

Each record uses this fixed registration sequence:

| Sequence | Formatted name | Member address | Units | timestep | store |
|---|---|---|---|---|---|
| 1 | `{prefix} Air Temperature` | `ZT` | C | System | Average |
| 2 | `{prefix} Air Humidity Ratio` | `airHumRat` | None | System | Average |
| 3 | `{prefix} Air Relative Humidity` | `airRelHum` | percent | System | Average |
| 4 | `{prefix} Mean Radiant Temperature` | `MRT` | C | Zone | Average |

The supplied stored name is the key for all four rows. Production prefixes are
the constants `zonePrefix = "Zone"` and `spacePrefix = "Space"`. CP201 leaves
the `SetupOutputVariable` optional arguments at their defaults: resource,
group, and end use are Invalid; end-use subcategory, meter Zone, space type,
and custom unit are empty; both multipliers are one; `indexGroupKey` is -999;
and report frequency is Hour. A matching `Output:Variable` request can replace
that Hour value with its requested frequency and schedule on the concrete
output entry. These calls never attach meters and do not increment Sum or
meter counters.

CP201 does not read, validate, normalize, or assign `ZT`, `airHumRat`,
`airRelHum`, or `MRT`. It passes a mutable address for each member to
OutputProcessor. On first use, `SetupOutputVariable` can initialize global
output state and parse requested-variable input. It then checks matching
requests and the DataOutputs variable list, calls `AddDDOutVar`, increments
`NumOfRVariable_Setup`, and increments `NumTotalRVariable` by at least one for
every CP201 row.

`AddDDOutVar` keys its dictionary map by uppercase variable name and reuses an
entry only when units also match. It does not compare the supplied key,
timestep, store, or variable type. Zone records consequently share four
dictionary names, and reached Space records share four distinct prefixed names.
With neither a report request nor a variable-list match, setup returns after
the dictionary and counter effects without creating an `OutVarReal`. A
variable-list-only match appends one dummy keyed entry, dictionary link, report
identifier, and `Which = &member` pointer, but leaves `Report` false and skips
report-dictionary sinks. Each distinct frequency/schedule report request
appends a reporting entry with the same link, identifier, pointer, and metadata
and writes the applicable ESO, SQL, and ResultsFramework dictionary records.
Duplicate matching requests within one call are collapsed only by
frequency/schedule identity; prior Used state does not suppress a later helper
call.

There is no local idempotence, status, catch, cleanup, or rollback. The four
format operations and four child registrations are sequential. Formatting,
OutputProcessor initialization, requested-variable parsing, allocation, or a
dictionary sink can fail after any completed prefix of dictionary, counter,
link, pointer, identifier, or external-output side effects. CP199 clears
`InitZoneAirSetPointsOneTimeFlag` only at line 2618, after all Zone, Space,
demand, thermostat, comfort, ZoneList, and ZoneGroup setup returns. A non-return
therefore leaves the parent latch true and can cause a same-state retry to
attempt the one-time phase again.

A direct complete replay reuses same-name/unit dictionary entries but advances
setup and total counters again. With neither a request nor a DataOutputs-list
match, it repeats only those counter effects. A list match also duplicates a
dummy or reporting keyed `OutVarReal`, dictionary link, and report identifier;
a report request additionally duplicates the report flag and emitted
dictionary rows. The raw `Which` pointers remain valid only while the
registered Zone/Space storage stays stable. Predictor/corrector `clear_state`
placement-news the record owner and rearms its latch without clearing
OutputProcessor, so using it alone can leave dangling pointers.
OutputProcessor `clear_state` deletes output and DD arenas and resets most
registry state but does not rebuild the records, cannot retract emitted rows,
and does not explicitly reset `NumOfRVariable` or `NumOfIVariable`. A clean
replay needs coordinated fresh ownership rather than either reset alone.

Search finds zero direct CP201 unit calls and zero positive assertions of its
registered name, unit, key, timestep, store, or member binding. The four direct CP199
fixtures at `HVACUnitaryBypassVAV.unit.cc` lines 659 and 1668 and
`SystemReports.unit.cc` lines 204 and 365 indirectly run Zone CP201 with
simulation-Space false, but assert unrelated outputs. Of 57 active
`ManageSimulation` unit calls, `EMSManager.unit.cc` line 1123 terminates before
CP199. The other 56 reach CP199, but `WeatherManager_SetRainFlag` has zero
Zones, so only 55 execute a Zone CP201 registration. Only
`HeatBalanceAirManager_GetMixingAndCrossMixing` enables simulation-Space,
visits three stored Spaces, and asserts later geometry/mixing behavior. The
seven `SizingManager.unit.cc` cases that reach CP200's sizing-Space reset keep
simulation-Space off and do not reach CP201 Space setup.

Two exact-name test literals are not positive coverage. The
`ThermalChimney.unit.cc` `Zone Air Temperature` string is an EMS input in a
test that does not call CP199. The API data-transfer test asks for a
`Zone Mean Radiant Temperature` handle in synthetic state without CP201 setup
and expects -1.

Rust's `RuntimeOutputRegistry::register_model_outputs` registers
`Zone Mean Air Temperature` with C/Hourly metadata for each typed Zone, not
`Zone Air Temperature`. The heat-balance ResultStore separately emits
`Zone Mean Air Temperature` and `Zone Mean Air Humidity Ratio` from trace
series and infers Average storage, but represents neither CP201's exact names
and members nor its System-versus-Zone timestep distinction. Rust has no
`Zone Air Relative Humidity`, `Zone Mean Radiant Temperature`, any of the four
Space names, distinct `ZT`/`airHumRat`/`airRelHum`/`MRT` output bindings,
Space heat-balance record, or OutputProcessor pointer/latch lifecycle.

The compiler test literal `Zone Air Temperature` only demonstrates typed
`Output:Variable` request intake. The IdealLoads CLI constants `Zone Air
Temperature` and `Zone Air Humidity Ratio` are ESO oracle input identities;
the humidity name also labels a closed-loop diagnostic comparison. Neither is
a RuntimeOutputRegistry or ResultStore production definition. Adjacent Zone
mean-air state, registry, ResultStore, and generic Average-store inference
therefore do not establish CP201 parity.

CP201 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_space_heat_balance_set_up_output_vars`
immediately after
`routine.zone_space_heat_balance_begin_environment_init`. The heat-balance
project contract adds `zone_space_heat_balance_set_up_output_vars` after
`zone_space_heat_balance_begin_environment_init`. CP202 now follows that entry
before `update_final_surface_heat_balance`. The algorithm remains a `scaffold`
with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 209 routines, split 58 `state_mapped` plus 151
`source_mapped`, with 86 required; the heat-balance project list becomes 55.

### CP202 `PredictSystemLoads` source map

`PredictSystemLoads(EnergyPlusData &state, bool ShortenTimeStepSys,
bool UseZoneTimeStepHistory, Real64 PriorTimeStep)` is declared at
`ZoneTempPredictorCorrector.hh` lines 276-280 and implemented at
`ZoneTempPredictorCorrector.cc` lines 2870-3145. Its only production call
expression is CP195 `ManageZoneAirUpdates` line 227 under the `PredictStep`
selector. It is reached only after the wrapper's optional CP196 input and
unconditional CP199 initialization return, receives all three timestep
arguments unchanged, and never receives or alters caller-owned
`ZoneTempChange`.

Three production PredictStep entrances share that expression:

| Entrance | Immediate source order and arguments |
|---|---|
| `HVACManager` lines 262-267 | after setpoint acquisition, airflow/AFN, lagged dependent loads, indoor-green, and internal-gain updates; before `SimHVAC`; the initial path has `ShortenTimeStepSys = false`, `UseZoneTimeStepHistory = true`, and `PriorTimeStep = TimeStepZone` |
| `HVACManager` lines 346-351 | only inside `TimeStepSys < TimeStepZone` substeps; adaptive downstepping makes the first call shortened with Zone history disabled, then line 372 clears shortening so later substeps can call with false/false while retaining the prior Zone timestep |
| `SimulationManager::Resimulate` lines 2929-2930 | under `ResimHVAC`, after Get setpoints and simple airflow and before `SimHVAC`; passes false shortening, the current history selector, and zero prior timestep |

CP202 then executes four ordered phases.

#### Staged thermostat phase

When `NumStageCtrZone > 0`, lines 2915-2992 walk exactly that many
`StageControlledZone` records in stored order. Each record trusts
`ActualZoneNum`, both schedule pointers, the stage counts and offset arrays,
Zone/Space membership, and all destination arrays. It samples both base
schedules on every call and overwrites `HeatSetPoint` and `CoolSetPoint`.
`ZoneT` is the Zone record's `MAT`, replaced by `XMPT` only when shortening.

If sampled heating is greater than or equal to cooling, CP202 increments
`StageErrCount`. The first occurrence emits a warning, continuation, and
timestamp; every later occurrence updates a recurring warning using
`StageErrIndex` and the sampled heating value as both tracked extrema. It then
sets heating to cooling minus 0.1 C and performs no second validity check.

| Mode selection | `StageNum` and thermostat setpoint writes |
|---|---|
| strict `CoolSetPoint < ZoneT` | offset is `ZoneT - CoolSetPoint`; the last index whose offset is greater than or equal to `CoolTOffset(I)` wins as negative `-I`; both high and low become cooling minus half `CoolThroRange` when offset reaches that half range, otherwise cooling plus that half range |
| otherwise strict `HeatSetPoint > ZoneT` | offset is `ZoneT - HeatSetPoint`; the last index whose absolute offset reaches the absolute `HeatTOffset(I)` wins as positive `I`; both low and high become heating plus or minus half `HeatThroRange` |
| otherwise | high becomes cooling plus half `CoolThroRange`, low becomes heating minus half `HeatThroRange`, and `StageNum = 0` |

The heating half-range decision at line 2973 intentionally compares the
absolute offset with half `CoolThroRange`, not `HeatThroRange`, although the
assigned displacement uses `HeatThroRange`. This source-literal asymmetry is
part of the map. After each staged Zone, `doSpaceHeatBalance` copies its final
`StageNum` to every stored Space demand membership; simulation and sizing modes
both qualify.

#### On/off cutout prefix

`NumOnOffCtrZone > 0` is only an outer gate. Lines 2999-3111 still scan all
`NumTempControlledZones` and process every record whose `DeltaTCutSet > 0`.
A shortened call restores `HeatModeLast` and `CoolModeLast` from their saved
values; a nonshortened call first saves the current values. It then clears both
off flags. ThirdOrder selects `MAT`, or `XMPT` when shortened, as `Tprev`;
Analytical, Euler, and every other algorithm use `T1` regardless of shortening.

The fixed tolerance is 0.02 C, and the implementation preserves its literal
strict comparisons:

| Control type | Revision and mode-memory override |
|---|---|
| `SingleHeat` | initialize generic and low setpoints to base low; write base low plus delta when `Tprev < low + 0.02` or when `Tprev > low && Tprev < low + delta - 0.02`, otherwise set `HeatOffFlag`; prior heat mode with `Tprev > low` overrides back to base and sets the off flag |
| `SingleCool` | initialize generic and high setpoints to base high; write base high minus delta when `Tprev > high - 0.02` or when `Tprev < high && Tprev > high - delta + 0.02`, otherwise set `CoolOffFlag`; prior cool mode with `Tprev < high` overrides back to base and sets the off flag |
| `DualHeatCool` | initialize high and low separately, run the same cooling logic first and heating logic second, and do not write the generic `setpt` member |
| `SingleHeatCool`, Uncontrolled, and every other value | no switch-owned setpoint write; the preceding save/restore and off-flag clears still persist |

After both DualHeatCool halves, an inclusive `setptLo >= setptHi` test emits a
severe diagnostic, a timestamp naming the Zone, two formatted setpoint
continuations, and a fatal. The diagnostic occurs after all already reached
staged and on/off mutations and before any Zone load child.

#### Zone and Space child dispatch

Lines 3114-3126 always visit `zoneNum = 1..NumOfZones`. Each Zone first calls
the next source definition, CP203
`ZoneSpaceHeatBalanceData::predictSystemLoad`, with the three original
timestep arguments and `zoneNum`. Its temperature/history update, coefficient,
sensible demand, and moisture demand calculations remain CP203 dependency
behavior rather than CP202 implementation credit.

The current Zone's `spaceIndexes` are then visited in stored order:

| Space condition | CP202 action |
|---|---|
| `doSpaceHeatBalance` | call CP203 on `spaceHeatBalance(spaceNum)` with the same arguments plus `zoneNum` and `spaceNum` |
| inactive Space heat balance and shortened call | copy only the already processed Zone `MAT` and `airHumRat` to that Space record |
| inactive Space heat balance and nonshortened call | leave the Space record unchanged |

Zone and Space work is interleaved per Zone. `UseZoneTimeStepHistory` and
`PriorTimeStep` are not otherwise read by CP202. Counts, indices, duplicate
memberships, array shape, and identity consistency are not locally reconciled.

#### Final on/off memory phase

Only after every reached child returns do lines 3127-3144 rescan positive
cutout controls. Cooling last-mode memory becomes exactly
`CoolOffFlag && TotalOutputRequired >= 0`; heating becomes exactly
`HeatOffFlag && TotalOutputRequired <= 0`. The inclusive tests allow both to
be true at zero when both off flags are true. A NaN load makes both comparisons
false. Controls skipped by the positive-delta predicate retain their prior
mode fields.

CP202 owns no latch, status result, finite/sign/range validation, catch,
cleanup, or rollback. NaN schedule or temperature comparisons follow native
false branches, can select staged deadband or off flags, and can bypass the
dual collision; infinities and unusual ranges propagate through arithmetic.
Null schedules, invalid actual Zone numbers, short stage arrays, malformed
demand arrays, and bad Space memberships reach unchecked dependency or
container behavior.

A dual fatal retains the complete staged phase plus the reached on/off
save/restore, off-flag, and setpoint prefix, and suppresses every child and the
final mode pass. A Zone or Space child non-return retains all earlier
Zone/Space effects and fallback copies and likewise suppresses the remaining
tail. Same-state retry samples schedules again, increments staged error state
again, may replace saved mode state, and repeats child effects. A clean replay
requires coordinated reconstruction of predictor counts and Zone/Space
records, ZoneControls staged and temperature records, HeatBalFanSys setpoints
and control types, ZoneEnergyDemand stages and loads, HeatBalance topology,
HVAC timestep/history state, diagnostic recurrence state, and CP203's
dependencies. Clearing only one owner can leave stale counts, indices, or
state in another.

Search finds exactly 16 direct CP202 calls in two fixtures:

- `SetPointWithCutoutDeltaT_test` makes eight nonshortened Euler calls.
- `TempAtPrevTimeStepWithCutoutDeltaT_test` makes four normal and four
  shortened ThirdOrder calls, using `MAT` versus `XMPT` and saved last modes.
- Together they assert 24 low/high setpoint values across two SingleHeat, two
  SingleCool, one SingleHeatCool, and three DualHeatCool calls per fixture.

Both fixtures pass false `UseZoneTimeStepHistory` and 0.01 prior time and leave
global `NumOfZones` at zero. They therefore execute the on/off prefix and final
mode loop but no Zone/Space CP203 child. They assert no off flags, saved or
final modes, staged thresholds or diagnostics, exact 0.02 boundaries, dual
fatal, argument forwarding, Space branch, partial failure, retry, or reset.

Of 57 active C++ `ManageSimulation` calls, one EMS callback test fatals before
CP202. The other 56 reach CP202; the no-Zone
`WeatherManager_SetRainFlag` case executes no Zone child, while 55 execute at
least one. `HeatBalanceAirManager_GetMixingAndCrossMixing` enables
simulation-Space heat balance for three Spaces, and seven Space-sizing tests
also enable three Space children each. Their assertions concern downstream
mixing or sizing reports. None of the full simulations supplies a staged
thermostat or positive cutout delta, so they do not positively cover either
control phase.

Rust's `predict_system_loads_compat` calls
`predict_step_source_order_path`, and both are identity closures. The enclosing
bounded timestep loop shifts Zone histories, assembles gains and surface terms,
and directly updates MAT; it does not execute CP202 thermostat or load
dispatch. The parsed
`temperature_difference_between_cutout_and_setpoint_delta_c` field has no
runtime consumer. `EvaluateZoneThermostat` is execution-plan metadata, and the
IdealLoads setpoint helper samples only the first DualSetpoint's constant
schedules while ignoring the control-type schedule and cutout delta.

Rust has no staged thermostat record, `StageNum`, source
`zoneTstatSetpts` arena, heat/cool last/save/off flags, sensible
`ZoneSysEnergyDemand::TotalOutputRequired`, Space heat-balance record, Space
demand, or CP203
Zone/Space dispatch. Its `ZoneSysEnergyDemand` carries only oracle-fed remaining
heat/cool and moisture snapshots for IdealLoads. Coefficient and zone-air
algorithm tests exercise adjacent bounded MAT/history math, not this wrapper.
There is no direct `predict_system_loads_compat` test; one pass-through test
asserts only the string `PredictStep` in a wrapper order.

CP202 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.predict_system_loads`
immediately after
`routine.zone_space_heat_balance_set_up_output_vars`. The heat-balance project
contract adds `predict_system_loads` after
`zone_space_heat_balance_set_up_output_vars`. CP203 now follows that entry
before `update_final_surface_heat_balance`. The algorithm remains a `scaffold`
with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 210 routines, split 58 `state_mapped` plus 152
`source_mapped`, with 87 required; the heat-balance project list becomes 56.

### CP203 `ZoneSpaceHeatBalanceData::predictSystemLoad` source map

`ZoneSpaceHeatBalanceData::predictSystemLoad(EnergyPlusData &state,
bool shortenTimeStepSys, bool useZoneTimeStepHistory, Real64 priorTimeStep,
int zoneNum, int spaceNum = 0)` is declared at
`ZoneTempPredictorCorrector.hh` lines 217-222 and implemented at
`ZoneTempPredictorCorrector.cc` lines 3146-3257. The only production call
expressions are its CP202 parent: line 3116 invokes the member on every Zone
record, then lines 3119-3120 invoke it on every stored Space membership only
when `doSpaceHeatBalance` is true. Zone work therefore precedes that Zone's
stored-order Space work, and the parent forwards all three timestep/history
arguments unchanged.

CP203 owns no traversal count or mode gate. Its only local identity check is the
debug `assert(zoneNum > 0)` at line 3154; release builds may compile that check
out. The default `spaceNum = 0` denotes the Zone record. Production supplies
only zero or positive Space identities.

#### Ordered coefficient and hybrid path

The first executable child is
`updateTemperatures(state, shortenTimeStepSys, useZoneTimeStepHistory,
priorTimeStep, zoneNum, spaceNum)`. Its shortened-step rollback,
down-interpolation, node history, and selected `ZTM`/humidity history behavior
remain a later child boundary. CP203 reads `TimeStepSys` and
`TimeStepSysSec` only after that child returns. `useZoneTimeStepHistory` and
`priorTimeStep` have no later direct use in CP203.

Volume selection and the first direct write are literal:

| `spaceNum` | Selected volume | Sensible capacitance multiplier |
|---|---|---|
| positive | `space(spaceNum).Volume` | parent `Zone(zoneNum).ZoneVolCapMultpSens` |
| zero | `Zone(zoneNum).Volume` | that Zone's `ZoneVolCapMultpSens` |

```text
AirPowerCap =
    volume * ZoneVolCapMultpSens
    * PsyRhoAirFnPbTdbW(OutBaroPress, MAT, airHumRat)
    * PsyCpAirFnW(airHumRat)
    / TimeStepSysSec
```

A Space therefore uses its own volume and record `MAT`/`airHumRat` but still
uses its parent Zone's sensible capacitance multiplier. `TimeStepSys` and
`TimeStepSysSec` are independent source inputs: the former later controls a
history branch, while only the latter divides capacity.

CP203 initializes local `RAFNFrac = 0` and calls
`calcZoneOrSpaceSums(state, false, zoneNum, spaceNum)`. The false corrector flag
builds predictor sums; source comments explicitly note that `SumSysMCp` and
`SumSysMCpT` are unused in this prediction step. Only a Zone record
(`spaceNum == 0`) under `FlagHybridModel_PC` enters the hybrid branch. It first
writes `SumIntGainExceptPeople = 0` and then assigns the result of
`SumAllInternalConvectionGainsExceptPeople(state, zoneNum)`; a non-return from
that child can therefore retain the explicit zero.

After those children return, five ordered direct equations establish the base
ThirdOrder-shaped state:

```text
TempDepCoef  = SumHA + SumMCp
TempIndCoef  = SumIntGain + SumHATsurf - SumHATref
             + SumMCpT + SysDepZoneLoadsLagged
TempHistoryTerm = AirPowerCap
                * (3 * ZTM[0] - 1.5 * ZTM[1] + ZTM[2] / 3)
tempDepLoad  = (11 / 6) * AirPowerCap + TempDepCoef
tempIndLoad  = TempHistoryTerm + TempIndCoef
```

Only `ZTM[0]` through `ZTM[2]` participate; the fourth fixed history slot is not
read by these equations. The lagged system-dependent load is included, while
the two system-air sums remain excluded through the predictor-sum child.

#### RoomAir AirflowNetwork override

The base values survive unless all three source gates are true:
`anyNonMixingRoomAirModel`, the parent Zone's `AirModel == AirflowNetwork`,
and `AFNZoneInfo(zoneNum).IsUsed`. CP203 then gets
`ControlAirNodeID`, calls
`LoadPredictionRoomAirModelAFN(state, zoneNum, RoomAirNode)`, and only after
that child returns overwrites:

- `TempDepCoef` from control-node `SumHA + SumLinkMCp`;
- `TempIndCoef` from control-node internal, surface, reference-air, link, and
  lagged load terms;
- `AirPowerCap` from control-node volume, the parent Zone multiplier, node
  `RhoAir` and `CpAir`, divided by `TimeStepSysSec`;
- `TempHistoryTerm` from that capacity and this Zone/Space record's three
  `ZTM` values;
- `tempDepLoad` and `tempIndLoad` from the same ThirdOrder formulas.

When the control node has assigned HVAC, `HVAC(1).SupplyFraction` replaces the
local `RAFNFrac`; otherwise it remains zero. The fraction is not range- or
finite-checked here. This entire override is keyed by `zoneNum` and has no
`spaceNum` gate. An active Space invocation can therefore rerun the Zone AFN
load predictor and replace its Space-specific base sums and capacity with the
same control-node coefficients and node volume, while retaining that Space
record's `ZTM` in the history term.

#### Solution-algorithm history path

Line 3212 unconditionally writes shared
`HVACGlobal.ShortenTimeStepSysRoomAir = false` before inspecting the solution
algorithm. The shared value is reset on every Zone and active Space invocation.

| Solution and timestep condition | Direct record and AFN-node writes |
|---|---|
| `ThirdOrder` | leave `T1`, `W1`, and AFN-node T1 histories untouched; retain the capacity/history-bearing load scalars and the shared false flag |
| non-ThirdOrder, `shortenTimeStepSys && TimeStepSys < TimeStepZone`, and shared `PreviousTimeStep < TimeStepZone` | copy `TM2`/`WM2` to `T1`/`W1`; copy every AFN node's T2 temperature/humidity to T1; set the shared flag true |
| same shortened condition, but the shared previous-timestep comparison is false | copy `TMX`/`WMX` and every AFN node's TX histories; set the shared flag true |
| every other non-ThirdOrder path | copy current `ZT`/`airHumRat` and every AFN node's current temperature/humidity to T1; leave the shared flag false |

The shortened choice reads global `PreviousTimeStep`, not the
`priorTimeStep` argument already forwarded to `updateTemperatures`. NaN
comparisons naturally select false branches. These later AFN-node loops test
only `AirModel(zoneNum).AirModel == AirflowNetwork`; they do not repeat the
earlier `anyNonMixingRoomAirModel` or `AFNZoneInfo.IsUsed` gates. After any
non-ThirdOrder branch, CP203 overwrites `tempDepLoad = TempDepCoef` and
`tempIndLoad = TempIndCoef`, deliberately removing the capacity and history
terms from the load scalars while leaving `AirPowerCap` and
`TempHistoryTerm` stored.

Because the shared shortening flag is cleared and possibly reset on every
record call, an abnormal exit can expose the value from the last reached
record. A completely successful CP202 traversal leaves the value produced by
its last Zone or active Space CP203 invocation.

#### Ordered demand children and ownership

The final calls are strictly ordered:

1. `calcPredictedSystemLoad(state, RAFNFrac, zoneNum, spaceNum)`;
2. `calcPredictedHumidityRatio(state, RAFNFrac, zoneNum, spaceNum)`.

CP203 owns this ordering and the arguments but does not own either child's
control-type/setpoint equations, sensible `ZoneSysEnergyDemand` or node writes,
`setPointLast`/setback state, humidity-control equations, moisture demand,
warnings, fatals, or report fields. The same dependency boundary applies to
`updateTemperatures`, `calcZoneOrSpaceSums`, both psychrometric helpers, hybrid
gain collection, and RoomAir AFN load prediction.

Its direct record writes are `AirPowerCap`, optional
`SumIntGainExceptPeople`, `TempDepCoef`, `TempIndCoef`,
`TempHistoryTerm`, `tempDepLoad`, `tempIndLoad`, and, only outside
ThirdOrder, `T1` and `W1`. Direct external writes are the shared
`ShortenTimeStepSysRoomAir` flag and the non-ThirdOrder AFN-node
`AirTempT1`/`HumRatT1` histories. All other mutations belong to called
dependencies.

#### Validation, failure, retry, and reset

Apart from the debug assertion, CP203 does not validate upper bounds, Space
membership, record-array shape, volume, multiplier, pressure, temperature,
humidity, timestep, histories, AFN control-node/HVAC topology, supply fraction,
or finite values. A zero, negative, infinite, or NaN `TimeStepSysSec` and
unusual volume, capacitance, or psychrometric inputs flow through native
floating-point arithmetic. Invalid `spaceNum < 0` is internally inconsistent:
the first child treats nonzero as Space and may index Space state before the
capacity branch later selects Zone volume and the hybrid branch skips Zone-only
work. Production never supplies that value.

There is no local diagnostic, latch, status result, catch, cleanup, or
rollback. Failure boundaries preserve ordered prefixes:

- a non-return from `updateTemperatures` suppresses every local write but keeps
  that child's reached history effects;
- capacity precedes predictor sums, whose effects precede the hybrid clear and
  refresh;
- base coefficients precede the optional AFN child and its overwrite prefix;
- the shared flag, record histories, and AFN-node histories precede both demand
  children;
- sensible demand commits before a moisture-child warning or fatal can stop
  the call.

Same-state retry reruns every dependency and direct write. It is not a general
idempotent transaction: among later child effects,
`calcPredictedSystemLoad` compares with and then overwrites per-record
`setPointLast`, so retry can change setback state even with otherwise unchanged
inputs, while warning/recurrence state can advance. CP200
`beginEnvironmentInit` resets only a subset and is not a complete retry reset.
Clean replay requires reconstruction or coordinated clearing of predictor
Zone/Space records, HVAC timestep and shared flag state, HeatBalFanSys and loop
nodes, HeatBalance topology and volumes, RoomAir/AFN node state,
HybridModel/internal gains, surface/airflow sums, environment and
psychrometric diagnostics, Zone/Space sensible and moisture demand, and every
child-owned warning and history owner.

#### C++ and Rust evidence boundary

No C++ test calls `predictSystemLoad` directly. CP202's two focused fixtures
make 16 wrapper calls and 24 setpoint assertions, but both retain
`NumOfZones = 0` and therefore enter CP203 zero times. Dependency-only tests
call `calcPredictedSystemLoad` seven times with 19 post-call assertions,
`calcZoneOrSpaceSums` five times with 12 assertions, and the two
`DownInterpolate4HistoryValues` overloads once each with 14 assertions. They do
not compose CP203 or directly cover `updateTemperatures`,
`calcPredictedHumidityRatio`, `LoadPredictionRoomAirModelAFN`, the coefficient
write sequence, shared shortening flag, Space path, failure prefix, retry, or
reset.

Of 57 active C++ `ManageSimulation` call sites, one expected EMS fatal stops
before CP202. The other 56 reach CP202. `WeatherManager_SetRainFlag` has zero
Zones, leaving 55 configurations that execute at least one Zone CP203.
`HeatBalanceAirManager_GetMixingAndCrossMixing` enables simulation-Space heat
balance, and seven sizing fixtures enable sizing-Space heat balance, for eight
Space-enabled configurations. Their assertions concern downstream mixing or
sizing. Six configurations explicitly select AnalyticalSolution, none selects
Euler, and no active full-simulation block configures or manually assigns the
RoomAir AirflowNetwork model. This is transitive execution evidence, not a
focused CP203 oracle.

Rust's `predict_system_loads_compat` and
`predict_step_source_order_path` are identity closures. The enclosing
heat-balance timestep loop forces its main Predict record shortening field
false, shifts three-slot Zone histories, assembles a bounded gain/surface
subset, and directly updates `MAT`. It has no semantic CP203 wrapper test.

`energyplus_zone_air_temperature_coefficients` and one direct test reproduce
six adjacent algebraic quantities, but the helper omits
`SysDepZoneLoadsLagged`, guards nonpositive capacity/timestep to zero, and its
runtime callers add `sum_sys_mcp` and `sum_sys_mcp_t` even though CP203's
predictor sums exclude those system-air terms. Rust's moist-air capacity helper
covers unit-multiplier `volume * rho * Cp`, but has no Zone capacitance
multiplier or Space state and is refreshed in a different CorrectStep order.
Its down-interpolation helper owns three histories rather than CP203's full
temperature, humidity, node, and RoomAir history transaction.

A separate IdealLoads
`calc_no_oa_third_order_moisture_demand_compat` implements a bounded,
fixed-timestep, no-outdoor-air ThirdOrder humidity subset with validation and
transactional `Option` failure. It is not called from the heat-balance CP203
path and does not supply CP203's ordered sensible child, schedule/EMS/fault,
airflow, Analytical/Euler, RAFN, Space, warning, or partial-effect behavior.
Rust has no CP203-equivalent heat-balance sensible-load producer, Space heat-balance/demand record,
RoomAir/AFN or hybrid state, `RAFNFrac`, shared
`ShortenTimeStepSysRoomAir`, or exact `T1`/`W1` and AFN-node history path.

CP203 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_space_heat_balance_predict_system_load`
immediately after `routine.predict_system_loads`. The heat-balance project
contract adds `zone_space_heat_balance_predict_system_load` after
`predict_system_loads` and before `calc_zone_air_temp_set_points`. The
algorithm remains a `scaffold` with `claim_level = none`. No EnergyPlus source
inventory, Rust target, code, mapped state, test, support, capability, output
implementation, comparator, manifest, numerical, performance, or conformance
promotion is added. The inventory becomes 32 algorithms and 211 routines,
split 58 `state_mapped` plus 153 `source_mapped`, with 88 required; the
heat-balance project list becomes 57.

### CP204 `CalcZoneAirTempSetPoints` source map

`CalcZoneAirTempSetPoints(EnergyPlusData &state)` is declared at
`ZoneTempPredictorCorrector.hh` line 282 and implemented at
`ZoneTempPredictorCorrector.cc` lines 3259-3460. Its only production call
expression is CP195 `ManageZoneAirUpdates` line 224, selected only by
`PredictorCorrectorCtrl::GetZoneSetPoints`. `HVACManager::ManageHVAC` enters
that wrapper at lines 224-229 after its
`BeginTimestepBeforePredictor` EMS call, outdoor-air node update, and
refrigerated-case-rack update, and before contaminant, hybrid-ventilation,
airflow, AirflowNetwork, lagged-load, gain, and CP202 prediction work.
`SimulationManager::Resimulate` supplies the second production entrance at
lines 2917-2922 under `ResimHVAC`. CP196 input and CP199 initialization must
return before either entrance reaches CP204.

The routine takes no timestep, shortening, history, or caller-owned
`ZoneTempChange` argument. It traverses only ordinary
`TempControlledZone` records. There is no Space record path.

#### Entry reset and record traversal

Every entry performs this ordered prefix:

1. Assign the complete allocated `TempControlType` array to
   `HVAC::SetptType::Uncontrolled`.
2. Allocate `OccRoomTSetPointHeat(NumOfZones)` only when it is absent.
3. Allocate `OccRoomTSetPointCool(NumOfZones)` only when it is absent.
4. Fill every existing occupied-heating entry with `0.0` and every existing
   occupied-cooling entry with `100.0`.
5. Assign local `DeltaT = 0.0`; the value is never read.

Already allocated occupied arrays are not resized when `NumOfZones` changes.
The routine does not globally clear `TempControlTypeRpt` or any member of
`zoneTstatSetpts`. If the second allocation does not return, the first array
can remain newly allocated after the type-array reset.

For `RelativeZoneNum = 1..NumTempControlledZones`, CP204 trusts the stored
record and its `ActualZoneNum`. It samples
`tempZone.setptTypeSched->getCurrentVal()`, directly casts the result to
`HVAC::SetptType`, writes that enum to `TempControlType(ActualZoneNum)`, and
writes its integer form to `TempControlTypeRpt(ActualZoneNum)`. It then aliases
`zoneTstatSetpts(ActualZoneNum)` and dispatches on the sampled type. The local
routine does not verify that the schedule value is finite, integral, within
the enum, or consistent with an available setpoint family.

#### Setpoint branch matrix

| Control type | Source-ordered direct and child effects | Intentionally retained fields |
|---|---|---|
| `Uncontrolled` | no thermostat-setpoint write | `setpt`, `setptLo`, `setptHi`, raw low/high, and adaptive snapshot all retain prior values |
| `SingleHeat` | only when `setpts[SingleHeat].isUsed`: sample heat into `setpt`, copy the raw value to `ZoneThermostatSetPointLo`, call `AdjustAirSetPointsforOpTempCntrl` on `setpt`, then copy it to `setptLo` | `setptHi` and adaptive cool remain untouched; a false `isUsed` writes none of the setpoint fields |
| `SingleCool` | sample cool into `setpt` without an `isUsed` guard, copy the raw value to `ZoneThermostatSetPointHi`, optionally adaptive-adjust `setpt` and copy it to `setptAdapComfortCool`, operative-adjust `setpt`, copy it to `setptHi`, then call the humidity-overcool helper | `setptLo` remains untouched; a false adaptive flag leaves its earlier adaptive snapshot |
| `SingleHeatCool` | sample that family's heat schedule into `setpt`, optionally adaptive-adjust and snapshot it, operative-adjust it, then assign both `setptLo` and `setptHi` | raw low/high are not refreshed; optimum-start can subsequently diverge generic and bound values |
| `DualHeatCool` | sample cool into `setptHi` and raw high, optionally adaptive-adjust and snapshot high, operative-adjust high, then sample heat into `setptLo` and raw low and operative-adjust low; optimum-start follows, then humidity overcool runs last | generic `setpt` is never directly written by this switch arm |

Adaptive comfort is applied only to cooling values in `SingleCool` and
`DualHeatCool`, and to the shared value in `SingleHeatCool`. The raw
`ZoneThermostatSetPointLo/Hi` snapshots are taken before adaptive, operative,
optimum-start, humidity-overcool, fault, comfort, or EMS changes. CP204 owns
the call order and copies but not the math, diagnostics, recurrence state, or
lifecycle inside the later-defined helpers:

- `AdjustAirSetPointsforOpTempCntrl` at lines 5863-5897;
- `AdjustOperativeSetPointsforAdapComfort` at lines 5899-5964;
- `CalcZoneAirComfortSetPoints` at lines 5966-6329;
- `AdjustCoolingSetPointforTempAndHumidityControl` at lines 6417-6458;
- `OverrideAirSetPointsforEMSCntrl` at lines 6460-6555.

#### Optimum-start and thermostat-fault order

`SingleHeatCool` tests only whether the global `OptStart` array is allocated.
When `OptStart(ActualZoneNum).ActualZoneNum == ActualZoneNum`, it computes
`OccStartTime = ceil(OccStartTime) + 1` and samples

```text
setpts[SingleHeat].heatSetptSched
    .getDayVals(state)[OccStartTime * TimeStepsInHour]
```

into generic `setpt`. This deliberately reads the `SingleHeat` family rather
than the currently selected `SingleHeatCool` family. An independent
`OptStartFlag` test then copies the current generic value to both bounds.
Consequently, a matching identity with a false flag can leave generic
`setpt` at the occupied value while both bounds retain the normal adjusted
value; a true flag with a mismatched identity copies the normal current value.

`DualHeatCool` uses the same rounded-plus-one time and array index. A matching
identity samples the dual cooling and heating day arrays into the global
occupied cool/heat entries. The independent flag then copies those entries to
high and low before humidity overcool. Because both occupied arrays were
globally reset at entry, a malformed mismatched identity with a true flag can
write the default `100.0` high and `0.0` low. CP204 performs no hour or array
index validation.

After each ordinary switch, the thermostat-fault block runs only when
`NumFaultyThermostat > 0` and warmup, sizing, and kickoff are all false. It
scans fault definitions in stored order and stops at the first
case-insensitive match with the controlled-thermostat name, even if that
fault's availability schedule is nonpositive. For an available match, severity
defaults to one or is sampled from its optional schedule; CP204 subtracts
`severity * Offset` from `setpt`, `setptLo`, and `setptHi`. It does not change
the raw low/high snapshots or `setptAdapComfortCool`.

The unconditional three-field subtraction is branch-independent. It may
therefore offset stale fields and can accumulate across a same-state retry:
all three fields after Uncontrolled, all three after an unused SingleHeat,
high after SingleHeat, low after SingleCool, and generic after Dual can lack a
fresh switch write.

#### Comfort and EMS final precedence

Only after every ordinary record and fault block has finished does CP204 test
`NumComfortControlledZones > 0` and call
`CalcZoneAirComfortSetPoints(state)`. Comfort processing can revisit a Zone
and replace ordinary schedule, adjustment, and fault results. CP204 then calls
`OverrideAirSetPointsforEMSCntrl(state)` unconditionally. The EMS child owns
the final source precedence over reached ordinary and comfort thermostat
values. Neither child's internal control-family equations, diagnostics,
outputs, latches, or actuator state count as CP204 implementation.

#### Validation, failure, retry, and reset

Counts, array allocation and shape, `ActualZoneNum`, schedule pointers, control
casts, `isUsed` consistency, setpoint-family pointers, optimum-start identity
and time topology, `TimeStepsInHour`, fault arrays and pointers, and all
numeric values are trusted. An invalid switch value emits one Severe message
containing the Zone, sampled value, and schedule name, but sets no local error
flag and does not fatal; the routine proceeds through fault handling, later
records, comfort, and EMS.

There is no local latch, status, catch, cleanup, or rollback. A schedule,
allocation, formatting, diagnostic, or child non-return preserves every
completed reset, allocation, record, raw snapshot, adjustment, optimum-start,
fault, comfort, or EMS prefix and suppresses the remaining suffix. The comfort
child also owns first-time state that CP204 does not reset.

Complete repeat resamples every reached schedule, resets both occupied arrays,
and repeats helpers and diagnostics. It is not generally idempotent because
the switch does not overwrite every fault-adjusted field, and helper warning,
recurrence, comfort, or actuator state can advance. CP199's selected
begin-environment initialization is not a complete replay reset. Clean replay
requires reconstruction or coordinated reset of ZoneControls,
HeatBalFanSys control/report and setpoint arrays, Availability optimum-start,
the schedule manager, globals and environment/weather, FaultsManager, Zone
heat-balance MRT/RH inputs, People and thermal-comfort state, EMS overrides and
actuators, diagnostics, and every called helper owner.

#### C++ and Rust evidence boundary

Four C++ fixtures contain 21 direct call expressions and 33 thermostat-field
post-call assertions:

| Fixture | Calls | Thermostat-field assertions | Composition boundary |
|---|---:|---:|---|
| `SysAvailManager_OptimumStart` | 1 | 2 | the two low/high checks follow CP204 directly, but the asserted values are the unoccupied dual values rather than an active optimum-start override |
| `ZoneTempPredictorCorrector_ReportingTest` | 4 | 7 | exercises records whose schedules select all five nominal switch values, then composes the state with `calcPredictedSystemLoad` |
| `SetPointWithCutoutDeltaT_test` | 8 | 12 | every CP204 call is followed by CP202 before its low/high assertions |
| `TempAtPrevTimeStepWithCutoutDeltaT_test` | 8 | 12 | every CP204 call is followed by CP202 before its low/high assertions |

A helper-only adaptive fixture makes four direct
`AdjustOperativeSetPointsforAdapComfort` calls with four assertions, and an
EMS fixture makes two direct `OverrideAirSetPointsforEMSCntrl` calls with four
assertions. There is no direct test of
`AdjustAirSetPointsforOpTempCntrl`,
`AdjustCoolingSetPointforTempAndHumidityControl`, or
`CalcZoneAirComfortSetPoints`. No composed CP204 fixture covers active
adaptive or operative control, humidity overcool, active optimum-start
override, fault offset, comfort overwrite, EMS overwrite, invalid control,
partial failure, retry, or reset.

Of 57 active C++ `ManageSimulation` call sites, one expected EMS fatal stops in
the `BeginTimestepBeforePredictor` callback before CP204. The other 56 reach
the routine; one zero-Zone weather fixture still executes the global
reset/allocation and final EMS child but has no record loop. Static enclosing
input evidence finds thermostat declarations in 38 of those 56 configurations
and none in 18. The 38 include 35 Dual, six SingleHeat, and six SingleCool
declarations with overlap, but no positive comfort, operative-temperature,
temperature-and-humidity overcool, optimum-start, or thermostat-fault object.
No full-simulation assertion directly checks CP204 setpoint or control state.

Rust defines `calc_zone_air_temp_set_points_compat<T>` only as
`execute()` and calls it once around an empty closure in
`heat_balance/timestep.rs`. That call is nested inside a hard-coded
`PredictStep` wrapper, whereas the source invokes CP204 only for
`GetZoneSetPoints`; the Rust enum value exists but has no live dispatch use.
There is no direct wrapper test and the heat-balance state has no thermostat
setpoint, control/report, comfort, optimum-start, or fault owner.

The compiler retains only direct-Zone `ZoneControl:Thermostat` records pointing
to `ThermostatSetpoint:DualSetpoint` plus their control-type schedule and
cutout delta. It rejects SingleHeating and does not evaluate the control-type
schedule at runtime. A CLI IdealLoads diagnostic helper selects the first
control, ignores that schedule, and repeats two `Schedule:Constant` hourly
values as observed series; it is not called by the heat-balance runtime.
Compiler tests cover graph retention, a missing schedule, and rejection of
SingleHeating, while the runtime test checks only that an
`EvaluateZoneThermostat` plan label precedes `SolveZone`. These inputs and
metadata do not implement CP204's branch transaction.

Rust has no runtime implementation of SingleHeat, SingleCool,
SingleHeatCool, staged control, operative/adaptive comfort, thermal-comfort
setpoints, temperature-and-humidity overcool, optimum start, EMS thermostat
override, or thermostat faults. Parsed cutout delta is unused. Adjacent
humidistat/IdealLoads moisture code and adaptive system-timestep code have
different ownership and receive no CP204 credit.

CP204 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.calc_zone_air_temp_set_points`
immediately after
`routine.zone_space_heat_balance_predict_system_load`. The heat-balance
project contract adds `calc_zone_air_temp_set_points` after
`zone_space_heat_balance_predict_system_load` and before
`update_final_surface_heat_balance`. The algorithm remains a `scaffold` with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 212 routines, split 58 `state_mapped` plus 154
`source_mapped`, with 89 required; the heat-balance project list becomes 58.

CP205 next maps
`ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio(EnergyPlusData &state,
Real64 RAFNFrac, int zoneNum, int spaceNum = 0)`, declared at
`ZoneTempPredictorCorrector.hh` line 243 and implemented at
`ZoneTempPredictorCorrector.cc` lines 3462-3815.

## Promotion Requirements

An official ExampleFile zone-air series may become conformance only after:

- Rust computes the hourly series without reading EnergyPlus ESO values.
- warmup exclusion/inclusion is explicit and matches the report contract.
- zone timestep count, hourly reporting timestamp, and run-period dates match.
- all heat inputs used in the promoted case have source-map entries.
- failure deltas are below declared max absolute and RMSE tolerances.
- the case has a blocking gate and `conformance_claim=true`.

## Current Boundary

`Zone Mean Air Temperature`, `Zone Air Heat Balance Surface Convection Rate`,
and `Zone Air Heat Balance Air Energy Storage Rate` are now conformance for the
official `1ZoneUncontrolled` compatibility candidate, in addition to the older
no-mass local MAT case. The broad diagnostic and probe lanes still exist for
source-order investigation, but their diagnostic rows do not inherit the
candidate claim.

The EnergyPlus moist-air capacitance equations are runtime helpers and are now
wired into the active `1ZoneUncontrolled` dynamic candidate solver immediately
before `AirPowerCap`/zone-air coefficient construction. The current lane uses
the timestep-interpolated weather pressure/RH as a temporary `airHumRat` proxy.
Porting owned `ZoneAirHumRat` remains necessary before widening the claim
beyond the declared official dynamic candidate variables.

The 2026-06-20 adaptive system-timestep report fix aligns the subdivided
system-timestep `Zone Air Heat Balance Air Energy Storage Rate` average with
the same weather-proxy air heat capacity used by the single-timestep report
path, after the substep temperature and humidity update. That lowered the
official dynamic candidate's air-storage max absolute delta from
`0.533385790012 W` to `0.182078359183 W` and moved the blocking gate to
`status = pass` without changing the claim boundary. The 2026-06-22
source-order promotion now pins the official candidate's zone-air correction to
`after-surface-loop`, lowering the same row further to `0.076879349871 W` max
absolute delta and `0.005076386180 W` RMSE while keeping the declared gate at
`status = pass`.

The current third-order coupled probe is a useful candidate, not a promotion.
On the frozen-hconv interleaved grey-longwave surface lane it lowers MAT RMSE
to `0.069817 C` and floor heat-storage RMSE to `54.593582 W`, but the latent
`Zone Air Heat Balance Surface Convection Rate` and `Zone Air Heat Balance Air
Energy Storage Rate` rows rise to `29.623453 W` and `29.666388 W` RMSE. Keep
the next zone-air work on source-order parity and owned moist-air capacitance,
not on a standalone temperature-solver swap.

A non-frozen-hconv third-order sibling confirms that the latent air-balance
regression is not solved by simply unfreezing hconv. It nudges MAT and the two
latent zone-air RMSE rows to `0.069191 C`, `28.637227 W`, and `28.446243 W`, but
raises the current floor heat-storage/inside-conduction/outside-conduction rows
to `58.289839 W`, `33.704368 W`, and `24.970278 W`. That keeps the next
zone-air target on coefficient/source ordering rather than a different hconv
cadence.

A weather-proxy moist-air storage report fork then isolates the storage side of
that regression. It leaves the frozen third-order MAT and floor RMSE rows
unchanged, but lowers `Zone Air Heat Balance Air Energy Storage Rate` RMSE from
`29.666388 W` to `5.845285 W`; `Zone Air Heat Balance Surface Convection Rate`
stays at `29.623453 W`. This points the remaining latent zone-air work at
surface convection source-order/coefficient timing, while proper zone
`airHumRat` ownership remains required before promoting the moist-air capacity
formula into the active solver.

A previous-MAT surface-convection report probe was added to test whether the
remaining zone surface-convection row was using `ZTM[0]` rather than corrected
MAT as the reference air temperature. It is a rejected path: the MAT/floor/air
storage rows are unchanged, but `Zone Air Heat Balance Surface Convection Rate`
RMSE rises from `29.623453 W` to `104.589141 W`. Keep the next work on
`SurfTempInTmp`/hconv/source-order parity instead of changing the report
reference to previous MAT.

A balance-closure surface-convection report probe narrows the same latent row
without changing the active solver path. In the no-load/no-infiltration
`1ZoneUncontrolled` diagnostic, reporting surface convection as
`CzdTdt - SumIntGains` leaves MAT, floor rows, and weather-proxy air storage
unchanged while lowering `Zone Air Heat Balance Surface Convection Rate` RMSE
from `29.623453 W` to `19.203798 W`. Keep this as source-isolation evidence
only: EnergyPlus reports the direct `SumHADTsurfs` surface sum, so the
remaining work is still `SurfTempInTmp`/hconv/source-order parity plus owned
zone humidity before any conformance promotion.

A frozen-reference-air surface-solve probe improves the zone state but exposes
the remaining coupled-source trade-off. Holding the surface pass reference air
at the timestep-start MAT while still correcting zone air after each pass lowers
MAT RMSE from `0.069817 C` to `0.031508 C` and the mass-floor face-temperature
RMSEs from about `0.0534 C` to about `0.0322 C`. The latent zone-air rows
regress, though: balance-closure surface convection rises from `19.203798 W` to
`21.039586 W`, and weather-proxy air storage rises from `5.845285 W` to
`7.495999 W`. This makes frozen surface reference air a useful cadence clue,
not a standalone promotion path.

A current-pass interior-longwave sibling was added on top of frozen reference
air after source rechecking EnergyPlus' CTF-only inside loop. It leaves the
zone-air picture essentially unchanged: MAT nudges from `0.031508 C` to
`0.031507 C`, but surface convection remains `21.039633 W` and weather-proxy
air storage remains `7.496023 W`. The next zone-air work is therefore still
`SurfTempInTmp`/hconv/source-order parity plus owned zone humidity, not
longwave sampling cadence alone.

Adding the EnergyPlus inside-surface convergence cutoff to that same lane
improves MAT slightly (`0.031507 C` to `0.030867 C`) and lowers the floor and
aggregate conduction rows, but it does not solve the latent zone-air rows:
surface convection rises to `21.105254 W` and weather-proxy air storage rises
to `7.547299 W`. Keep the convergence cutoff as a source-aligned surface
cadence candidate, while the next zone-air work remains the explicit
`SurfTempInTmp`/hconv report path and owned humidity/capacitance.

On the promoted ScriptF-flat, 20-iteration source-order compatibility lane, the
alias now resolves to the execution variant before selecting surface convection
report timing. This applies the ScriptF-flat surface reference-air snapshot only
to the individual `Surface Inside Face Convection Heat Gain Rate` rows while
leaving the zone-air `SumHADTsurfs` report path on its existing source. The
official dynamic diagnostic now drops floor inside-convection RMSE from
`20.828820 W` to `0.021677 W`, roof from `18.955600 W` to `0.044044 W`, and
the wall rows below `0.018 W` RMSE. `Zone Air Heat Balance Surface Convection
Rate`, MAT, floor storage, and surface conduction are unchanged, so the fix is
report-scope only.

A final-hconv report sibling then tested whether EnergyPlus' reported
`SurfHConvInt` could be approximated by recomputing TARP from the final
`SurfTempIn` and report reference air while keeping the solver frozen. It is
still a rejected report path: under the current all-EIO, EnergyPlus-surf-initial
compatibility setup it worsens floor storage from `0.175929 W` to
`7.535715 W` RMSE and zone surface convection from `0.063018 W` to
`11.729318 W` RMSE. The remaining broad diagnostic gap is therefore not solved
by final hconv-only reporting.

A live-hconv solve sibling then refreshed TARP inside convection coefficients
during interleaved solve passes on the active ScriptF-flat lane. A sparse
30-pass re-evaluation improves the individual inside convection report rows
but regresses the promoted zone/surface state: zone surface convection rises
from `0.063018 W` to `4.500161 W` RMSE, floor storage from `0.175929 W` to
`7.581421 W`, and floor outside conduction from `0.075458 W` to `3.321042 W`.
Keep the active solve on frozen inside convection while future work maps the
exact EnergyPlus `InitIntConvCoeff` cadence without perturbing the promoted
state rows.

An inside-CTF report sibling then tested whether the aggregate conduction rows
should use the outside-temperature snapshot consumed by the last inside CTF
solve. It leaves MAT, surface convection, air storage, floor storage, and
individual floor conduction unchanged, but worsens zone opaque aggregate
conduction from `18.143612 W` to `22.208305 W` inside and from `11.590547 W` to
`12.785602 W` outside. The zone-air/aggregate report path therefore still needs
EnergyPlus advanced report timing mapped separately from the surface CTF solve
snapshot.

A zone surface-report aggregate sibling then summed per-surface conduction
report helpers for the zone opaque aggregate rows, matching the EnergyPlus
`UpdateThermalHistories` aggregate shape. It is neutral: MAT, surface
convection, air storage, zone inside/outside aggregate conduction, and floor
storage all retain the older pre-all-EIO ScriptF-flat RMSE values (`0.037329 C`,
`22.062956 W`, `9.127258 W`, `18.143612 W`, `11.590547 W`, and `28.786920 W`).
The next zone-air work therefore stays on `CalcZoneComponentLoadSums` timing,
`SurfTempInTmp`/hconv ownership, and upstream surface/source/history parity
rather than on a zone aggregate accumulator source swap.

EnergyPlus `DataHeatBalance.cc::AirReportVars::setUpOutputVars` registers the
zone air heat-balance component rows as `System/Average`, while `Zone Mean Air
Temperature` remains `Zone/Average`. Rust therefore keeps hourly averaging as
the default report contract and adds `zone_air_report_sampling=last-system-state`
only as a diagnostic probe to isolate whether the remaining `SumHADTsurfs` gap
comes from system-timestep sampling rather than surface/source state ownership.
This historical probe is rejected as a promotion path: on the pre-all-EIO
ScriptF-flat lane it leaves MAT and floor storage unchanged (`0.037329 C` and
`28.786920 W`) while
worsening `Zone Air Heat Balance Surface Convection Rate` from `22.062956 W` to
`28.645122 W` RMSE and `Zone Air Heat Balance Air Energy Storage Rate` from
`9.127258 W` to `42.591381 W` RMSE.

An adiabatic-report sibling then tested whether EnergyPlus reports the
adiabatic floor outside face after syncing it to the current inside face while
still committing the pre-sync outside snapshot to CTF history. This is rejected:
MAT, zone surface convection, and air storage stay unchanged, but floor outside
conduction jumps from `12.216935 W` to `747.544527 W` RMSE and floor storage
from `28.786920 W` to `732.801403 W`. The current ScriptF-flat lane should
therefore keep the adiabatic outside report state on the pre-sync outside
snapshot; the remaining floor storage gap is not a missing current-inside
outside-face report sync.

The official dynamic diagnostic digest/report now tracks zone
surface-convection report closure against the signed sum of individual
`Surface Inside Face Convection Heat Gain Rate` rows (`zone + surface_sum`). On
the active ScriptF-flat lane the surface report rows are now near oracle, while
the zone row remains a separate report source. Because EnergyPlus does not make
`SumHADTsurfs` a direct negative sum of `SurfQdotConvInRep`, the remaining
`SumHADTsurfs` work should stay on EnergyPlus `CalcZoneComponentLoadSums`
timing, `SurfTempInTmp`, and `getInsideAirTemperature`/hconv ownership rather
than directly summing surface report heat-gain rows.

The June 2026 EnergyPlus 26.1.0 source audit narrows this further:
`ZoneHeatBalanceData::calcSumHAT` is the solver-coefficient path and
`CalcZoneComponentLoadSums` is the report path. Both consume
`SurfTempInTmp` and `SurfHConvInt`, but the report path independently calls
`Surface::getInsideAirTemperature` and writes `AirReportVars::SumHADTsurfs`.
The official zone-air surface-convection row should therefore be diagnosed
against `SumHADTsurfs` ownership, not inferred from surface report rows or from
the solver `SumHA/SumHATsurf/SumHATref` coefficients alone.

An inside-surface loop ordering probe then tested the EnergyPlus source-order
fact that `CalcHeatBalanceInsideSurf*` converges surface temperatures before
zone-air correction. The broad `energyplus-heat-balance-compat-candidate`
diagnostic wrapper and the promoted official compatibility candidate now pin
`surface_loop_zone_air_correction=after-surface-loop`; `each-surface-iteration`
remains available explicitly as a comparison probe. This reduces the official
candidate's zone surface-convection max absolute delta from `0.320300753509 W`
to `0.085845581243 W`, air-storage max absolute delta from `0.182078359183 W`
to `0.076879349871 W`, and floor storage max absolute delta from
`0.909524480086 W` to `0.663522464624 W`. The remaining broad-diagnostic gaps
therefore stay on `SurfTempInTmp` update parity, ScriptF/longwave source
ownership, inside hconv re-evaluation state, exact non-window convergence set,
and outside quick-balance current face temperature parity.

The EnergyPlus inside-hconv source cadence has now been split from compensating
probe values. `DataHeatBalSurface.hh::ItersReevalConvCoeff` is `30`, and the
new `hconv-reeval30-iter20` wrapper was first checked on the older pre-all-EIO
ScriptF-flat lane with that cadence plus the source-aligned
`energyplus-surf-initial` CTF seed. Because that lane caps each inside-surface
solve at twenty passes, this cadence is neutral in that historical 1Zone
diagnostic: MAT remains `0.037329 C` RMSE, zone
surface convection `22.062956 W`, air storage `9.127258 W`, inside-surface
iteration count `10.643041`, and floor storage `28.786920 W`. Re-evaluating
hconv every two passes is still useful as a sensitivity check but is not
source-parity: it improves zone surface convection (`22.062956 W` to
`20.723652 W`), inside-surface iteration count (`10.643041` to `8.639204`),
and floor storage (`28.786920 W` to `27.005834 W`), while worsening MAT
(`0.037329 C` to `0.037718 C`), air storage (`9.127258 W` to `9.576803 W`),
floor inside hconv (`0.025744` to `0.037803 W/m2-K`), and floor inside
convection (`13.602803 W` to `17.038813 W`). Keep future promotion work on the
official 30-pass cadence and target the remaining `SurfTempInTmp`/hconv state
ownership mismatch directly.
