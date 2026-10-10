# CTF-03 — 유질량 1D 구성체의 절점 분할

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 신규·보완
- 선행 카드: CTF-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Construction.cc`
심벌: `ConstructionProps::calculateTransferFunction`
대상 블록: Construction.cc490-619 selected no-source1D massive nonreverse initial Nodes/dx/rcmax/selectors/CTFTimeStep/NumHistories phase. Genuine passive copy after619 precedes matrix allocation620-659 and later retry908-936; initial and retry/final state remain distinct. Whole original method and original first three preprocessing callbacks remain unchanged.
함께 읽을 선언/호출자: Construction.hh::ConstructionProps의 AMat/BMat/CMat/DMat 관련 필드
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Construction.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/heat_balance/surface_manager/ctf_initial_discretization.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

Actual CTF01 post-conversion active dl/rk/rho/cp/lr/ResLayer and LayersInConstruct/NumResLayers, actual original material membership/prior use for reverse routing, and actual caller TimeStepZone. No Native numeric answers or replaced merged material IDs.

## 출력·변경 상태 계약

Actual initial active integer Nodes and binary64 dx, rcmax, reset NodeSource/NodeUserTemp, initial CTFTimeStep/NumHistories and caller timestep; reached owner availability and whole source outcomes remain separate. No retry/final coefficient owner is substituted.

## 호출 시점

구성체 초기화·재시도

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

열확산도·두께가 다른 층; 최소절점 분기; EP 절점 수 일치

## 연결시험

CTF-04 행렬의 차원·경계 인덱스 일치

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋:
시험 명령:
증거 경로:
최대오차/RMSE/상태 불일치:
추가 검토할 helper:

## Source amendment before CTF-03 outputs (2026-10-10)

Root FULL and other-author source peers accepted the passive initial callback, two whole Native helpers, fresh input writer, strict PE reader and OriginalDataQA sources. The actual initial capture after original619 is separate from the unchanged three preprocessing checkpoints and from later retries908-936. It copies live Nodes/dx and initial scalar/count/selector owners without evaluating them. Unit39/42 and fixed15/195 reuse only literal/input rows; production construction obligations remain dynamic. Three separate dx/initial_CTFTimeStep/caller_TimeStepZone profiles atol=rtol=0, exact class/active-prefix-shape/signedzero. Nodes/counts/selectors/history/IDs/branch/caller inputs exact; inherited18 preprocessing profiles remain unchanged separate context. Required reached absence on either/both sides fails; unselected/unreached/stopped/unknown states never numerical PASS. No tolerance changes after outputs. Actual CTF03 Native build/helper, OriginalDataQA, existing Rust baseline and candidate comparisons have not run. Scope passed; the other three gates and dependent completion remain pending.

## Actual selected initial-discretization milestone (2026-10-11)

Evidence: `evidence/CTF-03-unit/summary.json`. Genuine OriginalDataQA0 in both lanes preceded the unchanged Rust baseline and candidate application. Actual unit39/42 selected25 initial owners: 148 numeric and7644 discrete comparisons; fixed15/195 with21 construction calls selected15:45 numeric and2556 discrete comparisons. All mismatches0; all three frozen zero-tolerance metrics maximum absolute error/RMSE0 and nonfinite errors0. Unavailable context37/12 is retained without PASS. Actual initial Nodes/dx/rcmax/selectors/time/history owners are paired; later Native retry/final owners are not substituted.

Actual `cargo test --locked --workspace --all-targets`:4667 passed/0 failed/0 ignored across37 test binaries. This was before two function-local lint attributes, which preserve the pinned source predicate/branch order without body, test or policy changes. Initial Clippy exit101 and exact amendment are preserved; subsequent workspace all-target Clippy and fmt exit0. Fresh Cargo example build after these checks supplied both actual candidate executions and both actual comparisons (all exit0/source-preserving). Unused bindings01 remain archived; actual bindings02 match the final source bytes.

Selected unit gate passed. CTF01 dependency completion, CTF04 matrix/coefficient handoff, genuine retry/final owners, integration and production gates remain pending. Fixed6 CLI at this source has not yet run; no whole-card or downstream PASS is claimed.

## Actual fixed6 CLI after selected initial milestone (2026-10-11)

Evidence: `evidence/CTF-03-production-cli/summary.json`. Fresh actual `ep_cli/eplus-rs` build02 supplied A-24H, A-72H, B-NOLIMIT-24H, B-FLOW-24H, B-CAPACITY-24H and B-BOTH-24H. All6 actual exit0, source-preserving, source-order matches=true, compatibility/partial-deny/full-trace, no hours override or compare-oracle. All45 input guards remain exact. Actual fixture-demand-injection field is null for A and false for B, preserved as emitted. Build01 wrong package ep_run exit101 is preserved; it ran no CLI model and no source changes were needed. These CLI outcomes do not establish differential numerical equality, downstream coefficients, integration or production gate completion.
