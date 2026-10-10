# CTF-08 — 최종 CTF 계수와 차수

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 신규·보완
- 선행 카드: CTF-07
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Construction.cc`  
심벌: `ConstructionProps::calculateFinalCoefficients`  
대상 블록: 메서드의 1D 무내부열원 결과 생성 경로. 원본 반복·항 절단 순서 유지.  
함께 읽을 선언/호출자: Construction.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Construction.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

신규 계수 생성 모듈 제안

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

상태전이·Gamma·C/D 및 작업배열

## 출력·변경 상태 계약

CTFOutside/Cross/Inside/Flux, 차수와 관련 상태

## 호출 시점

계수 생성

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

각 계수 원소와 부호·차수 비교; 정상상태 전달성 확인

## 연결시험

EIO 제공 계수를 사용하지 않고 SUR-02~07 구동

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

## Source-only selected private coefficient scope

Construction.cc1591-1906 and caller894 are fixed before any CTF08 helper build/run or scientific output. Only actual invocation1/assembly1 on the massive nonreversed 1D no-source path is selected. Actual preceding exponential884/inverse887/Gamma890 returns and initial619 context are required.

Seven new primary owners are s0, full allocated s/e including initialized tails, mutated Gamma1 and private PhiR0/Rnew/Rold immediately before1903-1905 deallocation. NumCTFTerms, inum and CTFConvrg compare exactly. Every profile uses zero absolute/relative tolerance with exact class/shape/signed zero. Existing preprocessing18/initial3/assembly7/method9/Gamma3 handoffs retain their own fixed profiles. No value, expected exit or outcome array is authored from outputs.

All real later visits, omissions, independent copy masks and six failure kinds must be retained and validated; later observations receive no PASS. R workspaces/private controls are unavailable at return and are not reconstructed. Same literal39 unit models/42 whole calls and original15 production models/195 loader phases/45 input guards are retained. No identity from prior07 frozen docs is rebound to live current docs.

Whole retry orchestration, final public SI19 stores and SUR/cache handoff remain pending; this selected scope establishes no unit, integration, production or complete-card PASS. The two new08 linked Originals are retained through PE and real helper review, then require separate precise actual retirement.

## Actual selected first private coefficient comparison

Same frozen inputs and strict zero profiles: unit numeric248178/discrete1574736, fixed15 loader numeric6510/discrete46896, all mismatch0. Seven primary full private owners have max absolute error/RMSE0, including initialized tails, exact class/shape/signed zero and integer/control state. Selected first owners25/15; Native later16 unit visits remain retained and unpaired. Unavailable122/42 records receive no PASS.

Actual Cargo clippy/fmt checks0; focused132 and workspace4748 tests passed with0 failed/ignored. All actual candidate/quality/comparison recorders bind Source761312a84eda41f616c5002808815011dcb6b46fb48c1ee6c1697cbdcb43fe6c. Full scientific bytes, source/commands and real OriginalDataQA baseline are preserved in evidence/CTF-08-unit/{manifest,summary}.json.

Only the selected unit gate passes. Whole retry, final public SI19 stores, SUR/cache connection and integration/production/full-card gates remain pending. Fixed6 CLI and precise Original retirement will be recorded separately.

## Actual fixed six ordinary CLI executions

A-24H, A-72H, B-NOLIMIT-24H, B-FLOW-24H, B-CAPACITY-24H and B-BOTH-24H all exit0 on the same final08 source. Each actual source-order gate matches; hours override is absent and compare_oracle=false. All45 original input/weather/metadata guards remain unchanged. This establishes ordinary run regression evidence only; no numerical or integration/production/full-card PASS is inferred. Evidence: evidence/CTF-08-production-cli/{manifest,summary}.json.
