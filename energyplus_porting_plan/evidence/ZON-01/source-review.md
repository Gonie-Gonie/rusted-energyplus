# ZON-01 bounded source and reuse review

This is a preparation packet. Its source/input/assignment policy must pass
independent review, then genuine original execution must precede Rust numerical
execution. All four ZON-01 gates remain pending. The original is EnergyPlus
26.1.0, `6f2e40d10250a105b49966baa24d843711e61048`.

The draft [source](../../contracts/ZON-01-source.json),
[cases](../../contracts/ZON-01-cases.json),
[exact policy](../../contracts/ZON-01-tolerances.json) and
[input-only request](../../cases/ZON-01/helper-request.json) separate constructor,
incoming bulk reconstruction, bare member initialization and genuine guarded
initialization. They contain no reference outputs to inject into Rust.

## Actual selected owners

`ZoneSpaceHeatBalanceData` in `ZoneTempPredictorCorrector.hh:102-145` gives MAT,
ZT, ZTAV, XMPT, XMAT[4], DSXMAT[4], TMX and TM2 the source ZoneInitialTemp of
23 degrees C. Current/average/temporary humidity defaults are .01; humidity
histories and the selected working fields start at +0. The separate module
MyEnvrnFlag defaults true. Global BeginEnvrnFlag belongs to external control.

`HeatBalanceSurfaceManager.cc:2231-2239` placement-news the concrete zone owner,
then sets only current airHumRat and airHumRatAvg from actual OutHumRat. Its
normal caller executes under BeginEnvrnFlag at 379-384. The helper selects this
byte-exact zone block; it does not certify all surface/flux/CTF histories,
Space reconstruction or enclosure MRT.

`ZoneSpaceHeatBalanceData::beginEnvironmentInit`,
`ZoneTempPredictorCorrector.cc:2818-2836`, owns four array fields and ten scalar
fields. In each index 0 through 3 it writes ZTM to +0, WPrevZoneTS and
DSWPrevZoneTS to external OutHumRat, and WPrevZoneTSTemp to +0. It then writes
WTimeMinusP/W1/WMX/WM2 from OutHumRat and resets airHumRatTemp,
tempIndLoad/tempDepLoad/airRelHum/AirPowerCap/T1 to +0 in source order.

The method leaves the ten selected constructor/current fields unchanged. In
particular it does not reset MAT/ZT/ZTAV, reconstruct the owner, update current
W, or impose a psychrometric W floor. Source AirPowerCap is W/K; it cannot be
identified with Rust air_heat_capacity_j_per_k in J/K.

`InitZoneAirSetPoints:2621-2670` calls the member only while actual MyEnvrnFlag
and BeginEnvrnFlag are both true, clears MyEnvrnFlag after the block, and
rearms it when BeginEnvrnFlag is false. The helper will call this whole genuine
function on an explicitly prepared one-zone, zero-controller state, with the
one-time allocation branch disabled and genuine required arrays allocated.
Sibling thermostat/control/demand/deadband/hybrid/day resets execute in the
genuine whole function, remain unpaired, and are not exhaustively observed.
Wrapper invocations and source-derived guard eligibility are not invented
direct observations of the member's individual invocation count.

## Reuse and actual handoff

Rust `heat_balance/initialization.rs:57-65` currently initializes temperature
and both three-slot T histories from a caller temperature, and current/average
W plus both three-slot W histories from the legacy .008 fallback. The normal
A and B paths then seed current/average W and both history projections from
their own first interpolated weather sample in `air_manager.rs:110-139`.

The temperature history mappings are XMAT[0:3] and DSXMAT[0:3], not the separate
ZTM[4] working buffer. Source selection at 3893-3900 and 6825-6832 makes that
distinction explicit. Humidity mappings are WPrevZoneTS[0:3] and
DSWPrevZoneTS[0:3]. Later source history pushes and working-buffer selection
are pinned for semantic mapping, not certified by ZON-01.

The missing boundary is a genuine mutable initialization owner for selected
constructor defaults, all four slots, working/scalar fields, reconstruction
and the environment guard. Its actual returned values must initialize the
existing solver's current/average state and first-three projections. An unused
shadow bundle or observer recomputation would not establish production use.
This card does not promise distinct MAT/ZT storage or full four-slot
synchronization after later corrections; those remain ZON-04/05/06 obligations.
The ordinary CON caller starts at the source default of 23 degrees C. A public
custom initial temperature is explicit prepared caller state retained by the
bare member, not a replacement constructor default or expanded CON input.

Normal A creates state once in `runtime.rs:304-336`; normal B does so in
`ideal_loads/coupled_runtime.rs:5943-5981`. Their default normal CLI options
start at 23 degrees C and do not execute Rust warmup. Existing fresh-state
independence tests do not cover a persistent guarded reset/rearm lifecycle,
the fourth slot, differing OutHumRat values or retained canaries.

## Pre-run packet and exact assignment policy

The three contracts pin eleven original files and 36 selected byte ranges.
Their input-only request is `zon01-helper-cases.v1`; the genuine helper output
is `zon01-helper-results.v1`. The packet awaits independent pre-run review and
original-first execution. All four card gates remain pending.

Eight fresh genuine-state sequences contain 34 ordered input-only operations:
constructor plus zero-W bare initialization; bulk current-W seed; positive
subfloor copying; negative-zero copying; selected retained-field/four-slot
canaries; repeated bare calls; guarded skip/rearm; and dirty old state followed
by explicit rearm and new bulk reconstruction. The request supplies only
finite binary64 inputs and companion hexadecimal bits. Helper-only subfloor
and negative-zero stimuli do not expand ordinary CON admission.

The helper separately copies `constructor` (before zone allocation),
`allocated_zone_constructor`, and `prepared_manager_state`. Each operation
copies `before_inputs`, `before`, and `after`, separating external input writes
from the wrapper action. Its first snapshot operation is named
`prepared_owner_snapshot`; it does not relabel the prepared state as a
constructor. Each zone row has the 24 selected fields: eighteen scalar objects
and six four-element scalar-object arrays. Actual source flags are retained;
only MyEnvrnFlag/BeginEnvrnFlag define the paired environment guard. Other
control/day flags remain prerequisite/context observations.

Every selected direct scalar assignment, four-slot array, retained canary,
zero sign, input bit and flag is compared exactly. There is no physical
tolerance on copied W or literal zero, and no tolerance may be introduced to
mask a different weather producer or observation stage.

## Ordinary native stages and bounded Rust connection

Seven unchanged CON inputs are planned for fresh ordinary original lifetimes:
A24/A72, four B limit variants at 24 hours, and BBoth72. The passive API
observer will retain genuinely named callback rows, actual context/OutHumRat,
flags and selected fields. It will call no source member as a probe.

`callbackBeginZoneTimeStepAfterInitHeatBalance` occurs at
`HeatBalanceManager.cc:199-200`, before ManageSurfaceHeatBalance and the bulk
zone reconstruction. `callbackBeginTimeStepBeforePredictor` is at
`HVACManager.cc:217`, after bulk reconstruction and the 165-170 writes
ZT=MAT, ZTAV=+0 and averageW=+0, but before GetZoneSetPoints and the member.
`callbackAfterPredictorBeforeHVACManagers` occurs at 844 after PredictStep;
selected working arrays/capacity/load fields have already evolved.

None of these callbacks exposes the pure fourteen-field member return. They
are source-stage witnesses. ZTAV or average-W stage differences cannot be
mistaken for a constructor/member mismatch. Native weather/calendar/warmup
producer equivalence is explicitly unpaired until its CLK/SYS obligations are
closed; a reference OutHumRat is never supplied to Rust's production provider.

The proposed Rust physical subset is A24/A72/BBoth24 Full/Summary: six ordinary
commands. Full must copy the actual initialization input, fourteen returned
write fields and actual current/average/three-slot handoff. Original-first
exact member/guard units establish the operator semantics; native callbacks
establish its real source context. Full/Summary ordinary-output equality only
tests observer independence. No annual/B72 Rust physics, warmup convergence,
SUR-01, ZON-02 AirPowerCap result, later history/retry, or whole EnergyPlus
equivalence is claimed.
