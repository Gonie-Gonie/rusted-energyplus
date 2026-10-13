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

원본 호출 위치를 구분한다. BeginSim의 Solar 초기화는 **`HeatBalanceManager.cc:2617–2630`**이며 2629는 안내 출력, 2630이 `InitSolarCalculations` 호출이다. **`HeatBalanceSurfaceManager.cc:424–425`**는 공유 firstTime 조건의 `InitInteriorRadExchange`; **`HeatBalanceIntRadExchange.cc:610–614`**는 조건부 Solar F 복사다. HBSM615는 ZoneSizing Gather 분기의 닫는 행이며 Solar 초기화 위치가 아니다. 현재 Rust의 선택 초기화 순서 검증을 이 전체 Original caller의 완료로 확대하지 않는다.

## 실제 증거와 검증 범위

아래 22개 요약은 plan의 actual 항목 전부다. 색인/ZIP은 실제 존재하는 파일만 연결한다. QA는 각 단위의 당시 Source에서 수행한 기록이며 현재 전체 포팅의 수치 PASS를 대신하지 않는다.

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

기존 Weather03·absent site/station과 cold guard는 [SRC-05 실제 선택 비교](../evidence/SRC-05-weather-cold/summary.json)에 별도로 고정되어 있다. 이는 complete ordinary weather/inside 배열·reachability·handoff의 완료 증거가 아니다.

## 실패와 미대조 수치의 현재 상태

| 항목 | 유지할 판정 | 의미 |
| --- | --- | --- |
| [전체 ordinary main-thread 비교](../evidence/CTF-10-main-thread-history-comparison/summary.json) | **FAIL 유지**: mismatch 235,456; required unavailable/unmatched 2,754 | 과거 실제 전체 비교다. 후속의 한정된 owner·수치 replay·Rust 회귀가 이 실패를 재검증하거나 대체하지 않았다. 현재 전체 동등성은 미확인이다. |
| DesignDay 계산값 strict-bit 진단 | **FAIL 유지**: 차이 9종, 최대 절대 차이 `2.2737367544323206e-13` | 원인 미확정. 새 허용오차로 PASS로 바꾸지 않았다. 기존 PSY 계약의 caller-output 5종 평가는 중간 입력 비트·cache 동등성 증거가 아니다. |
| DesignDay 계산값 8종 | **UNPAIRED**, 수치 PASS 없음 | `OutRelHum`, `OutRelHumValue`, `SkyTemp`, `SkyTempKelvin`, `HorizIRSky`, `BeamSolarRad`, `DifSolarRad`, `GndSolarRad`. 유한-domain·class/부호 있는 0 검사는 수치 크기 판정이 아니다. |

DesignDay의 정수·플래그·배열 범위·보고 문자열과 Rust 내부 Today→Tomorrow 전체 carrier copy는 별도의 exact 검증이다. Native 계산값 전체의 비트 동등성, psychrometric 입력/cache, 원인으로서의 libm은 확인되지 않았다. Natural의 생성 nonfinite 6개, dispatcher의 Native-only 264개와 geometry의 선택 외 19개도 수치 PASS에 포함하지 않는다.

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

## 이력과 증거 보존

반복된 착수 전 산문과 당시 미완료 목록은 현재 범위와 위 표로 통합했다. 원본 범위·적용/formatter diff·실패/중단 명령·실제 QA·선택 입력과 uncapped ledger는 기존 요약/색인/ZIP에 그대로 남긴다. 초기 입력/transport/compile/Clippy 실패를 나중 성공 로그로 바꾸지 않는다. 과거 global file-size 실패와 구조적 부채도 현재 통과로 다시 쓰지 않는다.

대형 Native 실행 파일·core·map·provider와 whole Source는 각 단위의 기존 provenance 범위다. 이 문서 정리는 payload 재검증이나 transitive build/replay 보장을 추가하지 않는다. Native endpoint는 생산 입력으로 주입하지 않으며, 실제 BEFORE 값을 쓰는 bounded replay는 테스트 범위로만 해석한다. 모든 gate와 A/B 완료 수는 그대로다.
