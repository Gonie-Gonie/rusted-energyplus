//! Complete genuine native constructors, independent of selected/config DD owners.
use super::Error;
use std::convert::Infallible;

/// Native DesignDayData225–256, including the four actual null day-schedule pointers.
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceDesignDayData {
    pub(crate) title: String,
    pub(crate) maximum_dry_bulb: f64,
    pub(crate) daily_dry_bulb_range: f64,
    pub(crate) humidity_indicator_value: f64,
    pub(crate) humidity_indicator_type: i32,
    pub(crate) barometric_pressure: f64,
    pub(crate) wind_speed: f64,
    pub(crate) wind_direction: f64,
    pub(crate) sky_clearness: f64,
    pub(crate) rain_indicator: i32,
    pub(crate) snow_indicator: i32,
    pub(crate) day_of_month: i32,
    pub(crate) month: i32,
    pub(crate) day_type: i32,
    pub(crate) daylight_saving_indicator: i32,
    pub(crate) solar_model: i32,
    pub(crate) dry_bulb_range_type: i32,
    pub(crate) temperature_range_schedule: Option<Infallible>,
    pub(crate) humidity_indicator_schedule: Option<Infallible>,
    pub(crate) beam_solar_schedule: Option<Infallible>,
    pub(crate) diffuse_solar_schedule: Option<Infallible>,
    pub(crate) tau_beam: f64,
    pub(crate) tau_diffuse: f64,
    pub(crate) daily_wet_bulb_range: f64,
    pub(crate) pressure_entered: bool,
    pub(crate) dew_point_needs_set: bool,
    pub(crate) maximum_warmup_days: i32,
    pub(crate) suppress_begin_environment_reset: bool,
}
impl Default for SourceDesignDayData {
    fn default() -> Self {
        Self {
            title: String::new(),
            maximum_dry_bulb: 0.0,
            daily_dry_bulb_range: 0.0,
            humidity_indicator_value: 0.0,
            humidity_indicator_type: 0,
            barometric_pressure: 0.0,
            wind_speed: 0.0,
            wind_direction: 0.0,
            sky_clearness: 0.0,
            rain_indicator: 0,
            snow_indicator: 0,
            day_of_month: 0,
            month: 0,
            day_type: 0,
            daylight_saving_indicator: 0,
            solar_model: 0,
            dry_bulb_range_type: 0,
            temperature_range_schedule: None,
            humidity_indicator_schedule: None,
            beam_solar_schedule: None,
            diffuse_solar_schedule: None,
            tau_beam: 0.0,
            tau_diffuse: 0.0,
            daily_wet_bulb_range: 0.0,
            pressure_entered: false,
            dew_point_needs_set: false,
            maximum_warmup_days: -1,
            suppress_begin_environment_reset: false,
        }
    }
}
/// Native DesDayMods746–752; its Array2D is allocated independently for each DD.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SourceDesignDayModifiers {
    pub(crate) outdoor_dry_bulb_temperature: f64,
    pub(crate) outdoor_relative_humidity: f64,
    pub(crate) beam_solar_radiation: f64,
    pub(crate) diffuse_solar_radiation: f64,
    pub(crate) sky_temperature: f64,
}
/// Native SPSiteSchedules754–762, not a copy of a modifier row or report snapshot.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub(crate) struct SourceSizingPeriodSiteSchedule {
    pub(crate) outdoor_dry_bulb_temperature: f64,
    pub(crate) outdoor_relative_humidity: f64,
    pub(crate) beam_solar_radiation: f64,
    pub(crate) diffuse_solar_radiation: f64,
    pub(crate) sky_temperature: f64,
}
#[derive(Clone, Debug, PartialEq)]
pub(crate) struct SourceDesignDayModifierGrid {
    pub(crate) time_steps_in_hour: i32,
    pub(crate) values: Vec<SourceDesignDayModifiers>,
}
impl SourceDesignDayModifierGrid {
    pub(super) fn cell(
        &self,
        timestep: i32,
        hour: i32,
    ) -> Result<&SourceDesignDayModifiers, Error> {
        if timestep < 1 || timestep > self.time_steps_in_hour || !(1..=24).contains(&hour) {
            return Err(Error::Unavailable(
                "actual DesignDay modifier Array2D bounds",
            ));
        }
        let index = (timestep - 1)
            .checked_mul(24)
            .and_then(|v| v.checked_add(hour - 1))
            .and_then(|v| usize::try_from(v).ok())
            .ok_or(Error::Unavailable(
                "actual DesignDay modifier Array2D offset",
            ))?;
        self.values
            .get(index)
            .ok_or(Error::Unavailable("actual DesignDay modifier allocation"))
    }
    pub(super) fn cell_mut(
        &mut self,
        timestep: i32,
        hour: i32,
    ) -> Result<&mut SourceDesignDayModifiers, Error> {
        if timestep < 1 || timestep > self.time_steps_in_hour || !(1..=24).contains(&hour) {
            return Err(Error::Unavailable(
                "actual DesignDay modifier Array2D bounds",
            ));
        }
        let i = (timestep - 1)
            .checked_mul(24)
            .and_then(|v| v.checked_add(hour - 1))
            .and_then(|v| usize::try_from(v).ok())
            .ok_or(Error::Unavailable(
                "actual DesignDay modifier Array2D offset",
            ))?;
        self.values
            .get_mut(i)
            .ok_or(Error::Unavailable("actual DesignDay modifier allocation"))
    }
}
/// Sole Source compartment of WeatherManager's actual three arrays and vector.
/// EPVector allocate(0) remains Some(empty); SPSiteSchedules.dimension(0) is
/// constructor-empty/not allocated because EPVector::dimension never sets m_allocated.
#[derive(Clone, Debug, Default, PartialEq)]
pub(crate) struct SourceDesignDayInputState {
    pub(crate) design_day_input: Option<Vec<SourceDesignDayData>>,
    pub(crate) design_day_modifiers: Option<Vec<Option<SourceDesignDayModifierGrid>>>,
    pub(crate) sizing_period_site_schedules: Vec<SourceSizingPeriodSiteSchedule>,
    pub(crate) sizing_period_site_schedule_numbers: Vec<i32>,
}
