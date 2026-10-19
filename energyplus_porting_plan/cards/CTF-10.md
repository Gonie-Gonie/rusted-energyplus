# CTF-10 — 구성체별 생성 연결·역순 재사용

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: CTF-02, CTF-09
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

## Pinned Original scope and contracts

Original symbols: `HeatBalanceManager.cc::InitConductionTransferFunctions` and `Construction.cc::calculateTransferFunction`.
[Pinned Original](https://github.com/NatLabRockies/EnergyPlus/blob/6f2e40d10250a105b49966baa24d843711e61048/src/EnergyPlus/HeatBalanceManager.cc).
The scope connects actual construction traversal/use/layer order, duplicate/reverse providers and independent generated owners. CTF01-09 arithmetic is reused through real typed owners.

| Pinned file and lines | Role |
| --- | --- |
| Construction.cc146-194 / 309-315 | reset, unused/used, timestep and shared loading-error return |
| Construction.cc405-485 / 1908-1984 | actual original-ID reverse provider/copy and companion helper |
| Construction.cc875-981 / 985-1077 / 981-1103 | retry terminal boundary, all-resistive and common fixed19 public stores |
| Construction.hh146-190 | fixed public coefficient owner declarations |
| HeatBalanceManager.cc6153-6202 / 2617-2630; .hh175-182 | whole Init call/aggregate/report/fatal order, caller and declaration |
| DataHeatBalance.hh1863-1875 (pinned1870-1872 included) | incoming global defaults and state declarations |
| General.cc1130-1469; .hh332-347 | real ScanForReports and declaration |
| InputProcessor.cc115-128 / 487-532; .hh325-343 | directly called input/report helper |
| OutputReportPredefined.cc228-239 / 1837-1875; IOFiles.cc247-265 | report/file prerequisites |
| UtilityRoutines.cc758-794 | actual ShowFatalError boundary/helper |
| EnergyPlus/CMakeLists.txt140-148 / 377-385 | original target/provider declarations |
| HeatBalanceSurfaceManager.cc2322-2436 / 5221-5496 | selected history input, initialization, update and completion boundaries |

Literal range SHA/size pins remain in plan `source_ranges_input_only` and the existing reviewed Source packets. These ranges do not claim whole-file port completion.
Inputs are actual construction/material/layer IDs, use, incoming flags and timestep. Outputs are independently generated public owners by original ConstructionId and ordered initialization outcomes.
All twelve public arrays retain19 slots. NumCTFTerms/NumHistories, timestep/UValue, reverse provider and shared flag lifetimes remain distinct.
The selected Rust fixed19 owner represents inclusive term0..18. PublicCapacity is a Rust limitation, not a Native guard, safe-store inference or return outcome.
Construction FatalNoCtfs stops before its public stores and its caller's postreturn aggregate; severe break reaches stores.
Whole Init final ErrorsFound fatal occurs after returned-call aggregates, reportLayers, ScanForReports and any reached conditional reporting. Missing return, unavailable, excluded and unrepresentable owners cannot become zero or PASS.
CON-01 exclusions are retained; this card does not expand unsupported surface/material families.

## Frozen numerical and custody policies

Six public profiles are CTFOutside/CTFCross/CTFInside/CTFFlux, CTFTimeStep and UValue: atol=rtol=0, exact class/shape/signed zero.
The eight source arrays reset to positive-zero bits. Existing CTF01-09 profiles retain their separate meanings.
The direct history-unit policy fixes17 owner profiles at atol=rtol=0. Retained-copy bits, IDs, order, allocations, counters and clock/call boundaries are exact.
Unavailable, unreached, nonfinite arithmetic and unobserved master branches receive no numerical PASS. Native equations and branch outcomes are not recomputed.

기존 카드의 추가 고정 원본 구간(완료 범위를 확대하지 않음):

`Data/EnergyPlusData.cc840–857`, `EMSManager.cc208–247`, `SimulationManager.cc622–705,801–944,1359–1507`, `Material.cc198–226,275–309`, `HeatBalanceManager.cc1390–1397,1497–1513,1777–1797`, `DataHeatBalance.cc893–982`, `SurfaceGeometry.cc641–669,1423–1530`, `OutputReportPredefined.cc1766–1835,1928–1973`, `HeatBalanceSurfaceManager.cc163-189`.

## 최종 네 gate 근거

| Gate | 범위별 Root 판정 |
|---|---|
| scope_review | A: 통과 / B: 통과 |
| unit_gate | A: 통과 / B: 통과 |
| integration_gate | A: 통과 / B: 통과 |
| production_gate | A: 통과 / B: 통과 |

Declared constructions/use/order/reverse/public19 ownership plus actual Init postlude. EC finite reverse/unused/two-owner/generation/cache rejection accepted, not full transitive cache certification. Public-store numeric module raw-identical; actual flag/reset/postlude extensions are semantic owner carry.

실제 구현 기준은 Source59 `a2c512a246f73dc806805c6c4e277c2960fdda44`입니다. [최종 판정](../evidence/SRC-06-final-current-source-validation-60/summary.json)의 해당 카드별 범위·gate 근거와 현재 연결 결과를 참조합니다. 이전 단위 명령은 원래 근거에 보존되며 현재 바이너리 재실행으로 집계하지 않습니다.

직접 단위 근거: [CTF-10-unit](../evidence/CTF-10-unit/summary.json), [최종증거팩: final64-existing-evidence-adjudication-Audit-03 b9f4ff21aee9](../evidence/SRC-06-final-current-source-validation-60/summary.json), [CTF-10-thermal-history-unit](../evidence/CTF-10-thermal-history-unit/summary.json), [CTF-10-report-scan-comparison](../evidence/CTF-10-report-scan-comparison/summary.json), [CTF-10-init-postlude-preparation-comparison](../evidence/CTF-10-init-postlude-preparation-comparison/summary.json), [CTF-10-ordinary-history-comparison](../evidence/CTF-10-ordinary-history-comparison/summary.json), [CTF-10-main-thread-history-comparison](../evidence/CTF-10-main-thread-history-comparison/summary.json).

선택 본문의 정확한 행·원문 해시·역할과 직접 helper 근거는 해당 카드의 `evidence_by_scope.selected_source_extents` 및 최종증거팩의 본문 구간 원장에 기록합니다. 선언 행은 심볼 위치 식별에만 사용합니다.

과거 실패·미관측 값·제외 분기는 불변 근거에 남습니다. 수치 단위, 현재 연결, 보고값 정책, 물리 파일 직렬화의 판정을 합치지 않습니다.
