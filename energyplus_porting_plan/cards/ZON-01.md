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

heat_balance/initialization.rs; state.rs

계획 반입 시 기준은 `d7b516627f421259012f3e61bc28cca831452468`이었다.
착수 상태의 `initialization.rs`와 `air_manager.rs`는 생성 시 온도와 날씨 W를
현재/평균 상태 및 세 칸 이력에 기록한다. 이 온도 이력은 원본 XMAT/DSXMAT의
앞 세 칸 후보이며, 원본이 환경 시작에 0으로 쓰는 별도 ZTM과 같지 않다.
아직 원본의 네 칸·임시 필드·guard를 소유한 초기화가 검증된 것은 아니다.

## 입력 계약

환경 시작, 초기 상태정책

착수 계약은 `contracts/ZON-01-source.json`, `ZON-01-cases.json`,
`ZON-01-tolerances.json`과 `cases/ZON-01/helper-request.json`에 고정했다.
독립 검토와 원본 우선 실행 전이며 네 gate는 모두 대기 상태이다.
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

## 연결시험

실제 초기화 반환과 기존 solver 저장·투영값의 인계

원본 계획은 변경하지 않은 CON 일곱 입력의 실제 세 API callback이다.
after-init-HB는 surface bulk 재구성보다 앞이고, beforePredictor는 bulk 이후
HVAC가 ZTAV/평균 W를 0으로 쓴 뒤이며 member 호출보다 앞이다.
afterPredictor는 작업 이력·계수·load가 이미 변한 단계이다. 이 관측들을
순수 14개 member 반환값으로 표현하지 않는다.

Rust 계획은 A24/A72/BBoth24 Full/Summary 여섯 정상 명령이다. 실제 초기화의
자체 외부 W 인수 → 14개 반환 필드 → 저장된 현재/평균 상태와 앞 세 칸 solver
handoff를 검증한다. 원본 우선 exact assignment 단위 증거에 근거하며, 실제
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
