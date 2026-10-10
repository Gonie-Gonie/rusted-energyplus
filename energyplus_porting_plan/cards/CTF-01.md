# CTF-01 — 구성체 열물성 전처리

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: CON-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/Construction.cc`  
심벌: `ConstructionProps::calculateTransferFunction`  
대상 블록: 재료층 입력을 dl/rk/rho/cp/lr로 가져오는 블록; 저항층 판정·연속 저항층 처리·필요 단위 변환.  
함께 읽을 선언/호출자: Construction.hh::ConstructionProps의 열전도 필드; Material.hh 사용 필드  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/Construction.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/heat_balance/surface_manager.rs

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

층별 두께·k·rho·cp·R; layer order

## 출력·변경 상태 계약

정규화 층목록, 저항층 flag, 총 R 및 CTF 계산용 단위값

## 호출 시점

구성체 초기화

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

단층·다층·연속 저항층·저항+유질량 층; 전처리 중간값 비교

## 연결시험

CTF-02/03에 전달되는 배열·순서 검증

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

## 착수 시 범위·owner 계약 보완

고정 EP 커밋의 `Construction.cc` 59–401을 선정 범위로 한다. 59–200의 실제 상태 초기화·미사용 early return과 로컬 할당, 201–309의 층 로딩·저항층 분류·오류, 311–315의 오류 후 return, 317–372의 연속 내부 저항층 병합, 374–401의 순서 있는 단위 변환·총 저항·conductance를 포함한다. 원본 함수 전체는 1103행까지 실제로 계속 호출한다. 401행 뒤 CTF 계산을 새 전처리 식으로 대체하거나 관찰을 위해 조기 종료하지 않는다.

직접 companion은 `Construction.hh`의 capacity·실제 Construct 배열·생성자, `Material.hh`의 실제 MaterialBase 필드, `Material.cc` 170–225/249–285의 Regular/NoMass 저장 owner, `DataConversions.hh` 58–69의 상수다. 실제 caller의 공유 오류 상태와 호출 순서는 `HeatBalanceManager.cc` 6153–6167, 실제 CTF 사용 owner는 `SurfaceGeometry.cc` 9081–9107을 기준으로 한다. 이 범위 표시는 뒤쪽 계수 생성과 CTF-02부터 CTF-10까지의 완료를 뜻하지 않는다.

단위 입력은 39개 모델·42회 선언 호출의 명시적 MaterialBase/Construct 멤버다. 스칼라 binary64 bits, 재료·구성체·LayerPoint·호출 순서를 보존한다. 저장된 Resistance는 독립 입력 그대로 공급하며 wrapper에서 d/k로 만들거나 실제 loader 출력이라고 주장하지 않는다. 각 모델은 새 실제 EnergyPlusData와 constant 초기화를 사용하고, 실제 객체를 모두 배치한 뒤 전체 public 메서드를 호출한다. ErrorsFound/DoCTFErrorReport는 모델별 한 쌍의 실제 reference로 공유하며 원본 오류·예외 뒤 미호출을 구분한다.

관찰 지점은 원본 309·372·401행 직후의 실제 활성 로컬 배열이다. dl/rk/rho/cp/lr/ResLayer와 LayersInConstruct/NumResLayers/오류 상태를 복사하고, dyn/rs/cnd는 실제 대입이 끝난 마지막 지점에서만 복사한다. 병합은 원본 TotLayers/LayerPoint를 바꾸지 않으므로 병합 후 재료 ID나 비활성 tail을 만들어 비교하지 않는다. 세 관찰 지점은 개별 병합 event나 sqrt 중간값 trace를 제공하지 않는다.

18개 변수별 산술 profile의 atol/rtol=0, class/shape 정확 일치와 복사 입력 bits 정확 일치를 원본 출력 확인 전에 채택한다. air cp=1.007, 실제 상수 의존·연산 순서, cp/=(CFC*1000), 순서 있는 lr 합, guard 없는 cnd=1/rs를 보존한다. 비교 실패를 이유로 허용오차를 늘리지 않는다. 도달한 필수 값이 양쪽 또는 한쪽에서 누락되면 실패다. 미사용·원본 중단·OS 비정상 종료·관찰 오류와 미도달 상태를 명시하고 수치 통과에 포함하지 않는다.

현재 Rust에는 Regular의 저장된 Resistance와 이 세 단계 정규화 배열 owner가 없다. 기존 derived R/C 합계나 최종 계수는 해당 owner의 대체 정답이 아니며 문맥으로만 보존한다. 실제 원본과 기존 구현의 차이를 확인한 뒤 기존 material/compiler/cache 초기화 경로를 보완한다. 나중의 원본 solver 오류 flag·계수와 Rust 전처리의 미소유 상태는 비교되지 않은 문맥으로 남긴다.

생산 검증은 별도다. 기존 CON-01의 고정 입력 15개·입력/기상/메타데이터 해시 45개를 유지하며, 실제 InputProcessor·재료·구성체 loader와 실제 CTF 사용 owner를 관찰해야 한다. 위 멤버 단위 입력은 이를 인증하지 않는다. source/sink·2차원·다른 재료군·0 또는 11 초과 층은 제외한다. 새 실행·수치 비교·네 gate 통과를 이 문서 변경만으로 주장하지 않는다.

## Actual Original and existing Rust baseline

`evidence/CTF-01-baseline/summary.json` preserves genuine Native39/42 and prior Rust fixed15 owner inventories. OriginalDataQA passed before Rust execution: 42 whole source calls returned, 106 callbacks were recorded, and 20 unreached phases remain unavailable. Existing Rust loader/compiler/initialization returned in all15 fixed cases; stored Regular Resistance and the selected preprocessing owners are absent as recorded. The two different lanes are not a numeric comparison. Scope review is complete; unit/integration/production gates remain pending.

## Actual selected preprocessing unit implementation

`evidence/CTF-01-unit/summary.json` preserves the actual candidate source, Cargo build/run, failed precomparison Windows path attempt, corrected comparer and quality records. The corrected comparison uses the original frozen 18 zero-tolerance profiles: 1,874 numeric and 4,346 discrete comparisons, zero mismatches, maximum absolute error and profile RMSE both zero. Twenty genuinely unreached phases remain unavailable and are not counted PASS. Regular stored Resistance and selected PostLoad/PostMerge/PostConversion owners now follow the actual compiler/cache/initialization path. Workspace tests passed 4,642/4,642; Clippy with warnings denied and formatting passed. Only the selected unit gate passes; fixed15 genuine-loader integration, downstream CTF02/03 handoff and production gates remain pending.
