# SCH-01 — Constant/Compact 입력 정규화

- 그룹: 03 스케줄·물성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 재검증
- 선행 카드: CON-01
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## 읽을 원본 코드

파일: `src/EnergyPlus/ScheduleManager.cc`  
심벌: `ProcessScheduleInput; ProcessForDayTypes; DecodeHHMMField`  
대상 블록: 선정 Schedule:Constant/Compact의 Through/For/Until·기본값만. 다른 스케줄 종류는 제외.  
함께 읽을 선언/호출자: ScheduleManager.hh  
출처: https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/ScheduleManager.cc

파일·심벌 기준의 작업 범위이다. 함수 전체가 아닌 분기 카드에서는 착수 시 해당 커밋의 실제 start/end 행과 직접 호출 helper를 고정한다. 이 작업계획은 모든 함수 본문을 잘라 검증한 소스 패킷은 아니다.

## 재사용 후보

crates/ep_compiler 스케줄 parser; crates/ep_runtime/src/schedules

현재 Rust 기준 커밋: `d7b516627f421259012f3e61bc28cca831452468`. 위 경로는 재사용·확인할 위치이며
이 카드의 새 검증이 통과했다는 의미가 아니다.

## 입력 계약

원본 Constant/Compact, ScheduleTypeLimits

## 출력·변경 상태 계약

기간·day type·시간구간·보간모드의 정규화 모델

## 호출 시점

모델 초기화

## 제외 범위

CON-01에서 제외한 입력·분기는 이 카드에서 구현하지 않는다. 실행 가능 여부를 확인하지 않고 무조건 0으로 대체하지 않는다.

## 단위시험

AllDays/선정 요일; AllOtherDays; 24:00; 누락/중복 구간

## 연결시험

SCH-02가 소비하는 구간을 EP와 대조

## 정밀도 정책

단위가 있는 수치는 변수별 atol/rtol을 계약에 고정하고 0 부근은 절대오차로 판정. 분기·ID·배열길이·타임스탬프는 정확 일치. 필요하면 원소/상태전이별 오차를 별도 제한. 하나의 포괄적인 온도/W/kg-kg 허용오차를 공유하지 않는다.

## 제출 증거

원본 파일/심벌/행구간과 입력해시; C++ reference wrapper 또는 검증된 EP trace; 단위 비교결과; 연결 trace; 생산경로 EP/fixture 주입 부재; 테스트 명령·실행 커밋·실패 재현자료.

## 종료 체크

- [x] 원본 범위와 입출력·변경상태 계약 확정
- [ ] 단위시험 통과 및 실제 활성 분기 확인
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋: `7047f3e486ee964d983db50d1107a590db69fbe3`, `47c0dedbfe296e86075671dc112d9c81bf3956cf`, `df865d96a56c36b6dfc3011994c0525c351ecce0`

시험 명령: 실제 argv·실행 소스 snapshot·종료 코드·stdout/stderr는 아래 보관 manifest의 각 `receipt.json`에 기록했다.

증거 경로: [기존 구현 비교](../evidence/SCH-01-baseline/README.md), [수정 후 비교와 생산 인계](../evidence/SCH-01-corrected/README.md)

최대오차/RMSE/상태 불일치: 비교된 수치의 최대 절대오차 0, 이산 불일치 0. RMSE는 별도 수집하지 않았다. 미가용 사례는 비교 통과에 포함하지 않았다.

추가 검토할 helper: SCH-02 평가·SCH-03 조회와 시간 전진은 별도 카드 범위다.

## SCH-01 착수 시 원본 범위 보완

EP 기준은 26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`이다. 생산 입력은 기존 CON-01 계약의 A/B IDF와 동일한 바이트를 사용한다. 진단용 잘못된 Constant/Compact/TypeLimits 입력은 선정 parser 분기를 확인하기 위한 것이며 생산 지원 범위를 늘리지 않는다.

`ScheduleManager.cc`의 선정 범위는 초기화와 저장소 helper 171–327, 입력 함수의 선언·일회 처리 guard·종류 집계 329–546, TypeLimits 724–804, Compact 1272–1527, Constant 1962–2003, 최종 검증 2197–2264이다. 직접 호출 helper는 전체 `ProcessIntervalFields` 2733–2961, `DecodeHHMMField` 2963–3073, `ProcessForDayTypes` 3075–3249로 고정한다. 날짜 helper는 `General.cc` 395–737, 숫자 helper는 `UtilityRoutines.cc` 94–173이다. 입력 순서는 실제 InputProcessor의 IDF 순서 처리 경로를 따른다. 이 범위 표시는 `ProcessScheduleInput`의 다른 스케줄 종류까지 검증했다는 뜻이 아니다.

관찰 대상은 TypeLimits의 실제 제한·숫자 종류·단위, 등록 순서와 실제 ID/name map, Constant 값, Compact 기간·요일·시간·보간모드, 실제 생성된 day/week 연결과 초기화 상태다. 원본이 사용하는 누락 요일의 실제 zero day owner와 윤일 복사도 포함한다. 수동 관찰에서는 사용 여부를 바꾸는 `GetSchedule`/`GetScheduleNum`을 호출하지 않는다. Native의 built-in ID, 사용자 ID, 보조 day/week ID를 보존하며 Rust와 다른 ID를 이름만으로 숨기지 않는다. 구체적인 대응 규칙은 실행 전 비교 계약에 고정한다.

Compact의 TypeLimits 검사는 원본이 채운 실제 timestep 배열에 의존한다. 이 초기화 의존성과 SCH-02가 소비하는 상태 인계는 SCH-01에서 관찰하되, SCH-02 전체 평가·분 단위 보간·시간 전진 계산의 완료로 계산하지 않는다. Rust 구현은 기존 parser와 runtime을 재사용하며 compiler에서 runtime으로 순환 의존을 만들지 않는다. 실제 수정 방식은 원본 실행과 기존 Rust의 차이를 확인한 뒤 결정한다.

호출 순서는 실제 constant 초기화, 선언된 timestep 문맥, 실제 IDF 입력 처리, 실제 `ProcessScheduleInput`이다. 입력 단계가 중단되면 스케줄 처리를 강제로 계속하지 않는다. 실제 반환, `EnergyPlus::FatalError`, 기타 원본 예외, 도구 자체의 오류를 구분하고 중단 시점의 부분 상태와 진단을 남긴다. 유효한 성공 출력의 필수 값은 양쪽 누락과 한쪽 누락 모두 실패로 처리한다. 사전에 선언한 중단·미호출·부분 상태의 미가용 값은 PASS에 포함하지 않으며 호출 결과 비교와 분리한다.

Rust 입력 변환에는 기존 생산 경로의 실제 IDF→epJSON 변환기를 사용한다. 원본 IDF, 변환 도구, 실제 명령·종료·로그, 변환된 입력 해시와 선언 순서를 함께 기록한다. 변환 실패 시 시험용 정답 JSON을 만들지 않는다. EP 계산 결과는 비교 기준으로만 사용하며 Rust 생산 계산에 주입하지 않는다.

## 고정 계약과 실제 수정 후 검증

실행 전에 입력 82개와 요청 작업 164개, 실제 ID 대응·owner·부분 상태·가용성 정책, 변수별 atol/rtol=0과 class/shape 정확 일치를 고정했다. CON-01 생산 입력·기상·메타데이터 45개 해시는 유지했다. 원본 소스와 직접 helper, 관찰·변환·비교 소스를 검토했으므로 범위 확정 항목만 통과로 갱신한다.

기존 구현의 실제 비교에서 확인한 선언 순서·ID, 단위 owner와 Compact 허용·경고·검증 차이를 수정했다. 수정 후 단위 관찰 비교는 수치 21,508,632개와 이산 716,890개에서 불일치 0개다. 실제 Native가 비정상 종료하여 결과가 없는 두 사례의 필수 가용성 실패 2개와, 실제 변환기가 epJSON을 남기지 않은 세 사례는 그대로 남았다. 원본 결과나 예상 배열을 Rust 입력으로 주입하지 않았다.

실제 CLI를 다시 빌드하고 A-24H, A-72H, B-NOLIMIT-24H, B-FLOW-24H, B-CAPACITY-24H, B-BOTH-24H를 실행했다. 선정한 실제 초기화 owner와 cache 인계 비교는 수치 10,119,332개, 이산 316,842개에서 불일치와 필수 누락이 모두 0개다. 비교 소스 검토 뒤 새로 실행한 두 번째 생산 묶음과 비교 보고서를 최종 근거로 사용한다. 첫 번째 실제 비교의 빈 Compact family 처리 오류와 종료 1 기록, 첫 번째 생산 묶음도 보존했다.

Dense cache 1,824개 값은 class/shape만 확인했으며 수치 비교에 포함하지 않았다. 개별 조회의 호출 시각, SCH-02/03 계산, 실행하지 않은 나머지 생산 입력 9개는 인증하지 않았다. 단위·연결·생산 gate와 카드 전체 완료는 계속 미확인이다.

실제 품질 검사에서 compiler/raw/runtime 단위시험 3,500개, 수정 후 compiler 재검사 487개, 생산 연결 후 ep_run 단위시험 779개가 통과했다. 해당 패키지의 Clippy와 전체 fmt 검사도 종료 0이다. 앞선 컴파일·시험·format 실패 기록과 수정 전후 소스 snapshot을 함께 보존했으며, 실행 당시 서로 다른 commit/dirty 상태를 단일 clean 실행으로 합쳐 주장하지 않는다.
