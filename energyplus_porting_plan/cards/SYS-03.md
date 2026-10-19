# SYS-03 — warmup→RunPeriod 상태 인계

- 그룹: 10 실행·검증
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: SYS-02, CLK-03, SCH-03, SUR-06
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/HeatBalanceManager.cc`  
심벌: `ManageHeatBalance; InitHeatBalance; CheckWarmupConvergence`  
대상 블록: warmup flag 전환과 첫 본계산의 초기화 조건. SimulationManager/Weather의 해당 caller 전환 블록만 동봉.  
함께 읽을 선언/호출자: SimulationManager.cc의 BeginEnvrn/WarmupFlag caller 블록; WeatherManager.cc::ReadEPlusWeatherForDay  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/HeatBalanceManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

heat_balance/warmup.rs; run_period.rs; ep_run

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

마지막 warmup 확정상태·calendar/weather 버퍼·환경flags

## 출력·변경 상태 계약

첫 RunPeriod에 유지/초기화되는 필드 목록과 정확한 상태

## 호출 시점

마지막 warmup 종료→본 계산 첫 timestep

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

전/후 전체 허용 필드 snapshot; 첫1/2/24시간 추적

## 연결시험

첫 SurfTempIn·CTF 상수항·MAT·W 차이가 시작되는 최초 단계 특정

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

## 현재 검증 근거와 남은 인계 확인

warmup 종료 판정의 현재 단위 근거는 [55번의 실제 RecKeep/Check 69회·45,072개 상태 비교](../evidence/SRC-06-canonical-warmup-multiplier-exterior-55/summary.json)입니다. 이 결과만으로 마지막 warmup부터 첫 RunPeriod까지의 전체 인계가 검증되지는 않습니다.

실제 마지막 EndDay 판정 뒤 thermal/demand/일별 소유자를 유지하고 WarmupFlag와 DayOfSim/DayOfSimChr를 원본 순서대로 전환해야 합니다. 다음 BeginDay와 InitHeatBalance 전후의 SurfTempIn·CTF 상수항과 이력·MAT·W, 실제 환경/clock 및 backend를 비교합니다. 보고된 시간평균 습도비로 내부 W를 추정하지 않습니다. 같은 최종 Source에서 첫 1·2·24시간 추적과 다일·연간 연결을 확인하는 계약도 유지합니다.

역사적 근거는 [실제 uncontrolled warmup 소유자 적용](../evidence/SYS-03-warmup-owner/summary.json), [원본 R 문자열 615개 비교](../evidence/SYS-03-round-format/summary.json), [선택 Report 14회·EIO 및 당시 일반 A 품질](../evidence/SYS-03-ordinary-warmup-report/summary.json)에 보존합니다. R 비교의 양의 무한대 5개 제외와 각 실행의 미검증 가족·실패 기록은 그대로 남습니다.

원본과 Source의 두 경계 전체 snapshot, 실제 Surface cadence와 같은 최종 Source의 15개 일반 실행·전체 출력 검증은 별도 과제입니다. 이 카드의 네 gate와 전체 A 14/54·B 14/64는 변경하지 않습니다.

## 전역 kickoff 관측 연결 59

[59번 증거](../evidence/SRC-06-source-live-kickoff-observer-59/summary.json)는 관측용 kickoff 입력을 진단용 복사본에서 실제 `SelectedHeatBalanceGlobals` 셀의 읽기로 바꾼 37곳을 보존한다. 원본 SimulationManager268/279와 실제 Source setup/postlude가 이미 그 전역 셀을 쓰고 있으며, 기존 과학 계산도 같은 셀을 사용한다. 35개 관측 인자와 두 opt-in 인계 관측 조건만 수정했으며 계산·플래그 쓰기·기대값·관측 스키마는 유지했다.

첫 P6 A24의 실제 정상 종료, 635,375개 수치 비트 일치와 kickoff 관측 1,007개 불일치를 구분하여 보존한다. 포맷·strict default-feature Clippy·workspace 4,975개(기존 제외 2개)는 통과했다. 수정한 관측기의 새 P7 A24 결과와 최종 연결·생산 gate는 아직 판정하지 않는다.
