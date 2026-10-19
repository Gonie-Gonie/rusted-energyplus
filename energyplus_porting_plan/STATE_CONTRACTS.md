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

- Today/Tomorrow·달력·스케줄·CTF history는 서로 다른 지속 owner입니다. 환경 초기화·warmup·RunPeriod의 인계와 accepted step 갱신을 구분합니다.
- 실제 Surface 부모에서 외표면→실내 반복→공기 계산→최종 표면→history 순서를 관측합니다.30회 대류 갱신의 실제 heap owner와 반환 H→계산 전 H→첫 소비 H를 연결하며 이동 가능한 Rust stack 주소의 전역 불변성을 요구하지 않습니다.
- 두 warmup 인계 경계의 MAT/W·CTF 상수·normal4배열×19항은 실제 할당 owner에서 직접 관측합니다. 보고 평균이나 생성하지 않은 배열 상태로 대체하지 않습니다. 34437 scope rows; 635375 exact binary64 observations; 2 direct handoffs; 4 same-owner convection connections
- Predictor/corrector·Demand·Node·IdealLoads는 실제 동일 owner를 사용합니다. 독립 포착 부모1,168과 연속 history40·배수2의21호출은 다른 관측 수명입니다. 단위 결과를 생산 입력으로 주입하지 않습니다.
- OutputProcessor의 system 누적과 zone 확정, warmup/주기 reset, 보고·meter 갱신은 원본 시점을 유지합니다. 일반 종료는 같은 오류 카운터·진단·실제 타이머를 소비합니다.

## 증거 해석

최종 [카드별 근거](evidence/SRC-06-final-current-source-validation-60/summary.json)는 같은 입력·정책·관측 필드·Source의 유효한 연결을 명시합니다. 실제 ERR/END/콘솔 경과 시간은 보존하며, 미관측 private cache·통계·부동소수점 환경을 추정값으로 채우지 않습니다. 과거 CTF 이력의235,456 불일치와2,754 미확인, SCH01 원본 비정상 종료2건 및 모든 수정 전 실패는 해당 실행의 기록으로 유지합니다. 새 결과가 이를 과거 PASS로 바꾸지 않습니다.

[SUR-02 보충 단위시험](evidence/SUR-02-isolated-history-impulse-61/summary.json)은 격리 이력·impulse·현재항 배제를 실제로 검사합니다. 타입 어댑터를 쓰는 선택 본문 시험과 기존 P7의 실제 소유자 연결 증거를 구분합니다.
