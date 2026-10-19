# CTF-02 — 전부 저항층인 경우의 CTF

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: CTF-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Construction.cc`
심벌: `ConstructionProps::calculateTransferFunction`
대상 블록: Construction.cc985-1103 selected all-resistive branch and public storage; literal dispatch at405 and else985, resets146-164, preprocessor error return263-315. Unchanged whole Native call returns before public observation. Massive/source-sink/2D/reverse paths are separate.
함께 읽을 선언/호출자: Construction.hh
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Construction.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/heat_balance/surface_manager/ctf_all_resistive.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

Actual CTF-01 converted active prefix, assigned English cnd, LayersInConstruct/NumResLayers/ErrorsFound/IsUsedCTF and actual caller TimeStepZone. No independent SI total-R resummation, area operand or new finite/resistance rejection.

## 출력·변경 상태 계약

Four actual public19-slot CTFOutside/CTFCross/CTFInside/CTFFlux arrays, CTFTimeStep, UValue, NumHistories=1 and NumCTFTerms=1 for reached selected all-resistive regular branch, retaining source signed zeros and unselected/early-return availability.

## 호출 시점

구성체 초기화

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

Same genuine39-model/42-whole-call inputs and15 unchanged production IDFs/195 loader phases; classify literal LayersInConstruct > NumResLayers dispatch, fresh6 zero/class/shape/signedzero profiles, all19 slots, timestep, selected counts, early-return/unavailable cases. Source classification and IEEE behavior retained without invented total-R guards.

## 연결시험

Actual CTF-01 converted-prefix -> construction cache -> ordered state -> surface coefficient copy; SUR-04 use and SUR-07 heat-balance sign, caller timestep and fresh-state/empty-override handoff remain required.

## 정밀도 정책

Six distinct public coefficient/dt/U profiles atol=rtol=0, exact class/shape/signedzero and all19 array slots; copied inputs/counts/selection/IDs exact. Inherited18 preprocessing profiles remain unchanged separate context. Required reached absence on either or both sides fails; unselected/unreached/exception/stopped states never numerical PASS. No post-output tolerance changes.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 최종 네 gate 근거

| Gate | 범위별 Root 판정 |
|---|---|
| scope_review | A: 통과 / B: 통과 |
| unit_gate | A: 통과 / B: 통과 |
| integration_gate | A: 통과 / B: 통과 |
| production_gate | A: 통과 / B: 통과 |

985–1103 all-resistive/public storage; six exact profiles/all19. Whole arithmetic module raw-identical to executed02. Massive/reverse/retry effects are separate.

실제 구현 기준은 Source59 `a2c512a246f73dc806805c6c4e277c2960fdda44`입니다. [최종 판정](../evidence/SRC-06-final-current-source-validation-60/summary.json)의 해당 카드별 범위·gate 근거와 현재 연결 결과를 참조합니다. 이전 단위 명령은 원래 근거에 보존되며 현재 바이너리 재실행으로 집계하지 않습니다.

직접 단위 근거: [CTF-02-unit](../evidence/CTF-02-unit/summary.json).

선택 본문의 정확한 행·원문 해시·역할과 직접 helper 근거는 해당 카드의 `evidence_by_scope.selected_source_extents` 및 최종증거팩의 본문 구간 원장에 기록합니다. 선언 행은 심볼 위치 식별에만 사용합니다.

과거 실패·미관측 값·제외 분기는 불변 근거에 남습니다. 수치 단위, 현재 연결, 보고값 정책, 물리 파일 직렬화의 판정을 합치지 않습니다.
