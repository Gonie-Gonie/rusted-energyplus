# GEO-01 — 좌표계와 꼭짓점 변환

- 그룹: 01 입력·기하
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: CON-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/SurfaceGeometry.cc`  
심벌: `GetVertices; GetHTSurfaceData`  
대상 블록: BuildingSurface:Detailed의 꼭짓점, Building/Zone 회전·원점·좌표계 처리 분기만.  
함께 읽을 선언/호출자: SurfaceGeometry.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/SurfaceGeometry.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_compiler; crates/ep_runtime/src/geometry.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

원본 꼭짓점[m], 원점[m], 회전각[deg], 좌표계 및 꼭짓점 순서

## 출력·변경 상태 계약

일관된 전역 좌표의 표면 꼭짓점

## 호출 시점

모델 초기화

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

회전 0/90도, 이동, Relative/World, 꼭짓점 방향의 예상값

## 연결시험

변환 뒤 GEO-02·GEO-03의 면적·체적·방위가 EP와 일치

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [x] 상태·시간·호출순서를 포함한 연결시험 통과
- [x] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋: `3f9b83978f2abe0963365aa66f7968920dad90fb`
시험 명령: `python -B tools/porting/check_geo01_units.py`, `python -B tools/porting/check_geo01_committed_equivalence.py`, `python -B tools/porting/check_geo01_production.py` — 실제 인수·실행 로그는 증거에 보존
증거 경로: `evidence/GEO-01/comparison-report.json`, `evidence/GEO-01/independent-review.json`, `evidence/GEO-01/build-receipt.json`
최대오차/RMSE/상태 불일치: 좌표 및 연결 소비값 최대오차·RMSE 0, 불일치 0
추가 검토할 helper: GEO-02·GEO-03에서 면적·방위·기울기·체적 계산 알고리즘을 별도 검증

## 완료 기록 — 단위·생산 연결 및 독립 검토 통과

고정 원본의 12개 파일·35개 행구간과 37개 입력은
`contracts/GEO-01-{source,cases,tolerances}.json`에 기록했다. CON의 15개 입력과
기하 진단용 22개 입력을 구분했으며, 진단 입력으로 생산 지원 범위를 확장하지 않는다.
원본의 실제 초기화·warmup·물리 실행에서 보존한 꼭짓점을 먼저 수집했다.

수정 전 Rust의 2,664개 좌표 비교에서 0의 부호 불일치 36개가 확인됐다.
기존 컴파일러에 원본 World 분기의 Appendix G 0도 회전 연산과 Relative 분기의
각도 합산 후 부호 반전 순서를 반영했다. 동일 입력·동일 허용오차로 다시 비교한
결과 불일치 0개, 최대 수치 오차와 RMSE 0을 확인했다.

Rust 단위 결과는 일반 CLI의 Full dry-run에서 실제 컴파일된 좌표를 복사했다.
37개 모두 컴파일 준비를 완료한 뒤 미지원 실행 의미를 종료 코드 4로 보고했고,
물리 계산·oracle 계산·진단 수치 probe는 실행하지 않았다. 이 과거 실행의
HEAD와 종료 코드 4는 그대로 보존했다. 커밋된 빌드와 실행 바이너리, 전체
3,637개 Rust 소스의 바이트가 일치함을 검증하여 기존 단위 결과를 연결했다.
추가 단위 엔진 실행으로 표시하지 않는다. 컴파일 소스와 커밋 blob은 3,636개가
바이트까지 같고, 기존 한 파일의 494행 CRLF 한 곳은 정규화 차이로 별도 기록했다.

생산 연결은 고정 CON 입력 7개에 대한 일반 CLI의 Full/Summary 실제 물리 실행
14개에서 검증했다. 실제 컴파일 좌표 504개와 최종 상태에서 복사한 면적·방위·
기울기·체적 133개가 원본과 일치했고, 소비값 133개의 비트도 모두 같았다.
Full의 실제 구간 관찰 1,056개와 Full/Summary의 CSV·시계열 결과 일치를 확인했다.
원본 결과·fixture 값을 생산 입력으로 주입하지 않았다. 각 입력의 최종 기하
투영 한 번을 비교한 것이며, 필드 수를 커널 호출 수로 해석하지 않는다.

원본 최대 꼭짓점 카운터의 초기값 4와 수명은 Rust 대응 필드가 없어 별도 보존했다.
A의 자동 계산 체적과 B의 선언 체적을 구분했다. 이 완료는 GEO-02·GEO-03의
계산 알고리즘, 시스템 시계·warmup·전체 물리 동등성 또는 B 연간 실행을 뜻하지 않는다.

시험: `cargo test -p ep_compiler global_geometry_rules --lib -j 2` 15개,
`cargo test -p ep_run geometry_trace --lib -j 2` 2개,
기존 출력 덮어쓰기 거부 시험 1개 통과. `cargo test --workspace --all-targets -j 2`
16개 묶음·4,535개 시험과 `cargo clippy --workspace --all-targets -j 2 -- -D warnings`
통과. 실행 시 HEAD는 `ead2c1b34f67cdf83facc0d08678186ed1453fdf`이며 수정된 실제
컴파일 소스와 바이너리는 별도로 보존했다. 이 당시 체크포인트는
`evidence/GEO-01/initial-checks.json`에 변경 없이 남겨 두었다. 이후 커밋 빌드의
소스 동등성, 실제 생산 연결 및 독립 검토 결과는 최종 비교·검토 증거에 기록했다.
