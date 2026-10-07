# PSY-02 — 포화·습구·습공기비 역산

- 그룹: 03 스케줄·물성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: PSY-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Psychrometrics.cc`  
심벌: `PsyTsatFnHPb; PsyWFnTdbRhPb; PsyWFnTdbH; PsyTwbFnTdbWPb`  
대상 블록: 현재 기상/냉방 분기가 호출하는 함수 및 직접 호출되는 포화압력 helper만. .hh inline/cached wrapper도 제한적으로 동봉.  
함께 읽을 선언/호출자: Psychrometrics.hh의 해당 wrapper 및 호출 상수  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Psychrometrics.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/psychrometrics

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

T/W/P/h, 역산 초기조건·허용오차

## 출력·변경 상태 계약

포화온도·습구·W 및 경계 처리

## 호출 시점

기상 및 설비 계산

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

포화선 안/밖; 결빙 부근; 하한; 수렴실패 처리

## 연결시험

HVAC-05 포화 보정 및 CLK-04 기상상태 연결

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [x] 상태·시간·호출순서를 포함한 연결시험 통과
- [x] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋: `e7eec6e030a1cf51236a74a427bf0fa3c90ea65a`
시험 명령: `python -B tools/porting/psy02_reference.py --check`; `.runtime/porting/PSY-02/run_committed_units.py`; `tools/porting/check_psy02_production.py --case <각 사례> --original-driver-build .runtime/porting/PSY-02/native-driver-replay-target-frozen/native-driver-build.json`
증거 경로: `contracts/PSY-02-{source,cases,tolerances}.json`, `evidence/PSY-02/{source-original-preparation,source-unit-preparation,build-receipt,native-reachability,comparison-report,independent-review}.json`, `evidence/PSY-02/production/*.json`
최대오차/RMSE/상태 불일치: 단위 516개 수치 호출·33,510개 검사는 최대오차/RMSE 0, 불일치 0. 실제 실행의 Twb 최대오차 `1.0436096431476471e-14 °C` / RMSE `1.8451792967017515e-15 °C`, W/RH `4.440892098500626e-16 kg/kg` / `2.451905405833003e-17 kg/kg`, 최종 Psat 캐시 `1.4551915228366852e-11 Pa` / `1.7537739051563307e-12 Pa`. 모두 사전 고정 허용오차 내 통과. 순서·입력·태그·분기·정확 비교 필드는 불일치 0.
추가 검토할 helper: 이 카드의 직접 Psat/Pb/W(T,Twb,P), F6/F7, 순서 의존 min/max와 원본 반복 보조함수의 본문·상태를 고정했다. `General::Iterate` 직접 호출 5개, 경고 재발행 수명과 통계 OFF의 내부 반복 횟수는 독립 Rust 비교 통과로 세지 않는다.

## 완료 근거와 적용 한계

원본 14개 파일·29개 구간과 입력·정밀도 계약을 수치 비교 전에 고정했다. 같은 입력의 원본 C++를 먼저 실행한 뒤 세 개의 새 Rust 상태 수명에서 단위 결과, 호출 전후 스칼라, 캐시 태그·첫 입력 보존·충돌·네 개 최종 테이블을 검증했다. Rust 생산 바이너리와 tuple 바이너리는 위 커밋에서 빌드·보관했으며, 실제 컴파일 소스의 보관 바이트와 Git의 개행 정규화를 구분했다.

일반 CLI Full/Summary 14회는 A24H/A72H, B의 NoLimit/flow/capacity/both24H, both72H를 실행했다. 1,056개의 실제 zone timestep에서 22,895개 선택 물성 호출을 순서대로 원본에 재생했고 누락과 불일치가 없다. 호출자의 실제 입력·상태·캐시를 비교하며, 원본 예상값은 Rust 계산에 주입하지 않는다. 선택 출력·meter·전체 JSON 시계열은 Full/Summary에서 같고 Summary는 관측 파일을 생성하지 않는다. 호출 합계에는 초기화·검증·재계산이 포함되므로 원본의 물리 생산자 호출 횟수와 같다고 주장하지 않는다.

원본의 별도 CON15 실제 실행과 캐시 수명·태그 쓰기 검토는 해당 입력의 warmup까지 cached HPb 호출이 비활성임을 입증한다. 그 함수를 실제 Rust 생산 호출로 가장하지 않으며, 정상·경계·비제로·반복 상태의 단위 증거로 구현을 검증했다. 현재 물성 소비 지점의 연결만 완료했다. HVAC-05 포화 보정 전체 조립, CLK-04 기상 조립, B 연간 Rust 물리 실행, warmup과 SYS 인계·통합 및 건물 전체 원본 일치는 후속 카드의 별도 의무이다.

기존 전체 workspace 4,533개 시험(16 suite), clippy, IdealLoads의 기존 소스·트랜잭션 검증을 통과했다. 개발 중 실패한 명령과 최초 native 정의 검증 실패도 변경 없이 보관했다. 수치 결과를 맞추기 위한 허용오차 완화, 근사식 또는 생산 fixture 주입은 없다.
