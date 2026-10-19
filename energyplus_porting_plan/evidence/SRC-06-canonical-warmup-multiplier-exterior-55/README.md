# Warmup·multiplier·Exterior 선택 원본 비교 — 55

[실제 결과와 단계별 실행 근거](summary.json)를 요약합니다. 완료 gate는 A **14/54**, B **14/64**로 유지합니다.

- Warmup: 실제 소유자에서 RecKeep 50회·Check 19회를 이어 실행하고 상태값 45,072개를 원시 비트로 비교했습니다.
- Multiplier 2: 실제 getter로 만든 소유자에서 예측·보정·이력 21회, 상태·반환값 17,893개를 원시 비트로 비교했습니다. 생산 입력의 multiplier 허용 범위는 확장하지 않았습니다.
- Exterior: 허용 범위 170개 기록을 확인했습니다. 실패한 55-05 실행에서 순서와 본문으로 확인한 prefix 137개, 이후 성공한 internal gains 9개·surface internal gains 6개·beam 18개를 구분해 보존합니다. 원본 179개 기록 모두를 보존하며, 탐색용 차양 9개는 고정 입력 범위 밖으로 제외합니다.

이는 단계별 실행과 변경되지 않은 본문·읽기 범위의 연결 근거를 합친 증거입니다. 170개를 한 번에 성공한 실행이나 전 범주의 동일 Source·바이너리 검증을 주장하지 않습니다. Exterior의 원시 비트 분모를 추정하지 않으며, 기록 수와 함수 호출 수도 구분합니다. 이전 Source 실패 12건과 메타데이터 preflight 중단 기록은 삭제하지 않았습니다.

최종 같은 Source의 15개 일반 실행·전체 JSON/CSV·부수 출력, 실제 Surface 호출 시점과 warmup→RunPeriod MAT·W·CTF 인계 확인은 별도 검증입니다. 이번 workspace 시험도 아직 통과하지 않았습니다. 관측된 실패는 `arbitrary_run.rs:102`의 audit 두 줄에 대한 Windows CRLF와 고정 LF 기대값 차이이며, 해당 assertion 수정은 다음 통합에 남습니다.

[유한 증거 ZIP](evidence.zip)은 995개 항목, 압축 전 103,069,268바이트입니다. [항목별 SHA·크기 목록](finite-whitelist.json)으로 내용을 확인할 수 있습니다. 대형 compiler/core/provider 트리와 실행되지 않은 cadence·인계·최종 15건 결과는 이 패킷에 포함하지 않습니다.
