use super::state::SourceReportMatrix;
use super::*;
impl SourceHeatBalanceManager {
    pub(super) fn allocate_heat_balance_arrays(
        &mut self,
        owners: &mut SourceInitHeatBalanceOwners<'_, '_>,
        providers: &mut impl SourceInitHeatBalanceProviders,
    ) -> Result<(), Error> {
        if owners.gains.allocations.zone_int_gain.is_none() {
            providers.allocate_internal_gains(owners)?;
        }
        owners.air.allocate_source_arrays(owners.globals)?;
        owners.radiation.allocate_source_solar_enclosure_arrays()?;
        owners
            .fan_zone
            .allocate_source_fan_zone_arrays(owners.globals)?;
        if owners.control.carbon_dioxide_simulation {
            providers.allocate_carbon_dioxide(&mut self.contaminants, owners)?;
        }
        if owners.control.generic_contaminant_simulation {
            providers.allocate_generic_contaminant(&mut self.contaminants, owners)?;
        }
        let zones = extent(owners.globals.num_of_zones)?;
        self.max_temp_prev_day = Some(vec![0.0; zones]);
        self.min_temp_prev_day = Some(vec![0.0; zones]);
        self.max_heat_load_prev_day = Some(vec![0.0; zones]);
        self.max_cool_load_prev_day = Some(vec![0.0; zones]);
        self.max_heat_load_zone = Some(vec![-9999.0; zones]);
        self.max_cool_load_zone = Some(vec![-9999.0; zones]);
        self.max_temp_zone = Some(vec![-9999.0; zones]);
        self.min_temp_zone = Some(vec![1000.0; zones]);
        self.temp_zone_prev_day = Some(vec![0.0; zones]);
        self.load_zone_prev_day = Some(vec![0.0; zones]);
        self.temp_zone_sec_prev_day = Some(vec![0.0; zones]);
        self.load_zone_sec_prev_day = Some(vec![0.0; zones]);
        self.report.temperature_difference = vec![0.0; zones];
        self.report
            .source_allocations
            .as_mut()
            .ok_or(Error::Unavailable("actual Source report availability"))?[0] = true;
        self.report.load_difference = vec![0.0; zones];
        self.report
            .source_allocations
            .as_mut()
            .ok_or(Error::Unavailable("actual Source report availability"))?[1] = true;
        self.temp_zone = Some(vec![0.0; zones]);
        self.load_zone = Some(vec![0.0; zones]);
        let points = extent(owners.clock.time_steps_in_hour)?
            .checked_mul(24)
            .ok_or(Error::Unavailable("actual HB report timestep extent"))?;
        self.report.points_per_day = points;
        self.report.temperature = vec![vec![0.0; zones]; points];
        self.report
            .source_allocations
            .as_mut()
            .ok_or(Error::Unavailable("actual Source report availability"))?[2] = true;
        self.report.load = vec![vec![0.0; zones]; points];
        self.report
            .source_allocations
            .as_mut()
            .ok_or(Error::Unavailable("actual Source report availability"))?[3] = true;
        self.report.maximum_load = vec![vec![0.0; zones]; points];
        self.report
            .source_allocations
            .as_mut()
            .ok_or(Error::Unavailable("actual Source report availability"))?[4] = true;
        self.convergence = Some(vec![SourceWarmupConvergence::default(); zones]);
        self.report.temperature_stddev = vec![None; points];
        self.report
            .source_allocations
            .as_mut()
            .ok_or(Error::Unavailable("actual Source report availability"))?[5] = true;
        self.report.load_stddev = vec![None; points];
        self.report
            .source_allocations
            .as_mut()
            .ok_or(Error::Unavailable("actual Source report availability"))?[6] = true;
        self.crossed_cold_thresh_rep_period = Some(SourceReportMatrix::allocate(
            zones,
            extent(owners.weather_owner.tot_thermal_report_periods)?,
        )?);
        self.crossed_heat_thresh_rep_period = Some(SourceReportMatrix::allocate(
            zones,
            extent(owners.weather_owner.tot_thermal_report_periods)?,
        )?);
        self.crossed_cold_thresh_rep_period
            .as_mut()
            .ok_or(Error::Unavailable(
                "actual allocated threshold report array",
            ))?
            .cells
            .fill(Some(false));
        self.crossed_heat_thresh_rep_period
            .as_mut()
            .ok_or(Error::Unavailable(
                "actual allocated threshold report array",
            ))?
            .cells
            .fill(Some(false));
        if owners.weather_owner.tot_thermal_report_periods > 0 {
            self.zone_heat_index_hour_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_thermal_report_periods)?,
            )?);
            self.zone_heat_index_occupied_hour_bins_rep_period =
                Some(SourceReportMatrix::allocate(
                    zones,
                    extent(owners.weather_owner.tot_thermal_report_periods)?,
                )?);
            self.zone_heat_index_occu_hour_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_thermal_report_periods)?,
            )?);
            self.zone_humidex_hour_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_thermal_report_periods)?,
            )?);
            self.zone_humidex_occupied_hour_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_thermal_report_periods)?,
            )?);
            self.zone_humidex_occu_hour_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_thermal_report_periods)?,
            )?);
            self.zone_cold_hour_of_safety_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_thermal_report_periods)?,
            )?);
            self.zone_heat_hour_of_safety_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_thermal_report_periods)?,
            )?);
            self.zone_unmet_degree_hour_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_thermal_report_periods)?,
            )?);
            self.zone_discomfort_wt_exceed_occu_hour_bins_rep_period =
                Some(SourceReportMatrix::allocate(
                    zones,
                    extent(owners.weather_owner.tot_thermal_report_periods)?,
                )?);
            self.zone_discomfort_wt_exceed_occupied_hour_bins_rep_period =
                Some(SourceReportMatrix::allocate(
                    zones,
                    extent(owners.weather_owner.tot_thermal_report_periods)?,
                )?);
        }
        if owners.weather_owner.tot_co2_report_periods > 0 {
            self.zone_c_o2_level_hour_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_co2_report_periods)?,
            )?);
            self.zone_c_o2_level_occu_hour_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_co2_report_periods)?,
            )?);
            self.zone_c_o2_level_occupied_hour_bins_rep_period =
                Some(SourceReportMatrix::allocate(
                    zones,
                    extent(owners.weather_owner.tot_co2_report_periods)?,
                )?);
        }
        if owners.weather_owner.tot_visual_report_periods > 0 {
            self.zone_lighting_level_hour_bins_rep_period = Some(SourceReportMatrix::allocate(
                zones,
                extent(owners.weather_owner.tot_visual_report_periods)?,
            )?);
            self.zone_lighting_level_occu_hour_bins_rep_period =
                Some(SourceReportMatrix::allocate(
                    zones,
                    extent(owners.weather_owner.tot_visual_report_periods)?,
                )?);
            self.zone_lighting_level_occupied_hour_bins_rep_period =
                Some(SourceReportMatrix::allocate(
                    zones,
                    extent(owners.weather_owner.tot_visual_report_periods)?,
                )?);
        }
        self.zone_low_s_e_t_hours_rep_period = Some(SourceReportMatrix::allocate(
            zones,
            extent(owners.weather_owner.tot_thermal_report_periods)?,
        )?);
        self.zone_high_s_e_t_hours_rep_period = Some(SourceReportMatrix::allocate(
            zones,
            extent(owners.weather_owner.tot_thermal_report_periods)?,
        )?);
        self.low_s_e_t_longest_hours_rep_period = Some(SourceReportMatrix::allocate(
            zones,
            extent(owners.weather_owner.tot_thermal_report_periods)?,
        )?);
        self.high_s_e_t_longest_hours_rep_period = Some(SourceReportMatrix::allocate(
            zones,
            extent(owners.weather_owner.tot_thermal_report_periods)?,
        )?);
        self.low_s_e_t_longest_start_rep_period = Some(SourceReportMatrix::allocate(
            zones,
            extent(owners.weather_owner.tot_thermal_report_periods)?,
        )?);
        self.high_s_e_t_longest_start_rep_period = Some(SourceReportMatrix::allocate(
            zones,
            extent(owners.weather_owner.tot_thermal_report_periods)?,
        )?);
        self.report.count = 0;
        Ok(())
    }
}
