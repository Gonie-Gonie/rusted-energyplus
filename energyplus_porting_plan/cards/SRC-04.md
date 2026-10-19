# SRC-04 — 외표면 입사·흡수 일사

- 그룹: 05 열원·외부 경계
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: SRC-03, CLK-05, GEO-02
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/SolarShading.cc`  
심벌: `FigureSolarBeamAtTimestep; AnisoSkyViewFactors; CalcAbsorbedOnExteriorOpaqueSurfaces`  
대상 블록: 차양기하가 없는 대상 불투명 표면의 beam/sky/ground 및 흡수율 적용. 활성 diffuse 분기는 0이 아닌 시험을 포함.  
함께 읽을 선언/호출자: SolarShading.hh; 표면별 태양/sky/ground view factor 선언  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/SolarShading.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/heat_balance/solar.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

태양방향, DNI/DHI, 반사율·흡수율, 표면노출 flag

## 출력·변경 상태 계약

표면 입사/흡수 일사[W/m²], 총 열원[W]

## 호출 시점

외표면 열수지 조립 전

## 제외 범위

차양 폴리곤 중첩, 창 투과·내부 일사분포는 범위 밖. 새 입력에서 활성화되면 별도 카드가 필요.

## 단위시험

밤/정오/사선; beam만/diffuse만; 노출 off; 흡수율0/1

## 연결시험

SUR-03의 흡수일사 입력과 SRC-08을 혼합하지 않음

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

Qualified Exterior proof:137 ordered prefix records inferred from actual unique failed05 stop and exact dispatcher checks, plus33 directly successful later records.170 is not one successful run/one commonSource; Native9 overhang records remain excluded. Actual no-caster beam/bootstrap/Figure/sky/absorbed opaque functions; finite exposed/noSun/night/nonzero sky/beam controls. No SHDGSS positive overhang production admission.

실제 구현 기준은 Source59 `a2c512a246f73dc806805c6c4e277c2960fdda44`입니다. [최종 판정](../evidence/SRC-06-final-current-source-validation-60/summary.json)의 해당 카드별 범위·gate 근거와 현재 연결 결과를 참조합니다. 이전 단위 명령은 원래 근거에 보존되며 현재 바이너리 재실행으로 집계하지 않습니다.

직접 단위 근거: [SRC-06-canonical-warmup-multiplier-exterior-55](../evidence/SRC-06-canonical-warmup-multiplier-exterior-55/summary.json).

선택 본문의 정확한 행·원문 해시·역할과 직접 helper 근거는 해당 카드의 `evidence_by_scope.selected_source_extents` 및 최종증거팩의 본문 구간 원장에 기록합니다. 선언 행은 심볼 위치 식별에만 사용합니다.

과거 실패·미관측 값·제외 분기는 불변 근거에 남습니다. 수치 단위, 현재 연결, 보고값 정책, 물리 파일 직렬화의 판정을 합치지 않습니다.
