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
