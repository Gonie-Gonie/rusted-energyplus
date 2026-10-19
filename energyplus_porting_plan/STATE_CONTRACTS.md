# 공통 상태 계약

EP struct 전체를 복제하지 않고 아래 데이터의 생산자·소비자·시점을 고정한다.
아래 계약명은 이 작업계획의 제안이며 현재 저장소에 같은 이름의 타입이 존재한다는 뜻은 아니다.

| 계약 | 핵심 필드·단위 | 생산자 | 주요 소비자 | 시점 |
|---|---|---|---|---|
| Geometry | global vertices[m], area[m²], normal, azimuth/tilt[deg], centroid[m], volume[m³] | GEO-01~03 | CTF/SRC/RAD/ZON | 초기화 후 불변 |
| CalendarFrame | civil date, weather ordinal, schedule ordinal, day type, hour/timestep, environment | CLK-01/03 | SCH/기상/보고 | 일자·환경별 |
| WeatherStep | dry/wet bulb[°C], W[kg/kg], P[Pa], wind, rain, DNI/DHI/IR[W/m²] | CLK-02~06 + PSY | SRC/ZON | zone timestep |
| ScheduleValue | schedule ID, scalar, 조회 날짜·시각·갱신단계 | SCH-03 | 내부발열·설정온도·가용성·효율 | 원본 조회 시점 |
| CtfCoefficients | X/Y/Z/flux coefficients, NumCTFTerms, CTFTimeStep, NumHistories | CTF-01~10 | SUR | 구성체 초기화 후 불변 |
| CtfHistory | 표면 내외 T·열유속의 current/lagged/master history | SUR-01/06 | SUR-02~04 | accepted zone step 후 commit |
| SurfaceIterationInput | 외부 solve 결과, 고정 CTF 상수항, 이전 반복 내표면 T, 참조 공기 T, h, 복사원 | SRC/RAD/SUR | SUR-04/05 | 반복 회차별 |
| ZoneAirState | MAT/ZT/ZTAV, W 및 zone/system 이력; dt·retry flags | ZON-01/04~06 | ZON-02/03, HVAC | 상태명별 의미를 유지 |
| ZoneCoefficients | SumHA, SumHATsurf/ref, SumIntGain, SumMCp(T), SumSysMCp(T), AirPowerCap | ZON-02/HVAC-08 | predictor/corrector | 각 단계의 snapshot |
| Demand | heating/cooling threshold load[W], active demand, deadband, zone/group multipliers | ZON-03 | HVAC-03~05 | 설비 호출 전 |
| SupplyState | T[°C], W[kg/kg], h[J/kg], m_dot[kg/s], delivered sensible/moisture | HVAC-04~06 | HVAC-08, reporting | Calc 말미에 commit |
| StepReport | 열처리율[W], step energy[J], Average/Sum, accepted dt[s], warmup/report flag | SUR-07/HVAC-07 | SYS-04 | 반복값 overwrite 후 승인된 step만 합산 |
| WarmupConvergence | Tmax/Tmin, heating/cooling load extrema, 이전날, 온도 절대차/부하 정규화 차이 | SYS-02 | SYS-03 | 일 종료 |

## 상태의 시간 의미

한 필드라도 `이전 zone step`, `이전 system step`, `현재 반복 시작`, `현재 반복 종료`,
`accepted step`, `zone 평균` 중 어느 값인지 명시한다.
`MAT`, `ZT`, `ZTAV`를 이름만 비슷하다는 이유로 하나의 temperature로 합치지 않는다.

## 상태 변경 규칙

카드는 선언된 출력/변경 필드 외에는 쓰지 않는다.
단위시험은 읽기 입력뿐 아니라 원본에서 변경되는 캐시·flag·배열도 비교한다.
캐시 재사용은 입력 키·무효화 시점·계산 순서를 보존한다.

## 실제 구현의 소유권과 호출 순서

- 날씨의 Today/Tomorrow, 달력, 스케줄, CTF master history는 서로 다른 지속 상태입니다. 환경 초기화·warmup·RunPeriod의 인계와 accepted step의 갱신 시점을 각각 검증합니다.
- 실내 대류는 공간별 MAT/ZT/ZTAV와 두 표면온도 배열을 구분합니다. Natural의 strict 부호·exact-zero 분기, Detailed의 하한 대입 및 원본 반복 갱신 순서를 유지합니다. [TARP 비교](evidence/SRC-06-tarp-inside-selected/summary.json), [dispatcher 상태 비교](evidence/SRC-06-tarp-dispatcher/summary.json).
- Predictor/corrector는 실제 AirHeatBalance·Node·Demand 소유자를 공유합니다. [전체 predictor 부모 1,168건](evidence/SRC-06-canonical-full-predictor-parent-48/summary.json)은 독립적으로 포착한 호출의 검증이며, 연속 push/revert/retry 이력과 multiplier 변형의 검증은 별도로 남아 있습니다. 코드에서 도출한 네 사후조건은 Native 관측 비트로 세지 않습니다.
- IdealLoads의 입력·가용성·초기화·계산·노드 갱신·보고는 같은 장비와 노드 상태를 사용합니다. [hard-sized 네 Sizer 및 전체 Size](evidence/SRC-06-canonical-hard-sized-purchased-air-43/summary.json)의 검증 범위와 전체 SimPurchasedAir의 지속 호출 검증을 구분합니다.
- OutputProcessor의 system step 누적과 zone step 확정은 별개입니다. accepted 시간만 누적하고, 원본 zone 호출이 수행하는 meter 갱신을 중복 호출하지 않습니다. warmup·시간·일·월·환경 reset과 report latch는 원본 시점에 유지합니다.
- 종료 처리는 같은 Source 오류 카운터·진단 상태·타이머를 소비합니다. [일반 종료 검증](evidence/SRC-06-canonical-normal-completion-47/summary.json)과 [성공 실행 로그 정리](evidence/SRC-06-canonical-successful-log-layout-49/summary.json)를 참조합니다. ERR·END·콘솔의 실제 실행 시간은 보존하며, 실패/Legacy 경로는 기존 출력 경로를 유지합니다.

## 증거 해석

같은 입력·초기 상태·관측 필드와 실제 Source 버전을 확인한 뒤 비교 결과를 재사용합니다. 단위 호출, 일반 실행, 보고 값, JSON/CSV 재읽기 검증은 각각 기록합니다. 과거 원본 실행의 부동소수점 환경이나 private cache 상태를 관측하지 않았다면 추정값으로 대체하지 않습니다.

[과거 CTF 이력 비교](evidence/CTF-10-main-thread-history-comparison/summary.json)의 235,456개 불일치와 2,754개 필수 미확인 경계는 해당 과거 Source의 실패 기록입니다. 현재 실행의 결과로 바꾸어 표기하거나 삭제하지 않습니다. 최신 검증 범위와 아직 닫히지 않은 gate는 [작업계획 안내](README.md#현재-진행-상태)와 `plan.json`을 참조합니다.
