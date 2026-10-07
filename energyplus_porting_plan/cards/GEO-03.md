# GEO-03 — 대상 존의 체적 산정

- 그룹: 01 입력·기하
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: GEO-02
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/SurfaceGeometry.cc`  
심벌: `CalculateZoneVolume`  
대상 블록: 입력 체적 우선순위 및 선정한 단순 폐합 존에 해당하는 경로만. bbox 결과 일치를 일반 다면체 이식 완료로 보지 않음.  
함께 읽을 선언/호출자: SurfaceGeometry.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/SurfaceGeometry.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/geometry.rs

계획 반입 시 재사용 기준은 `d7b516627f421259012f3e61bc28cca831452468`이었다.
최종 구현 `966af17aa5fc6bd68cddb9cfa23079dffc09ddc7`은
`geometry/zone_volume.rs`와 `zone_volume/topology.rs`의 실제 소유자를 사용한다.

## 입력 계약

존 표면·체적 명시값·천장높이·바닥면적

착수 계약은 `contracts/GEO-03-source.json`, `GEO-03-cases.json`,
`GEO-03-tolerances.json`에 동결했다. `evidence/GEO-03/independent-contract-review.json`과
`independent-native-source-review.json`의 독립 사전 검토는 통과했다.
원본 빌드와 helper 19개·일반 IDF 12개 실행 및 독립 검토를 완료했다.
`evidence/GEO-03/original-first.json`에 실제 실행과 관측 상태를 연결했다.
기존 Rust baseline은 실제 582개 검사에서 수치 불일치 3건과 미구현 관측
192건을 기록했다. 수정한 candidate는 실제 2,672개 검사와 다섯 owner
관측 단계에서 불일치 없이 통과했고, 체적·높이·면적의 최대오차와 RMSE는
모두 0이다. `evidence/GEO-03/baseline.json`, `candidate-unit.json`과 각
독립 검토에 보존했다. 최종 커밋 단위 비교는 2,672개 검사, 실제 생산 연결
비교는 758개 검사에서 불일치 없이 통과했다. `final-unit-comparison.json`,
`production-comparison.json`, `actual-commands.json`, `independent-review.json`에
원시 실행과 독립 검토를 연결했으며, 동결한 A/B 경계의 네 종료 gate를 닫았다.

단위 입력은 여섯 개의 유효한 사각형 면으로 닫힌 하나의 직육면체와
명시 Zone 숫자, 준비된 면적 읽기 필드이다. 준비된 면적은 입력으로 선언하며
단위 wrapper가 원본 면적 입력 처리까지 수행했다고 주장하지 않는다.
별도의 일반 IDF 원본 실행에서 실제 바닥면적·높이·입력 플래그 생산자를 관찰한다.

## 출력·변경 상태 계약

존 체적[m³] 및 산정 실패 진단

`CalculateZoneVolume`의 `12010–12322`행과 직접 helper를 고정한다.
면과 폐합 상태를 평가한 뒤 현재 양수 체적을 유지하고, 그렇지 않으면
`ceilingHeightEntered`와 양수 바닥면적에 따라 입력 높이의 곱을 우선한다.
나머지는 원본의 부호 있는 피라미드 합이다. 이 합은 일반 binary64 `/3.0`을
사용하며, GEO-02 중심점의 확장 정밀도 곱과 다르다.

`ceilingHeightEntered`는 파서가 설정하지 않는다. 원본 `SetupZoneGeometry`
`455–546`행은 자동 높이를 쓰기 전에 기존 양수 높이로 플래그를 설정한다.
최종 높이가 양수라는 사실만으로 입력 높이로 분류하지 않는다.
두 번째 체적 호출은 첫 호출이 쓴 현재 양수 체적을 읽으며 새 lexical 입력으로
분류하지 않는다. Space 분배·경고 카운터·임시 버퍼는 대응 Rust 소유자가
검증되기 전까지 원본 관찰로만 남긴다.

Rust는 `geometry/zone_volume.rs`와 `zone_volume/topology.rs`에서 실제
mutable owner와 폐합 면의 부호 있는 합을 구현한다. 정상 IDF 면의 선언
순서는 검증된 RawModel overlay로 보존하고, 체적 합은 Wall/Floor/Roof별
원본 순서를 따른다. 초기화 반환값의 열 가지 필드를 관찰하며, 이후 존
상태에 저장·소비되는 연결 검증 대상은 체적이다. 공기 열용량의 실제 상태
갱신 호출은 `zone_air_heat_capacity_update` 관측 구간으로 보고용 재계산과
구분한다. 이 관측 구간은 계산식이나 source stage를 변경하지 않는다.

첫 전체 회귀시험의 실제 종료 코드 101과 69개 실패를 보존했다. 공용 열
시험 fixture는 안쪽 winding에서 bbox로 얻던 1m³를 전제했다. 해당 fixture의
체적만 1m³로 명시하고 좌표·높이/바닥 자동값·기존 열 검증을 유지했다.
같은 형상의 양수 명시 체적 유지와 자동 체적 거부를 확인하는 별도 회귀
검사도 추가했다. 계산 소스와 source 비교 입력·허용치는 변경하지 않았다.
`evidence/GEO-03/independent-fixture-review.json`에 수정 범위 검토를 연결했다.

## 호출 시점

모델 초기화

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

명시 체적/자동 체적; 동일 직육면체의 회전·이동; 비허용 형상 차단

최종 단위 비교는 폐합 직육면체 16개와 대응하지 않는 원본 전용 진단 3개를 포함한다.
열 가지 Zone 필드, 별도 선언 읽기 단계를 포함한 다섯 관측 단계와
두 번째 호출의 현재 양수 체적 유지를 검증했다. 체적·높이·면적의 최대오차와
RMSE는 모두 0이며 입력 비트·플래그·면 순서는 정확히 대응한다.
열린 존, 중복 면, 뒤집힌 winding의 원본 fallback이나 보정을 Rust의 정상 입력
허용 또는 동일 오류 메시지로 간주하지 않는다. 뒤집힌 면 진단은 원본
`GetVertices` 자동 방향 보정을 명시적으로 우회한 준비 상태 실험이다.

## 연결시험

ZON-02 공기 열용량에 사용한 체적 비교

실제 저장 체적이 초기화와 공기 열용량 소비자의 인수로 전달되는 연결만 검증한다.
`AirPowerCap` 전체 식, 시스템 시간간격, 승수 및 ZON-02 수치 완료를 승격하지 않는다.
실제 Rust 실행은 A-24H, A-72H, B-BOTH-24H의 Full/Summary 여섯 명령이다.
실제 갱신 구간·호출자·시간 문맥의 672개 공기 열용량 호출이 480개 순서 있는
존 구간을 덮으며 누락은 없다. Full은 초기화가 반환한 열 가지 필드를 관찰하지만
최종 존 상태에 저장되고 소비되는 필드는 체적뿐이다. Summary는 일반 출력의
동일성과 Full 관측 파일의 부재만 확인하며 열 가지 필드의 직접 관측을 주장하지 않는다.
B72·연간·각 제한 분기의 새 연결시험을 주장하지 않는다.

B의 양수 명시 체적 1m³는 선언·원본·반환·저장·실제 인수 비트가 같다.
자체 진단의 winding 불일치와 부호 합 -1/3은 그대로 공개하며, 이를 자동 체적
허용으로 해석하지 않는다. 별도의 일반 A24 중복 벽 입력은 실제 체적 소유자에서
물리 실행 전 Runtime 종료 코드 6과 존 물리 루프 관측 0건으로 거부했다. 원본은 경고 후
종료 코드 0과 96개 실제 콜백으로 계속했다. 종료 코드·문구·fallback 동등성은 요구하지 않는다.

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [x] 상태·시간·호출순서를 포함한 연결시험 통과
- [x] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋: `966af17aa5fc6bd68cddb9cfa23079dffc09ddc7`

메타데이터 reader 수정 커밋: `6288c2b3153b7c4851efb387c585f0f68d56af7b`.
실제 두 reader 실패는 보존했으며 수정은 계산·입력·허용치를 변경하지 않았다.

시험 명령: `python -X utf8 -B tools/porting/check_geo03_units.py`와
`python -X utf8 -B tools/porting/check_geo03_production.py`의 실제 인수·종료 코드는
[actual-commands.json](../evidence/GEO-03/actual-commands.json)에 있다.

증거: [source-review.md](../evidence/GEO-03/source-review.md),
[final-unit-comparison.json](../evidence/GEO-03/final-unit-comparison.json),
[production-comparison.json](../evidence/GEO-03/production-comparison.json),
[independent-review.json](../evidence/GEO-03/independent-review.json).
최종 전체 회귀시험 4,548개와 Clippy도 통과했다.

최대오차/RMSE/상태 불일치: 비교한 체적·높이·면적은 0; 최종 단위 2,672개와
생산 연결 758개 검사에서 불일치 0. Space·원본 전역 카운터·IO·임시 버퍼·보정과
비허용 fallback은 대응하지 않으며 AirPowerCap/ZON-02/SYS·연간·전체 EP 인증으로 확장하지 않는다.
