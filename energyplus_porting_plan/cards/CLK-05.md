# CLK-05 — 일사 전용 timestep 보간

- 그룹: 02 시간·기상
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: CLK-03
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/WeatherManager.cc`  
심벌: `SetupInterpolationValues; ReadEPlusWeatherForDay`  
대상 블록: SolarInterpolation 및 시간 중앙값 성격을 반영하는 이전/현재/다음 시간 가중치 블록.  
함께 읽을 선언/호출자: WeatherManager.hh, DataEnvironment.hh, SimulationManager.cc, DataSystemVariables.cc, UtilityRoutines.cc
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/WeatherManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/weather_day/{hourly,producer,production}.rs 및 저장된 보간 배열.
기존 `crates/ep_runtime/src/heat_balance/solar/interpolation.rs`는 비교할 재사용 후보이다.

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

EPW 원시 DNI/DHI[W/m²], 저장된 SolarInterpolation·TimeStepFraction, timestep 위치,
Hour1/Hour24 첫 시간 정책, 이전 시간 이력 및 같은 읽기 날짜의 다음 시간.
누락값 경고와 IgnoreSolar/Beam/Diffuse 제어는 실제 입력·환경변수에서 결정하며 첫 읽기 전에 소유 상태에 적용한다.

## 출력·변경 상태 계약

Tomorrow/Today의 timestep 직달·확산 일사[W/m²], LastHour/NextHour의 선택 일사 필드,
누락·범위 카운터 및 해당 Today 슬롯을 실제 소비하는 자체 열 계산 입력.

## 호출 시점

하루 기상 읽기에서 timestep 배열을 생성하고, 날짜 경계에서 Tomorrow→Today를 인계한다.
각 zone timestep은 실제 Today 슬롯을 소비한다.

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

1/3/4 timesteps-per-hour; 일정·ramp 일사; 24시 다음레코드

## 연결시험

고정 A-24H/A-72H/B-BOTH-24H 입력의 일반 CLI Full/Summary 실행에서
Today→소유 기상 sample→실제 열 계산 입력의 DNI/DHI 전달을 비교한다.
SRC-04의 입사·흡수식은 선행 카드가 완료된 후 별도로 검증한다.

## 착수 시 원본 경계 보완

`WeatherManager.cc:8328–8386`의 실제 Setup과 `2751–2766`, `2978–3009`, `3052–3135`의
일사 전처리·저장 가중치 보간·이력 순서를 고정한다. 1 timestep/hour에서는 전체 보간 블록이 건너뛰어지며,
그 안의 문법상 1-step 분기를 활성 분기로 집계하지 않는다. 24시 NextHour는 방금 읽은 같은 날짜의 1시이다.

경고 제어가 켜진 경우의 음수→누락 sentinel 처리와 diffuse 누락 카운터의 beam 기준 대입을 보존한다.
첫 InitializeWeather는 읽기 전에 카운터를 초기화하므로 pre-GetNext 카운터 입력은 초기화 관측용이다.
첫 이름순 Output:Diagnostics 객체와 세 환경변수의 실제 문자열을 사용하며, 이후 실행에서도 그 입력을 다시 확인한다.

1/3/4 timestep, 비제로 일정·독립 ramp, 두 날짜의 이력, signed zero·9999 경계,
각 Ignore 제어 및 1.0에 인접한 저장 가중치를 실행 전 확정한다. 기존 CON 입력 45개 순서·해시는 유지한다.
이 카드의 기준 호출은 Setup/GetNext/Initialize이며 CurrentTime·SimTimeSteps는 일반 CLI의 SetCurrent 이력과 짝지어 인증하지 않는다.
태양 위치·현재 기상의 야간/EMS 보정·하늘·SRC-04 표면 물리식은 이번 경계의 통과 범위에 포함하지 않는다.

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
