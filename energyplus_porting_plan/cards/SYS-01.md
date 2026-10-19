# SYS-01 — 표면·공기·HVAC·시간간격 연결

- 그룹: 10 실행·검증
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: SUR-05, ZON-06
- 조건부 선행 카드: {"B": ["HVAC-08"]}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/HVACManager.cc`  
심벌: `ManageHVAC; SimHVAC; SimSelectedEquipment`  
대상 블록: 대상 zone-equipment dispatch, predict→Calc→correct 및 accepted/retry 제어. adaptive 활성 시 DownInterpolate4HistoryValues 포함.  
함께 읽을 선언/호출자: HeatBalanceSurfaceManager.cc::ManageSurfaceHeatBalance; ZoneTempPredictorCorrector.cc::DownInterpolate4HistoryValues; ZoneEquipmentManager.cc의 PurchasedAir 호출 분기  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/HVACManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

heat_balance/algorithm.rs; timestep.rs; ideal_loads/coupled_runtime

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

기상·스케줄·표면상태, system/zone 시간, 수렴·분할 규칙

## 출력·변경 상태 계약

같은 timestep의 최종 존/표면/설비 상태 및 accepted 단계열

## 호출 시점

zone timestep 안의 system 호출들

## 제외 범위

AirLoop/Plant 반복은 제외. EP가 선정 입력에서 timestep을 줄인다면 Rust의 고정 timestep으로 임의 대체하지 않는다.

## 단위시험

비공조/IdealLoads; 단일/축소step; 재시도시 원복; 분할 후 합계시간

## 연결시험

표면→공기/HVAC→CTF commit 순서를 원본 ManageSurfaceHeatBalance와 대조

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

Connection-only accepted clock/stage/predict→Sim→correct→report→history order. No whole ManageHVAC numerical matrix implied by a successful parent path.

실제 구현 기준은 Source59 `a2c512a246f73dc806805c6c4e277c2960fdda44`입니다. [최종 판정](../evidence/SRC-06-final-current-source-validation-60/summary.json)의 해당 카드별 범위·gate 근거와 현재 연결 결과를 참조합니다. 이전 단위 명령은 원래 근거에 보존되며 현재 바이너리 재실행으로 집계하지 않습니다.

직접 단위 근거: [SRC-06-canonical-predictor-corrector-native-cohort-31-cohort](../evidence/SRC-06-canonical-predictor-corrector-native-cohort-31-cohort/summary.json), [SRC-06-canonical-purchased-air-surface-53](../evidence/SRC-06-canonical-purchased-air-surface-53/summary.json), [SRC-06-canonical-output-processor-air-history-54](../evidence/SRC-06-canonical-output-processor-air-history-54/summary.json), [SRC-06-canonical-warmup-multiplier-exterior-55](../evidence/SRC-06-canonical-warmup-multiplier-exterior-55/summary.json).

선택 본문의 정확한 행·원문 해시·역할과 직접 helper 근거는 해당 카드의 `evidence_by_scope.selected_source_extents` 및 최종증거팩의 본문 구간 원장에 기록합니다. 선언 행은 심볼 위치 식별에만 사용합니다.

과거 실패·미관측 값·제외 분기는 불변 근거에 남습니다. 수치 단위, 현재 연결, 보고값 정책, 물리 파일 직렬화의 판정을 합치지 않습니다.
