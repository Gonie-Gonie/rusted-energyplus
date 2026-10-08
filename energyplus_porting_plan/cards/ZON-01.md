# ZON-01 — 존 공기 상태 초기화

- 그룹: 08 존 공기
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: CON-01, GEO-03
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/ZoneTempPredictorCorrector.cc`  
심벌: `ZoneSpaceHeatBalanceData::beginEnvironmentInit`  
대상 블록: 선택한 생성자 기본값·환경 재구성의 현재 W seed·beginEnvironmentInit의 14개 쓰기와 guard를 구분한다. MAT/ZT/ZTAV와 현재 W는 이 member가 재설정하지 않는다.
함께 읽을 선언/호출자: ZoneTempPredictorCorrector.hh::ZoneSpaceHeatBalanceData의 해당 필드만  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/ZoneTempPredictorCorrector.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

heat_balance/zone_air_initialization.rs; air_manager.rs; state.rs; timestep.rs

계획 반입 시 기준은 `d7b516627f421259012f3e61bc28cca831452468`이었다.
착수 상태의 `initialization.rs`와 `air_manager.rs`는 생성 시 온도와 날씨 W를
현재/평균 상태 및 세 칸 이력에 기록한다. 이 온도 이력은 원본 XMAT/DSXMAT의
앞 세 칸 후보이며, 원본이 환경 시작에 0으로 쓰는 별도 ZTM과 같지 않다.
최종 구현은 `ZoneAirInitializationState`의 24개 필드와 persistent 환경 guard를
실제로 사용한다. 생성자 → bulk W seed → 실제 caller 온도 준비 → member 초기화
→ 기존 solver의 현재/평균 값과 세 칸 이력 저장 순서를 분리했다. Full 관측은
저장 직후와 공통 solver 첫 진입을 복사하며, 후속 history 갱신은 대응하지 않는다.
선택한 24필드는 네 번째 칸을 포함한 transient owner 관측이다. 실제 legacy
저장은 네 scalar·네 3-slot 배열·diagnostic 세 scalar의 19개 값이며, persistent
환경 guard와 transient 24필드의 수명을 같다고 주장하지 않는다.
범위 내 Rust 파일 12개의 원본/연결 검토와 두 관측 closure의 lint 수정도 보존했다.

## 입력 계약

환경 시작, 초기 상태정책

착수 계약은 `contracts/ZON-01-source.json`, `ZON-01-cases.json`,
`ZON-01-tolerances.json`과 `cases/ZON-01/helper-request.json`에 고정했다.
원본 11파일·36구간과 입력 비트는 실행 전에 독립 검토·동결했고, 실제 원본 실행이
Rust 수치 실행보다 앞섰다. 아래 최종 단위·연결·생산 증거로 A/B의 네 gate를 닫았다.
직접 단위 입력은 유한 binary64 OutHumRat, 명시된 상태 canary와 환경 입력
플래그뿐이다. 출력·기대 상태는 Rust 입력에 공급하지 않는다.

## 출력·변경 상태 계약

존 공기온도·습공기비와 history의 초기값

원본 `ZoneSpaceHeatBalanceData::beginEnvironmentInit` 2818–2836행은
ZTM[4]와 WPrevZoneTSTemp[4]를 +0, WPrevZoneTS[4]/DSWPrevZoneTS[4]와
WTimeMinusP/W1/WMX/WM2를 실제 외부 OutHumRat로 기록한다. airHumRatTemp,
tempIndLoad/tempDepLoad/airRelHum/AirPowerCap/T1은 +0이다.
MAT/ZT/ZTAV/XMPT/XMAT[4]/DSXMAT[4]/TMX/TM2/현재 W/평균 W는 유지한다.

MAT/ZT/ZTAV 등의 23°C 기본값과 현재/평균 W의 .01 기본값은 생성자에서
온다. 앞선 `InitThermalAndFluxHistories`의 존 블록 2231–2239행은 원본
concrete owner를 재구성하고 현재/평균 W만 OutHumRat로 seed한다.
이 세 단계를 하나의 blanket reset으로 바꾸지 않는다. AirPowerCap의 W/K
필드는 Rust의 air_heat_capacity_j_per_k와 단위·상태 의미가 다르다.

genuine 전체 `InitZoneAirSetPoints`를 쓰는 단위 guard는
MyEnvrnFlag && BeginEnvrnFlag에서 호출하고 MyEnvrnFlag를 false로 쓰며,
!BeginEnvrnFlag에서 재무장한다. wrapper 호출 횟수와 읽은 guard 조건은
실제 member 호출 횟수의 직접 관측으로 바꾸지 않는다.
형제 thermostat/demand/day reset은 원본 전체 함수가 실행하지만 대응하지
않으며 모든 형제 상태를 관찰했다고 주장하지 않는다.

## 호출 시점

BeginEnvrn

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

Space/MRT/comfort/mixing·형제 thermostat/demand·전역 카운터·IO와 SUR-01,
ZON-02 AirPowerCap 결과, ZON-04/05/06 이후 history/correction/retry는 제외한다.
원본 weather·calendar·warmup과 Rust 외부 W 생산자의 동등성은 이 카드가
인증하지 않는다. 단계 불일치를 감추는 수치 허용치를 추가하지 않는다.

## 단위시험

첫환경/새환경 reset; 초기값 일관성

입력-only 여덟 sequence는 생성자/0, bulk 현재 W seed, 양수 subfloor,
−0 복사, 유지 필드와 네 번째 칸 canary, 연속 member 호출, guard skip/rearm,
새 환경 재구성을 구분한다. 직접 대입·+0 reset·array 길이·입력 비트와 flag는
정확 비교하며 물리 허용치를 쓰지 않는다.
최종 커밋의 8 sequence·34 operation은 5,528건 비교에서 불일치 0이다.
126 snapshot 쌍, 5,082 scalar-bit 쌍, 252 guard-flag 쌍과 guard 조건·wrapper
횟수 각 34쌍을 포함한다. 불완전했던 baseline은 동일 단계 수치 비교를 만들지
않고 비교 건수 0인 상태로 보존했다.

## 연결시험

실제 초기화 반환과 기존 solver 저장·투영값의 인계

원본은 변경하지 않은 CON 일곱 입력에서 실제 세 API callback을 관측했다.
after-init-HB는 surface bulk 재구성보다 앞이고, beforePredictor는 bulk 이후
HVAC가 ZTAV/평균 W를 0으로 쓴 뒤이며 member 호출보다 앞이다.
afterPredictor는 작업 이력·계수·load가 이미 변한 단계이다. 이 관측들을
순수 14개 member 반환값으로 표현하지 않는다.
원본 callback 28,081행과 physical 단계별 1,056/1,056/2,226행을 누락 없이
보존했다. 이 횟수를 직접 member 호출 횟수나 Rust warmup 횟수로 바꾸지 않는다.

Rust는 A24/A72/BBoth24 Full/Summary 여섯 정상 명령을 실제 실행했다. 실제 초기화의
자체 외부 W 인수 → 선택한 24개 반환 필드 → 저장된 현재/평균 상태와 앞 세 칸 solver
handoff를 검증했다. 690건 비교에서 불일치 0이며, 생성·bulk·caller 준비·member
관측 504비트 쌍과
저장·첫 solver 진입 인계 114비트 쌍을 확인했다. Full 초기화와 실제 index=0 진입
관측은 각 3개이다. 별도로 정상 clock 96/288/96구간의 순서·비트·누락 0을 확인했다.
원본 우선 exact assignment 단위 증거에 근거하며, 실제
native callback은 입력·flag·원본 단계의 문맥 증거로 남긴다. source 외부 W
또는 warmup 단계의 교차 엔진 대응은 CLK/SYS 검증 전까지 대응하지 않는다.
Summary는 관측 부재와 일반 출력 동일성만 확인한다. 연간/B72 Rust 물리,
SUR-01 또는 warmup driver 전체 완료는 주장하지 않는다.

## 정밀도 정책

선택한 대입·유지 값과 solver 인계는 atol=0, rtol=0이며 value_bits와
signed zero를 정확 비교한다. 필드·네 칸 배열·입력 비트·flag와 순서도 정확
일치한다. 원본 콜백과 Rust 외부 날씨 생산자의 불일치를 감추는 물리
허용치는 없다. source 외부 W·calendar·warmup 단계 대응은 별도 미검증이다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

[원본 우선](../evidence/ZON-01/original-first.json),
[최종 단위](../evidence/ZON-01/final-unit-comparison.json),
[최종 생산](../evidence/ZON-01/production-comparison.json),
[실제 명령](../evidence/ZON-01/actual-commands.json),
[독립 최종 검토](../evidence/ZON-01/independent-final-review.json),
[lint 수정과 새 소스 검증](../evidence/ZON-01/source-amendment.json)을 연결했다.
이전 candidate534 증거·Clippy 실패와
[reader 인수 오류](../evidence/ZON-01/reader-invocation-failure.json)는 바꾸지 않았다.

최종 소스의 workspace 시험은 21개 suite group에서 4,552 통과/0 실패/0 ignored이다.
Clippy `-D warnings`, source-quality, heat-balance-structure와 scoped rustfmt도 모두
exit 0이다. rustfmt 범위는 `air_manager.rs` lint 수정뿐이며 전체 workspace
format 검증으로 표현하지 않는다.

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [x] 상태·시간·호출순서를 포함한 연결시험 통과
- [x] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋: `af3f3cb2c1ed79ac2a7887e5ca2cc6334196cf9e`

시험 명령: 위 실제 명령 증거의 final02 단위·production command03와 5개 필수 검사.

증거 경로: `energyplus_porting_plan/evidence/ZON-01/`; 최종 crates tree는
`e94b3224a4f6aeb5e7b2b993cfe4587b6585322c`이다.

최대오차/RMSE/상태 불일치: 수치 RMSE를 재계산하지 않는 exact assignment 정책;
최종 단위·생산의 비트/상태 불일치는 각각 0이다.

추가 검토할 helper: Space·형제 manager 상태, 이후 history/correction/retry,
source weather/calendar/warmup 정렬, AirPowerCap 결과와 전체 SYS는 후속 범위이다.
