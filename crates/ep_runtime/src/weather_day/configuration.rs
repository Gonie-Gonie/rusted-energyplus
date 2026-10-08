//! Calendar and input controls supplied to the live weather reader.

use super::{WeatherDayError, WeatherEnvironmentState};
use crate::time_axis::{TimeAxis, TimePoint};
use ep_model::{DayOfWeek, FirstHourInterpolationStartingValues, RunPeriod, SiteLocation};

/// Literal selected weather controls registered by the actual caller.
///
/// The initializer matches DataEnvironment.hh:200-203. Effective production
/// values must be resolved from captured environment inputs and raw diagnostics.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WeatherSolarControls {
    /// Source DisplayWeatherMissingDataWarnings.
    pub display_weather_missing_data_warnings: bool,
    /// Source IgnoreSolarRadiation.
    pub ignore_solar_radiation: bool,
    /// Source IgnoreBeamRadiation.
    pub ignore_beam_radiation: bool,
    /// Source IgnoreDiffuseRadiation.
    pub ignore_diffuse_radiation: bool,
}

impl WeatherSolarControls {
    pub(super) fn apply_to(self, owner: &mut WeatherEnvironmentState) {
        owner.display_weather_missing_data_warnings = self.display_weather_missing_data_warnings;
        owner.ignore_solar_radiation = self.ignore_solar_radiation;
        owner.ignore_beam_radiation = self.ignore_beam_radiation;
        owner.ignore_diffuse_radiation = self.ignore_diffuse_radiation;
    }
}

/// Calendar metadata and input controls, containing no selected weather values.
#[derive(Clone, Debug)]
pub struct WeatherEnvironmentConfiguration {
    /// Weather-aware hourly axis, used only for caller clocks and date rules.
    pub time_axis: TimeAxis,
    /// Literal input run period, independent of weather-file raw years.
    pub run_period: RunPeriod,
    /// Site location already admitted by the model compiler.
    pub site: SiteLocation,
    /// Caller-prepared number of preceding design-day environments.
    pub design_day_count: i32,
    /// Caller-prepared count of input-file special-day objects.
    pub input_special_day_count: i32,
    /// Explicitly resolved caller controls, copied before the first source read.
    pub solar_controls: WeatherSolarControls,
}

impl WeatherEnvironmentConfiguration {
    /// Uses the same default RunPeriod policy as the existing time-axis owner.
    pub fn for_model(
        time_axis: &TimeAxis,
        model: &ep_model::TypedModel,
        fallback_site: &SiteLocation,
        design_day_count: i32,
    ) -> Result<Self, WeatherDayError> {
        let fallback_run_period = crate::time_axis::default_run_period();
        let run_period = model.run_periods.first().unwrap_or(&fallback_run_period);
        Self::new(
            time_axis,
            run_period,
            model.site.as_ref().unwrap_or(fallback_site),
            design_day_count,
            model.run_period_special_days.len() as i32,
        )
    }

    /// Retains calendar metadata without accepting a physical weather series.
    pub fn new(
        time_axis: &TimeAxis,
        run_period: &RunPeriod,
        site: &SiteLocation,
        design_day_count: i32,
        input_special_day_count: i32,
    ) -> Result<Self, WeatherDayError> {
        if time_axis.zone_timestep.timesteps_per_hour == 0
            || time_axis.weather_calendar.is_none()
            || time_axis.points.is_empty()
            || !time_axis.points.len().is_multiple_of(24)
        {
            return Err(WeatherDayError::admission(
                "live weather requires a complete weather-aware hourly axis",
            ));
        }
        if run_period.treat_weather_as_actual {
            return Err(WeatherDayError::admission(
                "actual-year live weather is outside the current bounded port",
            ));
        }
        if design_day_count < 0 || input_special_day_count < 0 {
            return Err(WeatherDayError::admission(
                "negative caller environment counts",
            ));
        }
        Ok(Self {
            time_axis: time_axis.clone(),
            run_period: run_period.clone(),
            site: site.clone(),
            design_day_count,
            input_special_day_count,
            solar_controls: WeatherSolarControls::default(),
        })
    }

    /// Effective environment length, independent of raw input positions.
    #[must_use]
    pub fn total_days(&self) -> usize {
        self.time_axis.points.len() / 24
    }

    /// Number of zone timesteps in an hour.
    #[must_use]
    pub fn steps(&self) -> u32 {
        self.time_axis.zone_timestep.timesteps_per_hour
    }

    /// Selected first-hour interpolation policy.
    #[must_use]
    pub fn first_policy(&self) -> FirstHourInterpolationStartingValues {
        self.time_axis.first_hour_interpolation_starting_values
    }

    pub(super) fn day_point(&self, day: usize) -> Option<&TimePoint> {
        self.time_axis
            .points
            .get(day.checked_sub(1)?.checked_mul(24)?)
    }
}

pub(super) fn weekday_index(value: DayOfWeek) -> i32 {
    match value {
        DayOfWeek::Sunday => 1,
        DayOfWeek::Monday => 2,
        DayOfWeek::Tuesday => 3,
        DayOfWeek::Wednesday => 4,
        DayOfWeek::Thursday => 5,
        DayOfWeek::Friday => 6,
        DayOfWeek::Saturday => 7,
    }
}

pub(super) fn ordinal(month: i32, day: i32, leap: i32) -> Result<i32, String> {
    let ends = [31, 28 + leap, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    let index = usize::try_from(month - 1).map_err(|_| "weather month outside ordinal domain")?;
    let end = ends
        .get(index)
        .ok_or("weather month outside ordinal domain")?;
    // The raw parser separately admits February 29 without a leap-shaped year.
    if day < 1 || day > *end + i32::from(month == 2 && leap == 0) {
        return Err("weather day outside ordinal domain".into());
    }
    Ok(ends[..index].iter().sum::<i32>() + day)
}

pub(super) fn between(value: i32, start: i32, end: i32) -> bool {
    if start <= end {
        value >= start && value <= end
    } else {
        value >= start || value <= end
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn explicit_controls_copy_without_overwriting_calendar_or_current_weather() {
        let controls = WeatherSolarControls {
            display_weather_missing_data_warnings: true,
            ignore_solar_radiation: false,
            ignore_beam_radiation: true,
            ignore_diffuse_radiation: false,
        };
        let mut owner = WeatherEnvironmentState {
            ignore_solar_radiation: true,
            ignore_diffuse_radiation: true,
            day_of_year: 123,
            std_baro_press: -0.0,
            ..WeatherEnvironmentState::default()
        };
        let prior_current = owner.current_weather;
        controls.apply_to(&mut owner);
        assert!(owner.display_weather_missing_data_warnings);
        assert!(!owner.ignore_solar_radiation);
        assert!(owner.ignore_beam_radiation);
        assert!(!owner.ignore_diffuse_radiation);
        assert_eq!(owner.day_of_year, 123);
        assert_eq!(owner.std_baro_press.to_bits(), (-0.0_f64).to_bits());
        assert_eq!(owner.current_weather, prior_current);
    }
}
