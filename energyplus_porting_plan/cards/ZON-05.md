# ZON-05 — 현재 범위의 수분수지

- 그룹: 08 존 공기
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: ZON-01, PSY-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/ZoneTempPredictorCorrector.cc`  
심벌: `ZoneSpaceHeatBalanceData::correctHumRat`  
대상 블록: 현재 no-OA·무잠열원 분기 및 공조노드 피드백의 습공기비 처리. 일반 습도제어와 별도.  
함께 읽을 선언/호출자: ZoneTempPredictorCorrector.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/ZoneTempPredictorCorrector.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

heat_balance/zone_humidity_correction.rs; zone_air_initialization.rs; state.rs

선택한 직접 함수와 단위 관측기를 구현했다. 실제 적용 소스와 검증 기준은 `evidence/ZON-05-unit/summary.json`에 기록한다.

## 입력 계약

존 W 이력·공기밀도·체적·dt, 범위 내 수분 유입/유출

## 출력·변경 상태 계약

직접 함수: 임시 W[kg/kg]와 양수 시스템 존 노드의 W·엔탈피. 현재 W·평균 W·이력 선택·갱신은 호출자의 별도 연결 범위이다.

## 호출 시점

Correct 단계

## 제외 범위

CON-01에서 제외한 입력·분기는 구현 대상에 넣지 않는다. 생략 분기가 실행되지 않는다는 근거를 남긴다.

## 단위시험

외부 수분원0에서 불변성; 표본 W 변화; 온도변화와 rho 영향

## 연결시험

ZON-02의 cp/rho 및 HVAC 공급노드 W와 일관성

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 검증 상태

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 고정 입력의 직접 수분수지 단위 비교 통과
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

원본 correctHumRat 4433–4619행 전체와 직접 helper를 확인했다. 확정 범위는 zone-only·완전혼합·ThirdOrder·NoAFN·no-OA·무잠열원이다. 유입 노드 순서, 정수 배수, 밀도·증발열·수분 용량의 계산 순서, 기존 4칸 working 이력, 하한 후 포화 비교, 독립적인 양수 시스템 노드 W/H 기록을 보존한다. 검증된 PSY-01/02 계산과 같은 지속 상태를 재사용한다. 저장 W에 별도 하한이나 대체 기본값을 넣지 않는다.

실제 출력 전에 입력 21개·호출 43개와 변수별 허용오차를 고정했다. 정상·온도/압력/용량/dt 변화·건조/습윤 공급·0유량·유입 순서·정수 배수·0 부근·반복 호출을 같은 입력으로 실행했다. 원본과 Rust 각각 43회 모두 반환했다. 수치 208개·이산 항목 21,573개에서 불일치 0, 네 수치 프로필의 최대 절대오차와 RMSE 모두 0이다. 원본 내부 지역 변수·guard 선택은 직접 관찰하지 않았으며, 미관찰·중단·결측을 통과로 계산하지 않는다. 다중 유입 순서 시험은 단위 기구 시험이며 생산 입력 범위를 확장하지 않는다.

포맷·전체 Clippy 통과, 열수지 테스트 211개 및 전체 workspace 테스트 4,816개 통과. 비교기 계약 테스트 7개도 통과했다. 품질·Rust 실행은 실제 SourceSnapshot `ab25fffa57a56d6b76d225c98cc5ecc4083ac70f454154a4cb90be83ee994193`에 결합된다. 명령·결과·소스 차이는 `evidence/ZON-05-unit/manifest.json`과 `summary.json`에 보존했다.

남은 작업은 전체 4칸 상태의 실제 지속 소유권, PurchasedAir 공급/피드백 노드 W/H 전달, 온도 후 습도 보정 순서, working 이력 선택, 현재 W/RH·가중 평균 기록, 조건부 zone/system 이력 push와 rollback이다. 기존 고정 경로의 system 이력 선택 및 무조건 DS push는 원본의 non-shortened zone-step 순서와 다르다. 실제 일반 실행 호출자를 관측한 뒤 연결한다. 고정 15개 입력에는 수분 용량 override 객체가 없으므로 원본의 명시적인 기본값 1.0 생산자를 사용할 수 있다. 더 넓은 입력의 override와 음수 밀도 진단 백엔드 동등성은 미검증이다.

증거: `evidence/ZON-05-original-baseline/summary.json`, `evidence/ZON-05-unit/summary.json`. 연결·생산 gate와 카드 전체는 미완료다.
