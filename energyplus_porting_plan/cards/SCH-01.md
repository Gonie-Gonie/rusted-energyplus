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

- [ ] 원본 범위와 입출력·변경상태 계약 확정
- [ ] 단위시험 통과 및 실제 활성 분기 확인
- [ ] 상태·시간·호출순서를 포함한 연결시험 통과
- [ ] 생산 경로 연결·EP/fixture 주입 부재·선행 gate 확인

구현 커밋:  
시험 명령:  
증거 경로:  
최대오차/RMSE/상태 불일치:  
추가 검토할 helper:  

## SCH-01 착수 시 원본 범위 보완

EP 기준은 26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`이다. 생산 입력은 기존 CON-01 계약의 A/B IDF와 동일한 바이트를 사용한다. 진단용 잘못된 Constant/Compact/TypeLimits 입력은 선정 parser 분기를 확인하기 위한 것이며 생산 지원 범위를 늘리지 않는다.

`ScheduleManager.cc`의 선정 범위는 초기화와 저장소 helper 171–327, 입력 함수의 선언·일회 처리 guard·종류 집계 329–546, TypeLimits 724–804, Compact 1272–1527, Constant 1962–2003, 최종 검증 2197–2264이다. 직접 호출 helper는 전체 `ProcessIntervalFields` 2733–2961, `DecodeHHMMField` 2963–3073, `ProcessForDayTypes` 3075–3249로 고정한다. 날짜 helper는 `General.cc` 395–737, 숫자 helper는 `UtilityRoutines.cc` 94–173이다. 입력 순서는 실제 InputProcessor의 IDF 순서 처리 경로를 따른다. 이 범위 표시는 `ProcessScheduleInput`의 다른 스케줄 종류까지 검증했다는 뜻이 아니다.

관찰 대상은 TypeLimits의 실제 제한·숫자 종류·단위, 등록 순서와 실제 ID/name map, Constant 값, Compact 기간·요일·시간·보간모드, 실제 생성된 day/week 연결과 초기화 상태다. 원본이 사용하는 누락 요일의 실제 zero day owner와 윤일 복사도 포함한다. 수동 관찰에서는 사용 여부를 바꾸는 `GetSchedule`/`GetScheduleNum`을 호출하지 않는다. Native의 built-in ID, 사용자 ID, 보조 day/week ID를 보존하며 Rust와 다른 ID를 이름만으로 숨기지 않는다. 구체적인 대응 규칙은 실행 전 비교 계약에 고정한다.

Compact의 TypeLimits 검사는 원본이 채운 실제 timestep 배열에 의존한다. 이 초기화 의존성과 SCH-02가 소비하는 상태 인계는 SCH-01에서 관찰하되, SCH-02 전체 평가·분 단위 보간·시간 전진 계산의 완료로 계산하지 않는다. Rust 구현은 기존 parser와 runtime을 재사용하며 compiler에서 runtime으로 순환 의존을 만들지 않는다. 실제 수정 방식은 원본 실행과 기존 Rust의 차이를 확인한 뒤 결정한다.

호출 순서는 실제 constant 초기화, 선언된 timestep 문맥, 실제 IDF 입력 처리, 실제 `ProcessScheduleInput`이다. 입력 단계가 중단되면 스케줄 처리를 강제로 계속하지 않는다. 실제 반환, `EnergyPlus::FatalError`, 기타 원본 예외, 도구 자체의 오류를 구분하고 중단 시점의 부분 상태와 진단을 남긴다. 유효한 성공 출력의 필수 값은 양쪽 누락과 한쪽 누락 모두 실패로 처리한다. 사전에 선언한 중단·미호출·부분 상태의 미가용 값은 PASS에 포함하지 않으며 호출 결과 비교와 분리한다.

Rust 입력 변환에는 기존 생산 경로의 실제 IDF→epJSON 변환기를 사용한다. 원본 IDF, 변환 도구, 실제 명령·종료·로그, 변환된 입력 해시와 선언 순서를 함께 기록한다. 변환 실패 시 시험용 정답 JSON을 만들지 않는다. EP 계산 결과는 비교 기준으로만 사용하며 Rust 생산 계산에 주입하지 않는다.

이 변경은 착수 시 문서 범위를 보완한 것이다. 입력 묶음·관찰 필드·수치 정밀도·사례 수는 아직 최종 고정하지 않았고 새 수치 실행이나 PASS를 주장하지 않는다. 네 완료 항목은 모두 미확인으로 유지한다. 실제 명령과 결과를 확보한 뒤 각각 갱신한다.
