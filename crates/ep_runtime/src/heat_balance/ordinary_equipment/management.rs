//! Genuine ZoneEquipmentManager142–320/3538–4194/4933–5284/5342–5570 ordinary path.
pub(crate) mod availability;
mod modules;
mod output_required;
use crate::heat_balance::ordinary_hvac_manager::{SourceHvacManager, SourceHvacOwners};
use crate::heat_balance::selected_heat_balance_input::HeatBalanceInputError as Error;
use crate::heat_balance::selected_sizing::SourceZoneEquipmentSizing;
pub(crate) use modules::SourceEquipmentManagementModules;
fn at(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "actual ZoneEquipment positive Source ordinal",
        ))
}
impl SourceHvacManager {
    pub(crate) fn manage_source_zone_equipment(
        &mut self,
        first_hvac_iteration: bool,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        self.initialize_source_zone_equipment(first_hvac_iteration, o)?;
        if o.globals.zone_sizing_calculation {
            return Err(Error::Unavailable("actual SizeZoneEquipment156"));
        }
        self.simulate_source_zone_equipment(first_hvac_iteration, o)?;
        o.equipment.equipment_simulated_once = true;
        self.update_source_zone_equipment(o)?;
        o.hvac.sim_zone_equipment = false;
        Ok(())
    }
    fn initialize_source_zone_equipment(
        &mut self,
        first_hvac_iteration: bool,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.equipment.init_equipment_one_time_flag {
            o.equipment.init_equipment_one_time_flag = false;
            let zones = usize::try_from(o.globals.num_of_zones)
                .map_err(|_| Error::Unavailable("actual ZoneEqSizing NumOfZones211"))?;
            o.sizing.zone_equipment_sizing = Some(vec![SourceZoneEquipmentSizing::default(); zones]);
            for number in 1..=o.globals.num_of_zones {
                let config = o.equipment.zone_config.as_deref()
                    .and_then(|rows| rows.get(at(number).ok()?))
                    .ok_or(Error::Unavailable("actual ZoneEquipConfig214"))?;
                if !config.is_controlled {
                    continue;
                }
                if config.equipment_list_index == 0 {
                    continue;
                }
                let count = o.equipment.equipment_lists.as_deref()
                    .and_then(|rows| config.equipment_list_index.checked_sub(1).and_then(|at| rows.get(at)))
                    .ok_or(Error::Unavailable("actual controlled ZoneEquipList221"))?
                    .number_of_equipment_types;
                let native_count = i32::try_from(count)
                    .map_err(|_| Error::Unavailable("actual ZoneEquipCount221 integer domain"))?;
                let sensible = o.controller.demand.zones.as_mut()
                    .and_then(|rows| rows.get_mut(at(number).ok()?))
                    .ok_or(Error::Unavailable("actual ZoneSysEnergyDemand222"))?;
                sensible.base.num_zone_equipment = native_count;
                sensible.sequenced_output_required = Some(vec![0.0; count]);
                sensible.sequenced_output_to_heating_sp = Some(vec![0.0; count]);
                sensible.sequenced_output_to_cooling_sp = Some(vec![0.0; count]);
                let moisture = o.controller.demand.zone_moisture.as_mut()
                    .and_then(|rows| rows.get_mut(at(number).ok()?))
                    .ok_or(Error::Unavailable("actual ZoneSysMoistureDemand227"))?;
                moisture.base.num_zone_equipment = native_count;
                moisture.sequenced_output_required = Some(vec![0.0; count]);
                moisture.sequenced_output_to_humid_sp = Some(vec![0.0; count]);
                moisture.sequenced_output_to_dehumid_sp = Some(vec![0.0; count]);
                let sizing = o.sizing.zone_equipment_sizing.as_mut()
                    .and_then(|rows| rows.get_mut(at(number).ok()?))
                    .ok_or(Error::Unavailable("actual ZoneEqSizing232"))?;
                // Native232 allocation followed by explicit233 whole-array =0.
                // Demand EPVector allocations separately follow their genuine zero-fill API.
                sizing.sizing__method = Some(vec![0; 35]); // HVAC::NumOfSizingTypes
                if o.space_controls.simulation || o.space_controls.sizing {
                    let zone = o.zones.zones.as_deref()
                        .and_then(|rows| rows.get(at(number).ok()?))
                        .ok_or(Error::Unavailable("actual controlled Zone spaceIndexes235"))?;
                    for &space in &zone.space_indexes {
                        let sensible = o.controller.demand.spaces.as_mut()
                            .and_then(|rows| rows.get_mut(at(space).ok()?))
                            .ok_or(Error::Unavailable("actual spaceSysEnergyDemand236"))?;
                        sensible.base.num_zone_equipment = native_count;
                        sensible.sequenced_output_required = Some(vec![0.0; count]);
                        sensible.sequenced_output_to_heating_sp = Some(vec![0.0; count]);
                        sensible.sequenced_output_to_cooling_sp = Some(vec![0.0; count]);
                        let moisture = o.controller.demand.space_moisture.as_mut()
                            .and_then(|rows| rows.get_mut(at(space).ok()?))
                            .ok_or(Error::Unavailable("actual spaceSysMoistureDemand241"))?;
                        moisture.base.num_zone_equipment = native_count;
                        moisture.sequenced_output_required = Some(vec![0.0; count]);
                        moisture.sequenced_output_to_humid_sp = Some(vec![0.0; count]);
                        moisture.sequenced_output_to_dehumid_sp = Some(vec![0.0; count]);
                    }
                }
            }
        }
        if o.equipment.init_equipment_environment_flag && o.clock.begin_envrn_flag {
            if let Some(rows) = o.equipment.equipment_availability.as_mut() {
                rows.fill(0);
            }
            if let Some(rows) = self.ancillary.availability.zone_components.as_mut() {
                for number in 1..=14 {
                    let row = rows.get_mut(number - 1)
                        .ok_or(Error::Unavailable("actual same ZoneComp type257"))?;
                    if let Some(managers) = row.managers.as_mut() {
                        for number in 1..=row.total_components {
                            let manager = managers.get_mut(at(number)?)
                                .ok_or(Error::Unavailable("actual ZoneCompAvailMgrs260"))?;
                            manager.status = 0;
                            manager.start_time = 0;
                            manager.stop_time = 0;
                        }
                    }
                }
            }
            o.equipment.begin_source_equipment_environment(false, o.environment, o.control, o.contaminants)?;
            if o.space_controls.simulation {
                o.equipment.begin_source_equipment_environment(true, o.environment, o.control, o.contaminants)?;
            }
            o.equipment.init_equipment_environment_flag = false;
        }
        if !o.clock.begin_envrn_flag {
            o.equipment.init_equipment_environment_flag = true;
        }
        o.equipment.initialize_source_equipment_timestep(false, first_hvac_iteration, o.control)?;
        if o.space_controls.simulation {
            o.equipment.initialize_source_equipment_timestep(true, first_hvac_iteration, o.control)?;
        }
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable("actual InitZoneEquipment AirLoopFlow308"));
        }
        Ok(())
    }
    fn simulate_source_zone_equipment(
        &mut self,
        first_hvac_iteration: bool,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        let mut _supply_path_inlet_changed = false;
        let _system_output_provided = 0.0;
        let _latent_output_provided = 0.0;
        let _air_system_output = 0.0;
        let _non_air_system_output = 0.0;
        let mut _first_call = true;
        let _error_flag = false;
        if o.equipment.number_of_supply_paths > 0 {
            return Err(Error::Unavailable(
                "actual SimZoneEquipment SupplyAirPath3578",
            ));
        }
        _first_call = false;
        if o.control.enforce_zone_mass_balance {
            if !first_hvac_iteration {
                // Passed native arguments must be supplied, even where later zero loops do not read them.
                let control = &o.controller.simple_air.mass_flow_control;
                if control.adjust_zone_mixing_flow || control.adjust_zone_infiltration_flow {
                    return Err(Error::Unavailable(
                        "actual CalcAirFlowSimple explicit adjustment arguments3636",
                    ));
                }
            }
            self.calculate_source_simple_airflow(Some(0), o)?;
        }
        if o.space_controls.simulation && !o.globals.doing_sizing {
            for row in &o.equipment.zone_equipment_mixers {
                match *row {}
            }
        }
        for number in 1..=o.globals.num_of_zones {
            if !*o
                .equipment
                .source_zone_equipment_config(number)?
                .is_controlled()
            {
                continue;
            }
            let air = &mut o
                .air
                .zones
                .as_mut()
                .and_then(|v| v.get_mut(at(number).ok()?))
                .ok_or(Error::Unavailable("actual controlled Zone AirHB3660"))?
                .air;
            air.non_air_system_response = 0.0;
            air.sys_dep_zone_loads = 0.0;
            {
                let config = o.equipment.zone_config.as_mut()
                    .and_then(|rows| rows.get_mut(at(number).ok()?))
                    .ok_or(Error::Unavailable("actual controlled ZoneEquipConfig3664"))?;
                config.zone_exhaust = 0.0;
                config.zone_exhaust_balanced = 0.0;
                config.plenum_mass_flow = 0.0;
            }
            o.sizing.current_zone_equipment_number = number;
            if o.space_controls.simulation && !o.globals.doing_sizing {
                let count = o.zones.zones.as_ref().and_then(|rows| rows.get(at(number).ok()?))
                    .ok_or(Error::Unavailable("actual controlled Zone spaceIndexes3670"))?.space_indexes.len();
                for i in 0..count {
                    let space = o.zones.zones.as_ref().and_then(|rows| rows.get(at(number).ok()?))
                        .and_then(|zone| zone.space_indexes.get(i)).copied()
                        .ok_or(Error::Unavailable("actual controlled Space ordinal3670"))?;
                    let air = &mut o.air.spaces.as_mut().and_then(|rows| rows.get_mut(at(space).ok()?))
                        .ok_or(Error::Unavailable("actual controlled Space AirHB3671"))?.air;
                    air.non_air_system_response = 0.0;
                    air.sys_dep_zone_loads = 0.0;
                    let config = o.equipment.space_config.as_mut().and_then(|rows| rows.get_mut(at(space).ok()?))
                        .ok_or(Error::Unavailable("actual spaceEquipConfig3674"))?;
                    if !config.is_controlled { continue; }
                    config.zone_exhaust = 0.0;
                    config.zone_exhaust_balanced = 0.0;
                    config.plenum_mass_flow = 0.0;
                }
            }
            output_required::initialize(o, number, first_hvac_iteration)?;
            let count = o.equipment.equipment_lists.as_ref().and_then(|rows| rows.get(at(number).ok()?))
                .ok_or(Error::Unavailable("actual SimZoneEquipment ZoneEquipList3688"))?.number_of_equipment_types;
            for equipment_number in 1..=count {
                o.hvac.turn_fans_on = false;
                o.hvac.turn_fans_off = false;
                o.hvac.unbalanced_exhaust_mass_flow = 0.0;
                o.hvac.balanced_exhaust_mass_flow = 0.0;
                o.hvac.plenum_induced_mass_flow = 0.0;
                let equipment_pointer = o.equipment.priority_simulation_order.as_ref()
                    .and_then(|rows| rows.get(equipment_number - 1))
                    .ok_or(Error::Unavailable("actual PrioritySimOrder EquipPtr3697"))?.equipment_pointer;
                let _system_output_provided = 0.0;
                let _latent_output_provided = 0.0;
                let _non_air_system_output = 0.0;
                o.sizing.data_cool_coil_capacity = 0.0;
                if o.equipment.first_pass_zone_equipment_flag {
                    let sizing = o.sizing.zone_equipment_sizing.as_mut()
                        .and_then(|rows| rows.get_mut(at(number).ok()?))
                        .ok_or(Error::Unavailable("actual first-pass ZoneEqSizing3704"))?;
                    sizing.air__vol__flow = 0.0;
                    sizing.max_hw_vol__flow = 0.0;
                    sizing.max_cw_vol__flow = 0.0;
                    sizing.oa_vol__flow = 0.0;
                    sizing.des__cooling__load = 0.0;
                    sizing.des__heating__load = 0.0;
                    sizing.cooling__air__vol__flow = 0.0;
                    sizing.heating__air__vol__flow = 0.0;
                    sizing.system__air__vol__flow = 0.0;
                    sizing.air__flow = false;
                    sizing.cooling__air__flow = false;
                    sizing.heating__air__flow = false;
                    sizing.system__air__flow = false;
                    sizing.capacity = false;
                    sizing.cooling__capacity = false;
                    sizing.heating__capacity = false;
                    sizing.system__capacity = false;
                    sizing.design__size__from__parent = false;
                }
                let equipment_type = o.equipment.priority_simulation_order.as_ref()
                    .and_then(|rows| rows.get(equipment_number - 1))
                    .ok_or(Error::Unavailable("actual PrioritySimOrder equipType3725"))?.equipment_type;
                let list = o.equipment.equipment_lists.as_ref()
                    .and_then(|rows| rows.get(at(o.sizing.current_zone_equipment_number).ok()?))
                    .ok_or(Error::Unavailable("actual current ZoneEquipList3727"))?;
                let component_number = list.equipment_indices.as_ref()
                    .and_then(|rows| rows.get(at(equipment_pointer).ok()?)).copied()
                    .ok_or(Error::Unavailable("actual ZoneEquipList EquipIndex3729"))?;
                let mut valid_availability_component = false;
                if equipment_type <= 14 { valid_availability_component = true; }
                if component_number > 0 && valid_availability_component {
                    // Genuine positive cached CompIndex enters this call on later
                    // iterations; no perpetual first-call/zero-count shortcut.
                    return Err(Error::Unavailable("actual GetZoneEqAvailabilityManager3739"));
                }
                if o.space_controls.simulation && !o.globals.doing_sizing {
                    let splitter = list.equipment_splitter_indices.as_ref()
                        .and_then(|rows| rows.get(at(equipment_pointer).ok()?)).copied()
                        .ok_or(Error::Unavailable("actual zoneEquipSplitterIndex3751"))?;
                    if splitter > -1 {
                        return Err(Error::Unavailable("actual zoneEquipSplitter adjustLoads3753"));
                    }
                }
                match equipment_type {
                    12 => {
                        let _name = &o.equipment.priority_simulation_order.as_ref()
                            .and_then(|rows| rows.get(equipment_number - 1))
                            .ok_or(Error::Unavailable("actual PurchasedAir priority name3872"))?.equipment_name;
                        let _same_component_index = o.equipment.equipment_lists.as_mut()
                            .and_then(|rows| rows.get_mut(at(o.sizing.current_zone_equipment_number).ok()?))
                            .and_then(|list| list.equipment_indices.as_mut())
                            .and_then(|rows| rows.get_mut(at(equipment_pointer).ok()?))
                            .ok_or(Error::Unavailable("actual PurchasedAir same EquipIndex3877"))?;
                        return Err(Error::Unavailable("actual PurchasedAirManager SimPurchasedAir3871"));
                    }
                    // Current real input admits only PurchasedAir. Any later
                    // represented positive type must enter its actual callee,
                    // never produce a fabricated zero equipment response.
                    _ => return Err(Error::Unavailable("actual SimZoneEquipment positive equipment callee switch3747")),
                }
            }
        }
        if o.space_controls.simulation && !o.globals.doing_sizing {
            for row in &o.equipment.zone_equipment_mixers {
                match *row {}
            }
        }
        o.sizing.current_zone_equipment_number = 0;
        o.equipment.first_pass_zone_equipment_flag = false;
        if o.equipment.number_of_supply_paths > 0 {
            return Err(Error::Unavailable("actual reverse SupplyAirPath4141"));
        }
        o.equipment
            .management_modules
            .simulate_source_exhaust_controls(o.input, o.equipment.number_zone_exhaust_controls)?;
        o.equipment
            .management_modules
            .simulate_source_exhaust_system(
                o.input,
                o.equipment.number_exhaust_air_systems,
                o.equipment.number_zone_exhaust_controls,
            )?;
        self.calculate_source_zone_mass_balance(first_hvac_iteration, o)?;
        self.calculate_source_zone_leaving_conditions(o)?;
        modules::simulate_source_duct_loss_default(o)?;
        o.equipment
            .management_modules
            .simulate_source_return_air_paths(
                &mut o.equipment.number_of_return_paths,
                &o.equipment.return_paths,
                o.input,
                o.shortcut,
            )?;
        Ok(())
    }
    fn update_source_zone_equipment(&self, o: &SourceHvacOwners<'_, '_>) -> Result<(), Error> {
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual UpdateZoneEquipment AirToZoneNodeInfo5560",
            ));
        }
        Ok(())
    }
    fn calculate_source_zone_mass_balance(
        &self,
        _first_hvac_iteration: bool,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        o.hvac.zone_mass_balance_hvac_resim = false;
        let mut iteration = 0;
        let mut building_zone_mixing_flow: f64 = 0.0;
        let mut building_zone_return_flow: f64 = 0.0;
        if let Some(units) = o.equipment.air_distribution_units.as_deref() {
            for unit in units {
                match *unit {}
            }
        }
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual CalcZoneMassBalance primary isAllOA4972",
            ));
        }
        loop {
            if o.control.enforce_zone_mass_balance {
                for number in 1..=o.globals.num_of_zones {
                    if !*o
                        .equipment
                        .source_zone_equipment_config(number)?
                        .is_controlled()
                    {
                        continue;
                    }
                    return Err(Error::Unavailable(
                        "actual controlled ZoneInfiltrationFlag4992",
                    ));
                }
            }
            let building_zone_mixing_flow_old = building_zone_mixing_flow;
            building_zone_mixing_flow = 0.0;
            let building_zone_return_flow_old = building_zone_return_flow;
            building_zone_return_flow = 0.0;
            for source_number in 1..=o.globals.num_of_zones {
                let number = if o.control.enforce_zone_mass_balance {
                    o.fan
                        .zone_reorder
                        .as_deref()
                        .and_then(|v| v.get(at(source_number).ok()?))
                        .copied()
                        .flatten()
                        .ok_or(Error::Unavailable("actual ZoneReOrder5017"))?
                } else {
                    source_number
                };
                if !*o
                    .equipment
                    .source_zone_equipment_config(number)?
                    .is_controlled()
                {
                    continue;
                }
                return Err(Error::Unavailable("actual controlled MassConservation5022"));
            }
            for number in 1..=o.globals.num_of_zones {
                let config = o.equipment.source_zone_equipment_config(number)?;
                let _zone_air = o
                    .air
                    .zones
                    .as_deref()
                    .and_then(|v| v.get(at(number).ok()?))
                    .ok_or(Error::Unavailable("actual same Zone AirHB5195"))?;
                if !*config.is_controlled() {
                    continue;
                }
                return Err(Error::Unavailable("actual controlled NumReturnNodes5199"));
            }
            if iteration > 0 {
                let total_residual = (building_zone_mixing_flow - building_zone_mixing_flow_old)
                    .abs()
                    + (building_zone_return_flow - building_zone_return_flow_old).abs();
                if total_residual < 0.000010 {
                    o.hvac.zone_mass_balance_hvac_resim = false;
                    break;
                }
                o.hvac.zone_mass_balance_hvac_resim = true;
            }
            if !o.control.enforce_zone_mass_balance {
                break;
            }
            iteration += 1;
            if iteration >= 25 {
                break;
            }
        }
        // Same genuine NumPrimaryAirSys zero makes final SysRetFlow traversal empty.
        Ok(())
    }
    fn calculate_source_zone_leaving_conditions(
        &self,
        o: &SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.space_controls.simulation && !o.globals.doing_sizing {
            for row in &o.equipment.zone_return_mixers {
                match *row {}
            }
        }
        for number in 1..=o.globals.num_of_zones {
            let row = o
                .equipment
                .zone_config
                .as_deref()
                .and_then(|v| v.get(at(number).ok()?))
                .ok_or(Error::Unavailable("actual same ZoneEquipConfig5371"))?;
            if !row.is_controlled {
                continue;
            }
            if row.return_nodes == 0 {
                continue;
            }
            return Err(Error::Unavailable("actual controlled return ZoneNode5380"));
        }
        Ok(())
    }
}
