# CLK-02 — EPW 헤더·시간별 레코드 해석

- 그룹: 02 시간·기상
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: CON-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/WeatherManager.cc`  
심벌: `ProcessEPWHeader; InterpretWeatherDataLine; OpenEPlusWeatherFile`  
대상 블록: raw EPW 레코드와 선정 LOCATION·HOLIDAYS/DST·DATA PERIODS 헤더 해석. 실제 파일 열기의 헤더 토큰·순서와 직접 enum 호출은 별도 경로로 검증한다.  
함께 읽을 선언/호출자: WeatherManager.hh, StringUtilities.hh, UtilityRoutines, General 날짜 helper, 실제 InputFile 읽기  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/WeatherManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/weather.rs; weather_calendar.rs; weather_data_periods.rs

착수 전 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

동결된 CON-01 공통 EPW 전체 8,760 raw 레코드 및 입력 전용 진단 텍스트·초기 canary·실제 continuation 파일.
기존 CON-01 15개 입력은 유지하고 추가 EPW 진단은 parser 단위 범위로 한정한다.

## 출력·변경 상태 계약

날짜·시간 정수 5개, 공개 필수 Real64 출력 20개, 정수 WObs, weather code 9개와 선택 tail Real64 출력 6개.
내부 RField21은 관측 출력으로 재구성하지 않는다. Data Source/Integrity 열과 35번째 강수량 측정기간 열은 원본 parser가 출력하지 않는다.
원본 sentinel·단위·signed zero를 보존하며 raw record year는 civil simulation year와 구분한다.
선정 헤더 상태, 초기·최종 ErrorsFound, 실제 Line 소비와 InputFile 위치·상태, weather-code missing count를 기록한다.

## 호출 시점

기상 입력 읽기

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

이 parser는 이전값·기본값 sentinel 대체, RH 비율 변환, rain/snow 판정, IR/하늘 온도 계산 또는 timestep 보간의 소유자가 아니다.
해당 ReadEPlusWeatherForDay·calcSky·현재기상 단계는 CLK-03~06에 남긴다. 원본 Typical/Extreme·Ground 헤더와 사용하지 않는 열이 실제 처리된다는 사실은 유지하되, 선정 CON의 해당 물리 소비를 인증하지 않는다.
일반 Site:Location override, Gregorian civil clock·warmup 동기화, 전체 ErrorManager/IO 메시지 동일성도 제외한다.
미초기화 원본 local은 관측하거나 canary로 보완하지 않는다. 활성 DST 단위 입력은 실제 weekday를 설정하는 형식을 사용하고, 고정 CON의 No/0/0은 inactive 원본 경로로 구분한다.

## 단위시험

고정 EPW 전체 raw 해석; 정상·sentinel 복사·signed zero·원본의 약한 날짜 경계; 실제 weather-code stateful 분기; tail 누락·빈칸·공백·D exponent·안전한 오류 진단.
LOCATION numeric canary retention, 실제 헤더 continuation, DATA PERIODS 연도·wrap·초기 error flag·0 count/interval·ordinal 진단과 full-file dispatch를 구분한다.
원본 readList의 마지막 status comma-fold·delimiter skip을 엄격한 CSV admission으로 바꾸어 주장하지 않는다. delimiter 부족 또는 끝나지 않는 continuation은 진단 admission에서 제외한다.

## 연결시험

A-24H·A-72H·B-BOTH-24H Full/Summary의 실제 raw parser owner와 물리 adapter 전달값·입력 identity를 연결한다.
8,760행 parser 단위시험은 annual thermal physics 인증이 아니다. CLK-03~06의 레코드 선택·결측 대체·보간·IR 계산은 별도 gate에서 검증한다.

## 정밀도 정책

유한 raw 숫자 해석·복사에는 binary64 bit exact와 signed-zero exact 정책을 적용한다. 정수·문자열·boolean·순서·배열 길이·상태는 타입까지 정확 일치한다.
원본의 실제 단위 변환이나 물리 부동소수점 계산은 이 paired 범위에 없다. 오류 진단은 실제 결과와 사용 가능한 출력을 보존하며 미리 정한 exit code·오류 문장 parity를 넣지 않는다. 비교 전에 contracts/CLK-02-*.json으로 정책과 입력 bytes를 동결한다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [x] 상태·시간·호출순서를 포함한 연결시험 통과
- [x] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋: `930633cbda9c839cb0bad2c97361dc8885087de3`; crates tree `2bf661c9123d66f5af2bca51394f66d6c496a8ea`

시험 명령: unit/production actual record_command receipt 및 workspace·Clippy·source-quality·structure의 actual exit 0을 [종료 증거](../evidence/CLK-02.json)에 결합했다.

증거 경로: `energyplus_porting_plan/evidence/CLK-02.json`; 원본 21파일/64구간 및 header-container 보충 계약.

최대오차/RMSE/상태 불일치: exact 정책; 단위 1,009,979건/불일치 0, 생산 15,078건/불일치 0. RMSE를 재구성하지 않는다.

workspace 실제 시험: 4,552 passed/0 failed/0 ignored; 21 reported suite groups.

추가 검토할 helper: CLK-03~06 record selection·Today/Tomorrow·결측 대체·보간·sky/IR·calendar/warmup 및 이후 물리 계산.

기존 public legacy API는 유지한다. 새로운 생산 로더는 동일 raw 파싱 결과와 별도 physical projection을 반환하며, 실제 소비 인계만 이 카드에서 인증한다.

## 계획 데이터의 형식 보완

계획 검사에 맞춰 EP 범위 54개에 선정 helper 목록을 추가하고 외부 라이브러리 범위 10개를 별도 항목에 보존했다. EnergyPlus.hh의 실제 마지막 행은 196이므로 계획 범위의 끝을 196으로 고쳤으며, 동결 계약에 기재된 197도 별도 기록했다. 두 범위의 실제 바이트와 기존 해시는 같다. 동결 계약·원본·시험 기록·완료 gate는 변경하지 않았다.
