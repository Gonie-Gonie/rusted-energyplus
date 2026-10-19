# ZON-05 — 현재 범위의 수분수지

- 그룹: 08 존 공기
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: ZON-01, PSY-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/ZoneTempPredictorCorrector.cc`  
심벌: `ZoneSpaceHeatBalanceData::correctHumRat`  
대상 블록: 현재 no-OA·무잠열원 분기 및 공조노드 피드백의 습공기비 처리. 일반 습도제어와 별도.  
함께 읽을 선언/호출자: ZoneTempPredictorCorrector.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/ZoneTempPredictorCorrector.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

heat_balance/zone_humidity_correction.rs; zone_air_initialization.rs; state.rs

선택한 직접 함수와 단위 관측기를 구현했다. 실제 적용 소스와 검증 기준은 `evidence/ZON-05-unit/summary.json`에 기록한다.

## 입력 계약

존 W 이력·공기밀도·체적·dt, 범위 내 수분 유입/유출

## 출력·변경 상태 계약

직접 함수: 임시 W[kg/kg]와 양수 시스템 존 노드의 W·엔탈피. 현재 W·평균 W·이력 선택·갱신은 호출자의 별도 연결 범위이다.

## 호출 시점

Correct 단계

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

외부 수분원0에서 불변성; 표본 W 변화; 온도변화와 rho 영향

## 연결시험

ZON-02의 cp/rho 및 HVAC 공급노드 W와 일관성

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

Full correctHumRat4433–4619 selected zone/Mixing/ThirdOrder/noAFN/noOA; lower/saturation/positiveNodeW/H. Current W/RH/weighted means/history are actual parent/multiplier/P1 connections, not inferred copies.

실제 구현 기준은 Source59 `a2c512a246f73dc806805c6c4e277c2960fdda44`입니다. [최종 판정](../evidence/SRC-06-final-current-source-validation-60/summary.json)의 해당 카드별 범위·gate 근거와 현재 연결 결과를 참조합니다. 이전 단위 명령은 원래 근거에 보존되며 현재 바이너리 재실행으로 집계하지 않습니다.

직접 단위 근거: [SRC-06-canonical-predictor-corrector-native-cohort-31-cohort](../evidence/SRC-06-canonical-predictor-corrector-native-cohort-31-cohort/summary.json), [SRC-06-canonical-warmup-multiplier-exterior-55](../evidence/SRC-06-canonical-warmup-multiplier-exterior-55/summary.json), [ZON-05-unit](../evidence/ZON-05-unit/summary.json).

선택 본문의 정확한 행·원문 해시·역할과 직접 helper 근거는 해당 카드의 `evidence_by_scope.selected_source_extents` 및 최종증거팩의 본문 구간 원장에 기록합니다. 선언 행은 심볼 위치 식별에만 사용합니다.

과거 실패·미관측 값·제외 분기는 불변 근거에 남습니다. 수치 단위, 현재 연결, 보고값 정책, 물리 파일 직렬화의 판정을 합치지 않습니다.
