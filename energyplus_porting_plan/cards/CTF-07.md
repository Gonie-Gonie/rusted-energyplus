# CTF-07 — Gamma 행렬 계산

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 신규·보완
- 선행 카드: CTF-05, CTF-06
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Construction.cc`  
심벌: `ConstructionProps::calculateGammas`  
대상 블록: 메서드 전체. 역행렬·상태전이·B 및 시간증분을 외부 계약으로 공급.  
함께 읽을 선언/호출자: Construction.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Construction.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

신규 계수 생성 모듈 제안

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

A^-1, exp(A·dt), B, dt 및 필요한 작업배열

## 출력·변경 상태 계약

Gamma 계열 행렬과 변경 작업상태

## 호출 시점

계수 생성

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

동일 EP 행렬 입력에 대한 원소별 비교; dt 변화

## 연결시험

CTF-08 최종 계수에 독립 테스트와 동일 데이터 전달

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 보존한 상세 범위·정밀도 계약

Pinned Original Construction.cc1496-1589 and caller875-894 are fully reviewed.
Original executes exponential884, inverse887, then Gamma890. Selected first
Gamma invocation1 and assembly1 borrow the same actual AExp, AInv, IdenMatrix,
BMat, rcmax, dt/history and source selector context; no replay or answer input.
The selected route remains massive nonreversed 1D with no source/sink. Original
Gamma loops/evaluation order, dt division and third-row IEEE results are retained.
ATemp is observed before its deallocation; actual Gamma1/Gamma2 return markers
remain distinct from private checkpoints. Later visits, omissions and copy failures
are all retained; unavailable or later owners do not count as passing pairs.

Gamma1/Gamma2 (3-by-rcmax) and ATemp (rcmax-by-rcmax) have separate strict profiles:
atol=0, rtol=0, exact class/shape/signed zero. Prior exponential7/inverse2,
assembly7, initial3 and preprocessing18 policies remain separate unchanged handoffs.
Inputs stay literal39 unit models/42 declared whole calls and15 production IDFs/
195 declared loader phases with45 scope guards; no expected exits or arrays.

## 최종 네 gate 근거

| Gate | 범위별 Root 판정 |
|---|---|
| scope_review | A: 통과 / B: 통과 |
| unit_gate | A: 통과 / B: 통과 |
| integration_gate | A: 통과 / B: 통과 |
| production_gate | A: 통과 / B: 통과 |

1496–1589 whole Gamma on source-produced AExp/AInv/identity/B/dt; first invocation policy; raw kernel identical.

실제 구현 기준은 Source59 `a2c512a246f73dc806805c6c4e277c2960fdda44`입니다. [최종 판정](../evidence/SRC-06-final-current-source-validation-60/summary.json)의 해당 카드별 범위·gate 근거와 현재 연결 결과를 참조합니다. 이전 단위 명령은 원래 근거에 보존되며 현재 바이너리 재실행으로 집계하지 않습니다.

직접 단위 근거: [CTF-07-unit](../evidence/CTF-07-unit/summary.json).

선택 본문의 정확한 행·원문 해시·역할과 직접 helper 근거는 해당 카드의 `evidence_by_scope.selected_source_extents` 및 최종증거팩의 본문 구간 원장에 기록합니다. 선언 행은 심볼 위치 식별에만 사용합니다.

과거 실패·미관측 값·제외 분기는 불변 근거에 남습니다. 수치 단위, 현재 연결, 보고값 정책, 물리 파일 직렬화의 판정을 합치지 않습니다.
