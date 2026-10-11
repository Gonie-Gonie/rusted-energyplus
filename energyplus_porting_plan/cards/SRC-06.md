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

The implementation removes the former EPS-zero branch, upper clamp and nonfinite floor from these selected equations, and reads the actual stored cosine. Production integration remains incomplete: InitIntConvCoeff first-call setup/ordered zone-space traversal, retained space MAT and balance reference air, initial/refresh temperature-array distinction, positive completed-iteration cadence30 after radiation, and immediate surface-term updates require the next caller unit. Kiva/windows/EMS and the full ordinary handoff remain outside this selected proof. All four full-card gates, A 14/54 and B 14/64 remain unchanged.

Evidence: [complete selected TARP comparison, application and quality](../evidence/SRC-06-tarp-inside-selected/summary.json).
