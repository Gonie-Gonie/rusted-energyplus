//! Actual HVACManager::SetHeatToReturnAirFlag2884-3015 selected empty-air-loop route.
use super::OrdinaryEquipment;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::error_backend::{SourceDiagnosticFlags, SourceErrorBackend};
use crate::heat_balance::ordinary_hvac_manager::SourceHvacManager;
use crate::heat_balance::selected_heat_balance_globals::SelectedHeatBalanceGlobals;
use crate::heat_balance::selected_heat_balance_input::surface::SurfaceGeometryInputState;
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
    #[allow(clippy::too_many_arguments)] // Reborrow the genuine reached module owners.
    pub(crate) fn set_source_heat_to_return_air_flags(
        &mut self,
        hvac: &SelectedHvacGlobals,
        manager: &mut SourceHvacManager,
        globals: &SelectedHeatBalanceGlobals,
        zones: &mut SelectedZoneInputOwners,
        gains: &SelectedInternalHeatGains,
        geometry: &SurfaceGeometryInputState,
        backend: &mut SourceErrorBackend<'_>,
        diagnostic_flags: &SourceDiagnosticFlags<'_>,
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
                    let zone = zones
                        .zones
                        .as_ref()
                        .and_then(|v| v.get(index(number).ok()?))
                        .ok_or(Error::Unavailable(
                            "actual same return-gain warning Zone2961",
                        ))?;
                    if zone.refrig_case_return_air {
                        backend.show_source_warning_error(
                            &format!(
                                "For zone={} return air cooling by refrigerated cases will be applied to the zone air.",
                                zone.name
                            ),
                            diagnostic_flags,
                        )?;
                        backend.show_continue_error(
                            "  This zone has no return air or is served by an on/off HVAC system.",
                        )?;
                    }
                    // The retained TotLights cell is the actual loop bound.
                    // Positive Lights rows are not yet produced by the Source
                    // getter; misc input rows and Zone aggregates are not Lights.
                    #[allow(clippy::never_loop)]
                    for _light in 1..=gains.totals[1] {
                        return Err(Error::Unavailable(
                            "actual SetHeatToReturnAirFlag Lights.ZonePtr2969",
                        ));
                    }
                    for &space_number in &zone.space_indexes {
                        let space = zones
                            .spaces
                            .as_ref()
                            .and_then(|rows| rows.get(index(space_number).ok()?))
                            .ok_or(Error::Unavailable(
                                "actual SetHeatToReturnAirFlag same Space2982",
                            ))?;
                        for surface_number in space.ht_surface_first..=space.ht_surface_last {
                            let destination = geometry
                                .window_arrays
                                .as_ref()
                                .and_then(|arrays| {
                                    arrays
                                        .surf_win_airflow_destination
                                        .get(index(surface_number).ok()?)
                                })
                                .ok_or(Error::Unavailable(
                                    "actual SetHeatToReturnAirFlag SurfWinAirflowDestination2984",
                                ))?;
                            // DataSurfaces.hh438-444: Invalid=-1, Indoor=0,
                            // Outdoor=1, Return=2. The actual allocation is -1.
                            if *destination == 2 {
                                backend.show_source_warning_error(
                                    &format!(
                                        "For zone={} return air heat gain from air flow windows will be applied to the zone air.",
                                        zone.name
                                    ),
                                    diagnostic_flags,
                                )?;
                                backend.show_continue_error(
                                    "  This zone has no return air or is served by an on/off HVAC system.",
                                )?;
                            }
                        }
                    }
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
