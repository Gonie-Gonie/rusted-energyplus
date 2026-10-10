# CTF-06 — 행렬 지수 계산

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 신규·보완
- 선행 카드: CTF-04
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Construction.cc`  
심벌: `ConstructionProps::calculateExponentialMatrix`  
대상 블록: 메서드 전체: scaling/급수/종료 순서를 포함. 임의 라이브러리 치환은 별도 수치정책 필요.  
함께 읽을 선언/호출자: Construction.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Construction.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

신규 계수 생성 모듈 제안

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

A, CTF timestep

## 출력·변경 상태 계약

상태전이 행렬 exp(A·dt)와 작업상태

## 호출 시점

계수 생성·재시도

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

dt 변화; 대각행렬/다층 실제행렬; EP 원소 및 종료 횟수

## 연결시험

CTF-07/08에 연결

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

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


## Reviewed combined first-method scope before observation

Construction.cc1105-1373 whole exponential: literal norm/log/cast/scaling/power/sentinel/Backup/squaring order; actual caller884 before inverse887. First invocation1/assembly1 under selected no-source1D context, actual current CTFTimeStep and caller history. Final workspaces and all genuine controls retained.

Actual same first assembly AMat/IdenMatrix/rcmax/context and current initial619 dt/history; no Native answers, retry states or generated identity. Exponential precedes inverse; inverse always uses original AMat.

Actual AExp/final_AMat1/final_AMato/final_AMatN/AMatRowNormMax/fact/CheckVal with exact typed shape/class/signedzero/control/caller stamps. All later Native invocations/partial failures/omissions remain retained unpaired/noPASS.

Primary separate profiles: AExp, final_AMat1, final_AMato, final_AMatN, AMatRowNormMax, fact, CheckVal. Each prospectively fixed atol0.0/rtol0.0, exact class/shape/signedzero. Seven inherited assembly profiles, three initial profiles and eighteen preprocessing profiles remain separate handoff checks. Native actual entry/stage/control/final/caller-return ledgers are retained without caps or reconstruction. Missing first owner on either or both sides is a comparison failure; later visits and unavailable contexts are never PASS.

Same literal unit39/42 and fixed production15/195/45 input guards only. Both CTF05 and CTF06 real card checks are required before input freeze. Root FULL and independent other-author FULL Source review preceded this amendment. Future actual build/PE/OriginalDataQA must precede fresh unchanged Rust baseline and candidate application. No actual build, engine output, numeric equivalence or remaining-gate PASS is claimed. New06 debug originals follow forward-only prospective retention until a separate actual approved cleanup; earlier histories remain literal.
