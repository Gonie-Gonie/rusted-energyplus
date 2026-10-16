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

## SYS-02/03 선택 보고서와 실제 A caller

선택 보고서 계약은 Zone별 여섯 rolling owner, 첫 Run day의 차이 벡터·전체 행렬·Count, 보고서 scratch의 실제 할당/기록 범위와 caller 소유 EIO를 구분한다. BeginEnvrn reset, 매 zone timestep의 RecKeep, 실제 첫 Run EndDay의 Report 호출은 각각 원본 시점과 상태를 유지한다. [선택 owner/EIO 비교와 현재 A caller 시험](evidence/SYS-03-ordinary-warmup-report/summary.json)은 전체 backend·부하 생산자·ordinary weather handoff의 Native 동등성을 뜻하지 않는다.

## SRC-05 선택 setter와 실행 문맥

표면의 기존 네 상태 배열, 실제 높이·ExtWind·날씨 보정 입력과 원본 쓰기 순서를 유지한다. [별도 main 실행의 선택 setter 비교](evidence/SRC-05-wind-diagnostics-and-main-thread/summary.json)는 실제 argc3/Cargo 산출물과 역사적 Source에 묶여 있다. read-only main/spawn 부동소수점 진단과 과거 실패는 별도 증거이며, 생산 worker의 동등성이나 차이의 원인으로 단정하지 않는다. 실제 ordinary 초기화·온도 검사·후속 소비자 연결은 계속 미확인이다.

이후 [실제 Windows 생산 진입점 검증](evidence/SRC-05-production-main-thread/summary.json)에서 CLI와 ordinary observer를 main에서 실행하고 실제 PE의 8 MiB stack을 확인했다. 동일한 A-24H 입력의 두 실행은 정상 종료했으며 전체 출력과 관찰 기록의 완결성을 검토했다. 부동소수점 제어값을 쓰지 않으며, 수치 비교·경계 도달·네 배열 owner와 후속 소비자 연결의 동등성을 이 실행 종료만으로 판정하지 않는다.

## SRC-06 선택 TARP 상태·정밀도 계약

Natural은 실제 Tsurf−Tamb와 저장된 cosine을 읽고 exact-zero 및 strict 부호 분기를 유지한다. Detailed은 실제 outward cosine의 음수를 전달하며 Hc<LowHConvLimit일 때만 하한을 대입한다. 선택 원본에는 EPS, 상한 clamp, NaN 대체가 없다. [선택 API 비교](evidence/SRC-06-tarp-inside-selected/summary.json)는 입력·초기 상태·모든 pre-owner·유지 셀을 NaN payload까지 비트 그대로 대조한다. 현재 계산된 비정상 endpoint만 class 확인 후 unavailable로 남긴다. 공간 MAT, balance reference snapshot, 초기/반복 표면온도 배열과 갱신 시점은 서로 다른 실제 owner이며 다음 caller 연결에서 보존해야 한다.

## CTF-10 실제 main 실행의 이력 비교

[동일 A-24H 입력의 전체 비교](evidence/CTF-10-main-thread-history-comparison/summary.json)는 1,340개 대응 단계의 불일치 235,456개와 필수 미확인·미대응 경계 2,754개를 FAIL로 보존한다. 최초 대응 차이는 이력 갱신 전 입력 SurfTempIn에 이미 존재한다. Warmup 횟수·실제 시계·배열 소유자·초기화·반복 호출 시점을 각각 검증해야 하며, 입력 경계의 차이를 특정 생산자의 원인이나 갱신 식의 실패로 단정하지 않는다. 실제 상태와 호출 순서의 동등성 및 전체 게이트는 계속 미확인이다.

## SRC-06 선택 dispatcher의 지속 상태

[실제 선택 초기화 비교](evidence/SRC-06-tarp-dispatcher/summary.json)는 공간별 MAT/ZT/ZTAV를 별개로 보존하고, 비활성 space correction에서 MAT/ZT만 복사한다. 첫 호출·정적 형상·Scan·backend 상태와 실제 zone-space 순서, 명시한 SurfTempInTmp 또는 SurfTempIn을 함께 대조했다. 입력과 전체 pre-owner가 먼저 일치해야 endpoint를 판정하며, 선택 범위 밖264개 문맥은 PASS에 포함하지 않는다. 일반 실행에서의 형상 생산·환경 reset·공기 상태 갱신·두 온도 배열·복사 후 완료 반복30회마다 갱신·즉시 열수지 항 갱신은 아직 별도로 연결하고 검증해야 한다.

[Space 상태 참조 인터페이스](evidence/SRC-06-live-space-convection-interface.json)는 같은 대류 계산이 호출자 소유 MAT/ZT/ZTAV를 직접 빌려 읽도록 한다. 기존 호출자와 계산식은 유지하고 관련 기존 시험8개·Clippy·형식·소스 품질 검사를 통과했다. 실제 Source AirHB 생성·환경 초기화와 전체 표면 열수지 호출 연결은 계속 진행 중이며, 수치 비교 완료 건수와 카드 gate는 늘리지 않는다.

[음영 진단 상태](evidence/SRC-06-shadow-diagnostic-owners.json)는 같은 ErrorTracking에 원본의 비볼록 수신·투사 표면 카운터2개와 실제 생성자 값0을 유지한다. 기존 생성자15곳만 확장하고 오류 처리 시험16개·runtime/CLI 전체 대상 Clippy·형식·소스 품질 검사를 통과했다. 음영 보고서와 전체 태양열 호출은 아직 활성화하지 않았으며, 이 생성자 확장으로 수치 비교나 카드 완료를 주장하지 않는다.

## SRC-06 ordinary opaque geometry owners

[실제 지중온도 생산자 연결](evidence/SRC-06-ground-temperature-implementation.json)은 EPW의 FC 헤더 상태·네 월별 모델·실제 입력과 EIO를 유지하고 현재 기상 상태에 월별 값을 기록한다. 기존 관련 시험14개·Clippy·형식·소스 품질 검사와 A-24H를 통과했다. 이전 실행의 결과147,618바이트는 그대로이며, EIO에는 네 모델의 실제8행1,012바이트만 추가되었다. Ground/FC 이력 소비 연결과 나머지 기상 입력 child 및 Native 수치 대조는 계속 진행 중이고 카드 gate는 유지한다.

[실제 보조 기상 입력 연결](evidence/SRC-06-weather-auxiliary-implementation.json)은 Ground와 Station 사이의 GroundReflectance·SnowModifier·WaterMains 입력 조회와 EIO를 원본 순서로 실행하고, 현재 기상의 날짜·WaterMains 상태를 유지한다. 기존 날씨 시험4개·Clippy·형식·소스 품질 검사와 A-24H를 통과했다. 이전 결과147,618바이트는 그대로이고 EIO에는 실제8행1,265바이트만 추가되었다. 선택 객체가 존재하는 getter·상관식 및 전체 기상 입력과 열수지·공기·워밍업 연결은 계속 진행 중이다. 이는 회귀 확인이며 새로운 Native 수치 비교나 카드 gate 완료 판정은 아니다.

[실제 Surface 면적 소유자 연결](evidence/SRC-06-geometry-area-owner-implementation.json)은 Area·GrossArea·NetAreaShadowCalc의 생성자0과 GetVertices9387→9388→9389 기록을 같은 Surface에 유지한다. 면적 검사2459는 Area, 중심점14555·높이·기하 총계는 GrossArea를 읽고, 층·천장·총 표면 면적은 Area를 읽는다. 독립 원본 검토10파일·13범위와 Clippy·형식·소스 품질·A-24H를 통과했다. 기존 결과147,618바이트와 EIO12,614바이트는 각각 완전히 같다. 새 시험·Native 수치 비교·카드 gate 변경은 없으며, 전체 열수지·공기·HVAC·워밍업 실행 연결은 계속 진행 중이다.

[실제 표면 볼록성 검사 연결](evidence/SRC-06-surface-convexity-implementation.json)은 GetVertices·면적 진단 다음에 원본 CheckConvexity를 호출하고 같은 표면의 IsConvex와 지속 scratch·진단 상태를 유지한다. 기존 CLI 시험1개와 A-24H 실행을 통과했으며, 이전 실행과 결과147,618바이트·EIO10,337바이트가 모두 일치했다. 이는 기존 실행의 회귀 확인이며 새로운 Native 수치 비교나 전체 InitSurface 완료 판정은 아니다. 새 시험은 추가하지 않았고 카드 gate는 유지한다.

[Actual producer comparison](evidence/SRC-06-ordinary-geometry/summary.json) binds complete physical input/Raw/typed identity before selected retained GEO03 returns, perimeter/dimensions, six bounds and exterior-area owners. The returned GEO03 object is moved once from its existing initializer call; no observer recomputes an owner. This prerequisite does not activate ordinary space air/reset, named inside-temperature arrays, post-radiation cadence or handoff, and does not expand historical Volume-only certification.


[날씨·실제 호출 연결 검증 체크포인트](evidence/SRC-06-provider-integration-checkpoint-07/summary.json)에 검증한 제안 코드와 재현 ZIP을 보존했습니다. 같은 세션의 GetNext·설계일 생성·일별 판독·보간·스케줄·강수량·외부 에너지 사용·열수지 호출을 연결했고, 실제 준비 루프와 본 실행 루프의 원본 순서를 추가했습니다. 새 빌드 디렉터리에서 runtime·run·CLI 및 기존 테스트 타깃 전체 컴파일을 통과했습니다. 기존 판독기 9개와 설계일 준비 과정 1개 테스트가 통과했으며 새 테스트는 추가하지 않았습니다. 수명·문자열 타입 오류와 수정 기록도 보존했습니다. 전체 실행 파이프라인 활성화, 준비 과정 후처리·본 실행의 실제 호출 연결 및 새로운 동일 입력 Native A/B 검증은 남아 있습니다. 네 gate·A14/54·B14/64와 기존 수치 FAIL은 유지합니다.
