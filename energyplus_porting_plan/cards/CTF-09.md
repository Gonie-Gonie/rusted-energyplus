# CTF-09 — 계수 안정성 검사·timestep 재시도

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 신규·보완
- 선행 카드: CTF-08
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Construction.cc`  
심벌: `ConstructionProps::calculateTransferFunction`  
대상 블록: 최종 계수 뒤 CTF 합 검증, 항수·시간증분 재시도 및 종료/오류 블록.  
함께 읽을 선언/호출자: Construction.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Construction.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

신규 계수 생성 모듈 제안

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

계수·차수·CTF timestep 후보·허용 한계

## 출력·변경 상태 계약

채택 계수, CTFTimeStep, NumHistories, 종료/실패 상태

## 호출 시점

계수 생성 종료

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

첫 시도 성공/시간증분 증가/최대치 실패; 경계 직전·직후

## 연결시험

SUR-06에서 실제 NumHistories 소비; 단위 W와 상대오차 혼용 금지

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [ ] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋:  
시험 명령:  
증거 경로:  
최대오차/RMSE/상태 불일치:  
추가 검토할 helper:  

## Prospective selected 1D retry scope (Root amendment pending)

Construction.cc638-981 preserves once-only allocations/Iden construction followed by each actual repeated1D assignment and exponential884 -> inverse887 -> Gamma890 -> finalCoefficients894 -> caller903-981. The initial619 timestep/history remain distinct from caller TimeStepZone; Nodes/dx are not recalculated during retry. One original assembly storage owner is consumed and retained; each Gamma invocation is fresh.

The eight new retry profiles are pre_AMat, pre_IdenMatrix, assignedAbsSumXi, assignedAbsSumYi, assignedAbsSumZi, BiggestSum, pre_CTFTimeStep and post_CTFTimeStep. Every profile has atol=rtol=0 with exact class, shape and signed zero. Actual integer counts/ordinals, owner identities/order, branch flags and reached diagnostic indices/level/text are exact. Existing preprocessing18/initial3/assembly7/exponential7/inverse2/Gamma3/private7 profiles remain literal; the new selection uses separately named all-visit handoffs and does not broaden historical first-only claims.

Sum owners exist only after the actual919-930 branch. The >18 term branch skips those owners; ratio expressions have no stored Native locals and are never observed/recomputed by the collector. Fatal939 is recorded before actual fatal unwinding; the seven-hour severe block948-977 can be reached even when convergence is true. All reached visits, partial records, copy masks, failure and omission ledgers remain required, with no record cap or invented method return.

LoopExit is distinct from whole calculateTransferFunction return. Severe break may continue into public SI stores1024-1077; public19 capacity, SI conversion/stores, reverse/2D/internal-source branches and SUR handoff remain outside this selected retry proposal. No fatal, severe, partial, uninvoked or unavailable branch receives PASS. Actual Native diagnostics remain global occupied message records; Rust selected call intents are not substituted for that backend.

Same literal39 unit models/42 whole calls and fixed15 production models/195 loader phases/45 guards are inherited without new expected outcomes. Current09 amendment/check authority must be real and separate from immutable historical_prior_08_authority, including nested historical07 ancestry. This draft changes no canonical gate and claims no numerical, integration, production or complete-card PASS. Source/full peer, actual amendment/check, fresh freeze, protected build/derive/PE and structural Original review remain Root actions.

## Actual selected retry-visit comparison

Same frozen inputs and strict zero profiles: unit numeric945014/discrete3840604, fixed15 loader numeric7350/discrete40251; all mismatch0. Eight primary retry profiles and inherited assembly/method/Gamma/private-final handoffs have max absolute error/RMSE0. All actual selected visit owners are paired by real call/attempt identity; partial, absent and unavailable observations receive no PASS.

Actual Clippy/fmt checks0; focused162 and workspace4782 tests passed with0 failed. All candidate/quality/comparison recorders bind Sourcef280b83b219794f7cca1cfd57a3e28d6443cc83c976569bf042a2034110c3ced. The genuine first Clippy missing-docs and second macro recursion-limit failures are preserved. Corrections add eight field descriptions and a single unit-observer crate macro expansion limit; no math or input changes. Full actual scientific bytes, sources and commands are retained in evidence/CTF-09-unit/{manifest,summary}.json.

Only the selected unit gate passes. Whole Native public SI19 stores, reports/final fatal backend, SUR/cache handoff and integration/production/full-card gates remain pending. Fixed6 CLI and precise Original retirement will be recorded separately.
