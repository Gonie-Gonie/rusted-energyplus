//! Same-owner PurchasedAirManager Calc1906-2760, NoOA children and Update/Report.
//! Positive OA/psychrometric diagnostic operations retain explicit boundaries.
use super::{at, owners::*, OrdinaryEquipment};
use super::super::source_nodes::SourceNodeData;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_air_heat_balance::{SourceAirHeatBalance,
    environment::SourceZoneFanHeatBalanceArrays,
    predictor_corrector::SourceZoneAirPredictorCorrector};
use crate::heat_balance::ordinary_equipment::management::availability::SourceAvailability;
use crate::heat_balance::selected_heat_balance_input::{HeatBalanceInputError as Error, SelectedHeatBalanceInputState};
use crate::heat_balance::selected_schedule_manager::{SelectedScheduleManager, SourceScheduleIndex};
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::psychrometrics::{SourcePsychrometricAccess, EnergyPlusPsychrometricFunction as Property,
    energyplus_psy_cp_air_fn_w as cp, energyplus_psy_h_fn_tdb_w as enthalpy,
    energyplus_psy_tdb_fn_h_w as dry_bulb};
use crate::weather::day::WeatherEnvironmentState;

/// Only temporary borrows of the existing owners. No numerical storage or cache.
pub(crate) struct SourcePurchasedAirCalcOwners<'a, 'backend> {
    pub(crate) availability: &'a mut SourceAvailability,
    pub(crate) controller: &'a SourceZoneAirPredictorCorrector,
    pub(crate) air: &'a SourceAirHeatBalance,
    pub(crate) fan: &'a SourceZoneFanHeatBalanceArrays,
    pub(crate) hvac: &'a SelectedHvacGlobals,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) psychrometrics: &'a mut SourcePsychrometricAccess,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
// Actual ObjexxFCL/Fmath.hh double overloads: min selects the second on
// equality; max selects the first. Five-way max has the native 2/2/1 tree.
fn min(a:f64,b:f64)->f64 {if a<b {a}else{b}}
fn max(a:f64,b:f64)->f64 {if a<b {b}else{a}}
fn max3(a:f64,b:f64,c:f64)->f64 {if a<b {if b<c {c}else{b}}else if a<c {c}else{a}}
fn max5(a:f64,b:f64,c:f64,d:f64,e:f64)->f64 {max3(max(a,b),max(c,d),e)}
fn flow_limited(limit:SourcePurchasedAirLimit)->bool {
    matches!(limit,SourcePurchasedAirLimit::FlowRate|SourcePurchasedAirLimit::FlowRateAndCapacity)
}
fn capacity_limited(limit:SourcePurchasedAirLimit)->bool {
    matches!(limit,SourcePurchasedAirLimit::Capacity|SourcePurchasedAirLimit::FlowRateAndCapacity)
}
fn node(rows:&Option<Vec<SourceNodeData>>,number:i32)->Result<&SourceNodeData,Error> {
    rows.as_ref().and_then(|v|v.get(at(number).ok()?))
        .ok_or(Error::Unavailable("actual CalcPurchAirLoads physical Node ordinal"))
}
fn node_mut(rows:&mut Option<Vec<SourceNodeData>>,number:i32)->Result<&mut SourceNodeData,Error> {
    rows.as_mut().and_then(|v|v.get_mut(at(number).ok()?))
        .ok_or(Error::Unavailable("actual CalcPurchAirLoads mutable physical Node ordinal"))
}
fn schedule(schedules:&SelectedScheduleManager,pointer:Option<SourceScheduleIndex>)->Result<f64,Error>{
    schedules.source_current_schedule_value(pointer.ok_or(Error::Unavailable("actual PurchasedAir Schedule pointer"))?)
}
fn saturation(o:&mut SourcePurchasedAirCalcOwners<'_, '_>,temperature:f64)->Result<f64,Error>{
    // A positive out-of-range Psat diagnostic is not represented by the
    // existing property transport. Stop at the actual call, never clamp input.
    if temperature<=-100.0 || temperature>=200.0 {
        return Err(Error::Unavailable("actual CalcPurchAirLoads PsyPsatFnTemp diagnostic inside WFromRh"));
    }
    Ok(o.psychrometrics.evaluate(Property::WFromRh,&[temperature,1.0,o.environment.current_weather.out_baro_press])
        .map_err(|_|Error::Unavailable("actual CalcPurchAirLoads WFromRh state/cache"))?.result)
}
fn humidity_from_h(_o:&mut SourcePurchasedAirCalcOwners<'_, '_>,_temperature:f64,_h:f64)->Result<f64,Error>{
    // The current scalar helper hides the pre-floor raw W used by the Native
    // warning call. No second inverse equation or fabricated recurrence state.
    Err(Error::Unavailable("actual CalcPurchAirLoads PsyWFnTdbH diagnostic-aware operation"))
}
fn saturation_temperature(_o:&mut SourcePurchasedAirCalcOwners<'_, '_>,_h:f64)->Result<f64,Error>{
    // Native raw Tsat has range, nested Psat and convergence diagnostics which
    // the current property transport does not expose. Stop at the genuine
    // property call before silently accepting that unrepresented operation.
    Err(Error::Unavailable("actual CalcPurchAirLoads PsyTsatFnHPb diagnostic-aware operation"))
}
fn mixed_air(unit:&mut SourceZonePurchasedAir,nodes:&Option<Vec<SourceNodeData>>,oa:f64)->Result<f64,Error>{
    // Native2812-2939 reads each recirculation scalar before its OA branch.
    let recirc_temperature=node(nodes,unit.zone_recirc_air_node_num)?.temperature;
    let recirc_humidity=node(nodes,unit.zone_recirc_air_node_num)?.humidity_ratio;
    let recirc_enthalpy=node(nodes,unit.zone_recirc_air_node_num)?.enthalpy;
    if unit.outdoor_air && oa>0.0 {
        return Err(Error::Unavailable("actual CalcPurchAirMixedAir positive outdoor-air heat-recovery operation"));
    }
    unit.mixed_air_temp=recirc_temperature;
    unit.mixed_air_hum_rat=recirc_humidity;
    unit.ht_rec_sen_output=0.0;
    unit.ht_rec_lat_output=0.0;
    Ok(recirc_enthalpy)
}

impl OrdinaryEquipment {
    pub(super) fn calculate_source_purchased_air(&mut self,unit_number:i32,controlled_zone:i32,
        system_output:&mut f64,moisture_output:&mut f64,schedules:&SelectedScheduleManager,
        o:&mut SourcePurchasedAirCalcOwners<'_, '_>)->Result<(),Error>{
        let unit=self.purchased_air.units.as_mut().and_then(|rows|rows.get_mut(at(unit_number).ok()?))
            .ok_or(Error::Unavailable("actual CalcPurchAirLoads retained unit"))?;
        // These are Native local integer reads, not snapshots of Node state.
        let supply_node=unit.zone_supply_air_node_num;
        let zone_node=i32::try_from(self.zone_config.as_ref().and_then(|rows|rows.get(at(controlled_zone).ok()?))
            .ok_or(Error::Unavailable("actual CalcPurchAirLoads controlled Config.ZoneNode"))?.zone_node)
            .map_err(|_|Error::Unavailable("actual CalcPurchAirLoads ZoneNode integer domain"))?;
        let outdoor_node=unit.outdoor_air_node_num;
        let recirc_node=unit.zone_recirc_air_node_num;
        let nodes=&mut self.nodes.physical;
        let mut supply_flow=0.0;
        let mut oa_flow=0.0;
        unit.min_oa_mass_flow_rate=0.0;
        unit.time_econo_active=0.0;
        unit.time_ht_rec_active=0.0;
        *system_output=0.0;
        *moisture_output=0.0;
        let mut unit_on=true;
        let heat_load=o.controller.demand.zones.as_ref().and_then(|rows|rows.get(at(controlled_zone).ok()?))
            .ok_or(Error::Unavailable("actual CalcPurchAirLoads heating demand"))?.remaining_output_to_heat_sp;
        let cool_load=o.controller.demand.zones.as_ref().and_then(|rows|rows.get(at(controlled_zone).ok()?))
            .ok_or(Error::Unavailable("actual CalcPurchAirLoads cooling demand"))?.remaining_output_to_cool_sp;
        if let Some(types)=o.availability.zone_components.as_mut(){
            let manager=types.get_mut(11).and_then(|row|row.managers.as_mut())
                .and_then(|rows|rows.get_mut(at(unit_number).ok()?))
                .ok_or(Error::Unavailable("actual PurchasedAir ZoneCompAvailMgrs ordinal"))?;
            manager.zone_number=controlled_zone;
            unit.avail_status=manager.status;
            if unit.avail_status==1 {unit_on=false;}
        }
        if schedule(schedules,unit.avail_sched)?<=0.0 {unit_on=false;}
        let mut heat_on=true;
        if schedule(schedules,unit.heat_avail_sched)?<=0.0 {heat_on=false;}
        let mut cool_on=true;
        if schedule(schedules,unit.cool_avail_sched)?<=0.0 {cool_on=false;}
        if unit_on {
            // Actual CalcPurchAirMinOAMassFlow2762-2810. NoOA is a real
            // producer branch; positive OA stops at its first real child.
            if unit.outdoor_air {
                return Err(Error::Unavailable("actual CalcPurchAirMinOAMassFlow calcDesignSpecificationOutdoorAir2789"));
            }else{oa_flow=0.0;}
            unit.min_oa_mass_flow_rate=oa_flow;
            if unit.ems_override_oa_mdot_on {oa_flow=unit.ems_value_oa_mass_flow_rate;}
            let min_oa_sensible;
            let _min_oa_latent;
            if unit.outdoor_air {
                let specific_heat=cp(node(nodes,outdoor_node)?.humidity_ratio);
                min_oa_sensible=oa_flow*specific_heat*(node(nodes,outdoor_node)?.temperature-node(nodes,zone_node)?.temperature);
                _min_oa_latent=oa_flow*(node(nodes,outdoor_node)?.humidity_ratio-node(nodes,zone_node)?.humidity_ratio);
            }else{min_oa_sensible=0.0;_min_oa_latent=0.0;}
            let control_type=*o.fan.thermostat.temp_control_type.as_ref().and_then(|rows|rows.get(at(controlled_zone).ok()?))
                .ok_or(Error::Unavailable("actual CalcPurchAirLoads TempControlType"))?;
            let air_humidity=o.air.zones.as_ref().and_then(|rows|rows.get(at(controlled_zone).ok()?))
                .ok_or(Error::Unavailable("actual CalcPurchAirLoads same zoneHeatBalance"))?.air.air_hum_rat;
            let mixed_enthalpy;
            let mut supply_enthalpy;
            if min_oa_sensible>=cool_load && control_type!=1 {
                if flow_limited(unit.cooling_limit) && oa_flow>unit.max_cool_mass_flow_rate {
                    let _oa_volume=oa_flow/o.environment.std_rho_air;
                    if unit.oa_flow_max_cool_output_error<1 {unit.oa_flow_max_cool_output_error+=1;
                        return Err(Error::Unavailable("actual PurchasedAir cooling OA-flow ShowWarningError"));}
                    return Err(Error::Unavailable("actual PurchasedAir cooling OA-flow recurring warning"));
                }else if unit.economizer_type!=SourcePurchasedAirEconomizer::NoEconomizer {
                    return Err(Error::Unavailable("actual PurchasedAir positive economizer operation"));
                }
                let mut cool_flow=0.0;
                if cool_on {let specific_heat=cp(air_humidity);
                    let delta=unit.min_cool_supp_air_temp-node(nodes,zone_node)?.temperature;
                    if delta< -1.0e-5 {cool_flow=cool_load/specific_heat/delta;}}
                let mut dehumid_flow=0.0;
                if cool_on && unit.dehumid_ctrl_type==SourcePurchasedAirHumidityControl::Humidistat {
                    return Err(Error::Unavailable("actual PurchasedAir cooling humidistat moisture-demand operation"));
                }
                let mut humid_flow=0.0;
                if heat_on && unit.humid_ctrl_type==SourcePurchasedAirHumidityControl::Humidistat &&
                    matches!(unit.dehumid_ctrl_type,SourcePurchasedAirHumidityControl::Humidistat|SourcePurchasedAirHumidityControl::None){
                    return Err(Error::Unavailable("actual PurchasedAir cooling humidification moisture-demand operation"));
                }
                if capacity_limited(unit.cooling_limit)&&unit.max_cool_tot_cap==0.0 {
                    cool_flow=0.0;dehumid_flow=0.0;humid_flow=0.0;
                }
                supply_flow=max5(0.0,oa_flow,cool_flow,dehumid_flow,humid_flow);
                if unit.ems_override_mdot_on {supply_flow=unit.ems_value_mass_flow_rate;oa_flow=min(oa_flow,supply_flow);}
                if flow_limited(unit.cooling_limit)&&unit.max_cool_mass_flow_rate>0.0 {supply_flow=min(supply_flow,unit.max_cool_mass_flow_rate);}
                if supply_flow<=1.0e-30 {supply_flow=0.0;}
                mixed_enthalpy=mixed_air(unit,nodes,oa_flow)?;
                if supply_flow>0.0 {
                    let specific_heat=cp(air_humidity);
                    unit.supply_temp=cool_load/(specific_heat*supply_flow)+node(nodes,zone_node)?.temperature;
                    unit.supply_temp=max(unit.supply_temp,unit.min_cool_supp_air_temp);
                    unit.supply_temp=min(unit.supply_temp,unit.mixed_air_temp);
                    unit.supply_hum_rat=unit.mixed_air_hum_rat;
                    supply_enthalpy=enthalpy(unit.supply_temp,unit.supply_hum_rat);
                    if capacity_limited(unit.cooling_limit){
                        let _specific_heat=cp(unit.mixed_air_hum_rat); // Native call is real despite unused Cp local.
                        let mut cooling_sensible=supply_flow*(mixed_enthalpy-supply_enthalpy);
                        if cooling_sensible>=unit.max_cool_tot_cap {
                            cooling_sensible=unit.max_cool_tot_cap;
                            supply_enthalpy=mixed_enthalpy-cooling_sensible/supply_flow;
                            unit.supply_temp=dry_bulb(supply_enthalpy,unit.supply_hum_rat);
                            unit.supply_temp=min(unit.supply_temp,unit.mixed_air_temp);
                        }
                    }
                    unit.supply_hum_rat=unit.mixed_air_hum_rat;
                    match unit.dehumid_ctrl_type {
                        SourcePurchasedAirHumidityControl::None=>{unit.supply_hum_rat=unit.mixed_air_hum_rat;}
                        SourcePurchasedAirHumidityControl::ConstantSupplyHumidityRatio=>{unit.supply_hum_rat=unit.min_cool_supp_air_hum_rat;}
                        SourcePurchasedAirHumidityControl::ConstantSensibleHeatRatio=>return Err(Error::Unavailable("actual PurchasedAir cooling SHR humidity operation")),
                        SourcePurchasedAirHumidityControl::Humidistat=>return Err(Error::Unavailable("actual PurchasedAir cooling humidistat supply humidity operation")),
                        _=>{unit.supply_hum_rat=unit.mixed_air_hum_rat;}
                    }
                    if heat_on && unit.humid_ctrl_type==SourcePurchasedAirHumidityControl::Humidistat &&
                        matches!(unit.dehumid_ctrl_type,SourcePurchasedAirHumidityControl::Humidistat|SourcePurchasedAirHumidityControl::None){
                        return Err(Error::Unavailable("actual PurchasedAir cooling humidification supply humidity operation"));
                    }
                    let original_humidity=unit.supply_hum_rat;
                    let saturated_humidity=saturation(o,unit.supply_temp)?;
                    unit.supply_hum_rat=min(original_humidity,saturated_humidity);
                    supply_enthalpy=enthalpy(unit.supply_temp,unit.supply_hum_rat);
                    if capacity_limited(unit.cooling_limit){
                        if unit.supply_hum_rat<unit.mixed_air_hum_rat {
                            let mut cooling_total=supply_flow*(mixed_enthalpy-supply_enthalpy);
                            if cooling_total>unit.max_cool_tot_cap {
                                cooling_total=unit.max_cool_tot_cap;
                                supply_enthalpy=mixed_enthalpy-cooling_total/supply_flow;
                                match unit.dehumid_ctrl_type {
                                    SourcePurchasedAirHumidityControl::None|SourcePurchasedAirHumidityControl::ConstantSupplyHumidityRatio=>{
                                        let specific_heat=cp(unit.mixed_air_hum_rat);
                                        let cooling_sensible=supply_flow*specific_heat*(unit.mixed_air_temp-unit.supply_temp);
                                        let cooling_latent=cooling_total-cooling_sensible;
                                        if cooling_latent>=unit.max_cool_tot_cap {
                                            unit.supply_temp=unit.mixed_air_temp;
                                            unit.supply_hum_rat=humidity_from_h(o,unit.supply_temp,supply_enthalpy)?;
                                        }else{
                                            unit.supply_temp=dry_bulb(supply_enthalpy,unit.supply_hum_rat);
                                            unit.supply_temp=min(unit.supply_temp,unit.mixed_air_temp);
                                        }
                                    }
                                    SourcePurchasedAirHumidityControl::ConstantSensibleHeatRatio=>return Err(Error::Unavailable("actual PurchasedAir capacity SHR humidity operation")),
                                    SourcePurchasedAirHumidityControl::Humidistat=>{unit.supply_hum_rat=humidity_from_h(o,unit.supply_temp,supply_enthalpy)?;}
                                    _=>{}
                                }
                                let original_humidity=unit.supply_hum_rat;
                                let saturated_humidity=saturation(o,unit.supply_temp)?;
                                if saturated_humidity<original_humidity {
                                    unit.supply_temp=saturation_temperature(o,supply_enthalpy)?;
                                    unit.supply_temp=min(unit.supply_temp,unit.mixed_air_temp);
                                    unit.supply_hum_rat=humidity_from_h(o,unit.supply_temp,supply_enthalpy)?;
                                    supply_enthalpy=enthalpy(unit.supply_temp,unit.supply_hum_rat);
                                }
                            }
                        }else{
                            let specific_heat=cp(unit.mixed_air_hum_rat);
                            let mut cooling_sensible=supply_flow*specific_heat*(unit.mixed_air_temp-unit.supply_temp);
                            if cooling_sensible>=unit.max_cool_tot_cap {
                                cooling_sensible=unit.max_cool_tot_cap;
                                unit.supply_temp=unit.mixed_air_temp-cooling_sensible/(supply_flow*specific_heat);
                            }
                        }
                    }
                }else{
                    supply_enthalpy=mixed_enthalpy;
                    unit.supply_hum_rat=unit.mixed_air_hum_rat;
                    unit.supply_temp=unit.mixed_air_temp;
                }
            }else{
                let heating=min_oa_sensible<heat_load && control_type!=2;
                if flow_limited(unit.heating_limit)&&oa_flow>unit.max_heat_mass_flow_rate {
                    let _oa_volume=oa_flow/o.environment.std_rho_air;
                    if unit.oa_flow_max_heat_output_error<1 {unit.oa_flow_max_heat_output_error+=1;
                        return Err(Error::Unavailable("actual PurchasedAir heating OA-flow ShowWarningError"));}
                    return Err(Error::Unavailable("actual PurchasedAir heating OA-flow recurring warning"));
                }
                let mut heat_flow=0.0;
                if heat_on&&heating {let specific_heat=cp(air_humidity);
                    let delta=unit.max_heat_supp_air_temp-node(nodes,zone_node)?.temperature;
                    if delta>1.0e-5 {heat_flow=heat_load/specific_heat/delta;}}
                let mut dehumid_flow=0.0;
                if cool_on&&unit.dehumid_ctrl_type==SourcePurchasedAirHumidityControl::Humidistat &&
                    (matches!(unit.humid_ctrl_type,SourcePurchasedAirHumidityControl::Humidistat|SourcePurchasedAirHumidityControl::None)||!heating){
                    return Err(Error::Unavailable("actual PurchasedAir heating/deadband dehumidification demand operation"));
                }
                let mut humid_flow=0.0;
                if heat_on&&unit.humid_ctrl_type==SourcePurchasedAirHumidityControl::Humidistat {
                    return Err(Error::Unavailable("actual PurchasedAir heating humidification demand operation"));
                }
                if capacity_limited(unit.heating_limit)&&unit.max_heat_sens_cap==0.0 {heat_flow=0.0;dehumid_flow=0.0;humid_flow=0.0;}
                supply_flow=max5(0.0,oa_flow,heat_flow,dehumid_flow,humid_flow);
                if unit.ems_override_mdot_on {supply_flow=unit.ems_value_mass_flow_rate;oa_flow=min(oa_flow,supply_flow);}
                if flow_limited(unit.heating_limit)&&unit.max_heat_mass_flow_rate>0.0 {supply_flow=min(supply_flow,unit.max_heat_mass_flow_rate);}
                if supply_flow<=1.0e-30 {supply_flow=0.0;}
                mixed_enthalpy=mixed_air(unit,nodes,oa_flow)?;
                if supply_flow>0.0 {
                    if heat_on&&heating {
                        let specific_heat=cp(air_humidity);
                        unit.supply_temp=heat_load/(specific_heat*supply_flow)+node(nodes,zone_node)?.temperature;
                        unit.supply_temp=min(unit.supply_temp,unit.max_heat_supp_air_temp);
                        unit.supply_temp=max(unit.supply_temp,unit.mixed_air_temp);
                        if capacity_limited(unit.heating_limit){
                            let specific_heat=cp(unit.mixed_air_hum_rat);
                            let heating_sensible=supply_flow*specific_heat*(unit.supply_temp-unit.mixed_air_temp);
                            if heating_sensible>unit.max_heat_sens_cap {
                                unit.supply_temp=unit.max_heat_sens_cap/(supply_flow*specific_heat)+unit.mixed_air_temp;
                            }
                        }
                    }else{unit.supply_temp=unit.mixed_air_temp;}
                    unit.supply_hum_rat=unit.mixed_air_hum_rat;
                    match unit.humid_ctrl_type {
                        SourcePurchasedAirHumidityControl::None=>{unit.supply_hum_rat=unit.mixed_air_hum_rat;}
                        SourcePurchasedAirHumidityControl::Humidistat=>return Err(Error::Unavailable("actual PurchasedAir heating humidistat supply humidity operation")),
                        SourcePurchasedAirHumidityControl::ConstantSupplyHumidityRatio=>{
                            if heating {return Err(Error::Unavailable("actual PurchasedAir heating constant supply humidity operation"));}
                            unit.supply_hum_rat=unit.mixed_air_hum_rat;
                        }
                        _=>{unit.supply_hum_rat=unit.mixed_air_hum_rat;}
                    }
                    if cool_on&&unit.dehumid_ctrl_type==SourcePurchasedAirHumidityControl::Humidistat &&
                        (matches!(unit.humid_ctrl_type,SourcePurchasedAirHumidityControl::Humidistat|SourcePurchasedAirHumidityControl::None)||!heating){
                        return Err(Error::Unavailable("actual PurchasedAir heating/deadband dehumidification supply operation"));
                    }
                    unit.supply_hum_rat=min(unit.supply_hum_rat,saturation(o,unit.supply_temp)?);
                    supply_enthalpy=enthalpy(unit.supply_temp,unit.supply_hum_rat);
                }else{
                    supply_enthalpy=mixed_enthalpy;
                    unit.supply_hum_rat=unit.mixed_air_hum_rat;
                    unit.supply_temp=unit.mixed_air_temp;
                }
            }
            if supply_flow>0.0 {
                if unit.ems_override_supply_temp_on {unit.supply_temp=unit.ems_value_supply_temp;}
                if unit.ems_override_supply_hum_rat_on {unit.supply_hum_rat=unit.ems_value_supply_hum_rat;}
                supply_enthalpy=enthalpy(unit.supply_temp,unit.supply_hum_rat);
                if unit.supply_hum_rat==unit.mixed_air_hum_rat && unit.supply_temp==unit.mixed_air_temp {
                    unit.sen_coil_load=0.0;unit.lat_coil_load=0.0;
                }else if unit.supply_hum_rat==unit.mixed_air_hum_rat && unit.supply_temp!=unit.mixed_air_temp {
                    unit.sen_coil_load=supply_flow*(supply_enthalpy-mixed_enthalpy);unit.lat_coil_load=0.0;
                }else{
                    let specific_heat=cp(unit.mixed_air_hum_rat);
                    unit.sen_coil_load=supply_flow*specific_heat*(unit.supply_temp-unit.mixed_air_temp);
                    unit.lat_coil_load=supply_flow*(supply_enthalpy-mixed_enthalpy)-unit.sen_coil_load;
                }
                if (unit.sen_coil_load>0.0&&!heat_on)||(unit.sen_coil_load<0.0&&!cool_on){unit.sen_coil_load=0.0;unit.supply_temp=unit.mixed_air_temp;}
                if (unit.lat_coil_load>0.0&&!heat_on)||(unit.lat_coil_load<0.0&&!cool_on){unit.lat_coil_load=0.0;unit.supply_hum_rat=unit.mixed_air_hum_rat;}
                let original_humidity=unit.supply_hum_rat;
                let saturated_humidity=saturation(o,unit.supply_temp)?;
                let delta_humidity=original_humidity-saturated_humidity;
                if delta_humidity>0.00025 {
                    if unit.saturation_output_error<1 {unit.saturation_output_error+=1;
                        return Err(Error::Unavailable("actual PurchasedAir saturation ShowWarningError"));}
                    return Err(Error::Unavailable("actual PurchasedAir saturation recurring warning"));
                }
                supply_enthalpy=enthalpy(unit.supply_temp,unit.supply_hum_rat);
                let specific_heat=cp(air_humidity);
                *system_output=supply_flow*specific_heat*(unit.supply_temp-node(nodes,zone_node)?.temperature);
                *moisture_output=supply_flow*(unit.supply_hum_rat-node(nodes,zone_node)?.humidity_ratio);
                unit.sen_output_to_zone=*system_output;
                unit.lat_output_to_zone=supply_flow*(supply_enthalpy-node(nodes,zone_node)?.enthalpy)-unit.sen_output_to_zone;
                let specific_heat=cp(air_humidity);
                if unit.outdoor_air {
                    unit.oa_sen_output=oa_flow*specific_heat*(node(nodes,outdoor_node)?.temperature-node(nodes,zone_node)?.temperature);
                    unit.oa_lat_output=oa_flow*(node(nodes,outdoor_node)?.enthalpy-node(nodes,zone_node)?.enthalpy)-unit.oa_sen_output;
                }else{unit.oa_sen_output=0.0;unit.oa_lat_output=0.0;}
                if o.control.carbon_dioxide_simulation {
                    let value=node(nodes,recirc_node)?.carbon_dioxide;
                    node_mut(nodes,supply_node)?.carbon_dioxide=value;
                }
                if o.control.generic_contaminant_simulation {
                    let value=node(nodes,recirc_node)?.generic_contaminant;
                    node_mut(nodes,supply_node)?.generic_contaminant=value;
                }
            }else{
                *system_output=0.0;*moisture_output=0.0;
                unit.sen_output_to_zone=0.0;unit.lat_output_to_zone=0.0;
                unit.sen_coil_load=0.0;unit.lat_coil_load=0.0;unit.oa_sen_output=0.0;unit.oa_lat_output=0.0;
                unit.mixed_air_temp=node(nodes,recirc_node)?.temperature;
                unit.mixed_air_hum_rat=node(nodes,recirc_node)?.humidity_ratio;
                if o.control.carbon_dioxide_simulation {let value=node(nodes,zone_node)?.carbon_dioxide;node_mut(nodes,supply_node)?.carbon_dioxide=value;}
                if o.control.generic_contaminant_simulation {let value=node(nodes,zone_node)?.generic_contaminant;node_mut(nodes,supply_node)?.generic_contaminant=value;}
            }
            node_mut(nodes,supply_node)?.temperature=unit.supply_temp;
            node_mut(nodes,supply_node)?.humidity_ratio=unit.supply_hum_rat;
            node_mut(nodes,supply_node)?.enthalpy=supply_enthalpy;
            node_mut(nodes,supply_node)?.mass_flow_rate=supply_flow;
            if unit.outdoor_air {node_mut(nodes,outdoor_node)?.mass_flow_rate=oa_flow;}
        }else{
            *system_output=0.0;*moisture_output=0.0;supply_flow=0.0;oa_flow=0.0;
            let value=node(nodes,zone_node)?.temperature;node_mut(nodes,supply_node)?.temperature=value;
            let value=node(nodes,zone_node)?.humidity_ratio;node_mut(nodes,supply_node)?.humidity_ratio=value;
            let value=node(nodes,zone_node)?.enthalpy;node_mut(nodes,supply_node)?.enthalpy=value;
            if o.control.carbon_dioxide_simulation {let value=node(nodes,zone_node)?.carbon_dioxide;node_mut(nodes,supply_node)?.carbon_dioxide=value;}
            if o.control.generic_contaminant_simulation {let value=node(nodes,zone_node)?.generic_contaminant;node_mut(nodes,supply_node)?.generic_contaminant=value;}
            node_mut(nodes,supply_node)?.mass_flow_rate=0.0;
            if unit.outdoor_air {node_mut(nodes,outdoor_node)?.mass_flow_rate=0.0;}
            unit.sen_heat_rate=0.0;unit.sen_cool_rate=0.0;unit.tot_cool_rate=0.0;
            unit.sen_output_to_zone=0.0;unit.lat_output_to_zone=0.0;unit.sen_coil_load=0.0;unit.lat_coil_load=0.0;
            unit.oa_sen_output=0.0;unit.oa_lat_output=0.0;
            unit.mixed_air_temp=node(nodes,recirc_node)?.temperature;
            unit.mixed_air_hum_rat=node(nodes,recirc_node)?.humidity_ratio;
            unit.supply_temp=node(nodes,supply_node)?.temperature;
            unit.supply_hum_rat=node(nodes,supply_node)?.humidity_ratio;
        }
        unit.outdoor_air_mass_flow_rate=oa_flow;
        unit.outdoor_air_vol_flow_rate_std_rho=oa_flow/o.environment.std_rho_air;
        unit.supply_air_mass_flow_rate=supply_flow;
        unit.supply_air_vol_flow_rate_std_rho=supply_flow/o.environment.std_rho_air;
        if unit.plenum_exhaust_air_node_num>0 {node_mut(nodes,unit.plenum_exhaust_air_node_num)?.mass_flow_rate=supply_flow;}
        node_mut(nodes,recirc_node)?.mass_flow_rate=supply_flow;
        Ok(())
    }

    pub(super) fn update_source_purchased_air(&mut self,unit_number:i32,_first_hvac_iteration:bool)->Result<(),Error>{
        let unit=self.purchased_unit(unit_number)?;
        let plenum_index=unit.return_plenum_index;
        let array_index=unit.purch_air_array_index;
        if plenum_index>0 {
            let flags=self.purchased_air.plenum_arrays.as_mut().and_then(|rows|rows.get_mut(at(plenum_index).ok()?))
                .and_then(|row|row.is_simulated.as_mut()).ok_or(Error::Unavailable("actual UpdatePurchasedAir PlenumArray.IsSimulated"))?;
            *flags.get_mut(at(array_index)?).ok_or(Error::Unavailable("actual UpdatePurchasedAir PurchAirArrayIndex"))?=true;
            if flags.iter().all(|flag|*flag){return Err(Error::Unavailable("actual UpdatePurchasedAir SimAirZonePlenum2970"));}
        }
        Ok(())
    }

    pub(super) fn report_source_purchased_air(&mut self,unit_number:i32,schedules:&SelectedScheduleManager,
        o:&SourcePurchasedAirCalcOwners<'_, '_>)->Result<(),Error>{
        let seconds=o.hvac.time_step_sys_seconds;
        let unit=self.purchased_unit_mut(unit_number)?;
        unit.sen_heat_rate=max(unit.sen_coil_load,0.0);
        unit.sen_cool_rate=min(unit.sen_coil_load,0.0).abs();
        unit.lat_heat_rate=max(unit.lat_coil_load,0.0);
        unit.lat_cool_rate=min(unit.lat_coil_load,0.0).abs();
        unit.tot_heat_rate=unit.sen_heat_rate+unit.lat_heat_rate;
        unit.tot_cool_rate=unit.sen_cool_rate+unit.lat_cool_rate;
        unit.zone_sen_heat_rate=max(unit.sen_output_to_zone,0.0);
        unit.zone_sen_cool_rate=min(unit.sen_output_to_zone,0.0).abs();
        unit.zone_lat_heat_rate=max(unit.lat_output_to_zone,0.0);
        unit.zone_lat_cool_rate=min(unit.lat_output_to_zone,0.0).abs();
        unit.zone_tot_heat_rate=unit.zone_sen_heat_rate+unit.zone_lat_heat_rate;
        unit.zone_tot_cool_rate=unit.zone_sen_cool_rate+unit.zone_lat_cool_rate;
        if unit.sen_coil_load>0.0 {unit.oa_sen_heat_rate=min(unit.oa_sen_output,0.0).abs();}
        else{unit.oa_sen_heat_rate=0.0;}
        if unit.sen_coil_load<0.0 {unit.oa_sen_cool_rate=max(unit.oa_sen_output,0.0);}
        else{unit.oa_sen_cool_rate=0.0;}
        if unit.lat_coil_load>0.0 {unit.oa_lat_heat_rate=min(unit.oa_lat_output,0.0).abs();}
        else{unit.oa_lat_heat_rate=0.0;}
        if unit.lat_coil_load<0.0 {unit.oa_lat_cool_rate=max(unit.oa_lat_output,0.0);}
        else{unit.oa_lat_cool_rate=0.0;}
        unit.oa_tot_heat_rate=unit.oa_sen_heat_rate+unit.oa_lat_heat_rate;
        unit.oa_tot_cool_rate=unit.oa_sen_cool_rate+unit.oa_lat_cool_rate;
        unit.ht_rec_sen_heat_rate=max(unit.ht_rec_sen_output,0.0);
        unit.ht_rec_sen_cool_rate=min(unit.ht_rec_sen_output,0.0).abs();
        unit.ht_rec_lat_heat_rate=max(unit.ht_rec_lat_output,0.0);
        unit.ht_rec_lat_cool_rate=min(unit.ht_rec_lat_output,0.0).abs();
        unit.ht_rec_tot_heat_rate=unit.ht_rec_sen_heat_rate+unit.ht_rec_lat_heat_rate;
        unit.ht_rec_tot_cool_rate=unit.ht_rec_sen_cool_rate+unit.ht_rec_lat_cool_rate;
        let mut heating_efficiency=schedule(schedules,unit.heat_fuel_eff_sched)?;
        if heating_efficiency<=0.0 {heating_efficiency=1.0;}
        unit.tot_heat_fuel_rate=unit.tot_heat_rate/heating_efficiency;
        unit.zone_tot_heat_fuel_rate=unit.zone_tot_heat_rate/heating_efficiency;
        let mut cooling_efficiency=schedule(schedules,unit.cool_fuel_eff_sched)?;
        if cooling_efficiency<=0.0 {cooling_efficiency=1.0;}
        unit.zone_tot_cool_fuel_rate=unit.zone_tot_cool_rate/cooling_efficiency;
        unit.tot_cool_fuel_rate=unit.tot_cool_rate/cooling_efficiency;
        unit.sen_heat_energy=unit.sen_heat_rate*seconds;
        unit.sen_cool_energy=unit.sen_cool_rate*seconds;
        unit.lat_heat_energy=unit.lat_heat_rate*seconds;
        unit.lat_cool_energy=unit.lat_cool_rate*seconds;
        unit.tot_heat_energy=unit.tot_heat_rate*seconds;
        unit.tot_cool_energy=unit.tot_cool_rate*seconds;
        unit.zone_sen_heat_energy=unit.zone_sen_heat_rate*seconds;
        unit.zone_sen_cool_energy=unit.zone_sen_cool_rate*seconds;
        unit.zone_lat_heat_energy=unit.zone_lat_heat_rate*seconds;
        unit.zone_lat_cool_energy=unit.zone_lat_cool_rate*seconds;
        unit.zone_tot_heat_energy=unit.zone_tot_heat_rate*seconds;
        unit.zone_tot_cool_energy=unit.zone_tot_cool_rate*seconds;
        unit.oa_sen_heat_energy=unit.oa_sen_heat_rate*seconds;
        unit.oa_sen_cool_energy=unit.oa_sen_cool_rate*seconds;
        unit.oa_lat_heat_energy=unit.oa_lat_heat_rate*seconds;
        unit.oa_lat_cool_energy=unit.oa_lat_cool_rate*seconds;
        unit.oa_tot_heat_energy=unit.oa_tot_heat_rate*seconds;
        unit.oa_tot_cool_energy=unit.oa_tot_cool_rate*seconds;
        unit.ht_rec_sen_heat_energy=unit.ht_rec_sen_heat_rate*seconds;
        unit.ht_rec_sen_cool_energy=unit.ht_rec_sen_cool_rate*seconds;
        unit.ht_rec_lat_heat_energy=unit.ht_rec_lat_heat_rate*seconds;
        unit.ht_rec_lat_cool_energy=unit.ht_rec_lat_cool_rate*seconds;
        unit.ht_rec_tot_heat_energy=unit.ht_rec_tot_heat_rate*seconds;
        unit.ht_rec_tot_cool_energy=unit.ht_rec_tot_cool_rate*seconds;
        unit.zone_tot_heat_fuel_energy=unit.zone_tot_heat_fuel_rate*seconds;
        unit.zone_tot_cool_fuel_energy=unit.zone_tot_cool_fuel_rate*seconds;
        unit.tot_heat_fuel_energy=unit.tot_heat_fuel_rate*seconds;
        unit.tot_cool_fuel_energy=unit.tot_cool_fuel_rate*seconds;
        Ok(())
    }
}
