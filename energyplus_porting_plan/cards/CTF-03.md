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

## 최종 네 gate 근거

| Gate | 범위별 Root 판정 |
|---|---|
| scope_review | A: 통과 / B: 통과 |
| unit_gate | A: 통과 / B: 통과 |
| integration_gate | A: 통과 / B: 통과 |
| production_gate | A: 통과 / B: 통과 |

490–619 initial Nodes/dx/rcmax/dt/history only; exact whole arithmetic module. Initial callback state is not final retry state.

실제 구현 기준은 Source59 `a2c512a246f73dc806805c6c4e277c2960fdda44`입니다. [최종 판정](../evidence/SRC-06-final-current-source-validation-60/summary.json)의 해당 카드별 범위·gate 근거와 현재 연결 결과를 참조합니다. 이전 단위 명령은 원래 근거에 보존되며 현재 바이너리 재실행으로 집계하지 않습니다.

직접 단위 근거: [CTF-03-unit](../evidence/CTF-03-unit/summary.json).

선택 본문의 정확한 행·원문 해시·역할과 직접 helper 근거는 해당 카드의 `evidence_by_scope.selected_source_extents` 및 최종증거팩의 본문 구간 원장에 기록합니다. 선언 행은 심볼 위치 식별에만 사용합니다.

과거 실패·미관측 값·제외 분기는 불변 근거에 남습니다. 수치 단위, 현재 연결, 보고값 정책, 물리 파일 직렬화의 판정을 합치지 않습니다.
