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
Shared SpaceAir identity with the dispatcher, independent SurfTempInTmp,
genuine equipment/Node readiness, actual radiation/cache lifecycle, positive
completed-iteration30 refresh and immediate CTF term rebuild remain unfinished.
The ordinary dispatcher is still inactive. All four full-card gates and
A14/54/B14/64 remain unchanged.
