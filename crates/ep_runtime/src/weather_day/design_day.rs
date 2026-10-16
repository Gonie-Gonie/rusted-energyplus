//! Actual selected DesignDay input and WeatherManager producer ownership.
//! DefaultMultipliers, WetBulb/DewPoint, ASHRAEClearSky, default ClarkAllen.

mod producer;
mod reports;
pub(super) mod solar;

use super::{DailyWeatherVariables, WeatherDayError, WeatherDayState};

/// Source humidity indicator on the admitted ordinary input domain.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum DesignDayHumidity {
    /// Constant humidity from maximum DB and entered WB.
    WetBulb,
    /// Constant humidity from entered DP.
    DewPoint,
}

/// Actual selected GetDesignDayData fields; schedules/other models stay outside.
#[derive(Clone, Debug, PartialEq)]
pub struct DesignDayParameters {
    /// Uppercase actual getObjectItem name, in effective input order.
    pub name: String,
    /// Actual month.
    pub month: i32,
    /// Actual nonleap design-day date.
    pub day_of_month: i32,
    /// Actual schedule DayType integer, 1..12.
    pub day_type: i32,
    /// Actual maximum dry bulb, Celsius.
    pub maximum_dry_bulb: f64,
    /// Actual daily dry-bulb range.
    pub daily_dry_bulb_range: f64,
    /// Actual selected humidity indicator type.
    pub humidity_type: DesignDayHumidity,
    /// Actual WB/DP indicator at maximum DB.
    pub humidity_indicator: f64,
    /// Entered pressure, or the actual blank-field fact before setup resolution.
    pub barometric_pressure: Option<f64>,
    /// Actual speed, m/s.
    pub wind_speed: f64,
    /// Actual clockwise direction, degrees.
    pub wind_direction: f64,
    /// Actual rain input.
    pub rain: bool,
    /// Actual snow input.
    pub snow: bool,
    /// Actual DST input.
    pub daylight_saving: bool,
    /// Actual ASHRAE clear-sky clearness.
    pub sky_clearness: f64,
    /// Actual maximum-warmup override; -1 is the Source blank default.
    pub maximum_warmup_days: i32,
}

/// Sole mutable DesDayInput/DesignDay/desDayMods/spSiteSchedules projection.
#[derive(Clone, Debug, PartialEq)]
pub struct DesignDayInput {
    parameters: DesignDayParameters,
    pressure_entered: bool,
    resolved_pressure: f64,
    dry_bulb_modifiers: Vec<f64>,
    daily_variables: DailyWeatherVariables,
    pub(super) site_schedule_values: [f64; 5],
}

impl DesignDayInput {
    /// Actual DefaultTempRangeMult interpolation at GetDesignDayData6162-6172.
    ///
    /// # Errors
    /// Rejects unsupported diagnostics/invalid data before any reached owner write.
    pub fn from_selected_parameters(
        mut parameters: DesignDayParameters,
        steps: u32,
    ) -> Result<Self, WeatherDayError> {
        const MULTIPLIERS: [f64; 24] = [
            0.88, 0.92, 0.95, 0.98, 1.0, 0.98, 0.91, 0.74, 0.55, 0.38, 0.23, 0.13, 0.05, 0.00,
            0.00, 0.06, 0.14, 0.24, 0.39, 0.50, 0.59, 0.68, 0.75, 0.82,
        ];
        let ends = [31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
        let month = parameters
            .month
            .checked_sub(1)
            .and_then(|value| usize::try_from(value).ok());
        if steps == 0
            || steps > 60
            || 60 % steps != 0
            || parameters.name.is_empty()
            || month
                .and_then(|i| ends.get(i))
                .is_none_or(|&end| parameters.day_of_month < 1 || parameters.day_of_month > end)
            || !(1..=12).contains(&parameters.day_type)
            || !parameters.maximum_dry_bulb.is_finite()
            || !(-90.0..=70.0).contains(&parameters.maximum_dry_bulb)
            || !parameters.daily_dry_bulb_range.is_finite()
            || parameters.daily_dry_bulb_range < 0.0
            || parameters.maximum_dry_bulb - parameters.daily_dry_bulb_range < -90.0
            || !parameters.humidity_indicator.is_finite()
            || !(-90.0..=70.0).contains(&parameters.humidity_indicator)
            || parameters.humidity_indicator > parameters.maximum_dry_bulb
            || parameters
                .barometric_pressure
                .is_some_and(|p| !p.is_finite() || !(31000.0..=120000.0).contains(&p))
            || !parameters.wind_speed.is_finite()
            || !(0.0..=40.0).contains(&parameters.wind_speed)
            || !parameters.wind_direction.is_finite()
            || !(0.0..=360.0).contains(&parameters.wind_direction)
            || !parameters.sky_clearness.is_finite()
            || !(0.0..=1.2).contains(&parameters.sky_clearness)
        {
            return Err(WeatherDayError::admission(
                "DesignDay outside selected warning-free input domain",
            ));
        }
        parameters.wind_direction %= 360.0;
        let mut modifiers = Vec::with_capacity(24 * steps as usize);
        let mut last_hour = MULTIPLIERS[23];
        for current in MULTIPLIERS {
            for timestep in 1..=steps {
                let now = f64::from(timestep) / f64::from(steps);
                let previous = 1.0 - now;
                modifiers.push(last_hour * previous + current * now);
            }
            last_hour = current;
        }
        let pressure_entered = parameters.barometric_pressure.is_some();
        let resolved_pressure = parameters.barometric_pressure.unwrap_or(0.0);
        Ok(Self {
            parameters,
            pressure_entered,
            resolved_pressure,
            dry_bulb_modifiers: modifiers,
            daily_variables: DailyWeatherVariables::default(),
            site_schedule_values: [0.0; 5],
        })
    }

    /// Actual environment title.
    #[must_use]
    pub fn name(&self) -> &str {
        &self.parameters.name
    }

    /// Actual retained DesignDay daily owner, not a RunPeriod date projection.
    #[must_use]
    pub fn daily_variables(&self) -> &DailyWeatherVariables {
        &self.daily_variables
    }

    /// Actual DefaultMultipliers timestep values in hour-major order.
    #[must_use]
    pub fn dry_bulb_modifiers(&self) -> &[f64] {
        &self.dry_bulb_modifiers
    }

    /// Actual DesDayInput pressure after its reached setup branch.
    #[must_use]
    pub fn resolved_pressure(&self) -> f64 {
        self.resolved_pressure
    }

    /// Actual five spSiteSchedules diagnostics, independently retained per DD.
    #[must_use]
    pub fn site_schedule_values(&self) -> &[f64; 5] {
        &self.site_schedule_values
    }

    pub(super) fn reset_site_schedule_values(&mut self) {
        self.site_schedule_values = [-999.0; 5];
    }

    /// Source GetNextEnvironment775-847, on the real selected DesignDay owner.
    /// The initially default DesignDay ordinal and Environment.StartYear are
    /// zero; SetUpDesignDay later fills daily data using the actual wall year.
    pub fn select_environment(
        &self,
        state: &mut WeatherDayState,
        environment_ordinal: i32,
    ) -> Result<(), WeatherDayError> {
        if environment_ordinal < 1 || environment_ordinal > state.environment.tot_des_days {
            return Err(WeatherDayError::admission(
                "DesignDay environment ordinal outside actual owner",
            ));
        }
        state.weather.envrn = environment_ordinal;
        state.weather.dates_should_be_reset = false;
        state.global.kind_of_sim = 1;
        // Environment.StartJDay was copied from constructor DesignDay.DayOfYear.
        state.environment.day_of_year = 0;
        state.environment.day_of_month = self.parameters.day_of_month;
        state.global.calendar_year = 0;
        state.global.calendar_year_chr = "0".into();
        state.environment.month = self.parameters.month;
        state.global.num_of_day_in_envrn = 1;
        state.environment.environment_name = self.parameters.name.clone();
        state.environment.cur_envir_num = environment_ordinal;
        state.environment.run_period_start_day_of_week = 0;
        Ok(())
    }
}

#[cfg(test)]
pub(crate) use producer::{
    replay_dew_point, replay_humidity_from_dew_point, replay_relative_humidity,
};
