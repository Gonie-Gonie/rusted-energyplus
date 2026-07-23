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
yet CP240 still takes their non-air path. Across all 69 static roles, one
enables DOAS, four enable latent sizing, 43 have a residual supply node, and 26
use non-air output; cooling ITE and an adjustment factor above one have zero
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
Zones plus 21 Spaces and exactly one Zone role, no Space role, enables DOAS.
That fixture defaults to `NeutralSup` and supplies high- and low-side
summer/winter design conditions; only downstream table loads are asserted.
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
calls across seven Space-sizing configurations. Exactly one Zone enables
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

CP245 next maps `ZoneEquipmentManager::calcSizingOA`, declared at
`ZoneEquipmentManager.hh` lines 111-117 and implemented at
`ZoneEquipmentManager.cc` lines 1084-1206.

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
