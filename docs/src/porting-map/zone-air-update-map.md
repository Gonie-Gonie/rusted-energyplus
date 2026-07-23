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
| Zone/Space heat-balance surface result family | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneHeatBalanceData::calcSumHAT` and `SpaceHeatBalanceData::calcSumHAT` | direct Zone opaque-Surface index fold returning only HA/HATsurf/HATref=0 | CP221 maps the Zone stored-Space child fold; CP222 expands the same required source-mapped routine to the Space inclusive Surface range, Window/report terms, reference-air dispatch, and failure effects. No exact Rust Space/Window/result or execution parity. |
| Zone/Space component-load reporting | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::CalcZoneComponentLoadSums` | separate Zone-only internal-gain, opaque-Surface convection, and air-storage helpers plus a hard-coded zero outdoor-transfer report | CP223 required source-mapped correction-only ten-field reporting update sequence with parent-Zone topology, whole-Zone Surface rewalks for Zone and Space reports, ADU and imbalance-warning side effects, and no complete Rust reporting parity |
| thermostat presence verification | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::VerifyThermostatInZone` | bounded direct-Zone DualSetpoint records, ZoneId graph/IdealLoads lookups, and planning metadata only | CP224 required source-mapped shared-latch exact-name sizing predicate; no exact Rust lazy-input, sizing-caller, lookup, or failure parity |
| thermostat-to-controlled-Zone verification | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::VerifyControlledZoneForThermostat` | normalized raw controlled marker, independent typed ZoneId thermostat/equipment records, and an IdealLoads-only dispatch validator | CP225 required source-mapped full-arena exact-name predicate plus ordinary/comfort caller latch and fatal lifecycle; no exact Rust helper, cross-family validation, or failure parity |
| Zone-temperature oscillation detection | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::DetectOscillatingZoneTemp` | adjacent three-slot MAT histories, adaptive `0.3 C` step count, IdealLoads-only occupancy/deadband concepts, and hourly MAT/debug output | CP226 required source-mapped one-time activation plus zero-seeded four-slot strict `0.15 C` detector, Zone/Facility duration outputs, annual/perflog aggregation, and system-step lifecycle; no exact Rust helper, state, caller, output, or test |
| operative-temperature air-setpoint conversion | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::AdjustAirSetPointsforOpTempCntrl` | bounded direct-Zone DualSetpoint graph and raw-schedule IdealLoads output only | CP227 required source-mapped global/per-record guards plus fixed/scheduled fraction and Zone-MRT inverse, caller overwrite order, resimulation, IEEE, and replay lifecycle; operative input run-blocks and no exact Rust state, helper, live caller, or test exists |
| adaptive-comfort operative setpoint selection | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::AdjustOperativeSetPointsforAdapComfort` | weather-run-period day-of-year state plus a bounded direct-Zone DualSetpoint graph and schedule references only | CP228 required source-mapped seven-model daily/design-day selector, toward-zero integer baseline/lower-bound and exact `-1` fallback, pre-CP227 snapshot, cadence, and failure lifecycle; operative input run-blocks and no exact Rust adaptive state, helper, output, caller, or test exists |
| thermal-comfort Zone air setpoint calculation | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::CalcZoneAirComfortSetPoints` | ordinary DualSetpoint thermostat and narrow People count/schedule state only | CP229 required source-mapped first-use comfort initialization, PMV control dispatch, four People averaging modes, dry-bulb assignment/clamps, ordinary-to-comfort overwrite and final EMS precedence, cross-Zone accumulator anomalies, diagnostics, and retry lifecycle; comfort objects run-block and no exact Rust Fanger state, inverse child, outputs, caller, or test exists |
| thermal-comfort PMV-to-dry-bulb inversion | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::GetComfortSetPoints` | narrow People count/schedule state plus an unrelated private IdealLoads psychrometric bisection | CP230 required source-mapped strict endpoint dispatch and configurable `SolveRoot` inverse with duplicated impure Fanger trials, shared diagnostics, stale-output/report asymmetries, and failure/retry lifecycle; comfort objects run-block and no exact Rust PMV/Fanger state, generic root solver, live caller, or composed test exists |
| temperature-and-humidity cooling-setpoint overcool | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::AdjustCoolingSetPointforTempAndHumidityControl` | ordinary DualSetpoint graph plus separate typed Humidistat, psychrometric RH, and IdealLoads moisture-demand paths only | CP231 required source-mapped pre-guard aliases, global/exact-None guards, constant/scheduled range and positive gap/RH-ratio caps, high-only mutation, mixed-record null dependency, parent precedence, and replay lifecycle; the modifier run-blocks and no exact Rust input, setpoint state, helper, caller, or test exists |
| EMS air-temperature setpoint override | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::OverrideAirSetPointsforEMSCntrl` | ordinary direct-Zone DualSetpoint graph, raw-schedule IdealLoads diagnostics, and EMS stage metadata only | CP232 required source-mapped ordinary-then-comfort traversal, heating-before-cooling field matrix, live control-type dispatch, shared-Zone precedence, actuator binding and unit anomaly, downstream cutout replacement, and replay/reset lifecycle; all EMS input run-blocks and no exact Rust actuator state, mutable setpoint triple, comfort control, helper, caller, or active test exists |
| thermostat-setpoint predefined LEED table | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::FillPredefinedTableOnThermostatSetpoints` | normalized DualSetpoint graph, calendar-aware schedule series, and separate constant-schedule IdealLoads diagnostics only | CP233 required source-mapped four-family first-schedule-ID-wins traversal, winter/summer Wednesday samples and counts, base/synthetic row keys, append-only predefined cells, final-report cadence, and failure/retry lifecycle; reporting input stays ignored and no exact Rust arena, seasonal query, table store, helper, caller, or test exists |
| thermostat-schedule predefined System Summary table | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::FillPredefinedTableOnThermostatSchedules` | direct-Zone DualSetpoint graph and IdealLoads schedule resolution only | CP234 required source-mapped stored ordinary-Zone traversal, nonempty-name slot selection, tuple sort, independently filtered string joins, four-to-six append-only cells, final-report cadence, and failure/retry/reset lifecycle; reporting input stays ignored and no complete Rust arena, predefined table store, helper, caller, serializer, or comparator exists |
| Zone/Space predictor temperature-history preparation | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::updateTemperatures` | Zone-only three-slot adaptive temperature/humidity histories and helper | CP235 required source-mapped unconditional four-slot working-history selection plus shortened Zone/Space node rollback and count-change RoomAir interpolation orchestration; no exact Rust Space/node/thermostat/enthalpy/RoomAir topology, source cadence, wrapper, or test exists |
| Zone/Space predicted sensible system load | `src/EnergyPlus/ZoneTempPredictorCorrector.cc::ZoneSpaceHeatBalanceData::calcPredictedSystemLoad` | adjacent guarded Zone-only coefficient helpers, bounded DualSetpoint graph, node setpoint storage, oracle-fed IdealLoads demand, and Zone multipliers only | CP236 required source-mapped five-way/three-algorithm load selection plus RAFN/ITE asymmetries, staged override, shared Zone writes, and selected Zone/Space reporting; no exact Rust dispatcher, Space binding, live demand synthesis, or composed test exists |
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
`SimplifiedProcedureTest3` configuration in `Standard621SimplifiedProcedure.unit.cc`.

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

### CP222 `SpaceHeatBalanceData::calcSumHAT` source map

CP222 expands the existing required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.zone_heat_balance_calc_sum_hat`
mapping to the independent Space override. It adds no second routine row or
heat-balance project-contract item because both C++ definitions share the
unqualified source identifier `calcSumHAT`. The project list retains
`zone_heat_balance_calc_sum_hat` before
`update_final_surface_heat_balance`. The Space override is declared at
`ZoneTempPredictorCorrector.hh` line 259 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5300-5413. The four-field
`SumHATOutput` return type remains the header lines 93-100 boundary.

#### Entry and unfiltered inclusive Surface range

CP222 performs debug-only assertions in this exact order:

```text
zoneNum > 0
spaceNum > 0
```

It then aliases the predictor/corrector owner, parent `Zone(zoneNum)`, and
selected `space(spaceNum)`, and default-constructs `results`. Member
initializers set `sumIntGain`, `sumHA`, `sumHATsurf`, and `sumHATref` to exact
zero. The body never reads or writes receiver `this`; production constructs
align the receiver with `spaceNum`, but CP222 does not enforce that alignment.

The routine walks every integer from the selected Space's `HTSurfaceFirst`
through `HTSurfaceLast`, inclusive and ascending. An inverted range returns the
four exact zeros. CP222 does not test `HeatTransSurf`, Surface class before
entry, stored count, Zone or Space membership, bounds ownership, or overlap
with another Space range. A foreign identity inside malformed numeric bounds
is processed at its integer position; repeated visits arise only from
overlapping ranges or later calls, not within one ascending range.

For each identity, CP222 initializes local `HA` to zero and local `Area` from
`Surface(SurfNum).Area`. For a Window this is the glazing area. Every Surface,
including a non-Window and a `GlassDoor`, eventually receives the base terms:

```text
HA                 += SurfHConvInt * Area
results.sumHATsurf += SurfHConvInt * Area * SurfTempInTmp
```

#### Ordered Window gains and convection terms

Only exact `SurfaceClass::Window` enters the special branch. CP222 reads its
shading flag and applies these operations in source order:

1. An interior shade or blind adds divider area to local `Area` and adds
   `SurfWinDividerHeatGain` to `sumIntGain`.
2. A referenced construction with `WindowTypeEQL` adds
   `SurfWinOtherConvHeatGain`, independently of shading.
3. An interior shade or blind adds `SurfWinConvHeatFlowNatural`.
4. Strictly positive `SurfWinAirflowThisTS` adds
   `SurfWinConvHeatGainToZoneAir`.
5. Under that airflow gate, parent-Zone `NoHeatToReturnAir` additionally adds
   `SurfWinRetHeatGainToZoneAir` to the local result and mutates
   `SurfWinHeatGain += SurfWinRetHeatGainToZoneAir`.

After the persistent `SurfWinHeatGain` addition, a nonnegative accumulated
rate overwrites only `SurfWinHeatGainRep` and its zone-timestep energy. Any
other comparison outcome, including a negative or NaN rate, overwrites only
`SurfWinHeatLossRep` with the negated rate and its energy. Neither branch
clears the opposite rate/energy pair, so an earlier value can remain stale.
CP222 then overwrites signed `SurfWinHeatTransferRepEnergy` as the accumulated
heat gain times `TimeStepZoneSec`.

A strictly positive frame area contributes:

```text
HA_surf = SurfHConvInt * SurfWinFrameArea * (1 + SurfWinProjCorrFrIn)
sumHATsurf += HA_surf * SurfWinFrameTempIn
HA         += HA_surf
```

A strictly positive divider area contributes separately only without an
interior shade or blind:

```text
HA_surf = SurfHConvInt * SurfWinDividerArea * (1 + 2 * SurfWinProjCorrDivIn)
sumHATsurf += HA_surf * SurfWinDividerTempIn
HA         += HA_surf
```

The later base terms use glazing area for an unshaded Window and
glazing-plus-divider area for an interior-shaded Window. Ordinary equipment
internal gains are assembled by CP220; CP222's `sumIntGain` contains only the
Window additions above.

#### Reference-air dispatch

Each Surface reaches the reference-air switch only after its Window and base
HATsurf work:

- `ZoneMeanAirTemp` adds local HA to `sumHA`.
- `AdjacentAirTemp` adds HA times `SurfTempEffBulkAir` to `sumHATref`.
- `ZoneSupplyAirTemp` first requires the parent Zone to be controlled. An
  uncontrolled Zone calls `ShowFatalError` with
  `Zones must be controlled for Ceiling-Diffuser Convection model. No system serves zone {}`,
  formatted with the parent Zone name. The following source `return results`
  is unreachable on the normal fatal path.
- A controlled supply-air Surface uses parent
  `zoneHeatBalance(zoneNum).SumSysMCp` and `SumSysMCpT`, never the selected
  Space record. Strictly positive `SumSysMCp` adds
  `HA * SumSysMCpT / SumSysMCp` to `sumHATref`; zero, negative, or NaN
  `SumSysMCp` adds HA to `sumHA` without reading a mean-air temperature.
- The default branch, including an invalid reference-air sentinel, silently
  adds HA to `sumHA` despite the source comment that a warning should exist.

On prediction CP220 has zeroed the parent Zone system sums before nested CP222
dispatch, so supply-air references take the HA fallback. Correction CP220
assembles the parent Zone system sums before CP221, and both nested and later
explicit Space visits can observe those Zone-level values.

#### Caller ownership and cadence

CP222 has two production ingress shapes. Every Zone CP220 call dispatches
CP221, which calls CP222 once for each stored Space regardless of the explicit
Space scheduling gate. Separately, the prediction and correction wrappers call
CP220 on eligible explicit Space receivers; virtual dispatch enters CP222
directly without CP221. Zone work precedes those explicit Space calls, so a
stored Space may be processed once nested and again explicitly in the same
pass.

The corrected 55-configuration nonzero-Zone corpus has 99 stored Space
identities. A static single prediction pass therefore contains 99 nested plus
24 explicit CP222 calls, or 123. A static single correction pass contains 99
nested plus three explicit calls, or 102. The combined configuration census is
225, not a runtime total. Initial and adaptive fine-step work can repeat
prediction and correction, while demand resimulation adds prediction without
a matching correction.

CP220 commits the returned `sumIntGain` only after adding it to its earlier
internal gains, then assigns HA, HATsurf, and HATref. A nested result is first
folded into CP221's local Zone result; a direct result is committed to its
Space receiver.

#### Validation, failure, retry, and reset

Beyond two debug assertions, CP222 has no release identity check, upper bound,
Zone/Space membership test, allocation or range validation, construction
validation, Surface-class compatibility check, reference-enum validation,
finite-value check, timestep check, or finite numerator/ratio validation beyond
the strict-positive supply denominator gate. Array and construction access is
unchecked, and all arithmetic accepts nonfinite input.

The airflow/no-return Window writes occur before reference dispatch. A later
uncontrolled supply-air Surface can therefore terminate after current and
earlier Window reports have mutated. The local four-field result does not
return, later surfaces are skipped, a containing CP221 aggregate does not
return, and CP220's post-call surface commit does not execute. CP220's
entry-zeroed surface fields and its earlier gain/airflow coefficient work
remain, as do external Window writes and fatal diagnostics.

Retry creates a fresh local zero result and restarts at `HTSurfaceFirst`; it
does not resume or roll back. Stable side-effect-free inputs reproduce the
same ordered return, but `SurfWinHeatGain +=` makes the general Window path
non-idempotent. The changed accumulated sign can overwrite a different report
side while stale opposite fields survive. Nested CP221 and later explicit
Space visits can create the same repetition without a failure.

CP222 owns no persistent receiver state, completion marker, status, catch,
transaction, cleanup, recovery path, or reset hook. Clean replay requires
coordinated restoration of Surface reports, topology, temperatures,
constructions, convection coefficients, timestep state, and parent-Zone
system sums.

#### C++ test and corpus boundary

No C++ test directly calls either `calcSumHAT` override. The focused
`ZoneTempPredictorCorrector_calcZoneOrSpaceSums_SurfConvectionTest` calls Zone
CP220 five times with one Space and three non-Window surfaces. CP221 reaches
that CP222 child on every call. The first two calls assert HA, HATsurf, and
HATref, six aggregate assertions total; the remaining three assert only CP220
system sums.

The three surfaces select ZoneMean, Adjacent, and ZoneSupply reference modes.
The first two calls provide bounded positive supply-flow and fallback
coverage, but no assertion isolates `sumIntGain`. The fixture does not cover a
Window, default reference, uncontrolled fatal, receiver/Space mismatch, empty,
Window-bearing, overlapping, foreign, inverted, or otherwise malformed range,
nonfinite input, partial external effect, failure, retry, or reset. Five HybridModel
correction calls also reach CP222 through empty child ranges, but assert only
hybrid state.

Within the 55 completing full-simulation configurations, 19 contain 36 exact
Window-class identities; two additional GlassDoor identities take only the
generic path. Five Windows are equivalent-layer types. No configuration
provides an airflow-control object, active frame/divider reference, or active
airflow/no-return mutation. One configuration attaches interior blinds to
eight Windows, but its permission schedule is constant zero, so the active
interior-blind path has no corpus potential.

All corpus reference-air selections remain ZoneMean; Adjacent, ZoneSupply,
fatal, and default branches have zero full-simulation potential. Window
configurations do not overlap the explicit-Space scheduler configurations, so
the one-pass structural Window visit count is 36 prediction and 36
correction, with five equivalent-layer entries per pass. No oracle isolates
CP222's returned fields, Window branch, report side effects, range order, or
failure transaction.

#### Rust boundary

A crate-wide search finds no `calcSumHAT`, `calc_sum_hat`, `SumHATOutput`, or
the CP222 Window and reference-air symbols. The nearest
`zone_surface_convection_sums_for_indices` takes a Zone-owned Surface index
slice, silently skips an invalid index through `filter_map`, folds only
h-times-area and h-times-area-times-inside-temperature, and returns
`(sumHA, sumHATsurf, 0.0)`. It has no `sumIntGain`, Space range, Window,
reference-air switch, supply ratio, fatal, or side effect.

`SurfaceHeatBalanceState` owns a `ZoneId` but no `SpaceId`. The runtime boundary
blocks authored and generated-remainder Space topology, discards the compiler's
Surface Space identity, and builds Zone-only opaque-surface indexes. Its
surface loop always writes the owning Zone temperature into
`inside_reference_air_temperature_c`; a reporting helper can consume that
snapshot, but the convection-sum helper still hard-codes HATref to zero.

The compiler types Space relationships and several fenestration materials and
constructions, but the heat-balance runtime rejects fenestration construction
use and owns no `FenestrationSurface:Detailed` or Window report state. Its
fenestration index vectors remain empty. Zone system sums are initialized to
zero without a production supply-state assembly matching CP222.

Rust `part03` checks one-Zone initial HA/HATsurf/HATref values, `part04` checks
a direct Zone opaque-Surface sum with HATref zero, and `part05` only seeds
fields for downstream reporting. None creates a Space result, Window state,
reference mode, or failure/retry case.

The official candidate has one Zone, six opaque surfaces, no authored Space,
Window, AirLoop, or ZoneHVAC object. The compiler creates one default Space,
but runtime discards its identity. Its 8760-sample Zone surface-convection-rate
artifact remains within maximum absolute difference 0.085845581243 W and RMSE
0.005357748923 W. That is bounded downstream ZoneMean opaque-Surface evidence,
not CP222 Space range/order, four-field result, Window, reference dispatch, or
side-effect proof.

CP222 expands the same required `source_mapped` routine and adds no second
routine, project-contract item, Rust target, code, mapped state, test, support,
capability, output implementation, comparator, manifest, numerical,
performance, or conformance promotion. Counts remain 32 algorithms and 228
routines, split 58 `state_mapped` plus 170 `source_mapped`, with 105 required;
the heat-balance project list remains 74.

### CP223 `CalcZoneComponentLoadSums` source map

CP223 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.calc_zone_component_load_sums`
and heat-balance project item `calc_zone_component_load_sums` immediately
after `zone_heat_balance_calc_sum_hat` and before
`update_final_surface_heat_balance`. The nonmember routine is declared at
`ZoneTempPredictorCorrector.hh` lines 345-348 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5414-5677. It has one implementation and
one unqualified source identifier, so unlike CP222 it receives a new routine
row and project item.

#### Purpose, target ownership, and entry reset

The source describes CP223 as reporting and diagnostic work only. Its component
rates are current state at the end of the last system timestep, are not
necessarily Zone-timestep averages, are not multiplied by Zone multipliers,
and are expressed in Watts. The two phase-change fields are raw enthalpy sums
registered separately in J/kg.

The signature receives a parent `ZoneNum`, a pointer to either a Zone or Space
`ZoneSpaceHeatBalanceData`, and a reference to the target `AirReportVars`. It
does not receive `spaceNum`. Before validating or dereferencing any other
argument, it overwrites these ten target fields with exact zero in source
order:

```text
SumIntGains
SumHADTsurfs
SumMCpDTzones
SumMCpDtInfil
SumMCpDTsystem
SumNonAirSystem
CzdTdt
imBalance
SumEnthalpyM
SumEnthalpyH
```

It then aliases `Zone(ZoneNum)` and snapshots `TimeStepSysSec`. A Space target
therefore selects receiver MAT, humidity, airflow coefficients, non-air
response, temperature coefficients, and histories, but it does not select a
Space-only topology.

#### Internal, interzone, outdoor, and AFN terms

`SumIntGains` is always
`zoneSumAllInternalConvectionGains(state, ZoneNum)`. Parent-Zone
`NoHeatToReturnAir` additionally calls
`zoneSumAllReturnAirConvectionGains(state, ZoneNum, 0)`. Both helpers remain
Zone-wide for a Space report.

The default interzone expression is:

```text
thisHB.MCPTM - thisHB.MCPM * thisHB.MAT
```

The outdoor/infiltration expression retains this exact source grouping and
order:

```text
(thisHB.MCPTI - thisHB.MCPI * thisHB.MAT)
+ (thisHB.MCPTV - thisHB.MCPV * thisHB.MAT)
+ (thisHB.MCPTE - thisHB.MCPE * thisHB.MAT)
+ (thisHB.MCPTC - thisHB.MCPC * thisHB.MAT)
+ (thisHB.MDotCPOA * Zone.OutDryBulbTemp
   - thisHB.MDotCPOA * thisHB.MAT)
```

Thermal-chimney fields are not part of this report expression.

When AFN is always multizone-simulated, or when the control type is
`MultizoneWithDistributionOnlyDuringFanOperation` and the AFN fan is active,
CP223 replaces both prior results from parent-Zone `exchangeData(ZoneNum)`:

```text
SumMCpDtInfil =
    SumMCpT + SumMVCpT - (SumMCp + SumMVCp) * thisHB.MAT
SumMCpDTzones =
    SumMMCpT - SumMMCp * thisHB.MAT
```

A Space report consequently reuses Zone AFN exchange state while applying its
own receiver MAT.

#### Parent-Zone system topology and shared ADU writes

CP223 initializes local `QSensRate` to zero, then chooses one mutually
exclusive primary branch from the parent Zone:

1. A controlled Zone visits every `ZoneEquipConfig(ZoneNum)` inlet.
2. A return plenum visits all plenum inlets, then each stored ADU index and its
   independently flagged upstream and downstream leak.
3. A supply plenum visits its single inlet.
4. Any other Zone contributes no primary system term.

Each branch uses `calcZoneSensibleOutput`. A strictly positive mass flow
returns mass flow times
`PsyDeltaHSenFnTdb2Tdb1W(supply temperature, thisHB.MAT,
thisHB.airHumRat)`; zero, negative, or NaN mass flow returns exact zero.

For every controlled inlet whose `InletNodeADUNum` is positive, CP223 also
recalculates sensible output from that ADU's outlet node and overwrites four
shared ADU report fields:

```text
HeatRate = max(0, outlet sensible output)
CoolRate = abs(min(0, outlet sensible output))
HeatGain = HeatRate * TimeStepSysSec
CoolGain = CoolRate * TimeStepSysSec
```

The outlet result is not separately added to `SumMCpDTsystem`; the inlet
result already owns that report term. Zone and later Space reports can
overwrite the same ADU fields with different receiver MAT and humidity, so the
last stored Space report can remain visible after a complete Zone traversal.

The independent parallel-PIU tail repeats the CP214/CP220 ordinal anomaly.
A nonempty `leakageParallelPIUNums` list supplies only its size; CP223 reads
global `PIU(1)` through `PIU(size)` instead of the stored identities. Only a
strictly positive `leakFlow` contributes. A positive parent
`SystemZoneNodeNumber` routes the sensible result to the system term;
otherwise it routes it to the interzone term. CP223 performs no Zone multiplier
division.

`SumNonAirSystem` is then assigned from receiver
`NonAirSystemResponse` plus parent-Zone `SumConvHTRadSys(ZoneNum)` and
`SumConvPool(ZoneNum)`.

#### Whole-Zone Surface rewalk and reference-air dependency

Every CP223 call traverses every identity in
`Zone(ZoneNum).spaceIndexes`, then every integer in each Space's inclusive
`HTSurfaceFirst..HTSurfaceLast` range. There is no selected-Space argument,
heat-transfer flag, bounds, membership, class, sorting, overlap, or duplicate
check. For a Zone with N stored Spaces, its Zone report walks N ranges and its
N Space reports each walk the same N ranges, yielding N plus N-squared range
walks when Space reporting is active.

For each Surface, CP223 reads the base area and calls
`Surface::getInsideAirTemperature` before any Window contribution. That
dependency first aliases the heat-balance record of the Surface's own
`spaceNum`, not the CP223 receiver:

- `ZoneMeanAirTemp` returns the owning Space MAT.
- `AdjacentAirTemp` returns `SurfTempEffBulkAir`.
- `ZoneSupplyAirTemp` is fatal when `Zone(Surface.Zone)` is uncontrolled.
  Otherwise it uses the owning Space equipment inlet list when aggregate
  `doSpaceHeatBalance` is true, or the Surface Zone inlet list otherwise. It
  computes each inlet heat capacity from the owning Space humidity ratio and
  returns a weighted temperature only for a strictly positive total
  mass-flow-times-heat-capacity; all other totals fall back to owning Space
  MAT.
- The default branch silently returns owning Space MAT.

The exact uncontrolled supply-reference fatal text is:

```text
Zones must be controlled for Ceiling-Diffuser Convection model. No system serves zone {}
```

It is formatted with `Zone(Surface.Zone).Name`. A malformed Surface can
therefore make caller `ZoneNum`, Surface Zone ownership, receiver HB, and
reference-air Space ownership disagree.

Only exact `SurfaceClass::Window` enters the special branch, in this order:

1. An interior shade or blind adds divider area to local area and adds
   `SurfWinDividerHeatGain` to internal gains.
2. An equivalent-layer construction independently adds
   `SurfWinOtherConvHeatGain`.
3. An interior shade or blind independently adds
   `SurfWinConvHeatFlowNatural`.
4. Strictly positive window airflow adds
   `SurfWinConvHeatGainToZoneAir`; parent caller-Zone
   `NoHeatToReturnAir` also adds `SurfWinRetHeatGainToZoneAir`.
5. Strictly positive frame area adds its projected
   `h * area * (frame temperature - reference)` term.
6. Strictly positive divider area without an interior shade/blind adds its
   projected `h * area * (divider temperature - reference)` term.

Unlike CP222, CP223 does not mutate `SurfWinHeatGain` or Window gain/loss
report energies. Every Surface then adds:

```text
SurfHConvInt * effective area * (SurfTempInTmp - RefAirTemp)
```

to `SumHADTsurfs`. An exact CondFD Surface additionally adds
`SurfaceFD.EnthalpyM` to `SumEnthalpyM` and `SurfaceFD.EnthalpyF` to
`SumEnthalpyH`, without area weighting. Zone and all Space reports for the
same parent therefore rewalk the same Surface set and use each Surface's
owning-Space reference state.

#### Air storage and imbalance diagnostics

CP223 evaluates `PsyCpAirFnW(thisHB.airHumRat)` and
`PsyRhoAirFnPbTdbW(OutBaroPress, thisHB.MAT, thisHB.airHumRat)`. Both use a
`1.0e-5` humidity floor in their numerical formulas. `PsyCpAirFnW` also owns a
process-static last-input/result cache. With EnergyPlus psychrometric errors
enabled, a strictly negative density delegates to a severe/continuation/
unknown-caller timestamp/fatal path; zero and NaN do not satisfy that check.

The global Zone-air solution selector then assigns one storage formula:

- ThirdOrder multiplies density, heat capacity, parent Zone volume, parent
  `ZoneVolCapMultpSens`, and
  `(thisHB.MAT - thisHB.ZTM[0]) / TimeStepSysSec` in source order.
- Analytical assigns
  `thisHB.TempIndCoef - thisHB.TempDepCoef * thisHB.MAT`.
- Euler assigns `thisHB.AirPowerCap * (thisHB.MAT - thisHB.T1)`.
- The default branch leaves the entry zero without a diagnostic.

A Space report still uses full parent-Zone volume and capacitance multiplier.

Only `DisplayZoneAirHeatBalanceOffBalance` true assigns `imBalance` as the
six gain/transfer terms minus storage and computes a threshold equal to 20
percent of the square root of the seven squared components. A warning requires
strict `abs(imBalance) > Threshold`, plus both non-warmup and non-sizing state.
NaN comparisons suppress the warning.

When the parent Zone's `AirHBimBalanceErrIndex` is zero, the first event emits
the Zone-named warning, a one-decimal threshold continuation, an optional
night-cycle continuation when `TurnFansOn`, and occurrence timestamp. Every
event then calls the recurring-warning helper with
`abs(imBalance) - Threshold` as both max- and min-tracked values; the optional sum-tracked argument is omitted. Zone and
Space reports share the same parent Zone name and index, so multiple reports
in one correction can update one recurring stream.

#### Caller order, output registration, and cadence

The only production expressions are at the end of
`correctZoneAirTemps`. For each Zone, the wrapper:

1. completes the Zone record correction;
2. corrects each eligible simulation Space or mirrors Zone state and any
   sizing node state into it;
3. folds the Zone and Space temperature changes;
4. calls CP223 for `ZnAirRpt(zoneNum)`;
5. when `doSpaceHeatBalanceSimulation` is true, calls CP223 once for every
   stored `spaceAirRpt(spaceNum)`;
6. advances to the next Zone.

The Space report gate does not test `DoingSizing`. The dispatcher reaches this
wrapper only for `CorrectStep`. Initial HVAC correction and each selected
adaptive fine-step correction can repeat it; demand resimulation performs
prediction without a matching CP223 correction.

`AirReportVars::setUpOutputVars` registers internal, surface, interzone,
outdoor, system-air, system-convective, and air-storage rates as
`System/Average`. It registers deviation only when advanced report variables
are enabled. `ZnAirRpt` always exists for all Zones. `spaceAirRpt` is allocated
for sizing or simulation Space heat balance, but Space report variables are
registered only for simulation Space heat balance. Advanced melting and
freezing enthalpy outputs are registered as `Zone/Average` only from
`ZnAirRpt`; CP223 still computes the otherwise unregistered Space fields.

#### Validation, failure, retry, and reset

CP223 has no local assertion, null check, positive or upper identity check,
allocation, range, topology, membership, finite, enum, pressure, volume,
timestep, or denominator validation. It owns no return status, catch,
completion marker, cleanup, transaction, or rollback. A zero system timestep
can divide in the ThirdOrder expression. Array access and dependent
construction, node, equipment, plenum, PIU, Surface, and finite-difference
state remain unchecked.

The ten report zeros always commit first. A later failure can retain a
partially rebuilt report, shared ADU overwrites, dependency cache updates, and
earlier Surface sums. An uncontrolled supply reference fails before the
current Surface's Window/base/enthalpy contributions but after all earlier
flow, ADU, non-air, and Surface work. A negative-density fatal occurs after
the complete Surface loop while storage and imbalance remain zero. A Zone
report failure suppresses its Space reports and all later Zones; a Space
failure retains the Zone and earlier Space reports and suppresses the rest.

Retry starts by wiping the same ten target fields and restarts from the first
operation. Stable ordinary arithmetic reconstructs the target report and ADU
assignments deterministically, but shared `AirHBimBalanceErrIndex` and global
recurring-warning history are not reset here: a retry can skip the first
message and add another recurrence. Repeated Zone/Space calls can also
overwrite shared ADU state with different receiver conditions. A clean reset
requires coordinated report, Zone, ADU, node, Surface/CondFD, AFN,
psychrometric-diagnostic, and predictor/corrector owner restoration.

#### C++ test and corpus boundary

No C++ test calls CP223 directly. The focused
`HybridModel_correctZoneAirTempsTest` calls the wrapper five times. Each call
has one Zone, inactive Space reporting, an empty `0..-1` stored-Space Surface
range, no inlet nodes, default ThirdOrder selection, and the imbalance display
off. It therefore reaches five Zone-only CP223 calls, but all five immediate
assertions inspect HybridModel multiplier, infiltration, or People results.
No assertion targets a CP223 report or ADU field. Separate SimulationManager
tests inspect only the imbalance-display input flag.

Of 57 active full-simulation `ManageSimulation` expressions, one expected EMS
fatal stops before CP223 and one Weather fixture has zero Zones. The remaining
55 configurations contain 81 Zones. Their static one-correction-pass census is
81 Zone reports plus three active simulation-Space reports, or 84 CP223 calls,
split 74 ThirdOrder and ten Analytical with no Euler. This is configuration
topology, not a runtime total.

The 81 Zone reports walk 99 stored-Space ranges. The active two-Zone,
three-Space configuration has per-Zone Space counts two and one, so its three
Space reports add two-squared plus one-squared, or five, repeated range walks.
The static total is therefore 104. Parent topology contains 55 controlled
records and 29 other records, with no return plenum, supply plenum, or
parallel-PIU reach; AFN replacement has potential in five Zone reports without
an isolating assertion.

Nineteen completing configurations contain 36 exact Window identities,
including five equivalent-layer Windows. These do not overlap the active
Space-report configuration, so the one-pass Window visit count remains 36.
Two GlassDoor identities use only the generic Surface path. No active
airflow-window or frame/divider object exists, and eight blind-controlled
Windows have a constant-zero permission schedule, so the active
shade/blind paths have no corpus reach. Every reference-air selection is
ZoneMean. One CondFD Surface reaches the enthalpy accumulation once, but its
fixture asserts only construction override. No completing configuration
enables imbalance display, and no full-simulation assertion isolates the
complete reporting update sequence, ADU effect, diagnostic, failure, retry, or reset.

#### Rust and numerical-evidence boundary

A crate-wide search finds no `CalcZoneComponentLoadSums`,
`calc_zone_component_load_sums`, ten-field report record, Space report arena,
ADU/PIU/plenum report state, shared imbalance index, or PCM enthalpy report.
`ZoneHeatBalanceState` owns only adjacent Zone fields: current/history air
state, internal gain, opaque-Surface HA/HAT values, zero-initialized
non-system/system coefficients, temperature coefficients, and optional
averaged surface/storage reports. No production writer fills its airflow
coefficient fields with CP223 topology.

The run-period report path separately samples a three-value Zone tuple:
`convective_internal_gain_w`, a selectable opaque-Surface convection helper,
and a guarded storage helper. The ResultStore publishes those three series and
a fourth outdoor-air transfer series whose vector is hard-coded zero. It
publishes no interzone, system-air, system-convective, deviation, enthalpy, or
Space series.

The nearest Surface helpers either silently skip invalid indexes and return
opaque `HA/HATsurf/HATref=0`, directly sum retained Surface
`hA * (Tsurf - stored reference)`, or substitute `storage - internal` as a
diagnostic balance probe. They implement no Window, owning-Space reference,
or side effect. The storage helper selects ThirdOrder only through runtime
configuration, returns zero for a nonpositive timestep, and otherwise collapses
to the Analytical expression; it has no exact Euler/default switch.

Rust unit tests cover isolated Analytical/ThirdOrder/optional-capacity/
nonpositive-timestep storage results, weather capacity, alternative Surface
report formulas, and average-versus-last sampling. They do not construct the
ten-field CP223 update sequence, Space topology, equipment/ADU, Window/PCM, AFN,
diagnostic, or failure lifecycle.

The official one-Zone candidate contains one uncontrolled Zone, six opaque
ZoneMean Surfaces, and no authored Space, Window, AirLoop, ZoneHVAC, AFN, or
Space simulation. Its 8760 samples have exact-zero internal-gain and
outdoor-transfer differences. Surface convection has maximum absolute
difference 0.085845581243 W and RMSE 0.005357748923 W; air storage has maximum
absolute difference 0.076879349871 W and RMSE 0.005076386180 W. Those are four
existing bounded output-variable claims assembled by separate Rust paths, not
proof of positive internal/outdoor terms, the other report fields, or the
complete CP223 update sequence.

CP223 adds no algorithm-level `energyplus_source` entry, Rust target, code, mapped state,
test, support, capability, output implementation, comparator, manifest,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 229 routines, split 58 `state_mapped` plus 171 `source_mapped`,
with 106 required; the heat-balance project list becomes 75.

### CP224 `VerifyThermostatInZone` source map

CP224 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.verify_thermostat_in_zone`
and heat-balance project item `verify_thermostat_in_zone` immediately after
`calc_zone_component_load_sums` and before
`update_final_surface_heat_balance`. The nonmember routine is declared at
`ZoneTempPredictorCorrector.hh` line 350 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5679-5700.

#### Lazy input acquisition and exact membership

The signature receives mutable `EnergyPlusData` plus a constant `ZoneName`
reference and returns only a boolean. In exact source order it:

1. tests `DataZoneControlsData::GetZoneAirStatsInputFlag`;
2. when true, calls CP196 `GetZoneAirSetPoints(state)`;
3. clears that shared flag only after the loader returns normally;
4. tests `NumTempControlledZones > 0`;
5. if positive, calls `FindItemInList` over
   `TempControlledZone` and the `ZoneTempControls::ZoneName` member;
6. returns true only for a positive one-based match index, otherwise false.

The member-pointer `FindItemInList` overload uses the array's full `isize()`,
not `NumTempControlledZones`, and compares `ZoneName == stored member`
directly. The count is therefore only a zero/nonzero gate. CP224 performs no
case conversion, trimming, Zone-name-map resolution, count/allocation
consistency check, or identity validation. It does not search comfort
controls, humidistats, staged controls, or zone-equipment connections.
Ordinary CP196 thermostat input expands a ZoneList into individual
`TempControlledZone.ZoneName` records before this predicate runs.

CP224 itself emits no diagnostic and registers no output. Its only direct
write is the successful lazy-input latch clear; every other mutation, output
registration, EIO row, or diagnostic reached through the first call belongs
to CP196.

#### Production caller and cadence

The sole production expression is
`ZoneEquipmentManager::SetUpZoneSizingArrays` line 812. That routine loops
`ZoneSizingInput` in stored order. CP224 is reachable only inside a global
`any_of(ZoneEquipConfig.IsControlled)` branch and only when the current
sizing record's cooling or heating airflow method is exactly
`AirflowSizingMethod::FromDDCalc`. `DesignDayWithLimit` is a different enum
and does not satisfy this comparison. When both heating and cooling match,
the `||` guard still calls CP224 once.

The caller first tries to match the sizing Zone in `ZoneEquipConfig`, but a
missing current match only warns; the later CP224 expression still runs while
some other equipment configuration is controlled. A false predicate produces,
except during pulse sizing, this caller-owned warning:

```text
SetUpZoneSizingArrays: Requested Sizing for Zone="{}", Zone has no thermostat (ref: ZoneControl:Thermostat, et al)
```

That warning does not set `ErrorsFound` and does not stop setup. CP224 cannot
distinguish a blank/mismatched name, an empty thermostat set, or a name
present only in another control arena; each normal lookup is false.

`ManageZoneEquipment` enters `SizeZoneEquipment` only during
`ZoneSizingCalc`. Its default-true `SizeZoneEquipmentOneTimeFlag` calls
`SetUpZoneSizingArrays` once and clears only after normal return. Normal
simulation cadence is therefore one CP224 lookup per eligible sizing record
during first setup, not one per HVAC timestep. Direct callers may invoke the
predicate independently.

#### Failure, retry, and reset

A normal false lookup is not an error. After the shared input flag is already
false, CP224 is read-only, stable-state replay returns the same boolean, and
there is no separate error status, catch, cleanup, cache, allocation, transaction, or
rollback.

A CP196 fatal or other abnormal non-return occurs before CP224's latch clear
and before its boolean return. CP196's completed allocation, Zone/control
mutation, output, EIO, and diagnostic prefix survives while
`GetZoneAirStatsInputFlag` remains true. In the production chain,
`SizeZoneEquipmentOneTimeFlag` also remains true because its clear follows
the setup return. A caught same-state retry can therefore re-enter the
non-idempotent full input loader. Clean replay requires coordinated reset of
CP196's owners, `DataZoneControlsData`, and the zone-equipment-manager sizing
latch.

#### C++ and corpus evidence

There is no direct C++ call or assertion for CP224. The focused
`AirTerminalSingleDuctMixer_GetInputDOASpecs` fixture creates two direct
DesignDay sizing Zones and two controlled equipment connections but no
thermostat. Its direct `SetUpZoneSizingArrays` call reaches CP224 twice: the
first call acquires empty thermostat input and returns false, and the second
is lookup-only and false. Assertions inspect only two outdoor-air pointers,
not the predicate or its warnings. Two other direct setup fixtures have no
eligible sizing/controlled topology and do not reach CP224.

Among 57 active `ManageSimulation` expressions, 34 completing configurations
request Zone sizing and contain 48 uncommented, direct-Zone `Sizing:Zone`
records. Every one uses `DesignDay` for both airflow methods and has exact
matching `ZoneControl:Thermostat` and
`ZoneHVAC:EquipmentConnections` names; none targets a ZoneList. Their static
first-setup census is therefore 48 true CP224 calls. This is configuration
and one-time topology evidence, not an isolated assertion. No test directly
covers exact-case mismatch, inconsistent count/arena state, a false-result
warning, pulse suppression, lazy-load failure, retry, or reset.

#### Rust boundary

A crate-wide search finds no `VerifyThermostatInZone`,
`verify_thermostat_in_zone`, source-shaped boolean helper, shared
`GetZoneAirStatsInputFlag`, or executable `Sizing:Zone` setup. Rust eagerly
parses a bounded direct-Zone `ZoneControl:Thermostat` subset into
`ZoneThermostat`: only resolved ZoneIds and
`ThermostatSetpoint:DualSetpoint` controls are retained. Names are trimmed and
ASCII-uppercased, unlike CP224's exact stored-string comparison.

`ModelGraph` builds ZoneId/thermostat and thermostat/setpoint edges, and the
execution plan emits `EvaluateZoneThermostat` metadata before `SolveZone`.
A separate IdealLoads diagnostic uses a normalized ZoneId edge and returns
an error when no thermostat is found; it is not CP224's boolean or sizing
caller.
The adjacent `get_zone_air_set_points_compat` merely invokes an arbitrary
closure and owns no input latch. Compiler/graph/plan tests prove one typed
edge and reject unsupported control types, but none implements CP224's lazy
full CP196 arena, sizing caller, exact-name result, warning ownership, or
failure/retry lifecycle. Existing IdealLoads thermostat outputs remain
case-bounded runtime evidence and do not promote this predicate.

CP224 adds no algorithm-level `energyplus_source` entry, Rust target, code,
mapped state, test, support, capability, output implementation, comparator,
manifest, numerical, performance, or conformance promotion. The inventory
becomes 32 algorithms and 230 routines, split 58 `state_mapped` plus 172
`source_mapped`, with 107 required; the heat-balance project list becomes 76.

### CP225 `VerifyControlledZoneForThermostat` source map

CP225 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.verify_controlled_zone_for_thermostat`
and heat-balance project item `verify_controlled_zone_for_thermostat`
immediately after `verify_thermostat_in_zone` and before
`update_final_surface_heat_balance`. The nonmember routine is declared at
`ZoneTempPredictorCorrector.hh` line 352 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5702-5713.

#### Exact equipment-configuration membership

The signature receives mutable `EnergyPlusData` plus a constant `ZoneName`
reference and returns only a boolean. Its sole body expression calls the
member-pointer `FindItemInList` overload over
`state.dataZoneEquip->ZoneEquipConfig` and
`DataZoneEquipment::EquipConfiguration::ZoneName`, then tests whether the
returned first-match index is positive.

That overload uses the arena's full `isize()` and direct
`ZoneName == stored member` comparison. CP225 performs no case conversion,
trimming, Zone-name-map lookup, controlled-Zone count or identity check,
allocation/count consistency check, or duplicate detection. It does not test
`EquipConfiguration::IsControlled`, inspect the equipment list, or search
`spaceEquipConfig`. An empty arena or no match returns false. Any matching
stored member returns true regardless of the rest of that record.

Normal `GetZoneEquipmentData` allocates `ZoneEquipConfig` to `NumOfZones` at
`DataZoneEquipment.cc` line 271. Each successfully processed
`ZoneHVAC:EquipmentConnections` writes its name at the resolved actual-Zone
index; every uncontrolled slot retains constructor defaults
`ZoneName = "Uncontrolled Zone"` and `IsControlled = false`. CP225's missing
flag test means a direct argument exactly matching this mixed-case sentinel,
or a manually corrupted name in any uncontrolled slot, returns true. Standard
parsed Zone names and references are uppercased, so a normal input Zone written
with that spelling becomes `UNCONTROLLED ZONE` and does not collide with the
sentinel.

CP225 performs no lazy input acquisition. It owns no write, diagnostic, output, separate status state, allocation,
cache, catch, cleanup, transaction, or rollback.

#### Production callers and one-time cadence

Exactly two production expressions call CP225, both inside CP199
`InitZoneAirSetPoints`:

1. lines 2681-2691 visit all ordinary `TempControlledZone` entries in stored
   order and call at line 2684;
2. lines 2743-2753 then visit all `ComfortControlledZone` entries and call at
   line 2746.

Each expression is gated by
`ZoneEquipInputsFilled && !ControlledZonesChecked`. The gate is reevaluated
per record, but neither value changes inside either loop. A false ordinary
lookup emits one Severe plus one Continue diagnostic:

```text
InitZoneAirSetpoints: Zone="{}" has specified a Thermostatic control but is not a controlled zone.
...must have a ZoneHVAC:EquipmentConnections specification for this zone.
```

The comfort branch emits the same Continue line after a family-specific
Severe that says the Zone has specified a Comfort control. Each false result
sets CP199's persistent `ErrorsFound = true`. The ordinary loop and then the
complete comfort loop still finish. Only afterward does any retained error
cause:

```text
InitZoneAirSetpoints - program terminates due to previous condition.
```

A normal input-filled return passes that fatal point and commits
`ControlledZonesChecked = true`. This is a predictor/corrector state-lifetime
latch, not an environment, day, or timestep latch. A successful pass therefore
prevents rechecking even if the arena later changes. If both control counts
are zero, no CP225 call occurs but an input-filled normal return still commits
the latch.

CP199 is called before the selector switch on every
`ManageZoneAirUpdates` invocation. The external-HVAC initializer can also call
CP199 directly. Verification is deferred until the first parent invocation
that observes equipment input ready. The standard `GetZoneEquipment` path sets
`ZoneEquipInputsFilled = true` only after `GetZoneEquipmentData` returns
normally, so it does not expose a loader-fatal partial arena to this standard
caller. Direct state construction or direct CP225 calls are not protected by
that lifecycle.

#### Failure, retry, and reset

A normal CP225 false result is only a boolean; caller-owned effects make it
fatal. CP225 itself is read-only, deterministic, and idempotent for a stable
arena. The parent transaction is not: CP199 clears its one-time initialization
latch at line 2618 before these loops, may already have committed allocations,
output registrations, environment resets, diagnostics, and demand-limit
writes, and never clears `ErrorsFound` on entry.

After any missing match, the line-2810 fatal leaves `ErrorsFound = true` and
`ControlledZonesChecked = false`. A caught same-state retry repeats all lookup
work and each still-missing diagnostic, then fatals again. Even correcting the
arena so all later lookups return true cannot overcome the retained error
without resetting its owner. Clean replay requires coordinated reset of CP199
and its Zone-controls, Zone-equipment, heat-balance, demand, environment, and
output owners.

#### C++ and corpus evidence

No C++ test calls CP225 directly. The four direct
`InitZoneAirSetPoints` fixture calls at `HVACUnitaryBypassVAV.unit.cc` lines
659 and 1668 and `SystemReports.unit.cc` lines 204 and 365 all run before
`ZoneEquipInputsFilled` is true, so their CP225 reach is zero.

Among 57 active `ManageSimulation` expressions, 38 completing configurations
contain 52 active ordinary `ZoneControl:Thermostat` records. All target direct
Zones, expand to 52 `TempControlledZone` records, and have exact matching
`ZoneHVAC:EquipmentConnections.ZoneName` values. No configuration contains an
active thermal-comfort thermostat. Their static first-ready-check census is
therefore 52 true ordinary calls, zero false calls, and zero comfort calls.
This includes both raw-string SQLite fixtures in
`OutputReportTabular.unit.cc`; two later thermostat lines in that file are
commented out and excluded.

This is indirect topology evidence only. No assertion isolates CP225's
boolean, full-arena or sentinel behavior, exact-name mismatch, ordinary versus
comfort ordering, diagnostics, sticky fatal, deferred/committed latch,
post-success mutation, failure, retry, or reset.

#### Rust boundary

A crate-wide search finds no `VerifyControlledZoneForThermostat`,
`verify_controlled_zone_for_thermostat`, preserved Zone-name lookup arena,
`ZoneEquipInputsFilled`, `ControlledZonesChecked`, or equivalent
ordinary/comfort validation lifecycle.

Before full equipment-connection parsing, the compiler's
`mark_nominal_controlled_zones` collects nonblank raw
`ZoneHVAC:EquipmentConnections.zone_name` values, normalizes them by trimming
and ASCII uppercasing, and marks matching typed Zones. An incomplete raw
connection can set `Zone::is_nominal_controlled`, an unknown Zone is silently
absent from the marked set, and production code does not consume the marker.
This is neither CP225's preserved-string full-arena lookup nor its caller
check.

Rust later parses bounded direct-Zone DualSetpoint thermostats and typed
equipment connections independently. Both resolve normalized names to
`ZoneId`; connection parsing also requires a typed equipment list and Zone-air
node and rejects duplicate connections for one Zone. The typed
`ZoneEquipmentConnection` stores only `ZoneId`, not the original Zone string.
There is no cross-family compiler error for a valid thermostat whose Zone has
no equipment connection, and no thermal-comfort thermostat model.

`ModelGraph` likewise builds thermostat edges independently from
equipment-backed IdealLoads edges. `EvaluateZoneThermostat` remains plan
metadata without CP225 validation. A separate
`validate_ideal_loads_zone_equipment_dispatch` checks the selected IdealLoads
system's typed graph edge and `(ZoneId, EquipmentListId)` connection, returning
Rust issue values when dispatch prerequisites are missing. It does not iterate
ordinary or comfort thermostat entries, return this source boolean, or
reproduce the deferred one-shot gate, Severe/Continue/Fatal ownership, sticky
failure, retry, or reset.

CP225 adds no algorithm-level `energyplus_source` entry, Rust target, code,
mapped state, test, support, capability, output implementation, comparator,
manifest, numerical, performance, or conformance promotion. The inventory
becomes 32 algorithms and 231 routines, split 58 `state_mapped` plus 173
`source_mapped`, with 108 required; the heat-balance project list becomes 77.

### CP226 `DetectOscillatingZoneTemp` source map

CP226 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.detect_oscillating_zone_temp`
and heat-balance project item `detect_oscillating_zone_temp` immediately after
`verify_controlled_zone_for_thermostat` and before
`update_final_surface_heat_balance`. The nonmember routine is declared at
`ZoneTempPredictorCorrector.hh` line 354 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5715-5861.

#### One-time allocation, registration, and activation

The state owner starts with `SetupOscillationOutputFlag = true`,
`OscillationVariablesNeeded = false`, six Facility and annual scalar values at
zero, and unallocated history/result arrays. On the first reached call, the
setup branch allocates `ZoneTempHist(4, NumOfZones)` and fills it with zero,
then dimensions and zeroes `ZoneTempOscillate`,
`ZoneTempOscillateDuringOccupancy`, and
`ZoneTempOscillateInDeadband` to `NumOfZones`.

For each one-based Zone in stored order, setup reads `Zone(iZone).Name` and
registers these three values under that key:

- `Zone Oscillating Temperatures Time`;
- `Zone Oscillating Temperatures During Occupancy Time`;
- `Zone Oscillating Temperatures in Deadband Time`.

It then registers the matching three `Facility Any Zone ...` values under the
`Facility` key. All six definitions use hours, System timestep, and Sum
storage. Registration count is therefore `3 * NumOfZones + 3`, including the
three Facility registrations for a zero-Zone model.

Only after registration does setup call `ReportingThisVariable` for the six
names in Zone-first then Facility order. That helper uppercases the requested
name and exact-matches the output-request arena, with a secondary active-meter
check. Any true result sets `OscillationVariablesNeeded = true`; setup does not
write false. The predefined
`ZoneTemperatureOscillationReportMonthly` requests the first Zone value when
that monthly report is selected.

An independently parsed valid `PerformancePrecisionTradeoffs` object sets
`OscillationVariablesNeeded = true` at `SimulationManager.cc` line 1226 before
normal runtime and also enables the performance log. Thus either an applicable
output request or that object activates calculation. Allocation and all output
registrations still occur when neither route activates it. The setup flag is
cleared only after all allocation, registration, and request checks return
normally. A request added later is not rescanned.

#### Four-sample detector and duration classification

Every invocation snapshots `TimeStepSys`, but the complete numerical path is
guarded by `OscillationVariablesNeeded`. When false, histories never advance,
per-Zone and Facility values remain at their initialized values, and annual
scalars do not accumulate.

When true, each Zone is processed in numeric order. The history shifts
`4 <- 3`, `3 <- 2`, `2 <- 1`, then writes current
`zoneHeatBalance(iZone).ZT` into slot 1. Slots one through four are therefore
newest through oldest. The routine computes:

```text
Diff12 = T1 - T2
Diff23 = T2 - T3
Diff34 = T3 - T4
```

The exact positive-first predicate is strict
`Diff12 > 0.15 && Diff23 < -0.15 && Diff34 > 0.15`; the negative-first
predicate uses the strict opposite signs. `HVAC::OscillateMagnitude` owns the
positive `0.15 C` constant and the routine precomputes only its negative.
Equality at either boundary is false. NaN comparisons are false. There is no
absolute-magnitude alternative, finite check, elapsed-time scaling,
uniform-timestep validation, `UseZoneTimeStepHistory` branch, shortened-step
rollback, or valid-sample counter.

The history is zero-seeded rather than initialized from Zone temperature.
The first two active calls cannot satisfy all three strict differences because
an untouched adjacent pair remains zero. The third can classify a swing using
the initial zero as its oldest sample.

Before classification output, the routine clears every Zone's occupancy and
deadband duration for this call. An oscillating Zone gets
`ZoneTempOscillate = TimeStepSys`. Its occupancy duration is set to the same
value only when the complete `ThermalComfortInASH55` array is allocated and
that Zone slot has `ZoneIsOccupied = true`. There is no size check. Its
deadband duration is set when the unguarded
`CurDeadBandOrSetback(iZone)` lookup is true. A nonoscillating Zone explicitly
gets zero base duration.

Three local booleans OR these classifications across all Zones. Each Facility
value becomes `TimeStepSys` when any Zone qualifies in its category and zero
otherwise, so it measures time with at least one qualifying Zone rather than
Zone-hours. Occupancy and deadband categories are independent and can overlap.
The routine then adds each Facility value once to its corresponding
`AnnualAnyZoneTempOscillate...` scalar.

#### Caller cadence and downstream consumers

A source-wide search finds one production call expression:
`HVACManager::ManageHVAC` line 431. It is inside the system-timestep loop,
after any shortened-step predictor, corrector, and system-history push and
after Zone/Space average temperature and humidity accumulation. It runs before
Zone-list loads, storage/water/electric updates, the end-system-timestep EMS
checkpoint, and System output processing.

Accordingly CP226 runs once per accepted system timestep that reaches this
point: once for an unshortened Zone step or once for each shortened substep.
The tentative full Zone-step work used to decide shortening is not separately
inserted into this history. A `stopSimulation` break at the loop head or any
earlier non-return skips it. The routine itself has no `WarmupFlag`, kickoff,
sizing, `ZoneSizingCalc`, `DoOutputReporting`, environment, or history-mode
gate, so enabled state advances during every standard caller pass that reaches
it. The external-HVAC-manager path bypasses `ManageHVAC` and CP226 entirely.

The registered Zone and Facility backing values are available to the subsequent System-step
output update. The three annual Facility scalars are separate unregistered
state read by `OutputReportTabular.cc` lines 7909-7917 for two-decimal
performance-log fields. Exact-field search finds no environment/day/run-period
reset, so these raw accumulators span every enabled caller pass until the
whole owner is cleared.

#### Failure, replay, and reset

CP226 is void and owns no explicit diagnostic, validation, status, catch,
cleanup, transaction, or rollback. A non-return during setup leaves its clear
commit unreached but may retain arrays or a partial output registry. A caught
same-state retry re-enters allocation and registration and can encounter or
repeat those effects.

A non-return during the Zone loop can retain shifted histories and output
values for an earlier Zone prefix while later Zones remain untouched; Facility
values and annual scalars are not recomputed until the loop completes. Retry
then shifts the retained prefix again. Even normal duplicate enabled calls are
generally non-idempotent because history advances again and a qualifying
duration can be accumulated twice.

`ZoneTempPredictorCorrectorData::clear_state()` placement-news the complete
owner, restoring the setup and calculation defaults, zero scalars, and empty
arrays. There is no narrower CP226, begin-environment, begin-day, or annual
reset.

#### C++ and corpus evidence

No exact CP226 routine, setup flag, calculation flag, history, Zone/Facility
duration, or annual-field reference exists anywhere under the C++ test tree.
There is no direct test, focused wrapper, or destination assertion.

Among 57 active `ManageSimulation` expressions, one expected EMS fatal stops
at the begin-timestep EMS checkpoint before `ManageHVAC`. The other 56
configurations reach first setup; one is a zero-Zone weather fixture and the
55 nonzero-Zone configurations contain 81 Zones. Static first-setup topology
therefore yields 243 Zone plus 168 Facility output registrations, 411 total,
324 history slots, and three 81-entry result arrays.

The 57 enclosing test blocks contain none of the six exact output names,
`ZoneTemperatureOscillationReportMonthly`, `AllMonthly`,
`AllSummaryAndMonthly`, or `PerformancePrecisionTradeoffs`. No monthly request
is injected on their paths. All 56 reached setups therefore retain
`OscillationVariablesNeeded = false`; their strict detector, history shifts,
occupancy/deadband reads, Facility values, and annual accumulation have zero
execution evidence. Nineteen configurations contain 33 raw People objects and
38 contain 52 thermostat objects, but those static objects cannot establish
runtime occupancy or deadband classification, especially while the guarded
body is disabled.

These are configuration and first-setup counts, not runtime timestep totals.
The sole caller can repeat through warmup, design periods, ordinary timesteps,
and adaptive substeps. No assertion isolates allocation, registration,
activation, zero-seed startup, strict boundaries, any-Zone collapse, output,
performance log, failure, retry, or reset.

#### Rust boundary

A crate-wide search finds no `DetectOscillatingZoneTemp`,
`detect_oscillating_zone_temp`, `OscillateMagnitude`, any of the six output
names, `AnnualAnyZoneTempOscillate`, matching performance-log state,
`ThermalComfortInASH55.ZoneIsOccupied`, or
`CurDeadBandOrSetback`.

Rust retains current MAT plus separate three-slot Zone and system temperature
histories, both seeded from initial temperature. Its nearest threshold is the
`0.3 C` maximum-Zone-temperature-difference constant used only to select
adaptive step count. Adaptive correction mutates a local three-slot history
without an oscillation callback. Neither path is CP226's independent
zero-seeded four-slot system-timestep history or three-difference strict
alternating predicate.

Typed People and a CLI IdealLoads DCV diagnostic provide unrelated numeric
occupancy inputs; arbitrary runtime supplies no current people count. An
IdealLoads-local `Deadband` result enum is neither persistent Zone
`CurDeadBandOrSetback` state nor detector input. The output registry supports
hourly Zone MAT and debug series and rejects unavailable requested variables;
it has none of CP226's three Zone plus three Facility System/Sum output-name families,
setup/request latch, any-Zone values, annual accumulators, or performance-log
writer.

Predictor/corrector control, execution-plan, and source-order metadata contain
no Detect stage or `HVACManager` call position. The nearest tests cover
three-slot adaptive synchronization, an IdealLoads-local Deadband result, and
direct numeric OA-DCV people input. None covers the four-sample predicate,
strict `0.15 C` boundaries, activation, classifications, Facility OR, annual
sum, output, or lifecycle.

CP226 adds no algorithm-level `energyplus_source` entry, Rust target, code,
mapped state, test, support, capability, output implementation, comparator,
manifest, numerical, performance, or conformance promotion. The inventory
becomes 32 algorithms and 232 routines, split 58 `state_mapped` plus 174
`source_mapped`, with 109 required; the heat-balance project list becomes 78.

### CP227 `AdjustAirSetPointsforOpTempCntrl` source map

CP227 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.adjust_air_set_points_for_op_temp_cntrl`
and heat-balance project item `adjust_air_set_points_for_op_temp_cntrl`
immediately after `detect_oscillating_zone_temp` and before
`update_final_surface_heat_balance`. The nonmember routine is declared at
`ZoneTempPredictorCorrector.hh` line 356 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5863-5897.

#### Guards, inputs, and exact inverse

The signature receives mutable state, separate one-based temperature-control
and actual-Zone identities, and a mutable setpoint reference. A false
`AnyOpTempControl` returns immediately without touching either identity,
schedule state, MRT, or output. When true, CP227 indexes
`TempControlledZone(TempControlledZoneID)`. Exact `OpTempCtrl::None` then
returns before the actual-Zone identity or output is used.

For every non-None record, Scheduled mode dereferences
`opTempRadiativeFractionSched` and samples its current value. Constant,
Invalid, and every other enum value use `FixedRadiativeFraction`. CP227 next
copies `zoneHeatBalance(ActualZoneNum).MRT`. It does not cross-check that the
control record's `ActualZoneNum` equals the supplied actual-Zone identity.

With local fraction `f`, local MRT `Tr`, and the referenced input target `Top`,
the sole write is:

```text
ZoneAirSetPoint = (Top - f * Tr) / (1.0 - f)
```

This is the direct inverse of `Top = (1-f) * Tair + f * Tr`. CP227 writes no
other record, output, diagnostic, cache, flag, or status. The fraction and MRT
are local snapshots before the assignment, so even a nonproduction alias of
the output reference to one of those source fields uses the copied RHS values.

#### Parser guarantees and default-mode anomaly

`GetZoneAirSetPoints` counts
`ZoneControl:Thermostat:OperativeTemperature` objects and sets
`AnyOpTempControl = true` when the count is positive. Each valid target selects
Constant or Scheduled mode. Constant input validates
`0.0 <= FixedRadiativeFraction < 0.9`; Scheduled input requires a schedule and
checks every schedule value over the same inclusive-lower/exclusive-upper
range. A normal error-free runtime therefore has
`0.1 < 1-f <= 1.0`.

Those checks belong to input acquisition. CP227 does not validate identities,
allocation, record membership, mode, schedule pointer, current fraction,
MRT, setpoint, denominator, finiteness, or output range, and it does not clamp.
Direct state construction or later mutation can violate every parser
invariant.

`ZoneTempControls::OpTempCtrl` defaults to Invalid and its fixed fraction
defaults to zero. Only the operative-object parser assigns this field. It can
assign a parsed None value, but immediately marks that key invalid and the
input error prevents normal completion. Consequently, an error-free parsed
runtime retains no None record. Once any target sets the global flag, another
controlled Zone with no operative object does not take CP227's per-record None
return. It takes the Invalid/fixed-zero path,
reads that Zone's MRT, and evaluates the formula. This preserves a finite
setpoint for finite MRT, but IEEE `0 * Inf` and `0 * NaN` can make even that
nominal identity produce NaN.

For invalid direct state, `f == 1` exposes division by zero; NaN and infinity
propagate according to the build's floating-point behavior, and large finite
operands can overflow or lose precision. The routine has no special equality,
negative-fraction, nonfinite, or exception path.

#### Caller branch order and result lifetime

A source-wide search finds five production call expressions, all inside
`CalcZoneAirTempSetPoints`:

1. SingleHeat calls once after its used heating schedule is loaded;
2. SingleCool calls once;
3. SingleHeatCool calls once;
4. DualHeatCool calls first for cooling and then for heating;
5. Uncontrolled calls zero times.

The caller samples its control-type schedule for every temperature-controlled
Zone on each pass. SingleHeat, SingleCool, and Dual branches store raw source
targets in `TempControlledZone.ZoneThermostatSetPointHi/Lo` before CP227;
SingleHeatCool does not write those record fields. CP227 mutates only the
corresponding `zoneTstatSetpts` reference.

SingleCool, SingleHeatCool, and Dual cooling can run CP228 adaptive-comfort
selection before CP227, making that operative target the conversion input.
SingleHeatCool and Dual optimum-start handling occurs later and can overwrite
the converted high/low setpoints with raw occupied schedule values. Cooling
humidity adjustment follows CP227 for SingleCool and after optimum-start for
Dual. Thermostat-fault offsets then modify generic, low, and high setpoints.
Comfort-control calculation and EMS overrides run after the complete ordinary
control loop. CP227's value is therefore an intermediate air target, not an
unconditionally final reported or demand target.

`ManageZoneAirUpdates(GetZoneSetPoints)` is the only production parent path to
`CalcZoneAirTempSetPoints`. Built-in `ManageHVAC` calls it once per Zone
timestep before entering the system-timestep loop, so system-step shortening
alone does not repeat CP227. A DemandManager HVAC resimulation can invoke
another GetZoneSetPoints/predict/correct sequence at the same simulation time.
The full parent reloads raw setpoint schedules before CP227, so stable
schedule, adaptive target, fraction, and MRT inputs reproduce rather than
compound the result. An HVAC-only resimulation reuses the existing MRT; a
heat-balance resimulation can recalculate MRT first. The external-HVAC-manager
route bypasses built-in `ManageHVAC` and CP227.

There is no CP227 warmup, kickoff, sizing, environment, timestep, or
resimulation gate. Entry count depends on Zone timesteps, dynamic
control-type schedules, and demand resimulation, not the number of shortened
system substeps.

#### Failure, retry, reset, and direct replay

With the global flag false, malformed identities and pointers remain
unobserved. With it true, the control arena and identity must be valid. None
mode still protects the actual-Zone identity. Scheduled mode requires a
nonnull schedule pointer before MRT lookup; all active paths require a valid
Zone heat-balance identity. The routine performs its only write after all
these reads, so an abnormal non-return before assignment leaves the referenced
setpoint unchanged, aside from any dependency-internal behavior.

The parent loop is not transactional. A failure on one later record retains
raw and converted setpoint writes for earlier records plus the current
caller's pre-CP227 schedule/adaptive prefix, and prevents downstream optimum,
humidity, fault, comfort, and EMS work. A caught full-parent retry reloads
each reached raw schedule before conversion. CP227 alone, however, accepts its
already transformed output as the next input; repeated direct calls are
generally non-idempotent and double-transform unless a special value such as a
finite fixed-zero fraction makes the operation an identity.

CP227 owns no persistent state or independent reset. Normal parent passes
overwrite their setpoint targets. Clean recovery from parent failure requires
the owning control, schedule, heat-balance, setpoint, and downstream managers
to be restored consistently.

#### C++ and corpus evidence

No C++ test calls `AdjustAirSetPointsforOpTempCntrl` directly. Four fixtures
make 21 direct `CalcZoneAirTempSetPoints` calls which expand to 46 CP227
entries: six in `SysAvailManager_OptimumStart`, 18 in the reporting fixture,
and 11 in each of two cutout fixtures. Every entry has
`AnyOpTempControl = false` and returns at the first guard. Thirty-three
downstream thermostat assertions exercise broader parent behavior but isolate
neither active mode nor MRT conversion.

The only four raw operative-temperature objects in the complete unit tree are
Constant with fixed fraction zero in the adaptive-thermostat fixture. That
fixture invokes neither `GetZoneAirSetPoints`,
`CalcZoneAirTempSetPoints`, nor CP227; it manually constructs adaptive state
and tests CP228 instead. There is no Scheduled fixture, nonzero fixed fraction,
or active CP227 formula assertion.

None of the 57 active `ManageSimulation` expressions contains
`ZoneControl:Thermostat:OperativeTemperature`; the one expected EMS fatal
stops before GetZoneSetPoints and the other 56 configurations retain
`AnyOpTempControl = false`. Thirty-eight configurations contain 52 ordinary
thermostat records. Their linked control topology is 49 Dual-only records plus
three SingleHeat/SingleCool-switching records, so one active setpoint sweep has
`49 * 2 + 3 * 1 = 101` static CP227 entry opportunities. Every one returns at
the global guard. Constant, Scheduled, default-Invalid fixed-zero, MRT, and
formula body reach are all zero.

These are configuration and direct-parent counts, not accumulated runtime
calls. Warmup, ordinary Zone timesteps, schedule switching, and demand
resimulation can repeat entry. No test covers the active global gate,
per-record None, another Zone's Invalid/fixed-zero behavior, fixed or
scheduled nonzero fraction, invalid pointer or identity, mismatched identities,
nonfinite arithmetic, downstream overwrite ordering, direct replay, failure,
or reset.

#### Rust boundary

A crate-wide search finds no `AdjustAirSetPointsforOpTempCntrl`,
`adjust_air_set_points_for_op_temp_cntrl`, operative-temperature control,
Zone MRT, radiative-fraction binding, or inverse setpoint formula.

Rust types only a bounded direct-Zone DualSetpoint thermostat graph: it retains
heating/cooling schedule IDs, a control-type schedule, and cutout delta.
`ZoneControl:Thermostat:OperativeTemperature` is absent from typed object,
capability, and object-coverage registries, has no partial-support rule, and
therefore run-blocks as raw unsupported input. Zone heat-balance state has MAT
and history but no MRT; similarly named equivalent-radiant state belongs only
to outside-Surface diagnostics.

The adjacent `get_zone_air_set_points_compat` wrapper is an identity call
around an empty closure under a hard-coded PredictStep, not a live
GetZoneSetPoints evaluator. `EvaluateZoneThermostat` is execution-plan
metadata. The CLI IdealLoads path reads the first typed DualSetpoint and
repeats raw constant schedule values for thermostat outputs without operative
adjustment. Existing object and variable claims explicitly remain graph- or
case-bounded and do not establish control parity.

No Rust test covers an operative object, fixed/scheduled fraction, Zone MRT,
in-place conversion, caller order, or lifecycle. Generic schedules, surface
radiant fields, DualSetpoint graph wiring, and raw thermostat output evidence
do not implement CP227.

CP227 adds no algorithm-level `energyplus_source` entry, Rust target, code,
mapped state, test, support, capability, output implementation, comparator,
manifest, numerical, performance, or conformance promotion. The inventory
becomes 32 algorithms and 233 routines, split 58 `state_mapped` plus 175
`source_mapped`, with 110 required; the heat-balance project list becomes 79.

### CP228 `AdjustOperativeSetPointsforAdapComfort` source map

CP228 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.adjust_operative_set_points_for_adap_comfort`
and heat-balance project item
`adjust_operative_set_points_for_adap_comfort` immediately after
`adjust_air_set_points_for_op_temp_cntrl` and before
`update_final_surface_heat_balance`. The nonmember routine is declared at
`ZoneTempPredictorCorrector.hh` line 358 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5899-5964.

#### Entry order, guard, and environment dispatch

The function performs these operations in source order before selecting a
candidate:

1. aliases `state.dataZoneTempPredictorCorrector`;
2. indexes `TempControlledZone(TempControlledZoneID)`;
3. aliases the shared `AdapComfortDailySetPointSchedule`;
4. converts the incoming `Real64 ZoneAirSetPoint` to
   `int originZoneAirSetPoint`;
5. copies `AdaptiveComfortModelTypeIndex`; and
6. only then tests `AdaptiveComfortTempControl`.

A false flag returns after all six steps and makes no assignment to the
reference. For a finite, int-representable input this preserves the original
`Real64` exactly, including its fractional part. It is not an identity guard
for malformed state: an invalid record index can fail first, and C++
floating-to-integer conversion of NaN, infinity, or an out-of-range finite
value has undefined behavior before the return. CP228's only authored write
is to the referenced setpoint; it directly mutates no schedule, environment,
record, output registry, diagnostic, or latch.

When the flag is true, every environment kind except exact
`Constant::KindOfSim::DesignDay` and `HVACSizeDesignDay` takes the daily path.
That includes weather and design run periods and other enum values. The
routine switches on the copied model index and reads the one-based
`DayOfYear` cell from exactly one array:

| Model index | Adaptive model | Daily array | Summer design-day slot |
|---:|---|---|---:|
| 2 | ASH55 central | `ThermalComfortAdaptiveASH55_Central` | 0 |
| 3 | ASH55 90-percent upper | `ThermalComfortAdaptiveASH55_Upper_90` | 1 |
| 4 | ASH55 80-percent upper | `ThermalComfortAdaptiveASH55_Upper_80` | 2 |
| 5 | CEN15251 central | `ThermalComfortAdaptiveCEN15251_Central` | 3 |
| 6 | CEN15251 category I upper | `ThermalComfortAdaptiveCEN15251_Upper_I` | 4 |
| 7 | CEN15251 category II upper | `ThermalComfortAdaptiveCEN15251_Upper_II` | 5 |
| 8 | CEN15251 category III upper | `ThermalComfortAdaptiveCEN15251_Upper_III` | 6 |

The daily switch default performs no candidate assignment. Normal error-free
input activates only indices 2-8; unknown model text accumulates a Severe
error and reaches CP196's fatal tail, so the default is reachable only through
direct or corrupted active state. CP228 does not check the shared
`initialized` flag, array allocation, `DayOfYear`, or model validity before a
recognized case indexes its array.

For either design-day kind, the routine instead reads
`Environment(Envrn).DesignDayNum`, indexes `DesDayInput`, and tests the
literal summer day type 9. A nonsummer design day makes no candidate
assignment. A summer day indexes
`AdapComfortSetPointSummerDesDay[AdaptiveComfortModelTypeIndex - 2]`
without the daily switch, so a direct or corrupted model outside 2-8 can form
a negative or out-of-range subscript. There is no check that the shared
summer vector represents the current design day.

#### Integer baseline and final selection

The source intentionally or accidentally stores the original `Real64`
setpoint in an `int`; ordinary finite conversion truncates toward zero. Let
`x` be the incoming reference, `k = trunc(x)`, and `c` a selected candidate.
After the environment branch CP228 executes exactly:

1. if the current reference is lower than `k`, assign `k`;
2. if the resulting reference equals exact `-1`, assign `k`.

This is not a nondecreasing comparison against the original real value.
For a positive example, `x = 26.8` and `c = 26.5` retains 26.5 because it is
not below 26, while `c = 25` or `c = -1` returns 26.0 rather than 26.8.
A default-switch or nonsummer path leaves the reference at `x` before these
tests; typical positive `x` remains exact, but a negative fractional `x`
satisfies `x < trunc(x)` and is raised to the integer.

If the current reference remains exact `-1`, the second comparison assigns
`k` regardless of whether the first comparison fired. This need not eliminate
`-1` when `k` itself is `-1`. Candidate NaN makes both comparisons false and
is retained. Positive infinity is retained; negative infinity is lower than any
finite integer baseline and falls back. Those observations require the
original `x` itself to have converted validly. CP228 has no finite test,
candidate range validation, rounding correction, clamp, or independent
failure indicator.

#### Producer and input lifecycle

`ZoneTempControls` defaults the adaptive flag false and model index zero.
CP196 `GetZoneAirSetPoints` can set them only while processing
`ZoneControl:Thermostat:OperativeTemperature` with Constant or Scheduled
operative mode and a nonblank, recognized model other than None. Unknown
model text contributes a Severe error and the accumulated CP196 error reaches
its fatal tail. On a fresh record, None or a blank field leaves the defaults.
Assignments are set-only: a later object targeting the same expanded control
with None or blank does not explicitly clear an earlier
true flag or index.

The first active record under a false
`AdapComfortDailySetPointSchedule.initialized` latch allocates fresh ASH and
CEN running-average arrays, calls CP197
`CalculateMonthlyRunningAverageDryBulb`, then calls CP198
`CalculateAdaptiveComfortSetPointSchl`. A missing weather file can fatal in
CP197 even for a design-day-only use. CP198 owns these candidate semantics:

- daily ASH values are the three `0.31*T` formulas only for strict
  `10 < T < 33.5`, otherwise all three cells are `-1`;
- daily CEN values are the four `0.33*T` formulas only for strict
  `10 < T < 30`, otherwise all four cells are `-1`;
- every successful daily pass fills all days and commits the shared
  initialized latch only at its tail; and
- the seven design slots are shared across all summer design days, with the
  last qualifying day winning separately for ASH and CEN and no invalid
  branch clearing an earlier value.

The declaration
`std::array<Real64, 7> AdapComfortSetPointSummerDesDay = {-1}` initializes
only slot 0 to `-1`; slots 1-6 start at zero. CP228 therefore consumes an
asymmetric default vector when no qualifying producer write exists. It
performs no formula itself and does not distinguish a current design day from
the design day that last populated each family. The daily arrays, vector, and
latch reset only through full
`ZoneTempPredictorCorrectorData::clear_state` reconstruction, not at each
environment.

#### Parent order, snapshot, and cadence

There are exactly three production call expressions, all in
`CalcZoneAirTempSetPoints` and all wrapped by an outer
`AdaptiveComfortTempControl` test. Consequently normal production never uses
CP228's internal false guard.

| Parent branch | Order around CP228 | Result lifetime |
|---|---|---|
| `SingleCool` | sample raw cooling schedule, save raw `ZoneThermostatSetPointHi`, call CP228 on `setpt`, copy it to `setptAdapComfortCool`, then call CP227 and assign final high | humidity overcooling can change the later cooling target |
| `SingleHeatCool` | sample the shared schedule, call CP228 on `setpt`, snapshot it, call CP227, then assign both low and high | raw low/high record fields are not refreshed; optimum start can overwrite the later bounds |
| `DualHeatCool` | sample/save raw cooling high, call CP228 on high only, snapshot it, call CP227 on high, then sample/save and CP227-convert heating low | adaptive comfort never selects the heating low; optimum start and humidity control can change later values |

`SingleHeat` and `Uncontrolled` do not call CP228. The snapshot is therefore
an adaptive operative target, not CP227's air-temperature result. It backs
the Zone-timestep output
`Zone Adaptive Comfort Operative Temperature Set Point`, registered by
`InitZoneAirSetPoints` for every Zone. Begin-environment resets every
snapshot to zero, but a later SingleHeat or Uncontrolled control-type branch
does not refresh it, so a prior value can remain stale within the environment.
A false adaptive flag likewise skips refresh in direct or malformed state.
CP227, optimum start, humidity overcooling, later thermostat fault offsets,
comfort setpoint calculation, and EMS overrides do not update the snapshot.

The ordinary `ManageHVAC` chain reaches `GetZoneSetPoints` and this parent
once per Zone timestep before the system-substep loop. System timestep
shortening alone does not repeat CP228. DemandManager resimulation can add
same-time parent passes, while the external-HVAC route bypasses the built-in
caller. A full parent replay reloads the raw schedule before selecting the
same day/model candidate, so normal positive finite or `-1` behavior does not
compound. A direct call operates on its current reference; normal producer
values settle immediately, while malformed negative or nonfinite candidates
can expose a second truncation or undefined conversion. No warmup, sizing,
kickoff, occupancy, window-opening, or current-zone-condition gate exists
inside CP228.

#### Failure, retry, and reset

CP228 emits no authored warning, error, fatal, output, status, or exception
and owns no catch, cleanup, transaction, or rollback. Invalid
`TempControlledZoneID`, `Envrn`, `DesignDayNum`, recognized-model daily
array, `DayOfYear`, or summer-vector model state can abort or assert before
the candidate assignment; the initial floating-to-integer conversion can
already be undefined. In those preassignment cases the referenced setpoint
normally retains its entry value, but source behavior outside valid indexing
or conversion is not guaranteed.

A later-Zone parent failure leaves earlier Zone setpoints and adaptive
snapshots committed. Retrying the full parent reloads its processed raw
schedule prefix. CP228 itself has no persistent state or reset operation; its
candidate stores use the full owner reset described above, and the parent
begin-environment path separately clears the output snapshot.

#### C++ and corpus evidence

`ZoneTempPredictorCorrector_AdaptiveThermostat` is the only C++ test that
calls CP228 by name. It makes four direct calls after manually allocating
four records and setting every adaptive flag true:

- ASH55 central changes 0 to 25.55;
- CEN15251 central changes 0 to 27.05;
- a manually overwritten ASH central candidate of `-1` restores 0; and
- the same still-`-1` candidate restores 26.

The final call is therefore not a valid 25.55-below-26 floor test. All four
calls use a run-period environment. The fixture's IDF contains four Constant,
zero-radiative-fraction operative objects, including one model None. Its input
acquisition calls only `GetZoneData`, not CP196, and it never invokes the
production parent; it later calls CP198 and CP228 directly after freshly
allocating and manually activating four control records. It supplies no
parser/caller
integration evidence. The five upper-model selectors, internal false guard,
default switch, DesignDay and HVACSizeDesignDay summer/nonsummer paths,
fractional baseline truncation, independent second comparison, output
snapshot, failure, replay, and reset remain untested.

Four separate fixtures make 21 direct `CalcZoneAirTempSetPoints` calls. Their
branch mix evaluates CP228's outer guard 26 times: three in the optimum-start
fixture, 11 in the reporting fixture, and six in each cutout fixture. Every
record retains the default false adaptive flag, so the exact routine is
entered zero times.

The unit tree has 57 active full-simulation expressions after excluding five
commented calls. None contains
`ZoneControl:Thermostat:OperativeTemperature`; one expected EMS fatal stops
before `GetZoneSetPoints`, and the other 56 retain false adaptive flags.
Thus the full-simulation corpus has zero CP228 entries and zero daily,
design-day, integer-selection, or parent-snapshot reach. The installed oracle
has three ExampleFiles with an operative object and one with an adaptive
model, but no repository case or script references that example, so it is
not Rust runtime evidence.

#### Rust boundary

Authored Rust code and repository cases contain no CP228 implementation,
adaptive-comfort model/flag, daily adaptive schedule, summer design vector, or
adaptive operative output. Specifications contain only the new source-mapped
routine and project entries and no Rust target.
`ThermostatControlObjectType` exposes only DualSetpoint, and the compiler types
only ordinary `ThermostatSetpoint:DualSetpoint` and
`ZoneControl:Thermostat`. The operative-temperature object remains RawOnly;
without a partial rule it becomes a generic unsupported object and
run-blocks.

The closest time state supplies day-of-year and effective day-type state only
inside a weather run period. Its `SummerDesignDay` classification is
schedule/special-day state, not the source `DesDayInput(DesignDayNum).DayType`.
Rust has no DesignDay or HVACSizeDesignDay environment, `DesignDayNum`,
`SizingPeriod:DesignDay` record, or candidate stores. CP197 and CP198 remain
source-mapped with no Rust targets. The only live call site passes an empty
closure through the setpoint compatibility wrapper,
`EvaluateZoneThermostat` is planning metadata, and the narrow IdealLoads
diagnostic repeats only a referenced `Schedule:Constant` value without an
adaptive selection or snapshot. No Rust test covers any CP228 path.

CP228 therefore adds no algorithm-level `energyplus_source` entry, Rust
target, code, mapped state, test, support, capability, output implementation,
comparator, manifest, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 234 routines, split 58 `state_mapped`
plus 176 `source_mapped`, with 111 required; the heat-balance project list
becomes 80.

### CP229 `CalcZoneAirComfortSetPoints` source map

CP229 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.calc_zone_air_comfort_set_points`
and heat-balance project item `calc_zone_air_comfort_set_points` immediately
after `adjust_operative_set_points_for_adap_comfort` and before
`update_final_surface_heat_balance`. The nonmember
`void CalcZoneAirComfortSetPoints(EnergyPlusData &state)` is declared at
`ZoneTempPredictorCorrector.hh` line 360 and implemented at
`ZoneTempPredictorCorrector.cc` lines 5966-6329.

#### First-entry prefix and control-type sampling

Five locals are initialized once per function invocation, before the
comfort-Zone loop: `SetPointLo = 0`, `SetPointHi = 0`, `Tset = 0`,
`ObjectCount = 0`, and `PeopleCount = 0`. The two counts and `Tset` are not
reset at each Zone. This function-scope lifetime is observable in the later
OBJ and PEO branches.

When `CalcZoneAirComfortSetPointsFirstTimeFlag` is true, CP229 first calls
`ThermalComfort::ManageThermalComfort(state, true)` and clears its own flag
only after normal return. The child independently calls
`InitThermalComfort` under its own first-time latch, allocating People comfort
state and registering requested comfort outputs. Even with
`InitializeOnly = true`, it updates `TemporarySixAMTemperature`: day one
before hour 7 writes the literal 1.868132, and timestep one of hour 7 samples
outdoor dry-bulb. It then returns before the ordinary thermal-comfort
calculation block. CP229 does not read the comfort-control input here; CP196
has already created those records.

After first-use work, CP229 assigns the entire `ComfortControlType` array to
Uncontrolled. It does not reset `ComfortControlTypeRpt`, Fanger records,
setpoints, or diagnostic indexes. A direct zero-comfort-Zone call therefore
can still initialize ThermalComfort and perform this array reset. The
production parent instead calls CP229 only when
`NumComfortControlledZones > 0`.

For each stored comfort record in ascending relative order, CP229 aliases the
record, its `ActualZoneNum`, the actual Zone, `zoneTstatSetpts`, and
`ZoneComfortControlsFanger`. It samples `setptTypeSched`, converts the
`Real64` to `HVAC::SetptType`, writes the actual-Zone control array, and copies
its integer value to `ComfortControlTypeRpt`. Normal input validates only the
schedule extrema in inclusive `[0,4]`; it does not impose an integer-valued
runtime sample. A finite fractional value follows the floating-to-underlying
integer conversion, while a nonfinite or unrepresentable value enters
undefined conversion territory before either switch.

#### PMV control dispatch and diagnostics

The first control-type switch updates PMV/Fanger state as follows:

| Control type | Fanger state and schedule work |
|---|---|
| Uncontrolled | writes `LowPMV = HighPMV = -999` but leaves `FangerType` stale |
| SingleHeat | writes type 1, samples the heating PMV schedule into `LowPMV`, and writes `HighPMV = -999` |
| SingleCool | writes type 2, writes `LowPMV = -999`, and samples the cooling PMV schedule into `HighPMV` |
| SingleHeatCool | writes type 3 and samples its cooling pointer once into both PMV fields; CP196 binds both pointers to the same schedule |
| DualHeatCool | writes type 4 and samples heating then cooling PMV schedules |
| default | emits a Severe whose text incorrectly names `CalcZoneAirTempSetpoints`, leaves prior PMV/Fanger state in place, and continues |

For Dual control, strict `LowPMV > HighPMV` increments the persistent
`DualPMVErrCount`. Count one emits an immediate warning, continuation, and
timestamp; later counts call the recurring warning through
`DualPMVErrIndex`. Both recurring numeric arguments are the pre-correction
low value. CP229 then assigns `LowPMV = HighPMV`; equality is accepted.
Normal CP196 input validates active PMV schedule extrema within `[-3,3]`.

An invalid control value does not set a local error flag or stop the record.
It can therefore run the averaging switch with stale PMV fields and later
reach a second Severe in the assignment switch.

#### People selection and averaging

`AverageMethod::NO` and `SPE` have identical runtime code. Both use
`SpecificObjectNum`; SingleCool passes `HighPMV` to the child and every other
control value passes `LowPMV` into `SetPointLo`. Dual makes a second call with
`HighPMV` into `SetPointHi`. CP196 assigns NO automatically when exactly one
People object belongs to the Zone. With multiple objects, SPE resolves its
name against the complete People arena but does not verify that the selected
People record belongs to the controlled Zone.

OBJ resets only the two sums. It scans every People record whose `ZonePtr`
matches the actual Zone, increments the function-scope `ObjectCount`, calls
`GetComfortSetPoints` with `LowPMV`, and accumulates `Tset`; Dual additionally
calls and accumulates the high result. It then divides the Zone-local
numerators by the cumulative object count. Consequently a later OBJ Zone is
biased by objects visited in prior Zones or prior PEO fallbacks. Unlike
NO/SPE, OBJ has no SingleCool special case and passes its `LowPMV = -999`
sentinel, normally selecting the minimum dry-bulb bound instead of the
cooling PMV.

PEO also resets only the sums. For each matching People record it computes

`int NumberOccupants = NumberOfPeople * current_people_schedule`

with toward-zero conversion, adds that integer to the function-scope
`Real64 PeopleCount`, calls the child even for zero occupants, and accumulates
the low result weighted by the integer. Dual repeats the call and weighting
for the high result. PEO also lacks the SingleCool special case and therefore
uses `LowPMV = -999`.

A positive cumulative `PeopleCount` divides the current Zone's numerators by
all occupant integers seen earlier in the invocation. A prior occupied Zone
can therefore suppress the warning and fallback for a currently empty Zone
and yield zero from a zero numerator. When the cumulative count is not
positive, CP229 emits first/recurring zero-People diagnostics and re-runs the
Zone as an object average. That fallback calls each child a second time and
uses the same non-reset `ObjectCount`, so prior OBJ or fallback Zones also
bias its denominator.

For a Zone with `N` People records, NO/SPE make one child call, or two for
Dual. OBJ and positive PEO make `N`, or `2N` for Dual. A nonpositive PEO
weighted pass plus fallback makes `2N`, or `4N` for Dual. Uncontrolled still
executes the selected averaging method with sentinel PMV and discards the
temperature in its final branch. An invalid averaging enum performs no work,
so an active final control branch can consume function-scope values left by a
prior Zone.

CP230 `GetComfortSetPoints` owns the PMV-to-dry-bulb inverse. Relevant to
CP229's reference lifetime, its comparisons are strict: with ordered or equal
endpoint PMVs, exact equality preserves the output; a NaN target or any other
state making all three comparisons false also leaves it untouched. Reversed
endpoint equality can instead select the opposite temperature bound. Thus
`SetPointLo`,
`SetPointHi`, or shared `Tset` can retain zero or a result from an earlier
People object or Zone. The child also updates Fanger/People comfort scratch
and report state on its endpoint and root trials; CP229 is not a pure
selector. CP230's full root-solver and diagnostic boundary remains the next
checkpoint.

#### Dry-bulb assignment and ordinary-control overwrite

The final switch uses the calculated locals as follows:

| Control type | Clamp, write, and retained state |
|---|---|
| Uncontrolled | if ordinary `TempControlType` is SingleHeat, clears only `setptHi`; if SingleCool, clears only `setptLo`; otherwise writes nothing and preserves the ordinary type/report |
| SingleHeat | clamps only below `TdbMinSetPoint`, writes scalar `setpt` and `setptLo`, leaves `setptHi` stale, and overwrites ordinary type/report with SingleHeat |
| SingleCool | clamps only above `TdbMaxSetPoint`, writes scalar `setpt` and `setptHi`, leaves `setptLo` stale, and overwrites type/report with SingleCool |
| SingleHeatCool | forces equality when min equals max, then clamps high and low, writes scalar plus both bounds, and overwrites type/report |
| DualHeatCool | clamps low only below min and high only above max, writes both bounds, leaves scalar `setpt` stale, and overwrites type/report with Dual |
| default | emits a second Severe, now naming `CalcZoneAirComfortSetpoints`, and performs no assignment |

SingleHeat's immediate-warning gate is literally `TdbMinErrIndex < 2`, unlike
the other zero-index tests; every violation also calls its recurring warning
with the already clamped minimum as both numeric arguments. SingleCool is
analogous at the maximum with a zero-index immediate gate. The
SingleHeatCool range diagnostic is checked only after both clamps, so it is
unreachable for ordinary finite ordered bounds. Dual owns separate low and
high first/recurring diagnostic indexes, but the high recurring call
mistakenly supplies `SetPointLo` as both numeric arguments.

SingleHeat has no upper clamp, SingleCool has no lower clamp, and Dual neither
clamps low downward from above max nor high upward from below min. CP229 never
rechecks `SetPointLo <= SetPointHi` after independent PMV inversion and
clamping. NaN can bypass all comparison-based clamps.

#### Input lifecycle, parent order, outputs, and cadence

CP196 requires at least one People object for every expanded comfort Zone and
rejects duplicate comfort control of a Zone. It validates each dry-bulb bound
within `[0,50]`, rejects min greater than max, checks the control schedule
range `[0,4]`, and checks PMV schedules within `[-3,3]`; accumulated input
errors reach its fatal tail. Equal dry-bulb bounds emit a Severe in the shown
branch without setting its local `ErrorsFound`, and CP229 can proceed if no
other error exists. With multiple People, CP196 selects SPE, OBJ, or PEO from
input; with one People it forces NO regardless of the authored averaging
field. Normal validation removes many malformed zero-object paths but does
not remove fractional control samples, SPE cross-Zone selection, cross-Zone
counter carry, or child endpoint equality.

There is exactly one production call expression, in
`CalcZoneAirTempSetPoints` after all ordinary control branches, CP228/CP227,
optimum-start and humidity adjustments, and ordinary thermostat fault
offsets. Active comfort control can overwrite those ordinary temperature
setpoints and `TempControlType`. The parent then unconditionally calls
`OverrideAirSetPointsforEMSCntrl`, which has final setpoint precedence. The
earlier CP199 initialization also performs comfort demand-manager processing
before this runtime calculation.

CP199 allocates the comfort arrays and registers three Zone-timestep average
outputs for each comfort-controlled Zone:

- `Zone Thermal Comfort Control Type`;
- `Zone Thermal Comfort Control Fanger Low Setpoint PMV`; and
- `Zone Thermal Comfort Control Fanger High Setpoint PMV`.

CP229 refreshes the type/report and PMV fields only for records it visits.
Ordinary HVAC execution reaches the parent through
`ManageZoneAirUpdates(GetZoneSetPoints)` once per Zone timestep before system
substeps. DemandManager resimulation can add same-time passes; external HVAC
bypasses the built-in caller. CP229 itself has no warmup, sizing, kickoff,
occupancy, environment, or current-zone-condition guard.

#### Failure, retry, and reset

CP229 returns no status and owns no catch, cleanup, transaction, or rollback.
The authored invalid-control path emits Severe messages but continues.
Invalid pointers, identities, arrays, People state, schedules, psychrometric
state, or a failing child can stop after some type, PMV, diagnostic, Fanger,
People, or setpoint writes.

If first-use `ManageThermalComfort` does not return, CP229's latch remains
true; the child may already have partial allocations or output registrations
and its separately placed flag can have changed. After a successful child
return, the CP229 latch is false before the global control-type reset and Zone
loop. A later-Zone failure therefore leaves earlier Zone setpoints and
comfort state committed, the failing Zone at an arbitrary prefix, and
unvisited `ComfortControlType` entries Uncontrolled while their report and
Fanger fields can remain stale.

Same-state retry after successful first-use work skips that work, recreates
all five locals, and resets the control-type array again. Cross-Zone count
bias therefore restarts at zero on each invocation rather than accumulating
between calls, while persistent warning counts/indexes, Fanger/People trial
state, report fields, and stale setpoint fields survive and can advance or be
overwritten again. Begin-environment initialization clears ordinary setpoint
and control fields but does not rearm the CP229 latch or all comfort warnings.
A clean replay requires coordinated reconstruction of the
ZoneTempPredictorCorrector, HeatBalFanSys, DataZoneControls, ThermalComfort,
People/schedule, output, and dependent environment owners.

#### C++ and corpus evidence

No C++ unit test calls CP229 or CP230 by name. Four fixtures make 21 direct
`CalcZoneAirTempSetPoints` calls, but none creates comfort-control state and
all 21 skip the `NumComfortControlledZones > 0` guard. A separate EMS fixture
manually sets the comfort count to one but calls only
`OverrideAirSetPointsforEMSCntrl`, asserting its low/high override.

Only two unit-tree IDF setup fixtures contain thermal-comfort input. Each has
one comfort thermostat with SingleHeating and SingleCooling objects, but one
stops after `GetHeatBalanceInput` and the other performs input, geometry, and
Surface setup; neither calls CP196, the setpoint parent, or CP229. Separate
ThermalComfort tests numerically exercise the forward Fanger model, not the
comfort thermostat, PMV inverse, averaging, clamps, assignment, or
ordinary-comfort-EMS order.

The unit tree has 57 active full-simulation expressions after excluding five
commented calls. None contains a thermal-comfort thermostat or setpoint
object, so every full-simulation configuration has zero CP229 entries and
zero CP230 child reach. The installed oracle has one executable ExampleFile,
`FurnaceWithDXSystemComfortControl.idf`, with control values 0-4 and all four
Fanger setpoint families. Its comfort-controlled EAST Zone has one People
record, so CP196 forces NO despite the authored PeopleAverage field. No
repository case or script adopts
that file; it is an unexecuted candidate, not Rust evidence. OBJ, SPE, PEO,
invalid controls, counter carry, clamps, diagnostics, failure, retry, and
reset remain uncovered.

#### Rust boundary

Crate-wide authored Rust code contains no CP229 or CP230 implementation,
thermal-comfort thermostat/setpoint type, PMV/Fanger record, averaging method,
dry-bulb comfort bounds, first-use latch, three comfort outputs, diagnostic
state, or live caller. `ThermostatControlObjectType` and the compiler retain
only ordinary direct-Zone DualSetpoint controls. The setpoint compatibility
wrapper's only heat-balance call receives an empty closure, and
`EvaluateZoneThermostat` remains planning metadata.

Rust's typed `People` record retains only Zone identity, design-count fields,
and an optional number schedule. Its current consumers use occupancy for
OtherEquipment sizing and bounded IdealLoads outdoor-air/DCV work, not
activity, work efficiency, clothing, air velocity, Fanger inversion, object
averaging, or CP229's integer occupant weighting. The ordinary control
schedule is not evaluated by the heat-balance runtime.

`ZoneControl:Thermostat:ThermalComfort` and all four
`ThermostatSetpoint:ThermalComfort:Fanger:*` families remain RawOnly without a
partial-support rule. Active input becomes a generic unsupported object and
run-blocks before Rust runtime. Existing source-mapped
`routine.manage_thermal_comfort`, forward-adjacent People/schedule state, and
ordinary thermostat graph evidence do not promote CP229.

CP229 therefore adds no algorithm-level `energyplus_source` entry, Rust
target, code, mapped state, test, support, capability, output implementation,
comparator, manifest, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 235 routines, split 58 `state_mapped`
plus 177 `source_mapped`, with 112 required; the heat-balance project list
becomes 81.

### CP230 `GetComfortSetPoints` source map

CP230 adds required `source_mapped`
`zone_temp_predictor_corrector_source_order.routine.get_comfort_set_points`
and heat-balance project item `get_comfort_set_points` immediately after
`calc_zone_air_comfort_set_points` and before
`update_final_surface_heat_balance`. The nonmember
`void GetComfortSetPoints(EnergyPlusData &state, int PeopleNum,
int ComfortControlNum, Real64 PMVSet, Real64 &Tset)` is declared at
`ZoneTempPredictorCorrector.hh` lines 362-367 and implemented at
`ZoneTempPredictorCorrector.cc` lines 6331-6415.

#### Endpoint evaluation and strict dispatch

`PeopleNum`, `ComfortControlNum`, and `PMVSet` are copied arguments; only
`Tset` is a writable reference. CP230 initializes `PMVResult = 0`, indexes
`ComfortControlledZone(ComfortControlNum)`, and snapshots its
`TdbMinSetPoint` and `TdbMaxSetPoint` as `Tmin` and `Tmax`. It then calls
`CalcThermalComfortFanger(PeopleNum, Tmin, PMVResult)`, copies `PMVMin`,
calls the child again at `Tmax`, and copies `PMVMax`. Both forward
evaluations therefore occur before every clamp, root, or no-write result.

The branch order is literal and strict:

- only `PMVSet > PMVMin && PMVSet < PMVMax` enters the root solver;
- otherwise `PMVSet < PMVMin` writes `Tset = Tmin`;
- otherwise `PMVSet > PMVMax` writes `Tset = Tmax`;
- every remaining case returns without writing `Tset`.

With ordered or equal endpoint PMVs, exact equality with either endpoint
preserves the incoming reference. With reversed PMVs, equality to the
numerically higher `PMVMin` can select `Tmax`, while equality to the lower
`PMVMax` can select `Tmin` through the later branches. A NaN target makes all
three comparisons false. With ordinary finite endpoints, positive infinity
selects `Tmax` and negative infinity selects `Tmin`. An endpoint NaN makes
only comparisons involving that value unordered, so an earlier finite
lower-bound comparison can still select `Tmin`.

CP230 assumes PMV rises from the lower to the upper dry-bulb bound but does
not validate or reorder temperatures or PMVs. Reversed or equal endpoints
therefore follow the shown branch precedence rather than a mathematical
inverse contract. CP196 can let equal dry-bulb bounds proceed after its
non-sticky Severe branch; deterministic equal endpoint PMVs then make a
target below or above select the same temperature, while exact equality
preserves stale `Tset`. The source comment describing a returned 0/1/2
solution classification is stale: this `void` routine returns or records no
such result.

The last unconditional endpoint evaluation is at `Tmax`. A lower clamp can
therefore return `Tmin` while transitive People/Fanger report state still
describes `Tmax`; the upper clamp matches that last endpoint, while ordered-
endpoint equality and other no-write cases leave both a stale caller value
and the `Tmax`-side child state.

#### Root-solver contract and forward-call multiplicity

The interior branch constructs a callback returning
`PMVSet - PMV(candidate)` and calls
`General::SolveRoot(state, 0.001, 500, SolFla, Tset, callback, Tmin, Tmax)`.
The tolerance is an absolute PMV residual, not a temperature tolerance.
CP230 does not select a root method: `SolveRoot` reads
`state.dataRootFinder->rootAlgo`. Its default is Regula Falsi, and the
`HVACSystemRootFindingAlgorithm` input can choose Regula Falsi, Bisection,
Regula Falsi then Bisection, Bisection then Regula Falsi, or Alternation.
`RootAlgo` also has an internal short-Bisection-then-Regula-Falsi value used
by other source paths; CP230 honors it if that shared state is present. The
global configured iteration-switch count affects the two ordered hybrids and
Alternation. The internal short mode instead fixes its first three candidates
to Bisection. CP230's maximum remains the literal 500.

Before each method choice, the solver computes `DY = Y0 - Y1`. If
`abs(DY) < 1e-10`, it replaces that value with positive `1e-10` without
preserving the original sign. Regula Falsi estimates are not clamped to the
current interval, so tiny residual separation or malformed state can produce
an outside-bracket candidate.

`SolveRoot` calls the callback at `Tmin` and `Tmax` again before iteration.
Thus an interior attempt has four forward Fanger evaluations before its first
candidate. Same-sign endpoint residual product greater than zero returns
flag `-2` with `Tset = Tmin`. Otherwise the solver seeds the result with
`Tmin`, generates candidates according to the configured method, and tests
strict `abs(residual) < 0.001`.

Each candidate is evaluated before the counter is incremented; convergence is
checked before `NIte > MaxIte`, and the limit comparison is strict. A 501st
candidate can therefore succeed with positive flag 501; if it does not,
flag `-1` retains that last evaluated candidate. An interval initially narrower than
`1e-10` exits with `-1` before any candidate and retains the seeded `Tmin`.
The routine consequently performs:

- two Fanger evaluations for a clamp, equality, or no-write path;
- four for a solver `-2` or a zero-candidate narrow-interval failure;
- `4 + k` for a normal-width interior attempt with `1 <= k <= 501`, hence
  five through 505 total evaluations.

For deterministic finite ascending endpoint values, the strict interior
precheck implies opposite residual signs, so `-2` is normally unreachable.
Repeated-evaluation side effects, malformed or nonfinite state, and direct
misuse keep it part of the observable contract. Endpoint residuals are not
accepted as roots before the candidate loop. Overflow, underflow, or NaN in
the residual product can also alter bracketing behavior.

A positive solver flag is accepted silently. An iterated `-1` leaves the last
evaluated candidate in `Tset`; a zero-candidate width `-1` and flag `-2`
leave `Tmin`. On a normal successful or iterated `-1` path, child report
state normally reflects the returned last candidate. On `-2`, or a
zero-candidate width exit, the solver's last forward call was at `Tmax` while
the returned reference is `Tmin`.

#### Transitive Fanger state and dependencies

The optional-`PeopleNum` call mode of `CalcThermalComfortFanger` is an
impure evaluator, not a numerical callback. Each invocation loops the full
People arena, skips nonmatching records without an early break, and evaluates
the matching record regardless of its ordinary `Fanger` reporting flag. The
shared ThermalComfort `PeopleNum` is itself the loop counter, so normal return
leaves it at `TotPeople + 1`, or one for an empty arena, even when no record
matches. A match copies that People record's Zone identity and samples
activity, work efficiency, clothing, and air-velocity schedules on every
endpoint or candidate.

In the ordinary mixed-air path, the trial value supplies air temperature.
Displacement ventilation and UFAD instead replace it with `TCMF`; Cross
Ventilation Jet and the literal Recirculation branch use `ZTJET`. These
room-air overrides can ignore the trial, make endpoint PMVs equal, or flatten
the root response. Relative humidity for comfort-control evaluation is
calculated from the Zone MAT rather than the candidate temperature, together
with `airHumRatAvgComf` and barometric pressure. The child also reads mean
radiant temperature and radiant-to-person state.

SurfaceWeighted MRT adds a stateful endpoint anomaly. Its first call clears
`FirstTimeSurfaceWeightedFlag`, rearms `FirstTimeError`, initializes every
radiant-enclosure Surface `AE` and every excluded-surface `enclAESum`, and a
true enclosure `radReCalc` can rewrite the selected sum and member `AE`
values again. If the selected `enclAESum <= 0.01`, that first call warns,
clears `FirstTimeError`, seeds MRT from the Space MAT, and then applies the
default half-surface-temperature average; later calls on the same bad sum
return the local zero instead. Outer and solver-repeated endpoints can
therefore observe different MRT/PMV state, providing a concrete path to `-2`
after an initially valid strict bracket.

Every successful forward evaluation can overwrite:

- shared ThermalComfort scratch for selected People/Zone identity,
  air/radiant temperature, relative humidity, schedules, Fanger coefficients,
  losses, clothing iteration, and PMV intermediates;
- selected People `TemperatureInZone` and `RelativeHumidityInZone`;
- selected `ThermalComfortData` Fanger PMV, PPD, MRT, operative temperature,
  clothing-surface temperature, and clothing value;
- `HeatBalFanSys::ZoneQdotRadHVACToPerson` through the MRT calculation;
- Surface `AE`/`enclAESum` plus ThermalComfort first-use/error latches in the
  SurfaceWeighted path;
- per-People air-velocity diagnostic state and other transitive diagnostics.

An air-velocity sample outside the child's accepted range can warn or advance
its recurring index once per forward evaluation, including duplicated
endpoints, every root candidate, and warmup. Clothing and psychrometric work
can also diagnose. CP230's warmup guard does not suppress any of these child
effects.

An unmatched `PeopleNum` does not directly index that number: the child scans
the arena, selects nothing, and leaves CP230's freshly zeroed `PMVResult`
unchanged at each endpoint. Both PMV bounds then become zero, so a negative
target chooses `Tmin`, a positive target chooses `Tmax`, and zero preserves
the incoming reference. A direct call before the required ThermalComfort and
People arrays exist can fail earlier. Normal SPE input can select a globally
named People record from another Zone, so comfort bounds and diagnostic name
come from `ComfortControlNum` while the forward conditions come from that
People record's Zone.

#### Solver diagnostics and persistent ownership

After `SolveRoot` returns, CP230 handles only flags `-1` and `-2`. Outside
warmup, `-1` increments global
`ZoneTempPredictorCorrectorData::IterLimitExceededNum1`. Count one emits an
immediate iteration-limit warning using the selected comfort-control record
`Name`; later counts use the shared `IterLimitErrIndex1` recurring warning
with `Tset` as both numeric arguments. Flag `-2` analogously uses
`IterLimitExceededNum2` and `IterLimitErrIndex2` and says the minimum
temperature setpoint was used. There is no timestamp continuation.

The immediate and recurring text is not normalized. The `-1` recurring text
drops `Fanger` and preserves two spaces after the record-name colon. The `-2`
recurring text omits the minimum-setpoint-used clause and preserves two spaces
both after the colon and between `in` and `calculating`. Exact diagnostic
parity must retain those literal differences.

These four fields are shared across every comfort Zone and People record,
not stored per comfort record. Later Zones can therefore inherit the
first-occurrence state and aggregate under one recurring identity. During
warmup, CP230 still performs all forward/root work and returns the selected
or failed `Tset`, but it neither increments these counters nor emits these
two diagnostic families. The fields have no environment reset and return to
zero only when the ZoneTempPredictorCorrector owner is reconstructed or
cleared. CP230 registers no output variable of its own.

#### Parent order, call cardinality, and cadence

All 12 production call expressions are inside CP229
`CalcZoneAirComfortSetPoints`; no other production routine calls CP230.
NO and SPE each call the selected People record once for a non-Dual control
and low then high for Dual. OBJ visits matching People in ascending global
order and calls low, then high per People for Dual. PEO has the same order and
cardinality even when the integer occupant weight is zero. Its nonpositive
cumulative-weight fallback repeats the complete object-average pass.

For a Zone with `N` matching People records, runtime call counts are therefore
one or two for NO/SPE, `N` or `2N` for OBJ and positive PEO, and `2N` or
`4N` for PEO plus fallback. Each of those calls can itself perform two, four,
or five through 505 forward Fanger evaluations. NO/SPE SingleCool passes
`HighPMV`
into its low output; OBJ and PEO pass the `LowPMV = -999` sentinel. An
Uncontrolled or malformed control value can still call CP230 when the
averaging enum chooses a branch. With ordered or equal endpoint PMVs,
endpoint equality preserves NO/SPE output locals or the shared OBJ/PEO
`Tset`, allowing prior People or Zone values to be accumulated; reversed
equality can instead select the opposite temperature bound.

The parent runs after ordinary thermostat selection, operative/adaptive,
optimum-start, humidity, and fault work and before the final EMS override.
Built-in execution normally reaches it once per Zone timestep before HVAC
system substeps. Demand-manager resimulation can repeat the setpoint parent at
the same time; external HVAC can bypass it. CP230 has no independent cadence,
environment, sizing, warmup, occupancy, or control-type guard.

#### Malformed state, aliasing, failure, and retry

CP230 has no identity, range, finite, allocation, pointer, or alias checks.
An invalid comfort-record index fails before the endpoint work. The writable
reference can alias arbitrary `Real64` state in a direct call. CP230 snapshots
both bounds and receives PMV by value before its own writes, but forward
children can mutate other aliased state; normal CP229 calls pass only local
temperatures.

NaN generated inside `SolveRoot` bypasses ordinary same-sign and convergence
comparisons and can propagate through the configured algorithm until `-1`,
possibly leaving a NaN result. A failed first solver endpoint callback occurs
before the solver seeds `Tset`; failure during later candidate work occurs
after it has seeded `Tmin`. More generally, CP230 is `void` and owns no
status, catch, cleanup, transaction, rollback, or one-time latch. A non-return
can leave any prefix of Fanger scratch, People/report fields, radiant state,
or diagnostics, with either the incoming output, `Tmin`, or a last candidate.

Same-state retry repeats both outer endpoints and every reached solver
evaluation. It can advance per-People child warnings and, outside warmup,
CP230's global failure counters. A deterministic retry normally recalculates
from the snapshotted bounds, but schedules, dynamic clothing, room-air state,
diagnostics, and reports already mutated by the first attempt remain inputs or
persistent effects. An ordered-endpoint equality/no-write retry also preserves
whatever `Tset` the caller now supplies. Clean replay requires coordinated
reconstruction of ZoneTempPredictorCorrector, DataZoneControls, RootFinding,
ThermalComfort, HeatBalance/People, HeatBalFanSys, Surface, Construction,
ViewFactor, HeatBalSurf, RoomAir, schedules, environment/psychrometric,
output, and diagnostic owners.

#### C++ tests, corpus, and oracle evidence

No C++ unit test calls CP230 directly or reaches it indirectly. The four
parent fixtures make 21 direct `CalcZoneAirTempSetPoints` calls, all with the
comfort guard false. Two raw thermal-comfort setup fixtures stop before CP196
or the setpoint parent. The 57 active full-simulation configurations contain
no thermal-comfort thermostat or Fanger setpoint object, so their production
CP230 reach is zero.

Forward-model and generic-solver evidence remains separate. Six direct
`CalcThermalComfortFanger` test expressions include five ordinary calls that
assert selected PMV/PPD or averaged conditions. The sole call supplying optional `PeopleNum` as CP230 does asserts only
clothing value, not the returned
PMV. Two lower-level `CalcFangerPMV` cases are numeric. Ten direct generic
`SolveRoot` test calls cover successes and `-1`; none asserts `-2`, and none
composes the solver with Fanger or CP230.

A manual stock-26.1 oracle run of the installed
`FurnaceWithDXSystemComfortControl.idf` completed its winter and summer
DesignDays at six timesteps per hour with zero Warning and zero Severe. Its
comfort-controlled EAST Zone has one People record and therefore forces NO
averaging despite the authored PeopleAverage choice. Under its six warmup
days plus one reported day per environment,
schedule/cadence arithmetic gives 2,226 CP230 calls: 1,176 Uncontrolled,
210 each for control types 1, 2, and 3, and 420 Dual child calls. The reported
days account for 318 calls, including 168 sentinel and 150 active-target
calls. Reported active setpoints stayed inside the authored `[12.8,40]`
bounds and no CP230 solver warning appeared.

That count is source-and-schedule arithmetic for the completed run, not an
instrumented function counter or per-call iteration trace. The ExampleFile is
not copied into a repository case or script, and no Rust result is compared
with it. It is manual diagnostic evidence only, not checked-in test,
numerical-conformance, or runtime-support evidence.

#### Rust boundary

Crate-wide authored Rust code contains no CP230 routine, comfort thermostat
or Fanger setpoint type, PMV/Fanger state, activity/work/clothing/air-velocity
comfort inputs, inverse residual callback, configurable generic root solver,
solver flags, four diagnostic counters/indexes, report side effects, or live
caller. The setpoint compatibility wrapper still receives an empty closure.

Rust's typed People state retains Zone identity, design-count fields, and an
optional number schedule for sizing and bounded IdealLoads outdoor-air/DCV
work. Ordinary thermostat support is limited to direct-Zone DualSetpoint
state. Thermal-comfort control and Fanger setpoint families remain RawOnly
without a partial-support rule and run-block before runtime.

The private fixed-bracket bisection used by one IdealLoads outdoor-air
psychrometric helper has different bounds, iteration policy, callback/state,
status, and diagnostics. It is not evidence for CP230 or the configurable
EnergyPlus root solver. Existing forward-adjacent People/schedule state,
ordinary thermostat graph evidence, and source-mapped ThermalComfort parent
also do not promote this inverse.

CP230 therefore adds no algorithm-level `energyplus_source` entry, Rust
target, code, mapped state, test, support, capability, output implementation,
comparator, manifest, numerical, performance, or conformance promotion. The
inventory becomes 32 algorithms and 236 routines, split 58 `state_mapped`
plus 178 `source_mapped`, with 113 required; the heat-balance project list
becomes 82.

### CP231 `AdjustCoolingSetPointforTempAndHumidityControl` source map

CP231 adds canonical required
`routine.adjust_cooling_set_point_for_temp_and_humidity_control` and the
project-contract item
`adjust_cooling_set_point_for_temp_and_humidity_control` immediately after
CP230 `get_comfort_set_points`. The pinned source declaration is
`ZoneTempPredictorCorrector.hh` lines 369-372, and the complete definition is
`ZoneTempPredictorCorrector.cc` lines 6417-6458.

The routine conditionally lowers one cooling high setpoint from current Zone
relative humidity, a dehumidifying schedule, a constant or scheduled maximum
overcool range, and a percent-RH-per-kelvin ratio. It is a source boundary
only. No Rust implementation, typed input, state promotion, or conformance
claim is inferred.

#### Entry aliases, guards, and cross-index identity

The body first binds three references in this exact order:

1. the ZoneTempPredictorCorrector owner;
2. `TempControlledZone(TempControlledZoneID)`;
3. `zoneTstatSetpts(ActualZoneNum)`.

Only then does it test `AnyZoneTempAndHumidityControl`. A false global flag
therefore does not protect a bad temperature-control index, actual-Zone
index, unallocated record arena, or missing DataZoneControls/HeatBalFanSys
owner. The second quick return is
only exact `OvercoolCtrl == TempCtrl::None`. The record constructor default is
`Invalid`, not None, so default state proceeds into the constant-range path.

There is no assertion or comparison between
`tempZone.ActualZoneNum` and `ActualZoneNum`. A direct caller can therefore
combine the mode, range, ratio, and schedules of one temperature-control
record with another Zone's thermostat bounds and
`zoneHeatBalance(...).airRelHum`. The normal parent supplies matching
identities, but the routine does not establish that invariant itself.

The global flag means only that at least one raw
`ZoneControl:Thermostat:TemperatureAndHumidity` object was counted. It does
not say the current record is targeted, active, valid, or fully bound. It is
set before object validation and has no environment reset; only owner clear
restores false.

#### Range selection, humidity cap, and sole write

After both guards, the routine performs this ordered calculation:

1. exact Scheduled mode samples `zoneOvercoolRangeSched->getCurrentVal()`;
   every other enum value, including Invalid, copies
   `ZoneOvercoolConstRange`;
2. it copies `ZoneOvercoolControlRatio`;
3. it computes `setptHi - setptLo` and, only when that gap is strictly
   positive, replaces range with `min(range, gap)`;
4. it samples `dehumidifyingSched->getCurrentVal()` and computes
   `zoneHeatBalance(ActualZoneNum).airRelHum - schedule`;
5. only when both humidity excess and ratio are strictly positive, it replaces
   range with `min(range, excess / ratio)` and executes
   `zoneTstatSetpt.setptHi -= range`.

The dehumidifying schedule is therefore sampled for every non-None mode even
when range or ratio is zero, the gap is invalid, or RH is already expected to
be below target. Constant and Invalid modes do not touch the range-schedule
pointer. Schedule values are current values, including any schedule-level
override already present; CP231 does not consume the separate
`ZoneControl:Humidistat` EMS or fault-adjusted setpoint.

The final subtraction is the only write. CP231 does not modify scalar
`setpt`, `setptLo`, `TempControlledZone.ZoneThermostatSetPointHi`, the stored
range or ratio, an RH field, a report status, or a diagnostic counter. It
registers no output. When the strict final gates pass with a zero range, the
assignment still executes as subtraction of zero.

Although the source comment describes a Dual-setpoint cap, the calculation
has no control-type branch and applies the gap to SingleCool as well. The
SingleCool parent has just refreshed scalar and high but has not refreshed
low, so a default or stale low is part of the cap. A zero or negative gap
skips the cap completely; a reversed Dual deadband can therefore be lowered
farther. A requested nonzero subtraction changes the gap only when its
floating-point result differs; rounding or infinities can preserve it, while
no-write and zero-range paths leave it unchanged.

#### Bounds, units, and nonfinite behavior

Normal input describes `airRelHum` and the dehumidifying schedule in percent,
the ratio in percent/K, and range in delta C. Thus
`humidity_excess / ratio` is a temperature reduction. The Constant producer
path validates range inclusively from 0 through 3. A Scheduled branch that
obtains a schedule pointer attempts the same all-values check, while ratio has
only a zero lower bound. A zero ratio is valid input and disables the final
write after the dehumidifying schedule has already been sampled.

CP231 itself has no range, finite, unit, monotonicity, deadband, or pointer
validation. Its strict comparisons reject zero, negative values, and NaN at
the gap and final gates, while positive infinity passes. A malformed negative
range remains the minimum against a positive candidate and makes subtraction
raise the cooling setpoint. A positive range with zero or reversed deadband is
not capped and can cross the heating bound.

The imported ObjexxFCL double `min(a, b)` is implemented as
`a < b ? a : b`; equality and unordered comparison choose the second
argument. Consequently a NaN first range can be replaced by a finite positive
gap or RH candidate, while a NaN second candidate is selected and written.
For example, positive-infinite excess divided by positive-infinite ratio
produces a NaN candidate, and the final high becomes NaN. No diagnostic
records any of these paths.

#### CP196 producer and mixed-record lifecycle

CP196 reads temperature-and-humidity objects after the operative-temperature
modifier input. It sets `AnyZoneTempAndHumidityControl = true` solely from
positive object count, before parsing records. In the normal branch where A1
matches a thermostat object, it expands every Zone/ZoneList child, binds the
dehumidifying schedule, maps A3 None or Overcool plus A4 Constant/Scheduled,
stores the constant range, and assigns the ratio to each expanded record.
Most repeated ZoneList diagnostics are emitted only for item one, but values
and pointers are assigned per child.

Two pinned-source producer anomalies constrain usable input:

- the normal Scheduled branch reads `cAlphaArgs(6)` even though the schema has
  only alpha fields A1 through A5 and names A5 as the range schedule, so the
  documented schedule is not bound through that branch;
- the documented `<Zone Name> <Thermostat Name>` fallback maps A3 against the
  None/Constant/Scheduled enum before examining A4, so schema-valid Overcool
  becomes Invalid and severe; that Severe incorrectly reports A4's field and
  value instead of A3. A malformed Scheduled value can reach its range-schedule
  branch while leaving ratio at the default zero.

The ordinary Constant thermostat-object path is therefore the normal
schema-conforming active path in the pinned source. None returns at CP231 but
still contributes to the global object count. Input errors are sticky and
normally end CP196 before simulation; caught failures or direct-state tests
can expose partial records and the already-true global flag.

Every `TempControlledZone` not assigned by a modifier retains
`OvercoolCtrl = Invalid`, constant range and ratio zero, and null schedule
pointers. When any other object makes the global flag true, an untargeted
SingleCool or Dual record passes the exact-None guard, takes constant zero,
then dereferences its null dehumidifying schedule before the ratio gate. This
mixed-record dependency is part of the source lifecycle. There is no blanket
initialization to None and no per-record bound flag.

The dehumidifying schedule has no CP196-local percent-range validation.
Constant range uses inclusive `[0,3]`; any Scheduled branch that obtains a
schedule pointer calls the same all-values check; ratio accepts any value
greater than or equal to zero. Runtime does not repeat those checks.

#### Parent order, consumers, and cadence

There are exactly two production call expressions, both in CP204
`CalcZoneAirTempSetPoints`:

- SingleCool at line 3338 samples the raw cooling schedule, saves that raw
  value in the control record, applies CP228 adaptive comfort when enabled,
  applies CP227 operative conversion to scalar `setpt`, copies scalar to
  `setptHi`, and then calls CP231;
- DualHeatCool at line 3410 samples and saves raw cooling high, applies
  adaptive and operative work to high, samples and saves raw heating low,
  applies operative conversion to low, optionally lets optimum start replace
  both live bounds, and then calls CP231 once.

SingleHeat, SingleHeatCool, Uncontrolled, and the default branch never call
CP231. Runtime cardinality is one call per temperature-control record whose
sampled ordinary type is SingleCool or Dual. Dual does not call it once per
bound.

Immediately afterward, the parent can subtract a thermostat-fault offset from
scalar, low, and high. CP229 comfort control then can overwrite ordinary
control types and setpoints, and CP232
`OverrideAirSetPointsforEMSCntrl` has final precedence inside the setpoint
parent. CP231 has no way to distinguish or preserve an earlier/later
override.

The high-only write has different consumers by control type. Ordinary
SingleCool prediction uses scalar `setpt`, which CP231 leaves at the
pre-overcool value, so the reduction can appear in the already registered
system-timestep `Zone Thermostat Cooling Setpoint Temperature` backing value
without driving the SingleCool load. Dual prediction uses `setptHi` and can
consume the reduction.

A later `PredictSystemLoads` pass adds another override boundary. For a
positive thermostat cutout difference, it reconstructs SingleCool or Dual
high from `TempControlledZone.ZoneThermostatSetPointHi`, captured before
adaptive, operative, CP231, fault, comfort, and EMS work. That path can
discard CP231 before load calculation and later output sampling.

The ordinary built-in path reaches the setpoint parent once per Zone timestep
before HVAC system substeps. Demand-manager `ResimHVAC` can request another
same-time setpoint sweep, while the external-HVAC path bypasses the built-in
call. CP231 has no separate cadence and no warmup, sizing, kickoff,
environment, occupancy, availability, separate Humidistat, or
active-dehumidification guard.

#### Failure, replay, and reset

Every potentially failing CP231 operation, including both initial indexed
aliases, an optional range-schedule dereference, the Zone RH index, and the
mandatory dehumidifying-schedule dereference, occurs before the sole
subtraction. An abnormal exit before line 6456 therefore makes no CP231-owned
setpoint write,
but it retains the complete prefix of earlier parent schedule, adaptive,
operative, and optimum-start state. There is no status, catch, transaction,
rollback, cleanup, or latch.

A successful direct duplicate call is generally non-idempotent because it
subtracts from the already reduced high. If one call makes high exactly equal
to low, the next call observes a zero gap, skips the cap, and can subtract the
full range below low. Other repeats accumulate until changing gap, RH,
schedule, ratio, or nonfinite behavior alters the branch.

A full-parent replay normally resamples and rebuilds SingleCool scalar/high or
Dual high/low before calling CP231 again, but it also repeats all sibling
modifiers and can observe
changed schedules, optimum-start state, RH, faults, comfort state, or EMS.
Clean isolated replay requires reconstruction of DataZoneControls records,
global flag, and schedule pointers; HeatBalFanSys setpoints;
ZoneTempPredictorCorrector Zone RH; and ScheduleManager current/override
state. `InitZoneAirSetPoints` begin-environment work zeroes HeatBalFanSys
scalar, adaptive-cooling, low, and high setpoints, and each Zone/Space heat
balance begin-environment initializer zeroes `airRelHum`. Neither resets the
DataZoneControls global flag, mode, range, ratio, or schedule pointers; those
require owner clear. Replaying the parent also spans availability, fault,
comfort, EMS, and environment owners. CP231 owns no environment reset.

#### C++ tests, full corpus, and oracle candidate

No C++ unit test directly names CP231, and no unit fixture contains the exact
temperature-and-humidity object. Four fixtures make 21 direct
`CalcZoneAirTempSetPoints` calls and yield exactly 20 indirect CP231
call-expression entries:

- the SystemAvailability optimum-start fixture contributes three expanded
  Dual entries;
- the reporting fixture contributes seven SingleCool/Dual entries;
- two cutout-difference fixtures contribute five entries each.

All 20 retain `AnyZoneTempAndHumidityControl = false` and return at the first
guard. Their assertions cover sibling parent behavior, not CP231 range,
humidity, ratio, high-only mutation, mixed-record failure, repeat, or reset.

The unit tree contains 57 active full-simulation expressions after excluding
five commented expressions. One expected EMS fatal stops before setpoint
acquisition, while 56 reach setup. Thirty-eight configurations contain 52
ordinary thermostat records: 49 Dual-only records and three schedule-switching
SingleHeat/SingleCool records. Aggregating one reached setpoint sweep per
configuration therefore yields 49 guaranteed CP231 expression entries and at
most 52 depending on those three
control schedules. This is a per-sweep static census, not a runtime total
through warmup, timesteps, and resimulation.

None of the 57 configurations contains a
`ZoneControl:Thermostat:TemperatureAndHumidity` object. Their global flag
remains false, so active range/RH/ratio work and the high write have zero
full-corpus reach.

The installed EnergyPlus 26.1 ExampleFiles contain one exact source candidate:
`AirflowNetwork_MultiZone_House_OvercoolDehumid.idf`. It binds one
DualSetpoint Living Zone thermostat to a 45 percent dehumidifying schedule,
Constant 1.7 K maximum range, and 3 percent/K ratio. Upstream
`testfiles/CMakeLists.txt` line 159 registers it with the Miami EPW through
`add_simulation_test`, so it is full-model Constant-path regression evidence.
It requests Zone RH but not
`Zone Thermostat Cooling Setpoint Temperature`, has no direct CP231 assertion
or counter, and does not cover Scheduled mode. Neither the input nor a runner
is adopted by this repository. It remains an oracle candidate, not
repository isolation, Rust comparison, numerical evidence, or conformance.

Eighty-five other installed ExampleFiles contain
`ZoneControl:Humidistat`. That separate object neither raises CP231's global
flag nor supplies its per-thermostat modifier, so those files are not CP231
reach evidence.

#### Rust boundary

Crate-wide authored Rust contains no exact CP231 symbol, typed
`ZoneControl:Thermostat:TemperatureAndHumidity` object, overcool enum,
constant/scheduled range binding, percent/K ratio, per-thermostat
dehumidifying schedule, current Zone RH-percent field, mutable
scalar/low/high thermostat-setpoint record, high-only modifier, diagnostic
state, or live caller. The raw modifier has no partial-support rule and
run-blocks before runtime.

Rust's ordinary thermostat model retains a direct-Zone DualSetpoint graph and
cutout metadata, and the setpoint compatibility wrapper receives an empty
closure in the live heat-balance path. Its narrow IdealLoads thermostat output
samples the first raw DualSetpoint schedules; it does not compose an overcool
modifier or write a runtime high setpoint.

Typed `ZoneControl:Humidistat`, psychrometric relative-humidity primitives,
and bounded IdealLoads humidification/dehumidification demand are adjacent
moisture-control evidence only. The source humidistat path also applies its
own EMS and fault offsets after schedule sampling, whereas CP231 reads an
independent raw dehumidifying schedule. None reproduces the global/per-record
guards, range and ratio caps, SingleCool scalar/high asymmetry, producer
anomalies, parent precedence, or failure lifecycle.

CP231 therefore adds no algorithm-level `energyplus_source` entry, Rust
target, code, mapped state, test, support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance promotion.
The inventory becomes 32 algorithms and 237 routines, split 58
`state_mapped` plus 179 `source_mapped`, with 114 required; the heat-balance
project list becomes 83.

### CP232 `OverrideAirSetPointsforEMSCntrl` source map

CP232 adds canonical required
`routine.override_air_set_points_for_ems_cntrl` and the project-contract item
`override_air_set_points_for_ems_cntrl` immediately after CP231
`adjust_cooling_set_point_for_temp_and_humidity_control`. The pinned declaration
is `ZoneTempPredictorCorrector.hh` line 374, and the complete definition is
`ZoneTempPredictorCorrector.cc` lines 6460-6555.

The routine copies current EMS actuator values into the live Zone thermostat
setpoint triple. It is a source boundary only. No Rust implementation, typed
EMS input, state promotion, or conformance claim is inferred.

#### Traversal, flag gates, and assignment matrix

The body first aliases the HeatBalFanSys owner, then traverses every ordinary
`TempControlledZone` in ascending 1-based record order, followed by every
`ComfortControlledZone` in ascending order. Each record is aliased before
either flag is tested. A count larger than its allocated arena can therefore
fail even when every override flag would have been false. Zero or negative
counts skip their respective loop.

Heating and cooling are independent `if` blocks, always in that order. An
active block reads `ActualZoneNum`, aliases `zoneTstatSetpts(ZoneNum)`, and only
then dispatches on the live per-Zone control type. Thus an active flag with a
bad Zone index can fail before an unsupported control type would select the
silent default branch. A valid but wrong Zone index writes that other Zone;
CP232 performs no record-to-Zone identity check.

For ordinary records dispatch uses `TempControlType(ZoneNum)`. For comfort
records it uses `ComfortControlType(ZoneNum)`; the explicit cast around the
comfort cooling switch is redundant because the stored value already has the
same enum type. The four writes are:

| live control type | active heating flag | active cooling flag |
| --- | --- | --- |
| `SingleHeat` | scalar `setpt`, then low `setptLo` receive the heating value | no write |
| `SingleCool` | no write | scalar `setpt`, then high `setptHi` receive the cooling value |
| `SingleHeatCool` | scalar, then low receive heating | scalar, then high receive cooling |
| `DualHeatCool` | low receives heating; scalar is preserved | high receives cooling; scalar is preserved |
| every other value | no write | no write |

The chained assignments are right-associative: scalar is assigned before its
low or high companion. With both flags active on `SingleHeatCool`, heating
first leaves scalar/low at the heating value, then cooling replaces scalar and
high. The final triple is therefore scalar=cooling, low=heating,
high=cooling. On Dual, both bounds change and scalar remains whatever earlier
work left there. Opposite single-mode flags are silently ignored.

The constructor defaults both ordinary and comfort flags to false and both
values to zero. CP232 itself has no `AnyEnergyManagementSystemInModel`,
environment, warmup, sizing, kickoff, occupancy, availability, finite-value,
range, unit, deadband, or monotonicity guard. NaN, infinities, signed zero,
reversed bounds, and arbitrary Celsius-like values are copied literally. It
emits no warning or status and changes no actuator field, raw thermostat
snapshot, control type/report code, PMV/Fanger state, or adaptive state.

#### Live type dispatch and shared-Zone precedence

CP229 comfort calculation runs before CP232 and writes its selected comfort
family into ordinary `TempControlType`. Consequently the ordinary actuator
loop dispatches on that final live type, not necessarily on the ordinary
record's authored setpoint family. The comfort loop then dispatches separately
on `ComfortControlType`.

There is no duplicate-target or cross-family validation. Multiple ordinary
records targeting one Zone resolve in record order, with the later record
winning only the fields it writes. After all ordinary records, comfort records
can overwrite the same scalar or bound, so comfort is final for overlapping
fields. Nonoverlapping fields survive and can form a mixed triple from several
records or families. Both loops consult shared per-Zone type arrays rather than
record-local types.

#### EMS registration, timing, and unit anomaly

`EMSManager::SetupThermostatActuators` registers two real actuators for each
ordinary record:

- component `Zone Temperature Control`;
- unique key `TempControlledZone.ZoneName`;
- controls `Heating Setpoint` and `Cooling Setpoint`;
- units `[C]`.

It requests the same two controls for each comfort record under component
`Zone Comfort Control` and `ComfortControlledZone.ZoneName`, but reports units
`[]`. Each unique component/name/control tuple points directly to the
corresponding CP232 boolean and value field. With unique Zone names, these
families contribute
`2 * NumTempControlledZones + 2 * NumComfortControlledZones` available
actuators, excluding the separate humidity controls. `SetupEMSActuator`
uppercases its tuple key and suppresses a duplicate, retaining the first
binding even though CP232 still traverses every record.

The comfort values are nominally registered as dimensionless, yet CP232 copies
them without PMV inversion or unit conversion into the Celsius-backed live
thermostat fields. That unit mismatch is pinned source behavior, not a Rust
design recommendation.

The usual built-in order in `HVACManager` calls `ManageEMS` at
`BeginTimestepBeforePredictor`, then requests `GetZoneSetPoints`. EMS programs,
plugins, callbacks, external interfaces, or API clients can therefore update
the bound flags and values before CP232 consumes them. CP232 does not execute
EMS or verify that the current values came from a program.

#### Parent order, consumers, and later replacement

There is one production call expression: line 3459 is the unconditional final
action of CP204 `CalcZoneAirTempSetPoints`. Before it, ordinary schedule
sampling, adaptive and operative conversion, optional optimum start,
temperature-and-humidity overcool, and thermostat-fault offsets have run.
Optional CP229 comfort control runs next and can replace ordinary types and
setpoints. CP232 then gives ordinary EMS records followed by comfort EMS
records final precedence inside that parent.

The already registered system-timestep outputs
`Zone Thermostat Heating Setpoint Temperature` and
`Zone Thermostat Cooling Setpoint Temperature` reference low and high.
CP232 registers no output and scalar has no matching thermostat-setpoint output
here. Load prediction consumes scalar for SingleHeat, SingleCool, and
SingleHeatCool, but consumes low and high for Dual. Therefore cooling wins the
actual SingleHeatCool scalar load when both flags are active, while the
heating low can remain observable in the output. Dual consumes both overridden
bounds and ignores the preserved scalar.

CP232 is not necessarily the final writer before load calculation. A separate
staged-control record targeting the same Zone can replace low/high at the
start of `PredictSystemLoads`. More commonly, any positive ordinary thermostat
cutout difference rebuilds SingleHeat scalar/low, SingleCool scalar/high, or
Dual low/high from raw ordinary schedule snapshots captured before adaptive,
operative, overcool, fault, comfort, and EMS work. There is no
SingleHeatCool cutout switch case. A comfort-derived live type combined with
stale ordinary snapshots can therefore erase or mismatch CP232 values.
Dual cutout alone performs the later fatal low-greater-than-or-equal-high
check; CP232 performs none.

The ordinary built-in path reaches CP232 once per Zone-timestep setpoint sweep,
after the usual EMS calling point and before HVAC system substeps.
Demand-manager `ResimHVAC` can repeat `GetZoneSetPoints` at the same simulation
time without first rerunning that EMS calling point, reapplying retained
flags/values. The external-HVAC path bypasses the built-in sweep.

#### Failure, replay, and reset

Writes occur as each flag is processed. If a later flag, record, Zone lookup,
or type-array lookup fails, every earlier scalar/bound assignment remains.
There is no catch, transaction, rollback, cleanup, completion status, or
latch. A successful duplicate call with unchanged records, types, flags, and
values is overwrite-idempotent because every write is absolute. Changed state
or a retry after partial failure can produce a different mixed result, while
the successful prefix is simply replayed.

DataZoneControls constructors initialize flags false and values zero.
`BeginEnvrnInitializeRuntimeLanguage` clears and zeroes actuator state for
used EMS actuators at `BeginNewEnvironment`; it does not establish a reset
contract for manually populated or otherwise unregistered record fields.
`InitZoneAirSetPoints` separately zeroes the live setpoint triple and resets
ordinary control type at begin environment, while comfort calculation rebuilds
its live type during each parent call. CP232 owns no reset. Clean isolated
replay therefore spans DataZoneControls records, EMS RuntimeLanguage actuator
bindings and values, HeatBalFanSys type/setpoint arrays, comfort state, the
parent modifiers, and any downstream staged/cutout state.

#### C++ tests, active corpus, and oracle inventory

One direct C++ test, `ZoneTempPredictorCorrector_EMSOverrideSetpointTest`, calls
CP232 twice. The first call gives one ordinary Dual record both flags with
23/26 and asserts low/high. The second disables the ordinary count, gives one
comfort Dual record 22/25, and asserts low/high. Those four assertions do not
cover scalar, single modes, defaults, duplicate or cross-family collisions,
bad indexes/types, nonfinite/deadband values, parent order, downstream cutout,
failure, replay, or reset. No test directly isolates actuator registration.

Separate unit fixtures make 21 direct `CalcZoneAirTempSetPoints` calls. They
enter CP232 21 times and visit 35 ordinary records, producing 70 default-false
flag checks, zero comfort visits, and zero CP232 writes. Their assertions cover
sibling parent behavior.

The unit tree contains 57 active full-simulation expressions. One expected EMS
fatal ends before setpoint acquisition. The remaining 56 reach the setpoint
sweep; across 38 configurations they contain 52 ordinary thermostat records.
A one-sweep static census therefore gives 52 ordinary record visits and 104
false flag checks, with zero comfort visits and zero active override writes.
This is not a fixed total across warmup, timesteps, and resimulation. None of
the active inputs contains either exact CP232 actuator component key.

The installed EnergyPlus 26.1 testfiles and related scripts also contain no
actuator whose exact component is `Zone Temperature Control` or
`Zone Comfort Control`. Two `Zone Comfort Control` text occurrences in
`FurnaceWithDXSystemComfortControl.idf` are schedule names/comments, not
`EnergyManagementSystem:Actuator` objects. There is therefore no stock exact
actuator-active ExampleFile candidate for CP232; output sensors and actuators
for other components are not evidence.

#### Rust boundary

Authored Rust contains no exact CP232 routine or snake-case target, no four
ordinary/comfort EMS override fields, no actuator registry or Erl execution
engine, no comfort thermostat/type, and no mutable scalar/low/high setpoint
record. The typed thermostat graph retains only direct-Zone DualSetpoint
schedule identities, control schedule, and cutout metadata. The live
setpoint-compatibility wrapper still receives an empty closure, while the
IdealLoads diagnostic path repeats the first DualSetpoint's raw schedule
values rather than consuming a mutable source-order setpoint triple.

Every `EnergyManagementSystem:*` object is RawOnly and run-blocked with no
partial-support exception. The existing negative arbitrary-run test uses an
EMS Program only to prove blocking before runtime; it does not exercise
actuation. Four EMS execution-plan stages are labels and identity metadata,
not an actuator registry, runtime engine, state arena, callback application,
or CP232 caller. Repository conformance inputs contain only unrelated raw EMS
construction-index evidence and no actuator object.

CP232 therefore adds no algorithm-level `energyplus_source` entry, Rust
target, code, mapped state, test, support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance promotion.
The inventory becomes 32 algorithms and 238 routines, split 58
`state_mapped` plus 180 `source_mapped`, with 115 required; the heat-balance
project list becomes 84.

### CP233 `FillPredefinedTableOnThermostatSetpoints` source map

CP233 adds canonical required
`routine.fill_predefined_table_on_thermostat_setpoints` and the
project-contract item `fill_predefined_table_on_thermostat_setpoints`
immediately after CP232 `override_air_set_points_for_ems_cntrl`. The pinned
declaration is `ZoneTempPredictorCorrector.hh` line 376, and the complete
definition is `ZoneTempPredictorCorrector.cc` lines 6558-6672.

The routine builds entries for the LEED
`Schedules-SetPoints (Schedule Type=Temperature)` predefined subtable. It is a
source boundary only. No Rust implementation, typed report promotion, or
conformance claim is inferred.

#### Input arenas, traversal, and first-occurrence ownership

CP196 `GetZoneAirSetPoints` populates four `ZoneSetptScheds` arenas from every
authored ordinary setpoint definition, whether or not a
`ZoneControl:Thermostat` references it. Each record stores the definition name
plus heating/cooling schedule pointers, and SingleHeatingOrCooling binds both
pointers to the same schedule. CP233 never reads the independent comfort
setpoint arenas.

Traversal is fixed by family and then input order:

1. every SingleHeating definition;
2. every SingleCooling definition;
3. every SingleHeatingOrCooling definition; and
4. every DualSetpoint definition, heating side before cooling side.

A local `uniqSch` vector spans all four loops. Each candidate dereferences its
schedule pointer and linearly searches by numeric `Schedule::Num`; a first
occurrence inserts the number before any report query or cell write, and every
later occurrence is skipped completely. Deduplication is therefore by source
schedule identity, not name, pointer, role, season, value, or actual Zone use.
A schedule first encountered as heating never receives a later cooling/summer
representation. A Dual record with the same schedule on both sides keeps its
winter heating representation because heating inserts first.

`NumTempControls` contributes only to the initial vector-capacity sum; the
allocated arrays control traversal. Corrupted counts can under-reserve, or a
negative/huge converted capacity can fail allocation, without limiting which
records would otherwise be visited. Null pointers fail on the pre-dedup
`Num` dereference. CP233 performs no sorting, usage filtering, schedule-type
check, pointer validation, or comfort-family merge.

#### Row and cell matrix

The six predefined columns are `First Object Used`, `Month Assumed`,
`11am First Wednesday [C]`, `Days with Same 11am Value`,
`11pm First Wednesday [C]`, and `Days with Same 11pm Value`.

| first surviving role | season query | exact row layout | appended cells |
| --- | --- | --- | ---: |
| SingleHeating | winter | all six columns under the base schedule name | 6 |
| SingleCooling | summer | all six columns under the base schedule name | 6 |
| SingleHeatingOrCooling | summer, then winter | first object plus combined months under the base name; four value/count cells under each `<name> (summer)` and `<name> (winter)` | 10 |
| Dual heating, then cooling | winter for heat, summer for cool | six base-name columns for each side not already deduplicated | 6 per surviving side |

`First Object Used` is the first setpoint definition encountered, not the
first Zone or thermostat reference. For the combined family the base row is
sparse, and both synthetic numeric rows lack first-object and month cells.
The month cell concatenates summer first and winter second with ` and `.

Deduplication uses numeric IDs, while predefined rows use exact,
case-sensitive schedule-name strings. Normal input uppercases schedule names,
whereas CP233 appends literal lowercase suffixes, so valid parsed names do not
collide. Manually constructed or corrupted mixed-case state can still merge a
real `X (summer)` or `X (winter)` row with the synthetic row for `X`; append
order can then combine first-object/month cells from one schedule with sampled
cells from another.

#### Seasonal Wednesday query

For a detailed schedule, `Schedule::getValAndCountOnDay` chooses July for
summer and January for winter when `Latitude > 0`; southern and exactly zero
latitude reverse those months. It derives the first Wednesday from
`RunPeriodStartDayOfWeek` and leap-year state, without holiday adjustment.
The selected date's one `DSTIndex` shifts hour 11 or 23, and the query reads
only timestep 1 of that shifted hour rather than an hourly average.

The reference comes from the selected Julian date's week schedule and its
Wednesday day profile. The matching count then walks every one of the 365 or
366 Julian dates, selects each date's week schedule and that week's Wednesday
profile, and inspects the same fixed shifted-hour/timestep index. Identical
week-schedule or day-schedule pointers count immediately; other profiles use
exact floating equality. The result is a count of annual calendar-day rules
whose Wednesday profile matches, not a count of actual Wednesdays or hourly
occurrences. The selected first Wednesday's DST shift is reused for every
comparison date.

A constant schedule ignores weekday and hour, returns all 365/366 days, and
reports its end-of-run `currentVal`. `UpdateScheduleVals` can place an active
EMS value in that field, so constant reporting can reflect final EMS
actuation. Detailed reporting reads definition `tsVals` directly and ignores
the schedule's EMS override state. CP233 adds no finite, bounds, type-limit,
calendar-shape, DST-index, or table-format validation.

#### Caller, visibility, and table mutation

There is one production call expression:
`OutputReportTabular.cc` line 6998 inside
`FillRemainingPredefinedEntries`, immediately followed by CP234 at line 6999.
`SimulationManager` reaches top-level `WriteTabularReports` after the
environment loop, tariff work, final EMS checks, and final meter reporting.
`WriteTabularReports` calls `FillRemainingPredefinedEntries` before testing
`WriteTabularFiles`; CP233 therefore appends predefined state even when no
tabular/JSON/SQLite table will be emitted. Actual rendering still depends on
the LEED report show flag.

Each real, integer, or string `PreDefTableEntry` call increments the global
entry count and appends a cell. It does not upsert or reject a duplicate
row/column pair. The renderer scans append order and later duplicate cells
overwrite earlier table-body assignments. In contrast,
`RetrievePreDefTableEntry` scans from the beginning and returns the earliest
match.

CP233 has no local once guard. Repeated unchanged calls are not state
idempotent: the local dedup vector starts empty and a full duplicate set is
appended. Changed retry state can make last-wins rendering disagree with
first-wins retrieval. Multiyear `ResetTabularReports` does not clear
predefined entries; only full owning-data reconstruction establishes a clean
table store.

#### Failure, retry, and reset

All writes are immediate. A capacity allocation failure, null or malformed
schedule graph, bad calendar/DST/timestep index, report-entry allocation
failure, or later helper failure leaves every earlier cell committed. There
is no catch, status, transaction, rollback, cleanup, diagnostic, or completion
latch. A retry recreates only `uniqSch`, replays its prefix, and appends new
cells beside the retained partial attempt.

Normal input processing diagnoses missing schedule references and can fatal
before final reporting, but CP233 does not call that loader or repeat its
checks. `ZoneTempPredictorCorrectorData::clear_state` reconstructs counts and
setpoint arenas, while predefined-report data owns its separate entry store.
CP233 owns neither reset. Clean isolated replay therefore spans both owners
plus ScheduleManager calendar, DST, current/EMS, week/day, and timestep state.

#### C++ tests, full-simulation census, and stock candidates

No C++ test calls CP233 or references its six predefined column handles or
display strings. The closest `temperatureAndCountInSch_test` calls the
schedule helper nine times and makes 21 assertions for hemisphere/month
selection plus constant, seasonal, count, and hour-dependent values. It does
not compose family traversal, global deduplication, synthetic rows,
`PreDefTableEntry`, caller timing, visibility, failure, or retry.

The established unit-tree census has 57 active full-simulation expressions.
One expected EMS fatal stops before final reporting. Across the other 56
successful CP233 calls, 18 configurations have empty arenas and 38 provide 47
definitions: six SingleHeating, six SingleCooling, zero
SingleHeatingOrCooling, and 35 DualSetpoint. Per-configuration deduplication
leaves 76 ordinary schedule rows, producing a static one-finalization total
of 152 helper calls and 456 appended cells. Three VRF configurations reach
the Dual duplicate-skip branch after earlier SingleHeating/SingleCooling
definitions reuse the same schedules. None asserts a CP233 row or cell.

Among installed 26.1 files, `5ZoneAirCooled.idf` requests
AllSummaryAndSizingPeriod and offers a clear cross-family dedup candidate,
while `TermRhSingleHeatCoolNoDB.idf` requests AllSummary and exercises the
combined-family split-row shape. They are not adopted repository cases and
supply no focused comparator. Repository conformance inputs contain 30
thermostat IDFs, all DualSetpoint; none combines a relevant summary request
with a CP233 assertion. The only repository summary-report input is an
unrelated plant diagnostic.

#### Rust boundary

Rust has adjacent normalized DualSetpoint graph records with heating/cooling
`ScheduleId`s, calendar-aware schedule-series evaluation, and a separate
constant-schedule IdealLoads diagnostic stream. It has no typed SingleHeating,
SingleCooling, or SingleHeatingOrCooling arena, complete source record order,
numeric source schedule-ID deduplication, seasonal Wednesday value/count/month
query, predefined LEED table store, string cells, column identities, exact
helper, end-report caller, or composed test.

Generic `RuntimeOutputRegistry` and `ResultStore` own numeric time series, not
predefined append-order cells. `Output:Table:SummaryReports` remains a RawOnly
ignored reporting object, and the DualSetpoint coverage declaration remains
graph wiring rather than HVAC or tabular conformance. Calendar schedule tests
and IdealLoads raw-schedule outputs are adjacent evidence only.

CP233 therefore adds no algorithm-level `energyplus_source` entry, Rust
target, code, mapped state, test, support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance promotion.
The inventory becomes 32 algorithms and 239 routines, split 58
`state_mapped` plus 181 `source_mapped`, with 116 required; the heat-balance
project list becomes 85.

### CP234 `FillPredefinedTableOnThermostatSchedules` source map

CP234 adds canonical required
`routine.fill_predefined_table_on_thermostat_schedules` and the
project-contract item `fill_predefined_table_on_thermostat_schedules`
immediately after CP233 `fill_predefined_table_on_thermostat_setpoints`. The
pinned declaration is `ZoneTempPredictorCorrector.hh` line 378, and the
complete definition is `ZoneTempPredictorCorrector.cc` lines 6674-6766.

The routine populates the System Summary `Thermostat Schedules` predefined
subtable with thermostat, control-object, and schedule names. It reports
references only: it does not sample schedule values, determine the active
control type, read setpoint temperatures, inspect EMS state or calendar data,
or merge comfort, staged, or humidity controls. This is a source boundary
only, with no Rust table or conformance promotion.

#### Materialized arena and traversal

CP234 visits `TempControlledZone(1..NumTempControlledZones)` in ascending
stored order. CP196 input processing has already expanded ZoneList controls
into individual Zone records, collapsed authored field sets into the fixed
per-type `setpts` slots, and resolved schedule pointers. A repeated authored
field set of the same control type overwrites its earlier slot. Normal input
also rejects assigning one Zone more than once, but CP234 itself neither
rechecks uniqueness nor validates the count against the allocated array.

Every predefined cell uses exact `tcz.ZoneName` as its row key. For each
record CP234 first appends `tcz.Name` to `Thermostat Name 1`, then dereferences
`tcz.setptTypeSched` and appends its name to `Control Type Schedule`. It next
visits the four fixed `HVAC::controlledSetptTypes`: SingleHeating,
SingleCooling, SingleHeatingOrCooling, and DualSetpoint. `Uncontrolled` is
never represented, even when the control-type schedule selects zero.

A slot participates solely when `setpt.Name` is nonempty; `setpt.isUsed` is
not consulted. A participating DualSetpoint or
SingleHeatingOrCooling slot dereferences cooling before heating, while
SingleCooling dereferences only cooling and SingleHeating only heating. CP234
does not validate pointer presence, schedule type, Zone membership, name
uniqueness, or whether any referenced control is currently selected.

#### Move construction, sorting, and joins

The local `infos` vector is first resized to `HVAC::SetptType::Num`, producing
five blank records. Each participating indexed record is populated and then
move-appended to the same vector, leaving the indexed source record
moved-from. The vector therefore contains five original or moved-from slots
plus one appended record per participating type before sorting. Observed
standard-library behavior leaves the moved-from strings empty, so the join
filter suppresses them; the C++ standard guarantees only a valid unspecified
moved-from string state, making absence of duplicate remnants a
portability-sensitive source behavior.

All records are lexicographically sorted by the tuple
`(thermostatType, controlTypeName, heatSchName, coolSchName)`. In normal
multi-type state the primary display order is therefore
`DualSetPointWithDeadBand`, `SingleCooling`, `SingleHeatCool`, then
`SingleHeating`, independent of authored field-set order and the fixed
enumeration traversal.

Each column is joined separately with `, ` after copying and dropping exact
empty strings. There is no deduplication, escaping, quoting, or positional
placeholder. Consequently type and control-name lists include every
participating slot, the heating list omits SingleCooling, and the cooling list
omits SingleHeating. The heat/cool lists preserve their own filtered sorted
order but cannot be positionally zipped with the type list.

#### Row and cell matrix

The six columns, created under System Summary at
`OutputReportPredefined.cc` lines 1098-1104, are `Thermostat Name 1`,
`Control Type Schedule`, `Control Type`, `Control Type Name`,
`Heating Schedule`, and `Cooling Schedule`.

Four cells are appended for every visited Zone record: thermostat name,
control-type schedule, the joined type list, and the joined control-object
name list. The last two are appended even when their joined strings are
empty. Heating and cooling cells are conditional on a nonempty joined value,
so a record contributes four through six cells. Reused names remain repeated.
Repeated or corrupted `ZoneName` keys create duplicate row/column cells rather
than a merged or rejected record.

#### Caller, visibility, and table mutation

The sole production call expression is `OutputReportTabular.cc` line 6999
inside `FillRemainingPredefinedEntries`, immediately after CP233 at line
6998. Top-level `WriteTabularReports` reaches this path after the environment
loop and calls it before the later `WriteTabularFiles` guard. CP234 therefore
appends predefined state even when no tabular file is emitted; rendering the
subtable still requires System Summary or an encompassing summary request.

`PreDefTableEntry` is append-only. It increments the global entry count and
never upserts or rejects a duplicate row/column pair. Render assembly scans
append order and later duplicates overwrite earlier table-body assignments,
whereas `RetrievePreDefTableEntry` scans from the beginning and returns the
earliest match. CP234 has no local once guard, status, diagnostic, transaction,
rollback, or cleanup.

Repeated unchanged calls append a complete duplicate set. Changed state or a
partial retry can make last-wins rendered output disagree with first-wins
retrieval. Multiyear `ResetTabularReports` does not clear predefined entries;
full `OutputReportPredefinedData::clear_state` reconstructs the table store,
while `DataZoneControlsData::clear_state` separately deallocates thermostat
records. CP234 owns neither reset.

#### Failure and retry boundary

A null `setptTypeSched` fails after the thermostat-name cell has committed. A
null participating heat or cool pointer fails after both leading cells; Dual
and SingleHeatingOrCooling fail on cooling before heating. Count/allocation
mismatch, vector allocation, sort/join/formatting, or predefined-entry growth
can fail after prior Zones or the current prefix have committed. There is no
bounds defense or compensating action. A retry starts a new local vector and
duplicates the retained prefix before continuing.

#### C++ tests, corpus reach, and stock candidates

One active direct C++ test calls CP234 once and makes 24 assertions across
four Zones. It covers one SingleHeating, SingleCooling,
SingleHeatingOrCooling, and DualSetpoint slot apiece, all six columns, and
absence of the inapplicable heat/cool cell. It intentionally succeeds with
the participating `isUsed` flags left false. It does not cover multiple types
in one Zone, tuple order, filtered-list alignment, moved-from slots, blank
names, duplicate row keys, nulls, failure, retry, reset, or serialization.

A second intended multi-control test is disabled behind `#ifdef GET_OUT` and
uses obsolete fields. Its four-record loop contains six assertion statements
and is not compiled evidence.

Of 57 active full-simulation expressions, one expected EMS fatal stops before
final reporting. The other 56 CP234 calls comprise 18 empty and 38 nonempty
configurations. The nonempty calls contain 52 expanded Zone records, 49 with
DualSetpoint only and three with both SingleHeating and SingleCooling. Collectively, those finalizations visit 208 slots, retain 55 populated
slots, and append 312 cells. The three switching records exercise a two-item lexical join, but no
full-simulation assertion reads this subtable.

Among installed 26.1 ExampleFiles, 639 contain `ZoneControl:Thermostat` and
606 of those request System Summary or an encompassing summary.
`5ZoneAirCooled.idf` is a strong multi-type order/join candidate,
`TermRhSingleHeatCoolNoDB.idf` covers SingleHeatingOrCooling, and
`FurnaceWithDXSystemComfortControl.idf` demonstrates that CP234 reports the
ordinary thermostat rather than comfort control. None is adopted as a
repository row oracle. The 30 repository conformance thermostat IDFs all use
DualSetpoint, none requests a summary table, and no comparator asserts any of
the six cells.

#### Rust boundary

Rust retains a normalized direct-Zone `ZoneThermostat` graph with
DualSetpoint control edges and heating/cooling `ScheduleId`s, execution-plan
metadata, and an IdealLoads first-control constant-schedule resolver. It has
no typed SingleHeating, SingleCooling, or SingleHeatingOrCooling records,
ZoneList expansion into source `TempControlledZone` order, fixed four-slot
layout, exact tuple sort and independent filtered joins, predefined System
Summary table store, six column identities, append/retry/reset lifecycle,
final-report caller, serializer, or comparator.

`Output:Table:SummaryReports` remains ignored through the `Output:*`
capability boundary. Adjacent DualSetpoint graph wiring and schedule
evaluation do not implement CP234. No algorithm-level source, Rust target,
code, mapped state, test, support, capability, output implementation,
comparator, case, manifest, numerical, performance, or conformance promotion
is added.

The inventory becomes 32 algorithms and 240 routines, split 58
`state_mapped` plus 182 `source_mapped`, with 117 required; the heat-balance
project list becomes 86.

### CP235 `ZoneSpaceHeatBalanceData::updateTemperatures` source map

CP235 adds canonical required
`routine.zone_space_heat_balance_update_temperatures` and the
project-contract item `zone_space_heat_balance_update_temperatures`
immediately after CP234
`fill_predefined_table_on_thermostat_schedules`. The member declaration is
`ZoneTempPredictorCorrector.hh` lines 233-234, and the complete definition is
`ZoneTempPredictorCorrector.cc` lines 6768-6833.

The sole production call expression is the first executable child of CP203
`ZoneSpaceHeatBalanceData::predictSystemLoad` at line 3155. CP203 already owns
the parent traversal and later load transaction, while CP216 owns the
`DownInterpolate4HistoryValues` arithmetic and generic alias behavior. CP235
owns the node rollback, helper-call conditions and topology, returned-current
assignments, and final working-history selection without promoting either
dependency.

#### Entry and unconditional working-history selection

The only local identity check is debug `assert(zoneNum > 0)`. On every normal
return, regardless of shortening, the routine performs two ordered whole-array
assignments:

| `UseZoneTimeStepHistory` | first `ZTM` source | second `WPrevZoneTSTemp` source |
|---|---|---|
| true | four-slot `XMAT` | four-slot `WPrevZoneTS` |
| false | four-slot `DSXMAT` | four-slot `DSWPrevZoneTS` |

This selector does not itself change `MAT` or `airHumRat`. A false shortening
flag with false history selection therefore copies preexisting downstepped
arrays while leaving both current record values untouched. Conversely, a
shortened count-change call can populate downstepped state and current values,
then select the Zone-timestep arrays when the independent history flag is
true. The fourth slot is copied even though later ThirdOrder equations use
only the first three.

#### Shortened Zone and Space node rollback

Only `ShortenTimeStepSys` enters rollback. Exact `spaceNum == 0` reads the
Zone's `SystemZoneNodeNumber`; every nonzero value, including malformed
negative identities in a release build, indexes the Space arena. A node number
must be strictly positive, but CP235 applies no upper-bound check.

A surviving node is overwritten in this order:

1. node `Temp = XMAT[0]`;
2. shared parent-Zone `TempTstatAir(zoneNum) = XMAT[0]`;
3. node `HumRat = WPrevZoneTS[0]`;
4. node `Enthalpy = PsyHFnTdbW(XMAT[0], WPrevZoneTS[0])`.

The inline enthalpy helper writes no state and floors only its humidity
operand through `max(dW, 1.0e-5)`. A negative history therefore remains
negative in node `HumRat` while the associated enthalpy uses `1.0e-5`.
Nonfinite values otherwise follow native floating behavior.

A Space call still writes the parent Zone's shared `TempTstatAir`. Under
CP202's Zone-first then stored-Space traversal, the last reached positive-node
Space can therefore replace the earlier Zone or Space value. Repeated node
identities likewise resolve by last reached writer. Rollback occurs on every
shortened call even when the current and previous system-step counts match and
independently of the final history selector.

#### Count-change interpolation transaction

After node rollback, CP235 compares
`NumOfSysTimeSteps` with `NumOfSysTimeStepsLastZoneTimeStep`. Equality performs
no interpolation and leaves `DSXMAT`, `DSWPrevZoneTS`, `MAT`, `airHumRat`, and
all RoomAir current/downstepped fields unchanged. The routine still reaches
the final selector.

On inequality it reads `TimeStepSys` and calls the CP216 array helper in this
fixed order:

1. record `XMAT -> DSXMAT`, then assign the returned `XMAT[0]` to `MAT`;
2. record `WPrevZoneTS -> DSWPrevZoneTS`, then assign the returned
   `WPrevZoneTS[0]` to `airHumRat`;
3. for an exact Zone under global `anyNonMixingRoomAirModel`, conditionally
   interpolate Floor, occupied, then mixed temperature histories when the
   Zone is three-node displacement ventilation or UFAD;
4. under the same exact-Zone/global gate but an independent branch, an
   `AirflowNetwork` RoomAir enum visits stored AFN nodes and interpolates each
   node's temperature then humidity histories.

The RoomAir AFN branch does not test `AFNZoneInfo.IsUsed`. A false global
non-Mixing flag suppresses it even if malformed state sets the Zone enum
directly. If malformed state makes a stratified predicate and the AFN enum
simultaneously true, both ordered branches run. Space records receive only
the two base calls and never touch shared RoomAir history.

Every normal production helper call passes distinct fixed arrays, so CP216's
generic same-array alias behavior is not a normal CP235 path. CP235 adds no
new formula, tolerance, or timestep validation: zero, negative, noninteger,
infinite, or NaN ratios and helper write-prefix behavior remain CP216's
source-only dependency boundary.

#### Caller traversal and HVAC cadence

CP202 visits Zones in ascending identity order. After each Zone it invokes
CP203 and CP235 for stored Spaces only while `doSpaceHeatBalance` is true. With
Space heat balance off, no Space CP235 call occurs; only the parent shortened
fallback mirrors the already updated Zone `MAT` and `airHumRat` into its
Spaces.

At each Zone timestep `HVACManager` initializes `ShortenTimeStepSys = false`,
`UseZoneTimeStepHistory = true`, and `NumOfSysTimeSteps = 1`. The initial
prediction therefore skips rollback/interpolation and selects Zone-timestep
histories. A sufficiently large correction can set shortening true and
history selection false. The first fine-step prediction then rolls nodes back,
interpolates only if the new count differs from the prior Zone timestep, and
selects downstepped histories. `HVACManager` clears shortening after that
fine-step correction while leaving history selection false, so later fine-step
predictions select the existing downstepped arrays without another rollback.

The Zone-timestep tail stores the final count for the next timestep. A repeated
count can therefore make the next first shortened prediction roll nodes back
but reuse prior downstepped state. `SimulationManager::Resimulate` passes
literal false shortening and only performs the current history selection.

#### Validation, failure, replay, and reset

Beyond the debug assertion, CP235 validates no Zone or Space upper bound,
record-to-identity correspondence, Space membership, node upper bound,
shared-node topology, RoomAir/AFN shape or use flag, step-count positivity,
count/time consistency, timestep sign or finiteness, or history values. With
shortening false, a release build can complete the member-array copies without
using a malformed `zoneNum`. With shortening true, every invalid indexed owner
can fail before later work.

There is no status, diagnostic, allocation, latch, catch, cleanup,
transaction, rollback, or locally owned reset. A node enthalpy abnormal
non-return can preserve the preceding temperature, shared thermostat, and
humidity writes. Each CP216 helper writes its destination before returning the
scalar that CP235 assigns to the matching current value, so a floating trap or
other abnormal exit can retain a destination prefix without `MAT`,
`airHumRat`, or the RoomAir current scalar.

Failure preserves the reached sequence: node rollback; base temperature; base
humidity; Floor, occupied, and mixed histories; then each AFN node temperature
and humidity. The final `ZTM` and `WPrevZoneTSTemp` assignments occur only
after all shortened work returns. A CP235 non-return blocks every later CP203
capacity, sum, coefficient, and demand effect but retains CP235 and earlier
record prefixes.

For valid stable topology, histories, timesteps, and counts, all production
arrays are distinct and a complete repeated CP235 call overwrites the same
values deterministically. It is not a transaction-wide retry guarantee:
changed counts can reuse partial downstepped state, shared-node traversal
changes the last writer, and CP203's later children own independent
non-idempotent state.

CP200 `beginEnvironmentInit` resets `ZTM`, `WPrevZoneTS`, `DSWPrevZoneTS`,
and `WPrevZoneTSTemp` but not `XMAT`, `DSXMAT`, `MAT`, or `airHumRat`.
History push/revert routines, HVAC counters, loop nodes, HeatBalFanSys, and
RoomAir/AFN state have separate owners. Clean replay therefore requires
coordinated reconstruction or reset across all of them.

#### C++ tests, full-simulation census, and stock candidates

No C++ test calls `updateTemperatures` or `predictSystemLoad` directly. Two
focused `PredictSystemLoads` fixtures make 16 wrapper calls and 24 setpoint
assertions, including four shortened wrapper calls, but both retain
`NumOfZones = 0`; focused CP235 reach and CP235-owned assertions are zero.
The CP216 array-helper test makes one ratio-two call with nine assertions but
does not compose this wrapper.

Of 57 active full-simulation expressions, one expected EMS fatal stops before
prediction and one successful case has zero Zones. The other 55 configurations
transitively reach CP235. Across one initial PredictStep per configuration,
their aggregate topology is 81 Zone plus 24 active Space records, or 105 CP235
calls. That gives 105 complete `ZTM` selections and 105 complete humidity
working-history selections. No assertion isolates either assignment.

Actual adaptive shortening is not instrumented and is conservatively bounded
from zero through all 55 configurations. Across one hypothetical first
shortened pass, at most 76 records have positive system nodes: 55 controlled
Zones plus 21 automatically created sizing-Space nodes. Those provide at most
76 node rollback transactions and 76 shared `TempTstatAir` writes. A
count-change pass has at most 210 base interpolation calls across the 105
records.

All 81 corpus Zones use the Mixing RoomAir model, so displacement/UFAD and
RoomAir-AFN additions have zero corpus potential. No focused or full-simulation
assertion covers a false/true shortening pair, either history selector,
positive/absent Zone or Space node, equal/different step counts, special
RoomAir topology, malformed state, failure prefix, replay, or reset.

Installed files offer future branch candidates: `RoomAirflowNetwork.idf` has
six RoomAir AFN nodes; four `DisplacementVent_*.idf` files select a three-node
Zone; `5ZoneSupRetPlenVSATU.idf` selects five UFAD Zones; and two
`5ZoneAirCooledWithSpace*` files enable Space heat balance. None is referenced
by repository comparison/smoke scripts, and an ordinary stock run does not
prove that adaptive shortening or a count change occurred.

#### Rust boundary

Rust has Zone-only three-slot Zone/system temperature and humidity histories,
a `use_zone_timestep_history` flag, an adaptive system-step count, and the
CP216-adjacent by-value interpolation helper. Its local adaptive correction
chooses a count from MAT change, reuses or rebuilds three-slot histories,
copies slot zero into current Zone state only on a count change, runs its own
fine-step correction/average loop, and commits the final local histories.

It has no fourth slot, CP235 working `ZTM`/`WPrevZoneTSTemp` selection
transaction, Space heat-balance record, Zone or Space system-node rollback,
shared `TempTstatAir`, node enthalpy update, stratified RoomAir state, AFN
nodes, global HVAC count/cadence, exact wrapper, source failure/retry shape, or
composed test. Its helper rejects nonpositive timesteps, unlike the source
dependency. State flags and diagnostic traces are adjacent metadata, not
output or execution parity.

CP235 adds no algorithm-level source, Rust target, code, mapped state, test,
support, capability, output implementation, comparator, case, manifest,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 241 routines, split 58 `state_mapped` plus 183
`source_mapped`, with 118 required; the heat-balance project list becomes 87.

### CP236 `ZoneSpaceHeatBalanceData::calcPredictedSystemLoad` source map

CP236 adds canonical required
`routine.zone_space_heat_balance_calc_predicted_system_load` and the
project-contract item `zone_space_heat_balance_calc_predicted_system_load`
immediately after CP235
`zone_space_heat_balance_update_temperatures`. The member is declared at
`ZoneTempPredictorCorrector.hh` line 224 and its complete definition is
`ZoneTempPredictorCorrector.cc` lines 6835-7243.

The sole production call expression is CP203
`ZoneSpaceHeatBalanceData::predictSystemLoad` line 3253, after CP235 history
selection and the parent's capacitance, sum, coefficient, RoomAir-AFN, and
non-ThirdOrder preparation, and before the predicted-humidity child at line
3256. CP202 supplies Zone-first then stored active-Space traversal. CP236 owns
the five-way temperature-control dispatch, sensible-load selection, staged
override, final thermostat/deadband writes, and selected demand reporting; it
does not promote its parent or reporting helper.

#### Identity and inherited predictor state

The only local identity check is debug `assert(zoneNum > 0)`. Every call reads
the parent Zone, that Zone's thermostat setpoint triple, solution algorithm,
temperature-control type, staged-control gate, and ITE state. Exact
`spaceNum > 0` selects a Space system node, `StageNum`, heat-balance record,
and sensible-demand destination; zero or a malformed negative identity selects
the Zone path. This differs from CP235, where every nonzero identity selects a
Space during shortened rollback.

A Space calculation still uses its parent Zone's control type, setpoint triple,
ITE adjustment, load-correction factor, multipliers, staged-control flag, and
diagnostic name. Only the record coefficients/history, selected node,
`StageNum`, `setPointLast`, and final demand record vary. CP203's active
RoomAir-AFN coefficient block has no Space guard, so it can replace even a
Space record's `tempDepLoad` and `tempIndLoad` from the Zone control node and
control fraction before CP236.

#### Three load equations

Let `D = tempDepLoad`, `I = tempIndLoad`, `C = AirPowerCap`,
`T1 = T1`, and `S` be the selected setpoint. Each ordinary or nonzero staged
branch repeats one of three source equations:

| Solution algorithm | Predicted load |
|---|---|
| `ThirdOrder` | `D * S - I` |
| `AnalyticalSolution`, exact `D == 0.0` | `C * (S - T1) - I` |
| `AnalyticalSolution`, otherwise | with `e = exp(min(700.0, -D / C))`, `D * (S - T1 * e) / (1 - e) - I` |
| `EulerMethod` | `C * (S - T1) + D * S - I` |

There is no local positive-capacitance, denominator, overflow, or finite-value
guard. The exponential cap inherits ObjexxFCL `min` behavior, including its
second-argument result when comparison is unordered. An invalid solution enum
only debug-asserts; a release build retains the branch-local value already in
the load variable. CP203 retains its history/capacitance-expanded `D` and `I`
for ThirdOrder, but resets them to `TempDepCoef` and `TempIndCoef` before
Analytical or Euler entry.

#### Ordinary control dispatch

All three load locals and `ZoneSetPoint` start at zero, and the local deadband
flag starts false.

| Zone control type | Source selection and persistent local result |
|---|---|
| `Uncontrolled` | leaves both setpoint loads, total load, setpoint, and deadband at zero/false |
| `SingleHeat` | calculates the scalar setpoint load, divides it by a strictly positive RAFN fraction, copies it to total and cooling-setpoint load, publishes the scalar setpoint, and marks deadband for `total <= 0` |
| `SingleCool` | calculates the scalar cooling load, applies the source RAFN defect described below, optionally replaces cooling from ITE, copies it to total and heating-setpoint load, publishes the scalar setpoint, and marks deadband for `total >= 0` |
| `SingleHeatCool` | calculates both loads at the scalar setpoint, scales both for positive RAFN, optionally replaces cooling from ITE, publishes the scalar setpoint, then chooses heating when both are positive, cooling when both are negative, or zero/deadband when they straddle zero |
| `DualHeatCool` | calculates heating at `setptLo` and cooling at `setptHi`, scales both for positive RAFN, optionally replaces cooling from ITE, then publishes low for heating, high for cooling, or zero/deadband with a node-clamped target when they straddle zero |

For either combined control, `heating > cooling` emits the source diagnostic
sequence and fatals before staged control can rescue it. If neither both-sign
nor straddling predicate matches, including common NaN shapes, the separate
unanticipated-combination diagnostics also fatal. In a deadband branch a
positive selected node clamps its current temperature to `[setptLo, setptHi]`.
Without a positive node, Dual leaves `ZoneSetPoint` at its initial zero;
SingleHeatCool retains the scalar setpoint assigned before classification.

The default/invalid control type silently preserves the initial zero locals,
unless a later staged branch overwrites them. Single-mode nonfinite results can
reach the final writes because their sign test merely remains false.

#### RAFN and ITE asymmetries

RAFN scaling occurs only when `RAFNFrac > 0.0`; zero, negative, and NaN values
skip it, while values above one, infinity, and arbitrarily small positive
fractions are accepted. `SingleCool` lines 6926-6928 divide the still-zero
heating-load local instead of the calculated cooling load, so its cooling
demand is not RAFN-scaled. The other controlled branches divide their intended
load or loads.

When the parent Zone has `HasAdjustedReturnTempByITE` and `BeginSimFlag` is
false, SingleCool, SingleHeatCool, and Dual overwrite only cooling with
`D * AdjustedReturnTempByITE - I`. This is a ThirdOrder-shaped expression
regardless of the selected solution algorithm, occurs after RAFN division, and
therefore discards cooling scaling. It applies to Space records through the
parent Zone and can create the combined-control fatal ordering. SingleHeat and
Uncontrolled have no ITE branch.

#### Staged override

CP236 unconditionally reads a selected `StageNum` and node before checking the
global staged-zone count or per-Zone `StageZoneLogic`. A Space reads its own
demand record and only debug-asserts equality with the Zone `StageNum`; a
release build can use divergent Space state. Sensible-demand environment
initialization does not reset `StageNum`, so the pre-gate read can observe
retained state even when staged logic is inactive.

When both staged gates are true, only the sign of `StageNum` matters:

- zero sets all loads to zero and deadband true; a positive node clamps its
  temperature into the deadband, while an absent node leaves the ordinary
  branch's `ZoneSetPoint` rather than forcing zero;
- negative recomputes cooling at `setptHi`, copies it to total and heating,
  selects high, and marks deadband if the resulting total is nonnegative;
- positive recomputes heating at `setptLo`, copies it to total and cooling,
  selects low, and marks deadband if the resulting total is nonpositive.

Stage magnitude is ignored. The recomputation neither reapplies RAFN nor ITE,
and it does not clear a deadband flag set by the ordinary branch. Thus a valid
stage load can retain a prior true flag, while a wrong-sign stage load is
retained and marked deadband. With an invalid solution enum in release mode,
the negative or positive stage can reuse the corresponding ordinary-branch
load rather than compute a new one.

#### Final shared state and demand reporting

Every normal return commits this exact order:

1. write the selected positive Zone or Space node `TempSetPoint`;
2. set shared parent-Zone `Setback` from
   `ZoneSetPoint > this->setPointLast`;
3. overwrite this record's `setPointLast`;
4. overwrite shared parent-Zone scalar thermostat `setpt`;
5. overwrite shared `DeadBandOrSetback`;
6. overwrite shared `CurDeadBandOrSetback`;
7. report into the selected Zone or Space sensible-demand record.

The report helper at `DataZoneEnergyDemands.cc` lines 330-351 first multiplies
raw total/heating/cooling loads by the parent Zone `LoadCorrectionFactor` into
the three predicted rates. It then multiplies by Zone `Multiplier *
ListMultiplier` into total and heating/cooling-setpoint required output. For a
controlled Zone with positive `NumZoneEquipment`, it overwrites all three
sequenced-demand arrays with those final values.

Space node and demand destinations are distinct, but the scalar thermostat,
Setback, and both deadband flags are shared by the parent Zone.
`setPointLast` is per record, so CP202's Zone-first then Space traversal can
make the last Space win the shared flags and scalar while comparing against
that Space's own prior target. The shared scalar can also be read by a later
Space call through the same setpoint record, making traversal order observable.

#### Validation, failure, replay, and reset

CP236 validates no Zone or Space upper bound, Space membership or ownership,
record/arena alignment, node upper bound, coefficient or setpoint ordering,
algorithm/control enum, RAFN or multiplier range/finiteness, staged-state
range, demand-array shape, or equipment-sequence allocation consistency. The
combined-control fatal diagnostics occur before any CP236 final persistent
writes; their diagnostic effects remain while previously stored thermostat,
deadband, demand, and record state survives. A later node or report failure
retains the ordered commit prefix and blocks the humidity child and later
records; earlier Zone/Space calls from the parent traversal remain committed.

There is no catch, status, cleanup, transaction, rollback, or once latch.
Complete replay is not generally idempotent: the first call changes
`setPointLast`, so a second stable call can change Setback from true to false;
shared thermostat/deadband state is traversal-sensitive; and diagnostics and
sequenced-demand overwrites have their own lifecycle.

`setPointLast` defaults to zero but CP200 `beginEnvironmentInit` does not reset
it. CP199 resets the thermostat setpoint arrays and load correction factor,
initializes demand records, and clears `DeadBandOrSetback`, but not `Setback`
or `CurDeadBandOrSetback`. Sensible-demand initialization clears remaining
total, sequenced, air-system, and predicted-rate fields, but retains
`StageNum`, unadjusted/heat/cool remaining fields, supply-air adjustment, and
the total heating/cooling-setpoint required outputs. Full state clearing
reconstructs the owning records.

#### Tests, active corpus, and future candidates

One C++ reporting fixture calls CP236 directly seven times at unit-source lines
498, 518, 552, 560, 568, 577, and 594. Its 19 related assertions cover one
uncontrolled call; negative and positive SingleHeat; one SingleCool cooling
case; one SingleHeatCool cooling case; and Dual heating then cooling. All calls
are Zone-only with default ThirdOrder, unit correction/multipliers, no positive
system node, Space, ITE, staged logic, failure, replay, or reset. The six
controlled calls pass literal `RAFNFrac = 1.0`, so division is numerically
invisible and the SingleCool defect is not tested.

The 16 focused CP202 wrapper calls and 24 setpoint assertions retain zero
Zones, so focused CP236 reach is zero even where a fixture selects Euler.
A separate sensible-demand helper test makes five direct report calls with 27
assertions for correction/multiplier behavior, but it does not compose CP236.

Of 57 active full-simulation expressions, one expected EMS fatal stops before
prediction and one has zero Zones. One initial prediction census across the
other 55 configurations yields 81 Zone plus 24 active Space records, or 105
CP236 calls. The records partition into 95 ThirdOrder and 10 Analytical calls
across 49 and six configurations respectively; Euler has zero active-corpus
reach, and the Analytical exact-zero versus exponential split is not
instrumented.

The 52 expanded ordinary thermostat records contain 49 fixed Dual controls and
three seasonal SingleHeat/SingleCool records, with no comfort or staged
thermostat. Across Zone/Space calls the identity mix is 32 Uncontrolled, 70
fixed Dual, and three seasonal records. Twenty-one sizing Spaces inherit Dual
control; three simulation Spaces are Uncontrolled Analytical. There are no
staged objects, adjusted-return ITE objects, or non-Mixing RoomAir models, so
staged, ITE, and nonunit production RAFN paths have zero active potential.
No full-simulation assertion isolates CP236-owned state.

Installed but unadopted candidates include
`MultiSpeedHP_StagedThermostat.idf`,
`SmOffPSZ_OnOffStagedControl.idf`, five DataCenter files with adjusted return
temperature, and `RoomAirflowNetwork.idf`. They are source-discovery
candidates only and add no repository evidence.

#### Rust boundary

Rust has no `calcPredictedSystemLoad` analog, `setPointLast`, five-way live
temperature-control dispatch, RAFN/ITE/staged override, shared
deadband/Setback transaction, or source sensible-demand reporting behavior.
Its Zone-air coefficient helpers cover bounded Zone-only ThirdOrder and
Analytical calculations but not Euler or a load-to-thermostat-setpoint
dispatcher; they also guard nonpositive capacitance/timestep inputs that the
source routine does not.

Rust owns no Space predictor/demand/control/node binding. Its typed thermostat
graph represents a bounded direct-Zone DualSetpoint subset rather than the live
mutable five-way setpoint triple. Node `TempSetPoint` storage is adjacent but
not wired here. Rust `ZoneSysEnergyDemand` carries oracle-fed remaining
heating/cooling/moisture values for IdealLoads, without predicted/total
heating/cooling-setpoint rates, staged state, equipment sequences, or the
source multiplier helper. Zone multipliers are adjacent input state, not a
composed CP236 report path.

CP236 adds no algorithm-level source, Rust target, code, mapped state, test,
support, capability, output implementation, comparator, case, manifest,
numerical, performance, or conformance promotion. The inventory becomes 32
algorithms and 242 routines, split 58 `state_mapped` plus 184
`source_mapped`, with 119 required; the heat-balance project list becomes 88.

CP237 expands the existing required `routine.manage_zone_equipment` mapping for
`ZoneEquipmentManager::ManageZoneEquipment`, declared at
`ZoneEquipmentManager.hh` lines 82-86 and implemented at
`ZoneEquipmentManager.cc` lines 141-167. It adds no routine or project item.
Every entry ignores the incoming `SimZone`, calls `InitZoneEquipment`, selects
`SizeZoneEquipment` only while `ZoneSizingCalc` is true or otherwise calls
`SimZoneEquipment` and then sets `ZoneEquipSimulatedOnce = true`, calls
`UpdateZoneEquipment`, and clears `SimZone` only after that child returns.
`FirstHVACIteration` reaches Init and the non-sizing Sim child; `SimAir` is
never cleared locally and is passed by reference through Sim when selected and
then Update.

The wrapper has no local validation, status, catch, cleanup, transaction, or
rollback. A failing child preserves its completed prefix. In particular, a
non-sizing Update failure occurs after the one-way simulated-once write but
before the caller's `SimZone` is cleared. Re-entry always repeats the children
because incoming `SimZone` and the latch are not gates. Nine direct C++ calls
across eight tests all use the non-sizing branch and assert descendant
equipment, node, or load effects rather than the parent protocol. Of 57 active
full-simulation expressions, one expected EMS fatal stops before HVAC and the
other 56 establish only a lower bound of 56 parent executions; repeated HVAC,
warmup, and sizing calls are not instrumented.

Existing Rust three-stage metadata, the typed IdealLoads graph validator,
execution-plan labels, and the direct PurchasedAir compatibility loop do not
implement the exact Init/Size-or-Sim/Update parent, its reference flags, latch,
failure prefixes, replay, reset, multi-family dispatch, or broad HVAC
behavior. CP237 therefore changes no Rust code, mapped state, support,
capability, output, numerical, performance, or conformance claim. The
inventory remains 32 algorithms and 242 routines, split 58 `state_mapped`
plus 184 `source_mapped`, with 119 required; the heat-balance and HVAC project
lists remain 88 and 8.

CP238 adds canonical required `routine.get_zone_equipment` after
`manage_zone_equipment` and before `sim_zone_equipment`, plus the matching HVAC
project item. `ZoneEquipmentManager::GetZoneEquipment` is declared at
`ZoneEquipmentManager.hh` line 88 and implemented at
`ZoneEquipmentManager.cc` lines 169-197. Its sole one-time guard encloses every
operation. A true entry calls the separate full `GetZoneEquipmentData`
dependency, clears `GetZoneEquipmentInputFlag`, sets
`ZoneEquipInputsFilled = true`, snapshots `NumOfTimeStepInDay` as the raw
integer `TimeStepsInHour * 24`, scans controlled Zone indexes for the maximum
same-index equipment-list count, and allocates but does not populate or sort
`PrioritySimOrder` to that extent. A false entry is a complete no-op.

The wrapper has no local range, allocation, count, arena, or consistency
validation and no status, diagnostic, catch, cleanup, transaction, or rollback.
A child fatal leaves the wrapper guard true and does not modify readiness
(false on a fresh-state entry), but can retain the child's partial input state
and sticky errors. Once the child returns, the guard commits false before
readiness, arithmetic, scanning, and allocation;
a later failure can therefore leave a false guard and true readiness with
unfinished derived state, and retry silently does nothing. There is no
per-environment rearm. The manager and data-owner clear paths reconstruct their
flags separately, so only coordinated full-state reset restores the normal
pair.

The only production expression is `SurfaceGeometry::SetupZoneGeometry` after
successful `GetSurfaceData` and before window-gap and storm-window input; CP237
`ManageZoneEquipment` never calls this routine. Twenty-three direct C++ calls
span 22 tests. The focused two-call test proves the default-true guard, first
snapshot `1 * 24 = 24`, populated Zone configuration, and a second-call no-op
after changing `TimeStepsInHour` to 2, but it does not assert readiness,
priority extent/content, failure, retry, or reset. Source-order tracing shows
all 57 active `ManageSimulation` expressions complete the one-time wrapper
during input setup, including the case that later fatals in EMS; 56 later
complete the simulation. No full-simulation assertion isolates CP238-owned
state.

Rust eagerly compiles immutable, IdealLoads-only typed equipment
lists/connections and separately derives time-axis sizes. It has no lazy
`GetZoneEquipment`, input/readiness latches, equipment-manager day snapshot,
full Zone/Space configuration, controlled-Zone maximum scan,
`SimulationOrder` scratch allocation, or source failure/retry/reset lifecycle.
Its graph sort and execution labels are not `PrioritySimOrder`, which CP238
only allocates and a later source routine fills. CP238 adds no algorithm-level
source, Rust target/code/state, support, capability, output, numerical,
performance, or conformance promotion. The inventory becomes 32 algorithms
and 243 routines, split 58 `state_mapped` plus 185 `source_mapped`, with 120
required; the heat-balance project list remains 88 and the HVAC list becomes 9.

CP239 adds canonical required `routine.init_zone_equipment` after
`get_zone_equipment` and before `sim_zone_equipment`, plus the matching HVAC
project item. `ZoneEquipmentManager::InitZoneEquipment` is declared at
`ZoneEquipmentManager.hh` line 90 and implemented at
`ZoneEquipmentManager.cc` lines 199-316. Its sole direct production call is the
unconditional `ManageZoneEquipment` line-155 child before that parent's
sizing-versus-simulation branch; it does not acquire equipment input.

A true one-time flag clears itself before any allocation, allocates
`ZoneEqSizing` to `NumOfZones`, and then visits ascending controlled Zone
indexes with nonzero equipment-list pointers. It publishes each selected
list's equipment count into sensible and moisture demand state, allocates six
sequenced-demand vectors, and allocates and zeroes the 35-entry sizing-method
array. Space demand vectors receive the parent Zone count only when Space heat
balance simulation or sizing is active. This allocation path uses each Zone's
stored Space membership, while the later Space initialization paths use the
full Space configuration array.

The independent begin-environment gate resets the Zone availability array and
the status/start/stop fields of allocated managers for the 14 valid component
types, then calls the separate `EquipConfiguration::beginEnvirnInit` dependency
for every controlled Zone and, only during Space simulation, controlled Space.
Those children reset selected Zone/inlet/exhaust/return node fields from fixed
20 C and current outdoor conditions. The environment flag clears only after
that whole block returns and rearms only on a reached call with
`BeginEnvrnFlag = false`.

Every invocation then calls `EquipConfiguration::hvacTimeStepInit` for
controlled Zones and optional simulation-time Spaces. It always clears each
configuration's excess exhaust; only `FirstHVACIteration` copies its Zone node
state to exhaust nodes and zeroes their flow availability. Finally CP239 zeros
exactly `SupFlow`, `ZoneRetFlow`, `SysRetFlow`, `RecircFlow`, `LeakFlow`, and
`ExcessZoneExhFlow` for every primary air loop.

There is no local topology, bounds, allocation, node, or finite-value
validation and no diagnostic, status, catch, cleanup, transaction, or rollback.
Failure after the early one-time-flag clear leaves unfinished storage that
retry skips. Environment failure before its late flag clear replays the prefix,
whereas timestep or air-loop failure after that clear retries without the
environment block during the same BeginEnvironment interval. Manager-only
reset restores the two flags but not the separately owned mutated state.

No C++ unit test directly calls CP239 or either delegated configuration method.
Nine non-sizing `ManageZoneEquipment` expressions across eight tests enter it
indirectly, but zero assertions target its latches, storage, availability,
node-reset protocol, excess exhaust, or six air-loop fields. Fifty-six active
full simulations provide only a lower bound of one CP239 entry each: 55 have
Zones and the WeatherManager fixture has zero Zones. The remaining intentional
EMS-fatal expression stops before HVAC. Exact sizing, warmup, environment, and
HVAC-iteration multiplicity is uninstrumented.

Rust has adjacent immutable IdealLoads equipment graphs, a four-scalar
`ZoneSysEnergyDemand`, diagnostic node state, and precomputed
begin-environment time-axis metadata. It has no equipment-count/sequenced
demand arenas, separate Zone moisture-demand arena or Space demand state, `ZoneEqSizing`,
availability-manager lifecycle, complete role-specific node state, persistent
one-time/environment latches, or primary-air-loop aggregate-flow state.
`IdealLoadsInitFlags` belongs to `InitPurchasedAir` and is not CP239. CP239 adds
no algorithm-level source, Rust target/code/state, test, support, capability,
output, numerical, performance, or conformance promotion. The inventory
becomes 32 algorithms and 244 routines, split 58 `state_mapped` plus 186
`source_mapped`, with 121 required; the heat-balance project list remains 88
and the HVAC list becomes 10.

CP240 adds canonical required
`routine.size_zone_space_equipment_part1` after `init_zone_equipment` and before
`sim_zone_equipment`, plus the matching HVAC project item. The exact lowercase
`ZoneEquipmentManager::sizeZoneSpaceEquipmentPart1` is declared at
`ZoneEquipmentManager.hh` lines 92-99 and implemented at
`ZoneEquipmentManager.cc` lines 317-597.

Its two production call expressions are the Zone call and optional Space-loop
call inside `SizeZoneEquipment`. The parent visits controlled Zones ascending,
calls the Zone first, and under current `doSpaceHeatBalance` visits every stored
Space without checking the Space configuration's controlled flag. The Space
call selects Space configuration, sizing, demand, heat-balance state, and node,
but deliberately retains the parent `ZoneData` and `zoneNum` for deadband, ITE,
multipliers, and final-Zone outdoor-air sizing.

Every entry zeros selected non-air and system-dependent responses, then calls
`initOutputRequired` with first-iteration true and simulation-order reset false.
That child rebuilds twelve remaining/unadjusted scalars, restores the shared
parent-Zone current deadband from its original flag on every entry, and on the
production sizing path fills allocated sequence arrays from full demand. CP240
snapshots
pre-DOAS sensible and moisture loads with separate deadband and strict
same-sign humidistat gates.

`AccountForDOAS` requires at least one inlet. It derives 90-percent-RH bounds,
uses final-Zone minimum outdoor air times standard density, delegates supply
conditions, updates remaining demand, writes DOAS state to inlet 1, and records
sensible/latent sizing fields. Two inlets route the residual load to inlet 2;
one inlet routes the residual through the non-air path. The false branch leaves
eight earlier DOAS fields stale.

The main sensible gate requires no original deadband and more than 1 W. It
selects cooling/heating supply temperature or difference, applies cooling-only
post-BeginSim ITE return adjustment, solves nonnegative mass flow above the
1e-5 C delta threshold, and applies only an adjustment factor above one.
Latent sizing independently uses strict same-sign setpoint loads and a 1e-30
absolute humidity-difference threshold, then can recompute the shared supply
state. Its
false branch leaves eight latent/no-DOAS fields stale.

A positive residual node receives only temperature, humidity ratio, enthalpy,
and mass flow. Otherwise CP240 writes non-air response and, when latent sizing
is active, additively updates latent gain; a Zone no-air result may first
distribute to Spaces, but each
following Space call zeros its own response before writing its result. The final
demand update makes two update calls on a DOAS path. CP240 has no local latch,
validation, status, catch, transaction, or rollback; failure retains ordered
Zone/Space, demand, sizing, node, non-air, and additive latent prefixes and
suppresses mass balance, leaving conditions, Part2, and the manager suffix.

No test directly calls CP240. Six `SizeZoneEquipment` calls across three tests
produce seven Zone entries, zero Space entries, and 88 mixed CP240/Part2/
downstream assertion lines. The fixtures bypass sizing setup and make
configuration controlled while `ZoneData` remains uncontrolled, so they do not
prove coherent controlled demand distribution. They omit Space, ITE, one- and
zero-inlet edges, non-air Zone output, adjustment above one, and failure/retry.

Of 56 completing active full simulations, 34 sizing configurations reach CP240
with a static first-sweep topology of 48 controlled Zones. Seven of them add 21
stored Spaces. Those Spaces are uncontrolled, zero-inlet records without DOAS,
yet CP240 still takes their non-air path. Across all 69 static roles, six Zones
enable DOAS, 13 roles enable latent sizing, 43 have a residual supply node, and
26 use non-air output; cooling ITE and an adjustment factor above one have zero
active roles. The other 22 completing simulations and the EMS-fatal context do
not reach CP240; exact repeated sizing cadence is uninstrumented.

Rust's sole raw `Sizing:Zone` epJSON fixture expects `UnsupportedSizing` before
runtime, and active IDFs contain none. It has exact adjacent psychrometrics,
fixed-option four-scalar `ZoneSysEnergyDemand`, IdealLoads supply limits, a
narrow purchased-air node update, and diagnostic node state, but no
typed/executable `Sizing:Zone`,
Zone/Space sizing and moisture arenas, total/unadjusted/sequenced demand
transaction, DOAS sizing/routing, non-air/latent distribution, or CP240
failure/replay lifecycle. CP240 adds no algorithm-level source, Rust
target/code/state, test, support, capability, output, numerical, performance,
or conformance promotion. The inventory becomes 32 algorithms and 245
routines, split 58 `state_mapped` plus 187 `source_mapped`, with 122 required;
the heat-balance project list remains 88 and the HVAC list becomes 11.

CP241 adds canonical required
`routine.size_zone_space_equipment_part2` after Part1 and before
`sim_zone_equipment`, plus the matching HVAC project item. The exact lowercase
`ZoneEquipmentManager::sizeZoneSpaceEquipmentPart2` is declared at
`ZoneEquipmentManager.hh` lines 101-105 and implemented at
`ZoneEquipmentManager.cc` lines 599-625.

Its only two production call expressions are the Zone and Space calls in
`SizeZoneEquipment`'s second pass. That pass starts only after every CP240
Zone/Space call, `CalcZoneMassBalance(state, true)`, and
`CalcZoneLeavingConditions(state, true)` return. It again visits controlled
Zones ascending, calls the Zone first, and under current `doSpaceHeatBalance`
calls every stored Space without a Space-control check.

The Zone call passes its Zone equipment configuration and `CalcZoneSizing`.
The Space call passes `CalcSpaceSizing`, parent `zoneNum`, and `spaceNum`, but
deliberately reuses the parent Zone equipment configuration rather than
`spaceEquipConfig`. Thus both calls use the parent Zone return-node list and
thermostat triplet; only the fallback system node and sizing record become
Space-specific.

CP241 selects the parent's first return node when `NumReturnNodes > 0` and that
first node identity is positive. A nonpositive count or first identity falls
back to the selected Zone/Space `SystemZoneNodeNumber`; later return nodes are
ignored. It reads only that node's temperature after the leaving-condition
dependency and never writes a node.

Strict-positive `HeatLoad` selects the heating branch before strict-positive
`CoolLoad`; all other values take the catch-all branch. Heating writes
`HeatZoneRetTemp`, chooses `HeatTstatTemp` from a strict-positive central
`setpt` or `setptLo`, and writes `CoolTstatTemp = setptHi`. Cooling writes
`CoolZoneRetTemp`, chooses `CoolTstatTemp` from the central setpoint or
`setptHi`, and writes `HeatTstatTemp = setptLo`. The catch-all writes the cool
return snapshot and both low/high thermostat bounds. Every branch overwrites
both thermostat fields and exactly one return field, leaving the opposite
return snapshot stale; heating wins if both loads are positive.
`UpdateZoneSizing` later consumes both return snapshots into sizing sequences,
so the inactive stale value is downstream-observable.

There is no child call or local latch, allocation, validation, diagnostic,
status, catch, cleanup, transaction, or rollback. Indexed return/configuration,
Zone/Space, node, or thermostat failures occur before the current record's
three writes. Parent failure before the second pass suppresses CP241 entirely;
failure during it retains the complete CP240/mass/leaving prefix and earlier
Part2 records. Same-state retry reruns that parent prefix and overwrites the
selected CP241 fields, while the inactive return field and CP240 additive
effects can remain history-dependent.

No test calls CP241 directly. Six direct `SizeZoneEquipment` calls across three
tests produce seven Zone entries and zero Space entries: one heating, one
cooling, and five catch-all, all through system-node fallback. Only four
assertions in two catch-all tests name CP241 thermostat fields; no direct
wrapper assertion names either return snapshot. The separate sizing-array reset
test proves zero reset only and never executes CP241.

Seventeen of 18 direct `ManageSizing` contexts reach 24 Zone entries, all with
one positive return node, but assert none of the four CP241 fields. Among 57
active full `ManageSimulation` contexts, 56 complete and exactly 34 reach a
static 48-Zone plus 21-Space Part2 topology. Fifty-six roles use a first return
and 13 use system-node fallback: Zones split 44/4 and Spaces 12/9. The 12 Space
roles share their parent first return; the other nine use their own Space
system node. No active role has multiple returns. Exact heat/cool/catch-all,
central-setpoint, design-day, warmup, timestep, retry, and repeated-sweep
cadence is uninstrumented.

Rust has typed thermostat schedules and direct thermostat report series,
equipment-connection return identities, diagnostic node temperatures, and a
finite-limit recirculation helper that can resolve a first return. These are
adjacent only. Rust has no Zone/Space sizing snapshot, four CP241 fields,
post-leaving second pass, parent-config Space alias, mutable
`setpt`/`setptLo`/`setptHi` triplet, load/setpoint selection, stale-field
lifecycle, or failure/replay transaction. The sole raw `Sizing:Zone` fixture
still blocks before runtime and active IDFs contain none.

CP241 adds no algorithm-level source, Rust target/code/state, test, support,
capability, output, comparator, case, manifest, numerical, performance, or
conformance promotion. The inventory becomes 32 algorithms and 246 routines,
split 58 `state_mapped` plus 188 `source_mapped`, with 123 required; the
heat-balance project list remains 88 and the HVAC list becomes 12.

CP242 adds canonical required `routine.size_zone_equipment` after Part2 and
before `sim_zone_equipment`, plus the matching HVAC project item. The exact
capitalized `ZoneEquipmentManager::SizeZoneEquipment` is declared at
`ZoneEquipmentManager.hh` line 107 and implemented completely at
`ZoneEquipmentManager.cc` lines 627-694.

Its sole production call expression is `ManageZoneEquipment` line 158, after
CP239 Init and only when the current `ZoneSizingCalc` is true. CP242 itself
accepts only `state`, does not inspect that selector, and can be called
directly. The manager's `FirstHVACIteration` argument is not forwarded.

The manager-data latch `SizeZoneEquipmentOneTimeFlag` defaults true. A true
entry delegates the still-separate `SetUpZoneSizingArrays` dependency and
clears the latch only after normal return. Setup failure therefore retains a
true latch and any child prefix; success followed by later failure leaves the
latch false, so retry skips setup. Begin-environment transitions do not rearm
it. Manager `clear_state()` reconstructs the default-true latch but does not
undo independently owned child state. External `RezeroZoneSizingArrays` is
not a CP242 per-call reset and does not change the latch.

After setup, CP242 completes an ascending controlled-Zone Part1 pass. Each
Zone call precedes its stored-order Spaces when the current
`doSpaceHeatBalance` is true; no Space-controlled check, sort, deduplication,
or cross-pass membership snapshot exists. Space Part1 uses Space
configuration, sizing, and demand state but the parent Zone record. The
parent then unconditionally calls `CalcZoneMassBalance(state, true)` and
`CalcZoneLeavingConditions(state, true)`, even with no controlled Zone.
Only after both return does a second ascending pass call Part2 for each Zone
then its gated Spaces; Space Part2 deliberately reuses the parent Zone
configuration.

Apart from clearing its setup latch, CP242 owns no output assignment,
validation, diagnostic, status, catch, cleanup, transaction, or rollback.
Child or indexed-access failure preserves completed prefixes plus any
partial effects of the failing child: setup, earlier Part1 roles, mass
balance, leaving conditions, and earlier Part2 roles as applicable. Later
roles and the outer manager update are suppressed. Same-state retry restarts
the traversal; delegated additive Part1 and mass-balance effects make the
parent generally non-idempotent.

Six direct C++ calls across three tests produce six complete wrapper
invocations, seven Zone Part1/Part2 role pairs, six mass-balance calls, six
leaving-condition calls, and zero Spaces. All three tests force the setup
latch false, and their 88 assertion lines inspect descendant or downstream
results rather than the latch, either global barrier, exact call trace, or
failure prefix. Within these direct wrappers, setup-true,
uncontrolled/zero-Zone, Space, malformed-topology, child failure, and retry
recovery are absent.

Across one parent invocation in each of the 17 reaching among 18 direct
`ManageSizing` contexts, the static aggregate is 24 Zones and zero Spaces;
the plant-only context does not enter CP242. Across one parent invocation in
each of the 34 reaching among 56 completing active `ManageSimulation`
contexts, the static aggregate is 48 Zones plus 21 Spaces. Each context
contributes only its subset. Fresh successful sizing states necessarily
cross the default-true setup route once, but no assertion isolates its call
count or latch transition. Exact design-day, warmup, timestep, HVAC-iteration,
and repeated-parent invocation counts remain uninstrumented.

Rust contains no CP242 symbol, snake-case counterpart, Zone/Space sizing
arena, mass-balance or leaving-condition parent, or one-time sizing latch.
Its three zone-equipment stage labels are only Manage, Sim, and
SimPurchasedAir; graph validation, a four-scalar demand snapshot,
psychrometrics, node projection, and direct prebound IdealLoads execution are
adjacent rather than this setup-plus-two-pass transaction. `Sizing:*` remains
run-blocked, the sole raw `Sizing:Zone` fixture fails before runtime, and the
active data-model corpus contains no `Sizing:Zone`.

CP242 adds no algorithm-level source, Rust target/code/state, test, support,
capability, output, comparator, case, manifest, numerical, performance, or
conformance promotion. The algorithm remains `scaffold` with claim level
`none`. The inventory becomes 32 algorithms and 247 routines, split 58
`state_mapped` plus 189 `source_mapped`, with 124 required; the heat-balance
project list remains 88 and the HVAC list becomes 13.

## CP243 `CalcDOASSupCondsForSizing` DOAS Supply Selector

CP243 adds canonical required
`routine.calc_doas_sup_conds_for_sizing` after `size_zone_equipment` and
before `sim_zone_equipment`, plus the matching HVAC project item. The exact
routine is declared at `ZoneEquipmentManager.hh` lines 244-254 and implemented
completely at `ZoneEquipmentManager.cc` lines 696-765. Its sole production
call expression is CP240 `sizeZoneSpaceEquipmentPart1` line 387, reached only
for a current Zone or Space sizing role whose `AccountForDOAS` is true.

The helper first writes `DOASSupTemp = 0.0` and then `DOASSupHR = 0.0`.
`NeutralSup` clamps temperature below Low or above High, using outdoor
humidity below Low and `min(OutHR, W90H)` above High; its middle branch passes
through both outdoor values. `NeutralDehumSup` always selects High
temperature, using outdoor humidity below Low and `min(OutHR, W90L)`
otherwise. `CoolSup` selects High temperature plus outdoor humidity below
Low, otherwise Low temperature plus `min(OutHR, W90L)`. Comparisons are raw
strict `<` and `>` with no epsilon, threshold-order check, finite check,
nonnegative-humidity check, or clamp beyond those explicit branches.

The unqualified `min` is ObjexxFCL's `a < b ? a : b`, not `std::fmin`.
Ordinary values select the numeric minimum, but ties select the second
`W90*` operand, including its signed-zero bit. A NaN first operand therefore
selects a finite second operand, while a NaN second operand is selected after
a finite first operand. Raw IEEE comparisons also send `OutDB = NaN` to the
`NeutralSup` pass-through branch and to the other strategies' else branches.
With inverted thresholds, the first `OutDB < Low` test owns the overlapping
`NeutralSup` range.

`Invalid`, `Num`, and cast enum values outside the three valid enumerators
retain the two zero writes and then fatal with
`CalcDOASSupCondsForSizing:illegal DOAS design control strategy`. Valid paths
do not read or mutate `state`; only the fatal path uses it for diagnostics.
There is no local latch, allocation, numeric-input validation beyond enum
dispatch, status, catch, cleanup, checkpoint, transaction, or rollback.
Output-reference aliasing is unchecked: temperature is written first and
humidity second, so the final shared value is the humidity result. All
scalar/control calculation inputs other than `state` are passed by value.

CP240 has already reset current response state, rebuilt demands, snapshotted
pre-DOAS loads, validated inlet count, calculated the two 90%-RH values, and
calculated DOAS mass flow before calling CP243. Only a normal return permits
its heat-capacity, enthalpy, load, demand, inlet-node, and sizing-record
suffix. An invalid-control fatal therefore retains the completed model-state
prefix, writes only stack-local outputs to zero without publishing them to
node or sizing state, and suppresses the current suffix, later Part1 roles,
mass/leaving barriers, all Part2 roles, and the production manager suffix.
A valid direct repeat deterministically overwrites its two outputs; an
invalid repeat can zero again and repeat the fatal diagnostic. A CP242
retry remains generally non-idempotent because it replays the wider Part1
transaction.

The direct helper test makes seven calls and has 14 output assertions:
three `NeutralSup`, two `NeutralDehumSup`, and two `CoolSup` calls cover every
valid branch. Its finite ordered inputs test only cap-selected min branches.
It does not cover equality, inverted thresholds, IEEE specials, signed zero,
invalid enum, output aliasing, failure, or retry. Six direct
`SizeZoneEquipment` wrapper calls across three tests cause only three CP243
executions: two `CoolSup` else/cap executions and one `NeutralSup`
high/non-cap execution. Six stored-output assertions observe those results;
the other four wrapper calls have DOAS disabled.

Across one parent invocation in each of the 17 reaching direct
`ManageSizing` contexts, all 24 Zone roles have `AccountForDOAS` false, so
CP243 is not reached. Across one parent invocation in each of the 34 reaching
among 56 completing active full simulations, the static aggregate is 48
Zones plus 21 Spaces; exactly six Zone roles and no Space role enable DOAS.
Five fixed `CoolSup` Zones use 12.8/15.6 C setpoints, while one defaults to
auto-resolved `NeutralSup`; only downstream results are asserted.
Exact repeated sizing and dynamic call counts remain uninstrumented; each
context contributes only its own subset.

Rust has no exact CP243 symbol, snake-case counterpart, `DOASControl`,
`AccountForDOAS`, or DOAS sizing-output field. Its PurchasedAir outdoor-air
supply path, IdealLoads supply limits, and psychrometric helpers are adjacent
runtime behavior, not the `Sizing:Zone` DOAS selector. `Sizing:*` and
`ZoneSizing*` remain run-blocked, the sole raw `Sizing:Zone` fixture fails
before runtime, and the active data-model corpus contains no `Sizing:Zone`.

CP243 adds no algorithm-level EnergyPlus source, Rust target/code/state,
test, object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The algorithm
remains `scaffold` with claim level `none`. The inventory becomes 32
algorithms and 248 routines, split 58 `state_mapped` plus 190
`source_mapped`, with 125 required; the heat-balance project list remains 88
and the HVAC list becomes 14.

## CP244 `SetUpZoneSizingArrays` One-Time Sizing-State Constructor

CP244 adds canonical required `routine.set_up_zone_sizing_arrays` after
`calc_doas_sup_conds_for_sizing` and before `sim_zone_equipment`, plus the
matching HVAC project item. The exact routine is declared at
`ZoneEquipmentManager.hh` line 109 and implemented completely at
`ZoneEquipmentManager.cc` lines 767-1082.

Its sole production call expression is `SizeZoneEquipment` line 644 under
the default-true `SizeZoneEquipmentOneTimeFlag`. The caller clears that latch
at line 645 only after CP244 returns normally. Direct calls do not read or
write the latch. Production therefore runs setup once on the first reached
sizing-parent entry in a fresh state, skips it on later sizing timesteps, and
retains a true latch after any setup abnormal non-return.

CP244 starts with local `ErrorsFound = false`. If `ZoneIntGain` alone is not
allocated, it delegates `AllocateIntGains`; that guard does not independently
check the other arrays the child creates. It then visits every
`ZoneSizingInput` in stored order. An exact HeatBalance Zone-name miss emits
a severe and latches the local error. For each record it recomputes whether
any equipment configuration is controlled. When at least one is, an exact
configuration-name match writes `ZoneNum`; a miss only warns outside pulse
sizing, and the matched configuration itself is not rechecked as controlled.
If either airflow method is `FromDDCalc`, CP224 lazily verifies the exact
thermostat name and owns another pulse-suppressed warning. With no controlled
configuration anywhere, every sizing input emits a severe and latches an
error. An empty sizing-input arena skips this validation loop entirely.

The still-separate `AutoCalcDOASControlStrategy` child then runs
unconditionally, even when the parent's local error is already true. It can
mutate and report DOAS setpoints and can issue its own earlier fatal for an
inverted low/high pair. On normal return, CP244 allocates four Zone sizing
arenas over design days and Zones, four analogous Space arenas only under
`doSpaceHeatBalanceSizing`, terminal-final member sequences, three zeroed
weather sequences per design day, and averaging storage.

Each controlled Zone next selects an exact-name sizing input or, when none
matches, unguardedly uses input 1 and emits the third pulse-suppressed
warning. A missing first input is therefore not locally protected. The
separate `fillZoneSizingFromInput` child fills the Zone and then each stored
Space under the sizing-Space gate from that same selected input. With EMS
present, CP244 registers 17 internal variables and six actuators per
controlled Zone; it registers none for Spaces.

The routine then scans every `DesignSpecification:OutdoorAir:SpaceList`.
Valid Space indexes are appended without first clearing persistent
`dsoaSpaceIndexes`; missing and already-seen members emit severe diagnostics,
set shared `dsoaError` and `ErrorsFound`, and duplicates remain appended.
A single shared `dsoaError` suppresses DSOA dereference and design-OA
calculation for every later Zone and Space child. CP244 calls the separate
`calcSizingOA` for controlled Zones first, then, when Space sizing is active,
for every globally indexed Space whose parent Zone is controlled.
Cross-Zone SpaceList membership can add to `ErrorsFound` while calculation
continues.

Finally CP244 writes the averaging-window EIO rows, global heating factor,
nonunit controlled-Zone heating factors, global cooling factor, and nonunit
controlled-Zone cooling factors. If the accumulated flag is true, only after
those writes it fatals with
`SetUpZoneSizingArrays: Errors found in Sizing:Zone input`. The routine owns
no status, catch, cleanup, checkpoint, transaction, or rollback.

A late failure preserves allocation, fills, OA and equipment mutations, EMS
registrations, EIO, diagnostics, and the true caller latch. Same-state replay
is not idempotent: SpaceList indexes append again and can create new
duplicates; DOAS and main EIO plus diagnostics repeat; EMS registration
attempts repeat; final Zone/Space records are re-zeroed, so peak occupancy is
rebuilt rather than carried across a full replay; selected member/weather
sequences reset, while other same-extent fields can remain. A partial
`AllocateIntGains` failure can also leave `ZoneIntGain` present, causing its
single guard to skip repair. Manager `clear_state()` rearms the caller latch
but does not reset all other modules, and `RezeroZoneSizingArrays` is not a
complete setup reset.

Three tests call CP244 directly. None immediately asserts a CP244-owned
field; five downstream assertions depend on its results. The AirTerminal
fixture covers two matching controlled Zone records and two unasserted
missing-thermostat warnings. The other two direct fixtures have no sizing
input or controlled Zone and append four plus six valid unique DSOA
SpaceList members. All three omit Space sizing, EMS, and enabled DOAS.
Six direct `SizeZoneEquipment` calls force the setup latch false and execute
CP244 zero times.

Seventeen fresh direct `ManageSizing` contexts each complete setup once:
their aggregate is 24 matching Zone inputs, controlled Zones, and successful
thermostat checks, with 24 Zone fills and OA calls but no Space fill, DOAS,
EMS, or DSOA SpaceList. Thirty-four sizing configurations among 56
completing active simulations likewise complete one setup each. Their static
aggregate is 48 matching Zone inputs and fills, plus 21 Space fills and OA
calls across seven Space-sizing configurations. Exactly six Zones enable
DOAS, one configuration has a valid unique two-member DSOA SpaceList, and
none reaches EMS registration. These are per-fresh-state setup counts, not
sizing-timestep counts.

No test isolates allocation extents or initialization, `ZoneNum`, copied
sizing fields, `MinOA`, EMS bindings, sizing-factor EIO, latch transition,
pulse warning suppression, fallback, malformed Zone or SpaceList input,
child failure, partial prefix, retry, or reset. Direct and corpus evidence is
normal-path composition only.

Rust has no exact CP244 symbol, snake-case counterpart, `ZoneSizingInput`,
Final/Calc Zone or Space sizing arenas, terminal sizing arrays, design-day
weather sizing store, DSOA SpaceList population, EMS sizing binding, or
sizing-factor EIO path. Its typed Zone, Space, ordinary `SpaceList`,
thermostat, equipment connection, and individual DSOA objects are adjacent
typed or limited-runtime subsets, not this setup transaction. Authored
`Space` and ordinary `SpaceList` remain run-blocked;
`DesignSpecification:OutdoorAir:SpaceList` is untyped, and `Sizing:*`,
`ZoneSizing*`, and EMS remain run-blocked.

CP244 adds no algorithm-level EnergyPlus source, Rust target/code/state,
test, object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The algorithm
remains `scaffold` with claim level `none`. The inventory becomes 32
algorithms and 249 routines, split 58 `state_mapped` plus 191
`source_mapped`, with 126 required; the heat-balance project list remains 88
and the HVAC list becomes 15.

## CP245 `calcSizingOA` Zone/Space Outdoor-Air Sizing Mutator

CP245 adds canonical required `routine.calc_sizing_oa` immediately after
`set_up_zone_sizing_arrays` and before `sim_zone_equipment`, plus the matching
HVAC project item. The exact routine is declared at
`ZoneEquipmentManager.hh` lines 111-117 and implemented completely at
`ZoneEquipmentManager.cc` lines 1084-1206. It returns `void`, mutates separate
final and calculated-final sizing records plus shared module state, reads the
non-const `dsoaError` reference without assigning it, and only raises
`ErrorsFound` on a cross-Zone DSOA SpaceList member. It validates neither
record aliasing, bool aliasing, indexes, allocation, nor a Space's parent Zone.

The only production call expressions are CP244's controlled-Zone call at line
1032 and optional controlled-parent Space call at line 1042. CP244 visits all
controlled Zones first, then globally indexed Spaces in ascending order, and
shares the same two error flags across the whole pass. CP245 snapshots the
final record's `ZoneDesignSpecOAIndex`, reads the Zone's signed integer
`Multiplier * ListMultiplier`, and selects Zone or Space floor area. Only a
positive DSOA pointer with false `dsoaError` is dereferenced. A DSOA SpaceList
then checks every positive member against `zoneNum`; each mismatch emits one
severe plus two continuation messages and sets `ErrorsFound`, but does not
break, remove the member, set `dsoaError`, or stop later calculation. Zero
member indexes are skipped locally. The same guarded block writes only the
final record's per-person and per-area design OA rates; an existing value is
retained when the guard is false.

CP245 scans the complete People arena. Zone roles select `People.ZonePtr`,
while Space roles select `People.spaceIndex`; each matching design count is
multiplied by the Zone multiplier. Peak occupancy is accumulated with `+=`.
A strictly positive schedule maximum scales the contribution, whereas zero,
negative, or NaN maxima fall back to the full design count. Minimum occupancy
always uses the schedule minimum without a clamp. Null schedules, inconsistent
Zone/Space ownership, negative values, and non-finite values have no local
protection, and schedule extrema are lazily cached by the child accessors.

The final record then receives multiplied floor area, total design People,
and per-person and per-area OA totals. Minimum breathing-zone OA for the
predefined report uses minimum scheduled occupancy and
`std::min(ZoneADEffCooling, ZoneADEffHeating)`, replacing either signed zero
with one; negative and NaN values otherwise flow through. Only a Zone role
publishes `ZonePreDefRep.VozMin`; a Space role computes and discards that
report value. The selected Zone or Space equipment configuration always
stores the DSOA and air-distribution indexes, including when `dsoaError` is
already true.

With false `dsoaError`, CP245 delegates
`calcDesignSpecificationOutdoorAir` using four false control flags, the
current role, and the default-enabled IAQ-method path. The child owns DSOA
method arithmetic, SpaceList selection, multiplier application,
contaminant-state dependencies, diagnostics, fatals, and persistent warning
flags. CP245 does not multiply its returned OA again. The local accumulator
stays zero when the child is suppressed; otherwise it receives the child
result, which CP245 writes to both `MinOA` records, then, if either final air-distribution effectiveness
is positive, divides by unqualified ObjexxFCL `min` and copies the final
answer back to calculated-final. This second minimum chooses the heating
operand on a tie, differs from the earlier `std::min` for NaN operands, and
can divide by zero or a negative/NaN operand; no finite or nonnegative clamp
exists and calculated-final effectiveness is ignored.

Per-area cooling and heating limits are freshly derived from role floor area
and Zone multiplier. Four final/calculated-final input-flow fields are then
scaled in place with `*=`, so direct same-state replay compounds any nonunit
multiplier. Every design or run-period design day finally receives exactly
five final and calculated Zone-array fields: `MinOA`,
`DesCoolMinAirFlow2`, `DesCoolMinAirFlow`, `DesHeatMaxAirFlow2`, and
`DesHeatMaxAirFlow`. Space roles do not write the corresponding Space daily
arrays; they overwrite the parent Zone's daily column, so under CP244 order
the highest global Space index belonging to a Zone supplies the last values.

The routine owns no status, catch, checkpoint, cleanup, transaction, or
rollback. Any failure preserves its completed prefix. In particular, an OA
child failure occurs after aggregates, `VozMin`, and equipment indexes but
before the new `MinOA`; an interrupted daily loop preserves earlier days and
earlier fields. Direct retry is non-idempotent because peak occupancy and the
four multiplier-scaled flows accumulate, diagnostics can repeat, and schedule
or OA-child caches and flags can change behavior. Aliasing the two sizing
records applies the four `*=` operations twice; aliasing the two bool
references lets a cross-Zone error suppress the same call's OA child. A full
CP244 replay zero-fills and refills final records, avoiding those two direct
record accumulations, but remains non-idempotent through DSOA indexes,
diagnostics, and child state. `RezeroZoneSizingArrays` and manager
`clear_state()` do not constitute a coordinated reset of all CP245 owners.

No C++ test calls CP245 directly or immediately asserts a CP245-owned field.
Static corpus reachability is 95 calls: 74 Zone and 21 Space roles. All enter with false
`dsoaError` and reach the OA child; 94 have positive DSOA pointers and one PIU
role has zero. The positive set contains 93 individual DSOA roles and one
valid two-member SpaceList role. OA methods are 34 Sum, 54 Flow/Person, and
six Flow/Zone plus the pointer-zero role. Forty-one calls match one People
object with a positive schedule maximum, while 54 match none. All 95 use
1.0/1.0 effectiveness; 67 roles use unit multipliers and 28 use 10. Existing
Beam, OccupantDiversity, OutputReportTabular, and Standard621 assertions are
downstream composition oracles, not isolated checks. No test covers the
owned scalar and daily writes, `VozMin`, equipment indexes, true
`dsoaError`, malformed or cross-Zone topology, schedule fallback, multiple or
null-schedule People, nonunit/IEEE effectiveness, Maximum OA, aliasing,
partial failure, direct retry, reset, or Space overwrite.

Rust has no exact `calcSizingOA` routine or Zone/Space sizing and design-day
arrays, shared `dsoaError`/`ErrorsFound` protocol, DSOA SpaceList cross-Zone
validation, mutable equipment OA/air-distribution indexes, air-distribution
effectiveness, per-People schedule-extrema accumulation,
multiplier-applied occupancy/floor-area state, `ZonePreDefRep.VozMin`,
in-place sizing-flow scaling, or design-day fanout. Typed Zone, Space, People,
schedules, IdealLoads equipment, and individual DSOA plus the PurchasedAir OA
design-flow helper are adjacent subsets only; authored Space/SpaceList, zone
grouping, and sizing remain run-blocked.

CP245 adds no algorithm-level EnergyPlus source, Rust target/code/state, test,
object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The algorithm
remains `scaffold` with claim level `none`. The inventory becomes 32
algorithms and 250 routines, split 58 `state_mapped` plus 192
`source_mapped`, with 127 required; the heat-balance project list remains 88
and the HVAC list becomes 16.

## CP246 `fillZoneSizingFromInput` Sizing-Input Projection and Sequence Allocation

CP246 adds canonical required `routine.fill_zone_sizing_from_input` after
`calc_sizing_oa` and before `sim_zone_equipment`, plus the matching HVAC
project item. The exact helper is declared at `ZoneEquipmentManager.hh` lines
119-126 and implemented completely at `ZoneEquipmentManager.cc` lines
1208-1400. It returns `void`, takes one const `ZoneSizingInputData`, two
mutable daily arrays, two mutable final records, and caller-provided identity.
It reads only the total design/run-period design-day count and manager
`NumOfTimeStepInDay`; it owns no Zone/Space lookup or input interpretation.

The only production call expressions are CP244 lines 876 and 886. For each
controlled Zone in ascending order, CP244 selects one exact-name or fallback
Zone sizing input, calls CP246 for that Zone, and, when
`doSpaceHeatBalanceSizing` is true, calls it for the Zone's stored
`spaceIndexes` in order using the same parent input. CP246 is
role-agnostic. A Space call targets the Space arrays and final records, writes
the Space name, and stores the global Space index even in the destination
field named `ZoneNum`; it neither reads the input's `ZoneName`/`ZoneNum` nor
checks the Space-parent relationship.

All daily records are processed before either final record. For each design
or run-period design day, CP246 obtains both normal and calculated record
references, writes both identities, projects the normal subset, projects the
calculated subset, then allocates/zeros normal and calculated sequences in
that order. It next writes both final identities, the complete final subset,
the calculated-final subset, and finally allocates/zeros those two sequence
sets. With a nonpositive summed day count, the daily loop is skipped but both
final records are still filled and dimensioned.

Every destination receives caller identity plus the same 35 input fields:
sensible supply-air methods, temperatures, differences, and humidity ratios;
cooling/heating airflow methods and input/per-area/absolute/fraction values;
sizing factors; DOAS enable/strategy/setpoints; Space concurrence and Zone
sizing method; latent enable, RH constants and shallow schedule pointers,
and latent method integers; and heat-coil sizing method/ratio.
`InpDesCoolAirFlow` and `InpDesHeatAirFlow` receive input
`DesCoolAirFlow` and `DesHeatAirFlow`. Enum, integer, pointer, and floating
values are copied without validation or arithmetic.

The four destination write sets are intentionally asymmetric:

| Destination kind | Member assignments | Additional fields beyond identity plus common 35 |
|---|---:|---|
| normal daily | 37 | none |
| calculated daily | 41 | latent cooling/heating design humidity ratios and differences |
| final | 47 | latent four; DSOA and air-distribution indexes; cooling/heating air-distribution effectiveness; secondary recirculation; ventilation efficiency |
| calculated-final | 45 | latent four; both indexes; cooling/heating air-distribution effectiveness |

Thus normal daily records retain any prior values in the four omitted latent
fields, daily records receive no OA/air-distribution indexes or
effectiveness, and calculated-final retains prior secondary-recirculation and
ventilation-efficiency values. The two input object-name strings are not
copied; only resolved indexes reach the final pair. Production copies the
current DOAS values after `AutoCalcDOASControlStrategy` has already run.

After each member projection, `ZoneSizingData::allocateMemberArrays`
dimensions exactly 36 sequences from `HeatFlowSeq` through
`LatentHeatFlowSeq` to `NumOfTimeStepInDay` with `0.0`. ObjexxFCL
`dimension(range, value)` assigns the initializer even at an unchanged
extent, so every completed retry zeros all 36 sequences again. A completed
CP246 call therefore performs `2 * max(day_count, 0) + 2` member-array
helper calls. It does not initialize any member outside the listed projection.

There is no local validation of allocation, bounds, identity, topology,
record distinctness, enum values, finite/nonnegative values, timestep extent,
or old contents. Invalid enums, out-of-range method integers, negative or
non-finite values, and schedule pointers are copied raw. There is no
diagnostic, error flag, status, catch, checkpoint, transaction, cleanup, or
rollback. CP246 still runs when CP244 has already accumulated another input
error.

Failure preserves exact source-order prefixes. Both daily references are
obtained before the current day's first write, so failure obtaining the
calculated reference leaves that day untouched while preserving earlier
days. Later failure can leave normal or calculated member-assignment prefixes and a
partially dimensioned 36-sequence prefix. After every daily record completes,
both final identities are written before either final projection; both final
member-assignment subsets complete before final and calculated-final sequence
allocation. A CP246 abnormal exit prevents remaining role fills, later EMS
registration, DSOA population, CP245 OA work, sizing-factor EIO, and the
parent latch transition.

Mutable destinations need not be distinct. Aliased daily arrays finish with
the calculated-only latent four added to the common union and zero all
sequences twice. Aliased final records retain the final-only secondary and
ventilation fields because the later calculated-final block does not clear
them, and also zero sequences twice. If a final reference aliases a daily
element, the final suffix overwrites it after all daily work. Production
passes distinct stores.

With stable input, extents, and nonaliased destinations, a completed direct
retry deterministically overwrites the copied subset and rezeros sequences;
there is no CP245-style `+=` or `*=` accumulation. CP246 is not a full-record
reset, however. Omitted fields and all unrelated computed/EMS/OA/peak sizing
scalars survive. Same-extent parent replay can therefore preserve stale
normal-daily latent values and other untouched state even as CP246 resets its
sequences. The separate `RezeroZoneSizingArrays` wrapper delegates
`zeroMemberData`, which returns without changing that record unless
`DOASSupMassFlowSeq` is allocated. When that guard passes, the helper
zero-fills the current extents of 36 sequences and resets only 104 selected
members while preserving CP246 identity/static input fields. `ZoneEquipmentManager` state
reset does not own the DataSizing stores. Clean replay still requires
coordinated owner reset.

No C++ test calls CP246 directly or immediately asserts a CP246-owned write.
Static fresh-state reachability is 95 calls: two direct-CP244 Zone roles, 24
Zone roles across 17 `ManageSizing` contexts, and 48 Zone plus 21 Space roles
across 34 sizing-active simulations. The other two direct CP244 fixtures and
all six direct `SizeZoneEquipment` wrappers execute CP246 zero times. Of the
95 roles, 89 have DOAS disabled; five Zones use fixed cold-supply DOAS and one
uses auto-resolved neutral-supply DOAS. Zone sizing methods are 82
sensible-only/no-latent, nine sensible, and four sensible-and-latent, making
latent sizing active in 13 roles. Both RH schedule pointers are null and both
latent methods are humidity-ratio difference in all 95 roles. Existing
descendant sizing assertions prove only normal-path composition.

There is no isolated evidence for the four write sets, exact 36-array order
or zero contents, Space identity and parent-input reuse, calculated-final
omissions, zero design days, invalid/raw values, schedule pointers, malformed
array shapes, aliasing, allocation failure, partial prefix, retry,
same-extent stale fields, or reset behavior.

Rust has no exact `fillZoneSizingFromInput` routine, typed `Sizing:Zone`
input, Zone/Space design-day/final/calculated-final sizing arenas, source
field-copy asymmetries, or per-record timestep-sequence allocation. Typed
Zone/Space identities, schedules, Humidistat controls, individual DSOA,
IdealLoads operational supply limits, equipment graph, time-axis metadata,
and sizing-checked flags are adjacent subsets only; authored
Space/SpaceList, grouping, and sizing/autosizing remain blocked.

CP246 adds no algorithm-level EnergyPlus source, Rust target/code/state,
test, object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 251 routines, split 58 `state_mapped` plus 193
`source_mapped`, with 128 required; the heat-balance project list remains 88
and the HVAC list becomes 17.

## CP247 `RezeroZoneSizingArrays` Pulse-to-Normal Selective Sizing Reset

CP247 adds canonical required `routine.rezero_zone_sizing_arrays` after
`fill_zone_sizing_from_input` and before `sim_zone_equipment`, plus the
matching HVAC project item. The exact wrapper is declared at
`ZoneEquipmentManager.hh` line 128 and implemented completely at
`ZoneEquipmentManager.cc` lines 1401-1430. Its reset dependency is
`ZoneSizingData::zeroMemberData`, declared at `DataSizing.hh` line 646 and
implemented at `DataSizing.cc` lines 131-278.

The sole production expression is `SizingManager.cc` lines 400-402 after a
Zone sizing iteration's end-of-calculation updates. When an accepted
component-load report is requested and `DoZoneSizing`, at least one Zone
sizing input, and sizing periods are present, the caller selects two Zone
sizing iterations and makes the first a pulse pass. When
`isPulseZoneSizing && runZeroingOnce`, the caller invokes CP247 and clears
`runZeroingOnce` only after normal return. The latch defaults true and
`SizingManagerData::clear_state()` rearms it. There is no `ErrorsFound` gate,
so the condition can also be evaluated after the no-sizing-period severe
path. CP247 itself changes neither pulse/report flags, the latch,
`ZoneSizingRunDone`, nor component-load pulse/decay storage.

The wrapper first unconditionally emits
`Re-zeroing zone sizing arrays`. It then traverses global Zone indexes in
ascending order, reads same-index `ZoneEquipConfig`, and skips uncontrolled
Zones. For every selected Zone and each
`D = TotDesDays + TotRunDesPersDays`, it resets normal daily then calculated
daily records. After all days it resets calculated-final before final. Only
after all Zones, `doSpaceHeatBalanceSizing` gates an ascending traversal of
all global Space indexes. A Space is selected solely when its stored parent
Zone's equipment configuration is controlled; no Space-local control flag or
parent `spaceIndexes` membership is checked. Space record order is likewise
normal daily, calculated daily, calculated-final, then final.

A nonpositive `D` skips daily records but not either final record. With
`Cz` controlled Zones and `Cs` global Spaces whose parent is controlled, a
completed valid-state wrapper dispatches

```text
(Cz + (doSpaceHeatBalanceSizing ? Cs : 0))
    * (2 * max(D, 0) + 2)
```

`zeroMemberData` calls. Each record independently applies one sentinel guard:
if `DOASSupMassFlowSeq` is not allocated, the whole helper returns silently
without changing that record. This sequence is allocation step 25 of 36 in
CP246, so a partial allocation before it can leave earlier arrays and every
member untouched. A passing guard zero-fills the current, independently
retained extents of exactly 36 sequence fields; it neither allocates,
redimensions, nor normalizes heterogeneous extents.

All 36 sequence fills precede exactly 104 selected member assignments:
12 strings become empty, 80 `Real64` values become `0.0`, and 12 integers
become zero. No bool, enum, pointer, or allocation state is assigned. The
strings cover eight sensible/latent with/without-DOAS design-day names and
four sensible/latent peak dates. The integers cover sensible/latent peak
timestep and design-day indexes. Reals cover selected design flows, loads,
densities, coil-inlet states, current sensible/latent/DOAS state, and
sensible/latent peak conditions.

This is not a blanket `ZoneSizingData` clear. It preserves CP246 identity,
input methods, temperatures/humidity targets, flows and factors, DOAS
configuration, concurrence, indexes/effectiveness, latent RH pointers and
methods, and heat-coil sizing fields. It also preserves EMS flags/values,
OA/People/area aggregates, non-air and several no-OA results,
`ZonePeakOccupancy`, scalar `DOASHeatAdd`/`DOASLatAdd`, selected no-DOAS and
latent peak metadata, and all other unlisted state. Component-load arrays
outside these records are untouched so the later decay/report pipeline keeps
its pulse evidence.

There is no local allocation, bounds, topology, extent, day-count, or old
state validation. Apart from the progress output, it emits no
warning/severe/fatal and mutates no error state; it owns no status, catch,
checkpoint, cleanup, transaction, or rollback. Output is committed before the first indexed read. Failure retains
source-order prefixes: earlier Zones; normal before calculated within a day;
calculated-final before final; every Zone before any Space; and earlier
Spaces. A malformed Space parent fails before the current Space record.
Within a guard-passing helper, all sequence fills precede the 104-member
assignment prefix. Ordinary owning records are distinct; every reset write is
zero or an empty string, and completed direct replay is idempotent over the
touched subset, but
it repeats the progress line and never repairs guard-skipped or unlisted
state. Production abnormal return leaves `runZeroingOnce` true and a partial
reset for a retry; successful return makes later same-state caller entries
skip CP247 until the sizing-manager state is cleared.

The focused C++ unit calls CP247 once with five controlled Zones, 12 design
days, three run-period design days, four timesteps, and no Spaces. It
dispatches 150 daily records whose sentinel is allocated and ten unseeded
final records whose guard returns. For both daily kinds, active checks cover
only 58 of 104 reset members and 28 of 36 sequences; 172 assertion source
lines execute 25,500 checks. The eight missing sequence oracles are the two
no-DOAS sensible, four latent/no-DOAS load, and two latent-flow sequences.
Another 46 reset members are not seeded or asserted. Seventy-five seeded
preserved members have no active preservation assertion; 154 expectation
lines are commented out. Final mutation, guard no-op, Space and uncontrolled
selection, display, latch, failure, and replay are not proved.

Exactly six fresh production contexts reach CP247: two direct
`ManageSizing` tests and four full simulations, all through
`AllSummaryAndSizingPeriod`. Their aggregate is nine controlled Zones, no
Spaces, two design days per role, and no run-period design day: 36 daily plus
18 final guard-passing records. Six records have extent 24 and 48 have extent
96, for 171,072 statically zero-filled sequence slots. Downstream checks
cover selected component-load reports, final sizing, and OA results only
after the following normal pass; none isolates the intermediate reset,
message, call count, flags, or latch transition.

Rust contains no exact `RezeroZoneSizingArrays`, `zeroMemberData`,
`runZeroingOnce`, `isPulseZoneSizing`, `ZoneSizingData`, Zone/Space sizing
record arena, or component-load pulse/reset/decay orchestration. Typed
Zone/Space identities, equipment graphs, IdealLoads scalar demand and limits,
OA helpers, time metadata, and a `sizing_checked` flag are adjacent only.
The raw `Sizing:Zone` fixture expects `UnsupportedSizing`, active cases have
neither executable sizing input nor component-load-summary requests, and
sizing remains run-blocked.

CP247 adds no algorithm-level EnergyPlus source, Rust target/code/state,
test, object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 252 routines, split 58 `state_mapped` plus 194
`source_mapped`, with 129 required; the heat-balance project list remains 88
and the HVAC list becomes 18.

## CP248 `updateZoneSizingBeginDay` Calculated Daily Metadata Seed

CP248 adds canonical required `routine.update_zone_sizing_begin_day` after
`rezero_zone_sizing_arrays` and before `sim_zone_equipment`, plus the matching
HVAC project item. The exact role-agnostic helper is declared at
`ZoneEquipmentManager.hh` line 132 and implemented completely at
`ZoneEquipmentManager.cc` lines 1431-1453.

The only helper call expressions are in the `UpdateZoneSizing` `BeginDay` arm.
The sole production parent expression is `SizingManager.cc` line 307, once per
non-warmup day in each retained, non-`RunPeriodWeather` Zone-sizing
environment and iteration, before Facility begin-day and the hourly
simulation. Component-load reporting executes pulse then normal iterations,
resets `CurOverallSimDay` for each, and therefore rewrites the same daily
records after CP247 runs between the passes.

The parent scans Zone indexes ascending, skips an uncontrolled Zone, writes
its `CalcZoneSizing(CurOverallSimDay, zone)`, then, only under
`doSpaceHeatBalanceSizing`, writes every
`CalcSpaceSizing(CurOverallSimDay, space)` in that Zone's stored
`spaceIndexes` order. There is no Space-local control check, global Space
scan, sort, deduplication, or membership/parent validation. For `C` controlled
Zones and `M` stored membership occurrences under them, one completed
begin-day parent call dispatches
`C + (doSpaceHeatBalanceSizing ? M : 0)` helpers. Only calculated daily
records are selected; normal daily and both final record families are
untouched. This Zone-then-its-Spaces order differs from CP247's all-Zones then
global-Spaces traversal.

The branchless helper performs exactly 20 ordered assignments:

1. `CoolDesDay` and `HeatDesDay` copy `EnvironmentName`;
2. `DesHeatDens` and `DesCoolDens` copy raw `StdRhoAir`;
3. `HeatDDNum` and `CoolDDNum` copy raw `CurOverallSimDay`;
4. six sensible/latent with/without-DOAS design-day names copy
   `EnvironmentName`;
5. six sensible no-DOAS and latent day indexes copy `CurOverallSimDay`;
6. `CoolSizingType` becomes `Cooling`, then `HeatSizingType` becomes
   `Heating`.

That is ten string, two `Real64`, and eight integer writes, with every source
read repeated rather than locally snapshotted. Empty names, non-finite or
negative density, and arbitrary direct-call day indexes are copied unchanged.
No sequence or result array is zeroed despite the parent's legacy BeginDay
comment. Outside the 20 named metadata members, identity/input state,
load/flow/condition peak values, peak timestep/date-string fields, OA/DOAS
load state, latent calculation state, EMS, pointers, extents, and allocation
persist; normal daily and both final record families are untouched. CP247 clears 16 of the 20
fields on a guard-passing pulse reset but preserves the two sensible no-DOAS
day indexes and both sizing-type strings; normal CP248 overwrites all 20.

There is no explicit allocation, local bounds/topology/finite/day validation,
diagnostic, status, latch, catch, checkpoint, cleanup, transaction, or
rollback. Parent lookup failure occurs before the current record is passed.
A later Space or helper failure retains prior Zone/Space records, and a string
assignment failure retains its statement prefix. Stable completed replay is
idempotent over only the 20-field subset; changed source values replace it,
while omitted state remains stale.

No C++ test calls CP248 directly. Two direct parent tests each dispatch one
controlled Zone, no Space, and day one with an empty environment name and
standard density zero or `1.20`; neither asserts a CP248 field. Across
production-style active tests, 105 parent begin-day calls dispatch 195
helpers: 153 Zone and 42 Space, split into 135 normal Zone, 42 normal Space,
and 18 pulse Zone writes. The sole direct member-name descendant is one
`CalcFinalZoneSizing.HeatDesDay` assertion after later peak/final processing.
Another 60 predefined-table design-day assertions are composite report
evidence. No immediate oracle covers the 20-write transaction, density/day
copies, labels, invalid state, topology, failure, replay, warmup, or
run-period-design-day behavior.

Rust has no exact helper, field family, overall sizing-day index,
Zone/Space calculated sizing arena, begin-day dispatcher, stored-Space
traversal, or downstream sizing peak/report transaction. Run-period timing,
design-day schedule labels, EIO parsing, standard-density-derived IdealLoads
limits, identities, and equipment graphs are adjacent only. Four active-case
IDFs contain raw design-day declarations but disable Zone sizing and the
runtime ignores them; the raw `Sizing:Zone` fixture remains run-blocked.

CP248 adds no algorithm-level source, Rust target/code/state, test, object
support, capability, output implementation, comparator, case, manifest,
numerical, performance, or conformance promotion. The parent algorithm
remains `scaffold` with claim level `none`. Inventory becomes 32 algorithms
and 253 routines, split 58 `state_mapped` plus 195 `source_mapped`, with 130
required; the heat-balance project list remains 88 and the HVAC list becomes
19.

## CP249 `updateZoneSizingDuringDay` System-Substep Sizing Accumulation

CP249 adds canonical required `routine.update_zone_sizing_during_day` after
`update_zone_sizing_begin_day` and before `sim_zone_equipment`, plus the
matching HVAC project item. The exact helper is declared at
`ZoneEquipmentManager.hh` lines 134-141 and implemented completely at
`ZoneEquipmentManager.cc` lines 1455-1506.

The only helper expressions are in the `UpdateZoneSizing` `DuringDay` arm.
The sole production parent expression is `HVACManager.cc` line 475, inside
the accepted `SysTimestepLoop` and under both `!WarmupFlag` and
`ZoneSizingCalc`. The full-zone trial and optional optimized-condenser HVAC
repeats have no separate CP249 expression: a no-downstep result is accumulated
once by the one-iteration loop, while adaptive downstepping recalculates and
accumulates once per smaller accepted system substep. Each call uses
`FracTimeStepZone = TimeStepSys / TimeStepZone` and one zone-timestep slot
computed from hour, timesteps per hour, and current timestep. All substeps of
that zone timestep share the slot.

The parent scans controlled Zones ascending. For each it passes current-day
normal and calculated Zone records, the Zone thermostat pair, and that Zone's
final high/low extrema, then conditionally visits stored Spaces in container
order. Space calls use Space normal/calculated records but reuse their parent
Zone's thermostat values and the same parent final extrema; there is no
`FinalSpaceSizing` target, Space-local control check, global scan, sort,
deduplication, or membership validation. One parent call dispatches
`C + (doSpaceHeatBalanceSizing ? M : 0)` helpers for controlled Zone count
`C` and stored membership occurrence count `M`.

The helper first applies two raw strict conditions. Positive `tstatHi` replaces
`sizTstatHi` only when greater. Positive `tstatLo` then replaces
`sizTstatLo` only when less than the possibly updated **high**. It never reads
the old low, whose declaration default is `1000.0`; low is the last eligible
positive value below current high, not a running minimum. NaN comparisons are
false, and equal, zero, or negative inputs do not update. Both possible
extrema writes are unweighted and occur before sequence access. Valid Space
calls cannot raise high after their preceding Zone call with identical input.

Next, CP249 unconditionally overwrites four normal-daily slots in exact order:
heating design setpoint, heating calculated thermostat temperature, cooling
design setpoint, and cooling calculated thermostat temperature. These copy
raw values without fraction weighting, so the last completed system substep
wins.

It then applies 22 unconditional calculated-daily
`destination += source * fracTimeStepZone` statements: seven heating
flow/load/Zone/outdoor/return/humidity fields, the analogous seven cooling
fields, and eight DOAS load/addition/supply fields. There is no
`AccountForDOAS` gate. When `zoneLatentSizing` is true, eight more weighted
additions follow: latent heating/cooling load and flow, four no-DOAS load
fields, including sensible `CoolLoadNoDOASSeq` and `HeatLoadNoDOASSeq` inside
the latent gate. A false gate preserves those eight elements.
`HeatFlowSeqNoOA` and `CoolFlowSeqNoOA` are never CP249 targets.

Thus one latent-false helper mutates 26 sequence elements and a latent-true
helper 34, plus zero to two extrema scalars. The four normal fields overwrite;
the 22 or 30 calculated fields accumulate. The fraction and the 22/30
additive source scalars are neither checked nor normalized; negative,
greater-than-one, zero, NaN, and infinite values follow raw IEEE arithmetic.
A zero fraction can still create NaN from infinity, and accumulation order can
change rounding.

CP246 provides initial array allocation/zeros. Under consistent production
topology, a completed guard-passing CP247 clears every sequence CP249 touches
between pulse and normal passes, but does not clear the two final extrema;
pulse extrema carry into normal. The different CP247 global-Space and CP249
stored-membership traversals do not guarantee that reset for malformed
topology. CP248 updates disjoint calculated-record metadata and is not a
prerequisite for a direct CP249 call. CP249 has no local reset, latch, allocation, bounds, extent,
timestep, fraction, topology, finite-value, role, diagnostic, status, catch,
checkpoint, cleanup, transaction, or rollback.

Possible extrema changes precede the first of 34 independent sequence
accesses. Failure retains that scalar and array-statement prefix. Retry
overwrites the four normal slots but repeats already committed `+=`
contributions, so stable replay is generally non-idempotent. Duplicate Space
membership also double-adds. Parent argument lookup failure occurs before
helper entry, with argument evaluation order unspecified. Production storage
is distinct, but direct callers can alias the two records, high/low refs, or a
scalar ref with record state and thereby alter later reads; no alias guard
exists.

No C++ test calls the helper directly. Two direct parent tests each use one
Zone, no Space, one slot, unit fraction, latent false, and positive thermostat
pairs. Both extrema conditions succeed, but neither extrema nor any sequence
element is asserted. The only test expectations naming the 26 unconditional
sequence fields belong to CP247's manually seeded reset test, which never
calls DuringDay; the eight latent-gated sequences and both extrema have no
direct oracle.

Adaptive traces are absent, so production-style active-test calls are not
exactly measured. Their one-system-substep nominal floor is 12,288 parent
calls and 23,424 helpers: 17,376 Zone plus 6,048 Space, with 21,840 normal and
1,584 pulse helpers. The latent gate is true for a nominal 3,744 and false for
19,680. Adaptive downsteps can increase those counts. Later final
thermostat/peak and sizing-table assertions are composite evidence after
moving average, peak selection, final propagation, and reporting, not
isolating CP249.

Rust has no exact helper, fraction, extrema, sequence family,
normal/calculated Zone/Space sizing records, dispatcher, or accumulation
transaction. Thermostat links, diagnostic setpoint series, demand snapshots,
IdealLoads timing, and adaptive run-period heat-balance averages are adjacent
only. No active case has `Sizing:Zone`; raw design days disable sizing and are
ignored, while the raw sizing fixture remains run-blocked.

CP249 adds no algorithm-level source, Rust target/code/state, test, object
support, capability, output implementation, comparator, case, manifest,
numerical, performance, or conformance promotion. The parent algorithm
remains `scaffold` with claim level `none`. Inventory becomes 32 algorithms
and 254 routines, split 58 `state_mapped` plus 196 `source_mapped`, with 131
required; the heat-balance project list remains 88 and the HVAC list becomes
20.

## CP250 `updateZoneSizingEndDayMovingAvg` Circular End-Day Smoothing

CP250 adds canonical required
`routine.update_zone_sizing_end_day_moving_avg` after
`update_zone_sizing_during_day` and before `sim_zone_equipment`, plus the
matching HVAC project item. This is the physical source-definition order. The
public helper is declared at `ZoneEquipmentManager.hh` line 143 and its
complete wrapper is `ZoneEquipmentManager.cc` lines 1508-1529:

```cpp
void updateZoneSizingEndDayMovingAvg(
    DataSizing::ZoneSizingData &zsCalcSizing,
    int const numTimeStepsInAvg);
```

The body has one `if`, no direct assignment, and at most 16 ordered
`General::MovingAvg` child calls. Twelve calculated-daily sequences are
unconditional:

```text
CoolFlowSeq
CoolLoadSeq
HeatFlowSeq
HeatLoadSeq
CoolZoneRetTempSeq
HeatZoneRetTempSeq
DOASHeatAddSeq
DOASLatAddSeq
CoolLatentLoadNoDOASSeq
HeatLatentLoadNoDOASSeq
CoolLoadNoDOASSeq
HeatLoadNoDOASSeq
```

There is no `AccountForDOAS` gate, and all four no-DOAS fields remain in this
unconditional set. Only when `zoneLatentSizing` is true does the wrapper then
smooth `LatentHeatLoadSeq`, `LatentHeatFlowSeq`, `LatentCoolLoadSeq`, and
`LatentCoolFlowSeq`, in that order. It targets only the current calculated
Zone/Space daily record. It does not touch normal-daily thermostat sequences,
final records, any scalar, either no-OA flow sequence, or CP249's remaining
14 calculated temperature, humidity, DOAS-load, and DOAS-supply sequences.

`General::MovingAvg` is declared at `General.hh` line 107 and implemented at
`General.cc` lines 374-393. For `N <= 1` it returns before inspecting or
allocating the array. For `N > 1` and extent `L`, it allocates `2L` scratch
elements, duplicates the original array into both halves while zeroing the
target, then evaluates:

```text
out(i) = sum(j = 1..N, scratch(L - N + i + j)) / N
```

For `2 <= N <= L`, this is a circular trailing mean of the current element
and the preceding `N - 1`, so early-day outputs wrap through end-of-day
samples. `N = L` is a whole-day mean. `N = L + 1` is still in bounds but
weights the current element twice; an empty array skips both loops. For a
positive extent and `N > L + 1`, unsigned index arithmetic reaches an invalid
element. ObjexxFCL asserts membership before raw storage access, so that
invalid index terminates with assertions enabled and has undefined behavior
otherwise; it is not a recoverable throw. Non-one-based arrays are likewise
unsupported by the hard-coded `1..size` traversal. No local guard normalizes
the window to the extent.

Production sequence extent is `24 * TimeStepsInHour`. The
`Sizing:Parameters` averaging-window field is an integer with minimum one and
no upper maximum. Blank, absent, nonpositive source fallback, and fast-mode
override paths select `TimeStepsInHour`; the only range warning is for a
window shorter than one hour. There is no upper clamp. Raw ordered additions
and division have no finite-value guard, so NaN, infinity, overflow, and
rounding behavior propagate. Each child snapshots its own entire target
before output, but a second completed call generally smooths the already
smoothed result and is not idempotent.

The `UpdateZoneSizing(EndDay)` parent first completes one entire smoothing
sweep: controlled Zones in ascending index order, each Zone first and then
its stored `spaceIndexes` when Space sizing is enabled. It passes only
`CalcZoneSizing(CurOverallSimDay, zone)` or
`CalcSpaceSizing(CurOverallSimDay, space)` and the one global window. There is
no Space-local control check, global Space scan, sort, deduplication,
membership validation, or parent validation. With `C` controlled Zones, `M`
stored membership occurrences, and `R` latent-true role occurrences, a
completed valid-state parent dispatches:

```text
H = C + (doSpaceHeatBalanceSizing ? M : 0)
helper calls = H
MovingAvg calls = 12 * H + 4 * R
```

Duplicate or cross-listed Space indexes therefore smooth the same calculated
record repeatedly. Only after the full CP250 sweep completes does the parent
start its analogous CP251 `updateZoneSizingEndDay` peak-selection sweep.
CP251 sees every role's fully smoothed arrays, including any compounded
duplicate. It selects peaks from smoothed load fields and reads paired
smoothed flow/return-temperature fields, but samples unsmoothed Zone/outdoor
temperature and humidity companions at those selected timesteps. CP250 writes
no peak or final scalar.

The sole production parent expression is `SizingManager.cc` line 374, after
all hourly/timestep work for a completed non-warmup sizing day and before
facility end-day processing or the current-overall-day increment. The parent
has no equivalent local guard, so direct calls bypass that cadence. A
load-component pulse pass also reaches CP250. After a successful pulse sizing
iteration, a guard-passing CP247 clears the selected arrays before the normal
pass under consistent topology. Because CP247 globally scans Spaces while
CP250 follows stored membership, malformed cross-listing can evade that reset.

CP250 has no status, diagnostic, catch, checkpoint, transaction, cleanup, or
rollback. Scratch construction `std::bad_alloc` leaves the current target
untouched but retains earlier child and role results. Once scratch exists, the
loops have no source-defined recoverable exception path. Invalid indexing
assert-terminates or has undefined behavior, so no post-failure state or retry
is guaranteed. Only as a hypothetical statement-order interruption model, the
copy loop could expose a zeroed prefix and the averaging loop could expose
completed outputs, a partial current element, and later zeros; this is not a
recoverable C++ guarantee. Defined re-entry after a completed call or caught
allocation failure starts at the first role and smooths prior completed arrays
again. Scratch-allocation non-return suppresses every CP251 call; a later
CP251 non-return occurs after all CP250 mutations are committed. Production
array members are distinct, so duplicate/cross-listed record identity is the
material same-record replay route.

No C++ test calls CP250 directly. Two unit tests call the EndDay parent
directly with one Zone, no Space, latent sizing false, extent one, and
`N = 1`; all 12 child calls return immediately and no assertion reads a CP250
target. The independent `General_MovingAvg` test uses a 12-element quadratic
array and checks all 12 outputs for `N = 1`, `N = 2`, and `N = 4`. It proves
the child algorithm, not CP250's field set, order, gate, or parent routing.

A fresh completing production-style census finds 105 parent calls and 195
helpers: 153 Zone plus 42 Space, split 177 normal and 18 pulse. Helper windows
are exactly `N = 1/4/6` in counts `4/87/104`; the 191 `N > 1` helpers perform
real smoothing. The latent gate is true for 26 helpers, all at `N = 6`, and
false for 169. Thus the corpus dispatches exactly 2,444 child calls: 48 no-op
calls at `N = 1`, 1,044 transformations at `N = 4`, and 1,352 at `N = 6`.
Unlike CP249, there is no adaptive-system-substep multiplier.

Eight `SizingManager` production runs assert exact downstream Zone/Space
design load, flow, design-day, and peak-time report values at `N = 6`,
including one final Space latent-cooling peak timestep. Those are composite
results after CP249 accumulation, CP250 smoothing, CP251 selection, final
propagation, and reporting. The reset test asserts eight overlapping
sequences but never calls CP250. No focused oracle covers all 16 targets,
parent `N > 1` before/after values, either latent-gate branch with sentinels,
duplicate topology, invalid windows/extents, IEEE-special values, scratch
allocation failure, invalid-access termination or undefined behavior,
hypothetical statement-order interruption, defined re-entry, or replay.

Exact `crates` and `data` searches find no CP250 helper or canonical key,
`MovingAvg`/`moving_avg`, `NumTimeStepsInAvg`, `Sizing:Parameters`,
`ZoneSizingData`, `zoneLatentSizing`, or any of the 16 target sequences. Rust
has no Zone/Space sizing-day arena, circular trailing-window transaction,
EndDay dispatcher, stored-Space sizing traversal, or peak-selection handoff.
Adaptive heat-balance weighted averages, schedule averages, report-frequency
`Average` classification, and run-period time state are adjacent only and do
not implement this design-day mutation.

No active case contains `Sizing:Zone` or `Sizing:Parameters`. Four active-case
files contain five raw `SizingPeriod:DesignDay` objects, but all disable Zone,
System, and Plant sizing and are ignored by the compatibility runtime. The
raw `Sizing:Zone` fixture expects `UnsupportedSizing`; sizing and authored
Space/SpaceList workflows remain run-blocked.

CP250 adds no algorithm-level EnergyPlus source, Rust target/code/state, test,
object support, capability, output implementation, comparator, case,
manifest, numerical, performance, or conformance promotion. The parent
algorithm remains `scaffold` with claim level `none`. Inventory becomes 32
algorithms and 255 routines, split 58 `state_mapped` plus 197
`source_mapped`, with 132 required; the heat-balance project list remains 88
and the HVAC list becomes 21.

## CP251 `updateZoneSizingEndDay` Daily Peak and Final-Period Reduction

CP251 adds canonical required `routine.update_zone_sizing_end_day` after
`update_zone_sizing_end_day_moving_avg` and before `sim_zone_equipment`. Its
complete source is `ZoneEquipmentManager.hh` lines 145-149 and
`ZoneEquipmentManager.cc` lines 1531-1944:

```cpp
void updateZoneSizingEndDay(
    DataSizing::ZoneSizingData &zsCalcSizing,
    DataSizing::ZoneSizingData &zsCalcFinalSizing,
    int const numTimeStepInDay,
    DataSizing::DesDayWeathData const &desDayWeath,
    Real64 const stdRhoAir);
```

The leaf has no EnergyPlus child, state argument, diagnostic, status, or
return value. It first overwrites final `CoolSizingType` then
`HeatSizingType` from the current day, so those strings follow the latest
call rather than necessarily the winning period.

Its ordered daily reducers use strict `>` throughout:

- sensible heat writes load, smoothed flow, Zone/outdoor/return conditions,
  humidity companions, and timestep;
- latent heat does the analogous nine writes only under current-record latent
  sizing, assigning both latent heat mass fields from one flow;
- one unconditional loop reduces sensible/latent heat and cool no-DOAS loads,
  with no latent or `AccountForDOAS` gate;
- sensible cool then optional latent cool follow; latent cool does not assign
  `ZoneCoolLatentMassFlow`.

With zero incumbents, only positive values win, ascending ties retain the
first timestep, NaN candidates lose, and a NaN incumbent blocks later
candidates. CP251 selects every ordinary load peak above from a
CP250-smoothed load array. Where the sensible and enabled latent reducers
have flow and return-temperature companions, they sample CP250-smoothed
values; their Zone/outdoor temperature and humidity companions remain
unsmoothed. CP251 reads no DOAS sequence, including the two DOAS-addition
arrays CP250 smooths.

Positive sensible mass becomes volume by its stored sensible density. OA
fraction is `clamp(MinOA / max(volume, 0.001), 0, 1)`, and current-day weather
is mixed with the sensible Zone peak. Latent mass divides by `stdRhoAir`, but
its OA denominator is still the corresponding sensible volume and its Zone
side still uses the sensible peak; only the weather index is latent. The four
mass-flow derivations are strictly positive-gated, but there is no finite
validation, no zero/sign validation of any density divisor, and no peak-index
bounds check. The ordered source clamp maps a NaN raw fraction to zero, but
multiplication does not short-circuit, so zero-weight NaN/infinity can still
propagate.

Final selection is flow-first and can form hybrid records:

| family | larger-volume branch | volume `else` / larger-load branch |
|---|---|---|
| sensible heat/cool | copies 22 fields, including volume/mass, seven sequences, five peaks, identifiers, density, and coil inputs; thermostat is omitted | unconditionally overwrites final density with `stdRhoAir`, then a larger load copies 19 fields including thermostat but retains prior volume, mass, and flow sequence |
| latent heat | copies 14 fields, mass from `ZoneHeatLatentMassFlow`, no outdoor latent peak | copies only load, date/DD/time, and load+flow sequences; day name and peak/coil/flow scalars remain stale |
| latent cool | analogous 14 fields, mass from `DesLatentCoolMassFlow`, no outdoor latent peak | copies load, date/DD/day/time and only the load sequence |
| four no-DOAS loads | each strict winner copies scalar, sequence, DD, day, and time | no alternate branch and no date-string copy |

Strict cross-day ties retain the prior winner. A larger-volume day may lower
the associated final load; a lower/equal-volume but higher-load day may
replace load companions while retaining prior flow state. Any sensible
volume loser overwrites the selected density even when its load also loses.

Four zero-load fallbacks follow. Sensible heat chooses the within-day minimum
Zone temperature, then inclusively prefers a lower/equal paired outdoor
temperature across days and copies 17 companion fields. Sensible cool chooses
the within-day maximum and strictly prefers a higher paired outdoor
temperature. Latent heat selects the current-day minimum Zone temperature
while mutating only the current-day record's latent Zone temperature and
paired outdoor temperature/humidity. An independent scan selects the final
minimum outdoor temperature and its humidity companion plus metadata, but
copies the current-day record's existing latent timestep rather than the loop
index; it copies no final latent Zone peak or sequence. Latent cool mutates
a running current-day maximum, compares its paired outdoor value against a
final threshold that CP251 never updates, and writes only day/DD/date/stale
time. It is not a maximum reducer.

The EndDay parent completes the entire CP250 Zone/Space smoothing sweep before
starting this CP251 sweep. It scans controlled Zones ascending, then each
stored Space in order when Space sizing is enabled, with no local Space
control, global scan, sort, deduplication, membership, or parent validation.
Duplicate/cross-listed Space identity repeats CP251 against the same daily and
final pair after CP250 has already multiply-smoothed it. The sole production
call is once per completed non-warmup sizing day before facility EndDay and
the day-index increment; direct callers bypass those guards.

A pulse sizing iteration also reaches CP251. CP247 later resets most fields
before normal sizing, but preserves the sensible no-DOAS heat/cool peak
timesteps in daily and final records, their DD numbers in final records, and
the latent heat/cool Zone peak temperature/humidity in both; CP248/CP251
overwrite the also-preserved sizing labels.
No-winner normal paths can retain that pulse state; no current test has a
latent pulse role. Malformed stored membership can also evade CP247's
actual-parent Space scan.

`T <= 0` skips all nine possible loops but not stale-scalar finalization.
Out-of-range sequence/weather access assertion-terminates or has undefined
behavior and supplies no defined continuation. String and whole-array copies
can allocate after a strict winning scalar has committed; a caught allocation
failure can leave companions incomplete, and equality makes retry skip that
branch. Successful replay can also be non-idempotent: an equal sensible
volume enters `else` and can replace the copied density with `stdRhoAir`.
Parent replay first reruns CP250. The two references may alias, collapsing
ordinary final strict winner comparisons into self-comparisons and producing
a separate unvalidated in-place hybrid.

No C++ test calls CP251 directly. Two direct EndDay parent tests each use one
Zone, no Space, extent one, and latent false, but their four peak assertions
occur after helper 7 rewrites those peaks; they prove only integrated reach.
A BaseSizer full simulation pins the positive-heating-load/zero-flow fallback.
One latent Space simulation pins calculated-final latent cooling timestep 72.
Seven Space-sizing simulations assert downstream Space load, flow, day, and
peak-time reports.

The completing production-style corpus has exactly 105 parents and 195
helpers: 153 Zone plus 42 Space, split 177 normal plus 18 pulse. Helper
extents 24/96/144 occur 4/87/104 times; 26 latent-true helpers all have extent
144. The fixed daily peak scans execute 77,760 loop bodies and 148,032
comparisons. Six DOAS-enabled Zones contribute 14 calls, all latent false;
CP251 has no DOAS branch, and no test asserts a final no-DOAS field. Ties,
latent heat/zero fallbacks, hybrid/density behavior, latent coil asymmetry,
invalid state, aliasing, failure, retry, and pulse omissions remain
unisolated.

Exact Rust/data searches find no helper/key, calculated Zone/Space sizing-day
or final arena, peak reducer, or any of the 103 accessed members in token or
snake-case form. Current-timestep demand, IdealLoads limits/OA mixing, warmup
extrema, and sizing-object-name detection are adjacent only. Active cases
contain no `Sizing:Zone`, `Sizing:Parameters`, authored Space, authored
SpaceList, or Zone-sizing-enabled `SimulationControl`; sizing remains
run-blocked.

CP251 adds no EnergyPlus algorithm source, Rust target/state, support, output,
case, numerical, performance, or conformance promotion. Counts become 32
algorithms and 256 routines, split 58 `state_mapped` plus 198
`source_mapped`, with 133 required; heat-balance/HVAC lists become 88/22 and
HVAC readiness remains `0/22`. The parent stays `scaffold` with claim level
`none`.

## CP252 `updateZoneSizingEndZoneSizingCalc1` Noncoincident Space Aggregation

CP252 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc1` after
`update_zone_sizing_end_day` and before `sim_zone_equipment`. Its complete
source is `ZoneEquipmentManager.hh` line 151 and
`ZoneEquipmentManager.cc` lines 1946-2278:

```cpp
void updateZoneSizingEndZoneSizingCalc1(EnergyPlusData &state,
                                        int const zoneNum);
```

The leaf has no EnergyPlus child, diagnostic, output, status, catch, or return
value. It writes 92 calculated-final Zone members and accesses 95 unique
sizing-record member names across the Zone target and Space sources. It has
six explicit loops plus four ordinary and four latent-gated
`std::max_element` scans.

The sole production parent reaches EndZone sizing from `SizingManager` only
after at least one sizing period. It first runs Zone-sizing EMS, then
independently applies each of six Zone volume/mass/load overrides only when
EMS is present, that actuator's flag is on, and its preoverride target is
strictly positive. Only inside the non-pulse block, Space
sizing then visits controlled Zones ascending, skips exactly
`Zone.numSpaces == 1`, and calls CP252. The leaf binds the calculated-final
Zone record and returns unchanged only for exact `Coincident`; NonCoincident
and Invalid values rebuild. A normally completing non-Coincident call
therefore resets and rebuilds all
six EMS-adjustable fields from Space aggregates, including any applied
override.

The leaf does not recheck pulse, Space sizing, control, Zone bounds,
`numSpaces`, list length, membership parent, duplicates, cross-listing, Space
latent flags, or extents. It indexes `spaceIndexes[0]` after its numeric reset,
so malformed empty topology fails after that prefix. Stored order and
multiplicity are authoritative. A local Space counter increments but is
unused.

For target `F`, raw timestep count `T`, and latent gate `L`, reset and fold are:

| phase | sensible/unconditional | latent-gated |
|---|---|---|
| reset scalars | eight volume/load/mass/no-DOAS sums and 16 density/peak/coil numerators | eight latent sums and ten latent peak/coil numerators |
| reset arrays over `1..T` | 16 flow/load/no-DOAS/condition arrays | six latent load/flow/no-DOAS arrays |
| first-Space seed | 11 heat, heat-no-DOAS, and cool day/DD/date/timestep fields | 14 latent fields plus the three *ordinary* cool-no-DOAS fields |
| each Space | eight scalar sums; 16 peak products weighted by sensible design mass; six sequence sums; ten condition products weighted by timestep flow | eight latent sums; ten peak products; four DD checks; six sequence sums |

The ordinary cool-no-DOAS first-Space seed being inside `L` means a nonlatent
call begins that DD/name consensus from incoming Zone state. The first-Space
timestep copies are later replaced by maximum scans on normal completion.

Every ordinary or latent consensus compares only DD numbers. While the
current DD is nonzero, the first mismatch changes a primary day/DD/date to
`"N/A"/0/""` or a no-DOAS DD/day to `0/"N/A"`. Zero then latches off all
later comparisons. A first DD of zero suppresses mismatch detection from the
start; names, dates, and timesteps are never compared independently.

Sensible peak companions are divided by summed design mass only when it is
strictly positive. Timestep condition numerators are divided by summed
timestep flow only when positive. The four ordinary maximum scans then run.
Only afterward, under `L`, latent heat uses summed Space
`ZoneHeatLatentMassFlow` for both numerator weight and denominator, while
latent cool weights five peak/coil numerators by Space
`DesLatentCoolVolFlow` but divides by summed `DesLatentCoolMassFlow`; four
latent maximum scans follow. Nonpositive or NaN denominators leave raw
weighted sums; positive infinity enters division.

Each maximum scan recomputes a one-based timestep from its full allocated
array extent, not `1..T`:

- sensible heat/cool and their no-DOAS fields use their load arrays;
- latent heat uses `LatentHeatFlowSeq`, while latent cool uses its load array;
- latent no-DOAS fields use their corresponding no-DOAS load arrays.

Finite ties retain the first maximum; a portable NaN selection rule is not
claimed. Scalar loads/flows remain sums of independent Space peaks, so the
aggregate-sequence timestep can describe a different coincident peak.
Untouched tails can win when `T` is smaller than an extent.

CP252 is a subset rebuild. Thermostats, sizing configuration/labels, latent
outdoor peaks, Zone latent mass fields, DOAS state, EMS flags/values, and many
identity/input fields remain from the pre-call Zone record. The result can
therefore mix Space sums, weighted Space conditions, sequence maxima,
consensus/stale metadata, and untouched Zone state.

`T <= 0` skips timestep reset/fold/normalization but not scalar work,
metadata, or full-array scans. Excess `T`, invalid indexes, or malformed
extents assertion-terminate or have undefined unchecked behavior after an
ordered prefix. Floating sums/products preserve raw source-order IEEE
effects. String copies can allocate. Every no-DOAS mismatch arm sets DD
zero before
`"N/A"`, so a failure can leave a torn label in that invocation. Heat and
latent no-DOAS fields are reseeded from the first Space on retry; only
ordinary cool-no-DOAS under latent false lacks that reseed and can retain the
zero latch while skipping label repair. Stable valid replay normally
reconstructs the touched numerical subset, but it does not repair untouched
fields, tails, or that nonlatent cool-no-DOAS state.

Pulse EndZone skips CP252 entirely; the normal pass can later aggregate
pulse-preserved Space omissions. CP253 runs for all controlled Zones and
stored Spaces only after the complete CP252 Zone sweep, then owns diagnostics
and peak timestamp strings. Reporting and calculated-to-user copies remain
downstream.

No C++ test calls CP252 directly; two direct parent tests are pulse-gated and
dispatch none. Across 57 completing production-style EndZone parent entries,
only seven normal full simulations call CP252: five Coincident returns and
two NonCoincident bodies. Each call has one Zone, three Spaces, and `T = 144`;
three of the Coincident returns are latent true, while both NonCoincident
bodies are latent false and total six Space visits, 1,440 explicit
timestep-loop iterations, and eight maximum scans over 1,152 elements.

The two `SizingManager_ZoneSizing_NonCoincident*` tests strongly assert
downstream cooling load/volume Space sums. The common-day case retains the
day and reports `7/21 16:00:00`; the different-day case reports day `"N/A"`
and time-only `16:00:00`. Five Coincident tests retain Zone values distinct
from Space sums. There is no executed latent body, positive-heating, no-DOAS,
DOAS, EMS, pulse, weighted-field, malformed-topology, IEEE, failure, replay,
or retry oracle.

Exact Rust/data searches find no helper/key, concurrence type/value,
calculated-final Zone/Space arena, or any of the 95 sizing members in token or
snake-case form. Typed Zone/Space topology, demand, equipment sequences,
autosize wrappers, counters, and sizing-object names are adjacent only.
Active data contain no `Sizing:Zone`, `Sizing:Parameters`, authored
`Space`/`SpaceList`, `NonCoincident`, Space-sizing enablement, or
Zone-sizing-enabled `SimulationControl`; sizing and Space partitioning remain
run-blocked.

CP252 adds no EnergyPlus algorithm source, Rust target/state, support, output,
case, numerical, performance, or conformance promotion. Counts become 32
algorithms and 257 routines, split 58 `state_mapped` plus 199
`source_mapped`, with 134 required; heat-balance/HVAC lists become 88/23 and
HVAC readiness remains `0/23`. The parent stays `scaffold` with claim level
`none`.

## CP253 `updateZoneSizingEndZoneSizingCalc2` Supply-Delta Diagnostics and Peak Timestamps

CP253 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc2` after
`update_zone_sizing_end_zone_sizing_calc1` and before `sim_zone_equipment`.
The public leaf is declared at `ZoneEquipmentManager.hh` line 153 and
implemented completely at `ZoneEquipmentManager.cc` lines 2280-2387. Its
local formatting dependency `sizingPeakTimeStamp`, declared at header line
162 and implemented at source lines 2389-2399, stays bundled rather than
becoming a second routine row.

The sole production selector is `SizingManager.cc` line 391. On a normal
EndZone entry, the parent first applies EMS and all six independently gated
Zone overrides, completes the optional CP252 controlled-Zone aggregation
sweep, and then runs CP253 for each controlled Zone followed by every stored
Space occurrence when Space sizing is enabled. Unlike CP252, CP253 does not
skip a one-Space Zone. Only after the complete traversal does the parent write
ZSZ/SPSZ, perform latent selection through Calc3, and later run Calc4-7 and
final reports. Pulse sizing skips CP252, CP253, sizing-file output, and Calc3,
but the later copy stages still execute.

With `C` controlled Zones and `M` stored membership occurrences, a successful
parent dispatches `C + M` CP253 leaves under Space sizing and otherwise `C`.
The leaf does not validate role, control, topology, membership, duplicates,
cross-listing, record identity, indices, or extents. Stored order and
multiplicity are authoritative, so a repeated or cross-listed Space repeats
diagnostics and overwrites the same four strings.

The leaf accesses 29 unique `ZoneSizingData` members: 25 are read and only
four peak timestamp strings are written. Its source order is:

1. independently warn for an absolute cooling load at or below `1e-8`;
2. independently warn for an absolute heating load at or below `1e-8`;
3. analyze cooling only when its absolute load is above `1e-8`;
4. analyze heating only when its absolute load is above `1e-8`;
5. always write heat, cool, latent-heat, and latent-cool timestamp strings.

Each zero-load event is one warning plus one continuation diagnostic; it does
not return or suppress the timestamp tail. Signed zero and exact `+/-1e-8`
warn. Negative loads with greater magnitude enter analysis. A NaN load
satisfies neither comparison, while infinity enters analysis.

For either mode, only exact `SupplyAirTemperature` uses the supplied design
temperature. Every other method integer, including the intended
temperature-difference value and invalid values, takes the difference path.
Cooling forces `DeltaTemp = -abs(CoolDesTempDiff)` and reconstructs supply
temperature from the Zone peak; heating preserves the raw signed
`HeatDesTempDiff`.

For `D = abs(DeltaTemp)` and `HVAC::SmallTempDiff = 1e-5`, the diagnostic
partition is exact:

| condition | primary diagnostic | continuation diagnostics |
|---|---|---|
| `1e-5 < D < 2` | severe | nine, plus an optional wrong-direction note |
| `2 <= D < 5` | warning | nine, plus an optional wrong-direction note |
| outside that interval, `D > 1e-5`, and supply is in the wrong direction | severe | seven |

Cooling's wrong direction is supply above the Zone peak; heating's is supply
below it. Exact `1e-5` is silent, exact `2` is a warning, and exact `5` can
enter only the outer wrong-direction branch. There are four static warning,
four severe, and 36 continuation call sites; one receiver can execute at most
22 `Show*` calls. Diagnostics are side effects only and never block the four
timestamp assignments.

Those assignments always occur in strict
`HeatPeakDateHrMin`, `CoolPeakDateHrMin`, `LatHeatPeakDateHrMin`,
`LatCoolPeakDateHrMin` order, even when latent sizing is disabled. Each value
is the raw design-day date, one literal space, and a child-formatted time, so
an empty date creates a leading space. The child evaluates the signed-integer
product `timeStepIndex * MinutesInTimeStep * 60` before conversion to
`Real64`, passes it to `General::ParseTime`, discards returned seconds, and
formats the hour/minute with `PeakHrMinFmt = "{:02}:{:02}:00"`. It performs no
range, sign, cadence, day-extent, or overflow check and does not clamp or wrap;
a valid day end can format as `24:00:00`, while negative and out-of-day values
format mechanically. A sufficiently large integer product has signed-overflow
undefined behavior.

`writeZszSpsz` follows CP253 but does not consume these four strings. Calc3
later can replace sensible peak data and heat/cool strings with latent values,
so CP253 diagnostics observe prelatent sensible state while later predefined
reports can show a latent-selected string. Calc4-7 do not copy the strings.
`reportZoneSizing` reads calculated-final sensible strings only for a positive
final volume flow and otherwise writes `N/A`.

CP253 has no status, catch, transaction, rollback, or cleanup. A diagnostic or
formatting failure preserves emitted diagnostics and any earlier string
assignments, suppresses later receivers and sizing-file/report work, and can
leave a Heat-only, Heat/Cool-only, or three-string prefix. Stable direct replay
overwrites all four strings but repeats diagnostics. Whole-parent replay can
produce different diagnostics after an earlier successful Calc3 has replaced
sensible fields. Malformed record indices and timestamp integer overflow have
no defined local recovery.

No C++ test calls CP253 or `sizingPeakTimeStamp` directly. Two direct EndZone
parent tests switch pulse sizing on immediately and dispatch none. Across 57
completing production-style EndZone entries, 51 normal entries dispatch 93
leaves: 72 Zone and 21 Space; six pulse entries dispatch zero. The leaf makes
372 timestamp-helper calls, two sensible and two latent per receiver. Thirteen
receivers are latent-enabled, but CP253 has no latent gate: 26 latent formats
occur on those receivers and 160 more on 80 latent-disabled receivers.

Eight `SizingManager` simulations contain 58 downstream peak-time table
assertions. Eight `N/A` cells are the later no-flow report branch; the other 50
are composite CP253 descendants, split 36 Space and 14 Zone. They assert 49
sensible strings and exactly one latent-cooling descendant after Calc3; no
latent-heating string is asserted. All 21 Space receivers belong to the seven
Space-sizing runs, so their two sensible report paths are covered downstream,
apart from six no-flow cells. Only eight of 72 Zone receivers have analogous
table coverage.

The two NonCoincident tests prove eight zero-heating warning/continuation
pairs, and a BaseSizer full call proves at least one wrong-direction heating
severe event. No test asserts exact diagnostic text or count. Generic
`General_ParseTime` tests and a separate `TimeIndexToHrMinString` test are
adjacent evidence only, not wrapper parity. Direct string state, raw leading
space, latent heat, threshold boundaries, near-delta severity, cooling wrong
direction, invalid methods, negative/NaN/infinite inputs, duplicate topology,
invalid time indices, nonstandard cadence, overflow, pulse preservation,
failure, replay, and retry remain unisolated.

Exact Rust/data searches find no main or child helper, canonical key,
calculated-final Zone/Space sizing arena, supply-air sizing method,
`PeakHrMinFmt`, `MinutesInTimeStep`, diagnostic text, sizing-file artifact, or
counterpart for the 29-member boundary. The generic `zone_name`, thermostat
and IdealLoads supply constraints, mass-flow delta guard, schedule minutes,
and normalized ESO labels are adjacent only. All 61 active
`SimulationControl` objects disable Zone sizing; active data contain no
`Sizing:Zone`, `Sizing:Parameters`, `NonCoincident`, authored `Space`, or
`SpaceList`. Existing raw sizing and Space fixtures remain run-blocked.

CP253 adds no EnergyPlus algorithm source, Rust target/state, support, output,
case, numerical, performance, or conformance promotion. Counts become 32
algorithms and 258 routines, split 58 `state_mapped` plus 200 `source_mapped`,
with 135 required; heat-balance/HVAC lists become 88/24 and HVAC readiness
remains `0/24`. The parent stays `scaffold` with claim level `none`.

## CP254 `writeZszSpsz` Zone/Space Sizing-Series File Writer

CP254 adds canonical required `routine.write_zsz_spsz` after
`update_zone_sizing_end_zone_sizing_calc2` and before `sim_zone_equipment`.
The routine is declared at `ZoneEquipmentManager.hh` lines 155-160 and
implemented completely at `ZoneEquipmentManager.cc` lines 2401-2644. It takes
the raw role count, immutable calculated-final and design-day sizing arrays, a
mutable output handle, and `forSpaces`; it returns `void`, mutates no sizing
record, incrementally writes the stream, and closes that stream only at its
normal tail.

The sole upstream selector remains `SizingManager.cc` lines 390-393 after at
least one sizing period. Within nonpulse EndZone processing, the parent
completes EMS overrides, CP252, and every CP253 Zone/Space call before CP254.
It selects comma `.csv`, tab `.tab`, or otherwise `.txt`, ensures ZSZ open,
and always calls the Zone writer. Under Space sizing it then selects, opens,
and calls SPSZ. Only after both writers return does Calc3 perform latent
selection; Calc4-7 follow outside the pulse guard. Pulse sizing skips both
files and Calc3 but still runs Calc4-7.

`ensure_open` reuses an already-good stream even after the parent changes its
path; only a not-good handle is replaced by a truncating real stream when
enabled or a null stream when disabled. False output control therefore retains
any already-good sink but never skips CP254's loops or psychrometric work.
Open failure for a requested real stream is a parent-side fatal.
The leaf snapshots the raw separator and unconditionally closes even a
caller-supplied stream on normal completion.

Each of four receiver traversals uses `i = 1..numSpacesOrZones`. Zone mode
uses owner `i`; Space mode first reads global `space(i).zoneNum`. Eligibility
is `Zone(owner).IsControlled`, not the earlier
`ZoneEquipConfig(owner).IsControlled`. SPSZ consequently visits every global
Space once in global order and does not follow stored `Zone.spaceIndexes`;
duplicate/cross-listed memberships that repeat CP253 do not repeat SPSZ.
There is no role, owner, control-flag-consistency, membership, index, array,
or extent validation.

For each eligible receiver the writer emits `Time` plus 16 raw, unquoted
header fields. Every header is
`ZoneName:design-day-string:suffix`; separator, colon, and newline characters
are not escaped. The aligned columns are:

| columns | time-row values | `Peak` values | `Peak Vol Flow (m3/s)` values |
|---|---|---|---|
| 1-4 | sensible heat/cool load and mass-flow sequences | sensible design loads and mass flows | blanks, blanks, sensible heat/cool volume flows |
| 5-8 | latent heat/cool load and mass-flow sequences | latent design loads and mass flows | blanks, blanks, latent heat/cool volume flows |
| 9-12 | sensible heat/cool and latent heat/cool no-DOAS load sequences | four no-DOAS design loads | blanks |
| 13-16 | heating temperature/RH and cooling temperature/RH | four blanks | four blanks |

All 16 time values use `{:12.6E}`. Peak contains 12 scalars and four trailing
blank fields. The volume-flow row deliberately places four volume values
under the corresponding mass-flow headers and leaves the other 12 fields
blank. A literal leading newline before that row creates one blank physical
line.

The time loop runs 24 hours by raw `TimeStepsInHour`. It increments one
`TimeStepIndex`, adds `MinutesInTimeStep`, and resets minutes only on exact
equality with 60. Labels use `{:02}:{:02}:00`; normal cadence ends at
`24:00:00`, while inconsistent cadence can repeat, move backward, or print
minutes above 60. It neither calls CP253's `sizingPeakTimeStamp` nor checks
`NumOfTimeStepInDay`, range, cadence, or integer overflow.

Twelve final-sizing sequences are indexed and printed for every eligible
receiver/timestep regardless of latent sizing, DOAS, or sizing method. Only
positive sensible `HeatDDNum` and `CoolDDNum` gate the last four fields.
A positive day selects the corresponding daily temperature/humidity-ratio
sequences and computes
`100 * PsyRhFnTdbWPb(state, T, W, current OutBaroPress)`; a nonpositive day
leaves that mode's temperature and RH at zero. Positive out-of-range day
numbers are unchecked. The already-mapped psychrometric child owns humidity
ratio flooring, saturation/cache behavior, out-of-range RH clamping, and
optional diagnostics. CP254 omits `CalledFrom` and adds no local validation.

The leaf reads 39 unique calculated-final members and four daily members,
43 unique sizing-record member names in total. It does not read CP253 peak
timestamp strings, the latent flag, `AccountForDOAS`, a sizing-method enum, or
latent/no-DOAS design-day numbers. All latent and no-DOAS columns are emitted
unconditionally, and Space headers still say Zone Temperature and Zone
Relative Humidity.

For stable state, let `N` be the nonnegative candidate count, `K` eligible
receivers, `R = 24 * max(TimeStepsInHour, 0)`, and `P_H`/`P_C` eligible
records with positive heat/cool day numbers. One call performs
`N(R+3)` candidate filter visits, `K(R+3)` eligible receiver formats,
`(R+3)(K+2)` dynamic print calls, `12RK` final sequence reads,
`3R(P_H+P_C)` daily reads, and `R(P_H+P_C)` psychrometric calls. It emits
`16K(R+1)` numeric fields and issues `R+4` structural LF literals; raw
separator/name/day line breaks can create additional physical lines. Runtime is
`Theta(N(R+3) + formatted byte length)` without a size guard.

CP254 has no status, catch, transaction, rollback, cleanup guard, or stream
result check. Bad topology or array state can fail after an output prefix.
Psychrometric side effects occur after a time label and before its record
bytes. A non-return before the tail leaves the stream open; ordinary iostream
bad/fail state can instead be silently retained, followed by close and
downstream execution with truncated output. ZSZ failure suppresses SPSZ and
all later parent work; SPSZ failure preserves the completed closed ZSZ but
suppresses Calc3 and later work.

After successful close, a production retry with the corresponding output flag
enabled reopens with truncation and rebuilds stable bytes; with the flag false
it installs a null sink and leaves any prior physical file untouched. Both
repeat psychrometric cache/diagnostic effects. After an exception leaves a
still-good stream open, retry reuses it and appends a second header/prefix. A
not-good ordinary sink follows the same current-flag choice; the special bad
dev-null sink is treated as good and reused.
A whole-parent replay after a prior Calc3 completion can write different
bytes because Calc3 already mutated calculated-final sensible state.

No C++ test calls CP254 directly. Two direct EndZone parent tests set pulse
true and write nothing. A fresh completing production-style census has 58
writer calls: 51 ZSZ and seven SPSZ. They select 93 records, 72 Zone plus 21
Space. Calls at one, four, and six timesteps per hour number 1, 21, and 36,
producing 7,224 time lines, 11,496 eligible time-record blocks, 11,775 total
eligible receiver formats, 13,251 candidate visits, 26,571 dynamic prints,
7,456 physical lines, and 185,424 numeric fields. Exact positive-day counts
are not asserted, so psychrometric execution remains expressed by the
`P_H`/`P_C` formula.

Thirteen selected records are latent-enabled and 80 are disabled, yet all
format latent columns. Six known Zone roles enable DOAS, yet no-DOAS columns
remain unconditional. Tests provide upstream field variation but no writer
byte oracle. The unit fixture merely preopens ZSZ/SPSZ stringstreams that the
writer closes, and the output-control test checks only booleans. No test
asserts path, separator, header, column order, precision, blank fields,
timestamp, row count, bytes, close state, output error, or tracked golden
artifact. Tab/text routing, malformed input, nonfinite values, partial output,
silent badbit, replay, and retry remain unisolated.

Exact Rust/data searches find no main helper or canonical key, ZSZ/SPSZ
artifact/path/schema/format literal, calculated sizing arena, cadence/
separator/pressure integration, or counterpart for any of the 43 exact sizing
members; only unrelated generic `zone_name` resembles one snake form. Rust's
nearby ordinary-finite psychrometric projection has no writer integration.
All 61 active `SimulationControl` objects disable Zone sizing; active data
contain no `Sizing:Zone`, `Sizing:Parameters`, authored `Space`, or
`SpaceList`. Existing sizing and Space fixtures remain run-blocked.

CP254 adds no Rust target/state, test, support, output implementation,
comparator, case, numerical, performance, or conformance promotion. Counts
become 32 algorithms and 259 routines, split 58 `state_mapped` plus 201
`source_mapped`, with 136 required; heat-balance/HVAC lists become 88/25 and
HVAC readiness remains `0/25`. The parent stays `scaffold` with claim level
`none`.

## CP255 `updateZoneSizingEndZoneSizingCalc3` Latent Peak Selection

CP255 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc3` immediately after
`write_zsz_spsz` and before `sim_zone_equipment`. It is declared at
`ZoneEquipmentManager.hh` lines 164-167 and implemented completely at
`ZoneEquipmentManager.cc` lines 2646-2764:

```cpp
void updateZoneSizingEndZoneSizingCalc3(
    DataSizing::ZoneSizingData &zsCalcFinalSizing,
    Array2D<DataSizing::ZoneSizingData> &zsCalcSizing,
    bool &anyLatentLoad,
    int const zoneOrSpaceNum);
```

The nonpulse EndZone parent reaches CP255 only after CP254 has closed ZSZ and,
when enabled, SPSZ, so those files preserve preselection sizing state. It
traverses ascending controlled `ZoneEquipConfig` Zones, skips a whole Zone
unless that Zone's calculated-final `zoneLatentSizing` is true, calls the Zone
record first, then under Space sizing calls every stored
`Zone.spaceIndexes` occurrence. Space control, owner, membership consistency,
and Space-local latent flags are not checked. Pulse sizing skips CP255.
Calc4-7 start only after the complete CP255 sweep and consume its mutations.

Cooling and heating are independent ordered branches. For each mode, exact
`ZoneSizing::Latent` selects only when latent volume flow is strictly positive.
Exact `ZoneSizing::SensibleAndLatent` instead selects when latent load is
strictly greater than the current sensible load, despite the source comment
describing a volume-flow comparison. `Sensible`, `SensibleOnly`, `Invalid`,
other enum values, equality, and NaN comparisons select neither corresponding
branch. Cooling executes first and alone sets shared `anyLatentLoad = true`;
heating never sets or clears it. The flag is therefore a monotonic,
cooling-only cross-role latch.

A selected mode performs 16 calculated-final assignments:

- set the sizing-type literal to `Latent Cooling` or `Latent Heating`;
- replace ordinary volume flow, mass flow, and load with latent scalars;
- replace ordinary design-day name/date/number, peak timestep, and entire
  `CoolFlowSeq` or `HeatFlowSeq`;
- replace coil-in temperature/humidity ratio and return/Zone
  temperature/humidity ratio at the peak;
- replace the ordinary peak timestamp;
- replace design supply humidity ratio.

Only after all 16 final writes, a strictly positive final
`LatentCoolDDNum` or `LatentHeatDDNum` selects
`zsCalcSizing(finalLatentDay, zoneOrSpaceNum)` without upper-bound, role, or
extent validation. That daily record receives the same 15 assignments except
for the absent sizing-type literal. Its ordinary day number comes from its
own latent day number, while its ordinary date and peak timestamp come from
the final record's latent strings. Its humidity method and other latent values
are also read independently, so final and daily records can choose different
formulas or retain inconsistent day identity. A nonpositive final day skips
daily mutation after the final record is already replaced.

For both final and daily humidity, exact integer
`SupplyAirHumidityRatio` copies `Latent*DesHumRat`; every other integer uses
`ZoneHumRatAtLatentCoolPeak - CoolDesHumRatDiff` for cooling or
`ZoneHumRatAtLatentHeatPeak + HeatDesHumRatDiff` for heating. There is no
clamp, psychrometric call, finite check, or range validation. The routine
copies latent flow sequences but never latent load sequences and leaves
outdoor peak conditions, thermostat/density/no-DOAS fields, latent-source
fields, and all unselected design days untouched.

The body touches 67 unique `ZoneSizingData` member names: all 67 on the final
record and a 60-member subset on selected daily records. There are 32 unique
final destinations, 30 unique daily destinations, 66 static record-assignment
sites, and one shared-bool site. Let `C` and `H` indicate cooling/heating
selection and `d_C`/`d_H` indicate positive final latent day numbers. One leaf
executes

`16(C+H) + 15(C*d_C + H*d_H)`

record assignments, plus `C` flag assignment and
`C+H+C*d_C+H*d_H` whole-sequence copies. The maximum is 62 record assignments,
one flag assignment, and four sequence copies. Runtime is constant apart from
copied string and sequence lengths.

There is no local validation, diagnostic, status, catch, transaction,
cleanup, or rollback. Cooling latches the flag before its first record write.
String/sequence allocation or invalid positive daily lookup preserves every
prior assignment; cooling failure suppresses heating, while heating failure
preserves completed cooling. A final reference can alias a selected daily
array element, and stored duplicate/cross-listed Spaces repeat in source
order.

Once a partial or completed `SensibleAndLatent` selection reaches the
ordinary-load assignment, it overwrites ordinary load with latent load. On
replay the strict greater-than predicate becomes equality and skips that branch,
so any failure after the load assignment can strand a torn tail that direct leaf
retry cannot repair. Exact `Latent` with positive
latent volume flow re-enters. Whole-parent retry first replays CP253 and CP254
against the retained ordinary/latent mixture, so rebuilt file bytes and later
copies can differ.

No C++ test calls CP255 directly. Two direct EndZone tests set pulse true and
dispatch nothing. Across 51 normal and six pulse production-style EndZone
entries, only four `SizingManager` simulations dispatch CP255: 13 leaves,
four Zone plus nine Space. Nine are `Sensible` no-ops. Four are
`SensibleAndLatent`; only one Space selects cooling, uses positive final day
2, and executes both difference-humidity arms. Heating and exact `Latent`
never select. That one call performs 31 record assignments plus the flag;
with two 144-element flow-sequence copies, its scalar/element projection is
318 writes including the flag.

There is no direct assertion for a CP255 destination, daily record, sizing
label, humidity ratio, or shared flag. One assertion pins the selected
Space's latent-cooling source timestep, and five downstream table assertions
cover its selected load, two flow values, day, and peak time. The four
dispatching simulations contain 130 composite role/table assertions, of which
the other 125 are no-op/pass-through descendants rather than exact copy
oracles. Latent-volume selection, every heating branch, direct humidity
selection, nonpositive/invalid day state, mismatched final/daily state,
aliasing, duplicate topology, IEEE edges, failure, and replay remain
unisolated.

Rust contains adjacent `supply_air_humidity_ratio` and operational IdealLoads
latent calculation/reporting, but no calculated Zone/Space sizing arena,
Calc3 helper/key, sizing method, shared any-latent latch, or counterpart for
any of the 67 exact or mechanical snake-case members. All 61 active
`SimulationControl` objects disable Zone sizing; active data contain no
`Sizing:Zone`, `Sizing:Parameters`, authored `Space`, or `SpaceList`.
Existing sizing and authored-Space fixtures remain run-blocked.

CP255 adds no Rust target/state, support declaration, test, capability,
output implementation, comparator, case, manifest evidence, numerical or
performance claim, or conformance promotion. Counts become 32 algorithms and
260 routines, split 58 `state_mapped` plus 202 `source_mapped`, with 137
required; heat-balance/HVAC lists become 88/26 and HVAC readiness remains
`0/26`. The parent stays `scaffold` with claim level `none`.

CP256 next maps
`ZoneEquipmentManager::updateZoneSizingEndZoneSizingCalc4`, declared at
`ZoneEquipmentManager.hh` line 169 and implemented completely at
`ZoneEquipmentManager.cc` lines 2765-2799.
## CP256 `updateZoneSizingEndZoneSizingCalc4` Daily User-Array Projection

CP256 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc4` immediately after Calc3
and before `sim_zone_equipment`. It is declared at
`ZoneEquipmentManager.hh` line 169 and implemented completely at
`ZoneEquipmentManager.cc` lines 2765-2799:

```cpp
void updateZoneSizingEndZoneSizingCalc4(
    DataSizing::ZoneSizingData &zsSizing,
    DataSizing::ZoneSizingData const &zsCalcSizing);
```

The EndZone parent reaches CP256 only after the complete CP255 role sweep on
a nonpulse pass. Pulse sizing skips CP255, but the Calc4 loop at lines
3459-3466 is outside that guard and still runs. For each zero-based linear
`i` in `ZoneSizing.size()`, the parent first copies
`CalcZoneSizing[i] -> ZoneSizing[i]`. When Space sizing is enabled, it then
copies every linear `CalcSpaceSizing[j] -> SpaceSizing[j]` for that same
outer `i`. It checks no controlled flag, latent method, day identity, owner,
Zone membership, or corresponding-day relationship.

Let `Z = ZoneSizing.size()`, `S = SpaceSizing.size()`, and `I` be one when
Space sizing is enabled. The parent dispatches

`L = Z * (1 + I*S)`

leaves. It targets all `Z` Zone records once and, only when `Z > 0` and
`I = 1`, all `S` Space records `Z` times each. A zero Zone target therefore
suppresses Space copying even if Space targets exist; a false Space flag
leaves every Space target untouched. Normal setup allocates
`Z = D*N` and `S = D*P` for `D` design/run-period days, `N` Zones, and `P`
Spaces, giving `D*N + I*D*D*N*P` calls. Objexx flat order is day-major with
the Zone or Space index varying fastest inside each day, but each individual
Zone-day copy is followed by the complete all-day/all-Space sweep.

The destination sizes alone bound both loops. A longer source tail is
ignored; equal flat cardinality with different dimensions silently pairs
semantic records by linear position. A shorter source can hit an Objexx
assertion in an asserted build or invalid access in a release build. Calc4
owns no extent, allocation, shape, or overlap check.

Each valid leaf is branchless and performs 29 unconditional same-name
assignments in exact source order:

- two design-day name strings: `CoolDesDay`, then `HeatDesDay`;
- two densities and two design-day indexes;
- 11 heating fields: load, mass flow, five Zone/outdoor peak conditions,
  peak timestep, volume flow, and two coil-in conditions;
- `CoolDesHumRat` by itself;
- the symmetric 11 cooling fields.

That is two `std::string`, four integer, and 23 `Real64` assignments, 29
unique destination members, and 58 member accesses. Every right-hand side
is the identically named member of the const calculated record. There is no
predicate, arithmetic, unit conversion, clamp, finite/range check, child
call, state argument, or diagnostic. Negative, nonfinite, and invalid
day/timestep integers are copied as values rather than dereferenced here.

The executable projection is partial. It copies `CoolDesHumRat` but
not `HeatDesHumRat`, and copies no flow/load sequence, sizing-type label,
latent-source field, design-day date string, or peak timestamp. CP255 can
replace 30 unique daily calculated destinations across cooling and heating;
CP256 carries only 23 of them: 12 of 15 cooling fields and 11 of 15 heating
fields. It omits both date strings, both flow sequences, both peak strings,
and heating design humidity ratio. The remaining six Calc4 fields are the
two densities and four outdoor peak conditions that CP255 does not mutate.
Consequently a selected latent-heating calculated record can have a new
`HeatDesHumRat` while the user daily record retains its prior/input value.
Calc6 later handles the flow sequences; Calc4 itself does not.

One parent pass executes `29L` assignment statements, including `2L`
string and `27L` scalar assignments. Runtime is
`Theta(L + copied design-day-name bytes)` with constant local state. The
redundant Space sweeps are value-idempotent while their source values remain
stable; exact source/destination alias likewise performs self-copies.

There is no local status, catch, transaction, cleanup, or rollback. The two
possibly allocation-bearing string copies execute first. A defined failure
on the second preserves the completed `CoolDesDay`; no scalar has yet been
written. For valid live records the remaining 27 scalar assignments do not
throw. In the parent, a defined Space-leaf failure preserves the already
completed Zone for that outer iteration plus all earlier/repeated Space
copies and suppresses later outer iterations, Calc5-7, facility sizing, and
the run-done latch.

A later successful direct retry re-executes all 29 assignments and repairs
the subset from a stable source. Completed retry and repeated Space sweeps
converge to the same values only for stable sources. Exact source/destination
alias is permitted by the signature and becomes 29 same-name self-copies;
the const source reference does not prove disjointness. Whole-parent
nonpulse retry reruns EMS and any gated CP252 work, then CP253, CP254, and
any gated CP255 work. Their replay can change the calculated source before
CP256 overwrites its prior user-array prefix. Within the current attempt,
Calc4 cannot retroactively alter already closed CP254 artifacts. A
whole-parent retry reruns CP254 before CP255/Calc4, so rebuilt artifacts can
differ.

No C++ test calls Calc4 directly. The completing high-level corpus contains
51 normal plus six additional pulse EndZone entries and dispatches 271
leaves: 181 Zone and 90 Space. Seven normal Space-enabled contexts own 42
distinct Space targets but copy them 90 times, adding 48 structurally
redundant calls. Two direct pulse parent tests add one Zone leaf each.
Overall, 59 parents execute 273 leaves and 7,917 assignment statements:
546 string plus 7,371 scalar assignments over 203 distinct test-local
targets. Target multiplicity is 142 once, 52 twice, and nine three times.
The production Zone leaves split 153 controlled and 28 uncontrolled; all 90
Space leaves occur in seven controlled-Zone contexts, and both direct leaves
are controlled. Of 271 production leaves, 44 have latent sizing enabled:
30 `Sensible`, 14 `SensibleAndLatent`, and zero exact `Latent`; the other
227 are latent-off. None of those states changes Calc4 dispatch or writes.

Every assignment site therefore executes, but there is no direct leaf test,
no Calc4-executing `SpaceSizing` target assertion, and no Calc4-executing
assertion of any of the 29 `ZoneSizing` destinations. The two direct parent
tests inspect calculated or final records instead. There are 803 static
post-call gtest assertion sites plus one invocation-site `EXPECT_NO_THROW`
around `ManageSizing`; none compares a Calc4 source member with its target.
Sizing tables use calculated-final/final state, and the only daily user-array
reads found in that table path are `CoolTstatTempSeq` and
`HeatTstatTempSeq`, both outside Calc4's 29-field set. Calc7 later accesses a 19-member subset of Calc4 destinations,
with 13 right-hand-side reads and six conditional rewrites, but reported
results combine Calc5/Calc7 work and yield no uniquely attributable Calc4
oracle. Exact field identity/order, nondefault source coverage, nested
repetition, malformed shape, alias, failure, and retry therefore remain
isolated gaps. The separate Rezero test asserts reset behavior for 28 of
these fields but never invokes Calc4 and is not a copy oracle.

The Rust/data audit covers 721 current-worktree `crates` plus `data` files
. It finds no Calc4 key/helper, calculated
or user Zone/Space sizing arena, any of the 29 exact member names, or any of
their 29 mechanical snake-case forms. Rust does have adjacent current-step
`ZoneSysEnergyDemand`, IdealLoads supply/rate/mass-flow/humidity results,
typed limits, density/OA/node/report fields, and design-day schedule labels;
none is this daily calculated-to-user peak-sizing projection.

All 61 active `SimulationControl` objects disable Zone sizing. Active data
contain five raw design days but no `Sizing:Zone`, `Sizing:Parameters`,
authored `Space`, or `SpaceList`, and no corresponding epJSON keys. Sizing
and authored-Space fixtures remain run-blocked.

CP256 adds no Rust target/state, support declaration, test, capability,
output implementation, comparator, case, manifest evidence, numerical or
performance claim, or conformance promotion. Counts become 32 algorithms
and 261 routines, split 58 `state_mapped` plus 203 `source_mapped`, with 138
required; heat-balance/HVAC lists become 88/27 and HVAC readiness remains
`0/27`. The parent stays `scaffold` with claim level `none`.

CP257 next maps
`ZoneEquipmentManager::updateZoneSizingEndZoneSizingCalc5`, declared at
`ZoneEquipmentManager.hh` line 171 and implemented completely at
`ZoneEquipmentManager.cc` lines 2801-2842. Calc6 begins at line 2844.
## CP257 `updateZoneSizingEndZoneSizingCalc5` Final User-Array Projection

CP257 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc5` immediately after Calc4
and before `sim_zone_equipment`. It is declared at
`ZoneEquipmentManager.hh` line 171 and implemented completely at
`ZoneEquipmentManager.cc` lines 2801-2842:

```cpp
void updateZoneSizingEndZoneSizingCalc5(
    DataSizing::ZoneSizingData &zsFinalSizing,
    DataSizing::ZoneSizingData const &zsCalcFinalSizing);
```

The EndZone parent starts CP257 only after the complete CP256 daily-array
sweep. Like Calc4, the Calc5 loop at lines 3468-3475 is outside the
nonpulse guard and executes on pulse and normal passes. For each zero-based
linear `i` in `FinalZoneSizing.size()`, it first copies
`CalcFinalZoneSizing[i] -> FinalZoneSizing[i]`. When Space sizing is
enabled, it then copies every linear
`CalcFinalSpaceSizing[j] -> FinalSpaceSizing[j]` for that outer `i`.
Calc6 cannot start until this entire sweep completes.

Let `F = FinalZoneSizing.size()`, `G = FinalSpaceSizing.size()`, and `I`
be one when Space sizing is enabled. The parent dispatches

`L = F * (1 + I*G)`

leaves. It targets all `F` final Zone records once and, only when `F > 0`
and `I = 1`, all `G` final Space records `F` times each. A zero Zone target
suppresses Space copying; a false Space flag leaves Space targets untouched.
Normal setup allocates `F = N` and `G = P` for `N` Zones and `P` Spaces,
giving `N + I*N*P` leaves. Every Zone is followed by the complete Space
array. There is no controlled flag, latent method, owner, membership, day,
name, or identity gate.

Both loops are bounded only by their mutable destination `EPVector`. A
longer calculated source tail is ignored, while equal sizes pair solely by
flat index. In asserted builds a shorter source reaches the debug
`vector::at` path and throws before leaf entry; release subscripting is
unchecked. Calc5 owns no extent, allocation, identity, or overlap check.

Each valid leaf is branchless and performs 35 assignments in source order:
two strings, four integers, and 29 `Real64` values. It has 35 unique
destinations, 31 unique right-hand-side names, and 70 member accesses.
Thirty-one assignments copy the same member name. Four interleaved fan-outs
instead read ordinary design values:

- `NonAirSysDesHeatLoad <- DesHeatLoad`;
- `NonAirSysDesHeatVolFlow <- DesHeatVolFlow`;
- `NonAirSysDesCoolLoad <- DesCoolLoad`;
- `NonAirSysDesCoolVolFlow <- DesCoolVolFlow`.

Calc5 contains every Calc4 destination plus six additional destinations:
the two same-name latent design loads and four `NonAirSys` fields. The
source comment says Calc5 differs by two extra fields; executable
destination/statement count is six, while only the two latent loads are new
unique right-hand-side names. Source NonAir fields are never read. The two
NonAir volume lines are themselves marked `Suspicious` by source TODOs.

The projection copies `CoolDesHumRat` but still omits `HeatDesHumRat`. It
also copies no sequence, sizing-type label, design-day date string, peak
timestamp, latent flow, method enum, thermostat field, or shared latent
flag. There is no predicate, arithmetic, unit conversion, clamp,
finite/range check, child call, state argument, or diagnostic. Negative,
nonfinite, and invalid day/timestep integers are copied as values.

CP255 can write 32 unique calculated-final destinations. Calc5 directly
carries 23 of them: 12 of 16 cooling fields and 11 of 16 heating fields.
It omits cooling sizing type/date/flow sequence/peak string and the
corresponding four heating fields plus `HeatDesHumRat`. Its other 12
destinations are six density/outdoor conditions, two latent loads, and four
NonAir fields. Because those four fan-outs read CP255-selected ordinary
load/volume, selected cooling can influence 14 final destinations, selected
heating 13, and both together 27; the direct destination intersection
remains 23. Final `HeatDesHumRat`
retains prior/input state despite a CP255 calculated-final replacement.

One parent pass executes `35L` assignment statements: `2L` string and
`33L` scalar assignments. Runtime is
`Theta(L + copied design-day-name bytes)` with constant local state.
Redundant Space sweeps converge to the same values only for stable source
records; in normal topology they repeat each final Space once per final
Zone.

There is no local status, catch, transaction, cleanup, or rollback. The two
possibly allocation-bearing string copies execute first. A defined failure
on the second preserves the completed `CoolDesDay`; no scalar has yet been
written. The remaining 33 scalar assignments do not throw for valid live
records. A defined Space-leaf failure preserves the completed current Zone,
earlier Zones, and prior/repeated Spaces, then suppresses later leaves,
Calc6, Calc7, facility sizing, and the run-done latch.

A later successful retry from a stable distinct source repairs the subset
and completed replay is value-idempotent. Exact source/destination alias is
not a no-op: 31 same-name self-copies are interleaved with four deterministic
NonAir-from-ordinary overwrites. No fan-out target is later read as a source,
so completed alias replay converges to that projection.

Whole-parent replay reruns EMS. At retry entry, the calculated-final source
is fully retained, while completed Calc5 Zone leaves remain in the
user-final destination. EMS can observe the registered subset of that prefix: eight Final-Zone
scalar inputs per controlled Zone, which can feed six calculated-final
actuators; a defined current-leaf string failure contributes no new scalar.
A normal retry then replays any gated CP252 work, CP253, CP254, any gated
CP255 work, and CP256 before Calc5. Pulse skips CP252-255 but retains the
same EMS feedback path, so neither retry is universally a pure
retained-source copy. Within one attempt Calc5 cannot retroactively alter
closed CP254 artifacts; whole retry rebuilds them earlier and bytes can
differ.

No C++ test calls Calc5 directly. The completing corpus has 59 EndZone
parents: 51 normal, six additional pulse entries, and two direct pulse
parents. It executes 118 leaves: 84 normal Zone, 21 normal Space, 11 pulse
Zone, and two direct Zone. This is 4,130 assignment statements: 236 string,
472 integer, and 3,422 `Real64`. There are 107 distinct test-local targets;
96 execute once and 11 pulse/normal Zone targets execute twice. All seven
Space-enabled contexts have `(F,G)=(1,3)`, so the source's repeated-Space
case for `F > 1` has zero execution coverage.

The leaves span 104 controlled contexts and 14 uncontrolled Zones. Thirteen
have latent sizing enabled: nine `Sensible`, four
`SensibleAndLatent`, and zero exact `Latent`; the other 105 are latent-off.
Calc5 ignores all those states and performs the same 35 assignments.

Across 803 static post-call assertion sites plus one invocation-site
`EXPECT_NO_THROW`, eight direct Final target reads cover six destination
names, but none compares a Calc5 source with its target or isolates copy
order. Four retain positive Calc5-dependent values, two are default zero,
and two peak temperatures are demonstrably overwritten by Calc7.
`FinalSpaceSizing` has no direct assertion.

A bounded set of 300 report descendants uses Final design-flow sign as a gate; 100
positive user-flow/design-day cells render target values. They still run
after Calc7 and are composite evidence. Calc7 accesses 32 of Calc5's 35
destination names: two densities read-only and 30 write-capable. Only the
two latent loads and `CoolDesHumRat` are untouched. Calc6 copies 14 sequence
families and overlaps none of the 35 scalar/string destinations.

The Rezero implementation overlaps 30 of the 35 names, but its focused test
actively asserts only 28 daily-record fields through 56 static and 4,200
dynamic checks. It never invokes Calc5. Its ten final records quick-return
because member arrays are unallocated, and no final or Space destination is
asserted. Direct copy identity, latent-load retention, all four NonAir
fan-outs, omitted heating humidity ratio, repeated Space topology, malformed
extent, alias, raw IEEE/index state, string failure, partial parent state,
and replay remain unisolated.

The Rust/data audit covers 721 UTF-8-readable current-worktree files
returned by `rg --files crates data`. It finds no
Calc5 key/helper, calculated-final or user-final Zone/Space sizing arena,
any of the 35 exact destination names, any of the 31 right-hand-side names,
or any of their 35 mechanical snake-case forms. Rust does have adjacent
current-step `ZoneSysEnergyDemand`, operational IdealLoads sensible/latent
rates and supply conditions, typed limits, density/OA/node/report state;
none is this final sizing projection, latent design-load retention, or the
four NonAir fan-outs.

All 61 active `SimulationControl` objects disable Zone sizing. Active data
contain five raw design days but no `Sizing:Zone`, `Sizing:Parameters`,
authored `Space`, or `SpaceList`, and no corresponding epJSON keys. Sizing
and authored-Space fixtures remain run-blocked.

CP257 adds no Rust target/state, support declaration, test, capability,
output implementation, comparator, case, manifest evidence, numerical or
performance claim, or conformance promotion. Counts become 32 algorithms
and 262 routines, split 58 `state_mapped` plus 204 `source_mapped`, with 139
required; heat-balance/HVAC lists become 88/28 and HVAC readiness remains
`0/28`. The parent stays `scaffold` with claim level `none`.

CP258 next maps
`ZoneEquipmentManager::updateZoneSizingEndZoneSizingCalc6`, declared at
`ZoneEquipmentManager.hh` lines 173-175 and implemented completely at
`ZoneEquipmentManager.cc` lines 2844-2865. Calc7 begins at line 2867.

## CP258 `updateZoneSizingEndZoneSizingCalc6` Daily and Final User-Sequence Projection

CP258 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc6` immediately after Calc5
and before `sim_zone_equipment`. It is declared at
`ZoneEquipmentManager.hh` lines 173-175 and implemented completely at
`ZoneEquipmentManager.cc` lines 2844-2865:

```cpp
void updateZoneSizingEndZoneSizingCalc6(
    DataSizing::ZoneSizingData &zsSizing,
    DataSizing::ZoneSizingData const &zsCalcSizing,
    int const numTimeStepsInDay);
```

The EndZone parent starts CP258 only after the complete CP256 daily and
CP257 final scalar/string sweeps. The four call sites at lines 3482, 3487,
3500, and 3505 are outside the nonpulse guard, so Calc6 executes on pulse
and normal passes. It first completes every daily record before touching a
final record:

1. design/run-design day ascending;
2. controlled Zone number ascending;
3. the Zone leaf, then that Zone's stored `spaceIndexes` in list order;
4. after all days, controlled final Zone ascending, followed by its stored
   Spaces.

Let `E = max(TotDesDays + TotRunDesPersDays, 0)`, `C` be the controlled
Zone count, `M` the number of stored Space-index occurrences under those
Zones, and `I` be one when Space sizing is enabled. The parent dispatches

`K = (E + 1) * (C + I*M)`

leaves. With `U` unique valid referenced Spaces, it addresses
`(E + 1)*(C + I*U)` distinct records and repeats
`(E + 1)*I*(M-U)` leaves. Duplicate or cross-list memberships count.
There is no per-Space control, owner, deduplication, latent, name, or
identity gate.

This is not a dense counterpart of Calc4/5. Uncontrolled Zone records and
unreferenced/orphan Space records can receive CP256/257 scalar fields but
retain prior sequence state. The source comment saying Calc6 is called for
all Zone/Space daily and final arrays therefore overstates literal parent
coverage. The daily loop is driven by global day and Zone counts plus
membership identities, while the final loop is driven by controlled Zone
numbers, not destination extents. Array shapes, indexes, and corresponding
source identities are assumed rather than compared.

For `Q = max(numTimeStepsInDay, 0)`, each valid leaf executes 14
same-name, same-index `Real64` copies at every one-based timestep, in exact
source order:

- `HeatFlowSeq`, `HeatLoadSeq`, `CoolFlowSeq`, and `CoolLoadSeq`;
- heating `HeatZoneTempSeq`, `HeatOutTempSeq`, `HeatZoneRetTempSeq`,
  `HeatZoneHumRatSeq`, and `HeatOutHumRatSeq`;
- cooling `CoolZoneTempSeq`, `CoolOutTempSeq`, `CoolZoneRetTempSeq`,
  `CoolZoneHumRatSeq`, and `CoolOutHumRatSeq`.

The leaf therefore has 14 assignment sites, 14 unique destination names,
14 unique right-hand-side names, and 28 member accesses per timestep.
Twelve containers are `Array1D<Real64>`; `HeatZoneTempSeq` and
`CoolZoneTempSeq` are `EPVector<Real64>`. Normal setup dimensions all of
them to `TimeStepsInHour*24`. Values are copied without arithmetic, unit
conversion, clamping, finite/range checks, allocation, state access, child
calls, or diagnostics.

`ZoneSizingData::allocateMemberArrays` dimensions 36 sequence families.
Calc6 copies only these 14. It omits both no-OA flow sequences, both design
setpoint and thermostat sequences, eight DOAS sequences, both no-DOAS load
sequences, both latent load sequences, both latent no-DOAS load sequences,
and both latent flow sequences.

CP255 intersects Calc6 only at `HeatFlowSeq` and `CoolFlowSeq`. A selected
latent branch substitutes a latent flow sequence into calculated-final and,
for a positive selected day, calculated-daily ordinary flow. Calc6 carries
those two ordinary-name sequences but still copies ordinary
`HeatLoadSeq`/`CoolLoadSeq`; it can therefore project a selected latent-flow
plus ordinary-load hybrid. The latent sequence families themselves remain
untouched. Calc4 and Calc5 have zero member-name overlap.

Calc7 waits for both Calc6 sweeps and accesses 12 of the 14 names. It can
rewrite the four flow/load sequences in final and selected daily records,
and it reads eight daily Zone/outdoor temperature and humidity sequences in
zero-load fallback paths. Only the two Zone return-temperature sequences
have no Calc7 access. Any later evidence is therefore composite except for
an isolated pre-Calc7 copy check.

`Q = 0` is a silent no-op. A smaller loop bound copies a prefix and leaves
both source-independent destination tails untouched. A larger bound or
malformed record shape eventually indexes outside one of 28 sequence
containers. The 12 Objexx arrays assert containment in asserted builds and
are unchecked in release. The two `EPVector` destinations/sources use
debug `vector::at` and release unchecked subscripting. A recoverable debug
exception at the fifth assignment leaves assignments 1-4 for the current
timestep; one at the tenth leaves assignments 1-9. All prior timesteps and
leaves remain committed.

There is no local status, catch, transaction, cleanup, or rollback. A
record-access failure can occur before leaf entry; a leaf failure suppresses
later Calc6 leaves, Calc7, facility sizing, and the run-done latch. Valid
scalar copies allocate nothing and do not throw. Stable distinct-source
completion and replay are value-idempotent. Exact record alias is 14
same-member self-copies per timestep: a value no-op that still performs
indexing and work. Stable duplicate Space calls converge but repeat work.

Calc6 owns no EMS call, and none of its sequence fields is one of the eight
registered Final-Zone scalar inputs or six calculated-final actuators. A
whole-parent retry can expose the already completed Calc5 scalar prefix to
EMS, but not a partial Calc6 sequence prefix directly. A normal retry can
then rebuild CP252 state and repeat CP253, CP254, CP255, Calc4, and Calc5;
that work can change the two selected calculated flow-sequence sources.
Pulse skips CP252-255 and copies retained sequence sources after EMS and
Calc4/5.

The CP254 writer has already closed before Calc6, so current-attempt Calc6
cannot alter ZSZ/SPSZ bytes. On a nonpulse whole retry, however, CP254 runs
before CP255. A prior retained CP255 flow-sequence selection can therefore
be visible to the rebuilt artifact even though Calc6 itself writes only
user arrays.

The completing C++ corpus has the same 59 parent entries: 51 normal, six
additional pulse, and two direct-parent pulse calls. It executes 301 Calc6
leaves:

- 197 daily: 155 Zone and 42 Space;
- 104 final: 83 Zone and 21 Space;
- equivalently, 270 normal, 27 pulse, and four direct-parent leaves.

The day-count histogram is `E=1:12, 2:46, 3:1`; the timestep histogram is
`Q=144:29, 96:26, 24:2, 1:2`. Together they execute 35,716 timestep-loop
iterations, exactly 500,024 `Real64` assignments and 1,000,048 member
accesses. There are 274 distinct test-local records: 247 execute once and
27 normal targets are revisited by pulse. All 301 are controlled Zone or
controlled-owner Space contexts. The seven Space contexts each use one
controlled Zone with three unique Spaces, so duplicate/cross-list Space
coverage is zero.

Thirty-nine leaves have latent sizing enabled: 27 `Sensible` and 12
`SensibleAndLatent`; exact `Latent` is absent. The other 262 are latent-off.
Calc6 reads none of those method/flag fields and performs the same loop.

No C++ test calls Calc6 directly, compares a source sequence with its
destination, or asserts a copied sequence element. The two direct parent
tests execute four leaves and 56 assignments at `E=C=Q=1` but assert no
sequence. Exactly four genuine downstream scalar sites cover only
`HeatZoneTempSeq` and `CoolZoneTempSeq`; Calc7 reads those daily user
destinations to derive calculated-final peak temperatures. No final-record
copy has downstream proof.

The Rezero implementation resets all 14 names, but its focused test checks
only daily Zone and calculated-daily arrays: 28 static sites and 8,400
dynamic checks over 15 days, five Zones, and four timesteps. It never calls
Calc6 and is a reset oracle, not a copy oracle. Ten final records
quick-return because their sentinel array is unallocated; final and Space
sequence resets are unasserted.

Calc6 has zero attributable report assertion. CP254 writes before Calc6
from calculated arrays. The component-load table path also reads
calculated-final sources directly, bypassing copied user destinations.
Direct identity/order for all 14 fields, 12 fields beyond the two
temperature descendants, both return-temperature sequences, final copying,
nonpositive/short/long bounds, malformed shapes, alias, partial failure,
replay, and duplicate Space topology remain unisolated.

The Rust/data audit covers 721 UTF-8-readable current-worktree files
returned by `rg --files crates data`. It finds no Calc6 key/helper, daily or
final Zone/Space sizing sequence arena, any of the 14 exact member names,
or any of their 14 mechanical snake-case forms. Adjacent current-step
`ZoneSysEnergyDemand`, operational IdealLoads state, typed limits,
density/OA/node/report fields, and design-day labels are not this
calculated-to-user sequence projection.

All 61 active `SimulationControl` objects disable Zone sizing. Active data
contain five raw design days but no `Sizing:Zone`, `Sizing:Parameters`,
authored `Space`, or `SpaceList`, and no corresponding epJSON keys. Sizing
and authored-Space fixtures remain run-blocked.

CP258 adds no Rust target/state, support declaration, test, capability,
output implementation, comparator, case, manifest evidence, numerical or
performance claim, or conformance promotion. Counts become 32 algorithms
and 263 routines, split 58 `state_mapped` plus 205 `source_mapped`, with 140
required; heat-balance/HVAC lists become 88/29 and HVAC readiness remains
`0/29`. The parent stays `scaffold` with claim level `none`.

CP259 next maps
`ZoneEquipmentManager::updateZoneSizingEndZoneSizingCalc7`, declared at
`ZoneEquipmentManager.hh` lines 177-182 and implemented completely at
`ZoneEquipmentManager.cc` lines 2867-3221. `UpdateZoneSizing` begins at
line 3223.
## CP259 `updateZoneSizingEndZoneSizingCalc7` Final Sizing Adjustment

CP259 adds canonical required
`routine.update_zone_sizing_end_zone_sizing_calc7` immediately after Calc6
and before `sim_zone_equipment`. It is declared at
`ZoneEquipmentManager.hh` lines 177-182 and implemented completely at
`ZoneEquipmentManager.cc` lines 2867-3221:

```cpp
void updateZoneSizingEndZoneSizingCalc7(
    EnergyPlusData &state,
    DataSizing::ZoneSizingData &zsFinalSizing,
    DataSizing::ZoneSizingData &zsCalcFinalSizing,
    Array2D<DataSizing::ZoneSizingData> &zsSizing,
    Array2D<DataSizing::ZoneSizingData> &zsCalcSizing,
    int const zoneOrSpaceNum);
```

The EndZone parent calls Calc7 only after both Calc6 daily/final sweeps
complete. Lines 3511-3531 are outside the nonpulse guard, so normal and
pulse entries both visit each controlled Zone in ascending order, complete
its leaf, and then visit that Zone's stored `spaceIndexes` in list order
when Space sizing is active.

For controlled count `C`, stored Space occurrence count `M`, Space flag
`I`, and unique valid referenced Space count `U`, the parent dispatches
`L=C+I*M` leaves over `C+I*U` distinct targets. Duplicate or cross-listed
Space identities repeat the complete mutating leaf. There is no per-Space
control, owner, identity, latent, method, or deduplication gate.
Uncontrolled Zones and unreferenced Spaces receive Calc4-5 copies and may
retain prior Calc6 state, but receive no Calc7 adjustment.

The mutable user-final record, mutable calculated-final record, mutable
user-daily array, syntactically mutable but body-read-only calculated-daily
array, and raw Zone/Space index are trusted to correspond. The body touches
76 unique `ZoneSizingData` member names and has 112 record-assignment sites:
61 to user-final, 41 to daily user records, and ten to calculated-final.
It writes 44 unique user-final names, 20 unique daily-user names, and ten
unique calculated-final names. State supplies the design-day count,
`NumOfTimeStepInDay`, design-day weather, and diagnostics.

Cooling executes first. It unconditionally multiplies both cooling
`NonAirSys` load and volume flow by `CoolSizingFactor`. The total cooling
multiplier is

`(InpDesCoolAirFlow / current DesCoolVolFlow) * CoolSizingFactor`

only for positive input flow, exact `InpDesAirFlow`, and positive current
user-final flow; otherwise it is the sizing factor. Only an absolute
difference from one greater than `1e-5` enters rescaling. A positive
current flow rebuilds final and each positive daily volume, mass, load,
flow sequence, and load sequence from the calculated record times the
multiplier, then recomputes OA-mixed coil inlet state. A nonpositive
current flow instead writes the input volume and density-derived mass.
Every daily record then snapshots its no-OA cooling flow state.

The positive-flow guard checks the old user flow, but OA fraction divides
by the newly assigned calculated flow times multiplier without rechecking
it. A zero or nonfinite replacement can therefore produce nonfinite OA
state before the source-order `[0,1]` clamp.

Final cooling no-OA minimums honor exact `DesAirFlowWithLim` and take the
maximum of both authored minima. The daily implementation instead
literally evaluates `max(DesCoolMinAirFlow, DesCoolMinAirFlow)` with no
method guard, omitting `DesCoolMinAirFlow2`. The including-OA daily
calculation repeats that duplicated first operand and adds `MinOA`, again
without the final record's method branch. Scalar and flow-sequence floors
do not recompute load sequences or coil inlet state.

Despite comments describing zero cooling flow, the fallback gate is exact
`DesCoolLoad == 0`. It changes calculated-final day and timestep only when
each equals zero, copies those identities to user-final, and indexes the
selected daily user record. An empty cooling setpoint sequence emits Severe
then Fatal. Otherwise user-final Zone peak temperature is the whole
setpoint-sequence minimum, outdoor temperature is the unguarded whole
outdoor-sequence minimum, outdoor humidity uses the selected timestep,
and user-final Zone humidity uses the daily design scalar. Calculated-final
Zone temperature/humidity use the daily Zone sequences at that timestep.
Each record's return temperature is set to its own Zone temperature, while
the user coil inlet is set to the user Zone peak state.

Heating then repeats the two unconditional `NonAirSys` multiplications.
Its positive-input multiplier has the same factor-multiplied formula. Exact
`DesAirFlowWithLim` instead computes a maximum from both heating limits and
the already adjusted cooling volume times the heating fraction; only a
strictly lower maximum produces a ratio, which is still multiplied by the
heating sizing factor despite the source comment saying the input
overrides it. Scaling, daily no-OA snapshots, and coil mixing otherwise
parallel cooling.

Heating saves final no-OA scalars and sequence before applying `MinOA`; it
does not apply a separate user maximum to that snapshot. Final and daily
ordinary heating scalar/sequence flows are then floored only by `MinOA`.
The exact-zero gate is again load rather than the commented flow. It
defaults exact-zero calculated day/time identities, uses a guarded
whole-setpoint maximum, but still uses an unguarded whole outdoor-
temperature minimum, and preserves the same user/calculated peak-state
split as cooling.

The tail always derives `DesCoolVolFlowMin` from both cooling minima plus
the current cooling-flow fraction and derives `DesHeatVolFlowMax` from
both heating maxima plus the fraction of the larger current cooling or
heating flow. Exact integer `TemperatureDifference` methods overwrite
cooling supply temperature with Zone peak minus the absolute difference
and heating supply temperature with Zone peak plus the absolute
difference. The source cooling-minimum comment itself notes that its
description is incorrect.

The body has 18 explicit loops and 34 `if` sites. Each leaf makes five
complete `1..D` day sweeps, four unconditional final `1..T` loops, and
three unconditional daily `1..T` loops per day. Conditional rescaling
loops are zero-based and bounded only by destination `FlowSeq.size()`
while indexing the matching destination load and both calculated
sequences. Whole-array daily no-OA copies use their own shapes.
`T <= 0` therefore skips only seven explicit timestep loops; scalar work,
size-driven scaling, whole-array copies, zero-load reductions, and the tail
still execute. `D <= 0` skips day sweeps, but a zero-load default can still
force day one and index absent state.

Only empty setpoint sequences have explicit diagnostics. Weather day/time,
daily Array2D coordinates, selected sequences, matching sequence extents,
and unguarded outdoor-temperature reductions are unchecked. There is no
status, catch, transaction, or rollback. Cooling failure preserves its
ordered prefix and blocks all heating/tail work; heating failure preserves
all cooling and its own prefix. Earlier parent leaves survive, and failure
blocks later leaves and sizing completion.

Direct leaf replay is not generally idempotent: four `NonAirSys` fields
multiply in place, the input-flow multiplier denominator is mutable, and
the zero-load path mutates calculated-final state. Duplicate Space
membership can therefore compound within one parent entry. Exact
user/calculated alias turns source-to-target scaling into in-place scaling
and also collapses the zero-load user/calculated peak split. Whole-parent
replay reruns EMS and Calc4-6, but retained calculated-final defaults and
earlier Calc3 selection can still alter a retry.

On a normal nonpulse entry, the current-attempt CP254 ZSZ/SPSZ writer has
already closed before Calc7. A later nonpulse whole-parent replay can
nevertheless let that writer observe Calc7-defaulted calculated-final day
identities and retained Calc3 sequence state. Pulse entries skip Calc1-3
and the writer but repeat EMS and Calc4-7.

The completing C++ corpus has 59 EndZone parent entries and executes 104
Calc7 leaves: 93 normal, nine additional pulse, and two bare pulse leaves;
83 are Zone and 21 Space calls. It addresses 95 distinct final targets,
with 86 executed once and nine normal targets revisited by pulse. The
final-leaf timestep histogram is `Q=144:55, 96:45, 24:2, 1:2`, totaling
12,290 final-leaf timestep units. Across 197 aggregate daily-record visits,
the seven unconditional timestep-loop families execute 119,438 iterations;
five unconditional day sweeps make 985 record visits before conditional
sequence loops and whole-array copies.

No test calls Calc7 directly. Eleven normal Zone leaves have nonunit
cooling and heating factors and enter both multiplier gates; explicit
input-flow override has zero coverage. Cooling methods are 95
`FromDDCalc`, seven `DesAirFlowWithLim`, and two default/invalid bare
records; heating has 102 `FromDDCalc` plus two default/invalid records.
Neither supply-air `TemperatureDifference` tail branch executes. The two
bare parent tests prove both zero-load paths, avoid both fatals, and assert
the calculated/user peak-temperature split, but direct formula, daily
minimum, OA mixing, sequence, failure, alias, duplicate membership, and
retry behavior remain unisolated.

Rust has no Calc7 key/helper, final or daily Zone/Space sizing arena,
airflow-sizing method state, or counterpart for this 76-member boundary.
Generic `zone_name`, density, outdoor-air, design-day, timestep demand, and
operational IdealLoads fields are not final sizing adjustment state. All
61 active `SimulationControl` objects disable Zone sizing. Active data
contain five raw design days but no `Sizing:Zone`, `Sizing:Parameters`,
authored `Space`, or `SpaceList`, and no corresponding epJSON keys. Sizing
and authored-Space fixtures remain run-blocked.

CP259 adds no Rust target/state, support declaration, test, capability,
output implementation, comparator, case, manifest evidence, numerical or
performance claim, or conformance promotion. Counts become 32 algorithms
and 264 routines, split 58 `state_mapped` plus 206 `source_mapped`, with 141
required; heat-balance/HVAC lists become 88/30 and HVAC readiness remains
`0/30`. The parent stays `scaffold` with claim level `none`.

CP260 next maps the complete parent
`ZoneEquipmentManager::UpdateZoneSizing`, declared at
`ZoneEquipmentManager.hh` line 130 and implemented at
`ZoneEquipmentManager.cc` lines 3223-3536. `SimZoneEquipment` begins at
line 3538.

## CP260 `UpdateZoneSizing` Four-Phase Sizing Dispatcher

CP260 adds canonical required `routine.update_zone_sizing` immediately
after Calc7 and before the existing `sim_zone_equipment` row. It is
declared at `ZoneEquipmentManager.hh` line 130 and implemented completely
at `ZoneEquipmentManager.cc` lines 3223-3536:

```cpp
void UpdateZoneSizing(
    EnergyPlusData &state,
    Constant::CallIndicator const CallIndicator);
```

The header omits the definition's top-level `const` on the by-value
indicator; that does not change the C++ function type. The authoritative
`CallIndicator` values are `Invalid=-1`, `BeginDay=0`,
`DuringDay=1`, `EndDay=2`, `EndZoneSizingCalc=3`,
`EndSysSizingCalc=4`, and `Num=5`. The parent comments label the four
handled stages as 1 through 4, so they are off by one. The switch has
explicit BeginDay, DuringDay, EndDay, and EndZoneSizingCalc cases.
`Invalid`, `EndSysSizingCalc`, `Num`, and arbitrary cast values reach the
default and silently do nothing. The BeginDay comment also promises to
zero result arrays, but its child only stamps calculated metadata and
clears no sequence arrays.

Production gates live outside this routine. SizingManager calls BeginDay
on non-warmup sizing days, EndDay at a non-warmup day end, and EndZone only
after at least one sizing period. HVACManager calls DuringDay under its
non-warmup Zone-sizing gate for each accepted system substep. Direct
callers can bypass every lifecycle condition.

Let `Z=max(NumOfZones,0)`, `C` be the controlled Zone count, `I` be one
when Space sizing is active, `M` be stored Space-index occurrences under
controlled Zones, `U` be their unique valid identities, and
`H=C+I*M`. BeginDay visits controlled Zones ascending, runs the Zone child,
then stored Spaces in list order, dispatching `H` metadata seeds over
`C+I*U` distinct records.

DuringDay computes signed integer

`(HourOfDay-1)*TimeStepsInHour+TimeStep`

and snapshots `FracTimeStepZone`, then uses the same `H` traversal. Space
records receive the referring Zone's thermostat values and references to
that Zone's final thermostat extrema, never FinalSpace extrema.
Duplicate/cross-listed Space identities can therefore receive repeated
additions and different parent parameters.

EndDay completes the entire `H` moving-average sweep before starting the
entire `H` daily-peak/final reduction sweep. It dispatches `2H` leaves and
forms a real barrier: failure during smoothing prevents every reducer;
failure during reduction retains all completed smoothing.

EndZone order is exact:

1. call Zone-sizing `ManageEMS` unconditionally and ignore `anyEMSRan`;
2. when any EMS exists, scan all `Z` calculated-final Zone records without
   a controlled-Zone gate and conditionally apply six overrides;
3. only when nonpulse, optionally run noncoincident Calc1, then Calc2,
   ZSZ and optional SPSZ routing/writers, then latent-selection Calc3;
4. on both pulse and normal entries, complete dense Calc4, dense Calc5,
   daily then final Calc6, and final Calc7 barriers.

The six inline overrides are independent and ordered heating mass, cooling
mass, heating load, cooling load, heating volume, and cooling volume. Each
requires its flag and the current target to be strictly positive, then
copies the raw EMS value without finite, sign, or consistency validation.
The scan includes uncontrolled Zones and never overrides a Space. A
nonpositive or NaN EMS result can make the same override fail its current-
target gate on replay. Noncoincident Calc1 can later rebuild the six Zone
fields from Spaces.

The parent directly persists only those six calculated-final fields plus
two output paths. Comma selects CSV, tab selects TAB, and every other
separator selects TXT. ZSZ is routed, opened, written, and closed before
optional SPSZ. Current files contain post-EMS/Calc2 but pre-Calc3/Calc4-7
calculated state. The writer uses dense Zone/global-Space traversal and
filters the Space owner with HeatBalance `Zone.IsControlled`, not this
parent's equipment-control or membership topology. A successful replay
reopens a closed not-good handle and truncates/rebuilds it; failure before
close can leave a partial good stream that a retry may reuse and append.

For EndZone cardinality, let `N` be controlled Zones whose declared
`numSpaces` is not exactly one; `L` be controlled Zones whose
calculated-final latent flag is true; `M_L` be their stored Space
occurrences; `K=L+I*M_L`; `A/B` be flat Zone/Space daily target sizes;
`F/G` be flat Zone/Space final target sizes; and
`D=max(TotDesDays+TotRunDesPersDays,0)`.

A normal entry dispatches Calc1 `I*N`, Calc2 `H`, writers `1+I`, Calc3
`K`, Calc4 `A*(1+I*B)`, Calc5 `F*(1+I*G)`, Calc6 `(D+1)*H`, and Calc7
`H`, plus one EMS call. Its mapped-child total is

`2+I+I*N+K+A*(1+I*B)+F*(1+I*G)+(D+3)*H`,

with a further `1+I` `ensure_open` service calls. Pulse skips Calc1/2,
both writers, Calc3, and every file open, but still runs EMS and Calc4-7;
its mapped-child total is

`1+A*(1+I*B)+F*(1+I*G)+(D+2)*H`.

Normal allocation makes Calc4
`D*Z + I*D^2*Z*S` and Calc5 `Z + I*Z*S`, where `S` is the global Space
count. Those literal Cartesian Space re-copies have no control, owner,
membership, or paired-day filter; Calc6/7 return to controlled stored
membership.
The source body has 25 `for`, 43 `if`, four explicit cases plus default,
12 `continue`, and 26 mapped child-call sites, including the one
`ManageEMS` and two writer sites. It adds two `ensure_open` sites. Child
routines own the numerical work already mapped in CP248-259; the parent
owns their cross-case gates, barriers, topology changes, and side effects.

There is no local validation, status, catch, checkpoint, cleanup, or
rollback. Any lookup, child, allocation, diagnostic fatal, or output-open/
write failure retains all earlier child and inline effects and suppresses
the remaining barriers. EndZone failure prevents Facility EndZone,
`ZoneSizingRunDone=true`, and, on a pulse pass, the later Rezero call in
SizingManager.

Replay semantics depend on the selected case. BeginDay re-stamps metadata
without zeroing sequences; DuringDay repeats weighted additions; EndDay
repeats smoothing and peak selection; EndZone repeats EMS callbacks,
diagnostics, writers, dense copies, latent selection, and factor/floor
adjustment. Duplicate Space
memberships add further re-entry. No whole-parent idempotence or repair
boundary exists.

The completing C++ census has 107 BeginDay and 107 EndDay parent calls,
including two direct calls of each. Their child counts are 197 BeginDay
leaves and 197 leaves in each EndDay sweep. DuringDay has a nominal
one-accepted-system-substep floor of 12,290 parent calls and 23,426 child
leaves; adaptive downsteps can increase both. EndZone has 59 calls: 51
nonpulse and eight pulse, including the two direct pulse calls.

Across EndZone, the mapped child matrix is: `ManageEMS` 59, Calc1 7,
Calc2 93, writers 58, Calc3 13, Calc4 273, Calc5 118, Calc6 301, and Calc7
104. Calc4 reaches 28 uncontrolled daily Zone records and Calc5 reaches 14
uncontrolled final Zone records. All 58 writers take the default comma
route; tab and text have zero completion coverage. Each of the two direct
tests calls every handled indicator once, for eight expressions total, but
uses one controlled Zone, no Space or EMS, and pulse EndZone.

There is no test of default/EndSys/invalid dispatch, an EMS override,
tab/text routing, output failure, mixed control, duplicate/cross-listed
membership, malformed indexes/extents, parent failure prefix, or replay.
Direct assertions are downstream composites of child stages rather than
oracles for the complete switch and barrier structure.

The Rust/data audit covers the same 721 strict-UTF-8 files. It finds no
`UpdateZoneSizing` key/helper, CallIndicator sizing dispatcher, handled
stage names, Zone-sizing run-done lifecycle, Zone/Space sizing arenas,
pulse/Space-sizing flags, calculated-final EMS override state, ZSZ/SPSZ
artifact, or Facility-sizing handoff. Adjacent run-period time, thermostat,
IdealLoads, equipment graph, generic output, and unsupported-EMS concepts
are not this design-sizing dispatcher.

All 61 active `SimulationControl` objects disable Zone sizing. Active data
contain five raw design days but no `Sizing:Zone`, `Sizing:Parameters`,
authored `Space`, or `SpaceList`, and no corresponding epJSON keys. The
sole raw Sizing:Zone test fixture expects `UnsupportedSizing`; sizing and
Space partitioning remain run-blocked.

CP260 adds no Rust target/state, support declaration, test, capability,
output implementation, comparator, case, manifest evidence, numerical or
performance claim, or conformance promotion. Counts become 32 algorithms
and 265 routines, split 58 `state_mapped` plus 207 `source_mapped`, with 142
required; heat-balance/HVAC lists become 88/31 and HVAC readiness remains
`0/31`. The parent stays `scaffold` with claim level `none`.

CP261 next expands the existing required `routine.sim_zone_equipment`
boundary rather than adding a duplicate row. `SimZoneEquipment` is declared
at `ZoneEquipmentManager.hh` line 184 and implemented completely at
`ZoneEquipmentManager.cc` lines 3538-4193. `SetZoneEquipSimOrder` begins at
line 4195.

## CP261 `SimZoneEquipment` Zone-Equipment Dispatcher

CP261 expands the already-existing required
`routine.sim_zone_equipment` source boundary in place. It adds neither a
duplicate routine nor another HVAC project-contract item. The declaration
is `ZoneEquipmentManager.hh` line 184 and the complete definition is
`ZoneEquipmentManager.cc` lines 3538-4193:

```cpp
void SimZoneEquipment(
    EnergyPlusData &state,
    bool const FirstHVACIteration,
    bool &SimAir);
```

The header omits the definition's top-level `const` on the by-value Boolean;
the C++ function type is unchanged. `state` is the mutable simulation graph.
`FirstHVACIteration` reaches airflow and equipment-child arguments, selects
capacity caching, and reaches the iteration-aware exhaust-system,
mass-balance, and leaving-condition tail children. `SimAir` is never read
or cleared here: only a completed reverse supply path reporting an inlet
change assigns it `true`, so a false result preserves the caller's incoming
value rather than writing false.

The exact normal source order is:

1. traverse every supply path and its components forward, simulating
   splitters unless both AirflowNetwork fan activation and distribution
   simulation are true, always simulating supply plenums, and fatally
   rejecting every other component type;
2. optionally calculate simple airflow for enforced Zone mass balance, then
   under active non-sizing SpaceHVAC set every equipment mixer's outlet
   conditions;
3. scan all Zones, skip uncontrolled configurations, reset Zone and optional
   Space response/exhaust state, and call `InitSystemOutputRequired` with
   `ResetSimOrder=true`;
4. traverse each controlled Zone's priority order, reset per-slot globals
   and outputs, perform the first-pass 18-field sizing reset, apply the
   availability-manager result, optionally adjust Space loads, dispatch the
   equipment type, reconcile exhaust/capacity/Space-or-Zone output, and
   update remaining sensible and moisture demand;
5. set SpaceHVAC mixer inlet flows, clear `CurZoneEqNum`, and clear the
   first-pass latch;
6. traverse each supply path again in reverse component order, optionally
   apply per-splitter supply duct loss, and monotonically set `SimAir`; then
7. run Zone HVAC exhaust controls, the exhaust-air system, Zone mass balance,
   Zone leaving conditions, whole-system duct loss, and the return-air path
   in that fixed order.

`FirstCall` is true for the complete forward pass and false for the complete
reverse pass; it is not a first-component marker. Forward
`SupPathInletChanged` is neither reset per path nor consumed. The reverse
pass resets it per path and is the only pass that can affect `SimAir`.
Supply-path defaults emit severe, continue, then fatal diagnostics, whereas
the equipment switch default is a silent no-op followed by the ordinary
zero-output reconciliation.

The equipment loop selects the real list entry through
`PrioritySimOrder(EquipTypeNum).EquipPtr`. Its switch has 33 explicit type
labels grouped into 27 simulator bodies: ADU, VRF, window AC, four shared
packaged/unitary types, DX dehumidifier, fan coil, unit ventilator, unit
heater, PurchasedAir, six water/steam/electric baseboard or cooling-panel
families, high- and three low-temperature radiant types, exhaust fan, heat
exchanger, ERV, two heat-pump water-heater types, ventilated slab, outdoor
air unit, refrigeration chiller set, user-defined forced air, evaporative
cooler, and hybrid evaporative cooler. PurchasedAir is only one branch.
Non-air families copy or retain their sensible output according to their
source branch; the dehumidifier instead accumulates sensible output into
`SysDepZoneLoads` and then zeros the common sensible result.

Every slot first clears fan commands, three exhaust/plenum globals, three
local outputs, and `DataCoolCoilCap`. While
`FirstPassZoneEquipFlag` remains true, every slot also clears the same 18
`ZoneEqSizing` fields; the latch is cleared only after the entire Zone
sweep. Availability eligibility tests only that the integer equipment type
is at most 14, with no local lower-bound test. The shared `ErrorFlag` is
initialized once, passed by reference, and never inspected or reset here.
`CycleOn` and `ForceOff` rewrite the fan command pair.

Common reconciliation adds three flow accumulators for every slot. On the
first HVAC iteration of a non-sequential list, positive sensible output
overwrites only that slot's heating capacity; zero or negative output
overwrites only cooling capacity, leaving the opposite cell intact. A
Space splitter receives output when enabled; otherwise non-air output is
added to the Zone heat-balance response. Every completed slot calls
`updateSystemOutputRequired` and resets `CurTermUnitSizingNum`.

Let `X/Y` be splitter/plenum components, `A` be one unless the two
AirflowNetwork suppression flags are both true, `B` indicate enforced mass
balance, `H` indicate active non-sizing SpaceHVAC, `M` be the mixer count,
`C` controlled Zones, `Q` equipment slots, `V` availability-eligible slots,
`T` Space-splitter slots, `R` explicitly handled slots, `G` lazy exhaust-fan
index lookups, and `D` the duct-loss flag. A successful call performs

`2*(A*X+Y)+B+2*H*M+C+V+2*T+R+G+Q+A*D*X+6`

principal child/service invocations, excluding diagnostics and nested child
work. The body has nine loops, 25 `if` tokens including one `else if`, three
switches, 37 case labels, three defaults, two continues, and no explicit
return. Its 48 operational call sites become 54 with six diagnostic calls
and 58 with four diagnostic formatting calls. It has 55 direct persistent
assignment sites over 41 normalized lvalue families including `SimAir`;
excluding that in/out flag gives 54 state-graph sites over 40 families.

There is no comprehensive up-front validation, local result status, catch,
cleanup guard, checkpoint, transaction, rollback, or replay repair. A forward or
equipment failure retains all completed child and direct-write prefixes.
Failure before the normal post-Zone cleanup can leave `CurZoneEqNum`
nonzero and the first-pass latch true. Failure after a child simulator but
before common reconciliation retains the child's state while skipping
exhaust accumulation, demand update, and the terminal-sizing reset. A
reverse failure occurs after the two normal cleanup writes but before the
fixed tail; a tail failure retains its earlier tail prefix.

Successful replay is deliberately iterative rather than idempotent:
supply components, availability, equipment, demand reconciliation, and
tail children run again; the first-pass sizing reset disappears once an
invocation reaches the post-Zone latch clear; an exhaust-fan index can stay
cached; capacity replay rewrites only one sign-selected cell per eligible
slot; and `SimAir` is monotonic within this routine.
`ManageZoneEquipment` sets `ZoneEquipSimulatedOnce`, calls
`UpdateZoneEquipment`, and clears its caller's `SimZone` only after this
routine returns.

The C++ test census finds four direct calls across two tests and nine
wrapper calls across eight tests, for 13 directly attributable successful
invocations: 11 first-iteration and two later-iteration. Seventeen further
direct `ManageSizing` contexts reach this routine once after their sizing
gate clears. Across those 30 statically attributable calls, 65 handled
equipment slots dispatch: 36 ADU, 18 PTAC, four PurchasedAir, three unitary
systems, two unit heaters, and two window ACs. They make 26 availability
calls, 34 splitter simulations, two plenum simulations, 65 demand updates,
and 30 executions of each fixed-tail child.

Those tests cover selected node/flow propagation, PTAC fan-cycle flags,
PurchasedAir outputs, and one ADU-before-unit-heater priority result. Every
list is sequential. None covers the non-sequential capacity cache,
SpaceHVAC mixers/splitters, enforced mass balance, a supply-path fatal, the
silent equipment default, `ForceOff`, a direct `SimAir=true` assertion,
failure prefixes, or retry semantics. Fifty-six full `ManageSimulation`
tests complete and exercise repeated HVAC passes, but warmup, environments,
timesteps, and convergence make an exact dynamic call count unavailable
without tracing; the one EMS-fatal test stops before this boundary.

Rust has only the three-label reporting metadata, two execution-plan labels
placed together in one `ZoneEquipmentManager` stage, and IdealLoads graph
validation/order tests. The plan's
`ExecutionStep::SimZoneEquipment(ZoneEquipmentListId)` is not interpreted
by the active pipeline; that pipeline calls the prebound PurchasedAir
compatibility runtime directly and does not execute this mutable parent
protocol. `ZoneEquipmentObjectType` supports only
IdealLoads, and the active typed lane has 30 equipment connections, 30
equipment lists, and 30 IdealLoads objects, each list containing one
`SequentialLoad` entry at cooling/heating sequence one. It has no active
supply-path, SpaceHVAC mixer/splitter, plenum, or Zone exhaust-fan topology.

No Rust state implements the priority array, first-pass latch, supply-path
change protocol, fan commands, Zone/Space response and exhaust accumulators,
capacity cache, mass-balance gate, 32 other equipment families, or fixed
six-call tail. Stage names and execution-plan ordering are descriptive
evidence, not execution of the C++ body.

CP261 changes no routine metadata, algorithm count, Rust target/state,
support declaration, test, capability, output, comparator, case, manifest,
numerical/performance claim, or conformance status. Counts remain 32
algorithms and 265 routines, split 58 `state_mapped` plus 207
`source_mapped`, with 142 required; heat-balance/HVAC lists remain 88/31
and HVAC readiness remains `0/31`. The parent stays `scaffold` at claim
level `none`.

CP262 next adds required source-mapped `routine.set_zone_equip_sim_order`
after `routine.sim_zone_equipment` and before `routine.sim_purchased_air`.
`SetZoneEquipSimOrder` is declared at
`ZoneEquipmentManager.hh` line 186 and implemented completely at
`ZoneEquipmentManager.cc` lines 4195-4255. `InitSystemOutputRequired`
begins at line 4257 and is declared at header line 188.

## CP262 `SetZoneEquipSimOrder` Shared Priority-Scratch Rebuilder

CP262 adds canonical required `routine.set_zone_equip_sim_order`
immediately after `routine.sim_zone_equipment` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. The routine is declared at `ZoneEquipmentManager.hh` line 186 and
implemented completely at `ZoneEquipmentManager.cc` lines 4195-4255:

```cpp
void SetZoneEquipSimOrder(
    EnergyPlusData &state,
    int const ControlledZoneNum);
```

The declaration omits the definition's top-level `const` on the by-value
integer; the C++ function type is unchanged. `ControlledZoneNum` is the
actual Zone index, not an ordinal within the controlled-Zone subset. The
routine returns `void` and writes the manager-global `PrioritySimOrder`
scratch array while reading the canonical equipment list and current Zone
sensible demand.

The exact source order is:

1. alias `ZoneEquipList(ControlledZoneNum)`, read its `NumOfEquipTypes` as
   `N`, and copy each active list row into scratch positions `1..N`;
2. copy six fields per row: equipment type name, equipment name, enum type,
   cooling priority, heating priority, and the original list ordinal as
   `EquipPtr`;
3. visit scratch positions `N+1..U`, where `U` is the scratch upper bound,
   clearing the two names, setting the enum to `Invalid`, and setting
   `EquipPtr` to zero, while leaving both priority integers untouched; then
4. for each active position `i`, compare it with every position `j=i..N`
   and immediately exchange all six fields whenever the selected candidate
   has a smaller priority, refreshing both current-priority locals after
   every exchange.

This is an in-place exchange-selection pass, not a conventional selection
sort that finds one minimum before one swap. Negative
`RemainingOutputRequired` selects ascending cooling priority. Zero or
positive demand selects ascending heating/no-load priority; both positive
and negative zero therefore use heating. A NaN demand satisfies neither
sign comparison and leaves the freshly copied source order unchanged.
The routine does not read `LoadDistScheme`, availability counts, capacity
caches, schedules, `FirstHVACIteration`, or Space demand.

Source input parsing rejects values below zero or above `N`, so zero is
accepted despite the diagnostic text saying priorities must be positive.
It rejects duplicate positive priorities and warns about missing positive
sequence numbers; multiple zeros can remain. CP262 itself neither skips
zero nor consults `NumAvailHeatEquip` or `NumAvailCoolEquip`, so a selected
zero sorts before every positive value. The comparison is strict, which
prevents a direct equal-key exchange, but the immediate-exchange algorithm
is not globally stable if malformed equal priorities reach it because an
intervening smaller row can reverse equal-key rows.

All six fields move as one logical record. In particular, `EquipPtr`
continues to identify the original list row after sorting, and the
unselected priority dimension travels with that row rather than being
recomputed. Names, enum, and pointer above `N` are scrubbed, but the two priority fields
there preserve their pre-existing bytes: allocation defaults until written,
then values from whichever prior larger Zone populated them.
`PrioritySimOrder` is allocated by `GetZoneEquipment` to the maximum
equipment-list count and shared across Zones, so the last caller wins; it
is not a per-Zone cache.

For valid `0 <= N <= U`, let `S` be the number of successful exchanges.
The copy performs `6N` field writes, upper cleanup performs `4(U-N)`
mutations, and the nested loops visit exactly `N(N+1)/2` pairs, including
self-pairs. Each exchange has six swap calls and twelve destination-field
endpoints, with `0 <= S <= N(N-1)/2`. Dynamic persistent mutation-statement
count is

`6N + 4(U-N) + 6S = 2N + 4U + 6S`;

counting the two endpoints of every swap separately gives
`2N + 4U + 12S`. Exactly `2(U-N)` upper priority cells are not written.
The body has four `for` loops and one `if`, with no `else`, switch, return,
break, or continue. It has eight direct persistent assignment sites and
eight mutating call sites: two string clears, two string swaps, and four
scalar `std::swap` calls. It allocates no local scratch array and calls no
EnergyPlus child, service, or diagnostic routine.

The only direct production caller is `initOutputRequired` line 4315, gated
by `ResetSimOrder && spaceNum == 0`. That child first restores the selected
Zone's `RemainingOutputRequired` from `TotalOutputRequired`, so ordinary
runtime sorting uses total Zone-load sign rather than a prior equipment
residual. `InitSystemOutputRequired` initializes the Zone first and may then
initialize its Spaces with nonzero `spaceNum`; those Space calls do not
reorder, even when a Space load sign differs from the Zone sign.

CP261 `SimZoneEquipment` invokes the reset-true wrapper once before
dispatch for every controlled Zone. Its tail
`CalcZoneLeavingConditions` invokes it a second time only for controlled
Zones with at least one return node. A successful CP261 call therefore
sorts `C+K` times for `C` controlled Zones and `K` such Zones with return
nodes. The sizing path can reach the same return-node call without the
front dispatch call. The second normal-runtime sort occurs after equipment
effects and overwrites the same shared scratch; it does not preserve a
Zone-specific result.

There is no comprehensive up-front validation, local status, diagnostic,
catch, cleanup guard, checkpoint, transaction, or rollback. An invalid Zone
identity, missing demand/list state, short canonical field array, or
unallocated or undersized scratch can fail before completion; depending on
the failing access, it can leave no new write or retain an already copied
or cleared prefix. A string-copy allocation failure can similarly leave
mixed old and new active rows. No rollback restores prior scratch bytes,
and a tail failure in CP261 also retains all earlier equipment and
mass-balance effects.

A successful replay with unchanged canonical list and load sign is
active-prefix idempotent: all six fields are recopied before sorting, so it
reconstructs a prior torn or permuted active prefix. The untouched upper
priority cells remain history-dependent. A sign change deliberately
rebuilds from canonical order and selects the other priority dimension;
it does not incrementally sort the previous result.

The direct unit census finds one explicit `SetZoneEquipSimOrder` call and
seven reset-true `InitSystemOutputRequired` calls. Four use `N=3` and four
use `N=4`, for 28 copied rows and 64 pair visits, but all heating and
cooling priorities are already `1..N`, so no exchange occurs. Named
parent-level unit paths bring the statically attributable total to 59
successful executions, yet every nonempty list is already ordered and no
exchange executes. Four UnitHeater assertions read slots one and two
`EquipTypeName`/`EquipName` fields and prove only the already-ordered
ADU-before-UnitHeater tuple. Nearby parsed data with different cooling
order never calls the routine.

The tests therefore do not prove an unsorted heating or cooling result,
different orders across load signs, exact-zero or NaN selection, zero or
gapped priorities, full-record tuple integrity, `U>N` cleanup and retained
priority tails, Space non-reordering, shared-scratch overwrite, successful
replay, invalid extents, or partial-failure state. Uniform, UniformPLR, and
SequentialUniformPLR distribution tests call initialization with
`ResetSimOrder=false`; only Sequential paths exercise this boundary.

Rust contains no exact or snake-case occurrence of the routine, scratch,
remaining-demand discriminator, two priority fields, or equipment pointer.
It parses all four distribution-scheme names but the three nonsequential
variants have no active runtime consumer. Compiler input requires positive
`u32` sequences and rejects duplicate cooling or heating priorities,
whereas the source accepts zero. `ModelGraph` performs a one-time static
sort by `(zone, heating, cooling, ideal_loads_id)`, and node projection uses
a similar heating-first minimum. Neither can express source cooling-first
ordering under negative demand or a runtime sign change.

The active compatibility runtime does not interpret the
`SimZoneEquipment` execution step or graph order; it directly iterates
prebound IdealLoads systems. All 30 active equipment lists contain exactly
one SequentialLoad IdealLoads entry at cooling/heating sequence `1/1`, with
both fraction schedules blank. Thus the current Rust lane makes sorting a
one-record no-op and owns no dynamic priority scratch, upper-tail policy,
record exchange, replay behavior, or list capacity cache.

CP262 changes no Rust target/state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 266 routines, split 58
`state_mapped` plus 208 `source_mapped`, with 143 required. Domain-required
counts become heat-balance 88, HVAC 32, plant 1, and time/schedule 22, with
readiness `0/88`, `0/32`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

CP263 next adds required source-mapped
`routine.init_system_output_required` immediately after
`routine.set_zone_equip_sim_order` and before
`routine.sim_purchased_air`. `InitSystemOutputRequired` is declared at
`ZoneEquipmentManager.hh` line 188 and implemented completely at
`ZoneEquipmentManager.cc` lines 4257-4290. Its child
`initOutputRequired` begins at source line 4292.

## CP263 `InitSystemOutputRequired` Zone/Space Demand-Initialization Wrapper

CP263 adds canonical required `routine.init_system_output_required`
immediately after `routine.set_zone_equip_sim_order` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. The public wrapper is declared at `ZoneEquipmentManager.hh` line 188
and implemented completely at `ZoneEquipmentManager.cc` lines 4257-4290:

```cpp
void InitSystemOutputRequired(
    EnergyPlusData &state,
    int const ZoneNum,
    bool const FirstHVACIteration,
    bool const ResetSimOrder);
```

The header omits the definition's top-level `const` on all three by-value
parameters; the C++ function type is unchanged. Only the header supplies
`ResetSimOrder = false`. `ZoneNum` is an actual parent Zone index,
`FirstHVACIteration` is forwarded to initialization and distribution, and
`ResetSimOrder` is forwarded only to the initializers.

The exact wrapper order is:

1. call `initOutputRequired` for
   `ZoneSysEnergyDemand(ZoneNum)` and
   `ZoneSysMoistureDemand(ZoneNum)`;
2. when `doSpaceHeatBalance` is true, visit every occurrence in
   `Zone(ZoneNum).spaceIndexes` in stored order and call the same child for
   the selected Space sensible/moisture pair, passing both the unchanged
   parent `ZoneNum` and explicit `spaceNum`, which is nonzero for a valid
   Space identity; then
3. after every initializer returns, call
   `DistributeSystemOutputRequired(state, ZoneNum,
   FirstHVACIteration)` exactly once.

The wrapper owns no controlled-Zone, `ZoneSizingCalc`, `DoingSizing`,
simulation-only Space, per-Space control, owner, uniqueness, or validity
gate. Its Space flag is `doSpaceHeatBalance`, not the narrower
`doSpaceHeatBalanceSimulation` used by parts of CP261. Duplicate or
cross-listed Space identities therefore repeat, and every occurrence uses
the referring parent Zone's equipment-list, sizing, control, and deadband
context. The wrapper does not deduplicate or verify Zone membership.

The Zone initializer receives references to the Zone demand pair. Space
initializers receive references into the separate Space demand arenas but
still receive the parent `ZoneNum`; a duplicated identity revisits the same
Space records. Each successful child unconditionally copies
six sensible and six moisture remaining/unadjusted scalars from predictor
totals and setpoint totals, and finally copies
`DeadBandOrSetback(ZoneNum)` into the same
`CurDeadBandOrSetback(ZoneNum)` cell. Thus Space traversal repeats that one
Zone deadband write rather than owning a per-Space flag.

`ResetSimOrder` reaches every initializer unchanged, but CP262
`SetZoneEquipSimOrder` runs only from the Zone child because the child gate
also requires `spaceNum == 0`. Valid Space calls never select their own
priority order; all share the Zone-selected manager-global scratch. The
Zone child performs its 12 scalar demand copies before CP262, so sorting
uses the newly restored Zone `RemainingOutputRequired` sign.

After those base writes, `initOutputRequired` conditionally initializes
sequenced arrays. It tests allocation of only the main sensible sequence
before assuming that the other two sensible and all three moisture arrays
are conformable. Uncontrolled or Zone-sizing entries bulk-fill all six
sequences from predictor totals. On a controlled, nonsizing first HVAC
iteration, Sequential and Uniform do the same, whereas UniformPLR and
SequentialUniformPLR seed all three sensible arrays from the sign-selected
final design load and copy moisture totals. A later entry writes only
sequence slot one from predictor totals and may add current sensible and
latent duct loss, leaving higher slots untouched at that phase. CP264 maps
that complete child separately.

`DistributeSystemOutputRequired` is called even when it will return
immediately. Its current source expands into one Zone and the same stored
Space occurrences only when

```text
G = Zone.IsControlled
    && !ZoneSizingCalc
    && !(FirstHVACIteration
         && LoadDistScheme != Uniform
         && LoadDistScheme != Sequential)
```

so first-iteration UniformPLR and SequentialUniformPLR, uncontrolled, and
Zone-sizing calls retain initialization-only state. Later iterations of all
valid schemes can distribute. CP265 maps the distributor and its leaf
separately; CP263 records this parent ordering and gate interaction without
promoting those children.

Let `I` be one when Space heat balance is enabled and `M` the number of
stored Space-index occurrences. Then `H = 1 + I*M` initializers run and the
wrapper makes exactly `H+1 = 2+I*M` direct child calls. When `G` is true,
the distributor expands into another `H` distribution leaves. A successful
reset-true call also reaches CP262 exactly once. The fully expanded
principal count is therefore

`H + 1 + G*H + R`,

where `R` is one for reset-true and zero otherwise. Successful initializer
base work contributes `12H` demand-scalar assignments before conditional
sequence writes plus `H` deadband assignments afterward, `13H` total.

The wrapper body itself has one `if`, one range `for`, no `else`, switch,
return, break, or continue, and zero direct assignment or persistent-write
sites. It has eight syntactic call expressions when five Zone/Space array
accessors are counted with the two initializer sites and one distributor.
It performs no direct arithmetic, allocation, diagnostic, or cleanup.

There are exactly two executable production call sites, and both explicitly
pass `ResetSimOrder=true`. CP261 `SimZoneEquipment` calls the wrapper once
before equipment dispatch for every controlled Zone.
`CalcZoneLeavingConditions` calls it once, not once per return node, for
each controlled Zone having at least one return node. The latter parent is
reached from normal CP261 and from `SizeZoneEquipment` with
`FirstHVACIteration=true`.

A normal successful equipment pass therefore invokes CP263 `C+K` times for
`C` controlled Zones and `K` controlled Zones with return nodes. The second
call for a return-node Zone occurs after equipment, mass-balance, and
leaving-condition effects and restores predictor demand again. A controlled
Zone without return nodes receives only the front reset and can finish with
post-equipment residual demand. The sizing path reaches only the return-node
call, and its distributor returns immediately because `ZoneSizingCalc` is
true.

There is no local validation, result status, diagnostic, catch, cleanup
guard, checkpoint, transaction, or rollback. Invalid Zone or top-level
demand identity can fail before child entry. A Zone-child failure blocks
every Space and distribution. CP262 failure occurs after the 12 Zone scalar
copies but before sequenced-array and deadband completion. Sequence shape
failure can preserve a partial bulk-write prefix.

Failure resolving or initializing Space occurrence `k` retains the
completed Zone and prior-Space prefixes and prevents all later Spaces and
distribution. A distribution failure retains every initialization effect;
failure in a later distribution leaf additionally retains the Zone and
earlier-Space distribution prefix. No wrapper action restores any of those
states.

With every mutable dependency fixed, an immediate successful replay is
overwrite-idempotent on fields actually rewritten. It is not a canonical
whole-state repair: later Sequential initialization leaves sequence slots
above one, upper priority fields retain history, and reset-false calls can
consume another Zone's shared order. Capacities, fraction schedules, duct
loss, load sign, Space membership, and control flags are resampled, while
duplicate Space occurrences are replayed independently.

The direct C++ census finds 24 wrapper expressions across six tests and all
four distribution schemes. Ten use the first HVAC iteration and 14 a later
iteration; seven pass reset true and 17 use the header default false. Basic
Sequential and each of Uniform and UniformPLR contribute four calls,
SequentialUniformPLR contributes eight, and two mixed-equipment Sequential
tests contribute two each. Every direct call uses one controlled,
nonsizing Zone with allocated sequences and no Space traversal.

Those tests strongly assert sensible sequence arrays for heating and
cooling, first and later iterations, active-equipment counts, design-load
PLR seeding, learned-capacity PLR distribution, and sequential fraction
schedules. However, every one of the 17 reset-false calls is followed by a
separate explicit distributor call; the mixed test also inserts a direct
priority reset. Their end-state assertions therefore do not isolate
CP263's final child. Only the seven reset-true Sequential calls exercise
the wrapper end to end without a duplicate distribution.

Named parent paths add 51 statically attributable wrapper executions, for
an audited total of 75: 58 reset true versus 17 false and 49 first versus
26 later iterations. None enables Space heat balance, so zero Space-demand
children are tested. No assertion proves any of the six unadjusted scalars,
a meaningful current-deadband copy, a nonzero moisture sequence, duct-loss
addition, Space traversal, uncontrolled or sizing behavior, an invalid
scheme, mismatched allocation, failure prefix, or replay repair. Three
moisture remaining assertions use zero inputs/defaults, and both sides of
the observed deadband copy are false.

Rust contains no exact or snake-case wrapper, child, distributor,
`FirstHVACIteration`, `ResetSimOrder`, `doSpaceHeatBalance`, or matching
reset-field implementation. Its `ZoneSysEnergyDemand` subset owns only a
Zone ID plus four heating/cooling/humidifying/dehumidifying
setpoint-remaining values. It has no total or unadjusted predictor fields,
six sequenced arrays, Zone/Space demand arenas, deadband state, shared
priority scratch, or distribution lifecycle.

The active compatibility runtime constructs one fresh four-field demand
snapshot per IdealLoads system from options and calls the prebound
PurchasedAir path directly. Execution-plan Zone-equipment labels are not
interpreted as CP263. The active data census has 30 equipment lists,
30 connections, and 30 IdealLoads systems but no Space, SpaceList, or
SpaceHVAC object. Every list is one-entry SequentialLoad at cooling/heating
sequence `1/1`, so current fixtures expose only the trivial `M=0` topology.

CP263 changes no Rust target/state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 267 routines, split 58
`state_mapped` plus 209 `source_mapped`, with 144 required. Domain-required
counts become heat-balance 88, HVAC 33, plant 1, and time/schedule 22, with
readiness `0/88`, `0/33`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

CP264 next adds required source-mapped `routine.init_output_required`
immediately after `routine.init_system_output_required` and before
`routine.sim_purchased_air`. `initOutputRequired` is declared at
`ZoneEquipmentManager.hh` lines 190-196 and implemented completely at
`ZoneEquipmentManager.cc` lines 4292-4388.
`DistributeSystemOutputRequired` begins at source line 4390.

## CP264 `initOutputRequired` Demand and Sequence Reset Leaf

CP264 adds canonical required `routine.init_output_required` immediately
after `routine.init_system_output_required` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. The lowercase leaf is declared at `ZoneEquipmentManager.hh` lines
190-196 and implemented completely at `ZoneEquipmentManager.cc` lines
4292-4388:

```cpp
void initOutputRequired(
    EnergyPlusData &state,
    int const ZoneNum,
    DataZoneEnergyDemands::ZoneSystemSensibleDemand &energy,
    DataZoneEnergyDemands::ZoneSystemMoistureDemand &moisture,
    bool const FirstHVACIteration,
    bool const ResetSimOrder,
    int spaceNum);
```

Only the header supplies `spaceNum = 0`. The two demand arguments are
mutable references and can identify a Zone pair, a Space pair, or arbitrary
caller-owned records. `ZoneNum` still controls every shared Zone lookup.
`spaceNum` is otherwise unused: zero permits the reset-order child, while
any nonzero value, including malformed negative input, suppresses it.

The exact source order begins with 12 unconditional scalar restores. The
six sensible writes are:

- `RemainingOutputRequired` and `UnadjRemainingOutputRequired` from
  `TotalOutputRequired`;
- `RemainingOutputReqToHeatSP` and its unadjusted counterpart from
  `OutputRequiredToHeatingSP`; and
- `RemainingOutputReqToCoolSP` and its unadjusted counterpart from
  `OutputRequiredToCoolingSP`.

The six moisture writes repeat the same pattern for total,
humidifying-setpoint, and dehumidifying-setpoint demand. No clamp,
finite-value check, sign normalization, multiplier, or unit conversion is
performed.

If `ResetSimOrder && spaceNum == 0`, CP264 then calls CP262
`SetZoneEquipSimOrder(state, ZoneNum)`. The normal CP263 Zone call passes
the same state demand record as `energy`, so the preceding total-to-
remaining copy determines CP262's cooling-versus-heating sign. An arbitrary
direct caller can pass a different sensible record; CP262 still reads the
state-owned Zone demand, and no identity check ties the two together.

Every sequence mutation is then gated only by

```cpp
allocated(energy.SequencedOutputRequired)
```

The other two sensible and all three moisture sequences are assumed to
exist, and a zero-length allocated main vector still passes. The gate does
not check companion allocation, nonempty slot one, equal extents, active
equipment count, or the correspondence between Zone/Space demand and
`ZoneNum`.

For an uncontrolled Zone or while `ZoneSizingCalc` is true, the leaf
broadcasts predictor totals across all six independent destination
extents: sensible total, heating setpoint, cooling setpoint, moisture total,
humidifying setpoint, and dehumidifying setpoint. This branch ignores
`FirstHVACIteration`, load-distribution scheme, design load, duct loss, and
`spaceNum`.

For a controlled, nonsizing first HVAC iteration, the source reads the
parent Zone equipment-list scheme once:

- Sequential and Uniform perform the same six predictor-value broadcasts.
- UniformPLR and SequentialUniformPLR broadcast a design value into each of
  the three sensible sequences and predictor values into the three moisture
  sequences.
- Invalid, `Num`, or any other cast value matches neither branch and writes
  no sequence element.

Each of the three PLR sensible broadcasts independently tests only
`energy.TotalOutputRequired >= 0.0`. A nonnegative total, including positive
or negative zero, selects the parent
`FinalZoneSizing(ZoneNum).DesHeatLoad`; a negative total selects
`-DesCoolLoad`. NaN makes the comparison false and therefore selects
negative cooling design load. Heating- and cooling-setpoint sequence seeds
do not use their own setpoint-demand signs or magnitudes.

A Space call uses the passed Space demand sign but still reads the parent
Zone's controlled flag, equipment-list scheme, `FinalZoneSizing`, and
deadband, plus shared duct-loss state. It never reads `FinalSpaceSizing`.
Full broadcasts
fill each vector's own complete extent, including any tail beyond an active
equipment prefix.

For a controlled, nonsizing later HVAC iteration, scheme is not read. The
leaf writes only index one of all six sequences from the predictor totals
and setpoint totals. When `DuctLossSimu` is true, it then adds the same
`SysSen` value to the three sensible cells and the same `SysLat` value to
the three moisture cells. The additions are raw: no finite, sign, or
magnitude guard exists. Every sequence element above one retains its prior
value.

Finally, after all sequence handling, CP264 always writes

```cpp
CurDeadBandOrSetback(ZoneNum) = DeadBandOrSetback(ZoneNum);
```

This is a Zone-level destination even when the demand references identify a
Space. Repeated or duplicate Space calls therefore rewrite the same parent
Zone flag; there is no per-Space deadband state in this leaf.

The body has 11 `if` tokens, including two `else if`, six `else` tokens,
and no loop, switch, return, break, continue, diagnostic, catch, cleanup,
transaction, or rollback. It contains 46 direct persistent mutation sites:
40 plain assignments and six compound additions. One separate local initialization establishes the distribution-scheme
value. Its 24 syntactic
calls/accessors comprise CP262, `allocated`, the Zone and equipment-list
lookups, six final-sizing accesses, 12 index-one sequence accesses, and two
deadband accesses.

Those sites address 19 direct destination families: 12 scalar demand
fields, six sequence vectors, and one Zone deadband field. Baseline
successful work executes 13 assignments. An allocated recognized
full-initialization path executes six more statements, for 19. A later
duct-off path also executes 19; duct-on executes 25 because each of the six
slot-one cells is written and then incremented. An invalid first-iteration
scheme or unallocated main gate remains at 13.

Let `L` be the sum of the six independent vector extents. A full broadcast
performs `L` sequence-element writes in six statements, for `13+L` direct
destination writes overall. Later duct-off touches six sequence cells;
duct-on performs 12 operations on those same six cells. If CP262 runs, its
`2N + 4U + 6S` scratch-mutation count is additional.

There are exactly three production call expressions. Sizing
`sizeZoneSpaceEquipmentPart1` line 339 passes first iteration true, reset
false, and the current Zone or Space demand pair. CP263 calls the leaf once
for the Zone and once per stored Space occurrence. The latter wrapper
forwards its first/reset flags and uses reset true at both executable
production call sites.

Let `I` indicate Space heat balance, `C` be controlled Zones, `M` their
stored Space occurrences, `K` be controlled Zones with return nodes, and
`M_K` their stored Space occurrences. A normal or sizing manager pass
therefore reaches

`C + K + I*(M + M_K)`

CP264 calls. Normal simulation gets `C+I*M` through CP263 before equipment
and `K+I*M_K` through leaving conditions afterward; all wrapper calls pass
reset true, so CP262 runs for the `C+K` Zone children. Sizing gets the first
group directly through Part1 with reset false, then the return-node group
through CP263; only its `K` valid Zone children run CP262. `ZoneSizingCalc`
selects full sequence broadcasts only when the main sensible sequence is
allocated, and the following distributor returns.

No C++ test calls the lowercase leaf directly. The audited lower-helper
census finds 82 executions: 75 through CP263 and seven through sizing
Part1. Fifty-six are first-iteration and 26 later; 58 pass reset true and
24 false. Total sensible signs are 23 positive, 43 negative, and 16 exact
zero. Sequence shapes are eight unallocated, 48 length-one, two
length-two, 20 length-three, and four length-four cases. The allocated
schemes comprise 58 Sequential, four Uniform, four UniformPLR, and eight
SequentialUniformPLR executions.

The 24 explicit wrapper calls strongly assert all three sensible sequence
families across four schemes, both load signs, and first/later behavior.
First-iteration PLR tests directly preserve and verify positive heating or
negative cooling design-load seeds. Seventeen reset-false calls explicitly
run distribution again, so many end-state assertions cannot isolate CP264.
The tests containing the 51 named-parent executions assert downstream
equipment, airflow, and return results rather than the leaf's immediate
destinations. The tests containing the seven sizing executions likewise
assert downstream load, DOAS, and node behavior.

Across all 82 audited executions, no Space-HB child runs and duct loss is
always disabled. Eight unallocated cases execute the gate but have no
sentinel oracle. There are zero assertions for the three sensible
unadjusted fields, moisture unadjusted fields, any moisture sequence, or
`CurDeadBandOrSetback`. Only three moisture remaining assertions read
zero-valued defaults. The sizing calls include nonzero moisture and five
true deadband sources, but only downstream descendants observe them and
the destination copy is never asserted.

Tests also omit an uncontrolled or sizing call with allocated arrays, an
invalid scheme, the zero/NaN PLR sign boundary, mixed total-versus-setpoint
signs, empty or mismatched companions, invalid Zone/Space identity, missing
final sizing, partial failure, rollback, and deliberate replay
reconstruction. Twenty-six leaving-condition replay executions occur (20
first-iteration and six later-iteration), but none deliberately corrupts
all destinations first or distinguishes repaired fields from retained
tails.

There is no up-front validation, result status, diagnostic, catch,
checkpoint, cleanup, transaction, or rollback. A failure after entry
retains the 12 restored scalars. CP262 failure retains that scalar prefix
and any scratch prefix already mutated; sequence or final-sizing failure
additionally retains completed CP262 work and any prior sequence writes. A final deadband lookup failure follows every earlier
mutation. Malformed later companion vectors can fail partway through the
six slot-one writes. An allocated zero-length main vector reaches that
later indexing path, which can throw under bounds checking or be undefined
without it.

With all mutable dependencies fixed, successful replay overwrites every
destination it rewrites. Duct-loss additions do not accumulate because
slot one is restored before `+=`. Full broadcasts can repair current
vector extents, while later, unallocated, and invalid-first paths preserve
some or all sequence tails. CP262's upper priority tail also remains
history-dependent. Allocation/extents, Zone control, sizing flag, scheme,
demand, design loads, duct state, and deadband are resampled on every call,
so the leaf has no canonical whole-state repair protocol.

Rust contains no exact or snake-case CP264 leaf, total/unadjusted demand,
six sequenced arrays, Zone/Space demand arenas, predictor/current deadband,
final Zone sizing design loads, duct-loss state, or operational
first-iteration, reset-order, or Zone-sizing flags. Exact source-name hits are confined to metadata/comments for selected
remaining and predictor-output fields; none implements CP264 state.

Rust `ZoneSysEnergyDemand` owns only a Zone ID and four heating, cooling,
humidifying, and dehumidifying setpoint-remaining scalars. The moisture
predictor can compute total and setpoint loads, but only two setpoint loads
enter this snapshot. `Deadband` is an IdealLoads calculation-result mode,
not the source predictor/current deadband pair, and
`hvac_iteration_count` is initialized and report-copied rather than used as
this branch discriminator.

The active runtime creates a fresh independent demand snapshot for each
IdealLoads system and calls prebound PurchasedAir directly. It never reads
the stored load-distribution scheme. Active data contain 30 equipment
lists, 30 connections, and 30 IdealLoads systems, with zero Space,
SpaceList, SpaceHVAC, Sizing:Zone, or duct-loss object. Every list is
one-entry SequentialLoad at sequence `1/1`, and all 61 active
SimulationControl objects disable Zone sizing.

CP264 changes no Rust target/state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 268 routines, split 58
`state_mapped` plus 210 `source_mapped`, with 145 required. Domain-required
counts become heat-balance 88, HVAC 34, plant 1, and time/schedule 22, with
readiness `0/88`, `0/34`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

CP265 next adds required source-mapped
`routine.distribute_system_output_required` immediately after
`routine.init_output_required` and before `routine.sim_purchased_air`.
`DistributeSystemOutputRequired` is declared at
`ZoneEquipmentManager.hh` line 198 and implemented completely at
`ZoneEquipmentManager.cc` lines 4390-4419. Its leaf
`distributeOutputRequired` begins at source line 4421.

## CP265 `DistributeSystemOutputRequired` Gate and Zone/Space Dispatcher

CP265 adds canonical required
`routine.distribute_system_output_required` immediately after
`routine.init_output_required` and before `routine.sim_purchased_air`,
plus the same ordered HVAC project-contract item. The wrapper is declared
at `ZoneEquipmentManager.hh` line 198 and implemented completely at
`ZoneEquipmentManager.cc` lines 4390-4419:

```cpp
void DistributeSystemOutputRequired(
    EnergyPlusData &state,
    int const ZoneNum,
    bool const FirstHVACIteration);
```

The header has no default argument and spells the two by-value parameters
without top-level `const`; the definition adds function-type-neutral
top-level `const` to both. There is no Space-number parameter. `ZoneNum`
selects the Zone and equipment-list priority, capacity, and fraction
context. CP266 separately consumes manager-global `PrioritySimOrder`
scratch whose correspondence to that Zone is an unchecked upstream
invariant.

The gate order is exact:

1. read `Zone(ZoneNum).IsControlled` and return when false;
2. read `ZoneSizingCalc` and return when true;
3. on the first HVAC iteration, return unless the Zone equipment-list
   scheme is Uniform or Sequential;
4. call lowercase `distributeOutputRequired` for the Zone sensible and
   moisture demand records; and
5. after that Zone child returns, read `doSpaceHeatBalance`, visit every
   stored `Zone(ZoneNum).spaceIndexes` occurrence in order, and call the
   same child for each Space demand pair.

Short-circuit evaluation matters. A later iteration performs no
wrapper-level equipment-list lookup. A first-iteration Uniform call reads
the list once because the first inequality is false. Sequential reads it
twice, first rejecting Uniform and then accepting Sequential. UniformPLR,
SequentialUniformPLR, Invalid, `Num`, and arbitrary other enum casts also
read it twice and return silently. The scheme is not cached in a local
snapshot.

This preserves the CP264 protocol. First-iteration Sequential and Uniform
calls enter CP266 and redistribute CP264 predictor broadcasts.
First-iteration UniformPLR and SequentialUniformPLR return so CP264's
design-load sequence seeds remain available for capacity discovery. An
unknown first-iteration scheme also returns, while CP264 has no matching
sequence-write branch, so prior sequence state can survive without a
diagnostic. Later iterations pass every scheme to CP266; its invalid
default is fatal. Uncontrolled and Zone-sizing calls likewise leave the
CP264 result untouched.

Define

```text
G = IsControlled
    && !ZoneSizingCalc
    && (!FirstHVACIteration
        || scheme == Uniform
        || scheme == Sequential)
```

For current Space flag `I` and `M` stored Space occurrences, a fully
successful wrapper call dispatches

```text
G * (1 + I*M)
```

lower-leaf executions. The Zone always precedes every Space. Duplicate or
cross-listed Space identities are not deduplicated, so the same demand
record can be revisited. A Space child receives its own demand records but
the unchanged parent `ZoneNum`; it therefore reuses the parent equipment
list with its priorities, learned capacities, and fraction schedules, plus
the current manager-global priority scratch and shared duct-loss state.
Scratch-to-parent correspondence is not validated. The flag is
`doSpaceHeatBalance`, not the narrower simulation-only flag. No Space
control, ownership, identity, or membership validation exists.

The wrapper has four `if` statements, one range-for, three `return`
statements, two `&&` tokens, and no `else`, switch, case, break, continue,
catch, diagnostic, or assignment operator. It has two lower-leaf call
sites and ten syntactic calls/accessors: those two children, two Zone
lookups, two equipment-list lookups, and four Zone/Space demand lookups.
There is no direct persistent mutation, result status, local recovery,
cleanup, checkpoint, transaction, or rollback.

All successful writes belong to CP266. In dependency context, that leaf
targets 12 demand families per selected record: six sequence vectors and
six adjusted remaining-demand scalars. It does not rewrite predictor
totals, unadjusted demand, CP264 deadband state, equipment-list data,
learned capacities, or priority scratch.

CP265 performs no allocation or extent check. Sequential CP266 needs
priority slot one in scratch corresponding to the current Zone, a valid
equipment pointer, and slot one in all six sequences. Nonsequential paths
need compatible list priority/capacity extents and demand-vector coverage
through the active equipment range;
their common tail reads slot one. CP264's sole main-sequence allocation
test governs only CP264's own writes and does not make CP265 safe.

The only production call expression is CP263
`InitSystemOutputRequired` at `ZoneEquipmentManager.cc` line 4289, after
the Zone initializer and every `doSpaceHeatBalance`-selected stored
Space-occurrence initializer return. A normal `SimZoneEquipment` pass
reaches CP265 once for each `ZoneEquipConfig.IsControlled` Zone before
equipment and again for each such Zone with a return node during leaving
conditions. CP265 independently gates on `Zone(ZoneNum).IsControlled`;
ordinary input processing aligns the two flags, but the wrapper does not
validate that invariant. With upstream-controlled count `C`, return-node
upstream-controlled count `K`, Space flag `I`, their stored occurrence
counts `M` and `M_K`, this is nominally `C+K` wrapper calls. A later
valid-scheme, control-aligned pass can dispatch

```text
C + K + I*(M + M_K)
```

lower leaves. A first pass includes only the Sequential/Uniform subsets.
Sizing Part1 calls CP264 directly and never CP265. Sizing can later reach
CP265 through the `K` leaving-condition wrappers, but `ZoneSizingCalc`
makes every one return before a lower Zone or Space call.

The audited C++ unit corpus executes the public wrapper exactly 92 times:
75 through CP263 and 17 through direct public call expressions. The 75
comprise 24 explicit CP263 calls plus 51 named-parent executions. Sixteen
direct calls immediately repeat the CP263 child; the seventeenth follows
CP263 plus an explicit `SetZoneEquipSimOrder`. No unit test calls lowercase
`distributeOutputRequired` directly.

The 92 public calls divide as follows:

- 55 first-iteration and 37 later-iteration calls;
- 60 Sequential, eight Uniform, eight UniformPLR, and 16
  SequentialUniformPLR schemes;
- 31 positive, 48 negative, and 13 exact-zero total sensible loads; and
- one unallocated main sequence, 48 length-one, two length-two, 36
  length-three, and five length-four shapes at public entry.

Ninety-one calls are controlled; the lone unallocated public entry is
uncontrolled. All 92 are outside Zone sizing, Zone-level, and have
`doSpaceHeatBalance=false`. Thus the corpus executes the uncontrolled
return once, never executes the sizing return, and dispatches no Space
child.

Eight first-iteration PLR calls return at the scheme gate: four CP263
children and their four direct repeats, covering heating/cooling
UniformPLR and SequentialUniformPLR cases. One additional first-iteration
call returns at the uncontrolled gate. The other 83 calls dispatch the
Zone leaf. Those lower calls divide into 46 first and 37 later calls, with
59 Sequential, eight Uniform, four UniformPLR, and 12
SequentialUniformPLR schemes. Their entry shapes are 48 length-one, two
length-two, 28 length-three, and five length-four records. Every later PLR
case computes a positive PLR; no `plr <= 0` no-write branch runs.

The lone unallocated public entry occurs in
`CZoeEquipmentManager_CalcZoneLeavingConditions_Test`.
`ZoneEquipConfig.IsControlled` is true, but
`Zone(1).IsControlled` remains default false, so CP265 returns at its first
gate before reading the scheme or reaching CP266. Its assertions read
return temperature, not CP265 demand preservation. It is an unasserted
uncontrolled-gate execution, not an unallocated lower-leaf dispatch.

Six explicit distribution tests contain exactly 222 sensible sequence
endpoint assertions:

- Sequential: 36;
- Uniform: 36;
- UniformPLR: 36;
- SequentialUniformPLR: 72;
- mixed Sequential equipment: 24; and
- mixed Sequential equipment with fractions: 18.

They strongly cover sensible formulas, both load signs, Uniform active
heating/cooling counts, UniformPLR capacities, and
SequentialUniformPLR selection of one, two, or three heating units and one
or two cooling units. The four first-PLR scenarios contribute 36
no-op-gate assertions: the positive heating or negative cooling CP264
design seeds survive both the internal CP265 call and its direct repeat.

The mixed-fraction test is the clearest Sequential mutation evidence. Its
positive heating fraction 0.4 scales slot one while the first-call tails
remain at full demand; a later update applies the second equipment's 0.6
fraction. A cooling fraction 0.3 is configured but no negative
mixed-fraction call exercises it.

Sixteen scenarios contain exactly 48 assertions for the three adjusted
sensible `Remaining*` fields. Twelve nonsequential scenarios reflect
CP265 slot-one distribution. Two basic Sequential later cases are
indistinguishable from CP264 because their fraction is one, and two mixed
later cases are observed only after `updateSystemOutputRequired`.

There are zero moisture-sequence assertions. Only three moisture
`Remaining*` assertions exist, all reading zero, and every CP265 input has
zero moisture predictor demand. The 51 named-parent executions have no
direct CP265 destination assertion; their host tests check downstream
equipment, airflow, and return behavior.

Repeated direct calls make the positive distribution evidence less
isolated. Uniform and later-PLR assertions follow a second deterministic
distribution, so they prove the aggregate replay-stable result but cannot
identify which invocation repaired a destination. All four repeated direct
first-PLR calls are no-ops. The corpus contains 13 direct distributing
replays and 26 leaving-condition wrapper replays (25 dispatching and one
returning as uncontrolled), but no test corrupts all six sequence vectors
plus all six adjusted Remaining destinations between calls.

The single uncontrolled return has no demand-state no-op oracle, and
coverage omits a `ZoneSizingCalc` return oracle, every Space traversal and
duplicate-membership case, nonzero moisture, every moisture sequence
ratio, and duct loss. Uniform has no zero-available
fallback case. PLR paths omit zero or wrong-sign capacity, `plr <= 0`,
zero total, NaN/Inf, inconsistent priority/capacity, and malformed active
counts. Sequential fraction coverage omits cooling, out-of-range,
negative, NaN, and schedule-failure fractions.

Tests also omit an invalid first-iteration silent return, a later invalid
fatal, allocated-zero-length or mismatched companion arrays, an active
missing-priority-scratch failure oracle, invalid Zone/Space identity,
isolated partial failure, rollback, and replay after changed scheme,
capacities, priorities, fractions, allocation, or Space topology. The one
unallocated uncontrolled call returns before sequence and priority
prerequisites are needed; without a demand sentinel it proves neither
no-op preservation nor active malformed-state behavior.

Gate returns occur before any wrapper-owned mutation and carry no result
status. For an active call, both Zone demand accessor arguments must be
evaluated before the Zone child begins, but their relative evaluation
order is not specified by the call syntax. A Zone-child failure prevents
the Space flag and membership traversal and retains the child's mutation
prefix. Failure while acquiring the membership after a completed Zone
child retains the complete Zone result. A Space accessor or lower-child
failure retains the Zone, every completed prior occurrence, and any
current-leaf prefix; later occurrences are skipped.

The first-iteration invalid-scheme path is silent. The corresponding later
path reaches CP266 `ShowFatalError`. CP265 has no catch, local diagnostic,
checkpoint, cleanup, recovery, or rollback around either case.

With scheme, priorities, capacities, fractions, demands, duct state, and
membership fixed, successful active replay is overwrite-idempotent for the
destinations CP266 rewrites. Sequential duct additions do not accumulate
because the child assigns before adding, and duplicate Space occurrences
repeat the same fixed calculation. This is not canonical whole-state
repair: wrapper skip paths, lower nonpositive-PLR no-write paths, and
sequence tails outside the active range retain history. Control and sizing
flags, first-iteration state, scheme, priorities, capacities, fractions,
demand, duct state, Space flag, and membership are all resampled on every
call.

Rust has no exact or snake-case CP265 wrapper or CP266 leaf. It lacks
source total/unadjusted/sequenced sensible and moisture state, Zone/Space
demand arenas, `PrioritySimOrder`, heating/cooling list priorities,
available-equipment counts, learned capacity caches, and operational
first-iteration, sizing, Space-HB, or duct-loss state.

Rust parses all four load-distribution enums plus heating/cooling sequences
and optional Sequential fraction schedules. The active runtime reads none
of the schemes or fraction schedules. Sequence values serve static graph
validation and binding order, while runtime visits each independently
bound IdealLoads system with a fresh four-setpoint-value demand snapshot.
Per-system maximum capacity limits are component caps, not the source
equipment-list learned capacities used by PLR distribution.

Active data contain 30 equipment lists, 30 connections, and 30 IdealLoads
systems. Every list has one SequentialLoad entry at heating/cooling
sequence `1/1` with blank fraction schedules. There are zero Space or
SpaceList objects, multi-equipment lists, non-Sequential schemes, active
fraction schedules, Sizing:Zone objects, or duct-loss cases. All 61 active
SimulationControl objects disable Zone sizing. These fixtures expose only
the trivial single-equipment topology and cannot establish CP265 parity.

The roadmap still requires real `FirstHVACIteration` semantics, multiple
ZoneHVAC equipment load distribution, equipment-list order and sequences,
availability, residual-load updates, and shared adaptive system-timestep
state. CP265 is source-only dependency evidence for that work, not an
implementation checkpoint.

CP265 changes no Rust target/state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 269 routines, split 58
`state_mapped` plus 211 `source_mapped`, with 146 required.
Domain-required counts become heat-balance 88, HVAC 35, plant 1, and
time/schedule 22, with readiness `0/88`, `0/35`, `0/1`, and `0/22`. The
IdealLoads parent remains `scaffold` at claim level `none`.

## CP266 `distributeOutputRequired` Equipment Load Distribution Leaf

CP266 adds canonical required `routine.distribute_output_required`
immediately after `routine.distribute_system_output_required` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. Lowercase `distributeOutputRequired` is declared at
`ZoneEquipmentManager.hh` lines 200-203 and implemented completely at
`ZoneEquipmentManager.cc` lines 4421-4715:

```cpp
void distributeOutputRequired(
    EnergyPlusData &state,
    int const ZoneNum,
    DataZoneEnergyDemands::ZoneSystemSensibleDemand &energy,
    DataZoneEnergyDemands::ZoneSystemMoistureDemand &moisture);
```

There is no default argument. `state`, `energy`, and `moisture` are mutable
references; `ZoneNum` is a const by-value selector. The leaf receives
neither `FirstHVACIteration` nor a Space identity. CP265 has already
selected whether to call it and may pass either a Zone demand pair or a
Space demand pair, but CP266 always reads equipment-list context through
the unchanged parent `ZoneNum`.

The leaf first binds `ZoneEquipList(ZoneNum)` and switches on its
`LoadDistScheme`. For the formulas below, define:

```text
Q, QH, QC = sensible predictor total, heating-setpoint, cooling-setpoint
W, WH, WC = moisture predictor total, humidifying-setpoint,
            dehumidifying-setpoint
N         = NumOfEquipTypes
E         = max(N, 0), the number of loop iterations
D         = duct-loss flag
SS, SL    = manager-global sensible and latent system duct loss
```

CP266 directly mutates only 12 persistent demand families: the three
sensible and three moisture sequence vectors, plus the six adjusted
`Remaining*` scalars corresponding to their slot-one values. It does not
write predictor totals, unadjusted Remaining demand, deadband/setback
state, list priorities, available counts, learned capacities, fraction
schedules, or manager-global priority scratch.

The exact four scheme branches are Sequential, Uniform, UniformPLR, and
SequentialUniformPLR.

### Sequential

Sequential ignores `N`, available-equipment counts, list priority values,
and learned capacities. It reads
`PrioritySimOrder(1).EquipPtr`, then unconditionally evaluates both that
equipment's heating and cooling fraction schedule getters before choosing
one result:

```text
rh = SequentialHeatingFraction(state, EquipPtr)
rc = SequentialCoolingFraction(state, EquipPtr)
r  = Q >= 0 ? rh : rc
```

Both schedule pointers therefore must be valid even though only one
sampled value is used. Positive zero and negative zero select heating;
NaN makes the comparison false and selects cooling. No finite, sign,
range, or clamp check is applied to either fraction.

The branch first assigns sensible slot one from the chosen raw fraction,
then conditionally adds sensible duct loss with the same fraction:

```text
energy.total[1] = r*Q
energy.heat[1]  = r*QH
energy.cool[1]  = r*QC

if D:
    energy.total[1] += r*SS
    energy.heat[1]  += r*SS
    energy.cool[1]  += r*SS
```

It copies those three values to the adjusted sensible Remaining fields,
then performs the analogous moisture writes and copies:

```text
moisture.total[1] = r*W
moisture.humid[1] = r*WH
moisture.dehum[1] = r*WC

if D:
    moisture.total[1] += r*SL
    moisture.humid[1] += r*SL
    moisture.dehum[1] += r*SL
```

Equivalently, each final slot-one value is
`r*(predictor + D*duct_loss)`. Assignment precedes addition, so a fixed
successful replay does not accumulate duct loss. Exactly the six
slot-one sequences and six adjusted Remaining fields are touched; every
higher sequence slot survives. The dynamic persistent mutation count is
`12 + 6D`.

### Uniform

Uniform computes both ratios before selecting a load sign:

```text
rh = NumAvailHeatEquip > 0 ? 1 / NumAvailHeatEquip : 1
rc = NumAvailCoolEquip > 0 ? 1 / NumAvailCoolEquip : 1
```

For every raw equipment index `1..N`, nonnegative `Q` selects the heating
priority and `rh`; negative or NaN `Q` selects the cooling priority and
`rc`. A selected priority greater than zero receives the corresponding
ratio times each of `Q`, `QH`, `QC`, `W`, `WH`, and `WC`. A nonpositive
priority receives six zeroes. The available count is not checked against
the number of positive priorities, so zero or negative counts use ratio
one and inconsistent positive counts simply over- or under-distribute.

Uniform does not read capacity, fractions, priority scratch, or duct
loss. After the loop, the shared non-Sequential tail copies the six
slot-one sequences into adjusted Remaining. Thus its dynamic persistent
mutation count is `6E + 6`. When `N <= 0`, the loop is empty but the common
tail still requires slot one in all six vectors.

### UniformPLR

UniformPLR uses the sign of sensible total demand only. For nonnegative
`Q`, it sums `HeatingCapacity(i)` for entries whose heating priority is
positive; for negative or NaN `Q`, it sums `CoolingCapacity(i)` for
entries whose cooling priority is positive:

```text
A = sum(active signed capacity)
p = Q/A
```

The division occurs only when the aggregate has the expected sign:
`A > 0` for heating and `A < 0` for cooling. Otherwise `p` remains zero.
There is no per-slot capacity-sign validation, finite check, or clamp to
one.

When `p <= 0`, the branch breaks from the switch without changing any
sequence. The common non-Sequential tail still copies the historical
slot-one values into all six adjusted Remaining fields, so the source
comment's no-change behavior applies only to sequences. This path performs
six persistent writes. A NaN `p` does not satisfy `p <= 0` and therefore
takes the full-write path.

On the full path, every index `1..N` is written. A nonpositive selected
priority gets six zeroes. For an active entry with selected capacity `C`,
all three sensible sequences receive the same value:

```text
energy.total[i] = C*p
energy.heat[i]  = C*p
energy.cool[i]  = C*p
```

The moisture direction follows the sensible sign rather than the moisture
load sign. In heating mode:

```text
if QH != 0:
    moisture.total[i] = W  * C*p/QH
    moisture.humid[i] = WH * C*p/QH
else:
    moisture.total[i] = W*p
    moisture.humid[i] = WH*p
moisture.dehum[i] = 0
```

Cooling is symmetric with denominator `QC`, values `W` and `WC`, and
`moisture.humid[i] = 0`. Only exact zero selects the fallback; NaN or
infinite denominators enter ordinary division. The full dynamic mutation
count, including the common slot-one copy, is `6E + 6`.

`Q=+0` or `Q=-0` selects heating but yields `p=0`, so sequences remain
unchanged and only the common tail writes. `Q=NaN` selects cooling. If the
cooling aggregate is negative, `p` becomes NaN and the full path can
write NaNs; if the aggregate fails its sign gate, `p=0` and only the
common tail runs.

### SequentialUniformPLR

SequentialUniformPLR also selects heating for nonnegative `Q` and cooling
otherwise, then scans every raw index `1..N` without early termination.
A heating entry is a candidate when `HeatingCapacity(i) > 0 && A < Q`; a
cooling entry is a candidate when
`CoolingCapacity(i) < 0 && A > Q`.

For every candidate, capacity contributes to `A` only when the matching
priority is positive, but `numOperating` increments regardless of that
priority. Consequently the count is neither necessarily the number of
capacity contributors nor the last candidate's raw index. The subsequent
distribution does not replay the candidate positions: it treats the raw
prefix `1..numOperating` as operating. Wrong-sign or inactive entries can
therefore make the scan set differ from the distributed prefix.

If the final aggregate has the expected sign, `p=Q/A`; otherwise
`p=0` and `numOperating=0`. There is again no clamp. When `p <= 0`, no
sequence changes and the common tail still performs six Remaining writes.
On the full path, the raw operating prefix uses exactly the UniformPLR
energy and moisture formulas, including its active-priority test and
exact-zero denominator fallback. Every index after the prefix through
`N` is explicitly zeroed in all six sequences. The scan visits `E`
indices and the distribution-plus-zero pass visits another `E`; the
full dynamic persistent mutation count remains `6E + 6`.

For signed zero, the heating threshold `A < Q` is initially false and the
branch takes its sequence no-write path. For NaN, the cooling threshold
`A > Q` is always false, so it also takes the no-write path. This differs
from UniformPLR's possible full NaN write. SequentialUniformPLR reads
neither available counts, fraction schedules, priority scratch, nor duct
loss.

### Fatal default and shared tail

The default calls:

```cpp
ShowFatalError(
    state,
    "DistributeSystemOutputRequired: Illegal load distribution scheme type.");
```

Under normal fatal semantics it terminates before demand mutation. The
following `break` is nominally unreachable; if the fatal helper ever
returned, execution would continue to the shared tail.

Sequential performs its six Remaining copies inside its case and skips
the shared tail. Every other case, including either PLR sequence no-write
path, reaches a six-assignment tail in this exact order: sensible total,
moisture total, sensible heating, moisture humidifying, sensible cooling,
and moisture dehumidifying. Each value is read from sequence slot one.
No unadjusted field is changed.

The complete leaf has 32 `if` tokens, 21 `else` tokens and no
`else if`, eight `for` loops, one switch, four cases, one default, seven
breaks, no return, two `&&` tokens, and one ternary. Its 136 plain
assignment tokens divide into 104 direct persistent writes and 32 local
writes. Ten `+=` tokens divide into six persistent Sequential duct
additions and four local capacity sums; ten `++` tokens are local loop or
operating-count increments.

There are 110 direct persistent mutation sites across the 12 destination
families: Sequential contributes 12 assignments plus six additions,
Uniform contributes 24 assignments, UniformPLR contributes 28,
SequentialUniformPLR contributes 34, and the shared tail contributes six.
The 151 syntactic calls/accessors comprise 110 sequence-vector accesses,
26 capacity accesses, ten priority accesses, two fraction getters, and one
each for `ZoneEquipList`, `PrioritySimOrder`, and `ShowFatalError`.

There is no up-front validation that:

- `ZoneNum` owns a list;
- priority and capacity arrays cover `1..N`;
- the six sequence vectors have mutually compatible extents;
- every non-Sequential vector owns slot one even when `N <= 0`;
- Sequential priority scratch slot one belongs to this Zone and holds a
  valid equipment pointer;
- both Sequential fraction-schedule arrays cover the equipment pointer and
  both referenced schedule pointers are valid;
- available counts match active priorities; or
- capacity signs, totals, ratios, and schedule values are finite.

Depending on build and container behavior, a malformed access can assert,
throw, or become undefined behavior. CP266 has no result status, catch,
local diagnostic except the fatal default, checkpoint, cleanup,
transaction, rollback, or recovery.

The only production lowercase call sites are CP265's Zone call at
`ZoneEquipmentManager.cc` lines 4408-4409 and stored-Space call at
lines 4413-4416. No C++ unit expression calls lowercase
`distributeOutputRequired` directly. A Space execution receives its own
mutable demand pair but still uses the parent Zone's list, priority arrays,
learned capacities, available counts, fraction schedules, and the same
manager-global priority scratch and duct-loss state. Demand identity and
`ZoneNum` correspondence are unchecked, and duplicate Space occurrences
can overwrite the same record repeatedly.

Failure preserves the exact prefix already written. A list lookup failure
precedes all persistent mutation. Sequential scratch or fraction sampling
also precedes mutation, but a later vector access can preserve a prefix of
sensible base assignments, sensible duct additions, sensible Remaining
copies, moisture base assignments, moisture duct additions, and moisture
Remaining copies. Uniform can retain all prior equipment plus the current
equipment's energy-then-moisture prefix. PLR capacity-scan failure occurs
before sequence writes; its sequence no-write path can still fail partway
through the six shared-tail copies. A PLR full-path failure preserves all
prior equipment and a current assignment prefix. SequentialUniformPLR can
also preserve a completed operating prefix followed by only part of the
zeroed tail. CP265 adds the already-completed Zone and prior-Space prefixes
around those leaf-local effects.

With every mutable dependency fixed, a successful full-write replay is
overwrite-idempotent on the destinations it rewrites. Sequential duct
loss does not accumulate because base assignment precedes addition. This
is not canonical whole-state repair: a PLR nonpositive path preserves all
old sequences, Sequential preserves every slot above one, and other
full-write paths preserve slots above `N`. Scheme, sign, totals, priorities,
available counts, learned capacities, fraction current values, priority
scratch, duct state, extents, and parent Zone/Space membership are
resampled on every parent call.

The audited C++ corpus executes CP266 exactly 83 times: 20 leaves from 24
explicit CP263 calls after four first-PLR gates, 13 direct-public
distributing replays, and 50 named-parent leaves from 51 public executions
after one uncontrolled return. Although the leaf receives no iteration
flag, its parent context divides those calls into 46 first and 37 later
iterations.

The scheme census is 59 Sequential, eight Uniform, four UniformPLR, and
12 SequentialUniformPLR. Total sensible signs are 27 positive, 44
negative, and 12 exact zero. By scheme they are:

- Sequential: 15 positive, 32 negative, and 12 zero;
- Uniform: four positive and four negative;
- UniformPLR: two positive and two negative; and
- SequentialUniformPLR: six positive and six negative.

All 83 calls use Zone demand records, `doSpaceHeatBalance=false`, valid
list/scratch prerequisites, duct loss disabled, and zero moisture
predictors. Their compatible sequence/list shapes are 48 one-entry, two
two-entry, 28 three-entry, and five four-entry records. There is no
unallocated, empty, mismatched, malformed, uncontrolled, or active Space
leaf execution.

Of the 59 Sequential leaves, 57 sample a selected fraction of one. Exactly
two positive leaves in the mixed-equipment fraction scenario select the
first equipment's heating fraction 0.4. A configured cooling fraction 0.3
is never selected, while the second equipment's heating fraction 0.6 is
consumed later by CP267 rather than CP266. Duct additions never execute.

All eight Uniform leaves have `N=3`. Four positive calls divide by three
available heating entries and write all three slots. Four negative calls
divide by two available cooling entries, write slots one and two, and
zero inactive slot three. Both positive-count ratio branches execute on
every call; neither ratio-one fallback does. Across 24 loop iterations,
12 are active heating, eight active cooling, and four inactive cooling.

The four later UniformPLR leaves also have `N=3` and all compute a
positive PLR. Heating runs twice with capacities `[2000, 1000, 500]`,
aggregate 3500, load 1000, and `p=2/7`. Cooling runs twice with capacities
`[-1200, -800, -500]`, only the first two priorities active, aggregate
-2000, load -1000, and `p=0.5`; the third slot is zeroed. The 12
assignment iterations comprise six active heating, four active cooling,
and two inactive cooling. No aggregate-sign failure or sequence no-write
path executes.

The 12 later SequentialUniformPLR leaves cover each of six scenarios
twice:

```text
Q =  1000: numOperating=1, A= 2000, p=0.5
Q =  2100: numOperating=2, A= 3000, p=0.7
Q =  3600: numOperating=3, A= 3500, p=36/35
Q = -1000: numOperating=1, A=-1200, p=5/6
Q = -1500: numOperating=2, A=-2000, p=0.75
Q = -2500: numOperating=3, A=-2000, p=1.25
```

The final negative scenario increments `numOperating` for the third
priority-zero candidate without adding its capacity, then processes that
raw third prefix slot as inactive and writes zero. It is direct evidence
for the unconditional operating-count increment. Across all 36 scans,
the operating loops perform 24 iterations, 22 active and two inactive,
and the remaining-tail loops perform 12 iterations. One-, two-, and
three-unit operating counts each occur four times. Four executions exceed
PLR one and confirm the absence of a clamp; every execution still avoids
the nonpositive-PLR branch.

The six explicit distribution blocks contain 222 sensible sequence
assertions, but only 186 follow an actual CP266 leaf: 78 Sequential, 36
Uniform, 18 later-UniformPLR, and 54 later-SequentialUniformPLR. The other
36 assert CP264 design seeds after first-PLR CP265 gate returns. Each of
the three sensible sequence families therefore has 62 post-CP266
assertions.

There are 48 assertions over the three adjusted sensible Remaining
fields. Twelve non-Sequential scenarios directly reflect CP266 slot one;
two fraction-one Sequential later cases are indistinguishable from the
CP264 value; and two mixed later cases are observed only after CP267 has
already updated the residual. No test asserts a moisture sequence.
Exactly three adjusted moisture Remaining assertions exist, all zero
after later cooling UniformPLR calls with zero moisture inputs.

The 50 named-parent leaves have no immediate CP266 destination assertion.
Thirteen direct distributing calls and 25 leaving-condition calls form 38
actual distributing replays, but none corrupts all 12 destinations
between invocations. The assertions therefore demonstrate many successful
aggregate formulas without isolating rollback, canonical repair, or
history-dependent tails.

Coverage omits the fatal default, active malformed identity or extent,
zero equipment, allocated-empty vectors, companion-vector mismatch,
priority/capacity mismatch, aliasing, partial failure, and rollback. It
also omits every Space leaf, duplicate or cross-Zone Space membership,
partial Zone/Space failure, duct loss, and meaningful moisture.

Uniform never exercises a nonpositive available-count fallback. PLR tests
omit zero or wrong-sign aggregate capacity, a nonpositive PLR, NaN or
infinite load, wrong-sign or extreme per-slot capacity, exact-zero
sensible-setpoint denominator fallback,
inconsistent active count, and capacity mutation between replays.
Sequential tests omit negative, out-of-range, or NaN fractions, selected
cooling fraction, either schedule failure, and fraction-scaled duct loss.
The 12 Sequential exact-zero leaves select heating, including two
UnitHeater contexts with nonzero setpoint loads, but no direct CP266 demand
oracle isolates that edge.

Tests also omit dependency mutation between retries, failure retry,
sentinel preservation on every no-write branch, and canonical repair of
retained sequence tails. The lower leaf's absence of a
`FirstHVACIteration` parameter means the first/later census proves only
parent-selected entry context, not a CP266-owned iteration decision.

Rust has no exact or snake-case CP266 leaf. Its by-value
`ZoneSysEnergyDemand` snapshot contains only four combined sensible and
moisture setpoint Remaining scalars. It owns no source-shaped Zone/Space sensible or
moisture arena, predictor totals, unadjusted or total adjusted Remaining
fields, six sequence vectors, manager-global priority scratch,
heating/cooling priority arrays, available-equipment counts, signed
first-iteration learned capacity caches, or duct-loss state.

The adjacent third-order moisture predictor can compute a transient
moisture total, but closed-loop and CLI paths copy only humidifying and
dehumidifying setpoint loads into the PurchasedAir snapshot. That value
is neither a persistent source-shaped moisture demand record nor CP266
distribution state.

Rust parses all four load-distribution enums and both optional Sequential
fraction schedule identifiers, but runtime reads neither the scheme nor
the fractions. Non-Sequential variants stop at parser arms. The compiler
requires positive unique heating/cooling sequence numbers, whereas source
CP266 treats nonpositive priorities as inactive and does not reject
duplicates. Graph helpers use static heating-first order, and active
compatibility execution visits each typed IdealLoads system with a fresh
full demand snapshot rather than shared residual demand. A multi-equipment
list is only marked diagnostic-only and remains dispatchable, so each
system would independently receive the full snapshot. Component maximum
capacities are authored caps, not the source list's signed learned
capacity arrays.

The active corpus remains 30 equipment lists, 30 connections, and 30
IdealLoads systems. Every list has one `SequentialLoad` entry at
heating/cooling sequence `1/1` with blank fraction schedules. It contains
zero Space, SpaceList, SpaceHVAC, multi-equipment, non-Sequential,
active-fraction, `Sizing:Zone`, or `Duct:Loss:*` cases. All 61
SimulationControl objects disable Zone sizing. This topology would reduce
source CP266 to a one-slot, fraction-one, duct-off Sequential transition,
but Rust still owns neither that slot nor its total/remaining distribution
state.

The roadmap still requires Rust-owned `ZoneSysEnergyDemand`, removal of
oracle demand injection, real first-iteration and adaptive system-timestep
state, multiple-equipment distribution, equipment-list order and
availability, residual-load updates, and shared lifecycle state. CP266 is
source-only dependency evidence for that work, not an implementation
checkpoint.

CP266 changes no Rust target/state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim,
or conformance status. Counts become 32 algorithms and 270 routines,
split 58 `state_mapped` plus 212 `source_mapped`, with 147 required.
Domain-required counts become heat-balance 88, HVAC 36, plant 1, and
time/schedule 22, with readiness `0/88`, `0/36`, `0/1`, and `0/22`. The
IdealLoads parent remains `scaffold` at claim level `none`.

## CP267 `updateSystemOutputRequired` System Residual Update Leaf

CP267 adds canonical required
`routine.update_system_output_required` immediately after
`routine.distribute_output_required` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. The leaf is declared at `ZoneEquipmentManager.hh` lines 205-212 and
implemented completely at `ZoneEquipmentManager.cc` lines 4717-4908:

```cpp
void updateSystemOutputRequired(
    EnergyPlusData &state,
    int const ZoneNum,
    Real64 const SysOutputProvided,
    Real64 const LatOutputProvided,
    DataZoneEnergyDemands::ZoneSystemSensibleDemand &energy,
    DataZoneEnergyDemands::ZoneSystemMoistureDemand &moisture,
    int const EquipPriorityNum = -1);
```

The default appears only in the header. There is no `FirstHVACIteration`,
Space identity, sizing, or duct-loss argument. The mutable demand references
may designate either Zone or Space records, while `ZoneNum` always selects
the control type and, for a controlled call, the parent Zone equipment-list
context.

Let

```text
S, L       = provided sensible and latent output
U, UH, UC  = sensible unadjusted total, heating-SP, and cooling-SP residuals
R, RH, RC  = corresponding sensible adjusted residuals
V, VH, VD  = moisture unadjusted total, humidifying-SP, and dehumidifying-SP residuals
M, MH, MD  = corresponding moisture adjusted residuals
P          = EquipPriorityNum
Z          = energy.NumZoneEquipment
N          = selected Zone equipment list NumOfEquipTypes
q          = P + 1
HP(P),CP(P)= manager-global heating and cooling priorities at scratch slot P
```

For an uncontrolled Zone, the routine applies these mutations in exact
source order:

```text
U  -= S; R  = U
UH -= S; RH = UH
UC -= S; RC = UC
V  -= L; M  = V
VH -= L; MH = VH
VD -= L; MD = VD
```

It then recomputes the parent Zone's `CurDeadBandOrSetback` according to
`TempControlType(ZoneNum)`:

- Uncontrolled writes false;
- SingleHeat writes `R < 1.0` through the source expression
  `(R - 1.0) < 0.0`;
- SingleCool writes `R > -1.0` through `(R + 1.0) > 0.0`;
- SingleHeatCool and DualHeatCool write `RH < 0.0 && RC > 0.0`; and
- an unknown control type retains the prior flag.

All comparisons are strict, so the SingleHeat boundary `R == 1.0`, the
SingleCool boundary `R == -1.0`, and either dual-setpoint zero boundary are
outside deadband. A NaN comparison is false; the dual expression also
retains C++ short-circuit order.

Only when `P >= 0`, the uncontrolled path attempts three independently
gated sequence-pair writes. The total sensible/moisture pair uses slot
`q` when `q <= Z`. The heating/humidifying pair uses slot `HP(P)+1` when
that value is at most `Z`, and the cooling/dehumidifying pair analogously
uses `CP(P)+1`. The sole `return` follows these writes. The gates impose no
lower bound, scratch bound, vector-extent check, or moisture-side equipment
count check.

For a controlled Zone with Sequential distribution, the leaf first
subtracts `S` from all three sensible unadjusted residuals and `L` from all
three moisture unadjusted residuals. If

```text
P >= 0 && P < N
```

it treats `q=P+1` as the next priority slot, reads
`PrioritySimOrder(q).EquipPtr`, and lazily evaluates exactly one fraction:

```text
r = energy.TotalOutputRequired >= 0.0
      ? SequentialHeatingFraction(state, nextSystem)
      : SequentialCoolingFraction(state, nextSystem)
```

Unlike CP266, the unselected getter is not called. The discriminator is
the original predictor total, not any updated residual or provided output.
Positive and negative zero select heating, while NaN selects cooling. The
raw fraction is neither clamped nor checked for finiteness, range, schedule
validity, or consistency with the selected equipment.

The valid-next branch writes

```text
R  = r*U;  RH = r*UH; RC = r*UC
M  = r*V;  MH = r*VH; MD = r*VD
```

and then copies those six adjusted values into the six sensible/moisture
sequence families at slot `q`. It does not check `q` against `Z` or any
sequence extent. If the valid-next predicate is false, it instead copies
the six updated unadjusted residuals directly into their adjusted partners
and writes no sequence slot. Both Sequential paths then run the same
thermostat/deadband switch described above.

Sequential ignores duct-loss state, available-equipment counts, learned
capacities, list priority values other than the manager scratch lookup,
and every nonselected fraction. Its next-equipment test uses only raw `N`;
it does not establish allocation, extent, or identity agreement among the
six sequence vectors, `energy`, `moisture`, the selected Zone list, and the
manager-global scratch arena.

The three controlled non-Sequential schemes—Uniform, UniformPLR, and
SequentialUniformPLR—share one body. They ignore `S` and `L` completely.
When `P < 0`, the body is a no-op. Otherwise it independently copies at
most three sequence pairs into the six adjusted residuals:

```text
q <= Z:
    R = sensible total sequence(q)
    M = moisture total sequence(q)
HP(P)+1 <= Z:
    RH = sensible heating sequence(HP(P)+1)
    MH = moisture humidifying sequence(HP(P)+1)
CP(P)+1 <= Z:
    RC = sensible cooling sequence(CP(P)+1)
    MD = moisture dehumidifying sequence(CP(P)+1)
```

This body does not mutate unadjusted residuals, sequence vectors, or the
deadband flag. A skipped pair retains its historical adjusted values. It
again uses only upper-bound tests: `P=0` can complete the total slot-one
pair and then fail at scratch slot zero, a negative priority plus one can
pass the upper check, and signed `+1` overflow is not guarded.

Any other controlled load-distribution value calls

```cpp
ShowFatalError(
    state,
    "UpdateSystemOutputRequired: Illegal load distribution scheme type.");
```

before a direct demand mutation. Under the fatal helper contract, no
mutation follows.
NaN or infinity in `S` or `L` propagates through the subtracting
uncontrolled and Sequential paths but is ignored by the controlled
non-Sequential body. A selected NaN or infinite fraction propagates through
all six valid-next products. The routine performs no division. Deadband
comparisons against NaN produce false results; an unknown thermostat type
preserves history rather than normalizing it.

With `d=1` for a recognized thermostat case and zero otherwise, and with
`t`, `h`, and `c` denoting successful total, heating, and cooling pair
gates, the successful dynamic direct-mutation counts are

```text
uncontrolled:                 12 + d + 2(t+h+c)
controlled Sequential next:  18 + d
controlled Sequential tail:  12 + d
controlled non-Sequential:    2(t+h+c)
```

The static body contains 58 direct persistent mutation sites over 19
families:

- 12 sites for six unadjusted residual families, each appearing in the
  uncontrolled and controlled-Sequential branches;
- 24 sites for six adjusted residual families, each appearing in four
  branch locations;
- 12 sites for six sequence families, each appearing in two branch
  locations; and
- ten deadband assignments across the two thermostat switches.

Those sites divide into 12 compound subtractions and 46 plain assignments.
By branch location, 23 are uncontrolled, 29 are controlled Sequential, and
six are in the shared non-Sequential body. The complete body has ten `if`
tokens, one `else`, three switches, 14 cases, three defaults, 15 breaks,
one return, one ternary, five `&&` tokens, no `||`, no loop, and one unary
`!`. Its 49 plain `=` tokens comprise the 46 persistent assignments and
three local initializations.

Under the established audit convention that counts Objexx indexing as a
syntactic accessor, the body has 48 calls/accessors: one Zone lookup, two
temperature-control lookups, ten deadband accesses, one Zone equipment-list
lookup, 13 priority-scratch accesses, two fraction getters, 18 sequence
vector accesses, and one fatal call.

The leaf assumes a valid `ZoneNum`, a matching controlled equipment list,
valid `P`/`q` scratch positions, a valid next `EquipPtr`, a valid selected
fraction array and schedule value, and independently valid indices in all
six sequence vectors. `energy.NumZoneEquipment` is the only count used to
authorize moisture-vector access. Controlled Sequential instead trusts
list `N` to authorize all six slot-`q` writes and never compares it with
`Z`. The demand references may belong to a different Zone or Space, and a
Space call deliberately reuses its parent Zone's control type, equipment
list, priority scratch, and deadband destination.

There is no result status, local validation, catch, checkpoint, cleanup,
transaction, or rollback. A Zone lookup failure precedes all work. On the
uncontrolled path, a thermostat lookup or switch failure follows the 12
ordered residual mutations. Later failures can retain that prefix, a
new deadband value, and completed total, heating, then cooling sequence
pairs. `P=0` can write the total pair before failing on scratch slot zero.

For a controlled call, the list lookup precedes mutation. A Sequential
scratch or fraction failure follows the six unadjusted subtractions;
adjusted and sequence failures retain their source-ordered prefixes, and
the thermostat/deadband work occurs last. The shared non-Sequential body
can retain the total pair, then heating pair, then cooling-pair prefix.
The invalid-scheme fatal performs no direct demand write.

Uncontrolled and controlled Sequential calls are intrinsically
non-idempotent because they subtract the provided output on every replay.
For fixed `S` and `L`, after `k` successful subtracting calls,

```text
U_k = U_0 - k*S
V_k = V_0 - k*L
```

with analogous setpoint fields. Retrying after a partial failure therefore
cannot reconstruct the intended one-call state without an external reset.
The non-Sequential branch is overwrite-idempotent only for the adjusted
pairs whose fixed gates succeed; skipped fields retain history, and it
never repairs unadjusted demand, sequences, or deadband state. Every replay
can resample scheme, list counts, scratch identities and priorities,
fraction values, predictor sign, provided outputs, sequence contents and
extents, and Zone/Space membership.

There are four direct production call expressions:

- `sizeZoneSpaceEquipmentPart1` calls the leaf after each optional DOAS
  prefix at `ZoneEquipmentManager.cc` line 404, using the default `P=-1`;
- the same sizing helper calls it at its final tail at line 596, again with
  default priority;
- `SimZoneEquipment` calls it after each dispatched equipment slot at lines
  4108-4114 with explicit priority; and
- `ZoneEquipmentSplitter::distributeOutput` calls it for each Space at
  `DataZoneEquipment.cc` lines 2224-2230 with the parent Zone number,
  Space demand references, and explicit priority.

The splitter call can overwrite the parent Zone's
`CurDeadBandOrSetback(ZoneNum)` from a Space residual. It does not verify
that the Space belongs to that Zone or that the parent scratch/list state
matches the supplied demand records. Repeated Space occurrences repeat the
same cumulative subtraction.
The bounded, statically attributable C++ unit corpus executes the leaf 80
times. This count excludes unbounded repeated passes hidden inside complete
`ManageSimulation` runs:

- 65 calls follow individual `SimZoneEquipment` Zone slots;
- two tests call the lowercase leaf directly;
- three calls come from the splitter for Space demand; and
- ten sizing calls comprise seven final-tail calls plus three DOAS-prefix
  calls.

All ten sizing calls see an uncontrolled Zone and default `P=-1`. The other
70 calls are controlled Sequential calls with explicit priority. Of those,
18 take the valid-next branch—16 equipment-slot calls plus both direct
calls—and 52 take the fallback—49 last equipment slots plus the three
splitter calls whose selected list count is zero. No controlled Uniform,
UniformPLR, SequentialUniformPLR, or invalid-default execution occurs.
With recognized thermostat cases, the corpus therefore performs exactly

```text
10*13 + 18*19 + 52*13 = 1148
```

direct mutation-statement executions in this leaf.

The two direct unit calls are positive-heating, `P=1`, `N=4` cases. One
selects fraction one and the mixed-fraction case selects heating fraction
0.6. Together they make 18 sensible sequence assertions and six adjusted
sensible `Remaining*` assertions. Only six sequence assertions target the
three slot-two values freshly written by the two CP267 calls; the other 12
prove retention of other slots. There is no direct assertion for any
unadjusted field, moisture sequence or adjusted moisture field, or
`CurDeadBandOrSetback`.

The three splitter executions subtract nonzero Space sensible outputs but
assert no Space demand destination. Sizing tests inspect downstream sizing
results rather than the immediate residual fields. The named-parent
UnitHeater case adds one final sensible `RemainingOutputRequired == 0`
observation after two equipment slots, but it does not isolate CP267 from
its callers and equipment calculations.

Coverage therefore omits every non-Sequential branch, the invalid-scheme
fatal, unknown thermostat retention, direct deadband boundaries at `R=1`,
`R=-1`, and dual zero, meaningful latent output assertions, cooling
fraction selection, negative/out-of-range/NaN/infinite fractions, and
NaN/infinite provided output. It also omits `P=0`, `P<-1`, oversized or
overflowing priority, negative scratch priorities, mismatched Zone/demand
identity, allocated-empty or inconsistent sequence extents, Space-demand
destination assertions, partial failure, rollback, and a direct cumulative
replay/drift oracle.

Rust has no exact or snake-case CP267 function. Its copied
`ZoneSysEnergyDemand` snapshot contains a Zone identity plus only four
heating, cooling, humidifying, and dehumidifying setpoint-remaining values.
It owns no total or unadjusted demand, six sequence vectors,
`NumZoneEquipment`, `PrioritySimOrder`, temperature-control array, shared
`CurDeadBandOrSetback`, or mutable Zone/Space demand arenas.

The compatibility runtime constructs a fresh complete demand snapshot for
each compiled IdealLoads system from fixed run options, passes that snapshot
by value, and discards it after dispatch. Equipment sensible and latent
outputs are never subtracted from a shared residual and never feed the next
system. Humidistat logic changes only a local copied moisture snapshot; its
local deadband enum is not the source's shared Zone deadband flag.

Rust parses all four load-distribution enums, positive unique heating and
cooling sequence numbers, and optional heating/cooling fraction schedule
identities, but runtime consumes none of those distribution or fraction
fields. A multi-equipment topology is marked diagnostic-only yet remains
dispatchable, with every system receiving the same full input snapshot
rather than a sequenced residual. Rust's authored IdealLoads capacity limits
are not the source manager's signed learned capacity state.

The active fixture census remains 30 equipment lists, 30 equipment
connections, and 30 IdealLoads systems. Every list has one SequentialLoad
entry at heating/cooling sequence `1/1` with both fraction schedules blank.
There are no active Space, SpaceList, SpaceHVAC, multi-equipment,
non-Sequential, `Sizing:Zone`, or duct-loss objects, and all 61 active
SimulationControl records disable Zone sizing.

Even that one-equipment topology requires source CP267: the sole equipment
is the last Sequential slot, so the fallback subtracts `S` and `L`, copies
all six updated residuals, and recomputes deadband. Rust still omits that
transition. The roadmap continues to require Rust-owned Zone system demand,
removal of oracle demand injection, operational first-iteration and adaptive
system-timestep state, multiple-equipment distribution, list ordering and
availability, and residual-load feedback.

CP267 changes no Rust target or state, support declaration, test,
capability, output, comparator, case, manifest, numerical claim,
performance claim, or conformance status. Counts become 32 algorithms and
271 routines, split 58 `state_mapped` plus 213 `source_mapped`, with 148
required. Domain-required counts become heat-balance 88, HVAC 37, plant 1,
and time/schedule 22, with readiness `0/88`, `0/37`, `0/1`, and `0/22`.
The IdealLoads parent remains `scaffold` at claim level `none`.

## CP268 `adjustSystemOutputRequired` Zone/Sequence Ratio Leaf

CP268 adds canonical required
`routine.adjust_system_output_required` immediately after
`routine.update_system_output_required` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. The leaf is declared at `ZoneEquipmentManager.hh` lines 214-219 and
implemented completely at `ZoneEquipmentManager.cc` lines 4910-4931:

```cpp
void adjustSystemOutputRequired(
    Real64 const sensibleRatio,
    Real64 const latentRatio,
    DataZoneEnergyDemands::ZoneSystemSensibleDemand &energy,
    DataZoneEnergyDemands::ZoneSystemMoistureDemand &moisture,
    int const equipPriorityNum);
```

The header and definition agree. All three scalar inputs are top-level
`const` by value, while the two demand records are mutable lvalue
references. There is no `EnergyPlusData`, Zone or Space identity, default
argument, `FirstHVACIteration`, or `noexcept` qualifier.

Let `s=sensibleRatio`, `l=latentRatio`, and `p=equipPriorityNum`. The body
contains exactly these 12 mutations in source order:

```text
energy.RemainingOutputRequired                  *= s
energy.RemainingOutputReqToHeatSP               *= s
energy.RemainingOutputReqToCoolSP               *= s
moisture.RemainingOutputRequired                *= l
moisture.RemainingOutputReqToHumidSP            *= l
moisture.RemainingOutputReqToDehumidSP          *= l
energy.SequencedOutputRequired(p)                *= s
energy.SequencedOutputRequiredToHeatingSP(p)     *= s
energy.SequencedOutputRequiredToCoolingSP(p)     *= s
moisture.SequencedOutputRequired(p)              *= l
moisture.SequencedOutputRequiredToHumidSP(p)     *= l
moisture.SequencedOutputRequiredToDehumidSP(p)   *= l
```

Thus `s` scales three adjusted sensible residuals and the matching three
sensible sequence cells, while `l` independently scales the three adjusted
moisture residuals and matching moisture sequence cells. The routine does
not touch predictor totals, any unadjusted residual, another sequence slot,
`NumZoneEquipment`, list or priority scratch, capacities, fractions,
deadband state, node state, or a saved demand copy.

The static and successful dynamic mutation counts are both 12, spanning 12
families. Every mutation is `*=`; there is no plain assignment, other
compound assignment, increment, local variable, branch, switch, loop,
ternary, logical operator, break, or explicit return. Each compound
assignment reads and writes its destination once. Under the established
audit convention, the only six calls/accessors are the six sequence-vector
indexing expressions.

The leaf uses raw `p` for every sequence vector. It performs no `+1`
conversion and consults no equipment count. It assumes all six vectors are
allocated and independently contain the same index, and does not validate
the index lower or upper bound, vector extent agreement, demand ownership,
or Zone/Space identity. A nonpositive or oversized index is not rejected;
a zero ratio does not skip indexing.

Ratios are likewise used raw, without sign, range, or finiteness checks,
clamping, division, or diagnostics. A negative ratio reverses signs. A
finite value times positive or negative zero becomes signed zero; repeated
negative-zero scaling can alternate a zero sign. NaN propagates through
that ratio's six destinations. Infinity can produce signed infinity, while
zero times infinity can produce NaN. Finite multiplication preserves the
platform's ordinary rounding, overflow, and underflow behavior. Because
both ratios are captured by value, earlier demand mutations cannot change
the later multiplier.

There is no local validation, result status, diagnostic, catch, checkpoint,
cleanup, transaction, or rollback. All six adjusted scalar multiplications
complete before the first indexed sequence access. An invalid first
sequence access therefore leaves those six scalar mutations. A later
malformed vector retains the same scalar prefix plus every sequence
multiplication already completed in the documented order. Under ordinary
IEEE behavior, NaN and infinity propagate as values rather than failures;
a trap-enabled environment can expose an earlier numeric prefix.

For fixed ratios, a fixed priority index, and successful calls, replay
compounds rather than reconstructing state. In ideal real arithmetic, after `k` calls each
sensible destination is its initial value times `s^k`, and each moisture
destination is its initial value times `l^k`. A retry after partial failure
therefore scales the retained prefix again and cannot recover the intended
one-call state. Unity ratios and some zero/NaN fixed points are incidental
special cases, not a replay guarantee.
The only direct production call expression is in
`ZoneEquipmentSplitter::adjustLoads` at `DataZoneEquipment.cc` lines
2180-2184. The transitive production entry is the `SimZoneEquipment`
equipment loop at `ZoneEquipmentManager.cc` lines 3740-3743. It calls
`adjustLoads` only when Space heat-balance simulation is active, sizing is
false, and the current equipment owns a nonnegative splitter index. The
priority-loop value `EquipTypeNum` is passed unchanged as `p`.

The caller initializes `s=l=1` and selects its ratio protocol from the
splitter thermostat-control enum:

- Ideal returns before saving demand or calling CP268;
- SingleSpace, when its configured control-Space fraction is positive,
  independently sets each ratio to
  `(selected Space total Remaining / Zone total Remaining) / fraction`
  when the corresponding Zone total Remaining is nonzero;
- Maximum scans stored Spaces in order for the greatest strictly positive
  value of `max(setptLo-T1, T1-setptHi)`, then applies the same independent
  total-Remaining ratios when the winning index and fraction are positive;
  and
- an unknown/default enum retains unity ratios and still calls CP268.

Neither total-derived ratio distinguishes the setpoint residuals that it
subsequently scales. The caller does not clamp or validate finiteness,
ratio sign, or magnitude. SingleSpace trusts its control-space ordinal and
identity. Maximum's strict comparison retains the first winner and leaves
unity ratios when no Space has a positive deviation.

Immediately before the leaf, `adjustLoads` copies the complete Zone sensible
and moisture demand records into splitter save storage. A later successful
`distributeOutput` call restores those copies before each Space update for
non-Ideal control. That is an external caller protocol, not CP268 recovery:
`adjustLoads` has no catch, and a failure in this leaf leaves both the saved
copy and the torn live demand state.

The bounded C++ unit corpus executes CP268 exactly twice, both through
`SpaceHVACSplitterTest` in `ZoneEquipmentManager.unit.cc`. Its three direct
`adjustLoads` calls classify as follows:

- line 5057 uses Ideal control and returns above the leaf;
- line 5074 uses SingleSpace control with `p=1`, `s=-0.2`, and `l=1`; and
- line 5153 uses Maximum control with `p=1`, `s=-0.9`, and `l=1`.

For SingleSpace, the configured second splitter entry has fraction 0.5 and
refers to Space index 3, whose total sensible Remaining is `+10`, while the
Zone total is `-100`:

```text
s = 10 / (-100 * 0.5) = -0.2
```

The Zone moisture total is zero, so the guarded latent calculation leaves
`l=1`. This execution reverses the six sensible destination signs. Lines
5076-5081 assert all three adjusted sensible residuals and all three
sensible slot-one sequence values.

The intervening `distributeOutput` restores the saved original Zone demand
before each Space update, but it also passes each Space demand through
`updateSystemOutputRequired`. In this test the Zone equipment list retains
its default `NumOfEquipTypes=0`, every Space's unadjusted sensible fields
retain zero, `sysOutputProvided=-90`, and the Sequential valid-next guard
fails for `p=1`. The fallback therefore overwrites all three sensible
Remaining fields for Space indices 1, 3, and 2 with `+18`, `+45`, and `+27`,
respectively, from `0 - (-90 * fraction)`. Latent values remain zero because
`latOutputProvided=0`.

For Maximum, Space index 2 wins with `T1=16` and fraction 0.3. Its total
sensible Remaining is now `+27`, not the originally seeded `-40`, producing

```text
s = 27 / (-100 * 0.3) = -0.9
```

The Zone moisture denominator is still zero, so `l=1`. Lines 5154-5159
assert the same six sensible destinations after this second sign-reversing
scale.
Together the two calls execute 24 leaf mutation statements and have 12
immediate sensible assertions.

All moisture scalars and sequences remain zero and are unasserted. The six
Ideal-case assertions prove only the caller's early return. Six later
restoration assertions prove the surrounding `distributeOutput` protocol,
not CP268 replay. No named `SimZoneEquipment` unit path activates a
SpaceHVAC splitter, so there is no other statically attributable leaf
execution.

Coverage omits a direct lowercase call, an observable nonunit latent ratio,
priority other than one, multi-slot isolation, positive nonunit, zero, or
unity sensible scaling, signed zero, NaN, infinity, and malformed or
mismatched sequence
vectors. It also omits caller fallbacks for zero Zone demand, nonpositive
fraction, Maximum ties or no positive deviation, the invalid enum, partial
failure, rollback, and a fixed-ratio compounding replay oracle.
Rust has no exact or snake-case CP268 function. It has no SpaceHVAC
splitter, splitter thermostat-control enum, control-Space ratio protocol,
mutable Zone/Space demand arena, six sequence vectors, or equipment-priority
indexed demand state. Its copied `ZoneSysEnergyDemand` contains a Zone
identity plus only four heating, cooling, humidifying, and dehumidifying
setpoint-remaining values; it has neither total adjusted residuals nor the
six sequenced destinations that CP268 scales.

The compatibility runtime constructs a fresh demand snapshot for each
compiled IdealLoads system from fixed run options and passes it by value.
It neither scales a shared Zone demand before equipment dispatch nor
restores a splitter-owned snapshot afterward. Humidistat code mutates only
a local copied demand. Static sequence numbers and parsed distribution
metadata therefore do not supply this runtime transition.

The active fixture census remains 30 equipment lists, 30 equipment
connections, and 30 IdealLoads systems. Every list has one SequentialLoad
entry at heating/cooling sequence `1/1` with blank fraction schedules.
There are no active Space, SpaceList, SpaceHVAC, multi-equipment,
non-Sequential, `Sizing:Zone`, or duct-loss objects, and all 61 active
SimulationControl records disable Zone sizing. The production guard for
CP268 is therefore inactive throughout the Rust fixture lane.

The roadmap still requires Rust-owned Zone and Space system demand,
SpaceHVAC topology and thermostat-control semantics, operational equipment
priority and sequence state, shared residual-load mutation, and removal of
oracle demand injection. A four-value by-value snapshot cannot establish
parity for this 12-destination in-place ratio leaf.

CP268 changes no Rust target or state, support declaration, test,
capability, output, comparator, case, manifest, numerical claim,
performance claim, or conformance status. Counts become 32 algorithms and
272 routines, split 58 `state_mapped` plus 214 `source_mapped`, with 149
required. Domain-required counts become heat-balance 88, HVAC 38, plant 1,
and time/schedule 22, with readiness `0/88`, `0/38`, `0/1`, and `0/22`.
The IdealLoads parent remains `scaffold` at claim level `none`.

## CP269 `CalcZoneMassBalance` Iterative Zone/Air-Loop Flow Solver

CP269 adds canonical required `routine.calc_zone_mass_balance` immediately
after `routine.adjust_system_output_required` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
item. `CalcZoneMassBalance` is declared at `ZoneEquipmentManager.hh` line
221 and implemented completely at `ZoneEquipmentManager.cc` lines 4933-5283:

```cpp
void CalcZoneMassBalance(
    EnergyPlusData &state,
    bool FirstHVACIteration);
```

The definition adds top-level `const` to the by-value Boolean, which does not
change the C++ function type. There is no default argument, result status, or
`noexcept`. `state` supplies every mutable Zone, Space, node, air-loop, mixing,
infiltration, environment, sizing, and diagnostic dependency.

There are exactly two production call expressions:

- `SizeZoneEquipment` calls it unconditionally at
  `ZoneEquipmentManager.cc` line 675 with `true`, after the complete
  controlled-Zone and optional-Space Part1 sizing sweep and before
  `CalcZoneLeavingConditions(state, true)` at line 677 and every Part2 entry;
  and
- `SimZoneEquipment` calls it at line 4186 with its incoming
  `FirstHVACIteration`, after Zone exhaust controls and exhaust-system
  simulation at lines 4182-4184 and before leaving conditions, whole-system
  duct loss, and return-path simulation at lines 4188-4192.

`ManageZoneEquipment` selects sizing or simulation at lines 157-161. Neither
parent adds a mass-balance-specific guard, so sizing reaches CP269 even when
there is no controlled Zone.

The body defines `IterMax=25` and `ConvergenceTolerance=1.0e-5`. Its first
persistent write clears `ZoneMassBalanceHVACReSim`. Before entering the
iterative body it performs two complete source-ordered passes:

1. every `AirDistUnit` whose `AirLoopNum` is positive additively contributes
   `MassFlowRateSup` to `SupFlow`, `MassFlowRatePlenInd` to `RecircFlow`, and
   three leakage terms to `LeakFlow`; these targets are not cleared locally,
   and the source comment relies on prior `InitZoneEquipment`; then
2. every primary air loop marked `isAllOA` receives `MaxOutAir=SupFlow`,
   `OAFlow=SupFlow`, and `OAFrac=1`.

The `do/while` body at lines 4979-5277 executes the following ordered
building pass:

- When `EnforceZoneMassBalance` is true, it first clears each air loop's
  `ZoneRetFlow`, `SysRetFlow`, and `ExcessZoneExhFlow`. For every controlled
  Zone it clears `ZoneInfiltrationFlag`,
  `IncludeInfilToZoneMassBal`, mass-conservation `RetMassFlowRate`, and Zone
  `ExcessZoneExh`; with Space heat balance enabled it also clears
  `ExcessZoneExh` for controlled stored Spaces.
- It snapshots the prior building mixing and return totals and zeros the
  current local totals.
- The main Zone pass uses numeric order without enforcement and
  `ZoneReOrder(ZoneNum1)` with enforcement. Uncontrolled Zones are skipped.
  The reorder array is trusted without local range, uniqueness, or ownership
  validation.
- Each controlled Zone clears `TotExhaustAirMassFlowRate` and calls its
  `setTotalInletFlows`. Under `doSpaceHeatBalance`, controlled stored Spaces
  call their own inlet-flow child, while uncontrolled Spaces receive
  `scaleInletFlows` from the Zone node to the Space system node using raw
  `fracZoneVolume`. The source explicitly leaves mass balance at Zone level.
- Exhaust-node mass flow is accumulated only when the global
  `AirflowNetworkNumOfExhFan` is zero. Any nonzero global AFN exhaust-fan
  count suppresses that direct node summation for every Zone.
- If `ZoneMassBalanceFlag(ZoneNum)` is true, the routine sums return-node
  flows for positive node identities. Iteration zero, `AdjustReturnOnly`,
  and `AdjustReturnThenMixing` start from the stored incoming mixing flow;
  later mixing-first passes instead derive
  `max(0, return + exhaust - inlet + source mixing)`. It then calls
  `CalcZoneMixingFlowRateOfReceivingZone` and forms net mixing as receiving
  minus source mass flow.
- Standard return flow is
  `inlet + net mixing - (exhaust - balanced exhaust)`. Without enforcement,
  a negative value becomes positive `ExcessZoneExh` and the return target is
  clamped to zero; a nonnegative value clears the excess. With enforcement,
  excess is always zero and the target is clamped nonnegative.
- `calcReturnFlows` is then called once for every controlled Zone visit.
  Under enforcement, inlet and exhaust mass-conservation fields are
  overwritten and the selected adjustment mode adds work as follows:

| Adjustment | Extra receiving-mixing calls | Extra return-flow calls | Infiltration child |
|---|---:|---:|---:|
| `AdjustMixingOnly` | 0 | 0 | once |
| `AdjustMixingThenReturn` | 0 | 1 | once |
| `AdjustReturnOnly` | 0 | 1 | once |
| `AdjustReturnThenMixing` | 1 | 2 | once |
| any other value | 0 | 0 | once |

The table excludes the conditional initial receiving-mixing call and the
unconditional first return-flow call. Every enforced controlled-Zone visit
reaches exactly one of the three static `CalcZoneInfiltrationFlows` call
sites. Each return-derived adjustment uses
`max(0, inlet - exhaust + net mixing)` and, only outside `DoingSizing`,
applies raw `min(..., AirLoopDesSupply)` before delegating return allocation.

For any other adjustment value, enforcement has already zeroed the
mass-conservation `RetMassFlowRate`. Local `ZoneReturnAirMassFlowRate` also
starts at zero, but an independently true `ZoneMassBalanceFlag` first adds
current flows from positive return-node identities. The unconditional baseline
`calcReturnFlows` result is not copied into either value, so the building
return total adds zero. In ordinary freshly initialized
`NoAdjustReturnAndMixing` topology, `SetZoneMassConservationFlag` leaves the
Zone flag unset and the fallback infiltration child receives zero; an
inconsistent true flag plus an unrecognized adjustment can pass the pre-summed
return-node flow instead.

After each controlled Zone calculation, the routine accumulates building
mixing and return totals. Every positive return-air-loop identity receives
that node flow in `ZoneRetFlow`; when `TotAvailAirLoopOA > 0`, it also receives
the Zone excess exhaust in proportion to
`MaxOutAir / TotAvailAirLoopOA`.

The next primary-air-loop pass computes
`adjusted=max(0, ZoneRetFlow-ExcessZoneExhFlow)`. A strictly positive
`ZoneRetFlow` produces `ZoneRetFlowRatio=adjusted/ZoneRetFlow`; otherwise the
ratio is one. It then clears `ZoneRetFlow` for reconstruction. A second Zone
pass always uses numeric Zone order, skips uncontrolled Zones, and multiplies
the flow at each return node whose identity is positive by its air-loop ratio
when that air-loop identity is positive. It then rebuilds air-loop plus per-Zone
return totals. Aliased or
repeated node identities are neither deduplicated nor validated.

The imbalance-warning path runs only when all of these are true:

- Zone mass-balance enforcement is false;
- ordinary sizing and HVAC sizing simulation are both false;
- warmup is false;
- `FirstHVACIteration` is false; and
- the Zone's sticky `FlowError` latch is false.

It first applies the strict `HVAC::SmallMassFlow` threshold to unbalanced
system outflow. Only when that passes does it subtract outdoor-air,
ventilation, and incoming-mixing mass flow and require a second strict
`unbalancedFlow > HVAC::SmallMassFlow` comparison. Only then does it convert
the remaining imbalance to volume using the current psychrometric Zone
density and `StdRhoAir` and apply the strict `HVAC::SmallAirVolFlow`
threshold. A reported imbalance emits one warning,
one timestamp, and four continuation messages before setting
`FlowError=true`, so later calls suppress the warning for that Zone.

From `Iteration > 0`, convergence is the sum of absolute changes in building
mixing and return flow. Strict residual `< 1.0e-5` clears
`ZoneMassBalanceHVACReSim` and breaks; equality does not converge, while any
failed comparison sets the flag true. Non-enforced execution breaks after
one building pass. Enforced execution therefore performs between two and 25
passes; exhaustion or a NaN residual leaves the re-simulation request true.
After loop exit every primary air loop receives the unclamped
`SysRetFlow = ZoneRetFlow - RecircFlow + LeakFlow`.

`FirstHVACIteration` affects only warning suppression. It changes no solver,
iteration, airflow, mixing, infiltration, return, excess, or convergence
arithmetic.

The function contains 37 direct persistent mutation sites over 21 normalized
state-path families: 29 plain assignments, seven `+=` sites, and one `*=`
site. Those families cover the re-simulation flag; air-loop supply,
recirculation, leakage, outdoor-air, return, excess, ratio, and system-return
state; Zone infiltration and mass-conservation state; Zone/Space equipment
excess and Zone exhaust totals; return-node flow; and the warning latch.
Mutations inside the inlet, scaling, mixing, return, infiltration,
psychrometric, and diagnostic children are additional.

Lexically the body has 38 `if` tokens, seven `else` tokens including one
`else if`, 14 `for` loops split 12 indexed plus two range loops, one
`do/while`, two `break`, and four `continue`. Thus there are 15 loop
constructs in total. There is no switch, ternary, explicit return, result
status, or catch.

Under the established non-accessor convention, its 19 operational/service
call sites are two `setTotalInletFlows`, one `scaleInletFlows`, two receiving
mixing, four return-flow, three infiltration, one density, and six diagnostic
calls. Nine `max`, three `min`, two `abs`, and four formatting sites are
counted separately.

The routine performs no complete up-front validation of reorder, extent,
Zone/Space ownership, node or air-loop identity, aliasing, density, finite
arithmetic, or child state. It has no checkpoint, cleanup, transaction,
catch, rollback, or retry repair. Failure retains every completed ordered
prefix: the initial re-simulation clear, partial AirDistUnit additions,
all-OA overwrites, enforcement resets, completed Zone/Space and air-loop
passes, return-node scaling, child mutations, and any diagnostics.

A warning failure occurs before `FlowError=true`, so retry can repeat a
partial message sequence; successful completion makes the latch sticky. A
failure before the final system-return loop suppresses that tail, while a
failure within it preserves the completed air-loop prefix.

Same-state replay is generally non-idempotent. The AirDistUnit prefix adds
again, non-enforced excess/return aggregation can reuse prior air-loop state,
return nodes are multiplied in place, mixing and infiltration children own
additional state, and warning output is sticky. Normal parent flow calls
`InitZoneEquipment` first and externally zeros six air-loop aggregates, but
that is not CP269-local recovery. Enforced per-pass resets repair only a
selected subset and cannot roll back an interrupted call.

The C++ unit sources contain exactly 19 literal direct calls, all with
`FirstHVACIteration=false`: three warning-threshold calls in
`ZoneEquipmentManager_CalcZoneMassBalanceTest`, two return-basis calls in
`CalcZoneMassBalanceTest3`, one enforced no-adjust call in
`HeatBalanceManager_ZoneAirMassFlowConservationData2`, three calls each for
`AdjustMixingOnly`, `AdjustReturnOnly`, `AdjustReturnThenMixing`, and
`AdjustMixingThenReturn`, plus one additional source-and-receiving-Zone
`AdjustMixingOnly` call.

Those direct calls split into five non-enforced and 14 enforced entries. The
enforced modes are one no-adjust, four mixing-only, three return-only, three
return-then-mixing, and three mixing-then-return.

The 19 direct calls have 204 post-call `EXPECT` or `ASSERT` macros. All six
`AdjustReturnThenMixing` and `AdjustMixingThenReturn` calls immediately run
`CalcAirFlowSimple` before their 89 macros, split 47 plus 42, so those
outcomes are not CP269-isolated. Remaining assertions cover warning presence
but not exact text or the `FlowError` latch, return-node flow, conservation,
mixing state, and infiltration.
The enforced tests consume stored `ZoneReOrder` values, but each fixture
exercises only one ordering and no test perturbs or compares permutations.

The exact bounded route-representative unit census is 72 CP269 route
entries, not 72 literal dynamic function invocations:

- 19 direct leaf calls;
- six direct `SizeZoneEquipment` parents;
- 13 directly attributable `SimZoneEquipment` parents; and
- 17 effective `ManageSizing` contexts, each contributing one representative
  sizing route and one representative later simulation route.

The unit sources contain 18 lexical `ManageSizing` calls. The
`WaterToWaterSimple` call at source line 1538 performs plant sizing only and
reaches no CP269 route. In each of the other 17 contexts, the Zone-sizing
route can repeat inside design-day and timestep `ManageHeatBalance` cadence.
The bounded representative total is 51 true and 21 false
`FirstHVACIteration` routes, 14 enforced and 58 non-enforced; none activates
Space heat balance. Beyond that bound, 56 completing `ManageSimulation`
contexts include 55 with Zones and 34 that also perform Zone sizing, but
environment, design-day, timestep, and HVAC convergence cadence prevents an
exact dynamic invocation count.

Coverage omits a direct iteration-count or re-simulation-flag assertion,
exact tolerance equality, 25-pass exhaustion, nonconvergence and NaN,
`FirstHVACIteration=true` warning suppression, the `DoingSizing` supply-cap
bypass, AirDistUnit prefix, uncontrolled-Zone and controlled/uncontrolled
Space variants, AFN exhaust suppression, all-OA setup, positive-available-OA
excess allocation and `ZoneRetFlowRatio`, final `SysRetFlow`, and
`Add`, `No`, or `MixingSourceZonesOnly` infiltration modes. Every direct
enforced call uses `Adjust` plus `AllZones`; input-only enum tests do not call
CP269. Malformed topology, failure/rollback, and unchanged-input replay are
also uncovered.

Rust has no exact or snake-case CP269 function, re-simulation flag,
`ZoneRetFlowRatio`, excess-exhaust allocation, mass-conservation arena,
`ZoneReOrder`, or flow-adjustment enum. The compatibility runtime instead
iterates prebound IdealLoads systems directly, constructs a fresh four-value
by-copy `ZoneSysEnergyDemand`, and calls PurchasedAir. Its execution plan
contains metadata labels for `SimZoneEquipment` and `SimPurchasedAir`, but no
mass-balance execution step.

The diagnostic `simulate_ideal_loads_node_state_projection` is a superficially
similar but explicitly non-parity path. It seeds a design/default supply flow
and assigns that same fixed flow to Zone-air and return-node records; it owns
no inlet/exhaust/mixing/infiltration or AirLoop aggregates, adjustment mode,
iteration, convergence, re-simulation latch, first-iteration warning gate, or
return allocation.

The Rust demand record, diagnostic `NodeStateStore`, and static AirLoop graph
skeleton therefore do not supply a shared mutable CP269 solver. Rust lacks
operational `AirLoopFlow` and AirDistUnit aggregates, Zone equipment flow
topology, Space heat-balance allocation, and the mass-balance lifecycle.

Across 120 unique data models, the census includes 30 equipment
lists, 30 equipment connections, and 30 IdealLoads systems. Every list is
one-entry `SequentialLoad` at heating/cooling sequence `1/1` with blank
fraction schedules. The census has zero `ZoneAirMassFlowConservation`,
air-distribution-unit, air-terminal, mixing, cross-mixing, infiltration, ventilation,
AirflowNetwork, duct-loss, `Sizing:Zone`, Space, SpaceList, or SpaceHVAC objects, and all 61
SimulationControl records disable Zone sizing. Three AirLoopHVAC skeletons
exist only in diagnostic/nonclaim, run-blocked cases; they are not CP269
execution.

All 30 IdealLoads equipment connections do have one nonblank inlet and one
nonblank return node, with blank exhaust. Their EnergyPlus oracle runs CP269's
ordinary non-enforced one-pass inlet-to-return bookkeeping during simulation;
the 61 sizing-disabled controls remove only the separate sizing parent call.
Rust dispatch validates list edges and inlet nodes but does not consume return
or exhaust topology, then invokes PurchasedAir directly. Existing System Node
Mass Flow Rate coverage is supply-node-only and explicitly excludes broad HVAC
flow balancing, so that oracle activity is not Rust parity evidence.

The roadmap still requires Rust-owned shared node and air-loop flow state,
Zone/Space equipment topology, simple-airflow and mass-conservation arenas,
ordered mixing/return/infiltration adjustment, convergence, lifecycle, and
diagnostic parity. Static graph metadata and isolated node mass-flow fields
cannot establish this iterative transaction.

CP269 changes no Rust target or state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 273 routines, split 58
`state_mapped` plus 215 `source_mapped`, with 150 required. Domain-required
counts become heat-balance 88, HVAC 39, plant 1, and time/schedule 22, with
readiness `0/88`, `0/39`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

## CP270 `CalcZoneInfiltrationFlows` Mass-Conservation Infiltration Leaf

CP270 adds canonical required `routine.calc_zone_infiltration_flows`
immediately after `routine.calc_zone_mass_balance` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
requirement. It changes no EnergyPlus source inventory.

The canonical declaration is `ZoneEquipmentManager.hh` lines 223-226 and the
complete definition is `ZoneEquipmentManager.cc` lines 5285-5340. CP269 ends
at source line 5283 and line 5284 is blank. The exact interface is:

```cpp
void CalcZoneInfiltrationFlows(
    EnergyPlusData &state,
    int const ZoneNum,
    Real64 const &ZoneReturnAirMassFlowRate);
```

The function returns no status. `state` is the only mutable aggregate;
`ZoneNum` is a const by-value identity and the passed total return flow is a
const reference. There is no Space identity, first-iteration argument,
sizing argument, default, result object, or `noexcept` boundary.

There are exactly three production call expressions, all inside CP269 after
its `EnforceZoneMassBalance` guard:

- line 5118 serves `AdjustMixingOnly` and `AdjustMixingThenReturn`;
- line 5154 serves `AdjustReturnOnly` and `AdjustReturnThenMixing`; and
- line 5158 serves every remaining return/mixing adjustment value.

Exactly one site executes for every controlled Zone visit in every enforced
CP269 solver pass. Non-enforced CP269 never invokes CP270. Before each
enforced pass, CP269 clears that Zone's `ZoneInfiltrationFlag` and
`IncludeInfilToZoneMassBal`, but it does not clear mass-conservation
`InfiltrationMassFlowRate` or the selected infiltration object's
`MassFlowRate`. Its two-to-25-pass solver can therefore repeat CP270 two to 25
times per controlled Zone.

CP269 does not consume a CP270 result status or include infiltration state in
its building mixing/return convergence residual. The leaf's state is instead
available to the later simple-airflow calculation. The passed return-flow
value already reflects the selected CP269 return/mixing adjustment branch.

For an eligible Zone the local signed residual is computed in this exact
order:

```text
R = MixingSourceMassFlowRate - MixingMassFlowRate
  + TotExhaustAirMassFlowRate + ZoneReturnAirMassFlowRate
  - TotInletAirMassFlowRate
```

The leaf uses `ConvergenceTolerance = 0.000010`. Every threshold is strict;
no branch uses an inclusive comparison.

The outer branch matrix is source-ordered as follows:

- Exact `InfiltrationFlow::No` performs no persistent write after acquiring
  `MassConservation(ZoneNum)`.
- For any other treatment, `InfiltrationPtr <= 0` writes only conservation
  `InfiltrationMassFlowRate = 0`.
- With a positive pointer, eligibility is exactly `IsOnlySourceZone` or
  `InfiltrationForZones == AllZones`.
- Eligible `Adjust` with `abs(R) > 1.0e-5` sets the Zone infiltration flag,
  stores signed `R` in conservation state, sets the include marker to one,
  writes `R` to the infiltration object, and then clamps only that object to
  `max(0, R)`.
- Eligible `Adjust` when the comparison fails zeros conservation and object
  mass flow, but does not locally clear the flag or include marker.
- Eligible `Add` with `R > 1.0e-5` sets the same flag, conservation value, and
  include marker, then performs `object.MassFlowRate += R` without a clamp.
- Eligible `Add` when the comparison fails zeros only conservation mass flow;
  the object, flag, and include marker are retained locally.
- An ineligible `Adjust` Zone copies the current infiltration-object flow into
  conservation state, while an ineligible `Add` Zone zeros conservation flow.
- With a positive pointer, an invalid or otherwise unmatched non-`No`
  treatment reaches no treatment-specific persistent write.

Under `MixingSourceZonesOnly`, only exact `IsOnlySourceZone` qualifies. A Zone
that is both a source and a receiver does not qualify through a separate
source-and-receiving flag. `AllZones` makes the classification irrelevant.

The body contains two syntactic inner `else if (... == No)` arms. Both are
unreachable because the unchanged treatment already passed the outer
`... != No` guard. A nonpositive pointer is handled before this inner dispatch,
so even an unmatched non-`No` enum zeros conservation flow on that path.

`Adjust` deliberately separates signed conservation state from the physical
object. A residual below `-1.0e-5` sets the flag and include marker, retains the
negative value in conservation state, briefly writes it to the object, and
then clamps only the object to zero. `Add` accepts only a strictly positive
residual and can accumulate onto any preexisting finite or nonfinite object
value.

Exact positive or negative tolerance equality takes the comparison-false
branch. A NaN also fails both ordered comparisons: `Adjust` zeros conservation
and object flow, whereas `Add` zeros conservation only. Neither path emits a
diagnostic. Positive or negative infinity is not rejected; it follows the
corresponding strict-comparison and arithmetic path.

The function contains 17 direct persistent mutation sites over four normalized
state-path families: 16 plain assignments and one `+=`. Two of the plain
assignments are the unreachable inner-`No` sites. The families are:

- per-Zone `ZoneInfiltrationFlag` at two sites;
- mass-conservation `InfiltrationMassFlowRate` at nine sites;
- mass-conservation `IncludeInfilToZoneMassBal` at two sites; and
- selected infiltration-object `MassFlowRate` at three assignments plus one
  compound addition.

Lexically the body has 11 `if` tokens and eight `else` tokens, including four
`else if` tokens. It has no loop, switch, ternary, explicit return statement,
`break`, `continue`, diagnostic, result status, or catch. Under the established
non-accessor convention it has zero operational child or service calls. One
`std::abs` and one `max` mathematical site are counted separately.

The indexed accessor census is one `MassConservation`, two `ZoneEquipConfig`,
two `ZoneInfiltrationFlag`, and six `Infiltration` sites. The leaf performs no
density, volume-flow, schedule, psychrometric, node, or air-loop update.

There is no complete validation of `ZoneNum`, allocation, pointer upper bound,
Zone/object ownership, enum validity, aliasing, or finite arithmetic. The
`MassConservation(ZoneNum)` reference is acquired before the outer treatment
guard, so an invalid Zone identity can fail even for treatment `No`. A positive
`InfiltrationPtr` is trusted without checking its allocated upper bound.

The routine has no checkpoint, cleanup, transaction, rollback, retry repair,
or local failure diagnostic. An abnormal exit retains every completed ordered
write. On the active `Adjust` path, flag, signed conservation flow, and include
state commit before the first object write; interruption after the raw object
write and before the clamp can expose a negative object flow. Its zero path can
clear conservation before a failing object access.

The active `Add` path likewise commits flag, conservation flow, and include
state before its final object `+=`. Failure there retains that prefix; failure
later in CP269 after a successful addition can cause a parent replay to add the
same residual again.

For fixed dependencies, a successful active `Adjust` replay overwrites the
fields it reaches. It is not canonical whole-state repair: treatment `No`,
comparison-false, pointerless, ineligible, and invalid-enum paths intentionally
leave selected flag, include, conservation, or object values untouched.
Positive `Add` replay is directly non-idempotent because it compounds object
flow, including across CP269 solver passes.

CP269's per-pass flag/include clear is external setup rather than CP270-local
recovery. It does not clear conservation or object infiltration flow. Direct
leaf calls, malformed lifecycles, and abnormal parent re-entry can therefore
observe state that the ordinary parent sequence would have reset only in part.

The C++ unit sources contain zero direct calls to the leaf. The established
72-entry bounded CP269 route-representative census has only 14 entries that
enable enforcement and can reach CP270; all are direct `CalcZoneMassBalance`
test calls with `FirstHVACIteration=false`. The other 58 routes are
non-enforced and skip all three CP270 call sites.

The 14 enforced parent entries split into one no-return/mixing-adjustment
case, four mixing-only, three return-only, three return-then-mixing, and three
mixing-then-return cases. Every one configures infiltration treatment
`Adjust` and Zone selection `AllZones`.

Across one solver pass their controlled-Zone footprint is 29 visits:

- two visits from the no-adjustment parent;
- nine from mixing-only parents;
- six from return-only parents;
- six from return-then-mixing parents; and
- six from mixing-then-return parents.

Twenty-seven visits have a positive infiltration pointer and two have zero.
The topology split is 14 source-only, one source-and-receiving, and 14
receiving-only Zones. CP269's guaranteed two-to-25 enforced passes bound these
successful test executions between 58 and 725 literal CP270 calls. The exact
iteration total is not instrumented.

The tests contain 28 post-parent infiltration assertions. Twenty-seven read
mass-conservation `InfiltrationMassFlowRate`, split six positive and 21 zero;
one reads an infiltration object's positive `MassFlowRate`. They contain zero
assertions on `ZoneInfiltrationFlag` or `IncludeInfilToZoneMassBal`, and zero
assertions on object zeroing or the negative-residual clamp.

Six return/mixing parent calls execute
`CalcAirFlowSimple(state, 0, true, true)` after CP269 and before 12 of those
conservation assertions. That child consumes CP270 flag/object state without
overwriting the conservation scalar, so the scalar checks remain observable
but the surrounding outcome is integration-level rather than leaf-isolated.
The two zero-pointer visits are also incidental and are not isolated from
other parent branches.

Behavioral coverage therefore omits direct leaf entry, `Add`, outer `No`, and
`MixingSourceZonesOnly`; the latter modes have input-enum tests only. It also
omits source-and-receiving exclusion under source-only scope, negative active
`Adjust`, exact positive and negative tolerance equality, near-zero clearing,
NaN, infinity, invalid modes, and an oversized positive pointer.

No test isolates flag/include lifetime, underlying-object clearing, additive
iteration accumulation, fixed-input replay, malformed Zone or object identity,
partial failure, rollback, or retry. The positive `Add` `+=` path and the
outer-`No` stale-state no-op have no runtime oracle.

Rust has no exact or snake-case CP270 function, typed
`ZoneAirMassFlowConservation`, mass-conservation arena, infiltration object
flow, `InfiltrationFlow`, `InfiltrationZoneType`, eligibility classification,
pointer, flag, or include-marker lifecycle. It also has no runtime residual
that combines mixing, exhaust, return, and inlet mass flow.

Typed `ZoneEquipmentConnection` metadata retains Zone-air, inlet, exhaust, and
return names, but compatibility dispatch validates and consumes only the
equipment-list edge and inlet before calling PurchasedAir with a fresh demand
snapshot. The diagnostic `NodeStateStore` projection assigns a fixed supply
flow to supply, Zone-air, and return records; it ignores exhaust and owns no
infiltration-balance transition.

`DesignSpecification:OutdoorAir` and demand-controlled-ventilation state are
PurchasedAir outdoor-air inputs, not mass-conservation infiltration state.
Run-blocked AirBoundary mixing metadata and the hard-coded zero
Zone outdoor-air-transfer report are also not CP270 implementations. Raw
`ZoneAirMassFlowConservation`, infiltration, mixing, cross-mixing, ventilation,
and AirflowNetwork inputs remain unsupported or run-blocked for arbitrary
runtime.

Across 120 unique data models, split 108 IDF and 12 epJSON, the census contains
zero `ZoneAirMassFlowConservation`, all three infiltration families, mixing,
cross-mixing, both ventilation families, and AirflowNetwork topology. Thirty
models contain one IdealLoads system, list, and equipment connection. Every
connection has nonblank inlet and return names and a blank exhaust name.

All 61 `SimulationControl` records disable Zone sizing, including all 30
IdealLoads models. More importantly, no model enables Zone mass-balance
enforcement. EnergyPlus therefore executes ordinary non-enforced CP269
bookkeeping during those simulations and reaches CP270 zero times. Rust's
direct PurchasedAir route supplies no alternative evidence.

The roadmap still requires Rust-owned Zone and infiltration identities,
mass-conservation state, mixing/exhaust/return/inlet aggregates, treatment and
Zone-selection enums, threshold behavior, additive lifecycle, and
failure/replay semantics. Static node metadata cannot establish this mutable
leaf.

CP270 changes no Rust target or state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 274 routines, split 58
`state_mapped` plus 216 `source_mapped`, with 151 required. Domain-required
counts become heat-balance 88, HVAC 40, plant 1, and time/schedule 22, with
readiness `0/88`, `0/40`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

## CP271 `CalcZoneLeavingConditions` Return-Node State Projection

CP271 adds canonical required `routine.calc_zone_leaving_conditions`
immediately after `routine.calc_zone_infiltration_flows` and before
`routine.sim_purchased_air`, plus the same ordered HVAC project-contract
requirement. It changes no EnergyPlus source inventory.

`CalcZoneLeavingConditions(EnergyPlusData &, bool FirstHVACIteration)` is
declared at `ZoneEquipmentManager.hh` line 240 and implemented completely at
`ZoneEquipmentManager.cc` lines 5342-5543. The definition adds only
function-type-neutral top-level `const` to the by-value Boolean. It has no
default argument, status result, or exception specification. CP270 ends at
source line 5340; CP272 `UpdateZoneEquipment` starts at line 5545.

There are exactly two executable production call expressions:

- `SizeZoneEquipment` calls the leaf unconditionally at line 677 with `true`,
  after the complete Part1 sizing sweep and CP269 mass balance and before any
  Part2 sizing entry; and
- `SimZoneEquipment` calls it unconditionally at line 4188 with its incoming
  `FirstHVACIteration`, after Zone exhaust controls, exhaust-system simulation,
  and CP269 and before whole-system duct loss and return-path simulation.

`ManageZoneEquipment` selects one of those parents. Neither caller adds a
controlled-Zone or return-topology guard. `FirstHVACIteration` is not sampled
by the leaving-state arithmetic; its only body use is forwarding the value to
the final demand-initialization child.

Before the numeric Zone pass, the leaf tests
`doSpaceHeatBalanceSimulation && !DoingSizing`. When true, it range-visits
every stored `zoneReturnMixer`, without filtering by controlled Zone or return
node, and invokes these three methods in order for each occurrence:

1. `setInletFlows`;
2. `setInletConditions`; and
3. `setOutletConditions`.

The following numeric Zone loop skips every uncontrolled configuration and
every controlled configuration with zero return nodes. A skipped Zone also
skips the final demand initializer. For each entered Zone, `ZoneMult` is the
product of `Multiplier` and `ListMultiplier`. Each return-node visit forms
`MassFlowRA` from return-node mass flow divided by `ZoneMult`, then adds the
full mapped exhaust-node mass flow only when the exhaust identity and its flow
are positive. The exhaust contribution is not divided by the multiplier.

The return-air base temperature is selected in this precedence order:

1. the already-written return-node temperature when Space heat-balance
   simulation is active, sizing is inactive, and
   `returnNodeSpaceMixerIndex > -1`;
2. allocated, active room-air-pattern `Tleaving` when `!BeginEnvrnFlag`; or
3. the Zone node temperature.

Every return-node visit first calls the sensible return-air convection-gain
sum. When the Zone reports an airflow-window return, the same visit then scans
all stored Zone Space identities and every Surface in each Space's inclusive
heat-transfer range. A Surface contributes only for positive current gap flow
and return-air destination. Its mass is
`rho(OutBaroPress, gap-outlet temperature, Zone-node humidity ratio)` times
current gap flow and Surface width; mass and mass-times-temperature are
accumulated to form a positive-flow mixture temperature.

That complete Zone-level airflow-window scan is inside the return-node loop.
Multiple return nodes therefore recompute and apply the same unpartitioned
aggregate once per node. There is no per-return-node window ownership, fraction, or filter, and no
sorting or deduplication. Density also uses Zone-node humidity even
when the base temperature came from a Space mixer or room-air pattern.

With `NoHeatToReturnAir=false`, the leaf computes moist-air specific heat from
Zone-node humidity. Positive `MassFlowRA` follows this ordered sensible path:

- positive airflow-window mass is blended with the selected base temperature
  when return mass is at least the window mass;
- when window mass exceeds return mass, return temperature becomes the window
  mixture and the excess window sensible term is added to
  `SysDepZoneLoads`;
- sensible return-air gain is divided by return mass and specific heat and
  added to the working return temperature;
- the return node is clamped to `HVAC::RetTempMin` and `HVAC::RetTempMax`;
  outside `ZoneSizingCalc`, but not during it, the clipped energy is added to
  `SysDepZoneLoads`; and
- with a positive mapped exhaust flow and positive sensible return gain, an
  exact `Shared` configuration adds only the gain temperature rise to the
  existing exhaust temperature, while every other configuration overwrites
  exhaust temperature with the unclamped working return temperature.

Nonpositive or NaN `MassFlowRA` takes the other branch. Positive window mass
contributes its signed sensible term, positive sensible return gain moves to
`SysDepZoneLoads`, and return
temperature is forced to Zone-node temperature rather than the selected Space
or room-air base. With `NoHeatToReturnAir=true`, return temperature is also
forced to the Zone value, but the locally computed sensible return gain and
window heat are not transferred by this branch.

Only exhaust-node temperature is changed. Exhaust pressure, humidity,
enthalpy, CO2, and generic contaminant are not synchronized. Return pressure
always copies Zone-node pressure, regardless of the heat branch.

Humidity handling follows temperature work. With heat-to-return enabled and
positive return mass, the leaf calls the node-specific latent return-gain sum,
computes water-vapor enthalpy, and sets return humidity ratio to Zone humidity
plus latent gain divided by vapor enthalpy and mass. Every other path copies
Zone humidity, adds the Zone's full `LatCaseCreditToHVAC` to
`LatCaseCreditToZone` without clearing the HVAC credit, calls the same latent
sum, and adds that result to the Zone heat-balance `latentGain`. These Zone
additions repeat for every return node.

The leaf always recomputes return enthalpy from final temperature and humidity.
It conditionally copies Zone-node CO2 and generic contaminant when their global
simulations are active. After all return nodes complete, it calls
`InitSystemOutputRequired(state, ZoneNum, FirstHVACIteration, true)` exactly
once for that entered Zone. The explicit final `true` requests simulation-order
reset; this child is the only consumer of `FirstHVACIteration` in CP271.

The body has 26 `if` tokens, 11 `else` tokens including two `else if` tokens,
three indexed and two range loops, and two `continue` statements. It has no
`while`, `switch`, ternary, explicit return, `break`, diagnostic, status,
checkpoint, catch, transaction, rollback, or cleanup.

There are 23 direct persistent mutation sites over nine normalized state
families: 13 plain assignments and ten compound additions. The families are
five `SysDepZoneLoads` additions; seven node-temperature sites across return
and exhaust roles; one return-pressure site; three return-humidity sites; two
refrigeration Zone-credit additions; two Zone latent-gain additions; and one
each for return enthalpy, CO2, and generic contaminant. Mixer and demand-child
mutations are additional.

Under the established census convention, the leaf owns 12 operational service
call sites: three mixer methods, four internal-gain queries, four
psychrometric functions, and one demand initializer. One allocation predicate
and 67 indexed accessors bring the complete syntactic call/accessor expression
count to 80.

No complete Zone, return, exhaust, mixer, Space, Surface, ownership, alias,
array-extent, finite-value, multiplier, or denominator validation precedes
mutation. A mixer failure retains completed methods and mixers and prevents
all Zone work. A Zone or return-node failure retains earlier Zones, nodes, and
the current-node prefix; temperature and load writes may survive without the
later humidity, enthalpy, or contaminant writes. A final demand-child failure
retains every leaving-node write for that Zone plus the reached child prefix.
There is no local repair protocol.

Same-state replay is generally non-idempotent. Ten `+=` sites can compound
system-dependent load, shared exhaust temperature, refrigeration Zone credit,
and Zone latent gain. `LatCaseCreditToHVAC` is not cleared after transfer, and multiple return nodes
can repeat the same Zone-level credit and
unpartitioned airflow-window aggregate. Repeated or aliased return, exhaust,
Space, and Surface identities are order-dependent. Plain node fields may
reconstruct some fixed paths, but optional exhaust and contaminant branches can
leave old values when disabled. Mixer and demand children add their own replay
semantics. The earlier `SimZoneEquipment` clear of `SysDepZoneLoads` is parent
setup, not CP271-local recovery.

The bounded C++ route-representative census contains 54 leaf entries: one
direct call, 23 sizing routes, and 30 simulation routes. It is composed of six
direct `SizeZoneEquipment` parents, 13 directly attributable simulation routes,
and 34 size/simulation projections from 17 effective `ManageSizing` contexts.
The 18th lexical sizing context is plant-only and does not reach Zone sizing.
The flags split 52 true and two false; both false entries are simulation
routes. Fifty-six completing `ManageSimulation` tests have runtime-dependent
HVAC cadence, so this bounded census does not invent an exact dynamic call
count.

All six direct sizing parents configure zero return nodes. They call CP271 but
skip its node and final-demand work. Other parent tests can contain return
nodes, availability, air-terminal mixer, or plenum behavior, but none owns a
CP271-specific complete return-node-state oracle. Their results are confounded
with equipment and parent simulation.

The C++ unit sources contain exactly one literal direct leaf call, in
`CZoeEquipmentManager_CalcZoneLeavingConditions_Test` at
`ZoneEquipmentManager.unit.cc` line 4480. It uses one controlled equipment
configuration, two positive-flow return nodes, one shared positive-flow
exhaust node, `NoHeatToReturnAir=false`, and 50 W then 100 W sensible return
gains. A non-Shared write followed by a `Shared` addition is checked with five
post-call expectations: preserved Zone temperature, two return temperatures,
one exhaust temperature, and a relation between the two return rises and the
exhaust rise.

That test does not activate a Space return mixer, room-air pattern,
airflow-window return, no/negative flow, clamp, refrigeration latent gain,
contaminant, or `NoHeatToReturnAir=true` branch. It asserts no return pressure,
humidity, enthalpy, latent state, system-dependent load, or demand reset. Its
`Zone.IsControlled` field remains false, so the final distribution wrapper
returns at its first gate; the meaningful first/later-iteration tail difference
is not isolated. No test covers malformed state, aliasing, partial failure,
rollback, replay, or repair.

Rust has no exact or snake-case CP271 function. Active compatibility classes
validate a connection and construct a fresh four-value sensible/moisture demand
snapshot before calling PurchasedAir directly. They never execute the leaving
projection or its reset-true demand tail and own no total, unadjusted, six-way
sequence, deadband, or Zone/Space demand arena.

Typed `ZoneEquipmentConnection` records retain inlet, exhaust, Zone-air,
return, return-fraction schedule, and return-basis names, and the compiler
registers those node references. Active dispatch resolves and consumes only
the supply/inlet edge. It does not use return/exhaust pairing, return basis or
schedule, Shared/Multi configuration, Space mixer identity, or Zone
multipliers for CP271 arithmetic.

`IdealLoadsNodeStateProjection` is a separate explicitly diagnostic,
non-parity path. Its `AirNodeState` contains temperature, humidity ratio, mass
flow, and optional temperature setpoint, but no pressure, enthalpy, CO2, or
generic contaminant. The projection copies design/default supply flow to Zone
and return records and assigns fixed default Zone temperature and humidity to
returns. It implements no exhaust, gain, airflow-window, room-air/mixer, clamp,
latent/refrigeration, contaminant, or demand-reset behavior. Active System Node
result output covers the supply node rather than a CP271-computed return node.

The CLI finite-limit and humidity evidence instead reads EnergyPlus same-call
return temperature and humidity and injects that oracle recirculation state
into the Rust case path. Across manifests that reference the 30 IdealLoads IDF
models, the broader return-node output census is 35 rows in 17 cases: 17
temperature, 17 humidity-ratio, and one mass-flow row. Three are baseline and
32 are diagnostic; zero is conformance-level. Those rows therefore do not
establish Rust CP271 numerics.

Across 120 unique data models, split 108 IDF and 12 epJSON, 30 models contain
one IdealLoads system, equipment list, and connection. Every connection has a
nonblank inlet and direct return and a blank exhaust. The corpus has no
`Sizing:Zone`, Space or SpaceList, SpaceHVAC return mixer, room-air model,
airflow window, Lights, refrigeration case/walk-in/air-chiller, return path,
or AirflowNetwork topology.

Ordinary EnergyPlus simulation enters CP271 for the controlled Zone and return
node in all 30 IdealLoads models, with no sizing route. The zonal-only setup
selects `NoHeatToReturnAir=true`, so the represented branch copies Zone
return temperature, humidity, and pressure, recomputes enthalpy, sees zero
return gain, and runs the demand tail. One CO2-DCV model also enables return
CO2 copying, but has no return-CO2 conformance output. Dynamic HVAC invocation
counts are not instrumented. Rust's direct PurchasedAir route and oracle
recirculation input supply no equivalent transition evidence.

The roadmap still requires Rust-owned return/exhaust node state including
pressure, enthalpy, and contaminants; Space and room-air topology; airflow
window and gain ownership; multiplier, clamp, latent, refrigeration, exhaust,
and demand-reset semantics; and complete failure/replay behavior. Static
connection names and diagnostic projection state cannot establish this mutable
leaf.

CP271 changes no Rust target or state, support declaration, test, capability,
output, comparator, case, manifest, numerical claim, performance claim, or
conformance status. Counts become 32 algorithms and 275 routines, split 58
`state_mapped` plus 217 `source_mapped`, with 152 required. Domain-required
counts become heat-balance 88, HVAC 41, plant 1, and time/schedule 22, with
readiness `0/88`, `0/41`, `0/1`, and `0/22`. The IdealLoads parent remains
`scaffold` at claim level `none`.

CP272 next adds required source-mapped `routine.update_zone_equipment`
immediately after `routine.calc_zone_leaving_conditions` and before
`routine.sim_purchased_air`. `UpdateZoneEquipment` is declared at
`ZoneEquipmentManager.hh` line 242 and implemented completely at
`ZoneEquipmentManager.cc` lines 5545-5568. `CalcAirFlowSimple` begins at
source line 5570.

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
