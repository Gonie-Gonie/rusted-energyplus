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
