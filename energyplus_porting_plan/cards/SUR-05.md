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

## 종료 체크

- [ ] 원본 범위와 입출력·변경상태 계약 확정
- [ ] 단위시험 통과 및 실제 활성 분기 확인
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

## 현재 구현과 검증

`ordinary_surface_initialization/inside_driver.rs`는 실제 반복의 수렴 종료부를 공유 메서드로 호출합니다. 동일한 오류 소유자가 시각 진단, 13필드 recurring 행과 실제 카운터를 갱신합니다. 수렴 허용치·최소 반복·500회 한계와 진단 순서는 원본을 유지합니다.

비수렴 단위는 원본 종료부를 바이트 단위로 추출한 지정 분기 단위입니다. 3개 지속 수명에서 30회 평가하며 500/501 경계, warmup·sizing 상태, 반복 진단과 실제 저장 상태를 Source와 비교했습니다. 전체 실내 반복 함수를 호출한 단위로 계산하지 않습니다. Source의 계속/중단 반환 분류 중 원본 wrapper에 반환값이 없는 부분은 원문 제어 흐름에서 도출한 조건으로 구분합니다.

정상 반복의 iteration count·첫 두 상태·종료 상태와 30회마다의 대류 갱신·즉시 소비 순서는 같은 실제 A24 실행의 수동 관측으로 연결합니다. 수집기는 기본 비활성이고 계산·입력·기대값을 변경하지 않습니다. 이 연결 결과와 분기 단위 결과의 범위는 [선택 표면 단위 검증](../evidence/SRC-06-canonical-surface-unit-checkpoint-56/summary.json)에서 구분합니다. 기존 카드 체크와 게이트는 전체 64개 카드의 최종 판정에서 정리합니다.
