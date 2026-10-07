# GEO-02 — 면적·방위·경사·중심점

- 그룹: 01 입력·기하
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: GEO-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/SurfaceGeometry.cc`  
심벌: `GetVertices`의 불투명 표면 기하 계산, `CalcSurfaceCentroid`, `ProcessSurfaceVertices`

대상 블록: 원본에서 실제 값을 만드는 `GetVertices`의 Newell 법선·면적·방위·경사 및 외향 법선 저장, `CalcSurfaceCentroid`의 중심점 저장. `ProcessSurfaceVertices`는 이미 계산된 기하값의 후속 소비·형상·scratch 수명 문맥이다. 호출되는 Vectors 보조 함수와 ObjexxFCL `cen`은 해당 계산만 동봉한다.
함께 읽을 선언/호출자: SurfaceGeometry.hh; Vectors.hh/.cc의 실제 호출 보조 함수만  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/SurfaceGeometry.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/geometry.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

전역 꼭짓점[m], SurfaceClass

## 출력·변경 상태 계약

면적[m²], 법선, 방위/경사[deg], 중심점[m]

## 호출 시점

모델 초기화

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

수직벽·수평지붕·바닥; 뒤집힌 방향; 퇴화면 차단

## 연결시험

표면별 EIO 기하값 및 SRC-05 높이·SRC-04 입사각 연결

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [x] 상태·시간·호출순서를 포함한 연결시험 통과
- [x] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋: `edb6f57822b319692b0edaa30bc276a66bd3319e`

시험 명령: 아래 실제 실행 명령 및 [전체 명령 기록](../evidence/GEO-02/actual-commands.json)

증거 경로: [원본·상태 검토](../evidence/GEO-02/source-review.md), [최종 단위 비교](../evidence/GEO-02/final-unit-comparison.json), [생산 비교](../evidence/GEO-02/production-comparison.json)

최대오차/RMSE/상태 불일치: 보고된 기하·실제 소비 필드별 최대 절대오차와 RMSE 0, 비교 불일치 0; centroid 526개 결과 exact bits

추가 검토할 helper: 이 카드의 직접 helper는 고정 계약에서 검증. 존 체적, SRC-04/05 전체 계산·시간 상태는 후속 카드 대상.

```powershell
python -B tools/porting/check_geo02_units.py --original-first .runtime/porting/GEO-02/original-helper-first-01/helper-reference.json --native-output .runtime/porting/GEO-02/original-helper-first-01/helper-results.json --rust-execution .runtime/porting/GEO-02/rust-final-01/execution.json
python -B tools/porting/check_geo02_production.py --matrix .runtime/porting/GEO-02/production-edb6f578-01/matrix.json --output-dir .runtime/porting/GEO-02/production-comparison-04
```

두 번째 명령은 실제 성공 명령의 기록이다. 재검토할 때는 기존 증빙을
덮어쓰지 않도록 새로운 `--output-dir`를 사용한다. 두 검증기는 보존된
출력을 읽으며 엔진이나 Cargo를 실행하지 않는다.

## 범위 보완 및 완료 기록

원래 카드의 심벌 목록에는 면적·법선·방위·경사의 실제 생산자인 `GetVertices`
계산 블록이 빠져 있었다. 고정 원본에서는 중심점 저장도
`ProcessSurfaceVertices` 호출보다 먼저 실행된다. 원본 행구간·직접 helper·
초기화 및 상태 수명 계약을 별도로 고정한 뒤 수치 비교했다.

원본을 먼저 실행한 뒤 최종 committed Rust 단위 비교 3,182검사에서
불일치 0을 확인했다. 유효 사각형 15개 기하 scalar 450개와 세 입력을
독립적으로 합산·곱한 centroid 결과 526개가 모두 원본과 bit 일치한다.
unsafe helper 2개는 원본 경고·기존 centroid 보존 관측이며 Rust parser
동등성으로 집계하지 않는다. 실제 원본 x87 PC64/RNE를 관측한 정밀도
정책을 사용했고, signed zero를 포함한 고정 입력·허용오차는 변경하지 않았다.
수평 epsilon 입력 2개는 모두 실제 near-horizontal 분기였으며 이름만으로
경계 양쪽이 검증되었다고 주장하지 않는다.

생산 비교는 A-24H, A-72H, B-BOTH-24H의 Full/Summary 6회와 일반 입력
퇴화면 실패 2회이다. 총 72,555검사, 불일치 0이며 Full 저장 기하 scalar
540개, 실제 zone 호출 480개에 연결된 ordered operand 이벤트 23,232개를
누락 없이 보존했다. 온도 높이와 면적은 모든 구간에서 소비되지만
convection·wind·solar는 실제 조건부 경로이다. B의 NoWind/NoSun은
해당 소비가 비활성이라는 증거이며 계산 완료로 세지 않는다.
Summary는 일반 결과의 Full 동일성과 observer 부재를 검증하며 16개
기하 필드의 별도 직접 관측은 아니다. 퇴화면은 Rust runtime exit 6,
원본 exit 1로 물리 단계 전에 차단되며 오류 문구·코드 일치는 주장하지 않는다.

기존 baseline 실패와 precommit candidate는 그대로 보존했다. 최종 reader의
selector·source path·enum 표기 수정도 기존 입력·수치·실행 결과를 바꾸지
않고 별도 실패 이력 및 성공 검토로 기록했다. 선행 GEO-01 gate를 확인했으며
이 카드 완료는 GEO-03 및 SRC-04/05의 완료를 의미하지 않는다.

단위 진단 입력은 생산 CON 범위를 확장하지 않는다. 기존 GEO-01 기록에는
법선과 중심점이 없어 새 원본 관측이 필요하다. 높이 소비값은 중심점의 Z이며,
`ProcessSurfaceVertices`의 표면 모서리 높이와 구분한다. 이후 SRC-04·SRC-05의
전체 계산식과 시간 상태 동등성은 각 후속 카드에서 검증한다.
