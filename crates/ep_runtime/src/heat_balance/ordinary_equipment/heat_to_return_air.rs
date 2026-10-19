//! Actual HVACManager::SetHeatToReturnAirFlag2884-3015 selected empty-air-loop route.
use super::OrdinaryEquipment;
use crate::heat_balance::ordinary_hvac_manager::SourceHvacManager;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::{
    HeatBalanceInputError as Error, zone::SelectedZoneInputOwners,
};
use crate::heat_balance::selected_simulation_startup::SelectedHvacGlobals;
fn index(number: i32) -> Result<usize, Error> {
    number
        .checked_sub(1)
        .and_then(|n| usize::try_from(n).ok())
        .ok_or(Error::Unavailable(
            "actual positive SetHeatToReturnAirFlag Zone ordinal",
        ))
}
impl OrdinaryEquipment {
    pub(crate) fn set_source_heat_to_return_air_flags(
        &mut self,
        hvac: &SelectedHvacGlobals,
        manager: &mut SourceHvacManager,
        globals: &SelectedHeatBalanceGlobals,
        zones: &mut SelectedZoneInputOwners,
    ) -> Result<(), Error> {
        if !hvac.air_loops_sim_once {
            return Ok(());
        }
        let number_primary_air_systems = hvac.number_of_primary_air_systems;
        if manager.my_one_time_flag {
            if number_primary_air_systems > 0 {
                return Err(Error::Unavailable(
                    "actual first AirLoopControlInfo one-time fan owner2911",
                ));
            }
            for number in 1..=globals.num_of_zones {
                let config = self
                    .zone_config
                    .as_mut()
                    .and_then(|v| v.get_mut(index(number).ok()?))
                    .ok_or(Error::Unavailable("actual same ZoneEquipConfig2930"))?;
                let mut air_loop_found = false;
                for ordinal in 0..config.inlet_nodes {
                    let air_loop = *config
                        .inlet_node_air_loop_numbers
                        .as_ref()
                        .and_then(|rows| rows.get(ordinal))
                        .ok_or(Error::Unavailable("actual same InletNodeAirLoopNum2934"))?;
                    if air_loop > 0 {
                        air_loop_found = true;
                    }
                }
                if !air_loop_found && config.inlet_nodes == config.exhaust_nodes {
                    config.zonal_system_only = true;
                }
            }
            for number in 1..=globals.num_of_zones {
                let config = self
                    .zone_config
                    .as_ref()
                    .and_then(|v| v.get(index(number).ok()?))
                    .ok_or(Error::Unavailable(
                        "actual same warning ZoneEquipConfig2945",
                    ))?;
                if !config.is_controlled {
                    continue;
                }
                let cycling_fan = false;
                for ordinal in 0..config.inlet_nodes {
                    let air_loop = *config
                        .inlet_node_air_loop_numbers
                        .as_ref()
                        .and_then(|rows| rows.get(ordinal))
                        .ok_or(Error::Unavailable("actual cycling InletNodeAirLoopNum2951"))?;
                    if air_loop > 0 {
                        return Err(Error::Unavailable(
                            "actual cycling AirLoopControlInfo cycFanSched2953",
                        ));
                    }
                }
                if config.zonal_system_only || cycling_fan {
                    let _zone = zones
                        .zones
                        .as_ref()
                        .and_then(|v| v.get(index(number).ok()?))
                        .ok_or(Error::Unavailable(
                            "actual same return-gain warning Zone2961",
                        ))?;
                    return Err(Error::Unavailable(
                        "actual positive controlled-zone RefrigCaseRA/Lights/airflow-window warning operands2962",
                    ));
                }
            }
            manager.my_one_time_flag = false;
        }
        if number_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual first AirLoopControlInfo fan-mode owner2990",
            ));
        }
        for number in 1..=globals.num_of_zones {
            let config = self
                .zone_config
                .as_ref()
                .and_then(|v| v.get(index(number).ok()?))
                .ok_or(Error::Unavailable("actual same final ZoneEquipConfig3001"))?;
            let zone = zones
                .zones
                .as_mut()
                .and_then(|v| v.get_mut(index(number).ok()?))
                .ok_or(Error::Unavailable("actual same NoHeatToReturnAir Zone3002"))?;
            if !config.is_controlled {
                continue;
            }
            zone.no_heat_to_return_air = true;
            if !config.zonal_system_only {
                for ordinal in 0..config.inlet_nodes {
                    let air_loop = *config
                        .inlet_node_air_loop_numbers
                        .as_ref()
                        .and_then(|rows| rows.get(ordinal))
                        .ok_or(Error::Unavailable("actual final InletNodeAirLoopNum3009"))?;
                    if air_loop > 0 {
                        return Err(Error::Unavailable(
                            "actual continuous AirLoopControlInfo fanOp3011",
                        ));
                    }
                }
            }
        }
        Ok(())
    }
}
