# CLK-04 — 비일사 기상값 timestep 보간

- 그룹: 02 시간·기상
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: CLK-03
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/WeatherManager.cc`  
심벌: `SetupInterpolationValues; ReadEPlusWeatherForDay; SetCurrentWeather; interpolateWindDirection`  
대상 블록: 건구·노점/습도·기압·풍속/풍향·강우에 실제 적용되는 보간/선택 블록만.  
함께 읽을 선언/호출자: WeatherManager.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/WeatherManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/weather

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

이전/현재 기상값, timestep index 및 개수

## 출력·변경 상태 계약

현재 zone timestep 기상상태

## 호출 시점

각 zone timestep

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

Timestep 1/4; 첫시간 seed; 359→1도 풍향; 강우 전환

## 연결시험

SRC-05의 표면 외기 조건 및 PSY 입력으로 연결

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [x] 상태·시간·호출순서를 포함한 연결시험 통과
- [x] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋:  
시험 명령:  
증거 경로:  
최대오차/RMSE/상태 불일치:  
추가 검토할 helper:  

## 실제 완료 증거

구현 `15127617dbc39ee82f0e631e3fe99a3a2fc1e87e`: 단위 5,517,963건·불일치 0, A/B 생산 3,736,442건·불일치 0. 기존 3개 CON 입력의 Full/Summary 6회 실행과 Current 480개를 확인했습니다.

최종 품질 `a036b012100d2a4cd38240925a7f92f6784f1ce7`: workspace 4,591개 통과, Clippy·형식·소스·구조 검사 실제 0. 생산 수학과 Cargo는 동일하며 관찰자 5개 파일의 후속 정리 계보를 별도로 보존했습니다.

원본 native 0 / 당시 wrapper 1 / metadata recovery 0, 과거 단위·생산 실패와 비가용 경로를 그대로 보존했습니다. 일사·하늘 및 이후 카드 물리식은 이번 통과 범위에 포함하지 않습니다.

증거: `energyplus_porting_plan/evidence/CLK-04.json`; 압축 원문 보존: `energyplus_porting_plan/evidence/CLK-04-archive/archive.json`.
