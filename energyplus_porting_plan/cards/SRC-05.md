# SRC-05 — 표면 높이의 외기조건

- 그룹: 05 열원·외부 경계
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: CLK-04, GEO-02
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/DataSurfaces.cc`  
심벌: `SetSurfaceOutBulbTempAt; SetSurfaceWindSpeedAt; SetSurfaceWindDirAt`  
대상 블록: 기본 weather-station·terrain 보정 경로만. 호출 위치는 InitSurfaceHeatBalance.  
함께 읽을 선언/호출자: DataSurfaces.hh; HeatBalanceSurfaceManager.cc::InitSurfaceHeatBalance 호출 블록  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/DataSurfaces.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

heat_balance/convection.rs; surface_weather.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

기상 dry/wet bulb·풍속/풍향, 지형, 중심점 높이[m]

## 출력·변경 상태 계약

표면별 dry/wet bulb[°C]·풍속[m/s]·풍향[deg]

## 호출 시점

표면 timestep 초기화

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

높이0/기준/상이한 높이; terrain; WindExposed/NoWind

## 연결시험

SRC-07/08에 같은 snapshot 전달

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

## Historical SRC-05 bounded Source proposal (before application)

The exact selected bodies are `DataSurfaces.cc:594–618,634–666`; their caller is
`HeatBalanceSurfaceManager.cc:308–345`. The intervening temperature guard
`DataSurfaces.cc:620–632` delegates the severe/continue/fatal backend to
`DataEnvironment.cc:236–247`; that backend is a remaining integration obligation.
Declarations and allocation/initialization are `DataSurfaces.hh:1540–1543` and
`SurfaceGeometry.cc:862–873`. Weather-station coefficients are produced at
`WeatherManager.cc:7233–7301`; constants are `DataEnvironment.hh:79–80`.
Terrain/default scope is `HeatBalanceManager.cc:566–585,1252–1317` plus CON-01.

Use actual all-surface order, stored centroid height, ExtWind and incoming four
owners. Zero gradient copies global bulbs without the station modifier. Nonzero
gradient uses the EarthRadius quotient and clamps nonpositive height to base.
Wind uses the original factor/pow order; nonzero exponent skips NoWind writes,
while zero exponent writes every surface. Wind direction copies raw bits.
Local environments, EMS overrides and explicit station/height variation remain
outside ordinary admission; direct operand cases do not expand that admission.

Held Rust setters and compatibility-helper corrections are Source proposals,
with four unexecuted owner/branch regressions. No numerical Native claim or gate
change is made. The ordinary runtime still lacks the retained four-array owner
and true setter/check sequence at InitSurfaceHeatBalance. Full production
handoff, guard/backend and SRC-07/08 consumers remain separate prerequisites.
The actual failed persistent comparison remains evidence; no policy is changed.

## Historical selected surface environment comparison (libtest observer)

Genuine unchanged-core Native calls and actual Rust selected setters did not pass the selected scope on 26 literal cases and 30 weather operations. Actual status: fail-selected-setters-or-input-join; input joins 2,112, full pre-owner checks 1,384, selected endpoint checks 1,334, point-helper checks 189; mismatches 3. Required-unavailable observations 103 and nonfinite arithmetic exclusions 0 are retained without PASS. Rust Source371ee432 has 3,919 members; its one-row predicate repair left 3,918 unrelated files unchanged. fmt/Clippy passed; focused329/workspace4,953 passed with two intentionally ignored observers, and the explicit SRC05 invocation passed one test. Both attempted comparison chains, complete observations and all ledgers are retained. Cold guard/backend, Native-unreached/skipped observations, ordinary manager handoff and whole-card gates remain pending. No gate changed.
Evidence: [selected setters, complete observations and quality](../evidence/SRC-05-surface-environment/summary.json) (cc8a0746b6b5143a7b79376270ee66f3aaf860c4c1184e025953fcfef814193f).

## Actual standalone main comparison and wind diagnostics

The independent standalone Rust main and genuine Native setters matched the available selected scope: 26 literal cases, 30 weather operations, 71 paired setter boundaries, 2,112 input joins, 1,384 full pre-owner and 1,384 selected endpoint checks, 189 point-helper checks, zero mismatches. All 102 required-unavailable rows remain excluded from PASS. This proof is bound to historical Source d34a84b7, its four closed fmt/Clippy/focused/workspace checks (341/4,968 passed, two ignored observers), the sole actual Cargo-emitted dev executable and its argc3 main execution. Later SRC-06 changes do not rewrite this capture.

Separate read-only floating point diagnostics retain all 25 cases in main and spawned lanes: 29 intermediate/result differences, 662 stage-control field differences and four lane-control differences. Native and Rust main lanes had no result differences; the spawned lane had 29. Actual x87 control words were 0x037f for Native main/spawn and Rust main, and 0x027f for Rust spawn. Both linked pow providers use the observed x87 helper family. These observations establish neither causation nor the historical libtest worker controls. No floating point controls, formula, tolerance or production execution path were changed for this result. The previous three-mismatch comparison and all diagnostics remain retained.

The production CLI and ordinary observer still use a worker. Standalone selected parity does not prove that path, the cold guard/error backend, retained ordinary four-array setter/check sequence, SRC-07/08 handoff or whole SRC-05. All four gates remain pending; A 14/54 and B 14/64 are unchanged.

Evidence: [complete selected observations, diagnostics, custody and historical failure](../evidence/SRC-05-wind-diagnostics-and-main-thread/summary.json).
