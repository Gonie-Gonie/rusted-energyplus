# CTF-10 — 구성체별 생성 연결·역순 재사용

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: CTF-02, CTF-09
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/HeatBalanceManager.cc`  
심벌: `InitConductionTransferFunctions; ConstructionProps::calculateTransferFunction`  
대상 블록: 구성체 순회와 중복·역순 구성체의 계수 재사용 경로. 상세 수치계산은 CTF-01~09 호출.  
함께 읽을 선언/호출자: Construction.cc::calculateTransferFunction의 역순 재사용 블록  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/HeatBalanceManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/heat_balance/surface_manager.rs; initialization.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

구성체 목록·사용 여부·층순서

## 출력·변경 상태 계약

구성체 ID별 독립 생성 계수 캐시

## 호출 시점

본 계산 전

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

두 구성체·역순층·무사용 구성체; 캐시 invalidation

## 연결시험

생산 경로에서 EnergyPlus EIO를 읽지 않고 열수지 초기화

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

## Prospective selected CTF10 source scope (not an actual amendment)

Pinned public scope: Construction.cc146-194 reset/use/timestep,309-315 shared
loading-error return,405-485 original-ID reverse reuse,985-1077 allR/common
fixed19 public stores; whole HeatBalanceManager.cc6153-6202 Init call/aggregate/
report/fatal order. Existing CTF01-09 math is reused through real typed owners.

Proposed lanes remain distinct: existing39 explicit-member models42 direct whole
calls for unit public storage; fixedCON15/45 input guards with genuine public
loaders/use/timestep and actual whole Init once for production. Direct unit
caller flags/report prerequisites cannot be claimed as true Init ownership.

Freeze six separate zero atol/rtol profiles (four regular19-slot arrays, actual
CTFTimeStep and UValue), exact class/shape/signedzero and reset bits for the eight
source arrays. Original IDs/order/use/reverse provider/flags/counts/Init aggregate
read and report/fatal availability are exact. All19 slots are retained.

The Rust fixed owner is selected only within representable inclusive count0..18.
PublicCapacity is an honest Rust limitation, not an inferred Native guard/exit.
Fatal bypasses stores/aggregate; severe break still reaches stores. Missing,
unreached, excluded and unrepresentable owners cannot be PASS. Surface activation
/cache invalidation/history metadata and genuine Native whole-call observations
remain pending separate reviewed work. No gate or execution outcome is changed.

## Actual pre-output source scope

Root and another author reviewed held native-helper1aca57d1, passivea2765a61, public03/9adaabdf and readiness profilesfac618d6. Unit39/42 remain direct calls; fixed15 production retains13 loaders/45 guards and calls the true whole Init once. Per-call full-model before/after are unavailable; actual public snapshots and reached Init events own IDs, timestep and shared flags. Missing return remains unknown, with the actual outer Init owning exception outcome. Six public profiles are exact zero/class/shape/signed-zero; all12 arrays retain19 slots and source reset bits remain positive zero. Scope review only; unit/integration/production and complete-card gates remain unconfirmed until their actual evidence.

## Actual selected public storage comparison

Unit numeric948290/discrete3874496; fixed15 loader numeric8988/discrete57532; all mismatch0. Six public profiles and inherited handoffs have maximum absolute error/RMSE0. Actual returned owners, original IDs/order/use, reverse providers, reset arrays and final matching Init prefix are paired. Missing, unreturned, unavailable and unrepresentable observations receive no PASS.

Actual Clippy/fmt checks0; focused177 and workspace4797 tests passed with0 failed. Candidate/quality/comparison recorders bind Source88274ea251bca6736a28e09cc6474ae7c837655a04e972942638b603f82799fb. The first actual Clippy test clone warning is preserved; the accepted correction uses a borrowed one-element slice. Held comparer01 was never executed; fresh02 corrects the shared-error stage to genuine PostLoad for used constructions. Profiles, scientific arithmetic and required absence rules remain unchanged. Full actual observations and command/source identities are retained in evidence/CTF-10-unit/{manifest,summary}.json.

Selected public storage verification does not complete the unit gate: generated-cache invalidation remains pending. Surface/history activation, whole Init reporting/final-fatal backend and integration/production/full-card gates remain pending. Fixed6 ordinary CLI and precise Original retirement will be recorded separately.

## Actual fixed six ordinary CLI executions

A-24H, A-72H, B-NOLIMIT-24H, B-FLOW-24H, B-CAPACITY-24H and B-BOTH-24H all exit0 on the same final10 source. Each actual source-order gate matches; hours override is absent and compare_oracle=false. All45 original input/weather/metadata guards remain unchanged. This establishes ordinary run regression evidence only; no numerical or integration/production/full-card PASS is inferred. Evidence: evidence/CTF-10-production-cli/{manifest,summary}.json.

## Actual exact-pair Original retirement

The two new10 linked debug Originals were removed through the reviewed exact LiteralPath mechanism after the committed comparison/quality/CLI proof. Actual removed bytes2612693985; both paths are absent. Prior46 protected artifacts, both10 runtime derivatives and631 core objects remain unchanged;12 absent registered paths and all prior04/06/07/08/09 Original absences remain unchanged. No registry/history rewrite or scientific rerun. Evidence: evidence/CTF-10-retention/{manifest,summary}.json. Integration/production/full-card gates remain pending.

Completed10 ordinary CLI traces (6 files) were transparently compressed with NTFS LZX. Their paths, lengths and SHA256 bytes are unchanged before/after; no trace was deleted or moved. The actual operation and command receipt are included in this selected retention archive.

## Actual standalone surface coefficient binding

The public heat_balance::ctf_surface_binding module borrows actual ordered public owners and typed surface/construction identities. It preserves all12x19 coefficient bits, counts, timestep and U; its exact generation key includes material/caller/order/route/global dependencies and exposes uninitialized history requirements. It has no ordinary runtime, producer cache, weather or thermal-history caller. Seven contract tests and the focused/workspace suites, Clippy and format check were executed on the source snapshot recorded in evidence/CTF-10-surface-binding/summary.json. The prior Native public-storage comparisons were not rerun. Actual producer invalidation, history activation, Native consumer comparison and all three CTF10 gates remain pending.

## Protected Original storage maintenance

Four older CTF01/02 protected Original executables were transparently compressed with NTFS LZX. Before/after logical bytes, SHA256 and original paths match exactly; none were deleted, moved or executed. Actual command, logs and free-space observations are archived in evidence/CTF-10-storage-maintenance. This is storage maintenance and changes no scientific gate.

## Prospective ordinary surface lifecycle capture

The held passive consumer03 observes the complete unchanged A-24H input and weather through genuine EnergyPlusPgm, with no duration override, design-day-only switch, warmup shortcut or seeded engine state. All actual procedure order, flags, owner identities, allocations and full history bounds are retained at51 existing passive sites. Entry observations include scratch/report/counter arrays; absence remains absence. A prospective4GiB full-file QA resource bound rejects excess and never truncates the producer.

Root and another author completed cumulative FULL collector/fork/recipe Source reviews. Actual build, strict PE review, capture and factual OriginalDataQA remain pending. Captured Native operands are permitted only for future same-input direct-unit tests; ordinary Rust production must generate its own coefficients/state. Standalone thermal-history Source remains unapplied until genuine same-input observations are reviewed. No unit/integration/production gate is changed by this scope.

The first consumer compile stopped before any engine execution on unused Windows main argc/argv under unchanged Werror. Reviewed consumer04 adds only explicit void casts for those two parameters. All51 callbacks, collector, Original HBSM, recipe, inputs and prospective4GiB QA bound remain unchanged. A fresh scope/freeze02 binds this build-only successor and retains the failed attempt; actual engine capture remains pending.

Actual ordinary A-24H Original capture and independent full-event custody review passed: 404878 attempted/retained events, zero omissions, all six lifecycle procedures reached and genuine process exit0. The complete962546811-byte observation is retained and losslessly archived with verified decompressed SHA. This supplies actual observation custody, not thermal-history numerical or integration PASS. Exact pre2323 input, complete returned allocated history/scratch/report owners and current referenced coefficients require the reviewed minimal05 augmentation. See evidence/CTF-10-surface-consumer-original-baseline/summary.json.
