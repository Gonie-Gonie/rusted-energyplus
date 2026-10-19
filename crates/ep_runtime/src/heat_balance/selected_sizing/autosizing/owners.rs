//! Complete actual Base/FanHeat/Scalable local constructor fields.
#![allow(non_snake_case, dead_code)]
use super::super::{SourceZoneEquipmentSizing, SourceTerminalUnitSizingData};
#[derive(Clone, Debug)]
pub(super) struct SourceBaseSizer {
    pub(super) stdRhoAir: f64, // Base.hh::stdRhoAir
    pub(super) zoneAirFlowSizMethod: i32, // Base.hh::zoneAirFlowSizMethod
    pub(super) dataScalableSizingON: bool, // Base.hh::dataScalableSizingON
    pub(super) dataScalableCapSizingON: bool, // Base.hh::dataScalableCapSizingON
    pub(super) isCoilReportObject: bool, // Base.hh::isCoilReportObject
    pub(super) isFanReportObject: bool, // Base.hh::isFanReportObject
    pub(super) initialized: bool, // Base.hh::initialized
    pub(super) errorType: i32, // Base.hh::errorType
    pub(super) sizingType: i32, // Base.hh::sizingType
    pub(super) sizingString: String, // Base.hh::sizingString
    pub(super) sizingStringScalable: String, // Base.hh::sizingStringScalable
    pub(super) overrideSizeString: bool, // Base.hh::overrideSizeString
    pub(super) originalValue: f64, // Base.hh::originalValue
    pub(super) autoSizedValue: f64, // Base.hh::autoSizedValue
    pub(super) wasAutoSized: bool, // Base.hh::wasAutoSized
    pub(super) hardSizeNoDesignRun: bool, // Base.hh::hardSizeNoDesignRun
    pub(super) sizingDesRunThisAirSys: bool, // Base.hh::sizingDesRunThisAirSys
    pub(super) sizingDesRunThisZone: bool, // Base.hh::sizingDesRunThisZone
    pub(super) sizingDesValueFromParent: bool, // Base.hh::sizingDesValueFromParent
    pub(super) airLoopSysFlag: bool, // Base.hh::airLoopSysFlag
    pub(super) oaSysFlag: bool, // Base.hh::oaSysFlag
    pub(super) coilType_Num: i32, // Base.hh::coilType_Num
    pub(super) compType: String, // Base.hh::compType
    pub(super) compName: String, // Base.hh::compName
    pub(super) isEpJSON: bool, // Base.hh::isEpJSON
    pub(super) sysSizingRunDone: bool, // Base.hh::sysSizingRunDone
    pub(super) zoneSizingRunDone: bool, // Base.hh::zoneSizingRunDone
    pub(super) curSysNum: i32, // Base.hh::curSysNum
    pub(super) curOASysNum: i32, // Base.hh::curOASysNum
    pub(super) curZoneEqNum: i32, // Base.hh::curZoneEqNum
    pub(super) curDuctType: i32, // Base.hh::curDuctType
    pub(super) curTermUnitSizingNum: i32, // Base.hh::curTermUnitSizingNum
    pub(super) numPrimaryAirSys: i32, // Base.hh::numPrimaryAirSys
    pub(super) numSysSizInput: i32, // Base.hh::numSysSizInput
    pub(super) doSystemSizing: bool, // Base.hh::doSystemSizing
    pub(super) numZoneSizingInput: i32, // Base.hh::numZoneSizingInput
    pub(super) doZoneSizing: bool, // Base.hh::doZoneSizing
    pub(super) autoCalculate: bool, // Base.hh::autoCalculate
    pub(super) termUnitSingDuct: bool, // Base.hh::termUnitSingDuct
    pub(super) termUnitPIU: bool, // Base.hh::termUnitPIU
    pub(super) termUnitIU: bool, // Base.hh::termUnitIU
    pub(super) zoneEqFanCoil: bool, // Base.hh::zoneEqFanCoil
    pub(super) otherEqType: bool, // Base.hh::otherEqType
    pub(super) zoneEqUnitHeater: bool, // Base.hh::zoneEqUnitHeater
    pub(super) zoneEqUnitVent: bool, // Base.hh::zoneEqUnitVent
    pub(super) zoneEqVentedSlab: bool, // Base.hh::zoneEqVentedSlab
    pub(super) minOA: i32, // Base.hh::minOA
    pub(super) dataEMSOverrideON: bool, // Base.hh::dataEMSOverrideON
    pub(super) dataEMSOverride: f64, // Base.hh::dataEMSOverride
    pub(super) dataAutosizable: bool, // Base.hh::dataAutosizable
    pub(super) dataConstantUsedForSizing: f64, // Base.hh::dataConstantUsedForSizing
    pub(super) dataFractionUsedForSizing: f64, // Base.hh::dataFractionUsedForSizing
    pub(super) dataDXCoolsLowSpeedsAutozize: bool, // Base.hh::dataDXCoolsLowSpeedsAutozize
    pub(super) dataPltSizHeatNum: i32, // Base.hh::dataPltSizHeatNum
    pub(super) dataWaterLoopNum: i32, // Base.hh::dataWaterLoopNum
    pub(super) dataFanIndex: i32, // Base.hh::dataFanIndex
    pub(super) dataFanType: i32, // Base.hh::dataFanType
    pub(super) dataWaterCoilSizCoolDeltaT: f64, // Base.hh::dataWaterCoilSizCoolDeltaT
    pub(super) dataWaterCoilSizHeatDeltaT: f64, // Base.hh::dataWaterCoilSizHeatDeltaT
    pub(super) dataCapacityUsedForSizing: f64, // Base.hh::dataCapacityUsedForSizing
    pub(super) dataPltSizCoolNum: i32, // Base.hh::dataPltSizCoolNum
    pub(super) dataDesInletAirHumRat: f64, // Base.hh::dataDesInletAirHumRat
    pub(super) dataAirFlowUsedForSizing: f64, // Base.hh::dataAirFlowUsedForSizing
    pub(super) dataDesInletAirTemp: f64, // Base.hh::dataDesInletAirTemp
    pub(super) dataDesAccountForFanHeat: bool, // Base.hh::dataDesAccountForFanHeat
    pub(super) dataFanPlacement: i32, // Base.hh::dataFanPlacement
    pub(super) dataFlowUsedForSizing: f64, // Base.hh::dataFlowUsedForSizing
    pub(super) dataDesOutletAirHumRat: f64, // Base.hh::dataDesOutletAirHumRat
    pub(super) dataDesInletWaterTemp: f64, // Base.hh::dataDesInletWaterTemp
    pub(super) dataDesOutletAirTemp: f64, // Base.hh::dataDesOutletAirTemp
    pub(super) dataWaterFlowUsedForSizing: f64, // Base.hh::dataWaterFlowUsedForSizing
    pub(super) dataSizingFraction: f64, // Base.hh::dataSizingFraction
    pub(super) dataDXSpeedNum: i32, // Base.hh::dataDXSpeedNum
    pub(super) dataDesicRegCoil: bool, // Base.hh::dataDesicRegCoil
    pub(super) dataHeatSizeRatio: f64, // Base.hh::dataHeatSizeRatio
    pub(super) dataZoneUsedForSizing: i32, // Base.hh::dataZoneUsedForSizing
    pub(super) dataDesicDehumNum: i32, // Base.hh::dataDesicDehumNum
    pub(super) dataNomCapInpMeth: bool, // Base.hh::dataNomCapInpMeth
    pub(super) dataCoilNum: i32, // Base.hh::dataCoilNum
    pub(super) dataFanOp: i32, // Base.hh::dataFanOp
    pub(super) dataDesignCoilCapacity: f64, // Base.hh::dataDesignCoilCapacity
    pub(super) dataErrorsFound: bool, // Base.hh::dataErrorsFound
    pub(super) dataBypassFrac: f64, // Base.hh::dataBypassFrac
    pub(super) dataIsDXCoil: bool, // Base.hh::dataIsDXCoil
    pub(super) dataNonZoneNonAirloopValue: f64, // Base.hh::dataNonZoneNonAirloopValue
    pub(super) printWarningFlag: bool, // Base.hh::printWarningFlag
    pub(super) callingRoutine: String, // Base.hh::callingRoutine
    pub(super) sysSizingInputData: Option<Vec<std::convert::Infallible>>, // Base.hh::sysSizingInputData
    pub(super) zoneSizingInput: Option<Vec<std::convert::Infallible>>, // Base.hh::zoneSizingInput
    pub(super) unitarySysEqSizing: Option<Vec<SourceZoneEquipmentSizing>>, // Base.hh::unitarySysEqSizing
    pub(super) oaSysEqSizing: Option<Vec<SourceZoneEquipmentSizing>>, // Base.hh::oaSysEqSizing
    pub(super) zoneEqSizing: Option<Vec<SourceZoneEquipmentSizing>>, // Base.hh::zoneEqSizing
    pub(super) outsideAirSys: Option<Vec<std::convert::Infallible>>, // Base.hh::outsideAirSys
    pub(super) termUnitSizing: Option<Vec<SourceTerminalUnitSizingData>>, // Base.hh::termUnitSizing
    pub(super) termUnitFinalZoneSizing: Option<Vec<std::convert::Infallible>>, // Base.hh::termUnitFinalZoneSizing
    pub(super) finalZoneSizing: Option<Vec<std::convert::Infallible>>, // Base.hh::finalZoneSizing
    pub(super) finalSysSizing: Option<Vec<std::convert::Infallible>>, // Base.hh::finalSysSizing
    pub(super) plantSizData: Option<Vec<std::convert::Infallible>>, // Base.hh::plantSizData
    pub(super) primaryAirSystem: Option<Vec<std::convert::Infallible>>, // Base.hh::primaryAirSystem
    pub(super) airloopDOAS: Vec<std::convert::Infallible>, // Base.hh::airloopDOAS
    pub(super) airLoopControlInfo: Option<Vec<std::convert::Infallible>>, // Base.hh::airLoopControlInfo
    pub(super) deltaP: f64, // BaseSizerWithFanHeatInputs.hh::deltaP
    pub(super) motEff: f64, // BaseSizerWithFanHeatInputs.hh::motEff
    pub(super) totEff: f64, // BaseSizerWithFanHeatInputs.hh::totEff
    pub(super) motInAirFrac: f64, // BaseSizerWithFanHeatInputs.hh::motInAirFrac
    pub(super) fanCompModel: bool, // BaseSizerWithFanHeatInputs.hh::fanCompModel
    pub(super) fanShaftPow: f64, // BaseSizerWithFanHeatInputs.hh::fanShaftPow
    pub(super) motInPower: f64, // BaseSizerWithFanHeatInputs.hh::motInPower
    pub(super) zoneCoolingOnlyFan: bool, // BaseSizerWithScalableInputs.hh::zoneCoolingOnlyFan
    pub(super) zoneHeatingOnlyFan: bool, // BaseSizerWithScalableInputs.hh::zoneHeatingOnlyFan
    pub(super) dataHRFlowSizingFlag: bool, // BaseSizerWithScalableInputs.hh::dataHRFlowSizingFlag
    pub(super) dataFracOfAutosizedCoolingAirflow: f64, // BaseSizerWithScalableInputs.hh::dataFracOfAutosizedCoolingAirflow
    pub(super) dataFracOfAutosizedHeatingAirflow: f64, // BaseSizerWithScalableInputs.hh::dataFracOfAutosizedHeatingAirflow
    pub(super) dataFlowPerCoolingCapacity: f64, // BaseSizerWithScalableInputs.hh::dataFlowPerCoolingCapacity
    pub(super) dataAutosizedCoolingCapacity: f64, // BaseSizerWithScalableInputs.hh::dataAutosizedCoolingCapacity
    pub(super) dataFlowPerHeatingCapacity: f64, // BaseSizerWithScalableInputs.hh::dataFlowPerHeatingCapacity
    pub(super) dataAutosizedHeatingCapacity: f64, // BaseSizerWithScalableInputs.hh::dataAutosizedHeatingCapacity
    pub(super) dataCoilSizingAirInTemp: f64, // BaseSizerWithScalableInputs.hh::dataCoilSizingAirInTemp
    pub(super) dataCoilSizingAirInHumRat: f64, // BaseSizerWithScalableInputs.hh::dataCoilSizingAirInHumRat
    pub(super) dataCoilSizingAirOutTemp: f64, // BaseSizerWithScalableInputs.hh::dataCoilSizingAirOutTemp
    pub(super) dataCoilSizingAirOutHumRat: f64, // BaseSizerWithScalableInputs.hh::dataCoilSizingAirOutHumRat
    pub(super) dataCoilSizingFanCoolLoad: f64, // BaseSizerWithScalableInputs.hh::dataCoilSizingFanCoolLoad
    pub(super) dataCoilSizingCapFT: f64, // BaseSizerWithScalableInputs.hh::dataCoilSizingCapFT
    pub(super) dataTotCapCurveValue: f64, // BaseSizerWithScalableInputs.hh::dataTotCapCurveValue
    pub(super) dataFracOfAutosizedCoolingCapacity: f64, // BaseSizerWithScalableInputs.hh::dataFracOfAutosizedCoolingCapacity
    pub(super) dataFracOfAutosizedHeatingCapacity: f64, // BaseSizerWithScalableInputs.hh::dataFracOfAutosizedHeatingCapacity
    pub(super) dataCoolCoilCap: f64, // BaseSizerWithScalableInputs.hh::dataCoolCoilCap
    pub(super) dataCoilIsSuppHeater: bool, // BaseSizerWithScalableInputs.hh::dataCoilIsSuppHeater
    pub(super) suppHeatCap: f64, // BaseSizerWithScalableInputs.hh::suppHeatCap
    pub(super) unitaryHeatCap: f64, // BaseSizerWithScalableInputs.hh::unitaryHeatCap
    pub(super) dataTotCapCurveIndex: i32, // BaseSizerWithScalableInputs.hh::dataTotCapCurveIndex
    pub(super) dataCoolCoilType: i32, // BaseSizerWithScalableInputs.hh::dataCoolCoilType
    pub(super) dataCoolCoilIndex: i32, // BaseSizerWithScalableInputs.hh::dataCoolCoilIndex
    pub(super) zoneHVACSizingIndex: i32, // BaseSizerWithScalableInputs.hh::zoneHVACSizingIndex
    pub(super) zoneHVACSizing: Option<Vec<std::convert::Infallible>>, // BaseSizerWithScalableInputs.hh::zoneHVACSizing
    pub(super) lastErrorMessages: String, // Base.hh::lastErrorMessages
}
impl Default for SourceBaseSizer {fn default()->Self {Self {
    stdRhoAir: 0.0,
    zoneAirFlowSizMethod: 0,
    dataScalableSizingON: false,
    dataScalableCapSizingON: false,
    isCoilReportObject: false,
    isFanReportObject: false,
    initialized: false,
    errorType: 0,
    sizingType: -1,
    sizingString: String::new(),
    sizingStringScalable: String::new(),
    overrideSizeString: true,
    originalValue: 0.0,
    autoSizedValue: 0.0,
    wasAutoSized: false,
    hardSizeNoDesignRun: false,
    sizingDesRunThisAirSys: false,
    sizingDesRunThisZone: false,
    sizingDesValueFromParent: false,
    airLoopSysFlag: false,
    oaSysFlag: false,
    coilType_Num: 0,
    compType: String::new(),
    compName: String::new(),
    isEpJSON: false,
    sysSizingRunDone: false,
    zoneSizingRunDone: false,
    curSysNum: 0,
    curOASysNum: 0,
    curZoneEqNum: 0,
    curDuctType: -1,
    curTermUnitSizingNum: 0,
    numPrimaryAirSys: 0,
    numSysSizInput: 0,
    doSystemSizing: false,
    numZoneSizingInput: 0,
    doZoneSizing: false,
    autoCalculate: false,
    termUnitSingDuct: false,
    termUnitPIU: false,
    termUnitIU: false,
    zoneEqFanCoil: false,
    otherEqType: false,
    zoneEqUnitHeater: false,
    zoneEqUnitVent: false,
    zoneEqVentedSlab: false,
    minOA: -1,
    dataEMSOverrideON: false,
    dataEMSOverride: 0.0,
    dataAutosizable: false,
    dataConstantUsedForSizing: 0.0,
    dataFractionUsedForSizing: 0.0,
    dataDXCoolsLowSpeedsAutozize: false,
    dataPltSizHeatNum: 0,
    dataWaterLoopNum: 0,
    dataFanIndex: 0,
    dataFanType: -1,
    dataWaterCoilSizCoolDeltaT: 0.0,
    dataWaterCoilSizHeatDeltaT: 0.0,
    dataCapacityUsedForSizing: 0.0,
    dataPltSizCoolNum: 0,
    dataDesInletAirHumRat: 0.0,
    dataAirFlowUsedForSizing: 0.0,
    dataDesInletAirTemp: 0.0,
    dataDesAccountForFanHeat: false,
    dataFanPlacement: -1,
    dataFlowUsedForSizing: 0.0,
    dataDesOutletAirHumRat: 0.0,
    dataDesInletWaterTemp: 0.0,
    dataDesOutletAirTemp: 0.0,
    dataWaterFlowUsedForSizing: 0.0,
    dataSizingFraction: 1.0,
    dataDXSpeedNum: 0,
    dataDesicRegCoil: false,
    dataHeatSizeRatio: 0.0,
    dataZoneUsedForSizing: 0,
    dataDesicDehumNum: 0,
    dataNomCapInpMeth: false,
    dataCoilNum: 0,
    dataFanOp: -1,
    dataDesignCoilCapacity: 0.0,
    dataErrorsFound: false,
    dataBypassFrac: 0.0,
    dataIsDXCoil: false,
    dataNonZoneNonAirloopValue: 0.0,
    printWarningFlag: false,
    callingRoutine: String::new(),
    sysSizingInputData: None,
    zoneSizingInput: None,
    unitarySysEqSizing: None,
    oaSysEqSizing: None,
    zoneEqSizing: None,
    outsideAirSys: None,
    termUnitSizing: None,
    termUnitFinalZoneSizing: None,
    finalZoneSizing: None,
    finalSysSizing: None,
    plantSizData: None,
    primaryAirSystem: None,
    airloopDOAS: Vec::new(),
    airLoopControlInfo: None,
    deltaP: 0.0,
    motEff: 0.0,
    totEff: 0.0,
    motInAirFrac: 0.0,
    fanCompModel: false,
    fanShaftPow: 0.0,
    motInPower: 0.0,
    zoneCoolingOnlyFan: false,
    zoneHeatingOnlyFan: false,
    dataHRFlowSizingFlag: false,
    dataFracOfAutosizedCoolingAirflow: 0.0,
    dataFracOfAutosizedHeatingAirflow: 0.0,
    dataFlowPerCoolingCapacity: 0.0,
    dataAutosizedCoolingCapacity: 0.0,
    dataFlowPerHeatingCapacity: 0.0,
    dataAutosizedHeatingCapacity: 0.0,
    dataCoilSizingAirInTemp: 0.0,
    dataCoilSizingAirInHumRat: 0.0,
    dataCoilSizingAirOutTemp: 0.0,
    dataCoilSizingAirOutHumRat: 0.0,
    dataCoilSizingFanCoolLoad: 0.0,
    dataCoilSizingCapFT: 0.0,
    dataTotCapCurveValue: 0.0,
    dataFracOfAutosizedCoolingCapacity: 0.0,
    dataFracOfAutosizedHeatingCapacity: 0.0,
    dataCoolCoilCap: 0.0,
    dataCoilIsSuppHeater: false,
    suppHeatCap: 0.0,
    unitaryHeatCap: 0.0,
    dataTotCapCurveIndex: 0,
    dataCoolCoilType: -1,
    dataCoolCoilIndex: -1,
    zoneHVACSizingIndex: 0,
    zoneHVACSizing: None,
    lastErrorMessages: String::new(),
}}}
