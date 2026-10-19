//! Actual SystemReports controlled-zone report bodies on the sole live owners.
use super::*;
use crate::heat_balance::ordinary_air_heat_balance::{
    SourceAirHeatBalance,
    predictor_corrector::{SourceZoneAirDemand, SourceZoneAirReports},
};
use crate::heat_balance::selected_sizing::SelectedSizingManager;
use crate::psychrometrics::{energyplus_psy_h_fn_tdb_w, energyplus_psy_rho_air_fn_pb_tdb_w};
use crate::weather::day::WeatherEnvironmentState;

// ObjexxFCL double max338-355 retains its first operand on equality/NaN.
fn cpp_max(a: f64, b: f64) -> f64 {
    if a < b { b } else { a }
}

impl SourceSystemReports {
    pub(crate) fn report_source_system_energy_use(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        hvac: &SelectedHvacGlobals,
        equipment: &OrdinaryEquipment,
        demand: &SourceZoneAirDemand,
    ) -> Result<(), Error> {
        let _time_step_sys_sec = hvac.time_step_sys_seconds;
        if !self.air_loop_loads_report_enabled {
            return Ok(());
        }
        if hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual ReportSystemEnergyUse3020 SysLoadRepVars live row before resets",
            ));
        }
        // Actual primary reset3019 and component3067 loops have no rows here.
        for zone in 1..=globals.num_of_zones {
            let config = equipment.source_zone_equipment_config(zone)?;
            if !*config.is_controlled() {
                continue;
            }
            let mut zone_load = demand
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at(zone).ok()?))
                .ok_or(Error::Unavailable(
                    "actual ReportSystemEnergyUse3155 ZoneSysEnergyDemand row",
                ))?
                .base
                .total_output_required;
            if *demand
                .deadband_or_setback
                .as_ref()
                .and_then(|rows| rows.get(at(zone).ok()?))
                .ok_or(Error::Unavailable(
                    "actual ReportSystemEnergyUse3158 DeadBandOrSetback",
                ))?
            {
                zone_load = 0.0;
            }
            // Native reads this local even though its controlled suffix does not use it.
            let _zone_load = zone_load;
            for inlet in 1..=*config.number_inlet_nodes() {
                let air_loop =
                    *config
                        .inlet_node_air_loop_number(inlet)
                        .ok_or(Error::Unavailable(
                            "actual ReportSystemEnergyUse3165 InletNodeAirLoopNum",
                        ))?;
                if air_loop == 0 {
                    continue;
                }
                return Err(Error::Unavailable(
                    "actual ReportSystemEnergyUse3173 AirDistUnitCool.InNode positive reporting child",
                ));
            }
        }
        // Actual final primary sum3279 is likewise empty.
        Ok(())
    }

    #[allow(clippy::too_many_arguments)]
    pub(crate) fn report_source_ventilation_loads(
        &mut self,
        globals: &SelectedHeatBalanceGlobals,
        hvac: &SelectedHvacGlobals,
        equipment: &OrdinaryEquipment,
        zones: &mut SelectedZoneInputOwners,
        demand: &SourceZoneAirDemand,
        air_reports: &SourceZoneAirReports,
        air: &SourceAirHeatBalance,
        environment: &WeatherEnvironmentState,
        sizing: &SelectedSizingManager,
        predefined_summary: &mut SourceComfortPredefinedSummary,
    ) -> Result<(), Error> {
        // This is the report-local Native threshold, distinct from HVAC::SmallLoad.
        const SMALL_LOAD: f64 = 0.1;
        const SMALL_AIR_VOL_FLOW: f64 = 0.001;
        let time_step_sys = hvac.time_step_sys;
        let time_step_sys_sec = hvac.time_step_sys_seconds;
        if !self.vent_report_structure_created {
            return Ok(());
        }
        if !self.vent_loads_report_enabled {
            return Ok(());
        }
        for zone in 1..=globals.num_of_zones {
            // Native takes this real row reference before IsControlled.
            let row = self
                .zone_vent_rep_vars
                .as_mut()
                .and_then(|v| v.get_mut(at(zone).ok()?))
                .ok_or(Error::Unavailable("actual ZoneVentRepVars3857 live row"))?;
            if !*equipment
                .source_zone_equipment_config(zone)?
                .is_controlled()
            {
                continue;
            }
            row.oa_mass_flow = 0.0;
            row.oa_mass = 0.0;
            row.oa_vol_flow_std_rho = 0.0;
            row.oa_vol_std_rho = 0.0;
            row.oa_vol_flow_crnt_rho = 0.0;
            row.oa_vol_crnt_rho = 0.0;
            row.mech_ach = 0.0;
            row.target_ventilation_flow_voz = 0.0;
            row.time_below_voz_dyn = 0.0;
            row.time_at_voz_dyn = 0.0;
            row.time_above_voz_dyn = 0.0;
            row.time_vent_unocc = 0.0;
            row.cooling_load_met_by_vent = 0.0;
            row.cooling_load_added_by_vent = 0.0;
            row.overcooling_by_vent = 0.0;
            row.heating_load_met_by_vent = 0.0;
            row.heating_load_added_by_vent = 0.0;
            row.overheating_by_vent = 0.0;
            row.no_load_heating_by_vent = 0.0;
            row.no_load_cooling_by_vent = 0.0;
        }
        self.any_zone_time_below_voz_dyn = 0.0;
        self.all_zones_time_at_voz_dyn = 0.0;
        self.any_zone_time_above_voz_dyn = 0.0;
        self.any_zone_time_vent_unocc = 0.0;
        self.any_zone_time_below_voz_dyn_occ = 0.0;
        self.all_zones_time_at_voz_dyn_occ = 0.0;
        self.any_zone_time_above_voz_dyn_occ = 0.0;
        if hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual ReportVentilationLoads3892 SysVentRepVars live row",
            ));
        }
        for zone in 1..=globals.num_of_zones {
            let config = equipment.source_zone_equipment_config(zone)?;
            if !*config.is_controlled() {
                continue;
            }
            let mut z_air_sys_zone_vent_load = 0.0;
            let mut z_air_sys_out_air_flow = 0.0;
            let mut zfau_flow_rate = 0.0;
            let mut zfau_zone_vent_load = 0.0;
            let mut zfau_out_air_flow = 0.0;
            let mut zone_load = demand
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at(zone).ok()?))
                .ok_or(Error::Unavailable(
                    "actual ReportVentilationLoads3923 ZoneSysEnergyDemand row",
                ))?
                .base
                .total_output_required;
            let this_zone = zones
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at(zone).ok()?))
                .ok_or(Error::Unavailable(
                    "actual ReportVentilationLoads3924 Zone row",
                ))?;
            let zone_mult = this_zone
                .multiplier
                .checked_mul(this_zone.list_multiplier)
                .ok_or(Error::Unavailable(
                    "actual ReportVentilationLoads3925 Native integer multiplier product domain",
                ))?;
            let zone_volume = this_zone.volume * f64::from(zone_mult);
            let predef = zones
                .zone_predefined
                .as_mut()
                .and_then(|rows| rows.get_mut(at(zone).ok()?))
                .ok_or(Error::Unavailable(
                    "actual ReportVentilationLoads3931 ZonePreDefRep",
                ))?;
            let row = self
                .zone_vent_rep_vars
                .as_mut()
                .and_then(|rows| rows.get_mut(at(zone).ok()?))
                .ok_or(Error::Unavailable(
                    "actual ReportVentilationLoads3932 ZoneVentRepVars",
                ))?;
            row.target_ventilation_flow_voz = sizing
                .calculate_source_design_specification_outdoor_air(
                    *config.zone_design_specification_outdoor_air_index(),
                    zone,
                    true,
                    true,
                )?;
            if *config.zone_air_distribution_index() > 0 {
                return Err(Error::Unavailable(
                    "actual ReportVentilationLoads3938 ZoneAirDistribution.calculateEz",
                ));
            }
            if *demand
                .deadband_or_setback
                .as_ref()
                .and_then(|rows| rows.get(at(zone).ok()?))
                .ok_or(Error::Unavailable(
                    "actual ReportVentilationLoads3942 DeadBandOrSetback",
                ))?
            {
                zone_load = 0.0;
            }
            let list_index = i32::try_from(*config.equipment_list_index()).map_err(|_| {
                Error::Unavailable(
                    "actual ReportVentilationLoads3947 EquipListIndex Native integer domain",
                )
            })?;
            let list = equipment.source_zone_equipment_list(list_index)?;
            for equipment_number in 1..=*list.number_equipment_types() {
                let equipment_index =
                    *list
                        .equipment_index(equipment_number)
                        .ok_or(Error::Unavailable(
                            "actual ReportVentilationLoads3949 EquipIndex",
                        ))?;
                let equipment_type =
                    *list
                        .equipment_type(equipment_number)
                        .ok_or(Error::Unavailable(
                            "actual ReportVentilationLoads3951 EquipType",
                        ))?;
                // Exact DataZoneEquipment::ZoneEquipType underlying values.
                match equipment_type {
                    12 => {
                        zfau_out_air_flow +=
                            equipment.source_purchased_air_out_air_mass_flow(equipment_index)?;
                        let zone_inlet_air_node =
                            equipment.source_purchased_air_zone_inlet_air_node(equipment_index)?;
                        if zone_inlet_air_node > 0 {
                            zfau_flow_rate = cpp_max(
                                *equipment
                                    .source_zone_air_node(zone_inlet_air_node)?
                                    .mass_flow_rate,
                                0.0,
                            );
                        }
                        let zfau_temp_mixed_air = equipment
                            .source_purchased_air_mixed_air_temperature(equipment_index)?;
                        let zfau_hum_rat_mixed_air = equipment
                            .source_purchased_air_mixed_air_humidity_ratio(equipment_index)?;
                        let return_air_node =
                            equipment.source_purchased_air_return_air_node(equipment_index)?;
                        if zfau_flow_rate > 0.0 && return_air_node > 0 {
                            let zfau_enth_mixed_air = energyplus_psy_h_fn_tdb_w(
                                zfau_temp_mixed_air,
                                zfau_hum_rat_mixed_air,
                            );
                            let return_node = equipment.source_zone_air_node(return_air_node)?;
                            let zfau_enth_return_air = energyplus_psy_h_fn_tdb_w(
                                *return_node.temperature,
                                *return_node.humidity_ratio,
                            );
                            zfau_zone_vent_load += zfau_flow_rate
                                * (zfau_enth_mixed_air - zfau_enth_return_air)
                                * time_step_sys_sec;
                        } else {
                            zfau_zone_vent_load += 0.0;
                        }
                    }
                    1 => {
                        return Err(Error::Unavailable(
                            "actual ReportVentilationLoads4021 FanCoil GetFanCoilOutAirNode child",
                        ));
                    }
                    2..=4 => {
                        return Err(Error::Unavailable(
                            "actual ReportVentilationLoads4002 PTUnit compPointer.getMixerOANode child",
                        ));
                    }
                    5 => {
                        return Err(Error::Unavailable(
                            "actual ReportVentilationLoads3955 WindowAC GetWindowACOutAirNode child",
                        ));
                    }
                    7 => {
                        return Err(Error::Unavailable(
                            "actual ReportVentilationLoads4043 UnitVentilator GetUnitVentilatorOutAirNode child",
                        ));
                    }
                    8 => {
                        return Err(Error::Unavailable(
                            "actual ReportVentilationLoads4084 ERV GetStandAloneERVOutAirNode child",
                        ));
                    }
                    10 => {
                        return Err(Error::Unavailable(
                            "actual ReportVentilationLoads4110 OutdoorAirUnit GetOutdoorAirUnitOutAirNode child",
                        ));
                    }
                    11 => {
                        return Err(Error::Unavailable(
                            "actual ReportVentilationLoads3977 VRF GetVRFTUOutAirNode child",
                        ));
                    }
                    14 => {
                        return Err(Error::Unavailable(
                            "actual ReportVentilationLoads4131 Hybrid GetHybridUnitaryACOutAirNode child",
                        ));
                    }
                    // Native4105-4107 has a separate actual empty UnitarySystem case.
                    33 => {}
                    // Native4153-4178 explicitly excludes OA for these real types.
                    6 | 9 | 13 | 15..=32 => {}
                    _ => {
                        return Err(Error::Unavailable(
                            "actual ReportVentilationLoads4181 unsupported EquipType fatal",
                        ));
                    }
                }
            }
            for inlet in 1..=*config.number_inlet_nodes() {
                let air_loop =
                    *config
                        .inlet_node_air_loop_number(inlet)
                        .ok_or(Error::Unavailable(
                            "actual ReportVentilationLoads4200 InletNodeAirLoopNum",
                        ))?;
                if air_loop != 0 {
                    return Err(Error::Unavailable(
                        "actual ReportVentilationLoads4204 AirDistUnitCool.InNode positive reporting child",
                    ));
                }
                // Real MixedAirNode/ReturnAirNode remain0 for this actual association;
                // Native4251-4253 sets both per-inlet values0 before accumulation.
                let air_sys_zone_vent_load = 0.0;
                let air_sys_out_air_flow = 0.0;
                z_air_sys_zone_vent_load += air_sys_zone_vent_load;
                z_air_sys_out_air_flow += air_sys_out_air_flow;
            }
            let out_air_flow = z_air_sys_out_air_flow + zfau_out_air_flow;
            row.oa_mass_flow = out_air_flow;
            row.oa_mass = row.oa_mass_flow * time_step_sys_sec;
            row.oa_vol_flow_std_rho = row.oa_mass_flow / environment.std_rho_air;
            row.oa_vol_std_rho = row.oa_vol_flow_std_rho * time_step_sys_sec;
            let zone_air_report = air_reports
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at(zone).ok()?))
                .ok_or(Error::Unavailable(
                    "actual ReportVentilationLoads4295 ZnAirRpt",
                ))?;
            let tot_mech_nat_vent_vol_std_rho = row.oa_vol_std_rho
                + (zone_air_report.ventil_volume_std_density + predef.afn_vent_vol_std_den)
                    * f64::from(zone_mult);
            let target_voz = row.target_ventilation_flow_voz * time_step_sys_sec;
            if tot_mech_nat_vent_vol_std_rho < 0.99 * target_voz {
                row.time_below_voz_dyn = time_step_sys;
                self.any_zone_time_below_voz_dyn = time_step_sys;
            } else if tot_mech_nat_vent_vol_std_rho > 1.01 * target_voz {
                row.time_above_voz_dyn = time_step_sys;
                self.any_zone_time_above_voz_dyn = time_step_sys;
            } else if tot_mech_nat_vent_vol_std_rho > SMALL_AIR_VOL_FLOW {
                row.time_at_voz_dyn = time_step_sys;
                self.all_zones_time_at_voz_dyn = time_step_sys;
            }
            let zone_heat_balance = air
                .zones
                .as_ref()
                .and_then(|rows| rows.get(at(zone).ok()?))
                .ok_or(Error::Unavailable(
                    "actual ReportVentilationLoads4313 zoneHeatBalance",
                ))?;
            // This real property call occurs even when actual OA mass flow is0.
            let current_zone_air_density = energyplus_psy_rho_air_fn_pb_tdb_w(
                environment.current_weather.out_baro_press,
                zone_heat_balance.air.mat,
                zone_heat_balance.air.air_hum_rat_avg,
            );
            if current_zone_air_density < 0.0 {
                return Err(Error::Unavailable(
                    "actual PsyRhoAirFnPbTdbW negative-density diagnostic Psychrometrics.hh543",
                ));
            }
            if current_zone_air_density > 0.0 {
                row.oa_vol_flow_crnt_rho = row.oa_mass_flow / current_zone_air_density;
            }
            row.oa_vol_crnt_rho = row.oa_vol_flow_crnt_rho * time_step_sys_sec;
            if zone_volume > 0.0 {
                row.mech_ach = (row.oa_vol_crnt_rho / time_step_sys) / zone_volume;
            }
            if predef.is_occupied {
                predef.tot_time_occ += time_step_sys;
                predef.mech_vent_vol_total_occ += row.oa_vol_crnt_rho;
                if row.oa_vol_crnt_rho / time_step_sys < predef.mech_vent_vol_min {
                    predef.mech_vent_vol_min = row.oa_vol_crnt_rho / time_step_sys;
                }
                predef.mech_vent_vol_total_occ_std_den += row.oa_vol_std_rho;
                predef.infil_vol_total_occ += zone_air_report.infil_volume_cur_density;
                if zone_air_report.infil_volume_cur_density < predef.infil_vol_min {
                    predef.infil_vol_min = zone_air_report.infil_volume_cur_density;
                }
                predef.infil_vol_total_occ_std_den += zone_air_report.infil_volume_std_density;
                predef.simp_vent_vol_total_occ += zone_air_report.ventil_volume_cur_density;
                if zone_air_report.ventil_volume_cur_density < predef.simp_vent_vol_min {
                    predef.simp_vent_vol_min = zone_air_report.ventil_volume_cur_density;
                }
                predef.simp_vent_vol_total_occ_std_den += zone_air_report.ventil_volume_std_density;
                self.any_zone_time_below_voz_dyn_occ = self.any_zone_time_below_voz_dyn;
                self.all_zones_time_at_voz_dyn_occ = self.all_zones_time_at_voz_dyn;
                self.any_zone_time_above_voz_dyn_occ = self.any_zone_time_above_voz_dyn;
                predef.voz_target_total_occ += target_voz;
                predef.voz_target_time_below_occ += row.time_below_voz_dyn;
                predef.voz_target_time_at_occ += row.time_at_voz_dyn;
                predef.voz_target_time_above_occ += row.time_above_voz_dyn;
            } else if tot_mech_nat_vent_vol_std_rho > SMALL_AIR_VOL_FLOW {
                row.time_vent_unocc = time_step_sys;
                self.any_zone_time_vent_unocc = time_step_sys;
                predef.tot_vent_time_non_zero_unocc += row.time_vent_unocc;
            }
            predef.mech_vent_vol_total_std_den += row.oa_vol_std_rho;
            predef.infil_vol_total_std_den += zone_air_report.infil_volume_std_density;
            predef.simp_vent_vol_total_std_den += zone_air_report.ventil_volume_std_density;
            predef.voz_target_total += target_voz;
            predef.voz_target_time_below += row.time_below_voz_dyn;
            predef.voz_target_time_at += row.time_at_voz_dyn;
            predef.voz_target_time_above += row.time_above_voz_dyn;
            let zone_vent_load: f64 = z_air_sys_zone_vent_load + zfau_zone_vent_load;
            if zone_vent_load.abs() < SMALL_LOAD {
                continue;
            }
            if zone_vent_load > SMALL_LOAD {
                if zone_load < -SMALL_LOAD {
                    row.cooling_load_added_by_vent += zone_vent_load.abs();
                } else if zone_load > SMALL_LOAD {
                    if zone_vent_load > zone_load {
                        row.heating_load_met_by_vent += zone_load.abs();
                        row.overheating_by_vent += zone_vent_load - zone_load;
                    } else {
                        row.heating_load_met_by_vent += zone_vent_load.abs();
                    }
                } else {
                    row.no_load_heating_by_vent += zone_vent_load.abs();
                }
            } else if zone_vent_load < -SMALL_LOAD {
                if zone_load < -SMALL_LOAD {
                    if zone_vent_load < zone_load {
                        row.cooling_load_met_by_vent += zone_load.abs();
                        row.overcooling_by_vent += (zone_vent_load - zone_load).abs();
                    } else {
                        row.cooling_load_met_by_vent += zone_vent_load.abs();
                    }
                } else if zone_load > SMALL_LOAD {
                    row.heating_load_added_by_vent += zone_vent_load.abs();
                } else {
                    row.no_load_cooling_by_vent += zone_vent_load.abs();
                }
            }
        }
        // Both primary SysVent loops3891/4418 have no rows here.
        predefined_summary.total_any_zone_below_voz_dyn_for_oa += self.any_zone_time_below_voz_dyn;
        predefined_summary.total_all_zones_at_voz_dyn_for_oa += self.all_zones_time_at_voz_dyn;
        predefined_summary.total_any_zone_above_voz_dyn_for_oa += self.any_zone_time_above_voz_dyn;
        predefined_summary.total_any_zone_vent_unocc_for_oa += self.any_zone_time_vent_unocc;
        predefined_summary.total_any_zone_below_voz_dyn_occ_for_oa +=
            self.any_zone_time_below_voz_dyn_occ;
        predefined_summary.total_all_zones_at_voz_dyn_occ_for_oa +=
            self.all_zones_time_at_voz_dyn_occ;
        predefined_summary.total_any_zone_above_voz_dyn_occ_for_oa +=
            self.any_zone_time_above_voz_dyn_occ;
        Ok(())
    }
}
