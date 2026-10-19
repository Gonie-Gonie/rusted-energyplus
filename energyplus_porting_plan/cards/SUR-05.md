# SUR-05 — 실내 표면 반복·수렴

- 그룹: 07 표면 열수지
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: SUR-03, SUR-04
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/HeatBalanceSurfaceManager.cc`  
심벌: `CalcHeatBalanceInsideSurf2CTFOnly`  
대상 블록: 반복 초기화→복사→내표면 solve→수렴검사·대류 재평가→종료 블록. 수식은 독립 카드 호출.  
함께 읽을 선언/호출자: HeatBalanceSurfaceManager.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/HeatBalanceSurfaceManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 현재 Source 경로

`crates/ep_runtime/src/heat_balance/ordinary_surface_initialization/inside_driver.rs`, `inside_providers.rs`와 실제 `error_backend/inside_diagnostics.rs`를 사용합니다. 관측기는 `source_surface_cadence.rs`, 지정 분기 호출은 `native_convergence_branch.rs`입니다.

## 입력 계약

첫 반복 snapshot, SUR-04 kernel, 수렴 기준·최대 반복수

## 출력·변경 상태 계약

수렴 내표면온도·h·복사·iteration count

## 호출 시점

zone timestep의 표면 solve

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

빠른수렴·여러회수렴·비수렴; 첫 반복과 후속 반복의 입력

## 연결시험

EP와 iteration count·첫 2회 표면 상태 및 종료상태 비교

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 최종 네 gate 근거

| Gate | 범위별 Root 판정 |
|---|---|
| scope_review | A: 통과 / B: 통과 |
| unit_gate | A: 통과 / B: 통과 |
| integration_gate | A: 통과 / B: 통과 |
| production_gate | A: 통과 / B: 통과 |

30 mechanically literal Original9236–9272 suffix evaluations/3 lifetimes cover500/501,converged,warmup/sizing and real diagnostics; zero full inside calls in that unit. Normal full actual2025 solver lifetimes/first2/exit are separate P3. No natural501 reach claim.

실제 구현 기준은 Source59 `a2c512a246f73dc806805c6c4e277c2960fdda44`입니다. [최종 판정](../evidence/SRC-06-final-current-source-validation-60/summary.json)의 해당 카드별 범위·gate 근거와 현재 연결 결과를 참조합니다. 이전 단위 명령은 원래 근거에 보존되며 현재 바이너리 재실행으로 집계하지 않습니다.

직접 단위 근거: [최종증거팩: Root-SUR03-SUR05-actual-review-56 a3210cec2e4a](../evidence/SRC-06-final-current-source-validation-60/summary.json).

선택 본문의 정확한 행·원문 해시·역할과 직접 helper 근거는 해당 카드의 `evidence_by_scope.selected_source_extents` 및 최종증거팩의 본문 구간 원장에 기록합니다. 선언 행은 심볼 위치 식별에만 사용합니다.

과거 실패·미관측 값·제외 분기는 불변 근거에 남습니다. 수치 단위, 현재 연결, 보고값 정책, 물리 파일 직렬화의 판정을 합치지 않습니다.
