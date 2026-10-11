# SYS-03 — warmup→RunPeriod 상태 인계

- 그룹: 10 실행·검증
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: SYS-02, CLK-03, SCH-03, SUR-06
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/HeatBalanceManager.cc`  
심벌: `ManageHeatBalance; InitHeatBalance; CheckWarmupConvergence`  
대상 블록: warmup flag 전환과 첫 본계산의 초기화 조건. SimulationManager/Weather의 해당 caller 전환 블록만 동봉.  
함께 읽을 선언/호출자: SimulationManager.cc의 BeginEnvrn/WarmupFlag caller 블록; WeatherManager.cc::ReadEPlusWeatherForDay  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/HeatBalanceManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

heat_balance/warmup.rs; run_period.rs; ep_run

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

마지막 warmup 확정상태·calendar/weather 버퍼·환경flags

## 출력·변경 상태 계약

첫 RunPeriod에 유지/초기화되는 필드 목록과 정확한 상태

## 호출 시점

마지막 warmup 종료→본 계산 첫 timestep

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

전/후 전체 허용 필드 snapshot; 첫1/2/24시간 추적

## 연결시험

첫 SurfTempIn·CTF 상수항·MAT·W 차이가 시작되는 최초 단계 특정

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

## Historical bounded warmup owner/caller amendment (before application; all gates remain pending)

- Pin HeatBalanceManager.cc 180-244 (caller), 2598-2663 and 2880-2918 (actual allocation/BeginEnvrn), 2971-3057 (RecKeepHeatBalance), and 3059-3226 (CheckWarmupConvergence). The daily owner is per Zone, distinct from MAT, CTF history and summary.
- Direct demand prerequisites are ZoneTempPredictorCorrector.cc 3863-3940, 4043-4128 and DataZoneEnergyDemands.cc 323-329 with its actual declarations. Only the real absent system-zone-node branch supplies literal SNLoad=0; predicted demand, PurchasedAir supply output or missing metadata must never become its replacement.
- BeginEnvrn previous heat/cool/max/min owners start at +0. Current heat/cool/max start at -9999 and min at 1000. Record actual ZTAV and accepted unmultiplied airSysHeatRate/airSysCoolRate after each real timestep; never substitute MAT for unavailable ZTAV.
- Check every Zone's max/min temperature differences and normalized heat/cool changes in Native order. Preserve the strict >1e-4 load guard, in-place 100 W floor, retained comparison values in inactive branches, then per-zone transfer/reset. LoadsConvergTol is dimensionless; the existing Rust `_w` spelling is historical.
- The bounded first implementation admits actual uncontrolled A topology through real ModelGraph node absence plus real equipment-connection absence. Controlled or unknown demand ownership is explicitly unavailable; no B warmup activation or synthetic load producer is included.
- The genuine ordinary A pipeline must choose the existing Building-based warmup options producer; the current hourly_samples call disables warmup. The separate diagnostic branch keeps its explicit old options, and B is unchanged.
- Native maximum-day severe/continue effects and first-RunPeriod convergence reports remain required actual backend/environment prerequisites. A forced maximum-day exit is not convergence, and this bounded unit does not close those report/error gates.
- After the actual final warmup EndDay decision, retain all thermal/demand/daily owners, set the real WarmupFlag false and DayOfSim/DayOfSimChr to zero without weather transport, then let the existing next RunPeriod BeginDay establish day 1. Pin SimulationManager.cc 404-556 and the direct weather caller/owner declarations.
- Required tests include first-day previous-zero behavior, all four tests including real load failure, floor/guard/retained-value ordering, actual uncontrolled report reachability, rejection before callback for controlled topology, and warmup-to-RunPeriod owner/clock retention. Any future Native comparison uses genuine same inputs and all reached owners; no result-based fixtures or convergence-count expectations.

At this amendment stage, the proposed Rust changes were held Source only. No Native/candidate execution, numerical PASS, completed scope/unit/integration/production gate or predecessor completion is claimed.

## Applied uncontrolled warmup owner and quality

Actual Source8fc2a29b retains3,915 files with3,902 unrelated files byte-exact. The strict ordinary A caller now enables model warmup; actual uncontrolled sensible reports feed all-zone four-criterion convergence and the final accepted EndDay hands weather globals through Day0. fmt/Clippy passed; focused323/workspace4,947 tests passed, zero failed/ignored. Native same-input numerical comparison and ordinary handoff parity remain unverified. Full rolling/report/EIO/diagnostic effects and controlled demand remain pending; no card gates changed.
Evidence: [actual application and quality](../evidence/SYS-03-warmup-owner/summary.json) (`3922d1a4dae0c8f190903bc6c8e3d2833ef2fb36a8c20158fdbbeb396c509e71`).

## Bounded Original R formatter prerequisite

Before extending actual warmup report and diagnostic effects, port only the Original IOFiles.hh R formatting branch and its directly invoked helpers: lines117-140,247-305 and734-786 at the pinned26.1.0 revision. All relevant fmt caller ranges and input contracts are retained in the reviewed Source preparation. Inputs are actual IEEE binary64 values and explicit precision; output is the exact UTF-8 string or an explicit unavailable domain. Preserve operation order, strict fixed-output threshold, sign/zero behavior, three fixed versus one exponential nextafter steps, positive large-number conversion/truncation and minimum three-digit exponent padding. Do not rescale decimals, substitute expected strings, change F/G formatting or weaken comparisons.

The prospective124 input values and five formats declare620 observations. Positive infinity reaches Original undefined log10-to-int conversion; its five rows must remain explicitly uninvoked/unavailable in both lanes. All other615 rows require genuine Original formatting and exact text comparison. Record full input/executable bytes before and after each invocation, actual linked inline header/provider provenance, actual Cargo-emitted binary, all five literal-to-formatted Source correspondences and complete outputs. Native executes before applying the Rust formatter. This helper prerequisite does not claim complete report/backend, warmup handoff, whole-card or production gates; those retain their existing pending status.

## Actual bounded Original R format comparison

Genuine unchanged-core Original R formatting and the actual Rust formatter passed the selected available exact-text scope: 124 literal binary64 values, 620 declarations and 615 required nonexcluded text pairs; actual returned pairs 615, mismatches 0. The five positive-infinity rows in each lane remain explicitly uninvoked/unavailable and are not PASS. All precisions0/2/3/4/10, full input/executable/custody proofs, provider/loader attempts and complete observations are retained. Actual Source5660999c has3,922 members and3,917 unrelated files unchanged. fmt/Clippy passed; focused333/workspace4,957 passed, zero failed, with actual ignored counts2/2. Floating exception state/NaN payload, full report/backend, ordinary warmup handoff and whole-card gates remain pending; no gate changed.
Evidence: [selected exact texts, full observations and quality](../evidence/SYS-03-round-format/summary.json) (0bb2ff7ffaace772c680b90cf3b4a0aed57eb185e941048783e42c83dcae09ab).

## Ordinary A report/backend continuation contract (reviewed before application)

- The explicit ordinary uncontrolled A entry retains actual first-instance Raw Output:Diagnostics detailed-report facts and selected SimulationControl inputs. Active sizing/pure-load/controlled demand are unavailable; B, generic and manual callers do not infer this reporting contract.
- Reborrow the same caller-owned CtfRunOutputs EIO/error/console/backend through initialization, warmup and Run. Preserve reached bytes and typed IO/backend causes across failure; optional callback/SQLite owners remain the actual caller operands, with no substitute sinks.
- Genuine BeginEnvrn resets daily previous/current extrema, rolling Temp/Load/Prev/SecPrev sentinels, report difference/full matrices and Count in Original 2639-2652 order. LoadZoneSecPrevDay, scratch and constructor header/warning flags are not reset by that block. Direct Allocate remains a distinct +0 lane.
- After each actual successful thermal timestep, the admitted uncontrolled SNLoad report supplies fresh accepted rates/stamp, then RecKeep updates every Zone through all warmup and Run timesteps. The first non-Warmup Run day alone collects detailed/full report matrices; the summary requires actual EndDay/DayOfSim1/!DoingSizing. Truncated first-day samples do not report a fabricated full day.
- At maximum warmup days, each real Zone check precedes its severe/continue effects and then transfer/reset. Retain both once-warning flags, actual environment/phase and all backend counters/prefixes; emit the global insufficient-days severe after the Zone loop before WarmupFlag mutation. Shared Source R formatting is a separately reviewed dependency, not an assumed text PASS.
- New tests exercise real Rust production weather A caller continuation through partial/full/two Run days, report Count/headers/tails and fresh timestamps, plus maximum-day multi-Zone diagnostic order/once flags and actual failing sink stops. No new Native observation, whole SYS-02/03, ordinary parity or gate PASS is claimed by this held Source.

## Actual selected report comparison and ordinary A quality

Genuine unchanged-core Native and Rust selected report owners/EIO matched across 25 cases, 371 boundaries and 14 actual Report calls: 3,325 initial report-owner checks and 210,245 report endpoint checks, zero mismatches. This comparison remains bound to Source d362e54d and was not rerun after the test-only fixture correction. The correction supplies the existing authored loader-name owner; all 3,927 unrelated Source members, including production code, remain byte-exact. Current Source d34a84b7 passed fmt/Clippy, the targeted ordinary A regression 1/0/0, focused 341/0/2 and workspace 4,968/0/2 (passed/failed/ignored). Earlier Clippy/workspace failures and both Source histories are retained.

Five families remain explicitly unavailable and are not PASS: warning_flags, sensible_report_producer_and_energies, error_backend_and_callbacks, error_console_prefixes and ordinary_weather_handoff. Selected report proof and genuine Rust caller tests do not establish full backend/producer equivalence, ordinary Native handoff parity or whole SYS-02/03 completion. All gates and A 14/54 / B 14/64 totals remain unchanged.

Evidence: [selected report owners/EIO, historical comparison and current quality](../evidence/SYS-03-ordinary-warmup-report/summary.json).
