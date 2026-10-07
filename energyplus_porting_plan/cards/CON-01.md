# CON-01 — 대상 입력과 상태 경계 고정

- 그룹: 00 범위·계약
- 적용: A/B
- 기존 코드에 대한 작업 성격: 작업 관리 신규
- 선행 카드: 없음
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/HeatBalanceManager.cc`  
심벌: `GetProjectControlData; GetHeatBalanceInput`  
대상 블록: 선정 IDF의 활성 알고리즘·기본값만 추출. EP 전역 상태 전체를 복제하지 않고 후속 카드의 읽기/쓰기 필드를 확정.  
함께 읽을 선언/호출자: 선정 객체의 epJSON schema + 대상 .hh의 실제 사용 필드만  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/HeatBalanceManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

specs/project_contract.toml; specs/capabilities.toml; ep_run 지원 판정

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

고정 IDF/EPW, EP/Rust 커밋, 요청 출력, timestep 및 기상·스케줄 정책

## 출력·변경 상태 계약

scope.json, 입력 해시, 활성/비활성 분기표, typed state 계약, 출력별 단위·허용오차표

## 호출 시점

포팅 착수 전 1회; 입력·분기 변경 시 재승인

## 제외 범위

창호·차양기하·침기/AFN·Plant·EMS·autosizing·다중 설비는 기본 범위 밖. 본표는 전체 EP의 함수 전수 목록이 아니다.

## 단위시험

기준 입력 승인; 미지원 입력을 추가하면 사전 차단; 단순 분기 비실행을 계산 검증으로 집계하지 않음

## 연결시험

두 기준 입력 A=비공조 불투명 1존, B=no-OA 직결 IdealLoads의 실행 분기와 카드 목록 대조

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
