# CTF-04 — 상태공간 A/B/C/D 행렬 구성

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 신규·보완
- 선행 카드: CTF-03
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Construction.cc`  
심벌: `ConstructionProps::calculateTransferFunction`  
대상 블록: 1D 무내부열원 경로의 AMat/BMat/CMat/DMat 조립 및 층 경계 처리.  
함께 읽을 선언/호출자: Construction.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Construction.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

신규 계수 생성 모듈 제안

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

절점 배치·열물성·저항층 경계

## 출력·변경 상태 계약

연속시간 상태공간 행렬

## 호출 시점

구성체 초기화·재시도

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

단층/다층 행렬 원소 비교; 역방향 층; 열저항 극단

## 연결시험

CTF-05~08에 오차·차원·단위 그대로 전달

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋:  
시험 명령:  
증거 경로:  
최대오차/RMSE/상태 불일치:  
추가 검토할 helper:  

## Source amendment before CTF-04 outputs (2026-10-11)

Pinned Construction.cc638-648 allocates actual AMat/IdenMatrix and produces identity;661-728 assembles selected no-source1D AMat/B3/C2/D2. A separate passive callback after728 copies every actual attempt. Original whole-method arithmetic/branches and old four checkpoints/counts remain unchanged. Root FULL and other-author FULL source reviews accepted the passive transport, whole Native helpers, input writer, PE reader, OriginalDataQA, pure matrix/identity body, two-owner cache ingress and candidate observers. Unchanged accepted predecessor bodies are authenticated by exact before/diff correspondence.

Numerical selection is first actual ordinal1 only, with genuine initial619 dt/history, same converted prefix/Nodes/dx/rcmax/selectors/construction/caller. Actual source-produced IdenMatrix is required; no identity/offband matrix answers are synthesized by observers/readers. All later Native visits, bounds, failure events and omissions are retained as context/unpaired/noPASS until a genuine Rust retry driver exists. Native retry dt/history/counts are never Rust operands. Missing required first capture or copy/order/shape/identity integrity fails; a later visit cannot replace it.

Seven separate AMat/IdenMatrix/BMat/CMat/DMat/assembly_CTFTimeStep/caller_TimeStepZone profiles are frozen at atol=rtol=0 with exact class, actual shape and signedzero. Initial3 and preprocessing18 remain unchanged separate handoffs. Unit39/42 and fixed15/195 reuse only original literal/member/IDF input rows; production construction obligations remain dynamic. Actual source-return/fatal/exception/OS outcomes remain context and never imply missing values or PASS.

New CTF04 linked originals carry prospective-debug-original intent from the start: actual identity/strict PE and both files verified before current helper execution, retained through PE/current execution, any future removal only via separately Root-authorized actual existing retention lifecycle record. Earlier live refs remain literal; no deletion occurs here. Scope passed; unit/integration/production and dependency completion remain pending. Actual CTF04 configure/compile/helper, candidate application and comparisons have not run.


## Actual Original data and unchanged Rust baseline

Both genuine CTF04 Native helpers and independent Original DataQA completed with actual exit0 before fresh unchanged CTF03 Cargo/production execution. Unit39/42 retained41 assembly visits: first25 selected, later16 unpaired/noPASS, no matrix omissions/failures. Fixed15 production/195 loader phases/21 CTF calls retained15 selected first visits, no omissions/failures. Actual preservation kept36 protected artifacts,631 present core objects,12 registered absent objects and all679 old compile rows (actual685); both independent PE reviews passed. Originals remain retained under the prospective04 intent.

The fresh unchanged Rust baseline returned on all15 same authentic converted input-only models and exposed21 existing CTF03 owners. Exact nine new04 source paths are absent from disk/captured source snapshot. The first-assembly gap is structural unavailability; numerical mismatch count is null. Two unpublished derived summary field mistakes were corrected against actual producer manifests/owner arrays, with both earlier summaries retained; actual inputs/build/execution/results were unchanged. Reviewed candidate remains unapplied.

Evidence: `evidence/CTF-04-original-baseline/summary.json` and selected content-addressed manifest. Binary/map payloads and full transitive replay closure are not included. No numerical or remaining-gate PASS is claimed.


## Actual selected first assembly comparisons

Reviewed14 Rust source files applied after genuine OriginalQA and unchanged baseline; Rustfmt2024 only, no post-format arithmetic amendments. Fresh actual Cargo build followed all quality checks. Unit25 first owners:39,475 primary numeric and90,388 discrete comparisons, zero mismatches. Fixed15 initialized loader first owners:885 primary numeric and5,401 discrete comparisons, zero mismatches. All7 separate frozen profiles have max absolute error0/RMSE0/nonfinite errors0. Shape, class, signed-zero, integer/ID/order/caller and first-attempt stamps are exact.

Inherited preprocessing/initial handoff copies remain separate metrics. Native unit later16 attempts remain retained unpaired/noPASS. Unavailable contexts54 unit/18 production were not counted as PASS. Focused13 and full workspace4,679 tests passed (0failed/0ignored;39 workspace binaries); Clippy and fmt passed. Both genuine candidate executions and strict comparisons ended0 with unchanged captured source bytes. Evidence: `evidence/CTF-04-unit/summary.json`.

Only the selected first-assembly unit gate is passed. Fixed6 CLI after this application, retry/whole coefficient handoff, integration/production gates and predecessor completion remain pending.


## Actual fixed-six production CLI after selected assembly port

Fresh actual ep_cli/eplus-rs Cargo build followed selected04 comparisons and quality. A24/A72/B-NOLIMIT/B-FLOW/B-CAPACITY/B-BOTH all ended0 with the same captured source bytes and Cargo binary. Compatibility/partial-deny/full-trace flags, no hours override/oracle comparison, all45 input guards and source-order matches are verified. Fixture-demand-injection is actual null for both A runs and false for four B runs; null is not reinterpreted as a boolean observation.

Evidence: `evidence/CTF-04-production-cli/summary.json`. This verifies actual CLI execution after this application; it does not certify whole CTF coefficient/retry handoff or complete integration/production gates. The earlier selected-unit summary retains its historical pre-CLI flag literally.
