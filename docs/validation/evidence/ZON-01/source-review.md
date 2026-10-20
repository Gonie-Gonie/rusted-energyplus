# ZON-01 bounded source, ownership and final handoff review

ZON-01 is closed for the frozen A/B scope at implementation
`af3f3cb2c1ed79ac2a7887e5ca2cc6334196cf9e`, crates tree
`e94b3224a4f6aeb5e7b2b993cfe4587b6585322c`. Original EnergyPlus is
26.1.0, `6f2e40d10250a105b49966baa24d843711e61048`.

The [source](../../contracts/ZON-01-source.json),
[cases](../../contracts/ZON-01-cases.json),
[exact assignment policy](../../contracts/ZON-01-tolerances.json) and
[input-only request](../../cases/ZON-01/helper-request.json) were frozen and
independently reviewed before scientific execution. They pin eleven original
files and 36 byte ranges. Reference outputs were never Rust inputs.

## Selected source owners and distinct phases

`ZoneSpaceHeatBalanceData`, `ZoneTempPredictorCorrector.hh:102-145`, initializes
MAT/ZT/ZTAV/XMPT/XMAT[4]/DSXMAT[4]/TMX/TM2 to ZoneInitialTemp 23 degrees C.
Current, average and temporary humidity default to .01; humidity histories and
selected working fields start at +0. Module MyEnvrnFlag defaults true, while
BeginEnvrnFlag belongs to external control.

The byte-exact zone block in `HeatBalanceSurfaceManager.cc:2231-2239`
placement-news the concrete owner, then seeds current airHumRat and airHumRatAvg
from OutHumRat. Its ordinary guard is at 379-384. Only this incoming zone block
is paired: TempTstatAir, Space, surface/flux/CTF and enclosure MRT remain unpaired.

`ZoneSpaceHeatBalanceData::beginEnvironmentInit:2818-2836` writes fourteen fields
through 26 scalar stores. For all four indices, ZTM and WPrevZoneTSTemp become
+0; WPrevZoneTS and DSWPrevZoneTS receive OutHumRat. WTimeMinusP/W1/WMX/WM2
also receive OutHumRat; airHumRatTemp, tempIndLoad/tempDepLoad/airRelHum/
AirPowerCap/T1 become +0. It retains the ten selected constructor/current
fields, including MAT/ZT/ZTAV and current/average W. It does not reconstruct
the owner or impose a W floor. Source AirPowerCap W/K is distinct from Rust
air_heat_capacity_j_per_k J/K.

The genuine whole `InitZoneAirSetPoints:2621-2670` calls this member for
MyEnvrnFlag && BeginEnvrnFlag, clears MyEnvrnFlag and rearms it when BeginEnvrnFlag
is false. The helper prepared genuine one-zone, zero-controller arrays and
disabled its one-time allocation branch. Sibling control/demand/day resets
executed but remain unpaired and not exhaustively observed. Wrapper invocation
counts and guard eligibility are not direct member-call observations.

## Original-first evidence and exact unit policy

[Original-first evidence](original-first.json) binds actual native configure,
build, commands, binaries, source archives, four contracts and independent
reviews. Eight fresh genuine-state sequences executed 34 ordered operations:
seven bare calls, seven guarded calls and three bulk reconstructions, plus
input preparation and snapshots. The helper observes constructor-before-zone,
allocated-zone defaults, prepared-manager state and each operation's
before_inputs/before/after separately.

The selected payload has eighteen scalar fields and six four-element arrays.
Finite input bits, negative zero, positive subfloor W, all four slots, retained
canaries, repeated calls and guard skip/rearm are compared exactly: atol=0 and
rtol=0. No physical tolerance hides a different weather provider or source
stage. No source RHS is reconstructed by the reader.

The [final committed unit comparison](final-unit-comparison.json) passes
5,528 checks with zero mismatches: 126 snapshot pairs, 5,082 scalar-bit pairs,
252 flag pairs, 34 guard-eligibility pairs and 34 wrapper-count pairs.

Seven unchanged ordinary CON original lifetimes emitted 28,081 callback rows
with zero omissions. Physical after-init-HB/beforePredictor/afterPredictor
counts are 1,056/1,056/2,226. AfterInitHeatBalance at HeatBalanceManager:199-200
precedes surface bulk reconstruction. BeforePredictor at HVACManager:217 is
after bulk and ZT=MAT/ZTAV=0/averageW=0 writes, but before the member.
AfterPredictor at 844 observes evolved working histories/capacity/load state.
These are genuine named stage witnesses, not pristine fourteen-field returns.
Native weather/calendar/warmup alignment with Rust remains unpaired.

## Actual Rust ownership and solver handoff

The earlier public initializer exposed four scalars and four three-slot history
projections, with separate diagnostic coefficients. Its preserved
[baseline review](independent-baseline-review.json) records eight shell
snapshots, nine partial seeds and seventeen unsupported source operations.
It intentionally makes zero source-vs-legacy numerical pairs and cannot pass
scientific certification; missing initializer ownership is not fabricated as
a same-phase numerical error.

The canonical `ZoneAirInitializationState` now owns all selected 24 fields.
`ZoneAirEnvironmentGuard` persists on real HeatBalanceState. The common weather
entry executes constructor, bulk current-W seed, actual caller-temperature
preparation, genuine Rust guard and member API, then projects actual returned
values into the existing solver. Ordinary input W comes from Rust's own
provider; Begin=true is explicit Rust caller policy, not a native callback
flag. Custom T is prepared caller state retained by the member, not a changed
source constructor default or wider CON admission.

XMAT/DSXMAT first-three temperature histories remain distinct from ZTM[4]
working zeros. WPrevZoneTS/DSWPrevZoneTS first-three histories, current/average
T/W and three diagnostic coefficients are actually stored. The separate
four-slot transient owner is not claimed to remain synchronized after later
solver corrections. MAT/ZT projection is an initialization mapping only.

The scoped owner/facade/observer/export/shared-entry review archives twelve
Rust files. Full-only observation copies actual input, constructor/bulk/
before-member/after-member returns and stored projection. The shared A/B
source-order timestep hook copies actual index=0 entry before history reads.
Default-off behavior, nested/unwind restoration, own IDs/callers/context and
retained-prefix/omission semantics are reviewed; observation recalculates no
initialization values.

[Final production evidence](production-comparison.json) binds six actual normal
A24/A72/BBoth24 Full/Summary CLI commands. It passes 690 checks with zero
mismatches, including 504 constructor/bulk/caller-preparation/member snapshot
anchor bit pairs and 114 actual
stored/first-entry handoff bit pairs. Three actual initializer records and
three zero-index entry records are retained. Separately, all 480 physical clock intervals
(96/288/96) retain exact order and zero omissions. Entry context may be null or
warmup and is copied honestly, without cross-engine phase alignment. Later
history/correction is unpaired. Summary proves ordinary-output equality and
observer absence, not direct 24-field Summary observation.

## Final revision, required checks and preserved failures

Candidate `534144cb92fac8e6bf206d41067671734e436bd0` unit/source evidence remains
immutable, including its prior unit 5,528/0 and bounded production 690/0.
[Checks before lint cleanup](checks-before-lint-cleanup.json) retain its real
Clippy exit 101. [Source amendment](source-amendment.json) binds the independent
two-expression Copy-closure lint change in air_manager.rs, all other unchanged
available Rust source bytes, and fresh final02 proofs for af3f3cb2.

All five [required final checks](actual-commands.json) succeeded with unchanged
source: workspace tests (21 suite groups, 4,552 passed/0 failed/0 ignored),
Clippy --workspace --all-targets -D warnings, source-quality,
heat-balance-structure and scoped rustfmt --check. Format scope is only
air_manager.rs; no whole-workspace format assertion is made.

The archived available-source inventory has 3,656 Rust files: 3,655 exact Git
byte matches and one pre-existing CRLF-only difference in
ideal_loads/binding/scheduled_output.rs. This inventories available source,
including files not selected by a particular build. Actual Cargo emitted
binary identity is bound separately; not every available .rs file is claimed
compiled into each executable.

[Reader invocation failure](reader-invocation-failure.json) preserves the real
production command02 child exit 2: --rust-matrix was an invalid argument, stdout
was empty and scientific comparison did not start. Correct command03 changed
only that flag to --rust and used the same completed matrix/proofs. No engine
or CLI rerun, source/tolerance change or invented wrapper exit is claimed.
The [independent final review](independent-final-review.json) binds authoritative
review03, separate unit review02, static lineage and all successful proofs.
Its independent reader's earlier metadata-selector failure (assuming an absent
workspace --all-targets flag), exact selector amendment and successful review02
remain separate history in plan provenance. The subsequent output-path-only
amendment is not labeled as a separate execution receipt; authoritative
review03 itself records its actual argv and time.

## Limits retained at closure

Source Space, manager/sibling states, MRT/comfort/mixing, global counters and IO
remain unpaired. Later history pushes, correction/retry/downsteps, SUR-01,
CTF and full SYS remain pending. AirPowerCap reset ownership is not numerical
capacity/load parity. No B72/annual/per-limit Rust physics renewal, warmup
convergence, external weather/calendar alignment or whole-engine equivalence
is certified. Frozen helper subfloor/negative-zero inputs do not widen normal
CON admission. These limits remain explicit in the card and plan evidence.
