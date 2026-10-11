# CTF-10 — 구성체별 생성 연결·역순 재사용

- 그룹: 04 CTF 계수 생성
- 적용: A/B
- 기존 코드에 대한 작업 성격: 기존 구현 보완
- 선행 카드: CTF-02, CTF-09
- 조건부 선행 카드: {}
- EP 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`

Scope review passed. Unit, integration and production gates remain unconfirmed.

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

## Existing selected evidence

| Evidence | Confirmed scope |
| --- | --- |
| [Original baseline](../evidence/CTF-10-original-baseline/summary.json) | genuine Native custody and unchanged Rust baseline |
| [Public storage](../evidence/CTF-10-unit/summary.json) | unit39/42 direct calls; fixed15 genuine13 loaders then true whole Init;45 guards |
| [Earlier fixed6 CLI](../evidence/CTF-10-production-cli/summary.json) | pre-activation ordinary6 exit0/source-order match/nohours/nooracle; no numerical equivalence claim |
| [Exact Original pair retirement](../evidence/CTF-10-retention/summary.json) | two new10 Original paths removed with protected/core/absence guards; no registry/history rewrite |
| [Standalone surface binding](../evidence/CTF-10-surface-binding/summary.json) | actual12x19 owners, IDs, generation key and uninitialized history contract |
| [Protected Original maintenance](../evidence/CTF-10-storage-maintenance/summary.json) | four existing executables losslessly NTFS-LZX compressed; paths/logical bytes/SHA unchanged |
| [Consumer04 Original capture](../evidence/CTF-10-surface-consumer-original-baseline/summary.json) | ordinary A-24H complete lifecycle404878 events, zero omissions; factual custody only |
| [Consumer05 history boundaries](../evidence/CTF-10-surface-consumer05-original-baseline/summary.json) |404886 events,4 Init+2025 simple Update, complete allocated owners/catalog; master unreached |
| [Initial direct history comparison](../evidence/CTF-10-thermal-history-unit/summary.json) |2029 input-only cases, numeric973854/exact5671425, mismatch0; standalone API milestone |

Selected public comparisons: unit numeric948290/discrete3874496; fixed15 numeric8988/discrete57532; mismatch0.
Actual returned owners, original IDs/order/use, reverse providers, reset arrays and matching Init prefix are paired. This does not certify unpaired report/fatal backend behavior.
Native capture uses unchanged A-24H input/weather and ordinary duration. The4GiB full-file QA admission bound never caps, samples or truncates producer events.
Complete Native/projected-input/Rust bytes remain losslessly archived with full decompressed SHA/size. Native operands are restricted to same-input direct tests.

## Current ordinary producer and surface lifecycle

The actual cache/public generation result is moved and connected once, with full19 history owners associated by each surface's ConstructionId.
Generation key and compiler Source witnesses preserve material/caller/order/route/global dependencies. Unused unsupported definitions are not activated.
Zone/Space/surface membership, actual class traversal, environment/ProductionWeather clock and timestep ownership are explicit.
Selected final/history update occurs once after the existing CorrectStep air tail and before timestep_index increments. There is no legacy double history shift or numerical fallback.
Prepared caller clock validation/commit, duplicate-commit rejection, generation freshness/invalidation and typed stop propagation remain distinct.
Truly empty global0 layout permits empty-array operations only; nonempty global0 and unsupported branches retain typed stops.

Activation milestone final03: `.runtime/porting/CTF-10/production-activation-applied-final-source-03/manifest.json`, SHA `2626c66ee2fe7171cfc8102b694324959cd7b6c2121087a160c3136ee112cafb`.
Activation SourceSnapshot: `8f448858790b56e3ff7ed97109aea23be1c936ad8e94bd3c4999c016a92c7216`.
Root-confirmed quality02: fmt0, Clippy0, focused245 PASS and workspace4851 PASS/21 groups, all on that same SourceSnapshot.

Activation retained same-input replay02 matches all2029 input/API/endpoint/normal-exit/parent-return joins and available cases.
Comparison SHA `bd01b399e5a7588d94fef91726173d5ea8f6ae52dbe9b475cfa84ad5ac887de5`; numeric973854/exact5671425, mismatch0.
All13 observed owner profiles have maxAbs/l2 zero. Four master profiles remain unobserved and receive no PASS. Native was not replayed.
This confirms the direct API replay and current source quality; it does not establish persistent ordinary Rust/Native numerical equivalence.

## Post-activation fixed6 CLI and publication

Ordinary A-24H/A-72H/B-NOLIMIT-24H/B-FLOW-24H/B-CAPACITY-24H/B-BOTH-24H all exited0 on the same current SourceSnapshot.
All45 input/weather/metadata guards, compatibility/partial-deny/trace-full and original ordinary arguments were retained; no hours override, oracle or Native answers were supplied.
The [current activation evidence](../evidence/CTF-10-production-activation/summary.json) retains all six actual receipts, current quality and complete direct replay with the failure-to-fix lineage.
These ordinary runs establish regression success; persistent ordinary Rust/Native numerical equivalence remains pending.

## Report input and Scan implementation

The ordinary compiler now preserves all five report input families, unnamed IDF declaration order, physical alpha counts and exact default/blank/case facts.
General's first-scan flags/options/EMS effects, retained cache and optional queries are implemented in source order. Unknown surface-list tokens retain the EMPTY warning path.
The [report-input and Scan evidence](../evidence/CTF-10-report-input-scan/summary.json) records fmt/Clippy0, focused250 and workspace4859 PASS, including all eight new regression tests, on SourceSnapshot `95197194b3c81732677230e3699d3522f03e3890077688d85907745ebf57a723`.
Same-input genuine Native getter/cache/query comparison remains pending. The scan is not yet called by the ordinary whole-Init postlude; predefined/EIO reporting and final fatal backend composition remain pending.
This milestone promotes no card gate; the earlier activation replay and ordinary CLI results retain their own recorded source identity.

## Report output and error backend implementation

Actual compiler names, construction layers, retained predefined string entries and borrowed EIO output are implemented. Source warning/continue/fatal output, counters, optional callbacks and SQLite adapter boundaries retain their real order and output prefix on failure.
The [report output/backend evidence](../evidence/CTF-10-report-output/summary.json) records fmt/Clippy0, focused270 and workspace4880 PASS with21 added tests. The observer compilation failure and exact correction are retained. Concurrent recorder publication passed eight-process regression.
Native Scan/getter and numeric-format byte comparisons, full registry/severe producer plumbing and ordinary whole-Init composition remain pending. No card gate is promoted.

## Retained failure-to-fix lineage

The earlier public Clippy clone warning and borrowed-slice correction remain in the public-storage evidence.
Consumer03 unused Windows argc/argv compile failure: `.runtime/porting/CTF-10/surface-consumer-native-build-01/native-compile-command-01/receipt.json`; stopped before engine execution.
Consumer05 optional size-ref preflight failure: `.runtime/porting/CTF-10/surface-consumer05-original-stages-command-01/receipt.json`; stopped before PE/engine execution.
Initial activation workspace failure: `.runtime/porting/CTF-10/production-activation-workspace-command-01/receipt.json`; retained separately from empty-layout/caller-clock/fixture/unused-owner/boundary corrections.
Reviewed Source, exact before/diff, application/formatter and genuine failure receipt/log/snapshot objects are retained as explicit lineage in the current activation archive. Success receipts never overwrite failures.

## Remaining scope and completion checks

- [x] Original scope and input/output/mutated-owner contracts fixed
- [ ] Whole unit gate: remaining whole Init/helper/report/fatal evidence; selected cache-invalidation regressions already passed in current quality
- [ ] Integration gate: ordinary Rust/Native time/state/call-order and persistent-history numerical pairing
- [ ] Production gate: ordinary path, prerequisite gates and absence of EP/fixture answer injection

Whole Init reportLayers/ScanForReports and final-fatal backend custody cannot be replaced by the selected Init prefix.
Unobserved general/master history arithmetic, CON-01-excluded surface/material families and unavailable/excluded paths have no PASS.
Existing evidence stays immutable. Source activation, quality, direct replay and ordinary CLI success alone do not promote the three gates or complete CTF-10.
