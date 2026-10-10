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

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [ ] 단위시험 통과 및 실제 활성 분기 확인
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋:
시험 명령:
증거 경로:
최대오차/RMSE/상태 불일치:
추가 검토할 helper:

## Source amendment before CTF-02 outputs (2026-10-10)

Root reviewed the full held Rust proposal and Native unit/production helpers; other-author full source peers accepted them. Original public arrays are copied only after the unchanged whole method returns. Selection is the literal source predicate from captured operands, not an invented branch flag. Private s0/e/s are neither read nor reconstructed. Unit39/42 and production15/195 are inherited input-only rows; production construction calls remain dynamic. Fresh CTF-02 coefficient policies must be frozen before its first helper execution. Actual Native data QA, existing Rust baseline, candidate comparisons, SUR-04/SUR-07 and production gates remain pending. Historical CTF-01 outputs are not CTF-02 coefficient evidence.

## Actual Original and unchanged Rust baseline (2026-10-10)

Genuine Native unit39/42 and production15/195 completed with actual process exit0. Exact PE/build review and actual OriginalDataQA passed before a fresh unchanged Rust build/run. Unit selected8/public42 and production selected6/public21 are factual observations; all other public records remain unpaired context. Existing Rust initialized all15 models and21 CTF-01 construction owners, with no CTF-02 public owner/API in current source. The actual legacy surface storage is retained as context; no CTF-02 numeric comparison is claimed. Evidence: `energyplus_porting_plan/evidence/CTF-02-original-baseline/summary.json`. Candidate implementation, comparisons, integration and production gates remain pending.
