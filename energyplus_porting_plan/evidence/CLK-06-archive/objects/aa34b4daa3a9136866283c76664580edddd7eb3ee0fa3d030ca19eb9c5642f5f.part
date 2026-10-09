//! Typed daily weather transport from WeatherManager.hh at the frozen source pin.
//!
//! These owners retain processed WeatherVars independently of raw EPW records.
//! Constructing or copying them performs no missing-value or physical projection.

use std::fmt::{Display, Formatter};

#[path = "auxiliary.rs"]
mod auxiliary;
#[path = "context.rs"]
mod context;
pub use auxiliary::{ExtendedWeatherVars, WeatherVarCounts};
pub use context::{WeatherEnvironmentState, WeatherGlobalState, WeatherOwnerState};

/// Source `DayWeatherVariables`: eight integers and three unmodified reals.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct DailyWeatherVariables {
    /// Source weather day ordinal.
    pub day_of_year: i32,
    /// Source schedule day ordinal, separately carried from the weather ordinal.
    pub day_of_year_schedule: i32,
    /// Source calendar year.
    pub year: i32,
    /// Source calendar month.
    pub month: i32,
    /// Source day of month.
    pub day_of_month: i32,
    /// Source weekday integer.
    pub day_of_week: i32,
    /// Source daylight-saving index.
    pub daylight_saving_index: i32,
    /// Source holiday index; negative caller canaries remain valid storage.
    pub holiday_index: i32,
    /// Already supplied sine of solar declination.
    pub sin_solar_declin_angle: f64,
    /// Already supplied cosine of solar declination.
    pub cos_solar_declin_angle: f64,
    /// Already supplied equation of time.
    pub equation_of_time: f64,
}

impl DailyWeatherVariables {
    /// Returns the integer members in native declaration order.
    #[must_use]
    pub fn integer_values(&self) -> [i32; 8] {
        [
            self.day_of_year,
            self.day_of_year_schedule,
            self.year,
            self.month,
            self.day_of_month,
            self.day_of_week,
            self.daylight_saving_index,
            self.holiday_index,
        ]
    }

    /// Returns the real members without changing their binary64 representation.
    #[must_use]
    pub fn real_values(&self) -> [f64; 3] {
        [
            self.sin_solar_declin_angle,
            self.cos_solar_declin_angle,
            self.equation_of_time,
        ]
    }
}

/// Complete source `WeatherVars`, including fields absent from the hourly EPW DTO.
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct WeatherVars {
    /// Source rain indicator.
    pub is_rain: bool,
    /// Source snow indicator.
    pub is_snow: bool,
    /// Outdoor dry-bulb temperature.
    pub out_dry_bulb_temp: f64,
    /// Outdoor dew-point temperature.
    pub out_dew_point_temp: f64,
    /// Outdoor barometric pressure.
    pub out_baro_press: f64,
    /// Outdoor relative humidity.
    pub out_rel_hum: f64,
    /// Wind speed.
    pub wind_speed: f64,
    /// Wind direction.
    pub wind_dir: f64,
    /// Sky temperature.
    pub sky_temp: f64,
    /// Horizontal infrared sky radiation.
    pub horiz_ir_sky: f64,
    /// Beam solar radiation.
    pub beam_solar_rad: f64,
    /// Diffuse solar radiation.
    pub dif_solar_rad: f64,
    /// Surface albedo.
    pub albedo: f64,
    /// Source water precipitation storage, distinct from liquid precipitation.
    pub water_precip: f64,
    /// Source liquid precipitation storage.
    pub liquid_precip: f64,
    /// Total sky cover.
    pub total_sky_cover: f64,
    /// Opaque sky cover.
    pub opaque_sky_cover: f64,
}

impl WeatherVars {
    /// Returns source boolean members in declaration order.
    #[must_use]
    pub fn boolean_values(&self) -> [bool; 2] {
        [self.is_rain, self.is_snow]
    }

    /// Returns all fifteen source reals without projection or arithmetic.
    #[must_use]
    pub fn real_values(&self) -> [f64; 15] {
        [
            self.out_dry_bulb_temp,
            self.out_dew_point_temp,
            self.out_baro_press,
            self.out_rel_hum,
            self.wind_speed,
            self.wind_dir,
            self.sky_temp,
            self.horiz_ir_sky,
            self.beam_solar_rad,
            self.dif_solar_rad,
            self.albedo,
            self.water_precip,
            self.liquid_precip,
            self.total_sky_cover,
            self.opaque_sky_cover,
        ]
    }
}

/// Storage admission failures; these are not original EnergyPlus fatal outcomes.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WeatherDayValuesError {
    /// An allocated day requires at least one timestep per hour.
    ZeroTimeSteps,
    /// The requested day shape cannot be represented by Rust indexing.
    SizeOverflow,
    /// The requested allocation could not be reserved.
    AllocationFailed,
    /// A caller requested a slot from an unallocated owner.
    Unallocated,
    /// Native one-based hour indexing admits only hours 1 through 24.
    HourOutsideDay(usize),
}

impl Display for WeatherDayValuesError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::ZeroTimeSteps => formatter.write_str("weather day requires timesteps per hour"),
            Self::SizeOverflow => formatter.write_str("weather day allocation size overflow"),
            Self::AllocationFailed => formatter.write_str("weather day allocation failed"),
            Self::Unallocated => formatter.write_str("weather day values are unallocated"),
            Self::HourOutsideDay(hour) => {
                write!(formatter, "weather hour {hour} is outside 1 through 24")
            }
        }
    }
}

impl std::error::Error for WeatherDayValuesError {}

/// Owned source day array, serialized hour-major and timestep-minor.
///
/// The unallocated default is distinct from an allocated zero-filled day.
/// Whole-owner clones retain allocation, shape, all slots, and signed-zero bits.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WeatherDayValues {
    time_steps: usize,
    slots: Vec<WeatherVars>,
}

impl WeatherDayValues {
    /// Allocates the same 24-hour shape as `AllocateWeatherData`.
    pub fn allocated(time_steps: usize) -> Result<Self, WeatherDayValuesError> {
        if time_steps == 0 {
            return Err(WeatherDayValuesError::ZeroTimeSteps);
        }
        let size = time_steps
            .checked_mul(24)
            .ok_or(WeatherDayValuesError::SizeOverflow)?;
        let mut slots = Vec::new();
        slots
            .try_reserve_exact(size)
            .map_err(|_| WeatherDayValuesError::AllocationFailed)?;
        slots.resize(size, WeatherVars::default());
        Ok(Self { time_steps, slots })
    }

    /// Whether the day array has been allocated.
    #[must_use]
    pub fn is_allocated(&self) -> bool {
        self.time_steps != 0
    }

    /// Source array dimension one, zero while unallocated.
    #[must_use]
    pub fn time_steps(&self) -> usize {
        self.time_steps
    }

    /// Source array dimension two, zero while unallocated.
    #[must_use]
    pub fn hours(&self) -> usize {
        if self.is_allocated() { 24 } else { 0 }
    }

    /// All stored slots in hour-major, timestep-minor order.
    #[must_use]
    pub fn slots(&self) -> &[WeatherVars] {
        &self.slots
    }

    /// All stored slots; the caller owns all writes and their source ordering.
    pub fn slots_mut(&mut self) -> &mut [WeatherVars] {
        &mut self.slots
    }

    /// Returns all timestep slots for one native one-based hour.
    pub fn hour(&self, hour: usize) -> Result<&[WeatherVars], WeatherDayValuesError> {
        let range = self.hour_range(hour)?;
        Ok(&self.slots[range])
    }

    /// Returns slots without clearing them; native read callbacks clear only their interval.
    pub fn hour_mut(&mut self, hour: usize) -> Result<&mut [WeatherVars], WeatherDayValuesError> {
        let range = self.hour_range(hour)?;
        Ok(&mut self.slots[range])
    }

    fn hour_range(&self, hour: usize) -> Result<std::ops::Range<usize>, WeatherDayValuesError> {
        if !self.is_allocated() {
            return Err(WeatherDayValuesError::Unallocated);
        }
        if !(1..=24).contains(&hour) {
            return Err(WeatherDayValuesError::HourOutsideDay(hour));
        }
        let start = (hour - 1) * self.time_steps;
        Ok(start..start + self.time_steps)
    }
}

/// Selected owned weather state shared by daily transport and its real caller.
///
/// Raw header/parser/stream owners remain separate. Their counters and cursor are
/// not modified by the pure transport operation.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct WeatherDayState {
    /// Current daily variables.
    pub today_variables: DailyWeatherVariables,
    /// Next daily variables retained until the caller overwrites them.
    pub tomorrow_variables: DailyWeatherVariables,
    /// Current processed timestep slots.
    pub today_values: WeatherDayValues,
    /// Next processed timestep slots.
    pub tomorrow_values: WeatherDayValues,
    /// Source prior-hour cache.
    pub last_hour: WeatherVars,
    /// Source next-hour cache.
    pub next_hour: WeatherVars,
    /// Complete source missing-value storage.
    pub missing_values: ExtendedWeatherVars,
    /// Complete source missing-value counters.
    pub missed_counts: WeatherVarCounts,
    /// Complete source range counters.
    pub out_of_range_counts: WeatherVarCounts,
    /// Selected global flags, counters, and caller clocks.
    pub global: WeatherGlobalState,
    /// Selected current and tomorrow environment fields.
    pub environment: WeatherEnvironmentState,
    /// Selected weather-owner flags, counters, and read context.
    pub weather: WeatherOwnerState,
}
