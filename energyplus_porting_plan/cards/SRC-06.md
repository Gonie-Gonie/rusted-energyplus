# SRC-06 — TARP 실내 대류

- 그룹: 05 열원·외부 경계
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: GEO-02, PSY-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/ConvectionCoefficients.cc`  
심벌: `CalcASHRAETARPNatural; CalcASHRAEDetailedIntConvCoeff; InitIntConvCoeff`  
대상 블록: TARP 수직·안정/불안정 경사 분기와 strict 하한 대입. 선택 원본에는 상한 clamp가 없음. InitIntConvCoeff에서는 해당 선택 경로만.
함께 읽을 선언/호출자: ConvectionCoefficients.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/ConvectionCoefficients.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

heat_balance/convection.rs; inside_convection.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

실내 표면온도·참조공기온도[°C], 경사

## 출력·변경 상태 계약

실내 h[W/(m²K)] 및 참조온도 snapshot

## 호출 시점

실내 반복의 정해진 계수 갱신 시점

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

양/음/0 ΔT; 수직·수평; 안정/불안정; clamp

## 연결시험

SUR-05에서 EP와 같은 반복 회차에 갱신

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [ ] 원본 범위와 입출력·변경상태 계약 확정
- [ ] 단위시험 통과 및 실제 활성 분기 확인
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋:  
시험 명령:  
증거 경로:  
최대오차/RMSE/상태 불일치:  
추가 검토할 helper:  

## Bounded Source contract (reviewed before application)

`ConvectionCoefficients.cc:1907–1975` and the three inline correlations in
`ConvectionCoefficients.hh:394–460` are complete direct-unit Source bounds.
Natural uses one Tsurf−Tamb subtraction, exact DeltaTemp==0 or cosTilt==0,
then strict same-sign enhanced versus remaining stable expressions. Detailed
non-Kiva passes negative actual Surface.CosTilt and only applies strict
Hc<LowHConvLimit assignment. There is no upper clamp, EPS predicate or nonfinite
floor in these selected bodies; HighHConvLimit is not an operand. Default lower
limit0.1 is Source-owned at DataHeatBalance.hh1790 and fixed by CON01 admission.
The direct API takes an explicit actual lower-limit owner for diagnostic cases.

The accepted inside-consumer change borrows initialized geometry.cos_tilt instead of
recomputing tilt degrees. Exact geometric90 degrees has a stored nonzero cosine;
it must not be silently changed to the exact-zero vertical branch. Existing tests
preserve unrelated assertions and add real stored-owner independence checks.

The full selected caller is InitIntConvCoeff141–391: user setup/flags, true
Zone.spaceIndexes/space HT traversal, spaceHeatBalance.MAT, optional resimulation,
representative filtering, SetByZone/ASHRAETARP non-window dispatch, lower bound,
EMS and window tails. Only CON01-admitted non-Kiva/no-window/no-override branch
is the proposed arithmetic scope; a direct call does not certify this traversal.
Real entry is HBSM473 and refresh only positive InsideSurfIterations divisible by
ItersReevalConvCoeff30 at HBSM7944–7946/8824–8826 (DataHeatBalSurface.hh70).
Current Rust per-iteration/reference-map fallbacks remain a documented caller
gap and are not asserted equivalent or changed by this held unit proposal.

Native direct inputs/actual calls must be frozen and released before math applies.
At the proposal stage no tests had been executed and no numerical Native answers
had been supplied. The actual selected proof is recorded below; gates remain unchanged. Full caller/space-MAT/cadence and SUR05 integration remain pending.

## Actual selected Natural/Detailed math and quality

The unchanged-core Native factual baseline was released before applying the Rust math. The selected public Natural and non-Kiva Detailed APIs were genuinely invoked on 28 literal inputs and 60 ordered calls. All input/initial full-owner joins passed: 1,064 input joins, 960 full pre-owner and 988 endpoint checks, zero mismatches. The 60 paired-call rows include 54 fully finite endpoints that match IEEE bits/class/signed zero exactly, plus six generated nonfinite endpoints checked by class and excluded from numerical PASS. All 34 required-unavailable rows remain explicit: six generated nonfinite observations and 28 Native-only constructor/high-limit/Objexx/backend/prefix contexts. No tolerance or formula was changed to admit a result.

The first comparer stopped before any pairs because Rust's Windows verbatim path spelling differed from Python's resolved spelling. The bounded successor checks filesystem same-file identity against the fully authenticated input; input bytes/size, every literal owner, the original prospective policy02 and all numerical rules are unchanged. The failed comparison and fresh successful comparison remain retained. No target rerun or floating point control change was used for this repair.

Actual Rust Source 8aa752f4 has 3,929 members; 3,926 unrelated members remain byte-exact across application. Formatter review accepted only the import move, line wrapping and optional trailing commas. Four genuine Source-identical checks passed: fmt, Clippy with warnings denied, focused 346/0/2 and workspace 4,973/0/2 (passed/failed/ignored). The actual standalone dev Cargo executable and argc3 execution, Native provider/core/PE audit, complete observations and uncapped ledgers are retained.

The implementation removes the former EPS-zero branch, upper clamp and nonfinite floor from these selected equations, and reads the actual stored cosine. At that selected-math milestone, production integration remained incomplete: InitIntConvCoeff first-call setup/ordered zone-space traversal, retained space MAT and balance reference air, initial/refresh temperature-array distinction, positive completed-iteration cadence30 after radiation, and immediate surface-term updates require the next caller unit. Kiva/windows/EMS and the full ordinary handoff remain outside this selected proof. All four full-card gates, A 14/54 and B 14/64 remain unchanged.

Evidence: [complete selected TARP comparison, application and quality](../evidence/SRC-06-tarp-inside-selected/summary.json).

## Reviewed dispatcher/setup contract (before its Rust application)

The next unit covers the actual no-user first call of InitIntConvCoeff141–391,
GetUserConvCoeffs733–1672, unconditional static setup2800–3099 and inactive
radiant readiness3101–3149, with genuine ScanForReports1130–1469 and roof
geometry15576–15686. It admits only explicit six-family zero input counts,
opaque TARP surfaces, exact complete zone/space/surface associations, positive
literal static geometry and actual readiness flags. Missing/unsupported owners
stop explicitly. Source normalization's computed zero roof-edge length is also
unavailable, including underflow from distinct finite endpoints.

Persistent owners retain separate space MAT/ZT/ZTAV, true constructor/setup
flags, actual zero user-family allocation state, Int/Ext geometry and unchanged
classification/reference-air fields. The caller borrows a complete named
SurfTempInTmp or SurfTempIn array, reads MAT once per actual ordered space and
reuses Detailed plus the caller's second strict lower-limit check. The selected
space reset/correction-copy methods are explicit; ZTAV is not overwritten by
the selected disabled-space correction copy. Details/advanced geometry EIO
stops after the genuine static/Scan prefix; backend failures retain their cause.

Before applying Rust, the unchanged Native helper must actually process the
frozen physical no-user IDF and capture first/repeat/environment/readiness,
reverse-space-order, signed-zero, zero/small delta, empty-HT and explicit/default
TARP cases. Source, input, core, linked providers, full output and all owners
must be authenticated before pairing. Finite coefficient endpoints require
exact binary64 equality; generated nonfinite fields remain diagnostic/unavailable.
Static/Scan/backend observations follow a separately frozen prospective scope.
No inner Native callback trace or ordinary geometry/MAT producer is inferred.

This unit does not activate the ordinary surface manager. Actual geometry/raw
input binding, independent temperature-array production, reset/correction hooks,
post-radiation completed-iteration cadence30, immediate balance-term refresh and
ordinary handoff remain required. No new full-card or A/B gate is complete.

## Actual selected dispatcher initialization and retained owners

The genuine unchanged-core Native helper processed the frozen physical no-user IDF in eight cases and invoked InitIntConvCoeff20 times. Rust independently parsed and compiled the same input-only request, then invoked the selected dispatcher20 times. All700 literal binary64 input cells and complete selected initial/pre-owner joins passed:24,033 input checks,8,963 pre-owner checks and9,083 endpoint checks, with zero mismatches, required unmatched or required unavailable observations. The20 complete paired boundaries retain264 optional Native-only contexts outside this selected proof; those contexts are not counted as PASS. The frozen finite policy uses exact bits/class/signed zero and zero numerical tolerance.

Applied owners preserve distinct space MAT/ZT/ZTAV, first-call/setup and readiness flags, six explicit absent user families, ordered zone-space HT traversal, actual named temperature arrays, geometry setup, Scan/report state and backend prefixes. The selected disabled-space correction copies MAT and ZT without overwriting ZTAV. Four genuine checks passed on Source4c02bed5:fmt, Clippy with warnings denied, focused354/0/2 and workspace4,981/0/2. The two ignored observer tests remain unchanged. Two earlier Clippy failures, the authenticated owned-path restoration, a comparer custody failure before any pairs, and the bounded Source repairs remain retained rather than replaced by a successful run.

[Complete selected dispatcher evidence](../evidence/SRC-06-tarp-dispatcher/summary.json) retains all four uncapped gzip ledgers and a finite exact-byte archive of426 rows/289 objects. Whole Source payloads and large executable/map/library payloads remain provenance references; this archive does not claim transitive replayability. The selected dispatcher is not yet activated by the ordinary surface manager. Actual raw geometry producers, ordinary space MAT/reset/correction, independent SurfTempInTmp/SurfTempIn/old-temperature owners, post-radiation positive completed-iteration cadence30, immediate balance-term updates and ordinary warmup/handoff still require integration and Native comparison. All full-card gates and A14/54/B14/64 remain unchanged.

## Bounded ordinary geometry producer prerequisite (reviewed before Rust application)

Retain every actual successful GEO03 return once at its existing caller, while
leaving the Volume consumer and error timing unchanged. The next selected owner
unit covers no-pop cyclic Surface.Perimeter, base Width/Height, complete six Zone
bounds and opaque exterior gross-wall/window reset and ordered additions.
Perimeter follows the actual pow(delta,2) provider path; Width/Height follow the
separate Vectors::VecLength multiply path. No rectangular shortcut is admitted.

Before application, genuine GetBuildingData/SetupZoneGeometry returned owners,
exact input and Zone/Space/Surface associations, diagnostics and retained prefixes
must be captured and independently reviewed. No-transform/no-subsurface/complete
opaque input admission is required; missing correction branches are unavailable.
Empty Space uses actual 0/-1 pointers, not an injected 0/0 range. Its separate
branch needs genuine input evidence; a typed unit assertion is not Native proof.

Whole setup/centroid/scratch, vertex disposal, nonopaque corrections, ordinary
space MAT/temperature production, cadence30 and handoff remain unfinished.
This prerequisite supplement promotes no card gate and changes no A/B counts.

## Actual bounded ordinary opaque geometry producers

The unchanged-core Native loader genuinely returned from all 13 selected public calls. Rust parsed and compiled the same physical A-24H input, then called its real initializer once. Returned GEO03 owners are retained once at that caller; the existing Volume consumer is unchanged. Selected new owners cover cyclic no-pop perimeter, Width/Height, six zone bounds and ordered opaque exterior area accumulation. This later proof does not expand the historical GEO03 Volume-only certification.

The actual comparison status is `pass-available-selected-ordinary-geometry-only`. All input-join/pre-owner/selected-endpoint/point-helper counters are copied from the genuine comparison: `{"input_join_comparisons":2255,"pre_owner_comparisons":0,"selected_endpoint_comparisons":36,"point_helper_comparisons":0}`; mismatches 0, required unavailable 0, optional unpaired 19. Optional Native-only contexts remain outside PASS. Full input families/values/order and construction/zone/space/surface identities are joined before selected endpoints, using exact binary64 bits/class/signed zero with zero tolerance. The 13 extra null families created by Original input getters are explicit unpaired metadata, not supplied input objects.

Source47ebf61d passed fmt, Clippy with warnings denied, focused354/0/2 and workspace4988/0/2. The same two ignored observers remain unchanged. Earlier Native compile failure, generated-metadata restoration, input-metadata admission preflight, the two factual-checker transport/Scan-shape failures, and the first comparison, which stopped before any pairs on a recorder-descriptor error, are retained along with the first evidence-publication metadata-shape failure. These transport repairs use the same closed Native/Rust captures without target reruns. The three uncapped ledgers were fully decoded and checked independently; the finite archive stores its selected bytes exactly, while large executable/map/object and whole Source payloads remain provenance references rather than a transitive replay claim.

[Actual bounded geometry evidence](../evidence/SRC-06-ordinary-geometry/summary.json). Whole SetupZoneGeometry, nonopaque corrections, ordinary persistent MAT/reset/correction, independent SurfTempInTmp/SurfTempIn/SurfTempInsOld, radiation-before-positive-completed-iteration30 refresh and immediate cached term rebuild remain separate work. The ordinary dispatcher is not activated by this geometry prerequisite. Weather03 and its absent-site/station producer extension are now applied; the [actual bounded weather/cold comparison](../evidence/SRC-05-weather-cold/summary.json) records the available selected results. Complete ordinary reachability and handoff remain pending. All four SRC-06 gates and A14/54/B14/64 stay unchanged.

## Actual bounded ordinary temperature owners and convection inputs

The existing ordinary A caller now retains independent current Zone ZT,
Space MAT/ZT/ZTAV and the complete SurfTempInsOld array. Air owners reset at
BeginEnvrn; the existing solver return is retained, MAT commits before reporting
and humidity, and disabled-space MAT/ZT copies after correction without changing
ZTAV. The complete associated SpaceMAT map supplies inside reference air, and
Old copies In before the existing radiation stage. Exact complete associations,
cardinality and retained input validity are checked before writes.

One genuine A-24H Native engine run captured all 58,039 selected events. Root
and an independent reviewer validated every observed selected assignment pair:
4 resets, 12,150 reference-air pairs, 12,447 Old copies, 2,209 MAT commits and
2,209 disabled-space copies, with zero mismatches. The genuine 19-case fixture
then exercised the Rust private owner operations using actual BEFORE states and
exact AFTER binary64 bits/classes. It covers allocation and observed kickoff,
warmup and run phases. The first checker failure is retained: Original's
constructor reference flag is Invalid(-1), whose default getter branch returns
SpaceMAT. The reviewed successor admits -1/0 for this selected branch and reuses
the same capture; no Native rerun or numerical rule change occurred.

Actual raw counts for the six user-convection object families and the unique
HeatBalanceAlgorithm N2/default0.1 now flow through the ordinary caller into
retained owners. Unsupported counts/types and invalid retained values stop
explicitly. One 28-case input table and the existing production regression cover
these rules; nondefault0.375 proves retention only. Final Source `fb798c3b`
passed fmt, workspace Clippy with warnings denied and one workspace test run:
4,969 passed, 0 failed, 2 existing ignored. Earlier owner-only focused checks
passed36 and2; they were not repeated after the final workspace suite.

Plan/view, documentation, spec, source-quality and false-conformance guards also
passed. The global file-size guard failed on 19 existing files, all byte-exact
with parent commit8213151e. No file in this temperature/input change exceeds its
hard limit; the existing structural debt is not reported as a successful gate.

[Bounded temperature owner evidence](../evidence/SRC-06-ordinary-temperature-owners/summary.json)
uses three files: summary, indexed manifest and finite evidence ZIP. The complete
compressed Native capture and failure history are retained. Large binaries,
maps, cores and whole Source payloads remain provenance references, with no
transitive replay claim.

This proof does not compare zone solvers: actual Native uses ThirdOrder while
the existing Rust A admission remains SimplifiedAnalytical. Native observes one
Space and reference flag-1 only; multiple-space and explicit0 actual coverage,
full ZTAV averaging and every Native invocation's Rust replay are not claimed.
Independent SurfTempInTmp, genuine equipment/Node readiness, actual
radiation/cache lifecycle, positive
completed-iteration30 refresh and immediate CTF term rebuild remain unfinished.
The ordinary dispatcher is still inactive. All four full-card gates and
A14/54/B14/64 remain unchanged.

## Actual shared SpaceAir borrow and complete selected Rust regression

The selected dispatcher now borrows the caller's SpaceAir owner. It no longer
allocates a second MAT/ZT/ZTAV array; the ordinary owner keeps its existing
allocation. The complete validated SpaceMAT mapping is shared with the ordinary
inside balance. The selected observer likewise owns one external array with the
same seeds and write order. An existing two-space test now changes the borrowed
MAT between calls and checks the affected coefficient and reference-air mapping
while retaining the other space and independent ZT/ZTAV values. No test was added.

Final Source `02b03300` passed fmt, workspace Clippy with warnings denied and
one workspace test run: 4,969 passed, 0 failed, 2 existing ignored. A genuine
current Cargo artifact then processed the unchanged eight selected input cases
and made all 20 dispatcher calls. The complete old/new Rust observations match:
16,398 objects, 3,067 arrays and 57,930 exact leaves, including binary64 signed
zero, owners, branches, stops, unavailable labels, IO and callbacks. Complete
stderr and actual exit also match. A prospective list of 53 invocation, current
Source SHA/size and physical-file-path leaves is authenticated separately; no
scientific owner or output is excluded. The uncapped difference ledger is empty.

This is a current Rust regression against the previously accepted Native-paired
Rust capture. No new Native execution or direct Native comparison was performed.
The historical 264 optional Native-only contexts remain outside PASS. The
[shared SpaceAir evidence](../evidence/SRC-06-ordinary-shared-space-air/summary.json)
retains the finite Source, execution and whole-result comparison records in three
files; large binaries and whole Source payloads remain provenance references.

The ordinary dispatcher remains inactive. Independent SurfTempInTmp, actual
equipment/Node readiness, radiation/cache lifecycle, positive completed-
iteration30 refresh, immediate CTF term rebuild and full ordinary solver/handoff
comparison remain pending. All four SRC-06 gates and A14/54/B14/64 are unchanged.

## Actual independent ordinary inside scratch temperatures

The ordinary owner now allocates a separate complete represented-HT
SurfTempInTmp array. BeginEnvironment resets Tmp to23 without resetting Old.
Complete surface identities, associations and CTF indices are validated before
any writes. The existing inside solver keeps its operands, call and arithmetic;
its returned value is retained in Tmp and immediately copied to In through the
validated borrowed owner. No second solve or per-surface allocation is added.

One genuine A-24H Native run produced322,474 selected events. The complete
stream checker validated74,682 primary solve pairs and74,682 immediate-copy
pairs, one allocation, four resets,2,025 initial-convection pairs and four
positive completed-iteration30 refresh pairs, with zero factual mismatches.
Readiness and caller order were observed, not implemented by this unit. The
complete raw stream is retained as a gzip whose decoded length, SHA and CRC
were checked against the original capture. The build, provider addresses,
normal outputs and earlier preconfigure failure remain in the evidence.

Seven unchanged actual cases from the138-case Native fixture extend one
existing private replay test: allocation, two resets and four complete primary
HT groups covering kickoff, first/later warmup and run. The24 adjacent
solve/copy triplets supply actual solved values only as test inputs to the
private copy operation. Replay checks exact copied bits and independent owner
retention; it does not compare CTF equations. Initial Native In and HConv are
not manufactured as effects of the Rust BeginEnvironment operation.

Final Source `21d06c68` passed fmt, workspace Clippy with warnings denied,
the source-quality guard and one workspace test run:4,969 passed,0 failed,
2 existing ignored. The affected ordinary filter passed36 tests. Existing
tests were extended; no test functions or binaries were added. No changed
file exceeds its source-size limit; the previously recorded global size debt
is not claimed as a passing global gate.

[Inside scratch-temperature evidence](../evidence/SRC-06-ordinary-inside-tmp/summary.json)
contains the summary, indexed manifest and finite ZIP with205 selected
artifact references. Large executable, map, core and provider payloads remain
provenance references, without a transitive replay claim.

Independent represented-HT Tmp allocation/reset/copy is complete within this
bounded owner scope. Native non-HT/TDD/window branches, real equipment/Node
readiness, radiation/cache lifecycle, positive completed-iteration30 refresh,
immediate CTF term rebuild, full ZTAV and ordinary solver/handoff comparison
remain pending. The ordinary dispatcher remains inactive. All four SRC-06 gates
and A14/54/B14/64 are unchanged.

## Actual consumed radiation LU and inverse

The existing private radiation inverse now reduces its caller's coefficient
buffer in place and returns the inverse using the Original LU index and
arithmetic order. The existing ScriptF caller passes the same buffer mutably.
The former augmented Gauss-Jordan operation and its numerical epsilon skips
are removed. This change does not activate ordinary interior radiation.

One genuine A-24H Native execution produced48 complete selected events:
four admitted N6 inverse calls,24 pivots and no swaps. The complete factual
checker and an independent review validated all events, matrix shapes,
transactions, parent clocks and returned buffers. The observations include
one kickoff and three first-day warmup calls; no run or later-warmup call
was reached. The actual pre-call inverse buffer is unallocated, and its
contents are not invented.

One conventional private Rust table supplies each actual entry coefficient
matrix to the consumed helper and compares all288 returned coefficient and
inverse scalars as binary64 bits. All four cases are admitted with no
exclusions: absolute, relative and ULP tolerance are zero, including signed
zero. The existing seven tests in the module remain intact.

Final Source `779fc206` passed fmt, workspace Clippy with warnings denied,
source-quality and one workspace test run:4,970 passed,0 failed,2 existing
ignored. The affected ScriptF filter passed4 tests. No extra test binary or
observer framework was added. Both affected Rust files are below their
source-size limits; no global size-debt gate is claimed.

[Bounded radiation LU evidence](../evidence/SRC-06-ordinary-radiation-lu/summary.json)
retains186 finite artifact references in the summary, indexed manifest and
CRC/SHA-verified evidence ZIP. Historical Native Source identities and the
formatter's actual Source change remain recorded. Large executable, map,
core, object and provider payloads are identity/provenance references only,
without a transitive build or replay claim.

Pivot swaps, first ties, singular/exact-zero and tiny-pivot numerical coverage
remain uncertified. The coefficient/ScriptF producers, actual enclosure
ownership, cache lifecycle, EIO rows and ordinary radiation callers remain
unfinished. Equipment/Node readiness and full ordinary solver/handoff parity
also remain pending. All four SRC-06 gates and A14/54/B14/64 are unchanged.


## Actual bounded geometry equipment getter and sole Config owner

The ordinary route now validates the finite 29-positive-family and field admission before equipment writes, validates actual surfaces, runs the bounded GetZoneEquipment lookup/allocation chain, then calculates volume. Successful state assembly retains that equipment owner and independent physical Node allocation. The dispatcher borrows the sole Config owner's wall/ceiling/floor flags; ordinary dispatcher activation remains pending. This follows SurfaceGeometry.cc lines 290 (GetSurfaceData), 298 (GetZoneEquipment), and 548 (CalculateZoneVolume) for the admitted subset; full GetSurfaceData warning/scratch parity is not claimed.

One conventional table reuses eight complete historical Native objects: seven count/find inputs at source indices 0, 1, 2, 3, 6, 7 and 8, and the single actual GetZoneEquipment before/after pair at index 13. It compares represented flags, allocation shapes, Config/list/availability fields and independent Node state. Supplied absent versus explicit-empty and lookup insert-or-retain transitions are separate Source checks; there is no Native InputProcessor family creator/prepresence claim. Existing diagnostic routes and the three prior temperature/Tmp/LU fixtures remain outside this change's Native expectations.

Actual checkpoint checks on Source eb2d06b7ae9b3a85fe058ffb77dcd5dbb38bc8cff68ce8c7767e71a326b2f3ae passed: formatting check, source quality and 41 affected tests. Clippy and the full workspace are consolidated with the following radiation integration; this checkpoint claims neither run. The complete released-Rust dispatcher regression retained all eight cases and 20 calls: 16398 JSON objects, 3067 arrays and 57930 exact leaves, with 53 prospective execution-metadata joins and zero differences. This is a whole selected Rust regression; its accepted historical Native pairing is provenance, not a new direct Native comparison. Prior failed or interrupted executions remain in the finite packet.

[Bounded getter, sole Config and selected regression evidence](../evidence/SRC-06-ordinary-geometry-equipment/summary.json) archives the selected Source/diffs/literal Original ranges, eight whole Native objects and actual QA/build/capture/regression records and logs. The full 138-case fixture, large raw Native capture, binaries/maps/core/providers and transitive historical Source payloads remain identity references. Full InputProcessor/scratch parity, gain registration/equations, full Config/Node ABI, availability/HVAC Manage and Sim tails, AnyEMS readiness, ordinary dispatcher/cadence, radiation enclosure/cache/caller and complete zone solution/handoff remain unfinished. All four SRC-06 gates and A14/54/B14/64 stay unchanged.

## 실제 FixViewFactors / CalcScriptF 선택 수치 생산자

Original 순서의 FixViewFactors와 무차원 CalcScriptF 생산자를 연결했다.
실제 A-24H Native 실행의 전체 58개 이벤트 중 Fix 1회와 ScriptF 4회를
모두 채택하여 F, ScriptF, EMISS의 204개 출력 binary64 비트를 비교한다.
절대·상대·ULP 허용오차는 모두 0이며 signed zero도 정확히 일치한다.
관측된 호출은 Fix kickoff 1회, ScriptF kickoff 1회와 첫 warmup day 3회다.
후속 warmup/run, pivot swap/tie, singular/tiny 및 지원하지 않는 경고·상태
분기의 수치 검증은 아직 없다. 기존 LU 테스트와 과거 fixture는 보존했다.

Probe 입력은 전체 zone/surface 연결과 유한성을 먼저 검증하고, 모든 zone의
결과를 임시 저장한 뒤 성공할 때 함께 반영한다. 지원 불가 오류는 실제
surface/timestep/warmup/run/PurchasedAir 호출을 따라 전달되며 public manual
step API도 Result를 반환한다. Original 초기화가 N1에서 Fix/ScriptF를
건너뛰는 규칙을 따라 단일 표면의 교환량을 0으로 처리한다. 이 N1 처리는
소스에 근거하며 Native N1 관측을 주장하지 않는다. N2/N3은 지원 불가다.
Timestep 객체가 존재하고 필드만 생략되면 실제 schema 기본값 6을 사용한다.
객체 전체가 없을 때의 Original/compiler 기본값 차이는 지원 불가로 남긴다.

최종 Source `78050bebaad0910a239a6b2d0d289dffb1638cf54e6f289c1e04e8ffd84f8df2`에서
fmt, workspace Clippy(-D warnings), source-quality와 전체 테스트가 통과했다:
18개 그룹, 4,974 passed, 0 failed, 기존 ignored 2개. 이전 focused 6개 통과는
Source `690a7374`의 과거 실행이며 최종 Source 검증으로 대신하지 않는다.
수정 전 workspace 실패(exit 1), Clippy 실패(exit 101), runtime 진단 성공
(exit 0)의 실제 로그와 Source도 함께 보존한다. 최종 workspace 실행은
204개 비트 검증과 N1·기본값 회귀 및 이전에 실패한 CLI 경로를 포함한다.
새 테스트 binary나 관측 framework는 추가하지 않았다.

[실제 수치 생산자 요약](../evidence/SRC-06-ordinary-radiation-producer/summary.json),
[파일 색인](../evidence/SRC-06-ordinary-radiation-producer/manifest.json),
[검증 기록 ZIP](../evidence/SRC-06-ordinary-radiation-producer/evidence.zip)은
288개 artifact 참조와 CRC/SHA/크기를 다시 검증한 242개 ZIP member를 담는다.
대형 Native 실행 파일·map·core·provider는 기존 provenance 참조이며 이번
변경에서 다시 읽거나 전체 빌드 재현을 주장하지 않는다.

실제 enclosure/cache/first-call/report/sigma 소유권, ordinary radiation 활성화,
실제 반복 cadence와 전체 zone 해법·handoff 일치는 아직 미완료다.
네 SRC-06 gate와 A14/54, B14/64는 그대로 유지한다.

## 실제 태양복사 초기화 경로와 부분 상태 보존

전체 ordinary raw 입력 검증에서 얻은 radiation 입력과 실제 advanced-report
선택을 초기화 경로에 전달한다. 실제 내부 재료의 solar absorptance를 외부
재료의 값과 독립적으로 보존하고, construction/surface 준비가 끝난 뒤
현재 convective/radiant gain 소비자보다 먼저 InitSolar를 호출한다.
전체 Original GetInternalHeatGainsInput 구현 완료를 주장하지 않는다.

선정된 단일 Zone·implicit Space의 Wall/Floor/Roof enclosure는 실제 순서와
surface 연결로 배열·F·backpointer를 생성한다. 실제 report scan, EIO와
error backend를 빌려 쓰며, 중단 시 도달한 부분 owner를 caller reports에
남긴다. 성공한 owner는 State로 이동한다. 기존 None 경로는 유지한다.
기존 approximate API가 모아 반환한 경고는 NumericDomain으로 중단하여
Original의 중간 경고·부분 수치 결과처럼 재생하지 않는다.

기존 초기화 테스트 안의 두 authored 입력으로 N1 성공, 내부0.25/외부0.75
흡수율 구분, 실제 배열·연결·owner 이동, 성공 후 gain9W+3W를 확인한다.
평행 Wall 4개 입력에서는 할당된 +0 F·연결·초기화 미완료 상태를 보존하고
gain 조회 전에 중단함을 확인한다. 이 검사는 Rust 경로 회귀이며 새 Solar
owner/EIO의 Native 수치 동등성 검증은 아직 아니다. 기존 실제 Native5회와
204개 비트 검증을 유지하고, 같은 fixture의 실제 Fix diagnostic4개 비트와
iteration 출력 검사를 추가했다. 테스트 수·binary·framework는 늘리지 않았다.

최종 Source `90b489e38f4b41d61ff6cd726ea22e03f823179163db04777f5e1107f068d2ee`에서
fmt, workspace Clippy(-D warnings), source-quality와 전체 테스트 1회가 통과했다:
18개 그룹, 4,974 passed, 0 failed, 기존 ignored2개. 개별 테스트의 중복 실행은
추가하지 않았다. 변경된18개 Source, 전체 direct/formatter diff, Original18개
구간, 독립 검토와 실제 QA 로그는
[초기화 검증 요약](../evidence/SRC-06-ordinary-solar-initialization/summary.json),
[색인](../evidence/SRC-06-ordinary-solar-initialization/manifest.json),
[119개 member의 ZIP](../evidence/SRC-06-ordinary-solar-initialization/evidence.zip)에 보존한다.

실제 SetupSimulation kickoff는 기준 입력의 DesignDay2개도 방문한다.
현재 없는 DesignDay 생산자, 실제 InitSurface 전체 suffix, 공유 firstTime,
radiant enclosure·ScriptF cache·반복 caller와 ordinary 활성화는 계속 포팅한다.
SRC-06의 네 gate와 A14/54, B14/64는 변경하지 않는다.
