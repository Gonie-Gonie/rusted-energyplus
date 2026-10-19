# SYS-04 — 요청 출력과 meter의 수치 집계

- 그룹: 10 실행·검증
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: SUR-07, SYS-01
- 조건부 선행 카드: {"B": ["HVAC-07"]}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/OutputProcessor.cc`  
심벌: `UpdateDataandReport`  
대상 블록: 선정 Average/Sum, system→zone→hourly 및 요청한 energy meter 누적. 파일 직렬화 전체는 별도.  
함께 읽을 선언/호출자: OutputProcessor.hh의 해당 변수·meter 구조; 원본의 선택 집계 블록  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/OutputProcessor.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/output.rs; ideal_loads/coupled_output.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

accepted step 값·dt·Average/Sum 종류·meter 연결

## 출력·변경 상태 계약

시간가중 평균, energy 합, 선정 hourly/monthly/run-period 집계

## 호출 시점

accepted system/zone timestep 및 보고경계

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

동일/상이한 dt; 월말; 부호상쇄; warmup 제외; double-count 방지

## 연결시험

각 step energy 합=시간별 합=요청 기간합, EP timestamp 일치

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [ ] 원본 범위와 입출력·변경상태 계약 확정
- [ ] 단위시험 통과 및 실제 활성 분기 확인
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

현재 검증: 실제 GetZoneData가 만든 출력 owner에서 진짜 요청 읽기·시간 포인터·5개 스칼라 등록을 거쳐 전체 UpdateDataandReport 37회와 ResetAccumulationWhenWarmupComplete 1회를 지속 호출했습니다. 35개 live 변수·7개 보고 주기·누적·meter·보고 스트림과 95개 관측 snapshot을 비교해 상태값 34,321개 원시 비트가 일치했습니다. 입력 calendar·dt·외부 스칼라만 공급하며 집계 상태나 기대 출력을 복원하지 않습니다.

[원본 범위·실제 명령·입력/소스 해시·독립 검토·실패 기록](../evidence/SRC-06-canonical-output-processor-air-history-54/summary.json)에 모았습니다. 두 그룹은 같은 Source snapshot에서 실행했습니다. 이 단위 비교로 카드의 네 gate를 일괄 완료 처리하지 않습니다.
