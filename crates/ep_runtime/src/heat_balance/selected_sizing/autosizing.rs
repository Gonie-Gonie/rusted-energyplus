//! Genuine Base/FanHeat/Scalable Sizer lifecycle and finite hard-size normal paths.
//! Missing positive design/diagnostic child operations stop at their actual position.
#![allow(non_snake_case, unused_variables, unused_assignments, dead_code)]
mod owners;
mod globals;
pub(crate) use globals::SourceSizingGlobals;
use owners::SourceBaseSizer;
use super::{SelectedSizingManager, SourceZoneEquipmentSizing};
use super::sizer_output::{SourceSizerReportingOwners, report_sizer_output};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::ordinary_hvac_manager::simulation::state::SourceAirLoopData;
use crate::heat_balance::source_output_registration::SourceOutputDictionaryHost;
#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub(crate) enum SourceAutoSizingType { CoolingAirFlow=1, HeatingAirFlow=14, CoolingCapacity=17, HeatingCapacity=18 }
fn at(n:i32)->Result<usize,Error>{n.checked_sub(1).and_then(|v|usize::try_from(v).ok()).ok_or(Error::Unavailable("actual Sizer positive ordinal"))}
fn zone(rows:&Option<Vec<SourceZoneEquipmentSizing>>,n:i32)->Result<&SourceZoneEquipmentSizing,Error>{rows.as_ref().and_then(|v|v.get(at(n).ok()?)).ok_or(Error::Unavailable("actual Sizer copied ZoneEqSizing ordinal"))}
fn count<T>(rows:&Option<Vec<T>>)->usize {rows.as_ref().map_or(0,Vec::len)}
const COIL_TYPES:[&str;37]=[
    "Coil:Cooling:DX:SingleSpeed",
    "Coil:Heating:DX:SingleSpeed",
    "Coil:Cooling:DX:TwoSpeed",
    "CoilSystem:Cooling:DX:HeatExchangerAssisted",
    "Coil:Cooling:DX:TwoStageWithHumidityControlMode",
    "Coil:WaterHeating:AirToWaterHeatPump:Pumped",
    "Coil:WaterHeating:AirToWaterHeatPump:Wrapped",
    "Coil:Cooling:DX:MultiSpeed",
    "Coil:Heating:DX:MultiSpeed",
    "Coil:Heating:Fuel",
    "Coil:Heating:Gas:MultiStage",
    "Coil:Heating:Electric",
    "Coil:Heating:Electric:MultiStage",
    "Coil:Heating:Desuperheater",
    "Coil:Cooling:Water",
    "Coil:Cooling:Water:DetailedGeometry",
    "Coil:Heating:Water",
    "Coil:Heating:Steam",
    "CoilSystem:Cooling:Water:HeatExchangerAssisted",
    "Coil:Cooling:WaterToAirHeatPump:ParameterEstimation",
    "Coil:Heating:WaterToAirHeatPump:ParameterEstimation",
    "Coil:Cooling:WaterToAirHeatPump:EquationFit",
    "Coil:Heating:WaterToAirHeatPump:EquationFit",
    "Coil:Cooling:DX:VariableRefrigerantFlow",
    "Coil:Heating:DX:VariableRefrigerantFlow",
    "Coil:UserDefined",
    "Coil:Cooling:DX:SingleSpeed:ThermalStorage",
    "Coil:Cooling:WaterToAirHeatPump:VariableSpeedEquationFit",
    "Coil:Heating:WaterToAirHeatPump:VariableSpeedEquationFit",
    "Coil:Cooling:DX:VariableSpeed",
    "Coil:Heating:DX:VariableSpeed",
    "Coil:WaterHeating:AirToWaterHeatPump:VariableSpeed",
    "Coil:Cooling:DX:VariableRefrigerantFlow:FluidTemperatureControl",
    "Coil:Heating:DX:VariableRefrigerantFlow:FluidTemperatureControl",
    "Coil:Cooling:DX",
    "Coil:Cooling:DX:SubcoolReheat",
    "Coil:Cooling:DX:CurveFit:Speed",
];
impl SourceBaseSizer {
 fn new(kind:SourceAutoSizingType)->Self {
  let mut s=Self::default();s.sizingType=kind as i32;
  s.sizingString=match kind {SourceAutoSizingType::HeatingAirFlow=>"Heating Supply Air Flow Rate [m3/s]",SourceAutoSizingType::CoolingAirFlow=>"Cooling Supply Air Flow Rate [m3/s]",SourceAutoSizingType::HeatingCapacity=>"Heating Capacity [W]",SourceAutoSizingType::CoolingCapacity=>"Cooling Design Capacity [W]"}.into();s
 }
 fn override_sizing_string(&mut self,value:&str){self.sizingString=value.into();self.overrideSizeString=false;}
 fn initialize(&mut self,sizing:&mut SelectedSizingManager,globals:&SelectedHeatBalanceGlobals,hvac:&SelectedHvacGlobals,air_loop:&SourceAirLoopData,is_epjson:&bool,environment:&crate::weather::day::WeatherEnvironmentState,comp_type:&str,comp_name:&str,print:bool)->Result<(),Error>{
  self.initialized=true;self.compType=comp_type.into();self.compName=comp_name.into();self.isEpJSON=*is_epjson;self.printWarningFlag=print;self.callingRoutine="SizePurchasedAir: ".into();
  self.stdRhoAir=environment.std_rho_air;
  self.sysSizingRunDone=sizing.system_sizing_run_done;
  self.zoneSizingRunDone=sizing.zone_sizing_run_done;
  self.curSysNum=sizing.current_system_number;
  self.curOASysNum=sizing.autosizing.CurOASysNum;
  self.curZoneEqNum=sizing.current_zone_equipment_number;
  self.curDuctType=sizing.autosizing.CurDuctType;
  self.numPrimaryAirSys=hvac.number_of_primary_air_systems;
  self.numSysSizInput=i32::try_from(sizing.number_of_system_sizing_input).map_err(|_|Error::Unavailable("actual BaseSizer input count integer domain"))?;
  self.doSystemSizing=globals.do_system_sizing;
  self.numZoneSizingInput=i32::try_from(sizing.number_of_zone_sizing_input).map_err(|_|Error::Unavailable("actual BaseSizer input count integer domain"))?;
  self.doZoneSizing=globals.do_zone_sizing;
  self.curTermUnitSizingNum=sizing.current_terminal_unit_sizing_number;
  self.termUnitSingDuct=sizing.autosizing.TermUnitSingDuct;
  self.termUnitPIU=sizing.autosizing.TermUnitPIU;
  self.termUnitIU=sizing.autosizing.TermUnitIU;
  self.zoneEqFanCoil=sizing.autosizing.ZoneEqFanCoil;
  self.otherEqType=!(self.termUnitSingDuct || self.termUnitPIU || self.termUnitIU || self.zoneEqFanCoil);
  self.zoneEqUnitHeater=sizing.autosizing.ZoneEqUnitHeater;
  self.zoneEqUnitVent=sizing.autosizing.ZoneEqUnitVent;
  self.zoneEqVentedSlab=sizing.autosizing.ZoneEqVentedSlab;
  self.zoneSizingInput=sizing.autosizing.ZoneSizingInput.clone();
  self.unitarySysEqSizing=sizing.unitary_system_equipment_sizing.clone();
  self.oaSysEqSizing=sizing.outside_air_system_equipment_sizing.clone();
  self.outsideAirSys=air_loop.outside_air_systems.clone();
  self.termUnitSizing=sizing.terminal_unit_sizing.clone();
  self.finalZoneSizing=sizing.autosizing.FinalZoneSizing.clone();
  self.termUnitFinalZoneSizing=sizing.autosizing.TermUnitFinalZoneSizing.clone();
  self.zoneEqSizing=sizing.zone_equipment_sizing.clone();
  self.sysSizingInputData=sizing.autosizing.SysSizInput.clone();
  self.finalSysSizing=sizing.autosizing.FinalSysSizing.clone();
  self.plantSizData=sizing.autosizing.PlantSizData.clone();
  self.primaryAirSystem=air_loop.primary_air_systems.clone();
  self.airLoopControlInfo=air_loop.control_info.clone();
  self.airloopDOAS=air_loop.dedicated_outdoor_air_systems.clone();
  self.coilType_Num=0;
  for (i,name) in COIL_TYPES.iter().enumerate(){if self.compType.eq_ignore_ascii_case(name){self.coilType_Num=i32::try_from(i+1).map_err(|_|Error::Unavailable("actual Coil type integer domain"))?;self.isCoilReportObject=true;break;}}
  if ["Fan:SystemModel","Fan:ComponentModel","Fan:OnOff","Fan:ConstantVolume","Fan:VariableVolume"].iter().any(|n|self.compType.eq_ignore_ascii_case(n)){self.isFanReportObject=true;}
  self.dataEMSOverrideON=sizing.autosizing.DataEMSOverrideON;
  self.dataEMSOverride=sizing.autosizing.DataEMSOverride;
  self.dataAutosizable=sizing.autosizing.DataAutosizable;
  self.minOA=1; // Native OAControl::MinOA
  self.dataConstantUsedForSizing=sizing.autosizing.DataConstantUsedForSizing;
  self.dataFractionUsedForSizing=sizing.autosizing.DataFractionUsedForSizing;
  sizing.autosizing.DataConstantUsedForSizing=0.0;
  sizing.autosizing.DataFractionUsedForSizing=0.0;
  self.dataFanIndex=sizing.autosizing.DataFanIndex;
  self.dataFanType=sizing.autosizing.DataFanType;
  self.dataPltSizHeatNum=sizing.autosizing.DataPltSizHeatNum;
  self.dataWaterLoopNum=sizing.autosizing.DataWaterLoopNum;
  self.dataPltSizCoolNum=sizing.autosizing.DataPltSizCoolNum;
  self.dataWaterCoilSizHeatDeltaT=sizing.autosizing.DataWaterCoilSizHeatDeltaT;
  self.dataWaterCoilSizCoolDeltaT=sizing.autosizing.DataWaterCoilSizCoolDeltaT;
  self.dataCapacityUsedForSizing=sizing.autosizing.DataCapacityUsedForSizing;
  self.dataHeatSizeRatio=sizing.autosizing.DataHeatSizeRatio;
  self.dataAirFlowUsedForSizing=sizing.autosizing.DataAirFlowUsedForSizing;
  self.dataDesInletAirTemp=sizing.autosizing.DataDesInletAirTemp;
  self.dataDesAccountForFanHeat=sizing.autosizing.DataDesAccountForFanHeat;
  self.dataFanPlacement=sizing.autosizing.DataFanPlacement;
  self.dataDesInletAirHumRat=sizing.autosizing.DataDesInletAirHumRat;
  self.dataDesOutletAirHumRat=sizing.autosizing.DataDesOutletAirHumRat;
  self.dataDesOutletAirTemp=sizing.autosizing.DataDesOutletAirTemp;
  self.dataDesInletWaterTemp=sizing.autosizing.DataDesInletWaterTemp;
  self.dataFlowUsedForSizing=sizing.autosizing.DataFlowUsedForSizing;
  self.dataWaterFlowUsedForSizing=sizing.autosizing.DataWaterFlowUsedForSizing;
  self.dataSizingFraction=sizing.autosizing.DataSizingFraction;
  self.dataDXSpeedNum=sizing.autosizing.DataDXSpeedNum;
  self.dataDesicRegCoil=sizing.autosizing.DataDesicRegCoil;
  self.dataZoneUsedForSizing=sizing.autosizing.DataZoneUsedForSizing;
  self.dataDesicDehumNum=sizing.autosizing.DataDesicDehumNum;
  self.dataNomCapInpMeth=sizing.autosizing.DataNomCapInpMeth;
  self.dataCoilNum=sizing.autosizing.DataCoilNum;
  self.dataFanOp=sizing.autosizing.DataFanOp;
  self.dataDesignCoilCapacity=sizing.autosizing.DataDesignCoilCapacity;
  self.dataErrorsFound=sizing.autosizing.DataErrorsFound;
  self.dataBypassFrac=sizing.autosizing.DataBypassFrac;
  self.dataIsDXCoil=sizing.autosizing.DataIsDXCoil;
  self.dataNonZoneNonAirloopValue=sizing.autosizing.DataNonZoneNonAirloopValue;
  self.dataDXCoolsLowSpeedsAutozize=sizing.autosizing.DataDXCoolsLowSpeedsAutozize;
  // BaseSizerWithFanHeatInputs: repeat this actual read, then real conditional fan lookup.
  self.dataDesAccountForFanHeat=sizing.autosizing.DataDesAccountForFanHeat;
  if count(&self.primaryAirSystem)>0 && self.curSysNum>0 && self.curOASysNum==0 {return Err(Error::Unavailable("actual Sizer primaryAirSystem supFanType63"));}
  if self.dataFanIndex>0 && !self.isFanReportObject {return Err(Error::Unavailable("actual getFanInputsForDesHeatGain fans105"));}
  // BaseSizerWithScalableInputs follows FanHeat unconditionally.
  self.dataScalableSizingON=sizing.autosizing.DataScalableSizingON;
  self.dataScalableCapSizingON=sizing.autosizing.DataScalableCapSizingON;
  self.dataHRFlowSizingFlag=sizing.autosizing.HRFlowSizingFlag;
  self.zoneCoolingOnlyFan=sizing.zone_cooling_only_fan;
  self.zoneHeatingOnlyFan=sizing.zone_heating_only_fan;
  self.dataFracOfAutosizedCoolingAirflow=sizing.autosizing.DataFracOfAutosizedCoolingAirflow;
  self.dataFracOfAutosizedHeatingAirflow=sizing.autosizing.DataFracOfAutosizedHeatingAirflow;
  self.dataFlowPerCoolingCapacity=sizing.autosizing.DataFlowPerCoolingCapacity;
  self.dataAutosizedCoolingCapacity=sizing.autosizing.DataAutosizedCoolingCapacity;
  self.dataFlowPerHeatingCapacity=sizing.autosizing.DataFlowPerHeatingCapacity;
  self.dataAutosizedHeatingCapacity=sizing.autosizing.DataAutosizedHeatingCapacity;
  self.dataCoilSizingAirInTemp=sizing.autosizing.DataCoilSizingAirInTemp;
  self.dataCoilSizingAirInHumRat=sizing.autosizing.DataCoilSizingAirInHumRat;
  self.dataCoilSizingAirOutTemp=sizing.autosizing.DataCoilSizingAirOutTemp;
  self.dataCoilSizingAirOutHumRat=sizing.autosizing.DataCoilSizingAirOutHumRat;
  self.dataCoilSizingFanCoolLoad=sizing.autosizing.DataCoilSizingFanCoolLoad;
  self.dataCoilSizingCapFT=sizing.autosizing.DataCoilSizingCapFT;
  self.dataTotCapCurveIndex=sizing.autosizing.DataTotCapCurveIndex;
  self.dataTotCapCurveValue=sizing.autosizing.DataTotCapCurveValue;
  self.dataFracOfAutosizedCoolingCapacity=sizing.autosizing.DataFracOfAutosizedCoolingCapacity;
  self.dataFracOfAutosizedHeatingCapacity=sizing.autosizing.DataFracOfAutosizedHeatingCapacity;
  self.dataCoolCoilCap=sizing.data_cool_coil_capacity;
  self.dataCoilIsSuppHeater=sizing.autosizing.DataCoilIsSuppHeater;
  self.suppHeatCap=sizing.autosizing.SuppHeatCap;
  self.unitaryHeatCap=sizing.autosizing.UnitaryHeatCap;
  self.dataCoolCoilType=sizing.autosizing.DataCoolCoilType;
  self.dataCoolCoilIndex=sizing.autosizing.DataCoolCoilIndex;
  self.zoneHVACSizing=sizing.autosizing.ZoneHVACSizing.clone();
  if self.isCoilReportObject && self.curSysNum>0 && count(&self.primaryAirSystem)>0 && self.curSysNum<=hvac.number_of_primary_air_systems {return Err(Error::Unavailable("actual ScalableSizer primaryAirSystem supFanNum103"));}
  if self.curZoneEqNum!=0 {
   if self.zoneHVACSizingIndex>0 {return Err(Error::Unavailable("actual ScalableSizer ZoneHVACSizing CoolingSAFMethod122"));}
   else {
    if count(&self.zoneEqSizing)>0 && zone(&self.zoneEqSizing,self.curZoneEqNum)?.sizing__method.as_ref().is_some_and(|r|!r.is_empty()) {
     self.zoneAirFlowSizMethod=*zone(&self.zoneEqSizing,self.curZoneEqNum)?.sizing__method.as_ref().and_then(|r|r.get(at(self.sizingType).ok()?)).ok_or(Error::Unavailable("actual ScalableSizer SizingMethod ordinal141"))?;
    }else{self.zoneAirFlowSizMethod=0;}
    // Native has no write for method0, SupplyAirFlowRate or None in this tail.
   }
  }
  Ok(())
 }
 fn pre_size(&mut self,original:f64)->Result<(),Error>{
  if self.sizingType == -1 {self.lastErrorMessages.push_str("Sizing Library Base Class: preSize, SizingType not defined.\n");return Err(Error::Unavailable("actual BaseSizer preSize invalid type severe197"));}
  self.originalValue=original;self.autoCalculate=false;self.errorType=0;self.initialized=false;self.hardSizeNoDesignRun=!(self.sysSizingRunDone||self.zoneSizingRunDone);self.sizingDesRunThisZone=false;self.sizingDesRunThisAirSys=false;
  if self.dataFractionUsedForSizing==0.0 && self.dataConstantUsedForSizing>0.0 {
   self.errorType=1;self.autoCalculate=true;self.hardSizeNoDesignRun=false;
   if self.wasAutoSized {self.autoSizedValue=0.0;return Err(Error::Unavailable("actual BaseSizer preSize autocalculate severe220"));}
  }else if self.dataFractionUsedForSizing>0.0 {self.autoCalculate=true;self.hardSizeNoDesignRun=false;}
  else if self.sizingType==25 {self.autoCalculate=true;if self.originalValue == -99999.0 && !self.dataEMSOverrideON {self.errorType=1;return Err(Error::Unavailable("actual BaseSizer preSize autocalculate severe233"));}}
  if self.curSysNum>0 && self.curSysNum<=self.numPrimaryAirSys {
   if self.sysSizingRunDone {if let Some(rows)=&self.sysSizingInputData {for row in rows {match *row {}}}}
   if self.unitarySysEqSizing.is_some() {let row=zone(&self.unitarySysEqSizing,self.curSysNum)?;self.airLoopSysFlag=row.cooling__capacity||row.heating__capacity;}
   if self.curOASysNum>0 {let row=zone(&self.oaSysEqSizing,self.curOASysNum)?;self.oaSysFlag=row.cooling__capacity||row.heating__capacity;}
  }
  if self.curZoneEqNum>0 {
   if self.zoneEqSizing.is_some(){self.sizingDesValueFromParent=zone(&self.zoneEqSizing,self.curZoneEqNum)?.design__size__from__parent;}
   if self.zoneSizingRunDone {if let Some(rows)=&self.zoneSizingInput {for row in rows {match *row {}}}}
   self.hardSizeNoDesignRun=false;
  }
  if self.originalValue == -99999.0 {
   self.wasAutoSized=true;self.hardSizeNoDesignRun=false;
   if !self.sizingDesRunThisAirSys && self.curSysNum>0 && !self.autoCalculate && !self.sysSizingRunDone {return Err(Error::Unavailable("actual BaseSizer autosizing system severe269"));}
   if !self.sizingDesRunThisZone && self.curZoneEqNum>0 && !self.sizingDesValueFromParent && !self.autoCalculate && !self.zoneSizingRunDone {return Err(Error::Unavailable("actual BaseSizer autosizing zone severe284"));}
  }else{self.wasAutoSized=false;}
  Ok(())
 }
 fn emit<D:SourceOutputDictionaryHost>(&self,o:&mut SourceSizerReportingOwners<'_,D>,prefix:&str,value:f64)->Result<(),Error>{
  report_sizer_output(o,&self.compType,&self.compName,&(prefix.to_owned()+&self.sizingStringScalable+&self.sizingString),value,None,None)
 }
 fn apply_two_stage_bypass<D:SourceOutputDictionaryHost>(&mut self,o:&mut SourceSizerReportingOwners<'_,D>,prefix:&str)->Result<(),Error>{
  if self.compType.eq_ignore_ascii_case("COIL:COOLING:DX:TWOSTAGEWITHHUMIDITYCONTROLMODE") {
   self.autoSizedValue*=1.0-self.dataBypassFrac;
   report_sizer_output(o,&self.compType,&self.compName,&(prefix.to_owned()+&self.sizingStringScalable+&self.sizingString+" ( non-bypassed )"),self.autoSizedValue,None,None)?;
  }Ok(())
 }
 fn select_output<D:SourceOutputDictionaryHost>(&mut self,two_stage:bool,sizing:&SelectedSizingManager,globals:&SelectedHeatBalanceGlobals,o:&mut SourceSizerReportingOwners<'_,D>,errors:&mut bool)->Result<(),Error>{
  if self.printWarningFlag {
   if self.dataEMSOverrideON {
    if !two_stage{self.autoSizedValue=self.dataEMSOverride;}
    self.emit(o,"User-Specified ",self.autoSizedValue)?;
    if two_stage{self.apply_two_stage_bypass(o,"User-Specified ")?;}
   }else if !two_stage && self.hardSizeNoDesignRun && !self.wasAutoSized && self.compType.eq_ignore_ascii_case("Fan:ZoneExhaust") {self.autoSizedValue=self.originalValue;}
   else if !two_stage && self.wasAutoSized && self.dataFractionUsedForSizing>0.0 && self.dataConstantUsedForSizing>0.0 {
    self.autoSizedValue=self.dataFractionUsedForSizing*self.dataConstantUsedForSizing;self.emit(o,"Design Size ",self.autoSizedValue)?;
   }else if !self.wasAutoSized && (self.autoSizedValue==self.originalValue||self.autoSizedValue==0.0) {
    self.autoSizedValue=self.originalValue;
    if two_stage || self.dataAutosizable || (!self.sizingDesRunThisZone && self.compType.eq_ignore_ascii_case("Fan:ZoneExhaust")){self.emit(o,"User-Specified ",self.autoSizedValue)?;}
    if two_stage{self.apply_two_stage_bypass(o,"User-Specified ")?;}
   }else if !self.wasAutoSized && self.autoSizedValue>=0.0 && self.originalValue==0.0 {
    self.autoSizedValue=self.originalValue;self.emit(o,"User-Specified ",self.autoSizedValue)?;if two_stage{self.apply_two_stage_bypass(o,"User-Specified ")?;}
   }else if self.wasAutoSized && self.autoSizedValue>=0.0 && self.originalValue<=0.0 {
    if self.dataScalableSizingON && self.zoneAirFlowSizMethod>0 {self.emit(o,"User-Specified ",self.autoSizedValue)?;}else{report_sizer_output(o,&self.compType,&self.compName,&("Design Size ".to_owned()+&self.sizingString),self.autoSizedValue,None,None)?;}
    if two_stage{self.apply_two_stage_bypass(o,"Design Size ")?;}
   }else if self.autoSizedValue>=0.0 && self.originalValue>0.0 {
    if ((self.autoSizedValue-self.originalValue).abs()/self.originalValue)>sizing.autosizing.AutoVsHardSizingThreshold {
     if two_stage||self.dataAutosizable {
      report_sizer_output(o,&self.compType,&self.compName,&("Design Size ".to_owned()+&self.sizingString),self.autoSizedValue,Some(&("User-Specified ".to_owned()+&self.sizingStringScalable+&self.sizingString)),Some(self.originalValue))?;
     }
     if two_stage && self.compType.eq_ignore_ascii_case("COIL:COOLING:DX:TWOSTAGEWITHHUMIDITYCONTROLMODE") {self.autoSizedValue*=1.0-self.dataBypassFrac;self.originalValue*=1.0-self.dataBypassFrac;report_sizer_output(o,&self.compType,&self.compName,&("Design Size ".to_owned()+&self.sizingString+" ( non-bypassed )"),self.autoSizedValue,Some(&("User-Specified ".to_owned()+&self.sizingStringScalable+&self.sizingString+" ( non-bypassed )")),Some(self.originalValue))?;}
    }else{
     if two_stage && self.compType.eq_ignore_ascii_case("COIL:COOLING:DX:TWOSTAGEWITHHUMIDITYCONTROLMODE"){self.autoSizedValue/=1.0-self.dataBypassFrac;}
     if two_stage||self.dataAutosizable {self.emit(o,"User-Specified ",self.originalValue)?;}
     if two_stage{self.apply_two_stage_bypass(o,"User-Specified ")?;}
    }
    if globals.display_extra_warnings && (two_stage||self.dataAutosizable) {
     if ((self.autoSizedValue-self.originalValue).abs()/self.originalValue)>sizing.autosizing.AutoVsHardSizingThreshold {return Err(Error::Unavailable("actual selectSizerOutput sizing difference ShowMessage426/558"));}
    }
    if !self.wasAutoSized {self.autoSizedValue=self.originalValue;}
   }else if self.wasAutoSized && self.autoSizedValue != -99999.0 {self.emit(o,"Design Size ",self.autoSizedValue)?;}
   else {return Err(Error::Unavailable("actual selectSizerOutput incomplete sizing severe451/581"));}
  }else if !self.wasAutoSized && !self.autoCalculate {self.autoSizedValue=self.originalValue;}
  self.overrideSizeString=true;
  if self.errorType!=0 {return Err(Error::Unavailable("actual selectSizerOutput errorType severe465/595"));}
  Ok(())
 }
 fn size<D:SourceOutputDictionaryHost>(&mut self,kind:SourceAutoSizingType,original:f64,sizing:&SelectedSizingManager,globals:&SelectedHeatBalanceGlobals,o:&mut SourceSizerReportingOwners<'_,D>,errors:&mut bool)->Result<f64,Error>{
  if !self.initialized { *errors=true;self.errorType=2;self.autoSizedValue=0.0;return Err(Error::Unavailable("actual BaseSizer checkInitialized severe651")); }
  self.pre_size(original)?;
  // These are real derived-method locals. HeatingCapacity invokes the normal cached Cp helper here even for a hard value.
  let mut DesVolFlow=0.0;let mut CoilInTemp=-999.0;let mut CoilInHumRat=-999.0;let mut CoilOutTemp=-999.0;let mut CoilOutHumRat=-999.0;
  let mut DXFlowPerCapMinRatio=1.0;let mut DXFlowPerCapMaxRatio=1.0;let mut NominalCapacityDes=0.0;let mut DesMassFlow=0.0;let mut DesCoilLoad=0.0;let mut OutAirFrac=0.0;
  let mut FanCoolLoad=0.0;let mut TotCapTempModFac=1.0;let mut coolingFlow=false;let mut heatingFlow=false;
  if kind==SourceAutoSizingType::HeatingCapacity {let CpAirStd=crate::psychrometrics::energyplus_psy_cp_air_fn_w(0.0);}
  let capacity=matches!(kind,SourceAutoSizingType::HeatingCapacity|SourceAutoSizingType::CoolingCapacity);
  if self.dataEMSOverrideON {self.autoSizedValue=self.dataEMSOverride;}
  else if (if capacity{self.dataConstantUsedForSizing>=0.0}else{self.dataConstantUsedForSizing>0.0}) && self.dataFractionUsedForSizing>0.0 {self.autoSizedValue=self.dataConstantUsedForSizing*self.dataFractionUsedForSizing;}
  else if self.curZoneEqNum>0 {
   if !self.wasAutoSized && !self.sizingDesRunThisZone {
    self.autoSizedValue=original;
    if kind==SourceAutoSizingType::CoolingAirFlow && self.compType.eq_ignore_ascii_case("Coil:Cooling:DX:TwoStageWithHumidityControlMode"){self.autoSizedValue/=1.0-self.dataBypassFrac;self.originalValue/=1.0-self.dataBypassFrac;}
   }else if zone(&self.zoneEqSizing,self.curZoneEqNum)?.design__size__from__parent {
    let row=zone(&self.zoneEqSizing,self.curZoneEqNum)?;
    self.autoSizedValue=match kind {SourceAutoSizingType::HeatingCapacity=>row.des__heating__load,SourceAutoSizingType::CoolingCapacity=>row.des__cooling__load,_=>row.air__vol__flow};
   }else{return Err(Error::Unavailable("actual Sizer positive zone design SizingMethod/capacity child"));}
  }else{return Err(Error::Unavailable("actual Sizer non-zone system/design child"));}
  // Native airflow metadata tails are inside its non-EMS/non-constant else; capacity tails are unconditional.
  let airflow_metadata=!self.dataEMSOverrideON && !(self.dataConstantUsedForSizing>0.0 && self.dataFractionUsedForSizing>0.0);
  if (capacity||airflow_metadata) && self.overrideSizeString && self.isEpJSON {
   self.sizingString=match kind {SourceAutoSizingType::HeatingAirFlow=>"heating_supply_air_flow_rate [m3/s]",SourceAutoSizingType::CoolingAirFlow=>"cooling_supply_air_flow_rate [m3/s]",SourceAutoSizingType::HeatingCapacity=>"nominal_capacity [W]",SourceAutoSizingType::CoolingCapacity=>"cooling_design_capacity [W]"}.into();
  }
  if capacity && self.dataScalableCapSizingON {
   let number=if kind==SourceAutoSizingType::HeatingCapacity{18}else{17};
   let method=*zone(&self.zoneEqSizing,self.curZoneEqNum)?.sizing__method.as_ref().and_then(|r|r.get(at(number).ok()?)).ok_or(Error::Unavailable("actual capacity Sizer scalable SizingMethod ordinal"))?;
   if method==10 {self.sizingStringScalable="(scaled by capacity / area) ".into();}
   else if method==11||method==12 {self.sizingStringScalable="(scaled by fractional multiplier) ".into();}
  }
  if !capacity && airflow_metadata && self.dataScalableSizingON {
   self.sizingStringScalable=match self.zoneAirFlowSizMethod {1|2=>"(scaled by flow / zone) ",3=>"(scaled by flow / area) ",4|5=>"(scaled by fractional multiplier) ",6|7=>"(scaled by flow / capacity) ",_=>""}.into();
  }
  if kind==SourceAutoSizingType::CoolingAirFlow && self.dataDXCoolsLowSpeedsAutozize {self.autoSizedValue*=self.dataFractionUsedForSizing;}
  self.select_output(kind==SourceAutoSizingType::CoolingAirFlow,sizing,globals,o,errors)?;
  if self.isCoilReportObject {return Err(Error::Unavailable("actual Sizer CoilSelection report child after select output"));}
  if !capacity && self.isFanReportObject {return Err(Error::Unavailable("actual airflow Sizer predefined fan peak child"));}
  Ok(self.autoSizedValue)
 }
}
#[allow(clippy::too_many_arguments)]
pub(crate) fn size_source_component<D:SourceOutputDictionaryHost>(kind:SourceAutoSizingType,description:&str,value:f64,print:bool,comp_type:&str,comp_name:&str,sizing:&mut SelectedSizingManager,globals:&SelectedHeatBalanceGlobals,hvac:&SelectedHvacGlobals,air_loop:&SourceAirLoopData,is_epjson:&bool,report:&mut SourceSizerReportingOwners<'_,D>,errors:&mut bool)->Result<f64,Error>{
 let mut s=SourceBaseSizer::new(kind);
 s.override_sizing_string(description);
 s.initialize(sizing,globals,hvac,air_loop,is_epjson,&*report.environment,comp_type,comp_name,print)?;
 s.size(kind,value,sizing,globals,report,errors)
}

#[cfg(all(test, feature = "native-controlled-air-cohort"))]
mod native_cohort;
