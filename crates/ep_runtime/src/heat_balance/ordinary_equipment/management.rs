//! Genuine ZoneEquipmentManager142–320/3538–4194/4933–5284/5342–5570 ordinary path.
pub(crate) mod availability;
mod modules;
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
        _first_hvac_iteration: bool,
        o: &mut SourceHvacOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.equipment.init_equipment_one_time_flag {
            o.equipment.init_equipment_one_time_flag = false;
            let count = usize::try_from(o.globals.num_of_zones)
                .map_err(|_| Error::Unavailable("actual ZoneEqSizing NumOfZones allocation214"))?;
            o.sizing.zone_equipment_sizing =
                Some(vec![SourceZoneEquipmentSizing::default(); count]);
            for number in 1..=o.globals.num_of_zones {
                let config = o.equipment.source_zone_equipment_config(number)?;
                if !*config.is_controlled() {
                    continue;
                }
                let row = o
                    .equipment
                    .zone_config
                    .as_deref()
                    .and_then(|v| v.get(at(number).ok()?))
                    .ok_or(Error::Unavailable("actual controlled ZoneEquipConfig218"))?;
                if row.equipment_list_index == 0 {
                    continue;
                }
                return Err(Error::Unavailable(
                    "actual controlled ZoneEquipList count223",
                ));
            }
        }
        if o.equipment.init_equipment_environment_flag && o.clock.begin_envrn_flag {
            if let Some(rows) = o.equipment.equipment_availability.as_mut() {
                rows.fill(0);
            }
            if let Some(rows) = self.ancillary.availability.zone_components.as_mut() {
                for number in 1..=14 {
                    let row = rows.get_mut(number - 1).ok_or(Error::Unavailable(
                        "actual same ZoneComp type allocation263",
                    ))?;
                    if let Some(managers) = row.managers.as_mut() {
                        for number in 1..=row.total_components {
                            let manager = managers
                                .get_mut(at(number)?)
                                .ok_or(Error::Unavailable("actual ZoneCompAvailMgrs268"))?;
                            manager.status = 0;
                            manager.start_time = 0;
                            manager.stop_time = 0;
                        }
                    }
                }
            }
            if let Some(rows) = &o.equipment.zone_config {
                for row in rows {
                    if !row.is_controlled {
                        continue;
                    }
                    return Err(Error::Unavailable(
                        "actual controlled Config beginEnvironmentInit278",
                    ));
                }
            }
            if o.space_controls.simulation {
                if let Some(rows) = &o.equipment.space_config {
                    for row in rows {
                        if !row.is_controlled {
                            continue;
                        }
                        return Err(Error::Unavailable(
                            "actual controlled SpaceConfig beginEnvironmentInit285",
                        ));
                    }
                }
            }
            o.equipment.init_equipment_environment_flag = false;
        }
        if !o.clock.begin_envrn_flag {
            o.equipment.init_equipment_environment_flag = true;
        }
        if let Some(rows) = &o.equipment.zone_config {
            for row in rows {
                if !row.is_controlled {
                    continue;
                }
                return Err(Error::Unavailable(
                    "actual controlled Config hvacTimeStepInit304",
                ));
            }
        }
        if o.space_controls.simulation {
            if let Some(rows) = &o.equipment.space_config {
                for row in rows {
                    if !row.is_controlled {
                        continue;
                    }
                    return Err(Error::Unavailable(
                        "actual controlled SpaceConfig hvacTimeStepInit311",
                    ));
                }
            }
        }
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual InitZoneEquipment AirLoopFlow317",
            ));
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
            return Err(Error::Unavailable("actual controlled Config.ZoneExh3665"));
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
