//! Actual per-System-step energy/ventilation zero-primary and uncontrolled route.
use super::*;
impl SourceSystemReports {
    pub(crate) fn report_source_system_energy_use(
        &mut self,
        o: &SourceSystemReportOwners<'_>,
    ) -> Result<(), Error> {
        let _time_step_sys_sec = o.hvac.time_step_sys_seconds;
        if !self.air_loop_loads_report_enabled {
            return Ok(());
        }
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual ReportSystemEnergyUse3020 SysLoadRepVars live row before resets",
            ));
        }
        // Both actual primary loops3019/3067 have zero rows.
        for zone in 1..=o.globals.num_of_zones {
            if !*o
                .equipment
                .source_zone_equipment_config(zone)?
                .is_controlled()
            {
                continue;
            }
            return Err(Error::Unavailable(
                "actual ReportSystemEnergyUse3155 ZoneSysEnergyDemand.TotalOutputRequired",
            ));
        }
        // Final actual primary sum loop3279 is likewise empty.
        Ok(())
    }
    pub(crate) fn report_source_ventilation_loads(
        &mut self,
        o: &mut SourceSystemReportOwners<'_>,
    ) -> Result<(), Error> {
        let _time_step_sys = o.hvac.time_step_sys;
        let _time_step_sys_sec = o.hvac.time_step_sys_seconds;
        if !self.vent_report_structure_created {
            return Ok(());
        }
        if !self.vent_loads_report_enabled {
            return Ok(());
        }
        for zone in 1..=o.globals.num_of_zones {
            // Native takes this real row reference before the IsControlled check.
            let row = self
                .zone_vent_rep_vars
                .as_mut()
                .and_then(|v| v.get_mut(at(zone).ok()?))
                .ok_or(Error::Unavailable("actual ZoneVentRepVars3857 live row"))?;
            if !*o
                .equipment
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
        if o.hvac.number_of_primary_air_systems > 0 {
            return Err(Error::Unavailable(
                "actual ReportVentilationLoads3892 SysVentRepVars live row",
            ));
        }
        for zone in 1..=o.globals.num_of_zones {
            if !*o
                .equipment
                .source_zone_equipment_config(zone)?
                .is_controlled()
            {
                continue;
            }
            return Err(Error::Unavailable(
                "actual ReportVentilationLoads3923 ZoneSysEnergyDemand.TotalOutputRequired",
            ));
        }
        // Both actual primary loops3891/4418 have zero rows.
        o.predefined_summary.total_any_zone_below_voz_dyn_for_oa +=
            self.any_zone_time_below_voz_dyn;
        o.predefined_summary.total_all_zones_at_voz_dyn_for_oa += self.all_zones_time_at_voz_dyn;
        o.predefined_summary.total_any_zone_above_voz_dyn_for_oa +=
            self.any_zone_time_above_voz_dyn;
        o.predefined_summary.total_any_zone_vent_unocc_for_oa += self.any_zone_time_vent_unocc;
        o.predefined_summary.total_any_zone_below_voz_dyn_occ_for_oa +=
            self.any_zone_time_below_voz_dyn_occ;
        o.predefined_summary.total_all_zones_at_voz_dyn_occ_for_oa +=
            self.all_zones_time_at_voz_dyn_occ;
        o.predefined_summary.total_any_zone_above_voz_dyn_occ_for_oa +=
            self.any_zone_time_above_voz_dyn_occ;
        Ok(())
    }
}
