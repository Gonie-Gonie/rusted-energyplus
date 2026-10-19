# SRC-06 — TARP 실내 대류

- 그룹: 05 열원·외부 경계
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: GEO-02, PSY-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/ConvectionCoefficients.cc`  
심벌: `CalcASHRAETARPNatural; CalcASHRAEDetailedIntConvCoeff; InitIntConvCoeff`  
대상 블록: TARP 수직·안정/불안정 경사 분기와 strict 하한 대입. 선택 원본에는 상한 clamp가 없음. InitIntConvCoeff에서는 해당 선택 경로만.
함께 읽을 선언/호출자: ConvectionCoefficients.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/ConvectionCoefficients.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

heat_balance/convection.rs; inside_convection.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

실내 표면온도·참조공기온도[°C], 경사

## 출력·변경 상태 계약

실내 h[W/(m²K)] 및 참조온도 snapshot

## 호출 시점

실내 반복의 정해진 계수 갱신 시점

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

양/음/0 ΔT; 수직·수평; 안정/불안정; clamp

## 연결시험

SUR-05에서 EP와 같은 반복 회차에 갱신

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

## 현재 구현 범위와 원본 경계

이 문서는 선택 단위의 현재 범위와 실제 증거를 모은다. 위 template의 Rust 재사용 기준 커밋은 최초 계획 당시 기준이며, 각 단위의 실제 Source·명령·실패 이력은 아래 요약에 고정되어 있다. 선택 단위 통과는 전체 카드 완료가 아니다. 네 gate는 모두 **미확인**, A **14/54**, B **14/64**다.

- **TARP 식:** `ConvectionCoefficients.cc:1907–1975`, `ConvectionCoefficients.hh:394–460`. 실제 `Tsurf−Tamb`, exact-zero·부호 분기, 저장된 `Surface.CosTilt`의 음수를 사용하고 strict `Hc < LowHConvLimit`만 대입한다. constructor 하한 0.1은 `DataHeatBalance.hh:1790`; 실제 입력 하한을 별도 보존한다. 상한 clamp·EPS·nonfinite floor는 없으며 HighHConvLimit는 이 식의 입력이 아니다. Template의 clamp 시험은 이 선택 하한을 뜻한다.
- **선택 dispatcher:** `InitIntConvCoeff:141–391`, 사용자 입력/정적 geometry/Scan prefix와 순서 있는 Zone→Space→HT traversal을 구현했다. SpaceAir·Config·SurfHConvInt는 호출자의 단일 저장소를 대여한다. 명시적 입력·완전한 ID/개수·지원 범위를 쓰기 전에 검증한다. 선택 example의 성공은 ordinary manager 활성화가 아니다.
- **ordinary 소유자:** 실제 raw 여섯 대류 family 개수와 하한, Zone ZT·Space MAT/ZT/ZTAV, In·Old·독립 Tmp를 보존한다. BeginEnvironment reset, Old←In, solver 반환값 Tmp→In, MAT commit과 disabled-space MAT/ZT copy의 한정된 전이를 연결했다. ZTAV를 이 copy로 덮어쓰지 않는다. geometry getter/Config와 공통 입력 저장소도 실제 caller에 연결했지만 전체 readiness는 미완성이다.
- **복사·수치 생산자:** LU와 Fix/ScriptF의 실제 선택 입력 replay, typed 오류 전달 및 N1 Source 분기를 연결했다. 단일 불투명 enclosure의 선택 Solar 초기화와 부분 상태 이동, 현재 기상 태양 상태, DesignDay 기상 API도 구현했다. 실제 radiation cache·sigma·ordinary 반복 caller 완료를 의미하지 않는다.
- **DD 습공기 관측·재현:** Native의 실제 두 SetUp 구간에서 함수 인자·중첩 호출·cache 최초 접근 전 상태와 192개 slot을 관측했다. 동일 BEFORE 입력·cache 상태의 Rust 재현은 두 구간 모두 끝까지 실행했으며, 962개 바깥 호출 중 241개와 11,585개 cache 이벤트 중 9,705개에서 차이를 확인했다. 최초 Psat 값 쓰기는 1 ULP 차이다. 전체 raw/Iterate·DP/RH 대입 경로와 원인은 미확인이다.
- **선택 World 기하:** 실제 Surface·창호 기본 생성과 병렬/window 배열 할당, Space/Zone 바닥·지붕 면적을 연결했다. 체적의 현재 수치 연결은 보존한 raw face를 사용하며 외기 행은 ID 매핑을 검증해 물리 Surface로 이동·역복원한다. 실제 재료에서 만든 Construction 종류·표면 목록, 진단·CTF 대입과 빈 부속 객체 조회를 거쳐 한정된 GetSurfaceData 본문 반환까지 연결했다. 초기 radiant enclosure는 후속 Solar가 같은 상태를 이동해 사용한다. 이어지는 Setup292–311에서 같은 입력 처리기의 장비·Node·gain 소유자와 빈 airflow/storm 입력, 불투명 class 분기 및 실제 trig 배열 해제를 연결했다. Setup313–369의 표면 순회·Process 기하·scratch와 마지막 Zone/Space reset도 연결했다. 같은 입력의 Native BEFORE kernel 대조에 이어 실제 A 생산자와 보존된 Process 상태의 선택 대조도 통과했다. 370–454의 같은 Zone/Space 면적 누적과 실제 A 선택 수치 대조도 연결했다. 455–547의 실제 높이 계산과 경고 순서, 선택 높이 대조도 연결했다. 548의 선택 닫힌 체적·Space 비율, 550–639의 중심점·경계·인접 배열·표면 수 쓰기도 연결했다. Native 표면 중심점·Zone 경계 대조를 통과했고, 실제 저장 U를 사용하는641–792 보고와 EIO 세 구간의 원문 대조도 연결했다.797의 같은 Solar 저장소 할당·Space 연결·반복 MRT 등록과800/801의 실제 기상 상태를 사용하는 Zone 온도 설정·검사도 연결했다. 그 성공 반환 뒤 HeatBalanceManager278–313의 같은 입력·Construction 소유자와 오류 흐름, 앞선1795에서 호출되는 불투명 CheckAndSet 속성 쓰기도 연결했다. 실제 InitSolar316을 같은 Solar 저장소에 연결했고, 뒤의 물리 연결은 초기화 배열·SourcePtr·보고 순서·Surface 역참조를 보존한다. 새 부모 호출 Native 대조에서 Solar 배열60개와 기존 재료·Construction·nominal51개를 합친111개 수치·676개 이산 검사가 통과했다. raw Zone 중심점·인접·표면 수, film 표,800/801 온도와 새 입력 개수·진단, HB320 내부발열 입력과 보존 상태는 아래 새 부모 호출 대조에서 확인했다. 시간별 내부발열·지속 방사 계산과 전체 transitive Setup/HB 반환은 남아 있다.

원본 호출 위치를 구분한다. BeginSim의 Solar 초기화는 **`HeatBalanceManager.cc:2617–2630`**이며 2629는 안내 출력, 2630이 `InitSolarCalculations` 호출이다. **`HeatBalanceSurfaceManager.cc:424–425`**는 공유 firstTime 조건의 `InitInteriorRadExchange`; **`HeatBalanceIntRadExchange.cc:610–614`**는 조건부 Solar F 복사다. HBSM615는 ZoneSizing Gather 분기의 닫는 행이며 Solar 초기화 위치가 아니다. 현재 Rust의 선택 초기화 순서 검증을 이 전체 Original caller의 완료로 확대하지 않는다.

## 실제 증거와 검증 범위

아래 46개 요약은 plan의 actual 항목 전부다. 색인/ZIP은 실제 존재하는 파일만 연결한다. QA는 각 단위의 당시 Source에서 수행한 기록이며 현재 전체 포팅의 수치 PASS를 대신하지 않는다.

| 단위 | 실제 증거 | 확인된 범위와 제한 |
| --- | --- | --- |
| TARP Natural / Detailed | [요약](../evidence/SRC-06-tarp-inside-selected/summary.json) | 28개 입력·60회 호출의 선택 수치 비교. 유한값은 비트·부호 있는 0까지 일치; 생성 nonfinite 6개와 Native 문맥 28개는 수치 PASS 제외. |
| 선택 dispatcher / setup | [요약](../evidence/SRC-06-tarp-dispatcher/summary.json) · [색인](../evidence/SRC-06-tarp-dispatcher/manifest.json) | 8개 사례·20회 실제 InitIntConvCoeff. 입력 24,033·pre-owner 8,963·endpoint 9,083 검사, 불일치 0. 선택 외 Native 문맥 264개 제외; ordinary 활성화 증거는 아님. |
| 불투명 geometry | [요약](../evidence/SRC-06-ordinary-geometry/summary.json) · [색인](../evidence/SRC-06-ordinary-geometry/manifest.json) | 실제 입력 join 2,255·선택 endpoint 36개, 불일치 0; 선택 외 19개 제외. Perimeter·Width/Height·Zone bounds·면적 누적 범위이며 전체 SetupZoneGeometry 증명은 아님. |
| 공기온도 / Old 소유자 | [요약](../evidence/SRC-06-ordinary-temperature-owners/summary.json) · [색인](../evidence/SRC-06-ordinary-temperature-owners/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-temperature-owners/evidence.zip) | 실제 Native 58,039 이벤트의 선택 대입 검증과 19개 Rust replay. reset·SpaceMAT·Old·MAT commit·disabled-space copy; solver·full ZTAV 동등성은 아님. |
| 단일 SpaceAir 대여 | [요약](../evidence/SRC-06-ordinary-shared-space-air/summary.json) · [색인](../evidence/SRC-06-ordinary-shared-space-air/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-shared-space-air/evidence.zip) | 기존 Native-paired Rust에 대한 전체 선택 Rust 회귀: 8개 사례·20회, 57,930 leaf 차이 0; 실행 metadata 53개 별도 join. 새 직접 Native 비교는 아님. |
| 독립 SurfTempInTmp | [요약](../evidence/SRC-06-ordinary-inside-tmp/summary.json) · [색인](../evidence/SRC-06-ordinary-inside-tmp/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-inside-tmp/evidence.zip) | Native 322,474 이벤트에서 solve/copy 각 74,682쌍 검증, 불일치 0. 실제 7개 사례의 allocation/reset/copy replay이며 Native solved 값은 테스트 전용; CTF 식 증명은 아님. |
| 소비되는 LU / inverse | [요약](../evidence/SRC-06-ordinary-radiation-lu/summary.json) · [색인](../evidence/SRC-06-ordinary-radiation-lu/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-radiation-lu/evidence.zip) | 실제 kickoff 1회·첫 warmup 3회, A/I 출력 288개 비트 일치. swap·tie·singular·tiny pivot 및 후속 warmup/run은 미검증. |
| GetZoneEquipment / 단일 Config | [요약](../evidence/SRC-06-ordinary-geometry-equipment/summary.json) · [색인](../evidence/SRC-06-ordinary-geometry-equipment/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-geometry-equipment/evidence.zip) | 29-family·field admission, surface 검증 → getter → volume; Node allocation은 NumNodes와 독립. 실제 getter pair 1개와 count/find 객체 7개를 재사용. 선택 Rust 8개·20회 전체 차이 0; full IP/HVAC readiness는 아님. |
| FixViewFactors / CalcScriptF | [요약](../evidence/SRC-06-ordinary-radiation-producer/summary.json) · [색인](../evidence/SRC-06-ordinary-radiation-producer/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-radiation-producer/evidence.zip) | 실제 Fix 1회·ScriptF 4회, F/ScriptF/EMISS 204개 비트 일치. N1 교환 0은 Source 근거이며 Native N1 증거 없음; N2/N3은 지원 불가. typed 오류는 실제 caller로 전달. |
| 선택 Solar 초기화 소유자 | [요약](../evidence/SRC-06-ordinary-solar-initialization/summary.json) · [색인](../evidence/SRC-06-ordinary-solar-initialization/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-solar-initialization/evidence.zip) | inside/outside 흡수율 분리, gain 소비 전 초기화, 오류 부분 상태 보존·성공 시 State 이동. 실제 기존 204개 비트와 Fix 진단 4개/iteration 검사 유지; 새 Solar owner/EIO Native 동등성은 미검증. |
| 현재 기상 태양 상태 | [요약](../evidence/SRC-06-ordinary-current-solar/summary.json) · [색인](../evidence/SRC-06-ordinary-current-solar/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-current-solar/evidence.zip) | 실제 Today·위치로 sun/previous-sun/SOLCOS·각도, sky/IR/cloud/snow, 월별 반사율·야간 현재 일사를 생산. Rust 회귀이며 새 Native 수치 동등성은 아님. |
| DesignDay 기상 / 호출 역할 | [요약](../evidence/SRC-06-ordinary-design-day-weather/summary.json) · [색인](../evidence/SRC-06-ordinary-design-day-weather/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-design-day-weather/evidence.zip) | 실제 DesignDay 2개·6회 호출, 전체 96-slot·입력 순서/ReverseDD·연도·보고 셀 12개 검증. strict-bit FAIL 9종·UNPAIRED 8종은 아래 제한 유지; 전체 Setup 완료 아님. |
| 단일 SurfHConvInt 대여 | [요약](../evidence/SRC-06-ordinary-inside-convection-borrow/summary.json) · [색인](../evidence/SRC-06-ordinary-inside-convection-borrow/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-inside-convection-borrow/evidence.zip) | dispatcher가 실제 SurfaceHB 계수를 직접 대여; 전체 개수·순서 ID를 쓰기 전에 검증. 기존 8개 테스트 통과; 식/cadence 변경·새 Native 수치 비교 없음. |
| 공통 InputProcessor / shortcut | [요약](../evidence/SRC-06-ordinary-shared-input/summary.json) · [색인](../evidence/SRC-06-ordinary-shared-input/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-shared-input/evidence.zip) | 입력 존재·JSON 크기, count 비변경·definition null 삽입, 실제 최대값의 여섯 배열·이동/오류 보존. 이 증거 당시 metadata 60종; 전체 parser/startup 증명은 아님. |
| 공통 보고 입력 getter / unused | [요약](../evidence/SRC-06-ordinary-shared-report-input/summary.json) · [색인](../evidence/SRC-06-ordinary-shared-report-input/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-shared-report-input/evidence.zip) | 실제 다섯 report family의 여섯 배열 reset→정확 identity erase→alpha/default 쓰기, cache/error-prefix·metadata 87종. fmt·quality·all-target Clippy와 기존 테스트 8/0/0; 전체 workspace 중복 실행·새 Native 비교 없음. |
| 실제 입력 형식 / IDF 필드 개수 | [요약](../evidence/SRC-06-ordinary-input-origin/summary.json) · [색인](../evidence/SRC-06-ordinary-input-origin/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-input-origin/evidence.zip) | 전체 입력 형식을 family order와 독립 보존; 프로젝트 네 family의 실제 IDF field count, IDF 전용 NumAlpha·빈 필드와 실패 시 원상 보존. 전체 GetProjectData/startup 활성화는 아님. |
| 실제 PostIP / 출력 사전 검색 | [요약](../evidence/SRC-06-ordinary-output-prescan/summary.json) · [색인](../evidence/SRC-06-ordinary-output-prescan/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-output-prescan/evidence.zip) | 실제 raw lexical 입력·첫 이름 표기·중복 count·공백/단위·wildcard와 반복/오류 상태를 보존. 전체 startup/RE2/API/monthly 완료는 아님. |
| 실제 여섯 초기화 handler / 파일·시간 전달 | [요약](../evidence/SRC-06-ordinary-source-initialization/summary.json) · [색인](../evidence/SRC-06-ordinary-source-initialization/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-source-initialization/evidence.zip) | 실제 guard·Psych/Fluid/Schedule·Project/EMS/Curve 호출, 동일 EIO·오류 prefix와 effective timestep/reporting 전달. 전체 startup/Setup 완료나 Native 수치 동등성은 아님. |
| 실제 실행 전 검사 / Sizing·출력 시간 연결 | [요약](../evidence/SRC-06-ordinary-run-prefix/summary.json) · [색인](../evidence/SRC-06-ordinary-run-prefix/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-run-prefix/evidence.zip) | 실제205/213–241의 검사·등록·bounds·서비스·8개 sizing getter, 동일 상태와 오류 prefix. 전체 startup/Setup·Native 수치 완료는 아님. |
| 실제 kickoff 직전 검사 / sizing 조정의 비활성 경로 | [요약](../evidence/SRC-06-ordinary-run-prefix-checks/summary.json) · [색인](../evidence/SRC-06-ordinary-run-prefix-checks/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-run-prefix-checks/evidence.zip) | 실제243–267의 Available·Branch fatal·DisplayString과 두 실제 비활성 sizing 조정 함수. Kickoff268/Reset270/Setup272·양의 sizing 수치 완료는 아님. |
| 실제 HB 입력 / 기상·출력 소유자 전달 | [요약](../evidence/SRC-06-ordinary-hb-input/summary.json) · [색인](../evidence/SRC-06-ordinary-hb-input/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-hb-input/evidence.zip) | 동일 IP·기상·EIO/MTD의 ProjectControl/Site/Material/Construction/Shadowing/Zone 입력과 기존 geometry 사용. 전체 GetHBInput/SurfaceTmp/Setup·수치 완료는 아님. |
| 동일 기상 상태 / Zone·Surface 외기와 오류 보존 | [요약](../evidence/SRC-06-ordinary-same-environment-outdoor/summary.json) · [색인](../evidence/SRC-06-ordinary-same-environment-outdoor/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-same-environment-outdoor/evidence.zip) | 동일 Station/Site/current와 Zone 온도→guard→풍속·풍향→기존 Surface. 실패한 실제 State를 이동 보존; 전체 InitHB/Setup·Native 수치 완료는 아님. |
| HeatBalanceAlgorithm 기본값 admission | [요약](../evidence/SRC-06-ordinary-hb-default-admission/summary.json) · [색인](../evidence/SRC-06-ordinary-hb-default-admission/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-hb-default-admission/evidence.zip) | 실제 numeric200/1000 기본값만 허용. 기존 lib1·일반 CLI1·실제 CI CLI1 확인; A 전후 실제 결과는 별도 보존하며 full Setup·Native 수치 PASS로 확대하지 않음. |
| 실제 World 표면 기하 연결 | [요약](../evidence/SRC-06-ordinary-surface-world-geometry/summary.json) · [색인](../evidence/SRC-06-ordinary-surface-world-geometry/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-surface-world-geometry/evidence.zip) | 명시적 ULC/CCW/World의 raw 꼭짓점·최종 연결·중심점과 두 직접 Surface 소비자를 연결. 기존 lib9·일반 CLI1·실제 CI CLI1 및 실제 A의 EIO·여섯 표면 경로를 확인. 전체 geometry/HB/Setup·Native 수치 동등성은 미확인. |
| DD 같은 입력 sky 계산 | [요약](../evidence/SRC-06-ordinary-design-day-same-input-sky/summary.json) · [색인](../evidence/SRC-06-ordinary-design-day-same-input-sky/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-design-day-same-input-sky/evidence.zip) | 실제 여섯 호출·576 slot의 캡처된 DB/DP/RH/opaque-cover 입력으로 IR/Sky 두 출력 1,152 strict bits 일치. whole-DD 9 strict 차이·8 UNPAIRED와 모든 gate는 그대로. |
| World 배열·바닥 면적·상태 전달 | [요약](../evidence/SRC-06-ordinary-surface-array-floor/summary.json) · [색인](../evidence/SRC-06-ordinary-surface-array-floor/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-surface-array-floor/evidence.zip) | 실제 기본 생성·배열 할당·면적 소유자와 raw face 수치 연결, 외기 이동·역복원. 기존 테스트 3개와 실제 A 성공; 전체 getter/Setup·Native 수치 동등성은 미완료. |
| 빈 표면 부속 객체 조회 | [요약](../evidence/SRC-06-ordinary-surface-zero-getters/summary.json) · [색인](../evidence/SRC-06-ordinary-surface-zero-getters/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-surface-zero-getters/evidence.zip) | 창·차양 등의 빈 getter, 실제 개수·출력 인자·정의 조회와 중심점 전후 호출을 연결. 기존 테스트 3개와 실제 A 성공; 전체 getter/Setup 반환은 미완료. |
| Construction 판정·표면 목록 | [요약](../evidence/SRC-06-ordinary-construction-surface-lists/summary.json) · [색인](../evidence/SRC-06-ordinary-construction-surface-lists/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-construction-surface-lists/evidence.zip) | 실제 Material group·Construction layer/type 판정과 순서 있는 Surface/Zone 목록·filter 연결. 기존 본문 +18줄·테스트 3개와 실제 A 성공; 전체 CheckAndSet/getter/Setup은 미완료. |
| 표면 입력 반환·radiant enclosure | [요약](../evidence/SRC-06-ordinary-surface-tail-radiant-enclosures/summary.json) · [색인](../evidence/SRC-06-ordinary-surface-tail-radiant-enclosures/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-surface-tail-radiant-enclosures/evidence.zip) | Source2795–2915의 진단·빈 경계 입력·CTF 대입, 같은 radiant 상태와 선택 본문 반환 연결. 기존 테스트 4개·실제 A 성공; 전체 Setup/HB 반환·수치 복사 동등성은 미확인. |
| DD 습공기 실제 입력·cache 관측 | [요약](../evidence/SRC-06-ordinary-design-day-passive-psy/summary.json) · [색인](../evidence/SRC-06-ordinary-design-day-passive-psy/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-design-day-passive-psy/evidence.zip) | 과거 F66 Source에서 Native 2개 구간·192 slot·5,583 호출/반환 쌍·최초 cache 접근 1,949개를 확인, 관측 검증 issue0. 이 관측 단위 당시 Rust 재현 미실행; 후속 같은 입력 재현은 아래 별도 FAIL 증거. 기존 9FAIL/8UNPAIRED 유지. |
| DD 같은 입력·cache 재현 | [요약](../evidence/SRC-06-ordinary-design-day-psy-root-cache-replay/summary.json) · [색인](../evidence/SRC-06-ordinary-design-day-psy-root-cache-replay/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-design-day-psy-root-cache-replay/evidence.zip) | 실제 두 구간·962개 바깥 호출·11,585개 cache 이벤트를 완주, **strict FAIL**. 호출 차이241·cache 차이9,705·최종 상태 차이 범주3; 최초 Psat 값 쓰기1 ULP. 전체 raw/Iterate·대입 경로 UNKNOWN, whole-DD9FAIL/8UNPAIRED 유지. |
| 표면 입력 이후 Setup 초기화 | [요약](../evidence/SRC-06-ordinary-setup-zone-continuation/summary.json) · [색인](../evidence/SRC-06-ordinary-setup-zone-continuation/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-setup-zone-continuation/evidence.zip) | Source292–311의 실제 오류 cleanup·같은 입력 처리기/장비/Node/gain 소유자·빈 airflow/storm·불투명 class skip·trig 배열 해제. 기존 테스트4개·두 crate Clippy·실제 A24 및 Rust 결과 전체 bytes 일치; Source313+·전체 Setup/HB 완료는 미확인. |
| DD 계산의 스레드 정밀도 진단 | [요약](../evidence/SRC-06-ordinary-design-day-psychrometric-precision-context/summary.json) · [색인](../evidence/SRC-06-ordinary-design-day-psychrometric-precision-context/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-design-day-psychrometric-precision-context/evidence.zip) | 두 입력의 독립 실행에서 x87 PC 비트 교환만으로 exp 결과1 ULP 차이가 양방향으로 바뀌고 복원됐다. 기존 전체9,949개 차이·whole-DD 실패와 과거 실제 프로세스 FP 상태/호출 대상은 별도 미확정. 생산 수식·unsafe 정책·gate 변경 없음. |
| Setup 표면 순회·Process 기하 | [요약](../evidence/SRC-06-ordinary-setup-process-surfaces/summary.json) · [색인](../evidence/SRC-06-ordinary-setup-process-surfaces/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-setup-process-surfaces/evidence.zip) | Source313–369의 순서 있는 표면 순회·scratch·Process 계산·Zone/Space reset을 연결했다. 실제 Native BEFORE 6개 사례·180개 scalar가 strict bits 일치한다. 최종 기존 테스트3개·정적 검사·실제 A24와 이전 Rust 결과 bytes 일치; 실제 World 생산자/endpoint 대조·370 이후·전체 gate는 미확인. |
| 실제 World 생산자·보존된 Process 상태 | [요약](../evidence/SRC-06-ordinary-setup-process-world-join/summary.json) · [색인](../evidence/SRC-06-ordinary-setup-process-world-join/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-setup-process-world-join/evidence.zip) | 실제 A 입력의 표면6·Zone1·Space1·Construction3 이름과 소속을 독립 연결했다. 꼭짓점·입력·보존된 Process/global 값270개 strict bits 및 이산 검사449개에서 차이·UNKNOWN0. 기존 CLI2개·A24와 정적 검사 통과. Surface 추적은 Full과 같은 기하·결과 bytes를 보존하며 전체 Setup/gate 완료는 아님. |
| Setup 같은 Zone/Space 면적 누적 | [요약](../evidence/SRC-06-ordinary-setup-area-accumulation/summary.json) · [색인](../evidence/SRC-06-ordinary-setup-area-accumulation/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-setup-area-accumulation/evidence.zip) | Source370–454의 순서 있는 면적·외벽 배수·둘레 대입을 같은 소유자에 연결했다. 기존270/449와 새20/15를 합쳐290개 수치·464개 이산 검사에서 차이·UNKNOWN0. 기존 테스트3개와 A24·정적 검사 통과. 창·지면·양의 debug 분기와455 이후·전체 gate는 미확인. |
| Setup 같은 Zone 높이·경고 순서 | [요약](../evidence/SRC-06-ordinary-setup-height-continuation/summary.json) · [색인](../evidence/SRC-06-ordinary-setup-height-continuation/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-setup-height-continuation/evidence.zip) | Source455–547의 실제 Space 범위 순회와 기존 높이 식을 연결했다. 자동 높이의 재계산을 제거해 입력 여부를 보존한다. 기존290/464와 새 높이1/입력 여부1을 합쳐291개 수치·465개 이산 검사에서 차이·UNKNOWN0. 기존 테스트3개·A24·정적 검사 통과; 경고 변형·전체 체적/Setup은 미확인. |
| Setup 선택 닫힌 체적·Space 비율 | [요약](../evidence/SRC-06-ordinary-setup-volume-continuation/summary.json) · [색인](../evidence/SRC-06-ordinary-setup-volume-continuation/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-setup-volume-continuation/evidence.zip) | Source548의 선택 닫힌 여섯 면 계산과 같은 Zone/Space 체적·비율 쓰기를 연결하고 뒤의 재계산을 제거했다. 기존291/465와 새 체적·비율3/ErrCount5 1을 합쳐294개 수치·466개 이산 검사에서 차이·UNKNOWN0. 기존 선택 테스트4회·A24·정적 검사 통과. 최초 작성 fixture 실패도 보존하며 전체 체적 분기/Setup은 미확인. |
| Setup 중심점·Zone 경계·인접·표면 수 | [요약](../evidence/SRC-06-ordinary-setup-zone-bounds-continuation/summary.json) · [색인](../evidence/SRC-06-ordinary-setup-zone-bounds-continuation/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-setup-zone-bounds-continuation/evidence.zip) | Source550–639의 같은 Zone 중심점·경계, 별도 SurfAdjacentZone 배열과 누적 표면 수를 연결했다. 실제 보존된 Surface 중심점18·Zone 경계6을 추가해318개 수치·487개 이산 검사에서 차이·UNKNOWN0. 기존 테스트3회·A24·정적 검사 통과. Native Zone 중심점·인접·표면 수와641 이후·전체 gate는 미확인. |
| Material·Construction NominalR/U 소유자 | [요약](../evidence/SRC-06-ordinary-construction-nominal-owners/summary.json) · [색인](../evidence/SRC-06-ordinary-construction-nominal-owners/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-construction-nominal-owners/evidence.zip) | 실제 재료 getter의 NominalR와 Construction 네 배열·층 합산·역수/오류 순서를 연결했다. 기존 테스트3회·A24·정적 검사와 기존318/487 기하 회귀 통과, 기하·결과 전체 bytes 동일. 최초 테스트 import 컴파일 실패·수정도 보존했다. 새 NominalR/U Native 대조와641 보고는 미검증. |
| Setup 실제 순서·저장 U·EIO 보고 | [요약](../evidence/SRC-06-ordinary-setup-reporting-continuation/summary.json) · [색인](../evidence/SRC-06-ordinary-setup-reporting-continuation/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-setup-reporting-continuation/evidence.zip) | Source641–792의 실제 보고 순서·저장 NominalU·표 항목과 Shading/Zone EIO를 연결했다. Native의 실제 header3/row3,1,115바이트가 정확히 일치하며 기존318/487 회귀도 통과했다. 기존 테스트3회·A24·정적 검사 통과; raw NominalR/U·film 표 값과797 이후·전체 Setup/HB는 미검증. |
| Setup 같은 Solar 저장소·Zone 온도 설정 | [요약](../evidence/SRC-06-ordinary-setup-solar-temperature/summary.json) · [색인](../evidence/SRC-06-ordinary-setup-solar-temperature/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-setup-solar-temperature/evidence.zip) | 선택 Source797의 같은 Solar 할당·Space 연결·반복 MRT 등록과800/801의 Zone 온도 설정·검사를 연결했다. 기존 테스트3회·A24·정적 검사와318/487·EIO1,115바이트 회귀 통과; 기하·결과·EIO 전체 bytes 동일. raw Solar·800/801 온도 Native 대조와 전체 transitive Setup/HB는 미확인. |
| GetBuildingData 이후 HB 입력·불투명 Construction 속성 | [요약](../evidence/SRC-06-ordinary-hb-pre-solar-input/summary.json) · [색인](../evidence/SRC-06-ordinary-hb-pre-solar-input/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-hb-pre-solar-input/evidence.zip) | HBM278–313의 같은 입력/오류 소유자와 원본1795의 불투명 CheckAndSet 속성을 연결했다. 최종 기존 테스트3회·정적 검사·A24와318/487·EIO1,115바이트 회귀 통과; 최초 테스트 기대값 실패·수정도 보존. 새 count/진단/속성 Native 대조는 UNPAIRED이며 실제 InitSolar316·전체 HB는 미완료. |
| Solar·Material·Construction·nominal 실제 상태 | [요약](../evidence/SRC-06-ordinary-setup-solar-material/summary.json) · [색인](../evidence/SRC-06-ordinary-setup-solar-material/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-setup-solar-material/evidence.zip) | 새 Native 실행의 같은 입력·실제 Source 순서/ID와 전체 LayerPoint·네 nominal 배열을 연결했다. 새51개 유한 binary64·321개 이산 검사와 기존318/487·EIO1,115바이트 모두 차이0·UNKNOWN0. 읽기 전용5파일·기존 CLI1개만 실행했으며 새 테스트 body/bin은 없다. 온도800·후행 Solar 수치와 전체 Setup/HB·gate는 미검증. |
| 실제 HB316 Solar 초기화 | [요약](../evidence/SRC-06-ordinary-hb-solar-initialization/summary.json) · [색인](../evidence/SRC-06-ordinary-hb-solar-initialization/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-hb-solar-initialization/evidence.zip) | 원본 부모 호출의 같은 F36·네 배열24개와 기존51개 수치, 총111개 binary64·676개 이산 검사에서 차이0·UNKNOWN0. 원본9개 실제 호출·22개 함수 연결을 확인했다. 기존3회 테스트·두 본문을 재사용했으며 새 body/bin은 없다. 전체 HB·ScriptF·온도·오류/보정 분기는 미검증. |
| 실제 HB320 내부발열 입력 | [요약](../evidence/SRC-06-ordinary-hb-internal-gains-input/summary.json) · [색인](../evidence/SRC-06-ordinary-hb-internal-gains-input/manifest.json) · [ZIP](../evidence/SRC-06-ordinary-hb-internal-gains-input/evidence.zip) | 실제 부모 반환의 새226개 binary64·867개 이산 검사와 nominal EIO1,576바이트가 일치했다. 기존 Solar와 합쳐337개 수치·1,543개 이산 검사를 통과했다. Source20·실제 A24·정적 검사와 기존 테스트4회 중2회 재사용을 확인했다. 시간별 발열·전역 DD 절대번호·전체 HB는 미검증. |

기존 Weather03·absent site/station과 cold guard는 [SRC-05 실제 선택 비교](../evidence/SRC-05-weather-cold/summary.json)에 별도로 고정되어 있다. 이는 complete ordinary weather/inside 배열·reachability·handoff의 완료 증거가 아니다.

## 실패와 미대조 수치의 현재 상태

| 항목 | 유지할 판정 | 의미 |
| --- | --- | --- |
| [전체 ordinary main-thread 비교](../evidence/CTF-10-main-thread-history-comparison/summary.json) | **FAIL 유지**: mismatch 235,456; required unavailable/unmatched 2,754 | 과거 실제 전체 비교다. 후속의 한정된 owner·수치 replay·Rust 회귀가 이 실패를 재검증하거나 대체하지 않았다. 현재 전체 동등성은 미확인이다. |
| DesignDay 계산값 strict-bit 진단 | **FAIL 유지**: 차이 9종, 최대 절대 차이 `2.2737367544323206e-13` | 원인 미확정. 새 허용오차로 PASS로 바꾸지 않았다. 기존 PSY 계약의 caller-output 5종 평가는 중간 입력 비트·cache 동등성 증거가 아니다. |
| DesignDay 계산값 8종 | **UNPAIRED**, 수치 PASS 없음 | `OutRelHum`, `OutRelHumValue`, `SkyTemp`, `SkyTempKelvin`, `HorizIRSky`, `BeamSolarRad`, `DifSolarRad`, `GndSolarRad`. 유한-domain·class/부호 있는 0 검사는 수치 크기 판정이 아니다. |
| DD 같은 입력·cache 선택 비교 | **FAIL**: 호출241·cache 이벤트9,705·최종 상태 범주3 | 동일 Native BEFORE 상태에서 재현. 최초 Psat 값 쓰기1 ULP이며 원인은 미확정. 전체 raw/Iterate·caller 대입 경로를 대조한 증거는 아니다. |

DesignDay의 정수·플래그·배열 범위·보고 문자열과 Rust 내부 Today→Tomorrow 전체 carrier copy는 별도의 exact 검증이다. Native의 선택 psychrometric 입력/cache를 사용한 같은 상태의 Rust 재현은 strict FAIL이다. 최초 Psat 값 쓰기부터 1 ULP 차이가 재현되며, 전체 계산값·대입 경로의 동등성과 차이의 원인은 미확인이다. Natural의 생성 nonfinite 6개, dispatcher의 Native-only 264개와 geometry의 선택 외 19개도 수치 PASS에 포함하지 않는다.

## 남은 실제 연결 작업

1. **전체 startup / SetupSimulation:** 실제 초기 getter·Fluid/Schedule/Sizing·출력 등록과 파일 host·Exterior·HB·Cost/Fault/Curve를 원본 순서로 연결한다. DesignDay 2개를 방문하는 실제 kickoff를 weather API만으로 대체하지 않는다. 공유 flag·전역 상태를 raw 스위치나 Ready bool로 만들어내지 않는다.
2. **InitHeatBalance / InitSurface 전체 경로:** 실제 AllocateHeatBal·CTF/window/Solar prefix, InitInterior radiant owner와 Solar F 복사, 흡수율·sky/solar/internal gain/distribution·초기 convection·CTF constants·계수 reset·BeginSim report suffix가 모두 성공한 뒤에만 원본 공유 firstTime을 지운다. 일부 생산자나 보유 중인 제안의 존재는 전체 연결 완료가 아니다.
3. **ManageSurfaceHeatBalance / inside loop:** 실제 initial Tmp와 반복 In의 구분, radiation 이후 양의 완료 iteration에서만 30회 간격 갱신(`HBSM:473`, `7944–7946`, `8824–8826`), 즉시 surface term 재계산과 수렴·오류 순서를 연결한다. 현재 ordinary dispatcher는 활성화되지 않았다.
4. **HVAC / zone solution / handoff:** 독립 Node allocation과 Config getter 이후의 equipment·availability·Manage/Sim tails·EMS readiness를 실제 소유자로 생산해야 한다. 기존 Rust A의 SimplifiedAnalytical과 관측 Native ThirdOrder는 같다고 검증하지 않았다. 전체 ZTAV 평균, 다중 Space·명시 reference flag0, 후속 warmup/run과 최종 handoff 비교도 남아 있다.
5. **선택 외 분기:** CON-01 제외 범위인 Kiva·window·EMS/override·대표 surface 및 미지원 입력을 성공으로 대체하지 않는다. 실제 TARP/방사 수치 관측 밖의 pivot swap/tie·singular/tiny·경고 domain도 별도 증거가 필요하다. Timestep 객체의 생략 필드는 schema6이지만 객체 자체 부재의 Original/compiler 차이는 지원 불가로 유지한다.

## 공통 보고 입력 단위의 실제 QA

최종 Source `e181f6ff45f1957ff0299fe7fd414e68c4e59e68935f169bb73dd2e9765339ac`에서 fmt·source-quality·all-target workspace Clippy가 통과했다. 기존 report 관련 6개·raw 1개·CLI 1개 테스트가 통과해 합계 **8 passed / 0 failed / 0 ignored**다. 테스트 본문·binary는 늘리지 않았고 전체 workspace를 중복 실행하지 않았다. 직전 전체 baseline **4,974/0/2**는 과거 Source의 검증으로 유지한다.

Assembly01의 실제 Clippy 실패(exit101)는 보존했다. Assembly02는 중첩된 테스트의 잘못된 module 경로 두 곳만 고쳤으며 최종 QA는 수정본에서 수행했다. plan의 `actual_selected_ordinary_shared_report_input`과 위 실제 요약/색인/ZIP이 이 이력을 고정한다. metadata 87종은 positive-family admission 확대가 아니며 전체 unused-input reporting·parser/InputProcessor ABI·startup/Setup 완료를 주장하지 않는다.

## 실제 입력 형식 단위의 QA

최종 Source `bc9440f54851344b6fd373d9256edb72e5b70bd3ba22b8a0cda847b094d84dfa`에서 fmt와 RawModel 전체 16개·compiler report 8개·raw admission 1개 테스트, 합계 **25 passed / 0 failed / 0 ignored**를 확인했다. 새 테스트 본문·binary는 없다. all-target Clippy와 source-quality는 직전 assembly01에서 통과했고, 이후에는 두 테스트 입력 쌍의 누락 객체만 고쳤다. 다섯 파일의 production bytes가 동일함을 검토해 두 정적 검사와 전체 workspace 테스트는 반복하지 않았다. 과거 RawModel 15 passed/1 failed(exit101) 기록은 별도 Source로 보존했다. IDF `Output:VariableDictionary;`의 실제 NumAlpha0과 epJSON의 min_fields1을 구분하며 기본 alpha 값 REGULAR는 둘 다 보존한다. 모든 gate와 A/B 완료 수는 그대로다.

## 실제 PostIP 사전 검색 단위의 QA

최종 Source `d277882b8cf1a63bda2fff8bf8b2dd2574524f2f8829eb67b286cc867a45983c`에서 fmt·all-target workspace Clippy와 기존 raw admission·실제 CLI 테스트 **2 passed / 0 failed / 0 ignored**를 확인했다. 새 테스트 본문·binary는 없다. source-quality는 직전 Source에서 통과했으며, 이후 child 함수 공개 범위 한 토큰만 고쳐 전체 파일 역변환과 나머지 여섯 파일 동일성을 확인했다. 첫 컴파일의 실제 E0364/E0603 실패와 supplemental diff의 파일명 header 오기는 역사 및 별도 정정 증거로 남겼다. 입력·상태·scratch/unused 보존의 선택 검증이며 전체 workspace 중복 실행이나 새 Native 수치 비교는 수행하지 않았다. 모든 gate와 A/B 완료 수는 그대로다.

## 실제 초기화 연결 단위의 QA

최종 Source `29be17da9f5e9cbaf9ac8cbc47913ddebfac121226093acefe385c984515f4b8`에서 fmt·source-quality·all-target workspace Clippy와 입력·출력 오류 2개·일반 CLI 1개·실제 `CI_FORCE_TIME_STEP=Y` CLI 1개를 확인했다. 캐시·기상 소유자 16개는 직전 Source `59192f3a1eb35f4e718c43e9203d9f1471431f3774447f8a3ac03c2d4075ba27`에서 통과했고 해당 전체 파일 bytes가 그대로여서 반복하지 않았다. 선택 검증 합계 **20 passed / 0 failed / 0 ignored**이며 과거 중복 성공은 합산하지 않는다. 새 테스트 본문·binary는 없다. 강제 timestep 실행이 raw6/effective2 장비 입력 불일치를 실제로 발견했고, 원본 GetZoneEquipment의 global timestep을 전달해 수정했다. 첫 Clippy exit101과 강제 CLI wrapper exit1(내부 Cargo101), 실패 당시의 24개 입력·출력 파일은 별도 Source와 로그로 보존했다. 전체 workspace 반복 실행·새 Native 수치 PASS는 없다. 모든 gate와 A/B 완료 수는 그대로다.

## 실제 실행 전 검사 연결 단위의 QA

Source `424bf058824956e69d1e29309f76e5976bb8a235772c9828e5d4fdd1e8fa8ae1`에서 fmt·source-quality·all-target workspace Clippy와 기존 입력/출력 2개·일반 CLI 1개·실제 `CI_FORCE_TIME_STEP=Y` CLI 1개, **4 passed / 0 failed / 0 ignored**를 확인했다. 기존 output/fatal 본문에 SummaryReports의 정확한 unused erase, 실제215 이후218 재호출 fatal과234 실패 시 DoingSizing=true 보존을 추가했다. 새 테스트 본문·binary와 전체 workspace 반복 실행은 없다. actual 파일 16개의 전체 diff·독립 검토와 실제 QA Source를 고정했으며, 앞 단위의20개 성공은 중복 합산하지 않는다. 모든 gate와 A/B 완료 수는 그대로다.

## 실제 kickoff 직전 검사 단위의 QA

Source `7fc13f241290a1edb5caaba011937b9328020662df20820ddaf9b680f08458ca`에서 fmt·source-quality·all-target workspace Clippy와 기존 입력/출력 2개·일반 CLI 1개·실제 `CI_FORCE_TIME_STEP=Y` CLI 1개, **4 passed / 0 failed / 0 ignored**를 확인했다. 기존 본문에서 세 실제 진행 문자열 순서와249 Available·kickoff 미대입을 검사했다. 새 테스트 본문·binary·전체 workspace 반복·새 Native 실행은 없다. 앞 단위의 성공을 중복 합산하지 않으며 실제 양의 sizing/Branch fatal 실행은 이 테스트의 PASS에 넣지 않는다. 모든 gate와 A/B 완료 수는 그대로다.

## 실제 HB 입력 연결 단위의 QA

Source `7cef065bae90e71aafb7bc3d64fc788ce8a06d0f1199b3c3474bc72fbbca8ba8`에서 fmt·source-quality·all-target workspace Clippy와 기존 테스트 **16 passed / 0 failed / 0 ignored**를 확인했다. raw IDF 필드 개수1·compiler 보고 입력8·runtime3·입력/오류2를 네 패키지의 한 Cargo 선택으로 묶어14개를 실행했고, 일반 CLI1·실제 `CI_FORCE_TIME_STEP=Y` CLI1을 확인했다. 실제 Station→Building→Site→Shadow EIO 순서·MTD와 출력 getter의 local scratch/unused/오류 prefix를 검사했다. Source01 컴파일 실패101, Source02 Clippy와 오래된 staged-IDF 불일치 실패101·부분 성공10개를 보존했다. API/import/기존 호출부와 동일 출력·문자 집합의 lint 문법 및 해당 fixture를 수정한 최종 Source03에서 확인했으며 생산 검증은 완화하지 않았다. 새 본문·binary·전체 workspace 중복 실행·새 Native 실행은 없다. 앞 단위의 성공은 합산하지 않으며 모든 gate와 A/B 완료 수는 그대로다.

## 동일 기상 상태 외기 연결 단위의 QA

Source `0a08f1192e7375d816f0119e01b79b5fb4dc222a0902d9c8ba66f0ca5b0abd4a`에서 fmt·source-quality·all-target workspace Clippy와 기존 테스트 **7 passed / 0 failed / 0 ignored**를 확인했다. runtime 한 선택으로 기존 외기 guard4·초기화 table1을 묶고 일반 CLI1·실제 `CI_FORCE_TIME_STEP=Y` CLI1을 실행했다. 기존 본문3개에서 기상 매개변수·부호 있는0·cold fatal 이전 완료 온도/미실행 wind 및 실제 Surface/Solar와 작성한 Zone/global의 이동 보존을 검사했다. Source01 Clippy101과 이전 bytes/로그를 보존했고, 같은 마지막 컨텍스트를 직접 전달한 Source02에서 확인했다. 실제 전체 runtime 실패/새 Native endpoint는 이 PASS의 범위가 아니다. 새 본문·binary·전체 workspace 반복·새 Native 실행은 없다. 앞 단위 성공을 합산하지 않으며 모든 gate와 A/B 완료 수는 그대로다.

## HeatBalanceAlgorithm 기본값 admission 단위의 QA

Source `194be5e62f5dcbb34d9a52adfd41883dc8cdc8d22d2fb743e771ee32f3e6dd8e`에서 fmt·source-quality·영향받는 ep_run all-target Clippy와 기존 테스트 **3 passed / 0 failed / 0 ignored**를 확인했다. 같은 admission 본문에 열 개 표 행을 추가하고 일반 CLI1·실제 CI CLI1을 실행했다. A-24H 전후 실제 exit는 각각 6 / 0이며 실제 diagnostics와 함께 보존한다. A 실행은 테스트 수에 더하지 않고 성공·수치 동등성을 미리 가정하지 않는다. 전체 workspace 반복·새 Native 실행은 없으며 모든 gate와 A/B 완료 수는 그대로다.

## 실제 World 표면 기하 연결 단위의 QA

최종 Source `f793d3c3f896c106989d1c983fc3cc16562cee49b7cd5652d60de93a6fef83ac`에서 fmt·source-quality·all-target workspace Clippy와 기존 테스트 **11 passed / 0 failed / 0 ignored**를 확인했다. 한 Cargo 선택으로 geometry5·초기화1·raw2·admission1을 묶었고 일반 CLI1·실제 `CI_FORCE_TIME_STEP=Y` CLI1을 실행했다. 새 테스트 본문·binary와 전체 workspace 테스트 반복은 없다.

같은 A-24H 입력의 실제 실행은 exit0이며, 새 원본 getter가 기록한 유일한 Surface Geometry EIO 행과 변환된 여섯 raw 표면을 확인했다. 이전 기본값 admission 실행에는 해당 행이 없다. 실제 출력 24 hourly samples는 96개 Source timestep·전체 Setup·Native 수치 동등성의 증거로 확대하지 않는다. 빠진/다른 좌표 규칙의 기존 경로도 유지한다.

Source01의 지수 문법 컴파일 실패101과 Source02의 Clippy 일곱 항목 실패101 및 당시 bytes·로그를 보존했다. 동일한 `1.0e-6` 임계값, 마지막 scratch 전달·같은 반개구간 조건·Wall 기본 생성·Kiva 대입 순서를 유지한 Source03에서 최종 QA를 수행했다. raw GetVertices 결과와 중심점은 두 직접 Surface 소비자가 재사용하지만, Zone 체적의 기존 `ZoneVolumeFace::from_surface` 경로는 아직 형상을 재계산한다. 병렬/window 배열·전체 getter 반환·후속 보고/enclosure·HB tail/firstTime 완료는 남아 있다. 기존 전체 비교 실패와 DesignDay 수치 실패, 네 gate 및 A/B 완료 수는 그대로다.

## DD 같은 입력 sky 계산 단위의 QA

실제 Source `8242dfa9e85e325478e13b5897481b9a347196b08ae352e32cc72ece170b0a82`에서 기존 sky4+lifecycle1 결과는 **5 passed / 0 failed / 0 ignored**다. 1,152 strict bits 일치이며 실패의 원인을 추정하거나 whole-DD 수치 PASS로 확대하지 않는다. fmt와 영향받는 ep_runtime all-target Clippy는 실제 최종 Source에 고정하고, quality01은 production 두 파일 전체 동일·test-only 한 줄 수정의 명시적 범위로 재사용했다. 앞선 Clippy01 E0433=101과 역사적 whole-DD strict 실패는 별도 보존한다. 새 Native·A/CLI/CI 실행이나 새 tolerance는 없다.

## World 배열·바닥 면적·상태 전달 단위의 QA

실제 Source `bd998f82e3356116b869a06566ab072be12cf3aefc8eb47ad566f8ff673a6715`에서 fmt·source-quality·영향받는 ep_runtime all-target Clippy와 기존 테스트 **3 passed / 0 failed / 0 ignored**를 확인했다. 기존 runtime 본문 한 곳에 실제 배열 할당·초기값·비자명한 ID 순열·잘못된 매핑의 이동 전 거부·부호 있는0을 포함한 이동/역복원을 묶었고, 일반 CLI1·실제 CI CLI1을 실행했다. 새 본문·binary와 변경되지 않은 geometry 수학 테스트의 반복은 없다. 이 API 검증을 실제 Solar 오류 endpoint나 Native 배열 관측으로 확대하지 않는다.

같은 Source의 실제 A-24H 실행은 exit0, 24 hourly samples·390계열이며 직전 World A의 result-store와 전체 bytes가 같다. Source 바닥 면적은 후속 초기화에서 덮어쓰지 않고, 현재 체적 수치 연결은 이전 World 단위의 typed face 재계산 대신 보존한 raw face를 사용한다. 완전한 Original Setup의 높이/체적 경고·보정·Space 체적 순서, 후속 getter·enclosure·보고와 full 반환은 남아 있다. 기존 전체 비교·DesignDay 실패와 네 gate, A/B 완료 수는 그대로다.

## 빈 표면 부속 객체 조회 단위의 QA

실제 Source `d3cbb350b19a368bb39086ebfbba6d5601f69123bce155c409b296ce16e3a318`에서 fmt·source-quality·영향받는 ep_runtime/ep_run all-target Clippy와 기존 테스트 **3 passed / 0 failed / 0 ignored**를 확인했다. runtime1·일반 CLI1·실제 CI CLI1을 재사용했고 새 테스트 본문·binary는 없다. 같은 Source의 실제 A-24H 실행은 exit0, 24 hourly samples·390계열이며 직전 배열·바닥 면적 단위와 result-store 전체 bytes가 같다.

원본 30개 count와 실제 출력 인자 reset 위치를 보존하고, 개수0에서도 실행되는 창·차양 정의 조회·null 삽입·공유 module 변경·InternalMass 재조회, 중심점 전후의 Zone 검사·태양광 네 조회·가동 단열 조회를 연결했다. 이는 선택된 빈 객체 경로의 Source 검증과 Rust 실행이며 개별 getter endpoint의 새 Native 관측은 아니다. 양성 객체 도메인, 후속 Construction/list/filter·enclosure·보고·fatal과 전체 getter/Setup 반환은 남아 있다. 기존 전체 비교·DesignDay 실패와 네 gate, A/B 완료 수는 그대로다.

## Construction 판정·표면 목록 단위의 QA

실제 Source `f66ef5df4f6b8542349fd7689ee8f89fcd4d92a1f0c2e7bfbca5186eb1eed8b6`에서 fmt·source-quality·영향받는 ep_runtime all-target Clippy와 기존 테스트 **3 passed / 0 failed / 0 ignored**를 확인했다. 기존 6개 표면 fixture 본문에 18줄을 더해 실제 목록 생성 호출, 전역/Zone 목록의 Source 순서와 분류, 원본의 exterior Roof→InteriorCeilings 동작을 검사했다. 새 테스트 본문·실행 선택·가짜 InputProcessor 설정은 없다. 작성한 Default ConstructionUsage의 검증이며 실제 Material 종류 생산자의 Native 관측은 아니다.

같은 Source의 실제 A-24H는 실제 Material/NoMass group·Construction layer/type 생산자와 목록 경로에서 exit0, 24 hourly samples·390계열이다. result-store는 직전 빈 객체 조회 단위와 전체 bytes가 같다. Construction의 선택 type 대입을 전체 CheckAndSet의 광학·열·NominalU·오류 순서 완료로 확대하지 않는다. 후속 warning/errTracking·getter·enclosure·보고·fatal과 전체 getter/Setup 반환은 남아 있다. 리뷰 작성 중 두 사전검증 오류는 실제 Rust QA 실패와 구분해 보존했다. 기존 전체 비교·DesignDay 실패와 네 gate, A/B 완료 수는 그대로다.

## 표면 입력 반환·radiant enclosure 단위의 QA

최종 Source `04bd56182cc9ba37e1a48991bef243fa7ebfb85151cb9f1571607cc05a80efcc`에서 fmt·source-quality·영향받는 ep_runtime/ep_run all-target Clippy와 기존 테스트 **4 passed / 0 failed / 0 ignored**를 확인했다. 기존 runtime·backend 본문 두 곳을 보강하고 일반 CLI1·실제 CI CLI1을 실행했다. 새 테스트 본문·binary나 전체 workspace 반복 실행은 없다. 초기 Source8cda의 Clippy101과 독립 검토한 한 곳의 오류 전달 문법 수정, 적용 전 Surface 기본 알고리즘 Invalid 정정 이력을 별도로 보존했다.

원본2795–2915의 message-only 진단 후 일괄 개수 갱신, 빈 vented/exposed·override/물성 조회·실제 CTF/EIO, 초기 radiant enclosure·출력 등록, 빈 Ground/Surrounding/Local 입력과 두 fatal predicate·active/representative 대입을 연결했다. 실제 Surface 생성자의 Invalid와 전역 CTF 기본값을 구분한다. 후속 Solar는 초기 radiant 상태를 이동해 쓰고 실패 시 실제 부분 상태·외기 소유자를 보존한다. 한정된 GetSurfaceData 본문 반환은 전체 SetupZoneGeometry나 HB 입력 완료 flag가 아니다.

같은 Source의 A-24H는 exit0, 24 hourly samples·390계열이며 기존 result-store와 전체 bytes가 같다. 실제 유일한 World Surface Geometry·CTF EIO 행도 확인했다. 양성 부속 객체 분기·요청 MRT 수치·InitInterior F/ScriptF·전체 Setup 반환과 Native 수치 비교는 남아 있다. 기존 전체 비교·DesignDay 실패와 네 gate, A/B 완료 수는 그대로다.

## DD 습공기 실제 입력·cache 관측 단위

과거 Source `f66ef5df4f6b8542349fd7689ee8f89fcd4d92a1f0c2e7bfbca5186eb1eed8b6`에서 승인한 Native Source08·build05·provider04·capture/checker05로 실제 관측을 완료했다. Native child 1회 exit0 후 checker exit0, 실제 두 SetUp 구간·192개 slot·5,583개 호출/반환 쌍·1,949개 최초 cache 접근을 확인했고 issue는 0이다. 최초 접근 전 tag/value와 실제 scalar 상태를 보존하므로 cold cache를 가정하지 않는다. 별도 독립 검토는 세 작은 결과 파일의 모든 행과 receipt/custody 연결을 확인했다.

이 관측 단위 당시에는 Rust 같은 입력 재현을 실행하지 않았고 DP/RH는 **UNKNOWN**이었다. 후속 재현 결과는 별도 단위로 아래 기록한다. whole-DD 9FAIL/8UNPAIRED와 네 gate·A/B 완료 수는 유지한다. 현재 표면 입력 Source의 QA를 이 과거 관측에 빌려 쓰지 않는다. 두 번의 실제 Native 빌드 실패·두 provider 중단과 최종 Source/명령/로그를 보존했고, 공개 ZIP은 약 1.3MB의 유한 자료로 정리했다. 대형 helper·core·map·raw/events와 바이너리 provider는 기존 식별 metadata만 남겼으며, 이 문서화 과정에서 다시 읽거나 실행하지 않았다.

## DD 같은 입력·cache 재현 단위의 QA

최종 Source `60019ac26f0f0a2731011613a72d287ff03f81185284b224ee1f2d83743ddf65`에서 fmt·source-quality·ep_runtime all-target Clippy는 exit0이다. 기존 기상 lifecycle 본문 한 개의 일반 실행은 **1 passed / 0 failed / 0 ignored**, 실제 Native BEFORE 입력·cache 상태를 공급한 선택 실행은 **0 passed / 1 failed / 0 ignored**, exit101이다. 새 테스트 본문·binary·전체 workspace 반복 실행은 없다. 초기 실제 Clippy101, 수정·재검증과 별도 작은 lint 탐색 실패는 구분해 보존했다.

실제 두 구간의 962개 바깥 호출과 11,585개 cache 이벤트를 모두 비교했다. 호출 출력241개·cache 이벤트9,705개·최종 상태 차이 범주3개, 합계9,949개 진단 차이이며 UNKNOWN 중단이나 누락 구간은 없다. 첫 구간 cache 이벤트24의 Psat 값 쓰기는 같은 인자·index·tag에서 Rust `40f4cb6a380f9204`, Native `40f4cb6a380f9203`으로 1 ULP 다르다. 최초 DP 바깥 호출은 40 ULP 차이다. 모든 cache 차이와 바깥 호출 비트는 실제 stderr 및 인증된 Native 기대 행에 연결해 보존했다. 최종 상태 범주 수는 승인된 진단 코드와 stderr에 근거하며 최종 scalar/slot payload 전체를 재출력한 것은 아니다.

재현은 테스트에만 연결하며 실제 첫 접근 BEFORE 상태를 한 번만 공급하고 이미 접근한 slot은 다시 덮어쓰지 않는다. 실제 캐시를 사용하는 함수와 중첩 호출을 실행하며 Native AFTER 값을 생산 계산에 넣지 않는다. 전체 saturation/RH×100 대입·raw/Iterate·backend/CalledFrom 경로는 **UNKNOWN**이다. 수학 라이브러리나 컴파일러를 원인으로 확정하지 않았고, 허용오차·생산 수식을 바꾸지 않았다. 기존 whole-DD9FAIL/8UNPAIRED·네 gate·A/B 완료 수는 그대로다.

## 표면 입력 이후 Setup 초기화 단위의 QA

최종 Source `da81210e456a2d5ff9e1949789dca08d895aa406fa5518f0fc5c04319dcf8e1d`에서 fmt·source-quality·ep_runtime/ep_run all-target Clippy는 exit0이다. 기존 runtime 상태 검증1·장비 입력 fixture1·일반 CLI1·실제 CI 시간간격 CLI1, 합계 **4 passed / 0 failed / 0 ignored**다. 새 테스트 본문·binary·Native 실행이나 전체 workspace 반복은 없다. 기존 장비 fixture의 과거 Native 근거를 현재 Source 장비 경로의 새 Native endpoint 관측으로 해석하지 않는다.

선택한 GetSurfaceData 반환 이후 원본292의 오류 정리, 298의 같은 입력 처리기와 실제 Zone/Space/global 개수·연결을 사용하는 장비 초기화, 300/302의 빈 airflow/storm 입력과 할당 상태, 308의 class 우선 불투명 분기, 310/311의 실제 trig 배열 해제를 연결했다. 장비 입력용 인자 배열은 local이며 공유 shortcut을 덮어쓰지 않는다. 실제 schema 여섯 정의의 부재/빈 최대 alpha 개수1/2/8/7/2/2와 numeric0을 사용한다. Source 경로는 family projection을 다시 만들지 않으며 기존 legacy 경로의 projection만 명시적으로 보존한다. 같은 Construction의 CTF 사용 표시는 World GetSurfaceData의 실제 writer 한 곳에서 수행한다.

장비·Node·gain·Config 부분 소유자는 실패 시 Reports에 남고 마지막 성공 시 다른 상태와 함께 이동한다. 실제 A-24H는 exit0, 24 hourly samples·390계열이며 기존 같은 입력 Rust result-store와 전체 bytes가 같다. World/CTF EIO 행은 도달한 기존 표면 경로의 증거이고, 새 Setup continuation 연결은 Source·기존 상태 검증으로 확인했다. Source313 이후 순회·ProcessSurfaceVertices·전체 Setup/HB 반환과 readiness, 수치 복사 동등성은 남아 있다. 앞선 DD 같은 입력 strict FAIL·whole-DD9FAIL/8UNPAIRED·네 gate·A/B 완료 수는 그대로다.

## DD 계산의 스레드 정밀도 진단

실제 Native에서 얻은 대표 입력95.18151092529297°C와15.5°C를 작은 C++/Rust 독립 실행으로 비교했다. 메인 스레드에서는 두 언어의40개 비교 가능한 중간값과 최종값이 일치했다. 같은 Rust 실행 파일의 새 스레드에서는 ln 결과·exp 인자는 그대로인데 exp 결과만 각1 ULP 커졌고, 기존 Cargo 재현의 첫 차이 값과 같았다. 두 스레드에서 읽은 x87 control word는 각각 `037f`·`027f`이며 MXCSR 초기값은 같았다.

독립 진단 프로세스 안에서 x87 precision-control 비트만 서로 교환하자 두 입력의 차이가 양방향으로 바뀌었다. 원래 control word로 복원하면 결과도 되돌아왔고, 정상 경로의 정확한 CW 복원·MXCSR 보존2건을 확인했다. 6개 문맥·278행을 전부 인증했으며, 메인/스레드 구분 실험90행과 읽기 전용 FP 관측92행도 별도로 보존했다. 비트8·9의 정밀도 제어 의미는 [Intel 설명](https://www.intel.com/content/dam/support/us/en/documents/processors/pentium4/sb/25366521.pdf)에 따른다.

이는 두 입력의 독립 실행에서 확인한 원인이다. 과거 실제 Native helper와 Cargo 테스트 프로세스의 FP 상태·정확한 수학 함수 호출 대상은 당시 작은 기록으로 묶이지 않았으므로 전체9,949개 차이의 원인이나 해결로 확대하지 않는다. 별도 진단용 unsafe assembly는 ignored Source에만 있으며 생산 수식·캐시 계산·workspace unsafe 금지 정책은 그대로다. 새 Cargo 테스트 본문·binary나 전체 workspace 반복은 없다. 네 번의 작은 실험에서 컴파일/실행 명령10개가 exit0이고, 공개 ZIP은 약190KB다. 기존 같은 입력 재현 FAIL·whole-DD9FAIL/8UNPAIRED·전체 raw/Iterate/대입 경로 UNKNOWN과 네 gate·A/B 완료 수는 유지한다.

## Setup 표면 순회·Process 기하 단위의 QA

최종 Source `093c0c1a4ee3f5ba5c71ac9c4d8670f1a31715617fb608eb61cf1eb230f66584`에서 fmt·source-quality·ep_runtime/ep_run all-target Clippy가 통과했다. 기존 runtime 본문1·일반 CLI1·실제 `CI_FORCE_TIME_STEP=Y` CLI1, **3 passed / 0 failed / 0 ignored**이며 새 테스트 본문·binary는 없다. 같은 최종 runtime 본문에서 실제 Native BEFORE 여섯 표면의 입력으로 180개 유한 scalar를 부호 있는 0까지 정확히 대조했다. Native AFTER 값은 생산 입력으로 공급하지 않는다. 작성된 소유자 fixture와 원본 입력 재현은 실제 World 생산자/endpoint 대조와 구분한다.

Source313–369는 shading counters·reserve/append·sky split·IsShadowing 대입, Process의 단회 scratch 할당·기존 처리 short circuit·plane/coplanarity/rectangle/변환·ShadeV 복사와 마지막 Zone5/Space2 reset을 보존한다. 실제 A-24H는 exit0·24 hourly samples·390계열이며 이전 같은 입력의 Rust result-store147,618바이트와 전체 bytes가 같다. Native의 96 Source timestep 실행이나 전체 Setup/HB 완료를 뜻하지 않는다. 최초 두 Rust101과 수정 전 Clippy101, configure 성공 후 adapter 검사 실패·복구는 당시 기록 그대로 보존했다. 전체 workspace 반복·장비 테스트 중복 실행은 없고 기존 수치 FAIL 및 모든 gate는 유지한다.

## 실제 World 생산자·보존 상태 대조와 추적 비용

최종 Source `38abf9324345f14e56ee93dbc2d580eb9a55758bfe2d257f9212863169ed1ab2`에서 Construction.Name을 원본 HeatBalanceManager1455 순서에 보존하고, 같은 reports·성공한 State의 Zone/Space 소유자를 읽기 전용으로 관찰한다. 실제 A CLI에서 생성한 여섯 표면의 이름·순서·소속, 전체 꼭짓점, GrossArea·CosTilt·local axes, Process 결과와 scratch/list를 기존 Native 관측에 직접 대조했다. **270개 유한 수치 비트·449개 이산 검사, 차이0·UNKNOWN0**이며 부호 있는 0을 보존한다. Native의 outer GetBuildingData 반환과 Rust의 성공한 실행 반환에서 살아 있는 선택 필드의 대조다. Native 최종 Zone/Space 면적·전체 Setup/HB·시간적 전체 동등성은 포함하지 않는다.

fmt·source-quality·두 crate all-target Clippy와 기존 일반 CLI1·실제 CI CLI1이 통과했다. 새 테스트 본문·binary·수학식·Native 빌드·kernel 재실행은 없다. A-24H는24표본·390계열이며 이전 Rust 결과147,618바이트와 같다. 기존 `surface` 모드에서도 기하 관찰을 제공하도록 두 조건만 확장했다. Full/Surface의 기하36,133바이트와 결과 전체 bytes가 같고, 기하 대조에 불필요한734,855,267바이트 기상 로그는 Surface에서 생성되지 않는다. 최초 Full 실행·당시 정적 검사·비교는 별도 Source 이력으로 보존했다. 그 로그의 삭제가 자동 검토에서 차단되어 사용자 요청대로 Downloads의 별도 폴더로 옮겼다. 기존 전체 수치 FAIL와 네 gate는 유지한다.

## Setup 같은 Zone/Space 면적 누적의 QA

최종 Source `56feb0493c4ee5c61e8a032a6ee8d61c5018c8526c532f463558d3905328e2dc`에서 fmt·source-quality·두 crate all-target Clippy와 기존 runtime1·일반 CLI1·실제 CI CLI1이 통과했다. 새 테스트 본문·binary 없이 같은 runtime 본문에 debug 입력의 무대입 실패와 창 분기의 부분 대입 보존 검사 두 개를 추가했다. 실제 A-24H는24표본·390계열이며 이전 결과147,618바이트와 전체 bytes가 같다. 같은 입력의 이전 Native 관측을 재사용해 새 면적 operand/endpoint20개·이산 검사15개를 확인했고, 기존 대조를 포함한 **290개 유한 수치 비트·464개 이산 검사에서 차이0·UNKNOWN0**이다. 독립 Source 검토와 최종 실제 readback도 통과했다.

Source370–454는 생성자에서0인 여덟 필드를 같은 Zone/Space에 보존하고, Surface.Area 합계→Roof/Floor GrossArea→Construction 분기→외기 면적·배수·Process.Width 둘레 순서를 유지한다. Native outer GetBuildingData 이후 살아 있는 선택 면적 값의 대조이며 내부 Setup checkpoint나 전체 함수 완료 증거는 아니다. Native Construction.TypeIsWindow 자체는 캡처되지 않았고 창·frame·지면·비단위 배수·양의 DETAILEDWWR_DEBUG는 미대조다. Rust 실패 경계 검사를 이 분기들의 Native 수치 검증으로 확대하지 않는다. 기존 높이/체적 helper 제한, 전체 수치 FAIL와 네 gate·A/B 완료 수는 그대로다. 새 Native 빌드·kernel 재실행·전체 workspace 반복은 없다.

## Setup 같은 Zone 높이·경고 순서의 QA

최종 Source `420462fa6348d9556b43369b814d3a08acad1b35ffa7a8671be1062d8b1a22d4`에서 fmt·source-quality·두 crate all-target Clippy와 기존 runtime1·일반 CLI1·실제 CI CLI1, **3 passed / 0 failed / 0 ignored**를 확인했다. 같은 runtime 본문에 자동 높이의 entered=false와 경고 stream 실패 시 entered=true·기존 높이·경고 counter의 보존 검사를 추가했다. 새 테스트 본문·binary·Native 빌드·kernel 재실행·전체 workspace 반복은 없다. 실제 A-24H는24표본·390계열이며 이전 결과147,618바이트와 전체 bytes가 같다. 기존 Process/면적 대조를 그대로 보존하고 실제 같은 Zone의 CeilingHeight와 ceilingHeightEntered를 추가해 **291개 유한 수치 비트·465개 이산 검사에서 차이0·UNKNOWN0**을 확인했다.

Source455–547는 Zone→spaceIndexes→Space.AllSurfaceFirst..Last 순서로 보존된 꼭짓점·GrossArea와 같은 geometric 면적을 기존 높이 식에 전달한다. 양의 입력 높이는 경고보다 먼저 입력 여부를 기록하고 실제 backend 오류에서도 그 대입을 유지한다. ErrCount는 생성자0이며 원본처럼 읽기만 한다. 상세 경고의 .2F는 .R 보정과 구분한다. 이후 선택 체적 handoff에서 높이 계산을 다시 호출하지 않아 자동 계산값이 사용자 입력으로 바뀌지 않는다. 기존 legacy wrapper의 수식·검증 순서는 보존했다.

대조 경계는 Native outer GetBuildingData와 Rust 성공 실행 후 살아 있는 선택 필드다. 내부 Setup547 checkpoint·Space 높이·550 이후 bounds·전체 CalculateZoneVolume/Setup/HB 증명은 포함하지 않는다. 입력 높이 경고·ErrCount·DisplayExtraWarnings·.2F 텍스트의 Native 변형은 미대조이며 Rust 실패 검사를 Native 경고 동등성으로 확대하지 않는다. 앞선 미대조 면적 분기와 전체 수치 FAIL·네 gate·A/B 완료 수는 그대로다.

## Setup 선택 닫힌 체적·Space 비율의 QA

최종 Source `e65f7ef83b067753ca9af08c293b278cd63162c7022dcfab444b988e12649eea`에서 fmt·source-quality·두 crate all-target Clippy와 기존 runtime1·backend1·일반 CLI1·실제 CI CLI1, **4 passed / 0 failed / 0 ignored**를 확인했다. 서로 다른 기존 본문은 세 개이며 CLI 본문을 두 설정에서 실행했다. 새 테스트 본문·binary·Native 빌드·전체 workspace 반복은 없다. 작성한 큐브 복사본으로 fresh Newell 계산·합계6, 입력 체적 우선순위, 디버그 출력 직전까지의 Zone/Space/비율 쓰기, 안내 출력 실패 이전 ErrCount5 증가를 검사했다. 기존 backend 본문에서는 일반 메시지의 SQLite -1/count0→Info1 콜백 순서, 빈 메시지와 counter 보존을 확인했다.

Source548의 선택 경로는 실제 Zone.AllSurfaceFirst..Last의 꼭짓점을 복사하고 Newell·VecLength→수평/수직 검사→벽 높이 검사→초기 edge 폐합→부호 있는 체적 계산 순서를 실행한다. 같은 Zone의 입력 체적/높이 우선순위와 Space 체적·비율을 기록한 뒤 기존 초기화 소비자에 계산 반환값을 이동한다. 뒤에서 높이·체적을 재계산하지 않는다. 여섯 사각형·일관된 면 방향·양의 부호 체적은 Rust의 한정된 admission이며 Native 폐합 조건을 추가 검증한 것으로 보지 않는다. Native 임시 배열의 할당·해제나 p0 전역 소유자 전체 대응도 이 반환 projection으로 증명하지 않는다.

실제 A-24H는24표본·390계열이며 이전 결과147,618바이트와 전체 bytes가 같다. Native outer GetBuildingData의 같은 Zone.Volume·Space.Volume·fracZoneVolume 세 수치와 ErrCount5를 추가 대조해 기존 검사를 포함한 **294개 유한 수치 비트·466개 이산 검사에서 차이0·UNKNOWN0**을 확인했다. Native p0·ShowZoneSurfaceHeaders는 기존 캡처에 있으나 Rust의 보존된 대응 소유자가 없어 대조하지 않는다. Native 폐합 반환, 양의 사용자 Zone/Space 체적·바닥 부재 경고·일반/상세 경고·양의 debug·polygon repair/open fallback·10m³ clamp와 내부 Setup548 checkpoint·전체 함수/Setup/HB는 미검증이다. 기존 수치 FAIL와 네 gate·A/B 완료 수는 그대로다.

최초 Source `323f7f453f6ff0ffb9e90fa3ba0b3bb36c9e9ee491b83cf0b0ca6c8e52baa3e8`의 정적 검사 세 개는 통과했으나 runtime101이었다. 작성한 큐브의 여섯 면이 모두 안쪽인데 수평 두 면만 뒤집어 한정된 폐합 admission에서 실패한 기록이다. 기존 본문의 복사본에서 여섯 Vertex 목록만 뒤집고 cached Newell은 유지해 실제 재계산을 검사했다. 다른 Source 15파일은 동일하며 최초 실패·수정 diff와 최종 QA를 함께 보존한다.

## Setup 중심점·Zone 경계·인접·표면 수의 QA

최종 Source `a5aeb18952a324cc6a9ea82060ce93d91c7e42ca663bb998af375bea434bf292`에서 fmt·source-quality·두 crate all-target Clippy와 기존 runtime1·일반 CLI1·실제 CI CLI1, **3 passed / 0 failed / 0 ignored**를 확인했다. 서로 다른 기존 본문은 두 개이며, 변경하지 않은 backend 테스트는 반복하지 않았다. 기존 runtime 본문에24줄을 추가해 중심점·경계값, 자기 Zone 인접, 반복 호출의 표면 수 누적과 잘못된 두 번째 인접 참조에서 새 배열·앞선 쓰기 보존을 검사했다. 새 테스트 본문·binary·Native 빌드·kernel 재실행·전체 workspace 반복은 없다.

Source550–639는 실제 Zone→spaceIndexes→Space.AllSurfaceFirst..Last 순서로 final Surface.Centroid·GrossArea·Vertex를 사용한다. 중심점 초기화와 가중 합·나눗셈, 조건부 첫 꼭짓점 경계 초기화, 원본 min/max의 동률·부호 있는0 처리, IntMass skip과 severe 이전/이후 상태 순서를 유지한다. 별도 SurfAdjacentZone을 실제 TotSurfaces 크기의0 배열로 초기화한 뒤 양의 ExtBoundCond가 가리키는 표면의 Zone을 기록한다. shading 이름 비교가 숫자 Zone 불일치 skip보다 먼저이며, 세 표면 수는 원본처럼 초기화하지 않고 증가한다. 선택 World 경로의 뒤쪽 중심점 재계산을 제거했고 기존 legacy 계산은 유지했다.

실제 A-24H는24표본·390계열이며 이전 결과147,618바이트와 전체 bytes가 같다. 원본에서 보존한 여섯 Surface.Centroid18개와 Zone 경계6개를 새로 대조했다. 기존294개 수치·466개 이산 검사와 새24개 수치·배열 길이21개를 합친 **318개 유한 수치 비트·487개 이산 검사에서 차이0·UNKNOWN0**이며, 앞선 모든 비교 조건도 유지한다. Native outer GetBuildingData와 Rust 성공 실행 후 보존된 값의 대조로, 내부 Setup639 checkpoint는 아니다. Native Zone.Centroid·인접 배열·세 표면 수는 기존 캡처에 없어 미대조이며 Rust fixture 검사로 대신 인증하지 않는다. 내부 질량만 있는 오류·air boundary 등 선택 밖 분기,641 이후 NominalU/보고·전체 Setup/HB와 기존 전체 수치 FAIL·네 gate·A/B 완료 수는 그대로다.

## Material·Construction NominalR/U 소유자의 QA

최종 Source `6de3ad2dc7932f107637c4b7a3c0f61c37bfb23c725bd6af7fc6c1bfa8f6d427`에서 fmt·source-quality·ep_runtime all-target Clippy와 기존 runtime1·일반 CLI1·실제 CI CLI1, **3 passed / 0 failed / 0 ignored**를 확인했다. 변경하지 않은 ep_run의 Clippy와 backend 테스트는 반복하지 않았다. 기존 runtime 본문에50줄을 추가해 작성한 NoMass 상류 소유자에서 실제 Construction getter가 반복된 두 층을 순서대로 더하고 역수를 기록하는지, 네 배열 초기값과 0 저항 severe 출력 실패 이전 상태를 보존하는지 검사했다. 새 테스트 본문·binary·Native 빌드·전체 workspace 반복은 없다.

Material.cc219–220/308의 실제 raw 두께·전도율 나눗셈과 NoMass 인자 복사를 MaterialNum에 맞는 NominalR에 기록한다. 물리 재료 속성을 새 cache로 재구성하지 않는다. HeatBalanceManager1393–1396은 같은 HB 소유자의 R/U/U-before 배열을+0, 보정 비율을+1로 할당한다.1504의 실제 Material.NominalR 합산은 층 순서를 유지하며 NoRegularMaterialsUsed 쓰기보다 먼저다.1779–1795의 비영 저항 역수와 0 저항 severe→ErrorsFound→기존 선택 Type* 대입 순서를 보존한다. 오류 플래그는 GetHeatBalanceInput에서 재료·Construction·GetBuildingData가 공유하는 실제 소유자다. 전체 MaterialBase·CheckAndSetConstructionProperties나641 보고 완료를 뜻하지 않는다.

실제 A-24H는24표본·390계열이며 기하 추적39,475바이트와 결과147,618바이트가 직전 bounds 단위와 전체 bytes 동일하다. 기존 **318개 유한 수치 비트·487개 이산 검사에서 차이0·UNKNOWN0**인 것은 기하 회귀 증거다. 새 Material.NominalR·Construction.NominalU와 새로운 오류 분기의 Native 직접 대조는 **UNPAIRED**이며 이를 새 수치 PASS로 확대하지 않는다. 전체 수치 FAIL·네 gate·A/B 완료 수는 그대로다.

최초 Clippy는 테스트가 private 하위 모듈을 import해 E0603·exit101이었다. 기존 crate-visible 재노출 경로로 import 한 곳만 수정했으며 네 생산 파일의 bytes는 동일하다. 최초 fmt/quality 통과·실패 Source와 수정 diff를 최종 QA와 구분해 보존한다. Native 캡처 부재에 대한 앞선 표현은 selected_geometry의 원시 Zone 중심점·표면 수·인접 필드에 한정한다. 같은 캡처의 실제 EIO 문자열에는 반올림한 Zone 중심점과 정수 표면 수가 이미 있다. 이를 원시 binary64로 역산하지 않으며, 후속 보고 단계에서 실제 문자열 자체를 대조할 수 있다.

## Setup 실제 순서·저장 U·EIO 보고의 QA

최종 Source `f2e29a5cfe28eb34514e7016be54aa91a78a69b6d9791264de1454ca9da1004b`에서 fmt·source-quality·ep_runtime all-target Clippy와 기존 runtime1·일반 CLI1·실제 CI CLI1, **3 passed / 0 failed / 0 ignored**를 확인했다. 기존 본문에40줄을 추가해 실제 보고 ID/이름 순서, Wall의 인접 Floor film 선택, 여섯 EIO 행과 첫 헤더 다음 출력 실패에서 표 항목·헤더 보존을 검사했다. Construction0은 OSC 계산을 호출하지 않고 원본처럼 기본+0을 표에 남긴다. 새 테스트 본문·binary·Native 빌드·전체 workspace 반복은 없다.

Source641–792는 같은 all_surface_list_report_order에서 final Surface와 실제 NominalU를 읽는다. 외부 film 선택→양의 U 분기→`1/(inside + (1/U) + outside)`의 순서와 결합을 보존하며, inside0일 때 outside도0으로 만들고 역수 왕복을 생략하지 않는다. 실제 predefined 열·표 저장소에 기록한 뒤 같은 shading counters·global 개수·Zone 중심점/경계/면적/표면 수와 알고리즘 enum으로 원본 EIO 세 구간을 출력한다. 기존 `.R` formatter를 재사용하고 선택 World 경로의 뒤쪽 typed 보고 재계산을 제거했다. legacy 경로는 유지하며, 표의 기존 유한값·절댓값1e8 미만 범위를 전체 포맷 지원으로 확대하지 않는다.

실제 A-24H는24표본·390계열이며 기하 추적39,475바이트와 결과147,618바이트가 이전 Nominal 단위와 전체 bytes 동일하다. 기존 **318개 유한 수치 비트·487개 이산 검사에서 차이0·UNKNOWN0**을 유지했다. 같은 Native 캡처의 실제 EIO 파일5,595바이트와 보존 문자열이 일치하는지 확인하고, Shading Summary·Zone Summary·Zone Information의 header3/row3, **1,115바이트**를 Rust 실제 파일과 공백·행 끝까지 정확히 대조했다. Rust EIO8,761바이트에서 이 한 구간만 제거하면 이전7,646바이트와 전체 bytes가 같다.

이는 Native outer GetBuildingData와 Rust 성공 실행에서 살아 있는 선택 출력 문자열의 대조다. 반올림된 Zone 중심점 문자열을 원시 binary64로 역산하지 않는다. raw Material.NominalR·Construction.NominalU·film 표 값과 raw Zone 중심점/인접/표면 수는 미대조다. 실제 후속 호출은 Solar enclosure797, Zone 외기 설정800·검사801이며 이번792행 경계 밖이다. 전체 Setup/HB·엔진 수치 FAIL와 네 gate·A/B 완료 수는 그대로다.

## Setup 같은 Solar 저장소·Zone 온도 설정의 QA

최종 Source `164b014df8a5275980d563fbcbe0f16dbeb794a75678ae65bb5de7c66bec50f2`에서 fmt·source-quality·ep_runtime all-target Clippy와 기존 runtime1·일반 CLI1·실제 CI CLI1, **3 passed / 0 failed / 0 ignored**를 확인했다. 서로 다른 기존 본문은 두 개이며 새 테스트 본문·binary·backend 재실행·Native 빌드는 없다. 기존 runtime 본문의 diff는53줄 추가·4줄 제거로, 테스트 개수 증가를 뜻하지 않는다. 같은 Solar 배열 유지·반복 MRT 등록과 등록 실패 이전의 할당·Space 쓰기 보존, 온도 설정·오류 뒤에도 바람 값이 유지되는지를 검사했다. 최초 fmt/quality 통과와 미사용 `model` 인자의 Clippy exit101은 별도로 보존했고, `_model` 이름 수정 후 최종 정적 검사를 다시 통과했다.

선택797은 앞선 radiant 초기화와 같은 저장소에 Solar 배열·Space 연결을 기록하고 공통 MRT 출력 등록을 실제 순서대로 다시 호출한다. 뒤의 수치 연결은 이 배열을 채우며 새로 만들지 않는다.800/801은 같은 live Station/Site·현재 기상 상태와 실제 Zone 중심점을 사용해 모든 Zone 온도를 먼저 쓰고 누적 최솟값 검사·severe/continue/fatal 처리를 수행한다. 이 온도 전용 호출은 바람·Surface 외기 값을 변경하지 않는다. 현재 Rust 기상은 초기화 전에 이미 진행된 상태이며 Native 생성자의0으로 되돌리거나 새 기상 상태로 대신하지 않는다.

실제 A-24H는24표본·390계열이고, 기하39,475바이트·결과147,618바이트·EIO8,761바이트가 직전 보고 단위와 전체 bytes 동일하다. 기존 **318개 유한 수치 비트·487개 이산 검사와 EIO header3/row3의1,115바이트** 대조에서 차이0·UNKNOWN0을 유지했다. 이는 기존 회귀이며 새 Solar 저장소·연결이나800/801의 Native 온도 입력·출력 수치 대조가 아니다. 해당 원시 상태는 **UNPAIRED**이고, 전체 transitive Setup/HB·엔진 완료, 기존 수치 FAIL·네 gate·A/B 완료 수는 그대로다.

## GetBuildingData 이후 HB 입력·불투명 Construction 속성의 QA

최종 Source `ba09d3ca12630f4bcd0c7d7438a857d3bed8716722f0cb62e25ade27de6a20f0`에서 fmt·source-quality·ep_runtime/ep_run all-target Clippy와 기존 runtime1·일반 CLI1·실제 CI CLI1, 최종 **3 passed / 0 failed / 0 ignored**를 확인했다. 서로 다른 기존 본문은 두 개이고 최종 diff는60줄 추가·1줄 제거다. 최초 정적 검사 세 개는 통과했으나 runtime01은 작성한 AbortProcessing 기대값 때문에 실패했다. 실제 ShowFatalError는 이 flag를 설정하지 않아 테스트만 ErrorsFound=true·AbortProcessing=false로 수정했고, 생산8파일의 bytes는 그대로다. 전체 테스트 시도는4회·과거 실패1회이며 최종 정적02/runtime02와 실패 기록을 구분해 보존한다. 새 본문·binary·backend 재실행·Native 빌드는 없다.

선택 GetBuildingData 성공 반환 뒤 HBM278–313을 같은 입력 처리기·shortcut·Surface 개수·Construction 사용 flag·ErrorsFound에 연결했다. 객체0개에서도 SolarIncidentInside의 실제4-alpha 정의 조회와 null 삽입을 수행하고, module/count 쓰기와 severe→ErrorsFound·warning/continue/ShowMessage·최종 fatal의 실제 순서 및 실패 이전 상태를 유지한다. 앞선 GetConstructData1795의 불투명 CheckAndSet은 DataHeatBalance341–423/753–798 순서로 생성자0/roughness Invalid와 early return을 보존하고, 실제 MaterialNum으로 연결된 단일 TypedMaterial을 빌려 안팎 visible/solar/thermal 흡수율·반사율·roughness를 같은 Construction에 기록한다. 새 속성 캐시는 만들지 않았다. 실제 InitSolar316은 미연결이며 기존 후행 Solar 수치 초기화로 원본 caller 완료를 주장하지 않는다.

실제 A-24H는24표본·390계열이고 기하39,475바이트·결과147,618바이트·EIO8,761바이트가 직전 Solar 단위와 전체 bytes 동일하다. 기존 **318개 유한 수치 비트·487개 이산 검사와 EIO header3/row3의1,115바이트** 대조에서 차이0·UNKNOWN0을 유지했다. 이 대조는 기존 회귀이며 새 입력 개수·진단·Construction 속성의 Native 원시 상태는 **UNPAIRED**다. 양의 window/부속 객체·경고 분기와 전체 GetHeatBalanceInput·transitive Setup/HB, 기존 수치 FAIL·네 gate·A/B 완료 수는 그대로 남는다.

## Solar·재료·Construction 실제 상태 직접 대조

읽기 전용 관측 확장5파일은 같은 Solar·Material·Construction·HeatBalance nominal 소유자를 복사한다. 계산·초기화·생성자·테스트 파일은 변경하지 않았다. 새 원본 실행은 실제13개 입력 호출과 GetBuildingData 반환 뒤의 관측을 완료했고, 실제15개 Original 함수 연결·선택 실행 바이트와 정적 PE/DLL 검색 구성을 확인했다. 원본 함수명 계약의 오기는 기존01을 보존하고 capture 전용02에서 실제 `SetupEnclosuresAndAirBoundaries`로 바로잡았다. 이는 실제 Native loaded-module trace나 전체 OS·FP 환경 인증은 아니다.

같은 A 입력의 재료3·Construction3·Solar enclosure1을 이름/map으로 일대일 연결한 뒤 실제 Source 번호·배열 범위·전체11개 LayerPoint·Space/Solar 소속을 엄격 비교했다. 재료의 NominalR·검증된 흡수율, Construction의 보존 속성, 네 nominal 배열 및 Solar 면적의 **51개 유한 binary64 값과321개 이산 검사에서 차이0·UNKNOWN0**이다. 비트와 부호 있는0을 그대로 비교했으며 tolerance·재계산·예상값 주입은 없다. 기존318개 수치·487개 이산 검사와 EIO header3/row3의1,115바이트 대조도 통과했다.

Rust 관측 QA는 Source `4a2298a33fe3db81842cbc0ebae7633b0c3e0e228232dbbe7417c83a3956fc20`에서 fmt·source-quality·두 crate Clippy, 기존 CLI 테스트1개, A24 surface 관측, 기존 comparator07의6개 명령이다. runtime·CI·backend·전체 workspace 테스트는 반복하지 않았다. 실제 A는24표본·390계열이고 결과147,618바이트·EIO8,761바이트가 직전 단위와 전체 bytes 동일하다. Rust 실행HEAD `0a47d231`와 이후 Native 실행HEAD `b8dd341b`는 같은 SourceSnapshot·승인된5파일·실제 Git bytes로 연결했으며 서로 같은 실행 커밋으로 기록하지 않았다.

별도 Native Material.Name/Resistance/Thickness/Input 필드·포인터 할당 동등성, fixed LayerPoint의 할당 flag 동등성, 후행 Solar F/ScriptF 및 실제800 온도 호출은 **UNPAIRED**다. 실제 InitSolar316 연결과 전체 GetHeatBalanceInput·Setup/HB·engine 완료는 남아 있다. 과거 수치 FAIL·네 gate·A/B 완료 수는 그대로이며, 위44개 actual 요약과1,572,346바이트 증거 ZIP에 현재 범위를 보존한다.

## 실제 InitSolar316 연결과 보존 배열 직접 대조

`GetHeatBalanceInput`의 원본316 위치에서 같은 Solar 저장소를 초기화한다. 배열 할당·Source SurfacePtr·보고 순서·Surface 역참조를 원본 순서로 기록하고, 뒤의 물리 연결은 이 수치와 할당을 유지한다. 기존 수치 helper와 legacy 초기화는 그대로다. NumSolar0의 무할당 접근, 반복 호출의 보고 항목 누적과 실제 F 할당 보존도 기존 런타임 본문에서 검사했다. 양의 사용자 view factor·전체 상세 보고·zero-surface 배열·진단/오류 보정 분기는 선택 범위 밖이며, Rust 검사로 Native 실패 동등성을 대신하지 않는다.

동결 Source `63d258fe13817aa66689af6165e4f098d85707624b81a1817c13e35aaaca3831`에서 fmt·source-quality·두 crate all-target Clippy와 기존 runtime1·일반 CLI1·실제 CI CLI1, 최종3 passed/0 failed/0 ignored를 확인했다. 기존 두 본문의3회 실행이며 새 본문·binary·backend·전체 workspace 반복은 없다. 런타임 본문에는42줄만 추가했다. 실제 A24는24표본·390계열이고 결과147,618바이트는 이전과 전체 bytes가 같다. EIO8,761바이트는 동일한 Solar 두 행만 실제 초기 호출 위치로 이동했으며, 두 행을 제거한 나머지는 전체 bytes가 같다. 기존318개 수치·487개 이산 검사와 EIO1,115바이트 대조도 유지했다.

새 Native 실행은 첫8개 실제 loader와 `GetHeatBalanceInput` 부모 호출을 완료했다. Solar 계산 커널이나 다섯 getter를 뒤에 덧붙이지 않았다. 실제22개 원본 함수 연결·선택 코드와 정적 PE/DLL 검색을 확인했으며, loaded-module trace·전체 OS/FP 인증을 뜻하지 않는다. 부모 반환 뒤 보존된 F36개·Area/SolAbsorptance/Azimuth/Tilt24개, 기존 재료·Construction·nominal51개를 원시 Source 번호·완전한 배열 범위·SurfacePtr/ReportNums·세 역참조와 연결했다. **111개 유한 binary64 값·676개 이산 검사에서 차이0·UNKNOWN0**이다. 부호 있는0도 그대로 비교하며 허용오차·재계산·예상값 주입은 없다.

Rust QA 실행HEAD `b042d88c`와 이후 Native 실행HEAD `add087dc`는 실제7파일 Git bytes와 같은 SourceSnapshot으로 연결했으며 같은 실행 커밋으로 기록하지 않았다. 대조 경계는 후속 내부발열 입력까지 포함한 Native 부모 반환과 Rust의 살아 있는 선택 상태다. 내부316의 정확한 checkpoint·parent local ErrorsFound·전체 GetHeatBalanceInput 완료를 인증하지 않는다. HB320 입력, ScriptF/후속 장파 복사, 실제800 기상 상태, 양의 입력·경고·fatal·보정 분기 및 전체 Setup/HB/engine은 남아 있다. 네 gate·A14/54·B14/64와 기존 전체 수치 FAIL는 그대로다. 위45개 actual 요약과1,480,963바이트 ZIP에 증거를 보존한다.

## 실제 HB320 내부발열 입력과 보존 상태 직접 대조

같은 입력 처리기·Zone/Space·스케줄·출력 등록 소유자를 사용하는 `ManageInternalHeatGains(initOnly=true)`를 원본320 위치에 연결했다. `OtherEquipment`의 `None` 연료·`EquipmentLevel`, 실제 선언 순서·필드 수·최소11필드, 배열 할당·장비 연결·보고 등록·nominal EIO와 반복 입력 생략을 구현했다. 양의 미지원 장비·목록·EMS·출력 요청·Kiva는 해당 원본 연산 전에 unavailable을 반환한다. 기존 hourly 소비자는 유지하며 실제 시간별 Source 발열 생산자는 별도 작업이다.

수정 후 Source `0da66735c54d87232d53a99fedfcb7c4d2e43ebe9127692e2befb87646246104`에서 fmt·품질·ep_run all-target Clippy, 기존 CLI 본문의 일반·실제 CI 설정 실행이 모두 통과했다. raw-model·runtime 두 검사는 이전 Source `6c765686`의 성공 결과를 재사용했다. 전체4,067행 소스 색인을 대조해 유일한 변경이 두 패키지 밖 ep_run 메타데이터 허용 목록의31바이트 한 줄임을 확인했다. 새 테스트 본문·binary·전체 workspace 실행은 없다. 최초 Clippy101과 실제 A24 exit6 두 건, 각 수정·소스 검토는 별도 보존했다.

깨끗한 Rust 실행 커밋 `bb100668`의 A24는24표본·390계열을 생성했고, 직전 결과147,618바이트와 전체 bytes가 같다. EIO에는 원본 nominal 발열 구간1,576바이트·두 header·세 row만 추가됐다. 원본은 `a8dc04fe`·Source `6c765686`에서 첫8개 실제 loader와 GetHeatBalanceInput 부모 호출을 실행했다. 두 문맥을 같은 Source/커밋으로 적지 않고 위 한 줄의 전체 색인 bridge로 연결했다. 새226개 유한 binary64 값·867개 이산 검사와 nominal EIO가 일치했고, 기존 Solar를 합친337개 수치·1,543개 이산 검사에 차이0·UNKNOWN0이다. 기존318/487·EIO1,115바이트 회귀와 독립 전체 readback도 통과했다.

장비의 실제 필드 연결, 활성 device prefix·capacity·공유zero 대상, 보고 배열·스케줄과 선택 DD 순서/속성을 대조했다. 전역 DD 절대번호·Next/keyed OutVar·보존되지 않은60회 호출 이력, Native-only 필드, 양의 미지원 입력·경고/fatal 및 시간별 발열·ScriptF/후속 장파복사·실제800 기상·전체 Setup/HB/engine은 UNPAIRED 또는 미완료다. 네 gate·A14/54·B14/64와 기존 전체 수치 FAIL는 그대로다. 현재46개 actual 요약에 이 선택 단위의 증거를 보존했다. ZIP554개 항목 전체의 CRC·SHA·bytes 검증과 발행 명령의 커밋 인자 오류·성공 재실행은 [별도 발행 기록](../evidence/SRC-06-ordinary-hb-internal-gains-input/publication-readback.json)에 남겼다.

## 이력과 증거 보존

반복된 착수 전 산문과 당시 미완료 목록은 현재 범위와 위 표로 통합했다. 원본 범위·적용/formatter diff·실패/중단 명령·실제 QA·선택 입력과 uncapped ledger는 기존 요약/색인/ZIP에 그대로 남긴다. 초기 입력/transport/compile/Clippy 실패를 나중 성공 로그로 바꾸지 않는다. 과거 global file-size 실패와 구조적 부채도 현재 통과로 다시 쓰지 않는다.

대형 Native 실행 파일·core·map·provider와 whole Source는 각 단위의 기존 provenance 범위다. 이 문서 정리는 payload 재검증이나 transitive build/replay 보장을 추가하지 않는다. Native endpoint는 생산 입력으로 주입하지 않으며, 실제 BEFORE 값을 쓰는 bounded replay는 테스트 범위로만 해석한다. 모든 gate와 A/B 완료 수는 그대로다.


## 제안 호출 연결의 전체 대상 컴파일 체크포인트


[날씨·실제 호출 연결 검증 체크포인트](../evidence/SRC-06-provider-integration-checkpoint-07/summary.json)에 검증한 제안 코드와 재현 ZIP을 보존했습니다. 같은 세션의 GetNext·설계일 생성·일별 판독·보간·스케줄·강수량·외부 에너지 사용·열수지 호출을 연결했고, 실제 준비 루프와 본 실행 루프의 원본 순서를 추가했습니다. 새 빌드 디렉터리에서 runtime·run·CLI 및 기존 테스트 타깃 전체 컴파일을 통과했습니다. 기존 판독기 9개와 설계일 준비 과정 1개 테스트가 통과했으며 새 테스트는 추가하지 않았습니다. 수명·문자열 타입 오류와 수정 기록도 보존했습니다. 전체 실행 파이프라인 활성화, 준비 과정 후처리·본 실행의 실제 호출 연결 및 새로운 동일 입력 Native A/B 검증은 남아 있습니다. 네 gate·A14/54·B14/64와 기존 수치 FAIL은 유지합니다.


[저장소 Source 적용과 실제 검증 기록](../evidence/SRC-06-canonical-provider-integration-01/summary.json). 검토한 일반 실행 Source 417개 파일(신규 325개)을 저장소 본체에 적용했습니다. 동일 세션의 날씨·준비 과정·열수지·본 실행 부모와 실제 시작 단계 child, 진단 상태 참조를 포함합니다. 기존 전체 워크스페이스·타깃 테스트 4,985개가 통과했고, 실패는 0개이며 기존 ignored 2개는 그대로입니다. 새 테스트는 추가하지 않았습니다. 기존 테스트 입력의 빠진 재료 선언, 중복 Site:Location 스키마, 상태 객체의 스택 보관 문제 및 일반 실행 출력 목록 두 곳을 수정했습니다. 기존 표면 테스트 준비값에는 실제 면적 필드 세 곳을 보완했습니다. 기존 전체 출력 집합과 구역 상태 비교는 유지했습니다. 실패·진단 기록과 각 수정의 범위를 별도로 보존했습니다. 기본 실행 파이프라인 전환, 전체 종료 후처리, 남은 실제 호출 연결과 동일 입력 Native A/B 검증은 미완료입니다. 네 gate·A14/54·B14/64 및 기존 수치 FAIL은 유지합니다. 작은 수정은 영향을 받는 기존 테스트로 확인하고, 이번 전체 Source 적용처럼 범위가 큰 통합 시점에 전체 테스트를 수행합니다.


[보고서 및 종료 함수 Source 적용](../evidence/SRC-06-canonical-provider-integration-02/summary.json). 연결 보고서 3종과 RDD/MDD를 실제 pre-primary host에 연결하고, 같은 날씨·세션·원래 지역 상태를 사용하는 kickoff부터 primary까지의 호출 함수를 적용했습니다. 종료 단계의 작은 함수, 실제 요금 계산 진입 순서와 상태 필드도 포함합니다. 전체 타깃 컴파일 및 기존 실행 테스트 27개가 원래 8MiB 스택에서 통과했습니다. 새 테스트를 추가하지 않았고 전체 테스트는 반복하지 않았습니다. 실제 기본 파이프라인 전환·전체 종료 처리·Native 수치 비교는 미완료이며 A14/54·B14/64와 기존 FAIL을 유지합니다. 별도 실험용 파이프라인과 이후 종료 prefix는 이 적용에 포함하지 않습니다.


[날씨 입력 메타데이터 수정과 실제 실행 기록](../evidence/SRC-06-canonical-weather-idf-metadata-03/summary.json). Site:Location·RunPeriod·SizingPeriod:DesignDay의 실제 IDF 필드 수 전달을 수정했습니다. 선택 필드의 전체 용량으로 입력 개수를 대체하지 않으며, RunPeriod와 DesignDay의 세미콜론 없는 EOF 입력은 정확한 메타데이터를 보장할 수 없어 명시적으로 미지원 처리합니다. 기존 raw-model 테스트 16개와 기본 실행 경로 A-24H 회귀 실행이 통과했습니다. 새 테스트 본문과 전체 테스트 반복은 없습니다. 별도 실제 Source 경로는 세 입력 오류를 넘고 20일 워밍업 후 본 기상 실행에 진입했으며, 첫 열적 복원력 누적 보고에서 미지원으로 종료합니다. 실제 Source 24시간 완료·기본 파이프라인 전환·Native 수치 동등성을 뜻하지 않습니다. A14/54·B14/64와 기존 전체 FAIL을 유지합니다.


[열적 복원력 계산과 실제 본 실행·원본 출력 대조](../evidence/SRC-06-canonical-thermal-weather-04/summary.json). 실제 구역 상태·온도·냉난방 설정점을 사용하는 열적 복원력 기상 누적 계산을 적용했습니다. 같은 재실 상태를 보존하고 원본 분기·연산·갱신 순서를 유지합니다. 변경 Source를 포함한 진단 조합의 전체 타깃 컴파일이 통과했고 새 테스트나 전체 테스트 반복은 없습니다. 실제 Source A-24H는 20일 워밍업과 15분 기록 96개·시간별 기록 24개를 생성한 뒤 Primary에서 반환했습니다. 전체 실행은 종료 보고·결과 내보내기 경계에서 exit6이며 기본 파이프라인은 아직 전환하지 않았습니다. 같은 입력의 원본26.1.0 실행은 성공했고 출력 사전·시간축은 일치합니다. ESO 5,280개 값 중 3,536개는 보고된 십진수를 binary64로 해석했을 때 다르며 최대 절대 차이는 7.429434845107608e-11입니다. MTR 72개 보고 값은 같습니다. 내부 원시 상태·FP 환경 동등성을 뜻하지 않으며 허용오차를 추가하지 않았습니다. 원인 검토와 수치 동등성 검증은 남아 있습니다. A14/54·B14/64와 기존 전체 FAIL을 유지합니다.


[입력 통계와 실제 표 파일 생성 검증](../evidence/SRC-06-canonical-input-statistics-tabular-05/summary.json). 입력 원시 필드와 스키마 메타데이터로 원본 입력 통계 7개를 계산하도록 연결했습니다. 실제 A-24H 통계 7개는 동일 입력의 Native 출력과 일치합니다. 종료 보고 선행 단계와 실제 표 파일 열기를 반영했으며, 진단 실행은 CSV·TAB·TXT·HTML·XML의 초기 헤더를 생성한 뒤 596번 날씨 통계 보고 진입 전 미구현 경계에서 exit6으로 끝납니다. 강우 보고는 원본의 날씨·나머지 보고 순서를 기다리므로 아직 실행되지 않았습니다. 기본 파이프라인은 입력 통계 초기화 호출만 갱신했고 진단 경로는 전환하지 않았습니다. 관련 기존 테스트 27개는 원래 8MiB 설정에서 통과했으며 새 테스트와 전체 테스트 반복은 없습니다. ESO·MTR·EIO는 직전 진단과 바이트까지 같아 기존 수치 차이가 유지됩니다. 전체 실행·수치 동등성·카드 완료를 뜻하지 않으며 A14/54·B14/64와 기존 전체 FAIL은 유지합니다.


[날씨·잔여·강우 보고 단계 검증](../evidence/SRC-06-canonical-weather-remaining-reports-06/summary.json). 날씨 STAT 원시 바이트, 실제 보고 레지스트리와 동일 세션의 상태를 이용해 날씨·잔여·강우 보고 및 출력 형식 선택을 연결했습니다. 실제 진단 실행은 5212·5213·5214 순서를 거쳐 연간 에너지 요약표 WriteBEPSTable5230 진입 전 미구현 경계에서 exit6으로 끝납니다. 새로 도달한 표 항목 219개 중 218개는 동일 입력 Native 최종 보고서와 출력 바이트가 같고, 1개는 Native의 뒤 단계 덮어쓰기 전 자리표시자이므로 단계가 다른 미짝 항목입니다. 관련 기존 테스트 19개와 기본 실행 기존 테스트 27개가 원래 8MiB 설정에서 통과했으며 새 테스트와 전체 테스트 반복은 없습니다. 기본 파이프라인과 읽기 전용 진단 관측은 전환하지 않았습니다. ESO·MTR·EIO는 직전과 바이트까지 같아 기존 ESO 수치 차이가 남습니다. 전체 실행·수치 동등성·카드 완료를 뜻하지 않으며 A14/54·B14/64와 기존 전체 FAIL을 유지합니다.


[BEPS 보고 핵심 단계 검증](../evidence/SRC-06-canonical-beps-report-core-07/summary.json). BEPS 보고 핵심 Rust 파일 17개(신규 12개·기존 5개)를 같은 실제 상태 소유자와 연결했습니다. 실제 격리 Source72 실행은 BEPS 표 13개를 작성한 뒤 목차 호출 5231의 미구현 경계에서 exit6으로 종료됩니다. 같은 입력의 Native 최종 보고서와 다섯 출력 형식의 앞부분을 비교해 원시 바이트 3,007줄이 같고, 버전·실제 시각 차이 9줄은 원문 그대로 별도 보존했습니다. LEED predefined 항목은 505개에서 726개로 늘었습니다. canonical all-targets 검사를 통과했으며 새 테스트나 전체 테스트 반복은 없습니다. 이전 27개 테스트 결과는 이전 checkpoint의 검증 범위로만 유지합니다. 진단 경로 9개·읽기 전용 snapshot·기본 pipeline은 이번 적용에서 제외했습니다. 기존 ESO 수치 차이와 A14/54·B14/64 및 전체 FAIL은 유지하며, 전체 실행·수치 동등성·카드 완료를 뜻하지 않습니다.


[Tabular 보고서 핵심 단계 검증](../evidence/SRC-06-canonical-tabular-report-core-08/summary.json). 실제 TOC·Verification Summary·Demand·Source Energy 보고서 경로의 Rust 파일 20개(신규 10개, 기존 10개)를 같은 상태 소유자에 연결했습니다. 격리 Source75 실행은 이름 있는 표 25개를 작성한 뒤 실제 Component Sizing 진입 5235에서 exit6으로 중단됩니다. 같은 입력의 Native 최종 보고서와 다섯 출력 형식의 앞부분 6,680줄을 비교해 6,663줄이 바이트 단위로 같았으며, 버전·실제 시각 차이 17줄은 원문과 전체 hex로 보존했습니다. Predefined 항목은 732개에서 914개로 늘었고, 실제 할당 공간 1,600개 전체의 변경 내역도 보존했습니다. 첫 비교 시도의 시각 분류·할당 꼬리 관찰 오류는 미승인 이력으로 남기고 수정된 비교만 승인했습니다. Canonical all-targets 검사는 통과했으며 새 테스트나 전체 테스트 반복은 없었습니다. 이전 27개 테스트 증거는 이전 검증 범위로만 유지합니다. 진단 경로 9개·읽기 전용 snapshot·기본 pipeline·미컴파일 후속 네 보고서는 이번 적용에서 제외했습니다. 기존 ESO 수치 차이와 원인 미확정 상태, A14/54·B14/64 및 전체 FAIL은 유지하며 전체 실행·수치 동등성·내보내기 완료를 뜻하지 않습니다.


[후속 요약 보고 단계 검증](../evidence/SRC-06-canonical-post-summary-core-09/summary.json). 크기 산정·음영·비용·적응 쾌적 보고, 초기화 요약·부하 보고 조건·열배출 보고의 Rust 파일 7개(신규 3개·기존 4개)를 반영했습니다. 실제 격리 Source77 실행은 이름 있는 표 75개를 작성한 뒤 열적 복원력 보고 5244 진입에서 exit6으로 끝납니다. 같은 입력 Native와 다섯 출력 형식의 앞부분 9,620줄을 대조해 9,588줄이 바이트까지 같으며, 버전 10줄·실제 시각 11줄·실행 날짜 11줄의 차이는 전체 원문 hex로 보존했습니다. Source 실행일은 10월 19일, 보존한 Native 실행일은 10월 17일입니다. EIO는 같은 열린 파일의 위치를 복원하며 읽습니다. 본체에는 그 메서드 2,186바이트만 추가하고 기존 진단 관측 변경 1,164바이트는 제외했습니다. Predefined 914개 항목과 ESO·MTR·EIO·원시 진단 출력은 직전 실행과 같습니다. Canonical 전체 타깃 컴파일을 통과했고 새 테스트나 전체 테스트 반복은 없습니다. 기본 pipeline과 진단 관측은 전환하지 않았습니다. 기존 ESO 수치 차이·원인 미확정 상태, A14/54·B14/64와 전체 FAIL은 유지하며 전체 실행·수치 동등성·내보내기 완료를 뜻하지 않습니다.


[실제 endrun 종료 경로 검증](../evidence/SRC-06-canonical-completed-endrun-core-10/summary.json). 유한 A-24H 포팅의 복원력·주기·코일·사전 정의 보고서와 감사·파일 닫기·endrun 620까지 실제 Source 경로를 반영했습니다. 실제 격리 Source79는 이름 있는 표 191개를 작성했으며, 다섯 물리 출력 형식 전체 19,312줄 중 19,242줄이 Native와 바이트까지 같고 나머지 70줄은 버전·시계·날짜 메타데이터 차이로 원문을 보존했습니다. 초기 조명 입력 처리의 누락된 합계 세 항목은 실제 Native 생산자 순서로 복원했고, 같은 출력 스트림의 Windows 텍스트 모드를 적용했습니다. 실제 ESO 5,280개·MTR 72개 수치 레코드의 종료 기록과 감사·SHD 파일을 확인했습니다. Canonical Rust 파일 31개(신규 14개·기존 17개), 전체 컴파일 입력 4,467개를 검증했으며 기존 테스트 하나의 물리 줄바꿈 기대값 두 곳만 수정했습니다. 기본 pipeline과 진단 관측은 전환하지 않았으며, Source 결과 내보내기는 아직 명시적으로 중단됩니다. 기존 ESO 수치 차이 3,536개와 미규명 부동소수점 원인, A14/54·B14/64 및 전체 FAIL 상태는 유지합니다. 이는 endrun 완료 근거이며 전체 결과 내보내기나 Native 수치 동등성 완료를 뜻하지 않습니다.


[Source 실행·결과 내보내기 검증](../evidence/SRC-06-canonical-completed-source-activation-11/summary.json). 명시한 `--porting-scope A/B` 실행을 실제 Source 경로에 연결했습니다. 같은 상태 소유자로 초기화·본 실행·종료 620을 거친 뒤 완성된 ESO/MTR만 읽고, 보고 ID·빈도·저장 방식·원문 값·Native 시간 튜플을 JSON/CSV로 보존합니다. 정식 A-24H 실행과 전체 대상 컴파일이 성공했고, 58계열 5,352개 값의 물리 출력·JSON·CSV 일치를 검증했습니다. 보고값 차이 3,536개는 변경하지 않은 기존 허용오차와 계열별 RMSE 한계 안에 있습니다. 기본 실행 경로는 보존하며, B는 처음 만나는 미구현 입력·분기에서 오류로 중단합니다. 진단 관측 파일은 배포 경로에서 제외했습니다. 추가 기간·B 사례·전체 포팅 게이트는 이 A-24H 검증으로 완료 처리하지 않습니다. A14/54·B14/64와 기존 전체 비교 기록은 유지합니다.


[Source B 입력·출력 형식·보고값 비교 검증](../evidence/SRC-06-canonical-B-ingress-IO-reported-cases-12/summary.json). B 입력의 실제 값·확장 필드·선언 순서·양수 개수를 같은 InputProcessor에 보존하고, 실제 초기화의 Schedule:Compact 미구현 경계까지 진행했습니다. B 실행은 exit6으로 명시적으로 중단하며 완료 처리하지 않습니다. RDD/MDD는 원래 출력 시점에 Native와 같은 Windows 줄바꿈을 사용합니다. 같은 새 코어의 A-24H는 종료·내보내기에 성공했고 5,352개 값의 기존 전체 증명과 일치합니다. 이전 Source11의 A-72H 16,056개·연간 1,953,480개 보고값은 원래 허용오차·계열별 RMSE 실패가 모두 0입니다. 공개 스트림 비교 도구는 같은 알고리즘을 유지하며 A-72H 대조로 확인했습니다. 연간 비교는 반복하지 않았습니다. 숫자 차이와 정책 메타데이터의 기존 한계를 보존하며, 내부 비트 동일성·B 및 전체 15사례·카드 게이트 완료를 주장하지 않습니다. A14/54·B14/64는 유지합니다.


[Source Compact 스케줄 검증](../evidence/SRC-06-canonical-compact-schedules-13/summary.json). 무보간 Compact의 분별 값·타임스텝 합계·최솟값/최댓값·현재 시계 조회를 Native 순서로 연결했습니다. 단일 테스트에서 기본 입력과 빈 시각(:15) 변형의 96×5×2개 스케줄 값이 Native와 비트 단위로 일치했고, A-24H 종료·내보내기와 기존 5,352개 보고값 검증도 유지했습니다. B는 Compact 처리를 마친 뒤 GetHeatBalanceInput의 제어 대상 Zone 조회에서 중단합니다. 추가 보간·진단 분기와 후속 B 장비 포팅은 남아 있으며 A14/54·B14/64 게이트는 유지합니다.


[Source Zone·Node 입력 검증](../evidence/SRC-06-canonical-Zone-Node-14/summary.json). 같은 입력 소유자의 제어 Zone 조회와 NodeList·Assign·InitUnique 경로를 연결했습니다. 기존 Native 비제어 입력 1쌍(pair7)·소유자 전이 테스트 및 A-24H 회귀를 통과했고 5,352개 보고값의 기존 검증도 유지했습니다. B는 GetZoneEquipmentData308의 EquipmentConnections getObjectItem에서 중단합니다. 입력된 Node의 Native 상태 대조와 후속 B 장비 포팅은 남아 있으며 A14/54·B14/64 게이트는 유지합니다.


[Source 제어장비 입력 검증](../evidence/SRC-06-canonical-controlled-equipment-15/summary.json). 단일 Zone의 IdealLoads 장비 연결·목록·노드·terminal sizing 입력을 같은 실제 소유자에 연결했습니다. 기존 Native 비제어 입력 1쌍(pair7)·소유권 전이 테스트와 A-24H 회귀를 통과하고 보고값 5,352개의 기존 검증을 유지했습니다. B의 실제 미완료 경계는 `GetHeatBalanceInput: selected initially-closed six-quad volume domain`입니다. 제어장비 내부 상태의 Native 대조와 후속 B 경로는 남아 있으며 게이트는 A14/54·B14/64를 유지합니다.


[Source 체적·Node 경로 검증](../evidence/SRC-06-canonical-Node-lifecycle-16/summary.json). Native에 없는 추가 체적 제한을 제거하고 같은 Node 소유자의 보고 등록·실제 값 참조·필드 초기화를 연결했습니다. 기존 Native 비제어 입력 1쌍(pair7)·소유권 전이 테스트와 A-24H 회귀를 통과하고 보고값 5,352개의 기존 검증을 유지했습니다. B의 실제 미완료 경계는 `GetHeatBalanceInput: positive thermostat getObjectItem332`입니다. Node setup/reset의 실제 도달 범위는 증빙 요약을 따르며, 내부 상태의 Native 대조와 후속 B 포팅은 남아 있습니다. 게이트는 A14/54·B14/64를 유지합니다.


[Source 온도조절기 입력 검증](../evidence/SRC-06-canonical-thermostat-17/summary.json). 온도조절기·설정값 입력과 실제 선언 순서를 처리하고 같은 Zone·Schedule 소유자에 연결했습니다. 기존 소유권 테스트와 A-24H 회귀를 통과했으며 B의 실제 미완료 경계는 `GetHeatBalanceInput: positive actual TempControlledZone2700`입니다. 내부 상태의 Native 대조와 후속 B 포팅은 남아 있습니다. 게이트는 A14/54·B14/64를 유지합니다.


[Source 온도 설정값 경로 검증](../evidence/SRC-06-canonical-thermostat-current-19/summary.json). 실제 B 미완료 경계는 `GetHeatBalanceInput: positive controlled predicted load6862`입니다. 전체 B 완료와 내부 수치의 Native 동등성은 별도 검증 대상으로 남습니다.


[Source 제어 부하 예측 검증](../evidence/SRC-06-canonical-controlled-prediction-21/summary.json). 실제 B 미완료 경계는 `GetHeatBalanceInput: actual ZoneCompAvailMgrs positive allocation405`입니다. 내부 수치의 Native 검증과 전체 B 완료는 별도 과제로 남습니다.


[Source 설비 가용성 경로 검증](../evidence/SRC-06-canonical-component-availability-22/summary.json). 실제 B 미완료 경계는 `GetHeatBalanceInput: actual controlled ZoneEquipList count223`입니다. 전체 B 완료와 Node 상태의 Native 검증은 아직 주장하지 않습니다.


[Source 구역 설비 초기화 검증](../evidence/SRC-06-canonical-zone-equipment-initialization-23/summary.json). 실제 B의 미완료 경계는 `GetHeatBalanceInput: actual controlled Config.ZoneExh3665`이다. 내부 Native 상태 일치와 B 전체 완료를 주장하지 않는다.


[Source 구역 설비 시뮬레이션 진입 검증](../evidence/SRC-06-canonical-zone-equipment-orchestration-24/summary.json). B의 실제 미완료 경계는 `GetHeatBalanceInput: actual PurchasedAirManager SimPurchasedAir3871`이다. IdealLoads 본체와 B 전체 완료를 주장하지 않는다.


[단일 Native 코호트 테스트](../evidence/SRC-06-canonical-controlled-prediction-native-cohort-21/summary.json). 한 테스트에서 1,168개 호출과 시간 간격 축소 77회를 검증했습니다. 신규 CLI 실행을 주장하지 않으며 게이트는 A14/54·B14/64를 유지합니다.


[Source IdealLoads 입력·보고 연결 검증](../evidence/SRC-06-canonical-purchased-air-input-25/summary.json). PurchasedAir 입력과 실제 장비의 보고 선언을 연결했다. B는 `GetHeatBalanceInput: actual PurchasedAirManager InitPurchasedAir201`에서 중단되며, B 전체 완료는 후속 작업이다.


[Source IdealLoads 초기화 검증](../evidence/SRC-06-canonical-purchased-air-initialization-26/summary.json). IdealLoads 초기화의 실행 결과를 기록했다. B의 실제 경계는 `GetHeatBalanceInput: actual PurchasedAir CalcPurchAirLoads203`이며 Calc·Update·Report 및 B 전체 완료는 별도 작업이다.


[Source IdealLoads 계산·갱신·보고 검증](../evidence/SRC-06-canonical-purchased-air-calculation-27/summary.json). IdealLoads Calc·Update·Report 실행 결과를 기록합니다. B의 실제 경계는 `GetHeatBalanceInput: actual PurchasedAir post-callee output response4084`이며, 전체 B 및 내부 Native 수치 동등성 완료를 주장하지 않습니다.


[Source 장비 출력·수요 갱신 검증](../evidence/SRC-06-canonical-zone-equipment-output-28/summary.json). 출력 후처리·수요 갱신·가용성 경로의 실행 결과를 기록합니다. B의 실제 경계는 `GetHeatBalanceInput: actual controlled MassConservation5022`이며, 전체 B 및 내부 Native 수치 동등성 완료를 주장하지 않습니다.


[Source 구역 질량 보존·리턴 공기 검증](../evidence/SRC-06-canonical-controlled-air-balance-29/summary.json). 제어 구역 질량 보존과 리턴 공기 조건의 실행 결과를 기록합니다. 실제 B 경계는 `GetHeatBalanceInput: positive actual ZoneEquipConfig InletNode(1) read1939`입니다. 전체 B 및 내부 Native 수치 동등성 완료는 별도입니다.


[Source HVAC Node 생명주기 검증](../evidence/SRC-06-canonical-hvac-controlled-node-lifecycle-30/summary.json). HVAC Node 생명주기 묶음의 실제 실행 결과를 기록합니다. B의 실제 경계는 `GetHeatBalanceInput: positive controlled Zone supply sums5203`이며, 전체 B나 미정 Native 이력의 수치 동등성 완료를 주장하지 않습니다.


[Source 구역 온습도 보정 검증](../evidence/SRC-06-canonical-controlled-zone-corrector-31/summary.json). 제어 구역 온습도 보정 묶음의 실행 결과를 기록합니다. B의 실제 경계는 `GetHeatBalanceInput: positive component ZoneEquip inlet5488`이며, 전체 B 또는 내부 Native 보정 상태의 수치 동등성 완료를 주장하지 않습니다.


[예측·보정 Native 코호트 전용 실행](../evidence/SRC-06-canonical-predictor-corrector-native-cohort-31-cohort/summary.json). 기존 코호트 본문을 메인 스레드 전용 타깃으로 옮겨 예측·보정 각 1,168건과 양수 유입 810건을 비트 단위로 확인했다. 실행 명령은 `cargo test --locked -p ep_runtime --features native-controlled-air-cohort --test native_controlled_air_cohort`이다. 작업 스레드에서 발생한 실패·진단 3회를 보존했으며 계산식·기대값·허용오차는 바꾸지 않았다. 생산 경로 증거와 게이트 수는 체크포인트 31에서 이어받는다.


[Source33 component/Node report](../evidence/SRC-06-canonical-component-node-report-33/summary.json). B의 실제 결과: `GetHeatBalanceInput: actual controlled ZoneVentRepVars/ZoneEquipConfig.ZoneName1993–1994 before zone declarations`. 이번 단계는 CLI 두 건으로 검증했으며, 테스트 코호트 결과는 부모 증거로 구분합니다.


[Source34 controlled-zone ventilation](../evidence/SRC-06-canonical-controlled-zone-ventilation-34/summary.json). 제어 구역의 실제 환기 보고 상태에 출력 20개를 Native 순서로 연결했다. 구역명과 보고값은 기존 소유자에서 직접 참조하며 앞 8개는 실제 VentLoadsReportEnabled를 따른다. B 실행 결과: `GetHeatBalanceInput: actual CheckNodeConnections919 NodeConnections.ConnectionType first record access`. A24 회귀는 기존 5,352개 보고값 증거를 이어받고, 계산식 변경이나 테스트 추가는 없다.


[Source35 startup connection validation/report](../evidence/SRC-06-canonical-startup-connection-validation-report-35/summary.json). 시작 단계의 노드 연결 검사와 연결 보고를 하나의 변경 단위로 연결했다. 실제 연결 기록과 Config·장비 목록·물리 NodeID를 사용하며, 미구현 진단과 양수 부모 목록은 명시적 경계로 남긴다. B 실행 결과: `GetHeatBalanceInput: actual CreateEnergyReportStructure2502 controlled ZoneEquipList.NumOfEquipTypes`. A24의 기존 5,352개 보고값 증거를 이어받고 새 테스트나 계산식 변경은 없다. 각 파일의 정확한 Program Version 첫 줄만 제외한 BND 본문 8,518바이트·106줄이 보존된 Native 결과와 일치했다. 이 비교는 보고 I/O에 한정한다.


[Source37 energy report structure](../evidence/SRC-06-canonical-energy-report-structure-37/summary.json). 제어 구역 장비 목록과 실제 연결·계량변수 조회 결과로 에너지 보고 구조를 만들도록 연결했다. 같은 EquipmentData에 기록하며 미구현 부모·자식·다른 계통은 실제 연산 경계로 남긴다. B 실행 결과: `GetHeatBalanceInput: actual InitEnergyReports145 FindItemInList/EquipListIndex assignment`. A24의 기존 5,352개 보고값 증거를 이어받고 새 테스트나 계산식 변경은 없다. 각 파일의 정확한 Program Version 첫 줄만 제외한 BND 본문 8,518바이트·106줄이 보존된 Native 결과와 일치했다. 이 비교는 보고 I/O에 한정한다.


[Source38 energy report initialization](../evidence/SRC-06-canonical-energy-report-initialization-38/summary.json). 실제 장비 목록과 연결 정보를 사용해 에너지 보고 초기화의 두 순회와 표 기록 순서를 연결했다. 반복 보고에서는 같은 Which 참조의 현재 값을 읽어 기존 MeterData에 기록한다. B 실행 결과: `GetHeatBalanceInput: actual ReportSystemEnergyUse3155 ZoneSysEnergyDemand.TotalOutputRequired`. A24의 기존 5,352개 보고값 증거를 이어받으며 새 테스트나 계산식 변경은 없다.


[Source39 controlled system reports](../evidence/SRC-06-canonical-controlled-system-report-39/summary.json). 실제 수요·장비·물리 노드·구역 평균 상태를 빌려 정상 제어 구역의 에너지·환기 보고 계산과 누적 순서를 연결했다. 외기 유량이 0이어도 혼합 공기와 현재 환류 상태를 읽고 밀도를 계산한다. B 실행 결과: `GetHeatBalanceInput: actual checkSetpointNodesAtEnd first NodeSetpointCheck1588`. A24는 기존 보고값 5,352개의 증거를 이어받으며, 새 보고 계산의 Native 수치 대조는 별도 관측·재생 검증 대기 상태다. 보존된 Native BND 본문 8,518바이트·106줄과 일치하며, 이 비교는 보고 I/O 범위다.


[Source41 node setpoint endrun](../evidence/SRC-06-canonical-node-setpoint-endrun-41/summary.json). 같은 노드 검사 상태와 RuntimeLanguage를 사용해 종료 시 setpoint 검사의 전체 정상 순회를 연결했다. 실제로 요청된 제어만 순서대로 검사하고, 미구현 액추에이터·경고·최종 오류는 해당 연산 경계로 남긴다. B 실행 결과: `GetHeatBalanceInput: actual FillRemaining controlled ZoneEquipConfig.NumInletNodes6370`. A24의 기존 5,352개 보고값 증거를 이어받으며 새 테스트나 수치 계산 변경은 없다.


[Source40 Native report cohort](../evidence/SRC-06-canonical-system-report-native-cohort-40/summary.json). 기존 프로세스 메인 코호트의 보고 전용 선택으로 실제 엔진 306회와 전체 Native 함수의 별도 입력 변형 165회, 총 471회 보고 호출을 비트 단위로 대조했습니다. 비영 부하 8분류와 인접 이진64 경계를 포함하며, 기존 예측·보정 1,168쌍은 수정 없이 이번 실행에서 제외했습니다. 양의 1차 공조·DSOA 및 전체 B 완료는 주장하지 않습니다. 관측 입력 배열 복원 누락으로 실패한 첫 실행과 수정 후 성공 실행을 함께 보존합니다.


[Source42 controlled predefined entries](../evidence/SRC-06-canonical-fill-remaining-controlled-42/summary.json). 제어 존의 FillRemaining 표 항목과 누계를 같은 Config·Zone·ZonePreDefRep에 연결했다. 입력된 노드 순서와 나눗셈 묶음, 출력 자릿수를 보존한다. B 실행 결과: `GetHeatBalanceInput: actual thermostat DualHeatCool heatSched->Num6642`. A24의 기존 5,352개 보고값 증거는 전체 결과 비교 후 이어받는다.


[Source44 thermostat predefined reports](../evidence/SRC-06-canonical-thermostat-predefined-44/summary.json). 온도조절기의 설정값과 스케줄 요약을 실제 Schedule 소유자에 연결했다. 날짜별 값·일수·월을 Native 순서대로 구한다. B 실행 결과: `Source B-NOLIMIT-24H completed through actual620`. A24의 기존 5,352개 보고값 증거는 전체 결과 비교 후 이어받는다.


[Source45 NoLimit B24/72/annual validation](../evidence/SRC-06-canonical-nolimit-duration-validation-45/summary.json). 무제한 B의 24시간·72시간·연간 Source 실행과 같은 케이스의 Native 보고값 전체 비교를 확인했다. 기존 허용오차와 RMSE 조건을 모두 통과했으며 원시값·비트 차이는 비교 보고서에 남겼다.


[Source43 hard-sized PurchasedAir](../evidence/SRC-06-canonical-hard-sized-purchased-air-43/summary.json). Combined16 files (production13 plus six test successors with three shared paths) retain the existing test target/feature and old Air/report helpers. Actual59case science and five Source CLI runs are separate proofs; NoLimit9792/A5352 regressions transfer by whole outputs, and three limit24h reported comparisons pass with CAPACITY25 raw/bit differences preserved. Limit72h/annual, remaining card obligations and ancillary full-I/O equivalence stay open.


[Source46 limit-mode reported validation](../evidence/SRC-06-canonical-limit-mode-reported-validation-46/summary.json). 유량·용량·동시 제한 B의 24시간·72시간·연간 9개 케이스를 각각 같은 Native 기준과 전체 비교했다. 기존 허용오차와 RMSE 조건을 모두 통과했으며 원시값·비트 차이는 보고서에 남겼다.
