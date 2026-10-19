//! Actual SetCurrentWeather2152–2182 site-schedule reset and selected live modifier reads.
use super::state::SourceSizingPeriodSiteSchedule;
use super::{Error, SourceDesignDayInputState, SourceEnvironmentData, WeatherGlobalState, ordinal};
use crate::weather::day::WeatherEnvironmentState;
impl SourceDesignDayInputState {
    pub(crate) fn update_source_site_schedule_values(
        &mut self,
        clock: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        current_environment: i32,
        environments: Option<&[SourceEnvironmentData]>,
    ) -> Result<(), Error> {
        if matches!(clock.kind_of_sim, 1 | 4) {
            self.reset_source_site_schedule_values(environment.tot_des_days)?;
            let env = environments
                .and_then(|v| v.get(ordinal(current_environment).ok()?))
                .ok_or(Error::Unavailable("actual current Environment row2157"))?;
            let number = env.design_day_number;
            let input = self
                .design_day_input
                .as_ref()
                .and_then(|v| v.get(ordinal(number).ok()?))
                .ok_or(Error::Unavailable("actual current DesDayInput row2159"))?;
            let schedule = self
                .sizing_period_site_schedules
                .get_mut(ordinal(number)?)
                .ok_or(Error::Unavailable("actual current SPSiteSchedules row2160"))?;
            let modifier = self
                .design_day_modifiers
                .as_ref()
                .and_then(|v| v.get(ordinal(number).ok()?))
                .and_then(Option::as_ref)
                .ok_or(Error::Unavailable("actual current modifier grid2161"))?
                .cell(clock.time_step, clock.hour_of_day)?;
            if input.dry_bulb_range_type != 0 {
                schedule.outdoor_dry_bulb_temperature = modifier.outdoor_dry_bulb_temperature;
            }
            if matches!(input.humidity_indicator_type, 4..=7) {
                schedule.outdoor_relative_humidity = modifier.outdoor_relative_humidity;
            }
            if input.solar_model == 2 {
                schedule.beam_solar_radiation = modifier.beam_solar_radiation;
                schedule.diffuse_solar_radiation = modifier.diffuse_solar_radiation;
            }
            if matches!(env.sky_temperature_model, 1..=3) {
                schedule.sky_temperature = modifier.sky_temperature;
            }
        } else if environment.tot_des_days > 0 {
            self.reset_source_site_schedule_values(environment.tot_des_days)?;
        }
        Ok(())
    }
    fn reset_source_site_schedule_values(&mut self, count: i32) -> Result<(), Error> {
        for number in 1..=count {
            let row = self
                .sizing_period_site_schedules
                .get_mut(ordinal(number)?)
                .ok_or(Error::Unavailable(
                    "actual SPSiteSchedules reset allocation2154",
                ))?;
            *row = SourceSizingPeriodSiteSchedule {
                outdoor_dry_bulb_temperature: -999.0,
                outdoor_relative_humidity: -999.0,
                beam_solar_radiation: -999.0,
                diffuse_solar_radiation: -999.0,
                sky_temperature: -999.0,
            };
        }
        Ok(())
    }
}
