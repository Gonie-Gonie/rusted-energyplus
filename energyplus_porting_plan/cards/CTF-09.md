# CTF-09 — 계수 안정성 검사·timestep 재시도

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 신규·보완
- 선행 카드: CTF-08
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Construction.cc`  
심벌: `ConstructionProps::calculateTransferFunction`  
대상 블록: 최종 계수 뒤 CTF 합 검증, 항수·시간증분 재시도 및 종료/오류 블록.  
함께 읽을 선언/호출자: Construction.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Construction.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

신규 계수 생성 모듈 제안

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

계수·차수·CTF timestep 후보·허용 한계

## 출력·변경 상태 계약

채택 계수, CTFTimeStep, NumHistories, 종료/실패 상태

## 호출 시점

계수 생성 종료

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

첫 시도 성공/시간증분 증가/최대치 실패; 경계 직전·직후

## 연결시험

SUR-06에서 실제 NumHistories 소비; 단위 W와 상대오차 혼용 금지

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 보존한 상세 범위·정밀도 계약

Construction.cc638-981 preserves once-only allocations/Iden construction followed by each actual repeated1D assignment and exponential884 -> inverse887 -> Gamma890 -> finalCoefficients894 -> caller903-981. The initial619 timestep/history remain distinct from caller TimeStepZone; Nodes/dx are not recalculated during retry. One original assembly storage owner is consumed and retained; each Gamma invocation is fresh.

The eight new retry profiles are pre_AMat, pre_IdenMatrix, assignedAbsSumXi, assignedAbsSumYi, assignedAbsSumZi, BiggestSum, pre_CTFTimeStep and post_CTFTimeStep. Every profile has atol=rtol=0 with exact class, shape and signed zero. Actual integer counts/ordinals, owner identities/order, branch flags and reached diagnostic indices/level/text are exact. Existing preprocessing18/initial3/assembly7/exponential7/inverse2/Gamma3/private7 profiles remain literal; the new selection uses separately named all-visit handoffs and does not broaden historical first-only claims.

Sum owners exist only after the actual919-930 branch. The >18 term branch skips those owners; ratio expressions have no stored Native locals and are never observed/recomputed by the collector. Fatal939 is recorded before actual fatal unwinding; the seven-hour severe block948-977 can be reached even when convergence is true. All reached visits, partial records, copy masks, failure and omission ledgers remain required, with no record cap or invented method return.

LoopExit is distinct from whole calculateTransferFunction return. Severe break may continue into public SI stores1024-1077; public19 capacity, SI conversion/stores, reverse/2D/internal-source branches and SUR handoff remain outside this selected retry proposal. No fatal, severe, partial, uninvoked or unavailable branch receives PASS. Actual Native diagnostics remain global occupied message records; Rust selected call intents are not substituted for that backend.

## 최종 네 gate 근거

| Gate | 범위별 Root 판정 |
|---|---|
| scope_review | A: 통과 / B: 통과 |
| unit_gate | A: 통과 / B: 통과 |
| integration_gate | A: 통과 / B: 통과 |
| production_gate | A: 통과 / B: 통과 |

638–981 real repeated 1D driver/all reached visits. Pure child kernels retained; wrappers now propagate actual diagnostic failures before later calls/stores. This is semantic carry, not identical old wrapper bytes or full error-backend certification.

실제 구현 기준은 Source59 `a2c512a246f73dc806805c6c4e277c2960fdda44`입니다. [최종 판정](../evidence/SRC-06-final-current-source-validation-60/summary.json)의 해당 카드별 범위·gate 근거와 현재 연결 결과를 참조합니다. 이전 단위 명령은 원래 근거에 보존되며 현재 바이너리 재실행으로 집계하지 않습니다.

직접 단위 근거: [CTF-09-unit](../evidence/CTF-09-unit/summary.json).

선택 본문의 정확한 행·원문 해시·역할과 직접 helper 근거는 해당 카드의 `evidence_by_scope.selected_source_extents` 및 최종증거팩의 본문 구간 원장에 기록합니다. 선언 행은 심볼 위치 식별에만 사용합니다.

과거 실패·미관측 값·제외 분기는 불변 근거에 남습니다. 수치 단위, 현재 연결, 보고값 정책, 물리 파일 직렬화의 판정을 합치지 않습니다.
