# EnergyPlus 26.1 함수·알고리즘 포팅 작업계획

작업계획 원본 검토일: 2026-10-07

EnergyPlus 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`  
rusted-energyplus 기준: `d7b516627f421259012f3e61bc28cca831452468`


## 현재 진행 상태

최종 판정은 A **54/54**, B **64/64**입니다. 이는 고정 CON-01 입력과 각 카드에 적힌 원본 분기·정책의 판정이며 전체 EnergyPlus 동등성이 아닙니다. 각 판정은 실제 영수증과 개별 Root 판정에 연결합니다.

| 근거 | 최종 실제 결과 |
|---|---|
| 현재 Source·품질 | Source59; fmt/strict Clippy/workspace 4975 passed (2 existing ignored); retained-unit carry separately bound |
| 일반 고정15·수치 및 자체 출력 | 15 normal cases (A3/B12), 16427880 reported samples and own JSON/CSV records; tolerance failures 0, RMSE failures 0 |
| Surface 반복·warmup 인계 | 34437 scope rows; 635375 exact binary64 observations; 2 direct handoffs; 4 same-owner convection connections |
| SUR-02 격리 이력·impulse·현재항 검사 | 178회 호출, 368개 binary64 출력값 비트 일치; 입력·저장소 보존 |
| 물리·부수 출력 | 15 scoped ancillary readbacks; owned termination/layout checked; raw physical inequalities preserved; overall ancillary IO PASS=false |
| 오류 입력5·SCH01 결과 구분 | Five executed invalid outcomes: converter-stage3 and Rust structured compilation2; two Original abnormal afterstates remain unavailable; no82/82 numerical PASS |

실제 단위·연결·생산 근거와 역사적 실패는 [최종 카드별 판정](evidence/SRC-06-final-current-source-validation-60/summary.json)에 모읍니다. 변경하지 않은 단위를 새 바이너리에서 다시 실행한 것으로 세지 않습니다. Exterior170은 도출한 앞137과 직접 성공33의 결합이고 차양기하9는 Native-only입니다. SUR05의30은 전체 내부 함수가 아닌 원문 종료 분기입니다. SCH01의 두 원본 수치 후상태는 계속 미확인으로 남깁니다.

같은 코드·원인에 대한 불필요한 cohort/연간 재실행을 요구하지 않습니다. 변경 범위에 맞는 선택 검증 후 실제 fmt·Clippy·workspace를 한 번 기록하며, 현재 판정은 아래 정책·카드와 `plan.json`으로 확인합니다.

[SUR-02 보충 근거](evidence/SUR-02-isolated-history-impulse-61/summary.json)는 정적 코드 비교만 했던 명시적 단위시험을 실제 실행으로 보완합니다. 생산 코드와 기존 15개 통합 실행 결과는 동일합니다.

## 범위

A: 앞서 검토한 비공조 불투명 1존.  
B: 단일 완전혼합 존 + 직결 IdealLoads, no-OA 현열 처리, NoLimit 또는 hard-sized flow/capacity limits.

64개는 **작업 단위의 분해안**이며 EP 함수 수, 코드 완성률, 또는 전체 EnergyPlus 범위의 전수 목록이 아니다.
함수 1개를 여러 분기로 나눈 카드와 관련 보조함수를 묶은 카드가 공존하며, 통합 gate도 포함한다.
입력 범위 밖의 창호·AFN·Plant·EMS·autosizing 등은 분모에서 제외한다.
새로운 입력에서 필요한 분기가 추가되면 CON-01의 범위와 관련 카드를 확장해야 한다.

## 완료 체크의 의미

앞선 답변의 '구현 있음'과 이 계획의 '완료 인증'은 다르다.
`기존 구현 재검증`은 재사용 후보이지 새 시험 통과 판정이 아니다.
계획을 처음 가져올 때는 카드별 인증을 모두 미확인으로 시작했다.
현재 상태는 `CHECKLIST.md`와 `plan.json`, 실제 검증 근거는 각 카드의
`evidence/` 기록에서 확인한다. 원본 실행·Rust 단위 및 생산 비교와 독립 검토를
통과한 카드만 완료 상태로 갱신한다.
다음 4개 조건과 해당 경로의 선행 작업이 충족되어야 한 카드를 닫는다.

- [ ] 범위 확정: 고정 커밋의 실제 함수/분기 및 직접 helper·읽기/쓰기 필드 확정
- [ ] 단위 검증: 동일 입력·초기상태의 C++ reference와 Rust 결과·분기·변경상태 비교
- [ ] 연결 검증: 선행/후행 함수에 실제 상태를 연결하고 호출시점·이력 갱신 확인
- [ ] 생산 경로: 실제 실행에서 호출되며 EP 출력/fixture를 계산 입력으로 읽지 않음; 선행 gate 확인

행번호를 임의로 기입하지 않았다. 파일·정확한 심벌·대상 분기와 동반 선언을 제공한다.
대형 함수의 내부 블록 카드는 '범위 확정'에서 start/end 행과 helper를 추가 고정한다.
이 자료는 **작업카드 묶음**이며 모든 원본 함수 본문을 추출해 놓은 소스코드 묶음은 아니다.

## 작은 코드 범위만 읽도록 만드는 원칙

한 작업자는 그 카드의 원본 함수/분기, 필요한 선언, 선행 카드의 상태 계약만 읽는다.
전역 EnergyPlusData 객체를 그대로 Rust로 옮기는 대신 카드별 입력·변경 가능 상태만 노출한다.
upstream 계산은 이미 검증한 모듈의 결과로 받고, downstream의 계산을 해당 카드에서 재작성하지 않는다.
다만 최상위 연결 카드(SYS-01/03/05/06)는 의도적으로 관리자 호출 순서까지 읽는다.

원본 상태에 직접 대입하는 코드의 위치를 확인한다.
예를 들어 EP 26.1의 공급노드 기록은 CalcPurchAirLoads 말미에 있으며,
UpdatePurchasedAir는 ReturnPlenumIndex가 있는 경우의 플레넘 후처리를 맡는다.
no-plenum 범위에서는 그 함수의 비활성/no-op 경로를 확인하고 별도 물리 알고리즘 완료로 세지 않는다.

## 정확도·시험 정책

단위시험에는 정상, 경계 양쪽, 0, 비제로 활성, 여러 timestep 상태전이, 실패 경로를 포함한다.
배열 인덱스·분기선택·ID·타임스탬프는 정확히 비교하고,
물리값의 허용오차는 °C/W/W·m⁻²/kg·kg⁻¹/J 등을 구분해 계약에 고정한다.
EP 기준값은 시험 입력/기대값으로 사용할 수 있지만 생산 계산에 공급하지 않는다.
상태이력이 핵심인 함수는 출력 한 숫자뿐 아니라 호출 전후 상태를 비교한다.

## 추천 실행 순서

1. CON-01에서 scope와 상태 계약을 고정한다.
2. 기존 계산 kernel을 개별 단위시험으로 잠근다(PSY, SRC, RAD, SUR, ZON, HVAC).
3. CTF-01~10으로 동적 계수 생성과 독립 초기화를 완결한다.
4. 비공조 경로 A에서 표면→공기→CTF history→보고의 24시간 수직 slice를 닫는다.
5. SYS-02/03의 warmup과 첫 RunPeriod 인계, 다일·연간 경로 A를 닫는다.
6. no-limit 경로 B 후 flow/capacity/both 제한을 차례로 실제 활성화한다.
7. 시간집계 및 모든 생산경로에서의 EP/fixture 주입 부재를 확인한다.

A-only에서는 SYS-01의 HVAC-08, SYS-04의 HVAC-07 조건부 의존성을 적용하지 않는다.
B에서는 그 의존성을 적용한다.

## 파일

- `energyplus_porting_checklist.html`: 브라우저에서 검색·단계 필터·체크·진행상태 JSON 저장/불러오기
- `CHECKLIST.md`: 전체 목록
- `cards/*.md`: 64개 개별 작업카드
- `plan.json`: 기계 판독용 작업 계약·의존성·출처
- `STATE_CONTRACTS.md`: 공통 상태 계약
- `AGENT_TASK_TEMPLATE.md`: 한 카드씩 작업할 때의 지시문 틀

HTML의 진행상태는 가능한 경우 해당 브라우저 localStorage에 저장된다.
브라우저/보안설정에 따라 로컬파일 저장이 제한될 수 있으므로 JSON 내보내기를 사용한다.
HTML에는 외부 스크립트나 네트워크 요청이 없으며, EP 소스 링크만 클릭 시 열린다.
