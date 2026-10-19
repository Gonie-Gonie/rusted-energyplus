//! Actual missing DataSizing constructor cells on the sole SelectedSizingManager.
#![allow(non_snake_case)]
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceSizingGlobals {
    pub(crate) CurOASysNum: i32,       // Native CurOASysNum
    pub(crate) CurDuctType: i32,       // Native CurDuctType
    pub(crate) TermUnitSingDuct: bool, // Native TermUnitSingDuct
    pub(crate) TermUnitPIU: bool,      // Native TermUnitPIU
    pub(crate) TermUnitIU: bool,       // Native TermUnitIU
    pub(crate) ZoneEqFanCoil: bool,    // Native ZoneEqFanCoil
    pub(crate) ZoneEqUnitHeater: bool, // Native ZoneEqUnitHeater
    pub(crate) ZoneEqUnitVent: bool,   // Native ZoneEqUnitVent
    pub(crate) ZoneEqVentedSlab: bool, // Native ZoneEqVentedSlab
    pub(crate) ZoneSizingInput: Option<Vec<std::convert::Infallible>>, // Native ZoneSizingInput
    pub(crate) FinalZoneSizing: Option<Vec<std::convert::Infallible>>, // Native FinalZoneSizing
    pub(crate) TermUnitFinalZoneSizing: Option<Vec<std::convert::Infallible>>, // Native TermUnitFinalZoneSizing
    pub(crate) SysSizInput: Option<Vec<std::convert::Infallible>>,             // Native SysSizInput
    pub(crate) FinalSysSizing: Option<Vec<std::convert::Infallible>>, // Native FinalSysSizing
    pub(crate) PlantSizData: Option<Vec<std::convert::Infallible>>,   // Native PlantSizData
    pub(crate) DataEMSOverrideON: bool,                               // Native DataEMSOverrideON
    pub(crate) DataEMSOverride: f64,                                  // Native DataEMSOverride
    pub(crate) DataAutosizable: bool,                                 // Native DataAutosizable
    pub(crate) DataConstantUsedForSizing: f64, // Native DataConstantUsedForSizing
    pub(crate) DataFractionUsedForSizing: f64, // Native DataFractionUsedForSizing
    pub(crate) DataFanIndex: i32,              // Native DataFanIndex
    pub(crate) DataFanType: i32,               // Native DataFanType
    pub(crate) DataPltSizHeatNum: i32,         // Native DataPltSizHeatNum
    pub(crate) DataWaterLoopNum: i32,          // Native DataWaterLoopNum
    pub(crate) DataPltSizCoolNum: i32,         // Native DataPltSizCoolNum
    pub(crate) DataWaterCoilSizHeatDeltaT: f64, // Native DataWaterCoilSizHeatDeltaT
    pub(crate) DataWaterCoilSizCoolDeltaT: f64, // Native DataWaterCoilSizCoolDeltaT
    pub(crate) DataCapacityUsedForSizing: f64, // Native DataCapacityUsedForSizing
    pub(crate) DataHeatSizeRatio: f64,         // Native DataHeatSizeRatio
    pub(crate) DataAirFlowUsedForSizing: f64,  // Native DataAirFlowUsedForSizing
    pub(crate) DataDesInletAirTemp: f64,       // Native DataDesInletAirTemp
    pub(crate) DataDesAccountForFanHeat: bool, // Native DataDesAccountForFanHeat
    pub(crate) DataFanPlacement: i32,          // Native DataFanPlacement
    pub(crate) DataDesInletAirHumRat: f64,     // Native DataDesInletAirHumRat
    pub(crate) DataDesOutletAirHumRat: f64,    // Native DataDesOutletAirHumRat
    pub(crate) DataDesOutletAirTemp: f64,      // Native DataDesOutletAirTemp
    pub(crate) DataDesInletWaterTemp: f64,     // Native DataDesInletWaterTemp
    pub(crate) DataFlowUsedForSizing: f64,     // Native DataFlowUsedForSizing
    pub(crate) DataWaterFlowUsedForSizing: f64, // Native DataWaterFlowUsedForSizing
    pub(crate) DataSizingFraction: f64,        // Native DataSizingFraction
    pub(crate) DataDXSpeedNum: i32,            // Native DataDXSpeedNum
    pub(crate) DataDesicRegCoil: bool,         // Native DataDesicRegCoil
    pub(crate) DataZoneUsedForSizing: i32,     // Native DataZoneUsedForSizing
    pub(crate) DataDesicDehumNum: i32,         // Native DataDesicDehumNum
    pub(crate) DataNomCapInpMeth: bool,        // Native DataNomCapInpMeth
    pub(crate) DataCoilNum: i32,               // Native DataCoilNum
    pub(crate) DataFanOp: i32,                 // Native DataFanOp
    pub(crate) DataDesignCoilCapacity: f64,    // Native DataDesignCoilCapacity
    pub(crate) DataErrorsFound: bool,          // Native DataErrorsFound
    pub(crate) DataBypassFrac: f64,            // Native DataBypassFrac
    pub(crate) DataIsDXCoil: bool,             // Native DataIsDXCoil
    pub(crate) DataNonZoneNonAirloopValue: f64, // Native DataNonZoneNonAirloopValue
    pub(crate) DataDXCoolsLowSpeedsAutozize: bool, // Native DataDXCoolsLowSpeedsAutozize
    pub(crate) DataScalableSizingON: bool,     // Native DataScalableSizingON
    pub(crate) DataScalableCapSizingON: bool,  // Native DataScalableCapSizingON
    pub(crate) HRFlowSizingFlag: bool,         // Native HRFlowSizingFlag
    pub(crate) DataFracOfAutosizedCoolingAirflow: f64, // Native DataFracOfAutosizedCoolingAirflow
    pub(crate) DataFracOfAutosizedHeatingAirflow: f64, // Native DataFracOfAutosizedHeatingAirflow
    pub(crate) DataFlowPerCoolingCapacity: f64, // Native DataFlowPerCoolingCapacity
    pub(crate) DataAutosizedCoolingCapacity: f64, // Native DataAutosizedCoolingCapacity
    pub(crate) DataFlowPerHeatingCapacity: f64, // Native DataFlowPerHeatingCapacity
    pub(crate) DataAutosizedHeatingCapacity: f64, // Native DataAutosizedHeatingCapacity
    pub(crate) DataCoilSizingAirInTemp: f64,   // Native DataCoilSizingAirInTemp
    pub(crate) DataCoilSizingAirInHumRat: f64, // Native DataCoilSizingAirInHumRat
    pub(crate) DataCoilSizingAirOutTemp: f64,  // Native DataCoilSizingAirOutTemp
    pub(crate) DataCoilSizingAirOutHumRat: f64, // Native DataCoilSizingAirOutHumRat
    pub(crate) DataCoilSizingFanCoolLoad: f64, // Native DataCoilSizingFanCoolLoad
    pub(crate) DataCoilSizingCapFT: f64,       // Native DataCoilSizingCapFT
    pub(crate) DataTotCapCurveIndex: i32,      // Native DataTotCapCurveIndex
    pub(crate) DataTotCapCurveValue: f64,      // Native DataTotCapCurveValue
    pub(crate) DataFracOfAutosizedCoolingCapacity: f64, // Native DataFracOfAutosizedCoolingCapacity
    pub(crate) DataFracOfAutosizedHeatingCapacity: f64, // Native DataFracOfAutosizedHeatingCapacity
    pub(crate) DataCoilIsSuppHeater: bool,     // Native DataCoilIsSuppHeater
    pub(crate) SuppHeatCap: f64,               // Native SuppHeatCap
    pub(crate) UnitaryHeatCap: f64,            // Native UnitaryHeatCap
    pub(crate) DataCoolCoilType: i32,          // Native DataCoolCoilType
    pub(crate) DataCoolCoilIndex: i32,         // Native DataCoolCoilIndex
    pub(crate) ZoneHVACSizing: Option<Vec<std::convert::Infallible>>, // Native ZoneHVACSizing
    pub(crate) DataZoneNumber: i32,            // Native DataZoneNumber
    pub(crate) AutoVsHardSizingThreshold: f64, // Native AutoVsHardSizingThreshold
}
impl Default for SourceSizingGlobals {
    fn default() -> Self {
        Self {
            CurOASysNum: 0,
            CurDuctType: -1,
            TermUnitSingDuct: false,
            TermUnitPIU: false,
            TermUnitIU: false,
            ZoneEqFanCoil: false,
            ZoneEqUnitHeater: false,
            ZoneEqUnitVent: false,
            ZoneEqVentedSlab: false,
            ZoneSizingInput: None,
            FinalZoneSizing: None,
            TermUnitFinalZoneSizing: None,
            SysSizInput: None,
            FinalSysSizing: None,
            PlantSizData: None,
            DataEMSOverrideON: false,
            DataEMSOverride: 0.0,
            DataAutosizable: true,
            DataConstantUsedForSizing: 0.0,
            DataFractionUsedForSizing: 0.0,
            DataFanIndex: -1,
            DataFanType: -1,
            DataPltSizHeatNum: 0,
            DataWaterLoopNum: 0,
            DataPltSizCoolNum: 0,
            DataWaterCoilSizHeatDeltaT: 0.0,
            DataWaterCoilSizCoolDeltaT: 0.0,
            DataCapacityUsedForSizing: 0.0,
            DataHeatSizeRatio: 1.0,
            DataAirFlowUsedForSizing: 0.0,
            DataDesInletAirTemp: 0.0,
            DataDesAccountForFanHeat: true,
            DataFanPlacement: -1,
            DataDesInletAirHumRat: 0.0,
            DataDesOutletAirHumRat: 0.0,
            DataDesOutletAirTemp: 0.0,
            DataDesInletWaterTemp: 0.0,
            DataFlowUsedForSizing: 0.0,
            DataWaterFlowUsedForSizing: 0.0,
            DataSizingFraction: 1.0,
            DataDXSpeedNum: 0,
            DataDesicRegCoil: false,
            DataZoneUsedForSizing: 0,
            DataDesicDehumNum: 0,
            DataNomCapInpMeth: false,
            DataCoilNum: 0,
            DataFanOp: -1,
            DataDesignCoilCapacity: 0.0,
            DataErrorsFound: false,
            DataBypassFrac: 0.0,
            DataIsDXCoil: false,
            DataNonZoneNonAirloopValue: 0.0,
            DataDXCoolsLowSpeedsAutozize: false,
            DataScalableSizingON: false,
            DataScalableCapSizingON: false,
            HRFlowSizingFlag: false,
            DataFracOfAutosizedCoolingAirflow: 1.0,
            DataFracOfAutosizedHeatingAirflow: 1.0,
            DataFlowPerCoolingCapacity: 0.0,
            DataAutosizedCoolingCapacity: 0.0,
            DataFlowPerHeatingCapacity: 0.0,
            DataAutosizedHeatingCapacity: 0.0,
            DataCoilSizingAirInTemp: 0.0,
            DataCoilSizingAirInHumRat: 0.0,
            DataCoilSizingAirOutTemp: 0.0,
            DataCoilSizingAirOutHumRat: 0.0,
            DataCoilSizingFanCoolLoad: 0.0,
            DataCoilSizingCapFT: 1.0,
            DataTotCapCurveIndex: 0,
            DataTotCapCurveValue: 0.0,
            DataFracOfAutosizedCoolingCapacity: 1.0,
            DataFracOfAutosizedHeatingCapacity: 1.0,
            DataCoilIsSuppHeater: false,
            SuppHeatCap: 0.0,
            UnitaryHeatCap: 0.0,
            DataCoolCoilType: -1,
            DataCoolCoilIndex: -1,
            ZoneHVACSizing: None,
            DataZoneNumber: 0,
            AutoVsHardSizingThreshold: 0.1,
        }
    }
}
