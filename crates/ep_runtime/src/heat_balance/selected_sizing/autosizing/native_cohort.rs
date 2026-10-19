//! One existing process-main cohort selector; true Native full-method answers only.
#![allow(non_snake_case,dead_code)]
use super::*;
use crate::heat_balance::ordinary_equipment::{OrdinaryEquipment,source_purchased_air::owners::{SourceZonePurchasedAir,SourcePurchasedAirNumericFieldData,SourcePurchasedAirLimit}};
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::source_output_registration::SourceOutputDictionaryStreams;
use crate::weather::day::WeatherEnvironmentState;
use serde_json::{Value,json,Map};
type Result<T=()> = std::result::Result<T,Box<dyn std::error::Error>>;
// This compiler-captured fixture is written ONLY from accepted Root Native43
// output after its real run, and includes named INPUT provenance separately.
include!("native_cohort/fixtures/native_hard_size.rs");
fn invalid(s:&str)->std::io::Error{std::io::Error::new(std::io::ErrorKind::InvalidData,s)}
fn number(v:f64)->Value{assert!(v.is_finite());json!({"value_bits":format!("{:016x}",v.to_bits())})}
fn real(v:&Value)->Result<f64>{let s=v.get("value_bits").and_then(Value::as_str).ok_or_else(||invalid("raw16hex"))?;if s.len()!=16||v.as_object().map_or(true,|x|x.len()!=1){return Err(invalid("raw16hex scalar only").into());}let x=f64::from_bits(u64::from_str_radix(s,16)?);if !x.is_finite(){return Err(invalid("finite input").into());}Ok(x)}
fn shape<T>(a:&Option<Vec<T>>)->Value{json!({"allocated":a.is_some(),"size":a.as_ref().map_or(0,Vec::len)})}
fn array1<T>(a:&Option<Vec<T>>)->Value{let mut v=shape(a);v["lower"]=json!(1);v["upper"]=json!(a.as_ref().map_or(0,Vec::len));v}
fn integers(a:&Option<Vec<i32>>)->Value{let mut v=array1(a);v["items"]=json!(a.as_deref().unwrap_or(&[]));v}
fn zone_row(r:&SourceZoneEquipmentSizing)->Value{let mut out=Map::new();
    out.insert("AirVolFlow".into(),number(r.air__vol__flow));
    out.insert("MaxHWVolFlow".into(),number(r.max_hw_vol__flow));
    out.insert("MaxCWVolFlow".into(),number(r.max_cw_vol__flow));
    out.insert("OAVolFlow".into(),number(r.oa_vol__flow));
    out.insert("ATMixerVolFlow".into(),number(r.at_mixer__vol__flow));
    out.insert("ATMixerCoolPriDryBulb".into(),number(r.at_mixer__cool__pri__dry__bulb));
    out.insert("ATMixerCoolPriHumRat".into(),number(r.at_mixer__cool__pri__hum__rat));
    out.insert("ATMixerHeatPriDryBulb".into(),number(r.at_mixer__heat__pri__dry__bulb));
    out.insert("ATMixerHeatPriHumRat".into(),number(r.at_mixer__heat__pri__hum__rat));
    out.insert("DesCoolingLoad".into(),number(r.des__cooling__load));
    out.insert("DesHeatingLoad".into(),number(r.des__heating__load));
    out.insert("CoolingAirVolFlow".into(),number(r.cooling__air__vol__flow));
    out.insert("HeatingAirVolFlow".into(),number(r.heating__air__vol__flow));
    out.insert("SystemAirVolFlow".into(),number(r.system__air__vol__flow));
    out.insert("AirFlow".into(),json!(r.air__flow));
    out.insert("CoolingAirFlow".into(),json!(r.cooling__air__flow));
    out.insert("HeatingAirFlow".into(),json!(r.heating__air__flow));
    out.insert("SystemAirFlow".into(),json!(r.system__air__flow));
    out.insert("Capacity".into(),json!(r.capacity));
    out.insert("CoolingCapacity".into(),json!(r.cooling__capacity));
    out.insert("HeatingCapacity".into(),json!(r.heating__capacity));
    out.insert("SystemCapacity".into(),json!(r.system__capacity));
    out.insert("DesignSizeFromParent".into(),json!(r.design__size__from__parent));
    out.insert("HVACSizingIndex".into(),json!(r.hvac_sizing__index));
 out.insert("SizingMethod".into(),integers(&r.sizing__method));out.insert("CapSizingMethod".into(),integers(&r.cap__sizing__method));Value::Object(out)}
fn zone_array(a:&Option<Vec<SourceZoneEquipmentSizing>>)->Value{let mut v=shape(a);v["items"]=Value::Array(a.as_deref().unwrap_or(&[]).iter().map(zone_row).collect());v}
fn local(s:&SourceBaseSizer)->Value{let mut out=Map::new();
    out.insert("stdRhoAir".into(),number(s.stdRhoAir));
    out.insert("zoneAirFlowSizMethod".into(),json!(s.zoneAirFlowSizMethod));
    out.insert("dataScalableSizingON".into(),json!(s.dataScalableSizingON));
    out.insert("dataScalableCapSizingON".into(),json!(s.dataScalableCapSizingON));
    out.insert("isCoilReportObject".into(),json!(s.isCoilReportObject));
    out.insert("isFanReportObject".into(),json!(s.isFanReportObject));
    out.insert("initialized".into(),json!(s.initialized));
    out.insert("errorType".into(),json!(s.errorType));
    out.insert("sizingType".into(),json!(s.sizingType));
    out.insert("sizingString".into(),json!(s.sizingString));
    out.insert("sizingStringScalable".into(),json!(s.sizingStringScalable));
    out.insert("overrideSizeString".into(),json!(s.overrideSizeString));
    out.insert("originalValue".into(),number(s.originalValue));
    out.insert("autoSizedValue".into(),number(s.autoSizedValue));
    out.insert("wasAutoSized".into(),json!(s.wasAutoSized));
    out.insert("hardSizeNoDesignRun".into(),json!(s.hardSizeNoDesignRun));
    out.insert("sizingDesRunThisAirSys".into(),json!(s.sizingDesRunThisAirSys));
    out.insert("sizingDesRunThisZone".into(),json!(s.sizingDesRunThisZone));
    out.insert("sizingDesValueFromParent".into(),json!(s.sizingDesValueFromParent));
    out.insert("airLoopSysFlag".into(),json!(s.airLoopSysFlag));
    out.insert("oaSysFlag".into(),json!(s.oaSysFlag));
    out.insert("coilType_Num".into(),json!(s.coilType_Num));
    out.insert("compType".into(),json!(s.compType));
    out.insert("compName".into(),json!(s.compName));
    out.insert("isEpJSON".into(),json!(s.isEpJSON));
    out.insert("sysSizingRunDone".into(),json!(s.sysSizingRunDone));
    out.insert("zoneSizingRunDone".into(),json!(s.zoneSizingRunDone));
    out.insert("curSysNum".into(),json!(s.curSysNum));
    out.insert("curOASysNum".into(),json!(s.curOASysNum));
    out.insert("curZoneEqNum".into(),json!(s.curZoneEqNum));
    out.insert("curDuctType".into(),json!(s.curDuctType));
    out.insert("curTermUnitSizingNum".into(),json!(s.curTermUnitSizingNum));
    out.insert("numPrimaryAirSys".into(),json!(s.numPrimaryAirSys));
    out.insert("numSysSizInput".into(),json!(s.numSysSizInput));
    out.insert("doSystemSizing".into(),json!(s.doSystemSizing));
    out.insert("numZoneSizingInput".into(),json!(s.numZoneSizingInput));
    out.insert("doZoneSizing".into(),json!(s.doZoneSizing));
    out.insert("autoCalculate".into(),json!(s.autoCalculate));
    out.insert("termUnitSingDuct".into(),json!(s.termUnitSingDuct));
    out.insert("termUnitPIU".into(),json!(s.termUnitPIU));
    out.insert("termUnitIU".into(),json!(s.termUnitIU));
    out.insert("zoneEqFanCoil".into(),json!(s.zoneEqFanCoil));
    out.insert("otherEqType".into(),json!(s.otherEqType));
    out.insert("zoneEqUnitHeater".into(),json!(s.zoneEqUnitHeater));
    out.insert("zoneEqUnitVent".into(),json!(s.zoneEqUnitVent));
    out.insert("zoneEqVentedSlab".into(),json!(s.zoneEqVentedSlab));
    out.insert("minOA".into(),json!(s.minOA));
    out.insert("dataEMSOverrideON".into(),json!(s.dataEMSOverrideON));
    out.insert("dataEMSOverride".into(),number(s.dataEMSOverride));
    out.insert("dataAutosizable".into(),json!(s.dataAutosizable));
    out.insert("dataConstantUsedForSizing".into(),number(s.dataConstantUsedForSizing));
    out.insert("dataFractionUsedForSizing".into(),number(s.dataFractionUsedForSizing));
    out.insert("dataDXCoolsLowSpeedsAutozize".into(),json!(s.dataDXCoolsLowSpeedsAutozize));
    out.insert("dataPltSizHeatNum".into(),json!(s.dataPltSizHeatNum));
    out.insert("dataWaterLoopNum".into(),json!(s.dataWaterLoopNum));
    out.insert("dataFanIndex".into(),json!(s.dataFanIndex));
    out.insert("dataFanType".into(),json!(s.dataFanType));
    out.insert("dataWaterCoilSizCoolDeltaT".into(),number(s.dataWaterCoilSizCoolDeltaT));
    out.insert("dataWaterCoilSizHeatDeltaT".into(),number(s.dataWaterCoilSizHeatDeltaT));
    out.insert("dataCapacityUsedForSizing".into(),number(s.dataCapacityUsedForSizing));
    out.insert("dataPltSizCoolNum".into(),json!(s.dataPltSizCoolNum));
    out.insert("dataDesInletAirHumRat".into(),number(s.dataDesInletAirHumRat));
    out.insert("dataAirFlowUsedForSizing".into(),number(s.dataAirFlowUsedForSizing));
    out.insert("dataDesInletAirTemp".into(),number(s.dataDesInletAirTemp));
    out.insert("dataDesAccountForFanHeat".into(),json!(s.dataDesAccountForFanHeat));
    out.insert("dataFanPlacement".into(),json!(s.dataFanPlacement));
    out.insert("dataFlowUsedForSizing".into(),number(s.dataFlowUsedForSizing));
    out.insert("dataDesOutletAirHumRat".into(),number(s.dataDesOutletAirHumRat));
    out.insert("dataDesInletWaterTemp".into(),number(s.dataDesInletWaterTemp));
    out.insert("dataDesOutletAirTemp".into(),number(s.dataDesOutletAirTemp));
    out.insert("dataWaterFlowUsedForSizing".into(),number(s.dataWaterFlowUsedForSizing));
    out.insert("dataSizingFraction".into(),number(s.dataSizingFraction));
    out.insert("dataDXSpeedNum".into(),json!(s.dataDXSpeedNum));
    out.insert("dataDesicRegCoil".into(),json!(s.dataDesicRegCoil));
    out.insert("dataHeatSizeRatio".into(),number(s.dataHeatSizeRatio));
    out.insert("dataZoneUsedForSizing".into(),json!(s.dataZoneUsedForSizing));
    out.insert("dataDesicDehumNum".into(),json!(s.dataDesicDehumNum));
    out.insert("dataNomCapInpMeth".into(),json!(s.dataNomCapInpMeth));
    out.insert("dataCoilNum".into(),json!(s.dataCoilNum));
    out.insert("dataFanOp".into(),json!(s.dataFanOp));
    out.insert("dataDesignCoilCapacity".into(),number(s.dataDesignCoilCapacity));
    out.insert("dataErrorsFound".into(),json!(s.dataErrorsFound));
    out.insert("dataBypassFrac".into(),number(s.dataBypassFrac));
    out.insert("dataIsDXCoil".into(),json!(s.dataIsDXCoil));
    out.insert("dataNonZoneNonAirloopValue".into(),number(s.dataNonZoneNonAirloopValue));
    out.insert("printWarningFlag".into(),json!(s.printWarningFlag));
    out.insert("callingRoutine".into(),json!(s.callingRoutine));
    out.insert("sysSizingInputData".into(),shape(&s.sysSizingInputData));
    out.insert("zoneSizingInput".into(),shape(&s.zoneSizingInput));
    out.insert("unitarySysEqSizing".into(),zone_array(&s.unitarySysEqSizing));
    out.insert("oaSysEqSizing".into(),zone_array(&s.oaSysEqSizing));
    out.insert("zoneEqSizing".into(),zone_array(&s.zoneEqSizing));
    out.insert("outsideAirSys".into(),shape(&s.outsideAirSys));
    out.insert("termUnitSizing".into(),shape(&s.termUnitSizing));
    out.insert("termUnitFinalZoneSizing".into(),shape(&s.termUnitFinalZoneSizing));
    out.insert("finalZoneSizing".into(),shape(&s.finalZoneSizing));
    out.insert("finalSysSizing".into(),shape(&s.finalSysSizing));
    out.insert("plantSizData".into(),shape(&s.plantSizData));
    out.insert("primaryAirSystem".into(),shape(&s.primaryAirSystem));
    out.insert("airloopDOAS".into(),json!({"size":s.airloopDOAS.len(),"container":"std::vector"}));
    out.insert("airLoopControlInfo".into(),shape(&s.airLoopControlInfo));
    out.insert("deltaP".into(),number(s.deltaP));
    out.insert("motEff".into(),number(s.motEff));
    out.insert("totEff".into(),number(s.totEff));
    out.insert("motInAirFrac".into(),number(s.motInAirFrac));
    out.insert("fanCompModel".into(),json!(s.fanCompModel));
    out.insert("fanShaftPow".into(),number(s.fanShaftPow));
    out.insert("motInPower".into(),number(s.motInPower));
    out.insert("zoneCoolingOnlyFan".into(),json!(s.zoneCoolingOnlyFan));
    out.insert("zoneHeatingOnlyFan".into(),json!(s.zoneHeatingOnlyFan));
    out.insert("dataHRFlowSizingFlag".into(),json!(s.dataHRFlowSizingFlag));
    out.insert("dataFracOfAutosizedCoolingAirflow".into(),number(s.dataFracOfAutosizedCoolingAirflow));
    out.insert("dataFracOfAutosizedHeatingAirflow".into(),number(s.dataFracOfAutosizedHeatingAirflow));
    out.insert("dataFlowPerCoolingCapacity".into(),number(s.dataFlowPerCoolingCapacity));
    out.insert("dataAutosizedCoolingCapacity".into(),number(s.dataAutosizedCoolingCapacity));
    out.insert("dataFlowPerHeatingCapacity".into(),number(s.dataFlowPerHeatingCapacity));
    out.insert("dataAutosizedHeatingCapacity".into(),number(s.dataAutosizedHeatingCapacity));
    out.insert("dataCoilSizingAirInTemp".into(),number(s.dataCoilSizingAirInTemp));
    out.insert("dataCoilSizingAirInHumRat".into(),number(s.dataCoilSizingAirInHumRat));
    out.insert("dataCoilSizingAirOutTemp".into(),number(s.dataCoilSizingAirOutTemp));
    out.insert("dataCoilSizingAirOutHumRat".into(),number(s.dataCoilSizingAirOutHumRat));
    out.insert("dataCoilSizingFanCoolLoad".into(),number(s.dataCoilSizingFanCoolLoad));
    out.insert("dataCoilSizingCapFT".into(),number(s.dataCoilSizingCapFT));
    out.insert("dataTotCapCurveValue".into(),number(s.dataTotCapCurveValue));
    out.insert("dataFracOfAutosizedCoolingCapacity".into(),number(s.dataFracOfAutosizedCoolingCapacity));
    out.insert("dataFracOfAutosizedHeatingCapacity".into(),number(s.dataFracOfAutosizedHeatingCapacity));
    out.insert("dataCoolCoilCap".into(),number(s.dataCoolCoilCap));
    out.insert("dataCoilIsSuppHeater".into(),json!(s.dataCoilIsSuppHeater));
    out.insert("suppHeatCap".into(),number(s.suppHeatCap));
    out.insert("unitaryHeatCap".into(),number(s.unitaryHeatCap));
    out.insert("dataTotCapCurveIndex".into(),json!(s.dataTotCapCurveIndex));
    out.insert("dataCoolCoilType".into(),json!(s.dataCoolCoilType));
    out.insert("dataCoolCoilIndex".into(),json!(s.dataCoolCoilIndex));
    out.insert("zoneHVACSizingIndex".into(),json!(s.zoneHVACSizingIndex));
    out.insert("zoneHVACSizing".into(),shape(&s.zoneHVACSizing));
    out.insert("lastErrorMessages".into(),json!(s.lastErrorMessages));
 Value::Object(out)}
struct State{sizing:SelectedSizingManager,globals:SelectedHeatBalanceGlobals,hvac:SelectedHvacGlobals,air_loop:SourceAirLoopData,environment:WeatherEnvironmentState,registry:PredefinedRegistry,eio:Vec<u8>}
impl State{fn new()->Self{let mut sizing=SelectedSizingManager::default();sizing.current_zone_equipment_number=1;let mut row=SourceZoneEquipmentSizing::default();row.sizing__method=Some(vec![0;35]);sizing.zone_equipment_sizing=Some(vec![row]);let mut environment=WeatherEnvironmentState::default();environment.std_rho_air=1.2;Self{sizing,globals:Default::default(),hvac:Default::default(),air_loop:Default::default(),environment,registry:Default::default(),eio:Vec::new()}}}
fn globals(s:&State)->Value{let mut out=Map::new();
    out.insert("CurOASysNum".into(),json!(s.sizing.autosizing.CurOASysNum));
    out.insert("CurDuctType".into(),json!(s.sizing.autosizing.CurDuctType));
    out.insert("TermUnitSingDuct".into(),json!(s.sizing.autosizing.TermUnitSingDuct));
    out.insert("TermUnitPIU".into(),json!(s.sizing.autosizing.TermUnitPIU));
    out.insert("TermUnitIU".into(),json!(s.sizing.autosizing.TermUnitIU));
    out.insert("ZoneEqFanCoil".into(),json!(s.sizing.autosizing.ZoneEqFanCoil));
    out.insert("ZoneEqUnitHeater".into(),json!(s.sizing.autosizing.ZoneEqUnitHeater));
    out.insert("ZoneEqUnitVent".into(),json!(s.sizing.autosizing.ZoneEqUnitVent));
    out.insert("ZoneEqVentedSlab".into(),json!(s.sizing.autosizing.ZoneEqVentedSlab));
    out.insert("ZoneSizingInput".into(),shape(&s.sizing.autosizing.ZoneSizingInput));
    out.insert("FinalZoneSizing".into(),shape(&s.sizing.autosizing.FinalZoneSizing));
    out.insert("TermUnitFinalZoneSizing".into(),shape(&s.sizing.autosizing.TermUnitFinalZoneSizing));
    out.insert("SysSizInput".into(),shape(&s.sizing.autosizing.SysSizInput));
    out.insert("FinalSysSizing".into(),shape(&s.sizing.autosizing.FinalSysSizing));
    out.insert("PlantSizData".into(),shape(&s.sizing.autosizing.PlantSizData));
    out.insert("DataEMSOverrideON".into(),json!(s.sizing.autosizing.DataEMSOverrideON));
    out.insert("DataEMSOverride".into(),number(s.sizing.autosizing.DataEMSOverride));
    out.insert("DataAutosizable".into(),json!(s.sizing.autosizing.DataAutosizable));
    out.insert("DataConstantUsedForSizing".into(),number(s.sizing.autosizing.DataConstantUsedForSizing));
    out.insert("DataFractionUsedForSizing".into(),number(s.sizing.autosizing.DataFractionUsedForSizing));
    out.insert("DataFanIndex".into(),json!(s.sizing.autosizing.DataFanIndex));
    out.insert("DataFanType".into(),json!(s.sizing.autosizing.DataFanType));
    out.insert("DataPltSizHeatNum".into(),json!(s.sizing.autosizing.DataPltSizHeatNum));
    out.insert("DataWaterLoopNum".into(),json!(s.sizing.autosizing.DataWaterLoopNum));
    out.insert("DataPltSizCoolNum".into(),json!(s.sizing.autosizing.DataPltSizCoolNum));
    out.insert("DataWaterCoilSizHeatDeltaT".into(),number(s.sizing.autosizing.DataWaterCoilSizHeatDeltaT));
    out.insert("DataWaterCoilSizCoolDeltaT".into(),number(s.sizing.autosizing.DataWaterCoilSizCoolDeltaT));
    out.insert("DataCapacityUsedForSizing".into(),number(s.sizing.autosizing.DataCapacityUsedForSizing));
    out.insert("DataHeatSizeRatio".into(),number(s.sizing.autosizing.DataHeatSizeRatio));
    out.insert("DataAirFlowUsedForSizing".into(),number(s.sizing.autosizing.DataAirFlowUsedForSizing));
    out.insert("DataDesInletAirTemp".into(),number(s.sizing.autosizing.DataDesInletAirTemp));
    out.insert("DataDesAccountForFanHeat".into(),json!(s.sizing.autosizing.DataDesAccountForFanHeat));
    out.insert("DataFanPlacement".into(),json!(s.sizing.autosizing.DataFanPlacement));
    out.insert("DataDesInletAirHumRat".into(),number(s.sizing.autosizing.DataDesInletAirHumRat));
    out.insert("DataDesOutletAirHumRat".into(),number(s.sizing.autosizing.DataDesOutletAirHumRat));
    out.insert("DataDesOutletAirTemp".into(),number(s.sizing.autosizing.DataDesOutletAirTemp));
    out.insert("DataDesInletWaterTemp".into(),number(s.sizing.autosizing.DataDesInletWaterTemp));
    out.insert("DataFlowUsedForSizing".into(),number(s.sizing.autosizing.DataFlowUsedForSizing));
    out.insert("DataWaterFlowUsedForSizing".into(),number(s.sizing.autosizing.DataWaterFlowUsedForSizing));
    out.insert("DataSizingFraction".into(),number(s.sizing.autosizing.DataSizingFraction));
    out.insert("DataDXSpeedNum".into(),json!(s.sizing.autosizing.DataDXSpeedNum));
    out.insert("DataDesicRegCoil".into(),json!(s.sizing.autosizing.DataDesicRegCoil));
    out.insert("DataZoneUsedForSizing".into(),json!(s.sizing.autosizing.DataZoneUsedForSizing));
    out.insert("DataDesicDehumNum".into(),json!(s.sizing.autosizing.DataDesicDehumNum));
    out.insert("DataNomCapInpMeth".into(),json!(s.sizing.autosizing.DataNomCapInpMeth));
    out.insert("DataCoilNum".into(),json!(s.sizing.autosizing.DataCoilNum));
    out.insert("DataFanOp".into(),json!(s.sizing.autosizing.DataFanOp));
    out.insert("DataDesignCoilCapacity".into(),number(s.sizing.autosizing.DataDesignCoilCapacity));
    out.insert("DataErrorsFound".into(),json!(s.sizing.autosizing.DataErrorsFound));
    out.insert("DataBypassFrac".into(),number(s.sizing.autosizing.DataBypassFrac));
    out.insert("DataIsDXCoil".into(),json!(s.sizing.autosizing.DataIsDXCoil));
    out.insert("DataNonZoneNonAirloopValue".into(),number(s.sizing.autosizing.DataNonZoneNonAirloopValue));
    out.insert("DataDXCoolsLowSpeedsAutozize".into(),json!(s.sizing.autosizing.DataDXCoolsLowSpeedsAutozize));
    out.insert("DataScalableSizingON".into(),json!(s.sizing.autosizing.DataScalableSizingON));
    out.insert("DataScalableCapSizingON".into(),json!(s.sizing.autosizing.DataScalableCapSizingON));
    out.insert("HRFlowSizingFlag".into(),json!(s.sizing.autosizing.HRFlowSizingFlag));
    out.insert("DataFracOfAutosizedCoolingAirflow".into(),number(s.sizing.autosizing.DataFracOfAutosizedCoolingAirflow));
    out.insert("DataFracOfAutosizedHeatingAirflow".into(),number(s.sizing.autosizing.DataFracOfAutosizedHeatingAirflow));
    out.insert("DataFlowPerCoolingCapacity".into(),number(s.sizing.autosizing.DataFlowPerCoolingCapacity));
    out.insert("DataAutosizedCoolingCapacity".into(),number(s.sizing.autosizing.DataAutosizedCoolingCapacity));
    out.insert("DataFlowPerHeatingCapacity".into(),number(s.sizing.autosizing.DataFlowPerHeatingCapacity));
    out.insert("DataAutosizedHeatingCapacity".into(),number(s.sizing.autosizing.DataAutosizedHeatingCapacity));
    out.insert("DataCoilSizingAirInTemp".into(),number(s.sizing.autosizing.DataCoilSizingAirInTemp));
    out.insert("DataCoilSizingAirInHumRat".into(),number(s.sizing.autosizing.DataCoilSizingAirInHumRat));
    out.insert("DataCoilSizingAirOutTemp".into(),number(s.sizing.autosizing.DataCoilSizingAirOutTemp));
    out.insert("DataCoilSizingAirOutHumRat".into(),number(s.sizing.autosizing.DataCoilSizingAirOutHumRat));
    out.insert("DataCoilSizingFanCoolLoad".into(),number(s.sizing.autosizing.DataCoilSizingFanCoolLoad));
    out.insert("DataCoilSizingCapFT".into(),number(s.sizing.autosizing.DataCoilSizingCapFT));
    out.insert("DataTotCapCurveIndex".into(),json!(s.sizing.autosizing.DataTotCapCurveIndex));
    out.insert("DataTotCapCurveValue".into(),number(s.sizing.autosizing.DataTotCapCurveValue));
    out.insert("DataFracOfAutosizedCoolingCapacity".into(),number(s.sizing.autosizing.DataFracOfAutosizedCoolingCapacity));
    out.insert("DataFracOfAutosizedHeatingCapacity".into(),number(s.sizing.autosizing.DataFracOfAutosizedHeatingCapacity));
    out.insert("DataCoilIsSuppHeater".into(),json!(s.sizing.autosizing.DataCoilIsSuppHeater));
    out.insert("SuppHeatCap".into(),number(s.sizing.autosizing.SuppHeatCap));
    out.insert("UnitaryHeatCap".into(),number(s.sizing.autosizing.UnitaryHeatCap));
    out.insert("DataCoolCoilType".into(),json!(s.sizing.autosizing.DataCoolCoilType));
    out.insert("DataCoolCoilIndex".into(),json!(s.sizing.autosizing.DataCoolCoilIndex));
    out.insert("ZoneHVACSizing".into(),shape(&s.sizing.autosizing.ZoneHVACSizing));
    out.insert("DataZoneNumber".into(),json!(s.sizing.autosizing.DataZoneNumber));
    out.insert("AutoVsHardSizingThreshold".into(),number(s.sizing.autosizing.AutoVsHardSizingThreshold));
    out.insert("SysSizingRunDone".into(),json!(s.sizing.system_sizing_run_done));
    out.insert("ZoneSizingRunDone".into(),json!(s.sizing.zone_sizing_run_done));
    out.insert("CurSysNum".into(),json!(s.sizing.current_system_number));
    out.insert("CurZoneEqNum".into(),json!(s.sizing.current_zone_equipment_number));
    out.insert("CurTermUnitSizingNum".into(),json!(s.sizing.current_terminal_unit_sizing_number));
    out.insert("NumSysSizInput".into(),json!(s.sizing.number_of_system_sizing_input));
    out.insert("NumZoneSizingInput".into(),json!(s.sizing.number_of_zone_sizing_input));
    out.insert("UnitarySysEqSizing".into(),zone_array(&s.sizing.unitary_system_equipment_sizing));
    out.insert("OASysEqSizing".into(),zone_array(&s.sizing.outside_air_system_equipment_sizing));
    out.insert("TermUnitSizing".into(),shape(&s.sizing.terminal_unit_sizing));
    out.insert("ZoneEqSizing".into(),zone_array(&s.sizing.zone_equipment_sizing));
    out.insert("ZoneCoolingOnlyFan".into(),json!(s.sizing.zone_cooling_only_fan));
    out.insert("ZoneHeatingOnlyFan".into(),json!(s.sizing.zone_heating_only_fan));
    out.insert("DataCoolCoilCap".into(),number(s.sizing.data_cool_coil_capacity));
 out.insert("StdRhoAir".into(),number(s.environment.std_rho_air));out.insert("isEpJSON".into(),json!(false));out.insert("DoZoneSizing".into(),json!(s.globals.do_zone_sizing));out.insert("DoSystemSizing".into(),json!(s.globals.do_system_sizing));out.insert("DisplayExtraWarnings".into(),json!(s.globals.display_extra_warnings));out.insert("NumPrimaryAirSys".into(),json!(s.hvac.number_of_primary_air_systems));Value::Object(out)}
fn reports(s:&State)->Result<Value>{let mut rows=array1(&s.registry.component_sizing_entries);rows["items"]=Value::Array(s.registry.component_sizing_entries.as_deref().unwrap_or(&[]).iter().map(|r|json!({"typeField":r.component_type,"nameField":r.component_name,"description":r.description,"valField":number(r.value),"strField":r.string_value,"active":r.active,"written":r.written})).collect());Ok(json!({"oneTimeCompRptHeaderFlag":s.environment.one_time_component_report_header_flag,"eio":std::str::from_utf8(&s.eio)?,"sqlite_present":false,"sizeCompSizeTableEntry":s.registry.component_sizing_capacity,"numCompSizeTableEntry":s.registry.component_sizing_count,"CompSizeTableEntry":rows}))}
fn unit(e:&OrdinaryEquipment)->Result<Value>{let p=e.purchased_air.units.as_ref().and_then(|x|x.first()).ok_or_else(||invalid("unit1"))?;let mut out=Map::new();out.insert("Name".into(),json!(p.name));out.insert("cObjectName".into(),json!(p.c_object_name));out.insert("HeatingLimit".into(),json!(p.heating_limit as i32));out.insert("MaxHeatVolFlowRate".into(),number(p.max_heat_vol_flow_rate));out.insert("MaxHeatSensCap".into(),number(p.max_heat_sens_cap));out.insert("CoolingLimit".into(),json!(p.cooling_limit as i32));out.insert("MaxCoolVolFlowRate".into(),number(p.max_cool_vol_flow_rate));out.insert("MaxCoolTotCap".into(),number(p.max_cool_tot_cap));out.insert("HVACSizingIndex".into(),json!(p.hvac_sizing_index));out.insert("ZonePtr".into(),json!(p.zone_ptr));Ok(Value::Object(out))}
fn limit(v:&str)->Result<SourcePurchasedAirLimit>{match v{"LimitFlowRate"=>Ok(SourcePurchasedAirLimit::FlowRate),"LimitCapacity"=>Ok(SourcePurchasedAirLimit::Capacity),"LimitFlowRateAndCapacity"=>Ok(SourcePurchasedAirLimit::FlowRateAndCapacity),_=>Err(invalid("actual limit enum").into())}}
fn setup(input:&Value)->Result<OrdinaryEquipment>{let mut e=OrdinaryEquipment::source_sizer_cohort_cold_owner()?;let mut row=SourceZonePurchasedAir::default();row.name=input["Name"].as_str().ok_or_else(||invalid("name"))?.into();row.c_object_name=input["cObjectName"].as_str().ok_or_else(||invalid("object"))?.into();row.heating_limit=limit(input["HeatingLimit"].as_str().ok_or_else(||invalid("heat limit"))?)?;row.cooling_limit=limit(input["CoolingLimit"].as_str().ok_or_else(||invalid("cool limit"))?)?;row.max_heat_vol_flow_rate=real(&input["MaxHeatVolFlowRate"])?;row.max_heat_sens_cap=real(&input["MaxHeatSensCap"])?;row.max_cool_vol_flow_rate=real(&input["MaxCoolVolFlowRate"])?;row.max_cool_tot_cap=real(&input["MaxCoolTotCap"])?;e.purchased_air.number_purchased_air=1;e.purchased_air.units=Some(vec![row]);let names=input["numeric_field_names"].as_array().ok_or_else(||invalid("actual field names"))?.iter().map(|x|x.as_str().map(str::to_owned).ok_or_else(||invalid("fieldname"))).collect::<std::result::Result<Vec<_>,_>>()?;e.purchased_air.numeric_fields=Some(vec![SourcePurchasedAirNumericFieldData{field_names:Some(names)}]);Ok(e)}
fn call_size(s:&mut State,e:&mut OrdinaryEquipment,name:&str,origin:Value)->Result<Value>{let entry=json!({"globals":globals(s),"reports":reports(s)?,"unit":unit(e)?});let mut rf=Vec::new();let dict=SourceOutputDictionaryStreams{eso:None,sqlite_present:false,results_framework_output_variables:&mut rf};{let mut report=SourceSizerReportingOwners{registry:&mut s.registry,environment:&mut s.environment,dictionary_host:&dict,eio:&mut s.eio};e.source_sizer_cohort_size_bridge(&mut s.sizing,&s.globals,&s.hvac,&s.air_loop,&mut report)?;}Ok(json!({"kind":"full_SizePurchasedAir","name":name,"origin":origin,"entry":entry,"exit":{"globals":globals(s),"reports":reports(s)?,"unit":unit(e)?}}))}
fn kind(n:usize)->Result<SourceAutoSizingType>{match n{0=>Ok(SourceAutoSizingType::HeatingAirFlow),1=>Ok(SourceAutoSizingType::HeatingCapacity),2=>Ok(SourceAutoSizingType::CoolingAirFlow),3=>Ok(SourceAutoSizingType::CoolingCapacity),_=>Err(invalid("actual Sizer method ordinal").into())}}
fn call_direct(s:&mut State,n:usize,name:&str,description:&str,value:f64,print:bool,origin:Value)->Result<Value>{let kind=kind(n)?;let method=*["HeatingAirFlowSizer","HeatingCapacitySizer","CoolingAirFlowSizer","CoolingCapacitySizer"].get(n).ok_or_else(||invalid("actual method name"))?;let mut local_sizer=SourceBaseSizer::new(kind);let constructor=local(&local_sizer);let before=globals(s);let reports_before=reports(s)?;local_sizer.override_sizing_string(description);local_sizer.initialize(&mut s.sizing,&s.globals,&s.hvac,&s.air_loop,&false,&s.environment,"ZoneHVAC:IdealLoadsAirSystem","ZONE ONE IDEAL LOADS",print)?;let initialized=local(&local_sizer);let after_init=globals(s);let mut errors=false;let mut rf=Vec::new();let dict=SourceOutputDictionaryStreams{eso:None,sqlite_present:false,results_framework_output_variables:&mut rf};let returned;{let mut report=SourceSizerReportingOwners{registry:&mut s.registry,environment:&mut s.environment,dictionary_host:&dict,eio:&mut s.eio};returned=local_sizer.size(kind,value,&mut s.sizing,&s.globals,&mut report,&mut errors)?;}Ok(json!({"kind":"full_Sizer","name":name,"method":method,"origin":origin,"input":number(value),"print_flag":print,"constructor":constructor,"globals_before_initialize":before,"reports_before":reports_before,"initialized":initialized,"globals_after_initialize":after_init,"returned":number(returned),"errorsFound":errors,"exit":local(&local_sizer),"globals_after":globals(s),"reports_after":reports(s)?}))}
fn set_method(s:&mut State,ordinal:usize,value:i32)->Result{let i=ordinal.checked_sub(1).ok_or_else(||invalid("actual positive sizing ordinal"))?;let row=s.sizing.zone_equipment_sizing.as_mut().and_then(|v|v.first_mut()).and_then(|r|r.sizing__method.as_mut()).and_then(|r|r.get_mut(i)).ok_or_else(||invalid("actual sizing method row"))?;*row=value;Ok(())}
fn cp(name:&str,value:f64)->Value{json!({"kind":"public_normal_Cp_probe","name":name,"input":number(value),"returned":number(crate::psychrometrics::energyplus_psy_cp_air_fn_w(value)),"qualification":"Explicit named public call sequence, not direct private static cache state/hitcount observation"})}
struct Check{expected:Value,count:usize}
impl Check{fn case(&mut self,actual:Value){let expected=&self.expected["cases"][self.count];assert_eq!(actual["name"],expected["name"],"case order{}",self.count);assert_eq!(&actual,expected,"full Native case{} {}",self.count,actual["name"]);self.count+=1;}}
pub(crate) fn run()->Result<usize>{let inputs:Value=serde_json::from_str(NATIVE_HARD_SIZE_INPUT_JSON)?;let native:Value=serde_json::from_str(NATIVE_HARD_SIZE_EXPECTED_JSON)?;if native["schema"]!="src06-hard-size-full-Native-unit.v1"||native["case_count"]!=59||native["cases"].as_array().map_or(0,Vec::len)!=59{return Err(invalid("complete genuine59case Native fixture required").into());}let mut c=Check{expected:native,count:0};
 c.case(cp("fresh-process-cold-minus100",-100.0));let actual=&inputs["cases"];let names=actual["BOTH"]["numeric_field_names"].as_array().ok_or_else(||invalid("numeric fields"))?;let field=|i:usize|->Result<&str>{names.get(i).and_then(Value::as_str).ok_or_else(||invalid("actual numeric field name").into())};let desc=[format!("{} [m3/s]",field(4)?),format!("{} [m3/s]",field(5)?),"Maximum Cooling Air Flow Rate [m3/s]".into(),format!("{} [m3/s]",field(7)?)];
 {let mut s=State::new();c.case(call_direct(&mut s,1,"Cp-sequence-HeatingCapacity-positive",&desc[1],120.0,true,json!({"kind":"named_cache_sequence"}))?);}c.case(cp("post-HeatingCapacity-minus100",-100.0));c.case(cp("post-minus100-zero",0.0));{let mut s=State::new();c.case(call_direct(&mut s,1,"Cp-sequence-HeatingCapacity-zero",&desc[1],0.0,true,json!({"kind":"named_cache_sequence"}))?);}c.case(cp("post-zero-Sizer-zero",0.0));
 for method in 0..4{let positive=if method==0||method==2{0.02}else{120.0};for value in [positive,0.0,-1.0]{let mut s=State::new();c.case(call_direct(&mut s,method,&format!("direct-hard-{method}-{value:.6}"),&desc[method],value,true,json!({"kind":"named_direct_input"}))?);} {let mut s=State::new();c.case(call_direct(&mut s,method,&format!("direct-print-false-{method}"),&desc[method],positive,false,json!({"kind":"named_direct_input"}))?);} {let mut s=State::new();s.sizing.autosizing.DataAutosizable=false;c.case(call_direct(&mut s,method,&format!("direct-not-autosizable-{method}"),&desc[method],positive,true,json!({"kind":"named_direct_input","input_overrides":{"DataAutosizable":false}}))?);} {let mut s=State::new();s.sizing.autosizing.DataEMSOverrideON=true;s.sizing.autosizing.DataEMSOverride=7.25;c.case(call_direct(&mut s,method,&format!("direct-EMS-{method}"),&desc[method],positive,true,json!({"kind":"named_direct_input","input_overrides":{"DataEMSOverrideON":true,"DataEMSOverride":number(7.25)}}))?);}}
 for method in [0,2]{let mut s=State::new();s.sizing.autosizing.DataEMSOverrideON=true;s.sizing.autosizing.DataEMSOverride=7.25;s.sizing.autosizing.DataScalableSizingON=true;let ordinal=if method==0{14}else{1};set_method(&mut s,ordinal,4)?;c.case(call_direct(&mut s,method,&format!("airflow-EMS-plus-scalable-{method}"),&desc[method],0.02,true,json!({"kind":"named_generalized_metadata_input"}))?);}
 for method in [1,3]{let mut s=State::new();s.sizing.autosizing.DataEMSOverrideON=true;s.sizing.autosizing.DataEMSOverride=7.25;s.sizing.autosizing.DataScalableCapSizingON=true;let ordinal=if method==1{18}else{17};set_method(&mut s,ordinal,10)?;c.case(call_direct(&mut s,method,&format!("capacity-EMS-plus-scalable-{method}"),&desc[method],120.0,true,json!({"kind":"named_generalized_metadata_input"}))?);}
 for mode in ["FLOW","CAPACITY","BOTH"]{let mut s=State::new();let mut e=setup(&actual[mode])?;c.case(call_size(&mut s,&mut e,&format!("full-SizePurchasedAir-{mode}"),json!({"kind":"actual_case_sizing_user_inputs","case_id":format!("B-{mode}-24H"),"input":actual[mode]}))?);}
 for value in [0.5,f64::from_bits(0x3fefffffffffffff),1.0]{let mut s=State::new();let mut input=actual["CAPACITY"].clone();input["MaxHeatSensCap"]=number(value);let mut e=setup(&input)?;c.case(call_size(&mut s,&mut e,&format!("full-SizePurchasedAir-SmallLoad-{}",value.to_bits()),json!({"kind":"named_user_input_boundary","base_case":"B-CAPACITY-24H","input_override":{"MaxHeatSensCap":number(value)}}))?);}
 {let mut s=State::new();s.sizing.autosizing.DataConstantUsedForSizing=5.0;s.sizing.autosizing.DataFractionUsedForSizing=2.0;c.case(call_direct(&mut s,0,"constant-fraction-first",&desc[0],0.02,true,json!({"kind":"named_shared_state_sequence","sequence":"constant-fraction-reset","ordinal":1}))?);c.case(call_direct(&mut s,2,"constant-fraction-second-no-reseed",&desc[2],0.02,true,json!({"kind":"named_shared_state_sequence","sequence":"constant-fraction-reset","ordinal":2}))?);}
 {let mut s=State::new();let mut e=setup(&actual["BOTH"])?;for i in 1..=17{c.case(call_size(&mut s,&mut e,&format!("repeated-BOTH-{i}"),json!({"kind":"named_shared_state_sequence","sequence":"real-class-row-growth","ordinal":i,"input":actual["BOTH"]}))?);}}
 assert_eq!(c.count,59);Ok(c.count)}
impl SelectedSizingManager{pub(crate) fn run_native_hard_size_cohort()->Result<usize>{run()}}
