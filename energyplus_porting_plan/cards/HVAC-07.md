# HVAC-07 — 냉난방 rate·연료·energy 보고

- 그룹: 09 IdealLoads
- 적용: B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: HVAC-06, SCH-03
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/PurchasedAirManager.cc`  
심벌: `ReportPurchasedAir`  
대상 블록: 현열/잠열 부호분리, coil/zone rate, 효율스케줄, rate×TimeStepSysSec. no-OA/no-HX 출력은 비활성 계약.  
함께 읽을 선언/호출자: PurchasedAirManager.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/PurchasedAirManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

ideal_loads 보고 구현; coupled_output.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

Sen/LatCoilLoad, Sen/LatOutputToZone[W], 현재 효율, system dt[s]

## 출력·변경 상태 계약

양의 heating/cooling rate[W], step energy[J], fuel rate/energy

## 호출 시점

계산·노드 commit 후; 매 호출 overwrite

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

±500W/0;900s;η=1/0.8/≤0 fallback; 반복호출로 누적오류 없음

## 연결시험

SYS-04가 accepted step만 한번 합산; 시간가중 평균

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
