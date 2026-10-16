//! Actual UpdatePrecipitation947–990 on the sole WaterManager/DataWater and EcoRoof owners.
use super::{Error, SourceRainfallMode, SourceWaterManager, month_index};
use crate::heat_balance::selected_schedule_manager::SelectedScheduleManager;
use crate::weather::day::{WeatherEnvironmentState, WeatherGlobalState};

/// Bounded actual EcoRoofManagerData compartment, not WeatherManager storage.
/// Native header107 has no explicit scalar initializer; no value is fabricated
/// before first actual UpdatePrecipitation assignment. Other EcoRoof fields are
/// outside this admitted route and cannot be consumed through this compartment.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceEcoRoofData {
    pub(crate) current_precipitation: Option<f64>,
}
impl SourceWaterManager {
    pub(crate) fn update_source_precipitation(
        &mut self,
        clock: &WeatherGlobalState,
        environment: &WeatherEnvironmentState,
        schedules: &SelectedScheduleManager,
        eco_roof: &mut SourceEcoRoofData,
    ) -> Result<(), Error> {
        if self.data.rainfall.mode == SourceRainfallMode::RainSchedDesign {
            let index = self.data.rainfall.rain_schedule.ok_or(Error::Unavailable(
                "actual UpdatePrecipitation rainSched pointer962",
            ))?;
            let rate = schedules.source_current_schedule_value(index)?;
            let scale = if self.data.rainfall.nominal_annual_rain > 0.0 {
                self.data.rainfall.design_annual_rain / self.data.rainfall.nominal_annual_rain
            } else {
                0.0
            };
            self.data.rainfall.current_rate = rate * scale / 3600.0;
        } else if environment.current_weather.liquid_precipitation > 0.0 {
            self.data.rainfall.current_rate =
                environment.current_weather.liquid_precipitation / clock.time_step_zone_sec;
        } else {
            self.data.rainfall.current_rate = 0.0;
        }
        self.data.rainfall.current_amount =
            self.data.rainfall.current_rate * clock.time_step_zone_sec;
        eco_roof.current_precipitation = Some(self.data.rainfall.current_amount);
        if self.data.rainfall.mode == SourceRainfallMode::RainSchedDesign
            && environment.run_period_environment
            && !clock.warmup_flag
        {
            let month = month_index(environment.month)?;
            self.data.rainfall.monthly_total_precipitation_site[month] +=
                self.data.rainfall.current_amount * 1000.0;
        }
        Ok(())
    }
}
