//! Genuine first-call reports and actual no-optional-model People weather accumulation.
//! Positive report-period, PierceSET and recurring mismatch children remain boundaries.
mod thermal_weather;
use super::*;
use crate::heat_balance::ctf_initialization_context::internal_heat_gains_input::SelectedInternalHeatGains;
use crate::heat_balance::error_backend::SourceErrorBackend;
use crate::heat_balance::ordinary_air_heat_balance::{
    SourceAirHeatBalance, environment::SourceZoneFanHeatBalanceArrays,
};
use crate::heat_balance::ordinary_daylighting::SourceDaylighting;
use crate::heat_balance::ordinary_heat_balance_initialization::SourceHeatBalanceManager;
use crate::heat_balance::ordinary_thermal_comfort::SourceThermalComfort;
use crate::heat_balance::selected_heat_balance_input::zone::SelectedZoneInputOwners;
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState, WeatherOwnerState};

pub(crate) struct SourceResilienceReportOwners<'a, 'backend> {
    pub(crate) globals: &'a SelectedHeatBalanceGlobals,
    pub(crate) zones: &'a mut SelectedZoneInputOwners,
    pub(crate) air: &'a SourceAirHeatBalance,
    pub(crate) fan: &'a SourceZoneFanHeatBalanceArrays,
    pub(crate) clock: &'a WeatherGlobalState,
    pub(crate) weather: &'a WeatherOwnerState,
    pub(crate) environment: &'a WeatherEnvironmentState,
    pub(crate) control: &'a SelectedHeatBalanceInputState,
    pub(crate) gains: &'a SelectedInternalHeatGains,
    pub(crate) schedules: &'a SelectedScheduleManager,
    pub(crate) comfort: &'a SourceThermalComfort,
    pub(crate) heat_balance: &'a mut SourceHeatBalanceManager,
    pub(crate) daylighting: &'a SourceDaylighting,
    pub(crate) tabular: &'a mut SourceTabularReports,
    pub(crate) backend: &'a mut SourceErrorBackend<'backend>,
}
// This macro borrows the real column-major SourceReportMatrix cell, not a report copy.
macro_rules! bins {($o:ident,$field:ident,$z:ident,$p:ident,$n:expr)=>{{
    let matrix=$o.heat_balance.$field.as_mut().ok_or(Error::Unavailable(concat!("actual allocated ",stringify!($field))))?;
    let zone=usize::try_from($z-1).map_err(|_|Error::Unavailable("actual resilience Zone ordinal"))?;
    let period=usize::try_from($p-1).map_err(|_|Error::Unavailable("actual resilience period ordinal"))?;
    if zone>=matrix.rows||period>=matrix.columns{return Err(Error::Unavailable("actual resilience report matrix domain"));}
    let cell=matrix.cells.get_mut(period*matrix.rows+zone).ok_or(Error::Unavailable("actual resilience report matrix cell"))?;
    *cell=vec![0.0;$n];
}};}
macro_rules! fill {
    ($o:ident,$field:ident,$value:expr) => {{
        $o.heat_balance
            .$field
            .as_mut()
            .ok_or(Error::Unavailable(concat!(
                "actual allocated ",
                stringify!($field)
            )))?
            .cells
            .fill(Some($value));
    }};
}
impl SourceSurfaceManagement {
    pub(crate) fn report_source_thermal_resilience(
        &mut self,
        o: &mut SourceResilienceReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if o.weather.tot_report_periods > 0 {
            let _report_period_flags =
                vec![
                    false;
                    usize::try_from(o.weather.tot_thermal_report_periods)
                        .map_err(|_| Error::Unavailable("actual thermal report count"))?
                ];
            return Err(Error::Unavailable(
                "actual General::findReportPeriodIdx5807 and thermal period input owners",
            ));
        }
        // Native reads a local unallocated reportPeriodFlags if these grouped
        // periods are inconsistent with TotReportPers; do not infer false flags.
        if o.weather.tot_thermal_report_periods > 0 {
            return Err(Error::Unavailable(
                "actual thermal reportPeriodFlags5812 and meter owner",
            ));
        }
        if self.report_thermal_resilience_first_time {
            if o.gains.totals[0] == 0 {
                self.has_pierce_set = false;
            }
            for people in 1..=o.gains.totals[0] {
                let at = usize::try_from(people - 1)
                    .map_err(|_| Error::Unavailable("actual People ordinal"))?;
                if !o
                    .gains
                    .allocations
                    .people
                    .get(at)
                    .ok_or(Error::Unavailable("actual live People.Pierce5831"))?
                    .pierce
                {
                    self.has_pierce_set = false;
                }
            }
            for zone in 1..=o.globals.num_of_zones {
                for period in 1..=o.weather.tot_thermal_report_periods {
                    bins!(o, zone_heat_index_hour_bins_rep_period, zone, period, 5);
                    bins!(
                        o,
                        zone_heat_index_occu_hour_bins_rep_period,
                        zone,
                        period,
                        5
                    );
                    bins!(
                        o,
                        zone_heat_index_occupied_hour_bins_rep_period,
                        zone,
                        period,
                        5
                    );
                    bins!(o, zone_humidex_hour_bins_rep_period, zone, period, 5);
                    bins!(
                        o,
                        zone_humidex_occupied_hour_bins_rep_period,
                        zone,
                        period,
                        5
                    );
                    bins!(o, zone_humidex_occu_hour_bins_rep_period, zone, period, 5);
                    if self.has_pierce_set {
                        bins!(o, zone_low_s_e_t_hours_rep_period, zone, period, 5);
                        bins!(o, zone_high_s_e_t_hours_rep_period, zone, period, 5);
                    }
                    bins!(o, zone_cold_hour_of_safety_bins_rep_period, zone, period, 5);
                    bins!(o, zone_heat_hour_of_safety_bins_rep_period, zone, period, 5);
                    bins!(o, zone_unmet_degree_hour_bins_rep_period, zone, period, 6);
                    bins!(
                        o,
                        zone_discomfort_wt_exceed_occu_hour_bins_rep_period,
                        zone,
                        period,
                        4
                    );
                    bins!(
                        o,
                        zone_discomfort_wt_exceed_occupied_hour_bins_rep_period,
                        zone,
                        period,
                        4
                    );
                }
                fill!(o, low_s_e_t_longest_hours_rep_period, 0.0);
                fill!(o, high_s_e_t_longest_hours_rep_period, 0.0);
                fill!(o, low_s_e_t_longest_start_rep_period, 0);
                fill!(o, high_s_e_t_longest_start_rep_period, 0);
            }
            let count = usize::try_from(o.globals.num_of_zones)
                .map_err(|_| Error::Unavailable("actual NumOfZones longest SET extent"))?;
            self.low_set_longest_hours = vec![0.0; count];
            self.high_set_longest_hours = vec![0.0; count];
            self.low_set_longest_start = vec![0; count];
            self.high_set_longest_start = vec![0; count];
            self.report_thermal_resilience_first_time = false;
        }
        if o.clock.kind_of_sim == 3 && !o.clock.warmup_flag {
            self.report_source_thermal_weather(o)?;
        }
        Ok(())
    }
    pub(crate) fn report_source_co2_resilience(
        &mut self,
        o: &mut SourceResilienceReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.report_co2_resilience_first_time {
            for zone in 1..=o.globals.num_of_zones {
                for period in 1..=o.weather.tot_co2_report_periods {
                    bins!(o, zone_c_o2_level_hour_bins_rep_period, zone, period, 3);
                    bins!(
                        o,
                        zone_c_o2_level_occu_hour_bins_rep_period,
                        zone,
                        period,
                        3
                    );
                    bins!(
                        o,
                        zone_c_o2_level_occupied_hour_bins_rep_period,
                        zone,
                        period,
                        3
                    );
                }
            }
            self.report_co2_resilience_first_time = false;
            if !o.control.carbon_dioxide_simulation {
                if o.tabular.display_co2_resilience_summary_explicitly {
                    o.backend.show_warning_error("Writing Annual CO2 Resilience Summary - CO2 Level Hours reports: Zone Air CO2 Concentration output is required, but no ZoneAirContaminantBalance object is defined.")?;
                }
                o.tabular.display_co2_resilience_summary = false;
                return Ok(());
            }
        }
        if o.clock.kind_of_sim == 3 && !o.clock.warmup_flag {
            return Err(Error::Unavailable(
                "actual CO2 weather accumulation6415-6477",
            ));
        }
        Ok(())
    }
    pub(crate) fn report_source_visual_resilience(
        &mut self,
        o: &mut SourceResilienceReportOwners<'_, '_>,
    ) -> Result<(), Error> {
        if self.report_visual_resilience_first_time {
            for zone in 1..=o.globals.num_of_zones {
                for period in 1..=o.weather.tot_visual_report_periods {
                    bins!(o, zone_lighting_level_hour_bins_rep_period, zone, period, 4);
                    bins!(
                        o,
                        zone_lighting_level_occu_hour_bins_rep_period,
                        zone,
                        period,
                        4
                    );
                    bins!(
                        o,
                        zone_lighting_level_occupied_hour_bins_rep_period,
                        zone,
                        period,
                        4
                    );
                }
            }
            self.report_visual_resilience_first_time = false;
            if o.daylighting.source_daylight_control_count() == 0 {
                if o.tabular.display_visual_resilience_summary_explicitly {
                    o.backend.show_warning_error("Writing Annual Visual Resilience Summary - Lighting Level Hours reports: Zone Average Daylighting Reference Point Illuminance output is required, but no Daylighting Control Object is defined.")?;
                }
                o.tabular.display_visual_resilience_summary = false;
                return Ok(());
            }
        }
        if o.clock.kind_of_sim == 3 && !o.clock.warmup_flag {
            return Err(Error::Unavailable(
                "actual visual weather accumulation6506-6601",
            ));
        }
        Ok(())
    }
}
