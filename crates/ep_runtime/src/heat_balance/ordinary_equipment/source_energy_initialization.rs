//! Native SystemReports140-773/973-1005/5213-5273 on the same equipment owner.
use super::{EquipmentConfig, EquipmentList, OrdinaryEquipment, SourceAirIn, SourceEquipmentData};
use crate::heat_balance::ctf_reports::PredefinedStringTable;
use crate::heat_balance::ordinary_hvac_manager::simulation::state::SourceAirLoopData;
use crate::heat_balance::predefined_tables::PredefinedRegistry;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
use crate::heat_balance::source_output_registration::{
    SourceMeterResource, SourceOutputRegistration,
};

fn at(ordinal: usize) -> Result<usize, Error> {
    ordinal.checked_sub(1).ok_or(Error::Unavailable(
        "actual InitEnergyReports one-based retained ordinal",
    ))
}
fn zone_at(ordinal: i32) -> Result<usize, Error> {
    ordinal
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "actual InitEnergyReports native zone ordinal",
        ))
}
fn extent(value: usize) -> Result<i32, Error> {
    i32::try_from(value)
        .map_err(|_| Error::Unavailable("actual InitEnergyReports native isize domain"))
}

impl OrdinaryEquipment {
    fn energy_init_config(&self, zone: i32) -> Result<&EquipmentConfig, Error> {
        self.zone_config
            .as_ref()
            .and_then(|v| v.get(zone_at(zone).ok()?))
            .ok_or(Error::Unavailable(
                "actual InitEnergyReports retained ZoneEquipConfig",
            ))
    }
    fn energy_init_config_mut(&mut self, zone: i32) -> Result<&mut EquipmentConfig, Error> {
        self.zone_config
            .as_mut()
            .and_then(|v| v.get_mut(zone_at(zone).ok()?))
            .ok_or(Error::Unavailable(
                "actual InitEnergyReports retained ZoneEquipConfig",
            ))
    }
    fn energy_init_list(&self, list: usize) -> Result<&EquipmentList, Error> {
        self.equipment_lists
            .as_ref()
            .and_then(|v| v.get(at(list).ok()?))
            .ok_or(Error::Unavailable(
                "actual InitEnergyReports146 retained ZoneEquipList index",
            ))
    }
    fn energy_init_data(
        &self,
        list: usize,
        component: usize,
    ) -> Result<&SourceEquipmentData, Error> {
        self.energy_init_list(list)?
            .equipment_data
            .as_ref()
            .and_then(|v| v.get(at(component).ok()?))
            .ok_or(Error::Unavailable(
                "actual InitEnergyReports retained EquipData",
            ))
    }
    fn energy_init_data_mut(
        &mut self,
        list: usize,
        component: usize,
    ) -> Result<&mut SourceEquipmentData, Error> {
        self.equipment_lists
            .as_mut()
            .and_then(|v| v.get_mut(at(list).ok()?))
            .and_then(|v| v.equipment_data.as_mut())
            .and_then(|v| v.get_mut(at(component).ok()?))
            .ok_or(Error::Unavailable(
                "actual InitEnergyReports retained EquipData",
            ))
    }
    fn energy_init_distribution(
        &self,
        zone: i32,
        inlet: usize,
        cooling: bool,
    ) -> Result<&SourceAirIn, Error> {
        let config = self.energy_init_config(zone)?;
        let rows = if cooling {
            &config.air_distribution_units_cool
        } else {
            &config.air_distribution_units_heat
        };
        rows.as_ref()
            .and_then(|v| v.get(at(inlet).ok()?))
            .ok_or(Error::Unavailable(
                "actual InitEnergyReports retained AirDistUnitCool/Heat",
            ))
    }
    fn energy_init_distribution_mut(
        &mut self,
        zone: i32,
        inlet: usize,
        cooling: bool,
    ) -> Result<&mut SourceAirIn, Error> {
        let config = self.energy_init_config_mut(zone)?;
        let rows = if cooling {
            &mut config.air_distribution_units_cool
        } else {
            &mut config.air_distribution_units_heat
        };
        rows.as_mut()
            .and_then(|v| v.get_mut(at(inlet).ok()?))
            .ok_or(Error::Unavailable(
                "actual InitEnergyReports retained AirDistUnitCool/Heat",
            ))
    }
    fn find_energy_report_list(&self, zone: i32) -> Result<usize, Error> {
        let name = &self.energy_init_config(zone)?.equipment_list_name;
        // Native exact FindItemInList searches the WHOLE allocated array.
        let lists = self.equipment_lists.as_ref().ok_or(Error::Unavailable(
            "actual InitEnergyReports FindItemInList allocation",
        ))?;
        Ok(match lists.iter().position(|row| row.name == *name) {
            Some(index) => index + 1,
            None => 0, // genuine FindItemInList no-match return, not a fallback list
        })
    }

    pub(crate) fn initialize_source_equipment_energy_connections(
        &mut self,
        number_zones: i32,
        hvac: &SelectedHvacGlobals,
        air_loop: &SourceAirLoopData,
    ) -> Result<(), Error> {
        for zone in 1..=number_zones {
            if !self.energy_init_config(zone)?.is_controlled {
                continue;
            }
            let found = self.find_energy_report_list(zone)?;
            self.energy_init_config_mut(zone)?.equipment_list_index = found;
            let _selected_list = self.energy_init_list(found)?; // actual146 before inlet loop
            let inlets = self.energy_init_config(zone)?.inlet_nodes;
            for inlet in 1..=inlets {
                let air_loop_number = *self
                    .energy_init_config(zone)?
                    .inlet_node_air_loop_numbers
                    .as_ref()
                    .and_then(|v| v.get(at(inlet).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual InitEnergyReports148 InletNodeAirLoopNum",
                    ))?;
                let components = self.energy_init_list(found)?.number_of_equipment_types;
                for component in 1..=components {
                    let outlets = self.energy_init_data(found, component)?.number_outlets;
                    for outlet in 1..=outlets {
                        let outlet_index = usize::try_from(outlet - 1).map_err(|_| {
                            Error::Unavailable("actual InitEnergyReports150 outlet ordinal")
                        })?;
                        let node = *self
                            .energy_init_data(found, component)?
                            .outlet_node_numbers
                            .as_ref()
                            .and_then(|v| v.get(outlet_index))
                            .ok_or(Error::Unavailable(
                                "actual InitEnergyReports151 OutletNodeNums",
                            ))?;
                        let cool_node = i32::try_from(
                            self.energy_init_distribution(zone, inlet, true)?
                                .outlet_node,
                        )
                        .map_err(|_| {
                            Error::Unavailable(
                                "actual InitEnergyReports152 Cool.OutNode native domain",
                            )
                        })?;
                        if node == cool_node {
                            self.energy_init_distribution_mut(zone, inlet, true)?
                                .air_distribution_unit_index = extent(component)?;
                            if self
                                .energy_init_distribution(zone, inlet, true)?
                                .supply_air_path_exists
                            {
                                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                                #[allow(clippy::never_loop)]
                                for _path in 1..=self.number_of_supply_paths {
                                    return Err(Error::Unavailable(
                                        "actual InitEnergyReports156 SupplyAirPath.NumOutletNodes",
                                    ));
                                }
                            } else if air_loop_number > 0 {
                                return Err(Error::Unavailable(
                                    "actual InitEnergyReports190 AirToZoneNodeInfo.NumSupplyNodes",
                                ));
                            }
                        }
                        let heat_node = i32::try_from(
                            self.energy_init_distribution(zone, inlet, false)?
                                .outlet_node,
                        )
                        .map_err(|_| {
                            Error::Unavailable(
                                "actual InitEnergyReports223 Heat.OutNode native domain",
                            )
                        })?;
                        if node == heat_node {
                            self.energy_init_distribution_mut(zone, inlet, false)?
                                .air_distribution_unit_index = extent(component)?;
                            if self
                                .energy_init_distribution(zone, inlet, false)?
                                .supply_air_path_exists
                            {
                                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                                #[allow(clippy::never_loop)]
                                for _path in 1..=self.number_of_supply_paths {
                                    return Err(Error::Unavailable(
                                        "actual InitEnergyReports227 SupplyAirPath.NumOutletNodes",
                                    ));
                                }
                            } else if air_loop_number > 0 {
                                return Err(Error::Unavailable(
                                    "actual InitEnergyReports261 AirToZoneNodeInfo.NumSupplyNodes",
                                ));
                            }
                        }
                    }
                }
            }
        }
        for zone in 1..=number_zones {
            if !self.energy_init_config(zone)?.is_controlled {
                continue;
            }
            let found = self.find_energy_report_list(zone)?;
            self.energy_init_config_mut(zone)?.equipment_list_index = found; // second actual303 write
            let inlets = self.energy_init_config(zone)?.inlet_nodes;
            for inlet in 1..=inlets {
                let _air_loop_number = *self
                    .energy_init_config(zone)?
                    .inlet_node_air_loop_numbers
                    .as_ref()
                    .and_then(|v| v.get(at(inlet).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual InitEnergyReports307 InletNodeAirLoopNum",
                    ))?;
                let mut main_branch = self
                    .energy_init_distribution(zone, inlet, false)?
                    .main_branch_index;
                main_branch = std::cmp::max(
                    main_branch,
                    self.energy_init_distribution(zone, inlet, true)?
                        .main_branch_index,
                );
                if main_branch > 0 {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports313 MatchPlantSys main branch",
                    ));
                }
                let cooling_branch = self
                    .energy_init_distribution(zone, inlet, true)?
                    .supply_branch_index;
                if cooling_branch > 0 && cooling_branch != main_branch {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports317 MatchPlantSys cooling branch",
                    ));
                }
                let heating_branch = self
                    .energy_init_distribution(zone, inlet, false)?
                    .supply_branch_index;
                if heating_branch > 0 && heating_branch != main_branch {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports321 MatchPlantSys heating branch",
                    ));
                }
                let mut distribution = self
                    .energy_init_distribution(zone, inlet, true)?
                    .air_distribution_unit_index;
                distribution = std::cmp::max(
                    distribution,
                    self.energy_init_distribution(zone, inlet, false)?
                        .air_distribution_unit_index,
                );
                if found > 0 && distribution > 0 {
                    let component = usize::try_from(distribution).map_err(|_| {
                        Error::Unavailable("actual InitEnergyReports327 AirDistUnitNum")
                    })?;
                    let variables = self
                        .energy_init_data(found, component)?
                        .number_metered_variables;
                    for variable in 1..=variables {
                        let index = usize::try_from(variable - 1).map_err(|_| {
                            Error::Unavailable("actual InitEnergyReports328 VarNum")
                        })?;
                        let resource = self
                            .energy_init_data(found, component)?
                            .metered_variables
                            .as_ref()
                            .and_then(|v| v.get(index))
                            .ok_or(Error::Unavailable(
                                "actual InitEnergyReports329 MeteredVar.resource",
                            ))?
                            .variable
                            .resource;
                        if resource == SourceMeterResource::EnergyTransfer {
                            self.energy_init_data_mut(found, component)?
                                .energy_transfer_component = 1;
                            let _kind = &self.energy_init_data(found, component)?.type_of;
                            let _name = &self.energy_init_data(found, component)?.name;
                            return Err(Error::Unavailable(
                                "actual InitEnergyReports339 FindDemandSideMatch",
                            ));
                        }
                    }
                    let children = self
                        .energy_init_data(found, component)?
                        .number_sub_equipment;
                    // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                    #[allow(clippy::never_loop)]
                    for _child in 1..=children {
                        return Err(Error::Unavailable(
                            "actual InitEnergyReports348 SubEquipData.NumMeteredVars",
                        ));
                    }
                }
                // These traversals remain INSIDE each actual inlet, even if
                // AirDistUnitNum was0. No lifted readiness/count certificate.
                let zone_components =
                    extent(air_loop.zone_comp_to_plant.as_ref().map_or(0, Vec::len))?;
                let zone_children =
                    extent(air_loop.zone_sub_comp_to_plant.as_ref().map_or(0, Vec::len))?;
                let zone_grandchildren = extent(
                    air_loop
                        .zone_sub_sub_comp_to_plant
                        .as_ref()
                        .map_or(0, Vec::len),
                )?;
                let air_components =
                    extent(air_loop.air_sys_comp_to_plant.as_ref().map_or(0, Vec::len))?;
                let air_children = extent(
                    air_loop
                        .air_sys_sub_comp_to_plant
                        .as_ref()
                        .map_or(0, Vec::len),
                )?;
                let air_grandchildren = extent(
                    air_loop
                        .air_sys_sub_sub_comp_to_plant
                        .as_ref()
                        .map_or(0, Vec::len),
                )?;
                if zone_components > 0 {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports449 ZoneCompToPlant first dedup record",
                    ));
                }
                if zone_children > 0 {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports489 ZoneSubCompToPlant first dedup record",
                    ));
                }
                if zone_grandchildren > 0 {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports532 ZoneSubSubCompToPlant first dedup record",
                    ));
                }
                if air_components > 0 {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports577 AirSysCompToPlant first dedup record",
                    ));
                }
                if air_children > 0 {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports620 AirSysSubCompToPlant first dedup record",
                    ));
                }
                if air_grandchildren > 0 {
                    // Native664/693 deliberately use SubCompNum, not the
                    // SubSubCompNum guard; preserve that exact normal loop.
                    // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                    #[allow(clippy::never_loop)]
                    for _row in 1..=air_children {
                        return Err(Error::Unavailable(
                            "actual InitEnergyReports665 AirSysSubSubCompToPlant first dedup record",
                        ));
                    }
                    // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                    #[allow(clippy::never_loop)]
                    for _row in 1..=air_children {
                        return Err(Error::Unavailable(
                            "actual InitEnergyReports694 AirSysSubSubCompToPlant first zero-tail record",
                        ));
                    }
                }
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _plant in 1..=hvac.number_plant_loops {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports717 VentRepPlant Supply.TotalBranches",
                    ));
                }
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _plant in 1..=hvac.number_condenser_loops {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports746 VentRepCond Supply.TotalBranches",
                    ));
                }
            }
        }
        Ok(())
    }

    pub(crate) fn load_source_zone_equipment_energy_readings(
        &mut self,
        number_zones: i32,
        registration: &SourceOutputRegistration,
    ) -> Result<(), Error> {
        for zone in 1..=number_zones {
            if !self.energy_init_config(zone)?.is_controlled {
                continue;
            }
            let list = usize::try_from(zone).map_err(|_| {
                Error::Unavailable("actual InitEnergyReports978 direct list ordinal")
            })?;
            let components = self.energy_init_list(list)?.number_of_equipment_types;
            for component in 1..=components {
                let variables = self
                    .energy_init_data(list, component)?
                    .number_metered_variables;
                for variable in 1..=variables {
                    let index = usize::try_from(variable - 1)
                        .map_err(|_| Error::Unavailable("actual InitEnergyReports981 VarNum"))?;
                    let metered = self
                        .energy_init_data(list, component)?
                        .metered_variables
                        .as_ref()
                        .and_then(|v| v.get(index))
                        .ok_or(Error::Unavailable("actual InitEnergyReports982 MeteredVar"))?;
                    let variable_type = metered.variable.variable_type;
                    let variable_index = metered.variable.number;
                    let value = registration.get_source_internal_variable_value(
                        variable_type,
                        variable_index,
                        &*self,
                    )?;
                    self.energy_init_data_mut(list, component)?
                        .metered_variables
                        .as_mut()
                        .and_then(|v| v.get_mut(index))
                        .ok_or(Error::Unavailable(
                            "actual InitEnergyReports985 MeteredVar.curMeterReading",
                        ))?
                        .current_meter_reading = value;
                }
                let children = self.energy_init_data(list, component)?.number_sub_equipment;
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _child in 1..=children {
                    return Err(Error::Unavailable(
                        "actual InitEnergyReports988 SubEquipData retained row",
                    ));
                }
            }
        }
        Ok(())
    }

    pub(crate) fn report_source_zone_energy_topology(
        &self,
        number_zones: i32,
        zones: &SelectedZoneInputOwners,
        registry: &PredefinedRegistry,
        table: &mut PredefinedStringTable,
    ) -> Result<(), Error> {
        let mut row_counter = 1_i32;
        for zone in 1..=number_zones {
            let zone_name = &zones
                .zones
                .as_ref()
                .and_then(|v| v.get(zone_at(zone).ok()?))
                .ok_or(Error::Unavailable("actual topology5227 Zone.Name"))?
                .name;
            let zone_column = *registry
                .named_indices
                .get("pdchTopZnEqpName")
                .ok_or(Error::Unavailable("actual topology zone name column"))?;
            table.string_entry(zone_column, &format!("{row_counter}"), zone_name)?;
            row_counter = row_counter
                .checked_add(1)
                .ok_or(Error::Unavailable("undefined topology5229 row counter"))?;
            if !self.energy_init_config(zone)?.is_controlled {
                continue;
            }
            let list = usize::try_from(zone)
                .map_err(|_| Error::Unavailable("actual topology5233 direct list ordinal"))?;
            let components = self.energy_init_list(list)?.number_of_equipment_types;
            for component in 1..=components {
                let row = self.energy_init_data(list, component)?;
                table.string_entry(zone_column, &format!("{row_counter}"), zone_name)?;
                let type_column = *registry
                    .named_indices
                    .get("pdchTopZnEqpCompType")
                    .ok_or(Error::Unavailable("actual topology component type column"))?;
                table.string_entry(type_column, &format!("{row_counter}"), &row.type_of)?;
                let name_column = *registry
                    .named_indices
                    .get("pdchTopZnEqpCompName")
                    .ok_or(Error::Unavailable("actual topology component name column"))?;
                table.string_entry(name_column, &format!("{row_counter}"), &row.name)?;
                row_counter = row_counter
                    .checked_add(1)
                    .ok_or(Error::Unavailable("undefined topology5272 row counter"))?;
                // Preserve empty-range behavior, checked first-row access and the first unsupported positive-child failure after prior writes.
                #[allow(clippy::never_loop)]
                for _child in 1..=row.number_sub_equipment {
                    return Err(Error::Unavailable(
                        "actual topology5238 SubEquipData retained row",
                    ));
                }
            }
        }
        Ok(())
    }
}
