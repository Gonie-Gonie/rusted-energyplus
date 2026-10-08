# CLK-03 — 기상 레코드 선택·Today/Tomorrow 인계

- 그룹: 02 시간·기상
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: CLK-01, CLK-02
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/WeatherManager.cc`  
심벌: `GetNextEnvironment; ReadEPlusWeatherForDay; UpdateWeatherData`  
대상 블록: 선정 비실측 RunPeriod의 시작 위치, 하루 읽기 및 오늘/내일 버퍼 교체. 관련 분기만.  
함께 읽을 선언/호출자: WeatherManager.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/WeatherManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_runtime/src/weather; ep_run 기상 선택 경로

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

일자 상태, EPW 레코드, 환경 시작/일 종료 플래그

## 출력·변경 상태 계약

선택 record index, Today/Tomorrow 및 이전시간 기상값

## 호출 시점

환경 시작·일자 전환·warmup 반복

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

시작 앞에 decoy day; 24→1시; 첫날 재사용; 연도 처리 정책

## 연결시험

같은 타임스탬프의 Rust/EP 기상 원본 index와 값 비교

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 동결한 계약

원본 범위는 [source 계약](../contracts/CLK-03-source.json), 입력·caller는 [cases 계약](../contracts/CLK-03-cases.json), 정밀도는 [exact 비교 정책](../contracts/CLK-03-tolerances.json)에 고정한다.

GetNextEnvironment의 prepared 환경 승인과 ReadDay1의 실제 시작 검색을 구분한다. 원본 byte position으로부터 입력 행 identity를 외부에서 매핑하며 native record-index 필드를 만들지 않는다. UpdateWeatherData는 11개 daily 필드와 4×24개 전체 WeatherVars carrier의 복사·변경·불변 상태를 비교한다.

Warmup 첫날 재사용, 24→1시, 다음 날 lookahead, 마지막 날 no-prefetch, Hour1 대안, 첫날 검색 재호출 및 한 레코드 backspace를 별도 입력으로 둔다. 원본의 전체 prepared 함수 호출을 보존하되 결측 처리·보간·sky·solar 계산 결과와 raw EPW 저장소를 동일한 검증으로 취급하지 않는다.

현재 eager full-file parser와 mutable source-order cursor의 차이를 실제 legacy baseline으로 기록한 뒤 수정한다. 모든 종료 gate는 실제 시험과 생산 연결 검증 전까지 미확인이다.

## 준비 검증

[준비 증거](../evidence/CLK-03-native-preparation.json)에 고정 입력 14개 sequence·51개 요청 operation, 원본 helper 빌드와 기존 Rust probe의 실제 컴파일 영수증을 기록했다.

첫 원본 helper 컴파일은 관찰 코드의 타입 오류와 존재하지 않는 상태 필드 참조로 종료 코드 1을 반환했다. 실패 소스·로그를 보존하고 관찰 코드 두 파일만 수정했다. 두 번째 configure·compile/link·derivation은 모두 실제 종료 코드 0이다. EnergyPlus 원본 13,537개 파일, 기존 655개 compile row와 643개 core row, 보호 바이너리 14개는 보존됐다.

기존 Rust probe는 `b3e1a4ad2c63c80dfef9baa21cbb6e16599d6bcd`에서 실제 컴파일 종료 코드 0으로 생성한 EXE와 소스를 보존했다. 모든 종료 gate는 실제 수치 비교와 생산 연결 시험 전까지 미확인이다.

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

## 준비 순서 수정 2차

원본 첫 실행은 native assertion으로 종료했고 결과 JSON은 생성되지 않았다. 실제 GDB stack은 GetDesignDayData의 interpolation 접근을 확인했다. 실패 reference·command·debugger 증거와 기존 packet bytes를 보존한다.

SetupInterpolationValues를 ReadUserWeatherInput보다 먼저 호출하고, 기존 TimeStepFraction 값은 Setup 다음·Open 전에 기록한다. 입력·14개 sequence/51개 operation·byte map·CON45·비교 정책은 변경하지 않는다. 수정 계약은 재실행 전에 다시 동결했다. 모든 gate는 미확인이다.

[수정 및 실패 보존 증거](../evidence/CLK-03-preparation-amendment.json)에 원본 종료 코드 3221226505, 실제 디버거 호출 스택, 수정 계약의 독립 검토와 실패 원본 EXE의 보관 위치를 기록했다. 디버거 종료 코드 0은 수치 시험 통과를 의미하지 않는다.

## 필수 입력 수정 3차

두 번째 원본 실행의 종료 코드는 0이지만, 진단 IDF 7개에 필수 GlobalGeometryRules가 없어 27개 요청 작업이 실행되지 않았다. Decoy 검색·재호출·backspace·의도한 reader 오류 경로 검증과 Rust 기준 실행은 보류한다.

원본 IDD에서 요구하는 객체 한 줄만 추가한 새 IDF 7개를 만들고 입력 계약을 다시 동결했다. 기존 빌드·소스·바이너리는 보존한다. 새 실행 입력은 기존 빌드 기록과 별도로 검토한 뒤 사용한다. [입력 수정 증거](../evidence/CLK-03-input-preparation.json)에 실제 24개 호출·27개 skip과 이전 입력 보존 위치를 기록했다. 모든 gate는 미확인이다.

## 수정 입력의 원본 실행

3차 원본 실행은 native·launcher·외부 기록 도구 모두 실제 종료 코드 0이며 소스·입력·보호 바이너리 보존 검사를 통과했다. 같은 빌드에 대한 별도 입력 승인으로 실행했으며 기존 빌드 기록을 변경하거나 새 컴파일로 취급하지 않았다.

실제 요청 51개 중 48개가 호출됐다. 정상 반환 46개, 의도한 기상 오류 2개와 그 뒤 건너뜀 3개를 기록했다. 모든 준비 호출이 정상 반환했으며 decoy 검색, 첫날 재호출, 한 레코드 backspace, warmup 재사용, 다일 인계와 마지막 rewind 호출을 독립 결과 검토에서 확인했다.

[원본 실행 증거](../evidence/CLK-03-original-execution.json)에 명령·결과 해시와 사례별 실제 호출을 기록했다. 기존 Rust 기준 실행과 수정 구현의 단위·연결·생산 검증은 아직 수행 전이며 모든 종료 gate는 미확인이다.

## 기존 Rust 기준선과 차이

보존한 기존 Rust EXE의 실제 실행은 종료 코드 0이며 빌드 당시 Rust·Cargo·설정 3,690개 파일의 바이트가 유지됐다. 기존 eager loader와 불변 weather series에는 원본과 같은 일별 작업 전후 상태가 없어 51개 요청 작업을 모두 관측 불가로 기록했다. 수치 비교 건수·불일치 건수는 미확인으로 유지한다.

차이 기록 도구의 파일 목록 검사 오류 두 건은 실패 소스·명령·로그를 보존하고 수정했다. 최종 메타데이터 실행은 종료 코드 0이다. [기준선 증거](../evidence/CLK-03-legacy-baseline.json)에 실제 실행과 수정 범위를 기록했다. 이제 실제 파일 커서·전체 carrier 인계·공통 A/B 소비 연결을 구현하며 종료 gate는 단위·연결·생산 시험 전까지 미확인이다.
