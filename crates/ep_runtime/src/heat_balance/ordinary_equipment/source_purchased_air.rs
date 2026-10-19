//! Genuine PurchasedAir input, calculation/update/report and live report lookups.
pub(crate) mod owners;
mod getter;
mod schema;
mod registration;
mod initialization;
mod calculation;
pub(super) use calculation::SourcePurchasedAirCalcOwners;
pub(super) use initialization::SourcePurchasedAirInitOwners;
use owners::*;
use super::{OrdinaryEquipment, source_nodes::SourceNodeFluidType};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_input_processor::{SourceInputProcessorShortCut, SourceInputFamily};
use crate::heat_balance::selected_schedule_manager::{SelectedScheduleManager, SourceScheduleIndex};
use crate::heat_balance::source_output_registration::{SourceOutputFileHost, SourceOutputDictionaryHost, SourcePurchasedAirReportingOutputContext};
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::startup_integrity::{SourceBranchNodeConnections, SourceNodeObjectType, SourceNodeConnectionType, SourceCompFluidStream};

fn at(number:i32)->Result<usize,Error>{
    number.checked_sub(1).and_then(|n|usize::try_from(n).ok())
        .ok_or(Error::Unavailable("actual PurchasedAir positive ordinal"))
}
fn limit(value:&str)->SourcePurchasedAirLimit {
    match value {"NOLIMIT"=>SourcePurchasedAirLimit::None,"LIMITFLOWRATE"=>SourcePurchasedAirLimit::FlowRate,
        "LIMITCAPACITY"=>SourcePurchasedAirLimit::Capacity,"LIMITFLOWRATEANDCAPACITY"=>SourcePurchasedAirLimit::FlowRateAndCapacity,
        _=>SourcePurchasedAirLimit::Invalid}
}
fn humidity(value:&str)->SourcePurchasedAirHumidityControl {
    match value {"NONE"=>SourcePurchasedAirHumidityControl::None,"CONSTANTSENSIBLEHEATRATIO"=>SourcePurchasedAirHumidityControl::ConstantSensibleHeatRatio,
        "HUMIDISTAT"=>SourcePurchasedAirHumidityControl::Humidistat,"CONSTANTSUPPLYHUMIDITYRATIO"=>SourcePurchasedAirHumidityControl::ConstantSupplyHumidityRatio,
        _=>SourcePurchasedAirHumidityControl::Invalid}
}
fn fuel(value:&str)->SourcePurchasedAirFuel {
    use SourcePurchasedAirFuel as F;
    match value {"ELECTRICITY"=>F::Electricity,"NATURALGAS"=>F::NaturalGas,"GASOLINE"=>F::Gasoline,"DIESEL"=>F::Diesel,
        "COAL"=>F::Coal,"PROPANE"=>F::Propane,"FUELOILNO1"=>F::FuelOilNo1,"FUELOILNO2"=>F::FuelOilNo2,
        "OTHERFUEL1"=>F::OtherFuel1,"OTHERFUEL2"=>F::OtherFuel2,"DISTRICTCOOLING"=>F::DistrictCooling,
        "DISTRICTHEATINGWATER"=>F::DistrictHeatingWater,"DISTRICTHEATINGSTEAM"=>F::DistrictHeatingSteam,
        "WATER"=>F::Water,"NONE"=>F::None,_=>F::Invalid}
}
fn schedule(schedules:&mut SelectedScheduleManager,value:&str)->Result<Option<SourceScheduleIndex>,Error>{
    if value.is_empty(){Ok(Some(schedules.always_on().map_err(|_|Error::Unavailable("actual PurchasedAir GetScheduleAlwaysOn pointer"))?))}
    else {schedules.get_source_schedule(value)}
}
impl OrdinaryEquipment {
    fn purchased_unit(&self,number:i32)->Result<&SourceZonePurchasedAir,Error>{
        self.purchased_air.units.as_ref().and_then(|rows|rows.get(at(number).ok()?))
            .ok_or(Error::Unavailable("actual retained PurchAir unit ordinal"))
    }
    fn purchased_unit_mut(&mut self,number:i32)->Result<&mut SourceZonePurchasedAir,Error>{
        self.purchased_air.units.as_mut().and_then(|rows|rows.get_mut(at(number).ok()?))
            .ok_or(Error::Unavailable("actual retained mutable PurchAir unit ordinal"))
    }
    fn purchased_equipment_index(&mut self,zone:i32,pointer:i32)->Result<&mut i32,Error>{
        self.equipment_lists.as_mut().and_then(|rows|rows.get_mut(at(zone).ok()?))
            .and_then(|list|list.equipment_indices.as_mut()).and_then(|rows|rows.get_mut(at(pointer).ok()?))
            .ok_or(Error::Unavailable("actual PurchasedAir same passed EquipIndex cell"))
    }
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn simulate_source_purchased_air<H:SourceOutputFileHost,D:SourceOutputDictionaryHost>(
        &mut self,name:&str,current_equipment_zone:i32,equipment_pointer:i32,controlled_zone:i32,
        shortcut:&mut SourceInputProcessorShortCut,source_is_epjson:bool,schedules:&mut SelectedScheduleManager,
        number_zones:i32,branch:&mut SourceBranchNodeConnections,any_ems:bool,
        output:&mut SourcePurchasedAirReportingOutputContext<'_,H,D>,
        initialization:&mut SourcePurchasedAirInitOwners<'_>,
        calculation:&mut SourcePurchasedAirCalcOwners<'_, '_>,
        first_hvac_iteration:bool,system_output:&mut f64,moisture_output:&mut f64,
    )->Result<(),Error>{
        if self.purchased_air.get_purchased_air_input {
            self.get_source_purchased_air(shortcut,source_is_epjson,schedules,number_zones,branch,any_ems,calculation.backend,output)?;
            self.purchased_air.get_purchased_air_input=false;
        }
        // Native passes the actual equipment-list cell by reference. This
        // ordinal transport reborrows that same cell at each read/write, so
        // GetInput can simultaneously use the sole Config/Node owners.
        let component_index=*self.purchased_equipment_index(current_equipment_zone,equipment_pointer)?;
        let unit_number;
        if component_index==0 {
            let found=self.purchased_air.units.as_ref()
                .ok_or(Error::Unavailable("actual SimPurchasedAir FindItemInList unit allocation175"))?
                .iter().position(|row|row.name==name);
            unit_number=match found {
                Some(index)=>i32::try_from(index+1).map_err(|_|Error::Unavailable("actual SimPurchasedAir unit index integer domain"))?,
                None=>return Err(Error::Unavailable("actual SimPurchasedAir unit-not-found fatal177")),
            };
            *self.purchased_equipment_index(current_equipment_zone,equipment_pointer)?=unit_number;
        }else{
            unit_number=component_index;
            if unit_number>self.purchased_air.number_purchased_air || unit_number<1 {
                return Err(Error::Unavailable("actual SimPurchasedAir invalid CompIndex fatal184"));
            }
            let check=*self.purchased_air.check_equipment_name.as_ref().and_then(|rows|rows.get(at(unit_number).ok()?))
                .ok_or(Error::Unavailable("actual SimPurchasedAir CheckEquipName189"))?;
            if check {
                if name!=self.purchased_unit(unit_number)?.name {
                    return Err(Error::Unavailable("actual SimPurchasedAir cached unit-name fatal191"));
                }
                *self.purchased_air.check_equipment_name.as_mut().and_then(|rows|rows.get_mut(at(unit_number).ok()?))
                    .ok_or(Error::Unavailable("actual SimPurchasedAir CheckEquipName197"))?=false;
            }
        }
        self.initialize_source_purchased_air(unit_number, controlled_zone, schedules, initialization)?;
        self.calculate_source_purchased_air(unit_number,controlled_zone,system_output,moisture_output,schedules,calculation)?;
        self.update_source_purchased_air(unit_number,first_hvac_iteration)?;
        self.report_source_purchased_air(unit_number,schedules,calculation)?;
        Ok(())
    }
    #[allow(clippy::too_many_arguments)]
    fn get_source_purchased_air<H:SourceOutputFileHost,D:SourceOutputDictionaryHost>(
        &mut self,shortcut:&mut SourceInputProcessorShortCut,source_is_epjson:bool,schedules:&mut SelectedScheduleManager,
        number_zones:i32,branch:&mut SourceBranchNodeConnections,any_ems:bool,
        backend:&mut SourceErrorBackend<'_>,output:&mut SourcePurchasedAirReportingOutputContext<'_,H,D>,
    )->Result<(),Error>{
        let mut errors_found=false;
        shortcut.current_module_object=getter::FAMILY.into();
        let count=output.input.get_num_objects_found(&shortcut.current_module_object);
        self.purchased_air.number_purchased_air=i32::try_from(count)
            .map_err(|_|Error::Unavailable("actual GetPurchasedAir NumPurchAir integer domain240"))?;
        self.purchased_air.units=Some(vec![SourceZonePurchasedAir::default();count]);
        // All cells are overwritten true by the genuine unconditional whole
        // assignment before any read. No Native primitive allocation value is
        // certified by this Rust-required initial storage value.
        self.purchased_air.check_equipment_name=Some(vec![false;count]);
        self.purchased_air.numeric_fields=Some(vec![SourcePurchasedAirNumericFieldData::default();count]);
        self.purchased_air.check_equipment_name.as_mut().ok_or(Error::Unavailable("actual CheckEquipName allocation243"))?.fill(true);
        // Native repeats both allocations and whole true assignment245-248.
        // Their cold class rows and written true flags are preserved in order.
        self.purchased_air.check_equipment_name=Some(vec![false;count]);
        self.purchased_air.numeric_fields=Some(vec![SourcePurchasedAirNumericFieldData::default();count]);
        self.purchased_air.check_equipment_name.as_mut().ok_or(Error::Unavailable("actual repeated CheckEquipName allocation246"))?.fill(true);
        let family_present=output.input.find_exact_family(getter::FAMILY).is_some();
        if family_present {
            self.nodes.init_unique_check_source(getter::FAMILY,output.input)
                .map_err(|_|Error::Unavailable("actual PurchasedAir InitUniqueNodeCheck257"))?;
            // Exact installed/pinned schema is compiled into schema.rs; the
            // actual outer JSON object-map traversal is lexicographic.
            let mut objects=match output.input.find_exact_family(getter::FAMILY) {
                Some(SourceInputFamily::Null)=>Vec::new(),
                Some(SourceInputFamily::Objects(rows)) if rows.is_empty()=>Vec::new(),
                Some(SourceInputFamily::Objects(_))=>output.input.source_raw_object_names(getter::FAMILY)
                    .map_err(|_|Error::Unavailable("actual PurchasedAir retained JSON instance map260"))?.to_vec(),
                None=>return Err(Error::Unavailable("actual PurchasedAir retained family find250")),
            };
            objects.sort();
            for (index,object) in objects.iter().enumerate() {
                let number=i32::try_from(index+1).map_err(|_|Error::Unavailable("actual PurchasedAir loop ordinal262"))?;
                self.purchased_unit_mut(number)?.c_object_name=shortcut.current_module_object.clone();
                let (_,num_numbers)=getter::get(output.input,shortcut,object,index+1,source_is_epjson)?;
                let num_numbers=usize::try_from(num_numbers).map_err(|_|Error::Unavailable("actual PurchAirNumericFields NumNums281"))?;
                let names=&mut self.purchased_air.numeric_fields.as_mut().and_then(|rows|rows.get_mut(index))
                    .ok_or(Error::Unavailable("actual PurchAirNumericFields unit281"))?.field_names;
                *names=Some(vec![String::new();num_numbers]);
                names.as_mut().ok_or(Error::Unavailable("actual PurchAir FieldNames allocation281"))?.fill(String::new());
                // Array1D copy assignment redimensions to the actual shared
                // cNumericFieldNames extent; Native283 copies the WHOLE array.
                *names=Some(shortcut.numeric_field_names.clone());
                if !object.is_ascii(){return Err(Error::Unavailable("actual PurchasedAir object-name makeUPPER domain284"));}
                self.purchased_unit_mut(number)?.name=object.to_ascii_uppercase();
                let name=self.purchased_unit(number)?.name.clone();
                let value=getter::alpha(output.input,object,"availability_schedule_name")?;
                self.purchased_unit_mut(number)?.avail_sched=schedule(schedules,&value)?;
                if self.purchased_unit(number)?.avail_sched.is_none(){return Err(Error::Unavailable("actual PurchasedAir availability schedule severe292"));}
                let supply=getter::alpha(output.input,object,"zone_supply_air_node_name")?;
                let node=self.nodes.get_source_only_single_node(output.input,branch,&supply,&mut errors_found,
                    SourceNodeObjectType::ZoneHvacIdealLoadsAirSystem,&name,SourceNodeFluidType::Air,
                    SourceNodeConnectionType::Outlet,SourceCompFluidStream::Primary,false,"")
                    .map_err(|_|Error::Unavailable("actual PurchasedAir supply GetOnlySingleNode299"))?;
                self.purchased_unit_mut(number)?.zone_supply_air_node_num=i32::try_from(node).map_err(|_|Error::Unavailable("actual PurchasedAir supply node int domain"))?;
                let mut unique_error=false;
                self.nodes.check_source_unique_node_names("Zone Supply Air Node Name",&mut unique_error,&supply,&name)
                    .map_err(|_|Error::Unavailable("actual PurchasedAir supply CheckUniqueNodeNames310"))?;
                if unique_error{errors_found=true;}
                let exhaust=getter::alpha(output.input,object,"zone_exhaust_air_node_name")?;
                let inlet=getter::alpha(output.input,object,"system_inlet_air_node_name")?;
                if !exhaust.is_empty(){
                    let connection=if inlet.is_empty(){SourceNodeConnectionType::Inlet}else{SourceNodeConnectionType::Outlet};
                    let node=self.nodes.get_source_only_single_node(output.input,branch,&exhaust,&mut errors_found,
                        SourceNodeObjectType::ZoneHvacIdealLoadsAirSystem,&name,SourceNodeFluidType::Air,connection,SourceCompFluidStream::Primary,false,"")
                        .map_err(|_|Error::Unavailable("actual PurchasedAir exhaust GetOnlySingleNode322"))?;
                    self.purchased_unit_mut(number)?.zone_exhaust_air_node_num=i32::try_from(node).map_err(|_|Error::Unavailable("actual PurchasedAir exhaust node int domain"))?;
                    unique_error=false;
                    self.nodes.check_source_unique_node_names("Zone Exhaust Air Node Name",&mut unique_error,&exhaust,&name)
                        .map_err(|_|Error::Unavailable("actual PurchasedAir exhaust CheckUniqueNodeNames345"))?;
                    if unique_error{errors_found=true;}
                }
                if !inlet.is_empty(){
                    let node=self.nodes.get_source_only_single_node(output.input,branch,&inlet,&mut errors_found,
                        SourceNodeObjectType::ZoneHvacIdealLoadsAirSystem,&name,SourceNodeFluidType::Air,
                        SourceNodeConnectionType::Inlet,SourceCompFluidStream::Primary,false,"")
                        .map_err(|_|Error::Unavailable("actual PurchasedAir plenum GetOnlySingleNode353"))?;
                    self.purchased_unit_mut(number)?.plenum_exhaust_air_node_num=i32::try_from(node).map_err(|_|Error::Unavailable("actual PurchasedAir plenum node int domain"))?;
                }
                self.purchased_unit_mut(number)?.max_heat_supp_air_temp=getter::real(output.input,object,"maximum_heating_supply_air_temperature")?;
                self.purchased_unit_mut(number)?.min_cool_supp_air_temp=getter::real(output.input,object,"minimum_cooling_supply_air_temperature")?;
                self.purchased_unit_mut(number)?.max_heat_supp_air_hum_rat=getter::real(output.input,object,"maximum_heating_supply_air_humidity_ratio")?;
                self.purchased_unit_mut(number)?.min_cool_supp_air_hum_rat=getter::real(output.input,object,"minimum_cooling_supply_air_humidity_ratio")?;
                let value=getter::alpha(output.input,object,"heating_limit")?;
                self.purchased_unit_mut(number)?.heating_limit=limit(&value);
                self.purchased_unit_mut(number)?.max_heat_vol_flow_rate=getter::real(output.input,object,"maximum_heating_air_flow_rate")?;
                self.purchased_unit_mut(number)?.max_heat_sens_cap=getter::real(output.input,object,"maximum_sensible_heating_capacity")?;
                let value=getter::alpha(output.input,object,"cooling_limit")?;
                self.purchased_unit_mut(number)?.cooling_limit=limit(&value);
                self.purchased_unit_mut(number)?.max_cool_vol_flow_rate=getter::real(output.input,object,"maximum_cooling_air_flow_rate")?;
                self.purchased_unit_mut(number)?.max_cool_tot_cap=getter::real(output.input,object,"maximum_total_cooling_capacity")?;
                let value=getter::alpha(output.input,object,"heating_availability_schedule_name")?;
                self.purchased_unit_mut(number)?.heat_avail_sched=schedule(schedules,&value)?;
                if self.purchased_unit(number)?.heat_avail_sched.is_none(){return Err(Error::Unavailable("actual PurchasedAir heating schedule severe380"));}
                let value=getter::alpha(output.input,object,"cooling_availability_schedule_name")?;
                self.purchased_unit_mut(number)?.cool_avail_sched=schedule(schedules,&value)?;
                if self.purchased_unit(number)?.cool_avail_sched.is_none(){return Err(Error::Unavailable("actual PurchasedAir cooling schedule severe389"));}
                let value=getter::alpha(output.input,object,"dehumidification_control_type")?;
                self.purchased_unit_mut(number)?.dehumid_ctrl_type=humidity(&value);
                self.purchased_unit_mut(number)?.cool_shr=getter::real(output.input,object,"cooling_sensible_heat_ratio")?;
                let value=getter::alpha(output.input,object,"humidification_control_type")?;
                self.purchased_unit_mut(number)?.humid_ctrl_type=humidity(&value);
                let value=getter::alpha(output.input,object,"design_specification_outdoor_air_object_name")?;
                if !value.is_empty(){return Err(Error::Unavailable("actual PurchasedAir OARequirements FindItemInList405"));}
                if self.purchased_unit(number)?.outdoor_air {
                    return Err(Error::Unavailable("actual PurchasedAir outdoor-air input path415"));
                }else{
                    self.purchased_unit_mut(number)?.dcv_type=SourcePurchasedAirDcv::None;
                    self.purchased_unit_mut(number)?.economizer_type=SourcePurchasedAirEconomizer::NoEconomizer;
                    self.purchased_unit_mut(number)?.ht_rec_type=SourcePurchasedAirHeatRecovery::None;
                }
                self.purchased_unit_mut(number)?.ht_rec_sen_eff=getter::real(output.input,object,"sensible_heat_recovery_effectiveness")?;
                self.purchased_unit_mut(number)?.ht_rec_lat_eff=getter::real(output.input,object,"latent_heat_recovery_effectiveness")?;
                for zone in 1..=number_zones {
                    let config=self.zone_config.as_ref().and_then(|rows|rows.get(at(zone).ok()?))
                        .ok_or(Error::Unavailable("actual PurchasedAir ZoneEquipConfig473"))?;
                    if !config.is_controlled {continue;}
                    for i in 0..config.inlet_nodes {
                        let node=*config.inlet_node_numbers.as_ref().and_then(|rows|rows.get(i))
                            .ok_or(Error::Unavailable("actual PurchasedAir Config.InletNode477"))?;
                        if self.purchased_unit(number)?.zone_supply_air_node_num==i32::try_from(node)
                            .map_err(|_|Error::Unavailable("actual PurchasedAir Config node int domain"))? {
                            self.purchased_unit_mut(number)?.zone_ptr=zone;
                            break;
                        }
                    }
                }
                self.purchased_unit_mut(number)?.hvac_sizing_index=0;
                let value=getter::alpha(output.input,object,"design_specification_zonehvac_sizing_object_name")?;
                if !value.is_empty(){return Err(Error::Unavailable("actual PurchasedAir ZoneHVACSizing FindItemInList490"));}
                let value=getter::alpha(output.input,object,"heating_fuel_efficiency_schedule_name")?;
                self.purchased_unit_mut(number)?.heat_fuel_eff_sched=schedule(schedules,&value)?;
                if self.purchased_unit(number)?.heat_fuel_eff_sched.is_none(){return Err(Error::Unavailable("actual PurchasedAir heating fuel schedule severe503"));}
                let value=getter::alpha(output.input,object,"heating_fuel_type")?;
                self.purchased_unit_mut(number)?.heating_fuel_type=fuel(&value);
                let value=getter::alpha(output.input,object,"cooling_fuel_efficiency_schedule_name")?;
                self.purchased_unit_mut(number)?.cool_fuel_eff_sched=schedule(schedules,&value)?;
                if self.purchased_unit(number)?.cool_fuel_eff_sched.is_none(){return Err(Error::Unavailable("actual PurchasedAir cooling fuel schedule severe515"));}
                let value=getter::alpha(output.input,object,"cooling_fuel_type")?;
                self.purchased_unit_mut(number)?.cooling_fuel_type=fuel(&value);
            }
            self.nodes.end_unique_check(getter::FAMILY)
                .map_err(|_|Error::Unavailable("actual PurchasedAir EndUniqueNodeCheck520"))?;
        }
        for number in 1..=self.purchased_air.number_purchased_air {
            registration::register_unit(self,number,backend,output)?;
            if any_ems{return Err(Error::Unavailable("actual PurchasedAir SetupEMSActuator1018"));}
        }
        if errors_found{return Err(Error::Unavailable("actual GetPurchasedAir ErrorsFound fatal1049"));}
        Ok(())
    }
}

impl OrdinaryEquipment {
    /// Every Native report getter first tests the same real lazy-input latch.
    /// The true GetPurchasedAir child needs its actual input/output host context;
    /// normal reporting reads the existing same unit after real simulation input.
    fn source_purchased_air_report_input(&self) -> Result<(), Error> {
        if self.purchased_air.get_purchased_air_input {
            return Err(Error::Unavailable("actual PurchasedAir report getter GetPurchasedAir lazy-input child"));
        }
        Ok(())
    }
    pub(crate) fn source_purchased_air_out_air_mass_flow(&self, number: i32) -> Result<f64, Error> {
        self.source_purchased_air_report_input()?;
        Ok(self.purchased_unit(number)?.outdoor_air_mass_flow_rate)
    }
    pub(crate) fn source_purchased_air_zone_inlet_air_node(&self, number: i32) -> Result<i32, Error> {
        self.source_purchased_air_report_input()?;
        let mut node = 0;
        if number > 0 && number <= self.purchased_air.number_purchased_air {
            node = self.purchased_unit(number)?.zone_supply_air_node_num;
        }
        Ok(node)
    }
    pub(crate) fn source_purchased_air_return_air_node(&self, number: i32) -> Result<i32, Error> {
        self.source_purchased_air_report_input()?;
        let mut node = 0;
        if number > 0 && number <= self.purchased_air.number_purchased_air {
            node = self.purchased_unit(number)?.zone_recirc_air_node_num;
        }
        Ok(node)
    }
    pub(crate) fn source_purchased_air_mixed_air_temperature(&self, number: i32) -> Result<f64, Error> {
        self.source_purchased_air_report_input()?;
        Ok(self.purchased_unit(number)?.mixed_air_temp)
    }
    pub(crate) fn source_purchased_air_mixed_air_humidity_ratio(&self, number: i32) -> Result<f64, Error> {
        self.source_purchased_air_report_input()?;
        Ok(self.purchased_unit(number)?.mixed_air_hum_rat)
    }
}
