# CTF-07 — Gamma 행렬 계산

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 신규·보완
- 선행 카드: CTF-05, CTF-06
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Construction.cc`  
심벌: `ConstructionProps::calculateGammas`  
대상 블록: 메서드 전체. 역행렬·상태전이·B 및 시간증분을 외부 계약으로 공급.  
함께 읽을 선언/호출자: Construction.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Construction.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

신규 계수 생성 모듈 제안

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

A^-1, exp(A·dt), B, dt 및 필요한 작업배열

## 출력·변경 상태 계약

Gamma 계열 행렬과 변경 작업상태

## 호출 시점

계수 생성

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

동일 EP 행렬 입력에 대한 원소별 비교; dt 변화

## 연결시험

CTF-08 최종 계수에 독립 테스트와 동일 데이터 전달

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

## Source-only first Gamma scope fixed before outputs

Pinned Original Construction.cc1496-1589 and caller875-894 are fully reviewed.
Original executes exponential884, inverse887, then Gamma890. Selected first
Gamma invocation1 and assembly1 borrow the same actual AExp, AInv, IdenMatrix,
BMat, rcmax, dt/history and source selector context; no replay or answer input.
The selected route remains massive nonreversed 1D with no source/sink. Original
Gamma loops/evaluation order, dt division and third-row IEEE results are retained.
ATemp is observed before its deallocation; actual Gamma1/Gamma2 return markers
remain distinct from private checkpoints. Later visits, omissions and copy failures
are all retained; unavailable or later owners do not count as passing pairs.

Gamma1/Gamma2 (3-by-rcmax) and ATemp (rcmax-by-rcmax) have separate strict profiles:
atol=0, rtol=0, exact class/shape/signed zero. Prior exponential7/inverse2,
assembly7, initial3 and preprocessing18 policies remain separate unchanged handoffs.
Inputs stay literal39 unit models/42 declared whole calls and15 production IDFs/
195 declared loader phases with45 scope guards; no expected exits or arrays.

Source authority: .runtime/porting/CTF-07/native-helper-prep-01/manifest.json
SHA256 67ffbd0ee248b5f5bed72797539af3cbd19dd304b174f15b7131098fecb50c3c;
passive observer02 d61910f07155cfe5fcdf165b0178c5b7c0810300b01a073b66a70987cf4e1f06;
projection policy4368c7364c86e1227864a18777d8109d8a31afa0ac0e0411e9559bf7ed6f76d6.
Both new linked Originals require actual PE/provider/runtime proof before execution;
any later removal requires a separate actual retention record. Old retention refs
remain literal. Scope review alone passes; unit/integration/production remain
pending. Full retry, final coefficients, SI19 stores and surface handoff are pending.

## Actual selected first Gamma validation

- Applied Source `293f3e45e5c69f264be9e1becbba233b6a785e44d6880330cdd8301af0133f0e`; actual Cargo artifacts and execution custody verified.
- Original unit 25 selected first pairs: 180219 numeric and 1130134 discrete comparisons, zero mismatches.
- Fixed 15 loader pairs: 4005 numeric and 30477 discrete comparisons, zero mismatches.
- Gamma1, Gamma2 and ATemp maximum absolute error/RMSE are zero; frozen zero tolerances and exact classes/shapes/signed zero are unchanged.
- Focused Gamma tests 110 passed; workspace tests 4726 passed; clippy with denied warnings and format check exited zero.
- Actual initial clippy failure and production comparer diagnostic label failure are preserved. The scoped lint expectation and diagnostic-only repair received independent Source review before execution.
- [Complete selected evidence](../evidence/CTF-07-unit/summary.json) and [actual plan validation](../evidence/CTF-07-unit/validation.json).
- Native 16 later unit invocations and unavailable 105 unit/36 production observations remain unpaired/noPASS.
- Whole retry orchestration, final coefficients, public SI19 stores, cache handoff and complete integration/production gates remain pending.
- Fixed six ordinary CLI executions and precise two-Original retirement are recorded separately.

## Actual six ordinary CLI executions

A-24H, A-72H, B-NOLIMIT-24H, B-FLOW-24H, B-CAPACITY-24H and B-BOTH-24H all exited zero at Source293f3e45. All 45 fixed input hashes remained unchanged. Actual Cargo artifacts were used; compatibility/partial deny/full trace, no hours override, no oracle comparison. [Evidence](../evidence/CTF-07-production-cli/summary.json). Complete integration and production gates remain pending.
