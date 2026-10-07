# CLK-01 — 달력·RunPeriod·day type

- 그룹: 02 시간·기상
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: CON-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/WeatherManager.cc`  
심벌: `GetRunPeriodData; SetupWeekDaysByMonth; calculateDayOfYear; isLeapYear`  
대상 블록: 선정 RunPeriod의 날짜·요일·윤년 정책. DST/holiday를 사용하는 입력이면 관련 날짜 해석을 이 카드에 명시적으로 추가.  
함께 읽을 선언/호출자: WeatherManager.hh의 DayWeatherVariables/RunPeriod 관련 필드  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/WeatherManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/time_axis; 관련 calendar 구현

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

시작/종료일, 시작 요일, EPW 윤년 정책, 활성 special-day/DST 정책

## 출력·변경 상태 계약

날짜·요일·일련일·day type·보고시각 열

## 호출 시점

환경 초기화와 일자 전환

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

첫날/마지막날/월말; 윤일 포함 여부; 활성 holiday 정책

## 연결시험

SCH-03의 조회 날짜와 CLK-03의 EPW record index 분리 검증

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

## 착수 시 확인한 범위

현재 작업은 CON-01의 고정된 15개 입력과 2013년 단일 ordinary weather
RunPeriod에 한정한다. 입력의 weather holiday/DST 사용 정책은 Yes이지만,
선정 EPW에 활성 holiday/DST 정의가 없어 실제 override는 없다. 윤년·세기년
helper 경계시험은 단위시험이며 생산 입력의 연도 범위를 확장하지 않는다.

검증 대상은 계산에 전달되는 civil 날짜·요일·day type, Gregorian/weather/
always-leap schedule ordinal, hour와 canonical zone interval이다. Gregorian
year와 EPW의 TMY record year를 구분한다. A의 원본 weather 환경 번호 3과
B의 번호 1은 비활성 DesignDay 선언 수가 다른 결과이며, Rust가 준비한
환경 index의 의미와 별도로 검증한다.

원본 `SimulationManager.cc:532`의 Weather 갱신 뒤, `HeatBalanceManager.cc:190`
이후의 `begin_zone_timestep_before_init_heat_balance` 공개 callback에서 현재
달력을 관찰한다. `currentTime`과 `minutes`는 이 시점에도 이전 system 상태를
반영할 수 있으므로 canonical zone interval로 취급하지 않는다. 보고 후 API
필드도 별도 보존한다. 원본 연간 실행의 마지막 12월 31일 보고에서는
CalendarYear가 2014로 갱신된다. 이 mutation과 보고 serializer는 SYS-04의
범위이며, 계산 전 현재 날짜의 2013년과 섞어서 비교하지 않는다.

전체 source/helper/state 계약은 `contracts/CLK-01-source.json`, 고정 입력과
helper recipe는 `contracts/CLK-01-cases.json`, 정확 일치 정책은
`contracts/CLK-01-tolerances.json`에 기록했다. GetRunPeriodData 외에
SetupEnvironmentTypes, Julian 변환과 weekday 계산, General::OrdinalDay,
실제 weather/callback 호출 시점을 추가로 검토했다.

Rust의 기존 calendar API와 실제 physical zone-loop hook을 재사용한다.
관찰 코드는 날짜 계산이나 시뮬레이션 상태를 갱신하지 않는다. prepared
calendar row와 실제 zone invocation을 구분하고, 기본 Summary 실행에서는
clock artifact를 생성하지 않는다. 원본의 monthly weekday 배열은 C++의
실제 before/after 상태와 Rust가 소비하는 월별 projection을 대조하며, 원본
전역 mutable 배열 전체를 복제했다고 주장하지 않는다.

원본 공개 API의 15개 실행은 모두 종료 코드 0이며, 각 입력에서 pre-report와
post-report callback이 24H 96회, 72H 288회, annual 35,040회씩 순서대로
기록되었다. raw 증거는 `.runtime/porting/CLK-01/native-api-first/`에 보존한다.
이 관찰만으로 Rust 단위·연결·생산 gate를 완료 처리하지 않는다. 전체 원본
C++ core와 genuine-state 단위시험, Rust 대응 비교와 생산 실행은 진행 중이다.

연간 준비 row는 `physics_executed=false`이다. B의 연간 물리 실행, Tomorrow
handoff/EPW cursor, DatesShouldBeReset, warmup 및 adaptive system 호출 순서는
이 카드의 준비 row로 인증하지 않는다. CLK-03 및 SYS 카드에서 별도로 검증한다.
