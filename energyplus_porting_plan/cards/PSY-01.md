# PSY-01 — 기본 습공기 물성

- 그룹: 03 스케줄·물성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: CON-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Psychrometrics.hh`  
심벌: `PsyCpAirFnW; PsyRhoAirFnPbTdbW; PsyHFnTdbW; PsyTdbFnHW`  
대상 블록: 호출되는 inline 함수 본문과 상수·하한 처리만. 파일 I/O·경고 집계는 별도.  
함께 읽을 선언/호출자: Psychrometrics.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Psychrometrics.hh

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

착수 시 확인한 정확한 범위와 파일·행 해시는 `contracts/PSY-01-source.json`에 고정했다.
정상 rho의 state/constexpr overload는 512–547/549–574행, H는 648–666행,
Cp는 679–716행, Tdb 역산은 743–762행이다. fast rho/H/Cp의
576–591/668–677/718–741행은 추가 단위 경계 검증에 사용한다.
원본 Cp에는 정상/fast 각각의 독립된 `dwSave/cpaSave=-100` 캐시가 있으며,
하한 적용 전에 원래 W의 동등성을 검사한다. 이 상태와 같은 실행 스레드의
환경 간 유지가 검증 대상이다. 원본의 스레드 간 data race는 포함하지 않는다.

실제 H 연결의 계산 순서를 바로잡으면서 기존 소비 함수의 습도비 역산도
`PsyWFnTdbH` 962–998행의 J/kg 계산 순서로 연결한다. 이 직접 소비 helper의
추가 단위 검증은 PSY-02 완료 판정과 구별한다. 공급온도 반환값의 실제 소비는
`PurchasedAirManager.cc` 2191–2204행의 용량 제한 분기를 기준으로 검토하며,
CSHR 소비 분기 2213–2227행의 기존 회귀 검사도 유지한다. CON-01의 허용 입력은
변경하지 않는다. 정확한 제외 범위와 reference compile 조건은 source 계약에 기록한다.

## 재사용 후보

crates/ep_runtime/src/psychrometrics

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

T[°C], W[kgWater/kgDryAir], P[Pa]

## 출력·변경 상태 계약

cp[J/(kg·K)], rho[kg/m³], h[J/kg], 역산 T[°C]

## 호출 시점

호출 시

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

건조/습윤·0°C 부근·기압 변화; h↔T 왕복; 음수 W 처리

## 연결시험

ZON-02와 HVAC-04/05에서 같은 입력 tuple과 호출 순서

기존 실행의 실제 kernel 인수·반환값·Cp 상태·호출자·시간 context를 관찰하고
동일 순서로 원본 C++ body를 재생한다. 물리 계산과 사후 snapshot 재검산 호출을
구분한다. 압축 dictionary와 순서 ID는 모든 호출을 보존해야 하며, 누락된 suffix는
검증 완료로 판정하지 않는다. 실제 용량 제한 분기의 Tdb 반환값이 공급온도에
사용되는 시험과 Full/Summary 계산 결과의 동일성도 확인한다. 이 helper 연결
검증은 후속 존/HVAC 조립과 SYS 전체 호출 순서의 완료를 의미하지 않는다.

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
