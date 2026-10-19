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

## 보존한 상세 범위·정밀도 계약

EP 기준은 26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`이다. 생산 입력은 기존 CON-01 계약의 A/B IDF와 동일한 바이트를 사용한다. 진단용 잘못된 Constant/Compact/TypeLimits 입력은 선정 parser 분기를 확인하기 위한 것이며 생산 지원 범위를 늘리지 않는다.

`ScheduleManager.cc`의 선정 범위는 초기화와 저장소 helper 171–327, 입력 함수의 선언·일회 처리 guard·종류 집계 329–546, TypeLimits 724–804, Compact 1272–1527, Constant 1962–2003, 최종 검증 2197–2264이다. 직접 호출 helper는 전체 `ProcessIntervalFields` 2733–2961, `DecodeHHMMField` 2963–3073, `ProcessForDayTypes` 3075–3249로 고정한다. 날짜 helper는 `General.cc` 395–737, 숫자 helper는 `UtilityRoutines.cc` 94–173이다. 입력 순서는 실제 InputProcessor의 IDF 순서 처리 경로를 따른다. 이 범위 표시는 `ProcessScheduleInput`의 다른 스케줄 종류까지 검증했다는 뜻이 아니다.

관찰 대상은 TypeLimits의 실제 제한·숫자 종류·단위, 등록 순서와 실제 ID/name map, Constant 값, Compact 기간·요일·시간·보간모드, 실제 생성된 day/week 연결과 초기화 상태다. 원본이 사용하는 누락 요일의 실제 zero day owner와 윤일 복사도 포함한다. 수동 관찰에서는 사용 여부를 바꾸는 `GetSchedule`/`GetScheduleNum`을 호출하지 않는다. Native의 built-in ID, 사용자 ID, 보조 day/week ID를 보존하며 Rust와 다른 ID를 이름만으로 숨기지 않는다. 구체적인 대응 규칙은 실행 전 비교 계약에 고정한다.

Compact의 TypeLimits 검사는 원본이 채운 실제 timestep 배열에 의존한다. 이 초기화 의존성과 SCH-02가 소비하는 상태 인계는 SCH-01에서 관찰하되, SCH-02 전체 평가·분 단위 보간·시간 전진 계산의 완료로 계산하지 않는다. Rust 구현은 기존 parser와 runtime을 재사용하며 compiler에서 runtime으로 순환 의존을 만들지 않는다. 실제 수정 방식은 원본 실행과 기존 Rust의 차이를 확인한 뒤 결정한다.

호출 순서는 실제 constant 초기화, 선언된 timestep 문맥, 실제 IDF 입력 처리, 실제 `ProcessScheduleInput`이다. 입력 단계가 중단되면 스케줄 처리를 강제로 계속하지 않는다. 실제 반환, `EnergyPlus::FatalError`, 기타 원본 예외, 도구 자체의 오류를 구분하고 중단 시점의 부분 상태와 진단을 남긴다. 유효한 성공 출력의 필수 값은 양쪽 누락과 한쪽 누락 모두 실패로 처리한다. 사전에 선언한 중단·미호출·부분 상태의 미가용 값은 PASS에 포함하지 않으며 호출 결과 비교와 분리한다.

Rust 입력 변환에는 기존 생산 경로의 실제 IDF→epJSON 변환기를 사용한다. 원본 IDF, 변환 도구, 실제 명령·종료·로그, 변환된 입력 해시와 선언 순서를 함께 기록한다. 변환 실패 시 시험용 정답 JSON을 만들지 않는다. EP 계산 결과는 비교 기준으로만 사용하며 Rust 생산 계산에 주입하지 않는다.

실행 전에 입력 82개와 요청 작업 164개, 실제 ID 대응·owner·부분 상태·가용성 정책, 변수별 atol/rtol=0과 class/shape 정확 일치를 고정했다. CON-01 생산 입력·기상·메타데이터 45개 해시는 유지했다. 원본 소스와 직접 helper, 관찰·변환·비교 소스를 검토했으므로 범위 확정 항목만 통과로 갱신한다.

## 최종 네 gate 근거

| Gate | 범위별 Root 판정 |
|---|---|
| scope_review | A: 통과 / B: 통과 |
| unit_gate | A: 통과 / B: 통과 |
| integration_gate | A: 통과 / B: 통과 |
| production_gate | A: 통과 / B: 통과 |

77 paired semantic outcomes(55 accepted22 rejected)+3 conversion-stage invalids+2 Original OS abnormal processes. Old availability=false/numerical afterstates stay unavailable. Current5 ingress outcomes are separate, not82/82 numeric.

실제 구현 기준은 Source59 `a2c512a246f73dc806805c6c4e277c2960fdda44`입니다. [최종 판정](../evidence/SRC-06-final-current-source-validation-60/summary.json)의 해당 카드별 범위·gate 근거와 현재 연결 결과를 참조합니다. 이전 단위 명령은 원래 근거에 보존되며 현재 바이너리 재실행으로 집계하지 않습니다.

직접 단위 근거: [SCH-01-corrected](../evidence/SCH-01-corrected/summary.json), [최종증거팩: Root60-final-actual-review 0db456e8d3ca](../evidence/SRC-06-final-current-source-validation-60/summary.json).

선택 본문의 정확한 행·원문 해시·역할과 직접 helper 근거는 해당 카드의 `evidence_by_scope.selected_source_extents` 및 최종증거팩의 본문 구간 원장에 기록합니다. 선언 행은 심볼 위치 식별에만 사용합니다.

과거 실패·미관측 값·제외 분기는 불변 근거에 남습니다. 수치 단위, 현재 연결, 보고값 정책, 물리 파일 직렬화의 판정을 합치지 않습니다.

명명한 결과 구분: 대응 의미 결과 77개(승인 55, 거부 22), 변환 단계 무효 입력 3개, 원본 비정상 프로세스 2개입니다. 후자의 수치 후상태는 계속 미가용이며, 예외·문구·부분 상태 동등성이나82/82 수치 통과를 주장하지 않습니다.
