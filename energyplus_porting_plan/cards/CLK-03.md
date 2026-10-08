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

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [x] 단위시험 통과 및 실제 활성 분기 확인
- [x] 상태·시간·호출순서를 포함한 연결시험 통과
- [x] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋: `df88eb4c7b03cd8a04ad09d5fce3300a4d5b7b22`; crates tree `47e544dafabcec8e30197597b8832ee1c290c8fc`
시험 명령: 실제 unit·production·workspace·Clippy·구조·형식 명령 및 기록은 아래 종료 증거에 결합한다.
증거 경로: [종료 증거](../evidence/CLK-03.json).
최대오차/RMSE/상태 불일치: exact 단위 239,178건/0, 생산 1,742,058건/0. RMSE를 재계산하지 않는다.
추가 검토할 helper: CLK-04/06 및 기존 물리 계산·native 내부 상태는 계속 별도 검증 대상이다.

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

## 라이브 커서와 A/B 연결 후보

실제 EPW 커서와 header/parser 상태를 유지하는 일별 읽기, 전체 daily 11필드·weather 17필드 인계, 준비된 첫날의 일회성 소비, A/B의 owned Today 소비 경로를 구현했다. 입력을 미리 읽은 값은 달력·별도 관측 준비에 사용하며 생산 기상 값은 라이브 커서가 공급한다. 초기 CTF 이력은 실제 첫 시간의 원자료를 사용한다.

workspace 회귀시험 4,580개가 통과했다. 그 뒤 독립 소스 검토에서 찾은 완료 시간의 PreviousHour 인계와 warmup의 DayOfSimChr="0" 처리를 고쳤고, 실제 반복 warmup·다일 경계를 포함한 기상 시험 56개가 통과했다. 최종 `cargo clippy --workspace --all-targets -- -D warnings`도 종료 코드 0이다. 초기 하늘복사 시험의 비활성 지붕 형상과 정적 검사 실패 기록은 보존했다.

[구현 후보 및 회귀시험 증거](../evidence/CLK-03-candidate-implementation.json)에 실행별 소스·명령과 검사 시점을 기록했다. 최초 구현 증거 작성 시점에는 원본 대조를 실행하지 않았고, 네 종료 gate는 계속 미확인이다. 생성된 기상 물리값의 동등성, 전체 native registry와 내부 locals는 이 후보의 복사·커서 검증으로 인증하지 않는다.

첫 커밋 후보의 실제 관측 실행은 종료 코드 0이다. 첫 대조는 Windows canonical 경로의 `\\?\C:\...` 표기를 일반 저장소 경로와 같은 입력으로 해석하지 못해 종료 코드 1이며 과학 비교 보고서는 생성되지 않았다. 기존 EXE·관측 결과·실패 기록을 보존하고 새 관측 serializer의 rooted VerbatimDisk 표기만 일반 disk 경로로 바꾼다. 실제 읽기 경로·입력·caller·scalar·엄격한 비교 기준은 그대로 유지하며 새 커밋과 빌드로 재실행한다.

[경로 표기 수정 증거](../evidence/CLK-03-candidate-path-amendment.json)에 종료 코드·빈 비교 stdout·미생성 과학 보고서, 기존 결과 보존과 수정 소스의 독립 검토를 기록했다. 수정한 관측 프로그램의 Clippy도 실제 종료 코드 0이다.

## 실제 단위 대조와 최종 품질 검사

`34364787737a0f5799b11a2b7ba80594cf6c70f9`에서 빌드·실행한 후보의 실제 단위 대조는 239,178건·불일치 0이다. 14개 sequence·51개 요청에서 실제 호출 48개, 정상 반환 46개, 의도한 source fatal 2개와 후속 skip 3개를 확인했다. 사용 불가능한 값 114개와 내부 인계 경계 12개는 PASS에서 제외했다. 보고서는 `.runtime/porting/CLK-03/candidate-unit-comparison-02/unit-comparison.json`, 독립 결과 검토는 `independent-unit-result-review-03/review.json`에 보존한다. 처리된 기상 물리값 생성의 동등성을 이 인계 검증으로 인증하지 않는다.

저장소 구조 검사에서 기존 solar 760행·run-period 920행 제한을 초과해, 기존 보간 함수 3개와 standalone timestep wrapper를 각각 하위 모듈로 분리했다. 함수 서명·본문과 shared day driver는 그대로 유지하며, 새 wrapper에는 100행 제한을 추가했다. 구조 검사의 이전 weather-series/precompute 단언도 실제 current-context/live-owner 경로를 확인하도록 갱신했다. 독립 검토 `quality-source-lineage-review-02/review.json`은 변경한 부모 2개·추가한 자식 2개와 테스트 줄바꿈 1개만 기존 수치 실행의 Rust 소스와 다름을 확인한다. 기존 수치 실행과 최종 QA 소스의 identity를 같다고 기록하지 않는다.

최종 실제 명령 `cargo test --workspace -j 2`는 4,580 passed·0 failed·0 ignored, `cargo clippy --workspace --all-targets -- -D warnings`는 종료 코드 0이다. source-quality는 production 파일 2,533개 검사·test 파일 1,172개 제외, structure 및 변경 파일의 scoped rustfmt 검사도 실제 종료 코드 0이다. 각 기록은 `.runtime/porting/CLK-03/final-workspace-unit-command-01`, `final-clippy-command-01`, `final-source-quality-command-03`, `final-structure-command-06`, `final-scoped-format-command-03`에 있다. 구조·형식 및 최초 PowerShell 실행 실패는 덮어쓰지 않았다.

Full/Summary 생산 실행 6개는 모두 실제 종료 코드 0이다. 첫 판독은 CRLF를 포함한 입력 행 해시와 native readLine 문자열 해시를 동일하게 요구한 metadata 오류로 종료 코드 1이며 수치 보고서는 생성되지 않았다. `production-comparison-command-01`의 실패를 보존하고, 전체 원본 행과 실제 전달 문자열을 각각 해시·cursor 위치로 확인하는 수정 판독기를 `independent-production-reader-amendment-source-review-01`에서 독립 검토했다. 입력·원본 출력·6개 실행·기존 관측 선택과 수치 정책은 유지한다. 수정 판독과 최종 생산 결과 검토를 마치기 전에는 네 종료 gate를 체크하지 않는다.

## 최종 생산 호출 이력 대조와 종료

앞 절의 준비·실패·초기 결과는 당시 기록으로 보존한다. 이전 생산 대조는 1,647,956건 중 DatesShouldBeReset 한 건이 불일치했다. 생산은 각 timestep에 Initialize를 호출했지만 최초 원본 대조 입력은 일 경계만 호출했다. 실패 보고서의 값·정책·상태를 바꾸지 않았다.

현재 Rust caller 소스의 부분 대입과 실제 호출 시점을 입력만으로 재구성한 3사례/483호출 계약과 관측 선택을 새 원본 실행 전에 동결했다. 새 genuine 원본의 3개 GetNextEnvironment·480개 Initialize 호출과 독립 결과 검토가 끝난 뒤, 같은 최종 커밋 EXE로 Full/Summary 6개를 실행했다. Rust trace나 원본 결과값은 실행 입력으로 공급하지 않았다.

최종 커밋의 단위 대조는 239,178건/불일치0, 새 생산 대조는 1,742,058건/불일치0이다. 두 결과는 각각 별도의 독립 메타데이터 검토를 받았다. workspace 4,580개, Clippy·source-quality·structure·scoped rustfmt도 실제 종료 코드0이며 최종 source snapshot과 연결한다. 단위 원본14/51과 별도 생산 원본3/483의 시간 순서는 구분한다.

완료 범위는 CON-01의 비실측 hourly EPW와 4 timestep 환경에서 라이브 커서·선택 상태·전체 carrier 복사·owned A/B 소비 인계이다. 결측·보간·sky·solar·처리된 기상 물리 RHS, native private index와 관측 불가 경계에는 PASS를 부여하지 않는다. 자세한 실제 명령·입력·원본·소스 해시·실패 보존·독립 검토는 [종료 증거](../evidence/CLK-03.json)에 기록한다.
