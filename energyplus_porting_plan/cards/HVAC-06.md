# HVAC-06 — Calc 말미의 출력·노드 commit

- 그룹: 09 IdealLoads
- 적용: B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: HVAC-04, HVAC-05
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/PurchasedAirManager.cc`  
심벌: `CalcPurchAirLoads; UpdatePurchasedAir`  
대상 블록: Calc 말미의 SysOutputProvided/MoistOutputProvided 및 InNode/RecircNode 기록, Off 분기. Update는 ReturnPlenumIndex=0 no-op 확인.  
함께 읽을 선언/호출자: DataLoopNode.hh의 필요 Node 필드; PurchasedAirManager.hh; Calc 원본 2590–2795 주변  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/PurchasedAirManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

ideal_loads/node 관련 처리; coupling.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

확정 급기상태·유량·존/환기 노드의 사전 상태

## 출력·변경 상태 계약

공급/환기 노드 온도·W·h·m_dot, zone 전달열·수분량

## 호출 시점

Calc 완료 시; Report 이전

## 제외 범위

리턴 플레넘 없음. UpdatePurchasedAir를 일반 노드갱신 함수로 오인하지 않는다.

## 단위시험

On/Off/0유량; enthalpy 일관성; 보존되어야 할 필드

## 연결시험

HVAC-08이 갱신된 node를 읽으며 이전/같은 timestep 혼동 없음

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
