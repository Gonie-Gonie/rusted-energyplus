# EnergyPlus 26.1 함수·알고리즘 포팅 작업계획

검토일: 2026-10-07  
EnergyPlus 기준: v26.1.0 / `6f2e40d10250a105b49966baa24d843711e61048`  
rusted-energyplus 기준: `d7b516627f421259012f3e61bc28cca831452468`

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
계획 반입 시에는 모든 인증 체크를 미확인으로 시작했다. 이후 각 완료 행은 고정 원본·실제 실행·독립 검토 증거를 반영한다.
범위 밖의 알고리즘이나 전체 EnergyPlus 동등성은 해당 카드의 부분 검증으로 승격하지 않는다.
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


## 전체 체크리스트

CTF-10의 [현재 Init 비교 증거](evidence/CTF-10-init-postlude-preparation-comparison/summary.json)는 동일한 15개 모델·6개 직접 상태 사례에서 수치 2,106개·상태/보고서 696,389개를 비교해 불일치 0개를 기록했습니다. 전체 4,932개 테스트와 추가 관측기 4개도 통과했습니다. 선택 범위 밖 132개 항목은 미확인으로 유지하며, 전체 초기화와 지속 실행 이력 인계는 아직 완료되지 않았습니다. 이전 829개 불일치와 수정 기록을 보존했고 A14/54·B14/64 완료 수는 변하지 않습니다.


### 00 범위·계약
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [x] | [CON-01](cards/CON-01.md) | 대상 입력과 상태 경계 고정 | `HeatBalanceManager.cc` · `GetProjectControlData; GetHeatBalanceInput` | 없음 |

### 01 입력·기하
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [x] | [GEO-01](cards/GEO-01.md) | 좌표계와 꼭짓점 변환 | `SurfaceGeometry.cc` · `GetVertices; GetHTSurfaceData` | CON-01 |
| [x] | [GEO-02](cards/GEO-02.md) | 면적·방위·경사·중심점 | `SurfaceGeometry.cc` · `ProcessSurfaceVertices; CalcSurfaceCentroid` | GEO-01 |
| [x] | [GEO-03](cards/GEO-03.md) | 대상 존의 체적 산정 | `SurfaceGeometry.cc` · `CalculateZoneVolume` | GEO-02 |

### 02 시간·기상
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [x] | [CLK-01](cards/CLK-01.md) | 달력·RunPeriod·day type | `WeatherManager.cc` · `GetRunPeriodData; SetupWeekDaysByMonth; calculateDayOfYear; isLeapYear` | CON-01 |
| [x] | [CLK-02](cards/CLK-02.md) | EPW 헤더·시간별 레코드 해석 | `WeatherManager.cc` · `ProcessEPWHeader; InterpretWeatherDataLine` | CON-01 |
| [x] | [CLK-03](cards/CLK-03.md) | 기상 레코드 선택·Today/Tomorrow 인계 | `WeatherManager.cc` · `GetNextEnvironment; ReadEPlusWeatherForDay; UpdateWeatherData` | CLK-01, CLK-02 |
| [x] | [CLK-04](cards/CLK-04.md) | 비일사 기상값 timestep 보간 | `WeatherManager.cc` · `SetupInterpolationValues; ReadEPlusWeatherForDay; SetCurrentWeather; interpolateWindDirection` | CLK-03 |
| [x] | [CLK-05](cards/CLK-05.md) | 일사 전용 timestep 보간 | `WeatherManager.cc` · `SetupInterpolationValues; ReadEPlusWeatherForDay` | CLK-03 |
| [x] | [CLK-06](cards/CLK-06.md) | 하늘 온도·수평면 IR | `WeatherManager.cc` · `calcSky; CalcSkyEmissivity` | CLK-02, CLK-04 |

### 03 스케줄·물성
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [ ] | [SCH-01](cards/SCH-01.md) | Constant/Compact 입력 정규화 | `ScheduleManager.cc` · `ProcessScheduleInput; ProcessForDayTypes; DecodeHHMMField` | CON-01 |
| [ ] | [SCH-02](cards/SCH-02.md) | 분 단위 확장과 timestep 표 생성 | `ScheduleManager.cc` · `ProcessIntervalFields; DaySchedule::populateFromMinuteVals` | SCH-01, CLK-01 |
| [ ] | [SCH-03](cards/SCH-03.md) | 현재 스케줄 조회·갱신 시점 | `ScheduleManager.cc` · `ScheduleConstant::getHrTsVal; ScheduleDetailed::getHrTsVal; UpdateScheduleVals` | SCH-02, CLK-01 |
| [x] | [PSY-01](cards/PSY-01.md) | 기본 습공기 물성 | `Psychrometrics.hh` · `PsyCpAirFnW; PsyRhoAirFnPbTdbW; PsyHFnTdbW; PsyTdbFnHW` | CON-01 |
| [x] | [PSY-02](cards/PSY-02.md) | 포화·습구·습공기비 역산 | `Psychrometrics.cc` · `PsyTsatFnHPb; PsyWFnTdbRhPb; PsyWFnTdbH; PsyTwbFnTdbWPb` | PSY-01 |

### 04 CTF 계수 생성
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [x] | [CTF-01](cards/CTF-01.md) | 구성체 열물성 전처리 | `Construction.cc` · `ConstructionProps::calculateTransferFunction` | CON-01 |
| [ ] | [CTF-02](cards/CTF-02.md) | 전부 저항층인 경우의 CTF | `Construction.cc` · `ConstructionProps::calculateTransferFunction` | CTF-01 |
| [ ] | [CTF-03](cards/CTF-03.md) | 유질량 1D 구성체의 절점 분할 | `Construction.cc` · `ConstructionProps::calculateTransferFunction` | CTF-01 |
| [ ] | [CTF-04](cards/CTF-04.md) | 상태공간 A/B/C/D 행렬 구성 | `Construction.cc` · `ConstructionProps::calculateTransferFunction` | CTF-03 |
| [ ] | [CTF-05](cards/CTF-05.md) | CTF 행렬 역산 | `Construction.cc` · `ConstructionProps::calculateInverseMatrix` | CTF-04 |
| [ ] | [CTF-06](cards/CTF-06.md) | 행렬 지수 계산 | `Construction.cc` · `ConstructionProps::calculateExponentialMatrix` | CTF-04 |
| [ ] | [CTF-07](cards/CTF-07.md) | Gamma 행렬 계산 | `Construction.cc` · `ConstructionProps::calculateGammas` | CTF-05, CTF-06 |
| [ ] | [CTF-08](cards/CTF-08.md) | 최종 CTF 계수와 차수 | `Construction.cc` · `ConstructionProps::calculateFinalCoefficients` | CTF-07 |
| [ ] | [CTF-09](cards/CTF-09.md) | 계수 안정성 검사·timestep 재시도 | `Construction.cc` · `ConstructionProps::calculateTransferFunction` | CTF-08 |
| [ ] | [CTF-10](cards/CTF-10.md) | 구성체별 생성 연결·역순 재사용 | `HeatBalanceManager.cc` · `InitConductionTransferFunctions; ConstructionProps::calculateTransferFunction` | CTF-02, CTF-09 |

### 05 열원·외부 경계
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [ ] | [SRC-01](cards/SRC-01.md) | 내부 대류발열 | `InternalHeatGains.cc` · `InitInternalHeatGains; zoneSumAllInternalConvectionGains` | SCH-03 |
| [ ] | [SRC-02](cards/SRC-02.md) | 내부 복사발열의 표면 배분 | `HeatBalanceSurfaceManager.cc` · `ComputeIntThermalAbsorpFactors; InitIntSolarDistribution` | SRC-01, GEO-02 |
| [ ] | [SRC-03](cards/SRC-03.md) | 태양위치·태양방향 | `WeatherManager.cc` · `CalculateDailySolarCoeffs; CalculateSunDirectionCosines; DetermineSunUpDown` | CLK-01, CLK-03 |
| [ ] | [SRC-04](cards/SRC-04.md) | 외표면 입사·흡수 일사 | `SolarShading.cc` · `FigureSolarBeamAtTimestep; AnisoSkyViewFactors; CalcAbsorbedOnExteriorOpaqueSurfaces` | SRC-03, CLK-05, GEO-02 |
| [ ] | [SRC-05](cards/SRC-05.md) | 표면 높이의 외기조건 | `DataSurfaces.cc` · `SetSurfaceOutBulbTempAt; SetSurfaceWindSpeedAt; SetSurfaceWindDirAt` | CLK-04, GEO-02 |
| [ ] | [SRC-06](cards/SRC-06.md) | TARP 실내 대류 | `ConvectionCoefficients.cc` · `CalcASHRAETARPNatural; CalcASHRAEDetailedIntConvCoeff; InitIntConvCoeff` | GEO-02, PSY-01 |
| [ ] | [SRC-07](cards/SRC-07.md) | 실외 대류계수·습윤 경계 | `ConvectionCoefficients.cc` · `InitExtConvCoeff; CalcASHRAESimpExtConvCoeff` | SRC-05, PSY-01 |
| [ ] | [SRC-08](cards/SRC-08.md) | 실외 장파복사 선형화 | `ConvectionCoefficients.cc` · `InitExtConvCoeff` | CLK-06, SRC-05, GEO-02 |

### 06 실내 장파복사
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [ ] | [RAD-01](cards/RAD-01.md) | 근사 view-factor 행렬 | `HeatBalanceIntRadExchange.cc` · `CalcApproximateViewFactors` | GEO-02 |
| [ ] | [RAD-02](cards/RAD-02.md) | view-factor 보정 | `HeatBalanceIntRadExchange.cc` · `FixViewFactors` | RAD-01 |
| [ ] | [RAD-03](cards/RAD-03.md) | ScriptF 생성 | `HeatBalanceIntRadExchange.cc` · `CalcScriptF; CalcMatrixInverse` | RAD-02 |
| [ ] | [RAD-04](cards/RAD-04.md) | 표면 간 장파 교환 | `HeatBalanceIntRadExchange.cc` · `CalcInteriorRadExchange` | RAD-03 |

### 07 표면 열수지
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [ ] | [SUR-01](cards/SUR-01.md) | 표면온도·열유속 이력 초기화 | `HeatBalanceSurfaceManager.cc` · `InitThermalAndFluxHistories` | CTF-10, GEO-02 |
| [ ] | [SUR-02](cards/SUR-02.md) | 과거 CTF 항 합산 | `HeatBalanceSurfaceManager.cc` · `InitSurfaceHeatBalance` | SUR-01 |
| [ ] | [SUR-03](cards/SUR-03.md) | 외기 접촉면의 외표면온도 | `HeatBalanceSurfaceManager.cc` · `CalcOutsideSurfTemp` | SUR-02, SRC-04, SRC-07, SRC-08 |
| [ ] | [SUR-04](cards/SUR-04.md) | 내표면 방정식과 단열 경계 | `HeatBalanceSurfaceManager.cc` · `CalcHeatBalanceInsideSurf2CTFOnly; CalcHeatBalanceOutsideSurf` | SUR-02, SRC-02, SRC-06, RAD-04 |
| [ ] | [SUR-05](cards/SUR-05.md) | 실내 표면 반복·수렴 | `HeatBalanceSurfaceManager.cc` · `CalcHeatBalanceInsideSurf2CTFOnly` | SUR-03, SUR-04 |
| [ ] | [SUR-06](cards/SUR-06.md) | 확정 열유속·CTF 이력 commit | `HeatBalanceSurfaceManager.cc` · `UpdateThermalHistories` | SUR-05 |
| [ ] | [SUR-07](cards/SUR-07.md) | 표면 열수지 출력 | `HeatBalanceSurfaceManager.cc` · `UpdateIntermediateSurfaceHeatBalanceResults; ReportSurfaceHeatBalance` | SUR-06 |

### 08 존 공기
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [x] | [ZON-01](cards/ZON-01.md) | 존 공기 상태 초기화 | `ZoneTempPredictorCorrector.cc` · `ZoneSpaceHeatBalanceData::beginEnvironmentInit` | CON-01, GEO-03 |
| [ ] | [ZON-02](cards/ZON-02.md) | 존 열수지 계수와 공기 열용량 | `ZoneTempPredictorCorrector.cc` · `ZoneHeatBalanceData::calcSumHAT; ZoneSpaceHeatBalanceData::calcZoneOrSpaceSums; predictSystemLoad` | ZON-01, SUR-05, SRC-01, PSY-01 |
| [ ] | [ZON-03](cards/ZON-03.md) | ThirdOrder·이중 설정온도 부하예측 | `ZoneTempPredictorCorrector.cc` · `ZoneSpaceHeatBalanceData::predictSystemLoad; calcPredictedSystemLoad; CalcZoneAirTempSetPoints` | ZON-02, SCH-03 |
| [ ] | [ZON-04](cards/ZON-04.md) | 공기온도 corrector | `ZoneTempPredictorCorrector.cc` · `ZoneSpaceHeatBalanceData::correctAirTemp; correctZoneAirTemps` | ZON-02 |
| [ ] | [ZON-05](cards/ZON-05.md) | 현재 범위의 수분수지 | `ZoneTempPredictorCorrector.cc` · `ZoneSpaceHeatBalanceData::correctHumRat` | ZON-01, PSY-01 |
| [ ] | [ZON-06](cards/ZON-06.md) | zone/system 온습도 이력 관리 | `ZoneTempPredictorCorrector.cc` · `pushZoneTimestepHistory; pushSystemTimestepHistory; revertZoneTimestepHistory` | ZON-04, ZON-05 |

### 09 IdealLoads
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [ ] | [HVAC-01](cards/HVAC-01.md) | 설비·존·노드·스케줄 결합 | `PurchasedAirManager.cc` · `GetPurchasedAir; SimPurchasedAir` | CON-01, SCH-01 |
| [ ] | [HVAC-02](cards/HVAC-02.md) | 초기화 lifecycle·hard-size 제한 | `PurchasedAirManager.cc` · `InitPurchasedAir; SizePurchasedAir` | HVAC-01, PSY-01, SCH-03 |
| [ ] | [HVAC-03](cards/HVAC-03.md) | 매호출 reset·가용성·운전모드 | `PurchasedAirManager.cc` · `CalcPurchAirLoads` | HVAC-02, ZON-03 |
| [ ] | [HVAC-04](cards/HVAC-04.md) | no-OA 난방·유량/용량 제한 | `PurchasedAirManager.cc` · `CalcPurchAirLoads; CalcPurchAirMixedAir` | HVAC-03, PSY-01 |
| [ ] | [HVAC-05](cards/HVAC-05.md) | no-OA 냉방·유량/용량 제한 | `PurchasedAirManager.cc` · `CalcPurchAirLoads; CalcPurchAirMixedAir` | HVAC-03, PSY-01, PSY-02 |
| [ ] | [HVAC-06](cards/HVAC-06.md) | Calc 말미의 출력·노드 commit | `PurchasedAirManager.cc` · `CalcPurchAirLoads; UpdatePurchasedAir` | HVAC-04, HVAC-05 |
| [ ] | [HVAC-07](cards/HVAC-07.md) | 냉난방 rate·연료·energy 보고 | `PurchasedAirManager.cc` · `ReportPurchasedAir` | HVAC-06, SCH-03 |
| [ ] | [HVAC-08](cards/HVAC-08.md) | 공급공기를 존 corrector에 피드백 | `ZoneTempPredictorCorrector.cc` · `ZoneSpaceHeatBalanceData::calcZoneOrSpaceSums` | HVAC-06, ZON-02 |

### 10 실행·검증
| 완료 | ID | 작업 | EP 파일·심벌 | 선행 |
|---|---|---|---|---|
| [ ] | [SYS-01](cards/SYS-01.md) | 표면·공기·HVAC·시간간격 연결 | `HVACManager.cc` · `ManageHVAC; SimHVAC; SimSelectedEquipment` | SUR-05, ZON-06; B: HVAC-08 |
| [ ] | [SYS-02](cards/SYS-02.md) | warmup 최고/최저온도·냉난방부하 수렴 | `HeatBalanceManager.cc` · `RecKeepHeatBalance; CheckWarmupConvergence` | SYS-01 |
| [ ] | [SYS-03](cards/SYS-03.md) | warmup→RunPeriod 상태 인계 | `HeatBalanceManager.cc` · `ManageHeatBalance; InitHeatBalance; CheckWarmupConvergence` | SYS-02, CLK-03, SCH-03, SUR-06 |
| [ ] | [SYS-04](cards/SYS-04.md) | 요청 출력과 meter의 수치 집계 | `OutputProcessor.cc` · `UpdateDataandReport` | SUR-07, SYS-01; B: HVAC-07 |
| [ ] | [SYS-05](cards/SYS-05.md) | 비공조 1존 수직 통합시험 | `HeatBalanceSurfaceManager.cc` · `ManageSurfaceHeatBalance` | SYS-03, SYS-04, CTF-10 |
| [ ] | [SYS-06](cards/SYS-06.md) | 직결 IdealLoads 수직 통합시험 | `PurchasedAirManager.cc` · `SimPurchasedAir` | SYS-05, HVAC-07, HVAC-08 |

CTF/SUR Source 진행: 원본 순서의 내부면 식과 이력 관측을 적용했다. fmt/Clippy, 관련316개·전체4,940개 시험 통과(실패·무시0). [실제 적용·검증 기록](evidence/CTF-10-ordinary-history-capture-source/summary.json). Native 경계 수치 비교와 남은 게이트는 미확인이다.

CTF-10 관측 예제를 기존 A 범위 진입 함수에 연결했습니다. Deny와 입력·계산·비교 정책을 유지했고, 관련316개/전체4,940개 테스트가 통과했습니다. [실제 보정·검증 기록](evidence/CTF-10-ordinary-history-capture-source/scoped-caller-and-quality.json). 일반 실행의 이력 도달·수치 비교는 아직 미확인입니다.

Historical CTF-10 observer capture used the then-current CLI 8 MiB worker stack with strict A/Deny. Focused316/workspace4,940 tests passed with zero failed/ignored. [Historical entry and quality](evidence/CTF-10-ordinary-history-capture-source/entry-and-quality.json). Ordinary reachability and numerical comparison were not verified by that capture.

CTF-10 genuine A ordinary history capture completed (578 events, runtime exit0); full comparison retained FAIL (192 paired stages,3,898 required unavailable/unmatched,35,031 mismatches). [Full evidence and remaining issues](evidence/CTF-10-ordinary-history-comparison/summary.json). Card gates remain unchanged.

SYS-03 uncontrolled warmup owner applied in strict ordinary A; focused323/workspace4,947 tests passed. [Source and quality](evidence/SYS-03-warmup-owner/summary.json). Native numerical/handoff parity and report/backend obligations remain pending; gates unchanged.

SYS-02 selected daily warmup methods: genuine Native/Rust25-case comparison matched10,785 input/initial checks and7,383 state comparisons with zero mismatches. [Full evidence](evidence/SYS-02-selected-warmup/summary.json). Rolling/report/backend/producer and ordinary handoff remain pending; gates unchanged.

SYS-02 selected daily and six rolling warmup methods: genuine Native/Rust25-case comparison matched10,953 input/initial checks and10,023 state comparisons with zero mismatches. [Full evidence](evidence/SYS-02-rolling-warmup/summary.json). Report/backend/producer and ordinary handoff remain pending; gates unchanged.

SRC-05 actual selected surface environment comparison: fail-selected-setters-or-input-join, 26 cases/30 weather operations, 3 mismatches; complete required-unavailable ledgers retained. [Full evidence](evidence/SRC-05-surface-environment/summary.json). Cold guard/backend/ordinary handoff and whole-card gates remain pending; gates unchanged.

SYS-03 R prerequisite actual status: pass-selected-615-available-exact-texts, 124 values/615 required text pairs, 0 mismatches; five positive-infinity rows per lane explicitly unavailable. [Full evidence](evidence/SYS-03-round-format/summary.json). Full report/backend/ordinary warmup and whole-card gates remain pending; gates unchanged.

SYS-03 selected report owners/EIO: genuine 25-case comparison matched 371 boundaries/14 Report calls/210,245 report endpoint checks with zero mismatches (historical Source d362e54d). Current test-only-corrected ordinary A Source passed fmt/Clippy, targeted 1, focused 341/workspace 4,968 (ignored 0/2/2). [Full evidence](evidence/SYS-03-ordinary-warmup-report/summary.json). Warning flags, sensible-report producer/energies, backend/callbacks, error/console prefixes and ordinary weather handoff remain unavailable/not PASS; whole SYS and A 14/54 / B 14/64 gates unchanged.

SRC-05 standalone main: 26 cases/30 operations/71 available setter boundaries, zero mismatches, 102 unavailable observations excluded from PASS. [Complete selected proof and separate wind diagnostics](evidence/SRC-05-wind-diagnostics-and-main-thread/summary.json). Earlier three-mismatch libtest result remains retained. At that capture, production used a worker; full guard/backend and ordinary caller/handoff remained pending. All gates unchanged.

SRC-05 actual Windows CLI and ordinary observer now run on main with actual Cargo-emitted 8 MiB-stack PE artifacts. Source00bfa56d passed fmt/Clippy, focused346/workspace4,973 (zero failures, same two ignored observers). Both unchanged A-24H executions exited0; independent full hashes cover all62 outputs and complete4,034-event uncapped capture. [Actual production-entry custody](evidence/SRC-05-production-main-thread/summary.json). Numerical equivalence, boundary reachability, ordinary four-array/guard/backend and consumer handoff remain unverified; full-card and A14/54/B14/64 gates unchanged.

SRC-06 selected Natural/Detailed: 28 cases/60 genuine calls, 54 fully finite endpoints exact, six nonfinite class diagnostics excluded from numerical PASS, zero mismatches and 34 required-unavailable rows. Focused346/workspace4,973 passed (two ignored observers), fmt/Clippy passed. [Complete application and comparison evidence](evidence/SRC-06-tarp-inside-selected/summary.json). Actual InitIntConvCoeff traversal/space MAT/cadence30/SUR-05 and ordinary handoff remain pending; gates unchanged.

CTF-10의 실제 Windows main 일반 실행 비교는 FAIL로 보존했다. 1,340개 대응 단계에서 13,807,360개 exact 관측 중 235,456개 불일치가 있었고, 필수 경계 2,754개는 미확인·미대응으로 남았다. 최초 대응 차이는 이력 갱신 전 입력 SurfTempIn에 이미 존재하므로 원인으로 단정하지 않는다. [전체 비교·다섯 상세 장부·남은 범위](evidence/CTF-10-main-thread-history-comparison/summary.json). 기존 실패와 정책을 유지하며 A14/54·B14/64 및 카드 게이트는 변경하지 않았다.

SRC-06 선택 초기화 dispatcher: 실제 원본·Rust20회 호출에서 입력24,033개·pre-owner8,963개·endpoint9,083개 비교가 일치했다. 불일치·필수 누락은0개이며 선택 범위 밖264개 문맥은 PASS에서 제외했다. fmt/Clippy와 관련354개·전체4,981개 시험 통과(기존 ignored2개). [전체 선택 비교와 네 장부](evidence/SRC-06-tarp-dispatcher/summary.json). 일반 실행의 형상·MAT·온도 배열·복사 후 cadence30·열수지 및 handoff 연결은 미완료이며 A14/54·B14/64와 카드 게이트는 유지한다.

SRC-06 bounded ordinary geometry: actual status `pass-available-selected-ordinary-geometry-only`, mismatches0, required unavailable0, optional unpaired19. fmt/Clippy and focused354/workspace4988 passed, with the same two ignored observers. [Complete actual producer evidence and three full ledgers](evidence/SRC-06-ordinary-geometry/summary.json). Ordinary MAT/temperature owners/cadence/weather and handoff remain pending; all card gates and A14/54/B14/64 stay unchanged.

SRC-06의 실제 HB320 내부발열 입력은 원본 부모 반환과 새226개 binary64·867개 이산 항목 및 nominal EIO1,576바이트가 일치했다. 기존 Solar를 합쳐337개 수치·1,543개 이산 검사에서 차이0·UNKNOWN0이다. [선택 범위·실행·실패 이력·증거](evidence/SRC-06-ordinary-hb-internal-gains-input/summary.json). 수정 후 기존 CLI2회만 실행하고 영향 없는 raw-model/runtime2회는 전체 소스 색인의 한 줄 차이를 확인해 재사용했다. 시간별 발열·실제 방사 소비·전역 DD 절대 도메인과 전체 Setup/HB는 미완료이며 네 gate·A14/54·B14/64 및 기존 수치 FAIL는 유지한다.


[날씨·실제 호출 연결 검증 체크포인트](evidence/SRC-06-provider-integration-checkpoint-07/summary.json)에 검증한 제안 코드와 재현 ZIP을 보존했습니다. 같은 세션의 GetNext·설계일 생성·일별 판독·보간·스케줄·강수량·외부 에너지 사용·열수지 호출을 연결했고, 실제 준비 루프와 본 실행 루프의 원본 순서를 추가했습니다. 새 빌드 디렉터리에서 runtime·run·CLI 및 기존 테스트 타깃 전체 컴파일을 통과했습니다. 기존 판독기 9개와 설계일 준비 과정 1개 테스트가 통과했으며 새 테스트는 추가하지 않았습니다. 수명·문자열 타입 오류와 수정 기록도 보존했습니다. 전체 실행 파이프라인 활성화, 준비 과정 후처리·본 실행의 실제 호출 연결 및 새로운 동일 입력 Native A/B 검증은 남아 있습니다. 네 gate·A14/54·B14/64와 기존 수치 FAIL은 유지합니다.


[저장소 Source 적용과 실제 검증 기록](evidence/SRC-06-canonical-provider-integration-01/summary.json). 검토한 일반 실행 Source 417개 파일(신규 325개)을 저장소 본체에 적용했습니다. 동일 세션의 날씨·준비 과정·열수지·본 실행 부모와 실제 시작 단계 child, 진단 상태 참조를 포함합니다. 기존 전체 워크스페이스·타깃 테스트 4,985개가 통과했고, 실패는 0개이며 기존 ignored 2개는 그대로입니다. 새 테스트는 추가하지 않았습니다. 기존 테스트 입력의 빠진 재료 선언, 중복 Site:Location 스키마, 상태 객체의 스택 보관 문제 및 일반 실행 출력 목록 두 곳을 수정했습니다. 기존 표면 테스트 준비값에는 실제 면적 필드 세 곳을 보완했습니다. 기존 전체 출력 집합과 구역 상태 비교는 유지했습니다. 실패·진단 기록과 각 수정의 범위를 별도로 보존했습니다. 기본 실행 파이프라인 전환, 전체 종료 후처리, 남은 실제 호출 연결과 동일 입력 Native A/B 검증은 미완료입니다. 네 gate·A14/54·B14/64 및 기존 수치 FAIL은 유지합니다. 작은 수정은 영향을 받는 기존 테스트로 확인하고, 이번 전체 Source 적용처럼 범위가 큰 통합 시점에 전체 테스트를 수행합니다.


[보고서 및 종료 함수 Source 적용](evidence/SRC-06-canonical-provider-integration-02/summary.json). 연결 보고서 3종과 RDD/MDD를 실제 pre-primary host에 연결하고, 같은 날씨·세션·원래 지역 상태를 사용하는 kickoff부터 primary까지의 호출 함수를 적용했습니다. 종료 단계의 작은 함수, 실제 요금 계산 진입 순서와 상태 필드도 포함합니다. 전체 타깃 컴파일 및 기존 실행 테스트 27개가 원래 8MiB 스택에서 통과했습니다. 새 테스트를 추가하지 않았고 전체 테스트는 반복하지 않았습니다. 실제 기본 파이프라인 전환·전체 종료 처리·Native 수치 비교는 미완료이며 A14/54·B14/64와 기존 FAIL을 유지합니다. 별도 실험용 파이프라인과 이후 종료 prefix는 이 적용에 포함하지 않습니다.


[날씨 입력 메타데이터 수정과 실제 실행 기록](evidence/SRC-06-canonical-weather-idf-metadata-03/summary.json). Site:Location·RunPeriod·SizingPeriod:DesignDay의 실제 IDF 필드 수 전달을 수정했습니다. 선택 필드의 전체 용량으로 입력 개수를 대체하지 않으며, RunPeriod와 DesignDay의 세미콜론 없는 EOF 입력은 정확한 메타데이터를 보장할 수 없어 명시적으로 미지원 처리합니다. 기존 raw-model 테스트 16개와 기본 실행 경로 A-24H 회귀 실행이 통과했습니다. 새 테스트 본문과 전체 테스트 반복은 없습니다. 별도 실제 Source 경로는 세 입력 오류를 넘고 20일 워밍업 후 본 기상 실행에 진입했으며, 첫 열적 복원력 누적 보고에서 미지원으로 종료합니다. 실제 Source 24시간 완료·기본 파이프라인 전환·Native 수치 동등성을 뜻하지 않습니다. A14/54·B14/64와 기존 전체 FAIL을 유지합니다.


[열적 복원력 계산과 실제 본 실행·원본 출력 대조](evidence/SRC-06-canonical-thermal-weather-04/summary.json). 실제 구역 상태·온도·냉난방 설정점을 사용하는 열적 복원력 기상 누적 계산을 적용했습니다. 같은 재실 상태를 보존하고 원본 분기·연산·갱신 순서를 유지합니다. 변경 Source를 포함한 진단 조합의 전체 타깃 컴파일이 통과했고 새 테스트나 전체 테스트 반복은 없습니다. 실제 Source A-24H는 20일 워밍업과 15분 기록 96개·시간별 기록 24개를 생성한 뒤 Primary에서 반환했습니다. 전체 실행은 종료 보고·결과 내보내기 경계에서 exit6이며 기본 파이프라인은 아직 전환하지 않았습니다. 같은 입력의 원본26.1.0 실행은 성공했고 출력 사전·시간축은 일치합니다. ESO 5,280개 값 중 3,536개는 보고된 십진수를 binary64로 해석했을 때 다르며 최대 절대 차이는 7.429434845107608e-11입니다. MTR 72개 보고 값은 같습니다. 내부 원시 상태·FP 환경 동등성을 뜻하지 않으며 허용오차를 추가하지 않았습니다. 원인 검토와 수치 동등성 검증은 남아 있습니다. A14/54·B14/64와 기존 전체 FAIL을 유지합니다.


[입력 통계와 실제 표 파일 생성 검증](evidence/SRC-06-canonical-input-statistics-tabular-05/summary.json). 입력 원시 필드와 스키마 메타데이터로 원본 입력 통계 7개를 계산하도록 연결했습니다. 실제 A-24H 통계 7개는 동일 입력의 Native 출력과 일치합니다. 종료 보고 선행 단계와 실제 표 파일 열기를 반영했으며, 진단 실행은 CSV·TAB·TXT·HTML·XML의 초기 헤더를 생성한 뒤 596번 날씨 통계 보고 진입 전 미구현 경계에서 exit6으로 끝납니다. 강우 보고는 원본의 날씨·나머지 보고 순서를 기다리므로 아직 실행되지 않았습니다. 기본 파이프라인은 입력 통계 초기화 호출만 갱신했고 진단 경로는 전환하지 않았습니다. 관련 기존 테스트 27개는 원래 8MiB 설정에서 통과했으며 새 테스트와 전체 테스트 반복은 없습니다. ESO·MTR·EIO는 직전 진단과 바이트까지 같아 기존 수치 차이가 유지됩니다. 전체 실행·수치 동등성·카드 완료를 뜻하지 않으며 A14/54·B14/64와 기존 전체 FAIL은 유지합니다.


[날씨·잔여·강우 보고 단계 검증](evidence/SRC-06-canonical-weather-remaining-reports-06/summary.json). 날씨 STAT 원시 바이트, 실제 보고 레지스트리와 동일 세션의 상태를 이용해 날씨·잔여·강우 보고 및 출력 형식 선택을 연결했습니다. 실제 진단 실행은 5212·5213·5214 순서를 거쳐 연간 에너지 요약표 WriteBEPSTable5230 진입 전 미구현 경계에서 exit6으로 끝납니다. 새로 도달한 표 항목 219개 중 218개는 동일 입력 Native 최종 보고서와 출력 바이트가 같고, 1개는 Native의 뒤 단계 덮어쓰기 전 자리표시자이므로 단계가 다른 미짝 항목입니다. 관련 기존 테스트 19개와 기본 실행 기존 테스트 27개가 원래 8MiB 설정에서 통과했으며 새 테스트와 전체 테스트 반복은 없습니다. 기본 파이프라인과 읽기 전용 진단 관측은 전환하지 않았습니다. ESO·MTR·EIO는 직전과 바이트까지 같아 기존 ESO 수치 차이가 남습니다. 전체 실행·수치 동등성·카드 완료를 뜻하지 않으며 A14/54·B14/64와 기존 전체 FAIL을 유지합니다.


[BEPS 보고 핵심 단계 검증](evidence/SRC-06-canonical-beps-report-core-07/summary.json). BEPS 보고 핵심 Rust 파일 17개(신규 12개·기존 5개)를 같은 실제 상태 소유자와 연결했습니다. 실제 격리 Source72 실행은 BEPS 표 13개를 작성한 뒤 목차 호출 5231의 미구현 경계에서 exit6으로 종료됩니다. 같은 입력의 Native 최종 보고서와 다섯 출력 형식의 앞부분을 비교해 원시 바이트 3,007줄이 같고, 버전·실제 시각 차이 9줄은 원문 그대로 별도 보존했습니다. LEED predefined 항목은 505개에서 726개로 늘었습니다. canonical all-targets 검사를 통과했으며 새 테스트나 전체 테스트 반복은 없습니다. 이전 27개 테스트 결과는 이전 checkpoint의 검증 범위로만 유지합니다. 진단 경로 9개·읽기 전용 snapshot·기본 pipeline은 이번 적용에서 제외했습니다. 기존 ESO 수치 차이와 A14/54·B14/64 및 전체 FAIL은 유지하며, 전체 실행·수치 동등성·카드 완료를 뜻하지 않습니다.


[Tabular 보고서 핵심 단계 검증](evidence/SRC-06-canonical-tabular-report-core-08/summary.json). 실제 TOC·Verification Summary·Demand·Source Energy 보고서 경로의 Rust 파일 20개(신규 10개, 기존 10개)를 같은 상태 소유자에 연결했습니다. 격리 Source75 실행은 이름 있는 표 25개를 작성한 뒤 실제 Component Sizing 진입 5235에서 exit6으로 중단됩니다. 같은 입력의 Native 최종 보고서와 다섯 출력 형식의 앞부분 6,680줄을 비교해 6,663줄이 바이트 단위로 같았으며, 버전·실제 시각 차이 17줄은 원문과 전체 hex로 보존했습니다. Predefined 항목은 732개에서 914개로 늘었고, 실제 할당 공간 1,600개 전체의 변경 내역도 보존했습니다. 첫 비교 시도의 시각 분류·할당 꼬리 관찰 오류는 미승인 이력으로 남기고 수정된 비교만 승인했습니다. Canonical all-targets 검사는 통과했으며 새 테스트나 전체 테스트 반복은 없었습니다. 이전 27개 테스트 증거는 이전 검증 범위로만 유지합니다. 진단 경로 9개·읽기 전용 snapshot·기본 pipeline·미컴파일 후속 네 보고서는 이번 적용에서 제외했습니다. 기존 ESO 수치 차이와 원인 미확정 상태, A14/54·B14/64 및 전체 FAIL은 유지하며 전체 실행·수치 동등성·내보내기 완료를 뜻하지 않습니다.


[후속 요약 보고 단계 검증](evidence/SRC-06-canonical-post-summary-core-09/summary.json). 크기 산정·음영·비용·적응 쾌적 보고, 초기화 요약·부하 보고 조건·열배출 보고의 Rust 파일 7개(신규 3개·기존 4개)를 반영했습니다. 실제 격리 Source77 실행은 이름 있는 표 75개를 작성한 뒤 열적 복원력 보고 5244 진입에서 exit6으로 끝납니다. 같은 입력 Native와 다섯 출력 형식의 앞부분 9,620줄을 대조해 9,588줄이 바이트까지 같으며, 버전 10줄·실제 시각 11줄·실행 날짜 11줄의 차이는 전체 원문 hex로 보존했습니다. Source 실행일은 10월 19일, 보존한 Native 실행일은 10월 17일입니다. EIO는 같은 열린 파일의 위치를 복원하며 읽습니다. 본체에는 그 메서드 2,186바이트만 추가하고 기존 진단 관측 변경 1,164바이트는 제외했습니다. Predefined 914개 항목과 ESO·MTR·EIO·원시 진단 출력은 직전 실행과 같습니다. Canonical 전체 타깃 컴파일을 통과했고 새 테스트나 전체 테스트 반복은 없습니다. 기본 pipeline과 진단 관측은 전환하지 않았습니다. 기존 ESO 수치 차이·원인 미확정 상태, A14/54·B14/64와 전체 FAIL은 유지하며 전체 실행·수치 동등성·내보내기 완료를 뜻하지 않습니다.


[실제 endrun 종료 경로 검증](evidence/SRC-06-canonical-completed-endrun-core-10/summary.json). 유한 A-24H 포팅의 복원력·주기·코일·사전 정의 보고서와 감사·파일 닫기·endrun 620까지 실제 Source 경로를 반영했습니다. 실제 격리 Source79는 이름 있는 표 191개를 작성했으며, 다섯 물리 출력 형식 전체 19,312줄 중 19,242줄이 Native와 바이트까지 같고 나머지 70줄은 버전·시계·날짜 메타데이터 차이로 원문을 보존했습니다. 초기 조명 입력 처리의 누락된 합계 세 항목은 실제 Native 생산자 순서로 복원했고, 같은 출력 스트림의 Windows 텍스트 모드를 적용했습니다. 실제 ESO 5,280개·MTR 72개 수치 레코드의 종료 기록과 감사·SHD 파일을 확인했습니다. Canonical Rust 파일 31개(신규 14개·기존 17개), 전체 컴파일 입력 4,467개를 검증했으며 기존 테스트 하나의 물리 줄바꿈 기대값 두 곳만 수정했습니다. 기본 pipeline과 진단 관측은 전환하지 않았으며, Source 결과 내보내기는 아직 명시적으로 중단됩니다. 기존 ESO 수치 차이 3,536개와 미규명 부동소수점 원인, A14/54·B14/64 및 전체 FAIL 상태는 유지합니다. 이는 endrun 완료 근거이며 전체 결과 내보내기나 Native 수치 동등성 완료를 뜻하지 않습니다.


[Source 실행·결과 내보내기 검증](evidence/SRC-06-canonical-completed-source-activation-11/summary.json). 명시한 `--porting-scope A/B` 실행을 실제 Source 경로에 연결했습니다. 같은 상태 소유자로 초기화·본 실행·종료 620을 거친 뒤 완성된 ESO/MTR만 읽고, 보고 ID·빈도·저장 방식·원문 값·Native 시간 튜플을 JSON/CSV로 보존합니다. 정식 A-24H 실행과 전체 대상 컴파일이 성공했고, 58계열 5,352개 값의 물리 출력·JSON·CSV 일치를 검증했습니다. 보고값 차이 3,536개는 변경하지 않은 기존 허용오차와 계열별 RMSE 한계 안에 있습니다. 기본 실행 경로는 보존하며, B는 처음 만나는 미구현 입력·분기에서 오류로 중단합니다. 진단 관측 파일은 배포 경로에서 제외했습니다. 추가 기간·B 사례·전체 포팅 게이트는 이 A-24H 검증으로 완료 처리하지 않습니다. A14/54·B14/64와 기존 전체 비교 기록은 유지합니다.


[Source B 입력·출력 형식·보고값 비교 검증](evidence/SRC-06-canonical-B-ingress-IO-reported-cases-12/summary.json). B 입력의 실제 값·확장 필드·선언 순서·양수 개수를 같은 InputProcessor에 보존하고, 실제 초기화의 Schedule:Compact 미구현 경계까지 진행했습니다. B 실행은 exit6으로 명시적으로 중단하며 완료 처리하지 않습니다. RDD/MDD는 원래 출력 시점에 Native와 같은 Windows 줄바꿈을 사용합니다. 같은 새 코어의 A-24H는 종료·내보내기에 성공했고 5,352개 값의 기존 전체 증명과 일치합니다. 이전 Source11의 A-72H 16,056개·연간 1,953,480개 보고값은 원래 허용오차·계열별 RMSE 실패가 모두 0입니다. 공개 스트림 비교 도구는 같은 알고리즘을 유지하며 A-72H 대조로 확인했습니다. 연간 비교는 반복하지 않았습니다. 숫자 차이와 정책 메타데이터의 기존 한계를 보존하며, 내부 비트 동일성·B 및 전체 15사례·카드 게이트 완료를 주장하지 않습니다. A14/54·B14/64는 유지합니다.


[Source Compact 스케줄 검증](evidence/SRC-06-canonical-compact-schedules-13/summary.json). Compact 포팅의 Native 대조 단일 테스트와 A-24H 회귀를 통과했습니다. B는 제어 대상 Zone 조회에서 미완료이며 A14/54·B14/64 게이트는 유지합니다.


[Source Zone·Node 입력 검증](evidence/SRC-06-canonical-Zone-Node-14/summary.json). Zone·Node 입력 경로의 기존 소유자 테스트와 A-24H 회귀를 통과했습니다. B는 장비 연결 입력 조회에서 미완료이며 A14/54·B14/64 게이트는 유지합니다.


[Source 제어장비 입력 검증](evidence/SRC-06-canonical-controlled-equipment-15/summary.json). 제어장비 입력을 연결하고 기존 소유권 테스트와 A-24H 회귀를 통과했습니다. B는 다음 실제 미포팅 경계에서 중단하며 A14/54·B14/64 게이트는 유지합니다.


[Source 체적·Node 경로 검증](evidence/SRC-06-canonical-Node-lifecycle-16/summary.json). 체적·Node 경로를 연결하고 기존 소유권 테스트와 A-24H 회귀를 통과했습니다. B의 실제 다음 경계는 증빙 요약에 기록했으며 A14/54·B14/64 게이트는 유지합니다.


[Source 온도조절기 입력 검증](evidence/SRC-06-canonical-thermostat-17/summary.json). 온도조절기 입력 경로를 연결하고 기존 소유권 테스트와 A-24H 회귀를 검증했습니다. B의 실제 경계는 증빙 요약을 따르며 A14/54·B14/64 게이트는 유지합니다.


[Source 온도 설정값 경로 검증](evidence/SRC-06-canonical-thermostat-current-19/summary.json). 온도 제어 환경 초기화와 현재 설정값 경로를 연결했습니다. 실제 B 경계와 A24 검증 결과는 공개 요약에 기록합니다.


[Source 제어 부하 예측 검증](evidence/SRC-06-canonical-controlled-prediction-21/summary.json). 제어 부하 예측과 실제 Node 설정값 갱신을 연결했습니다. B·A24 CLI 결과와 남은 경계는 공개 요약에 기록합니다.


[Source 설비 가용성 경로 검증](evidence/SRC-06-canonical-component-availability-22/summary.json). 실제 설비 수만큼 가용성 상태를 할당하고 관리자 없는 설비의 처리 경로를 연결했습니다. B·A24 실행 결과와 남은 경계를 공개 요약에 기록합니다.


[Source 구역 설비 초기화 검증](evidence/SRC-06-canonical-zone-equipment-initialization-23/summary.json). 구역 설비 초기화 경로를 연결했다. 실제 B·A24 실행과 남은 경계는 요약 자료에 기록한다.


[Source 구역 설비 시뮬레이션 진입 검증](evidence/SRC-06-canonical-zone-equipment-orchestration-24/summary.json). 설비 우선순위와 부하 배분을 실제 실행 경로에 연결했다. B의 다음 경계와 A24 회귀 검증은 요약 자료에 기록한다.


[Source 예측 수치 코호트 검증](evidence/SRC-06-canonical-controlled-prediction-native-cohort-21/summary.json). Native 실제 호출 1,168개를 단일 테스트로 대조했습니다. 테스트 전용 변경의 운영 근거는 checkpoint 24의 B·A 실행 결과를 상속합니다.


[Source IdealLoads 입력·보고 연결 검증](evidence/SRC-06-canonical-purchased-air-input-25/summary.json). IdealLoads 입력과 67개 보고 항목의 실시간 연결 결과를 정리했다. B의 실제 다음 경계와 A24 결과는 요약을 따른다.


[Source IdealLoads 초기화 검증](evidence/SRC-06-canonical-purchased-air-initialization-26/summary.json). IdealLoads 초기화를 실제 장비·노드·일정·환경 소유자에 연결했다. 실행 도달 범위와 A24 회귀 결과는 연결 증거에 기록한다.


[Source IdealLoads 계산·갱신·보고 검증](evidence/SRC-06-canonical-purchased-air-calculation-27/summary.json). IdealLoads 계산·갱신·보고 경로를 같은 실제 소유자에 연결했습니다. 관측된 중단 지점과 A24 회귀 결과는 연결된 증거에 기록합니다.


[Source 장비 출력·수요 갱신 검증](evidence/SRC-06-canonical-zone-equipment-output-28/summary.json). 장비 호출 뒤 실제 반환 출력으로 구역 유량과 남은 부하를 갱신하는 경로를 연결했습니다. 관측된 B 경계와 A24 회귀 결과는 연결된 증거에 기록합니다.


[Source 구역 질량 보존·리턴 공기 검증](evidence/SRC-06-canonical-controlled-air-balance-29/summary.json). 제어 구역의 유입·리턴 유량과 리턴 공기 상태 계산을 실제 소유자에 연결했습니다. 실행 범위와 A24 결과는 연결된 증거에서 확인합니다.


[Source HVAC Node 생명주기 검증](evidence/SRC-06-canonical-hvac-controlled-node-lifecycle-30/summary.json). 기존 물리 Node와 Config를 사용하는 HVAC 초기화·이력·설정값 경로와 반환공기 열 플래그를 연결했습니다. 실제 B 경계와 A24 회귀 결과는 연결된 증거에 기록합니다.


[Source 구역 온습도 보정 검증](evidence/SRC-06-canonical-controlled-zone-corrector-31/summary.json). 제어 구역의 실제 공급 Node를 사용하는 ThirdOrder 온도·습도 보정 경로를 연결했습니다. 실제 B 경계와 A24 회귀 결과는 연결된 증거에 기록합니다.


[SRC-06 통합 코호트 전용 타깃](evidence/SRC-06-canonical-predictor-corrector-native-cohort-31-cohort/summary.json). 기존 본문 1개를 명시적 메인 스레드 테스트로 실행했다. 일반 단위 테스트에서는 큰 코호트를 제외하며, 실패·진단 3회와 성공 1회를 구분해 기록했다. 게이트 수는 그대로다.


[Source33 component/Node report](evidence/SRC-06-canonical-component-node-report-33/summary.json). 제어 구역 성분별 열수지와 노드 이력·추가정보 계산 묶음의 B/A 실행 결과를 기록합니다.


[Source34 controlled-zone ventilation](evidence/SRC-06-canonical-controlled-zone-ventilation-34/summary.json). 제어 구역 환기 출력 등록 20개와 m3·ach 단위를 연결했다. B의 실제 다음 경계와 A24 증거는 연결된 요약에 기록한다. 게이트 수는 그대로다.


[Source35 startup connection validation/report](evidence/SRC-06-canonical-startup-connection-validation-report-35/summary.json). 노드 연결 검사와 제어 구역·장비 목록·비부모 노드 연결 보고를 통합했다. 별도 빌드와 체크포인트를 반복하지 않고 실제 B/A24 한 쌍으로 확인한다. 게이트 수는 그대로다.


[Source37 energy report structure](evidence/SRC-06-canonical-energy-report-structure-37/summary.json). 에너지 보고 구조 생성 경로를 기존 파일 6개와 새 파일 3개로 연결했다. 실제 B/A24 한 쌍을 확인하며, 추가 테스트나 별도 Native 실행은 없다. 게이트 수는 그대로다.


[Source38 energy report initialization](evidence/SRC-06-canonical-energy-report-initialization-38/summary.json). 에너지 보고 초기화와 현재 계량값 읽기를 기존 파일 4개와 새 파일 1개로 연결했다. 실제 B/A24 한 쌍으로 확인하고, 추가 테스트나 Native 재실행 없이 게이트 수를 유지한다.


[Source39 controlled system reports](evidence/SRC-06-canonical-controlled-system-report-39/summary.json). 정상 제어 구역 보고 경로를 기존 Rust 파일 6개로 연결하고 실제 B/A24 경로를 확인했다. 이번 단위의 추가 테스트는 없으며 게이트 수는 유지한다. 보고 계산의 Native 수치 검증은 아직 완료로 표시하지 않는다.


[Source41 node setpoint endrun](evidence/SRC-06-canonical-node-setpoint-endrun-41/summary.json). 노드 setpoint 종료 검사를 기존 Rust 파일 2개로 연결하고 실제 B/A24 실행 결과를 기록한다. 추가 테스트 없이 기존 게이트 수를 유지하며 별도 보고 수치 검증을 완료로 표시하지 않는다. 최초 A24의 빌드 경로 오류 기록은 보존하고, 교정된 A24-02를 승인된 결과로 사용한다.


[Source40 Native report cohort](evidence/SRC-06-canonical-system-report-native-cohort-40/summary.json). 보고 전용 단일 코호트에 471회 Native 대조를 통합했습니다. 실행 범위 선택은 CONTRIBUTING.md에 정리했으며 카드·게이트 수는 유지합니다.


[Source42 controlled predefined entries](evidence/SRC-06-canonical-fill-remaining-controlled-42/summary.json). 기존 Rust 파일 3개로 제어 존 최종 표 기입을 연결했다. 실제 B/A24 CLI 2회만 사용하고 실행 도구가 canonical 빌드 경로와 incremental0을 직접 지정한다. 새 테스트와 게이트 변경은 없다.


[Source44 thermostat predefined reports](evidence/SRC-06-canonical-thermostat-predefined-44/summary.json). 기존 Rust 파일 4개와 신규 공통 helper 1개로 thermostat 최종 요약을 이었다. B/A24 CLI 2회로 확인하며 실행 도구가 canonical 빌드 경로와 incremental0을 강제한다. 새 테스트와 게이트 변경은 없다.
