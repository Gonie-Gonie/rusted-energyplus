//! Genuine finite controlled Zone mass balance and leaving conditions.
//! Temporary same-owner borrows only; no Config/Node/flow numerical mirrors.
use super::{at, output_required};
use super::super::{EquipmentConfig, source_nodes::SourceNodeData};
use crate::heat_balance::ordinary_hvac_manager::SourceHvacOwners;
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::psychrometrics::{energyplus_psy_cp_air_fn_w as cp,energyplus_psy_h_fn_tdb_w as enthalpy,
    energyplus_psy_hg_air_fn_w_tdb as heat_of_vaporization};

fn max(a:f64,b:f64)->f64{if a<b {b}else{a}}
fn row<T>(rows:&Option<Vec<T>>,index:usize)->Result<&T,Error>{
    rows.as_ref().and_then(|v|v.get(index)).ok_or(Error::Unavailable("actual controlled air-balance array element"))
}
fn row_mut<T>(rows:&mut Option<Vec<T>>,index:usize)->Result<&mut T,Error>{
    rows.as_mut().and_then(|v|v.get_mut(index)).ok_or(Error::Unavailable("actual controlled air-balance mutable array element"))
}
fn node(rows:&Option<Vec<SourceNodeData>>,number:usize)->Result<&SourceNodeData,Error>{
    row(rows,number.checked_sub(1).ok_or(Error::Unavailable("actual positive physical Node number"))?)
}
fn node_mut(rows:&mut Option<Vec<SourceNodeData>>,number:usize)->Result<&mut SourceNodeData,Error>{
    row_mut(rows,number.checked_sub(1).ok_or(Error::Unavailable("actual positive mutable physical Node number"))?)
}
fn node_number(number:i32)->Result<usize,Error>{
    usize::try_from(number).map_err(|_|Error::Unavailable("actual physical Node integer domain"))
}

// DataZoneEquipment1866-1889. Each sum reads the actual inlet in order; only
// afterwards are the five ZoneNode flow fields written, including alias cases.
fn set_total_inlet_flows(config:&mut EquipmentConfig,nodes:&mut Option<Vec<SourceNodeData>>)->Result<(),Error>{
    config.total_inlet_air_mass_flow_rate=0.0;
    let mut maximum=0.0;let mut maximum_available=0.0;
    let mut minimum=0.0;let mut minimum_available=0.0;
    for index in 0..config.inlet_nodes {
        let number=*row(&config.inlet_node_numbers,index)?;
        let current=node(nodes,number)?;
        config.total_inlet_air_mass_flow_rate+=current.mass_flow_rate;
        maximum+=current.mass_flow_rate_max;
        maximum_available+=current.mass_flow_rate_max_available;
        minimum+=current.mass_flow_rate_min;
        minimum_available+=current.mass_flow_rate_min_available;
    }
    let zone=node_mut(nodes,config.zone_node)?;
    zone.mass_flow_rate=config.total_inlet_air_mass_flow_rate;
    zone.mass_flow_rate_max=maximum;
    zone.mass_flow_rate_max_available=maximum_available;
    zone.mass_flow_rate_min=minimum;
    zone.mass_flow_rate_min_available=minimum_available;
    Ok(())
}

// DataZoneEquipment2329-2505. Expected and final are the actual Native ref
// locals; all source schedules and FixedReturnFlow cells retain their owner.
fn calculate_return_flows(config:&mut EquipmentConfig,nodes:&mut Option<Vec<SourceNodeData>>,
    schedules:&SelectedScheduleManager,doing_sizing:bool,flow_adjustment:i32,
    expected:&mut f64,final_flow:&mut f64)->Result<(),Error>{
    let count=config.return_nodes;
    let mut total=0.0;let mut variable_total=0.0;
    let fraction=schedules.source_current_schedule_value(config.return_flow_fraction_schedule
        .ok_or(Error::Unavailable("actual Config returnFlowFracSched pointer"))?)?;
    if let Some(flags)=config.fixed_return_flow.as_mut(){flags.fill(false);}
    *final_flow=0.0;config.total_available_air_loop_outdoor_air=0.0;
    for index in 0..count {
        let return_node=*row(&config.return_node_numbers,index)?;
        if return_node==0 {continue;}
        let mut return_flow;
        // Actual Node reference is obtained before these config indices.
        let _return_node_data=node(nodes,return_node)?;
        let inlet=*row(&config.return_node_inlet_numbers,index)?;
        let mut distribution_unit=0;
        if inlet>0 {distribution_unit=*row(&config.inlet_node_air_distribution_unit_numbers,at(inlet)?)?;}
        let air_loop=*row(&config.return_node_air_loop_numbers,index)?;
        let air_loop_return_fraction=1.0;
        if air_loop>0 {
            let _distribution_unit=distribution_unit;
            return Err(Error::Unavailable("actual calcReturnFlows same AirLoopFlow2381"));
        }else{return_flow=0.0;}
        if index==0 {
            if doing_sizing&&count==1 {return_flow=*expected;}
            else if !doing_sizing {
                if config.number_return_flow_basis_nodes>0 {
                    let mut basis_flow=0.0;
                    for basis in 0..config.number_return_flow_basis_nodes {
                        basis_flow+=node(nodes,*row(&config.return_flow_basis_node_numbers,basis)?)?.mass_flow_rate;
                    }
                    return_flow=max(0.0,basis_flow*fraction);
                    *row_mut(&mut config.fixed_return_flow,index)?=true;
                }else if count==1&&!*row(&config.fixed_return_flow,index)? {
                    return_flow=max(0.0,*expected*fraction*air_loop_return_fraction);
                }
            }
        }
        total+=return_flow;
        node_mut(nodes,return_node)?.mass_flow_rate=return_flow;
        node_mut(nodes,return_node)?.mass_flow_rate_min_available=0.0;
        if !*row(&config.fixed_return_flow,index)? {variable_total+=return_flow;}
    }
    if flow_adjustment!=4 { // Native NoAdjustReturnAndMixing underlying4.
        *expected=fraction* *expected;
        let mut zone_total=0.0;let mut return_flow=0.0;
        for index in 0..count {
            let return_node=*row(&config.return_node_numbers,index)?;
            if return_node>0 {
                if count==1 {return_flow=*expected;}
                else if *expected>0.0 {
                    let adjustment=node(nodes,return_node)?.mass_flow_rate/ *expected;
                    return_flow=adjustment* *expected;
                }else{return_flow=0.0;}
            }
            zone_total+=return_flow;
        }
        if zone_total>0.0 {
            for index in 0..count {
                let return_node=*row(&config.return_node_numbers,index)?;
                if return_node>0 {
                    if count==1 {node_mut(nodes,return_node)?.mass_flow_rate=*expected;*final_flow=*expected;}
                    else{
                        let adjustment=*expected/zone_total;
                        let current=node(nodes,return_node)?.mass_flow_rate;
                        let new_return=current*adjustment;
                        node_mut(nodes,return_node)?.mass_flow_rate=new_return;
                        *final_flow+=new_return;
                    }
                }
            }
        }else{*final_flow=*expected;}
    }else if total> *expected&&variable_total>0.0 {
        let adjustment=1.0-((total- *expected)/variable_total);
        for index in 0..count {
            let return_node=*row(&config.return_node_numbers,index)?;
            // Native reads current flow before the positive-node predicate.
            let current=node(nodes,return_node)?.mass_flow_rate;
            if return_node>0 {
                if !*row(&config.fixed_return_flow,index)? {
                    let new_return=current*adjustment;
                    *final_flow+=new_return;
                    node_mut(nodes,return_node)?.mass_flow_rate=new_return;
                }else{*final_flow+=current;}
            }
        }
    }else{*final_flow=total;}
    Ok(())
}

/// Complete finite no-primary/no-mixing operation, Native4933-5283.
pub(super) fn mass_balance(first_hvac_iteration:bool,o:&mut SourceHvacOwners<'_, '_>)->Result<(),Error>{
    o.hvac.zone_mass_balance_hvac_resim=false;
    let mut iteration=0;
    let mut building_mixing: f64=0.0;
    let mut building_return: f64=0.0;
    if let Some(units)=o.equipment.air_distribution_units.as_deref(){for unit in units {match *unit {}}}
    if o.hvac.number_of_primary_air_systems>0 {
        return Err(Error::Unavailable("actual CalcZoneMassBalance primary isAllOA4972"));
    }
    loop {
        if o.control.enforce_zone_mass_balance {
            for number in 1..=o.globals.num_of_zones {
                if !row(&o.equipment.zone_config,at(number)?)?.is_controlled {continue;}
                *row_mut(&mut o.fan.zone_infiltration_flag,at(number)?)?=false;
                let mass=row_mut(&mut o.controller.simple_air.mass_conservation,at(number)?)?;
                mass.include_infiltration_to_zone_mass_balance=0;
                mass.return_mass_flow_rate=0.0;
                row_mut(&mut o.equipment.zone_config,at(number)?)?.excess_zone_exhaust=0.0;
                if o.space_controls.current {
                    return Err(Error::Unavailable("actual CalcZoneMassBalance controlled Space ExcessZoneExh5002"));
                }
            }
        }
        let previous_mixing=building_mixing;building_mixing=0.0;
        let previous_return=building_return;building_return=0.0;
        for source_number in 1..=o.globals.num_of_zones {
            let number=if o.control.enforce_zone_mass_balance {
                row(&o.fan.zone_reorder,at(source_number)?)?.ok_or(Error::Unavailable("actual ZoneReOrder5017 initialized cell"))?
            }else{source_number};
            let index=at(number)?;
            if !row(&o.equipment.zone_config,index)?.is_controlled {continue;}
            // Native obtains the real MassConservation row before zeroing Config.
            let _mass=row(&o.controller.simple_air.mass_conservation,index)?;
            let total_exhaust;
            row_mut(&mut o.equipment.zone_config,index)?.total_exhaust_air_mass_flow_rate=0.0;
            let mixing_net=0.0;
            let mut zone_return=0.0;
            {
                let config=row_mut(&mut o.equipment.zone_config,index)?;
                set_total_inlet_flows(config,&mut o.equipment.nodes.physical)?;
            }
            let inlet=row(&o.equipment.zone_config,index)?.total_inlet_air_mass_flow_rate;
            if o.space_controls.current {
                return Err(Error::Unavailable("actual CalcZoneMassBalance Space setTotalInletFlows5038"));
            }
            let exhaust_count=row(&o.equipment.zone_config,index)?.exhaust_nodes;
            for _exhaust in 0..exhaust_count {
                // The current AFN owner has no retained NumOfExhFan scalar.
                // Zero actual Config.NumExhaustNodes naturally executes no read.
                return Err(Error::Unavailable("actual CalcZoneMassBalance AFN AirflowNetworkNumOfExhFan5049"));
            }
            total_exhaust=row(&o.equipment.zone_config,index)?.total_exhaust_air_mass_flow_rate;
            if *row(&o.fan.zone_mass_balance_flag,index)? {
                let count=row(&o.equipment.zone_config,index)?.return_nodes;
                for return_index in 0..count {
                    let return_node=*row(&row(&o.equipment.zone_config,index)?.return_node_numbers,return_index)?;
                    if return_node>0 {zone_return+=node(&o.equipment.nodes.physical,return_node)?.mass_flow_rate;}
                }
                let adjustment=o.controller.simple_air.mass_flow_control.zone_flow_adjustment;
                let mixing_flow=if iteration==0||adjustment==1||adjustment==3 {
                    row(&o.air.zones,index)?.air.mixing_mass_flow_zone
                }else{
                    max(0.0,zone_return+total_exhaust-inlet+
                        row(&o.controller.simple_air.mass_conservation,index)?.mixing_source_mass_flow_rate)
                };
                let _same_mixing_flow=mixing_flow;
                return Err(Error::Unavailable("actual CalcZoneMixingFlowRateOfReceivingZone5075"));
            }
            let mut expected=inlet+mixing_net-(total_exhaust-row(&o.equipment.zone_config,index)?.zone_exhaust_balanced);
            if !o.control.enforce_zone_mass_balance {
                if expected<0.0 {row_mut(&mut o.equipment.zone_config,index)?.excess_zone_exhaust= -expected;expected=0.0;}
                else{row_mut(&mut o.equipment.zone_config,index)?.excess_zone_exhaust=0.0;}
            }else{
                row_mut(&mut o.equipment.zone_config,index)?.excess_zone_exhaust=0.0;
                expected=max(0.0,expected);
            }
            let mut final_flow=0.0;
            {
                let config=row_mut(&mut o.equipment.zone_config,index)?;
                calculate_return_flows(config,&mut o.equipment.nodes.physical,o.schedules,o.globals.doing_sizing,
                    o.controller.simple_air.mass_flow_control.zone_flow_adjustment,&mut expected,&mut final_flow)?;
            }
            if o.control.enforce_zone_mass_balance {
                let mass=row_mut(&mut o.controller.simple_air.mass_conservation,index)?;
                mass.inlet_mass_flow_rate=inlet;mass.exhaust_mass_flow_rate=total_exhaust;
                return Err(Error::Unavailable("actual CalcZoneMassBalance positive return/mixing adjustment and infiltration operation5107"));
            }
            building_mixing+=row(&o.controller.simple_air.mass_conservation,index)?.mixing_mass_flow_rate;
            building_return+=row(&o.controller.simple_air.mass_conservation,index)?.return_mass_flow_rate;
            let count=row(&o.equipment.zone_config,index)?.return_nodes;
            for return_index in 0..count {
                let _return_node=*row(&row(&o.equipment.zone_config,index)?.return_node_numbers,return_index)?;
                let air_loop=*row(&row(&o.equipment.zone_config,index)?.return_node_air_loop_numbers,return_index)?;
                if air_loop>0 {return Err(Error::Unavailable("actual CalcZoneMassBalance accumulate same AirLoopFlow5171"));}
            }
        }
        // Real NumPrimaryAirSys is still the same owner; positive operation
        // would have stopped in the earlier first primary-system traversal.
        for number in 1..=o.globals.num_of_zones {
            let index=at(number)?;
            let _air=row(&o.air.zones,index)?;
            if !row(&o.equipment.zone_config,index)?.is_controlled {continue;}
            let count=row(&o.equipment.zone_config,index)?.return_nodes;
            let mut total_return=0.0;
            for return_index in 0..count {
                let return_node=*row(&row(&o.equipment.zone_config,index)?.return_node_numbers,return_index)?;
                let air_loop=*row(&row(&o.equipment.zone_config,index)?.return_node_air_loop_numbers,return_index)?;
                if return_node>0 {
                    if air_loop>0 {return Err(Error::Unavailable("actual CalcZoneMassBalance return adjustment AirLoopFlow5208"));}
                    total_return+=node(&o.equipment.nodes.physical,return_node)?.mass_flow_rate;
                }
            }
            if !o.control.enforce_zone_mass_balance && !o.globals.doing_sizing &&
                !o.globals.doing_hvac_sizing_simulations && !o.clock.warmup_flag && !first_hvac_iteration {
                let config=row(&o.equipment.zone_config,index)?;
                if !config.flow_error {
                    let unbalanced_exhaust=config.total_exhaust_air_mass_flow_rate-config.zone_exhaust_balanced;
                    let unbalanced_system=unbalanced_exhaust+total_return-config.total_inlet_air_mass_flow_rate;
                    if unbalanced_system>0.001 {
                        let air=&row(&o.air.zones,index)?.air;
                        let incoming=air.oamfl+air.vamfl+air.mixing_mass_flow_zone;
                        let unbalanced=max(0.0,unbalanced_system-incoming);
                        if unbalanced>0.001 {
                            let _temperature=node(&o.equipment.nodes.physical,config.zone_node)?.temperature;
                            let _humidity=node(&o.equipment.nodes.physical,config.zone_node)?.humidity_ratio;
                            return Err(Error::Unavailable("actual CalcZoneMassBalance diagnostic-aware PsyRhoAirFnPbTdbW5225"));
                        }
                    }
                }
            }
        }
        if iteration>0 {
            let residual=(building_mixing-previous_mixing).abs()+(building_return-previous_return).abs();
            if residual<0.000010 {o.hvac.zone_mass_balance_hvac_resim=false;break;}
            o.hvac.zone_mass_balance_hvac_resim=true;
        }
        if !o.control.enforce_zone_mass_balance {break;}
        iteration+=1;if iteration>=25 {break;}
    }
    Ok(())
}

/// Native5342-5543, using the same true caller phase and mutable owners.
pub(super) fn leaving_conditions(first_hvac_iteration:bool,o:&mut SourceHvacOwners<'_, '_>)->Result<(),Error>{
    if o.space_controls.simulation&&!o.globals.doing_sizing {
        for mixer in &o.equipment.zone_return_mixers {match *mixer {}}
    }
    for number in 1..=o.globals.num_of_zones {
        let index=at(number)?;
        if !row(&o.equipment.zone_config,index)?.is_controlled {continue;}
        if row(&o.equipment.zone_config,index)?.return_nodes==0 {continue;}
        let zone_node=row(&o.equipment.zone_config,index)?.zone_node;
        let zone=row(&o.zones.zones,index)?;
        let multiplier=f64::from(zone.multiplier.checked_mul(zone.list_multiplier)
            .ok_or(Error::Unavailable("actual leaving Zone multiplier product domain"))?);
        let _air=row(&o.air.zones,index)?;
        let count=row(&o.equipment.zone_config,index)?.return_nodes;
        for return_index in 0..count {
            let return_node=*row(&row(&o.equipment.zone_config,index)?.return_node_numbers,return_index)?;
            let return_exhaust=*row(&row(&o.equipment.zone_config,index)?.return_node_exhaust_node_numbers,return_index)?;
            let return_node_integer=i32::try_from(return_node).map_err(|_|Error::Unavailable("actual ReturnNode integer domain"))?;
            let return_gain=o.gains.source_zone_return_air_convective_gain(o.zones,number,return_node_integer)?;
            let mut mass_flow=node(&o.equipment.nodes.physical,return_node)?.mass_flow_rate/multiplier;
            if return_exhaust>0&&node(&o.equipment.nodes.physical,node_number(return_exhaust)?)?.mass_flow_rate>0.0 {
                mass_flow+=node(&o.equipment.nodes.physical,node_number(return_exhaust)?)?.mass_flow_rate;
            }
            let temperature_zone;
            let mut temperature_return;
            if o.space_controls.simulation&&!o.globals.doing_sizing &&
                *row(&row(&o.equipment.zone_config,index)?.return_node_space_mixer_indices,return_index)?> -1 {
                temperature_zone=node(&o.equipment.nodes.physical,return_node)?.temperature;
                temperature_return=temperature_zone;
            }else if o.controller.subsystems.room_air.air_pattern_zone_info.is_some(){
                return Err(Error::Unavailable("actual leaving allocated AirPatternZoneInfo.IsUsed5399"));
            }else{
                temperature_zone=node(&o.equipment.nodes.physical,zone_node)?.temperature;
                temperature_return=temperature_zone;
            }
            let window_flow=0.0;
            let mut window_temperature=0.0;
            let window_flow_temperature=0.0;
            if row(&o.zones.zones,index)?.has_airflow_window_return {
                return Err(Error::Unavailable("actual leaving positive airflow-window return Surface operation5413"));
            }
            if window_flow>0.0 {window_temperature=window_flow_temperature/window_flow;}
            if !row(&o.zones.zones,index)?.no_heat_to_return_air {
                let specific_heat=cp(node(&o.equipment.nodes.physical,zone_node)?.humidity_ratio);
                if mass_flow>0.0 {
                    if window_flow>0.0 {
                        if mass_flow>=window_flow {
                            temperature_return=(window_flow_temperature+(mass_flow-window_flow)*temperature_zone)/mass_flow;
                        }else{
                            temperature_return=window_temperature;
                            row_mut(&mut o.air.zones,index)?.air.sys_dep_zone_loads+=
                                (window_flow-mass_flow)*specific_heat*(window_temperature-temperature_zone);
                        }
                    }
                    temperature_return+=return_gain/(mass_flow*specific_heat);
                    if temperature_return>60.0 {
                        node_mut(&mut o.equipment.nodes.physical,return_node)?.temperature=60.0;
                        if !o.globals.zone_sizing_calculation {
                            row_mut(&mut o.air.zones,index)?.air.sys_dep_zone_loads+=specific_heat*mass_flow*(temperature_return-60.0);
                        }
                    }else if temperature_return< -30.0 {
                        node_mut(&mut o.equipment.nodes.physical,return_node)?.temperature= -30.0;
                        if !o.globals.zone_sizing_calculation {
                            row_mut(&mut o.air.zones,index)?.air.sys_dep_zone_loads+=specific_heat*mass_flow*(temperature_return- -30.0);
                        }
                    }else{node_mut(&mut o.equipment.nodes.physical,return_node)?.temperature=temperature_return;}
                    if return_exhaust>0&&node(&o.equipment.nodes.physical,node_number(return_exhaust)?)?.mass_flow_rate>0.0&&return_gain>0.0 {
                        if *row(&row(&o.equipment.zone_config,index)?.shared_exhaust_nodes,return_index)?!=3 {
                            node_mut(&mut o.equipment.nodes.physical,node_number(return_exhaust)?)?.temperature=temperature_return;
                        }else{
                            node_mut(&mut o.equipment.nodes.physical,node_number(return_exhaust)?)?.temperature+=return_gain/(mass_flow*specific_heat);
                        }
                    }
                }else{
                    if window_flow>0.0 {
                        row_mut(&mut o.air.zones,index)?.air.sys_dep_zone_loads+=window_flow*specific_heat*(window_temperature-temperature_zone);
                    }
                    if return_gain>0.0 {row_mut(&mut o.air.zones,index)?.air.sys_dep_zone_loads+=return_gain;}
                    let temperature=node(&o.equipment.nodes.physical,zone_node)?.temperature;
                    node_mut(&mut o.equipment.nodes.physical,return_node)?.temperature=temperature;
                }
            }else{
                let temperature=node(&o.equipment.nodes.physical,zone_node)?.temperature;
                node_mut(&mut o.equipment.nodes.physical,return_node)?.temperature=temperature;
            }
            let pressure=node(&o.equipment.nodes.physical,zone_node)?.pressure;
            node_mut(&mut o.equipment.nodes.physical,return_node)?.pressure=pressure;
            if !row(&o.zones.zones,index)?.no_heat_to_return_air {
                if mass_flow>0.0 {
                    let latent_gain=o.gains.source_zone_return_air_latent_gain(o.zones,number,return_node_integer)?;
                    let vaporization=heat_of_vaporization(node(&o.equipment.nodes.physical,zone_node)?.humidity_ratio,
                        node(&o.equipment.nodes.physical,return_node)?.temperature);
                    let humidity=node(&o.equipment.nodes.physical,zone_node)?.humidity_ratio+latent_gain/(vaporization*mass_flow);
                    node_mut(&mut o.equipment.nodes.physical,return_node)?.humidity_ratio=humidity;
                }else{
                    let humidity=node(&o.equipment.nodes.physical,zone_node)?.humidity_ratio;
                    node_mut(&mut o.equipment.nodes.physical,return_node)?.humidity_ratio=humidity;
                    let credit=row_mut(&mut o.gains.allocations.refrigeration_credit,index)?;
                    credit[1]+=credit[3];
                    let latent_gain=o.gains.source_zone_return_air_latent_gain(o.zones,number,return_node_integer)?;
                    row_mut(&mut o.air.zones,index)?.air.latent_gain+=latent_gain;
                }
            }else{
                let humidity=node(&o.equipment.nodes.physical,zone_node)?.humidity_ratio;
                node_mut(&mut o.equipment.nodes.physical,return_node)?.humidity_ratio=humidity;
                let credit=row_mut(&mut o.gains.allocations.refrigeration_credit,index)?;
                credit[1]+=credit[3];
                let latent_gain=o.gains.source_zone_return_air_latent_gain(o.zones,number,return_node_integer)?;
                row_mut(&mut o.air.zones,index)?.air.latent_gain+=latent_gain;
            }
            let h=enthalpy(node(&o.equipment.nodes.physical,return_node)?.temperature,
                node(&o.equipment.nodes.physical,return_node)?.humidity_ratio);
            node_mut(&mut o.equipment.nodes.physical,return_node)?.enthalpy=h;
            if o.control.carbon_dioxide_simulation {
                let carbon_dioxide=node(&o.equipment.nodes.physical,zone_node)?.carbon_dioxide;
                node_mut(&mut o.equipment.nodes.physical,return_node)?.carbon_dioxide=carbon_dioxide;
            }
            if o.control.generic_contaminant_simulation {
                let contaminant=node(&o.equipment.nodes.physical,zone_node)?.generic_contaminant;
                node_mut(&mut o.equipment.nodes.physical,return_node)?.generic_contaminant=contaminant;
            }
        }
        output_required::initialize(o,number,first_hvac_iteration)?;
    }
    Ok(())
}
