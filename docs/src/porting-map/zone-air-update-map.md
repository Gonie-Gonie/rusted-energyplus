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
| Zone/Space predicted moisture demand | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio` | adjacent `calc_no_oa_third_order_moisture_demand_compat` guarded Zone-only subset and fixed-one-step IdealLoads Humidistat loop | CP205 required source-mapped humidistat/fault/sizing selection, airflow/AFN coefficients, three solution algorithms, Zone/Space demand reporting, and failure transaction; no heat-balance Rust implementation |
| Zone/Space air correction and component reporting | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::correctZoneAirTemps` | `correct_zone_air_temps_compat` identity closure around Zone-only temperature/humidity correction and project-specific adaptive work | CP206 required source-mapped Zone-first correction, conditional Space correction or mirroring, maximum-change fold, and component-report dispatch; no exact Rust topology, return, or lifecycle parity |
| Zone/Space record-level air correction | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::correctAirTemp` | surface-driven Zone-only coefficient, ThirdOrder, and Analytical helpers plus a separate history-only humidity pass | CP207 required source-mapped history/capacitance, controlled/uncontrolled solve, RoomAir/node/demand/hybrid/humidity order, and returned delta; no exact Rust Zone/Space lifecycle or numerical parity |
| Zone/Space Zone-timestep history dispatch | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::PushZoneTimestepHistories` | `push_zone_timestep_histories_compat` identity closure around an earlier three-slot Zone-only shift and predictor work | CP208 required source-mapped Zone-first and aggregate-flag Space dispatch; no exact Rust timing, topology, or record-child parity |
| Zone/Space record-level Zone-timestep history commit | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::pushZoneTimestepHistory` | adjacent inline three-slot Zone-only temperature/humidity shift | CP209 required source-mapped four-slot record, psychrometric, non-ThirdOrder, and Zone-only RoomAir/AFN transaction; no exact Rust record or lifecycle parity |
| Zone/Space system-timestep history dispatch | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::PushSystemTimestepHistories` | per-Zone named compat synchronization and adaptive local-history loop | CP210 required source-mapped strict fine-step global Zone-first/Space dispatch; no exact Rust gate, cadence, topology, or record-child parity |
| Zone/Space record-level system-timestep history commit | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::pushSystemTimestepHistory` | no singular helper; nearest paths rebuild or locally shift three-slot Zone histories | CP211 required source-mapped four-slot record commit plus conditional RoomAir/AFN and non-ThirdOrder state; no exact Rust record, Space, cadence, or auxiliary-state parity |
| Zone/Space Zone-timestep history revert dispatch | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::RevertZoneTimestepHistories` | per-Zone compat current-state reset in the local adaptive count-greater-than-one path | CP212 required source-mapped dormant global Zone-first/Space dispatch; no built-in source request or exact Rust timing, topology, child-state, or selector parity |
| Zone/Space record-level Zone-timestep history revert | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::revertZoneTimestepHistory` | no singular helper; nearest Rust paths push three-slot Zone or local system histories in the opposite direction | CP213 required source-mapped four-slot forward copy plus exact-Zone RoomAir/AFN branches and the literal mixed-level slot anomaly; no built-in reach or Rust record parity |
| Zone/Space record-level humidity correction | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::correctHumRat` | history-only Zone humidity passes plus a separate bounded no-OA ThirdOrder IdealLoads helper | CP214 required source-mapped airflow/coefficient solve, clamps, RoomAir/hybrid/node/latent-sizing effects, and failure transaction; no exact Rust Zone/Space record parity |
| history down-interpolation overload family | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::DownInterpolate4HistoryValues` scalar-output and array-return overloads | three-output thermal Zone history helper only | CP215 maps the contaminant-owned five-reference overload; CP216 expands the same required source-mapped routine to the thermal four-slot array/returned-current overload, with no exact Rust width, topology, invalid-input, alias, or lifecycle parity |
| hybrid inverse temperature inference | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::InverseModelTemperature` | no typed `HybridModel:Zone` object or inverse-model state/runtime path | CP217 required source-mapped measured-temperature override, infiltration/internal-mass/people inverse branches, and unconditional measured-history shift; no Rust parser, state, output, test, or execution parity |
| hybrid thermal-mass multiplier postprocessing | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::processInverseModelMultpHM` | no inferred multiplier, aggregate, or per-Zone recurring-warning state | CP218 required source-mapped lower clamp, uncapped over-limit diagnostics, persistent sum/count/average update, and caller transaction; no Rust parser, state, output, diagnostic, test, or execution parity |
| hybrid humidity inverse inference | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::InverseModelHumidity` | no typed `HybridModel:Zone`, measured humidity history, inverse state, or exact outputs | CP219 required source-mapped unconditional sampling/history shift plus date/history-gated infiltration and People inversion; no Rust parser, state, output, test, or execution parity |
| Zone/Space heat-balance sum assembly | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::calcZoneOrSpaceSums` | adjacent Zone-only opaque-surface hA/hAT helper, OtherEquipment convection subset, and zero-initialized flow fields | CP220 required source-mapped internal/non-system/system/surface transaction with parent-Zone AFN/equipment/plenum/PIU context and uncontrolled-Space system allocation; no exact Rust routine, Space topology, airflow writer, lifecycle, test, or execution parity |
| Zone heat-balance stored-Space result fold | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneHeatBalanceData::calcSumHAT` | direct Zone opaque-Surface index fold returning only HA/HATsurf/HATref=0 | CP221 required source-mapped four-field stored-Space child aggregation; no exact Rust Space record, dispatch/order, internal-gain return, child-side-effect, failure, test, or execution parity |
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
CP205 `ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio` at lines
3462-3815, CP206 `correctZoneAirTemps` at lines 3817-3861,
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
`zone_space_heat_balance_calc_predicted_humidity_ratio`. The algorithm remains
a `scaffold` with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 212 routines, split 58 `state_mapped` plus 154
`source_mapped`, with 89 required; the heat-balance project list becomes 58.

### CP205 `ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio` source map

`ZoneSpaceHeatBalanceData::calcPredictedHumidityRatio(EnergyPlusData &state,
Real64 RAFNFrac, int zoneNum, int spaceNum = 0)` is declared at
`ZoneTempPredictorCorrector.hh` line 243 and implemented at
`ZoneTempPredictorCorrector.cc` lines 3462-3815. Its only production call
expression is CP203 line 3256, after `calcPredictedSystemLoad`. CP202 and CP203
therefore invoke CP205 once for every Zone and, while Space heat balance is
active, once for every stored Space in the parent Zone. A Zone sensible demand
has already committed before its moisture call, and a Zone moisture demand can
commit before a later Space call fails.

#### Humidity-control setpoint selection

Each invocation reconstructs local humidifying and dehumidifying relative
humidity setpoints at zero, a single-setpoint flag at false, and a controlled
flag at false. It aliases the parent `Zone(zoneNum)` even for a Space record.
When that Zone has a positive `humidityControlZoneIndex`, CP205 trusts the
indexed humidity-control record, debug-asserts only its Zone identity, samples
the humidifying schedule before the dehumidifying schedule, and then applies
the humidifying EMS override before the dehumidifying EMS override.

Outside warmup, sizing, and kickoff, the routine scans stored humidistat faults
only when at least one exists. It stops at the first case-insensitive match
between the control name and `FaultyHumidistatName` even when that fault is
unavailable. A `ThermostatOffsetDependent` match ignores the humidistat
fault's own availability, severity, and offset and instead scans thermostat
faults for the referenced `FaultyThermostatName`. Failure to find the named
thermostat object emits Severe and then Fatal diagnostics regardless of the
humidistat fault's availability. The first matching thermostat object stops
that scan even when unavailable.

For an available dependent thermostat fault, severity defaults to one or is
sampled from its optional schedule and scales the thermostat fault offset.
Only a nonzero result transforms each RH value to humidity ratio at this
record's `MAT + offset` and back to RH at this record's `MAT`, then clamps
both results to [0, 100]. A found but unavailable thermostat produces no
transform and no clamp. Every other humidistat fault type uses the humidistat
fault's own availability and optional severity, subtracts the scaled offset
from both RH values, and clamps only on the active path. Schedule and EMS
values otherwise have no local range or finite guard.

If humidifying RH is greater than dehumidifying RH, the first occurrence emits
a Warning, Continue detail, and timestamp through the control-owned
`ErrorIndex`. Every occurrence also emits a recurring warning, passing the
humidifying value as both its minimum and maximum sample, and reduces
humidifying RH to dehumidifying RH. Exact equality sets the local
single-setpoint flag. The humidistat branch always marks the invocation
controlled. A Space invocation repeats this Zone-keyed schedule, EMS, fault,
warning, and recurrence-index path, but a dependent fault uses that Space
record's `MAT` for the psychrometric transform.

#### Latent-sizing fallback

Only `DoingSizing && !ControlledHumidZoneFlag && DoLatentSizing` enters the
fallback. CP205 scans one-based `ZoneEquipConfig` records, skips uncontrolled
entries, and stops unconditionally after the first controlled entry. It does
not compare that entry with `zoneNum`. Within that one entry it searches
`ZoneSizingInput` by the equipment configuration's Zone name and falls back to
the first sizing input when the search misses and the array is nonempty.

When the selected input enables Zone latent sizing, CP205 obtains
dehumidifying and then humidifying RH from schedules or stored constants,
silently reduces a reversed humidifying value to the dehumidifying value,
tests exact equality, and marks the invocation controlled. The unconditional
break means a first controlled configuration with no usable latent sizing
suppresses all later candidates. The same first configuration can therefore
control, or suppress control for, every otherwise uncontrolled Zone and Space
call during sizing.

#### Moisture coefficients and RoomAir AFN replacement

Controlled work first forms latent gain from this record's `latentGain` plus
the parent Zone's radiant-system and swimming-pool latent sums. It obtains
system-timestep seconds, moist-air density from pressure, `ZT`, and
`airHumRat`, and water-vapor enthalpy from `airHumRat` and `ZT`.

For multizone AirflowNetwork simulation, or distribution-only simulation while
the fan is active, `B` is latent gain divided by vapor enthalpy plus the
Zone-indexed `SumMHrW` and `SumMMHrW` exchange and this record's
`SumHmARaW`; `A` is the corresponding `SumMHr`, `SumMMHr`, and
`SumHmARa` sum. Otherwise `B` combines latent gain, outdoor/ventilation/
cross-mixing flow times outdoor humidity, exhaust flow times its humidity,
surface moisture, mixing humidity flow, and `MDotOA * OutHumRat`. `A`
combines the matching outdoor, ventilation, exhaust, cross-mixing, surface,
mixing, and outdoor-air mass flows.

`C` uses Space volume only for positive `spaceNum` and otherwise Zone volume,
then multiplies moist-air density, volume, and the parent Zone
`ZoneVolCapMultpMoist` and divides by unguarded system-timestep seconds. If
the parent Zone's RoomAir model is AirflowNetwork, CP205 replaces `A`, `B`,
and vapor enthalpy and recomputes `C` from its Zone control node's density and
volume. This
replacement has neither CP203's state-wide nonmixing gate nor its node
`IsUsed` guard, and it also replaces a Space record's otherwise Space-specific
state. The subsequent humidity setpoint conversions still use this record's
`ZT` rather than the control-node temperature.

#### Three algorithms, RAFN scaling, and load choice

For humidifying and then dehumidifying setpoints, CP205 converts RH fraction at
this record's `ZT` and environment pressure to humidity ratio. The ThirdOrder
load is

```text
((11 / 6) * C + A) * Wsp
  - (B + C * (3 * WPrev[0] - 1.5 * WPrev[1] + WPrev[2] / 3))
```

using the first three `WPrevZoneTSTemp` values. Analytical solution uses
`C * (Wsp - W1) - B` for exact `A == 0`. Otherwise the humidifying
calculation obtains `exp(min(700, -A / C))` and the dehumidifying calculation
reuses that same exponential in `A * (Wsp - W1 * e) / (1 - e) - B`. Euler
uses `C * (Wsp - W1) + A * Wsp - B`. An unrecognized solution enum leaves
both initialized loads at zero without a diagnostic.

A strictly positive `RAFNFrac` divides each calculated load. Zero, negative,
and NaN fractions bypass scaling. Exact-equal RH setpoints choose the
humidifying load without the sign matrix, so a nonfinite calculated load can
continue to reporting. Otherwise two positive loads choose humidifying, two
negative loads choose dehumidifying, and
`humidifying <= 0 && dehumidifying >= 0` selects zero. Every other
combination, including unequal-setpoint NaN comparisons, emits Severe and
detail diagnostics and fatals before the final demand commit.

#### Demand ownership and partial effects

Only positive `spaceNum` selects `spaceSysMoistureDemand(spaceNum)`; every
nonpositive value selects `ZoneSysMoistureDemand(zoneNum)`. Controlled work
calls
`reportMoistLoadsZoneMultiplier(state, zoneNum, total, humidifying,
dehumidifying)`. That child writes the three unmultiplied predicted rates,
then the three public values multiplied by the parent Zone's Zone and List
multipliers, and conditionally initializes all three sequenced-equipment
arrays when the parent Zone is controlled and the selected demand record has
equipment slots.

Uncontrolled work bypasses the child and directly zeros only
`TotalOutputRequired`, `OutputRequiredToDehumidifyingSP`, and
`OutputRequiredToHumidifyingSP`. It leaves predicted rates, sequenced arrays,
remaining values, unadjusted values, and report fields stale. The controlled
child likewise does not own remaining or unadjusted demand state.

Beyond the debug assertion, CP205 has no local count, shape, identity,
membership, pointer, schedule, EMS, fault, sizing, setpoint-range, pressure,
temperature, humidity, history, volume, multiplier, timestep, flow, AFN,
`RAFNFrac`, or finite-value validation and no latch, status, catch, cleanup,
or rollback. A missing dependent thermostat or impossible load combination
fatals before the final demand write. Schedule, psychrometric, allocation,
diagnostic, or report-child failure retains any completed warning,
`ErrorIndex`, predicted/public demand, or sequenced prefix. Same-state retry
resamples schedules and faults, advances recurring diagnostics, and repeats
all calculations and child writes. Clean replay requires coordinated reset of
Zone and Space heat-balance records, humidity controls and their schedules/EMS
and `ErrorIndex`, both fault families, sizing and Zone equipment, timestep and
environment/psychrometrics, radiant and pool latent gains, AFN/RoomAir,
Zone/Space moisture demand and sequences, multipliers, diagnostics, and the
outer CP202/CP203 owners.

#### C++ reach and Rust boundary

No C++ test calls CP205 directly. One helper-only fixture calls
`ZoneSystemMoistureDemand::reportMoistLoadsZoneMultiplier` four times with 21
assertions, covering only Zone raw and multiplied values rather than Space,
sequenced, or uncontrolled-zero behavior. Of 57 active
`ManageSimulation` tests, one expected EMS fatal stops before prediction and
one zero-Zone weather case reaches CP202 without CP205. The other 55 execute
at least one Zone CP205 and eight enable Space CP205. Their inputs contain no
humidistat, humidistat fault, humidistat EMS override, or RoomAir AFN. Only one
ThirdOrder latent-sizing configuration enters positive controlled equations,
for one Zone and three Spaces, and its assertions are downstream. Six
Analytical configurations remain uncontrolled, and no reached Euler
configuration exists.

Rust has no heat-balance CP205 identity wrapper, production call, Zone/Space
moisture-demand owner, or corresponding runtime mutation. The separate
`calc_no_oa_third_order_moisture_demand_compat` implements a guarded
Zone-only no-outdoor-air, `A = 0`, ThirdOrder subset: density and capacity,
internal latent gain, two RH conversions, three-value history, sign/deadband
selection, and one combined Zone multiplier. It has two non-test callers, two
direct predictor tests, and one adjacent history-term test. The fixed-one-step
IdealLoads Humidistat wrapper composes predictor, PurchasedAir, correction,
and history push in seven tests with eight wrapper call expressions.

That Rust subset rejects invalid input through `Option`, always clamps RH
fractions, and returns only multiplied loads. It omits source-owned schedules,
EMS, both fault modes, latent-sizing selection, radiant/pool gains, outdoor,
ventilation, exhaust, cross-mixing, mixing, surface-moisture, AFN, RoomAir,
`RAFNFrac`, Space state, Analytical/Euler algorithms, diagnostics and partial
effects, uncontrolled public-zero writes, raw predicted rates, and sequenced
demand. Its CLI still consumes EnergyPlus-seeded histories, temperatures,
sensible demand, and pressure. Existing manifest-specific no-OA IdealLoads
evidence therefore remains separately bounded and does not state-map CP205.

CP205 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_space_heat_balance_calc_predicted_humidity_ratio`
immediately after `routine.calc_zone_air_temp_set_points`. The heat-balance
project contract adds `zone_space_heat_balance_calc_predicted_humidity_ratio`
after `calc_zone_air_temp_set_points` and before `correct_zone_air_temps`. The
algorithm remains a `scaffold` with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code,
mapped state, test, support, capability, output implementation, comparator,
manifest, numerical, performance, or conformance promotion is added. The
inventory becomes 32 algorithms and 213 routines, split 58 `state_mapped` plus
155 `source_mapped`, with 90 required; the heat-balance project list becomes
59.

### CP206 `correctZoneAirTemps` source map

`correctZoneAirTemps(EnergyPlusData &state,
bool useZoneTimeStepHistory)` is declared at
`ZoneTempPredictorCorrector.hh` lines 289-291 and implemented at
`ZoneTempPredictorCorrector.cc` lines 3817-3861. Its only production direct
call is CP195 `ManageZoneAirUpdates` line 230. The `CorrectStep` arm assigns
the returned value to caller-owned `ZoneTempChange` only after CP206 returns;
`ShortenTimeStepSys` and `PriorTimeStep` are not forwarded.

The ordinary `HVACManager::ManageHVAC` path runs Get setpoints, Predict, and
`SimHVAC` before its initial Correct call. It selects adaptive system-timestep
shortening only when that first returned maximum exceeds `MaxZoneTempDiff` and
`KickOffSimulation` is false; otherwise it uses one system step and
Zone-timestep history. Every selected fine step repeats Predict, `SimHVAC`,
Correct, contaminant correction, and system-history push. Fine-step CP206
returns overwrite the local `ZoneTempChange` but do not recalculate the already
chosen substep count. The demand-resimulation path runs only Get and Predict
before `SimHVAC` and does not enter CP206.

#### Per-Zone and Space order

CP206 initializes `maxTempChange` to zero and traverses
`zoneNum = 1..NumOfZones`. Within each Zone the exact order is:

1. Alias the Zone heat-balance record and call
   `correctAirTemp(state, useZoneTimeStepHistory, zoneNum)`. Its result is
   saved locally but not folded yet.
2. Alias the parent Zone and visit every stored `spaceIndexes` identity,
   regardless of either Space heat-balance flag.
3. Under `doSpaceHeatBalanceSimulation && !DoingSizing`, call the Space
   `correctAirTemp` child with both identities and immediately fold that
   result into the running maximum.
4. In every other case, conditionally mirror the parent Zone system node's
   `Temp`, `HumRat`, and `Enthalpy` to the Space system node when
   `doSpaceHeatBalanceSizing && Zone.IsControlled`.
5. Still in that fallback, unconditionally copy Zone `ZT`, complete `ZTM`,
   `MAT`, `airHumRat`, and `airRelHum` to the Space heat-balance record.
   The commented `ZTAVComf` assignment performs no write.
6. Only after every Space does CP206 fold the saved Zone correction result.
7. Call `CalcZoneComponentLoadSums` unconditionally for the Zone record and
   `ZnAirRpt(zoneNum)`.
8. When `doSpaceHeatBalanceSimulation` is true, traverse the stored Spaces a
   second time and call the report child for each Space record and
   `spaceAirRpt(spaceNum)`.

The node-copy condition is nested in the broad correction fallback and has no
direct `DoingSizing` test of its own. Conversely, the Space report gate has no
`!DoingSizing` condition, so a simulation-Space record mirrored during sizing
still receives component reporting. A fallback-mirrored Space contributes no
temperature-change candidate. The `useZoneTimeStepHistory` input is only
forwarded to correction children; CP206 does not inspect or store it.

`ZoneSpaceHeatBalanceData::correctAirTemp` is the following CP207 source
definition and owns temperature/humidity correction, hybrid, RoomAir,
solution-algorithm, node, and returned-delta behavior. The much later
`CalcZoneComponentLoadSums` definition owns component report calculations.
CP206 owns their Zone/Space dispatch order and arguments but receives no
implementation credit for either child's internal equations or mutations.

#### Maximum, direct writes, and edge behavior

CP206 applies no absolute value, tolerance, clamp, or finite check. Normal
correction children return nonnegative absolute temperature changes, so normal
output is the maximum over every Zone and only actively corrected Spaces.
For each two-argument `std::max` call, the accumulator is first and the child
candidate second. Starting from zero therefore ignores a negative or NaN
candidate and retains positive infinity. A nonpositive `NumOfZones` executes
no body and returns zero.

The wrapper's only direct persistent writes are the optional three Space-node
fields and five fallback Space heat-balance fields. The running maximum is
local until return. Zone and active-Space correction, moisture state, hybrid
state, component reports, and all child diagnostics remain dependency effects.

CP206 has no local assertion, identity, count, membership, arena-shape,
allocation, Space, node, report, finite, or topology validation. It emits no
diagnostic and owns no stop flag, status, latch, catch, cleanup, transaction,
or rollback.

A Zone-child non-return preserves prior Zones and that child's prefix while
suppressing the current Zone's Spaces, fold, reports, and every later Zone. An
active Space failure follows the completed Zone child and any earlier Spaces,
but occurs before the saved Zone result is folded and before that Zone's
reports. Node or heat-balance mirroring can retain a prefix of the three or
five direct writes. A component-report failure follows all current correction,
copy, and maximum work and can retain earlier report mutations while
suppressing the remaining reports and Zones.

Because CP206 did not return, its local maximum is lost and CP195's right-hand
side assignment does not complete; the caller's old `ZoneTempChange` remains.
Same-state retry restarts at Zone one and repeats every correction child,
mirror, and report without an idempotency guard. CP206 owns neither adaptive
retry nor history push/revert. Clean replay requires coordinated reconstruction
of predictor/corrector records, Zone/Space heat balance and topology, nodes,
RoomAir, HVAC globals, report state, diagnostics, and both child owners; the
predictor/corrector owner's own `clear_state` is insufficient for all of them.

#### C++ test reach

`HybridModel_correctZoneAirTempsTest` is the only direct fixture. It makes
five calls, all with `useZoneTimeStepHistory = true`, using one Zone, one
stored Space, false `DoingSizing`, and default-false Space-HB flags. Every call
therefore reaches the Zone child and five-field Space mirroring, but not active
Space correction, sizing node mirroring, or Space component reporting. The
five post-call assertions each inspect a hybrid-model child effect. None checks
the returned `ZoneTempChange`, Space mirrors, component reports, false-history
path, failure, retry, or reset. There is no direct
`ZoneSpaceHeatBalanceData::correctAirTemp` test and no test-side
`ManageZoneAirUpdates(CorrectStep)` call.

Of 57 active `ManageSimulation` call expressions, one expected EMS fatal stops
at `BeginTimestepBeforePredictor` before CP206. The zero-Zone
`WeatherManager_SetRainFlag` case reaches CP206's zero-iteration return. The
other 55 execute at least one Zone correction.

Exactly one full-simulation configuration enables simulation Space heat
balance. `HeatBalanceAirManager_GetMixingAndCrossMixing` uses Analytical
solution and proves two Zones and three stored Spaces after the run, so it
reaches active Space correction and Space component reporting; its assertions
cover topology and mixing rather than CP206 outputs. Seven SizingManager
simulations enable Space HB only for sizing. Each has one controlled Zone and
three Spaces, so it reaches the controlled node triplet and five-field
mirroring with only downstream sizing-table assertions. An eighth related case
disables sizing Space HB and still traverses and mirrors its Spaces without the
node subbranch.

One thermal-comfort simulation asserts later corrected-Zone averages
`ZTAVComf` and `airHumRatAvgComf`. No test isolates the wrapper maximum,
`std::max` fold order, adaptive downstep choice, component-report dispatch,
partial failure, retry, or reset. The initial production call is statically
history-true. Fine-step history-false execution is source-valid after adaptive
downstepping, but no focused or full-simulation assertion proves that branch.

#### Rust boundary

Rust defines `correct_zone_air_temps_compat<T>` as an identity alias over
`correct_step_source_order_path`. It has one non-test call expression and no
direct test. The lower CorrectStep identity has four non-test expressions when
the alias delegation is counted and one generic pass-through test; several
surface-iteration and adaptive calls bypass the named CP206 wrapper.

The one live named-wrapper closure returns unit. It first performs one complete
all-Zone temperature pass, then one complete all-Zone humidity pass, and then
either performs project-specific adaptive per-Zone correction or updates
per-Zone histories and averages. This differs from CP206's Zone correction,
that Zone's Spaces and reports, then next-Zone transaction. It supplies no
source history-selector argument and produces no global maximum for an HVAC
caller.

Rust heat-balance state has Zone and Surface owners and Zone-indexed surface
membership, but no Space heat-balance record, Space system node, Space relative
humidity or enthalpy owner, `ZnAirRpt`, `spaceAirRpt`, or exact
`CalcZoneComponentLoadSums` transaction. The separate diagnostic/IdealLoads
node store is not passed to this heat-balance corrector. The Rust temperature
pass writes coefficient and report-adjacent Zone aggregates, but it is not the
source report child or CP206 order.

Rust adaptive work derives an independent substep count for each Zone from a
hard-coded 0.3 C threshold and 60 s minimum, excludes Space deltas, and repeats
only local temperature/humidity work against existing surface/HVAC inputs. It
does not implement the source's one global maximum, history-selector change,
or full Predict-HVAC-Correct retry sequence. Invalid cases use guards,
fallbacks, or early returns rather than CP206 failure-prefix semantics.

Adjacent coefficient, ThirdOrder, Analytical, history interpolation, adaptive
no-shortening, and storage-report helper tests cover formulas delegated to
CP207 or project-specific output behavior, not CP206's wrapper topology.
Existing official `1ZoneUncontrolled` MAT, surface-convection, and air-storage
evidence remains case-bounded. Authored Space execution, sizing, EMS, Python,
and AirflowNetwork are run-blocked, and no declared claim covers CP206's
hybrid, RoomAir, Space, sizing, component-report, or HVAC retry behavior.

CP206 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.correct_zone_air_temps`
immediately after
`routine.zone_space_heat_balance_calc_predicted_humidity_ratio`. The
heat-balance project contract adds `correct_zone_air_temps` after
`zone_space_heat_balance_calc_predicted_humidity_ratio` and before
`zone_space_heat_balance_correct_air_temp`. The algorithm remains a `scaffold`
with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 214 routines, split 58 `state_mapped` plus 156
`source_mapped`, with 91 required; the heat-balance project list becomes 60.

### CP207 `ZoneSpaceHeatBalanceData::correctAirTemp` source map

`ZoneSpaceHeatBalanceData::correctAirTemp(EnergyPlusData &state,
bool useZoneTimeStepHistory, int zoneNum, int spaceNum = 0)` is declared at
`ZoneTempPredictorCorrector.hh` lines 236-239 and implemented at
`ZoneTempPredictorCorrector.cc` lines 3863-4165.

Its only two production direct call expressions are both in CP206. Line 3825
calls the Zone record with the default `spaceNum = 0`. Line 3831 calls a Space
record with both positive identities only under
`doSpaceHeatBalanceSimulation && !DoingSizing`. CP206 orders the Zone before
eligible Spaces, so production supplies only zero or positive `spaceNum` and
folds CP207's return only after the complete record call returns.

#### History, capacitance, and sum order

CP207 starts local `tempChange` at zero, debug-asserts only
`zoneNum > 0`, aliases the parent Zone, computes
`ZoneMult = Multiplier * ListMultiplier`, and reads unguarded
`TimeStepSysSec`. It then writes both selected history arrays in this order:

| `useZoneTimeStepHistory` | `ZTM` source | `WPrevZoneTSTemp` source |
|---|---|---|
| false | complete `DSXMAT` | complete `DSWPrevZoneTS` |
| true | complete `XMAT` | complete `WPrevZoneTS` |

A positive `spaceNum` selects Space volume; zero or negative selects Zone
volume. CP207 then overwrites `AirPowerCap` from the selected volume, the
parent Zone sensible capacitance multiplier, and the record's pre-correction
`MAT` and `airHumRat`:

```text
AirPowerCap =
    volume * ZoneVolCapMultpSens
  * PsyRhoAirFnPbTdbW(OutBaroPress, MAT, airHumRat)
  * PsyCpAirFnW(airHumRat)
  / TimeStepSysSec
```

There is no local positive-time, volume, pressure, humidity, density, capacity,
or finite guard. Exact `spaceNum == 0` next calls
`RoomAir::ManageAirModel(state, zoneNum)`; every other identity skips that
child. Every call then invokes
`calcZoneOrSpaceSums(state, true, zoneNum, spaceNum)`. If
`FlagHybridModel_PC` is true, the current record's
`SumIntGainExceptPeople` is replaced from the parent Zone aggregate. The TODO
mentions Space heat balance, but the code has no Space gate, so an active Space
also receives that Zone value.

`RoomAir::ManageAirModel`, `calcZoneOrSpaceSums`, the psychrometric
routines, and the internal-gain aggregate remain dependency implementations.
CP207 owns their arguments and order, not their internal equations or
diagnostics.

#### Controlled and uncontrolled solves

`ZoneNodeNum` starts from the Zone system node and changes to the Space system
node only for positive `spaceNum`. A positive node number, rather than
`Zone.IsControlled`, selects the controlled/plenum branch.

The controlled coefficients are:

```text
TempDepCoef =
    SumHA + SumMCp + SumSysMCp

TempIndCoef =
    SumIntGain + SumHATsurf - SumHATref
  + SumMCpT + SumSysMCpT
  + NonAirSystemResponse / ZoneMult
  + SysDepZoneLoadsLagged
  + optional AFN exchangeData(zoneNum).TotalSen
  + optional DuctLoss.ZoneSen(zoneNum)
```

The uncontrolled branch omits system flow, non-air response, and lagged system
load while retaining the parent-Zone AFN and duct additions:

```text
TempDepCoef =
    SumHA + SumMCp

TempIndCoef =
    SumIntGain + SumHATsurf - SumHATref + SumMCpT
  + optional AFN exchangeData(zoneNum).TotalSen
  + optional DuctLoss.ZoneSen(zoneNum)
```

Both branches write `TempDepCoef` and `TempIndCoef`, then use the same
solution switch:

| algorithm | direct `ZT` assignment |
|---|---|
| `ThirdOrder` | `(TempIndCoef + AirPowerCap * (3*ZTM[0] - 1.5*ZTM[1] + ZTM[2]/3)) / ((11/6)*AirPowerCap + TempDepCoef)` |
| `AnalyticalSolution`, exact `TempDepCoef == 0` | `T1 + TempIndCoef / AirPowerCap` |
| `AnalyticalSolution`, otherwise | `(T1 - TempIndCoef/TempDepCoef) * exp(min(700, -TempDepCoef/AirPowerCap)) + TempIndCoef/TempDepCoef` |
| `EulerMethod` | `(AirPowerCap*T1 + TempIndCoef) / (AirPowerCap + TempDepCoef)` |
| unknown enum | no assignment; retain prior `ZT` |

The denominators, signs, enum, and finite values are trusted. Because 700 is
the first operand of the source `std::min`, a NaN analytical exponent candidate
selects 700. An unknown enum does not stop later node, demand, humidity, or
report work.

#### RoomAir, node, and sensible-load precedence

Without any global nonmixing model, the controlled branch copies `ZT` to the
selected system node, copies it to `TempTstatAir(zoneNum)` only for exact Zone
identity, and sets the Zone-shared `LoadCorrectionFactor` to one. With the
global flag set, the exact precedence is:

1. A Mixing model or `!SimAirModel` performs those fully mixed writes.
2. A three-node displacement or UFAD Zone leaves node temperature and
   thermostat untouched here. It derives a correction factor only when
   `SumSysMCp > SmallMassFlow` and
   `abs(SupplyTemp - ZT) > TempConvergTol`; otherwise it writes one.
3. A simulated UserDefined or one-node displacement model uses the same
   correction-factor branch.
4. An AirflowNetwork model replaces this record's `ZT` from the parent Zone
   control-air node, updates the selected system node, updates the thermostat
   only for exact Zone identity, and writes correction factor one.
5. The fallback performs the fully mixed writes.

The correction-factor calculation is:

```text
raw = (SupplyTemp - existing systemNode.Temp) / (SupplyTemp - ZT)
LoadCorrectionFactor = min(3, max(-3, raw))
```

The source orders `max(-3, raw)` before `min(3, ...)`. A NaN raw value
therefore becomes -3, while positive and negative infinity clamp to 3 and -3.
The displacement/UFAD flag checks precede the AirModel-enum branches.

A positive Space call uses its Space record and node but the parent Zone's
RoomAir flags and Zone-indexed `LoadCorrectionFactor`. It never writes
`TempTstatAir`, yet it can overwrite the Zone child correction factor, and
later Spaces can overwrite it again.

After those controlled RoomAir/node branches, CP207 computes local
`SNLoad` from the possibly retained or updated system node:

```text
SNLoad =
    SumSysMCpT
  - (systemNode.MassFlowRate / ZoneMult)
      * PsyCpAirFnW(airHumRat) * systemNode.Temp
  + NonAirSystemResponse / ZoneMult
  + SysDepZoneLoadsLagged
```

The uncontrolled branch leaves `SNLoad = 0` and does not write a system node,
thermostat, or correction factor. Only an exact Zone call under the global
nonmixing flag and AirflowNetwork enum replaces uncontrolled `ZT` from the
control-air node; it does not test `SimAirModel`.

#### Hybrid, demand, humidity, and returned delta

After either solve, an exact Zone call enters `InverseModelTemperature` only
when global hybrid modeling is enabled, at least one temperature inverse mode
is active, and neither warmup nor sizing is active. This child runs after the
thermal solve, node and correction-factor writes, and `SNLoad` calculation. It
can replace Zone `ZT` with measured temperature and advance measured history;
it consults global `UseZoneTimeStepHistory` rather than CP207's boolean.
Consequently the later `MAT`, humidity, relative humidity, and returned delta
can use measured `ZT` while the selected node, thermostat, and `SNLoad` still
reflect the pre-inverse solution.

The remaining order is unconditional:

1. write `MAT = ZT`;
2. report `SNLoad` to Space demand only for positive `spaceNum`, otherwise
   Zone demand;
3. call `correctHumRat(state, zoneNum, spaceNum)`;
4. after that child returns, write `airHumRat = airHumRatTemp`;
5. write `airRelHum = 100 * PsyRhFnTdbWPb(ZT, airHumRat, OutBaroPress)`;
6. classify and return the temperature change.

Sensible demand therefore commits before humidity correction. The much later
`correctHumRat` definition owns humidity equations, node humidity/enthalpy,
latent demand, and hybrid-moisture behavior. CP207 owns the child dispatch and
post-child humidity/RH writes only.

The return classifier starts mixed. Only an exact Zone identity under the
global nonmixing flag becomes nonmixed, and then only for three-node
displacement with `ZoneDispVent3NodeMixedFlag == 0` or UFAD with
`ZoneUFADMixedFlag == 0`. It does not consult the AirModel enum or
`SimAirModel`; every Space is return-classified as mixed.

| solution | mixed delta | nonmixed delta |
|---|---|---|
| `ThirdOrder` | `abs(ZT - ZTM[0])` | max of occupied and mixed room-air differences against `ZTMOC[0]` and `ZTMMX[0]` |
| `AnalyticalSolution` / `EulerMethod` | `abs(ZT - T1)` | max of occupied and mixed room-air differences against `Zone1OC` and `Zone1MX` |
| unknown | zero | zero |

The outer `std::max` has zero first and the candidate second, so a NaN
candidate is ignored while positive infinity survives. In the nested nonmixed
maximum, a NaN first delta suppresses a finite second delta, whereas a finite
first delta survives a NaN second delta. A normal return is zero, finite
nonnegative, or positive infinity. Humidity and RH must finish before CP206 can
receive or fold it.

#### Identity, writes, failure, retry, and reset

A malformed negative `spaceNum` is not rejected. It selects Zone volume, node,
and sensible demand through the positive tests, while skipping the exact-zero
RoomAir manager, thermostat write, uncontrolled AFN override, hybrid inverse,
and nonmixed return classifier. It still passes the negative identity to sum
and humidity children. Production callers never supply this combination.

CP207 directly writes record `ZTM`, `WPrevZoneTSTemp`, `AirPowerCap`,
optional `SumIntGainExceptPeople`, `TempDepCoef`, `TempIndCoef`, `ZT`,
`MAT`, `airHumRat`, and `airRelHum`. Its direct shared writes are applicable
system-node `Temp`, Zone-only `TempTstatAir`, and Zone-shared
`LoadCorrectionFactor`. RoomAir, sum assembly, hybrid inverse, sensible
demand, humidity correction, and psychrometric diagnostics/caches are
dependency effects.

The positive-Zone debug assertion is the only local validation. CP207 has no
upper-bound, self/identity, Space-membership, arena, node, history-shape,
solution-enum, RoomAir-topology, multiplier, volume, pressure, timestep,
denominator, finite, or consistency validation. It emits no explicit
diagnostic and owns no latch, status, catch, cleanup, transaction, or rollback.

A non-return can preserve history selection before capacitance; capacitance
before RoomAir or sum work; coefficients and `ZT` before node/thermostat/load
correction; node or correction-factor prefixes before hybrid work; hybrid
measured state before `MAT`; and `MAT` plus sensible demand before humidity.
Humidity-child prefixes can remain before `airHumRat`, and a final
psychrometric failure can leave new humidity with stale relative humidity. A
late failure prevents CP207 return, so CP206 cannot fold an otherwise completed
temperature change.

Same-state retry reselects history but recomputes from already-mutated `MAT`,
humidity, nodes, RoomAir, shared correction factor, demand, and hybrid measured
history. Hybrid history advancement and child reports can repeat. Clean replay
requires coordinated reconstruction of predictor/corrector records, Zone/Space
heat balance and topology, nodes, HeatBalFanSys arrays, RoomAir, AFN, duct
loss, HybridModel, ZoneEnergyDemand, HVAC/environment, psychrometric
diagnostics, and all child owners; predictor/corrector `clear_state` alone is
insufficient.

#### C++ test and full-simulation reach

No C++ test calls CP207 directly. The focused
`HybridModel_correctZoneAirTempsTest` calls CP206 five times and therefore
reaches CP207 Zone five times with history true, `spaceNum = 0`, one positive
controlled node, default ThirdOrder, fully mixed RoomAir, no AFN distribution
or duct loss, and no warmup or sizing. It reaches one internal-mass, two
infiltration, and two people inverse-temperature modes. Its five assertions
inspect only hybrid child results, not `ZT`, `MAT`, node, sensible load,
humidity/RH, returned change, or failure behavior. `FlagHybridModel_PC` stays
false.

Helper-only tests make ten direct `correctHumRat` calls with ten assertions and
five direct `calcZoneOrSpaceSums` calls with twelve assertions; four of the
latter use the correction flag. They do not call CP207. No focused test
directly calls `reportZoneAirSystemSensibleLoads`,
`InverseModelTemperature`, `RoomAir::ManageAirModel`, or the except-People
aggregate.

Of 57 active full-simulation call expressions, one expected EMS fatal stops
before CP207 and one zero-Zone weather fixture has no record call. The other 55
configurations reach CP207. A static one-pass census contains 81 Zone records:
55 controlled and 26 uncontrolled. Forty-nine configurations select
ThirdOrder for 74 records; six select Analytical for seven; none selects
Euler.

Exactly one configuration enables simulation Space heat balance.
`HeatBalanceAirManager_GetMixingAndCrossMixing` reaches two uncontrolled
Analytical Zones and three uncontrolled Analytical Spaces; its assertions
cover topology and mixing, not CP207 state. No controlled Space is exercised.
One AirflowNetwork distribution configuration reaches the optional AFN
sensible addition for three ThirdOrder Zones, and one DuctLoss configuration
reaches the duct addition for three ThirdOrder Zones; their assertions do not
test CP207 coefficients.

No full simulation declares a RoomAir model or HybridModel:Zone. Thus the
nonmixing node branches, RoomAir AFN override, nonmixed delta, and hybrid
temperature path have no full-simulation oracle. All five focused calls and
every statically known initial full-simulation correction use Zone history.
Fine-step system-history execution is source-valid after adaptive shortening
but has no focused assertion. The only downstream corrected-air assertions are
later thermal-comfort averages. No test isolates either solver, the exact-zero
Analytical arm, unknown enum, node/thermostat/correction-factor writes, sensible
report, RH, returned delta, partial failure, retry, or reset.

#### Rust boundary

Rust has no `correct_air_temp` compatibility definition, named wrapper, or
direct CP207 test. Its all-Zone
`correct_zone_air_temperatures_from_current_surfaces` has four non-test calls
and no direct test. The adaptive-only single-Zone corrector has one non-test
call and no direct test. The live Correct closure performs one complete
all-Zone temperature pass, then a separate all-Zone humidity pass, then
project-specific adaptive or history work. Other surface-iteration calls run
the temperature helper outside that named closure. This does not preserve
CP207's per-record temperature, demand, humidity, and returned-delta
transaction.

The bounded coefficient helper has five non-test calls and one direct fixture.
The public ThirdOrder helper has no production call and two calls in one direct
fixture; the coefficient-form ThirdOrder solver has four non-test calls and no
direct test. The Analytical helper has three non-test calls and two calls in
one direct fixture. There is no Euler helper.

Those helpers preserve the basic finite Zone-only coefficient algebra,
ThirdOrder history numerator, and most normal Analytical algebra. Live callers
merge non-system and system `MCp` terms without a controlled-node decision;
those four runtime flow fields have no production writer, and surface sum
assembly hard-codes `SumHATref` to zero. Rust omits non-air response, lagged
system loads, AFN sensible exchange, duct loss, and exact
`calcZoneOrSpaceSums`.

Rust capacity uses guarded Zone volume, weather pressure, and current
temperature/humidity, omits `ZoneVolCapMultpSens`, preserves stale capacity
when weather or psychrometrics reject inputs, and has no Space volume. It sets
AirPowerCap to zero for nonpositive capacity or timestep, returns the prior
temperature for a ThirdOrder denominator within epsilon, treats
`abs(TempDepCoef) <= epsilon` as the Analytical zero branch, and returns the
prior temperature for nonpositive capacity or timestep. The source uses raw
division and exact coefficient equality. Rust also has no distinct `T1` owner,
Euler branch, or unknown-enum behavior.

Rust state owns Zones and Surfaces but no Space heat-balance records, selected
system nodes, distinct `ZT`/`MAT`/`T1`, RoomAir, thermostat temperature, load
correction factor, sensible demand, hybrid inverse, AFN/duct input, relative
humidity, or CP207 return. `use_zone_timestep_history` is written and traced
but does not select correction history. The separate humidity pass is a
validated, saturation-clamped/defaulting history approximation rather than the
source `correctHumRat` transaction. Separate IdealLoads nodes and demand state
are not passed here.

Existing coefficient and solver tests and bounded official
`1ZoneUncontrolled` MAT evidence belong only to adjacent Zone formulas and
case-specific results. Authored Space, sizing, EMS, Python, and AFN execution
remain run-blocked. CP207 adds no Rust target, code, mapped state, test,
support, capability, output implementation, comparator, manifest, numerical,
performance, or conformance promotion.

CP207 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_space_heat_balance_correct_air_temp`
immediately after `routine.correct_zone_air_temps`. The heat-balance project
contract adds `zone_space_heat_balance_correct_air_temp` after
`correct_zone_air_temps` and before `update_final_surface_heat_balance`. The
algorithm remains a `scaffold` with `claim_level = none`. The inventory becomes
32 algorithms and 215 routines, split 58 `state_mapped` plus 157
`source_mapped`, with 92 required; the heat-balance project list becomes 61.

### CP208 `PushZoneTimestepHistories` source map

`PushZoneTimestepHistories(EnergyPlusData &state)` is declared at
`ZoneTempPredictorCorrector.hh` line 293 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4167-4185. The only built-in direct call
expression is CP195 `ManageZoneAirUpdates` line 236. That switch arm is reached
only after the dispatcher's optional input acquisition and unconditional
setpoint initialization prefix. It ignores `ShortenTimeStepSys`,
`UseZoneTimeStepHistory`, and `PriorTimeStep` and leaves caller-owned
`ZoneTempChange` unchanged.

#### Production placement

The sole built-in selector request is `HVACManager::ManageHVAC` lines 579-584.
It runs once after all selected system-timestep iterations and after the
complete allocated Zone and Space arenas have copied `ZTAV` and
`airHumRatAvg` into their comfort-average fields. It runs before the separate
contaminant-history push, the last-system-step-count commit, and demand-manager
update. Adaptive fine steps can push system-timestep histories after each
correction, but CP208 still runs only once after the loop.

A `stopSimulation` break leaves that loop without a post-loop CP208 guard.
The ordinary `CalcHeatBalanceAir` path invokes the built-in HVAC manager, while
an active external HVAC callback bypasses it and receives only the supplied
setpoint initializer; external code must arrange any equivalent history work.
The immediately following contaminant selector uses a different namespace and
is not a second CP208 call.

#### Zone and Space dispatch

CP208 first aliases `state.dataZoneTempPredictorCorrector`, then visits
`zoneNum = 1..NumOfZones` in ascending identity order. For each reached Zone it
performs this exact sequence:

1. Index the Zone heat-balance record and call the following CP209
   `pushZoneTimestepHistory` child with default zero Space identity.
2. Read the current aggregate `doSpaceHeatBalance` flag.
3. When true, visit that Zone's stored `spaceIndexes` in container order.
4. Index each Space heat-balance record and call CP209 with the parent Zone and
   stored Space identities before advancing to the next Space or Zone.

The wrapper does not scan `numSpaces` independently. Missing membership is
skipped; duplicate or cross-Zone membership is replayed exactly as stored.
There is no sorting, deduplication, parent-membership verification, count,
bound, arena-shape, allocation, or identity validation. Target-record indexing
precedes the child's positive-Zone assertion. A nonpositive `NumOfZones` is a
silent no-op.

`doSpaceHeatBalance` is the phase aggregate: sizing can assign it from
`doSpaceHeatBalanceSizing`, and normal simulation later assigns it from
`doSpaceHeatBalanceSimulation`. CP208 does not inspect `DoingSizing`, either
constituent flag, warmup, kickoff, stop state, timestep shortening, history
selection, or solution algorithm. The HVAC parent accumulates averages for
every stored Space regardless of the aggregate flag, but CP208 advances Space
records only while that flag is true.

CP208 directly reads the predictor/corrector arena, Zone count, aggregate flag,
Zone memberships, and target identities. It has no direct record, history, or
numerical write. The four-slot temperature and humidity shifts, current and
temporary record commits, psychrometric relative humidity, non-ThirdOrder
state, and optional RoomAir and AFN histories all belong to CP209 and remain
uncredited here.

#### Failure, retry, and reset

CP208 is void and exposes no completion count. It has no local assertion,
diagnostic, status, latch, catch, cleanup, transaction, or rollback. A Zone
child non-return retains every completed earlier Zone and the failing child's
ordered prefix while suppressing that Zone's Spaces and all later Zones. A
Space child non-return retains its parent Zone and earlier Spaces while
suppressing later traversal. A later contaminant-history failure cannot undo a
completed CP208 pass.

Same-state retry restarts at Zone one and invokes already completed children
again, destructively shifting their histories a second time. Clean replay
requires coordinated restoration of the visited Zone and Space heat-balance
records, child-owned RoomAir and AFN state, topology and aggregate flags, and
any common-dispatch initialization or diagnostic owners. Resetting only the
predictor/corrector arena does not restore external RoomAir owners.

#### C++ test and full-simulation reach

The C++ unit tree contains zero direct CP208 calls, zero direct record-child
calls, zero test-side `ManageZoneAirUpdates` calls, and zero direct
`ManageHVAC` calls. Of 57 active `ManageSimulation` call expressions, one
expected EMS fatal stops at `BeginTimestepBeforePredictor` before CP208. The
other 56 reach the end-of-HVAC wrapper. `WeatherManager_SetRainFlag` has zero
Zones and therefore calls CP208 without a child; the remaining 55
configurations collectively span a static one-pass census of 81 Zone record
identities.

Exactly eight configurations enable the aggregate Space branch. One
`HeatBalanceAirManager` simulation configuration contributes two Zones and
three active simulation Spaces. Seven `SizingManager` configurations each
contribute one Zone and three sizing Spaces. The resulting static Space-child
identity census is 24; a related sizing configuration stores three Spaces with
both Space-HB flags false and skips them. These are configuration identities,
not total calls across timesteps.

No assertion names `XMAT`, `WPrevZoneTS`, or directly checks the wrapper or
record shift. Full-simulation assertions concern downstream mixing, sizing,
weather, or other results. They do not isolate Zone-before-Space order,
aggregate-flag rereads, duplicate membership, end-of-HVAC timing, partial
failure, same-state retry, or coordinated reset.

Tracked conformance inputs declare no explicit `Space`, `RoomAirModelType`, or
`ZoneAirHeatBalanceAlgorithm` object. The official one-Zone family compares
downstream hourly mean air temperature and humidity ratio, but its
varied-timestep lane remains planned-not-claimed with no blocking gate. That
evidence cannot isolate one CP208 push or any Space or RoomAir branch.

#### Rust boundary

Rust defines the selector name plus
`push_zone_timestep_histories_source_order_path` and
`push_zone_timestep_histories_compat`, but both wrappers are generic identity
closures. Production has one named compat call and zero
`ManageZoneAirUpdates` calls carrying the PushZone selector; the Rust
dispatcher ignores its selector argument.

The live compat closure occurs at the start of the Predict path rather than
after the HVAC system loop. It visits every Rust Zone, shifts only three
temperature and humidity slots, then continues into predictor calculations
inside the same closure. Its new first slot chooses the stored Zone-timestep
average only under the project-specific adaptive flag and otherwise chooses
the current value. Source CP209 instead owns an unconditional four-slot
average-based commit plus additional current, temporary, psychrometric,
non-ThirdOrder, RoomAir, and AFN state.

Rust heat-balance state has no Space heat-balance arena, Zone `spaceIndexes`,
aggregate `doSpaceHeatBalance` traversal, fourth Zone history slot, or exact
record-child boundary. The named source-order wrapper has one generic test
that checks only the wrapper call label and order in a larger vector. The compat
wrapper has no direct test. One timestep integration assertion sees an all-20 C history before
and after the shift, so it cannot distinguish slot order and checks no humidity
history. A system-timestep synchronization test exercises a different path.
Separate IdealLoads humidistat history is single-Zone moisture state and is not
CP208 parity.

CP208 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.push_zone_timestep_histories`
immediately after
`routine.zone_space_heat_balance_correct_air_temp`. The heat-balance project
contract adds `push_zone_timestep_histories` after
`zone_space_heat_balance_correct_air_temp` and before
`update_final_surface_heat_balance`. The algorithm remains a `scaffold` with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 216 routines, split 58 `state_mapped` plus 158
`source_mapped`, with 93 required; the heat-balance project list becomes 62.

### CP209 `ZoneSpaceHeatBalanceData::pushZoneTimestepHistory` source map

`ZoneSpaceHeatBalanceData::pushZoneTimestepHistory(
EnergyPlusData &state, int zoneNum, int spaceNum = 0)` is declared at
`ZoneTempPredictorCorrector.hh` line 245 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4187-4275. Its only direct production
expressions are CP208 line 4178 for the Zone record with default zero Space
identity and line 4181 for a stored Space record with positive identity.
CP208 supplies Zone-first order and the aggregate Space-HB gate; CP209 itself
does not inspect that flag.

#### Common record commit

CP209 creates the diagnostic label `pushTimestepHistories`, debug-asserts only
`zoneNum > 0`, and aliases the parent Zone's `AirModel` before any mutation.
Every Space call therefore also requires a valid parent RoomAir entry. It then
performs this exact record order:

1. For `iHistory = 3, 2, 1`, write
   `XMAT[iHistory] = XMAT[iHistory - 1]` and then
   `WPrevZoneTS[iHistory] = WPrevZoneTS[iHistory - 1]`.
2. Write `XMAT[0] = ZTAV` and `XMPT = ZT`.
3. Write `WPrevZoneTS[0] = airHumRatAvg`.
4. Write both `airHumRat` and `WTimeMinusP` from `airHumRatTemp`.
5. Compute and write
   `airRelHum = 100 * PsyRhFnTdbWPb(state, ZT, airHumRat,
   OutBaroPress, pushTimestepHistories)`.

The descending loop safely preserves three old values and discards the old
slot three. Temperature and humidity writes are interleaved at every shifted
slot. The newest four-slot history values use Zone-timestep averages, while
`XMPT` uses the current system-step `ZT` and relative humidity uses that `ZT`
plus the newly committed temporary humidity. `MAT`, `ZT`, `ZTAV`,
`airHumRatAvg`, `airHumRatTemp`, `ZTM`, `DSXMAT`, `DSWPrevZoneTS`, `T1`,
and `W1` remain unchanged.

The psychrometric dependency floors only its local working humidity ratio to
`1.0e-5`; it does not repair the already stored `airHumRat`. A
comparison-detected result outside [0,1] can emit optional diagnostics and is
returned clamped to [0.01,1.0], while NaN comparisons can fall through and
propagate. CP209 applies the factor 100 but no caller-side finite, pressure, or
range guard. A negative finite `airHumRatTemp` can therefore remain stored
while RH is evaluated from the dependency's positive local floor. Saturation
pressure cache and diagnostic effects belong to the dependency.

#### Exact-Zone RoomAir histories

Only exact `spaceNum == 0` enters RoomAir work. A positive Space and any
malformed negative identity both skip it.

For `DispVent3Node`, `UFADInt`, or `UFADExt`, CP209 processes Floor, occupied,
then mixed levels. At each level it shifts the four-slot `XMAT*` history,
inserts current `ZTFloor`, `ZTOC`, or `ZTMX` at slot zero, and copies the same
current value to `MATFloor`, `MATOC`, or `MATMX`.

For `AirflowNetwork`, CP209 visits the stored node container in order. Each
node first shifts four `AirTempX` slots and inserts `AirTemp`, then shifts four
`HumRatX` slots and inserts `HumRat`. The stratified and AFN branches are
mutually exclusive enum tests.

There is no `SimAirModel`, global nonmixing, node-count, or activity guard.
Mixing, UserDefined, one-node displacement, CrossVent, Invalid, `Num`, and
other unmatched RoomAir enum values perform no shared RoomAir write.

#### Non-ThirdOrder histories

Every solution enum other than exact `ThirdOrder` enters the final branch.
This includes Analytical, Euler, Invalid, `Num`, and arbitrary unmatched
casts. Every Zone or Space record receives, in order:

1. `TM2 =` old `TMX`, then `TMX = ZTAV`.
2. `WM2 =` old `WMX`, then `WMX = airHumRatAvg`.

For exact Zone identity, the three stratified RoomAir enums then advance
Floor, occupied, and mixed `ZoneM2*` from the old `ZoneMX*` and set each
`ZoneMX*` from its current `ZT*`. AirflowNetwork instead advances each node's
`AirTempT2/AirTempTX` pair from `AirTemp` and then its
`HumRatT2/HumRatTX` pair from `HumRat`.

Exact ThirdOrder skips this entire final stage, including record scalar and
applicable RoomAir two-slot histories. It still receives the common record
shifts, humidity/RH commit, and applicable exact-Zone stratified or AFN
four-slot histories.

#### Validation, failure, retry, and reset

The positive-Zone assertion can compile out. CP209 has no Zone upper-bound,
Space sign or upper-bound, record-kind, parent membership, arena-shape,
allocation, solution-enum, RoomAir-topology, pressure, or finite validation.
Because the caller selects `this` before entry and the method uses `spaceNum`
only as an exact-zero classifier, a malformed direct call can apply Zone
RoomAir semantics to a Space record or suppress them for a Zone record. CP209
has no local diagnostic beyond its psychrometric dependency, return status,
latch, catch, cleanup, transaction, or rollback.

An assertion failure or abnormal non-return while indexing the parent AirModel
occurs before record mutation. If malformed dependency state causes an
abnormal non-return during the psychrometric call, every common shift and
commit through `WTimeMinusP` remains while old `airRelHum` and all later
RoomAir and solution-state work remain untouched. A later abnormal RoomAir or
AFN-node indexing non-return retains the complete record prefix and any
earlier fields or nodes. CP208 then cannot visit the remaining Spaces or
Zones.

Same-state retry destructively shifts every completed four-slot history again.
Outside ThirdOrder it also advances already changed `TM2/TMX`, `WM2/WMX`,
RoomAir M2/MX, and AFN T2/TX state again. The later
`revertZoneTimestepHistory` is not a full inverse: it does not restore
`XMPT`, current humidity, relative humidity, non-ThirdOrder scalars, or every
external RoomAir/AFN field. Clean replay requires coordinated record,
RoomAir/AFN, psychrometric cache/diagnostic, topology, environment, and caller
reset.

#### C++ test and full-simulation reach

No C++ test directly calls CP209 or CP208, and no assertion names any CP209
destination. Of 57 active `ManageSimulation` call expressions, one expected
EMS fatal stops before CP209 and one zero-Zone case reaches CP208 without a
child. The other 55 configurations collectively span a static one-pass census
of 81 Zone plus 24 Space records.

The exact solution split is:

| Solution path | Zone identities | Space identities | Total |
|---|---:|---:|---:|
| ThirdOrder | 74 | 21 | 95 |
| Analytical | 7 | 3 | 10 |
| Euler | 0 | 0 | 0 |

All 105 record identities reach the common four-slot, current humidity, and RH
path. Only the ten Analytical identities reach the non-ThirdOrder scalar
branch. All 81 Zones use Mixing RoomAir, and all 24 Spaces skip shared RoomAir
by identity, so displacement, UFAD, and RoomAir AFN histories have zero active
full-simulation reach.

The only later full-simulation corrected-air assertions inspect comfort
averages and PMV/PPD. The comfort averages are copied before CP208 and none of
those assertions isolates CP209. Tracked conformance inputs declare no
explicit Space, RoomAir model, or Zone-air solution algorithm. The official
one-Zone hourly temperature and humidity outputs can be downstream-sensitive
to accumulated history, but the varied-timestep lane remains
planned-not-claimed and none proves one record commit, RoomAir branch,
psychrometric edge, partial failure, retry, or reset.

#### Rust boundary

Rust has no singular `push_zone_timestep_history` definition, call, or direct
test. It has only CP208's plural identity wrappers. Their one live compat
closure starts before predictor work and contains one all-Zone loop that shifts
three-slot `previous_mean_air_temperatures_c` and
`previous_air_humidity_ratios`. The inserted value is the saved average only
under a project-specific adaptive flag and otherwise the current value.

Rust therefore drops the source old slot two instead of preserving it in slot
three and does not unconditionally insert the Zone-timestep averages. It has no
Space heat-balance arena, record call boundary, fourth slot, `XMPT`,
`airHumRatTemp`, `WTimeMinusP`, stored relative humidity,
`TM2/TMX/WM2/WMX`, RoomAir stratification, or AFN node histories. Rust exposes
Analytical and ThirdOrder predictor modes but no Euler mode, and those modes do
not create CP209 solution-specific history state.

The plural source-order wrapper's one generic test checks only its call label
and order in a larger vector. One timestep integration assertion starts and
ends with uniform `[20.0; 3]` temperature history, so it cannot identify the
shift and checks no humidity. A nonuniform adaptive test exercises system-step
history instead. Rust's psychrometric RH helper has pure formula tests but no
production CP209 record commit. Separate IdealLoads humidistat history is an
independent three-slot single-Zone moisture transaction and is not CP209
parity.

CP209 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_space_heat_balance_push_zone_timestep_history`
immediately after `routine.push_zone_timestep_histories`. The heat-balance
project contract adds `zone_space_heat_balance_push_zone_timestep_history`
after `push_zone_timestep_histories` and before
`update_final_surface_heat_balance`. The algorithm remains a `scaffold` with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 217 routines, split 58 `state_mapped` plus 159
`source_mapped`, with 94 required; the heat-balance project list becomes 63.

### CP210 `PushSystemTimestepHistories` source map

`PushSystemTimestepHistories(EnergyPlusData &state)` is declared at
`ZoneTempPredictorCorrector.hh` line 295 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4277-4295. Its only built-in direct call
is the CP195 `ManageZoneAirUpdates` selector arm at lines 238-240. The
dispatcher first performs optional setpoint-input acquisition and unconditional
setpoint initialization. This arm ignores `ShortenTimeStepSys`,
`UseZoneTimeStepHistory`, and `PriorTimeStep` and leaves caller-owned
`ZoneTempChange` unchanged.

#### Production placement and gate

The sole built-in selector request is `HVACManager::ManageHVAC` lines 388-393.
At entry the parent resets `TimeStepSys = TimeStepZone`,
`UseZoneTimeStepHistory = true`, and `NumOfSysTimeSteps = 1`. After an initial
Predict/HVAC/Correct pass, it selects adaptive downstepping only when
`ZoneTempChange > MaxZoneTempDiff && !KickOffSimulation`. Even then, the
per-iteration strict `TimeStepSys < TimeStepZone` comparison is the actual
CP210 gate.

For every entered fine-system-step branch, the parent orders:

1. fine-step Predict and HVAC simulation;
2. `ShortenTimeStepSys = false`;
3. thermal Correct, then optional contaminant Correct;
4. CP210 thermal system-history dispatch;
5. the separate contaminant system-history dispatch;
6. `PreviousTimeStep = TimeStepSys`;
7. current-step Zone and all-Space average accumulation.

Thus CP210 runs once after each selected global fine-step correction and before
the current result contributes to `ZTAV` and `airHumRatAvg`. Equal
system/Zone-step and kickoff paths do not request it. At every built-in request,
`ShortenTimeStepSys` is already false and the adaptive branch left
`UseZoneTimeStepHistory` false, but CP210 does not inspect either value.

A `stopSimulation` value present at loop entry breaks before the request. If it
becomes true during a fine-step body, there is no second check before CP210, so
the current push still runs if preceding work returns. An external HVAC
callback bypasses this built-in request. The following contaminant selector is
a different namespace routine, not a second CP210 call.

#### Zone and Space dispatch

CP210 aliases `state.dataZoneTempPredictorCorrector` and visits
`zoneNum = 1..NumOfZones` in ascending order. For every reached Zone it:

1. indexes the Zone heat-balance record and calls the following CP211
   `pushSystemTimestepHistory` child with default zero Space identity;
2. reads the current aggregate `doSpaceHeatBalance` flag;
3. when true, visits that Zone's stored `spaceIndexes` in container order;
4. indexes each Space heat-balance record and calls CP211 with the parent Zone
   and stored Space identities before advancing to the next Space or Zone.

It does not independently scan all Spaces, sort, deduplicate, or validate
membership, counts, bounds, arena shape, allocation, or identity consistency.
Missing membership is skipped; duplicate or cross-Zone membership is replayed
as stored. Target indexing precedes the child's positive-Zone assertion, and a
nonpositive `NumOfZones` is a silent no-op. The aggregate flag is reread after
each Zone child; CP210 does not inspect its sizing/simulation constituent flags.

The wrapper directly reads only traversal state and selected identities. It
writes no record, history, or numerical state. CP211 owns every
`DSXMAT`/`DSWPrevZoneTS` shift, current-value insertion, optional RoomAir/AFN
downstep history, and non-ThirdOrder scalar mutation and remains uncredited
here.

#### Failure, retry, and reset

CP210 is void and exposes no completion count. It has no local assertion,
diagnostic, status, latch, catch, cleanup, transaction, or rollback. A Zone
child abnormal non-return retains earlier Zones and the failing child's prefix
while suppressing that Zone's Spaces and all later Zones. A Space child
abnormal non-return retains its Zone and earlier Spaces while suppressing later
traversal.

A CP210 abnormal non-return also prevents the parent from performing the
following contaminant push, `PreviousTimeStep` commit, and current-step average
accumulation. If CP210 completes but the contaminant push does not return, its
history effects remain while the time and average commits stay pending.

Same-state retry starts again at Zone one and destructively advances every
already completed system history. The later Zone-timestep revert routine does
not restore CP211 downstepped histories. Clean replay therefore requires
coordinated Zone/Space record, RoomAir/AFN, topology, aggregate-flag,
HVAC-clock, dispatcher, dependency, and diagnostic restoration.

#### C++ test and oracle boundary

The C++ unit tree contains zero direct CP210 calls, zero direct CP211 calls,
zero uses of the PushSystem selector, and zero test-side `ManageZoneAirUpdates`
or `ManageHVAC` calls. No assertion names a CP211 destination.

Of 57 active `ManageSimulation` call expressions, one expected EMS fatal stops
before CP210 and one has zero Zones. The other 55 configurations provide only a
static potential census of 81 Zone identities and 24 eligible Spaces across
eight configurations. Actual CP210 entry is conservatively bounded at zero to
55 configurations because the numerical adaptive decision and exact timestep
comparison are not observed. No test asserts `NumOfSysTimeSteps`, the strict
comparison, `UseZoneTimeStepHistory = false`, or a push count.

If one pass were selected for every potential record, the corpus split would be
95 ThirdOrder and ten Analytical identities, with no Euler. All 81 Zones use
Mixing RoomAir and positive Space identities skip shared RoomAir, so special
RoomAir/AFN child branches have zero potential in this corpus. These are
conditional topology counts, not execution evidence.

The tracked one-Zone family has seven unique inputs, no `ConvergenceLimits`,
Space, RoomAir, AFN, or explicit solution algorithm, and only hourly
downstream Zone/surface outputs. Six inputs use four Zone timesteps per hour;
the varied fixture uses six, but remains planned-not-claimed with an empty gate
script. Changing the Zone timestep does not force or prove the adaptive system
gate. IdealLoads evidence explicitly excludes adaptive system-timestep
behavior. No test or oracle covers a partial prefix, retry, or reset.

#### Rust boundary

Rust defines the PushSystem selector name plus one source-order wrapper and one
compat wrapper. The source-order wrapper has no production call outside the
compat alias. Production has exactly three lexical compat sites: adaptive count
one, adaptive count greater than one, and adaptive feature disabled. The
feature-disabled site is exclusive with the adaptive helper. With adaptive
correction enabled, each Zone independently selects the count-one or
count-greater-than-one site, so both adaptive sites can execute in one
multi-Zone timestep; exactly one site executes per Zone. Production never calls
`manage_zone_air_updates_compat` with the PushSystem selector, and that
dispatcher ignores its selector argument.

The count-one and feature-disabled paths rebuild three system-history slots as
`[current, zone_history[0], zone_history[1]]` and set the stored count to one.
They do not shift prior system histories, and the feature-disabled path runs
even for a full Zone step where source CP210 is absent.

For an adaptive count greater than one, Rust chooses a count independently for
each Zone, performs every local correction and three-slot local shift for that
Zone, then invokes the named wrapper once to commit the final histories,
averages, count, and report fields. Source instead selects one global count and
runs CP210 after every global fine step, completing all eligible Zones and
Spaces before the next fine step. The named Rust boundary therefore has
different gating, cadence, cross-Zone order, and ownership.

Rust heat-balance state has no Space record arena, Zone membership traversal,
aggregate flag, fourth system-history slot, singular child boundary,
`airHumRatTemp`, `TM2/TMX/WM2/WMX`, RoomAir stratification, or AFN node
histories. The source-order wrapper has one direct test that checks only its
call label and order; the compat wrapper and PushSystem selector dispatch have
zero direct tests. One focused count-one test reaches the compat site indirectly
through the adaptive helper and proves Rust's nonuniform three-slot rebuild and
average writes, not CP210 global traversal, source child shifts, or adaptive
cadence; there is no explicit adaptive-count-greater-than-one commit test.

CP210 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.push_system_timestep_histories`
immediately after
`routine.zone_space_heat_balance_push_zone_timestep_history`. The heat-balance
project contract adds `push_system_timestep_histories` after
`zone_space_heat_balance_push_zone_timestep_history` and before
`update_final_surface_heat_balance`. The algorithm remains a `scaffold` with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 218 routines, split 58 `state_mapped` plus 160
`source_mapped`, with 95 required; the heat-balance project list becomes 64.

### CP211 `ZoneSpaceHeatBalanceData::pushSystemTimestepHistory` source map

`ZoneSpaceHeatBalanceData::pushSystemTimestepHistory(EnergyPlusData &state,
int zoneNum, int spaceNum = 0)` is declared at
`ZoneTempPredictorCorrector.hh` line 247 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4297-4370. Its only production call
expressions are the CP210 Zone child at line 4288 with default zero Space
identity and the stored-Space child at line 4291 with both identities. CP210
owns Zone-first traversal, the current aggregate `doSpaceHeatBalance` gate,
stored membership order, and dynamic multiplicity; CP211 does not inspect that
flag or discover records independently.

#### Common record history transaction

After a debug-only `assert(zoneNum > 0)`, CP211 loops
`iHistory = 3, 2, 1`. At each index it first copies
`DSXMAT[iHistory - 1]` to `DSXMAT[iHistory]`, then copies
`DSWPrevZoneTS[iHistory - 1]` to `DSWPrevZoneTS[iHistory]`. It next writes
`DSXMAT[0] = MAT` and `DSWPrevZoneTS[0] = airHumRat`. The resulting arrays
are `[current, old0, old1, old2]` and the old slot three is discarded.

This common prefix runs for every reached Zone and Space and every solution
algorithm. It copies finite, infinite, or NaN values verbatim and performs no
psychrometric call or caller-side finite/range repair. It does not modify
`MAT`, `airHumRat`, `ZT`, `XMAT`, `WPrevZoneTS`, Zone-timestep averages,
relative humidity, or diagnostic state.

#### Exact-Zone RoomAir and non-ThirdOrder stages

The first RoomAir stage requires both exact `spaceNum == 0` and the global
`anyNonMixingRoomAirModel` flag. A positive Space or malformed negative Space
identity short-circuits this entire stage.

When `IsZoneDispVent3Node(zoneNum) || IsZoneUFAD(zoneNum)` is true, CP211
processes Floor, occupied, then mixed levels. Each four-slot
`DSXMATFloor`, `DSXMATOC`, or `DSXMATMX` history shifts downward and slot
zero receives current `MATFloor`, `MATOC`, or `MATMX`. Independently, when
the exact per-Zone AirModel enum is `AirflowNetwork`, CP211 visits every
stored node in container order, shifts its four-slot `AirTempDSX` and inserts
`AirTemp`, then shifts `HumRatDSX` and inserts `HumRat`. The stratified
predicates and AFN enum are separate tests, so malformed inconsistent state can
run both. There is no `SimAirModel` check, and a false global nonmixing flag
suppresses both even if the per-Zone state says otherwise.

Every `ZoneAirSolutionAlgo` value other than exact `ThirdOrder` enters the
final stage, including `Analytical`, `Euler`, source `Invalid`/`Num`, or an
unmatched stored value. Every reached Zone or Space record receives these
ordered writes:

1. `TM2 =` old `TMX`;
2. `TMX = MAT`;
3. `WM2 =` old `WMX`;
4. `WMX = airHumRatTemp`.

Thus the common humidity history inserts `airHumRat` while the non-ThirdOrder
scalar history deliberately reads the distinct temporary
`airHumRatTemp`.

For exact Zone identity, exact `DispVent3Node`, `UFADInt`, or `UFADExt`
then advances Floor, occupied, and mixed `ZoneM2*` from old `ZoneMX*` and
`ZoneMX*` from current `ZT*`. Exact `AirflowNetwork` advances each indexed
node's `AirTempT2/AirTempTX` and then `HumRatT2/HumRatTX` pairs. This final
AFN loop trusts `NumOfAirNodes` and indexes
`Node(1..NumOfAirNodes)`, unlike the earlier range-based pass over the stored
container; a count/container mismatch can skip stored nodes or index beyond
them. It does not test the global nonmixing flag.

Exact ThirdOrder skips this entire final stage but still receives the common
record shifts and any applicable first RoomAir/AFN four-slot work. A Space
receives common and non-ThirdOrder record writes but skips both shared RoomAir
blocks.

#### Validation, failure, retry, and reset

The positive-Zone assertion can compile out. CP211 has no Zone upper-bound,
Space sign or upper-bound, membership, record-kind, arena-shape, allocation,
RoomAir topology, AFN count/container consistency, solution-enum, or finite
validation. The caller selects `this` before entry and `spaceNum` is only an
exact-zero classifier, so malformed direct calls can apply Zone RoomAir
semantics to a Space record, suppress them for a Zone record, or run common and
non-ThirdOrder writes with a negative Space identity.

CP211 is void and has no local diagnostic, status, latch, catch, cleanup,
transaction, or rollback. Assertion failure precedes mutation. An abnormal
non-return in the first RoomAir stage retains the complete common record prefix
and can retain partial Floor/occupied/mixed or AFN-node histories. A later
non-ThirdOrder RoomAir/AFN failure occurs after all four record scalar writes.
CP210 then cannot reach later Spaces or Zones, and the parent cannot reach the
following contaminant push, `PreviousTimeStep` commit, or current-step average
accumulation.

Same-state retry advances completed `DSXMAT`, `DSWPrevZoneTS`, and applicable
RoomAir/AFN four-slot histories again, discarding another oldest value.
Non-ThirdOrder retry also advances already-mutated `TM2/TMX`, `WM2/WMX`, and
applicable RoomAir/AFN two-slot state again. The following CP212 Zone-timestep
revert changes Zone-timestep histories, not CP211 downstepped histories or
scalar/shared state. Clean replay therefore requires coordinated Zone/Space
record, RoomAir/AFN, topology/count, solution, HVAC-clock, caller, dependency,
and diagnostic restoration.

#### C++ test and oracle boundary

The C++ unit tree contains zero direct CP211 calls, zero direct CP210 calls,
zero uses of the PushSystem selector, and zero test-side
`ManageZoneAirUpdates` or `ManageHVAC` calls. No assertion names a CP211
destination. No failure, partial-prefix, retry, or reset test exists.

Of 57 active `ManageSimulation` call expressions, one expected EMS fatal stops
before CP211 and one has zero Zones. The other 55 configurations bound actual
CP211-entering configurations at zero to 55 because no test observes the
adaptive gate. If one CP210 pass were selected for each, their conditional
one-pass potential is 105 record identities:

- 74 Zone plus 21 Space records use ThirdOrder, for 95 total;
- seven Zone plus three Space records use Analytical, for ten total;
- zero records use Euler.

The common record path therefore has 105 potential identities and the
non-ThirdOrder scalar path has ten, but these are not observed calls. All 81
Zones are Mixing and 24 Spaces skip shared RoomAir, so the stratified and AFN
destinations have zero corpus potential.

The tracked one-Zone family has eight members over seven unique inputs. Six use
`Timestep,4` and the varied fixture uses six, but none declares
`ConvergenceLimits`, an explicit Zone-air algorithm, Space, RoomAir, or AFN.
The varied fixture is planned-not-claimed with no gate. Shared oracle outputs
are hourly downstream variables and expose no system-history slot or push
count, so they do not prove CP211.

#### Rust boundary

Rust has zero singular `push_system_timestep_history` definitions, calls, or
tests. Its only named boundary is the CP210 plural source-order/compat identity
pair. The source-order wrapper has no external production call; the compat
wrapper contains its only source-order call.

Production has three lexical compat sites. The feature-disabled site is
exclusive with the adaptive helper; with adaptive correction enabled, each
Zone independently chooses the count-one or count-greater-than-one site, so
both adaptive sites can run in one multi-Zone timestep. Exactly one compat site
executes per Zone.

The nearest record helper, `synchronize_single_system_timestep_history`, is
called only by the count-one and feature-disabled sites. It rebuilds three
temperature and humidity slots as
`[current, zone_history[0], zone_history[1]]`, discards the prior system
history, and sets a project-specific count to one. Both paths can run at a full
Zone step where source CP210 and CP211 are absent.

For adaptive count greater than one, Rust optionally initializes local
three-slot arrays by interpolation, then after each Zone-local correction
shifts them inline as `[current, local_old0, local_old1]`. Those CP211-like
assignments bypass the named wrapper. Only after all local substeps does the
plural wrapper commit the final arrays together with averages, count, and
report fields. Source instead calls CP211 after each global fine-step Correct
for all eligible Zones and Spaces before the next fine step.

Rust heat-balance state has only Zone and Surface arenas and three-slot Zone
temperature/humidity histories. It has no Space heat-balance record,
`spaceIndexes` traversal, aggregate flag, fourth slot, singular child,
`airHumRatTemp` distinction, `TM2/TMX/WM2/WMX`, stratified RoomAir, or AFN
node histories. Its boolean ThirdOrder-versus-approximate-Analytical choice is
not passed to either system-history helper, and there is no Euler or source
non-ThirdOrder CP211 history branch.

One direct source-order-wrapper test checks only a label and larger call order.
The compat wrapper and PushSystem selector dispatch have zero direct tests. One
focused count-one test indirectly reaches the helper, seeds nonuniform prior
system histories, and proves Rust discards them in favor of current plus Zone
history; it therefore proves divergence from, not parity with, the source
shift. There is no adaptive-count-greater-than-one final-commit test,
solution-branch test, Space/RoomAir/AFN test, or fourth-slot assertion. A pure
three-slot interpolation test does not compose CP211.

CP211 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_space_heat_balance_push_system_timestep_history`
immediately after `routine.push_system_timestep_histories`. The heat-balance
project contract adds
`zone_space_heat_balance_push_system_timestep_history` after
`push_system_timestep_histories` and before
`update_final_surface_heat_balance`. The algorithm remains a `scaffold` with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 219 routines, split 58 `state_mapped` plus 161
`source_mapped`, with 96 required; the heat-balance project list becomes 65.

### CP212 `RevertZoneTimestepHistories` source map

`RevertZoneTimestepHistories(EnergyPlusData &state)` is declared at
`ZoneTempPredictorCorrector.hh` line 297 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4372-4389. Its sole direct thermal
source call expression is the CP195 `ManageZoneAirUpdates` selector arm at
lines 232-234.

#### Dormant dispatcher boundary

Repository-wide source and test search finds zero
`ManageZoneAirUpdates` calls selecting
`PredictorCorrectorCtrl::RevertZoneTimestepHistories` and zero direct CP212
calls outside its dispatcher arm. The nine built-in thermal dispatcher call
sites in `HVACManager.cc` and `SimulationManager.cc` select GetSetPoints,
Predict, Correct, PushSystem, or PushZone instead. The similarly named
contaminant dispatcher arm and routine are a separate namespace and likewise
have no built-in selector request.

EnergyPlus 26.1 therefore defines CP212 as a dormant public branch without a
built-in production timing or runtime gate. An external dispatcher selection
would still execute the CP195 optional setpoint-input acquisition and
unconditional `InitZoneAirSetPoints` prefix first. The CP212 arm ignores
`ShortenTimeStepSys`, `UseZoneTimeStepHistory`, and `PriorTimeStep` and
preserves `ZoneTempChange`. A direct external CP212 call would bypass that
dispatcher prefix.

#### Zone and Space dispatch

CP212 aliases `state.dataZoneTempPredictorCorrector` and visits
`zoneNum = 1..NumOfZones` in ascending order. For every reached Zone it:

1. indexes the Zone heat-balance record and calls the following CP213
   `revertZoneTimestepHistory` child with default zero Space identity;
2. reads the current aggregate `doSpaceHeatBalance` flag;
3. when true, visits that Zone's stored `spaceIndexes` in container order;
4. indexes each Space heat-balance record and calls CP213 with the parent Zone
   and stored Space identities before advancing.

It does not independently scan the Space arena, sort, deduplicate, validate
membership, verify counts or arena dimensions, or inspect the aggregate flag's
sizing/simulation constituents. Missing membership is skipped and duplicate or
cross-Zone membership is replayed as stored. Target indexing precedes CP213's
positive-Zone assertion. A nonpositive `NumOfZones` is a silent no-op. There
is no local stop, warmup, kickoff, sizing, timestep, history-selector, or
solution-algorithm gate.

The wrapper reads traversal state and selected identities but writes no record
or numerical state. CP213 owns every `XMAT`/`WPrevZoneTS` forward copy and
optional RoomAir/AFN mutation and remains uncredited here. Neither CP212 nor
CP213 touches CP211 `DSXMAT`/`DSWPrevZoneTS`, non-ThirdOrder `TM2/TMX` or
`WM2/WMX`, or downstepped RoomAir/AFN histories. CP212 is therefore not a
system-timestep rollback.

#### Failure, retry, and reset

CP212 is void and exposes no completion count. It has no local assertion,
identity or topology validation, diagnostic, status, latch, catch, cleanup,
transaction, or rollback. A Zone-child abnormal non-return retains earlier
Zones and the failing child's ordered prefix while suppressing that Zone's
Spaces and all later Zones. A Space-child abnormal non-return retains its Zone
and earlier Spaces while suppressing later traversal.

Same-state retry restarts at Zone one and reapplies CP213's forward-copy
`revert` to already processed four-slot histories, so it is destructive and
non-idempotent. Clean replay requires coordinated Zone/Space Zone-timestep
histories, RoomAir/AFN state, topology, aggregate flag, dispatcher
initialization, caller state, dependencies, and diagnostics. CP211 system
histories require separate restoration. Because no built-in selector request
exists, these lifecycle effects are externally triggered or counterfactual in
the EnergyPlus 26.1 in-tree call graph.

#### C++ test and oracle boundary

The C++ unit tree contains zero direct CP212 calls, zero direct CP213 calls,
zero Revert selector uses, and zero test-side `ManageZoneAirUpdates` or
`ManageHVAC` calls. No assertion names `XMAT`, `WPrevZoneTS`, stratified
`XMAT*`, or AFN `AirTempX`/`HumRatX` destinations. No test covers partial
failure, retry, reset, Zone/Space order, or duplicate membership.

All 57 active `ManageSimulation` call expressions execute CP212 exactly zero
times because no in-tree caller requests the selector. The 55 completing
nonzero-Zone configurations provide only counterfactual topology if an
external request were injected: 81 Zone plus 24 eligible Space records, for
105 total. CP212 itself is solution-algorithm independent, so the ThirdOrder
versus Analytical census does not imply a branch split. All 81 Zones are
Mixing and Spaces skip shared RoomAir; special stratified and RoomAir-AFN
topology therefore has zero counterfactual corpus reach.

Tracked one-Zone oracle cases cannot execute CP212 and expose no Zone-history,
RoomAir, AFN, or revert-count variable. Their hourly downstream outputs cannot
prove a dormant revert transaction.

#### Rust boundary

Rust defines the Revert selector name and one source-order wrapper plus one
compat wrapper. The source-order wrapper has no external production call; the
compat wrapper contains its only non-test source-order call.
`manage_zone_air_updates_compat` ignores its selector argument, and production
passes only Predict and Correct selectors to it.

Rust nevertheless has one live lexical compat site in
`apply_energyplus_adaptive_system_timestep_zone_air_correction`. It runs once
for each Zone whose locally computed adaptive count is greater than one. The
wrapper always executes on that path; its closure writes only when the new
count differs from `previous_system_timestep_count`. A count match is a no-op.
Count one and the feature-disabled path bypass it.

On its sole production path, Rust has already pushed three-slot Zone histories
before this site. When the count differs, it initializes local three-slot
temperature and humidity arrays through down-interpolation; their slot zero
remains the newly pushed Zone history head. The closure then overwrites current
`mean_air_temperature_c` and `air_humidity_ratio` from those local slot-zero
values. It does not shift or remove a Zone-history sample and does not perform
CP213's slot-one-to-zero, slot-two-to-one, or slot-three-to-two forward copies.

The Rust operation is Zone-local and occurs before that Zone's local fine-step
loop. Source CP212, if externally selected, would traverse every Zone and
eligible Space globally and mutate child-owned history arrays. Rust has no
Space heat-balance arena, membership traversal, aggregate flag, four-slot
`XMAT`/`WPrevZoneTS`, stratified RoomAir state, or AFN-node histories. Its
three-slot system histories and current-value reset are different state.

One direct source-order-wrapper test checks only its label and order in a
larger vector. There is no direct compat, selector-dispatch, or state test. The
only focused adaptive-helper test chooses count one and never reaches the
Revert site; the pure interpolation test does not compose it. There is no
count-match/count-mismatch, adaptive-count-greater-than-one, push-then-revert,
nonuniform fourth-slot, Zone/Space order, RoomAir, AFN, failure, retry, or
reset evidence.

CP212 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.revert_zone_timestep_histories`
immediately after
`routine.zone_space_heat_balance_push_system_timestep_history`. The
heat-balance project contract adds `revert_zone_timestep_histories` after
`zone_space_heat_balance_push_system_timestep_history` and before
`update_final_surface_heat_balance`. The algorithm remains a `scaffold` with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 220 routines, split 58 `state_mapped` plus 162
`source_mapped`, with 97 required; the heat-balance project list becomes 66.

CP213 next maps
`ZoneSpaceHeatBalanceData::revertZoneTimestepHistory(EnergyPlusData &state,
int zoneNum, int spaceNum = 0)`, declared at
`ZoneTempPredictorCorrector.hh` line 249 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4391-4431.

### CP213 `ZoneSpaceHeatBalanceData::revertZoneTimestepHistory` source map

`ZoneSpaceHeatBalanceData::revertZoneTimestepHistory(EnergyPlusData &state,
int zoneNum, int spaceNum = 0)` is declared at
`ZoneTempPredictorCorrector.hh` line 249 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4391-4431. Its only production call
expressions are the dormant CP212 Zone child at line 4382 with default zero
Space identity and stored-Space child at line 4385 with both identities. CP212
owns Zone-first traversal, the current aggregate `doSpaceHeatBalance` gate,
stored membership order, and dynamic multiplicity; CP213 does not discover
records or inspect that flag.

Because no built-in caller requests CP212's Revert selector, EnergyPlus 26.1
has zero built-in CP213 executions. The behavior below is reachable only
through an external CP212 request or a direct external child call in the
in-tree call graph.

#### Common four-slot forward copy

After a debug-only `assert(zoneNum > 0)`, CP213 loops
`iHistory = 0, 1, 2` in ascending order. At each index it first copies
`XMAT[iHistory + 1]` to `XMAT[iHistory]`, then copies
`WPrevZoneTS[iHistory + 1]` to `WPrevZoneTS[iHistory]`. For either history,
`[old0, old1, old2, old3]` becomes
`[old1, old2, old3, old3]`: old slot zero is discarded and old slot three is
duplicated.

This common prefix runs for every reached Zone and Space and for every
solution algorithm. It copies finite, infinite, or NaN values verbatim and
performs no calculation or finite/range repair. It does not modify current
`MAT` or `airHumRat`, `ZT`, `XMPT`, `WTimeMinusP`, `airRelHum`,
`DSXMAT`/`DSWPrevZoneTS`, `TM2/TMX`, `WM2/WMX`, Zone-timestep averages,
or diagnostic state.

CP213 is therefore not a byte-for-byte inverse of CP209. If CP209 changes
`[old0, old1, old2, old3]` to `[new, old0, old1, old2]`, one CP213 call
produces `[old0, old1, old2, old2]`; the original oldest slot is not restored.

#### Exact-Zone RoomAir and literal mixed-level anomaly

Only exact `spaceNum == 0` enables shared RoomAir work. Any positive Space or
malformed negative Space identity receives the common record copy and skips
all RoomAir branches. Like CP209 but unlike CP211, CP213 does not read the global
`anyNonMixingRoomAirModel` flag.

When the exact per-Zone AirModel enum is `DispVent3Node`, `UFADInt`, or
`UFADExt`, CP213 first forward-copies `XMATFloor` slots one through three
into zero through two, then does the same for `XMATOC`. Each becomes
`[old1, old2, old3, old3]`.

The immediately following mixed-level code is deliberately recorded literally:

1. `XMATMX[0] = XMATMX[1]`;
2. `XMATMX[1] = XMATMX[2]`;
3. `XMATMX[3] = XMATMX[3]`.

It never writes slot two, and the slot-three assignment is a no-op. Therefore
`[old0, old1, old2, old3]` becomes
`[old1, old2, old2, old3]` rather than the Floor/occupied result. This
asymmetry is consistent with a possible source typo, but the source map does
not normalize or silently correct it.

An independent exact `AirflowNetwork` enum test then visits every stored
`AFNZoneInfo(zoneNum).Node` in container order. For each node it forward-copies
`AirTempX` slots one through three into zero through two, then does the same
for `HumRatX`. The stratified and AFN tests are separate `if` statements but
a normal single enum value cannot select both. Mixing, UserDefined,
DispVent1Node, CrossVent, Invalid, Num, or an unmatched stored value receives
only the common record copy.

#### Validation, failure, retry, and reset

The positive-Zone assertion can compile out. CP213 has no Zone upper-bound,
Space sign or upper-bound, membership, record-kind, arena-shape, RoomAir
allocation/topology, enum-validity, or finite validation. The caller selects
`this` before entry and `spaceNum` is only an exact-zero classifier, so a
malformed direct call can apply Zone RoomAir semantics to a Space record or
suppress them for a Zone record.

CP213 is void and has no diagnostic, status, latch, catch, cleanup,
transaction, or rollback. Assertion failure precedes mutation. A later
RoomAir access failure retains the complete common record prefix and can retain
partial Floor, occupied, mixed, or earlier AFN-node histories. CP212 then
cannot reach later Spaces or Zones.

Same-state retry applies the forward copy again. A normal history becomes
`[old2, old3, old3, old3]` after two calls; the anomalous mixed-level history
becomes `[old2, old2, old2, old3]`. The operation is destructive and
non-idempotent unless the relevant slots are already equal. Clean replay
requires coordinated Zone/Space `XMAT`/`WPrevZoneTS`, RoomAir/AFN histories,
topology, aggregate traversal state, caller state, dependencies, and
diagnostics. CP211 downstepped histories and non-ThirdOrder scalar state
require separate restoration.

#### C++ test and oracle boundary

The C++ unit tree contains zero direct CP213 or CP212 calls, zero Revert
selector uses, and zero test-side `ManageZoneAirUpdates` or `ManageHVAC`
calls. It also contains zero assertions or occurrences naming `XMAT`,
`WPrevZoneTS`, `XMATFloor`, `XMATOC`, `XMATMX`, `AirTempX`, or
`HumRatX` destinations. No test detects the mixed-level anomaly or covers
partial failure, retry, reset, or record identity.

All 57 active `ManageSimulation` call expressions execute CP213 exactly zero
times. If one external CP212 request were injected in each configuration's
corresponding active sizing or simulation phase, the maximum counterfactual
one-pass topology would be 81 Zone plus 24 eligible Space records, 105 common
copies total. CP213 is solution-algorithm independent, so the ThirdOrder versus
Analytical census does not split this path. All 81 Zones are Mixing and every
Space skips shared RoomAir, leaving stratified and RoomAir-AFN potential at
zero. The 24 Spaces comprise three simulation and 21 sizing identities across
eight configurations.

Tracked one-Zone manifests request no history or revert variable and cannot
execute source CP213. Hourly MAT and humidity outputs are downstream-only and
cannot prove a dormant record transaction or expose the mixed-level anomaly.

#### Rust boundary

Rust has zero singular `revert_zone_timestep_history` definitions, calls, or
tests. Only the CP212 plural source-order and compat identity wrappers exist.
Production never dispatches the Revert selector, and
`manage_zone_air_updates_compat` ignores its selector argument.

The nearest Rust Zone-history mutation is the opposite direction. The
predictor closure pushes three-slot `previous_mean_air_temperatures_c` and
`previous_air_humidity_ratios` as `[new, old0, old1]`. There is no production
forward copy of either Zone history.

The plural Revert compat site runs only on the adaptive count-greater-than-one
path. A count match reuses prior local system histories and its closure is a
no-op. A count mismatch down-interpolates the already-pushed three-slot Zone
histories, preserving slot zero, then overwrites only current
`mean_air_temperature_c` and `air_humidity_ratio` from local slot zero. Each
fine step later pushes temporary local system arrays as
`[current, old0, old1]`, and the plural PushSystem wrapper commits them once
after the Zone-local loop. Count one and the feature-disabled path instead
rebuild three-slot system histories. None performs CP213's unconditional
Zone-record forward copy.

Rust heat-balance state has Zone and Surface arenas only. Its Zone record owns
three-slot Zone/system temperature and humidity arrays but no fourth slot,
Space heat-balance arena or membership, aggregate Space flag, Floor/occupied/
mixed RoomAir histories, RoomAir enum, or AFN node histories. Uniform weather initialization resets the same Rust Zone humidity state
uniformly rather than forward-copying; separate IdealLoads humidity pushes use
different state and the opposite direction.

One parent source-order label test has no state assertion. The focused
single-system-timestep test takes count one and never reaches the Revert site;
the pure interpolation test does not compose it. A full-timestep assertion
uses uniform three-slot temperature history and no humidity assertion, so it
cannot distinguish direction. There is no direct singular, compat, or selector
test; count-greater-than-one match/mismatch test; nonuniform forward-copy or
fourth-slot assertion; Space, mixed-level anomaly, RoomAir, AFN, failure,
retry, or reset evidence.

CP213 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_space_heat_balance_revert_zone_timestep_history`
immediately after `routine.revert_zone_timestep_histories`. The heat-balance
project contract adds
`zone_space_heat_balance_revert_zone_timestep_history` after
`revert_zone_timestep_histories` and before
`update_final_surface_heat_balance`. The algorithm remains a `scaffold` with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code, mapped
state, test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion is added. The inventory
becomes 32 algorithms and 221 routines, split 58 `state_mapped` plus 163
`source_mapped`, with 98 required; the heat-balance project list becomes 67.

### CP214 `ZoneSpaceHeatBalanceData::correctHumRat` source map

`ZoneSpaceHeatBalanceData::correctHumRat(EnergyPlusData &state, int zoneNum,
int spaceNum = 0)` is declared at `ZoneTempPredictorCorrector.hh` line 241 and
implemented at `ZoneTempPredictorCorrector.cc` lines 4433-4619.

Its sole production direct call expression is CP207 `correctAirTemp` line
4128. Before entry, that parent has selected histories, solved temperature,
written `MAT = ZT`, and reported sensible demand. Only after CP214 returns
does CP207 commit `airHumRat = airHumRatTemp`, calculate relative humidity,
classify temperature change, and return. A CP214 non-return therefore leaves
the parent record's committed humidity and RH old even when earlier CP214
effects survive.

The ordinary HVAC path reaches CP214 through
`ManageZoneAirUpdates(CorrectStep)` -> CP206 `correctZoneAirTemps` -> CP207.
The initial correction follows Predict and `SimHVAC`. If adaptive shortening
selects strict `TimeStepSys < TimeStepZone`, every fine step repeats Predict,
`SimHVAC`, correction, contaminant correction, and system-history push.
Within each correction pass CP206 calls every Zone first and then only stored
Spaces satisfying
`doSpaceHeatBalanceSimulation && !DoingSizing`. CP214 therefore executes once
per selected record, not as a standalone global traversal.

#### Flow collection and coefficient order

CP214 debug-asserts only `zoneNum > 0`, initializes local moisture and dry-air
mass flows to zero, aliases the parent Zone, calculates
`ZoneMult = Multiplier * ListMultiplier`, and snapshots controlled,
return-plenum, and supply-plenum flags. Exactly one primary branch runs, in
this priority:

| Branch | Ordered contribution |
|---|---|
| controlled Zone | every configured inlet node contributes `MassFlowRate * HumRat / ZoneMult` and `MassFlowRate / ZoneMult` |
| return plenum | every plenum inlet, then every ADU's upstream leak followed by downstream leak, contributes the corresponding node humidity and leak flow divided by `ZoneMult` |
| supply plenum | the one plenum inlet contributes its flow and humidity divided by `ZoneMult` |
| none | both primary-flow locals remain zero |

Parallel-PIU leakage is independent of that branch. A nonempty
`leakageParallelPIUNums` vector causes an ordinal loop from one through the
vector size. The literal implementation indexes global `PIU(piuNum)` instead
of reading the PIU identities stored in the vector. Thus stored identities
such as `[7, 12]` select `PIU(1)` and `PIU(2)`; each selected positive
`leakFlow` uses its primary-inlet humidity. CP214 maps this source behavior
without normalizing it.

Next it constructs:

```text
LatentGain =
    record latentGain
  + SumLatentHTRadSys(zoneNum)
  + SumLatentPool(zoneNum)

B =
    LatentGain / H2OHtOfVap
  + (OAMFL + VAMFL + CTMFL) * OutHumRat
  + EAMFLxHumRat
  + MoistureMassFlowRate
  + SumHmARaW
  + MixingMassFlowXHumRat
  + MDotOA * OutHumRat

A =
    ZoneMassFlowRate
  + OAMFL + VAMFL + EAMFL + CTMFL
  + SumHmARa + MixingMassFlowZone + MDotOA

C =
    PsyRhoAirFnPbTdbW(OutBaroPress, ZT, airHumRat)
  * parent Zone.Volume
  * parent Zone.ZoneVolCapMultpMoist
  / TimeStepSysSec
```

Density is evaluated before water-vapor enthalpy, and default `B` is assigned
before `A`. When AFN is always multizone-simulated, or its control is
distribution-only-during-fan-operation with the fan active, CP214 replaces
rather than augments those default `A/B` values:

```text
B =
    LatentGain / H2OHtOfVap
  + exchange.SumMHrW + exchange.SumMMHrW
  + MoistureMassFlowRate + SumHmARaW

A =
    ZoneMassFlowRate
  + exchange.SumMHr + exchange.SumMMHr
  + SumHmARa
```

After `C` is calculated, independent AFN-distribution and duct-loss flags add
`exchange.TotalLat` and then `ZoneLat(zoneNum)` to `B`. These additions do not
depend on whether the multizone replacement ran.

A positive Space call still uses the parent Zone's equipment/plenum/PIU
topology, multiplier, radiant and pool gains, AFN exchange, duct loss, volume,
and moisture-capacitance multiplier. Only record fields addressed through
`this`, the final Space node, and the Space latent-sizing demand owner are
Space-specific. In particular, Space `C` does not use Space volume.

#### Solver, clamps, and RoomAir override

The solution switch writes `airHumRatTemp` as follows:

| Solution enum | Calculation |
|---|---|
| `ThirdOrder` | `(B + C * (3*WPrev[0] - 1.5*WPrev[1] + WPrev[2]/3)) / ((11/6)*C + A)` |
| `AnalyticalSolution`, exact `A == 0` | `W1 + B/C`; the source comment says `B=0`, but the code neither tests nor forces that |
| `AnalyticalSolution`, nonzero `A` | `(W1 - B/A) * exp(min(700, -A/C)) + B/A` |
| `EulerMethod` | `(C*W1 + B) / (C + A)` |
| any unmatched value | no assignment; the previous `airHumRatTemp` continues |

There is no denominator, finite, or enum diagnostic. CP214 first changes a
strictly negative result to literal zero, calculates saturation humidity at
`ZT` and outdoor barometric pressure, and caps a strictly higher result.
Comparison-detected values alone are clamped: zero survives, and NaN can pass
both tests.

An exact parent-Zone RoomAir `AirflowNetwork` enum then overwrites
`airHumRatTemp` from the Zone control-air node. This test has no exact-Zone,
global nonmixing, RoomAir-active, or AFN-activity gate, so a positive Space
record receives the same Zone control-node humidity. Because the overwrite is
after both clamps and has no second clamp, it can restore a negative,
supersaturated, or nonfinite value.

The density helper floors only its local working humidity to `1e-5`; it does
not change the record. The node-enthalpy helper likewise uses its own
psychrometric handling. Consequently an AFN-supplied negative node
`HumRat` can coexist with enthalpy calculated from a locally floored humidity.

#### Hybrid, node, and latent-sizing effects

Hybrid humidity work requires exact `spaceNum == 0`, the global hybrid flag,
either humidity infiltration or people inference, and both non-warmup and
non-sizing state. People inference alone builds
`latentGainExceptPeople`. The child always samples measured humidity and, after the optional date-window
block, always shifts measured-humidity history on successful return. Within
the date window it may mutate Zone `airHumRat`, measured supply fields, and
infiltration/people inference outputs; inference additionally requires
Zone-timestep history. The child does not update `airHumRatTemp`; on a normal
return CP207 later overwrites its temporary measured `airHumRat` with the
solved value.

Node selection starts from the parent Zone system node. Only a positive
`spaceNum` selects the Space system node; any nonpositive identity uses the
Zone node. A strictly positive node number causes `Node.HumRat` to be written
before `Node.Enthalpy`. Controlled or plenum status is not part of the actual
write gate despite the source comment.

When `DoLatentSizing` is true, CP214 computes saturation pressure at `ZT`,
dewpoint from `airHumRatTemp` at standard barometric pressure, and vapor
pressure difference. A positive Space selects its sensible and moisture
demand records; otherwise the parent Zone records are used. The report child
receives the raw `LatentGain`, the already reported sensible heat-plus-cool
rate, and vapor-pressure difference. It writes latent heating/cooling rates,
energies, sensible heat ratio, and vapor-pressure difference. AFN and duct
terms added to solver `B` are not added to this raw report argument.

A malformed negative `spaceNum` is not rejected. It skips only the exact-Zone
hybrid branch while otherwise using parent-Zone node, demand, topology, and
capacity state; the RoomAir AFN override still runs.

#### Failure, retry, and reset

Beyond the debug assertion, CP214 validates no Zone upper bound, Space sign,
record kind or membership, multiplier, volume, timestep, pressure, flow,
denominator, solution enum, allocation, node, plenum, ADU, PIU, AFN, hybrid,
demand topology, or finite result. It returns void and owns no status, catch,
cleanup, completion marker, transaction, or rollback. Psychrometric children
can also mutate diagnostic or cache state.

A topology or early psychrometric non-return can precede any record write.
Failure during saturation retains a newly solved or lower-clamped
`airHumRatTemp`. Later failures can retain RoomAir or hybrid effects, a new
node humidity with stale enthalpy, or partial latent-sizing demand reports.
The parent `MAT` write and sensible report already precede every CP214 call.
Any non-return blocks CP207's final humidity/RH commit and returned delta,
then blocks the remaining CP206 record traversal.

Retry recomputes from live nodes, the current record humidity, AFN, schedules,
and demand state. It can resample hybrid schedules, shift measured histories
again, repeat reports and diagnostics, or use humidity committed by an earlier
successful CP207 call. `ZoneTempPredictorCorrectorData::clear_state` rebuilds
only its predictor/corrector records; clean replay also requires coordinated
reset of nodes, Zone/Space and equipment topology, AFN/RoomAir, hybrid state,
demands, HVAC/environment state, and psychrometric diagnostics/cache.

#### C++ reach

There are ten direct unit-test call expressions. The
`ZoneTempPredictorCorrector_CorrectZoneHumRatTest` fixture calls CP214 six
times under Euler: five uncontrolled/no-plenum calls followed by one
controlled call. Its six assertions inspect only the Zone system node humidity
at `0.008`. `HybridModel_correctZoneAirTempsTest` directly calls CP214 four
times for two humidity-infiltration and two humidity-people cases; its four
assertions inspect only inferred infiltration or people results.

The same hybrid fixture calls CP206 five times. With one Zone and inactive
Space-HB flags, each transitively enters CP214 once while humidity hybrid modes
are off; those assertions target temperature-hybrid effects. Focused normal
dynamic entry is therefore ten direct plus five indirect calls. There is no
focused assertion for a direct Space, return/supply plenum, ADU/PIU, AFN,
duct, negative or saturation clamp, ThirdOrder or Analytical formula,
latent-sizing report, node enthalpy, unknown enum, failure, retry, or reset.

Of 57 active `ManageSimulation` expressions, one expected EMS fatal stops
before correction and one zero-Zone case executes no record. The remaining 55
configurations reach Zone CP214. Their static one-correction-pass topology is
81 Zone plus three active simulation-Space records, for 84 record calls split
74 ThirdOrder, ten Analytical, and zero Euler. Warmup, run periods, the initial
correction, and any adaptive fine steps make actual runtime counts larger;
84 is a configuration census, not an execution total.

Those 84 records split into 55 controlled Zones and 29 records with no
controlled primary flow: 26 Zones plus the three active Spaces. The corpus has
no return/supply-plenum or parallel-PIU leakage topology. AFN multizone
replacement can reach five Zone identities, its distribution `TotalLat`
addition three, and duct latent addition three. No active full simulation has
RoomAir AFN or HybridModel Zone humidity. One latent-sizing configuration can
reach the Zone moisture-report branch, while positive-Space latent reporting
has zero corpus reach. Existing assertions do not isolate any of those
coefficient, override, or report effects, CP214's branch order, or its failure
transaction.

#### Rust boundary

Rust has no source-shaped singular `correct_hum_rat` helper, wrapper, or
Zone/Space record transaction. The main heat-balance path instead calls
`correct_zone_air_humidity_ratios_from_current_state` once after a complete
all-Zone temperature pass. It writes every Zone's current humidity directly
from either a three-slot history term divided by `11/6` or history slot zero.
The adaptive count-greater-than-one path separately invokes
`correct_single_zone_air_humidity_ratio_from_history` inside each Zone-local
fine-step loop.

Those helpers have no moisture `A/B/C`, volume/capacitance term, latent or
supply flow, plenum/PIU/leak, outdoor/mixing/surface-moisture, AFN/duct,
Analytical/Euler, RoomAir, hybrid, node, RH, Space, or latent-sizing report.
They clamp to saturation, then enforce a `1e-5` minimum, and replace a nonpositive
timestep or nonfinite output with `0.008`; these guards differ from CP214's raw
math and literal-zero allowance. Neither helper has a direct test, and the
focused adaptive test takes count one and bypasses the per-substep helper.

A separate public IdealLoads helper,
`correct_no_oa_third_order_humidity_ratio_compat`, is an explicit bounded
analogue. For one aggregated purchased-air supply it validates inputs, builds
density and vapor enthalpy, uses
`C = rho * volume * moistureMultiplier / timestep`,
`B = latent/Hvap + supplyMass * supplyHumidity`, `A = supplyMass`, applies the
exact ThirdOrder equation, and clamps negative and saturated results. It
returns corrected humidity plus `A/B` through `Option`.

That helper omits every other CP214 source term and lifecycle effect, all
plenum/leak/AFN/duct and alternate-solver branches, Space state, RoomAir,
hybrid, node/RH commit, and latent-sizing reporting. Its closed-loop
IdealLoads owner atomically replaces two independent three-slot histories
after PurchasedAir succeeds. Two direct tests check the formula and saturation,
and one humidistat test makes an independent helper call; none proves the
heat-balance transaction. Existing official dynamic and no-OA IdealLoads
humidity claims remain at their declared case boundaries and are not widened
or reattributed to full CP214 parity.

CP214 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_space_heat_balance_correct_hum_rat`
immediately after
`routine.zone_space_heat_balance_revert_zone_timestep_history`. The
heat-balance project contract adds
`zone_space_heat_balance_correct_hum_rat` after
`zone_space_heat_balance_revert_zone_timestep_history` and before
`update_final_surface_heat_balance`. The algorithm remains a `scaffold` with
`claim_level = none`. No EnergyPlus source inventory, Rust target, code,
mapped state, test, support, capability, output implementation, comparator,
manifest, numerical, performance, or conformance promotion is added. The
inventory becomes 32 algorithms and 222 routines, split 58 `state_mapped`
plus 164 `source_mapped`, with 99 required; the heat-balance project list
becomes 68.

### CP215 scalar-output `DownInterpolate4HistoryValues` source map

CP215 maps only the void overload that snapshots three scalar history values
and writes five scalar outputs by reference. It is declared at
`ZoneTempPredictorCorrector.hh` lines 299-308 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4621-4702. The following array-return
overload is a separate definition and does not call this one.

The source calculates `DSRatio = OldTimeStep / NewTimeStep` before writing any
output, then writes `newVal0 = oldVal0`. Branch selection is ordered and uses
strict floating comparisons:

| Gate | Remaining ordered writes |
|---|---|
| `abs(DSRatio - 2.0) < 0.01` | `newVal1 = (oldVal0 + oldVal1) / 2`; `newVal2 = oldVal1`; `newVal3 = (oldVal1 + oldVal2) / 2`; `newVal4 = oldVal2` |
| otherwise, `abs(DSRatio - 3.0) < 0.01` | `delta = (oldVal1 - oldVal0) / 3`; `newVal1 = oldVal0 + delta`; `newVal2 = newVal1 + delta`; `newVal3 = oldVal1`; `newVal4 = oldVal1 + (oldVal2 - oldVal1) / 3` |
| every other ratio | `delta = (oldVal1 - oldVal0) / DSRatio`; then `newVal1` through `newVal4` are formed by four sequential additions of `delta` |

The ratio-two averages use sum-before-division, and the ratio-three and
fallback branches read the just-written preceding output rather than using
independent closed forms. Their rounding and overflow behavior therefore
belongs to the contract. Despite the fallback comment saying “4 or more,” its
actual domain includes ratios below one, negative values, values merely
outside either tolerance band, zero, infinity, and NaN. Exact decimal-looking
tolerance edges remain subject to their represented binary values.

CP215 validates no positive or finite timestep, shortening direction, integer
ratio, finite history value, or distinct output storage. Under ordinary IEEE
floating behavior, division by zero and nonfinite arithmetic propagate rather
than reporting a diagnostic. A nonzero old timestep divided by zero normally
selects the fallback with an infinite ratio and zero-like increments; zero
divided by zero or a NaN timestep reaches the fallback with NaN increments.
An old timestep of zero produces a zero ratio and can create infinite or NaN
increments. `oldVal2` is unused in the fallback branch.

The three old values are copied before the body executes, so a first call
whose output aliases caller input still reads the entry snapshots. The five
output references are not checked for mutual aliasing: assignments occur in
`newVal0` through `newVal4` order, and the last write to shared storage wins.
Production and focused-test callers pass distinct input and output locations.

#### Production ownership and cadence

The only production expressions are in
`ZoneContaminantPredictorCorrector::PredictZoneContaminants`, inside its
ascending Zone loop:

1. lines 1588-1597 interpolate CO2 history into `ZoneAirCO2` followed by
   `DSCO2ZoneTimeMinus1` through `DSCO2ZoneTimeMinus4`;
2. lines 1600-1609 interpolate generic-contaminant history into `ZoneAirGC`
   followed by `DSGCZoneTimeMinus1` through `DSGCZoneTimeMinus4`.

The manager must first pass its `SimulateContaminants` early return and select
`PredictStep`. Within each Zone, CP215 additionally requires
`ShortenTimeStepSys`, a current `NumOfSysTimeSteps` different from
`NumOfSysTimeStepsLastZoneTimeStep`, and the corresponding CO2 or generic
simulation flag. A positive system Zone node controls only the preceding node
rollback and is not a helper gate. `UseZoneTimeStepHistory` likewise does not
gate the writes; it controls which histories the caller copies afterward.

For one eligible predictor call, the dynamic count is the number of Zones
multiplied by the number of enabled contaminant species, with CO2 preceding
generic contaminant for each Zone. `HVACManager` starts the initial prediction
with shortening false. If the initial correction selects a shorter adaptive
system timestep, the first strict-shorter fine-step prediction can enter
CP215. The manager clears shortening after that fine-step simulation, so later
fine-step predictions do not enter it. If the system-step count matches the
previous Zone timestep's count, the caller instead reuses existing
downstepped histories. On the normal adaptive path,
`UseZoneTimeStepHistory` is false and the completed downstepped slots one
through three immediately become the working contaminant histories.

#### Failure, retry, and reset

CP215 is void and owns no status, assertion, diagnostic, callback, allocation,
catch, cleanup, transaction, cache, static state, or rollback. Caller argument
indexing fails before function entry. In the ordinary IEEE environment the
body has no normal non-return path and overwrites all five destinations.
With explicitly enabled floating traps or an external abnormal interruption,
the ratio division can fail before `newVal0`, or later arithmetic can retain
the already written prefix.

The CO2 call precedes the generic call, so a later caller failure can retain a
complete CO2 interpolation and no or partial generic interpolation. A
complete direct retry with stable by-value inputs and five distinct outputs
is deterministic overwrite-idempotent. Input/output aliasing, caller-mutated
histories, or changed timesteps can make retry observe different input
snapshots. The helper has no reset owner; restoration belongs to the
contaminant histories and the surrounding HVAC state.

#### C++ reach

`DownInterpolate4HistoryValues_Test` calls this scalar overload once with
`0.25 / 0.125`, exercising only the ratio-two branch. Five post-call
assertions inspect `newVal0` and the four downstepped values. Its later call
and nine further assertions target the independent array overload, not CP215.

The three focused `PredictZoneContaminants` fixtures call at source lines 230,
547, and 736 with `ShortenTimeStepSys` false, so indirect focused CP215 reach
is zero. No test-side contaminant-manager `PredictStep` or `ManageHVAC` call
adds another route. Ratio three, the fallback, tolerance boundaries,
nonpositive or nonfinite timesteps and values, aliasing, partial failure,
retry, and reset have no scalar C++ test.

All 57 active full-simulation `ManageSimulation` expressions execute CP215
zero times. The only such unit input containing
`ZoneAirContaminantBalance` sets CO2 simulation to `No`, generic contaminant
simulation is absent, and no full-simulation fixture programmatically enables
either flag. The corpus therefore supplies neither adaptive-gate reach nor a
contaminant interpolation oracle.

#### Rust boundary

The nearest Rust function is
`energyplus_down_interpolate_three_history_values` in
`crates/ep_runtime/src/heat_balance/zone_air_correction.rs`. For ordinary
positive timesteps it reproduces only source outputs `newVal0` through
`newVal2` for the ratio-two, ratio-three, and fallback formulas. It returns a
three-element value array, never produces source `newVal3` or `newVal4`, never
uses its third old value, and has no reference-alias behavior. Unlike CP215,
it returns the original three values immediately when either timestep is
nonpositive.

Production has two lexical Rust calls, one for Zone temperature histories and
one for Zone humidity histories. Their enclosing adaptive path is enabled by
the compatibility configuration and disabled by the ordinary diagnostic
configuration. They run per Zone only after the project selects an adaptive
system-step count greater than one and only when that integer count differs
from the prior count. This is local thermal
heat-balance ownership, not the source's CO2/generic-contaminant transaction;
Rust has no corresponding contaminant arena or predictor call.

One Rust test calls the helper three times and uses three array equality
assertions for ratios two, three, and four. It does not establish the two
missing outputs, old-value-three use, tolerance edges, invalid-input behavior,
aliasing, source production cadence, failure, retry, or reset.

CP215 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.down_interpolate_4_history_values`
after `routine.zone_space_heat_balance_correct_hum_rat`. The heat-balance
project contract adds `down_interpolate_4_history_values` after
`zone_space_heat_balance_correct_hum_rat` and before
`update_final_surface_heat_balance`. This is one logical overloaded-routine
entry, but its CP215 evidence boundary covers only the scalar-output
definition. The algorithm remains a `scaffold` with `claim_level = none`. No
EnergyPlus source inventory, Rust target, code, mapped state, test, support,
capability, output implementation, comparator, manifest, numerical,
performance, or conformance promotion is added. The inventory becomes 32
algorithms and 223 routines, split 58 `state_mapped` plus 165
`source_mapped`, with 100 required; the heat-balance project list becomes 69.

### CP216 array-return `DownInterpolate4HistoryValues` source map

CP216 expands the existing logical
`routine.down_interpolate_4_history_values` mapping to its second overload.
This independent definition returns a scalar and fills a four-element array; it
does not call the CP215 scalar-output overload. It is declared at
`ZoneTempPredictorCorrector.hh` lines 310-311 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4704-4736.

For `a = oldVals[0]`, `b = oldVals[1]`, `c = oldVals[2]`, and
`r = OldTimeStep / NewTimeStep`, the function calculates `r` before any output
and then writes `newVals[0] = a`. The remaining ordered behavior is:

| Ordered gate | Remaining writes |
|---|---|
| `abs(r - 2.0) < 0.01` | `newVals[1] = (a + b) / 2`; `newVals[2] = b`; `newVals[3] = (b + c) / 2` |
| otherwise, `abs(r - 3.0) < 0.01` | `d = (b - a) / 3`; `newVals[1] = a + d`; `newVals[2] = newVals[1] + d`; `newVals[3] = b` |
| every other ratio | `d = (b - a) / r`; `newVals[1] = a + d`; `newVals[2] = newVals[1] + d`; `newVals[3] = newVals[2] + d` |

Only after all four writes does the helper return `oldVals[0]`. Ratio-two
averages use sum-before-division, while ratio-three and fallback outputs use
the just-written prior value. Those evaluation orders, including their
rounding and overflow behavior, are part of the source boundary.
`oldVals[3]` is never read. `oldVals[2]` contributes only to the ratio-two
final output.

The strict ratio-two test precedes the strict ratio-three test. Despite the
fallback comment saying four or more, every other represented ratio reaches
it, including ratios below one, negative and noninteger ratios, strict
tolerance-edge values, zero, infinity, and NaN. CP216 validates no positive or
finite timestep, shortening direction, integer ratio, or finite history
value. The fixed `std::array<Real64, 4>` type supplies shape at the call
boundary, but the helper performs no separate object or alias check. Under
ordinary masked IEEE behavior, zero and nonfinite division or arithmetic
propagates without a diagnostic; floating traps can instead stop the ordered
write prefix.

Unlike CP215's by-value scalar inputs, `oldVals` is a live const reference.
The const input and mutable output references may name the same array. With
initial input `[a, b, c, x]`, same-array aliasing changes ratio-two output to
`[a, m, m, m]`, where `m = (a + b) / 2`, because later reads observe earlier
writes. Ratio three becomes `[a, a + d, a + 2d, a + d]`. The fallback's first
same-array result matches the distinct-array recurrence because `d` is
captured before writes, but it still destroys the next call's inputs.
Production and the direct C++ test use distinct arrays.

#### Production ownership and cadence

All seven production call expressions are inside
`ZoneSpaceHeatBalanceData::updateTemperatures`, implemented at
`ZoneTempPredictorCorrector.cc` lines 6768-6833:

| Source order | Input to output array; returned-current owner |
|---|---|
| line 6800 | `XMAT` to `DSXMAT`; return to `MAT` |
| line 6801 | `WPrevZoneTS` to `DSWPrevZoneTS`; return to `airHumRat` |
| lines 6806, 6808, and 6810 | exact-Zone Floor, occupied, then mixed RoomAir temperature histories; returns to `MATFloor`, `MATOC`, then `MATMX` |
| lines 6815 and 6817 | each stored exact-Zone AFN node's temperature then humidity histories; returns to `AirTemp` then `HumRat` |

`ZoneSpaceHeatBalanceData::predictSystemLoad` calls `updateTemperatures` first
at line 3155. CP202 `PredictSystemLoads` visits Zones in ascending identity
order and, after each Zone, visits its stored Spaces when
`doSpaceHeatBalance` is true. A positive Space therefore receives the two
record-level calls but skips all shared RoomAir branches. If Space heat
balance is inactive, a shortened predictor merely mirrors the already updated
Zone `MAT` and `airHumRat` into each Space and does not call CP216 for that
Space.

Before interpolation, a positive exact Zone or Space system node can have
temperature, thermostat air temperature, humidity, and enthalpy rolled back
from the first Zone-timestep histories. CP216 then requires both
`ShortenTimeStepSys` and
`NumOfSysTimeSteps != NumOfSysTimeStepsLastZoneTimeStep`. The history selector
is not a helper gate: after the block, `UseZoneTimeStepHistory` chooses either
the Zone-timestep arrays or the downstepped arrays for `ZTM` and
`WPrevZoneTSTemp`.

Every eligible Zone or active Space has two base calls. Only exact Zones can
add three calls when the global non-Mixing flag is set and that Zone is
displacement-ventilation or UFAD. Within that same global-gated exact-Zone
block, an independent AirflowNetwork enum branch adds two calls per stored AFN
node. Thus one eligible prediction has:

```text
2 * (eligible Zone records + eligible Space records)
+ 3 * eligible displacement/UFAD Zones
+ 2 * eligible AFN nodes
```

`HVACManager` begins with a full Zone timestep and shortening false, so its
initial prediction cannot enter CP216. If the initial correction selects a
strictly shorter adaptive timestep, the first fine-step prediction can enter;
the manager clears shortening after that fine-step simulation, preventing
later fine steps from entering. A matching previous Zone-timestep system-step
count reuses existing downstepped arrays instead. The minimum system-timestep
clamp can also make the raw old/new ratio differ from the selected integer
count. `SimulationManager::Resimulate` passes literal false shortening and
cannot enter CP216.

#### Failure, retry, and reset

CP216 has no status, assertion, diagnostic, callback, allocation, catch,
cleanup, transaction, cache, static state, or rollback. In ordinary masked
IEEE operation its body completes. A floating trap can occur before
`newVals[0]` during ratio calculation or after an output prefix during branch
arithmetic. The caller's scalar assignment happens only after a completed
return, so an abnormal interruption can retain an array prefix without the
matching `MAT`, humidity, or RoomAir/AFN current-value assignment.

The base temperature call precedes the base humidity call, followed by
stratified and AFN calls in the table order. A later abnormal non-return
therefore preserves earlier completed helper transactions, the preceding node
rollback, and every earlier Zone or Space. It blocks later interpolation and
the following predicted-load work.

With stable timesteps, immutable input arrays, and distinct output arrays, a
complete retry deterministically overwrites the same four outputs and returned
scalar. A same-array retry is generally non-idempotent because the first call
mutates the next call's live inputs. Changed histories, topology, counts, or
timesteps can likewise change replay. CP216 owns no reset; recovery belongs to
the Zone/Space record, RoomAir/AFN histories, node state, and surrounding
predictor/HVAC owners.

#### C++ reach and corpus boundary

`DownInterpolate4HistoryValues_Test` calls the array overload once at
`ZoneTempPredictorCorrector.unit.cc` line 1790 with
`0.25 / 0.125 = 2`. Nine post-call assertions inspect the returned scalar,
all four output elements, and all four unchanged elements of the distinct
input array. The earlier scalar call and five destination assertions in the
same fixture belong to CP215.

The two focused `PredictSystemLoads` fixtures make 16 wrapper calls, including
four shortened calls, but both retain `NumOfZones = 0`; they invoke no
`predictSystemLoad` or `updateTemperatures` child. There is no direct child,
test-side `ManageZoneAirUpdates`, or `ManageHVAC` call. Focused indirect CP216
reach is therefore zero. No C++ test covers ratio three, the fallback,
tolerance boundaries, invalid or nonfinite inputs, same-array aliasing,
partial failure, retry, or reset.

Of 57 active full-simulation `ManageSimulation` expressions, one expected EMS
fatal stops before prediction and one has zero Zones. The remaining 55
configurations conservatively bound actual adaptive count-change entry at zero
through 55 because no test observes that runtime gate. Their conditional
one-pass topology contains 81 Zones plus 24 eligible Spaces, or 105 records
and 210 base temperature/humidity calls if each crossed the gate once. All 81
Zones are Mixing, so the three stratified calls and AFN-node calls have zero
corpus potential. This is a static conditional census, not observed execution;
downstream timestep and output assertions do not isolate CP216.

#### Rust boundary

The nearest Rust helper,
`energyplus_down_interpolate_three_history_values`, takes and returns
three-element arrays by value. For ordinary positive timesteps it reproduces
source `newVals[0]` through `newVals[2]` with the same branch order, strict
tolerance, sum-before-division, and sequential additions. It has no fourth
output, separate `oldVals[0]` scalar return, fourth input shape, or reference
alias behavior; its third input is never used. Unlike CP216, it returns the
original array immediately when either timestep is nonpositive.

Rust has two lexical production calls, for Zone temperature and humidity.
They run in the compatibility-enabled adaptive path only when the local
integer system-step count is greater than one and differs from the previous
count, before the Zone-local fine-step loop. A later compat closure copies
returned element zero into current Zone temperature and humidity. Rust has no
Space record, stratified RoomAir, AFN node, or source node-rollback
transaction.

One Rust test calls the helper three times and has three array-equality
assertions for ratios two, three, and four. Its focused adaptive parent test
selects count one and never calls the helper. No Rust test establishes the
missing fourth output and separate scalar return, source topology/cadence,
tolerance edge, invalid/nonfinite behavior, aliasing, partial failure, retry,
or reset.

CP216 expands the evidence boundary of the already required
`source_mapped`
`zone_temp_predictor_corrector_source_order.routine.down_interpolate_4_history_values`.
It adds no second routine or heat-balance project-contract item. The algorithm
remains a `scaffold` with `claim_level = none`; no EnergyPlus source inventory,
Rust target, code, mapped state, test, support, capability, output
implementation, comparator, manifest, numerical, performance, or conformance
promotion is added. Counts remain 32 algorithms and 223 routines, split 58
`state_mapped` plus 165 `source_mapped`, with 100 required; the heat-balance
project list remains 69.

### CP217 `InverseModelTemperature` source map

CP217 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.inverse_model_temperature`
immediately after `routine.down_interpolate_4_history_values`. The heat-balance
project contract adds `inverse_model_temperature` after
`down_interpolate_4_history_values` and before
`update_final_surface_heat_balance`. `InverseModelTemperature` is declared at
`ZoneTempPredictorCorrector.hh` lines 313-325 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4737-4951.

The routine aliases the selected Zone, its HybridModel record, and its
Zone heat-balance record. It stores
`ZoneMult = zone.Multiplier * zone.ListMultiplier` in an integer, samples the
measured-temperature schedule or uses zero when that pointer is null, writes
the sample to `ZoneMeasuredTemperature`, and unconditionally resets the current
`ZoneVolCapMultpSensHM` to one. Only the inclusive
`DayOfYear >= HybridStartDayOfYear && DayOfYear <= HybridEndDayOfYear` window
enters the main body. That body first overwrites `ZT` with the sampled measured
temperature and then evaluates three independent branches in source order:
infiltration, internal thermal mass, and people count. Valid input normally
selects only one unknown, but the routine itself uses three separate `if`
statements and can execute more than one branch for malformed or directly
constructed state.

All three numerical branches also require the global
`state.dataHVACGlobal->UseZoneTimeStepHistory`; the similarly named boolean
passed into the surrounding correction wrapper is not CP217's gate. Regardless
of the date window or that global history flag, a normally returned call then
shifts `PreviousMeasuredZT3 <- PreviousMeasuredZT2`,
`PreviousMeasuredZT2 <- PreviousMeasuredZT1`, and
`PreviousMeasuredZT1 <- ZT`. Outside the date window, `ZT` was not replaced, so
the shifted value is the ordinary solved Zone temperature rather than the
schedule sample. Inside the date window with system-timestep history selected,
the measured-temperature overwrite and final shift still occur while every
inverse calculation is skipped.

#### Infiltration-temperature branch

The infiltration branch runs when `InfiltrationCalc_T` and Zone-timestep
history are both true. With `IncludeSystemSupplyParameters`, it dereferences
the measured supply-temperature schedule, treats missing mass-flow and
humidity schedules as zero, evaluates moist-air specific heat from the measured
supply humidity, and builds measured `SumSysMCp_HM` and `SumSysMCpT_HM`.
Without that option it does not use the incoming `SumSysMCp` or `SumSysMCpT`.
The two paths assemble:

| Path | `AA` | `BB` |
|---|---|---|
| measured supply | `SumSysMCp_HM + SumHA + MCPV + MCPM + MCPE + MCPC + MDotCPOA` | `SumSysMCpT_HM + SumIntGain + SumHATsurf - SumHATref + MCPTV + MCPTM + MCPTE + MCPTC + MDotCPOA * OutDryBulbTemp + NonAirSystemResponse / ZoneMult + SysDepZoneLoadsLagged` |
| no measured supply | `SumHA + MCPV + MCPM + MCPE + MCPC + MDotCPOA` | `SumIntGain + SumHATsurf - SumHATref + MCPTV + MCPTM + MCPTE + MCPTC + MDotCPOA * OutDryBulbTemp` |

It then sets `CC = AirCap`,
`DD = 3 * PreviousMeasuredZT1 - 1.5 * PreviousMeasuredZT2 +
PreviousMeasuredZT3 / 3`, stores
`delta_T = ZoneMeasuredTemperature - zone.OutDryBulbTemp`, and evaluates
outdoor-air specific heat from `OutHumRat` and density from
`OutBaroPress`, the Zone-local outdoor dry-bulb temperature, and `OutHumRat`.
Raw infiltration mass flow stays zero unless the strict
`abs(delta_T) > 0.5` gate succeeds, in which case it is:

```text
(BB + CC * DD - ((11 / 6) * CC + AA) * ZoneMeasuredTemperature)
/
(PsyCpAirFnW(OutHumRat) * delta_T)
```

The source converts that result to air changes per hour, clamps it with the
literal nested `max(0, min(10, candidate))`, reconstructs mass flow from the
clamped rate, density, and Zone volume, and writes `MCPIHM` and
`InfilOAAirChangeRateHM`. Equality at either `+0.5` or `-0.5` produces zero.
There is no local finite, density, heat-capacity, volume, or denominator check;
native floating and Objexx min/max behavior remains part of the source
boundary.

#### Internal-thermal-mass branch

The internal-mass branch has the exact combined gate
`InternalThermalMassCalc_T && SumSysMCpT == 0 &&
ZT != PreviousMeasuredZT1 && UseZoneTimeStepHistory`. It forms:

```text
TempDepCoef = SumHA + SumMCp + SumSysMCp
TempIndCoef = SumIntGain + SumHATsurf - SumHATref
            + SumMCpT + SumSysMCpT
            + NonAirSystemResponse / ZoneMult
            + SysDepZoneLoadsLagged
```

AirflowNetwork distribution adds `exchangeData(zoneNum).TotalSen`. Duct-loss
simulation then adds the literal `ZoneLat(zoneNum)`, not `ZoneSen`. With a
zero dependent coefficient, inferred air capacity is
`TempIndCoef / (ZT - PreviousMeasuredZT1)`. Otherwise the routine sets the
temperature ratio to zero on exact
`TempIndCoef == TempDepCoef * ZT`, or computes:

```text
(TempIndCoef - TempDepCoef * PreviousMeasuredZT1)
/
(TempIndCoef - TempDepCoef * ZT)
```

A strictly positive ratio other than exactly one selects
`AirCapHM = TempDepCoef / log(ratio)`; every other ratio falls back to
`TempIndCoef / (ZT - PreviousMeasuredZT1)`. Only strict
`abs(ZT - PreviousMeasuredZT1) > 0.05` derives the multiplier:

```text
AirCapHM
/
(zone.Volume * PsyRhoAirFnPbTdbW(OutBaroPress, ZT, airHumRat)
 * PsyCpAirFnW(airHumRat))
*
(TimeStepZone * 3600)
```

Otherwise it uses one. CP217 passes that value and the running sum, count,
average, and Zone identity by reference to the next lexical routine,
`processInverseModelMultpHM`, then stores the returned/mutated value in
`ZoneVolCapMultpSensHM`. The child owns its clamp, warning, and aggregate
mutations. In particular, its values above 30 are warned but not clamped and
still contribute to the aggregate; mapping that child body is deferred to
CP218.

#### People-count branch

The people branch runs when `PeopleCountCalc_T` and Zone-timestep history are
true. It dereferences and re-samples the measured-temperature schedule, even
though entry already performed a null-safe sample. It stores raw activity,
sensible-fraction, and radiant-fraction schedule values, using zero for a
missing schedule. The activity schedule is sampled a second time for the local
calculation. Local-only defaults replace a nonpositive activity with 130, a
nonpositive sensible fraction with 0.6, and a nonpositive radiant fraction
with convection fraction 0.7; otherwise convection fraction is
`1 - radiantFraction` without clamping. The raw stored fields retain their
sampled zero values when those defaults are used.

With measured supply enabled, this branch again requires the supply-temperature
schedule, treats missing mass-flow and humidity schedules as zero, and forms:

```text
AA = SumSysMCp_HM + SumHA + SumMCp
BB = SumSysMCpT_HM + SumIntGainExceptPeople
   + SumHATsurf - SumHATref + SumMCpT
   + NonAirSystemResponse / ZoneMult + SysDepZoneLoadsLagged
```

Without measured supply, `AA = SumHA + SumMCp` and
`BB = SumIntGainExceptPeople + SumHATsurf - SumHATref + SumMCpT`.
Using the same `CC` and three-history `DD`, the inferred sensible people gain
is `((11 / 6) * CC + AA) * ZoneMeasuredTemperature - BB - CC * DD`.
The denominator is activity times sensible fraction times convection fraction.
The upper bound is `max(0, SumIntGain / denominator)`, and the count is
`min(upperBound, max(0, inferredGain / denominator))`; a strict result below
0.05 is changed to zero before `NumOccHM` is written. CP217 does not validate
the denominator, fractions, schedules, or finite result.

#### Production ownership and cadence

The sole production call expression is inside
`ZoneSpaceHeatBalanceData::correctAirTemp` at
`ZoneTempPredictorCorrector.cc` lines 4103-4114. It requires exact Zone identity
(`spaceNum == 0`), the global `FlagHybridModel`, at least one of the three
temperature-inference flags, and both `!WarmupFlag` and `!DoingSizing`. It runs
after the forward temperature solve, system-node and thermostat writes, load
correction, and `SNLoad` calculation, but before `MAT = ZT`, sensible reporting,
humidity correction, final humidity/relative-humidity commits, and the returned
temperature delta. `correctZoneAirTemps` visits Zones in ascending order before
its Space traversal, but CP217 never runs for a Space.

`HVACManager` begins a normal Zone timestep with global
`UseZoneTimeStepHistory = true`, so the initial correction can run one full
inverse calculation. When adaptive shortening is selected afterward, the
manager sets that flag false and calls the correction again on every fine
step. Each fine-step CP217 entry still samples schedules, resets the current
thermal-mass multiplier to one, overwrites active-window `ZT`, and shifts the
three measured histories, while all three inference calculations skip.
Consequently the provisional internal-mass multiplier can be lost on the first
fine correction, infiltration and people outputs can remain stale, and measured
histories can advance `1 + N` times in one shortened Zone timestep. The demand
manager's HVAC resimulation route has no correction step and adds no CP217 call.

#### Failure, retry, and reset

CP217 owns no assertion, bounds check, configuration validation, status,
completion marker, diagnostic, catch, cleanup, transaction, or rollback.
Upstream HybridModel input normally rejects combined temperature unknowns and
requires measured-temperature and complete measured-supply schedule groups
where needed, but direct state construction can bypass those protections. The
entry tolerates a null measured-temperature schedule by sampling zero; the
people branch later dereferences that pointer. Both measured-supply branches
dereference supply temperature while only mass flow and humidity are nullable.
Psychrometric, schedule, indexing, logarithm, and the CP218 child remain
external dependencies.

An abnormal non-return retains the reached ordered prefix: entry samples and
reset, active-window `ZT`, infiltration fields, internal child aggregates and
diagnostics, or people fields. A failure before the tail leaves measured
histories unshifted; otherwise the three assignments themselves are ordered.
A same-state retry re-samples schedules, resets the current multiplier, consumes
already shifted histories, and can repeat CP218 accumulation and warnings, so
it is generally non-idempotent. Begin-environment initialization zeros only
`PreviousMeasuredZT1`, `PreviousMeasuredZT2`, and `PreviousMeasuredZT3`.
CP217 does not reset infiltration, people, running aggregate, or diagnostic
state, and skipped branches can preserve stale outputs.

#### C++ reach and corpus boundary

No C++ test calls `InverseModelTemperature` directly. One
`HybridModel.unit.cc` fixture reaches it through `correctZoneAirTemps` exactly
five times, all on day 1 inside a day-1-through-day-2 window, with one fully
mixed Zone, no AFN or duct contribution, one inference mode at a time, and
Zone-timestep history selected:

| Focused path | Asserted output |
|---|---|
| internal thermal mass, no measured supply, null measured-temperature schedule | current multiplier approximately 15.13 |
| infiltration, no measured supply, null measured-temperature schedule | air-change rate approximately 0.2444 |
| people count, no measured supply | people count zero |
| infiltration with measured supply | air-change rate approximately 0.49 |
| people count with measured supply | people count zero |

Those are the only five CP217 assertions. They do not inspect the `ZT`
override, raw schedule fields, measured-history tail, aggregate state,
diagnostics, thresholds, outside-window or false-history paths, nonzero people,
AFN/duct additions, combined flags, invalid/nonfinite inputs, partial failure,
retry, or reset. The separate direct
`HybridModel_processInverseModelMultpHMTest` belongs to CP218, not CP217.

All 57 active full-simulation `ManageSimulation` expressions declare no
`HybridModel:Zone` input or manual hybrid flag. Actual CP217 reach and output
oracle count are therefore zero. The 55 completing nonzero-Zone configurations
contain 81 exact-Zone correction records only as counterfactual topology; the
global flag blocks all of them, and the three active Space records are
independently excluded by `spaceNum == 0`.

#### Rust boundary

A crate-wide search finds no `HybridModel`, `InverseModelTemperature`,
`PreviousMeasuredZT`, `ZoneMeasuredTemperature`, `InfilOAAirChangeRateHM`,
`NumOccHM`, or `ZoneVolCapMultpSensHM` symbol. `HybridModel:Zone` is absent from
the typed compiler list, becomes `RawOnly`, has no partial capability rule, and
therefore run-blocks as unsupported. The typed Zone and People records own no
inverse dates, flags, measured/supply/people schedules, inferred results, or
hybrid output identities, and Rust has no typed Zone infiltration model.

The nearest runtime state owns ordinary current/average temperature, three
Zone/system temperature histories, adaptive counters, volume, forward air
capacity, gains, and coefficients. Its predictor assembles related forward
coefficients and the same third-order history combination, then solves
temperature forward with Rust-specific denominator and capacity guards.
Adaptive correction is Zone-only and down-interpolates three history slots.
None of those paths samples a measured Zone temperature, overwrites the solved
temperature for inverse inference, shifts measured histories, infers
infiltration/mass/people, or calls a multiplier aggregate child.

Typed People state supplies only design occupant count and a number schedule
for Watts-per-person and IdealLoads outdoor-air consumers; the runtime
convective/radiant internal-gain pass currently covers OtherEquipment rather
than inverse people estimation. The result store and output registry expose
ordinary Zone mean air temperature and humidity only. The four EnergyPlus
hybrid output names have no Rust registry, variable-coverage, case, or test
match. Existing forward-solver, adaptive-history, People parser, and ordinary
MAT/humidity output tests establish none of CP217.

CP217 adds no EnergyPlus source inventory, Rust target, code, mapped state,
test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 224 routines, split 58 `state_mapped` plus 166 `source_mapped`,
with 101 required; the heat-balance project list becomes 70.

### CP218 `processInverseModelMultpHM` source map

CP218 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.process_inverse_model_multp_hm`
immediately after `routine.inverse_model_temperature`. The heat-balance project
contract adds `process_inverse_model_multp_hm` after
`inverse_model_temperature` and before `update_final_surface_heat_balance`.
`processInverseModelMultpHM` is declared at
`ZoneTempPredictorCorrector.hh` lines 327-333 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4953-4991.

The helper accepts state, four mutable `Real64` references for the current
multiplier, accumulated sum, sample count, and average, plus a Zone identity by
value. It first obtains the named `Zone(zoneNum)` and
`zoneHeatBalance(zoneNum)` records. It owns no date, HybridModel mode, history,
warmup, sizing, or output gate of its own.

Two constants define the comparison boundaries:
`minHMMultValue = 1.0` and `maxHMMultValue = 30.0`. The exact ordered behavior
is:

| Input multiplier class | Current multiplier after limiting | Immediate/recurring diagnostic | Statistics |
|---|---|---|---|
| `< 1.0` | overwritten with exactly 1.0 | none | excluded |
| `== 1.0` | unchanged | none | excluded |
| `> 1.0 && <= 30.0` | unchanged | none | added |
| `> 30.0` | unchanged; never capped | over-limit diagnostics | added |

The strict lower branch precedes the strict upper branch. After it, every
multiplier strictly greater than one executes
`multSumHM += multiplierHM` and then increments the `Real64` count with
`countSumHM++`. A resulting or preexisting count greater than or equal to one
then overwrites `multAvgHM = multSumHM / countSumHM`; a count below one leaves
the incoming average unchanged. Consequently exactly 30 is accepted and added
without warning, while a value above 30 is warned but still added. The source
comment saying valid statistics are not higher than the maximum does not match
the executable predicate, which checks only `multiplierHM > 1.0`.

#### Diagnostic ownership

For the first multiplier above 30 while
`hmThermalMassMultErrIndex == 0`, CP218 emits one warning using the Zone name
and two continuation lines:

```text
Hybrid model thermal mass multiplier higher than the limit for {Zone name}
This means that the ratio of the zone air heat capacity for the current time step to the
zone air heat storage is higher than the maximum limit of 30.0.
```

Every above-30 occurrence then calls `ShowRecurringWarningErrorAtEnd` with the
per-Zone heat-balance-record index by reference and the recurring identity:

```text
Hybrid model thermal mass multiplier limit exceeded in zone {Zone name}
```

The first recurring call assigns a nonzero index; later calls suppress the
immediate three-line warning but continue updating the recurring record. CP218
passes no current multiplier to recurring min/max/sum reporting. Diagnostic
work completes before numerical aggregation, the routine returns no status,
and no high value is corrected.

#### Floating-point, malformed-state, and alias boundary

CP218 performs no finite, overflow, count-integrality, sign, bounds,
allocation, denominator, or distinct-reference validation. Under ordinary
masked IEEE behavior, negative infinity enters the lower clamp, positive
infinity is warned and accumulated into an infinite sum/average, and every
comparison with a NaN multiplier is false. A NaN multiplier is therefore not
clamped, warned, or added, although an existing count at least one still causes
the old sum/count to rewrite the average.

The count is not an integer type; fractional, negative, infinite, and NaN
incoming counts retain native comparison, increment, and division behavior.
The four mutable references may alias one another. A write to the multiplier,
sum, count, or average can then change a later operand or the final current
multiplier observed by the caller. The sole production call and direct fixture
use distinct storage, but CP218 itself establishes no such precondition.

#### Production ownership and cadence

The sole production call expression is inside CP217
`InverseModelTemperature` at `ZoneTempPredictorCorrector.cc` lines 4879-4880.
The caller passes local `MultpHM` and the Zone-owned
`ZoneVolCapMultpSensHMSum`, `ZoneVolCapMultpSensHMCountSum`, and
`ZoneVolCapMultpSensHMAverage`. CP218 runs only after CP217's internal-mass
branch has inferred a multiplier under this exact child gate:

```text
InternalThermalMassCalc_T
&& SumSysMCpT == 0
&& ZT != PreviousMeasuredZT1
&& UseZoneTimeStepHistory
```

All parent conditions also apply: exact Zone identity, the global HybridModel
gate, non-warmup and non-sizing correction, inclusive hybrid date window, and
the measured-temperature override. After CP218 returns, CP217 separately
writes the possibly lower-clamped local value to
`ZoneVolCapMultpSensHM`; CP218 does not write that current output field itself.

Normal HVAC starts a Zone timestep with Zone-timestep history selected, so an
eligible initial correction can contribute one sample. If adaptive shortening
is selected afterward, its fine corrections set global history false and do
not call CP218, even though CP217's surrounding prefix and history tail repeat.
Demand-manager HVAC resimulation has no correction step. An external or
same-state repeated CP217 call that again satisfies the gate contributes
another sample.

`ZoneVolCapMultpSensHMSum` and `ZoneVolCapMultpSensHMCountSum` default to zero,
while `ZoneVolCapMultpSensHMAverage` defaults to one. The average is consumed
for every Zone in the `Hybrid Model: Internal Thermal Mass` tabular subtable
when global `FlagHybridModel_TM` is true; the individual Zone flag controls
only its adjacent Yes/No column. The current multiplier is registered by the
upstream HybridModel input transaction. CP218 itself registers no output.

#### Failure, retry, and reset

An invalid or unallocated Zone or heat-balance record fails during the initial
record access before numerical mutation. An abnormal diagnostic non-return can
preserve an immediate-warning or recurring-index prefix while preventing the
following sum, count, and average updates. Later arithmetic or aliased-state
failure can preserve a sum or count prefix. CP218 owns no assertion, catch,
cleanup, completion marker, transaction, rollback, or recovery status.

Repeating an unchanged multiplier above one is non-idempotent: the same value
is added and counted again. Repeating an above-30 value also updates recurring
occurrence state again, while the already nonzero index normally suppresses
the immediate warning. A below-one call mutates its referenced value to one;
replaying that same storage remains excluded. Even an excluded sample can
rewrite the average when a prior count is at least one.

The Zone aggregate defaults are declared in `DataHeatBalance`, and the
per-Zone warning index defaults to zero in `ZoneSpaceHeatBalanceData`.
CP217 resets only the current multiplier on entry. Neither CP218 nor the
Zone/Space begin-environment initializer resets the sum, count, average, or
warning index, so they persist across ordinary environment boundaries. Clean
reset requires reconstruction or clear of their respective state owners.

#### C++ test and corpus boundary

`HybridModel_processInverseModelMultpHMTest` directly calls CP218 five times at
`ZoneTempPredictorCorrector.unit.cc` lines 1827, 1840, 1853, 1866, and 1886.
Its 20 numeric assertions, five warning-index assertions, and one error-stream
assertion establish these sequential states with tolerance 0.001:

| Input | Expected `(multiplier, sum, count, average)` | Warning evidence |
|---|---|---|
| 0.5 | `(1, 0, 0, 0)` | index remains zero |
| 1.0 | `(1, 0, 0, 0)` | index remains zero |
| 10.0 | `(10, 10, 1, 10)` | index remains zero |
| 50.0 | `(50, 60, 2, 30)` | index becomes nonzero; immediate warning text checked |
| 0.5 | `(1, 60, 2, 30)` | index remains nonzero |

The fixture uses local sum, count, and average references initialized to zero
rather than the production Zone fields, whose average default is one. The
50-input case directly proves the over-limit value is neither capped nor
excluded. The final low-input comment says no error message, but no post-call
error-stream assertion proves that statement.

The five `correctZoneAirTemps` calls in `HybridModel.unit.cc` provide only one
indirect CP218 execution: the first case enables internal thermal mass and
asserts the downstream current multiplier is approximately 15.13. The other
four cases disable that mode. No indirect assertion inspects the production
sum, count, average, warning index, tabular output, or diagnostics.

No C++ test covers exactly 30, nonfinite values, fractional or malformed
counts, reference aliasing, a repeated above-30 occurrence, recurring-summary
output/count, production-default average behavior, environment persistence,
invalid state, partial failure, or valid-sample retry. All 57 active
full-simulation `ManageSimulation` expressions configure no HybridModel, so
actual CP218 reach, hybrid output oracle count, and hybrid tabular oracle count
are zero.

#### Rust boundary

A crate-wide search finds no `processInverseModelMultpHM`, `HybridModel`,
`ZoneVolCapMultpSensHM`, `hmThermalMassMultErrIndex`, or exact HybridModel
thermal-mass output. `HybridModel:Zone` is absent from the typed compiler list,
becomes `RawOnly`, has no partial capability rule, and run-blocks with the
generic `UnsupportedObject` boundary.

Rust `ZoneHeatBalanceState` has no inferred multiplier, persistent sum, count,
average, or per-Zone recurring-warning index. Its nearest
`air_heat_capacity_j_per_k` is initialized and weather-refreshed from Zone
volume and psychrometric properties for the forward solver; it is not inverse
thermal-mass state. Generic Rust runtime diagnostics retain encountered
messages but provide no equivalent per-Zone recurring index or at-end
aggregation transaction.

Rust reports diagnostic physical air capacity/power-capacity values, not the
current HybridModel multiplier or its tabular average. It has no parser,
runtime consumer, report table, capability, focused test, or support evidence
for CP218. Forward capacity and generic diagnostic tests establish none of the
strict thresholds, uncapped high values, ordered references, warning lifecycle,
persistent aggregates, or caller cadence.

CP218 adds no EnergyPlus source inventory, Rust target, code, mapped state,
test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 225 routines, split 58 `state_mapped` plus 167 `source_mapped`,
with 102 required; the heat-balance project list becomes 71.

### CP219 `InverseModelHumidity` source map

CP219 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.inverse_model_humidity`
immediately after `routine.process_inverse_model_multp_hm`. The heat-balance
project contract adds `inverse_model_humidity` after
`process_inverse_model_multp_hm` and before
`update_final_surface_heat_balance`. `InverseModelHumidity` is declared at
`ZoneTempPredictorCorrector.hh` lines 335-343 and implemented at
`ZoneTempPredictorCorrector.cc` lines 4993-5131.

The routine accepts state and Zone identity plus latent gain, latent gain
excluding People, Zone mass flow, Zone moisture mass flow, vaporization
enthalpy, and moist-air density by value. It snapshots
`TimeStepSysSec`, then aliases the selected Zone, HybridModel Zone, and Zone
heat-balance record. It owns no Space identity or return status.

#### Entry, date, and history transaction

Every call unconditionally dereferences `measuredHumRatSched`, samples it once,
and writes `ZoneMeasuredHumidityRatio` before testing the date. The active
window is inclusive at both
`DayOfYear >= HybridStartDayOfYear` and
`DayOfYear <= HybridEndDayOfYear`. Inside that window CP219 first overwrites
the record's `airHumRat`, not `airHumRatTemp`, with the measured value. It then
tests two independent infiltration and People branches in source order; a
malformed direct state with both flags true runs both.

Each inverse calculation additionally requires the global
`state.dataHVACGlobal->UseZoneTimeStepHistory`. CP219 does not receive or read
the enclosing `correctAirTemp` history argument directly. After the date
block, every normally returning call performs this unconditional ordered
shift:

```text
PreviousMeasuredHumRat3 = PreviousMeasuredHumRat2
PreviousMeasuredHumRat2 = PreviousMeasuredHumRat1
PreviousMeasuredHumRat1 = ZoneMeasuredHumidityRatio
```

Consequently an outside-window call still samples the measured schedule and
advances all three histories, but does not overwrite `airHumRat` or inference
outputs. An active-window call with global system history selected still
performs the temporary measured write and history shift while skipping both
inverse calculations. The required measured schedule has no nullable fallback.

#### Humidity-derived infiltration

The infiltration branch requires
`InfiltrationCalc_H && UseZoneTimeStepHistory`. With system-supply parameters
enabled, CP219 unconditionally samples mass-flow then humidity-ratio schedules,
stores both raw values, and includes their product. Using `Wo = OutHumRat`, its
two coefficient paths are:

```text
with measured supply:
AA = Ms + VAMFL + EAMFL + CTMFL + SumHmARa
     + MixingMassFlowZone + MDotOA
BB = Ms*Ws + LatentGain/H2OHtOfVap
     + (VAMFL + CTMFL)*Wo + EAMFLxHumRat
     + SumHmARaW + MixingMassFlowXHumRat + MDotOA*Wo

without measured supply:
AA = VAMFL + EAMFL + CTMFL + SumHmARa
     + MixingMassFlowZone + MDotOA
BB = LatentGain/H2OHtOfVap
     + (VAMFL + CTMFL)*Wo + EAMFLxHumRat
     + SumHmARaW + MixingMassFlowXHumRat + MDotOA*Wo
```

Both paths deliberately omit `OAMFL` and ignore the passed
`ZoneMassFlowRate` and `MoistureMassFlowRate`. The storage and history terms
are:

```text
CC = RhoAir * Volume * ZoneVolCapMultpMoist / TimeStepSysSec
DD = 3*PreviousMeasuredHumRat1
     - 1.5*PreviousMeasuredHumRat2
     + (1/3)*PreviousMeasuredHumRat3
delta_HR = ZoneMeasuredHumidityRatio - OutHumRat
```

`RhoAir` is the caller's density at current Zone temperature and committed
humidity. CP219 separately recomputes outdoor `AirDensity` from outdoor
barometric pressure, the Zone outdoor dry-bulb temperature, and outdoor
humidity ratio. Raw mass flow remains zero unless the strict
`abs(delta_HR) > 1.0e-7` test passes. When it does:

```text
M_inf =
    (CC*DD + BB - ((11/6)*CC + AA)*ZoneMeasuredHumidityRatio)
    / delta_HR
```

The exact `1.0e-7` boundary therefore produces zero. CP219 converts the raw
mass flow to air changes per hour, applies the literal nested clamp
`max(0, min(10, ACH))`, reconstructs mass flow from the clamped ACH, outdoor
density, and Zone volume, then writes `MCPIHM` followed by
`InfilOAAirChangeRateHM`. Despite its name, `MCPIHM` receives kg/s mass flow,
not a heat-capacity product. The local `delta_HR` never updates the existing
Zone `delta_HumRat` field.

#### Humidity-derived People count

The People branch requires
`PeopleCountCalc_H && UseZoneTimeStepHistory`. It nullable-samples activity,
sensible-fraction, and radiant-fraction schedules in that order, storing zero
for an absent schedule. The stored sensible fraction is copied locally and
defaults to 0.6 only when it is less than or equal to zero. The radiant
fraction is stored but never consumed.

The local `ActivityLevel` was initialized to zero at routine entry and is
never assigned from either the sampled activity or
`ZonePeopleActivityLevel`. Its `<= 0` fallback therefore always chooses
130 W/person in ordinary execution, even when the raw activity schedule is
positive or nonfinite. This source anomaly is part of the CP219 boundary.

With measured supply enabled, the People branch again unconditionally samples
and stores supply mass flow then humidity ratio, and ignores both passed flow
arguments:

```text
AA = Ms + OAMFL + VAMFL + EAMFL + CTMFL + SumHmARa
     + MixingMassFlowZone + MDotOA
BB = Ms*Ws + LatentGainExceptPeople/H2OHtOfVap
     + (OAMFL + VAMFL + CTMFL)*Wo + EAMFLxHumRat
     + SumHmARaW + MixingMassFlowXHumRat + MDotOA*Wo
```

Without measured supply, it instead consumes both passed flows:

```text
AA = ZoneMassFlowRate
     + OAMFL + VAMFL + EAMFL + CTMFL + SumHmARa
     + MixingMassFlowZone + MDotOA
BB = LatentGainExceptPeople/H2OHtOfVap
     + (OAMFL + VAMFL + CTMFL)*Wo + EAMFLxHumRat
     + MoistureMassFlowRate + SumHmARaW
     + MixingMassFlowXHumRat + MDotOA*Wo
```

Using the same `CC` and `DD`, CP219 computes:

```text
LatentGainPeople =
    (((11/6)*CC + AA)*ZoneMeasuredHumidityRatio - BB - CC*DD)
    * H2OHtOfVap

denominator = 130 * (1 - FractionSensible)
UpperBound = max(0, LatentGain / denominator)
NumPeople =
    min(UpperBound, max(0, LatentGainPeople / denominator))
```

It rounds half-up to two decimal places with
`floor(NumPeople*100 + 0.5)/100`, then changes only values strictly below
0.05 to zero before writing `NumOccHM`. Exactly 0.05 survives. There is no
upper bound on the sampled sensible fraction and no denominator or finite
guard.

#### Caller ownership and cadence

The sole production call expression is line 4589 inside
`ZoneSpaceHeatBalanceData::correctHumRat`. Its parent gate requires exact
`spaceNum == 0`, global HybridModel enablement, at least one humidity
infiltration or People flag, and both non-warmup and non-sizing state. The
caller constructs total latent gain from record latent gain plus radiant-system
and pool terms. It constructs `LatentGainExceptPeople` from the analogous
record field only when People inference is enabled. Normal input validation
makes the humidity infiltration and People unknowns exclusive, but CP219
itself does not enforce that invariant.

The call occurs after the forward humidity solve, negative and saturation
clamps, and optional RoomAir AirflowNetwork control-node override. CP219's
active-window measured write targets only `airHumRat`; the following node
humidity, node enthalpy, and latent-sizing work all use the unchanged
`airHumRatTemp`. After `correctHumRat` returns, `correctAirTemp` line 4130
unconditionally replaces `airHumRat` with `airHumRatTemp` and computes RH.
Thus the measured write is normally transient in production, although it can
remain visible when an abnormal non-return prevents the outer overwrite. The
inverse equations and measured history still use
`ZoneMeasuredHumidityRatio`.

Normal HVAC begins with Zone-timestep history, so an eligible initial
correction can infer one output and shift history. If adaptive shortening is
selected, each fine correction re-enters CP219 while global history is false:
it resamples the schedule, repeats the active-window transient write, skips the
two calculations, and shifts measured history again. Demand-manager
resimulation has no correction step. Positive Spaces never enter CP219.

The HybridModel input transaction, not CP219, registers Zone-timestep Average
outputs named `Zone Infiltration Hybrid Model Air Change Rate`,
`Zone Infiltration Hybrid Model Mass Flow Rate`, and
`Zone Hybrid Model People Count`. CP219 registers no output itself.

#### Validation, failure, retry, and reset

CP219 has no Zone assertion, upper-bound or allocation check, date validation,
schedule-pointer check, finite check, direct diagnostic, status, catch,
cleanup, completion marker, transaction, rollback, or recovery path. It does
not validate timestep seconds, volume, moisture capacitance multiplier,
vaporization enthalpy, either density, pressure, humidity, history, sensible
fraction, or People denominator. The outdoor-density psychrometric dependency
can diagnose or fail, but CP219 owns no translation.

An abnormal non-return can retain this ordered prefix: sampled measured
humidity; active `airHumRat`; infiltration supply fields; infiltration mass
flow and ACH; People raw fields; People supply fields; People count; and only
then the three history writes. With both malformed mode flags, a People-path
failure can therefore preserve completed infiltration outputs. A same-state replay after a normal return resamples schedules, consumes
histories already shifted by the completed call, overwrites outputs again, and
shifts histories again, so it is generally non-idempotent. After an abnormal
non-return, retry observes only the ordered prefix that actually committed,
including any partial final-history writes.

The three measured histories are allocated as zero during one-time setup and
reset to zero at begin environment. CP219 does not reset its measured, supply,
raw People, infiltration, or People-count fields on entry, outside the date
window, with system history selected, or at begin environment. Skipped
inference outputs can therefore remain stale until another owner overwrites
them.

#### C++ test and corpus boundary

No C++ test directly calls `InverseModelHumidity`. Four calls to
`correctHumRat` inside `HybridModel_correctZoneAirTempsTest` reach it
indirectly:

| Case | Supply mode | Assertion |
|---|---|---|
| humidity infiltration, lines 242-269 | excluded | ACH approximately 0.5 |
| humidity People, lines 299-327 | excluded | People approximately 4 |
| humidity infiltration, lines 362-392 | included | ACH approximately 0.5 |
| humidity People, lines 432-467 | included | People approximately 4 |

All four use Day 1 inside the inclusive 1-2 window, exact Zone identity, and
Zone history. The final case supplies activity 120 and radiant fraction 0.3,
but its loose downstream People assertion does not isolate the unused activity
or radiant behavior. No assertion covers the measured sample, raw fields,
history shift, transient/final humidity distinction, mass-flow output, exact
thresholds, clamp edges, rounding, stale output, false-history or
outside-window paths, combined flags, invalid/nonfinite state, failure, retry,
reset, or output registration.

All 57 active full-simulation `ManageSimulation` expressions configure no
`HybridModel:Zone` and no manual HybridModel flag. Actual CP219 reach and
hybrid output-oracle count are therefore zero.

#### Rust boundary

A crate-wide search finds no `InverseModelHumidity`,
`ZoneMeasuredHumidityRatio`, measured-humidity history, humidity inverse flags,
`MCPIHM`, `InfilOAAirChangeRateHM`, `NumOccHM`, or exact HybridModel humidity
output identity. `HybridModel:Zone` is absent from the typed compiler list,
becomes `RawOnly`, has no partial capability rule, and run-blocks through the
generic `UnsupportedObject` boundary.

Rust `ZoneHeatBalanceState` owns forward current/average humidity and two
three-slot solved histories, not a measured schedule binding or measured
history. Its main humidity correction reconstructs forward humidity, applies
project-specific clamps/fallbacks, and shifts no measured state. The separate
no-outdoor-air ThirdOrder IdealLoads moisture helper is also a forward,
single-Zone purchased-air subset, not inverse inference.

Rust has no typed infiltration record. Typed `People` retains design count and
an occupancy schedule only; it has no activity, sensible-fraction,
radiant-fraction, or inferred-count state. The nearest ordinary
`Zone Mean Air Humidity Ratio` report exposes forward state, while all three
HybridModel output names have no registry, case, or test match. Existing
forward humidity, IdealLoads, People, and generic diagnostic tests establish
none of CP219.

CP219 adds no EnergyPlus source inventory, Rust target, code, mapped state,
test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 226 routines, split 58 `state_mapped` plus 168 `source_mapped`,
with 103 required; the heat-balance project list becomes 72.

### CP220 `ZoneSpaceHeatBalanceData::calcZoneOrSpaceSums` source map

CP220 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_space_heat_balance_calc_zone_or_space_sums`
immediately after `routine.inverse_model_humidity`. The heat-balance project
contract adds `zone_space_heat_balance_calc_zone_or_space_sums` after
`inverse_model_humidity` and before `update_final_surface_heat_balance`. The
member is declared at `ZoneTempPredictorCorrector.hh` lines 226-230 and
implemented at `ZoneTempPredictorCorrector.cc` lines 5133-5281.

The receiver is the selected Zone or Space heat-balance record. The required
positive `zoneNum`, optional `spaceNum`, and `CorrectorFlag` determine which
global topology is read, but the member returns no status. Its output is an
ordered in-place reconstruction of internal, non-system-air, system-air, and
surface-convection sums.

#### Ordered coefficient transaction

After a debug-only positive-Zone assertion, CP220 writes these five fields to
zero in exact source order:

```text
SumHA
SumHATsurf
SumHATref
SumSysMCp
SumSysMCpT
```

It does not first clear `SumIntGain`, `SumMCp`, or `SumMCpT`. When
`spaceNum == 0`, it assigns `SumIntGain` from all Zone internal convection
gains. Every nonzero `spaceNum`, including a malformed negative value, instead
takes the Space internal-gain path. It then adds the parent Zone's complete
`SumConvHTRadSys + SumConvPool` to either record. Every Space-path call
therefore receives the full Zone radiant-system and pool convection terms
rather than a Space allocation.

After a second positive-Zone assertion and parent-Zone alias, the
`NoHeatToReturnAir` branch likewise uses the Zone return-gain helper only for
exact zero and the Space helper for every nonzero `spaceNum`, with return-node
argument zero. The later CP220 control and allocation gates use `spaceNum > 0`; the
derived Space virtual dependency separately asserts a positive identity. CP220 then overwrites the two
non-system airflow coefficients:

```text
SumMCp =
    MCPI + MCPV + MCPM + MCPE + MCPC + MDotCPOA

SumMCpT =
    MCPTI + MCPTV + MCPTM + MCPTE + MCPTC
    + MDotCPOA * Zone.OutDryBulbTemp
```

When AirflowNetwork control is always multizone, or is
multizone-with-distribution-only-during-fan-operation while the AFN fan is
active, these are replaced, not incremented:

```text
SumMCp =
    exchangeData(zoneNum).SumMCp + SumMVCp + SumMMCp

SumMCpT =
    exchangeData(zoneNum).SumMCpT + SumMVCpT + SumMMCpT
```

A Space receiver still consumes the parent Zone AFN exchange record. Neither
the ordinary nor AFN non-system coefficients are volume-allocated later.

CP220 evaluates
`spaceNum > 0 && spaceEquipConfig(spaceNum).IsControlled` before testing
`CorrectorFlag`; a false predictor call with a positive Space identity still
requires allocated Space equipment configuration. `CorrectorFlag == false`
leaves both system-air sums at their entry zeros, but all preceding work, the
later Space allocation gate, and the virtual surface-convection dependency
still run.

With `CorrectorFlag == true`, primary system-air work chooses one mutually
exclusive parent-Zone branch:

1. A controlled parent Zone uses the controlled Space equipment configuration
   only when the selected Space is itself controlled; otherwise it uses the
   parent Zone configuration. It visits inlet nodes in stored 1-based order
   and accumulates mass flow times `PsyCpAirFnW(this->airHumRat)`, then that
   product times node temperature. Node humidity ratio is not used.
2. A return-plenum Zone visits its stored inlet nodes, then stored air
   distribution unit identities. Enabled upstream and downstream leakage adds
   the respective leak flow and inlet or outlet temperature with the
   receiver's humidity-ratio heat capacity; there is no positive-flow guard.
3. A supply-plenum Zone consumes its single inlet node with the same
   receiver-humidity heat capacity.
4. Any other Zone contributes no primary system flow.

The independent parallel-PIU tail then forms
`Zone.Multiplier * Zone.ListMultiplier`. If
`leakageParallelPIUNums` is nonempty, the loop ignores the identities stored in
that container and instead reads global PIU ordinals one through the
container's size. This literal source anomaly is part of CP220. Only positive
leak flow contributes. A positive parent `SystemZoneNodeNumber` routes the
leak into system sums; otherwise it adds only the leak contribution divided by the Zone multiplier
product to the otherwise unscaled non-system sums. CP220 finally divides both complete
system sums by that product without a zero, sign, overflow, or finite guard.

For a positive Space that is not itself controlled, CP220 then multiplies only
`SumSysMCp` and `SumSysMCpT` by `space.fracZoneVolume`. A controlled Space is
not volume-scaled. Internal gains, ordinary/AFN non-system coefficients, and
the later surface sums are never scaled here. A Space marked controlled while
its parent Zone is not controlled also cannot enter the controlled parent
branch.

#### Virtual surface dependency and commit

CP220 always calls the receiver's virtual
`calcSumHAT(state, zoneNum, spaceNum)` after coefficient assembly. Only after
that call returns does it add the returned internal-gain term and assign
`SumHA`, `SumHATsurf`, and `SumHATref` in that order.

The immediately following definitions belong to CP221 and later checkpoints,
but their dependency behavior constrains CP220. The Zone override at
`ZoneTempPredictorCorrector.cc` lines 5283-5298 visits every stored
`Zone.spaceIndexes` identity in container order without testing
`doSpaceHeatBalance`, calls the Space override, and folds all four returned
fields. A Zone CP220 call therefore traverses all of its Space surfaces even
when no explicit Space record calculation is enabled; a later explicit Space
CP220 call can traverse those surfaces again.

The Space override's `ZoneSupplyAirTemp` reference branch always reads the
parent `zoneHeatBalance(zoneNum).SumSysMCp` and `SumSysMCpT`, not the selected
Space record's system sums. Production Zone-first traversal makes the current
Zone correction coefficients visible. Predictor mode has just zeroed them, so
that branch falls back to `SumHA`. The child also owns Window convection and
report mutations. In the airflow-Window plus `NoHeatToReturnAir` path it can
add return heat to `SurfWinHeatGain` and update gain/loss/transfer reports.
Those external Surface-owner effects can repeat across Zone-plus-Space
traversal or replay even though the CP220 record fields are overwritten.

#### Caller ownership and cadence

There are exactly two production call expressions. Line 3175 inside
`ZoneSpaceHeatBalanceData::predictSystemLoad` passes literal false after
temperature/history update and `AirPowerCap`, before hybrid except-People
state, coefficient writes, and load solving. Line 3918 inside
`ZoneSpaceHeatBalanceData::correctAirTemp` passes literal true after selected
history and capacitance work and exact-Zone RoomAir management, before hybrid
except-People state, coefficient writes, temperature solving, node and demand
writes, and humidity correction.

The prediction wrapper visits Zones first and stored Spaces under the aggregate
`doSpaceHeatBalance` flag. The correction wrapper visits every Zone first and
only active, non-sizing simulation Spaces. Normal HVAC performs an initial
prediction and correction; adaptive shortening can repeat both for each fine
step. Demand-manager resimulation adds a prediction without a matching
correction. `CorrectorFlag` therefore gates only the system/PIU assembly, not
internal gains, non-system/AFN replacement, Space configuration lookup,
virtual surface work, or its external effects.

#### Validation, failure, retry, and reset

Besides its duplicated debug positive-Zone assertions, CP220 has no upper
bound, Space sign/membership/record-kind, equipment, node, plenum, ADU, PIU,
AFN topology, multiplier, volume-fraction, or finite validation. It owns no
diagnostic, status, catch, cleanup, completion marker, transaction, rollback,
or recovery path. The virtual child can assert, diagnose, or fail, but CP220 does not translate
that result. `PsyCpAirFnW` performs no validation or diagnostic; it only
updates its process-static last-input/result cache.

An abnormal non-return preserves the exact committed prefix. The five entry
zeros can coexist with an old `SumIntGain`, `SumMCp`, or `SumMCpT` if failure
prevents those later assignments. Later failures can retain assigned gains,
ordinary or AFN coefficients, partial system/plenum/PIU sums, multiplier
division, or Space allocation. If `calcSumHAT` does not return, the four local
results are not committed: the record surface sums remain at their entry
zeros, while earlier coefficient writes and completed child Surface effects
remain. A later interruption can expose only an ordered prefix of the final
gain/HA/HAT assignments.

On stable side-effect-free dependencies, a completed replay deterministically
reconstructs the receiver sums. It is not generally idempotent because
`calcSumHAT` can increment external Window report state; retry after abnormal
exit observes and can repeat only the child prefix that actually committed.
`beginEnvironmentInit` does not reset any of these CP220 sums. Predictor/
corrector data `clear_state` placement-news the records, but does not reset the
process-static `PsyCpAirFnW` cache; that cache changes evaluation reuse, not the
sequential numerical result. A clean recovery also requires coordinated
restoration of internal-gain, AFN, equipment, node, Surface-report, and
topology owners.

#### C++ test and corpus boundary

`ZoneTempPredictorCorrector_calcZoneOrSpaceSums_SurfConvectionTest` is the sole
direct fixture. It makes five exact-Zone receiver calls and has 12 assertions:
two true calls check three `SumHA`/`SumHATsurf`/`SumHATref` supply-reference
and fallback results; a third true call checks two system sums for inlet flows
0.1 and 0.2 kg/s; one false call checks both system sums are zero; and a final
true call adds PIU ordinal 1 leakage and checks two system sums. Its controlled
Zone has multiplier one, two inlets, one stored Space, and three non-Window
surfaces spanning the three reference-air modes. The fixture's inlet-node
humidity differs from the receiver humidity used in expected system sums, so
it indirectly reflects the latter ownership, but the one-element PIU list
cannot expose the identity bug.

Five indirect `correctZoneAirTemps` calls in the HybridModel fixture each pass
through one true Zone CP220 call, but assert only hybrid multiplier,
infiltration, and People outputs. Two predictor fixtures make 16 wrapper calls
with zero Zones and reach CP220 zero times. No focused test directly calls a
Space receiver or isolates internal/return-air gains, ordinary non-system
flows, AFN replacement, controlled-Space behavior, volume allocation, either
plenum, PIU identity, multiplier edges, Window side effects, malformed state,
failure, retry, or reset.

Of 57 active full-simulation `ManageSimulation` expressions, one expected EMS
fatal stops before CP220 and one Weather fixture has zero Zones. The remaining
55 configurations contain 81 Zone identities. A static single prediction-pass
census is 81 Zones plus 24 eligible Spaces, or 105 calls. A static single
correction-pass census is 81 Zones plus three active simulation Spaces, or 84
calls. Their combined 189-call configuration census is not a runtime total;
warmup, run-period, adaptive, and demand-resimulation cadence makes actual
execution larger.

The correction topology contains 55 controlled Zones, 26 uncontrolled Zones,
and three uncontrolled Spaces, with zero controlled Space, return-plenum,
supply-plenum, or parallel-PIU identity. AFN multizone replacement is
potentially reachable for five Zone identities, but no corpus assertion
isolates its coefficients. Dynamic `NoHeatToReturnAir`, exact
surface-reference and Window subbranches, duplicate child effects, and
adaptive/demand multiplicity are not established by a corpus oracle.

#### Rust boundary

A crate-wide search finds no `calcZoneOrSpaceSums`,
`calc_zone_or_space_sums`, `calcSumHAT`, or direct test. The nearest
`zone_surface_convection_sums_for_indices` visits retained opaque surfaces for
one Zone and computes only hA and hA times inside temperature, with
`SumHATref` hard-coded to zero. It has no additional returned internal gain,
Space or Window topology, frame/divider work, or reference-air branches.

`ZoneHeatBalanceState` stores adjacent HA/HAT fields and four airflow
coefficient fields. The airflow fields are only zero-initialized in production;
the crate has no source-equivalent assembly writer, although isolated tests can
supply them manually to coefficient algebra. Runtime surface indexes preserve
Zone identity only, discard the compiler Surface's Space identity, and own no
Space heat-balance record or `fracZoneVolume` allocation. Typed Space volume
and nominal Zone-control metadata are not consumed by heat-balance sum
assembly.

Rust internal gains cover a direct-Zone OtherEquipment convection subset.
People activity/latent ownership, return-air gains, radiant-system and pool
terms, ordinary infiltration/ventilation/mixing coefficients, AFN exchange,
controlled inlet assembly, plenums, ADUs, parallel PIUs, and their state are
absent or run-blocking. Equipment connection/node projection remains legacy
diagnostic state rather than a writer of these sums. Runtime reports expose
internal and opaque-surface convection, while Outdoor Air Transfer is a
literal zero vector; an offline comparator can reconstruct selected HA/HAT
series, but it is not a runtime CP220 implementation.

The official one-Zone candidate has one Zone, six opaque surfaces, no authored
`Space` object, and no airflow, AFN, AirLoop, or ZoneHVAC topology. Rust has no
runtime Space heat-balance topology, and equal/opposite OtherEquipment inputs
make relevant internal and outdoor-transfer evidence zero-only. Existing variable-level limited conformance therefore
does not establish CP220.

CP220 adds no EnergyPlus source inventory beyond this routine row, Rust target,
code, mapped state, test, support, capability, output implementation,
comparator, manifest, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 227 routines, split 58 `state_mapped` plus
169 `source_mapped`, with 104 required; the heat-balance project list becomes
73.

### CP221 `ZoneHeatBalanceData::calcSumHAT` source map

CP221 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_heat_balance_calc_sum_hat`
immediately after
`routine.zone_space_heat_balance_calc_zone_or_space_sums`. The heat-balance
project contract adds `zone_heat_balance_calc_sum_hat` after
`zone_space_heat_balance_calc_zone_or_space_sums` and before
`update_final_surface_heat_balance`. The Zone override is declared at
`ZoneTempPredictorCorrector.hh` line 254 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5283-5298. The four-field
`SumHATOutput` return type is declared at header lines 93-100.

#### Local return and ordered Space fold

CP221 first aliases the predictor/corrector owner, then performs debug-only
assertions in this exact order:

```text
zoneNum > 0
spaceNum == 0
```

The second assertion enforces Zone receiver convention only in debug builds.
With assertions compiled out, `spaceNum` is completely ignored. CP221 then
default-constructs `zoneResults`; member initializers set `sumIntGain`,
`sumHA`, `sumHATsurf`, and `sumHATref` to exact zero.

The routine visits every integer in
`state.dataHeatBal->Zone(zoneNum).spaceIndexes` in stored container order. For
each `zoneSpaceNum`, it default-constructs a temporary result, replaces that
temporary with:

```text
spaceHeatBalance(zoneSpaceNum).calcSumHAT(
    state,
    zoneNum,
    zoneSpaceNum
)
```

and, only after the child returns, accumulates these fields in exact order:

```text
zoneResults.sumIntGain += spaceResults.sumIntGain
zoneResults.sumHA      += spaceResults.sumHA
zoneResults.sumHATsurf += spaceResults.sumHATsurf
zoneResults.sumHATref  += spaceResults.sumHATref
```

It returns the local aggregate by value. An empty stored Space list therefore
returns four exact zeros. CP221 does not test `doSpaceHeatBalance`, sort,
filter, deduplicate, or verify that a stored identity belongs to the supplied
Zone. Duplicate identities call the child and add its result repeatedly.
Floating-point reduction order and child side effects follow the stored order.

CP221 writes no Zone or Space heat-balance record itself, performs no volume or
multiplier allocation, and registers no output. CP220 owns the later commit of
the returned fields into its receiver.

#### Child and caller ownership

The child expression dispatches the immediately following
`SpaceHeatBalanceData::calcSumHAT` definition, reserved for CP222. CP221 passes
the original parent Zone identity and the same stored Space identity for both
record selection and the child argument. CP222 owns all Surface formulas,
reference-air branches, Window report mutations, and its possible fatal path;
CP221 owns only child ordering and the four-field fold.

The sole production ingress is CP220's virtual
`this->calcSumHAT(state, zoneNum, spaceNum)` expression at line 5276. A
`ZoneHeatBalanceData` receiver dispatches to CP221; a
`SpaceHeatBalanceData` receiver dispatches directly to CP222 and never enters
CP221. Thus each Zone CP220 call invokes CP221 exactly once in both the false
prediction path at line 3175 and the true correction path at line 3918.

CP221 inherits CP220's full cadence: initial HVAC prediction/correction,
adaptive fine-step repetitions of both, and demand resimulation's additional
prediction without a matching correction. It always traverses all stored
Spaces, even when the outer prediction/correction wrappers do not schedule
explicit Space records.

#### Validation, failure, retry, and reset

Besides the two debug assertions, CP221 has no positive-Zone release check,
upper bound, allocation, Space identity, membership, duplicate, count,
finite-value, or topology validation. It owns no diagnostic, status, catch,
cleanup, completion marker, transaction, rollback, or recovery path. Invalid
Zone or Space indexes can fail through unchecked container access. A returned
nonfinite child field is accumulated without rejection or normalization.

All four aggregate fields are local until normal return. If a child does not
return, none of that failing child's values are added, the partial Zone
aggregate is not returned, and later children are skipped. Completed or
failing CP222 children can nevertheless retain external Window report writes
or diagnostics. The enclosing CP220 record surface fields remain at their
entry zeros because its post-call commit has not executed.

A retry starts a fresh zero aggregate and replays children from the first
stored identity; it does not resume after the failed child or roll back
external prefixes. Stable side-effect-free children yield the same ordered
floating-point result, but the CP222 airflow-Window
`SurfWinHeatGain +=` path makes general replay non-idempotent. CP221 owns no
persistent state or reset hook. `beginEnvironmentInit` has nothing local to
reset; clean recovery requires coordinated restoration of the Zone/Space
topology, predictor records, Surface reports, and every CP222 dependency.

#### C++ test and corpus boundary

No C++ test directly calls either `calcSumHAT` override. The sole focused
indirect fixture,
`ZoneTempPredictorCorrector_calcZoneOrSpaceSums_SurfConvectionTest`, calls
CP220 on one Zone five times. Each call dispatches CP221 and its one stored
Space child. Only the first two calls assert CP221-derived surface results:
three `SumHA`/`SumHATsurf`/`SumHATref` values each, or six aggregate
assertions. The remaining three calls assert only CP220 system sums.

That fixture has one Space, three non-Window surfaces, and one surface for each
mean-air, adjacent-air, and supply-air reference mode. It does not isolate
`sumIntGain`, an empty, multiple, duplicate, reordered, or foreign Space list,
nonfinite child output, invalid identity, child failure, partial effects,
retry, or reset. Five HybridModel correction calls also pass through one Zone
CP221 each, but their child surface ranges are empty and every assertion
targets hybrid state. Two predictor fixtures contain zero Zones and never
reach CP221.

Of 57 active full-simulation `ManageSimulation` expressions, one expected EMS
fatal stops before CP221 and one Weather fixture has zero Zones. The remaining
55 configurations contain 81 Zone identities and 99 stored Space identities.
The 99 comprise one baseline Space per Zone, 16 additional identities across
eight one-Zone/three-Space sizing configurations, one additional identity in
the two-Zone/three-Space `HeatBalanceAirManager_GetMixingAndCrossMixing`
configuration, and one additional identity in the separate two-Zone/three-Space
`Standard621SimplifiedProcedure::SimplifiedProcedureTest3` configuration.

A static single prediction pass is therefore 81 CP221 calls dispatching 99
nested CP222 children. A static single correction pass has the same CP221 and
nested-child counts because CP221 ignores the explicit-Space scheduling gate.
The combined configuration census is 162 CP221 calls and 198 nested CP222
calls, not a runtime total. The outer CP220 wrappers separately schedule 24
explicit prediction Space records and three explicit correction Space records,
which go directly to CP222 and can repeat children already visited by CP221.
Warmup, run-period, adaptive, and demand-resimulation cadence makes actual
execution larger. No full-simulation oracle isolates the Zone fold, its stored
order, duplicate behavior, child side effects, or failure transaction.

#### Rust boundary

A crate-wide search finds no `calcSumHAT`, `calc_sum_hat`, `SumHATOutput`, or
four-field Zone/Space aggregation result. The nearest
`zone_surface_convection_sums_for_indices` directly folds runtime Surface
indexes for one Zone and returns only `(HA, HATsurf, HATref)`. It computes hA
and hA times inside-face temperature for retained opaque surfaces, hard-codes
HATref to zero, and silently skips an out-of-range index through `filter_map`.
It returns no `sumIntGain` and performs no Zone-to-Space child dispatch.

That helper is called during initialization, ordinary Zone correction, the
local adaptive correction helper, and predictor-local Analytical/ThirdOrder
work. `ZoneHeatBalanceState` stores adjacent HA/HAT fields and separate
internal gain, but there is no Space heat-balance record or result. Compiler
state can retain ordered Zone/Space relationships and a Surface Space
identity; the runtime boundary run-blocks authored and remainder Space
topology, drops Surface Space identity, and indexes heat-balance surfaces by
Zone only. Its direct Surface fold therefore does not implement CP221's stored
Space order, duplicate child calls, four-field addition order, or side-effect
and failure behavior.

Rust `part03` checks one-Zone initial HA/HATsurf/HATref values, `part04` checks
a one-Zone direct opaque-surface sum and coefficients, and `part05` seeds
fields to test downstream reporting. Structural tests establish helper
ownership only. None constructs or folds Space child results, and the offline
CLI coefficient-sum reconstruction is comparator analysis rather than a
runtime CP221 implementation.

The official one-Zone candidate has one Zone, six opaque surfaces, no authored
Space, Window, or HVAC object. The compiler creates one default Space, but
runtime discards that identity. Its 8760-sample Zone surface-convection-rate
artifact passes the existing tolerance with maximum absolute difference
0.085845581243 W and RMSE 0.005357748923 W. Equal and opposite OtherEquipment
inputs also make relevant internal convection evidence net zero. This is
bounded single-partition opaque-Surface numerical evidence, not proof of
CP221's four-field Space aggregation, ordering, duplication, Window effects,
or `sumIntGain` fold.

CP221 adds no Rust target, code, mapped state, test, support, capability,
output implementation, comparator, manifest, numerical, performance, or
conformance promotion. The inventory becomes 32 algorithms and 228 routines,
split 58 `state_mapped` plus 170 `source_mapped`, with 105 required; the
heat-balance project list becomes 74.

CP222 next maps `SpaceHeatBalanceData::calcSumHAT`, declared at
`ZoneTempPredictorCorrector.hh` line 259 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5300-5413.

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
