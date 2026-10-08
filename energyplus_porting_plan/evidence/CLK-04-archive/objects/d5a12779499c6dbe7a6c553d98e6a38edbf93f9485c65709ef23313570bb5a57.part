//! Selected hourly missing values and history from WeatherManager.cc:2880–3040.
//!
//! The raw parser and its date admission run before this callback. Optional sky,
//! snow, solar and albedo fields keep their existing compatibility policy.

use super::producer::weather_vars_from_raw;
use super::{WeatherDayState, WeatherVars};
use crate::weather::EpwRecord;
use crate::weather::raw::RawEpwOutputs;

/// Processes one admitted raw hour exactly once, before the next record read.
///
/// The six hourly history fields and seven selected count fields belong to this
/// owner. The incoming liquid missing cache is retained, including after rain's
/// 2 mm default. The returned carrier is the actual hourly interval-one value.
#[allow(clippy::manual_range_contains)] // Retain the source's explicit lower/upper predicates.
pub(crate) fn process_hour(
    raw: &RawEpwOutputs,
    record: EpwRecord,
    state: &mut WeatherDayState,
) -> WeatherVars {
    let mut value = weather_vars_from_raw(raw, record);
    let missing = &mut state.missing_values.base;
    let missed = &mut state.missed_counts;
    let range = &mut state.out_of_range_counts;
    let mut dry = raw.mandatory_reals[0];
    let mut dew = raw.mandatory_reals[1];
    let mut humidity = raw.mandatory_reals[2];
    let mut pressure = raw.mandatory_reals[3];
    let mut direction = raw.mandatory_reals[14];
    let mut speed = raw.mandatory_reals[15];
    let mut liquid = raw.optional_reals[5];

    // Source preconditioning follows hour/date admission and precedes the
    // missing/range/history pass. It changes local values, never the raw owner.
    // WeatherManager.cc:2779-2793 and 2821-2822.
    if pressure < 0.0 {
        pressure = 999999.0;
    }
    if speed < 0.0 {
        speed = 999.0;
    }
    if direction < -360.0 || direction > 360.0 {
        direction = 999.0;
    }
    if humidity < 0.0 {
        humidity = 999.0;
    }
    if liquid < 0.0 {
        liquid = 999.0;
    }

    if dry >= 99.9 {
        dry = missing.out_dry_bulb_temp;
        missed.out_dry_bulb_temp += 1;
    }
    if dry < -90.0 || dry > 70.0 {
        range.out_dry_bulb_temp += 1;
    }
    if dew >= 99.9 {
        dew = missing.out_dew_point_temp;
        missed.out_dew_point_temp += 1;
    }
    if dew < -90.0 || dew > 70.0 {
        range.out_dew_point_temp += 1;
    }
    if humidity >= 999.0 {
        humidity = missing.out_rel_hum;
        missed.out_rel_hum += 1;
    }
    if humidity < 0.0 || humidity > 110.0 {
        range.out_rel_hum += 1;
    }
    if pressure >= 999999.0 {
        pressure = missing.out_baro_press;
        missed.out_baro_press += 1;
    }
    if pressure <= 31000.0 || pressure > 120000.0 {
        range.out_baro_press += 1;
        pressure = missing.out_baro_press;
    }
    if direction >= 999.0 {
        direction = missing.wind_dir;
        missed.wind_dir += 1;
    }
    if direction < 0.0 || direction > 360.0 {
        range.wind_dir += 1;
    }
    if speed >= 999.0 {
        speed = missing.wind_speed;
        missed.wind_speed += 1;
    }
    if speed < 0.0 || speed > 40.0 {
        range.wind_speed += 1;
    }
    if liquid >= 999.0 {
        liquid = missing.liquid_precip;
        missed.liquid_precip += 1;
    }

    value.out_dry_bulb_temp = dry;
    value.out_dew_point_temp = dew;
    value.out_baro_press = pressure;
    value.out_rel_hum = humidity;
    value.wind_dir = direction;
    value.wind_speed = speed;
    value.liquid_precip = liquid;
    value.is_rain =
        raw.observation_indicator == 0 && raw.weather_codes[..3].iter().any(|code| *code < 9);
    if value.is_rain && value.liquid_precip == 0.0 {
        value.liquid_precip = 2.0;
    }
    missing.out_dry_bulb_temp = dry;
    missing.out_dew_point_temp = dew;
    // Preserve the source's percent -> fraction -> percent arithmetic and
    // integer rounding, rather than storing the unrounded hourly percentage.
    missing.out_rel_hum = f64::from((humidity * 0.01 * 100.0).round() as i32);
    missing.out_baro_press = pressure;
    missing.wind_dir = direction;
    missing.wind_speed = speed;
    value
}

/// Whole source wind interpolation, WeatherManager.cc:3183–3198.
///
/// Signed remainder matches `std::fmod`; out-of-range negative directions are
/// counted by the hourly owner and are not silently normalized here.
pub(crate) fn interpolate_wind_direction(previous: f64, current: f64, weight: f64) -> f64 {
    let mut current_angle = current;
    let mut previous_angle = previous;
    let difference = (current_angle - previous_angle).abs();
    if difference > 180.0 {
        if current_angle > previous_angle {
            previous_angle += 360.0;
        } else {
            current_angle += 360.0;
        }
    }
    (previous_angle + (current_angle - previous_angle) * weight) % 360.0
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::weather::raw::project_record;

    fn raw() -> RawEpwOutputs {
        let mut raw = RawEpwOutputs {
            dates: [2013, 1, 1, 1, 60],
            observation_indicator: 9,
            weather_codes: [9; 9],
            ..RawEpwOutputs::default()
        };
        raw.mandatory_reals[0] = 12.0;
        raw.mandatory_reals[1] = 4.0;
        raw.mandatory_reals[2] = 50.6;
        raw.mandatory_reals[3] = 100000.0;
        raw.mandatory_reals[6] = 300.0;
        raw.mandatory_reals[14] = 350.0;
        raw.mandatory_reals[15] = 3.0;
        raw.optional_reals[5] = 1.25;
        raw
    }

    fn processed(raw: &RawEpwOutputs, state: &mut WeatherDayState) -> WeatherVars {
        process_hour(raw, project_record(raw, 1).unwrap(), state)
    }

    #[test]
    fn missing_hours_reuse_six_histories_and_keep_the_liquid_cache() {
        let mut owner = WeatherDayState::default();
        owner.missing_values.base.liquid_precip = 0.75;
        let first = processed(&raw(), &mut owner);
        assert_eq!(first.out_rel_hum, 50.6);
        assert_eq!(owner.missing_values.base.out_rel_hum, 51.0);
        let mut next = raw();
        for index in [0, 1] {
            next.mandatory_reals[index] = 99.9;
        }
        for index in [2, 14, 15] {
            next.mandatory_reals[index] = 999.0;
        }
        next.mandatory_reals[3] = 999999.0;
        next.optional_reals[5] = 999.0;
        let second = processed(&next, &mut owner);
        assert_eq!(second.out_dry_bulb_temp, 12.0);
        assert_eq!(second.out_dew_point_temp, 4.0);
        assert_eq!(second.out_rel_hum, 51.0);
        assert_eq!(second.out_baro_press, 100000.0);
        assert_eq!(second.wind_dir, 350.0);
        assert_eq!(second.wind_speed, 3.0);
        assert_eq!(second.liquid_precip, 0.75);
        assert_eq!(owner.missing_values.base.liquid_precip, 0.75);
        let counts = owner.missed_counts;
        assert_eq!(counts.out_dry_bulb_temp, 1);
        assert_eq!(counts.out_dew_point_temp, 1);
        assert_eq!(counts.out_rel_hum, 1);
        assert_eq!(counts.out_baro_press, 1);
        assert_eq!(counts.wind_dir, 1);
        assert_eq!(counts.wind_speed, 1);
        assert_eq!(counts.liquid_precip, 1);
        assert_eq!(owner.out_of_range_counts, Default::default());
    }

    #[test]
    fn range_counts_keep_values_except_pressure_and_do_not_write_other_histories() {
        let mut owner = WeatherDayState::default();
        owner.missing_values.base.out_baro_press = 101000.0;
        owner.missing_values.base.total_sky_cover = -0.0;
        owner.out_of_range_counts.liquid_precip = 19;
        let mut input = raw();
        input.mandatory_reals[0] = -90.1;
        input.mandatory_reals[1] = 70.1;
        input.mandatory_reals[2] = 110.1;
        input.mandatory_reals[3] = 31000.0;
        input.mandatory_reals[14] = -1.0;
        input.mandatory_reals[15] = 40.1;
        input.optional_reals[5] = -2.0;
        let value = processed(&input, &mut owner);
        assert_eq!(value.out_dry_bulb_temp, -90.1);
        assert_eq!(value.out_dew_point_temp, 70.1);
        assert_eq!(value.out_rel_hum, 110.1);
        assert_eq!(value.out_baro_press, 101000.0);
        assert_eq!(value.wind_dir, -1.0);
        assert_eq!(value.wind_speed, 40.1);
        assert_eq!(value.liquid_precip, 0.0);
        assert_eq!(owner.missed_counts.liquid_precip, 1);
        let counts = owner.out_of_range_counts;
        assert_eq!(counts.out_dry_bulb_temp, 1);
        assert_eq!(counts.out_dew_point_temp, 1);
        assert_eq!(counts.out_rel_hum, 1);
        assert_eq!(counts.out_baro_press, 1);
        assert_eq!(counts.wind_dir, 1);
        assert_eq!(counts.wind_speed, 1);
        assert_eq!(counts.liquid_precip, 19);
        assert_eq!(
            owner.missing_values.base.total_sky_cover.to_bits(),
            (-0.0_f64).to_bits()
        );
    }

    #[test]
    fn negative_selected_inputs_become_missing_before_range_and_history() {
        let mut owner = WeatherDayState::default();
        owner.missing_values.base.liquid_precip = 0.75;
        let first = processed(&raw(), &mut owner);
        let mut input = raw();
        input.mandatory_reals[2] = -0.1;
        input.mandatory_reals[3] = -1.0;
        input.mandatory_reals[14] = 360.0001;
        input.mandatory_reals[15] = -0.1;
        input.optional_reals[5] = -0.1;
        let value = processed(&input, &mut owner);
        assert_eq!(value.out_rel_hum, 51.0);
        assert_eq!(value.out_baro_press, first.out_baro_press);
        assert_eq!(value.wind_dir, first.wind_dir);
        assert_eq!(value.wind_speed, first.wind_speed);
        assert_eq!(value.liquid_precip, 0.75);
        let missed = owner.missed_counts;
        assert_eq!(missed.out_rel_hum, 1);
        assert_eq!(missed.out_baro_press, 1);
        assert_eq!(missed.wind_dir, 1);
        assert_eq!(missed.wind_speed, 1);
        assert_eq!(missed.liquid_precip, 1);
        assert_eq!(owner.out_of_range_counts, Default::default());
        assert_eq!(owner.missing_values.base.out_rel_hum, 51.0);
        assert_eq!(owner.missing_values.base.liquid_precip, 0.75);
        // Actual parser values and source identity are not rewritten.
        assert_eq!(input.mandatory_reals[2], -0.1);
        assert_eq!(input.mandatory_reals[14], 360.0001);
        assert_eq!(input.optional_reals[5], -0.1);
    }

    #[test]
    fn minus_360_wind_remains_range_only_but_lower_values_are_missing() {
        let mut owner = WeatherDayState::default();
        owner.missing_values.base.wind_dir = 180.0;
        let mut input = raw();
        input.mandatory_reals[14] = -360.0001;
        let missing = processed(&input, &mut owner);
        assert_eq!(missing.wind_dir, 180.0);
        assert_eq!(owner.missed_counts.wind_dir, 1);
        assert_eq!(owner.out_of_range_counts.wind_dir, 0);
        input.mandatory_reals[14] = -360.0;
        let boundary = processed(&input, &mut owner);
        assert_eq!(boundary.wind_dir, -360.0);
        assert_eq!(owner.missed_counts.wind_dir, 1);
        assert_eq!(owner.out_of_range_counts.wind_dir, 1);
        assert_eq!(owner.missing_values.base.wind_dir, -360.0);
    }

    #[test]
    fn observation_rain_default_is_distinct_from_liquid_missing_history() {
        let mut owner = WeatherDayState::default();
        owner.missing_values.base.liquid_precip = -0.0;
        let mut input = raw();
        input.observation_indicator = 0;
        input.weather_codes[2] = 8;
        input.optional_reals[5] = 0.0;
        let rain = processed(&input, &mut owner);
        assert!(rain.is_rain);
        assert_eq!(rain.liquid_precip, 2.0);
        assert_eq!(
            owner.missing_values.base.liquid_precip.to_bits(),
            (-0.0_f64).to_bits()
        );
        input.optional_reals[5] = 999.0;
        input.observation_indicator = 9;
        let missing = processed(&input, &mut owner);
        assert!(!missing.is_rain);
        assert_eq!(missing.liquid_precip.to_bits(), (-0.0_f64).to_bits());
        assert_eq!(owner.missed_counts.liquid_precip, 1);
    }

    #[test]
    fn wind_uses_wraparound_and_signed_remainder_without_clamping() {
        assert_eq!(interpolate_wind_direction(350.0, 10.0, 0.5), 0.0);
        assert_eq!(interpolate_wind_direction(10.0, 350.0, 0.25), 5.0);
        assert_eq!(interpolate_wind_direction(-20.0, -10.0, 0.5), -15.0);
        assert_eq!(
            interpolate_wind_direction(-0.0, -0.0, 1.0).to_bits(),
            0.0_f64.to_bits()
        );
    }
}
