# SUR-03 — 외기 접촉면의 외표면온도

- 그룹: 07 표면 열수지
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: SUR-02, SRC-04, SRC-07, SRC-08
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/HeatBalanceSurfaceManager.cc`  
심벌: `CalcOutsideSurfTemp`  
대상 블록: 불투명 CTF·무내부열원·무이동단열 분기; quick-conduction 조건을 포함.  
함께 읽을 선언/호출자: HeatBalanceSurfaceManager.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/HeatBalanceSurfaceManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 현재 Source 경로

`crates/ep_runtime/src/heat_balance/ordinary_surface_initialization/outside_driver.rs`와 같은 모듈의 `native_gap_cohort.rs`가 실제 계산과 단위 호출을 소유합니다.

## 입력 계약

CTF 현재계수·이력항, 흡수일사·대류/장파 조건, 필요한 실내 결합항

## 출력·변경 상태 계약

외표면온도[°C], 외부 열수지 중간항

## 호출 시점

실내 표면 반복 전

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

quick-conduction 경계 양쪽; steady/mass; 일사 on/off

## 연결시험

SUR-04에서 사용한 외표면온도와 원본의 고정/갱신 시점 비교

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

`ordinary_surface_initialization/outside_driver.rs`의 실제 외표면 계산을 같은 소유자에서 호출합니다. 선택한 무내부열원·무이동단열 분기에 대해 원본 전체 함수와 Source를 7개 수명, 24회 호출로 비교했습니다. quick-conduction 경계 0.01의 아래·동일·위, 정상/축열, 일사 유무와 반복 호출을 포함하며, 관측한 상태와 반환값은 binary64 원시 비트로 비교합니다.

CTF 캐시의 실제 생성 뒤 선언된 세 계수와 이력은 이 좁은 소비자 단위의 입력입니다. 이 결과만으로 전체 CTF 생성 과정이나 외표면 부모 전체를 검증했다고 보지 않습니다. SUR-04에 전달되는 값의 시점과 표면 반복 연결은 실제 A24 관측으로 별도 확인합니다.

실행 영수증, 입력·출력 원문, 범위와 보존된 최초 컴파일 실패는 [선택 표면 단위 검증](../evidence/SRC-06-canonical-surface-unit-checkpoint-56/summary.json)에 모읍니다. 기존 카드 체크와 게이트는 전체 64개 카드의 최종 판정에서 정리합니다.
