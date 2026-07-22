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
`InitZoneAirSetPoints` at lines 2350-2816, `PredictSystemLoads` at lines
2870-3145, `CalcZoneAirTempSetPoints` at lines 3259-3460,
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

CP197 next maps `CalculateMonthlyRunningAverageDryBulb`, declared at
`ZoneTempPredictorCorrector.hh` line 284 and implemented at
`ZoneTempPredictorCorrector.cc` lines 2176-2275.

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
