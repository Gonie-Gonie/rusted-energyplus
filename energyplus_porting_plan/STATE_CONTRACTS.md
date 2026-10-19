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


[저장소 Source 적용과 실제 검증 기록](evidence/SRC-06-canonical-provider-integration-01/summary.json). 검토한 일반 실행 Source 417개 파일(신규 325개)을 저장소 본체에 적용했습니다. 동일 세션의 날씨·준비 과정·열수지·본 실행 부모와 실제 시작 단계 child, 진단 상태 참조를 포함합니다. 기존 전체 워크스페이스·타깃 테스트 4,985개가 통과했고, 실패는 0개이며 기존 ignored 2개는 그대로입니다. 새 테스트는 추가하지 않았습니다. 기존 테스트 입력의 빠진 재료 선언, 중복 Site:Location 스키마, 상태 객체의 스택 보관 문제 및 일반 실행 출력 목록 두 곳을 수정했습니다. 기존 표면 테스트 준비값에는 실제 면적 필드 세 곳을 보완했습니다. 기존 전체 출력 집합과 구역 상태 비교는 유지했습니다. 실패·진단 기록과 각 수정의 범위를 별도로 보존했습니다. 기본 실행 파이프라인 전환, 전체 종료 후처리, 남은 실제 호출 연결과 동일 입력 Native A/B 검증은 미완료입니다. 네 gate·A14/54·B14/64 및 기존 수치 FAIL은 유지합니다. 작은 수정은 영향을 받는 기존 테스트로 확인하고, 이번 전체 Source 적용처럼 범위가 큰 통합 시점에 전체 테스트를 수행합니다.


[보고서 및 종료 함수 Source 적용](evidence/SRC-06-canonical-provider-integration-02/summary.json). 연결 보고서 3종과 RDD/MDD를 실제 pre-primary host에 연결하고, 같은 날씨·세션·원래 지역 상태를 사용하는 kickoff부터 primary까지의 호출 함수를 적용했습니다. 종료 단계의 작은 함수, 실제 요금 계산 진입 순서와 상태 필드도 포함합니다. 전체 타깃 컴파일 및 기존 실행 테스트 27개가 원래 8MiB 스택에서 통과했습니다. 새 테스트를 추가하지 않았고 전체 테스트는 반복하지 않았습니다. 실제 기본 파이프라인 전환·전체 종료 처리·Native 수치 비교는 미완료이며 A14/54·B14/64와 기존 FAIL을 유지합니다. 별도 실험용 파이프라인과 이후 종료 prefix는 이 적용에 포함하지 않습니다.


[날씨 입력 메타데이터 수정과 실제 실행 기록](evidence/SRC-06-canonical-weather-idf-metadata-03/summary.json). Site:Location·RunPeriod·SizingPeriod:DesignDay의 실제 IDF 필드 수 전달을 수정했습니다. 선택 필드의 전체 용량으로 입력 개수를 대체하지 않으며, RunPeriod와 DesignDay의 세미콜론 없는 EOF 입력은 정확한 메타데이터를 보장할 수 없어 명시적으로 미지원 처리합니다. 기존 raw-model 테스트 16개와 기본 실행 경로 A-24H 회귀 실행이 통과했습니다. 새 테스트 본문과 전체 테스트 반복은 없습니다. 별도 실제 Source 경로는 세 입력 오류를 넘고 20일 워밍업 후 본 기상 실행에 진입했으며, 첫 열적 복원력 누적 보고에서 미지원으로 종료합니다. 실제 Source 24시간 완료·기본 파이프라인 전환·Native 수치 동등성을 뜻하지 않습니다. A14/54·B14/64와 기존 전체 FAIL을 유지합니다.


[열적 복원력 계산과 실제 본 실행·원본 출력 대조](evidence/SRC-06-canonical-thermal-weather-04/summary.json). 실제 구역 상태·온도·냉난방 설정점을 사용하는 열적 복원력 기상 누적 계산을 적용했습니다. 같은 재실 상태를 보존하고 원본 분기·연산·갱신 순서를 유지합니다. 변경 Source를 포함한 진단 조합의 전체 타깃 컴파일이 통과했고 새 테스트나 전체 테스트 반복은 없습니다. 실제 Source A-24H는 20일 워밍업과 15분 기록 96개·시간별 기록 24개를 생성한 뒤 Primary에서 반환했습니다. 전체 실행은 종료 보고·결과 내보내기 경계에서 exit6이며 기본 파이프라인은 아직 전환하지 않았습니다. 같은 입력의 원본26.1.0 실행은 성공했고 출력 사전·시간축은 일치합니다. ESO 5,280개 값 중 3,536개는 보고된 십진수를 binary64로 해석했을 때 다르며 최대 절대 차이는 7.429434845107608e-11입니다. MTR 72개 보고 값은 같습니다. 내부 원시 상태·FP 환경 동등성을 뜻하지 않으며 허용오차를 추가하지 않았습니다. 원인 검토와 수치 동등성 검증은 남아 있습니다. A14/54·B14/64와 기존 전체 FAIL을 유지합니다.


[입력 통계와 실제 표 파일 생성 검증](evidence/SRC-06-canonical-input-statistics-tabular-05/summary.json). 입력 원시 필드와 스키마 메타데이터로 원본 입력 통계 7개를 계산하도록 연결했습니다. 실제 A-24H 통계 7개는 동일 입력의 Native 출력과 일치합니다. 종료 보고 선행 단계와 실제 표 파일 열기를 반영했으며, 진단 실행은 CSV·TAB·TXT·HTML·XML의 초기 헤더를 생성한 뒤 596번 날씨 통계 보고 진입 전 미구현 경계에서 exit6으로 끝납니다. 강우 보고는 원본의 날씨·나머지 보고 순서를 기다리므로 아직 실행되지 않았습니다. 기본 파이프라인은 입력 통계 초기화 호출만 갱신했고 진단 경로는 전환하지 않았습니다. 관련 기존 테스트 27개는 원래 8MiB 설정에서 통과했으며 새 테스트와 전체 테스트 반복은 없습니다. ESO·MTR·EIO는 직전 진단과 바이트까지 같아 기존 수치 차이가 유지됩니다. 전체 실행·수치 동등성·카드 완료를 뜻하지 않으며 A14/54·B14/64와 기존 전체 FAIL은 유지합니다.


[날씨·잔여·강우 보고 단계 검증](evidence/SRC-06-canonical-weather-remaining-reports-06/summary.json). 날씨 STAT 원시 바이트, 실제 보고 레지스트리와 동일 세션의 상태를 이용해 날씨·잔여·강우 보고 및 출력 형식 선택을 연결했습니다. 실제 진단 실행은 5212·5213·5214 순서를 거쳐 연간 에너지 요약표 WriteBEPSTable5230 진입 전 미구현 경계에서 exit6으로 끝납니다. 새로 도달한 표 항목 219개 중 218개는 동일 입력 Native 최종 보고서와 출력 바이트가 같고, 1개는 Native의 뒤 단계 덮어쓰기 전 자리표시자이므로 단계가 다른 미짝 항목입니다. 관련 기존 테스트 19개와 기본 실행 기존 테스트 27개가 원래 8MiB 설정에서 통과했으며 새 테스트와 전체 테스트 반복은 없습니다. 기본 파이프라인과 읽기 전용 진단 관측은 전환하지 않았습니다. ESO·MTR·EIO는 직전과 바이트까지 같아 기존 ESO 수치 차이가 남습니다. 전체 실행·수치 동등성·카드 완료를 뜻하지 않으며 A14/54·B14/64와 기존 전체 FAIL을 유지합니다.


[BEPS 보고 핵심 단계 검증](evidence/SRC-06-canonical-beps-report-core-07/summary.json). BEPS 보고 핵심 Rust 파일 17개(신규 12개·기존 5개)를 같은 실제 상태 소유자와 연결했습니다. 실제 격리 Source72 실행은 BEPS 표 13개를 작성한 뒤 목차 호출 5231의 미구현 경계에서 exit6으로 종료됩니다. 같은 입력의 Native 최종 보고서와 다섯 출력 형식의 앞부분을 비교해 원시 바이트 3,007줄이 같고, 버전·실제 시각 차이 9줄은 원문 그대로 별도 보존했습니다. LEED predefined 항목은 505개에서 726개로 늘었습니다. canonical all-targets 검사를 통과했으며 새 테스트나 전체 테스트 반복은 없습니다. 이전 27개 테스트 결과는 이전 checkpoint의 검증 범위로만 유지합니다. 진단 경로 9개·읽기 전용 snapshot·기본 pipeline은 이번 적용에서 제외했습니다. 기존 ESO 수치 차이와 A14/54·B14/64 및 전체 FAIL은 유지하며, 전체 실행·수치 동등성·카드 완료를 뜻하지 않습니다.


[Tabular 보고서 핵심 단계 검증](evidence/SRC-06-canonical-tabular-report-core-08/summary.json). 실제 TOC·Verification Summary·Demand·Source Energy 보고서 경로의 Rust 파일 20개(신규 10개, 기존 10개)를 같은 상태 소유자에 연결했습니다. 격리 Source75 실행은 이름 있는 표 25개를 작성한 뒤 실제 Component Sizing 진입 5235에서 exit6으로 중단됩니다. 같은 입력의 Native 최종 보고서와 다섯 출력 형식의 앞부분 6,680줄을 비교해 6,663줄이 바이트 단위로 같았으며, 버전·실제 시각 차이 17줄은 원문과 전체 hex로 보존했습니다. Predefined 항목은 732개에서 914개로 늘었고, 실제 할당 공간 1,600개 전체의 변경 내역도 보존했습니다. 첫 비교 시도의 시각 분류·할당 꼬리 관찰 오류는 미승인 이력으로 남기고 수정된 비교만 승인했습니다. Canonical all-targets 검사는 통과했으며 새 테스트나 전체 테스트 반복은 없었습니다. 이전 27개 테스트 증거는 이전 검증 범위로만 유지합니다. 진단 경로 9개·읽기 전용 snapshot·기본 pipeline·미컴파일 후속 네 보고서는 이번 적용에서 제외했습니다. 기존 ESO 수치 차이와 원인 미확정 상태, A14/54·B14/64 및 전체 FAIL은 유지하며 전체 실행·수치 동등성·내보내기 완료를 뜻하지 않습니다.


[후속 요약 보고 단계 검증](evidence/SRC-06-canonical-post-summary-core-09/summary.json). 크기 산정·음영·비용·적응 쾌적 보고, 초기화 요약·부하 보고 조건·열배출 보고의 Rust 파일 7개(신규 3개·기존 4개)를 반영했습니다. 실제 격리 Source77 실행은 이름 있는 표 75개를 작성한 뒤 열적 복원력 보고 5244 진입에서 exit6으로 끝납니다. 같은 입력 Native와 다섯 출력 형식의 앞부분 9,620줄을 대조해 9,588줄이 바이트까지 같으며, 버전 10줄·실제 시각 11줄·실행 날짜 11줄의 차이는 전체 원문 hex로 보존했습니다. Source 실행일은 10월 19일, 보존한 Native 실행일은 10월 17일입니다. EIO는 같은 열린 파일의 위치를 복원하며 읽습니다. 본체에는 그 메서드 2,186바이트만 추가하고 기존 진단 관측 변경 1,164바이트는 제외했습니다. Predefined 914개 항목과 ESO·MTR·EIO·원시 진단 출력은 직전 실행과 같습니다. Canonical 전체 타깃 컴파일을 통과했고 새 테스트나 전체 테스트 반복은 없습니다. 기본 pipeline과 진단 관측은 전환하지 않았습니다. 기존 ESO 수치 차이·원인 미확정 상태, A14/54·B14/64와 전체 FAIL은 유지하며 전체 실행·수치 동등성·내보내기 완료를 뜻하지 않습니다.


[실제 endrun 종료 경로 검증](evidence/SRC-06-canonical-completed-endrun-core-10/summary.json). 유한 A-24H 포팅의 복원력·주기·코일·사전 정의 보고서와 감사·파일 닫기·endrun 620까지 실제 Source 경로를 반영했습니다. 실제 격리 Source79는 이름 있는 표 191개를 작성했으며, 다섯 물리 출력 형식 전체 19,312줄 중 19,242줄이 Native와 바이트까지 같고 나머지 70줄은 버전·시계·날짜 메타데이터 차이로 원문을 보존했습니다. 초기 조명 입력 처리의 누락된 합계 세 항목은 실제 Native 생산자 순서로 복원했고, 같은 출력 스트림의 Windows 텍스트 모드를 적용했습니다. 실제 ESO 5,280개·MTR 72개 수치 레코드의 종료 기록과 감사·SHD 파일을 확인했습니다. Canonical Rust 파일 31개(신규 14개·기존 17개), 전체 컴파일 입력 4,467개를 검증했으며 기존 테스트 하나의 물리 줄바꿈 기대값 두 곳만 수정했습니다. 기본 pipeline과 진단 관측은 전환하지 않았으며, Source 결과 내보내기는 아직 명시적으로 중단됩니다. 기존 ESO 수치 차이 3,536개와 미규명 부동소수점 원인, A14/54·B14/64 및 전체 FAIL 상태는 유지합니다. 이는 endrun 완료 근거이며 전체 결과 내보내기나 Native 수치 동등성 완료를 뜻하지 않습니다.


[Source 실행·결과 내보내기 검증](evidence/SRC-06-canonical-completed-source-activation-11/summary.json). 명시한 `--porting-scope A/B` 실행을 실제 Source 경로에 연결했습니다. 같은 상태 소유자로 초기화·본 실행·종료 620을 거친 뒤 완성된 ESO/MTR만 읽고, 보고 ID·빈도·저장 방식·원문 값·Native 시간 튜플을 JSON/CSV로 보존합니다. 정식 A-24H 실행과 전체 대상 컴파일이 성공했고, 58계열 5,352개 값의 물리 출력·JSON·CSV 일치를 검증했습니다. 보고값 차이 3,536개는 변경하지 않은 기존 허용오차와 계열별 RMSE 한계 안에 있습니다. 기본 실행 경로는 보존하며, B는 처음 만나는 미구현 입력·분기에서 오류로 중단합니다. 진단 관측 파일은 배포 경로에서 제외했습니다. 추가 기간·B 사례·전체 포팅 게이트는 이 A-24H 검증으로 완료 처리하지 않습니다. A14/54·B14/64와 기존 전체 비교 기록은 유지합니다.


[Source B 입력·출력 형식·보고값 비교 검증](evidence/SRC-06-canonical-B-ingress-IO-reported-cases-12/summary.json). B 입력의 실제 값·확장 필드·선언 순서·양수 개수를 같은 InputProcessor에 보존하고, 실제 초기화의 Schedule:Compact 미구현 경계까지 진행했습니다. B 실행은 exit6으로 명시적으로 중단하며 완료 처리하지 않습니다. RDD/MDD는 원래 출력 시점에 Native와 같은 Windows 줄바꿈을 사용합니다. 같은 새 코어의 A-24H는 종료·내보내기에 성공했고 5,352개 값의 기존 전체 증명과 일치합니다. 이전 Source11의 A-72H 16,056개·연간 1,953,480개 보고값은 원래 허용오차·계열별 RMSE 실패가 모두 0입니다. 공개 스트림 비교 도구는 같은 알고리즘을 유지하며 A-72H 대조로 확인했습니다. 연간 비교는 반복하지 않았습니다. 숫자 차이와 정책 메타데이터의 기존 한계를 보존하며, 내부 비트 동일성·B 및 전체 15사례·카드 게이트 완료를 주장하지 않습니다. A14/54·B14/64는 유지합니다.


[Source Compact 스케줄 검증](evidence/SRC-06-canonical-compact-schedules-13/summary.json). Compact는 동일 InputProcessor·Schedule·Week·Day 소유자와 실제 시계를 사용합니다. Native1269의 같은 MissingDay0 재할당을 보존하며, 실제 A/B의 시간당 4단계에서는 24→96개 값으로 바뀝니다. 후속 B 연산은 미완료입니다.


[Source Zone·Node 입력 검증](evidence/SRC-06-canonical-Zone-Node-14/summary.json). Zone 제어 여부는 같은 InputProcessor의 실제 필드 조회로 설정하며, 알려진 부재 연결 유형에는 Native와 같은 Null을 삽입합니다. Node의 실제 개수·할당·NodeID0/Undefined·첫 확장 순서를 보존하고, 검증되지 않은 다중 IDF 순서는 명시적으로 중단합니다.


[Source 제어장비 입력 검증](evidence/SRC-06-canonical-controlled-equipment-15/summary.json). 제어장비 입력은 기존 Zone·Schedule·Sizing·Node·Branch 소유자를 직접 갱신합니다. AirIn의 실제 terminal index 초기값 0과 미초기화 outlet index를 구별하고, 생성자·배열 할당·호출 순서를 보존합니다.


[Source 체적·Node 경로 검증](evidence/SRC-06-canonical-Node-lifecycle-16/summary.json). 체적 계산은 Native의 최초 폐쇄 판정·부호와 양수 사용자 입력값 처리를 보존합니다. Node 보고는 같은 물리 필드·실제 스트림을 참조하고, 초기화는 지정된 19+6개 필드만 갱신합니다.
