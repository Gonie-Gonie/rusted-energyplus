# RAD-01 — 근사 view-factor 행렬

- 그룹: 06 실내 장파복사
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: GEO-02
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/HeatBalanceIntRadExchange.cc`  
심벌: `CalcApproximateViewFactors`  
대상 블록: 대상 단일 불투명 enclosure의 가시성·면적비 행렬만.  
함께 읽을 선언/호출자: HeatBalanceIntRadExchange.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/HeatBalanceIntRadExchange.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

heat_balance/radiation.rs

구현 커밋: `7356431a3fb0243af2b8c6427817ccda84c7050f`. 같은 입력의 원본·Rust 단위 비교 및 품질 검사를 통과했다.

## 입력 계약

표면면적[m²], 방위/경사·표면종류

## 출력·변경 상태 계약

원본 근사 F 행렬

## 호출 시점

enclosure 초기화

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

동일/비대칭 면적; 바닥↔바닥 제외; 면 정렬 순서

## 연결시험

RAD-02의 행렬 인덱스와 행/열 의미 보존

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 검증 상태

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

원본 전체 함수 1406–1519행과 선언 109–116행을 기준으로, 명시한 유한 불투명 geometry 21개를 검증했다. 면적 누적 순서, 엄격한 방위·경사 경계, 바닥 간 제외, 양수 면적의 나눗셈, F(j,i) 축 의미와 경고 후 계속 진행을 보존한다. ZoneArea와 F는 각각 atol=0, rtol=0이며 값 분류·배열 모양·부호 있는 0을 비교한다. 입력 비트·식별자·순서도 정확히 비교한다.

원본과 Rust가 모두 21회 반환했다. 수치 378개와 이산 항목 2115개에서 불일치 0, 최대 절대오차와 RMSE 0이다. 경고 11개의 식별자·개수·순서도 일치했다. 미관찰 항목을 통과로 계산하지 않았다. Native 전역 진단 백엔드의 문구는 별도 미검증 항목이다.

fmt·Clippy 통과, 열수지 203개 및 workspace 4808개 테스트 통과. 고정한 일반 CLI 6종도 모두 exit0과 실제 source-order 검사를 통과했다. 입력·날씨·메타데이터 45개 해시는 유지됐다.

남은 연결 범위는 GEO-02가 실제 생성한 enclosure 표면 순서와 raw ZoneArea/F/경고 소유권, RAD-02 보정 행렬과의 축·소유권 연결, 보정된 solar F 재사용 및 radiant ScriptF 호출 순서, 생산 초기화 상태의 지속·무효화 시점이다. 일반 CLI 성공만으로 이 항목들을 통과 처리하지 않는다.

증거: `evidence/RAD-01-unit/summary.json`, `evidence/RAD-01-production-cli/summary.json`, `evidence/RAD-01-unit-gate/summary.json`. 과거 빌드 실패와 재현 기록은 기존 증거에 보존되어 있다.
